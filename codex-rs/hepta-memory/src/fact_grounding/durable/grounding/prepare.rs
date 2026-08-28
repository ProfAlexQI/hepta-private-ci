use super::*;

pub(super) fn validate_source_binding(
    source: &SourceDraft,
    scope: &crate::CognitiveScope,
    expected_content: &str,
) -> Result<(), CognitiveStoreError> {
    if &source.scope != scope {
        return Err(CognitiveStoreError::AccessDenied(
            "source and grounded memory revision must have the same scope".to_string(),
        ));
    }
    if source.content != expected_content.as_bytes() {
        return Err(CognitiveStoreError::Invalid(
            "source content must exactly bind the grounded memory input".to_string(),
        ));
    }
    Ok(())
}

pub(super) fn require_groundable_revision(
    revision: &MemoryRevisionDraft,
    grounded: &GroundedKgFactSetDraft,
) -> Result<(), CognitiveStoreError> {
    if revision.verification != MemoryVerification::Verified
        || revision.lifecycle != MemoryLifecycleState::Active
    {
        return Err(CognitiveStoreError::Invalid(
            "durable grounding requires a verified active memory revision".to_string(),
        ));
    }
    if grounded.facts.entities.is_empty() && grounded.facts.relations.is_empty() {
        return Err(CognitiveStoreError::Invalid(
            "zero-fact revisions do not require a durable grounding receipt".to_string(),
        ));
    }
    Ok(())
}

pub(super) fn bind_exact_citation(
    revision: &mut MemoryRevisionDraft,
    citation: &SourceRevisionId,
) -> Result<(), CognitiveStoreError> {
    if !revision.citations.is_empty() {
        return Err(CognitiveStoreError::Invalid(
            "durable grounded writer owns the exact source citation set".to_string(),
        ));
    }
    revision.citations.push(citation.clone());
    Ok(())
}

pub(super) fn prepare(
    source: &SourceDraft,
    grounded: &GroundedKgFactSetDraft,
) -> Result<PreparedGrounding, CognitiveStoreError> {
    if source.content.len() > MAX_GROUNDING_SOURCE_BYTES {
        return Err(CognitiveStoreError::Invalid(format!(
            "fact-grounding source exceeds {MAX_GROUNDING_SOURCE_BYTES} bytes"
        )));
    }
    let source_text = str::from_utf8(&source.content).map_err(|_| {
        CognitiveStoreError::Invalid("fact-grounding source must be valid UTF-8".to_string())
    })?;
    if grounded.evidence.len() > MAX_TOTAL_SPANS {
        return Err(CognitiveStoreError::Invalid(format!(
            "fact grounding exceeds {MAX_TOTAL_SPANS} evidence spans"
        )));
    }

    let supports = fact_supports(grounded)?;
    let mut seen = BTreeSet::new();
    let mut per_fact = BTreeMap::<FactIdentity, usize>::new();
    let mut support_text = BTreeMap::<FactIdentity, String>::new();
    let mut spans = Vec::with_capacity(grounded.evidence.len());

    for evidence in &grounded.evidence {
        let identity = FactIdentity {
            kind: evidence.fact_kind,
            key: canonical_token(
                &evidence.fact_key,
                MAX_FACT_KEY_BYTES,
                "evidence fact key",
            )?,
        };
        if !supports.contains_key(&identity) {
            return Err(CognitiveStoreError::Invalid(format!(
                "grounding evidence references unknown {} fact `{}`",
                identity.kind.as_str(),
                identity.key
            )));
        }
        let start = usize::try_from(evidence.start_byte).map_err(|_| {
            CognitiveStoreError::Invalid("evidence start byte exceeds usize".to_string())
        })?;
        let end = usize::try_from(evidence.end_byte).map_err(|_| {
            CognitiveStoreError::Invalid("evidence end byte exceeds usize".to_string())
        })?;
        validate_span_range(source_text, start, end)?;
        let actual = Sha256Digest::for_bytes(&source.content[start..end]);
        if actual != evidence.evidence_sha256 {
            return Err(CognitiveStoreError::Invalid(format!(
                "evidence digest mismatch for {} fact `{}`",
                identity.kind.as_str(),
                identity.key
            )));
        }
        let duplicate = (
            identity.clone(),
            evidence.start_byte,
            evidence.end_byte,
            evidence.evidence_sha256.as_str().to_string(),
        );
        if !seen.insert(duplicate) {
            return Err(CognitiveStoreError::Invalid(format!(
                "duplicate evidence span for {} fact `{}`",
                identity.kind.as_str(),
                identity.key
            )));
        }
        let count = per_fact.entry(identity.clone()).or_default();
        *count += 1;
        if *count > MAX_SPANS_PER_FACT {
            return Err(CognitiveStoreError::Invalid(format!(
                "{} fact `{}` exceeds {MAX_SPANS_PER_FACT} evidence spans",
                identity.kind.as_str(),
                identity.key
            )));
        }
        let normalized = semantic_normalize(&source_text[start..end]);
        if normalized.is_empty() {
            return Err(CognitiveStoreError::Invalid(
                "evidence span contains no semantic text".to_string(),
            ));
        }
        support_text
            .entry(identity.clone())
            .and_modify(|text| {
                text.push(' ');
                text.push_str(&normalized);
            })
            .or_insert(normalized);
        spans.push((
            identity,
            evidence.start_byte,
            evidence.end_byte,
            evidence.evidence_sha256.clone(),
        ));
    }

    for (identity, support) in &supports {
        let count = per_fact.get(identity).copied().unwrap_or_default();
        if count == 0 {
            return Err(CognitiveStoreError::Invalid(format!(
                "{} fact `{}` has no grounding evidence",
                identity.kind.as_str(),
                identity.key
            )));
        }
        if !support_is_sufficient(
            support_text.get(identity).map(String::as_str).unwrap_or_default(),
            support,
        ) {
            return Err(CognitiveStoreError::Invalid(format!(
                "evidence does not textually support {} fact `{}`",
                identity.kind.as_str(),
                identity.key
            )));
        }
    }

    spans.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
            .then_with(|| left.3.as_str().cmp(right.3.as_str()))
    });
    let mut ordinals = BTreeMap::<FactIdentity, u32>::new();
    let spans = spans
        .into_iter()
        .map(|(identity, start_byte, end_byte, evidence_sha256)| {
            let ordinal = ordinals.entry(identity.clone()).or_default();
            let prepared = PreparedSpan {
                identity,
                ordinal: *ordinal,
                start_byte,
                end_byte,
                evidence_sha256,
            };
            *ordinal = ordinal.checked_add(1).unwrap_or(u32::MAX);
            prepared
        })
        .collect::<Vec<_>>();

    Ok(PreparedGrounding {
        source_content_sha256: Sha256Digest::for_bytes(&source.content),
        fact_identity_sha256: fact_identity_digest(supports.keys()),
        spans,
    })
}

fn fact_supports(
    grounded: &GroundedKgFactSetDraft,
) -> Result<BTreeMap<FactIdentity, FactSupport>, CognitiveStoreError> {
    let mut supports = BTreeMap::new();
    let mut entity_labels = BTreeMap::<String, String>::new();
    for entity in &grounded.facts.entities {
        let key = canonical_token(&entity.key, MAX_FACT_KEY_BYTES, "entity key")?;
        let label = canonical_text(&entity.label, MAX_ENTITY_LABEL_BYTES, "entity label")?;
        require_semantic_text(&label, "entity label")?;
        if entity_labels.insert(key.clone(), label.clone()).is_some() {
            return Err(CognitiveStoreError::Invalid(format!(
                "duplicate grounded entity key `{key}`"
            )));
        }
        supports.insert(
            FactIdentity {
                kind: GroundedFactKind::Entity,
                key,
            },
            FactSupport::Entity { label },
        );
    }
    for relation in &grounded.facts.relations {
        let key = canonical_token(&relation.key, MAX_FACT_KEY_BYTES, "relation key")?;
        let from_key = canonical_token(
            &relation.from_entity_key,
            MAX_FACT_KEY_BYTES,
            "relation source key",
        )?;
        let to_key = canonical_token(
            &relation.to_entity_key,
            MAX_FACT_KEY_BYTES,
            "relation target key",
        )?;
        let predicate =
            canonical_token(&relation.relation, MAX_RELATION_BYTES, "relation predicate")?;
        require_semantic_text(&predicate, "relation predicate")?;
        let from_label = entity_labels.get(&from_key).cloned().ok_or_else(|| {
            CognitiveStoreError::Invalid(format!(
                "relation `{key}` references undeclared source entity `{from_key}`"
            ))
        })?;
        let to_label = entity_labels.get(&to_key).cloned().ok_or_else(|| {
            CognitiveStoreError::Invalid(format!(
                "relation `{key}` references undeclared target entity `{to_key}`"
            ))
        })?;
        if supports
            .insert(
                FactIdentity {
                    kind: GroundedFactKind::Relation,
                    key: key.clone(),
                },
                FactSupport::Relation {
                    from_label,
                    to_label,
                    relation: predicate,
                },
            )
            .is_some()
        {
            return Err(CognitiveStoreError::Invalid(format!(
                "duplicate grounded relation key `{key}`"
            )));
        }
    }
    Ok(supports)
}

pub(super) fn validate_canonical_identity_binding(
    prepared: &PreparedGrounding,
    canonical: &CanonicalFactSet,
) -> Result<(), CognitiveStoreError> {
    let identities = canonical
        .entities
        .iter()
        .map(|entity| FactIdentity {
            kind: GroundedFactKind::Entity,
            key: entity.key.clone(),
        })
        .chain(canonical.relations.iter().map(|relation| FactIdentity {
            kind: GroundedFactKind::Relation,
            key: relation.key.clone(),
        }))
        .collect::<BTreeSet<_>>();
    let digest = fact_identity_digest(identities.iter());
    if digest != prepared.fact_identity_sha256 {
        return Err(CognitiveStoreError::Corrupt(
            "grounding identities do not match canonical KG facts".to_string(),
        ));
    }
    Ok(())
}
