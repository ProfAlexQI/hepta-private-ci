#!/usr/bin/env python3
"""Unit tests for the offline Servo provenance generator."""

from __future__ import annotations

import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

ROOT = Path(__file__).resolve().parents[1]
GENERATOR_PATH = ROOT / "scripts/generate-hepta-servo-provenance.py"


def load_generator() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_provenance_generator",
        GENERATOR_PATH,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load Servo provenance generator")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class ServoProvenanceTests(unittest.TestCase):
    def setUp(self) -> None:
        self.generator = load_generator()

    def test_canonical_bytes_are_stable_and_sorted(self) -> None:
        left = self.generator.canonical_bytes({"z": 1, "a": {"d": 4, "b": 2}})
        right = self.generator.canonical_bytes({"a": {"b": 2, "d": 4}, "z": 1})
        self.assertEqual(left, right)
        self.assertEqual(left, b'{"a":{"b":2,"d":4},"z":1}')

    def test_sha256_bytes_matches_known_vector(self) -> None:
        self.assertEqual(
            self.generator.sha256_bytes(b"hepta-servo"),
            "15e54c4687ed19212094fe34da336c398d6d617d6f85b96ed03cffdaf5910aa9",
        )

    def test_empty_patch_inventory_is_deterministic(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            original_root = self.generator.PATCH_ROOT
            self.generator.PATCH_ROOT = Path(directory)
            try:
                inventory = {
                    "schema": "hepta.browser.servo_patch_inventory.v1",
                    "servo_commit": "1" * 40,
                    "servo_tree": "2" * 40,
                    "patches": [],
                }
                records, digest = self.generator.verify_patch_inventory(
                    inventory,
                    {"commit": "1" * 40, "tree": "2" * 40},
                )
            finally:
                self.generator.PATCH_ROOT = original_root
        self.assertEqual(records, [])
        expected = self.generator.sha256_bytes(self.generator.canonical_bytes(inventory))
        self.assertEqual(digest, expected)

    def test_unregistered_patch_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "0001-unregistered.patch").write_text("test patch", encoding="utf-8")
            original_root = self.generator.PATCH_ROOT
            self.generator.PATCH_ROOT = root
            try:
                with self.assertRaises(self.generator.ProvenanceError):
                    self.generator.verify_patch_inventory(
                        {
                            "schema": "hepta.browser.servo_patch_inventory.v1",
                            "servo_commit": "1" * 40,
                            "servo_tree": "2" * 40,
                            "patches": [],
                        },
                        {"commit": "1" * 40, "tree": "2" * 40},
                    )
            finally:
                self.generator.PATCH_ROOT = original_root

    def test_receipt_serialization_contains_no_incidental_whitespace(self) -> None:
        value = json.loads(
            self.generator.canonical_bytes({"authority": {"promotion": False}})
        )
        self.assertEqual(value, {"authority": {"promotion": False}})


if __name__ == "__main__":
    unittest.main()
