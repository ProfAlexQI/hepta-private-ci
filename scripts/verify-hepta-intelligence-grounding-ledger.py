#!/usr/bin/env python3
"""Fail-closed source and SQLite-schema gate for Hepta Intelligence P0.2."""

from __future__ import annotations

import hashlib
import json
import sqlite3
import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "module": ROOT / "codex-rs/hepta-memory/src/fact_grounding/durable.rs",
    "schema_module": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/schema.rs",
    "grounding_module": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/grounding.rs",
    "grounding_prepare_module": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/prepare.rs",
    "grounding_ledger_module": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger.rs",
    "grounding_ledger_insert_module": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/insert.rs",
    "grounding_ledger_verify_module": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs",
    "grounding_ledger_support_module": ROOT
    / "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs",
    "tests_module": ROOT / "codex-rs/hepta-memory/src/fact_grounding/durable/tests.rs",
    "framing": ROOT / "codex-rs/hepta-memory/src/framing.rs",
    "migration": ROOT
    / "codex-rs/hepta-memory/grounding-migrations/0011_fact_grounding.sql",
    "bazel": ROOT / "codex-rs/hepta-memory/BUILD.bazel",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json",
    "tranche": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_2_IMPLEMENTATION_2026-08-28.md",
}

EXPECTED_SCHEMA_ORACLE = (
    "67bbe2776e2bae9ace02e2a258b878159183735075688334cde1ef1f81dba44a"
)
SCHEMA_DOMAIN = b"hepta:cognitive:fact-grounding-required-schema-oracle:v1"
REQUIRED_OBJECTS = {
    "cognitive_fact_grounding_migrations": "table",
    "cognitive_fact_grounding_migrations_no_delete": "trigger",
    "cognitive_fact_grounding_migrations_no_update": "trigger",
    "kg_revision_fact_grounding_receipts": "table",
    "kg_revision_fact_grounding_receipts_binding_guard": "trigger",
    "kg_revision_fact_grounding_receipts_digest_lookup": "index",
    "kg_revision_fact_grounding_receipts_no_delete": "trigger",
    "kg_revision_fact_grounding_receipts_no_update": "trigger",
    "kg_revision_fact_grounding_receipts_source_lookup": "index",
    "kg_revision_fact_grounding_spans": "table",
    "kg_revision_fact_grounding_spans_digest_lookup": "index",
    "kg_revision_fact_grounding_spans_fact_guard": "trigger",
    "kg_revision_fact_grounding_spans_fact_lookup": "index",
    "kg_revision_fact_grounding_spans_no_delete": "trigger",
    "kg_revision_fact_grounding_spans_no_update": "trigger",
    "kg_revision_fact_grounding_spans_ordinal_guard": "trigger",
    "kg_revision_fact_grounding_spans_range_guard": "trigger",
    "kg_revision_fact_grounding_spans_total_guard": "trigger",
}


def frame(hasher: "hashlib._Hash", value: bytes) -> None:
    hasher.update(struct.pack(">Q", len(value)))
    hasher.update(value)


def schema_oracle(sql: str) -> tuple[str, dict[str, str]]:
    connection = sqlite3.connect(":memory:")
    connection.execute("PRAGMA foreign_keys = ON")
    connection.executescript(sql)
    rows = connection.execute(
        """
        SELECT name, type, sql
        FROM sqlite_schema
        WHERE name NOT LIKE 'sqlite_%' AND sql IS NOT NULL
        ORDER BY name
        """
    ).fetchall()
    objects = {name: object_type for name, object_type, _ in rows}
    hasher = hashlib.sha256()
    frame(hasher, SCHEMA_DOMAIN)
    frame(hasher, struct.pack(">Q", len(rows)))
    for name, object_type, definition in rows:
        frame(hasher, name.encode())
        frame(hasher, object_type.encode())
        frame(hasher, definition.encode())
    return hasher.hexdigest(), objects


def ordered(text: str, *markers: str) -> bool:
    position = -1
    for marker in markers:
        next_position = text.find(marker, position + 1)
        if next_position < 0:
            return False
        position = next_position
    return True


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    payload = {
        "schema": "hepta.intelligence.p0_2.source-gate.v1",
        "status": (
            "PASS_P0_2_DURABLE_GROUNDING_SOURCE_ONLY"
            if not failures
            else "FAIL_P0_2_DURABLE_GROUNDING_SOURCE"
        ),
        "scope": "COMPONENT_MIGRATION_0011_AND_QUALIFICATION_API_ONLY",
        "rust_compile_validation": False,
        "rust_test_validation": False,
        "main_sqlx_lineage_promoted": False,
        "default_open_wired": False,
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
    checks = {
        f"file.{name}": path.is_file() and path.stat().st_size > 0
        for name, path in FILES.items()
    }
    if not all(checks.values()):
        return emit(checks)

    module = "\n".join(
        FILES[name].read_text(encoding="utf-8")
        for name in (
            "module",
            "schema_module",
            "grounding_module",
            "grounding_prepare_module",
            "grounding_ledger_module",
            "grounding_ledger_insert_module",
            "grounding_ledger_verify_module",
            "grounding_ledger_support_module",
            "tests_module",
        )
    )
    framing = FILES["framing"].read_text(encoding="utf-8")
    migration = FILES["migration"].read_text(encoding="utf-8")
    bazel = FILES["bazel"].read_text(encoding="utf-8")
    tranche = FILES["tranche"].read_text(encoding="utf-8")
    status = json.loads(FILES["status"].read_text(encoding="utf-8"))

    actual_oracle, actual_objects = schema_oracle(migration)
    checks["migration.executes"] = True
    checks["migration.version_11"] = (
        "version INTEGER PRIMARY KEY CHECK (version = 11)" in migration
    )
    checks["migration.receipt_and_spans"] = all(
        marker in migration
        for marker in (
            "CREATE TABLE kg_revision_fact_grounding_receipts",
            "CREATE TABLE kg_revision_fact_grounding_spans",
            "fact-grounding receipt binding is invalid",
            "fact-grounding span references an unknown fact",
            "fact-grounding span ordinals must be contiguous",
            "fact-grounding spans exceed the receipt count",
        )
    )
    checks["migration.append_only"] = all(
        marker in migration
        for marker in (
            "kg_revision_fact_grounding_receipts_no_update",
            "kg_revision_fact_grounding_receipts_no_delete",
            "kg_revision_fact_grounding_spans_no_update",
            "kg_revision_fact_grounding_spans_no_delete",
        )
    )
    checks["migration.no_legacy_backfill"] = (
        "INSERT INTO kg_revision_fact_grounding_receipts" not in migration
        and "INSERT INTO kg_revision_fact_grounding_spans" not in migration
    )
    checks["migration.schema_objects_exact"] = actual_objects == REQUIRED_OBJECTS
    checks["migration.schema_oracle"] = actual_oracle == EXPECTED_SCHEMA_ORACLE
    checks["module.oracle_bound"] = EXPECTED_SCHEMA_ORACLE in module
    checks["module.component_migration"] = all(
        marker in module
        for marker in (
            "open_with_durable_fact_grounding",
            "ensure_durable_fact_grounding_schema",
            "verify_migration_ledger",
            "verify_schema_oracle",
            "COMPONENT_MIGRATION_VERSION: i64 = 11",
        )
    )
    checks["module.atomic_remember_order"] = ordered(
        module,
        "pub async fn remember_with_durable_grounded_kg",
        'begin_with("BEGIN IMMEDIATE")',
        "append_source_tx",
        "create_memory_revision_tx",
        "insert_revision_facts_tx",
        "grounding::insert_tx",
        "refresh_scope_projection_tx",
        "transaction.commit()",
    )
    correction = module[module.find("pub async fn correct_with_durable_grounded_kg") :]
    checks["module.atomic_correct_order"] = ordered(
        correction,
        'begin_with("BEGIN IMMEDIATE")',
        "append_source_tx",
        "revise_memory_revision_tx",
        "insert_revision_facts_tx",
        "grounding::insert_tx",
        "refresh_scope_projection_tx",
        "transaction.commit()",
    )
    checks["module.reopen_verification"] = all(
        marker in module
        for marker in (
            "verify_durable_fact_grounding_ledger",
            "durable grounding source digest failed recomputation",
            "durable grounding evidence digest failed",
            "durable grounding fact-identity digest failed recomputation",
            "durable fact-grounding receipt digest failed recomputation",
            "splits a UTF-8 character",
        )
    )
    checks["module.explicit_legacy_status"] = all(
        marker in module for marker in ("grounded_v1", "legacy_unreviewed", "zero_fact")
    )
    checks["module.tests"] = all(
        marker in module
        for marker in (
            "durable_grounding_round_trips_and_reopens",
            "legacy_and_zero_fact_statuses_are_explicit",
            "invalid_grounding_rolls_back_without_rows",
            "correction_persists_a_second_grounded_revision",
            "tampered_evidence_digest_is_rejected_on_reopen",
        )
    )
    checks["module.no_projection_gate"] = (
        "production_projection_gate = true" not in module
        and "PRODUCTION_AUTHORITY: bool = true" not in module
    )
    checks["framing.module_bound"] = (
        '#[path = "fact_grounding/durable.rs"]' in framing
        and "mod durable_fact_grounding;" in framing
    )
    checks["bazel.component_data"] = '"grounding-migrations/**"' in bazel
    checks["status.p0_2"] = (
        status.get("current_tranche", {}).get("id") == "P0.2"
        and status.get("current_tranche", {}).get("qualified") is False
    )
    authority = status.get("authority", {})
    checks["status.authority_false"] = bool(authority) and all(
        value is False for value in authority.values()
    )
    checks["status.p0_3_inactive"] = (
        status.get("next_tranche", {}).get("id") == "P0.3"
        and status.get("next_tranche", {}).get("activation") == "blocked"
    )
    checks["tranche.claim_boundary"] = all(
        marker in tranche
        for marker in (
            "component migration 0011",
            "default `CognitiveStore::open` remains unchanged",
            "`production_projection_gate=false`",
            "`qualified=false`",
        )
    )
    return emit(checks)


if __name__ == "__main__":
    sys.exit(main())
