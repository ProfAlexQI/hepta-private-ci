use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE,
    WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackSideEffects,
    hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_report,
};
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_non_request_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewPreconditionMatrixReport {
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
    pub precondition_check_count: usize,
    pub precondition_satisfied_count: usize,
    pub precondition_unsatisfied_count: usize,
    pub blocking_precondition_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub precondition_checks: Vec<WorkGraphOperatorReviewPreconditionCheckPreview>,
    pub blockers: Vec<WorkGraphOperatorReviewPreconditionBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub source_non_persistence_readback_preconditions_complete: bool,
    pub source_non_persistence_readback_no_record_persist_request_confirmed: bool,
    pub source_non_persistence_readback_no_authorization_confirmed: bool,
    pub source_non_persistence_readback_ready: bool,
    pub precondition_checks_complete: bool,
    pub blockers_complete: bool,
    pub operator_review_precondition_matrix_preconditions_complete: bool,
    pub matrix_mode: &'static str,
    pub operator_review_request_allowed: bool,
    pub operator_review_request_sent: bool,
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
    pub live_cutover_allowed: bool,
    pub ready_for_non_request_readback: bool,
    pub ready_for_operator_review_request: bool,
    pub ready_for_approval_recording: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewPreconditionMatrixSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewPreconditionCheckPreview {
    pub id: &'static str,
    pub category: &'static str,
    pub required: bool,
    pub satisfied: bool,
    pub blocking: bool,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphOperatorReviewPreconditionBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewPreconditionMatrixSideEffects {
    pub filesystem_written: bool,
    pub operator_review_requested: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewPreconditionMatrixReport {
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_audit_index_non_persistence_readback_report();
    let precondition_checks =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_checks();
    let blockers =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_required_prior_gates(
        );
    let precondition_satisfied_count = precondition_checks
        .iter()
        .filter(|check| check.satisfied)
        .count();
    let blocking_precondition_count = precondition_checks
        .iter()
        .filter(|check| check.blocking)
        .count();
    let source_non_persistence_readback_no_record_persist_request_confirmed =
        source.audit_index_visible
            && !source.audit_index_recorded
            && !source.audit_index_persisted
            && !source.audit_index_authoritative
            && !source.audit_index_accepted
            && !source.readback_persisted
            && !source.operator_review_request_allowed
            && source.side_effects
                == WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDenialAuditIndexNonPersistenceReadbackSideEffects::none(
                );
    let source_non_persistence_readback_no_authorization_confirmed = !source
        .operator_review_request_allowed
        && !source.approval_recorded
        && !source.config_write_allowed
        && !source.feature_flag_enablement_allowed
        && !source.canary_traffic_allowed
        && !source.scheduler_enforcement_allowed
        && !source.replay_execution_allowed
        && !source.rollback_execution_allowed
        && !source.live_cutover_allowed
        && !source.ready_for_feature_flag_config_write
        && !source.ready_for_feature_flag_enablement
        && !source.ready_for_canary_traffic
        && !source.ready_for_live_cutover;
    let source_non_persistence_readback_ready = source.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        && source.non_persistence_readback_preconditions_complete
        && source.ready_for_operator_review_precondition_matrix
        && source_non_persistence_readback_no_record_persist_request_confirmed
        && source_non_persistence_readback_no_authorization_confirmed;
    let precondition_checks_complete = !precondition_checks.is_empty()
        && precondition_checks.iter().all(|check| check.required)
        && precondition_satisfied_count == 2
        && precondition_checks.len() - precondition_satisfied_count == blocking_precondition_count
        && blocking_precondition_count == 7;
    let blockers_complete = !blockers.is_empty() && blockers.iter().all(|blocker| blocker.blocked);
    let operator_review_precondition_matrix_preconditions_complete =
        source_non_persistence_readback_ready && precondition_checks_complete && blockers_complete;

    WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewPreconditionMatrixReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_SCHEMA_VERSION,
        preview_mode: "operator_review_precondition_matrix_no_request_no_approval_no_write",
        source_non_persistence_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_readback_blocker_count: source.readback_blocker_count,
        source_required_prior_gate_count: source.required_prior_gate_count,
        precondition_check_count: precondition_checks.len(),
        precondition_satisfied_count,
        precondition_unsatisfied_count: precondition_checks.len() - precondition_satisfied_count,
        blocking_precondition_count,
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        precondition_checks,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_REVIEW_PRECONDITION_MATRIX_RECOMMENDED_NEXT_GATE,
        source_non_persistence_readback_preconditions_complete: source
            .non_persistence_readback_preconditions_complete,
        source_non_persistence_readback_no_record_persist_request_confirmed,
        source_non_persistence_readback_no_authorization_confirmed,
        source_non_persistence_readback_ready,
        precondition_checks_complete,
        blockers_complete,
        operator_review_precondition_matrix_preconditions_complete,
        matrix_mode: "deny_request_until_explicit_operator_review_authorization",
        operator_review_request_allowed: false,
        operator_review_request_sent: false,
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
        live_cutover_allowed: false,
        ready_for_non_request_readback: operator_review_precondition_matrix_preconditions_complete,
        ready_for_operator_review_request: false,
        ready_for_approval_recording: false,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewPreconditionMatrixSideEffects::none(
            ),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_checks()
-> Vec<WorkGraphOperatorReviewPreconditionCheckPreview> {
    vec![
        precondition_check(
            "denial_audit_index_non_persistence_readback_ready",
            "source_evidence",
            true,
            false,
            "denial audit index non-persistence readback is available",
        ),
        precondition_check(
            "required_prior_chain_visible",
            "source_evidence",
            true,
            false,
            "the canary prior chain is visible through report-only gates",
        ),
        precondition_check(
            "operator_review_request_authorization_missing",
            "operator_review_boundary",
            false,
            true,
            "no explicit authorization exists to request operator review",
        ),
        precondition_check(
            "operator_packet_acceptance_missing",
            "operator_packet_boundary",
            false,
            true,
            "operator packet remains unsent and unaccepted",
        ),
        precondition_check(
            "approval_recording_authorization_missing",
            "approval_boundary",
            false,
            true,
            "approval recording remains disallowed",
        ),
        precondition_check(
            "config_write_authorization_missing",
            "config_boundary",
            false,
            true,
            "feature-flag config writes remain disallowed",
        ),
        precondition_check(
            "scheduler_guardrail_enforcement_missing",
            "enforcement_boundary",
            false,
            true,
            "scheduler and guardrail checks remain dry-run only",
        ),
        precondition_check(
            "replay_rollback_execution_missing",
            "replay_rollback_boundary",
            false,
            true,
            "replay and rollback remain preview-only",
        ),
        precondition_check(
            "live_cutover_authorization_missing",
            "live_cutover_boundary",
            false,
            true,
            "live WorkGraph cutover remains disabled",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_blockers()
-> Vec<WorkGraphOperatorReviewPreconditionBlockerPreview> {
    vec![
        blocker("operator_review_request_blocked", "request_operator_review"),
        blocker("operator_packet_send_blocked", "send_operator_packet"),
        blocker(
            "operator_packet_acceptance_blocked",
            "accept_operator_packet",
        ),
        blocker("approval_record_blocked", "record_operator_approval"),
        blocker(
            "feature_flag_config_write_blocked",
            "write_feature_flag_config",
        ),
        blocker("feature_flag_enablement_blocked", "enable_feature_flag"),
        blocker("canary_traffic_blocked", "route_canary_traffic"),
        blocker(
            "scheduler_enforcement_blocked",
            "enforce_scheduler_admission",
        ),
        blocker(
            "guardrail_enforcement_blocked",
            "enable_guardrail_enforcement",
        ),
        blocker("replay_execution_blocked", "execute_replay"),
        blocker("rollback_execution_blocked", "execute_rollback"),
        blocker(
            "work_graph_persistence_blocked",
            "persist_work_graph_projection",
        ),
        blocker("live_cutover_blocked", "perform_live_cutover"),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_required_prior_gates()
-> Vec<&'static str> {
    vec![
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

impl WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewPreconditionMatrixSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            operator_review_requested: false,
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

fn precondition_check(
    id: &'static str,
    category: &'static str,
    satisfied: bool,
    blocking: bool,
    explanation: &'static str,
) -> WorkGraphOperatorReviewPreconditionCheckPreview {
    WorkGraphOperatorReviewPreconditionCheckPreview {
        id,
        category,
        required: true,
        satisfied,
        blocking,
        explanation,
    }
}

fn blocker(
    id: &'static str,
    blocked_action: &'static str,
) -> WorkGraphOperatorReviewPreconditionBlockerPreview {
    WorkGraphOperatorReviewPreconditionBlockerPreview {
        id,
        blocked_action,
        blocked: true,
        reason: "operator review precondition matrix cannot authorize this action",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_review_precondition_matrix_derives_from_non_persistence_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_report();

        assert_eq!(
            report.source_non_persistence_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DENIAL_AUDIT_INDEX_NON_PERSISTENCE_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 5);
        assert_eq!(report.source_readback_blocker_count, 12);
        assert_eq!(report.source_required_prior_gate_count, 12);
        assert!(report.source_non_persistence_readback_preconditions_complete);
        assert!(report.source_non_persistence_readback_no_record_persist_request_confirmed);
        assert!(report.source_non_persistence_readback_no_authorization_confirmed);
        assert!(report.source_non_persistence_readback_ready);
        assert_eq!(report.precondition_check_count, 9);
        assert_eq!(report.precondition_satisfied_count, 2);
        assert_eq!(report.precondition_unsatisfied_count, 7);
        assert_eq!(report.blocking_precondition_count, 7);
        assert!(report.precondition_checks_complete);
    }

    #[test]
    fn operator_review_precondition_matrix_keeps_review_unrequested() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_report();

        assert_eq!(
            report.matrix_mode,
            "deny_request_until_explicit_operator_review_authorization"
        );
        assert_eq!(report.blocker_count, 13);
        assert!(report.blockers_complete);
        assert!(report.operator_review_precondition_matrix_preconditions_complete);
        assert!(!report.operator_review_request_allowed);
        assert!(!report.operator_review_request_sent);
        assert!(!report.operator_packet_send_allowed);
        assert!(!report.operator_packet_acceptance_allowed);
        assert!(!report.approval_recording_allowed);
        assert!(report.ready_for_non_request_readback);
        assert!(!report.ready_for_operator_review_request);
        assert!(!report.ready_for_approval_recording);
    }

    #[test]
    fn operator_review_precondition_matrix_blocks_live_paths_and_requires_priors() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
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
        );
        assert_eq!(report.required_prior_gate_count, 13);
        assert!(report.blockers.iter().all(|blocker| blocker.blocked));
        assert!(!report.config_write_allowed);
        assert!(!report.feature_flag_enablement_allowed);
        assert!(!report.canary_traffic_allowed);
        assert!(!report.scheduler_enforcement_allowed);
        assert!(!report.guardrail_enforcement_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert!(!report.live_cutover_allowed);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn operator_review_precondition_matrix_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_precondition_matrix_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagOperatorReviewPreconditionMatrixSideEffects::none(
            )
        );
    }
}
