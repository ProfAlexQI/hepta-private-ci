#!/usr/bin/env python3
"""Verify the repository-native Servo worker artifact binding contract."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs" / "hepta-vnext" / "browser"
GENERATOR = ROOT / "scripts" / "hepta-servo-artifact-receipt.py"
TESTS = ROOT / "scripts" / "tests" / "test_hepta_servo_artifact_receipt.py"
WORKFLOW = ROOT / ".github" / "workflows" / "hepta-servo-artifact-contract.yml"
STATUS = DOCS / "C1_ARTIFACT_STATUS.json"
PLAN = DOCS / "C1_ARTIFACT_BINDING.md"
SCHEMAS = [
    DOCS / "hepta.servo.worker_build_manifest.v1.schema.json",
    DOCS / "hepta.servo.patch_inventory.v1.schema.json",
    DOCS / "hepta.servo.license_packet.v1.schema.json",
    DOCS / "hepta.servo.worker_artifact_receipt.v1.schema.json",
]

COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
TREE = "b04d2f75b3217374d079d579c270177b57fa1389"

FALSE_BUILD_FIELDS = [
    "network_access_during_build",
    "worker_tcp_listener",
    "worker_http_surface",
    "worker_external_network",
    "worker_credential_export",
    "worker_production_authority",
    "worker_effect_authority",
]

FALSE_RUNTIME_FIELDS = [
    "artifact_executed",
    "servo_webview_started",
    "listener_scan_passed",
    "egress_scan_passed",
    "sandbox_qualified",
    "platform_matrix_qualified",
]

FALSE_AUTHORITY_FIELDS = [
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
]


def fail(message: str) -> None:
    raise ValueError(message)


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")


def require_files() -> None:
    required = [GENERATOR, TESTS, WORKFLOW, STATUS, PLAN, *SCHEMAS]
    missing = [str(path.relative_to(ROOT)) for path in required if not path.is_file()]
    if missing:
        fail(f"Servo artifact contract files are missing: {missing}")


def verify_generator() -> None:
    source = GENERATOR.read_text(encoding="utf-8")
    for forbidden in (
        "subprocess",
        "os.exec",
        "os.system",
        "Popen",
        "TcpListener",
        "0.0.0.0",
        "requests",
        "urllib",
        "socket.",
        "--expected-commit",
        "--expected-tree",
    ):
        if forbidden in source:
            fail(f"artifact generator contains forbidden execution/network marker: {forbidden}")
    for marker in (
        f'EXPECTED_SERVO_COMMIT = "{COMMIT}"',
        f'EXPECTED_SERVO_TREE = "{TREE}"',
        'EXPECTED_REPOSITORY = "https://github.com/servo/servo"',
        'ARTIFACT_RECEIPT_DOMAIN = b"hepta.servo.worker-artifact-receipt.v1"',
        'ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED',
        'ARTIFACT_DIGEST_AND_BUILD_INPUTS_ONLY',
        'stat.S_ISLNK',
        'st_nlink',
        'mode & 0o022',
        'identify_binary',
        'expected_binary_for_target',
        'SPDX-2.3',
        'os.O_EXCL',
        'os.fsync',
    ):
        if marker not in source:
            fail(f"artifact generator marker is missing: {marker}")
    if re.search(r"artifact_executed\s*[:=]\s*True", source):
        fail("artifact generator appears to claim execution")


def verify_status() -> None:
    status = load_json(STATUS)
    if not isinstance(status, dict):
        fail("artifact status must be an object")
    if status.get("status") != "TOOL_IMPLEMENTED_SYNTHETIC_FIXTURE_EVIDENCE_PENDING":
        fail("artifact status overclaims qualification")
    dependency = status.get("source_dependency")
    if not isinstance(dependency, dict):
        fail("artifact status lacks source dependency")
    if dependency.get("canonical_source_receipt") is not None or dependency.get("source_archive") is not None:
        fail("artifact status claims canonical source/archive evidence that is not present")
    implementation = status.get("implementation")
    if not isinstance(implementation, dict):
        fail("artifact status lacks implementation posture")
    for field in (
        "real_worker_artifact",
        "artifact_executed",
        "servo_webview_started",
        "listener_scan_passed",
        "egress_scan_passed",
        "sandbox_qualified",
        "platform_matrix_qualified",
    ):
        if implementation.get(field) is not False:
            fail(f"artifact status must keep {field}=false")
    authority = status.get("authority")
    if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
        fail("artifact status contains positive or non-boolean authority")
    if status.get("merge_authorized") is not False:
        fail("artifact status must not authorize merge")


def verify_build_schema(schema: dict[str, object]) -> None:
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        fail("build manifest schema lacks properties")
    if properties.get("source_receipt_id") is None:
        fail("build manifest schema lacks source receipt binding")
    if properties.get("target_triple") is None:
        fail("build manifest schema lacks target triple")
    features = properties.get("features")
    if not isinstance(features, dict) or features.get("uniqueItems") is not True:
        fail("build manifest features must be unique")
    for field in FALSE_BUILD_FIELDS:
        definition = properties.get(field)
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"build manifest field is not const false: {field}")


def verify_artifact_schema(schema: dict[str, object]) -> None:
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        fail("artifact receipt schema lacks properties")
    runtime = properties.get("runtime_qualification")
    if not isinstance(runtime, dict):
        fail("artifact receipt schema lacks runtime qualification posture")
    runtime_properties = runtime.get("properties")
    if not isinstance(runtime_properties, dict):
        fail("artifact receipt runtime posture is invalid")
    for field in FALSE_RUNTIME_FIELDS:
        definition = runtime_properties.get(field)
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"artifact runtime field is not const false: {field}")
    authority = properties.get("authority")
    if not isinstance(authority, dict):
        fail("artifact receipt schema lacks authority posture")
    authority_properties = authority.get("properties")
    if not isinstance(authority_properties, dict):
        fail("artifact receipt authority posture is invalid")
    for field in FALSE_AUTHORITY_FIELDS:
        definition = authority_properties.get(field)
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"artifact authority field is not const false: {field}")
    decision = properties.get("decision")
    if not isinstance(decision, dict) or decision.get("const") != "ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED":
        fail("artifact receipt decision is missing or overclaims")


def verify_schemas() -> None:
    schemas: dict[str, dict[str, object]] = {}
    for path in SCHEMAS:
        value = load_json(path)
        if not isinstance(value, dict):
            fail(f"schema must be an object: {path.name}")
        if value.get("additionalProperties") is not False:
            fail(f"schema must deny unknown top-level fields: {path.name}")
        identifier = value.get("$id")
        if not isinstance(identifier, str):
            fail(f"schema lacks $id: {path.name}")
        schemas[identifier] = value
    verify_build_schema(schemas["hepta.servo.worker_build_manifest.v1"])
    verify_artifact_schema(schemas["hepta.servo.worker_artifact_receipt.v1"])


def verify_tests_and_plan() -> None:
    tests = TESTS.read_text(encoding="utf-8")
    for test_name in (
        "test_artifact_receipt_binds_all_inputs_without_runtime_claim",
        "test_binary_format_and_target_must_match",
        "test_positive_build_or_worker_capability_fails_closed",
        "test_supporting_input_digest_tamper_fails_closed",
        "test_artifact_permissions_symlink_and_hardlink_fail_closed",
        "test_source_receipt_authority_or_identifier_tamper_fails_closed",
        "test_patch_and_license_inventories_are_strict",
    ):
        if test_name not in tests:
            fail(f"artifact contract regression test is missing: {test_name}")
    plan = PLAN.read_text(encoding="utf-8")
    for marker in (
        "REAL_ARTIFACT_NOT_BUILT",
        "ARTIFACT_DIGEST_AND_BUILD_INPUTS_ONLY",
        "ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED",
        COMMIT,
        TREE,
        "synthetic bounded executable headers",
        "C1-004B-5 independently repeat build",
    ):
        if marker not in plan:
            fail(f"artifact binding plan marker is missing: {marker}")
    for local_path in ("/Users/", "/Volumes/T5/", "/home/qian", "Dropbox/OpenClaw"):
        if local_path in plan:
            fail(f"artifact canonical plan contains a machine-local path: {local_path}")


def verify_workflow() -> None:
    workflow = WORKFLOW.read_text(encoding="utf-8")
    for marker in (
        "verify-hepta-servo-artifact-contract.py",
        "test_hepta_servo_artifact_receipt.py",
        "py_compile",
        "synthetic_fixture=true",
        "real_worker_artifact=false",
        "artifact_executed=false",
        "runtime_authority=false",
        "promotion=false",
    ):
        if marker not in workflow:
            fail(f"artifact contract workflow marker is missing: {marker}")
    if "git clone" in workflow or "servo/servo.git" in workflow or "cargo build" in workflow:
        fail("artifact fixture workflow must not fetch/build Servo")


def main() -> int:
    try:
        require_files()
        verify_generator()
        verify_status()
        verify_schemas()
        verify_tests_and_plan()
        verify_workflow()
    except (KeyError, ValueError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(
        json.dumps(
            {
                "status": "ARTIFACT_CONTRACT_INPUT_VERIFIED",
                "real_worker_artifact": False,
                "artifact_executed": False,
                "runtime_authority": False,
                "promotion": False,
                "servo_commit": COMMIT,
                "servo_tree": TREE,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
