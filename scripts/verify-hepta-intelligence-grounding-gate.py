#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P0.3 shadow grounding."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "core_shadow": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/shadow_projection_gate.rs",
    "core_framing": ROOT / "codex-rs/hepta-memory/src/framing.rs",
    "tool_v3": ROOT / "codex-rs/ext/hepta-memory/src/cognitive/grounding_v3.rs",
    "extension_framing": ROOT / "codex-rs/ext/hepta-memory/src/framing.rs",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_3_SOURCE_TRANCHE_2026-08-28.md",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    result = {
        "schema": "hepta.intelligence.p0.3.shadow-grounding-source-gate.v1",
        "status": (
            "PASS_P0_3_SHADOW_GROUNDING_SOURCE_ONLY"
            if not failures
            else "FAIL_P0_3_SHADOW_GROUNDING_SOURCE_CONTRACT"
        ),
        "scope": "P0_3_TOOL_V3_AND_SHADOW_PROJECTION_SOURCE_ONLY",
        "tool_v3_schema_implemented": not failures,
        "tool_v3_registered": False,
        "shadow_compare_implemented": not failures,
        "shadow_compare_executable_validation": False,
        "default_projection_pointer_changed": False,
        "default_recall_query_changed": False,
        "production_projection_gate": False,
        "production_authority": False,
        "external_effects": False,
        "operator_acceptance": False,
        "promotion": False,
        "callers_ratchet": False,
        "checks": checks,
        "failures": failures,
    }
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if not failures else 1


def main() -> int:
    checks: dict[str, bool] = {
        f"file.{name}": path.is_file() and path.stat().st_size > 0
        for name, path in FILES.items()
    }
    if not all(checks.values()):
        return emit(checks)

    core = FILES["core_shadow"].read_text(encoding="utf-8")
    core_framing = FILES["core_framing"].read_text(encoding="utf-8")
    tool = FILES["tool_v3"].read_text(encoding="utf-8")
    extension_framing = FILES["extension_framing"].read_text(encoding="utf-8")
    plan = FILES["plan"].read_text(encoding="utf-8")
    try:
        status = json.loads(FILES["status"].read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["status.valid_json"] = False
        return emit(checks)
    checks["status.valid_json"] = True

    checks["core.registered"] = contains_all(
        core_framing,
        [
            '#[path = "fact_grounding/shadow_projection_gate.rs"]',
            "mod shadow_projection_gate;",
        ],
    )
    checks["core.shadow_compare"] = contains_all(
        core,
        [
            "pub async fn shadow_grounded_projection_compare",
            "grounded_projection_shadow_compare_v1",
            "grounded_candidate_digest",
            "legacy_unreviewed",
            "zero_fact",
        ],
    )
    checks["core.shadow_explain"] = contains_all(
        core,
        [
            "pub async fn shadow_grounding_explain",
            "grounding_explain_shadow_v1",
            "grounding_receipt_sha256",
            "evidence_sha256",
        ],
    )
    checks["core.verifies_p0_2_first"] = contains_all(
        core,
        [
            "self.ensure_durable_fact_grounding_schema().await?;",
            "self.verify_durable_fact_grounding_ledger().await?;",
        ],
    )
    checks["core.no_projection_write"] = not any(
        marker in core
        for marker in [
            "UPDATE kg_projection",
            "INSERT INTO kg_projection",
            "DELETE FROM kg_projection",
            "refresh_scope_projection_tx",
        ]
    )
    checks["core.authority_false"] = contains_all(
        core,
        [
            "write_performed: false",
            "default_projection_pointer_changed: false",
            "default_recall_query_changed: false",
            "production_projection_gate: false",
            "production_authority: false",
            "external_effects: false",
            "operator_acceptance: false",
            "promotion: false",
        ],
    )
    checks["core.tests"] = contains_all(
        core,
        [
            "shadow_compare_excludes_legacy_without_changing_projection",
            "shadow_explain_returns_digests_and_not_source_content",
            "assert_eq!(generation_before, generation_after)",
            "assert!(!receipt.contains(text))",
        ],
    )

    checks["tool.registered_for_compile"] = contains_all(
        extension_framing,
        [
            '#[path = "cognitive/grounding_v3.rs"]',
            "pub(crate) mod grounding_v3;",
        ],
    )
    checks["tool.v3_schema"] = contains_all(
        tool,
        [
            "GROUNDED_TOOL_V3_SCHEMA_VERSION: u32 = 3",
            "pub(crate) fn grounded_tool_v3_schema",
            '"evidence": evidence()',
            '"required": ["key", "entity_type", "label", "evidence"]',
            '"start_byte"',
            '"end_byte"',
            '"sha256"',
        ],
    )
    checks["tool.v3_validator"] = contains_all(
        tool,
        [
            "pub(crate) fn prepare_grounded_tool_v3",
            "source_content.is_char_boundary",
            "Sha256Digest::for_bytes",
            "GroundedKgFactSetDraft",
            "FactEvidenceSpanDraft::new",
        ],
    )
    checks["tool.not_registered"] = contains_all(
        tool,
        [
            "GROUNDED_TOOL_V3_REGISTERED: bool = false",
            "GROUNDED_TOOL_V3_PRODUCTION_AUTHORITY: bool = false",
            "GROUNDED_TOOL_V3_EXTERNAL_EFFECTS: bool = false",
            "GROUNDED_TOOL_V3_OPERATOR_ACCEPTANCE: bool = false",
            "GROUNDED_TOOL_V3_PROMOTION: bool = false",
        ],
    )
    checks["tool.tests"] = contains_all(
        tool,
        [
            "schema_requires_evidence_for_each_fact",
            "valid_v3_input_produces_grounded_fact_set",
            "v3_rejects_missing_or_drifted_evidence",
        ],
    )

    current = status.get("current_tranche", {})
    dependency = status.get("dependency", {})
    checks["status.p0_3_source_only"] = (
        current.get("id") == "P0.3"
        and current.get("qualified") is False
        and current.get("claims", {}).get("tool_v3_registered") is False
        and current.get("claims", {}).get("production_projection_gate") is False
    )
    checks["status.p0_2_unqualified_dependency"] = (
        dependency.get("id") == "P0.2" and dependency.get("qualified") is False
    )
    checks["plan.boundary"] = contains_all(
        plan,
        [
            "SOURCE_ONLY",
            "ACTIVATION_BLOCKED",
            "tool_v3_registered=false",
            "production_projection_gate=false",
            "default projection pointer is unchanged",
        ],
    )

    return emit(checks)


if __name__ == "__main__":
    sys.exit(main())
