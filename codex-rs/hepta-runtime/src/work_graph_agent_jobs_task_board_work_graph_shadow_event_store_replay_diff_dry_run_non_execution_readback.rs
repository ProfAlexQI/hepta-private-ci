use serde::Serialize;

use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE;
use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback::WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackSideEffects;
use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback::hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_report;
use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run::WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_GATE;
use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run::WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunSideEffects;
use crate::work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run::hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_report;

pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_SCHEMA_VERSION:
    &str = "work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_v1";
pub const WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_replay_diff_dry_run_gate: &'static str,
    pub source_replay_diff_plan_count: usize,
    pub source_replay_scope_count: usize,
    pub source_non_execution_blocker_count: usize,
    pub source_replay_diff_dry_run_ready: bool,
    pub source_replay_diff_dry_run_no_execution_confirmed: bool,
    pub source_shadow_event_store_readback_gate: &'static str,
    pub source_shadow_readback_entry_count: usize,
    pub source_shadow_event_join_count: usize,
    pub source_shadow_event_store_readback_ready: bool,
    pub source_shadow_event_store_readback_no_execution_confirmed: bool,
    pub non_execution_readback_entry_count: usize,
    pub replay_scope_readback_count: usize,
    pub non_execution_blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub non_execution_readback_entries:
        Vec<WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackEntryPreview>,
    pub replay_scope_readbacks:
        Vec<WorkGraphShadowEventStoreReplayDiffDryRunScopeReadbackPreview>,
    pub non_execution_blockers:
        Vec<WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub source_prior_readbacks_complete: bool,
    pub non_execution_readback_entries_visible_only_complete: bool,
    pub replay_scope_readbacks_visible_only_complete: bool,
    pub non_execution_blockers_complete: bool,
    pub dry_run_non_execution_readback_ready: bool,
    pub replay_diff_plan_readback_ready: bool,
    pub replay_scope_readback_ready: bool,
    pub side_effect_boundary_readback_ready: bool,
    pub replay_execution_confirmed_absent: bool,
    pub replay_diff_recording_confirmed_absent: bool,
    pub replay_diff_persistence_confirmed_absent: bool,
    pub rollback_execution_confirmed_absent: bool,
    pub idempotency_mutation_confirmed_absent: bool,
    pub readback_execution_enabled: bool,
    pub readback_recording_enabled: bool,
    pub readback_persistence_enabled: bool,
    pub replay_execution_enabled: bool,
    pub replay_diff_persistence_enabled: bool,
    pub shadow_event_persistence_enabled: bool,
    pub scheduler_guardrail_live_enforcement_enabled: bool,
    pub runtime_interception_enabled: bool,
    pub ready_for_audit_index: bool,
    pub ready_for_live_execution: bool,
    pub side_effects:
        WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackEntryPreview {
    pub id: &'static str,
    pub source_plan_id: &'static str,
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
pub struct WorkGraphShadowEventStoreReplayDiffDryRunScopeReadbackPreview {
    pub id: &'static str,
    pub entrypoint_id: &'static str,
    pub replay_scope_ref: &'static str,
    pub trace_id: &'static str,
    pub shadow_event_ref: &'static str,
    pub readback_status: &'static str,
    pub dry_run_only: bool,
    pub replay_executed: bool,
    pub diff_recorded: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackBlockerPreview {
    pub id: &'static str,
    pub blocks: &'static str,
    pub reason: &'static str,
    pub required_before_audit_acceptance: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub shadow_event_persisted: bool,
    pub projection_index_persisted: bool,
    pub readback_executed: bool,
    pub readback_recorded: bool,
    pub readback_persisted: bool,
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

pub fn hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_report()
-> WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackReport {
    let replay_diff =
        hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_report();
    let shadow_readback =
        hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_report();
    let non_execution_readback_entries =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_entries();
    let replay_scope_readbacks =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_scope_readbacks();
    let non_execution_blockers =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_blockers();
    let required_prior_gates =
        work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_required_prior_gates();
    let source_replay_diff_dry_run_no_execution_confirmed =
        !replay_diff.replay_execution_enabled
            && !replay_diff.replay_diff_recording_enabled
            && !replay_diff.replay_diff_persistence_enabled
            && !replay_diff.rollback_execution_enabled
            && !replay_diff.shadow_event_persistence_enabled
            && !replay_diff.projection_index_persistence_enabled
            && !replay_diff.scheduler_guardrail_live_enforcement_enabled
            && !replay_diff.runtime_interception_enabled
            && !replay_diff.ready_for_live_execution
            && replay_diff.side_effects
                == WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunSideEffects::none();
    let source_replay_diff_dry_run_ready = replay_diff.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_GATE
        && replay_diff.source_prior_readbacks_complete
        && replay_diff.ready_for_non_execution_readback
        && replay_diff.replay_diff_plan_count == 6
        && replay_diff.replay_scope_count == 4
        && replay_diff.non_execution_blocker_count == 16
        && source_replay_diff_dry_run_no_execution_confirmed;
    let source_shadow_event_store_readback_no_execution_confirmed = !shadow_readback
        .shadow_readback_executed
        && !shadow_readback.shadow_event_persistence_enabled
        && !shadow_readback.projection_index_persistence_enabled
        && !shadow_readback.scheduler_guardrail_live_enforcement_enabled
        && !shadow_readback.runtime_interception_enabled
        && !shadow_readback.ready_for_live_execution
        && shadow_readback.side_effects
            == WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReadbackSideEffects::none();
    let source_shadow_event_store_readback_ready = shadow_readback.gate
        == WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE
        && shadow_readback.source_prior_readbacks_complete
        && shadow_readback.shadow_event_store_readback_ready
        && shadow_readback.ready_for_replay_diff_dry_run
        && shadow_readback.readback_entry_count == 6
        && shadow_readback.shadow_event_join_count == 4
        && source_shadow_event_store_readback_no_execution_confirmed;
    let source_prior_readbacks_complete =
        source_replay_diff_dry_run_ready && source_shadow_event_store_readback_ready;
    let non_execution_readback_entries_visible_only_complete = non_execution_readback_entries.len()
        == 7
        && non_execution_readback_entries.iter().all(|entry| {
            entry.visible
                && !entry.executed
                && !entry.recorded
                && !entry.persisted
                && !entry.authoritative
                && !entry.required_fields.is_empty()
        });
    let replay_scope_readbacks_visible_only_complete = replay_scope_readbacks.len() == 4
        && replay_scope_readbacks.iter().all(|scope| {
            scope.readback_status == "scope_readback_ready_not_executed"
                && scope.dry_run_only
                && !scope.replay_executed
                && !scope.diff_recorded
                && !scope.persisted
                && scope.trace_id.starts_with("trace-blocking-dry-run-")
                && scope.shadow_event_ref.starts_with("wg-event-shadow-")
        });
    let non_execution_blockers_complete = non_execution_blockers.len() == 18
        && non_execution_blockers
            .iter()
            .all(|blocker| blocker.required_before_audit_acceptance);
    let replay_diff_plan_readback_ready =
        source_replay_diff_dry_run_ready && non_execution_readback_entries_visible_only_complete;
    let replay_scope_readback_ready =
        source_replay_diff_dry_run_ready && replay_scope_readbacks_visible_only_complete;
    let side_effect_boundary_readback_ready = source_prior_readbacks_complete
        && non_execution_blockers_complete
        && !replay_diff.side_effects.replay_executed
        && !replay_diff.side_effects.replay_diff_recorded
        && !replay_diff.side_effects.replay_diff_persisted
        && !replay_diff.side_effects.rollback_executed
        && !replay_diff.side_effects.idempotency_index_mutated;
    let dry_run_non_execution_readback_ready = source_prior_readbacks_complete
        && replay_diff_plan_readback_ready
        && replay_scope_readback_ready
        && side_effect_boundary_readback_ready;

    WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_GATE,
        schema_version:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_SCHEMA_VERSION,
        preview_mode:
            "work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_no_execute_no_persist_no_live",
        source_replay_diff_dry_run_gate: replay_diff.gate,
        source_replay_diff_plan_count: replay_diff.replay_diff_plan_count,
        source_replay_scope_count: replay_diff.replay_scope_count,
        source_non_execution_blocker_count: replay_diff.non_execution_blocker_count,
        source_replay_diff_dry_run_ready,
        source_replay_diff_dry_run_no_execution_confirmed,
        source_shadow_event_store_readback_gate: shadow_readback.gate,
        source_shadow_readback_entry_count: shadow_readback.readback_entry_count,
        source_shadow_event_join_count: shadow_readback.shadow_event_join_count,
        source_shadow_event_store_readback_ready,
        source_shadow_event_store_readback_no_execution_confirmed,
        non_execution_readback_entry_count: non_execution_readback_entries.len(),
        replay_scope_readback_count: replay_scope_readbacks.len(),
        non_execution_blocker_count: non_execution_blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        non_execution_readback_entries,
        replay_scope_readbacks,
        non_execution_blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_RECOMMENDED_NEXT_GATE,
        source_prior_readbacks_complete,
        non_execution_readback_entries_visible_only_complete,
        replay_scope_readbacks_visible_only_complete,
        non_execution_blockers_complete,
        dry_run_non_execution_readback_ready,
        replay_diff_plan_readback_ready,
        replay_scope_readback_ready,
        side_effect_boundary_readback_ready,
        replay_execution_confirmed_absent: source_replay_diff_dry_run_no_execution_confirmed,
        replay_diff_recording_confirmed_absent: !replay_diff.replay_diff_recording_enabled,
        replay_diff_persistence_confirmed_absent: !replay_diff.replay_diff_persistence_enabled,
        rollback_execution_confirmed_absent: !replay_diff.rollback_execution_enabled,
        idempotency_mutation_confirmed_absent: !replay_diff.side_effects.idempotency_index_mutated,
        readback_execution_enabled: false,
        readback_recording_enabled: false,
        readback_persistence_enabled: false,
        replay_execution_enabled: false,
        replay_diff_persistence_enabled: false,
        shadow_event_persistence_enabled: false,
        scheduler_guardrail_live_enforcement_enabled: false,
        runtime_interception_enabled: false,
        ready_for_audit_index: dry_run_non_execution_readback_ready,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackSideEffects::none(),
    }
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_entries()
-> Vec<WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackEntryPreview> {
    vec![
        non_execution_readback_entry(
            "replay_diff_plan_inventory_non_execution_readback",
            "entrypoint_shadow_join_noop_projection_diff",
            "replay_diff_plan_inventory",
            vec![
                "replayDiffPlanId",
                "dryRunReady",
                "replayExecuted",
                "diffPersisted",
            ],
            "evidence:replay-diff-plan-inventory-non-execution-readback",
        ),
        non_execution_readback_entry(
            "replay_scope_inventory_non_execution_readback",
            "spawn_agent_replay_diff_scope",
            "replay_scope_inventory",
            vec!["entrypointId", "traceId", "shadowEventRef", "dryRunOnly"],
            "evidence:replay-scope-inventory-non-execution-readback",
        ),
        non_execution_readback_entry(
            "projection_diff_non_execution_readback",
            "projection_index_rebuild_dry_run_diff",
            "projection_diff_no_execution",
            vec!["projectionIndexRef", "expectedResult", "replayExecuted"],
            "evidence:projection-diff-non-execution-readback",
        ),
        non_execution_readback_entry(
            "redacted_payload_hash_non_execution_readback",
            "redacted_payload_hash_stability_diff",
            "redacted_payload_hash_no_execution",
            vec!["redactedPayloadRef", "payloadHash", "diffRecorded"],
            "evidence:redacted-payload-hash-non-execution-readback",
        ),
        non_execution_readback_entry(
            "canary_task_result_shape_non_execution_readback",
            "canary_report_only_task_result_diff",
            "canary_task_result_shape_no_execution",
            vec!["workGraphReportOnly", "taskId", "diffPersisted"],
            "evidence:canary-task-result-shape-non-execution-readback",
        ),
        non_execution_readback_entry(
            "idempotency_duplicate_suppression_non_execution_readback",
            "scheduler_admission_duplicate_suppression_diff",
            "idempotency_duplicate_suppression_no_mutation",
            vec!["deterministicEventId", "traceId", "idempotencyMutated"],
            "evidence:idempotency-duplicate-suppression-non-execution-readback",
        ),
        non_execution_readback_entry(
            "non_persistence_boundary_non_execution_readback",
            "shadow_event_store_non_persistence_boundary_diff",
            "non_persistence_boundary_no_live",
            vec!["shadowPersisted", "eventStoreEnabled", "liveCutoverEnabled"],
            "evidence:non-persistence-boundary-non-execution-readback",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_scope_readbacks()
-> Vec<WorkGraphShadowEventStoreReplayDiffDryRunScopeReadbackPreview> {
    vec![
        scope_readback(
            "spawn_agent_replay_scope_non_execution_readback",
            "spawn_agent",
            "spawn_agent_replay_diff_scope",
            "trace-blocking-dry-run-spawn-agent-001",
            "wg-event-shadow-spawn-001",
        ),
        scope_readback(
            "spawn_agents_on_csv_replay_scope_non_execution_readback",
            "spawn_agents_on_csv",
            "spawn_agents_on_csv_replay_diff_scope",
            "trace-blocking-dry-run-agent-jobs-001",
            "wg-event-shadow-agent-job-result-001",
        ),
        scope_readback(
            "task_board_claim_replay_scope_non_execution_readback",
            "task_board_claim",
            "task_board_claim_replay_diff_scope",
            "trace-blocking-dry-run-task-board-001",
            "wg-event-shadow-task-board-terminal-001",
        ),
        scope_readback(
            "worker_task_run_replay_scope_non_execution_readback",
            "worker_task_run",
            "worker_task_run_replay_diff_scope",
            "trace-blocking-dry-run-worker-task-001",
            "wg-event-shadow-worker-artifact-001",
        ),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_blockers()
-> Vec<WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackBlockerPreview> {
    vec![
        blocker("readback_execution_blocked", "readback_execution"),
        blocker("readback_recording_blocked", "readback_recording"),
        blocker("readback_persistence_blocked", "readback_persistence"),
        blocker("replay_execution_blocked", "replay_execution"),
        blocker("replay_diff_recording_blocked", "replay_diff_recording"),
        blocker("replay_diff_persistence_blocked", "replay_diff_persistence"),
        blocker("rollback_execution_blocked", "rollback_execution"),
        blocker("idempotency_mutation_blocked", "idempotency_mutation"),
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
        blocker("audit_index_acceptance_blocked", "audit_index_acceptance"),
        blocker("live_cutover_blocked", "live_cutover"),
    ]
}

pub fn work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_required_prior_gates()
-> Vec<&'static str> {
    vec![
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_GATE,
        WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE,
    ]
}

impl WorkGraphAgentJobsTaskBoardWorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackSideEffects {
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

fn non_execution_readback_entry(
    id: &'static str,
    source_plan_id: &'static str,
    readback_target: &'static str,
    required_fields: Vec<&'static str>,
    evidence_ref: &'static str,
) -> WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackEntryPreview {
    WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackEntryPreview {
        id,
        source_plan_id,
        readback_target,
        required_fields,
        evidence_ref,
        status: "non_execution_readback_ready_not_executed",
        visible: true,
        executed: false,
        recorded: false,
        persisted: false,
        authoritative: false,
    }
}

fn scope_readback(
    id: &'static str,
    entrypoint_id: &'static str,
    replay_scope_ref: &'static str,
    trace_id: &'static str,
    shadow_event_ref: &'static str,
) -> WorkGraphShadowEventStoreReplayDiffDryRunScopeReadbackPreview {
    WorkGraphShadowEventStoreReplayDiffDryRunScopeReadbackPreview {
        id,
        entrypoint_id,
        replay_scope_ref,
        trace_id,
        shadow_event_ref,
        readback_status: "scope_readback_ready_not_executed",
        dry_run_only: true,
        replay_executed: false,
        diff_recorded: false,
        persisted: false,
    }
}

fn blocker(
    id: &'static str,
    blocks: &'static str,
) -> WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackBlockerPreview {
    WorkGraphShadowEventStoreReplayDiffDryRunNonExecutionReadbackBlockerPreview {
        id,
        blocks,
        reason: "required before non-execution readback can be recorded, accepted, enforced, or cut live",
        required_before_audit_acceptance: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_execution_readback_derives_from_replay_diff_dry_run() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_report();

        assert_eq!(
            report.source_replay_diff_dry_run_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_GATE
        );
        assert_eq!(
            report.source_shadow_event_store_readback_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE
        );
        assert_eq!(report.source_replay_diff_plan_count, 6);
        assert_eq!(report.source_replay_scope_count, 4);
        assert_eq!(report.source_non_execution_blocker_count, 16);
        assert!(report.source_replay_diff_dry_run_ready);
        assert!(report.source_replay_diff_dry_run_no_execution_confirmed);
        assert_eq!(report.source_shadow_readback_entry_count, 6);
        assert_eq!(report.source_shadow_event_join_count, 4);
        assert!(report.source_shadow_event_store_readback_ready);
        assert!(report.source_shadow_event_store_readback_no_execution_confirmed);
        assert!(report.source_prior_readbacks_complete);
    }

    #[test]
    fn non_execution_readback_declares_entries_and_scope_readbacks() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_report();
        let entrypoint_ids = report
            .replay_scope_readbacks
            .iter()
            .map(|scope| scope.entrypoint_id)
            .collect::<Vec<_>>();

        assert_eq!(report.non_execution_readback_entry_count, 7);
        assert_eq!(report.replay_scope_readback_count, 4);
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
                .non_execution_readback_entries
                .iter()
                .all(|entry| entry.visible
                    && !entry.executed
                    && !entry.recorded
                    && !entry.persisted
                    && !entry.authoritative)
        );
        assert!(report.non_execution_readback_entries_visible_only_complete);
        assert!(report.replay_scope_readbacks_visible_only_complete);
    }

    #[test]
    fn non_execution_readback_keeps_execution_persistence_and_live_disabled() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_report();

        assert_eq!(report.non_execution_blocker_count, 18);
        assert!(report.non_execution_blockers_complete);
        assert!(report.dry_run_non_execution_readback_ready);
        assert!(report.replay_execution_confirmed_absent);
        assert!(report.replay_diff_recording_confirmed_absent);
        assert!(report.replay_diff_persistence_confirmed_absent);
        assert!(report.rollback_execution_confirmed_absent);
        assert!(report.idempotency_mutation_confirmed_absent);
        assert!(!report.readback_execution_enabled);
        assert!(!report.replay_execution_enabled);
        assert!(!report.replay_diff_persistence_enabled);
        assert!(!report.shadow_event_persistence_enabled);
        assert!(!report.scheduler_guardrail_live_enforcement_enabled);
        assert!(!report.runtime_interception_enabled);
        assert!(report.ready_for_audit_index);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn non_execution_readback_links_priors_and_side_effects() {
        let report =
            hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_report();

        assert_eq!(
            report.required_prior_gates,
            vec![
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_GATE,
                WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_READBACK_GATE,
            ]
        );
        assert_eq!(report.required_prior_gate_count, 2);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_AGENT_JOBS_TASK_BOARD_WORK_GRAPH_SHADOW_EVENT_STORE_REPLAY_DIFF_DRY_RUN_NON_EXECUTION_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(!report.side_effects.filesystem_written);
        assert!(!report.side_effects.readback_executed);
        assert!(!report.side_effects.replay_executed);
        assert!(!report.side_effects.replay_diff_recorded);
        assert!(!report.side_effects.idempotency_index_mutated);
        assert!(!report.side_effects.guardrail_enforcement_enabled);
    }
}
