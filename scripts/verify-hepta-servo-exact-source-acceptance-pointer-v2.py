#!/usr/bin/env python3
"""Static merge verifier for hardened source acceptance pointer v2."""
from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-exact-source-acceptance-pointer-v2.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_exact_source_acceptance_pointer_v2.py"
POLICY = ROOT / "docs/hepta-vnext/browser/SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json"
POINTER_SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.accepted_source_pointer.v1.schema.json"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_EXACT_SOURCE_ACCEPTANCE_POINTER_V1_STATUS.json"
CONTRACT = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml"
LIVE = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-live-review.yml"
AGGREGATE = ROOT / ".github/workflows/hepta-browser-next-required-v8.yml"
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
CURRENT = ROOT / "docs/hepta-vnext/browser/CURRENT.yaml"
C1_CURRENT = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V6.json"
README = ROOT / "docs/hepta-vnext/browser/README.md"


def fail(message: str) -> None:
    raise RuntimeError(message)


def require(text: str, *tokens: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"missing token {token!r}")


def main() -> int:
    try:
        paths = (
            TOOL, TEST, POLICY, POINTER_SCHEMA, STATUS, CONTRACT, LIVE,
            AGGREGATE, BLOCKING, CURRENT, C1_CURRENT, README,
        )
        for path in paths:
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")
        tool = TOOL.read_text(encoding="utf-8")
        test = TEST.read_text(encoding="utf-8")
        contract_workflow = CONTRACT.read_text(encoding="utf-8")
        live = LIVE.read_text(encoding="utf-8")
        aggregate = AGGREGATE.read_text(encoding="utf-8")
        blocking = BLOCKING.read_text(encoding="utf-8")
        current = json.loads(CURRENT.read_text(encoding="utf-8"))
        c1 = json.loads(C1_CURRENT.read_text(encoding="utf-8"))
        policy = json.loads(POLICY.read_text(encoding="utf-8"))
        schema = json.loads(POINTER_SCHEMA.read_text(encoding="utf-8"))
        status = json.loads(STATUS.read_text(encoding="utf-8"))

        require(
            tool,
            "challenge_snapshot_prefix",
            "pull_request_number",
            "accepted pointer head ref differs from live review evidence",
            "source acceptance PR omits pointer, candidate, or challenge snapshot",
            "codeowner_identity_claimed",
            "pointer_creation_command",
        )
        for forbidden in ('add_parser("accept")', 'add_parser("create-pointer")', "git push"):
            if forbidden in tool:
                fail(f"tool contains forbidden mutation surface {forbidden!r}")
        require(
            test,
            "test_missing_challenge_snapshot_is_rejected",
            "test_pull_request_number_mismatch_is_rejected",
            "AcceptancePointerV2Tests",
        )
        review = policy.get("review", {})
        if review.get("codeowner_review_required") is not False:
            fail("policy claims an unenforced CODEOWNER identity check")
        for key in (
            "pointer_must_bind_pull_request_number",
            "pointer_must_bind_head_ref",
            "require_current_head_commit",
            "reviewer_must_differ_from_pr_author",
        ):
            if review.get(key) is not True:
                fail(f"policy does not require {key}")
        if review.get("required_status_check") != "Source-only accepted pointer live review":
            fail("policy status check name drifted")
        if "challenge_snapshot_prefix" not in policy:
            fail("policy does not retain challenge snapshots")
        pointer_props = schema.get("properties", {})
        if "snapshot_path" not in pointer_props.get("challenge", {}).get("properties", {}):
            fail("pointer schema does not bind challenge snapshot")
        review_props = pointer_props.get("review", {}).get("properties", {})
        for key in ("pull_request_number", "head_ref"):
            if key not in review_props:
                fail(f"pointer schema does not bind {key}")

        require(
            contract_workflow,
            "hepta-servo-exact-source-acceptance-pointer-v2.py contract",
            "test_hepta_servo_exact_source_acceptance_pointer_v2.py -v",
            "verify-hepta-servo-exact-source-acceptance-pointer-v2.py",
            "pointer_creation_command=false",
            "build_authorized=false",
        )
        require(
            live,
            "name: Source-only accepted pointer live review",
            "pull_request_review:",
            "GITHUB_TOKEN",
            "verify-live-review",
            "PASS_LIVE_REVIEW_SOURCE_ONLY",
            "build authorized: false",
        )
        require(
            aggregate,
            "uses: ./.github/workflows/hepta-servo-exact-source-acceptance-live-review.yml",
            "- source-acceptance-live-review",
            '"build_authorized": False',
        )
        require(
            blocking,
            "hepta-browser-next-v8:",
            "uses: ./.github/workflows/hepta-browser-next-required-v8.yml",
            "- hepta-browser-next-v8",
        )
        if current.get("c1_current") != "docs/hepta-vnext/browser/C1_CURRENT_V6.json":
            fail("root CURRENT does not select C1 v6")
        if current.get("source_acceptance_pointer_tool") != (
            "scripts/hepta-servo-exact-source-acceptance-pointer-v2.py"
        ):
            fail("root CURRENT does not select pointer tool v2")
        if c1.get("canonical_exact_source_acceptance_pointer_tool") != (
            "scripts/hepta-servo-exact-source-acceptance-pointer-v2.py"
        ):
            fail("C1 current does not select pointer tool v2")
        if status.get("evidence", {}).get("local_fixture_tests") != "22_PASS":
            fail("status does not record 22 fixture passes")
        for key in ("exact_servo_source_accepted", "build_authorized", "servo_built"):
            if status.get("claims", {}).get(key) is not False:
                fail(f"status overclaims {key}")

        command = subprocess.run(
            [sys.executable, str(TOOL), "contract", "--policy", str(POLICY.resolve())],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
        if command.returncode != 0:
            fail((command.stderr or command.stdout)[-1000:])
        summary = json.loads(command.stdout)
        if summary.get("status") != "PASS_CONTRACT_ONLY":
            fail("contract status drifted")
        if summary.get("codeowner_identity_claimed") is not False:
            fail("contract claims CODEOWNER identity")
        if summary.get("pointer_creation_command") is not False:
            fail("contract creates pointers")
    except (OSError, RuntimeError, UnicodeError, json.JSONDecodeError, subprocess.TimeoutExpired) as error:
        print(f"HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V2_STATIC=FAIL: {error}", file=sys.stderr)
        return 1
    print(json.dumps({
        "status": "HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V2_STATIC_PASS",
        "fixture_tests": "22_PASS",
        "live_review_workflow": True,
        "codeowner_identity_claimed": False,
        "pointer_creation_command": False,
        "exact_servo_source_accepted": False,
        "build_authorized": False,
        "servo_runtime_qualified": False,
        "authority": "all_false",
    }, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
