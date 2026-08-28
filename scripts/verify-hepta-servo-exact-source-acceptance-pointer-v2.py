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
STATUS = ROOT / "docs/hepta-vnext/browser/C1_EXACT_SOURCE_ACCEPTANCE_POINTER_V1_STATUS.json"
CONTRACT = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml"
LIVE_V2 = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-live-review-v2.yml"
AGGREGATE_V8 = ROOT / ".github/workflows/hepta-browser-next-required-v8.yml"
AGGREGATE_V9 = ROOT / ".github/workflows/hepta-browser-next-required-v9.yml"
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
CURRENT = ROOT / "docs/hepta-vnext/browser/CURRENT.yaml"
C1_CURRENT = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V7.json"
README = ROOT / "docs/hepta-vnext/browser/README.md"


def fail(message: str) -> None:
    raise RuntimeError(message)


def require(text: str, *tokens: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"missing token {token!r}")


def main() -> int:
    try:
        for path in (TOOL, TEST, POLICY, STATUS, CONTRACT, LIVE_V2, AGGREGATE_V8, AGGREGATE_V9, BLOCKING, CURRENT, C1_CURRENT, README):
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")
        tool = TOOL.read_text(encoding="utf-8")
        test = TEST.read_text(encoding="utf-8")
        contract = CONTRACT.read_text(encoding="utf-8")
        live = LIVE_V2.read_text(encoding="utf-8")
        aggregate_v8 = AGGREGATE_V8.read_text(encoding="utf-8")
        aggregate_v9 = AGGREGATE_V9.read_text(encoding="utf-8")
        blocking = BLOCKING.read_text(encoding="utf-8")
        current = json.loads(CURRENT.read_text(encoding="utf-8"))
        c1 = json.loads(C1_CURRENT.read_text(encoding="utf-8"))
        policy = json.loads(POLICY.read_text(encoding="utf-8"))
        status = json.loads(STATUS.read_text(encoding="utf-8"))

        require(tool, "challenge_snapshot_prefix", "pull_request_number", "verify-live-review", "pointer_creation_command", "build_authorized")
        for forbidden in ('add_parser("accept")', 'add_parser("create-pointer")', "git push", "update_ref"):
            if forbidden in tool:
                fail(f"source acceptance tool contains forbidden mutation surface {forbidden!r}")
        require(test, "test_missing_challenge_snapshot_is_rejected", "test_pull_request_number_mismatch_is_rejected", "AcceptancePointerV2Tests")
        require(contract, "workflow_call:", "hepta-servo-exact-source-acceptance-pointer-v2.py contract", "test_hepta_servo_exact_source_acceptance_pointer_v2.py -v", "pointer_creation_command=false")
        require(live, "name: Source-only accepted pointer live review", "ref: ${{ github.event.pull_request.base.sha }}", "PR-head code executed: false", "accepted-source-pointer.json", "source-review-candidate.json", "source-review-challenge.json", "PASS_LIVE_REVIEW_SOURCE_ONLY")
        if "ref: ${{ github.event.pull_request.head.sha }}" in live:
            fail("source live review executes PR-head verifier code")
        require(aggregate_v8, "hepta-servo-exact-source-acceptance-live-review-v2.yml", "- source-acceptance-live-review-v2", '"build_authorized": False')
        require(aggregate_v9, "uses: ./.github/workflows/hepta-browser-next-required-v8.yml")
        require(blocking, "hepta-browser-next-v9:", "uses: ./.github/workflows/hepta-browser-next-required-v9.yml", "- hepta-browser-next-v9")

        review = policy.get("review", {})
        if review.get("reviewer_must_differ_from_pr_author") is not True or review.get("require_current_head_commit") is not True:
            fail("source review policy permits self or stale approval")
        if review.get("codeowner_review_required") is not False:
            fail("source review policy claims unenforced CODEOWNER identity")
        if policy.get("claims_after_acceptance", {}).get("build_authorized") is not False:
            fail("source review policy grants build authority")
        if any(value is not False for value in policy.get("authority", {}).values()):
            fail("source review policy authority posture is open")

        if current.get("c1_current") != "docs/hepta-vnext/browser/C1_CURRENT_V7.json":
            fail("root CURRENT does not select C1 v7")
        if current.get("source_acceptance_live_review_workflow") != (
            ".github/workflows/hepta-servo-exact-source-acceptance-live-review-v2.yml"
        ):
            fail("root CURRENT does not select trusted-base source live review v2")
        if c1.get("canonical_source_acceptance_live_review_workflow") != (
            ".github/workflows/hepta-servo-exact-source-acceptance-live-review-v2.yml"
        ):
            fail("C1 v7 does not select trusted-base source live review v2")
        if status.get("evidence", {}).get("local_fixture_tests") != "22_PASS":
            fail("source acceptance status does not record 22 fixture passes")

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
        if summary.get("status") != "PASS_CONTRACT_ONLY" or summary.get("pointer_creation_command") is not False:
            fail("source acceptance contract summary drifted")
    except (OSError, RuntimeError, UnicodeError, json.JSONDecodeError, subprocess.TimeoutExpired) as error:
        print(f"HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V2_STATIC=FAIL: {error}", file=sys.stderr)
        return 1
    print(json.dumps({
        "status": "HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V2_STATIC_PASS",
        "fixture_tests": "22_PASS",
        "trusted_base_live_review": True,
        "pr_head_verifier_executed": False,
        "pointer_creation_command": False,
        "exact_servo_source_accepted": False,
        "build_authorized": False,
        "servo_runtime_qualified": False,
        "authority": "all_false",
    }, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
