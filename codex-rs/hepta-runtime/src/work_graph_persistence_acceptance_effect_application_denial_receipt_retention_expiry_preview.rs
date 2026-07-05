use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryPreviewReport {
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
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionPolicyPreview>,
    pub expiry_guards:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryGuardPreview>,
    pub supersession_guards:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptSupersessionGuardPreview>,
    pub garbage_collection_denials:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptGarbageCollectionDenialPreview>,
    pub local_views:
        Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionPolicyPreview {
    pub id: &'static str,
    pub scope: &'static str,
    pub retention_window: &'static str,
    pub required_fields: Vec<&'static str>,
    pub hash_only: bool,
    pub persistence_enabled: bool,
    pub garbage_collection_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryGuardPreview {
    pub id: &'static str,
    pub applies_to_policy_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub blocks_acceptance: bool,
    pub blocks_persistence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptSupersessionGuardPreview {
    pub id: &'static str,
    pub supersedes: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptGarbageCollectionDenialPreview {
    pub id: &'static str,
    pub target: &'static str,
    pub reason: &'static str,
    pub garbage_collection_allowed: bool,
    pub blocks_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionLocalViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionDurableIdentityEvidencePreview
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
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
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
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_report()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryPreviewReport {
    let retention_policies =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_policies();
    let expiry_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_expiry_guards();
    let supersession_guards =
        work_graph_persistence_acceptance_effect_denial_receipt_supersession_guards();
    let garbage_collection_denials =
        work_graph_persistence_acceptance_effect_denial_receipt_garbage_collection_denials();
    let local_views =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_durable_identity_evidence(
        );
    let invariants = work_graph_persistence_acceptance_effect_denial_receipt_retention_invariants();

    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_no_retention_write",
        retention_policy_count: retention_policies.len(),
        expiry_guard_count: expiry_guards.len(),
        supersession_guard_count: supersession_guards.len(),
        garbage_collection_denial_count: garbage_collection_denials.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_required_prior_gates(),
        retention_policies,
        expiry_guards,
        supersession_guards,
        garbage_collection_denials,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_required_prior_gates()
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_policy_ids()
-> Vec<&'static str> {
    vec![
        "effect_denial_receipt_local_view_retention_policy",
        "effect_denial_receipt_acknowledgement_retention_policy",
        "effect_denial_receipt_replay_index_retention_policy",
        "effect_denial_receipt_zero_effect_digest_retention_policy",
        "effect_denial_receipt_supersession_marker_retention_policy",
        "effect_denial_receipt_release_external_denial_retention_policy",
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionDurableIdentityEvidencePreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_durable_identity_field_ids(),
        required_for_retention_policy_ids:
            work_graph_persistence_acceptance_effect_denial_receipt_retention_policy_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_policies()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionPolicyPreview> {
    vec![
        retention_policy(
            "effect_denial_receipt_local_view_retention_policy",
            "operator_auditor_release_owner_local_views",
            "bounded_to_preview_session_window",
        ),
        retention_policy(
            "effect_denial_receipt_acknowledgement_retention_policy",
            "non_recording_acknowledgement_visibility",
            "bounded_to_acknowledgement_preview_window",
        ),
        retention_policy(
            "effect_denial_receipt_replay_index_retention_policy",
            "idempotency_and_monotonicity_keys",
            "bounded_to_replay_preview_window",
        ),
        retention_policy(
            "effect_denial_receipt_zero_effect_digest_retention_policy",
            "zero_write_zero_traffic_zero_external_digest",
            "bounded_to_digest_verification_window",
        ),
        retention_policy(
            "effect_denial_receipt_supersession_marker_retention_policy",
            "scope_epoch_and_supersession_markers",
            "bounded_to_current_scope_epoch",
        ),
        retention_policy(
            "effect_denial_receipt_release_external_denial_retention_policy",
            "release_publication_external_delivery_denials",
            "bounded_to_release_owner_preview_window",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_expiry_guards()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryGuardPreview> {
    let policy_ids = work_graph_persistence_acceptance_effect_denial_receipt_retention_policy_ids();

    vec![
        expiry_guard(
            "retention_window_expired",
            policy_ids.clone(),
            "retention window expired without authority packet",
        ),
        expiry_guard(
            "receipt_scope_superseded",
            policy_ids.clone(),
            "receipt scope was superseded by a newer effect blocker report",
        ),
        expiry_guard(
            "prior_gate_digest_expired",
            policy_ids.clone(),
            "prior gate digest is no longer current for this scope",
        ),
        expiry_guard(
            "zero_effect_digest_stale",
            policy_ids.clone(),
            "zero-effect digest no longer matches local readback",
        ),
        expiry_guard(
            "operator_visibility_window_expired",
            policy_ids.clone(),
            "operator local visibility window expired without acceptance authority",
        ),
        expiry_guard(
            "release_external_delivery_scope_expired",
            policy_ids,
            "release and external delivery denial scope expired without publication authority",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_supersession_guards()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptSupersessionGuardPreview> {
    vec![
        supersession_guard(
            "newer_effect_blocker_report_supersedes_receipt",
            "effect_application_blocker_report",
            with_acceptance_effect_denial_receipt_retention_durable_identity_fields(vec![
                "priorGateDigest",
                "newGateDigest",
                "scopeEpoch",
            ]),
        ),
        supersession_guard(
            "newer_denial_receipt_supersedes_acknowledgement",
            "effect_application_denial_receipt_acknowledgement",
            with_acceptance_effect_denial_receipt_retention_durable_identity_fields(vec![
                "denialReceiptHash",
                "acknowledgementHash",
                "supersessionHash",
            ]),
        ),
        supersession_guard(
            "replay_epoch_supersedes_retention_scope",
            "denial_receipt_replay_epoch",
            with_acceptance_effect_denial_receipt_retention_durable_identity_fields(vec![
                "replayEpoch",
                "readbackSequence",
                "scopeEpoch",
            ]),
        ),
        supersession_guard(
            "rollback_quarantine_owner_scope_superseded",
            "rollback_quarantine_owner_scope",
            with_acceptance_effect_denial_receipt_retention_durable_identity_fields(vec![
                "rollbackOwnerHash",
                "quarantineHash",
                "scopeEpoch",
            ]),
        ),
        supersession_guard(
            "release_owner_scope_superseded",
            "release_publication_external_delivery_scope",
            with_acceptance_effect_denial_receipt_retention_durable_identity_fields(vec![
                "releaseOwnerHash",
                "publicationDenied",
                "externalDeliveryDenied",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_garbage_collection_denials()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptGarbageCollectionDenialPreview> {
    vec![
        garbage_collection_denial(
            "durable_identity_evidence_missing",
            "durable_identity_retention_evidence",
            "retention expiry preview cannot advance without durable identity evidence",
        ),
        garbage_collection_denial(
            "gc_cannot_delete_live_state",
            "work_graph_state_store",
            "retention expiry preview cannot mutate live graph state",
        ),
        garbage_collection_denial(
            "gc_cannot_delete_receipt_evidence",
            "effect_denial_receipt_evidence",
            "hash-only denial evidence remains local readback evidence, not a deletable live record",
        ),
        garbage_collection_denial(
            "gc_cannot_persist_tombstone",
            "retention_tombstone_store",
            "preview garbage collection cannot persist tombstones",
        ),
        garbage_collection_denial(
            "gc_cannot_unlock_authority",
            "authority_grant_record",
            "expiry or garbage collection cannot grant authority",
        ),
        garbage_collection_denial(
            "gc_cannot_publish_release",
            "release_publication_record",
            "expiry or garbage collection cannot publish release status",
        ),
        garbage_collection_denial(
            "gc_cannot_send_external_delivery",
            "external_delivery_record",
            "expiry or garbage collection cannot send external delivery",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_local_views()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionLocalViewPreview> {
    vec![
        local_view(
            "operator_effect_denial_receipt_retention_expiry_view",
            "operator",
            with_acceptance_effect_denial_receipt_retention_durable_identity_fields(vec![
                "retentionPolicyId",
                "expiryGuardId",
                "expired",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_effect_denial_receipt_retention_digest_view",
            "auditor",
            with_acceptance_effect_denial_receipt_retention_durable_identity_fields(vec![
                "retentionPolicyHash",
                "expiryGuardHash",
                "supersessionHash",
                "zeroEffectHash",
            ]),
        ),
        local_view(
            "release_owner_effect_denial_receipt_gc_denial_view",
            "release_owner",
            with_acceptance_effect_denial_receipt_retention_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "garbageCollectionDenied",
            ]),
        ),
        local_view(
            "runtime_effect_denial_receipt_retention_zero_effect_view",
            "system",
            with_acceptance_effect_denial_receipt_retention_durable_identity_fields(vec![
                "retentionStatePersisted",
                "garbageCollectionPerformed",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_denial_receipt_retention_invariants()
-> Vec<WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionInvariantPreview> {
    vec![
        invariant(
            "effect_denial_receipt_retention_requires_durable_identity_evidence",
            "effect denial receipt retention expiry requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "effect_denial_receipt_retention_is_bounded",
            "retention policies are bounded to local preview windows",
        ),
        invariant(
            "effect_denial_receipt_expiry_blocks_acceptance",
            "expired receipt visibility cannot become acceptance or approval recording",
        ),
        invariant(
            "effect_denial_receipt_supersession_blocks_mutation",
            "superseded scope, digest, replay epoch, or owner scope cannot mutate state",
        ),
        invariant(
            "effect_denial_receipt_gc_is_denied",
            "garbage collection is preview-denied and cannot persist tombstones",
        ),
        invariant(
            "effect_denial_receipt_retention_views_are_local_only",
            "retention, expiry, supersession, and GC denial views cannot be sent externally",
        ),
        invariant(
            "effect_denial_receipt_retention_preview_has_no_side_effects",
            "this gate cannot persist retention state, record expiry, grant authority, publish, or send externally",
        ),
    ]
}

fn retention_policy(
    id: &'static str,
    scope: &'static str,
    retention_window: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionPolicyPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionPolicyPreview {
        id,
        scope,
        retention_window,
        required_fields: with_acceptance_effect_denial_receipt_retention_durable_identity_fields(
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
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryGuardPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryGuardPreview {
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
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptSupersessionGuardPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptSupersessionGuardPreview {
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
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptGarbageCollectionDenialPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptGarbageCollectionDenialPreview {
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
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionLocalViewPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionInvariantPreview {
    WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionInvariantPreview {
        id,
        required: true,
        reason,
    }
}

fn with_acceptance_effect_denial_receipt_retention_durable_identity_fields(
    mut fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut durable_fields =
        work_graph_persistence_acceptance_effect_denial_receipt_retention_durable_identity_field_ids(
        );
    durable_fields.append(&mut fields);
    durable_fields
}

impl WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
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
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retention_expiry_declares_bounded_hash_only_policies() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_report();

        assert_eq!(report.retention_policy_count, 6);
        assert_eq!(
            report
                .retention_policies
                .iter()
                .map(|policy| policy.id)
                .collect::<Vec<_>>(),
            work_graph_persistence_acceptance_effect_denial_receipt_retention_policy_ids()
        );
        assert!(report.retention_policies.iter().all(|policy| {
            policy.hash_only
                && !policy.persistence_enabled
                && !policy.garbage_collection_allowed
                && policy.required_fields.contains(&"workflow_id")
                && policy.required_fields.contains(&"receipt_hash")
                && policy.required_fields.len() >= 11
        }));
    }

    #[test]
    fn retention_expiry_blocks_expired_or_superseded_scope() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_report();

        assert_eq!(report.expiry_guard_count, 6);
        assert!(report.expiry_guards.iter().all(|guard| {
            guard.blocks_acceptance
                && guard.blocks_persistence
                && guard.applies_to_policy_ids.len() == 6
        }));
        assert_eq!(report.supersession_guard_count, 5);
        assert!(report.supersession_guards.iter().all(|guard| {
            guard.blocks_mutation
                && guard.required_fields.contains(&"workflow_id")
                && guard.required_fields.contains(&"receipt_hash")
                && guard.required_fields.len() >= 10
        }));
    }

    #[test]
    fn retention_expiry_denies_garbage_collection_mutations() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_report();

        assert_eq!(report.garbage_collection_denial_count, 7);
        assert!(
            report
                .garbage_collection_denials
                .iter()
                .all(|denial| { !denial.garbage_collection_allowed && denial.blocks_mutation })
        );
        assert!(
            report
                .garbage_collection_denials
                .iter()
                .any(|denial| denial.id == "durable_identity_evidence_missing")
        );
        assert!(
            report
                .garbage_collection_denials
                .iter()
                .any(|denial| denial.id == "gc_cannot_send_external_delivery")
        );
    }

    #[test]
    fn retention_expiry_requires_replay_idempotency_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview
        );
    }

    #[test]
    fn retention_expiry_requires_durable_identity_evidence() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_report();

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
            work_graph_persistence_acceptance_effect_denial_receipt_retention_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_retention_policy_ids,
            work_graph_persistence_acceptance_effect_denial_receipt_retention_policy_ids()
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert_eq!(report.durable_identity_evidence.preview_binding_count, 5);
        assert_eq!(report.durable_identity_evidence.invariant_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn retention_expiry_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert!(report.local_views.iter().all(|view| {
            !view.external_delivery_enabled
                && view.required_fields.contains(&"workflow_id")
                && view.required_fields.contains(&"receipt_hash")
                && view.required_fields.len() >= 11
        }));
        assert_eq!(report.invariant_count, 7);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
    }

    #[test]
    fn retention_expiry_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_report();

        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionExpiryPreviewSideEffects::none()
        );
    }
}
