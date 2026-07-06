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
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_report,
};
use crate::work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_non_persistence_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackAuditIndexReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_request_denial_readback_gate: &'static str,
    pub source_readback_entry_count: usize,
    pub source_readback_blocker_count: usize,
    pub source_required_prior_gate_count: usize,
    pub audit_index_entry_count: usize,
    pub audit_index_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub audit_index_scope:
        WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexScopePreview,
    pub audit_index_entries:
        Vec<WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexEntryPreview>,
    pub audit_index_blockers:
        Vec<WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub audit_index_visible: bool,
    pub audit_index_recorded: bool,
    pub audit_index_persisted: bool,
    pub audit_index_authoritative: bool,
    pub audit_index_accepted: bool,
    pub request_denial_readback_visible: bool,
    pub request_denial_readback_persisted: bool,
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
        WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackAuditIndexSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexScopePreview {
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
pub struct WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexEntryPreview {
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
pub struct WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackAuditIndexSideEffects
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackAuditIndexReport{
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_report();
    let audit_index_scope =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_scope();
    let audit_index_entries =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_entries();
    let audit_index_blockers =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_required_prior_gates();

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackAuditIndexReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_SCHEMA_VERSION,
        preview_mode: "operator_review_request_precondition_denial_readback_audit_index_no_request_no_record_no_persistence",
        source_request_denial_readback_gate: source.gate,
        source_readback_entry_count: source.request_denial_readback_entry_count,
        source_readback_blocker_count: source.request_denial_readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        audit_index_entry_count: audit_index_entries.len(),
        audit_index_blocker_count: audit_index_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        audit_index_scope,
        audit_index_entries,
        audit_index_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_AUDIT_INDEX_RECOMMENDED_NEXT_GATE,
        audit_index_visible: true,
        audit_index_recorded: false,
        audit_index_persisted: false,
        audit_index_authoritative: false,
        audit_index_accepted: false,
        request_denial_readback_visible: source.request_denial_visible,
        request_denial_readback_persisted: false,
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
        ready_for_non_persistence_readback: true,
        ready_for_operator_review_request: false,
        ready_for_approval_recording: false,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackAuditIndexSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_scope()
-> WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexScopePreview {
    WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexScopePreview {
        id: "agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_scope",
        source_surface_id: "work_graph_agent_jobs_task_board.feature_flag.operator_review_request_precondition_denial_readback",
        index_mode: "operator_review_request_precondition_denial_readback_audit_index_report_only",
        stable_index_key: "work_graph.agent_jobs_task_board.feature_flag.operator_review_request_precondition.denial_readback.audit_index",
        index_visible: true,
        index_recorded: false,
        index_persisted: false,
        index_authoritative: false,
        index_accepted: false,
        operator_review_requested: false,
        acceptance_allowed: false,
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_entries()
-> Vec<WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexEntryPreview> {
    vec![
        audit_index_entry(
            "operator_review_request_denial_decision_audit_index",
            "operator_review_request_denial_audit_index.denial_decision",
            "operator_review_request_denial_decision_readback",
            "operator_review_request_denial_boundary",
        ),
        audit_index_entry(
            "operator_review_request_check_catalog_audit_index",
            "operator_review_request_denial_audit_index.precondition_checks",
            "operator_review_request_precondition_check_catalog_readback",
            "request_precondition_check_catalog",
        ),
        audit_index_entry(
            "operator_review_request_blocker_catalog_audit_index",
            "operator_review_request_denial_audit_index.blocker_catalog",
            "operator_review_request_blocker_catalog_readback",
            "request_blocker_catalog",
        ),
        audit_index_entry(
            "operator_review_request_boundary_audit_index",
            "operator_review_request_denial_audit_index.request_boundary",
            "operator_review_request_boundary_readback",
            "operator_review_request_boundary",
        ),
        audit_index_entry(
            "operator_review_request_live_boundary_audit_index",
            "operator_review_request_denial_audit_index.live_boundary",
            "operator_review_request_live_boundary_readback",
            "live_cutover_boundary",
        ),
        audit_index_entry(
            "operator_review_request_no_acceptance_audit_index",
            "operator_review_request_denial_audit_index.no_acceptance",
            "operator_review_request_precondition_denial_readback_gate",
            "no_acceptance_boundary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_blockers()
-> Vec<WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexBlockerPreview> {
    vec![
        audit_index_blocker(
            "request_denial_audit_index_record_blocked",
            "record_operator_review_request_denial_audit_index",
            "request denial audit index remains report-only and unrecorded",
        ),
        audit_index_blocker(
            "request_denial_audit_index_persistence_blocked",
            "persist_operator_review_request_denial_audit_index",
            "request denial audit index is not written to WorkGraph or projection storage",
        ),
        audit_index_blocker(
            "request_denial_audit_index_acceptance_blocked",
            "accept_operator_review_request_denial_audit_index",
            "request denial audit index does not create operator acceptance",
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

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_required_prior_gates()
-> Vec<&'static str> {
    vec![
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
    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackAuditIndexSideEffects
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

fn audit_index_entry(
    id: &'static str,
    stable_index_key: &'static str,
    source_readback_id: &'static str,
    audit_category: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexEntryPreview {
    WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexEntryPreview {
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
) -> WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexBlockerPreview {
    WorkGraphOperatorReviewRequestPreconditionDenialReadbackAuditIndexBlockerPreview {
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
    fn request_denial_audit_index_derives_from_denial_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_report();

        assert_eq!(
            report.source_request_denial_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 5);
        assert_eq!(report.source_readback_blocker_count, 18);
        assert_eq!(report.source_required_prior_gate_count, 18);
        assert_eq!(report.audit_index_entry_count, 6);
        assert_eq!(report.audit_index_blocker_count, 19);
    }

    #[test]
    fn request_denial_audit_index_entries_are_visible_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_report();

        assert_eq!(
            report.audit_index_scope.index_mode,
            "operator_review_request_precondition_denial_readback_audit_index_report_only"
        );
        assert!(report.audit_index_scope.index_visible);
        assert!(!report.audit_index_scope.index_recorded);
        assert!(!report.audit_index_scope.index_persisted);
        assert!(!report.audit_index_scope.index_authoritative);
        assert!(!report.audit_index_scope.index_accepted);
        assert!(!report.audit_index_scope.operator_review_requested);
        assert!(!report.audit_index_scope.acceptance_allowed);
        assert!(report.audit_index_entries.iter().all(|entry| {
            entry.indexed
                && entry.ready
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.operator_review_requested
                && !entry.mutation_allowed
        }));
    }

    #[test]
    fn request_denial_audit_index_blocks_request_and_live_paths() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_report();

        assert_eq!(report.required_prior_gate_count, 19);
        assert_eq!(
            report.required_prior_gates[0],
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_DENIAL_READBACK_GATE
        );
        assert!(
            report
                .audit_index_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert!(report.ready_for_non_persistence_readback);
        assert!(report.audit_index_visible);
        assert!(!report.audit_index_recorded);
        assert!(!report.audit_index_persisted);
        assert!(!report.audit_index_authoritative);
        assert!(!report.audit_index_accepted);
        assert!(report.request_denial_readback_visible);
        assert!(!report.request_denial_readback_persisted);
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
        assert!(!report.ready_for_operator_review_request);
        assert!(!report.ready_for_approval_recording);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn request_denial_audit_index_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_audit_index_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionDenialReadbackAuditIndexSideEffects::none()
        );
    }
}
