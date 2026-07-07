use serde::Serialize;

use crate::work_graph_agent_role_agent_card_manifest_report_only::WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_GATE;
use crate::work_graph_agent_role_agent_card_manifest_report_only::WorkGraphAgentRoleAgentCardManifestReportOnlySideEffects;
use crate::work_graph_agent_role_agent_card_manifest_report_only::hepta_work_graph_agent_role_agent_card_manifest_report_only_report;
use crate::work_graph_append_only_event_store_shadow_path::WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE;
use crate::work_graph_append_only_event_store_shadow_path::WorkGraphAppendOnlyEventStoreShadowPathSideEffects;
use crate::work_graph_append_only_event_store_shadow_path::hepta_work_graph_append_only_event_store_shadow_path_report;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::WorkGraphSchedulerAdmissionDryRunEnforcementSideEffects;
use crate::work_graph_scheduler_admission_dry_run_enforcement::hepta_work_graph_scheduler_admission_dry_run_enforcement_report;
use crate::work_graph_task_result_envelope_report_only_validator::WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE;
use crate::work_graph_task_result_envelope_report_only_validator::WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects;
use crate::work_graph_task_result_envelope_report_only_validator::hepta_work_graph_task_result_envelope_report_only_validator_report;

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
    pub source_agent_role_agent_card_required_prior_gate_count: usize,
    pub source_append_only_shadow_path_scheduler_prior_gate_count: usize,
    pub source_append_only_shadow_path_required_prior_gate_count: usize,
    pub source_task_result_envelope_source_adapter_count: usize,
    pub source_task_result_envelope_source_envelope_count: usize,
    pub source_scheduler_admission_entrypoint_count: usize,
    pub source_scheduler_admission_required_prior_gate_count: usize,
    pub required_wire_fields: Vec<&'static str>,
    pub spans: Vec<WorkGraphTraceSpanPreview>,
    pub guardrail_bindings: Vec<WorkGraphGuardrailSpanBindingPreview>,
    pub source_bindings: Vec<WorkGraphTraceGuardrailSourceBindingPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub source_agent_role_agent_card_gate: &'static str,
    pub source_append_only_shadow_path_gate: &'static str,
    pub source_task_result_envelope_validator_gate: &'static str,
    pub source_scheduler_admission_dry_run_gate: &'static str,
    pub recommended_next_gate: &'static str,
    pub source_agent_role_agent_card_readiness_complete: bool,
    pub source_agent_role_agent_card_no_enforcement_confirmed: bool,
    pub source_append_only_shadow_path_readiness_complete: bool,
    pub source_append_only_shadow_path_no_persistence_confirmed: bool,
    pub source_task_result_envelope_validator_ready: bool,
    pub source_task_result_envelope_no_enforcement_confirmed: bool,
    pub source_scheduler_admission_dry_run_ready: bool,
    pub source_scheduler_admission_no_live_blocking_confirmed: bool,
    pub trace_guardrail_prior_readbacks_complete: bool,
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
    let agent_role_agent_card =
        hepta_work_graph_agent_role_agent_card_manifest_report_only_report();
    let append_only_shadow_path = hepta_work_graph_append_only_event_store_shadow_path_report();
    let task_result_envelope = hepta_work_graph_task_result_envelope_report_only_validator_report();
    let scheduler_admission = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();
    let blocking_guardrail_count = spans
        .iter()
        .filter(|span| span.blocking_guardrail_required)
        .count();
    let source_agent_role_agent_card_no_enforcement_confirmed = !agent_role_agent_card
        .role_enforcement_enabled
        && !agent_role_agent_card.ready_for_live_execution
        && agent_role_agent_card.side_effects
            == WorkGraphAgentRoleAgentCardManifestReportOnlySideEffects::none();
    let source_agent_role_agent_card_readiness_complete = agent_role_agent_card.gate
        == WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_GATE
        && agent_role_agent_card.agent_role_agent_card_manifest_readiness_complete
        && agent_role_agent_card.ready_for_trace_guardrail_span
        && source_agent_role_agent_card_no_enforcement_confirmed;
    let source_append_only_shadow_path_no_persistence_confirmed = !append_only_shadow_path
        .shadow_store_write_enabled
        && !append_only_shadow_path.live_cutover_enabled
        && !append_only_shadow_path.ready_for_live_execution
        && append_only_shadow_path.side_effects
            == WorkGraphAppendOnlyEventStoreShadowPathSideEffects::none();
    let source_append_only_shadow_path_readiness_complete = append_only_shadow_path.gate
        == WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE
        && append_only_shadow_path.append_only_shadow_path_readiness_complete
        && append_only_shadow_path.ready_for_persistent_mailbox_handoff
        && source_append_only_shadow_path_no_persistence_confirmed;
    let source_task_result_envelope_no_enforcement_confirmed = !task_result_envelope
        .live_enforcement_enabled
        && !task_result_envelope.ready_for_live_execution
        && task_result_envelope.side_effects
            == WorkGraphTaskResultEnvelopeReportOnlyValidatorSideEffects::none();
    let source_task_result_envelope_validator_ready = task_result_envelope.gate
        == WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE
        && task_result_envelope.ready_for_scheduler_admission_dry_run_enforcement
        && task_result_envelope.report_only_validator_attached
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
        && scheduler_admission.required_prior_gates
            == append_only_shadow_path.scheduler_prior_gates
        && source_scheduler_admission_no_live_blocking_confirmed;
    let trace_guardrail_prior_readbacks_complete = source_agent_role_agent_card_readiness_complete
        && source_append_only_shadow_path_readiness_complete
        && source_task_result_envelope_validator_ready
        && source_scheduler_admission_dry_run_ready;
    let trace_spine_complete = trace_guardrail_prior_readbacks_complete
        && !spans.is_empty()
        && spans.iter().all(|span| {
            span.trace_id == "trace-work-graph-report-only-001"
                && !span.span_id.is_empty()
                && !span.source_surface_id.is_empty()
                && !span.source_entrypoint.is_empty()
                && !span.decision.is_empty()
                && !span.guardrail_span_id.is_empty()
                && !span.evidence_ref.is_empty()
                && !span.redaction_policy.is_empty()
                && span.payload_hash.starts_with("sha256:")
        });
    let blocking_guardrail_preview_complete = blocking_guardrail_count == guardrail_bindings.len()
        && spans
            .iter()
            .filter(|span| span.blocking_guardrail_required)
            .all(|span| {
                guardrail_bindings
                    .iter()
                    .any(|binding| binding.span_id == span.span_id && binding.blocking_preview)
            });
    let report_only_guardrail_attached =
        trace_spine_complete && blocking_guardrail_preview_complete;

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
        source_agent_role_agent_card_required_prior_gate_count: agent_role_agent_card
            .required_prior_gate_count,
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
        required_wire_fields,
        spans,
        guardrail_bindings,
        source_bindings,
        required_prior_gates,
        source_agent_role_agent_card_gate: agent_role_agent_card.gate,
        source_append_only_shadow_path_gate: append_only_shadow_path.gate,
        source_task_result_envelope_validator_gate: task_result_envelope.gate,
        source_scheduler_admission_dry_run_gate: scheduler_admission.gate,
        recommended_next_gate: WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_RECOMMENDED_NEXT_GATE,
        source_agent_role_agent_card_readiness_complete,
        source_agent_role_agent_card_no_enforcement_confirmed,
        source_append_only_shadow_path_readiness_complete,
        source_append_only_shadow_path_no_persistence_confirmed,
        source_task_result_envelope_validator_ready,
        source_task_result_envelope_no_enforcement_confirmed,
        source_scheduler_admission_dry_run_ready,
        source_scheduler_admission_no_live_blocking_confirmed,
        trace_guardrail_prior_readbacks_complete,
        trace_spine_complete,
        blocking_guardrail_preview_complete,
        report_only_guardrail_attached,
        live_guardrail_enforcement_enabled: false,
        ready_for_agent_jobs_task_board_report_only_emission: report_only_guardrail_attached,
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
    fn trace_guardrail_consumes_prior_report_readbacks() {
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
        assert_eq!(
            report.source_agent_role_agent_card_gate,
            WORK_GRAPH_AGENT_ROLE_AGENT_CARD_MANIFEST_REPORT_ONLY_GATE
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
            report.source_agent_role_agent_card_required_prior_gate_count,
            2
        );
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
        assert!(report.source_agent_role_agent_card_readiness_complete);
        assert!(report.source_agent_role_agent_card_no_enforcement_confirmed);
        assert!(report.source_append_only_shadow_path_readiness_complete);
        assert!(report.source_append_only_shadow_path_no_persistence_confirmed);
        assert!(report.source_task_result_envelope_validator_ready);
        assert!(report.source_task_result_envelope_no_enforcement_confirmed);
        assert!(report.source_scheduler_admission_dry_run_ready);
        assert!(report.source_scheduler_admission_no_live_blocking_confirmed);
        assert!(report.trace_guardrail_prior_readbacks_complete);
    }

    #[test]
    fn trace_guardrail_remains_report_only() {
        let report = hepta_work_graph_trace_guardrail_span_report_only_report();

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
