#!/usr/bin/env python3
"""Verify the canonical Servo build-input v3 contract."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
V1 = ROOT / "scripts/hepta-servo-build-input-seal.py"
V2 = ROOT / "scripts/hepta-servo-build-input-seal-v2.py"
V3 = ROOT / "scripts/hepta-servo-build-input-seal-v3.py"
V3_TESTS = ROOT / "scripts/tests/test_hepta_servo_build_input_seal_v3.py"
POINTER = BUNDLE / "BUILD_INPUT_CURRENT_V2.json"
WORKFLOW = ROOT / ".github/workflows/hepta-servo-build-input-contract-v3.yml"
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
    if pointer.get("schema") != "hepta.browser.build_input_current.v2":
        fail("build-input v3 pointer schema is invalid")
    if pointer.get("canonical_sealer") != "scripts/hepta-servo-build-input-seal-v3.py":
        fail("build-input pointer does not select v3")
    for key in ("actual_recipe", "actual_toolchain_receipt", "actual_build_input_manifest"):
        if pointer.get(key) is not None:
            fail(f"build-input v3 pointer may not claim {key}")
    if pointer.get("build_run") is not False or pointer.get("artifact_created") is not False:
        fail("build-input v3 pointer may not claim a build or artifact")
    authority = pointer.get("authority")
    if not isinstance(authority, dict) or set(authority) != AUTHORITY_KEYS:
        fail("build-input v3 pointer authority keys differ")
    if any(value is not False for value in authority.values()):
        fail("build-input v3 pointer attempted to enable authority")


def verify_v3_code() -> None:
    text = V3.read_text(encoding="utf-8")
    for token in (
        "SAFE_VERSION_PATTERN",
        "hepta-servo-build-input-seal-v2.py",
        "base.VERSION_PATTERN = SAFE_VERSION_PATTERN",
        "module.load_base = load_patched_base",
        "A-Za-z0-9.+_ :()/,=@-",
    ):
        if token not in text:
            fail(f"build-input v3 entrypoint is missing {token!r}")
    for forbidden in ("shell=True", "subprocess.run(", "os.system(", "eval(", "exec("):
        if forbidden in text:
            fail(f"build-input v3 entrypoint contains forbidden surface {forbidden!r}")


def verify_tests() -> None:
    text = V3_TESTS.read_text(encoding="utf-8")
    for name in (
        "test_realistic_rustc_cargo_and_linker_versions_are_accepted",
        "test_control_characters_remain_rejected",
        "test_path_like_backslash_remains_rejected",
        "test_shell_metacharacters_remain_rejected",
    ):
        if f"def {name}" not in text:
            fail(f"build-input v3 test is missing: {name}")


def verify_workflow() -> None:
    text = WORKFLOW.read_text(encoding="utf-8")
    for token in (
        "workflow_call:",
        "pull_request:",
        "python3 scripts/tests/test_hepta_servo_build_input_seal_v3.py",
        "python3 scripts/verify-hepta-servo-build-input-v3-contract.py",
        "canonical_sealer=scripts/hepta-servo-build-input-seal-v3.py",
        "actual_build_input_manifest=false",
        "build_run=false",
        "artifact_created=false",
    ):
        if token not in text:
            fail(f"build-input v3 workflow is missing {token}")
    if "scripts/hepta-servo-build-input-seal-v3.py --source-verification" in text:
        fail("build-input v3 contract workflow must not seal real build inputs")


def main() -> int:
    try:
        for path in (V1, V2, V3, V3_TESTS, POINTER, WORKFLOW):
            if not path.is_file():
                fail(f"missing build-input v3 file: {path.relative_to(ROOT)}")
        verify_pointer()
        verify_v3_code()
        verify_tests()
        verify_workflow()
    except ContractError as error:
        print(f"HEPTA_SERVO_BUILD_INPUT_V3_CONTRACT=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": "hepta.browser.servo_build_input_v3_contract_check.v1",
                "status": "PASS_TOOLING_CONTRACT_ONLY",
                "canonical_sealer": "scripts/hepta-servo-build-input-seal-v3.py",
                "realistic_version_text": True,
                "actual_toolchain_receipt": False,
                "actual_build_input_manifest": False,
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
