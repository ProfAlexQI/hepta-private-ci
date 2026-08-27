#!/usr/bin/env python3
"""Verify the repository-native Servo toolchain receipt contract."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
CAPTURE = ROOT / "scripts/hepta-servo-toolchain-receipt.py"
TESTS = ROOT / "scripts/tests/test_hepta_servo_toolchain_receipt.py"
SCHEMA = BUNDLE / "hepta.servo.toolchain_receipt.v1.schema.json"
DOCUMENT = BUNDLE / "C1_TOOLCHAIN_RECEIPT.md"
PROGRESS = BUNDLE / "C1_TOOLCHAIN_PROGRESS.json"
WORKFLOW = ROOT / ".github/workflows/hepta-servo-toolchain-contract.yml"
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
        fail("toolchain receipt schema must use draft 2020-12")
    if schema.get("additionalProperties") is not False:
        fail("toolchain receipt schema must reject unknown fields")
    authority = schema.get("$defs", {}).get("authority", {}).get("properties", {})
    if set(authority) != AUTHORITY_KEYS:
        fail("toolchain receipt authority keys differ")
    for key, definition in authority.items():
        if definition != {"const": False}:
            fail(f"toolchain receipt may not enable {key}")
    capture = schema.get("properties", {}).get("capture", {}).get("properties", {})
    for key in ("network_access_used", "build_run", "artifact_created"):
        if capture.get(key) != {"const": False}:
            fail(f"toolchain receipt schema must keep {key}=false")


def verify_capture() -> None:
    text = CAPTURE.read_text(encoding="utf-8")
    required = (
        "metadata.st_nlink != 1",
        "metadata.st_mode & 0o022",
        "os.access(path, os.X_OK)",
        "subprocess.run(",
        "stdin=subprocess.DEVNULL",
        "stderr=subprocess.STDOUT",
        "timeout=timeout_seconds",
        "minimal_environment()",
        '"HOME"',
        '"PATH"',
        '"HTTPS_PROXY"',
        "commit_hash",
        "rustc_binary_sha256",
        "cargo_binary_sha256",
        "linker_binary_sha256",
        '"network_access_used": False',
        '"build_run": False',
        '"artifact_created": False',
        '"machine_local_paths_included": False',
        "os.O_EXCL",
        "0o600",
    )
    # HOME/PATH/HTTPS_PROXY are intentionally mentioned only by the tests and
    # documentation, not by the capture implementation. Check the actual
    # implementation for the positive minimal-environment keys separately.
    for token in required[:8] + required[11:]:
        if token not in text:
            fail(f"toolchain capture is missing invariant {token!r}")
    for token in (
        '"LC_ALL": "C"',
        '"LANG": "C"',
        '"TZ": "UTC"',
        '"GIT_TERMINAL_PROMPT": "0"',
    ):
        if token not in text:
            fail(f"toolchain capture minimal environment is missing {token}")
    forbidden = (
        "shell=True",
        "os.system(",
        "eval(",
        "exec(",
        "requests.",
        "urllib.request",
        "cargo build",
        "rustc --crate",
    )
    for token in forbidden:
        if token in text:
            fail(f"toolchain capture contains forbidden surface {token!r}")


def verify_tests() -> None:
    text = TESTS.read_text(encoding="utf-8")
    names = (
        "test_capture_binds_versions_binaries_and_closed_authority",
        "test_capture_is_deterministic_for_unchanged_binaries",
        "test_minimal_environment_drops_home_proxy_path_and_credentials",
        "test_symlink_and_hardlink_binaries_fail_closed",
        "test_group_writable_binary_fails_closed",
        "test_invalid_rustc_commit_hash_fails_closed",
        "test_mismatched_cargo_and_rustc_hosts_fail_closed",
        "test_oversized_version_output_fails_closed",
        "test_timeout_fails_closed",
        "test_private_atomic_output_refuses_overwrite",
    )
    for name in names:
        if f"def {name}" not in text:
            fail(f"toolchain receipt test is missing: {name}")


def verify_progress() -> None:
    progress = load_json(PROGRESS)
    if progress.get("schema") != "hepta.browser.c1_toolchain_progress.v1":
        fail("toolchain progress schema is invalid")
    if progress.get("claim_level") != "TOOLCHAIN_CAPTURE_TOOLING_ONLY":
        fail("toolchain progress must remain tooling-only")
    if progress.get("actual_toolchain_receipt") is not None:
        fail("toolchain progress may not claim an actual receipt")
    if progress.get("accepted_toolchain_receipt") is not None:
        fail("toolchain progress may not claim an accepted receipt")
    authority = progress.get("authority")
    if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
        fail("toolchain progress authority keys differ")
    if any(value is not False for value in authority.values()):
        fail("toolchain progress attempted to enable authority")


def verify_workflow() -> None:
    text = WORKFLOW.read_text(encoding="utf-8")
    for token in (
        "pull_request:",
        "workflow_call:",
        "python3 scripts/tests/test_hepta_servo_toolchain_receipt.py",
        "python3 scripts/verify-hepta-servo-toolchain-contract.py",
        "actual_toolchain_receipt=false",
        "build_run=false",
        "artifact_created=false",
        "external_network=false",
    ):
        if token not in text:
            fail(f"toolchain contract workflow is missing {token}")
    if "scripts/hepta-servo-toolchain-receipt.py --target" in text:
        fail("toolchain contract workflow must not capture and accept a real toolchain")


def main() -> int:
    try:
        for path in (CAPTURE, TESTS, SCHEMA, DOCUMENT, PROGRESS, WORKFLOW):
            if not path.is_file():
                fail(f"missing toolchain contract file: {path.relative_to(ROOT)}")
        verify_schema()
        verify_capture()
        verify_tests()
        verify_progress()
        verify_workflow()
    except ContractError as error:
        print(f"HEPTA_SERVO_TOOLCHAIN_CONTRACT=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": "hepta.browser.servo_toolchain_contract_check.v1",
                "status": "PASS_TOOLING_CONTRACT_ONLY",
                "actual_toolchain_receipt": False,
                "accepted_toolchain_receipt": False,
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
