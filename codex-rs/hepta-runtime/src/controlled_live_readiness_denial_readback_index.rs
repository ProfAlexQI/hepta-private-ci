use crate::controlled_live_readiness_audit::controlled_live_readiness_audit_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_READINESS_DENIAL_READBACK_INDEX_GATE: &str =
    "controlled_live_readiness_denial_readback_index_gate";
pub const CONTROLLED_LIVE_READINESS_DENIAL_READBACK_INDEX_SCHEMA_VERSION: &str =
    "controlled_live_readiness_denial_readback_index_v1";
pub const CONTROLLED_LIVE_READINESS_DENIAL_READBACK_INDEX_RECOMMENDED_NEXT_GATE: &str =
    "phase5b_controlled_live_operator_packet_preview_without_approval_request";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveReadinessDenialReadbackIndexReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_audit_ready: bool,
    pub source_cutover_blocked: bool,
    pub source_blocker_count: usize,
    pub index_entry_count: usize,
    pub queryable_entry_count: usize,
    pub operator_facing_entry_count: usize,
    pub readback_route_count: usize,
    pub accepted_denial_count: usize,
    pub waived_blocker_count: usize,
    pub readback_index_ready: bool,
    pub controlled_live_cutover_ready: bool,
    pub ready_for_approval_request: bool,
    pub ready_for_approval_recording: bool,
    pub ready_for_readback_persistence: bool,
    pub ready_for_live_execution: bool,
    pub entries: Vec<ControlledLiveReadinessDenialReadbackEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveReadinessDenialReadbackIndexSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveReadinessDenialReadbackEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub layer: &'static str,
    pub query_key: &'static str,
    pub readback_route: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub current_state: &'static str,
    pub queryable: bool,
    pub operator_facing: bool,
    pub blocks_cutover: bool,
    pub operator_recoverable: bool,
    pub waiver_allowed: bool,
    pub acceptance_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveReadinessDenialReadbackIndexSideEffects {
    pub approval_requested: bool,
    pub approval_recorded: bool,
    pub blocker_waived: bool,
    pub denial_accepted: bool,
    pub readback_persisted: bool,
    pub ledger_written: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub native_post_mutation_performed: bool,
    pub gateway_or_auth_mutated: bool,
    pub telegram_transport_mutated: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub rollback_executed: bool,
    pub kill_switch_mutated: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn controlled_live_readiness_denial_readback_index_report()
-> ControlledLiveReadinessDenialReadbackIndexReport {
    let audit = controlled_live_readiness_audit_report();
    let entries = controlled_live_readiness_denial_readback_entries();
    let queryable_entry_count = entries.iter().filter(|entry| entry.queryable).count();
    let operator_facing_entry_count = entries.iter().filter(|entry| entry.operator_facing).count();
    let readback_route_count = entries
        .iter()
        .filter(|entry| !entry.readback_route.is_empty())
        .count();
    let accepted_denial_count = entries
        .iter()
        .filter(|entry| entry.acceptance_allowed)
        .count();
    let waived_blocker_count = entries.iter().filter(|entry| entry.waiver_allowed).count();
    let readback_index_ready = audit.controlled_live_audit_ready
        && !audit.controlled_live_cutover_ready
        && audit.blocker_count == 7
        && entries.len() == 7
        && queryable_entry_count == 7
        && operator_facing_entry_count == 7
        && readback_route_count == 7
        && accepted_denial_count == 0
        && waived_blocker_count == 0
        && entries.iter().all(|entry| {
            entry.blocks_cutover
                && entry.operator_recoverable
                && !entry.waiver_allowed
                && !entry.acceptance_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveReadinessDenialReadbackIndexReport {
        runtime: "hepta",
        surface: "controlled_live_readiness_denial_readback_index",
        status: if readback_index_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_READINESS_DENIAL_READBACK_INDEX_GATE,
        schema_version: CONTROLLED_LIVE_READINESS_DENIAL_READBACK_INDEX_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_audit_ready: audit.controlled_live_audit_ready,
        source_cutover_blocked: !audit.controlled_live_cutover_ready,
        source_blocker_count: audit.blocker_count,
        index_entry_count: entries.len(),
        queryable_entry_count,
        operator_facing_entry_count,
        readback_route_count,
        accepted_denial_count,
        waived_blocker_count,
        readback_index_ready,
        controlled_live_cutover_ready: false,
        ready_for_approval_request: false,
        ready_for_approval_recording: false,
        ready_for_readback_persistence: false,
        ready_for_live_execution: false,
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_READINESS_DENIAL_READBACK_INDEX_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveReadinessDenialReadbackIndexSideEffects::none(),
    }
}

pub fn controlled_live_readiness_denial_readback_entries()
-> Vec<ControlledLiveReadinessDenialReadbackEntry> {
    vec![
        entry(
            "dirty_worktree_boundary_readback",
            "dirty_worktree_boundary",
            "release",
            "controlled_live.blockers.dirty_worktree_boundary",
            "controlled_live_readiness_denial.dirty_worktree_boundary",
            "Dirty worktree boundary",
            "A clean scoped worktree or explicit release boundary attestation",
            "missing",
        ),
        entry(
            "operator_live_approval_missing_readback",
            "operator_live_approval_missing",
            "operator",
            "controlled_live.blockers.operator_live_approval_missing",
            "controlled_live_readiness_denial.operator_live_approval_missing",
            "Operator live approval",
            "Explicit operator live approval packet with scope, payload hash, and rollback owner",
            "missing",
        ),
        entry(
            "fresh_soak_readback_missing_readback",
            "fresh_soak_readback_missing",
            "observability",
            "controlled_live.blockers.fresh_soak_readback_missing",
            "controlled_live_readiness_denial.fresh_soak_readback_missing",
            "Fresh soak/readback evidence",
            "Fresh soak samples and readback evidence for this exact cutover",
            "missing",
        ),
        entry(
            "credential_boundary_attestation_missing_readback",
            "credential_boundary_attestation_missing",
            "security",
            "controlled_live.blockers.credential_boundary_attestation_missing",
            "controlled_live_readiness_denial.credential_boundary_attestation_missing",
            "Credential boundary attestation",
            "Credential access boundary attestation without exposing secrets",
            "missing",
        ),
        entry(
            "gateway_native_telegram_post_boundary_approval_missing_readback",
            "gateway_native_telegram_post_boundary_approval_missing",
            "transport",
            "controlled_live.blockers.gateway_native_telegram_post_boundary_approval_missing",
            "controlled_live_readiness_denial.gateway_native_telegram_post_boundary_approval_missing",
            "Gateway/Native/Telegram POST boundary approval",
            "Explicit transport mutation boundary approval for Gateway, Native POST, and Telegram",
            "missing",
        ),
        entry(
            "rollback_rehearsal_missing_readback",
            "rollback_rehearsal_missing",
            "rollback",
            "controlled_live.blockers.rollback_rehearsal_missing",
            "controlled_live_readiness_denial.rollback_rehearsal_missing",
            "Rollback rehearsal evidence",
            "Rollback rehearsal evidence tied to the cutover payload and owner",
            "missing",
        ),
        entry(
            "kill_switch_rehearsal_missing_readback",
            "kill_switch_rehearsal_missing",
            "rollback",
            "controlled_live.blockers.kill_switch_rehearsal_missing",
            "controlled_live_readiness_denial.kill_switch_rehearsal_missing",
            "Kill-switch rehearsal evidence",
            "Kill-switch rehearsal evidence tied to the cutover payload and owner",
            "missing",
        ),
    ]
}

fn entry(
    id: &'static str,
    source_blocker_id: &'static str,
    layer: &'static str,
    query_key: &'static str,
    readback_route: &'static str,
    operator_label: &'static str,
    required_evidence: &'static str,
    current_state: &'static str,
) -> ControlledLiveReadinessDenialReadbackEntry {
    ControlledLiveReadinessDenialReadbackEntry {
        id,
        source_blocker_id,
        layer,
        query_key,
        readback_route,
        operator_label,
        required_evidence,
        current_state,
        queryable: true,
        operator_facing: true,
        blocks_cutover: true,
        operator_recoverable: true,
        waiver_allowed: false,
        acceptance_allowed: false,
        live_mutation_allowed: false,
    }
}

impl ControlledLiveReadinessDenialReadbackIndexSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_recorded: false,
            blocker_waived: false,
            denial_accepted: false,
            readback_persisted: false,
            ledger_written: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            native_post_mutation_performed: false,
            gateway_or_auth_mutated: false,
            telegram_transport_mutated: false,
            channel_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
            rollback_executed: false,
            kill_switch_mutated: false,
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
    fn denial_readback_index_is_ready_but_cutover_blocked() {
        let report = controlled_live_readiness_denial_readback_index_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_audit_ready);
        assert!(report.source_cutover_blocked);
        assert_eq!(report.source_blocker_count, 7);
        assert_eq!(report.index_entry_count, 7);
        assert_eq!(report.queryable_entry_count, 7);
        assert_eq!(report.operator_facing_entry_count, 7);
        assert_eq!(report.readback_route_count, 7);
        assert!(report.readback_index_ready);
        assert!(!report.controlled_live_cutover_ready);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn denial_readback_index_exposes_required_query_keys() {
        let report = controlled_live_readiness_denial_readback_index_report();
        let query_keys = report
            .entries
            .iter()
            .map(|entry| entry.query_key)
            .collect::<Vec<_>>();

        assert!(query_keys.contains(&"controlled_live.blockers.operator_live_approval_missing"));
        assert!(query_keys.contains(&"controlled_live.blockers.fresh_soak_readback_missing"));
        assert!(query_keys.contains(
            &"controlled_live.blockers.gateway_native_telegram_post_boundary_approval_missing"
        ));
        assert!(query_keys.contains(&"controlled_live.blockers.rollback_rehearsal_missing"));
        assert!(query_keys.contains(&"controlled_live.blockers.kill_switch_rehearsal_missing"));
    }

    #[test]
    fn denial_readback_index_does_not_accept_or_waive_blockers() {
        let report = controlled_live_readiness_denial_readback_index_report();

        assert_eq!(report.accepted_denial_count, 0);
        assert_eq!(report.waived_blocker_count, 0);
        assert!(report.entries.iter().all(|entry| entry.blocks_cutover
            && entry.operator_recoverable
            && !entry.waiver_allowed
            && !entry.acceptance_allowed
            && !entry.live_mutation_allowed));
        assert_eq!(
            report.side_effects,
            ControlledLiveReadinessDenialReadbackIndexSideEffects::none()
        );
    }
}
