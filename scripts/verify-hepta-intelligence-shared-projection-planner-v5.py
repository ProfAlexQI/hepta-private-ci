#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P0.3.2 shared projection planning."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "planner": ROOT / "codex-rs/hepta-memory/src/cognitive_projection_planner.rs",
    "product": ROOT / "codex-rs/hepta-memory/src/cognitive_kg_store.rs",
    "durable": ROOT / "codex-rs/hepta-memory/src/fact_grounding/durable.rs",
    "schema": ROOT / "codex-rs/hepta-memory/src/fact_grounding/durable/schema.rs",
    "ledger": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger.rs",
    "support": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs",
    "verify": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs",
    "shadow": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/shadow_projection_gate.rs",
    "framing": ROOT / "codex-rs/hepta-memory/src/framing.rs",
}


def has_all(text: str, markers: tuple[str, ...]) -> bool:
    return all(marker in text for marker in markers)


def main() -> int:
    checks: dict[str, bool] = {
        f"file.{name}": path.is_file() and path.stat().st_size > 0
        for name, path in FILES.items()
    }
    if not all(checks.values()):
        return emit(checks)

    text = {name: path.read_text(encoding="utf-8") for name, path in FILES.items()}
    planner = text["planner"]
    product = text["product"]
    durable = text["durable"]
    schema = text["schema"]
    ledger = text["ledger"]
    support = text["support"]
    verify = text["verify"]
    shadow = text["shadow"]
    framing = text["framing"]

    checks["planner.shared_contract"] = has_all(
        planner,
        (
            "pub(crate) enum ProjectionEligibilityPolicy",
            "CurrentActiveVerified",
            "GroundedActiveVerified",
            "pub(crate) struct ProjectionSemanticPlan",
            "pub(crate) fn build(",
            "semantic projection entity identity",
            "semantic projection relation identity",
            "semantic projection planner observed an incomplete immutable fact set",
            "eligibility_sha256",
            "output_sha256",
        ),
    )
    checks["product.uses_shared_current_policy"] = has_all(
        product,
        (
            "ProjectionSemanticPlan::build(",
            "ProjectionEligibilityPolicy::CurrentActiveVerified",
            "shared semantic planner diverged from the product projection input",
            "let nodes = semantic_plan.nodes;",
            "let edges = semantic_plan.edges;",
            "let output_sha256 = semantic_plan.output_sha256;",
        ),
    )
    checks["shadow.uses_shared_grounded_policy"] = has_all(
        shadow,
        (
            "ProjectionSemanticPlan::build(",
            "ProjectionEligibilityPolicy::GroundedActiveVerified",
            'candidate_kind: "shared_semantic_projection_plan_v1"',
            "shared_projection_planner: true",
            "semantic_projection_parity: true",
        ),
    )
    checks["shadow.replans_current_generation"] = has_all(
        shadow,
        (
            "ProjectionEligibilityPolicy::CurrentActiveVerified",
            "let current_plan = ProjectionSemanticPlan::build(",
            "verify_current_projection(&current, &current_plan)?;",
            "current_projection_replanned: true",
            "r.input_heads_sha256",
            "current KG projection generation diverges from the shared semantic plan",
        ),
    )
    checks["shadow.authorized"] = has_all(
        shadow,
        (
            "self.authorize(access, scope)?;",
            "self.authorize(access, expected_scope)?;",
            "m.owner_agent_id = ? AND m.scope_kind = ?",
            "m.workspace_sha256 IS ?",
        ),
    )
    checks["shadow.same_snapshot_ledger"] = has_all(
        shadow,
        (
            "let mut transaction = self.pool.begin().await.map_err(unavailable)?;",
            "self.verify_durable_fact_grounding_ledger_tx(&mut transaction)",
            "ledger_verified_in_snapshot: true",
            "read_current_projection(&mut transaction",
            "read_heads(self, &mut transaction",
            "read_nodes(self, &mut transaction",
            "read_edges(self, &mut transaction",
        ),
    ) and "ledger_verified_before_snapshot" not in shadow
    checks["shadow.no_pool_verifier"] = (
        "self.verify_durable_fact_grounding_ledger().await?;" not in shadow
    )
    checks["shadow.no_migration_or_projection_write"] = not any(
        marker in shadow
        for marker in (
            "ensure_durable_fact_grounding_schema",
            "refresh_scope_projection_tx",
            "INSERT INTO kg_projection",
            "UPDATE kg_projection SET generation = ?",
            "DELETE FROM kg_projection",
        )
    )
    checks["shadow.authority_false"] = has_all(
        shadow,
        (
            "schema_mutation_performed: false",
            "write_performed: false",
            "default_projection_pointer_changed: false",
            "default_recall_query_changed: false",
            "production_projection_gate: false",
            "production_authority: false",
            "external_effects: false",
            "operator_acceptance: false",
            "promotion: false",
        ),
    )
    checks["durable.transaction_entrypoint"] = has_all(
        durable,
        (
            "pub(crate) async fn verify_durable_fact_grounding_ledger_tx",
            "schema::verify_tx(transaction).await?;",
            "grounding::verify_receipts(&mut **transaction",
            "let mut transaction = self.pool.begin().await.map_err(unavailable)?;",
        ),
    )
    checks["schema.transaction_scoped"] = has_all(
        schema,
        (
            "pub(super) async fn verify_tx(",
            "verify_schema_oracle_connection(&mut **transaction).await?;",
            "verify_migration_ledger_connection(",
            "connection: &mut SqliteConnection",
        ),
    )
    checks["ledger.transaction_scoped"] = has_all(
        verify + "\n" + support + "\n" + ledger,
        (
            "connection: &mut SqliteConnection",
            ".fetch_all(&mut *connection)",
            "stored_fact_supports(\n            connection,",
            "pub(super) use support::stored_fact_supports;",
            "pub(super) use support::durable_receipt_digest;",
            "pub(in super::super) use insert::insert_tx;",
            "pub(in super::super) use verify::verify_receipts;",
        ),
    )
    checks["ledger.helper_visibility_bounded"] = has_all(
        support,
        (
            "pub(in super::super) async fn stored_fact_supports",
            "pub(in super::super) fn durable_receipt_digest",
            "pub(in super::super) fn parse_fact_kind",
            "pub(in super::super) fn validate_span_range_corrupt",
            "pub(in super::super) fn to_i64_len",
            "pub(in super::super) fn limit_plus_one",
        ),
    ) and "pub(crate) fn durable_receipt_digest" not in support
    checks["modules.explicit_paths"] = has_all(
        framing + "\n" + durable + "\n" + ledger,
        (
            '#[path = "cognitive_projection_planner.rs"]',
            '#[path = "fact_grounding/durable.rs"]',
            '#[path = "fact_grounding/shadow_projection_gate.rs"]',
            '#[path = "durable/grounding.rs"]',
            '#[path = "durable/schema.rs"]',
            '#[path = "ledger/support.rs"]',
            '#[path = "ledger/verify.rs"]',
        ),
    )
    checks["tests.current_replan_snapshot_and_no_migration"] = has_all(
        shadow,
        (
            'assert_eq!(value["current_projection_replanned"], true);',
            'assert_eq!(value["ledger_verified_in_snapshot"], true);',
            "shadow_compare_rejects_a_current_receipt_that_diverges_from_the_shared_plan",
            "DROP TRIGGER kg_projection_generation_receipts_no_update",
            "UPDATE kg_projection_generation_receipts",
            "current receipt drift must fail closed",
            "shadow_read_does_not_install_missing_grounding_schema",
            "missing grounding schema must fail closed",
            "assert!(!installed);",
        ),
    )

    return emit(checks)


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    receipt = {
        "schema": "hepta.intelligence.p0.3.2.shared-projection-source-gate.v3",
        "status": (
            "PASS_P0_3_2_SHARED_PROJECTION_SOURCE_ONLY"
            if not failures
            else "FAIL_P0_3_2_SHARED_PROJECTION_SOURCE_CONTRACT"
        ),
        "shared_projection_planner": not failures,
        "current_projection_replanned": not failures,
        "ledger_verified_in_snapshot": not failures,
        "wired": False,
        "qualified": False,
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


if __name__ == "__main__":
    sys.exit(main())
