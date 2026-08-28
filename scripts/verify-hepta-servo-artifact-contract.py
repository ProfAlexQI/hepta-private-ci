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
SERVO_PIN = DOCS / "SERVO_UPSTREAM_PIN.json"
C1_CURRENT = DOCS / "C1_CURRENT_V7.json"
SCHEMAS = [
    DOCS / "hepta.servo.worker_build_manifest.v1.schema.json",
    DOCS / "hepta.servo.patch_inventory.v1.schema.json",
    DOCS / "hepta.servo.license_packet.v1.schema.json",
    DOCS / "hepta.servo.worker_artifact_receipt.v1.schema.json",
]

COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
FALSE_BUILD_FIELDS = (
    "network_access_during_build",
    "worker_tcp_listener",
    "worker_http_surface",
    "worker_external_network",
    "worker_credential_export",
    "worker_production_authority",
    "worker_effect_authority",
)
FALSE_RUNTIME_FIELDS = (
    "artifact_executed",
    "servo_webview_started",
    "listener_scan_passed",
    "egress_scan_passed",
    "sandbox_qualified",
    "platform_matrix_qualified",
)
FALSE_AUTHORITY_FIELDS = (
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
)


def fail(message: str) -> None:
    raise ValueError(message)


def load_json(path: Path) -> dict[str, object]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"expected one JSON object: {path.relative_to(ROOT)}")
    return value


def require_files() -> None:
    required = [
        GENERATOR,
        TESTS,
        WORKFLOW,
        STATUS,
        PLAN,
        SERVO_PIN,
        C1_CURRENT,
        *SCHEMAS,
    ]
    missing = [str(path.relative_to(ROOT)) for path in required if not path.is_file()]
    if missing:
        fail(f"Servo artifact contract files are missing: {missing}")


def verify_canonical_source_selection() -> None:
    pin = load_json(SERVO_PIN)
    expected_pin = {
        "schema": "hepta.browser.servo_upstream_pin.v1",
        "schema_version": 1,
        "repository": "servo/servo",
        "repository_url": "https://github.com/servo/servo",
        "commit": COMMIT,
        "tree": TREE,
        "license": "MPL-2.0",
        "integration_status": "SOURCE_PIN_ONLY_NOT_IMPORTED",
    }
    for key, expected in expected_pin.items():
        if pin.get(key) != expected:
            fail(f"canonical Servo pin field drifted: {key}")
    if pin.get("commit_signature_verified") is not True:
        fail("canonical Servo pin lacks verified commit signature posture")
    authority = pin.get("authority")
    if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
        fail("canonical Servo pin attempted to enable authority")

    current = load_json(C1_CURRENT)
    if current.get("schema") != "hepta.browser.c1_current.v7":
        fail("artifact contract is not bound to C1 current v7")
    if current.get("canonical_aggregate_workflow") != (
        ".github/workflows/hepta-browser-next-required-v9.yml"
    ):
        fail("C1 current does not select Browser aggregate v9")
    claims = current.get("claims")
    if not isinstance(claims, dict):
        fail("C1 current lacks claims posture")
    for field in (
        "artifact_created",
        "build_authorized",
        "build_run",
        "exact_servo_source_accepted",
        "servo_runtime_qualified",
        "worker_source_topology_accepted",
    ):
        if claims.get(field) is not False:
            fail(f"C1 current overclaims {field}")
    current_authority = current.get("authority")
    if not isinstance(current_authority, dict) or any(
        value is not False for value in current_authority.values()
    ):
        fail("C1 current attempted to enable authority")


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
            fail(f"artifact generator contains forbidden surface: {forbidden}")
    for marker in (
        f'EXPECTED_SERVO_COMMIT = "{COMMIT}"',
        f'EXPECTED_SERVO_TREE = "{TREE}"',
        'EXPECTED_REPOSITORY = "https://github.com/servo/servo"',
        'ARTIFACT_RECEIPT_DOMAIN = b"hepta.servo.worker-artifact-receipt.v1"',
        "ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED",
        "ARTIFACT_DIGEST_AND_BUILD_INPUTS_ONLY",
        "stat.S_ISLNK",
        "st_nlink",
        "mode & 0o022",
        "identify_binary",
        "expected_binary_for_target",
        "SPDX-2.3",
        "os.O_EXCL",
        "os.fsync",
    ):
        if marker not in source:
            fail(f"artifact generator marker is missing: {marker}")
    if re.search(r"artifact_executed\s*[:=]\s*True", source):
        fail("artifact generator appears to claim execution")


def verify_status() -> None:
    status = load_json(STATUS)
    if status.get("status") != "TOOL_IMPLEMENTED_SYNTHETIC_FIXTURE_EVIDENCE_PENDING":
        fail("artifact status overclaims qualification")
    dependency = status.get("source_dependency")
    if not isinstance(dependency, dict):
        fail("artifact status lacks source dependency")
    if dependency.get("canonical_source_receipt") is not None or dependency.get(
        "source_archive"
    ) is not None:
        fail("artifact status claims source evidence that is not present")
    implementation = status.get("implementation")
    if not isinstance(implementation, dict):
        fail("artifact status lacks implementation posture")
    for field in ("real_worker_artifact", *FALSE_RUNTIME_FIELDS):
        if implementation.get(field) is not False:
            fail(f"artifact status must keep {field}=false")
    authority = status.get("authority")
    if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
        fail("artifact status contains positive authority")
    if status.get("merge_authorized") is not False:
        fail("artifact status must not authorize merge")


def verify_schemas() -> None:
    schemas: dict[str, dict[str, object]] = {}
    for path in SCHEMAS:
        value = load_json(path)
        if value.get("additionalProperties") is not False:
            fail(f"schema must deny unknown fields: {path.name}")
        identifier = value.get("$id")
        if not isinstance(identifier, str):
            fail(f"schema lacks $id: {path.name}")
        schemas[identifier] = value

    build = schemas["hepta.servo.worker_build_manifest.v1"].get("properties")
    if not isinstance(build, dict):
        fail("build manifest schema lacks properties")
    if build.get("source_receipt_id") is None or build.get("target_triple") is None:
        fail("build manifest lacks source/target binding")
    features = build.get("features")
    if not isinstance(features, dict) or features.get("uniqueItems") is not True:
        fail("build manifest features must be unique")
    for field in FALSE_BUILD_FIELDS:
        definition = build.get(field)
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"build manifest field is not const false: {field}")

    artifact = schemas["hepta.servo.worker_artifact_receipt.v1"].get("properties")
    if not isinstance(artifact, dict):
        fail("artifact receipt schema lacks properties")
    runtime = artifact.get("runtime_qualification")
    runtime_properties = runtime.get("properties") if isinstance(runtime, dict) else None
    if not isinstance(runtime_properties, dict):
        fail("artifact receipt runtime posture is invalid")
    for field in FALSE_RUNTIME_FIELDS:
        definition = runtime_properties.get(field)
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"artifact runtime field is not const false: {field}")
    authority = artifact.get("authority")
    authority_properties = authority.get("properties") if isinstance(authority, dict) else None
    if not isinstance(authority_properties, dict):
        fail("artifact receipt authority posture is invalid")
    for field in FALSE_AUTHORITY_FIELDS:
        definition = authority_properties.get(field)
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"artifact authority field is not const false: {field}")
    decision = artifact.get("decision")
    if not isinstance(decision, dict) or decision.get("const") != (
        "ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED"
    ):
        fail("artifact receipt decision is missing or overclaims")


def verify_tests_plan_and_workflow() -> None:
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
            fail(f"artifact regression test is missing: {test_name}")

    plan = PLAN.read_text(encoding="utf-8")
    for marker in (
        "REAL_ARTIFACT_NOT_BUILT",
        "ARTIFACT_DIGEST_AND_BUILD_INPUTS_ONLY",
        "ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED",
        "synthetic bounded executable headers",
        "C1-004B-5 independently repeat build",
    ):
        if marker not in plan:
            fail(f"artifact binding plan marker is missing: {marker}")
    for local_path in ("/Users/", "/Volumes/T5/", "/home/qian", "Dropbox/OpenClaw"):
        if local_path in plan:
            fail(f"artifact plan contains a machine-local path: {local_path}")

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
            fail(f"artifact workflow marker is missing: {marker}")
    if any(marker in workflow for marker in ("git clone", "servo/servo.git", "cargo build")):
        fail("artifact fixture workflow must not fetch or build Servo")


def main() -> int:
    try:
        require_files()
        verify_canonical_source_selection()
        verify_generator()
        verify_status()
        verify_schemas()
        verify_tests_plan_and_workflow()
    except (KeyError, OSError, UnicodeError, ValueError) as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(
        json.dumps(
            {
                "status": "ARTIFACT_CONTRACT_INPUT_VERIFIED",
                "canonical_current": "C1_CURRENT_V7",
                "canonical_aggregate": "hepta-browser-next-required-v9",
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
    raise SystemExit(main())
