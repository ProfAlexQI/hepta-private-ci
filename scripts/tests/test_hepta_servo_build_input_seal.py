#!/usr/bin/env python3
"""Tests for the Servo build-input manifest sealer."""

from __future__ import annotations

import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

SCRIPT = Path(__file__).resolve().parents[1] / "hepta-servo-build-input-seal.py"


def load_module() -> ModuleType:
    specification = importlib.util.spec_from_file_location("hepta_servo_build_input_seal", SCRIPT)
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load build-input sealer")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class BuildInputSealTests(unittest.TestCase):
    def setUp(self) -> None:
        self.module = load_module()
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name).resolve()
        self.source = self.valid_source_verification()
        self.recipe = self.valid_recipe()

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def valid_source_verification(self) -> dict[str, object]:
        authority = {key: False for key in sorted(self.module.AUTHORITY_KEYS)}
        return {
            "schema": self.module.EXPECTED_SOURCE_VERIFICATION_SCHEMA,
            "schema_version": 1,
            "source": {
                "repository": "servo/servo",
                "commit": self.module.EXPECTED_COMMIT,
                "tree": self.module.EXPECTED_TREE,
                "tree_manifest_sha256": "1" * 64,
                "recomputed_tree": self.module.EXPECTED_TREE,
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
            "authority": authority,
        }

    def valid_recipe(self) -> dict[str, object]:
        authority = {key: False for key in sorted(self.module.AUTHORITY_KEYS)}
        return {
            "schema": self.module.EXPECTED_RECIPE_SCHEMA,
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
            "environment": dict(self.module.ALLOWED_ENVIRONMENT),
            "toolchain": {
                "rustc_version": "rustc 1.95.0",
                "rustc_commit_hash": "8" * 40,
                "cargo_version": "cargo 1.95.0",
                "host": "x86_64-unknown-linux-gnu",
                "target": "x86_64-unknown-linux-gnu",
                "rustc_binary_sha256": "9" * 64,
                "cargo_binary_sha256": "a" * 64,
                "linker_kind": "clang",
                "linker_version": "clang 20.1.0",
                "linker_binary_sha256": "b" * 64,
            },
            "build_network": False,
            "source_mutation_allowed": False,
            "runtime_external_network": False,
            "authority": authority,
        }

    def seal(self) -> dict[str, object]:
        source_raw = self.module.canonical_bytes(self.source)
        recipe_raw = self.module.canonical_bytes(self.recipe)
        return self.module.seal(self.source, source_raw, self.recipe, recipe_raw)

    def test_valid_inputs_produce_deterministic_build_not_run_manifest(self) -> None:
        left = self.seal()
        right = self.seal()
        self.assertEqual(left, right)
        self.assertEqual(left["status"], "SEALED_INPUTS_BUILD_NOT_RUN")
        self.assertEqual(left["source"]["tree"], self.module.EXPECTED_TREE)
        self.assertEqual(left["source"]["recomputed_tree"], self.module.EXPECTED_TREE)
        self.assertTrue(left["qualification"]["inputs_sealed"])
        self.assertFalse(left["qualification"]["build_run"])
        self.assertFalse(left["qualification"]["artifact_created"])
        self.assertFalse(left["qualification"]["sbom_created"])
        self.assertFalse(left["qualification"]["servo_runtime_qualified"])
        self.assertTrue(all(value is False for value in left["authority"].values()))
        command = left["build"]["canonical_command"]
        self.assertEqual(command[:5], ["cargo", "build", "--locked", "--offline", "--frozen"])
        self.assertIn("--no-default-features", command)
        self.assertEqual(command[-2:], ["--features", "background-hang-monitor,baked-in-resources"])

    def test_recipe_and_source_digests_bind_exact_canonical_bytes(self) -> None:
        manifest = self.seal()
        self.assertEqual(
            manifest["source"]["source_verification_receipt_sha256"],
            self.module.sha256_bytes(self.module.canonical_bytes(self.source)),
        )
        self.assertEqual(
            manifest["recipe_sha256"],
            self.module.sha256_bytes(self.module.canonical_bytes(self.recipe)),
        )

    def test_recomputed_tree_mismatch_fails_closed(self) -> None:
        self.source["source"]["recomputed_tree"] = "c" * 40
        with self.assertRaises(self.module.BuildInputError):
            self.seal()

    def test_open_source_authority_fails_closed(self) -> None:
        self.source["authority"]["runtime_authority"] = True
        with self.assertRaises(self.module.BuildInputError):
            self.seal()

    def test_unsorted_or_duplicate_features_fail_closed(self) -> None:
        self.recipe["features"] = ["baked-in-resources", "background-hang-monitor"]
        with self.assertRaises(self.module.BuildInputError):
            self.seal()
        self.recipe = self.valid_recipe()
        self.recipe["features"] = ["background-hang-monitor", "background-hang-monitor"]
        with self.assertRaises(self.module.BuildInputError):
            self.seal()

    def test_command_requires_locked_offline_frozen_direct_cargo(self) -> None:
        for invalid in (
            ["cargo", "build", "--locked", "--frozen"],
            ["sh", "-c", "cargo build --locked --offline --frozen"],
            ["cargo", "build", "--locked", "--offline", "--frozen", "--features"],
            ["cargo", "build", "--locked", "--offline", "--frozen", "https://example.invalid"],
        ):
            self.recipe = self.valid_recipe()
            self.recipe["command_prefix"] = invalid
            with self.assertRaises(self.module.BuildInputError):
                self.seal()

    def test_environment_must_equal_allowlist(self) -> None:
        self.recipe["environment"]["HOME"] = "/tmp/home"
        with self.assertRaises(self.module.BuildInputError):
            self.seal()
        self.recipe = self.valid_recipe()
        del self.recipe["environment"]["CARGO_NET_OFFLINE"]
        with self.assertRaises(self.module.BuildInputError):
            self.seal()

    def test_target_and_toolchain_target_must_match(self) -> None:
        self.recipe["toolchain"]["target"] = "aarch64-apple-darwin"
        with self.assertRaises(self.module.BuildInputError):
            self.seal()

    def test_network_source_mutation_and_authority_cannot_be_enabled(self) -> None:
        for key in ("build_network", "source_mutation_allowed", "runtime_external_network"):
            self.recipe = self.valid_recipe()
            self.recipe[key] = True
            with self.assertRaises(self.module.BuildInputError):
                self.seal()
        self.recipe = self.valid_recipe()
        self.recipe["authority"]["promotion"] = True
        with self.assertRaises(self.module.BuildInputError):
            self.seal()

    def test_absolute_and_parent_paths_fail_closed(self) -> None:
        for key, value in (
            ("manifest_path", "/worker/Cargo.toml"),
            ("manifest_path", "../worker/Cargo.toml"),
            ("artifact_path", "target/../outside"),
            ("artifact_path", "C:\\worker.exe"),
        ):
            self.recipe = self.valid_recipe()
            self.recipe[key] = value
            with self.assertRaises(self.module.BuildInputError):
                self.seal()

    def test_atomic_output_is_private_and_refuses_overwrite(self) -> None:
        manifest = self.seal()
        output = self.root / "manifest.json"
        self.module.write_atomic(output, manifest)
        self.assertEqual(stat_mode(output), 0o600)
        with self.assertRaises(self.module.BuildInputError):
            self.module.write_atomic(output, manifest)


def stat_mode(path: Path) -> int:
    return path.stat().st_mode & 0o777


if __name__ == "__main__":
    unittest.main()
