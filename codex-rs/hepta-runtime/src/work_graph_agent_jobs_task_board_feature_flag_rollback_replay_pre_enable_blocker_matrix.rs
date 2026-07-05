use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback::hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_report;
use crate::work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_REPORT_ONLY_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE:
    &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_enablement_precondition_dry_run_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagRollbackReplayPreEnableBlockerMatrixReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_non_send_readback_gate: &'static str,
    pub source_readback_entry_count: usize,
    pub source_readback_blocker_count: usize,
    pub rollback_replay_check_count: usize,
    pub pre_enable_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub rollback_replay_checks: Vec<WorkGraphFeatureFlagRollbackReplayCheckPreview>,
    pub pre_enable_blockers: Vec<WorkGraphFeatureFlagPreEnableBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub rollback_anchor_present: bool,
    pub deterministic_replay_required: bool,
    pub replay_diff_required: bool,
    pub rollback_rehearsal_required: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub ready_for_enablement_precondition_dry_run: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary_traffic: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardFeatureFlagRollbackReplayPreEnableBlockerMatrixSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagRollbackReplayCheckPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub check_kind: &'static str,
    pub expected_state: &'static str,
    pub rollback_anchor_ref: &'static str,
    pub deterministic: bool,
    pub diff_required: bool,
    pub executed: bool,
    pub passed_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagPreEnableBlockerPreview {
    pub id: &'static str,
    pub blocked_action: &'static str,
    pub blocker_class: &'static str,
    pub blocked: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagRollbackReplayPreEnableBlockerMatrixSideEffects {
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

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagRollbackReplayPreEnableBlockerMatrixReport {
    let source =
        hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_report(
        );
    let rollback_replay_checks =
        work_graph_agent_jobs_task_board_feature_flag_rollback_replay_checks();
    let pre_enable_blockers = work_graph_agent_jobs_task_board_feature_flag_pre_enable_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_rollback_replay_required_prior_gates();

    WorkGraphAgentJobsTaskBoardFeatureFlagRollbackReplayPreEnableBlockerMatrixReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_SCHEMA_VERSION,
        preview_mode: "rollback_replay_pre_enable_blocker_matrix_no_execution_no_enablement",
        source_non_send_readback_gate: source.gate,
        source_readback_entry_count: source.readback_entry_count,
        source_readback_blocker_count: source.readback_blocker_count,
        rollback_replay_check_count: rollback_replay_checks.len(),
        pre_enable_blocker_count: pre_enable_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        rollback_replay_checks,
        pre_enable_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_ROLLBACK_REPLAY_PRE_ENABLE_BLOCKER_MATRIX_RECOMMENDED_NEXT_GATE,
        rollback_anchor_present: true,
        deterministic_replay_required: true,
        replay_diff_required: true,
        rollback_rehearsal_required: true,
        replay_executed: false,
        rollback_executed: false,
        ready_for_enablement_precondition_dry_run: true,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary_traffic: false,
        ready_for_live_cutover: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardFeatureFlagRollbackReplayPreEnableBlockerMatrixSideEffects::none(
            ),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_rollback_replay_checks()
-> Vec<WorkGraphFeatureFlagRollbackReplayCheckPreview> {
    vec![
        rollback_replay_check(
            "operator_packet_non_send_replay_check",
            "work_graph_agent_jobs_task_board_feature_flag_operator_packet",
            "operator_packet_non_send_replay",
            "visible_unsent_unrecorded_unpersisted_non_authoritative",
        ),
        rollback_replay_check(
            "config_contract_digest_replay_check",
            "work_graph_agent_jobs_task_board_canary_flags",
            "config_contract_digest_replay",
            "default_off_current_off_zero_ppm_digest_unpersisted",
        ),
        rollback_replay_check(
            "agent_jobs_entrypoint_projection_replay_check",
            "agent_jobs_batch_workers",
            "entrypoint_projection_replay",
            "work_graph_report_only_metadata_no_blocking",
        ),
        rollback_replay_check(
            "task_board_entrypoint_projection_replay_check",
            "hepta_runtime_task_board",
            "entrypoint_projection_replay",
            "terminal_event_report_only_metadata_no_blocking",
        ),
        rollback_replay_check(
            "trace_guardrail_span_replay_check",
            "work_graph_trace_guardrail_span",
            "trace_guardrail_span_replay",
            "blocking_guardrail_preview_without_enforcement",
        ),
        rollback_replay_check(
            "scheduler_admission_dry_run_replay_check",
            "work_graph_scheduler_admission",
            "admission_dry_run_replay",
            "allow_deny_explanations_without_enforcement",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_pre_enable_blockers()
-> Vec<WorkGraphFeatureFlagPreEnableBlockerPreview> {
    vec![
        pre_enable_blocker(
            "operator_packet_send_pre_enable_blocker",
            "send_operator_packet",
            "operator_packet_boundary",
            "operator packet send remains disabled before explicit approval path",
        ),
        pre_enable_blocker(
            "operator_packet_acceptance_pre_enable_blocker",
            "accept_operator_packet",
            "operator_packet_boundary",
            "operator packet acceptance is not recorded by this matrix",
        ),
        pre_enable_blocker(
            "approval_recording_pre_enable_blocker",
            "record_operator_approval",
            "approval_boundary",
            "no approval record exists for enablement",
        ),
        pre_enable_blocker(
            "config_write_pre_enable_blocker",
            "write_feature_flag_config",
            "config_boundary",
            "feature flag config writes remain disabled",
        ),
        pre_enable_blocker(
            "feature_flag_mutation_pre_enable_blocker",
            "mutate_feature_flag_state",
            "feature_flag_boundary",
            "feature flag current state remains off",
        ),
        pre_enable_blocker(
            "canary_traffic_pre_enable_blocker",
            "route_canary_traffic",
            "traffic_boundary",
            "canary traffic remains 0ppm",
        ),
        pre_enable_blocker(
            "replay_execution_pre_enable_blocker",
            "execute_replay",
            "replay_boundary",
            "this matrix previews replay requirements without execution",
        ),
        pre_enable_blocker(
            "rollback_execution_pre_enable_blocker",
            "execute_rollback",
            "rollback_boundary",
            "this matrix previews rollback requirements without execution",
        ),
        pre_enable_blocker(
            "scheduler_enforcement_pre_enable_blocker",
            "enforce_scheduler_admission",
            "scheduler_boundary",
            "admission remains dry-run only",
        ),
        pre_enable_blocker(
            "live_cutover_pre_enable_blocker",
            "perform_live_cutover",
            "live_cutover_boundary",
            "live WorkGraph cutover remains disabled",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_rollback_replay_required_prior_gates()
-> Vec<&'static str> {
    vec![
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

impl WorkGraphAgentJobsTaskBoardFeatureFlagRollbackReplayPreEnableBlockerMatrixSideEffects {
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

fn rollback_replay_check(
    id: &'static str,
    source_surface_id: &'static str,
    check_kind: &'static str,
    expected_state: &'static str,
) -> WorkGraphFeatureFlagRollbackReplayCheckPreview {
    WorkGraphFeatureFlagRollbackReplayCheckPreview {
        id,
        source_surface_id,
        check_kind,
        expected_state,
        rollback_anchor_ref: "agent_jobs_task_board_feature_flag_canary_rollback_anchor",
        deterministic: true,
        diff_required: true,
        executed: false,
        passed_preview: true,
    }
}

fn pre_enable_blocker(
    id: &'static str,
    blocked_action: &'static str,
    blocker_class: &'static str,
    reason: &'static str,
) -> WorkGraphFeatureFlagPreEnableBlockerPreview {
    WorkGraphFeatureFlagPreEnableBlockerPreview {
        id,
        blocked_action,
        blocker_class,
        blocked: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rollback_replay_matrix_derives_from_non_send_readback() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_report();

        assert_eq!(
            report.source_non_send_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_OPERATOR_PACKET_NON_SEND_READBACK_GATE
        );
        assert_eq!(report.source_readback_entry_count, 4);
        assert_eq!(report.source_readback_blocker_count, 8);
        assert!(report.rollback_anchor_present);
        assert!(report.deterministic_replay_required);
        assert!(report.replay_diff_required);
        assert!(report.rollback_rehearsal_required);
        assert!(!report.replay_executed);
        assert!(!report.rollback_executed);
    }

    #[test]
    fn rollback_replay_matrix_checks_are_preview_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_report();

        assert_eq!(report.rollback_replay_check_count, 6);
        assert!(report.rollback_replay_checks.iter().all(|check| {
            check.deterministic && check.diff_required && !check.executed && check.passed_preview
        }));
        assert!(report.ready_for_enablement_precondition_dry_run);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_canary_traffic);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn rollback_replay_matrix_blocks_all_pre_enable_mutations() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_report();

        assert_eq!(report.pre_enable_blocker_count, 10);
        assert!(
            report
                .pre_enable_blockers
                .iter()
                .all(|blocker| blocker.blocked)
        );
        assert_eq!(
            report.required_prior_gates,
            vec![
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
        assert_eq!(report.required_prior_gate_count, 8);
    }

    #[test]
    fn rollback_replay_matrix_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_rollback_replay_pre_enable_blocker_matrix_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagRollbackReplayPreEnableBlockerMatrixSideEffects::none(
            )
        );
    }
}
