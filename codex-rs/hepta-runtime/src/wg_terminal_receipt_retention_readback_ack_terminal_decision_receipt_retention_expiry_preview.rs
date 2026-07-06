use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionExpiryPreviewReport
{
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub retention_policy_count: usize,
    pub expiry_guard_count: usize,
    pub supersession_guard_count: usize,
    pub garbage_collection_denial_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub retention_policies:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionPolicyPreview>,
    pub expiry_guards:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptExpiryGuardPreview>,
    pub supersession_guards:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptSupersessionGuardPreview>,
    pub garbage_collection_denials:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGarbageCollectionDenialPreview>,
    pub local_views:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionExpiryPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionPolicyPreview
{
    pub id: &'static str,
    pub scope: &'static str,
    pub retention_window: &'static str,
    pub required_fields: Vec<&'static str>,
    pub hash_only: bool,
    pub persistence_enabled: bool,
    pub garbage_collection_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptExpiryGuardPreview {
    pub id: &'static str,
    pub applies_to_policy_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub blocks_acceptance: bool,
    pub blocks_persistence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptSupersessionGuardPreview
{
    pub id: &'static str,
    pub supersedes: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGarbageCollectionDenialPreview
{
    pub id: &'static str,
    pub target: &'static str,
    pub reason: &'static str,
    pub garbage_collection_allowed: bool,
    pub blocks_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionDurableIdentityEvidencePreview
{
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_retention_policy_ids: Vec<&'static str>,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionExpiryPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_persisted: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
    pub terminal_decision_receipt_acknowledgement_recorded: bool,
    pub replay_recorded: bool,
    pub retention_state_persisted: bool,
    pub expiry_recorded: bool,
    pub garbage_collection_performed: bool,
    pub tombstone_persisted: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report()
-> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionExpiryPreviewReport {
    let retention_policies =
        work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_policies();
    let expiry_guards =
        work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_expiry_guards();
    let supersession_guards =
        work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_supersession_guards();
    let garbage_collection_denials =
        work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_garbage_collection_denials();
    let local_views =
        work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_local_views();
    let durable_identity_evidence =
        work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_evidence();
    let invariants =
        work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_invariants();

    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionExpiryPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_no_retention_write",
        retention_policy_count: retention_policies.len(),
        expiry_guard_count: expiry_guards.len(),
        supersession_guard_count: supersession_guards.len(),
        garbage_collection_denial_count: garbage_collection_denials.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_required_prior_gates(),
        retention_policies,
        expiry_guards,
        supersession_guards,
        garbage_collection_denials,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionExpiryPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        crate::work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_ack_replay_required_prior_gates();
    gates.retain(|gate| *gate != "hepta_work_graph_durable_identity_preview_gate");
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate",
    );
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_policy_ids()
-> Vec<&'static str> {
    vec![
        "terminal_decision_receipt_ack_replay_index_retention_policy",
        "terminal_decision_receipt_ack_local_view_retention_policy",
        "terminal_decision_receipt_ack_zero_effect_digest_retention_policy",
        "terminal_decision_receipt_ack_scope_epoch_retention_policy",
        "terminal_decision_receipt_ack_release_public_claim_denial_retention_policy",
        "terminal_decision_receipt_ack_gc_denial_retention_policy",
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_field_ids()
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

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_policies()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionPolicyPreview> {
    vec![
        retention_policy(
            "terminal_decision_receipt_ack_replay_index_retention_policy",
            "terminal_decision_receipt_acknowledgement_replay_index",
            "bounded_to_terminal_decision_receipt_ack_replay_window",
        ),
        retention_policy(
            "terminal_decision_receipt_ack_local_view_retention_policy",
            "operator_auditor_release_owner_terminal_decision_receipt_ack_views",
            "bounded_to_terminal_decision_receipt_visibility_window",
        ),
        retention_policy(
            "terminal_decision_receipt_ack_zero_effect_digest_retention_policy",
            "zero_write_zero_traffic_zero_release_zero_public_claim_zero_external_digest",
            "bounded_to_terminal_decision_zero_effect_digest_window",
        ),
        retention_policy(
            "terminal_decision_receipt_ack_scope_epoch_retention_policy",
            "terminal_decision_receipt_scope_epoch_and_supersession_markers",
            "bounded_to_current_terminal_decision_scope_epoch",
        ),
        retention_policy(
            "terminal_decision_receipt_ack_release_public_claim_denial_retention_policy",
            "release_publication_public_claim_external_delivery_denials",
            "bounded_to_release_owner_terminal_decision_receipt_window",
        ),
        retention_policy(
            "terminal_decision_receipt_ack_gc_denial_retention_policy",
            "terminal_decision_receipt_acknowledgement_garbage_collection_denials",
            "bounded_to_gc_denial_preview_window",
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_expiry_guards()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptExpiryGuardPreview> {
    let policy_ids =
        work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_policy_ids();

    vec![
        expiry_guard(
            "terminal_decision_receipt_ack_retention_window_expired",
            policy_ids.clone(),
            "terminal decision receipt acknowledgement replay retention window expired",
        ),
        expiry_guard(
            "terminal_decision_receipt_ack_scope_superseded",
            policy_ids.clone(),
            "terminal decision receipt scope was superseded by a newer denial receipt",
        ),
        expiry_guard(
            "terminal_decision_receipt_ack_prior_digest_expired",
            policy_ids.clone(),
            "prior gate digest is no longer current for terminal decision receipt acknowledgement replay",
        ),
        expiry_guard(
            "terminal_decision_receipt_ack_zero_effect_digest_stale",
            policy_ids.clone(),
            "zero-effect digest no longer matches terminal decision receipt local readback",
        ),
        expiry_guard(
            "terminal_decision_receipt_ack_replay_epoch_expired",
            policy_ids.clone(),
            "terminal decision receipt acknowledgement replay epoch expired without authority packet",
        ),
        expiry_guard(
            "terminal_decision_receipt_ack_release_public_claim_scope_expired",
            policy_ids,
            "release, public claim, and external delivery denial scope expired without publication authority",
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_supersession_guards()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptSupersessionGuardPreview>
{
    vec![
        supersession_guard(
            "newer_terminal_decision_supersedes_decision_receipt_ack_replay",
            "terminal_non_promotion_decision_report",
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["terminalDecisionHash", "receiptHash", "scopeEpoch"],
            ),
        ),
        supersession_guard(
            "newer_terminal_decision_receipt_supersedes_acknowledgement_replay",
            "terminal_decision_non_promotion_receipt_acknowledgement",
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["receiptHash", "acknowledgementHash", "supersessionHash"],
            ),
        ),
        supersession_guard(
            "terminal_decision_ack_replay_epoch_supersedes_retention_scope",
            "terminal_decision_receipt_acknowledgement_replay_epoch",
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["replayEpoch", "acknowledgementSequence", "scopeEpoch"],
            ),
        ),
        supersession_guard(
            "terminal_decision_release_owner_scope_superseded",
            "terminal_decision_release_public_claim_external_delivery_scope",
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["releaseOwnerHash", "publicClaimDenied", "scopeEpoch"],
            ),
        ),
        supersession_guard(
            "terminal_decision_rollback_owner_scope_superseded",
            "terminal_decision_rollback_quarantine_owner_scope",
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["rollbackOwnerHash", "quarantineHash", "scopeEpoch"],
            ),
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_garbage_collection_denials()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGarbageCollectionDenialPreview>
{
    vec![
        garbage_collection_denial(
            "durable_identity_evidence_missing",
            "durable_identity_terminal_decision_receipt_retention_evidence",
            "terminal decision receipt retention expiry preview cannot advance without durable identity evidence",
        ),
        garbage_collection_denial(
            "terminal_decision_gc_cannot_delete_live_state",
            "work_graph_state_store",
            "terminal decision receipt retention expiry preview cannot mutate live graph state",
        ),
        garbage_collection_denial(
            "terminal_decision_gc_cannot_delete_receipt_ack_evidence",
            "terminal_decision_non_promotion_receipt_acknowledgement_evidence",
            "hash-only terminal decision receipt acknowledgement evidence remains local readback evidence",
        ),
        garbage_collection_denial(
            "terminal_decision_gc_cannot_persist_tombstone",
            "terminal_decision_receipt_retention_tombstone_store",
            "preview garbage collection cannot persist terminal decision receipt tombstones",
        ),
        garbage_collection_denial(
            "terminal_decision_gc_cannot_unlock_authority",
            "authority_grant_record",
            "expiry or garbage collection cannot grant authority",
        ),
        garbage_collection_denial(
            "terminal_decision_gc_cannot_publish_release_or_public_claim",
            "release_public_claim_record",
            "expiry or garbage collection cannot publish release status or public claims",
        ),
        garbage_collection_denial(
            "terminal_decision_gc_cannot_send_external_delivery",
            "external_delivery_record",
            "expiry or garbage collection cannot send external delivery",
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_local_views()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionLocalViewPreview>
{
    vec![
        local_view(
            "operator_terminal_decision_receipt_retention_expiry_view",
            "operator",
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec!["retentionPolicyId", "expiryGuardId", "expired", "nextGate"],
            ),
        ),
        local_view(
            "auditor_terminal_decision_receipt_retention_digest_view",
            "auditor",
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "receiptHash",
                    "acknowledgementHash",
                    "retentionPolicyHash",
                    "supersessionHash",
                ],
            ),
        ),
        local_view(
            "release_owner_terminal_decision_receipt_gc_denial_view",
            "release_owner",
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "releaseDenied",
                    "publicationDenied",
                    "publicClaimDenied",
                    "externalDeliveryDenied",
                ],
            ),
        ),
        local_view(
            "runtime_terminal_decision_receipt_retention_zero_effect_view",
            "system",
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "retentionStatePersisted",
                    "garbageCollectionPerformed",
                    "authorityGranted",
                    "publicClaimRecorded",
                    "externalSendPerformed",
                ],
            ),
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_evidence()
-> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionDurableIdentityEvidencePreview
{
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_field_ids(),
        required_for_retention_policy_ids:
            work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_policy_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_invariants()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionInvariantPreview>
{
    vec![
        invariant(
            "terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_requires_durable_identity_evidence",
            "terminal decision receipt retention expiry requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_decision_receipt_retention_is_bounded",
            "terminal decision receipt acknowledgement replay retention is bounded to local preview windows",
        ),
        invariant(
            "terminal_decision_receipt_expiry_blocks_acceptance",
            "expired terminal decision receipt acknowledgement visibility cannot become acceptance or approval recording",
        ),
        invariant(
            "terminal_decision_receipt_supersession_blocks_mutation",
            "superseded terminal scope, digest, replay epoch, or owner scope cannot mutate state",
        ),
        invariant(
            "terminal_decision_receipt_gc_is_denied",
            "garbage collection is preview-denied and cannot persist terminal decision tombstones",
        ),
        invariant(
            "terminal_decision_receipt_retention_views_are_local_only",
            "terminal decision receipt retention, expiry, supersession, and GC denial views cannot be sent externally",
        ),
        invariant(
            "terminal_decision_receipt_retention_preview_has_no_side_effects",
            "this gate cannot persist retention state, record expiry, grant authority, publish, record public claims, or send externally",
        ),
    ]
}

fn retention_policy(
    id: &'static str,
    scope: &'static str,
    retention_window: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionPolicyPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionPolicyPreview {
        id,
        scope,
        retention_window,
        required_fields:
            with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
                vec![
                    "retentionPolicyId",
                    "scope",
                    "retentionWindow",
                    "hashOnlyEvidence",
                ],
            ),
        hash_only: true,
        persistence_enabled: false,
        garbage_collection_allowed: false,
    }
}

fn expiry_guard(
    id: &'static str,
    applies_to_policy_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptExpiryGuardPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptExpiryGuardPreview {
        id,
        applies_to_policy_ids,
        trigger,
        blocks_acceptance: true,
        blocks_persistence: true,
    }
}

fn supersession_guard(
    id: &'static str,
    supersedes: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptSupersessionGuardPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptSupersessionGuardPreview {
        id,
        supersedes,
        required_fields,
        blocks_mutation: true,
    }
}

fn garbage_collection_denial(
    id: &'static str,
    target: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGarbageCollectionDenialPreview
{
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptGarbageCollectionDenialPreview {
        id,
        target,
        reason,
        garbage_collection_allowed: false,
        blocks_mutation: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionLocalViewPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn with_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionInvariantPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionExpiryPreviewSideEffects {
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
            retention_state_persisted: false,
            expiry_recorded: false,
            garbage_collection_performed: false,
            tombstone_persisted: false,
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
    fn work_graph_terminal_decision_receipt_retention_declares_bounded_hash_only_policies() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report();

        assert_eq!(report.retention_policy_count, 6);
        assert_eq!(
            report
                .retention_policies
                .iter()
                .map(|policy| policy.id)
                .collect::<Vec<_>>(),
            work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_policy_ids()
        );
        assert!(report.retention_policies.iter().all(|policy| {
            policy.hash_only
                && !policy.persistence_enabled
                && !policy.garbage_collection_allowed
                && policy.required_fields.len() >= 11
                && policy.required_fields.contains(&"workflow_id")
                && policy.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_blocks_expired_or_superseded_scope() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report();

        assert_eq!(report.expiry_guard_count, 6);
        assert!(report.expiry_guards.iter().all(|guard| {
            guard.blocks_acceptance
                && guard.blocks_persistence
                && guard.applies_to_policy_ids.len() == 6
        }));
        assert_eq!(report.supersession_guard_count, 5);
        assert!(report.supersession_guards.iter().all(|guard| {
            guard.blocks_mutation
                && guard.required_fields.len() >= 10
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_denies_garbage_collection_mutations() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report();

        assert_eq!(report.garbage_collection_denial_count, 7);
        assert!(
            report
                .garbage_collection_denials
                .iter()
                .all(|denial| { !denial.garbage_collection_allowed && denial.blocks_mutation })
        );
        assert_eq!(
            report
                .garbage_collection_denials
                .first()
                .map(|denial| denial.id),
            Some("durable_identity_evidence_missing")
        );
        assert!(
            report
                .garbage_collection_denials
                .iter()
                .any(|denial| denial.id == "terminal_decision_gc_cannot_send_external_delivery")
        );
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_requires_ack_replay_idempotency_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report();

        assert!(matches!(
            report.required_prior_gates.as_slice(),
            [
                ..,
                "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate"
            ]
        ));
        assert!(report.ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview);
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_requires_durable_identity_evidence() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report();

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
            work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_retention_policy_ids,
            work_graph_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_policy_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert!(report.local_views.iter().all(|view| {
            !view.external_delivery_enabled
                && view.required_fields.len() >= 11
                && view.required_fields.contains(&"workflow_id")
                && view.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report();

        assert_eq!(report.invariant_count, 7);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert_eq!(
            report.invariants.first().map(|invariant| invariant.id),
            Some(
                "terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_requires_durable_identity_evidence"
            )
        );
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionExpiryPreviewSideEffects::none()
        );
    }
}
