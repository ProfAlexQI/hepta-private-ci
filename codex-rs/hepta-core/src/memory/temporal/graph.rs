use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_TEMPORAL_FACT_GRAPH_SCHEMA_VERSION;
use super::super::privacy_class_is_payload_light;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use super::fact::ContextMemoryTemporalFact;
use super::fact::ContextMemoryTemporalFactReport;
use super::fact::ContextMemoryTemporalFactType;

/// Temporal graph edge kind for dry-run temporal fact topology.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryTemporalFactGraphEdgeKind {
    Provenance,
    ValidityWindow,
    Supersedes,
    #[default]
    Unknown,
}

impl ContextMemoryTemporalFactGraphEdgeKind {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light node for a future temporal fact graph.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalFactGraphNode {
    pub fact_hash: String,
    pub fact_type: ContextMemoryTemporalFactType,
    pub provenance_span_count: usize,
    pub valid_from_sequence: u64,
    pub invalid_at_sequence: Option<u64>,
    pub confidence_basis_points: u32,
    pub has_supersedes: bool,
    pub privacy_class: String,
    pub dry_run_only: bool,
    pub production_write: bool,
    pub graph_write: bool,
}

impl ContextMemoryTemporalFactGraphNode {
    fn from_fact(fact: &ContextMemoryTemporalFact) -> Self {
        Self {
            fact_hash: temporal_fact_graph_fact_hash(fact),
            fact_type: fact.fact_type,
            provenance_span_count: fact.provenance_span_count,
            valid_from_sequence: fact.valid_from_sequence,
            invalid_at_sequence: fact.invalid_at_sequence,
            confidence_basis_points: fact.confidence_basis_points,
            has_supersedes: fact.supersedes_fact_hash.is_some(),
            privacy_class: fact.privacy_class.clone(),
            dry_run_only: true,
            production_write: false,
            graph_write: false,
        }
    }

    pub fn has_node_integrity(&self) -> bool {
        stable_receipt_hash_is_valid(&self.fact_hash)
            && !self.fact_type.is_unknown()
            && self.provenance_span_count > 0
            && self.valid_from_sequence > 0
            && self
                .invalid_at_sequence
                .is_none_or(|sequence| sequence > self.valid_from_sequence)
            && self.confidence_basis_points <= 10_000
            && privacy_class_is_payload_light(&self.privacy_class)
            && self.dry_run_only
            && !self.production_write
            && !self.graph_write
    }
}

/// Payload-light edge for a future temporal fact graph.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalFactGraphEdge {
    pub edge_hash: String,
    pub edge_kind: ContextMemoryTemporalFactGraphEdgeKind,
    pub from_fact_hash: String,
    pub to_fact_hash: Option<String>,
    pub provenance_span_count: usize,
    pub valid_from_sequence: u64,
    pub invalid_at_sequence: Option<u64>,
    pub dry_run_only: bool,
    pub production_write: bool,
    pub graph_write: bool,
}

impl ContextMemoryTemporalFactGraphEdge {
    fn provenance(fact_hash: &str, fact: &ContextMemoryTemporalFact) -> Self {
        Self {
            edge_hash: stable_receipt_hash(&[
                "memory_temporal_fact_graph_edge",
                "provenance",
                fact_hash,
                &fact.provenance_span_count.to_string(),
            ]),
            edge_kind: ContextMemoryTemporalFactGraphEdgeKind::Provenance,
            from_fact_hash: fact_hash.to_string(),
            to_fact_hash: None,
            provenance_span_count: fact.provenance_span_count,
            valid_from_sequence: fact.valid_from_sequence,
            invalid_at_sequence: fact.invalid_at_sequence,
            dry_run_only: true,
            production_write: false,
            graph_write: false,
        }
    }

    fn validity_window(fact_hash: &str, fact: &ContextMemoryTemporalFact) -> Self {
        let invalid_at_sequence = fact
            .invalid_at_sequence
            .map(|sequence| sequence.to_string())
            .unwrap_or_else(|| "open".to_string());
        Self {
            edge_hash: stable_receipt_hash(&[
                "memory_temporal_fact_graph_edge",
                "validity_window",
                fact_hash,
                &fact.valid_from_sequence.to_string(),
                &invalid_at_sequence,
            ]),
            edge_kind: ContextMemoryTemporalFactGraphEdgeKind::ValidityWindow,
            from_fact_hash: fact_hash.to_string(),
            to_fact_hash: None,
            provenance_span_count: fact.provenance_span_count,
            valid_from_sequence: fact.valid_from_sequence,
            invalid_at_sequence: fact.invalid_at_sequence,
            dry_run_only: true,
            production_write: false,
            graph_write: false,
        }
    }

    fn supersedes(
        fact_hash: &str,
        supersedes_fact_hash: &str,
        fact: &ContextMemoryTemporalFact,
    ) -> Self {
        Self {
            edge_hash: stable_receipt_hash(&[
                "memory_temporal_fact_graph_edge",
                "supersedes",
                fact_hash,
                supersedes_fact_hash,
            ]),
            edge_kind: ContextMemoryTemporalFactGraphEdgeKind::Supersedes,
            from_fact_hash: fact_hash.to_string(),
            to_fact_hash: Some(supersedes_fact_hash.to_string()),
            provenance_span_count: fact.provenance_span_count,
            valid_from_sequence: fact.valid_from_sequence,
            invalid_at_sequence: fact.invalid_at_sequence,
            dry_run_only: true,
            production_write: false,
            graph_write: false,
        }
    }

    pub fn has_edge_integrity(&self) -> bool {
        stable_receipt_hash_is_valid(&self.edge_hash)
            && !self.edge_kind.is_unknown()
            && stable_receipt_hash_is_valid(&self.from_fact_hash)
            && match self.edge_kind {
                ContextMemoryTemporalFactGraphEdgeKind::Supersedes => self
                    .to_fact_hash
                    .as_deref()
                    .is_some_and(stable_receipt_hash_is_valid),
                ContextMemoryTemporalFactGraphEdgeKind::Provenance
                | ContextMemoryTemporalFactGraphEdgeKind::ValidityWindow => {
                    self.to_fact_hash.is_none()
                }
                ContextMemoryTemporalFactGraphEdgeKind::Unknown => false,
            }
            && self.provenance_span_count > 0
            && self.valid_from_sequence > 0
            && self
                .invalid_at_sequence
                .is_none_or(|sequence| sequence > self.valid_from_sequence)
            && self.dry_run_only
            && !self.production_write
            && !self.graph_write
    }
}

/// Dry-run temporal fact graph topology for future temporal memory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalFactGraphReport {
    pub schema_version: u32,
    pub nodes: Vec<ContextMemoryTemporalFactGraphNode>,
    pub edges: Vec<ContextMemoryTemporalFactGraphEdge>,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
}

impl Default for ContextMemoryTemporalFactGraphReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_TEMPORAL_FACT_GRAPH_SCHEMA_VERSION,
            nodes: Vec::new(),
            edges: Vec::new(),
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
        }
    }
}

impl ContextMemoryTemporalFactGraphReport {
    pub fn from_temporal_facts(facts: &ContextMemoryTemporalFactReport) -> Self {
        let mut nodes = Vec::with_capacity(facts.facts.len());
        let mut edges = Vec::with_capacity(facts.facts.len().saturating_mul(3));

        for fact in &facts.facts {
            let node = ContextMemoryTemporalFactGraphNode::from_fact(fact);
            edges.push(ContextMemoryTemporalFactGraphEdge::provenance(
                &node.fact_hash,
                fact,
            ));
            edges.push(ContextMemoryTemporalFactGraphEdge::validity_window(
                &node.fact_hash,
                fact,
            ));
            if let Some(supersedes_fact_hash) = fact.supersedes_fact_hash.as_deref() {
                edges.push(ContextMemoryTemporalFactGraphEdge::supersedes(
                    &node.fact_hash,
                    supersedes_fact_hash,
                    fact,
                ));
            }
            nodes.push(node);
        }

        Self {
            nodes,
            edges,
            ..Self::default()
        }
    }

    pub fn has_graph_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_TEMPORAL_FACT_GRAPH_SCHEMA_VERSION
            && self
                .nodes
                .iter()
                .all(ContextMemoryTemporalFactGraphNode::has_node_integrity)
            && self
                .edges
                .iter()
                .all(ContextMemoryTemporalFactGraphEdge::has_edge_integrity)
            && self.edges.iter().all(|edge| {
                self.nodes
                    .iter()
                    .any(|node| node.fact_hash == edge.from_fact_hash)
            })
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
    }

    pub fn provenance_edge_count(&self) -> usize {
        self.edges
            .iter()
            .filter(|edge| edge.edge_kind == ContextMemoryTemporalFactGraphEdgeKind::Provenance)
            .count()
    }

    pub fn validity_window_edge_count(&self) -> usize {
        self.edges
            .iter()
            .filter(|edge| edge.edge_kind == ContextMemoryTemporalFactGraphEdgeKind::ValidityWindow)
            .count()
    }

    pub fn supersedes_edge_count(&self) -> usize {
        self.edges
            .iter()
            .filter(|edge| edge.edge_kind == ContextMemoryTemporalFactGraphEdgeKind::Supersedes)
            .count()
    }

    pub fn open_node_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|node| node.invalid_at_sequence.is_none())
            .count()
    }

    pub fn invalidated_node_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|node| node.invalid_at_sequence.is_some())
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }
}

fn temporal_fact_graph_fact_hash(fact: &ContextMemoryTemporalFact) -> String {
    stable_receipt_hash(&[
        "memory_temporal_fact_graph_fact",
        fact.fact_type.as_str(),
        &fact.entity_hash,
        &fact.valid_from_sequence.to_string(),
        &fact.confidence_basis_points.to_string(),
    ])
}
