#!/usr/bin/env python3
"""Tests for the independent Servo source pipeline."""

from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

SCRIPT = Path(__file__).resolve().parents[1] / "hepta-servo-independent-source.py"


def load_module() -> ModuleType:
    specification = importlib.util.spec_from_file_location("hepta_servo_independent_source", SCRIPT)
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load source-pipeline module")
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


class IndependentSourcePipelineTests(unittest.TestCase):
    def setUp(self) -> None:
        self.module = load_module()
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
        assets = source / "assets"
        assets.mkdir()
        (assets / "page.html").write_text("<title>fixture</title>\n", encoding="utf-8")
        environment = os.environ.copy()
        environment.update(
            {
                "GIT_AUTHOR_DATE": "2026-08-28T00:00:00Z",
                "GIT_COMMITTER_DATE": "2026-08-28T00:00:00Z",
            }
        )
        subprocess.run(["git", "-C", os.fspath(source), "add", "."], check=True, env=environment)
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

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def run_pipeline(self, name: str = "output") -> dict[str, object]:
        return self.module.execute_pipeline(
            repository_url=os.fspath(self.origin),
            output_dir=self.root / name,
            pin_path=self.pin,
            patch_inventory_path=self.patches,
            keep_checkouts=False,
            allow_local_test_origin=True,
        )

    def test_two_independent_fetches_produce_identical_archives(self) -> None:
        bundle = self.run_pipeline()
        self.assertFalse(bundle["canonical"])
        self.assertTrue(bundle["independence"]["roots_distinct"])
        self.assertTrue(bundle["independence"]["object_stores_distinct"])
        self.assertTrue(bundle["qualification"]["deterministic_tar_verified"])
        self.assertTrue(bundle["qualification"]["deterministic_gzip_verified"])
        self.assertFalse(bundle["qualification"]["servo_built"])
        self.assertFalse(bundle["qualification"]["servo_runtime_qualified"])
        self.assertTrue(all(value is False for value in bundle["authority"].values()))
        output = self.root / "output"
        self.assertTrue((output / "servo-source-a.tar").is_file())
        self.assertTrue((output / "servo-source-a.tar.gz").is_file())
        self.assertFalse((output / "fetch-a").exists())
        receipt_text = (output / "independent-source-bundle.receipt.json").read_text(
            encoding="utf-8"
        )
        self.assertNotIn(os.fspath(self.root), receipt_text)

    def test_same_fixture_repo_produces_stable_source_projection_and_archive(self) -> None:
        left = self.run_pipeline("left")
        right = self.run_pipeline("right")
        self.assertEqual(left["source"], right["source"])
        self.assertEqual(left["archive"], right["archive"])
        self.assertEqual(left["license_packet"], right["license_packet"])

    def test_local_origin_is_rejected_without_explicit_test_mode(self) -> None:
        pin = self.module.load_pin(self.pin)
        with self.assertRaises(self.module.SourcePipelineError):
            self.module.acquire_source(
                os.fspath(self.origin),
                self.root / "rejected",
                pin,
                allow_local_test_origin=False,
            )

    def test_dirty_checkout_fails_closed(self) -> None:
        pin = self.module.load_pin(self.pin)
        destination = self.root / "dirty"
        self.module.acquire_source(
            os.fspath(self.origin),
            destination,
            pin,
            allow_local_test_origin=True,
        )
        (destination / "README.md").write_text("changed\n", encoding="utf-8")
        with self.assertRaises(self.module.SourcePipelineError):
            self.module.assert_clean_checkout(destination, pin)

    def test_alternate_object_database_fails_closed(self) -> None:
        pin = self.module.load_pin(self.pin)
        destination = self.root / "alternates"
        self.module.acquire_source(
            os.fspath(self.origin),
            destination,
            pin,
            allow_local_test_origin=True,
        )
        info = destination / ".git/objects/info"
        info.mkdir(parents=True, exist_ok=True)
        (info / "alternates").write_text("/tmp/shared-objects\n", encoding="utf-8")
        with self.assertRaises(self.module.SourcePipelineError):
            self.module.assert_clean_checkout(destination, pin)

    def test_unsafe_archive_symlink_is_rejected(self) -> None:
        source = self.root / "unsafe-source"
        source.mkdir()
        git(source, "init", "--quiet")
        git(source, "config", "user.name", "Hepta Qualification")
        git(source, "config", "user.email", "hepta@example.invalid")
        (source / "LICENSE").write_text(
            "Mozilla Public License Version 2.0\n",
            encoding="utf-8",
        )
        os.symlink("../../outside", source / "escape")
        git(source, "add", ".")
        git(source, "commit", "--quiet", "-m", "unsafe")
        commit = git(source, "rev-parse", "HEAD")
        with self.assertRaises(self.module.SourcePipelineError):
            self.module.git_archive(source, commit, self.root / "unsafe.tar")

    def test_canonical_bytes_and_framing_are_order_independent(self) -> None:
        left = self.module.canonical_bytes({"z": 1, "a": {"y": 2, "x": 3}})
        right = self.module.canonical_bytes({"a": {"x": 3, "y": 2}, "z": 1})
        self.assertEqual(left, right)
        self.assertEqual(
            self.module.framed_sha256(b"domain", [b"a", b"bc"]),
            self.module.framed_sha256(b"domain", [b"a", b"bc"]),
        )
        self.assertNotEqual(
            self.module.framed_sha256(b"domain", [b"a", b"bc"]),
            self.module.framed_sha256(b"domain", [b"ab", b"c"]),
        )


if __name__ == "__main__":
    unittest.main()
