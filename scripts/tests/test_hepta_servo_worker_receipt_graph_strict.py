#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path
from types import ModuleType

ROOT = Path(__file__).resolve().parents[2]
WRAPPER = ROOT / "scripts/hepta-servo-worker-receipt-graph-strict.py"


def load_wrapper() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_worker_receipt_graph_strict",
        WRAPPER,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load strict receipt graph entrypoint")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class StrictReceiptGraphTests(unittest.TestCase):
    def setUp(self) -> None:
        self.wrapper = load_wrapper()
        self.core = self.wrapper.load_core()
        self.wrapper.install_strict_checks(self.core)

    def manifest(self) -> dict:
        authority = self.core["AUTHORITY"]
        commit = self.core["SERVO_COMMIT"]
        tree = self.core["SERVO_TREE"]
        return {
            "schema": self.core["MANIFEST_SCHEMA"],
            "schema_version": 1,
            "phase": "DEVELOPMENT",
            "servo_commit": commit,
            "servo_tree": tree,
            "nodes": [
                {"id": "artifact", "kind": "canonical_json", "path": "artifact.json", "expected_schema": "hepta.servo.worker_artifact_receipt.v1", "negative_posture_scan": True},
                {"id": "build", "kind": "canonical_json", "path": "build.json", "expected_schema": "hepta.servo.worker_build_manifest.v1", "negative_posture_scan": True},
                {"id": "reproducibility", "kind": "canonical_json", "path": "reproducibility.json", "expected_schema": "hepta.servo.worker_reproducibility_receipt.v1", "negative_posture_scan": True},
                {"id": "source", "kind": "canonical_json", "path": "source.json", "expected_schema": "hepta.servo.source_receipt.v1", "negative_posture_scan": True},
                {"id": "worker", "kind": "binary", "path": "worker.bin", "expected_schema": None, "negative_posture_scan": False},
            ],
            "edges": [
                {"id": "artifact-build", "type": "pointer_equals_file_sha256", "left_node": "artifact", "left_pointer": "/build_manifest_sha256", "right_node": "build"},
                {"id": "artifact-source", "type": "pointer_equals_file_sha256", "left_node": "artifact", "left_pointer": "/source_receipt_sha256", "right_node": "source"},
                {"id": "artifact-worker", "type": "pointer_equals_file_sha256", "left_node": "artifact", "left_pointer": "/artifact/sha256", "right_node": "worker"},
                {"id": "repro-build", "type": "pointer_equals_file_sha256", "left_node": "reproducibility", "left_pointer": "/source_binding/build_manifest_sha256", "right_node": "build"},
                {"id": "repro-worker", "type": "pointer_equals_file_sha256", "left_node": "reproducibility", "left_pointer": "/outputs/0/sha256", "right_node": "worker"},
                {"id": "source-commit", "type": "pointer_equals_literal", "left_node": "source", "left_pointer": "/source/commit", "literal": commit},
                {"id": "source-tree", "type": "pointer_equals_literal", "left_node": "source", "left_pointer": "/source/tree", "literal": tree},
            ],
            "worker_node": "worker",
            "policy": {"require_all_edges": True, "launch_authorized": False, "runtime_qualified": False, "allow_unknown_nodes": False, "allow_unknown_edges": False},
            "authority": authority,
        }

    def validate(self, manifest: dict) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "manifest.json"
            path.write_bytes(self.core["canonical"](manifest))
            self.core["validate_manifest"](path)

    def test_complete_connected_graph_passes(self) -> None:
        self.validate(self.manifest())

    def test_duplicate_node_paths_fail_closed(self) -> None:
        manifest = self.manifest()
        manifest["nodes"][1]["path"] = manifest["nodes"][0]["path"]
        with self.assertRaises(self.core["GraphError"]):
            self.validate(manifest)

    def test_missing_required_schema_fails_closed(self) -> None:
        manifest = self.manifest()
        manifest["nodes"][1]["expected_schema"] = "hepta.fixture.other.v1"
        with self.assertRaises(self.core["GraphError"]):
            self.validate(manifest)

    def test_disconnected_node_fails_closed(self) -> None:
        manifest = self.manifest()
        manifest["edges"] = [
            edge
            for edge in manifest["edges"]
            if edge["id"] not in {"artifact-source"}
        ]
        with self.assertRaises(self.core["GraphError"]):
            self.validate(manifest)

    def test_worker_needs_two_independent_hash_edges(self) -> None:
        manifest = self.manifest()
        manifest["edges"] = [
            edge for edge in manifest["edges"] if edge["id"] != "repro-worker"
        ]
        with self.assertRaises(self.core["GraphError"]):
            self.validate(manifest)

    def test_strict_self_test_passes_fixture_only(self) -> None:
        result = self.core["self_test"]()
        self.assertEqual(result["status"], "PASS_LOCAL_FIXTURE_ONLY")
        self.assertFalse(result["launch_authorized"])


if __name__ == "__main__":
    unittest.main()
