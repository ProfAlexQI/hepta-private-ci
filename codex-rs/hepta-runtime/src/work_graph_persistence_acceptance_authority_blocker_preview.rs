use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_AUTHORITY_BLOCKER_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_AUTHORITY_BLOCKER_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_authority_blocker_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_AUTHORITY_BLOCKER_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_acceptance_record_intake_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceAuthorityBlockerPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub authority_surface_count: usize,
    pub authority_denial_count: usize,
    pub escalation_guard_count: usize,
    pub required_record_count: usize,
    pub authority_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub authority_surfaces: Vec<WorkGraphPersistenceAcceptanceAuthoritySurfacePreview>,
    pub authority_denials: Vec<WorkGraphPersistenceAcceptanceAuthorityDenialPreview>,
    pub escalation_guards: Vec<WorkGraphPersistenceAcceptanceAuthorityEscalationGuardPreview>,
    pub required_records: Vec<WorkGraphPersistenceAcceptanceAuthorityRequiredRecordPreview>,
    pub authority_views: Vec<WorkGraphPersistenceAcceptanceAuthorityViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceAuthorityDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceAcceptanceAuthorityInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_record_intake_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceAcceptanceAuthorityBlockerPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceAuthoritySurfacePreview {
    pub id: &'static str,
    pub requested_capability: &'static str,
    pub required_fields: Vec<&'static str>,
    pub authority_granted: bool,
    pub approval_recording_enabled: bool,
    pub live_execution_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceAuthorityDenialPreview {
    pub id: &'static str,
    pub applies_to_surface_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceAuthorityEscalationGuardPreview {
    pub id: &'static str,
    pub from_signal: &'static str,
    pub to_blocked_capability: &'static str,
    pub required_denial_fields: Vec<&'static str>,
    pub blocks_escalation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceAuthorityRequiredRecordPreview {
    pub id: &'static str,
    pub required_for_surface_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub present_in_preview: bool,
    pub accepted_in_preview: bool,
    pub recording_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceAuthorityViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceAuthorityDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_surface_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceAuthorityInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceAuthorityBlockerPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub readiness_receipt_persisted: bool,
    pub acknowledgement_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_authority_blocker_preview_report()
-> WorkGraphPersistenceAcceptanceAuthorityBlockerPreviewReport {
    let authority_surfaces = work_graph_persistence_acceptance_authority_surfaces();
    let authority_denials = work_graph_persistence_acceptance_authority_denials();
    let escalation_guards = work_graph_persistence_acceptance_authority_escalation_guards();
    let required_records = work_graph_persistence_acceptance_authority_required_records();
    let authority_views = work_graph_persistence_acceptance_authority_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_authority_durable_identity_evidence();
    let invariants = work_graph_persistence_acceptance_authority_invariants();

    WorkGraphPersistenceAcceptanceAuthorityBlockerPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_AUTHORITY_BLOCKER_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_AUTHORITY_BLOCKER_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_authority_blocker_preview_no_authority",
        authority_surface_count: authority_surfaces.len(),
        authority_denial_count: authority_denials.len(),
        escalation_guard_count: escalation_guards.len(),
        required_record_count: required_records.len(),
        authority_view_count: authority_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_persistence_acceptance_authority_required_prior_gates(),
        authority_surfaces,
        authority_denials,
        escalation_guards,
        required_records,
        authority_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_AUTHORITY_BLOCKER_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_record_intake_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceAcceptanceAuthorityBlockerPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_authority_required_prior_gates() -> Vec<&'static str> {
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_authority_durable_identity_field_ids() -> Vec<&'static str>
{
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

pub fn work_graph_persistence_acceptance_authority_surface_ids() -> Vec<&'static str> {
    vec![
        "operator_acceptance_authority",
        "approval_recording_authority",
        "live_persistence_authority",
        "wal_checkpoint_authority",
        "enforcement_rollout_authority",
        "release_publication_authority",
        "external_delivery_authority",
    ]
}

pub fn work_graph_persistence_acceptance_authority_surfaces()
-> Vec<WorkGraphPersistenceAcceptanceAuthoritySurfacePreview> {
    vec![
        authority_surface(
            "operator_acceptance_authority",
            "record trusted operator acceptance for WorkGraph persistence",
            vec![
                "trustedOperatorIdHash",
                "operatorScopeHash",
                "acceptanceRecordHash",
                "authorityDenialIds",
            ],
        ),
        authority_surface(
            "approval_recording_authority",
            "write approval or readiness acceptance into the WorkGraph ledger",
            vec![
                "approvalLedgerId",
                "approvalRecordHash",
                "readinessReceiptHash",
                "recordingDenialIds",
            ],
        ),
        authority_surface(
            "live_persistence_authority",
            "enable state store persistence for WorkGraph collections",
            vec![
                "featureFlagId",
                "enablementPacketHash",
                "zeroWriteProofHash",
                "killSwitchId",
            ],
        ),
        authority_surface(
            "wal_checkpoint_authority",
            "enable WAL append or checkpoint write execution",
            vec![
                "walScopeHash",
                "checkpointScopeHash",
                "idempotencyGuardHash",
                "rollbackOwnerHash",
            ],
        ),
        authority_surface(
            "enforcement_rollout_authority",
            "turn enforcement rollout or traffic ramp beyond zero",
            vec![
                "rolloutStageId",
                "trafficRampId",
                "operatorEnablementHash",
                "quarantineOwnerHash",
            ],
        ),
        authority_surface(
            "release_publication_authority",
            "publish release status or external artifact availability",
            vec![
                "releaseOwnerHash",
                "publicationPolicyHash",
                "publicClaimDenialIds",
                "artifactManifestHash",
            ],
        ),
        authority_surface(
            "external_delivery_authority",
            "send readiness, rollout, or release receipts outside the local preview",
            vec![
                "deliveryChannelId",
                "recipientScopeHash",
                "externalPolicyHash",
                "deliveryDenialIds",
            ],
        ),
    ]
}

pub fn work_graph_persistence_acceptance_authority_denials()
-> Vec<WorkGraphPersistenceAcceptanceAuthorityDenialPreview> {
    let surface_ids = work_graph_persistence_acceptance_authority_surface_ids();

    vec![
        authority_denial(
            "durable_identity_evidence_missing",
            surface_ids.clone(),
            "authority blocker cannot clear while durable identity evidence is missing",
        ),
        authority_denial(
            "readiness_packet_is_not_acceptance_authority",
            surface_ids.clone(),
            "a completed readiness packet is still a preview input and cannot grant authority",
        ),
        authority_denial(
            "readiness_receipt_is_hash_only_evidence",
            surface_ids.clone(),
            "a hash-only readiness receipt proves local evidence only and cannot grant authority",
        ),
        authority_denial(
            "acknowledgement_visibility_is_not_acceptance",
            surface_ids.clone(),
            "operator acknowledgement visibility cannot become acceptance or approval recording",
        ),
        authority_denial(
            "signature_hash_is_not_live_signature",
            surface_ids.clone(),
            "redacted signature hashes are not executable live authorization",
        ),
        authority_denial(
            "approval_ledger_write_is_blocked",
            vec![
                "approval_recording_authority",
                "operator_acceptance_authority",
                "live_persistence_authority",
            ],
            "approval and acceptance ledger writes are disabled in preview",
        ),
        authority_denial(
            "live_persistence_enablement_is_blocked",
            vec![
                "live_persistence_authority",
                "wal_checkpoint_authority",
                "enforcement_rollout_authority",
            ],
            "feature flags, WAL, checkpoints, enforcement, and traffic remain disabled",
        ),
        authority_denial(
            "release_publication_policy_is_incomplete",
            vec![
                "release_publication_authority",
                "external_delivery_authority",
            ],
            "release publication and external delivery policy are not accepted",
        ),
        authority_denial(
            "operator_scope_expired_or_revoked_blocks_authority",
            surface_ids,
            "expired, superseded, revoked, or digest-mismatched operator scope blocks authority",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_authority_escalation_guards()
-> Vec<WorkGraphPersistenceAcceptanceAuthorityEscalationGuardPreview> {
    vec![
        escalation_guard(
            "receipt_to_acceptance_guard",
            "hash_only_readiness_receipt",
            "operator_acceptance_authority",
            with_acceptance_authority_durable_identity_fields(vec![
                "receiptHash",
                "nonAcceptanceReasonIds",
                "authorityDenied",
            ]),
        ),
        escalation_guard(
            "acknowledgement_to_approval_guard",
            "receipt_acknowledgement_visibility",
            "approval_recording_authority",
            with_acceptance_authority_durable_identity_fields(vec![
                "acknowledgementHash",
                "recordingDenialIds",
                "approvalRecorded",
            ]),
        ),
        escalation_guard(
            "approval_to_live_persistence_guard",
            "approval_or_acceptance_attempt",
            "live_persistence_authority",
            with_acceptance_authority_durable_identity_fields(vec![
                "approvalLedgerWriteDenied",
                "featureFlagStillOff",
                "zeroWriteProofHash",
            ]),
        ),
        escalation_guard(
            "persistence_to_rollout_guard",
            "persistence_enablement_attempt",
            "enforcement_rollout_authority",
            with_acceptance_authority_durable_identity_fields(vec![
                "rolloutStageZeroTraffic",
                "enforcementDisabled",
                "killSwitchId",
            ]),
        ),
        escalation_guard(
            "rollout_to_release_guard",
            "rollout_or_traffic_attempt",
            "release_publication_authority",
            with_acceptance_authority_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "publicClaimDenied",
            ]),
        ),
        escalation_guard(
            "release_to_external_delivery_guard",
            "release_publication_attempt",
            "external_delivery_authority",
            with_acceptance_authority_durable_identity_fields(vec![
                "externalDeliveryDenied",
                "recipientScopeDenied",
                "channelSendDenied",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_authority_required_records()
-> Vec<WorkGraphPersistenceAcceptanceAuthorityRequiredRecordPreview> {
    vec![
        required_record(
            "trusted_operator_acceptance_record",
            vec![
                "operator_acceptance_authority",
                "approval_recording_authority",
            ],
            vec![
                "trustedOperatorIdHash",
                "operatorScopeHash",
                "acceptedCapabilityIds",
                "explicitAuthorityGrant",
                "expiresAtUnixMs",
            ],
        ),
        required_record(
            "approval_decision_record",
            vec!["approval_recording_authority", "live_persistence_authority"],
            vec![
                "approvalLedgerId",
                "decisionHash",
                "approverScopeHash",
                "recordedAtUnixMs",
                "rollbackBindingHash",
            ],
        ),
        required_record(
            "live_persistence_enablement_record",
            vec!["live_persistence_authority", "wal_checkpoint_authority"],
            vec![
                "featureFlagState",
                "enablementPacketHash",
                "zeroTrafficProofHash",
                "zeroWriteProofHash",
                "killSwitchId",
            ],
        ),
        required_record(
            "rollback_quarantine_owner_attestation",
            vec!["wal_checkpoint_authority", "enforcement_rollout_authority"],
            vec![
                "rollbackOwnerHash",
                "quarantineOwnerHash",
                "killSwitchId",
                "revertPlanHash",
                "ownerSignatureHash",
            ],
        ),
        required_record(
            "release_publication_owner_attestation",
            vec!["release_publication_authority"],
            vec![
                "releaseOwnerHash",
                "publicationPolicyHash",
                "artifactManifestHash",
                "publicClaimScopeHash",
                "releaseDenialOverrideHash",
            ],
        ),
        required_record(
            "external_delivery_consent_record",
            vec!["external_delivery_authority"],
            vec![
                "deliveryOwnerHash",
                "recipientScopeHash",
                "channelPolicyHash",
                "externalSendConsentHash",
                "redactionPolicyHash",
            ],
        ),
    ]
}

pub fn work_graph_persistence_acceptance_authority_views()
-> Vec<WorkGraphPersistenceAcceptanceAuthorityViewPreview> {
    vec![
        authority_view(
            "operator_authority_denial_view",
            "operator",
            with_acceptance_authority_durable_identity_fields(vec![
                "authorityDenied",
                "nonAcceptanceReasonIds",
                "requiredRecordIds",
                "nextGate",
            ]),
        ),
        authority_view(
            "auditor_authority_evidence_view",
            "auditor",
            with_acceptance_authority_durable_identity_fields(vec![
                "readinessReceiptHash",
                "acknowledgementHash",
                "authorityDenialIds",
                "sideEffectHash",
            ]),
        ),
        authority_view(
            "release_owner_authority_blocker_view",
            "release_owner",
            with_acceptance_authority_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "requiredRecordIds",
            ]),
        ),
        authority_view(
            "runtime_authority_zero_effect_view",
            "system",
            with_acceptance_authority_durable_identity_fields(vec![
                "authorityGranted",
                "approvalRecorded",
                "livePersistenceEnabled",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_authority_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceAuthorityDurableIdentityEvidencePreview {
    WorkGraphPersistenceAcceptanceAuthorityDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: work_graph_persistence_acceptance_authority_durable_identity_field_ids(
        ),
        required_for_surface_ids: work_graph_persistence_acceptance_authority_surface_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_authority_invariants()
-> Vec<WorkGraphPersistenceAcceptanceAuthorityInvariantPreview> {
    vec![
        invariant(
            "acceptance_authority_requires_durable_identity_evidence",
            "authority blocker surfaces require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "receipt_and_acknowledgement_cannot_grant_authority",
            "readiness receipt and acknowledgement visibility are evidence only",
        ),
        invariant(
            "authority_requires_explicit_record_intake",
            "every authority surface requires missing explicit trusted records",
        ),
        invariant(
            "approval_recording_is_blocked",
            "approval and operator acceptance recording remain disabled",
        ),
        invariant(
            "live_persistence_and_rollout_are_blocked",
            "persistence, WAL, checkpoints, enforcement, rollout, and traffic remain disabled",
        ),
        invariant(
            "release_and_external_delivery_are_blocked",
            "release publication and external delivery remain denied",
        ),
        invariant(
            "acceptance_authority_blocker_preview_has_no_side_effects",
            "this gate cannot write state, record authority, enable live execution, publish, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceAcceptanceAuthorityBlockerPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            readiness_receipt_persisted: false,
            acknowledgement_recorded: false,
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

fn authority_surface(
    id: &'static str,
    requested_capability: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceAuthoritySurfacePreview {
    WorkGraphPersistenceAcceptanceAuthoritySurfacePreview {
        id,
        requested_capability,
        required_fields: with_acceptance_authority_durable_identity_fields(required_fields),
        authority_granted: false,
        approval_recording_enabled: false,
        live_execution_enabled: false,
        external_delivery_enabled: false,
    }
}

fn authority_denial(
    id: &'static str,
    applies_to_surface_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceAuthorityDenialPreview {
    WorkGraphPersistenceAcceptanceAuthorityDenialPreview {
        id,
        applies_to_surface_ids,
        reason,
        blocks_authority: true,
    }
}

fn escalation_guard(
    id: &'static str,
    from_signal: &'static str,
    to_blocked_capability: &'static str,
    required_denial_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceAuthorityEscalationGuardPreview {
    WorkGraphPersistenceAcceptanceAuthorityEscalationGuardPreview {
        id,
        from_signal,
        to_blocked_capability,
        required_denial_fields,
        blocks_escalation: true,
    }
}

fn required_record(
    id: &'static str,
    required_for_surface_ids: Vec<&'static str>,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceAuthorityRequiredRecordPreview {
    WorkGraphPersistenceAcceptanceAuthorityRequiredRecordPreview {
        id,
        required_for_surface_ids,
        required_fields: with_acceptance_authority_durable_identity_fields(required_fields),
        present_in_preview: false,
        accepted_in_preview: false,
        recording_enabled: false,
    }
}

fn with_acceptance_authority_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged = work_graph_persistence_acceptance_authority_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn authority_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceAuthorityViewPreview {
    WorkGraphPersistenceAcceptanceAuthorityViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceAuthorityInvariantPreview {
    WorkGraphPersistenceAcceptanceAuthorityInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_authority_blocker_declares_blocked_surfaces() {
        let report = hepta_work_graph_persistence_acceptance_authority_blocker_preview_report();
        let surface_ids = report
            .authority_surfaces
            .iter()
            .map(|surface| surface.id)
            .collect::<Vec<_>>();

        assert_eq!(
            surface_ids,
            [
                "operator_acceptance_authority",
                "approval_recording_authority",
                "live_persistence_authority",
                "wal_checkpoint_authority",
                "enforcement_rollout_authority",
                "release_publication_authority",
                "external_delivery_authority",
            ]
        );
        assert_eq!(report.authority_surface_count, 7);
        assert!(report.authority_surfaces.iter().all(|surface| {
            !surface.authority_granted
                && !surface.approval_recording_enabled
                && !surface.live_execution_enabled
                && !surface.external_delivery_enabled
                && surface.required_fields.len() >= 11
                && surface.required_fields.contains(&"workflow_id")
                && surface.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn acceptance_authority_blocker_denies_receipts_and_acknowledgements() {
        let report = hepta_work_graph_persistence_acceptance_authority_blocker_preview_report();
        let denial_ids = report
            .authority_denials
            .iter()
            .map(|denial| denial.id)
            .collect::<Vec<_>>();

        assert_eq!(
            denial_ids,
            [
                "durable_identity_evidence_missing",
                "readiness_packet_is_not_acceptance_authority",
                "readiness_receipt_is_hash_only_evidence",
                "acknowledgement_visibility_is_not_acceptance",
                "signature_hash_is_not_live_signature",
                "approval_ledger_write_is_blocked",
                "live_persistence_enablement_is_blocked",
                "release_publication_policy_is_incomplete",
                "operator_scope_expired_or_revoked_blocks_authority",
            ]
        );
        assert_eq!(report.authority_denial_count, 9);
        assert!(
            report
                .authority_denials
                .iter()
                .all(|denial| denial.blocks_authority && !denial.applies_to_surface_ids.is_empty())
        );
    }

    #[test]
    fn acceptance_authority_blocker_blocks_escalation_chain() {
        let report = hepta_work_graph_persistence_acceptance_authority_blocker_preview_report();
        let guard_ids = report
            .escalation_guards
            .iter()
            .map(|guard| guard.id)
            .collect::<Vec<_>>();

        assert_eq!(
            guard_ids,
            [
                "receipt_to_acceptance_guard",
                "acknowledgement_to_approval_guard",
                "approval_to_live_persistence_guard",
                "persistence_to_rollout_guard",
                "rollout_to_release_guard",
                "release_to_external_delivery_guard",
            ]
        );
        assert_eq!(report.escalation_guard_count, 6);
        assert!(
            report
                .escalation_guards
                .iter()
                .all(|guard| guard.blocks_escalation
                    && guard.required_denial_fields.len() >= 10
                    && guard.required_denial_fields.contains(&"workflow_id")
                    && guard.required_denial_fields.contains(&"receipt_hash"))
        );
    }

    #[test]
    fn acceptance_authority_blocker_requires_absent_records() {
        let report = hepta_work_graph_persistence_acceptance_authority_blocker_preview_report();

        assert_eq!(report.required_record_count, 6);
        assert!(report.required_records.iter().all(|record| {
            !record.present_in_preview
                && !record.accepted_in_preview
                && !record.recording_enabled
                && record.required_fields.len() >= 12
                && record.required_fields.contains(&"workflow_id")
                && record.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn acceptance_authority_blocker_requires_durable_identity_evidence() {
        let report = hepta_work_graph_persistence_acceptance_authority_blocker_preview_report();

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
            [
                "workflow_id",
                "run_id",
                "step_id",
                "checkpoint",
                "replay_key",
                "rollback_anchor",
                "receipt_hash",
            ]
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_surface_ids
                .len(),
            7
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn acceptance_authority_blocker_requires_acknowledgement_gate() {
        let report = hepta_work_graph_persistence_acceptance_authority_blocker_preview_report();

        assert_eq!(
            report.required_prior_gates,
            [
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
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_AUTHORITY_BLOCKER_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn acceptance_authority_blocker_has_no_side_effects() {
        let report = hepta_work_graph_persistence_acceptance_authority_blocker_preview_report();

        assert_eq!(report.authority_view_count, 4);
        assert!(
            report
                .authority_views
                .iter()
                .all(|view| !view.external_delivery_enabled
                    && view.required_fields.len() >= 11
                    && view.required_fields.contains(&"workflow_id")
                    && view.required_fields.contains(&"receipt_hash"))
        );
        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceAuthorityBlockerPreviewSideEffects::none()
        );
        assert!(report.ready_for_acceptance_record_intake_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
