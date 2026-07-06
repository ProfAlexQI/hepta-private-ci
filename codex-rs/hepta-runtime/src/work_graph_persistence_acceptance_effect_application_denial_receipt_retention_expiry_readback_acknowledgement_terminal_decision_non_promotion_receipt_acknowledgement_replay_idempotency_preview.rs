use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayPreviewReport
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
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayScenarioPreview>,
    pub idempotency_guards:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckIdempotencyGuardPreview>,
    pub replay_denials:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayDenialPreview>,
    pub monotonicity_checks:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckMonotonicityCheckPreview>,
    pub local_views:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayScenarioPreview
{
    pub id: &'static str,
    pub source_acknowledgement_ids: Vec<&'static str>,
    pub replay_mode: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acknowledgement_recording_allowed: bool,
    pub mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckIdempotencyGuardPreview
{
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_replay_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayDenialPreview
{
    pub id: &'static str,
    pub applies_to_replay_scenario_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckMonotonicityCheckPreview
{
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_out_of_order_replay: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayDurableIdentityEvidencePreview
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
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_persisted: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
    pub terminal_decision_receipt_acknowledgement_recorded: bool,
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
    pub public_claim_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_report()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayPreviewReport
{
    let replay_scenarios =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_scenarios();
    let idempotency_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_idempotency_guards();
    let replay_denials =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_denials();
    let monotonicity_checks =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_monotonicity_checks();
    let local_views =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_evidence();
    let invariants =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_invariants();

    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_no_replay_write",
        replay_scenario_count: replay_scenarios.len(),
        idempotency_guard_count: idempotency_guards.len(),
        replay_denial_count: replay_denials.len(),
        monotonicity_check_count: monotonicity_checks.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_required_prior_gates(),
        replay_scenarios,
        idempotency_guards,
        replay_denials,
        monotonicity_checks,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = crate::work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_required_prior_gates();
    gates.retain(|gate| *gate != "hepta_work_graph_durable_identity_preview_gate");
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate",
    );
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_scenario_ids()
-> Vec<&'static str> {
    vec![
        "duplicate_terminal_non_promotion_receipt_replay",
        "duplicate_terminal_non_promotion_receipt_acknowledgement_replay",
        "stale_terminal_non_promotion_receipt_digest_replay",
        "superseded_terminal_non_promotion_receipt_scope_replay",
        "cross_scope_terminal_non_promotion_receipt_acknowledgement_replay",
        "out_of_order_terminal_non_promotion_receipt_acknowledgement_replay",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_source_acknowledgement_ids()
-> Vec<&'static str> {
    vec![
        "operator_terminal_non_promotion_receipt_acknowledgement",
        "release_owner_terminal_non_promotion_receipt_acknowledgement",
        "authority_denial_receipt_acknowledgement",
        "rollout_denial_receipt_acknowledgement",
        "release_publication_denial_receipt_acknowledgement",
        "external_delivery_denial_receipt_acknowledgement",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_scenarios()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayScenarioPreview>
{
    let acknowledgement_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_source_acknowledgement_ids();

    vec![
        replay_scenario(
            "duplicate_terminal_non_promotion_receipt_replay",
            acknowledgement_ids.clone(),
            "duplicate_receipt",
        ),
        replay_scenario(
            "duplicate_terminal_non_promotion_receipt_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "duplicate_acknowledgement",
        ),
        replay_scenario(
            "stale_terminal_non_promotion_receipt_digest_replay",
            acknowledgement_ids.clone(),
            "stale_receipt_digest",
        ),
        replay_scenario(
            "superseded_terminal_non_promotion_receipt_scope_replay",
            acknowledgement_ids.clone(),
            "superseded_receipt_scope",
        ),
        replay_scenario(
            "cross_scope_terminal_non_promotion_receipt_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "cross_scope_acknowledgement",
        ),
        replay_scenario(
            "out_of_order_terminal_non_promotion_receipt_acknowledgement_replay",
            acknowledgement_ids,
            "out_of_order_acknowledgement",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_idempotency_guards()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckIdempotencyGuardPreview,
>{
    vec![
        idempotency_guard(
            "terminal_receipt_idempotency_key_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "receiptId",
                "receiptHash",
                "terminalDecisionHash",
            ]),
        ),
        idempotency_guard(
            "terminal_receipt_acknowledgement_idempotency_key_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "acknowledgementId",
                "acknowledgementHash",
                "localViewHash",
            ]),
        ),
        idempotency_guard(
            "terminal_receipt_prior_gate_digest_binding_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "priorGateId",
                "priorGateDigest",
                "sourceReportHash",
            ]),
        ),
        idempotency_guard(
            "terminal_receipt_scope_epoch_binding_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "receiptScope",
                "scopeEpoch",
                "supersessionHash",
            ]),
        ),
        idempotency_guard(
            "terminal_receipt_zero_side_effect_digest_binding_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
        idempotency_guard(
            "terminal_receipt_acknowledgement_sequence_required",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "receiptSequence",
                "acknowledgementSequence",
                "sequenceHash",
            ]),
        ),
        idempotency_guard(
            "replay_does_not_unlock_terminal_non_promotion_receipt",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "acknowledgementRecorded",
                "acceptanceAllowed",
                "authorityGranted",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_denials()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayDenialPreview>
{
    let replay_scenario_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_scenario_ids();

    vec![
        replay_denial(
            "durable_identity_evidence_missing",
            replay_scenario_ids.clone(),
            "terminal receipt acknowledgement replay cannot proceed without durable identity evidence",
        ),
        replay_denial(
            "duplicate_terminal_receipt_replay_cannot_record_acknowledgement",
            replay_scenario_ids.clone(),
            "duplicate terminal non-promotion receipt replay cannot record acknowledgement",
        ),
        replay_denial(
            "duplicate_terminal_receipt_acknowledgement_cannot_record_acceptance",
            replay_scenario_ids.clone(),
            "duplicate terminal receipt acknowledgement replay cannot record acceptance",
        ),
        replay_denial(
            "stale_terminal_receipt_digest_cannot_grant_authority",
            replay_scenario_ids.clone(),
            "stale terminal receipt digest replay cannot grant authority",
        ),
        replay_denial(
            "cross_scope_terminal_receipt_replay_cannot_enable_live_persistence",
            replay_scenario_ids.clone(),
            "cross-scope terminal receipt replay cannot enable live persistence, WAL, or checkpoints",
        ),
        replay_denial(
            "out_of_order_terminal_receipt_replay_cannot_start_rollout",
            replay_scenario_ids.clone(),
            "out-of-order terminal receipt replay cannot start rollout or route traffic",
        ),
        replay_denial(
            "superseded_terminal_receipt_replay_cannot_publish_release",
            replay_scenario_ids.clone(),
            "superseded terminal receipt replay cannot publish release status",
        ),
        replay_denial(
            "replayed_terminal_receipt_acknowledgement_cannot_send_external_delivery",
            replay_scenario_ids,
            "replayed terminal receipt acknowledgement cannot send external delivery",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_monotonicity_checks()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckMonotonicityCheckPreview,
>{
    vec![
        monotonicity_check(
            "terminal_receipt_sequence_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "receiptId",
                "receiptSequence",
                "receiptHash",
            ]),
        ),
        monotonicity_check(
            "terminal_receipt_acknowledgement_sequence_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "acknowledgementId",
                "acknowledgementSequence",
                "acknowledgementHash",
            ]),
        ),
        monotonicity_check(
            "terminal_receipt_prior_gate_digest_monotonicity_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "priorGateId",
                "priorGateDigest",
                "observedAt",
            ]),
        ),
        monotonicity_check(
            "terminal_receipt_scope_epoch_monotonicity_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "receiptScope",
                "scopeEpoch",
                "supersessionHash",
            ]),
        ),
        monotonicity_check(
            "terminal_receipt_zero_effect_digest_stability_check",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_local_views()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayLocalViewPreview>
{
    vec![
        local_view(
            "operator_terminal_receipt_ack_replay_idempotency_view",
            "operator",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "replayScenarioId",
                "idempotencyKey",
                "acknowledgementRecordingAllowed",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_terminal_receipt_ack_replay_digest_view",
            "auditor",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "receiptHash",
                "acknowledgementHash",
                "priorGateDigest",
                "monotonicityCheckId",
            ]),
        ),
        local_view(
            "release_owner_terminal_receipt_ack_replay_denial_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "replayDenialId",
            ]),
        ),
        local_view(
            "runtime_terminal_receipt_ack_replay_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(vec![
                "replayRecorded",
                "acknowledgementRecorded",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayDurableIdentityEvidencePreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_field_ids(),
        required_for_replay_scenario_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_scenario_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_invariants()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayInvariantPreview>
{
    vec![
        invariant(
            "terminal_receipt_ack_replay_requires_durable_identity_evidence",
            "terminal receipt acknowledgement replay requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_receipt_ack_replay_is_idempotent",
            "duplicate receipt, duplicate acknowledgement, and stale digest replay cannot change state",
        ),
        invariant(
            "terminal_receipt_ack_replay_keeps_zero_side_effects",
            "replay must preserve zero writes, zero traffic, zero release, and zero external sends",
        ),
        invariant(
            "terminal_receipt_ack_replay_requires_acknowledgement_gate",
            "replay idempotency requires the terminal non-promotion receipt acknowledgement gate",
        ),
        invariant(
            "terminal_receipt_ack_replay_is_scope_bound",
            "cross-scope and superseded acknowledgement replay cannot unlock receipt recording",
        ),
        invariant(
            "terminal_receipt_ack_replay_views_are_local_only",
            "operator, auditor, release-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "terminal_receipt_ack_replay_preview_has_no_side_effects",
            "this gate cannot persist replay records, record acknowledgement, grant authority, publish, or send externally",
        ),
    ]
}

fn replay_scenario(
    id: &'static str,
    source_acknowledgement_ids: Vec<&'static str>,
    replay_mode: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayScenarioPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayScenarioPreview {
        id,
        source_acknowledgement_ids,
        replay_mode,
        required_fields:
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(
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

fn with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn idempotency_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckIdempotencyGuardPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckIdempotencyGuardPreview {
        id,
        required_fields,
        blocks_replay_mutation: true,
    }
}

fn replay_denial(
    id: &'static str,
    applies_to_replay_scenario_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayDenialPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayDenialPreview {
        id,
        applies_to_replay_scenario_ids,
        reason,
        blocks_acknowledgement_recording: true,
        blocks_acceptance: true,
        blocks_authority: true,
        blocks_rollout: true,
        blocks_release_publication: true,
        blocks_external_delivery: true,
    }
}

fn monotonicity_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckMonotonicityCheckPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckMonotonicityCheckPreview {
        id,
        compared_fields,
        blocks_out_of_order_replay: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayLocalViewPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayInvariantPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_persisted: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_persisted: false,
            terminal_decision_receipt_acknowledgement_recorded: false,
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
            public_claim_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_receipt_ack_replay_declares_blocked_scenarios() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.replay_scenario_count, 6);
        assert_eq!(
            report
                .replay_scenarios
                .iter()
                .map(|scenario| scenario.id)
                .collect::<Vec<_>>(),
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_scenario_ids()
        );
        assert!(report.replay_scenarios.iter().all(|scenario| {
            !scenario.acknowledgement_recording_allowed
                && !scenario.mutation_allowed
                && scenario.source_acknowledgement_ids.len() == 6
                && scenario.required_fields.len() >= 11
                && scenario.required_fields.contains(&"workflow_id")
                && scenario.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn terminal_receipt_ack_replay_requires_idempotency_guards() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.idempotency_guard_count, 7);
        assert!(report.idempotency_guards.iter().all(|guard| {
            guard.blocks_replay_mutation
                && guard.required_fields.len() >= 10
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn terminal_receipt_ack_replay_denies_every_mutating_outcome() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.replay_denial_count, 8);
        assert!(
            report
                .replay_denials
                .iter()
                .any(|denial| denial.id == "durable_identity_evidence_missing")
        );
        assert!(report.replay_denials.iter().all(|denial| {
            denial.blocks_acknowledgement_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_rollout
                && denial.blocks_release_publication
                && denial.blocks_external_delivery
                && denial.applies_to_replay_scenario_ids.len() == 6
        }));
    }

    #[test]
    fn terminal_receipt_ack_replay_enforces_monotonicity() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.monotonicity_check_count, 5);
        assert!(report.monotonicity_checks.iter().all(|check| {
            check.blocks_out_of_order_replay
                && check.compared_fields.len() >= 10
                && check.compared_fields.contains(&"workflow_id")
                && check.compared_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn terminal_receipt_ack_replay_requires_acknowledgement_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_durable_identity_field_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview
        );
    }

    #[test]
    fn terminal_receipt_ack_replay_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert!(report.local_views.iter().all(|view| {
            !view.external_delivery_enabled
                && view.required_fields.len() >= 11
                && view.required_fields.contains(&"workflow_id")
                && view.required_fields.contains(&"receipt_hash")
        }));
        assert_eq!(report.invariant_count, 7);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(report.invariants.iter().any(|invariant| {
            invariant.id == "terminal_receipt_ack_replay_requires_durable_identity_evidence"
        }));
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAckReplayPreviewSideEffects::none()
        );
    }
}
