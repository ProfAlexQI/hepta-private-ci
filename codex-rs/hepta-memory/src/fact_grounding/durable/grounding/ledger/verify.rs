use super::*;

pub(super) async fn verify_receipts(
    pool: &SqlitePool,
    owner_agent_id: &str,
) -> Result<(), CognitiveStoreError> {
    let rows = sqlx::query(
        "SELECT g.memory_id, g.memory_revision, g.grounding_contract,
                g.source_id, g.source_revision, g.source_content_sha256,
                g.fact_set_sha256, g.fact_identity_sha256,
                g.evidence_count, g.receipt_sha256,
                m.owner_agent_id, s.content, s.content_sha256 AS ledger_source_sha256,
                f.entity_count, f.relation_count
         FROM kg_revision_fact_grounding_receipts AS g
         JOIN memory_revisions AS m
           ON m.memory_id = g.memory_id AND m.revision = g.memory_revision
         JOIN source_ledger AS s
           ON s.source_id = g.source_id AND s.source_revision = g.source_revision
         JOIN kg_revision_fact_sets AS f
           ON f.memory_id = g.memory_id
          AND f.memory_revision = g.memory_revision
          AND f.fact_set_sha256 = g.fact_set_sha256
         ORDER BY g.memory_id, g.memory_revision
         LIMIT ?",
    )
    .bind(limit_plus_one(MAX_GROUNDING_RECEIPTS)?)
    .fetch_all(pool)
    .await
    .map_err(unavailable)?;
    if rows.len() > MAX_GROUNDING_RECEIPTS {
        return Err(CognitiveStoreError::Corrupt(format!(
            "fact-grounding ledger exceeds {MAX_GROUNDING_RECEIPTS} receipts"
        )));
    }

    for row in rows {
        let memory_id: String = row.try_get("memory_id").map_err(unavailable)?;
        let memory_revision_i64: i64 =
            row.try_get("memory_revision").map_err(unavailable)?;
        let memory_revision = u64::try_from(memory_revision_i64).map_err(|_| {
            CognitiveStoreError::Corrupt(
                "negative fact-grounding memory revision".to_string(),
            )
        })?;
        let stored_owner: String = row.try_get("owner_agent_id").map_err(unavailable)?;
        if stored_owner != owner_agent_id {
            return Err(CognitiveStoreError::Corrupt(
                "fact-grounding receipt belongs to a foreign agent".to_string(),
            ));
        }
        let contract: String = row.try_get("grounding_contract").map_err(unavailable)?;
        if contract != GROUNDING_CONTRACT {
            return Err(CognitiveStoreError::Corrupt(
                "unsupported durable fact-grounding contract".to_string(),
            ));
        }
        let source_id: String = row.try_get("source_id").map_err(unavailable)?;
        let source_revision_i64: i64 =
            row.try_get("source_revision").map_err(unavailable)?;
        let source_revision = u64::try_from(source_revision_i64).map_err(|_| {
            CognitiveStoreError::Corrupt(
                "negative fact-grounding source revision".to_string(),
            )
        })?;
        let source: Vec<u8> = row.try_get("content").map_err(unavailable)?;
        let source_text = str::from_utf8(&source).map_err(|_| {
            CognitiveStoreError::Corrupt(
                "durably grounded source is not valid UTF-8".to_string(),
            )
        })?;
        if source.len() > MAX_GROUNDING_SOURCE_BYTES {
            return Err(CognitiveStoreError::Corrupt(
                "durably grounded source exceeds the verification limit".to_string(),
            ));
        }
        let source_content_sha256 =
            Sha256Digest::for_bytes(&source);
        let stored_source_sha256: String =
            row.try_get("source_content_sha256").map_err(unavailable)?;
        let ledger_source_sha256: String =
            row.try_get("ledger_source_sha256").map_err(unavailable)?;
        if source_content_sha256.as_str() != stored_source_sha256
            || source_content_sha256.as_str() != ledger_source_sha256
        {
            return Err(CognitiveStoreError::Corrupt(
                "durable grounding source digest failed recomputation".to_string(),
            ));
        }
        let fact_set_sha256: String =
            row.try_get("fact_set_sha256").map_err(unavailable)?;
        Sha256Digest::parse(fact_set_sha256.clone())
            .map_err(CognitiveStoreError::Corrupt)?;
        let entity_count: i64 = row.try_get("entity_count").map_err(unavailable)?;
        let relation_count: i64 =
            row.try_get("relation_count").map_err(unavailable)?;
        if entity_count + relation_count <= 0 {
            return Err(CognitiveStoreError::Corrupt(
                "zero-fact revision has an unexpected grounding receipt".to_string(),
            ));
        }

        let supports = stored_fact_supports(
            pool,
            &memory_id,
            memory_revision_i64,
            entity_count,
            relation_count,
        )
        .await?;
        let span_rows = sqlx::query(
            "SELECT fact_kind, fact_key, evidence_ordinal,
                    start_byte, end_byte, evidence_sha256
             FROM kg_revision_fact_grounding_spans
             WHERE memory_id = ? AND memory_revision = ?
             ORDER BY fact_kind, fact_key, evidence_ordinal",
        )
        .bind(&memory_id)
        .bind(memory_revision_i64)
        .fetch_all(pool)
        .await
        .map_err(unavailable)?;
        let evidence_count_i64: i64 =
            row.try_get("evidence_count").map_err(unavailable)?;
        let evidence_count = usize::try_from(evidence_count_i64).map_err(|_| {
            CognitiveStoreError::Corrupt(
                "negative durable grounding evidence count".to_string(),
            )
        })?;
        if span_rows.len() != evidence_count || evidence_count > MAX_TOTAL_SPANS {
            return Err(CognitiveStoreError::Corrupt(
                "durable grounding span count does not match its receipt".to_string(),
            ));
        }

        let mut spans = Vec::with_capacity(span_rows.len());
        let mut per_fact = BTreeMap::<FactIdentity, usize>::new();
        let mut support_text = BTreeMap::<FactIdentity, String>::new();
        let mut seen = BTreeSet::new();
        for span_row in span_rows {
            let kind_text: String =
                span_row.try_get("fact_kind").map_err(unavailable)?;
            let kind = parse_fact_kind(&kind_text)?;
            let key: String = span_row.try_get("fact_key").map_err(unavailable)?;
            if canonical_token(&key, MAX_FACT_KEY_BYTES, "stored fact key")? != key {
                return Err(CognitiveStoreError::Corrupt(
                    "durable grounding fact key is not canonical".to_string(),
                ));
            }
            let identity = FactIdentity { kind, key };
            if !supports.contains_key(&identity) {
                return Err(CognitiveStoreError::Corrupt(format!(
                    "durable grounding references unknown {} fact `{}`",
                    identity.kind.as_str(),
                    identity.key
                )));
            }
            let ordinal_i64: i64 =
                span_row.try_get("evidence_ordinal").map_err(unavailable)?;
            let ordinal = u32::try_from(ordinal_i64).map_err(|_| {
                CognitiveStoreError::Corrupt(
                    "negative durable grounding evidence ordinal".to_string(),
                )
            })?;
            let expected_ordinal =
                u32::try_from(per_fact.get(&identity).copied().unwrap_or_default())
                    .unwrap_or(u32::MAX);
            if ordinal != expected_ordinal {
                return Err(CognitiveStoreError::Corrupt(
                    "durable grounding evidence ordinals are not contiguous".to_string(),
                ));
            }
            let start_i64: i64 = span_row.try_get("start_byte").map_err(unavailable)?;
            let end_i64: i64 = span_row.try_get("end_byte").map_err(unavailable)?;
            let start = usize::try_from(start_i64).map_err(|_| {
                CognitiveStoreError::Corrupt(
                    "negative durable grounding start byte".to_string(),
                )
            })?;
            let end = usize::try_from(end_i64).map_err(|_| {
                CognitiveStoreError::Corrupt(
                    "negative durable grounding end byte".to_string(),
                )
            })?;
            validate_span_range_corrupt(source_text, start, end)?;
            let stored_evidence_sha256: String =
                span_row.try_get("evidence_sha256").map_err(unavailable)?;
            let actual_evidence_sha256 = Sha256Digest::for_bytes(&source[start..end]);
            if actual_evidence_sha256.as_str() != stored_evidence_sha256 {
                return Err(CognitiveStoreError::Corrupt(format!(
                    "durable grounding evidence digest failed for {} fact `{}`",
                    identity.kind.as_str(),
                    identity.key
                )));
            }
            let duplicate = (
                identity.clone(),
                start_i64,
                end_i64,
                stored_evidence_sha256.clone(),
            );
            if !seen.insert(duplicate) {
                return Err(CognitiveStoreError::Corrupt(
                    "durable grounding contains a duplicate span".to_string(),
                ));
            }
            let count = per_fact.entry(identity.clone()).or_default();
            *count += 1;
            if *count > MAX_SPANS_PER_FACT {
                return Err(CognitiveStoreError::Corrupt(
                    "durable grounding exceeds the per-fact span limit".to_string(),
                ));
            }
            let normalized = semantic_normalize(&source_text[start..end]);
            if normalized.is_empty() {
                return Err(CognitiveStoreError::Corrupt(
                    "durable grounding evidence contains no semantic text".to_string(),
                ));
            }
            support_text
                .entry(identity.clone())
                .and_modify(|text| {
                    text.push(' ');
                    text.push_str(&normalized);
                })
                .or_insert(normalized);
            spans.push(PreparedSpan {
                identity,
                ordinal,
                start_byte: u32::try_from(start).map_err(|_| {
                    CognitiveStoreError::Corrupt(
                        "durable grounding start byte exceeds u32".to_string(),
                    )
                })?,
                end_byte: u32::try_from(end).map_err(|_| {
                    CognitiveStoreError::Corrupt(
                        "durable grounding end byte exceeds u32".to_string(),
                    )
                })?,
                evidence_sha256: actual_evidence_sha256,
            });
        }

        for (identity, support) in &supports {
            let count = per_fact.get(identity).copied().unwrap_or_default();
            if count == 0 || count > MAX_SPANS_PER_FACT {
                return Err(CognitiveStoreError::Corrupt(format!(
                    "{} fact `{}` has invalid durable evidence coverage",
                    identity.kind.as_str(),
                    identity.key
                )));
            }
            if !support_is_sufficient(
                support_text.get(identity).map(String::as_str).unwrap_or_default(),
                support,
            ) {
                return Err(CognitiveStoreError::Corrupt(format!(
                    "durable evidence no longer supports {} fact `{}`",
                    identity.kind.as_str(),
                    identity.key
                )));
            }
        }

        let expected_identity_sha256 = fact_identity_digest(supports.keys());
        let stored_identity_sha256: String =
            row.try_get("fact_identity_sha256").map_err(unavailable)?;
        if expected_identity_sha256.as_str() != stored_identity_sha256 {
            return Err(CognitiveStoreError::Corrupt(
                "durable grounding fact-identity digest failed recomputation".to_string(),
            ));
        }
        let expected_receipt_sha256 = durable_receipt_digest(
            DurableReceiptDigestParts {
                memory_id: &memory_id,
                memory_revision,
                source_id: &source_id,
                source_revision,
                source_content_sha256: source_content_sha256.as_str(),
                fact_set_sha256: &fact_set_sha256,
                fact_identity_sha256: expected_identity_sha256.as_str(),
                spans: &spans,
            },
        );
        let stored_receipt_sha256: String =
            row.try_get("receipt_sha256").map_err(unavailable)?;
        if expected_receipt_sha256.as_str() != stored_receipt_sha256 {
            return Err(CognitiveStoreError::Corrupt(
                "durable fact-grounding receipt digest failed recomputation".to_string(),
            ));
        }
    }
    Ok(())
}
