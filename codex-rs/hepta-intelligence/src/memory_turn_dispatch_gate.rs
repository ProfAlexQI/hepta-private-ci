use serde::{Deserialize, Serialize};

use crate::{MEMORY_LIVE_TURN_PREFLIGHT_V1_CONTRACT, memory_live_turn_preflight_sample_report};

pub const MEMORY_TURN_DISPATCH_GATE_V1_CONTRACT: &str =
    "hepta-intelligence-memory-turn-dispatch-gate-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryTurnDispatchPolicy {
    pub policy_id: &'static str,
    pub require_explicit_policy_approval: bool,
    pub require_preflight_ready: bool,
    pub require_operator_visible_preview: bool,
    pub require_exact_node_readback: bool,
    pub require_stale_conflict_blocking: bool,
    pub require_token_budget_evidence: bool,
    pub block_silent_injection: bool,
    pub context_attachment_mode: &'static str,
    pub max_estimated_tokens: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryTurnDispatchDecision {
    pub turn_id: String,
    pub dispatcher_stage: &'static str,
    pub dispatch_action: &'static str,
    pub preflight_contract: &'static str,
    pub policy_gate_id: &'static str,
    pub prompt_bundle_id: String,
    pub preview_hash: String,
    pub approved_by_policy_gate: bool,
    pub operator_preview_acknowledged: bool,
    pub readback_evidence_present: bool,
    pub attached_node_ids: Vec<String>,
    pub excluded_conflict_count: usize,
    pub estimated_tokens: usize,
    pub context_attachment_allowed: bool,
    pub context_injection_performed: bool,
    pub model_invoked: bool,
    pub reply_delivered: bool,
    pub decision_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryTurnDispatchChecks {
    pub preflight_ready: bool,
    pub explicit_policy_approval_present: bool,
    pub operator_preview_acknowledged: bool,
    pub exact_node_readback_preserved: bool,
    pub source_citations_preserved: bool,
    pub stale_conflicts_blocked_before_dispatch: bool,
    pub token_budget_evidence_preserved: bool,
    pub dispatch_attachment_allowed_after_approval: bool,
    pub no_dispatch_when_approval_missing: bool,
    pub no_context_injection_in_sample: bool,
    pub no_model_invocation: bool,
    pub no_external_delivery: bool,
    pub no_production_memory_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryTurnDispatchChecks {
    pub fn ready(&self) -> bool {
        self.preflight_ready
            && self.explicit_policy_approval_present
            && self.operator_preview_acknowledged
            && self.exact_node_readback_preserved
            && self.source_citations_preserved
            && self.stale_conflicts_blocked_before_dispatch
            && self.token_budget_evidence_preserved
            && self.dispatch_attachment_allowed_after_approval
            && self.no_dispatch_when_approval_missing
            && self.no_context_injection_in_sample
            && self.no_model_invocation
            && self.no_external_delivery
            && self.no_production_memory_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryTurnDispatchGateReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p11_turn_dispatch_gate_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub preflight_contract: &'static str,
    pub prompt_bundle_id: String,
    pub included_node_count: usize,
    pub stale_conflict_count: usize,
    pub estimated_tokens: usize,
    pub policy: MemoryTurnDispatchPolicy,
    pub approved_dispatch_decision: MemoryTurnDispatchDecision,
    pub missing_approval_decision: MemoryTurnDispatchDecision,
    pub checks: MemoryTurnDispatchChecks,
    pub next_phase: &'static str,
}

pub fn memory_turn_dispatch_gate_sample_report(sample_run: bool) -> MemoryTurnDispatchGateReport {
    let preflight = memory_live_turn_preflight_sample_report(true);
    let policy = MemoryTurnDispatchPolicy {
        policy_id: "memory-turn-dispatch-policy-v1",
        require_explicit_policy_approval: true,
        require_preflight_ready: true,
        require_operator_visible_preview: true,
        require_exact_node_readback: true,
        require_stale_conflict_blocking: true,
        require_token_budget_evidence: true,
        block_silent_injection: true,
        context_attachment_mode: "approved_dry_run_plan",
        max_estimated_tokens: preflight.policy.max_estimated_tokens,
    };
    let approved_dispatch_decision = dispatch_decision_from_preflight(&preflight, &policy, true);
    let missing_approval_decision = dispatch_decision_from_preflight(&preflight, &policy, false);

    let checks = MemoryTurnDispatchChecks {
        preflight_ready: policy.require_preflight_ready && preflight.p10_live_turn_preflight_ready,
        explicit_policy_approval_present: policy.require_explicit_policy_approval
            && approved_dispatch_decision.approved_by_policy_gate,
        operator_preview_acknowledged: policy.require_operator_visible_preview
            && approved_dispatch_decision.operator_preview_acknowledged,
        exact_node_readback_preserved: policy.require_exact_node_readback
            && preflight.checks.exact_node_readback
            && approved_dispatch_decision.attached_node_ids == preflight.preview.included_node_ids,
        source_citations_preserved: preflight.checks.source_citations_complete,
        stale_conflicts_blocked_before_dispatch: policy.require_stale_conflict_blocking
            && preflight
                .stale_fact_conflicts
                .iter()
                .all(|conflict| conflict.blocks_injection)
            && approved_dispatch_decision.excluded_conflict_count == preflight.stale_conflict_count,
        token_budget_evidence_preserved: policy.require_token_budget_evidence
            && approved_dispatch_decision.estimated_tokens <= policy.max_estimated_tokens
            && approved_dispatch_decision.estimated_tokens == preflight.estimated_tokens,
        dispatch_attachment_allowed_after_approval: approved_dispatch_decision
            .context_attachment_allowed
            && approved_dispatch_decision.dispatch_action == "stage_memory_context_for_dispatch",
        no_dispatch_when_approval_missing: !missing_approval_decision.context_attachment_allowed
            && missing_approval_decision.dispatch_action == "reject_memory_context_attachment",
        no_context_injection_in_sample: !approved_dispatch_decision.context_injection_performed
            && policy.block_silent_injection
            && !preflight.preview.injection_allowed,
        no_model_invocation: !approved_dispatch_decision.model_invoked
            && !missing_approval_decision.model_invoked,
        no_external_delivery: !approved_dispatch_decision.reply_delivered
            && !missing_approval_decision.reply_delivered,
        no_production_memory_mutation: preflight.checks.no_production_memory_mutation,
        no_raw_private_memory_logged: preflight.checks.no_raw_private_memory_logged,
    };
    let p11_turn_dispatch_gate_ready = checks.ready();

    MemoryTurnDispatchGateReport {
        product: "Hepta",
        command: "memory-turn-dispatch-gate",
        contract: MEMORY_TURN_DISPATCH_GATE_V1_CONTRACT,
        status: if p11_turn_dispatch_gate_ready {
            "ready"
        } else {
            "attention"
        },
        p11_turn_dispatch_gate_ready,
        native_rewrite: true,
        sample_run,
        preflight_contract: MEMORY_LIVE_TURN_PREFLIGHT_V1_CONTRACT,
        prompt_bundle_id: preflight.prompt_bundle_id.clone(),
        included_node_count: preflight.included_node_count,
        stale_conflict_count: preflight.stale_conflict_count,
        estimated_tokens: preflight.estimated_tokens,
        policy,
        approved_dispatch_decision,
        missing_approval_decision,
        checks,
        next_phase: "connect the approved memory dispatch plan to the live provider turn only behind feature flags, telemetry readback, and installed runtime rollback gates",
    }
}

fn dispatch_decision_from_preflight(
    preflight: &crate::MemoryLiveTurnPreflightReport,
    policy: &MemoryTurnDispatchPolicy,
    approved_by_policy_gate: bool,
) -> MemoryTurnDispatchDecision {
    let readback_evidence_present = preflight.p10_live_turn_preflight_ready
        && preflight.checks.exact_node_readback
        && preflight.checks.source_citations_complete
        && !preflight.preview.preview_hash.trim().is_empty();
    let operator_preview_acknowledged =
        approved_by_policy_gate && preflight.preview.visible_to_operator;
    let context_attachment_allowed = approved_by_policy_gate
        && readback_evidence_present
        && operator_preview_acknowledged
        && preflight.estimated_tokens <= policy.max_estimated_tokens
        && preflight
            .stale_fact_conflicts
            .iter()
            .all(|conflict| conflict.blocks_injection);
    let dispatch_action = if context_attachment_allowed {
        "stage_memory_context_for_dispatch"
    } else {
        "reject_memory_context_attachment"
    };
    let attached_node_ids = if context_attachment_allowed {
        preflight.preview.included_node_ids.clone()
    } else {
        Vec::new()
    };
    let decision_fingerprint = format!(
        "{}:{}:{}:{}:{}",
        preflight.preview.turn_id,
        preflight.preview.preview_hash,
        policy.policy_id,
        approved_by_policy_gate,
        attached_node_ids.join(",")
    );

    MemoryTurnDispatchDecision {
        turn_id: preflight.preview.turn_id.clone(),
        dispatcher_stage: "pre_provider_turn",
        dispatch_action,
        preflight_contract: preflight.contract,
        policy_gate_id: policy.policy_id,
        prompt_bundle_id: preflight.prompt_bundle_id.clone(),
        preview_hash: preflight.preview.preview_hash.clone(),
        approved_by_policy_gate,
        operator_preview_acknowledged,
        readback_evidence_present,
        attached_node_ids,
        excluded_conflict_count: preflight.stale_conflict_count,
        estimated_tokens: preflight.estimated_tokens,
        context_attachment_allowed,
        context_injection_performed: false,
        model_invoked: false,
        reply_delivered: false,
        decision_hash: stable_digest(&decision_fingerprint),
    }
}

fn stable_digest(value: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in value.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("sha256-sample-{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_turn_dispatch_gate_sample_gate_is_ready() {
        let report = memory_turn_dispatch_gate_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p11_turn_dispatch_gate_ready);
        assert!(report.checks.ready());
        assert_eq!(report.included_node_count, 4);
        assert_eq!(report.stale_conflict_count, 1);
        assert!(report.approved_dispatch_decision.context_attachment_allowed);
        assert!(
            !report
                .approved_dispatch_decision
                .context_injection_performed
        );
    }

    #[test]
    fn memory_turn_dispatch_blocks_missing_approval() {
        let report = memory_turn_dispatch_gate_sample_report(true);

        assert!(report.checks.no_dispatch_when_approval_missing);
        assert!(!report.missing_approval_decision.context_attachment_allowed);
        assert_eq!(
            report.missing_approval_decision.dispatch_action,
            "reject_memory_context_attachment"
        );
        assert!(
            report
                .missing_approval_decision
                .attached_node_ids
                .is_empty()
        );
    }

    #[test]
    fn memory_turn_dispatch_preserves_preflight_readback() {
        let report = memory_turn_dispatch_gate_sample_report(true);

        assert!(report.checks.exact_node_readback_preserved);
        assert!(report.checks.source_citations_preserved);
        assert!(report.checks.stale_conflicts_blocked_before_dispatch);
        assert_eq!(
            report.approved_dispatch_decision.attached_node_ids,
            vec!["MP1", "MP2", "MP3", "MP4"]
        );
        assert!(
            !report
                .approved_dispatch_decision
                .decision_hash
                .trim()
                .is_empty()
        );
    }
}
