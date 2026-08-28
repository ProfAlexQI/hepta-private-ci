#!/usr/bin/env python3
"""Verify Hepta architecture exact-head and merge-candidate CI identities stay separate."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXACT = ROOT / ".github/workflows/hepta-architecture-convergence.yml"
MERGE = ROOT / ".github/workflows/hepta-architecture-merge-candidate.yml"
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"


class VerificationError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def read(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as error:
        raise VerificationError(f"cannot read {path}: {error}") from error


def require_all(source: str, needles: tuple[str, ...], label: str) -> None:
    missing = [needle for needle in needles if needle not in source]
    require(not missing, f"{label} missing contracts: {missing}")


def require_none(source: str, needles: tuple[str, ...], label: str) -> None:
    present = [needle for needle in needles if needle in source]
    require(not present, f"{label} contains forbidden contracts: {present}")


def main() -> int:
    try:
        exact = read(EXACT)
        merge = read(MERGE)
        blocking = read(BLOCKING)

        require_all(
            exact,
            (
                "github.event.pull_request.head.sha || github.sha",
                "ref: ${{ env.CANDIDATE_SHA }}",
                'test "$(git rev-parse HEAD)" = "$CANDIDATE_SHA"',
                "Real product graph P0.2 (exact head, no authority)",
                "hepta-architecture-convergence-p0-2-${{ env.CANDIDATE_SHA }}",
                "source_verifier=passed",
                "qualified=false",
            ),
            "exact-head workflow",
        )
        require_none(
            exact,
            ("evidence_identity=merge_candidate", "source_head_claim=false"),
            "exact-head workflow",
        )

        require_all(
            merge,
            (
                "workflow_call:",
                "Architecture merge candidate",
                'test "$(git rev-parse HEAD)" = "$GITHUB_SHA"',
                "evidence_identity=merge_candidate",
                "source_head_claim=false",
                "PASS_ARCHITECTURE_MERGE_CANDIDATE",
                "hepta-architecture-merge-candidate-${{ github.sha }}",
                "qualified=false",
            ),
            "merge-candidate workflow",
        )
        require_none(
            merge,
            (
                "github.event.pull_request.head.sha || github.sha",
                "ref: ${{ env.CANDIDATE_SHA }}",
                "evidence_identity=source_head",
            ),
            "merge-candidate workflow",
        )

        require_all(
            blocking,
            (
                "hepta-architecture:",
                "uses: ./.github/workflows/hepta-architecture-merge-candidate.yml",
                "- hepta-architecture",
                "name: CI required",
            ),
            "blocking CI",
        )
        require(
            blocking.count("- hepta-architecture") == 1,
            "blocking CI must require the architecture job exactly once",
        )
        require(
            exact != merge,
            "exact-head and merge-candidate workflows must be distinct documents",
        )
    except VerificationError as error:
        print(f"FAIL_ARCHITECTURE_CI_IDENTITY_CONTRACT: {error}", file=sys.stderr)
        return 1

    print(
        json.dumps(
            {
                "result": "PASS_ARCHITECTURE_CI_IDENTITY_CONTRACT",
                "exact_head_identity": True,
                "merge_candidate_identity": True,
                "blocking_dependency": True,
                "identities_interchangeable": False,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
