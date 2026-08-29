#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P0.4 mutation state model."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "state": ROOT / "codex-rs/hepta-memory/src/intelligence_mutation_state.rs",
    "framing": ROOT / "codex-rs/hepta-memory/src/framing.rs",
    "model": ROOT
    / "plans/hepta-intelligence/models/P0_4_INTELLIGENCE_MUTATION_STATE_MACHINE.tla",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_4_SOURCE_TRANCHE_2026-08-28.md",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json",
    "workflow": ROOT
    / ".github/workflows/hepta-intelligence-q0-paired-candidate-v10.yml",
    "prepare": ROOT / "scripts/q0-qualification/00-prepare.sh",
    "source_gates": ROOT / "scripts/q0-qualification/10-source-gates.sh",
    "rust_matrix": ROOT / "scripts/q0-qualification/20-rust-matrix.sh",
    "workflow_consolidation": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_WORKFLOW_CONSOLIDATION_V1.json",
    "workflow_consolidation_verifier": ROOT
    / "scripts/verify-hepta-intelligence-q0-workflow-consolidation.py",
    "workspace_toolchain": ROOT / "codex-rs/rust-toolchain.toml",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    result = {
        "schema": "hepta.intelligence.p0.4.mutation-state-source-gate.v1",
        "status": (
            "PASS_P0_4_MUTATION_STATE_SOURCE_ONLY"
            if not failures
            else "FAIL_P0_4_MUTATION_STATE_SOURCE_CONTRACT"
        ),
        "scope": "P0_4_TYPED_MUTATION_STATE_SOURCE_ONLY",
        "runtime_wired": False,
        "sqlite_persistence": False,
        "sqlite_failpoint_execution": False,
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

    state = FILES["state"].read_text(encoding="utf-8")
    framing = FILES["framing"].read_text(encoding="utf-8")
    model = FILES["model"].read_text(encoding="utf-8")
    plan = FILES["plan"].read_text(encoding="utf-8")
    workflow = FILES["workflow"].read_text(encoding="utf-8")
    prepare = FILES["prepare"].read_text(encoding="utf-8")
    source_gates = FILES["source_gates"].read_text(encoding="utf-8")
    rust_matrix = FILES["rust_matrix"].read_text(encoding="utf-8")
    consolidation_verifier = FILES["workflow_consolidation_verifier"].read_text(
        encoding="utf-8"
    )
    workspace_toolchain = FILES["workspace_toolchain"].read_text(encoding="utf-8")
    try:
        status = json.loads(FILES["status"].read_text(encoding="utf-8"))
        consolidation = json.loads(
            FILES["workflow_consolidation"].read_text(encoding="utf-8")
        )
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["status.valid_json"] = False
        checks["workflow_consolidation.valid_json"] = False
        return emit(checks)
    checks["status.valid_json"] = True
    checks["workflow_consolidation.valid_json"] = True

    checks["module.registered_dormant"] = contains_all(
        framing,
        [
            "#[allow(dead_code)]",
            '#[path = "intelligence_mutation_state.rs"]',
            "mod intelligence_mutation_state;",
        ],
    )
    checks["state.phases"] = contains_all(
        state,
        [
            "Planned",
            "SourceWitnessed",
            "GroundingValidated",
            "DurableIntentAppended",
            "MemoryFactsCommitted",
            "ProjectionPublished",
            "OutboxSettled",
            "Terminal",
            "Indeterminate",
            "ReconciledApplied",
            "ReconciledNotApplied",
            "Quarantined",
        ],
    )
    checks["state.binding"] = contains_all(
        state,
        [
            "operation_id",
            "lease_id",
            "lease_epoch",
            "expected_revision",
            "starting_projection_generation",
            "causal_root_sha256",
            "causal_parent_sha256",
        ],
    )
    checks["state.order_and_settlement"] = contains_all(
        state,
        [
            "OutboxSettled",
            "IntelligenceMutationAction::Terminalize",
            "UnsettledIntent",
            "durable_intent_settled = true",
        ],
    )
    checks["state.replay_and_reorder"] = contains_all(
        state,
        [
            "IntelligenceMutationApplyDisposition::Replay",
            "ReplayConflict",
            "SequenceMismatch",
            "CausalParentMismatch",
        ],
    )
    checks["state.no_double_write_or_publish"] = contains_all(
        state,
        [
            "memory_write_count != 1",
            "projection_publish_count != 1",
            "DoubleWrite",
            "DoubleProjectionPublish",
        ],
    )
    checks["state.generation_fence"] = contains_all(
        state,
        [
            "StaleProjectionGeneration",
            "expected_previous_generation",
            "last_published_generation",
            "checked_add(1)",
        ],
    )
    checks["state.indeterminate_resolution"] = contains_all(
        state,
        [
            "MarkIndeterminate",
            "ReconcileApplied",
            "ReconcileNotApplied",
            "Quarantine",
            "InvalidReconciliation",
        ],
    )
    checks["state.authority_false"] = contains_all(
        state,
        [
            "INTELLIGENCE_MUTATION_STATE_RUNTIME_WIRED: bool = false",
            "INTELLIGENCE_MUTATION_STATE_SQLITE_PERSISTENCE: bool = false",
            "INTELLIGENCE_MUTATION_STATE_EXTERNAL_EFFECTS: bool = false",
            "INTELLIGENCE_MUTATION_STATE_PRODUCTION_AUTHORITY: bool = false",
            "INTELLIGENCE_MUTATION_STATE_OPERATOR_ACCEPTANCE: bool = false",
            "INTELLIGENCE_MUTATION_STATE_PROMOTION: bool = false",
        ],
    ) and not any(
        marker in state
        for marker in [
            "INTELLIGENCE_MUTATION_STATE_RUNTIME_WIRED: bool = true",
            "INTELLIGENCE_MUTATION_STATE_SQLITE_PERSISTENCE: bool = true",
            "INTELLIGENCE_MUTATION_STATE_PRODUCTION_AUTHORITY: bool = true",
        ]
    )
    checks["state.no_sql_or_effect_calls"] = not any(
        marker in state
        for marker in [
            "sqlx::",
            "INSERT INTO",
            "UPDATE ",
            "DELETE FROM",
            "dispatch(",
            "refresh_scope_projection_tx",
        ]
    )
    checks["state.tests"] = contains_all(
        state,
        [
            "normal_path_requires_outbox_settlement_before_terminal",
            "exact_duplicate_replays_without_double_write",
            "changed_replay_reorder_and_parent_drift_fail_closed",
            "binding_and_generation_drift_fail_closed",
            "crash_before_write_reconciles_not_applied_without_stranded_intent",
            "crash_after_write_reconciles_applied_without_second_write",
            "not_applied_cannot_overwrite_an_observed_commit",
            "quarantine_settles_an_indeterminate_intent",
            "identical_paths_have_identical_transition_digests",
        ],
    )
    checks["model.invariants"] = contains_all(
        model,
        [
            "NoDoubleWrite",
            "NoDoublePublish",
            "GenerationBound",
            "TerminalImpliesSettled",
            "ResolvedImpliesSettled",
            "ProjectionImpliesOneWrite",
        ],
    )
    checks["workflow.canonical_paired"] = (
        consolidation.get("schema")
        == "hepta.intelligence.q0.workflow_consolidation.v1"
        and consolidation.get("status") == "CANONICAL_PAIRED_WORKFLOW"
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
                "needs:\n      - prove-primary\n      - prove-independent",
                "runs-on: ubuntu-24.04\n",
                "runs-on: ubuntu-24.04-arm\n",
            ],
        )
        and contains_all(
            consolidation_verifier,
            [
                "CANONICAL_PAIRED_WORKFLOW",
                "assert all(not path.exists() for path in RETIRED)",
            ],
        )
    )
    checks["workflow.toolchain"] = (
        'channel = "1.95.0"' in workspace_toolchain
        and workflow.count("toolchain: 1.95.0") == 2
        and "toolchain: 1.88.0" not in workflow
        and contains_all(
            prepare,
            [
                "expected_toolchain",
                "rustc --version",
                "Q0_EXPECTED_RUST_HOST",
                "rustfmt --edition 2024 --check",
            ],
        )
        and contains_all(
            rust_matrix,
            [
                "cargo test --locked -p codex-hepta-memory intelligence_mutation_state",
                "cargo clippy --locked -p codex-hepta-memory --all-targets --no-deps -- -D warnings",
            ],
        )
        and "verify-hepta-intelligence-mutation-state.py" in source_gates
    )
    checks["workflow.retired_lanes_absent"] = all(
        lane in consolidation.get("retired_workflows", [])
        for lane in [
            ".github/workflows/hepta-intelligence-mutation-state-machine.yml",
            ".github/workflows/hepta-intelligence-grounding-ledger.yml",
            ".github/workflows/hepta-intelligence-grounding-gate.yml",
        ]
    ) and all(
        not (ROOT / lane).exists()
        for lane in consolidation.get("retired_workflows", [])
    )
    checks["plan.boundary"] = contains_all(
        plan,
        [
            "SOURCE_ONLY",
            "RUNTIME_WIRING_BLOCKED",
            "runtime_wired=false",
            "sqlite_persistence=false",
            "production_authority=false",
        ],
    )
    current = status.get("current_tranche", {})
    checks["status.p0_4"] = (
        current.get("id") == "P0.4"
        and current.get("qualified") is False
        and current.get("claims", {}).get("runtime_wired") is False
        and current.get("claims", {}).get("sqlite_persistence") is False
        and current.get("claims", {}).get("production_authority") is False
    )

    return emit(checks)


if __name__ == "__main__":
    sys.exit(main())
