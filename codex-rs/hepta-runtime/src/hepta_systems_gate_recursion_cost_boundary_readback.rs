use serde::Serialize;

use crate::hepta_current_reality_matrix_compact_cache_boundary_readback_report;

pub const HEPTA_SYSTEMS_GATE_RECURSION_COST_BOUNDARY_READBACK_GATE: &str =
    "hepta_systems_gate_recursion_cost_boundary_readback_gate";
pub const HEPTA_SYSTEMS_GATE_RECURSION_COST_BOUNDARY_READBACK_SCHEMA_VERSION: &str =
    "hepta_systems_gate_recursion_cost_boundary_readback_v1";
pub const HEPTA_SYSTEMS_GATE_RECURSION_COST_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_systems_matrix_report_single_render_cache_boundary_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsGateRecursionCostBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_matrix_capability_count: usize,
    pub source_matrix_live_enabled_count: usize,
    pub controlled_live_blocker_count: usize,
    pub cost_scope: &'static str,
    pub boundary_projection_count: usize,
    pub source_gate_recursion_boundary_count: usize,
    pub bounded_source_gate_count: usize,
    pub full_matrix_render_boundary_count: usize,
    pub lane_lock_boundary_count: usize,
    pub full_upstream_gate_chain_invocation_allowed: bool,
    pub matrix_report_cache_write_allowed: bool,
    pub compact_cache_persistence_allowed: bool,
    pub source_report_semantics_change_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub gate_recursion_cost_boundary_readback_ready: bool,
    pub entries: Vec<HeptaSystemsGateRecursionCostBoundaryReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsGateRecursionCostBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsGateRecursionCostBoundaryReadbackEntry {
    pub entry_id: &'static str,
    pub boundary_area: &'static str,
    pub source_route: &'static str,
    pub current_cost_state: &'static str,
    pub recommended_reduction: &'static str,
    pub projected_in_memory: bool,
    pub source_gate_recursion_bounded: bool,
    pub full_upstream_gate_chain_invoked: bool,
    pub full_matrix_render_required: bool,
    pub lane_lock_serialization_required: bool,
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
pub struct HeptaSystemsGateRecursionCostBoundaryReadbackSideEffects {
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

pub fn hepta_systems_gate_recursion_cost_boundary_readback_report()
-> HeptaSystemsGateRecursionCostBoundaryReadbackReport {
    let current_reality = hepta_current_reality_matrix_compact_cache_boundary_readback_report();
    let entries = hepta_systems_gate_recursion_cost_boundary_readback_entries();
    let boundary_projection_count = entries
        .iter()
        .filter(|entry| entry.projected_in_memory)
        .count();
    let source_gate_recursion_boundary_count = entries
        .iter()
        .filter(|entry| entry.boundary_area == "source_gate_recursion")
        .count();
    let bounded_source_gate_count = entries
        .iter()
        .filter(|entry| entry.source_gate_recursion_bounded)
        .count();
    let full_matrix_render_boundary_count = entries
        .iter()
        .filter(|entry| entry.full_matrix_render_required)
        .count();
    let lane_lock_boundary_count = entries
        .iter()
        .filter(|entry| entry.lane_lock_serialization_required)
        .count();
    let gate_recursion_cost_boundary_readback_ready =
        current_reality.source_matrix_capability_count > 0
            && current_reality.source_live_enabled_count == 0
            && entries.len() == 4
            && boundary_projection_count == 4
            && source_gate_recursion_boundary_count == 2
            && bounded_source_gate_count == 1
            && full_matrix_render_boundary_count == 1
            && lane_lock_boundary_count == 1
            && entries.iter().all(|entry| {
                !entry.matrix_cache_written
                    && !entry.compact_cache_persisted
                    && !entry.source_report_semantics_changed
                    && !entry.cargo_test_executed_by_report
                    && !entry.workflow_execution_started
                    && !entry.replay_executed
                    && !entry.event_log_written
                    && !entry.sqlite_written
                    && !entry.live_execution_started
            });

    HeptaSystemsGateRecursionCostBoundaryReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_gate_recursion_cost_boundary_readback",
        status: if gate_recursion_cost_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_GATE_RECURSION_COST_BOUNDARY_READBACK_GATE,
        schema_version: HEPTA_SYSTEMS_GATE_RECURSION_COST_BOUNDARY_READBACK_SCHEMA_VERSION,
        source_matrix_capability_count: current_reality.source_matrix_capability_count,
        source_matrix_live_enabled_count: current_reality.source_live_enabled_count,
        controlled_live_blocker_count: 7,
        cost_scope: "readback_only_gate_recursion_cost_boundary_no_cache_write",
        boundary_projection_count,
        source_gate_recursion_boundary_count,
        bounded_source_gate_count,
        full_matrix_render_boundary_count,
        lane_lock_boundary_count,
        full_upstream_gate_chain_invocation_allowed: false,
        matrix_report_cache_write_allowed: false,
        compact_cache_persistence_allowed: false,
        source_report_semantics_change_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        event_log_write_allowed: false,
        sqlite_write_allowed: false,
        live_execution_allowed: false,
        gate_recursion_cost_boundary_readback_ready,
        entries,
        blockers: vec![
            "full_upstream_gate_chain_invocation_disabled",
            "matrix_report_cache_write_disabled",
            "compact_cache_persistence_disabled",
            "source_report_semantics_change_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_GATE_RECURSION_COST_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsGateRecursionCostBoundaryReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_gate_recursion_cost_boundary_readback_entries()
-> Vec<HeptaSystemsGateRecursionCostBoundaryReadbackEntry> {
    vec![
        HeptaSystemsGateRecursionCostBoundaryReadbackEntry {
            entry_id: "recovery_receipt_source_report_invariant_boundary",
            boundary_area: "source_gate_recursion",
            source_route: "scripts/hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-feature-gated-readback-gate.sh",
            current_cost_state: "bounded_to_source_report_invariants",
            recommended_reduction: "keep new gates on source report invariant checks plus local targeted tests",
            projected_in_memory: true,
            source_gate_recursion_bounded: true,
            full_upstream_gate_chain_invoked: false,
            full_matrix_render_required: false,
            lane_lock_serialization_required: false,
            matrix_cache_written: false,
            compact_cache_persisted: false,
            source_report_semantics_changed: false,
            cargo_test_executed_by_report: false,
            workflow_execution_started: false,
            replay_executed: false,
            event_log_written: false,
            sqlite_written: false,
            live_execution_started: false,
        },
        HeptaSystemsGateRecursionCostBoundaryReadbackEntry {
            entry_id: "upstream_recovery_window_source_gate_chain_boundary",
            boundary_area: "source_gate_recursion",
            source_route: "scripts/hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback-gate.sh",
            current_cost_state: "full_upstream_source_gate_chain_remains_expensive",
            recommended_reduction: "migrate upstream source gates to source report invariant checks before adding more suffix rows",
            projected_in_memory: true,
            source_gate_recursion_bounded: false,
            full_upstream_gate_chain_invoked: true,
            full_matrix_render_required: false,
            lane_lock_serialization_required: false,
            matrix_cache_written: false,
            compact_cache_persisted: false,
            source_report_semantics_changed: false,
            cargo_test_executed_by_report: false,
            workflow_execution_started: false,
            replay_executed: false,
            event_log_written: false,
            sqlite_written: false,
            live_execution_started: false,
        },
        HeptaSystemsGateRecursionCostBoundaryReadbackEntry {
            entry_id: "current_reality_matrix_full_render_boundary",
            boundary_area: "matrix_report_render",
            source_route: "scripts/hepta-systems-current-reality-capability-matrix-report.sh",
            current_cost_state: "full_matrix_render_still_required_by_matrix_and_dashboard_reports",
            recommended_reduction: "add a single-render matrix summary boundary before persistent cache writes",
            projected_in_memory: true,
            source_gate_recursion_bounded: false,
            full_upstream_gate_chain_invoked: false,
            full_matrix_render_required: true,
            lane_lock_serialization_required: false,
            matrix_cache_written: false,
            compact_cache_persisted: false,
            source_report_semantics_changed: false,
            cargo_test_executed_by_report: false,
            workflow_execution_started: false,
            replay_executed: false,
            event_log_written: false,
            sqlite_written: false,
            live_execution_started: false,
        },
        HeptaSystemsGateRecursionCostBoundaryReadbackEntry {
            entry_id: "hepta_systems_lane_lock_serialization_boundary",
            boundary_area: "lane_lock",
            source_route: "/Users/qianqi/.openclaw/bin/hepta-heavy-lock",
            current_cost_state: "parallel_hepta_systems_heavy_gates_are_serialized_by_lane_lock",
            recommended_reduction: "keep heavy gates serial while splitting readback reports from cargo tests",
            projected_in_memory: true,
            source_gate_recursion_bounded: false,
            full_upstream_gate_chain_invoked: false,
            full_matrix_render_required: false,
            lane_lock_serialization_required: true,
            matrix_cache_written: false,
            compact_cache_persisted: false,
            source_report_semantics_changed: false,
            cargo_test_executed_by_report: false,
            workflow_execution_started: false,
            replay_executed: false,
            event_log_written: false,
            sqlite_written: false,
            live_execution_started: false,
        },
    ]
}

impl HeptaSystemsGateRecursionCostBoundaryReadbackSideEffects {
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
    fn gate_recursion_cost_boundary_projects_current_costs_without_writes() {
        let report = hepta_systems_gate_recursion_cost_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert_eq!(
            report.source_matrix_capability_count,
            hepta_current_reality_matrix_compact_cache_boundary_readback_report()
                .source_matrix_capability_count
        );
        assert_eq!(report.source_matrix_live_enabled_count, 0);
        assert_eq!(report.controlled_live_blocker_count, 7);
        assert_eq!(report.boundary_projection_count, 4);
        assert_eq!(report.source_gate_recursion_boundary_count, 2);
        assert_eq!(report.bounded_source_gate_count, 1);
        assert_eq!(report.full_matrix_render_boundary_count, 1);
        assert_eq!(report.lane_lock_boundary_count, 1);
        assert!(report.gate_recursion_cost_boundary_readback_ready);
    }

    #[test]
    fn gate_recursion_cost_entries_are_readback_only() {
        let report = hepta_systems_gate_recursion_cost_boundary_readback_report();

        assert!(report.entries.iter().all(|entry| {
            entry.projected_in_memory
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
        assert!(report.entries.iter().any(|entry| entry.entry_id
            == "recovery_receipt_source_report_invariant_boundary"
            && entry.source_gate_recursion_bounded
            && !entry.full_upstream_gate_chain_invoked));
        assert!(report.entries.iter().any(|entry| entry.entry_id
            == "current_reality_matrix_full_render_boundary"
            && entry.full_matrix_render_required));
    }

    #[test]
    fn gate_recursion_cost_side_effects_remain_closed() {
        let report = hepta_systems_gate_recursion_cost_boundary_readback_report();

        assert!(!report.full_upstream_gate_chain_invocation_allowed);
        assert!(!report.matrix_report_cache_write_allowed);
        assert!(!report.compact_cache_persistence_allowed);
        assert!(!report.source_report_semantics_change_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsGateRecursionCostBoundaryReadbackSideEffects::none()
        );
    }
}
