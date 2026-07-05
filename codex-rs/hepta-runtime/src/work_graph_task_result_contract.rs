use serde::Serialize;

pub const WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE: &str =
    "hepta_work_graph_task_result_contract_preview_gate";
pub const WORK_GRAPH_TASK_RESULT_CONTRACT_SCHEMA_VERSION: &str =
    "work_graph_task_result_contract_preview_v1";
pub const WORK_GRAPH_TASK_RESULT_CONTRACT_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_scheduler_admission_controller_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultContractPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub required_field_count: usize,
    pub status_count: usize,
    pub terminal_status_count: usize,
    pub validator_count: usize,
    pub adapter_preview_count: usize,
    pub required_fields: Vec<WorkGraphTaskResultFieldPreview>,
    pub statuses: Vec<WorkGraphTaskResultStatusPreview>,
    pub validators: Vec<WorkGraphTaskResultValidatorPreview>,
    pub adapter_previews: Vec<WorkGraphTaskResultAdapterPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_scheduler_admission_preview: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTaskResultContractPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultFieldPreview {
    pub wire_name: &'static str,
    pub rust_name: &'static str,
    pub field_kind: &'static str,
    pub required: bool,
    pub terminal_required: bool,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultStatusPreview {
    pub id: &'static str,
    pub terminal: bool,
    pub promotion_allowed: bool,
    pub requires_evidence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultValidatorPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultAdapterPreview {
    pub source_surface_id: &'static str,
    pub source_status_field: &'static str,
    pub projected_result_node_kind: &'static str,
    pub covered_wire_fields: Vec<&'static str>,
    pub blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultContractPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub task_result_enforcement_enabled: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_task_result_contract_preview_report()
-> WorkGraphTaskResultContractPreviewReport {
    let required_fields = work_graph_task_result_required_fields();
    let statuses = work_graph_task_result_statuses();
    let validators = work_graph_task_result_validators();
    let adapter_previews = work_graph_task_result_adapter_previews();
    let terminal_status_count = statuses.iter().filter(|status| status.terminal).count();

    WorkGraphTaskResultContractPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE,
        schema_version: WORK_GRAPH_TASK_RESULT_CONTRACT_SCHEMA_VERSION,
        preview_mode: "validator_first_schema_preview_no_enforcement",
        required_field_count: required_fields.len(),
        status_count: statuses.len(),
        terminal_status_count,
        validator_count: validators.len(),
        adapter_preview_count: adapter_previews.len(),
        required_fields,
        statuses,
        validators,
        adapter_previews,
        recommended_next_gate: WORK_GRAPH_TASK_RESULT_CONTRACT_RECOMMENDED_NEXT_GATE,
        ready_for_scheduler_admission_preview: true,
        ready_for_task_result_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphTaskResultContractPreviewSideEffects::none(),
    }
}

pub fn work_graph_task_result_required_fields() -> Vec<WorkGraphTaskResultFieldPreview> {
    vec![
        field(
            "taskId",
            "task_id",
            "stable_id",
            true,
            true,
            "stable task, agent, worker, or scheduler result identity",
        ),
        field(
            "status",
            "status",
            "enum",
            true,
            true,
            "normalized lifecycle outcome for graph promotion",
        ),
        field(
            "summary",
            "summary",
            "string",
            true,
            true,
            "operator-facing result summary with no secret payload",
        ),
        field(
            "artifacts",
            "artifacts",
            "array",
            true,
            false,
            "artifact ids, content hashes, path hints, or external handoff references",
        ),
        field(
            "evidence",
            "evidence",
            "array",
            true,
            true,
            "readback, command, gate, mailbox, or reducer evidence references",
        ),
        field(
            "risks",
            "risks",
            "array",
            true,
            false,
            "known risks, blocked states, redaction notes, and review requirements",
        ),
        field(
            "nextActions",
            "next_actions",
            "array",
            true,
            false,
            "follow-up actions for parent agents, operators, or schedulers",
        ),
        field(
            "verifier",
            "verifier",
            "object",
            true,
            true,
            "verification gate identity, status, and report hash",
        ),
        field(
            "reducer",
            "reducer",
            "object",
            true,
            false,
            "multi-agent reducer mode, decision, and consensus evidence",
        ),
        field(
            "usage",
            "usage",
            "object",
            true,
            false,
            "model, tool, command, budget, and token usage accounting",
        ),
        field(
            "traceId",
            "trace_id",
            "stable_id",
            true,
            true,
            "join key across plan, spawn, mailbox, tools, artifacts, gates, and result",
        ),
    ]
}

pub fn work_graph_task_result_statuses() -> Vec<WorkGraphTaskResultStatusPreview> {
    vec![
        status("queued", false, false, false),
        status("running", false, false, false),
        status("succeeded", true, true, true),
        status("failed", true, true, true),
        status("cancelled", true, true, true),
        status("blocked", true, false, true),
        status("superseded", true, true, true),
    ]
}

pub fn work_graph_task_result_validators() -> Vec<WorkGraphTaskResultValidatorPreview> {
    vec![
        validator(
            "required_wire_fields_present",
            "every TaskResult must include the canonical wire fields before it can be accepted",
        ),
        validator(
            "terminal_status_requires_summary_evidence_and_trace",
            "terminal TaskResults must include summary, evidence, verifier, and traceId",
        ),
        validator(
            "artifact_reference_requires_identity_and_hash_or_path",
            "artifact entries must be joinable without embedding raw payloads",
        ),
        validator(
            "risk_entry_requires_severity_reason_and_owner",
            "risks must be actionable by parent agents, schedulers, or operators",
        ),
        validator(
            "verifier_reducer_and_usage_are_structured",
            "gate, reducer, and budget information cannot be free-form text only",
        ),
        validator(
            "terminal_promotion_requires_no_secret_payload",
            "summaries and evidence references must not expose raw secrets or private payloads",
        ),
        validator(
            "adapter_projection_is_preview_only",
            "existing agent, worker, and scheduler result stores are only projected, not enforced",
        ),
    ]
}

pub fn work_graph_task_result_adapter_previews() -> Vec<WorkGraphTaskResultAdapterPreview> {
    vec![
        adapter(
            "agent_jobs_batch_workers",
            "AgentJobItem.status",
            "worker_task",
            vec![
                "taskId",
                "status",
                "summary",
                "evidence",
                "nextActions",
                "traceId",
            ],
            vec!["agent_job_result_json_is_not_task_result_schema"],
        ),
        adapter(
            "hepta_runtime_worker_tasks",
            "WorkerTaskRecord.status",
            "worker_task",
            vec![
                "taskId",
                "status",
                "summary",
                "artifacts",
                "evidence",
                "risks",
                "nextActions",
                "usage",
                "traceId",
            ],
            vec!["worker_task_missing_verifier_and_reducer_projection"],
        ),
        adapter(
            "hepta_runtime_multi_agent_reducer",
            "AgentRuntimeRunReport.reducer_passed",
            "agent_task",
            vec![
                "taskId", "status", "summary", "evidence", "risks", "reducer", "traceId",
            ],
            vec!["reducer_output_missing_task_result_wrapper"],
        ),
        adapter(
            "multi_agent_v2_thread_spawn",
            "thread_spawn_edge.status",
            "agent_task",
            vec!["taskId", "status", "summary", "evidence", "traceId"],
            vec!["thread_spawn_edge_missing_terminal_task_result"],
        ),
        adapter(
            "hepta_runtime_scheduler_store",
            "SchedulerRunRecord.status",
            "scheduler_run",
            vec![
                "taskId",
                "status",
                "summary",
                "evidence",
                "risks",
                "nextActions",
                "traceId",
            ],
            vec!["scheduler_run_missing_task_result_projection"],
        ),
        adapter(
            "hepta_runtime_agent_harness",
            "AgentHarnessRunRecord.status",
            "external_handoff",
            vec![
                "taskId",
                "status",
                "summary",
                "artifacts",
                "evidence",
                "risks",
                "traceId",
            ],
            vec!["agent_harness_ledger_missing_task_result_projection"],
        ),
    ]
}

impl WorkGraphTaskResultContractPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            task_result_enforcement_enabled: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn field(
    wire_name: &'static str,
    rust_name: &'static str,
    field_kind: &'static str,
    required: bool,
    terminal_required: bool,
    purpose: &'static str,
) -> WorkGraphTaskResultFieldPreview {
    WorkGraphTaskResultFieldPreview {
        wire_name,
        rust_name,
        field_kind,
        required,
        terminal_required,
        purpose,
    }
}

fn status(
    id: &'static str,
    terminal: bool,
    promotion_allowed: bool,
    requires_evidence: bool,
) -> WorkGraphTaskResultStatusPreview {
    WorkGraphTaskResultStatusPreview {
        id,
        terminal,
        promotion_allowed,
        requires_evidence,
    }
}

fn validator(id: &'static str, reason: &'static str) -> WorkGraphTaskResultValidatorPreview {
    WorkGraphTaskResultValidatorPreview {
        id,
        required: true,
        reason,
    }
}

fn adapter(
    source_surface_id: &'static str,
    source_status_field: &'static str,
    projected_result_node_kind: &'static str,
    covered_wire_fields: Vec<&'static str>,
    blocker_ids: Vec<&'static str>,
) -> WorkGraphTaskResultAdapterPreview {
    WorkGraphTaskResultAdapterPreview {
        source_surface_id,
        source_status_field,
        projected_result_node_kind,
        covered_wire_fields,
        blocker_ids,
        enforcement_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_result_contract_declares_canonical_wire_fields() {
        let report = hepta_work_graph_task_result_contract_preview_report();
        let wire_fields = report
            .required_fields
            .iter()
            .map(|field| field.wire_name)
            .collect::<Vec<_>>();

        assert_eq!(
            wire_fields,
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
        assert_eq!(report.required_field_count, 11);
        assert!(report.required_fields.iter().all(|field| field.required));
    }

    #[test]
    fn task_result_contract_marks_terminal_statuses_as_evidence_bound() {
        let report = hepta_work_graph_task_result_contract_preview_report();
        let terminal_statuses = report
            .statuses
            .iter()
            .filter(|status| status.terminal)
            .map(|status| status.id)
            .collect::<Vec<_>>();

        assert_eq!(
            terminal_statuses,
            ["succeeded", "failed", "cancelled", "blocked", "superseded"]
        );
        assert_eq!(report.terminal_status_count, 5);
        assert!(
            report
                .statuses
                .iter()
                .filter(|status| status.terminal)
                .all(|status| status.requires_evidence)
        );
    }

    #[test]
    fn task_result_contract_keeps_enforcement_disabled() {
        let report = hepta_work_graph_task_result_contract_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphTaskResultContractPreviewSideEffects::none()
        );
        assert!(report.ready_for_scheduler_admission_preview);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .adapter_previews
                .iter()
                .all(|adapter| !adapter.enforcement_enabled)
        );
    }

    #[test]
    fn task_result_contract_projects_existing_result_surfaces() {
        let report = hepta_work_graph_task_result_contract_preview_report();
        let adapter_ids = report
            .adapter_previews
            .iter()
            .map(|adapter| adapter.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            adapter_ids,
            [
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_multi_agent_reducer",
                "multi_agent_v2_thread_spawn",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.adapter_preview_count, 6);
        assert_eq!(report.validator_count, 7);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TASK_RESULT_CONTRACT_RECOMMENDED_NEXT_GATE
        );
    }
}
