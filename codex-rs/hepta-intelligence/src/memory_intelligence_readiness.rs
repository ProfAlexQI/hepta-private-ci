use serde::Deserialize;
use serde::Serialize;

use crate::memory_atom_pipeline_sample_report;
use crate::memory_core_blocks_sample_report;
use crate::memory_eval_gate_sample_report;
use crate::memory_hybrid_recall_sample_report;
use crate::memory_short_term_offload_sample_report;
use crate::memory_temporal_graph_sample_report;

pub const MEMORY_INTELLIGENCE_READINESS_V1_CONTRACT: &str =
    "hepta-intelligence-memory-readiness-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryIntelligencePhaseStatus {
    pub phase: &'static str,
    pub command: &'static str,
    pub ready: bool,
    pub evidence: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryIntelligenceReadinessChecks {
    pub all_phase_gates_ready: bool,
    pub transcript_source_of_truth: bool,
    pub add_only_atom_pipeline: bool,
    pub hybrid_recall_rrf_ready: bool,
    pub temporal_conflict_resolution_ready: bool,
    pub short_term_offload_ready: bool,
    pub core_blocks_ready: bool,
    pub eval_gate_ready: bool,
    pub source_provenance_complete: bool,
    pub delete_tombstone_covered: bool,
    pub timeout_nonblocking: bool,
    pub no_llm_extraction_performed: bool,
    pub no_external_network_read: bool,
    pub no_external_side_effects: bool,
    pub no_production_memory_mutation: bool,
    pub no_raw_private_memory_logged: bool,
}

impl MemoryIntelligenceReadinessChecks {
    pub fn ready(&self) -> bool {
        self.all_phase_gates_ready
            && self.transcript_source_of_truth
            && self.add_only_atom_pipeline
            && self.hybrid_recall_rrf_ready
            && self.temporal_conflict_resolution_ready
            && self.short_term_offload_ready
            && self.core_blocks_ready
            && self.eval_gate_ready
            && self.source_provenance_complete
            && self.delete_tombstone_covered
            && self.timeout_nonblocking
            && self.no_llm_extraction_performed
            && self.no_external_network_read
            && self.no_external_side_effects
            && self.no_production_memory_mutation
            && self.no_raw_private_memory_logged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryIntelligenceReadinessReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub sample_run: bool,
    pub phase_count: usize,
    pub ready_phase_count: usize,
    pub p0_kernel_ready: bool,
    pub p1_atom_pipeline_ready: bool,
    pub p2_hybrid_recall_ready: bool,
    pub p3_temporal_graph_ready: bool,
    pub p4_short_term_offload_ready: bool,
    pub p5_core_blocks_ready: bool,
    pub p6_eval_gate_ready: bool,
    pub phases: Vec<MemoryIntelligencePhaseStatus>,
    pub checks: MemoryIntelligenceReadinessChecks,
    pub next_phase: &'static str,
}

pub fn memory_intelligence_readiness_sample_report(
    sample_run: bool,
) -> MemoryIntelligenceReadinessReport {
    let p0 = hepta_core::memory_kernel_v1_sample_report(true);
    let p1 = memory_atom_pipeline_sample_report(true);
    let p2 = memory_hybrid_recall_sample_report(true);
    let p3 = memory_temporal_graph_sample_report(true);
    let p4 = memory_short_term_offload_sample_report(true);
    let p5 = memory_core_blocks_sample_report(true);
    let p6 = memory_eval_gate_sample_report(true);

    let phases = vec![
        MemoryIntelligencePhaseStatus {
            phase: "P0",
            command: "memory-kernel",
            ready: p0.p0_ready,
            evidence: "MemoryCube/MemoryUnit lifecycle, provenance, tombstone, conflict, and recall bundle contract",
        },
        MemoryIntelligencePhaseStatus {
            phase: "P1",
            command: "memory-atom-pipeline",
            ready: p1.p1_ready,
            evidence: "deterministic transcript-to-atom extraction for preference, decision, task fact, and entity fact",
        },
        MemoryIntelligencePhaseStatus {
            phase: "P2",
            command: "memory-hybrid-recall",
            ready: p2.p2_ready,
            evidence: "BM25-like lexical score, embedding slot, entity, graph, recency, RRF fusion, and timeout fallback",
        },
        MemoryIntelligencePhaseStatus {
            phase: "P3",
            command: "memory-temporal-graph",
            ready: p3.p3_ready,
            evidence: "current/past/superseded fact states with provenance and inhibition reasons",
        },
        MemoryIntelligencePhaseStatus {
            phase: "P4",
            command: "memory-short-term-offload",
            ready: p4.p4_ready,
            evidence: "refs/*.md offload plan, Mermaid canvas, node_id links, and token-saving proof",
        },
        MemoryIntelligencePhaseStatus {
            phase: "P5",
            command: "memory-core-blocks",
            ready: p5.p5_ready,
            evidence: "editable or pinned CoreMemoryBlock identity/project/preference/objective blocks with source spans",
        },
        MemoryIntelligencePhaseStatus {
            phase: "P6",
            command: "memory-eval-gate",
            ready: p6.p6_ready,
            evidence: "cross-phase eval coverage for recall, supersession, tombstone, provenance, and long-session recovery",
        },
    ];

    let ready_phase_count = phases.iter().filter(|phase| phase.ready).count();
    let all_phase_gates_ready = ready_phase_count == phases.len();
    let checks = MemoryIntelligenceReadinessChecks {
        all_phase_gates_ready,
        transcript_source_of_truth: p1.checks.transcript_source_of_truth,
        add_only_atom_pipeline: p1.checks.add_only_no_updates_or_deletes,
        hybrid_recall_rrf_ready: p2.checks.rrf_fusion_ready,
        temporal_conflict_resolution_ready: p3.checks.superseded_fact_present
            && p3.checks.conflict_or_inhibition_visible,
        short_term_offload_ready: p4.checks.ready(),
        core_blocks_ready: p5.checks.ready(),
        eval_gate_ready: p6.checks.ready(),
        source_provenance_complete: p0.recall_bundle.provenance_complete
            && p1.checks.all_source_spans_traceable
            && p2.checks.provenance_complete
            && p3.checks.provenance_complete
            && p5.checks.all_blocks_have_source_provenance,
        delete_tombstone_covered: p0.checks.tombstone_supported
            && p0.checks.delete_cascade_required
            && p0.checks.tombstoned_unit_not_recalled,
        timeout_nonblocking: p2.checks.timeout_nonblocking && p2.fallback_nonblocking,
        no_llm_extraction_performed: !p0.llm_extraction_performed && !p1.llm_extraction_performed,
        no_external_network_read: !p0.external_network_read
            && !p1.external_network_read
            && !p2.external_network_read,
        no_external_side_effects: p0.checks.no_external_side_effects
            && p1.checks.no_external_side_effects
            && p2.checks.no_external_side_effects
            && p3.checks.no_external_side_effects
            && !p4.checks.filesystem_mutation_performed
            && !p4.checks.raw_tool_log_injected,
        no_production_memory_mutation: !p0.memory_store_mutation_performed
            && !p1.memory_store_mutation_performed
            && !p4.checks.filesystem_mutation_performed,
        no_raw_private_memory_logged: !p0.raw_private_memory_logged
            && !p1.raw_private_memory_logged
            && p5.checks.no_private_memory_logged,
    };
    let status = if checks.ready() { "ready" } else { "attention" };

    MemoryIntelligenceReadinessReport {
        product: "Hepta",
        command: "memory-intelligence",
        contract: MEMORY_INTELLIGENCE_READINESS_V1_CONTRACT,
        status,
        sample_run,
        phase_count: phases.len(),
        ready_phase_count,
        p0_kernel_ready: p0.p0_ready,
        p1_atom_pipeline_ready: p1.p1_ready,
        p2_hybrid_recall_ready: p2.p2_ready,
        p3_temporal_graph_ready: p3.p3_ready,
        p4_short_term_offload_ready: p4.p4_ready,
        p5_core_blocks_ready: p5.p5_ready,
        p6_eval_gate_ready: p6.p6_ready,
        phases,
        checks,
        next_phase: "use /memory-runtime-handoff to bind runtime storage, indexing, temporal refresh, and prompt assembly plans to durable policy/readback gates",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_intelligence_readiness_aggregates_p0_to_p6() {
        let report = memory_intelligence_readiness_sample_report(true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.phase_count, 7);
        assert_eq!(report.ready_phase_count, 7);
        assert!(report.checks.ready());
        assert!(report.p0_kernel_ready);
        assert!(report.p1_atom_pipeline_ready);
        assert!(report.p2_hybrid_recall_ready);
        assert!(report.p3_temporal_graph_ready);
        assert!(report.p4_short_term_offload_ready);
        assert!(report.p5_core_blocks_ready);
        assert!(report.p6_eval_gate_ready);
        assert!(report.checks.source_provenance_complete);
        assert!(report.checks.delete_tombstone_covered);
        assert!(report.checks.timeout_nonblocking);
        assert!(report.checks.no_external_side_effects);
        assert!(report.checks.no_production_memory_mutation);
    }
}
