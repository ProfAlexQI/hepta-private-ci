#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "scripts/hepta-servo-worker-reproducibility.py"


def load_tool() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_worker_reproducibility",
        TOOL,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load reproducibility tool")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class ReproducibilityTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tool = load_tool()

    def test_canonical_bytes_are_sorted(self) -> None:
        self.assertEqual(self.tool.canonical({"z": 1, "a": 2}), b'{"a":2,"z":1}')

    def test_duplicate_keys_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "duplicate.json"
            path.write_text('{"a":1,"a":2}', encoding="utf-8")
            with self.assertRaises(self.tool.ReproducibilityError):
                self.tool.load_json(path, "duplicate fixture")

    def test_unsafe_output_paths_are_rejected(self) -> None:
        for value in ("../escape", "/absolute", "a\\b", "./relative"):
            with self.subTest(value=value):
                with self.assertRaises(self.tool.ReproducibilityError):
                    self.tool.validate_relative_path(value)

    def test_positive_runtime_posture_is_rejected_recursively(self) -> None:
        with self.assertRaises(self.tool.ReproducibilityError):
            self.tool.reject_positive_posture({"nested": {"runtime_qualified": True}})

    def test_contract_remains_negative_authority(self) -> None:
        result = self.tool.contract()
        self.assertFalse(result["runtime_qualified"])
        self.assertFalse(any(result["authority"].values()))

    def test_self_test_covers_five_fail_closed_cases(self) -> None:
        result = self.tool.self_test()
        self.assertEqual(result["status"], "PASS_LOCAL_FIXTURE_ONLY")
        self.assertEqual(result["test_count"], 5)
        self.assertFalse(result["real_worker_build_compared"])


if __name__ == "__main__":
    unittest.main()
