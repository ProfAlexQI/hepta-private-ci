use serde::Serialize;

use crate::work_graph_agent_role_agent_card_manifest_report_only::WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_GATE;
use crate::work_graph_append_only_event_store_shadow_path::WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_task_result_envelope_report_only_validator::WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE;

pub const WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE: &str =
    "hepta_work_graph_trace_guardrail_span_report_only_gate";
pub const WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_SCHEMA_VERSION: &str =
    "work_graph_trace_guardrail_span_report_only_v1";
pub const WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTraceGuardrailSpanReportOnlyReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub required_wire_field_count: usize,
    pub span_count: usize,
    pub blocking_guardrail_count: usize,
    pub source_binding_count: usize,
    pub required_prior_gate_count: usize,
    pub required_wire_fields: Vec<&'static str>,
    pub spans: Vec<WorkGraphTraceSpanPreview>,
    pub guardrail_bindings: Vec<WorkGraphGuardrailSpanBindingPreview>,
    pub source_bindings: Vec<WorkGraphTraceGuardrailSourceBindingPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub trace_spine_complete: bool,
    pub blocking_guardrail_preview_complete: bool,
    pub report_only_guardrail_attached: bool,
    pub live_guardrail_enforcement_enabled: bool,
    pub ready_for_agent_jobs_task_board_report_only_emission: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTraceGuardrailSpanReportOnlySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTraceSpanPreview {
    pub trace_id: &'static str,
    pub span_id: &'static str,
    pub parent_span_id: Option<&'static str>,
    pub kind: &'static str,
    pub source_surface_id: &'static str,
    pub source_entrypoint: &'static str,
    pub decision: &'static str,
    pub blocking_guardrail_required: bool,
    pub guardrail_span_id: &'static str,
    pub evidence_ref: &'static str,
    pub redaction_policy: &'static str,
    pub payload_hash: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphGuardrailSpanBindingPreview {
    pub guardrail_id: &'static str,
    pub span_id: &'static str,
    pub guardrail_kind: &'static str,
    pub source_surface_id: &'static str,
    pub blocking_preview: bool,
    pub required_for_live_promotion: bool,
    pub decision: &'static str,
    pub evidence_ref: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTraceGuardrailSourceBindingPreview {
    pub source_surface_id: &'static str,
    pub entrypoint_id: &'static str,
    pub required_span_kinds: Vec<&'static str>,
    pub required_guardrail_ids: Vec<&'static str>,
    pub trace_join_fields: Vec<&'static str>,
    pub report_only_attached: bool,
    pub live_blocking_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTraceGuardrailSpanReportOnlySideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub trace_persisted: bool,
    pub guardrail_persisted: bool,
    pub guardrail_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub side_effect_lock_established: bool,
    pub scheduler_admission_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_trace_guardrail_span_report_only_report()
-> WorkGraphTraceGuardrailSpanReportOnlyReport {
    let required_wire_fields = work_graph_trace_guardrail_span_required_wire_fields();
    let spans = work_graph_trace_guardrail_spans();
    let guardrail_bindings = work_graph_guardrail_span_bindings();
    let source_bindings = work_graph_trace_guardrail_source_bindings();
    let required_prior_gates = work_graph_trace_guardrail_span_required_prior_gates();
    let blocking_guardrail_count = spans
        .iter()
        .filter(|span| span.blocking_guardrail_required)
        .count();

    WorkGraphTraceGuardrailSpanReportOnlyReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE,
        schema_version: WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_SCHEMA_VERSION,
        preview_mode: "report_only_trace_guardrail_span_no_live_blocking",
        required_wire_field_count: required_wire_fields.len(),
        span_count: spans.len(),
        blocking_guardrail_count,
        source_binding_count: source_bindings.len(),
        required_prior_gate_count: required_prior_gates.len(),
        required_wire_fields,
        spans,
        guardrail_bindings,
        source_bindings,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_RECOMMENDED_NEXT_GATE,
        trace_spine_complete: true,
        blocking_guardrail_preview_complete: true,
        report_only_guardrail_attached: true,
        live_guardrail_enforcement_enabled: false,
        ready_for_agent_jobs_task_board_report_only_emission: true,
        ready_for_live_execution: false,
        side_effects: WorkGraphTraceGuardrailSpanReportOnlySideEffects::none(),
    }
}

pub fn work_graph_trace_guardrail_span_required_wire_fields() -> Vec<&'static str> {
    vec![
        "traceId",
        "spanId",
        "parentSpanId",
        "kind",
        "source",
        "decision",
        "blocking",
        "evidenceRef",
        "redaction",
        "hash",
    ]
}

pub fn work_graph_trace_guardrail_spans() -> Vec<WorkGraphTraceSpanPreview> {
    vec![
        span(
            "span-plan-001",
            None,
            "plan",
            "planning_update_plan",
            "update_plan",
            "trace_only",
            false,
            "guardrail-read-only-boundary",
            "evidence:canonical-projection-readiness",
            "redact_freeform_plan_text",
            "sha256:trace-plan-001",
        ),
        span(
            "span-spawn-001",
            Some("span-plan-001"),
            "spawn",
            "multi_agent_v2_thread_spawn",
            "spawn_agent",
            "admission_trace_only",
            true,
            "guardrail-spawn-admission",
            "evidence:scheduler-admission-dry-run",
            "redact_spawn_prompt_preview",
            "sha256:trace-spawn-001",
        ),
        span(
            "span-handoff-001",
            Some("span-spawn-001"),
            "handoff",
            "hepta_runtime_agent_harness",
            "handoff_event",
            "external_handoff_preview_only",
            true,
            "guardrail-external-handoff",
            "evidence:persistent-mailbox-handoff-mapping",
            "redact_handoff_payload_preview",
            "sha256:trace-handoff-001",
        ),
        span(
            "span-mailbox-001",
            Some("span-handoff-001"),
            "mailbox",
            "multi_agent_mailbox",
            "mailbox_message",
            "ack_deadline_trace_only",
            false,
            "guardrail-mailbox-deadline",
            "evidence:mailbox-ack-deadline-contract",
            "redact_mailbox_content",
            "sha256:trace-mailbox-001",
        ),
        span(
            "span-tool-001",
            Some("span-spawn-001"),
            "tool",
            "tool_invocation",
            "tool_call",
            "tool_side_effect_preview_only",
            true,
            "guardrail-tool-side-effect",
            "evidence:side-effect-boundary",
            "redact_tool_arguments",
            "sha256:trace-tool-001",
        ),
        span(
            "span-result-001",
            Some("span-tool-001"),
            "result",
            "task_result_envelope",
            "TaskResultEnvelope",
            "result_validation_report_only",
            false,
            "guardrail-result-contract",
            "evidence:task-result-envelope-report-only",
            "redact_result_summary",
            "sha256:trace-result-001",
        ),
        span(
            "span-artifact-001",
            Some("span-result-001"),
            "artifact",
            "artifact_reference",
            "artifact_ref",
            "artifact_publication_blocked",
            true,
            "guardrail-artifact-publication",
            "evidence:artifact-ref-readback",
            "redact_artifact_payload_to_ref",
            "sha256:trace-artifact-001",
        ),
        span(
            "span-approval-001",
            Some("span-handoff-001"),
            "approval",
            "operator_approval",
            "approval_readback",
            "approval_required_but_not_recorded",
            true,
            "guardrail-approval-authority",
            "evidence:approval-absence-proof",
            "redact_operator_payload",
            "sha256:trace-approval-001",
        ),
        span(
            "span-guardrail-001",
            Some("span-approval-001"),
            "guardrail",
            "guardrail_policy",
            "blocking_guardrail_preview",
            "blocking_guardrail_preview_only",
            true,
            "guardrail-trace-integrity",
            "evidence:trace-guardrail-span",
            "redact_guardrail_payload",
            "sha256:trace-guardrail-001",
        ),
    ]
}

pub fn work_graph_guardrail_span_bindings() -> Vec<WorkGraphGuardrailSpanBindingPreview> {
    vec![
        guardrail(
            "guardrail-spawn-admission",
            "span-spawn-001",
            "scheduler_admission",
            "multi_agent_v2_thread_spawn",
            "evidence:scheduler-admission-dry-run",
        ),
        guardrail(
            "guardrail-external-handoff",
            "span-handoff-001",
            "external_handoff",
            "hepta_runtime_agent_harness",
            "evidence:persistent-mailbox-handoff-mapping",
        ),
        guardrail(
            "guardrail-tool-side-effect",
            "span-tool-001",
            "side_effect_boundary",
            "tool_invocation",
            "evidence:side-effect-boundary",
        ),
        guardrail(
            "guardrail-artifact-publication",
            "span-artifact-001",
            "artifact_publication",
            "artifact_reference",
            "evidence:artifact-ref-readback",
        ),
        guardrail(
            "guardrail-approval-authority",
            "span-approval-001",
            "approval_authority",
            "operator_approval",
            "evidence:approval-absence-proof",
        ),
        guardrail(
            "guardrail-trace-integrity",
            "span-guardrail-001",
            "trace_integrity",
            "guardrail_policy",
            "evidence:trace-guardrail-span",
        ),
    ]
}

pub fn work_graph_trace_guardrail_source_bindings()
-> Vec<WorkGraphTraceGuardrailSourceBindingPreview> {
    vec![
        source_binding(
            "multi_agent_v2_thread_spawn",
            "spawn_agent",
            vec!["plan", "spawn", "mailbox", "guardrail"],
            vec!["guardrail-spawn-admission", "guardrail-trace-integrity"],
        ),
        source_binding(
            "agent_jobs_batch_workers",
            "report_agent_job_result",
            vec!["tool", "result", "artifact", "guardrail"],
            vec!["guardrail-tool-side-effect", "guardrail-trace-integrity"],
        ),
        source_binding(
            "hepta_runtime_task_board",
            "task_board_terminal_event",
            vec!["result", "artifact", "approval", "guardrail"],
            vec![
                "guardrail-artifact-publication",
                "guardrail-approval-authority",
            ],
        ),
        source_binding(
            "hepta_runtime_agent_harness",
            "handoff_event",
            vec!["handoff", "mailbox", "approval", "guardrail"],
            vec!["guardrail-external-handoff", "guardrail-approval-authority"],
        ),
        source_binding(
            "hepta_runtime_worker_tasks",
            "worker_task_run",
            vec!["tool", "result", "artifact", "guardrail"],
            vec![
                "guardrail-tool-side-effect",
                "guardrail-artifact-publication",
            ],
        ),
    ]
}

pub fn work_graph_trace_guardrail_span_required_prior_gates() -> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_GATE,
        WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
        WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    ]
}

impl WorkGraphTraceGuardrailSpanReportOnlySideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            trace_persisted: false,
            guardrail_persisted: false,
            guardrail_enforcement_enabled: false,
            approval_recorded: false,
            side_effect_lock_established: false,
            scheduler_admission_enforced: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn span(
    span_id: &'static str,
    parent_span_id: Option<&'static str>,
    kind: &'static str,
    source_surface_id: &'static str,
    source_entrypoint: &'static str,
    decision: &'static str,
    blocking_guardrail_required: bool,
    guardrail_span_id: &'static str,
    evidence_ref: &'static str,
    redaction_policy: &'static str,
    payload_hash: &'static str,
) -> WorkGraphTraceSpanPreview {
    WorkGraphTraceSpanPreview {
        trace_id: "trace-work-graph-report-only-001",
        span_id,
        parent_span_id,
        kind,
        source_surface_id,
        source_entrypoint,
        decision,
        blocking_guardrail_required,
        guardrail_span_id,
        evidence_ref,
        redaction_policy,
        payload_hash,
    }
}

fn guardrail(
    guardrail_id: &'static str,
    span_id: &'static str,
    guardrail_kind: &'static str,
    source_surface_id: &'static str,
    evidence_ref: &'static str,
) -> WorkGraphGuardrailSpanBindingPreview {
    WorkGraphGuardrailSpanBindingPreview {
        guardrail_id,
        span_id,
        guardrail_kind,
        source_surface_id,
        blocking_preview: true,
        required_for_live_promotion: true,
        decision: "block_live_execution_report_only",
        evidence_ref,
    }
}

fn source_binding(
    source_surface_id: &'static str,
    entrypoint_id: &'static str,
    required_span_kinds: Vec<&'static str>,
    required_guardrail_ids: Vec<&'static str>,
) -> WorkGraphTraceGuardrailSourceBindingPreview {
    WorkGraphTraceGuardrailSourceBindingPreview {
        source_surface_id,
        entrypoint_id,
        required_span_kinds,
        required_guardrail_ids,
        trace_join_fields: vec!["traceId", "spanId", "parentSpanId", "evidenceRef"],
        report_only_attached: true,
        live_blocking_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_guardrail_declares_required_wire_fields() {
        let fields = work_graph_trace_guardrail_span_required_wire_fields();

        assert_eq!(fields.len(), 10);
        assert!(fields.contains(&"traceId"));
        assert!(fields.contains(&"spanId"));
        assert!(fields.contains(&"parentSpanId"));
        assert!(fields.contains(&"decision"));
        assert!(fields.contains(&"evidenceRef"));
        assert!(fields.contains(&"hash"));
    }

    #[test]
    fn trace_guardrail_covers_requested_span_kinds() {
        let report = hepta_work_graph_trace_guardrail_span_report_only_report();
        let kinds = report
            .spans
            .iter()
            .map(|span| span.kind)
            .collect::<Vec<_>>();

        assert_eq!(report.span_count, 9);
        assert!(kinds.contains(&"plan"));
        assert!(kinds.contains(&"spawn"));
        assert!(kinds.contains(&"handoff"));
        assert!(kinds.contains(&"mailbox"));
        assert!(kinds.contains(&"tool"));
        assert!(kinds.contains(&"result"));
        assert!(kinds.contains(&"artifact"));
        assert!(kinds.contains(&"approval"));
        assert!(kinds.contains(&"guardrail"));
        assert!(report.spans.iter().all(|span| {
            span.trace_id == "trace-work-graph-report-only-001"
                && !span.span_id.is_empty()
                && !span.payload_hash.is_empty()
        }));
    }

    #[test]
    fn trace_guardrail_marks_risky_paths_blocking() {
        let report = hepta_work_graph_trace_guardrail_span_report_only_report();
        let blocking_span_ids = report
            .spans
            .iter()
            .filter(|span| span.blocking_guardrail_required)
            .map(|span| span.span_id)
            .collect::<Vec<_>>();
        let guardrail_span_ids = report
            .guardrail_bindings
            .iter()
            .filter(|binding| binding.blocking_preview)
            .map(|binding| binding.span_id)
            .collect::<Vec<_>>();

        assert_eq!(report.blocking_guardrail_count, 6);
        assert!(blocking_span_ids.contains(&"span-spawn-001"));
        assert!(blocking_span_ids.contains(&"span-handoff-001"));
        assert!(blocking_span_ids.contains(&"span-tool-001"));
        assert!(blocking_span_ids.contains(&"span-artifact-001"));
        assert!(blocking_span_ids.contains(&"span-approval-001"));
        assert!(blocking_span_ids.contains(&"span-guardrail-001"));
        assert!(
            blocking_span_ids
                .iter()
                .all(|span_id| guardrail_span_ids.contains(span_id))
        );
    }

    #[test]
    fn trace_guardrail_remains_report_only() {
        let report = hepta_work_graph_trace_guardrail_span_report_only_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_GATE,
                WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
                WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
            ]
        );
        assert!(report.trace_spine_complete);
        assert!(report.blocking_guardrail_preview_complete);
        assert!(report.report_only_guardrail_attached);
        assert!(!report.live_guardrail_enforcement_enabled);
        assert!(report.ready_for_agent_jobs_task_board_report_only_emission);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTraceGuardrailSpanReportOnlySideEffects::none()
        );
    }
}
