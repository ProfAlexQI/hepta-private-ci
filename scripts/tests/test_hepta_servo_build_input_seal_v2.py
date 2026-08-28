#!/usr/bin/env python3
"""Tests for the toolchain-bound Servo build-input sealer v2."""

from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

SCRIPT = Path(__file__).resolve().parents[1] / "hepta-servo-build-input-seal-v2.py"


def load_module() -> ModuleType:
    specification = importlib.util.spec_from_file_location("hepta_servo_build_input_v2", SCRIPT)
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load build-input v2 sealer")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class BuildInputV2Tests(unittest.TestCase):
    def setUp(self) -> None:
        self.module = load_module()
        self.base = self.module.load_base()
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name).resolve()
        self.source = self.valid_source()
        self.toolchain = self.valid_toolchain()
        self.recipe = self.valid_recipe()

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def authority(self) -> dict[str, bool]:
        return {key: False for key in sorted(self.base.AUTHORITY_KEYS)}

    def valid_source(self) -> dict[str, object]:
        return {
            "schema": self.base.EXPECTED_SOURCE_VERIFICATION_SCHEMA,
            "schema_version": 1,
            "source": {
                "repository": "servo/servo",
                "commit": self.base.EXPECTED_COMMIT,
                "tree": self.base.EXPECTED_TREE,
                "tree_manifest_sha256": "1" * 64,
                "recomputed_tree": self.base.EXPECTED_TREE,
            },
            "bundle_receipt_sha256": "2" * 64,
            "fetch_receipt_sha256": ["3" * 64, "4" * 64],
            "compressed_archive_sha256": "5" * 64,
            "tar_sha256": "6" * 64,
            "license_packet_sha256": "7" * 64,
            "verification": {
                "canonical_json": True,
                "no_machine_local_paths": True,
                "two_distinct_acquisition_nonces": True,
                "gzip_single_member_mtime_zero": True,
                "archive_paths_safe": True,
                "git_tree_recomputed": True,
                "pinned_tree_matched": True,
                "license_matched": True,
                "servo_built": False,
                "servo_runtime_qualified": False,
                "release_qualified": False,
            },
            "machine_local_paths_included": False,
            "authority": self.authority(),
        }

    def valid_toolchain(self) -> dict[str, object]:
        return {
            "schema": self.module.TOOLCHAIN_SCHEMA,
            "schema_version": 1,
            "target": "x86_64-unknown-linux-gnu",
            "host": "x86_64-unknown-linux-gnu",
            "rustc": {
                "version": "rustc 1.95.0",
                "release": "1.95.0",
                "commit_hash": "8" * 40,
                "host": "x86_64-unknown-linux-gnu",
                "output_sha256": "9" * 64,
                "binary_sha256": "a" * 64,
                "binary_bytes": 100,
            },
            "cargo": {
                "version": "cargo 1.95.0",
                "release": "1.95.0",
                "commit_hash": "b" * 40,
                "host": "x86_64-unknown-linux-gnu",
                "output_sha256": "c" * 64,
                "binary_sha256": "d" * 64,
                "binary_bytes": 101,
            },
            "linker": {
                "kind": "clang",
                "version": "clang version 20.1.0",
                "output_sha256": "e" * 64,
                "binary_sha256": "f" * 64,
                "binary_bytes": 102,
            },
            "capture": {
                "commands": ["rustc -vV", "cargo -Vv", "clang --version"],
                "minimal_environment": {
                    "GIT_CONFIG_NOSYSTEM": "1",
                    "GIT_TERMINAL_PROMPT": "0",
                    "LANG": "C",
                    "LC_ALL": "C",
                    "TZ": "UTC",
                },
                "network_access_used": False,
                "build_run": False,
                "artifact_created": False,
            },
            "machine_local_paths_included": False,
            "authority": self.authority(),
        }

    def toolchain_projection(self) -> dict[str, str]:
        return {
            "rustc_version": self.toolchain["rustc"]["version"],
            "rustc_commit_hash": self.toolchain["rustc"]["commit_hash"],
            "cargo_version": self.toolchain["cargo"]["version"],
            "host": self.toolchain["host"],
            "target": self.toolchain["target"],
            "rustc_binary_sha256": self.toolchain["rustc"]["binary_sha256"],
            "cargo_binary_sha256": self.toolchain["cargo"]["binary_sha256"],
            "linker_kind": self.toolchain["linker"]["kind"],
            "linker_version": self.toolchain["linker"]["version"],
            "linker_binary_sha256": self.toolchain["linker"]["binary_sha256"],
        }

    def valid_recipe(self) -> dict[str, object]:
        return {
            "schema": self.base.EXPECTED_RECIPE_SCHEMA,
            "schema_version": 1,
            "plan_id": "HEPTA-BROWSER-WEB-D",
            "stage": "WEB_C1",
            "status": "FROZEN_INPUTS_NOT_EXECUTED",
            "target": "x86_64-unknown-linux-gnu",
            "profile": "release",
            "manifest_path": "worker/Cargo.toml",
            "package": "hepta-servo-worker",
            "artifact_path": "target/x86_64-unknown-linux-gnu/release/hepta-servo-worker",
            "features": ["background-hang-monitor", "baked-in-resources"],
            "default_features": False,
            "jobs": 4,
            "command_prefix": ["cargo", "build", "--locked", "--offline", "--frozen"],
            "environment": dict(self.base.ALLOWED_ENVIRONMENT),
            "toolchain": self.toolchain_projection(),
            "build_network": False,
            "source_mutation_allowed": False,
            "runtime_external_network": False,
            "authority": self.authority(),
        }

    def seal(self) -> dict[str, object]:
        return self.module.seal_v2(
            self.source,
            self.base.canonical_bytes(self.source),
            self.recipe,
            self.base.canonical_bytes(self.recipe),
            self.toolchain,
            self.base.canonical_bytes(self.toolchain),
            self.base,
        )

    def test_v2_binds_independent_toolchain_receipt_digest(self) -> None:
        manifest = self.seal()
        self.assertEqual(manifest["schema"], self.module.OUTPUT_SCHEMA)
        self.assertEqual(
            manifest["toolchain_receipt_sha256"],
            self.base.sha256_bytes(self.base.canonical_bytes(self.toolchain)),
        )
        self.assertTrue(
            manifest["qualification"]["toolchain_receipt_independently_captured"]
        )
        self.assertFalse(manifest["qualification"]["build_run"])
        self.assertFalse(manifest["qualification"]["artifact_created"])
        self.assertTrue(all(value is False for value in manifest["authority"].values()))

    def test_v2_is_deterministic_for_identical_receipts(self) -> None:
        self.assertEqual(self.seal(), self.seal())

    def test_recipe_toolchain_version_mismatch_fails_closed(self) -> None:
        self.recipe["toolchain"]["rustc_version"] = "rustc 9.9.9"
        with self.assertRaises(self.module.BuildInputV2Error):
            self.seal()

    def test_recipe_toolchain_binary_digest_mismatch_fails_closed(self) -> None:
        self.recipe["toolchain"]["linker_binary_sha256"] = "0" * 64
        with self.assertRaises(self.module.BuildInputV2Error):
            self.seal()

    def test_recipe_and_receipt_target_mismatch_fails_closed(self) -> None:
        self.toolchain["target"] = "aarch64-apple-darwin"
        with self.assertRaises(self.module.BuildInputV2Error):
            self.seal()

    def test_toolchain_network_or_build_claim_fails_closed(self) -> None:
        for key in ("network_access_used", "build_run", "artifact_created"):
            self.toolchain = self.valid_toolchain()
            self.recipe = self.valid_recipe()
            self.toolchain["capture"][key] = True
            with self.assertRaises(self.module.BuildInputV2Error):
                self.seal()

    def test_toolchain_open_authority_fails_closed(self) -> None:
        self.toolchain["authority"]["runtime_authority"] = True
        with self.assertRaises(self.module.BuildInputV2Error):
            self.seal()

    def test_toolchain_machine_path_marker_fails_closed(self) -> None:
        self.toolchain["machine_local_paths_included"] = True
        with self.assertRaises(self.module.BuildInputV2Error):
            self.seal()

    def test_toolchain_component_host_mismatch_fails_closed(self) -> None:
        self.toolchain["cargo"]["host"] = "aarch64-apple-darwin"
        with self.assertRaises(self.module.BuildInputV2Error):
            self.seal()


if __name__ == "__main__":
    unittest.main()
