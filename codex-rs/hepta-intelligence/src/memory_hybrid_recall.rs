use std::collections::BTreeSet;

use hepta_core::MemoryLinkKind;
use hepta_core::MemorySourceSpan;
use hepta_core::MemoryUnit;
use hepta_core::MemoryUnitKind;
use serde::Deserialize;
use serde::Serialize;

use crate::memory_atom_pipeline_sample_report;

pub const MEMORY_HYBRID_RECALL_V1_CONTRACT: &str = "hepta-intelligence-hybrid-recall-v1";
const RRF_K: f32 = 60.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HybridRecallScores {
    pub bm25: f32,
    pub embedding_slot: f32,
    pub entity: f32,
    pub graph: f32,
    pub recency: f32,
    pub rrf: f32,
    pub final_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HybridRecallHit {
    pub unit_id: String,
    pub kind: MemoryUnitKind,
    pub summary: String,
    pub scores: HybridRecallScores,
    #[serde(default)]
    pub source_spans: Vec<MemorySourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HybridRecallChecks {
    pub bm25_signal_ready: bool,
    pub embedding_slot_ready: bool,
    pub entity_signal_ready: bool,
    pub graph_signal_ready: bool,
    pub recency_signal_ready: bool,
    pub rrf_fusion_ready: bool,
    pub provenance_complete: bool,
    pub timeout_nonblocking: bool,
    pub no_remote_embedding_invoked: bool,
    pub no_external_side_effects: bool,
}

impl HybridRecallChecks {
    pub fn ready(&self) -> bool {
        self.bm25_signal_ready
            && self.embedding_slot_ready
            && self.entity_signal_ready
            && self.graph_signal_ready
            && self.recency_signal_ready
            && self.rrf_fusion_ready
            && self.provenance_complete
            && self.timeout_nonblocking
            && self.no_remote_embedding_invoked
            && self.no_external_side_effects
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HybridRecallReport {
    pub product: &'static str,
    pub command: &'static str,
    pub contract: &'static str,
    pub status: &'static str,
    pub p2_ready: bool,
    pub native_rewrite: bool,
    pub sample_run: bool,
    pub query: String,
    pub candidate_count: usize,
    pub returned_count: usize,
    pub timeout_ms: u64,
    pub elapsed_ms: u64,
    pub timed_out: bool,
    pub fallback_nonblocking: bool,
    pub remote_embedding_invoked: bool,
    pub external_network_read: bool,
    pub hits: Vec<HybridRecallHit>,
    pub checks: HybridRecallChecks,
    pub next_phase: &'static str,
}

pub fn memory_hybrid_recall_sample_report(sample_run: bool) -> HybridRecallReport {
    let query = "Hepta memory transcript source truth hybrid recall".to_string();
    let mut atoms = memory_atom_pipeline_sample_report(true).atoms;
    add_sample_graph_links(&mut atoms);
    let hits = hybrid_recall(&query, &atoms, 4, 250, 17);
    let checks = HybridRecallChecks {
        bm25_signal_ready: hits.iter().any(|hit| hit.scores.bm25 > 0.0),
        embedding_slot_ready: hits.iter().any(|hit| hit.scores.embedding_slot > 0.0),
        entity_signal_ready: hits.iter().any(|hit| hit.scores.entity > 0.0),
        graph_signal_ready: hits.iter().any(|hit| hit.scores.graph > 0.0),
        recency_signal_ready: hits.iter().any(|hit| hit.scores.recency > 0.0),
        rrf_fusion_ready: hits.iter().all(|hit| hit.scores.rrf > 0.0),
        provenance_complete: hits.iter().all(|hit| {
            !hit.source_spans.is_empty()
                && hit.source_spans.iter().all(MemorySourceSpan::is_traceable)
        }),
        timeout_nonblocking: true,
        no_remote_embedding_invoked: true,
        no_external_side_effects: true,
    };
    let p2_ready = checks.ready() && !hits.is_empty();

    HybridRecallReport {
        product: "Hepta",
        command: "memory-hybrid-recall",
        contract: MEMORY_HYBRID_RECALL_V1_CONTRACT,
        status: if p2_ready { "ready" } else { "attention" },
        p2_ready,
        native_rewrite: true,
        sample_run,
        query,
        candidate_count: atoms.len(),
        returned_count: hits.len(),
        timeout_ms: 250,
        elapsed_ms: 17,
        timed_out: false,
        fallback_nonblocking: true,
        remote_embedding_invoked: false,
        external_network_read: false,
        hits,
        checks,
        next_phase: "P3 temporal graph with current/past/superseded fact handling",
    }
}

pub fn hybrid_recall(
    query: &str,
    atoms: &[MemoryUnit],
    limit: usize,
    timeout_ms: u64,
    elapsed_ms: u64,
) -> Vec<HybridRecallHit> {
    if elapsed_ms > timeout_ms {
        return Vec::new();
    }

    let query_terms = terms(query);
    let mut scored = atoms
        .iter()
        .map(|atom| {
            let bm25 = lexical_score(&query_terms, &terms(&atom.content));
            let embedding_slot = deterministic_embedding_slot_score(&query_terms, atom);
            let entity = entity_score(&query_terms, atom);
            let graph = graph_score(atom);
            let recency = recency_score(atom.updated_at_unix_ms, atoms);
            (atom, bm25, embedding_slot, entity, graph, recency)
        })
        .collect::<Vec<_>>();

    let bm25_ranks = ranks(scored.iter().map(|(_, bm25, _, _, _, _)| *bm25).collect());
    let embedding_ranks = ranks(
        scored
            .iter()
            .map(|(_, _, embedding, _, _, _)| *embedding)
            .collect(),
    );
    let entity_ranks = ranks(
        scored
            .iter()
            .map(|(_, _, _, entity, _, _)| *entity)
            .collect(),
    );
    let graph_ranks = ranks(scored.iter().map(|(_, _, _, _, graph, _)| *graph).collect());
    let recency_ranks = ranks(
        scored
            .iter()
            .map(|(_, _, _, _, _, recency)| *recency)
            .collect(),
    );

    let mut hits = scored
        .drain(..)
        .enumerate()
        .map(
            |(idx, (atom, bm25, embedding_slot, entity, graph, recency))| {
                let rrf = rrf_score([
                    bm25_ranks[idx],
                    embedding_ranks[idx],
                    entity_ranks[idx],
                    graph_ranks[idx],
                    recency_ranks[idx],
                ]);
                let final_score = bm25 * 0.30
                    + embedding_slot * 0.25
                    + entity * 0.20
                    + graph * 0.15
                    + recency * 0.10
                    + rrf;
                HybridRecallHit {
                    unit_id: atom.id.clone(),
                    kind: atom.kind,
                    summary: atom.content.clone(),
                    scores: HybridRecallScores {
                        bm25,
                        embedding_slot,
                        entity,
                        graph,
                        recency,
                        rrf,
                        final_score,
                    },
                    source_spans: atom.source_spans.clone(),
                }
            },
        )
        .collect::<Vec<_>>();

    hits.sort_by(|left, right| {
        right
            .scores
            .final_score
            .partial_cmp(&left.scores.final_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    hits.truncate(limit);
    hits
}

fn add_sample_graph_links(atoms: &mut [MemoryUnit]) {
    let task_id = atoms
        .iter()
        .find(|atom| atom.kind == MemoryUnitKind::TaskFact)
        .map(|atom| atom.id.clone());
    if let Some(task_id) = task_id {
        if let Some(preference) = atoms
            .iter_mut()
            .find(|atom| atom.kind == MemoryUnitKind::Preference)
        {
            preference.links.push(hepta_core::MemoryLink {
                target_id: task_id,
                kind: MemoryLinkKind::WorkflowAdjacency,
                weight_ppm: 700_000,
                reason: "preference drives atom-pipeline implementation priority".into(),
            });
        }
    }
}

fn terms(text: &str) -> BTreeSet<String> {
    text.split(|ch: char| !ch.is_alphanumeric())
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| part.to_ascii_lowercase())
        .collect()
}

fn lexical_score(query_terms: &BTreeSet<String>, doc_terms: &BTreeSet<String>) -> f32 {
    if query_terms.is_empty() {
        return 0.0;
    }
    let overlap = query_terms
        .iter()
        .filter(|term| doc_terms.contains(*term))
        .count();
    overlap as f32 / query_terms.len() as f32
}

fn deterministic_embedding_slot_score(query_terms: &BTreeSet<String>, atom: &MemoryUnit) -> f32 {
    let doc_terms = terms(&atom.content);
    let lexical = lexical_score(query_terms, &doc_terms);
    let kind_boost = match atom.kind {
        MemoryUnitKind::Preference | MemoryUnitKind::Decision => 0.20,
        MemoryUnitKind::TaskFact | MemoryUnitKind::EntityFact => 0.15,
        _ => 0.05,
    };
    (lexical + kind_boost).min(1.0)
}

fn entity_score(query_terms: &BTreeSet<String>, atom: &MemoryUnit) -> f32 {
    if atom.entity_ids.iter().any(|entity| {
        entity
            .strip_prefix("entity:")
            .map(|name| query_terms.contains(name))
            .unwrap_or(false)
    }) {
        1.0
    } else {
        0.0
    }
}

fn graph_score(atom: &MemoryUnit) -> f32 {
    atom.links
        .iter()
        .map(|link| link.weight_ppm as f32 / 1_000_000.0)
        .fold(0.0, f32::max)
}

fn recency_score(timestamp: u64, atoms: &[MemoryUnit]) -> f32 {
    let min = atoms
        .iter()
        .map(|atom| atom.updated_at_unix_ms)
        .min()
        .unwrap_or(timestamp);
    let max = atoms
        .iter()
        .map(|atom| atom.updated_at_unix_ms)
        .max()
        .unwrap_or(timestamp);
    if max == min {
        1.0
    } else {
        (timestamp.saturating_sub(min) as f32 / max.saturating_sub(min) as f32).clamp(0.0, 1.0)
    }
}

fn ranks(scores: Vec<f32>) -> Vec<usize> {
    let mut indexed = scores.into_iter().enumerate().collect::<Vec<_>>();
    indexed.sort_by(|left, right| {
        right
            .1
            .partial_cmp(&left.1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut ranks = vec![indexed.len(); indexed.len()];
    for (rank, (idx, _)) in indexed.into_iter().enumerate() {
        ranks[idx] = rank + 1;
    }
    ranks
}

fn rrf_score(ranks: [usize; 5]) -> f32 {
    ranks
        .into_iter()
        .map(|rank| 1.0 / (RRF_K + rank as f32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hybrid_recall_sample_gate_is_ready() {
        let report = memory_hybrid_recall_sample_report(true);

        assert_eq!(report.status, "ready");
        assert!(report.p2_ready);
        assert!(report.checks.ready());
        assert!(!report.remote_embedding_invoked);
        assert!(!report.external_network_read);
    }

    #[test]
    fn hybrid_recall_preserves_provenance_and_ranks_results() {
        let report = memory_hybrid_recall_sample_report(true);

        assert!(!report.hits.is_empty());
        assert!(
            report
                .hits
                .iter()
                .all(|hit| hit.source_spans.iter().all(MemorySourceSpan::is_traceable))
        );
        assert!(
            report
                .hits
                .windows(2)
                .all(|pair| pair[0].scores.final_score >= pair[1].scores.final_score)
        );
    }

    #[test]
    fn hybrid_recall_timeout_returns_nonblocking_empty_result() {
        let atoms = memory_atom_pipeline_sample_report(true).atoms;
        let hits = hybrid_recall("Hepta memory", &atoms, 4, 1, 2);

        assert!(hits.is_empty());
    }
}
