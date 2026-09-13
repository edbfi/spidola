# Continuous integration

All pull requests and main pushes run the native core, Android and Apple lanes,
hygiene and the aggregate gate. The ten exact checks are listed in
`.github/merge-policy.json`; missing, skipped, failed or cancelled checks block
unattended merging. CI preserves Rust tests and strict linting, the Criterion
regression gate, REUSE and per-platform license checks, generated binding drift,
both real-core contract harnesses and emulator/simulator tests. Physical hardware
and store acceptance remain the deferred Phase 8 work in the implementation plan.

The v1.1.0 default, mixed and automerge presets make all Renovate update types
eligible, including majors, native toolchains and shared-policy updates, without
dashboard approval. LGPL media grouping and custom version extraction remain;
compatibility and license gates must pass for updates to merge. No application,
toolchain pin, generated binding, license allowance or test threshold is changed
by this configuration. Biome is not used by this Rust/Swift/Kotlin repository.

Checked merging preserves genuine author sign-offs and dispatches full CI for the
exact merged default-branch commit, checked again at the aggregate gate. The
reusable CI entry point for releases remains available. Tag publication and manual
release dry runs retain their existing workflow and signing requirements.

Other changes require review of the exact head/base, full diff, authors/DCO,
every expected CI job and relevant artifacts before merging through ghmerge.
No branch protections or repository rulesets are configured. This extends the
Phase 0 CI/governance setup and preserves Phase 7 software validation.
