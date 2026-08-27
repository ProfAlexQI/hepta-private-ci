#!/usr/bin/env python3
"""Verify the canonical bounded Servo build-preflight contract."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
V1 = ROOT / "scripts/hepta-servo-build-preflight.py"
V2 = ROOT / "scripts/hepta-servo-build-preflight-v2.py"
V1_TESTS = ROOT / "scripts/tests/test_hepta_servo_build_preflight.py"
V2_TESTS = ROOT / "scripts/tests/test_hepta_servo_build_preflight_v2.py"
SCHEMA = BUNDLE / "hepta.servo.build_preflight.v1.schema.json"
DOCUMENT = BUNDLE / "C1_BUILD_PREFLIGHT.md"
PROGRESS = BUNDLE / "C1_BUILD_PREFLIGHT_PROGRESS.json"
WORKFLOW = ROOT / ".github/workflows/hepta-servo-build-preflight-contract.yml"
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


def verify_schema() -> None:
    schema = load_json(SCHEMA)
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        fail("build-preflight schema must use draft 2020-12")
    if schema.get("additionalProperties") is not False:
        fail("build-preflight schema must reject unknown fields")
    properties = schema.get("properties", {})
    if properties.get("status") != {"const": "READY_FOR_SEPARATE_BOUNDED_BUILD"}:
        fail("build-preflight status is not frozen")
    build = properties.get("build", {}).get("properties", {})
    if build.get("target") != {"const": "x86_64-unknown-linux-gnu"}:
        fail("build-preflight schema does not bind the first Linux target")
    for key in ("build_network", "runtime_external_network"):
        if build.get(key) != {"const": False}:
            fail(f"build-preflight schema must keep {key}=false")
    preflight = properties.get("preflight", {}).get("properties", {})
    if preflight.get("ready_for_separate_bounded_build") != {"const": True}:
        fail("build-preflight schema does not require readiness proof")
    for key in (
        "build_run",
        "artifact_created",
        "sbom_created",
        "servo_runtime_qualified",
        "operator_accepted",
        "release_qualified",
    ):
        if preflight.get(key) != {"const": False}:
            fail(f"build-preflight schema must keep {key}=false")
    authority = schema.get("$defs", {}).get("authority", {}).get("properties", {})
    if set(authority) != AUTHORITY_KEYS:
        fail("build-preflight authority keys differ")
    if any(definition != {"const": False} for definition in authority.values()):
        fail("build-preflight schema may not enable authority")


def verify_v1() -> None:
    text = V1.read_text(encoding="utf-8")
    required = (
        EXPECTED_COMMIT,
        EXPECTED_TREE,
        "SOURCE_VERIFICATION_SCHEMA",
        "TOOLCHAIN_SCHEMA",
        "RECIPE_SCHEMA",
        "BUILD_INPUT_SCHEMA",
        "compressed_source_archive_sha256",
        "source_verification_receipt_sha256",
        "toolchain_receipt_sha256",
        "build_input_manifest_sha256",
        "toolchain_binaries_rehashed",
        "private_empty_directory",
        "os.path.samefile(source_root, artifact_root)",
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
            fail(f"build-preflight core is missing invariant {token!r}")
    for forbidden in (
        "subprocess.run(",
        "subprocess.Popen(",
        "os.system(",
        "shell=True",
        "cargo build\"",
        "requests.",
        "urllib.request",
    ):
        if forbidden in text:
            fail(f"build-preflight core contains forbidden execution surface {forbidden!r}")


def verify_v2() -> None:
    text = V2.read_text(encoding="utf-8")
    for token in (
        "TOOLCHAIN_CAPTURE_ENVIRONMENT",
        "ALLOWED_LINKER_KINDS",
        "PATH_FREE_VERSION",
        "expected_commands",
        "capture.get(\"minimal_environment\")",
        "module.toolchain_projection = toolchain_projection",
    ):
        if token not in text:
            fail(f"build-preflight v2 is missing invariant {token!r}")
    for forbidden in ("subprocess.run(", "os.system(", "shell=True", "eval(", "exec("):
        if forbidden in text:
            fail(f"build-preflight v2 contains forbidden surface {forbidden!r}")


def verify_tests() -> None:
    text = V1_TESTS.read_text(encoding="utf-8")
    for name in (
        "test_valid_inputs_are_ready_only_for_a_separate_bounded_build",
        "test_preflight_is_deterministic_for_unchanged_inputs",
        "test_tampered_source_archive_fails_closed",
        "test_source_receipt_digest_mismatch_fails_closed",
        "test_recipe_digest_or_projection_mismatch_fails_closed",
        "test_toolchain_receipt_digest_mismatch_fails_closed",
        "test_toolchain_binary_tamper_fails_closed",
        "test_symlink_or_hardlink_toolchain_binary_fails_closed",
        "test_nonempty_or_public_root_fails_closed",
        "test_same_source_and_artifact_root_fails_closed",
        "test_command_or_environment_drift_fails_closed",
        "test_open_authority_fails_closed",
        "test_private_atomic_output_refuses_overwrite",
    ):
        if f"def {name}" not in text:
            fail(f"build-preflight core test is missing: {name}")
    text = V2_TESTS.read_text(encoding="utf-8")
    for name in (
        "test_exact_capture_commands_environment_and_version_text_pass",
        "test_capture_command_drift_fails_closed",
        "test_capture_environment_extra_or_missing_key_fails_closed",
        "test_unlisted_linker_kind_fails_closed",
        "test_version_text_with_path_backslash_or_shell_character_fails_closed",
    ):
        if f"def {name}" not in text:
            fail(f"build-preflight v2 test is missing: {name}")


def verify_progress() -> None:
    progress = load_json(PROGRESS)
    if progress.get("schema") != "hepta.browser.c1_build_preflight_progress.v1":
        fail("build-preflight progress schema is invalid")
    if progress.get("claim_level") != "BUILD_PREFLIGHT_TOOLING_ONLY":
        fail("build-preflight progress must remain tooling-only")
    for key in (
        "actual_source_verification_receipt",
        "actual_toolchain_receipt",
        "actual_build_recipe",
        "actual_build_input_manifest",
        "actual_preflight_receipt",
    ):
        if progress.get(key) is not None:
            fail(f"build-preflight progress may not claim {key}")
    actual_build = progress.get("actual_build")
    if not isinstance(actual_build, dict) or actual_build.get("status") != "NOT_RUN":
        fail("build-preflight progress may not claim a build")
    authority = progress.get("authority")
    if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
        fail("build-preflight progress authority keys differ")
    if any(value is not False for value in authority.values()):
        fail("build-preflight progress attempted to enable authority")


def verify_workflow() -> None:
    text = WORKFLOW.read_text(encoding="utf-8")
    for token in (
        "workflow_call:",
        "pull_request:",
        "python3 scripts/tests/test_hepta_servo_build_preflight.py",
        "python3 scripts/tests/test_hepta_servo_build_preflight_v2.py",
        "python3 scripts/verify-hepta-servo-build-preflight-contract.py",
        "actual_preflight_receipt=false",
        "build_run=false",
        "artifact_created=false",
        "external_network=false",
    ):
        if token not in text:
            fail(f"build-preflight workflow is missing {token}")
    if "scripts/hepta-servo-build-preflight-v2.py --source-verification" in text:
        fail("contract workflow must not run a real build preflight")


def main() -> int:
    try:
        for path in (V1, V2, V1_TESTS, V2_TESTS, SCHEMA, DOCUMENT, PROGRESS, WORKFLOW):
            if not path.is_file():
                fail(f"missing build-preflight contract file: {path.relative_to(ROOT)}")
        verify_schema()
        verify_v1()
        verify_v2()
        verify_tests()
        verify_progress()
        verify_workflow()
    except ContractError as error:
        print(f"HEPTA_SERVO_BUILD_PREFLIGHT_CONTRACT=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": "hepta.browser.servo_build_preflight_contract_check.v1",
                "status": "PASS_TOOLING_CONTRACT_ONLY",
                "commit": EXPECTED_COMMIT,
                "tree": EXPECTED_TREE,
                "actual_preflight_receipt": False,
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
