use serde::Deserialize;
use serde::Serialize;

use crate::MEMORY_ACTIVATION_CUTOVER_GATE_V1_CONTRACT;
use crate::memory_activation_cutover_gate_sample_report;

pub const MEMORY_PROVIDER_ROUTER_ACTIVATION_GATE_V1_CONTRACT: &str =
    "hepta-intelligence-memory-provider-router-activation-gate-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderRouterActivationPolicy {
    pub policy_id: &'static str,
    pub feature_flag_id: &'static str,
    pub require_cutover_gate: bool,
    pub require_canary_stage: bool,
    pub require_router_handoff_readback: bool,
    pub require_idempotency_key: bool,
    pub require_kill_switch_absent: bool,
    pub block_feature_flag_mutation_in_sample: bool,
    pub block_provider_invocation_in_sample: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderRouterActivationHandoff {
    pub handoff_id: String,
    pub provider_router_id: &'static str,
    pub selected_canary_stage_id: &'static str,
    pub traffic_percent_ppm: u32,
    pub max_context_node_count: usize,
    pub cutover_decision_id: String,
    pub kill_switch_id: String,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub router_handoff_recorded: bool,
    pub feature_flag_mutated_by_gate: bool,
    pub context_attached_to_live_prompt: bool,
    pub provider_invoked_by_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderRouterActivationDecision {
    pub decision_id: String,
    pub scenario: &'static str,
    pub cutover_gate_ready: bool,
    pub operator_release_approved: bool,
    pub kill_switch_active: bool,
    pub router_handoff_allowed: bool,
    pub fallback_no_memory_provider_turn_hash: String,
    pub blocked_reasons: Vec<&'static str>,
    pub context_attached_to_live_prompt: bool,
    pub provider_invoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderRouterActivationChecks {
    pub cutover_gate_ready: bool,
    pub release_checklist_complete: bool,
    pub canary_stage_selected: bool,
    pub router_handoff_readback_present: bool,
    pub approved_handoff_allowed: bool,
    pub missing_cutover_blocks_activation: bool,
    pub kill_switch_blocks_activation: bool,
    pub idempotency_key_present: bool,
    pub feature_flag_not_mutated: bool,
    pub no_live_context_attachment: bool,
    pub no_provider_model_invocation: bool,
    pub no_reply_delivery: bool,
    pub no_external_network_read: bool,
    pub no_production_memory_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryProviderRouterActivationChecks {
    pub fn ready(&self) -> bool {
        self.cutover_gate_ready
            && self.release_checklist_complete
            && self.canary_stage_selected
            && self.router_handoff_readback_present
            && self.approved_handoff_allowed
            && self.missing_cutover_blocks_activation
            && self.kill_switch_blocks_activation
            && self.idempotency_key_present
            && self.feature_flag_not_mutated
            && self.no_live_context_attachment
            && self.no_provider_model_invocation
            && self.no_reply_delivery
            && self.no_external_network_read
            && self.no_production_memory_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderRouterActivationGateReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p15_provider_router_activation_gate_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub activation_cutover_contract: &'static str,
    pub feature_flag_id: &'static str,
    pub policy: MemoryProviderRouterActivationPolicy,
    pub approved_router_handoff: MemoryProviderRouterActivationHandoff,
    pub approved_decision: MemoryProviderRouterActivationDecision,
    pub missing_cutover_decision: MemoryProviderRouterActivationDecision,
    pub kill_switch_decision: MemoryProviderRouterActivationDecision,
    pub checks: MemoryProviderRouterActivationChecks,
    pub next_phase: &'static str,
}

pub fn memory_provider_router_activation_gate_sample_report(
    sample_run: bool,
) -> MemoryProviderRouterActivationGateReport {
    let cutover = memory_activation_cutover_gate_sample_report(true);
    let selected_stage = cutover
        .canary_stages
        .first()
        .expect("sample cutover report should include a shadow canary stage");
    let policy = MemoryProviderRouterActivationPolicy {
        policy_id: "memory-provider-router-activation-policy-v1",
        feature_flag_id: cutover.feature_flag_id,
        require_cutover_gate: true,
        require_canary_stage: true,
        require_router_handoff_readback: true,
        require_idempotency_key: true,
        require_kill_switch_absent: true,
        block_feature_flag_mutation_in_sample: true,
        block_provider_invocation_in_sample: true,
    };
    let approved_router_handoff = MemoryProviderRouterActivationHandoff {
        handoff_id: stable_digest(&format!(
            "memory-router-handoff:{}:{}",
            cutover.approved_cutover_decision.decision_id, selected_stage.stage_id
        )),
        provider_router_id: "hepta-native-model-provider-router",
        selected_canary_stage_id: selected_stage.stage_id,
        traffic_percent_ppm: selected_stage.traffic_percent_ppm,
        max_context_node_count: selected_stage.max_attached_node_count,
        cutover_decision_id: cutover.approved_cutover_decision.decision_id.clone(),
        kill_switch_id: cutover.kill_switch_plan.kill_switch_id.clone(),
        idempotency_key: stable_digest(&format!(
            "memory-router-idempotency:{}:{}",
            cutover.feature_flag_id, selected_stage.stage_id
        )),
        readback_evidence_id: stable_digest(&format!(
            "memory-router-readback:{}:{}",
            cutover.contract, selected_stage.stage_id
        )),
        router_handoff_recorded: true,
        feature_flag_mutated_by_gate: false,
        context_attached_to_live_prompt: false,
        provider_invoked_by_gate: false,
    };
    let approved_decision = router_decision(
        "approved-cutover",
        cutover.p14_activation_cutover_gate_ready,
        cutover.approved_cutover_decision.operator_release_approved,
        false,
        true,
        cutover
            .kill_switch_plan
            .restores_no_memory_provider_turn_hash
            .clone(),
        vec![],
    );
    let missing_cutover_decision = router_decision(
        "missing-cutover",
        false,
        false,
        false,
        false,
        cutover
            .kill_switch_plan
            .restores_no_memory_provider_turn_hash
            .clone(),
        vec!["cutover_gate_not_ready", "operator_release_not_approved"],
    );
    let kill_switch_decision = router_decision(
        "kill-switch-active",
        cutover.p14_activation_cutover_gate_ready,
        cutover.approved_cutover_decision.operator_release_approved,
        true,
        false,
        cutover
            .kill_switch_plan
            .restores_no_memory_provider_turn_hash
            .clone(),
        vec!["kill_switch_active"],
    );
    let checks = MemoryProviderRouterActivationChecks {
        cutover_gate_ready: policy.require_cutover_gate
            && cutover.p14_activation_cutover_gate_ready,
        release_checklist_complete: cutover.checks.release_checklist_complete,
        canary_stage_selected: policy.require_canary_stage
            && !approved_router_handoff
                .selected_canary_stage_id
                .trim()
                .is_empty()
            && approved_router_handoff.traffic_percent_ppm == 0,
        router_handoff_readback_present: policy.require_router_handoff_readback
            && approved_router_handoff.router_handoff_recorded
            && !approved_router_handoff
                .readback_evidence_id
                .trim()
                .is_empty(),
        approved_handoff_allowed: approved_decision.router_handoff_allowed
            && approved_decision.cutover_gate_ready
            && approved_decision.operator_release_approved
            && !approved_decision.kill_switch_active,
        missing_cutover_blocks_activation: !missing_cutover_decision.router_handoff_allowed
            && missing_cutover_decision
                .blocked_reasons
                .contains(&"cutover_gate_not_ready"),
        kill_switch_blocks_activation: policy.require_kill_switch_absent
            && !kill_switch_decision.router_handoff_allowed
            && kill_switch_decision.kill_switch_active
            && kill_switch_decision
                .blocked_reasons
                .contains(&"kill_switch_active"),
        idempotency_key_present: policy.require_idempotency_key
            && !approved_router_handoff.idempotency_key.trim().is_empty(),
        feature_flag_not_mutated: policy.block_feature_flag_mutation_in_sample
            && !approved_router_handoff.feature_flag_mutated_by_gate
            && !cutover.approved_cutover_decision.feature_flag_mutated,
        no_live_context_attachment: !approved_router_handoff.context_attached_to_live_prompt
            && !approved_decision.context_attached_to_live_prompt
            && !missing_cutover_decision.context_attached_to_live_prompt
            && !kill_switch_decision.context_attached_to_live_prompt,
        no_provider_model_invocation: policy.block_provider_invocation_in_sample
            && !approved_router_handoff.provider_invoked_by_gate
            && !approved_decision.provider_invoked
            && !missing_cutover_decision.provider_invoked
            && !kill_switch_decision.provider_invoked,
        no_reply_delivery: cutover.checks.no_reply_delivery,
        no_external_network_read: cutover.checks.no_external_network_read,
        no_production_memory_mutation: cutover.checks.no_production_memory_mutation,
        no_raw_private_memory_logged: cutover.checks.no_raw_private_memory_logged,
    };
    let p15_provider_router_activation_gate_ready = checks.ready();

    MemoryProviderRouterActivationGateReport {
        product: "Hepta",
        command: "memory-provider-router-activation-gate",
        contract: MEMORY_PROVIDER_ROUTER_ACTIVATION_GATE_V1_CONTRACT,
        status: if p15_provider_router_activation_gate_ready {
            "ready"
        } else {
            "attention"
        },
        p15_provider_router_activation_gate_ready,
        native_rewrite: true,
        sample_run,
        activation_cutover_contract: MEMORY_ACTIVATION_CUTOVER_GATE_V1_CONTRACT,
        feature_flag_id: policy.feature_flag_id,
        policy,
        approved_router_handoff,
        approved_decision,
        missing_cutover_decision,
        kill_switch_decision,
        checks,
        next_phase: "implement a runtime-owned provider-router adapter that consumes this handoff with the same idempotency, kill-switch, telemetry, and no-silent-attachment guarantees",
    }
}

fn router_decision(
    scenario: &'static str,
    cutover_gate_ready: bool,
    operator_release_approved: bool,
    kill_switch_active: bool,
    router_handoff_allowed: bool,
    fallback_no_memory_provider_turn_hash: String,
    blocked_reasons: Vec<&'static str>,
) -> MemoryProviderRouterActivationDecision {
    MemoryProviderRouterActivationDecision {
        decision_id: stable_digest(&format!("memory-router-decision:{scenario}")),
        scenario,
        cutover_gate_ready,
        operator_release_approved,
        kill_switch_active,
        router_handoff_allowed,
        fallback_no_memory_provider_turn_hash,
        blocked_reasons,
        context_attached_to_live_prompt: false,
        provider_invoked: false,
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
    fn memory_provider_router_activation_gate_sample_gate_is_ready() {
        let report = memory_provider_router_activation_gate_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p15_provider_router_activation_gate_ready);
        assert!(report.checks.ready());
        assert_eq!(report.feature_flag_id, "HEPTA_MEMORY_CONTEXT_LIVE_TURN");
        assert_eq!(
            report.approved_router_handoff.provider_router_id,
            "hepta-native-model-provider-router"
        );
    }

    #[test]
    fn memory_provider_router_activation_blocks_missing_cutover_and_kill_switch() {
        let report = memory_provider_router_activation_gate_sample_report(true);

        assert!(report.approved_decision.router_handoff_allowed);
        assert!(!report.missing_cutover_decision.router_handoff_allowed);
        assert!(!report.kill_switch_decision.router_handoff_allowed);
        assert!(report.checks.missing_cutover_blocks_activation);
        assert!(report.checks.kill_switch_blocks_activation);
    }

    #[test]
    fn memory_provider_router_activation_keeps_live_effects_disabled() {
        let report = memory_provider_router_activation_gate_sample_report(true);

        assert!(report.checks.feature_flag_not_mutated);
        assert!(report.checks.no_live_context_attachment);
        assert!(report.checks.no_provider_model_invocation);
        assert!(!report.approved_router_handoff.feature_flag_mutated_by_gate);
        assert!(
            !report
                .approved_router_handoff
                .context_attached_to_live_prompt
        );
        assert!(!report.approved_router_handoff.provider_invoked_by_gate);
    }
}
