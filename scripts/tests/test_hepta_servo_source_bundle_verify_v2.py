#!/usr/bin/env python3
"""Tests for Git-compatible tree ordering in source-bundle verifier v2."""

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
PIPELINE_V2 = SCRIPTS / "hepta-servo-independent-source-v2.py"
VERIFY_V2 = SCRIPTS / "hepta-servo-source-bundle-verify-v2.py"


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


class SourceBundleVerifierV2Tests(unittest.TestCase):
    def setUp(self) -> None:
        self.pipeline_entry = load(PIPELINE_V2, "hepta_source_pipeline_v2_for_verify_v2")
        self.pipeline = self.pipeline_entry.load_base()
        self.verifier_entry = load(VERIFY_V2, "hepta_source_bundle_verify_v2")
        self.verifier = self.verifier_entry.load_base()
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name).resolve()

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def create_source(self) -> tuple[Path, str, str]:
        source = self.root / "source"
        source.mkdir()
        git(source, "init", "--quiet")
        git(source, "config", "user.name", "Hepta Qualification")
        git(source, "config", "user.email", "hepta@example.invalid")
        (source / "LICENSE").write_text(
            "Mozilla Public License Version 2.0\nfixture license body\n",
            encoding="utf-8",
        )
        # `foo.rs` must sort before tree `foo/` under Git's tree ordering even
        # though ordinary Python bytes sort the shorter `foo` first.
        (source / "foo.rs").write_text("file\n", encoding="utf-8")
        directory = source / "foo"
        directory.mkdir()
        (directory / "bar.rs").write_text("nested\n", encoding="utf-8")
        (source / "z-last").write_text("last\n", encoding="utf-8")
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
        return source, git(source, "rev-parse", "HEAD"), git(source, "rev-parse", "HEAD^{tree}")

    def make_inputs(self, source: Path, commit: str, tree: str) -> tuple[Path, Path, Path]:
        origin = self.root / "origin.git"
        subprocess.run(
            ["git", "clone", "--quiet", "--bare", os.fspath(source), os.fspath(origin)],
            check=True,
        )
        pin = self.root / "pin.json"
        pin.write_text(
            json.dumps(
                {
                    "schema": "hepta.browser.servo_upstream_pin.v1",
                    "schema_version": 1,
                    "repository": "servo/servo",
                    "repository_url": "https://github.com/servo/servo",
                    "commit": commit,
                    "tree": tree,
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
        patches = self.root / "patches.json"
        patches.write_text(
            json.dumps(
                {
                    "schema": "hepta.browser.servo_patch_inventory.v1",
                    "schema_version": 1,
                    "servo_commit": commit,
                    "servo_tree": tree,
                    "patches": [],
                },
                sort_keys=True,
                separators=(",", ":"),
            ),
            encoding="utf-8",
        )
        return origin, pin, patches

    def test_v2_tree_order_matches_git_for_file_directory_prefix_collision(self) -> None:
        source, _commit, expected_tree = self.create_source()
        root = self.verifier.TreeNode()
        root.add_file(b"foo.rs", b"100644", self.verifier.git_hash(b"blob", b"file\n"))
        child = root.directory(b"foo")
        child.add_file(b"bar.rs", b"100644", self.verifier.git_hash(b"blob", b"nested\n"))
        root.add_file(b"LICENSE", b"100644", self.verifier.git_hash(
            b"blob", b"Mozilla Public License Version 2.0\nfixture license body\n"
        ))
        root.add_file(b"z-last", b"100644", self.verifier.git_hash(b"blob", b"last\n"))
        self.assertEqual(root.object_id().hex(), expected_tree)
        self.assertEqual(git(source, "rev-parse", "HEAD^{tree}"), expected_tree)

    def test_v1_plain_sort_would_not_match_prefix_collision_tree(self) -> None:
        source, _commit, expected_tree = self.create_source()
        v1 = load(SCRIPTS / "hepta-servo-source-bundle-verify.py", "hepta_verify_v1_for_order")
        root = v1.TreeNode()
        root.add_file(b"foo.rs", b"100644", v1.git_hash(b"blob", b"file\n"))
        child = root.directory(b"foo")
        child.add_file(b"bar.rs", b"100644", v1.git_hash(b"blob", b"nested\n"))
        root.add_file(b"LICENSE", b"100644", v1.git_hash(
            b"blob", b"Mozilla Public License Version 2.0\nfixture license body\n"
        ))
        root.add_file(b"z-last", b"100644", v1.git_hash(b"blob", b"last\n"))
        self.assertNotEqual(root.object_id().hex(), expected_tree)
        self.assertEqual(git(source, "rev-parse", "HEAD^{tree}"), expected_tree)

    def test_full_bundle_verification_recomputes_prefix_collision_tree(self) -> None:
        source, commit, tree = self.create_source()
        origin, pin, patches = self.make_inputs(source, commit, tree)
        bundle_dir = self.root / "bundle"
        bundle = self.pipeline.execute_pipeline(
            repository_url=os.fspath(origin),
            output_dir=bundle_dir,
            pin_path=pin,
            patch_inventory_path=patches,
            keep_checkouts=False,
            allow_local_test_origin=True,
        )
        for slot in ("a", "b"):
            path = bundle_dir / f"fetch-{slot}.receipt.json"
            receipt = json.loads(path.read_text(encoding="utf-8"))
            receipt["canonical"] = True
            receipt["acquisition"]["origin_kind"] = "pinned_https"
            receipt["acquisition"]["network_access_used"] = True
            path.write_bytes(self.pipeline.canonical_bytes(receipt))
        bundle["canonical"] = True
        bundle["acquisition_network_used"] = True
        bundle["qualification"]["canonical_source_acquired"] = True
        for summary in bundle["fetch_receipts"]:
            raw = (bundle_dir / f"fetch-{summary['slot']}.receipt.json").read_bytes()
            summary["sha256"] = self.pipeline.sha256_bytes(raw)
            summary["bytes"] = len(raw)
            summary["canonical"] = True
        (bundle_dir / "independent-source-bundle.receipt.json").write_bytes(
            self.pipeline.canonical_bytes(bundle)
        )
        result = self.verifier.verify_bundle(bundle_dir, pin, None)
        self.assertEqual(result["source"]["tree"], tree)
        self.assertEqual(result["source"]["recomputed_tree"], tree)
        self.assertTrue(result["verification"]["git_tree_recomputed"])
        self.assertFalse(result["verification"]["servo_built"])


if __name__ == "__main__":
    unittest.main()
