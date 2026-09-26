<!--
SPDX-FileCopyrightText: 2026 Spidola contributors
SPDX-License-Identifier: AGPL-3.0-or-later
-->

# tools/ci

Shared CI helper scripts. `assert-toolchains.sh` enforces the pinned Rust/Swift/Xcode
versions (TECH_SPEC §9).

`ci.yml` runs on every pull request and default-branch push, and can be dispatched
manually. It calls the existing three native lanes; `ci / required` fails unless
all three plus repository hygiene succeed. Require this context from GitHub
Actions with branches up to date, enforce it for administrators, and disallow
force pushes, deletion and bypass. Keep any existing external DCO check required.
Tag releases call the complete CI workflow before building their signed APK.

Apple CLI releases are in `apple-tools.env`, installed by
`install-apple-tools.sh` (add its printed paths locally). Rust compiler/MSRV,
Kotlin/compiler catalog, cargo-ndk and the NDK have coupled pins: update their
machine sources and `docs/toolchains.md` together. Kotlin FFI harness libraries
are tracked separately alongside Gradle's catalog. SwiftPM's committed MPVKit
resolution and LGPL verification remain authoritative; generated bindings must
come from `cargo xtask gen-bindings`, never hand edits.

CI retains parser/database/property tests, both real-core foreign contract
harnesses, the three-ABI APK check, TV emulator/simulator smoke tests, REUSE,
translations, and Criterion's 20% PR regression gate. Push/manual benchmarks
validate candidate estimates without claiming a comparative PR measurement.
Simulator smoke coverage does not replace the documented real-device media,
remote-control, DRM, thermal/performance and signed store-release acceptance.
No distribution credentials are needed for pull-request CI.
