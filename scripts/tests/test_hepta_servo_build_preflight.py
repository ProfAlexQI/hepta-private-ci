#!/usr/bin/env python3
"""Tests for the Linux-first Servo build preflight."""

from __future__ import annotations

import importlib.util
import os
import stat
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

SCRIPT = Path(__file__).resolve().parents[1] / "hepta-servo-build-preflight.py"


def load_module() -> ModuleType:
    specification = importlib.util.spec_from_file_location("hepta_servo_build_preflight", SCRIPT)
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load build preflight")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class BuildPreflightTests(unittest.TestCase):
    def setUp(self) -> None:
        self.module = load_module()
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name).resolve()
        self.archive = self.write_file("servo-source.tar.gz", b"fixture-source-archive", 0o600)
        self.rustc = self.write_file("rustc", b"rustc-binary", 0o700)
        self.cargo = self.write_file("cargo", b"cargo-binary", 0o700)
        self.linker = self.write_file("clang", b"linker-binary", 0o700)
        self.source_root = self.root / "source-root"
        self.artifact_root = self.root / "artifact-root"
        self.source_root.mkdir(mode=0o700)
        self.artifact_root.mkdir(mode=0o700)
        self.source = self.valid_source()
        self.toolchain = self.valid_toolchain()
        self.recipe = self.valid_recipe()
        self.manifest = self.valid_manifest()

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def write_file(self, name: str, content: bytes, mode: int) -> Path:
        path = self.root / name
        path.write_bytes(content)
        path.chmod(mode)
        return path

    def authority(self) -> dict[str, bool]:
        return {key: False for key in sorted(self.module.AUTHORITY_KEYS)}

    def digest(self, path: Path) -> tuple[str, int]:
        return self.module.sha256_file(path)

    def valid_source(self) -> dict[str, object]:
        archive_sha256, _archive_bytes = self.digest(self.archive)
        return {
            "schema": self.module.SOURCE_VERIFICATION_SCHEMA,
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
            "compressed_archive_sha256": archive_sha256,
            "tar_sha256": "5" * 64,
            "license_packet_sha256": "6" * 64,
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

    def component(self, path: Path, version: str, **extra: object) -> dict[str, object]:
        digest, length = self.digest(path)
        return {
            "version": version,
            "output_sha256": "7" * 64,
            "binary_sha256": digest,
            "binary_bytes": length,
            **extra,
        }

    def valid_toolchain(self) -> dict[str, object]:
        host = self.module.EXPECTED_TARGET
        return {
            "schema": self.module.TOOLCHAIN_SCHEMA,
            "schema_version": 1,
            "target": self.module.EXPECTED_TARGET,
            "host": host,
            "rustc": self.component(
                self.rustc,
                "rustc 1.95.0 (012345678 2026-08-01)",
                release="1.95.0",
                commit_hash="8" * 40,
                host=host,
            ),
            "cargo": self.component(
                self.cargo,
                "cargo 1.95.0 (abcdef123 2026-08-01)",
                release="1.95.0",
                commit_hash="9" * 40,
                host=host,
            ),
            "linker": self.component(
                self.linker,
                "Ubuntu clang version 20.1.0 (build@host)",
                kind="clang",
            ),
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

    def projection(self) -> dict[str, str]:
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
            "schema": self.module.RECIPE_SCHEMA,
            "schema_version": 1,
            "plan_id": "HEPTA-BROWSER-WEB-D",
            "stage": "WEB_C1",
            "status": "FROZEN_INPUTS_NOT_EXECUTED",
            "target": self.module.EXPECTED_TARGET,
            "profile": "release",
            "manifest_path": "worker/Cargo.toml",
            "package": "hepta-servo-worker",
            "artifact_path": "target/x86_64-unknown-linux-gnu/release/hepta-servo-worker",
            "features": ["baked-in-resources", "background-hang-monitor"],
            "default_features": False,
            "jobs": 4,
            "command_prefix": list(self.module.COMMAND_PREFIX),
            "environment": dict(self.module.FROZEN_ENVIRONMENT),
            "toolchain": self.projection(),
            "build_network": False,
            "source_mutation_allowed": False,
            "runtime_external_network": False,
            "authority": self.authority(),
        }

    def normalized_recipe(self) -> dict[str, object]:
        return self.module.validate_recipe(self.recipe, self.projection())

    def valid_manifest(self) -> dict[str, object]:
        source_raw = self.module.canonical_bytes(self.source)
        recipe_raw = self.module.canonical_bytes(self.recipe)
        toolchain_raw = self.module.canonical_bytes(self.toolchain)
        return {
            "schema": self.module.BUILD_INPUT_SCHEMA,
            "schema_version": 1,
            "plan_id": "HEPTA-BROWSER-WEB-D",
            "stage": "WEB_C1",
            "status": "SEALED_INPUTS_BUILD_NOT_RUN",
            "source": {
                "repository": "servo/servo",
                "commit": self.module.EXPECTED_COMMIT,
                "tree": self.module.EXPECTED_TREE,
                "recomputed_tree": self.module.EXPECTED_TREE,
                "source_verification_receipt_sha256": self.module.sha256_bytes(source_raw),
                "source_bundle_receipt_sha256": self.source["bundle_receipt_sha256"],
                "compressed_source_archive_sha256": self.source["compressed_archive_sha256"],
                "source_tar_sha256": self.source["tar_sha256"],
                "license_packet_sha256": self.source["license_packet_sha256"],
                "tree_manifest_sha256": self.source["source"]["tree_manifest_sha256"],
            },
            "recipe_sha256": self.module.sha256_bytes(recipe_raw),
            "toolchain_receipt_sha256": self.module.sha256_bytes(toolchain_raw),
            "build": self.normalized_recipe(),
            "qualification": {
                "inputs_sealed": True,
                "source_tree_independently_verified": True,
                "command_canonicalized": True,
                "environment_allowlisted": True,
                "toolchain_digests_bound": True,
                "toolchain_receipt_independently_captured": True,
                "build_network_disabled": True,
                "build_run": False,
                "artifact_created": False,
                "sbom_created": False,
                "servo_runtime_qualified": False,
                "operator_accepted": False,
                "release_qualified": False,
            },
            "machine_local_paths_included": False,
            "authority": self.authority(),
        }

    def preflight(self) -> dict[str, object]:
        return self.module.run_preflight(
            source=self.source,
            source_raw=self.module.canonical_bytes(self.source),
            recipe=self.recipe,
            recipe_raw=self.module.canonical_bytes(self.recipe),
            toolchain=self.toolchain,
            toolchain_raw=self.module.canonical_bytes(self.toolchain),
            manifest=self.manifest,
            manifest_raw=self.module.canonical_bytes(self.manifest),
            source_archive=self.archive,
            rustc=self.rustc,
            cargo=self.cargo,
            linker=self.linker,
            source_root=self.source_root,
            artifact_root=self.artifact_root,
        )

    def test_valid_inputs_are_ready_only_for_a_separate_bounded_build(self) -> None:
        receipt = self.preflight()
        self.assertEqual(receipt["status"], "READY_FOR_SEPARATE_BOUNDED_BUILD")
        self.assertTrue(receipt["preflight"]["ready_for_separate_bounded_build"])
        self.assertTrue(receipt["preflight"]["toolchain_binaries_rehashed"])
        self.assertFalse(receipt["preflight"]["build_run"])
        self.assertFalse(receipt["preflight"]["artifact_created"])
        self.assertFalse(receipt["preflight"]["sbom_created"])
        self.assertFalse(receipt["preflight"]["servo_runtime_qualified"])
        self.assertTrue(all(value is False for value in receipt["authority"].values()))
        encoded = self.module.canonical_bytes(receipt).decode("utf-8")
        self.assertNotIn(str(self.root), encoded)

    def test_preflight_is_deterministic_for_unchanged_inputs(self) -> None:
        self.assertEqual(self.preflight(), self.preflight())

    def test_tampered_source_archive_fails_closed(self) -> None:
        self.archive.write_bytes(b"tampered")
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()

    def test_source_receipt_digest_mismatch_fails_closed(self) -> None:
        self.manifest["source"]["source_verification_receipt_sha256"] = "0" * 64
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()

    def test_recipe_digest_or_projection_mismatch_fails_closed(self) -> None:
        self.recipe["jobs"] = 5
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()

    def test_toolchain_receipt_digest_mismatch_fails_closed(self) -> None:
        self.manifest["toolchain_receipt_sha256"] = "0" * 64
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()

    def test_toolchain_binary_tamper_fails_closed(self) -> None:
        self.rustc.write_bytes(b"tampered-rustc")
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()

    def test_symlink_or_hardlink_toolchain_binary_fails_closed(self) -> None:
        link = self.root / "linker-link"
        link.symlink_to(self.linker.name)
        with self.assertRaises(self.module.BuildPreflightError):
            self.module.verify_binary(
                link,
                self.toolchain["linker"]["binary_sha256"],
                self.toolchain["linker"]["binary_bytes"],
                "linker",
            )
        hardlink = self.root / "rustc-hardlink"
        os.link(self.rustc, hardlink)
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()

    def test_nonempty_or_public_root_fails_closed(self) -> None:
        (self.source_root / "unexpected").write_text("x", encoding="utf-8")
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()
        (self.source_root / "unexpected").unlink()
        self.artifact_root.chmod(0o750)
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()

    def test_same_source_and_artifact_root_fails_closed(self) -> None:
        with self.assertRaises(self.module.BuildPreflightError):
            self.module.run_preflight(
                source=self.source,
                source_raw=self.module.canonical_bytes(self.source),
                recipe=self.recipe,
                recipe_raw=self.module.canonical_bytes(self.recipe),
                toolchain=self.toolchain,
                toolchain_raw=self.module.canonical_bytes(self.toolchain),
                manifest=self.manifest,
                manifest_raw=self.module.canonical_bytes(self.manifest),
                source_archive=self.archive,
                rustc=self.rustc,
                cargo=self.cargo,
                linker=self.linker,
                source_root=self.source_root,
                artifact_root=self.source_root,
            )

    def test_command_or_environment_drift_fails_closed(self) -> None:
        self.recipe["command_prefix"] = ["cargo", "build", "--locked", "--offline"]
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()
        self.recipe = self.valid_recipe()
        self.manifest = self.valid_manifest()
        self.recipe["environment"]["HOME"] = "/tmp/home"
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()

    def test_open_authority_fails_closed(self) -> None:
        self.manifest["authority"]["runtime_authority"] = True
        with self.assertRaises(self.module.BuildPreflightError):
            self.preflight()

    def test_private_atomic_output_refuses_overwrite(self) -> None:
        receipt = self.preflight()
        output = self.root / "preflight.json"
        self.module.write_atomic(output, receipt)
        self.assertEqual(stat.S_IMODE(output.stat().st_mode), 0o600)
        with self.assertRaises(self.module.BuildPreflightError):
            self.module.write_atomic(output, receipt)


if __name__ == "__main__":
    unittest.main()
