use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8Td3RbackAckTd17TerminalDecisionPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: String,
    pub schema_version: String,
    pub preview_mode: &'static str,
    pub terminal_decision_surface_count: usize,
    pub non_promotion_denial_count: usize,
    pub authority_guard_count: usize,
    pub release_delivery_guard_count: usize,
    pub local_view_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<String>,
    pub terminal_decision_surfaces: Vec<DeepTd8Td3RbackAckTd17TerminalDecisionItem>,
    pub non_promotion_denials: Vec<DeepTd8Td3RbackAckTd17TerminalDecisionItem>,
    pub authority_guards: Vec<DeepTd8Td3RbackAckTd17TerminalDecisionItem>,
    pub release_delivery_guards: Vec<DeepTd8Td3RbackAckTd17TerminalDecisionItem>,
    pub local_views: Vec<DeepTd8Td3RbackAckTd17TerminalDecisionItem>,
    pub invariants: Vec<DeepTd8Td3RbackAckTd17TerminalDecisionItem>,
    pub recommended_next_gate: String,
    pub ready_for_terminal_decision_receipt_preview: bool,
    pub ready_for_operator_acceptance: bool,
    pub ready_for_live_persistence: bool,
    pub side_effects: DeepTd8Td3RbackAckTd17TerminalDecisionSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeepTd8Td3RbackAckTd17TerminalDecisionItem {
    pub id: String,
    pub source_ids: Vec<String>,
    pub required_fields: Vec<&'static str>,
    pub blocks_terminal_decision_recording: bool,
    pub blocks_persistence_promotion: bool,
    pub blocks_authority_grant: bool,
    pub blocks_rollout: bool,
    pub blocks_release_publication: bool,
    pub blocks_public_claim: bool,
    pub blocks_external_delivery: bool,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DeepTd8Td3RbackAckTd17TerminalDecisionSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub terminal_decision_recorded: bool,
    pub terminal_decision_receipt_recorded: bool,
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

pub fn deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_base()
-> String {
    let gate =
        crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_terminal_decision_gate();
    gate.strip_suffix("_preview_gate")
        .unwrap_or(gate.as_str())
        .to_owned()
}

pub fn deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_receipt_gate()
-> String {
    format!(
        "{}_receipt_preview_gate",
        deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_base()
    )
}

pub fn hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_preview_report()
-> DeepTd8Td3RbackAckTd17TerminalDecisionPreviewReport {
    let terminal_decision_surfaces = terminal_decision_items("surface", 6);
    let non_promotion_denials = terminal_decision_items("non_promotion_denial", 8);
    let authority_guards = terminal_decision_items("authority_guard", 6);
    let release_delivery_guards = terminal_decision_items("release_delivery_guard", 6);
    let local_views = terminal_decision_items("local_view", 4);
    let invariants = terminal_decision_items("invariant", 6);
    let gate =
        crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_terminal_decision_gate();

    DeepTd8Td3RbackAckTd17TerminalDecisionPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        schema_version: crate::deep_td8_schema_for(&gate),
        gate,
        preview_mode: "read_only_deep_td8_td3_rbackack_td17_receipt_retention_readback_ack_terminal_decision_non_promotion_preview_no_promotion",
        terminal_decision_surface_count: terminal_decision_surfaces.len(),
        non_promotion_denial_count: non_promotion_denials.len(),
        authority_guard_count: authority_guards.len(),
        release_delivery_guard_count: release_delivery_guards.len(),
        local_view_count: local_views.len(),
        invariant_count: invariants.len(),
        required_prior_gates:
            deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_required_prior_gates(),
        terminal_decision_surfaces,
        non_promotion_denials,
        authority_guards,
        release_delivery_guards,
        local_views,
        invariants,
        recommended_next_gate:
            deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_receipt_gate(),
        ready_for_terminal_decision_receipt_preview: true,
        ready_for_operator_acceptance: false,
        ready_for_live_persistence: false,
        side_effects: DeepTd8Td3RbackAckTd17TerminalDecisionSideEffects::none(),
    }
}

pub fn deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_required_prior_gates()
-> Vec<String> {
    let mut gates =
        crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_replay_required_prior_gates();
    gates.push(crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_readback_ack_replay_gate());
    gates
}

pub fn deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_surface_ids()
-> Vec<String> {
    (0..6)
        .map(|index| {
            format!("deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_surface_{index}")
        })
        .collect()
}

fn terminal_decision_items(
    prefix: &str,
    count: usize,
) -> Vec<DeepTd8Td3RbackAckTd17TerminalDecisionItem> {
    let source_ids =
        deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_surface_ids(
        );

    (0..count)
        .map(|index| DeepTd8Td3RbackAckTd17TerminalDecisionItem {
            id: format!(
                "deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_{prefix}_{index}"
            ),
            source_ids: source_ids.clone(),
            required_fields: vec!["priorGate", "terminalDecisionHash", "zeroEffectHash"],
            blocks_terminal_decision_recording: true,
            blocks_persistence_promotion: true,
            blocks_authority_grant: true,
            blocks_rollout: true,
            blocks_release_publication: true,
            blocks_public_claim: true,
            blocks_external_delivery: true,
            required: true,
        })
        .collect()
}

impl DeepTd8Td3RbackAckTd17TerminalDecisionSideEffects {
    pub fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            terminal_decision_recorded: false,
            terminal_decision_receipt_recorded: false,
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
            && !self.terminal_decision_receipt_recorded
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
    fn work_graph_td3_rbackack_td17_requires_latest_ack_replay_gate() {
        let report = hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_preview_report();

        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &crate::deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_readback_ack_replay_gate()
            )
        );
    }

    #[test]
    fn work_graph_td3_rbackack_td17_declares_local_terminal_surfaces() {
        let report = hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_preview_report();

        assert_eq!(report.terminal_decision_surface_count, 6);
        assert!(
            report
                .terminal_decision_surfaces
                .iter()
                .all(|surface| surface.blocks_terminal_decision_recording)
        );
    }

    #[test]
    fn work_graph_td3_rbackack_td17_blocks_promotion_paths() {
        let report = hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_preview_report();

        assert_eq!(report.non_promotion_denial_count, 8);
        assert!(report.non_promotion_denials.iter().all(|denial| {
            denial.blocks_persistence_promotion
                && denial.blocks_rollout
                && denial.blocks_release_publication
                && denial.blocks_public_claim
        }));
    }

    #[test]
    fn work_graph_td3_rbackack_td17_keeps_authority_and_delivery_denied() {
        let report = hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_preview_report();

        assert_eq!(report.authority_guard_count, 6);
        assert_eq!(report.release_delivery_guard_count, 6);
        assert!(
            report
                .authority_guards
                .iter()
                .chain(report.release_delivery_guards.iter())
                .all(|guard| guard.blocks_authority_grant && guard.blocks_external_delivery)
        );
    }

    #[test]
    fn work_graph_td3_rbackack_td17_points_to_receipt_gate() {
        let report = hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_preview_report();

        assert_eq!(
            report.recommended_next_gate,
            deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_receipt_gate()
        );
        assert!(report.ready_for_terminal_decision_receipt_preview);
        assert!(!report.ready_for_operator_acceptance);
        assert!(!report.ready_for_live_persistence);
    }

    #[test]
    fn work_graph_td3_rbackack_td17_has_no_side_effects() {
        let report = hepta_work_graph_deep_td8_retack_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_preview_report();

        assert_eq!(report.local_view_count, 4);
        assert_eq!(report.invariant_count, 6);
        assert!(report.invariants.iter().all(|invariant| invariant.required));
        assert!(report.side_effects.all_false());
    }
}
