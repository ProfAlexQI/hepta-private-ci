use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub replay_scenario_count: usize,
    pub idempotency_guard_count: usize,
    pub replay_denial_count: usize,
    pub monotonicity_check_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub replay_scenarios: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayItem>,
    pub idempotency_guards: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayItem>,
    pub replay_denials: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayItem>,
    pub monotonicity_checks: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayItem>,
    pub local_views: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayItem>,
    pub invariants: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayItem>,
    pub recommended_next_gate: String,
    pub ready_for_terminal_decision_non_promotion_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplaySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayItem
{
    pub id: &'static str,
    pub source_acknowledgement_ids: Vec<&'static str>,
    pub required_fields: Vec<&'static str>,
    pub blocks_replay_mutation: bool,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplaySideEffects
{
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub acknowledgement_replay_recorded: bool,
    pub acknowledgement_recorded: bool,
    pub receipt_recorded: bool,
    pub operator_acceptance_recorded: bool,
    pub approval_recorded: bool,
    pub authority_granted: bool,
    pub live_persistence_enabled: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub rollout_started: bool,
    pub release_published: bool,
    pub public_claim_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_terminal_decision_gate()
-> String {
    format!(
        "{}_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate",
        crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_base()
    )
}

pub fn hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_preview_report()
-> DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayPreviewReport{
    let replay_scenarios = replay_items("scenario", 6);
    let idempotency_guards = replay_items("idempotency_guard", 7);
    let replay_denials = replay_items("replay_denial", 7);
    let monotonicity_checks = replay_items("monotonicity_check", 5);
    let local_views = replay_items("local_view", 4);
    let invariants = replay_items("invariant", 6);
    let gate = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_gate();

    DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_acknowledgement_replay_idempotency_preview_no_replay_write",
        replay_scenario_count: replay_scenarios.len(),
        idempotency_guard_count: idempotency_guards.len(),
        replay_denial_count: replay_denials.len(),
        monotonicity_check_count: monotonicity_checks.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_required_prior_gates(),
        replay_scenarios,
        idempotency_guards,
        replay_denials,
        monotonicity_checks,
        local_views,
        invariants,
        recommended_next_gate:
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_terminal_decision_gate(),
        ready_for_terminal_decision_non_promotion_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplaySideEffects::none(),
    }
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_required_prior_gates()
-> Vec<String> {
    let mut gates =
        crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_required_prior_gates(
        );
    gates.push(crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_gate());
    gates
}

fn replay_items(
    prefix: &'static str,
    count: usize,
) -> Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayItem>
{
    let ack_ids = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_ids();
    (0..count)
        .map(
            |index| DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplayItem {
                id: match (prefix, index) {
                    ("scenario", 0) => "duplicate_deep_td8_ret_ack_td_readback_receipt_replay",
                    ("scenario", 1) => "duplicate_deep_td8_ret_ack_td_readback_ack_replay",
                    ("scenario", 2) => "stale_deep_td8_ret_ack_td_readback_digest_replay",
                    ("scenario", 3) => "superseded_deep_td8_ret_ack_td_readback_scope_replay",
                    ("scenario", 4) => "cross_scope_deep_td8_ret_ack_td_readback_ack_replay",
                    ("scenario", _) => "out_of_order_deep_td8_ret_ack_td_readback_ack_replay",
                    ("idempotency_guard", 0) => {
                        "deep_td8_ret_ack_td_replay_idempotency_key_required"
                    }
                    ("idempotency_guard", 1) => {
                        "deep_td8_ret_ack_td_replay_prior_gate_digest_required"
                    }
                    ("idempotency_guard", 2) => "deep_td8_ret_ack_td_replay_scope_epoch_required",
                    ("idempotency_guard", 3) => {
                        "deep_td8_ret_ack_td_replay_supersession_guard_required"
                    }
                    ("idempotency_guard", 4) => {
                        "deep_td8_ret_ack_td_replay_zero_effect_digest_required"
                    }
                    ("idempotency_guard", 5) => {
                        "deep_td8_ret_ack_td_replay_release_denial_binding_required"
                    }
                    ("idempotency_guard", _) => {
                        "deep_td8_ret_ack_td_replay_external_delivery_denial_required"
                    }
                    ("replay_denial", 0) => "deep_td8_ret_ack_td_duplicate_receipt_replay_denied",
                    ("replay_denial", 1) => "deep_td8_ret_ack_td_duplicate_ack_replay_denied",
                    ("replay_denial", 2) => "deep_td8_ret_ack_td_stale_digest_replay_denied",
                    ("replay_denial", 3) => "deep_td8_ret_ack_td_superseded_scope_replay_denied",
                    ("replay_denial", 4) => "deep_td8_ret_ack_td_cross_scope_replay_denied",
                    ("replay_denial", 5) => "deep_td8_ret_ack_td_out_of_order_replay_denied",
                    ("replay_denial", _) => "deep_td8_ret_ack_td_external_delivery_replay_denied",
                    ("monotonicity_check", 0) => {
                        "deep_td8_ret_ack_td_check_readback_receipt_sequence"
                    }
                    ("monotonicity_check", 1) => {
                        "deep_td8_ret_ack_td_check_acknowledgement_sequence"
                    }
                    ("monotonicity_check", 2) => "deep_td8_ret_ack_td_check_digest_epoch",
                    ("monotonicity_check", 3) => {
                        "deep_td8_ret_ack_td_check_release_public_claim_epoch"
                    }
                    ("monotonicity_check", _) => "deep_td8_ret_ack_td_check_next_gate_order",
                    ("local_view", 0) => "operator_deep_td8_ret_ack_td_ack_replay_view",
                    ("local_view", 1) => "auditor_deep_td8_ret_ack_td_ack_replay_digest_view",
                    ("local_view", 2) => "release_owner_deep_td8_ret_ack_td_ack_replay_denial_view",
                    ("local_view", _) => "runtime_deep_td8_ret_ack_td_ack_replay_zero_effect_view",
                    ("invariant", 0) => "deep_td8_ret_ack_td_ack_replay_is_idempotent",
                    ("invariant", 1) => "deep_td8_ret_ack_td_ack_replay_blocks_recording",
                    ("invariant", 2) => "deep_td8_ret_ack_td_ack_replay_blocks_authority",
                    ("invariant", 3) => "deep_td8_ret_ack_td_ack_replay_blocks_rollout",
                    ("invariant", 4) => "deep_td8_ret_ack_td_ack_replay_views_are_local_only",
                    ("invariant", _) => {
                        "deep_td8_ret_ack_td_ack_replay_preview_has_no_side_effects"
                    }
                    _ => unreachable!(),
                },
                source_acknowledgement_ids: ack_ids.clone(),
                required_fields: vec!["idempotencyKey", "priorGateDigest", "zeroEffectHash"],
                blocks_replay_mutation: true,
                blocks_acknowledgement_recording: true,
                blocks_acceptance: true,
                blocks_authority: true,
                blocks_rollout: true,
                blocks_release_publication: true,
                blocks_public_claim: true,
                blocks_external_delivery: true,
                required: true,
            },
        )
        .collect()
}

impl DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptRetentionReadbackAckReplaySideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            acknowledgement_replay_recorded: false,
            acknowledgement_recorded: false,
            receipt_recorded: false,
            operator_acceptance_recorded: false,
            approval_recorded: false,
            authority_granted: false,
            live_persistence_enabled: false,
            wal_written: false,
            checkpoint_written: false,
            rollout_started: false,
            release_published: false,
            public_claim_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }

    #[cfg(test)]
    fn all_false(self) -> bool {
        !self.filesystem_written
            && !self.graph_state_persisted
            && !self.acknowledgement_replay_recorded
            && !self.acknowledgement_recorded
            && !self.receipt_recorded
            && !self.operator_acceptance_recorded
            && !self.approval_recorded
            && !self.authority_granted
            && !self.live_persistence_enabled
            && !self.wal_written
            && !self.checkpoint_written
            && !self.rollout_started
            && !self.release_published
            && !self.public_claim_recorded
            && !self.external_send_performed
            && !self.model_invoked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_graph_deep_td8_ret_ack_td_ack_replay_requires_acknowledgement_gate() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_preview_report(
            );

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_gate())
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_ack_replay_declares_blocked_scenarios() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_preview_report(
            );

        assert_eq!(report.replay_scenario_count, 6);
        assert!(
            report
                .replay_scenarios
                .iter()
                .all(|scenario| scenario.source_acknowledgement_ids.len() == 6)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_ack_replay_requires_idempotency_guards() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_preview_report(
            );

        assert_eq!(report.idempotency_guard_count, 7);
        assert!(
            report
                .idempotency_guards
                .iter()
                .all(|guard| guard.blocks_replay_mutation)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_ack_replay_denies_every_mutating_outcome() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_preview_report(
            );

        assert_eq!(report.replay_denial_count, 7);
        assert!(report.replay_denials.iter().all(|denial| {
            denial.blocks_acknowledgement_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_release_publication
                && denial.blocks_public_claim
                && denial.blocks_external_delivery
        }));
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_ack_replay_enforces_monotonicity_and_next_gate() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_preview_report(
            );

        assert_eq!(report.monotonicity_check_count, 5);
        assert_eq!(
            report.recommended_next_gate,
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_terminal_decision_gate(
            )
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_ack_replay_has_no_side_effects() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_preview_report(
            );

        assert_eq!(report.local_view_count, 4);
        assert_eq!(report.invariant_count, 6);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(report.side_effects.all_false());
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }
}
