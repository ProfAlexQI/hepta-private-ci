use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_BLOCKER_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_BLOCKER_SCHEMA_VERSION: &str =
    "work_graph_persistence_acceptance_effect_application_blocker_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_BLOCKER_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectApplicationBlockerPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub effect_surface_count: usize,
    pub effect_blocker_count: usize,
    pub apply_guard_count: usize,
    pub rollback_quarantine_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub effect_surfaces: Vec<WorkGraphPersistenceAcceptanceEffectSurfacePreview>,
    pub effect_blockers: Vec<WorkGraphPersistenceAcceptanceEffectBlockerPreview>,
    pub apply_guards: Vec<WorkGraphPersistenceAcceptanceEffectApplyGuardPreview>,
    pub rollback_quarantines: Vec<WorkGraphPersistenceAcceptanceEffectRollbackQuarantinePreview>,
    pub local_views: Vec<WorkGraphPersistenceAcceptanceEffectLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceAcceptanceEffectApplicationDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceAcceptanceEffectInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceAcceptanceEffectApplicationBlockerPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectSurfacePreview {
    pub id: &'static str,
    pub requested_effect: &'static str,
    pub required_fields: Vec<&'static str>,
    pub effect_applied: bool,
    pub persistence_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectBlockerPreview {
    pub id: &'static str,
    pub applies_to_effect_surface_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_effect_application: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectApplyGuardPreview {
    pub id: &'static str,
    pub from_signal: &'static str,
    pub blocked_effect_surface_id: &'static str,
    pub required_denial_fields: Vec<&'static str>,
    pub blocks_apply: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectRollbackQuarantinePreview {
    pub id: &'static str,
    pub guarded_effect_surface_id: &'static str,
    pub rollback_owner_required: bool,
    pub quarantine_required: bool,
    pub armed_in_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectLocalViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectApplicationDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_effect_surface_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceAcceptanceEffectApplicationBlockerPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub acceptance_record_persisted: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_report()
-> WorkGraphPersistenceAcceptanceEffectApplicationBlockerPreviewReport {
    let effect_surfaces = work_graph_persistence_acceptance_effect_surfaces();
    let effect_blockers = work_graph_persistence_acceptance_effect_blockers();
    let apply_guards = work_graph_persistence_acceptance_effect_apply_guards();
    let rollback_quarantines = work_graph_persistence_acceptance_effect_rollback_quarantines();
    let local_views = work_graph_persistence_acceptance_effect_local_views();
    let durable_identity_evidence =
        work_graph_persistence_acceptance_effect_application_durable_identity_evidence();
    let invariants = work_graph_persistence_acceptance_effect_invariants();

    WorkGraphPersistenceAcceptanceEffectApplicationBlockerPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_BLOCKER_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_BLOCKER_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_blocker_preview_no_apply",
        effect_surface_count: effect_surfaces.len(),
        effect_blocker_count: effect_blockers.len(),
        apply_guard_count: apply_guards.len(),
        rollback_quarantine_count: rollback_quarantines.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_acceptance_effect_application_required_prior_gates(),
        effect_surfaces,
        effect_blockers,
        apply_guards,
        rollback_quarantines,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_BLOCKER_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceAcceptanceEffectApplicationBlockerPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_acceptance_effect_application_required_prior_gates()
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_acceptance_effect_application_durable_identity_field_ids()
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

pub fn work_graph_persistence_acceptance_effect_surface_ids() -> Vec<&'static str> {
    vec![
        "operator_acceptance_recording_effect",
        "approval_ledger_write_effect",
        "authority_grant_effect",
        "graph_state_persistence_effect",
        "wal_checkpoint_write_effect",
        "enforcement_rollout_effect",
        "release_publication_effect",
        "external_delivery_effect",
    ]
}

pub fn work_graph_persistence_acceptance_effect_surfaces()
-> Vec<WorkGraphPersistenceAcceptanceEffectSurfacePreview> {
    vec![
        effect_surface(
            "operator_acceptance_recording_effect",
            "record operator acceptance for persistence authority",
            vec![
                "acceptanceRecordHash",
                "operatorScopeHash",
                "recordingDenialIds",
                "sideEffectHash",
            ],
        ),
        effect_surface(
            "approval_ledger_write_effect",
            "write approval decision into the WorkGraph ledger",
            vec![
                "approvalLedgerId",
                "approvalRecordHash",
                "ledgerWriteDenied",
                "sideEffectHash",
            ],
        ),
        effect_surface(
            "authority_grant_effect",
            "grant WorkGraph persistence authority",
            vec![
                "authoritySurfaceId",
                "authorityGrantHash",
                "authorityDenied",
                "sideEffectHash",
            ],
        ),
        effect_surface(
            "graph_state_persistence_effect",
            "persist WorkGraph state store collections",
            vec![
                "collectionId",
                "stateStorePathHash",
                "persistenceDenied",
                "zeroWriteProofHash",
            ],
        ),
        effect_surface(
            "wal_checkpoint_write_effect",
            "write WAL entries or checkpoints",
            vec![
                "walScopeHash",
                "checkpointScopeHash",
                "writeDenied",
                "idempotencyGuardHash",
            ],
        ),
        effect_surface(
            "enforcement_rollout_effect",
            "start enforcement rollout or traffic ramp",
            vec![
                "rolloutStageId",
                "trafficRampId",
                "enforcementDisabled",
                "zeroTrafficProofHash",
            ],
        ),
        effect_surface(
            "release_publication_effect",
            "publish release status or artifact availability",
            vec![
                "releaseOwnerHash",
                "publicationPolicyHash",
                "publicationDenied",
                "artifactWriteDenied",
            ],
        ),
        effect_surface(
            "external_delivery_effect",
            "send acceptance, rollout, or release receipts externally",
            vec![
                "deliveryChannelId",
                "recipientScopeHash",
                "externalDeliveryDenied",
                "channelSendDenied",
            ],
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_blockers()
-> Vec<WorkGraphPersistenceAcceptanceEffectBlockerPreview> {
    let all_surfaces = work_graph_persistence_acceptance_effect_surface_ids();

    vec![
        effect_blocker(
            "durable_identity_evidence_missing",
            all_surfaces.clone(),
            "effect application cannot proceed without durable identity evidence",
        ),
        effect_blocker(
            "accepted_looking_record_is_not_apply_authority",
            all_surfaces.clone(),
            "accepted-looking records are still preview evidence and cannot apply side effects",
        ),
        effect_blocker(
            "receipt_acknowledgement_is_not_apply_authority",
            all_surfaces.clone(),
            "receipt acknowledgement visibility cannot grant side-effect application authority",
        ),
        effect_blocker(
            "approval_recording_precondition_absent",
            vec![
                "operator_acceptance_recording_effect",
                "approval_ledger_write_effect",
            ],
            "approval and acceptance recording preconditions are absent",
        ),
        effect_blocker(
            "authority_grant_precondition_absent",
            vec!["authority_grant_effect", "graph_state_persistence_effect"],
            "explicit authority grant preconditions are absent",
        ),
        effect_blocker(
            "persistence_feature_flag_still_disabled",
            vec![
                "graph_state_persistence_effect",
                "wal_checkpoint_write_effect",
                "enforcement_rollout_effect",
            ],
            "persistence feature flags remain disabled",
        ),
        effect_blocker(
            "zero_write_or_traffic_receipt_required",
            vec![
                "graph_state_persistence_effect",
                "wal_checkpoint_write_effect",
                "enforcement_rollout_effect",
            ],
            "zero-write and zero-traffic proofs are still required",
        ),
        effect_blocker(
            "rollback_quarantine_not_armed_for_apply",
            vec!["wal_checkpoint_write_effect", "enforcement_rollout_effect"],
            "rollback and quarantine owners are not armed for effect application",
        ),
        effect_blocker(
            "release_publication_policy_not_accepted",
            vec!["release_publication_effect", "external_delivery_effect"],
            "release publication policy is not accepted",
        ),
        effect_blocker(
            "external_delivery_consent_absent",
            vec!["external_delivery_effect"],
            "external delivery consent is absent",
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_apply_guards()
-> Vec<WorkGraphPersistenceAcceptanceEffectApplyGuardPreview> {
    vec![
        apply_guard(
            "record_to_approval_recording_guard",
            "accepted_looking_record",
            "operator_acceptance_recording_effect",
            vec![
                "recordingDenied",
                "operatorAcceptanceRecorded",
                "approvalRecorded",
            ],
        ),
        apply_guard(
            "receipt_ack_to_authority_guard",
            "acceptance_receipt_acknowledgement",
            "authority_grant_effect",
            vec![
                "authorityDenied",
                "acknowledgementHash",
                "nonAcceptanceReasonIds",
            ],
        ),
        apply_guard(
            "authority_to_persistence_guard",
            "authority_grant_attempt",
            "graph_state_persistence_effect",
            vec![
                "featureFlagStillOff",
                "persistenceDenied",
                "zeroWriteProofHash",
            ],
        ),
        apply_guard(
            "persistence_to_wal_checkpoint_guard",
            "persistence_apply_attempt",
            "wal_checkpoint_write_effect",
            vec![
                "walWriteDenied",
                "checkpointWriteDenied",
                "idempotencyGuardHash",
            ],
        ),
        apply_guard(
            "persistence_to_rollout_guard",
            "persistence_apply_attempt",
            "enforcement_rollout_effect",
            vec![
                "rolloutStageZeroTraffic",
                "trafficRouted",
                "enforcementDisabled",
            ],
        ),
        apply_guard(
            "rollout_to_release_guard",
            "rollout_apply_attempt",
            "release_publication_effect",
            vec!["releaseDenied", "publicationDenied", "artifactWriteDenied"],
        ),
        apply_guard(
            "release_to_external_delivery_guard",
            "release_publication_attempt",
            "external_delivery_effect",
            vec![
                "externalDeliveryDenied",
                "recipientScopeDenied",
                "channelSendDenied",
            ],
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_rollback_quarantines()
-> Vec<WorkGraphPersistenceAcceptanceEffectRollbackQuarantinePreview> {
    vec![
        rollback_quarantine(
            "graph_state_persistence_quarantine",
            "graph_state_persistence_effect",
        ),
        rollback_quarantine(
            "wal_checkpoint_write_quarantine",
            "wal_checkpoint_write_effect",
        ),
        rollback_quarantine(
            "enforcement_rollout_quarantine",
            "enforcement_rollout_effect",
        ),
        rollback_quarantine(
            "release_publication_quarantine",
            "release_publication_effect",
        ),
        rollback_quarantine("external_delivery_quarantine", "external_delivery_effect"),
    ]
}

pub fn work_graph_persistence_acceptance_effect_local_views()
-> Vec<WorkGraphPersistenceAcceptanceEffectLocalViewPreview> {
    vec![
        local_view(
            "operator_effect_application_blocker_view",
            "operator",
            with_acceptance_effect_application_durable_identity_fields(vec![
                "effectSurfaceId",
                "effectBlocked",
                "blockerIds",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_effect_application_denial_view",
            "auditor",
            with_acceptance_effect_application_durable_identity_fields(vec![
                "effectSurfaceId",
                "applyGuardIds",
                "rollbackQuarantineIds",
                "sideEffectHash",
            ]),
        ),
        local_view(
            "release_owner_effect_application_blocker_view",
            "release_owner",
            with_acceptance_effect_application_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "quarantineRequired",
            ]),
        ),
        local_view(
            "runtime_effect_application_zero_effect_view",
            "system",
            with_acceptance_effect_application_durable_identity_fields(vec![
                "authorityGranted",
                "livePersistenceEnabled",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_acceptance_effect_application_durable_identity_evidence()
-> WorkGraphPersistenceAcceptanceEffectApplicationDurableIdentityEvidencePreview {
    WorkGraphPersistenceAcceptanceEffectApplicationDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_acceptance_effect_application_durable_identity_field_ids(),
        required_for_effect_surface_ids: work_graph_persistence_acceptance_effect_surface_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_acceptance_effect_invariants()
-> Vec<WorkGraphPersistenceAcceptanceEffectInvariantPreview> {
    vec![
        invariant(
            "acceptance_effect_application_requires_durable_identity_evidence",
            "effect application blockers require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "accepted_looking_records_cannot_apply_effects",
            "records, receipts, and acknowledgements cannot apply side effects",
        ),
        invariant(
            "approval_and_authority_effects_are_blocked",
            "operator acceptance, approval recording, and authority grant effects remain blocked",
        ),
        invariant(
            "persistence_and_rollout_effects_are_blocked",
            "state persistence, WAL, checkpoints, enforcement, rollout, and traffic effects remain blocked",
        ),
        invariant(
            "release_and_external_delivery_effects_are_blocked",
            "release publication and external delivery effects remain blocked",
        ),
        invariant(
            "rollback_quarantine_required_but_not_armed",
            "rollback and quarantine ownership is required and remains unarmed in preview",
        ),
        invariant(
            "acceptance_effect_application_blocker_preview_has_no_side_effects",
            "this gate cannot record acceptance, grant authority, persist state, start rollout, publish, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceAcceptanceEffectApplicationBlockerPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            acceptance_record_persisted: false,
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

fn effect_surface(
    id: &'static str,
    requested_effect: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectSurfacePreview {
    WorkGraphPersistenceAcceptanceEffectSurfacePreview {
        id,
        requested_effect,
        required_fields: with_acceptance_effect_application_durable_identity_fields(
            required_fields,
        ),
        effect_applied: false,
        persistence_enabled: false,
        external_delivery_enabled: false,
    }
}

fn effect_blocker(
    id: &'static str,
    applies_to_effect_surface_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectBlockerPreview {
    WorkGraphPersistenceAcceptanceEffectBlockerPreview {
        id,
        applies_to_effect_surface_ids,
        reason,
        blocks_effect_application: true,
    }
}

fn apply_guard(
    id: &'static str,
    from_signal: &'static str,
    blocked_effect_surface_id: &'static str,
    required_denial_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectApplyGuardPreview {
    WorkGraphPersistenceAcceptanceEffectApplyGuardPreview {
        id,
        from_signal,
        blocked_effect_surface_id,
        required_denial_fields: with_acceptance_effect_application_durable_identity_fields(
            required_denial_fields,
        ),
        blocks_apply: true,
    }
}

fn with_acceptance_effect_application_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_acceptance_effect_application_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn rollback_quarantine(
    id: &'static str,
    guarded_effect_surface_id: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectRollbackQuarantinePreview {
    WorkGraphPersistenceAcceptanceEffectRollbackQuarantinePreview {
        id,
        guarded_effect_surface_id,
        rollback_owner_required: true,
        quarantine_required: true,
        armed_in_preview: false,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceAcceptanceEffectLocalViewPreview {
    WorkGraphPersistenceAcceptanceEffectLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceAcceptanceEffectInvariantPreview {
    WorkGraphPersistenceAcceptanceEffectInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_effect_application_declares_blocked_surfaces() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_report();
        let surface_ids = report
            .effect_surfaces
            .iter()
            .map(|surface| surface.id)
            .collect::<Vec<_>>();

        assert_eq!(
            surface_ids,
            [
                "operator_acceptance_recording_effect",
                "approval_ledger_write_effect",
                "authority_grant_effect",
                "graph_state_persistence_effect",
                "wal_checkpoint_write_effect",
                "enforcement_rollout_effect",
                "release_publication_effect",
                "external_delivery_effect",
            ]
        );
        assert_eq!(report.effect_surface_count, 8);
        assert!(report.effect_surfaces.iter().all(|surface| {
            !surface.effect_applied
                && !surface.persistence_enabled
                && !surface.external_delivery_enabled
                && surface.required_fields.len() >= 11
                && surface.required_fields.contains(&"workflow_id")
                && surface.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn acceptance_effect_application_blocks_every_effect_path() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_report();

        assert_eq!(report.effect_blocker_count, 10);
        assert!(report.effect_blockers.iter().all(|blocker| {
            blocker.blocks_effect_application && !blocker.applies_to_effect_surface_ids.is_empty()
        }));
        assert_eq!(report.apply_guard_count, 7);
        assert!(report.apply_guards.iter().all(|guard| guard.blocks_apply
            && guard.required_denial_fields.len() >= 10
            && guard.required_denial_fields.contains(&"workflow_id")
            && guard.required_denial_fields.contains(&"receipt_hash")));
    }

    #[test]
    fn acceptance_effect_application_requires_rollback_and_quarantine() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_report();

        assert_eq!(report.rollback_quarantine_count, 5);
        assert!(report.rollback_quarantines.iter().all(|rollback| {
            rollback.rollback_owner_required
                && rollback.quarantine_required
                && !rollback.armed_in_preview
        }));
    }

    #[test]
    fn acceptance_effect_application_requires_receipt_acknowledgement_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert!(report.required_prior_gates.contains(
            &"hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate"
        ));
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_BLOCKER_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn acceptance_effect_application_requires_durable_identity_evidence() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_report();

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
                .required_for_effect_surface_ids
                .len(),
            8
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn acceptance_effect_application_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_report();

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
            report.side_effects,
            WorkGraphPersistenceAcceptanceEffectApplicationBlockerPreviewSideEffects::none()
        );
        assert!(report.ready_for_acceptance_effect_application_denial_receipt_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
