use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_INTAKE_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_record_intake_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_INTAKE_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_record_intake_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_INTAKE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_acceptance_record_receipt_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordIntakePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub record_template_count: usize,
    pub validation_denial_count: usize,
    pub intake_guard_count: usize,
    pub redaction_digest_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub record_templates: Vec<WorkGraphPersistenceAcceptanceRecordTemplatePreview>,
    pub validation_denials: Vec<WorkGraphPersistenceAcceptanceRecordValidationDenialPreview>,
    pub intake_guards: Vec<WorkGraphPersistenceAcceptanceRecordIntakeGuardPreview>,
    pub redaction_digests: Vec<WorkGraphPersistenceAcceptanceRecordRedactionDigestPreview>,
    pub local_views: Vec<WorkGraphPersistenceAcceptanceRecordLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceRecordIntakeDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceAcceptanceRecordInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_record_receipt_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceAcceptanceRecordIntakePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordTemplatePreview {
    pub id: &'static str,
    pub target_authority_surface_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub present_in_preview: bool,
    pub accepted_in_preview: bool,
    pub recording_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordValidationDenialPreview {
    pub id: &'static str,
    pub applies_to_record_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_record_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordIntakeGuardPreview {
    pub id: &'static str,
    pub blocked_effect: &'static str,
    pub required_evidence_fields: Vec<&'static str>,
    pub enforced_in_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordRedactionDigestPreview {
    pub id: &'static str,
    pub hashed_fields: Vec<&'static str>,
    pub hash_only: bool,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordLocalViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordIntakeDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_record_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceRecordIntakePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub acceptance_record_persisted: bool,
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

pub fn hepta_work_graph_persistence_acceptance_record_intake_preview_report()
-> WorkGraphPersistenceAcceptanceRecordIntakePreviewReport {
    let record_templates = work_graph_persistence_acceptance_record_templates();
    let validation_denials = work_graph_persistence_acceptance_record_validation_denials();
    let intake_guards = work_graph_persistence_acceptance_record_intake_guards();
    let redaction_digests = work_graph_persistence_acceptance_record_redaction_digests();
    let local_views = work_graph_persistence_acceptance_record_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_record_intake_durable_identity_evidence();
    let invariants = work_graph_persistence_acceptance_record_invariants();

    WorkGraphPersistenceAcceptanceRecordIntakePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_INTAKE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_INTAKE_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_record_intake_preview_no_recording",
        record_template_count: record_templates.len(),
        validation_denial_count: validation_denials.len(),
        intake_guard_count: intake_guards.len(),
        redaction_digest_count: redaction_digests.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_persistence_acceptance_record_intake_required_prior_gates(
        ),
        record_templates,
        validation_denials,
        intake_guards,
        redaction_digests,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_INTAKE_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_record_receipt_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceAcceptanceRecordIntakePreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_record_intake_required_prior_gates() -> Vec<&'static str> {
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_record_intake_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_record_template_ids() -> Vec<&'static str> {
    vec![
        "trusted_operator_acceptance_record",
        "approval_decision_record",
        "live_persistence_enablement_record",
        "rollback_quarantine_owner_attestation",
        "release_publication_owner_attestation",
        "external_delivery_consent_record",
    ]
}

pub fn work_graph_persistence_acceptance_record_templates()
-> Vec<WorkGraphPersistenceAcceptanceRecordTemplatePreview> {
    vec![
        record_template(
            "trusted_operator_acceptance_record",
            vec![
                "operator_acceptance_authority",
                "approval_recording_authority",
            ],
        ),
        record_template(
            "approval_decision_record",
            vec!["approval_recording_authority", "live_persistence_authority"],
        ),
        record_template(
            "live_persistence_enablement_record",
            vec!["live_persistence_authority", "wal_checkpoint_authority"],
        ),
        record_template(
            "rollback_quarantine_owner_attestation",
            vec!["wal_checkpoint_authority", "enforcement_rollout_authority"],
        ),
        record_template(
            "release_publication_owner_attestation",
            vec!["release_publication_authority"],
        ),
        record_template(
            "external_delivery_consent_record",
            vec!["external_delivery_authority"],
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_validation_denials()
-> Vec<WorkGraphPersistenceAcceptanceRecordValidationDenialPreview> {
    let record_ids = work_graph_persistence_acceptance_record_template_ids();

    vec![
        validation_denial(
            "durable_identity_evidence_missing",
            record_ids.clone(),
            "acceptance record intake cannot continue without durable identity evidence",
        ),
        validation_denial(
            "missing_trusted_operator_identity_hash",
            record_ids.clone(),
            "trusted operator identity hash is required before acceptance intake can continue",
        ),
        validation_denial(
            "missing_explicit_authority_grant",
            record_ids.clone(),
            "record lacks explicit authority grant for each requested WorkGraph capability",
        ),
        validation_denial(
            "capability_scope_mismatch",
            record_ids.clone(),
            "record capability scope does not match blocked authority surfaces",
        ),
        validation_denial(
            "approval_ledger_write_attempted",
            record_ids.clone(),
            "acceptance intake cannot write approval ledger records",
        ),
        validation_denial(
            "live_persistence_enablement_attempted",
            record_ids.clone(),
            "acceptance intake cannot enable persistence, WAL, checkpoints, or rollout",
        ),
        validation_denial(
            "release_publication_attempted",
            record_ids.clone(),
            "acceptance intake cannot publish release status or artifacts",
        ),
        validation_denial(
            "external_delivery_attempted",
            record_ids.clone(),
            "acceptance intake cannot send receipts or acknowledgements externally",
        ),
        validation_denial(
            "expired_or_revoked_acceptance_scope",
            record_ids,
            "expired, superseded, revoked, or digest-mismatched acceptance scope blocks intake",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_intake_guards()
-> Vec<WorkGraphPersistenceAcceptanceRecordIntakeGuardPreview> {
    vec![
        intake_guard(
            "acceptance_record_persistence_guard",
            "acceptance_record_persisted",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "acceptanceRecordHash",
                "persistenceDenied",
                "sideEffectHash",
            ]),
        ),
        intake_guard(
            "approval_recording_guard",
            "approval_recorded",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "approvalLedgerId",
                "recordingDenied",
                "operatorAcceptanceRecorded",
            ]),
        ),
        intake_guard(
            "live_execution_guard",
            "live_persistence_enabled",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "featureFlagStillOff",
                "liveExecutionDenied",
                "zeroWriteProofHash",
            ]),
        ),
        intake_guard(
            "rollout_guard",
            "rollout_started",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "rolloutStageZeroTraffic",
                "trafficRouted",
                "enforcementDisabled",
            ]),
        ),
        intake_guard(
            "release_publication_guard",
            "release_published",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "artifactWriteDenied",
            ]),
        ),
        intake_guard(
            "external_delivery_guard",
            "external_send_performed",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "externalDeliveryDenied",
                "recipientScopeDenied",
                "channelSendDenied",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_redaction_digests()
-> Vec<WorkGraphPersistenceAcceptanceRecordRedactionDigestPreview> {
    vec![
        redaction_digest(
            "durable_identity_digest",
            work_graph_persistence_acceptance_record_intake_durable_identity_field_ids(),
        ),
        redaction_digest(
            "operator_identity_digest",
            vec!["trustedOperatorIdHash", "operatorScopeHash"],
        ),
        redaction_digest(
            "authority_scope_digest",
            vec!["acceptedCapabilityIds", "explicitAuthorityGrant"],
        ),
        redaction_digest(
            "approval_decision_digest",
            vec!["approvalLedgerId", "decisionHash", "approverScopeHash"],
        ),
        redaction_digest(
            "rollback_quarantine_digest",
            vec!["rollbackOwnerHash", "quarantineOwnerHash", "killSwitchId"],
        ),
        redaction_digest(
            "publication_delivery_digest",
            vec![
                "releaseOwnerHash",
                "recipientScopeHash",
                "redactionPolicyHash",
            ],
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_local_views()
-> Vec<WorkGraphPersistenceAcceptanceRecordLocalViewPreview> {
    vec![
        local_view(
            "operator_acceptance_record_missing_fields_view",
            "operator",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "recordTemplateId",
                "missingFieldIds",
                "validationDenialIds",
                "acceptanceDenied",
            ]),
        ),
        local_view(
            "auditor_acceptance_record_digest_view",
            "auditor",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "recordDigestHash",
                "redactionDigestIds",
                "intakeGuardIds",
                "sideEffectHash",
            ]),
        ),
        local_view(
            "release_owner_acceptance_record_denial_view",
            "release_owner",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "requiredRecordIds",
            ]),
        ),
        local_view(
            "runtime_acceptance_record_zero_effect_view",
            "system",
            with_acceptance_record_intake_durable_identity_fields(vec![
                "operatorAcceptanceRecorded",
                "approvalRecorded",
                "authorityGranted",
                "livePersistenceEnabled",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_record_intake_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceRecordIntakeDurableIdentityEvidencePreview {
    WorkGraphPersistenceAcceptanceRecordIntakeDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_record_intake_durable_identity_field_ids(),
        required_for_record_ids: work_graph_persistence_acceptance_record_template_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_record_invariants()
-> Vec<WorkGraphPersistenceAcceptanceRecordInvariantPreview> {
    vec![
        invariant(
            "acceptance_record_intake_requires_durable_identity_evidence",
            "record intake templates require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "acceptance_records_are_templates_only",
            "record templates are declared but not accepted, recorded, or persisted",
        ),
        invariant(
            "acceptance_record_validation_denies_all_live_effects",
            "validation denials block authority, approval recording, persistence, rollout, release, and delivery",
        ),
        invariant(
            "acceptance_record_digests_are_hash_only",
            "operator identity, scope, approval, rollback, publication, and delivery fields stay hash-only",
        ),
        invariant(
            "acceptance_record_intake_views_are_local_only",
            "operator, auditor, release-owner, and system intake views cannot be sent externally",
        ),
        invariant(
            "acceptance_record_intake_requires_prior_authority_blocker",
            "intake cannot run unless the acceptance authority blocker gate is present",
        ),
        invariant(
            "acceptance_record_intake_preview_has_no_side_effects",
            "this gate cannot record acceptance, grant authority, enable live execution, publish, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceAcceptanceRecordIntakePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            acceptance_record_persisted: false,
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

fn record_template(
    id: &'static str,
    target_authority_surface_ids: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceRecordTemplatePreview {
    WorkGraphPersistenceAcceptanceRecordTemplatePreview {
        id,
        target_authority_surface_ids,
        required_fields: with_acceptance_record_intake_durable_identity_fields(vec![
            "recordTemplateId",
            "trustedOperatorIdHash",
            "operatorScopeHash",
            "acceptedCapabilityIds",
            "explicitAuthorityGrant",
            "expiresAtUnixMs",
            "revocationStatus",
            "sideEffectDenialHash",
        ]),
        redaction_state: "hash_only_redacted",
        present_in_preview: false,
        accepted_in_preview: false,
        recording_enabled: false,
    }
}

fn with_acceptance_record_intake_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged = work_graph_persistence_acceptance_record_intake_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn validation_denial(
    id: &'static str,
    applies_to_record_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceRecordValidationDenialPreview {
    WorkGraphPersistenceAcceptanceRecordValidationDenialPreview {
        id,
        applies_to_record_ids,
        reason,
        blocks_record_acceptance: true,
    }
}

fn intake_guard(
    id: &'static str,
    blocked_effect: &'static str,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceRecordIntakeGuardPreview {
    WorkGraphPersistenceAcceptanceRecordIntakeGuardPreview {
        id,
        blocked_effect,
        required_evidence_fields,
        enforced_in_preview: true,
    }
}

fn redaction_digest(
    id: &'static str,
    hashed_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceRecordRedactionDigestPreview {
    WorkGraphPersistenceAcceptanceRecordRedactionDigestPreview {
        id,
        hashed_fields,
        hash_only: true,
        required: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceRecordLocalViewPreview {
    WorkGraphPersistenceAcceptanceRecordLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceRecordInvariantPreview {
    WorkGraphPersistenceAcceptanceRecordInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_record_intake_declares_non_accepting_templates() {
        let report = hepta_work_graph_persistence_acceptance_record_intake_preview_report();
        let template_ids = report
            .record_templates
            .iter()
            .map(|template| template.id)
            .collect::<Vec<_>>();

        assert_eq!(
            template_ids,
            [
                "trusted_operator_acceptance_record",
                "approval_decision_record",
                "live_persistence_enablement_record",
                "rollback_quarantine_owner_attestation",
                "release_publication_owner_attestation",
                "external_delivery_consent_record",
            ]
        );
        assert_eq!(report.record_template_count, 6);
        assert!(report.record_templates.iter().all(|template| {
            template.redaction_state == "hash_only_redacted"
                && !template.present_in_preview
                && !template.accepted_in_preview
                && !template.recording_enabled
                && template.required_fields.len() >= 15
                && template.required_fields.contains(&"workflow_id")
                && template.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn acceptance_record_intake_blocks_invalid_records() {
        let report = hepta_work_graph_persistence_acceptance_record_intake_preview_report();

        assert_eq!(report.validation_denial_count, 9);
        assert!(report.validation_denials.iter().all(|denial| {
            denial.blocks_record_acceptance && denial.applies_to_record_ids.len() == 6
        }));
    }

    #[test]
    fn acceptance_record_intake_guards_every_side_effect() {
        let report = hepta_work_graph_persistence_acceptance_record_intake_preview_report();
        let guard_ids = report
            .intake_guards
            .iter()
            .map(|guard| guard.id)
            .collect::<Vec<_>>();

        assert_eq!(
            guard_ids,
            [
                "acceptance_record_persistence_guard",
                "approval_recording_guard",
                "live_execution_guard",
                "rollout_guard",
                "release_publication_guard",
                "external_delivery_guard",
            ]
        );
        assert_eq!(report.intake_guard_count, 6);
        assert!(report.intake_guards.iter().all(|guard| {
            guard.enforced_in_preview
                && guard.required_evidence_fields.len() >= 10
                && guard.required_evidence_fields.contains(&"workflow_id")
                && guard.required_evidence_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn acceptance_record_intake_keeps_digests_hash_only_and_views_local() {
        let report = hepta_work_graph_persistence_acceptance_record_intake_preview_report();

        assert_eq!(report.redaction_digest_count, 6);
        assert!(
            report
                .redaction_digests
                .iter()
                .all(|digest| digest.hash_only
                    && digest.required
                    && digest.hashed_fields.len() >= 2)
        );
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
    }

    #[test]
    fn acceptance_record_intake_requires_durable_identity_evidence() {
        let report = hepta_work_graph_persistence_acceptance_record_intake_preview_report();

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
                .required_for_record_ids
                .len(),
            6
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn acceptance_record_intake_requires_authority_blocker_gate() {
        let report = hepta_work_graph_persistence_acceptance_record_intake_preview_report();

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
                "hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate",
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_RECORD_INTAKE_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn acceptance_record_intake_has_no_side_effects() {
        let report = hepta_work_graph_persistence_acceptance_record_intake_preview_report();

        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceAcceptanceRecordIntakePreviewSideEffects::none()
        );
        assert!(report.ready_for_acceptance_record_receipt_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
