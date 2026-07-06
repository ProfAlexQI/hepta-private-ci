use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION: &str =
    "work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub acknowledgement_contract_count: usize,
    pub non_acceptance_reason_count: usize,
    pub recording_denial_count: usize,
    pub expiry_guard_count: usize,
    pub operator_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub acknowledgement_contracts:
        Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementContractPreview>,
    pub non_acceptance_reasons:
        Vec<WorkGraphPersistenceOperatorReadinessReceiptNonAcceptanceReasonPreview>,
    pub recording_denials:
        Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementRecordingDenialPreview>,
    pub expiry_guards:
        Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementExpiryGuardPreview>,
    pub operator_views: Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementViewPreview>,
    pub durable_identity_evidence:
        WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementDurableIdentityEvidencePreview,
    pub invariants:
        Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_authority_blocker_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementContractPreview {
    pub id: &'static str,
    pub source_readiness_receipt_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acceptance_allowed: bool,
    pub acknowledgement_recording_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptNonAcceptanceReasonPreview {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acceptance: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementRecordingDenialPreview {
    pub id: &'static str,
    pub target_record: &'static str,
    pub reason: &'static str,
    pub blocks_recording: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementExpiryGuardPreview {
    pub id: &'static str,
    pub applies_to_acknowledgement_ids: Vec<&'static str>,
    pub trigger: &'static str,
    pub blocks_acknowledgement: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementDurableIdentityEvidencePreview
{
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_acknowledgement_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub acknowledgement_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub approval_recorded: bool,
    pub authority_granted: bool,
    pub enforcement_enabled: bool,
    pub rollout_started: bool,
    pub traffic_routed: bool,
    pub release_published: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_report()
-> WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementPreviewReport {
    let acknowledgement_contracts =
        work_graph_persistence_operator_readiness_receipt_acknowledgement_contracts();
    let non_acceptance_reasons =
        work_graph_persistence_operator_readiness_receipt_non_acceptance_reasons();
    let recording_denials =
        work_graph_persistence_operator_readiness_receipt_acknowledgement_recording_denials();
    let expiry_guards =
        work_graph_persistence_operator_readiness_receipt_acknowledgement_expiry_guards();
    let operator_views = work_graph_persistence_operator_readiness_receipt_acknowledgement_views();
    let durable_identity_evidence =
        work_graph_persistence_operator_readiness_receipt_acknowledgement_durable_identity_evidence(
        );
    let invariants = work_graph_persistence_operator_readiness_receipt_acknowledgement_invariants();

    WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_ACKNOWLEDGEMENT_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_ACKNOWLEDGEMENT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_operator_readiness_receipt_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_guard_count: expiry_guards.len(),
        operator_view_count: operator_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_persistence_operator_readiness_receipt_acknowledgement_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_guards,
        operator_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_authority_blocker_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_operator_readiness_receipt_acknowledgement_required_prior_gates()
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_acknowledgement_durable_identity_field_ids()
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

pub fn work_graph_persistence_operator_readiness_receipt_acknowledgement_ids() -> Vec<&'static str>
{
    vec![
        "store_persistence_readiness_receipt_acknowledgement",
        "wal_checkpoint_readiness_receipt_acknowledgement",
        "readback_receipt_readiness_receipt_acknowledgement",
        "replay_execution_readiness_receipt_acknowledgement",
        "external_publication_readiness_receipt_acknowledgement",
        "full_rollout_abort_readiness_receipt_acknowledgement",
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_acknowledgement_contracts()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementContractPreview> {
    vec![
        acknowledgement_contract(
            "store_persistence_readiness_receipt_acknowledgement",
            "store_persistence_readiness_receipt",
        ),
        acknowledgement_contract(
            "wal_checkpoint_readiness_receipt_acknowledgement",
            "wal_checkpoint_readiness_receipt",
        ),
        acknowledgement_contract(
            "readback_receipt_readiness_receipt_acknowledgement",
            "readback_receipt_readiness_receipt",
        ),
        acknowledgement_contract(
            "replay_execution_readiness_receipt_acknowledgement",
            "replay_execution_readiness_receipt",
        ),
        acknowledgement_contract(
            "external_publication_readiness_receipt_acknowledgement",
            "external_publication_readiness_receipt",
        ),
        acknowledgement_contract(
            "full_rollout_abort_readiness_receipt_acknowledgement",
            "full_rollout_abort_readiness_receipt",
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_non_acceptance_reasons()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptNonAcceptanceReasonPreview> {
    let acknowledgement_ids =
        work_graph_persistence_operator_readiness_receipt_acknowledgement_ids();

    vec![
        non_acceptance_reason(
            "durable_identity_evidence_missing",
            acknowledgement_ids.clone(),
            "receipt acknowledgement does not include durable identity evidence",
        ),
        non_acceptance_reason(
            "acknowledgement_is_not_operator_acceptance",
            acknowledgement_ids.clone(),
            "receipt acknowledgement only confirms local preview visibility",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_grant_authority",
            acknowledgement_ids.clone(),
            "acknowledgement cannot grant persistence or rollout authority",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_record_approval",
            acknowledgement_ids.clone(),
            "acknowledgement cannot record approval or readiness acceptance",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_enable_live_execution",
            acknowledgement_ids.clone(),
            "acknowledgement cannot enable live persistence, replay, or enforcement",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_release_or_publish",
            acknowledgement_ids.clone(),
            "acknowledgement cannot release artifacts or publish public status",
        ),
        non_acceptance_reason(
            "acknowledgement_cannot_send_externally",
            acknowledgement_ids,
            "acknowledgement cannot send channel or external delivery receipts",
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_acknowledgement_recording_denials()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementRecordingDenialPreview> {
    vec![
        recording_denial(
            "deny_durable_identity_ack_recording",
            "durable_identity_acknowledgement_evidence",
            "acknowledgement recording is blocked without durable identity evidence",
        ),
        recording_denial(
            "deny_operator_received_recording",
            "operator_received_receipt",
            "operator receipt visibility cannot become a recorded acceptance",
        ),
        recording_denial(
            "deny_operator_confirmed_recording",
            "operator_confirmed_receipt",
            "operator confirmation cannot be recorded from preview",
        ),
        recording_denial(
            "deny_readback_ack_recording",
            "readback_ack_receipt",
            "readback acknowledgement cannot persist receipt state",
        ),
        recording_denial(
            "deny_status_ack_recording",
            "status_ack_receipt",
            "status acknowledgement cannot promote rollout status",
        ),
        recording_denial(
            "deny_channel_ack_delivery",
            "channel_delivery_ack",
            "channel acknowledgement delivery is blocked",
        ),
        recording_denial(
            "deny_external_ack_send",
            "external_ack_send",
            "external acknowledgement send is blocked",
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_acknowledgement_expiry_guards()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementExpiryGuardPreview> {
    let acknowledgement_ids =
        work_graph_persistence_operator_readiness_receipt_acknowledgement_ids();

    vec![
        expiry_guard(
            "acknowledgement_expired",
            acknowledgement_ids.clone(),
            "acknowledgement preview expires before receipt readback",
        ),
        expiry_guard(
            "acknowledgement_superseded",
            acknowledgement_ids.clone(),
            "newer acknowledgement packet supersedes this preview",
        ),
        expiry_guard(
            "acknowledgement_scope_revoked",
            acknowledgement_ids.clone(),
            "operator scope is revoked before acknowledgement",
        ),
        expiry_guard(
            "acknowledgement_receipt_digest_mismatch",
            acknowledgement_ids,
            "acknowledgement digest does not match readiness receipt digest",
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_acknowledgement_views()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementViewPreview> {
    vec![
        acknowledgement_view(
            "operator_acknowledgement_non_acceptance_view",
            "operator",
            with_operator_readiness_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementId",
                "receiptId",
                "nonAcceptanceReasonIds",
                "acceptanceDenied",
            ]),
        ),
        acknowledgement_view(
            "auditor_acknowledgement_digest_view",
            "auditor",
            with_operator_readiness_receipt_acknowledgement_durable_identity_fields(vec![
                "acknowledgementHash",
                "receiptHash",
                "recordingDenialIds",
                "sideEffectHash",
            ]),
        ),
        acknowledgement_view(
            "release_owner_acknowledgement_denial_view",
            "release_owner",
            with_operator_readiness_receipt_acknowledgement_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "externalDeliveryDenied",
                "nextGate",
            ]),
        ),
        acknowledgement_view(
            "authority_blocker_preview_view",
            "system",
            with_operator_readiness_receipt_acknowledgement_durable_identity_fields(vec![
                "authorityGranted",
                "approvalRecorded",
                "liveExecutionEnabled",
                "nextGate",
            ]),
        ),
    ]
}

pub fn work_graph_persistence_operator_readiness_receipt_acknowledgement_durable_identity_evidence()
-> WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementDurableIdentityEvidencePreview {
    WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_persistence_operator_readiness_receipt_acknowledgement_durable_identity_field_ids(),
        required_for_acknowledgement_ids:
            work_graph_persistence_operator_readiness_receipt_acknowledgement_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_operator_readiness_receipt_acknowledgement_invariants()
-> Vec<WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementInvariantPreview> {
    vec![
        invariant(
            "operator_readiness_receipt_acknowledgements_require_durable_identity_evidence",
            "readiness receipt acknowledgement contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "acknowledgement_is_non_acceptance",
            "acknowledgement contracts explicitly deny operator acceptance and live authority",
        ),
        invariant(
            "acknowledgement_recording_is_blocked",
            "all acknowledgement receipt recording surfaces are denied in preview",
        ),
        invariant(
            "authority_cannot_derive_from_acknowledgement",
            "acknowledgement cannot grant persistence, enforcement, replay, release, or delivery authority",
        ),
        invariant(
            "expiry_and_digest_guards_block_acknowledgement",
            "expired, superseded, revoked, or mismatched acknowledgement digests block the preview",
        ),
        invariant(
            "acknowledgement_views_are_local_only",
            "operator and auditor acknowledgement views cannot be sent externally",
        ),
        invariant(
            "operator_readiness_receipt_acknowledgement_preview_has_no_side_effects",
            "this gate cannot record acknowledgements, approvals, authority, enforcement, rollout, release, or external sends",
        ),
    ]
}

impl WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            acknowledgement_recorded: false,
            operator_acceptance_recorded: false,
            approval_recorded: false,
            authority_granted: false,
            enforcement_enabled: false,
            rollout_started: false,
            traffic_routed: false,
            release_published: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn acknowledgement_contract(
    id: &'static str,
    source_readiness_receipt_id: &'static str,
) -> WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementContractPreview {
    WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementContractPreview {
        id,
        source_readiness_receipt_id,
        required_fields: with_operator_readiness_receipt_acknowledgement_durable_identity_fields(
            vec![
                "acknowledgementId",
                "receiptId",
                "acknowledgementHash",
                "receiptHash",
                "nonAcceptanceReasonIds",
                "recordingDenied",
                "deliveryDenied",
                "authorityDenied",
                "liveExecutionDenied",
            ],
        ),
        acceptance_allowed: false,
        acknowledgement_recording_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_operator_readiness_receipt_acknowledgement_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_persistence_operator_readiness_receipt_acknowledgement_durable_identity_field_ids(
        );
    merged.extend(fields);
    merged
}

fn non_acceptance_reason(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphPersistenceOperatorReadinessReceiptNonAcceptanceReasonPreview {
    WorkGraphPersistenceOperatorReadinessReceiptNonAcceptanceReasonPreview {
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
) -> WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementRecordingDenialPreview {
    WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementRecordingDenialPreview {
        id,
        target_record,
        reason,
        blocks_recording: true,
    }
}

fn expiry_guard(
    id: &'static str,
    applies_to_acknowledgement_ids: Vec<&'static str>,
    trigger: &'static str,
) -> WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementExpiryGuardPreview {
    WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementExpiryGuardPreview {
        id,
        applies_to_acknowledgement_ids,
        trigger,
        blocks_acknowledgement: true,
    }
}

fn acknowledgement_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementViewPreview {
    WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementInvariantPreview {
    WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acknowledgement_preview_declares_non_accepting_contracts() {
        let report =
            hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_report(
            );
        let acknowledgement_ids = report
            .acknowledgement_contracts
            .iter()
            .map(|acknowledgement| acknowledgement.id)
            .collect::<Vec<_>>();

        assert_eq!(
            acknowledgement_ids,
            [
                "store_persistence_readiness_receipt_acknowledgement",
                "wal_checkpoint_readiness_receipt_acknowledgement",
                "readback_receipt_readiness_receipt_acknowledgement",
                "replay_execution_readiness_receipt_acknowledgement",
                "external_publication_readiness_receipt_acknowledgement",
                "full_rollout_abort_readiness_receipt_acknowledgement",
            ]
        );
        assert_eq!(report.acknowledgement_contract_count, 6);
        assert!(
            report
                .acknowledgement_contracts
                .iter()
                .all(|acknowledgement| {
                    !acknowledgement.acceptance_allowed
                        && !acknowledgement.acknowledgement_recording_enabled
                        && !acknowledgement.external_delivery_enabled
                        && acknowledgement.required_fields.len() >= 16
                        && acknowledgement.required_fields.contains(&"workflow_id")
                        && acknowledgement.required_fields.contains(&"receipt_hash")
                })
        );
    }

    #[test]
    fn acknowledgement_preview_blocks_acceptance_and_recording() {
        let report =
            hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_report(
            );

        assert_eq!(report.non_acceptance_reason_count, 7);
        assert!(report.non_acceptance_reasons.iter().all(|reason| {
            reason.blocks_acceptance && reason.applies_to_acknowledgement_ids.len() == 6
        }));
        assert_eq!(report.recording_denial_count, 7);
        assert!(
            report
                .recording_denials
                .iter()
                .all(|denial| denial.blocks_recording)
        );
    }

    #[test]
    fn acknowledgement_preview_guards_expiry_scope_and_digest() {
        let report =
            hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_report(
            );
        let guard_ids = report
            .expiry_guards
            .iter()
            .map(|guard| guard.id)
            .collect::<Vec<_>>();

        assert_eq!(
            guard_ids,
            [
                "acknowledgement_expired",
                "acknowledgement_superseded",
                "acknowledgement_scope_revoked",
                "acknowledgement_receipt_digest_mismatch",
            ]
        );
        assert_eq!(report.expiry_guard_count, 4);
        assert!(report.expiry_guards.iter().all(|guard| {
            guard.blocks_acknowledgement && guard.applies_to_acknowledgement_ids.len() == 6
        }));
    }

    #[test]
    fn acknowledgement_preview_keeps_views_local() {
        let report =
            hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_report(
            );
        let view_ids = report
            .operator_views
            .iter()
            .map(|view| view.id)
            .collect::<Vec<_>>();

        assert_eq!(
            view_ids,
            [
                "operator_acknowledgement_non_acceptance_view",
                "auditor_acknowledgement_digest_view",
                "release_owner_acknowledgement_denial_view",
                "authority_blocker_preview_view",
            ]
        );
        assert_eq!(report.operator_view_count, 4);
        assert!(
            report
                .operator_views
                .iter()
                .all(|view| !view.external_delivery_enabled
                    && view.required_fields.len() >= 11
                    && view.required_fields.contains(&"workflow_id")
                    && view.required_fields.contains(&"receipt_hash"))
        );
    }

    #[test]
    fn acknowledgement_preview_requires_durable_identity_evidence() {
        let report =
            hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_report(
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
                .required_for_acknowledgement_ids
                .len(),
            6
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
    }

    #[test]
    fn acknowledgement_preview_requires_readiness_receipt_gate() {
        let report =
            hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_report(
            );

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
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_OPERATOR_READINESS_RECEIPT_ACKNOWLEDGEMENT_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn acknowledgement_preview_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_report(
            );

        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceOperatorReadinessReceiptAcknowledgementPreviewSideEffects::none()
        );
        assert!(report.ready_for_acceptance_authority_blocker_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
