use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewReport
{
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_receipt_count: usize,
    pub digest_check_count: usize,
    pub mismatch_denial_count: usize,
    pub receipt_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub readback_receipts:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview>,
    pub digest_checks:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview>,
    pub mismatch_denials:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview>,
    pub receipt_guards:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview>,
    pub local_views:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview
{
    pub id: &'static str,
    pub source_retention_surface: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub persistence_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview
{
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_receipt_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview
{
    pub id: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_receipt_recording: bool,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_persistence: bool,
    pub blocks_authority: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview
{
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDurableIdentityEvidencePreview
{
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_readback_receipt_ids: Vec<&'static str>,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_persisted: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report()
-> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewReport
{
    let readback_receipts =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipts();
    let digest_checks =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_digest_checks();
    let mismatch_denials =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_mismatch_denials();
    let receipt_guards =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_guards();
    let local_views =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_local_views();
    let durable_identity_evidence =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_evidence();
    let invariants =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_invariants();

    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_no_receipt_write",
        readback_receipt_count: readback_receipts.len(),
        digest_check_count: digest_checks.len(),
        mismatch_denial_count: mismatch_denials.len(),
        receipt_guard_count: receipt_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_required_prior_gates(),
        readback_receipts,
        digest_checks,
        mismatch_denials,
        receipt_guards,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_RECEIPT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects::none(),
    }
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        crate::work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_required_prior_gates();
    gates.retain(|gate| *gate != "hepta_work_graph_durable_identity_preview_gate");
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate",
    );
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids()
-> Vec<&'static str> {
    vec![
        "terminal_decision_receipt_retention_policy_readback_receipt",
        "terminal_decision_receipt_expiry_guard_readback_receipt",
        "terminal_decision_receipt_supersession_guard_readback_receipt",
        "terminal_decision_receipt_gc_denial_readback_receipt",
        "terminal_decision_receipt_zero_effect_digest_readback_receipt",
        "terminal_decision_receipt_release_public_claim_denial_readback_receipt",
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids()
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

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipts()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview,
>{
    vec![
        readback_receipt(
            "terminal_decision_receipt_retention_policy_readback_receipt",
            "terminal_decision_receipt_retention_policies",
        ),
        readback_receipt(
            "terminal_decision_receipt_expiry_guard_readback_receipt",
            "terminal_decision_receipt_retention_expiry_guards",
        ),
        readback_receipt(
            "terminal_decision_receipt_supersession_guard_readback_receipt",
            "terminal_decision_receipt_retention_supersession_guards",
        ),
        readback_receipt(
            "terminal_decision_receipt_gc_denial_readback_receipt",
            "terminal_decision_receipt_garbage_collection_denials",
        ),
        readback_receipt(
            "terminal_decision_receipt_zero_effect_digest_readback_receipt",
            "terminal_decision_receipt_retention_zero_effect_digests",
        ),
        readback_receipt(
            "terminal_decision_receipt_release_public_claim_denial_readback_receipt",
            "terminal_decision_receipt_release_public_claim_external_delivery_denials",
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_digest_checks()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview,
>{
    vec![
        digest_check(
            "check_durable_identity_digest",
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids(),
        ),
        digest_check(
            "check_terminal_decision_receipt_retention_policy_digest",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["retentionPolicyIds", "retentionWindowHash", "hashOnly"],
            ),
        ),
        digest_check(
            "check_terminal_decision_receipt_expiry_guard_digest",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["expiryGuardIds", "expired", "blocksPersistence"],
            ),
        ),
        digest_check(
            "check_terminal_decision_receipt_supersession_digest",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["supersessionGuardIds", "scopeEpochHash", "blocksMutation"],
            ),
        ),
        digest_check(
            "check_terminal_decision_receipt_gc_denial_digest",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "garbageCollectionDenialIds",
                    "gcAllowed",
                    "tombstonePersisted",
                ],
            ),
        ),
        digest_check(
            "check_terminal_decision_receipt_zero_effect_digest",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"],
            ),
        ),
        digest_check(
            "check_terminal_decision_receipt_prior_gate_digest",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["priorGateId", "priorGateDigest", "readbackReceiptHash"],
            ),
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_mismatch_denials()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview,
>{
    let receipt_ids =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids();

    vec![
        mismatch_denial(
            "durable_identity_evidence_missing",
            receipt_ids.clone(),
            "terminal decision receipt retention readback receipt is missing durable identity evidence",
        ),
        mismatch_denial(
            "missing_terminal_decision_receipt_retention_policy_digest",
            receipt_ids.clone(),
            "terminal decision receipt retention readback is missing policy digest",
        ),
        mismatch_denial(
            "expired_terminal_decision_receipt_replayed",
            receipt_ids.clone(),
            "expired terminal decision receipt acknowledgement evidence was replayed",
        ),
        mismatch_denial(
            "superseded_terminal_decision_receipt_scope_replayed",
            receipt_ids.clone(),
            "superseded terminal decision receipt scope was replayed",
        ),
        mismatch_denial(
            "terminal_decision_receipt_gc_tombstone_persistence_attempted",
            receipt_ids.clone(),
            "terminal decision receipt retention readback attempted to persist a GC tombstone",
        ),
        mismatch_denial(
            "terminal_decision_receipt_zero_effect_digest_nonzero",
            receipt_ids.clone(),
            "terminal decision receipt retention readback does not prove zero side effects",
        ),
        mismatch_denial(
            "terminal_decision_receipt_public_claim_attempted",
            receipt_ids.clone(),
            "terminal decision receipt retention readback cannot record public claims",
        ),
        mismatch_denial(
            "terminal_decision_receipt_external_delivery_attempted",
            receipt_ids,
            "terminal decision receipt retention readback cannot send external delivery",
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_guards()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview,
>{
    vec![
        receipt_guard(
            "hash_only_terminal_decision_receipt_retention_receipt_required",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["retentionPolicyHash", "expiryGuardHash", "supersessionHash"],
            ),
        ),
        receipt_guard(
            "non_persistent_terminal_decision_receipt_readback_required",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "persistenceEnabled",
                    "receiptPersisted",
                    "tombstonePersisted",
                ],
            ),
        ),
        receipt_guard(
            "terminal_decision_receipt_local_view_only_required",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "operatorViewHash",
                    "auditorViewHash",
                    "releaseOwnerViewHash",
                ],
            ),
        ),
        receipt_guard(
            "terminal_decision_receipt_bounded_retention_window_required",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["retentionWindow", "expiryState", "scopeEpoch"],
            ),
        ),
        receipt_guard(
            "terminal_decision_receipt_next_gate_acknowledgement_required",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "recommendedNextGate",
                    "acknowledgementAllowed",
                    "acceptanceAllowed",
                ],
            ),
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_local_views()
-> Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview>
{
    vec![
        local_view(
            "operator_terminal_decision_receipt_retention_readback_receipt_view",
            "operator",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "readbackReceiptId",
                    "retentionPolicyId",
                    "expired",
                    "nextGate",
                ],
            ),
        ),
        local_view(
            "auditor_terminal_decision_receipt_retention_readback_digest_view",
            "auditor",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "readbackReceiptHash",
                    "retentionPolicyHash",
                    "gcDenialHash",
                    "zeroEffectHash",
                ],
            ),
        ),
        local_view(
            "release_owner_terminal_decision_receipt_retention_readback_denial_view",
            "release_owner",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "releaseDenied",
                    "publicationDenied",
                    "publicClaimDenied",
                    "externalDeliveryDenied",
                ],
            ),
        ),
        local_view(
            "runtime_terminal_decision_receipt_retention_readback_zero_effect_view",
            "system",
            with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "retentionStatePersisted",
                    "readbackReceiptPersisted",
                    "authorityGranted",
                    "publicClaimRecorded",
                    "externalSendPerformed",
                ],
            ),
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_evidence()
-> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDurableIdentityEvidencePreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids(),
        required_for_readback_receipt_ids:
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_invariants()
-> Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview>
{
    vec![
        invariant(
            "terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipts_require_durable_identity_evidence",
            "terminal decision receipt retention readback receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_receipts_are_hash_only",
            "terminal decision receipt retention readback receipts contain hash-only redacted evidence",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_receipts_are_non_persistent",
            "terminal decision receipt retention readback cannot write receipt, retention, expiry, or tombstone state",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_receipts_block_acceptance",
            "terminal decision receipt retention readback cannot become acceptance or approval recording",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_receipts_block_gc_mutation",
            "terminal decision receipt retention readback cannot perform garbage collection or persist tombstones",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_views_are_local_only",
            "operator, auditor, release-owner, and runtime terminal decision receipt readback views cannot be sent externally",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_preview_has_no_side_effects",
            "this gate cannot persist, grant authority, enable live execution, publish, record public claims, or send externally",
        ),
    ]
}

fn readback_receipt(
    id: &'static str,
    source_retention_surface: &'static str,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreview {
        id,
        source_retention_surface,
        required_fields: with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
            vec![
                "readbackReceiptId",
                "sourceRetentionSurface",
                "retentionPolicyHash",
                "expiryGuardHash",
                "supersessionHash",
                "garbageCollectionDenialHash",
                "zeroEffectHash",
                "nextGate",
            ],
        ),
        redaction_state: "hash_only_redacted",
        persistence_enabled: false,
        external_delivery_enabled: false,
    }
}

fn digest_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackDigestCheckPreview {
        id,
        compared_fields,
        blocks_receipt_acceptance: true,
    }
}

fn mismatch_denial(
    id: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackMismatchDenialPreview {
        id,
        applies_to_receipt_ids,
        reason,
        blocks_receipt_recording: true,
        blocks_acknowledgement_recording: true,
        blocks_acceptance: true,
        blocks_persistence: true,
        blocks_authority: true,
        blocks_rollout: true,
        blocks_release_publication: true,
        blocks_public_claim: true,
        blocks_external_delivery: true,
    }
}

fn receipt_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptGuardPreview {
        id,
        required_fields,
        blocks_recording: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn with_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut required =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids();
    required.extend(fields);
    required
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_persisted: false,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_declares_hash_only_receipts() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.readback_receipt_count, 6);
        assert_eq!(
            report
                .readback_receipts
                .iter()
                .map(|receipt| receipt.id)
                .collect::<Vec<_>>(),
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids()
        );
        assert!(report.readback_receipts.iter().all(|receipt| {
            receipt.redaction_state == "hash_only_redacted"
                && !receipt.persistence_enabled
                && !receipt.external_delivery_enabled
                && receipt.required_fields.len() >= 15
                && receipt.required_fields.contains(&"workflow_id")
                && receipt.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_checks_digests_and_mismatches() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.digest_check_count, 7);
        assert!(report.digest_checks.iter().all(|check| {
            check.blocks_receipt_acceptance
                && check.compared_fields.len() >= 7
                && check.compared_fields.contains(&"workflow_id")
                && check.compared_fields.contains(&"receipt_hash")
        }));
        assert_eq!(report.mismatch_denial_count, 8);
        assert_eq!(
            report.mismatch_denials.first().map(|denial| denial.id),
            Some("durable_identity_evidence_missing")
        );
        assert!(report.mismatch_denials.iter().all(|denial| {
            denial.blocks_receipt_recording
                && denial.blocks_acknowledgement_recording
                && denial.blocks_acceptance
                && denial.blocks_persistence
                && denial.blocks_authority
                && denial.blocks_rollout
                && denial.blocks_release_publication
                && denial.blocks_public_claim
                && denial.blocks_external_delivery
                && denial.applies_to_receipt_ids.len() == 6
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_requires_non_recording_guards() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.receipt_guard_count, 5);
        assert!(report.receipt_guards.iter().all(|guard| {
            guard.blocks_recording
                && guard.required_fields.len() >= 10
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_requires_retention_expiry_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        let prior_tail = &report.required_prior_gates[report.required_prior_gates.len() - 2..];
        assert_eq!(
            prior_tail,
            [
                "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate"
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
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_readback_receipt_ids,
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert!(report.ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview);
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert!(report.local_views.iter().all(|view| {
            !view.external_delivery_enabled
                && view.required_fields.len() >= 11
                && view.required_fields.contains(&"workflow_id")
                && view.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_report();

        assert_eq!(report.invariant_count, 7);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(report.invariants.iter().any(|invariant| {
            invariant.id
                == "terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipts_require_durable_identity_evidence"
        }));
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackReceiptPreviewSideEffects::none()
        );
    }
}
