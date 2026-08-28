use std::collections::BTreeSet;

use super::*;
use super::super::grounding_v3::EvidenceSpanV3;
use super::super::grounding_v3::GroundedEntityV3;
use super::super::grounding_v3::GroundedRelationV3;
use super::super::grounding_v3::GroundedToolV3Input;
use super::super::grounding_v3::prepare_grounded_tool_v3;
use super::support::evidence_resolution_receipt_digest;
use super::support::resolve_exact_quote;
use super::support::segment_catalog_digest;
use super::support::source_segment_id;
use super::support::validate_segment_id;
use super::support::validate_source_range;

impl<'a> HostEvidenceResolverV1<'a> {
    pub(crate) fn new(
        source_content: &'a str,
        segment_drafts: &[SourceSegmentDraftV1],
    ) -> Result<Self, String> {
        if !(1..=MAX_SOURCE_BYTES).contains(&source_content.len()) {
            return Err(format!(
                "host evidence source must contain 1..={MAX_SOURCE_BYTES} UTF-8 bytes"
            ));
        }
        if segment_drafts.len() > MAX_SOURCE_SEGMENTS {
            return Err(format!(
                "host evidence source exceeds {MAX_SOURCE_SEGMENTS} segments"
            ));
        }

        let source_content_sha256 = Sha256Digest::for_bytes(source_content.as_bytes());
        let mut ranges = segment_drafts.to_vec();
        ranges.sort_by_key(|draft| (draft.start_byte, draft.end_byte));
        let mut seen_ranges = BTreeSet::new();
        let mut segments = Vec::with_capacity(ranges.len());
        let mut segment_index = BTreeMap::new();
        for (ordinal, draft) in ranges.into_iter().enumerate() {
            let (start, end) = validate_source_range(
                source_content,
                draft.start_byte,
                draft.end_byte,
                "source segment",
            )?;
            if end - start > MAX_SEGMENT_BYTES {
                return Err(format!(
                    "host evidence source segment exceeds {MAX_SEGMENT_BYTES} bytes"
                ));
            }
            if !seen_ranges.insert((draft.start_byte, draft.end_byte)) {
                return Err("host evidence source contains a duplicate segment range".to_string());
            }
            let evidence_sha256 =
                Sha256Digest::for_bytes(&source_content.as_bytes()[start..end]);
            let segment_id = source_segment_id(
                &source_content_sha256,
                draft.start_byte,
                draft.end_byte,
                &evidence_sha256,
            );
            let descriptor = SourceSegmentDescriptorV1 {
                segment_id: segment_id.clone(),
                ordinal: u32::try_from(ordinal)
                    .map_err(|_| "source segment ordinal exceeds u32".to_string())?,
                start_byte: draft.start_byte,
                end_byte: draft.end_byte,
                evidence_sha256,
            };
            let next_index = segments.len();
            if segment_index.insert(segment_id, next_index).is_some() {
                return Err("host evidence source produced a duplicate segment ID".to_string());
            }
            segments.push(descriptor);
        }
        let segment_catalog_sha256 =
            segment_catalog_digest(&source_content_sha256, &segments);
        Ok(Self {
            source_content,
            source_content_sha256,
            segment_catalog_sha256,
            segments,
            segment_index,
        })
    }

    pub(crate) fn source_content_sha256(&self) -> &Sha256Digest {
        &self.source_content_sha256
    }

    pub(crate) fn segment_catalog_sha256(&self) -> &Sha256Digest {
        &self.segment_catalog_sha256
    }

    pub(crate) fn segment_descriptors(&self) -> &[SourceSegmentDescriptorV1] {
        &self.segments
    }

    pub(crate) fn prepare_grounded_tool_v4(
        &self,
        input: GroundedToolV4Input,
    ) -> Result<HostResolvedGroundingV4, String> {
        if input.entities.len() > MAX_ENTITIES || input.relations.len() > MAX_RELATIONS {
            return Err("grounded v4 input exceeds entity or relation limits".to_string());
        }
        let total_selectors = input
            .entities
            .iter()
            .map(|entity| entity.evidence.len())
            .chain(
                input
                    .relations
                    .iter()
                    .map(|relation| relation.evidence.len()),
            )
            .try_fold(0usize, |total, count| total.checked_add(count))
            .ok_or_else(|| "grounded v4 selector count overflow".to_string())?;
        if total_selectors > MAX_TOTAL_SPANS {
            return Err("grounded v4 input exceeds total evidence limit".to_string());
        }
        let tool_input_json = serde_json::to_vec(&input)
            .map_err(|error| format!("grounded v4 input serialization failed: {error}"))?;
        let tool_input_sha256 = super::super::domain_digest(
            b"hepta:cognitive:grounded-tool-v4-input:v1",
            &tool_input_json,
        );

        let mut seen_entities = BTreeSet::new();
        let mut seen_relations = BTreeSet::new();
        let mut v3_entities = Vec::with_capacity(input.entities.len());
        let mut v3_relations = Vec::with_capacity(input.relations.len());
        let mut bindings = Vec::with_capacity(total_selectors);

        for entity in input.entities {
            if !seen_entities.insert(entity.key.clone()) {
                let entity_key = entity.key.as_str();
                return Err(format!(
                    "grounded v4 input contains duplicate entity key `{entity_key}`"
                ));
            }
            let resolved = self.resolve_fact_evidence(
                GroundedFactKind::Entity,
                entity.key.as_str(),
                entity.evidence,
            )?;
            bindings.extend(resolved.bindings);
            v3_entities.push(GroundedEntityV3 {
                key: entity.key,
                entity_type: entity.entity_type,
                label: entity.label,
                evidence: resolved.spans,
            });
        }
        for relation in input.relations {
            if !seen_relations.insert(relation.key.clone()) {
                let relation_key = relation.key.as_str();
                return Err(format!(
                    "grounded v4 input contains duplicate relation key `{relation_key}`"
                ));
            }
            if !seen_entities.contains(&relation.from_entity_key)
                || !seen_entities.contains(&relation.to_entity_key)
            {
                let relation_key = relation.key.as_str();
                return Err(format!(
                    "grounded v4 relation `{relation_key}` references an unknown entity key"
                ));
            }
            let resolved = self.resolve_fact_evidence(
                GroundedFactKind::Relation,
                relation.key.as_str(),
                relation.evidence,
            )?;
            bindings.extend(resolved.bindings);
            v3_relations.push(GroundedRelationV3 {
                key: relation.key,
                from_entity_key: relation.from_entity_key,
                to_entity_key: relation.to_entity_key,
                relation: relation.relation,
                evidence: resolved.spans,
            });
        }

        let grounded = prepare_grounded_tool_v3(
            self.source_content,
            GroundedToolV3Input {
                entities: v3_entities,
                relations: v3_relations,
            },
        )?;
        let resolution = self.resolution_receipt(bindings, tool_input_sha256)?;
        Ok(HostResolvedGroundingV4 {
            grounded,
            resolution,
        })
    }

    fn resolve_fact_evidence(
        &self,
        fact_kind: GroundedFactKind,
        fact_key: &str,
        selectors: Vec<EvidenceLocatorV4>,
    ) -> Result<ResolvedFactEvidence, String> {
        if selectors.is_empty() || selectors.len() > MAX_SPANS_PER_FACT {
            let kind = fact_kind.as_str();
            return Err(format!(
                "{kind} fact `{fact_key}` must contain 1..={MAX_SPANS_PER_FACT} evidence selectors"
            ));
        }
        let mut candidates = Vec::with_capacity(selectors.len());
        for selector in selectors {
            candidates.push(self.resolve_selector(fact_kind, fact_key, selector)?);
        }
        candidates.sort_by_key(|candidate| {
            (
                candidate.start_byte,
                candidate.end_byte,
                candidate.locator_kind,
            )
        });
        for pair in candidates.windows(2) {
            let previous = &pair[0];
            let current = &pair[1];
            if previous.start_byte == current.start_byte
                && previous.end_byte == current.end_byte
            {
                let kind = fact_kind.as_str();
                return Err(format!(
                    "{kind} fact `{fact_key}` resolves duplicate evidence ranges"
                ));
            }
            if current.start_byte < previous.end_byte {
                let kind = fact_kind.as_str();
                return Err(format!(
                    "{kind} fact `{fact_key}` resolves overlapping evidence ranges"
                ));
            }
        }

        let mut spans = Vec::with_capacity(candidates.len());
        let mut bindings = Vec::with_capacity(candidates.len());
        for (ordinal, candidate) in candidates.into_iter().enumerate() {
            let evidence_ordinal = u32::try_from(ordinal)
                .map_err(|_| "resolved evidence ordinal exceeds u32".to_string())?;
            spans.push(EvidenceSpanV3 {
                start_byte: candidate.start_byte,
                end_byte: candidate.end_byte,
                sha256: candidate.evidence_sha256.as_str().to_string(),
            });
            bindings.push(ResolvedEvidenceBindingV1 {
                fact_kind,
                fact_key: fact_key.to_string(),
                evidence_ordinal,
                locator_kind: candidate.locator_kind,
                locator_sha256: candidate.locator_sha256,
                start_byte: candidate.start_byte,
                end_byte: candidate.end_byte,
                evidence_sha256: candidate.evidence_sha256,
            });
        }
        Ok(ResolvedFactEvidence { spans, bindings })
    }

    fn resolve_selector(
        &self,
        fact_kind: GroundedFactKind,
        fact_key: &str,
        selector: EvidenceLocatorV4,
    ) -> Result<ResolvedCandidate, String> {
        match selector {
            EvidenceLocatorV4::ExactQuote(locator) => {
                let (start_byte, end_byte) = resolve_exact_quote(
                    self.source_content,
                    locator.quote.as_str(),
                    locator.occurrence,
                )?;
                let start = usize::try_from(start_byte)
                    .map_err(|_| "resolved quote start exceeds usize".to_string())?;
                let end = usize::try_from(end_byte)
                    .map_err(|_| "resolved quote end exceeds usize".to_string())?;
                let evidence_sha256 =
                    Sha256Digest::for_bytes(&self.source_content.as_bytes()[start..end]);
                let occurrence = locator.occurrence.to_be_bytes();
                let locator_sha256 = super::super::digest_many(
                    b"hepta:cognitive:host-evidence:exact-quote:v1",
                    &[locator.quote.as_bytes(), &occurrence],
                );
                Ok(ResolvedCandidate {
                    locator_kind: ResolvedLocatorKindV1::ExactQuote,
                    locator_sha256,
                    start_byte,
                    end_byte,
                    evidence_sha256,
                })
            }
            EvidenceLocatorV4::SourceSegment(locator) => {
                validate_segment_id(locator.segment_id.as_str())?;
                let index = self
                    .segment_index
                    .get(locator.segment_id.as_str())
                    .copied()
                    .ok_or_else(|| {
                        let kind = fact_kind.as_str();
                        format!(
                            "{kind} fact `{fact_key}` references an unknown source segment"
                        )
                    })?;
                let segment = self.segments.get(index).ok_or_else(|| {
                    "host evidence segment index is corrupt".to_string()
                })?;
                let locator_sha256 = super::super::domain_digest(
                    b"hepta:cognitive:host-evidence:segment-locator:v1",
                    locator.segment_id.as_bytes(),
                );
                Ok(ResolvedCandidate {
                    locator_kind: ResolvedLocatorKindV1::SourceSegment,
                    locator_sha256,
                    start_byte: segment.start_byte,
                    end_byte: segment.end_byte,
                    evidence_sha256: segment.evidence_sha256.clone(),
                })
            }
        }
    }

    fn resolution_receipt(
        &self,
        mut bindings: Vec<ResolvedEvidenceBindingV1>,
        tool_input_sha256: Sha256Digest,
    ) -> Result<HostEvidenceResolutionReceiptV1, String> {
        bindings.sort_by(|left, right| {
            (
                left.fact_kind,
                left.fact_key.as_str(),
                left.evidence_ordinal,
                left.start_byte,
                left.end_byte,
                left.locator_kind,
            )
                .cmp(&(
                    right.fact_kind,
                    right.fact_key.as_str(),
                    right.evidence_ordinal,
                    right.start_byte,
                    right.end_byte,
                    right.locator_kind,
                ))
        });
        let count = u32::try_from(bindings.len())
            .map_err(|_| "resolved evidence count exceeds u32".to_string())?;
        let segment_count = u32::try_from(self.segments.len())
            .map_err(|_| "source segment count exceeds u32".to_string())?;
        let mut receipt = HostEvidenceResolutionReceiptV1 {
            schema_version: HOST_EVIDENCE_RESOLVER_SCHEMA_VERSION,
            tool_schema_version: GROUNDED_TOOL_V4_SCHEMA_VERSION,
            contract: HOST_EVIDENCE_RESOLVER_CONTRACT.to_string(),
            source_content_sha256: self.source_content_sha256.clone(),
            tool_input_sha256,
            segment_catalog_sha256: self.segment_catalog_sha256.clone(),
            segment_count,
            selector_count: count,
            resolved_span_count: count,
            resolved_spans: bindings,
            model_supplied_byte_offsets: MODEL_SUPPLIED_BYTE_OFFSETS,
            model_supplied_digests: MODEL_SUPPLIED_DIGESTS,
            host_resolved_byte_offsets: HOST_RESOLVED_BYTE_OFFSETS,
            host_resolved_digests: HOST_RESOLVED_DIGESTS,
            tool_registered: GROUNDED_TOOL_V4_REGISTERED,
            production_authority: GROUNDED_TOOL_V4_PRODUCTION_AUTHORITY,
            external_effects: GROUNDED_TOOL_V4_EXTERNAL_EFFECTS,
            operator_acceptance: GROUNDED_TOOL_V4_OPERATOR_ACCEPTANCE,
            promotion: GROUNDED_TOOL_V4_PROMOTION,
            callers_ratchet: GROUNDED_TOOL_V4_CALLERS_RATCHET,
            receipt_sha256: Sha256Digest::for_bytes(b"uncomputed"),
        };
        receipt.receipt_sha256 = evidence_resolution_receipt_digest(&receipt);
        receipt.validate()?;
        Ok(receipt)
    }
}

pub(crate) fn prepare_grounded_tool_v4(
    source_content: &str,
    segment_drafts: &[SourceSegmentDraftV1],
    input: GroundedToolV4Input,
) -> Result<HostResolvedGroundingV4, String> {
    HostEvidenceResolverV1::new(source_content, segment_drafts)?
        .prepare_grounded_tool_v4(input)
}

struct ResolvedFactEvidence {
    spans: Vec<EvidenceSpanV3>,
    bindings: Vec<ResolvedEvidenceBindingV1>,
}

struct ResolvedCandidate {
    locator_kind: ResolvedLocatorKindV1,
    locator_sha256: Sha256Digest,
    start_byte: u32,
    end_byte: u32,
    evidence_sha256: Sha256Digest,
}
