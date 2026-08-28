#!/usr/bin/env python3
"""Tests for independent verification of the Servo source bundle."""

from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

SCRIPTS = Path(__file__).resolve().parents[1]
PIPELINE_ENTRYPOINT_PATH = SCRIPTS / "hepta-servo-independent-source-v2.py"
VERIFY_PATH = SCRIPTS / "hepta-servo-source-bundle-verify.py"


def load(path: Path, name: str) -> ModuleType:
    specification = importlib.util.spec_from_file_location(name, path)
    if specification is None or specification.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def git(cwd: Path, *arguments: str) -> str:
    result = subprocess.run(
        ["git", "-C", os.fspath(cwd), *arguments],
        check=True,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )
    return result.stdout.strip()


class SourceBundleVerificationTests(unittest.TestCase):
    def setUp(self) -> None:
        entrypoint = load(
            PIPELINE_ENTRYPOINT_PATH,
            "hepta_servo_pipeline_v2_entrypoint_for_verify",
        )
        self.pipeline = entrypoint.load_base()
        self.verify = load(VERIFY_PATH, "hepta_servo_bundle_verify")
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name).resolve()
        source = self.root / "source"
        source.mkdir()
        git(source, "init", "--quiet")
        git(source, "config", "user.name", "Hepta Qualification")
        git(source, "config", "user.email", "hepta@example.invalid")
        (source / "LICENSE").write_text(
            "Mozilla Public License Version 2.0\nfixture license body\n",
            encoding="utf-8",
        )
        (source / "Cargo.toml").write_text("[workspace]\nresolver='2'\n", encoding="utf-8")
        (source / "README.md").write_text("fixture\n", encoding="utf-8")
        executable = source / "run-fixture"
        executable.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
        executable.chmod(0o755)
        assets = source / "assets"
        assets.mkdir()
        (assets / "page.html").write_text("<title>fixture</title>\n", encoding="utf-8")
        git(source, "add", ".")
        environment = os.environ.copy()
        environment.update(
            {
                "GIT_AUTHOR_DATE": "2026-08-28T00:00:00Z",
                "GIT_COMMITTER_DATE": "2026-08-28T00:00:00Z",
            }
        )
        subprocess.run(
            ["git", "-C", os.fspath(source), "commit", "--quiet", "-m", "fixture"],
            check=True,
            env=environment,
        )
        self.commit = git(source, "rev-parse", "HEAD")
        self.tree = git(source, "rev-parse", "HEAD^{tree}")
        origin = self.root / "origin.git"
        subprocess.run(
            ["git", "clone", "--quiet", "--bare", os.fspath(source), os.fspath(origin)],
            check=True,
        )
        self.origin = origin
        self.pin = self.root / "pin.json"
        self.pin.write_text(
            json.dumps(
                {
                    "schema": "hepta.browser.servo_upstream_pin.v1",
                    "schema_version": 1,
                    "repository": "servo/servo",
                    "repository_url": "https://github.com/servo/servo",
                    "commit": self.commit,
                    "tree": self.tree,
                    "license": "MPL-2.0",
                    "integration_status": "SOURCE_PIN_ONLY_NOT_IMPORTED",
                    "authority": {
                        "runtime_authority": False,
                        "external_network": False,
                        "production_caller": False,
                        "promotion": False,
                    },
                },
                sort_keys=True,
                separators=(",", ":"),
            ),
            encoding="utf-8",
        )
        self.patches = self.root / "patches.json"
        self.patches.write_text(
            json.dumps(
                {
                    "schema": "hepta.browser.servo_patch_inventory.v1",
                    "schema_version": 1,
                    "servo_commit": self.commit,
                    "servo_tree": self.tree,
                    "patches": [],
                },
                sort_keys=True,
                separators=(",", ":"),
            ),
            encoding="utf-8",
        )
        self.bundle_dir = self.root / "bundle"
        bundle = self.pipeline.execute_pipeline(
            repository_url=os.fspath(origin),
            output_dir=self.bundle_dir,
            pin_path=self.pin,
            patch_inventory_path=self.patches,
            keep_checkouts=False,
            allow_local_test_origin=True,
        )
        for slot in ("a", "b"):
            path = self.bundle_dir / f"fetch-{slot}.receipt.json"
            receipt = json.loads(path.read_text(encoding="utf-8"))
            receipt["canonical"] = True
            receipt["acquisition"]["origin_kind"] = "pinned_https"
            receipt["acquisition"]["network_access_used"] = True
            path.write_bytes(self.pipeline.canonical_bytes(receipt))
        bundle["canonical"] = True
        bundle["acquisition_network_used"] = True
        bundle["qualification"]["canonical_source_acquired"] = True
        for summary in bundle["fetch_receipts"]:
            path = self.bundle_dir / f"fetch-{summary['slot']}.receipt.json"
            raw = path.read_bytes()
            summary["sha256"] = self.pipeline.sha256_bytes(raw)
            summary["bytes"] = len(raw)
            summary["canonical"] = True
        (self.bundle_dir / "independent-source-bundle.receipt.json").write_bytes(
            self.pipeline.canonical_bytes(bundle)
        )

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def test_offline_verifier_recomputes_the_exact_git_tree(self) -> None:
        output = self.root / "verification.json"
        result = self.verify.verify_bundle(self.bundle_dir, self.pin, output)
        self.assertEqual(result["source"]["tree"], self.tree)
        self.assertEqual(result["source"]["recomputed_tree"], self.tree)
        self.assertTrue(result["verification"]["git_tree_recomputed"])
        self.assertFalse(result["verification"]["servo_built"])
        self.assertFalse(result["verification"]["servo_runtime_qualified"])
        self.assertTrue(all(value is False for value in result["authority"].values()))
        self.assertTrue(output.is_file())

    def test_tampered_compressed_archive_fails(self) -> None:
        archive = self.bundle_dir / "servo-source-a.tar.gz"
        with archive.open("ab") as handle:
            handle.write(b"tamper")
        with self.assertRaises(self.verify.BundleVerificationError):
            self.verify.verify_bundle(self.bundle_dir, self.pin, None)

    def test_tampered_fetch_receipt_fails(self) -> None:
        receipt = self.bundle_dir / "fetch-a.receipt.json"
        value = json.loads(receipt.read_text(encoding="utf-8"))
        value["source"]["license_bytes"] += 1
        receipt.write_bytes(self.pipeline.canonical_bytes(value))
        with self.assertRaises(self.verify.BundleVerificationError):
            self.verify.verify_bundle(self.bundle_dir, self.pin, None)

    def test_concatenated_gzip_member_fails(self) -> None:
        archive = self.bundle_dir / "servo-source-a.tar.gz"
        archive.write_bytes(archive.read_bytes() + archive.read_bytes())
        with self.assertRaises(self.verify.BundleVerificationError):
            self.verify.verify_bundle(self.bundle_dir, self.pin, None)

    def test_noncanonical_bundle_json_fails(self) -> None:
        receipt = self.bundle_dir / "independent-source-bundle.receipt.json"
        value = json.loads(receipt.read_text(encoding="utf-8"))
        receipt.write_text(json.dumps(value, indent=2), encoding="utf-8")
        with self.assertRaises(self.verify.BundleVerificationError):
            self.verify.verify_bundle(self.bundle_dir, self.pin, None)


if __name__ == "__main__":
    unittest.main()
