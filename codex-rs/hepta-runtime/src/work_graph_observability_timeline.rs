use serde::Serialize;

pub const WORK_GRAPH_OBSERVABILITY_TIMELINE_PREVIEW_GATE: &str =
    "hepta_work_graph_observability_timeline_preview_gate";
pub const WORK_GRAPH_OBSERVABILITY_TIMELINE_SCHEMA_VERSION: &str =
    "work_graph_observability_timeline_preview_v1";
pub const WORK_GRAPH_OBSERVABILITY_TIMELINE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_role_manifest_contract_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphObservabilityTimelinePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub event_type_count: usize,
    pub span_field_count: usize,
    pub timeline_view_count: usize,
    pub adapter_preview_count: usize,
    pub event_types: Vec<WorkGraphTimelineEventTypePreview>,
    pub span_fields: Vec<WorkGraphTimelineSpanFieldPreview>,
    pub timeline_views: Vec<WorkGraphTimelineViewPreview>,
    pub adapter_previews: Vec<WorkGraphTimelineAdapterPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_role_manifest_preview: bool,
    pub ready_for_operator_dashboard: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphObservabilityTimelinePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTimelineEventTypePreview {
    pub id: &'static str,
    pub node_kind: &'static str,
    pub required_evidence_fields: Vec<&'static str>,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTimelineSpanFieldPreview {
    pub wire_name: &'static str,
    pub required: bool,
    pub redaction_required: bool,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTimelineViewPreview {
    pub id: &'static str,
    pub required: bool,
    pub purpose: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTimelineAdapterPreview {
    pub source_surface_id: &'static str,
    pub event_type_ids: Vec<&'static str>,
    pub required_join_fields: Vec<&'static str>,
    pub persistence_enabled: bool,
    pub blocker_ids: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphObservabilityTimelinePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub timeline_persisted: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub dashboard_exposed: bool,
    pub external_export_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_observability_timeline_preview_report()
-> WorkGraphObservabilityTimelinePreviewReport {
    let event_types = work_graph_observability_timeline_event_types();
    let span_fields = work_graph_observability_timeline_span_fields();
    let timeline_views = work_graph_observability_timeline_views();
    let adapter_previews = work_graph_observability_timeline_adapter_previews();

    WorkGraphObservabilityTimelinePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_OBSERVABILITY_TIMELINE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_OBSERVABILITY_TIMELINE_SCHEMA_VERSION,
        preview_mode: "read_only_timeline_shape_preview_no_persistence",
        event_type_count: event_types.len(),
        span_field_count: span_fields.len(),
        timeline_view_count: timeline_views.len(),
        adapter_preview_count: adapter_previews.len(),
        event_types,
        span_fields,
        timeline_views,
        adapter_previews,
        recommended_next_gate: WORK_GRAPH_OBSERVABILITY_TIMELINE_RECOMMENDED_NEXT_GATE,
        ready_for_role_manifest_preview: true,
        ready_for_operator_dashboard: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphObservabilityTimelinePreviewSideEffects::none(),
    }
}

pub fn work_graph_observability_timeline_event_types() -> Vec<WorkGraphTimelineEventTypePreview> {
    vec![
        event_type(
            "plan_step_observed",
            "plan_step",
            vec!["traceId", "nodeId", "sourceSurfaceId", "status"],
            "plan notifications become joinable timeline events without becoming execution authority",
        ),
        event_type(
            "agent_task_spawned",
            "agent_task",
            vec!["traceId", "nodeId", "parentTraceId", "status"],
            "subagent lineage and parent authority are visible across thread boundaries",
        ),
        event_type(
            "mailbox_progress_observed",
            "agent_task",
            vec!["traceId", "nodeId", "eventKind", "evidenceRefs"],
            "mailbox progress can be distinguished from terminal task status",
        ),
        event_type(
            "tool_invocation_observed",
            "agent_task",
            vec!["traceId", "nodeId", "eventKind", "sideEffectClass"],
            "tool calls can be audited without embedding private tool payloads",
        ),
        event_type(
            "artifact_produced",
            "artifact",
            vec!["traceId", "nodeId", "evidenceRefs", "redactionState"],
            "artifact lineage is traceable by id and hash rather than raw content",
        ),
        event_type(
            "verification_gate_observed",
            "verification_gate",
            vec!["traceId", "nodeId", "status", "evidenceRefs"],
            "gate results can explain promotion, block, retry, and rollback decisions",
        ),
        event_type(
            "task_result_observed",
            "worker_task",
            vec!["traceId", "nodeId", "status", "evidenceRefs"],
            "TaskResult projection becomes the terminal join point for work completion",
        ),
        event_type(
            "scheduler_admission_decision_observed",
            "scheduler_run",
            vec!["traceId", "nodeId", "status", "evidenceRefs"],
            "scheduler allow or deny decisions are reviewable before enforcement",
        ),
        event_type(
            "external_handoff_observed",
            "external_handoff",
            vec!["traceId", "nodeId", "status", "redactionState"],
            "external actions remain previewable without performing delivery",
        ),
    ]
}

pub fn work_graph_observability_timeline_span_fields() -> Vec<WorkGraphTimelineSpanFieldPreview> {
    vec![
        span_field(
            "traceId",
            true,
            false,
            "stable join key for the full work graph",
        ),
        span_field(
            "parentTraceId",
            false,
            false,
            "optional parent trace for spawned or retried work",
        ),
        span_field("nodeId", true, false, "stable WorkGraph node identity"),
        span_field(
            "sourceSurfaceId",
            true,
            false,
            "originating planning, agent, worker, scheduler, or harness surface",
        ),
        span_field(
            "eventKind",
            true,
            false,
            "canonical event type for timeline ordering",
        ),
        span_field("status", true, false, "normalized node or event status"),
        span_field(
            "startedAtUnixMs",
            false,
            false,
            "start timestamp when known",
        ),
        span_field(
            "completedAtUnixMs",
            false,
            false,
            "completion timestamp when known",
        ),
        span_field(
            "evidenceRefs",
            true,
            true,
            "redacted evidence ids, hashes, or report references",
        ),
        span_field(
            "redactionState",
            true,
            false,
            "whether sensitive payload was omitted, hashed, or unavailable",
        ),
        span_field(
            "sideEffectClass",
            true,
            false,
            "declares none, preview, filesystem, runtime, gateway, or external impact",
        ),
    ]
}

pub fn work_graph_observability_timeline_views() -> Vec<WorkGraphTimelineViewPreview> {
    vec![
        timeline_view("trace_timeline", "events ordered by traceId and timestamp"),
        timeline_view("node_history", "all events for a single WorkGraph node"),
        timeline_view(
            "blocking_path",
            "why a node is not runnable or not promotable",
        ),
        timeline_view(
            "artifact_lineage",
            "producer, verifier, and consumer path for an artifact",
        ),
        timeline_view(
            "operator_audit_excerpt",
            "redacted operator-facing explanation for a gate or handoff",
        ),
    ]
}

pub fn work_graph_observability_timeline_adapter_previews() -> Vec<WorkGraphTimelineAdapterPreview>
{
    vec![
        adapter(
            "app_server_turn_plan_notification",
            vec!["plan_step_observed"],
            vec!["traceId", "nodeId", "sourceSurfaceId"],
            vec!["turn_plan_notification_timeline_adapter_not_enforced"],
        ),
        adapter(
            "multi_agent_v2_thread_spawn",
            vec!["agent_task_spawned"],
            vec!["traceId", "parentTraceId", "nodeId"],
            vec!["agent_task_timeline_adapter_not_enforced"],
        ),
        adapter(
            "multi_agent_v2_mailbox_wait",
            vec!["mailbox_progress_observed"],
            vec!["traceId", "nodeId", "eventKind"],
            vec!["mailbox_progress_timeline_adapter_not_enforced"],
        ),
        adapter(
            "agent_jobs_batch_workers",
            vec!["agent_task_spawned", "task_result_observed"],
            vec!["traceId", "nodeId", "status"],
            vec!["agent_job_timeline_adapter_not_enforced"],
        ),
        adapter(
            "hepta_runtime_worker_tasks",
            vec![
                "tool_invocation_observed",
                "artifact_produced",
                "task_result_observed",
            ],
            vec!["traceId", "nodeId", "evidenceRefs"],
            vec!["worker_task_timeline_adapter_not_enforced"],
        ),
        adapter(
            "hepta_runtime_scheduler_store",
            vec!["scheduler_admission_decision_observed"],
            vec!["traceId", "nodeId", "status"],
            vec!["scheduler_timeline_adapter_not_enforced"],
        ),
        adapter(
            "hepta_runtime_agent_harness",
            vec![
                "external_handoff_observed",
                "artifact_produced",
                "verification_gate_observed",
            ],
            vec!["traceId", "nodeId", "redactionState"],
            vec!["agent_harness_timeline_adapter_not_enforced"],
        ),
    ]
}

impl WorkGraphObservabilityTimelinePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            timeline_persisted: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            dashboard_exposed: false,
            external_export_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn event_type(
    id: &'static str,
    node_kind: &'static str,
    required_evidence_fields: Vec<&'static str>,
    purpose: &'static str,
) -> WorkGraphTimelineEventTypePreview {
    WorkGraphTimelineEventTypePreview {
        id,
        node_kind,
        required_evidence_fields,
        purpose,
    }
}

fn span_field(
    wire_name: &'static str,
    required: bool,
    redaction_required: bool,
    purpose: &'static str,
) -> WorkGraphTimelineSpanFieldPreview {
    WorkGraphTimelineSpanFieldPreview {
        wire_name,
        required,
        redaction_required,
        purpose,
    }
}

fn timeline_view(id: &'static str, purpose: &'static str) -> WorkGraphTimelineViewPreview {
    WorkGraphTimelineViewPreview {
        id,
        required: true,
        purpose,
    }
}

fn adapter(
    source_surface_id: &'static str,
    event_type_ids: Vec<&'static str>,
    required_join_fields: Vec<&'static str>,
    blocker_ids: Vec<&'static str>,
) -> WorkGraphTimelineAdapterPreview {
    WorkGraphTimelineAdapterPreview {
        source_surface_id,
        event_type_ids,
        required_join_fields,
        persistence_enabled: false,
        blocker_ids,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observability_timeline_preview_declares_event_types() {
        let report = hepta_work_graph_observability_timeline_preview_report();
        let event_ids = report
            .event_types
            .iter()
            .map(|event| event.id)
            .collect::<Vec<_>>();

        assert_eq!(
            event_ids,
            [
                "plan_step_observed",
                "agent_task_spawned",
                "mailbox_progress_observed",
                "tool_invocation_observed",
                "artifact_produced",
                "verification_gate_observed",
                "task_result_observed",
                "scheduler_admission_decision_observed",
                "external_handoff_observed",
            ]
        );
        assert_eq!(report.event_type_count, 9);
    }

    #[test]
    fn observability_timeline_preview_keeps_persistence_disabled() {
        let report = hepta_work_graph_observability_timeline_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphObservabilityTimelinePreviewSideEffects::none()
        );
        assert!(report.ready_for_role_manifest_preview);
        assert!(!report.ready_for_operator_dashboard);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .adapter_previews
                .iter()
                .all(|adapter| !adapter.persistence_enabled)
        );
    }

    #[test]
    fn observability_timeline_preview_declares_redacted_span_fields() {
        let report = hepta_work_graph_observability_timeline_preview_report();
        let required_fields = report
            .span_fields
            .iter()
            .filter(|field| field.required)
            .map(|field| field.wire_name)
            .collect::<Vec<_>>();

        assert_eq!(
            required_fields,
            [
                "traceId",
                "nodeId",
                "sourceSurfaceId",
                "eventKind",
                "status",
                "evidenceRefs",
                "redactionState",
                "sideEffectClass",
            ]
        );
        assert_eq!(report.span_field_count, 11);
        assert!(
            report
                .span_fields
                .iter()
                .any(|field| field.wire_name == "evidenceRefs" && field.redaction_required)
        );
    }

    #[test]
    fn observability_timeline_preview_projects_existing_trace_surfaces() {
        let report = hepta_work_graph_observability_timeline_preview_report();
        let adapter_ids = report
            .adapter_previews
            .iter()
            .map(|adapter| adapter.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            adapter_ids,
            [
                "app_server_turn_plan_notification",
                "multi_agent_v2_thread_spawn",
                "multi_agent_v2_mailbox_wait",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.adapter_preview_count, 7);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_OBSERVABILITY_TIMELINE_RECOMMENDED_NEXT_GATE
        );
    }
}
