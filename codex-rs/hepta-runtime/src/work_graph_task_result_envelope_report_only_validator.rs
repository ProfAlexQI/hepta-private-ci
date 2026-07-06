use serde::Serialize;

use crate::work_graph_canonical_projection_readiness::WORK_GRAPH_CANONICAL_PROJECTION_READINESS_GATE;
use crate::work_graph_task_result_contract::WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE;

pub const WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE: &str =
    "hepta_work_graph_task_result_envelope_report_only_validator_gate";
pub const WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_SCHEMA_VERSION: &str =
    "work_graph_task_result_envelope_report_only_validator_v1";
pub const WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultEnvelopeReportOnlyValidatorReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub canonical_wire_field_count: usize,
    pub validation_rule_count: usize,
    pub source_adapter_count: usize,
    pub source_envelope_count: usize,
    pub report_only_valid_source_count: usize,
    pub canonical_wire_fields: Vec<&'static str>,
    pub validation_rules: Vec<WorkGraphTaskResultEnvelopeValidationRulePreview>,
    pub source_adapters: Vec<WorkGraphTaskResultEnvelopeSourceAdapterPreview>,
    pub source_envelopes: Vec<WorkGraphTaskResultEnvelopePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub report_only_validator_attached: bool,
    pub live_enforcement_enabled: bool,
    pub ready_for_scheduler_admission_dry_run_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultEnvelopeValidationRulePreview {
    pub id: &'static str,
    pub required: bool,
    pub report_only_blocks_promotion: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultEnvelopeSourceAdapterPreview {
    pub source_surface_id: &'static str,
    pub terminal_event: &'static str,
    pub entrypoint_or_reducer: &'static str,
    pub covered_wire_fields: Vec<&'static str>,
    pub validation_rule_ids: Vec<&'static str>,
    pub report_only_attached: bool,
    pub live_enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultEnvelopePreview {
    pub source_surface_id: &'static str,
    pub task_id: &'static str,
    pub status: &'static str,
    pub summary: &'static str,
    pub artifacts: Vec<&'static str>,
    pub evidence: Vec<&'static str>,
    pub risks: Vec<&'static str>,
    pub next_actions: Vec<&'static str>,
    pub verifier: WorkGraphTaskResultEnvelopeVerifierPreview,
    pub reducer: WorkGraphTaskResultEnvelopeReducerPreview,
    pub usage: WorkGraphTaskResultEnvelopeUsagePreview,
    pub trace_id: &'static str,
    pub validation_decision: &'static str,
    pub live_promotion_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultEnvelopeVerifierPreview {
    pub verifier_id: &'static str,
    pub gate_id: &'static str,
    pub status: &'static str,
    pub evidence_ref: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultEnvelopeReducerPreview {
    pub reducer_id: &'static str,
    pub mode: &'static str,
    pub decision: &'static str,
    pub evidence_ref: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultEnvelopeUsagePreview {
    pub model_tokens: u64,
    pub tool_calls: u64,
    pub command_count: u64,
    pub budget_state: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_task_result_envelope_report_only_validator_report()
-> WorkGraphTaskResultEnvelopeReportOnlyValidatorReport {
    let canonical_wire_fields = work_graph_task_result_envelope_canonical_wire_fields();
    let validation_rules = work_graph_task_result_envelope_validation_rules();
    let source_adapters = work_graph_task_result_envelope_source_adapters();
    let source_envelopes = work_graph_task_result_envelope_source_envelopes();
    let report_only_valid_source_count = source_envelopes
        .iter()
        .filter(|envelope| envelope.validation_decision == "allow_report_only")
        .count();

    WorkGraphTaskResultEnvelopeReportOnlyValidatorReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
        schema_version: WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_SCHEMA_VERSION,
        preview_mode: "report_only_task_result_envelope_validator_no_live_enforcement",
        canonical_wire_field_count: canonical_wire_fields.len(),
        validation_rule_count: validation_rules.len(),
        source_adapter_count: source_adapters.len(),
        source_envelope_count: source_envelopes.len(),
        report_only_valid_source_count,
        canonical_wire_fields,
        validation_rules,
        source_adapters,
        source_envelopes,
        required_prior_gates: vec![
            WORK_GRAPH_CANONICAL_PROJECTION_READINESS_GATE,
            WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE,
        ],
        recommended_next_gate:
            WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_RECOMMENDED_NEXT_GATE,
        report_only_validator_attached: true,
        live_enforcement_enabled: false,
        ready_for_scheduler_admission_dry_run_enforcement: true,
        ready_for_live_execution: false,
        side_effects: WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects::none(),
    }
}

pub fn work_graph_task_result_envelope_canonical_wire_fields() -> Vec<&'static str> {
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
    ]
}

pub fn work_graph_task_result_envelope_validation_rules()
-> Vec<WorkGraphTaskResultEnvelopeValidationRulePreview> {
    vec![
        rule(
            "required_wire_fields_present",
            "all canonical TaskResultEnvelope wire fields must be present before terminal promotion",
        ),
        rule(
            "status_is_normalized",
            "source statuses must map to queued, running, succeeded, failed, cancelled, blocked, or superseded",
        ),
        rule(
            "summary_is_redacted_and_non_empty",
            "summaries must be operator-readable without embedding raw private payloads",
        ),
        rule(
            "artifact_refs_are_ids_hashes_or_paths",
            "artifacts must be references rather than raw payload blobs",
        ),
        rule(
            "evidence_refs_are_readback_bound",
            "evidence must identify commands, gates, reducer output, mailbox records, or readback probes",
        ),
        rule(
            "risks_and_next_actions_are_actionable",
            "risk and next action entries need owner, severity, or scheduler intent",
        ),
        rule(
            "verifier_reducer_usage_are_structured",
            "verifier, reducer, and usage cannot be free-form strings",
        ),
        rule(
            "trace_id_joins_plan_spawn_mailbox_tool_result",
            "traceId must join the result with upstream plan, spawn, mailbox, tool, artifact, and guardrail spans",
        ),
    ]
}

pub fn work_graph_task_result_envelope_source_adapters()
-> Vec<WorkGraphTaskResultEnvelopeSourceAdapterPreview> {
    let fields = work_graph_task_result_envelope_canonical_wire_fields();
    let rules = work_graph_task_result_envelope_validation_rule_ids();

    vec![
        adapter(
            "agent_jobs_batch_workers",
            "report_agent_job_result.accepted",
            "report_agent_job_result",
            fields.clone(),
            rules.clone(),
        ),
        adapter(
            "hepta_runtime_worker_tasks",
            "WorkerTaskRecord.terminal_status",
            "worker_task_run",
            fields.clone(),
            rules.clone(),
        ),
        adapter(
            "hepta_runtime_multi_agent_reducer",
            "AgentRuntimeRunReport.reducer_passed",
            "multi_agent_reducer",
            fields.clone(),
            rules.clone(),
        ),
        adapter(
            "hepta_runtime_task_board",
            "TaskBoardTerminalEvent.status",
            "task_board_terminal_event",
            fields,
            rules,
        ),
    ]
}

pub fn work_graph_task_result_envelope_source_envelopes() -> Vec<WorkGraphTaskResultEnvelopePreview>
{
    vec![
        envelope(
            "agent_jobs_batch_workers",
            "wg-task-result-agent-job-item-preview-001",
            "succeeded",
            "agent job item reported a structured result object",
            vec!["artifact:agent-job-output-csv-preview"],
            vec!["gate:report-agent-job-result-json-object"],
            Vec::new(),
            vec!["next:scheduler-admission-dry-run"],
            "trace-agent-job-preview-001",
            "agent_job_result_verifier",
            "agent_job_item_report_only_reducer",
            "single",
        ),
        envelope(
            "hepta_runtime_worker_tasks",
            "wg-task-result-worker-task-preview-001",
            "succeeded",
            "worker task reached a terminal status with artifact and evidence refs",
            vec!["artifact:worker-task-output-preview"],
            vec!["readback:worker-task-record-preview"],
            Vec::new(),
            vec!["next:task-board-terminal-event"],
            "trace-worker-task-preview-001",
            "worker_task_terminal_verifier",
            "worker_task_terminal_reducer",
            "single",
        ),
        envelope(
            "hepta_runtime_multi_agent_reducer",
            "wg-task-result-multi-agent-reducer-preview-001",
            "succeeded",
            "multi-agent reducer selected a consensus result from child outputs",
            vec!["artifact:reducer-summary-preview"],
            vec!["reducer:quorum-preview"],
            vec!["risk:child-output-drift-watch"],
            vec!["next:parent-agent-merge"],
            "trace-multi-agent-reducer-preview-001",
            "multi_agent_reducer_verifier",
            "multi_agent_quorum_reducer",
            "quorum",
        ),
        envelope(
            "hepta_runtime_task_board",
            "wg-task-result-task-board-terminal-preview-001",
            "blocked",
            "task board terminal event recorded a blocked result for scheduler readback",
            Vec::new(),
            vec!["task-board:terminal-event-preview"],
            vec!["risk:operator-review-required"],
            vec!["next:surface-deny-explanation"],
            "trace-task-board-preview-001",
            "task_board_terminal_verifier",
            "task_board_terminal_reducer",
            "single",
        ),
    ]
}

impl WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            task_result_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_task_result_envelope_validation_rule_ids() -> Vec<&'static str> {
    work_graph_task_result_envelope_validation_rules()
        .iter()
        .map(|rule| rule.id)
        .collect()
}

fn rule(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTaskResultEnvelopeValidationRulePreview {
    WorkGraphTaskResultEnvelopeValidationRulePreview {
        id,
        required: true,
        report_only_blocks_promotion: false,
        reason,
    }
}

fn adapter(
    source_surface_id: &'static str,
    terminal_event: &'static str,
    entrypoint_or_reducer: &'static str,
    covered_wire_fields: Vec<&'static str>,
    validation_rule_ids: Vec<&'static str>,
) -> WorkGraphTaskResultEnvelopeSourceAdapterPreview {
    WorkGraphTaskResultEnvelopeSourceAdapterPreview {
        source_surface_id,
        terminal_event,
        entrypoint_or_reducer,
        covered_wire_fields,
        validation_rule_ids,
        report_only_attached: true,
        live_enforcement_enabled: false,
    }
}

fn envelope(
    source_surface_id: &'static str,
    task_id: &'static str,
    status: &'static str,
    summary: &'static str,
    artifacts: Vec<&'static str>,
    evidence: Vec<&'static str>,
    risks: Vec<&'static str>,
    next_actions: Vec<&'static str>,
    trace_id: &'static str,
    verifier_id: &'static str,
    reducer_id: &'static str,
    reducer_mode: &'static str,
) -> WorkGraphTaskResultEnvelopePreview {
    WorkGraphTaskResultEnvelopePreview {
        source_surface_id,
        task_id,
        status,
        summary,
        artifacts,
        evidence,
        risks,
        next_actions,
        verifier: WorkGraphTaskResultEnvelopeVerifierPreview {
            verifier_id,
            gate_id: WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
            status: "report_only_valid",
            evidence_ref: "evidence:task-result-envelope-report-only-validator",
        },
        reducer: WorkGraphTaskResultEnvelopeReducerPreview {
            reducer_id,
            mode: reducer_mode,
            decision: "report_only_accept",
            evidence_ref: "evidence:task-result-envelope-reducer-preview",
        },
        usage: WorkGraphTaskResultEnvelopeUsagePreview {
            model_tokens: 0,
            tool_calls: 0,
            command_count: 0,
            budget_state: "not_debited_report_only",
        },
        trace_id,
        validation_decision: "allow_report_only",
        live_promotion_allowed: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_result_envelope_validator_declares_canonical_wire_fields() {
        let report = hepta_work_graph_task_result_envelope_report_only_validator_report();

        assert_eq!(
            report.canonical_wire_fields,
            [
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
            ]
        );
        assert_eq!(report.canonical_wire_field_count, 11);
        assert_eq!(report.validation_rule_count, 8);
    }

    #[test]
    fn task_result_envelope_validator_attaches_requested_sources() {
        let report = hepta_work_graph_task_result_envelope_report_only_validator_report();
        let adapter_ids = report
            .source_adapters
            .iter()
            .map(|adapter| adapter.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            adapter_ids,
            [
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ]
        );
        assert_eq!(report.source_adapter_count, 4);
        assert!(
            report
                .source_adapters
                .iter()
                .all(|adapter| adapter.report_only_attached && !adapter.live_enforcement_enabled)
        );
    }

    #[test]
    fn task_result_envelope_validator_projects_report_only_envelopes() {
        let report = hepta_work_graph_task_result_envelope_report_only_validator_report();
        let source_ids = report
            .source_envelopes
            .iter()
            .map(|envelope| envelope.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            source_ids,
            [
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ]
        );
        assert_eq!(report.source_envelope_count, 4);
        assert_eq!(report.report_only_valid_source_count, 4);
        assert!(
            report
                .source_envelopes
                .iter()
                .all(
                    |envelope| envelope.validation_decision == "allow_report_only"
                        && !envelope.live_promotion_allowed
                        && !envelope.trace_id.is_empty()
                )
        );
    }

    #[test]
    fn task_result_envelope_validator_remains_non_mutating() {
        let report = hepta_work_graph_task_result_envelope_report_only_validator_report();

        assert_eq!(
            report.side_effects,
            WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects::none()
        );
        assert!(report.report_only_validator_attached);
        assert!(!report.live_enforcement_enabled);
        assert!(report.ready_for_scheduler_admission_dry_run_enforcement);
        assert!(!report.ready_for_live_execution);
    }
}
