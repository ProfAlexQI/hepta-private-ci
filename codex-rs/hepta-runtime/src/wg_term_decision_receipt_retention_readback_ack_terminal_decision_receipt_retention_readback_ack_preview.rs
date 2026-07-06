use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementPreviewReport
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
    pub acknowledgement_contracts:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementContractPreview>,
    pub non_acceptance_reasons:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementNonAcceptancePreview>,
    pub recording_denials:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementRecordingDenialPreview>,
    pub expiry_replay_guards:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview>,
    pub local_views:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementContractPreview
{
    pub id: &'static str,
    pub source_readback_receipt_ids: Vec<&'static str>,
    pub acknowledgement_visibility: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acknowledgement_recording_allowed: bool,
    pub acceptance_allowed: bool,
    pub approval_recording_allowed: bool,
    pub authority_grant_allowed: bool,
    pub public_claim_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementNonAcceptancePreview
{
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
    pub blocks_approval: bool,
    pub blocks_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementRecordingDenialPreview
{
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub target_record: &'static str,
    pub reason: &'static str,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_receipt_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview
{
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub blocks_acknowledgement_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview
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
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
    pub terminal_decision_receipt_acknowledgement_recorded: bool,
    pub retention_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub receipt_acknowledgement_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report()
-> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementPreviewReport{
    let acknowledgement_contracts =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_contracts();
    let non_acceptance_reasons =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_non_acceptance_reasons();
    let recording_denials =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_recording_denials();
    let expiry_replay_guards =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_expiry_replay_guards();
    let local_views =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_local_views();
    let durable_identity_evidence =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_evidence();
    let invariants =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_invariants();

    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_replay_guard_count: expiry_replay_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_replay_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        crate::work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_required_prior_gates();
    gates.retain(|gate| *gate != "hepta_work_graph_durable_identity_preview_gate");
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate",
    );
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_ids()
-> Vec<&'static str> {
    vec![
        "operator_terminal_decision_receipt_retention_readback_acknowledgement",
        "auditor_terminal_decision_receipt_retention_readback_acknowledgement",
        "release_owner_terminal_decision_receipt_retention_readback_acknowledgement",
        "authority_denial_terminal_decision_receipt_retention_readback_acknowledgement",
        "public_claim_denial_terminal_decision_receipt_retention_readback_acknowledgement",
        "external_delivery_denial_terminal_decision_receipt_retention_readback_acknowledgement",
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_field_ids()
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

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_contracts()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementContractPreview,
>{
    let receipt_ids = crate::work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids();
    vec![
        acknowledgement_contract(
            "operator_terminal_decision_receipt_retention_readback_acknowledgement",
            receipt_ids.clone(),
            "local_operator_retention_readback_receipt_visibility",
        ),
        acknowledgement_contract(
            "auditor_terminal_decision_receipt_retention_readback_acknowledgement",
            receipt_ids.clone(),
            "local_auditor_retention_digest_visibility",
        ),
        acknowledgement_contract(
            "release_owner_terminal_decision_receipt_retention_readback_acknowledgement",
            receipt_ids.clone(),
            "local_release_owner_denial_visibility",
        ),
        acknowledgement_contract(
            "authority_denial_terminal_decision_receipt_retention_readback_acknowledgement",
            receipt_ids.clone(),
            "local_authority_denial_visibility",
        ),
        acknowledgement_contract(
            "public_claim_denial_terminal_decision_receipt_retention_readback_acknowledgement",
            receipt_ids.clone(),
            "local_public_claim_denial_visibility",
        ),
        acknowledgement_contract(
            "external_delivery_denial_terminal_decision_receipt_retention_readback_acknowledgement",
            receipt_ids,
            "local_external_delivery_denial_visibility",
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_non_acceptance_reasons()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementNonAcceptancePreview,
>{
    let acknowledgement_ids =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_ids();
    vec![
        non_acceptance(
            "durable_identity_evidence_missing",
            acknowledgement_ids.clone(),
            "terminal decision receipt retention readback acknowledgement does not include durable identity evidence",
        ),
        non_acceptance(
            "terminal_decision_receipt_retention_readback_ack_is_not_acceptance",
            acknowledgement_ids.clone(),
            "readback acknowledgement is local visibility, not acceptance",
        ),
        non_acceptance(
            "terminal_decision_receipt_retention_readback_ack_cannot_record_receipt",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot record the readback receipt",
        ),
        non_acceptance(
            "terminal_decision_receipt_retention_readback_ack_cannot_record_approval",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot record approval",
        ),
        non_acceptance(
            "terminal_decision_receipt_retention_readback_ack_cannot_grant_authority",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot grant authority",
        ),
        non_acceptance(
            "terminal_decision_receipt_retention_readback_ack_cannot_enable_live_persistence",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot enable live persistence, WAL, or checkpoints",
        ),
        non_acceptance(
            "terminal_decision_receipt_retention_readback_ack_cannot_start_rollout",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot start rollout or route traffic",
        ),
        non_acceptance(
            "terminal_decision_receipt_retention_readback_ack_cannot_publish_or_send",
            acknowledgement_ids,
            "readback acknowledgement cannot publish, record public claims, or send externally",
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_recording_denials()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementRecordingDenialPreview,
>{
    let acknowledgement_ids =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_ids();
    vec![
        recording_denial(
            "deny_durable_identity_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_recording",
            acknowledgement_ids.clone(),
            "durable_identity_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_evidence",
            "terminal decision receipt retention readback acknowledgement recording is blocked without durable identity evidence",
        ),
        recording_denial(
            "terminal_decision_receipt_retention_readback_ack_recording_denied",
            acknowledgement_ids.clone(),
            "acknowledgement_record",
            "acknowledgement recording is denied",
        ),
        recording_denial(
            "terminal_decision_receipt_retention_readback_receipt_recording_denied",
            acknowledgement_ids.clone(),
            "readback_receipt_record",
            "readback receipt recording remains denied",
        ),
        recording_denial(
            "terminal_decision_receipt_retention_readback_acceptance_recording_denied",
            acknowledgement_ids.clone(),
            "operator_acceptance_record",
            "operator acceptance recording is denied",
        ),
        recording_denial(
            "terminal_decision_receipt_retention_readback_approval_recording_denied",
            acknowledgement_ids.clone(),
            "approval_ledger_record",
            "approval recording is denied",
        ),
        recording_denial(
            "terminal_decision_receipt_retention_readback_authority_recording_denied",
            acknowledgement_ids.clone(),
            "authority_grant_record",
            "authority grant recording is denied",
        ),
        recording_denial(
            "terminal_decision_receipt_retention_readback_release_public_claim_recording_denied",
            acknowledgement_ids.clone(),
            "release_public_claim_record",
            "release publication and public claim recording are denied",
        ),
        recording_denial(
            "terminal_decision_receipt_retention_readback_external_delivery_recording_denied",
            acknowledgement_ids,
            "external_delivery_record",
            "external delivery recording and send are denied",
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_expiry_replay_guards()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview,
>{
    let acknowledgement_ids =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_ids();
    vec![
        expiry_replay_guard(
            "terminal_decision_receipt_retention_readback_receipt_expired_before_ack",
            acknowledgement_ids.clone(),
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "readbackReceiptId",
                "expiresAt",
                "observedAt",
            ]),
        ),
        expiry_replay_guard(
            "terminal_decision_receipt_retention_readback_scope_superseded_before_ack",
            acknowledgement_ids.clone(),
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "receiptScope",
                "scopeEpoch",
                "supersessionHash",
            ]),
        ),
        expiry_replay_guard(
            "terminal_decision_receipt_retention_readback_digest_mismatch_before_ack",
            acknowledgement_ids.clone(),
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "readbackReceiptHash",
                "acknowledgementHash",
                "priorGateDigest",
            ]),
        ),
        expiry_replay_guard(
            "terminal_decision_receipt_retention_readback_ack_replay_detected",
            acknowledgement_ids.clone(),
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "idempotencyKey",
                "replayHash",
            ]),
        ),
        expiry_replay_guard(
            "terminal_decision_receipt_retention_readback_cross_scope_ack_detected",
            acknowledgement_ids,
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "readbackReceiptScope",
                "acknowledgementScope",
                "scopeBindingHash",
            ]),
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_local_views()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementLocalViewPreview,
>{
    vec![
        local_view(
            "operator_terminal_decision_receipt_retention_readback_ack_view",
            "operator",
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "readbackReceiptHash",
                "acceptanceAllowed",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_terminal_decision_receipt_retention_readback_ack_digest_view",
            "auditor",
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "readbackReceiptHash",
                "acknowledgementHash",
                "priorGateDigest",
                "recordingDenialId",
            ]),
        ),
        local_view(
            "release_owner_terminal_decision_receipt_retention_readback_ack_denial_view",
            "release_owner",
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "releasePublished",
                "publicClaimRecorded",
                "authorityGranted",
                "externalDeliveryDenied",
            ]),
        ),
        local_view(
            "runtime_terminal_decision_receipt_retention_readback_ack_zero_effect_view",
            "system",
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementRecorded",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_evidence()
-> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_field_ids(),
        required_for_acknowledgement_ids:
            work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_invariants()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementInvariantPreview,
>{
    vec![
        invariant(
            "terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgements_require_durable_identity_evidence",
            "terminal decision receipt retention readback acknowledgement contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_is_hash_only",
            "acknowledgement visibility exposes hash-only local receipt evidence",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_is_not_acceptance",
            "acknowledgement does not become acceptance, approval, or authority",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_is_not_recorded",
            "acknowledgement and readback receipt recording remain disabled",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_views_are_local_only",
            "operator, auditor, release-owner, and runtime views stay local only",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_requires_readback_receipt_gate",
            "acknowledgement preview requires the retention readback receipt gate",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_preview_has_no_side_effects",
            "this gate cannot persist, write WAL/checkpoints, start rollout, publish, record public claims, or send externally",
        ),
    ]
}

impl
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementPreviewSideEffects
{
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_persisted: false,
            terminal_decision_receipt_acknowledgement_recorded: false,
            retention_state_persisted: false,
            readback_receipt_persisted: false,
            receipt_acknowledgement_recorded: false,
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

fn acknowledgement_contract(
    id: &'static str,
    source_readback_receipt_ids: Vec<&'static str>,
    acknowledgement_visibility: &'static str,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementContractPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementContractPreview {
        id,
        source_readback_receipt_ids,
        acknowledgement_visibility,
        required_fields:
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "readbackReceiptHash",
                "acknowledgementHash",
                "retentionScope",
                "acceptanceAllowed",
                "recordingEnabled",
                "nextGate",
            ]),
        acknowledgement_recording_allowed: false,
        acceptance_allowed: false,
        approval_recording_allowed: false,
        authority_grant_allowed: false,
        public_claim_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn non_acceptance(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementNonAcceptancePreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementNonAcceptancePreview {
        id,
        applies_to_acknowledgement_ids,
        reason,
        blocks_acceptance: true,
        blocks_approval: true,
        blocks_authority: true,
    }
}

fn recording_denial(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    target_record: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementRecordingDenialPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementRecordingDenialPreview {
        id,
        applies_to_acknowledgement_ids,
        target_record,
        reason,
        blocks_acknowledgement_recording: true,
        blocks_receipt_recording: true,
        blocks_acceptance: true,
        blocks_authority: true,
        blocks_release_publication: true,
        blocks_public_claim: true,
        blocks_external_delivery: true,
    }
}

fn expiry_replay_guard(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview {
        id,
        applies_to_acknowledgement_ids,
        required_fields,
        blocks_acknowledgement_recording: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementLocalViewPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementInvariantPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_declares_non_accepting_contracts()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.acknowledgement_contract_count, 6);
        assert!(report.acknowledgement_contracts.iter().all(|contract| {
            !contract.acknowledgement_recording_allowed
                && !contract.acceptance_allowed
                && !contract.approval_recording_allowed
                && !contract.authority_grant_allowed
                && contract.required_fields.len() >= 14
                && contract.required_fields.contains(&"workflow_id")
                && contract.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_blocks_acceptance_and_authority()
    {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.non_acceptance_reason_count, 8);
        assert!(
            report
                .non_acceptance_reasons
                .iter()
                .all(|reason| reason.blocks_acceptance
                    && reason.blocks_approval
                    && reason.blocks_authority)
        );
        assert_eq!(
            report
                .non_acceptance_reasons
                .first()
                .map(|reason| reason.id),
            Some("durable_identity_evidence_missing")
        );
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_denies_recording_and_delivery() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.recording_denial_count, 8);
        assert!(report.recording_denials.iter().all(
            |denial| denial.blocks_acknowledgement_recording
                && denial.blocks_receipt_recording
                && denial.blocks_external_delivery
        ));
        assert_eq!(
            report.recording_denials.first().map(|denial| denial.id),
            Some(
                "deny_durable_identity_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_recording"
            )
        );
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_guards_expiry_scope_and_replay()
    {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.expiry_replay_guard_count, 5);
        assert!(report.expiry_replay_guards.iter().all(|guard| {
            guard.blocks_acknowledgement_recording
                && guard.required_fields.len() >= 10
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_requires_readback_receipt_gate()
    {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert!(matches!(
            report.required_prior_gates.as_slice(),
            [
                ..,
                "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate"
            ]
        ));
        assert_eq!(report.recommended_next_gate, WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE);
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_requires_durable_identity_evidence()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

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
            work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_acknowledgement_ids,
            work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_has_no_side_effects() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert!(
            report
                .local_views
                .iter()
                .all(|view| !view.external_delivery_enabled
                    && view.required_fields.len() >= 11
                    && view.required_fields.contains(&"workflow_id")
                    && view.required_fields.contains(&"receipt_hash"))
        );
        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.invariants.first().map(|invariant| invariant.id),
            Some(
                "terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgements_require_durable_identity_evidence"
            )
        );
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAcknowledgementPreviewSideEffects::none()
        );
    }
}
