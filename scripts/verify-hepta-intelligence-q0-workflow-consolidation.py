#!/usr/bin/env python3
"""Verify the single canonical Q0 qualification workflow and retired lanes."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RECEIPT_V1 = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_WORKFLOW_CONSOLIDATION_V1.json"
RECEIPT = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_WORKFLOW_CONSOLIDATION_V2.json"
WORKFLOW_ROOT = ROOT / ".github/workflows"
CANONICAL = WORKFLOW_ROOT / "hepta-intelligence-q0-paired-candidate-v10.yml"
RETIRED = (
    WORKFLOW_ROOT / "hepta-intelligence-q0-exact-candidate-v9.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-independent-witness.yml",
    WORKFLOW_ROOT / "hepta-intelligence-grounding-ledger.yml",
    WORKFLOW_ROOT / "hepta-intelligence-grounding-gate.yml",
    WORKFLOW_ROOT / "hepta-intelligence-mutation-state-machine.yml",
    WORKFLOW_ROOT / "hepta-intelligence-mutation-journal.yml",
    WORKFLOW_ROOT / "hepta-intelligence-shadow-host.yml",
    WORKFLOW_ROOT / "hepta-ci-superseded-run-cleanup.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-clippy-fix-v5.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-clippy-fix-v6.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-clippy-fix-v7.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-clippy-fix-v8.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-compile-fix-apply.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-compile-fix-v3.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-compile-fix-v4.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-one-shot.yml",
    WORKFLOW_ROOT / "hepta-intelligence-q0-postfmt.yml",
)
NEGATIVE = (
    "runtime_wired",
    "learning_write_authority",
    "production_authority",
    "external_effects",
    "operator_acceptance",
    "promotion",
    "callers_ratchet",
)


def relative(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def main() -> int:
    assert RECEIPT_V1.is_file(), "append-only v1 receipt missing"
    payload = json.loads(RECEIPT.read_text(encoding="utf-8"))
    assert payload["schema"] == "hepta.intelligence.q0.workflow_consolidation.v2"
    assert payload["status"] == "CANONICAL_PAIRED_WORKFLOW_NO_COMPETING_WRITERS"
    assert payload["supersedes"] == relative(RECEIPT_V1)
    assert payload["canonical_workflow"] == relative(CANONICAL)
    assert payload["retired_workflows"] == [relative(path) for path in RETIRED]
    assert payload["e1_e2_same_run"] is True
    assert payload["e1_e2_distinct_jobs"] is True
    assert payload["e1_e2_distinct_architectures"] is True
    assert payload["single_canonical_q0_workflow"] is True
    assert payload["competing_q0_contents_write_workflows"] == 0
    assert payload["canonical_workflow_contents_write"] is False
    assert payload["retired_workflows_absent"] is True
    assert payload["trigger_scope"] == "ALL_PUSHES_ON_Q0_BRANCH"
    assert payload["path_filtering"] is False
    assert all(payload[key] is False for key in NEGATIVE)
    assert CANONICAL.is_file()
    assert all(not path.exists() for path in RETIRED)

    q0_workflows = sorted(WORKFLOW_ROOT.glob("*q0*.yml"))
    assert q0_workflows == [CANONICAL], [relative(path) for path in q0_workflows]

    competing_writers: list[str] = []
    branch = "codex/hepta-intelligence-plan-v3-20260828"
    for path in sorted(WORKFLOW_ROOT.glob("*.yml")):
        text = path.read_text(encoding="utf-8")
        if branch in text and "contents: write" in text:
            competing_writers.append(relative(path))
    assert not competing_writers, competing_writers

    workflow = CANONICAL.read_text(encoding="utf-8")
    for job in ("prove-primary:", "prove-independent:", "pair-evidence:"):
        assert workflow.count(job) == 1, job
    assert "needs:\n      - prove-primary\n      - prove-independent" in workflow
    assert "runs-on: ubuntu-24.04\n" in workflow
    assert "runs-on: ubuntu-24.04-arm\n" in workflow
    assert "permissions:\n  contents: read\n" in workflow
    assert "contents: write" not in workflow
    assert "\n    paths:\n" not in workflow
    assert "PASS_Q0_E1_E2_EVIDENCE_PAIR" not in workflow
    assert "runtime_wired: true" not in workflow
    assert "production_authority: true" not in workflow

    print("PASS_HEPTA_INTELLIGENCE_Q0_WORKFLOW_CONSOLIDATION_V2")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
