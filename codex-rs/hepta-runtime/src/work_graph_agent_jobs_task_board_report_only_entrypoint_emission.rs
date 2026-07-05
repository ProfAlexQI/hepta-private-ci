use serde::Serialize;

use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_task_result_envelope_report_only_validator::WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE: &str =
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_SCHEMA_VERSION: &str =
    "work_graph_agent_jobs_task_board_report_only_entrypoint_emission_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub entrypoint_count: usize,
    pub emission_count: usize,
    pub required_prior_gate_count: usize,
    pub canonical_wire_fields: Vec<&'static str>,
    pub emissions: Vec<WorkGraphReportOnlyEntrypointEmissionPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub agent_jobs_report_only_emission_attached: bool,
    pub task_board_report_only_emission_attached: bool,
    pub ready_for_canary_readback_replay_gate: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphReportOnlyEntrypointEmissionPreview {
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub emission_field: &'static str,
    pub task_result_status_mapping: &'static str,
    pub trace_guardrail_join_fields: Vec<&'static str>,
    pub evidence_refs: Vec<&'static str>,
    pub actual_runtime_hook_attached: bool,
    pub report_only_attached: bool,
    pub live_blocking_enabled: bool,
    pub persistence_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub task_result_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report()
-> WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionReport {
    let canonical_wire_fields = work_graph_agent_jobs_task_board_report_only_emission_wire_fields();
    let emissions = work_graph_agent_jobs_task_board_report_only_emissions();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_report_only_emission_required_prior_gates();

    WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_SCHEMA_VERSION,
        preview_mode: "report_only_entrypoint_emission_no_live_blocking",
        entrypoint_count: emissions.len(),
        emission_count: emissions.len(),
        required_prior_gate_count: required_prior_gates.len(),
        canonical_wire_fields,
        emissions,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_RECOMMENDED_NEXT_GATE,
        agent_jobs_report_only_emission_attached: true,
        task_board_report_only_emission_attached: true,
        ready_for_canary_readback_replay_gate: true,
        ready_for_live_execution: false,
        side_effects: WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_report_only_emission_wire_fields() -> Vec<&'static str> {
    vec![
        "taskId",
        "status",
        "summary",
        "artifacts",
        "evidence",
        "risks",
        "nextActions",
        "verifier",
        "reducer",
        "usage",
        "traceId",
        "spanId",
        "blockingGuardrailPreview",
        "liveBlockingEnabled",
    ]
}

pub fn work_graph_agent_jobs_task_board_report_only_emissions()
-> Vec<WorkGraphReportOnlyEntrypointEmissionPreview> {
    vec![
        emission(
            "agent_jobs_batch_workers",
            "report_agent_job_result",
            "workGraphReportOnly",
            "accepted=true -> succeeded; accepted=false -> blocked",
            vec!["agent_job_id", "agent_job_item_id", "reporting_thread_id"],
        ),
        emission(
            "hepta_runtime_task_board",
            "task_board_terminal_event",
            "workGraphReportOnly",
            "completed/failed/cancelled -> terminal TaskResultEnvelope preview",
            vec!["task_board_event_id", "delivery_id", "readback_evidence_id"],
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_report_only_emission_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            task_result_persisted: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn emission(
    source_surface_id: &'static str,
    entrypoint_id: &'static str,
    emission_field: &'static str,
    task_result_status_mapping: &'static str,
    evidence_refs: Vec<&'static str>,
) -> WorkGraphReportOnlyEntrypointEmissionPreview {
    WorkGraphReportOnlyEntrypointEmissionPreview {
        source_surface_id,
        entrypoint_id,
        emission_field,
        task_result_status_mapping,
        trace_guardrail_join_fields: vec![
            "traceId",
            "spanId",
            "evidence",
            "blockingGuardrailPreview",
        ],
        evidence_refs,
        actual_runtime_hook_attached: true,
        report_only_attached: true,
        live_blocking_enabled: false,
        persistence_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_jobs_task_board_emission_declares_canonical_fields() {
        let fields = work_graph_agent_jobs_task_board_report_only_emission_wire_fields();

        assert_eq!(fields.len(), 14);
        assert!(fields.contains(&"taskId"));
        assert!(fields.contains(&"status"));
        assert!(fields.contains(&"evidence"));
        assert!(fields.contains(&"traceId"));
        assert!(fields.contains(&"spanId"));
        assert!(fields.contains(&"blockingGuardrailPreview"));
    }

    #[test]
    fn agent_jobs_task_board_emission_covers_canary_entrypoints() {
        let report =
            hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report();
        let sources = report
            .emissions
            .iter()
            .map(|emission| emission.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.emission_count, 2);
        assert!(sources.contains(&"agent_jobs_batch_workers"));
        assert!(sources.contains(&"hepta_runtime_task_board"));
        assert!(report.emissions.iter().all(|emission| {
            emission.emission_field == "workGraphReportOnly"
                && emission.actual_runtime_hook_attached
                && emission.report_only_attached
                && !emission.live_blocking_enabled
                && !emission.persistence_enabled
        }));
    }

    #[test]
    fn agent_jobs_task_board_emission_links_trace_guardrail_prior() {
        let report =
            hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
                WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
            ]
        );
        assert!(report.agent_jobs_report_only_emission_attached);
        assert!(report.task_board_report_only_emission_attached);
        assert!(report.ready_for_canary_readback_replay_gate);
    }

    #[test]
    fn agent_jobs_task_board_emission_remains_report_only() {
        let report =
            hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report();

        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionSideEffects::none()
        );
    }
}
