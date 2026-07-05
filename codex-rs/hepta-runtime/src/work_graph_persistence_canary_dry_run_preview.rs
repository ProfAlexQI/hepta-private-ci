use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_CANARY_DRY_RUN_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_canary_dry_run_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_CANARY_DRY_RUN_SCHEMA_VERSION: &str =
    "work_graph_persistence_canary_dry_run_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_CANARY_DRY_RUN_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_canary_readback_receipt_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryDryRunPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub lane_guard_count: usize,
    pub dry_run_scenario_count: usize,
    pub traffic_guard_count: usize,
    pub write_guard_count: usize,
    pub rollback_receipt_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub lane_guards: Vec<WorkGraphPersistenceCanaryLaneGuardPreview>,
    pub dry_run_scenarios: Vec<WorkGraphPersistenceCanaryDryRunScenarioPreview>,
    pub traffic_guards: Vec<WorkGraphPersistenceCanaryTrafficGuardPreview>,
    pub write_guards: Vec<WorkGraphPersistenceCanaryWriteGuardPreview>,
    pub rollback_receipts: Vec<WorkGraphPersistenceCanaryRollbackReceiptPreview>,
    pub durable_identity_evidence: WorkGraphPersistenceCanaryDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceCanaryDryRunInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_canary_readback_receipt_preview: bool,
    pub ready_for_canary_execution: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceCanaryDryRunPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryLaneGuardPreview {
    pub id: &'static str,
    pub lane_id: &'static str,
    pub required_env: &'static str,
    pub scope: &'static str,
    pub blocks_cross_lane_execution: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryDryRunScenarioPreview {
    pub id: &'static str,
    pub source_feature_flag_id: &'static str,
    pub input_fixture_id: &'static str,
    pub expected_evidence_ids: Vec<&'static str>,
    pub max_runtime_ms: u64,
    pub traffic_ppm: u32,
    pub writes_allowed: bool,
    pub promotion_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryTrafficGuardPreview {
    pub id: &'static str,
    pub applies_to_stage_ids: Vec<&'static str>,
    pub max_traffic_ppm: u32,
    pub blocks_live_traffic: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryWriteGuardPreview {
    pub id: &'static str,
    pub target_collection_id: &'static str,
    pub allowed_write_mode: &'static str,
    pub blocks_live_writes: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryRollbackReceiptPreview {
    pub id: &'static str,
    pub trigger_guard_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub persistence_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_dry_run_scenario_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryDryRunInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryDryRunPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub feature_flag_mutated: bool,
    pub canary_executed: bool,
    pub live_traffic_routed: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub receipt_persisted: bool,
    pub rollback_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub approval_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_canary_dry_run_preview_report()
-> WorkGraphPersistenceCanaryDryRunPreviewReport {
    let lane_guards = work_graph_persistence_canary_lane_guards();
    let dry_run_scenarios = work_graph_persistence_canary_dry_run_scenarios();
    let traffic_guards = work_graph_persistence_canary_traffic_guards();
    let write_guards = work_graph_persistence_canary_write_guards();
    let rollback_receipts = work_graph_persistence_canary_rollback_receipts();
    let durable_identity_evidence = work_graph_persistence_canary_durable_identity_evidence();
    let invariants = work_graph_persistence_canary_dry_run_invariants();

    WorkGraphPersistenceCanaryDryRunPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_CANARY_DRY_RUN_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_CANARY_DRY_RUN_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_canary_dry_run_preview_no_canary_execution",
        lane_guard_count: lane_guards.len(),
        dry_run_scenario_count: dry_run_scenarios.len(),
        traffic_guard_count: traffic_guards.len(),
        write_guard_count: write_guards.len(),
        rollback_receipt_count: rollback_receipts.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_persistence_canary_dry_run_required_prior_gates(),
        lane_guards,
        dry_run_scenarios,
        traffic_guards,
        write_guards,
        rollback_receipts,
        durable_identity_evidence,
        invariants,
        recommended_next_gate: WORK_GRAPH_PERSISTENCE_CANARY_DRY_RUN_RECOMMENDED_NEXT_GATE,
        ready_for_canary_readback_receipt_preview: true,
        ready_for_canary_execution: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceCanaryDryRunPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_canary_dry_run_required_prior_gates() -> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
        "hepta_work_graph_state_store_persistence_preview_gate",
        "hepta_work_graph_replay_readback_preview_gate",
        "hepta_work_graph_promotion_precondition_preview_gate",
        "hepta_work_graph_activation_enforcement_blocker_preview_gate",
        "hepta_work_graph_shadow_adapter_readback_preview_gate",
        "hepta_work_graph_persistence_feature_flag_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_canary_dry_run_scenario_ids() -> Vec<&'static str> {
    vec![
        "canary_store_persistence_dry_run",
        "canary_wal_append_dry_run",
        "canary_checkpoint_write_dry_run",
        "canary_readback_receipt_dry_run",
        "canary_idempotency_index_dry_run",
        "canary_replay_execution_dry_run",
    ]
}

pub fn work_graph_persistence_canary_durable_identity_field_ids() -> Vec<&'static str> {
    vec![
        "workflow_id",
        "run_id",
        "step_id",
        "checkpoint",
        "replay_key",
        "rollback_anchor",
        "receipt_hash",
    ]
}

pub fn work_graph_persistence_canary_lane_guards() -> Vec<WorkGraphPersistenceCanaryLaneGuardPreview>
{
    vec![
        lane_guard(
            "hepta_backend_lane_lock_required",
            "hepta-backend",
            "HEPTA_LANE=hepta-backend",
            "backend_preview_canary_only",
        ),
        lane_guard(
            "cargo_target_dir_isolated",
            "hepta-backend",
            "CARGO_TARGET_DIR=/Users/qianqi/.openclaw/tmp/cargo-targets/hepta-backend",
            "build_artifact_isolation",
        ),
        lane_guard(
            "no_cross_lane_runtime_write",
            "hepta-backend",
            "HEPTA_AGENT_ID=hepta-backend",
            "runtime_state_write_blocked",
        ),
        lane_guard(
            "no_external_delivery_lane",
            "hepta-backend",
            "OPENCLAW_DELIVERY=disabled",
            "external_side_effect_blocked",
        ),
        lane_guard(
            "no_operator_approval_recording_lane",
            "hepta-backend",
            "HEPTA_APPROVAL_RECORDING=preview-only",
            "operator_receipt_write_blocked",
        ),
    ]
}

pub fn work_graph_persistence_canary_dry_run_scenarios()
-> Vec<WorkGraphPersistenceCanaryDryRunScenarioPreview> {
    vec![
        dry_run_scenario(
            "canary_store_persistence_dry_run",
            "work_graph_store_persistence_flag",
            "fixture_graph_collections_snapshot",
            vec![
                "durable_identity_evidence_packet",
                "prior_gate_digest",
                "shadow_readback_digest",
                "zero_live_write_evidence",
            ],
        ),
        dry_run_scenario(
            "canary_wal_append_dry_run",
            "work_graph_wal_append_flag",
            "fixture_wal_record_batch",
            vec![
                "durable_identity_evidence_packet",
                "wal_schema_digest",
                "idempotency_guard_digest",
                "zero_wal_write_evidence",
            ],
        ),
        dry_run_scenario(
            "canary_checkpoint_write_dry_run",
            "work_graph_checkpoint_write_flag",
            "fixture_checkpoint_hash_plan",
            vec![
                "durable_identity_evidence_packet",
                "checkpoint_hash_plan",
                "disk_budget_packet",
                "zero_checkpoint_write_evidence",
            ],
        ),
        dry_run_scenario(
            "canary_readback_receipt_dry_run",
            "work_graph_readback_receipt_persistence_flag",
            "fixture_readback_receipt",
            vec![
                "durable_identity_evidence_packet",
                "redaction_packet",
                "shadow_readback_digest",
                "zero_receipt_persistence_evidence",
            ],
        ),
        dry_run_scenario(
            "canary_idempotency_index_dry_run",
            "work_graph_idempotency_index_write_flag",
            "fixture_idempotency_index",
            vec![
                "durable_identity_evidence_packet",
                "idempotency_guard_digest",
                "collision_policy_evidence",
                "zero_index_write_evidence",
            ],
        ),
        dry_run_scenario(
            "canary_replay_execution_dry_run",
            "work_graph_replay_execution_feature_flag",
            "fixture_replay_stage_plan",
            vec![
                "durable_identity_evidence_packet",
                "drift_budget_packet",
                "rollback_plan",
                "zero_replay_execution_evidence",
            ],
        ),
    ]
}

pub fn work_graph_persistence_canary_traffic_guards()
-> Vec<WorkGraphPersistenceCanaryTrafficGuardPreview> {
    vec![
        traffic_guard("disabled_stage_traffic_guard", vec!["disabled"]),
        traffic_guard("local_dry_run_traffic_guard", vec!["local_dry_run"]),
        traffic_guard(
            "shadow_write_fixture_traffic_guard",
            vec!["shadow_write_fixture_only"],
        ),
        traffic_guard(
            "shadow_readback_compare_traffic_guard",
            vec!["shadow_readback_compare"],
        ),
        traffic_guard(
            "canary_lane_dry_run_traffic_guard",
            vec!["canary_lane_dry_run"],
        ),
    ]
}

pub fn work_graph_persistence_canary_write_guards()
-> Vec<WorkGraphPersistenceCanaryWriteGuardPreview> {
    vec![
        write_guard("nodes_no_live_write", "nodes"),
        write_guard("edges_no_live_write", "edges"),
        write_guard("task_results_no_live_write", "taskResults"),
        write_guard("artifacts_no_live_write", "artifacts"),
        write_guard("approvals_no_live_write", "approvals"),
        write_guard("timeline_events_no_live_write", "timelineEvents"),
    ]
}

pub fn work_graph_persistence_canary_rollback_receipts()
-> Vec<WorkGraphPersistenceCanaryRollbackReceiptPreview> {
    vec![
        rollback_receipt("operator_kill_switch_receipt", "operator_kill_switch"),
        rollback_receipt("wal_checksum_mismatch_receipt", "wal_checksum_mismatch"),
        rollback_receipt("shadow_readback_drift_receipt", "shadow_readback_drift"),
        rollback_receipt("idempotency_collision_receipt", "idempotency_collision"),
        rollback_receipt("disk_budget_exceeded_receipt", "disk_budget_exceeded"),
        rollback_receipt(
            "operator_approval_expired_receipt",
            "operator_approval_expired",
        ),
    ]
}

pub fn work_graph_persistence_canary_durable_identity_evidence()
-> WorkGraphPersistenceCanaryDurableIdentityEvidencePreview {
    WorkGraphPersistenceCanaryDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: work_graph_persistence_canary_durable_identity_field_ids(),
        required_for_dry_run_scenario_ids: work_graph_persistence_canary_dry_run_scenario_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_canary_dry_run_invariants()
-> Vec<WorkGraphPersistenceCanaryDryRunInvariantPreview> {
    vec![
        invariant(
            "canary_dry_run_requires_durable_identity_evidence",
            "every canary dry-run scenario requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "canary_dry_run_requires_feature_flags_default_off",
            "canary dry-run cannot start unless persistence feature flags remain disabled",
        ),
        invariant(
            "canary_dry_run_is_lane_scoped",
            "dry-run evidence is scoped to the hepta-backend lane and cannot cross into live runtime lanes",
        ),
        invariant(
            "canary_dry_run_has_zero_live_traffic",
            "all canary stages remain at 0 traffic ppm in this preview",
        ),
        invariant(
            "canary_dry_run_has_zero_live_writes",
            "nodes, edges, TaskResults, artifacts, approvals, and timeline events cannot be persisted",
        ),
        invariant(
            "rollback_receipts_are_redacted_and_non_persistent",
            "rollback receipt previews carry hashes and fields only and cannot be stored by this gate",
        ),
        invariant(
            "persistence_canary_dry_run_preview_has_no_side_effects",
            "this gate cannot execute canaries, mutate flags, write WAL/checkpoints, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceCanaryDryRunPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            feature_flag_mutated: false,
            canary_executed: false,
            live_traffic_routed: false,
            wal_written: false,
            checkpoint_written: false,
            receipt_persisted: false,
            rollback_performed: false,
            scheduler_cutover_performed: false,
            approval_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn lane_guard(
    id: &'static str,
    lane_id: &'static str,
    required_env: &'static str,
    scope: &'static str,
) -> WorkGraphPersistenceCanaryLaneGuardPreview {
    WorkGraphPersistenceCanaryLaneGuardPreview {
        id,
        lane_id,
        required_env,
        scope,
        blocks_cross_lane_execution: true,
        live_execution_allowed: false,
    }
}

fn dry_run_scenario(
    id: &'static str,
    source_feature_flag_id: &'static str,
    input_fixture_id: &'static str,
    expected_evidence_ids: Vec<&'static str>,
) -> WorkGraphPersistenceCanaryDryRunScenarioPreview {
    WorkGraphPersistenceCanaryDryRunScenarioPreview {
        id,
        source_feature_flag_id,
        input_fixture_id,
        expected_evidence_ids,
        max_runtime_ms: 30_000,
        traffic_ppm: 0,
        writes_allowed: false,
        promotion_allowed: false,
    }
}

fn traffic_guard(
    id: &'static str,
    applies_to_stage_ids: Vec<&'static str>,
) -> WorkGraphPersistenceCanaryTrafficGuardPreview {
    WorkGraphPersistenceCanaryTrafficGuardPreview {
        id,
        applies_to_stage_ids,
        max_traffic_ppm: 0,
        blocks_live_traffic: true,
    }
}

fn write_guard(
    id: &'static str,
    target_collection_id: &'static str,
) -> WorkGraphPersistenceCanaryWriteGuardPreview {
    WorkGraphPersistenceCanaryWriteGuardPreview {
        id,
        target_collection_id,
        allowed_write_mode: "none",
        blocks_live_writes: true,
        mutates_store: false,
    }
}

fn rollback_receipt(
    id: &'static str,
    trigger_guard_id: &'static str,
) -> WorkGraphPersistenceCanaryRollbackReceiptPreview {
    WorkGraphPersistenceCanaryRollbackReceiptPreview {
        id,
        trigger_guard_id,
        required_fields: with_canary_durable_identity_fields(vec![
            "receiptId",
            "triggerGuardId",
            "featureFlagId",
            "laneId",
            "evidenceHash",
            "redactionState",
        ]),
        persistence_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_canary_durable_identity_fields(fields: Vec<&'static str>) -> Vec<&'static str> {
    let mut merged = work_graph_persistence_canary_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceCanaryDryRunInvariantPreview {
    WorkGraphPersistenceCanaryDryRunInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistence_canary_dry_run_preview_declares_lane_guards() {
        let report = hepta_work_graph_persistence_canary_dry_run_preview_report();
        let guard_ids = report
            .lane_guards
            .iter()
            .map(|guard| guard.id)
            .collect::<Vec<_>>();

        assert_eq!(
            guard_ids,
            [
                "hepta_backend_lane_lock_required",
                "cargo_target_dir_isolated",
                "no_cross_lane_runtime_write",
                "no_external_delivery_lane",
                "no_operator_approval_recording_lane",
            ]
        );
        assert_eq!(report.lane_guard_count, 5);
        assert!(report.lane_guards.iter().all(|guard| {
            guard.lane_id == "hepta-backend"
                && guard.blocks_cross_lane_execution
                && !guard.live_execution_allowed
        }));
    }

    #[test]
    fn persistence_canary_dry_run_preview_covers_feature_flags() {
        let report = hepta_work_graph_persistence_canary_dry_run_preview_report();
        let feature_flag_ids = report
            .dry_run_scenarios
            .iter()
            .map(|scenario| scenario.source_feature_flag_id)
            .collect::<Vec<_>>();

        assert_eq!(
            feature_flag_ids,
            [
                "work_graph_store_persistence_flag",
                "work_graph_wal_append_flag",
                "work_graph_checkpoint_write_flag",
                "work_graph_readback_receipt_persistence_flag",
                "work_graph_idempotency_index_write_flag",
                "work_graph_replay_execution_feature_flag",
            ]
        );
        assert_eq!(report.dry_run_scenario_count, 6);
        assert!(report.dry_run_scenarios.iter().all(|scenario| {
            scenario.traffic_ppm == 0
                && !scenario.writes_allowed
                && !scenario.promotion_allowed
                && scenario
                    .expected_evidence_ids
                    .contains(&"durable_identity_evidence_packet")
        }));
    }

    #[test]
    fn persistence_canary_dry_run_preview_blocks_traffic_and_writes() {
        let report = hepta_work_graph_persistence_canary_dry_run_preview_report();
        let write_targets = report
            .write_guards
            .iter()
            .map(|guard| guard.target_collection_id)
            .collect::<Vec<_>>();

        assert_eq!(report.traffic_guard_count, 5);
        assert!(
            report
                .traffic_guards
                .iter()
                .all(|guard| guard.max_traffic_ppm == 0 && guard.blocks_live_traffic)
        );
        assert_eq!(
            write_targets,
            [
                "nodes",
                "edges",
                "taskResults",
                "artifacts",
                "approvals",
                "timelineEvents",
            ]
        );
        assert_eq!(report.write_guard_count, 6);
        assert!(
            report
                .write_guards
                .iter()
                .all(|guard| guard.blocks_live_writes && !guard.mutates_store)
        );
    }

    #[test]
    fn persistence_canary_dry_run_preview_keeps_receipts_non_persistent() {
        let report = hepta_work_graph_persistence_canary_dry_run_preview_report();
        let receipt_ids = report
            .rollback_receipts
            .iter()
            .map(|receipt| receipt.id)
            .collect::<Vec<_>>();

        assert_eq!(
            receipt_ids,
            [
                "operator_kill_switch_receipt",
                "wal_checksum_mismatch_receipt",
                "shadow_readback_drift_receipt",
                "idempotency_collision_receipt",
                "disk_budget_exceeded_receipt",
                "operator_approval_expired_receipt",
            ]
        );
        assert_eq!(report.rollback_receipt_count, 6);
        assert!(report.rollback_receipts.iter().all(|receipt| {
            !receipt.persistence_enabled
                && !receipt.external_delivery_enabled
                && receipt.required_fields.contains(&"redactionState")
                && work_graph_persistence_canary_durable_identity_field_ids()
                    .iter()
                    .all(|field| receipt.required_fields.contains(field))
        }));
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_canary_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_dry_run_scenario_ids,
            work_graph_persistence_canary_dry_run_scenario_ids()
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceCanaryDryRunPreviewSideEffects::none()
        );
        assert!(report.ready_for_canary_readback_receipt_preview);
        assert!(!report.ready_for_canary_execution);
        assert!(!report.ready_for_live_persistence);
    }

    #[test]
    fn persistence_canary_dry_run_preview_requires_feature_flag_gate() {
        let report = hepta_work_graph_persistence_canary_dry_run_preview_report();

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
                "hepta_work_graph_state_store_persistence_preview_gate",
                "hepta_work_graph_replay_readback_preview_gate",
                "hepta_work_graph_promotion_precondition_preview_gate",
                "hepta_work_graph_activation_enforcement_blocker_preview_gate",
                "hepta_work_graph_shadow_adapter_readback_preview_gate",
                "hepta_work_graph_persistence_feature_flag_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_CANARY_DRY_RUN_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.invariant_count, 7);
    }
}
