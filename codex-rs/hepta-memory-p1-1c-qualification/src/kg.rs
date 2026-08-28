use crate::{ContractError, Digest32, PPM_DENOMINATOR, checked_ppm, validate_id};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_KG_HOPS: u8 = 2;
pub const MAX_GRAPH_NODES: usize = 256;
pub const MAX_GRAPH_EDGES: usize = 1_024;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KgEdge {
    pub source: String,
    pub target: String,
    pub relation: String,
    pub truth_ppm: u32,
    pub contradiction_ppm: u32,
}

impl KgEdge {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.source, "KG edge source")?;
        validate_id(&self.target, "KG edge target")?;
        validate_id(&self.relation, "KG edge relation")?;
        if self.source == self.target {
            return Err(ContractError::Invalid(
                "KG self-edges are not permitted in the P1.1c evidence graph".to_string(),
            ));
        }
        checked_ppm(self.truth_ppm, "KG edge truth")?;
        checked_ppm(self.contradiction_ppm, "KG edge contradiction")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KgEvidence {
    pub found: bool,
    pub hops: u8,
    pub truth_ppm: u32,
    pub contradiction_ppm: u32,
    pub net_support_ppm: u32,
    pub path_sha256: Digest32,
    pub visited_nodes: u32,
    pub scanned_edges: u32,
}

impl KgEvidence {
    fn not_found(visited_nodes: usize, scanned_edges: usize) -> Result<Self, ContractError> {
        Ok(Self {
            found: false,
            hops: 0,
            truth_ppm: 0,
            contradiction_ppm: 0,
            net_support_ppm: 0,
            path_sha256: Digest32::for_bytes(b"kg-evidence-not-found"),
            visited_nodes: u32::try_from(visited_nodes).map_err(|_| ContractError::Overflow)?,
            scanned_edges: u32::try_from(scanned_edges).map_err(|_| ContractError::Overflow)?,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KgGraph {
    adjacency: BTreeMap<String, Vec<KgEdge>>,
    edge_count: usize,
}

impl KgGraph {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_edge(&mut self, edge: KgEdge) -> Result<(), ContractError> {
        edge.validate()?;
        if self.edge_count >= MAX_GRAPH_EDGES {
            return Err(ContractError::Limit(format!(
                "KG graph exceeds {MAX_GRAPH_EDGES} edges"
            )));
        }
        let mut nodes: BTreeSet<&str> = self.adjacency.keys().map(String::as_str).collect();
        for edges in self.adjacency.values() {
            for current in edges {
                nodes.insert(current.target.as_str());
            }
        }
        nodes.insert(edge.source.as_str());
        nodes.insert(edge.target.as_str());
        if nodes.len() > MAX_GRAPH_NODES {
            return Err(ContractError::Limit(format!(
                "KG graph exceeds {MAX_GRAPH_NODES} nodes"
            )));
        }

        let outgoing = self.adjacency.entry(edge.source.clone()).or_default();
        if outgoing.iter().any(|current| current == &edge) {
            return Err(ContractError::Invalid(
                "duplicate KG edge is not permitted".to_string(),
            ));
        }
        outgoing.push(edge);
        outgoing.sort();
        self.edge_count = self
            .edge_count
            .checked_add(1)
            .ok_or(ContractError::Overflow)?;
        Ok(())
    }

    pub fn bounded_two_hop(
        &self,
        start: &str,
        goal: &str,
    ) -> Result<KgEvidence, ContractError> {
        validate_id(start, "KG search start")?;
        validate_id(goal, "KG search goal")?;
        if start == goal {
            return Err(ContractError::Invalid(
                "KG evidence search requires distinct endpoints".to_string(),
            ));
        }

        let mut visited = BTreeSet::new();
        visited.insert(start);
        let mut scanned_edges = 0_usize;
        let mut candidates = Vec::new();

        if let Some(first_edges) = self.adjacency.get(start) {
            for first in first_edges {
                scanned_edges = scanned_edges
                    .checked_add(1)
                    .ok_or(ContractError::Overflow)?;
                if scanned_edges > MAX_GRAPH_EDGES {
                    return Err(ContractError::Limit(
                        "bounded KG search exceeded its edge scan budget".to_string(),
                    ));
                }
                visited.insert(first.target.as_str());
                if first.target == goal {
                    candidates.push(PathCandidate::from_edges(&[first])?);
                    continue;
                }
                if let Some(second_edges) = self.adjacency.get(&first.target) {
                    for second in second_edges {
                        scanned_edges = scanned_edges
                            .checked_add(1)
                            .ok_or(ContractError::Overflow)?;
                        if scanned_edges > MAX_GRAPH_EDGES {
                            return Err(ContractError::Limit(
                                "bounded KG search exceeded its edge scan budget".to_string(),
                            ));
                        }
                        visited.insert(second.target.as_str());
                        if second.target == goal && second.target != first.source {
                            candidates.push(PathCandidate::from_edges(&[first, second])?);
                        }
                    }
                }
            }
        }
        if visited.len() > MAX_GRAPH_NODES {
            return Err(ContractError::Limit(
                "bounded KG search exceeded its node visit budget".to_string(),
            ));
        }
        if candidates.is_empty() {
            return KgEvidence::not_found(visited.len(), scanned_edges);
        }
        candidates.sort_by(|left, right| {
            right
                .net_support_ppm
                .cmp(&left.net_support_ppm)
                .then_with(|| right.truth_ppm.cmp(&left.truth_ppm))
                .then_with(|| left.contradiction_ppm.cmp(&right.contradiction_ppm))
                .then_with(|| left.hops.cmp(&right.hops))
                .then_with(|| left.path_sha256.cmp(&right.path_sha256))
        });
        let best = candidates.remove(0);
        Ok(KgEvidence {
            found: true,
            hops: best.hops,
            truth_ppm: best.truth_ppm,
            contradiction_ppm: best.contradiction_ppm,
            net_support_ppm: best.net_support_ppm,
            path_sha256: best.path_sha256,
            visited_nodes: u32::try_from(visited.len()).map_err(|_| ContractError::Overflow)?,
            scanned_edges: u32::try_from(scanned_edges).map_err(|_| ContractError::Overflow)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathCandidate {
    hops: u8,
    truth_ppm: u32,
    contradiction_ppm: u32,
    net_support_ppm: u32,
    path_sha256: Digest32,
}

impl PathCandidate {
    fn from_edges(edges: &[&KgEdge]) -> Result<Self, ContractError> {
        if edges.is_empty() || edges.len() > usize::from(MAX_KG_HOPS) {
            return Err(ContractError::Invalid(
                "KG path must contain one or two edges".to_string(),
            ));
        }
        let truth_ppm = edges
            .iter()
            .map(|edge| edge.truth_ppm)
            .min()
            .unwrap_or(0);
        let contradiction_ppm = edges
            .iter()
            .map(|edge| edge.contradiction_ppm)
            .max()
            .unwrap_or(0);
        let net_support_ppm = truth_ppm.saturating_sub(contradiction_ppm);
        let mut canonical = String::new();
        for edge in edges {
            use std::fmt::Write as _;
            write!(
                &mut canonical,
                "{}>{}:{}:{}:{};",
                edge.source,
                edge.target,
                edge.relation,
                edge.truth_ppm,
                edge.contradiction_ppm
            )
            .expect("writing to String cannot fail");
        }
        Ok(Self {
            hops: u8::try_from(edges.len()).map_err(|_| ContractError::Overflow)?,
            truth_ppm,
            contradiction_ppm,
            net_support_ppm: net_support_ppm.min(PPM_DENOMINATOR),
            path_sha256: Digest32::for_bytes(canonical.as_bytes()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{KgEdge, KgGraph};

    #[test]
    fn two_hop_path_is_deterministic_and_bounded() {
        let mut graph = KgGraph::new();
        graph
            .add_edge(KgEdge {
                source: "start".to_string(),
                target: "middle".to_string(),
                relation: "supports".to_string(),
                truth_ppm: 900_000,
                contradiction_ppm: 10_000,
            })
            .expect("edge");
        graph
            .add_edge(KgEdge {
                source: "middle".to_string(),
                target: "goal".to_string(),
                relation: "supports".to_string(),
                truth_ppm: 800_000,
                contradiction_ppm: 20_000,
            })
            .expect("edge");
        let evidence = graph.bounded_two_hop("start", "goal").expect("evidence");
        assert!(evidence.found);
        assert_eq!(evidence.hops, 2);
        assert_eq!(evidence.truth_ppm, 800_000);
        assert_eq!(evidence.contradiction_ppm, 20_000);
        assert_eq!(evidence.net_support_ppm, 780_000);
    }
}
