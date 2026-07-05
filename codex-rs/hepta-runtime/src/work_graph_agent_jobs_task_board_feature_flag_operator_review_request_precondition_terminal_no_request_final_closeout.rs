use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutReport {
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
    pub final_closeout_entry_count: usize,
    pub final_closeout_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub final_closeout_scope: WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutScopePreview,
    pub final_closeout_entries:
        Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutEntryPreview>,
    pub final_closeout_blockers:
        Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub terminal_no_request_branch_closed: bool,
    pub final_closeout_visible: bool,
    pub final_closeout_recorded: bool,
    pub final_closeout_persisted: bool,
    pub final_closeout_authoritative: bool,
    pub final_closeout_accepted: bool,
    pub source_audit_index_visible: bool,
    pub source_audit_index_persisted: bool,
    pub source_readback_persisted: bool,
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
    pub ready_for_scheduler_guardrail_blocking_dry_run_entrypoint: bool,
    pub ready_for_work_graph_shadow_event_store_readback: bool,
    pub ready_for_operator_review_request: bool,
    pub ready_for_approval_recording: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutScopePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub closeout_mode: &'static str,
    pub stable_closeout_key: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
    pub accepted: bool,
    pub terminal: bool,
    pub operator_review_requested: bool,
    pub mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutEntryPreview {
    pub id: &'static str,
    pub stable_closeout_key: &'static str,
    pub source_readback_id: &'static str,
    pub closeout_category: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub accepted: bool,
    pub authoritative: bool,
    pub operator_review_requested: bool,
    pub mutation_allowed: bool,
    pub closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutSideEffects
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
    pub final_closeout_recorded: bool,
    pub final_closeout_persisted: bool,
    pub final_closeout_accepted: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_accepted: bool,
    pub terminal_closeout_recorded: bool,
    pub terminal_closeout_persisted: bool,
    pub terminal_closeout_accepted: bool,
    pub terminal_closeout_readback_recorded: bool,
    pub terminal_closeout_readback_persisted: bool,
    pub terminal_closeout_readback_accepted: bool,
    pub non_persistence_readback_recorded: bool,
    pub non_persistence_readback_persisted: bool,
    pub non_persistence_readback_accepted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback_report();
    let final_closeout_scope =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_scope();
    let final_closeout_entries =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_entries();
    let final_closeout_blockers =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_blockers();
    let mut required_prior_gates = vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    ];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_SCHEMA_VERSION,
        preview_mode: "operator_review_request_precondition_terminal_no_request_final_closeout_report_only",
        source_non_persistence_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_readback_blocker_count: source.readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        final_closeout_entry_count: final_closeout_entries.len(),
        final_closeout_blocker_count: final_closeout_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        final_closeout_scope,
        final_closeout_entries,
        final_closeout_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_FINAL_CLOSEOUT_RECOMMENDED_NEXT_GATE,
        terminal_no_request_branch_closed: true,
        final_closeout_visible: true,
        final_closeout_recorded: false,
        final_closeout_persisted: false,
        final_closeout_authoritative: false,
        final_closeout_accepted: false,
        source_audit_index_visible: source.audit_index_visible,
        source_audit_index_persisted: false,
        source_readback_persisted: false,
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
        ready_for_scheduler_guardrail_blocking_dry_run_entrypoint: true,
        ready_for_work_graph_shadow_event_store_readback: true,
        ready_for_operator_review_request: false,
        ready_for_approval_recording: false,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_scope()
-> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutScopePreview {
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutScopePreview {
        id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout_readback_audit_index_non_persistence_readback",
        closeout_mode: "operator_review_request_precondition_terminal_no_request_final_closeout_report_only",
        stable_closeout_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request.final_closeout",
        visible: true,
        recorded: false,
        persisted: false,
        authoritative: false,
        accepted: false,
        terminal: true,
        operator_review_requested: false,
        mutation_allowed: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_entries()
-> Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutEntryPreview> {
    vec![
        final_closeout_entry(
            "terminal_no_request_branch_final_closeout",
            "terminal_no_request.final_closeout.branch_closed",
            "terminal_no_request_closeout_readback_audit_index_non_persistence_readback",
            "terminal_no_request_branch",
        ),
        final_closeout_entry(
            "terminal_no_request_prior_chain_final_closeout",
            "terminal_no_request.final_closeout.required_priors_closed",
            "terminal_no_request_closeout_readback_audit_index_prior_chain_readback",
            "required_prior_chain",
        ),
        final_closeout_entry(
            "terminal_no_request_blocker_chain_final_closeout",
            "terminal_no_request.final_closeout.blockers_closed_visible_only",
            "terminal_no_request_closeout_readback_audit_index_blocker_readback",
            "blocker_chain",
        ),
        final_closeout_entry(
            "terminal_no_request_operator_review_boundary_final_closeout",
            "terminal_no_request.final_closeout.operator_review_request_boundary",
            "terminal_no_request_closeout_readback_audit_index_no_request_boundary_readback",
            "operator_review_request_boundary",
        ),
        final_closeout_entry(
            "terminal_no_request_operator_packet_boundary_final_closeout",
            "terminal_no_request.final_closeout.operator_packet_boundary",
            "operator_packet_send_blocked",
            "operator_packet_boundary",
        ),
        final_closeout_entry(
            "terminal_no_request_approval_boundary_final_closeout",
            "terminal_no_request.final_closeout.approval_boundary",
            "approval_record_blocked",
            "approval_boundary",
        ),
        final_closeout_entry(
            "terminal_no_request_config_flag_traffic_boundary_final_closeout",
            "terminal_no_request.final_closeout.config_flag_traffic_boundary",
            "feature_flag_config_write_blocked",
            "config_flag_traffic_boundary",
        ),
        final_closeout_entry(
            "terminal_no_request_live_boundary_final_closeout",
            "terminal_no_request.final_closeout.live_boundary",
            "live_cutover_blocked",
            "live_cutover_boundary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_blockers()
-> Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutBlockerPreview> {
    vec![
        final_closeout_blocker(
            "final_closeout_record_blocked",
            "record_terminal_no_request_final_closeout",
        ),
        final_closeout_blocker(
            "final_closeout_persistence_blocked",
            "persist_terminal_no_request_final_closeout",
        ),
        final_closeout_blocker(
            "final_closeout_acceptance_blocked",
            "accept_terminal_no_request_final_closeout",
        ),
        final_closeout_blocker(
            "audit_index_readback_persistence_blocked",
            "persist_terminal_no_request_closeout_readback_audit_index_readback",
        ),
        final_closeout_blocker(
            "audit_index_record_blocked",
            "record_terminal_no_request_closeout_readback_audit_index",
        ),
        final_closeout_blocker(
            "audit_index_persistence_blocked",
            "persist_terminal_no_request_closeout_readback_audit_index",
        ),
        final_closeout_blocker(
            "audit_index_acceptance_blocked",
            "accept_terminal_no_request_closeout_readback_audit_index",
        ),
        final_closeout_blocker(
            "terminal_closeout_readback_record_blocked",
            "record_terminal_no_request_closeout_readback",
        ),
        final_closeout_blocker(
            "terminal_closeout_readback_persistence_blocked",
            "persist_terminal_no_request_closeout_readback",
        ),
        final_closeout_blocker(
            "terminal_closeout_record_blocked",
            "record_terminal_no_request_closeout",
        ),
        final_closeout_blocker(
            "terminal_closeout_persistence_blocked",
            "persist_terminal_no_request_closeout",
        ),
        final_closeout_blocker(
            "terminal_closeout_acceptance_blocked",
            "accept_terminal_no_request_closeout",
        ),
        final_closeout_blocker("operator_review_request_blocked", "request_operator_review"),
        final_closeout_blocker(
            "operator_review_request_record_blocked",
            "record_operator_review_request",
        ),
        final_closeout_blocker(
            "operator_review_request_persistence_blocked",
            "persist_operator_review_request",
        ),
        final_closeout_blocker(
            "operator_review_request_acceptance_blocked",
            "accept_operator_review_request",
        ),
        final_closeout_blocker("operator_packet_send_blocked", "send_operator_packet"),
        final_closeout_blocker(
            "operator_packet_acceptance_blocked",
            "accept_operator_packet",
        ),
        final_closeout_blocker("approval_record_blocked", "record_operator_approval"),
        final_closeout_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
        ),
        final_closeout_blocker("feature_flag_enablement_blocked", "enable_feature_flag"),
        final_closeout_blocker("canary_traffic_blocked", "route_canary_traffic"),
        final_closeout_blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
        ),
        final_closeout_blocker(
            "guardrail_enforcement_blocked",
            "enable_guardrail_enforcement",
        ),
        final_closeout_blocker("replay_execution_blocked", "execute_replay"),
        final_closeout_blocker("rollback_execution_blocked", "execute_rollback"),
        final_closeout_blocker(
            "work_graph_projection_persistence_blocked",
            "persist_work_graph_projection",
        ),
        final_closeout_blocker("work_graph_event_record_blocked", "record_work_graph_event"),
        final_closeout_blocker("live_cutover_blocked", "perform_live_cutover"),
    ]
}

impl
    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutSideEffects
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
            final_closeout_recorded: false,
            final_closeout_persisted: false,
            final_closeout_accepted: false,
            audit_index_recorded: false,
            audit_index_persisted: false,
            audit_index_accepted: false,
            terminal_closeout_recorded: false,
            terminal_closeout_persisted: false,
            terminal_closeout_accepted: false,
            terminal_closeout_readback_recorded: false,
            terminal_closeout_readback_persisted: false,
            terminal_closeout_readback_accepted: false,
            non_persistence_readback_recorded: false,
            non_persistence_readback_persisted: false,
            non_persistence_readback_accepted: false,
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

fn final_closeout_entry(
    id: &'static str,
    stable_closeout_key: &'static str,
    source_readback_id: &'static str,
    closeout_category: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutEntryPreview {
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutEntryPreview {
        id,
        stable_closeout_key,
        source_readback_id,
        closeout_category,
        visible: true,
        recorded: false,
        persisted: false,
        accepted: false,
        authoritative: false,
        operator_review_requested: false,
        mutation_allowed: false,
        closed: true,
    }
}

fn final_closeout_blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutBlockerPreview {
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "terminal no-request final closeout cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_no_request_final_closeout_derives_from_non_persistence_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_report();

        assert_eq!(
            report.source_non_persistence_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 5);
        assert_eq!(report.source_readback_blocker_count, 26);
        assert_eq!(report.source_required_prior_gate_count, 24);
        assert_eq!(report.final_closeout_entry_count, 8);
        assert_eq!(report.final_closeout_blocker_count, 29);
        assert_eq!(report.required_prior_gate_count, 25);
    }

    #[test]
    fn terminal_no_request_final_closeout_is_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_report();

        assert!(report.final_closeout_scope.visible);
        assert!(!report.final_closeout_scope.recorded);
        assert!(!report.final_closeout_scope.persisted);
        assert!(!report.final_closeout_scope.authoritative);
        assert!(!report.final_closeout_scope.accepted);
        assert!(report.final_closeout_scope.terminal);
        assert!(!report.final_closeout_scope.operator_review_requested);
        assert!(!report.final_closeout_scope.mutation_allowed);
        assert!(report.final_closeout_entries.iter().all(|entry| {
            entry.visible
                && entry.closed
                && !entry.recorded
                && !entry.persisted
                && !entry.accepted
                && !entry.authoritative
                && !entry.operator_review_requested
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn terminal_no_request_final_closeout_blocks_request_and_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_report();

        assert_eq!(
            report.required_prior_gates[0],
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert!(
            report
                .final_closeout_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.terminal_no_request_branch_closed);
        assert!(report.final_closeout_visible);
        assert!(!report.final_closeout_recorded);
        assert!(!report.final_closeout_persisted);
        assert!(!report.final_closeout_authoritative);
        assert!(!report.final_closeout_accepted);
        assert!(report.source_audit_index_visible);
        assert!(!report.source_audit_index_persisted);
        assert!(!report.source_readback_persisted);
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
        assert!(report.ready_for_scheduler_guardrail_blocking_dry_run_entrypoint);
        assert!(report.ready_for_work_graph_shadow_event_store_readback);
        assert!(!report.ready_for_operator_review_request);
        assert!(!report.ready_for_approval_recording);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn terminal_no_request_final_closeout_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestFinalCloseoutSideEffects::none()
        );
    }
}
