from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import pathlib
import stat
import sys
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts" / "hepta-servo-artifact-receipt.py"
SPEC = importlib.util.spec_from_file_location("hepta_servo_artifact_receipt", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("cannot load Servo artifact receipt module")
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


class ServoArtifactReceiptTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = pathlib.Path(self.temporary.name)

        self.source_receipt = self.root / "source-receipt.json"
        self.patch_inventory = self.root / "patch-inventory.json"
        self.license_packet = self.root / "license-packet.json"
        self.sbom = self.root / "worker.spdx.json"
        self.build_manifest = self.root / "build-manifest.json"
        self.artifact = self.root / "hepta-servo-worker"

        self.write_source_receipt()
        self.write_json(
            self.patch_inventory,
            {
                "schema": MODULE.PATCH_SCHEMA,
                "upstream_commit": MODULE.EXPECTED_SERVO_COMMIT,
                "upstream_tree": MODULE.EXPECTED_SERVO_TREE,
                "patches": [],
            },
        )
        self.write_json(
            self.license_packet,
            {
                "schema": MODULE.LICENSE_SCHEMA,
                "upstream_repository": MODULE.EXPECTED_REPOSITORY,
                "upstream_commit": MODULE.EXPECTED_SERVO_COMMIT,
                "primary_license": "MPL-2.0",
                "license_file_sha256": "4" * 64,
                "notices": ["LICENSE", "NOTICE"],
                "source_offer_required": True,
            },
        )
        self.write_json(
            self.sbom,
            {
                "SPDXID": "SPDXRef-DOCUMENT",
                "spdxVersion": "SPDX-2.3",
                "dataLicense": "CC0-1.0",
                "documentNamespace": "urn:uuid:00000000-0000-4000-8000-000000000001",
                "name": "hepta-servo-worker-fixture",
                "packages": [
                    {
                        "SPDXID": "SPDXRef-Package-servo",
                        "name": "servo-fixture",
                        "versionInfo": "0.0.0",
                    }
                ],
            },
        )
        self.write_elf(self.artifact, machine=62)
        os.chmod(self.artifact, 0o755)
        self.write_build_manifest("x86_64-unknown-linux-gnu")

    def write_json(self, path: pathlib.Path, value: object) -> None:
        path.write_bytes(MODULE.canonical_bytes(value))

    def file_sha256(self, path: pathlib.Path) -> str:
        return hashlib.sha256(path.read_bytes()).hexdigest()

    def write_source_receipt(self) -> None:
        payload = {
            "schema": MODULE.SOURCE_SCHEMA,
            "phase": "DEVELOPMENT",
            "claim_level": "SOURCE_PIN_AND_TREE_ONLY",
            "captured_at_utc": "2026-08-27T00:00:00Z",
            "source": {
                "repository": MODULE.EXPECTED_REPOSITORY,
                "commit": MODULE.EXPECTED_SERVO_COMMIT,
                "tree": MODULE.EXPECTED_SERVO_TREE,
                "clean_worktree": True,
                "embedded_commit_signature": True,
                "tree_manifest": {
                    "algorithm": "sha256-framed-git-ls-tree-v1",
                    "sha256": "1" * 64,
                    "entry_count": 1,
                    "blob_count": 1,
                    "submodule_count": 0,
                    "symlink_count": 0,
                    "path_utf8_bytes": 7,
                },
                "submodules": [],
                "license": {
                    "spdx_id": "MPL-2.0",
                    "path": "LICENSE",
                    "bytes": 100,
                    "sha256": "2" * 64,
                },
            },
            "artifact": {
                "source_archive_created": False,
                "source_archive_sha256": None,
                "worker_artifact_built": False,
                "worker_artifact_sha256": None,
                "sbom_created": False,
            },
            "authority": {
                "machine_authority": False,
                "runtime_authority": False,
                "production_caller": False,
                "production_writer": False,
                "effect_authority": False,
                "external_effect": False,
                "operator_acceptance": False,
                "promotion": False,
                "release_qualified": False,
            },
            "decision": "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED",
        }
        identifier = MODULE.framed_digest(
            MODULE.SOURCE_RECEIPT_DOMAIN,
            [MODULE.canonical_bytes(payload)],
        )
        payload["receipt_id"] = f"servo-source-receipt:v1:{identifier}"
        self.write_json(self.source_receipt, payload)

    def build_manifest_value(self, target: str) -> dict[str, object]:
        source_value = json.loads(self.source_receipt.read_bytes())
        return {
            "schema": MODULE.BUILD_SCHEMA,
            "source_receipt_id": source_value["receipt_id"],
            "source_receipt_sha256": self.file_sha256(self.source_receipt),
            "target_triple": target,
            "build_profile": "release",
            "rustc_verbose_sha256": "5" * 64,
            "cargo_version": "cargo-1.95.0",
            "linker_id": "clang-18.1.0",
            "features": ["hepta-private-worker", "local-fixture-only"],
            "build_command_sha256": "6" * 64,
            "environment_allowlist_sha256": "7" * 64,
            "patch_inventory_sha256": self.file_sha256(self.patch_inventory),
            "license_packet_sha256": self.file_sha256(self.license_packet),
            "sbom_sha256": self.file_sha256(self.sbom),
            "sbom_format": "SPDX-2.3-json",
            "network_access_during_build": False,
            "worker_tcp_listener": False,
            "worker_http_surface": False,
            "worker_external_network": False,
            "worker_credential_export": False,
            "worker_production_authority": False,
            "worker_effect_authority": False,
        }

    def write_build_manifest(self, target: str) -> None:
        self.write_json(self.build_manifest, self.build_manifest_value(target))

    def create_receipt(self) -> dict[str, object]:
        return MODULE.artifact_receipt(
            source_receipt_path=self.source_receipt,
            build_manifest_path=self.build_manifest,
            artifact_path=self.artifact,
            patch_inventory_path=self.patch_inventory,
            license_packet_path=self.license_packet,
            sbom_path=self.sbom,
            captured_at="2026-08-27T00:00:00Z",
        )

    def write_elf(self, path: pathlib.Path, machine: int) -> None:
        header = bytearray(128)
        header[0:4] = b"\x7fELF"
        header[4] = 2
        header[5] = 1
        header[6] = 1
        header[16:18] = (2).to_bytes(2, "little")
        header[18:20] = machine.to_bytes(2, "little")
        path.write_bytes(header)

    def write_macho(self, path: pathlib.Path, cpu_type: int) -> None:
        header = bytearray(128)
        header[0:4] = bytes.fromhex("cffaedfe")
        header[4:8] = cpu_type.to_bytes(4, "little")
        path.write_bytes(header)

    def write_pe(self, path: pathlib.Path, machine: int) -> None:
        header = bytearray(512)
        header[0:2] = b"MZ"
        pe_offset = 0x80
        header[0x3C:0x40] = pe_offset.to_bytes(4, "little")
        header[pe_offset : pe_offset + 4] = b"PE\0\0"
        header[pe_offset + 4 : pe_offset + 6] = machine.to_bytes(2, "little")
        path.write_bytes(header)

    def test_artifact_receipt_binds_all_inputs_without_runtime_claim(self) -> None:
        receipt = self.create_receipt()
        self.assertEqual(receipt["decision"], "ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED")
        self.assertEqual(receipt["artifact"]["format"], "elf")
        self.assertEqual(receipt["artifact"]["architecture"], "x86_64")
        self.assertEqual(receipt["artifact"]["sha256"], self.file_sha256(self.artifact))
        self.assertEqual(receipt["supporting_inputs"]["patch_count"], 0)
        self.assertEqual(receipt["supporting_inputs"]["sbom_package_count"], 1)
        self.assertTrue(all(value is False for value in receipt["authority"].values()))
        self.assertTrue(
            all(value is False for value in receipt["runtime_qualification"].values())
        )
        self.assertRegex(
            receipt["receipt_id"],
            r"^servo-worker-artifact-receipt:v1:[0-9a-f]{64}$",
        )

    def test_binary_format_and_target_must_match(self) -> None:
        self.write_build_manifest("aarch64-unknown-linux-gnu")
        with self.assertRaisesRegex(MODULE.ArtifactError, "does not match target"):
            self.create_receipt()

        self.write_macho(self.artifact, 0x0100000C)
        os.chmod(self.artifact, 0o755)
        self.write_build_manifest("aarch64-apple-darwin")
        receipt = self.create_receipt()
        self.assertEqual(receipt["artifact"]["format"], "macho")
        self.assertEqual(receipt["artifact"]["architecture"], "aarch64")

        self.write_pe(self.artifact, 0x8664)
        os.chmod(self.artifact, 0o755)
        self.write_build_manifest("x86_64-pc-windows-msvc")
        receipt = self.create_receipt()
        self.assertEqual(receipt["artifact"]["format"], "pe")
        self.assertEqual(receipt["artifact"]["architecture"], "x86_64")

    def test_positive_build_or_worker_capability_fails_closed(self) -> None:
        for field in (
            "network_access_during_build",
            "worker_tcp_listener",
            "worker_http_surface",
            "worker_external_network",
            "worker_credential_export",
            "worker_production_authority",
            "worker_effect_authority",
        ):
            manifest = self.build_manifest_value("x86_64-unknown-linux-gnu")
            manifest[field] = True
            self.write_json(self.build_manifest, manifest)
            with self.assertRaisesRegex(MODULE.ArtifactError, field):
                self.create_receipt()

    def test_supporting_input_digest_tamper_fails_closed(self) -> None:
        self.license_packet.write_bytes(self.license_packet.read_bytes() + b"\n")
        with self.assertRaisesRegex(MODULE.ArtifactError, "not compact canonical|does not match"):
            self.create_receipt()

    def test_artifact_permissions_symlink_and_hardlink_fail_closed(self) -> None:
        os.chmod(self.artifact, 0o777)
        with self.assertRaisesRegex(MODULE.ArtifactError, "group/world writable"):
            self.create_receipt()

        os.chmod(self.artifact, 0o755)
        linked = self.root / "worker-hardlink"
        os.link(self.artifact, linked)
        with self.assertRaisesRegex(MODULE.ArtifactError, "hard link"):
            self.create_receipt()
        linked.unlink()

        target = self.root / "worker-target"
        self.artifact.rename(target)
        self.artifact.symlink_to(target)
        with self.assertRaisesRegex(MODULE.ArtifactError, "non-symlink"):
            self.create_receipt()

    def test_source_receipt_authority_or_identifier_tamper_fails_closed(self) -> None:
        source = json.loads(self.source_receipt.read_bytes())
        source["authority"]["runtime_authority"] = True
        self.write_json(self.source_receipt, source)
        self.write_build_manifest("x86_64-unknown-linux-gnu")
        with self.assertRaisesRegex(MODULE.ArtifactError, "positive authority"):
            self.create_receipt()

        self.write_source_receipt()
        source = json.loads(self.source_receipt.read_bytes())
        source["captured_at_utc"] = "2026-08-27T00:00:01Z"
        self.write_json(self.source_receipt, source)
        self.write_build_manifest("x86_64-unknown-linux-gnu")
        with self.assertRaisesRegex(MODULE.ArtifactError, "ID does not match"):
            self.create_receipt()

    def test_patch_and_license_inventories_are_strict(self) -> None:
        patch = {
            "schema": MODULE.PATCH_SCHEMA,
            "upstream_commit": MODULE.EXPECTED_SERVO_COMMIT,
            "upstream_tree": MODULE.EXPECTED_SERVO_TREE,
            "patches": [
                {
                    "id": "P-002",
                    "path": "patches/second.patch",
                    "sha256": "8" * 64,
                    "reason": "fixture",
                    "upstream_reference": "none",
                    "deletion_condition": "upstreamed",
                },
                {
                    "id": "P-001",
                    "path": "patches/first.patch",
                    "sha256": "9" * 64,
                    "reason": "fixture",
                    "upstream_reference": "none",
                    "deletion_condition": "upstreamed",
                },
            ],
        }
        self.write_json(self.patch_inventory, patch)
        with self.assertRaisesRegex(MODULE.ArtifactError, "sorted and unique"):
            MODULE.validate_patch_inventory(self.patch_inventory)

        packet = json.loads(self.license_packet.read_bytes())
        packet["source_offer_required"] = False
        self.write_json(self.license_packet, packet)
        with self.assertRaisesRegex(MODULE.ArtifactError, "source-distribution"):
            MODULE.validate_license_packet(self.license_packet)


if __name__ == "__main__":
    unittest.main()
