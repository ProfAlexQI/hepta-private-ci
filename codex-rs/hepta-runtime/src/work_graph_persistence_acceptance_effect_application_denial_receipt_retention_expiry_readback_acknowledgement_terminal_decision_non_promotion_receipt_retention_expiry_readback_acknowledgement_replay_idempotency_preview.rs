use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckReplayPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub replay_scenario_count: usize,
    pub idempotency_guard_count: usize,
    pub replay_denial_count: usize,
    pub monotonicity_check_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub replay_scenarios: Vec<WorkGraphTerminalReceiptRetentionReadbackAckReplayScenarioPreview>,
    pub idempotency_guards:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckIdempotencyGuardPreview>,
    pub replay_denials: Vec<WorkGraphTerminalReceiptRetentionReadbackAckReplayDenialPreview>,
    pub monotonicity_checks:
        Vec<WorkGraphTerminalReceiptRetentionReadbackAckMonotonicityCheckPreview>,
    pub local_views: Vec<WorkGraphTerminalReceiptRetentionReadbackAckReplayLocalViewPreview>,
    pub durable_identity_evidence:
        WorkGraphTerminalReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview,
    pub invariants: Vec<WorkGraphTerminalReceiptRetentionReadbackAckReplayInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: WorkGraphTerminalReceiptRetentionReadbackAckReplayPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckReplayScenarioPreview {
    pub id: &'static str,
    pub source_acknowledgement_ids: Vec<&'static str>,
    pub replay_mode: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acknowledgement_recording_allowed: bool,
    pub mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckIdempotencyGuardPreview {
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_replay_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckReplayDenialPreview {
    pub id: &'static str,
    pub applies_to_replay_scenario_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckMonotonicityCheckPreview {
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_out_of_order_replay: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckReplayLocalViewPreview {
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview {
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_replay_scenario_ids: Vec<&'static str>,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckReplayInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalReceiptRetentionReadbackAckReplayPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_receipt_retention_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub readback_acknowledgement_recorded: bool,
    pub replay_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report()
-> WorkGraphTerminalReceiptRetentionReadbackAckReplayPreviewReport {
    let replay_scenarios = work_graph_terminal_receipt_retention_readback_ack_replay_scenarios();
    let idempotency_guards =
        work_graph_terminal_receipt_retention_readback_ack_idempotency_guards();
    let replay_denials = work_graph_terminal_receipt_retention_readback_ack_replay_denials();
    let monotonicity_checks =
        work_graph_terminal_receipt_retention_readback_ack_monotonicity_checks();
    let local_views = work_graph_terminal_receipt_retention_readback_ack_replay_local_views();
    let durable_identity_evidence =
        work_graph_terminal_receipt_retention_readback_ack_replay_durable_identity_evidence();
    let invariants = work_graph_terminal_receipt_retention_readback_ack_replay_invariants();

    WorkGraphTerminalReceiptRetentionReadbackAckReplayPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_SCHEMA_VERSION,
        preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_no_replay_write",
        replay_scenario_count: replay_scenarios.len(),
        idempotency_guard_count: idempotency_guards.len(),
        replay_denial_count: replay_denials.len(),
        monotonicity_check_count: monotonicity_checks.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_terminal_receipt_retention_readback_ack_replay_required_prior_gates(),
        replay_scenarios,
        idempotency_guards,
        replay_denials,
        monotonicity_checks,
        local_views,
        durable_identity_evidence,
        invariants,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: WorkGraphTerminalReceiptRetentionReadbackAckReplayPreviewSideEffects::none(),
    }
}

pub fn work_graph_terminal_receipt_retention_readback_ack_replay_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        crate::work_graph_terminal_receipt_retention_readback_acknowledgement_required_prior_gates(
        );
    gates.retain(|gate| *gate != "hepta_work_graph_durable_identity_preview_gate");
    gates.push(
        "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate",
    );
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_terminal_receipt_retention_readback_ack_replay_scenario_ids() -> Vec<&'static str>
{
    vec![
        "duplicate_terminal_receipt_retention_readback_receipt_replay",
        "duplicate_terminal_receipt_retention_readback_acknowledgement_replay",
        "stale_terminal_receipt_retention_readback_digest_replay",
        "superseded_terminal_receipt_retention_scope_acknowledgement_replay",
        "cross_scope_terminal_receipt_retention_acknowledgement_replay",
        "out_of_order_terminal_receipt_retention_acknowledgement_replay",
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_replay_source_acknowledgement_ids()
-> Vec<&'static str> {
    vec![
        "terminal_receipt_retention_policy_readback_acknowledgement",
        "terminal_receipt_expiry_guard_readback_acknowledgement",
        "terminal_receipt_supersession_guard_readback_acknowledgement",
        "terminal_receipt_gc_denial_readback_acknowledgement",
        "terminal_receipt_zero_effect_digest_readback_acknowledgement",
        "terminal_receipt_release_public_claim_denial_readback_acknowledgement",
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_replay_durable_identity_field_ids()
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

pub fn work_graph_terminal_receipt_retention_readback_ack_replay_scenarios()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckReplayScenarioPreview> {
    let acknowledgement_ids =
        work_graph_terminal_receipt_retention_readback_ack_replay_source_acknowledgement_ids();

    vec![
        replay_scenario(
            "duplicate_terminal_receipt_retention_readback_receipt_replay",
            acknowledgement_ids.clone(),
            "duplicate_readback_receipt",
        ),
        replay_scenario(
            "duplicate_terminal_receipt_retention_readback_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "duplicate_acknowledgement",
        ),
        replay_scenario(
            "stale_terminal_receipt_retention_readback_digest_replay",
            acknowledgement_ids.clone(),
            "stale_readback_digest",
        ),
        replay_scenario(
            "superseded_terminal_receipt_retention_scope_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "superseded_retention_scope",
        ),
        replay_scenario(
            "cross_scope_terminal_receipt_retention_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "cross_scope_acknowledgement",
        ),
        replay_scenario(
            "out_of_order_terminal_receipt_retention_acknowledgement_replay",
            acknowledgement_ids,
            "out_of_order_acknowledgement",
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_idempotency_guards()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckIdempotencyGuardPreview> {
    vec![
        idempotency_guard(
            "terminal_retention_readback_receipt_idempotency_key_required",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackReceiptId",
                "retentionScope",
                "readbackReceiptHash",
            ]),
        ),
        idempotency_guard(
            "terminal_retention_readback_acknowledgement_idempotency_key_required",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "acknowledgementId",
                "acknowledgementHash",
                "readbackReceiptHash",
            ]),
        ),
        idempotency_guard(
            "terminal_retention_prior_gate_digest_binding_required",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "priorGateId",
                "priorGateDigest",
                "sourceReportHash",
            ]),
        ),
        idempotency_guard(
            "terminal_retention_scope_epoch_binding_required",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "retentionScope",
                "scopeEpoch",
                "supersessionHash",
            ]),
        ),
        idempotency_guard(
            "terminal_retention_zero_side_effect_digest_binding_required",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
        idempotency_guard(
            "terminal_retention_acknowledgement_sequence_required",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackSequence",
                "acknowledgementSequence",
                "sequenceHash",
            ]),
        ),
        idempotency_guard(
            "terminal_retention_replay_keeps_non_promotion_denied",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "acknowledgementRecorded",
                "acceptanceAllowed",
                "authorityGranted",
            ]),
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_replay_denials()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckReplayDenialPreview> {
    let replay_scenario_ids =
        work_graph_terminal_receipt_retention_readback_ack_replay_scenario_ids();

    vec![
        replay_denial(
            "durable_identity_evidence_missing",
            replay_scenario_ids.clone(),
            "terminal receipt retention readback acknowledgement replay cannot proceed without durable identity evidence",
        ),
        replay_denial(
            "duplicate_terminal_retention_readback_receipt_cannot_record_acknowledgement",
            replay_scenario_ids.clone(),
            "duplicate terminal retention readback receipt replay cannot record acknowledgement",
        ),
        replay_denial(
            "duplicate_terminal_retention_acknowledgement_cannot_record_acceptance",
            replay_scenario_ids.clone(),
            "duplicate terminal retention acknowledgement replay cannot record acceptance",
        ),
        replay_denial(
            "stale_terminal_retention_digest_cannot_grant_authority",
            replay_scenario_ids.clone(),
            "stale terminal retention digest replay cannot grant authority",
        ),
        replay_denial(
            "cross_scope_terminal_retention_replay_cannot_enable_live_persistence",
            replay_scenario_ids.clone(),
            "cross-scope terminal retention replay cannot enable live persistence, WAL, or checkpoints",
        ),
        replay_denial(
            "out_of_order_terminal_retention_replay_cannot_start_rollout",
            replay_scenario_ids.clone(),
            "out-of-order terminal retention replay cannot start rollout or route traffic",
        ),
        replay_denial(
            "superseded_terminal_retention_replay_cannot_publish_or_claim",
            replay_scenario_ids.clone(),
            "superseded terminal retention replay cannot publish release status or record public claims",
        ),
        replay_denial(
            "replayed_terminal_retention_acknowledgement_cannot_send_external_delivery",
            replay_scenario_ids,
            "replayed terminal retention acknowledgement cannot send external delivery",
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_monotonicity_checks()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckMonotonicityCheckPreview> {
    vec![
        monotonicity_check(
            "terminal_retention_readback_receipt_sequence_check",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackReceiptId",
                "readbackSequence",
                "readbackReceiptHash",
            ]),
        ),
        monotonicity_check(
            "terminal_retention_acknowledgement_sequence_check",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "acknowledgementId",
                "acknowledgementSequence",
                "acknowledgementHash",
            ]),
        ),
        monotonicity_check(
            "terminal_retention_prior_gate_digest_monotonicity_check",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "priorGateId",
                "priorGateDigest",
                "observedAt",
            ]),
        ),
        monotonicity_check(
            "terminal_retention_scope_epoch_monotonicity_check",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "retentionScope",
                "scopeEpoch",
                "supersessionHash",
            ]),
        ),
        monotonicity_check(
            "terminal_retention_zero_effect_digest_stability_check",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_replay_local_views()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckReplayLocalViewPreview> {
    vec![
        local_view(
            "operator_terminal_retention_readback_ack_replay_idempotency_view",
            "operator",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "replayScenarioId",
                "idempotencyKey",
                "acknowledgementRecordingAllowed",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_terminal_retention_readback_ack_replay_digest_view",
            "auditor",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackReceiptHash",
                "acknowledgementHash",
                "priorGateDigest",
                "monotonicityCheckId",
            ]),
        ),
        local_view(
            "release_owner_terminal_retention_readback_ack_replay_denial_view",
            "release_owner",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "publicClaimDenied",
                "externalDeliveryDenied",
            ]),
        ),
        local_view(
            "runtime_terminal_retention_readback_ack_replay_zero_effect_view",
            "system",
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "replayRecorded",
                "acknowledgementRecorded",
                "authorityGranted",
                "publicClaimRecorded",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_terminal_receipt_retention_readback_ack_replay_durable_identity_evidence()
-> WorkGraphTerminalReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview {
    WorkGraphTerminalReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview {
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids:
            work_graph_terminal_receipt_retention_readback_ack_replay_durable_identity_field_ids(),
        required_for_replay_scenario_ids:
            work_graph_terminal_receipt_retention_readback_ack_replay_scenario_ids(),
        durable_field_count: 7,
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
    }
}

pub fn work_graph_terminal_receipt_retention_readback_ack_replay_invariants()
-> Vec<WorkGraphTerminalReceiptRetentionReadbackAckReplayInvariantPreview> {
    vec![
        invariant(
            "terminal_receipt_retention_readback_ack_replay_requires_durable_identity_evidence",
            "terminal receipt retention readback acknowledgement replay requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence",
        ),
        invariant(
            "terminal_retention_readback_ack_replay_is_idempotent",
            "duplicate receipt, duplicate acknowledgement, and stale digest replay cannot change state",
        ),
        invariant(
            "terminal_retention_readback_ack_replay_keeps_zero_side_effects",
            "replay must preserve zero writes, zero traffic, zero release, zero public claims, and zero external sends",
        ),
        invariant(
            "terminal_retention_readback_ack_replay_requires_acknowledgement_gate",
            "replay idempotency requires the terminal receipt retention readback acknowledgement gate",
        ),
        invariant(
            "terminal_retention_readback_ack_replay_is_scope_bound",
            "cross-scope and superseded acknowledgement replay cannot unlock receipt recording",
        ),
        invariant(
            "terminal_retention_readback_ack_replay_views_are_local_only",
            "operator, auditor, release-owner, and runtime views cannot be sent externally",
        ),
        invariant(
            "terminal_retention_readback_ack_replay_preview_has_no_side_effects",
            "this gate cannot persist replay records, record acknowledgement, grant authority, publish, record public claims, or send externally",
        ),
    ]
}

fn replay_scenario(
    id: &'static str,
    source_acknowledgement_ids: Vec<&'static str>,
    replay_mode: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckReplayScenarioPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckReplayScenarioPreview {
        id,
        source_acknowledgement_ids,
        replay_mode,
        required_fields:
            with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "replayScenarioId",
                "sourceAcknowledgementIds",
                "replayMode",
                "zeroMutationProofHash",
            ]),
        acknowledgement_recording_allowed: false,
        mutation_allowed: false,
    }
}

fn with_terminal_receipt_retention_readback_ack_replay_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut merged =
        work_graph_terminal_receipt_retention_readback_ack_replay_durable_identity_field_ids();
    merged.extend(fields);
    merged
}

fn idempotency_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAckIdempotencyGuardPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckIdempotencyGuardPreview {
        id,
        required_fields,
        blocks_replay_mutation: true,
    }
}

fn replay_denial(
    id: &'static str,
    applies_to_replay_scenario_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckReplayDenialPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckReplayDenialPreview {
        id,
        applies_to_replay_scenario_ids,
        reason,
        blocks_acknowledgement_recording: true,
        blocks_acceptance: true,
        blocks_authority: true,
        blocks_rollout: true,
        blocks_release_publication: true,
        blocks_public_claim: true,
        blocks_external_delivery: true,
    }
}

fn monotonicity_check(
    id: &'static str,
    compared_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAckMonotonicityCheckPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckMonotonicityCheckPreview {
        id,
        compared_fields,
        blocks_out_of_order_replay: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTerminalReceiptRetentionReadbackAckReplayLocalViewPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckReplayLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTerminalReceiptRetentionReadbackAckReplayInvariantPreview {
    WorkGraphTerminalReceiptRetentionReadbackAckReplayInvariantPreview {
        id,
        required: true,
        reason,
    }
}

impl WorkGraphTerminalReceiptRetentionReadbackAckReplayPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            terminal_receipt_retention_state_persisted: false,
            readback_receipt_persisted: false,
            readback_acknowledgement_recorded: false,
            replay_recorded: false,
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
    fn terminal_retention_readback_ack_replay_declares_blocked_scenarios() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.replay_scenario_count, 6);
        assert_eq!(
            report
                .replay_scenarios
                .iter()
                .map(|scenario| scenario.id)
                .collect::<Vec<_>>(),
            work_graph_terminal_receipt_retention_readback_ack_replay_scenario_ids()
        );
        assert!(report.replay_scenarios.iter().all(|scenario| {
            !scenario.acknowledgement_recording_allowed
                && !scenario.mutation_allowed
                && scenario.source_acknowledgement_ids.len() == 6
                && scenario.required_fields.contains(&"workflow_id")
                && scenario.required_fields.contains(&"receipt_hash")
                && scenario.required_fields.len() >= 11
        }));
    }

    #[test]
    fn terminal_retention_readback_ack_replay_requires_idempotency_guards() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.idempotency_guard_count, 7);
        assert!(
            report
                .idempotency_guards
                .iter()
                .all(|guard| guard.blocks_replay_mutation
                    && guard.required_fields.contains(&"workflow_id")
                    && guard.required_fields.contains(&"receipt_hash")
                    && guard.required_fields.len() >= 10)
        );
    }

    #[test]
    fn terminal_retention_readback_ack_replay_denies_every_mutating_outcome() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.replay_denial_count, 8);
        assert!(
            report
                .replay_denials
                .iter()
                .any(|denial| denial.id == "durable_identity_evidence_missing")
        );
        assert!(report.replay_denials.iter().all(|denial| {
            denial.blocks_acknowledgement_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_rollout
                && denial.blocks_release_publication
                && denial.blocks_public_claim
                && denial.blocks_external_delivery
                && denial.applies_to_replay_scenario_ids.len() == 6
        }));
    }

    #[test]
    fn terminal_retention_readback_ack_replay_enforces_monotonicity() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.monotonicity_check_count, 5);
        assert!(
            report
                .monotonicity_checks
                .iter()
                .all(|check| check.blocks_out_of_order_replay
                    && check.compared_fields.contains(&"workflow_id")
                    && check.compared_fields.contains(&"receipt_hash")
                    && check.compared_fields.len() >= 10)
        );
    }

    #[test]
    fn terminal_retention_readback_ack_replay_requires_acknowledgement_gate() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(
            report
                .required_prior_gates
                .get(report.required_prior_gates.len() - 2),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.recommended_next_gate,
            "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate"
        );
        assert!(
            report
                .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview
        );
    }

    #[test]
    fn terminal_retention_readback_ack_replay_has_no_side_effects() {
        let report =
            hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

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
        let durable_evidence = report.durable_identity_evidence;
        assert_eq!(
            durable_evidence.required_prior_gate,
            "hepta_work_graph_durable_identity_preview_gate"
        );
        assert_eq!(durable_evidence.durable_field_count, 7);
        assert_eq!(durable_evidence.preview_binding_count, 5);
        assert_eq!(durable_evidence.invariant_count, 7);
        assert!(!durable_evidence.currently_satisfied);
        assert_eq!(report.invariant_count, 7);
        assert!(
            report
                .invariants
                .iter()
                .any(|invariant| invariant.id
                    == "terminal_receipt_retention_readback_ack_replay_requires_durable_identity_evidence")
        );
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalReceiptRetentionReadbackAckReplayPreviewSideEffects::none()
        );
    }
}
