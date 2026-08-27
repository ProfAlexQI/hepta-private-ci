#!/usr/bin/env python3
"""Verify the strict Servo worker receipt-graph manifest assembler contract."""
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-worker-receipt-graph-manifest.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_worker_receipt_graph_manifest.py"
GRAPH_SCHEMA = (
    ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_receipt_graph_manifest.v1.schema.json"
)
SPEC = ROOT / "docs/hepta-vnext/browser/C1_RECEIPT_GRAPH_ASSEMBLER.md"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_RECEIPT_GRAPH_ASSEMBLER_STATUS.json"

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


def main() -> int:
    try:
        for path in (TOOL, TEST, GRAPH_SCHEMA, SPEC, STATUS):
            if not path.is_file():
                fail(f"missing graph manifest assembler file: {path.relative_to(ROOT)}")
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
            "runtime_qualified\": True",
        ):
            if forbidden in source:
                fail(f"graph manifest assembler contains forbidden surface: {forbidden}")
        required = (
            "reject_duplicate_keys",
            "require_file",
            "st_nlink",
            "S_IWGRP",
            "S_IWOTH",
            "reject_positive_posture",
            "source receipt does not bind",
            "artifact receipt does not bind the source receipt bytes",
            "artifact receipt does not bind the build manifest bytes",
            "artifact receipt does not bind the worker bytes",
            "reproducibility receipt does not bind the build manifest bytes",
            "reproducibility receipt must bind exactly one matching worker output",
            "RECEIPT_GRAPH_MANIFEST_CREATED_LAUNCH_NOT_AUTHORIZED",
            "graph manifests are create-only",
            "launch_authorized\": False",
            "g5_allowed\": False",
            "execute_allowed\": False",
        )
        for token in required:
            if token not in source:
                fail(f"graph manifest assembler is missing {token}")
        for test_name in (
            "test_contract_keeps_launch_and_authority_closed",
            "test_self_test_covers_six_fail_closed_cases",
            "test_unsafe_paths_fail_closed",
            "test_duplicate_keys_fail_closed",
            "test_positive_g5_or_execute_posture_fails",
            "test_fixture_manifest_is_sorted_and_complete",
        ):
            if f"def {test_name}" not in tests:
                fail(f"graph manifest assembler test is missing {test_name}")

        schema = load(GRAPH_SCHEMA)
        if schema.get("$id") != "hepta.servo.worker_receipt_graph_manifest.v1":
            fail("graph manifest schema ID drifted")
        authority = schema.get("$defs", {}).get("authority", {}).get("properties", {})
        if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
            fail("graph manifest schema authority keys drifted")
        for key, definition in authority.items():
            if definition != {"const": False}:
                fail(f"graph manifest schema widens authority field {key}")
        policy = schema.get("properties", {}).get("policy", {}).get("properties", {})
        if policy != {
            "allow_unknown_edges": {"const": False},
            "allow_unknown_nodes": {"const": False},
            "launch_authorized": {"const": False},
            "require_all_edges": {"const": True},
            "runtime_qualified": {"const": False},
        }:
            fail("graph manifest schema policy is not fail-closed")

        status = load(STATUS)
        if status.get("merge_authorized") is not False:
            fail("graph manifest assembler status authorized merge")
        if status.get("real_packets_assembled") != 0:
            fail("graph manifest assembler status claims real evidence")
        authority_status = status.get("authority")
        if not isinstance(authority_status, dict) or set(authority_status) != AUTHORITY_KEYS:
            fail("graph manifest assembler status authority keys drifted")
        if any(value is not False for value in authority_status.values()):
            fail("graph manifest assembler status enables authority")
    except (VerificationError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(
        json.dumps(
            {
                "schema": "hepta.servo.worker_receipt_graph_manifest_assembler_contract_verification.v1",
                "status": "PASS_FIXTURE_CONTRACT_ONLY",
                "real_packet_assembled": False,
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
