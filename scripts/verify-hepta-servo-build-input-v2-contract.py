#!/usr/bin/env python3
"""Verify the canonical toolchain-bound Servo build-input v2 contract."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
V1 = ROOT / "scripts/hepta-servo-build-input-seal.py"
V2 = ROOT / "scripts/hepta-servo-build-input-seal-v2.py"
TOOLCHAIN = ROOT / "scripts/hepta-servo-toolchain-receipt.py"
V1_TESTS = ROOT / "scripts/tests/test_hepta_servo_build_input_seal.py"
V2_TESTS = ROOT / "scripts/tests/test_hepta_servo_build_input_seal_v2.py"
TOOLCHAIN_TESTS = ROOT / "scripts/tests/test_hepta_servo_toolchain_receipt.py"
POINTER = BUNDLE / "BUILD_INPUT_CURRENT.json"
V2_SCHEMA = BUNDLE / "hepta.servo.build_input_manifest.v2.schema.json"
TOOLCHAIN_SCHEMA = BUNDLE / "hepta.servo.toolchain_receipt.v1.schema.json"
WORKFLOW = ROOT / ".github/workflows/hepta-servo-build-input-contract-v2.yml"
EXPECTED_COMMIT = "0a48e298482659817eb50097df23841f2b8e3044"
EXPECTED_TREE = "b04d2f75b3217374d079d579c270177b57fa1389"
AUTHORITY_KEYS = {
    "runtime_authority",
    "effect_authority",
    "production_caller",
    "production_writer",
    "runtime_external_network",
    "raw_cookie_export",
    "credential_export",
    "operator_acceptance",
    "promotion",
    "release_qualified",
}


class ContractError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise ContractError(message)


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one object")
    return value


def verify_pointer() -> None:
    pointer = load_json(POINTER)
    if pointer.get("schema") != "hepta.browser.build_input_current.v1":
        fail("build-input current pointer schema is invalid")
    if pointer.get("canonical_sealer") != "scripts/hepta-servo-build-input-seal-v2.py":
        fail("build-input current pointer does not select v2")
    if pointer.get("canonical_manifest_schema") != "docs/hepta-vnext/browser/hepta.servo.build_input_manifest.v2.schema.json":
        fail("build-input current pointer does not select manifest v2")
    for key in ("actual_recipe", "actual_toolchain_receipt", "actual_build_input_manifest"):
        if pointer.get(key) is not None:
            fail(f"build-input current pointer may not claim {key}")
    if pointer.get("build_run") is not False or pointer.get("artifact_created") is not False:
        fail("build-input current pointer may not claim a build or artifact")
    authority = pointer.get("authority")
    if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
        fail("build-input current pointer authority keys differ")
    if any(value is not False for value in authority.values()):
        fail("build-input current pointer attempted to enable authority")


def verify_schema(path: Path, label: str) -> dict[str, Any]:
    schema = load_json(path)
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        fail(f"{label} must use JSON Schema draft 2020-12")
    if schema.get("additionalProperties") is not False:
        fail(f"{label} must reject unknown fields")
    authority = schema.get("$defs", {}).get("authority", {}).get("properties", {})
    if set(authority) != AUTHORITY_KEYS:
        fail(f"{label} authority keys differ")
    if any(definition != {"const": False} for definition in authority.values()):
        fail(f"{label} may not enable authority")
    return schema


def verify_v2_schema() -> None:
    schema = verify_schema(V2_SCHEMA, "build-input v2 schema")
    source = schema.get("properties", {}).get("source", {}).get("properties", {})
    for key, expected in (
        ("commit", EXPECTED_COMMIT),
        ("tree", EXPECTED_TREE),
        ("recomputed_tree", EXPECTED_TREE),
    ):
        if source.get(key) != {"const": expected}:
            fail(f"build-input v2 schema does not bind {key}")
    if schema.get("properties", {}).get("toolchain_receipt_sha256", {}).get("pattern") not in (None, "^[0-9a-f]{64}$"):
        fail("build-input v2 toolchain receipt digest schema is invalid")
    qualification = schema.get("properties", {}).get("qualification", {}).get("properties", {})
    if qualification.get("toolchain_receipt_independently_captured") != {"const": True}:
        fail("build-input v2 schema does not require independent toolchain capture")
    for key in ("build_run", "artifact_created", "sbom_created", "servo_runtime_qualified", "operator_accepted", "release_qualified"):
        if qualification.get(key) != {"const": False}:
            fail(f"build-input v2 schema must keep {key}=false")
    verify_schema(TOOLCHAIN_SCHEMA, "toolchain receipt schema")


def verify_v2_code() -> None:
    text = V2.read_text(encoding="utf-8")
    for token in (
        "TOOLCHAIN_SCHEMA",
        "toolchain_receipt_sha256",
        "validate_toolchain_receipt",
        "recipe_projection != expected_projection",
        "toolchain_receipt_independently_captured",
        "base.seal(",
        "base.write_atomic(",
        '"build_run": False',
        '"artifact_created": False',
    ):
        if token not in text:
            fail(f"build-input v2 sealer is missing {token!r}")
    for token in ("shell=True", "subprocess.run(", "os.system(", "eval(", "exec("):
        if token in text:
            fail(f"build-input v2 sealer contains forbidden surface {token!r}")


def verify_tests() -> None:
    for path in (V1_TESTS, V2_TESTS, TOOLCHAIN_TESTS):
        if not path.is_file():
            fail(f"missing test file: {path.relative_to(ROOT)}")
    text = V2_TESTS.read_text(encoding="utf-8")
    for name in (
        "test_v2_binds_independent_toolchain_receipt_digest",
        "test_v2_is_deterministic_for_identical_receipts",
        "test_recipe_toolchain_version_mismatch_fails_closed",
        "test_recipe_toolchain_binary_digest_mismatch_fails_closed",
        "test_recipe_and_receipt_target_mismatch_fails_closed",
        "test_toolchain_network_or_build_claim_fails_closed",
        "test_toolchain_open_authority_fails_closed",
        "test_toolchain_machine_path_marker_fails_closed",
        "test_toolchain_component_host_mismatch_fails_closed",
    ):
        if f"def {name}" not in text:
            fail(f"build-input v2 test is missing: {name}")


def verify_workflow() -> None:
    text = WORKFLOW.read_text(encoding="utf-8")
    for token in (
        "workflow_call:",
        "pull_request:",
        "python3 scripts/tests/test_hepta_servo_toolchain_receipt.py",
        "python3 scripts/tests/test_hepta_servo_build_input_seal.py",
        "python3 scripts/tests/test_hepta_servo_build_input_seal_v2.py",
        "python3 scripts/verify-hepta-servo-toolchain-contract-v2.py",
        "python3 scripts/verify-hepta-servo-build-input-v2-contract.py",
        "actual_toolchain_receipt=false",
        "actual_build_input_manifest=false",
        "build_run=false",
        "artifact_created=false",
    ):
        if token not in text:
            fail(f"build-input v2 workflow is missing {token}")
    if "scripts/hepta-servo-build-input-seal-v2.py --source-verification" in text:
        fail("contract workflow must not seal an actual build-input manifest")


def main() -> int:
    try:
        for path in (V1, V2, TOOLCHAIN, POINTER, V2_SCHEMA, TOOLCHAIN_SCHEMA, WORKFLOW):
            if not path.is_file():
                fail(f"missing canonical build-input v2 file: {path.relative_to(ROOT)}")
        verify_pointer()
        verify_v2_schema()
        verify_v2_code()
        verify_tests()
        verify_workflow()
    except ContractError as error:
        print(f"HEPTA_SERVO_BUILD_INPUT_V2_CONTRACT=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": "hepta.browser.servo_build_input_v2_contract_check.v1",
                "status": "PASS_TOOLING_CONTRACT_ONLY",
                "commit": EXPECTED_COMMIT,
                "tree": EXPECTED_TREE,
                "toolchain_receipt_bound": True,
                "actual_recipe": False,
                "actual_toolchain_receipt": False,
                "build_run": False,
                "artifact_created": False,
                "servo_runtime_qualified": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
