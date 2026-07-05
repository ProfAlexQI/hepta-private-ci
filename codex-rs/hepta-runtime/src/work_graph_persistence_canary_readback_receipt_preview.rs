use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_CANARY_READBACK_RECEIPT_PREVIEW_GATE: &str =
    "hepta_work_graph_persistence_canary_readback_receipt_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_CANARY_READBACK_RECEIPT_SCHEMA_VERSION: &str =
    "work_graph_persistence_canary_readback_receipt_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_CANARY_READBACK_RECEIPT_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_persistence_promotion_blocker_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryReadbackReceiptPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub receipt_contract_count: usize,
    pub digest_check_count: usize,
    pub denial_reason_count: usize,
    pub operator_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub receipt_contracts: Vec<WorkGraphPersistenceCanaryReceiptContractPreview>,
    pub digest_checks: Vec<WorkGraphPersistenceCanaryReceiptDigestCheckPreview>,
    pub denial_reasons: Vec<WorkGraphPersistenceCanaryReceiptDenialPreview>,
    pub operator_views: Vec<WorkGraphPersistenceCanaryReceiptOperatorViewPreview>,
    pub durable_identity_evidence: WorkGraphPersistenceCanaryReadbackDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphPersistenceCanaryReadbackReceiptInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_promotion_blocker_preview: bool,
    pub ready_for_receipt_persistence: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphPersistenceCanaryReadbackReceiptPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryReceiptContractPreview {
    pub id: &'static str,
    pub source_dry_run_scenario_id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub redaction_state: &'static str,
    pub persistence_enabled: bool,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryReceiptDigestCheckPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryReceiptDenialPreview {
    pub id: &'static str,
    pub trigger: &'static str,
    pub applies_to_receipt_ids: Vec<&'static str>,
    pub blocks_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryReceiptOperatorViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryReadbackDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_receipt_contract_ids: Vec<&'static str>,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryReadbackReceiptInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphPersistenceCanaryReadbackReceiptPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub receipt_persisted: bool,
    pub readback_performed: bool,
    pub promotion_performed: bool,
    pub feature_flag_mutated: bool,
    pub canary_executed: bool,
    pub live_traffic_routed: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub approval_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_persistence_canary_readback_receipt_preview_report()
-> WorkGraphPersistenceCanaryReadbackReceiptPreviewReport {
    let receipt_contracts = work_graph_persistence_canary_receipt_contracts();
    let digest_checks = work_graph_persistence_canary_receipt_digest_checks();
    let denial_reasons = work_graph_persistence_canary_receipt_denials();
    let operator_views = work_graph_persistence_canary_receipt_operator_views();
    let durable_identity_evidence =
        work_graph_persistence_canary_readback_durable_identity_evidence();
    let invariants = work_graph_persistence_canary_readback_receipt_invariants();

    WorkGraphPersistenceCanaryReadbackReceiptPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_CANARY_READBACK_RECEIPT_PREVIEW_GATE,
        schema_version: WORK_GRAPH_PERSISTENCE_CANARY_READBACK_RECEIPT_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_canary_readback_receipt_preview_no_receipt_write",
        receipt_contract_count: receipt_contracts.len(),
        digest_check_count: digest_checks.len(),
        denial_reason_count: denial_reasons.len(),
        operator_view_count: operator_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_persistence_canary_readback_receipt_required_prior_gates(),
        receipt_contracts,
        digest_checks,
        denial_reasons,
        operator_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate: WORK_GRAPH_PERSISTENCE_CANARY_READBACK_RECEIPT_RECOMMENDED_NEXT_GATE,
        ready_for_promotion_blocker_preview: true,
        ready_for_receipt_persistence: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphPersistenceCanaryReadbackReceiptPreviewSideEffects::none(),
    }
}

pub fn work_graph_persistence_canary_readback_receipt_required_prior_gates() -> Vec<&'static str> {
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
        "hepta_work_graph_durable_identity_preview_gate",
    ]
}

pub fn work_graph_persistence_canary_readback_receipt_contract_ids() -> Vec<&'static str> {
    vec![
        "store_persistence_canary_receipt",
        "wal_append_canary_receipt",
        "checkpoint_write_canary_receipt",
        "readback_receipt_canary_receipt",
        "idempotency_index_canary_receipt",
        "replay_execution_canary_receipt",
    ]
}

pub fn work_graph_persistence_canary_readback_durable_identity_field_ids() -> Vec<&'static str> {
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

pub fn work_graph_persistence_canary_receipt_contracts()
-> Vec<WorkGraphPersistenceCanaryReceiptContractPreview> {
    vec![
        receipt_contract(
            "store_persistence_canary_receipt",
            "canary_store_persistence_dry_run",
        ),
        receipt_contract("wal_append_canary_receipt", "canary_wal_append_dry_run"),
        receipt_contract(
            "checkpoint_write_canary_receipt",
            "canary_checkpoint_write_dry_run",
        ),
        receipt_contract(
            "readback_receipt_canary_receipt",
            "canary_readback_receipt_dry_run",
        ),
        receipt_contract(
            "idempotency_index_canary_receipt",
            "canary_idempotency_index_dry_run",
        ),
        receipt_contract(
            "replay_execution_canary_receipt",
            "canary_replay_execution_dry_run",
        ),
    ]
}

pub fn work_graph_persistence_canary_receipt_digest_checks()
-> Vec<WorkGraphPersistenceCanaryReceiptDigestCheckPreview> {
    vec![
        digest_check(
            "check_prior_gate_digest_hash",
            vec!["priorGateIds", "priorGateReportHash", "generatedAtUnixMs"],
        ),
        digest_check(
            "check_feature_flag_digest_hash",
            vec!["featureFlagId", "defaultState", "operatorIdHash"],
        ),
        digest_check(
            "check_canary_evidence_hash",
            vec!["scenarioId", "expectedEvidenceIds", "evidenceHash"],
        ),
        digest_check(
            "check_zero_write_and_traffic_hash",
            vec!["trafficPpm", "writeMode", "sideEffectHash"],
        ),
        digest_check(
            "check_rollback_guard_hash",
            vec!["rollbackGuardIds", "receiptHash", "redactionState"],
        ),
        digest_check(
            "check_durable_identity_digest_hash",
            work_graph_persistence_canary_readback_durable_identity_field_ids(),
        ),
    ]
}

pub fn work_graph_persistence_canary_receipt_denials()
-> Vec<WorkGraphPersistenceCanaryReceiptDenialPreview> {
    let receipt_ids = work_graph_persistence_canary_receipt_contracts()
        .iter()
        .map(|receipt| receipt.id)
        .collect::<Vec<_>>();

    vec![
        denial(
            "missing_prior_gate_digest",
            "receipt does not include the full prior gate digest",
            receipt_ids.clone(),
        ),
        denial(
            "feature_flag_not_default_off",
            "receipt proposes a mutable or enabled feature flag",
            receipt_ids.clone(),
        ),
        denial(
            "canary_evidence_hash_missing",
            "receipt omits scenario evidence hash",
            receipt_ids.clone(),
        ),
        denial(
            "zero_write_or_traffic_not_proven",
            "receipt cannot prove zero traffic and zero writes",
            receipt_ids.clone(),
        ),
        denial(
            "rollback_guard_receipt_missing",
            "receipt omits rollback guard coverage",
            receipt_ids.clone(),
        ),
        denial(
            "receipt_redaction_missing",
            "receipt is not redacted/hash-only",
            receipt_ids.clone(),
        ),
        denial(
            "durable_identity_evidence_missing",
            "receipt omits workflow, run, step, checkpoint, replay, rollback, or receipt identity",
            receipt_ids,
        ),
    ]
}

pub fn work_graph_persistence_canary_receipt_operator_views()
-> Vec<WorkGraphPersistenceCanaryReceiptOperatorViewPreview> {
    vec![
        operator_view(
            "operator_canary_receipt_summary",
            "operator",
            vec![
                "workflow_id",
                "run_id",
                "step_id",
                "receipt_hash",
                "scenarioId",
                "featureFlagId",
                "zeroTraffic",
                "zeroWrites",
                "rollbackGuardIds",
            ],
        ),
        operator_view(
            "auditor_canary_digest_view",
            "auditor",
            vec![
                "workflow_id",
                "run_id",
                "checkpoint",
                "receipt_hash",
                "receiptId",
                "priorGateReportHash",
                "evidenceHash",
                "redactionState",
            ],
        ),
        operator_view(
            "rollback_receipt_preview_view",
            "operator",
            vec![
                "workflow_id",
                "rollback_anchor",
                "replay_key",
                "receipt_hash",
                "triggerGuardId",
                "receiptHash",
                "rollbackOwnerHash",
                "expiresAtUnixMs",
            ],
        ),
        operator_view(
            "promotion_blocker_packet_view",
            "system",
            vec![
                "workflow_id",
                "run_id",
                "step_id",
                "receipt_hash",
                "denialReasonIds",
                "digestCheckIds",
                "receiptContractIds",
                "nextGate",
            ],
        ),
    ]
}

pub fn work_graph_persistence_canary_readback_durable_identity_evidence()
-> WorkGraphPersistenceCanaryReadbackDurableIdentityEvidencePreview {
    WorkGraphPersistenceCanaryReadbackDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: work_graph_persistence_canary_readback_durable_identity_field_ids(),
        required_for_receipt_contract_ids:
            work_graph_persistence_canary_readback_receipt_contract_ids(),
        currently_satisfied: false,
    }
}

pub fn work_graph_persistence_canary_readback_receipt_invariants()
-> Vec<WorkGraphPersistenceCanaryReadbackReceiptInvariantPreview> {
    vec![
        invariant(
            "canary_readback_receipts_require_durable_identity_evidence",
            "receipt contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "canary_receipts_are_hash_only",
            "receipt contracts expose hashes, refs, and redaction state instead of payloads",
        ),
        invariant(
            "canary_receipts_require_prior_gate_digest",
            "receipt previews cannot be accepted without the complete prior gate digest",
        ),
        invariant(
            "canary_receipts_prove_zero_write_and_traffic",
            "receipt previews must carry evidence for zero live traffic and zero persisted writes",
        ),
        invariant(
            "canary_receipt_denials_block_promotion",
            "any missing digest, redaction, rollback, or zero-write proof blocks promotion",
        ),
        invariant(
            "operator_views_are_not_external_delivery",
            "operator and auditor views are local preview shapes and cannot be sent externally",
        ),
        invariant(
            "persistence_canary_readback_receipt_preview_has_no_side_effects",
            "this gate cannot execute readback, persist receipts, promote state, or send externally",
        ),
    ]
}

impl WorkGraphPersistenceCanaryReadbackReceiptPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            receipt_persisted: false,
            readback_performed: false,
            promotion_performed: false,
            feature_flag_mutated: false,
            canary_executed: false,
            live_traffic_routed: false,
            wal_written: false,
            checkpoint_written: false,
            approval_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn receipt_contract(
    id: &'static str,
    source_dry_run_scenario_id: &'static str,
) -> WorkGraphPersistenceCanaryReceiptContractPreview {
    WorkGraphPersistenceCanaryReceiptContractPreview {
        id,
        source_dry_run_scenario_id,
        required_fields: with_canary_readback_durable_identity_fields(vec![
            "receiptId",
            "scenarioId",
            "featureFlagId",
            "priorGateReportHash",
            "evidenceHash",
            "zeroTrafficProof",
            "zeroWriteProof",
            "rollbackGuardIds",
            "redactionState",
        ]),
        redaction_state: "redacted_hash_only",
        persistence_enabled: false,
        external_delivery_enabled: false,
    }
}

fn with_canary_readback_durable_identity_fields(fields: Vec<&'static str>) -> Vec<&'static str> {
    let mut merged = work_graph_persistence_canary_readback_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn digest_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphPersistenceCanaryReceiptDigestCheckPreview {
    WorkGraphPersistenceCanaryReceiptDigestCheckPreview {
        id,
        compared_fields,
        blocks_promotion: true,
    }
}

fn denial(
    id: &'static str,
    trigger: &'static str,
    applies_to_receipt_ids: Vec<&'static str>,
) -> WorkGraphPersistenceCanaryReceiptDenialPreview {
    WorkGraphPersistenceCanaryReceiptDenialPreview {
        id,
        trigger,
        applies_to_receipt_ids,
        blocks_promotion: true,
    }
}

fn operator_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphPersistenceCanaryReceiptOperatorViewPreview {
    WorkGraphPersistenceCanaryReceiptOperatorViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphPersistenceCanaryReadbackReceiptInvariantPreview {
    WorkGraphPersistenceCanaryReadbackReceiptInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canary_readback_receipt_preview_declares_receipts_for_each_dry_run() {
        let report = hepta_work_graph_persistence_canary_readback_receipt_preview_report();
        let scenario_ids = report
            .receipt_contracts
            .iter()
            .map(|receipt| receipt.source_dry_run_scenario_id)
            .collect::<Vec<_>>();

        assert_eq!(
            scenario_ids,
            [
                "canary_store_persistence_dry_run",
                "canary_wal_append_dry_run",
                "canary_checkpoint_write_dry_run",
                "canary_readback_receipt_dry_run",
                "canary_idempotency_index_dry_run",
                "canary_replay_execution_dry_run",
            ]
        );
        assert_eq!(report.receipt_contract_count, 6);
        assert!(report.receipt_contracts.iter().all(|receipt| {
            receipt.redaction_state == "redacted_hash_only"
                && !receipt.persistence_enabled
                && !receipt.external_delivery_enabled
                && work_graph_persistence_canary_readback_durable_identity_field_ids()
                    .iter()
                    .all(|field| receipt.required_fields.contains(field))
        }));
    }

    #[test]
    fn canary_readback_receipt_preview_blocks_promotion_on_digest_failure() {
        let report = hepta_work_graph_persistence_canary_readback_receipt_preview_report();
        let check_ids = report
            .digest_checks
            .iter()
            .map(|check| check.id)
            .collect::<Vec<_>>();

        assert_eq!(
            check_ids,
            [
                "check_prior_gate_digest_hash",
                "check_feature_flag_digest_hash",
                "check_canary_evidence_hash",
                "check_zero_write_and_traffic_hash",
                "check_rollback_guard_hash",
                "check_durable_identity_digest_hash",
            ]
        );
        assert_eq!(report.digest_check_count, 6);
        assert!(
            report
                .digest_checks
                .iter()
                .all(|check| check.blocks_promotion)
        );
    }

    #[test]
    fn canary_readback_receipt_preview_declares_denials_and_views() {
        let report = hepta_work_graph_persistence_canary_readback_receipt_preview_report();
        let denial_ids = report
            .denial_reasons
            .iter()
            .map(|denial| denial.id)
            .collect::<Vec<_>>();
        let view_ids = report
            .operator_views
            .iter()
            .map(|view| view.id)
            .collect::<Vec<_>>();

        assert_eq!(
            denial_ids,
            [
                "missing_prior_gate_digest",
                "feature_flag_not_default_off",
                "canary_evidence_hash_missing",
                "zero_write_or_traffic_not_proven",
                "rollback_guard_receipt_missing",
                "receipt_redaction_missing",
                "durable_identity_evidence_missing",
            ]
        );
        assert_eq!(report.denial_reason_count, 7);
        assert!(
            report.denial_reasons.iter().all(|denial| {
                denial.blocks_promotion && denial.applies_to_receipt_ids.len() == 6
            })
        );
        assert_eq!(
            view_ids,
            [
                "operator_canary_receipt_summary",
                "auditor_canary_digest_view",
                "rollback_receipt_preview_view",
                "promotion_blocker_packet_view",
            ]
        );
        assert_eq!(report.operator_view_count, 4);
        assert!(report.operator_views.iter().all(|view| {
            !view.external_delivery_enabled
                && view.required_fields.contains(&"workflow_id")
                && view.required_fields.contains(&"receipt_hash")
        }));
    }

    #[test]
    fn canary_readback_receipt_preview_keeps_side_effects_disabled() {
        let report = hepta_work_graph_persistence_canary_readback_receipt_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphPersistenceCanaryReadbackReceiptPreviewSideEffects::none()
        );
        assert_eq!(
            report.durable_identity_evidence.required_field_ids,
            work_graph_persistence_canary_readback_durable_identity_field_ids()
        );
        assert_eq!(
            report
                .durable_identity_evidence
                .required_for_receipt_contract_ids,
            work_graph_persistence_canary_readback_receipt_contract_ids()
        );
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert!(report.ready_for_promotion_blocker_preview);
        assert!(!report.ready_for_receipt_persistence);
        assert!(!report.ready_for_live_persistence);
    }

    #[test]
    fn canary_readback_receipt_preview_requires_canary_dry_run_gate() {
        let report = hepta_work_graph_persistence_canary_readback_receipt_preview_report();

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
                "hepta_work_graph_durable_identity_preview_gate",
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_PERSISTENCE_CANARY_READBACK_RECEIPT_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(report.invariant_count, 7);
    }
}
