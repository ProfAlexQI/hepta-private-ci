#!/usr/bin/env python3
"""Fail-closed verifier for Hepta Intelligence canonical master plan V4.3."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
MASTER = PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
SPEC = PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
CURRENT = PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
DOCUMENT = PLAN / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
INTEGRATION = PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
AGENTS = PLAN / "AGENTS.md"
HISTORICAL = [
    PLAN / "HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V2_2026-08-28.md",
    PLAN / "HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V3_2026-08-28.md",
]


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3: {message}")


def require(value: bool, message: str) -> None:
    if not value:
        fail(message)


def load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.name} must contain an object")
    return value


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def all_false(value: Any) -> bool:
    return isinstance(value, dict) and bool(value) and all(item is False for item in value.values())


def main() -> int:
    for path in [MASTER, SPEC, CURRENT, DOCUMENT, INTEGRATION, AGENTS, *HISTORICAL]:
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    current = load(CURRENT)
    document = load(DOCUMENT)
    integration = load(INTEGRATION)
    canonical = current.get("canonical", {})
    require(canonical.get("plan_id") == "HEPTA_INTELLIGENCE_MASTER_PLAN_V4", "plan id")
    require(canonical.get("plan_version") == "4.3.0", "plan version")
    require(canonical.get("human_document") == MASTER.relative_to(ROOT).as_posix(), "master pointer")
    actual_master_digest = sha(MASTER)
    expected_master_digest = canonical.get("content_sha256")
    require(
        expected_master_digest == actual_master_digest,
        f"master digest expected={expected_master_digest} actual={actual_master_digest}",
    )
    operational = current.get("operational_execution", {})
    require(operational.get("execution_spec_version") == "1.1.0", "spec version")
    actual_spec_digest = sha(SPEC)
    expected_spec_digest = operational.get("execution_spec_sha256")
    require(
        expected_spec_digest == actual_spec_digest,
        f"spec digest expected={expected_spec_digest} actual={actual_spec_digest}",
    )
    require(current.get("active_phase", {}).get("id") == "A0", "A0 phase")
    require(
        current.get("active_phase", {}).get("current_work_unit")
        == "A0.3_REPLACE_BOT_HEAD_AND_OBTAIN_EXACT_HEAD_EXECUTABLE_EVIDENCE",
        "work unit",
    )
    require(current.get("stack_budget", {}).get("runtime_source_freeze") is True, "runtime freeze")
    require(all_false(current.get("authority")), "current authority")
    require(
        document.get("current_plan_authority", {}).get("human_plan_content_sha256") == actual_master_digest,
        "document master digest",
    )
    require(all_false(document.get("authority")), "document authority")
    require(integration.get("expected_changed_path_count") == 17, "changed path count")
    require(all_false(integration.get("authority")), "integration authority")

    text = MASTER.read_text(encoding="utf-8")
    markers = [
        "CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED",
        "Version: `4.3.0`",
        "SOURCE_SNAPSHOT",
        "LIVE_EVIDENCE",
        "RepositoryCheckAttributionReceiptV1",
        "IntegrationCandidateManifestV1",
        "B0 九包边界",
        "Field-level Causal Contracts",
        "LearningEpisodeV1",
        "PolicyDecisionReceiptV2",
        "UnlearningComplianceReceiptV1",
        "transactional outbox",
        "candidate LCB > baseline UCB",
        "MemoryRetrievalRank",
        "PackageHandoffReceiptV1",
        "shared frozen local encoder/backbone",
        "N2_TEMPORAL_RECURRENT_SIGNAL_NETWORK",
        "BLOCKED_EXTERNAL_EVIDENCE",
        "self_evolution=false",
        "closed_loop_learning=false",
        "neuromorphic_mechanism=false",
        "runtime_wired = false",
        "production_authority = false",
    ]
    # The last two authority spellings are present in AGENTS/spec, while the
    # master uses compact key=value claims. Accept either representation.
    for marker in markers[:-2]:
        require(marker in text, f"missing marker: {marker}")
    require("runtime" in text and "production authority" in text, "authority boundary text")
    for marker in (
        "self_evolution=true",
        "closed_loop_learning=true",
        "structural_plasticity=true",
        "neuromorphic_mechanism=true",
        "local_small_model_used_by_h5=true",
        "local_small_model_used_by_h6=true",
    ):
        require(marker not in text, f"forbidden positive claim: {marker}")

    candidates = sorted(PLAN.glob("HEPTA_INTELLIGENCE_*PLAN*.md"))
    canonical_count = 0
    for path in candidates:
        body = path.read_text(encoding="utf-8")
        if "CANONICAL_CURRENT" in body:
            canonical_count += 1
            require(path == MASTER, f"non-master canonical plan: {path.name}")
        elif path in HISTORICAL:
            require("HISTORICAL_REDIRECT" in body, f"historical redirect missing: {path.name}")
    require(canonical_count == 1, f"canonical plan count {canonical_count}")

    agents = AGENTS.read_text(encoding="utf-8").lower()
    for marker in (
        "hepta_intelligence_current_plan.json",
        "hepta_intelligence_master_plan.md",
        "hepta_intelligence_controlled_gap_closure_execution_spec_v1.md",
        "source_snapshot",
        "live_evidence",
        "separation of duty",
        "a0",
        "fail closed",
    ):
        require(marker in agents, f"AGENTS marker: {marker}")
    print("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_3_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
