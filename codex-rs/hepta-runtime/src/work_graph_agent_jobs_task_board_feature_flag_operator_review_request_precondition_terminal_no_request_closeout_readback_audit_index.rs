use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_GATE,
    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackSideEffects,
    hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_report,
};

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_non_persistence_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_terminal_closeout_readback_gate: &'static str,
    pub source_readback_entry_count: usize,
    pub source_readback_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub source_terminal_closeout_readback_preconditions_complete: bool,
    pub source_terminal_closeout_readback_no_request_confirmed: bool,
    pub source_terminal_closeout_readback_no_authorization_confirmed: bool,
    pub source_terminal_closeout_readback_ready: bool,
    pub audit_index_entry_count: usize,
    pub audit_index_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub audit_index_scope:
        WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexScopePreview,
    pub audit_index_entries:
        Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexEntryPreview>,
    pub audit_index_blockers:
        Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub audit_index_scope_report_only_complete: bool,
    pub audit_index_entries_report_only_complete: bool,
    pub audit_index_blockers_complete: bool,
    pub terminal_no_request_closeout_readback_audit_index_preconditions_complete: bool,
    pub audit_index_visible: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_authoritative: bool,
    pub audit_index_accepted: bool,
    pub terminal_closeout_readback_visible: bool,
    pub terminal_closeout_readback_persisted: bool,
    pub terminal_no_request: bool,
    pub operator_review_request_allowed: bool,
    pub operator_review_requested: bool,
    pub operator_packet_send_allowed: bool,
    pub operator_packet_acceptance_allowed: bool,
    pub approval_recording_allowed: bool,
    pub audit_index_authorizes_operator_review_request: bool,
    pub audit_index_authorizes_operator_packet_send: bool,
    pub audit_index_authorizes_approval_recording: bool,
    pub audit_index_authorizes_config_write: bool,
    pub audit_index_authorizes_feature_flag_enablement: bool,
    pub audit_index_authorizes_canary_traffic: bool,
    pub audit_index_authorizes_scheduler_enforcement: bool,
    pub audit_index_authorizes_guardrail_enforcement: bool,
    pub audit_index_authorizes_replay_execution: bool,
    pub audit_index_authorizes_rollback_execution: bool,
    pub audit_index_authorizes_work_graph_persistence: bool,
    pub audit_index_authorizes_live_cutover: bool,
    pub ready_for_non_persistence_readback: bool,
    pub ready_for_operator_review_request: bool,
    pub ready_for_approval_recording: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexScopePreview
{
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub index_mode: &'static str,
    pub stable_index_key: &'static str,
    pub index_visible: bool,
    pub index_recorded: bool,
    pub index_persisted: bool,
    pub index_authoritative: bool,
    pub index_accepted: bool,
    pub operator_review_requested: bool,
    pub acceptance_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexEntryPreview
{
    pub id: &'static str,
    pub stable_index_key: &'static str,
    pub source_readback_id: &'static str,
    pub audit_category: &'static str,
    pub indexed: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
    pub operator_review_requested: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexBlockerPreview
{
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexSideEffects
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
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_accepted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_report();
    let audit_index_scope =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_scope();
    let audit_index_entries =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_entries();
    let audit_index_blockers =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_blockers();
    let mut required_prior_gates =
        vec![WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_GATE];
    required_prior_gates.extend(source.required_prior_gates.iter().copied());
    let source_terminal_closeout_readback_no_request_confirmed = source.terminal_closeout_visible
        && !source.terminal_closeout_recorded
        && !source.terminal_closeout_persisted
        && !source.terminal_closeout_authoritative
        && !source.terminal_closeout_accepted
        && source.readback_visible
        && !source.readback_recorded
        && !source.readback_persisted
        && !source.readback_authoritative
        && !source.readback_accepted
        && source.terminal_no_request
        && !source.operator_review_request_allowed
        && !source.operator_review_requested
        && !source.operator_packet_send_allowed
        && !source.operator_packet_acceptance_allowed
        && !source.approval_recording_allowed
        && source.ready_for_terminal_no_request_closeout_readback_audit_index
        && source.side_effects
            == WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackSideEffects::none();
    let source_terminal_closeout_readback_no_authorization_confirmed = !source
        .operator_review_request_allowed
        && !source.operator_review_requested
        && !source.operator_packet_send_allowed
        && !source.operator_packet_acceptance_allowed
        && !source.approval_recording_allowed
        && !source.config_write_allowed
        && !source.feature_flag_enablement_allowed
        && !source.canary_traffic_allowed
        && !source.scheduler_enforcement_allowed
        && !source.guardrail_enforcement_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.work_graph_persistence_allowed
        && !source.live_cutover_allowed
        && !source.ready_for_operator_review_request
        && !source.ready_for_approval_recording
        && !source.ready_for_feature_flag_config_write
        && !source.ready_for_feature_flag_enablement
        && !source.ready_for_canary_traffic
        && !source.ready_for_live_cutover;
    let source_terminal_closeout_readback_ready = source.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_GATE
        && source.terminal_no_request_closeout_readback_preconditions_complete
        && source.ready_for_terminal_no_request_closeout_readback_audit_index
        && source_terminal_closeout_readback_no_request_confirmed
        && source_terminal_closeout_readback_no_authorization_confirmed;
    let audit_index_scope_report_only_complete = audit_index_scope.index_visible
        && !audit_index_scope.index_recorded
        && !audit_index_scope.index_persisted
        && !audit_index_scope.index_authoritative
        && !audit_index_scope.index_accepted
        && !audit_index_scope.operator_review_requested
        && !audit_index_scope.acceptance_allowed;
    let audit_index_entries_report_only_complete = !audit_index_entries.is_empty()
        && audit_index_entries.iter().all(|entry| {
            entry.indexed
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.operator_review_requested
                && !entry.mutation_allowed
        });
    let audit_index_blockers_complete = !audit_index_blockers.is_empty()
        && audit_index_blockers.iter().all(|blocker| blocker.blocked);
    let terminal_no_request_closeout_readback_audit_index_preconditions_complete =
        source_terminal_closeout_readback_ready
            && audit_index_scope_report_only_complete
            && audit_index_entries_report_only_complete
            && audit_index_blockers_complete;

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_SCHEMA_VERSION,
        preview_mode:
            "operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_report_only",
        source_terminal_closeout_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_readback_blocker_count: source.readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        source_terminal_closeout_readback_preconditions_complete: source
            .terminal_no_request_closeout_readback_preconditions_complete,
        source_terminal_closeout_readback_no_request_confirmed,
        source_terminal_closeout_readback_no_authorization_confirmed,
        source_terminal_closeout_readback_ready,
        audit_index_entry_count: audit_index_entries.len(),
        audit_index_blocker_count: audit_index_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        audit_index_scope,
        audit_index_entries,
        audit_index_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_AUDIT_INDEX_RECOMMENDED_NEXT_GATE,
        audit_index_scope_report_only_complete,
        audit_index_entries_report_only_complete,
        audit_index_blockers_complete,
        terminal_no_request_closeout_readback_audit_index_preconditions_complete,
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_accepted: false,
        terminal_closeout_readback_visible: source.readback_visible,
        terminal_closeout_readback_persisted: false,
        terminal_no_request: true,
        operator_review_request_allowed: false,
        operator_review_requested: false,
        operator_packet_send_allowed: false,
        operator_packet_acceptance_allowed: false,
        approval_recording_allowed: false,
        audit_index_authorizes_operator_review_request: false,
        audit_index_authorizes_operator_packet_send: false,
        audit_index_authorizes_approval_recording: false,
        audit_index_authorizes_config_write: false,
        audit_index_authorizes_feature_flag_enablement: false,
        audit_index_authorizes_canary_traffic: false,
        audit_index_authorizes_scheduler_enforcement: false,
        audit_index_authorizes_guardrail_enforcement: false,
        audit_index_authorizes_replay_execution: false,
        audit_index_authorizes_rollback_execution: false,
        audit_index_authorizes_work_graph_persistence: false,
        audit_index_authorizes_live_cutover: false,
        ready_for_non_persistence_readback:
            terminal_no_request_closeout_readback_audit_index_preconditions_complete,
        ready_for_operator_review_request: false,
        ready_for_approval_recording: false,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_scope()
-> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexScopePreview
{
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexScopePreview {
        id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout_readback",
        index_mode:
            "operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_report_only",
        stable_index_key:
            "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.terminal_no_request_closeout_readback.audit_index",
        index_visible: true,
        index_recorded: false,
        index_persisted: false,
        index_authoritative: false,
        index_accepted: false,
        operator_review_requested: false,
        acceptance_allowed: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_entries()
-> Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexEntryPreview>{
    vec![
        audit_index_entry(
            "terminal_no_request_closeout_decision_audit_index",
            "terminal_no_request_closeout_readback_audit_index.decision",
            "terminal_no_request_closeout_decision_readback",
            "terminal_no_request_decision_boundary",
        ),
        audit_index_entry(
            "terminal_no_request_closeout_blocker_chain_audit_index",
            "terminal_no_request_closeout_readback_audit_index.blocker_chain",
            "terminal_no_request_closeout_blocker_chain_readback",
            "terminal_closeout_blocker_chain",
        ),
        audit_index_entry(
            "terminal_no_request_closeout_prior_chain_audit_index",
            "terminal_no_request_closeout_readback_audit_index.prior_chain",
            "terminal_no_request_closeout_prior_chain_readback",
            "terminal_closeout_required_prior_chain",
        ),
        audit_index_entry(
            "terminal_no_request_closeout_request_boundary_audit_index",
            "terminal_no_request_closeout_readback_audit_index.request_boundary",
            "terminal_no_request_closeout_no_request_boundary_readback",
            "operator_review_request_boundary",
        ),
        audit_index_entry(
            "terminal_no_request_closeout_live_boundary_audit_index",
            "terminal_no_request_closeout_readback_audit_index.live_boundary",
            "terminal_no_request_closeout_no_live_boundary_readback",
            "live_cutover_boundary",
        ),
        audit_index_entry(
            "terminal_no_request_closeout_no_acceptance_audit_index",
            "terminal_no_request_closeout_readback_audit_index.no_acceptance",
            "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_gate",
            "no_acceptance_boundary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_blockers()
-> Vec<WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexBlockerPreview>{
    vec![
        audit_index_blocker(
            "terminal_closeout_readback_audit_index_record_blocked",
            "record_terminal_no_request_closeout_readback_audit_index",
            "terminal no-request closeout readback audit index remains report-only and unrecorded",
        ),
        audit_index_blocker(
            "terminal_closeout_readback_audit_index_persistence_blocked",
            "persist_terminal_no_request_closeout_readback_audit_index",
            "terminal no-request closeout readback audit index is not written to WorkGraph or projection storage",
        ),
        audit_index_blocker(
            "terminal_closeout_readback_audit_index_acceptance_blocked",
            "accept_terminal_no_request_closeout_readback_audit_index",
            "terminal no-request closeout readback audit index does not create operator acceptance",
        ),
        audit_index_blocker(
            "terminal_closeout_readback_record_blocked",
            "record_terminal_no_request_closeout_readback",
            "terminal no-request closeout readback remains unrecorded",
        ),
        audit_index_blocker(
            "terminal_closeout_readback_persistence_blocked",
            "persist_terminal_no_request_closeout_readback",
            "terminal no-request closeout readback remains unpersisted",
        ),
        audit_index_blocker(
            "terminal_closeout_record_blocked",
            "record_terminal_no_request_closeout",
            "terminal no-request closeout remains unrecorded",
        ),
        audit_index_blocker(
            "terminal_closeout_persistence_blocked",
            "persist_terminal_no_request_closeout",
            "terminal no-request closeout remains unpersisted",
        ),
        audit_index_blocker(
            "terminal_closeout_acceptance_blocked",
            "accept_terminal_no_request_closeout",
            "terminal no-request closeout remains unaccepted",
        ),
        audit_index_blocker(
            "operator_review_request_blocked",
            "request_operator_review",
            "operator review request remains unauthorized",
        ),
        audit_index_blocker(
            "operator_review_request_record_blocked",
            "record_operator_review_request",
            "operator review request recording remains disallowed",
        ),
        audit_index_blocker(
            "operator_review_request_persistence_blocked",
            "persist_operator_review_request",
            "operator review request persistence remains disallowed",
        ),
        audit_index_blocker(
            "operator_review_request_acceptance_blocked",
            "accept_operator_review_request",
            "operator review request acceptance remains disallowed",
        ),
        audit_index_blocker(
            "operator_packet_send_blocked",
            "send_operator_packet",
            "operator packet remains unsent",
        ),
        audit_index_blocker(
            "operator_packet_acceptance_blocked",
            "accept_operator_packet",
            "operator packet remains unaccepted",
        ),
        audit_index_blocker(
            "approval_record_blocked",
            "record_operator_approval",
            "approval recording remains disabled",
        ),
        audit_index_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
            "feature-flag config writes remain disabled",
        ),
        audit_index_blocker(
            "feature_flag_enablement_blocked",
            "enable_feature_flag",
            "feature flags remain current off",
        ),
        audit_index_blocker(
            "canary_traffic_blocked",
            "route_canary_traffic",
            "canary traffic remains 0ppm",
        ),
        audit_index_blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
            "scheduler admission remains dry-run only",
        ),
        audit_index_blocker(
            "guardrail_enforcement_blocked",
            "enable_guardrail_enforcement",
            "guardrail enforcement remains report-only",
        ),
        audit_index_blocker(
            "replay_execution_blocked",
            "execute_replay",
            "replay remains unexecuted",
        ),
        audit_index_blocker(
            "rollback_execution_blocked",
            "execute_rollback",
            "rollback remains unexecuted",
        ),
        audit_index_blocker(
            "work_graph_projection_persistence_blocked",
            "persist_work_graph_projection",
            "WorkGraph projection persistence remains disabled",
        ),
        audit_index_blocker(
            "work_graph_event_record_blocked",
            "record_work_graph_event",
            "WorkGraph event recording remains disabled",
        ),
        audit_index_blocker(
            "live_cutover_blocked",
            "perform_live_cutover",
            "live cutover remains disabled",
        ),
    ]
}

impl
    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexSideEffects
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
            audit_index_recorded: false,
            audit_index_persisted: false,
            audit_index_accepted: false,
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

fn audit_index_entry(
    id: &'static str,
    stable_index_key: &'static str,
    source_readback_id: &'static str,
    audit_category: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexEntryPreview
{
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexEntryPreview {
        id,
        stable_index_key,
        source_readback_id,
        audit_category,
        indexed: true,
        recorded: false,
        persisted: false,
        authoritative: false,
        operator_review_requested: false,
        mutation_allowed: false,
        ready: true,
    }
}

fn audit_index_blocker(
    id: &'static str,
    blocked_action: &'static str,
    reason: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexBlockerPreview
{
    WorkGraphOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexBlockerPreview {
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
    fn terminal_no_request_closeout_readback_audit_index_derives_from_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_report();

        assert_eq!(
            report.source_terminal_closeout_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_TERMINAL_NO_REQUEST_CLOSEOUT_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 5);
        assert_eq!(report.source_readback_blocker_count, 22);
        assert_eq!(report.source_required_prior_gate_count, 22);
        assert!(report.source_terminal_closeout_readback_preconditions_complete);
        assert!(report.source_terminal_closeout_readback_no_request_confirmed);
        assert!(report.source_terminal_closeout_readback_no_authorization_confirmed);
        assert!(report.source_terminal_closeout_readback_ready);
        assert_eq!(report.audit_index_entry_count, 6);
        assert_eq!(report.audit_index_blocker_count, 25);
        assert_eq!(report.required_prior_gate_count, 23);
    }

    #[test]
    fn terminal_no_request_closeout_readback_audit_index_is_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_report();

        assert!(report.audit_index_scope.index_visible);
        assert!(!report.audit_index_scope.index_recorded);
        assert!(!report.audit_index_scope.index_persisted);
        assert!(!report.audit_index_scope.index_authoritative);
        assert!(!report.audit_index_scope.index_accepted);
        assert!(!report.audit_index_scope.operator_review_requested);
        assert!(!report.audit_index_scope.acceptance_allowed);
        assert!(report.audit_index_scope_report_only_complete);
        assert!(report.audit_index_entries.iter().all(|entry| {
            entry.indexed
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.operator_review_requested
                && !entry.mutation_allowed
        }));
        assert!(report.audit_index_entries_report_only_complete);
    }

    #[test]
    fn terminal_no_request_closeout_readback_audit_index_blocks_request_and_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_report();

        assert!(
            report
                .audit_index_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.audit_index_blockers_complete);
        assert!(report.terminal_no_request_closeout_readback_audit_index_preconditions_complete);
        assert!(report.audit_index_visible);
        assert!(report.terminal_closeout_readback_visible);
        assert!(report.terminal_no_request);
        assert!(!report.audit_index_recorded);
        assert!(!report.audit_index_persisted);
        assert!(!report.audit_index_authoritative);
        assert!(!report.audit_index_accepted);
        assert!(!report.terminal_closeout_readback_persisted);
        assert!(!report.operator_review_request_allowed);
        assert!(!report.operator_review_requested);
        assert!(!report.operator_packet_send_allowed);
        assert!(!report.operator_packet_acceptance_allowed);
        assert!(!report.approval_recording_allowed);
        assert!(!report.audit_index_authorizes_operator_review_request);
        assert!(!report.audit_index_authorizes_operator_packet_send);
        assert!(!report.audit_index_authorizes_approval_recording);
        assert!(!report.audit_index_authorizes_config_write);
        assert!(!report.audit_index_authorizes_feature_flag_enablement);
        assert!(!report.audit_index_authorizes_canary_traffic);
        assert!(!report.audit_index_authorizes_scheduler_enforcement);
        assert!(!report.audit_index_authorizes_guardrail_enforcement);
        assert!(!report.audit_index_authorizes_replay_execution);
        assert!(!report.audit_index_authorizes_rollback_execution);
        assert!(!report.audit_index_authorizes_work_graph_persistence);
        assert!(!report.audit_index_authorizes_live_cutover);
        assert!(report.ready_for_non_persistence_readback);
        assert!(!report.ready_for_operator_review_request);
        assert!(!report.ready_for_approval_recording);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn terminal_no_request_closeout_readback_audit_index_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_closeout_readback_audit_index_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionTerminalNoRequestCloseoutReadbackAuditIndexSideEffects::none()
        );
    }
}
