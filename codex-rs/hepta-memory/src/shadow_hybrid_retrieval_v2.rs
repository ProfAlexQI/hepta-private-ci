//! P1.1a shadow-only hybrid retrieval planning and deterministic fusion.
//!
//! This module establishes the typed query, candidate, eligibility, token
//! budget, fixed-point fusion, and receipt contracts required before a real
//! lexical/vector/KG retrieval runtime can be wired. It deliberately performs
//! no SQLite query, embedding inference, KG traversal, model rerank, context
//! attachment, physical send, or external effect.
//!
//! The product retrieval implementation remains unchanged. All authority and
//! runtime flags in this source tranche are negative.

use std::collections::BTreeSet;

use codex_hepta_contracts::Sha256Digest;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::framing::frame_part;

pub(crate) const SHADOW_HYBRID_RETRIEVAL_SCHEMA_VERSION: u32 = 1;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_NAMESPACE: &str =
    "shadow_hybrid_retrieval_v2_contract_v1";
pub(crate) const SHADOW_HYBRID_RETRIEVAL_RUNTIME_WIRED: bool = false;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_DEFAULT_RECALL_CHANGED: bool = false;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_VECTOR_BACKEND_REGISTERED: bool = false;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_RERANKER_REGISTERED: bool = false;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_CONTEXT_ATTACHMENT: bool = false;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_PHYSICAL_SEND: bool = false;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_EXTERNAL_EFFECTS: bool = false;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_PRODUCTION_AUTHORITY: bool = false;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_OPERATOR_ACCEPTANCE: bool = false;
pub(crate) const SHADOW_HYBRID_RETRIEVAL_PROMOTION: bool = false;

pub(crate) const SCORE_SCALE_PPM: u32 = 1_000_000;
pub(crate) const MAX_QUERY_BYTES: usize = 32 * 1024;
pub(crate) const MAX_SCOPE_KEY_BYTES: usize = 512;
pub(crate) const MAX_TERM_BYTES: usize = 128;
pub(crate) const MAX_QUERY_TERMS: usize = 48;
pub(crate) const MAX_QUERY_ENTITIES: usize = 32;
pub(crate) const MAX_CHANNEL_EVIDENCE: usize = 8;
pub(crate) const MAX_CANDIDATES: usize = 512;
pub(crate) const MAX_RESULTS: usize = 32;
pub(crate) const MAX_CONTEXT_TOKENS: u32 = 32 * 1024;
pub(crate) const RECIPROCAL_RANK_K: u32 = 60;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HybridRetrievalIntent {
    RecallFact,
    RecallPreference,
    ResolveEntity,
    ResolveTemporalState,
    RetrieveProcedure,
    GeneralContext,
}

impl HybridRetrievalIntent {
    const fn as_str(self) -> &'static str {
        match self {
            Self::RecallFact => "recall_fact",
            Self::RecallPreference => "recall_preference",
            Self::ResolveEntity => "resolve_entity",
            Self::ResolveTemporalState => "resolve_temporal_state",
            Self::RetrieveProcedure => "retrieve_procedure",
            Self::GeneralContext => "general_context",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HybridRetrievalRiskClass {
    Low,
    Standard,
    High,
    Critical,
}

impl HybridRetrievalRiskClass {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Standard => "standard",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    const fn requires_grounded_memory(self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HybridRetrievalChannel {
    ExactLexical,
    LexicalFts,
    EntityAlias,
    SemanticVector,
    KnowledgeGraph,
    Recency,
}

impl HybridRetrievalChannel {
    const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLexical => "exact_lexical",
            Self::LexicalFts => "lexical_fts",
            Self::EntityAlias => "entity_alias",
            Self::SemanticVector => "semantic_vector",
            Self::KnowledgeGraph => "knowledge_graph",
            Self::Recency => "recency",
        }
    }

    const fn default_weight_ppm(self) -> u32 {
        match self {
            Self::ExactLexical => 1_000_000,
            Self::LexicalFts => 900_000,
            Self::EntityAlias => 850_000,
            Self::SemanticVector => 800_000,
            Self::KnowledgeGraph => 850_000,
            Self::Recency => 500_000,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HybridGroundingStatus {
    GroundedV1,
    BackfilledGroundedV1,
    LegacyUnreviewed,
    ZeroFact,
}

impl HybridGroundingStatus {
    const fn as_str(self) -> &'static str {
        match self {
            Self::GroundedV1 => "grounded_v1",
            Self::BackfilledGroundedV1 => "backfilled_grounded_v1",
            Self::LegacyUnreviewed => "legacy_unreviewed",
            Self::ZeroFact => "zero_fact",
        }
    }

    const fn is_grounded(self) -> bool {
        matches!(self, Self::GroundedV1 | Self::BackfilledGroundedV1)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HybridTruthStatus {
    Candidate,
    Grounded,
    Confirmed,
    Disputed,
    Contradicted,
    Expired,
}

impl HybridTruthStatus {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::Grounded => "grounded",
            Self::Confirmed => "confirmed",
            Self::Disputed => "disputed",
            Self::Contradicted => "contradicted",
            Self::Expired => "expired",
        }
    }

    const fn is_retrievable(self) -> bool {
        matches!(self, Self::Candidate | Self::Grounded | Self::Confirmed)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HybridLifecycle {
    Active,
    Tombstoned,
    Expired,
}

impl HybridLifecycle {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Tombstoned => "tombstoned",
            Self::Expired => "expired",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HybridTokenizerEstimator {
    Utf8ByteUpperBound,
    ExactTokenizer,
}

impl HybridTokenizerEstimator {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Utf8ByteUpperBound => "utf8_byte_upper_bound",
            Self::ExactTokenizer => "exact_tokenizer",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HybridRetrievalTimeRange {
    pub(crate) start_unix_seconds: i64,
    pub(crate) end_unix_seconds: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ShadowHybridRetrievalQueryDraft {
    pub(crate) query: String,
    pub(crate) scope_key: String,
    pub(crate) intent: HybridRetrievalIntent,
    pub(crate) risk_class: HybridRetrievalRiskClass,
    pub(crate) lexical_terms: Vec<String>,
    pub(crate) entities: Vec<String>,
    pub(crate) semantic_query: Option<String>,
    pub(crate) required_time_range: Option<HybridRetrievalTimeRange>,
    pub(crate) max_results: usize,
    pub(crate) max_context_tokens: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ShadowHybridRetrievalPlan {
    pub(crate) schema_version: u32,
    pub(crate) namespace: String,
    pub(crate) query_sha256: Sha256Digest,
    pub(crate) scope_key_sha256: Sha256Digest,
    pub(crate) intent: HybridRetrievalIntent,
    pub(crate) risk_class: HybridRetrievalRiskClass,
    pub(crate) lexical_terms: Vec<String>,
    pub(crate) entities: Vec<String>,
    pub(crate) semantic_query_sha256: Option<Sha256Digest>,
    pub(crate) required_time_range: Option<(i64, i64)>,
    pub(crate) enabled_channels: Vec<HybridRetrievalChannel>,
    pub(crate) max_results: u32,
    pub(crate) max_context_tokens: u32,
    pub(crate) tokenizer_estimator: HybridTokenizerEstimator,
    pub(crate) exact_tokenizer_available: bool,
    pub(crate) vector_backend_registered: bool,
    pub(crate) reranker_registered: bool,
    pub(crate) runtime_wired: bool,
    pub(crate) default_recall_changed: bool,
    pub(crate) context_attachment: bool,
    pub(crate) physical_send: bool,
    pub(crate) production_authority: bool,
    pub(crate) plan_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HybridChannelEvidenceDraft {
    pub(crate) channel: HybridRetrievalChannel,
    pub(crate) rank: u32,
    pub(crate) channel_score_ppm: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ShadowHybridRetrievalCandidateDraft {
    pub(crate) candidate_id: String,
    pub(crate) memory_revision: u64,
    pub(crate) lifecycle: HybridLifecycle,
    pub(crate) grounding_status: HybridGroundingStatus,
    pub(crate) truth_status: HybridTruthStatus,
    pub(crate) source_reliability_ppm: u32,
    pub(crate) freshness_ppm: u32,
    pub(crate) estimated_context_tokens: u32,
    pub(crate) secret_like: bool,
    pub(crate) evidence: Vec<HybridChannelEvidenceDraft>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ShadowHybridRetrievalResult {
    pub(crate) candidate_id: String,
    pub(crate) memory_revision: u64,
    pub(crate) fused_score_ppm: u32,
    pub(crate) lexical_score_ppm: u32,
    pub(crate) semantic_score_ppm: u32,
    pub(crate) graph_score_ppm: u32,
    pub(crate) freshness_ppm: u32,
    pub(crate) source_reliability_ppm: u32,
    pub(crate) grounding_status: HybridGroundingStatus,
    pub(crate) truth_status: HybridTruthStatus,
    pub(crate) estimated_context_tokens: u32,
    pub(crate) channel_count: u32,
    pub(crate) candidate_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ShadowHybridRetrievalReceipt {
    pub(crate) schema_version: u32,
    pub(crate) namespace: String,
    pub(crate) plan_sha256: Sha256Digest,
    pub(crate) candidate_set_sha256: Sha256Digest,
    pub(crate) selected_result_sha256: Sha256Digest,
    pub(crate) input_candidate_count: u32,
    pub(crate) eligible_candidate_count: u32,
    pub(crate) selected_candidate_count: u32,
    pub(crate) rejected_inactive_count: u32,
    pub(crate) rejected_truth_count: u32,
    pub(crate) rejected_grounding_count: u32,
    pub(crate) rejected_secret_count: u32,
    pub(crate) rejected_budget_count: u32,
    pub(crate) selected_context_tokens: u32,
    pub(crate) results: Vec<ShadowHybridRetrievalResult>,
    pub(crate) tokenizer_estimator: HybridTokenizerEstimator,
    pub(crate) exact_tokenizer_available: bool,
    pub(crate) vector_backend_registered: bool,
    pub(crate) reranker_registered: bool,
    pub(crate) runtime_wired: bool,
    pub(crate) default_recall_changed: bool,
    pub(crate) context_attachment: bool,
    pub(crate) physical_send: bool,
    pub(crate) external_effects: bool,
    pub(crate) production_authority: bool,
    pub(crate) operator_acceptance: bool,
    pub(crate) promotion: bool,
    pub(crate) receipt_sha256: Sha256Digest,
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub(crate) enum ShadowHybridRetrievalError {
    #[error("invalid shadow hybrid retrieval contract: {0}")]
    Invalid(String),
    #[error("shadow hybrid retrieval input exceeds {label} limit {max}")]
    Limit { label: &'static str, max: usize },
    #[error("shadow hybrid retrieval fixed-point arithmetic overflow")]
    Overflow,
}

#[derive(Default)]
struct RejectionCounts {
    inactive: u32,
    truth: u32,
    grounding: u32,
    secret: u32,
    budget: u32,
}

pub(crate) fn plan_shadow_hybrid_retrieval(
    draft: &ShadowHybridRetrievalQueryDraft,
) -> Result<ShadowHybridRetrievalPlan, ShadowHybridRetrievalError> {
    validate_bounded_text(&draft.query, MAX_QUERY_BYTES, "query")?;
    validate_bounded_text(&draft.scope_key, MAX_SCOPE_KEY_BYTES, "scope key")?;
    if draft.max_results == 0 || draft.max_results > MAX_RESULTS {
        return Err(ShadowHybridRetrievalError::Limit {
            label: "result count",
            max: MAX_RESULTS,
        });
    }
    if draft.max_context_tokens == 0 || draft.max_context_tokens > MAX_CONTEXT_TOKENS {
        return Err(ShadowHybridRetrievalError::Invalid(format!(
            "context token budget must contain 1..={MAX_CONTEXT_TOKENS} tokens"
        )));
    }
    if let Some(range) = &draft.required_time_range {
        if range.start_unix_seconds > range.end_unix_seconds {
            return Err(ShadowHybridRetrievalError::Invalid(
                "required time range is inverted".to_string(),
            ));
        }
    }

    let mut lexical_terms = canonical_terms(&draft.lexical_terms, MAX_QUERY_TERMS)?;
    if lexical_terms.is_empty() {
        lexical_terms = derive_terms(&draft.query)?;
    }
    let entities = canonical_terms(&draft.entities, MAX_QUERY_ENTITIES)?;
    let semantic_query_sha256 = draft
        .semantic_query
        .as_ref()
        .map(|query| {
            validate_bounded_text(query, MAX_QUERY_BYTES, "semantic query")?;
            Ok(Sha256Digest::for_bytes(query.as_bytes()))
        })
        .transpose()?;

    let mut enabled_channels = vec![
        HybridRetrievalChannel::ExactLexical,
        HybridRetrievalChannel::LexicalFts,
        HybridRetrievalChannel::Recency,
    ];
    if !entities.is_empty() {
        enabled_channels.push(HybridRetrievalChannel::EntityAlias);
        enabled_channels.push(HybridRetrievalChannel::KnowledgeGraph);
    }
    if semantic_query_sha256.is_some() {
        enabled_channels.push(HybridRetrievalChannel::SemanticVector);
    }
    enabled_channels.sort();
    enabled_channels.dedup();

    let mut plan = ShadowHybridRetrievalPlan {
        schema_version: SHADOW_HYBRID_RETRIEVAL_SCHEMA_VERSION,
        namespace: SHADOW_HYBRID_RETRIEVAL_NAMESPACE.to_string(),
        query_sha256: Sha256Digest::for_bytes(draft.query.as_bytes()),
        scope_key_sha256: Sha256Digest::for_bytes(draft.scope_key.as_bytes()),
        intent: draft.intent,
        risk_class: draft.risk_class,
        lexical_terms,
        entities,
        semantic_query_sha256,
        required_time_range: draft
            .required_time_range
            .as_ref()
            .map(|range| (range.start_unix_seconds, range.end_unix_seconds)),
        enabled_channels,
        max_results: u32::try_from(draft.max_results)
            .map_err(|_| ShadowHybridRetrievalError::Overflow)?,
        max_context_tokens: draft.max_context_tokens,
        tokenizer_estimator: HybridTokenizerEstimator::Utf8ByteUpperBound,
        exact_tokenizer_available: false,
        vector_backend_registered: SHADOW_HYBRID_RETRIEVAL_VECTOR_BACKEND_REGISTERED,
        reranker_registered: SHADOW_HYBRID_RETRIEVAL_RERANKER_REGISTERED,
        runtime_wired: SHADOW_HYBRID_RETRIEVAL_RUNTIME_WIRED,
        default_recall_changed: SHADOW_HYBRID_RETRIEVAL_DEFAULT_RECALL_CHANGED,
        context_attachment: SHADOW_HYBRID_RETRIEVAL_CONTEXT_ATTACHMENT,
        physical_send: SHADOW_HYBRID_RETRIEVAL_PHYSICAL_SEND,
        production_authority: SHADOW_HYBRID_RETRIEVAL_PRODUCTION_AUTHORITY,
        plan_sha256: Sha256Digest::for_bytes(b"uncomputed"),
    };
    plan.plan_sha256 = plan_digest(&plan);
    validate_plan_authority(&plan)?;
    Ok(plan)
}

pub(crate) fn fuse_shadow_hybrid_candidates(
    plan: &ShadowHybridRetrievalPlan,
    candidates: &[ShadowHybridRetrievalCandidateDraft],
) -> Result<ShadowHybridRetrievalReceipt, ShadowHybridRetrievalError> {
    validate_plan_authority(plan)?;
    if candidates.len() > MAX_CANDIDATES {
        return Err(ShadowHybridRetrievalError::Limit {
            label: "candidate count",
            max: MAX_CANDIDATES,
        });
    }

    let mut identities = BTreeSet::new();
    let mut rejection = RejectionCounts::default();
    let mut eligible = Vec::new();
    for candidate in candidates {
        validate_candidate(candidate)?;
        let identity = (candidate.candidate_id.clone(), candidate.memory_revision);
        if !identities.insert(identity) {
            return Err(ShadowHybridRetrievalError::Invalid(
                "duplicate candidate revision".to_string(),
            ));
        }
        if candidate.lifecycle != HybridLifecycle::Active {
            rejection.inactive = rejection.inactive.saturating_add(1);
            continue;
        }
        if !candidate.truth_status.is_retrievable() {
            rejection.truth = rejection.truth.saturating_add(1);
            continue;
        }
        if candidate.secret_like {
            rejection.secret = rejection.secret.saturating_add(1);
            continue;
        }
        if !grounding_eligible(plan.risk_class, candidate) {
            rejection.grounding = rejection.grounding.saturating_add(1);
            continue;
        }
        eligible.push(score_candidate(candidate)?);
    }

    eligible.sort_by(|left, right| {
        right
            .fused_score_ppm
            .cmp(&left.fused_score_ppm)
            .then_with(|| {
                left.estimated_context_tokens
                    .cmp(&right.estimated_context_tokens)
            })
            .then_with(|| left.candidate_id.cmp(&right.candidate_id))
            .then_with(|| right.memory_revision.cmp(&left.memory_revision))
    });

    let eligible_candidate_count =
        u32::try_from(eligible.len()).map_err(|_| ShadowHybridRetrievalError::Overflow)?;
    let mut selected = Vec::new();
    let mut selected_tokens = 0_u32;
    for result in eligible {
        if selected.len()
            >= usize::try_from(plan.max_results)
                .map_err(|_| ShadowHybridRetrievalError::Overflow)?
        {
            break;
        }
        let next_tokens = selected_tokens
            .checked_add(result.estimated_context_tokens)
            .ok_or(ShadowHybridRetrievalError::Overflow)?;
        if next_tokens > plan.max_context_tokens {
            rejection.budget = rejection.budget.saturating_add(1);
            continue;
        }
        selected_tokens = next_tokens;
        selected.push(result);
    }

    let candidate_set_sha256 = candidate_set_digest(candidates);
    let selected_result_sha256 = selected_result_digest(&selected);
    let mut receipt = ShadowHybridRetrievalReceipt {
        schema_version: SHADOW_HYBRID_RETRIEVAL_SCHEMA_VERSION,
        namespace: SHADOW_HYBRID_RETRIEVAL_NAMESPACE.to_string(),
        plan_sha256: plan.plan_sha256.clone(),
        candidate_set_sha256,
        selected_result_sha256,
        input_candidate_count: u32::try_from(candidates.len())
            .map_err(|_| ShadowHybridRetrievalError::Overflow)?,
        eligible_candidate_count,
        selected_candidate_count: u32::try_from(selected.len())
            .map_err(|_| ShadowHybridRetrievalError::Overflow)?,
        rejected_inactive_count: rejection.inactive,
        rejected_truth_count: rejection.truth,
        rejected_grounding_count: rejection.grounding,
        rejected_secret_count: rejection.secret,
        rejected_budget_count: rejection.budget,
        selected_context_tokens: selected_tokens,
        results: selected,
        tokenizer_estimator: HybridTokenizerEstimator::Utf8ByteUpperBound,
        exact_tokenizer_available: false,
        vector_backend_registered: false,
        reranker_registered: false,
        runtime_wired: false,
        default_recall_changed: false,
        context_attachment: false,
        physical_send: false,
        external_effects: false,
        production_authority: false,
        operator_acceptance: false,
        promotion: false,
        receipt_sha256: Sha256Digest::for_bytes(b"uncomputed"),
    };
    receipt.receipt_sha256 = receipt_digest(&receipt);
    validate_receipt(&receipt)?;
    Ok(receipt)
}

pub(crate) fn estimate_tokens_utf8_upper_bound(
    text: &str,
) -> Result<u32, ShadowHybridRetrievalError> {
    u32::try_from(text.len()).map_err(|_| ShadowHybridRetrievalError::Overflow)
}

fn canonical_terms(
    values: &[String],
    max_count: usize,
) -> Result<Vec<String>, ShadowHybridRetrievalError> {
    if values.len() > max_count {
        return Err(ShadowHybridRetrievalError::Limit {
            label: "query term count",
            max: max_count,
        });
    }
    let mut output = BTreeSet::new();
    for value in values {
        let normalized = normalize_text(value);
        if normalized.is_empty() {
            continue;
        }
        if normalized.len() > MAX_TERM_BYTES {
            return Err(ShadowHybridRetrievalError::Limit {
                label: "query term bytes",
                max: MAX_TERM_BYTES,
            });
        }
        output.insert(normalized);
    }
    Ok(output.into_iter().collect())
}

fn derive_terms(query: &str) -> Result<Vec<String>, ShadowHybridRetrievalError> {
    let mut terms = BTreeSet::new();
    let mut current = String::new();
    for character in query.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() {
            current.push(character);
            if current.len() > MAX_TERM_BYTES {
                return Err(ShadowHybridRetrievalError::Limit {
                    label: "derived term bytes",
                    max: MAX_TERM_BYTES,
                });
            }
        } else if !current.is_empty() {
            terms.insert(std::mem::take(&mut current));
            if terms.len() >= MAX_QUERY_TERMS {
                break;
            }
        }
    }
    if !current.is_empty() && terms.len() < MAX_QUERY_TERMS {
        terms.insert(current);
    }
    if terms.is_empty() {
        return Err(ShadowHybridRetrievalError::Invalid(
            "query contains no semantic term".to_string(),
        ));
    }
    Ok(terms.into_iter().collect())
}

fn normalize_text(value: &str) -> String {
    let mut output = String::new();
    let mut pending_space = false;
    for character in value.trim().chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() {
            if pending_space && !output.is_empty() {
                output.push(' ');
            }
            output.push(character);
            pending_space = false;
        } else {
            pending_space = true;
        }
    }
    output
}

fn validate_bounded_text(
    value: &str,
    max: usize,
    label: &str,
) -> Result<(), ShadowHybridRetrievalError> {
    if value.trim().is_empty() || value.len() > max || value.as_bytes().contains(&0) {
        return Err(ShadowHybridRetrievalError::Invalid(format!(
            "{label} must contain 1..={max} non-NUL bytes"
        )));
    }
    Ok(())
}

fn validate_plan_authority(
    plan: &ShadowHybridRetrievalPlan,
) -> Result<(), ShadowHybridRetrievalError> {
    if plan.schema_version != SHADOW_HYBRID_RETRIEVAL_SCHEMA_VERSION
        || plan.namespace != SHADOW_HYBRID_RETRIEVAL_NAMESPACE
    {
        return Err(ShadowHybridRetrievalError::Invalid(
            "unsupported hybrid retrieval plan contract".to_string(),
        ));
    }
    if plan.vector_backend_registered
        || plan.reranker_registered
        || plan.runtime_wired
        || plan.default_recall_changed
        || plan.context_attachment
        || plan.physical_send
        || plan.production_authority
    {
        return Err(ShadowHybridRetrievalError::Invalid(
            "hybrid retrieval plan crosses its source-only authority boundary".to_string(),
        ));
    }
    if plan.plan_sha256 != plan_digest(plan) {
        return Err(ShadowHybridRetrievalError::Invalid(
            "hybrid retrieval plan digest mismatch".to_string(),
        ));
    }
    Ok(())
}

fn validate_candidate(
    candidate: &ShadowHybridRetrievalCandidateDraft,
) -> Result<(), ShadowHybridRetrievalError> {
    validate_bounded_text(
        &candidate.candidate_id,
        MAX_SCOPE_KEY_BYTES,
        "candidate id",
    )?;
    if candidate.memory_revision == 0 {
        return Err(ShadowHybridRetrievalError::Invalid(
            "memory revision must be positive".to_string(),
        ));
    }
    if candidate.source_reliability_ppm > SCORE_SCALE_PPM
        || candidate.freshness_ppm > SCORE_SCALE_PPM
    {
        return Err(ShadowHybridRetrievalError::Invalid(
            "candidate reliability and freshness must be PPM values".to_string(),
        ));
    }
    if candidate.estimated_context_tokens == 0
        || candidate.estimated_context_tokens > MAX_CONTEXT_TOKENS
    {
        return Err(ShadowHybridRetrievalError::Invalid(
            "candidate token estimate is outside the bounded contract".to_string(),
        ));
    }
    if candidate.evidence.is_empty() || candidate.evidence.len() > MAX_CHANNEL_EVIDENCE {
        return Err(ShadowHybridRetrievalError::Limit {
            label: "channel evidence count",
            max: MAX_CHANNEL_EVIDENCE,
        });
    }
    let mut channels = BTreeSet::new();
    for evidence in &candidate.evidence {
        if !channels.insert(evidence.channel) {
            return Err(ShadowHybridRetrievalError::Invalid(
                "candidate contains duplicate channel evidence".to_string(),
            ));
        }
        if evidence.rank == 0 || evidence.channel_score_ppm > SCORE_SCALE_PPM {
            return Err(ShadowHybridRetrievalError::Invalid(
                "channel rank or score is outside the bounded contract".to_string(),
            ));
        }
    }
    Ok(())
}

fn grounding_eligible(
    risk: HybridRetrievalRiskClass,
    candidate: &ShadowHybridRetrievalCandidateDraft,
) -> bool {
    if risk.requires_grounded_memory()
        && !(candidate.grounding_status.is_grounded()
            || candidate.grounding_status == HybridGroundingStatus::ZeroFact)
    {
        return false;
    }
    let has_graph = candidate
        .evidence
        .iter()
        .any(|evidence| evidence.channel == HybridRetrievalChannel::KnowledgeGraph);
    if has_graph && !candidate.grounding_status.is_grounded() {
        return false;
    }
    candidate.grounding_status != HybridGroundingStatus::ZeroFact || !has_graph
}

fn score_candidate(
    candidate: &ShadowHybridRetrievalCandidateDraft,
) -> Result<ShadowHybridRetrievalResult, ShadowHybridRetrievalError> {
    let mut total = 0_u128;
    let mut lexical = 0_u32;
    let mut semantic = 0_u32;
    let mut graph = 0_u32;
    let mut ordered = candidate.evidence.clone();
    ordered.sort_by_key(|evidence| evidence.channel);
    for evidence in &ordered {
        let denominator = u128::from(
            evidence
                .rank
                .checked_add(RECIPROCAL_RANK_K)
                .ok_or(ShadowHybridRetrievalError::Overflow)?,
        );
        let numerator = u128::from(evidence.channel.default_weight_ppm())
            .checked_mul(u128::from(evidence.channel_score_ppm))
            .ok_or(ShadowHybridRetrievalError::Overflow)?;
        let contribution = numerator
            .checked_div(denominator)
            .ok_or(ShadowHybridRetrievalError::Overflow)?;
        total = total
            .checked_add(contribution)
            .ok_or(ShadowHybridRetrievalError::Overflow)?;
        match evidence.channel {
            HybridRetrievalChannel::ExactLexical
            | HybridRetrievalChannel::LexicalFts
            | HybridRetrievalChannel::EntityAlias => {
                lexical = lexical.max(evidence.channel_score_ppm);
            }
            HybridRetrievalChannel::SemanticVector => {
                semantic = semantic.max(evidence.channel_score_ppm);
            }
            HybridRetrievalChannel::KnowledgeGraph => {
                graph = graph.max(evidence.channel_score_ppm);
            }
            HybridRetrievalChannel::Recency => {}
        }
    }

    let channel_count =
        u32::try_from(ordered.len()).map_err(|_| ShadowHybridRetrievalError::Overflow)?;
    let normalization = u128::from(channel_count)
        .checked_mul(u128::from(SCORE_SCALE_PPM))
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .checked_div(u128::from(RECIPROCAL_RANK_K + 1))
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .max(1);
    let fused_channel_ppm = total
        .checked_mul(u128::from(SCORE_SCALE_PPM))
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .checked_div(normalization)
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .min(u128::from(SCORE_SCALE_PPM));

    let truth_multiplier_ppm = match candidate.truth_status {
        HybridTruthStatus::Confirmed => 1_000_000_u128,
        HybridTruthStatus::Grounded => 950_000_u128,
        HybridTruthStatus::Candidate => 800_000_u128,
        HybridTruthStatus::Disputed
        | HybridTruthStatus::Contradicted
        | HybridTruthStatus::Expired => 0_u128,
    };
    let grounding_multiplier_ppm = match candidate.grounding_status {
        HybridGroundingStatus::GroundedV1 => 1_000_000_u128,
        HybridGroundingStatus::BackfilledGroundedV1 => 950_000_u128,
        HybridGroundingStatus::LegacyUnreviewed => 700_000_u128,
        HybridGroundingStatus::ZeroFact => 900_000_u128,
    };

    let quality_ppm = u128::from(candidate.source_reliability_ppm)
        .checked_mul(3)
        .and_then(|value| {
            value.checked_add(u128::from(candidate.freshness_ppm).checked_mul(2)?)
        })
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .checked_div(5)
        .ok_or(ShadowHybridRetrievalError::Overflow)?;

    let fused = fused_channel_ppm
        .checked_mul(7)
        .and_then(|value| value.checked_add(quality_ppm.checked_mul(3)?))
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .checked_div(10)
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .checked_mul(truth_multiplier_ppm)
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .checked_div(u128::from(SCORE_SCALE_PPM))
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .checked_mul(grounding_multiplier_ppm)
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .checked_div(u128::from(SCORE_SCALE_PPM))
        .ok_or(ShadowHybridRetrievalError::Overflow)?
        .min(u128::from(SCORE_SCALE_PPM));
    let fused_score_ppm =
        u32::try_from(fused).map_err(|_| ShadowHybridRetrievalError::Overflow)?;

    let candidate_sha256 = scored_candidate_digest(
        candidate,
        fused_score_ppm,
        lexical,
        semantic,
        graph,
        channel_count,
    );
    Ok(ShadowHybridRetrievalResult {
        candidate_id: candidate.candidate_id.clone(),
        memory_revision: candidate.memory_revision,
        fused_score_ppm,
        lexical_score_ppm: lexical,
        semantic_score_ppm: semantic,
        graph_score_ppm: graph,
        freshness_ppm: candidate.freshness_ppm,
        source_reliability_ppm: candidate.source_reliability_ppm,
        grounding_status: candidate.grounding_status,
        truth_status: candidate.truth_status,
        estimated_context_tokens: candidate.estimated_context_tokens,
        channel_count,
        candidate_sha256,
    })
}

fn plan_digest(plan: &ShadowHybridRetrievalPlan) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, b"hepta:intelligence:hybrid-retrieval-plan:v1");
    frame_part(&mut hasher, &plan.schema_version.to_be_bytes());
    frame_part(&mut hasher, plan.namespace.as_bytes());
    frame_part(&mut hasher, plan.query_sha256.as_str().as_bytes());
    frame_part(&mut hasher, plan.scope_key_sha256.as_str().as_bytes());
    frame_part(&mut hasher, plan.intent.as_str().as_bytes());
    frame_part(&mut hasher, plan.risk_class.as_str().as_bytes());
    frame_string_list(&mut hasher, &plan.lexical_terms);
    frame_string_list(&mut hasher, &plan.entities);
    match &plan.semantic_query_sha256 {
        Some(digest) => frame_part(&mut hasher, digest.as_str().as_bytes()),
        None => frame_part(&mut hasher, b""),
    }
    match plan.required_time_range {
        Some((start, end)) => {
            frame_part(&mut hasher, &start.to_be_bytes());
            frame_part(&mut hasher, &end.to_be_bytes());
        }
        None => {
            frame_part(&mut hasher, b"");
            frame_part(&mut hasher, b"");
        }
    }
    frame_part(
        &mut hasher,
        &u64::try_from(plan.enabled_channels.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for channel in &plan.enabled_channels {
        frame_part(&mut hasher, channel.as_str().as_bytes());
    }
    frame_part(&mut hasher, &plan.max_results.to_be_bytes());
    frame_part(&mut hasher, &plan.max_context_tokens.to_be_bytes());
    frame_part(&mut hasher, plan.tokenizer_estimator.as_str().as_bytes());
    frame_part(&mut hasher, &[0, 0, 0, 0, 0, 0, 0]);
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn candidate_set_digest(candidates: &[ShadowHybridRetrievalCandidateDraft]) -> Sha256Digest {
    let mut ordered = candidates.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| {
        left.candidate_id
            .cmp(&right.candidate_id)
            .then_with(|| left.memory_revision.cmp(&right.memory_revision))
    });
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-candidate-set:v1",
    );
    frame_part(
        &mut hasher,
        &u64::try_from(ordered.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for candidate in ordered {
        frame_candidate(&mut hasher, candidate);
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn scored_candidate_digest(
    candidate: &ShadowHybridRetrievalCandidateDraft,
    fused_score_ppm: u32,
    lexical_score_ppm: u32,
    semantic_score_ppm: u32,
    graph_score_ppm: u32,
    channel_count: u32,
) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-scored-candidate:v1",
    );
    frame_candidate(&mut hasher, candidate);
    frame_part(&mut hasher, &fused_score_ppm.to_be_bytes());
    frame_part(&mut hasher, &lexical_score_ppm.to_be_bytes());
    frame_part(&mut hasher, &semantic_score_ppm.to_be_bytes());
    frame_part(&mut hasher, &graph_score_ppm.to_be_bytes());
    frame_part(&mut hasher, &channel_count.to_be_bytes());
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn selected_result_digest(results: &[ShadowHybridRetrievalResult]) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-selected-results:v1",
    );
    frame_part(
        &mut hasher,
        &u64::try_from(results.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for result in results {
        frame_part(&mut hasher, result.candidate_id.as_bytes());
        frame_part(&mut hasher, &result.memory_revision.to_be_bytes());
        frame_part(&mut hasher, &result.fused_score_ppm.to_be_bytes());
        frame_part(&mut hasher, result.candidate_sha256.as_str().as_bytes());
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn receipt_digest(receipt: &ShadowHybridRetrievalReceipt) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-receipt:v1",
    );
    frame_part(&mut hasher, &receipt.schema_version.to_be_bytes());
    frame_part(&mut hasher, receipt.namespace.as_bytes());
    frame_part(&mut hasher, receipt.plan_sha256.as_str().as_bytes());
    frame_part(
        &mut hasher,
        receipt.candidate_set_sha256.as_str().as_bytes(),
    );
    frame_part(
        &mut hasher,
        receipt.selected_result_sha256.as_str().as_bytes(),
    );
    frame_part(&mut hasher, &receipt.input_candidate_count.to_be_bytes());
    frame_part(
        &mut hasher,
        &receipt.eligible_candidate_count.to_be_bytes(),
    );
    frame_part(
        &mut hasher,
        &receipt.selected_candidate_count.to_be_bytes(),
    );
    frame_part(
        &mut hasher,
        &receipt.rejected_inactive_count.to_be_bytes(),
    );
    frame_part(&mut hasher, &receipt.rejected_truth_count.to_be_bytes());
    frame_part(
        &mut hasher,
        &receipt.rejected_grounding_count.to_be_bytes(),
    );
    frame_part(&mut hasher, &receipt.rejected_secret_count.to_be_bytes());
    frame_part(&mut hasher, &receipt.rejected_budget_count.to_be_bytes());
    frame_part(
        &mut hasher,
        &receipt.selected_context_tokens.to_be_bytes(),
    );
    frame_part(
        &mut hasher,
        receipt.tokenizer_estimator.as_str().as_bytes(),
    );
    frame_part(&mut hasher, &[0; 11]);
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn validate_receipt(
    receipt: &ShadowHybridRetrievalReceipt,
) -> Result<(), ShadowHybridRetrievalError> {
    if receipt.schema_version != SHADOW_HYBRID_RETRIEVAL_SCHEMA_VERSION
        || receipt.namespace != SHADOW_HYBRID_RETRIEVAL_NAMESPACE
    {
        return Err(ShadowHybridRetrievalError::Invalid(
            "unsupported hybrid retrieval receipt contract".to_string(),
        ));
    }
    if receipt.exact_tokenizer_available
        || receipt.vector_backend_registered
        || receipt.reranker_registered
        || receipt.runtime_wired
        || receipt.default_recall_changed
        || receipt.context_attachment
        || receipt.physical_send
        || receipt.external_effects
        || receipt.production_authority
        || receipt.operator_acceptance
        || receipt.promotion
    {
        return Err(ShadowHybridRetrievalError::Invalid(
            "hybrid retrieval receipt crosses its source-only authority boundary".to_string(),
        ));
    }
    if receipt.selected_candidate_count
        != u32::try_from(receipt.results.len())
            .map_err(|_| ShadowHybridRetrievalError::Overflow)?
        || receipt.selected_result_sha256 != selected_result_digest(&receipt.results)
        || receipt.receipt_sha256 != receipt_digest(receipt)
    {
        return Err(ShadowHybridRetrievalError::Invalid(
            "hybrid retrieval receipt digest or result count mismatch".to_string(),
        ));
    }
    Ok(())
}

fn frame_string_list(hasher: &mut Sha256, values: &[String]) {
    frame_part(
        hasher,
        &u64::try_from(values.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for value in values {
        frame_part(hasher, value.as_bytes());
    }
}

fn frame_candidate(hasher: &mut Sha256, candidate: &ShadowHybridRetrievalCandidateDraft) {
    frame_part(hasher, candidate.candidate_id.as_bytes());
    frame_part(hasher, &candidate.memory_revision.to_be_bytes());
    frame_part(hasher, candidate.lifecycle.as_str().as_bytes());
    frame_part(hasher, candidate.grounding_status.as_str().as_bytes());
    frame_part(hasher, candidate.truth_status.as_str().as_bytes());
    frame_part(hasher, &candidate.source_reliability_ppm.to_be_bytes());
    frame_part(hasher, &candidate.freshness_ppm.to_be_bytes());
    frame_part(hasher, &candidate.estimated_context_tokens.to_be_bytes());
    frame_part(hasher, &[u8::from(candidate.secret_like)]);
    let mut evidence = candidate.evidence.clone();
    evidence.sort_by_key(|item| item.channel);
    frame_part(
        hasher,
        &u64::try_from(evidence.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for item in evidence {
        frame_part(hasher, item.channel.as_str().as_bytes());
        frame_part(hasher, &item.rank.to_be_bytes());
        frame_part(hasher, &item.channel_score_ppm.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(risk_class: HybridRetrievalRiskClass) -> ShadowHybridRetrievalQueryDraft {
        ShadowHybridRetrievalQueryDraft {
            query: "Project Aurora 的 Rust deployment deadline".to_string(),
            scope_key: "agent-private".to_string(),
            intent: HybridRetrievalIntent::RecallFact,
            risk_class,
            lexical_terms: vec![
                "Rust".to_string(),
                "project   aurora".to_string(),
                "rust".to_string(),
            ],
            entities: vec!["Project Aurora".to_string()],
            semantic_query: Some("Aurora deployment deadline and language".to_string()),
            required_time_range: None,
            max_results: 4,
            max_context_tokens: 512,
        }
    }

    fn evidence(
        channel: HybridRetrievalChannel,
        rank: u32,
        score: u32,
    ) -> HybridChannelEvidenceDraft {
        HybridChannelEvidenceDraft {
            channel,
            rank,
            channel_score_ppm: score,
        }
    }

    fn candidate(
        id: &str,
        grounding_status: HybridGroundingStatus,
        truth_status: HybridTruthStatus,
        tokens: u32,
        evidence: Vec<HybridChannelEvidenceDraft>,
    ) -> ShadowHybridRetrievalCandidateDraft {
        ShadowHybridRetrievalCandidateDraft {
            candidate_id: id.to_string(),
            memory_revision: 1,
            lifecycle: HybridLifecycle::Active,
            grounding_status,
            truth_status,
            source_reliability_ppm: 900_000,
            freshness_ppm: 850_000,
            estimated_context_tokens: tokens,
            secret_like: false,
            evidence,
        }
    }

    #[test]
    fn planner_is_deterministic_multilingual_and_authority_negative() {
        let first = plan_shadow_hybrid_retrieval(&query(HybridRetrievalRiskClass::Standard))
            .expect("plan");
        let second = plan_shadow_hybrid_retrieval(&query(HybridRetrievalRiskClass::Standard))
            .expect("plan");
        assert_eq!(first, second);
        assert_eq!(
            first.lexical_terms,
            vec!["project aurora".to_string(), "rust".to_string()]
        );
        assert!(
            first
                .enabled_channels
                .contains(&HybridRetrievalChannel::SemanticVector)
        );
        assert!(
            first
                .enabled_channels
                .contains(&HybridRetrievalChannel::KnowledgeGraph)
        );
        assert!(!first.vector_backend_registered);
        assert!(!first.reranker_registered);
        assert!(!first.runtime_wired);
        assert!(!first.default_recall_changed);
        assert!(!first.context_attachment);
        assert!(!first.physical_send);
        assert!(!first.production_authority);
    }

    #[test]
    fn high_risk_excludes_legacy_and_graph_requires_grounding() {
        let plan = plan_shadow_hybrid_retrieval(&query(HybridRetrievalRiskClass::High))
            .expect("plan");
        let grounded = candidate(
            "grounded",
            HybridGroundingStatus::GroundedV1,
            HybridTruthStatus::Confirmed,
            100,
            vec![evidence(
                HybridRetrievalChannel::KnowledgeGraph,
                1,
                950_000,
            )],
        );
        let legacy = candidate(
            "legacy",
            HybridGroundingStatus::LegacyUnreviewed,
            HybridTruthStatus::Confirmed,
            100,
            vec![evidence(
                HybridRetrievalChannel::LexicalFts,
                1,
                950_000,
            )],
        );
        let receipt =
            fuse_shadow_hybrid_candidates(&plan, &[legacy, grounded]).expect("fusion");
        assert_eq!(receipt.selected_candidate_count, 1);
        assert_eq!(receipt.results[0].candidate_id, "grounded");
        assert_eq!(receipt.rejected_grounding_count, 1);
    }

    #[test]
    fn contradiction_secret_lifecycle_and_budget_fail_closed() {
        let mut draft = query(HybridRetrievalRiskClass::Standard);
        draft.max_context_tokens = 100;
        let plan = plan_shadow_hybrid_retrieval(&draft).expect("plan");
        let contradicted = candidate(
            "contradicted",
            HybridGroundingStatus::GroundedV1,
            HybridTruthStatus::Contradicted,
            10,
            vec![evidence(
                HybridRetrievalChannel::LexicalFts,
                1,
                990_000,
            )],
        );
        let mut secret = candidate(
            "secret",
            HybridGroundingStatus::GroundedV1,
            HybridTruthStatus::Confirmed,
            10,
            vec![evidence(
                HybridRetrievalChannel::LexicalFts,
                1,
                980_000,
            )],
        );
        secret.secret_like = true;
        let mut inactive = candidate(
            "inactive",
            HybridGroundingStatus::GroundedV1,
            HybridTruthStatus::Confirmed,
            10,
            vec![evidence(
                HybridRetrievalChannel::LexicalFts,
                1,
                970_000,
            )],
        );
        inactive.lifecycle = HybridLifecycle::Tombstoned;
        let over_budget = candidate(
            "over-budget",
            HybridGroundingStatus::GroundedV1,
            HybridTruthStatus::Confirmed,
            101,
            vec![evidence(
                HybridRetrievalChannel::LexicalFts,
                1,
                960_000,
            )],
        );
        let receipt = fuse_shadow_hybrid_candidates(
            &plan,
            &[contradicted, secret, inactive, over_budget],
        )
        .expect("fusion");
        assert_eq!(receipt.selected_candidate_count, 0);
        assert_eq!(receipt.rejected_truth_count, 1);
        assert_eq!(receipt.rejected_secret_count, 1);
        assert_eq!(receipt.rejected_inactive_count, 1);
        assert_eq!(receipt.rejected_budget_count, 1);
    }

    #[test]
    fn fusion_is_stable_and_does_not_claim_a_reranker_or_send() {
        let plan = plan_shadow_hybrid_retrieval(&query(HybridRetrievalRiskClass::Standard))
            .expect("plan");
        let first = candidate(
            "first",
            HybridGroundingStatus::GroundedV1,
            HybridTruthStatus::Confirmed,
            100,
            vec![
                evidence(HybridRetrievalChannel::ExactLexical, 1, 950_000),
                evidence(HybridRetrievalChannel::SemanticVector, 2, 900_000),
            ],
        );
        let second = candidate(
            "second",
            HybridGroundingStatus::GroundedV1,
            HybridTruthStatus::Grounded,
            100,
            vec![
                evidence(HybridRetrievalChannel::LexicalFts, 2, 900_000),
                evidence(HybridRetrievalChannel::Recency, 1, 950_000),
            ],
        );
        let forward = fuse_shadow_hybrid_candidates(&plan, &[first.clone(), second.clone()])
            .expect("fusion");
        let reverse =
            fuse_shadow_hybrid_candidates(&plan, &[second, first]).expect("fusion");
        assert_eq!(forward.results, reverse.results);
        assert_eq!(
            forward.selected_result_sha256,
            reverse.selected_result_sha256
        );
        assert!(!forward.reranker_registered);
        assert!(!forward.context_attachment);
        assert!(!forward.physical_send);
        assert!(!forward.production_authority);
    }

    #[test]
    fn receipt_tamper_is_detected() {
        let plan = plan_shadow_hybrid_retrieval(&query(HybridRetrievalRiskClass::Low))
            .expect("plan");
        let selected = candidate(
            "selected",
            HybridGroundingStatus::LegacyUnreviewed,
            HybridTruthStatus::Candidate,
            50,
            vec![evidence(
                HybridRetrievalChannel::LexicalFts,
                1,
                900_000,
            )],
        );
        let mut receipt =
            fuse_shadow_hybrid_candidates(&plan, &[selected]).expect("fusion");
        receipt.production_authority = true;
        assert!(validate_receipt(&receipt).is_err());
    }

    #[test]
    fn byte_upper_bound_is_conservative_for_multibyte_text() {
        let text = "中文 retrieval";
        let estimated = estimate_tokens_utf8_upper_bound(text).expect("estimate");
        assert_eq!(
            estimated,
            u32::try_from(text.len()).expect("text byte length")
        );
        assert!(
            estimated
                >= u32::try_from(text.chars().count()).expect("text character count")
        );
    }

    #[test]
    fn duplicate_channel_evidence_is_rejected() {
        let plan = plan_shadow_hybrid_retrieval(&query(HybridRetrievalRiskClass::Low))
            .expect("plan");
        let duplicate = candidate(
            "duplicate",
            HybridGroundingStatus::GroundedV1,
            HybridTruthStatus::Confirmed,
            50,
            vec![
                evidence(HybridRetrievalChannel::LexicalFts, 1, 900_000),
                evidence(HybridRetrievalChannel::LexicalFts, 2, 800_000),
            ],
        );
        assert!(fuse_shadow_hybrid_candidates(&plan, &[duplicate]).is_err());
    }
}
