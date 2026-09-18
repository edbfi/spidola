#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Spidola contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Assert the pinned Apple/Rust toolchains (TECH_SPEC §9: "build scripts assert them").
# The pins live in docs/toolchains.md, rust-toolchain.toml, and apps/tvos/project.yml;
# this script fails legibly on a mismatch. The Android JDK/Kotlin pins are asserted by
# apps/androidtv/build.gradle.kts.
set -euo pipefail

fail() { echo "toolchain assertion failed: $*" >&2; exit 1; }

# --- Rust (pinned by rust-toolchain.toml) ---
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
want_rust="$(python3 -c 'import sys, tomllib; print(tomllib.load(open(sys.argv[1], "rb"))["toolchain"]["channel"])' "$repo_root/rust-toolchain.toml")"
[[ "$want_rust" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || fail "expected an exact stable Rust pin, found $want_rust"
have_rust="$(rustc --version | awk '{print $2}')"
[ "$have_rust" = "$want_rust" ] || fail "rustc $want_rust required, found $have_rust"
echo "ok: rustc $have_rust"

# --- Swift / Xcode (macOS lanes only) ---
if command -v swift >/dev/null 2>&1; then
  want_swift_major="6.3"
  have_swift="$(swift --version 2>/dev/null | grep -oE 'Swift version [0-9]+\.[0-9]+' | awk '{print $3}' | head -1)"
  case "$have_swift" in
    "$want_swift_major"*) echo "ok: swift $have_swift" ;;
    *) fail "Swift $want_swift_major.x required, found ${have_swift:-none}" ;;
  esac
fi

# Only report Xcode when a real Xcode (not the Command Line Tools stub) answers.
if xcodebuild -version >/dev/null 2>&1; then
  echo "ok: $(xcodebuild -version | head -1)"
fi

echo "toolchain assertions passed"
