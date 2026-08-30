#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P0.1 fact grounding."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "module": ROOT / "codex-rs/hepta-memory/src/fact_grounding.rs",
    "lib": ROOT / "codex-rs/hepta-memory/src/lib.rs",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V2_2026-08-28.md",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def ordered(text: str, first: str, second: str) -> bool:
    first_index = text.find(first)
    second_index = text.find(second)
    return first_index >= 0 and second_index >= 0 and first_index < second_index


def emit(status: str, checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    payload = {
        "schema": "hepta.intelligence.fact-grounding-source-gate.v1",
        "status": status if not failures else "FAIL_FACT_GROUNDING_SOURCE_CONTRACT",
        "scope": "P0_1_QUALIFICATION_SOURCE_CONTRACT_ONLY",
        "source_contract_validation": not failures,
        "rust_compile_validation": False,
        "rust_test_validation": False,
        "durable_persistence": False,
        "production_projection_gate": False,
        "production_authority": False,
        "external_effects": False,
        "operator_acceptance": False,
        "promotion": False,
        "callers_ratchet": False,
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
        return emit("FAIL_MISSING_INPUT", checks)

    module = FILES["module"].read_text(encoding="utf-8")
    lib = FILES["lib"].read_text(encoding="utf-8")
    plan = FILES["plan"].read_text(encoding="utf-8")
    try:
        status = json.loads(FILES["status"].read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["status.valid_json"] = False
        return emit("FAIL_INVALID_STATUS", checks)

    checks["status.valid_json"] = True
    checks["module.contract_constants"] = contains_all(
        module,
        [
            'FACT_GROUNDING_NAMESPACE: &str = "local_qualification_only"',
            'FACT_GROUNDING_CONTRACT: &str = "source_span_grounding_v1"',
            "MAX_FACT_GROUNDING_SPANS_PER_FACT",
            "MAX_FACT_GROUNDING_SPANS",
        ],
    )
    checks["module.per_fact_evidence"] = contains_all(
        module,
        [
            "FactEvidenceSpanDraft",
            "GroundedKgFactSetDraft",
            "MissingEvidence",
            "UnknownFact",
            "DuplicateEvidence",
            "EvidenceDigestMismatch",
            "UnsupportedFact",
        ],
    )
    checks["module.span_integrity"] = contains_all(
        module,
        [
            "validate_span_range",
            "is_char_boundary",
            "Sha256Digest::for_bytes(&source.content[start..end])",
            "semantic_normalize",
            "support_is_sufficient",
        ],
    )
    checks["module.receipt_binding"] = contains_all(
        module,
        [
            "FactGroundingReceipt",
            "fact_identity_sha256",
            "source_content_sha256",
            "fact_set_sha256",
            "grounding_receipt_digest",
            "grounding receipt is not bound to the cognitive write",
        ],
    )
    checks["module.authority_negative"] = contains_all(
        module,
        [
            "durable_persistence: false",
            "production_authority: false",
            "projection_gate: false",
            "AuthorityBoundary",
        ],
    ) and not any(
        marker in module
        for marker in [
            "durable_persistence: true",
            "production_authority: true",
            "projection_gate: true",
        ]
    )
    checks["module.validation_precedes_remember"] = ordered(
        module,
        "let prepared = prepare_grounding(source, grounded)?;",
        ".remember_with_kg(access, source, draft, &grounded.facts)",
    )
    correct_start = module.find("pub async fn correct_with_grounded_kg")
    correct_body = module[correct_start:] if correct_start >= 0 else ""
    checks["module.validation_precedes_correct"] = ordered(
        correct_body,
        "let prepared = prepare_grounding(source, grounded)?;",
        ".correct_with_kg(",
    )
    checks["module.no_durable_grounding_insert"] = (
        "INSERT INTO kg_revision_fact_grounding" not in module
        and "CREATE TABLE kg_revision_fact_grounding" not in module
    )
    checks["module.negative_tests"] = contains_all(
        module,
        [
            "MissingEvidence",
            "UnknownFact",
            "EvidenceDigestMismatch",
            "InvalidSpan",
            "UnsupportedFact",
            "AuthorityBoundary",
            "invalid_grounding_does_not_mutate_the_store",
            "SELECT COUNT(*) FROM source_ledger",
            "SELECT COUNT(*) FROM memory_revisions",
        ],
    )
    checks["lib.module_registered"] = "mod fact_grounding;" in lib
    checks["lib.exports_registered"] = contains_all(
        lib,
        [
            "pub use fact_grounding::FACT_GROUNDING_CONTRACT;",
            "pub use fact_grounding::FACT_GROUNDING_NAMESPACE;",
            "pub use fact_grounding::FactGroundingReceipt;",
            "pub use fact_grounding::GroundedKgFactSetDraft;",
            "pub use fact_grounding::GroundedCognitiveWriteReceipt;",
        ],
    )
    checks["plan.capability_state_model"] = contains_all(
        plan,
        [
            "`implemented`",
            "`wired`",
            "`qualified`",
            "`efficacy_proven`",
            "`operator_accepted`",
            "`promoted`",
        ],
    )
    checks["plan.correctness_layers"] = contains_all(
        plan, ["`source_witness`", "`fact_grounding`", "`truth_status`"]
    )
    checks["plan.p0_sequence"] = contains_all(plan, ["P0.1", "P0.2", "P0.3", "P0.4"])
    checks["plan.explicit_limits"] = contains_all(
        plan,
        [
            "receipt 当前只在调用返回值中存在",
            "当前 active KG projection 尚未强制要求 persisted grounding",
            "textual support 不等于 external truth",
        ],
    )
    authority = status.get("authority", {})
    checks["status.authority_false"] = bool(authority) and all(
        value is False for value in authority.values()
    )
    claims = status.get("current_tranche", {}).get("claims", {})
    checks["status.claim_boundary"] = (
        claims.get("source_span_validation") is True
        and claims.get("per_fact_evidence_required") is True
        and claims.get("durable_persistence") is False
        and claims.get("production_projection_gate") is False
        and claims.get("production_authority") is False
    )
    checks["status.not_qualified"] = status.get("current_tranche", {}).get(
        "status"
    ) in {
        "implemented_on_branch_pending_ci",
        "source_contract_pass_blocked_runner_not_assigned",
    } and status.get("current_tranche", {}).get("qualification", {}).get(
        "workflow_ci"
    ) in {"pending", "blocked_runner_not_assigned"}

    return emit("PASS_FACT_GROUNDING_SOURCE_CONTRACT_ONLY", checks)


if __name__ == "__main__":
    sys.exit(main())
