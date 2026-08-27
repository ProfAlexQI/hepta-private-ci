#!/usr/bin/env python3
"""Verify the repository-native Servo source receipt contract without fetching Servo."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs" / "hepta-vnext" / "browser"
GENERATOR = ROOT / "scripts" / "hepta-servo-source-receipt.py"
TESTS = ROOT / "scripts" / "tests" / "test_hepta_servo_source_receipt.py"
WORKFLOW = ROOT / ".github" / "workflows" / "hepta-servo-source-contract.yml"
STATUS = DOCS / "SERVO_SOURCE_STATUS.json"
PLAN = DOCS / "SERVO_SOURCE_QUALIFICATION.md"
SCHEMA = DOCS / "hepta.servo.source_receipt.v1.schema.json"

COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
TREE = "b04d2f75b3217374d079d579c270177b57fa1389"

REQUIRED = [GENERATOR, TESTS, WORKFLOW, STATUS, PLAN, SCHEMA]
FORBIDDEN_GENERATOR_MARKERS = [
    "urllib",
    "requests",
    "http.client",
    "socket.",
    "--expected-commit",
    "--expected-tree",
    "--expected-repository",
]


def fail(message: str) -> None:
    raise ValueError(message)


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")


def verify_files() -> None:
    missing = [str(path.relative_to(ROOT)) for path in REQUIRED if not path.is_file()]
    if missing:
        fail(f"Servo source contract files are missing: {missing}")


def verify_generator() -> None:
    source = GENERATOR.read_text(encoding="utf-8")
    for marker in FORBIDDEN_GENERATOR_MARKERS:
        if marker in source:
            fail(f"source receipt generator contains forbidden marker: {marker}")
    for marker in (
        f'EXPECTED_COMMIT = "{COMMIT}"',
        f'EXPECTED_TREE = "{TREE}"',
        'EXPECTED_REPOSITORY = "https://github.com/servo/servo"',
        'EXPECTED_LICENSE = "MPL-2.0"',
        'MANIFEST_DOMAIN = b"hepta.servo.git-tree-manifest.v1"',
        'RECEIPT_DOMAIN = b"hepta.servo.source-receipt.v1"',
        '"--checkout"',
        '"--output"',
        '"--receipt"',
        '"--captured-at"',
        '"--porcelain=v1"',
        '"--untracked-files=all"',
        '"ls-tree", "-r", "-z", "--full-tree"',
        'os.O_EXCL',
        'os.fsync',
        'SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED',
    ):
        if marker not in source:
            fail(f"source receipt generator marker is missing: {marker}")
    if re.search(r"artifact_[a-z_]+\s*=\s*True", source):
        fail("source receipt generator appears to enable an artifact claim")


def verify_status() -> None:
    status = load_json(STATUS)
    if not isinstance(status, dict):
        fail("Servo source status must be an object")
    if status.get("status") != "TOOL_IMPLEMENTED_FIXTURE_QUALIFICATION_PENDING":
        fail("Servo source status overclaims canonical qualification")
    pin = status.get("pin")
    if not isinstance(pin, dict) or pin.get("commit") != COMMIT or pin.get("tree") != TREE:
        fail("Servo source status pin drifted")
    tooling = status.get("tooling")
    if not isinstance(tooling, dict):
        fail("Servo source status lacks tooling posture")
    for field in (
        "canonical_checkout_verified",
        "source_archive_created",
        "worker_artifact_built",
        "sbom_created",
    ):
        if tooling.get(field) is not False:
            fail(f"Servo source status must keep {field}=false")
    if tooling.get("canonical_source_receipt") is not None:
        fail("Servo source status must not point at a canonical receipt before evidence exists")
    authority = status.get("authority")
    if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
        fail("Servo source status contains positive or non-boolean authority")
    if status.get("merge_authorized") is not False:
        fail("Servo source status must not authorize merge")


def verify_schema() -> None:
    schema = load_json(SCHEMA)
    if not isinstance(schema, dict):
        fail("Servo source receipt schema must be an object")
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        fail("Servo source receipt schema lacks properties")
    source = properties.get("source")
    if not isinstance(source, dict):
        fail("Servo source receipt schema lacks source facts")
    source_properties = source.get("properties")
    if not isinstance(source_properties, dict):
        fail("Servo source receipt source properties are invalid")
    if source_properties.get("commit", {}).get("const") != COMMIT:
        fail("Servo receipt schema commit pin drifted")
    if source_properties.get("tree", {}).get("const") != TREE:
        fail("Servo receipt schema tree pin drifted")

    artifact = properties.get("artifact")
    if not isinstance(artifact, dict):
        fail("Servo source receipt schema lacks artifact posture")
    artifact_properties = artifact.get("properties")
    if not isinstance(artifact_properties, dict):
        fail("Servo source receipt artifact properties are invalid")
    for field in ("source_archive_created", "worker_artifact_built", "sbom_created"):
        if artifact_properties.get(field, {}).get("const") is not False:
            fail(f"Servo source receipt artifact field is not const false: {field}")
    for field in ("source_archive_sha256", "worker_artifact_sha256"):
        if artifact_properties.get(field, {}).get("type") != "null":
            fail(f"Servo source receipt digest field must remain null: {field}")

    authority = properties.get("authority")
    if not isinstance(authority, dict):
        fail("Servo source receipt schema lacks authority posture")
    authority_properties = authority.get("properties")
    if not isinstance(authority_properties, dict):
        fail("Servo source receipt authority properties are invalid")
    for field, definition in authority_properties.items():
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"Servo source receipt authority field is not const false: {field}")


def verify_tests_and_plan() -> None:
    tests = TESTS.read_text(encoding="utf-8")
    for name in (
        "test_collect_source_binds_exact_clean_tree_and_license",
        "test_receipt_is_canonical_self_bound_and_verifiable",
        "test_dirty_or_wrong_head_fails_closed",
        "test_unexpected_origin_fails_closed",
        "test_noncanonical_or_tampered_receipt_fails_closed",
        "test_receipt_id_detects_payload_change",
    ):
        if name not in tests:
            fail(f"Servo source receipt regression test is missing: {name}")

    plan = PLAN.read_text(encoding="utf-8")
    for marker in (
        "CANONICAL_CHECKOUT_NOT_YET_RECEIPTED",
        "BUILD_NOT_QUALIFIED",
        COMMIT,
        TREE,
        "SOURCE_PIN_AND_TREE_ONLY",
        "second independently fetched checkout",
    ):
        if marker not in plan:
            fail(f"Servo source qualification plan marker is missing: {marker}")
    for local_path in ("/Users/", "/Volumes/T5/", "/home/qian", "Dropbox/OpenClaw"):
        if local_path in plan:
            fail(f"Servo source canonical plan contains a machine-local path: {local_path}")


def verify_workflow() -> None:
    workflow = WORKFLOW.read_text(encoding="utf-8")
    for marker in (
        "verify-hepta-servo-source-contract.py",
        "test_hepta_servo_source_receipt.py",
        "py_compile",
        "canonical_checkout_verified=false",
        "source_archive_created=false",
        "worker_artifact_built=false",
        "runtime_authority=false",
        "promotion=false",
    ):
        if marker not in workflow:
            fail(f"Servo source contract workflow marker is missing: {marker}")
    if "git clone" in workflow or "servo/servo.git" in workflow:
        fail("fixture contract workflow must not fetch or clone Servo")


def main() -> int:
    try:
        verify_files()
        verify_generator()
        verify_status()
        verify_schema()
        verify_tests_and_plan()
        verify_workflow()
    except ValueError as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1
    print(
        json.dumps(
            {
                "status": "SOURCE_CONTRACT_INPUT_VERIFIED",
                "canonical_checkout_verified": False,
                "source_archive_created": False,
                "worker_artifact_built": False,
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
