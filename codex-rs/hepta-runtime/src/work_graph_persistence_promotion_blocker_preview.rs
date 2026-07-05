use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_PROMOTION_BLOCKER_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_promotion_blocker_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_PROMOTION_BLOCKER_SCHEMA_VERSION: &str =
    "work_graph_persistence_promotion_blocker_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_PROMOTION_BLOCKER_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistencePromotionBlockerPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub promotion_blocker_count: usize,
    pub release_denial_count: usize,
    pub operator_acknowledgement_count: usize,
    pub rollback_quarantine_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub promotion_blockers: Vec<WorkGraphPersistencePromotionBlockerPreview>,
    pub release_denials: Vec<WorkGraphPersistenceReleaseDenialPreview>,
    pub operator_acknowledgements: Vec<WorkGraphPersistenceOperatorAcknowledgementPreview>,
    pub rollback_quarantines: Vec<WorkGraphPersistenceRollbackQuarantinePreview>,
    pub durable_identity_evidence:
        WorkGraphPersistencePromotionBlockerDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistencePromotionBlockerInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_shadow_live_readback_comparison_preview: bool,
    pub ready_for_persistence_promotion: bool,
    pub ready_for_release_publication: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistencePromotionBlockerPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistencePromotionBlockerPreview {
    pub id: &'static str,
    pub applies_to_target_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub required_evidence_fields: Vec<&'static str>,
    pub blocks_persistence_promotion: bool,
    pub blocks_live_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceReleaseDenialPreview {
    pub id: &'static str,
    pub target_channel: &'static str,
    pub reason: &'static str,
    pub applies_to_blocker_ids: Vec<&'static str>,
    pub blocks_release: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorAcknowledgementPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub currently_satisfied: bool,
    pub external_delivery_enabled: bool,
    pub approval_recorded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceRollbackQuarantinePreview {
    pub id: &'static str,
    pub trigger_blocker_id: &'static str,
    pub quarantine_scope: &'static str,
    pub kill_switch_id: &'static str,
    pub armed_in_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistencePromotionBlockerDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_promotion_target_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistencePromotionBlockerInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistencePromotionBlockerPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub receipt_persisted: bool,
    pub promotion_performed: bool,
    pub release_published: bool,
    pub operator_acknowledgement_recorded: bool,
    pub rollback_performed: bool,
    pub quarantine_performed: bool,
    pub feature_flag_mutated: bool,
    pub live_traffic_routed: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub scheduler_cutover_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_promotion_blocker_preview_report()
-> WorkGraphPersistencePromotionBlockerPreviewReport {
    let promotion_blockers = work_graph_persistence_promotion_blockers();
    let release_denials = work_graph_persistence_release_denials();
    let operator_acknowledgements = work_graph_persistence_operator_acknowledgements();
    let rollback_quarantines = work_graph_persistence_rollback_quarantines();
    let durable_identity_evidence =
        work_graph_persistence_promotion_blocker_durable_identity_evidence();
    let invariants = work_graph_persistence_promotion_blocker_invariants();

    WorkGraphPersistencePromotionBlockerPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_PROMOTION_BLOCKER_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_PROMOTION_BLOCKER_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_promotion_blocker_preview_no_promotion",
        promotion_blocker_count: promotion_blockers.len(),
        release_denial_count: release_denials.len(),
        operator_acknowledgement_count: operator_acknowledgements.len(),
        rollback_quarantine_count: rollback_quarantines.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_persistence_promotion_blocker_required_prior_gates(),
        promotion_blockers,
        release_denials,
        operator_acknowledgements,
        rollback_quarantines,
        durable_identity_evidence,
        invariants,
        recommended_next_gate: WORK_GRAPH_PERSISTENCE_PROMOTION_BLOCKER_RECOMMENDED_NEXT_GATE,
        ready_for_shadow_live_readback_comparison_preview: true,
        ready_for_persistence_promotion: false,
        ready_for_release_publication: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistencePromotionBlockerPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_promotion_blocker_required_prior_gates() -> Vec<&'static str> {
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_promotion_target_ids() -> Vec<&'static str> {
    vec![
        "store_persistence_promotion",
        "wal_append_promotion",
        "checkpoint_write_promotion",
        "readback_receipt_persistence_promotion",
        "idempotency_index_promotion",
        "replay_execution_promotion",
        "external_release_publication_promotion",
    ]
}

pub fn work_graph_persistence_promotion_blocker_durable_identity_field_ids() -> Vec<&'static str> {
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

pub fn work_graph_persistence_promotion_blockers()
-> Vec<WorkGraphPersistencePromotionBlockerPreview> {
    let all_targets = work_graph_persistence_promotion_target_ids();
    let persistent_targets = vec![
        "store_persistence_promotion",
        "wal_append_promotion",
        "checkpoint_write_promotion",
        "readback_receipt_persistence_promotion",
        "idempotency_index_promotion",
        "replay_execution_promotion",
    ];

    vec![
        promotion_blocker(
            "missing_canary_receipt_digest",
            all_targets.clone(),
            "canary readback receipt digest is missing or not hash-only",
            with_promotion_blocker_durable_identity_fields(vec![
                "priorGateReportHash",
                "receiptHash",
                "redactionState",
            ]),
        ),
        promotion_blocker(
            "durable_identity_evidence_missing",
            all_targets.clone(),
            "promotion target is missing workflow, run, step, checkpoint, replay, rollback, or receipt identity",
            work_graph_persistence_promotion_blocker_durable_identity_field_ids(),
        ),
        promotion_blocker(
            "operator_acknowledgement_missing",
            all_targets.clone(),
            "operator acknowledgement packet has not been reviewed",
            with_promotion_blocker_durable_identity_fields(vec![
                "operatorScopeHash",
                "acknowledgementHash",
                "expiresAtUnixMs",
            ]),
        ),
        promotion_blocker(
            "release_denial_matrix_missing",
            all_targets.clone(),
            "release denial matrix has not been materialized for the target",
            with_promotion_blocker_durable_identity_fields(vec![
                "denialMatrixHash",
                "releaseScope",
                "targetChannel",
            ]),
        ),
        promotion_blocker(
            "rollback_quarantine_not_armed",
            persistent_targets.clone(),
            "rollback and quarantine switches are not armed for persistence promotion",
            with_promotion_blocker_durable_identity_fields(vec![
                "rollbackPlanId",
                "killSwitchId",
                "quarantineScope",
            ]),
        ),
        promotion_blocker(
            "zero_write_or_traffic_receipt_absent",
            persistent_targets,
            "receipt does not prove zero live traffic and zero persisted writes",
            with_promotion_blocker_durable_identity_fields(vec![
                "zeroTrafficProof",
                "zeroWriteProof",
                "sideEffectHash",
            ]),
        ),
        promotion_blocker(
            "canary_scope_exceeds_backend_lane",
            all_targets.clone(),
            "canary evidence is not scoped to the hepta-backend lane",
            with_promotion_blocker_durable_identity_fields(vec![
                "laneId",
                "agentId",
                "cargoTargetDirHash",
            ]),
        ),
        promotion_blocker(
            "external_delivery_policy_missing",
            vec!["external_release_publication_promotion"],
            "external delivery policy and readback gate are absent",
            with_promotion_blocker_durable_identity_fields(vec![
                "deliveryPolicyHash",
                "externalTargetScope",
                "readbackGate",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_release_denials() -> Vec<WorkGraphPersistenceReleaseDenialPreview> {
    vec![
        release_denial(
            "deny_store_persistence_release",
            "durable_work_graph_store",
            "store persistence cannot release while canary receipt, operator acknowledgement, or rollback evidence is missing",
            vec![
                "missing_canary_receipt_digest",
                "durable_identity_evidence_missing",
                "operator_acknowledgement_missing",
                "rollback_quarantine_not_armed",
                "zero_write_or_traffic_receipt_absent",
            ],
        ),
        release_denial(
            "deny_wal_append_release",
            "work_graph_wal",
            "WAL append cannot release without hash-only receipt and zero-write proof",
            vec![
                "missing_canary_receipt_digest",
                "durable_identity_evidence_missing",
                "rollback_quarantine_not_armed",
                "zero_write_or_traffic_receipt_absent",
            ],
        ),
        release_denial(
            "deny_checkpoint_release",
            "work_graph_checkpoint",
            "checkpoint persistence cannot release without release denial matrix and quarantine evidence",
            vec![
                "durable_identity_evidence_missing",
                "release_denial_matrix_missing",
                "rollback_quarantine_not_armed",
                "zero_write_or_traffic_receipt_absent",
            ],
        ),
        release_denial(
            "deny_readback_receipt_release",
            "readback_receipt_store",
            "readback receipt persistence cannot release while receipts remain preview-only",
            vec![
                "missing_canary_receipt_digest",
                "durable_identity_evidence_missing",
                "operator_acknowledgement_missing",
                "release_denial_matrix_missing",
            ],
        ),
        release_denial(
            "deny_replay_execution_release",
            "work_graph_replay_executor",
            "replay execution cannot release without lane-bound canary scope and rollback switches",
            vec![
                "durable_identity_evidence_missing",
                "canary_scope_exceeds_backend_lane",
                "rollback_quarantine_not_armed",
                "zero_write_or_traffic_receipt_absent",
            ],
        ),
        release_denial(
            "deny_external_publication_release",
            "external_delivery",
            "external publication cannot release without a separate delivery policy and readback gate",
            vec![
                "external_delivery_policy_missing",
                "durable_identity_evidence_missing",
                "operator_acknowledgement_missing",
                "release_denial_matrix_missing",
            ],
        ),
    ]
}

pub fn work_graph_persistence_operator_acknowledgements()
-> Vec<WorkGraphPersistenceOperatorAcknowledgementPreview> {
    vec![
        operator_acknowledgement(
            "operator_promotion_blocker_ack",
            "operator",
            with_promotion_blocker_durable_identity_fields(vec![
                "denialReasonIds",
                "targetIds",
                "receiptHash",
                "nextGate",
            ]),
        ),
        operator_acknowledgement(
            "auditor_digest_ack",
            "auditor",
            with_promotion_blocker_durable_identity_fields(vec![
                "priorGateReportHash",
                "canaryReceiptHash",
                "releaseDenialHash",
                "redactionState",
            ]),
        ),
        operator_acknowledgement(
            "release_owner_non_acceptance_ack",
            "release_owner",
            with_promotion_blocker_durable_identity_fields(vec![
                "targetChannel",
                "denialMatrixHash",
                "nonAcceptanceReason",
                "expiresAtUnixMs",
            ]),
        ),
        operator_acknowledgement(
            "rollback_owner_quarantine_ack",
            "rollback_owner",
            with_promotion_blocker_durable_identity_fields(vec![
                "rollbackPlanId",
                "killSwitchId",
                "quarantineScope",
                "recoveryOwnerHash",
            ]),
        ),
        operator_acknowledgement(
            "external_delivery_scope_ack",
            "delivery_owner",
            with_promotion_blocker_durable_identity_fields(vec![
                "deliveryPolicyHash",
                "externalTargetScope",
                "readbackGate",
                "externalDeliveryDisabled",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_rollback_quarantines()
-> Vec<WorkGraphPersistenceRollbackQuarantinePreview> {
    vec![
        rollback_quarantine(
            "quarantine_store_persistence_on_missing_receipt",
            "missing_canary_receipt_digest",
            "graph_state_store",
            "kill_work_graph_store_persistence",
        ),
        rollback_quarantine(
            "quarantine_wal_append_on_zero_write_failure",
            "zero_write_or_traffic_receipt_absent",
            "wal_writer",
            "kill_work_graph_wal_append",
        ),
        rollback_quarantine(
            "quarantine_replay_execution_on_lane_scope_failure",
            "canary_scope_exceeds_backend_lane",
            "replay_executor",
            "kill_work_graph_replay_execution",
        ),
        rollback_quarantine(
            "quarantine_release_publication_on_policy_gap",
            "external_delivery_policy_missing",
            "external_release_pipeline",
            "kill_work_graph_external_delivery",
        ),
        rollback_quarantine(
            "quarantine_promotion_on_operator_gap",
            "operator_acknowledgement_missing",
            "promotion_executor",
            "kill_work_graph_promotion_executor",
        ),
        rollback_quarantine(
            "quarantine_promotion_on_durable_identity_gap",
            "durable_identity_evidence_missing",
            "promotion_executor",
            "kill_work_graph_promotion_identity",
        ),
    ]
}

pub fn work_graph_persistence_promotion_blocker_durable_identity_evidence()
-> WorkGraphPersistencePromotionBlockerDurableIdentityEvidencePreview {
    WorkGraphPersistencePromotionBlockerDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: work_graph_persistence_promotion_blocker_durable_identity_field_ids(),
        required_for_promotion_target_ids: work_graph_persistence_promotion_target_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_promotion_blocker_invariants()
-> Vec<WorkGraphPersistencePromotionBlockerInvariantPreview> {
    vec![
        invariant(
            "promotion_blockers_require_durable_identity_evidence",
            "promotion blockers require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "promotion_blocked_after_canary_until_acknowledged",
            "canary readback receipts are evidence only and cannot promote state without operator acknowledgement",
        ),
        invariant(
            "release_denials_are_target_specific",
            "each persistence release target has an explicit denial path before any future release",
        ),
        invariant(
            "operator_acknowledgement_is_non_recording",
            "operator acknowledgement previews are local and do not write approval receipts",
        ),
        invariant(
            "rollback_quarantine_precedes_promotion_execution",
            "promotion execution must be blocked until rollback and quarantine switches are armed",
        ),
        invariant(
            "external_release_has_independent_denial",
            "external delivery and publication cannot inherit persistence or scheduler promotion",
        ),
        invariant(
            "persistence_promotion_blocker_preview_has_no_side_effects",
            "this gate cannot promote, release, persist receipts, record approvals, quarantine state, or send externally",
        ),
    ]
}

impl WorkGraphPersistencePromotionBlockerPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            receipt_persisted: false,
            promotion_performed: false,
            release_published: false,
            operator_acknowledgement_recorded: false,
            rollback_performed: false,
            quarantine_performed: false,
            feature_flag_mutated: false,
            live_traffic_routed: false,
            wal_written: false,
            checkpoint_written: false,
            scheduler_cutover_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn promotion_blocker(
    id: &'static str,
    applies_to_target_ids: Vec<&'static str>,
    trigger: &'static str,
    required_evidence_fields: Vec<&'static str>,
) -> WorkGraphPersistencePromotionBlockerPreview {
    WorkGraphPersistencePromotionBlockerPreview {
        id,
        applies_to_target_ids,
        trigger,
        required_evidence_fields,
        blocks_persistence_promotion: true,
        blocks_live_execution: true,
    }
}

fn with_promotion_blocker_durable_identity_fields(fields: Vec<&'static str>) -> Vec<&'static str> {
    let mut merged = work_graph_persistence_promotion_blocker_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn release_denial(
    id: &'static str,
    target_channel: &'static str,
    reason: &'static str,
    applies_to_blocker_ids: Vec<&'static str>,
) -> WorkGraphPersistenceReleaseDenialPreview {
    WorkGraphPersistenceReleaseDenialPreview {
        id,
        target_channel,
        reason,
        applies_to_blocker_ids,
        blocks_release: true,
    }
}

fn operator_acknowledgement(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceOperatorAcknowledgementPreview {
    WorkGraphPersistenceOperatorAcknowledgementPreview {
        id,
        audience,
        required_fields,
        currently_satisfied: false,
        external_delivery_enabled: false,
        approval_recorded: false,
    }
}

fn rollback_quarantine(
    id: &'static str,
    trigger_blocker_id: &'static str,
    quarantine_scope: &'static str,
    kill_switch_id: &'static str,
) -> WorkGraphPersistenceRollbackQuarantinePreview {
    WorkGraphPersistenceRollbackQuarantinePreview {
        id,
        trigger_blocker_id,
        quarantine_scope,
        kill_switch_id,
        armed_in_preview: true,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistencePromotionBlockerInvariantPreview {
    WorkGraphPersistencePromotionBlockerInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistence_promotion_blocker_preview_declares_blockers_after_canary_receipt() {
        let report = hepta_work_graph_persistence_promotion_blocker_preview_report();
        let blocker_ids = report
            .promotion_blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_ids,
            [
                "missing_canary_receipt_digest",
                "durable_identity_evidence_missing",
                "operator_acknowledgement_missing",
                "release_denial_matrix_missing",
                "rollback_quarantine_not_armed",
                "zero_write_or_traffic_receipt_absent",
                "canary_scope_exceeds_backend_lane",
                "external_delivery_policy_missing",
            ]
        );
        assert_eq!(report.promotion_blocker_count, 8);
        assert!(report.promotion_blockers.iter().all(|blocker| {
            blocker.blocks_persistence_promotion
                && blocker.blocks_live_execution
                && !blocker.applies_to_target_ids.is_empty()
                && blocker.required_evidence_fields.len() >= 3
                && work_graph_persistence_promotion_blocker_durable_identity_field_ids()
                    .iter()
                    .all(|field| blocker.required_evidence_fields.contains(field))
        }));
    }

    #[test]
    fn persistence_promotion_blocker_preview_declares_release_denials() {
        let report = hepta_work_graph_persistence_promotion_blocker_preview_report();
        let denial_ids = report
            .release_denials
            .iter()
            .map(|denial| denial.id)
            .collect::<Vec<_>>();

        assert_eq!(
            denial_ids,
            [
                "deny_store_persistence_release",
                "deny_wal_append_release",
                "deny_checkpoint_release",
                "deny_readback_receipt_release",
                "deny_replay_execution_release",
                "deny_external_publication_release",
            ]
        );
        assert_eq!(report.release_denial_count, 6);
        assert!(report.release_denials.iter().all(|denial| {
            denial.blocks_release
                && denial.applies_to_blocker_ids.len() >= 3
                && denial
                    .applies_to_blocker_ids
                    .contains(&"durable_identity_evidence_missing")
        }));
    }

    #[test]
    fn persistence_promotion_blocker_preview_requires_non_recording_acknowledgements() {
        let report = hepta_work_graph_persistence_promotion_blocker_preview_report();
        let acknowledgement_ids = report
            .operator_acknowledgements
            .iter()
            .map(|acknowledgement| acknowledgement.id)
            .collect::<Vec<_>>();

        assert_eq!(
            acknowledgement_ids,
            [
                "operator_promotion_blocker_ack",
                "auditor_digest_ack",
                "release_owner_non_acceptance_ack",
                "rollback_owner_quarantine_ack",
                "external_delivery_scope_ack",
            ]
        );
        assert_eq!(report.operator_acknowledgement_count, 5);
        assert!(
            report
                .operator_acknowledgements
                .iter()
                .all(|acknowledgement| {
                    !acknowledgement.currently_satisfied
                        && !acknowledgement.external_delivery_enabled
                        && !acknowledgement.approval_recorded
                        && acknowledgement.required_fields.len() >= 4
                        && acknowledgement.required_fields.contains(&"workflow_id")
                        && acknowledgement.required_fields.contains(&"receipt_hash")
                })
        );
    }

    #[test]
    fn persistence_promotion_blocker_preview_declares_quarantine_and_no_side_effects() {
        let report = hepta_work_graph_persistence_promotion_blocker_preview_report();

        assert_eq!(report.rollback_quarantine_count, 6);
        assert!(
            report
                .rollback_quarantines
                .iter()
                .all(|quarantine| quarantine.armed_in_preview)
        );
        assert_eq!(
            report.side_effects,
            WorkGraphPersistencePromotionBlockerPreviewSideEffects::none()
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_promotion_blocker_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_promotion_target_ids,
            work_graph_persistence_promotion_target_ids()
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert!(report.ready_for_shadow_live_readback_comparison_preview);
        assert!(!report.ready_for_persistence_promotion);
        assert!(!report.ready_for_release_publication);
        assert!(!report.ready_for_live_persistence);
    }

    #[test]
    fn persistence_promotion_blocker_preview_requires_canary_readback_receipt_gate() {
        let report = hepta_work_graph_persistence_promotion_blocker_preview_report();

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
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_PROMOTION_BLOCKER_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.invariant_count, 7);
    }
}
