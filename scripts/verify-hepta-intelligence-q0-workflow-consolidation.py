#!/usr/bin/env python3
"""Verify the single canonical Q0 qualification workflow and retired lanes."""

from __future__ import annotations

import json
from pathlib import Path

RECEIPT = Path(
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_WORKFLOW_CONSOLIDATION_V1.json"
)
CANONICAL = Path(".github/workflows/hepta-intelligence-q0-paired-candidate-v10.yml")
RETIRED = (
    Path(".github/workflows/hepta-intelligence-q0-exact-candidate-v9.yml"),
    Path(".github/workflows/hepta-intelligence-q0-independent-witness.yml"),
    Path(".github/workflows/hepta-intelligence-grounding-ledger.yml"),
    Path(".github/workflows/hepta-intelligence-grounding-gate.yml"),
    Path(".github/workflows/hepta-intelligence-mutation-state-machine.yml"),
    Path(".github/workflows/hepta-intelligence-mutation-journal.yml"),
    Path(".github/workflows/hepta-intelligence-shadow-host.yml"),
)
NEGATIVE = (
    "runtime_wired",
    "production_authority",
    "operator_acceptance",
    "promotion",
    "callers_ratchet",
)


def main() -> int:
    payload = json.loads(RECEIPT.read_text(encoding="utf-8"))
    assert payload["schema"] == "hepta.intelligence.q0.workflow_consolidation.v1"
    assert payload["status"] == "CANONICAL_PAIRED_WORKFLOW"
    assert payload["canonical_workflow"] == CANONICAL.as_posix()
    assert payload["retired_workflows"] == [path.as_posix() for path in RETIRED]
    assert payload["e1_e2_same_run"] is True
    assert payload["e1_e2_distinct_jobs"] is True
    assert payload["e1_e2_distinct_architectures"] is True
    assert all(payload[key] is False for key in NEGATIVE)
    assert CANONICAL.is_file()
    assert all(not path.exists() for path in RETIRED)

    workflow = CANONICAL.read_text(encoding="utf-8")
    for job in ("prove-primary:", "prove-independent:", "pair-evidence:"):
        assert workflow.count(job) == 1, job
    assert "needs:\n      - prove-primary\n      - prove-independent" in workflow
    assert "runs-on: ubuntu-24.04\n" in workflow
    assert "runs-on: ubuntu-24.04-arm\n" in workflow
    assert "PASS_Q0_E1_E2_EVIDENCE_PAIR" not in workflow
    assert "runtime_wired: true" not in workflow
    assert "production_authority: true" not in workflow

    print("PASS_HEPTA_INTELLIGENCE_Q0_WORKFLOW_CONSOLIDATION_V1")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
