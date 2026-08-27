#!/usr/bin/env python3
"""Strict entrypoint for the immutable Servo worker receipt graph engine."""
from __future__ import annotations

from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
CORE = Path(__file__).with_name("hepta-servo-worker-receipt-graph.py")
REQUIRED_SCHEMAS = {
    "hepta.servo.source_receipt.v1",
    "hepta.servo.worker_build_manifest.v1",
    "hepta.servo.worker_artifact_receipt.v1",
    "hepta.servo.worker_reproducibility_receipt.v1",
}


def load_core() -> dict[str, Any]:
    source = CORE.read_text(encoding="utf-8")
    namespace: dict[str, Any] = {
        "__file__": str(CORE),
        "__name__": "hepta_servo_worker_receipt_graph_core",
        "__package__": None,
    }
    exec(compile(source, str(CORE), "exec"), namespace)
    return namespace


def install_strict_checks(namespace: dict[str, Any]) -> None:
    original_validate = namespace.get("validate_manifest")
    original_build = namespace.get("build_receipt")
    fail = namespace.get("fail")
    if not callable(original_validate) or not callable(original_build) or not callable(fail):
        raise RuntimeError("receipt graph core is missing required validation functions")

    def validate_manifest(path: Path) -> tuple[dict[str, Any], bytes]:
        manifest, raw = original_validate(path)
        nodes = manifest["nodes"]
        paths = [node["path"] for node in nodes]
        if len(paths) != len(set(paths)):
            fail("receipt graph nodes must use unique file paths")

        schemas = {
            node["expected_schema"]
            for node in nodes
            if node["kind"] == "canonical_json"
        }
        missing_schemas = sorted(REQUIRED_SCHEMAS - schemas)
        if missing_schemas:
            fail(f"receipt graph is missing required schemas: {missing_schemas}")

        node_ids = {node["id"] for node in nodes}
        adjacency = {node_id: set() for node_id in node_ids}
        worker_targets = 0
        for edge in manifest["edges"]:
            if edge["type"] in {"pointer_equals_file_sha256", "pointers_equal"}:
                left = edge["left_node"]
                right = edge["right_node"]
                adjacency[left].add(right)
                adjacency[right].add(left)
                if edge["type"] == "pointer_equals_file_sha256" and right == manifest["worker_node"]:
                    worker_targets += 1
        if worker_targets < 2:
            fail("worker node must be bound by at least two independent file-SHA edges")

        visited: set[str] = set()
        pending = [manifest["worker_node"]]
        while pending:
            current = pending.pop()
            if current in visited:
                continue
            visited.add(current)
            pending.extend(sorted(adjacency[current] - visited))
        if visited != node_ids:
            fail(f"receipt graph contains disconnected nodes: {sorted(node_ids - visited)}")
        return manifest, raw

    def build_receipt(root: Path, manifest_path: Path, captured_at: str | None) -> dict[str, Any]:
        receipt = original_build(root, manifest_path, captured_at)
        if receipt["worker"]["bytes"] <= 0:
            fail("receipt graph worker node must not be empty")
        return receipt

    namespace["validate_manifest"] = validate_manifest
    namespace["build_receipt"] = build_receipt


def main() -> int:
    namespace = load_core()
    install_strict_checks(namespace)
    entrypoint = namespace.get("main")
    if not callable(entrypoint):
        raise RuntimeError("receipt graph core has no callable main")
    result = entrypoint()
    if not isinstance(result, int):
        raise RuntimeError("receipt graph core returned a non-integer result")
    return result


if __name__ == "__main__":
    raise SystemExit(main())
