use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_terminal_closeout_gate: &'static str,
    pub source_closeout_entry_count: usize,
    pub source_closeout_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub readback_entry_count: usize,
    pub readback_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_scope:
        WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackScopePreview,
    pub readback_entries:
        Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackEntryPreview>,
    pub readback_blockers:
        Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub terminal_closeout_visible: bool,
    pub terminal_closeout_recorded: bool,
    pub terminal_closeout_persisted: bool,
    pub terminal_closeout_authoritative: bool,
    pub terminal_closeout_accepted: bool,
    pub readback_visible: bool,
    pub readback_recorded: bool,
    pub readback_persisted: bool,
    pub readback_authoritative: bool,
    pub readback_accepted: bool,
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
    pub ready_for_terminal_no_request_closeout_readback_audit_index: bool,
    pub ready_for_operator_review_request: bool,
    pub ready_for_approval_recording: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackScopePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub readback_mode: &'static str,
    pub stable_readback_key: &'static str,
    pub closeout_visible: bool,
    pub closeout_recorded: bool,
    pub closeout_persisted: bool,
    pub closeout_authoritative: bool,
    pub closeout_accepted: bool,
    pub readback_visible: bool,
    pub readback_recorded: bool,
    pub readback_persisted: bool,
    pub readback_authoritative: bool,
    pub readback_accepted: bool,
    pub terminal_no_request: bool,
    pub operator_review_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackEntryPreview {
    pub id: &'static str,
    pub stable_readback_key: &'static str,
    pub observed_state: &'static str,
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
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackSideEffects
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
    pub terminal_closeout_readback_recorded: bool,
    pub terminal_closeout_readback_persisted: bool,
    pub terminal_closeout_readback_accepted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_report();
    let readback_scope =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_scope();
    let readback_entries =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_entries();
    let readback_blockers =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_blockers();
    let mut required_prior_gates =
        vec![WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_GATE];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_SCHEMA_VERSION,
        preview_mode:
            "operator_review_request_precondition_terminal_no_request_closeout_readback_only",
        source_terminal_closeout_gate: source.gate,
        source_closeout_entry_count: source.closeout_entry_count,
        source_closeout_blocker_count: source.closeout_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        readback_entry_count: readback_entries.len(),
        readback_blocker_count: readback_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_scope,
        readback_entries,
        readback_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_RECOMMENDED_NEXT_GATE,
        terminal_closeout_visible: true,
        terminal_closeout_recorded: false,
        terminal_closeout_persisted: false,
        terminal_closeout_authoritative: false,
        terminal_closeout_accepted: false,
        readback_visible: true,
        readback_recorded: false,
        readback_persisted: false,
        readback_authoritative: false,
        readback_accepted: false,
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
        ready_for_terminal_no_request_closeout_readback_audit_index: true,
        ready_for_operator_review_request: false,
        ready_for_approval_recording: false,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_scope()
-> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackScopePreview {
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackScopePreview {
        id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout",
        readback_mode: "operator_review_request_precondition_terminal_no_request_closeout_readback_only",
        stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout_readback",
        closeout_visible: true,
        closeout_recorded: false,
        closeout_persisted: false,
        closeout_authoritative: false,
        closeout_accepted: false,
        readback_visible: true,
        readback_recorded: false,
        readback_persisted: false,
        readback_authoritative: false,
        readback_accepted: false,
        terminal_no_request: true,
        operator_review_requested: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_entries()
-> Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackEntryPreview> {
    vec![
        readback_entry(
            "terminal_no_request_closeout_decision_readback",
            "terminal_no_request_decision_visible",
        ),
        readback_entry(
            "terminal_no_request_closeout_blocker_chain_readback",
            "terminal_closeout_blocker_chain_visible",
        ),
        readback_entry(
            "terminal_no_request_closeout_prior_chain_readback",
            "terminal_closeout_required_priors_visible",
        ),
        readback_entry(
            "terminal_no_request_closeout_no_request_boundary_readback",
            "operator_review_request_still_absent",
        ),
        readback_entry(
            "terminal_no_request_closeout_no_live_boundary_readback",
            "live_cutover_still_absent",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_blockers()
-> Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackBlockerPreview> {
    vec![
        readback_blocker(
            "terminal_closeout_readback_record_blocked",
            "record_terminal_no_request_closeout_readback",
        ),
        readback_blocker(
            "terminal_closeout_readback_persistence_blocked",
            "persist_terminal_no_request_closeout_readback",
        ),
        readback_blocker(
            "terminal_closeout_record_blocked",
            "record_terminal_no_request_closeout",
        ),
        readback_blocker(
            "terminal_closeout_persistence_blocked",
            "persist_terminal_no_request_closeout",
        ),
        readback_blocker(
            "terminal_closeout_acceptance_blocked",
            "accept_terminal_no_request_closeout",
        ),
        readback_blocker("operator_review_request_blocked", "request_operator_review"),
        readback_blocker(
            "operator_review_request_record_blocked",
            "record_operator_review_request",
        ),
        readback_blocker(
            "operator_review_request_persistence_blocked",
            "persist_operator_review_request",
        ),
        readback_blocker(
            "operator_review_request_acceptance_blocked",
            "accept_operator_review_request",
        ),
        readback_blocker("operator_packet_send_blocked", "send_operator_packet"),
        readback_blocker(
            "operator_packet_acceptance_blocked",
            "accept_operator_packet",
        ),
        readback_blocker("approval_record_blocked", "record_operator_approval"),
        readback_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
        ),
        readback_blocker("feature_flag_enablement_blocked", "enable_feature_flag"),
        readback_blocker("canary_traffic_blocked", "route_canary_traffic"),
        readback_blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
        ),
        readback_blocker(
            "guardrail_enforcement_blocked",
            "enable_guardrail_enforcement",
        ),
        readback_blocker("replay_execution_blocked", "execute_replay"),
        readback_blocker("rollback_execution_blocked", "execute_rollback"),
        readback_blocker(
            "work_graph_projection_persistence_blocked",
            "persist_work_graph_projection",
        ),
        readback_blocker("work_graph_event_record_blocked", "record_work_graph_event"),
        readback_blocker("live_cutover_blocked", "perform_live_cutover"),
    ]
}

impl
    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackSideEffects
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
            terminal_closeout_readback_recorded: false,
            terminal_closeout_readback_persisted: false,
            terminal_closeout_readback_accepted: false,
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

fn readback_entry(
    id: &'static str,
    observed_state: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackEntryPreview {
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackEntryPreview {
        id,
        stable_readback_key: id,
        observed_state,
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

fn readback_blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackBlockerPreview {
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "terminal no-request closeout readback cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_no_request_closeout_readback_derives_from_closeout() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_report();

        assert_eq!(
            report.source_terminal_closeout_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_GATE
        );
        assert_eq!(report.source_closeout_entry_count, 7);
        assert_eq!(report.source_closeout_blocker_count, 21);
        assert_eq!(report.source_required_prior_gate_count, 21);
        assert_eq!(report.readback_entry_count, 5);
        assert_eq!(report.readback_blocker_count, 22);
        assert_eq!(report.required_prior_gate_count, 22);
    }

    #[test]
    fn terminal_no_request_closeout_readback_entries_are_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_report();

        assert!(report.readback_scope.closeout_visible);
        assert!(report.readback_scope.readback_visible);
        assert!(report.readback_scope.terminal_no_request);
        assert!(!report.readback_scope.closeout_recorded);
        assert!(!report.readback_scope.closeout_persisted);
        assert!(!report.readback_scope.closeout_authoritative);
        assert!(!report.readback_scope.closeout_accepted);
        assert!(!report.readback_scope.readback_recorded);
        assert!(!report.readback_scope.readback_persisted);
        assert!(!report.readback_scope.readback_authoritative);
        assert!(!report.readback_scope.readback_accepted);
        assert!(!report.readback_scope.operator_review_requested);
        assert!(report.readback_entries.iter().all(|entry| {
            entry.visible
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
    fn terminal_no_request_closeout_readback_blocks_request_and_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_report();

        assert!(
            report
                .readback_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.terminal_closeout_visible);
        assert!(report.readback_visible);
        assert!(report.terminal_no_request);
        assert!(!report.terminal_closeout_recorded);
        assert!(!report.terminal_closeout_persisted);
        assert!(!report.terminal_closeout_authoritative);
        assert!(!report.terminal_closeout_accepted);
        assert!(!report.readback_recorded);
        assert!(!report.readback_persisted);
        assert!(!report.readback_authoritative);
        assert!(!report.readback_accepted);
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
        assert!(report.ready_for_terminal_no_request_closeout_readback_audit_index);
        assert!(!report.ready_for_operator_review_request);
        assert!(!report.ready_for_approval_recording);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn terminal_no_request_closeout_readback_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackSideEffects::none()
        );
    }
}
