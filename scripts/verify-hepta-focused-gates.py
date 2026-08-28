#!/usr/bin/env python3
"""Verify that the canonical Hepta Browser v8 gate reaches CI required."""
from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
AGGREGATE_V7 = ROOT / ".github/workflows/hepta-browser-next-required-v7.yml"
AGGREGATE_V8 = ROOT / ".github/workflows/hepta-browser-next-required-v8.yml"
ACCEPTANCE_WORKFLOW = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml"
ACCEPTANCE_TOOL = ROOT / "scripts/hepta-servo-exact-source-acceptance-pointer-v1.py"
ACCEPTANCE_STATIC = ROOT / "scripts/verify-hepta-servo-exact-source-acceptance-pointer-v1.py"
ACCEPTANCE_TEST = ROOT / "scripts/tests/test_hepta_servo_exact_source_acceptance_pointer_v1.py"
POLICY = ROOT / "docs/hepta-vnext/browser/SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json"
C1_CURRENT = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V6.json"
CURRENT = ROOT / "docs/hepta-vnext/browser/CURRENT.yaml"
README = ROOT / "docs/hepta-vnext/browser/README.md"


def fail(message: str) -> None:
    raise RuntimeError(message)


def require_tokens(text: str, tokens: tuple[str, ...], label: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"{label} is missing {token!r}")


def main() -> int:
    try:
        for path in (
            BLOCKING,
            AGGREGATE_V7,
            AGGREGATE_V8,
            ACCEPTANCE_WORKFLOW,
            ACCEPTANCE_TOOL,
            ACCEPTANCE_STATIC,
            ACCEPTANCE_TEST,
            POLICY,
            C1_CURRENT,
            CURRENT,
            README,
        ):
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")

        blocking = BLOCKING.read_text(encoding="utf-8")
        aggregate_v7 = AGGREGATE_V7.read_text(encoding="utf-8")
        aggregate_v8 = AGGREGATE_V8.read_text(encoding="utf-8")
        acceptance_workflow = ACCEPTANCE_WORKFLOW.read_text(encoding="utf-8")
        acceptance_tool = ACCEPTANCE_TOOL.read_text(encoding="utf-8")
        acceptance_static = ACCEPTANCE_STATIC.read_text(encoding="utf-8")
        acceptance_test = ACCEPTANCE_TEST.read_text(encoding="utf-8")
        current = json.loads(CURRENT.read_text(encoding="utf-8"))
        c1_current = json.loads(C1_CURRENT.read_text(encoding="utf-8"))
        policy = json.loads(POLICY.read_text(encoding="utf-8"))
        readme = README.read_text(encoding="utf-8")

        require_tokens(
            aggregate_v7,
            (
                "workflow_call:",
                "name: Hepta Browser next required v7",
                '"status": "PASS_TOOLING_AND_FIXTURES_ONLY"',
                '"exact_servo_source_receipt": False',
                '"servo_build_run": False',
                '"servo_runtime_qualified": False',
            ),
            "canonical aggregate v7",
        )
        require_tokens(
            aggregate_v8,
            (
                "workflow_call:",
                "uses: ./.github/workflows/hepta-browser-next-required-v7.yml",
                "uses: ./.github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml",
                "name: Hepta Browser next required v8",
                "- canonical-v7",
                "- source-acceptance-pointer-v1",
                '"status": "PASS_TOOLING_AND_FIXTURES_ONLY"',
                '"exact_servo_source_accepted": False',
                '"source_review_candidate_accepted": False',
                '"worker_source_topology_accepted": False',
                '"build_authorized": False',
                '"servo_build_run": False',
                '"servo_runtime_qualified": False',
                '"release_qualified": False',
            ),
            "canonical aggregate v8",
        )
        require_tokens(
            blocking,
            (
                "hepta-browser-next-v8:",
                "uses: ./.github/workflows/hepta-browser-next-required-v8.yml",
                "- hepta-browser-next-v8",
                "name: CI required",
                "if: ${{ always() }}",
                "python3 .github/scripts/check_ci_results.py",
            ),
            "blocking CI",
        )
        if blocking.count("- hepta-browser-next-v8") != 1:
            fail("blocking CI must require canonical v8 exactly once")
        if "hepta-browser-next-v7:" in blocking:
            fail("blocking CI must not require an obsolete aggregate alongside v8")

        require_tokens(
            acceptance_workflow,
            (
                "workflow_call:",
                "scripts/hepta-servo-exact-source-acceptance-pointer-v1.py contract",
                "scripts/tests/test_hepta_servo_exact_source_acceptance_pointer_v1.py -v",
                "scripts/verify-hepta-servo-exact-source-acceptance-pointer-v1.py",
                "pointer_creation_command=false",
                "exact_servo_source_accepted=false",
                "build_authorized=false",
                "servo_built=false",
                "servo_runtime_qualified=false",
            ),
            "source acceptance contract workflow",
        )
        require_tokens(
            acceptance_tool,
            (
                'subparsers.add_parser("contract")',
                'subparsers.add_parser("challenge")',
                'subparsers.add_parser("verify-challenge")',
                'subparsers.add_parser("verify-pointer")',
                'subparsers.add_parser("verify-live-review")',
                "reviewer_must_differ_from_pr_author",
                "require_current_head_commit",
                "REQUIRES_LIVE_APPROVAL_EVIDENCE",
                "EXACT_SOURCE_ACCEPTED_TOPOLOGY_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED",
                "build_authorized",
            ),
            "source acceptance verifier",
        )
        for forbidden in (
            'subparsers.add_parser("accept")',
            'subparsers.add_parser("create-pointer")',
            "git push",
            "update_ref",
        ):
            if forbidden in acceptance_tool:
                fail(f"source acceptance verifier contains forbidden mutation surface {forbidden!r}")
        require_tokens(
            acceptance_test,
            (
                "test_live_review_passes_with_distinct_current_head_approval",
                "test_self_approval_is_rejected",
                "test_stale_commit_approval_is_rejected",
                "test_current_head_change_request_is_rejected",
                "test_unknown_changed_path_is_rejected",
                "test_pointer_build_authority_is_rejected",
                "test_positive_authority_is_rejected",
            ),
            "source acceptance fixture suite",
        )
        require_tokens(
            acceptance_static,
            (
                "PASS_CONTRACT_ONLY",
                "20_PASS",
                "pointer_creation_command",
                "hepta-browser-next-required-v8.yml",
                "hepta-browser-next-v8",
            ),
            "source acceptance static verifier",
        )

        if policy.get("review", {}).get("reviewer_must_differ_from_pr_author") is not True:
            fail("source acceptance policy permits self-approval")
        if policy.get("review", {}).get("require_current_head_commit") is not True:
            fail("source acceptance policy permits stale approval")
        if policy.get("review", {}).get("draft_allowed") is not False:
            fail("source acceptance policy permits draft pointer PRs")
        if policy.get("claims_after_acceptance", {}).get("build_authorized") is not False:
            fail("source acceptance policy grants build authorization")
        if any(value is not False for value in policy.get("authority", {}).values()):
            fail("source acceptance policy authority posture is open")

        if current.get("c1_current") != "docs/hepta-vnext/browser/C1_CURRENT_V6.json":
            fail("CURRENT.yaml does not select C1 current v6")
        if current.get("canonical_aggregate_workflow") != ".github/workflows/hepta-browser-next-required-v8.yml":
            fail("CURRENT.yaml does not select aggregate v8")
        if c1_current.get("canonical_aggregate_workflow") != ".github/workflows/hepta-browser-next-required-v8.yml":
            fail("C1 current v6 does not select aggregate v8")
        if c1_current.get("claims", {}).get("exact_servo_source_accepted") is not False:
            fail("C1 current v6 overclaims source acceptance")
        if c1_current.get("claims", {}).get("build_authorized") is not False:
            fail("C1 current v6 overclaims build authority")
        if "Canonical C1 pointer: `C1_CURRENT_V6.json`" not in readme:
            fail("browser README does not identify C1 current v6")

        result = subprocess.run(
            [sys.executable, str(ACCEPTANCE_TOOL), "contract", "--policy", str(POLICY)],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
        if result.returncode != 0:
            fail(f"source acceptance contract command failed: {(result.stderr or result.stdout)[-1000:]}")
        summary = json.loads(result.stdout)
        if summary.get("status") != "PASS_CONTRACT_ONLY":
            fail("source acceptance contract summary drifted")
        if summary.get("pointer_creation_command") is not False:
            fail("source acceptance contract unexpectedly creates pointers")
    except (OSError, RuntimeError, UnicodeError, json.JSONDecodeError, subprocess.TimeoutExpired) as error:
        print(f"HEPTA_FOCUSED_GATES=FAIL: {error}", file=sys.stderr)
        return 1

    print(
        json.dumps(
            {
                "status": "HEPTA_FOCUSED_GATES_PASS",
                "canonical_aggregate": "hepta-browser-next-required-v8.yml",
                "blocking_ci_required": True,
                "source_acceptance_review_policy": "v1",
                "distinct_current_head_review_required": True,
                "pointer_creation_command": False,
                "exact_servo_source_accepted": False,
                "worker_source_topology_accepted": False,
                "build_authorized": False,
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
