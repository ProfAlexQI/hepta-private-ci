use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_FEATURE_FLAG_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_feature_flag_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_FEATURE_FLAG_SCHEMA_VERSION: &str =
    "work_graph_persistence_feature_flag_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_FEATURE_FLAG_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_canary_dry_run_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceFeatureFlagPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub feature_flag_count: usize,
    pub enablement_packet_count: usize,
    pub rollout_stage_count: usize,
    pub rollback_guard_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub feature_flags: Vec<WorkGraphPersistenceFeatureFlagPreview>,
    pub enablement_packets: Vec<WorkGraphPersistenceEnablementPacketPreview>,
    pub rollout_stages: Vec<WorkGraphPersistenceRolloutStagePreview>,
    pub rollback_guards: Vec<WorkGraphPersistenceRollbackGuardPreview>,
    pub durable_identity_evidence: WorkGraphPersistenceDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceFeatureFlagInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_persistence_canary_dry_run_preview: bool,
    pub ready_for_feature_flag_activation: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceFeatureFlagPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceFeatureFlagPreview {
    pub id: &'static str,
    pub activation_surface_id: &'static str,
    pub scope: &'static str,
    pub required_enablement_ids: Vec<&'static str>,
    pub default_enabled: bool,
    pub operator_mutable_in_preview: bool,
    pub allows_live_writes_in_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceEnablementPacketPreview {
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub source_gate_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceRolloutStagePreview {
    pub id: &'static str,
    pub order: usize,
    pub traffic_ppm: u32,
    pub write_mode: &'static str,
    pub promotion_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceRollbackGuardPreview {
    pub id: &'static str,
    pub trigger: &'static str,
    pub blocks_feature_flag_activation: bool,
    pub required_before_any_write: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_feature_flag_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceFeatureFlagInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceFeatureFlagPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub feature_flag_mutated: bool,
    pub persistence_enabled: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_execution_enabled: bool,
    pub adapter_projection_enforced: bool,
    pub rollback_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub approval_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_feature_flag_preview_report()
-> WorkGraphPersistenceFeatureFlagPreviewReport {
    let feature_flags = work_graph_persistence_feature_flags();
    let enablement_packets = work_graph_persistence_enablement_packets();
    let rollout_stages = work_graph_persistence_rollout_stages();
    let rollback_guards = work_graph_persistence_rollback_guards();
    let durable_identity_evidence = work_graph_persistence_durable_identity_evidence();
    let invariants = work_graph_persistence_feature_flag_invariants();

    WorkGraphPersistenceFeatureFlagPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_FEATURE_FLAG_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_FEATURE_FLAG_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_feature_flag_preview_no_flag_mutation",
        feature_flag_count: feature_flags.len(),
        enablement_packet_count: enablement_packets.len(),
        rollout_stage_count: rollout_stages.len(),
        rollback_guard_count: rollback_guards.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_persistence_feature_flag_required_prior_gates(),
        feature_flags,
        enablement_packets,
        rollout_stages,
        rollback_guards,
        durable_identity_evidence,
        invariants,
        recommended_next_gate: WORK_GRAPH_PERSISTENCE_FEATURE_FLAG_RECOMMENDED_NEXT_GATE,
        ready_for_persistence_canary_dry_run_preview: true,
        ready_for_feature_flag_activation: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceFeatureFlagPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_feature_flag_required_prior_gates() -> Vec<&'static str> {
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_feature_flag_ids() -> Vec<&'static str> {
    vec![
        "work_graph_store_persistence_flag",
        "work_graph_wal_append_flag",
        "work_graph_checkpoint_write_flag",
        "work_graph_readback_receipt_persistence_flag",
        "work_graph_idempotency_index_write_flag",
        "work_graph_replay_execution_feature_flag",
    ]
}

pub fn work_graph_persistence_durable_identity_field_ids() -> Vec<&'static str> {
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

pub fn work_graph_persistence_feature_flags() -> Vec<WorkGraphPersistenceFeatureFlagPreview> {
    vec![
        feature_flag(
            "work_graph_store_persistence_flag",
            "store_persistence_activation",
            "persist_canonical_graph_collections",
            vec![
                "durable_identity_evidence_packet",
                "explicit_feature_flag",
                "prior_gate_digest",
                "shadow_readback_digest",
                "operator_approval_packet",
                "rollback_plan",
            ],
        ),
        feature_flag(
            "work_graph_wal_append_flag",
            "store_persistence_activation",
            "append_wal_records",
            vec![
                "durable_identity_evidence_packet",
                "explicit_feature_flag",
                "wal_schema_digest",
                "idempotency_guard_digest",
                "rollback_plan",
            ],
        ),
        feature_flag(
            "work_graph_checkpoint_write_flag",
            "store_persistence_activation",
            "write_checkpoint_snapshots",
            vec![
                "durable_identity_evidence_packet",
                "explicit_feature_flag",
                "checkpoint_hash_plan",
                "disk_budget_packet",
                "rollback_plan",
            ],
        ),
        feature_flag(
            "work_graph_readback_receipt_persistence_flag",
            "approval_recording_activation",
            "persist_readback_receipts",
            vec![
                "durable_identity_evidence_packet",
                "explicit_feature_flag",
                "shadow_readback_digest",
                "redaction_packet",
                "operator_approval_packet",
            ],
        ),
        feature_flag(
            "work_graph_idempotency_index_write_flag",
            "store_persistence_activation",
            "write_idempotency_indexes",
            vec![
                "durable_identity_evidence_packet",
                "explicit_feature_flag",
                "idempotency_guard_digest",
                "rollback_plan",
            ],
        ),
        feature_flag(
            "work_graph_replay_execution_feature_flag",
            "wal_replay_execution_activation",
            "execute_replay_against_persisted_graph",
            vec![
                "durable_identity_evidence_packet",
                "explicit_feature_flag",
                "prior_gate_digest",
                "drift_budget_packet",
                "rollback_plan",
            ],
        ),
    ]
}

pub fn work_graph_persistence_enablement_packets()
-> Vec<WorkGraphPersistenceEnablementPacketPreview> {
    vec![
        enablement_packet(
            "durable_identity_evidence_packet",
            work_graph_persistence_durable_identity_field_ids(),
            vec!["hepta_work_graph_durable_identity_preview_gate"],
        ),
        enablement_packet(
            "explicit_feature_flag",
            vec![
                "featureFlagName",
                "defaultState",
                "proposedEnabledAtUnixMs",
                "operatorIdHash",
            ],
            vec!["hepta_work_graph_activation_enforcement_blocker_preview_gate"],
        ),
        enablement_packet(
            "prior_gate_digest",
            vec!["requiredGateIds", "reportHash", "generatedAtUnixMs"],
            work_graph_persistence_feature_flag_required_prior_gates(),
        ),
        enablement_packet(
            "shadow_readback_digest",
            vec![
                "adapterShadowCount",
                "collectionReadbackCount",
                "mismatchCount",
                "evidencePacketHash",
            ],
            vec!["hepta_work_graph_shadow_adapter_readback_preview_gate"],
        ),
        enablement_packet(
            "operator_approval_packet",
            vec![
                "approvalId",
                "operatorIdHash",
                "scopeHash",
                "expiresAtUnixMs",
            ],
            vec!["hepta_work_graph_activation_enforcement_blocker_preview_gate"],
        ),
        enablement_packet(
            "rollback_plan",
            vec![
                "killSwitchId",
                "rollbackOwnerHash",
                "maxRollbackSeconds",
                "dataRetentionPolicy",
            ],
            vec!["hepta_work_graph_promotion_precondition_preview_gate"],
        ),
        enablement_packet(
            "wal_schema_digest",
            vec![
                "walVersion",
                "recordKinds",
                "checksumPolicy",
                "migrationPlanHash",
            ],
            vec!["hepta_work_graph_state_store_persistence_preview_gate"],
        ),
        enablement_packet(
            "idempotency_guard_digest",
            vec![
                "sourceSurfaceIds",
                "keyFieldsHash",
                "collisionPolicy",
                "dryRunCollisionCount",
            ],
            vec!["hepta_work_graph_state_store_persistence_preview_gate"],
        ),
        enablement_packet(
            "checkpoint_hash_plan",
            vec!["checkpointKind", "merkleRootPolicy", "readbackProbeIds"],
            vec!["hepta_work_graph_state_store_persistence_preview_gate"],
        ),
        enablement_packet(
            "disk_budget_packet",
            vec!["maxWalBytes", "maxCheckpointBytes", "prunePolicy"],
            vec!["hepta_work_graph_state_store_persistence_preview_gate"],
        ),
        enablement_packet(
            "drift_budget_packet",
            vec!["maxMismatchCount", "maxReplayLagMs", "escalationPolicy"],
            vec!["hepta_work_graph_replay_readback_preview_gate"],
        ),
        enablement_packet(
            "redaction_packet",
            vec![
                "redactionState",
                "piiPolicyHash",
                "externalDeliveryDisabled",
            ],
            vec![
                "hepta_work_graph_observability_timeline_preview_gate",
                "hepta_work_graph_shadow_adapter_readback_preview_gate",
            ],
        ),
    ]
}

pub fn work_graph_persistence_durable_identity_evidence()
-> WorkGraphPersistenceDurableIdentityEvidencePreview {
    WorkGraphPersistenceDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: work_graph_persistence_durable_identity_field_ids(),
        required_for_feature_flag_ids: work_graph_persistence_feature_flag_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_rollout_stages() -> Vec<WorkGraphPersistenceRolloutStagePreview> {
    vec![
        rollout_stage("disabled", 0, 0, "no_writes_no_reads", false),
        rollout_stage("local_dry_run", 1, 0, "report_only_no_store_writes", false),
        rollout_stage(
            "shadow_write_fixture_only",
            2,
            0,
            "fixture_projection_no_live_store_writes",
            false,
        ),
        rollout_stage(
            "shadow_readback_compare",
            3,
            0,
            "hash_compare_no_promotion",
            false,
        ),
        rollout_stage("canary_lane_dry_run", 4, 0, "lane_scoped_dry_run", false),
    ]
}

pub fn work_graph_persistence_rollback_guards() -> Vec<WorkGraphPersistenceRollbackGuardPreview> {
    vec![
        rollback_guard(
            "operator_kill_switch",
            "operator revokes the WorkGraph persistence feature flag",
        ),
        rollback_guard(
            "wal_checksum_mismatch",
            "WAL append or replay checksum does not match the expected digest",
        ),
        rollback_guard(
            "shadow_readback_drift",
            "shadow adapter readback mismatch count exceeds the configured drift budget",
        ),
        rollback_guard(
            "idempotency_collision",
            "source-surface idempotency key collision is detected before write",
        ),
        rollback_guard(
            "disk_budget_exceeded",
            "WAL or checkpoint budget would exceed the operator packet limit",
        ),
        rollback_guard(
            "operator_approval_expired",
            "operator approval packet expires before feature flag activation",
        ),
    ]
}

pub fn work_graph_persistence_feature_flag_invariants()
-> Vec<WorkGraphPersistenceFeatureFlagInvariantPreview> {
    vec![
        invariant(
            "feature_flags_require_durable_identity_evidence",
            "persistence feature flags cannot be considered without workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "feature_flags_default_off",
            "every WorkGraph persistence flag is disabled and non-mutable in this preview",
        ),
        invariant(
            "prior_gate_digest_required_before_flag_enablement",
            "feature flag activation cannot be considered until every prior gate has a hashed report",
        ),
        invariant(
            "operator_packet_and_rollback_plan_required",
            "a feature flag cannot authorize persistence without operator evidence and rollback scope",
        ),
        invariant(
            "canary_stages_have_zero_live_traffic_in_preview",
            "this preview can describe canary stages but cannot route live traffic or write state",
        ),
        invariant(
            "rollback_guards_block_any_write_path",
            "kill switch, checksum, drift, idempotency, disk, and approval guards block persistence",
        ),
        invariant(
            "persistence_feature_flag_preview_has_no_side_effects",
            "this gate cannot mutate config, write WAL/checkpoints, execute replay, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceFeatureFlagPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            feature_flag_mutated: false,
            persistence_enabled: false,
            wal_written: false,
            checkpoint_written: false,
            replay_execution_enabled: false,
            adapter_projection_enforced: false,
            rollback_performed: false,
            scheduler_cutover_performed: false,
            approval_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn feature_flag(
    id: &'static str,
    activation_surface_id: &'static str,
    scope: &'static str,
    required_enablement_ids: Vec<&'static str>,
) -> WorkGraphPersistenceFeatureFlagPreview {
    WorkGraphPersistenceFeatureFlagPreview {
        id,
        activation_surface_id,
        scope,
        required_enablement_ids,
        default_enabled: false,
        operator_mutable_in_preview: false,
        allows_live_writes_in_preview: false,
    }
}

fn enablement_packet(
    id: &'static str,
    required_fields: Vec<&'static str>,
    source_gate_ids: Vec<&'static str>,
) -> WorkGraphPersistenceEnablementPacketPreview {
    WorkGraphPersistenceEnablementPacketPreview {
        id,
        required_fields,
        source_gate_ids,
        currently_satisfied: false,
    }
}

fn rollout_stage(
    id: &'static str,
    order: usize,
    traffic_ppm: u32,
    write_mode: &'static str,
    promotion_allowed: bool,
) -> WorkGraphPersistenceRolloutStagePreview {
    WorkGraphPersistenceRolloutStagePreview {
        id,
        order,
        traffic_ppm,
        write_mode,
        promotion_allowed,
    }
}

fn rollback_guard(
    id: &'static str,
    trigger: &'static str,
) -> WorkGraphPersistenceRollbackGuardPreview {
    WorkGraphPersistenceRollbackGuardPreview {
        id,
        trigger,
        blocks_feature_flag_activation: true,
        required_before_any_write: true,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceFeatureFlagInvariantPreview {
    WorkGraphPersistenceFeatureFlagInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistence_feature_flag_preview_declares_default_off_flags() {
        let report = hepta_work_graph_persistence_feature_flag_preview_report();
        let flag_ids = report
            .feature_flags
            .iter()
            .map(|flag| flag.id)
            .collect::<Vec<_>>();

        assert_eq!(
            flag_ids,
            [
                "work_graph_store_persistence_flag",
                "work_graph_wal_append_flag",
                "work_graph_checkpoint_write_flag",
                "work_graph_readback_receipt_persistence_flag",
                "work_graph_idempotency_index_write_flag",
                "work_graph_replay_execution_feature_flag",
            ]
        );
        assert_eq!(report.feature_flag_count, 6);
        assert!(report.feature_flags.iter().all(|flag| {
            !flag.default_enabled
                && !flag.operator_mutable_in_preview
                && !flag.allows_live_writes_in_preview
                && flag
                    .required_enablement_ids
                    .contains(&"durable_identity_evidence_packet")
        }));
    }

    #[test]
    fn persistence_feature_flag_preview_requires_unsatisfied_enablement_packets() {
        let report = hepta_work_graph_persistence_feature_flag_preview_report();
        let packet_ids = report
            .enablement_packets
            .iter()
            .map(|packet| packet.id)
            .collect::<Vec<_>>();

        assert_eq!(
            packet_ids,
            [
                "durable_identity_evidence_packet",
                "explicit_feature_flag",
                "prior_gate_digest",
                "shadow_readback_digest",
                "operator_approval_packet",
                "rollback_plan",
                "wal_schema_digest",
                "idempotency_guard_digest",
                "checkpoint_hash_plan",
                "disk_budget_packet",
                "drift_budget_packet",
                "redaction_packet",
            ]
        );
        assert_eq!(report.enablement_packet_count, 12);
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_feature_flag_ids,
            work_graph_persistence_feature_flag_ids()
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert!(
            report
                .enablement_packets
                .iter()
                .all(|packet| !packet.currently_satisfied && !packet.required_fields.is_empty())
        );
    }

    #[test]
    fn persistence_feature_flag_preview_keeps_rollout_stages_dry() {
        let report = hepta_work_graph_persistence_feature_flag_preview_report();
        let stage_ids = report
            .rollout_stages
            .iter()
            .map(|stage| stage.id)
            .collect::<Vec<_>>();

        assert_eq!(
            stage_ids,
            [
                "disabled",
                "local_dry_run",
                "shadow_write_fixture_only",
                "shadow_readback_compare",
                "canary_lane_dry_run",
            ]
        );
        assert_eq!(report.rollout_stage_count, 5);
        assert!(
            report
                .rollout_stages
                .iter()
                .all(|stage| stage.traffic_ppm == 0 && !stage.promotion_allowed)
        );
    }

    #[test]
    fn persistence_feature_flag_preview_blocks_activation_with_rollback_guards() {
        let report = hepta_work_graph_persistence_feature_flag_preview_report();
        let guard_ids = report
            .rollback_guards
            .iter()
            .map(|guard| guard.id)
            .collect::<Vec<_>>();

        assert_eq!(
            guard_ids,
            [
                "operator_kill_switch",
                "wal_checksum_mismatch",
                "shadow_readback_drift",
                "idempotency_collision",
                "disk_budget_exceeded",
                "operator_approval_expired",
            ]
        );
        assert_eq!(report.rollback_guard_count, 6);
        assert!(report.rollback_guards.iter().all(|guard| {
            guard.blocks_feature_flag_activation && guard.required_before_any_write
        }));
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceFeatureFlagPreviewSideEffects::none()
        );
        assert!(report.ready_for_persistence_canary_dry_run_preview);
        assert!(!report.ready_for_feature_flag_activation);
        assert!(!report.ready_for_live_persistence);
    }

    #[test]
    fn persistence_feature_flag_preview_requires_shadow_readback_gate() {
        let report = hepta_work_graph_persistence_feature_flag_preview_report();

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
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_FEATURE_FLAG_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.invariant_count, 7);
    }
}
