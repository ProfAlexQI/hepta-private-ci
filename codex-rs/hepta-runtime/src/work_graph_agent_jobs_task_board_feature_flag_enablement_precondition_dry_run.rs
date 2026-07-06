use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix::{
    WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE,
    hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_report,
};
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE: &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_denial_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDryRunReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_rollback_replay_gate: &'static str,
    pub source_rollback_replay_check_count: usize,
    pub source_pre_enable_blocker_count: usize,
    pub decision_count: usize,
    pub deny_reason_count: usize,
    pub required_prior_gate_count: usize,
    pub decisions: Vec<WorkGraphFeatureFlagEnablementDryRunDecisionPreview>,
    pub deny_reasons: Vec<WorkGraphFeatureFlagEnablementDenyReasonPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub dry_run_mode: &'static str,
    pub dry_run_enforced: bool,
    pub allow_count: usize,
    pub deny_count: usize,
    pub config_write_allowed: bool,
    pub feature_flag_enablement_allowed: bool,
    pub canary_traffic_allowed: bool,
    pub live_cutover_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub replay_execution_allowed: bool,
    pub rollback_execution_allowed: bool,
    pub ready_for_denial_readback: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects: WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDryRunSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementDryRunDecisionPreview {
    pub id: &'static str,
    pub flag_id: &'static str,
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub requested_action: &'static str,
    pub decision: &'static str,
    pub explanation: &'static str,
    pub deny_reason_ids: Vec<&'static str>,
    pub canary_stage: &'static str,
    pub requested_traffic_ppm: u32,
    pub allowed_traffic_ppm: u32,
    pub config_write_allowed: bool,
    pub feature_flag_enablement_allowed: bool,
    pub canary_traffic_allowed: bool,
    pub live_cutover_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagEnablementDenyReasonPreview {
    pub id: &'static str,
    pub blocker_class: &'static str,
    pub required: bool,
    pub satisfied: bool,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDryRunSideEffects {
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDryRunReport {
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_report(
        );
    let decisions = work_graph_agent_jobs_task_board_feature_flag_enablement_dry_run_decisions();
    let deny_reasons = work_graph_agent_jobs_task_board_feature_flag_enablement_deny_reasons();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_enablement_required_prior_gates();
    let deny_count = decisions
        .iter()
        .filter(|decision| decision.decision == "deny")
        .count();

    WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDryRunReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_SCHEMA_VERSION,
        preview_mode: "feature_flag_enablement_precondition_dry_run_deny_no_write_no_enablement",
        source_rollback_replay_gate: source.gate,
        source_rollback_replay_check_count: source.rollback_replay_check_count,
        source_pre_enable_blocker_count: source.pre_enable_blocker_count,
        decision_count: decisions.len(),
        deny_reason_count: deny_reasons.len(),
        required_prior_gate_count: required_prior_gates.len(),
        decisions,
        deny_reasons,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ENABLEMENT_PRECONDITION_DRY_RUN_RECOMMENDED_NEXT_GATE,
        dry_run_mode: "deny_only_precondition_explanation",
        dry_run_enforced: false,
        allow_count: 0,
        deny_count,
        config_write_allowed: false,
        feature_flag_enablement_allowed: false,
        canary_traffic_allowed: false,
        live_cutover_allowed: false,
        approval_acceptance_allowed: false,
        replay_execution_allowed: false,
        rollback_execution_allowed: false,
        ready_for_denial_readback: true,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDryRunSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_dry_run_decisions()
-> Vec<WorkGraphFeatureFlagEnablementDryRunDecisionPreview> {
    vec![
        enablement_decision(
            "agent_jobs_feature_flag_enablement_precondition_dry_run",
            "work_graph_agent_jobs_non_blocking_canary",
            "agent_jobs_batch_workers",
            "report_agent_job_result",
        ),
        enablement_decision(
            "task_board_feature_flag_enablement_precondition_dry_run",
            "work_graph_task_board_non_blocking_canary",
            "hepta_runtime_task_board",
            "task_board_terminal_event",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_deny_reasons()
-> Vec<WorkGraphFeatureFlagEnablementDenyReasonPreview> {
    vec![
        deny_reason(
            "operator_packet_unsent",
            "operator_packet_boundary",
            "operator packet is visible for readback but has not been sent",
        ),
        deny_reason(
            "operator_packet_unaccepted",
            "operator_packet_boundary",
            "operator packet has no acceptance or authoritative approval state",
        ),
        deny_reason(
            "approval_record_missing",
            "approval_boundary",
            "no operator approval record exists for feature-flag enablement",
        ),
        deny_reason(
            "config_write_disabled",
            "config_boundary",
            "feature-flag config writes remain disabled",
        ),
        deny_reason(
            "feature_flag_state_current_off",
            "feature_flag_boundary",
            "feature flag current state remains off",
        ),
        deny_reason(
            "canary_traffic_zero_ppm",
            "traffic_boundary",
            "non-blocking canary traffic remains 0ppm",
        ),
        deny_reason(
            "replay_not_executed",
            "replay_boundary",
            "replay is required but only previewed by the blocker matrix",
        ),
        deny_reason(
            "rollback_not_executed",
            "rollback_boundary",
            "rollback rehearsal is required but not executed",
        ),
        deny_reason(
            "scheduler_admission_not_enforced",
            "scheduler_boundary",
            "scheduler admission remains dry-run only",
        ),
        deny_reason(
            "live_cutover_disabled",
            "live_cutover_boundary",
            "live WorkGraph cutover remains disabled",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_enablement_required_prior_gates()
-> Vec<&'static str> {
    vec![
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

impl WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDryRunSideEffects {
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

fn enablement_decision(
    id: &'static str,
    flag_id: &'static str,
    source_surface_id: &'static str,
    entrypoint_id: &'static str,
) -> WorkGraphFeatureFlagEnablementDryRunDecisionPreview {
    WorkGraphFeatureFlagEnablementDryRunDecisionPreview {
        id,
        flag_id,
        source_surface_id,
        entrypoint_id,
        requested_action: "enable_feature_flag_non_blocking_canary",
        decision: "deny",
        explanation: "dry-run denies enablement until operator acceptance, config write authorization, replay/rollback execution, scheduler enforcement, and live cutover preconditions are explicit",
        deny_reason_ids: vec![
            "operator_packet_unsent",
            "operator_packet_unaccepted",
            "approval_record_missing",
            "config_write_disabled",
            "feature_flag_state_current_off",
            "canary_traffic_zero_ppm",
            "replay_not_executed",
            "rollback_not_executed",
            "scheduler_admission_not_enforced",
            "live_cutover_disabled",
        ],
        canary_stage: "shadow_0ppm_report_only",
        requested_traffic_ppm: 1,
        allowed_traffic_ppm: 0,
        config_write_allowed: false,
        feature_flag_enablement_allowed: false,
        canary_traffic_allowed: false,
        live_cutover_allowed: false,
    }
}

fn deny_reason(
    id: &'static str,
    blocker_class: &'static str,
    explanation: &'static str,
) -> WorkGraphFeatureFlagEnablementDenyReasonPreview {
    WorkGraphFeatureFlagEnablementDenyReasonPreview {
        id,
        blocker_class,
        required: true,
        satisfied: false,
        explanation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enablement_precondition_dry_run_derives_from_blocker_matrix() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_report(
            );

        assert_eq!(
            report.source_rollback_replay_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE
        );
        assert_eq!(report.source_rollback_replay_check_count, 6);
        assert_eq!(report.source_pre_enable_blocker_count, 10);
        assert_eq!(report.decision_count, 2);
        assert_eq!(report.deny_reason_count, 10);
        assert_eq!(report.dry_run_mode, "deny_only_precondition_explanation");
        assert!(!report.dry_run_enforced);
    }

    #[test]
    fn enablement_precondition_dry_run_denies_both_canary_flags() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_report(
            );
        let flag_ids = report
            .decisions
            .iter()
            .map(|decision| decision.flag_id)
            .collect::<Vec<_>>();

        assert_eq!(report.allow_count, 0);
        assert_eq!(report.deny_count, 2);
        assert!(flag_ids.contains(&"work_graph_agent_jobs_non_blocking_canary"));
        assert!(flag_ids.contains(&"work_graph_task_board_non_blocking_canary"));
        assert!(report.decisions.iter().all(|decision| {
            decision.decision == "deny"
                && decision.deny_reason_ids.len() == 10
                && decision.canary_stage == "shadow_0ppm_report_only"
                && decision.requested_traffic_ppm == 1
                && decision.allowed_traffic_ppm == 0
                && !decision.config_write_allowed
                && !decision.feature_flag_enablement_allowed
                && !decision.canary_traffic_allowed
                && !decision.live_cutover_allowed
        }));
    }

    #[test]
    fn enablement_precondition_dry_run_requires_priors_and_keeps_readiness_false() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_report(
            );

        assert_eq!(
            report.required_prior_gates,
            vec![
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
        assert_eq!(report.required_prior_gate_count, 9);
        assert!(report.ready_for_denial_readback);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
        assert!(
            report
                .deny_reasons
                .iter()
                .all(|reason| { reason.required && !reason.satisfied })
        );
    }

    #[test]
    fn enablement_precondition_dry_run_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_report(
            );

        assert!(!report.config_write_allowed);
        assert!(!report.feature_flag_enablement_allowed);
        assert!(!report.canary_traffic_allowed);
        assert!(!report.live_cutover_allowed);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.rollback_execution_allowed);
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagEnablementPreconditionDryRunSideEffects::none()
        );
    }
}
