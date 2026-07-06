use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only::hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_report;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketNonSendReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_operator_packet_gate: &'static str,
    pub source_operator_packet_section_count: usize,
    pub source_review_item_count: usize,
    pub source_evidence_ref_count: usize,
    pub source_blocked_action_count: usize,
    pub readback_entry_count: usize,
    pub readback_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_scope: WorkGraphFeatureFlagOperatorPacketNonSendReadbackScopePreview,
    pub readback_entries: Vec<WorkGraphFeatureFlagOperatorPacketNonSendReadbackEntryPreview>,
    pub readback_blockers: Vec<WorkGraphFeatureFlagOperatorPacketNonSendReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub operator_packet_visible: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_recorded: bool,
    pub operator_packet_persisted: bool,
    pub operator_packet_accepted: bool,
    pub operator_packet_authoritative: bool,
    pub operator_packet_authorizes_config_write: bool,
    pub operator_packet_authorizes_canary_traffic: bool,
    pub operator_packet_authorizes_live_cutover: bool,
    pub approval_recorded: bool,
    pub approval_acceptance_allowed: bool,
    pub readback_persisted: bool,
    pub ready_for_rollback_replay_pre_enable_blocker_matrix: bool,
    pub ready_for_operator_packet_acceptance: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketNonSendReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagOperatorPacketNonSendReadbackScopePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub readback_mode: &'static str,
    pub stable_readback_key: &'static str,
    pub packet_visible: bool,
    pub packet_sent: bool,
    pub packet_recorded: bool,
    pub packet_persisted: bool,
    pub packet_accepted: bool,
    pub packet_authoritative: bool,
    pub readback_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagOperatorPacketNonSendReadbackEntryPreview {
    pub id: &'static str,
    pub stable_readback_key: &'static str,
    pub observed_state: &'static str,
    pub visible: bool,
    pub sent: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub accepted: bool,
    pub authoritative: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagOperatorPacketNonSendReadbackBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketNonSendReadbackSideEffects {
    pub filesystem_written: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_recorded: bool,
    pub operator_packet_persisted: bool,
    pub operator_packet_accepted: bool,
    pub approval_recorded: bool,
    pub readback_persisted: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub non_blocking_canary_enabled: bool,
    pub live_cutover_enabled: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub config_digest_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketNonSendReadbackReport {
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_report();
    let readback_scope =
        work_graph_agent_jobs_task_board_feature_flag_operator_packet_readback_scope();
    let readback_entries =
        work_graph_agent_jobs_task_board_feature_flag_operator_packet_readback_entries();
    let readback_blockers =
        work_graph_agent_jobs_task_board_feature_flag_operator_packet_readback_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_operator_packet_readback_required_prior_gates(
        );

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketNonSendReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_SCHEMA_VERSION,
        preview_mode: "operator_packet_non_send_readback_only_no_send_no_record_no_persistence",
        source_operator_packet_gate: source.gate,
        source_operator_packet_section_count: source.operator_packet_section_count,
        source_review_item_count: source.review_item_count,
        source_evidence_ref_count: source.evidence_ref_count,
        source_blocked_action_count: source.blocked_action_count,
        readback_entry_count: readback_entries.len(),
        readback_blocker_count: readback_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_scope,
        readback_entries,
        readback_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_RECOMMENDED_NEXT_GATE,
        operator_packet_visible: true,
        operator_packet_sent: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        operator_packet_authoritative: false,
        operator_packet_authorizes_config_write: false,
        operator_packet_authorizes_canary_traffic: false,
        operator_packet_authorizes_live_cutover: false,
        approval_recorded: false,
        approval_acceptance_allowed: false,
        readback_persisted: false,
        ready_for_rollback_replay_pre_enable_blocker_matrix: true,
        ready_for_operator_packet_acceptance: false,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketNonSendReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_packet_readback_scope()
-> WorkGraphFeatureFlagOperatorPacketNonSendReadbackScopePreview {
    WorkGraphFeatureFlagOperatorPacketNonSendReadbackScopePreview {
        id: "agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_scope",
        source_surface_id: "work_graph_agent_jobs_task_board_feature_flag_operator_packet",
        readback_mode: "operator_packet_non_send_readback_only",
        stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.operator_packet.non_send_readback",
        packet_visible: true,
        packet_sent: false,
        packet_recorded: false,
        packet_persisted: false,
        packet_accepted: false,
        packet_authoritative: false,
        readback_persisted: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_packet_readback_entries()
-> Vec<WorkGraphFeatureFlagOperatorPacketNonSendReadbackEntryPreview> {
    vec![
        readback_entry(
            "operator_packet_surface_readback",
            "operator_packet_visible_unsent_unrecorded_unpersisted",
            "packet_visible_without_send_record_persist_or_acceptance",
            true,
        ),
        readback_entry(
            "operator_packet_review_state_readback",
            "operator_packet_pending_review_non_authoritative",
            "review_items_pending_without_authorization",
            true,
        ),
        readback_entry(
            "operator_packet_evidence_ref_readback",
            "operator_packet_evidence_redacted_unpersisted",
            "evidence_refs_visible_redacted_unpersisted",
            true,
        ),
        readback_entry(
            "operator_packet_blocked_action_readback",
            "operator_packet_blocked_actions_still_blocked",
            "config_write_enablement_traffic_and_cutover_blocked",
            true,
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_packet_readback_blockers()
-> Vec<WorkGraphFeatureFlagOperatorPacketNonSendReadbackBlockerPreview> {
    vec![
        readback_blocker(
            "operator_packet_send_blocked",
            "send_operator_packet",
            "non-send readback cannot deliver or request approval",
        ),
        readback_blocker(
            "operator_packet_record_blocked",
            "record_operator_packet",
            "operator packet readback is not an acceptance record",
        ),
        readback_blocker(
            "operator_packet_persistence_blocked",
            "persist_operator_packet",
            "readback remains stdout/report-only and unpersisted",
        ),
        readback_blocker(
            "operator_packet_acceptance_blocked",
            "accept_operator_packet",
            "no approval acceptance is allowed by non-send readback",
        ),
        readback_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
            "config write requires explicit future approval beyond readback",
        ),
        readback_blocker(
            "feature_flag_enablement_blocked",
            "enable_feature_flag",
            "feature flags remain current off after readback",
        ),
        readback_blocker(
            "canary_traffic_blocked",
            "route_canary_traffic",
            "canary traffic stays 0ppm in report-only readback",
        ),
        readback_blocker(
            "live_cutover_blocked",
            "perform_live_cutover",
            "live cutover remains outside non-send readback",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_packet_readback_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
        WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketNonSendReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            operator_packet_sent: false,
            operator_packet_recorded: false,
            operator_packet_persisted: false,
            operator_packet_accepted: false,
            approval_recorded: false,
            readback_persisted: false,
            config_written: false,
            feature_flag_mutated: false,
            non_blocking_canary_enabled: false,
            live_cutover_enabled: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            projection_index_persisted: false,
            config_digest_persisted: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            replay_executed: false,
            rollback_executed: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn readback_entry(
    id: &'static str,
    stable_readback_key: &'static str,
    observed_state: &'static str,
    visible: bool,
) -> WorkGraphFeatureFlagOperatorPacketNonSendReadbackEntryPreview {
    WorkGraphFeatureFlagOperatorPacketNonSendReadbackEntryPreview {
        id,
        stable_readback_key,
        observed_state,
        visible,
        sent: false,
        recorded: false,
        persisted: false,
        accepted: false,
        authoritative: false,
        mutation_allowed: false,
        ready: true,
    }
}

fn readback_blocker(
    id: &'static str,
    blocked_action: &'static str,
    reason: &'static str,
) -> WorkGraphFeatureFlagOperatorPacketNonSendReadbackBlockerPreview {
    WorkGraphFeatureFlagOperatorPacketNonSendReadbackBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_packet_non_send_readback_keeps_source_packet_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_report(
            );

        assert_eq!(
            report.source_operator_packet_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE
        );
        assert_eq!(report.source_operator_packet_section_count, 5);
        assert_eq!(report.source_review_item_count, 2);
        assert_eq!(report.source_evidence_ref_count, 5);
        assert_eq!(report.source_blocked_action_count, 6);
        assert!(report.operator_packet_visible);
        assert!(!report.operator_packet_sent);
        assert!(!report.operator_packet_recorded);
        assert!(!report.operator_packet_persisted);
        assert!(!report.operator_packet_accepted);
        assert!(!report.operator_packet_authoritative);
        assert!(report.ready_for_rollback_replay_pre_enable_blocker_matrix);
    }

    #[test]
    fn operator_packet_non_send_readback_entries_are_non_authoritative() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_report(
            );

        assert_eq!(report.readback_entry_count, 4);
        assert_eq!(
            report.readback_scope.readback_mode,
            "operator_packet_non_send_readback_only"
        );
        assert!(report.readback_scope.packet_visible);
        assert!(!report.readback_scope.packet_sent);
        assert!(!report.readback_scope.packet_recorded);
        assert!(!report.readback_scope.packet_persisted);
        assert!(!report.readback_scope.packet_accepted);
        assert!(!report.readback_scope.packet_authoritative);
        assert!(!report.readback_scope.readback_persisted);
        assert!(report.readback_entries.iter().all(|entry| {
            entry.visible
                && entry.ready
                && !entry.sent
                && !entry.recorded
                && !entry.persisted
                && !entry.accepted
                && !entry.authoritative
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn operator_packet_non_send_readback_blocks_enablement_and_requires_priors() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_report(
            );

        assert_eq!(report.readback_blocker_count, 8);
        assert!(
            report
                .readback_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
                WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
            ]
        );
        assert_eq!(report.required_prior_gate_count, 7);
        assert!(!report.operator_packet_authorizes_config_write);
        assert!(!report.operator_packet_authorizes_canary_traffic);
        assert!(!report.operator_packet_authorizes_live_cutover);
        assert!(!report.approval_recorded);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.ready_for_operator_packet_acceptance);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn operator_packet_non_send_readback_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_report(
            );

        assert!(!report.readback_persisted);
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorPacketNonSendReadbackSideEffects::none()
        );
    }
}
