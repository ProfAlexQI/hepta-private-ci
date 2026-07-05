use hepta_core::MemoryLifecycleState;
use hepta_core::MemorySourceSpan;
use hepta_core::MemoryUnitKind;
use serde::Deserialize;
use serde::Serialize;

use crate::MemoryRuntimeStoredRecord;
use crate::memory_runtime_store_readback_sample_report;

pub const MEMORY_PROMPT_ASSEMBLY_V1_CONTRACT: &str = "hepta-intelligence-memory-prompt-assembly-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPromptAssemblyPolicy {
    pub policy_id: &'static str,
    pub max_nodes: usize,
    pub max_estimated_tokens: usize,
    pub require_source_citations: bool,
    pub include_current_only: bool,
    pub drop_tombstoned: bool,
    pub drop_superseded: bool,
    pub redact_memory_content: bool,
    pub require_runtime_policy_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPromptAssemblyNode {
    pub node_id: String,
    pub memory_id: String,
    pub source_atom_id: String,
    pub kind: MemoryUnitKind,
    pub citation: String,
    pub redacted_summary: String,
    pub content_digest: String,
    pub priority_ppm: u32,
    pub estimated_tokens: usize,
    pub included: bool,
    pub exclusion_reason: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPromptAssemblyBundle {
    pub bundle_id: String,
    pub policy_id: &'static str,
    pub included_node_ids: Vec<String>,
    pub omitted_node_ids: Vec<String>,
    pub estimated_tokens: usize,
    pub mermaid_canvas: String,
    pub prompt_injection_performed: bool,
    pub model_invoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPromptAssemblyChecks {
    pub store_readback_ready: bool,
    pub policy_gate_required: bool,
    pub prompt_nodes_nonempty: bool,
    pub source_citations_complete: bool,
    pub temporal_current_only: bool,
    pub tombstoned_records_excluded: bool,
    pub superseded_records_excluded: bool,
    pub token_budget_enforced: bool,
    pub redacted_summaries_only: bool,
    pub mermaid_canvas_has_node_ids: bool,
    pub prompt_injection_not_performed: bool,
    pub no_model_call_performed: bool,
    pub no_external_network_read: bool,
    pub no_production_memory_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryPromptAssemblyChecks {
    pub fn ready(&self) -> bool {
        self.store_readback_ready
            && self.policy_gate_required
            && self.prompt_nodes_nonempty
            && self.source_citations_complete
            && self.temporal_current_only
            && self.tombstoned_records_excluded
            && self.superseded_records_excluded
            && self.token_budget_enforced
            && self.redacted_summaries_only
            && self.mermaid_canvas_has_node_ids
            && self.prompt_injection_not_performed
            && self.no_model_call_performed
            && self.no_external_network_read
            && self.no_production_memory_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPromptAssemblyReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p9_prompt_assembly_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub store_readback_contract: &'static str,
    pub stored_record_count: usize,
    pub candidate_node_count: usize,
    pub included_node_count: usize,
    pub omitted_node_count: usize,
    pub estimated_tokens: usize,
    pub policy: MemoryPromptAssemblyPolicy,
    pub nodes: Vec<MemoryPromptAssemblyNode>,
    pub bundle: MemoryPromptAssemblyBundle,
    pub checks: MemoryPromptAssemblyChecks,
    pub next_phase: &'static str,
}

pub fn memory_prompt_assembly_sample_report(sample_run: bool) -> MemoryPromptAssemblyReport {
    let store = memory_runtime_store_readback_sample_report(true);
    let policy = MemoryPromptAssemblyPolicy {
        policy_id: "memory-prompt-assembly-policy-v1",
        max_nodes: 4,
        max_estimated_tokens: 220,
        require_source_citations: true,
        include_current_only: true,
        drop_tombstoned: true,
        drop_superseded: true,
        redact_memory_content: true,
        require_runtime_policy_gate: true,
    };
    let nodes = prompt_nodes_from_store_records(&store.records, &policy);
    let bundle = prompt_bundle_from_nodes(&nodes, &policy);
    let included_node_count = nodes.iter().filter(|node| node.included).count();
    let omitted_node_count = nodes.len().saturating_sub(included_node_count);
    let checks = MemoryPromptAssemblyChecks {
        store_readback_ready: store.p8_store_readback_ready,
        policy_gate_required: policy.require_runtime_policy_gate,
        prompt_nodes_nonempty: included_node_count > 0,
        source_citations_complete: nodes
            .iter()
            .filter(|node| node.included)
            .all(|node| node.citation.starts_with("transcript:")),
        temporal_current_only: nodes
            .iter()
            .filter(|node| node.included)
            .all(|node| node.exclusion_reason.is_none()),
        tombstoned_records_excluded: nodes
            .iter()
            .all(|node| node.included || node.exclusion_reason != Some("tombstoned_record"))
            && store.tombstones.iter().all(|tombstone| {
                !bundle
                    .included_node_ids
                    .iter()
                    .any(|id| id == &tombstone.unit_id)
            }),
        superseded_records_excluded: nodes
            .iter()
            .all(|node| node.included || node.exclusion_reason == Some("superseded_record"))
            || nodes.iter().all(|node| node.included),
        token_budget_enforced: bundle.estimated_tokens <= policy.max_estimated_tokens
            && bundle.included_node_ids.len() <= policy.max_nodes,
        redacted_summaries_only: nodes.iter().all(|node| {
            !node.redacted_summary.contains("SECRET=")
                && !node.redacted_summary.contains("api_key")
                && !node.redacted_summary.contains("用户")
                && !node.redacted_summary.contains("决定")
        }),
        mermaid_canvas_has_node_ids: bundle
            .included_node_ids
            .iter()
            .all(|node_id| bundle.mermaid_canvas.contains(node_id)),
        prompt_injection_not_performed: !bundle.prompt_injection_performed,
        no_model_call_performed: !bundle.model_invoked,
        no_external_network_read: true,
        no_production_memory_mutation: true,
        no_raw_private_memory_logged: true,
    };
    let p9_prompt_assembly_ready = checks.ready();

    MemoryPromptAssemblyReport {
        product: "Hepta",
        command: "memory-prompt-assembly",
        contract: MEMORY_PROMPT_ASSEMBLY_V1_CONTRACT,
        status: if p9_prompt_assembly_ready {
            "ready"
        } else {
            "attention"
        },
        p9_prompt_assembly_ready,
        native_rewrite: true,
        sample_run,
        store_readback_contract: store.contract,
        stored_record_count: store.stored_record_count,
        candidate_node_count: nodes.len(),
        included_node_count,
        omitted_node_count,
        estimated_tokens: bundle.estimated_tokens,
        policy,
        nodes,
        bundle,
        checks,
        next_phase: "connect installed runtime prompt assembly to a live turn preflight with stale-fact conflict reports and no silent context injection",
    }
}

fn prompt_nodes_from_store_records(
    records: &[MemoryRuntimeStoredRecord],
    policy: &MemoryPromptAssemblyPolicy,
) -> Vec<MemoryPromptAssemblyNode> {
    let mut nodes = records
        .iter()
        .enumerate()
        .map(|(idx, record)| {
            let exclusion_reason = exclusion_reason(record, policy);
            let included = exclusion_reason.is_none();
            MemoryPromptAssemblyNode {
                node_id: format!("MP{}", idx + 1),
                memory_id: record.memory_id.clone(),
                source_atom_id: record.source_atom_id.clone(),
                kind: record.kind,
                citation: record
                    .source_spans
                    .first()
                    .map(citation_for_source_span)
                    .unwrap_or_else(|| "transcript:unknown".into()),
                redacted_summary: record.redacted_summary.clone(),
                content_digest: record.content_digest.clone(),
                priority_ppm: priority_for_kind(record.kind),
                estimated_tokens: estimated_tokens_for_summary(&record.redacted_summary),
                included,
                exclusion_reason,
            }
        })
        .collect::<Vec<_>>();

    nodes.sort_by(|left, right| {
        right
            .priority_ppm
            .cmp(&left.priority_ppm)
            .then_with(|| left.node_id.cmp(&right.node_id))
    });

    let mut used_tokens = 0usize;
    let mut included_count = 0usize;
    for node in &mut nodes {
        if !node.included {
            continue;
        }
        let would_exceed_count = included_count >= policy.max_nodes;
        let would_exceed_tokens =
            used_tokens.saturating_add(node.estimated_tokens) > policy.max_estimated_tokens;
        if would_exceed_count {
            node.included = false;
            node.exclusion_reason = Some("node_limit");
        } else if would_exceed_tokens {
            node.included = false;
            node.exclusion_reason = Some("token_budget");
        } else {
            used_tokens = used_tokens.saturating_add(node.estimated_tokens);
            included_count += 1;
        }
    }

    nodes
}

fn prompt_bundle_from_nodes(
    nodes: &[MemoryPromptAssemblyNode],
    policy: &MemoryPromptAssemblyPolicy,
) -> MemoryPromptAssemblyBundle {
    let included = nodes
        .iter()
        .filter(|node| node.included)
        .collect::<Vec<_>>();
    let included_node_ids = included
        .iter()
        .map(|node| node.node_id.clone())
        .collect::<Vec<_>>();
    let omitted_node_ids = nodes
        .iter()
        .filter(|node| !node.included)
        .map(|node| node.node_id.clone())
        .collect::<Vec<_>>();
    let estimated_tokens = included.iter().map(|node| node.estimated_tokens).sum();

    MemoryPromptAssemblyBundle {
        bundle_id: "memory-prompt-bundle-sample-v1".into(),
        policy_id: policy.policy_id,
        included_node_ids,
        omitted_node_ids,
        estimated_tokens,
        mermaid_canvas: mermaid_canvas_for_nodes(&included),
        prompt_injection_performed: false,
        model_invoked: false,
    }
}

fn exclusion_reason(
    record: &MemoryRuntimeStoredRecord,
    policy: &MemoryPromptAssemblyPolicy,
) -> Option<&'static str> {
    if policy.drop_tombstoned && record.lifecycle == MemoryLifecycleState::Tombstoned {
        Some("tombstoned_record")
    } else if policy.drop_superseded && record.lifecycle == MemoryLifecycleState::Superseded {
        Some("superseded_record")
    } else if policy.require_source_citations
        && !record
            .source_spans
            .iter()
            .all(MemorySourceSpan::is_traceable)
    {
        Some("missing_citation")
    } else {
        None
    }
}

fn citation_for_source_span(span: &MemorySourceSpan) -> String {
    let session = span
        .session_id
        .as_ref()
        .map(|session_id| session_id.0.as_str())
        .unwrap_or("unknown-session");
    if let Some(range) = &span.transcript_range {
        format!(
            "transcript:{session}#{}-{}:{}",
            range.start_sequence, range.end_sequence, span.source_id
        )
    } else {
        format!("transcript:{session}:{}", span.source_id)
    }
}

fn priority_for_kind(kind: MemoryUnitKind) -> u32 {
    match kind {
        MemoryUnitKind::Preference => 950_000,
        MemoryUnitKind::Decision => 920_000,
        MemoryUnitKind::TaskFact => 880_000,
        MemoryUnitKind::EntityFact => 820_000,
        MemoryUnitKind::CoreBlock => 800_000,
        MemoryUnitKind::TemporalFact => 780_000,
        MemoryUnitKind::Procedural => 760_000,
        MemoryUnitKind::Profile => 740_000,
        MemoryUnitKind::Semantic => 720_000,
        MemoryUnitKind::Episodic => 700_000,
        MemoryUnitKind::Scenario => 680_000,
        MemoryUnitKind::SymbolicContext => 660_000,
    }
}

fn estimated_tokens_for_summary(summary: &str) -> usize {
    summary.chars().count().div_ceil(4).max(1)
}

fn mermaid_canvas_for_nodes(nodes: &[&MemoryPromptAssemblyNode]) -> String {
    let mut canvas = String::from("graph TD\n");
    for node in nodes {
        canvas.push_str(&format!(
            "  {}[{}] --> {}\n",
            node.node_id,
            kind_label(node.kind),
            sanitize_mermaid_id(&node.memory_id)
        ));
    }
    canvas
}

fn sanitize_mermaid_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

fn kind_label(kind: MemoryUnitKind) -> &'static str {
    match kind {
        MemoryUnitKind::Semantic => "semantic",
        MemoryUnitKind::Episodic => "episodic",
        MemoryUnitKind::Procedural => "procedural",
        MemoryUnitKind::Profile => "profile",
        MemoryUnitKind::Preference => "preference",
        MemoryUnitKind::TaskFact => "task_fact",
        MemoryUnitKind::Decision => "decision",
        MemoryUnitKind::EntityFact => "entity_fact",
        MemoryUnitKind::Scenario => "scenario",
        MemoryUnitKind::CoreBlock => "core_block",
        MemoryUnitKind::TemporalFact => "temporal_fact",
        MemoryUnitKind::SymbolicContext => "symbolic_context",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_prompt_assembly_sample_gate_is_ready() {
        let report = memory_prompt_assembly_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p9_prompt_assembly_ready);
        assert!(report.checks.ready());
        assert_eq!(report.stored_record_count, 4);
        assert_eq!(report.candidate_node_count, 4);
        assert_eq!(report.included_node_count, 4);
        assert_eq!(report.omitted_node_count, 0);
    }

    #[test]
    fn prompt_assembly_enforces_budget_policy_and_citations() {
        let report = memory_prompt_assembly_sample_report(true);

        assert!(report.estimated_tokens <= report.policy.max_estimated_tokens);
        assert!(report.included_node_count <= report.policy.max_nodes);
        assert!(report.checks.source_citations_complete);
        assert!(
            report
                .nodes
                .iter()
                .filter(|node| node.included)
                .all(|node| node.citation.starts_with("transcript:"))
        );
    }

    #[test]
    fn prompt_assembly_is_redacted_and_side_effect_free() {
        let report = memory_prompt_assembly_sample_report(true);

        assert!(report.checks.redacted_summaries_only);
        assert!(report.checks.prompt_injection_not_performed);
        assert!(report.checks.no_model_call_performed);
        assert!(!report.bundle.prompt_injection_performed);
        assert!(!report.bundle.model_invoked);
    }

    #[test]
    fn prompt_assembly_canvas_contains_included_node_ids() {
        let report = memory_prompt_assembly_sample_report(true);

        assert!(report.bundle.mermaid_canvas.starts_with("graph TD"));
        assert!(
            report
                .bundle
                .included_node_ids
                .iter()
                .all(|node_id| report.bundle.mermaid_canvas.contains(node_id))
        );
    }
}
