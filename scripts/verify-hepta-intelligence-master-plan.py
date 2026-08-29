#!/usr/bin/env python3
"""Fail-closed verifier for the sole Hepta Intelligence master plan."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN_DIR = ROOT / "plans" / "hepta-intelligence"
MASTER = PLAN_DIR / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
SPEC = PLAN_DIR / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
CURRENT = PLAN_DIR / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
AGENTS = PLAN_DIR / "AGENTS.md"
CAPABILITIES = PLAN_DIR / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"
PR_STACK = PLAN_DIR / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json"
INTEGRATION = PLAN_DIR / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
HISTORICAL_PLANS = [
    PLAN_DIR / "HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V2_2026-08-28.md",
    PLAN_DIR / "HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V3_2026-08-28.md",
]
PASS = "PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_2_SOURCE_ONLY"


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_2: {message}")


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


def require_all_false(mapping: Any, label: str) -> None:
    require(isinstance(mapping, dict) and bool(mapping), f"{label} missing")
    for key, value in mapping.items():
        require(value is False, f"{label}.{key} must remain false")


def main() -> int:
    for path in [
        MASTER,
        SPEC,
        CURRENT,
        AGENTS,
        CAPABILITIES,
        PR_STACK,
        INTEGRATION,
        *HISTORICAL_PLANS,
    ]:
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")

    current = load_json(CURRENT)
    integration = load_json(INTEGRATION)
    require(
        current.get("schema") == "hepta_intelligence_current_plan_v2",
        "current schema mismatch",
    )
    require(
        current.get("repository") == "ProfHepta/hepta-private-ci",
        "repository identity drift",
    )

    canonical = current.get("canonical")
    require(isinstance(canonical, dict), "canonical object missing")
    expected_master = "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md"
    require(canonical.get("human_document") == expected_master, "master pointer drift")
    require(
        canonical.get("plan_id") == "HEPTA_INTELLIGENCE_MASTER_PLAN_V4",
        "plan ID drift",
    )
    require(canonical.get("plan_version") == "4.2.0", "plan version drift")
    require(
        canonical.get("content_sha256") == sha256(MASTER),
        "master plan SHA-256 mismatch",
    )

    operational = current.get("operational_execution")
    require(isinstance(operational, dict), "operational execution missing")
    expected_spec = (
        "plans/hepta-intelligence/"
        "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
    )
    require(operational.get("execution_spec") == expected_spec, "spec pointer drift")
    require(
        operational.get("execution_spec_version") == "1.1.0",
        "spec version drift",
    )
    require(
        operational.get("execution_spec_sha256") == sha256(SPEC),
        "spec digest drift",
    )
    require(
        operational.get("implementation_blueprint_location", "").endswith(
            "#5-package-implementation-blueprint"
        ),
        "implementation blueprint pointer missing",
    )
    require(operational.get("no_ci_source_writeback") is True, "CI writeback enabled")
    require(
        operational.get("source_publisher_separate_from_evidence_workflow") is True,
        "publisher/evidence separation disabled",
    )

    bootstrap = current.get("session_bootstrap")
    require(isinstance(bootstrap, dict), "session_bootstrap missing")
    require(
        bootstrap.get("only_current_human_plan") == expected_master,
        "session current plan drift",
    )
    require(
        bootstrap.get("subordinate_execution_spec") == expected_spec,
        "session execution spec drift",
    )
    require(bootstrap.get("on_mismatch") == "FAIL_CLOSED", "mismatch must fail closed")
    require(
        bootstrap.get("historical_documents_are_authority") is False,
        "historical docs gained authority",
    )
    expected_order = [
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
        expected_master,
    ]
    require(bootstrap.get("read_order") == expected_order, "mandatory read order drift")
    require_all_false(current.get("authority"), "authority")

    source_policy = current.get("source_snapshot_policy")
    require(isinstance(source_policy, dict), "source snapshot policy missing")
    require(
        source_policy.get("classification") == "SOURCE_SNAPSHOT_NOT_LIVE_CI",
        "source snapshot classification drift",
    )
    require(
        source_policy.get("live_evidence_may_directly_mutate_source") is False,
        "live evidence may mutate source",
    )
    require(
        source_policy.get("queued_or_incomplete_is_pass") is False,
        "queued/incomplete is pass",
    )

    claims = current.get("claim_levels")
    require(isinstance(claims, dict), "claim_levels missing")
    require(
        claims.get("system_learning")
        == "L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS",
        "system claim drift",
    )
    require(claims.get("h5") == "N0_METAPHORICAL_TYPED_PROPOSAL", "H5 drift")
    require(claims.get("h6") == "I0_DETERMINISTIC_SELECTIVE_POLICY", "H6 drift")
    for key in [
        "self_evolution",
        "longitudinal_learning_efficacy",
        "closed_loop_learning",
        "structural_plasticity",
        "neuromorphic_mechanism",
        "biological_mechanism_replication",
        "local_small_model_used_by_h5",
        "local_small_model_used_by_h6",
    ]:
        require(claims.get(key) is False, f"claim_levels.{key} must remain false")

    q0 = current.get("q0_qualification")
    require(isinstance(q0, dict), "q0_qualification missing")
    require(
        q0.get("status") == "EXECUTABLE_EXACT_CANDIDATE_QUALIFIED",
        "Q0 exact qualification missing",
    )
    require(
        q0.get("paired_receipt_qualified_candidate") is True,
        "paired Q0 qualification missing",
    )
    require(
        q0.get("runtime_capability_qualified") is False,
        "Q0 gained runtime capability",
    )

    active = current.get("active_phase")
    require(isinstance(active, dict), "active_phase missing")
    require(active.get("id") == "A0", "A0 must be active")
    require(active.get("status") == "ACTIVE_BLOCKING", "A0 status drift")
    require(
        active.get("active_task")
        == "A0.3_EXACT_PARENT_DOCUMENT_DEEPENING_AND_EXECUTABLE_EVIDENCE",
        "A0 active task drift",
    )
    require(
        active.get("current_work_unit")
        == "A0.3_REPLACE_BOT_HEAD_AND_OBTAIN_EXACT_HEAD_EXECUTABLE_EVIDENCE",
        "A0 work unit drift",
    )

    stack = current.get("stack_budget")
    require(isinstance(stack, dict), "stack_budget missing")
    require(stack.get("runtime_source_freeze") is True, "runtime freeze disabled")
    require(
        stack.get("expected_parent")
        == "c768bcbeb4c1168088d2499828c24da521a2a73a",
        "A0 expected parent drift",
    )

    required_contracts = set(current.get("new_required_contracts", []))
    for name in [
        "LearningEpisodeV1",
        "LearningEventV1",
        "PlasticityStateV1",
        "ExplorationPolicyReceiptV1",
        "EvaluationReceiptV2",
        "UnlearningComplianceReceiptV1",
        "TopologyProposalV1",
        "PackageHandoffReceiptV1",
        "A0IndependentReviewReceiptV1",
        "RepositoryCheckAttributionReceiptV1",
    ]:
        require(name in required_contracts, f"required contract missing: {name}")

    master_text = MASTER.read_text(encoding="utf-8")
    required_master_markers = [
        "CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED",
        "唯一有效的人类可读开发计划",
        "SOURCE_SNAPSHOT",
        "LIVE_EVIDENCE",
        "ExplorationPolicyReceiptV1",
        "PlasticityStateV1",
        "UnlearningComplianceReceiptV1",
        "shared frozen local encoder/backbone",
        "N2_TEMPORAL_RECURRENT_SIGNAL_NETWORK",
        "Q0 Qualification Debt Closure",
        "MemoryRetrievalRank",
        "candidate LCB",
        "baseline UCB",
        "next-snapshot",
        "self_evolution=false",
        "closed_loop_learning=false",
        "neuromorphic_mechanism=false",
        "A0 Canonical Authority",
        "qualified_candidate=true",
        "full_repository_merge_green=false",
        "transactional outbox",
        "BLOCKED_EXTERNAL_EVIDENCE",
        "PackageHandoffReceiptV1",
        "RepositoryCheckAttributionReceiptV1",
        "codex-rs/hepta-mutation-coordinator",
    ]
    for marker in required_master_markers:
        require(marker in master_text, f"master marker missing: {marker}")
    for marker in [
        "self_evolution=true",
        "closed_loop_learning=true",
        "structural_plasticity=true",
        "neuromorphic_mechanism=true",
        "local_small_model_used_by_h5=true",
        "local_small_model_used_by_h6=true",
    ]:
        require(marker not in master_text, f"forbidden positive claim: {marker}")

    spec_text = SPEC.read_text(encoding="utf-8")
    required_spec_markers = [
        "SUBORDINATE_EXECUTION_SPEC",
        "Implementation Blueprint",
        "Replacement-commit and CI-trigger protocol",
        "A candidate workflow must never modify",
        "Gap-loop state machine",
        "Package Implementation Blueprint",
        "Field-level causal contracts",
        "B0ContractsExtractionReceiptV1",
        "A0IndependentReviewReceiptV1",
        "RepositoryCheckAttributionReceiptV1",
        "ReviewedCorpusEvidencePackageV1",
        "SemanticArtifactEvidencePackageV1",
        "OperatorAcceptancePackageV1",
        "candidate LCB > baseline UCB",
        "system = L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS",
        "runtime_wired = false",
        "production_authority = false",
    ]
    for marker in required_spec_markers:
        require(marker in spec_text, f"spec marker missing: {marker}")

    operational_docs = integration.get("operational_documents")
    require(
        isinstance(operational_docs, list) and len(operational_docs) == 1,
        "integration operational document surface drift",
    )
    require(
        operational_docs[0].get("content_sha256") == sha256(SPEC),
        "integration spec digest drift",
    )
    require(
        integration.get("expected_changed_path_count")
        == len(integration.get("allowed_changed_paths", []))
        == 17,
        "A0 changed path count drift",
    )
    require_all_false(integration.get("authority"), "integration authority")

    plan_candidates = sorted(PLAN_DIR.glob("HEPTA_INTELLIGENCE_*PLAN*.md"))
    require(MASTER in plan_candidates, "master plan absent from plan candidates")
    canonical_count = 0
    for path in plan_candidates:
        body = path.read_text(encoding="utf-8")
        if "CANONICAL_CURRENT" in body:
            canonical_count += 1
            require(path == MASTER, f"non-master declares canonical: {path.name}")
        elif path in HISTORICAL_PLANS:
            require(
                "HISTORICAL_REDIRECT" in body,
                f"historical plan lacks redirect: {path.name}",
            )
    require(canonical_count == 1, f"expected one canonical plan, found {canonical_count}")

    agents = AGENTS.read_text(encoding="utf-8").lower()
    for marker in [
        "hepta_intelligence_current_plan.json",
        "hepta_intelligence_capability_registry_v1.json",
        "hepta_intelligence_pr_stack_registry_v1.json",
        "hepta_intelligence_integration_candidate_v1.json",
        "hepta_intelligence_master_plan.md",
        "hepta_intelligence_controlled_gap_closure_execution_spec_v1.md",
        "source_snapshot",
        "live_evidence",
        "a0",
        "fail closed",
    ]:
        require(marker.lower() in agents, f"AGENTS missing marker: {marker}")

    print(PASS)
    return 0


if __name__ == "__main__":
    sys.exit(main())
