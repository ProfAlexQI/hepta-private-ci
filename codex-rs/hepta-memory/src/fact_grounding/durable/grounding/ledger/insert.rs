use super::*;
use super::support::durable_receipt_digest;
use super::support::to_i64_len;

pub(in super::super::super) async fn insert_tx(
    transaction: &mut Transaction<'_, Sqlite>,
    memory: &MemoryRevisionRecord,
    source: &SourceRevisionId,
    canonical: &CanonicalFactSet,
    prepared: &PreparedGrounding,
) -> Result<(), CognitiveStoreError> {
    let revision = to_i64(memory.id.revision, "memory revision")?;
    let source_revision = to_i64(source.revision, "source revision")?;
    let evidence_count = to_i64_len(prepared.spans.len(), "grounding evidence count")?;
    let receipt_sha256 = durable_receipt_digest(DurableReceiptDigestParts {
        memory_id: memory.id.memory_id.as_str(),
        memory_revision: memory.id.revision,
        source_id: source.source_id.as_str(),
        source_revision: source.revision,
        source_content_sha256: prepared.source_content_sha256.as_str(),
        fact_set_sha256: canonical.digest.as_str(),
        fact_identity_sha256: prepared.fact_identity_sha256.as_str(),
        spans: &prepared.spans,
    });

    sqlx::query(
        "INSERT INTO kg_revision_fact_grounding_receipts (
            memory_id, memory_revision, grounding_contract,
            source_id, source_revision, source_content_sha256,
            fact_set_sha256, fact_identity_sha256, evidence_count,
            receipt_sha256, recorded_at_unix_seconds
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, unixepoch())",
    )
    .bind(memory.id.memory_id.as_str())
    .bind(revision)
    .bind(GROUNDING_CONTRACT)
    .bind(source.source_id.as_str())
    .bind(source_revision)
    .bind(prepared.source_content_sha256.as_str())
    .bind(canonical.digest.as_str())
    .bind(prepared.fact_identity_sha256.as_str())
    .bind(evidence_count)
    .bind(receipt_sha256.as_str())
    .execute(&mut **transaction)
    .await
    .map_err(unavailable)?;

    for span in &prepared.spans {
        sqlx::query(
            "INSERT INTO kg_revision_fact_grounding_spans (
                memory_id, memory_revision, fact_kind, fact_key,
                evidence_ordinal, start_byte, end_byte, evidence_sha256
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(memory.id.memory_id.as_str())
        .bind(revision)
        .bind(span.identity.kind.as_str())
        .bind(&span.identity.key)
        .bind(i64::from(span.ordinal))
        .bind(i64::from(span.start_byte))
        .bind(i64::from(span.end_byte))
        .bind(span.evidence_sha256.as_str())
        .execute(&mut **transaction)
        .await
        .map_err(unavailable)?;
    }

    let stored_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM kg_revision_fact_grounding_spans
         WHERE memory_id = ? AND memory_revision = ?",
    )
    .bind(memory.id.memory_id.as_str())
    .bind(revision)
    .fetch_one(&mut **transaction)
    .await
    .map_err(unavailable)?;
    if stored_count != evidence_count {
        return Err(CognitiveStoreError::Corrupt(
            "durable grounding receipt does not match its spans".to_string(),
        ));
    }
    Ok(())
}
