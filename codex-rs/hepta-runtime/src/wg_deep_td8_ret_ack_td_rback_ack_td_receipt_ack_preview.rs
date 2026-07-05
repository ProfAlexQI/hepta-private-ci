use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptAckPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub acknowledgement_contract_count: usize,
    pub non_acceptance_reason_count: usize,
    pub recording_denial_count: usize,
    pub expiry_replay_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub acknowledgement_contracts: Vec<DeepTd8RetAckTdRbackAckTdReceiptAckItem>,
    pub non_acceptance_reasons: Vec<DeepTd8RetAckTdRbackAckTdReceiptAckItem>,
    pub recording_denials: Vec<DeepTd8RetAckTdRbackAckTdReceiptAckItem>,
    pub expiry_replay_guards: Vec<DeepTd8RetAckTdRbackAckTdReceiptAckItem>,
    pub local_views: Vec<DeepTd8RetAckTdRbackAckTdReceiptAckItem>,
    pub invariants: Vec<DeepTd8RetAckTdRbackAckTdReceiptAckItem>,
    pub recommended_next_gate: String,
    pub ready_for_terminal_decision_receipt_acknowledgement_replay_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8RetAckTdRbackAckTdReceiptAckSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptAckItem {
    pub id: String,
    pub source_receipt_ids: Vec<String>,
    pub required_fields: Vec<&'static str>,
    pub hash_only: bool,
    pub blocks_acknowledgement_recording: bool,
    pub blocks_receipt_recording: bool,
    pub blocks_acceptance: bool,
    pub blocks_authority: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8RetAckTdRbackAckTdReceiptAckSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub receipt_recorded: bool,
    pub acknowledgement_recorded: bool,
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

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_ack_replay_gate() -> String {
    format!(
        "{}_receipt_acknowledgement_replay_idempotency_preview_gate",
        crate::deep_td8_ret_ack_td_rback_ack_td_base()
    )
}

pub fn hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_preview_report()
-> DeepTd8RetAckTdRbackAckTdReceiptAckPreviewReport {
    let acknowledgement_contracts = ack_items("acknowledgement_contract", 6);
    let non_acceptance_reasons = ack_items("non_acceptance_reason", 7);
    let recording_denials = ack_items("recording_denial", 7);
    let expiry_replay_guards = ack_items("expiry_replay_guard", 5);
    let local_views = ack_items("local_view", 4);
    let invariants = ack_items("invariant", 6);
    let gate = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_acknowledgement_gate();

    DeepTd8RetAckTdRbackAckTdReceiptAckPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_ret_ack_td_rback_ack_terminal_decision_receipt_acknowledgement_preview_no_recording",
        acknowledgement_contract_count: acknowledgement_contracts.len(),
        non_acceptance_reason_count: non_acceptance_reasons.len(),
        recording_denial_count: recording_denials.len(),
        expiry_replay_guard_count: expiry_replay_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates: deep_td8_ret_ack_td_rback_ack_td_receipt_ack_required_prior_gates(),
        acknowledgement_contracts,
        non_acceptance_reasons,
        recording_denials,
        expiry_replay_guards,
        local_views,
        invariants,
        recommended_next_gate: deep_td8_ret_ack_td_rback_ack_td_receipt_ack_replay_gate(),
        ready_for_terminal_decision_receipt_acknowledgement_replay_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8RetAckTdRbackAckTdReceiptAckSideEffects::none(),
    }
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_ack_required_prior_gates() -> Vec<String> {
    let mut gates = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_required_prior_gates();
    gates.push(crate::deep_td8_ret_ack_td_rback_ack_td_receipt_gate());
    gates
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_ack_ids() -> Vec<String> {
    (0..6)
        .map(|index| format!("deep_td8_ret_ack_td_rback_ack_td_receipt_ack_{index}"))
        .collect()
}

fn ack_items(prefix: &str, count: usize) -> Vec<DeepTd8RetAckTdRbackAckTdReceiptAckItem> {
    let source_receipt_ids = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_ids();
    (0..count)
        .map(|index| DeepTd8RetAckTdRbackAckTdReceiptAckItem {
            id: format!("deep_td8_ret_ack_td_rback_ack_td_{prefix}_{index}"),
            source_receipt_ids: source_receipt_ids.clone(),
            required_fields: vec!["receiptHash", "acknowledgementHash", "zeroEffectHash"],
            hash_only: true,
            blocks_acknowledgement_recording: true,
            blocks_receipt_recording: true,
            blocks_acceptance: true,
            blocks_authority: true,
            blocks_release_publication: true,
            blocks_public_claim: true,
            blocks_external_delivery: true,
            required: true,
        })
        .collect()
}

impl DeepTd8RetAckTdRbackAckTdReceiptAckSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            receipt_recorded: false,
            acknowledgement_recorded: false,
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
            && !self.receipt_recorded
            && !self.acknowledgement_recorded
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
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_requires_receipt_gate() {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&crate::deep_td8_ret_ack_td_rback_ack_td_receipt_gate())
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_declares_non_accepting_contracts() {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_preview_report();

        assert_eq!(report.acknowledgement_contract_count, 6);
        assert!(report.acknowledgement_contracts.iter().all(|contract| {
            contract.hash_only && contract.blocks_acceptance && contract.blocks_authority
        }));
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_denies_recording_and_delivery() {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_preview_report();

        assert_eq!(report.recording_denial_count, 7);
        assert!(report.recording_denials.iter().all(|denial| {
            denial.blocks_acknowledgement_recording
                && denial.blocks_receipt_recording
                && denial.blocks_external_delivery
        }));
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_guards_expiry_scope_and_replay() {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_preview_report();

        assert_eq!(report.expiry_replay_guard_count, 5);
        assert!(
            report
                .expiry_replay_guards
                .iter()
                .all(|guard| guard.blocks_acknowledgement_recording)
        );
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_points_to_replay_gate() {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_preview_report();

        assert_eq!(
            report.recommended_next_gate,
            deep_td8_ret_ack_td_rback_ack_td_receipt_ack_replay_gate()
        );
        assert!(report.ready_for_terminal_decision_receipt_acknowledgement_replay_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }

    #[test]
    fn work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_has_no_side_effects() {
        let report = hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_ack_preview_report();

        assert!(report.side_effects.all_false());
    }
}
