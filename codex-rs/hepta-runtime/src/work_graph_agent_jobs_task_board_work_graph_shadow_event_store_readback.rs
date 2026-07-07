use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE;
use crate::work_graph_agent_jobs_task_board_canary_readback_replay::WorkGraphAgentJobsTaskBoardCanaryReadbackReplaySideEffects;
use crate::work_graph_agent_jobs_task_board_canary_readback_replay::hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report;
use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE;
use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint::WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointSideEffects;
use crate::work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint::hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report;
use crate::work_graph_append_only_event_store_shadow_path::WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE;
use crate::work_graph_append_only_event_store_shadow_path::WorkGraphAppendOnlyEventStoreShadowPathSideEffects;
use crate::work_graph_append_only_event_store_shadow_path::hepta_work_graph_append_only_event_store_shadow_path_report;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE: &str =
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_scheduler_guardrail_gate: &'static str,
    pub source_entrypoint_binding_count: usize,
    pub source_dry_run_decision_count: usize,
    pub source_scheduler_guardrail_ready: bool,
    pub source_scheduler_guardrail_no_live_confirmed: bool,
    pub source_shadow_path_gate: &'static str,
    pub source_shadow_event_record_count: usize,
    pub source_projection_index_count: usize,
    pub source_readback_evidence_count: usize,
    pub source_replay_diff_count: usize,
    pub source_shadow_path_readiness_complete: bool,
    pub source_shadow_path_no_persistence_confirmed: bool,
    pub source_canary_readback_replay_gate: &'static str,
    pub source_canary_entrypoint_count: usize,
    pub source_canary_projection_index_count: usize,
    pub source_canary_readback_evidence_count: usize,
    pub source_canary_replay_diff_count: usize,
    pub source_canary_readback_replay_ready: bool,
    pub source_canary_readback_replay_no_live_confirmed: bool,
    pub readback_entry_count: usize,
    pub shadow_event_join_count: usize,
    pub non_persistence_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub readback_entries: Vec<WorkGraphShadowEventStoreReadbackEntryPreview>,
    pub shadow_event_joins: Vec<WorkGraphShadowEventStoreEntrypointJoinPreview>,
    pub non_persistence_blockers: Vec<WorkGraphShadowEventStoreReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub source_prior_readbacks_complete: bool,
    pub readback_entries_visible_only_complete: bool,
    pub shadow_event_joins_report_only_complete: bool,
    pub non_persistence_blockers_complete: bool,
    pub shadow_event_store_readback_ready: bool,
    pub entrypoint_shadow_event_join_ready: bool,
    pub redacted_payload_hash_join_ready: bool,
    pub projection_index_readback_ready: bool,
    pub canary_readback_join_ready: bool,
    pub replay_diff_readback_ready: bool,
    pub shadow_readback_executed: bool,
    pub shadow_event_persistence_enabled: bool,
    pub projection_index_persistence_enabled: bool,
    pub scheduler_guardrail_live_enforcement_enabled: bool,
    pub runtime_interception_enabled: bool,
    pub ready_for_replay_diff_dry_run: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReadbackEntryPreview {
    pub id: &'static str,
    pub source_ref: &'static str,
    pub readback_target: &'static str,
    pub required_fields: Vec<&'static str>,
    pub evidence_ref: &'static str,
    pub status: &'static str,
    pub visible: bool,
    pub executed: bool,
    pub recorded: bool,
    pub persisted: bool,
    pub authoritative: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreEntrypointJoinPreview {
    pub id: &'static str,
    pub entrypoint_id: &'static str,
    pub source_surface_id: &'static str,
    pub dry_run_trace_id: &'static str,
    pub shadow_event_kind: &'static str,
    pub shadow_event_ref: &'static str,
    pub scheduler_event_ref: &'static str,
    pub projection_index_ref: &'static str,
    pub readback_evidence_ref: &'static str,
    pub replay_diff_ref: &'static str,
    pub joined: bool,
    pub persisted: bool,
    pub live_enforced: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReadbackBlockerPreview {
    pub id: &'static str,
    pub blocks: &'static str,
    pub reason: &'static str,
    pub required_before_enablement: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub shadow_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub readback_executed: bool,
    pub readback_recorded: bool,
    pub readback_persisted: bool,
    pub replay_executed: bool,
    pub replay_diff_persisted: bool,
    pub scheduler_admission_enforced: bool,
    pub guardrail_enforcement_enabled: bool,
    pub runtime_interception_enabled: bool,
    pub config_written: bool,
    pub feature_flag_mutated: bool,
    pub canary_traffic_routed: bool,
    pub operator_review_requested: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_report()
-> WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackReport {
    let scheduler_guardrail =
        hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_report();
    let shadow_path = hepta_work_graph_append_only_event_store_shadow_path_report();
    let canary_readback = hepta_work_graph_agent_jobs_task_board_canary_readback_replay_report();
    let readback_entries =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_entries();
    let shadow_event_joins =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_entrypoint_joins();
    let non_persistence_blockers =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_required_prior_gates(
        );
    let source_scheduler_guardrail_no_live_confirmed = !scheduler_guardrail
        .live_blocking_enforcement_enabled
        && !scheduler_guardrail.runtime_interception_enabled
        && !scheduler_guardrail.work_graph_event_persistence_enabled
        && !scheduler_guardrail.ready_for_live_execution
        && scheduler_guardrail.side_effects
            == WorkGraphAgentJobsTaskBoardSchedulerGuardrailBlockingDryRunEntrypointSideEffects::none();
    let source_scheduler_guardrail_ready = scheduler_guardrail.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE
        && scheduler_guardrail.prior_readbacks_complete
        && scheduler_guardrail.pre_entrypoint_hook_contract_ready
        && scheduler_guardrail.ready_for_work_graph_shadow_event_store_readback
        && scheduler_guardrail.entrypoint_binding_count == 4
        && scheduler_guardrail.dry_run_decision_count == 4
        && source_scheduler_guardrail_no_live_confirmed;
    let source_shadow_path_no_persistence_confirmed = !shadow_path.shadow_store_write_enabled
        && !shadow_path.live_cutover_enabled
        && !shadow_path.ready_for_live_execution
        && shadow_path
            .event_records
            .iter()
            .all(|event| !event.shadow_persisted && !event.live_cutover_enabled)
        && shadow_path
            .projection_indexes
            .iter()
            .all(|index| !index.index_persisted)
        && shadow_path
            .readback_evidence
            .iter()
            .all(|evidence| !evidence.readback_executed)
        && shadow_path
            .replay_diffs
            .iter()
            .all(|diff| !diff.replay_executed && !diff.diff_persisted)
        && shadow_path.side_effects == WorkGraphAppendOnlyEventStoreShadowPathSideEffects::none();
    let source_shadow_path_readiness_complete = shadow_path.gate
        == WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE
        && shadow_path.append_only_shadow_path_readiness_complete
        && shadow_path.redacted_payload_policy_ready
        && shadow_path.deterministic_event_ids_ready
        && shadow_path.projection_index_ready
        && shadow_path.readback_evidence_ready
        && shadow_path.replay_diff_ready
        && shadow_path.event_record_count == 8
        && shadow_path.projection_index_count == 5
        && shadow_path.readback_evidence_count == 5
        && shadow_path.replay_diff_count == 4
        && source_shadow_path_no_persistence_confirmed;
    let source_canary_readback_replay_no_live_confirmed = !canary_readback.feature_flag_enabled
        && !canary_readback.ready_for_live_cutover
        && canary_readback.canary_entrypoints.iter().all(|entrypoint| {
            !entrypoint.live_blocking_enabled && !entrypoint.live_persistence_enabled
        })
        && canary_readback
            .projection_indexes
            .iter()
            .all(|index| !index.persisted)
        && canary_readback
            .readback_evidence
            .iter()
            .all(|evidence| !evidence.evidence_persisted)
        && canary_readback
            .replay_diffs
            .iter()
            .all(|diff| !diff.replay_executed)
        && canary_readback.side_effects
            == WorkGraphAgentJobsTaskBoardCanaryReadbackReplaySideEffects::none();
    let source_canary_readback_replay_ready = canary_readback.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE
        && canary_readback.canary_readback_replay_prior_readbacks_complete
        && canary_readback.canary_projection_readback_replay_preview_complete
        && canary_readback.ready_for_non_blocking_canary
        && canary_readback.canary_entrypoint_count == 2
        && canary_readback.projection_indexes.len() == 2
        && canary_readback.readback_evidence_count == 2
        && canary_readback.replay_diff_count == 2
        && source_canary_readback_replay_no_live_confirmed;
    let source_prior_readbacks_complete = source_scheduler_guardrail_ready
        && source_shadow_path_readiness_complete
        && source_canary_readback_replay_ready;
    let readback_entries_visible_only_complete = readback_entries.len() == 6
        && readback_entries.iter().all(|entry| {
            entry.visible
                && !entry.executed
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.required_fields.is_empty()
        });
    let shadow_event_joins_report_only_complete = shadow_event_joins.len() == 4
        && shadow_event_joins.iter().all(|join| {
            join.joined
                && !join.persisted
                && !join.live_enforced
                && join.dry_run_trace_id.starts_with("trace-blocking-dry-run-")
                && join.shadow_event_ref.starts_with("wg-event-shadow-")
                && join.scheduler_event_ref == "wg-event-shadow-scheduler-admission-001"
        });
    let non_persistence_blockers_complete = non_persistence_blockers.len() == 14
        && non_persistence_blockers
            .iter()
            .all(|blocker| blocker.required_before_enablement);
    let entrypoint_shadow_event_join_ready =
        source_scheduler_guardrail_ready && shadow_event_joins_report_only_complete;
    let redacted_payload_hash_join_ready = source_shadow_path_readiness_complete
        && readback_entries
            .iter()
            .any(|entry| entry.id == "redacted_payload_hash_shadow_readback");
    let projection_index_readback_ready = source_shadow_path_readiness_complete
        && readback_entries
            .iter()
            .any(|entry| entry.id == "projection_index_shadow_readback");
    let canary_readback_join_ready = source_canary_readback_replay_ready
        && readback_entries
            .iter()
            .any(|entry| entry.id == "canary_report_only_shadow_readback");
    let replay_diff_readback_ready = source_shadow_path_readiness_complete
        && readback_entries
            .iter()
            .any(|entry| entry.id == "replay_diff_preview_shadow_readback");
    let shadow_event_store_readback_ready = source_prior_readbacks_complete
        && readback_entries_visible_only_complete
        && shadow_event_joins_report_only_complete
        && non_persistence_blockers_complete
        && entrypoint_shadow_event_join_ready
        && redacted_payload_hash_join_ready
        && projection_index_readback_ready
        && canary_readback_join_ready
        && replay_diff_readback_ready;

    WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: if shadow_event_store_readback_ready {
            "ready"
        } else {
            "blocked"
        },
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_SCHEMA_VERSION,
        preview_mode: "work_graph_shadow_event_store_readback_ready_no_persistence_no_live",
        source_scheduler_guardrail_gate: scheduler_guardrail.gate,
        source_entrypoint_binding_count: scheduler_guardrail.entrypoint_binding_count,
        source_dry_run_decision_count: scheduler_guardrail.dry_run_decision_count,
        source_scheduler_guardrail_ready,
        source_scheduler_guardrail_no_live_confirmed,
        source_shadow_path_gate: shadow_path.gate,
        source_shadow_event_record_count: shadow_path.event_record_count,
        source_projection_index_count: shadow_path.projection_index_count,
        source_readback_evidence_count: shadow_path.readback_evidence_count,
        source_replay_diff_count: shadow_path.replay_diff_count,
        source_shadow_path_readiness_complete,
        source_shadow_path_no_persistence_confirmed,
        source_canary_readback_replay_gate: canary_readback.gate,
        source_canary_entrypoint_count: canary_readback.canary_entrypoint_count,
        source_canary_projection_index_count: canary_readback.projection_indexes.len(),
        source_canary_readback_evidence_count: canary_readback.readback_evidence_count,
        source_canary_replay_diff_count: canary_readback.replay_diff_count,
        source_canary_readback_replay_ready,
        source_canary_readback_replay_no_live_confirmed,
        readback_entry_count: readback_entries.len(),
        shadow_event_join_count: shadow_event_joins.len(),
        non_persistence_blocker_count: non_persistence_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        readback_entries,
        shadow_event_joins,
        non_persistence_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_RECOMMENDED_NEXT_GATE,
        source_prior_readbacks_complete,
        readback_entries_visible_only_complete,
        shadow_event_joins_report_only_complete,
        non_persistence_blockers_complete,
        shadow_event_store_readback_ready,
        entrypoint_shadow_event_join_ready,
        redacted_payload_hash_join_ready,
        projection_index_readback_ready,
        canary_readback_join_ready,
        replay_diff_readback_ready,
        shadow_readback_executed: false,
        shadow_event_persistence_enabled: false,
        projection_index_persistence_enabled: false,
        scheduler_guardrail_live_enforcement_enabled: false,
        runtime_interception_enabled: false,
        ready_for_replay_diff_dry_run: shadow_event_store_readback_ready,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_entries()
-> Vec<WorkGraphShadowEventStoreReadbackEntryPreview> {
    vec![
        readback_entry(
            "entrypoint_dry_run_decision_shadow_event_readback",
            "scheduler_guardrail_blocking_dry_run_entrypoint",
            "entrypoint_dry_run_decision_join",
            vec![
                "entrypointId",
                "dryRunDecision",
                "traceId",
                "deterministicEventId",
            ],
            "evidence:entrypoint-dry-run-shadow-event-readback",
        ),
        readback_entry(
            "redacted_payload_hash_shadow_readback",
            "append_only_event_store_shadow_path",
            "redacted_payload_hash",
            vec!["eventId", "redactedPayloadRef", "payloadHash"],
            "evidence:redacted-payload-hash-shadow-readback",
        ),
        readback_entry(
            "projection_index_shadow_readback",
            "append_only_event_store_shadow_path",
            "projection_index",
            vec!["projectionIndexKey", "collectionId", "deterministicOrder"],
            "evidence:projection-index-shadow-readback",
        ),
        readback_entry(
            "canary_report_only_shadow_readback",
            "agent_jobs_task_board_canary_readback_replay",
            "canary_report_only_join",
            vec!["workGraphReportOnly", "taskId", "traceId"],
            "evidence:canary-report-only-shadow-readback",
        ),
        readback_entry(
            "replay_diff_preview_shadow_readback",
            "append_only_event_store_shadow_path",
            "replay_diff_preview",
            vec!["replayDiffRef", "payloadHash", "expectedDiff"],
            "evidence:replay-diff-preview-shadow-readback",
        ),
        readback_entry(
            "shadow_event_store_non_persistence_readback",
            "shadow_event_store_boundary",
            "non_persistence_boundary",
            vec!["shadowPersisted", "eventStoreEnabled", "liveCutoverEnabled"],
            "evidence:shadow-event-store-non-persistence-readback",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_entrypoint_joins()
-> Vec<WorkGraphShadowEventStoreEntrypointJoinPreview> {
    vec![
        shadow_event_join(
            "spawn_agent_shadow_event_store_readback_join",
            "spawn_agent",
            "multi_agent_v2_thread_spawn",
            "trace-blocking-dry-run-spawn-agent-001",
            "AgentTaskSpawned",
            "wg-event-shadow-spawn-001",
            "wg-event-shadow-scheduler-admission-001",
            "projection_by_trace_id",
            "shadow_readback_scheduler_admission_join",
            "shadow_replay_noop_projection_diff",
        ),
        shadow_event_join(
            "agent_jobs_csv_shadow_event_store_readback_join",
            "spawn_agents_on_csv",
            "agent_jobs_batch_workers",
            "trace-blocking-dry-run-agent-jobs-001",
            "TaskResultReported",
            "wg-event-shadow-agent-job-result-001",
            "wg-event-shadow-scheduler-admission-001",
            "projection_by_task_id",
            "shadow_readback_terminal_task_result_join",
            "shadow_replay_duplicate_event_suppression_diff",
        ),
        shadow_event_join(
            "task_board_claim_shadow_event_store_readback_join",
            "task_board_claim",
            "hepta_runtime_task_board",
            "trace-blocking-dry-run-task-board-001",
            "TaskBoardTerminalEvent",
            "wg-event-shadow-task-board-terminal-001",
            "wg-event-shadow-scheduler-admission-001",
            "projection_by_task_id",
            "shadow_readback_scheduler_admission_join",
            "shadow_replay_projection_index_rebuild_diff",
        ),
        shadow_event_join(
            "worker_task_run_shadow_event_store_readback_join",
            "worker_task_run",
            "hepta_runtime_worker_tasks",
            "trace-blocking-dry-run-worker-task-001",
            "ArtifactProduced",
            "wg-event-shadow-worker-artifact-001",
            "wg-event-shadow-scheduler-admission-001",
            "projection_by_source_surface",
            "shadow_readback_payload_hash_check",
            "shadow_replay_redaction_hash_stability_diff",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_blockers()
-> Vec<WorkGraphShadowEventStoreReadbackBlockerPreview> {
    vec![
        blocker(
            "shadow_event_store_enablement_blocked",
            "event_store_enablement",
        ),
        blocker(
            "shadow_event_persistence_blocked",
            "shadow_event_persistence",
        ),
        blocker(
            "projection_index_persistence_blocked",
            "projection_index_persistence",
        ),
        blocker("readback_execution_blocked", "readback_execution"),
        blocker("readback_recording_blocked", "readback_recording"),
        blocker("replay_execution_blocked", "replay_execution"),
        blocker("replay_diff_persistence_blocked", "replay_diff_persistence"),
        blocker(
            "scheduler_guardrail_live_enforcement_blocked",
            "scheduler_guardrail_live_enforcement",
        ),
        blocker("runtime_interception_blocked", "runtime_interception"),
        blocker("feature_flag_enablement_blocked", "feature_flag_enablement"),
        blocker("canary_traffic_blocked", "canary_traffic"),
        blocker("operator_review_request_blocked", "operator_review_request"),
        blocker("approval_recording_blocked", "approval_recording"),
        blocker("live_cutover_blocked", "live_cutover"),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE,
        WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            shadow_event_persisted: false,
            projection_index_persisted: false,
            readback_executed: false,
            readback_recorded: false,
            readback_persisted: false,
            replay_executed: false,
            replay_diff_persisted: false,
            scheduler_admission_enforced: false,
            guardrail_enforcement_enabled: false,
            runtime_interception_enabled: false,
            config_written: false,
            feature_flag_mutated: false,
            canary_traffic_routed: false,
            operator_review_requested: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn readback_entry(
    id: &'static str,
    source_ref: &'static str,
    readback_target: &'static str,
    required_fields: Vec<&'static str>,
    evidence_ref: &'static str,
) -> WorkGraphShadowEventStoreReadbackEntryPreview {
    WorkGraphShadowEventStoreReadbackEntryPreview {
        id,
        source_ref,
        readback_target,
        required_fields,
        evidence_ref,
        status: "readback_ready_not_executed",
        visible: true,
        executed: false,
        recorded: false,
        persisted: false,
        authoritative: false,
    }
}

fn shadow_event_join(
    id: &'static str,
    entrypoint_id: &'static str,
    source_surface_id: &'static str,
    dry_run_trace_id: &'static str,
    shadow_event_kind: &'static str,
    shadow_event_ref: &'static str,
    scheduler_event_ref: &'static str,
    projection_index_ref: &'static str,
    readback_evidence_ref: &'static str,
    replay_diff_ref: &'static str,
) -> WorkGraphShadowEventStoreEntrypointJoinPreview {
    WorkGraphShadowEventStoreEntrypointJoinPreview {
        id,
        entrypoint_id,
        source_surface_id,
        dry_run_trace_id,
        shadow_event_kind,
        shadow_event_ref,
        scheduler_event_ref,
        projection_index_ref,
        readback_evidence_ref,
        replay_diff_ref,
        joined: true,
        persisted: false,
        live_enforced: false,
    }
}

fn blocker(
    id: &'static str,
    blocks: &'static str,
) -> WorkGraphShadowEventStoreReadbackBlockerPreview {
    WorkGraphShadowEventStoreReadbackBlockerPreview {
        id,
        blocks,
        reason: "required before shadow event-store readback can become persisted or live",
        required_before_enablement: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_event_store_readback_derives_from_scheduler_shadow_path_and_canary() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_report();

        assert_eq!(
            report.source_scheduler_guardrail_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE
        );
        assert_eq!(
            report.source_shadow_path_gate,
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE
        );
        assert_eq!(
            report.source_canary_readback_replay_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE
        );
        assert_eq!(report.source_entrypoint_binding_count, 4);
        assert_eq!(report.source_dry_run_decision_count, 4);
        assert!(report.source_scheduler_guardrail_ready);
        assert!(report.source_scheduler_guardrail_no_live_confirmed);
        assert_eq!(report.source_shadow_event_record_count, 8);
        assert_eq!(report.source_projection_index_count, 5);
        assert_eq!(report.source_readback_evidence_count, 5);
        assert_eq!(report.source_replay_diff_count, 4);
        assert!(report.source_shadow_path_readiness_complete);
        assert!(report.source_shadow_path_no_persistence_confirmed);
        assert_eq!(report.source_canary_entrypoint_count, 2);
        assert_eq!(report.source_canary_projection_index_count, 2);
        assert_eq!(report.source_canary_readback_evidence_count, 2);
        assert_eq!(report.source_canary_replay_diff_count, 2);
        assert!(report.source_canary_readback_replay_ready);
        assert!(report.source_canary_readback_replay_no_live_confirmed);
        assert!(report.source_prior_readbacks_complete);
    }

    #[test]
    fn shadow_event_store_readback_declares_entrypoint_joins() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_report();
        let entrypoint_ids = report
            .shadow_event_joins
            .iter()
            .map(|join| join.entrypoint_id)
            .collect::<Vec<_>>();

        assert_eq!(
            entrypoint_ids,
            vec![
                "spawn_agent",
                "spawn_agents_on_csv",
                "task_board_claim",
                "worker_task_run"
            ]
        );
        assert_eq!(report.shadow_event_join_count, 4);
        assert!(report.shadow_event_joins.iter().all(|join| {
            join.joined
                && !join.persisted
                && !join.live_enforced
                && join.dry_run_trace_id.starts_with("trace-blocking-dry-run-")
                && join.shadow_event_ref.starts_with("wg-event-shadow-")
        }));
        assert!(report.shadow_event_joins_report_only_complete);
    }

    #[test]
    fn shadow_event_store_readback_stays_non_persistent_and_non_live() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_report();

        assert_eq!(report.readback_entry_count, 6);
        assert_eq!(report.non_persistence_blocker_count, 14);
        assert!(report.readback_entries_visible_only_complete);
        assert!(report.non_persistence_blockers_complete);
        assert!(report.shadow_event_store_readback_ready);
        assert!(report.ready_for_replay_diff_dry_run);
        assert!(!report.shadow_readback_executed);
        assert!(!report.shadow_event_persistence_enabled);
        assert!(!report.projection_index_persistence_enabled);
        assert!(!report.scheduler_guardrail_live_enforcement_enabled);
        assert!(!report.runtime_interception_enabled);
        assert!(!report.ready_for_live_execution);
        assert!(report.readback_entries.iter().all(|entry| {
            entry.visible
                && !entry.executed
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
        }));
        assert!(
            report
                .non_persistence_blockers
                .iter()
                .all(|blocker| blocker.required_before_enablement)
        );
    }

    #[test]
    fn shadow_event_store_readback_links_required_priors_and_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_SCHEDULER_GUARDRAIL_BLOCKING_DRY_RUN_ENTRYPOINT_GATE,
                WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_CANARY_READBACK_REPLAY_GATE,
            ]
        );
        assert_eq!(report.required_prior_gate_count, 3);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(!report.side_effects.filesystem_written);
        assert!(!report.side_effects.work_graph_event_persisted);
        assert!(!report.side_effects.readback_executed);
        assert!(!report.side_effects.scheduler_admission_enforced);
        assert!(!report.side_effects.guardrail_enforcement_enabled);
        assert!(!report.side_effects.agent_spawn_performed);
    }
}
