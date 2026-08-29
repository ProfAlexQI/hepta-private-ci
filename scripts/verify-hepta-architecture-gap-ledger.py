#!/usr/bin/env python3
"""Fail-closed verifier for the canonical Hepta architecture gap ledger."""

from __future__ import annotations

import json
import pathlib
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
LEDGER = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V1.json"
WORKFLOW_ROOT = ROOT / ".github/workflows"

FORBIDDEN_MUTATING_PATHS = (
    ".github/workflows/hepta-architecture-convergence-p0-2-bootstrap.yml",
    ".github/workflows/hepta-architecture-convergence-p0-2-finalize.yml",
    ".github/workflows/hepta-architecture-convergence-p0-2-retire-bootstrap.yml",
    ".github/workflows/hepta-architecture-p0-2-portability-bootstrap.yml",
    ".github/workflows/hepta-memory-runtime-extraction-p0-1-bootstrap.yml",
    ".github/workflows/hepta-architecture-fault-matrix-p0-2.yml",
    ".github/workflows/hepta-authority-callers-p0-1.yml",
    ".github/workflows/hepta-legacy-production-authority-adapter-p0-1.yml",
    ".github/workflows/hepta-route-architecture-slim-once.yml",
    ".github/workflows/hepta-memory-runtime-lock-refresh-once.yml",
    ".github/workflows/hepta-automation-operation-repair-once.yml",
    ".github/workflows/hepta-operation-safety-repair-slim-once.yml",
    ".github/workflows/hepta-operation-materialize-once.yml",
    ".github/workflows/hepta-gap-closure-once.yml",
    ".github/workflows/hepta-automation-operation-tests-finalize-once.yml",
    ".github/workflows/hepta-automation-operation-tests-finalize-v2.yml",
    ".github/workflows/hepta-automation-operation-tests-finalize-v3.yml",
    ".github/workflows/hepta-resource-governor-once.yml",
    ".github/workflows/hepta-resource-governor-finalize-once.yml",
    ".github/workflows/hepta-source-snapshot-once.yml",
    "scripts/hepta-resource-governor-once.py",
)


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_ARCHITECTURE_GAP_LEDGER_V1: {message}")


def load_json_no_duplicates(path: pathlib.Path) -> dict[str, Any]:
    def pairs_hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                fail(f"duplicate JSON key {key!r}")
            result[key] = value
        return result

    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs_hook)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse ledger: {error}")
    if not isinstance(value, dict):
        fail("ledger must contain one object")
    return value


def reject_architecture_source_mutators() -> None:
    for relative in FORBIDDEN_MUTATING_PATHS:
        if (ROOT / relative).exists():
            fail(f"retired source-mutating path still exists: {relative}")

    for path in WORKFLOW_ROOT.glob("hepta-*"):
        if not path.is_file():
            continue
        name = path.name.lower()
        if not any(
            token in name
            for token in (
                "architecture",
                "authority",
                "memory-runtime",
                "product-graph",
                "operation",
                "resource-governor",
            )
        ):
            continue
        content = path.read_text(encoding="utf-8")
        for marker in (
            "permissions:\n  contents: write",
            "persist-credentials: true",
            "git push",
            "git commit",
            "git update-ref",
        ):
            if marker in content:
                fail(f"architecture workflow mutates source: {path.relative_to(ROOT)}: {marker}")


def main() -> int:
    ledger = load_json_no_duplicates(LEDGER)
    if (
        ledger.get("schema") != "hepta.architecture-gap-ledger.v1"
        or ledger.get("schemaVersion") != 2
    ):
        fail("unsupported gap-ledger schema")
    if ledger.get("canonicalBranch") != (
        "codex/hepta-architecture-gap-closure-p0-5-20260829"
    ):
        fail("canonical gap-closure branch drifted")
    if ledger.get("parentPullRequest") != 53:
        fail("parent pull request drifted")

    closure = ledger.get("sourceClosure")
    if not isinstance(closure, dict) or not closure:
        fail("sourceClosure must be a non-empty object")
    for gap_id, record in closure.items():
        if not isinstance(record, dict) or record.get("state") != "closed":
            fail(f"source closure gap is not closed: {gap_id}")
        evidence = record.get("evidence")
        if not isinstance(evidence, list) or not evidence:
            fail(f"source closure gap has no evidence: {gap_id}")
        for relative in evidence:
            if not isinstance(relative, str) or not (ROOT / relative).is_file():
                fail(f"source closure evidence is missing: {gap_id}: {relative!r}")

    controlled = ledger.get("repositoryControlledGaps")
    if not isinstance(controlled, dict) or not controlled:
        fail("repositoryControlledGaps must be explicit")
    for gap_id, record in controlled.items():
        if not isinstance(record, dict):
            fail(f"repository-controlled gap is invalid: {gap_id}")
        if record.get("state") not in {"open", "partial", "closed"}:
            fail(f"repository-controlled gap has invalid state: {gap_id}")
        if record.get("state") != "closed" and record.get("blocking") is not True:
            fail(f"unfinished repository-controlled gap is not blocking: {gap_id}")

    hosted = ledger.get("hostedQualification")
    if (
        not isinstance(hosted, dict)
        or hosted.get("sourceMutationAllowed") is not False
        or hosted.get("queuedOrEmptyJobCountsAsQualification") is not False
    ):
        fail("hosted qualification boundary drifted")
    if hosted.get("exactSourceHead") not in {"not_run", "queued", "running", "passed", "failed", "blocked", "superseded"}:
        fail("invalid exact-source-head execution state")
    if hosted.get("mergeCandidate") not in {"not_run", "queued", "running", "passed", "failed", "blocked", "superseded"}:
        fail("invalid merge-candidate execution state")

    external = ledger.get("externalDecisionGates")
    if not isinstance(external, dict):
        fail("external decision gates are missing")
    if external.get("liveRepositoryRuleset") != "blocked_external_configuration":
        fail("live repository ruleset gap is hidden")
    if any(
        external.get(key) != "not_self_issued"
        for key in ("operatorAcceptance", "promotion", "release")
    ):
        fail("operator/promotion/release gates must remain independently issued")

    authority = ledger.get("authorityBoundary")
    if not isinstance(authority, dict) or not authority or any(authority.values()):
        fail("gap ledger widened authority")

    reject_architecture_source_mutators()
    print("PASS_HEPTA_ARCHITECTURE_GAP_LEDGER_V2_HONEST_OPEN_GAPS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
