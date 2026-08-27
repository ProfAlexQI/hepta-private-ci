#!/usr/bin/env python3
"""Tests for the hardened Servo build preflight v2 entrypoint."""

from __future__ import annotations

import importlib.util
import unittest
from pathlib import Path
from types import ModuleType

SCRIPT = Path(__file__).resolve().parents[1] / "hepta-servo-build-preflight-v2.py"


def load_module() -> ModuleType:
    specification = importlib.util.spec_from_file_location("hepta_servo_build_preflight_v2", SCRIPT)
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load build preflight v2")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class BuildPreflightV2Tests(unittest.TestCase):
    def setUp(self) -> None:
        self.entrypoint = load_module()
        self.base = self.entrypoint.load_base()
        self.toolchain = self.valid_toolchain()

    def authority(self) -> dict[str, bool]:
        return {key: False for key in sorted(self.base.AUTHORITY_KEYS)}

    def valid_toolchain(self) -> dict[str, object]:
        host = self.base.EXPECTED_TARGET
        return {
            "schema": self.base.TOOLCHAIN_SCHEMA,
            "schema_version": 1,
            "target": host,
            "host": host,
            "rustc": {
                "version": "rustc 1.95.0 (012345678 2026-08-01)",
                "release": "1.95.0",
                "commit_hash": "8" * 40,
                "host": host,
                "output_sha256": "1" * 64,
                "binary_sha256": "2" * 64,
                "binary_bytes": 100,
            },
            "cargo": {
                "version": "cargo 1.95.0 (abcdef123 2026-08-01)",
                "release": "1.95.0",
                "commit_hash": "9" * 40,
                "host": host,
                "output_sha256": "3" * 64,
                "binary_sha256": "4" * 64,
                "binary_bytes": 101,
            },
            "linker": {
                "kind": "clang",
                "version": "Ubuntu clang version 20.1.0 (build@host)",
                "output_sha256": "5" * 64,
                "binary_sha256": "6" * 64,
                "binary_bytes": 102,
            },
            "capture": {
                "commands": ["rustc -vV", "cargo -Vv", "clang --version"],
                "minimal_environment": dict(
                    self.entrypoint.TOOLCHAIN_CAPTURE_ENVIRONMENT
                ),
                "network_access_used": False,
                "build_run": False,
                "artifact_created": False,
            },
            "machine_local_paths_included": False,
            "authority": self.authority(),
        }

    def test_exact_capture_commands_environment_and_version_text_pass(self) -> None:
        projection = self.base.toolchain_projection(self.toolchain)
        self.assertEqual(projection["linker_kind"], "clang")
        self.assertEqual(projection["target"], self.base.EXPECTED_TARGET)

    def test_capture_command_drift_fails_closed(self) -> None:
        self.toolchain["capture"]["commands"][-1] = "cc --version"
        with self.assertRaises(self.entrypoint.BuildPreflightV2Error):
            self.base.toolchain_projection(self.toolchain)

    def test_capture_environment_extra_or_missing_key_fails_closed(self) -> None:
        self.toolchain["capture"]["minimal_environment"]["HOME"] = "/secret"
        with self.assertRaises(self.entrypoint.BuildPreflightV2Error):
            self.base.toolchain_projection(self.toolchain)
        self.toolchain = self.valid_toolchain()
        del self.toolchain["capture"]["minimal_environment"]["TZ"]
        with self.assertRaises(self.entrypoint.BuildPreflightV2Error):
            self.base.toolchain_projection(self.toolchain)

    def test_unlisted_linker_kind_fails_closed(self) -> None:
        self.toolchain["linker"]["kind"] = "custom-linker"
        self.toolchain["capture"]["commands"][-1] = "custom-linker --version"
        with self.assertRaises(self.entrypoint.BuildPreflightV2Error):
            self.base.toolchain_projection(self.toolchain)

    def test_version_text_with_path_backslash_or_shell_character_fails_closed(self) -> None:
        invalid = (
            "/home/runner/rustc 1.95.0",
            "clang C:\\toolchain",
            "clang 20.1.0; touch pwned",
            "cargo 1.95.0\nforged",
        )
        for value in invalid:
            self.toolchain = self.valid_toolchain()
            self.toolchain["linker"]["version"] = value
            with self.assertRaises(self.entrypoint.BuildPreflightV2Error):
                self.base.toolchain_projection(self.toolchain)


if __name__ == "__main__":
    unittest.main()
