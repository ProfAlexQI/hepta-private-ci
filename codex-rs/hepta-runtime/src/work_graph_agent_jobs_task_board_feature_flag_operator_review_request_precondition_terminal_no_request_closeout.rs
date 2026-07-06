use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_non_request_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_NON_REQUEST_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_NON_REQUEST_READBACK_AUDIT_INDEX_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_NON_REQUEST_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_report,
};
use crate::work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_non_persistence_readback_gate: &'static str,
    pub source_readback_entry_count: usize,
    pub source_readback_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub closeout_entry_count: usize,
    pub closeout_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub closeout_scope: WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutScopePreview,
    pub closeout_entries: Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutEntryPreview>,
    pub closeout_blockers:
        Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub terminal_closeout_visible: bool,
    pub terminal_closeout_recorded: bool,
    pub terminal_closeout_persisted: bool,
    pub terminal_closeout_authoritative: bool,
    pub terminal_closeout_accepted: bool,
    pub terminal_no_request: bool,
    pub operator_review_request_allowed: bool,
    pub operator_review_requested: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_acceptance_allowed: bool,
    pub approval_recording_allowed: bool,
    pub config_write_allowed: bool,
    pub feature_flag_enablement_allowed: bool,
    pub canary_traffic_allowed: bool,
    pub scheduler_enforcement_allowed: bool,
    pub guardrail_enforcement_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub work_graph_persistence_allowed: bool,
    pub live_cutover_allowed: bool,
    pub ready_for_terminal_no_request_closeout_readback: bool,
    pub ready_for_operator_review_request: bool,
    pub ready_for_approval_recording: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutScopePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub closeout_mode: &'static str,
    pub stable_closeout_key: &'static str,
    pub closeout_visible: bool,
    pub closeout_recorded: bool,
    pub closeout_persisted: bool,
    pub closeout_authoritative: bool,
    pub closeout_accepted: bool,
    pub terminal_no_request: bool,
    pub operator_review_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutEntryPreview {
    pub id: &'static str,
    pub stable_closeout_key: &'static str,
    pub closeout_category: &'static str,
    pub terminal: bool,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub accepted: bool,
    pub authoritative: bool,
    pub operator_review_requested: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutSideEffects
{
    pub filesystem_written: bool,
    pub operator_review_requested: bool,
    pub operator_review_request_recorded: bool,
    pub operator_review_request_persisted: bool,
    pub operator_review_request_accepted: bool,
    pub operator_packet_sent: bool,
    pub operator_packet_recorded: bool,
    pub operator_packet_persisted: bool,
    pub operator_packet_accepted: bool,
    pub approval_recorded: bool,
    pub terminal_closeout_recorded: bool,
    pub terminal_closeout_persisted: bool,
    pub terminal_closeout_accepted: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub non_blocking_canary_enabled: bool,
    pub canary_traffic_routed: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_report();
    let closeout_scope =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_scope();
    let closeout_entries =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_entries();
    let closeout_blockers =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_required_prior_gates();

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_SCHEMA_VERSION,
        preview_mode: "operator_review_request_precondition_terminal_no_request_closeout_report_only",
        source_non_persistence_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_readback_blocker_count: source.readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        closeout_entry_count: closeout_entries.len(),
        closeout_blocker_count: closeout_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        closeout_scope,
        closeout_entries,
        closeout_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_RECOMMENDED_NEXT_GATE,
        terminal_closeout_visible: true,
        terminal_closeout_recorded: false,
        terminal_closeout_persisted: false,
        terminal_closeout_authoritative: false,
        terminal_closeout_accepted: false,
        terminal_no_request: true,
        operator_review_request_allowed: false,
        operator_review_requested: false,
        operator_packet_send_allowed: false,
        operator_packet_acceptance_allowed: false,
        approval_recording_allowed: false,
        config_write_allowed: false,
        feature_flag_enablement_allowed: false,
        canary_traffic_allowed: false,
        scheduler_enforcement_allowed: false,
        guardrail_enforcement_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        work_graph_persistence_allowed: false,
        live_cutover_allowed: false,
        ready_for_terminal_no_request_closeout_readback: true,
        ready_for_operator_review_request: false,
        ready_for_approval_recording: false,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_scope()
-> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutScopePreview {
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutScopePreview {
        id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition.denial_audit_index_non_persistence_readback",
        closeout_mode: "operator_review_request_precondition_terminal_no_request_closeout_report_only",
        stable_closeout_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout",
        closeout_visible: true,
        closeout_recorded: false,
        closeout_persisted: false,
        closeout_authoritative: false,
        closeout_accepted: false,
        terminal_no_request: true,
        operator_review_requested: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_entries()
-> Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutEntryPreview> {
    vec![
        closeout_entry(
            "terminal_no_request_decision_closeout",
            "no_request_decision",
        ),
        closeout_entry(
            "terminal_denial_readback_chain_closeout",
            "denial_readback_chain",
        ),
        closeout_entry("terminal_audit_index_chain_closeout", "audit_index_chain"),
        closeout_entry(
            "terminal_no_operator_packet_closeout",
            "operator_packet_boundary",
        ),
        closeout_entry(
            "terminal_no_approval_config_flag_traffic_closeout",
            "approval_config_flag_traffic_boundary",
        ),
        closeout_entry(
            "terminal_no_persistence_replay_rollback_closeout",
            "persistence_replay_rollback_boundary",
        ),
        closeout_entry("terminal_no_live_cutover_closeout", "live_cutover_boundary"),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_blockers()
-> Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutBlockerPreview> {
    vec![
        closeout_blocker(
            "terminal_closeout_readback_persistence_blocked",
            "persist_terminal_no_request_closeout_readback",
        ),
        closeout_blocker(
            "terminal_closeout_record_blocked",
            "record_terminal_no_request_closeout",
        ),
        closeout_blocker(
            "terminal_closeout_persistence_blocked",
            "persist_terminal_no_request_closeout",
        ),
        closeout_blocker(
            "terminal_closeout_acceptance_blocked",
            "accept_terminal_no_request_closeout",
        ),
        closeout_blocker("operator_review_request_blocked", "request_operator_review"),
        closeout_blocker(
            "operator_review_request_record_blocked",
            "record_operator_review_request",
        ),
        closeout_blocker(
            "operator_review_request_persistence_blocked",
            "persist_operator_review_request",
        ),
        closeout_blocker(
            "operator_review_request_acceptance_blocked",
            "accept_operator_review_request",
        ),
        closeout_blocker("operator_packet_send_blocked", "send_operator_packet"),
        closeout_blocker(
            "operator_packet_acceptance_blocked",
            "accept_operator_packet",
        ),
        closeout_blocker("approval_record_blocked", "record_operator_approval"),
        closeout_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
        ),
        closeout_blocker("feature_flag_enablement_blocked", "enable_feature_flag"),
        closeout_blocker("canary_traffic_blocked", "route_canary_traffic"),
        closeout_blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
        ),
        closeout_blocker(
            "guardrail_enforcement_blocked",
            "enable_guardrail_enforcement",
        ),
        closeout_blocker("replay_execution_blocked", "execute_replay"),
        closeout_blocker("rollback_execution_blocked", "execute_rollback"),
        closeout_blocker(
            "work_graph_projection_persistence_blocked",
            "persist_work_graph_projection",
        ),
        closeout_blocker("work_graph_event_record_blocked", "record_work_graph_event"),
        closeout_blocker("live_cutover_blocked", "perform_live_cutover"),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_NON_REQUEST_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_NON_REQUEST_READBACK_AUDIT_INDEX_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_NON_REQUEST_READBACK_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
        WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    ]
}

impl
    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutSideEffects
{
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            operator_review_requested: false,
            operator_review_request_recorded: false,
            operator_review_request_persisted: false,
            operator_review_request_accepted: false,
            operator_packet_sent: false,
            operator_packet_recorded: false,
            operator_packet_persisted: false,
            operator_packet_accepted: false,
            approval_recorded: false,
            terminal_closeout_recorded: false,
            terminal_closeout_persisted: false,
            terminal_closeout_accepted: false,
            config_written: false,
            feature_flag_mutated: false,
            non_blocking_canary_enabled: false,
            canary_traffic_routed: false,
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

fn closeout_entry(
    id: &'static str,
    closeout_category: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutEntryPreview {
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutEntryPreview {
        id,
        stable_closeout_key: id,
        closeout_category,
        terminal: true,
        visible: true,
        recorded: false,
        persisted: false,
        accepted: false,
        authoritative: false,
        operator_review_requested: false,
        mutation_allowed: false,
        ready: true,
    }
}

fn closeout_blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutBlockerPreview {
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "terminal no-request closeout cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_no_request_closeout_derives_from_non_persistence_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_report();

        assert_eq!(
            report.source_non_persistence_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 5);
        assert_eq!(report.source_readback_blocker_count, 20);
        assert_eq!(report.source_required_prior_gate_count, 20);
        assert_eq!(report.closeout_entry_count, 7);
        assert_eq!(report.closeout_blocker_count, 21);
        assert_eq!(report.required_prior_gate_count, 21);
    }

    #[test]
    fn terminal_no_request_closeout_entries_are_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_report();

        assert_eq!(
            report.closeout_scope.closeout_mode,
            "operator_review_request_precondition_terminal_no_request_closeout_report_only"
        );
        assert!(report.closeout_scope.closeout_visible);
        assert!(report.closeout_scope.terminal_no_request);
        assert!(!report.closeout_scope.closeout_recorded);
        assert!(!report.closeout_scope.closeout_persisted);
        assert!(!report.closeout_scope.closeout_authoritative);
        assert!(!report.closeout_scope.closeout_accepted);
        assert!(!report.closeout_scope.operator_review_requested);
        assert!(report.closeout_entries.iter().all(|entry| {
            entry.terminal
                && entry.visible
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.accepted
                && !entry.authoritative
                && !entry.operator_review_requested
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn terminal_no_request_closeout_blocks_request_and_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_report();

        assert!(
            report
                .closeout_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.terminal_closeout_visible);
        assert!(report.terminal_no_request);
        assert!(!report.terminal_closeout_recorded);
        assert!(!report.terminal_closeout_persisted);
        assert!(!report.terminal_closeout_authoritative);
        assert!(!report.terminal_closeout_accepted);
        assert!(!report.operator_review_request_allowed);
        assert!(!report.operator_review_requested);
        assert!(!report.operator_packet_send_allowed);
        assert!(!report.operator_packet_acceptance_allowed);
        assert!(!report.approval_recording_allowed);
        assert!(!report.config_write_allowed);
        assert!(!report.feature_flag_enablement_allowed);
        assert!(!report.canary_traffic_allowed);
        assert!(!report.scheduler_enforcement_allowed);
        assert!(!report.guardrail_enforcement_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.work_graph_persistence_allowed);
        assert!(!report.live_cutover_allowed);
        assert!(report.ready_for_terminal_no_request_closeout_readback);
        assert!(!report.ready_for_operator_review_request);
        assert!(!report.ready_for_approval_recording);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn terminal_no_request_closeout_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutSideEffects::none()
        );
    }
}
