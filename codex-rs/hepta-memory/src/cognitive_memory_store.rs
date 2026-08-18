use std::collections::BTreeSet;

use codex_hepta_contracts::Sha256Digest;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;

use crate::cognitive_model::CognitiveAccess;
use crate::cognitive_model::CognitiveScope;
use crate::cognitive_model::MAX_MEMORY_BYTES;
use crate::cognitive_model::MemoryDraft;
use crate::cognitive_model::MemoryLifecycleState;
use crate::cognitive_model::MemoryRevisionDraft;
use crate::cognitive_model::MemoryRevisionId;
use crate::cognitive_model::MemoryRevisionRecord;
use crate::cognitive_model::MemoryVerification;
use crate::cognitive_model::SOURCE_REVISION;
use crate::cognitive_model::SourceEventId;
use crate::cognitive_model::SourceRevisionId;
use crate::cognitive_model::StableMemoryId;
use crate::cognitive_store::CognitiveStore;
use crate::cognitive_store::CognitiveStoreError;
use crate::cognitive_store::decode_scope;
use crate::cognitive_store::unavailable;
use crate::cognitive_store::validate_key;

impl CognitiveStore {
    pub async fn create_memory(
        &self,
        access: &CognitiveAccess,
        draft: &MemoryDraft,
    ) -> Result<MemoryRevisionRecord, CognitiveStoreError> {
        self.authorize(access, &draft.revision.scope)?;
        validate_key(&draft.stable_key, "stable memory key")?;
        validate_revision_draft(&draft.revision)?;
        let memory_id = StableMemoryId::for_key(
            &self.owner_agent_id,
            &draft.revision.scope,
            &draft.stable_key,
        );
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        if sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM memory_heads WHERE memory_id = ?")
            .bind(memory_id.as_str())
            .fetch_one(&mut *transaction)
            .await
            .map_err(unavailable)?
            != 0
        {
            return Err(CognitiveStoreError::Conflict(format!(
                "memory {} already exists",
                memory_id.as_str()
            )));
        }
        insert_revision(
            &mut transaction,
            &self.owner_agent_id,
            &memory_id,
            1,
            None,
            &draft.revision,
        )
        .await?;
        sqlx::query("INSERT INTO memory_heads (memory_id, revision) VALUES (?, 1)")
            .bind(memory_id.as_str())
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
        transaction.commit().await.map_err(unavailable)?;
        self.latest_memory(access, &memory_id).await
    }

    pub async fn revise_memory(
        &self,
        access: &CognitiveAccess,
        memory_id: &StableMemoryId,
        expected_revision: u64,
        draft: &MemoryRevisionDraft,
    ) -> Result<MemoryRevisionRecord, CognitiveStoreError> {
        validate_revision_draft(draft)?;
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        let row = sqlx::query(
            "SELECT h.revision, r.scope_kind, r.workspace_sha256
             FROM memory_heads h JOIN memory_revisions r
               ON r.memory_id = h.memory_id AND r.revision = h.revision
             WHERE h.memory_id = ?",
        )
        .bind(memory_id.as_str())
        .fetch_optional(&mut *transaction)
        .await
        .map_err(unavailable)?
        .ok_or_else(|| CognitiveStoreError::Invalid("memory does not exist".to_string()))?;
        let current_revision: i64 = row.try_get("revision").map_err(unavailable)?;
        let current_scope = decode_scope(&row)?;
        self.authorize(access, &current_scope)?;
        if &current_scope != draft.scope_ref() {
            return Err(CognitiveStoreError::AccessDenied(
                "memory scope cannot change across revisions".to_string(),
            ));
        }
        let expected_revision = i64::try_from(expected_revision)
            .map_err(|_| CognitiveStoreError::Invalid("revision exceeds i64".to_string()))?;
        if current_revision != expected_revision {
            return Err(CognitiveStoreError::Conflict(format!(
                "expected memory revision {expected_revision}, found {current_revision}"
            )));
        }
        let next_revision = current_revision
            .checked_add(1)
            .ok_or_else(|| CognitiveStoreError::Corrupt("memory revision overflow".to_string()))?;
        insert_revision(
            &mut transaction,
            &self.owner_agent_id,
            memory_id,
            next_revision,
            Some(current_revision),
            draft,
        )
        .await?;
        let updated = sqlx::query(
            "UPDATE memory_heads SET revision = ? WHERE memory_id = ? AND revision = ?",
        )
        .bind(next_revision)
        .bind(memory_id.as_str())
        .bind(current_revision)
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
        if updated.rows_affected() != 1 {
            return Err(CognitiveStoreError::Conflict(
                "memory head changed during revision".to_string(),
            ));
        }
        transaction.commit().await.map_err(unavailable)?;
        self.latest_memory(access, memory_id).await
    }

    pub async fn latest_memory(
        &self,
        access: &CognitiveAccess,
        memory_id: &StableMemoryId,
    ) -> Result<MemoryRevisionRecord, CognitiveStoreError> {
        let row = sqlx::query(
            "SELECT r.* FROM memory_heads h JOIN memory_revisions r
               ON r.memory_id = h.memory_id AND r.revision = h.revision
             WHERE h.memory_id = ?",
        )
        .bind(memory_id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(unavailable)?
        .ok_or_else(|| CognitiveStoreError::Invalid("memory does not exist".to_string()))?;
        let scope = decode_scope(&row)?;
        self.authorize(access, &scope)?;
        decode_revision(&self.pool, row, scope).await
    }
}

impl MemoryRevisionDraft {
    fn scope_ref(&self) -> &CognitiveScope {
        &self.scope
    }
}

async fn insert_revision(
    transaction: &mut Transaction<'_, Sqlite>,
    owner_agent_id: &codex_hepta_contracts::AgentId,
    memory_id: &StableMemoryId,
    revision: i64,
    supersedes_revision: Option<i64>,
    draft: &MemoryRevisionDraft,
) -> Result<(), CognitiveStoreError> {
    verify_citations(transaction, owner_agent_id, &draft.scope, &draft.citations).await?;
    let content_sha256 = Sha256Digest::for_bytes(draft.content.as_bytes());
    let (scope_kind, workspace_sha256) = draft.scope.database_parts();
    let (lifecycle, tombstone_reason) = draft.lifecycle.database_parts();
    sqlx::query(
        "INSERT INTO memory_revisions (
            memory_id, revision, owner_agent_id, scope_kind, workspace_sha256,
            content, content_sha256, verification, lifecycle, tombstone_reason,
            valid_from_unix_seconds, valid_to_unix_seconds, supersedes_revision,
            recorded_at_unix_seconds
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, unixepoch())",
    )
    .bind(memory_id.as_str())
    .bind(revision)
    .bind(owner_agent_id.as_str())
    .bind(scope_kind)
    .bind(workspace_sha256)
    .bind(&draft.content)
    .bind(content_sha256.as_str())
    .bind(draft.verification.as_str())
    .bind(lifecycle)
    .bind(tombstone_reason)
    .bind(draft.valid_from_unix_seconds)
    .bind(draft.valid_to_unix_seconds)
    .bind(supersedes_revision)
    .execute(&mut **transaction)
    .await
    .map_err(unavailable)?;
    for (ordinal, citation) in draft.citations.iter().enumerate() {
        let ordinal = i64::try_from(ordinal)
            .map_err(|_| CognitiveStoreError::Invalid("too many citations".to_string()))?;
        sqlx::query(
            "INSERT INTO memory_citations (
                memory_id, memory_revision, ordinal, source_id, source_revision
             ) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(memory_id.as_str())
        .bind(revision)
        .bind(ordinal)
        .bind(citation.source_id.as_str())
        .bind(i64::try_from(citation.revision).unwrap_or(i64::MAX))
        .execute(&mut **transaction)
        .await
        .map_err(unavailable)?;
    }
    sqlx::query("INSERT INTO memory_fts (memory_id, revision, content) VALUES (?, ?, ?)")
        .bind(memory_id.as_str())
        .bind(revision)
        .bind(&draft.content)
        .execute(&mut **transaction)
        .await
        .map_err(unavailable)?;
    Ok(())
}

async fn verify_citations(
    transaction: &mut Transaction<'_, Sqlite>,
    owner_agent_id: &codex_hepta_contracts::AgentId,
    scope: &CognitiveScope,
    citations: &[SourceRevisionId],
) -> Result<(), CognitiveStoreError> {
    if citations.is_empty() || citations.len() > 32 {
        return Err(CognitiveStoreError::Invalid(
            "memory revisions require 1..=32 source citations".to_string(),
        ));
    }
    let mut unique = BTreeSet::new();
    for citation in citations {
        if citation.revision != SOURCE_REVISION
            || !unique.insert((citation.source_id.as_str(), citation.revision))
        {
            return Err(CognitiveStoreError::Invalid(
                "memory citations must be unique source revision 1 references".to_string(),
            ));
        }
        let row = sqlx::query(
            "SELECT owner_agent_id, scope_kind, workspace_sha256 FROM source_ledger
             WHERE source_id = ? AND source_revision = ?",
        )
        .bind(citation.source_id.as_str())
        .bind(i64::try_from(citation.revision).unwrap_or(i64::MAX))
        .fetch_optional(&mut **transaction)
        .await
        .map_err(unavailable)?
        .ok_or_else(|| {
            CognitiveStoreError::Invalid("citation source does not exist".to_string())
        })?;
        let source_owner: String = row.try_get("owner_agent_id").map_err(unavailable)?;
        let source_scope = decode_scope(&row)?;
        if source_owner != owner_agent_id.as_str() || &source_scope != scope {
            return Err(CognitiveStoreError::AccessDenied(
                "memory and citation source must have the same agent and scope".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_revision_draft(draft: &MemoryRevisionDraft) -> Result<(), CognitiveStoreError> {
    if draft.content.is_empty() || draft.content.len() > MAX_MEMORY_BYTES {
        return Err(CognitiveStoreError::Invalid(format!(
            "memory content must contain 1..={MAX_MEMORY_BYTES} bytes"
        )));
    }
    if draft
        .valid_to_unix_seconds
        .is_some_and(|until| until <= draft.valid_from_unix_seconds)
    {
        return Err(CognitiveStoreError::Invalid(
            "memory valid_to must be after valid_from".to_string(),
        ));
    }
    if let MemoryLifecycleState::Tombstoned { reason } = &draft.lifecycle
        && (reason.trim().is_empty() || reason.len() > 256)
    {
        return Err(CognitiveStoreError::Invalid(
            "tombstone reason must contain 1..=256 bytes".to_string(),
        ));
    }
    Ok(())
}

async fn decode_revision(
    pool: &sqlx::SqlitePool,
    row: sqlx::sqlite::SqliteRow,
    scope: CognitiveScope,
) -> Result<MemoryRevisionRecord, CognitiveStoreError> {
    let memory_id = StableMemoryId::parse(row.try_get("memory_id").map_err(unavailable)?)
        .map_err(CognitiveStoreError::Corrupt)?;
    let revision: i64 = row.try_get("revision").map_err(unavailable)?;
    let citations = sqlx::query(
        "SELECT source_id, source_revision FROM memory_citations
         WHERE memory_id = ? AND memory_revision = ? ORDER BY ordinal",
    )
    .bind(memory_id.as_str())
    .bind(revision)
    .fetch_all(pool)
    .await
    .map_err(unavailable)?
    .into_iter()
    .map(|citation| {
        let source_id = SourceEventId::parse(citation.try_get("source_id").map_err(unavailable)?)
            .map_err(CognitiveStoreError::Corrupt)?;
        let revision = u64::try_from(
            citation
                .try_get::<i64, _>("source_revision")
                .map_err(unavailable)?,
        )
        .map_err(|_| CognitiveStoreError::Corrupt("negative source revision".to_string()))?;
        Ok(SourceRevisionId {
            source_id,
            revision,
        })
    })
    .collect::<Result<Vec<_>, CognitiveStoreError>>()?;
    let content: String = row.try_get("content").map_err(unavailable)?;
    let content_sha256 = Sha256Digest::parse(
        row.try_get::<String, _>("content_sha256")
            .map_err(unavailable)?,
    )
    .map_err(CognitiveStoreError::Corrupt)?;
    if content_sha256 != Sha256Digest::for_bytes(content.as_bytes()) {
        return Err(CognitiveStoreError::Corrupt(
            "memory content digest does not match stored content".to_string(),
        ));
    }
    let lifecycle = MemoryLifecycleState::parse(
        row.try_get("lifecycle").map_err(unavailable)?,
        row.try_get("tombstone_reason").map_err(unavailable)?,
    )
    .map_err(CognitiveStoreError::Corrupt)?;
    Ok(MemoryRevisionRecord {
        id: MemoryRevisionId {
            memory_id,
            revision: u64::try_from(revision).map_err(|_| {
                CognitiveStoreError::Corrupt("negative memory revision".to_string())
            })?,
        },
        scope,
        content,
        content_sha256,
        verification: MemoryVerification::parse(row.try_get("verification").map_err(unavailable)?)
            .map_err(CognitiveStoreError::Corrupt)?,
        lifecycle,
        valid_from_unix_seconds: row
            .try_get("valid_from_unix_seconds")
            .map_err(unavailable)?,
        valid_to_unix_seconds: row.try_get("valid_to_unix_seconds").map_err(unavailable)?,
        supersedes_revision: row
            .try_get::<Option<i64>, _>("supersedes_revision")
            .map_err(unavailable)?
            .map(u64::try_from)
            .transpose()
            .map_err(|_| {
                CognitiveStoreError::Corrupt("negative superseded revision".to_string())
            })?,
        citations,
    })
}
