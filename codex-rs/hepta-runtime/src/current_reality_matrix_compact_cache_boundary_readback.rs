use serde::Serialize;

pub const CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_GATE: &str =
    "current_reality_matrix_compact_cache_boundary_readback_gate";
pub const CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_SCHEMA_VERSION: &str =
    "current_reality_matrix_compact_cache_boundary_readback_v1";
pub const CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "close_controlled_live_evidence_before_status_canary_start";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealityMatrixCompactCacheBoundaryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_matrix_capability_count: usize,
    pub source_matrix_ready_count: usize,
    pub source_live_enabled_count: usize,
    pub controlled_live_blocker_count: usize,
    pub compact_projection_count: usize,
    pub matrix_report_render_count: usize,
    pub dashboard_gate_matrix_rerun_removed: bool,
    pub cache_write_allowed: bool,
    pub cache_persisted: bool,
    pub evidence_recording_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub decision_recording_allowed: bool,
    pub live_execution_allowed: bool,
    pub compact_cache_boundary_readback_ready: bool,
    pub entries: Vec<CurrentRealityMatrixCompactCacheBoundaryReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: CurrentRealityMatrixCompactCacheBoundaryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealityMatrixCompactCacheBoundaryReadbackEntry {
    pub entry_id: &'static str,
    pub source: &'static str,
    pub readback_route: &'static str,
    pub projected_in_memory: bool,
    pub cache_write_allowed: bool,
    pub cache_persisted: bool,
    pub evidence_recording_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub decision_recording_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CurrentRealityMatrixCompactCacheBoundaryReadbackSideEffects {
    pub filesystem_written: bool,
    pub matrix_cache_written: bool,
    pub compact_cache_persisted: bool,
    pub evidence_recorded: bool,
    pub approval_accepted: bool,
    pub decision_recorded: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub live_execution_started: bool,
}

pub fn hepta_current_reality_matrix_compact_cache_boundary_readback_report()
-> CurrentRealityMatrixCompactCacheBoundaryReadbackReport {
    let entries = current_reality_matrix_compact_cache_boundary_readback_entries();
    let compact_projection_count = entries
        .iter()
        .filter(|entry| entry.projected_in_memory)
        .count();
    let compact_cache_boundary_readback_ready = compact_projection_count == entries.len()
        && !entries.iter().any(|entry| {
            entry.cache_write_allowed
                || entry.cache_persisted
                || entry.evidence_recording_allowed
                || entry.approval_acceptance_allowed
                || entry.decision_recording_allowed
                || entry.live_execution_allowed
        });

    CurrentRealityMatrixCompactCacheBoundaryReadbackReport {
        runtime: "hepta",
        surface: "current_reality_matrix_compact_cache_boundary_readback",
        status: if compact_cache_boundary_readback_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_GATE,
        schema_version: CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_SCHEMA_VERSION,
        source_matrix_capability_count: 104,
        source_matrix_ready_count: 104,
        source_live_enabled_count: 0,
        controlled_live_blocker_count: 7,
        compact_projection_count,
        matrix_report_render_count: 1,
        dashboard_gate_matrix_rerun_removed: true,
        cache_write_allowed: false,
        cache_persisted: false,
        evidence_recording_allowed: false,
        approval_acceptance_allowed: false,
        decision_recording_allowed: false,
        live_execution_allowed: false,
        compact_cache_boundary_readback_ready,
        entries,
        blockers: vec![
            "matrix_cache_write_disabled",
            "compact_cache_persistence_disabled",
            "evidence_recording_disabled",
            "approval_acceptance_disabled",
            "decision_recording_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            CURRENT_REALITY_MATRIX_COMPACT_CACHE_BOUNDARY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: CurrentRealityMatrixCompactCacheBoundaryReadbackSideEffects::none(),
    }
}

pub fn current_reality_matrix_compact_cache_boundary_readback_entries()
-> Vec<CurrentRealityMatrixCompactCacheBoundaryReadbackEntry> {
    vec![
        compact_entry(
            "matrix_capability_counts",
            "current_reality_capability_matrix",
            "readback://current-reality-matrix/compact-cache/capability-counts",
        ),
        compact_entry(
            "matrix_live_blockers",
            "current_reality_capability_matrix",
            "readback://current-reality-matrix/compact-cache/live-blockers",
        ),
        compact_entry(
            "dirty_worktree_counts",
            "current_reality_capability_matrix",
            "readback://current-reality-matrix/compact-cache/dirty-worktree-counts",
        ),
        compact_entry(
            "dashboard_matrix_rerun_boundary",
            "controlled_live_operator_readiness_dashboard_gate",
            "readback://current-reality-matrix/compact-cache/dashboard-rerun-boundary",
        ),
    ]
}

fn compact_entry(
    entry_id: &'static str,
    source: &'static str,
    readback_route: &'static str,
) -> CurrentRealityMatrixCompactCacheBoundaryReadbackEntry {
    CurrentRealityMatrixCompactCacheBoundaryReadbackEntry {
        entry_id,
        source,
        readback_route,
        projected_in_memory: true,
        cache_write_allowed: false,
        cache_persisted: false,
        evidence_recording_allowed: false,
        approval_acceptance_allowed: false,
        decision_recording_allowed: false,
        live_execution_allowed: false,
    }
}

impl CurrentRealityMatrixCompactCacheBoundaryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            matrix_cache_written: false,
            compact_cache_persisted: false,
            evidence_recorded: false,
            approval_accepted: false,
            decision_recorded: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_cache_boundary_projects_matrix_readback_without_cache_writes() {
        let report = hepta_current_reality_matrix_compact_cache_boundary_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert_eq!(report.source_matrix_capability_count, 104);
        assert_eq!(report.source_matrix_ready_count, 104);
        assert_eq!(report.source_live_enabled_count, 0);
        assert_eq!(report.controlled_live_blocker_count, 7);
        assert_eq!(report.compact_projection_count, 4);
        assert_eq!(report.matrix_report_render_count, 1);
        assert!(report.dashboard_gate_matrix_rerun_removed);
        assert!(report.compact_cache_boundary_readback_ready);
    }

    #[test]
    fn compact_cache_boundary_keeps_mutation_and_live_closed() {
        let report = hepta_current_reality_matrix_compact_cache_boundary_readback_report();

        assert!(!report.cache_write_allowed);
        assert!(!report.cache_persisted);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.decision_recording_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            CurrentRealityMatrixCompactCacheBoundaryReadbackSideEffects::none()
        );
    }

    #[test]
    fn compact_cache_entries_are_readback_only() {
        let report = hepta_current_reality_matrix_compact_cache_boundary_readback_report();

        assert_eq!(report.entries.len(), 4);
        assert!(report.entries.iter().all(|entry| {
            entry.projected_in_memory
                && entry
                    .readback_route
                    .starts_with("readback://current-reality-matrix/compact-cache/")
                && !entry.cache_write_allowed
                && !entry.cache_persisted
                && !entry.evidence_recording_allowed
                && !entry.approval_acceptance_allowed
                && !entry.decision_recording_allowed
                && !entry.live_execution_allowed
        }));
    }
}
