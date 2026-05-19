use hepta_core::{
    CoreMemoryBlock, CoreMemoryBlockKind, MemorySourceKind, MemorySourceSpan, TranscriptRange,
};
use serde::{Deserialize, Serialize};

use crate::{
    memory_atom_pipeline_sample_report, memory_hybrid_recall_sample_report,
    memory_temporal_graph_sample_report,
};

pub const MEMORY_SHORT_TERM_OFFLOAD_V1_CONTRACT: &str = "hepta-intelligence-short-term-offload-v1";
pub const MEMORY_CORE_BLOCKS_V1_CONTRACT: &str = "hepta-intelligence-core-memory-blocks-v1";
pub const MEMORY_EVAL_GATE_V1_CONTRACT: &str = "hepta-intelligence-memory-eval-gate-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortTermOffloadRef {
    pub node_id: String,
    pub result_ref: String,
    pub raw_token_estimate: usize,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortTermOffloadChecks {
    pub refs_paths_planned: bool,
    pub node_ids_present: bool,
    pub mermaid_canvas_present: bool,
    pub canvas_links_all_refs: bool,
    pub prompt_uses_canvas_not_raw_logs: bool,
    pub token_saving_ready: bool,
    pub filesystem_mutation_performed: bool,
    pub raw_tool_log_injected: bool,
}

impl ShortTermOffloadChecks {
    pub fn ready(&self) -> bool {
        self.refs_paths_planned
            && self.node_ids_present
            && self.mermaid_canvas_present
            && self.canvas_links_all_refs
            && self.prompt_uses_canvas_not_raw_logs
            && self.token_saving_ready
            && !self.filesystem_mutation_performed
            && !self.raw_tool_log_injected
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortTermOffloadReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p4_ready: bool,
    pub sample_run: bool,
    pub raw_token_estimate: usize,
    pub canvas_token_estimate: usize,
    pub token_saving_ppm: u32,
    pub refs: Vec<ShortTermOffloadRef>,
    pub mermaid_canvas: String,
    pub checks: ShortTermOffloadChecks,
    pub next_phase: &'static str,
}

pub fn memory_short_term_offload_sample_report(sample_run: bool) -> ShortTermOffloadReport {
    let refs = vec![
        ShortTermOffloadRef {
            node_id: "N1".into(),
            result_ref: "refs/session-memory-offload/N1-tool-result.md".into(),
            raw_token_estimate: 2400,
            digest: "sha256:offload-n1".into(),
        },
        ShortTermOffloadRef {
            node_id: "N2".into(),
            result_ref: "refs/session-memory-offload/N2-error-trace.md".into(),
            raw_token_estimate: 3100,
            digest: "sha256:offload-n2".into(),
        },
        ShortTermOffloadRef {
            node_id: "N3".into(),
            result_ref: "refs/session-memory-offload/N3-resolution.md".into(),
            raw_token_estimate: 900,
            digest: "sha256:offload-n3".into(),
        },
    ];
    let mermaid_canvas = "graph LR\n  N1[tool result\\nref:N1] --> N2[error trace\\nref:N2]\n  N2 --> N3[resolution\\nref:N3]\n".to_string();
    let raw_token_estimate = refs
        .iter()
        .map(|item| item.raw_token_estimate)
        .sum::<usize>();
    let canvas_token_estimate = 96usize;
    let token_saving_ppm = (((raw_token_estimate - canvas_token_estimate) as f64
        / raw_token_estimate as f64)
        * 1_000_000.0) as u32;
    let checks = ShortTermOffloadChecks {
        refs_paths_planned: refs
            .iter()
            .all(|item| item.result_ref.starts_with("refs/") && item.result_ref.ends_with(".md")),
        node_ids_present: refs.iter().all(|item| !item.node_id.is_empty()),
        mermaid_canvas_present: mermaid_canvas.starts_with("graph LR"),
        canvas_links_all_refs: refs
            .iter()
            .all(|item| mermaid_canvas.contains(&item.node_id)),
        prompt_uses_canvas_not_raw_logs: canvas_token_estimate < raw_token_estimate / 4,
        token_saving_ready: token_saving_ppm >= 500_000,
        filesystem_mutation_performed: false,
        raw_tool_log_injected: false,
    };
    let p4_ready = checks.ready();

    ShortTermOffloadReport {
        product: "Hepta",
        command: "memory-short-term-offload",
        contract: MEMORY_SHORT_TERM_OFFLOAD_V1_CONTRACT,
        status: if p4_ready { "ready" } else { "attention" },
        p4_ready,
        sample_run,
        raw_token_estimate,
        canvas_token_estimate,
        token_saving_ppm,
        refs,
        mermaid_canvas,
        checks,
        next_phase: "P5 editable pinned CoreMemoryBlock runtime management",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreMemoryBlockChecks {
    pub identity_block_present: bool,
    pub project_state_block_present: bool,
    pub stable_preference_block_present: bool,
    pub active_objective_block_present: bool,
    pub all_blocks_editable_or_pinned: bool,
    pub all_blocks_have_source_provenance: bool,
    pub audit_revision_supported: bool,
    pub no_private_memory_logged: bool,
}

impl CoreMemoryBlockChecks {
    pub fn ready(&self) -> bool {
        self.identity_block_present
            && self.project_state_block_present
            && self.stable_preference_block_present
            && self.active_objective_block_present
            && self.all_blocks_editable_or_pinned
            && self.all_blocks_have_source_provenance
            && self.audit_revision_supported
            && self.no_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreMemoryBlocksReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p5_ready: bool,
    pub sample_run: bool,
    pub block_count: usize,
    pub revision_count: usize,
    pub blocks: Vec<CoreMemoryBlock>,
    pub checks: CoreMemoryBlockChecks,
    pub next_phase: &'static str,
}

pub fn memory_core_blocks_sample_report(sample_run: bool) -> CoreMemoryBlocksReport {
    let blocks = sample_core_blocks();
    let checks = CoreMemoryBlockChecks {
        identity_block_present: blocks
            .iter()
            .any(|block| block.block_kind == CoreMemoryBlockKind::Identity),
        project_state_block_present: blocks
            .iter()
            .any(|block| block.block_kind == CoreMemoryBlockKind::ProjectState),
        stable_preference_block_present: blocks
            .iter()
            .any(|block| block.block_kind == CoreMemoryBlockKind::StablePreference),
        active_objective_block_present: blocks
            .iter()
            .any(|block| block.block_kind == CoreMemoryBlockKind::ActiveObjective),
        all_blocks_editable_or_pinned: blocks.iter().all(|block| block.editable || block.pinned),
        all_blocks_have_source_provenance: blocks.iter().all(CoreMemoryBlock::has_traceable_source),
        audit_revision_supported: blocks.iter().all(|block| block.version >= 1),
        no_private_memory_logged: true,
    };
    let p5_ready = checks.ready();

    CoreMemoryBlocksReport {
        product: "Hepta",
        command: "memory-core-blocks",
        contract: MEMORY_CORE_BLOCKS_V1_CONTRACT,
        status: if p5_ready { "ready" } else { "attention" },
        p5_ready,
        sample_run,
        block_count: blocks.len(),
        revision_count: blocks.iter().map(|block| block.version as usize).sum(),
        blocks,
        checks,
        next_phase: "P6 memory evaluation gate in CI",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryEvalGateChecks {
    pub p0_kernel_ready: bool,
    pub p1_atom_pipeline_ready: bool,
    pub p2_hybrid_recall_ready: bool,
    pub p3_temporal_graph_ready: bool,
    pub p4_short_term_offload_ready: bool,
    pub p5_core_blocks_ready: bool,
    pub cross_session_preference_recall: bool,
    pub conflict_supersession_covered: bool,
    pub tombstoned_memory_not_recalled: bool,
    pub source_span_traceability: bool,
    pub token_saving_covered: bool,
    pub long_session_recovery_covered: bool,
}

impl MemoryEvalGateChecks {
    pub fn ready(&self) -> bool {
        self.p0_kernel_ready
            && self.p1_atom_pipeline_ready
            && self.p2_hybrid_recall_ready
            && self.p3_temporal_graph_ready
            && self.p4_short_term_offload_ready
            && self.p5_core_blocks_ready
            && self.cross_session_preference_recall
            && self.conflict_supersession_covered
            && self.tombstoned_memory_not_recalled
            && self.source_span_traceability
            && self.token_saving_covered
            && self.long_session_recovery_covered
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryEvalGateReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p6_ready: bool,
    pub sample_run: bool,
    pub eval_case_count: usize,
    pub passed_case_count: usize,
    pub checks: MemoryEvalGateChecks,
    pub next_phase: &'static str,
}

pub fn memory_eval_gate_sample_report(sample_run: bool) -> MemoryEvalGateReport {
    let p0 = hepta_core::memory_kernel_v1_sample_report(true);
    let p1 = memory_atom_pipeline_sample_report(true);
    let p2 = memory_hybrid_recall_sample_report(true);
    let p3 = memory_temporal_graph_sample_report(true);
    let p4 = memory_short_term_offload_sample_report(true);
    let p5 = memory_core_blocks_sample_report(true);
    let checks = MemoryEvalGateChecks {
        p0_kernel_ready: p0.p0_ready,
        p1_atom_pipeline_ready: p1.p1_ready,
        p2_hybrid_recall_ready: p2.p2_ready,
        p3_temporal_graph_ready: p3.p3_ready,
        p4_short_term_offload_ready: p4.p4_ready,
        p5_core_blocks_ready: p5.p5_ready,
        cross_session_preference_recall: p2
            .hits
            .iter()
            .any(|hit| hit.kind == hepta_core::MemoryUnitKind::Preference),
        conflict_supersession_covered: !p3.superseded_fact_ids.is_empty(),
        tombstoned_memory_not_recalled: p0.checks.tombstoned_unit_not_recalled,
        source_span_traceability: p0.recall_bundle.provenance_complete
            && p1.checks.all_source_spans_traceable
            && p2.checks.provenance_complete
            && p3.checks.provenance_complete
            && p5.checks.all_blocks_have_source_provenance,
        token_saving_covered: p4.checks.token_saving_ready,
        long_session_recovery_covered: p4.checks.canvas_links_all_refs
            && p2.checks.timeout_nonblocking,
    };
    let p6_ready = checks.ready();
    let eval_case_count = 12;
    let passed_case_count = if p6_ready { eval_case_count } else { 0 };

    MemoryEvalGateReport {
        product: "Hepta",
        command: "memory-eval-gate",
        contract: MEMORY_EVAL_GATE_V1_CONTRACT,
        status: if p6_ready { "ready" } else { "attention" },
        p6_ready,
        sample_run,
        eval_case_count,
        passed_case_count,
        checks,
        next_phase: "wire gates into CI and promote production-backed storage/index implementations",
    }
}

fn sample_core_blocks() -> Vec<CoreMemoryBlock> {
    vec![
        sample_block(
            "core-identity",
            "Hepta Identity",
            CoreMemoryBlockKind::Identity,
            true,
            true,
            "Hepta is a Rust-native intelligence runtime with transcript-backed memory.",
            1,
        ),
        sample_block(
            "core-project-state",
            "Hepta Memory Project",
            CoreMemoryBlockKind::ProjectState,
            true,
            true,
            "Memory upgrade has P0-P6 phase gates and must stay auditable.",
            2,
        ),
        sample_block(
            "core-stable-preference",
            "Stable Memory Preference",
            CoreMemoryBlockKind::StablePreference,
            true,
            true,
            "Prefer source-span-backed memory over opaque summaries.",
            3,
        ),
        sample_block(
            "core-active-objective",
            "Active Objective",
            CoreMemoryBlockKind::ActiveObjective,
            true,
            true,
            "Complete Hepta Intelligence memory kernel through eval gates.",
            4,
        ),
    ]
}

fn sample_block(
    id: &str,
    title: &str,
    block_kind: CoreMemoryBlockKind,
    pinned: bool,
    editable: bool,
    content: &str,
    sequence: u64,
) -> CoreMemoryBlock {
    CoreMemoryBlock {
        id: id.into(),
        cube_id: "cube-hepta-intelligence".into(),
        title: title.into(),
        block_kind,
        pinned,
        editable,
        version: 1,
        content: content.into(),
        source_unit_ids: vec![format!("unit-{id}")],
        source_spans: vec![MemorySourceSpan {
            source_kind: MemorySourceKind::Transcript,
            source_id: format!("span-{id}"),
            session_id: Some(hepta_core::SessionId("session-core-blocks".into())),
            transcript_range: Some(TranscriptRange {
                start_sequence: sequence,
                end_sequence: sequence,
            }),
            transcript_entry_ids: vec![format!("session-core-blocks:{sequence}")],
            transcript_span_ref: None,
            evidence_digest: format!("sha256:span-{id}"),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_term_offload_sample_gate_is_ready() {
        let report = memory_short_term_offload_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p4_ready);
        assert!(report.checks.ready());
        assert!(report.token_saving_ppm >= 500_000);
    }

    #[test]
    fn core_memory_blocks_sample_gate_is_ready() {
        let report = memory_core_blocks_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p5_ready);
        assert!(report.checks.ready());
        assert_eq!(report.block_count, 4);
    }

    #[test]
    fn memory_eval_gate_covers_all_phase_requirements() {
        let report = memory_eval_gate_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p6_ready);
        assert!(report.checks.ready());
        assert_eq!(report.eval_case_count, report.passed_case_count);
    }
}
