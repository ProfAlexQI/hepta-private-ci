#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P0.4b."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "migration": ROOT
    / "codex-rs/hepta-memory/mutation-migrations/0012_intelligence_mutation_journal.sql",
    "journal": ROOT / "codex-rs/hepta-memory/src/intelligence_mutation_journal_v3.rs",
    "digest": ROOT
    / "codex-rs/hepta-memory/src/intelligence_mutation_journal_v3/digest.rs",
    "schema": ROOT
    / "codex-rs/hepta-memory/src/intelligence_mutation_journal_v3/schema.rs",
    "replay": ROOT
    / "codex-rs/hepta-memory/src/intelligence_mutation_journal_v3/replay.rs",
    "tests": ROOT
    / "codex-rs/hepta-memory/src/intelligence_mutation_journal_v3/tests.rs",
    "framing": ROOT / "codex-rs/hepta-memory/src/framing.rs",
    "build": ROOT / "codex-rs/hepta-memory/BUILD.bazel",
    "sqlite_selftest": ROOT
    / "scripts/hepta-intelligence-mutation-journal-sqlite-selftest.py",
    "workflow": ROOT
    / ".github/workflows/hepta-intelligence-q0-paired-candidate-v10.yml",
    "prepare": ROOT / "scripts/q0-qualification/00-prepare.sh",
    "source_gates": ROOT / "scripts/q0-qualification/10-source-gates.sh",
    "rust_matrix": ROOT / "scripts/q0-qualification/20-rust-matrix.sh",
    "workspace_toolchain": ROOT / "codex-rs/rust-toolchain.toml",
    "workflow_consolidation": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_WORKFLOW_CONSOLIDATION_V1.json",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_4B_SQLITE_JOURNAL_2026-08-28.md",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool], sqlite_receipt: dict | None) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    payload = {
        "schema": "hepta.intelligence.p0.4b.mutation-journal-source-gate.v1",
        "status": (
            "PASS_P0_4B_MUTATION_JOURNAL_SOURCE_ONLY"
            if not failures
            else "FAIL_P0_4B_MUTATION_JOURNAL_SOURCE_CONTRACT"
        ),
        "scope": "P0_4B_SQLITE_JOURNAL_AND_FAILPOINTS_SOURCE_ONLY",
        "sqlite_selftest": sqlite_receipt,
        "rust_compile_validation": False,
        "rust_test_validation": False,
        "runtime_wired": False,
        "default_open_wired": False,
        "main_sqlx_lineage_promoted": False,
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
        return emit(checks, None)

    migration = FILES["migration"].read_text(encoding="utf-8")
    journal = FILES["journal"].read_text(encoding="utf-8")
    digest = FILES["digest"].read_text(encoding="utf-8")
    schema = FILES["schema"].read_text(encoding="utf-8")
    replay = FILES["replay"].read_text(encoding="utf-8")
    tests = FILES["tests"].read_text(encoding="utf-8")
    framing = FILES["framing"].read_text(encoding="utf-8")
    build = FILES["build"].read_text(encoding="utf-8")
    workflow = FILES["workflow"].read_text(encoding="utf-8")
    prepare = FILES["prepare"].read_text(encoding="utf-8")
    source_gates = FILES["source_gates"].read_text(encoding="utf-8")
    rust_matrix = FILES["rust_matrix"].read_text(encoding="utf-8")
    workspace_toolchain = FILES["workspace_toolchain"].read_text(encoding="utf-8")
    plan = FILES["plan"].read_text(encoding="utf-8")
    try:
        status = json.loads(FILES["status"].read_text(encoding="utf-8"))
        consolidation = json.loads(
            FILES["workflow_consolidation"].read_text(encoding="utf-8")
        )
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["status.valid_json"] = False
        checks["workflow_consolidation.valid_json"] = False
        return emit(checks, None)
    checks["status.valid_json"] = True
    checks["workflow_consolidation.valid_json"] = True

    checks["migration.tables"] = contains_all(
        migration,
        [
            "CREATE TABLE cognitive_intelligence_mutation_migrations",
            "CREATE TABLE cognitive_intelligence_mutation_operations",
            "CREATE TABLE cognitive_intelligence_mutation_transitions",
        ],
    )
    checks["migration.immutable"] = contains_all(
        migration,
        [
            "operations_no_update",
            "operations_no_delete",
            "transitions_no_update",
            "transitions_no_delete",
            "migration ledger is immutable",
        ],
    )
    checks["migration.causal_and_counter_guards"] = contains_all(
        migration,
        [
            "transitions_chain_guard",
            "genesis transition binding is invalid",
            "transition sequence or causal parent is invalid",
            "transition counters are non-monotonic",
            "projection generation did not advance exactly once",
            "terminal resolution requires settled durable intent",
        ],
    )

    checks["journal.canonical_entry"] = contains_all(
        journal,
        [
            "open_with_intelligence_mutation_journal",
            "ensure_intelligence_mutation_journal_schema",
            "verify_intelligence_mutation_journal",
            "begin_intelligence_mutation_journal",
            "append_intelligence_mutation_transition",
            "replay_intelligence_mutation_operation",
        ],
    )
    checks["journal.atomic_begin_immediate"] = contains_all(
        journal,
        [
            'begin_with("BEGIN IMMEDIATE")',
            "replay::replay_operation_tx",
            "insert_transition_tx",
            "transaction.commit()",
        ],
    )
    checks["journal.explicit_acquire"] = "use sqlx::Acquire;" in journal
    checks["journal.failpoints"] = contains_all(
        journal,
        [
            "BeforeTransitionInsert",
            "AfterTransitionInsertBeforeCommit",
            "AfterCommitBeforeReturn",
            "Indeterminate",
        ],
    )
    checks["journal.authority_false"] = contains_all(
        journal,
        [
            "INTELLIGENCE_MUTATION_JOURNAL_RUNTIME_WIRED: bool = false",
            "INTELLIGENCE_MUTATION_JOURNAL_DEFAULT_OPEN_WIRED: bool = false",
            "INTELLIGENCE_MUTATION_JOURNAL_EXTERNAL_EFFECTS: bool = false",
            "INTELLIGENCE_MUTATION_JOURNAL_PRODUCTION_AUTHORITY: bool = false",
            "INTELLIGENCE_MUTATION_JOURNAL_OPERATOR_ACCEPTANCE: bool = false",
            "INTELLIGENCE_MUTATION_JOURNAL_PROMOTION: bool = false",
        ],
    )
    checks["journal.digests"] = contains_all(
        digest,
        [
            "intelligence-mutation-journal-binding:v1",
            "operation_id",
            "lease_epoch",
            "starting_projection_generation",
            "causal_root_sha256",
        ],
    )
    schema_pool_binding = (
        "use codex_state::SqliteConfig;" in schema
        and "SqliteConfig::open_in_memory_pool()" in schema
    ) or ("SqlitePoolOptions" in schema and "sqlite::memory:" in schema)
    checks["journal.schema_oracle"] = schema_pool_binding and contains_all(
        schema,
        [
            "use sqlx::Acquire;",
            "REQUIRED_MUTATION_JOURNAL_SCHEMA_OBJECTS",
            "schema_digest",
            "schema inventory contains missing or unknown objects",
            "component migration 0012 does not match",
        ],
    )
    checks["journal.exact_replay"] = contains_all(
        replay,
        [
            "replay_operation_pool",
            "replay_operation_tx",
            "IntelligenceMutationState::new",
            "state.apply",
            "verify_transition_row",
            "failed exact replay",
            "different inputs",
        ],
    )
    checks["journal.tests"] = contains_all(
        tests,
        [
            "journal_replays_normal_terminal_path_after_reopen",
            "exact_retry_replays_and_changed_retry_fails_closed",
            "precommit_failpoints_roll_back_without_a_transition",
            "postcommit_ack_loss_is_adopted_by_exact_retry",
            "changed_operation_binding_is_rejected",
            "raw_sequence_gap_and_immutable_rows_are_rejected",
            "schema_drift_is_rejected_by_reopen_verifier",
        ],
    )
    checks["framing.only_canonical_entry_registered"] = (
        contains_all(
            framing,
            [
                '#[path = "intelligence_mutation_state.rs"]',
                '#[path = "intelligence_mutation_journal_v3.rs"]',
                "mod intelligence_mutation_journal;",
            ],
        )
        and "intelligence_mutation_journal_v2.rs" not in framing
    )
    checks["build.component_data"] = "mutation-migrations/**" in build

    checks["workflow.canonical_paired"] = (
        consolidation.get("status") == "CANONICAL_PAIRED_WORKFLOW"
        and consolidation.get("canonical_workflow")
        == ".github/workflows/hepta-intelligence-q0-paired-candidate-v10.yml"
        and consolidation.get("e1_e2_same_run") is True
        and consolidation.get("e1_e2_distinct_jobs") is True
        and consolidation.get("e1_e2_distinct_architectures") is True
        and contains_all(
            workflow,
            [
                "prove-primary:",
                "prove-independent:",
                "pair-evidence:",
                "q0-e1-${{ github.sha }}",
                "q0-e2-${{ github.sha }}",
                "q0-pair-${{ github.sha }}",
            ],
        )
    )
    checks["workflow.repository_toolchain"] = (
        'channel = "1.95.0"' in workspace_toolchain
        and workflow.count("toolchain: 1.95.0") == 2
        and "toolchain: 1.88.0" not in workflow
        and contains_all(
            prepare,
            [
                "expected_toolchain",
                "rust-toolchain.toml",
                "rustc --version",
                "Q0_EXPECTED_RUST_HOST",
            ],
        )
        and contains_all(
            rust_matrix,
            [
                "cargo test --locked -p codex-hepta-memory intelligence_mutation_journal",
                "cargo clippy --locked -p codex-hepta-memory --all-targets --no-deps -- -D warnings",
            ],
        )
    )
    checks["workflow.source_and_sqlite_artifacts"] = contains_all(
        source_gates,
        [
            "verify-hepta-intelligence-mutation-journal.py",
            "hepta-intelligence-mutation-journal-sqlite-selftest.py",
            "mutation-journal-sqlite",
            "mutation-journal",
        ],
    ) and contains_all(
        workflow,
        [
            "scripts/verify-hepta-intelligence-q0-evidence-pair.py",
            "e1-qualification-receipt.json",
            "e2-qualification-receipt.json",
            "q0-evidence-pair-receipt.json",
        ],
    )

    current = status.get("current_tranche", {})
    dependency = status.get("dependency", {})
    claims = current.get("claims", {})
    checks["status.p0_4b"] = (
        current.get("id") == "P0.4b"
        and current.get("qualified") is False
        and claims.get("runtime_wired") is False
        and claims.get("default_open_wired") is False
        and claims.get("production_authority") is False
    )
    checks["status.p0_4a_unqualified_dependency"] = (
        dependency.get("id") == "P0.4a" and dependency.get("qualified") is False
    )
    checks["plan.boundary"] = contains_all(
        plan,
        [
            "STACKED DRAFT",
            "RUNTIME WIRING BLOCKED",
            "runtime_wired=false",
            "default_open_wired=false",
            "production_authority=false",
            "P1.1 must not be activated",
        ],
    )

    sqlite_receipt: dict | None = None
    try:
        completed = subprocess.run(
            [sys.executable, str(FILES["sqlite_selftest"])],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
            timeout=30,
        )
        sqlite_receipt = json.loads(completed.stdout) if completed.stdout else None
        checks["sqlite_selftest.exit_zero"] = completed.returncode == 0
        checks["sqlite_selftest.pass"] = bool(
            sqlite_receipt
            and sqlite_receipt.get("status") == "PASS_P0_4B_MUTATION_JOURNAL_SQLITE"
            and sqlite_receipt.get("schema_object_count") == 14
            and sqlite_receipt.get("ack_loss_adopted") is True
        )
    except (OSError, subprocess.SubprocessError, json.JSONDecodeError):
        checks["sqlite_selftest.exit_zero"] = False
        checks["sqlite_selftest.pass"] = False

    return emit(checks, sqlite_receipt)


if __name__ == "__main__":
    sys.exit(main())
