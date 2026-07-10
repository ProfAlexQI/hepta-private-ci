use serde::Serialize;

use crate::work_graph_append_only_event_store_feature_gated_wal_precondition::WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_GATE;

pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_GATE: &str =
    "hepta_work_graph_append_only_event_store_feature_gated_wal_no_write_plan_gate";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_SCHEMA_VERSION: &str =
    "work_graph_append_only_event_store_feature_gated_wal_no_write_plan_v1";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_append_only_event_store_feature_gated_wal_no_write_plan_readback_gate";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_READBACK_GATE: &str =
    "hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventStoreFeatureGatedWalNoWritePlanReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_wal_precondition_gate: &'static str,
    pub source_wal_precondition_readback_gate: &'static str,
    pub no_write_plan_step_count: usize,
    pub no_write_operation_count: usize,
    pub deterministic_identity_plan_count: usize,
    pub recovery_preview_plan_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub no_write_plan_steps: Vec<WorkGraphFeatureGatedWalNoWritePlanStep>,
    pub no_write_operations: Vec<WorkGraphFeatureGatedWalNoWriteOperation>,
    pub deterministic_identity_plans: Vec<WorkGraphFeatureGatedWalIdentityPlan>,
    pub recovery_preview_plans: Vec<WorkGraphFeatureGatedWalRecoveryPreviewPlan>,
    pub guards: Vec<WorkGraphFeatureGatedWalNoWriteGuard>,
    pub blockers: Vec<WorkGraphFeatureGatedWalNoWriteBlocker>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_append_only_event_store_feature_gated_wal_no_write_plan_readback: bool,
    pub ready_for_append_only_event_store_feature_gated_wal_replay_diff_plan: bool,
    pub ready_for_append_only_work_graph_event_store: bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphFeatureGatedWalNoWritePlanSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalNoWritePlanStep {
    pub id: &'static str,
    pub sequence: usize,
    pub scope: &'static str,
    pub source_contract_ids: Vec<&'static str>,
    pub write_allowed: bool,
    pub execution_allowed: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalNoWriteOperation {
    pub id: &'static str,
    pub category: &'static str,
    pub denied_operation: &'static str,
    pub planned_output: &'static str,
    pub no_write_guard_field: &'static str,
    pub attempted_count: usize,
    pub committed_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalIdentityPlan {
    pub id: &'static str,
    pub formula_id: &'static str,
    pub output_field: &'static str,
    pub dry_run_only: bool,
    pub materializes_index: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalRecoveryPreviewPlan {
    pub id: &'static str,
    pub scope: &'static str,
    pub source_contract_id: &'static str,
    pub executes_replay: bool,
    pub writes_checkpoint: bool,
    pub writes_rollback_anchor: bool,
    pub mutates_dead_letter_queue: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalNoWriteGuard {
    pub id: &'static str,
    pub guard_scope: &'static str,
    pub required_false_field: &'static str,
    pub currently_satisfied: bool,
    pub enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalNoWriteBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub surface: &'static str,
    pub blocks_live_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalNoWritePlanSideEffects {
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

pub fn hepta_work_graph_append_only_event_store_feature_gated_wal_no_write_plan_report()
-> WorkGraphAppendOnlyEventStoreFeatureGatedWalNoWritePlanReport {
    let no_write_plan_steps = work_graph_feature_gated_wal_no_write_plan_steps();
    let no_write_operations = work_graph_feature_gated_wal_no_write_operations();
    let deterministic_identity_plans = work_graph_feature_gated_wal_identity_plans();
    let recovery_preview_plans = work_graph_feature_gated_wal_recovery_preview_plans();
    let guards = work_graph_feature_gated_wal_no_write_guards();
    let blockers = work_graph_feature_gated_wal_no_write_blockers();
    let required_prior_gates = vec![
        WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_GATE,
        WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_READBACK_GATE,
    ];

    WorkGraphAppendOnlyEventStoreFeatureGatedWalNoWritePlanReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_SCHEMA_VERSION,
        preview_mode: "append_only_event_store_feature_gated_wal_no_write_plan_no_execution",
        source_wal_precondition_gate:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_GATE,
        source_wal_precondition_readback_gate:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_READBACK_GATE,
        no_write_plan_step_count: no_write_plan_steps.len(),
        no_write_operation_count: no_write_operations.len(),
        deterministic_identity_plan_count: deterministic_identity_plans.len(),
        recovery_preview_plan_count: recovery_preview_plans.len(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        no_write_plan_steps,
        no_write_operations,
        deterministic_identity_plans,
        recovery_preview_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_NO_WRITE_PLAN_RECOMMENDED_NEXT_GATE,
        ready_for_append_only_event_store_feature_gated_wal_no_write_plan_readback: true,
        ready_for_append_only_event_store_feature_gated_wal_replay_diff_plan: false,
        ready_for_append_only_work_graph_event_store: false,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphFeatureGatedWalNoWritePlanSideEffects::none(),
    }
}

pub fn work_graph_feature_gated_wal_no_write_plan_steps()
-> Vec<WorkGraphFeatureGatedWalNoWritePlanStep> {
    vec![
        step(
            1,
            "wal_record_batch_shape_plan",
            "wal_record",
            vec!["wal_record_schema_contract"],
        ),
        step(
            2,
            "event_id_derivation_preview_plan",
            "event_identity",
            vec!["deterministic_event_id_formula"],
        ),
        step(
            3,
            "idempotency_key_derivation_preview_plan",
            "idempotency",
            vec!["deterministic_idempotency_key_formula"],
        ),
        step(
            4,
            "idempotency_collision_check_no_mutation_plan",
            "idempotency",
            vec!["idempotency_collision_policy_contract"],
        ),
        step(
            5,
            "wal_segment_sequence_no_write_plan",
            "wal_sequence",
            vec!["wal_sequence_contract"],
        ),
        step(
            6,
            "wal_fsync_noop_plan",
            "wal_fsync",
            vec!["wal_fsync_contract", "no_write_guard_contract"],
        ),
        step(
            7,
            "event_store_append_noop_plan",
            "event_store",
            vec!["wal_record_schema_contract", "no_write_guard_contract"],
        ),
        step(
            8,
            "projection_index_rebuild_preview_plan",
            "projection",
            vec!["deterministic_event_id_formula"],
        ),
        step(
            9,
            "checkpoint_manifest_preview_plan",
            "checkpoint",
            vec!["checkpoint_manifest_contract"],
        ),
        step(
            10,
            "replay_diff_preview_plan",
            "replay",
            vec!["replay_diff_validator_contract"],
        ),
        step(
            11,
            "cancel_dead_letter_preview_plan",
            "recovery",
            vec!["cancel_token_contract", "dead_letter_contract"],
        ),
        step(
            12,
            "rollback_anchor_preview_plan",
            "rollback",
            vec!["rollback_anchor_contract", "feature_gate_contract"],
        ),
    ]
}

pub fn work_graph_feature_gated_wal_no_write_operations()
-> Vec<WorkGraphFeatureGatedWalNoWriteOperation> {
    vec![
        operation(
            "plan_wal_record_batch_shape",
            "wal",
            "write_wal_record_batch",
            "visible_wal_record_batch_shape",
            "wal_written",
        ),
        operation(
            "plan_event_id_derivation",
            "event_identity",
            "persist_event_identity",
            "deterministic_event_id_preview",
            "work_graph_event_persisted",
        ),
        operation(
            "plan_idempotency_key_derivation",
            "idempotency",
            "insert_idempotency_key",
            "deterministic_idempotency_key_preview",
            "idempotency_index_mutated",
        ),
        operation(
            "plan_idempotency_collision_check",
            "idempotency",
            "update_idempotency_collision_index",
            "collision_policy_preview",
            "idempotency_index_mutated",
        ),
        operation(
            "plan_wal_segment_sequence",
            "wal",
            "advance_wal_segment_sequence",
            "wal_sequence_preview",
            "wal_written",
        ),
        operation(
            "plan_wal_fsync_noop",
            "wal",
            "fsync_wal_segment",
            "fsync_noop_preview",
            "wal_fsynced",
        ),
        operation(
            "plan_event_store_append_noop",
            "event_store",
            "append_work_graph_event",
            "append_noop_preview",
            "event_store_enabled",
        ),
        operation(
            "plan_projection_index_rebuild",
            "projection",
            "materialize_projection_index",
            "projection_diff_preview",
            "graph_state_persisted",
        ),
        operation(
            "plan_checkpoint_manifest_preview",
            "checkpoint",
            "write_checkpoint_manifest",
            "checkpoint_manifest_preview",
            "checkpoint_written",
        ),
        operation(
            "plan_replay_diff_preview",
            "replay",
            "execute_replay_diff",
            "replay_diff_preview",
            "replay_executed",
        ),
        operation(
            "plan_cancel_dead_letter_preview",
            "recovery",
            "mutate_dead_letter_queue",
            "dead_letter_preview",
            "dead_letter_queue_mutated",
        ),
        operation(
            "plan_rollback_anchor_preview",
            "rollback",
            "write_rollback_anchor",
            "rollback_anchor_preview",
            "rollback_anchor_written",
        ),
    ]
}

pub fn work_graph_feature_gated_wal_identity_plans() -> Vec<WorkGraphFeatureGatedWalIdentityPlan> {
    vec![
        identity_plan(
            "identity_event_id_plan",
            "deterministic_event_id_formula",
            "event_id",
        ),
        identity_plan(
            "identity_idempotency_key_plan",
            "deterministic_idempotency_key_formula",
            "idempotency_key",
        ),
        identity_plan(
            "identity_checkpoint_id_plan",
            "deterministic_checkpoint_id_formula",
            "checkpoint_id",
        ),
        identity_plan(
            "identity_rollback_anchor_id_plan",
            "deterministic_rollback_anchor_id_formula",
            "rollback_anchor_id",
        ),
    ]
}

pub fn work_graph_feature_gated_wal_recovery_preview_plans()
-> Vec<WorkGraphFeatureGatedWalRecoveryPreviewPlan> {
    vec![
        recovery_plan(
            "recovery_checkpoint_manifest_preview",
            "checkpoint",
            "checkpoint_manifest_contract",
        ),
        recovery_plan(
            "recovery_replay_diff_preview",
            "replay_diff",
            "replay_diff_validator_contract",
        ),
        recovery_plan(
            "recovery_cancel_token_preview",
            "cancel",
            "cancel_token_contract",
        ),
        recovery_plan(
            "recovery_dead_letter_preview",
            "dead_letter",
            "dead_letter_contract",
        ),
        recovery_plan(
            "recovery_rollback_anchor_preview",
            "rollback",
            "rollback_anchor_contract",
        ),
    ]
}

pub fn work_graph_feature_gated_wal_no_write_guards() -> Vec<WorkGraphFeatureGatedWalNoWriteGuard> {
    vec![
        guard(
            "guard_event_store_enabled_false",
            "event_store",
            "event_store_enabled",
        ),
        guard("guard_wal_opened_false", "wal", "wal_opened"),
        guard("guard_wal_created_false", "wal", "wal_created"),
        guard("guard_wal_written_false", "wal", "wal_written"),
        guard("guard_wal_fsynced_false", "wal", "wal_fsynced"),
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
            "guard_live_execution_false",
            "live",
            "ready_for_live_execution",
        ),
    ]
}

pub fn work_graph_feature_gated_wal_no_write_blockers()
-> Vec<WorkGraphFeatureGatedWalNoWriteBlocker> {
    vec![
        blocker(
            "feature_gated_wal_no_write_plan_readback_missing",
            "high",
            "readback",
            "read back the no-write plan before replay/diff planning",
        ),
        blocker(
            "wal_writer_execution_disabled",
            "critical",
            "wal",
            "keep WAL writer execution disabled until no-write replay proves deterministic identities",
        ),
        blocker(
            "event_store_append_execution_disabled",
            "critical",
            "event_store",
            "keep append-only event-store append disabled until feature gate and rollback evidence close",
        ),
        blocker(
            "idempotency_index_materialization_disabled",
            "high",
            "idempotency",
            "do not materialize idempotency index before collision readback closes",
        ),
        blocker(
            "checkpoint_manifest_write_disabled",
            "high",
            "checkpoint",
            "keep checkpoint manifest preview-only until replay/diff readback closes",
        ),
        blocker(
            "replay_diff_execution_disabled",
            "high",
            "replay",
            "plan replay/diff without executing replay in this slice",
        ),
        blocker(
            "cancel_dead_letter_mutation_disabled",
            "medium",
            "recovery",
            "keep cancel and dead-letter queues preview-only",
        ),
        blocker(
            "rollback_anchor_write_disabled",
            "high",
            "rollback",
            "do not write rollback anchors before operator/canary rollback proof",
        ),
        blocker(
            "runtime_enforcement_disabled",
            "critical",
            "runtime_enforcement",
            "keep scheduler, TaskResult, and role admission denied until P4/P5 dry-run closes",
        ),
        blocker(
            "live_execution_blocked",
            "critical",
            "live",
            "open live execution only after P3-P6 blockers close",
        ),
    ]
}

impl WorkGraphFeatureGatedWalNoWritePlanSideEffects {
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
    source_contract_ids: Vec<&'static str>,
) -> WorkGraphFeatureGatedWalNoWritePlanStep {
    WorkGraphFeatureGatedWalNoWritePlanStep {
        id,
        sequence,
        scope,
        source_contract_ids,
        write_allowed: false,
        execution_allowed: false,
        persisted: false,
    }
}

fn operation(
    id: &'static str,
    category: &'static str,
    denied_operation: &'static str,
    planned_output: &'static str,
    no_write_guard_field: &'static str,
) -> WorkGraphFeatureGatedWalNoWriteOperation {
    WorkGraphFeatureGatedWalNoWriteOperation {
        id,
        category,
        denied_operation,
        planned_output,
        no_write_guard_field,
        attempted_count: 0,
        committed_count: 0,
    }
}

fn identity_plan(
    id: &'static str,
    formula_id: &'static str,
    output_field: &'static str,
) -> WorkGraphFeatureGatedWalIdentityPlan {
    WorkGraphFeatureGatedWalIdentityPlan {
        id,
        formula_id,
        output_field,
        dry_run_only: true,
        materializes_index: false,
    }
}

fn recovery_plan(
    id: &'static str,
    scope: &'static str,
    source_contract_id: &'static str,
) -> WorkGraphFeatureGatedWalRecoveryPreviewPlan {
    WorkGraphFeatureGatedWalRecoveryPreviewPlan {
        id,
        scope,
        source_contract_id,
        executes_replay: false,
        writes_checkpoint: false,
        writes_rollback_anchor: false,
        mutates_dead_letter_queue: false,
    }
}

fn guard(
    id: &'static str,
    guard_scope: &'static str,
    required_false_field: &'static str,
) -> WorkGraphFeatureGatedWalNoWriteGuard {
    WorkGraphFeatureGatedWalNoWriteGuard {
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
) -> WorkGraphFeatureGatedWalNoWriteBlocker {
    WorkGraphFeatureGatedWalNoWriteBlocker {
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
    fn no_write_plan_covers_expected_steps_and_operations() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_no_write_plan_report();

        assert_eq!(report.no_write_plan_step_count, 12);
        assert_eq!(report.no_write_operation_count, 12);
        assert_eq!(
            report
                .no_write_plan_steps
                .iter()
                .map(|step| step.id)
                .collect::<Vec<_>>(),
            [
                "wal_record_batch_shape_plan",
                "event_id_derivation_preview_plan",
                "idempotency_key_derivation_preview_plan",
                "idempotency_collision_check_no_mutation_plan",
                "wal_segment_sequence_no_write_plan",
                "wal_fsync_noop_plan",
                "event_store_append_noop_plan",
                "projection_index_rebuild_preview_plan",
                "checkpoint_manifest_preview_plan",
                "replay_diff_preview_plan",
                "cancel_dead_letter_preview_plan",
                "rollback_anchor_preview_plan",
            ]
        );
        assert!(
            report
                .no_write_plan_steps
                .iter()
                .all(|step| { !step.write_allowed && !step.execution_allowed && !step.persisted })
        );
        assert!(
            report
                .no_write_operations
                .iter()
                .all(|operation| operation.attempted_count == 0 && operation.committed_count == 0)
        );
    }

    #[test]
    fn deterministic_identity_plans_stay_dry_run_only() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_no_write_plan_report();

        assert_eq!(report.deterministic_identity_plan_count, 4);
        assert_eq!(
            report
                .deterministic_identity_plans
                .iter()
                .map(|plan| plan.output_field)
                .collect::<Vec<_>>(),
            [
                "event_id",
                "idempotency_key",
                "checkpoint_id",
                "rollback_anchor_id"
            ]
        );
        assert!(
            report
                .deterministic_identity_plans
                .iter()
                .all(|plan| plan.dry_run_only && !plan.materializes_index)
        );
    }

    #[test]
    fn recovery_preview_plans_do_not_execute_or_write() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_no_write_plan_report();

        assert_eq!(report.recovery_preview_plan_count, 5);
        assert!(report.recovery_preview_plans.iter().all(|plan| {
            !plan.executes_replay
                && !plan.writes_checkpoint
                && !plan.writes_rollback_anchor
                && !plan.mutates_dead_letter_queue
        }));
    }

    #[test]
    fn no_write_plan_preserves_precondition_frontier() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_no_write_plan_report();

        assert_eq!(
            report.required_prior_gates,
            [
                WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_GATE,
                WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_READBACK_GATE,
            ]
        );
        assert_eq!(report.guard_count, 11);
        assert_eq!(report.blocker_count, 10);
    }

    #[test]
    fn no_write_plan_keeps_runtime_disabled() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_no_write_plan_report();

        assert_eq!(
            report.side_effects,
            WorkGraphFeatureGatedWalNoWritePlanSideEffects::none()
        );
        assert!(report.ready_for_append_only_event_store_feature_gated_wal_no_write_plan_readback);
        assert!(!report.ready_for_append_only_event_store_feature_gated_wal_replay_diff_plan);
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
