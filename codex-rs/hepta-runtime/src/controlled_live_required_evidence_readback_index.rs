use crate::controlled_live_required_evidence_collection_plan::controlled_live_required_evidence_collection_plan_report;
use serde::Serialize;

pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_INDEX_GATE: &str =
    "controlled_live_required_evidence_readback_index_gate";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_INDEX_SCHEMA_VERSION: &str =
    "controlled_live_required_evidence_readback_index_v1";
pub const CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_INDEX_RECOMMENDED_NEXT_GATE: &str =
    "phase5f_controlled_live_required_evidence_gap_summary_without_acceptance";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceReadbackIndexReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub source_evidence_collection_plan_ready: bool,
    pub source_plan_entry_count: usize,
    pub index_entry_count: usize,
    pub queryable_entry_count: usize,
    pub operator_visible_entry_count: usize,
    pub diffable_entry_count: usize,
    pub fingerprint_count: usize,
    pub evidence_recorded_count: usize,
    pub readback_index_ready: bool,
    pub approval_acceptance_ready: bool,
    pub approval_accepted: bool,
    pub blocker_waived_count: usize,
    pub credential_read_allowed: bool,
    pub evidence_recording_allowed: bool,
    pub evidence_persisted: bool,
    pub controlled_live_cutover_ready: bool,
    pub live_execution_allowed: bool,
    pub entries: Vec<ControlledLiveRequiredEvidenceReadbackIndexEntry>,
    pub recommended_next_gate: &'static str,
    pub side_effects: ControlledLiveRequiredEvidenceReadbackIndexSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceReadbackIndexEntry {
    pub id: &'static str,
    pub source_blocker_id: &'static str,
    pub layer: &'static str,
    pub query_key: &'static str,
    pub readback_route: &'static str,
    pub diff_key: &'static str,
    pub fingerprint: &'static str,
    pub operator_label: &'static str,
    pub required_evidence: &'static str,
    pub evidence_state: &'static str,
    pub queryable: bool,
    pub operator_visible: bool,
    pub diffable: bool,
    pub evidence_recorded: bool,
    pub evidence_recording_allowed: bool,
    pub credential_read_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub blocker_waiver_allowed: bool,
    pub persistence_allowed: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveRequiredEvidenceReadbackIndexSideEffects {
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_recorded: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub blocker_waived: bool,
    pub credential_read: bool,
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

pub fn controlled_live_required_evidence_readback_index_report()
-> ControlledLiveRequiredEvidenceReadbackIndexReport {
    let plan = controlled_live_required_evidence_collection_plan_report();
    let entries = controlled_live_required_evidence_readback_index_entries();
    let queryable_entry_count = entries.iter().filter(|entry| entry.queryable).count();
    let operator_visible_entry_count = entries
        .iter()
        .filter(|entry| entry.operator_visible)
        .count();
    let diffable_entry_count = entries.iter().filter(|entry| entry.diffable).count();
    let fingerprint_count = entries
        .iter()
        .filter(|entry| !entry.fingerprint.is_empty())
        .count();
    let evidence_recorded_count = entries
        .iter()
        .filter(|entry| entry.evidence_recorded)
        .count();
    let blocker_waived_count = entries
        .iter()
        .filter(|entry| entry.blocker_waiver_allowed)
        .count();
    let readback_index_ready = plan.evidence_collection_plan_ready
        && plan.plan_entry_count == 7
        && entries.len() == 7
        && queryable_entry_count == 7
        && operator_visible_entry_count == 7
        && diffable_entry_count == 7
        && fingerprint_count == 7
        && evidence_recorded_count == 0
        && blocker_waived_count == 0
        && entries.iter().all(|entry| {
            entry.evidence_state == "missing"
                && !entry.evidence_recording_allowed
                && !entry.credential_read_allowed
                && !entry.approval_acceptance_allowed
                && !entry.persistence_allowed
                && !entry.live_mutation_allowed
        });

    ControlledLiveRequiredEvidenceReadbackIndexReport {
        runtime: "hepta",
        surface: "controlled_live_required_evidence_readback_index",
        status: if readback_index_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_INDEX_GATE,
        schema_version: CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_INDEX_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        source_evidence_collection_plan_ready: plan.evidence_collection_plan_ready,
        source_plan_entry_count: plan.plan_entry_count,
        index_entry_count: entries.len(),
        queryable_entry_count,
        operator_visible_entry_count,
        diffable_entry_count,
        fingerprint_count,
        evidence_recorded_count,
        readback_index_ready,
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
            CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_INDEX_RECOMMENDED_NEXT_GATE,
        side_effects: ControlledLiveRequiredEvidenceReadbackIndexSideEffects::none(),
    }
}

pub fn controlled_live_required_evidence_readback_index_entries()
-> Vec<ControlledLiveRequiredEvidenceReadbackIndexEntry> {
    controlled_live_required_evidence_collection_plan_report()
        .entries
        .into_iter()
        .map(|entry| ControlledLiveRequiredEvidenceReadbackIndexEntry {
            id: entry.id,
            source_blocker_id: entry.source_blocker_id,
            layer: entry.layer,
            query_key: evidence_query_key(entry.source_blocker_id),
            readback_route: evidence_readback_route(entry.source_blocker_id),
            diff_key: evidence_diff_key(entry.source_blocker_id),
            fingerprint: evidence_fingerprint(entry.source_blocker_id),
            operator_label: entry.operator_label,
            required_evidence: entry.required_evidence,
            evidence_state: entry.current_state,
            queryable: true,
            operator_visible: true,
            diffable: true,
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

fn evidence_query_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "controlled_live.required_evidence.dirty_worktree_boundary",
        "operator_live_approval_missing" => {
            "controlled_live.required_evidence.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.required_evidence.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.required_evidence.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.required_evidence.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.required_evidence.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.required_evidence.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.required_evidence.unknown",
    }
}

fn evidence_readback_route(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live_required_evidence.readback.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live_required_evidence.readback.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live_required_evidence.readback.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live_required_evidence.readback.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live_required_evidence.readback.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live_required_evidence.readback.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live_required_evidence.readback.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live_required_evidence.readback.unknown",
    }
}

fn evidence_diff_key(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => {
            "controlled_live.required_evidence.diff.dirty_worktree_boundary"
        }
        "operator_live_approval_missing" => {
            "controlled_live.required_evidence.diff.operator_live_approval_missing"
        }
        "fresh_soak_readback_missing" => {
            "controlled_live.required_evidence.diff.fresh_soak_readback_missing"
        }
        "credential_boundary_attestation_missing" => {
            "controlled_live.required_evidence.diff.credential_boundary_attestation_missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "controlled_live.required_evidence.diff.gateway_native_telegram_post_boundary_approval_missing"
        }
        "rollback_rehearsal_missing" => {
            "controlled_live.required_evidence.diff.rollback_rehearsal_missing"
        }
        "kill_switch_rehearsal_missing" => {
            "controlled_live.required_evidence.diff.kill_switch_rehearsal_missing"
        }
        _ => "controlled_live.required_evidence.diff.unknown",
    }
}

fn evidence_fingerprint(source_blocker_id: &str) -> &'static str {
    match source_blocker_id {
        "dirty_worktree_boundary" => "required-evidence:fingerprint:dirty-worktree-boundary",
        "operator_live_approval_missing" => {
            "required-evidence:fingerprint:operator-live-approval-missing"
        }
        "fresh_soak_readback_missing" => {
            "required-evidence:fingerprint:fresh-soak-readback-missing"
        }
        "credential_boundary_attestation_missing" => {
            "required-evidence:fingerprint:credential-boundary-attestation-missing"
        }
        "gateway_native_telegram_post_boundary_approval_missing" => {
            "required-evidence:fingerprint:gateway-native-telegram-post-boundary-approval-missing"
        }
        "rollback_rehearsal_missing" => "required-evidence:fingerprint:rollback-rehearsal-missing",
        "kill_switch_rehearsal_missing" => {
            "required-evidence:fingerprint:kill-switch-rehearsal-missing"
        }
        _ => "required-evidence:fingerprint:unknown",
    }
}

impl ControlledLiveRequiredEvidenceReadbackIndexSideEffects {
    pub const fn none() -> Self {
        Self {
            approval_requested: false,
            approval_accepted: false,
            approval_recorded: false,
            evidence_recorded: false,
            evidence_persisted: false,
            blocker_waived: false,
            credential_read: false,
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
    fn evidence_readback_index_is_ready_but_does_not_record_evidence() {
        let report = controlled_live_required_evidence_readback_index_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_evidence_collection_plan_ready);
        assert_eq!(report.source_plan_entry_count, 7);
        assert_eq!(report.index_entry_count, 7);
        assert_eq!(report.queryable_entry_count, 7);
        assert_eq!(report.operator_visible_entry_count, 7);
        assert_eq!(report.diffable_entry_count, 7);
        assert_eq!(report.fingerprint_count, 7);
        assert_eq!(report.evidence_recorded_count, 0);
        assert!(report.readback_index_ready);
        assert!(!report.approval_acceptance_ready);
        assert!(!report.approval_accepted);
        assert_eq!(report.blocker_waived_count, 0);
        assert!(!report.credential_read_allowed);
        assert!(!report.evidence_recording_allowed);
        assert!(!report.evidence_persisted);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn evidence_readback_index_is_queryable_and_diffable_for_all_blockers() {
        let report = controlled_live_required_evidence_readback_index_report();
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
            && entry.diffable
            && !entry.fingerprint.is_empty()
            && entry.evidence_state == "missing"
            && !entry.evidence_recorded
            && !entry.evidence_recording_allowed
            && !entry.credential_read_allowed
            && !entry.approval_acceptance_allowed
            && !entry.blocker_waiver_allowed
            && !entry.persistence_allowed
            && !entry.live_mutation_allowed));
    }

    #[test]
    fn evidence_readback_index_keeps_side_effects_closed() {
        let report = controlled_live_required_evidence_readback_index_report();

        assert_eq!(
            report.side_effects,
            ControlledLiveRequiredEvidenceReadbackIndexSideEffects::none()
        );
    }
}
