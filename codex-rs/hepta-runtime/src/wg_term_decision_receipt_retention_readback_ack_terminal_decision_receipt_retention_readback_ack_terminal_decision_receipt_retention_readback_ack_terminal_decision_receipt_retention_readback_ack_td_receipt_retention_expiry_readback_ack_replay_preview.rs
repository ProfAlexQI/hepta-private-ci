use serde::Serialize;

pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_PREVIEW_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_SCHEMA_VERSION:
    &str = "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_v1";
pub const WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayPreviewReport
{
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
    pub replay_scenarios:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayScenarioPreview>,
    pub idempotency_guards:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckIdempotencyGuardPreview>,
    pub replay_denials:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayDenialPreview>,
    pub monotonicity_checks:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckMonotonicityCheckPreview>,
    pub local_views:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayLocalViewPreview>,
    pub invariants:
        Vec<WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayInvariantPreview>,
    pub durable_identity_evidence:
        WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview,
    pub recommended_next_gate: &'static str,
    pub ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview:
        bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayScenarioPreview
{
    pub id: &'static str,
    pub source_acknowledgement_ids: Vec<&'static str>,
    pub replay_mode: &'static str,
    pub required_fields: Vec<&'static str>,
    pub acknowledgement_recording_allowed: bool,
    pub mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckIdempotencyGuardPreview
{
    pub id: &'static str,
    pub required_fields: Vec<&'static str>,
    pub blocks_replay_mutation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayDenialPreview
{
    pub id: &'static str,
    pub applies_to_replay_scenario_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_approval: bool,
    pub blocks_authority: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckMonotonicityCheckPreview
{
    pub id: &'static str,
    pub compared_fields: Vec<&'static str>,
    pub blocks_out_of_order_replay: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayLocalViewPreview
{
    pub id: &'static str,
    pub audience: &'static str,
    pub required_fields: Vec<&'static str>,
    pub external_delivery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayInvariantPreview
{
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview
{
    pub id: &'static str,
    pub schema_version: &'static str,
    pub required_prior_gate: &'static str,
    pub required_field_ids: Vec<&'static str>,
    pub required_for_replay_scenario_ids: Vec<&'static str>,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub currently_satisfied: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayPreviewSideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
    pub terminal_decision_receipt_persisted: bool,
    pub terminal_decision_receipt_acknowledgement_recorded: bool,
    pub retention_state_persisted: bool,
    pub readback_receipt_persisted: bool,
    pub receipt_acknowledgement_recorded: bool,
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

pub fn hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report()
-> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayPreviewReport
{
    let replay_scenarios =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_scenarios();
    let idempotency_guards =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_idempotency_guards();
    let replay_denials =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_denials();
    let monotonicity_checks =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_monotonicity_checks();
    let local_views =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_local_views();
    let invariants =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_invariants();
    let durable_identity_evidence =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_evidence();

    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_no_replay_write",
        replay_scenario_count: replay_scenarios.len(),
        idempotency_guard_count: idempotency_guards.len(),
        replay_denial_count: replay_denials.len(),
        monotonicity_check_count: monotonicity_checks.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_required_prior_gates(),
        replay_scenarios,
        idempotency_guards,
        replay_denials,
        monotonicity_checks,
        local_views,
        invariants,
        durable_identity_evidence,
        recommended_next_gate:
            WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_RECOMMENDED_NEXT_GATE,
        ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects:
            WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayPreviewSideEffects::none(),
    }
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_required_prior_gates()
-> Vec<&'static str> {
    let mut gates = crate::work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_required_prior_gates();
    gates.retain(|gate| *gate != "hepta_work_graph_durable_identity_preview_gate");
    gates.push("hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate");
    gates.push("hepta_work_graph_durable_identity_preview_gate");
    gates
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_field_ids()
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

fn with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(
    fields: Vec<&'static str>,
) -> Vec<&'static str> {
    let mut required =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_field_ids();
    required.extend(fields);
    required
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_scenario_ids()
-> Vec<&'static str> {
    vec![
        "duplicate_terminal_decision_receipt_retention_readback_receipt_replay",
        "duplicate_terminal_decision_receipt_retention_readback_acknowledgement_replay",
        "stale_terminal_decision_receipt_retention_readback_digest_replay",
        "superseded_terminal_decision_receipt_retention_readback_scope_replay",
        "cross_scope_terminal_decision_receipt_retention_readback_acknowledgement_replay",
        "out_of_order_terminal_decision_receipt_retention_readback_acknowledgement_replay",
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_source_acknowledgement_ids()
-> Vec<&'static str> {
    crate::work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_ids()
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_evidence()
-> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview{
    let required_field_ids =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_field_ids();
    let required_for_replay_scenario_ids =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_scenario_ids();

    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayDurableIdentityEvidencePreview {
        id: "terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_evidence",
        schema_version: "work_graph_durable_identity_preview_v1",
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        durable_field_count: required_field_ids.len(),
        preview_binding_count: 5,
        invariant_count: 7,
        currently_satisfied: false,
        required_field_ids,
        required_for_replay_scenario_ids,
    }
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_scenarios()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayScenarioPreview,
>{
    let acknowledgement_ids =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_source_acknowledgement_ids();
    vec![
        replay_scenario(
            "duplicate_terminal_decision_receipt_retention_readback_receipt_replay",
            acknowledgement_ids.clone(),
            "duplicate_readback_receipt",
        ),
        replay_scenario(
            "duplicate_terminal_decision_receipt_retention_readback_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "duplicate_acknowledgement",
        ),
        replay_scenario(
            "stale_terminal_decision_receipt_retention_readback_digest_replay",
            acknowledgement_ids.clone(),
            "stale_digest",
        ),
        replay_scenario(
            "superseded_terminal_decision_receipt_retention_readback_scope_replay",
            acknowledgement_ids.clone(),
            "superseded_scope",
        ),
        replay_scenario(
            "cross_scope_terminal_decision_receipt_retention_readback_acknowledgement_replay",
            acknowledgement_ids.clone(),
            "cross_scope_acknowledgement",
        ),
        replay_scenario(
            "out_of_order_terminal_decision_receipt_retention_readback_acknowledgement_replay",
            acknowledgement_ids,
            "out_of_order_acknowledgement",
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_idempotency_guards()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckIdempotencyGuardPreview,
>{
    vec![
        idempotency_guard(
            "terminal_decision_receipt_retention_readback_receipt_idempotency_key_required",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackReceiptId",
                "readbackReceiptHash",
                "retentionPolicyHash",
            ]),
        ),
        idempotency_guard(
            "terminal_decision_receipt_retention_readback_acknowledgement_idempotency_key_required",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "acknowledgementId",
                "acknowledgementHash",
                "localViewHash",
            ]),
        ),
        idempotency_guard(
            "terminal_decision_receipt_retention_readback_prior_gate_digest_binding_required",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "priorGateId",
                "priorGateDigest",
                "sourceReportHash",
            ]),
        ),
        idempotency_guard(
            "terminal_decision_receipt_retention_readback_scope_epoch_binding_required",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "receiptScope",
                "acknowledgementScope",
                "scopeEpoch",
            ]),
        ),
        idempotency_guard(
            "terminal_decision_receipt_retention_readback_supersession_guard_required",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "supersessionHash",
                "retentionWindow",
                "expiryState",
            ]),
        ),
        idempotency_guard(
            "terminal_decision_receipt_retention_readback_zero_effect_digest_required",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "zeroWriteHash",
                "zeroTrafficHash",
                "zeroExternalSendHash",
            ]),
        ),
        idempotency_guard(
            "terminal_decision_receipt_retention_readback_release_public_claim_denial_binding_required",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "releaseDenied",
                "publicClaimDenied",
                "externalDeliveryDenied",
            ]),
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_denials()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayDenialPreview,
>{
    let scenario_ids =
        work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_scenario_ids();
    vec![
        replay_denial(
            "durable_identity_evidence_missing",
            scenario_ids.clone(),
            "replay is denied without durable identity evidence",
        ),
        replay_denial(
            "terminal_decision_receipt_retention_readback_duplicate_receipt_denied",
            scenario_ids.clone(),
            "duplicate readback receipt cannot mutate or record",
        ),
        replay_denial(
            "terminal_decision_receipt_retention_readback_duplicate_ack_denied",
            scenario_ids.clone(),
            "duplicate acknowledgement cannot record acknowledgement",
        ),
        replay_denial(
            "terminal_decision_receipt_retention_readback_stale_digest_denied",
            scenario_ids.clone(),
            "stale digest cannot accept or grant authority",
        ),
        replay_denial(
            "terminal_decision_receipt_retention_readback_superseded_scope_denied",
            scenario_ids.clone(),
            "superseded scope cannot update retention state",
        ),
        replay_denial(
            "terminal_decision_receipt_retention_readback_cross_scope_denied",
            scenario_ids.clone(),
            "cross-scope acknowledgement cannot bind authority",
        ),
        replay_denial(
            "terminal_decision_receipt_retention_readback_out_of_order_denied",
            scenario_ids.clone(),
            "out-of-order replay cannot advance monotonicity",
        ),
        replay_denial(
            "terminal_decision_receipt_retention_readback_external_delivery_replay_denied",
            scenario_ids,
            "replay cannot send external delivery or record public claims",
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_monotonicity_checks()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckMonotonicityCheckPreview,
>{
    vec![
        monotonicity_check(
            "check_terminal_decision_receipt_retention_readback_receipt_sequence",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "readbackReceiptSequence",
                "priorReadbackReceiptSequence",
                "scopeEpoch",
            ]),
        ),
        monotonicity_check(
            "check_terminal_decision_receipt_retention_readback_acknowledgement_sequence",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "acknowledgementSequence",
                "priorAcknowledgementSequence",
                "scopeEpoch",
            ]),
        ),
        monotonicity_check(
            "check_terminal_decision_receipt_retention_readback_digest_epoch",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "digestEpoch",
                "priorDigestEpoch",
                "supersessionHash",
            ]),
        ),
        monotonicity_check(
            "check_terminal_decision_receipt_retention_readback_release_public_claim_epoch",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "releaseDenialEpoch",
                "publicClaimDenialEpoch",
                "externalDeliveryDenialEpoch",
            ]),
        ),
        monotonicity_check(
            "check_terminal_decision_receipt_retention_readback_next_gate_order",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "currentGate",
                "nextGate",
                "priorGate",
            ]),
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_local_views()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayLocalViewPreview,
>{
    vec![
        local_view(
            "operator_terminal_decision_receipt_retention_readback_ack_replay_view",
            "operator",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "replayScenarioId",
                "acknowledgementId",
                "recordingDenied",
                "nextGate",
            ]),
        ),
        local_view(
            "auditor_terminal_decision_receipt_retention_readback_ack_replay_digest_view",
            "auditor",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "idempotencyKeyHash",
                "priorGateDigest",
                "monotonicityCheckId",
                "zeroEffectHash",
            ]),
        ),
        local_view(
            "release_owner_terminal_decision_receipt_retention_readback_ack_replay_denial_view",
            "release_owner",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "releaseDenied",
                "publicationDenied",
                "publicClaimDenied",
                "externalDeliveryDenied",
            ]),
        ),
        local_view(
            "runtime_terminal_decision_receipt_retention_readback_ack_replay_zero_effect_view",
            "system",
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "replayRecorded",
                "authorityGranted",
                "trafficRouted",
                "externalSendPerformed",
            ]),
        ),
    ]
}

pub fn work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_invariants()
-> Vec<
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayInvariantPreview,
>{
    vec![
        invariant(
            "terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_requires_durable_identity_evidence",
            "replay/idempotency preview requires workflow, run, step, checkpoint, replay, rollback, and receipt hash evidence",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_replay_is_idempotent",
            "duplicate readback receipt and acknowledgement replays are idempotent no-ops",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_replay_blocks_recording",
            "replay cannot record acknowledgement, receipt, acceptance, or approval",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_replay_blocks_authority",
            "replay cannot grant authority or enable live persistence",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_replay_blocks_rollout",
            "replay cannot start rollout, traffic, release publication, or public claims",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_replay_views_are_local_only",
            "replay views remain local and hash-only",
        ),
        invariant(
            "terminal_decision_receipt_retention_readback_ack_replay_preview_has_no_side_effects",
            "this gate cannot persist, write WAL/checkpoints, publish, record public claims, or send externally",
        ),
    ]
}

impl
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayPreviewSideEffects
{
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
            terminal_decision_receipt_persisted: false,
            terminal_decision_receipt_acknowledgement_recorded: false,
            retention_state_persisted: false,
            readback_receipt_persisted: false,
            receipt_acknowledgement_recorded: false,
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

fn replay_scenario(
    id: &'static str,
    source_acknowledgement_ids: Vec<&'static str>,
    replay_mode: &'static str,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayScenarioPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayScenarioPreview {
        id,
        source_acknowledgement_ids,
        replay_mode,
        required_fields:
            with_work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_fields(vec![
                "replayScenarioId",
                "acknowledgementId",
                "replayMode",
                "nextGate",
            ]),
        acknowledgement_recording_allowed: false,
        mutation_allowed: false,
    }
}

fn idempotency_guard(
    id: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckIdempotencyGuardPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckIdempotencyGuardPreview {
        id,
        required_fields,
        blocks_replay_mutation: true,
    }
}

fn replay_denial(
    id: &'static str,
    applies_to_replay_scenario_ids: Vec<&'static str>,
    reason: &'static str,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayDenialPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayDenialPreview {
        id,
        applies_to_replay_scenario_ids,
        reason,
        blocks_acknowledgement_recording: true,
        blocks_acceptance: true,
        blocks_approval: true,
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
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckMonotonicityCheckPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckMonotonicityCheckPreview {
        id,
        compared_fields,
        blocks_out_of_order_replay: true,
    }
}

fn local_view(
    id: &'static str,
    audience: &'static str,
    required_fields: Vec<&'static str>,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayLocalViewPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayLocalViewPreview {
        id,
        audience,
        required_fields,
        external_delivery_enabled: false,
    }
}

fn invariant(
    id: &'static str,
    reason: &'static str,
) -> WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayInvariantPreview
{
    WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_replay_declares_blocked_scenarios()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.replay_scenario_count, 6);
        let durable_fields =
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_field_ids();
        assert!(report.replay_scenarios.iter().all(|scenario| {
            !scenario.acknowledgement_recording_allowed
                && !scenario.mutation_allowed
                && durable_fields
                    .iter()
                    .all(|field| scenario.required_fields.contains(field))
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_replay_requires_idempotency_guards()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.idempotency_guard_count, 7);
        let durable_fields =
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_field_ids();
        assert!(report.idempotency_guards.iter().all(|guard| {
            guard.blocks_replay_mutation
                && guard.required_fields.len() >= 10
                && durable_fields
                    .iter()
                    .all(|field| guard.required_fields.contains(field))
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_replay_denies_every_mutating_outcome()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.replay_denial_count, 8);
        assert!(
            report
                .replay_denials
                .iter()
                .any(|denial| denial.id == "durable_identity_evidence_missing")
        );
        assert!(
            report
                .replay_denials
                .iter()
                .all(|denial| denial.blocks_acknowledgement_recording
                    && denial.blocks_acceptance
                    && denial.blocks_approval
                    && denial.blocks_authority
                    && denial.blocks_public_claim
                    && denial.blocks_external_delivery)
        );
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_replay_enforces_monotonicity() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.monotonicity_check_count, 5);
        let durable_fields =
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_field_ids();
        assert!(report.monotonicity_checks.iter().all(|check| {
            check.blocks_out_of_order_replay
                && check.compared_fields.len() >= 10
                && durable_fields
                    .iter()
                    .all(|field| check.compared_fields.contains(field))
        }));
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_replay_requires_acknowledgement_gate()
     {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(
            report.required_prior_gates.iter().rev().nth(1),
            Some(
                &"hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate"
            )
        );
        assert_eq!(
            report.required_prior_gates.last(),
            Some(&"hepta_work_graph_durable_identity_preview_gate")
        );
        assert_eq!(
            report.durable_identity_evidence.required_prior_gate,
            "hepta_work_graph_durable_identity_preview_gate"
        );
        assert_eq!(report.durable_identity_evidence.durable_field_count, 7);
        assert!(!report.durable_identity_evidence.currently_satisfied);
        assert_eq!(report.recommended_next_gate, WORK_GRAPH_PERSISTENCE_ACCEPTANCE_EFFECT_APPLICATION_DENIAL_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_TERMINAL_DECISION_NON_PROMOTION_RECEIPT_RETENTION_EXPIRY_READBACK_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_RECOMMENDED_NEXT_GATE);
    }

    #[test]
    fn work_graph_terminal_decision_receipt_retention_readback_ack_replay_has_no_side_effects() {
        let report = hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert_eq!(report.invariant_count, 7);
        let durable_fields =
            work_graph_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_durable_identity_field_ids();
        assert!(report.local_views.iter().all(|view| {
            !view.external_delivery_enabled
                && durable_fields
                    .iter()
                    .all(|field| view.required_fields.contains(field))
        }));
        assert_eq!(
            report.side_effects,
            WorkGraphTermDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckTerminalDecisionReceiptRetentionReadbackAckReplayPreviewSideEffects::none()
        );
    }
}
