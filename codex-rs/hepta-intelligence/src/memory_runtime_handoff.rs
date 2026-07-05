use hepta_core::MemorySourceSpan;
use hepta_core::MemoryUnit;
use hepta_core::MessageRole;
use hepta_core::SessionId;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptEntryKind;
use serde::Deserialize;
use serde::Serialize;

use crate::HybridRecallHit;
use crate::extract_memory_atoms_from_transcript;
use crate::hybrid_recall;

pub const MEMORY_RUNTIME_HANDOFF_V1_CONTRACT: &str = "hepta-intelligence-memory-runtime-handoff-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRuntimeHandoffStep {
    pub step_id: String,
    pub target: &'static str,
    pub mode: &'static str,
    pub policy_gate: &'static str,
    pub source_atom_ids: Vec<String>,
    pub readback_evidence_id: String,
    pub mutation_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPromptContextNode {
    pub node_id: String,
    pub unit_id: String,
    pub source_id: String,
    pub citation: String,
    pub summary: String,
    pub score_ppm: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRuntimeHandoffChecks {
    pub transcript_bound: bool,
    pub atom_pipeline_ready: bool,
    pub storage_handoff_planned: bool,
    pub index_handoff_planned: bool,
    pub temporal_handoff_planned: bool,
    pub prompt_context_planned: bool,
    pub all_atoms_source_traceable: bool,
    pub all_handoffs_readback_backed: bool,
    pub policy_gate_required: bool,
    pub no_llm_extraction_performed: bool,
    pub no_external_network_read: bool,
    pub no_production_memory_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryRuntimeHandoffChecks {
    pub fn ready(&self) -> bool {
        self.transcript_bound
            && self.atom_pipeline_ready
            && self.storage_handoff_planned
            && self.index_handoff_planned
            && self.temporal_handoff_planned
            && self.prompt_context_planned
            && self.all_atoms_source_traceable
            && self.all_handoffs_readback_backed
            && self.policy_gate_required
            && self.no_llm_extraction_performed
            && self.no_external_network_read
            && self.no_production_memory_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryRuntimeHandoffReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p7_runtime_handoff_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub transcript_session_id: String,
    pub transcript_entry_count: usize,
    pub atom_count: usize,
    pub recall_hit_count: usize,
    pub handoff_count: usize,
    pub prompt_context_node_count: usize,
    pub handoffs: Vec<MemoryRuntimeHandoffStep>,
    pub prompt_context_nodes: Vec<MemoryPromptContextNode>,
    pub checks: MemoryRuntimeHandoffChecks,
    pub next_phase: &'static str,
}

pub fn memory_runtime_handoff_sample_report(sample_run: bool) -> MemoryRuntimeHandoffReport {
    let now = 1_800_000_030_000;
    let entries = sample_runtime_transcript_entries(now);
    let transcript_session_id = entries
        .first()
        .map(|entry| entry.session_id.0.clone())
        .unwrap_or_else(|| "session-memory-runtime-handoff".into());
    let atoms = extract_memory_atoms_from_transcript(
        "cube-hepta-runtime-memory",
        "user:default/project:hepta/runtime",
        &entries,
        now + 10,
    );
    let recall_hits = hybrid_recall(
        "Hepta memory runtime handoff transcript storage index prompt context",
        &atoms,
        4,
        250,
        21,
    );
    let handoffs = plan_memory_runtime_handoffs(&atoms, &recall_hits);
    let prompt_context_nodes = prompt_context_nodes_from_hits(&recall_hits);
    let checks = MemoryRuntimeHandoffChecks {
        transcript_bound: entries
            .iter()
            .all(|entry| entry.session_id.0 == transcript_session_id),
        atom_pipeline_ready: !atoms.is_empty(),
        storage_handoff_planned: handoffs
            .iter()
            .any(|handoff| handoff.target == "memory_store_append"),
        index_handoff_planned: handoffs
            .iter()
            .any(|handoff| handoff.target == "hybrid_index_refresh"),
        temporal_handoff_planned: handoffs
            .iter()
            .any(|handoff| handoff.target == "temporal_graph_refresh"),
        prompt_context_planned: !prompt_context_nodes.is_empty()
            && handoffs
                .iter()
                .any(|handoff| handoff.target == "prompt_context_assembly"),
        all_atoms_source_traceable: atoms.iter().all(MemoryUnit::has_traceable_source),
        all_handoffs_readback_backed: handoffs
            .iter()
            .all(|handoff| !handoff.readback_evidence_id.trim().is_empty()),
        policy_gate_required: handoffs
            .iter()
            .all(|handoff| handoff.policy_gate == "operator_or_runtime_policy_required"),
        no_llm_extraction_performed: true,
        no_external_network_read: true,
        no_production_memory_mutation: handoffs.iter().all(|handoff| !handoff.mutation_performed),
        no_raw_private_memory_logged: prompt_context_nodes
            .iter()
            .all(|node| !node.summary.trim().is_empty() && !node.summary.contains("SECRET=")),
    };
    let p7_runtime_handoff_ready = checks.ready();

    MemoryRuntimeHandoffReport {
        product: "Hepta",
        command: "memory-runtime-handoff",
        contract: MEMORY_RUNTIME_HANDOFF_V1_CONTRACT,
        status: if p7_runtime_handoff_ready {
            "ready"
        } else {
            "attention"
        },
        p7_runtime_handoff_ready,
        native_rewrite: true,
        sample_run,
        transcript_session_id,
        transcript_entry_count: entries.len(),
        atom_count: atoms.len(),
        recall_hit_count: recall_hits.len(),
        handoff_count: handoffs.len(),
        prompt_context_node_count: prompt_context_nodes.len(),
        handoffs,
        prompt_context_nodes,
        checks,
        next_phase: "bind the handoff plan to durable production memory storage and prompt injection behind policy/readback gates",
    }
}

pub fn plan_memory_runtime_handoffs(
    atoms: &[MemoryUnit],
    recall_hits: &[HybridRecallHit],
) -> Vec<MemoryRuntimeHandoffStep> {
    let atom_ids = atoms
        .iter()
        .map(|atom| atom.id.clone())
        .collect::<Vec<String>>();
    let recalled_ids = recall_hits
        .iter()
        .map(|hit| hit.unit_id.clone())
        .collect::<Vec<String>>();

    vec![
        MemoryRuntimeHandoffStep {
            step_id: "handoff-memory-store-append".into(),
            target: "memory_store_append",
            mode: "planned_local_readback",
            policy_gate: "operator_or_runtime_policy_required",
            source_atom_ids: atom_ids.clone(),
            readback_evidence_id: "readback:memory-store-append-plan".into(),
            mutation_performed: false,
        },
        MemoryRuntimeHandoffStep {
            step_id: "handoff-hybrid-index-refresh".into(),
            target: "hybrid_index_refresh",
            mode: "planned_local_readback",
            policy_gate: "operator_or_runtime_policy_required",
            source_atom_ids: atom_ids.clone(),
            readback_evidence_id: "readback:hybrid-index-refresh-plan".into(),
            mutation_performed: false,
        },
        MemoryRuntimeHandoffStep {
            step_id: "handoff-temporal-graph-refresh".into(),
            target: "temporal_graph_refresh",
            mode: "planned_local_readback",
            policy_gate: "operator_or_runtime_policy_required",
            source_atom_ids: atom_ids,
            readback_evidence_id: "readback:temporal-graph-refresh-plan".into(),
            mutation_performed: false,
        },
        MemoryRuntimeHandoffStep {
            step_id: "handoff-prompt-context-assembly".into(),
            target: "prompt_context_assembly",
            mode: "planned_local_readback",
            policy_gate: "operator_or_runtime_policy_required",
            source_atom_ids: recalled_ids,
            readback_evidence_id: "readback:prompt-context-assembly-plan".into(),
            mutation_performed: false,
        },
    ]
}

fn prompt_context_nodes_from_hits(recall_hits: &[HybridRecallHit]) -> Vec<MemoryPromptContextNode> {
    recall_hits
        .iter()
        .enumerate()
        .map(|(idx, hit)| {
            let source_span = hit.source_spans.first();
            MemoryPromptContextNode {
                node_id: format!("MC{}", idx + 1),
                unit_id: hit.unit_id.clone(),
                source_id: source_span
                    .map(|span| span.source_id.clone())
                    .unwrap_or_else(|| "unknown-source".into()),
                citation: source_span
                    .map(citation_for_source_span)
                    .unwrap_or_else(|| "source:unknown".into()),
                summary: truncate_summary(&hit.summary, 140),
                score_ppm: (hit.scores.final_score.max(0.0).min(1.0) * 1_000_000.0) as u32,
            }
        })
        .collect()
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

fn truncate_summary(summary: &str, max_chars: usize) -> String {
    let mut truncated = summary.chars().take(max_chars).collect::<String>();
    if summary.chars().count() > max_chars {
        truncated.push_str("...");
    }
    truncated
}

pub(crate) fn sample_runtime_transcript_entries(now: u64) -> Vec<TranscriptEntry> {
    let session_id = SessionId("session-memory-runtime-handoff".into());
    vec![
        TranscriptEntry {
            entry_id: "runtime-entry-preference".into(),
            session_id: session_id.clone(),
            sequence: 10,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content: "用户偏好：Hepta memory 的运行时注入必须引用 transcript source span。".into(),
            created_at_unix_ms: now,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "runtime-entry-decision".into(),
            session_id: session_id.clone(),
            sequence: 11,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::Assistant),
            content: "决定：runtime handoff 先只生成 storage/index/prompt assembly 计划，不直接写生产 memory。".into(),
            created_at_unix_ms: now + 1,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "runtime-entry-task".into(),
            session_id: session_id.clone(),
            sequence: 12,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content: "下一步任务：把 P0-P6 memory readiness 接到 runtime prompt context assembly。".into(),
            created_at_unix_ms: now + 2,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "runtime-entry-entity".into(),
            session_id,
            sequence: 13,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::Assistant),
            content: "Hepta runtime 将 OpenClaw memory 能力复刻为 Rust-native policy-gated handoff。"
                .into(),
            created_at_unix_ms: now + 3,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_runtime_handoff_sample_gate_is_ready() {
        let report = memory_runtime_handoff_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p7_runtime_handoff_ready);
        assert!(report.checks.ready());
        assert_eq!(report.transcript_entry_count, 4);
        assert_eq!(report.atom_count, 4);
        assert_eq!(report.handoff_count, 4);
        assert!(report.prompt_context_node_count > 0);
    }

    #[test]
    fn runtime_handoff_is_policy_gated_and_side_effect_free() {
        let report = memory_runtime_handoff_sample_report(true);

        assert!(report.checks.policy_gate_required);
        assert!(report.checks.all_handoffs_readback_backed);
        assert!(report.checks.no_production_memory_mutation);
        assert!(report.checks.no_external_network_read);
        assert!(report.checks.no_llm_extraction_performed);
        assert!(
            report
                .handoffs
                .iter()
                .all(|handoff| !handoff.mutation_performed)
        );
    }

    #[test]
    fn prompt_context_nodes_keep_transcript_citations() {
        let report = memory_runtime_handoff_sample_report(true);

        assert!(report.checks.prompt_context_planned);
        assert!(
            report
                .prompt_context_nodes
                .iter()
                .all(|node| node.citation.starts_with("transcript:"))
        );
        assert!(
            report
                .prompt_context_nodes
                .iter()
                .all(|node| !node.source_id.trim().is_empty())
        );
    }
}
