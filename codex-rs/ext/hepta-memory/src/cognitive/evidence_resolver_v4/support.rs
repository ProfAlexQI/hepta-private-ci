use sha2::Digest;
use sha2::Sha256;

use super::*;

pub(super) fn validate_source_range(
    source_content: &str,
    start_byte: u32,
    end_byte: u32,
    label: &str,
) -> Result<(usize, usize), String> {
    let start = usize::try_from(start_byte)
        .map_err(|_| format!("{label} start byte exceeds usize"))?;
    let end =
        usize::try_from(end_byte).map_err(|_| format!("{label} end byte exceeds usize"))?;
    if start >= end || end > source_content.len() {
        return Err(format!("{label} range is outside source content"));
    }
    if !source_content.is_char_boundary(start) || !source_content.is_char_boundary(end) {
        return Err(format!("{label} range splits a UTF-8 character"));
    }
    Ok((start, end))
}

pub(super) fn resolve_exact_quote(
    source_content: &str,
    quote: &str,
    occurrence: u32,
) -> Result<(u32, u32), String> {
    if quote.trim().is_empty() || quote.len() > MAX_QUOTE_BYTES || quote.as_bytes().contains(&0) {
        return Err(format!(
            "exact quote must contain 1..={MAX_QUOTE_BYTES} non-NUL bytes"
        ));
    }
    if occurrence > MAX_QUOTE_OCCURRENCE {
        return Err(format!(
            "exact quote occurrence exceeds {MAX_QUOTE_OCCURRENCE}"
        ));
    }

    let mut search_from = 0_usize;
    let mut seen = 0_u32;
    while search_from < source_content.len() {
        let Some(relative) = source_content[search_from..].find(quote) else {
            break;
        };
        let start = search_from
            .checked_add(relative)
            .ok_or_else(|| "exact quote start overflow".to_string())?;
        let end = start
            .checked_add(quote.len())
            .ok_or_else(|| "exact quote end overflow".to_string())?;
        if seen == occurrence {
            return Ok((
                u32::try_from(start).map_err(|_| "exact quote start exceeds u32".to_string())?,
                u32::try_from(end).map_err(|_| "exact quote end exceeds u32".to_string())?,
            ));
        }
        seen = seen
            .checked_add(1)
            .ok_or_else(|| "exact quote occurrence counter overflow".to_string())?;
        let advance = source_content[start..]
            .chars()
            .next()
            .ok_or_else(|| "exact quote search could not advance".to_string())?
            .len_utf8();
        search_from = start
            .checked_add(advance)
            .ok_or_else(|| "exact quote search offset overflow".to_string())?;
    }
    Err(format!(
        "exact quote occurrence {occurrence} does not exist; resolved {seen} matches"
    ))
}

pub(super) fn validate_segment_id(segment_id: &str) -> Result<(), String> {
    let Some(digest) = segment_id.strip_prefix(SOURCE_SEGMENT_ID_PREFIX) else {
        return Err("source segment ID has an invalid prefix".to_string());
    };
    if segment_id.len() != SOURCE_SEGMENT_ID_PREFIX.len() + 64 {
        return Err("source segment ID has an invalid length".to_string());
    }
    Sha256Digest::parse(digest)
        .map(|_| ())
        .map_err(|error| format!("source segment ID has an invalid digest: {error}"))
}

pub(super) fn source_segment_id(
    source_content_sha256: &Sha256Digest,
    start_byte: u32,
    end_byte: u32,
    evidence_sha256: &Sha256Digest,
) -> String {
    let start = start_byte.to_be_bytes();
    let end = end_byte.to_be_bytes();
    let digest = super::super::digest_many(
        b"hepta:cognitive:source-segment-id:v1",
        &[
            source_content_sha256.as_str().as_bytes(),
            &start,
            &end,
            evidence_sha256.as_str().as_bytes(),
        ],
    );
    let digest = digest.as_str();
    format!("{SOURCE_SEGMENT_ID_PREFIX}{digest}")
}

pub(super) fn segment_catalog_digest(
    source_content_sha256: &Sha256Digest,
    segments: &[SourceSegmentDescriptorV1],
) -> Sha256Digest {
    let mut hasher = Sha256::new();
    super::super::hash_part(
        &mut hasher,
        b"hepta:cognitive:source-segment-catalog:v1",
    );
    super::super::hash_part(&mut hasher, source_content_sha256.as_str().as_bytes());
    super::super::hash_part(
        &mut hasher,
        &u64::try_from(segments.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for segment in segments {
        super::super::hash_part(&mut hasher, segment.segment_id.as_bytes());
        super::super::hash_part(&mut hasher, &segment.ordinal.to_be_bytes());
        super::super::hash_part(&mut hasher, &segment.start_byte.to_be_bytes());
        super::super::hash_part(&mut hasher, &segment.end_byte.to_be_bytes());
        super::super::hash_part(&mut hasher, segment.evidence_sha256.as_str().as_bytes());
    }
    Sha256Digest::for_bytes(&hasher.finalize())
}

pub(super) fn evidence_resolution_receipt_digest(
    receipt: &HostEvidenceResolutionReceiptV1,
) -> Sha256Digest {
    let mut hasher = Sha256::new();
    super::super::hash_part(
        &mut hasher,
        b"hepta:cognitive:host-evidence-resolution-receipt:v1",
    );
    super::super::hash_part(&mut hasher, &receipt.schema_version.to_be_bytes());
    super::super::hash_part(&mut hasher, &receipt.tool_schema_version.to_be_bytes());
    super::super::hash_part(&mut hasher, receipt.contract.as_bytes());
    super::super::hash_part(
        &mut hasher,
        receipt.source_content_sha256.as_str().as_bytes(),
    );
    super::super::hash_part(
        &mut hasher,
        receipt.tool_input_sha256.as_str().as_bytes(),
    );
    super::super::hash_part(
        &mut hasher,
        receipt.segment_catalog_sha256.as_str().as_bytes(),
    );
    super::super::hash_part(&mut hasher, &receipt.segment_count.to_be_bytes());
    super::super::hash_part(&mut hasher, &receipt.selector_count.to_be_bytes());
    super::super::hash_part(&mut hasher, &receipt.resolved_span_count.to_be_bytes());
    for span in &receipt.resolved_spans {
        super::super::hash_part(&mut hasher, span.fact_kind.as_str().as_bytes());
        super::super::hash_part(&mut hasher, span.fact_key.as_bytes());
        super::super::hash_part(&mut hasher, &span.evidence_ordinal.to_be_bytes());
        super::super::hash_part(&mut hasher, span.locator_kind.as_str().as_bytes());
        super::super::hash_part(&mut hasher, span.locator_sha256.as_str().as_bytes());
        super::super::hash_part(&mut hasher, &span.start_byte.to_be_bytes());
        super::super::hash_part(&mut hasher, &span.end_byte.to_be_bytes());
        super::super::hash_part(&mut hasher, span.evidence_sha256.as_str().as_bytes());
    }
    for flag in [
        receipt.model_supplied_byte_offsets,
        receipt.model_supplied_digests,
        receipt.host_resolved_byte_offsets,
        receipt.host_resolved_digests,
        receipt.tool_registered,
        receipt.production_authority,
        receipt.external_effects,
        receipt.operator_acceptance,
        receipt.promotion,
        receipt.callers_ratchet,
    ] {
        super::super::hash_part(&mut hasher, &[u8::from(flag)]);
    }
    Sha256Digest::for_bytes(&hasher.finalize())
}
