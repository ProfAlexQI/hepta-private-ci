#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P0.4c shadow host adapter."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = {
    "core_host": ROOT
    / "codex-rs/hepta-memory/src/intelligence_mutation_shadow_host.rs",
    "core_framing": ROOT / "codex-rs/hepta-memory/src/framing.rs",
    "agentd_host": ROOT
    / "codex-rs/hepta-agentd/src/shadow_intelligence_mutation_host.rs",
    "agentd_lib": ROOT / "codex-rs/hepta-agentd/src/lib.rs",
    "agentd_cargo": ROOT / "codex-rs/hepta-agentd/Cargo.toml",
    "agentd_runtime": ROOT / "codex-rs/hepta-agentd/src/runtime.rs",
    "agentd_app_runtime": ROOT / "codex-rs/hepta-agentd/src/app_runtime.rs",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_4C_SHADOW_HOST_2026-08-28.md",
    "workflow": ROOT
    / ".github/workflows/hepta-intelligence-q0-paired-candidate-v10.yml",
    "prepare": ROOT / "scripts/q0-qualification/00-prepare.sh",
    "source_gates": ROOT / "scripts/q0-qualification/10-source-gates.sh",
    "rust_matrix": ROOT / "scripts/q0-qualification/20-rust-matrix.sh",
    "workspace_toolchain": ROOT / "codex-rs/rust-toolchain.toml",
    "workflow_consolidation": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_WORKFLOW_CONSOLIDATION_V1.json",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    result = {
        "schema": "hepta.intelligence.p0.4c.shadow-host-source-gate.v1",
        "status": (
            "PASS_P0_4C_SHADOW_HOST_SOURCE_ONLY"
            if not failures
            else "FAIL_P0_4C_SHADOW_HOST_SOURCE_CONTRACT"
        ),
        "scope": "P0_4C_SHADOW_HOST_ADAPTER_SOURCE_ONLY",
        "adapter_implemented": not failures,
        "runtime_wired": False,
        "default_open_wired": False,
        "app_runtime_attached": False,
        "tool_registered": False,
        "memory_write_authority": False,
        "projection_write_authority": False,
        "outbox_dispatch_authority": False,
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
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if not failures else 1


def main() -> int:
    checks: dict[str, bool] = {
        f"file.{name}": path.is_file() and path.stat().st_size > 0
        for name, path in FILES.items()
    }
    if not all(checks.values()):
        return emit(checks)

    core = FILES["core_host"].read_text(encoding="utf-8")
    framing = FILES["core_framing"].read_text(encoding="utf-8")
    agentd = FILES["agentd_host"].read_text(encoding="utf-8")
    agentd_lib = FILES["agentd_lib"].read_text(encoding="utf-8")
    agentd_cargo = FILES["agentd_cargo"].read_text(encoding="utf-8")
    runtime = FILES["agentd_runtime"].read_text(encoding="utf-8")
    app_runtime = FILES["agentd_app_runtime"].read_text(encoding="utf-8")
    plan = FILES["plan"].read_text(encoding="utf-8")
    workflow = FILES["workflow"].read_text(encoding="utf-8")
    prepare = FILES["prepare"].read_text(encoding="utf-8")
    source_gates = FILES["source_gates"].read_text(encoding="utf-8")
    rust_matrix = FILES["rust_matrix"].read_text(encoding="utf-8")
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

    checks["core.compiled"] = contains_all(
        framing,
        [
            '#[path = "intelligence_mutation_shadow_host.rs"]',
            "mod intelligence_mutation_shadow_host;",
        ],
    )
    checks["core.public_seam"] = contains_all(
        core,
        [
            "pub async fn open_with_shadow_intelligence_mutation_host",
            "pub async fn begin_shadow_intelligence_mutation",
            "pub async fn prepare_shadow_intelligence_mutation_observation",
            "pub async fn append_shadow_intelligence_mutation_observation",
            "pub async fn observe_shadow_intelligence_mutation",
            "pub async fn inspect_shadow_intelligence_mutation",
        ],
    )
    checks["core.typed_journal_delegation"] = contains_all(
        core,
        [
            "begin_intelligence_mutation_journal",
            "replay_intelligence_mutation_operation",
            "append_intelligence_mutation_transition",
            "IntelligenceMutationTransitionRequest",
            "prepared_sha256",
        ],
    )
    checks["core.exact_retry"] = contains_all(
        core,
        [
            "prepared_request_supports_exact_retry_and_rejects_tamper",
            "postcommit_ack_loss_is_adopted_by_exact_retry",
            'replay["journal_disposition"]',
            '"replay"',
        ],
    )
    checks["core.no_product_mutation"] = not any(
        marker in core
        for marker in [
            ".append_source(",
            ".remember_with_",
            ".correct_with_",
            "refresh_scope_projection",
            "ProductionDurableWriter",
            "ProductionOutboxDispatcher",
            "ToolContributor",
            "physical_send",
        ]
    )
    checks["core.zero_product_rows_test"] = contains_all(
        core,
        [
            "SELECT COUNT(*) FROM source_ledger",
            "SELECT COUNT(*) FROM memory_revisions",
            "SELECT COUNT(*) FROM kg_projection",
            "assert_eq!(source_count, 0)",
            "assert_eq!(memory_count, 0)",
            "assert_eq!(projection_count, 0)",
        ],
    )
    checks["core.authority_false"] = contains_all(
        core,
        [
            "SHADOW_INTELLIGENCE_MUTATION_HOST_RUNTIME_WIRED: bool = false",
            "SHADOW_INTELLIGENCE_MUTATION_HOST_DEFAULT_OPEN_WIRED: bool = false",
            "SHADOW_INTELLIGENCE_MUTATION_HOST_MEMORY_WRITE_AUTHORITY: bool = false",
            "SHADOW_INTELLIGENCE_MUTATION_HOST_PROJECTION_WRITE_AUTHORITY: bool = false",
            "SHADOW_INTELLIGENCE_MUTATION_HOST_OUTBOX_DISPATCH_AUTHORITY: bool = false",
            "SHADOW_INTELLIGENCE_MUTATION_HOST_EXTERNAL_EFFECTS: bool = false",
            "SHADOW_INTELLIGENCE_MUTATION_HOST_PRODUCTION_AUTHORITY: bool = false",
            "memory_write_performed_by_adapter: false",
            "projection_write_performed_by_adapter: false",
            "outbox_dispatch_performed_by_adapter: false",
        ],
    )

    checks["agentd.feature_default_off"] = contains_all(
        agentd_cargo,
        [
            "default = []",
            "qualification-intelligence-mutation-shadow = []",
        ],
    )
    checks["agentd.feature_gated_module"] = contains_all(
        agentd_lib,
        [
            '#[cfg(feature = "qualification-intelligence-mutation-shadow")]\nmod shadow_intelligence_mutation_host;',
            "pub use shadow_intelligence_mutation_host::AgentdShadowIntelligenceMutationHost;",
        ],
    )
    checks["agentd.explicit_host_seam"] = contains_all(
        agentd,
        [
            "pub struct AgentdShadowIntelligenceMutationHost",
            "CognitiveStore::open_with_shadow_intelligence_mutation_host",
            "begin_shadow_intelligence_mutation",
            "prepare_shadow_intelligence_mutation_observation",
            "append_shadow_intelligence_mutation_observation",
            "observe_shadow_intelligence_mutation",
            "inspect_shadow_intelligence_mutation",
        ],
    )
    checks["agentd.prepare_append_envelope"] = contains_all(
        agentd,
        [
            "unwrap_prepared_payload",
            "prepared host envelope belongs to another Agentd identity or spawn",
            "prepared host envelope payload digest mismatch",
            "prepared host envelope receipt digest mismatch",
            "prepared host envelope crosses the shadow authority boundary",
            "prepared_envelope_round_trips_and_rejects_cross_spawn",
            "prepared_envelope_rejects_payload_and_authority_tamper",
        ],
    )
    checks["agentd.spawn_bound_operation"] = contains_all(
        agentd,
        [
            "effective_operation_id",
            "effective_lease_id",
            "host_bound_causal_root",
            "operation_and_causal_root_are_spawn_bound",
            "agentd-shadow-operation:",
            "agentd-shadow-lease:",
        ],
    )
    checks["agentd.not_runtime_attached"] = (
        "AgentdShadowIntelligenceMutationHost" not in runtime
        and "AgentdShadowIntelligenceMutationHost" not in app_runtime
    )
    checks["agentd.no_effect_path"] = not any(
        marker in agentd
        for marker in [
            "ProductionDurableWriter",
            "ProductionOutboxDispatcher",
            "ToolContributor",
            "attach_target",
            "dispatcher.dispatch",
            "refresh_scope_projection",
            "remember_with_",
            "correct_with_",
        ]
    )
    checks["agentd.authority_false"] = contains_all(
        agentd,
        [
            "AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_RUNTIME_WIRED: bool = false",
            "AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_APP_RUNTIME_ATTACHED: bool = false",
            "AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_TOOL_REGISTERED: bool = false",
            "AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_PRODUCTION_AUTHORITY: bool = false",
            "memory_write_performed_by_agentd: false",
            "projection_write_performed_by_agentd: false",
            "outbox_dispatch_performed_by_agentd: false",
        ],
    )

    current = status.get("current_tranche", {})
    dependency = status.get("dependency", {})
    claims = current.get("claims", {})
    checks["status.p0_4c_source_only"] = (
        current.get("id") == "P0.4c"
        and current.get("qualified") is False
        and claims.get("runtime_wired") is False
        and claims.get("default_open_wired") is False
        and claims.get("app_runtime_attached") is False
        and claims.get("tool_registered") is False
        and claims.get("production_authority") is False
        and claims.get("external_effects") is False
    )
    checks["status.p0_4b_unqualified_dependency"] = (
        dependency.get("id") == "P0.4b" and dependency.get("qualified") is False
    )
    checks["plan.boundary"] = contains_all(
        plan,
        [
            "SOURCE_ONLY",
            "ACTIVATION_BLOCKED",
            "runtime_wired=false",
            "default_open_wired=false",
            "app_runtime_attached=false",
            "tool_registered=false",
            "production_authority=false",
            "prepare",
            "append",
            "spawn generation",
        ],
    )
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
            source_gates,
            [
                "verify-hepta-intelligence-shadow-host.py",
                "shadow-host",
            ],
        )
        and contains_all(
            rust_matrix,
            [
                "cargo test --locked -p codex-hepta-memory intelligence_mutation_shadow_host",
                "cargo test --locked -p codex-hepta-agentd",
                "qualification-intelligence-mutation-shadow",
                "shadow_intelligence_mutation_host",
                "agentd-shadow-strict-clippy",
            ],
        )
    )

    return emit(checks)


if __name__ == "__main__":
    sys.exit(main())
