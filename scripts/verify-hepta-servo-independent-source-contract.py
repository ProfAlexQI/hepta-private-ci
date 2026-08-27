#!/usr/bin/env python3
"""Verify the Hepta independent Servo source-pipeline contract."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
PIN = BUNDLE / "SERVO_UPSTREAM_PIN.json"
SCHEMA = BUNDLE / "hepta.servo.independent_source_bundle.v1.schema.json"
DOCUMENT = BUNDLE / "C1_INDEPENDENT_SOURCE_PIPELINE.md"
PROGRESS = BUNDLE / "C1_004A_PROGRESS.json"
SCRIPT = ROOT / "scripts/hepta-servo-independent-source.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_independent_source.py"
CONTRACT_WORKFLOW = ROOT / ".github/workflows/hepta-servo-independent-source-contract.yml"
QUALIFICATION_WORKFLOW = ROOT / ".github/workflows/hepta-servo-independent-source-qualification.yml"
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
    for path in (
        PIN,
        SCHEMA,
        DOCUMENT,
        PROGRESS,
        SCRIPT,
        TEST,
        CONTRACT_WORKFLOW,
        QUALIFICATION_WORKFLOW,
    ):
        if not path.is_file():
            fail(f"missing source-pipeline file: {path.relative_to(ROOT)}")


def verify_schema(pin: dict[str, Any]) -> None:
    schema = load_json(SCHEMA)
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        fail("independent source receipt schema must use draft 2020-12")
    if schema.get("additionalProperties") is not False:
        fail("independent source receipt schema must reject unknown fields")
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        fail("independent source receipt schema has no properties")
    source = properties.get("source", {}).get("properties", {})
    expected = {
        "repository": pin.get("repository"),
        "commit": pin.get("commit"),
        "tree": pin.get("tree"),
        "license": pin.get("license"),
    }
    for key, value in expected.items():
        if source.get(key) != {"const": value}:
            fail(f"receipt schema does not bind pinned source field {key}")
    authority = properties.get("authority", {}).get("properties", {})
    if set(authority) != AUTHORITY_KEYS:
        fail("receipt schema authority keys differ from the frozen set")
    for key, definition in authority.items():
        if definition != {"const": False}:
            fail(f"receipt schema may not enable {key}")
    qualification = properties.get("qualification", {}).get("properties", {})
    for key in ("servo_built", "servo_runtime_qualified", "operator_accepted", "release_qualified"):
        if qualification.get(key) != {"const": False}:
            fail(f"source-only schema must keep {key}=false")


def verify_script(pin: dict[str, Any]) -> None:
    text = SCRIPT.read_text(encoding="utf-8")
    required = (
        '"fetch_count": 2',
        '"--depth=1"',
        '"--filter=blob:none"',
        "alternate object database",
        "git_archive_tar",
        "gzip.GzipFile",
        "mtime=0",
        "source_projections_identical",
        "object_stores_distinct",
        "deterministic_tar_verified",
        "deterministic_gzip_verified",
        '"servo_built": False',
        '"servo_runtime_qualified": False',
        '"machine_local_paths_included": False',
        '"runtime_external_network": False',
    )
    for token in required:
        if token not in text:
            fail(f"independent source pipeline is missing invariant {token!r}")
    for value in (pin.get("commit"), pin.get("tree")):
        if not isinstance(value, str) or len(value) != 40:
            fail("Servo pin does not contain exact Git objects")
    forbidden = (
        "shell=True",
        "os.system(",
        "eval(",
        "exec(",
        "pickle",
        "requests.",
        "urllib.request",
        "curl ",
        "wget ",
        "TcpListener",
        "WebDriver",
    )
    for token in forbidden:
        if token in text:
            fail(f"independent source pipeline contains forbidden surface {token!r}")


def verify_tests() -> None:
    text = TEST.read_text(encoding="utf-8")
    required_tests = (
        "test_two_independent_fetches_produce_identical_archives",
        "test_same_fixture_repo_produces_stable_source_projection_and_archive",
        "test_local_origin_is_rejected_without_explicit_test_mode",
        "test_dirty_checkout_fails_closed",
        "test_alternate_object_database_fails_closed",
        "test_unsafe_archive_symlink_is_rejected",
    )
    for name in required_tests:
        if f"def {name}" not in text:
            fail(f"independent source-pipeline test is missing: {name}")


def verify_workflows(pin: dict[str, Any]) -> None:
    contract = CONTRACT_WORKFLOW.read_text(encoding="utf-8")
    qualification = QUALIFICATION_WORKFLOW.read_text(encoding="utf-8")
    for token in (
        "pull_request:",
        "workflow_call:",
        "python3 scripts/verify-hepta-servo-independent-source-contract.py",
        "python3 scripts/tests/test_hepta_servo_independent_source.py",
        "external_network=false",
        "servo_runtime_qualified=false",
    ):
        if token not in contract:
            fail(f"source contract workflow is missing {token}")
    for token in (
        "workflow_dispatch:",
        "SOURCE_ONLY_NO_RUNTIME_AUTHORITY",
        "python3 scripts/hepta-servo-independent-source.py",
        "--output-dir",
        "actions/upload-artifact@",
        "runtime_external_network=false",
        "production_caller=false",
        "promotion=false",
        str(pin.get("commit")),
        str(pin.get("tree")),
    ):
        if token not in qualification:
            fail(f"source qualification workflow is missing {token}")
    if "pull_request:" in qualification:
        fail("network-heavy independent source qualification must not run automatically on PRs")
    if "--test-only-allow-local-origin" in qualification:
        fail("canonical source qualification cannot use the local-origin test escape hatch")


def verify_progress() -> None:
    progress = load_json(PROGRESS)
    if progress.get("schema") != "hepta.browser.c1_004a_progress.v1":
        fail("C1-004A progress schema is invalid")
    if progress.get("phase") != "DEVELOPMENT" or progress.get("claim_level") != "SOURCE_TOOLING_ONLY":
        fail("C1-004A progress must remain development/source-tooling-only")
    actual = progress.get("actual_exact_servo_run")
    if not isinstance(actual, dict) or actual.get("status") != "NOT_RUN":
        fail("C1-004A may not claim an exact Servo run before evidence exists")
    authority = progress.get("authority")
    if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
        fail("C1-004A progress authority keys are invalid")
    if any(value is not False for value in authority.values()):
        fail("C1-004A progress attempted to enable authority")


def main() -> int:
    try:
        require_files()
        pin = load_json(PIN)
        verify_schema(pin)
        verify_script(pin)
        verify_tests()
        verify_workflows(pin)
        verify_progress()
    except ContractError as error:
        print(f"HEPTA_SERVO_INDEPENDENT_SOURCE_CONTRACT=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": "hepta.browser.servo_independent_source_contract_check.v1",
                "status": "PASS_TOOLING_CONTRACT_ONLY",
                "commit": pin["commit"],
                "tree": pin["tree"],
                "exact_servo_run": False,
                "servo_built": False,
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
