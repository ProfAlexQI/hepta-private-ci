#!/usr/bin/env python3
"""Make the legacy P0.2 source gate monotonic across fail-closed successors."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path.cwd()
VERIFIER = ROOT / "scripts/verify-hepta-intelligence-grounding-ledger.py"
STATUS = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json"
P033_HEAD = "eddcb59ca43a76ac83b64507983bd908f406ff48"
P033_RUN = 33226392404
P033_ARTIFACT = 9707307831

OLD = '''    checks["status.p0_2"] = (
        status.get("current_tranche", {}).get("id") == "P0.2"
        and status.get("current_tranche", {}).get("qualified") is False
    )
    authority = status.get("authority", {})
    checks["status.authority_false"] = bool(authority) and all(
        value is False for value in authority.values()
    )
    checks["status.p0_3_inactive"] = (
        status.get("next_tranche", {}).get("id") == "P0.3"
        and status.get("next_tranche", {}).get("activation") == "blocked"
    )
'''
NEW = '''    capabilities = {
        capability.get("id"): capability
        for capability in status.get("capabilities", [])
        if isinstance(capability, dict)
    }
    durable = capabilities.get("durable_fact_grounding_ledger", {})
    checks["status.p0_2_lineage_retained"] = (
        durable.get("implemented") is True
        and durable.get("wired") is False
        and durable.get("promoted") is False
    )
    authority = status.get("authority", {})
    checks["status.authority_false"] = bool(authority) and all(
        value is False for value in authority.values()
    )
    current = status.get("current_tranche", {})
    current_id = str(current.get("id", ""))
    next_tranche = status.get("next_tranche", {})
    claims = current.get("claims", {})
    original_boundary = (
        current_id == "P0.2"
        and next_tranche.get("id") == "P0.3"
        and next_tranche.get("activation") == "blocked"
    )
    fail_closed_successor = (
        (current_id == "P0.3" or current_id.startswith("P0.3."))
        and bool(authority)
        and all(value is False for value in authority.values())
        and claims.get("production_projection_gate") is not True
        and claims.get("production_authority") is not True
        and claims.get("external_effects") is not True
    )
    checks["status.forward_progress_fail_closed"] = (
        original_boundary or fail_closed_successor
    )
'''


def patch_verifier() -> None:
    text = VERIFIER.read_text(encoding="utf-8")
    if NEW in text:
        return
    if text.count(OLD) != 1:
        raise SystemExit("legacy P0.2 status assertion block drifted")
    VERIFIER.write_text(text.replace(OLD, NEW, 1), encoding="utf-8")


def patch_status() -> None:
    status = json.loads(STATUS.read_text(encoding="utf-8"))
    status["repository"] = "ProfHepta/hepta-private-ci"
    status["repository_renamed_from"] = "ProfAlexQI/hepta-private-ci"
    status["generated_at"] = "2026-08-29"
    status["lineage_revalidation"] = {
        "p0_2_durable_grounding_ledger": {
            "independent_branch_qualified": False,
            "source_contract_monotonic": True,
            "revalidated_by_descendant_exact_head": P033_HEAD,
            "revalidated_by_descendant_run": P033_RUN,
            "revalidated_by_descendant_artifact": P033_ARTIFACT,
            "same_snapshot_ledger_verification": True,
        },
        "p0_3_2_shared_semantic_projection_planner": {
            "qualified": True,
            "exact_head": "fa59bb090043ba8d6fbf0991b167779d2385888c",
            "exact_run": 33190943793,
            "exact_artifact": 9693847531,
        },
        "p0_3_3_host_owned_evidence_resolution": {
            "qualified": True,
            "exact_head": P033_HEAD,
            "exact_run": P033_RUN,
            "exact_artifact": P033_ARTIFACT,
        },
    }
    current = status.get("current_tranche")
    if isinstance(current, dict) and current.get("id") == "P0.3":
        qualification = current.setdefault("qualification", {})
        qualification["p0_3_3_source_qualified"] = True
        qualification["p0_3_3_exact_head"] = P033_HEAD
        qualification["p0_3_3_exact_run"] = P033_RUN
        qualification["p0_3_3_exact_artifact"] = P033_ARTIFACT
    STATUS.write_text(
        json.dumps(status, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def main() -> int:
    patch_verifier()
    patch_status()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
