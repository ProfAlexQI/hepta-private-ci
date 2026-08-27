#!/usr/bin/env python3
"""Verify the graph-bound platform qualification launch-plan contract."""
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-worker-launch-plan.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_worker_launch_plan.py"
POLICY_SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_launch_policy.v1.schema.json"
PLAN_SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_qualification_launch_plan.v1.schema.json"
SPEC = ROOT / "docs/hepta-vnext/browser/C1_QUALIFICATION_LAUNCH_PLAN.md"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_QUALIFICATION_LAUNCH_PLAN_STATUS.json"
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


def verify_authority(schema: dict[str, Any], label: str) -> None:
    properties = schema.get("$defs", {}).get("authority", {}).get("properties", {})
    if not isinstance(properties, dict) or set(properties) != AUTHORITY_KEYS:
        fail(f"{label} authority keys drifted")
    for key, definition in properties.items():
        if definition != {"const": False}:
            fail(f"{label} widens authority field {key}")


def main() -> int:
    try:
        for path in (TOOL, TEST, POLICY_SCHEMA, PLAN_SCHEMA, SPEC, STATUS):
            if not path.is_file():
                fail(f"missing launch-plan file: {path.relative_to(ROOT)}")
        source = TOOL.read_text(encoding="utf-8")
        tests = TEST.read_text(encoding="utf-8")
        for forbidden in (
            "import socket", "import urllib", "import requests", "import subprocess", "os.system",
            '"launch_authorized": True', '"worker_executed": True', '"servo_runtime_qualified": True',
        ):
            if forbidden in source:
                fail(f"launch-plan compiler contains forbidden surface: {forbidden}")
        for token in (
            "validate_descriptor", "verify_descriptor_id", "validate_policy", "LIMIT_BOUNDS",
            "PLATFORMS", "graph_reverification_at_launch", "BLOCKED_EVIDENCE_REQUIRED",
            "GRAPH_BOUND_PLATFORM_LAUNCH_PLAN_NOT_ADMITTED", "qualification launch plans are create-only",
            "sandbox_required", "egress_denial_required", "peer_identity", "parent_death",
            "descendant_cleanup", '"launch_authorized": False', '"g5_allowed": False',
            '"execute_allowed": False',
        ):
            if token not in source:
                fail(f"launch-plan compiler is missing {token}")
        for test_name in (
            "test_contract_keeps_launch_and_authority_closed",
            "test_self_test_covers_seven_fail_closed_cases",
            "test_fixture_plan_has_no_secret_or_execution_keys",
            "test_platform_policy_mapping_is_exact",
            "test_platform_policy_rejects_wrong_enforcement_strategy",
            "test_policy_limits_reject_bool_and_ordering_errors",
            "test_plan_id_rejects_tampering",
        ):
            if f"def {test_name}" not in tests:
                fail(f"launch-plan test is missing {test_name}")

        policy_schema = load(POLICY_SCHEMA)
        plan_schema = load(PLAN_SCHEMA)
        if policy_schema.get("$id") != "hepta.servo.worker_launch_policy.v1":
            fail("launch policy schema ID drifted")
        if plan_schema.get("$id") != "hepta.servo.worker_qualification_launch_plan.v1":
            fail("qualification launch plan schema ID drifted")
        verify_authority(policy_schema, "launch policy schema")
        verify_authority(plan_schema, "qualification launch plan schema")
        plan_properties = plan_schema.get("properties", {})
        if plan_properties.get("admission_state") != {"const": "BLOCKED_EVIDENCE_REQUIRED"}:
            fail("qualification launch plan admission state overclaims")
        if plan_properties.get("decision") != {"const": "GRAPH_BOUND_PLATFORM_LAUNCH_PLAN_NOT_ADMITTED"}:
            fail("qualification launch plan decision overclaims")
        runtime = plan_properties.get("runtime", {}).get("properties", {})
        if runtime != {
            "external_network_used": {"const": False},
            "launch_authorized": {"const": False},
            "servo_runtime_qualified": {"const": False},
            "worker_executed": {"const": False},
        }:
            fail("qualification launch plan runtime posture is open")
        evidence = plan_properties.get("required_evidence", {}).get("properties", {})
        if evidence.get("graph_reverification_at_launch") != {"const": True}:
            fail("qualification launch plan does not require graph reverification")
        for key, definition in evidence.items():
            if key != "graph_reverification_at_launch" and definition != {"const": None}:
                fail(f"qualification launch plan schema embeds unqualified evidence: {key}")

        status = load(STATUS)
        if status.get("merge_authorized") is not False or status.get("real_plans_created") != 0:
            fail("qualification launch plan status overclaims merge or real evidence")
        authority = status.get("authority")
        if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
            fail("qualification launch plan status authority keys drifted")
        if any(value is not False for value in authority.values()):
            fail("qualification launch plan status enables authority")
    except (VerificationError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(json.dumps({
        "schema": "hepta.servo.worker_qualification_launch_plan_contract_verification.v1",
        "status": "PASS_FIXTURE_CONTRACT_ONLY",
        "real_plan_created": False,
        "admission_state": "BLOCKED_EVIDENCE_REQUIRED",
        "launch_authorized": False,
        "worker_executed": False,
        "runtime_qualified": False,
        "authority": "all_false",
    }, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
