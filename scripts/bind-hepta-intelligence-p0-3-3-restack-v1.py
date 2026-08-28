#!/usr/bin/env python3
"""Bind a verified P0.3.2 receipt into the dormant P0.3.3 status machine."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STATUS = (
    ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_EXECUTION_STATUS_V3_2.json"
)
P032_BRANCH = "codex/hepta-intelligence-shared-projection-planner-v5-20260828"


def require_sha(value: str, label: str) -> str:
    if len(value) != 40 or any(character not in "0123456789abcdef" for character in value):
        raise SystemExit(f"{label} is not a lowercase SHA-1")
    return value


def require_run_id(value: str) -> int:
    try:
        run_id = int(value)
    except ValueError as error:
        raise SystemExit("qualification run ID is not an integer") from error
    if run_id <= 0:
        raise SystemExit("qualification run ID must be positive")
    return run_id


def main() -> int:
    if len(sys.argv) != 3:
        raise SystemExit(
            "usage: bind-hepta-intelligence-p0-3-3-restack-v1.py "
            "<qualified-p0.3.2-head> <qualification-run-id>"
        )
    qualified_head = require_sha(sys.argv[1], "qualified P0.3.2 head")
    qualification_run_id = require_run_id(sys.argv[2])

    status = json.loads(STATUS.read_text(encoding="utf-8"))
    status["stack_base"] = {
        "branch": P032_BRANCH,
        "head": qualified_head,
        "pull_request": 40,
        "qualification_run_id": qualification_run_id,
    }
    status["dependency"] = {
        "id": "P0.3.2",
        "name": "shared_semantic_projection_planner",
        "repository_branch": P032_BRANCH,
        "repository_head": qualified_head,
        "qualification_run_id": qualification_run_id,
        "implemented_in_repository": True,
        "qualified": True,
        "activation_blocking": False,
        "shared_projection_planner": True,
        "current_projection_replanned": True,
        "ledger_verified_in_snapshot": True,
    }

    current = status.setdefault("current_tranche", {})
    current.update(
        {
            "id": "P0.3.3",
            "name": "host_owned_evidence_resolution",
            "status": "restacked_on_qualified_p0_3_2_qualification_pending",
            "implemented": True,
            "wired": False,
            "qualified": False,
            "efficacy_proven": False,
            "operator_accepted": False,
            "promoted": False,
            "tool_v4_registered": False,
            "model_supplies_offsets": False,
            "model_supplies_digests": False,
            "host_resolves_offsets": True,
            "host_computes_digests": True,
            "production_projection_gate": False,
            "production_authority": False,
            "external_effects": False,
        }
    )
    current["qualification"] = {
        "source_gate": "not_run_on_restacked_head",
        "dependency_receipt": "verified_before_restack",
        "format": "not_run",
        "focused_tests": "not_run",
        "core_compatibility_tests": "not_run",
        "extension_full_tests": "not_run",
        "core_full_tests": "not_run",
        "extension_clippy": "not_run",
        "core_clippy": "not_run",
        "qualified": False,
    }

    authority = status.setdefault("authority", {})
    for key in (
        "production_authority",
        "external_effects",
        "operator_acceptance",
        "promotion",
        "callers_ratchet",
    ):
        authority[key] = False

    status["next_actions"] = [
        "Run the P0.3.3 v5 source and exact-head qualification jobs on the restacked head.",
        "Fix only executable format, compile, test, or Clippy failures without widening authority.",
        "Keep tool v4 unregistered and all production pointers unchanged until independent qualification is fully green.",
        "Start P0.3.4 legacy inventory/backfill/quarantine only after P0.3.3 qualification evidence exists.",
    ]
    STATUS.write_text(
        json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(
        json.dumps(
            {
                "qualified_p0_3_2_head": qualified_head,
                "qualification_run_id": qualification_run_id,
                "p0_3_3_qualified": False,
                "wired": False,
                "tool_v4_registered": False,
                "production_authority": False,
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
