use serde::{Deserialize, Serialize};

use crate::{
    MEMORY_PROMPT_ASSEMBLY_V1_CONTRACT, MEMORY_RUNTIME_STORE_READBACK_V1_CONTRACT,
    memory_prompt_assembly_sample_report, memory_runtime_store_readback_sample_report,
};

pub const MEMORY_LIVE_TURN_PREFLIGHT_V1_CONTRACT: &str =
    "hepta-intelligence-memory-live-turn-preflight-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryLiveTurnPreflightPolicy {
    pub policy_id: &'static str,
    pub require_runtime_policy_gate: bool,
    pub require_visible_context_preview: bool,
    pub require_stale_fact_conflict_report: bool,
    pub require_exact_node_readback: bool,
    pub block_silent_context_injection: bool,
    pub require_user_scope_match: bool,
    pub max_included_nodes: usize,
    pub max_estimated_tokens: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryLiveTurnContextPreview {
    pub turn_id: String,
    pub session_id: String,
    pub prompt_bundle_id: String,
    pub included_node_ids: Vec<String>,
    pub estimated_tokens: usize,
    pub preview_hash: String,
    pub visible_to_operator: bool,
    pub injection_allowed: bool,
    pub policy_gate_id: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryStaleFactConflictReport {
    pub conflict_id: String,
    pub memory_id: String,
    pub conflict_kind: &'static str,
    pub reason: String,
    pub source: String,
    pub action: &'static str,
    pub blocks_injection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryLiveTurnPreflightChecks {
    pub prompt_assembly_ready: bool,
    pub runtime_policy_gate_required: bool,
    pub visible_context_preview: bool,
    pub exact_node_readback: bool,
    pub source_citations_complete: bool,
    pub stale_fact_conflicts_reported: bool,
    pub tombstones_blocked: bool,
    pub silent_context_injection_blocked: bool,
    pub user_scope_matched: bool,
    pub token_budget_enforced: bool,
    pub no_model_invocation: bool,
    pub no_prompt_injection: bool,
    pub no_external_network_read: bool,
    pub no_production_memory_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryLiveTurnPreflightChecks {
    pub fn ready(&self) -> bool {
        self.prompt_assembly_ready
            && self.runtime_policy_gate_required
            && self.visible_context_preview
            && self.exact_node_readback
            && self.source_citations_complete
            && self.stale_fact_conflicts_reported
            && self.tombstones_blocked
            && self.silent_context_injection_blocked
            && self.user_scope_matched
            && self.token_budget_enforced
            && self.no_model_invocation
            && self.no_prompt_injection
            && self.no_external_network_read
            && self.no_production_memory_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryLiveTurnPreflightReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p10_live_turn_preflight_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub prompt_assembly_contract: &'static str,
    pub store_readback_contract: &'static str,
    pub prompt_bundle_id: String,
    pub included_node_count: usize,
    pub stale_conflict_count: usize,
    pub estimated_tokens: usize,
    pub policy: MemoryLiveTurnPreflightPolicy,
    pub preview: MemoryLiveTurnContextPreview,
    pub stale_fact_conflicts: Vec<MemoryStaleFactConflictReport>,
    pub checks: MemoryLiveTurnPreflightChecks,
    pub next_phase: &'static str,
}

pub fn memory_live_turn_preflight_sample_report(sample_run: bool) -> MemoryLiveTurnPreflightReport {
    let prompt = memory_prompt_assembly_sample_report(true);
    let store = memory_runtime_store_readback_sample_report(true);
    let policy = MemoryLiveTurnPreflightPolicy {
        policy_id: "memory-live-turn-preflight-policy-v1",
        require_runtime_policy_gate: true,
        require_visible_context_preview: true,
        require_stale_fact_conflict_report: true,
        require_exact_node_readback: true,
        block_silent_context_injection: true,
        require_user_scope_match: true,
        max_included_nodes: prompt.policy.max_nodes,
        max_estimated_tokens: prompt.policy.max_estimated_tokens,
    };
    let preview = context_preview_from_prompt(&prompt, &policy);
    let stale_fact_conflicts = stale_fact_conflicts_from_store(&store);
    let included_node_count = preview.included_node_ids.len();
    let checks = MemoryLiveTurnPreflightChecks {
        prompt_assembly_ready: prompt.p9_prompt_assembly_ready,
        runtime_policy_gate_required: policy.require_runtime_policy_gate,
        visible_context_preview: policy.require_visible_context_preview
            && preview.visible_to_operator
            && !preview.preview_hash.trim().is_empty(),
        exact_node_readback: preview.included_node_ids.iter().all(|node_id| {
            prompt
                .nodes
                .iter()
                .any(|node| node.included && node.node_id == *node_id)
        }),
        source_citations_complete: prompt.checks.source_citations_complete,
        stale_fact_conflicts_reported: policy.require_stale_fact_conflict_report
            && !stale_fact_conflicts.is_empty(),
        tombstones_blocked: store.tombstones.iter().all(|tombstone| {
            stale_fact_conflicts.iter().any(|conflict| {
                conflict.memory_id == tombstone.unit_id
                    && conflict.conflict_kind == "tombstone"
                    && conflict.blocks_injection
            })
        }),
        silent_context_injection_blocked: policy.block_silent_context_injection
            && !preview.injection_allowed
            && !prompt.bundle.prompt_injection_performed,
        user_scope_matched: policy.require_user_scope_match
            && prompt
                .nodes
                .iter()
                .filter(|node| node.included)
                .all(|node| node.citation.contains(&preview.session_id)),
        token_budget_enforced: preview.estimated_tokens <= policy.max_estimated_tokens
            && included_node_count <= policy.max_included_nodes,
        no_model_invocation: !prompt.bundle.model_invoked,
        no_prompt_injection: !prompt.bundle.prompt_injection_performed,
        no_external_network_read: prompt.checks.no_external_network_read,
        no_production_memory_mutation: prompt.checks.no_production_memory_mutation,
        no_raw_private_memory_logged: prompt.checks.no_raw_private_memory_logged,
    };
    let p10_live_turn_preflight_ready = checks.ready();

    MemoryLiveTurnPreflightReport {
        product: "Hepta",
        command: "memory-live-turn-preflight",
        contract: MEMORY_LIVE_TURN_PREFLIGHT_V1_CONTRACT,
        status: if p10_live_turn_preflight_ready {
            "ready"
        } else {
            "attention"
        },
        p10_live_turn_preflight_ready,
        native_rewrite: true,
        sample_run,
        prompt_assembly_contract: MEMORY_PROMPT_ASSEMBLY_V1_CONTRACT,
        store_readback_contract: MEMORY_RUNTIME_STORE_READBACK_V1_CONTRACT,
        prompt_bundle_id: prompt.bundle.bundle_id.clone(),
        included_node_count,
        stale_conflict_count: stale_fact_conflicts.len(),
        estimated_tokens: preview.estimated_tokens,
        policy,
        preview,
        stale_fact_conflicts,
        checks,
        next_phase: "wire live turn preflight into the turn dispatcher only after explicit policy approval, readback evidence, and operator-visible context preview",
    }
}

fn context_preview_from_prompt(
    prompt: &crate::MemoryPromptAssemblyReport,
    policy: &MemoryLiveTurnPreflightPolicy,
) -> MemoryLiveTurnContextPreview {
    let included_node_ids = prompt.bundle.included_node_ids.clone();
    let preview_fingerprint = format!(
        "{}:{}:{}:{}",
        prompt.bundle.bundle_id,
        included_node_ids.join(","),
        prompt.bundle.estimated_tokens,
        policy.policy_id
    );
    MemoryLiveTurnContextPreview {
        turn_id: "turn-memory-live-preflight-sample".into(),
        session_id: "session-memory-runtime-handoff".into(),
        prompt_bundle_id: prompt.bundle.bundle_id.clone(),
        included_node_ids,
        estimated_tokens: prompt.bundle.estimated_tokens,
        preview_hash: stable_digest(&preview_fingerprint),
        visible_to_operator: true,
        injection_allowed: false,
        policy_gate_id: policy.policy_id,
    }
}

fn stale_fact_conflicts_from_store(
    store: &crate::MemoryRuntimeStoreReadbackReport,
) -> Vec<MemoryStaleFactConflictReport> {
    let mut conflicts = Vec::new();
    for tombstone in &store.tombstones {
        conflicts.push(MemoryStaleFactConflictReport {
            conflict_id: format!("stale-conflict:{}", tombstone.unit_id),
            memory_id: tombstone.unit_id.clone(),
            conflict_kind: "tombstone",
            reason: tombstone.reason.clone(),
            source: format!(
                "delete:{}:{}",
                tombstone.deleted_by, tombstone.source_span_count_at_delete
            ),
            action: "exclude_and_report",
            blocks_injection: true,
        });
    }
    for memory_id in &store.temporal_readback.superseded_memory_ids {
        conflicts.push(MemoryStaleFactConflictReport {
            conflict_id: format!("stale-conflict:{memory_id}"),
            memory_id: memory_id.clone(),
            conflict_kind: "superseded",
            reason: store.temporal_readback.conflict_policy.into(),
            source: "temporal_readback".into(),
            action: "exclude_and_report",
            blocks_injection: true,
        });
    }
    conflicts
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
    fn memory_live_turn_preflight_sample_gate_is_ready() {
        let report = memory_live_turn_preflight_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p10_live_turn_preflight_ready);
        assert!(report.checks.ready());
        assert_eq!(report.included_node_count, 4);
        assert_eq!(report.stale_conflict_count, 1);
    }

    #[test]
    fn live_turn_preflight_blocks_silent_context_injection() {
        let report = memory_live_turn_preflight_sample_report(true);

        assert!(report.checks.visible_context_preview);
        assert!(report.checks.silent_context_injection_blocked);
        assert!(!report.preview.injection_allowed);
        assert!(report.preview.visible_to_operator);
        assert!(!report.preview.preview_hash.trim().is_empty());
    }

    #[test]
    fn live_turn_preflight_reports_stale_fact_conflicts() {
        let report = memory_live_turn_preflight_sample_report(true);

        assert!(report.checks.stale_fact_conflicts_reported);
        assert!(report.checks.tombstones_blocked);
        assert!(
            report.stale_fact_conflicts.iter().all(
                |conflict| conflict.blocks_injection && conflict.action == "exclude_and_report"
            )
        );
    }
}
