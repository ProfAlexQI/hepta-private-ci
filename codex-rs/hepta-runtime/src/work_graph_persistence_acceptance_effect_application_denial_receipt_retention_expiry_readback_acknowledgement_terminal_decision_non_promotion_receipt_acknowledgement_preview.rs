use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewReport
{
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub acknowledgement_contract_count: usize,
    pub non_acceptance_reason_count: usize,
    pub recording_denial_count: usize,
    pub expiry_replay_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub acknowledgement_contracts: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview,
    >,
    pub non_acceptance_reasons: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview,
    >,
    pub recording_denials: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview,
    >,
    pub expiry_replay_guards: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview,
    >,
    pub local_views: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview,
    >,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementDurableIdentityEvidencePreview,
    pub invariants: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview,
    >,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview
{
    pub id: &'static str,
    pub source_receipt_ids: Vec<&'static str>,
    pub acknowledgement_visibility: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acknowledgement_recording_allowed: bool,
    pub acceptance_allowed: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview
{
    pub id: &'static str,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview
{
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_release_publication: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview
{
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_acknowledgement_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementDurableIdentityEvidencePreview
{
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_acknowledgement_ids: Vec<&'static str>,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_persisted: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
    pub terminal_decision_receipt_acknowledgement_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewReport
{
    let acknowledgement_contracts =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_contracts();
    let non_acceptance_reasons =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_non_acceptance_reasons();
    let recording_denials =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_recording_denials();
    let expiry_replay_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_expiry_replay_guards();
    let local_views =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_evidence();
    let invariants =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_invariants();

    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_replay_guard_count: expiry_replay_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_replay_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = crate::work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_required_prior_gates();
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate",
    );
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_source_receipt_ids()
-> Vec<&'static str> {
    vec![
        "operator_terminal_non_promotion_decision_receipt",
        "release_owner_terminal_non_promotion_decision_receipt",
        "authority_denial_terminal_non_promotion_receipt",
        "rollout_denial_terminal_non_promotion_receipt",
        "release_publication_denial_terminal_non_promotion_receipt",
        "external_delivery_denial_terminal_non_promotion_receipt",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_contracts()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview,
>{
    let receipt_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_source_receipt_ids();

    vec![
        acknowledgement_contract(
            "operator_terminal_non_promotion_receipt_acknowledgement",
            receipt_ids.clone(),
            "local_operator_receipt_acknowledgement",
        ),
        acknowledgement_contract(
            "release_owner_terminal_non_promotion_receipt_acknowledgement",
            receipt_ids.clone(),
            "local_release_owner_receipt_acknowledgement",
        ),
        acknowledgement_contract(
            "authority_denial_receipt_acknowledgement",
            receipt_ids.clone(),
            "local_authority_denial_receipt_acknowledgement",
        ),
        acknowledgement_contract(
            "rollout_denial_receipt_acknowledgement",
            receipt_ids.clone(),
            "local_rollout_denial_receipt_acknowledgement",
        ),
        acknowledgement_contract(
            "release_publication_denial_receipt_acknowledgement",
            receipt_ids.clone(),
            "local_release_publication_denial_receipt_acknowledgement",
        ),
        acknowledgement_contract(
            "external_delivery_denial_receipt_acknowledgement",
            receipt_ids,
            "local_external_delivery_denial_receipt_acknowledgement",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_non_acceptance_reasons()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview,
>{
    vec![
        non_acceptance(
            "durable_identity_evidence_missing",
            "terminal non-promotion receipt acknowledgement lacks durable identity evidence",
        ),
        non_acceptance(
            "terminal_non_promotion_receipt_ack_is_not_acceptance",
            "terminal non-promotion receipt acknowledgement is not acceptance",
        ),
        non_acceptance(
            "terminal_non_promotion_receipt_ack_is_not_approval",
            "terminal non-promotion receipt acknowledgement is not approval",
        ),
        non_acceptance(
            "terminal_non_promotion_receipt_ack_is_not_authority",
            "terminal non-promotion receipt acknowledgement is not authority",
        ),
        non_acceptance(
            "terminal_non_promotion_receipt_ack_is_not_live_persistence",
            "terminal non-promotion receipt acknowledgement is not live persistence",
        ),
        non_acceptance(
            "terminal_non_promotion_receipt_ack_is_not_rollout",
            "terminal non-promotion receipt acknowledgement is not rollout",
        ),
        non_acceptance(
            "terminal_non_promotion_receipt_ack_is_not_release_publication",
            "terminal non-promotion receipt acknowledgement is not release publication",
        ),
        non_acceptance(
            "terminal_non_promotion_receipt_ack_is_not_external_delivery",
            "terminal non-promotion receipt acknowledgement is not external delivery",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_recording_denials()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview,
>{
    let acknowledgement_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_ids();

    vec![
        recording_denial(
            "deny_durable_identity_terminal_receipt_ack_recording",
            acknowledgement_ids.clone(),
            "terminal non-promotion receipt acknowledgement recording is blocked without durable identity evidence",
        ),
        recording_denial(
            "duplicate_receipt_acknowledgement_cannot_record",
            acknowledgement_ids.clone(),
            "duplicate terminal non-promotion receipt acknowledgement cannot record acknowledgement",
        ),
        recording_denial(
            "missing_receipt_hash_cannot_record_acknowledgement",
            acknowledgement_ids.clone(),
            "missing receipt hash cannot record acknowledgement",
        ),
        recording_denial(
            "stale_terminal_decision_digest_cannot_record_acknowledgement",
            acknowledgement_ids.clone(),
            "stale terminal decision digest cannot record acknowledgement",
        ),
        recording_denial(
            "superseded_receipt_scope_cannot_record_acknowledgement",
            acknowledgement_ids.clone(),
            "superseded receipt scope cannot record acknowledgement",
        ),
        recording_denial(
            "cross_scope_receipt_acknowledgement_cannot_record",
            acknowledgement_ids.clone(),
            "cross-scope receipt acknowledgement cannot record acknowledgement",
        ),
        recording_denial(
            "release_owner_receipt_acknowledgement_cannot_publish",
            acknowledgement_ids.clone(),
            "release-owner receipt acknowledgement cannot publish release state",
        ),
        recording_denial(
            "external_delivery_receipt_acknowledgement_cannot_send",
            acknowledgement_ids,
            "external delivery receipt acknowledgement cannot send externally",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_expiry_replay_guards()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview,
>{
    vec![
        expiry_replay_guard(
            "terminal_receipt_acknowledgement_expiry_guard",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "expiresAt",
                "receiptHash",
            ]),
        ),
        expiry_replay_guard(
            "terminal_receipt_acknowledgement_scope_guard",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "receiptScope",
                "retentionScope",
                "scopeHash",
            ]),
        ),
        expiry_replay_guard(
            "terminal_receipt_acknowledgement_digest_guard",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "terminalDecisionHash",
                "receiptHash",
                "acknowledgementHash",
            ]),
        ),
        expiry_replay_guard(
            "terminal_receipt_acknowledgement_sequence_guard",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "receiptSequence",
                "acknowledgementSequence",
                "sequenceHash",
            ]),
        ),
        expiry_replay_guard(
            "terminal_receipt_acknowledgement_replay_guard",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "priorGateDigest",
                "replayWindowHash",
                "zeroEffectHash",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_local_views()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview,
>{
    vec![
        local_view(
            "operator_terminal_receipt_acknowledgement_view",
            "operator",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "receiptHash",
                "acceptanceAllowed",
                "nextGate",
            ]),
        ),
        local_view(
            "release_owner_terminal_receipt_acknowledgement_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "releasePublished",
                "publicClaimRecorded",
                "acknowledgementRecorded",
                "externalDeliveryDenied",
            ]),
        ),
        local_view(
            "auditor_terminal_receipt_acknowledgement_digest_view",
            "auditor",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "priorReceiptGateDigest",
                "acknowledgementHash",
                "recordingDenialId",
                "expiryReplayGuardId",
            ]),
        ),
        local_view(
            "runtime_terminal_receipt_acknowledgement_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "terminalDecisionReceiptAcknowledgementRecorded",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementDurableIdentityEvidencePreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_field_ids(),
        required_for_acknowledgement_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_invariants()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview,
>{
    vec![
        invariant(
            "terminal_receipt_acknowledgements_require_durable_identity_evidence",
            "terminal non-promotion receipt acknowledgements require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_receipt_acknowledgement_is_not_acceptance",
            "terminal non-promotion receipt acknowledgement cannot become acceptance",
        ),
        invariant(
            "terminal_receipt_acknowledgement_is_not_recorded",
            "terminal non-promotion receipt acknowledgement cannot be recorded",
        ),
        invariant(
            "terminal_receipt_acknowledgement_requires_receipt_gate",
            "terminal non-promotion receipt acknowledgement requires receipt evidence first",
        ),
        invariant(
            "terminal_receipt_acknowledgement_keeps_release_and_rollout_denied",
            "release publication, public claim, rollout, and traffic routing remain denied",
        ),
        invariant(
            "terminal_receipt_acknowledgement_views_are_local_only",
            "operator, release-owner, auditor, and runtime acknowledgement views cannot be sent externally",
        ),
        invariant(
            "terminal_receipt_acknowledgement_preview_has_no_side_effects",
            "this gate cannot record acknowledgement, persist state, grant authority, publish, or send externally",
        ),
    ]
}

fn acknowledgement_contract(
    id: &'static str,
    source_receipt_ids: Vec<&'static str>,
    acknowledgement_visibility: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview {
        id,
        source_receipt_ids,
        acknowledgement_visibility,
        required_fields:
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(
                vec![
                    "acknowledgementId",
                    "sourceReceiptIds",
                    "acknowledgementVisibility",
                    "acknowledgementHash",
                    "recordingAllowed",
                    "nextGate",
                ],
            ),
        acknowledgement_recording_allowed: false,
        acceptance_allowed: false,
        external_delivery_enabled: false,
    }
}

fn with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn non_acceptance(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview {
        id,
        reason,
        blocks_acceptance: true,
        blocks_authority: true,
    }
}

fn recording_denial(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview {
        id,
        applies_to_acknowledgement_ids,
        reason,
        blocks_acknowledgement_recording: true,
        blocks_acceptance: true,
        blocks_authority: true,
        blocks_release_publication: true,
        blocks_external_delivery: true,
    }
}

fn expiry_replay_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview {
        id,
        required_fields,
        blocks_acknowledgement_recording: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_persisted: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_persisted: false,
            terminal_decision_receipt_acknowledgement_recorded: false,
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
    fn terminal_receipt_acknowledgement_declares_non_accepting_contracts() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(report.acknowledgement_contract_count, 6);
        assert_eq!(
            report
                .acknowledgement_contracts
                .iter()
                .map(|contract| contract.id)
                .collect::<Vec<_>>(),
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_ids()
        );
        assert!(report.acknowledgement_contracts.iter().all(|contract| {
            !contract.acknowledgement_recording_allowed
                && !contract.acceptance_allowed
                && !contract.external_delivery_enabled
                && contract.source_receipt_ids.len() == 6
                && contract.required_fields.contains(&"workflow_id")
                && contract.required_fields.contains(&"receipt_hash")
                && contract.required_fields.len() >= 13
        }));
    }

    #[test]
    fn terminal_receipt_acknowledgement_blocks_acceptance_and_authority() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(report.non_acceptance_reason_count, 8);
        assert!(
            report
                .non_acceptance_reasons
                .iter()
                .all(|reason| reason.blocks_acceptance && reason.blocks_authority)
        );
        assert!(
            report
                .non_acceptance_reasons
                .iter()
                .any(|reason| reason.id == "durable_identity_evidence_missing")
        );
    }

    #[test]
    fn terminal_receipt_acknowledgement_denies_recording_and_delivery() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(report.recording_denial_count, 8);
        assert!(report.recording_denials.iter().all(|denial| {
            denial.blocks_acknowledgement_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_release_publication
                && denial.blocks_external_delivery
                && denial.applies_to_acknowledgement_ids.len() == 6
        }));
        assert!(
            report.recording_denials.iter().any(|denial| {
                denial.id == "deny_durable_identity_terminal_receipt_ack_recording"
            })
        );
    }

    #[test]
    fn terminal_receipt_acknowledgement_guards_expiry_scope_and_replay() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(report.expiry_replay_guard_count, 5);
        assert!(report.expiry_replay_guards.iter().all(|guard| {
            guard.blocks_acknowledgement_recording
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
                && guard.required_fields.len() >= 10
        }));
    }

    #[test]
    fn terminal_receipt_acknowledgement_requires_receipt_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_field_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview
        );
    }

    #[test]
    fn terminal_receipt_acknowledgement_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

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
            == "terminal_receipt_acknowledgements_require_durable_identity_evidence"));
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects::none()
        );
    }
}
