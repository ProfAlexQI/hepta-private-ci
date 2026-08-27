#!/usr/bin/env python3
"""Tests for the canonical v2 Servo source acquisition entrypoint."""

from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import tarfile
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

SCRIPT = Path(__file__).resolve().parents[1] / "hepta-servo-independent-source-v2.py"


def load_module() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_independent_source_v2",
        SCRIPT,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load v2 source-pipeline entrypoint")
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


class IndependentSourceV2Tests(unittest.TestCase):
    def setUp(self) -> None:
        self.entrypoint = load_module()
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name).resolve()
        source = self.root / "source"
        source.mkdir()
        git(source, "init", "--quiet")
        git(source, "config", "user.name", "Hepta Qualification")
        git(source, "config", "user.email", "hepta@example.invalid")
        (source / "LICENSE").write_text(
            "Mozilla Public License Version 2.0\nfixture\n",
            encoding="utf-8",
        )
        (source / "regular.txt").write_text("regular\n", encoding="utf-8")
        executable = source / "executable.sh"
        executable.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
        executable.chmod(0o755)
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
        self.origin = self.root / "origin.git"
        subprocess.run(
            ["git", "clone", "--quiet", "--bare", os.fspath(source), os.fspath(self.origin)],
            check=True,
        )
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

    def test_entrypoint_replaces_all_inherited_git_config(self) -> None:
        os.environ["GIT_CONFIG_COUNT"] = "2"
        os.environ["GIT_CONFIG_KEY_0"] = "core.filemode"
        os.environ["GIT_CONFIG_VALUE_0"] = "false"
        os.environ["GIT_CONFIG_KEY_1"] = "tar.umask"
        os.environ["GIT_CONFIG_VALUE_1"] = "0000"
        self.entrypoint.install_deterministic_archive_environment()
        self.assertEqual(os.environ["GIT_CONFIG_COUNT"], "1")
        self.assertEqual(os.environ["GIT_CONFIG_KEY_0"], "tar.umask")
        self.assertEqual(os.environ["GIT_CONFIG_VALUE_0"], "0022")
        self.assertNotIn("GIT_CONFIG_KEY_1", os.environ)
        self.assertNotIn("GIT_CONFIG_VALUE_1", os.environ)
        self.assertEqual(os.environ["TZ"], "UTC")
        self.assertEqual(os.environ["SOURCE_DATE_EPOCH"], "0")

    def test_archive_modes_are_independent_of_process_umask(self) -> None:
        base = self.entrypoint.load_base()
        original_umask = os.umask(0o002)
        try:
            bundle = base.execute_pipeline(
                repository_url=os.fspath(self.origin),
                output_dir=self.root / "output",
                pin_path=self.pin,
                patch_inventory_path=self.patches,
                keep_checkouts=False,
                allow_local_test_origin=True,
            )
        finally:
            os.umask(original_umask)
        self.assertTrue(bundle["qualification"]["deterministic_tar_verified"])
        archive_path = self.root / "output/servo-source-a.tar"
        prefix = f"servo-{self.commit}/"
        with tarfile.open(archive_path, mode="r:") as archive:
            modes = {
                member.name[len(prefix) :]: member.mode
                for member in archive
                if member.isfile()
            }
        self.assertEqual(modes["LICENSE"], 0o644)
        self.assertEqual(modes["regular.txt"], 0o644)
        self.assertEqual(modes["executable.sh"], 0o755)


if __name__ == "__main__":
    unittest.main()
