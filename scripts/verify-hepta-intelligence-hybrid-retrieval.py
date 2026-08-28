#!/usr/bin/env python3
"""Fail-closed source gate for P1.1a shadow Hybrid Retrieval v2."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "contract": ROOT / "codex-rs/hepta-memory/src/shadow_hybrid_retrieval_v2.rs",
    "integration_test": ROOT
    / "codex-rs/hepta-memory/tests/shadow_hybrid_retrieval_v2_contract.rs",
    "framing": ROOT / "codex-rs/hepta-memory/src/framing.rs",
    "product_retrieval": ROOT / "codex-rs/hepta-memory/src/cognitive_retrieval.rs",
    "federation": ROOT / "codex-rs/hepta-memory/src/cognitive_federation.rs",
    "extension_recall": ROOT / "codex-rs/ext/hepta-memory/src/cognitive/mod.rs",
    "agentd_runtime": ROOT / "codex-rs/hepta-agentd/src/runtime.rs",
    "agentd_app_runtime": ROOT / "codex-rs/hepta-agentd/src/app_runtime.rs",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1A_SHADOW_HYBRID_RETRIEVAL_2026-08-28.md",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json",
    "workflow": ROOT
    / ".github/workflows/hepta-intelligence-hybrid-retrieval.yml",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    payload = {
        "schema": "hepta.intelligence.p1.1a.hybrid-retrieval-source-gate.v1",
        "status": (
            "PASS_P1_1A_SHADOW_HYBRID_RETRIEVAL_SOURCE_ONLY"
            if not failures
            else "FAIL_P1_1A_SHADOW_HYBRID_RETRIEVAL_SOURCE_CONTRACT"
        ),
        "scope": "P1_1A_QUERY_PLANNER_AND_FUSION_CONTRACT_SOURCE_ONLY",
        "query_execution_implemented": False,
        "product_module_registered": False,
        "vector_backend_registered": False,
        "reranker_registered": False,
        "runtime_wired": False,
        "default_recall_changed": False,
        "federation_recall_changed": False,
        "context_attachment": False,
        "physical_send": False,
        "external_effects": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "callers_ratchet": False,
        "rust_compile_validation": False,
        "rust_test_validation": False,
        "checks": checks,
        "failures": failures,
    }
    print(json.dumps(payload, indent=2, sort_keys=True))
    return 0 if not failures else 1


def main() -> int:
    checks: dict[str, bool] = {
        f"file.{name}": path.is_file() and path.stat().st_size > 0
        for name, path in FILES.items()
    }
    if not all(checks.values()):
        return emit(checks)

    contract = FILES["contract"].read_text(encoding="utf-8")
    integration_test = FILES["integration_test"].read_text(encoding="utf-8")
    framing = FILES["framing"].read_text(encoding="utf-8")
    product_retrieval = FILES["product_retrieval"].read_text(encoding="utf-8")
    federation = FILES["federation"].read_text(encoding="utf-8")
    extension_recall = FILES["extension_recall"].read_text(encoding="utf-8")
    agentd_runtime = FILES["agentd_runtime"].read_text(encoding="utf-8")
    agentd_app_runtime = FILES["agentd_app_runtime"].read_text(encoding="utf-8")
    plan = FILES["plan"].read_text(encoding="utf-8")
    workflow = FILES["workflow"].read_text(encoding="utf-8")
    try:
        status = json.loads(FILES["status"].read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["status.valid_json"] = False
        return emit(checks)
    checks["status.valid_json"] = True

    checks["contract.namespace_and_limits"] = contains_all(
        contract,
        [
            'SHADOW_HYBRID_RETRIEVAL_NAMESPACE: &str =',
            '"shadow_hybrid_retrieval_v2_contract_v1"',
            "MAX_QUERY_TERMS",
            "MAX_QUERY_ENTITIES",
            "MAX_CANDIDATES",
            "MAX_RESULTS",
            "MAX_CONTEXT_TOKENS",
            "RECIPROCAL_RANK_K",
        ],
    )
    checks["contract.query_plan"] = contains_all(
        contract,
        [
            "ShadowHybridRetrievalQueryDraft",
            "ShadowHybridRetrievalPlan",
            "plan_shadow_hybrid_retrieval",
            "query_sha256",
            "scope_key_sha256",
            "semantic_query_sha256",
            "required_time_range",
            "enabled_channels",
            "plan_sha256",
        ],
    )
    checks["contract.channels"] = contains_all(
        contract,
        [
            "ExactLexical",
            "LexicalFts",
            "EntityAlias",
            "SemanticVector",
            "KnowledgeGraph",
            "Recency",
            "HybridChannelEvidenceDraft",
            "channel_score_ppm",
        ],
    )
    checks["contract.eligibility"] = contains_all(
        contract,
        [
            "HybridGroundingStatus",
            "LegacyUnreviewed",
            "HybridTruthStatus",
            "Contradicted",
            "HybridLifecycle",
            "Tombstoned",
            "secret_like",
            "requires_grounded_memory",
            "grounding_eligible",
            "has_graph",
        ],
    )
    checks["contract.deterministic_fusion"] = contains_all(
        contract,
        [
            "fuse_shadow_hybrid_candidates",
            "checked_mul",
            "checked_add",
            "checked_div",
            "fused_score_ppm",
            "candidate_set_digest",
            "selected_result_digest",
            "receipt_digest",
            "stable",
        ],
    )
    checks["contract.token_budget"] = contains_all(
        contract,
        [
            "Utf8ByteUpperBound",
            "estimate_tokens_utf8_upper_bound",
            "selected_context_tokens",
            "rejected_budget_count",
            "next_tokens > plan.max_context_tokens",
        ],
    )
    checks["contract.authority_negative"] = contains_all(
        contract,
        [
            "SHADOW_HYBRID_RETRIEVAL_RUNTIME_WIRED: bool = false",
            "SHADOW_HYBRID_RETRIEVAL_DEFAULT_RECALL_CHANGED: bool = false",
            "SHADOW_HYBRID_RETRIEVAL_VECTOR_BACKEND_REGISTERED: bool = false",
            "SHADOW_HYBRID_RETRIEVAL_RERANKER_REGISTERED: bool = false",
            "SHADOW_HYBRID_RETRIEVAL_CONTEXT_ATTACHMENT: bool = false",
            "SHADOW_HYBRID_RETRIEVAL_PHYSICAL_SEND: bool = false",
            "SHADOW_HYBRID_RETRIEVAL_EXTERNAL_EFFECTS: bool = false",
            "SHADOW_HYBRID_RETRIEVAL_PRODUCTION_AUTHORITY: bool = false",
            "vector_backend_registered: false",
            "reranker_registered: false",
            "runtime_wired: false",
            "default_recall_changed: false",
            "context_attachment: false",
            "physical_send: false",
            "production_authority: false",
        ],
    )
    checks["contract.no_runtime_backend"] = not any(
        marker in contract
        for marker in [
            "sqlx::",
            "CognitiveStore",
            "Embedding",
            "onnx",
            "ort::",
            "candle",
            "ToolContributor",
            "PhysicalSend",
            "ProductionDurableWriter",
            "ProductionOutboxDispatcher",
            "refresh_scope_projection",
            "compile_automatic_recall_attachment",
        ]
    )
    checks["contract.focused_tests"] = contains_all(
        contract,
        [
            "planner_is_deterministic_multilingual_and_authority_negative",
            "high_risk_excludes_legacy_and_graph_requires_grounding",
            "contradiction_secret_lifecycle_and_budget_fail_closed",
            "fusion_is_stable_and_does_not_claim_a_reranker_or_send",
            "receipt_tamper_is_detected",
            "byte_upper_bound_is_conservative_for_multibyte_text",
            "duplicate_channel_evidence_is_rejected",
        ],
    )
    checks["integration.isolated_compile"] = contains_all(
        integration_test,
        [
            '#[path = "../src/shadow_hybrid_retrieval_v2.rs"]',
            "mod shadow_hybrid_retrieval_v2;",
            "fn frame_part",
        ],
    )

    product_paths = {
        "framing": framing,
        "product_retrieval": product_retrieval,
        "federation": federation,
        "extension_recall": extension_recall,
        "agentd_runtime": agentd_runtime,
        "agentd_app_runtime": agentd_app_runtime,
    }
    for name, text in product_paths.items():
        checks[f"product_path.{name}_not_wired"] = (
            "shadow_hybrid_retrieval_v2" not in text
            and "plan_shadow_hybrid_retrieval" not in text
            and "fuse_shadow_hybrid_candidates" not in text
        )

    current = status.get("current_tranche", {})
    dependency = status.get("dependency", {})
    claims = current.get("claims", {})
    checks["status.p1_1a_source_only"] = (
        current.get("id") == "P1.1a"
        and current.get("qualified") is False
        and claims.get("product_module_registered") is False
        and claims.get("query_execution_implemented") is False
        and claims.get("vector_backend_registered") is False
        and claims.get("reranker_registered") is False
        and claims.get("runtime_wired") is False
        and claims.get("default_recall_changed") is False
        and claims.get("context_attachment") is False
        and claims.get("physical_send") is False
        and claims.get("production_authority") is False
        and claims.get("external_effects") is False
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
            "default_recall_changed=false",
            "vector_backend_registered=false",
            "reranker_registered=false",
            "context_attachment=false",
            "physical_send=false",
            "production_authority=false",
            "P1.1b",
            "P1.1c",
            "P1.1d",
        ],
    )
    checks["workflow.toolchain_and_commands"] = contains_all(
        workflow,
        [
            'toolchain: "1.95.0"',
            "verify-hepta-intelligence-hybrid-retrieval.py",
            "cargo fmt --all -- --check",
            "--test shadow_hybrid_retrieval_v2_contract",
            "cargo test -p codex-hepta-memory",
            "cargo clippy",
        ],
    )

    return emit(checks)


if __name__ == "__main__":
    sys.exit(main())
