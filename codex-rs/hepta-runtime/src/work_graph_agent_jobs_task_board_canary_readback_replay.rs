use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionSideEffects;
use crate::work_graph_agent_jobs_task_board_report_only_entrypoint_emission::hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report;
use crate::work_graph_append_only_event_store_shadow_path::WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE;
use crate::work_graph_append_only_event_store_shadow_path::WorkGraphAppendOnlyEventStoreShadowPathSideEffects;
use crate::work_graph_append_only_event_store_shadow_path::hepta_work_graph_append_only_event_store_shadow_path_report;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects;
use crate::work_graph_scheduler_admission_dry_run_enforcement::hepta_work_graph_scheduler_admission_dry_run_enforcement_report;
use crate::work_graph_task_result_envelope_report_only_validator::WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE;
use crate::work_graph_task_result_envelope_report_only_validator::WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects;
use crate::work_graph_task_result_envelope_report_only_validator::hepta_work_graph_task_result_envelope_report_only_validator_report;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE: &str =
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_SCHEMA_VERSION: &str =
    "work_graph_agent_jobs_task_board_canary_readback_replay_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardCanaryReadbackReplayReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub canary_entrypoint_count: usize,
    pub readback_evidence_count: usize,
    pub replay_diff_count: usize,
    pub required_prior_gate_count: usize,
    pub source_entrypoint_emission_required_prior_gate_count: usize,
    pub source_entrypoint_emission_entrypoint_count: usize,
    pub source_entrypoint_emission_emission_count: usize,
    pub source_append_only_shadow_path_scheduler_prior_gate_count: usize,
    pub source_append_only_shadow_path_required_prior_gate_count: usize,
    pub source_task_result_envelope_source_adapter_count: usize,
    pub source_task_result_envelope_source_envelope_count: usize,
    pub source_scheduler_admission_entrypoint_count: usize,
    pub source_scheduler_admission_required_prior_gate_count: usize,
    pub canary_entrypoints: Vec<WorkGraphCanaryEntrypointPreview>,
    pub projection_indexes: Vec<WorkGraphCanaryProjectionIndexPreview>,
    pub readback_evidence: Vec<WorkGraphCanaryReadbackEvidencePreview>,
    pub replay_diffs: Vec<WorkGraphCanaryReplayDiffPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub source_entrypoint_emission_gate: &'static str,
    pub source_append_only_shadow_path_gate: &'static str,
    pub source_task_result_envelope_validator_gate: &'static str,
    pub source_scheduler_admission_dry_run_gate: &'static str,
    pub recommended_next_gate: &'static str,
    pub feature_flag_required: bool,
    pub feature_flag_enabled: bool,
    pub source_entrypoint_emission_readiness_complete: bool,
    pub source_entrypoint_emission_no_live_confirmed: bool,
    pub source_append_only_shadow_path_readiness_complete: bool,
    pub source_append_only_shadow_path_no_persistence_confirmed: bool,
    pub source_task_result_envelope_validator_ready: bool,
    pub source_task_result_envelope_no_enforcement_confirmed: bool,
    pub source_scheduler_admission_dry_run_ready: bool,
    pub source_scheduler_admission_no_live_blocking_confirmed: bool,
    pub canary_readback_replay_prior_readbacks_complete: bool,
    pub canary_projection_readback_replay_preview_complete: bool,
    pub ready_for_non_blocking_canary: bool,
    pub ready_for_live_cutover: bool,
    pub side_effects: WorkGraphAgentJobsTaskBoardCanaryReadbackReplaySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanaryEntrypointPreview {
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub report_only_field: &'static str,
    pub admission_decision: &'static str,
    pub trace_join: &'static str,
    pub task_result_preview: &'static str,
    pub rollback_anchor: &'static str,
    pub live_blocking_enabled: bool,
    pub live_persistence_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanaryProjectionIndexPreview {
    pub index_id: &'static str,
    pub source_surface_id: &'static str,
    pub key_fields: Vec<&'static str>,
    pub deterministic_id_rule: &'static str,
    pub redaction_rule: &'static str,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanaryReadbackEvidencePreview {
    pub evidence_id: &'static str,
    pub source_surface_id: &'static str,
    pub checks: Vec<&'static str>,
    pub evidence_status: &'static str,
    pub evidence_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanaryReplayDiffPreview {
    pub diff_id: &'static str,
    pub source_surface_id: &'static str,
    pub replay_scope: &'static str,
    pub expected_diff: &'static str,
    pub replay_executed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardCanaryReadbackReplaySideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub readback_evidence_persisted: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub feature_flag_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report()
-> WorkGraphAgentJobsTaskBoardCanaryReadbackReplayReport {
    let canary_entrypoints = work_graph_agent_jobs_task_board_canary_entrypoints();
    let projection_indexes = work_graph_agent_jobs_task_board_canary_projection_indexes();
    let readback_evidence = work_graph_agent_jobs_task_board_canary_readback_evidence();
    let replay_diffs = work_graph_agent_jobs_task_board_canary_replay_diffs();
    let required_prior_gates = work_graph_agent_jobs_task_board_canary_required_prior_gates();
    let entrypoint_emission =
        hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_report();
    let append_only_shadow_path = hepta_work_graph_append_only_event_store_shadow_path_report();
    let task_result_envelope = hepta_work_graph_task_result_envelope_report_only_validator_report();
    let scheduler_admission = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();
    let source_entrypoint_emission_no_live_confirmed = !entrypoint_emission
        .ready_for_live_execution
        && entrypoint_emission.side_effects
            == WorkGraphAgentJobsTaskBoardReportOnlyEntrypointEmissionSideEffects::none();
    let source_entrypoint_emission_readiness_complete = entrypoint_emission.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE
        && entrypoint_emission.entrypoint_emission_prior_readbacks_complete
        && entrypoint_emission.entrypoint_emission_readiness_complete
        && entrypoint_emission.ready_for_canary_readback_replay_gate
        && source_entrypoint_emission_no_live_confirmed;
    let source_append_only_shadow_path_no_persistence_confirmed = !append_only_shadow_path
        .shadow_store_write_enabled
        && !append_only_shadow_path.live_cutover_enabled
        && !append_only_shadow_path.ready_for_live_execution
        && append_only_shadow_path.side_effects
            == WorkGraphAppendOnlyEventStoreShadowPathSideEffects::none();
    let source_append_only_shadow_path_readiness_complete = append_only_shadow_path.gate
        == WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE
        && append_only_shadow_path.append_only_shadow_path_readiness_complete
        && source_append_only_shadow_path_no_persistence_confirmed;
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
    let canary_readback_replay_prior_readbacks_complete =
        source_entrypoint_emission_readiness_complete
            && source_append_only_shadow_path_readiness_complete
            && source_task_result_envelope_validator_ready
            && source_scheduler_admission_dry_run_ready;
    let canary_projection_readback_replay_preview_complete = !projection_indexes.is_empty()
        && !readback_evidence.is_empty()
        && !replay_diffs.is_empty()
        && projection_indexes.iter().all(|index| !index.persisted)
        && readback_evidence
            .iter()
            .all(|evidence| !evidence.evidence_persisted)
        && replay_diffs.iter().all(|diff| !diff.replay_executed);
    let ready_for_non_blocking_canary = canary_readback_replay_prior_readbacks_complete
        && canary_projection_readback_replay_preview_complete;

    WorkGraphAgentJobsTaskBoardCanaryReadbackReplayReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
        schema_version: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_SCHEMA_VERSION,
        preview_mode: "canary_readback_replay_report_only_no_live_cutover",
        canary_entrypoint_count: canary_entrypoints.len(),
        readback_evidence_count: readback_evidence.len(),
        replay_diff_count: replay_diffs.len(),
        required_prior_gate_count: required_prior_gates.len(),
        source_entrypoint_emission_required_prior_gate_count: entrypoint_emission
            .required_prior_gate_count,
        source_entrypoint_emission_entrypoint_count: entrypoint_emission.entrypoint_count,
        source_entrypoint_emission_emission_count: entrypoint_emission.emission_count,
        source_append_only_shadow_path_scheduler_prior_gate_count: append_only_shadow_path
            .scheduler_prior_gate_count,
        source_append_only_shadow_path_required_prior_gate_count: append_only_shadow_path
            .required_prior_gate_count,
        source_task_result_envelope_source_adapter_count: task_result_envelope.source_adapter_count,
        source_task_result_envelope_source_envelope_count: task_result_envelope
            .source_envelope_count,
        source_scheduler_admission_entrypoint_count: scheduler_admission.entrypoint_count,
        source_scheduler_admission_required_prior_gate_count: scheduler_admission
            .required_prior_gates
            .len(),
        canary_entrypoints,
        projection_indexes,
        readback_evidence,
        replay_diffs,
        required_prior_gates,
        source_entrypoint_emission_gate: entrypoint_emission.gate,
        source_append_only_shadow_path_gate: append_only_shadow_path.gate,
        source_task_result_envelope_validator_gate: task_result_envelope.gate,
        source_scheduler_admission_dry_run_gate: scheduler_admission.gate,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_RECOMMENDED_NEXT_GATE,
        feature_flag_required: true,
        feature_flag_enabled: false,
        source_entrypoint_emission_readiness_complete,
        source_entrypoint_emission_no_live_confirmed,
        source_append_only_shadow_path_readiness_complete,
        source_append_only_shadow_path_no_persistence_confirmed,
        source_task_result_envelope_validator_ready,
        source_task_result_envelope_no_enforcement_confirmed,
        source_scheduler_admission_dry_run_ready,
        source_scheduler_admission_no_live_blocking_confirmed,
        canary_readback_replay_prior_readbacks_complete,
        canary_projection_readback_replay_preview_complete,
        ready_for_non_blocking_canary,
        ready_for_live_cutover: false,
        side_effects: WorkGraphAgentJobsTaskBoardCanaryReadbackReplaySideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_canary_entrypoints() -> Vec<WorkGraphCanaryEntrypointPreview>
{
    vec![
        WorkGraphCanaryEntrypointPreview {
            source_surface_id: "agent_jobs_batch_workers",
            entrypoint_id: "report_agent_job_result",
            report_only_field: "workGraphReportOnly",
            admission_decision: "allow_report_only_no_live_blocking",
            trace_join: "traceId + spanId + agent_job_id + agent_job_item_id",
            task_result_preview: "TaskResultEnvelope report-only emission",
            rollback_anchor: "agent_job_state_db_item_status",
            live_blocking_enabled: false,
            live_persistence_enabled: false,
        },
        WorkGraphCanaryEntrypointPreview {
            source_surface_id: "hepta_runtime_task_board",
            entrypoint_id: "task_board_terminal_event",
            report_only_field: "workGraphReportOnly",
            admission_decision: "allow_report_only_no_live_blocking",
            trace_join: "traceId + spanId + task_board_event_id + delivery_id",
            task_result_preview: "terminal TaskResultEnvelope report-only emission",
            rollback_anchor: "task_board_json_state_terminal_event",
            live_blocking_enabled: false,
            live_persistence_enabled: false,
        },
    ]
}

pub fn work_graph_agent_jobs_task_board_canary_projection_indexes()
-> Vec<WorkGraphCanaryProjectionIndexPreview> {
    vec![
        projection_index(
            "agent_jobs_task_result_by_task_id",
            "agent_jobs_batch_workers",
            vec!["taskId", "traceId", "agent_job_id", "agent_job_item_id"],
        ),
        projection_index(
            "task_board_terminal_event_by_task_id",
            "hepta_runtime_task_board",
            vec!["taskId", "traceId", "task_board_event_id", "delivery_id"],
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_canary_readback_evidence()
-> Vec<WorkGraphCanaryReadbackEvidencePreview> {
    vec![
        readback_evidence(
            "agent_jobs_task_result_report_only_readback",
            "agent_jobs_batch_workers",
            vec![
                "workGraphReportOnly field present",
                "TaskResultEnvelope canonical fields present",
                "admission decision remains non-blocking",
                "no WorkGraph event persisted",
            ],
        ),
        readback_evidence(
            "task_board_terminal_report_only_readback",
            "hepta_runtime_task_board",
            vec![
                "workGraphReportOnly field present",
                "terminal event id joins evidence",
                "delivery readback evidence joins trace",
                "no WorkGraph event persisted",
            ],
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_canary_replay_diffs()
-> Vec<WorkGraphCanaryReplayDiffPreview> {
    vec![
        replay_diff(
            "agent_jobs_report_only_replay_diff",
            "agent_jobs_batch_workers",
            "report_agent_job_result dry-run envelope",
        ),
        replay_diff(
            "task_board_terminal_report_only_replay_diff",
            "hepta_runtime_task_board",
            "task_board terminal event dry-run envelope",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_canary_required_prior_gates() -> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
        WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
        WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardCanaryReadbackReplaySideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            projection_index_persisted: false,
            readback_evidence_persisted: false,
            replay_executed: false,
            rollback_executed: false,
            feature_flag_enabled: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn projection_index(
    index_id: &'static str,
    source_surface_id: &'static str,
    key_fields: Vec<&'static str>,
) -> WorkGraphCanaryProjectionIndexPreview {
    WorkGraphCanaryProjectionIndexPreview {
        index_id,
        source_surface_id,
        key_fields,
        deterministic_id_rule: "sha256(redacted source surface + taskId + traceId + spanId)",
        redaction_rule: "payload summaries only; no raw prompt, transcript, secret, or artifact body",
        persisted: false,
    }
}

fn readback_evidence(
    evidence_id: &'static str,
    source_surface_id: &'static str,
    checks: Vec<&'static str>,
) -> WorkGraphCanaryReadbackEvidencePreview {
    WorkGraphCanaryReadbackEvidencePreview {
        evidence_id,
        source_surface_id,
        checks,
        evidence_status: "preview_ready_not_persisted",
        evidence_persisted: false,
    }
}

fn replay_diff(
    diff_id: &'static str,
    source_surface_id: &'static str,
    replay_scope: &'static str,
) -> WorkGraphCanaryReplayDiffPreview {
    WorkGraphCanaryReplayDiffPreview {
        diff_id,
        source_surface_id,
        replay_scope,
        expected_diff: "deterministic report-only envelope matches readback projection",
        replay_executed: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canary_readback_replay_covers_agent_jobs_and_task_board() {
        let report = hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report();
        let surfaces = report
            .canary_entrypoints
            .iter()
            .map(|entrypoint| entrypoint.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(report.canary_entrypoint_count, 2);
        assert!(surfaces.contains(&"agent_jobs_batch_workers"));
        assert!(surfaces.contains(&"hepta_runtime_task_board"));
        assert!(report.canary_entrypoints.iter().all(|entrypoint| {
            entrypoint.report_only_field == "workGraphReportOnly"
                && entrypoint.admission_decision == "allow_report_only_no_live_blocking"
                && !entrypoint.live_blocking_enabled
                && !entrypoint.live_persistence_enabled
        }));
    }

    #[test]
    fn canary_readback_replay_declares_projection_readback_and_replay() {
        let report = hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report();

        assert_eq!(report.projection_indexes.len(), 2);
        assert_eq!(report.readback_evidence_count, 2);
        assert_eq!(report.replay_diff_count, 2);
        assert!(report.projection_indexes.iter().all(|index| {
            !index.persisted
                && index.key_fields.contains(&"taskId")
                && index.key_fields.contains(&"traceId")
        }));
        assert!(
            report
                .readback_evidence
                .iter()
                .all(|evidence| !evidence.evidence_persisted)
        );
        assert!(report.replay_diffs.iter().all(|diff| !diff.replay_executed));
    }

    #[test]
    fn canary_readback_replay_consumes_prior_report_readbacks() {
        let report = hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE,
                WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
                WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
            ]
        );
        assert_eq!(report.required_prior_gate_count, 4);
        assert_eq!(
            report.source_entrypoint_emission_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE
        );
        assert_eq!(
            report.source_append_only_shadow_path_gate,
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE
        );
        assert_eq!(
            report.source_task_result_envelope_validator_gate,
            WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE
        );
        assert_eq!(
            report.source_scheduler_admission_dry_run_gate,
            WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE
        );
        assert_eq!(
            report.source_entrypoint_emission_required_prior_gate_count,
            3
        );
        assert_eq!(report.source_entrypoint_emission_entrypoint_count, 2);
        assert_eq!(report.source_entrypoint_emission_emission_count, 2);
        assert_eq!(
            report.source_append_only_shadow_path_scheduler_prior_gate_count,
            5
        );
        assert_eq!(
            report.source_append_only_shadow_path_required_prior_gate_count,
            9
        );
        assert_eq!(report.source_task_result_envelope_source_adapter_count, 7);
        assert_eq!(report.source_task_result_envelope_source_envelope_count, 7);
        assert_eq!(report.source_scheduler_admission_entrypoint_count, 4);
        assert_eq!(
            report.source_scheduler_admission_required_prior_gate_count,
            5
        );
        assert!(report.source_entrypoint_emission_readiness_complete);
        assert!(report.source_entrypoint_emission_no_live_confirmed);
        assert!(report.source_append_only_shadow_path_readiness_complete);
        assert!(report.source_append_only_shadow_path_no_persistence_confirmed);
        assert!(report.source_task_result_envelope_validator_ready);
        assert!(report.source_task_result_envelope_no_enforcement_confirmed);
        assert!(report.source_scheduler_admission_dry_run_ready);
        assert!(report.source_scheduler_admission_no_live_blocking_confirmed);
        assert!(report.canary_readback_replay_prior_readbacks_complete);
        assert!(report.canary_projection_readback_replay_preview_complete);
    }

    #[test]
    fn canary_readback_replay_links_required_priors() {
        let report = hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report();

        assert!(report.feature_flag_required);
        assert!(!report.feature_flag_enabled);
        assert!(report.ready_for_non_blocking_canary);
        assert!(!report.ready_for_live_cutover);
    }

    #[test]
    fn canary_readback_replay_has_no_live_side_effects() {
        let report = hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAgentJobsTaskBoardCanaryReadbackReplaySideEffects::none()
        );
    }
}
