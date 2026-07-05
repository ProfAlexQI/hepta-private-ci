use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewReport
{
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub receipt_count: usize,
    pub digest_check_count: usize,
    pub mismatch_denial_count: usize,
    pub receipt_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub receipts:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreview>,
    pub digest_checks:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview>,
    pub mismatch_denials:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview>,
    pub receipt_guards:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview>,
    pub local_views:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreview
{
    pub id: &'static str,
    pub source_terminal_decision_surface_ids: Vec<&'static str>,
    pub receipt_hash_mode: &'static str,
    pub required_fields: Vec<&'static str>,
    pub persisted: bool,
    pub acceptance_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview
{
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_receipt_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview
{
    pub id: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_receipt_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_release_publication: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview
{
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub receipt_recording_allowed: bool,
    pub promotion_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptDurableIdentityEvidencePreview
{
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_receipt_ids: Vec<&'static str>,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_persisted: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewReport
{
    let receipts =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipts();
    let digest_checks =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_digest_checks();
    let mismatch_denials =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_mismatch_denials();
    let receipt_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_guards();
    let local_views =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_evidence();
    let invariants =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_invariants();

    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_hash_only_no_recording",
        receipt_count: receipts.len(),
        digest_check_count: digest_checks.len(),
        mismatch_denial_count: mismatch_denials.len(),
        receipt_guard_count: receipt_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_required_prior_gates(),
        receipts,
        digest_checks,
        mismatch_denials,
        receipt_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_required_prior_gates()
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
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate",
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate",
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_source_surface_ids()
-> Vec<&'static str> {
    vec![
        "operator_terminal_decision_visibility",
        "release_owner_terminal_decision_visibility",
        "auditor_terminal_decision_visibility",
        "rollback_owner_terminal_decision_visibility",
        "runtime_terminal_state_summary_visibility",
        "external_delivery_terminal_decision_echo",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipts()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreview>
{
    let surface_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_source_surface_ids();

    vec![
        receipt(
            "operator_terminal_non_promotion_decision_receipt",
            surface_ids.clone(),
            "hash_only_operator_terminal_decision_receipt",
        ),
        receipt(
            "release_owner_terminal_non_promotion_decision_receipt",
            surface_ids.clone(),
            "hash_only_release_owner_terminal_decision_receipt",
        ),
        receipt(
            "authority_denial_terminal_non_promotion_receipt",
            surface_ids.clone(),
            "hash_only_authority_denial_receipt",
        ),
        receipt(
            "rollout_denial_terminal_non_promotion_receipt",
            surface_ids.clone(),
            "hash_only_rollout_denial_receipt",
        ),
        receipt(
            "release_publication_denial_terminal_non_promotion_receipt",
            surface_ids.clone(),
            "hash_only_release_publication_denial_receipt",
        ),
        receipt(
            "external_delivery_denial_terminal_non_promotion_receipt",
            surface_ids,
            "hash_only_external_delivery_denial_receipt",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_digest_checks()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview,
>{
    vec![
        digest_check(
            "terminal_decision_surface_digest_matches",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "terminalDecisionSurfaceId",
                "terminalDecisionHash",
                "sourceGateDigest",
            ]),
        ),
        digest_check(
            "non_promotion_denial_digest_matches",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "nonPromotionDenialId",
                "denialHash",
                "zeroPromotionHash",
            ]),
        ),
        digest_check(
            "authority_guard_digest_matches",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "authorityGuardId",
                "authorityGuardHash",
                "authorityGranted",
            ]),
        ),
        digest_check(
            "release_delivery_guard_digest_matches",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "releaseDeliveryGuardId",
                "releaseHash",
                "deliveryHash",
            ]),
        ),
        digest_check(
            "local_view_digest_matches",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "localViewId",
                "localViewHash",
                "externalDeliveryEnabled",
            ]),
        ),
        digest_check(
            "zero_side_effect_digest_matches",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_mismatch_denials()
-> Vec<
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview,
>{
    let receipt_ids =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ids();

    vec![
        mismatch_denial(
            "durable_identity_evidence_missing",
            receipt_ids.clone(),
            "terminal decision non-promotion receipt cannot proceed without durable identity evidence",
        ),
        mismatch_denial(
            "missing_terminal_decision_surface_cannot_record_receipt",
            receipt_ids.clone(),
            "missing terminal decision surface cannot record receipt",
        ),
        mismatch_denial(
            "mismatched_terminal_decision_hash_cannot_accept",
            receipt_ids.clone(),
            "mismatched terminal decision hash cannot become acceptance",
        ),
        mismatch_denial(
            "stale_replay_idempotency_digest_cannot_grant_authority",
            receipt_ids.clone(),
            "stale replay idempotency digest cannot grant authority",
        ),
        mismatch_denial(
            "authority_guard_absence_cannot_start_rollout",
            receipt_ids.clone(),
            "absent authority guard cannot start rollout or route traffic",
        ),
        mismatch_denial(
            "release_delivery_guard_absence_cannot_publish",
            receipt_ids.clone(),
            "release and delivery guard absence cannot publish release state",
        ),
        mismatch_denial(
            "external_delivery_receipt_echo_cannot_send",
            receipt_ids.clone(),
            "external delivery receipt echo cannot send externally",
        ),
        mismatch_denial(
            "receipt_readback_is_not_live_completion",
            receipt_ids,
            "receipt readback cannot claim live persistence completion",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_guards()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview>
{
    vec![
        receipt_guard(
            "receipt_is_hash_only",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "receiptId",
                "receiptHash",
                "redactionHash",
            ]),
        ),
        receipt_guard(
            "receipt_is_non_persistent",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "receiptPersisted",
                "receiptStorageScope",
                "receiptWriteHash",
            ]),
        ),
        receipt_guard(
            "receipt_is_non_accepting",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "acceptanceAllowed",
                "approvalRecorded",
                "authorityGranted",
            ]),
        ),
        receipt_guard(
            "receipt_keeps_release_denied",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "releasePublished",
                "publicClaimRecorded",
                "artifactPublished",
            ]),
        ),
        receipt_guard(
            "receipt_keeps_external_delivery_denied",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "externalDeliveryEnabled",
                "destinationPolicy",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_local_views()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview>
{
    vec![
        local_view(
            "operator_terminal_non_promotion_receipt_view",
            "operator",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "receiptId",
                "receiptHash",
                "acceptanceAllowed",
                "nextGate",
            ]),
        ),
        local_view(
            "release_owner_terminal_non_promotion_receipt_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "releasePublished",
                "publicClaimRecorded",
                "receiptPersisted",
                "externalDeliveryDenied",
            ]),
        ),
        local_view(
            "auditor_terminal_non_promotion_receipt_digest_view",
            "auditor",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "sourceGateDigest",
                "receiptHash",
                "digestCheckId",
                "mismatchDenialId",
            ]),
        ),
        local_view(
            "runtime_terminal_non_promotion_receipt_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(vec![
                "terminalDecisionReceiptRecorded",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptDurableIdentityEvidencePreview
{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_field_ids(),
        required_for_receipt_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_invariants()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview>
{
    vec![
        invariant(
            "terminal_non_promotion_receipts_require_durable_identity_evidence",
            "terminal decision non-promotion receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_non_promotion_receipts_are_hash_only",
            "terminal decision non-promotion receipts expose hashes only",
        ),
        invariant(
            "terminal_non_promotion_receipts_are_not_recorded",
            "terminal decision receipt readback cannot record receipt state",
        ),
        invariant(
            "terminal_non_promotion_receipts_are_not_acceptance",
            "terminal decision receipt visibility cannot become acceptance or authority",
        ),
        invariant(
            "terminal_non_promotion_receipts_keep_release_denied",
            "release publication, public claim, rollout, and traffic routing remain denied",
        ),
        invariant(
            "terminal_non_promotion_receipt_views_are_local_only",
            "operator, release-owner, auditor, and runtime receipt views cannot be sent externally",
        ),
        invariant(
            "terminal_non_promotion_receipt_preview_has_no_side_effects",
            "this gate cannot persist receipts, record approval, grant authority, publish, or send externally",
        ),
    ]
}

fn receipt(
    id: &'static str,
    source_terminal_decision_surface_ids: Vec<&'static str>,
    receipt_hash_mode: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreview {
        id,
        source_terminal_decision_surface_ids,
        receipt_hash_mode,
        required_fields:
            with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(
                vec![
                    "receiptId",
                    "sourceTerminalDecisionSurfaceIds",
                    "receiptHashMode",
                    "receiptHash",
                    "redactionHash",
                ],
            ),
        persisted: false,
        acceptance_allowed: false,
    }
}

fn with_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn digest_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptDigestCheckPreview {
        id,
        compared_fields,
        blocks_receipt_recording: true,
    }
}

fn mismatch_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptMismatchDenialPreview {
        id,
        applies_to_receipt_ids,
        reason,
        blocks_receipt_recording: true,
        blocks_acceptance: true,
        blocks_authority: true,
        blocks_release_publication: true,
        blocks_external_delivery: true,
    }
}

fn receipt_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptGuardPreview {
        id,
        required_fields,
        receipt_recording_allowed: false,
        promotion_allowed: false,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview{
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_persisted: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_persisted: false,
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
    fn terminal_non_promotion_receipts_are_hash_only_and_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(report.receipt_count, 6);
        assert_eq!(
            report
                .receipts
                .iter()
                .map(|receipt| receipt.id)
                .collect::<Vec<_>>(),
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_ids()
        );
        assert!(report.receipts.iter().all(|receipt| {
            !receipt.persisted
                && !receipt.acceptance_allowed
                && receipt.source_terminal_decision_surface_ids.len() == 6
                && receipt.required_fields.contains(&"workflow_id")
                && receipt.required_fields.contains(&"receipt_hash")
                && receipt.required_fields.len() >= 12
        }));
    }

    #[test]
    fn terminal_non_promotion_receipts_require_digest_checks() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(report.digest_check_count, 6);
        assert!(report.digest_checks.iter().all(|check| {
            check.blocks_receipt_recording
                && check.compared_fields.contains(&"workflow_id")
                && check.compared_fields.contains(&"receipt_hash")
                && check.compared_fields.len() >= 10
        }));
    }

    #[test]
    fn terminal_non_promotion_receipts_deny_mismatched_readback() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(report.mismatch_denial_count, 8);
        assert!(report.mismatch_denials.iter().all(|denial| {
            denial.blocks_receipt_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_release_publication
                && denial.blocks_external_delivery
                && denial.applies_to_receipt_ids.len() == 6
        }));
        assert!(
            report
                .mismatch_denials
                .iter()
                .any(|denial| denial.id == "durable_identity_evidence_missing")
        );
    }

    #[test]
    fn terminal_non_promotion_receipts_guard_against_recording_and_promotion() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(report.receipt_guard_count, 5);
        assert!(report.receipt_guards.iter().all(|guard| {
            !guard.receipt_recording_allowed
                && !guard.promotion_allowed
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
                && guard.required_fields.len() >= 10
        }));
    }

    #[test]
    fn terminal_non_promotion_receipt_requires_terminal_decision_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_acceptance_effect_denial_receipt_retention_readback_ack_terminal_decision_receipt_durable_identity_field_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview
        );
    }

    #[test]
    fn terminal_non_promotion_receipt_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report();

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
            == "terminal_non_promotion_receipts_require_durable_identity_evidence"));
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewSideEffects::none()
        );
    }
}
