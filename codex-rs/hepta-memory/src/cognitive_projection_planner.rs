use std::collections::BTreeMap;
use std::collections::BTreeSet;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::Sha256Digest;
use sha2::Digest;
use sha2::Sha256;

use crate::CognitiveScope;
use crate::CognitiveStoreError;
use crate::cognitive_intelligence_writer::canonical_entity_id;
use crate::cognitive_intelligence_writer::canonical_relation_id;
use crate::cognitive_intelligence_writer::occurrence_edge_id;
use crate::cognitive_intelligence_writer::occurrence_node_id;
use crate::cognitive_kg_store::ProjectionEdge;
use crate::cognitive_kg_store::ProjectionHead;
use crate::cognitive_kg_store::ProjectionNode;
use crate::cognitive_kg_store::input_heads_digest;
use crate::cognitive_kg_store::output_digest;

use super::frame_part;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProjectionEligibilityPolicy {
    CurrentActiveVerified,
    GroundedActiveVerified,
}

impl ProjectionEligibilityPolicy {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentActiveVerified => "current_active_verified_v1",
            Self::GroundedActiveVerified => "grounded_active_verified_v1",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProjectionHeadDisposition {
    Included,
    ZeroFact,
    IneligibleHead,
    LegacyUnreviewed,
}

impl ProjectionHeadDisposition {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Included => "included",
            Self::ZeroFact => "zero_fact",
            Self::IneligibleHead => "ineligible_head",
            Self::LegacyUnreviewed => "legacy_unreviewed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProjectionHeadDecision {
    pub(crate) head: ProjectionHead,
    pub(crate) disposition: ProjectionHeadDisposition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProjectionSemanticPlan {
    pub(crate) projection_scope: String,
    pub(crate) policy: ProjectionEligibilityPolicy,
    pub(crate) head_decisions: Vec<ProjectionHeadDecision>,
    pub(crate) nodes: Vec<ProjectionNode>,
    pub(crate) edges: Vec<ProjectionEdge>,
    pub(crate) input_heads_sha256: Sha256Digest,
    pub(crate) eligibility_sha256: Sha256Digest,
    pub(crate) output_sha256: Sha256Digest,
}

impl ProjectionSemanticPlan {
    pub(crate) fn build(
        owner_agent_id: &AgentId,
        scope: &CognitiveScope,
        policy: ProjectionEligibilityPolicy,
        mut heads: Vec<ProjectionHead>,
        nodes: Vec<ProjectionNode>,
        edges: Vec<ProjectionEdge>,
    ) -> Result<Self, CognitiveStoreError> {
        let projection_scope = scope.projection_key();
        heads.sort_by(|left, right| {
            (left.memory_id.as_str(), left.revision)
                .cmp(&(right.memory_id.as_str(), right.revision))
        });
        let mut seen_heads = BTreeSet::new();
        let mut included = BTreeSet::new();
        let mut decisions = Vec::with_capacity(heads.len());
        for head in &heads {
            let identity = (head.memory_id.clone(), head.revision);
            if !seen_heads.insert(identity.clone()) {
                return Err(CognitiveStoreError::Corrupt(
                    "semantic projection planner received a duplicate memory head".to_string(),
                ));
            }
            if head.entity_count < 0 || head.relation_count < 0 {
                return Err(CognitiveStoreError::Corrupt(
                    "semantic projection planner received a negative fact count".to_string(),
                ));
            }
            let fact_count = head
                .entity_count
                .checked_add(head.relation_count)
                .ok_or_else(|| {
                    CognitiveStoreError::Corrupt(
                        "semantic projection fact count overflow".to_string(),
                    )
                })?;
            let disposition = if fact_count == 0 {
                ProjectionHeadDisposition::ZeroFact
            } else if head.verification != "verified" || head.lifecycle != "active" {
                ProjectionHeadDisposition::IneligibleHead
            } else if policy == ProjectionEligibilityPolicy::GroundedActiveVerified
                && head.grounding_receipt_sha256.is_none()
            {
                ProjectionHeadDisposition::LegacyUnreviewed
            } else {
                included.insert(identity);
                ProjectionHeadDisposition::Included
            };
            decisions.push(ProjectionHeadDecision {
                head: head.clone(),
                disposition,
            });
        }

        let mut filtered_nodes = nodes
            .into_iter()
            .filter(|node| included.contains(&(node.memory_id.clone(), node.memory_revision)))
            .collect::<Vec<_>>();
        filtered_nodes.sort_by(|left, right| {
            (
                left.memory_id.as_str(),
                left.memory_revision,
                left.entity_key.as_str(),
            )
                .cmp(&(
                    right.memory_id.as_str(),
                    right.memory_revision,
                    right.entity_key.as_str(),
                ))
        });
        let mut canonical_shapes = BTreeMap::<String, (String, String)>::new();
        let mut node_ids = BTreeSet::new();
        let mut actual_entities = BTreeMap::<(String, i64), i64>::new();
        for node in &filtered_nodes {
            let expected_canonical = canonical_entity_id(owner_agent_id, scope, &node.entity_key);
            if node.canonical_entity_id != expected_canonical {
                return Err(CognitiveStoreError::Corrupt(
                    "semantic projection entity identity does not match its scoped key".to_string(),
                ));
            }
            let expected_node =
                occurrence_node_id(&node.memory_id, node.memory_revision, &node.entity_key);
            if node.node_id != expected_node || !node_ids.insert(node.node_id.clone()) {
                return Err(CognitiveStoreError::Corrupt(
                    "semantic projection occurrence node identity is invalid or duplicated"
                        .to_string(),
                ));
            }
            if let Some(shape) = canonical_shapes.get(&node.canonical_entity_id) {
                if shape != &(node.entity_type.clone(), node.label.clone()) {
                    return Err(CognitiveStoreError::Conflict(format!(
                        "active KG supports disagree on type or label for {}",
                        node.canonical_entity_id
                    )));
                }
            } else {
                canonical_shapes.insert(
                    node.canonical_entity_id.clone(),
                    (node.entity_type.clone(), node.label.clone()),
                );
            }
            let count = actual_entities
                .entry((node.memory_id.clone(), node.memory_revision))
                .or_default();
            *count = count.checked_add(1).ok_or_else(|| {
                CognitiveStoreError::Corrupt(
                    "semantic projection entity count overflow".to_string(),
                )
            })?;
        }

        let mut filtered_edges = edges
            .into_iter()
            .filter(|edge| included.contains(&(edge.memory_id.clone(), edge.memory_revision)))
            .collect::<Vec<_>>();
        filtered_edges.sort_by(|left, right| {
            (
                left.memory_id.as_str(),
                left.memory_revision,
                left.relation_key.as_str(),
            )
                .cmp(&(
                    right.memory_id.as_str(),
                    right.memory_revision,
                    right.relation_key.as_str(),
                ))
        });
        let mut edge_ids = BTreeSet::new();
        let mut actual_relations = BTreeMap::<(String, i64), i64>::new();
        for edge in &filtered_edges {
            let from_canonical = canonical_entity_id(owner_agent_id, scope, &edge.from_entity_key);
            let to_canonical = canonical_entity_id(owner_agent_id, scope, &edge.to_entity_key);
            let expected_canonical = canonical_relation_id(
                owner_agent_id,
                scope,
                &from_canonical,
                &edge.relation,
                &to_canonical,
            );
            if edge.canonical_relation_id != expected_canonical {
                return Err(CognitiveStoreError::Corrupt(
                    "semantic projection relation identity does not match its endpoints"
                        .to_string(),
                ));
            }
            let expected_edge =
                occurrence_edge_id(&edge.memory_id, edge.memory_revision, &edge.relation_key);
            if edge.edge_id != expected_edge || !edge_ids.insert(edge.edge_id.clone()) {
                return Err(CognitiveStoreError::Corrupt(
                    "semantic projection occurrence edge identity is invalid or duplicated"
                        .to_string(),
                ));
            }
            let expected_from =
                occurrence_node_id(&edge.memory_id, edge.memory_revision, &edge.from_entity_key);
            let expected_to =
                occurrence_node_id(&edge.memory_id, edge.memory_revision, &edge.to_entity_key);
            if edge.from_node_id != expected_from
                || edge.to_node_id != expected_to
                || !node_ids.contains(&edge.from_node_id)
                || !node_ids.contains(&edge.to_node_id)
            {
                return Err(CognitiveStoreError::Corrupt(
                    "semantic projection relation has an invalid occurrence endpoint".to_string(),
                ));
            }
            let count = actual_relations
                .entry((edge.memory_id.clone(), edge.memory_revision))
                .or_default();
            *count = count.checked_add(1).ok_or_else(|| {
                CognitiveStoreError::Corrupt(
                    "semantic projection relation count overflow".to_string(),
                )
            })?;
        }

        for decision in &decisions {
            if decision.disposition != ProjectionHeadDisposition::Included {
                continue;
            }
            let identity = (decision.head.memory_id.clone(), decision.head.revision);
            let entity_count = actual_entities.get(&identity).copied().unwrap_or_default();
            let relation_count = actual_relations.get(&identity).copied().unwrap_or_default();
            if entity_count != decision.head.entity_count
                || relation_count != decision.head.relation_count
            {
                return Err(CognitiveStoreError::Corrupt(
                    "semantic projection planner observed an incomplete immutable fact set"
                        .to_string(),
                ));
            }
        }

        let input_heads_sha256 = input_heads_digest(&projection_scope, &heads);
        let eligibility_sha256 = eligibility_digest(&projection_scope, policy, &decisions);
        let output_sha256 = output_digest(&projection_scope, &filtered_nodes, &filtered_edges);
        Ok(Self {
            projection_scope,
            policy,
            head_decisions: decisions,
            nodes: filtered_nodes,
            edges: filtered_edges,
            input_heads_sha256,
            eligibility_sha256,
            output_sha256,
        })
    }
}

fn eligibility_digest(
    projection_scope: &str,
    policy: ProjectionEligibilityPolicy,
    decisions: &[ProjectionHeadDecision],
) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, b"hepta:cognitive:kg-projection-eligibility:v1");
    frame_part(&mut hasher, projection_scope.as_bytes());
    frame_part(&mut hasher, policy.as_str().as_bytes());
    frame_part(
        &mut hasher,
        &u64::try_from(decisions.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for decision in decisions {
        let head = &decision.head;
        frame_part(&mut hasher, head.memory_id.as_bytes());
        frame_part(&mut hasher, &head.revision.to_be_bytes());
        frame_part(&mut hasher, head.content_sha256.as_bytes());
        frame_part(&mut hasher, head.verification.as_bytes());
        frame_part(&mut hasher, head.lifecycle.as_bytes());
        frame_part(&mut hasher, head.fact_set_sha256.as_bytes());
        frame_part(&mut hasher, &head.entity_count.to_be_bytes());
        frame_part(&mut hasher, &head.relation_count.to_be_bytes());
        frame_part(
            &mut hasher,
            head.grounding_receipt_sha256
                .as_deref()
                .unwrap_or("<none>")
                .as_bytes(),
        );
        frame_part(&mut hasher, decision.disposition.as_str().as_bytes());
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognitive_test_support::agent_id;

    fn head(label: &str, receipt: Option<&str>) -> ProjectionHead {
        ProjectionHead {
            memory_id: label.to_string(),
            revision: 1,
            content_sha256: "1111111111111111111111111111111111111111111111111111111111111111"
                .to_string(),
            verification: "verified".to_string(),
            lifecycle: "active".to_string(),
            fact_set_sha256: "2222222222222222222222222222222222222222222222222222222222222222"
                .to_string(),
            entity_count: 0,
            relation_count: 0,
            grounding_receipt_sha256: receipt.map(str::to_string),
        }
    }

    #[test]
    fn grounded_policy_never_promotes_a_legacy_nonempty_head() {
        let owner = agent_id(249);
        let scope = CognitiveScope::AgentPrivate;
        let mut legacy = head("legacy", None);
        legacy.entity_count = 1;
        let plan = ProjectionSemanticPlan::build(
            &owner,
            &scope,
            ProjectionEligibilityPolicy::GroundedActiveVerified,
            vec![legacy],
            Vec::new(),
            Vec::new(),
        )
        .expect("plan");
        assert_eq!(
            plan.head_decisions[0].disposition,
            ProjectionHeadDisposition::LegacyUnreviewed
        );
        assert!(plan.nodes.is_empty());
    }
}
