#!/usr/bin/env python3
"""Verify the graph-bound Servo worker startup descriptor contract."""
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-worker-startup-descriptor.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_worker_startup_descriptor.py"
SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_startup_descriptor.v1.schema.json"
SPEC = ROOT / "docs/hepta-vnext/browser/C1_STARTUP_DESCRIPTOR.md"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_STARTUP_DESCRIPTOR_STATUS.json"
AUTHORITY_KEYS = {
    "machine_authority", "runtime_authority", "production_caller", "production_writer",
    "effect_authority", "external_effect", "external_network_allowed",
    "credential_export_allowed", "operator_acceptance", "g5_allowed",
    "execute_allowed", "promotion", "release_qualified",
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
        for path in (TOOL, TEST, SCHEMA, SPEC, STATUS):
            if not path.is_file():
                fail(f"missing startup descriptor file: {path.relative_to(ROOT)}")
        source = TOOL.read_text(encoding="utf-8")
        tests = TEST.read_text(encoding="utf-8")
        for forbidden in (
            "import socket",
            "import urllib",
            "import requests",
            "import subprocess",
            "os.system",
            '"launch_authorized": True',
            '"worker_executed": True',
            '"servo_runtime_qualified": True',
        ):
            if forbidden in source:
                fail(f"startup descriptor compiler contains forbidden surface: {forbidden}")
        required = (
            "verify_graph_receipt_id",
            "graph verification does not bind the manifest bytes",
            "graph verification worker node does not bind actual worker bytes",
            "graph verification worker summary does not bind actual worker bytes",
            "graph verification does not assert all edges matched",
            "graph verification node/edge count is incomplete",
            "verification_required_again_at_launch",
            "GRAPH_BOUND_STARTUP_CANDIDATE_LAUNCH_NOT_AUTHORIZED",
            "startup descriptors are create-only",
            "FORBIDDEN_DESCRIPTOR_KEYS",
            "startup_capability",
            "host_nonce",
            '"launch_authorized": False',
            '"g5_allowed": False',
            '"execute_allowed": False',
        )
        for token in required:
            if token not in source:
                fail(f"startup descriptor compiler is missing {token}")
        for test_name in (
            "test_contract_keeps_launch_and_authority_closed",
            "test_self_test_covers_seven_fail_closed_cases",
            "test_descriptor_contains_no_secret_keys",
            "test_only_private_transport_classes_are_accepted",
            "test_zero_generation_or_owner_epoch_fails",
            "test_descriptor_id_rejects_tampering",
        ):
            if f"def {test_name}" not in tests:
                fail(f"startup descriptor test is missing {test_name}")

        schema = load(SCHEMA)
        if schema.get("$id") != "hepta.servo.worker_startup_descriptor.v1":
            fail("startup descriptor schema ID drifted")
        authority = schema.get("$defs", {}).get("authority", {}).get("properties", {})
        if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
            fail("startup descriptor schema authority keys drifted")
        for key, definition in authority.items():
            if definition != {"const": False}:
                fail(f"startup descriptor schema widens authority field {key}")
        runtime = schema.get("properties", {}).get("runtime", {}).get("properties", {})
        if runtime != {
            "external_network_used": {"const": False},
            "launch_authorized": {"const": False},
            "servo_runtime_qualified": {"const": False},
            "worker_executed": {"const": False},
        }:
            fail("startup descriptor schema runtime posture is open")
        transport = schema.get("properties", {}).get("transport", {}).get("properties", {})
        if transport.get("kind") != {
            "enum": ["unix_inherited_socketpair", "windows_sid_named_pipe"]
        }:
            fail("startup descriptor schema transport classes drifted")
        if schema.get("properties", {}).get("decision") != {
            "const": "GRAPH_BOUND_STARTUP_CANDIDATE_LAUNCH_NOT_AUTHORIZED"
        }:
            fail("startup descriptor schema decision overclaims")

        status = load(STATUS)
        if status.get("merge_authorized") is not False:
            fail("startup descriptor status authorized merge")
        if status.get("real_descriptors_created") != 0:
            fail("startup descriptor status claims real evidence")
        authority_status = status.get("authority")
        if not isinstance(authority_status, dict) or set(authority_status) != AUTHORITY_KEYS:
            fail("startup descriptor status authority keys drifted")
        if any(value is not False for value in authority_status.values()):
            fail("startup descriptor status enables authority")
    except (VerificationError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(json.dumps({
        "schema": "hepta.servo.worker_startup_descriptor_contract_verification.v1",
        "status": "PASS_FIXTURE_CONTRACT_ONLY",
        "real_descriptor_created": False,
        "launch_authorized": False,
        "worker_executed": False,
        "runtime_qualified": False,
        "secret_material_in_descriptor": False,
        "authority": "all_false",
    }, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
