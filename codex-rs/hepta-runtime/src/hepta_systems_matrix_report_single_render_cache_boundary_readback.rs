use serde::Serialize;

use crate::current_reality_capability_registry_count;

pub const HEPTA_SYSTEMS_MATRIX_REPORT_SINGLE_RENDER_CACHE_BOUNDARY_READBACK_GATE: &str =
    "hepta_systems_matrix_report_single_render_cache_boundary_readback_gate";
pub const HEPTA_SYSTEMS_MATRIX_REPORT_SINGLE_RENDER_CACHE_BOUNDARY_READBACK_SCHEMA_VERSION: &str =
    "hepta_systems_matrix_report_single_render_cache_boundary_readback_v1";
pub const HEPTA_SYSTEMS_MATRIX_REPORT_SINGLE_RENDER_CACHE_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "close_controlled_live_evidence_before_status_canary_start";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_matrix_capability_count: usize,
    pub source_live_enabled_count: usize,
    pub controlled_live_blocker_count: usize,
    pub matrix_report_render_count: usize,
    pub single_render_projection_count: usize,
    pub downstream_consumer_count: usize,
    pub compact_cache_consumer_rewired: bool,
    pub dashboard_consumer_rewired: bool,
    pub matrix_cache_write_allowed: bool,
    pub matrix_cache_persisted: bool,
    pub compact_cache_persisted: bool,
    pub source_report_semantics_change_allowed: bool,
    pub downstream_direct_matrix_render_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub single_render_cache_boundary_readback_ready: bool,
    pub entries: Vec<HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackEntry {
    pub entry_id: &'static str,
    pub source_route: &'static str,
    pub consumer_route: &'static str,
    pub projected_in_memory: bool,
    pub matrix_report_render_consumed: bool,
    pub downstream_direct_matrix_render_required: bool,
    pub matrix_cache_written: bool,
    pub matrix_cache_persisted: bool,
    pub compact_cache_persisted: bool,
    pub source_report_semantics_changed: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub live_execution_started: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackSideEffects {
    pub filesystem_written: bool,
    pub matrix_cache_written: bool,
    pub matrix_cache_persisted: bool,
    pub compact_cache_persisted: bool,
    pub source_report_semantics_changed: bool,
    pub downstream_direct_matrix_render_performed: bool,
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

pub fn hepta_systems_matrix_report_single_render_cache_boundary_readback_report()
-> HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackReport {
    let entries = hepta_systems_matrix_report_single_render_cache_boundary_readback_entries();
    let single_render_projection_count = entries
        .iter()
        .filter(|entry| entry.projected_in_memory)
        .count();
    let downstream_consumer_count = entries
        .iter()
        .filter(|entry| entry.consumer_route.starts_with("scripts/hepta-systems-"))
        .count();
    let single_render_cache_boundary_readback_ready = entries.len() == 4
        && single_render_projection_count == 4
        && downstream_consumer_count == 2
        && entries.iter().all(|entry| {
            entry.projected_in_memory
                && entry.matrix_report_render_consumed
                && !entry.downstream_direct_matrix_render_required
                && !entry.matrix_cache_written
                && !entry.matrix_cache_persisted
                && !entry.compact_cache_persisted
                && !entry.source_report_semantics_changed
                && !entry.workflow_execution_started
                && !entry.replay_executed
                && !entry.event_log_written
                && !entry.sqlite_written
                && !entry.live_execution_started
        });

    HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_matrix_report_single_render_cache_boundary_readback",
        status: if single_render_cache_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_MATRIX_REPORT_SINGLE_RENDER_CACHE_BOUNDARY_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_MATRIX_REPORT_SINGLE_RENDER_CACHE_BOUNDARY_READBACK_SCHEMA_VERSION,
        source_matrix_capability_count: current_reality_capability_registry_count(),
        source_live_enabled_count: 0,
        controlled_live_blocker_count: 7,
        matrix_report_render_count: 1,
        single_render_projection_count,
        downstream_consumer_count,
        compact_cache_consumer_rewired: true,
        dashboard_consumer_rewired: true,
        matrix_cache_write_allowed: false,
        matrix_cache_persisted: false,
        compact_cache_persisted: false,
        source_report_semantics_change_allowed: false,
        downstream_direct_matrix_render_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        event_log_write_allowed: false,
        sqlite_write_allowed: false,
        live_execution_allowed: false,
        single_render_cache_boundary_readback_ready,
        entries,
        blockers: vec![
            "matrix_cache_write_disabled",
            "matrix_cache_persistence_disabled",
            "compact_cache_persistence_disabled",
            "source_report_semantics_change_disabled",
            "downstream_direct_matrix_render_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_MATRIX_REPORT_SINGLE_RENDER_CACHE_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_matrix_report_single_render_cache_boundary_readback_entries()
-> Vec<HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackEntry> {
    vec![
        single_render_entry(
            "matrix_capability_summary_projection",
            "scripts/hepta-systems-current-reality-capability-matrix-report.sh",
            "readback://current-reality-matrix/single-render/capability-summary",
        ),
        single_render_entry(
            "matrix_live_blocker_summary_projection",
            "scripts/hepta-systems-current-reality-capability-matrix-report.sh",
            "readback://current-reality-matrix/single-render/live-blockers",
        ),
        single_render_entry(
            "compact_cache_boundary_single_render_consumer",
            "scripts/hepta-systems-current-reality-capability-matrix-report.sh",
            "scripts/hepta-systems-current-reality-matrix-compact-cache-boundary-readback-report.sh",
        ),
        single_render_entry(
            "controlled_live_dashboard_single_render_consumer",
            "scripts/hepta-systems-current-reality-capability-matrix-report.sh",
            "scripts/hepta-systems-controlled-live-operator-readiness-dashboard-report.sh",
        ),
    ]
}

fn single_render_entry(
    entry_id: &'static str,
    source_route: &'static str,
    consumer_route: &'static str,
) -> HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackEntry {
    HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackEntry {
        entry_id,
        source_route,
        consumer_route,
        projected_in_memory: true,
        matrix_report_render_consumed: true,
        downstream_direct_matrix_render_required: false,
        matrix_cache_written: false,
        matrix_cache_persisted: false,
        compact_cache_persisted: false,
        source_report_semantics_changed: false,
        workflow_execution_started: false,
        replay_executed: false,
        event_log_written: false,
        sqlite_written: false,
        live_execution_started: false,
    }
}

impl HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            matrix_cache_written: false,
            matrix_cache_persisted: false,
            compact_cache_persisted: false,
            source_report_semantics_changed: false,
            downstream_direct_matrix_render_performed: false,
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
    fn single_render_boundary_projects_matrix_summary_without_cache_writes() {
        let report = hepta_systems_matrix_report_single_render_cache_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert_eq!(
            report.source_matrix_capability_count,
            current_reality_capability_registry_count()
        );
        assert_eq!(report.source_live_enabled_count, 0);
        assert_eq!(report.controlled_live_blocker_count, 7);
        assert_eq!(report.matrix_report_render_count, 1);
        assert_eq!(report.single_render_projection_count, 4);
        assert_eq!(report.downstream_consumer_count, 2);
        assert!(report.compact_cache_consumer_rewired);
        assert!(report.dashboard_consumer_rewired);
        assert!(report.single_render_cache_boundary_readback_ready);
    }

    #[test]
    fn single_render_boundary_keeps_persistence_and_live_closed() {
        let report = hepta_systems_matrix_report_single_render_cache_boundary_readback_report();

        assert!(!report.matrix_cache_write_allowed);
        assert!(!report.matrix_cache_persisted);
        assert!(!report.compact_cache_persisted);
        assert!(!report.source_report_semantics_change_allowed);
        assert!(!report.downstream_direct_matrix_render_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsMatrixReportSingleRenderCacheBoundaryReadbackSideEffects::none()
        );
    }

    #[test]
    fn single_render_entries_are_readback_only_downstream_consumers() {
        let report = hepta_systems_matrix_report_single_render_cache_boundary_readback_report();

        assert_eq!(report.entries.len(), 4);
        assert!(report.entries.iter().all(|entry| {
            entry.projected_in_memory
                && entry.matrix_report_render_consumed
                && !entry.downstream_direct_matrix_render_required
                && !entry.matrix_cache_written
                && !entry.matrix_cache_persisted
                && !entry.compact_cache_persisted
                && !entry.source_report_semantics_changed
                && !entry.workflow_execution_started
                && !entry.replay_executed
                && !entry.event_log_written
                && !entry.sqlite_written
                && !entry.live_execution_started
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.entry_id == "compact_cache_boundary_single_render_consumer"
                && entry.consumer_route.contains("compact-cache")
        }));
        assert!(report.entries.iter().any(|entry| {
            entry.entry_id == "controlled_live_dashboard_single_render_consumer"
                && entry.consumer_route.contains("dashboard")
        }));
    }
}
