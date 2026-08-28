#!/usr/bin/env python3
"""Verify that canonical Hepta Browser v9 gates reach CI required."""
from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
V8 = ROOT / ".github/workflows/hepta-browser-next-required-v8.yml"
V9 = ROOT / ".github/workflows/hepta-browser-next-required-v9.yml"
SOURCE_LIVE = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-live-review-v2.yml"
TOPOLOGY_CONTRACT = ROOT / ".github/workflows/hepta-servo-worker-source-topology-acceptance-pointer-v1-contract.yml"
TOPOLOGY_LIVE = ROOT / ".github/workflows/hepta-servo-worker-source-topology-acceptance-live-review-v1.yml"
TOPOLOGY_TOOL = ROOT / "scripts/hepta-servo-worker-source-topology-acceptance-pointer-v1.py"
TOPOLOGY_PART_ROOT = ROOT / "scripts/hepta-servo-worker-source-topology-acceptance-v1"
TOPOLOGY_PARTS = tuple(TOPOLOGY_PART_ROOT / f"part{index:02d}.pyinc" for index in range(1, 6))
TOPOLOGY_STATIC = ROOT / "scripts/verify-hepta-servo-worker-source-topology-acceptance-pointer-v1.py"
TOPOLOGY_TEST = ROOT / "scripts/tests/test_hepta_servo_worker_source_topology_acceptance_pointer_v1.py"
TOPOLOGY_TEST_PART_ROOT = ROOT / "scripts/tests/hepta_servo_worker_source_topology_acceptance_v1"
TOPOLOGY_TEST_PARTS = tuple(
    TOPOLOGY_TEST_PART_ROOT / f"part{index:02d}.pyinc" for index in range(1, 4)
)
SOURCE_STATIC = ROOT / "scripts/verify-hepta-servo-exact-source-acceptance-pointer-v2.py"
POLICY = ROOT / "docs/hepta-vnext/browser/WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_REVIEW_POLICY_V1.json"
CURRENT = ROOT / "docs/hepta-vnext/browser/CURRENT.yaml"
C1 = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V7.json"
README = ROOT / "docs/hepta-vnext/browser/README.md"


def fail(message: str) -> None:
    raise RuntimeError(message)


def require(text: str, *tokens: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"missing token {token!r}")


def run_verifier(path: pathlib.Path, expected_status: str) -> None:
    result = subprocess.run(
        [sys.executable, str(path)],
        cwd=ROOT,
        capture_output=True,
        text=True,
        timeout=90,
        check=False,
    )
    if result.returncode != 0:
        fail(f"{path.name} failed: {(result.stderr or result.stdout)[-1200:]}")
    summary = json.loads(result.stdout.strip().splitlines()[-1])
    if summary.get("status") != expected_status:
        fail(f"{path.name} status drifted: {summary}")


def main() -> int:
    try:
        paths = (
            BLOCKING,
            V8,
            V9,
            SOURCE_LIVE,
            TOPOLOGY_CONTRACT,
            TOPOLOGY_LIVE,
            TOPOLOGY_TOOL,
            *TOPOLOGY_PARTS,
            TOPOLOGY_STATIC,
            TOPOLOGY_TEST,
            *TOPOLOGY_TEST_PARTS,
            SOURCE_STATIC,
            POLICY,
            CURRENT,
            C1,
            README,
        )
        for path in paths:
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")

        blocking = BLOCKING.read_text(encoding="utf-8")
        v8 = V8.read_text(encoding="utf-8")
        v9 = V9.read_text(encoding="utf-8")
        source_live = SOURCE_LIVE.read_text(encoding="utf-8")
        topology_contract = TOPOLOGY_CONTRACT.read_text(encoding="utf-8")
        topology_live = TOPOLOGY_LIVE.read_text(encoding="utf-8")
        topology_loader = TOPOLOGY_TOOL.read_text(encoding="utf-8")
        topology_tool = "".join(path.read_text(encoding="utf-8") for path in TOPOLOGY_PARTS)
        topology_test_loader = TOPOLOGY_TEST.read_text(encoding="utf-8")
        topology_test = "".join(path.read_text(encoding="utf-8") for path in TOPOLOGY_TEST_PARTS)
        policy = json.loads(POLICY.read_text(encoding="utf-8"))
        current = json.loads(CURRENT.read_text(encoding="utf-8"))
        c1 = json.loads(C1.read_text(encoding="utf-8"))
        readme = README.read_text(encoding="utf-8")

        require(
            v8,
            "workflow_call:",
            "hepta-servo-exact-source-acceptance-live-review-v2.yml",
            '"build_authorized": False',
        )
        require(
            v9,
            "workflow_call:",
            "hepta-browser-next-required-v8.yml",
            "hepta-servo-worker-source-topology-acceptance-pointer-v1-contract.yml",
            "name: Hepta Browser next required v9",
            "- canonical-v8",
            "- topology-acceptance-pointer-v1",
            '"worker_source_topology_accepted": False',
            '"build_authorized": False',
        )
        require(
            blocking,
            "hepta-browser-next-v9:",
            "uses: ./.github/workflows/hepta-browser-next-required-v9.yml",
            "- hepta-browser-next-v9",
            "name: CI required",
        )
        if "hepta-browser-next-v8:" in blocking or "- hepta-browser-next-v8" in blocking:
            fail("blocking CI still requires obsolete v8")
        if blocking.count("- hepta-browser-next-v9") != 1:
            fail("blocking CI must require canonical v9 exactly once")

        require(source_live, "pull_request_target:", "ref: ${{ github.event.pull_request.base.sha }}", "PR-head code executed: false")
        require(
            topology_live,
            "pull_request_target:",
            "ref: ${{ github.event.pull_request.base.sha }}",
            "PR-head code executed: false",
            "PASS_LIVE_REVIEW_WORKER_SOURCE_TOPOLOGY_ONLY",
        )
        for workflow in (source_live, topology_live):
            if "ref: ${{ github.event.pull_request.head.sha }}" in workflow:
                fail("live review executes PR-head verifier code")

        require(
            topology_contract,
            "workflow_call:",
            "test_hepta_servo_worker_source_topology_acceptance_pointer_v1.py -v",
            "pointer_creation_command=false",
            "worker_source_topology_accepted=false",
            "build_authorized=false",
        )
        require(topology_loader, "part{index:02d}.pyinc", "exec(compile(")
        require(topology_test_loader, "part{index:02d}.pyinc", "exec(compile(")
        require(
            topology_tool,
            'subparsers.add_parser("contract")',
            'subparsers.add_parser("challenge")',
            'subparsers.add_parser("verify-pointer")',
            'subparsers.add_parser("verify-live-review")',
            "accepted_source_pointer",
            "servoshell_dependency",
            "webdriver_server_dependency",
            "build_authorized",
        )
        for forbidden in (
            'subparsers.add_parser("accept")',
            'subparsers.add_parser("create-pointer")',
            "git push",
            "update_ref",
        ):
            if forbidden in topology_tool:
                fail(f"topology acceptance tool contains forbidden surface {forbidden!r}")
        require(
            topology_test,
            "test_unaccepted_source_pointer_is_rejected",
            "test_servoshell_widening_is_rejected",
            "test_self_approval_is_rejected",
            "test_current_head_change_request_is_rejected",
        )

        if policy.get("claims_after_acceptance", {}).get("worker_source_topology_accepted") is not True:
            fail("topology policy does not define topology-only acceptance")
        if policy.get("claims_after_acceptance", {}).get("build_authorized") is not False:
            fail("topology policy grants build authority")
        if any(value is not False for value in policy.get("authority", {}).values()):
            fail("topology policy authority is open")

        if current.get("c1_current") != "docs/hepta-vnext/browser/C1_CURRENT_V7.json":
            fail("root CURRENT does not select C1 v7")
        if current.get("canonical_aggregate_workflow") != ".github/workflows/hepta-browser-next-required-v9.yml":
            fail("root CURRENT does not select v9")
        if c1.get("canonical_aggregate_workflow") != ".github/workflows/hepta-browser-next-required-v9.yml":
            fail("C1 v7 does not select v9")
        if "Canonical C1 pointer: `C1_CURRENT_V7.json`" not in readme:
            fail("README does not identify C1 v7")

        run_verifier(
            SOURCE_STATIC,
            "HEPTA_SERVO_EXACT_SOURCE_ACCEPTANCE_POINTER_V2_STATIC_PASS",
        )
        run_verifier(
            TOPOLOGY_STATIC,
            "HEPTA_SERVO_WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_STATIC_PASS",
        )
    except (
        OSError,
        RuntimeError,
        UnicodeError,
        json.JSONDecodeError,
        subprocess.TimeoutExpired,
    ) as error:
        print(f"HEPTA_FOCUSED_GATES=FAIL: {error}", file=sys.stderr)
        return 1

    print(
        json.dumps(
            {
                "status": "HEPTA_FOCUSED_GATES_PASS",
                "canonical_aggregate": "hepta-browser-next-required-v9.yml",
                "blocking_ci_required": True,
                "trusted_base_source_review": True,
                "trusted_base_topology_review": True,
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
