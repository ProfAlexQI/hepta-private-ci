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
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_GATE,
    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixSideEffects,
    hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_report,
};
use crate::work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_request_precondition_gate: &'static str,
    pub source_request_decision: &'static str,
    pub source_request_blocker_count: usize,
    pub source_request_precondition_check_count: usize,
    pub source_required_prior_gate_count: usize,
    pub source_request_blocker_matrix_preconditions_complete: bool,
    pub source_request_blocker_matrix_no_request_confirmed: bool,
    pub source_request_blocker_matrix_no_authorization_confirmed: bool,
    pub source_request_blocker_matrix_ready: bool,
    pub request_denial_readback_entry_count: usize,
    pub request_denial_readback_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub request_denial_readback_scope:
        WorkGraphOperatorReviewRequestPreconditionDenialReadbackScopePreview,
    pub request_denial_readback_entries:
        Vec<WorkGraphOperatorReviewRequestPreconditionDenialReadbackEntryPreview>,
    pub request_denial_readback_blockers:
        Vec<WorkGraphOperatorReviewRequestPreconditionDenialReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub request_denial_readback_scope_complete: bool,
    pub request_denial_readback_entries_visible_only_complete: bool,
    pub request_denial_readback_blockers_complete: bool,
    pub request_denial_readback_preconditions_complete: bool,
    pub request_denial_visible: bool,
    pub request_denial_recorded: bool,
    pub request_denial_persisted: bool,
    pub request_denial_accepted: bool,
    pub request_denial_authoritative: bool,
    pub request_denial_readback_persisted: bool,
    pub request_denial_authorizes_operator_review_request: bool,
    pub request_denial_authorizes_operator_packet_send: bool,
    pub request_denial_authorizes_approval_recording: bool,
    pub request_denial_authorizes_config_write: bool,
    pub request_denial_authorizes_feature_flag_enablement: bool,
    pub request_denial_authorizes_canary_traffic: bool,
    pub request_denial_authorizes_live_cutover: bool,
    pub operator_review_requested: bool,
    pub operator_review_request_recorded: bool,
    pub operator_review_request_persisted: bool,
    pub operator_review_request_accepted: bool,
    pub ready_for_request_denial_audit_index: bool,
    pub ready_for_operator_review_request: bool,
    pub ready_for_approval_recording: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionDenialReadbackScopePreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub readback_mode: &'static str,
    pub stable_readback_key: &'static str,
    pub denial_visible: bool,
    pub denial_recorded: bool,
    pub denial_persisted: bool,
    pub denial_accepted: bool,
    pub denial_authoritative: bool,
    pub readback_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionDenialReadbackEntryPreview {
    pub id: &'static str,
    pub stable_readback_key: &'static str,
    pub observed_state: &'static str,
    pub visible: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub accepted: bool,
    pub authoritative: bool,
    pub mutation_allowed: bool,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionDenialReadbackBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackSideEffects
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
    pub readback_persisted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackReport {
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_report();
    let request_denial_readback_scope =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_scope();
    let request_denial_readback_entries =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_entries();
    let request_denial_readback_blockers =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_required_prior_gates();
    let source_request_blocker_matrix_no_request_confirmed = source.request_decision == "deny"
        && !source.operator_review_request_allowed
        && !source.operator_review_requested
        && !source.operator_review_request_recorded
        && !source.operator_review_request_persisted
        && !source.operator_review_request_accepted
        && source.ready_for_request_denial_readback
        && source.side_effects
            == WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixSideEffects::none(
            );
    let source_request_blocker_matrix_no_authorization_confirmed = !source
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
    let source_request_blocker_matrix_ready = source.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_GATE
        && source.request_precondition_blocker_matrix_preconditions_complete
        && source.ready_for_request_denial_readback
        && source_request_blocker_matrix_no_request_confirmed
        && source_request_blocker_matrix_no_authorization_confirmed;
    let request_denial_readback_scope_complete = request_denial_readback_scope.denial_visible
        && !request_denial_readback_scope.denial_recorded
        && !request_denial_readback_scope.denial_persisted
        && !request_denial_readback_scope.denial_accepted
        && !request_denial_readback_scope.denial_authoritative
        && !request_denial_readback_scope.readback_persisted;
    let request_denial_readback_entries_visible_only_complete = !request_denial_readback_entries
        .is_empty()
        && request_denial_readback_entries.iter().all(|entry| {
            entry.visible
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.accepted
                && !entry.authoritative
                && !entry.mutation_allowed
        });
    let request_denial_readback_blockers_complete = !request_denial_readback_blockers.is_empty()
        && request_denial_readback_blockers
            .iter()
            .all(|blocker| blocker.blocked);
    let request_denial_readback_preconditions_complete = source_request_blocker_matrix_ready
        && request_denial_readback_scope_complete
        && request_denial_readback_entries_visible_only_complete
        && request_denial_readback_blockers_complete;

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_SCHEMA_VERSION,
        preview_mode: "operator_review_request_precondition_denial_readback_only_no_request_no_record_no_persistence",
        source_request_precondition_gate: source.gate,
        source_request_decision: source.request_decision,
        source_request_blocker_count: source.request_blocker_count,
        source_request_precondition_check_count: source.request_precondition_check_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        source_request_blocker_matrix_preconditions_complete: source
            .request_precondition_blocker_matrix_preconditions_complete,
        source_request_blocker_matrix_no_request_confirmed,
        source_request_blocker_matrix_no_authorization_confirmed,
        source_request_blocker_matrix_ready,
        request_denial_readback_entry_count: request_denial_readback_entries.len(),
        request_denial_readback_blocker_count: request_denial_readback_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        request_denial_readback_scope,
        request_denial_readback_entries,
        request_denial_readback_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_RECOMMENDED_NEXT_GATE,
        request_denial_readback_scope_complete,
        request_denial_readback_entries_visible_only_complete,
        request_denial_readback_blockers_complete,
        request_denial_readback_preconditions_complete,
        request_denial_visible: true,
        request_denial_recorded: false,
        request_denial_persisted: false,
        request_denial_accepted: false,
        request_denial_authoritative: false,
        request_denial_readback_persisted: false,
        request_denial_authorizes_operator_review_request: false,
        request_denial_authorizes_operator_packet_send: false,
        request_denial_authorizes_approval_recording: false,
        request_denial_authorizes_config_write: false,
        request_denial_authorizes_feature_flag_enablement: false,
        request_denial_authorizes_canary_traffic: false,
        request_denial_authorizes_live_cutover: false,
        operator_review_requested: false,
        operator_review_request_recorded: false,
        operator_review_request_persisted: false,
        operator_review_request_accepted: false,
        ready_for_request_denial_audit_index: request_denial_readback_preconditions_complete,
        ready_for_operator_review_request: false,
        ready_for_approval_recording: false,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_scope()
-> WorkGraphOperatorReviewRequestPreconditionDenialReadbackScopePreview {
    WorkGraphOperatorReviewRequestPreconditionDenialReadbackScopePreview {
        id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition_blocker_matrix",
        readback_mode: "operator_review_request_precondition_denial_readback_only",
        stable_readback_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.denial_readback",
        denial_visible: true,
        denial_recorded: false,
        denial_persisted: false,
        denial_accepted: false,
        denial_authoritative: false,
        readback_persisted: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_entries()
-> Vec<WorkGraphOperatorReviewRequestPreconditionDenialReadbackEntryPreview> {
    vec![
        request_denial_readback_entry(
            "operator_review_request_denial_decision_readback",
            "operator_review_request_precondition_decision_deny",
            "request_operator_review_deny_visible_without_record_accept_or_persistence",
        ),
        request_denial_readback_entry(
            "operator_review_request_precondition_check_catalog_readback",
            "operator_review_request_precondition_checks_visible",
            "twelve_checks_visible_with_ten_blocking_and_two_source_evidence_checks",
        ),
        request_denial_readback_entry(
            "operator_review_request_blocker_catalog_readback",
            "operator_review_request_precondition_blockers_visible",
            "seventeen_blocked_actions_visible_without_authority_to_mutate",
        ),
        request_denial_readback_entry(
            "operator_review_request_boundary_readback",
            "operator_review_request_not_requested_or_recorded",
            "operator_review_request_remains_not_requested_not_recorded_not_persisted_not_accepted",
        ),
        request_denial_readback_entry(
            "operator_review_request_live_boundary_readback",
            "operator_review_request_denial_does_not_unlock_live_paths",
            "denial_readback_cannot_authorize_packet_approval_config_enablement_traffic_or_cutover",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_blockers()
-> Vec<WorkGraphOperatorReviewRequestPreconditionDenialReadbackBlockerPreview> {
    vec![
        request_denial_readback_blocker(
            "request_denial_readback_acceptance_blocked",
            "accept_operator_review_request_denial_readback",
            "request denial readback is not an operator acceptance or approval record",
        ),
        request_denial_readback_blocker(
            "request_denial_readback_persistence_blocked",
            "persist_operator_review_request_denial_readback",
            "request denial readback remains stdout/report-only and unpersisted",
        ),
        request_denial_readback_blocker(
            "operator_review_request_blocked",
            "request_operator_review",
            "denial readback cannot request operator review",
        ),
        request_denial_readback_blocker(
            "operator_review_request_record_blocked",
            "record_operator_review_request",
            "denial readback cannot record an operator review request",
        ),
        request_denial_readback_blocker(
            "operator_review_request_acceptance_blocked",
            "accept_operator_review_request",
            "denial readback cannot accept an operator review request",
        ),
        request_denial_readback_blocker(
            "operator_packet_send_blocked",
            "send_operator_packet",
            "operator packet send remains disallowed",
        ),
        request_denial_readback_blocker(
            "operator_packet_acceptance_blocked",
            "accept_operator_packet",
            "operator packet acceptance remains missing",
        ),
        request_denial_readback_blocker(
            "approval_record_blocked",
            "record_operator_approval",
            "approval recording remains disallowed",
        ),
        request_denial_readback_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
            "feature-flag config writes remain disabled",
        ),
        request_denial_readback_blocker(
            "feature_flag_enablement_blocked",
            "enable_feature_flag",
            "feature flags remain current off",
        ),
        request_denial_readback_blocker(
            "canary_traffic_blocked",
            "route_canary_traffic",
            "canary traffic remains 0ppm",
        ),
        request_denial_readback_blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
            "scheduler admission remains dry-run only",
        ),
        request_denial_readback_blocker(
            "guardrail_enforcement_blocked",
            "enable_guardrail_enforcement",
            "guardrail enforcement remains report-only",
        ),
        request_denial_readback_blocker(
            "replay_execution_blocked",
            "execute_replay",
            "replay remains unexecuted",
        ),
        request_denial_readback_blocker(
            "rollback_execution_blocked",
            "execute_rollback",
            "rollback remains unexecuted",
        ),
        request_denial_readback_blocker(
            "work_graph_projection_persistence_blocked",
            "persist_work_graph_projection",
            "WorkGraph projection persistence remains disabled",
        ),
        request_denial_readback_blocker(
            "work_graph_event_record_blocked",
            "record_work_graph_event",
            "WorkGraph event recording remains disabled",
        ),
        request_denial_readback_blocker(
            "live_cutover_blocked",
            "perform_live_cutover",
            "live cutover remains disabled",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_required_prior_gates()
-> Vec<&'static str> {
    vec![
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
    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackSideEffects
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
            readback_persisted: false,
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

fn request_denial_readback_entry(
    id: &'static str,
    stable_readback_key: &'static str,
    observed_state: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionDenialReadbackEntryPreview {
    WorkGraphOperatorReviewRequestPreconditionDenialReadbackEntryPreview {
        id,
        stable_readback_key,
        observed_state,
        visible: true,
        recorded: false,
        persisted: false,
        accepted: false,
        authoritative: false,
        mutation_allowed: false,
        ready: true,
    }
}

fn request_denial_readback_blocker(
    id: &'static str,
    blocked_action: &'static str,
    reason: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionDenialReadbackBlockerPreview {
    WorkGraphOperatorReviewRequestPreconditionDenialReadbackBlockerPreview {
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
    fn request_denial_readback_derives_from_blocker_matrix() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_report();

        assert_eq!(
            report.source_request_precondition_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_GATE
        );
        assert_eq!(report.source_request_decision, "deny");
        assert_eq!(report.source_request_blocker_count, 17);
        assert_eq!(report.source_request_precondition_check_count, 12);
        assert_eq!(report.source_required_prior_gate_count, 17);
        assert!(report.source_request_blocker_matrix_preconditions_complete);
        assert!(report.source_request_blocker_matrix_no_request_confirmed);
        assert!(report.source_request_blocker_matrix_no_authorization_confirmed);
        assert!(report.source_request_blocker_matrix_ready);
        assert!(report.request_denial_visible);
        assert!(!report.request_denial_recorded);
        assert!(!report.request_denial_persisted);
        assert!(!report.request_denial_accepted);
        assert!(!report.request_denial_authoritative);
    }

    #[test]
    fn request_denial_readback_entries_are_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_report();

        assert_eq!(report.request_denial_readback_entry_count, 5);
        assert_eq!(
            report.request_denial_readback_scope.readback_mode,
            "operator_review_request_precondition_denial_readback_only"
        );
        assert!(report.request_denial_readback_scope.denial_visible);
        assert!(!report.request_denial_readback_scope.denial_recorded);
        assert!(!report.request_denial_readback_scope.denial_persisted);
        assert!(!report.request_denial_readback_scope.denial_accepted);
        assert!(!report.request_denial_readback_scope.denial_authoritative);
        assert!(!report.request_denial_readback_scope.readback_persisted);
        assert!(report.request_denial_readback_scope_complete);
        assert!(report.request_denial_readback_entries.iter().all(|entry| {
            entry.visible
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.accepted
                && !entry.authoritative
                && !entry.mutation_allowed
        }));
        assert!(report.request_denial_readback_entries_visible_only_complete);
    }

    #[test]
    fn request_denial_readback_blocks_request_and_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_report();

        assert_eq!(report.request_denial_readback_blocker_count, 18);
        assert_eq!(report.required_prior_gate_count, 18);
        assert_eq!(
            report.required_prior_gates[0],
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_GATE
        );
        assert!(
            report
                .request_denial_readback_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.request_denial_readback_blockers_complete);
        assert!(report.request_denial_readback_preconditions_complete);
        assert!(report.ready_for_request_denial_audit_index);
        assert!(!report.request_denial_readback_persisted);
        assert!(!report.request_denial_authorizes_operator_review_request);
        assert!(!report.request_denial_authorizes_operator_packet_send);
        assert!(!report.request_denial_authorizes_approval_recording);
        assert!(!report.request_denial_authorizes_config_write);
        assert!(!report.request_denial_authorizes_feature_flag_enablement);
        assert!(!report.request_denial_authorizes_canary_traffic);
        assert!(!report.request_denial_authorizes_live_cutover);
        assert!(!report.operator_review_requested);
        assert!(!report.operator_review_request_recorded);
        assert!(!report.operator_review_request_persisted);
        assert!(!report.operator_review_request_accepted);
        assert!(!report.ready_for_operator_review_request);
        assert!(!report.ready_for_approval_recording);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn request_denial_readback_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackSideEffects::none()
        );
    }
}
