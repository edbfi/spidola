# Continuous integration

All pull requests and main pushes run the native core, Android and Apple lanes,
hygiene and the fail-closed aggregate gate. Missing, skipped, failed or cancelled
mandatory jobs block merging. CI preserves Rust tests and strict linting, the
Criterion regression gate, REUSE and per-platform license checks, generated binding
drift, both real-core contract harnesses and emulator/simulator tests. Physical
hardware and store acceptance remain deferred Phase 8 work in the implementation
plan. LGPL media grouping and custom version extraction remain; compatibility,
license and genuine DCO gates must pass. No generated bindings, toolchain pins,
license allowances or test thresholds change.

Shared actions, workflows and presets use immutable `v3.0.1` references.
Renovate is the sole ongoing dependency merge owner. Direct automerge remains
explicitly disabled, including matching package rules, until the hosted rollout
proves native Renovate operation behind complete required CI. The legacy Actions
merger and its comment commands are retired.

The separate PR policy workflow verifies Conventional Commit titles, genuine
matching author sign-offs, Renovate provenance, holds, outstanding review requests
and unresolved changes requests. Require its actual emitted policy context alongside
all existing application/content checks, pinned to GitHub Actions, with strict
up-to-date branch protection. Preserve stronger review requirements. Explicit CI
dispatches do not substitute for a missing metadata policy result. Review exact
head/base, full diffs and all required results before a bootstrap merge, then
verify resulting default-branch CI. Repository-specific updater ownership and
manual publication or delivery controls remain unchanged.

The reusable CI entry point for releases remains available. Tag publication and
manual release dry runs retain their workflow and signing requirements. Biome is
not used by this Rust/Swift/Kotlin repository.
