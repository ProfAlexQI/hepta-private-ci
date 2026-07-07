use serde::Serialize;

use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects;
use crate::work_graph_scheduler_admission_dry_run_enforcement::hepta_work_graph_scheduler_admission_dry_run_enforcement_report;
use crate::work_graph_task_result_envelope_report_only_validator::WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE;
use crate::work_graph_task_result_envelope_report_only_validator::WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects;
use crate::work_graph_task_result_envelope_report_only_validator::hepta_work_graph_task_result_envelope_report_only_validator_report;
use crate::work_graph_trace_guardrail_span_report_only::WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE;
use crate::work_graph_trace_guardrail_span_report_only::WorkGraphTraceGuardrailSpanReportOnlySideEffects;
use crate::work_graph_trace_guardrail_span_report_only::hepta_work_graph_trace_guardrail_span_report_only_report;

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
    pub source_trace_guardrail_required_prior_gate_count: usize,
    pub source_trace_guardrail_span_count: usize,
    pub source_trace_guardrail_blocking_guardrail_count: usize,
    pub source_task_result_envelope_source_adapter_count: usize,
    pub source_task_result_envelope_source_envelope_count: usize,
    pub source_scheduler_admission_entrypoint_count: usize,
    pub source_scheduler_admission_required_prior_gate_count: usize,
    pub canonical_wire_fields: Vec<&'static str>,
    pub emissions: Vec<WorkGraphReportOnlyEntrypointEmissionPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub source_trace_guardrail_gate: &'static str,
    pub source_task_result_envelope_validator_gate: &'static str,
    pub source_scheduler_admission_dry_run_gate: &'static str,
    pub recommended_next_gate: &'static str,
    pub source_trace_guardrail_readiness_complete: bool,
    pub source_trace_guardrail_no_live_blocking_confirmed: bool,
    pub source_task_result_envelope_validator_ready: bool,
    pub source_task_result_envelope_no_enforcement_confirmed: bool,
    pub source_scheduler_admission_dry_run_ready: bool,
    pub source_scheduler_admission_no_live_blocking_confirmed: bool,
    pub entrypoint_emission_prior_readbacks_complete: bool,
    pub agent_jobs_report_only_emission_attached: bool,
    pub task_board_report_only_emission_attached: bool,
    pub entrypoint_emission_readiness_complete: bool,
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
    let trace_guardrail = hepta_work_graph_trace_guardrail_span_report_only_report();
    let task_result_envelope = hepta_work_graph_task_result_envelope_report_only_validator_report();
    let scheduler_admission = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();
    let source_trace_guardrail_no_live_blocking_confirmed = !trace_guardrail
        .live_guardrail_enforcement_enabled
        && !trace_guardrail.ready_for_live_execution
        && trace_guardrail.side_effects == WorkGraphTraceGuardrailSpanReportOnlySideEffects::none();
    let source_trace_guardrail_readiness_complete = trace_guardrail.gate
        == WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE
        && trace_guardrail.trace_guardrail_prior_readbacks_complete
        && trace_guardrail.ready_for_agent_jobs_task_board_report_only_emission
        && source_trace_guardrail_no_live_blocking_confirmed;
    let source_task_result_envelope_no_enforcement_confirmed = !task_result_envelope
        .live_enforcement_enabled
        && !task_result_envelope.ready_for_live_execution
        && task_result_envelope.side_effects
            == WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects::none();
    let source_task_result_envelope_validator_ready = task_result_envelope.gate
        == WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE
        && task_result_envelope.ready_for_scheduler_admission_dry_run_enforcement
        && task_result_envelope.report_only_valid_source_count
            == task_result_envelope.source_envelope_count
        && source_task_result_envelope_no_enforcement_confirmed;
    let source_scheduler_admission_no_live_blocking_confirmed = !scheduler_admission
        .live_blocking_enforcement_enabled
        && !scheduler_admission.ready_for_live_execution
        && scheduler_admission.side_effects
            == WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects::none();
    let source_scheduler_admission_dry_run_ready = scheduler_admission.gate
        == WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE
        && scheduler_admission.dry_run_enforcement_enabled
        && scheduler_admission.ready_for_append_only_event_store_shadow_path
        && source_scheduler_admission_no_live_blocking_confirmed;
    let entrypoint_emission_prior_readbacks_complete = source_trace_guardrail_readiness_complete
        && source_task_result_envelope_validator_ready
        && source_scheduler_admission_dry_run_ready;
    let agent_jobs_report_only_emission_attached = entrypoint_emission_prior_readbacks_complete
        && emissions.iter().any(|emission| {
            emission.source_surface_id == "agent_jobs_batch_workers"
                && emission.entrypoint_id == "report_agent_job_result"
                && emission.actual_runtime_hook_attached
                && emission.report_only_attached
                && !emission.live_blocking_enabled
                && !emission.persistence_enabled
        });
    let task_board_report_only_emission_attached = entrypoint_emission_prior_readbacks_complete
        && emissions.iter().any(|emission| {
            emission.source_surface_id == "hepta_runtime_task_board"
                && emission.entrypoint_id == "task_board_terminal_event"
                && emission.actual_runtime_hook_attached
                && emission.report_only_attached
                && !emission.live_blocking_enabled
                && !emission.persistence_enabled
        });
    let entrypoint_emission_readiness_complete = agent_jobs_report_only_emission_attached
        && task_board_report_only_emission_attached
        && !canonical_wire_fields.is_empty();

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
        source_trace_guardrail_required_prior_gate_count: trace_guardrail.required_prior_gate_count,
        source_trace_guardrail_span_count: trace_guardrail.span_count,
        source_trace_guardrail_blocking_guardrail_count: trace_guardrail.blocking_guardrail_count,
        source_task_result_envelope_source_adapter_count: task_result_envelope.source_adapter_count,
        source_task_result_envelope_source_envelope_count: task_result_envelope
            .source_envelope_count,
        source_scheduler_admission_entrypoint_count: scheduler_admission.entrypoint_count,
        source_scheduler_admission_required_prior_gate_count: scheduler_admission
            .required_prior_gates
            .len(),
        canonical_wire_fields,
        emissions,
        required_prior_gates,
        source_trace_guardrail_gate: trace_guardrail.gate,
        source_task_result_envelope_validator_gate: task_result_envelope.gate,
        source_scheduler_admission_dry_run_gate: scheduler_admission.gate,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_RECOMMENDED_NEXT_GATE,
        source_trace_guardrail_readiness_complete,
        source_trace_guardrail_no_live_blocking_confirmed,
        source_task_result_envelope_validator_ready,
        source_task_result_envelope_no_enforcement_confirmed,
        source_scheduler_admission_dry_run_ready,
        source_scheduler_admission_no_live_blocking_confirmed,
        entrypoint_emission_prior_readbacks_complete,
        agent_jobs_report_only_emission_attached,
        task_board_report_only_emission_attached,
        entrypoint_emission_readiness_complete,
        ready_for_canary_readback_replay_gate: entrypoint_emission_readiness_complete,
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
    fn agent_jobs_task_board_emission_consumes_prior_report_readbacks() {
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
        assert_eq!(
            report.source_trace_guardrail_gate,
            WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE
        );
        assert_eq!(
            report.source_task_result_envelope_validator_gate,
            WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE
        );
        assert_eq!(
            report.source_scheduler_admission_dry_run_gate,
            WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE
        );
        assert_eq!(report.source_trace_guardrail_required_prior_gate_count, 4);
        assert_eq!(report.source_trace_guardrail_span_count, 9);
        assert_eq!(report.source_trace_guardrail_blocking_guardrail_count, 6);
        assert_eq!(report.source_task_result_envelope_source_adapter_count, 7);
        assert_eq!(report.source_task_result_envelope_source_envelope_count, 7);
        assert_eq!(report.source_scheduler_admission_entrypoint_count, 4);
        assert_eq!(
            report.source_scheduler_admission_required_prior_gate_count,
            5
        );
        assert!(report.source_trace_guardrail_readiness_complete);
        assert!(report.source_trace_guardrail_no_live_blocking_confirmed);
        assert!(report.source_task_result_envelope_validator_ready);
        assert!(report.source_task_result_envelope_no_enforcement_confirmed);
        assert!(report.source_scheduler_admission_dry_run_ready);
        assert!(report.source_scheduler_admission_no_live_blocking_confirmed);
        assert!(report.entrypoint_emission_prior_readbacks_complete);
    }

    #[test]
    fn agent_jobs_task_board_emission_links_trace_guardrail_prior() {
        let report =
            hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report();

        assert!(report.agent_jobs_report_only_emission_attached);
        assert!(report.task_board_report_only_emission_attached);
        assert!(report.entrypoint_emission_readiness_complete);
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
