use serde::Serialize;

use crate::work_graph_adapter_task_result_index::WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE;
use crate::work_graph_append_only_event_intake_preview::WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_shadow_write_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_shadow_write_readback_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_PREVIEW_GATE;
use crate::work_graph_scheduler_admission_dry_run_enforcement::{
    WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
    hepta_work_graph_scheduler_admission_dry_run_enforcement_report,
};
use crate::work_graph_source_id_alignment_readback::WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_GATE;
use crate::work_graph_task_result_contract_field_gap_readback::{
    WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_GATE,
    hepta_work_graph_task_result_contract_field_gap_readback_report,
};
use crate::work_graph_task_result_envelope_report_only_validator::WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE;
use crate::work_graph_terminal_envelope_readback::WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE;

pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE: &str =
    "hepta_work_graph_append_only_event_store_shadow_path_gate";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_SCHEMA_VERSION: &str =
    "work_graph_append_only_event_store_shadow_path_v1";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistent_mailbox_handoff_event_mapping_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventStoreShadowPathReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub event_record_count: usize,
    pub projection_index_count: usize,
    pub readback_evidence_count: usize,
    pub replay_diff_count: usize,
    pub scheduler_prior_gate_count: usize,
    pub required_prior_gate_count: usize,
    pub event_records: Vec<WorkGraphShadowEventRecordPreview>,
    pub projection_indexes: Vec<WorkGraphShadowProjectionIndexPreview>,
    pub readback_evidence: Vec<WorkGraphShadowReadbackEvidencePreview>,
    pub replay_diffs: Vec<WorkGraphShadowReplayDiffPreview>,
    pub scheduler_prior_gates: Vec<&'static str>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub redacted_payload_policy_ready: bool,
    pub deterministic_event_ids_ready: bool,
    pub projection_index_ready: bool,
    pub readback_evidence_ready: bool,
    pub replay_diff_ready: bool,
    pub scheduler_prior_chain_ready: bool,
    pub task_result_contract_field_gap_readback_ready: bool,
    pub append_only_shadow_path_readiness_complete: bool,
    pub shadow_store_write_enabled: bool,
    pub live_cutover_enabled: bool,
    pub ready_for_persistent_mailbox_handoff: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyEventStoreShadowPathSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventRecordPreview {
    pub source_surface_id: &'static str,
    pub event_kind: &'static str,
    pub deterministic_event_id: &'static str,
    pub deterministic_id_inputs: Vec<&'static str>,
    pub redacted_payload_ref: &'static str,
    pub payload_hash: &'static str,
    pub projection_index_key: &'static str,
    pub readback_evidence_ref: &'static str,
    pub replay_diff_ref: &'static str,
    pub trace_id: &'static str,
    pub shadow_persisted: bool,
    pub live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowProjectionIndexPreview {
    pub id: &'static str,
    pub collection_id: &'static str,
    pub key_fields: Vec<&'static str>,
    pub event_kind_refs: Vec<&'static str>,
    pub deterministic_order: &'static str,
    pub index_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowReadbackEvidencePreview {
    pub id: &'static str,
    pub readback_target: &'static str,
    pub required_event_fields: Vec<&'static str>,
    pub evidence_ref: &'static str,
    pub readback_status: &'static str,
    pub readback_executed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowReplayDiffPreview {
    pub id: &'static str,
    pub replay_scope: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub expected_diff: &'static str,
    pub replay_executed: bool,
    pub diff_persisted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventStoreShadowPathSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub event_store_enabled: bool,
    pub shadow_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub readback_executed: bool,
    pub replay_executed: bool,
    pub replay_diff_persisted: bool,
    pub idempotency_index_mutated: bool,
    pub scheduler_admission_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_event_store_shadow_path_report()
-> WorkGraphAppendOnlyEventStoreShadowPathReport {
    let event_records = work_graph_append_only_event_store_shadow_path_records();
    let projection_indexes = work_graph_append_only_event_store_shadow_projection_indexes();
    let readback_evidence = work_graph_append_only_event_store_shadow_readback_evidence();
    let replay_diffs = work_graph_append_only_event_store_shadow_replay_diffs();
    let scheduler_admission = hepta_work_graph_scheduler_admission_dry_run_enforcement_report();
    let field_gap_readback = hepta_work_graph_task_result_contract_field_gap_readback_report();
    let scheduler_prior_gates =
        work_graph_append_only_event_store_shadow_path_scheduler_prior_gates();
    let required_prior_gates =
        work_graph_append_only_event_store_shadow_path_required_prior_gates();
    let scheduler_prior_chain_ready = scheduler_admission.required_prior_gates
        == scheduler_prior_gates
        && scheduler_admission.ready_for_append_only_event_store_shadow_path
        && !scheduler_admission.live_blocking_enforcement_enabled;
    let task_result_contract_field_gap_readback_ready = field_gap_readback
        .ready_for_append_only_event_store_shadow_path
        && field_gap_readback.gap_source_count == 0
        && field_gap_readback.contract_required_field_gap_count == 0
        && field_gap_readback.contract_terminal_field_gap_count == 0
        && !field_gap_readback.ready_for_task_result_enforcement;
    let append_only_shadow_path_readiness_complete = scheduler_prior_chain_ready
        && task_result_contract_field_gap_readback_ready
        && !event_records.is_empty()
        && !projection_indexes.is_empty()
        && !readback_evidence.is_empty()
        && !replay_diffs.is_empty();

    WorkGraphAppendOnlyEventStoreShadowPathReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_event_store_shadow_path_no_live_cutover",
        event_record_count: event_records.len(),
        projection_index_count: projection_indexes.len(),
        readback_evidence_count: readback_evidence.len(),
        replay_diff_count: replay_diffs.len(),
        scheduler_prior_gate_count: scheduler_prior_gates.len(),
        required_prior_gate_count: required_prior_gates.len(),
        event_records,
        projection_indexes,
        readback_evidence,
        replay_diffs,
        scheduler_prior_gates,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_RECOMMENDED_NEXT_GATE,
        redacted_payload_policy_ready: true,
        deterministic_event_ids_ready: true,
        projection_index_ready: true,
        readback_evidence_ready: true,
        replay_diff_ready: true,
        scheduler_prior_chain_ready,
        task_result_contract_field_gap_readback_ready,
        append_only_shadow_path_readiness_complete,
        shadow_store_write_enabled: false,
        live_cutover_enabled: false,
        ready_for_persistent_mailbox_handoff: append_only_shadow_path_readiness_complete,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyEventStoreShadowPathSideEffects::none(),
    }
}

pub fn work_graph_append_only_event_store_shadow_path_records()
-> Vec<WorkGraphShadowEventRecordPreview> {
    vec![
        shadow_event(
            "update_plan_tool",
            "PlanStepCreated",
            "wg-event-shadow-plan-001",
            "trace-shadow-plan-001",
            "idx:trace-shadow-plan-001:plan",
            "rb:shadow-plan-step-created-001",
            "diff:shadow-plan-step-created-001",
        ),
        shadow_event(
            "multi_agent_v2_thread_spawn",
            "AgentTaskSpawned",
            "wg-event-shadow-spawn-001",
            "trace-shadow-spawn-001",
            "idx:trace-shadow-spawn-001:agent",
            "rb:shadow-agent-task-spawned-001",
            "diff:shadow-agent-task-spawned-001",
        ),
        shadow_event(
            "multi_agent_v2_mailbox_wait",
            "MailboxEventLinked",
            "wg-event-shadow-mailbox-001",
            "trace-shadow-mailbox-001",
            "idx:trace-shadow-mailbox-001:mailbox",
            "rb:shadow-mailbox-event-linked-001",
            "diff:shadow-mailbox-event-linked-001",
        ),
        shadow_event(
            "agent_jobs_batch_workers",
            "TaskResultReported",
            "wg-event-shadow-agent-job-result-001",
            "trace-shadow-agent-job-001",
            "idx:trace-shadow-agent-job-001:task-result",
            "rb:shadow-agent-job-task-result-001",
            "diff:shadow-agent-job-task-result-001",
        ),
        shadow_event(
            "hepta_runtime_worker_tasks",
            "ArtifactProduced",
            "wg-event-shadow-worker-artifact-001",
            "trace-shadow-worker-001",
            "idx:trace-shadow-worker-001:artifact",
            "rb:shadow-worker-artifact-001",
            "diff:shadow-worker-artifact-001",
        ),
        shadow_event(
            "hepta_runtime_task_board",
            "TaskBoardTerminalEvent",
            "wg-event-shadow-task-board-terminal-001",
            "trace-shadow-task-board-001",
            "idx:trace-shadow-task-board-001:terminal",
            "rb:shadow-task-board-terminal-001",
            "diff:shadow-task-board-terminal-001",
        ),
        shadow_event(
            "hepta_runtime_scheduler_store",
            "SchedulerAdmissionEvaluated",
            "wg-event-shadow-scheduler-admission-001",
            "trace-shadow-scheduler-001",
            "idx:trace-shadow-scheduler-001:admission",
            "rb:shadow-scheduler-admission-001",
            "diff:shadow-scheduler-admission-001",
        ),
        shadow_event(
            "hepta_runtime_approval_broker",
            "GuardrailApprovalEvaluated",
            "wg-event-shadow-guardrail-approval-001",
            "trace-shadow-guardrail-001",
            "idx:trace-shadow-guardrail-001:approval",
            "rb:shadow-guardrail-approval-001",
            "diff:shadow-guardrail-approval-001",
        ),
    ]
}

pub fn work_graph_append_only_event_store_shadow_projection_indexes()
-> Vec<WorkGraphShadowProjectionIndexPreview> {
    vec![
        projection_index(
            "projection_by_trace_id",
            "timelineEvents",
            vec!["traceId", "eventId"],
            vec![
                "PlanStepCreated",
                "AgentTaskSpawned",
                "MailboxEventLinked",
                "TaskResultReported",
            ],
        ),
        projection_index(
            "projection_by_task_id",
            "taskResults",
            vec!["taskId", "eventId"],
            vec![
                "TaskResultReported",
                "TaskBoardTerminalEvent",
                "SchedulerAdmissionEvaluated",
            ],
        ),
        projection_index(
            "projection_by_source_surface",
            "nodes",
            vec!["sourceSurfaceId", "eventKind", "sequenceKey"],
            vec![
                "PlanStepCreated",
                "AgentTaskSpawned",
                "ArtifactProduced",
                "GuardrailApprovalEvaluated",
            ],
        ),
        projection_index(
            "projection_by_parent_child_task",
            "edges",
            vec!["parentTaskId", "childTaskId", "eventId"],
            vec!["AgentTaskSpawned", "MailboxEventLinked"],
        ),
        projection_index(
            "projection_by_replay_diff",
            "timelineEvents",
            vec!["replayDiffRef", "payloadHash", "eventId"],
            vec![
                "SchedulerAdmissionEvaluated",
                "TaskResultReported",
                "GuardrailApprovalEvaluated",
            ],
        ),
    ]
}

pub fn work_graph_append_only_event_store_shadow_readback_evidence()
-> Vec<WorkGraphShadowReadbackEvidencePreview> {
    vec![
        readback_evidence(
            "shadow_readback_event_id_lookup",
            "event_id",
            vec!["eventId", "eventKind", "traceId"],
            "evidence:shadow-event-id-lookup",
        ),
        readback_evidence(
            "shadow_readback_payload_hash_check",
            "payload_hash",
            vec!["eventId", "payloadHash", "redactedPayloadRef"],
            "evidence:shadow-payload-hash-check",
        ),
        readback_evidence(
            "shadow_readback_projection_index_lookup",
            "projection_index",
            vec!["projectionIndexKey", "collectionId", "deterministicOrder"],
            "evidence:shadow-projection-index-lookup",
        ),
        readback_evidence(
            "shadow_readback_terminal_task_result_join",
            "task_result_join",
            vec!["taskId", "traceId", "verifierRef"],
            "evidence:shadow-terminal-task-result-join",
        ),
        readback_evidence(
            "shadow_readback_scheduler_admission_join",
            "scheduler_admission_join",
            vec!["traceId", "admissionDecision", "failedChecks"],
            "evidence:shadow-scheduler-admission-join",
        ),
    ]
}

pub fn work_graph_append_only_event_store_shadow_replay_diffs()
-> Vec<WorkGraphShadowReplayDiffPreview> {
    vec![
        replay_diff(
            "shadow_replay_noop_projection_diff",
            "single_trace_projection",
            vec!["eventId", "projectionIndexKey", "payloadHash"],
            "no_diff_preview",
        ),
        replay_diff(
            "shadow_replay_duplicate_event_suppression_diff",
            "idempotency_window",
            vec!["deterministicEventId", "idempotencyKey", "payloadHash"],
            "duplicate_suppressed_preview",
        ),
        replay_diff(
            "shadow_replay_projection_index_rebuild_diff",
            "projection_index_rebuild",
            vec!["collectionId", "keyFields", "eventKindRefs"],
            "index_rebuild_matches_preview",
        ),
        replay_diff(
            "shadow_replay_redaction_hash_stability_diff",
            "redaction_hash_stability",
            vec!["redactedPayloadRef", "payloadHash", "evidenceRef"],
            "hash_stable_preview",
        ),
    ]
}

pub fn work_graph_append_only_event_store_shadow_path_required_prior_gates() -> Vec<&'static str> {
    let mut gates = work_graph_append_only_event_store_shadow_path_scheduler_prior_gates();
    gates.extend([
        WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
        WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_PREVIEW_GATE,
        WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_PREVIEW_GATE,
        WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_PREVIEW_GATE,
    ]);
    gates
}

pub fn work_graph_append_only_event_store_shadow_path_scheduler_prior_gates() -> Vec<&'static str> {
    vec![
        WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
        WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE,
        WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE,
        WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_GATE,
        WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_GATE,
    ]
}

impl WorkGraphAppendOnlyEventStoreShadowPathSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            event_store_enabled: false,
            shadow_event_persisted: false,
            projection_index_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            readback_executed: false,
            replay_executed: false,
            replay_diff_persisted: false,
            idempotency_index_mutated: false,
            scheduler_admission_enforced: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn shadow_event(
    source_surface_id: &'static str,
    event_kind: &'static str,
    deterministic_event_id: &'static str,
    trace_id: &'static str,
    projection_index_key: &'static str,
    readback_evidence_ref: &'static str,
    replay_diff_ref: &'static str,
) -> WorkGraphShadowEventRecordPreview {
    WorkGraphShadowEventRecordPreview {
        source_surface_id,
        event_kind,
        deterministic_event_id,
        deterministic_id_inputs: vec![
            "sourceSurfaceId",
            "traceId",
            "eventKind",
            "sequenceKey",
            "payloadHash",
        ],
        redacted_payload_ref: "redacted:work-graph-shadow-payload",
        payload_hash: "sha256:shadow-payload-preview-hash",
        projection_index_key,
        readback_evidence_ref,
        replay_diff_ref,
        trace_id,
        shadow_persisted: false,
        live_cutover_enabled: false,
    }
}

fn projection_index(
    id: &'static str,
    collection_id: &'static str,
    key_fields: Vec<&'static str>,
    event_kind_refs: Vec<&'static str>,
) -> WorkGraphShadowProjectionIndexPreview {
    WorkGraphShadowProjectionIndexPreview {
        id,
        collection_id,
        key_fields,
        event_kind_refs,
        deterministic_order: "traceId:eventId:sequenceKey",
        index_persisted: false,
    }
}

fn readback_evidence(
    id: &'static str,
    readback_target: &'static str,
    required_event_fields: Vec<&'static str>,
    evidence_ref: &'static str,
) -> WorkGraphShadowReadbackEvidencePreview {
    WorkGraphShadowReadbackEvidencePreview {
        id,
        readback_target,
        required_event_fields,
        evidence_ref,
        readback_status: "readback_evidence_ready_not_executed",
        readback_executed: false,
    }
}

fn replay_diff(
    id: &'static str,
    replay_scope: &'static str,
    compared_fields: Vec<&'static str>,
    expected_diff: &'static str,
) -> WorkGraphShadowReplayDiffPreview {
    WorkGraphShadowReplayDiffPreview {
        id,
        replay_scope,
        compared_fields,
        expected_diff,
        replay_executed: false,
        diff_persisted: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_only_event_store_shadow_path_projects_requested_event_records() {
        let report = hepta_work_graph_append_only_event_store_shadow_path_report();
        let event_kinds = report
            .event_records
            .iter()
            .map(|record| record.event_kind)
            .collect::<Vec<_>>();

        assert_eq!(report.event_record_count, 8);
        assert!(event_kinds.contains(&"PlanStepCreated"));
        assert!(event_kinds.contains(&"AgentTaskSpawned"));
        assert!(event_kinds.contains(&"MailboxEventLinked"));
        assert!(event_kinds.contains(&"TaskResultReported"));
        assert!(event_kinds.contains(&"SchedulerAdmissionEvaluated"));
        assert!(report.event_records.iter().all(|record| {
            record
                .deterministic_event_id
                .starts_with("wg-event-shadow-")
                && record.payload_hash.starts_with("sha256:")
                && record.redacted_payload_ref.starts_with("redacted:")
                && !record.shadow_persisted
                && !record.live_cutover_enabled
        }));
    }

    #[test]
    fn append_only_event_store_shadow_path_declares_indexes_readback_and_replay_diff() {
        let report = hepta_work_graph_append_only_event_store_shadow_path_report();

        assert_eq!(report.projection_index_count, 5);
        assert_eq!(report.readback_evidence_count, 5);
        assert_eq!(report.replay_diff_count, 4);
        assert!(report.projection_indexes.iter().all(|index| {
            !index.index_persisted
                && !index.key_fields.is_empty()
                && !index.event_kind_refs.is_empty()
        }));
        assert!(
            report
                .readback_evidence
                .iter()
                .all(|evidence| !evidence.readback_executed)
        );
        assert!(
            report
                .replay_diffs
                .iter()
                .all(|diff| !diff.replay_executed && !diff.diff_persisted)
        );
    }

    #[test]
    fn append_only_event_store_shadow_path_links_required_prior_gates() {
        let report = hepta_work_graph_append_only_event_store_shadow_path_report();

        assert_eq!(
            report.scheduler_prior_gates,
            vec![
                WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
                WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE,
                WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE,
                WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_GATE,
                WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_GATE,
            ]
        );
        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
                WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE,
                WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE,
                WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_GATE,
                WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_GATE,
                WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE,
                WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_PREVIEW_GATE,
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_PREVIEW_GATE,
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_SHADOW_WRITE_READBACK_PREVIEW_GATE,
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.scheduler_prior_gate_count, 5);
        assert_eq!(report.required_prior_gate_count, 9);
        assert!(report.scheduler_prior_chain_ready);
        assert!(report.task_result_contract_field_gap_readback_ready);
        assert!(report.append_only_shadow_path_readiness_complete);
        assert!(report.ready_for_persistent_mailbox_handoff);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn append_only_event_store_shadow_path_keeps_all_side_effects_disabled() {
        let report = hepta_work_graph_append_only_event_store_shadow_path_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAppendOnlyEventStoreShadowPathSideEffects::none()
        );
        assert!(!report.shadow_store_write_enabled);
        assert!(!report.live_cutover_enabled);
    }
}
