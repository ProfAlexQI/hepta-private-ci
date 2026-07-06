use crate::hepta_systems_gate_recursion_cost_boundary_readback::hepta_systems_gate_recursion_cost_boundary_readback_report;
use serde::Serialize;

pub const HEPTA_SYSTEMS_GATE_RECURSION_LEAN_CONTRACT_READBACK_GATE: &str =
    "hepta_systems_gate_recursion_lean_contract_readback_gate";
pub const HEPTA_SYSTEMS_GATE_RECURSION_LEAN_CONTRACT_READBACK_SCHEMA_VERSION: &str =
    "hepta_systems_gate_recursion_lean_contract_readback_v1";
pub const HEPTA_SYSTEMS_GATE_RECURSION_LEAN_CONTRACT_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_systems_workgraph_legacy_gate_recursion_inventory_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsGateRecursionLeanContractReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_cost_boundary_ready: bool,
    pub source_cost_boundary_projection_count: usize,
    pub contract_scope: &'static str,
    pub contract_entry_count: usize,
    pub source_report_smoke_contract_count: usize,
    pub targeted_rust_test_contract_count: usize,
    pub legacy_recursion_inventory_count: usize,
    pub current_full_upstream_gate_chain_count: usize,
    pub contract_full_upstream_gate_chain_allowed_count: usize,
    pub matrix_cache_write_allowed: bool,
    pub compact_cache_persistence_allowed: bool,
    pub source_report_semantics_change_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub lean_contract_readback_ready: bool,
    pub entries: Vec<HeptaSystemsGateRecursionLeanContractReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsGateRecursionLeanContractReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsGateRecursionLeanContractReadbackEntry {
    pub entry_id: &'static str,
    pub contract_area: &'static str,
    pub source_route: &'static str,
    pub current_cost_state: &'static str,
    pub required_contract: &'static str,
    pub projected_in_memory: bool,
    pub source_report_smoke_required: bool,
    pub targeted_rust_test_required: bool,
    pub legacy_recursion_inventory_required: bool,
    pub current_full_upstream_gate_chain_invoked: bool,
    pub contract_full_upstream_gate_chain_allowed: bool,
    pub downstream_direct_matrix_render_required: bool,
    pub matrix_cache_written: bool,
    pub compact_cache_persisted: bool,
    pub source_report_semantics_changed: bool,
    pub cargo_test_executed_by_report: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub live_execution_started: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsGateRecursionLeanContractReadbackSideEffects {
    pub filesystem_written: bool,
    pub matrix_cache_written: bool,
    pub compact_cache_persisted: bool,
    pub source_report_semantics_changed: bool,
    pub full_upstream_gate_chain_invoked: bool,
    pub cargo_test_executed_by_report: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub gateway_or_auth_mutated: bool,
    pub native_post_mutation_performed: bool,
    pub channel_send_performed: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn hepta_systems_gate_recursion_lean_contract_readback_report()
-> HeptaSystemsGateRecursionLeanContractReadbackReport {
    let source = hepta_systems_gate_recursion_cost_boundary_readback_report();
    let entries = hepta_systems_gate_recursion_lean_contract_readback_entries();
    let contract_entry_count = entries.len();
    let source_report_smoke_contract_count = entries
        .iter()
        .filter(|entry| entry.source_report_smoke_required)
        .count();
    let targeted_rust_test_contract_count = entries
        .iter()
        .filter(|entry| entry.targeted_rust_test_required)
        .count();
    let legacy_recursion_inventory_count = entries
        .iter()
        .filter(|entry| entry.legacy_recursion_inventory_required)
        .count();
    let current_full_upstream_gate_chain_count = entries
        .iter()
        .filter(|entry| entry.current_full_upstream_gate_chain_invoked)
        .count();
    let contract_full_upstream_gate_chain_allowed_count = entries
        .iter()
        .filter(|entry| entry.contract_full_upstream_gate_chain_allowed)
        .count();
    let lean_contract_readback_ready = source.gate_recursion_cost_boundary_readback_ready
        && source.boundary_projection_count == 4
        && contract_entry_count == 5
        && source_report_smoke_contract_count == 3
        && targeted_rust_test_contract_count == 3
        && legacy_recursion_inventory_count == 2
        && current_full_upstream_gate_chain_count == 2
        && contract_full_upstream_gate_chain_allowed_count == 0
        && entries.iter().all(|entry| {
            entry.projected_in_memory
                && !entry.contract_full_upstream_gate_chain_allowed
                && !entry.downstream_direct_matrix_render_required
                && !entry.matrix_cache_written
                && !entry.compact_cache_persisted
                && !entry.source_report_semantics_changed
                && !entry.cargo_test_executed_by_report
                && !entry.workflow_execution_started
                && !entry.replay_executed
                && !entry.event_log_written
                && !entry.sqlite_written
                && !entry.live_execution_started
        });

    HeptaSystemsGateRecursionLeanContractReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_gate_recursion_lean_contract_readback",
        status: if lean_contract_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_GATE_RECURSION_LEAN_CONTRACT_READBACK_GATE,
        schema_version: HEPTA_SYSTEMS_GATE_RECURSION_LEAN_CONTRACT_READBACK_SCHEMA_VERSION,
        source_cost_boundary_ready: source.gate_recursion_cost_boundary_readback_ready,
        source_cost_boundary_projection_count: source.boundary_projection_count,
        contract_scope: "source_report_smoke_plus_targeted_test_no_recursive_source_gate_chain",
        contract_entry_count,
        source_report_smoke_contract_count,
        targeted_rust_test_contract_count,
        legacy_recursion_inventory_count,
        current_full_upstream_gate_chain_count,
        contract_full_upstream_gate_chain_allowed_count,
        matrix_cache_write_allowed: false,
        compact_cache_persistence_allowed: false,
        source_report_semantics_change_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        event_log_write_allowed: false,
        sqlite_write_allowed: false,
        live_execution_allowed: false,
        lean_contract_readback_ready,
        entries,
        blockers: vec![
            "recursive_source_gate_chain_disabled_for_new_gates",
            "legacy_recursive_source_gate_inventory_required",
            "matrix_cache_write_disabled",
            "compact_cache_persistence_disabled",
            "source_report_semantics_change_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_GATE_RECURSION_LEAN_CONTRACT_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsGateRecursionLeanContractReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_gate_recursion_lean_contract_readback_entries()
-> Vec<HeptaSystemsGateRecursionLeanContractReadbackEntry> {
    vec![
        lean_entry(
            "recovery_receipt_local_source_report_contract",
            "source_report_invariant_smoke",
            "scripts/hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-local-persistence-readback-gate.sh",
            "bounded_to_source_report_invariants_and_targeted_rust_test",
            "source report smoke plus one targeted Rust test; no upstream source-gate recursion",
            true,
            true,
            false,
            false,
        ),
        lean_entry(
            "legacy_recovery_window_feature_gate_inventory",
            "legacy_source_gate_recursion_inventory",
            "scripts/hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback-gate.sh",
            "legacy_gate_still_invokes_upstream_source_gate_chain",
            "inventory and migrate to source report smoke plus targeted Rust test",
            false,
            false,
            true,
            true,
        ),
        lean_entry(
            "legacy_workgraph_closeout_receipt_chain_inventory",
            "legacy_source_gate_recursion_inventory",
            "scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-rerun-preview-gate.sh",
            "legacy_workgraph_gate_chain_risk_remains_visible",
            "inventory before another WorkGraph suffix or closeout rerun path is added",
            false,
            false,
            true,
            true,
        ),
        lean_entry(
            "matrix_single_render_contract",
            "matrix_report_single_render_contract",
            "scripts/hepta-systems-matrix-report-single-render-cache-boundary-readback-gate.sh",
            "single_matrix_render_contract_ready_without_cache_write",
            "consume one source matrix render and keep downstream direct renders removed",
            true,
            true,
            false,
            false,
        ),
        lean_entry(
            "controlled_live_dashboard_single_render_contract",
            "dashboard_single_render_contract",
            "scripts/hepta-systems-controlled-live-operator-readiness-dashboard-gate.sh",
            "dashboard_consumes_single_render_boundary_without_matrix_rerun",
            "validate dashboard from source reports and targeted Rust tests only",
            true,
            true,
            false,
            false,
        ),
    ]
}

fn lean_entry(
    entry_id: &'static str,
    contract_area: &'static str,
    source_route: &'static str,
    current_cost_state: &'static str,
    required_contract: &'static str,
    source_report_smoke_required: bool,
    targeted_rust_test_required: bool,
    legacy_recursion_inventory_required: bool,
    current_full_upstream_gate_chain_invoked: bool,
) -> HeptaSystemsGateRecursionLeanContractReadbackEntry {
    HeptaSystemsGateRecursionLeanContractReadbackEntry {
        entry_id,
        contract_area,
        source_route,
        current_cost_state,
        required_contract,
        projected_in_memory: true,
        source_report_smoke_required,
        targeted_rust_test_required,
        legacy_recursion_inventory_required,
        current_full_upstream_gate_chain_invoked,
        contract_full_upstream_gate_chain_allowed: false,
        downstream_direct_matrix_render_required: false,
        matrix_cache_written: false,
        compact_cache_persisted: false,
        source_report_semantics_changed: false,
        cargo_test_executed_by_report: false,
        workflow_execution_started: false,
        replay_executed: false,
        event_log_written: false,
        sqlite_written: false,
        live_execution_started: false,
    }
}

impl HeptaSystemsGateRecursionLeanContractReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            matrix_cache_written: false,
            compact_cache_persisted: false,
            source_report_semantics_changed: false,
            full_upstream_gate_chain_invoked: false,
            cargo_test_executed_by_report: false,
            workflow_execution_started: false,
            replay_executed: false,
            event_log_written: false,
            sqlite_written: false,
            provider_invoked: false,
            model_invoked: false,
            gateway_or_auth_mutated: false,
            native_post_mutation_performed: false,
            channel_send_performed: false,
            package_or_release_written: false,
            public_ga_promoted: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lean_contract_projects_source_report_smoke_and_targeted_test_boundary() {
        let report = hepta_systems_gate_recursion_lean_contract_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_cost_boundary_ready);
        assert_eq!(report.source_cost_boundary_projection_count, 4);
        assert_eq!(report.contract_entry_count, 5);
        assert_eq!(report.source_report_smoke_contract_count, 3);
        assert_eq!(report.targeted_rust_test_contract_count, 3);
        assert_eq!(report.legacy_recursion_inventory_count, 2);
        assert_eq!(report.current_full_upstream_gate_chain_count, 2);
        assert_eq!(report.contract_full_upstream_gate_chain_allowed_count, 0);
        assert!(report.lean_contract_readback_ready);
    }

    #[test]
    fn lean_contract_entries_keep_new_recursion_closed() {
        let report = hepta_systems_gate_recursion_lean_contract_readback_report();

        assert!(report.entries.iter().all(|entry| {
            entry.projected_in_memory
                && !entry.contract_full_upstream_gate_chain_allowed
                && !entry.downstream_direct_matrix_render_required
                && !entry.matrix_cache_written
                && !entry.compact_cache_persisted
                && !entry.source_report_semantics_changed
                && !entry.cargo_test_executed_by_report
                && !entry.workflow_execution_started
                && !entry.replay_executed
                && !entry.event_log_written
                && !entry.sqlite_written
                && !entry.live_execution_started
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.entry_id == "recovery_receipt_local_source_report_contract"
                && entry.source_report_smoke_required
                && entry.targeted_rust_test_required
                && !entry.current_full_upstream_gate_chain_invoked
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.entry_id == "legacy_workgraph_closeout_receipt_chain_inventory"
                && entry.legacy_recursion_inventory_required
                && entry.current_full_upstream_gate_chain_invoked
        }));
    }

    #[test]
    fn lean_contract_side_effects_remain_closed() {
        let report = hepta_systems_gate_recursion_lean_contract_readback_report();

        assert!(!report.matrix_cache_write_allowed);
        assert!(!report.compact_cache_persistence_allowed);
        assert!(!report.source_report_semantics_change_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsGateRecursionLeanContractReadbackSideEffects::none()
        );
    }
}
