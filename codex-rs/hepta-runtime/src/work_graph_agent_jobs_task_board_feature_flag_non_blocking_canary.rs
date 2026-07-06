use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE: &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_SCHEMA_VERSION: &str =
    "work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanaryReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub feature_flag_count: usize,
    pub rollout_stage_count: usize,
    pub emission_binding_count: usize,
    pub safety_check_count: usize,
    pub required_prior_gate_count: usize,
    pub feature_flags: Vec<WorkGraphCanaryFeatureFlagPreview>,
    pub rollout_stages: Vec<WorkGraphCanaryFeatureFlagStagePreview>,
    pub emission_bindings: Vec<WorkGraphCanaryFeatureFlagEmissionBindingPreview>,
    pub safety_checks: Vec<WorkGraphCanaryFeatureFlagSafetyCheckPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_feature_flag_config_wiring: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects: WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanarySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanaryFeatureFlagPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub config_surface_id: &'static str,
    pub default_enabled: bool,
    pub current_enabled: bool,
    pub canary_stage_id: &'static str,
    pub traffic_ppm: u32,
    pub required_before_enablement: Vec<&'static str>,
    pub allows_live_blocking: bool,
    pub allows_persistence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanaryFeatureFlagStagePreview {
    pub id: &'static str,
    pub order: usize,
    pub traffic_ppm: u32,
    pub mode: &'static str,
    pub blocks_runtime_mutation: bool,
    pub requires_readback_gate: bool,
    pub requires_rollback_replay_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanaryFeatureFlagEmissionBindingPreview {
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub report_only_field: &'static str,
    pub feature_flag_id: &'static str,
    pub feature_flag_field_attached: bool,
    pub readback_field_attached: bool,
    pub rollback_replay_field_attached: bool,
    pub live_cutover_field_attached: bool,
    pub live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanaryFeatureFlagSafetyCheckPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanarySideEffects {
    pub filesystem_written: bool,
    pub feature_flag_mutated: bool,
    pub non_blocking_canary_enabled: bool,
    pub live_cutover_enabled: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanaryReport {
    let feature_flags = work_graph_agent_jobs_task_board_non_blocking_canary_feature_flags();
    let rollout_stages = work_graph_agent_jobs_task_board_non_blocking_canary_rollout_stages();
    let emission_bindings =
        work_graph_agent_jobs_task_board_non_blocking_canary_emission_bindings();
    let safety_checks = work_graph_agent_jobs_task_board_non_blocking_canary_safety_checks();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_non_blocking_canary_required_prior_gates();

    WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanaryReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_SCHEMA_VERSION,
        preview_mode: "feature_flag_non_blocking_canary_report_only_no_flag_mutation",
        feature_flag_count: feature_flags.len(),
        rollout_stage_count: rollout_stages.len(),
        emission_binding_count: emission_bindings.len(),
        safety_check_count: safety_checks.len(),
        required_prior_gate_count: required_prior_gates.len(),
        feature_flags,
        rollout_stages,
        emission_bindings,
        safety_checks,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_RECOMMENDED_NEXT_GATE,
        ready_for_feature_flag_config_wiring: true,
        ready_for_feature_flag_enablement: false,
        ready_for_live_cutover: false,
        side_effects: WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanarySideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_non_blocking_canary_feature_flags()
-> Vec<WorkGraphCanaryFeatureFlagPreview> {
    vec![
        feature_flag(
            "work_graph_agent_jobs_non_blocking_canary",
            "agent_jobs_batch_workers",
            "report_agent_job_result",
            vec![
                "canary_readback_replay_gate_green",
                "operator_review_packet",
                "rollback_replay_gate_green",
                "feature_flag_config_digest",
            ],
        ),
        feature_flag(
            "work_graph_task_board_non_blocking_canary",
            "hepta_runtime_task_board",
            "task_board_terminal_event",
            vec![
                "canary_readback_replay_gate_green",
                "operator_review_packet",
                "rollback_replay_gate_green",
                "feature_flag_config_digest",
            ],
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_non_blocking_canary_rollout_stages()
-> Vec<WorkGraphCanaryFeatureFlagStagePreview> {
    vec![WorkGraphCanaryFeatureFlagStagePreview {
        id: "shadow_0ppm_report_only",
        order: 0,
        traffic_ppm: 0,
        mode: "report_only_observation",
        blocks_runtime_mutation: true,
        requires_readback_gate: true,
        requires_rollback_replay_gate: true,
    }]
}

pub fn work_graph_agent_jobs_task_board_non_blocking_canary_emission_bindings()
-> Vec<WorkGraphCanaryFeatureFlagEmissionBindingPreview> {
    vec![
        emission_binding(
            "agent_jobs_batch_workers",
            "report_agent_job_result",
            "work_graph_agent_jobs_non_blocking_canary",
        ),
        emission_binding(
            "hepta_runtime_task_board",
            "task_board_terminal_event",
            "work_graph_task_board_non_blocking_canary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_non_blocking_canary_safety_checks()
-> Vec<WorkGraphCanaryFeatureFlagSafetyCheckPreview> {
    vec![
        safety_check(
            "feature_flags_default_off",
            "canary flags must be present in report-only metadata but disabled by default",
        ),
        safety_check(
            "non_blocking_canary_traffic_zero_ppm",
            "the first canary stage must observe only and route zero live traffic",
        ),
        safety_check(
            "readback_and_rollback_replay_required",
            "canary enablement must require readback and rollback/replay gates before any runtime mutation",
        ),
        safety_check(
            "live_cutover_remains_false",
            "feature flag metadata must not enable live blocking, persistence, external send, or cutover",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_non_blocking_canary_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
        WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanarySideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            feature_flag_mutated: false,
            non_blocking_canary_enabled: false,
            live_cutover_enabled: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            projection_index_persisted: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            replay_executed: false,
            rollback_executed: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn feature_flag(
    id: &'static str,
    source_surface_id: &'static str,
    entrypoint_id: &'static str,
    required_before_enablement: Vec<&'static str>,
) -> WorkGraphCanaryFeatureFlagPreview {
    WorkGraphCanaryFeatureFlagPreview {
        id,
        source_surface_id,
        entrypoint_id,
        config_surface_id: "work_graph_agent_jobs_task_board_canary_flags",
        default_enabled: false,
        current_enabled: false,
        canary_stage_id: "shadow_0ppm_report_only",
        traffic_ppm: 0,
        required_before_enablement,
        allows_live_blocking: false,
        allows_persistence: false,
    }
}

fn emission_binding(
    source_surface_id: &'static str,
    entrypoint_id: &'static str,
    feature_flag_id: &'static str,
) -> WorkGraphCanaryFeatureFlagEmissionBindingPreview {
    WorkGraphCanaryFeatureFlagEmissionBindingPreview {
        source_surface_id,
        entrypoint_id,
        report_only_field: "workGraphReportOnly",
        feature_flag_id,
        feature_flag_field_attached: true,
        readback_field_attached: true,
        rollback_replay_field_attached: true,
        live_cutover_field_attached: true,
        live_cutover_enabled: false,
    }
}

fn safety_check(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphCanaryFeatureFlagSafetyCheckPreview {
    WorkGraphCanaryFeatureFlagSafetyCheckPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_flag_non_blocking_canary_declares_default_off_flags() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_report();
        let flag_ids = report
            .feature_flags
            .iter()
            .map(|flag| flag.id)
            .collect::<Vec<_>>();

        assert_eq!(report.feature_flag_count, 2);
        assert!(flag_ids.contains(&"work_graph_agent_jobs_non_blocking_canary"));
        assert!(flag_ids.contains(&"work_graph_task_board_non_blocking_canary"));
        assert!(report.feature_flags.iter().all(|flag| {
            !flag.default_enabled
                && !flag.current_enabled
                && flag.traffic_ppm == 0
                && !flag.allows_live_blocking
                && !flag.allows_persistence
        }));
    }

    #[test]
    fn feature_flag_non_blocking_canary_binds_report_only_emissions() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_report();

        assert_eq!(report.emission_binding_count, 2);
        assert!(report.emission_bindings.iter().all(|binding| {
            binding.report_only_field == "workGraphReportOnly"
                && binding.feature_flag_field_attached
                && binding.readback_field_attached
                && binding.rollback_replay_field_attached
                && binding.live_cutover_field_attached
                && !binding.live_cutover_enabled
        }));
    }

    #[test]
    fn feature_flag_non_blocking_canary_requires_readback_replay_and_prior_gates() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_report();

        assert_eq!(report.rollout_stage_count, 1);
        assert!(report.rollout_stages.iter().all(|stage| {
            stage.id == "shadow_0ppm_report_only"
                && stage.traffic_ppm == 0
                && stage.blocks_runtime_mutation
                && stage.requires_readback_gate
                && stage.requires_rollback_replay_gate
        }));
        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
                WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
            ]
        );
        assert_eq!(report.safety_check_count, 4);
        assert!(report.ready_for_feature_flag_config_wiring);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn feature_flag_non_blocking_canary_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagNonBlockingCanarySideEffects::none()
        );
    }
}
