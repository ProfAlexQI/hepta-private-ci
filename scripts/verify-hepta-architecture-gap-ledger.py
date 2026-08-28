#!/usr/bin/env python3
"""Fail-closed verifier for the canonical Hepta architecture gap ledger."""

from __future__ import annotations

import json
import pathlib
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
LEDGER = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V1.json"
CANONICAL_WORKFLOW = ROOT / ".github/workflows/hepta-architecture-convergence-p0-2.yml"
BLOCKING_WORKFLOW = ROOT / ".github/workflows/blocking-ci.yml"

RETIRED_WORKFLOWS = (
    ".github/workflows/hepta-architecture-convergence-p0-2-bootstrap.yml",
    ".github/workflows/hepta-architecture-convergence-p0-2-finalize.yml",
    ".github/workflows/hepta-architecture-convergence-p0-2-retire-bootstrap.yml",
    ".github/workflows/hepta-architecture-p0-2-portability-bootstrap.yml",
    ".github/workflows/hepta-memory-runtime-extraction-p0-1-bootstrap.yml",
    ".github/workflows/hepta-architecture-fault-matrix-p0-2.yml",
    ".github/workflows/hepta-authority-callers-p0-1.yml",
    ".github/workflows/hepta-legacy-production-authority-adapter-p0-1.yml",
    ".github/workflows/hepta-route-architecture-slim-once.yml",
)
RETIRED_MUTATORS = (
    "scripts/hepta-architecture-p0-2-portability-bootstrap.py",
    "scripts/hepta-architecture-p0-2-retire-bootstraps.py",
    "scripts/hepta-legacy-production-authority-adapter-p0-1.py",
    "scripts/hepta-memory-runtime-extraction-p0-1.py",
    "scripts/hepta-product-graph-authority-completion-p0-1-v2.py",
    "scripts/hepta-product-graph-authority-completion-p0-1.py",
)


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_ARCHITECTURE_GAP_LEDGER_V1: {message}")


def load_json_no_duplicates(path: pathlib.Path) -> dict[str, Any]:
    def pairs_hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                fail(f"duplicate JSON key {key!r} in {path.relative_to(ROOT)}")
            result[key] = value
        return result

    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs_hook)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail("gap ledger must contain one JSON object")
    return value


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")


def main() -> int:
    ledger = load_json_no_duplicates(LEDGER)
    if ledger.get("schema") != "hepta.architecture-gap-ledger.v1" or ledger.get("schemaVersion") != 1:
        fail("wrong gap-ledger schema")
    if ledger.get("canonicalBranch") != "codex/hepta-architecture-convergence-p0-2-20260828":
        fail("canonical architecture branch drifted")
    if ledger.get("canonicalPullRequest") != 53:
        fail("canonical architecture pull request drifted")

    closure = ledger.get("sourceClosure")
    if not isinstance(closure, dict) or not closure:
        fail("sourceClosure must be a non-empty object")
    for gap_id, record in closure.items():
        if not isinstance(record, dict) or record.get("state") != "closed":
            fail(f"source closure gap is not closed: {gap_id}")
        evidence = record.get("evidence")
        if not isinstance(evidence, list) or not evidence:
            fail(f"source closure gap has no evidence paths: {gap_id}")
        for relative in evidence:
            if not isinstance(relative, str) or not (ROOT / relative).is_file():
                fail(f"source closure evidence is missing: {gap_id}: {relative!r}")

    for relative in (*RETIRED_WORKFLOWS, *RETIRED_MUTATORS):
        if (ROOT / relative).exists():
            fail(f"retired source-mutating or duplicate qualification path still exists: {relative}")

    workflow = read(CANONICAL_WORKFLOW)
    for marker in (
        "name: Hepta architecture convergence P0",
        "workflow_call:",
        "workflow_dispatch:",
        "contents: read",
        "runs-on: ubuntu-24.04-arm",
        "Exact source-head architecture closure",
        "Merge-candidate architecture integration",
        "Hepta architecture convergence required",
        "python3 scripts/verify-hepta-architecture-gap-ledger.py",
        "source_mutation=false",
    ):
        if marker not in workflow:
            fail(f"canonical workflow marker is missing: {marker}")
    for forbidden in ("contents: write", "git push", "git commit", "persist-credentials: true"):
        if forbidden in workflow:
            fail(f"canonical workflow contains a write path: {forbidden}")

    blocking = read(BLOCKING_WORKFLOW)
    if "uses: ./.github/workflows/hepta-architecture-convergence-p0-2.yml" not in blocking:
        fail("blocking-ci does not call the canonical architecture workflow")
    if "- hepta-architecture-convergence" not in blocking:
        fail("blocking-ci required aggregator omits architecture convergence")

    hosted = ledger.get("hostedQualification")
    if not isinstance(hosted, dict):
        fail("hostedQualification must be an object")
    if hosted.get("requiredContext") != "Hepta architecture convergence required":
        fail("required hosted context drifted")
    if hosted.get("sourceMutationAllowed") is not False:
        fail("hosted qualification must remain read-only")
    if hosted.get("queuedOrEmptyJobCountsAsQualification") is not False:
        fail("queued/empty jobs cannot count as qualification")

    future = ledger.get("postP0ProductWork")
    if not isinstance(future, dict) or not future:
        fail("postP0ProductWork must remain explicit")
    for gap_id, state in future.items():
        if state not in {"planned_non_blocking", "source_implemented", "qualified"}:
            fail(f"invalid post-P0 work state: {gap_id}: {state!r}")

    decisions = ledger.get("externalDecisionGates")
    if not isinstance(decisions, dict) or any(state != "not_self_issued" for state in decisions.values()):
        fail("operator/promotion/release decisions must remain externally issued")
    authority = ledger.get("authorityBoundary")
    if not isinstance(authority, dict) or any(authority.values()):
        fail("gap ledger widened the authority boundary")

    print("PASS_HEPTA_ARCHITECTURE_GAP_LEDGER_V1_SOURCE_CLOSED_EXTERNAL_GATES_SEPARATE")
    return 0


if __name__ == "__main__":
    sys.exit(main())
