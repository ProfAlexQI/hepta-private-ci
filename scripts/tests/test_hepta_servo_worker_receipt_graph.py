#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import unittest
from pathlib import Path
from types import ModuleType

ROOT = Path(__file__).resolve().parents[2]
TOOL = ROOT / "scripts/hepta-servo-worker-receipt-graph.py"


def load_tool() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_worker_receipt_graph",
        TOOL,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load receipt graph tool")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class ReceiptGraphTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tool = load_tool()

    def test_contract_keeps_launch_and_authority_closed(self) -> None:
        result = self.tool.contract()
        self.assertFalse(result["launch_authorized"])
        self.assertFalse(result["runtime_qualified"])
        self.assertFalse(any(result["authority"].values()))

    def test_self_test_covers_six_fail_closed_cases(self) -> None:
        result = self.tool.self_test()
        self.assertEqual(result["status"], "PASS_LOCAL_FIXTURE_ONLY")
        self.assertEqual(result["test_count"], 6)
        self.assertFalse(result["real_worker_graph_verified"])
        self.assertFalse(result["launch_authorized"])

    def test_invalid_json_pointer_escape_fails(self) -> None:
        with self.assertRaises(self.tool.GraphError):
            self.tool.decode_pointer("/bad~2escape")

    def test_noncanonical_array_index_fails(self) -> None:
        with self.assertRaises(self.tool.GraphError):
            self.tool.resolve_pointer(["zero", "one"], "/01")

    def test_positive_execute_authority_fails_recursively(self) -> None:
        with self.assertRaises(self.tool.GraphError):
            self.tool.reject_positive_posture({"nested": {"execute_allowed": True}})

    def test_unsafe_paths_fail_closed(self) -> None:
        for value in ("../escape", "/absolute", "a\\b", "./relative"):
            with self.subTest(value=value):
                with self.assertRaises(self.tool.GraphError):
                    self.tool.validate_relative_path(value)


if __name__ == "__main__":
    unittest.main()
