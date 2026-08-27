#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import stat
import subprocess
import sys
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
from hepta_servo_build_manifest_io import SOURCE_ID_DOMAIN, canonical, framed  # noqa: E402

TOOL = ROOT / "scripts/hepta-servo-build-manifest.py"


def write(path: pathlib.Path, value: object) -> None:
    path.write_bytes(canonical(value))


def source() -> dict[str, object]:
    value: dict[str, object] = {
        "schema": "hepta.servo.source_receipt.v1", "phase": "DEVELOPMENT",
        "claim_level": "SOURCE_PIN_AND_TREE_ONLY", "captured_at_utc": "2026-08-27T00:00:00Z",
        "source": {
            "repository": "https://github.com/servo/servo",
            "commit": "0a48e298482659817eb50097df23841f2b8e3044",
            "tree": "b04d2f75b3217374d079d579c270177b57fa1389", "clean_worktree": True,
        },
        "artifact": {"source_archive_created": False, "source_archive_sha256": None,
                     "worker_artifact_built": False, "worker_artifact_sha256": None,
                     "sbom_created": False},
        "authority": {"machine_authority": False, "runtime_authority": False,
                      "production_caller": False, "production_writer": False,
                      "effect_authority": False, "external_effect": False,
                      "operator_acceptance": False, "promotion": False,
                      "release_qualified": False},
        "decision": "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED",
    }
    value["receipt_id"] = "servo-source-receipt:v1:" + framed(SOURCE_ID_DOMAIN, canonical(value))
    return value


class BuildManifestTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory(); self.root = pathlib.Path(self.temp.name)
        self.paths = {name: self.root / name for name in (
            "source.json", "patch.json", "license.json", "sbom.json", "rustc.txt",
            "command.json", "environment.json", "manifest.json")}
        write(self.paths["source.json"], source())
        write(self.paths["patch.json"], {"schema": "hepta.servo.patch_inventory.v1",
              "servo_commit": "0a48e298482659817eb50097df23841f2b8e3044",
              "servo_tree": "b04d2f75b3217374d079d579c270177b57fa1389", "patches": []})
        write(self.paths["license.json"], {"schema": "hepta.servo.license_packet.v1",
              "upstream_repository": "https://github.com/servo/servo",
              "upstream_commit": "0a48e298482659817eb50097df23841f2b8e3044",
              "primary_license": "MPL-2.0", "license_file_sha256": "1" * 64,
              "notices": ["LICENSE"], "source_offer_required": True})
        write(self.paths["sbom.json"], {"spdxVersion": "SPDX-2.3", "dataLicense": "CC0-1.0",
              "SPDXID": "SPDXRef-DOCUMENT", "name": "servo-worker", "packages": [{"name": "servo"}]})
        self.paths["rustc.txt"].write_text(
            "rustc 1.95.0\nbinary: rustc\ncommit-hash: 1\ncommit-date: 2026-08-01\nhost: x86_64-unknown-linux-gnu\nrelease: 1.95.0\nLLVM version: 21.1.0\n", encoding="utf-8")
        write(self.paths["command.json"], {"argv": ["cargo", "build", "--locked", "--release"]})
        write(self.paths["environment.json"], {"environment": {"LC_ALL": "C", "TZ": "UTC",
              "PATH_DIGEST_SHA256": "2" * 64}})

    def tearDown(self) -> None:
        self.temp.cleanup()

    def run_tool(self, command: str, *extra: str) -> subprocess.CompletedProcess[str]:
        args = [sys.executable, str(TOOL), command,
            "--source-receipt", str(self.paths["source.json"]),
            "--patch-inventory", str(self.paths["patch.json"]),
            "--license-packet", str(self.paths["license.json"]),
            "--sbom", str(self.paths["sbom.json"]), "--rustc-verbose", str(self.paths["rustc.txt"]),
            "--build-command", str(self.paths["command.json"]),
            "--environment-allowlist", str(self.paths["environment.json"]),
            "--target-triple", "x86_64-unknown-linux-gnu", "--build-profile", "release-hepta-c1",
            "--cargo-version", "cargo-1.95.0", "--linker-id", "clang-21",
            "--feature", "local-fixture", "--feature", "private-worker", *extra]
        return subprocess.run(args, text=True, capture_output=True, check=False)

    def test_snapshot_verify_is_canonical_sorted_closed_and_create_only(self) -> None:
        result = self.run_tool("snapshot", "--output", str(self.paths["manifest.json"])); self.assertEqual(result.returncode, 0, result.stderr)
        manifest = json.loads(self.paths["manifest.json"].read_bytes())
        self.assertEqual(self.paths["manifest.json"].read_bytes(), canonical(manifest))
        self.assertEqual(manifest["features"], ["local-fixture", "private-worker"])
        for key in ("network_access_during_build", "worker_tcp_listener", "worker_http_surface",
                    "worker_external_network", "worker_credential_export",
                    "worker_production_authority", "worker_effect_authority"):
            self.assertIs(manifest[key], False)
        self.assertEqual(stat.S_IMODE(self.paths["manifest.json"].stat().st_mode), 0o600)
        verified = self.run_tool("verify", "--manifest", str(self.paths["manifest.json"])); self.assertEqual(verified.returncode, 0, verified.stderr)
        second = self.run_tool("snapshot", "--output", str(self.paths["manifest.json"])); self.assertNotEqual(second.returncode, 0)

    def test_duplicate_feature_fails(self) -> None:
        result = self.run_tool("snapshot", "--feature", "local-fixture", "--output", str(self.paths["manifest.json"])); self.assertNotEqual(result.returncode, 0)

    def test_secret_or_unknown_environment_fails(self) -> None:
        write(self.paths["environment.json"], {"environment": {"GITHUB_TOKEN": "secret"}})
        self.assertNotEqual(self.run_tool("snapshot", "--output", str(self.paths["manifest.json"])).returncode, 0)

    def test_positive_source_authority_fails(self) -> None:
        value = source(); value["authority"]["runtime_authority"] = True
        payload = dict(value); payload.pop("receipt_id")
        value["receipt_id"] = "servo-source-receipt:v1:" + framed(SOURCE_ID_DOMAIN, canonical(payload))
        write(self.paths["source.json"], value)
        self.assertNotEqual(self.run_tool("snapshot", "--output", str(self.paths["manifest.json"])).returncode, 0)

    def test_input_tamper_breaks_verification(self) -> None:
        self.assertEqual(self.run_tool("snapshot", "--output", str(self.paths["manifest.json"])).returncode, 0)
        write(self.paths["command.json"], {"argv": ["cargo", "build", "--locked", "--profile", "changed"]})
        self.assertNotEqual(self.run_tool("verify", "--manifest", str(self.paths["manifest.json"])).returncode, 0)

    def test_noncanonical_json_fails(self) -> None:
        self.paths["command.json"].write_text('{"argv": ["cargo", "build"]}\n', encoding="utf-8")
        self.assertNotEqual(self.run_tool("snapshot", "--output", str(self.paths["manifest.json"])).returncode, 0)

    def test_registry_mutating_command_fails(self) -> None:
        write(self.paths["command.json"], {"argv": ["cargo", "publish"]})
        self.assertNotEqual(self.run_tool("snapshot", "--output", str(self.paths["manifest.json"])).returncode, 0)


if __name__ == "__main__":
    unittest.main()
