use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementPreviewReport
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
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementContractPreview,
    >,
    pub non_acceptance_reasons: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementNonAcceptancePreview,
    >,
    pub recording_denials: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementRecordingDenialPreview,
    >,
    pub expiry_replay_guards: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview,
    >,
    pub local_views: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementLocalViewPreview,
    >,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview,
    pub invariants: Vec<
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementInvariantPreview,
    >,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementContractPreview
{
    pub id: &'static str,
    pub source_readback_receipt_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acceptance_allowed: bool,
    pub acknowledgement_recording_enabled: bool,
    pub receipt_recording_enabled: bool,
    pub authority_grant_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementNonAcceptancePreview
{
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementRecordingDenialPreview
{
    pub id: &'static str,
    pub target_record: &'static str,
    pub reason: &'static str,
    pub blocks_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview
{
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub blocks_acknowledgement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview
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
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub retention_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub readback_acknowledgement_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_report()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementPreviewReport {
    let acknowledgement_contracts =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_contracts();
    let non_acceptance_reasons =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_non_acceptance_reasons();
    let recording_denials =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_recording_denials();
    let expiry_replay_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_expiry_replay_guards();
    let local_views =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_evidence();
    let invariants =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_invariants();

    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_replay_guard_count: expiry_replay_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_replay_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_required_prior_gates()
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_contracts()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementContractPreview>
{
    vec![
        acknowledgement_contract(
            "retention_policy_readback_receipt_acknowledgement",
            "retention_policy_readback_receipt",
        ),
        acknowledgement_contract(
            "expiry_guard_readback_receipt_acknowledgement",
            "expiry_guard_readback_receipt",
        ),
        acknowledgement_contract(
            "supersession_guard_readback_receipt_acknowledgement",
            "supersession_guard_readback_receipt",
        ),
        acknowledgement_contract(
            "garbage_collection_denial_readback_receipt_acknowledgement",
            "garbage_collection_denial_readback_receipt",
        ),
        acknowledgement_contract(
            "zero_effect_digest_readback_receipt_acknowledgement",
            "zero_effect_digest_readback_receipt",
        ),
        acknowledgement_contract(
            "release_external_denial_readback_receipt_acknowledgement",
            "release_external_denial_readback_receipt",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_non_acceptance_reasons()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementNonAcceptancePreview>
{
    let acknowledgement_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_ids();

    vec![
        non_acceptance_reason(
            "durable_identity_evidence_missing",
            acknowledgement_ids.clone(),
            "retention readback acknowledgement does not include durable identity evidence",
        ),
        non_acceptance_reason(
            "readback_acknowledgement_is_not_retention_acceptance",
            acknowledgement_ids.clone(),
            "readback acknowledgement only confirms local preview visibility",
        ),
        non_acceptance_reason(
            "readback_acknowledgement_cannot_record_receipt_or_acknowledgement",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot record receipt or acknowledgement state",
        ),
        non_acceptance_reason(
            "readback_acknowledgement_cannot_record_approval",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot record approval or acceptance",
        ),
        non_acceptance_reason(
            "readback_acknowledgement_cannot_grant_authority",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot grant WorkGraph authority",
        ),
        non_acceptance_reason(
            "readback_acknowledgement_cannot_enable_persistence_or_wal",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot enable live persistence, WAL, or checkpoints",
        ),
        non_acceptance_reason(
            "readback_acknowledgement_cannot_start_rollout",
            acknowledgement_ids.clone(),
            "readback acknowledgement cannot start rollout or route traffic",
        ),
        non_acceptance_reason(
            "readback_acknowledgement_cannot_publish_or_send",
            acknowledgement_ids,
            "readback acknowledgement cannot publish release state or send externally",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_recording_denials()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementRecordingDenialPreview>
{
    vec![
        recording_denial(
            "deny_durable_identity_readback_ack_recording",
            "durable_identity_readback_acknowledgement_evidence",
            "readback acknowledgement recording is blocked without durable identity evidence",
        ),
        recording_denial(
            "retention_readback_acknowledgement_recording_denied",
            "retention_readback_acknowledgement_store",
            "retention readback acknowledgement recording is disabled in preview",
        ),
        recording_denial(
            "retention_state_recording_denied",
            "retention_state_store",
            "readback acknowledgement cannot persist retention state",
        ),
        recording_denial(
            "readback_receipt_recording_denied",
            "retention_readback_receipt_store",
            "readback acknowledgement cannot persist receipt state",
        ),
        recording_denial(
            "operator_acceptance_recording_denied",
            "operator_acceptance_record",
            "readback acknowledgement is not operator acceptance",
        ),
        recording_denial(
            "approval_ledger_recording_denied",
            "approval_ledger",
            "readback acknowledgement cannot write approval ledger entries",
        ),
        recording_denial(
            "authority_grant_recording_denied",
            "authority_grant_record",
            "readback acknowledgement cannot grant authority",
        ),
        recording_denial(
            "release_external_recording_denied",
            "release_publication_external_delivery_record",
            "readback acknowledgement cannot publish release state or create delivery records",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_expiry_replay_guards()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview>
{
    let acknowledgement_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_ids();

    vec![
        expiry_replay_guard(
            "retention_readback_receipt_expired",
            acknowledgement_ids.clone(),
            "retention readback receipt exceeded the local preview window",
        ),
        expiry_replay_guard(
            "retention_readback_receipt_scope_superseded",
            acknowledgement_ids.clone(),
            "retention readback receipt scope was superseded by a newer blocker report",
        ),
        expiry_replay_guard(
            "retention_readback_receipt_digest_mismatch",
            acknowledgement_ids.clone(),
            "retention readback receipt digest does not match local readback evidence",
        ),
        expiry_replay_guard(
            "retention_garbage_collection_denial_receipt_replayed",
            acknowledgement_ids.clone(),
            "garbage-collection denial readback receipt replay was observed",
        ),
        expiry_replay_guard(
            "readback_acknowledgement_replay_detected",
            acknowledgement_ids,
            "readback acknowledgement idempotency key has already been observed",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_local_views()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementLocalViewPreview>
{
    vec![
        local_view(
            "operator_retention_readback_acknowledgement_view",
            "operator",
            with_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "sourceReadbackReceiptId",
                "accepted",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_retention_readback_acknowledgement_view",
            "auditor",
            with_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementHash",
                "sourceReadbackReceiptHash",
                "scopeDigest",
                "zeroEffectHash",
            ]),
        ),
        local_view(
            "release_owner_retention_readback_acknowledgement_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "acknowledgementId",
            ]),
        ),
        local_view(
            "runtime_retention_readback_acknowledgement_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_fields(vec![
                "acknowledgementRecorded",
                "retentionStatePersisted",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_field_ids(),
        required_for_acknowledgement_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_invariants()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementInvariantPreview>
{
    vec![
        invariant(
            "retention_readback_acknowledgements_require_durable_identity_evidence",
            "retention readback acknowledgement contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "retention_readback_acknowledgements_are_hash_only",
            "acknowledgements expose only local hash-only readback receipt references",
        ),
        invariant(
            "retention_readback_acknowledgements_are_non_accepting",
            "retention readback acknowledgement visibility cannot become acceptance",
        ),
        invariant(
            "retention_readback_acknowledgements_are_non_recording",
            "acknowledgement preview cannot record receipt, approval, acceptance, authority, or retention state",
        ),
        invariant(
            "retention_readback_acknowledgement_views_are_local_only",
            "operator, auditor, release-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "retention_readback_acknowledgement_requires_readback_receipt_gate",
            "acknowledgement preview requires retention expiry readback receipt evidence first",
        ),
        invariant(
            "retention_readback_acknowledgement_preview_has_no_side_effects",
            "this gate cannot persist, grant authority, enable live execution, publish, or send externally",
        ),
    ]
}

fn acknowledgement_contract(
    id: &'static str,
    source_readback_receipt_id: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementContractPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementContractPreview {
        id,
        source_readback_receipt_id,
        required_fields:
            with_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_fields(
                vec![
                    "acknowledgementId",
                    "sourceReadbackReceiptId",
                    "readbackReceiptHash",
                    "retentionScope",
                    "acknowledgementHash",
                    "accepted",
                    "recordingEnabled",
                    "nextGate",
                ],
            ),
        acceptance_allowed: false,
        acknowledgement_recording_enabled: false,
        receipt_recording_enabled: false,
        authority_grant_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn non_acceptance_reason(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementNonAcceptancePreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementNonAcceptancePreview {
        id,
        applies_to_acknowledgement_ids,
        reason,
        blocks_acceptance: true,
    }
}

fn recording_denial(
    id: &'static str,
    target_record: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementRecordingDenialPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementRecordingDenialPreview {
        id,
        target_record,
        reason,
        blocks_recording: true,
    }
}

fn expiry_replay_guard(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementExpiryReplayGuardPreview {
        id,
        applies_to_acknowledgement_ids,
        trigger,
        blocks_acknowledgement: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementLocalViewPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementInvariantPreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            retention_state_persisted: false,
            readback_receipt_persisted: false,
            readback_acknowledgement_recorded: false,
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
    fn retention_readback_acknowledgement_declares_non_accepting_contracts() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.acknowledgement_contract_count, 6);
        assert_eq!(
            report.acknowledgement_contracts.len(),
            report.acknowledgement_contract_count
        );
        assert_eq!(
            report
                .acknowledgement_contracts
                .iter()
                .map(|contract| contract.id)
                .collect::<Vec<_>>(),
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_ids()
        );
        assert!(report.acknowledgement_contracts.iter().all(|contract| {
            !contract.acceptance_allowed
                && !contract.acknowledgement_recording_enabled
                && !contract.receipt_recording_enabled
                && !contract.authority_grant_enabled
                && !contract.external_delivery_enabled
                && contract.required_fields.contains(&"workflow_id")
                && contract.required_fields.contains(&"receipt_hash")
                && contract.required_fields.len() >= 15
        }));
    }

    #[test]
    fn retention_readback_acknowledgement_blocks_acceptance_and_recording() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.non_acceptance_reason_count, 8);
        assert!(report.non_acceptance_reasons.iter().all(|reason| {
            reason.blocks_acceptance
                && reason.applies_to_acknowledgement_ids.len() == 6
                && reason.reason.contains("acknowledgement")
        }));
        assert!(
            report
                .non_acceptance_reasons
                .iter()
                .any(|reason| reason.id == "durable_identity_evidence_missing")
        );
        assert_eq!(report.recording_denial_count, 8);
        assert!(
            report
                .recording_denials
                .iter()
                .all(|denial| denial.blocks_recording)
        );
        assert!(
            report
                .recording_denials
                .iter()
                .any(|denial| denial.target_record
                    == "durable_identity_readback_acknowledgement_evidence")
        );
        assert!(
            report
                .recording_denials
                .iter()
                .any(|denial| denial.target_record == "approval_ledger")
        );
    }

    #[test]
    fn retention_readback_acknowledgement_guards_expiry_scope_and_replay() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(report.expiry_replay_guard_count, 5);
        assert!(report.expiry_replay_guards.iter().all(|guard| {
            guard.blocks_acknowledgement && guard.applies_to_acknowledgement_ids.len() == 6
        }));
        assert!(
            report
                .expiry_replay_guards
                .iter()
                .any(|guard| guard.id == "readback_acknowledgement_replay_detected")
        );
    }

    #[test]
    fn retention_readback_acknowledgement_requires_readback_receipt_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_acknowledgement_durable_identity_field_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview
        );
    }

    #[test]
    fn retention_readback_acknowledgement_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_report();

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
            == "retention_readback_acknowledgements_require_durable_identity_evidence"));
    }

    #[test]
    fn retention_readback_acknowledgement_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_report();

        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAcknowledgementPreviewSideEffects::none()
        );
    }
}
