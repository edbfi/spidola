#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Spidola contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
source "$root/tools/ci/apple-tools.env"
cache="$root/target/apple-tools/$XCODEGEN_VERSION-$SWIFTLINT_VERSION-$XCBEAUTIFY_VERSION"
mkdir -p "$cache"
if [[ ! -x "$cache/xcodegen/bin/xcodegen" ]]; then
  curl --fail --location --retry 3 "https://github.com/yonaskolb/XcodeGen/releases/download/$XCODEGEN_VERSION/xcodegen.zip" -o "$cache/xcodegen.zip"
  unzip -oq "$cache/xcodegen.zip" -d "$cache"
fi
if [[ ! -x "$cache/swiftlint/swiftlint" ]]; then
  curl --fail --location --retry 3 "https://github.com/realm/SwiftLint/releases/download/$SWIFTLINT_VERSION/portable_swiftlint.zip" -o "$cache/swiftlint.zip"
  unzip -oq "$cache/swiftlint.zip" -d "$cache/swiftlint"
fi
if [[ ! -x "$cache/xcbeautify/xcbeautify" ]]; then
  curl --fail --location --retry 3 "https://github.com/cpisciotta/xcbeautify/releases/download/$XCBEAUTIFY_VERSION/xcbeautify-$XCBEAUTIFY_VERSION-arm64-apple-macosx.zip" -o "$cache/xcbeautify.zip"
  unzip -oq "$cache/xcbeautify.zip" -d "$cache/xcbeautify"
fi
"$cache/xcodegen/bin/xcodegen" --version
"$cache/swiftlint/swiftlint" version
"$cache/xcbeautify/xcbeautify" --version
if [[ -n "${GITHUB_PATH:-}" ]]; then
  printf '%s\n' "$cache/xcodegen/bin" "$cache/swiftlint" "$cache/xcbeautify" >> "$GITHUB_PATH"
else
  printf 'Add these directories to PATH:\n%s\n%s\n%s\n' "$cache/xcodegen/bin" "$cache/swiftlint" "$cache/xcbeautify"
fi
