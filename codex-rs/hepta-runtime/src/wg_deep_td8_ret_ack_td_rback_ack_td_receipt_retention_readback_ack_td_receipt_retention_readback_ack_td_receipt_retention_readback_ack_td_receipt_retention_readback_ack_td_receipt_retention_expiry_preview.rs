use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8LatestTerminalDecisionReceiptRetentionTd2ExpiryPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub retention_policy_count: usize,
    pub expiry_guard_count: usize,
    pub supersession_guard_count: usize,
    pub garbage_collection_denial_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub retention_policies: Vec<DeepTd8LatestTerminalDecisionReceiptRetentionTd2Item>,
    pub expiry_guards: Vec<DeepTd8LatestTerminalDecisionReceiptRetentionTd2Item>,
    pub supersession_guards: Vec<DeepTd8LatestTerminalDecisionReceiptRetentionTd2Item>,
    pub garbage_collection_denials: Vec<DeepTd8LatestTerminalDecisionReceiptRetentionTd2Item>,
    pub local_views: Vec<DeepTd8LatestTerminalDecisionReceiptRetentionTd2Item>,
    pub invariants: Vec<DeepTd8LatestTerminalDecisionReceiptRetentionTd2Item>,
    pub recommended_next_gate: String,
    pub ready_for_readback_receipt_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8LatestTerminalDecisionReceiptRetentionTd2SideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8LatestTerminalDecisionReceiptRetentionTd2Item {
    pub id: String,
    pub required_fields: Vec<&'static str>,
    pub mutation_allowed: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8LatestTerminalDecisionReceiptRetentionTd2SideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub retention_state_persisted: bool,
    pub garbage_collection_mutated: bool,
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

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_base()
-> String {
    let gate = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_gate();
    gate.strip_suffix("_receipt_retention_expiry_preview_gate")
        .unwrap_or(gate.as_str())
        .to_owned()
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_receipt_gate()
-> String {
    format!(
        "{}_receipt_retention_expiry_readback_receipt_preview_gate",
        deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_base()
    )
}

pub fn hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_preview_report()
-> DeepTd8LatestTerminalDecisionReceiptRetentionTd2ExpiryPreviewReport {
    let retention_policies = retention_items("policy", 6);
    let expiry_guards = retention_items("expiry_guard", 6);
    let supersession_guards = retention_items("supersession_guard", 5);
    let garbage_collection_denials = retention_items("gc_denial", 6);
    let local_views = retention_items("local_view", 4);
    let invariants = retention_items("invariant", 6);
    let gate = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_gate();

    DeepTd8LatestTerminalDecisionReceiptRetentionTd2ExpiryPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode:
            "read_only_deep_td8_latest_terminal_decision_receipt_retention_expiry_preview_no_retention_mutation",
        retention_policy_count: retention_policies.len(),
        expiry_guard_count: expiry_guards.len(),
        supersession_guard_count: supersession_guards.len(),
        garbage_collection_denial_count: garbage_collection_denials.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_required_prior_gates(),
        retention_policies,
        expiry_guards,
        supersession_guards,
        garbage_collection_denials,
        local_views,
        invariants,
        recommended_next_gate:
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_receipt_gate(),
        ready_for_readback_receipt_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8LatestTerminalDecisionReceiptRetentionTd2SideEffects::none(),
    }
}

pub fn deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_required_prior_gates()
-> Vec<String> {
    let mut gates = crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_required_prior_gates();
    gates.push(crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_gate());
    gates
}

fn retention_items(
    prefix: &str,
    count: usize,
) -> Vec<DeepTd8LatestTerminalDecisionReceiptRetentionTd2Item> {
    let base = "deep_td8_latest_terminal_decision_receipt_retention";
    (0..count)
        .map(
            |index| DeepTd8LatestTerminalDecisionReceiptRetentionTd2Item {
                id: format!("{base}_{prefix}_{index}"),
                required_fields: vec!["priorGate", "receiptHash", "zeroEffectHash"],
                mutation_allowed: false,
                required: true,
            },
        )
        .collect()
}

impl DeepTd8LatestTerminalDecisionReceiptRetentionTd2SideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            retention_state_persisted: false,
            garbage_collection_mutated: false,
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
            && !self.retention_state_persisted
            && !self.garbage_collection_mutated
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
    fn work_graph_latest_td_receipt_retention_requires_ack_replay_gate() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(&crate::deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_gate())
        );
    }

    #[test]
    fn work_graph_latest_td_receipt_retention_declares_hash_only_policies() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_preview_report();

        assert_eq!(report.retention_policy_count, 6);
        assert!(
            report
                .retention_policies
                .iter()
                .all(|policy| !policy.mutation_allowed && policy.required)
        );
    }

    #[test]
    fn work_graph_latest_td_receipt_retention_blocks_expiry_and_supersession_mutation() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_preview_report();

        assert_eq!(report.expiry_guard_count, 6);
        assert_eq!(report.supersession_guard_count, 5);
        assert!(
            report
                .expiry_guards
                .iter()
                .chain(report.supersession_guards.iter())
                .all(|guard| !guard.mutation_allowed)
        );
    }

    #[test]
    fn work_graph_latest_td_receipt_retention_denies_gc_mutation() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_preview_report();

        assert_eq!(report.garbage_collection_denial_count, 6);
        assert!(
            report
                .garbage_collection_denials
                .iter()
                .all(|denial| !denial.mutation_allowed)
        );
    }

    #[test]
    fn work_graph_latest_td_receipt_retention_points_to_readback_receipt() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert_eq!(
            report.recommended_next_gate,
            deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_receipt_gate()
        );
        assert!(report.ready_for_readback_receipt_preview);
    }

    #[test]
    fn work_graph_latest_td_receipt_retention_has_no_side_effects() {
        let report =
            hepta_work_graph_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_expiry_preview_report();

        assert_eq!(report.invariant_count, 6);
        assert!(report.side_effects.all_false());
    }
}
