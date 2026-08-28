#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P1.1a hybrid retrieval."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "module": ROOT / "codex-rs/hepta-memory/src/hybrid_retrieval_v2.rs",
    "tests": ROOT / "codex-rs/hepta-memory/src/hybrid_retrieval_v2/tests.rs",
    "framing": ROOT / "codex-rs/hepta-memory/src/framing.rs",
    "baseline": ROOT / "codex-rs/hepta-memory/src/cognitive_retrieval.rs",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1A_HYBRID_RETRIEVAL_2026-08-28.md",
    "workflow": ROOT
    / ".github/workflows/hepta-intelligence-hybrid-retrieval.yml",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    receipt = {
        "schema": "hepta.intelligence.p1.1a.hybrid-retrieval-source-gate.v1",
        "status": (
            "PASS_P1_1A_HYBRID_RETRIEVAL_SOURCE_ONLY"
            if not failures
            else "FAIL_P1_1A_HYBRID_RETRIEVAL_SOURCE_CONTRACT"
        ),
        "scope": "P1_1A_QUERY_PLANNER_AND_FUSION_SOURCE_ONLY",
        "planner_and_fusion_implemented": not failures,
        "runtime_wired": False,
        "default_retrieval_changed": False,
        "attachment_compiler_changed": False,
        "physical_send_changed": False,
        "local_embedding_executed": False,
        "ann_index_executed": False,
        "semantic_index_provenance_verified": False,
        "grounding_filter_applied": False,
        "truth_filter_applied": False,
        "external_effects": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "callers_ratchet": False,
        "rust_compile_validation": False,
        "rust_test_validation": False,
        "efficacy_validation": False,
        "checks": checks,
        "failures": failures,
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0 if not failures else 1


def main() -> int:
    checks: dict[str, bool] = {
        f"file.{name}": path.is_file() and path.stat().st_size > 0
        for name, path in FILES.items()
    }
    if not all(checks.values()):
        return emit(checks)

    module = FILES["module"].read_text(encoding="utf-8")
    tests = FILES["tests"].read_text(encoding="utf-8")
    framing = FILES["framing"].read_text(encoding="utf-8")
    baseline = FILES["baseline"].read_text(encoding="utf-8")
    plan = FILES["plan"].read_text(encoding="utf-8")
    workflow = FILES["workflow"].read_text(encoding="utf-8")
    try:
        status = json.loads(FILES["status"].read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["status.valid_json"] = False
        return emit(checks)
    checks["status.valid_json"] = True

    checks["module.compiled"] = contains_all(
        framing,
        [
            '#[path = "hybrid_retrieval_v2.rs"]',
            "mod hybrid_retrieval_v2;",
        ],
    )
    checks["module.public_shadow_seam"] = contains_all(
        module,
        [
            "pub fn plan_shadow_hybrid_retrieval_v2",
            "pub async fn shadow_hybrid_retrieve_v2",
            "retrieve_memory_candidates(",
            "revalidate_memory_candidates(",
            "explain_memory_head(",
        ],
    )
    checks["module.query_planner"] = contains_all(
        module,
        [
            "QueryPlannerReceipt",
            "QueryIntent",
            "RequestedTemporalScope",
            "QueryLanguageProfile",
            "planner_terms",
            "query_intent",
            "requested_temporal_scope",
            "language_profile",
            "planner_sha256",
            "deterministic: true",
            "model_called: false",
            "query_persisted: false",
        ],
    )
    checks["module.semantic_batch_contract"] = contains_all(
        module,
        [
            "SemanticCandidateBatchDraft",
            "query_sha256",
            "model_sha256",
            "tokenizer_sha256",
            "index_sha256",
            "index_generation",
            "embedding_dimensions",
            "CosineSimilarityPpm",
            "batch_sha256",
            "semantic candidate batch belongs to another query",
            "semantic candidate batch digest does not match its contents",
            "semantic ranks must be contiguous and ordered from one",
        ],
    )
    checks["module.deterministic_fusion"] = contains_all(
        module,
        [
            "LEXICAL_WEIGHT_PPM",
            "SEMANTIC_WEIGHT_PPM",
            "CHANNEL_DIVERSITY_WEIGHT_PPM",
            "FRESHNESS_WEIGHT_PPM",
            "fused_score_ppm",
            "fusion_contract_sha256",
            "deterministic_fallback_used",
        ],
    )
    checks["module.revalidation_boundary"] = contains_all(
        module,
        [
            "candidate_union_single_snapshot: false",
            "physical_send_revalidation_required: true",
            "RevalidationStatus::Current",
            "RevalidationStatus::Stale",
            "revalidation_binding_sha256",
        ],
    )
    checks["module.semantic_claim_boundary"] = contains_all(
        module,
        [
            "semantic_host_supplied_untrusted: semantic_present",
            "semantic_index_provenance_verified: false",
            "local_embedding_executed: HYBRID_RETRIEVAL_V2_LOCAL_EMBEDDING_EXECUTED",
            "ann_index_executed: HYBRID_RETRIEVAL_V2_ANN_INDEX_EXECUTED",
            "grounding_filter_applied: HYBRID_RETRIEVAL_V2_GROUNDING_FILTER_APPLIED",
            "truth_filter_applied: HYBRID_RETRIEVAL_V2_TRUTH_FILTER_APPLIED",
        ],
    )
    checks["module.authority_false"] = contains_all(
        module,
        [
            "HYBRID_RETRIEVAL_V2_RUNTIME_WIRED: bool = false",
            "HYBRID_RETRIEVAL_V2_DEFAULT_RETRIEVAL_CHANGED: bool = false",
            "HYBRID_RETRIEVAL_V2_ATTACHMENT_COMPILER_CHANGED: bool = false",
            "HYBRID_RETRIEVAL_V2_PHYSICAL_SEND_CHANGED: bool = false",
            "HYBRID_RETRIEVAL_V2_EXTERNAL_EFFECTS: bool = false",
            "HYBRID_RETRIEVAL_V2_PRODUCTION_AUTHORITY: bool = false",
            "HYBRID_RETRIEVAL_V2_OPERATOR_ACCEPTANCE: bool = false",
            "HYBRID_RETRIEVAL_V2_PROMOTION: bool = false",
        ],
    )
    checks["module.no_product_mutation"] = not any(
        marker in module
        for marker in [
            ".append_source(",
            ".remember_with_",
            ".correct_with_",
            "refresh_scope_projection",
            "ProductionDurableWriter",
            "ProductionOutboxDispatcher",
            "ToolContributor",
            "physical_send(",
        ]
    )
    checks["baseline.unchanged_entrypoint"] = contains_all(
        baseline,
        [
            "pub async fn retrieve_memory_candidates",
            "pub async fn revalidate_memory_candidates",
            "RetrievalChannel::MemoryFts",
            "RetrievalChannel::EntityFts",
            "RetrievalChannel::GraphOneHop",
            "RetrievalChannel::Recency",
        ],
    ) and "hybrid_retrieval_v2" not in baseline
    checks["tests.coverage"] = contains_all(
        tests,
        [
            "planner_is_deterministic_and_intent_aware_for_mixed_language",
            "semantic_batch_is_query_bound_and_tamper_evident",
            "fusion_renormalizes_when_semantic_channel_is_absent",
            "shadow_hybrid_union_reorders_with_semantic_evidence_without_product_mutation",
            "SELECT COUNT(*) FROM source_ledger",
            "SELECT COUNT(*) FROM memory_revisions",
            "SELECT COUNT(*) FROM kg_projection",
        ],
    )

    current = status.get("current_tranche", {})
    dependency = status.get("dependency", {})
    claims = current.get("claims", {})
    checks["status.p1_1a_source_only"] = (
        current.get("id") == "P1.1a"
        and current.get("qualified") is False
        and claims.get("query_planner_implemented") is True
        and claims.get("deterministic_fusion_implemented") is True
        and claims.get("runtime_wired") is False
        and claims.get("default_retrieval_changed") is False
        and claims.get("production_authority") is False
    )
    checks["status.p0_4c_unqualified_dependency"] = (
        dependency.get("id") == "P0.4c"
        and dependency.get("qualified") is False
    )
    checks["plan.boundary"] = contains_all(
        plan,
        [
            "SOURCE_ONLY",
            "ACTIVATION_BLOCKED",
            "runtime_wired=false",
            "default_retrieval_changed=false",
            "local_embedding_executed=false",
            "ann_index_executed=false",
            "production_authority=false",
            "P1.1b",
        ],
    )
    checks["workflow.toolchain"] = contains_all(
        workflow,
        [
            'toolchain: "1.95.0"',
            "cargo fmt --all -- --check",
            "hybrid_retrieval_v2",
            "verify-hepta-intelligence-hybrid-retrieval.py",
            "cargo clippy -p codex-hepta-memory --all-targets -- -D warnings",
        ],
    )

    return emit(checks)


if __name__ == "__main__":
    sys.exit(main())
