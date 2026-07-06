use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewReport
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
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview>,
    pub non_acceptance_reasons:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview>,
    pub recording_denials:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview>,
    pub expiry_replay_guards:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview>,
    pub local_views:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview
{
    pub id: &'static str,
    pub source_receipt_ids: Vec<&'static str>,
    pub acknowledgement_visibility: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acknowledgement_recording_allowed: bool,
    pub acceptance_allowed: bool,
    pub authority_grant_allowed: bool,
    pub public_claim_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview
{
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview
{
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub target_record: &'static str,
    pub reason: &'static str,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview
{
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub blocks_acknowledgement_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementDurableIdentityEvidencePreview
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
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report()
-> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewReport{
    let acknowledgement_contracts =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_contracts();
    let non_acceptance_reasons =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_non_acceptance_reasons();
    let recording_denials =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_recording_denials();
    let expiry_replay_guards =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_expiry_replay_guards();
    let local_views =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_local_views();
    let durable_identity_evidence =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_evidence();
    let invariants =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_invariants();

    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_replay_guard_count: expiry_replay_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_replay_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        crate::work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_required_prior_gates();
    gates.retain(|gate| *gate != "hepta_work_graph_durable_identity_preview_gate");
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate",
    );
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_field_ids()
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

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_ids()
-> Vec<&'static str> {
    vec![
        "operator_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
        "release_owner_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
        "authority_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
        "rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
        "release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
        "external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_contracts()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview,
>{
    let source_receipt_ids =
        crate::work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_ids();
    vec![
        acknowledgement_contract(
            "operator_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_operator_receipt_acknowledgement_visibility",
        ),
        acknowledgement_contract(
            "release_owner_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_release_owner_receipt_acknowledgement_visibility",
        ),
        acknowledgement_contract(
            "authority_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_authority_denial_receipt_acknowledgement_visibility",
        ),
        acknowledgement_contract(
            "rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_rollout_denial_receipt_acknowledgement_visibility",
        ),
        acknowledgement_contract(
            "release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
            source_receipt_ids.clone(),
            "local_release_publication_denial_receipt_acknowledgement_visibility",
        ),
        acknowledgement_contract(
            "external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
            source_receipt_ids,
            "external_delivery_acknowledgement_echo_denied",
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_non_acceptance_reasons()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview,
>{
    let acknowledgement_ids =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_ids();

    vec![
        non_acceptance_reason(
            "durable_identity_evidence_missing",
            acknowledgement_ids.clone(),
            "terminal decision receipt acknowledgement cannot proceed without durable identity evidence",
        ),
        non_acceptance_reason(
            "terminal_decision_receipt_acknowledgement_is_not_acceptance",
            acknowledgement_ids.clone(),
            "terminal decision receipt acknowledgement only confirms local preview visibility",
        ),
        non_acceptance_reason(
            "terminal_decision_receipt_acknowledgement_cannot_record_receipt",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot record terminal decision receipt state",
        ),
        non_acceptance_reason(
            "terminal_decision_receipt_acknowledgement_cannot_record_approval",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot record approval or acceptance",
        ),
        non_acceptance_reason(
            "terminal_decision_receipt_acknowledgement_cannot_grant_authority",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot grant WorkGraph authority",
        ),
        non_acceptance_reason(
            "terminal_decision_receipt_acknowledgement_cannot_enable_live_persistence",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot enable live persistence, WAL, or checkpoints",
        ),
        non_acceptance_reason(
            "terminal_decision_receipt_acknowledgement_cannot_start_rollout",
            acknowledgement_ids.clone(),
            "receipt acknowledgement cannot start rollout or route traffic",
        ),
        non_acceptance_reason(
            "terminal_decision_receipt_acknowledgement_cannot_publish_or_send",
            acknowledgement_ids,
            "receipt acknowledgement cannot publish release/public claims or send externally",
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_recording_denials()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview,
>{
    let acknowledgement_ids =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_ids();

    vec![
        recording_denial(
            "deny_durable_identity_terminal_receipt_retention_readback_ack_terminal_decision_receipt_ack_recording",
            acknowledgement_ids.clone(),
            "terminal_decision_receipt_acknowledgement_store",
            "terminal decision receipt acknowledgement recording is blocked without durable identity evidence",
        ),
        recording_denial(
            "terminal_decision_receipt_acknowledgement_recording_denied",
            acknowledgement_ids.clone(),
            "terminal_decision_receipt_acknowledgement_store",
            "terminal decision receipt acknowledgement recording is disabled in preview",
        ),
        recording_denial(
            "terminal_decision_receipt_recording_denied_after_acknowledgement",
            acknowledgement_ids.clone(),
            "terminal_decision_receipt_store",
            "acknowledgement cannot persist terminal decision receipt state",
        ),
        recording_denial(
            "operator_acceptance_recording_denied_after_terminal_decision_receipt_ack",
            acknowledgement_ids.clone(),
            "operator_acceptance_record",
            "receipt acknowledgement is not operator acceptance",
        ),
        recording_denial(
            "approval_ledger_recording_denied_after_terminal_decision_receipt_ack",
            acknowledgement_ids.clone(),
            "approval_ledger",
            "receipt acknowledgement cannot write approval ledger entries",
        ),
        recording_denial(
            "authority_grant_recording_denied_after_terminal_decision_receipt_ack",
            acknowledgement_ids.clone(),
            "authority_grant_record",
            "receipt acknowledgement cannot grant authority",
        ),
        recording_denial(
            "release_public_claim_recording_denied_after_terminal_decision_receipt_ack",
            acknowledgement_ids.clone(),
            "release_publication_public_claim_record",
            "receipt acknowledgement cannot publish release or public claim state",
        ),
        recording_denial(
            "external_delivery_recording_denied_after_terminal_decision_receipt_ack",
            acknowledgement_ids,
            "external_delivery_record",
            "receipt acknowledgement cannot create delivery records",
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_expiry_replay_guards()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview,
>{
    let acknowledgement_ids =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_ids();

    vec![
        expiry_replay_guard(
            "terminal_decision_receipt_expired_before_acknowledgement",
            acknowledgement_ids.clone(),
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "receiptExpiresAt",
                "acknowledgementObservedAt",
                "receiptHash",
            ]),
        ),
        expiry_replay_guard(
            "terminal_decision_receipt_scope_superseded_before_acknowledgement",
            acknowledgement_ids.clone(),
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "receiptScope",
                "supersedingScope",
                "scopeDigest",
            ]),
        ),
        expiry_replay_guard(
            "terminal_decision_receipt_digest_mismatch_before_acknowledgement",
            acknowledgement_ids.clone(),
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "receiptHash",
                "acknowledgementHash",
                "digestCheckId",
            ]),
        ),
        expiry_replay_guard(
            "terminal_decision_receipt_acknowledgement_replay_detected",
            acknowledgement_ids.clone(),
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "idempotencyKey",
                "priorObservationHash",
            ]),
        ),
        expiry_replay_guard(
            "terminal_decision_receipt_cross_scope_acknowledgement_detected",
            acknowledgement_ids,
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "receiptScope",
                "acknowledgementScope",
                "bindingHash",
            ]),
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_local_views()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview,
>{
    vec![
        local_view(
            "operator_terminal_decision_receipt_acknowledgement_view",
            "operator",
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "sourceReceiptId",
                "accepted",
                "nextGate",
            ]),
        ),
        local_view(
            "release_owner_terminal_decision_receipt_acknowledgement_view",
            "release_owner",
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "releasePublished",
                "publicClaimRecorded",
                "externalDeliveryDenied",
                "acknowledgementId",
            ]),
        ),
        local_view(
            "auditor_terminal_decision_receipt_acknowledgement_digest_view",
            "auditor",
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementHash",
                "sourceReceiptHash",
                "scopeDigest",
                "zeroEffectHash",
            ]),
        ),
        local_view(
            "runtime_terminal_decision_receipt_acknowledgement_zero_effect_view",
            "system",
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementRecorded",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_evidence()
-> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementDurableIdentityEvidencePreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_field_ids(),
        required_for_acknowledgement_ids:
            work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_invariants()
-> Vec<
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview,
>{
    vec![
        invariant(
            "terminal_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgements_require_durable_identity_evidence",
            "terminal decision receipt acknowledgements require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_decision_receipt_acknowledgements_are_hash_only",
            "acknowledgements expose only local hash-only terminal decision receipt references",
        ),
        invariant(
            "terminal_decision_receipt_acknowledgements_are_non_accepting",
            "terminal decision receipt acknowledgement visibility cannot become acceptance",
        ),
        invariant(
            "terminal_decision_receipt_acknowledgements_are_non_recording",
            "acknowledgement preview cannot record receipt, approval, acceptance, authority, or release state",
        ),
        invariant(
            "terminal_decision_receipt_acknowledgement_views_are_local_only",
            "operator, auditor, release-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "terminal_decision_receipt_acknowledgement_requires_receipt_gate",
            "acknowledgement preview requires terminal non-promotion receipt evidence first",
        ),
        invariant(
            "terminal_decision_receipt_acknowledgement_preview_has_no_side_effects",
            "this gate cannot persist, grant authority, enable live execution, publish, or send externally",
        ),
    ]
}

fn acknowledgement_contract(
    id: &'static str,
    source_receipt_ids: Vec<&'static str>,
    acknowledgement_visibility: &'static str,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementContractPreview {
        id,
        source_receipt_ids,
        acknowledgement_visibility,
        required_fields:
            with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(
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
        authority_grant_allowed: false,
        public_claim_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn non_acceptance_reason(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementNonAcceptancePreview {
        id,
        applies_to_acknowledgement_ids,
        reason,
        blocks_acceptance: true,
        blocks_authority: true,
    }
}

fn recording_denial(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    target_record: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementRecordingDenialPreview {
        id,
        applies_to_acknowledgement_ids,
        target_record,
        reason,
        blocks_acknowledgement_recording: true,
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
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementExpiryReplayGuardPreview {
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
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview
{
    WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects
{
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
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
    fn work_graph_terminal_decision_receipt_acknowledgement_requires_receipt_gate() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2)
                .copied(),
            Some(
                "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some("hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement_durable_identity_field_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview);
    }

    #[test]
    fn work_graph_terminal_decision_receipt_acknowledgement_declares_non_accepting_contracts() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(report.acknowledgement_contract_count, 6);
        assert!(report.acknowledgement_contracts.iter().all(|contract| {
            !contract.acknowledgement_recording_allowed
                && !contract.acceptance_allowed
                && !contract.authority_grant_allowed
                && !contract.public_claim_enabled
                && !contract.external_delivery_enabled
                && contract.source_receipt_ids.len() == 6
                && contract.required_fields.contains(&"workflow_id")
                && contract.required_fields.contains(&"receipt_hash")
                && contract.required_fields.len() >= 13
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_acknowledgement_blocks_acceptance_and_authority() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(report.non_acceptance_reason_count, 8);
        assert!(report.non_acceptance_reasons.iter().all(|reason| {
            reason.blocks_acceptance
                && reason.blocks_authority
                && reason.applies_to_acknowledgement_ids.len() == 6
        }));
        assert!(
            report
                .non_acceptance_reasons
                .iter()
                .any(|reason| reason.id == "durable_identity_evidence_missing")
        );
    }

    #[test]
    fn work_graph_terminal_decision_receipt_acknowledgement_denies_recording_and_delivery() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(report.recording_denial_count, 8);
        assert!(report.recording_denials.iter().all(|denial| {
            denial.blocks_acknowledgement_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_release_publication
                && denial.blocks_public_claim
                && denial.blocks_external_delivery
                && denial.applies_to_acknowledgement_ids.len() == 6
        }));
        assert!(report.recording_denials.iter().any(|denial| {
            denial.id
                == "deny_durable_identity_terminal_receipt_retention_readback_ack_terminal_decision_receipt_ack_recording"
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_acknowledgement_guards_expiry_scope_and_replay() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(report.expiry_replay_guard_count, 5);
        assert!(report.expiry_replay_guards.iter().all(|guard| {
            guard.blocks_acknowledgement_recording
                && guard.applies_to_acknowledgement_ids.len() == 6
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
                && guard.required_fields.len() >= 10
        }));
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
    }

    #[test]
    fn work_graph_terminal_decision_receipt_acknowledgement_has_no_side_effects() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report();

        assert_eq!(report.invariant_count, 7);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(report.invariants.iter().any(|invariant| invariant.id
            == "terminal_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgements_require_durable_identity_evidence"));
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptAcknowledgementPreviewSideEffects::none()
        );
    }
}
