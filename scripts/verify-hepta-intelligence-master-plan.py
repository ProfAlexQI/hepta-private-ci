#!/usr/bin/env python3
"""Fail-closed verifier for Hepta Intelligence canonical master plan V4.4."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
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
CHECK_CLASSES = [
    "PASS", "INTRODUCED_BY_CANDIDATE", "PRE_EXISTING_ON_BASE",
    "MERGE_INTERACTION", "RUNNER_OR_PLATFORM_INFRA",
    "CANCELLED_OR_SUPERSEDED", "NOT_REQUIRED_BY_SELECTED_POLICY",
    "UNKNOWN_FAIL_CLOSED",
]


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_4: {message}")


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
    require(canonical.get("plan_version") == "4.4.0", "plan version")
    require(canonical.get("human_document") == MASTER.relative_to(ROOT).as_posix(), "master pointer")
    actual_master = sha(MASTER)
    require(canonical.get("content_sha256") == actual_master, "master digest")
    operational = current.get("operational_execution", {})
    require(operational.get("execution_spec_version") == "1.2.0", "spec version")
    actual_spec = sha(SPEC)
    require(operational.get("execution_spec_sha256") == actual_spec, "spec digest")
    require(current.get("active_phase", {}).get("id") == "A0", "A0 phase")
    require(current.get("stack_budget", {}).get("runtime_source_freeze") is True, "runtime freeze")
    require(current.get("generated_at_utc") is None, "source must not embed guessed wall-clock time")
    require(current.get("source_snapshot_policy", {}).get("generated_at_policy") == "COMMIT_BOUND_AT_EXECUTABLE_RECEIPT", "source timestamp policy")
    require(all_false(current.get("authority")), "current authority")
    authority = document.get("current_plan_authority", {})
    require(authority.get("human_plan_version") == "4.4.0", "document master version")
    require(authority.get("human_plan_content_sha256") == actual_master, "document master digest")
    operational_docs = document.get("registered_operational_documents", [])
    require(len(operational_docs) == 1, "operational document count")
    require(operational_docs[0].get("version") == "1.2.0", "document spec version")
    require(operational_docs[0].get("parent_plan_version") == "4.4.0", "spec parent version")
    require(operational_docs[0].get("content_sha256") == actual_spec, "document spec digest")
    require(all_false(document.get("authority")), "document authority")
    require(integration.get("expected_changed_path_count") == 17, "changed path count")
    require(all_false(integration.get("authority")), "integration authority")

    text = MASTER.read_text(encoding="utf-8")
    markers = [
        "CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED", "Version: `4.4.0`",
        "SOURCE_SNAPSHOT", "LIVE_EVIDENCE", "RepositoryCheckAttributionReceiptV1",
        "IntegrationCandidateManifestV1", "B0 九包边界", "Field-level Causal Contracts",
        "LearningEpisodeV1", "PolicyDecisionReceiptV2", "UnlearningComplianceReceiptV1",
        "transactional outbox", "candidate LCB > baseline UCB", "MemoryRetrievalRank",
        "PackageHandoffReceiptV1", "shared frozen local encoder/backbone",
        "N2_TEMPORAL_RECURRENT_SIGNAL_NETWORK", "BLOCKED_EXTERNAL_EVIDENCE",
        "MemoryAssetManifestV1", "DerivedArtifactV1", "EmbeddingSpaceManifestV1",
        "CrossModalQueryV1", "DeletionPropagationReceiptV1", "MM0", "MM1", "MM2",
        "MM3", "MM4", "MM5", "MM6", "source_truth=false",
        "native_media_memory_wired=false", "cross_modal_retrieval_qualified=false",
        "multimodal_efficacy_proven=false",
    ]
    for marker in markers:
        require(marker in text, f"missing marker: {marker}")
    for marker in (
        "self_evolution=true", "closed_loop_learning=true", "structural_plasticity=true",
        "neuromorphic_mechanism=true", "native_media_memory_wired=true",
        "cross_modal_retrieval_qualified=true", "multimodal_efficacy_proven=true",
    ):
        require(marker not in text, f"forbidden positive claim: {marker}")

    spec_text = SPEC.read_text(encoding="utf-8")
    require("Version: `1.2.0`" in spec_text, "spec header version")
    require("Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.4.0`" in spec_text, "spec parent header")
    match = re.search(r"RepositoryCheckAttributionReceiptV1\.classification` is exactly one of:\n\n```text\n(.*?)\n```", spec_text, re.S)
    require(match is not None, "check classification block")
    require(match.group(1).splitlines() == CHECK_CLASSES, "check classification vocabulary")
    for marker in ("MM0 —", "MM1 —", "MM2 —", "MM3 —", "MM4 —", "MM5 —", "MM6 —"):
        require(marker in spec_text, f"missing spec package: {marker}")

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
        "hepta_intelligence_current_plan.json", "hepta_intelligence_master_plan.md",
        "hepta_intelligence_controlled_gap_closure_execution_spec_v1.md",
        "source_snapshot", "live_evidence", "separation of duty", "a0", "fail closed",
        "multimodal_memory_gap_ledger", "source_truth=false",
    ):
        require(marker in agents, f"AGENTS marker: {marker}")
    print("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_4_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
