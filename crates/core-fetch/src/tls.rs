// SPDX-FileCopyrightText: 2026 Spidola contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Per-source self-signed-TLS escape hatch (TECH_SPEC §4.5, §12).
//!
//! Self-hosted headends with self-signed certificates are common, so an opt-in
//! "accept invalid TLS" hatch exists — but it is **off by default**, **loudly labeled**
//! (the source list carries a persistent badge, §12), and **scoped to a single source**:
//! it is applied to that source's own [`reqwest::Client`], never globally, so one source's
//! choice cannot weaken TLS for any other. TLS itself is rustls with platform roots; no
//! OpenSSL anywhere in the tree.

use reqwest::{Certificate, ClientBuilder};

/// Applies the per-source TLS posture to a client builder.
///
/// With `accept_invalid_tls == false` (the default), certificates are verified against
/// native and bundled roots. With `true`, certificate validation is disabled for
/// **this client only**.
pub(crate) fn apply(
    builder: ClientBuilder,
    accept_invalid_tls: bool,
) -> Result<ClientBuilder, reqwest::Error> {
    // Reqwest 0.13 otherwise delegates to a platform verifier. Preserve our
    // existing native-plus-bundled root policy, including TV targets without a store.
    let native = rustls_native_certs::load_native_certs();
    let roots = root_certificates(native.certs.iter().map(AsRef::as_ref))?;
    let builder = builder.tls_certs_only(roots);
    if accept_invalid_tls {
        Ok(builder.danger_accept_invalid_certs(true))
    } else {
        Ok(builder)
    }
}

fn root_certificates<'a>(
    native: impl Iterator<Item = &'a [u8]>,
) -> Result<Vec<Certificate>, reqwest::Error> {
    native
        .chain(
            webpki_root_certs::TLS_SERVER_ROOT_CERTS
                .iter()
                .map(AsRef::as_ref),
        )
        .map(Certificate::from_der)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::root_certificates;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    type TestResult = Result<(), Box<dyn std::error::Error>>;
    type TlsStub = (String, Vec<u8>, thread::JoinHandle<()>);

    fn tls_stub() -> Result<TlsStub, Box<dyn std::error::Error>> {
        let generated = rcgen::generate_simple_self_signed(vec!["localhost".to_owned()])?;
        let der = generated.cert.der().to_vec();
        let key =
            rustls::pki_types::PrivatePkcs8KeyDer::from(generated.signing_key.serialize_der());
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![generated.cert.der().clone()], key.into())?;
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let url = format!("https://localhost:{}/", listener.local_addr()?.port());
        let handle = thread::spawn(move || {
            let Ok((socket, _)) = listener.accept() else {
                return;
            };
            let _ = socket.set_read_timeout(Some(Duration::from_secs(5)));
            let Ok(connection) = rustls::ServerConnection::new(Arc::new(config)) else {
                return;
            };
            let mut stream = rustls::StreamOwned::new(connection, socket);
            let mut request = [0_u8; 4096];
            // An untrusted peer aborts the TLS handshake before sending HTTP.
            if stream.read(&mut request).is_ok() {
                let _ = stream.write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok",
                );
                let _ = stream.flush();
            }
        });
        Ok((url, der, handle))
    }

    #[tokio::test]
    async fn empty_native_store_keeps_bundled_roots_but_rejects_untrusted_tls() -> TestResult {
        let (url, _, server) = tls_stub()?;
        let roots = root_certificates(std::iter::empty())?;
        assert!(
            !roots.is_empty(),
            "TV targets still need the bundled root store"
        );
        let client = reqwest::Client::builder()
            .tls_certs_only(roots)
            .timeout(Duration::from_secs(5))
            .build()?;
        assert!(client.get(url).send().await.is_err());
        assert!(server.join().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn native_root_is_trusted_alongside_bundled_roots() -> TestResult {
        let (url, certificate, server) = tls_stub()?;
        let roots = root_certificates(std::iter::once(certificate.as_slice()))?;
        let client = reqwest::Client::builder()
            .tls_certs_only(roots)
            .timeout(Duration::from_secs(5))
            .build()?;
        assert_eq!(client.get(url).send().await?.text().await?, "ok");
        assert!(server.join().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn invalid_tls_opt_in_remains_scoped_to_the_selected_client() -> TestResult {
        let (url, _, server) = tls_stub()?;
        let client = super::apply(
            reqwest::Client::builder().timeout(Duration::from_secs(5)),
            true,
        )?
        .build()?;
        assert_eq!(client.get(url).send().await?.text().await?, "ok");
        assert!(server.join().is_ok());
        let (url, _, server) = tls_stub()?;
        let verified = super::apply(
            reqwest::Client::builder().timeout(Duration::from_secs(5)),
            false,
        )?
        .build()?;
        assert!(verified.get(url).send().await.is_err());
        assert!(server.join().is_ok());
        Ok(())
    }
}
