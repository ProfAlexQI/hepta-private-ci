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
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_NON_REQUEST_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_report,
};
use crate::work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_denial_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixReport {
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
    pub request_precondition_check_count: usize,
    pub request_precondition_satisfied_count: usize,
    pub request_precondition_unsatisfied_count: usize,
    pub request_precondition_blocking_count: usize,
    pub request_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub request_precondition_checks:
        Vec<WorkGraphOperatorReviewRequestPreconditionCheckPreview>,
    pub request_blockers: Vec<WorkGraphOperatorReviewRequestPreconditionBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub request_decision: &'static str,
    pub request_target_action: &'static str,
    pub operator_review_request_allowed: bool,
    pub operator_review_requested: bool,
    pub operator_review_request_recorded: bool,
    pub operator_review_request_persisted: bool,
    pub operator_review_request_accepted: bool,
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
    pub ready_for_request_denial_readback: bool,
    pub ready_for_operator_review_request: bool,
    pub ready_for_approval_recording: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionCheckPreview {
    pub id: &'static str,
    pub category: &'static str,
    pub required: bool,
    pub satisfied: bool,
    pub blocking: bool,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewRequestPreconditionBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocker_class: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixSideEffects
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixReport {
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_non_request_readback_audit_index_non_persistence_readback_report();
    let request_precondition_checks =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_checks();
    let request_blockers =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blockers(
        );
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_required_prior_gates();
    let request_precondition_satisfied_count = request_precondition_checks
        .iter()
        .filter(|check| check.satisfied)
        .count();
    let request_precondition_blocking_count = request_precondition_checks
        .iter()
        .filter(|check| check.blocking)
        .count();

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_SCHEMA_VERSION,
        preview_mode: "operator_review_request_precondition_blocker_matrix_deny_only_no_request",
        source_non_persistence_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_readback_blocker_count: source.readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        request_precondition_check_count: request_precondition_checks.len(),
        request_precondition_satisfied_count,
        request_precondition_unsatisfied_count: request_precondition_checks.len()
            - request_precondition_satisfied_count,
        request_precondition_blocking_count,
        request_blocker_count: request_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        request_precondition_checks,
        request_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_REQUEST_PRECONDITION_BLOCKER_MATRIX_RECOMMENDED_NEXT_GATE,
        request_decision: "deny",
        request_target_action: "request_operator_review",
        operator_review_request_allowed: false,
        operator_review_requested: false,
        operator_review_request_recorded: false,
        operator_review_request_persisted: false,
        operator_review_request_accepted: false,
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
        ready_for_request_denial_readback: true,
        ready_for_operator_review_request: false,
        ready_for_approval_recording: false,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_checks()
-> Vec<WorkGraphOperatorReviewRequestPreconditionCheckPreview> {
    vec![
        request_precondition_check(
            "non_request_audit_index_non_persistence_readback_ready",
            "source_evidence",
            true,
            false,
            "non-request audit index non-persistence readback is available",
        ),
        request_precondition_check(
            "required_prior_chain_visible",
            "source_evidence",
            true,
            false,
            "the request precondition prior chain is visible through report-only gates",
        ),
        request_precondition_check(
            "operator_review_request_authorization_missing",
            "operator_review_request_boundary",
            false,
            true,
            "no explicit authorization exists to request operator review",
        ),
        request_precondition_check(
            "operator_review_request_recording_authorization_missing",
            "operator_review_request_boundary",
            false,
            true,
            "operator review request recording remains disallowed",
        ),
        request_precondition_check(
            "operator_packet_send_boundary_unsatisfied",
            "operator_packet_boundary",
            false,
            true,
            "operator packet send remains disallowed",
        ),
        request_precondition_check(
            "operator_packet_acceptance_missing",
            "operator_packet_boundary",
            false,
            true,
            "operator packet acceptance remains missing",
        ),
        request_precondition_check(
            "approval_recording_authorization_missing",
            "approval_boundary",
            false,
            true,
            "approval recording remains disallowed",
        ),
        request_precondition_check(
            "config_write_authorization_missing",
            "config_boundary",
            false,
            true,
            "feature-flag config writes remain disallowed",
        ),
        request_precondition_check(
            "feature_flag_enablement_disabled",
            "feature_flag_boundary",
            false,
            true,
            "feature flags remain current off",
        ),
        request_precondition_check(
            "canary_traffic_disallowed",
            "traffic_boundary",
            false,
            true,
            "canary traffic remains 0ppm",
        ),
        request_precondition_check(
            "scheduler_guardrail_enforcement_missing",
            "enforcement_boundary",
            false,
            true,
            "scheduler and guardrail checks remain dry-run only",
        ),
        request_precondition_check(
            "replay_rollback_live_cutover_missing",
            "live_cutover_boundary",
            false,
            true,
            "replay, rollback, and live cutover remain unavailable",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blockers()
-> Vec<WorkGraphOperatorReviewRequestPreconditionBlockerPreview> {
    vec![
        request_blocker(
            "operator_review_request_blocked",
            "request_operator_review",
            "operator_review_request_boundary",
        ),
        request_blocker(
            "operator_review_request_record_blocked",
            "record_operator_review_request",
            "operator_review_request_boundary",
        ),
        request_blocker(
            "operator_review_request_persistence_blocked",
            "persist_operator_review_request",
            "operator_review_request_boundary",
        ),
        request_blocker(
            "operator_review_request_acceptance_blocked",
            "accept_operator_review_request",
            "operator_review_request_boundary",
        ),
        request_blocker(
            "operator_packet_send_blocked",
            "send_operator_packet",
            "operator_packet_boundary",
        ),
        request_blocker(
            "operator_packet_acceptance_blocked",
            "accept_operator_packet",
            "operator_packet_boundary",
        ),
        request_blocker(
            "approval_record_blocked",
            "record_operator_approval",
            "approval_boundary",
        ),
        request_blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
            "config_boundary",
        ),
        request_blocker(
            "feature_flag_enablement_blocked",
            "enable_feature_flag",
            "feature_flag_boundary",
        ),
        request_blocker(
            "canary_traffic_blocked",
            "route_canary_traffic",
            "traffic_boundary",
        ),
        request_blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
            "scheduler_boundary",
        ),
        request_blocker(
            "guardrail_enforcement_blocked",
            "enable_guardrail_enforcement",
            "guardrail_boundary",
        ),
        request_blocker(
            "replay_execution_blocked",
            "execute_replay",
            "replay_boundary",
        ),
        request_blocker(
            "rollback_execution_blocked",
            "execute_rollback",
            "rollback_boundary",
        ),
        request_blocker(
            "work_graph_projection_persistence_blocked",
            "persist_work_graph_projection",
            "work_graph_persistence_boundary",
        ),
        request_blocker(
            "work_graph_event_record_blocked",
            "record_work_graph_event",
            "work_graph_persistence_boundary",
        ),
        request_blocker(
            "live_cutover_blocked",
            "perform_live_cutover",
            "live_cutover_boundary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_required_prior_gates()
-> Vec<&'static str> {
    vec![
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
    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixSideEffects
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

fn request_precondition_check(
    id: &'static str,
    category: &'static str,
    satisfied: bool,
    blocking: bool,
    explanation: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionCheckPreview {
    WorkGraphOperatorReviewRequestPreconditionCheckPreview {
        id,
        category,
        required: true,
        satisfied,
        blocking,
        explanation,
    }
}

fn request_blocker(
    id: &'static str,
    blocked_action: &'static str,
    blocker_class: &'static str,
) -> WorkGraphOperatorReviewRequestPreconditionBlockerPreview {
    WorkGraphOperatorReviewRequestPreconditionBlockerPreview {
        id,
        blocked_action,
        blocker_class,
        blocked: true,
        reason: "operator review request precondition blocker matrix cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_precondition_blocker_matrix_derives_from_non_persistence_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_report();

        assert_eq!(
            report.source_non_persistence_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_NON_REQUEST_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 5);
        assert_eq!(report.source_readback_blocker_count, 16);
        assert_eq!(report.source_required_prior_gate_count, 16);
        assert_eq!(report.request_precondition_check_count, 12);
        assert_eq!(report.request_precondition_satisfied_count, 2);
        assert_eq!(report.request_precondition_unsatisfied_count, 10);
        assert_eq!(report.request_precondition_blocking_count, 10);
    }

    #[test]
    fn request_precondition_blocker_matrix_denies_request() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_report();

        assert_eq!(report.request_decision, "deny");
        assert_eq!(report.request_target_action, "request_operator_review");
        assert_eq!(report.request_blocker_count, 17);
        assert!(!report.operator_review_request_allowed);
        assert!(!report.operator_review_requested);
        assert!(!report.operator_review_request_recorded);
        assert!(!report.operator_review_request_persisted);
        assert!(!report.operator_review_request_accepted);
        assert!(report.ready_for_request_denial_readback);
        assert!(!report.ready_for_operator_review_request);
    }

    #[test]
    fn request_precondition_blocker_matrix_blocks_live_paths_and_requires_priors() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_report();

        assert_eq!(report.required_prior_gate_count, 17);
        assert_eq!(
            report.required_prior_gates[0],
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_NON_REQUEST_READBACK_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert!(
            report
                .request_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
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
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn request_precondition_blocker_matrix_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_blocker_matrix_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewRequestPreconditionBlockerMatrixSideEffects::none()
        );
    }
}
