#!/usr/bin/env python3
"""Static merge verifier for exact-source acceptance pointer v1."""
from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-exact-source-acceptance-pointer-v1.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_exact_source_acceptance_pointer_v1.py"
POLICY = ROOT / "docs/hepta-vnext/browser/SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json"
POLICY_SCHEMA = (
    ROOT
    / "docs/hepta-vnext/browser/hepta.servo.source_acceptance_review_policy.v1.schema.json"
)
CHALLENGE_SCHEMA = (
    ROOT
    / "docs/hepta-vnext/browser/hepta.servo.source_acceptance_review_challenge.v1.schema.json"
)
POINTER_SCHEMA = (
    ROOT / "docs/hepta-vnext/browser/hepta.servo.accepted_source_pointer.v1.schema.json"
)
SPEC = ROOT / "docs/hepta-vnext/browser/C1_EXACT_SOURCE_ACCEPTANCE_POINTER_V1.md"
STATUS = (
    ROOT
    / "docs/hepta-vnext/browser/C1_EXACT_SOURCE_ACCEPTANCE_POINTER_V1_STATUS.json"
)
LANE_README = ROOT / "docs/hepta-vnext/browser/source-acceptance/README.md"
WORKFLOW = (
    ROOT
    / ".github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml"
)
AGGREGATE = ROOT / ".github/workflows/hepta-browser-next-required-v8.yml"
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
CURRENT = ROOT / "docs/hepta-vnext/browser/CURRENT.yaml"
C1_CURRENT = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V6.json"
README = ROOT / "docs/hepta-vnext/browser/README.md"


def fail(message: str) -> None:
    raise RuntimeError(message)


def canonical(value: object) -> bytes:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")


def require_tokens(text: str, tokens: tuple[str, ...], label: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"{label} is missing {token!r}")


def main() -> int:
    try:
        paths = (
            TOOL,
            TEST,
            POLICY,
            POLICY_SCHEMA,
            CHALLENGE_SCHEMA,
            POINTER_SCHEMA,
            SPEC,
            STATUS,
            LANE_README,
            WORKFLOW,
            AGGREGATE,
            BLOCKING,
            CURRENT,
            C1_CURRENT,
            README,
        )
        for path in paths:
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")

        tool = TOOL.read_text(encoding="utf-8")
        test = TEST.read_text(encoding="utf-8")
        workflow = WORKFLOW.read_text(encoding="utf-8")
        aggregate = AGGREGATE.read_text(encoding="utf-8")
        blocking = BLOCKING.read_text(encoding="utf-8")
        spec = SPEC.read_text(encoding="utf-8")
        lane_readme = LANE_README.read_text(encoding="utf-8")
        readme = README.read_text(encoding="utf-8")
        status = json.loads(STATUS.read_text(encoding="utf-8"))
        policy = json.loads(POLICY.read_text(encoding="utf-8"))
        current = json.loads(CURRENT.read_text(encoding="utf-8"))
        c1_current = json.loads(C1_CURRENT.read_text(encoding="utf-8"))

        for path in (POLICY, POLICY_SCHEMA, CHALLENGE_SCHEMA, POINTER_SCHEMA, STATUS, C1_CURRENT):
            value = json.loads(path.read_text(encoding="utf-8"))
            if path.read_bytes() != canonical(value):
                fail(f"{path.relative_to(ROOT)} is not compact canonical JSON")

        require_tokens(
            tool,
            (
                "compile_challenge",
                "validate_pointer",
                "verify_live_review",
                "reviewer_must_differ_from_pr_author",
                "require_current_head_commit",
                "reject_changes_requested",
                "REQUIRES_LIVE_APPROVAL_EVIDENCE",
                "HEPTA_SOURCE_ACCEPT_V1 ",
                "EXACT_SOURCE_ACCEPTED_TOPOLOGY_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED",
                "PASS_LIVE_REVIEW_SOURCE_ONLY",
            ),
            "acceptance pointer tool",
        )
        for forbidden in (
            'subparsers.add_parser("accept")',
            'subparsers.add_parser("create-pointer")',
            'subparsers.add_parser("update-pointer")',
            "git push",
            "update_ref",
        ):
            if forbidden in tool:
                fail(f"acceptance pointer tool contains forbidden mutation surface {forbidden!r}")

        required_tests = (
            "test_contract_is_closed",
            "test_challenge_is_self_bound",
            "test_challenge_output_is_create_only_0600",
            "test_pointer_verifies_but_build_stays_closed",
            "test_live_review_passes_with_distinct_current_head_approval",
            "test_self_approval_is_rejected",
            "test_stale_commit_approval_is_rejected",
            "test_current_head_change_request_is_rejected",
            "test_review_without_exact_challenge_line_is_rejected",
            "test_untrusted_review_association_is_rejected",
            "test_unknown_changed_path_is_rejected",
            "test_missing_candidate_snapshot_is_rejected",
            "test_draft_acceptance_pr_is_rejected",
            "test_pointer_build_authority_is_rejected",
            "test_pointer_topology_acceptance_is_rejected",
            "test_positive_authority_is_rejected",
            "test_candidate_acceptance_claim_is_rejected",
            "test_policy_tamper_is_rejected",
            "test_challenge_id_tamper_is_rejected",
            "test_pointer_id_tamper_is_rejected",
        )
        for token in required_tests:
            if token not in test:
                fail(f"acceptance fixture suite is missing {token}")

        if policy.get("review", {}).get("minimum_approvals") != 1:
            fail("review policy minimum approval count drifted")
        if policy.get("review", {}).get("reviewer_must_differ_from_pr_author") is not True:
            fail("review policy permits self-approval")
        if policy.get("review", {}).get("require_current_head_commit") is not True:
            fail("review policy permits stale approvals")
        if policy.get("review", {}).get("reject_changes_requested") is not True:
            fail("review policy permits unresolved change requests")
        if policy.get("review", {}).get("draft_allowed") is not False:
            fail("review policy permits draft acceptance PRs")
        if policy.get("claims_after_acceptance", {}).get("exact_servo_source_accepted") is not True:
            fail("review policy does not express source-only acceptance")
        for key in (
            "worker_source_topology_accepted",
            "build_recipe_accepted",
            "build_authorized",
            "servo_built",
            "worker_artifact_created",
            "servo_runtime_qualified",
            "operator_acceptance",
            "promotion",
            "release_qualified",
        ):
            if policy.get("claims_after_acceptance", {}).get(key) is not False:
                fail(f"review policy overclaims {key}")
        if any(value is not False for value in policy.get("authority", {}).values()):
            fail("review policy authority posture is open")

        require_tokens(
            workflow,
            (
                "workflow_call:",
                "scripts/hepta-servo-exact-source-acceptance-pointer-v1.py contract",
                "scripts/tests/test_hepta_servo_exact_source_acceptance_pointer_v1.py -v",
                "scripts/verify-hepta-servo-exact-source-acceptance-pointer-v1.py",
                "pointer_creation_command=false",
                "exact_servo_source_accepted=false",
                "build_authorized=false",
                "servo_runtime_qualified=false",
            ),
            "acceptance pointer workflow",
        )
        require_tokens(
            aggregate,
            (
                "workflow_call:",
                "uses: ./.github/workflows/hepta-browser-next-required-v7.yml",
                "uses: ./.github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml",
                "name: Hepta Browser next required v8",
                '"exact_servo_source_accepted": False',
                '"source_review_candidate_accepted": False',
                '"build_authorized": False',
                '"servo_build_run": False',
                '"servo_runtime_qualified": False',
            ),
            "aggregate v8",
        )
        require_tokens(
            blocking,
            (
                "hepta-browser-next-v8:",
                "uses: ./.github/workflows/hepta-browser-next-required-v8.yml",
                "- hepta-browser-next-v8",
                "name: CI required",
            ),
            "blocking CI",
        )

        if status.get("status") != (
            "IMPLEMENTED_FIXTURE_ONLY_REAL_CANDIDATE_AND_REVIEW_ABSENT"
        ):
            fail("acceptance pointer status overclaims")
        if status.get("evidence", {}).get("local_fixture_tests") != "20_PASS":
            fail("acceptance pointer status does not record 20 fixture passes")
        if status.get("claims", {}).get("exact_servo_source_accepted") is not False:
            fail("acceptance pointer status claims source acceptance")
        if status.get("claims", {}).get("build_authorized") is not False:
            fail("acceptance pointer status claims build authority")
        if any(value is not False for value in status.get("authority", {}).values()):
            fail("acceptance pointer status authority posture is open")

        require_tokens(
            spec,
            (
                "The tool has no command that creates the accepted pointer",
                "reviewer must differ from the PR author",
                "HEPTA_SOURCE_ACCEPT_V1 <challenge_id>",
                "does not authorize a build",
            ),
            "acceptance pointer specification",
        )
        require_tokens(
            lane_readme,
            (
                "ACCEPTED_SOURCE_POINTER.json",
                "Creating them is permitted only in a dedicated non-draft review PR",
                "does not authorize a Servo build",
            ),
            "source acceptance lane README",
        )
        if current.get("c1_current") != "docs/hepta-vnext/browser/C1_CURRENT_V6.json":
            fail("CURRENT.yaml does not select C1 current v6")
        if current.get("canonical_aggregate_workflow") != (
            ".github/workflows/hepta-browser-next-required-v8.yml"
        ):
            fail("CURRENT.yaml does not select aggregate v8")
        if c1_current.get("claims", {}).get("exact_servo_source_accepted") is not False:
            fail("C1 current v6 claims source acceptance")
        if c1_current.get("claims", {}).get("build_authorized") is not False:
            fail("C1 current v6 claims build authority")
        if "Canonical C1 pointer: `C1_CURRENT_V6.json`" not in readme:
            fail("browser README does not select C1 current v6")

        result = subprocess.run(
            [
                sys.executable,
                str(TOOL),
                "contract",
                "--policy",
                str(POLICY),
            ],
            cwd=ROOT,
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
        )
        if result.returncode != 0:
            fail(f"acceptance pointer contract failed: {(result.stderr or result.stdout)[-1000:]}")
        summary = json.loads(result.stdout)
        if summary.get("status") != "PASS_CONTRACT_ONLY":
            fail("acceptance pointer contract summary drifted")
        if summary.get("pointer_creation_command") is not False:
            fail("acceptance pointer contract exposes pointer creation")
    except (
        OSError,
        RuntimeError,
        UnicodeError,
        json.JSONDecodeError,
        subprocess.TimeoutExpired,
    ) as error:
        print(
            f"HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V1_STATIC=FAIL: {error}",
            file=sys.stderr,
        )
        return 1

    print(
        json.dumps(
            {
                "status": "HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V1_STATIC_PASS",
                "fixture_tests": "20_PASS",
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
