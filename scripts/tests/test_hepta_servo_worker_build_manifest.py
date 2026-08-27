#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
import pathlib
import tempfile
import unittest
from types import ModuleType

ROOT = pathlib.Path(__file__).resolve().parents[2]
TOOL_PATH = ROOT / "scripts/hepta-servo-worker-build-manifest.py"


def load_tool() -> ModuleType:
    spec = importlib.util.spec_from_file_location("hepta_servo_worker_build_manifest", TOOL_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load build manifest tool")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class BuildManifestTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tool = load_tool()
        self.directory = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self.directory.name)
        self.paths = self._write_inputs()

    def tearDown(self) -> None:
        self.directory.cleanup()

    def _write(self, name: str, value: object) -> pathlib.Path:
        path = self.root / name
        path.write_bytes(self.tool.canonical(value))
        path.chmod(0o600)
        return path

    def _source_receipt(self) -> dict[str, object]:
        value: dict[str, object] = {
            "schema": self.tool.SOURCE_SCHEMA,
            "phase": "DEVELOPMENT",
            "claim_level": "SOURCE_PIN_AND_TREE_ONLY",
            "captured_at_utc": "2026-08-28T00:00:00Z",
            "source": {
                "repository": self.tool.REPOSITORY,
                "commit": self.tool.COMMIT,
                "tree": self.tool.TREE,
                "clean_worktree": True,
                "embedded_commit_signature": True,
                "tree_manifest": {
                    "algorithm": "sha256-framed-git-ls-tree-v1",
                    "sha256": "1" * 64,
                    "entry_count": 3,
                    "blob_count": 3,
                    "submodule_count": 0,
                    "symlink_count": 0,
                    "path_utf8_bytes": 24,
                },
                "submodules": [],
                "license": {
                    "spdx_id": "MPL-2.0",
                    "path": "LICENSE",
                    "bytes": 1024,
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
        value["receipt_id"] = "servo-source-receipt:v1:" + self.tool.framed_digest(
            self.tool.SOURCE_ID_DOMAIN, [self.tool.canonical(value)]
        )
        return value

    def _source_bundle(self) -> dict[str, object]:
        value: dict[str, object] = {
            "schema": self.tool.BUNDLE_SCHEMA,
            "phase": "DEVELOPMENT",
            "claim_level": "SOURCE_BUNDLE_RECOMPUTED_ONLY",
            "captured_at_utc": "2026-08-28T00:01:00Z",
            "source": {
                "repository": self.tool.REPOSITORY,
                "commit": self.tool.COMMIT,
                "tree": self.tool.TREE,
                "source_facts_sha256": "3" * 64,
                "source_comparison_receipt_sha256": "4" * 64,
            },
            "archive": {
                "sha256": "5" * 64,
                "bytes": 1234,
                "archive_comparison_receipt_sha256": "6" * 64,
            },
            "distribution": {
                "license_packet_sha256": "7" * 64,
                "patch_inventory_sha256": "8" * 64,
                "distribution_receipt_sha256": "9" * 64,
                "binary_distribution_authorized": False,
            },
            "artifact": {
                "worker_artifact_built": False,
                "sbom_created": False,
                "runtime_qualified": False,
            },
            "authority": self.tool.AUTHORITY,
            "decision": "SOURCE_BUNDLE_RECOMPUTED_BUILD_AND_RUNTIME_NOT_QUALIFIED",
        }
        value["receipt_id"] = "servo-source-bundle-verification:v1:" + self.tool.framed_digest(
            self.tool.BUNDLE_ID_DOMAIN, [self.tool.canonical(value)]
        )
        return value

    def _write_inputs(self) -> dict[str, pathlib.Path]:
        source = self._write("source.json", self._source_receipt())
        bundle = self._write("bundle.json", self._source_bundle())
        patch = self._write(
            "patch.json",
            {
                "schema": self.tool.PATCH_SCHEMA,
                "upstream_commit": self.tool.COMMIT,
                "upstream_tree": self.tool.TREE,
                "patches": [],
            },
        )
        license_packet = self._write(
            "license.json",
            {
                "schema": self.tool.LICENSE_SCHEMA,
                "upstream_repository": self.tool.REPOSITORY,
                "upstream_commit": self.tool.COMMIT,
                "primary_license": "MPL-2.0",
                "license_file_sha256": "a" * 64,
                "notices": ["LICENSE", "MPL-2.0"],
                "source_offer_required_by_project_policy": True,
                "legal_review_required_before_binary_distribution": True,
                "binary_distribution_authorized": False,
            },
        )
        sbom = self._write(
            "sbom.json",
            {
                "spdxVersion": "SPDX-2.3",
                "dataLicense": "CC0-1.0",
                "SPDXID": "SPDXRef-DOCUMENT",
                "name": "hepta-servo-worker-fixture",
                "documentNamespace": "https://hepta.invalid/spdx/fixture",
                "creationInfo": {"created": "2026-08-28T00:00:00Z", "creators": ["Tool: fixture"]},
                "packages": [
                    {"name": "servo", "SPDXID": "SPDXRef-Package-servo", "downloadLocation": "NOASSERTION"},
                    {"name": "worker", "SPDXID": "SPDXRef-Package-worker", "downloadLocation": "NOASSERTION"},
                ],
            },
        )
        command = self._write(
            "command.json",
            {
                "schema": self.tool.COMMAND_SCHEMA,
                "working_directory": "servo-source",
                "argv": [
                    "cargo",
                    "build",
                    "--locked",
                    "--offline",
                    "--target",
                    "x86_64-unknown-linux-gnu",
                    "--profile",
                    "hepta-worker-release",
                ],
                "network_access_during_build": False,
            },
        )
        environment = self._write(
            "environment.json",
            {
                "schema": self.tool.ENVIRONMENT_SCHEMA,
                "variables": {
                    "CARGO_NET_OFFLINE": "true",
                    "LANG": "C.UTF-8",
                    "SOURCE_DATE_EPOCH": "1787836800",
                    "TZ": "UTC",
                },
                "network_access_during_build": False,
            },
        )
        text_files: dict[str, pathlib.Path] = {}
        for name, text in {
            "rustc.txt": "rustc 1.95.0\nbinary: rustc\ncommit-hash: fixture\n",
            "cargo.txt": "cargo-1.95.0\n",
            "linker.txt": "clang-18.1.0\n",
        }.items():
            path = self.root / name
            path.write_text(text, encoding="utf-8")
            path.chmod(0o600)
            text_files[name] = path
        return {
            "source": source,
            "bundle": bundle,
            "patch": patch,
            "license": license_packet,
            "sbom": sbom,
            "command": command,
            "environment": environment,
            "rustc": text_files["rustc.txt"],
            "cargo": text_files["cargo.txt"],
            "linker": text_files["linker.txt"],
        }

    def _args(self, command: str) -> list[str]:
        common = [
            "--source-receipt", str(self.paths["source"]),
            "--source-bundle", str(self.paths["bundle"]),
            "--patch-inventory", str(self.paths["patch"]),
            "--license-packet", str(self.paths["license"]),
            "--sbom", str(self.paths["sbom"]),
            "--rustc-verbose", str(self.paths["rustc"]),
            "--cargo-version", str(self.paths["cargo"]),
            "--linker-id", str(self.paths["linker"]),
            "--build-command", str(self.paths["command"]),
            "--environment", str(self.paths["environment"]),
            "--target-triple", "x86_64-unknown-linux-gnu",
            "--build-profile", "hepta-worker-release",
            "--feature", "local-fixture",
            "--feature", "private-worker",
        ]
        if command == "create":
            return [
                "create",
                *common,
                "--captured-at", "2026-08-28T00:02:00Z",
                "--output-packet", str(self.root / "packet.json"),
                "--output-manifest", str(self.root / "manifest.json"),
            ]
        return [
            "verify",
            *common,
            "--packet", str(self.root / "packet.json"),
            "--manifest", str(self.root / "manifest.json"),
        ]

    def test_create_and_verify_recompute_exact_inputs(self) -> None:
        self.assertEqual(self.tool.main(self._args("create")), 0)
        self.assertEqual(self.tool.main(self._args("verify")), 0)
        packet = json.loads((self.root / "packet.json").read_bytes())
        manifest = json.loads((self.root / "manifest.json").read_bytes())
        self.assertEqual(packet["authority"], self.tool.AUTHORITY)
        self.assertFalse(packet["artifact"]["worker_artifact_built"])
        self.assertEqual(manifest["features"], ["local-fixture", "private-worker"])
        self.assertFalse(manifest["network_access_during_build"])
        self.assertNotIn(str(self.root), (self.root / "packet.json").read_text())

    def test_output_is_create_only(self) -> None:
        self.assertEqual(self.tool.main(self._args("create")), 0)
        self.assertEqual(self.tool.main(self._args("create")), 1)

    def test_tampered_environment_fails_recompute(self) -> None:
        self.assertEqual(self.tool.main(self._args("create")), 0)
        value = json.loads(self.paths["environment"].read_bytes())
        value["variables"]["LANG"] = "en_US.UTF-8"
        self.paths["environment"].write_bytes(self.tool.canonical(value))
        self.assertEqual(self.tool.main(self._args("verify")), 1)

    def test_unknown_environment_key_fails_closed(self) -> None:
        value = json.loads(self.paths["environment"].read_bytes())
        value["variables"]["SECRET_TOKEN"] = "do-not-accept"
        value["variables"] = dict(sorted(value["variables"].items()))
        self.paths["environment"].write_bytes(self.tool.canonical(value))
        self.assertEqual(self.tool.main(self._args("create")), 1)

    def test_positive_build_network_fails_closed(self) -> None:
        value = json.loads(self.paths["command"].read_bytes())
        value["network_access_during_build"] = True
        self.paths["command"].write_bytes(self.tool.canonical(value))
        self.assertEqual(self.tool.main(self._args("create")), 1)

    def test_noncanonical_supporting_json_fails_closed(self) -> None:
        value = json.loads(self.paths["patch"].read_bytes())
        self.paths["patch"].write_text(json.dumps(value, indent=2), encoding="utf-8")
        self.assertEqual(self.tool.main(self._args("create")), 1)

    def test_absolute_path_in_build_command_fails_closed(self) -> None:
        value = json.loads(self.paths["command"].read_bytes())
        value["argv"].append("/tmp/output")
        self.paths["command"].write_bytes(self.tool.canonical(value))
        self.assertEqual(self.tool.main(self._args("create")), 1)


if __name__ == "__main__":
    unittest.main()
