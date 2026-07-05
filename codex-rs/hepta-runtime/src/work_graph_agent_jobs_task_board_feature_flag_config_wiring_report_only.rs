use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE: &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlyReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub config_contract_count: usize,
    pub config_digest_preview_count: usize,
    pub source_binding_count: usize,
    pub safety_check_count: usize,
    pub required_prior_gate_count: usize,
    pub config_contracts: Vec<WorkGraphFeatureFlagConfigContractPreview>,
    pub config_digest_previews: Vec<WorkGraphFeatureFlagConfigDigestPreview>,
    pub source_bindings: Vec<WorkGraphFeatureFlagConfigSourceBindingPreview>,
    pub safety_checks: Vec<WorkGraphFeatureFlagConfigSafetyCheckPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_operator_packet_report_only: bool,
    pub ready_for_feature_flag_config_write: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects: WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagConfigContractPreview {
    pub id: &'static str,
    pub flag_id: &'static str,
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub owner: &'static str,
    pub config_surface_id: &'static str,
    pub default_enabled: bool,
    pub current_enabled: bool,
    pub canary_stage_id: &'static str,
    pub traffic_ppm: u32,
    pub readback_required: bool,
    pub rollback_replay_required: bool,
    pub operator_packet_required: bool,
    pub config_digest_required: bool,
    pub config_written: bool,
    pub live_mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagConfigDigestPreview {
    pub id: &'static str,
    pub flag_id: &'static str,
    pub digest_algorithm: &'static str,
    pub digest_source: &'static str,
    pub redaction_policy: &'static str,
    pub deterministic_input_count: usize,
    pub digest_required_before_enablement: bool,
    pub digest_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagConfigSourceBindingPreview {
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub metadata_field: &'static str,
    pub config_contract_id: &'static str,
    pub field_present: bool,
    pub default_off_observed: bool,
    pub zero_traffic_observed: bool,
    pub readback_observed: bool,
    pub rollback_replay_observed: bool,
    pub live_cutover_observed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureFlagConfigSafetyCheckPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlySideEffects {
    pub filesystem_written: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub non_blocking_canary_enabled: bool,
    pub live_cutover_enabled: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub config_digest_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub approval_recorded: bool,
    pub operator_packet_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_report()
-> WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlyReport {
    let config_contracts = work_graph_agent_jobs_task_board_feature_flag_config_contracts();
    let config_digest_previews =
        work_graph_agent_jobs_task_board_feature_flag_config_digest_previews();
    let source_bindings = work_graph_agent_jobs_task_board_feature_flag_config_source_bindings();
    let safety_checks = work_graph_agent_jobs_task_board_feature_flag_config_safety_checks();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_feature_flag_config_required_prior_gates();

    WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlyReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_SCHEMA_VERSION,
        preview_mode: "feature_flag_config_wiring_report_only_no_config_write",
        config_contract_count: config_contracts.len(),
        config_digest_preview_count: config_digest_previews.len(),
        source_binding_count: source_bindings.len(),
        safety_check_count: safety_checks.len(),
        required_prior_gate_count: required_prior_gates.len(),
        config_contracts,
        config_digest_previews,
        source_bindings,
        safety_checks,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_CONFIG_WIRING_REPORT_ONLY_RECOMMENDED_NEXT_GATE,
        ready_for_operator_packet_report_only: true,
        ready_for_feature_flag_config_write: false,
        ready_for_feature_flag_enablement: false,
        ready_for_live_cutover: false,
        side_effects: WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlySideEffects::none(
        ),
    }
}

pub fn work_graph_agent_jobs_task_board_feature_flag_config_contracts()
-> Vec<WorkGraphFeatureFlagConfigContractPreview> {
    vec![
        config_contract(
            "agent_jobs_feature_flag_config_contract",
            "work_graph_agent_jobs_non_blocking_canary",
            "agent_jobs_batch_workers",
            "report_agent_job_result",
        ),
        config_contract(
            "task_board_feature_flag_config_contract",
            "work_graph_task_board_non_blocking_canary",
            "hepta_runtime_task_board",
            "task_board_terminal_event",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_config_digest_previews()
-> Vec<WorkGraphFeatureFlagConfigDigestPreview> {
    vec![
        config_digest_preview(
            "agent_jobs_feature_flag_config_digest_preview",
            "work_graph_agent_jobs_non_blocking_canary",
        ),
        config_digest_preview(
            "task_board_feature_flag_config_digest_preview",
            "work_graph_task_board_non_blocking_canary",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_config_source_bindings()
-> Vec<WorkGraphFeatureFlagConfigSourceBindingPreview> {
    vec![
        config_source_binding(
            "agent_jobs_batch_workers",
            "report_agent_job_result",
            "agent_jobs_feature_flag_config_contract",
        ),
        config_source_binding(
            "hepta_runtime_task_board",
            "task_board_terminal_event",
            "task_board_feature_flag_config_contract",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_config_safety_checks()
-> Vec<WorkGraphFeatureFlagConfigSafetyCheckPreview> {
    vec![
        safety_check(
            "feature_flag_config_contract_default_off",
            "config wiring can name canary flags, but every contract must remain default/current off",
        ),
        safety_check(
            "feature_flag_config_digest_required_unpersisted",
            "enablement requires a deterministic digest preview without persisting the digest",
        ),
        safety_check(
            "feature_flag_operator_packet_required",
            "operator packet review is required before any flag write, traffic, or live cutover",
        ),
        safety_check(
            "feature_flag_config_wiring_has_no_runtime_mutation",
            "report-only config wiring must not mutate config, traffic, WorkGraph state, or runtime enforcement",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_feature_flag_config_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
        WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlySideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            config_written: false,
            feature_flag_mutated: false,
            non_blocking_canary_enabled: false,
            live_cutover_enabled: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            projection_index_persisted: false,
            config_digest_persisted: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            replay_executed: false,
            rollback_executed: false,
            approval_recorded: false,
            operator_packet_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn config_contract(
    id: &'static str,
    flag_id: &'static str,
    source_surface_id: &'static str,
    entrypoint_id: &'static str,
) -> WorkGraphFeatureFlagConfigContractPreview {
    WorkGraphFeatureFlagConfigContractPreview {
        id,
        flag_id,
        source_surface_id,
        entrypoint_id,
        owner: "hepta-backend",
        config_surface_id: "work_graph_agent_jobs_task_board_canary_flags",
        default_enabled: false,
        current_enabled: false,
        canary_stage_id: "shadow_0ppm_report_only",
        traffic_ppm: 0,
        readback_required: true,
        rollback_replay_required: true,
        operator_packet_required: true,
        config_digest_required: true,
        config_written: false,
        live_mutation_allowed: false,
    }
}

fn config_digest_preview(
    id: &'static str,
    flag_id: &'static str,
) -> WorkGraphFeatureFlagConfigDigestPreview {
    WorkGraphFeatureFlagConfigDigestPreview {
        id,
        flag_id,
        digest_algorithm: "sha256",
        digest_source: "canonical_config_contract_preview",
        redaction_policy: "redact_operator_packet_and_runtime_payloads",
        deterministic_input_count: 9,
        digest_required_before_enablement: true,
        digest_persisted: false,
    }
}

fn config_source_binding(
    source_surface_id: &'static str,
    entrypoint_id: &'static str,
    config_contract_id: &'static str,
) -> WorkGraphFeatureFlagConfigSourceBindingPreview {
    WorkGraphFeatureFlagConfigSourceBindingPreview {
        source_surface_id,
        entrypoint_id,
        metadata_field: "workGraphReportOnly",
        config_contract_id,
        field_present: true,
        default_off_observed: true,
        zero_traffic_observed: true,
        readback_observed: true,
        rollback_replay_observed: true,
        live_cutover_observed: false,
    }
}

fn safety_check(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphFeatureFlagConfigSafetyCheckPreview {
    WorkGraphFeatureFlagConfigSafetyCheckPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_flag_config_wiring_declares_default_off_contracts() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_report();
        let flag_ids = report
            .config_contracts
            .iter()
            .map(|contract| contract.flag_id)
            .collect::<Vec<_>>();

        assert_eq!(report.config_contract_count, 2);
        assert!(flag_ids.contains(&"work_graph_agent_jobs_non_blocking_canary"));
        assert!(flag_ids.contains(&"work_graph_task_board_non_blocking_canary"));
        assert!(report.config_contracts.iter().all(|contract| {
            !contract.default_enabled
                && !contract.current_enabled
                && contract.traffic_ppm == 0
                && contract.readback_required
                && contract.rollback_replay_required
                && contract.operator_packet_required
                && contract.config_digest_required
                && !contract.config_written
                && !contract.live_mutation_allowed
        }));
        assert!(report.ready_for_operator_packet_report_only);
        assert!(!report.ready_for_feature_flag_config_write);
        assert!(!report.ready_for_feature_flag_enablement);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn feature_flag_config_wiring_requires_digest_without_persistence() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_report();

        assert_eq!(report.config_digest_preview_count, 2);
        assert!(report.config_digest_previews.iter().all(|digest| {
            digest.digest_algorithm == "sha256"
                && digest.digest_source == "canonical_config_contract_preview"
                && digest.deterministic_input_count == 9
                && digest.digest_required_before_enablement
                && !digest.digest_persisted
        }));
    }

    #[test]
    fn feature_flag_config_wiring_binds_sources_and_prior_gates() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_report();

        assert_eq!(report.source_binding_count, 2);
        assert!(report.source_bindings.iter().all(|binding| {
            binding.metadata_field == "workGraphReportOnly"
                && binding.field_present
                && binding.default_off_observed
                && binding.zero_traffic_observed
                && binding.readback_observed
                && binding.rollback_replay_observed
                && !binding.live_cutover_observed
        }));
        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_FEATURE_FLAG_NON_BLOCKING_CANARY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
                WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
            ]
        );
    }

    #[test]
    fn feature_flag_config_wiring_has_no_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardFeatureFlagConfigWiringReportOnlySideEffects::none()
        );
    }
}
