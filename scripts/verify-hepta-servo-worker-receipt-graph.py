#!/usr/bin/env python3
"""Verify the immutable Servo worker receipt-graph contract."""
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-worker-receipt-graph.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_worker_receipt_graph.py"
MANIFEST_SCHEMA = (
    ROOT
    / "docs/hepta-vnext/browser/hepta.servo.worker_receipt_graph_manifest.v1.schema.json"
)
RECEIPT_SCHEMA = (
    ROOT
    / "docs/hepta-vnext/browser/hepta.servo.worker_receipt_graph_verification.v1.schema.json"
)
SPEC = ROOT / "docs/hepta-vnext/browser/C1_RECEIPT_GRAPH.md"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_RECEIPT_GRAPH_STATUS.json"

AUTHORITY_KEYS = {
    "machine_authority",
    "runtime_authority",
    "production_caller",
    "production_writer",
    "effect_authority",
    "external_effect",
    "external_network_allowed",
    "credential_export_allowed",
    "operator_acceptance",
    "g5_allowed",
    "execute_allowed",
    "promotion",
    "release_qualified",
}


class VerificationError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise VerificationError(message)


def load(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one object")
    return value


def verify_authority(schema: dict[str, Any], label: str) -> None:
    definitions = schema.get("$defs")
    if not isinstance(definitions, dict):
        fail(f"{label} has no definitions")
    authority = definitions.get("authority")
    if not isinstance(authority, dict):
        fail(f"{label} has no authority definition")
    properties = authority.get("properties")
    if not isinstance(properties, dict) or set(properties) != AUTHORITY_KEYS:
        fail(f"{label} authority keys drifted")
    for key, definition in properties.items():
        if definition != {"const": False}:
            fail(f"{label} widens authority field {key}")


def main() -> int:
    try:
        for path in (TOOL, TEST, MANIFEST_SCHEMA, RECEIPT_SCHEMA, SPEC, STATUS):
            if not path.is_file():
                fail(f"missing receipt graph file: {path.relative_to(ROOT)}")

        source = TOOL.read_text(encoding="utf-8")
        tests = TEST.read_text(encoding="utf-8")
        for forbidden in (
            "import socket",
            "import urllib",
            "import requests",
            "import subprocess",
            "os.system",
            "launch_authorized\": True",
            "worker_executed\": True",
            "servo_runtime_qualified\": True",
        ):
            if forbidden in source:
                fail(f"receipt graph tool contains forbidden surface: {forbidden}")

        required = (
            "reject_duplicate_keys",
            "require_file",
            "st_nlink",
            "S_IWGRP",
            "S_IWOTH",
            "reject_positive_posture",
            "decode_pointer",
            "resolve_pointer",
            "pointer_equals_literal",
            "pointer_equals_file_sha256",
            "pointers_equal",
            "receipt graph edge does not match",
            "RECEIPT_GRAPH_BOUND_LAUNCH_NOT_AUTHORIZED",
            "hepta-servo-worker-receipt-graph:v1:",
            "launch_authorized\": False",
            "worker_executed\": False",
            "servo_runtime_qualified\": False",
        )
        for token in required:
            if token not in source:
                fail(f"receipt graph tool is missing {token}")

        for test_name in (
            "test_contract_keeps_launch_and_authority_closed",
            "test_self_test_covers_six_fail_closed_cases",
            "test_invalid_json_pointer_escape_fails",
            "test_positive_execute_authority_fails_recursively",
        ):
            if f"def {test_name}" not in tests:
                fail(f"receipt graph test is missing {test_name}")

        manifest = load(MANIFEST_SCHEMA)
        receipt = load(RECEIPT_SCHEMA)
        if manifest.get("$id") != "hepta.servo.worker_receipt_graph_manifest.v1":
            fail("receipt graph manifest schema ID drifted")
        if receipt.get("$id") != "hepta.servo.worker_receipt_graph_verification.v1":
            fail("receipt graph verification schema ID drifted")
        verify_authority(manifest, "receipt graph manifest schema")
        verify_authority(receipt, "receipt graph verification schema")

        policy = manifest.get("properties", {}).get("policy", {}).get("properties", {})
        if policy != {
            "allow_unknown_edges": {"const": False},
            "allow_unknown_nodes": {"const": False},
            "launch_authorized": {"const": False},
            "require_all_edges": {"const": True},
            "runtime_qualified": {"const": False},
        }:
            fail("receipt graph manifest policy is not fail-closed")
        receipt_properties = receipt.get("properties", {})
        if receipt_properties.get("decision") != {
            "const": "RECEIPT_GRAPH_BOUND_LAUNCH_NOT_AUTHORIZED"
        }:
            fail("receipt graph decision overclaims")
        runtime = receipt_properties.get("runtime", {}).get("properties", {})
        if runtime != {
            "external_network_used": {"const": False},
            "launch_authorized": {"const": False},
            "servo_runtime_qualified": {"const": False},
            "worker_executed": {"const": False},
        }:
            fail("receipt graph runtime posture is open")

        status = load(STATUS)
        if status.get("merge_authorized") is not False:
            fail("receipt graph status authorized merge")
        if status.get("real_receipt_graphs_verified") != 0:
            fail("receipt graph status claims real evidence")
        authority = status.get("authority")
        if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
            fail("receipt graph status authority keys drifted")
        if any(value is not False for value in authority.values()):
            fail("receipt graph status enables authority")
    except (VerificationError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(
        json.dumps(
            {
                "schema": "hepta.servo.worker_receipt_graph_contract_verification.v1",
                "status": "PASS_FIXTURE_CONTRACT_ONLY",
                "real_receipt_graph_verified": False,
                "launch_authorized": False,
                "runtime_qualified": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
