#!/usr/bin/env python3
"""Verify the fail-closed Servo worker reproducibility contract."""
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-worker-reproducibility.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_worker_reproducibility.py"
MANIFEST_SCHEMA = (
    ROOT
    / "docs/hepta-vnext/browser/hepta.servo.worker_reproducibility_manifest.v1.schema.json"
)
RECEIPT_SCHEMA = (
    ROOT
    / "docs/hepta-vnext/browser/hepta.servo.worker_reproducibility_receipt.v1.schema.json"
)
STATUS = ROOT / "docs/hepta-vnext/browser/C1_REPRODUCIBILITY_STATUS.json"
SPEC = ROOT / "docs/hepta-vnext/browser/C1_REPRODUCIBILITY.md"

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


def verify_false_authority(schema: dict[str, Any], label: str) -> None:
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
        for path in (TOOL, TEST, MANIFEST_SCHEMA, RECEIPT_SCHEMA, STATUS, SPEC):
            if not path.is_file():
                fail(f"missing reproducibility file: {path.relative_to(ROOT)}")

        source = TOOL.read_text(encoding="utf-8")
        tests = TEST.read_text(encoding="utf-8")
        for forbidden in (
            "import socket",
            "import urllib",
            "import requests",
            "import subprocess",
            "os.system",
            "worker_executed\": True",
            "servo_runtime_qualified\": True",
            "allow_explained_differences\": True",
        ):
            if forbidden in source:
                fail(f"reproducibility tool contains forbidden surface: {forbidden}")

        required_tokens = (
            "require_byte_identical",
            "allow_missing_optional",
            "allow_explained_differences",
            "require_output",
            "st_nlink",
            "S_IWGRP",
            "S_IWOTH",
            "canonical_json",
            "reject_positive_posture",
            "independent worker build outputs are not byte-identical",
            "WORKER_BUILD_BYTE_IDENTICAL_RUNTIME_NOT_QUALIFIED",
            "hepta-servo-worker-reproducibility:v1:",
            "create-only",
            "worker_executed\": False",
            "servo_runtime_qualified\": False",
        )
        for token in required_tokens:
            if token not in source:
                fail(f"reproducibility tool is missing {token}")

        for test_name in (
            "test_duplicate_keys_fail_closed",
            "test_unsafe_output_paths_are_rejected",
            "test_positive_runtime_posture_is_rejected_recursively",
            "test_self_test_covers_five_fail_closed_cases",
        ):
            if f"def {test_name}" not in tests:
                fail(f"reproducibility test is missing {test_name}")

        manifest_schema = load(MANIFEST_SCHEMA)
        receipt_schema = load(RECEIPT_SCHEMA)
        if manifest_schema.get("$id") != (
            "hepta.servo.worker_reproducibility_manifest.v1"
        ):
            fail("reproducibility manifest schema ID drifted")
        if receipt_schema.get("$id") != (
            "hepta.servo.worker_reproducibility_receipt.v1"
        ):
            fail("reproducibility receipt schema ID drifted")
        verify_false_authority(manifest_schema, "manifest schema")
        verify_false_authority(receipt_schema, "receipt schema")

        manifest_policy = (
            manifest_schema.get("properties", {})
            .get("comparison_policy", {})
            .get("properties", {})
        )
        expected_policy = {
            "allow_explained_differences": {"const": False},
            "allow_missing_optional": {"const": False},
            "require_byte_identical": {"const": True},
        }
        if manifest_policy != expected_policy:
            fail("reproducibility manifest comparison policy is not strict")

        receipt_properties = receipt_schema.get("properties", {})
        if receipt_properties.get("decision") != {
            "const": "WORKER_BUILD_BYTE_IDENTICAL_RUNTIME_NOT_QUALIFIED"
        }:
            fail("reproducibility receipt decision overclaims")
        runtime = receipt_properties.get("runtime", {}).get("properties", {})
        if runtime != {
            "external_network_used": {"const": False},
            "servo_runtime_qualified": {"const": False},
            "worker_executed": {"const": False},
        }:
            fail("reproducibility receipt runtime posture is open")

        status = load(STATUS)
        if status.get("merge_authorized") is not False:
            fail("reproducibility status authorized merge")
        if status.get("real_worker_build_comparisons") != 0:
            fail("reproducibility status claims a real worker comparison")
        authority = status.get("authority")
        if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
            fail("reproducibility status authority keys drifted")
        if any(value is not False for value in authority.values()):
            fail("reproducibility status enables authority")
    except (VerificationError, OSError, UnicodeError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(
        json.dumps(
            {
                "schema": "hepta.servo.worker_reproducibility_contract_verification.v1",
                "status": "PASS_FIXTURE_CONTRACT_ONLY",
                "byte_identical_required": True,
                "real_worker_build_compared": False,
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
