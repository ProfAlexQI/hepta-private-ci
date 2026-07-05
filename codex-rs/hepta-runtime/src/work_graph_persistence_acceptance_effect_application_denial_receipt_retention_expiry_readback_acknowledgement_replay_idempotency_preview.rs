use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayPreviewReport
{
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub replay_scenario_count: usize,
    pub idempotency_guard_count: usize,
    pub replay_denial_count: usize,
    pub monotonicity_check_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub replay_scenarios:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayScenarioPreview>,
    pub idempotency_guards:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckIdempotencyGuardPreview>,
    pub replay_denials:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayDenialPreview>,
    pub monotonicity_checks:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckMonotonicityCheckPreview>,
    pub local_views:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayScenarioPreview
{
    pub id: &'static str,
    pub source_acknowledgement_ids: Vec<&'static str>,
    pub replay_mode: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acknowledgement_recording_allowed: bool,
    pub mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckIdempotencyGuardPreview
{
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_replay_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayDenialPreview
{
    pub id: &'static str,
    pub applies_to_replay_scenario_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckMonotonicityCheckPreview
{
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_out_of_order_replay: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview
{
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_replay_scenario_ids: Vec<&'static str>,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub retention_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub readback_acknowledgement_recorded: bool,
    pub replay_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub approval_recorded: bool,
    pub authority_granted: bool,
    pub live_persistence_enabled: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub enforcement_enabled: bool,
    pub rollout_started: bool,
    pub traffic_routed: bool,
    pub release_published: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayPreviewReport {
    let replay_scenarios =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_scenarios();
    let idempotency_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_idempotency_guards();
    let replay_denials =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_denials();
    let monotonicity_checks =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_monotonicity_checks();
    let local_views =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_evidence();
    let invariants =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_invariants();

    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_no_replay_write",
        replay_scenario_count: replay_scenarios.len(),
        idempotency_guard_count: idempotency_guards.len(),
        replay_denial_count: replay_denials.len(),
        monotonicity_check_count: monotonicity_checks.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_required_prior_gates(),
        replay_scenarios,
        idempotency_guards,
        replay_denials,
        monotonicity_checks,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_required_prior_gates()
-> Vec<&'static str> {
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
        "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate",
        "hepta_work_graph_persistence_operator_readiness_packet_preview_gate",
        "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate",
        "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate",
        "hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate",
        "hepta_work_graph_persistence_acceptance_record_intake_preview_gate",
        "hepta_work_graph_persistence_acceptance_record_receipt_preview_gate",
        "hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_scenario_ids()
-> Vec<&'static str> {
    vec![
        "duplicate_retention_readback_receipt_replay",
        "duplicate_retention_readback_acknowledgement_replay",
        "stale_retention_readback_digest_replay",
        "superseded_retention_scope_acknowledgement_replay",
        "cross_scope_retention_readback_acknowledgement_replay",
        "out_of_order_retention_readback_acknowledgement_replay",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_field_ids()
-> Vec<&'static str> {
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_scenarios()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayScenarioPreview> {
    let acknowledgement_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_source_acknowledgement_ids();

    vec![
        replay_scenario(
            "duplicate_retention_readback_receipt_replay",
            acknowledgement_ids.clone(),
            "duplicate_readback_receipt",
        ),
        replay_scenario(
            "duplicate_retention_readback_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "duplicate_acknowledgement",
        ),
        replay_scenario(
            "stale_retention_readback_digest_replay",
            acknowledgement_ids.clone(),
            "stale_readback_digest",
        ),
        replay_scenario(
            "superseded_retention_scope_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "superseded_retention_scope",
        ),
        replay_scenario(
            "cross_scope_retention_readback_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "cross_scope_acknowledgement",
        ),
        replay_scenario(
            "out_of_order_retention_readback_acknowledgement_replay",
            acknowledgement_ids,
            "out_of_order_acknowledgement",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_source_acknowledgement_ids()
-> Vec<&'static str> {
    vec![
        "retention_policy_readback_receipt_acknowledgement",
        "expiry_guard_readback_receipt_acknowledgement",
        "supersession_guard_readback_receipt_acknowledgement",
        "garbage_collection_denial_readback_receipt_acknowledgement",
        "zero_effect_digest_readback_receipt_acknowledgement",
        "release_external_denial_readback_receipt_acknowledgement",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_idempotency_guards()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckIdempotencyGuardPreview>
{
    vec![
        idempotency_guard(
            "retention_readback_receipt_idempotency_key_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackReceiptId",
                "retentionScope",
                "readbackReceiptHash",
            ]),
        ),
        idempotency_guard(
            "retention_readback_acknowledgement_idempotency_key_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "acknowledgementId",
                "acknowledgementHash",
                "localViewHash",
            ]),
        ),
        idempotency_guard(
            "retention_readback_prior_gate_digest_binding_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "priorGateId",
                "priorGateDigest",
                "sourceReportHash",
            ]),
        ),
        idempotency_guard(
            "retention_scope_epoch_binding_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "retentionScopeId",
                "scopeEpoch",
                "supersessionHash",
            ]),
        ),
        idempotency_guard(
            "retention_zero_side_effect_digest_binding_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
        idempotency_guard(
            "retention_readback_acknowledgement_sequence_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackSequence",
                "acknowledgementSequence",
                "sequenceHash",
            ]),
        ),
        idempotency_guard(
            "replay_does_not_unlock_retention_or_acceptance",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "retentionStatePersisted",
                "acceptanceAllowed",
                "authorityGranted",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_denials()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayDenialPreview> {
    let replay_scenario_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_scenario_ids();

    vec![
        replay_denial(
            "durable_identity_evidence_missing",
            replay_scenario_ids.clone(),
            "retention readback acknowledgement replay cannot proceed without durable identity evidence",
        ),
        replay_denial(
            "duplicate_readback_cannot_record_acknowledgement",
            replay_scenario_ids.clone(),
            "duplicate readback receipt replay cannot record acknowledgement",
        ),
        replay_denial(
            "duplicate_acknowledgement_cannot_record_acceptance",
            replay_scenario_ids.clone(),
            "duplicate acknowledgement replay cannot record acceptance",
        ),
        replay_denial(
            "stale_readback_digest_cannot_grant_authority",
            replay_scenario_ids.clone(),
            "stale readback digest replay cannot grant authority",
        ),
        replay_denial(
            "cross_scope_replay_cannot_enable_live_persistence",
            replay_scenario_ids.clone(),
            "cross-scope retention replay cannot enable live persistence, WAL, or checkpoints",
        ),
        replay_denial(
            "out_of_order_replay_cannot_start_rollout",
            replay_scenario_ids.clone(),
            "out-of-order retention replay cannot start rollout or route traffic",
        ),
        replay_denial(
            "superseded_replay_cannot_publish_release",
            replay_scenario_ids.clone(),
            "superseded retention replay cannot publish release status",
        ),
        replay_denial(
            "replayed_retention_acknowledgement_cannot_send_external_delivery",
            replay_scenario_ids,
            "replayed retention acknowledgement cannot send external delivery",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_monotonicity_checks()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckMonotonicityCheckPreview,
> {
    vec![
        monotonicity_check(
            "retention_readback_receipt_sequence_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackReceiptId",
                "readbackSequence",
                "readbackReceiptHash",
            ]),
        ),
        monotonicity_check(
            "retention_readback_acknowledgement_sequence_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "acknowledgementId",
                "acknowledgementSequence",
                "acknowledgementHash",
            ]),
        ),
        monotonicity_check(
            "retention_prior_gate_digest_monotonicity_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "priorGateId",
                "priorGateDigest",
                "observedAt",
            ]),
        ),
        monotonicity_check(
            "retention_scope_epoch_monotonicity_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "retentionScopeId",
                "scopeEpoch",
                "supersessionHash",
            ]),
        ),
        monotonicity_check(
            "retention_zero_effect_digest_stability_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_local_views()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayLocalViewPreview>
{
    vec![
        local_view(
            "operator_retention_readback_ack_replay_idempotency_view",
            "operator",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "replayScenarioId",
                "idempotencyKey",
                "acknowledgementRecordingAllowed",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_retention_readback_ack_replay_digest_view",
            "auditor",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackReceiptHash",
                "acknowledgementHash",
                "priorGateDigest",
                "monotonicityCheckId",
            ]),
        ),
        local_view(
            "release_owner_retention_readback_ack_replay_denial_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "replayDenialId",
            ]),
        ),
        local_view(
            "runtime_retention_readback_ack_replay_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "replayRecorded",
                "acknowledgementRecorded",
                "retentionStatePersisted",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_field_ids(),
        required_for_replay_scenario_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_scenario_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_invariants()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayInvariantPreview>
{
    vec![
        invariant(
            "retention_readback_ack_replay_requires_durable_identity_evidence",
            "retention readback acknowledgement replay requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "retention_readback_ack_replay_is_idempotent",
            "duplicate readback, duplicate acknowledgement, and stale digest replay cannot change state",
        ),
        invariant(
            "retention_readback_ack_replay_keeps_zero_side_effects",
            "replay must preserve zero writes, zero traffic, zero release, and zero external sends",
        ),
        invariant(
            "retention_readback_ack_replay_requires_acknowledgement_gate",
            "replay idempotency requires the retention readback acknowledgement gate",
        ),
        invariant(
            "retention_readback_ack_replay_is_scope_bound",
            "cross-scope and superseded acknowledgement replay cannot unlock effect application",
        ),
        invariant(
            "retention_readback_ack_replay_views_are_local_only",
            "operator, auditor, release-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "retention_readback_ack_replay_preview_has_no_side_effects",
            "this gate cannot persist replay records, record acknowledgement, grant authority, publish, or send externally",
        ),
    ]
}

fn replay_scenario(
    id: &'static str,
    source_acknowledgement_ids: Vec<&'static str>,
    replay_mode: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayScenarioPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayScenarioPreview {
        id,
        source_acknowledgement_ids,
        replay_mode,
        required_fields:
            with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(
                vec![
                    "replayScenarioId",
                    "sourceAcknowledgementIds",
                    "replayMode",
                    "zeroMutationProofHash",
                ],
            ),
        acknowledgement_recording_allowed: false,
        mutation_allowed: false,
    }
}

fn with_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn idempotency_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckIdempotencyGuardPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckIdempotencyGuardPreview {
        id,
        required_fields,
        blocks_replay_mutation: true,
    }
}

fn replay_denial(
    id: &'static str,
    applies_to_replay_scenario_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayDenialPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayDenialPreview {
        id,
        applies_to_replay_scenario_ids,
        reason,
        blocks_acknowledgement_recording: true,
        blocks_acceptance: true,
        blocks_mutation: true,
    }
}

fn monotonicity_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckMonotonicityCheckPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckMonotonicityCheckPreview {
        id,
        compared_fields,
        blocks_out_of_order_replay: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayLocalViewPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayInvariantPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            retention_state_persisted: false,
            readback_receipt_persisted: false,
            readback_acknowledgement_recorded: false,
            replay_recorded: false,
            operator_acceptance_recorded: false,
            approval_recorded: false,
            authority_granted: false,
            live_persistence_enabled: false,
            wal_written: false,
            checkpoint_written: false,
            enforcement_enabled: false,
            rollout_started: false,
            traffic_routed: false,
            release_published: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retention_readback_ack_replay_declares_blocked_scenarios() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.replay_scenario_count, 6);
        assert_eq!(
            report
                .replay_scenarios
                .iter()
                .map(|scenario| scenario.id)
                .collect::<Vec<_>>(),
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_scenario_ids()
        );
        assert!(report.replay_scenarios.iter().all(|scenario| {
            !scenario.acknowledgement_recording_allowed
                && !scenario.mutation_allowed
                && scenario.source_acknowledgement_ids.len() == 6
                && scenario.required_fields.contains(&"workflow_id")
                && scenario.required_fields.contains(&"receipt_hash")
                && scenario.required_fields.len() >= 11
        }));
    }

    #[test]
    fn retention_readback_ack_replay_requires_idempotency_guards() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.idempotency_guard_count, 7);
        assert!(report.idempotency_guards.iter().all(|guard| {
            guard.blocks_replay_mutation
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
                && guard.required_fields.len() >= 10
        }));
        assert!(
            report
                .idempotency_guards
                .iter()
                .any(|guard| guard.id == "replay_does_not_unlock_retention_or_acceptance")
        );
    }

    #[test]
    fn retention_readback_ack_replay_denies_every_mutating_outcome() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.replay_denial_count, 8);
        assert!(report.replay_denials.iter().all(|denial| {
            denial.blocks_acknowledgement_recording
                && denial.blocks_acceptance
                && denial.blocks_mutation
                && denial.applies_to_replay_scenario_ids.len() == 6
        }));
        assert!(
            report
                .replay_denials
                .iter()
                .any(|denial| denial.id == "durable_identity_evidence_missing")
        );
        assert!(report.replay_denials.iter().any(|denial| {
            denial.id == "replayed_retention_acknowledgement_cannot_send_external_delivery"
        }));
    }

    #[test]
    fn retention_readback_ack_replay_enforces_monotonicity() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.monotonicity_check_count, 5);
        assert!(report.monotonicity_checks.iter().all(|check| {
            check.blocks_out_of_order_replay
                && check.compared_fields.contains(&"workflow_id")
                && check.compared_fields.contains(&"receipt_hash")
                && check.compared_fields.len() >= 10
        }));
    }

    #[test]
    fn retention_readback_ack_replay_requires_acknowledgement_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_replay_durable_identity_field_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview
        );
    }

    #[test]
    fn retention_readback_ack_replay_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert!(
            report
                .local_views
                .iter()
                .all(|view| !view.external_delivery_enabled
                    && view.required_fields.contains(&"workflow_id")
                    && view.required_fields.contains(&"receipt_hash")
                    && view.required_fields.len() >= 11)
        );
        assert_eq!(report.invariant_count, 7);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(report.invariants.iter().any(|invariant| invariant.id
            == "retention_readback_ack_replay_requires_durable_identity_evidence"));
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckReplayPreviewSideEffects::none()
        );
    }
}
