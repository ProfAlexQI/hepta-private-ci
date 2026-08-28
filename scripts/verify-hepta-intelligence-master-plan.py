#!/usr/bin/env python3
"""Fail-closed verifier for the sole Hepta Intelligence master plan."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN_DIR = ROOT / "plans" / "hepta-intelligence"
MASTER = PLAN_DIR / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
CURRENT = PLAN_DIR / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
AGENTS = PLAN_DIR / "AGENTS.md"
HISTORICAL_PLANS = [
    PLAN_DIR / "HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V2_2026-08-28.md",
    PLAN_DIR / "HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V3_2026-08-28.md",
]
PASS = "PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_SOURCE_ONLY"


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_MASTER_PLAN_V4: {message}")


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"cannot parse {path.relative_to(ROOT)}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain a JSON object")
    return value


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_all_false(mapping: dict[str, Any], label: str) -> None:
    for key, value in mapping.items():
        if value is not False:
            fail(f"{label}.{key} must remain false, got {value!r}")


def main() -> int:
    for path in [MASTER, CURRENT, AGENTS, *HISTORICAL_PLANS]:
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    current = load_json(CURRENT)
    require(current.get("schema") == "hepta_intelligence_current_plan_v2", "current schema mismatch")
    canonical = current.get("canonical")
    require(isinstance(canonical, dict), "canonical object missing")
    expected_master = "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md"
    require(canonical.get("human_document") == expected_master, "canonical human document drift")
    require(canonical.get("plan_id") == "HEPTA_INTELLIGENCE_MASTER_PLAN_V4", "plan id drift")
    require(canonical.get("plan_version") == "4.0.0", "plan version drift")
    require(canonical.get("content_sha256") == sha256(MASTER), "master plan SHA-256 mismatch")
    bootstrap = current.get("session_bootstrap")
    require(isinstance(bootstrap, dict), "session_bootstrap missing")
    require(bootstrap.get("only_current_human_plan") == expected_master, "session current plan drift")
    require(bootstrap.get("on_mismatch") == "FAIL_CLOSED", "session mismatch policy must fail closed")
    require(bootstrap.get("historical_documents_are_authority") is False, "historical docs gained authority")
    authority = current.get("authority")
    require(isinstance(authority, dict), "authority object missing")
    require_all_false(authority, "authority")
    claims = current.get("claim_levels")
    require(isinstance(claims, dict), "claim_levels missing")
    require(claims.get("system_learning") == "L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS", "system claim drift")
    require(claims.get("h5") == "N0_METAPHORICAL_TYPED_PROPOSAL", "H5 claim drift")
    require(claims.get("h6") == "I0_DETERMINISTIC_SELECTIVE_POLICY", "H6 claim drift")
    for key in ["self_evolution", "longitudinal_learning_efficacy", "closed_loop_learning", "structural_plasticity", "neuromorphic_mechanism", "biological_mechanism_replication", "local_small_model_used_by_h5", "local_small_model_used_by_h6"]:
        require(claims.get(key) is False, f"claim_levels.{key} must remain false")
    active = current.get("active_phase")
    require(isinstance(active, dict), "active_phase missing")
    require(active.get("id") == "Q0", "Q0 must remain active")
    require(active.get("status") == "ACTIVE_BLOCKING", "Q0 blocking status drift")
    stack = current.get("stack_budget")
    require(isinstance(stack, dict), "stack_budget missing")
    require(stack.get("runtime_source_freeze") is True, "runtime source freeze must remain true")
    required_contracts = set(current.get("new_required_contracts", []))
    for name in ["LearningEpisodeV1", "LearningEventV1", "PlasticityStateV1", "ExplorationPolicyReceiptV1", "EvaluationReceiptV2", "UnlearningComplianceReceiptV1", "TopologyProposalV1"]:
        require(name in required_contracts, f"required contract missing: {name}")
    text = MASTER.read_text(encoding="utf-8")
    required_markers = ["CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED", "唯一有效的人类可读开发计划", "ExplorationPolicyReceiptV1", "PlasticityStateV1", "UnlearningComplianceReceiptV1", "shared frozen local encoder/backbone", "N2_TEMPORAL_RECURRENT_SIGNAL_NETWORK", "Q0 Qualification Debt Closure", "MemoryRetrievalRank", "candidate LCB", "baseline UCB", "next-snapshot", "self_evolution=false", "closed_loop_learning=false", "neuromorphic_mechanism=false"]
    for marker in required_markers:
        require(marker in text, f"master plan marker missing: {marker}")
    for marker in ["self_evolution=true", "closed_loop_learning=true", "structural_plasticity=true", "neuromorphic_mechanism=true", "local_small_model_used_by_h5=true", "local_small_model_used_by_h6=true"]:
        require(marker not in text, f"forbidden positive claim found: {marker}")
    plan_candidates = sorted(PLAN_DIR.glob("HEPTA_INTELLIGENCE_*PLAN*.md"))
    require(MASTER in plan_candidates, "master plan not found in plan candidate set")
    canonical_count = 0
    for path in plan_candidates:
        body = path.read_text(encoding="utf-8")
        if "CANONICAL_CURRENT" in body:
            canonical_count += 1
            require(path == MASTER, f"non-master plan declares CANONICAL_CURRENT: {path.name}")
        elif path in HISTORICAL_PLANS:
            require("HISTORICAL_REDIRECT" in body, f"historical plan lacks redirect marker: {path.name}")
    require(canonical_count == 1, f"expected exactly one canonical plan, found {canonical_count}")
    agents = AGENTS.read_text(encoding="utf-8")
    require("HEPTA_INTELLIGENCE_CURRENT_PLAN.json" in agents, "AGENTS missing machine pointer")
    require("HEPTA_INTELLIGENCE_MASTER_PLAN.md" in agents, "AGENTS missing master pointer")
    require("Q0" in agents and "fail closed" in agents.lower(), "AGENTS missing Q0/fail-closed rules")
    print(PASS)
    return 0


if __name__ == "__main__":
    sys.exit(main())
