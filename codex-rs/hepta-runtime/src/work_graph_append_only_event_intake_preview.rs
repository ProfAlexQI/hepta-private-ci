use std::collections::BTreeSet;

use serde::Serialize;

use crate::work_graph_state_store_persistence_preview::work_graph_state_store_idempotency_guards;
use crate::work_graph_state_store_persistence_preview::work_graph_state_store_persistence_wal_operations;
use crate::work_graph_state_store_persistence_preview::work_graph_state_store_readback_probes;
use crate::work_graph_unified_projection_audit_preview::WorkGraphUnifiedProjectionSourceAudit;
use crate::work_graph_unified_projection_audit_preview::work_graph_unified_projection_source_audits;

pub const WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_event_intake_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_SCHEMA_VERSION: &str =
    "work_graph_append_only_event_intake_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_replay_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventIntakePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub event_contract_count: usize,
    pub source_route_count: usize,
    pub target_collection_count: usize,
    pub required_prior_gate_count: usize,
    pub blocker_count: usize,
    pub event_contracts: Vec<WorkGraphAppendOnlyEventContractPreview>,
    pub source_routes: Vec<WorkGraphAppendOnlyEventRoutePreview>,
    pub blockers: Vec<WorkGraphAppendOnlyEventIntakeBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_replay_readback_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_store_persistence: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyEventIntakePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventContractPreview {
    pub id: &'static str,
    pub record_kind: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub target_collection_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub idempotency_key_fields: Vec<&'static str>,
    pub redaction_policy: &'static str,
    pub append_order: &'static str,
    pub requires_task_result_contract: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventRoutePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub emitted_event_kind_ids: Vec<&'static str>,
    pub target_collection_ids: Vec<&'static str>,
    pub wal_operation_id: &'static str,
    pub idempotency_guard_id: Option<&'static str>,
    pub readback_probe_ids: Vec<&'static str>,
    pub intake_state: &'static str,
    pub blocker_ids: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventIntakeBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_event_kind_ids: Vec<&'static str>,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_store_enablement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventIntakePreviewSideEffects {
    pub filesystem_written: bool,
    pub event_record_persisted: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub readback_performed: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_event_intake_preview_report()
-> WorkGraphAppendOnlyEventIntakePreviewReport {
    let event_contracts = work_graph_append_only_event_contracts();
    let source_routes = work_graph_append_only_event_routes();
    let blockers = work_graph_append_only_event_intake_blockers();
    let required_prior_gates = work_graph_append_only_event_intake_required_prior_gates();
    let target_collection_count = event_contracts
        .iter()
        .flat_map(|contract| contract.target_collection_ids.iter().copied())
        .collect::<BTreeSet<_>>()
        .len();

    WorkGraphAppendOnlyEventIntakePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_event_intake_preview_no_writes",
        event_contract_count: event_contracts.len(),
        source_route_count: source_routes.len(),
        target_collection_count,
        required_prior_gate_count: required_prior_gates.len(),
        blocker_count: blockers.len(),
        event_contracts,
        source_routes,
        blockers,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_RECOMMENDED_NEXT_GATE,
        ready_for_replay_readback_preview: true,
        ready_for_append_only_store_enablement: false,
        ready_for_store_persistence: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyEventIntakePreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_event_contracts() -> Vec<WorkGraphAppendOnlyEventContractPreview> {
    vec![
        event_contract(
            "plan_step_event_intake",
            "plan_step_event",
            vec![
                "update_plan_tool",
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
            ],
            vec!["nodes", "edges", "timelineEvents"],
            vec![
                "traceId",
                "eventId",
                "planStepId",
                "sourceSurfaceId",
                "status",
                "redactionState",
            ],
            vec!["traceId", "turnId", "stepIndex", "proposalHash"],
            "hash_raw_plan_text_and_store_redacted_refs_only",
            "append_after_plan_surface_parse_before_scheduler_visibility",
            false,
        ),
        event_contract(
            "agent_spawn_event_intake",
            "agent_spawn_event",
            vec!["multi_agent_v2_thread_spawn"],
            vec!["nodes", "edges", "timelineEvents"],
            vec![
                "traceId",
                "eventId",
                "parentThreadId",
                "childThreadId",
                "agentPath",
                "roleId",
                "status",
            ],
            vec!["parentThreadId", "childThreadId", "roleId"],
            "store_agent_paths_and_role_refs_without_prompt_payload",
            "append_after_role_manifest_preview_before_child_visibility",
            true,
        ),
        event_contract(
            "mailbox_delivery_event_intake",
            "mailbox_delivery_event",
            vec!["multi_agent_v2_mailbox_wait"],
            vec!["edges", "timelineEvents"],
            vec![
                "traceId",
                "eventId",
                "mailboxSeq",
                "agentPath",
                "deliveryState",
                "triggerTurn",
            ],
            vec!["traceId", "agentPath", "mailboxSeq"],
            "hash_inter_agent_message_content_and_store_delivery_refs",
            "append_after_mailbox_seq_observed_before_wait_completion",
            false,
        ),
        event_contract(
            "agent_job_item_event_intake",
            "agent_job_item_event",
            vec!["agent_jobs_batch_workers"],
            vec!["nodes", "taskResults", "timelineEvents"],
            vec!["traceId", "eventId", "jobId", "itemId", "attempt", "status"],
            vec!["jobId", "itemId", "attempt"],
            "store_worker_result_hash_and_schema_ref_not_raw_csv_payload",
            "append_after_job_item_status_read_before_csv_export_visibility",
            true,
        ),
        event_contract(
            "worker_task_event_intake",
            "worker_task_event",
            vec!["hepta_runtime_task_board", "hepta_runtime_worker_tasks"],
            vec!["nodes", "taskResults", "artifacts", "timelineEvents"],
            vec![
                "traceId",
                "eventId",
                "workerTaskId",
                "attempt",
                "status",
                "lane",
                "leaseState",
            ],
            vec!["workerTaskId", "attempt", "artifactHash"],
            "store_lease_and_artifact_hashes_without_command_payload",
            "append_after_lease_or_worker_state_observed_before_terminal_promotion",
            true,
        ),
        event_contract(
            "scheduler_run_event_intake",
            "scheduler_run_event",
            vec!["hepta_runtime_scheduler_store"],
            vec!["nodes", "edges", "taskResults", "timelineEvents"],
            vec![
                "traceId",
                "eventId",
                "schedulerRunId",
                "leaseId",
                "admissionDecision",
                "status",
            ],
            vec!["schedulerRunId", "leaseId", "admissionDecision"],
            "store_decision_hash_and_readback_ref_not_live_scheduler_state",
            "append_after_admission_preview_before_any_cutover",
            true,
        ),
        event_contract(
            "artifact_event_intake",
            "artifact_event",
            vec!["hepta_runtime_worker_tasks", "hepta_runtime_agent_harness"],
            vec!["artifacts", "timelineEvents"],
            vec![
                "traceId",
                "eventId",
                "artifactId",
                "producerNodeId",
                "artifactHash",
                "redactionState",
            ],
            vec!["artifactId", "producerNodeId", "artifactHash"],
            "store_artifact_hash_path_hint_and_redaction_state_only",
            "append_after_artifact_redaction_before_handoff_or_verifier_visibility",
            false,
        ),
        event_contract(
            "approval_event_intake",
            "approval_event",
            vec!["hepta_runtime_approval_broker"],
            vec!["approvals", "timelineEvents"],
            vec![
                "traceId",
                "eventId",
                "approvalId",
                "operatorScope",
                "status",
                "expiresAtUnixMs",
            ],
            vec!["approvalId", "operatorScope", "requestHash"],
            "store_request_hash_and_scope_without_recording_live_decision",
            "append_after_approval_request_projection_before_scheduler_unblock",
            false,
        ),
        event_contract(
            "task_result_event_intake",
            "task_result_event",
            vec![
                "multi_agent_v2_thread_spawn",
                "hepta_runtime_multi_agent_reducer",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
            ],
            vec!["taskResults", "timelineEvents"],
            vec![
                "traceId",
                "eventId",
                "taskId",
                "status",
                "summaryHash",
                "evidenceRefs",
                "verifierRef",
            ],
            vec!["traceId", "taskId", "status", "evidenceHash"],
            "store_task_result_contract_fields_and_evidence_refs_only",
            "append_after_task_result_contract_validation_before_terminal_promotion",
            true,
        ),
    ]
}

pub fn work_graph_append_only_event_routes() -> Vec<WorkGraphAppendOnlyEventRoutePreview> {
    let existing_guard_sources = work_graph_state_store_idempotency_guards()
        .into_iter()
        .map(|guard| guard.source_surface_id)
        .collect::<BTreeSet<_>>();
    let existing_wal_operation_ids = work_graph_state_store_persistence_wal_operations()
        .into_iter()
        .map(|operation| operation.id)
        .collect::<BTreeSet<_>>();
    let readback_collection_ids = work_graph_state_store_readback_probes()
        .into_iter()
        .map(|probe| probe.target_collection_id)
        .collect::<BTreeSet<_>>();

    work_graph_unified_projection_source_audits()
        .into_iter()
        .map(|source| {
            append_only_event_route(
                source,
                &existing_guard_sources,
                &existing_wal_operation_ids,
                &readback_collection_ids,
            )
        })
        .collect()
}

pub fn work_graph_append_only_event_intake_blockers()
-> Vec<WorkGraphAppendOnlyEventIntakeBlockerPreview> {
    vec![
        blocker(
            "source_projection_adapters_missing_for_new_surfaces",
            "high",
            vec![
                "plan_step_event_intake",
                "mailbox_delivery_event_intake",
                "worker_task_event_intake",
            ],
            vec![
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_task_board",
            ],
            "add unified store and timeline projections before these events can be appended",
        ),
        blocker(
            "idempotency_guards_missing_for_expanded_intake",
            "high",
            vec![
                "plan_step_event_intake",
                "mailbox_delivery_event_intake",
                "worker_task_event_intake",
                "task_result_event_intake",
            ],
            vec![
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ],
            "define stable replay keys for every newly covered source surface",
        ),
        blocker(
            "terminal_task_result_wrappers_not_enforced",
            "high",
            vec![
                "agent_spawn_event_intake",
                "agent_job_item_event_intake",
                "worker_task_event_intake",
                "scheduler_run_event_intake",
                "task_result_event_intake",
            ],
            vec![
                "multi_agent_v2_thread_spawn",
                "hepta_runtime_multi_agent_reducer",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
            ],
            "wrap all terminal completions in TaskResult before append-only promotion",
        ),
        blocker(
            "readback_is_contract_only",
            "medium",
            vec![
                "plan_step_event_intake",
                "agent_spawn_event_intake",
                "mailbox_delivery_event_intake",
                "agent_job_item_event_intake",
                "worker_task_event_intake",
                "scheduler_run_event_intake",
                "artifact_event_intake",
                "approval_event_intake",
                "task_result_event_intake",
            ],
            vec!["all_projected_source_surfaces"],
            "prove deterministic readback from appended events before enabling store writes",
        ),
        blocker(
            "append_only_store_disabled_by_design",
            "medium",
            vec![
                "plan_step_event_intake",
                "agent_spawn_event_intake",
                "mailbox_delivery_event_intake",
                "agent_job_item_event_intake",
                "worker_task_event_intake",
                "scheduler_run_event_intake",
                "artifact_event_intake",
                "approval_event_intake",
                "task_result_event_intake",
            ],
            vec!["all_projected_source_surfaces"],
            "keep this gate as preview-only until replay, retention, and operator readiness pass",
        ),
    ]
}

pub fn work_graph_append_only_event_intake_required_prior_gates() -> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
        "hepta_work_graph_unified_projection_audit_preview_gate",
        "hepta_work_graph_state_store_persistence_preview_gate",
    ]
}

impl WorkGraphAppendOnlyEventIntakePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            event_record_persisted: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            readback_performed: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn append_only_event_route(
    source: WorkGraphUnifiedProjectionSourceAudit,
    existing_guard_sources: &BTreeSet<&'static str>,
    existing_wal_operation_ids: &BTreeSet<&'static str>,
    readback_collection_ids: &BTreeSet<&'static str>,
) -> WorkGraphAppendOnlyEventRoutePreview {
    let emitted_event_kind_ids = emitted_event_kind_ids_for_source(source.source_surface_id);
    let target_collection_ids =
        target_collection_ids_for_events(&emitted_event_kind_ids, &source.projected_collection_ids);
    let wal_operation_id = wal_operation_id_for_events(&emitted_event_kind_ids);
    let idempotency_guard_id = idempotency_guard_id_for_source(
        source.source_surface_id,
        existing_guard_sources.contains(source.source_surface_id),
    );
    let readback_probe_ids =
        readback_probe_ids_for_collections(&target_collection_ids, readback_collection_ids);
    let blocker_ids = route_blockers(
        &source,
        wal_operation_id,
        idempotency_guard_id,
        &target_collection_ids,
        &readback_probe_ids,
        existing_wal_operation_ids,
    );

    WorkGraphAppendOnlyEventRoutePreview {
        source_surface_id: source.source_surface_id,
        source_category: source.source_category,
        emitted_event_kind_ids,
        target_collection_ids,
        wal_operation_id,
        idempotency_guard_id,
        readback_probe_ids,
        intake_state: route_intake_state(&source, idempotency_guard_id),
        blocker_ids,
    }
}

fn emitted_event_kind_ids_for_source(source_surface_id: &'static str) -> Vec<&'static str> {
    match source_surface_id {
        "update_plan_tool"
        | "plan_mode_proposed_plan_blocks"
        | "app_server_turn_plan_notification" => vec!["plan_step_event_intake"],
        "multi_agent_v2_thread_spawn" => {
            vec!["agent_spawn_event_intake", "task_result_event_intake"]
        }
        "multi_agent_v2_mailbox_wait" => vec!["mailbox_delivery_event_intake"],
        "hepta_runtime_multi_agent_reducer" => vec!["task_result_event_intake"],
        "agent_jobs_batch_workers" => {
            vec!["agent_job_item_event_intake", "task_result_event_intake"]
        }
        "hepta_runtime_task_board" => vec!["worker_task_event_intake"],
        "hepta_runtime_worker_tasks" => vec![
            "worker_task_event_intake",
            "artifact_event_intake",
            "task_result_event_intake",
        ],
        "hepta_runtime_scheduler_store" => {
            vec!["scheduler_run_event_intake", "task_result_event_intake"]
        }
        "hepta_runtime_approval_broker" => vec!["approval_event_intake"],
        "hepta_runtime_agent_harness" => {
            vec!["artifact_event_intake", "task_result_event_intake"]
        }
        _ => Vec::new(),
    }
}

fn target_collection_ids_for_events(
    event_kind_ids: &[&'static str],
    projected_collection_ids: &[&'static str],
) -> Vec<&'static str> {
    let mut collections = projected_collection_ids
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    for event_kind_id in event_kind_ids {
        for collection_id in event_contract_collections(event_kind_id) {
            collections.insert(collection_id);
        }
    }
    collections.into_iter().collect()
}

fn event_contract_collections(event_kind_id: &str) -> Vec<&'static str> {
    match event_kind_id {
        "plan_step_event_intake" => vec!["nodes", "edges", "timelineEvents"],
        "agent_spawn_event_intake" => vec!["nodes", "edges", "timelineEvents"],
        "mailbox_delivery_event_intake" => vec!["edges", "timelineEvents"],
        "agent_job_item_event_intake" => vec!["nodes", "taskResults", "timelineEvents"],
        "worker_task_event_intake" => {
            vec!["nodes", "taskResults", "artifacts", "timelineEvents"]
        }
        "scheduler_run_event_intake" => vec!["nodes", "edges", "taskResults", "timelineEvents"],
        "artifact_event_intake" => vec!["artifacts", "timelineEvents"],
        "approval_event_intake" => vec!["approvals", "timelineEvents"],
        "task_result_event_intake" => vec!["taskResults", "timelineEvents"],
        _ => Vec::new(),
    }
}

fn wal_operation_id_for_events(event_kind_ids: &[&'static str]) -> &'static str {
    if event_kind_ids.contains(&"task_result_event_intake") && event_kind_ids.len() == 1 {
        "preview_append_task_result_record"
    } else if event_kind_ids.contains(&"artifact_event_intake") && event_kind_ids.len() == 1 {
        "preview_append_artifact_record"
    } else if event_kind_ids.contains(&"approval_event_intake") {
        "preview_append_approval_record"
    } else if event_kind_ids.contains(&"mailbox_delivery_event_intake") {
        "preview_append_timeline_event_record"
    } else {
        "preview_append_node_record"
    }
}

fn idempotency_guard_id_for_source(
    source_surface_id: &'static str,
    existing_guard_present: bool,
) -> Option<&'static str> {
    if !existing_guard_present {
        return None;
    }

    match source_surface_id {
        "update_plan_tool" => Some("update_plan_projection_idempotency"),
        "multi_agent_v2_thread_spawn" => Some("multi_agent_spawn_projection_idempotency"),
        "agent_jobs_batch_workers" => Some("agent_job_result_projection_idempotency"),
        "hepta_runtime_worker_tasks" => Some("worker_task_projection_idempotency"),
        "hepta_runtime_scheduler_store" => Some("scheduler_run_projection_idempotency"),
        "hepta_runtime_approval_broker" => Some("approval_projection_idempotency"),
        "hepta_runtime_agent_harness" => Some("agent_harness_handoff_projection_idempotency"),
        _ => None,
    }
}

fn readback_probe_ids_for_collections(
    target_collection_ids: &[&'static str],
    readback_collection_ids: &BTreeSet<&'static str>,
) -> Vec<&'static str> {
    target_collection_ids
        .iter()
        .filter_map(|collection_id| {
            if readback_collection_ids.contains(collection_id) {
                readback_probe_id_for_collection(collection_id)
            } else {
                None
            }
        })
        .collect()
}

fn readback_probe_id_for_collection(collection_id: &str) -> Option<&'static str> {
    match collection_id {
        "nodes" => Some("preview_readback_nodes_by_trace"),
        "edges" => Some("preview_readback_edges_by_trace"),
        "taskResults" => Some("preview_readback_task_results_by_status"),
        "artifacts" => Some("preview_readback_artifacts_by_producer"),
        "approvals" => Some("preview_readback_approvals_by_scope"),
        "timelineEvents" => Some("preview_readback_timeline_by_trace"),
        _ => None,
    }
}

fn route_blockers(
    source: &WorkGraphUnifiedProjectionSourceAudit,
    wal_operation_id: &'static str,
    idempotency_guard_id: Option<&'static str>,
    target_collection_ids: &[&'static str],
    readback_probe_ids: &[&'static str],
    existing_wal_operation_ids: &BTreeSet<&'static str>,
) -> Vec<&'static str> {
    let mut blockers = BTreeSet::from(["append_only_store_disabled_by_design"]);
    if source.coverage_state != "contract_ready_preview" {
        blockers.insert("source_projection_not_contract_ready");
    }
    if idempotency_guard_id.is_none() {
        blockers.insert("event_intake_idempotency_guard_missing");
    }
    if !existing_wal_operation_ids.contains(wal_operation_id) {
        blockers.insert("wal_operation_contract_missing");
    }
    if readback_probe_ids.len() < target_collection_ids.len() {
        blockers.insert("readback_probe_missing_for_target_collection");
    }
    if source.requires_terminal_task_result {
        blockers.insert("terminal_task_result_enforcement_disabled");
    }
    blockers.into_iter().collect()
}

fn route_intake_state(
    source: &WorkGraphUnifiedProjectionSourceAudit,
    idempotency_guard_id: Option<&'static str>,
) -> &'static str {
    if idempotency_guard_id.is_none() {
        "blocked_until_idempotency_guard"
    } else if source.coverage_state == "contract_ready_preview" {
        "preview_intake_contract_ready"
    } else if source.coverage_state == "projection_gap" {
        "blocked_until_projection_adapter"
    } else {
        "preview_intake_with_projection_gap"
    }
}

fn event_contract(
    id: &'static str,
    record_kind: &'static str,
    source_surface_ids: Vec<&'static str>,
    target_collection_ids: Vec<&'static str>,
    required_fields: Vec<&'static str>,
    idempotency_key_fields: Vec<&'static str>,
    redaction_policy: &'static str,
    append_order: &'static str,
    requires_task_result_contract: bool,
) -> WorkGraphAppendOnlyEventContractPreview {
    WorkGraphAppendOnlyEventContractPreview {
        id,
        record_kind,
        source_surface_ids,
        target_collection_ids,
        required_fields,
        idempotency_key_fields,
        redaction_policy,
        append_order,
        requires_task_result_contract,
        mutates_store: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_event_kind_ids: Vec<&'static str>,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphAppendOnlyEventIntakeBlockerPreview {
    WorkGraphAppendOnlyEventIntakeBlockerPreview {
        id,
        severity,
        affected_event_kind_ids,
        affected_source_surface_ids,
        required_before_store_enablement: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_only_event_intake_declares_expected_event_contracts() {
        let report = hepta_work_graph_append_only_event_intake_preview_report();
        let event_ids = report
            .event_contracts
            .iter()
            .map(|event| event.id)
            .collect::<Vec<_>>();

        assert_eq!(
            event_ids,
            [
                "plan_step_event_intake",
                "agent_spawn_event_intake",
                "mailbox_delivery_event_intake",
                "agent_job_item_event_intake",
                "worker_task_event_intake",
                "scheduler_run_event_intake",
                "artifact_event_intake",
                "approval_event_intake",
                "task_result_event_intake",
            ]
        );
        assert_eq!(report.event_contract_count, 9);
        assert_eq!(report.target_collection_count, 6);
        assert!(
            report
                .event_contracts
                .iter()
                .all(|event| !event.mutates_store)
        );
    }

    #[test]
    fn append_only_event_intake_routes_all_projection_audit_surfaces() {
        let report = hepta_work_graph_append_only_event_intake_preview_report();
        let route_sources = report
            .source_routes
            .iter()
            .map(|route| route.source_surface_id)
            .collect::<Vec<_>>();
        let route_by_source = report
            .source_routes
            .iter()
            .map(|route| (route.source_surface_id, route))
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(
            route_sources,
            [
                "update_plan_tool",
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_thread_spawn",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "agent_jobs_batch_workers",
                "hepta_runtime_task_board",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_approval_broker",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.source_route_count, 12);
        assert_eq!(
            route_by_source["multi_agent_v2_mailbox_wait"].intake_state,
            "blocked_until_idempotency_guard"
        );
        assert!(
            route_by_source["hepta_runtime_task_board"]
                .blocker_ids
                .contains(&"source_projection_not_contract_ready")
        );
        assert!(
            route_by_source["hepta_runtime_worker_tasks"]
                .emitted_event_kind_ids
                .contains(&"task_result_event_intake")
        );
    }

    #[test]
    fn append_only_event_intake_requires_projection_and_persistence_priors() {
        let report = hepta_work_graph_append_only_event_intake_preview_report();

        assert_eq!(
            report.required_prior_gates,
            [
                "hepta_work_graph_contract_preview_gate",
                "hepta_work_graph_task_result_contract_preview_gate",
                "hepta_work_graph_scheduler_admission_controller_preview_gate",
                "hepta_work_graph_observability_timeline_preview_gate",
                "hepta_work_graph_role_manifest_contract_preview_gate",
                "hepta_work_graph_unified_state_store_preview_gate",
                "hepta_work_graph_adapter_projection_fixture_gate",
                "hepta_work_graph_unified_projection_audit_preview_gate",
                "hepta_work_graph_state_store_persistence_preview_gate",
            ]
        );
        assert_eq!(report.required_prior_gate_count, 9);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn append_only_event_intake_keeps_all_writes_and_enforcement_disabled() {
        let report = hepta_work_graph_append_only_event_intake_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAppendOnlyEventIntakePreviewSideEffects::none()
        );
        assert!(report.ready_for_replay_readback_preview);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_store_persistence);
        assert!(!report.ready_for_live_execution);
        assert!(report.source_routes.iter().all(|route| {
            route
                .blocker_ids
                .contains(&"append_only_store_disabled_by_design")
        }));
    }
}
