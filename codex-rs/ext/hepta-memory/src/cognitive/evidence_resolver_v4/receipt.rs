use super::*;
use super::support::evidence_resolution_receipt_digest;

impl HostEvidenceResolutionReceiptV1 {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.schema_version != HOST_EVIDENCE_RESOLVER_SCHEMA_VERSION
            || self.tool_schema_version != GROUNDED_TOOL_V4_SCHEMA_VERSION
            || self.contract != HOST_EVIDENCE_RESOLVER_CONTRACT
        {
            return Err("unsupported host evidence resolver receipt".to_string());
        }
        if self.model_supplied_byte_offsets
            || self.model_supplied_digests
            || !self.host_resolved_byte_offsets
            || !self.host_resolved_digests
        {
            return Err("host evidence resolver ownership flags are invalid".to_string());
        }
        if self.tool_registered
            || self.production_authority
            || self.external_effects
            || self.operator_acceptance
            || self.promotion
            || self.callers_ratchet
        {
            return Err("host evidence resolver crossed its authority boundary".to_string());
        }
        if self.selector_count == 0
            || usize::try_from(self.selector_count).ok() != Some(self.resolved_spans.len())
            || self.selector_count != self.resolved_span_count
            || self.resolved_spans.len() > MAX_TOTAL_SPANS
            || usize::try_from(self.segment_count)
                .ok()
                .is_none_or(|count| count > MAX_SOURCE_SEGMENTS)
        {
            return Err("host evidence resolver receipt count mismatch".to_string());
        }

        let mut previous: Option<(GroundedFactKind, &str, u32, u32, usize)> = None;
        for span in &self.resolved_spans {
            if span.fact_key.trim().is_empty()
                || span.fact_key.len() > MAX_KEY_BYTES
                || span.fact_key.as_bytes().contains(&0)
                || span.start_byte >= span.end_byte
            {
                return Err("host evidence resolver receipt contains an invalid span".to_string());
            }
            let identity = (span.fact_kind, span.fact_key.as_str());
            let per_fact_count = match previous {
                None => {
                    if span.evidence_ordinal != 0 {
                        return Err(
                            "host evidence resolver first evidence ordinal must be zero"
                                .to_string(),
                        );
                    }
                    1
                }
                Some((previous_kind, previous_key, previous_ordinal, previous_end, count)) => {
                    let previous_identity = (previous_kind, previous_key);
                    if previous_identity == identity {
                        let expected_ordinal =
                            previous_ordinal.checked_add(1).ok_or_else(|| {
                                "host evidence resolver evidence ordinal overflow".to_string()
                            })?;
                        if span.evidence_ordinal != expected_ordinal {
                            return Err(
                                "host evidence resolver evidence ordinals are not contiguous"
                                    .to_string(),
                            );
                        }
                        if span.start_byte < previous_end {
                            return Err(
                                "host evidence resolver receipt contains overlapping spans"
                                    .to_string(),
                            );
                        }
                        count.checked_add(1).ok_or_else(|| {
                            "host evidence resolver per-fact count overflow".to_string()
                        })?
                    } else {
                        if identity < previous_identity || span.evidence_ordinal != 0 {
                            return Err(
                                "host evidence resolver evidence ordering is invalid".to_string(),
                            );
                        }
                        1
                    }
                }
            };
            if per_fact_count > MAX_SPANS_PER_FACT {
                return Err(
                    "host evidence resolver receipt exceeds the per-fact span limit"
                        .to_string(),
                );
            }
            if span.locator_kind == ResolvedLocatorKindV1::SourceSegment
                && self.segment_count == 0
            {
                return Err(
                    "host evidence resolver segment binding has no segment catalog"
                        .to_string(),
                );
            }
            previous = Some((
                span.fact_kind,
                span.fact_key.as_str(),
                span.evidence_ordinal,
                span.end_byte,
                per_fact_count,
            ));
        }

        if self.receipt_sha256 != evidence_resolution_receipt_digest(self) {
            return Err("host evidence resolver receipt digest mismatch".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding(
        ordinal: u32,
        start_byte: u32,
        end_byte: u32,
        locator_kind: ResolvedLocatorKindV1,
    ) -> ResolvedEvidenceBindingV1 {
        ResolvedEvidenceBindingV1 {
            fact_kind: GroundedFactKind::Entity,
            fact_key: "aurora".to_string(),
            evidence_ordinal: ordinal,
            locator_kind,
            locator_sha256: Sha256Digest::for_bytes(format!("locator-{ordinal}").as_bytes()),
            start_byte,
            end_byte,
            evidence_sha256: Sha256Digest::for_bytes(format!("evidence-{ordinal}").as_bytes()),
        }
    }

    fn receipt(
        resolved_spans: Vec<ResolvedEvidenceBindingV1>,
        segment_count: u32,
    ) -> HostEvidenceResolutionReceiptV1 {
        let count = u32::try_from(resolved_spans.len()).expect("bounded count");
        let mut receipt = HostEvidenceResolutionReceiptV1 {
            schema_version: HOST_EVIDENCE_RESOLVER_SCHEMA_VERSION,
            tool_schema_version: GROUNDED_TOOL_V4_SCHEMA_VERSION,
            contract: HOST_EVIDENCE_RESOLVER_CONTRACT.to_string(),
            source_content_sha256: Sha256Digest::for_bytes(b"source"),
            tool_input_sha256: Sha256Digest::for_bytes(b"input"),
            segment_catalog_sha256: Sha256Digest::for_bytes(b"catalog"),
            segment_count,
            selector_count: count,
            resolved_span_count: count,
            resolved_spans,
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
            receipt_sha256: Sha256Digest::for_bytes(b"pending"),
        };
        receipt.receipt_sha256 = evidence_resolution_receipt_digest(&receipt);
        receipt
    }

    #[test]
    fn receipt_rejects_digest_tampering() {
        let mut receipt = receipt(
            vec![binding(0, 0, 8, ResolvedLocatorKindV1::ExactQuote)],
            0,
        );
        receipt.receipt_sha256 = Sha256Digest::for_bytes(b"tampered");
        assert!(receipt.validate().is_err());
    }

    #[test]
    fn receipt_rejects_overlapping_spans_even_with_a_recomputed_digest() {
        let receipt = receipt(
            vec![
                binding(0, 0, 8, ResolvedLocatorKindV1::ExactQuote),
                binding(1, 4, 12, ResolvedLocatorKindV1::ExactQuote),
            ],
            0,
        );
        assert!(receipt.validate().is_err());
    }

    #[test]
    fn receipt_rejects_more_than_four_spans_for_one_fact() {
        let spans = (0_u32..5)
            .map(|ordinal| {
                binding(
                    ordinal,
                    ordinal * 10,
                    ordinal * 10 + 5,
                    ResolvedLocatorKindV1::ExactQuote,
                )
            })
            .collect();
        assert!(receipt(spans, 0).validate().is_err());
    }

    #[test]
    fn receipt_rejects_segment_binding_without_a_catalog() {
        let receipt = receipt(
            vec![binding(0, 0, 8, ResolvedLocatorKindV1::SourceSegment)],
            0,
        );
        assert!(receipt.validate().is_err());
    }
}
