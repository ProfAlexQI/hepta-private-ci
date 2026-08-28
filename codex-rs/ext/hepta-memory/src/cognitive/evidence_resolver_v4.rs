//! P0.3.3 qualification-only host-owned evidence resolution.
//!
//! The future model-facing grounded tool contract accepts exact quotes or
//! opaque host-issued segment IDs. The Rust host resolves those selectors
//! against the exact witnessed source, computes UTF-8 byte offsets and
//! SHA-256 digests, and then lowers the result into the existing v3 span
//! contract. Models never provide byte arithmetic or cryptographic digests.
//!
//! This module is compiled and tested but deliberately not registered with
//! `ToolContributor`. It grants no production, projection, external-effect,
//! operator-acceptance, promotion, or CALLERS authority.

use std::collections::BTreeMap;

use codex_hepta_contracts::Sha256Digest;
use codex_hepta_memory::GroundedFactKind;
use codex_hepta_memory::GroundedKgFactSetDraft;
use serde::Deserialize;
use serde::Serialize;

#[path = "evidence_resolver_v4/receipt.rs"]
mod receipt;
#[path = "evidence_resolver_v4/resolver.rs"]
mod resolver;
#[path = "evidence_resolver_v4/schema.rs"]
mod schema;
#[path = "evidence_resolver_v4/support.rs"]
mod support;

#[cfg(test)]
#[path = "evidence_resolver_v4/tests.rs"]
mod tests;

pub(crate) fn grounded_tool_v4_schema() -> serde_json::Value {
    schema::grounded_tool_v4_schema_impl()
}

pub(crate) fn prepare_grounded_tool_v4(
    source_content: &str,
    segment_drafts: &[SourceSegmentDraftV1],
    input: GroundedToolV4Input,
) -> Result<HostResolvedGroundingV4, String> {
    resolver::prepare_grounded_tool_v4_impl(source_content, segment_drafts, input)
}

pub(crate) const GROUNDED_TOOL_V4_SCHEMA_VERSION: u32 = 4;
pub(crate) const HOST_EVIDENCE_RESOLVER_SCHEMA_VERSION: u32 = 1;
pub(crate) const GROUNDED_TOOL_V4_REGISTERED: bool = false;
pub(crate) const GROUNDED_TOOL_V4_PRODUCTION_AUTHORITY: bool = false;
pub(crate) const GROUNDED_TOOL_V4_EXTERNAL_EFFECTS: bool = false;
pub(crate) const GROUNDED_TOOL_V4_OPERATOR_ACCEPTANCE: bool = false;
pub(crate) const GROUNDED_TOOL_V4_PROMOTION: bool = false;
pub(crate) const GROUNDED_TOOL_V4_CALLERS_RATCHET: bool = false;
pub(crate) const MODEL_SUPPLIED_BYTE_OFFSETS: bool = false;
pub(crate) const MODEL_SUPPLIED_DIGESTS: bool = false;
pub(crate) const HOST_RESOLVED_BYTE_OFFSETS: bool = true;
pub(crate) const HOST_RESOLVED_DIGESTS: bool = true;

const HOST_EVIDENCE_RESOLVER_CONTRACT: &str = "host_owned_evidence_resolver_v1";
const SOURCE_SEGMENT_ID_PREFIX: &str = "source-segment:v1:";
// These limits intentionally mirror the dormant v3 lowering contract without
// widening its accepted surface. They remain local because v3 does not expose
// its implementation constants as a sibling-module API.
const MAX_ENTITIES: usize = 64;
const MAX_RELATIONS: usize = 128;
const MAX_SPANS_PER_FACT: usize = 4;
const MAX_TOTAL_SPANS: usize = 768;
const MAX_KEY_BYTES: usize = 256;
const MAX_TYPE_BYTES: usize = 128;
const MAX_LABEL_BYTES: usize = 1024;
const MAX_RELATION_BYTES: usize = 128;
const MAX_SOURCE_BYTES: usize = 256 * 1024;
const MAX_SOURCE_SEGMENTS: usize = 512;
const MAX_SEGMENT_BYTES: usize = 64 * 1024;
const MAX_QUOTE_BYTES: usize = 4096;
const MAX_QUOTE_OCCURRENCE: u32 = 1023;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GroundedToolV4Input {
    #[serde(default)]
    pub(crate) entities: Vec<GroundedEntityV4>,
    #[serde(default)]
    pub(crate) relations: Vec<GroundedRelationV4>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GroundedEntityV4 {
    pub(crate) key: String,
    pub(crate) entity_type: String,
    pub(crate) label: String,
    pub(crate) evidence: Vec<EvidenceLocatorV4>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GroundedRelationV4 {
    pub(crate) key: String,
    pub(crate) from_entity_key: String,
    pub(crate) to_entity_key: String,
    pub(crate) relation: String,
    pub(crate) evidence: Vec<EvidenceLocatorV4>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum EvidenceLocatorV4 {
    ExactQuote(ExactQuoteLocatorV4),
    SourceSegment(SourceSegmentLocatorV4),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExactQuoteLocatorV4 {
    pub(crate) quote: String,
    pub(crate) occurrence: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SourceSegmentLocatorV4 {
    pub(crate) segment_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SourceSegmentDraftV1 {
    pub(crate) start_byte: u32,
    pub(crate) end_byte: u32,
}

impl SourceSegmentDraftV1 {
    pub(crate) const fn new(start_byte: u32, end_byte: u32) -> Self {
        Self {
            start_byte,
            end_byte,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct SourceSegmentDescriptorV1 {
    pub(crate) segment_id: String,
    pub(crate) ordinal: u32,
    pub(crate) start_byte: u32,
    pub(crate) end_byte: u32,
    pub(crate) evidence_sha256: Sha256Digest,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ResolvedLocatorKindV1 {
    ExactQuote,
    SourceSegment,
}

impl ResolvedLocatorKindV1 {
    const fn as_str(self) -> &'static str {
        match self {
            Self::ExactQuote => "exact_quote",
            Self::SourceSegment => "source_segment",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ResolvedEvidenceBindingV1 {
    pub(crate) fact_kind: GroundedFactKind,
    pub(crate) fact_key: String,
    pub(crate) evidence_ordinal: u32,
    pub(crate) locator_kind: ResolvedLocatorKindV1,
    pub(crate) locator_sha256: Sha256Digest,
    pub(crate) start_byte: u32,
    pub(crate) end_byte: u32,
    pub(crate) evidence_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct HostEvidenceResolutionReceiptV1 {
    pub(crate) schema_version: u32,
    pub(crate) tool_schema_version: u32,
    pub(crate) contract: String,
    pub(crate) source_content_sha256: Sha256Digest,
    pub(crate) tool_input_sha256: Sha256Digest,
    pub(crate) segment_catalog_sha256: Sha256Digest,
    pub(crate) segment_count: u32,
    pub(crate) selector_count: u32,
    pub(crate) resolved_span_count: u32,
    pub(crate) resolved_spans: Vec<ResolvedEvidenceBindingV1>,
    pub(crate) model_supplied_byte_offsets: bool,
    pub(crate) model_supplied_digests: bool,
    pub(crate) host_resolved_byte_offsets: bool,
    pub(crate) host_resolved_digests: bool,
    pub(crate) tool_registered: bool,
    pub(crate) production_authority: bool,
    pub(crate) external_effects: bool,
    pub(crate) operator_acceptance: bool,
    pub(crate) promotion: bool,
    pub(crate) callers_ratchet: bool,
    pub(crate) receipt_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HostResolvedGroundingV4 {
    pub(crate) grounded: GroundedKgFactSetDraft,
    pub(crate) resolution: HostEvidenceResolutionReceiptV1,
}

pub(crate) struct HostEvidenceResolverV1<'a> {
    source_content: &'a str,
    source_content_sha256: Sha256Digest,
    segment_catalog_sha256: Sha256Digest,
    segments: Vec<SourceSegmentDescriptorV1>,
    segment_index: BTreeMap<String, usize>,
}
