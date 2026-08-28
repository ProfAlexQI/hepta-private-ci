//! P0.2 durable source-span grounding ledger.
//!
//! The module is intentionally nested under `framing` so the P0.1 public
//! surface remains unchanged. Public inherent methods on `CognitiveStore` are
//! still callable by qualification hosts. The default `CognitiveStore::open`
//! path and production projection authority remain unchanged.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::str;

use codex_hepta_contracts::Sha256Digest;
use codex_hepta_paths::HeptaAgentLayout;
use sha2::Digest;
use sha2::Sha256;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqliteConnection;
use sqlx::SqlitePool;
use sqlx::Transaction;

use crate::CognitiveAccess;
use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::CognitiveWriteReceipt;
use crate::GroundedFactKind;
use crate::GroundedKgFactSetDraft;
use crate::MemoryDraft;
use crate::MemoryLifecycleState;
use crate::MemoryRevisionDraft;
use crate::MemoryRevisionRecord;
use crate::MemoryVerification;
use crate::SourceDraft;
use crate::SourceRevisionId;
use crate::StableMemoryId;
use crate::cognitive_intelligence_writer::CanonicalFactSet;
use crate::cognitive_store::unavailable;

#[path = "durable/grounding.rs"]
mod grounding;
#[path = "durable/schema.rs"]
mod schema;

#[cfg(test)]
#[path = "durable/tests.rs"]
mod tests;

const COMPONENT_MIGRATION_VERSION: i64 = 11;
const COMPONENT_MIGRATION_DESCRIPTION: &str = "durable fact grounding ledger";
const GROUNDING_CONTRACT: &str = "source_span_grounding_v1";
const STRUCTURED_EXTRACTOR_CONTRACT: &str = "structured_cognitive_kg_v1";
const MAX_GROUNDING_SOURCE_BYTES: usize = 256 * 1024;
const MAX_FACT_KEY_BYTES: usize = 256;
const MAX_ENTITY_LABEL_BYTES: usize = 1024;
const MAX_RELATION_BYTES: usize = 128;
const MAX_SPANS_PER_FACT: usize = 4;
const MAX_TOTAL_SPANS: usize = 768;
const MAX_GROUNDING_RECEIPTS: usize = 10_000;
const COMPONENT_MIGRATION_SQL: &str =
    include_str!("../../grounding-migrations/0011_fact_grounding.sql");

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FactIdentity {
    kind: GroundedFactKind,
    key: String,
}

#[derive(Clone, Debug)]
enum FactSupport {
    Entity {
        label: String,
    },
    Relation {
        from_label: String,
        to_label: String,
        relation: String,
    },
}

#[derive(Clone, Debug)]
struct PreparedSpan {
    identity: FactIdentity,
    ordinal: u32,
    start_byte: u32,
    end_byte: u32,
    evidence_sha256: Sha256Digest,
}

#[derive(Clone, Debug)]
struct PreparedGrounding {
    source_content_sha256: Sha256Digest,
    fact_identity_sha256: Sha256Digest,
    spans: Vec<PreparedSpan>,
}

struct DurableReceiptDigestParts<'a> {
    memory_id: &'a str,
    memory_revision: u64,
    source_id: &'a str,
    source_revision: u64,
    source_content_sha256: &'a str,
    fact_set_sha256: &'a str,
    fact_identity_sha256: &'a str,
    spans: &'a [PreparedSpan],
}

impl CognitiveStore {
    /// Opens the normal cognitive store, applies component migration 0011,
    /// verifies the component schema, and recomputes every durable grounding
    /// receipt. This remains a qualification-only opt-in path.
    pub async fn open_with_durable_fact_grounding(
        layout: &HeptaAgentLayout,
    ) -> Result<Self, CognitiveStoreError> {
        let store = Self::open(layout).await?;
        store.ensure_durable_fact_grounding_schema().await?;
        store.verify_durable_fact_grounding_ledger().await?;
        Ok(store)
    }

    /// Applies the append-only P0.2 component migration exactly once and
    /// verifies its dedicated migration ledger and schema oracle.
    pub async fn ensure_durable_fact_grounding_schema(
        &self,
    ) -> Result<(), CognitiveStoreError> {
        schema::ensure(&self.pool).await
    }

    /// Recomputes the component schema and every durable fact-grounding
    /// receipt from source bytes and immutable KG facts inside one read
    /// transaction.
    pub async fn verify_durable_fact_grounding_ledger(
        &self,
    ) -> Result<(), CognitiveStoreError> {
        let mut transaction = self.pool.begin().await.map_err(unavailable)?;
        self.verify_durable_fact_grounding_ledger_tx(&mut transaction)
            .await?;
        transaction.rollback().await.map_err(unavailable)
    }

    /// Verifies the component schema and every durable grounding receipt using
    /// the caller's exact SQLite snapshot. Shadow readers call this before any
    /// projection/head/span read on the same transaction.
    pub(crate) async fn verify_durable_fact_grounding_ledger_tx(
        &self,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), CognitiveStoreError> {
        schema::verify_tx(transaction).await?;
        grounding::verify_receipts(&mut **transaction, self.owner_agent_id.as_str()).await
    }

    /// Atomically appends source, memory, KG facts, durable grounding evidence,
    /// and the next projection generation.
    pub async fn remember_with_durable_grounded_kg(
        &self,
        access: &CognitiveAccess,
        source: &SourceDraft,
        draft: &MemoryDraft,
        grounded: &GroundedKgFactSetDraft,
    ) -> Result<CognitiveWriteReceipt, CognitiveStoreError> {
        self.ensure_durable_fact_grounding_schema().await?;
        grounding::validate_source_binding(
            source,
            &draft.revision.scope,
            &draft.revision.content,
        )?;
        grounding::require_groundable_revision(&draft.revision, grounded)?;
        let prepared = grounding::prepare(source, grounded)?;

        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(unavailable)?;
        let citation = self
            .append_source_tx(&mut transaction, access, source)
            .await?;
        let mut bound_draft = draft.clone();
        grounding::bind_exact_citation(&mut bound_draft.revision, &citation)?;
        let memory = self
            .create_memory_revision_tx(&mut transaction, access, &bound_draft)
            .await?;
        let canonical = self.canonicalize_fact_set(
            &memory,
            &citation,
            &grounded.facts,
            STRUCTURED_EXTRACTOR_CONTRACT,
        )?;
        grounding::validate_canonical_identity_binding(&prepared, &canonical)?;
        self.insert_revision_facts_tx(&mut transaction, &memory, &citation, &canonical)
            .await?;
        grounding::insert_tx(
            &mut transaction,
            &memory,
            &citation,
            &canonical,
            &prepared,
        )
        .await?;
        let projection = self
            .refresh_scope_projection_tx(
                &mut transaction,
                &memory.scope,
                &memory,
                &citation,
                &canonical,
            )
            .await?;
        transaction.commit().await.map_err(unavailable)?;
        Ok(CognitiveWriteReceipt {
            memory,
            source: citation,
            projection,
        })
    }

    /// Atomically appends a compare-and-swap correction and its durable
    /// grounding evidence before publishing the next projection generation.
    pub async fn correct_with_durable_grounded_kg(
        &self,
        access: &CognitiveAccess,
        memory_id: &StableMemoryId,
        expected_revision: u64,
        source: &SourceDraft,
        draft: &MemoryRevisionDraft,
        grounded: &GroundedKgFactSetDraft,
    ) -> Result<CognitiveWriteReceipt, CognitiveStoreError> {
        self.ensure_durable_fact_grounding_schema().await?;
        grounding::validate_source_binding(source, &draft.scope, &draft.content)?;
        grounding::require_groundable_revision(draft, grounded)?;
        let prepared = grounding::prepare(source, grounded)?;

        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(unavailable)?;
        let citation = self
            .append_source_tx(&mut transaction, access, source)
            .await?;
        let mut bound_draft = draft.clone();
        grounding::bind_exact_citation(&mut bound_draft, &citation)?;
        let memory = self
            .revise_memory_revision_tx(
                &mut transaction,
                access,
                memory_id,
                expected_revision,
                &bound_draft,
            )
            .await?;
        let canonical = self.canonicalize_fact_set(
            &memory,
            &citation,
            &grounded.facts,
            STRUCTURED_EXTRACTOR_CONTRACT,
        )?;
        grounding::validate_canonical_identity_binding(&prepared, &canonical)?;
        self.insert_revision_facts_tx(&mut transaction, &memory, &citation, &canonical)
            .await?;
        grounding::insert_tx(
            &mut transaction,
            &memory,
            &citation,
            &canonical,
            &prepared,
        )
        .await?;
        let projection = self
            .refresh_scope_projection_tx(
                &mut transaction,
                &memory.scope,
                &memory,
                &citation,
                &canonical,
            )
            .await?;
        transaction.commit().await.map_err(unavailable)?;
        Ok(CognitiveWriteReceipt {
            memory,
            source: citation,
            projection,
        })
    }

    /// Returns `grounded_v1`, `legacy_unreviewed`, or `zero_fact` for one
    /// immutable revision. This method does not activate a projection gate.
    pub async fn durable_fact_grounding_status(
        &self,
        memory_id: &StableMemoryId,
        revision: u64,
    ) -> Result<String, CognitiveStoreError> {
        self.ensure_durable_fact_grounding_schema().await?;
        let revision = grounding::to_i64(revision, "memory revision")?;
        let row = sqlx::query(
            "SELECT f.entity_count, f.relation_count,
                    EXISTS(
                        SELECT 1
                        FROM kg_revision_fact_grounding_receipts AS g
                        WHERE g.memory_id = f.memory_id
                          AND g.memory_revision = f.memory_revision
                    ) AS grounded
             FROM kg_revision_fact_sets AS f
             WHERE f.memory_id = ? AND f.memory_revision = ?",
        )
        .bind(memory_id.as_str())
        .bind(revision)
        .fetch_optional(&self.pool)
        .await
        .map_err(unavailable)?
        .ok_or_else(|| {
            CognitiveStoreError::Invalid(
                "memory revision has no immutable KG fact-set receipt".to_string(),
            )
        })?;
        let entity_count: i64 = row.try_get("entity_count").map_err(unavailable)?;
        let relation_count: i64 = row.try_get("relation_count").map_err(unavailable)?;
        let grounded: bool = row.try_get("grounded").map_err(unavailable)?;
        Ok(if entity_count + relation_count == 0 {
            "zero_fact"
        } else if grounded {
            "grounded_v1"
        } else {
            "legacy_unreviewed"
        }
        .to_string())
    }

    /// Returns the immutable durable grounding receipt digest, when present.
    pub async fn durable_fact_grounding_receipt_digest(
        &self,
        memory_id: &StableMemoryId,
        revision: u64,
    ) -> Result<Option<Sha256Digest>, CognitiveStoreError> {
        self.ensure_durable_fact_grounding_schema().await?;
        let digest = sqlx::query_scalar::<_, String>(
            "SELECT receipt_sha256
             FROM kg_revision_fact_grounding_receipts
             WHERE memory_id = ? AND memory_revision = ?",
        )
        .bind(memory_id.as_str())
        .bind(grounding::to_i64(revision, "memory revision")?)
        .fetch_optional(&self.pool)
        .await
        .map_err(unavailable)?;
        digest
            .map(Sha256Digest::parse)
            .transpose()
            .map_err(CognitiveStoreError::Corrupt)
    }
}
