//! P0.3.1/P0.3.2 read-only grounded semantic projection comparison.
//!
//! The shadow path is explicitly authorized, verifies the durable grounding
//! ledger inside the same SQLite snapshot as every projection/head/span read,
//! replans the current product generation with the shared semantic planner,
//! and then builds the grounded-only candidate. It never migrates schema,
//! publishes a generation, changes recall, or grants production authority.

use codex_hepta_contracts::Sha256Digest;
use serde::Serialize;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;

use crate::CognitiveAccess;
use crate::CognitiveScope;
use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::StableMemoryId;
use crate::cognitive_intelligence_writer::occurrence_edge_id;
use crate::cognitive_intelligence_writer::occurrence_node_id;
use crate::cognitive_kg_store::MAX_SCOPE_EDGES;
use crate::cognitive_kg_store::MAX_SCOPE_HEADS;
use crate::cognitive_kg_store::MAX_SCOPE_NODES;
use crate::cognitive_kg_store::ProjectionEdge;
use crate::cognitive_kg_store::ProjectionHead;
use crate::cognitive_kg_store::ProjectionNode;
use crate::cognitive_store::unavailable;
use crate::framing::cognitive_projection_planner::ProjectionEligibilityPolicy;
use crate::framing::cognitive_projection_planner::ProjectionHeadDisposition;
use crate::framing::cognitive_projection_planner::ProjectionSemanticPlan;

const SHADOW_GATE_SCHEMA_VERSION: u32 = 4;
const SHADOW_GATE_MODE: &str = "grounded_semantic_projection_shadow_compare_v4";

#[derive(Clone, Debug, Serialize)]
struct ShadowProjectionHead {
    memory_id: String,
    revision: u64,
    fact_set_sha256: String,
    grounding_receipt_sha256: String,
    entity_count: u64,
    relation_count: u64,
}

#[derive(Clone, Debug, Serialize)]
struct ExcludedProjectionHead {
    memory_id: String,
    revision: u64,
    reason: &'static str,
    entity_count: u64,
    relation_count: u64,
}

#[derive(Clone, Debug, Serialize)]
struct CurrentProjectionSnapshot {
    generation: u64,
    input_heads_sha256: Option<String>,
    output_sha256: Option<String>,
    node_count: u64,
    edge_count: u64,
}

#[derive(Clone, Debug, Serialize)]
struct GroundedCandidateSnapshot {
    output_sha256: String,
    eligibility_sha256: String,
    node_count: u64,
    edge_count: u64,
    included_heads: Vec<ShadowProjectionHead>,
}

#[derive(Clone, Debug, Serialize)]
struct ShadowProjectionComparison {
    schema_version: u32,
    mode: &'static str,
    candidate_kind: &'static str,
    projection_scope: String,
    current: CurrentProjectionSnapshot,
    grounded_candidate: GroundedCandidateSnapshot,
    excluded_heads: Vec<ExcludedProjectionHead>,
    zero_fact_heads: Vec<ExcludedProjectionHead>,
    node_count_delta: i64,
    edge_count_delta: i64,
    shared_projection_planner: bool,
    current_projection_replanned: bool,
    semantic_projection_parity: bool,
    semantic_projection_parity_qualified: bool,
    read_snapshot_transaction: bool,
    ledger_verified_in_snapshot: bool,
    schema_mutation_performed: bool,
    write_performed: bool,
    default_projection_pointer_changed: bool,
    default_recall_query_changed: bool,
    production_projection_gate: bool,
    production_authority: bool,
    external_effects: bool,
    operator_acceptance: bool,
    promotion: bool,
}

#[derive(Clone, Debug, Serialize)]
struct GroundingEvidenceSpan {
    fact_kind: String,
    fact_key: String,
    evidence_ordinal: u32,
    start_byte: u32,
    end_byte: u32,
    evidence_sha256: String,
}

#[derive(Clone, Debug, Serialize)]
struct ShadowGroundingExplanation {
    schema_version: u32,
    mode: &'static str,
    memory_id: String,
    revision: u64,
    projection_scope: String,
    content_sha256: String,
    verification: String,
    lifecycle: String,
    fact_set_sha256: String,
    fact_identity_sha256: Option<String>,
    grounding_receipt_sha256: Option<String>,
    grounding_status: &'static str,
    entity_count: u64,
    relation_count: u64,
    evidence: Vec<GroundingEvidenceSpan>,
    read_snapshot_transaction: bool,
    ledger_verified_in_snapshot: bool,
    schema_mutation_performed: bool,
    write_performed: bool,
    production_projection_gate: bool,
    production_authority: bool,
    external_effects: bool,
    operator_acceptance: bool,
    promotion: bool,
}

impl CognitiveStore {
    pub async fn shadow_grounded_projection_compare(
        &self,
        access: &CognitiveAccess,
        scope: &CognitiveScope,
    ) -> Result<String, CognitiveStoreError> {
        self.authorize(access, scope)?;
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        self.verify_durable_fact_grounding_ledger_tx(&mut transaction)
            .await?;
        let projection_scope = scope.projection_key();
        let current = read_current_projection(&mut transaction, &projection_scope).await?;
        let heads = read_heads(self, &mut transaction, scope).await?;
        let nodes = read_nodes(self, &mut transaction, scope).await?;
        let edges = read_edges(self, &mut transaction, scope).await?;

        let current_plan = ProjectionSemanticPlan::build(
            &self.owner_agent_id,
            scope,
            ProjectionEligibilityPolicy::CurrentActiveVerified,
            heads.clone(),
            nodes.clone(),
            edges.clone(),
        )?;
        verify_current_projection(&current, &current_plan)?;

        let plan = ProjectionSemanticPlan::build(
            &self.owner_agent_id,
            scope,
            ProjectionEligibilityPolicy::GroundedActiveVerified,
            heads,
            nodes,
            edges,
        )?;

        let mut included = Vec::new();
        let mut excluded = Vec::new();
        let mut zero_fact = Vec::new();
        for decision in &plan.head_decisions {
            let head = &decision.head;
            let memory_revision = positive_u64(head.revision, "memory revision")?;
            let entity_count = nonnegative_u64(head.entity_count, "entity count")?;
            let relation_count = nonnegative_u64(head.relation_count, "relation count")?;
            match decision.disposition {
                ProjectionHeadDisposition::Included => {
                    let receipt = head.grounding_receipt_sha256.clone().ok_or_else(|| {
                        CognitiveStoreError::Corrupt(
                            "grounded semantic plan included a head without a receipt".to_string(),
                        )
                    })?;
                    Sha256Digest::parse(receipt.clone()).map_err(CognitiveStoreError::Corrupt)?;
                    included.push(ShadowProjectionHead {
                        memory_id: head.memory_id.clone(),
                        revision: memory_revision,
                        fact_set_sha256: head.fact_set_sha256.clone(),
                        grounding_receipt_sha256: receipt,
                        entity_count,
                        relation_count,
                    });
                }
                ProjectionHeadDisposition::ZeroFact => zero_fact.push(ExcludedProjectionHead {
                    memory_id: head.memory_id.clone(),
                    revision: memory_revision,
                    reason: decision.disposition.as_str(),
                    entity_count,
                    relation_count,
                }),
                ProjectionHeadDisposition::IneligibleHead
                | ProjectionHeadDisposition::LegacyUnreviewed => {
                    excluded.push(ExcludedProjectionHead {
                        memory_id: head.memory_id.clone(),
                        revision: memory_revision,
                        reason: decision.disposition.as_str(),
                        entity_count,
                        relation_count,
                    });
                }
            }
        }

        let candidate_nodes = u64::try_from(plan.nodes.len()).map_err(|_| {
            CognitiveStoreError::Corrupt("grounded candidate node count exceeds u64".to_string())
        })?;
        let candidate_edges = u64::try_from(plan.edges.len()).map_err(|_| {
            CognitiveStoreError::Corrupt("grounded candidate edge count exceeds u64".to_string())
        })?;
        let comparison = ShadowProjectionComparison {
            schema_version: SHADOW_GATE_SCHEMA_VERSION,
            mode: SHADOW_GATE_MODE,
            candidate_kind: "shared_semantic_projection_plan_v1",
            projection_scope,
            node_count_delta: signed_delta(candidate_nodes, current.node_count)?,
            edge_count_delta: signed_delta(candidate_edges, current.edge_count)?,
            current,
            grounded_candidate: GroundedCandidateSnapshot {
                output_sha256: plan.output_sha256.as_str().to_string(),
                eligibility_sha256: plan.eligibility_sha256.as_str().to_string(),
                node_count: candidate_nodes,
                edge_count: candidate_edges,
                included_heads: included,
            },
            excluded_heads: excluded,
            zero_fact_heads: zero_fact,
            shared_projection_planner: true,
            current_projection_replanned: true,
            semantic_projection_parity: true,
            semantic_projection_parity_qualified: false,
            read_snapshot_transaction: true,
            ledger_verified_in_snapshot: true,
            schema_mutation_performed: false,
            write_performed: false,
            default_projection_pointer_changed: false,
            default_recall_query_changed: false,
            production_projection_gate: false,
            production_authority: false,
            external_effects: false,
            operator_acceptance: false,
            promotion: false,
        };
        transaction.rollback().await.map_err(unavailable)?;
        serde_json::to_string(&comparison)
            .map_err(|error| CognitiveStoreError::Unavailable(error.to_string()))
    }

    pub async fn shadow_grounding_explain(
        &self,
        access: &CognitiveAccess,
        expected_scope: &CognitiveScope,
        memory_id: &StableMemoryId,
        revision: u64,
    ) -> Result<String, CognitiveStoreError> {
        self.authorize(access, expected_scope)?;
        let revision_i64 = i64::try_from(revision)
            .map_err(|_| CognitiveStoreError::Invalid("memory revision exceeds i64".to_string()))?;
        let (scope_kind, workspace_sha256) = expected_scope.database_parts();
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        self.verify_durable_fact_grounding_ledger_tx(&mut transaction)
            .await?;
        let row = sqlx::query(
            "SELECT m.scope_kind, m.workspace_sha256, m.content_sha256,
                    m.verification, m.lifecycle, f.fact_set_sha256,
                    f.entity_count, f.relation_count,
                    g.fact_identity_sha256, g.receipt_sha256
             FROM memory_revisions AS m
             JOIN kg_revision_fact_sets AS f
               ON f.memory_id = m.memory_id AND f.memory_revision = m.revision
             LEFT JOIN kg_revision_fact_grounding_receipts AS g
               ON g.memory_id = m.memory_id AND g.memory_revision = m.revision
             WHERE m.owner_agent_id = ? AND m.scope_kind = ?
               AND m.workspace_sha256 IS ?
               AND m.memory_id = ? AND m.revision = ?",
        )
        .bind(self.owner_agent_id.as_str())
        .bind(scope_kind)
        .bind(workspace_sha256)
        .bind(memory_id.as_str())
        .bind(revision_i64)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(unavailable)?
        .ok_or_else(|| {
            CognitiveStoreError::Invalid("memory revision does not exist".to_string())
        })?;
        let entity_count = nonnegative_u64(
            row.try_get("entity_count").map_err(unavailable)?,
            "entity count",
        )?;
        let relation_count = nonnegative_u64(
            row.try_get("relation_count").map_err(unavailable)?,
            "relation count",
        )?;
        let receipt_sha256: Option<String> = row.try_get("receipt_sha256").map_err(unavailable)?;
        let status = if entity_count + relation_count == 0 {
            "zero_fact"
        } else if receipt_sha256.is_some() {
            "grounded_v1"
        } else {
            "legacy_unreviewed"
        };
        let span_rows = sqlx::query(
            "SELECT fact_kind, fact_key, evidence_ordinal,
                    start_byte, end_byte, evidence_sha256
             FROM kg_revision_fact_grounding_spans
             WHERE memory_id = ? AND memory_revision = ?
             ORDER BY fact_kind, fact_key, evidence_ordinal",
        )
        .bind(memory_id.as_str())
        .bind(revision_i64)
        .fetch_all(&mut *transaction)
        .await
        .map_err(unavailable)?;
        let evidence = span_rows
            .into_iter()
            .map(|span| {
                Ok(GroundingEvidenceSpan {
                    fact_kind: span.try_get("fact_kind").map_err(unavailable)?,
                    fact_key: span.try_get("fact_key").map_err(unavailable)?,
                    evidence_ordinal: u32::try_from(
                        span.try_get::<i64, _>("evidence_ordinal")
                            .map_err(unavailable)?,
                    )
                    .map_err(|_| {
                        CognitiveStoreError::Corrupt(
                            "negative grounding evidence ordinal".to_string(),
                        )
                    })?,
                    start_byte: u32::try_from(
                        span.try_get::<i64, _>("start_byte").map_err(unavailable)?,
                    )
                    .map_err(|_| {
                        CognitiveStoreError::Corrupt("negative grounding start byte".to_string())
                    })?,
                    end_byte: u32::try_from(
                        span.try_get::<i64, _>("end_byte").map_err(unavailable)?,
                    )
                    .map_err(|_| {
                        CognitiveStoreError::Corrupt("negative grounding end byte".to_string())
                    })?,
                    evidence_sha256: span.try_get("evidence_sha256").map_err(unavailable)?,
                })
            })
            .collect::<Result<Vec<_>, CognitiveStoreError>>()?;
        let explanation = ShadowGroundingExplanation {
            schema_version: SHADOW_GATE_SCHEMA_VERSION,
            mode: "grounding_explain_shadow_v4",
            memory_id: memory_id.as_str().to_string(),
            revision,
            projection_scope: expected_scope.projection_key(),
            content_sha256: row.try_get("content_sha256").map_err(unavailable)?,
            verification: row.try_get("verification").map_err(unavailable)?,
            lifecycle: row.try_get("lifecycle").map_err(unavailable)?,
            fact_set_sha256: row.try_get("fact_set_sha256").map_err(unavailable)?,
            fact_identity_sha256: row.try_get("fact_identity_sha256").map_err(unavailable)?,
            grounding_receipt_sha256: receipt_sha256,
            grounding_status: status,
            entity_count,
            relation_count,
            evidence,
            read_snapshot_transaction: true,
            ledger_verified_in_snapshot: true,
            schema_mutation_performed: false,
            write_performed: false,
            production_projection_gate: false,
            production_authority: false,
            external_effects: false,
            operator_acceptance: false,
            promotion: false,
        };
        transaction.rollback().await.map_err(unavailable)?;
        serde_json::to_string(&explanation)
            .map_err(|error| CognitiveStoreError::Unavailable(error.to_string()))
    }
}

async fn read_current_projection(
    transaction: &mut Transaction<'_, Sqlite>,
    projection_scope: &str,
) -> Result<CurrentProjectionSnapshot, CognitiveStoreError> {
    let row = sqlx::query(
        "SELECT p.generation, r.input_heads_sha256, r.output_sha256
         FROM kg_projection AS p
         LEFT JOIN kg_projection_generation_receipts AS r
           ON r.projection_scope = p.projection_scope
          AND r.generation = p.generation
         WHERE p.projection_scope = ?",
    )
    .bind(projection_scope)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(unavailable)?;
    let Some(row) = row else {
        return Ok(CurrentProjectionSnapshot {
            generation: 0,
            input_heads_sha256: None,
            output_sha256: None,
            node_count: 0,
            edge_count: 0,
        });
    };
    let generation_i64: i64 = row.try_get("generation").map_err(unavailable)?;
    let generation = nonnegative_u64(generation_i64, "projection generation")?;
    let input_heads_sha256: Option<String> =
        row.try_get("input_heads_sha256").map_err(unavailable)?;
    let output_sha256: Option<String> = row.try_get("output_sha256").map_err(unavailable)?;
    if let Some(digest) = &input_heads_sha256 {
        Sha256Digest::parse(digest.clone()).map_err(CognitiveStoreError::Corrupt)?;
    }
    if let Some(digest) = &output_sha256 {
        Sha256Digest::parse(digest.clone()).map_err(CognitiveStoreError::Corrupt)?;
    }
    let node_count = nonnegative_u64(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM kg_nodes
             WHERE projection_scope = ? AND generation = ?",
        )
        .bind(projection_scope)
        .bind(generation_i64)
        .fetch_one(&mut **transaction)
        .await
        .map_err(unavailable)?,
        "current node count",
    )?;
    let edge_count = nonnegative_u64(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM kg_edges
             WHERE projection_scope = ? AND generation = ?",
        )
        .bind(projection_scope)
        .bind(generation_i64)
        .fetch_one(&mut **transaction)
        .await
        .map_err(unavailable)?,
        "current edge count",
    )?;
    Ok(CurrentProjectionSnapshot {
        generation,
        input_heads_sha256,
        output_sha256,
        node_count,
        edge_count,
    })
}

fn verify_current_projection(
    current: &CurrentProjectionSnapshot,
    plan: &ProjectionSemanticPlan,
) -> Result<(), CognitiveStoreError> {
    let planned_nodes = u64::try_from(plan.nodes.len()).map_err(|_| {
        CognitiveStoreError::Corrupt("current semantic plan node count exceeds u64".to_string())
    })?;
    let planned_edges = u64::try_from(plan.edges.len()).map_err(|_| {
        CognitiveStoreError::Corrupt("current semantic plan edge count exceeds u64".to_string())
    })?;
    if current.generation == 0 {
        if current.input_heads_sha256.is_some()
            || current.output_sha256.is_some()
            || current.node_count != 0
            || current.edge_count != 0
            || planned_nodes != 0
            || planned_edges != 0
        {
            return Err(CognitiveStoreError::Corrupt(
                "zero KG projection generation does not match the shared semantic plan".to_string(),
            ));
        }
        return Ok(());
    }

    let input = current.input_heads_sha256.as_deref().ok_or_else(|| {
        CognitiveStoreError::Corrupt(
            "current KG projection generation has no input-head receipt".to_string(),
        )
    })?;
    let output = current.output_sha256.as_deref().ok_or_else(|| {
        CognitiveStoreError::Corrupt(
            "current KG projection generation has no output receipt".to_string(),
        )
    })?;
    if input != plan.input_heads_sha256.as_str()
        || output != plan.output_sha256.as_str()
        || current.node_count != planned_nodes
        || current.edge_count != planned_edges
    {
        return Err(CognitiveStoreError::Corrupt(
            "current KG projection generation diverges from the shared semantic plan".to_string(),
        ));
    }
    Ok(())
}

async fn read_heads(
    store: &CognitiveStore,
    transaction: &mut Transaction<'_, Sqlite>,
    scope: &CognitiveScope,
) -> Result<Vec<ProjectionHead>, CognitiveStoreError> {
    let (scope_kind, workspace_sha256) = scope.database_parts();
    let rows = sqlx::query(
        "SELECT r.memory_id, r.revision, r.content_sha256,
                r.verification, r.lifecycle, f.fact_set_sha256,
                f.entity_count, f.relation_count, g.receipt_sha256
         FROM memory_heads AS h
         JOIN memory_revisions AS r
           ON r.memory_id = h.memory_id AND r.revision = h.revision
         JOIN kg_revision_fact_sets AS f
           ON f.memory_id = r.memory_id AND f.memory_revision = r.revision
         LEFT JOIN kg_revision_fact_grounding_receipts AS g
           ON g.memory_id = r.memory_id AND g.memory_revision = r.revision
         WHERE r.owner_agent_id = ? AND r.scope_kind = ?
           AND r.workspace_sha256 IS ?
         ORDER BY r.memory_id LIMIT ?",
    )
    .bind(store.owner_agent_id.as_str())
    .bind(scope_kind)
    .bind(workspace_sha256)
    .bind(limit_plus_one(MAX_SCOPE_HEADS)?)
    .fetch_all(&mut **transaction)
    .await
    .map_err(unavailable)?;
    if rows.len() > MAX_SCOPE_HEADS {
        return Err(CognitiveStoreError::Corrupt(format!(
            "semantic shadow projection exceeds {MAX_SCOPE_HEADS} heads"
        )));
    }
    rows.into_iter()
        .map(|row| {
            Ok(ProjectionHead {
                memory_id: row.try_get("memory_id").map_err(unavailable)?,
                revision: row.try_get("revision").map_err(unavailable)?,
                content_sha256: row.try_get("content_sha256").map_err(unavailable)?,
                verification: row.try_get("verification").map_err(unavailable)?,
                lifecycle: row.try_get("lifecycle").map_err(unavailable)?,
                fact_set_sha256: row.try_get("fact_set_sha256").map_err(unavailable)?,
                entity_count: row.try_get("entity_count").map_err(unavailable)?,
                relation_count: row.try_get("relation_count").map_err(unavailable)?,
                grounding_receipt_sha256: row.try_get("receipt_sha256").map_err(unavailable)?,
            })
        })
        .collect()
}

async fn read_nodes(
    store: &CognitiveStore,
    transaction: &mut Transaction<'_, Sqlite>,
    scope: &CognitiveScope,
) -> Result<Vec<ProjectionNode>, CognitiveStoreError> {
    let (scope_kind, workspace_sha256) = scope.database_parts();
    let rows = sqlx::query(
        "SELECT e.memory_id, e.memory_revision, e.entity_key,
                e.canonical_entity_id, e.entity_type, e.label,
                e.valid_from_unix_seconds, e.valid_to_unix_seconds,
                e.source_id, e.source_revision
         FROM memory_heads h
         JOIN memory_revisions r
           ON r.memory_id = h.memory_id AND r.revision = h.revision
         JOIN kg_revision_entities e
           ON e.memory_id = r.memory_id AND e.memory_revision = r.revision
         WHERE r.owner_agent_id = ? AND r.scope_kind = ?
           AND r.workspace_sha256 IS ?
         ORDER BY e.memory_id, e.memory_revision, e.entity_key LIMIT ?",
    )
    .bind(store.owner_agent_id.as_str())
    .bind(scope_kind)
    .bind(workspace_sha256)
    .bind(limit_plus_one(MAX_SCOPE_NODES)?)
    .fetch_all(&mut **transaction)
    .await
    .map_err(unavailable)?;
    if rows.len() > MAX_SCOPE_NODES {
        return Err(CognitiveStoreError::Corrupt(format!(
            "semantic shadow projection exceeds {MAX_SCOPE_NODES} nodes"
        )));
    }
    rows.into_iter()
        .map(|row| {
            let memory_id: String = row.try_get("memory_id").map_err(unavailable)?;
            let memory_revision: i64 = row.try_get("memory_revision").map_err(unavailable)?;
            let entity_key: String = row.try_get("entity_key").map_err(unavailable)?;
            Ok(ProjectionNode {
                node_id: occurrence_node_id(&memory_id, memory_revision, &entity_key),
                canonical_entity_id: row.try_get("canonical_entity_id").map_err(unavailable)?,
                entity_key,
                entity_type: row.try_get("entity_type").map_err(unavailable)?,
                label: row.try_get("label").map_err(unavailable)?,
                valid_from: row
                    .try_get("valid_from_unix_seconds")
                    .map_err(unavailable)?,
                valid_to: row.try_get("valid_to_unix_seconds").map_err(unavailable)?,
                memory_id,
                memory_revision,
                source_id: row.try_get("source_id").map_err(unavailable)?,
                source_revision: row.try_get("source_revision").map_err(unavailable)?,
            })
        })
        .collect()
}

async fn read_edges(
    store: &CognitiveStore,
    transaction: &mut Transaction<'_, Sqlite>,
    scope: &CognitiveScope,
) -> Result<Vec<ProjectionEdge>, CognitiveStoreError> {
    let (scope_kind, workspace_sha256) = scope.database_parts();
    let rows = sqlx::query(
        "SELECT q.memory_id, q.memory_revision, q.relation_key,
                q.canonical_relation_id, q.from_entity_key,
                q.to_entity_key, q.relation,
                q.valid_from_unix_seconds, q.valid_to_unix_seconds,
                q.source_id, q.source_revision
         FROM memory_heads h
         JOIN memory_revisions r
           ON r.memory_id = h.memory_id AND r.revision = h.revision
         JOIN kg_revision_relations q
           ON q.memory_id = r.memory_id AND q.memory_revision = r.revision
         WHERE r.owner_agent_id = ? AND r.scope_kind = ?
           AND r.workspace_sha256 IS ?
         ORDER BY q.memory_id, q.memory_revision, q.relation_key LIMIT ?",
    )
    .bind(store.owner_agent_id.as_str())
    .bind(scope_kind)
    .bind(workspace_sha256)
    .bind(limit_plus_one(MAX_SCOPE_EDGES)?)
    .fetch_all(&mut **transaction)
    .await
    .map_err(unavailable)?;
    if rows.len() > MAX_SCOPE_EDGES {
        return Err(CognitiveStoreError::Corrupt(format!(
            "semantic shadow projection exceeds {MAX_SCOPE_EDGES} edges"
        )));
    }
    rows.into_iter()
        .map(|row| {
            let memory_id: String = row.try_get("memory_id").map_err(unavailable)?;
            let memory_revision: i64 = row.try_get("memory_revision").map_err(unavailable)?;
            let relation_key: String = row.try_get("relation_key").map_err(unavailable)?;
            let from_entity_key: String = row.try_get("from_entity_key").map_err(unavailable)?;
            let to_entity_key: String = row.try_get("to_entity_key").map_err(unavailable)?;
            Ok(ProjectionEdge {
                edge_id: occurrence_edge_id(&memory_id, memory_revision, &relation_key),
                canonical_relation_id: row.try_get("canonical_relation_id").map_err(unavailable)?,
                relation_key,
                from_node_id: occurrence_node_id(&memory_id, memory_revision, &from_entity_key),
                to_node_id: occurrence_node_id(&memory_id, memory_revision, &to_entity_key),
                from_entity_key,
                to_entity_key,
                relation: row.try_get("relation").map_err(unavailable)?,
                valid_from: row
                    .try_get("valid_from_unix_seconds")
                    .map_err(unavailable)?,
                valid_to: row.try_get("valid_to_unix_seconds").map_err(unavailable)?,
                memory_id,
                memory_revision,
                source_id: row.try_get("source_id").map_err(unavailable)?,
                source_revision: row.try_get("source_revision").map_err(unavailable)?,
            })
        })
        .collect()
}

fn positive_u64(value: i64, label: &str) -> Result<u64, CognitiveStoreError> {
    let value = u64::try_from(value)
        .map_err(|_| CognitiveStoreError::Corrupt(format!("negative {label}")))?;
    if value == 0 {
        return Err(CognitiveStoreError::Corrupt(format!("zero {label}")));
    }
    Ok(value)
}

fn nonnegative_u64(value: i64, label: &str) -> Result<u64, CognitiveStoreError> {
    u64::try_from(value).map_err(|_| CognitiveStoreError::Corrupt(format!("negative {label}")))
}

fn signed_delta(candidate: u64, current: u64) -> Result<i64, CognitiveStoreError> {
    i64::try_from(i128::from(candidate) - i128::from(current))
        .map_err(|_| CognitiveStoreError::Corrupt("shadow projection delta overflow".to_string()))
}

fn limit_plus_one(value: usize) -> Result<i64, CognitiveStoreError> {
    value
        .checked_add(1)
        .and_then(|next| i64::try_from(next).ok())
        .ok_or_else(|| {
            CognitiveStoreError::Corrupt("semantic shadow limit exceeds i64".to_string())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FactEvidenceSpanDraft;
    use crate::GroundedFactKind;
    use crate::GroundedKgFactSetDraft;
    use crate::KgEntityFactDraft;
    use crate::KgFactSetDraft;
    use crate::LedgerSourceKind;
    use crate::MemoryDraft;
    use crate::MemoryLifecycleState;
    use crate::MemoryRevisionDraft;
    use crate::MemoryVerification;
    use crate::SourceDraft;
    use crate::cognitive_test_support::agent_id;
    use crate::cognitive_test_support::layout;
    use tempfile::TempDir;

    fn source(text: &str, key: &str) -> SourceDraft {
        SourceDraft {
            scope: CognitiveScope::AgentPrivate,
            kind: LedgerSourceKind::ExplicitMemoryDirective,
            event_key: key.to_string(),
            content: text.as_bytes().to_vec(),
            observed_at_unix_seconds: 100,
        }
    }

    fn memory(text: &str, key: &str) -> MemoryDraft {
        MemoryDraft {
            stable_key: key.to_string(),
            revision: MemoryRevisionDraft {
                scope: CognitiveScope::AgentPrivate,
                content: text.to_string(),
                verification: MemoryVerification::Verified,
                lifecycle: MemoryLifecycleState::Active,
                valid_from_unix_seconds: 100,
                valid_to_unix_seconds: None,
                citations: Vec::new(),
            },
        }
    }

    fn facts(label: &str) -> KgFactSetDraft {
        KgFactSetDraft {
            entities: vec![KgEntityFactDraft {
                key: label.to_ascii_lowercase(),
                entity_type: "project".to_string(),
                label: label.to_string(),
            }],
            relations: Vec::new(),
        }
    }

    fn grounded(text: &str, label: &str) -> GroundedKgFactSetDraft {
        let start = text.find(label).expect("label");
        let end = start + label.len();
        GroundedKgFactSetDraft {
            facts: facts(label),
            evidence: vec![
                FactEvidenceSpanDraft::from_source_text(
                    GroundedFactKind::Entity,
                    label.to_ascii_lowercase(),
                    text,
                    start,
                    end,
                )
                .expect("evidence"),
            ],
        }
    }

    #[tokio::test]
    async fn shadow_projection_planner_matches_product_when_all_heads_are_grounded() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(251);
        let store = CognitiveStore::open_with_durable_fact_grounding(&layout(&temp, &owner))
            .await
            .expect("store");
        let access = CognitiveAccess::agent_private(owner);
        let text = "Project Aurora is active.";
        store
            .remember_with_durable_grounded_kg(
                &access,
                &source(text, "shadow:grounded"),
                &memory(text, "shadow-grounded"),
                &grounded(text, "Project Aurora"),
            )
            .await
            .expect("write");
        let receipt = store
            .shadow_grounded_projection_compare(&access, &CognitiveScope::AgentPrivate)
            .await
            .expect("compare");
        let value: serde_json::Value = serde_json::from_str(&receipt).expect("json");
        assert_eq!(
            value["current"]["output_sha256"],
            value["grounded_candidate"]["output_sha256"]
        );
        assert_eq!(value["shared_projection_planner"], true);
        assert_eq!(value["current_projection_replanned"], true);
        assert_eq!(value["ledger_verified_in_snapshot"], true);
        assert_eq!(value["write_performed"], false);
    }

    #[tokio::test]
    async fn shadow_projection_planner_excludes_legacy_semantically() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(252);
        let store = CognitiveStore::open_with_durable_fact_grounding(&layout(&temp, &owner))
            .await
            .expect("store");
        let access = CognitiveAccess::agent_private(owner);
        let grounded_text = "Project Aurora is active.";
        store
            .remember_with_durable_grounded_kg(
                &access,
                &source(grounded_text, "shadow:grounded"),
                &memory(grounded_text, "shadow-grounded"),
                &grounded(grounded_text, "Project Aurora"),
            )
            .await
            .expect("grounded");
        let legacy_text = "Project Borealis is active.";
        store
            .remember_with_kg(
                &access,
                &source(legacy_text, "shadow:legacy"),
                &memory(legacy_text, "shadow-legacy"),
                &facts("Project Borealis"),
            )
            .await
            .expect("legacy");
        let receipt = store
            .shadow_grounded_projection_compare(&access, &CognitiveScope::AgentPrivate)
            .await
            .expect("compare");
        let value: serde_json::Value = serde_json::from_str(&receipt).expect("json");
        assert_eq!(value["grounded_candidate"]["node_count"], 1);
        assert_eq!(value["current"]["node_count"], 2);
        assert_eq!(value["excluded_heads"][0]["reason"], "legacy_unreviewed");
        assert_eq!(value["current_projection_replanned"], true);
        assert_eq!(value["ledger_verified_in_snapshot"], true);
        assert_ne!(
            value["current"]["output_sha256"],
            value["grounded_candidate"]["output_sha256"]
        );
    }

    #[tokio::test]
    async fn shadow_compare_rejects_a_current_receipt_that_diverges_from_the_shared_plan() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(253);
        let store = CognitiveStore::open_with_durable_fact_grounding(&layout(&temp, &owner))
            .await
            .expect("store");
        let access = CognitiveAccess::agent_private(owner);
        let text = "Project Aurora is active.";
        store
            .remember_with_durable_grounded_kg(
                &access,
                &source(text, "shadow:current-drift"),
                &memory(text, "shadow-current-drift"),
                &grounded(text, "Project Aurora"),
            )
            .await
            .expect("write");

        sqlx::query("DROP TRIGGER kg_projection_generation_receipts_no_update")
            .execute(&store.pool)
            .await
            .expect("drop receipt immutability guard");
        sqlx::query(
            "UPDATE kg_projection_generation_receipts
             SET output_sha256 =
                 '0000000000000000000000000000000000000000000000000000000000000000'
             WHERE projection_scope = 'agent_private'",
        )
        .execute(&store.pool)
        .await
        .expect("tamper current receipt");
        sqlx::query(
            "CREATE TRIGGER kg_projection_generation_receipts_no_update
             BEFORE UPDATE ON kg_projection_generation_receipts BEGIN
                 SELECT RAISE(ABORT, 'KG projection generation receipts are immutable');
             END",
        )
        .execute(&store.pool)
        .await
        .expect("restore receipt immutability guard");

        let error = store
            .shadow_grounded_projection_compare(&access, &CognitiveScope::AgentPrivate)
            .await
            .expect_err("current receipt drift must fail closed");
        assert!(matches!(error, CognitiveStoreError::Corrupt(_)));
    }

    #[tokio::test]
    async fn shadow_read_does_not_install_missing_grounding_schema() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(254);
        let store = CognitiveStore::open(&layout(&temp, &owner))
            .await
            .expect("store");
        let access = CognitiveAccess::agent_private(owner);
        let error = store
            .shadow_grounded_projection_compare(&access, &CognitiveScope::AgentPrivate)
            .await
            .expect_err("missing grounding schema must fail closed");
        assert!(matches!(error, CognitiveStoreError::Corrupt(_)));
        let installed: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                 SELECT 1 FROM sqlite_schema
                 WHERE type = 'table'
                   AND name = 'cognitive_fact_grounding_migrations'
             )",
        )
        .fetch_one(&store.pool)
        .await
        .expect("schema query");
        assert!(!installed);
    }
}
