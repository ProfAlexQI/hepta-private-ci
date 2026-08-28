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
        if usize::try_from(self.selector_count).ok() != Some(self.resolved_spans.len())
            || self.selector_count != self.resolved_span_count
            || self.resolved_spans.len() > MAX_TOTAL_SPANS
            || usize::try_from(self.segment_count)
                .ok()
                .is_none_or(|count| count > MAX_SOURCE_SEGMENTS)
        {
            return Err("host evidence resolver receipt count mismatch".to_string());
        }

        let mut previous_identity: Option<(GroundedFactKind, &str)> = None;
        let mut expected_ordinal = 0_u32;
        for span in &self.resolved_spans {
            if span.fact_key.trim().is_empty()
                || span.fact_key.len() > MAX_KEY_BYTES
                || span.fact_key.as_bytes().contains(&0)
                || span.start_byte >= span.end_byte
            {
                return Err("host evidence resolver receipt contains an invalid span".to_string());
            }
            let identity = (span.fact_kind, span.fact_key.as_str());
            match previous_identity {
                None if span.evidence_ordinal != 0 => {
                    return Err(
                        "host evidence resolver first evidence ordinal must be zero".to_string()
                    );
                }
                Some(previous) if previous == identity => {
                    if span.evidence_ordinal != expected_ordinal {
                        return Err(
                            "host evidence resolver evidence ordinals are not contiguous"
                                .to_string(),
                        );
                    }
                }
                Some(previous) if identity < previous || span.evidence_ordinal != 0 => {
                    return Err(
                        "host evidence resolver evidence ordering is invalid".to_string()
                    );
                }
                _ => {}
            }
            previous_identity = Some(identity);
            expected_ordinal = span.evidence_ordinal.checked_add(1).ok_or_else(|| {
                "host evidence resolver evidence ordinal overflow".to_string()
            })?;
        }

        if self.receipt_sha256 != evidence_resolution_receipt_digest(self) {
            return Err("host evidence resolver receipt digest mismatch".to_string());
        }
        Ok(())
    }
}
