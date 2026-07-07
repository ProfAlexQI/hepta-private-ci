use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback::WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackSideEffects;
use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback::hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_report;
use crate::work_graph_append_only_event_store_shadow_path::WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE;
use crate::work_graph_append_only_event_store_shadow_path::WorkGraphAppendOnlyEventStoreShadowPathSideEffects;
use crate::work_graph_append_only_event_store_shadow_path::hepta_work_graph_append_only_event_store_shadow_path_report;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_shadow_event_store_readback_gate: &'static str,
    pub source_readback_entry_count: usize,
    pub source_shadow_event_join_count: usize,
    pub source_non_persistence_blocker_count: usize,
    pub source_shadow_event_store_readback_ready: bool,
    pub source_shadow_event_store_readback_no_execution_confirmed: bool,
    pub source_append_only_shadow_path_gate: &'static str,
    pub source_shadow_path_replay_diff_count: usize,
    pub source_shadow_path_readiness_complete: bool,
    pub source_shadow_path_no_persistence_confirmed: bool,
    pub replay_diff_plan_count: usize,
    pub replay_scope_count: usize,
    pub non_execution_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub replay_diff_plans: Vec<WorkGraphShadowEventStoreReplayDiffDryRunPlanPreview>,
    pub replay_scopes: Vec<WorkGraphShadowEventStoreReplayDiffScopePreview>,
    pub non_execution_blockers: Vec<WorkGraphShadowEventStoreReplayDiffDryRunBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub source_prior_readbacks_complete: bool,
    pub replay_diff_plans_dry_run_complete: bool,
    pub replay_scopes_dry_run_complete: bool,
    pub non_execution_blockers_complete: bool,
    pub deterministic_replay_plan_ready: bool,
    pub projection_diff_plan_ready: bool,
    pub duplicate_suppression_diff_ready: bool,
    pub redaction_hash_diff_ready: bool,
    pub task_result_canary_diff_ready: bool,
    pub replay_execution_enabled: bool,
    pub replay_diff_recording_enabled: bool,
    pub replay_diff_persistence_enabled: bool,
    pub rollback_execution_enabled: bool,
    pub shadow_event_persistence_enabled: bool,
    pub projection_index_persistence_enabled: bool,
    pub scheduler_guardrail_live_enforcement_enabled: bool,
    pub runtime_interception_enabled: bool,
    pub ready_for_non_execution_readback: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReplayDiffDryRunPlanPreview {
    pub id: &'static str,
    pub replay_scope_ref: &'static str,
    pub diff_kind: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub expected_result: &'static str,
    pub dry_run_ready: bool,
    pub replay_executed: bool,
    pub diff_recorded: bool,
    pub diff_persisted: bool,
    pub live_enforced: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReplayDiffScopePreview {
    pub id: &'static str,
    pub entrypoint_id: &'static str,
    pub source_surface_id: &'static str,
    pub trace_id: &'static str,
    pub shadow_event_ref: &'static str,
    pub projection_index_ref: &'static str,
    pub replay_diff_ref: &'static str,
    pub deterministic_order: &'static str,
    pub dry_run_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReplayDiffDryRunBlockerPreview {
    pub id: &'static str,
    pub blocks: &'static str,
    pub reason: &'static str,
    pub required_before_execution: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub shadow_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub readback_executed: bool,
    pub readback_recorded: bool,
    pub replay_executed: bool,
    pub replay_diff_recorded: bool,
    pub replay_diff_persisted: bool,
    pub rollback_executed: bool,
    pub idempotency_index_mutated: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_report()
-> WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunReport {
    let readback =
        hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_report();
    let shadow_path = hepta_work_graph_append_only_event_store_shadow_path_report();
    let replay_diff_plans =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_plans();
    let replay_scopes =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_scopes();
    let non_execution_blockers =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_blockers(
        );
    let required_prior_gates =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_required_prior_gates();
    let source_shadow_event_store_readback_no_execution_confirmed = !readback
        .shadow_readback_executed
        && !readback.shadow_event_persistence_enabled
        && !readback.projection_index_persistence_enabled
        && !readback.scheduler_guardrail_live_enforcement_enabled
        && !readback.runtime_interception_enabled
        && !readback.ready_for_live_execution
        && readback.side_effects
            == WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackSideEffects::none();
    let source_shadow_event_store_readback_ready = readback.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE
        && readback.source_prior_readbacks_complete
        && readback.shadow_event_store_readback_ready
        && readback.ready_for_replay_diff_dry_run
        && readback.readback_entry_count == 6
        && readback.shadow_event_join_count == 4
        && readback.non_persistence_blocker_count == 14
        && source_shadow_event_store_readback_no_execution_confirmed;
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
        && shadow_path.replay_diff_ready
        && shadow_path.replay_diff_count == 4
        && source_shadow_path_no_persistence_confirmed;
    let source_prior_readbacks_complete =
        source_shadow_event_store_readback_ready && source_shadow_path_readiness_complete;
    let replay_diff_plans_dry_run_complete = replay_diff_plans.len() == 6
        && replay_diff_plans.iter().all(|plan| {
            plan.dry_run_ready
                && !plan.replay_executed
                && !plan.diff_recorded
                && !plan.diff_persisted
                && !plan.live_enforced
                && !plan.compared_fields.is_empty()
        });
    let replay_scopes_dry_run_complete = replay_scopes.len() == 4
        && replay_scopes.iter().all(|scope| {
            scope.dry_run_only
                && scope.trace_id.starts_with("trace-blocking-dry-run-")
                && scope.shadow_event_ref.starts_with("wg-event-shadow-")
                && scope.projection_index_ref.starts_with("projection_by_")
                && scope.replay_diff_ref.starts_with("shadow_replay_")
        });
    let non_execution_blockers_complete = non_execution_blockers.len() == 16
        && non_execution_blockers
            .iter()
            .all(|blocker| blocker.required_before_execution);
    let deterministic_replay_plan_ready = source_prior_readbacks_complete
        && replay_diff_plans_dry_run_complete
        && replay_scopes_dry_run_complete;
    let projection_diff_plan_ready = deterministic_replay_plan_ready
        && replay_diff_plans
            .iter()
            .any(|plan| plan.diff_kind == "projection_index_rebuild");
    let duplicate_suppression_diff_ready = deterministic_replay_plan_ready
        && replay_diff_plans
            .iter()
            .any(|plan| plan.diff_kind == "duplicate_event_suppression");
    let redaction_hash_diff_ready = deterministic_replay_plan_ready
        && replay_diff_plans
            .iter()
            .any(|plan| plan.diff_kind == "redaction_hash_stability");
    let task_result_canary_diff_ready = deterministic_replay_plan_ready
        && replay_diff_plans
            .iter()
            .any(|plan| plan.diff_kind == "task_result_report_only_join");
    let ready_for_non_execution_readback = deterministic_replay_plan_ready
        && projection_diff_plan_ready
        && duplicate_suppression_diff_ready
        && redaction_hash_diff_ready
        && task_result_canary_diff_ready
        && non_execution_blockers_complete;

    WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_SCHEMA_VERSION,
        preview_mode: "work_graph_shadow_event_store_replay_diff_dry_run_no_execute_no_persist_no_live",
        source_shadow_event_store_readback_gate: readback.gate,
        source_readback_entry_count: readback.readback_entry_count,
        source_shadow_event_join_count: readback.shadow_event_join_count,
        source_non_persistence_blocker_count: readback.non_persistence_blocker_count,
        source_shadow_event_store_readback_ready,
        source_shadow_event_store_readback_no_execution_confirmed,
        source_append_only_shadow_path_gate: shadow_path.gate,
        source_shadow_path_replay_diff_count: shadow_path.replay_diff_count,
        source_shadow_path_readiness_complete,
        source_shadow_path_no_persistence_confirmed,
        replay_diff_plan_count: replay_diff_plans.len(),
        replay_scope_count: replay_scopes.len(),
        non_execution_blocker_count: non_execution_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        replay_diff_plans,
        replay_scopes,
        non_execution_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_RECOMMENDED_NEXT_GATE,
        source_prior_readbacks_complete,
        replay_diff_plans_dry_run_complete,
        replay_scopes_dry_run_complete,
        non_execution_blockers_complete,
        deterministic_replay_plan_ready,
        projection_diff_plan_ready,
        duplicate_suppression_diff_ready,
        redaction_hash_diff_ready,
        task_result_canary_diff_ready,
        replay_execution_enabled: false,
        replay_diff_recording_enabled: false,
        replay_diff_persistence_enabled: false,
        rollback_execution_enabled: false,
        shadow_event_persistence_enabled: false,
        projection_index_persistence_enabled: false,
        scheduler_guardrail_live_enforcement_enabled: false,
        runtime_interception_enabled: false,
        ready_for_non_execution_readback,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_plans()
-> Vec<WorkGraphShadowEventStoreReplayDiffDryRunPlanPreview> {
    vec![
        replay_diff_plan(
            "entrypoint_shadow_join_noop_projection_diff",
            "all_entrypoint_shadow_event_joins",
            "noop_projection_diff",
            vec!["eventId", "projectionIndexKey", "payloadHash"],
            "no_diff_preview_only",
        ),
        replay_diff_plan(
            "redacted_payload_hash_stability_diff",
            "all_entrypoint_shadow_event_joins",
            "redaction_hash_stability",
            vec!["redactedPayloadRef", "payloadHash", "evidenceRef"],
            "hash_stable_preview_only",
        ),
        replay_diff_plan(
            "projection_index_rebuild_dry_run_diff",
            "projection_index_rebuild",
            "projection_index_rebuild",
            vec!["collectionId", "keyFields", "eventKindRefs"],
            "index_rebuild_matches_preview_only",
        ),
        replay_diff_plan(
            "scheduler_admission_duplicate_suppression_diff",
            "scheduler_admission_idempotency_window",
            "duplicate_event_suppression",
            vec!["deterministicEventId", "traceId", "payloadHash"],
            "duplicate_suppressed_preview_only",
        ),
        replay_diff_plan(
            "canary_report_only_task_result_diff",
            "agent_jobs_task_board_canary_report_only",
            "task_result_report_only_join",
            vec!["workGraphReportOnly", "taskId", "traceId"],
            "canary_task_result_shape_matches_preview_only",
        ),
        replay_diff_plan(
            "shadow_event_store_non_persistence_boundary_diff",
            "shadow_event_store_boundary",
            "non_persistence_boundary",
            vec!["shadowPersisted", "eventStoreEnabled", "liveCutoverEnabled"],
            "all_persistence_and_live_flags_remain_false",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_scopes()
-> Vec<WorkGraphShadowEventStoreReplayDiffScopePreview> {
    vec![
        replay_scope(
            "spawn_agent_replay_diff_scope",
            "spawn_agent",
            "multi_agent_v2_thread_spawn",
            "trace-blocking-dry-run-spawn-agent-001",
            "wg-event-shadow-spawn-001",
            "projection_by_trace_id",
            "shadow_replay_noop_projection_diff",
        ),
        replay_scope(
            "spawn_agents_on_csv_replay_diff_scope",
            "spawn_agents_on_csv",
            "agent_jobs_batch_workers",
            "trace-blocking-dry-run-agent-jobs-001",
            "wg-event-shadow-agent-job-result-001",
            "projection_by_task_id",
            "shadow_replay_duplicate_event_suppression_diff",
        ),
        replay_scope(
            "task_board_claim_replay_diff_scope",
            "task_board_claim",
            "hepta_runtime_task_board",
            "trace-blocking-dry-run-task-board-001",
            "wg-event-shadow-task-board-terminal-001",
            "projection_by_task_id",
            "shadow_replay_projection_index_rebuild_diff",
        ),
        replay_scope(
            "worker_task_run_replay_diff_scope",
            "worker_task_run",
            "hepta_runtime_worker_tasks",
            "trace-blocking-dry-run-worker-task-001",
            "wg-event-shadow-worker-artifact-001",
            "projection_by_source_surface",
            "shadow_replay_redaction_hash_stability_diff",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_blockers()
-> Vec<WorkGraphShadowEventStoreReplayDiffDryRunBlockerPreview> {
    vec![
        blocker("replay_execution_blocked", "replay_execution"),
        blocker("replay_diff_recording_blocked", "replay_diff_recording"),
        blocker("replay_diff_persistence_blocked", "replay_diff_persistence"),
        blocker(
            "projection_rebuild_execution_blocked",
            "projection_rebuild_execution",
        ),
        blocker("idempotency_mutation_blocked", "idempotency_mutation"),
        blocker("readback_execution_blocked", "readback_execution"),
        blocker(
            "work_graph_event_persistence_blocked",
            "work_graph_event_persistence",
        ),
        blocker(
            "projection_index_persistence_blocked",
            "projection_index_persistence",
        ),
        blocker(
            "scheduler_guardrail_live_enforcement_blocked",
            "scheduler_guardrail_live_enforcement",
        ),
        blocker("runtime_interception_blocked", "runtime_interception"),
        blocker("feature_flag_enablement_blocked", "feature_flag_enablement"),
        blocker("canary_traffic_blocked", "canary_traffic"),
        blocker("operator_review_request_blocked", "operator_review_request"),
        blocker("approval_recording_blocked", "approval_recording"),
        blocker("rollback_execution_blocked", "rollback_execution"),
        blocker("live_cutover_blocked", "live_cutover"),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE,
        WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            shadow_event_persisted: false,
            projection_index_persisted: false,
            readback_executed: false,
            readback_recorded: false,
            replay_executed: false,
            replay_diff_recorded: false,
            replay_diff_persisted: false,
            rollback_executed: false,
            idempotency_index_mutated: false,
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

fn replay_diff_plan(
    id: &'static str,
    replay_scope_ref: &'static str,
    diff_kind: &'static str,
    compared_fields: Vec<&'static str>,
    expected_result: &'static str,
) -> WorkGraphShadowEventStoreReplayDiffDryRunPlanPreview {
    WorkGraphShadowEventStoreReplayDiffDryRunPlanPreview {
        id,
        replay_scope_ref,
        diff_kind,
        compared_fields,
        expected_result,
        dry_run_ready: true,
        replay_executed: false,
        diff_recorded: false,
        diff_persisted: false,
        live_enforced: false,
    }
}

fn replay_scope(
    id: &'static str,
    entrypoint_id: &'static str,
    source_surface_id: &'static str,
    trace_id: &'static str,
    shadow_event_ref: &'static str,
    projection_index_ref: &'static str,
    replay_diff_ref: &'static str,
) -> WorkGraphShadowEventStoreReplayDiffScopePreview {
    WorkGraphShadowEventStoreReplayDiffScopePreview {
        id,
        entrypoint_id,
        source_surface_id,
        trace_id,
        shadow_event_ref,
        projection_index_ref,
        replay_diff_ref,
        deterministic_order: "traceId:eventId:sequenceKey",
        dry_run_only: true,
    }
}

fn blocker(
    id: &'static str,
    blocks: &'static str,
) -> WorkGraphShadowEventStoreReplayDiffDryRunBlockerPreview {
    WorkGraphShadowEventStoreReplayDiffDryRunBlockerPreview {
        id,
        blocks,
        reason: "required before replay/diff dry-run can execute, persist, enforce, or cut live",
        required_before_execution: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_diff_dry_run_derives_from_shadow_readback_and_shadow_path() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_report();

        assert_eq!(
            report.source_shadow_event_store_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE
        );
        assert_eq!(
            report.source_append_only_shadow_path_gate,
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE
        );
        assert_eq!(report.source_readback_entry_count, 6);
        assert_eq!(report.source_shadow_event_join_count, 4);
        assert_eq!(report.source_non_persistence_blocker_count, 14);
        assert!(report.source_shadow_event_store_readback_ready);
        assert!(report.source_shadow_event_store_readback_no_execution_confirmed);
        assert_eq!(report.source_shadow_path_replay_diff_count, 4);
        assert!(report.source_shadow_path_readiness_complete);
        assert!(report.source_shadow_path_no_persistence_confirmed);
        assert!(report.source_prior_readbacks_complete);
    }

    #[test]
    fn replay_diff_dry_run_declares_plans_and_scopes() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_report();
        let entrypoint_ids = report
            .replay_scopes
            .iter()
            .map(|scope| scope.entrypoint_id)
            .collect::<Vec<_>>();

        assert_eq!(report.replay_diff_plan_count, 6);
        assert_eq!(report.replay_scope_count, 4);
        assert_eq!(
            entrypoint_ids,
            vec![
                "spawn_agent",
                "spawn_agents_on_csv",
                "task_board_claim",
                "worker_task_run"
            ]
        );
        assert!(
            report
                .replay_diff_plans
                .iter()
                .all(|plan| plan.dry_run_ready
                    && !plan.replay_executed
                    && !plan.diff_recorded
                    && !plan.diff_persisted
                    && !plan.live_enforced)
        );
        assert!(report.replay_diff_plans_dry_run_complete);
        assert!(report.replay_scopes_dry_run_complete);
    }

    #[test]
    fn replay_diff_dry_run_stays_non_executing_and_non_live() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_report();

        assert_eq!(report.non_execution_blocker_count, 16);
        assert!(report.non_execution_blockers_complete);
        assert!(report.deterministic_replay_plan_ready);
        assert!(report.ready_for_non_execution_readback);
        assert!(!report.replay_execution_enabled);
        assert!(!report.replay_diff_recording_enabled);
        assert!(!report.replay_diff_persistence_enabled);
        assert!(!report.rollback_execution_enabled);
        assert!(!report.shadow_event_persistence_enabled);
        assert!(!report.scheduler_guardrail_live_enforcement_enabled);
        assert!(!report.runtime_interception_enabled);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .non_execution_blockers
                .iter()
                .all(|blocker| blocker.required_before_execution)
        );
    }

    #[test]
    fn replay_diff_dry_run_links_priors_and_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE,
                WORK_GRAPH_APPEND_ONLY_EVENT_STORE_SHADOW_PATH_GATE,
            ]
        );
        assert_eq!(report.required_prior_gate_count, 2);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_RECOMMENDED_NEXT_GATE
        );
        assert!(!report.side_effects.filesystem_written);
        assert!(!report.side_effects.work_graph_event_persisted);
        assert!(!report.side_effects.replay_executed);
        assert!(!report.side_effects.replay_diff_recorded);
        assert!(!report.side_effects.rollback_executed);
        assert!(!report.side_effects.guardrail_enforcement_enabled);
    }
}
