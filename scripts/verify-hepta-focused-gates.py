#!/usr/bin/env python3
"""Verify canonical Hepta required workflows and review gates."""
from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
V9 = ROOT / ".github/workflows/hepta-browser-next-required-v9.yml"
VNEXT = ROOT / ".github/workflows/hepta-vnext-qualification.yml"
CONTEXT_POLICY = ROOT / "docs/hepta-vnext/browser/CI_REQUIRED_CONTEXTS_V1.json"
CONTEXT_VERIFIER = ROOT / "scripts/verify-hepta-required-contexts.py"
SOURCE_STATIC = ROOT / "scripts/verify-hepta-servo-exact-source-acceptance-pointer-v2.py"
TOPOLOGY_STATIC = ROOT / "scripts/verify-hepta-servo-worker-source-topology-acceptance-pointer-v1.py"
SOURCE_LIVE = ROOT / ".github/workflows/hepta-servo-exact-source-acceptance-live-review-v2.yml"
TOPOLOGY_LIVE = ROOT / ".github/workflows/hepta-servo-worker-source-topology-acceptance-live-review-v1.yml"
CURRENT = ROOT / "docs/hepta-vnext/browser/CURRENT.yaml"
C1 = ROOT / "docs/hepta-vnext/browser/C1_CURRENT_V7.json"
README = ROOT / "docs/hepta-vnext/browser/README.md"


def fail(message: str) -> None:
    raise RuntimeError(message)


def require(text: str, label: str, *tokens: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"{label} is missing {token!r}")


def run_verifier(path: pathlib.Path, expected_status: str) -> dict[str, object]:
    result = subprocess.run(
        [sys.executable, str(path)],
        cwd=ROOT,
        capture_output=True,
        text=True,
        timeout=120,
        check=False,
    )
    if result.returncode != 0:
        fail(f"{path.name} failed: {(result.stderr or result.stdout)[-1600:]}")
    summary = json.loads(result.stdout.strip().splitlines()[-1])
    if summary.get("status") != expected_status:
        fail(f"{path.name} status drifted: {summary}")
    return summary


def main() -> int:
    try:
        for path in (
            BLOCKING,
            V9,
            VNEXT,
            CONTEXT_POLICY,
            CONTEXT_VERIFIER,
            SOURCE_STATIC,
            TOPOLOGY_STATIC,
            SOURCE_LIVE,
            TOPOLOGY_LIVE,
            CURRENT,
            C1,
            README,
        ):
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")
        blocking = BLOCKING.read_text(encoding="utf-8")
        v9 = V9.read_text(encoding="utf-8")
        vnext = VNEXT.read_text(encoding="utf-8")
        source_live = SOURCE_LIVE.read_text(encoding="utf-8")
        topology_live = TOPOLOGY_LIVE.read_text(encoding="utf-8")
        contexts = json.loads(CONTEXT_POLICY.read_text(encoding="utf-8"))
        current = json.loads(CURRENT.read_text(encoding="utf-8"))
        c1 = json.loads(C1.read_text(encoding="utf-8"))
        readme = README.read_text(encoding="utf-8")

        require(
            blocking,
            "blocking CI",
            "pull_request:",
            "name: CI required",
            "Hepta required-context policy",
            "scripts/verify-hepta-required-contexts.py",
        )
        for nested in (
            "uses: ./.github/workflows/hepta-browser-next-required-v9.yml",
            "uses: ./.github/workflows/hepta-vnext-qualification.yml",
        ):
            if nested in blocking:
                fail(f"blocking CI nests independent required workflow {nested!r}")
        require(
            v9,
            "Browser v9",
            "pull_request:",
            "name: Hepta Browser next required v9",
            "top-level required workflow with workflow_call-only leaves",
            "uses: ./.github/workflows/hepta-browser-ci.yml",
            "uses: ./.github/workflows/hepta-browser-c1-protocol.yml",
            "uses: ./.github/workflows/hepta-browser-c1-startup-bridge.yml",
            "uses: ./.github/workflows/hepta-servo-independent-source-contract-v3.yml",
            "uses: ./.github/workflows/hepta-servo-exact-source-review-candidate-v2-contract.yml",
            "uses: ./.github/workflows/hepta-servo-worker-source-topology-contract.yml",
            "uses: ./.github/workflows/hepta-servo-exact-source-acceptance-pointer-v1-contract.yml",
            "uses: ./.github/workflows/hepta-servo-worker-source-topology-acceptance-pointer-v1-contract.yml",
            "uses: ./.github/workflows/hepta-servo-build-input-contract-v3.yml",
            "uses: ./.github/workflows/hepta-servo-build-preflight-contract.yml",
            "- private-protocol",
            "- startup-bridge",
            '"build_authorized": False',
            '"servo_runtime_qualified": False',
        )
        if "uses: ./.github/workflows/hepta-browser-next-required-v8.yml" in v9:
            fail("Browser v9 nests obsolete v8")
        require(
            vnext,
            "Hepta vNext",
            "pull_request:",
            "name: Hepta vNext required",
            "- portable-product",
            "- authbus-local-qualification",
            "- browser-c0-c3",
            "- generated-and-locks",
        )
        expected_names = [
            "CI required",
            "Hepta Browser next required v9",
            "Hepta vNext required",
        ]
        if [item.get("check_name") for item in contexts.get("contexts", [])] != expected_names:
            fail("required-context policy check names drifted")
        if contexts.get("enforcement", {}).get("single_workflow_aggregation") is not False:
            fail("required-context policy re-enabled single-workflow aggregation")

        require(
            source_live,
            "source live review",
            "pull_request_target:",
            "ref: ${{ github.event.pull_request.base.sha }}",
            "PR-head code executed: false",
        )
        require(
            topology_live,
            "topology live review",
            "pull_request_target:",
            "ref: ${{ github.event.pull_request.base.sha }}",
            "PR-head code executed: false",
            "PASS_LIVE_REVIEW_WORKER_SOURCE_TOPOLOGY_ONLY",
        )
        for workflow in (source_live, topology_live):
            if "ref: ${{ github.event.pull_request.head.sha }}" in workflow:
                fail("live review executes PR-head verifier code")

        if current.get("c1_current") != "docs/hepta-vnext/browser/C1_CURRENT_V7.json":
            fail("root CURRENT does not select C1 v7")
        if current.get("canonical_aggregate_workflow") != (
            ".github/workflows/hepta-browser-next-required-v9.yml"
        ):
            fail("root CURRENT does not select Browser v9")
        if c1.get("canonical_aggregate_workflow") != (
            ".github/workflows/hepta-browser-next-required-v9.yml"
        ):
            fail("C1 v7 does not select Browser v9")
        if "Canonical C1 pointer: `C1_CURRENT_V7.json`" not in readme:
            fail("README does not identify C1 v7")

        run_verifier(
            CONTEXT_VERIFIER,
            "PASS_VERSION_CONTROLLED_CONTEXT_CONTRACT",
        )
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
                "required_contexts": [
                    "CI required",
                    "Hepta Browser next required v9",
                    "Hepta vNext required",
                ],
                "workflow_call_only_leaves": True,
                "private_protocol_required": True,
                "startup_bridge_required": True,
                "trusted_base_source_review": True,
                "trusted_base_topology_review": True,
                "exact_servo_source_accepted": False,
                "worker_source_topology_accepted": False,
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
