use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ENFORCEMENT_ROLLOUT_BLOCKER_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ENFORCEMENT_ROLLOUT_BLOCKER_SCHEMA_VERSION: &str =
    "work_graph_persistence_enforcement_rollout_blocker_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ENFORCEMENT_ROLLOUT_BLOCKER_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_operator_readiness_packet_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceEnforcementRolloutBlockerPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub rollout_stage_count: usize,
    pub traffic_ramp_blocker_count: usize,
    pub kill_switch_count: usize,
    pub operator_enablement_count: usize,
    pub rollback_owner_count: usize,
    pub release_denial_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub rollout_stages: Vec<WorkGraphPersistenceEnforcementRolloutStagePreview>,
    pub traffic_ramp_blockers: Vec<WorkGraphPersistenceTrafficRampBlockerPreview>,
    pub kill_switches: Vec<WorkGraphPersistenceEnforcementKillSwitchPreview>,
    pub operator_enablements: Vec<WorkGraphPersistenceOperatorEnablementPacketPreview>,
    pub rollback_owners: Vec<WorkGraphPersistenceRollbackOwnerPreview>,
    pub release_denials: Vec<WorkGraphPersistenceEnforcementReleaseDenialPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceEnforcementRolloutDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceEnforcementRolloutInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_operator_readiness_packet_preview: bool,
    pub ready_for_enforcement_rollout: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceEnforcementRolloutBlockerPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceEnforcementRolloutStagePreview {
    pub id: &'static str,
    pub target_surface_id: &'static str,
    pub required_evidence_fields: Vec<&'static str>,
    pub max_traffic_ppm: u32,
    pub enforcement_enabled: bool,
    pub blocks_release: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceTrafficRampBlockerPreview {
    pub id: &'static str,
    pub applies_to_stage_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub max_allowed_traffic_ppm: u32,
    pub blocks_ramp: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceEnforcementKillSwitchPreview {
    pub id: &'static str,
    pub target_stage_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub rollback_owner_id: &'static str,
    pub armed_in_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorEnablementPacketPreview {
    pub id: &'static str,
    pub required_for_stage_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub currently_satisfied: bool,
    pub approval_recorded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceRollbackOwnerPreview {
    pub id: &'static str,
    pub owns_stage_ids: Vec<&'static str>,
    pub quarantine_scope: &'static str,
    pub required_receipt_fields: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceEnforcementReleaseDenialPreview {
    pub id: &'static str,
    pub target_surface_id: &'static str,
    pub reason: &'static str,
    pub required_clearance_ids: Vec<&'static str>,
    pub blocks_release: bool,
    pub blocks_publication: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceEnforcementRolloutDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_rollout_stage_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceEnforcementRolloutInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceEnforcementRolloutBlockerPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub enforcement_enabled: bool,
    pub rollout_started: bool,
    pub traffic_routed: bool,
    pub live_readback_executed: bool,
    pub promotion_performed: bool,
    pub release_published: bool,
    pub operator_approval_recorded: bool,
    pub rollback_performed: bool,
    pub quarantine_performed: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_enforcement_rollout_blocker_preview_report()
-> WorkGraphPersistenceEnforcementRolloutBlockerPreviewReport {
    let rollout_stages = work_graph_persistence_enforcement_rollout_stages();
    let traffic_ramp_blockers = work_graph_persistence_traffic_ramp_blockers();
    let kill_switches = work_graph_persistence_enforcement_kill_switches();
    let operator_enablements = work_graph_persistence_operator_enablement_packets();
    let rollback_owners = work_graph_persistence_rollback_owners();
    let release_denials = work_graph_persistence_enforcement_release_denials();
    let durable_identity_evidence =
        work_graph_persistence_enforcement_rollout_durable_identity_evidence();
    let invariants = work_graph_persistence_enforcement_rollout_invariants();

    WorkGraphPersistenceEnforcementRolloutBlockerPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ENFORCEMENT_ROLLOUT_BLOCKER_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_ENFORCEMENT_ROLLOUT_BLOCKER_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_enforcement_rollout_blocker_preview_no_rollout",
        rollout_stage_count: rollout_stages.len(),
        traffic_ramp_blocker_count: traffic_ramp_blockers.len(),
        kill_switch_count: kill_switches.len(),
        operator_enablement_count: operator_enablements.len(),
        rollback_owner_count: rollback_owners.len(),
        release_denial_count: release_denials.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_persistence_enforcement_rollout_required_prior_gates(),
        rollout_stages,
        traffic_ramp_blockers,
        kill_switches,
        operator_enablements,
        rollback_owners,
        release_denials,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ENFORCEMENT_ROLLOUT_BLOCKER_RECOMMENDED_NEXT_GATE,
        ready_for_operator_readiness_packet_preview: true,
        ready_for_enforcement_rollout: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceEnforcementRolloutBlockerPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_enforcement_rollout_required_prior_gates() -> Vec<&'static str> {
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
        "hepta_work_graph_persistence_canary_dry_run_preview_gate",
        "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
        "hepta_work_graph_persistence_promotion_blocker_preview_gate",
        "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_enforcement_rollout_stage_ids() -> Vec<&'static str> {
    vec![
        "store_persistence_enforcement_rollout",
        "wal_append_enforcement_rollout",
        "checkpoint_write_enforcement_rollout",
        "readback_receipt_enforcement_rollout",
        "replay_execution_enforcement_rollout",
        "external_publication_enforcement_rollout",
    ]
}

pub fn work_graph_persistence_enforcement_rollout_durable_identity_field_ids() -> Vec<&'static str>
{
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

pub fn work_graph_persistence_enforcement_rollout_stages()
-> Vec<WorkGraphPersistenceEnforcementRolloutStagePreview> {
    vec![
        rollout_stage(
            "store_persistence_enforcement_rollout",
            "durable_work_graph_store",
            with_enforcement_rollout_durable_identity_fields(vec![
                "featureFlagDefaultOff",
                "shadowLiveDigestMatch",
                "operatorEnablementPacketHash",
                "rollbackOwnerHash",
            ]),
        ),
        rollout_stage(
            "wal_append_enforcement_rollout",
            "work_graph_wal",
            with_enforcement_rollout_durable_identity_fields(vec![
                "walSchemaDigest",
                "idempotencyGuardHash",
                "trafficRampBlockerIds",
                "killSwitchId",
            ]),
        ),
        rollout_stage(
            "checkpoint_write_enforcement_rollout",
            "work_graph_checkpoint",
            with_enforcement_rollout_durable_identity_fields(vec![
                "checkpointSchemaDigest",
                "diskBudgetHash",
                "rollbackOwnerHash",
                "releaseDenialId",
            ]),
        ),
        rollout_stage(
            "readback_receipt_enforcement_rollout",
            "readback_receipt_store",
            with_enforcement_rollout_durable_identity_fields(vec![
                "receiptSchemaHash",
                "redactionState",
                "operatorEnablementPacketHash",
                "quarantineScope",
            ]),
        ),
        rollout_stage(
            "replay_execution_enforcement_rollout",
            "work_graph_replay_executor",
            with_enforcement_rollout_durable_identity_fields(vec![
                "driftBudgetHash",
                "replayIdempotencyHash",
                "laneLeaseHash",
                "killSwitchId",
            ]),
        ),
        rollout_stage(
            "external_publication_enforcement_rollout",
            "external_delivery",
            with_enforcement_rollout_durable_identity_fields(vec![
                "externalPolicyHash",
                "deliveryReadbackGate",
                "operatorScopeHash",
                "publicationDenialId",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_traffic_ramp_blockers()
-> Vec<WorkGraphPersistenceTrafficRampBlockerPreview> {
    let all_stages = work_graph_persistence_enforcement_rollout_stage_ids();
    let persistent_stages = vec![
        "store_persistence_enforcement_rollout",
        "wal_append_enforcement_rollout",
        "checkpoint_write_enforcement_rollout",
        "readback_receipt_enforcement_rollout",
        "replay_execution_enforcement_rollout",
    ];

    vec![
        traffic_ramp_blocker(
            "ramp_blocked_without_operator_packet",
            all_stages.clone(),
            "operator enablement packet is missing or unsatisfied",
        ),
        traffic_ramp_blocker(
            "ramp_blocked_without_durable_identity",
            all_stages.clone(),
            "durable identity evidence packet is missing or unsatisfied",
        ),
        traffic_ramp_blocker(
            "ramp_blocked_without_shadow_live_match",
            persistent_stages.clone(),
            "shadow/live readback comparison is missing or mismatched",
        ),
        traffic_ramp_blocker(
            "ramp_blocked_without_kill_switch",
            all_stages.clone(),
            "kill switch is not armed before rollout",
        ),
        traffic_ramp_blocker(
            "ramp_blocked_without_rollback_owner",
            persistent_stages,
            "rollback/quarantine owner is missing",
        ),
        traffic_ramp_blocker(
            "ramp_blocked_without_release_denial_matrix",
            all_stages.clone(),
            "release and publication denial matrix is missing",
        ),
        traffic_ramp_blocker(
            "ramp_blocked_for_external_publication",
            vec!["external_publication_enforcement_rollout"],
            "external publication remains blocked in persistence rollout preview",
        ),
    ]
}

pub fn work_graph_persistence_enforcement_kill_switches()
-> Vec<WorkGraphPersistenceEnforcementKillSwitchPreview> {
    vec![
        kill_switch(
            "kill_store_persistence_rollout",
            vec!["store_persistence_enforcement_rollout"],
            "store readback, schema, or idempotency evidence fails",
            "rollback_owner_store_persistence",
        ),
        kill_switch(
            "kill_wal_checkpoint_rollout",
            vec![
                "wal_append_enforcement_rollout",
                "checkpoint_write_enforcement_rollout",
            ],
            "WAL or checkpoint digest diverges",
            "rollback_owner_wal_checkpoint",
        ),
        kill_switch(
            "kill_readback_receipt_rollout",
            vec!["readback_receipt_enforcement_rollout"],
            "receipt redaction or persistence evidence fails",
            "rollback_owner_receipts",
        ),
        kill_switch(
            "kill_replay_execution_rollout",
            vec!["replay_execution_enforcement_rollout"],
            "replay drift, idempotency, or lane lease evidence fails",
            "rollback_owner_replay",
        ),
        kill_switch(
            "kill_external_publication_rollout",
            vec!["external_publication_enforcement_rollout"],
            "external delivery policy, readback, or operator scope fails",
            "rollback_owner_external_publication",
        ),
    ]
}

pub fn work_graph_persistence_operator_enablement_packets()
-> Vec<WorkGraphPersistenceOperatorEnablementPacketPreview> {
    vec![
        operator_enablement(
            "operator_enable_store_persistence_rollout",
            vec!["store_persistence_enforcement_rollout"],
            with_enforcement_rollout_durable_identity_fields(vec![
                "operatorScopeHash",
                "featureFlagName",
                "shadowLiveDigestHash",
                "rollbackOwnerHash",
            ]),
        ),
        operator_enablement(
            "operator_enable_wal_checkpoint_rollout",
            vec![
                "wal_append_enforcement_rollout",
                "checkpoint_write_enforcement_rollout",
            ],
            with_enforcement_rollout_durable_identity_fields(vec![
                "walSchemaDigest",
                "checkpointSchemaDigest",
                "diskBudgetHash",
                "killSwitchId",
            ]),
        ),
        operator_enablement(
            "operator_enable_readback_receipt_rollout",
            vec!["readback_receipt_enforcement_rollout"],
            with_enforcement_rollout_durable_identity_fields(vec![
                "receiptSchemaHash",
                "redactionState",
                "receiptRetentionPolicyHash",
                "quarantineScope",
            ]),
        ),
        operator_enablement(
            "operator_enable_replay_execution_rollout",
            vec!["replay_execution_enforcement_rollout"],
            with_enforcement_rollout_durable_identity_fields(vec![
                "driftBudgetHash",
                "laneLeaseHash",
                "replayIdempotencyHash",
                "rollbackOwnerHash",
            ]),
        ),
        operator_enablement(
            "operator_enable_external_publication_rollout",
            vec!["external_publication_enforcement_rollout"],
            with_enforcement_rollout_durable_identity_fields(vec![
                "deliveryPolicyHash",
                "externalTargetScope",
                "deliveryReadbackGate",
                "publicationDenialId",
            ]),
        ),
        operator_enablement(
            "operator_enable_full_rollout_abort_packet",
            work_graph_persistence_enforcement_rollout_stage_ids(),
            with_enforcement_rollout_durable_identity_fields(vec![
                "abortReasonHash",
                "killSwitchIds",
                "rollbackOwnerIds",
                "quarantineScopes",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_rollback_owners() -> Vec<WorkGraphPersistenceRollbackOwnerPreview> {
    vec![
        rollback_owner(
            "rollback_owner_store_persistence",
            vec!["store_persistence_enforcement_rollout"],
            "durable_work_graph_store",
        ),
        rollback_owner(
            "rollback_owner_wal_checkpoint",
            vec![
                "wal_append_enforcement_rollout",
                "checkpoint_write_enforcement_rollout",
            ],
            "wal_and_checkpoint_writers",
        ),
        rollback_owner(
            "rollback_owner_receipts",
            vec!["readback_receipt_enforcement_rollout"],
            "readback_receipt_store",
        ),
        rollback_owner(
            "rollback_owner_replay",
            vec!["replay_execution_enforcement_rollout"],
            "replay_executor",
        ),
        rollback_owner(
            "rollback_owner_external_publication",
            vec!["external_publication_enforcement_rollout"],
            "external_delivery_pipeline",
        ),
    ]
}

pub fn work_graph_persistence_enforcement_release_denials()
-> Vec<WorkGraphPersistenceEnforcementReleaseDenialPreview> {
    vec![
        release_denial(
            "deny_store_persistence_enforcement_release",
            "durable_work_graph_store",
            "store persistence enforcement cannot release before operator enablement, shadow/live match, and rollback ownership are satisfied",
            vec![
                "operator_enable_store_persistence_rollout",
                "ramp_blocked_without_durable_identity",
                "ramp_blocked_without_shadow_live_match",
                "rollback_owner_store_persistence",
            ],
        ),
        release_denial(
            "deny_wal_checkpoint_enforcement_release",
            "wal_and_checkpoint_writers",
            "WAL/checkpoint enforcement cannot release before schema digest, disk budget, and kill switch evidence are satisfied",
            vec![
                "operator_enable_wal_checkpoint_rollout",
                "ramp_blocked_without_durable_identity",
                "kill_wal_checkpoint_rollout",
                "rollback_owner_wal_checkpoint",
            ],
        ),
        release_denial(
            "deny_readback_receipt_enforcement_release",
            "readback_receipt_store",
            "readback receipt enforcement cannot release while receipt persistence remains preview-only",
            vec![
                "operator_enable_readback_receipt_rollout",
                "ramp_blocked_without_durable_identity",
                "kill_readback_receipt_rollout",
                "rollback_owner_receipts",
            ],
        ),
        release_denial(
            "deny_replay_execution_enforcement_release",
            "work_graph_replay_executor",
            "replay execution enforcement cannot release before drift budget, lane lease, and rollback ownership are satisfied",
            vec![
                "operator_enable_replay_execution_rollout",
                "ramp_blocked_without_durable_identity",
                "kill_replay_execution_rollout",
                "rollback_owner_replay",
            ],
        ),
        release_denial(
            "deny_external_publication_enforcement_release",
            "external_delivery",
            "external publication enforcement cannot release without separate delivery policy and readback gate",
            vec![
                "operator_enable_external_publication_rollout",
                "ramp_blocked_without_durable_identity",
                "kill_external_publication_rollout",
                "ramp_blocked_for_external_publication",
            ],
        ),
        release_denial(
            "deny_full_rollout_public_claim",
            "operator_publication",
            "full rollout status cannot become a public claim from a preview gate",
            vec![
                "operator_enable_full_rollout_abort_packet",
                "ramp_blocked_without_durable_identity",
                "ramp_blocked_without_release_denial_matrix",
                "ramp_blocked_without_operator_packet",
            ],
        ),
    ]
}

pub fn work_graph_persistence_enforcement_rollout_durable_identity_evidence()
-> WorkGraphPersistenceEnforcementRolloutDurableIdentityEvidencePreview {
    WorkGraphPersistenceEnforcementRolloutDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: work_graph_persistence_enforcement_rollout_durable_identity_field_ids(),
        required_for_rollout_stage_ids: work_graph_persistence_enforcement_rollout_stage_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_enforcement_rollout_invariants()
-> Vec<WorkGraphPersistenceEnforcementRolloutInvariantPreview> {
    vec![
        invariant(
            "enforcement_rollout_requires_durable_identity_evidence",
            "rollout stages require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "enforcement_rollout_is_blocked_by_default",
            "every persistence enforcement stage has max traffic 0 and enforcement disabled",
        ),
        invariant(
            "traffic_ramp_requires_operator_packet_and_shadow_live_match",
            "traffic cannot ramp without operator enablement and shadow/live readback evidence",
        ),
        invariant(
            "kill_switches_precede_any_rollout_stage",
            "each rollout stage maps to an armed kill switch before future activation",
        ),
        invariant(
            "rollback_owners_are_explicit_and_unsatisfied",
            "rollback and quarantine ownership is declared but not accepted in preview",
        ),
        invariant(
            "release_and_publication_denied_independently",
            "release and external/publication paths have separate denial records",
        ),
        invariant(
            "enforcement_rollout_blocker_preview_has_no_side_effects",
            "this gate cannot enable enforcement, route traffic, write state, record approvals, publish releases, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceEnforcementRolloutBlockerPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            enforcement_enabled: false,
            rollout_started: false,
            traffic_routed: false,
            live_readback_executed: false,
            promotion_performed: false,
            release_published: false,
            operator_approval_recorded: false,
            rollback_performed: false,
            quarantine_performed: false,
            wal_written: false,
            checkpoint_written: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn with_enforcement_rollout_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged = work_graph_persistence_enforcement_rollout_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn rollout_stage(
    id: &'static str,
    target_surface_id: &'static str,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphPersistenceEnforcementRolloutStagePreview {
    WorkGraphPersistenceEnforcementRolloutStagePreview {
        id,
        target_surface_id,
        required_evidence_fields,
        max_traffic_ppm: 0,
        enforcement_enabled: false,
        blocks_release: true,
    }
}

fn traffic_ramp_blocker(
    id: &'static str,
    applies_to_stage_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphPersistenceTrafficRampBlockerPreview {
    WorkGraphPersistenceTrafficRampBlockerPreview {
        id,
        applies_to_stage_ids,
        trigger,
        max_allowed_traffic_ppm: 0,
        blocks_ramp: true,
    }
}

fn kill_switch(
    id: &'static str,
    target_stage_ids: Vec<&'static str>,
    trigger: &'static str,
    rollback_owner_id: &'static str,
) -> WorkGraphPersistenceEnforcementKillSwitchPreview {
    WorkGraphPersistenceEnforcementKillSwitchPreview {
        id,
        target_stage_ids,
        trigger,
        rollback_owner_id,
        armed_in_preview: true,
    }
}

fn operator_enablement(
    id: &'static str,
    required_for_stage_ids: Vec<&'static str>,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceOperatorEnablementPacketPreview {
    WorkGraphPersistenceOperatorEnablementPacketPreview {
        id,
        required_for_stage_ids,
        required_fields,
        currently_satisfied: false,
        approval_recorded: false,
    }
}

fn rollback_owner(
    id: &'static str,
    owns_stage_ids: Vec<&'static str>,
    quarantine_scope: &'static str,
) -> WorkGraphPersistenceRollbackOwnerPreview {
    WorkGraphPersistenceRollbackOwnerPreview {
        id,
        owns_stage_ids,
        quarantine_scope,
        required_receipt_fields: with_enforcement_rollout_durable_identity_fields(vec![
            "rollbackOwnerId",
            "quarantineScope",
            "killSwitchId",
            "recoveryOwnerHash",
        ]),
        currently_satisfied: false,
    }
}

fn release_denial(
    id: &'static str,
    target_surface_id: &'static str,
    reason: &'static str,
    required_clearance_ids: Vec<&'static str>,
) -> WorkGraphPersistenceEnforcementReleaseDenialPreview {
    WorkGraphPersistenceEnforcementReleaseDenialPreview {
        id,
        target_surface_id,
        reason,
        required_clearance_ids,
        blocks_release: true,
        blocks_publication: true,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceEnforcementRolloutInvariantPreview {
    WorkGraphPersistenceEnforcementRolloutInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforcement_rollout_blocker_preview_declares_blocked_stages() {
        let report = hepta_work_graph_persistence_enforcement_rollout_blocker_preview_report();
        let stage_ids = report
            .rollout_stages
            .iter()
            .map(|stage| stage.id)
            .collect::<Vec<_>>();

        assert_eq!(
            stage_ids,
            [
                "store_persistence_enforcement_rollout",
                "wal_append_enforcement_rollout",
                "checkpoint_write_enforcement_rollout",
                "readback_receipt_enforcement_rollout",
                "replay_execution_enforcement_rollout",
                "external_publication_enforcement_rollout",
            ]
        );
        assert_eq!(report.rollout_stage_count, 6);
        let durable_fields =
            work_graph_persistence_enforcement_rollout_durable_identity_field_ids();
        assert!(report.rollout_stages.iter().all(|stage| {
            stage.max_traffic_ppm == 0
                && !stage.enforcement_enabled
                && stage.blocks_release
                && stage.required_evidence_fields.len() >= 11
                && durable_fields
                    .iter()
                    .all(|field| stage.required_evidence_fields.contains(field))
        }));
    }

    #[test]
    fn enforcement_rollout_blocker_preview_blocks_all_traffic_ramps() {
        let report = hepta_work_graph_persistence_enforcement_rollout_blocker_preview_report();
        let blocker_ids = report
            .traffic_ramp_blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_ids,
            [
                "ramp_blocked_without_operator_packet",
                "ramp_blocked_without_durable_identity",
                "ramp_blocked_without_shadow_live_match",
                "ramp_blocked_without_kill_switch",
                "ramp_blocked_without_rollback_owner",
                "ramp_blocked_without_release_denial_matrix",
                "ramp_blocked_for_external_publication",
            ]
        );
        assert_eq!(report.traffic_ramp_blocker_count, 7);
        assert!(report.traffic_ramp_blockers.iter().all(|blocker| {
            blocker.blocks_ramp
                && blocker.max_allowed_traffic_ppm == 0
                && !blocker.applies_to_stage_ids.is_empty()
        }));
    }

    #[test]
    fn enforcement_rollout_blocker_preview_requires_kill_switches_and_owners() {
        let report = hepta_work_graph_persistence_enforcement_rollout_blocker_preview_report();
        let kill_switch_ids = report
            .kill_switches
            .iter()
            .map(|kill_switch| kill_switch.id)
            .collect::<Vec<_>>();
        let owner_ids = report
            .rollback_owners
            .iter()
            .map(|owner| owner.id)
            .collect::<Vec<_>>();

        assert_eq!(
            kill_switch_ids,
            [
                "kill_store_persistence_rollout",
                "kill_wal_checkpoint_rollout",
                "kill_readback_receipt_rollout",
                "kill_replay_execution_rollout",
                "kill_external_publication_rollout",
            ]
        );
        assert_eq!(report.kill_switch_count, 5);
        assert!(report.kill_switches.iter().all(|kill_switch| {
            kill_switch.armed_in_preview && !kill_switch.target_stage_ids.is_empty()
        }));
        assert_eq!(
            owner_ids,
            [
                "rollback_owner_store_persistence",
                "rollback_owner_wal_checkpoint",
                "rollback_owner_receipts",
                "rollback_owner_replay",
                "rollback_owner_external_publication",
            ]
        );
        assert_eq!(report.rollback_owner_count, 5);
        assert!(report.rollback_owners.iter().all(|owner| {
            !owner.currently_satisfied
                && owner.required_receipt_fields.len() >= 11
                && owner.required_receipt_fields.contains(&"workflow_id")
                && owner.required_receipt_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn enforcement_rollout_blocker_preview_keeps_operator_enablement_unsatisfied() {
        let report = hepta_work_graph_persistence_enforcement_rollout_blocker_preview_report();
        let enablement_ids = report
            .operator_enablements
            .iter()
            .map(|enablement| enablement.id)
            .collect::<Vec<_>>();

        assert_eq!(
            enablement_ids,
            [
                "operator_enable_store_persistence_rollout",
                "operator_enable_wal_checkpoint_rollout",
                "operator_enable_readback_receipt_rollout",
                "operator_enable_replay_execution_rollout",
                "operator_enable_external_publication_rollout",
                "operator_enable_full_rollout_abort_packet",
            ]
        );
        assert_eq!(report.operator_enablement_count, 6);
        assert!(report.operator_enablements.iter().all(|enablement| {
            !enablement.currently_satisfied
                && !enablement.approval_recorded
                && enablement.required_fields.len() >= 11
                && enablement.required_fields.contains(&"workflow_id")
                && enablement.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn enforcement_rollout_blocker_preview_denies_release_and_publication() {
        let report = hepta_work_graph_persistence_enforcement_rollout_blocker_preview_report();
        let denial_ids = report
            .release_denials
            .iter()
            .map(|denial| denial.id)
            .collect::<Vec<_>>();

        assert_eq!(
            denial_ids,
            [
                "deny_store_persistence_enforcement_release",
                "deny_wal_checkpoint_enforcement_release",
                "deny_readback_receipt_enforcement_release",
                "deny_replay_execution_enforcement_release",
                "deny_external_publication_enforcement_release",
                "deny_full_rollout_public_claim",
            ]
        );
        assert_eq!(report.release_denial_count, 6);
        assert!(report.release_denials.iter().all(|denial| {
            denial.blocks_release
                && denial.blocks_publication
                && denial.required_clearance_ids.len() >= 4
                && denial
                    .required_clearance_ids
                    .contains(&"ramp_blocked_without_durable_identity")
        }));
    }

    #[test]
    fn enforcement_rollout_blocker_preview_requires_shadow_live_comparison_gate() {
        let report = hepta_work_graph_persistence_enforcement_rollout_blocker_preview_report();

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
                "hepta_work_graph_persistence_canary_dry_run_preview_gate",
                "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
                "hepta_work_graph_persistence_promotion_blocker_preview_gate",
                "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.durable_identity_evidence.schema_version,
            "work_graph_durable_identity_preview_v1"
        );
        assert_eq!(
            report.durable_identity_evidence.required_prior_gate,
            "hepta_work_graph_durable_identity_preview_gate"
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_enforcement_rollout_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_rollout_stage_ids,
            work_graph_persistence_enforcement_rollout_stage_ids()
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ENFORCEMENT_ROLLOUT_BLOCKER_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceEnforcementRolloutBlockerPreviewSideEffects::none()
        );
        assert!(report.ready_for_operator_readiness_packet_preview);
        assert!(!report.ready_for_enforcement_rollout);
        assert!(!report.ready_for_live_persistence);
    }
}
