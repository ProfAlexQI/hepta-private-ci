use serde::Serialize;

use crate::work_graph_canonical_schema_fixture_report_generation::WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_GATE;

pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_GATE: &str =
    "hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_gate";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_SCHEMA_VERSION: &str =
    "work_graph_append_only_event_store_feature_gated_wal_precondition_v1";
pub const WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_readback_gate";
pub const WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_READBACK_GATE: &str =
    "hepta_work_graph_canonical_schema_fixture_report_generation_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyEventStoreFeatureGatedWalPreconditionReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_schema_fixture_generation_gate: &'static str,
    pub source_schema_fixture_generation_readback_gate: &'static str,
    pub wal_precondition_count: usize,
    pub wal_contract_count: usize,
    pub deterministic_formula_count: usize,
    pub recovery_contract_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub wal_preconditions: Vec<WorkGraphFeatureGatedWalPrecondition>,
    pub wal_contracts: Vec<WorkGraphFeatureGatedWalContract>,
    pub deterministic_formulas: Vec<WorkGraphFeatureGatedWalDeterministicFormula>,
    pub recovery_contracts: Vec<WorkGraphFeatureGatedWalRecoveryContract>,
    pub guards: Vec<WorkGraphFeatureGatedWalGuard>,
    pub blockers: Vec<WorkGraphFeatureGatedWalBlocker>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_append_only_event_store_feature_gated_wal_precondition_readback: bool,
    pub ready_for_append_only_event_store_feature_gated_wal_no_write_plan: bool,
    pub ready_for_append_only_work_graph_event_store: bool,
    pub ready_for_wal_write: bool,
    pub ready_for_checkpoint_write: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphFeatureGatedWalPreconditionSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalPrecondition {
    pub id: &'static str,
    pub scope: &'static str,
    pub required_contract_ids: Vec<&'static str>,
    pub required_before_feature_gate_enablement: bool,
    pub currently_satisfied: bool,
    pub runtime_enabled_after_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalContract {
    pub id: &'static str,
    pub scope: &'static str,
    pub required_fixture_collections: Vec<&'static str>,
    pub required_wire_fields: Vec<&'static str>,
    pub write_allowed: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalDeterministicFormula {
    pub id: &'static str,
    pub output_field: &'static str,
    pub input_fields: Vec<&'static str>,
    pub stable_hash_algorithm: &'static str,
    pub materializes_index: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalRecoveryContract {
    pub id: &'static str,
    pub scope: &'static str,
    pub required_fields: Vec<&'static str>,
    pub executes_replay: bool,
    pub writes_checkpoint: bool,
    pub writes_rollback_anchor: bool,
    pub mutates_dead_letter_queue: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalGuard {
    pub id: &'static str,
    pub guard_scope: &'static str,
    pub required_false_field: &'static str,
    pub currently_satisfied: bool,
    pub enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub surface: &'static str,
    pub blocks_live_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphFeatureGatedWalPreconditionSideEffects {
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

pub fn hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_report()
-> WorkGraphAppendOnlyEventStoreFeatureGatedWalPreconditionReport {
    let wal_preconditions = work_graph_feature_gated_wal_preconditions();
    let wal_contracts = work_graph_feature_gated_wal_contracts();
    let deterministic_formulas = work_graph_feature_gated_wal_deterministic_formulas();
    let recovery_contracts = work_graph_feature_gated_wal_recovery_contracts();
    let guards = work_graph_feature_gated_wal_guards();
    let blockers = work_graph_feature_gated_wal_blockers();
    let required_prior_gates = vec![
        WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_GATE,
        WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_READBACK_GATE,
    ];

    WorkGraphAppendOnlyEventStoreFeatureGatedWalPreconditionReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_SCHEMA_VERSION,
        preview_mode: "append_only_event_store_feature_gated_wal_precondition_no_write",
        source_schema_fixture_generation_gate:
            WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_GATE,
        source_schema_fixture_generation_readback_gate:
            WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_READBACK_GATE,
        wal_precondition_count: wal_preconditions.len(),
        wal_contract_count: wal_contracts.len(),
        deterministic_formula_count: deterministic_formulas.len(),
        recovery_contract_count: recovery_contracts.len(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        wal_preconditions,
        wal_contracts,
        deterministic_formulas,
        recovery_contracts,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_RECOMMENDED_NEXT_GATE,
        ready_for_append_only_event_store_feature_gated_wal_precondition_readback: true,
        ready_for_append_only_event_store_feature_gated_wal_no_write_plan: false,
        ready_for_append_only_work_graph_event_store: false,
        ready_for_wal_write: false,
        ready_for_checkpoint_write: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphFeatureGatedWalPreconditionSideEffects::none(),
    }
}

pub fn work_graph_feature_gated_wal_preconditions() -> Vec<WorkGraphFeatureGatedWalPrecondition> {
    vec![
        precondition(
            "feature_gate_default_off",
            "feature_gate",
            vec!["feature_gate_contract"],
        ),
        precondition(
            "wal_record_schema_bound_to_canonical_collections",
            "wal_schema",
            vec!["wal_record_schema_contract"],
        ),
        precondition(
            "deterministic_event_id_formula_bound",
            "event_identity",
            vec!["deterministic_event_id_formula"],
        ),
        precondition(
            "idempotency_key_formula_bound",
            "idempotency",
            vec!["deterministic_idempotency_key_formula"],
        ),
        precondition(
            "idempotency_collision_policy_bound",
            "idempotency",
            vec!["idempotency_collision_policy_contract"],
        ),
        precondition(
            "wal_sequence_and_fsync_contract_bound",
            "wal_sequence",
            vec!["wal_sequence_contract", "wal_fsync_contract"],
        ),
        precondition(
            "checkpoint_manifest_contract_bound",
            "checkpoint",
            vec!["checkpoint_manifest_contract"],
        ),
        precondition(
            "replay_diff_validator_contract_bound",
            "replay",
            vec!["replay_diff_validator_contract"],
        ),
        precondition(
            "cancel_dead_letter_contract_bound",
            "recovery",
            vec!["cancel_token_contract", "dead_letter_contract"],
        ),
        precondition(
            "rollback_anchor_contract_bound",
            "rollback",
            vec!["rollback_anchor_contract", "no_write_guard_contract"],
        ),
    ]
}

pub fn work_graph_feature_gated_wal_contracts() -> Vec<WorkGraphFeatureGatedWalContract> {
    vec![
        wal_contract(
            "wal_record_schema_contract",
            "wal_record",
            vec!["work_nodes", "work_edges", "task_results", "evidence"],
            vec![
                "event_id",
                "event_kind",
                "collection_name",
                "source_report_hash",
                "payload_hash",
                "trace_id",
            ],
        ),
        wal_contract(
            "wal_sequence_contract",
            "wal_sequence",
            vec!["timeline_events"],
            vec![
                "wal_id",
                "segment_id",
                "sequence_number",
                "previous_event_id",
            ],
        ),
        wal_contract(
            "wal_fsync_contract",
            "wal_fsync",
            vec!["timeline_events"],
            vec!["wal_id", "segment_id", "fsync_policy", "no_write_guard"],
        ),
        wal_contract(
            "feature_gate_contract",
            "feature_gate",
            vec!["approvals"],
            vec![
                "feature_flag",
                "operator_approval_id",
                "canary_id",
                "rollback_anchor_id",
            ],
        ),
        wal_contract(
            "idempotency_collision_policy_contract",
            "idempotency_collision",
            vec!["task_results", "evidence"],
            vec![
                "idempotency_key",
                "payload_hash",
                "collision_policy",
                "collision_state",
            ],
        ),
        wal_contract(
            "no_write_guard_contract",
            "no_write_guard",
            vec!["evidence", "timeline_events"],
            vec![
                "event_store_enabled",
                "wal_written",
                "checkpoint_written",
                "live_execution",
            ],
        ),
    ]
}

pub fn work_graph_feature_gated_wal_deterministic_formulas()
-> Vec<WorkGraphFeatureGatedWalDeterministicFormula> {
    vec![
        formula(
            "deterministic_event_id_formula",
            "event_id",
            vec![
                "collection_name",
                "identity_field",
                "source_report_hash",
                "payload_hash",
                "sequence_number",
            ],
        ),
        formula(
            "deterministic_idempotency_key_formula",
            "idempotency_key",
            vec![
                "work_node_id",
                "gate_id",
                "event_kind",
                "source_report_hash",
                "payload_hash",
            ],
        ),
        formula(
            "deterministic_checkpoint_id_formula",
            "checkpoint_id",
            vec!["wal_id", "segment_id", "last_event_id", "projection_hash"],
        ),
        formula(
            "deterministic_rollback_anchor_id_formula",
            "rollback_anchor_id",
            vec![
                "checkpoint_id",
                "feature_flag",
                "canary_id",
                "operator_approval_id",
            ],
        ),
    ]
}

pub fn work_graph_feature_gated_wal_recovery_contracts()
-> Vec<WorkGraphFeatureGatedWalRecoveryContract> {
    vec![
        recovery_contract(
            "checkpoint_manifest_contract",
            "checkpoint",
            vec![
                "checkpoint_id",
                "last_event_id",
                "projection_hash",
                "created_at_unix_seconds",
            ],
        ),
        recovery_contract(
            "replay_diff_validator_contract",
            "replay_diff",
            vec![
                "source_event_count",
                "rebuilt_projection_hash",
                "expected_projection_hash",
                "diff_status",
            ],
        ),
        recovery_contract(
            "cancel_token_contract",
            "cancel",
            vec![
                "work_node_id",
                "cancel_requested_at",
                "cancel_reason",
                "trace_id",
            ],
        ),
        recovery_contract(
            "dead_letter_contract",
            "dead_letter",
            vec![
                "event_id",
                "failure_kind",
                "retry_count",
                "dead_letter_reason",
            ],
        ),
        recovery_contract(
            "rollback_anchor_contract",
            "rollback",
            vec![
                "rollback_anchor_id",
                "checkpoint_id",
                "pre_cutover_feature_flag",
                "restore_plan_hash",
            ],
        ),
    ]
}

pub fn work_graph_feature_gated_wal_guards() -> Vec<WorkGraphFeatureGatedWalGuard> {
    vec![
        guard(
            "guard_event_store_enabled_false",
            "event_store",
            "event_store_enabled",
        ),
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
        guard("guard_replay_executed_false", "replay", "replay_executed"),
        guard(
            "guard_feature_flag_enabled_false",
            "release",
            "feature_flag_enabled",
        ),
        guard(
            "guard_live_execution_false",
            "live",
            "ready_for_live_execution",
        ),
    ]
}

pub fn work_graph_feature_gated_wal_blockers() -> Vec<WorkGraphFeatureGatedWalBlocker> {
    vec![
        blocker(
            "feature_gated_wal_precondition_readback_missing",
            "high",
            "readback",
            "read back this precondition catalog before planning WAL no-write execution",
        ),
        blocker(
            "wal_record_writer_not_implemented",
            "high",
            "wal",
            "implement a no-write WAL plan before adding any writer path",
        ),
        blocker(
            "idempotency_index_not_materialized",
            "high",
            "idempotency",
            "keep idempotency formula preview-only until replay diff and collision policy close",
        ),
        blocker(
            "checkpoint_manifest_not_materialized",
            "high",
            "checkpoint",
            "keep checkpoint manifests unwritten until recovery readback closes",
        ),
        blocker(
            "replay_diff_validator_not_executed",
            "high",
            "replay",
            "prove replay diff determinism before opening append-only event-store persistence",
        ),
        blocker(
            "cancel_dead_letter_queue_not_materialized",
            "medium",
            "recovery",
            "define cancel and dead-letter readbacks before live scheduler integration",
        ),
        blocker(
            "role_and_task_enforcement_not_fixture_backed",
            "critical",
            "runtime_enforcement",
            "keep scheduler, TaskResult, and role admission denied until fixture-backed dry-run closes",
        ),
        blocker(
            "operator_feature_canary_cutover_missing",
            "critical",
            "release",
            "require operator approval, feature flag, canary, and rollback proof before cutover",
        ),
        blocker(
            "live_execution_blocked",
            "critical",
            "live",
            "open live execution only after P3-P6 blockers close",
        ),
    ]
}

impl WorkGraphFeatureGatedWalPreconditionSideEffects {
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

fn precondition(
    id: &'static str,
    scope: &'static str,
    required_contract_ids: Vec<&'static str>,
) -> WorkGraphFeatureGatedWalPrecondition {
    WorkGraphFeatureGatedWalPrecondition {
        id,
        scope,
        required_contract_ids,
        required_before_feature_gate_enablement: true,
        currently_satisfied: true,
        runtime_enabled_after_preview: false,
    }
}

fn wal_contract(
    id: &'static str,
    scope: &'static str,
    required_fixture_collections: Vec<&'static str>,
    required_wire_fields: Vec<&'static str>,
) -> WorkGraphFeatureGatedWalContract {
    WorkGraphFeatureGatedWalContract {
        id,
        scope,
        required_fixture_collections,
        required_wire_fields,
        write_allowed: false,
        persisted: false,
    }
}

fn formula(
    id: &'static str,
    output_field: &'static str,
    input_fields: Vec<&'static str>,
) -> WorkGraphFeatureGatedWalDeterministicFormula {
    WorkGraphFeatureGatedWalDeterministicFormula {
        id,
        output_field,
        input_fields,
        stable_hash_algorithm: "sha256",
        materializes_index: false,
    }
}

fn recovery_contract(
    id: &'static str,
    scope: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphFeatureGatedWalRecoveryContract {
    WorkGraphFeatureGatedWalRecoveryContract {
        id,
        scope,
        required_fields,
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
) -> WorkGraphFeatureGatedWalGuard {
    WorkGraphFeatureGatedWalGuard {
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
) -> WorkGraphFeatureGatedWalBlocker {
    WorkGraphFeatureGatedWalBlocker {
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
    fn feature_gated_wal_preconditions_cover_p3_contracts() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_report();
        let precondition_ids = report
            .wal_preconditions
            .iter()
            .map(|precondition| precondition.id)
            .collect::<Vec<_>>();

        assert_eq!(
            precondition_ids,
            [
                "feature_gate_default_off",
                "wal_record_schema_bound_to_canonical_collections",
                "deterministic_event_id_formula_bound",
                "idempotency_key_formula_bound",
                "idempotency_collision_policy_bound",
                "wal_sequence_and_fsync_contract_bound",
                "checkpoint_manifest_contract_bound",
                "replay_diff_validator_contract_bound",
                "cancel_dead_letter_contract_bound",
                "rollback_anchor_contract_bound",
            ]
        );
        assert_eq!(report.wal_precondition_count, 10);
        assert_eq!(report.wal_contract_count, 6);
        assert_eq!(report.deterministic_formula_count, 4);
        assert_eq!(report.recovery_contract_count, 5);
    }

    #[test]
    fn deterministic_formulas_stay_preview_only() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_report();

        assert_eq!(
            report
                .deterministic_formulas
                .iter()
                .map(|formula| formula.output_field)
                .collect::<Vec<_>>(),
            [
                "event_id",
                "idempotency_key",
                "checkpoint_id",
                "rollback_anchor_id",
            ]
        );
        assert!(
            report
                .deterministic_formulas
                .iter()
                .all(|formula| formula.stable_hash_algorithm == "sha256"
                    && !formula.materializes_index)
        );
    }

    #[test]
    fn feature_gated_wal_precondition_preserves_source_frontier() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_report();

        assert_eq!(
            report.required_prior_gates,
            [
                WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_GATE,
                WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_READBACK_GATE,
            ]
        );
        assert_eq!(report.guard_count, 9);
        assert_eq!(report.blocker_count, 9);
    }

    #[test]
    fn feature_gated_wal_precondition_contract_refs_are_cataloged() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_report();
        let contract_ids = report
            .wal_contracts
            .iter()
            .map(|contract| contract.id)
            .chain(
                report
                    .deterministic_formulas
                    .iter()
                    .map(|formula| formula.id),
            )
            .chain(report.recovery_contracts.iter().map(|contract| contract.id))
            .collect::<Vec<_>>();

        assert!(report.wal_preconditions.iter().all(|precondition| {
            precondition
                .required_contract_ids
                .iter()
                .all(|required_contract_id| contract_ids.contains(required_contract_id))
        }));
    }

    #[test]
    fn feature_gated_wal_precondition_keeps_runtime_disabled() {
        let report =
            hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_report();

        assert_eq!(
            report.side_effects,
            WorkGraphFeatureGatedWalPreconditionSideEffects::none()
        );
        assert!(report.ready_for_append_only_event_store_feature_gated_wal_precondition_readback);
        assert!(!report.ready_for_append_only_event_store_feature_gated_wal_no_write_plan);
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
