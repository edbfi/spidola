# SPDX-FileCopyrightText: 2026 Spidola contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""The toolchain guard follows the manifest while rejecting drift and floating pins."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


class ToolchainGuardTests(unittest.TestCase):
    def check(self, pin, actual):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scripts = root / "tools/ci"
            scripts.mkdir(parents=True)
            shutil.copy2(Path(__file__).with_name("assert-toolchains.sh"), scripts)
            (root / "rust-toolchain.toml").write_text(f'[toolchain]\nchannel = "{pin}"\n')
            binaries = root / "bin"
            binaries.mkdir()
            for name, body in {"rustc": f'echo "rustc {actual} (fixture)"',
                               "swift": 'echo "Swift version 6.3.3"',
                               "xcodebuild": "exit 1"}.items():
                path = binaries / name
                path.write_text("#!/bin/sh\n" + body + "\n")
                path.chmod(0o755)
            return subprocess.run(["bash", str(scripts / "assert-toolchains.sh")],
                                  cwd=root, env={**os.environ, "PATH": str(binaries) + os.pathsep + os.environ["PATH"]},
                                  text=True, capture_output=True, check=False)

    def test_new_exact_pin_is_read_from_manifest(self):
        result = self.check("1.98.1", "1.98.1")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_compiler_must_match_manifest(self):
        result = self.check("1.98.1", "1.96.1")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("rustc 1.98.1 required, found 1.96.1", result.stderr)

    def test_floating_pin_is_rejected(self):
        result = self.check("stable", "1.98.1")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("expected an exact stable Rust pin", result.stderr)


if __name__ == "__main__":
    unittest.main()
