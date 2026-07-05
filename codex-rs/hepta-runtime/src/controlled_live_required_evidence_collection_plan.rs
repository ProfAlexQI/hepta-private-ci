use crate::controlled_live_operator_packet_non_send_readback::controlled_live_operator_packet_non_send_readback_report;
use crate::controlled_live_readiness_denial_readback_index::controlled_live_readiness_denial_readback_index_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_COLLECTION_PLAN_GATE: &str =
    "controlled_live_required_evidence_collection_plan_gate";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_COLLECTION_PLAN_SCHEMA_VERSION: &str =
    "controlled_live_required_evidence_collection_plan_v1";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_COLLECTION_PLAN_RECOMMENDED_NEXT_GATE: &str =
    "phase5e_controlled_live_required_evidence_readback_index_without_recording";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceCollectionPlanReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_denial_readback_index_ready: bool,
    pub source_non_send_readback_ready: bool,
    pub source_blocker_count: usize,
    pub plan_entry_count: usize,
    pub queryable_plan_count: usize,
    pub operator_visible_plan_count: usize,
    pub required_evidence_count: usize,
    pub evidence_recorded_count: usize,
    pub evidence_collection_plan_ready: bool,
    pub approval_acceptance_ready: bool,
    pub approval_accepted: bool,
    pub blocker_waived_count: usize,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<ControlledLiveRequiredEvidenceCollectionPlanEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveRequiredEvidenceCollectionPlanSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceCollectionPlanEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub layer: &'static str,
    pub query_key: &'static str,
    pub collection_route: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub collection_mode: &'static str,
    pub current_state: &'static str,
    pub queryable: bool,
    pub operator_visible: bool,
    pub evidence_required: bool,
    pub evidence_recorded: bool,
    pub evidence_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceCollectionPlanSideEffects {
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub blocker_waived: bool,
    pub credential_read: bool,
    pub packet_sent: bool,
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
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn controlled_live_required_evidence_collection_plan_report()
-> ControlledLiveRequiredEvidenceCollectionPlanReport {
    let denial_index = controlled_live_readiness_denial_readback_index_report();
    let non_send_readback = controlled_live_operator_packet_non_send_readback_report();
    let entries = controlled_live_required_evidence_collection_plan_entries();
    let queryable_plan_count = entries.iter().filter(|entry| entry.queryable).count();
    let operator_visible_plan_count = entries
        .iter()
        .filter(|entry| entry.operator_visible)
        .count();
    let required_evidence_count = entries
        .iter()
        .filter(|entry| entry.evidence_required && !entry.required_evidence.is_empty())
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();
    let evidence_collection_plan_ready = denial_index.readback_index_ready
        && non_send_readback.non_send_readback_ready
        && denial_index.index_entry_count == 7
        && entries.len() == 7
        && queryable_plan_count == 7
        && operator_visible_plan_count == 7
        && required_evidence_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.collection_mode == "plan_only_no_recording"
                && entry.current_state == "missing"
                && !entry.evidence_recording_allowed
                && !entry.credential_read_allowed
                && !entry.approval_acceptance_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveRequiredEvidenceCollectionPlanReport {
        runtime: "hepta",
        surface: "controlled_live_required_evidence_collection_plan",
        status: if evidence_collection_plan_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_REQUIRED_EVIDENCE_COLLECTION_PLAN_GATE,
        schema_version: CONTROLLED_LIVE_REQUIRED_EVIDENCE_COLLECTION_PLAN_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_denial_readback_index_ready: denial_index.readback_index_ready,
        source_non_send_readback_ready: non_send_readback.non_send_readback_ready,
        source_blocker_count: denial_index.source_blocker_count,
        plan_entry_count: entries.len(),
        queryable_plan_count,
        operator_visible_plan_count,
        required_evidence_count,
        evidence_recorded_count,
        evidence_collection_plan_ready,
        approval_acceptance_ready: false,
        approval_accepted: false,
        blocker_waived_count,
        credential_read_allowed: false,
        evidence_recording_allowed: false,
        evidence_persisted: false,
        controlled_live_cutover_ready: false,
        live_execution_allowed: false,
        entries,
        recommended_next_gate:
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_COLLECTION_PLAN_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveRequiredEvidenceCollectionPlanSideEffects::none(),
    }
}

pub fn controlled_live_required_evidence_collection_plan_entries()
-> Vec<ControlledLiveRequiredEvidenceCollectionPlanEntry> {
    controlled_live_readiness_denial_readback_index_report()
        .entries
        .into_iter()
        .map(|entry| ControlledLiveRequiredEvidenceCollectionPlanEntry {
            id: entry.id,
            source_blocker_id: entry.source_blocker_id,
            layer: entry.layer,
            query_key: entry.query_key,
            collection_route: evidence_collection_route(entry.source_blocker_id),
            operator_label: entry.operator_label,
            required_evidence: entry.required_evidence,
            collection_mode: "plan_only_no_recording",
            current_state: entry.current_state,
            queryable: true,
            operator_visible: true,
            evidence_required: true,
            evidence_recorded: false,
            evidence_recording_allowed: false,
            credential_read_allowed: false,
            approval_acceptance_allowed: false,
            blocker_waiver_allowed: false,
            persistence_allowed: false,
            live_mutation_allowed: false,
        })
        .collect()
}

fn evidence_collection_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live_required_evidence.collection.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live_required_evidence.collection.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live_required_evidence.collection.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live_required_evidence.collection.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live_required_evidence.collection.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live_required_evidence.collection.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live_required_evidence.collection.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live_required_evidence.collection.unknown",
    }
}

impl ControlledLiveRequiredEvidenceCollectionPlanSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            evidence_recorded: false,
            evidence_persisted: false,
            blocker_waived: false,
            credential_read: false,
            packet_sent: false,
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
            replay_executed: false,
            rollback_executed: false,
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
    fn evidence_collection_plan_is_ready_but_does_not_record_evidence() {
        let report = controlled_live_required_evidence_collection_plan_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_denial_readback_index_ready);
        assert!(report.source_non_send_readback_ready);
        assert_eq!(report.source_blocker_count, 7);
        assert_eq!(report.plan_entry_count, 7);
        assert_eq!(report.queryable_plan_count, 7);
        assert_eq!(report.operator_visible_plan_count, 7);
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.evidence_collection_plan_ready);
        assert!(!report.approval_acceptance_ready);
        assert!(!report.approval_accepted);
        assert_eq!(report.blocker_waived_count, 0);
        assert!(!report.credential_read_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.evidence_persisted);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn evidence_collection_plan_covers_all_cutover_blockers() {
        let report = controlled_live_required_evidence_collection_plan_report();
        let blocker_ids = report
            .entries
            .iter()
            .map(|entry| entry.source_blocker_id)
            .collect::<Vec<_>>();

        assert!(blocker_ids.contains(&"dirty_worktree_boundary"));
        assert!(blocker_ids.contains(&"operator_live_approval_missing"));
        assert!(blocker_ids.contains(&"fresh_soak_readback_missing"));
        assert!(blocker_ids.contains(&"credential_boundary_attestation_missing"));
        assert!(blocker_ids.contains(&"gateway_native_telegram_post_boundary_approval_missing"));
        assert!(blocker_ids.contains(&"rollback_rehearsal_missing"));
        assert!(blocker_ids.contains(&"kill_switch_rehearsal_missing"));
        assert!(report.entries.iter().all(|entry| entry.queryable
            && entry.operator_visible
            && entry.evidence_required
            && !entry.evidence_recorded
            && !entry.evidence_recording_allowed
            && !entry.credential_read_allowed
            && !entry.approval_acceptance_allowed
            && !entry.blocker_waiver_allowed
            && !entry.persistence_allowed
            && !entry.live_mutation_allowed));
    }

    #[test]
    fn evidence_collection_plan_keeps_side_effects_closed() {
        let report = controlled_live_required_evidence_collection_plan_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveRequiredEvidenceCollectionPlanSideEffects::none()
        );
    }
}
