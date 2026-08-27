#!/usr/bin/env python3
"""Tests for the Servo toolchain receipt capture utility."""

from __future__ import annotations

import importlib.util
import json
import os
import stat
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

SCRIPT = Path(__file__).resolve().parents[1] / "hepta-servo-toolchain-receipt.py"
PYTHON = Path("/usr/bin/python3")


def load_module() -> ModuleType:
    specification = importlib.util.spec_from_file_location("hepta_servo_toolchain_receipt", SCRIPT)
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load toolchain receipt utility")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class ToolchainReceiptTests(unittest.TestCase):
    def setUp(self) -> None:
        self.module = load_module()
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name).resolve()
        self.rustc = self.write_executable(
            "fake-rustc",
            """#!/usr/bin/python3
import sys
if sys.argv[1:] != ['-vV']:
    raise SystemExit(2)
print('rustc 1.95.0 (aaaaaaaaaaaaaaaa 2026-08-01)')
print('binary: rustc')
print('commit-hash: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa')
print('commit-date: 2026-08-01')
print('host: x86_64-unknown-linux-gnu')
print('release: 1.95.0')
print('LLVM version: 20.1.0')
""",
        )
        self.cargo = self.write_executable(
            "fake-cargo",
            """#!/usr/bin/python3
import sys
if sys.argv[1:] != ['-Vv']:
    raise SystemExit(2)
print('cargo 1.95.0 (bbbbbbbb 2026-08-01)')
print('release: 1.95.0')
print('commit-hash: bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb')
print('commit-date: 2026-08-01')
print('host: x86_64-unknown-linux-gnu')
""",
        )
        self.linker = self.write_executable(
            "fake-clang",
            """#!/usr/bin/python3
import sys
if sys.argv[1:] != ['--version']:
    raise SystemExit(2)
print('clang version 20.1.0')
print('Target: x86_64-unknown-linux-gnu')
""",
        )

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def write_executable(self, name: str, body: str) -> Path:
        path = self.root / name
        path.write_text(body, encoding="utf-8")
        path.chmod(0o700)
        return path

    def capture(self) -> dict[str, object]:
        return self.module.capture(
            target="x86_64-unknown-linux-gnu",
            linker_kind="clang",
            rustc_path=self.rustc,
            cargo_path=self.cargo,
            linker_path=self.linker,
        )

    def test_capture_binds_versions_binaries_and_closed_authority(self) -> None:
        receipt = self.capture()
        self.assertEqual(receipt["schema"], self.module.SCHEMA)
        self.assertEqual(receipt["host"], "x86_64-unknown-linux-gnu")
        self.assertEqual(receipt["target"], "x86_64-unknown-linux-gnu")
        self.assertEqual(
            receipt["rustc"]["commit_hash"],
            "a" * 40,
        )
        self.assertEqual(receipt["cargo"]["commit_hash"], "b" * 40)
        self.assertEqual(receipt["linker"]["kind"], "clang")
        for section in ("rustc", "cargo", "linker"):
            self.assertEqual(len(receipt[section]["binary_sha256"]), 64)
            self.assertEqual(len(receipt[section]["output_sha256"]), 64)
            self.assertGreater(receipt[section]["binary_bytes"], 0)
        self.assertFalse(receipt["capture"]["network_access_used"])
        self.assertFalse(receipt["capture"]["build_run"])
        self.assertFalse(receipt["capture"]["artifact_created"])
        self.assertFalse(receipt["machine_local_paths_included"])
        self.assertTrue(all(value is False for value in receipt["authority"].values()))
        encoded = self.module.canonical_bytes(receipt).decode("utf-8")
        self.assertNotIn(str(self.root), encoded)

    def test_capture_is_deterministic_for_unchanged_binaries(self) -> None:
        self.assertEqual(self.capture(), self.capture())

    def test_minimal_environment_drops_home_proxy_path_and_credentials(self) -> None:
        old = os.environ.copy()
        os.environ.update(
            {
                "HOME": "/secret/home",
                "PATH": "/secret/path",
                "HTTPS_PROXY": "http://proxy.invalid",
                "GITHUB_TOKEN": "secret",
                "AWS_SECRET_ACCESS_KEY": "secret",
            }
        )
        try:
            environment = self.module.minimal_environment()
        finally:
            os.environ.clear()
            os.environ.update(old)
        for key in ("HOME", "PATH", "HTTPS_PROXY", "GITHUB_TOKEN", "AWS_SECRET_ACCESS_KEY"):
            self.assertNotIn(key, environment)
        self.assertEqual(environment["LC_ALL"], "C")
        self.assertEqual(environment["GIT_TERMINAL_PROMPT"], "0")

    def test_symlink_and_hardlink_binaries_fail_closed(self) -> None:
        symlink = self.root / "rustc-link"
        symlink.symlink_to(self.rustc.name)
        with self.assertRaises(self.module.ToolchainReceiptError):
            self.module.canonical_binary(str(symlink), "rustc")
        hardlink = self.root / "rustc-hardlink"
        os.link(self.rustc, hardlink)
        with self.assertRaises(self.module.ToolchainReceiptError):
            self.module.canonical_binary(str(self.rustc), "rustc")

    def test_group_writable_binary_fails_closed(self) -> None:
        self.rustc.chmod(0o720)
        with self.assertRaises(self.module.ToolchainReceiptError):
            self.module.canonical_binary(str(self.rustc), "rustc")

    def test_invalid_rustc_commit_hash_fails_closed(self) -> None:
        self.rustc = self.write_executable(
            "invalid-rustc",
            """#!/usr/bin/python3
print('rustc 1.95.0')
print('commit-hash: not-a-hash')
print('host: x86_64-unknown-linux-gnu')
print('release: 1.95.0')
""",
        )
        with self.assertRaises(self.module.ToolchainReceiptError):
            self.capture()

    def test_mismatched_cargo_and_rustc_hosts_fail_closed(self) -> None:
        self.cargo = self.write_executable(
            "other-cargo",
            """#!/usr/bin/python3
print('cargo 1.95.0')
print('release: 1.95.0')
print('host: aarch64-apple-darwin')
""",
        )
        with self.assertRaises(self.module.ToolchainReceiptError):
            self.capture()

    def test_oversized_version_output_fails_closed(self) -> None:
        noisy = self.write_executable(
            "noisy",
            """#!/usr/bin/python3
print('x' * 70000)
""",
        )
        with self.assertRaises(self.module.ToolchainReceiptError):
            self.module.run_bounded(noisy, [])

    def test_timeout_fails_closed(self) -> None:
        sleeper = self.write_executable(
            "sleeper",
            """#!/usr/bin/python3
import time
time.sleep(1)
print('late')
""",
        )
        with self.assertRaises(self.module.ToolchainReceiptError):
            self.module.run_bounded(sleeper, [], timeout_seconds=0.05)

    def test_private_atomic_output_refuses_overwrite(self) -> None:
        receipt = self.capture()
        output = self.root / "toolchain.json"
        self.module.write_atomic(output, receipt)
        self.assertEqual(stat.S_IMODE(output.stat().st_mode), 0o600)
        self.assertEqual(json.loads(output.read_text(encoding="utf-8")), receipt)
        with self.assertRaises(self.module.ToolchainReceiptError):
            self.module.write_atomic(output, receipt)


if __name__ == "__main__":
    if not PYTHON.is_file():
        raise SystemExit("/usr/bin/python3 is required for this qualification fixture")
    unittest.main()
