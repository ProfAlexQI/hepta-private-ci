#!/usr/bin/env python3
"""Verify that repository-native Hepta focused gates reach CI required."""
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
AGGREGATE = ROOT / ".github/workflows/hepta-browser-next-required-v7.yml"
BROWSER = ROOT / ".github/workflows/hepta-browser-ci.yml"
SOURCE_V3 = ROOT / ".github/workflows/hepta-servo-independent-source-contract-v3.yml"
SOURCE_REVIEW = ROOT / ".github/workflows/hepta-servo-exact-source-review-candidate-v2-contract.yml"
SOURCE_TOPOLOGY = ROOT / ".github/workflows/hepta-servo-worker-source-topology-contract.yml"
BUILD_INPUT = ROOT / ".github/workflows/hepta-servo-build-input-contract-v3.yml"
PREFLIGHT = ROOT / ".github/workflows/hepta-servo-build-preflight-contract.yml"
EXACT_SOURCE = ROOT / ".github/workflows/hepta-servo-independent-source-qualification-v3.yml"
CANDIDATE_STATIC = ROOT / "scripts/verify-hepta-servo-exact-source-review-candidate-v2.py"
TOPOLOGY_STATIC = ROOT / "scripts/verify-hepta-servo-worker-source-topology.py"


def fail(message: str) -> None:
    raise RuntimeError(message)


def require_tokens(text: str, tokens: tuple[str, ...], label: str) -> None:
    for token in tokens:
        if token not in text:
            fail(f"{label} is missing {token!r}")


def main() -> int:
    try:
        paths = (
            BLOCKING,
            AGGREGATE,
            BROWSER,
            SOURCE_V3,
            SOURCE_REVIEW,
            SOURCE_TOPOLOGY,
            BUILD_INPUT,
            PREFLIGHT,
            EXACT_SOURCE,
            CANDIDATE_STATIC,
            TOPOLOGY_STATIC,
        )
        for path in paths:
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")
        blocking = BLOCKING.read_text(encoding="utf-8")
        aggregate = AGGREGATE.read_text(encoding="utf-8")
        exact_source = EXACT_SOURCE.read_text(encoding="utf-8")
        for workflow in (BROWSER, SOURCE_V3, SOURCE_REVIEW, SOURCE_TOPOLOGY, BUILD_INPUT, PREFLIGHT):
            text = workflow.read_text(encoding="utf-8")
            if "workflow_call:" not in text:
                fail(f"{workflow.relative_to(ROOT)} is not reusable")
        require_tokens(
            aggregate,
            (
                "workflow_call:",
                "uses: ./.github/workflows/hepta-browser-ci.yml",
                "uses: ./.github/workflows/hepta-servo-independent-source-contract-v3.yml",
                "uses: ./.github/workflows/hepta-servo-exact-source-review-candidate-v2-contract.yml",
                "uses: ./.github/workflows/hepta-servo-worker-source-topology-contract.yml",
                "uses: ./.github/workflows/hepta-servo-build-input-contract-v3.yml",
                "uses: ./.github/workflows/hepta-servo-build-preflight-contract.yml",
                "name: Hepta Browser next required v7",
                "- exact-source-review-candidate-v2",
                '"status": "PASS_TOOLING_AND_FIXTURES_ONLY"',
                '"exact_servo_source_receipt": False',
                '"source_review_candidate_accepted": False',
                '"servo_build_run": False',
                '"servo_runtime_qualified": False',
                '"release_qualified": False',
            ),
            "canonical aggregate v7",
        )
        require_tokens(
            blocking,
            (
                "hepta-browser-next-v7:",
                "uses: ./.github/workflows/hepta-browser-next-required-v7.yml",
                "- hepta-browser-next-v7",
                "name: CI required",
                "if: ${{ always() }}",
                "python3 .github/scripts/check_ci_results.py",
            ),
            "blocking CI",
        )
        if blocking.count("- hepta-browser-next-v7") != 1:
            fail("blocking CI must require canonical v7 exactly once")
        if 'sha256sum "$output_dir"/*' in exact_source:
            fail("exact-source v3 still emits absolute runner paths")
        require_tokens(
            exact_source,
            (
                'required = {',
                'actual = {path.name for path in root.iterdir() if path.is_file()}',
                'for path in sorted(root.iterdir(), key=lambda item: item.name.encode("utf-8")):',
                'lines.append(f"{digest.hexdigest()}  {path.name}")',
                'os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)',
                'sha256sums_portable_basenames=true',
            ),
            "exact-source portable evidence",
        )
        candidate_static = CANDIDATE_STATIC.read_text(encoding="utf-8")
        topology_static = TOPOLOGY_STATIC.read_text(encoding="utf-8")
        require_tokens(
            candidate_static,
            (
                "PENDING_SEPARATE_REVIEW",
                "runner_id",
                "portable checksum",
                "exact_servo_source_accepted",
            ),
            "source-review static verifier",
        )
        require_tokens(
            topology_static,
            (
                "SERVO_WORKER_SOURCE_TOPOLOGY_V1.json",
                "servoshell",
                "webdriver",
            ),
            "source-topology static verifier",
        )
    except (OSError, RuntimeError) as error:
        print(f"HEPTA_FOCUSED_GATES=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "status": "HEPTA_FOCUSED_GATES_PASS",
                "canonical_aggregate": "hepta-browser-next-required-v7.yml",
                "blocking_ci_required": True,
                "portable_source_evidence": True,
                "separate_source_review_required": True,
                "exact_servo_source_accepted": False,
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
