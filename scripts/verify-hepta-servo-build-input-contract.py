#!/usr/bin/env python3
"""Verify the repository-native Servo build-input sealing contract."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
SEALER = ROOT / "scripts/hepta-servo-build-input-seal.py"
TESTS = ROOT / "scripts/tests/test_hepta_servo_build_input_seal.py"
RECIPE_SCHEMA = BUNDLE / "hepta.servo.worker_build_recipe.v1.schema.json"
MANIFEST_SCHEMA = BUNDLE / "hepta.servo.build_input_manifest.v1.schema.json"
DOCUMENT = BUNDLE / "C1_BUILD_INPUT_CONTRACT.md"
PROGRESS = BUNDLE / "C1_004B_PROGRESS.json"
WORKFLOW = ROOT / ".github/workflows/hepta-servo-build-input-contract.yml"
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


def require_files() -> None:
    for path in (SEALER, TESTS, RECIPE_SCHEMA, MANIFEST_SCHEMA, DOCUMENT, PROGRESS, WORKFLOW):
        if not path.is_file():
            fail(f"missing build-input contract file: {path.relative_to(ROOT)}")


def verify_authority_schema(schema: dict[str, Any], label: str) -> None:
    properties = schema.get("$defs", {}).get("authority", {}).get("properties", {})
    if set(properties) != AUTHORITY_KEYS:
        fail(f"{label} authority keys differ")
    for key, definition in properties.items():
        if definition != {"const": False}:
            fail(f"{label} may not enable {key}")


def verify_schemas() -> None:
    recipe = load_json(RECIPE_SCHEMA)
    manifest = load_json(MANIFEST_SCHEMA)
    for schema, label in ((recipe, "recipe schema"), (manifest, "manifest schema")):
        if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
            fail(f"{label} must use JSON Schema draft 2020-12")
        if schema.get("additionalProperties") is not False:
            fail(f"{label} must reject unknown fields")
        verify_authority_schema(schema, label)
    recipe_properties = recipe.get("properties", {})
    for key in ("build_network", "source_mutation_allowed", "runtime_external_network"):
        if recipe_properties.get(key) != {"const": False}:
            fail(f"recipe schema must keep {key}=false")
    manifest_source = manifest.get("properties", {}).get("source", {}).get("properties", {})
    for key, expected in (("commit", EXPECTED_COMMIT), ("tree", EXPECTED_TREE), ("recomputed_tree", EXPECTED_TREE)):
        if manifest_source.get(key) != {"const": expected}:
            fail(f"manifest schema does not bind exact source field {key}")
    qualification = manifest.get("properties", {}).get("qualification", {}).get("properties", {})
    for key in ("build_run", "artifact_created", "sbom_created", "servo_runtime_qualified", "operator_accepted", "release_qualified"):
        if qualification.get(key) != {"const": False}:
            fail(f"manifest schema must keep {key}=false")


def verify_sealer() -> None:
    text = SEALER.read_text(encoding="utf-8")
    required = (
        EXPECTED_COMMIT,
        EXPECTED_TREE,
        "EXPECTED_SOURCE_VERIFICATION_SCHEMA",
        "recomputed_tree",
        '"--locked"',
        '"--offline"',
        '"--frozen"',
        '"--no-default-features"',
        "ALLOWED_ENVIRONMENT",
        "toolchain_binary_sha256",
        '"build_run": False',
        '"artifact_created": False',
        '"sbom_created": False',
        '"servo_runtime_qualified": False',
        '"machine_local_paths_included": False',
        "os.O_EXCL",
        "0o600",
    )
    for token in required:
        if token not in text:
            fail(f"build-input sealer is missing invariant {token!r}")
    forbidden = (
        "shell=True",
        "os.system(",
        "subprocess.run(",
        "subprocess.Popen(",
        "eval(",
        "exec(",
        "requests.",
        "urllib.request",
        "cargo build\"",
    )
    for token in forbidden:
        if token in text:
            fail(f"build-input sealer contains forbidden execution surface {token!r}")


def verify_tests() -> None:
    text = TESTS.read_text(encoding="utf-8")
    names = (
        "test_valid_inputs_produce_deterministic_build_not_run_manifest",
        "test_recomputed_tree_mismatch_fails_closed",
        "test_open_source_authority_fails_closed",
        "test_unsorted_or_duplicate_features_fail_closed",
        "test_command_requires_locked_offline_frozen_direct_cargo",
        "test_environment_must_equal_allowlist",
        "test_target_and_toolchain_target_must_match",
        "test_network_source_mutation_and_authority_cannot_be_enabled",
        "test_absolute_and_parent_paths_fail_closed",
        "test_atomic_output_is_private_and_refuses_overwrite",
    )
    for name in names:
        if f"def {name}" not in text:
            fail(f"build-input test is missing: {name}")


def verify_progress() -> None:
    progress = load_json(PROGRESS)
    if progress.get("schema") != "hepta.browser.c1_004b_progress.v1":
        fail("C1-004B progress schema is invalid")
    if progress.get("claim_level") != "BUILD_INPUT_TOOLING_ONLY":
        fail("C1-004B progress must remain tooling-only")
    if progress.get("actual_build_recipe") is not None:
        fail("C1-004B cannot claim an accepted build recipe")
    actual = progress.get("actual_build")
    if not isinstance(actual, dict) or actual.get("status") != "NOT_RUN":
        fail("C1-004B cannot claim a build run")
    authority = progress.get("authority")
    if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
        fail("C1-004B progress authority keys differ")
    if any(value is not False for value in authority.values()):
        fail("C1-004B progress attempted to enable authority")


def verify_workflow() -> None:
    text = WORKFLOW.read_text(encoding="utf-8")
    for token in (
        "pull_request:",
        "workflow_call:",
        "python3 scripts/tests/test_hepta_servo_build_input_seal.py",
        "python3 scripts/verify-hepta-servo-build-input-contract.py",
        "build_run=false",
        "artifact_created=false",
        "servo_runtime_qualified=false",
        "external_network=false",
    ):
        if token not in text:
            fail(f"build-input workflow is missing {token}")
    if "scripts/hepta-servo-build-input-seal.py --" in text:
        fail("contract workflow must not seal a real build manifest")


def main() -> int:
    try:
        require_files()
        verify_schemas()
        verify_sealer()
        verify_tests()
        verify_progress()
        verify_workflow()
    except ContractError as error:
        print(f"HEPTA_SERVO_BUILD_INPUT_CONTRACT=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": "hepta.browser.servo_build_input_contract_check.v1",
                "status": "PASS_TOOLING_CONTRACT_ONLY",
                "commit": EXPECTED_COMMIT,
                "tree": EXPECTED_TREE,
                "actual_recipe": False,
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
