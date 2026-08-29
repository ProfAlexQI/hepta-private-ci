use super::*;
use crate::framing::frame_part;

pub(super) async fn stored_fact_supports(
    pool: &SqlitePool,
    memory_id: &str,
    memory_revision: i64,
    declared_entity_count: i64,
    declared_relation_count: i64,
) -> Result<BTreeMap<FactIdentity, FactSupport>, CognitiveStoreError> {
    let entity_rows = sqlx::query(
        "SELECT entity_key, label
         FROM kg_revision_entities
         WHERE memory_id = ? AND memory_revision = ?
         ORDER BY entity_key",
    )
    .bind(memory_id)
    .bind(memory_revision)
    .fetch_all(pool)
    .await
    .map_err(unavailable)?;
    if to_i64_len(entity_rows.len(), "stored entity count")? != declared_entity_count {
        return Err(CognitiveStoreError::Corrupt(
            "durable grounding entity count differs from the fact-set receipt".to_string(),
        ));
    }
    let mut supports = BTreeMap::new();
    let mut labels = BTreeMap::<String, String>::new();
    for row in entity_rows {
        let key: String = row.try_get("entity_key").map_err(unavailable)?;
        let label: String = row.try_get("label").map_err(unavailable)?;
        labels.insert(key.clone(), label.clone());
        supports.insert(
            FactIdentity {
                kind: GroundedFactKind::Entity,
                key,
            },
            FactSupport::Entity { label },
        );
    }

    let relation_rows = sqlx::query(
        "SELECT relation_key, from_entity_key, to_entity_key, relation
         FROM kg_revision_relations
         WHERE memory_id = ? AND memory_revision = ?
         ORDER BY relation_key",
    )
    .bind(memory_id)
    .bind(memory_revision)
    .fetch_all(pool)
    .await
    .map_err(unavailable)?;
    if to_i64_len(relation_rows.len(), "stored relation count")? != declared_relation_count {
        return Err(CognitiveStoreError::Corrupt(
            "durable grounding relation count differs from the fact-set receipt".to_string(),
        ));
    }
    for row in relation_rows {
        let key: String = row.try_get("relation_key").map_err(unavailable)?;
        let from_key: String = row.try_get("from_entity_key").map_err(unavailable)?;
        let to_key: String = row.try_get("to_entity_key").map_err(unavailable)?;
        let relation: String = row.try_get("relation").map_err(unavailable)?;
        let from_label = labels.get(&from_key).cloned().ok_or_else(|| {
            CognitiveStoreError::Corrupt("stored relation has no source entity label".to_string())
        })?;
        let to_label = labels.get(&to_key).cloned().ok_or_else(|| {
            CognitiveStoreError::Corrupt("stored relation has no target entity label".to_string())
        })?;
        supports.insert(
            FactIdentity {
                kind: GroundedFactKind::Relation,
                key,
            },
            FactSupport::Relation {
                from_label,
                to_label,
                relation,
            },
        );
    }
    Ok(supports)
}

pub(super) fn durable_receipt_digest(parts: DurableReceiptDigestParts<'_>) -> Sha256Digest {
    let DurableReceiptDigestParts {
        memory_id,
        memory_revision,
        source_id,
        source_revision,
        source_content_sha256,
        fact_set_sha256,
        fact_identity_sha256,
        spans,
    } = parts;
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:cognitive:durable-fact-grounding-receipt:v1",
    );
    frame_part(&mut hasher, memory_id.as_bytes());
    frame_part(&mut hasher, &memory_revision.to_be_bytes());
    frame_part(&mut hasher, source_id.as_bytes());
    frame_part(&mut hasher, &source_revision.to_be_bytes());
    frame_part(&mut hasher, GROUNDING_CONTRACT.as_bytes());
    frame_part(&mut hasher, source_content_sha256.as_bytes());
    frame_part(&mut hasher, fact_set_sha256.as_bytes());
    frame_part(&mut hasher, fact_identity_sha256.as_bytes());
    frame_part(
        &mut hasher,
        &u64::try_from(spans.len()).unwrap_or(u64::MAX).to_be_bytes(),
    );
    for span in spans {
        frame_part(&mut hasher, span.identity.kind.as_str().as_bytes());
        frame_part(&mut hasher, span.identity.key.as_bytes());
        frame_part(&mut hasher, &span.ordinal.to_be_bytes());
        frame_part(&mut hasher, &span.start_byte.to_be_bytes());
        frame_part(&mut hasher, &span.end_byte.to_be_bytes());
        frame_part(&mut hasher, span.evidence_sha256.as_str().as_bytes());
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

pub(super) fn parse_fact_kind(value: &str) -> Result<GroundedFactKind, CognitiveStoreError> {
    match value {
        "entity" => Ok(GroundedFactKind::Entity),
        "relation" => Ok(GroundedFactKind::Relation),
        _ => Err(CognitiveStoreError::Corrupt(
            "invalid durable grounding fact kind".to_string(),
        )),
    }
}

pub(super) fn validate_span_range_corrupt(
    source_text: &str,
    start: usize,
    end: usize,
) -> Result<(), CognitiveStoreError> {
    if start >= end || end > source_text.len() {
        return Err(CognitiveStoreError::Corrupt(
            "durable grounding span is outside source bytes".to_string(),
        ));
    }
    if !source_text.is_char_boundary(start) || !source_text.is_char_boundary(end) {
        return Err(CognitiveStoreError::Corrupt(
            "durable grounding span splits a UTF-8 character".to_string(),
        ));
    }
    Ok(())
}

pub(super) fn to_i64_len(value: usize, label: &str) -> Result<i64, CognitiveStoreError> {
    i64::try_from(value).map_err(|_| CognitiveStoreError::Invalid(format!("{label} exceeds i64")))
}

pub(super) fn limit_plus_one(value: usize) -> Result<i64, CognitiveStoreError> {
    value
        .checked_add(1)
        .and_then(|next| i64::try_from(next).ok())
        .ok_or_else(|| {
            CognitiveStoreError::Corrupt(
                "durable grounding verification limit exceeds i64".to_string(),
            )
        })
}
