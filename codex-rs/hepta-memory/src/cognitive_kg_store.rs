use std::collections::BTreeSet;

use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;

use crate::cognitive_model::CognitiveAccess;
use crate::cognitive_model::CognitiveScope;
use crate::cognitive_model::KgEdge;
use crate::cognitive_model::KgNode;
use crate::cognitive_model::MemoryRevisionId;
use crate::cognitive_model::SourceRevisionId;
use crate::cognitive_store::CognitiveStore;
use crate::cognitive_store::CognitiveStoreError;
use crate::cognitive_store::ProjectionGeneration;
use crate::cognitive_store::decode_scope;
use crate::cognitive_store::unavailable;
use crate::cognitive_store::validate_key;

impl CognitiveStore {
    /// Atomically replaces one scope's rebuildable Temporal KG projection.
    ///
    /// Nodes and edges are accepted only when their memory revision and exact
    /// cited source revision already exist in this agent-owned store.
    pub async fn rebuild_kg_projection(
        &self,
        access: &CognitiveAccess,
        scope: &CognitiveScope,
        nodes: &[KgNode],
        edges: &[KgEdge],
    ) -> Result<ProjectionGeneration, CognitiveStoreError> {
        self.authorize(access, scope)?;
        validate_projection_shape(nodes, edges)?;
        let projection_scope = scope.projection_key();
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        for node in nodes {
            verify_projection_provenance(
                &mut transaction,
                &self.owner_agent_id,
                scope,
                &node.memory,
                &node.source,
            )
            .await?;
        }
        for edge in edges {
            verify_projection_provenance(
                &mut transaction,
                &self.owner_agent_id,
                scope,
                &edge.memory,
                &edge.source,
            )
            .await?;
        }
        sqlx::query(
            "INSERT INTO kg_projection (projection_scope, generation) VALUES (?, 0)
             ON CONFLICT(projection_scope) DO NOTHING",
        )
        .bind(&projection_scope)
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
        let current: i64 =
            sqlx::query_scalar("SELECT generation FROM kg_projection WHERE projection_scope = ?")
                .bind(&projection_scope)
                .fetch_one(&mut *transaction)
                .await
                .map_err(unavailable)?;
        let next = current
            .checked_add(1)
            .ok_or_else(|| CognitiveStoreError::Corrupt("KG generation overflow".to_string()))?;
        sqlx::query("DELETE FROM kg_edges WHERE projection_scope = ?")
            .bind(&projection_scope)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
        sqlx::query("DELETE FROM kg_nodes WHERE projection_scope = ?")
            .bind(&projection_scope)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
        sqlx::query("DELETE FROM kg_entity_fts WHERE projection_scope = ?")
            .bind(&projection_scope)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
        for node in nodes {
            sqlx::query(
                "INSERT INTO kg_nodes (
                    projection_scope, generation, node_id, entity_type, label,
                    valid_from_unix_seconds, valid_to_unix_seconds,
                    memory_id, memory_revision, source_id, source_revision
                 ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&projection_scope)
            .bind(next)
            .bind(&node.node_id)
            .bind(&node.entity_type)
            .bind(&node.label)
            .bind(node.valid_from_unix_seconds)
            .bind(node.valid_to_unix_seconds)
            .bind(node.memory.memory_id.as_str())
            .bind(to_i64(node.memory.revision, "memory revision")?)
            .bind(node.source.source_id.as_str())
            .bind(to_i64(node.source.revision, "source revision")?)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
            sqlx::query(
                "INSERT INTO kg_entity_fts (
                    projection_scope, generation, node_id, entity_type, label
                 ) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(&projection_scope)
            .bind(next)
            .bind(&node.node_id)
            .bind(&node.entity_type)
            .bind(&node.label)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
        }
        for edge in edges {
            sqlx::query(
                "INSERT INTO kg_edges (
                    projection_scope, generation, edge_id, from_node_id, to_node_id,
                    relation, valid_from_unix_seconds, valid_to_unix_seconds,
                    memory_id, memory_revision, source_id, source_revision
                 ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&projection_scope)
            .bind(next)
            .bind(&edge.edge_id)
            .bind(&edge.from_node_id)
            .bind(&edge.to_node_id)
            .bind(&edge.relation)
            .bind(edge.valid_from_unix_seconds)
            .bind(edge.valid_to_unix_seconds)
            .bind(edge.memory.memory_id.as_str())
            .bind(to_i64(edge.memory.revision, "memory revision")?)
            .bind(edge.source.source_id.as_str())
            .bind(to_i64(edge.source.revision, "source revision")?)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
        }
        let updated = sqlx::query(
            "UPDATE kg_projection SET generation = ?
             WHERE projection_scope = ? AND generation = ?",
        )
        .bind(next)
        .bind(&projection_scope)
        .bind(current)
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
        if updated.rows_affected() != 1 {
            return Err(CognitiveStoreError::Conflict(
                "KG projection generation changed during rebuild".to_string(),
            ));
        }
        transaction.commit().await.map_err(unavailable)?;
        Ok(ProjectionGeneration(u64::try_from(next).map_err(|_| {
            CognitiveStoreError::Corrupt("negative KG generation".to_string())
        })?))
    }
}

fn validate_projection_shape(
    nodes: &[KgNode],
    edges: &[KgEdge],
) -> Result<(), CognitiveStoreError> {
    if nodes.len() > 10_000 || edges.len() > 50_000 {
        return Err(CognitiveStoreError::Invalid(
            "KG projection exceeds the local bounded rebuild limit".to_string(),
        ));
    }
    let mut node_ids = BTreeSet::new();
    for node in nodes {
        validate_key(&node.node_id, "KG node id")?;
        validate_key(&node.entity_type, "KG entity type")?;
        validate_label(&node.label, "KG node label")?;
        validate_interval(node.valid_from_unix_seconds, node.valid_to_unix_seconds)?;
        if !node_ids.insert(node.node_id.as_str()) {
            return Err(CognitiveStoreError::Invalid(
                "KG node ids must be unique".to_string(),
            ));
        }
    }
    let mut edge_ids = BTreeSet::new();
    for edge in edges {
        validate_key(&edge.edge_id, "KG edge id")?;
        validate_key(&edge.relation, "KG relation")?;
        validate_interval(edge.valid_from_unix_seconds, edge.valid_to_unix_seconds)?;
        if !edge_ids.insert(edge.edge_id.as_str())
            || !node_ids.contains(edge.from_node_id.as_str())
            || !node_ids.contains(edge.to_node_id.as_str())
        {
            return Err(CognitiveStoreError::Invalid(
                "KG edges require unique ids and in-projection endpoints".to_string(),
            ));
        }
    }
    Ok(())
}

async fn verify_projection_provenance(
    transaction: &mut Transaction<'_, Sqlite>,
    owner: &codex_hepta_contracts::AgentId,
    scope: &CognitiveScope,
    memory: &MemoryRevisionId,
    source: &SourceRevisionId,
) -> Result<(), CognitiveStoreError> {
    let memory_row = sqlx::query(
        "SELECT owner_agent_id, scope_kind, workspace_sha256, lifecycle
         FROM memory_revisions WHERE memory_id = ? AND revision = ?",
    )
    .bind(memory.memory_id.as_str())
    .bind(to_i64(memory.revision, "memory revision")?)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(unavailable)?
    .ok_or_else(|| CognitiveStoreError::Invalid("KG memory revision does not exist".to_string()))?;
    let memory_owner: String = memory_row.try_get("owner_agent_id").map_err(unavailable)?;
    let memory_scope = decode_scope(&memory_row)?;
    let lifecycle: String = memory_row.try_get("lifecycle").map_err(unavailable)?;
    if memory_owner != owner.as_str() || &memory_scope != scope || lifecycle != "active" {
        return Err(CognitiveStoreError::AccessDenied(
            "KG projection memory must be active and match the requested agent scope".to_string(),
        ));
    }
    let source_row = sqlx::query(
        "SELECT owner_agent_id, scope_kind, workspace_sha256 FROM source_ledger
         WHERE source_id = ? AND source_revision = ?",
    )
    .bind(source.source_id.as_str())
    .bind(to_i64(source.revision, "source revision")?)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(unavailable)?
    .ok_or_else(|| CognitiveStoreError::Invalid("KG source revision does not exist".to_string()))?;
    let source_owner: String = source_row.try_get("owner_agent_id").map_err(unavailable)?;
    let source_scope = decode_scope(&source_row)?;
    if source_owner != owner.as_str() || &source_scope != scope {
        return Err(CognitiveStoreError::AccessDenied(
            "KG projection source must match the requested agent scope".to_string(),
        ));
    }
    let cited: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM memory_citations
         WHERE memory_id = ? AND memory_revision = ?
           AND source_id = ? AND source_revision = ?",
    )
    .bind(memory.memory_id.as_str())
    .bind(to_i64(memory.revision, "memory revision")?)
    .bind(source.source_id.as_str())
    .bind(to_i64(source.revision, "source revision")?)
    .fetch_one(&mut **transaction)
    .await
    .map_err(unavailable)?;
    if cited != 1 {
        return Err(CognitiveStoreError::Invalid(
            "KG source must be an exact citation of its memory revision".to_string(),
        ));
    }
    Ok(())
}

fn validate_label(value: &str, label: &str) -> Result<(), CognitiveStoreError> {
    if value.trim().is_empty() || value.len() > 4 * 1024 || value.as_bytes().contains(&0) {
        return Err(CognitiveStoreError::Invalid(format!(
            "{label} must contain 1..=4096 non-NUL bytes"
        )));
    }
    Ok(())
}

fn validate_interval(from: i64, to: Option<i64>) -> Result<(), CognitiveStoreError> {
    if to.is_some_and(|to| to <= from) {
        return Err(CognitiveStoreError::Invalid(
            "KG valid_to must be after valid_from".to_string(),
        ));
    }
    Ok(())
}

fn to_i64(value: u64, label: &str) -> Result<i64, CognitiveStoreError> {
    i64::try_from(value).map_err(|_| CognitiveStoreError::Invalid(format!("{label} exceeds i64")))
}
