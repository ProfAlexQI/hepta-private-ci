//! P1.1a source-only hybrid retrieval planner and deterministic fusion engine.
//!
//! This module is deliberately opt-in. It does not replace the existing
//! lexical/RRF retrieval path, attach memory to a model request, execute a
//! local embedding model, open an ANN index, change physical-send
//! revalidation, or grant production authority.
//!
//! The adapter combines the existing bounded retrieval result with an
//! optional host-supplied, query-bound semantic candidate batch. Every
//! semantic batch is digest-bound to the exact query, model, tokenizer, index
//! generation, candidate identity, rank, and similarity. Host-supplied
//! semantic evidence is still untrusted evidence: this tranche verifies its
//! integrity contract, not model efficacy or index provenance.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use unicode_segmentation::UnicodeSegmentation;

use crate::CognitiveAccess;
use crate::CognitiveScope;
use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::MemoryExplanation;
use crate::MemoryRevalidationBinding;
use crate::MemoryRevisionRecord;
use crate::RetrievalBatch;
use crate::RetrievalChannel;
use crate::RetrievalRequest;
use crate::RevalidationDrift;
use crate::RevalidationStatus;
use crate::SourceRevalidationBinding;
use crate::StableMemoryId;
use crate::MAX_RETRIEVAL_QUERY_BYTES;

use super::frame_part;

pub(crate) const HYBRID_RETRIEVAL_V2_SCHEMA_VERSION: u32 = 1;
pub(crate) const HYBRID_RETRIEVAL_V2_NAMESPACE: &str =
    "hybrid_retrieval_v2_source_only";
pub(crate) const HYBRID_RETRIEVAL_V2_RUNTIME_WIRED: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_DEFAULT_RETRIEVAL_CHANGED: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_ATTACHMENT_COMPILER_CHANGED: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_PHYSICAL_SEND_CHANGED: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_LOCAL_EMBEDDING_EXECUTED: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_ANN_INDEX_EXECUTED: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_GROUNDING_FILTER_APPLIED: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_TRUTH_FILTER_APPLIED: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_EXTERNAL_EFFECTS: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_PRODUCTION_AUTHORITY: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_OPERATOR_ACCEPTANCE: bool = false;
pub(crate) const HYBRID_RETRIEVAL_V2_PROMOTION: bool = false;

const MAX_HYBRID_REQUEST_BYTES: usize = 512 * 1024;
const MAX_HYBRID_RESULTS: usize = 8;
const MAX_SEMANTIC_CANDIDATES: usize = 32;
const MAX_PLANNER_TERMS: usize = 32;
const MAX_EMBEDDING_DIMENSIONS: u32 = 65_536;
const SCORE_SCALE_PPM: u32 = 1_000_000;
const LEXICAL_WEIGHT_PPM: u32 = 350_000;
const SEMANTIC_WEIGHT_PPM: u32 = 450_000;
const CHANNEL_DIVERSITY_WEIGHT_PPM: u32 = 100_000;
const FRESHNESS_WEIGHT_PPM: u32 = 100_000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct HybridRetrievalRequestDraft {
    query: String,
    now_unix_seconds: i64,
    max_results: Option<u32>,
    semantic: Option<SemanticCandidateBatchDraft>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum SemanticSimilarityMetric {
    CosineSimilarityPpm,
}

impl SemanticSimilarityMetric {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::CosineSimilarityPpm => "cosine_similarity_ppm",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SemanticCandidateBatchDraft {
    query_sha256: String,
    model_sha256: String,
    tokenizer_sha256: String,
    index_sha256: String,
    index_generation: u64,
    embedding_dimensions: u32,
    metric: SemanticSimilarityMetric,
    batch_sha256: String,
    candidates: Vec<SemanticCandidateDraft>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SemanticCandidateDraft {
    memory_id: String,
    revision: u64,
    rank: u32,
    similarity_ppm: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum QueryIntent {
    Lookup,
    Causal,
    Procedural,
    Comparative,
    Temporal,
    General,
}

impl QueryIntent {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Lookup => "lookup",
            Self::Causal => "causal",
            Self::Procedural => "procedural",
            Self::Comparative => "comparative",
            Self::Temporal => "temporal",
            Self::General => "general",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum RequestedTemporalScope {
    Current,
    Historical,
    Unspecified,
}

impl RequestedTemporalScope {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Historical => "historical",
            Self::Unspecified => "unspecified",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum QueryLanguageProfile {
    LatinDominant,
    CjkDominant,
    MixedLatinCjk,
    Other,
}

impl QueryLanguageProfile {
    const fn as_str(self) -> &'static str {
        match self {
            Self::LatinDominant => "latin_dominant",
            Self::CjkDominant => "cjk_dominant",
            Self::MixedLatinCjk => "mixed_latin_cjk",
            Self::Other => "other",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct QueryPlannerReceipt {
    schema_version: u32,
    namespace: &'static str,
    query_sha256: String,
    normalized_query_sha256: String,
    lexical_terms: Vec<String>,
    lexical_terms_sha256: String,
    semantic_query_sha256: String,
    intent: QueryIntent,
    requested_temporal_scope: RequestedTemporalScope,
    language_profile: QueryLanguageProfile,
    planner_sha256: String,
    deterministic: bool,
    model_called: bool,
    query_persisted: bool,
    external_effects: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CandidateKey {
    memory_id: String,
    revision: u64,
}

impl CandidateKey {
    fn from_memory(memory: &MemoryRevisionRecord) -> Self {
        Self {
            memory_id: memory.id.memory_id.as_str().to_string(),
            revision: memory.id.revision,
        }
    }
}

#[derive(Clone)]
struct CandidateAccumulator {
    memory: MemoryRevisionRecord,
    revalidation: MemoryRevalidationBinding,
    lexical_baseline_position: Option<u32>,
    baseline_rrf_score: Option<u64>,
    baseline_channels: BTreeSet<RetrievalChannel>,
    semantic_rank: Option<u32>,
    similarity_ppm: Option<u32>,
    semantic_evidence_sha256: Option<Sha256Digest>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct HybridCandidateReceipt {
    memory_id: String,
    revision: u64,
    scope: CognitiveScope,
    content_sha256: String,
    lexical_baseline_position: Option<u32>,
    baseline_rrf_score: Option<u64>,
    baseline_channels: Vec<RetrievalChannel>,
    semantic_rank: Option<u32>,
    semantic_similarity_ppm: Option<u32>,
    semantic_evidence_sha256: Option<String>,
    lexical_signal_ppm: u32,
    semantic_signal_ppm: u32,
    channel_diversity_ppm: u32,
    freshness_ppm: u32,
    fused_score_ppm: u32,
    revalidation_binding_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ExcludedCandidateReceipt {
    memory_id: String,
    revision: u64,
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct HybridRetrievalShadowReceipt {
    schema_version: u32,
    namespace: &'static str,
    query_sha256: String,
    planner: QueryPlannerReceipt,
    lexical_baseline_query_sha256: String,
    lexical_baseline_executed: bool,
    semantic_channel_present: bool,
    semantic_batch_sha256: Option<String>,
    semantic_model_sha256: Option<String>,
    semantic_tokenizer_sha256: Option<String>,
    semantic_index_sha256: Option<String>,
    semantic_index_generation: Option<u64>,
    semantic_metric: Option<String>,
    semantic_host_supplied_untrusted: bool,
    semantic_index_provenance_verified: bool,
    candidate_union_count: u32,
    candidate_count: u32,
    excluded_count: u32,
    candidates: Vec<HybridCandidateReceipt>,
    excluded: Vec<ExcludedCandidateReceipt>,
    fusion_contract_sha256: String,
    receipt_sha256: String,
    deterministic_fallback_used: bool,
    candidate_union_single_snapshot: bool,
    physical_send_revalidation_required: bool,
    runtime_wired: bool,
    default_retrieval_changed: bool,
    attachment_compiler_changed: bool,
    physical_send_changed: bool,
    local_embedding_executed: bool,
    ann_index_executed: bool,
    grounding_filter_applied: bool,
    truth_filter_applied: bool,
    external_effects: bool,
    production_authority: bool,
    operator_acceptance: bool,
    promotion: bool,
}

impl CognitiveStore {
    /// Returns the deterministic P1.1a query plan without reading the store.
    pub fn plan_shadow_hybrid_retrieval_v2(
        request_json: &str,
    ) -> Result<String, CognitiveStoreError> {
        let request = parse_request(request_json)?;
        validate_request(&request)?;
        validate_semantic_batch(request.semantic.as_ref(), &request.query)?;
        serialize_json(&plan_query(&request.query))
    }

    /// Executes an opt-in shadow hybrid union. The existing retrieval method
    /// remains the lexical baseline and is not replaced or reconfigured.
    pub async fn shadow_hybrid_retrieve_v2(
        &self,
        access: &CognitiveAccess,
        request_json: &str,
    ) -> Result<String, CognitiveStoreError> {
        let request = parse_request(request_json)?;
        let max_results = validate_request(&request)?;
        let semantic = validate_semantic_batch(request.semantic.as_ref(), &request.query)?;
        let planner = plan_query(&request.query);
        let lexical_has_terms = !planner.lexical_terms.is_empty();
        if !lexical_has_terms && semantic.is_none() {
            return Err(CognitiveStoreError::Invalid(
                "hybrid retrieval requires lexical terms or a semantic candidate batch"
                    .to_string(),
            ));
        }

        let (baseline, lexical_baseline_executed) = if lexical_has_terms {
            match self
                .retrieve_memory_candidates(
                    access,
                    &RetrievalRequest::new(request.query.clone(), request.now_unix_seconds),
                )
                .await
            {
                Ok(batch) => (batch, true),
                Err(CognitiveStoreError::Invalid(message))
                    if semantic.is_some() && message.contains("no searchable terms") =>
                {
                    (empty_baseline(&request.query), false)
                }
                Err(error) => return Err(error),
            }
        } else {
            (empty_baseline(&request.query), false)
        };

        let mut excluded = Vec::new();
        let mut union = BTreeMap::<CandidateKey, CandidateAccumulator>::new();
        for (index, candidate) in baseline.candidates.iter().enumerate() {
            let position = u32::try_from(index + 1).map_err(|_| {
                CognitiveStoreError::Invalid(
                    "lexical baseline position exceeds u32".to_string(),
                )
            })?;
            let key = CandidateKey::from_memory(&candidate.memory);
            union.insert(
                key,
                CandidateAccumulator {
                    memory: candidate.memory.clone(),
                    revalidation: candidate.revalidation.clone(),
                    lexical_baseline_position: Some(position),
                    baseline_rrf_score: Some(candidate.reciprocal_rank_score),
                    baseline_channels: candidate.channels.iter().copied().collect(),
                    semantic_rank: None,
                    similarity_ppm: None,
                    semantic_evidence_sha256: None,
                },
            );
        }

        if let Some(semantic_batch) = semantic.as_ref() {
            for candidate in &semantic_batch.candidates {
                let memory_id = StableMemoryId::parse(candidate.memory_id.clone())
                    .map_err(CognitiveStoreError::Invalid)?;
                let explanation = match self.explain_memory_head(access, &memory_id).await {
                    Ok(explanation) => explanation,
                    Err(CognitiveStoreError::Invalid(_)) => {
                        excluded.push(ExcludedCandidateReceipt {
                            memory_id: candidate.memory_id.clone(),
                            revision: candidate.revision,
                            reason: "semantic_memory_unavailable".to_string(),
                        });
                        continue;
                    }
                    Err(error) => return Err(error),
                };
                if explanation.memory.id.revision != candidate.revision {
                    excluded.push(ExcludedCandidateReceipt {
                        memory_id: candidate.memory_id.clone(),
                        revision: candidate.revision,
                        reason: "semantic_head_revision_drift".to_string(),
                    });
                    continue;
                }
                let key = CandidateKey::from_memory(&explanation.memory);
                let evidence = semantic_candidate_evidence_digest(semantic_batch, candidate);
                let binding = binding_from_explanation(&explanation);
                match union.get_mut(&key) {
                    Some(existing) => {
                        if existing.memory.content_sha256 != explanation.memory.content_sha256
                            || existing.revalidation != binding
                        {
                            return Err(CognitiveStoreError::Conflict(
                                "lexical and semantic candidates disagree on one memory binding"
                                    .to_string(),
                            ));
                        }
                        existing.semantic_rank = Some(candidate.rank);
                        existing.similarity_ppm = Some(candidate.similarity_ppm);
                        existing.semantic_evidence_sha256 = Some(evidence);
                    }
                    None => {
                        union.insert(
                            key,
                            CandidateAccumulator {
                                memory: explanation.memory,
                                revalidation: binding,
                                lexical_baseline_position: None,
                                baseline_rrf_score: None,
                                baseline_channels: BTreeSet::new(),
                                semantic_rank: Some(candidate.rank),
                                similarity_ppm: Some(candidate.similarity_ppm),
                                semantic_evidence_sha256: Some(evidence),
                            },
                        );
                    }
                }
            }
        }

        let candidate_union_count = u32::try_from(union.len()).map_err(|_| {
            CognitiveStoreError::Invalid("hybrid candidate union exceeds u32".to_string())
        })?;
        let entries = union.into_iter().collect::<Vec<_>>();
        let bindings = entries
            .iter()
            .map(|(_, candidate)| candidate.revalidation.clone())
            .collect::<Vec<_>>();
        let statuses = self
            .revalidate_memory_candidates(access, &bindings, request.now_unix_seconds)
            .await?;
        if statuses.len() != entries.len() {
            return Err(CognitiveStoreError::Corrupt(
                "hybrid retrieval revalidation cardinality drifted".to_string(),
            ));
        }

        let semantic_present = semantic.is_some();
        let mut candidates = Vec::new();
        for ((key, candidate), status) in entries.into_iter().zip(statuses) {
            match status {
                RevalidationStatus::Current(explanation) => {
                    if explanation.memory.id.memory_id.as_str() != key.memory_id
                        || explanation.memory.id.revision != key.revision
                    {
                        return Err(CognitiveStoreError::Corrupt(
                            "hybrid revalidation returned another memory identity".to_string(),
                        ));
                    }
                    candidates.push(candidate_receipt(
                        candidate,
                        &explanation,
                        request.now_unix_seconds,
                        semantic_present,
                    ));
                }
                RevalidationStatus::Stale(drift) => {
                    excluded.push(ExcludedCandidateReceipt {
                        memory_id: key.memory_id,
                        revision: key.revision,
                        reason: drift_name(drift).to_string(),
                    });
                }
            }
        }

        candidates.sort_by(|left, right| {
            right
                .fused_score_ppm
                .cmp(&left.fused_score_ppm)
                .then_with(|| {
                    left.semantic_rank
                        .unwrap_or(u32::MAX)
                        .cmp(&right.semantic_rank.unwrap_or(u32::MAX))
                })
                .then_with(|| {
                    left.lexical_baseline_position
                        .unwrap_or(u32::MAX)
                        .cmp(&right.lexical_baseline_position.unwrap_or(u32::MAX))
                })
                .then_with(|| left.memory_id.cmp(&right.memory_id))
                .then_with(|| left.revision.cmp(&right.revision))
        });
        candidates.truncate(max_results);
        excluded.sort_by(|left, right| {
            left.memory_id
                .cmp(&right.memory_id)
                .then_with(|| left.revision.cmp(&right.revision))
                .then_with(|| left.reason.cmp(&right.reason))
        });

        let candidate_count = u32::try_from(candidates.len()).map_err(|_| {
            CognitiveStoreError::Invalid("hybrid result count exceeds u32".to_string())
        })?;
        let excluded_count = u32::try_from(excluded.len()).map_err(|_| {
            CognitiveStoreError::Invalid("hybrid excluded count exceeds u32".to_string())
        })?;
        let semantic_batch_sha256 = semantic
            .as_ref()
            .map(|batch| batch.batch_sha256.clone());
        let mut receipt = HybridRetrievalShadowReceipt {
            schema_version: HYBRID_RETRIEVAL_V2_SCHEMA_VERSION,
            namespace: HYBRID_RETRIEVAL_V2_NAMESPACE,
            query_sha256: Sha256Digest::for_bytes(request.query.as_bytes())
                .as_str()
                .to_string(),
            planner,
            lexical_baseline_query_sha256: baseline.query_sha256.as_str().to_string(),
            lexical_baseline_executed,
            semantic_channel_present: semantic_present,
            semantic_batch_sha256,
            semantic_model_sha256: semantic
                .as_ref()
                .map(|batch| batch.model_sha256.clone()),
            semantic_tokenizer_sha256: semantic
                .as_ref()
                .map(|batch| batch.tokenizer_sha256.clone()),
            semantic_index_sha256: semantic
                .as_ref()
                .map(|batch| batch.index_sha256.clone()),
            semantic_index_generation: semantic.as_ref().map(|batch| batch.index_generation),
            semantic_metric: semantic
                .as_ref()
                .map(|batch| batch.metric.as_str().to_string()),
            semantic_host_supplied_untrusted: semantic_present,
            semantic_index_provenance_verified: false,
            candidate_union_count,
            candidate_count,
            excluded_count,
            candidates,
            excluded,
            fusion_contract_sha256: fusion_contract_digest().as_str().to_string(),
            receipt_sha256: Sha256Digest::for_bytes(b"uncomputed")
                .as_str()
                .to_string(),
            deterministic_fallback_used: !semantic_present,
            candidate_union_single_snapshot: false,
            physical_send_revalidation_required: true,
            runtime_wired: HYBRID_RETRIEVAL_V2_RUNTIME_WIRED,
            default_retrieval_changed: HYBRID_RETRIEVAL_V2_DEFAULT_RETRIEVAL_CHANGED,
            attachment_compiler_changed: HYBRID_RETRIEVAL_V2_ATTACHMENT_COMPILER_CHANGED,
            physical_send_changed: HYBRID_RETRIEVAL_V2_PHYSICAL_SEND_CHANGED,
            local_embedding_executed: HYBRID_RETRIEVAL_V2_LOCAL_EMBEDDING_EXECUTED,
            ann_index_executed: HYBRID_RETRIEVAL_V2_ANN_INDEX_EXECUTED,
            grounding_filter_applied: HYBRID_RETRIEVAL_V2_GROUNDING_FILTER_APPLIED,
            truth_filter_applied: HYBRID_RETRIEVAL_V2_TRUTH_FILTER_APPLIED,
            external_effects: HYBRID_RETRIEVAL_V2_EXTERNAL_EFFECTS,
            production_authority: HYBRID_RETRIEVAL_V2_PRODUCTION_AUTHORITY,
            operator_acceptance: HYBRID_RETRIEVAL_V2_OPERATOR_ACCEPTANCE,
            promotion: HYBRID_RETRIEVAL_V2_PROMOTION,
        };
        receipt.receipt_sha256 = hybrid_receipt_digest(&receipt).as_str().to_string();
        serialize_json(&receipt)
    }
}

fn parse_request(value: &str) -> Result<HybridRetrievalRequestDraft, CognitiveStoreError> {
    if value.is_empty()
        || value.len() > MAX_HYBRID_REQUEST_BYTES
        || value.as_bytes().contains(&0)
    {
        return Err(CognitiveStoreError::Invalid(format!(
            "hybrid retrieval request must contain 1..={MAX_HYBRID_REQUEST_BYTES} non-NUL bytes"
        )));
    }
    serde_json::from_str(value).map_err(|error| {
        CognitiveStoreError::Invalid(format!("invalid hybrid retrieval JSON: {error}"))
    })
}

fn validate_request(request: &HybridRetrievalRequestDraft) -> Result<usize, CognitiveStoreError> {
    if request.query.trim().is_empty() || request.query.len() > MAX_RETRIEVAL_QUERY_BYTES {
        return Err(CognitiveStoreError::Invalid(format!(
            "hybrid retrieval query must contain 1..={MAX_RETRIEVAL_QUERY_BYTES} bytes"
        )));
    }
    let max_results = request.max_results.unwrap_or(4);
    if max_results == 0
        || usize::try_from(max_results).ok().is_none()
        || usize::try_from(max_results).unwrap_or(usize::MAX) > MAX_HYBRID_RESULTS
    {
        return Err(CognitiveStoreError::Invalid(format!(
            "hybrid retrieval max_results must be 1..={MAX_HYBRID_RESULTS}"
        )));
    }
    usize::try_from(max_results).map_err(|_| {
        CognitiveStoreError::Invalid("hybrid retrieval max_results exceeds usize".to_string())
    })
}

fn validate_semantic_batch(
    batch: Option<&SemanticCandidateBatchDraft>,
    query: &str,
) -> Result<Option<SemanticCandidateBatchDraft>, CognitiveStoreError> {
    let Some(batch) = batch else {
        return Ok(None);
    };
    if batch.candidates.is_empty() || batch.candidates.len() > MAX_SEMANTIC_CANDIDATES {
        return Err(CognitiveStoreError::Invalid(format!(
            "semantic candidate batch must contain 1..={MAX_SEMANTIC_CANDIDATES} candidates"
        )));
    }
    let expected_query = Sha256Digest::for_bytes(query.as_bytes());
    let query_digest = parse_digest(&batch.query_sha256, "semantic query digest")?;
    if query_digest != expected_query {
        return Err(CognitiveStoreError::Conflict(
            "semantic candidate batch belongs to another query".to_string(),
        ));
    }
    let _ = parse_digest(&batch.model_sha256, "semantic model digest")?;
    let _ = parse_digest(&batch.tokenizer_sha256, "semantic tokenizer digest")?;
    let _ = parse_digest(&batch.index_sha256, "semantic index digest")?;
    let recorded_batch = parse_digest(&batch.batch_sha256, "semantic batch digest")?;
    if batch.index_generation == 0 {
        return Err(CognitiveStoreError::Invalid(
            "semantic index generation must be positive".to_string(),
        ));
    }
    if batch.embedding_dimensions == 0
        || batch.embedding_dimensions > MAX_EMBEDDING_DIMENSIONS
    {
        return Err(CognitiveStoreError::Invalid(format!(
            "embedding dimensions must be 1..={MAX_EMBEDDING_DIMENSIONS}"
        )));
    }
    let mut seen = BTreeSet::new();
    for (index, candidate) in batch.candidates.iter().enumerate() {
        let expected_rank = u32::try_from(index + 1).map_err(|_| {
            CognitiveStoreError::Invalid("semantic rank exceeds u32".to_string())
        })?;
        if candidate.rank != expected_rank {
            return Err(CognitiveStoreError::Invalid(
                "semantic ranks must be contiguous and ordered from one".to_string(),
            ));
        }
        if candidate.revision == 0 {
            return Err(CognitiveStoreError::Invalid(
                "semantic memory revision must be positive".to_string(),
            ));
        }
        if candidate.similarity_ppm > SCORE_SCALE_PPM {
            return Err(CognitiveStoreError::Invalid(format!(
                "semantic similarity must be 0..={SCORE_SCALE_PPM} ppm"
            )));
        }
        StableMemoryId::parse(candidate.memory_id.clone())
            .map_err(CognitiveStoreError::Invalid)?;
        if !seen.insert((candidate.memory_id.clone(), candidate.revision)) {
            return Err(CognitiveStoreError::Invalid(
                "semantic candidate batch contains a duplicate memory revision"
                    .to_string(),
            ));
        }
    }
    let expected_batch = semantic_batch_digest(batch)?;
    if expected_batch != recorded_batch {
        return Err(CognitiveStoreError::Conflict(
            "semantic candidate batch digest does not match its contents".to_string(),
        ));
    }
    Ok(Some(batch.clone()))
}

fn empty_baseline(query: &str) -> RetrievalBatch {
    RetrievalBatch {
        query_sha256: Sha256Digest::for_bytes(query.as_bytes()),
        candidates: Vec::new(),
    }
}

fn plan_query(query: &str) -> QueryPlannerReceipt {
    let normalized = query.split_whitespace().collect::<Vec<_>>().join(" ");
    let lexical_terms = planner_terms(&normalized);
    let intent = query_intent(&normalized);
    let requested_temporal_scope = requested_temporal_scope(&normalized);
    let language_profile = language_profile(&normalized);
    let query_sha256 = Sha256Digest::for_bytes(query.as_bytes());
    let normalized_query_sha256 = Sha256Digest::for_bytes(normalized.as_bytes());
    let lexical_terms_sha256 = terms_digest(&lexical_terms);
    let semantic_query_sha256 = normalized_query_sha256.clone();
    let mut planner = QueryPlannerReceipt {
        schema_version: HYBRID_RETRIEVAL_V2_SCHEMA_VERSION,
        namespace: HYBRID_RETRIEVAL_V2_NAMESPACE,
        query_sha256: query_sha256.as_str().to_string(),
        normalized_query_sha256: normalized_query_sha256.as_str().to_string(),
        lexical_terms,
        lexical_terms_sha256: lexical_terms_sha256.as_str().to_string(),
        semantic_query_sha256: semantic_query_sha256.as_str().to_string(),
        intent,
        requested_temporal_scope,
        language_profile,
        planner_sha256: Sha256Digest::for_bytes(b"uncomputed")
            .as_str()
            .to_string(),
        deterministic: true,
        model_called: false,
        query_persisted: false,
        external_effects: false,
    };
    planner.planner_sha256 = planner_digest(&planner).as_str().to_string();
    planner
}

fn planner_terms(query: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut terms = Vec::new();
    for word in UnicodeSegmentation::unicode_words(query) {
        let normalized = normalize_term(word);
        if normalized.is_empty() || !seen.insert(normalized.clone()) {
            continue;
        }
        terms.push(normalized);
        if terms.len() == MAX_PLANNER_TERMS {
            break;
        }
    }
    terms
}

fn normalize_term(value: &str) -> String {
    value
        .chars()
        .flat_map(char::to_lowercase)
        .filter(|character| character.is_alphanumeric())
        .collect()
}

fn query_intent(query: &str) -> QueryIntent {
    let lower = query.to_lowercase();
    if contains_any(&lower, &["why", "because", "为什么", "原因", "为何"]) {
        QueryIntent::Causal
    } else if contains_any(&lower, &["how", "steps", "procedure", "如何", "怎么", "步骤"]) {
        QueryIntent::Procedural
    } else if contains_any(
        &lower,
        &["compare", "versus", " vs ", "difference", "比较", "对比", "区别"],
    ) {
        QueryIntent::Comparative
    } else if contains_any(
        &lower,
        &["when", "latest", "current", "today", "何时", "什么时候", "最新", "当前"],
    ) {
        QueryIntent::Temporal
    } else if contains_any(
        &lower,
        &["who", "what", "where", "which", "谁", "什么", "哪里", "哪个"],
    ) {
        QueryIntent::Lookup
    } else {
        QueryIntent::General
    }
}

fn requested_temporal_scope(query: &str) -> RequestedTemporalScope {
    let lower = query.to_lowercase();
    if contains_any(
        &lower,
        &["latest", "current", "now", "today", "recent", "最新", "当前", "现在", "最近"],
    ) {
        RequestedTemporalScope::Current
    } else if contains_any(
        &lower,
        &["history", "historical", "past", "previous", "历史", "以前", "过去", "曾经"],
    ) {
        RequestedTemporalScope::Historical
    } else {
        RequestedTemporalScope::Unspecified
    }
}

fn language_profile(query: &str) -> QueryLanguageProfile {
    let mut latin = 0_u32;
    let mut cjk = 0_u32;
    let mut other = 0_u32;
    for character in query.chars().filter(|character| character.is_alphabetic()) {
        if character.is_ascii_alphabetic() {
            latin = latin.saturating_add(1);
        } else if is_cjk(character) {
            cjk = cjk.saturating_add(1);
        } else {
            other = other.saturating_add(1);
        }
    }
    if latin > 0 && cjk > 0 {
        QueryLanguageProfile::MixedLatinCjk
    } else if cjk > latin.saturating_add(other) {
        QueryLanguageProfile::CjkDominant
    } else if latin > cjk.saturating_add(other) {
        QueryLanguageProfile::LatinDominant
    } else {
        QueryLanguageProfile::Other
    }
}

fn is_cjk(character: char) -> bool {
    matches!(
        character as u32,
        0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xF900..=0xFAFF
            | 0x3040..=0x30FF
            | 0xAC00..=0xD7AF
    )
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn candidate_receipt(
    candidate: CandidateAccumulator,
    explanation: &MemoryExplanation,
    now_unix_seconds: i64,
    semantic_channel_present: bool,
) -> HybridCandidateReceipt {
    let lexical_signal_ppm = candidate
        .lexical_baseline_position
        .map(|position| SCORE_SCALE_PPM / position.max(1))
        .unwrap_or(0);
    let semantic_signal_ppm = candidate.similarity_ppm.unwrap_or(0);
    let channel_diversity_ppm = u32::try_from(candidate.baseline_channels.len())
        .unwrap_or(u32::MAX)
        .saturating_mul(250_000)
        .min(SCORE_SCALE_PPM);
    let freshness_ppm = freshness_ppm(
        explanation.memory.valid_from_unix_seconds,
        now_unix_seconds,
    );
    let fused_score_ppm = fused_score_ppm(
        lexical_signal_ppm,
        semantic_signal_ppm,
        channel_diversity_ppm,
        freshness_ppm,
        semantic_channel_present,
    );
    HybridCandidateReceipt {
        memory_id: explanation.memory.id.memory_id.as_str().to_string(),
        revision: explanation.memory.id.revision,
        scope: explanation.memory.scope.clone(),
        content_sha256: explanation.memory.content_sha256.as_str().to_string(),
        lexical_baseline_position: candidate.lexical_baseline_position,
        baseline_rrf_score: candidate.baseline_rrf_score,
        baseline_channels: candidate.baseline_channels.into_iter().collect(),
        semantic_rank: candidate.semantic_rank,
        semantic_similarity_ppm: candidate.similarity_ppm,
        semantic_evidence_sha256: candidate
            .semantic_evidence_sha256
            .map(|digest| digest.as_str().to_string()),
        lexical_signal_ppm,
        semantic_signal_ppm,
        channel_diversity_ppm,
        freshness_ppm,
        fused_score_ppm,
        revalidation_binding_sha256: revalidation_binding_digest(&candidate.revalidation)
            .as_str()
            .to_string(),
    }
}

fn fused_score_ppm(
    lexical_signal_ppm: u32,
    semantic_signal_ppm: u32,
    channel_diversity_ppm: u32,
    freshness_ppm: u32,
    semantic_channel_present: bool,
) -> u32 {
    let semantic_weight = if semantic_channel_present {
        SEMANTIC_WEIGHT_PPM
    } else {
        0
    };
    let total_weight = LEXICAL_WEIGHT_PPM
        .saturating_add(semantic_weight)
        .saturating_add(CHANNEL_DIVERSITY_WEIGHT_PPM)
        .saturating_add(FRESHNESS_WEIGHT_PPM);
    let weighted = u128::from(lexical_signal_ppm) * u128::from(LEXICAL_WEIGHT_PPM)
        + u128::from(semantic_signal_ppm) * u128::from(semantic_weight)
        + u128::from(channel_diversity_ppm) * u128::from(CHANNEL_DIVERSITY_WEIGHT_PPM)
        + u128::from(freshness_ppm) * u128::from(FRESHNESS_WEIGHT_PPM);
    u32::try_from(weighted / u128::from(total_weight.max(1)))
        .unwrap_or(SCORE_SCALE_PPM)
        .min(SCORE_SCALE_PPM)
}

fn freshness_ppm(valid_from: i64, now: i64) -> u32 {
    let age = now.saturating_sub(valid_from).max(0);
    match age {
        0..=86_400 => 1_000_000,
        86_401..=604_800 => 850_000,
        604_801..=2_592_000 => 650_000,
        2_592_001..=15_552_000 => 350_000,
        _ => 150_000,
    }
}

fn binding_from_explanation(explanation: &MemoryExplanation) -> MemoryRevalidationBinding {
    MemoryRevalidationBinding {
        memory: explanation.memory.id.clone(),
        scope: explanation.memory.scope.clone(),
        content_sha256: explanation.memory.content_sha256.clone(),
        verification: explanation.memory.verification,
        lifecycle: explanation.memory.lifecycle.clone(),
        valid_from_unix_seconds: explanation.memory.valid_from_unix_seconds,
        valid_to_unix_seconds: explanation.memory.valid_to_unix_seconds,
        citations: explanation
            .citations
            .iter()
            .map(|source| SourceRevalidationBinding {
                id: source.id.clone(),
                scope: source.scope.clone(),
                content_sha256: source.content_sha256.clone(),
            })
            .collect(),
        kg_projection_generation: explanation.kg_projection_generation,
    }
}

fn drift_name(drift: RevalidationDrift) -> &'static str {
    match drift {
        RevalidationDrift::HeadRevision => "revalidation_head_revision",
        RevalidationDrift::Scope => "revalidation_scope",
        RevalidationDrift::ContentHash => "revalidation_content_hash",
        RevalidationDrift::Verification => "revalidation_verification",
        RevalidationDrift::Lifecycle => "revalidation_lifecycle",
        RevalidationDrift::Validity => "revalidation_validity",
        RevalidationDrift::CitationSet => "revalidation_citation_set",
        RevalidationDrift::SourceHash => "revalidation_source_hash",
        RevalidationDrift::KgProjectionGeneration => "revalidation_kg_generation",
        RevalidationDrift::NotEligible => "revalidation_not_eligible",
    }
}

fn parse_digest(value: &str, label: &str) -> Result<Sha256Digest, CognitiveStoreError> {
    Sha256Digest::parse(value.to_string())
        .map_err(|error| CognitiveStoreError::Invalid(format!("invalid {label}: {error}")))
}

fn semantic_batch_digest(
    batch: &SemanticCandidateBatchDraft,
) -> Result<Sha256Digest, CognitiveStoreError> {
    let query_sha256 = parse_digest(&batch.query_sha256, "semantic query digest")?;
    let model_sha256 = parse_digest(&batch.model_sha256, "semantic model digest")?;
    let tokenizer_sha256 = parse_digest(&batch.tokenizer_sha256, "semantic tokenizer digest")?;
    let index_sha256 = parse_digest(&batch.index_sha256, "semantic index digest")?;
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-semantic-batch:v1",
    );
    frame_part(&mut hasher, query_sha256.as_str().as_bytes());
    frame_part(&mut hasher, model_sha256.as_str().as_bytes());
    frame_part(&mut hasher, tokenizer_sha256.as_str().as_bytes());
    frame_part(&mut hasher, index_sha256.as_str().as_bytes());
    frame_part(&mut hasher, &batch.index_generation.to_be_bytes());
    frame_part(&mut hasher, &batch.embedding_dimensions.to_be_bytes());
    frame_part(&mut hasher, batch.metric.as_str().as_bytes());
    frame_part(
        &mut hasher,
        &u64::try_from(batch.candidates.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for candidate in &batch.candidates {
        frame_part(&mut hasher, candidate.memory_id.as_bytes());
        frame_part(&mut hasher, &candidate.revision.to_be_bytes());
        frame_part(&mut hasher, &candidate.rank.to_be_bytes());
        frame_part(&mut hasher, &candidate.similarity_ppm.to_be_bytes());
    }
    Ok(Sha256Digest::from_sha256_output(hasher.finalize()))
}

fn semantic_candidate_evidence_digest(
    batch: &SemanticCandidateBatchDraft,
    candidate: &SemanticCandidateDraft,
) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-semantic-candidate:v1",
    );
    frame_part(&mut hasher, batch.batch_sha256.as_bytes());
    frame_part(&mut hasher, candidate.memory_id.as_bytes());
    frame_part(&mut hasher, &candidate.revision.to_be_bytes());
    frame_part(&mut hasher, &candidate.rank.to_be_bytes());
    frame_part(&mut hasher, &candidate.similarity_ppm.to_be_bytes());
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn terms_digest(terms: &[String]) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-terms:v1",
    );
    frame_part(
        &mut hasher,
        &u64::try_from(terms.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for term in terms {
        frame_part(&mut hasher, term.as_bytes());
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn planner_digest(planner: &QueryPlannerReceipt) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-planner:v1",
    );
    frame_part(&mut hasher, &planner.schema_version.to_be_bytes());
    frame_part(&mut hasher, planner.namespace.as_bytes());
    frame_part(&mut hasher, planner.query_sha256.as_bytes());
    frame_part(
        &mut hasher,
        planner.normalized_query_sha256.as_bytes(),
    );
    frame_part(&mut hasher, planner.lexical_terms_sha256.as_bytes());
    frame_part(&mut hasher, planner.semantic_query_sha256.as_bytes());
    frame_part(&mut hasher, planner.intent.as_str().as_bytes());
    frame_part(
        &mut hasher,
        planner.requested_temporal_scope.as_str().as_bytes(),
    );
    frame_part(
        &mut hasher,
        planner.language_profile.as_str().as_bytes(),
    );
    frame_part(&mut hasher, &[1, 0, 0, 0]);
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn revalidation_binding_digest(binding: &MemoryRevalidationBinding) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-revalidation:v1",
    );
    frame_part(
        &mut hasher,
        binding.memory.memory_id.as_str().as_bytes(),
    );
    frame_part(&mut hasher, &binding.memory.revision.to_be_bytes());
    frame_part(&mut hasher, binding.scope.projection_key().as_bytes());
    frame_part(&mut hasher, binding.content_sha256.as_str().as_bytes());
    frame_part(
        &mut hasher,
        serde_json::to_string(&binding.verification)
            .unwrap_or_else(|_| "invalid".to_string())
            .as_bytes(),
    );
    frame_part(
        &mut hasher,
        serde_json::to_string(&binding.lifecycle)
            .unwrap_or_else(|_| "invalid".to_string())
            .as_bytes(),
    );
    frame_part(
        &mut hasher,
        &binding.valid_from_unix_seconds.to_be_bytes(),
    );
    match binding.valid_to_unix_seconds {
        Some(value) => frame_part(&mut hasher, &value.to_be_bytes()),
        None => frame_part(&mut hasher, b""),
    }
    frame_part(
        &mut hasher,
        &u64::try_from(binding.citations.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for citation in &binding.citations {
        frame_part(
            &mut hasher,
            citation.id.source_id.as_str().as_bytes(),
        );
        frame_part(&mut hasher, &citation.id.revision.to_be_bytes());
        frame_part(&mut hasher, citation.scope.projection_key().as_bytes());
        frame_part(&mut hasher, citation.content_sha256.as_str().as_bytes());
    }
    match binding.kg_projection_generation {
        Some(generation) => frame_part(&mut hasher, &generation.get().to_be_bytes()),
        None => frame_part(&mut hasher, b""),
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn fusion_contract_digest() -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-fusion:v1",
    );
    for value in [
        SCORE_SCALE_PPM,
        LEXICAL_WEIGHT_PPM,
        SEMANTIC_WEIGHT_PPM,
        CHANNEL_DIVERSITY_WEIGHT_PPM,
        FRESHNESS_WEIGHT_PPM,
    ] {
        frame_part(&mut hasher, &value.to_be_bytes());
    }
    frame_part(&mut hasher, &[0, 0, 0, 0, 0, 0, 0, 0]);
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn hybrid_receipt_digest(receipt: &HybridRetrievalShadowReceipt) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:hybrid-retrieval-receipt:v1",
    );
    frame_part(&mut hasher, &receipt.schema_version.to_be_bytes());
    frame_part(&mut hasher, receipt.namespace.as_bytes());
    frame_part(&mut hasher, receipt.query_sha256.as_bytes());
    frame_part(&mut hasher, receipt.planner.planner_sha256.as_bytes());
    frame_part(
        &mut hasher,
        receipt.lexical_baseline_query_sha256.as_bytes(),
    );
    frame_part(
        &mut hasher,
        receipt
            .semantic_batch_sha256
            .as_deref()
            .unwrap_or("")
            .as_bytes(),
    );
    frame_part(&mut hasher, &receipt.candidate_union_count.to_be_bytes());
    frame_part(&mut hasher, &receipt.candidate_count.to_be_bytes());
    frame_part(&mut hasher, &receipt.excluded_count.to_be_bytes());
    for candidate in &receipt.candidates {
        frame_part(&mut hasher, candidate.memory_id.as_bytes());
        frame_part(&mut hasher, &candidate.revision.to_be_bytes());
        frame_part(&mut hasher, &candidate.fused_score_ppm.to_be_bytes());
        frame_part(
            &mut hasher,
            candidate.revalidation_binding_sha256.as_bytes(),
        );
        frame_part(
            &mut hasher,
            candidate
                .semantic_evidence_sha256
                .as_deref()
                .unwrap_or("")
                .as_bytes(),
        );
    }
    for excluded in &receipt.excluded {
        frame_part(&mut hasher, excluded.memory_id.as_bytes());
        frame_part(&mut hasher, &excluded.revision.to_be_bytes());
        frame_part(&mut hasher, excluded.reason.as_bytes());
    }
    frame_part(&mut hasher, receipt.fusion_contract_sha256.as_bytes());
    frame_part(
        &mut hasher,
        &[
            u8::from(receipt.lexical_baseline_executed),
            u8::from(receipt.semantic_channel_present),
            u8::from(receipt.deterministic_fallback_used),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    );
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn serialize_json<T>(value: &T) -> Result<String, CognitiveStoreError>
where
    T: Serialize,
{
    serde_json::to_string(value).map_err(|error| {
        CognitiveStoreError::Unavailable(format!(
            "serialize hybrid retrieval receipt: {error}"
        ))
    })
}

#[cfg(test)]
#[path = "hybrid_retrieval_v2/tests.rs"]
mod tests;
