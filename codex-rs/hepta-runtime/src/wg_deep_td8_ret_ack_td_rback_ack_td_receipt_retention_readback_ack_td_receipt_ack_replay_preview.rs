use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayPreviewReport {
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
    pub replay_scenarios:
        Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayItem>,
    pub idempotency_guards:
        Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayItem>,
    pub replay_denials:
        Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayItem>,
    pub monotonicity_checks:
        Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayItem>,
    pub local_views:
        Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayItem>,
    pub invariants: Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayItem>,
    pub recommended_next_gate: String,
    pub ready_for_receipt_retention_expiry_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects:
        DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplaySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayItem {
    pub id: String,
    pub source_acknowledgement_ids: Vec<String>,
    pub required_fields: Vec<&'static str>,
    pub blocks_replay_mutation: bool,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_receipt_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplaySideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
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

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_gate()
-> String {
    format!(
        "{}_receipt_retention_expiry_preview_gate",
        crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_base()
    )
}

pub fn hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_preview_report()
-> DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayPreviewReport {
    let replay_scenarios = replay_items("replay_scenario", 6);
    let idempotency_guards = replay_items("idempotency_guard", 7);
    let replay_denials = replay_items("replay_denial", 7);
    let monotonicity_checks = replay_items("monotonicity_check", 5);
    let local_views = replay_items("local_view", 4);
    let invariants = replay_items("invariant", 6);
    let gate = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_gate();

    DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_ret_ack_td_rback_ack_terminal_decision_receipt_acknowledgement_replay_idempotency_preview_no_replay_write",
        replay_scenario_count: replay_scenarios.len(),
        idempotency_guard_count: idempotency_guards.len(),
        replay_denial_count: replay_denials.len(),
        monotonicity_check_count: monotonicity_checks.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_required_prior_gates(),
        replay_scenarios,
        idempotency_guards,
        replay_denials,
        monotonicity_checks,
        local_views,
        invariants,
        recommended_next_gate: deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_gate(),
        ready_for_receipt_retention_expiry_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplaySideEffects::none(),
    }
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_required_prior_gates()
-> Vec<String> {
    let mut gates = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_required_prior_gates();
    gates.push(crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_acknowledgement_gate());
    gates
}

fn replay_items(
    prefix: &str,
    count: usize,
) -> Vec<DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayItem> {
    let source_acknowledgement_ids =
        crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_ids();
    (0..count)
        .map(|index| DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplayItem {
            id: format!("deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_{prefix}_{index}"),
            source_acknowledgement_ids: source_acknowledgement_ids.clone(),
            required_fields: vec!["idempotencyKey", "priorGateDigest", "zeroEffectHash"],
            blocks_replay_mutation: true,
            blocks_acknowledgement_recording: true,
            blocks_receipt_recording: true,
            blocks_acceptance: true,
            blocks_authority: true,
            blocks_rollout: true,
            blocks_release_publication: true,
            blocks_public_claim: true,
            blocks_external_delivery: true,
            required: true,
        })
        .collect()
}

impl DeepTd8RetAckTdRbackAckTdReceiptRetentionReadbackAckTdReceiptAckReplaySideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
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
            && !self.terminal_decision_recorded
            && !self.terminal_decision_receipt_recorded
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
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_requires_ack_gate()
     {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_acknowledgement_gate())
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_declares_blocked_scenarios()
     {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_preview_report();

        assert_eq!(report.replay_scenario_count, 6);
        assert!(
            report
                .replay_scenarios
                .iter()
                .all(|scenario| scenario.blocks_replay_mutation)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_requires_idempotency_guards()
     {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_preview_report();

        assert_eq!(report.idempotency_guard_count, 7);
        assert!(
            report
                .idempotency_guards
                .iter()
                .all(|guard| guard.blocks_replay_mutation)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_denies_every_mutating_outcome()
     {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_preview_report();

        assert_eq!(report.replay_denial_count, 7);
        assert!(report.replay_denials.iter().all(|denial| {
            denial.blocks_acknowledgement_recording
                && denial.blocks_receipt_recording
                && denial.blocks_acceptance
                && denial.blocks_authority
                && denial.blocks_external_delivery
        }));
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_points_to_retention()
     {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_preview_report();

        assert_eq!(
            report.recommended_next_gate,
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_gate()
        );
        assert!(report.ready_for_receipt_retention_expiry_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_has_no_side_effects()
     {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_preview_report();

        assert!(report.side_effects.all_false());
    }
}
