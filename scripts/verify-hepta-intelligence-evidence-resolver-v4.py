#!/usr/bin/env python3
"""Fail-closed source gate for P0.3.3 host-owned evidence resolution."""

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
    "runner": ROOT / "scripts/run-hepta-intelligence-evidence-resolver-v5.py",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_EXECUTION_STATUS_V3_2.json",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_DEVELOPMENT_PLAN_V3_2_2026-08-28.md",
    "tranche": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_3_3_HOST_EVIDENCE_RESOLVER_2026-08-28.md",
}


def contains_all(text: str, markers: tuple[str, ...]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool], dependency_qualified: bool) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    receipt = {
        "schema": "hepta.intelligence.p0.3.3.host-evidence-resolver-source-gate.v2",
        "status": (
            "PASS_P0_3_3_HOST_EVIDENCE_RESOLVER_SOURCE_ONLY"
            if not failures
            else "FAIL_P0_3_3_HOST_EVIDENCE_RESOLVER_SOURCE_CONTRACT"
        ),
        "scope": "P0_3_3_HOST_OWNED_EVIDENCE_RESOLUTION_SOURCE_ONLY",
        "qualified": False,
        "model_supplies_offsets": False,
        "model_supplies_digests": False,
        "host_resolves_offsets": True,
        "host_computes_digests": True,
        "tool_v4_registered": False,
        "p0_3_2_dependency_qualified": dependency_qualified,
        "production_projection_gate": False,
        "production_authority": False,
        "external_effects": False,
        "operator_acceptance": False,
        "promotion": False,
        "callers_ratchet": False,
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
        return emit(checks, False)

    try:
        status = json.loads(FILES["status"].read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["status.valid_json"] = False
        return emit(checks, False)
    checks["status.valid_json"] = True

    resolver_parts = tuple(
        FILES[name].read_text(encoding="utf-8")
        for name in (
            "resolver",
            "resolver_receipt",
            "resolver_impl",
            "resolver_schema",
            "resolver_support",
            "resolver_tests",
        )
    )
    resolver = "\n".join(resolver_parts)
    resolver_root = resolver_parts[0]
    resolver_schema = resolver_parts[3]
    resolver_tests = resolver_parts[5]
    framing = FILES["framing"].read_text(encoding="utf-8")
    tools = FILES["tools"].read_text(encoding="utf-8")
    workflow = FILES["workflow"].read_text(encoding="utf-8")
    runner = FILES["runner"].read_text(encoding="utf-8")
    plan = FILES["plan"].read_text(encoding="utf-8")
    tranche = FILES["tranche"].read_text(encoding="utf-8")

    dependency = status.get("dependency")
    dependency = dependency if isinstance(dependency, dict) else {}
    dependency_qualified = dependency.get("qualified") is True
    dependency_consistent = (
        dependency.get("id") == "P0.3.2"
        and dependency.get("activation_blocking") is (not dependency_qualified)
    )
    if dependency_qualified:
        dependency_consistent = dependency_consistent and (
            dependency.get("implemented_in_repository") is True
            and dependency.get("repository_branch")
            == "codex/hepta-intelligence-shared-projection-planner-v5-20260828"
            and dependency.get("ledger_verified_in_snapshot") is True
        )

    checks["resolver.compiled_module"] = contains_all(
        framing,
        (
            '#[path = "cognitive/evidence_resolver_v4.rs"]',
            "pub(crate) mod evidence_resolver_v4;",
        ),
    )
    checks["resolver.explicit_child_paths"] = contains_all(
        resolver_root,
        (
            '#[path = "evidence_resolver_v4/receipt.rs"]',
            '#[path = "evidence_resolver_v4/resolver.rs"]',
            '#[path = "evidence_resolver_v4/schema.rs"]',
            '#[path = "evidence_resolver_v4/support.rs"]',
            '#[path = "evidence_resolver_v4/tests.rs"]',
        ),
    )
    checks["resolver.contract"] = contains_all(
        resolver,
        (
            "GROUNDED_TOOL_V4_SCHEMA_VERSION: u32 = 4",
            "HOST_EVIDENCE_RESOLVER_SCHEMA_VERSION: u32 = 1",
            "HostEvidenceResolverV1",
            "HostEvidenceResolutionReceiptV1",
            "EvidenceLocatorV4",
            "SourceSegmentDraftV1",
            "prepare_grounded_tool_v4",
        ),
    )
    checks["resolver.host_owned_resolution"] = contains_all(
        resolver,
        (
            "fn resolve_exact_quote",
            "fn source_segment_id",
            "Sha256Digest::for_bytes",
            "prepare_grounded_tool_v3",
            "MODEL_SUPPLIED_BYTE_OFFSETS: bool = false",
            "MODEL_SUPPLIED_DIGESTS: bool = false",
            "HOST_RESOLVED_BYTE_OFFSETS: bool = true",
            "HOST_RESOLVED_DIGESTS: bool = true",
        ),
    )
    checks["resolver.selector_modes"] = contains_all(
        resolver,
        (
            "EvidenceLocatorV4::ExactQuote",
            "EvidenceLocatorV4::SourceSegment",
            '"quote"',
            '"occurrence"',
            '"segment_id"',
        ),
    )
    checks["resolver.fail_closed_limits"] = contains_all(
        resolver,
        (
            "MAX_SPANS_PER_FACT",
            "MAX_TOTAL_SPANS",
            "MAX_QUOTE_BYTES",
            "MAX_QUOTE_OCCURRENCE",
            "resolves duplicate evidence ranges",
            "resolves overlapping evidence ranges",
            "references an unknown source segment",
            "references an unknown entity key",
            "splits a UTF-8 character",
        ),
    )
    checks["resolver.receipt_binding"] = contains_all(
        resolver,
        (
            "tool_input_sha256",
            "segment_catalog_sha256",
            "source_content_sha256",
            "evidence_resolution_receipt_digest",
            "receipt digest mismatch",
        ),
    )
    checks["resolver.no_private_v3_limit_imports"] = (
        "grounding_v3::MAX_" not in resolver
    )
    checks["resolver.authority_false"] = contains_all(
        resolver,
        (
            "GROUNDED_TOOL_V4_REGISTERED: bool = false",
            "GROUNDED_TOOL_V4_PRODUCTION_AUTHORITY: bool = false",
            "GROUNDED_TOOL_V4_EXTERNAL_EFFECTS: bool = false",
            "GROUNDED_TOOL_V4_OPERATOR_ACCEPTANCE: bool = false",
            "GROUNDED_TOOL_V4_PROMOTION: bool = false",
            "GROUNDED_TOOL_V4_CALLERS_RATCHET: bool = false",
        ),
    )
    checks["resolver.tests"] = contains_all(
        resolver_tests,
        (
            "v4_schema_accepts_host_resolvable_selectors_only",
            "exact_quote_resolves_unicode_occurrence_and_host_digest",
            "host_segment_id_resolves_without_model_offsets_or_digest",
            "segment_ids_are_bound_to_the_exact_source",
            "duplicate_and_overlapping_ranges_are_rejected_per_fact",
            "receipt_is_digest_bound_and_contains_no_source_body",
            "relation_endpoints_must_reference_declared_entities",
            "resolution_receipt_binds_the_complete_tool_input",
            "source_segment_ranges_require_utf8_boundaries",
        ),
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

    checks["workflow.stable_exact_receipt"] = contains_all(
        workflow + "\n" + runner,
        (
            "toolchain: 1.95.0",
            "scripts/run-hepta-intelligence-evidence-resolver-v5.py",
            "rustfmt --edition 2024 --check P0.3.3 candidate files",
            "cargo test -p codex-hepta-memory-extension evidence_resolver_v4",
            "cargo test -p codex-hepta-memory fact_grounding",
            "cargo clippy -p codex-hepta-memory-extension --all-targets",
            "cargo clippy -p codex-hepta-memory --all-targets",
            "qualification-receipt.json",
            "p0_3_2_dependency_qualified",
        ),
    )
    checks["workflow.no_workspace_wide_fmt"] = (
        "cargo fmt --all -- --check" not in workflow + "\n" + runner
    )

    current = status.get("current_tranche")
    current = current if isinstance(current, dict) else {}
    authority = status.get("authority")
    authority = authority if isinstance(authority, dict) else {}
    checks["status.p0_3_3_fail_closed"] = (
        current.get("id") == "P0.3.3"
        and current.get("implemented") is True
        and current.get("wired") is False
        and current.get("qualified") is False
        and current.get("tool_v4_registered") is False
        and current.get("production_projection_gate") is False
        and current.get("production_authority") is False
        and current.get("external_effects") is False
    )
    checks["status.p0_3_2_dependency_consistent"] = dependency_consistent
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
        (
            "HOST_OWNED_EVIDENCE_RESOLUTION",
            "tool_v4_registered=false",
            "production_authority=false",
            "P0.3.2",
        ),
    )

    return emit(checks, dependency_qualified)


if __name__ == "__main__":
    sys.exit(main())
