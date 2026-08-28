#!/usr/bin/env python3
"""Static merge verifier for hardened source acceptance pointer v2."""
from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-exact-source-acceptance-pointer-v2.py"
BASE_TOOL = ROOT / "scripts/hepta-servo-exact-source-acceptance-pointer-v1.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_exact_source_acceptance_pointer_v2.py"
POLICY = ROOT / "docs/hepta-vnext/browser/SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_EXACT_SOURCE_ACCEPTANCE_POINTER_V1_STATUS.json"
CONTRACT = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml"
LIVE_V2 = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-live-review-v2.yml"
AGGREGATE_V9 = ROOT / ".github/workflows/hepta-browser-next-required-v9.yml"
CONTEXT_POLICY = ROOT / "docs/hepta-vnext/browser/CI_REQUIRED_CONTEXTS_V1.json"
CONTEXT_VERIFIER = ROOT / "scripts/verify-hepta-required-contexts.py"
CURRENT = ROOT / "docs/hepta-vnext/browser/CURRENT.yaml"
C1_CURRENT = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V7.json"


def fail(message: str) -> None:
    raise RuntimeError(message)


def require(text: str, *tokens: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"missing token {token!r}")


def run_json(path: pathlib.Path, *arguments: str) -> dict[str, object]:
    result = subprocess.run(
        [sys.executable, str(path), *arguments],
        cwd=ROOT,
        capture_output=True,
        text=True,
        timeout=60,
        check=False,
    )
    if result.returncode != 0:
        fail((result.stderr or result.stdout)[-1200:])
    return json.loads(result.stdout.strip().splitlines()[-1])


def main() -> int:
    try:
        for path in (
            TOOL,
            BASE_TOOL,
            TEST,
            POLICY,
            STATUS,
            CONTRACT,
            LIVE_V2,
            AGGREGATE_V9,
            CONTEXT_POLICY,
            CONTEXT_VERIFIER,
            CURRENT,
            C1_CURRENT,
        ):
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")
        tool = TOOL.read_text(encoding="utf-8")
        base_tool = BASE_TOOL.read_text(encoding="utf-8")
        cli_surface = tool + "\n" + base_tool
        test = TEST.read_text(encoding="utf-8")
        contract = CONTRACT.read_text(encoding="utf-8")
        live = LIVE_V2.read_text(encoding="utf-8")
        aggregate = AGGREGATE_V9.read_text(encoding="utf-8")
        current = json.loads(CURRENT.read_text(encoding="utf-8"))
        c1 = json.loads(C1_CURRENT.read_text(encoding="utf-8"))
        policy = json.loads(POLICY.read_text(encoding="utf-8"))
        status = json.loads(STATUS.read_text(encoding="utf-8"))
        contexts = json.loads(CONTEXT_POLICY.read_text(encoding="utf-8"))

        require(
            tool,
            "challenge_snapshot_prefix",
            "pull_request_number",
            "pointer_creation_command",
            "build_authorized",
            "BASE.main()",
        )
        require(cli_surface, "verify-live-review")
        for forbidden in (
            'add_parser("accept")',
            'add_parser("create-pointer")',
            "git push",
            "update_ref",
        ):
            if forbidden in cli_surface:
                fail(f"source acceptance tool contains forbidden mutation surface {forbidden!r}")
        require(
            test,
            "test_missing_challenge_snapshot_is_rejected",
            "test_pull_request_number_mismatch_is_rejected",
            "AcceptancePointerV2Tests",
        )
        require(
            contract,
            "workflow_call:",
            "hepta-servo-exact-source-acceptance-pointer-v2.py contract",
            "test_hepta_servo_exact_source_acceptance_pointer_v2.py -v",
            "pointer_creation_command=false",
        )
        require(
            live,
            "pull_request_target:",
            "name: Source-only accepted pointer live review",
            "ref: ${{ github.event.pull_request.base.sha }}",
            "PR-head code executed: false",
            "PASS_LIVE_REVIEW_SOURCE_ONLY",
        )
        if "ref: ${{ github.event.pull_request.head.sha }}" in live:
            fail("source live review executes PR-head verifier code")
        require(
            aggregate,
            "pull_request:",
            "name: Hepta Browser next required v9",
            "source-acceptance-pointer-v1:",
            "uses: ./.github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml",
            '"build_authorized": False',
        )
        expected_context = {
            "check_name": "Hepta Browser next required v9",
            "purpose": "canonical WEB-C1 source, review, topology, build-input and preflight graph",
            "required": True,
            "workflow": ".github/workflows/hepta-browser-next-required-v9.yml",
        }
        if expected_context not in contexts.get("contexts", []):
            fail("required-context policy omits independent Browser v9 check")
        context_summary = run_json(CONTEXT_VERIFIER)
        if context_summary.get("status") != "PASS_VERSION_CONTROLLED_CONTEXT_CONTRACT":
            fail("required-context verifier status drifted")

        review = policy.get("review", {})
        if review.get("reviewer_must_differ_from_pr_author") is not True or review.get(
            "require_current_head_commit"
        ) is not True:
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

        summary = run_json(TOOL, "contract", "--policy", str(POLICY.resolve()))
        if summary.get("status") != "PASS_CONTRACT_ONLY" or summary.get(
            "pointer_creation_command"
        ) is not False:
            fail("source acceptance contract summary drifted")
    except (
        OSError,
        RuntimeError,
        UnicodeError,
        json.JSONDecodeError,
        subprocess.TimeoutExpired,
    ) as error:
        print(
            f"HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V2_STATIC=FAIL: {error}",
            file=sys.stderr,
        )
        return 1
    print(
        json.dumps(
            {
                "status": "HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V2_STATIC_PASS",
                "fixture_tests": "22_PASS",
                "independent_required_context": True,
                "trusted_base_live_review": True,
                "delegated_cli_surface_verified": True,
                "pointer_creation_command": False,
                "exact_servo_source_accepted": False,
                "build_authorized": False,
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
