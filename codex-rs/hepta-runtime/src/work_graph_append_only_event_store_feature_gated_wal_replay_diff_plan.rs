use serde::Serialize;

use crate::work_graph_append_only_event_store_feature_gated_wal_no_write_plan::WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_GATE;

pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_REPLAY_DIFF_PLAN_GATE: &str =
    "hepta_work_graph_append_only_event_store_feature_gated_wal_replay_diff_plan_gate";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_REPLAY_DIFF_PLAN_SCHEMA_VERSION:
    &str = "work_graph_append_only_event_store_feature_gated_wal_replay_diff_plan_v1";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_REPLAY_DIFF_PLAN_RECOMMENDED_NEXT_GATE:
    &str =
    "hepta_work_graph_append_only_event_store_feature_gated_wal_replay_diff_plan_readback_gate";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_READBACK_GATE: &str =
    "hepta_work_graph_append_only_event_store_feature_gated_wal_no_write_plan_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventStoreFeatureGatedWalReplayDiffPlanReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_wal_no_write_plan_gate: &'static str,
    pub source_wal_no_write_plan_readback_gate: &'static str,
    pub replay_diff_plan_step_count: usize,
    pub replay_diff_case_count: usize,
    pub projection_rebuild_plan_count: usize,
    pub checkpoint_preview_plan_count: usize,
    pub recovery_preview_plan_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub replay_diff_plan_steps: Vec<WorkGraphFeatureGatedWalReplayDiffPlanStep>,
    pub replay_diff_cases: Vec<WorkGraphFeatureGatedWalReplayDiffCase>,
    pub projection_rebuild_plans: Vec<WorkGraphFeatureGatedWalProjectionRebuildPlan>,
    pub checkpoint_preview_plans: Vec<WorkGraphFeatureGatedWalCheckpointPreviewPlan>,
    pub recovery_preview_plans: Vec<WorkGraphFeatureGatedWalReplayRecoveryPreviewPlan>,
    pub guards: Vec<WorkGraphFeatureGatedWalReplayDiffGuard>,
    pub blockers: Vec<WorkGraphFeatureGatedWalReplayDiffBlocker>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_append_only_event_store_feature_gated_wal_replay_diff_plan_readback: bool,
    pub ready_for_append_only_work_graph_event_store: bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphFeatureGatedWalReplayDiffPlanSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalReplayDiffPlanStep {
    pub id: &'static str,
    pub sequence: usize,
    pub scope: &'static str,
    pub source_plan_ids: Vec<&'static str>,
    pub replay_allowed: bool,
    pub diff_persistence_allowed: bool,
    pub writes_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalReplayDiffCase {
    pub id: &'static str,
    pub replay_scope: &'static str,
    pub compares: &'static str,
    pub expected_result: &'static str,
    pub source_identity_fields: Vec<&'static str>,
    pub replay_executed: bool,
    pub diff_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalProjectionRebuildPlan {
    pub id: &'static str,
    pub collection_id: &'static str,
    pub collection_kind: &'static str,
    pub source_identity_fields: Vec<&'static str>,
    pub materializes_index: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalCheckpointPreviewPlan {
    pub id: &'static str,
    pub scope: &'static str,
    pub source_identity_field: &'static str,
    pub writes_checkpoint: bool,
    pub writes_rollback_anchor: bool,
    pub publishes_manifest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalReplayRecoveryPreviewPlan {
    pub id: &'static str,
    pub scope: &'static str,
    pub source_plan_id: &'static str,
    pub executes_replay: bool,
    pub mutates_queue: bool,
    pub writes_anchor: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalReplayDiffGuard {
    pub id: &'static str,
    pub guard_scope: &'static str,
    pub required_false_field: &'static str,
    pub currently_satisfied: bool,
    pub enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalReplayDiffBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub surface: &'static str,
    pub blocks_live_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalReplayDiffPlanSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub event_store_enabled: bool,
    pub wal_opened: bool,
    pub wal_created: bool,
    pub wal_written: bool,
    pub wal_fsynced: bool,
    pub checkpoint_written: bool,
    pub rollback_anchor_written: bool,
    pub replay_executed: bool,
    pub replay_diff_persisted: bool,
    pub idempotency_index_mutated: bool,
    pub dead_letter_queue_mutated: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub operator_approval_requested: bool,
    pub feature_flag_enabled: bool,
    pub canary_started: bool,
    pub cutover_performed: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_event_store_feature_gated_wal_replay_diff_plan_report()
-> WorkGraphAppendOnlyEventStoreFeatureGatedWalReplayDiffPlanReport {
    let replay_diff_plan_steps = work_graph_feature_gated_wal_replay_diff_plan_steps();
    let replay_diff_cases = work_graph_feature_gated_wal_replay_diff_cases();
    let projection_rebuild_plans = work_graph_feature_gated_wal_projection_rebuild_plans();
    let checkpoint_preview_plans = work_graph_feature_gated_wal_checkpoint_preview_plans();
    let recovery_preview_plans = work_graph_feature_gated_wal_replay_recovery_preview_plans();
    let guards = work_graph_feature_gated_wal_replay_diff_guards();
    let blockers = work_graph_feature_gated_wal_replay_diff_blockers();
    let required_prior_gates = vec![
        WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_GATE,
        WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_READBACK_GATE,
    ];

    WorkGraphAppendOnlyEventStoreFeatureGatedWalReplayDiffPlanReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_REPLAY_DIFF_PLAN_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_REPLAY_DIFF_PLAN_SCHEMA_VERSION,
        preview_mode: "append_only_event_store_feature_gated_wal_replay_diff_plan_no_execution",
        source_wal_no_write_plan_gate:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_GATE,
        source_wal_no_write_plan_readback_gate:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_READBACK_GATE,
        replay_diff_plan_step_count: replay_diff_plan_steps.len(),
        replay_diff_case_count: replay_diff_cases.len(),
        projection_rebuild_plan_count: projection_rebuild_plans.len(),
        checkpoint_preview_plan_count: checkpoint_preview_plans.len(),
        recovery_preview_plan_count: recovery_preview_plans.len(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        replay_diff_plan_steps,
        replay_diff_cases,
        projection_rebuild_plans,
        checkpoint_preview_plans,
        recovery_preview_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_REPLAY_DIFF_PLAN_RECOMMENDED_NEXT_GATE,
        ready_for_append_only_event_store_feature_gated_wal_replay_diff_plan_readback: true,
        ready_for_append_only_work_graph_event_store: false,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphFeatureGatedWalReplayDiffPlanSideEffects::none(),
    }
}

pub fn work_graph_feature_gated_wal_replay_diff_plan_steps()
-> Vec<WorkGraphFeatureGatedWalReplayDiffPlanStep> {
    vec![
        step(
            1,
            "wal_input_set_replay_plan",
            "wal_input",
            vec!["plan_wal_record_batch_shape"],
        ),
        step(
            2,
            "event_id_replay_stability_plan",
            "event_identity",
            vec!["identity_event_id_plan"],
        ),
        step(
            3,
            "idempotency_key_replay_stability_plan",
            "idempotency",
            vec!["identity_idempotency_key_plan"],
        ),
        step(
            4,
            "wal_sequence_replay_plan",
            "wal_sequence",
            vec!["plan_wal_segment_sequence"],
        ),
        step(
            5,
            "projection_index_rebuild_diff_plan",
            "projection",
            vec!["plan_projection_index_rebuild"],
        ),
        step(
            6,
            "checkpoint_manifest_rebuild_diff_plan",
            "checkpoint",
            vec!["identity_checkpoint_id_plan"],
        ),
        step(
            7,
            "replay_diff_validator_plan",
            "replay",
            vec!["plan_replay_diff_preview"],
        ),
        step(
            8,
            "cancel_dead_letter_replay_preview_plan",
            "recovery",
            vec!["plan_cancel_dead_letter_preview"],
        ),
        step(
            9,
            "rollback_anchor_replay_preview_plan",
            "rollback",
            vec!["identity_rollback_anchor_id_plan"],
        ),
        step(
            10,
            "no_write_side_effect_boundary_plan",
            "side_effect_boundary",
            vec!["plan_event_store_append_noop"],
        ),
    ]
}

pub fn work_graph_feature_gated_wal_replay_diff_cases()
-> Vec<WorkGraphFeatureGatedWalReplayDiffCase> {
    vec![
        diff_case(
            "replay_event_id_stability",
            "event_identity",
            "event_id",
            vec!["event_id"],
        ),
        diff_case(
            "replay_idempotency_key_stability",
            "idempotency",
            "idempotency_key",
            vec!["idempotency_key"],
        ),
        diff_case(
            "replay_wal_sequence_stability",
            "wal_sequence",
            "wal_sequence",
            vec!["event_id", "idempotency_key"],
        ),
        diff_case(
            "replay_projection_index_rebuild",
            "projection",
            "projection_index",
            vec!["event_id"],
        ),
        diff_case(
            "replay_checkpoint_manifest_preview",
            "checkpoint",
            "checkpoint_manifest",
            vec!["checkpoint_id"],
        ),
        diff_case(
            "replay_no_persistence_boundary",
            "side_effect_boundary",
            "side_effects",
            vec!["event_id", "rollback_anchor_id"],
        ),
    ]
}

pub fn work_graph_feature_gated_wal_projection_rebuild_plans()
-> Vec<WorkGraphFeatureGatedWalProjectionRebuildPlan> {
    vec![
        projection("projection_rebuild_work_nodes", "work_nodes", "WorkNode"),
        projection("projection_rebuild_work_edges", "work_edges", "WorkEdge"),
        projection(
            "projection_rebuild_task_results",
            "task_results",
            "TaskResult",
        ),
        projection("projection_rebuild_leases", "leases", "Lease"),
        projection("projection_rebuild_budgets", "budgets", "Budget"),
        projection("projection_rebuild_approvals", "approvals", "Approval"),
        projection("projection_rebuild_artifacts", "artifacts", "Artifact"),
        projection("projection_rebuild_evidence", "evidence", "Evidence"),
        projection(
            "projection_rebuild_timeline_events",
            "timeline_events",
            "TimelineEvent",
        ),
    ]
}

pub fn work_graph_feature_gated_wal_checkpoint_preview_plans()
-> Vec<WorkGraphFeatureGatedWalCheckpointPreviewPlan> {
    vec![
        checkpoint(
            "checkpoint_id_rebuild_preview",
            "checkpoint_id",
            "checkpoint_id",
        ),
        checkpoint(
            "checkpoint_manifest_rebuild_preview",
            "checkpoint_manifest",
            "checkpoint_id",
        ),
        checkpoint(
            "replay_cursor_checkpoint_preview",
            "replay_cursor",
            "event_id",
        ),
        checkpoint(
            "rollback_anchor_manifest_preview",
            "rollback_anchor",
            "rollback_anchor_id",
        ),
    ]
}

pub fn work_graph_feature_gated_wal_replay_recovery_preview_plans()
-> Vec<WorkGraphFeatureGatedWalReplayRecoveryPreviewPlan> {
    vec![
        recovery(
            "recovery_checkpoint_manifest_diff_preview",
            "checkpoint",
            "recovery_checkpoint_manifest_preview",
        ),
        recovery(
            "recovery_replay_cursor_diff_preview",
            "replay",
            "recovery_replay_diff_preview",
        ),
        recovery(
            "recovery_cancel_token_diff_preview",
            "cancel",
            "recovery_cancel_token_preview",
        ),
        recovery(
            "recovery_dead_letter_diff_preview",
            "dead_letter",
            "recovery_dead_letter_preview",
        ),
        recovery(
            "recovery_rollback_anchor_diff_preview",
            "rollback",
            "recovery_rollback_anchor_preview",
        ),
    ]
}

pub fn work_graph_feature_gated_wal_replay_diff_guards()
-> Vec<WorkGraphFeatureGatedWalReplayDiffGuard> {
    vec![
        guard(
            "guard_event_store_enabled_false",
            "event_store",
            "event_store_enabled",
        ),
        guard("guard_wal_opened_false", "wal", "wal_opened"),
        guard("guard_wal_written_false", "wal", "wal_written"),
        guard(
            "guard_checkpoint_written_false",
            "checkpoint",
            "checkpoint_written",
        ),
        guard(
            "guard_rollback_anchor_written_false",
            "rollback",
            "rollback_anchor_written",
        ),
        guard("guard_replay_executed_false", "replay", "replay_executed"),
        guard(
            "guard_replay_diff_persisted_false",
            "replay",
            "replay_diff_persisted",
        ),
        guard(
            "guard_idempotency_index_mutated_false",
            "idempotency",
            "idempotency_index_mutated",
        ),
        guard(
            "guard_dead_letter_queue_mutated_false",
            "recovery",
            "dead_letter_queue_mutated",
        ),
        guard(
            "guard_scheduler_admission_enforced_false",
            "scheduler",
            "scheduler_admission_enforced",
        ),
        guard(
            "guard_task_result_enforcement_false",
            "task_result",
            "task_result_enforcement_enabled",
        ),
        guard(
            "guard_role_manifest_enforcement_false",
            "role_manifest",
            "role_manifest_enforcement_enabled",
        ),
        guard(
            "guard_live_execution_false",
            "live",
            "ready_for_live_execution",
        ),
    ]
}

pub fn work_graph_feature_gated_wal_replay_diff_blockers()
-> Vec<WorkGraphFeatureGatedWalReplayDiffBlocker> {
    vec![
        blocker(
            "feature_gated_wal_replay_diff_plan_readback_missing",
            "high",
            "readback",
            "read back replay/diff plan before closeout",
        ),
        blocker(
            "replay_executor_disabled",
            "critical",
            "replay",
            "keep replay executor disabled until no-side-effect replay gate exists",
        ),
        blocker(
            "wal_writer_disabled",
            "critical",
            "wal",
            "do not write WAL segments during replay/diff planning",
        ),
        blocker(
            "checkpoint_writer_disabled",
            "high",
            "checkpoint",
            "keep checkpoint manifests preview-only",
        ),
        blocker(
            "idempotency_index_materialization_disabled",
            "high",
            "idempotency",
            "do not persist idempotency indexes before collision closeout",
        ),
        blocker(
            "projection_materialization_disabled",
            "high",
            "projection",
            "rebuild projection indexes as visible-only previews",
        ),
        blocker(
            "replay_diff_persistence_disabled",
            "high",
            "replay",
            "do not persist replay diff output in this slice",
        ),
        blocker(
            "cancel_dead_letter_mutation_disabled",
            "medium",
            "recovery",
            "keep cancel/dead-letter mutation denied",
        ),
        blocker(
            "rollback_anchor_write_disabled",
            "high",
            "rollback",
            "do not write rollback anchors before operator/canary proof",
        ),
        blocker(
            "runtime_enforcement_disabled",
            "critical",
            "runtime_enforcement",
            "keep scheduler, TaskResult, and role admission denied",
        ),
        blocker(
            "live_execution_blocked",
            "critical",
            "live",
            "open live only after P3-P6 closeout",
        ),
    ]
}

impl WorkGraphFeatureGatedWalReplayDiffPlanSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            event_store_enabled: false,
            wal_opened: false,
            wal_created: false,
            wal_written: false,
            wal_fsynced: false,
            checkpoint_written: false,
            rollback_anchor_written: false,
            replay_executed: false,
            replay_diff_persisted: false,
            idempotency_index_mutated: false,
            dead_letter_queue_mutated: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            operator_approval_requested: false,
            feature_flag_enabled: false,
            canary_started: false,
            cutover_performed: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn step(
    sequence: usize,
    id: &'static str,
    scope: &'static str,
    source_plan_ids: Vec<&'static str>,
) -> WorkGraphFeatureGatedWalReplayDiffPlanStep {
    WorkGraphFeatureGatedWalReplayDiffPlanStep {
        id,
        sequence,
        scope,
        source_plan_ids,
        replay_allowed: false,
        diff_persistence_allowed: false,
        writes_allowed: false,
    }
}

fn diff_case(
    id: &'static str,
    replay_scope: &'static str,
    compares: &'static str,
    source_identity_fields: Vec<&'static str>,
) -> WorkGraphFeatureGatedWalReplayDiffCase {
    WorkGraphFeatureGatedWalReplayDiffCase {
        id,
        replay_scope,
        compares,
        expected_result: "no_diff_preview_only",
        source_identity_fields,
        replay_executed: false,
        diff_persisted: false,
    }
}

fn projection(
    id: &'static str,
    collection_id: &'static str,
    collection_kind: &'static str,
) -> WorkGraphFeatureGatedWalProjectionRebuildPlan {
    WorkGraphFeatureGatedWalProjectionRebuildPlan {
        id,
        collection_id,
        collection_kind,
        source_identity_fields: vec!["event_id", "idempotency_key"],
        materializes_index: false,
        persisted: false,
    }
}

fn checkpoint(
    id: &'static str,
    scope: &'static str,
    source_identity_field: &'static str,
) -> WorkGraphFeatureGatedWalCheckpointPreviewPlan {
    WorkGraphFeatureGatedWalCheckpointPreviewPlan {
        id,
        scope,
        source_identity_field,
        writes_checkpoint: false,
        writes_rollback_anchor: false,
        publishes_manifest: false,
    }
}

fn recovery(
    id: &'static str,
    scope: &'static str,
    source_plan_id: &'static str,
) -> WorkGraphFeatureGatedWalReplayRecoveryPreviewPlan {
    WorkGraphFeatureGatedWalReplayRecoveryPreviewPlan {
        id,
        scope,
        source_plan_id,
        executes_replay: false,
        mutates_queue: false,
        writes_anchor: false,
    }
}

fn guard(
    id: &'static str,
    guard_scope: &'static str,
    required_false_field: &'static str,
) -> WorkGraphFeatureGatedWalReplayDiffGuard {
    WorkGraphFeatureGatedWalReplayDiffGuard {
        id,
        guard_scope,
        required_false_field,
        currently_satisfied: true,
        enforcement_enabled: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    surface: &'static str,
    recommended_fix: &'static str,
) -> WorkGraphFeatureGatedWalReplayDiffBlocker {
    WorkGraphFeatureGatedWalReplayDiffBlocker {
        id,
        severity,
        surface,
        blocks_live_execution: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_diff_plan_covers_expected_steps_and_cases() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_replay_diff_plan_report();

        assert_eq!(report.replay_diff_plan_step_count, 10);
        assert_eq!(report.replay_diff_case_count, 6);
        assert_eq!(
            report
                .replay_diff_plan_steps
                .iter()
                .map(|step| step.id)
                .collect::<Vec<_>>(),
            [
                "wal_input_set_replay_plan",
                "event_id_replay_stability_plan",
                "idempotency_key_replay_stability_plan",
                "wal_sequence_replay_plan",
                "projection_index_rebuild_diff_plan",
                "checkpoint_manifest_rebuild_diff_plan",
                "replay_diff_validator_plan",
                "cancel_dead_letter_replay_preview_plan",
                "rollback_anchor_replay_preview_plan",
                "no_write_side_effect_boundary_plan",
            ]
        );
        assert!(report.replay_diff_plan_steps.iter().all(|step| {
            !step.replay_allowed && !step.diff_persistence_allowed && !step.writes_allowed
        }));
    }

    #[test]
    fn replay_diff_cases_stay_preview_only() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_replay_diff_plan_report();

        assert_eq!(
            report
                .replay_diff_cases
                .iter()
                .map(|case| case.expected_result)
                .collect::<Vec<_>>(),
            [
                "no_diff_preview_only",
                "no_diff_preview_only",
                "no_diff_preview_only",
                "no_diff_preview_only",
                "no_diff_preview_only",
                "no_diff_preview_only",
            ]
        );
        assert!(
            report
                .replay_diff_cases
                .iter()
                .all(|case| !case.replay_executed && !case.diff_persisted)
        );
    }

    #[test]
    fn projection_checkpoint_and_recovery_plans_do_not_materialize() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_replay_diff_plan_report();

        assert_eq!(report.projection_rebuild_plan_count, 9);
        assert_eq!(report.checkpoint_preview_plan_count, 4);
        assert_eq!(report.recovery_preview_plan_count, 5);
        assert!(
            report
                .projection_rebuild_plans
                .iter()
                .all(|plan| !plan.materializes_index && !plan.persisted)
        );
        assert!(report.checkpoint_preview_plans.iter().all(|plan| {
            !plan.writes_checkpoint && !plan.writes_rollback_anchor && !plan.publishes_manifest
        }));
        assert!(
            report
                .recovery_preview_plans
                .iter()
                .all(|plan| !plan.executes_replay && !plan.mutates_queue && !plan.writes_anchor)
        );
    }

    #[test]
    fn replay_diff_plan_preserves_no_write_frontier() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_replay_diff_plan_report();

        assert_eq!(
            report.required_prior_gates,
            [
                WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_GATE,
                WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_READBACK_GATE,
            ]
        );
        assert_eq!(report.guard_count, 13);
        assert_eq!(report.blocker_count, 11);
    }

    #[test]
    fn replay_diff_plan_keeps_runtime_disabled() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_replay_diff_plan_report();

        assert_eq!(
            report.side_effects,
            WorkGraphFeatureGatedWalReplayDiffPlanSideEffects::none()
        );
        assert!(
            report.ready_for_append_only_event_store_feature_gated_wal_replay_diff_plan_readback
        );
        assert!(!report.ready_for_append_only_work_graph_event_store);
        assert!(!report.ready_for_wal_write);
        assert!(!report.ready_for_checkpoint_write);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .blockers
                .iter()
                .all(|blocker| blocker.blocks_live_execution)
        );
    }
}
