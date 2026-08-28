#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P0.3.3 evidence resolver."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "resolver": ROOT
    / "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4.rs",
    "resolver_receipt": ROOT
    / "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/receipt.rs",
    "resolver_impl": ROOT
    / "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/resolver.rs",
    "resolver_schema": ROOT
    / "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/schema.rs",
    "resolver_support": ROOT
    / "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/support.rs",
    "resolver_tests": ROOT
    / "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/tests.rs",
    "framing": ROOT / "codex-rs/ext/hepta-memory/src/framing.rs",
    "tools": ROOT / "codex-rs/ext/hepta-memory/src/cognitive/tools.rs",
    "workflow": ROOT
    / ".github/workflows/hepta-intelligence-evidence-resolver-v4.yml",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_EXECUTION_STATUS_V3_2.json",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_DEVELOPMENT_PLAN_V3_2_2026-08-28.md",
    "tranche": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_3_3_HOST_EVIDENCE_RESOLVER_2026-08-28.md",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    result = {
        "schema": "hepta.intelligence.p0.3.3.host-evidence-resolver-source-gate.v1",
        "status": (
            "PASS_P0_3_3_HOST_EVIDENCE_RESOLVER_SOURCE_ONLY"
            if not failures
            else "FAIL_P0_3_3_HOST_EVIDENCE_RESOLVER_SOURCE_CONTRACT"
        ),
        "scope": "P0_3_3_HOST_OWNED_EVIDENCE_RESOLUTION_SOURCE_ONLY",
        "model_supplies_offsets": False,
        "model_supplies_digests": False,
        "host_resolves_offsets": True,
        "host_computes_digests": True,
        "tool_v4_registered": False,
        "p0_3_2_dependency_qualified": False,
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

    resolver_root = FILES["resolver"].read_text(encoding="utf-8")
    resolver_receipt = FILES["resolver_receipt"].read_text(encoding="utf-8")
    resolver_impl = FILES["resolver_impl"].read_text(encoding="utf-8")
    resolver_schema = FILES["resolver_schema"].read_text(encoding="utf-8")
    resolver_support = FILES["resolver_support"].read_text(encoding="utf-8")
    resolver_tests = FILES["resolver_tests"].read_text(encoding="utf-8")
    resolver = "\n".join(
        (
            resolver_root,
            resolver_receipt,
            resolver_impl,
            resolver_schema,
            resolver_support,
            resolver_tests,
        )
    )
    framing = FILES["framing"].read_text(encoding="utf-8")
    tools = FILES["tools"].read_text(encoding="utf-8")
    workflow = FILES["workflow"].read_text(encoding="utf-8")
    plan = FILES["plan"].read_text(encoding="utf-8")
    tranche = FILES["tranche"].read_text(encoding="utf-8")
    try:
        status = json.loads(FILES["status"].read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["status.valid_json"] = False
        return emit(checks)
    checks["status.valid_json"] = True

    checks["resolver.compiled_module"] = contains_all(
        framing,
        [
            '#[path = "cognitive/evidence_resolver_v4.rs"]',
            "pub(crate) mod evidence_resolver_v4;",
        ],
    )
    checks["resolver.explicit_child_paths"] = contains_all(
        resolver_root,
        [
            '#[path = "evidence_resolver_v4/receipt.rs"]',
            '#[path = "evidence_resolver_v4/resolver.rs"]',
            '#[path = "evidence_resolver_v4/schema.rs"]',
            '#[path = "evidence_resolver_v4/support.rs"]',
            '#[path = "evidence_resolver_v4/tests.rs"]',
        ],
    )
    checks["workflow.repository_toolchain_and_scoped_fmt"] = (
        contains_all(
            workflow,
            [
                "toolchain: 1.95.0",
                "Check P0.3.3 candidate formatting",
                "rustfmt --edition 2024 --check",
                "ext/hepta-memory/src/cognitive/evidence_resolver_v4.rs",
                "ext/hepta-memory/src/cognitive/evidence_resolver_v4/tests.rs",
                "github.event.pull_request.head.sha || github.sha",
            ],
        )
        and "cargo fmt --all -- --check" not in workflow
        and "toolchain: 1.88.0" not in workflow
    )
    checks["resolver.contract"] = contains_all(
        resolver,
        [
            "GROUNDED_TOOL_V4_SCHEMA_VERSION: u32 = 4",
            "HOST_EVIDENCE_RESOLVER_SCHEMA_VERSION: u32 = 1",
            "pub(crate) fn grounded_tool_v4_schema",
            "pub(crate) fn prepare_grounded_tool_v4",
            "HostEvidenceResolverV1",
            "HostEvidenceResolutionReceiptV1",
            "EvidenceLocatorV4",
            "SourceSegmentDraftV1",
        ],
    )
    checks["resolver.host_owned_resolution"] = contains_all(
        resolver,
        [
            "fn resolve_exact_quote",
            "fn source_segment_id",
            "Sha256Digest::for_bytes",
            "prepare_grounded_tool_v3",
            "MODEL_SUPPLIED_BYTE_OFFSETS: bool = false",
            "MODEL_SUPPLIED_DIGESTS: bool = false",
            "HOST_RESOLVED_BYTE_OFFSETS: bool = true",
            "HOST_RESOLVED_DIGESTS: bool = true",
        ],
    )
    checks["resolver.selector_modes"] = contains_all(
        resolver,
        [
            "EvidenceLocatorV4::ExactQuote",
            "EvidenceLocatorV4::SourceSegment",
            '"quote"',
            '"occurrence"',
            '"segment_id"',
        ],
    )
    checks["resolver.fail_closed_limits"] = contains_all(
        resolver,
        [
            "MAX_SPANS_PER_FACT",
            "MAX_TOTAL_SPANS",
            "MAX_QUOTE_BYTES",
            "MAX_QUOTE_OCCURRENCE",
            "resolves duplicate evidence ranges",
            "resolves overlapping evidence ranges",
            "references an unknown source segment",
            "references an unknown entity key",
            "splits a UTF-8 character",
        ],
    )
    checks["resolver.receipt_binding"] = contains_all(
        resolver,
        [
            "tool_schema_version",
            "tool_input_sha256",
            "segment_catalog_sha256",
            "source_content_sha256",
            "evidence_resolution_receipt_digest",
            "receipt digest mismatch",
        ],
    )
    checks["resolver.no_private_v3_limit_imports"] = (
        "grounding_v3::MAX_" not in resolver
    )
    checks["resolver.authority_false"] = contains_all(
        resolver,
        [
            "GROUNDED_TOOL_V4_REGISTERED: bool = false",
            "GROUNDED_TOOL_V4_PRODUCTION_AUTHORITY: bool = false",
            "GROUNDED_TOOL_V4_EXTERNAL_EFFECTS: bool = false",
            "GROUNDED_TOOL_V4_OPERATOR_ACCEPTANCE: bool = false",
            "GROUNDED_TOOL_V4_PROMOTION: bool = false",
            "GROUNDED_TOOL_V4_CALLERS_RATCHET: bool = false",
        ],
    )
    checks["resolver.tests"] = contains_all(
        resolver,
        [
            "v4_schema_accepts_host_resolvable_selectors_only",
            "exact_quote_resolves_unicode_occurrence_and_host_digest",
            "exact_quote_occurrence_is_fail_closed",
            "host_segment_id_resolves_without_model_offsets_or_digest",
            "segment_ids_are_bound_to_the_exact_source",
            "duplicate_and_overlapping_ranges_are_rejected_per_fact",
            "receipt_is_digest_bound_and_contains_no_source_body",
            "relation_endpoints_must_reference_declared_entities",
            "resolution_receipt_binds_the_complete_tool_input",
            "exact_quote_occurrences_support_deterministic_overlaps",
            "exact_quote_validation_is_bounded_and_fail_closed",
            "source_segment_catalog_is_deterministic_and_rejects_duplicate_ranges",
            "malformed_and_unknown_segment_ids_are_rejected",
            "different_facts_may_share_the_same_resolved_source_span",
            "duplicate_fact_keys_are_rejected_before_lowering",
            "selector_counts_are_bounded_before_resolution",
            "wrapper_prepares_the_same_host_owned_contract",
            "source_and_segment_limits_are_fail_closed",
            "source_segment_ranges_require_utf8_boundaries",
        ],
    )

    checks["schema.no_model_offsets_or_digests"] = (
        bool(resolver_schema)
        and '"start_byte"' not in resolver_schema
        and '"end_byte"' not in resolver_schema
        and '"sha256"' not in resolver_schema
    )
    checks["runtime.not_registered"] = (
        "evidence_resolver_v4" not in tools
        and "GroundedToolV4Input" not in tools
        and "prepare_grounded_tool_v4" not in tools
    )

    current = status.get("current_tranche", {})
    dependency = status.get("dependency", {})
    authority = status.get("authority", {})
    checks["status.p0_3_3_source_only"] = (
        current.get("id") == "P0.3.3"
        and current.get("implemented") is True
        and current.get("wired") is False
        and current.get("qualified") is False
        and current.get("tool_v4_registered") is False
    )
    checks["status.p0_3_2_dependency_blocked"] = (
        dependency.get("id") == "P0.3.2"
        and dependency.get("qualified") is False
    )
    checks["status.authority_false"] = all(
        authority.get(key) is False
        for key in (
            "production_authority",
            "external_effects",
            "operator_acceptance",
            "promotion",
            "callers_ratchet",
        )
    )
    checks["docs.boundary"] = contains_all(
        plan + "\n" + tranche,
        [
            "HOST_OWNED_EVIDENCE_RESOLUTION",
            "tool_v4_registered=false",
            "production_authority=false",
            "P0.3.2",
            "quote",
            "occurrence",
            "segment_id",
        ],
    )

    return emit(checks)


if __name__ == "__main__":
    sys.exit(main())
