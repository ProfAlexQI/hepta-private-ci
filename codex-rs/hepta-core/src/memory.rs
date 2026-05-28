//! Core contracts for session metadata, transcript recall, and retrievable memory.
//!
//! The goal here is to keep the boundary intentionally small so runtimes can
//! swap storage backends without pulling storage details into higher layers.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    TranscriptSpanRef,
    intelligence::{HeptaNeuron, NeuronId, TopicSession, TopicSessionStatus},
    model::MessageRole,
    runtime_types::{AgentId, SessionId},
};
use serde::{Deserialize, Serialize};

const DEFAULT_CONTEXT_RECALL_TRANSCRIPT_PROVENANCE_LIMIT: usize = 8;

/// Declares whether a memory record belongs to a single session or to a wider
/// cross-session corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryScope {
    Session,
    LongTerm,
}

/// Minimal session projection that storage adapters must preserve across
/// snapshots, export/import, and lookup flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRecord {
    pub session_id: SessionId,
    pub agent_id: AgentId,
    pub title: String,
    pub created_at_unix_ms: u64,
    pub last_active_unix_ms: u64,
    pub last_user_intent_summary: Option<String>,
    pub archived_at_unix_ms: Option<u64>,
}

/// Portable memory payload used at the storage boundary.
///
/// Backends can maintain richer indexes internally, but the contract that moves
/// between crates remains this stable representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: String,
    pub scope: MemoryScope,
    pub content: String,
}

/// Semantic class for a promoted memory item.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotedMemoryKind {
    Preference,
    Task,
    Decision,
    Fact,
    Summary,
    #[default]
    Other,
}

/// Provenance attached to a promoted memory without forcing every lightweight
/// memory hit to carry full source payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PromotedMemoryProvenance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_session_id: Option<SessionId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_turn_range: Option<TranscriptRange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_entry_ids: Vec<String>,
}

/// Provenance-aware promoted memory envelope used by Hepta Intelligence.
///
/// `MemoryRecord` remains the portable storage/search payload; this wrapper is
/// the richer contract for durable promoted memories that must cite where they
/// came from and when their confidence was last checked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromotedMemoryRecord {
    pub record: MemoryRecord,
    #[serde(default)]
    pub memory_kind: PromotedMemoryKind,
    #[serde(default)]
    pub provenance: PromotedMemoryProvenance,
    pub confidence: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_revalidated_unix_ms: Option<u64>,
}

/// Runtime role for a memory provider exposed through the Hepta-native memory
/// plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderKind {
    Builtin,
    External,
}

/// Activation state for a provider descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderStatus {
    Active,
    Available,
    Rejected,
}

/// Capability advertised by a memory provider without binding Hepta to any
/// particular third-party backend implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderCapability {
    ProfileCard,
    SemanticSearch,
    Reasoning,
    ContextSnapshot,
    Conclusions,
    Prefetch,
    Sync,
    Delete,
}

/// Stable descriptor for one memory backend in the provider plane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderDescriptor {
    pub id: String,
    pub kind: MemoryProviderKind,
    pub status: MemoryProviderStatus,
    #[serde(default)]
    pub capabilities: Vec<MemoryProviderCapability>,
    #[serde(default)]
    pub tool_names: Vec<String>,
    pub context_fencing_required: bool,
    pub prefetch_enabled: bool,
    pub sync_enabled: bool,
    pub external_exclusive: bool,
    pub provenance_required: bool,
    pub deletion_supported: bool,
    pub summary: String,
}

impl MemoryProviderDescriptor {
    pub fn builtin() -> Self {
        Self {
            id: "builtin".into(),
            kind: MemoryProviderKind::Builtin,
            status: MemoryProviderStatus::Active,
            capabilities: vec![
                MemoryProviderCapability::SemanticSearch,
                MemoryProviderCapability::ContextSnapshot,
                MemoryProviderCapability::Prefetch,
                MemoryProviderCapability::Sync,
                MemoryProviderCapability::Delete,
            ],
            tool_names: vec!["memory".into(), "recall".into()],
            context_fencing_required: true,
            prefetch_enabled: true,
            sync_enabled: true,
            external_exclusive: false,
            provenance_required: true,
            deletion_supported: true,
            summary: "Hepta builtin transcript/memory recall provider".into(),
        }
    }

    pub fn external_slot(id: impl Into<String>, status: MemoryProviderStatus) -> Self {
        Self {
            id: id.into(),
            kind: MemoryProviderKind::External,
            status,
            capabilities: vec![
                MemoryProviderCapability::ProfileCard,
                MemoryProviderCapability::SemanticSearch,
                MemoryProviderCapability::Reasoning,
                MemoryProviderCapability::ContextSnapshot,
                MemoryProviderCapability::Conclusions,
                MemoryProviderCapability::Prefetch,
                MemoryProviderCapability::Sync,
                MemoryProviderCapability::Delete,
            ],
            tool_names: vec![
                "profile".into(),
                "search".into(),
                "reasoning".into(),
                "context".into(),
                "conclude".into(),
            ],
            context_fencing_required: true,
            prefetch_enabled: status == MemoryProviderStatus::Active,
            sync_enabled: status == MemoryProviderStatus::Active,
            external_exclusive: true,
            provenance_required: true,
            deletion_supported: true,
            summary: "External user-modeling provider slot; at most one may be active".into(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == MemoryProviderStatus::Active
    }
}

/// Machine-readable summary of the Hepta-native memory provider plane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProviderPlaneReport {
    pub provider_count: usize,
    pub active_provider_count: usize,
    pub external_provider_count: usize,
    pub active_external_provider_count: usize,
    pub builtin_present: bool,
    pub exactly_one_external_active_or_none: bool,
    pub context_fencing_required: bool,
    pub all_active_providers_prefetch: bool,
    pub all_active_providers_sync: bool,
    pub provenance_required: bool,
    pub deletion_path_available: bool,
    pub capability_count: usize,
    #[serde(default)]
    pub capabilities: Vec<MemoryProviderCapability>,
    #[serde(default)]
    pub providers: Vec<MemoryProviderDescriptor>,
}

impl MemoryProviderPlaneReport {
    pub fn from_providers(providers: Vec<MemoryProviderDescriptor>) -> Self {
        let provider_count = providers.len();
        let active_provider_count = providers
            .iter()
            .filter(|provider| provider.is_active())
            .count();
        let external_provider_count = providers
            .iter()
            .filter(|provider| provider.kind == MemoryProviderKind::External)
            .count();
        let active_external_provider_count = providers
            .iter()
            .filter(|provider| {
                provider.kind == MemoryProviderKind::External && provider.is_active()
            })
            .count();
        let active = providers
            .iter()
            .filter(|provider| provider.is_active())
            .collect::<Vec<_>>();
        let mut capabilities = BTreeSet::new();
        for provider in &providers {
            for capability in &provider.capabilities {
                capabilities.insert(*capability);
            }
        }

        Self {
            provider_count,
            active_provider_count,
            external_provider_count,
            active_external_provider_count,
            builtin_present: providers.iter().any(|provider| {
                provider.kind == MemoryProviderKind::Builtin && provider.is_active()
            }),
            exactly_one_external_active_or_none: active_external_provider_count <= 1,
            context_fencing_required: active
                .iter()
                .all(|provider| provider.context_fencing_required),
            all_active_providers_prefetch: active.iter().all(|provider| provider.prefetch_enabled),
            all_active_providers_sync: active.iter().all(|provider| provider.sync_enabled),
            provenance_required: active.iter().all(|provider| provider.provenance_required),
            deletion_path_available: active.iter().any(|provider| provider.deletion_supported),
            capability_count: capabilities.len(),
            capabilities: capabilities.into_iter().collect(),
            providers,
        }
    }

    pub fn native_default() -> Self {
        Self::from_providers(vec![
            MemoryProviderDescriptor::builtin(),
            MemoryProviderDescriptor::external_slot(
                "external-user-modeling-slot",
                MemoryProviderStatus::Available,
            ),
        ])
    }

    pub fn contract_ready(&self) -> bool {
        self.builtin_present
            && self.exactly_one_external_active_or_none
            && self.context_fencing_required
            && self.all_active_providers_prefetch
            && self.all_active_providers_sync
            && self.provenance_required
            && self.deletion_path_available
    }
}

/// Query contract for memory retrieval.
///
/// Implementations may apply ranking or indexing internally, but they should
/// treat `text` as the caller's retrieval hint and must honor `limit`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub text: String,
    pub limit: usize,
}

/// Compact returned-vs-matched counts derived from a query report.
///
/// This keeps only the aggregate coverage counts that automation and tests
/// often need when they want to reason about clipping without carrying the
/// full memory or transcript hit payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct QueryReportCoverage {
    pub returned_count: usize,
    pub matched_count: usize,
}

impl QueryReportCoverage {
    pub fn omitted_count(&self) -> usize {
        self.matched_count.saturating_sub(self.returned_count)
    }

    pub fn is_complete(&self) -> bool {
        self.returned_count == self.matched_count
    }

    pub fn is_empty(&self) -> bool {
        self.matched_count == 0
    }

    pub fn is_truncated(&self) -> bool {
        self.returned_count < self.matched_count
    }
}

/// Compact omission-focused summary derived from a query report.
///
/// Unlike [`QueryReportCoverage`], this focuses on limit pressure only: whether
/// the result was truncated and how many matched hits were left behind.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct QueryReportLimitPressure {
    pub truncated: bool,
    pub omitted_count: usize,
}

impl QueryReportLimitPressure {
    pub fn from_coverage(coverage: &QueryReportCoverage) -> Self {
        Self {
            truncated: coverage.is_truncated(),
            omitted_count: coverage.omitted_count(),
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.truncated && self.omitted_count == 0
    }

    pub fn is_empty(&self) -> bool {
        self.is_complete()
    }
}

/// Portable top-level report for memory retrieval.
///
/// This gives callers a stable machine-readable envelope for search results
/// without requiring them to infer truncation or total match counts from an
/// implementation-specific backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryQueryReport {
    pub query: MemoryQuery,
    pub matched_count: usize,
    pub returned_count: usize,
    pub truncated: bool,
    #[serde(default)]
    pub hits: Vec<MemoryRecord>,
}

impl MemoryQueryReport {
    pub fn from_hits(query: MemoryQuery, matched_count: usize, hits: Vec<MemoryRecord>) -> Self {
        let returned_count = hits.len();

        Self {
            query,
            matched_count,
            returned_count,
            truncated: returned_count < matched_count,
            hits,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.matched_count == 0
    }

    pub fn has_hits(&self) -> bool {
        !self.is_empty()
    }

    pub fn omitted_count(&self) -> usize {
        self.coverage().omitted_count()
    }

    pub fn is_complete(&self) -> bool {
        self.coverage().is_complete()
    }

    pub fn coverage(&self) -> QueryReportCoverage {
        QueryReportCoverage {
            returned_count: self.returned_count,
            matched_count: self.matched_count,
        }
    }

    pub fn limit_pressure(&self) -> QueryReportLimitPressure {
        QueryReportLimitPressure::from_coverage(&self.coverage())
    }
}

/// Stable transcript entry kind for full-session content storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptEntryKind {
    Message,
    ToolCall,
    ToolResult,
    Approval,
    Summary,
    Event,
}

/// Ordered range inside a single session transcript.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptRange {
    pub start_sequence: u64,
    pub end_sequence: u64,
}

impl TranscriptRange {
    pub fn contains(&self, sequence: u64) -> bool {
        sequence >= self.start_sequence && sequence <= self.end_sequence
    }
}

/// Portable full-fidelity transcript entry used for exact session recall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptEntry {
    pub entry_id: String,
    pub session_id: SessionId,
    pub sequence: u64,
    pub kind: TranscriptEntryKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<MessageRole>,
    pub content: String,
    pub created_at_unix_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_of_range: Option<TranscriptRange>,
}

impl TranscriptEntry {
    pub fn matches_query(&self, query: &TranscriptQuery) -> bool {
        if let Some(session_id) = &query.session_id
            && &self.session_id != session_id
        {
            return false;
        }

        self.content.contains(&query.text)
    }
}

/// Portable recalled transcript span.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptSpan {
    pub session_id: SessionId,
    pub range: TranscriptRange,
    pub entry_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    #[serde(default)]
    pub entries: Vec<TranscriptEntry>,
}

impl TranscriptSpan {
    pub fn from_entry(entry: TranscriptEntry) -> Self {
        Self {
            session_id: entry.session_id.clone(),
            range: TranscriptRange {
                start_sequence: entry.sequence,
                end_sequence: entry.sequence,
            },
            entry_count: 1,
            excerpt: Some(entry.content.clone()),
            entries: vec![entry],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entry_count == 0
    }
}

/// Query contract for transcript retrieval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<SessionId>,
    pub text: String,
    pub limit: usize,
}

/// Machine-readable transcript retrieval report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptQueryReport {
    pub query: TranscriptQuery,
    pub matched_count: usize,
    pub returned_count: usize,
    pub truncated: bool,
    #[serde(default)]
    pub hits: Vec<TranscriptSpan>,
}

impl TranscriptQueryReport {
    pub fn from_hits(
        query: TranscriptQuery,
        matched_count: usize,
        hits: Vec<TranscriptSpan>,
    ) -> Self {
        let returned_count = hits.len();

        Self {
            query,
            matched_count,
            returned_count,
            truncated: returned_count < matched_count,
            hits,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.matched_count == 0
    }

    pub fn has_hits(&self) -> bool {
        !self.is_empty()
    }

    pub fn omitted_count(&self) -> usize {
        self.coverage().omitted_count()
    }

    pub fn is_complete(&self) -> bool {
        self.coverage().is_complete()
    }

    pub fn coverage(&self) -> QueryReportCoverage {
        QueryReportCoverage {
            returned_count: self.returned_count,
            matched_count: self.matched_count,
        }
    }

    pub fn limit_pressure(&self) -> QueryReportLimitPressure {
        QueryReportLimitPressure::from_coverage(&self.coverage())
    }
}

/// Bounded request for blended transcript + memory recall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextRecallRequest {
    pub session_id: SessionId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    pub recent_window_limit: usize,
    pub transcript_limit: usize,
    pub memory_limit: usize,
    /// Advisory widening flag for memory sources.
    ///
    /// Recent entries and transcript hits remain anchored to `session_id`
    /// regardless of this value. Portable `MemoryRecord` payloads do not carry
    /// session ownership, so adapters that cannot distinguish session-local
    /// memory may legitimately return the same memory hits whether this is
    /// enabled or not.
    pub allow_cross_session: bool,
}

/// Global budget used when several recall sources compete for a bounded prompt
/// or routing frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextBudget {
    pub max_items: usize,
    pub max_tokens_estimate: usize,
    pub min_source_diversity: usize,
    pub max_per_source: usize,
}

impl Default for ContextBudget {
    fn default() -> Self {
        Self {
            max_items: 24,
            max_tokens_estimate: 4096,
            min_source_diversity: 3,
            max_per_source: 8,
        }
    }
}

impl ContextBudget {
    pub fn from_request(request: &ContextRecallRequest) -> Self {
        let max_items = request
            .recent_window_limit
            .saturating_add(request.transcript_limit)
            .saturating_add(request.memory_limit)
            .max(1);
        Self {
            max_items,
            max_tokens_estimate: max_items.saturating_mul(256),
            min_source_diversity: 3,
            max_per_source: request
                .recent_window_limit
                .max(request.transcript_limit)
                .max(request.memory_limit)
                .max(1),
        }
    }
}

impl ContextRecallRequest {
    /// Returns the trimmed query text when the request carries a non-blank
    /// search hint.
    pub fn normalized_query_text(&self) -> Option<&str> {
        self.query_text
            .as_deref()
            .map(str::trim)
            .filter(|text| !text.is_empty())
    }

    pub fn has_query_text(&self) -> bool {
        self.normalized_query_text().is_some()
    }

    /// Builds the session-scoped transcript query that corresponds to this
    /// recall request.
    pub fn transcript_query(&self) -> TranscriptQuery {
        TranscriptQuery {
            session_id: Some(self.session_id.clone()),
            text: self.normalized_query_text().unwrap_or_default().to_string(),
            limit: self.transcript_limit,
        }
    }

    /// Builds the memory query that corresponds to this recall request.
    pub fn memory_query(&self) -> MemoryQuery {
        MemoryQuery {
            text: self.normalized_query_text().unwrap_or_default().to_string(),
            limit: self.memory_limit,
        }
    }
}

/// Runtime-facing blended recall bundle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextRecallBundle {
    pub request: ContextRecallRequest,
    #[serde(default)]
    pub recent_entries: Vec<TranscriptEntry>,
    #[serde(default)]
    pub transcript_hits: Vec<TranscriptSpan>,
    #[serde(default)]
    pub durable_memory_hits: Vec<MemoryRecord>,
    #[serde(default)]
    pub summary_hits: Vec<MemoryRecord>,
    #[serde(default)]
    pub active_topic_sessions: Vec<TopicSession>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_neurons: Vec<HeptaNeuron>,
    #[serde(default)]
    pub budget: ContextBudget,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ranked_items: Vec<ContextRecallItem>,
    #[serde(default)]
    pub omitted_by_budget: usize,
    pub truncated: bool,
}

/// Source lane for one ranked recall item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextRecallSource {
    RecentWindow,
    Transcript,
    DurableMemory,
    SummaryMemory,
    ActiveTopicSession,
    ActiveNeuron,
    KnowledgeGraph,
}

/// Explainable score used to rank items inside an intelligence turn frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextRecallScore {
    pub recency: f32,
    pub relevance: f32,
    pub durability: f32,
    pub topic_activation: f32,
    pub neuron_activation: f32,
    pub confidence: f32,
    pub final_score: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Payload-light ranked item produced by blended recall.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextRecallItem {
    pub source: ContextRecallSource,
    pub source_id: String,
    pub summary: String,
    pub score: ContextRecallScore,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_transcript_spans: Vec<TranscriptSpanRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_memory_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_session_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub neuron_ids: Vec<NeuronId>,
}

/// One inspectable frame from which routing, neuron activation, and intuition
/// should be projected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntelligenceTurnFrame {
    pub recall_bundle: ContextRecallBundle,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_neurons: Vec<HeptaNeuron>,
    pub budget: ContextBudget,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance_spans: Vec<TranscriptSpanRef>,
    #[serde(default)]
    pub omitted_by_budget: usize,
}

/// Lightweight per-source item counts for a blended recall bundle.
///
/// This gives automation and tests a compact machine-readable summary without
/// needing to walk the full transcript and memory payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallSourceCounts {
    pub recent_entry_count: usize,
    pub transcript_hit_count: usize,
    pub durable_memory_hit_count: usize,
    pub summary_hit_count: usize,
}

impl ContextRecallSourceCounts {
    pub fn query_hit_count(&self) -> usize {
        self.transcript_hit_count + self.durable_memory_hit_count + self.summary_hit_count
    }

    pub fn total_item_count(&self) -> usize {
        self.recent_entry_count + self.query_hit_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.query_hit_count() > 0
    }

    pub fn is_empty(&self) -> bool {
        self.total_item_count() == 0
    }
}

/// Compact machine-readable report for blended transcript + memory recall.
///
/// This keeps the original request, per-source counts, and truncation state
/// without embedding the full transcript and memory payloads from the bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextRecallReport {
    pub request: ContextRecallRequest,
    pub source_counts: ContextRecallSourceCounts,
    pub truncated: bool,
}

impl ContextRecallReport {
    pub fn from_bundle(bundle: &ContextRecallBundle) -> Self {
        Self {
            request: bundle.request.clone(),
            source_counts: bundle.source_counts(),
            truncated: bundle.truncated,
        }
    }

    pub fn query_hit_count(&self) -> usize {
        self.source_counts.query_hit_count()
    }

    pub fn total_item_count(&self) -> usize {
        self.source_counts.total_item_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.source_counts.has_query_matches()
    }

    pub fn is_empty(&self) -> bool {
        self.source_counts.is_empty()
    }
}

/// Payload-light availability counts for blended recall sources before limits
/// are applied.
///
/// This complements [`ContextRecallReport`], whose counts describe returned
/// items only. Automation can use the availability view to detect which recall
/// sources were clipped by recent-window or query limits without loading the
/// full bundle payload. The counts stay pre-limit even when recent-window,
/// transcript, or memory caps return fewer items, which lets callers compare
/// compact availability data against returned counts without reconstructing the
/// full inspection payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallAvailability {
    pub total_recent_entry_count: usize,
    pub total_transcript_match_count: usize,
    pub total_memory_match_count: usize,
}

impl ContextRecallAvailability {
    pub fn query_match_count(&self) -> usize {
        self.total_transcript_match_count + self.total_memory_match_count
    }

    pub fn total_item_count(&self) -> usize {
        self.total_recent_entry_count + self.query_match_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.query_match_count() > 0
    }

    pub fn is_empty(&self) -> bool {
        self.total_item_count() == 0
    }
}

/// Compact pre-limit recall counts that preserve the returned-source split.
///
/// Unlike [`ContextRecallAvailability`], this retains separate durable-memory
/// and session-summary match counts so automation can reason about which memory
/// lane contributed omitted hits without loading the full recall payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallSourceAvailability {
    pub recent_entry_count: usize,
    pub transcript_match_count: usize,
    pub durable_memory_match_count: usize,
    pub summary_memory_match_count: usize,
}

impl ContextRecallSourceAvailability {
    pub fn memory_match_count(&self) -> usize {
        self.durable_memory_match_count + self.summary_memory_match_count
    }

    pub fn query_match_count(&self) -> usize {
        self.transcript_match_count + self.memory_match_count()
    }

    pub fn total_item_count(&self) -> usize {
        self.recent_entry_count + self.query_match_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.query_match_count() > 0
    }

    pub fn is_empty(&self) -> bool {
        self.total_item_count() == 0
    }
}

/// Compact inspection view for blended recall availability and returned items.
///
/// Unlike [`ContextRecallBundle`], this keeps only the report plus pre-limit
/// availability counts, which makes it suitable for doctor output, audit
/// trails, and automation that needs to distinguish between complete and
/// clipped recall results, including how many items were omitted by each
/// recall limit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextRecallInspection {
    pub report: ContextRecallReport,
    pub availability: ContextRecallAvailability,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_transcript_spans: Vec<TranscriptSpanRef>,
}

impl ContextRecallInspection {
    pub fn from_bundle(
        bundle: &ContextRecallBundle,
        availability: ContextRecallAvailability,
    ) -> Self {
        Self {
            report: bundle.report(),
            availability,
            source_transcript_spans: bundle.source_transcript_spans(),
        }
    }

    pub fn returned_memory_hit_count(&self) -> usize {
        self.report.source_counts.durable_memory_hit_count
            + self.report.source_counts.summary_hit_count
    }

    pub fn returned_query_hit_count(&self) -> usize {
        self.report.query_hit_count()
    }

    pub fn returned_total_item_count(&self) -> usize {
        self.report.total_item_count()
    }

    pub fn omitted_recent_entry_count(&self) -> usize {
        self.availability
            .total_recent_entry_count
            .saturating_sub(self.report.source_counts.recent_entry_count)
    }

    pub fn omitted_transcript_hit_count(&self) -> usize {
        self.availability
            .total_transcript_match_count
            .saturating_sub(self.report.source_counts.transcript_hit_count)
    }

    pub fn omitted_memory_hit_count(&self) -> usize {
        self.availability
            .total_memory_match_count
            .saturating_sub(self.returned_memory_hit_count())
    }

    pub fn omitted_query_hit_count(&self) -> usize {
        self.omitted_transcript_hit_count() + self.omitted_memory_hit_count()
    }

    pub fn omitted_total_item_count(&self) -> usize {
        self.omitted_recent_entry_count() + self.omitted_query_hit_count()
    }

    /// Returns a compact omitted-item summary for each recall source.
    pub fn omission_counts(&self) -> ContextRecallOmissionCounts {
        ContextRecallOmissionCounts {
            recent_entry_count: self.omitted_recent_entry_count(),
            transcript_hit_count: self.omitted_transcript_hit_count(),
            memory_hit_count: self.omitted_memory_hit_count(),
            query_hit_count: self.omitted_query_hit_count(),
            total_item_count: self.omitted_total_item_count(),
        }
    }

    pub fn matched_query_hit_count(&self) -> usize {
        self.availability.query_match_count()
    }

    pub fn matched_total_item_count(&self) -> usize {
        self.availability.total_item_count()
    }

    pub fn recent_entries_truncated(&self) -> bool {
        self.report.source_counts.recent_entry_count < self.availability.total_recent_entry_count
    }

    pub fn transcript_hits_truncated(&self) -> bool {
        self.report.source_counts.transcript_hit_count
            < self.availability.total_transcript_match_count
    }

    pub fn memory_hits_truncated(&self) -> bool {
        self.returned_memory_hit_count() < self.availability.total_memory_match_count
    }

    pub fn has_query_matches(&self) -> bool {
        self.availability.has_query_matches()
    }

    pub fn has_omissions(&self) -> bool {
        self.omitted_total_item_count() > 0
    }

    pub fn is_complete(&self) -> bool {
        !self.recent_entries_truncated()
            && !self.transcript_hits_truncated()
            && !self.memory_hits_truncated()
    }

    pub fn is_empty(&self) -> bool {
        self.report.is_empty() && self.availability.is_empty()
    }

    /// Returns a compact summary of transcript provenance attached to this
    /// inspection view.
    pub fn transcript_provenance_summary(&self) -> ContextRecallTranscriptProvenanceSummary {
        ContextRecallTranscriptProvenanceSummary::from_span_refs(&self.source_transcript_spans)
    }

    /// Returns a compact returned-vs-available coverage summary for recall
    /// sources and totals.
    pub fn coverage(&self) -> ContextRecallCoverage {
        ContextRecallCoverage::from_inspection(self)
    }

    /// Returns a compact limit-pressure summary for recall sources and totals.
    pub fn limit_pressure(&self) -> ContextRecallLimitPressure {
        ContextRecallLimitPressure::from_inspection(self)
    }
}

/// Compact transcript-provenance summary for blended recall.
///
/// This lets automation and doctor-style checks reason about how much
/// transcript evidence a recall result carries, how many sessions that
/// evidence spans, and whether provenance reasons were preserved, without
/// embedding the individual `TranscriptSpanRef` payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallTranscriptProvenanceSummary {
    pub span_count: usize,
    pub session_count: usize,
    pub spans_with_reason_count: usize,
    pub distinct_reason_count: usize,
}

impl ContextRecallTranscriptProvenanceSummary {
    pub fn from_span_refs(spans: &[TranscriptSpanRef]) -> Self {
        let mut session_ids = BTreeSet::new();
        let mut reasons = BTreeSet::new();
        let mut spans_with_reason_count = 0;

        for span in spans {
            let session_id = span.session_id.0.trim();
            if !session_id.is_empty() {
                session_ids.insert(session_id.to_string());
            }

            let mut has_reason = false;
            for reason in span.reason.as_deref().into_iter().flat_map(|reason| {
                reason
                    .split(',')
                    .map(str::trim)
                    .filter(|part| !part.is_empty())
                    .collect::<Vec<_>>()
            }) {
                has_reason = true;
                reasons.insert(reason.to_string());
            }

            if has_reason {
                spans_with_reason_count += 1;
            }
        }

        Self {
            span_count: spans.len(),
            session_count: session_ids.len(),
            spans_with_reason_count,
            distinct_reason_count: reasons.len(),
        }
    }

    pub fn has_spans(&self) -> bool {
        self.span_count > 0
    }

    pub fn has_reasons(&self) -> bool {
        self.distinct_reason_count > 0
    }

    pub fn is_empty(&self) -> bool {
        self.span_count == 0
    }
}

/// Compact returned-vs-available counts for one recall source.
///
/// This lets automation observe omission pressure and completeness without
/// diffing counts by hand.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallCoverageCounts {
    pub returned_count: usize,
    pub available_count: usize,
}

impl ContextRecallCoverageCounts {
    pub fn omitted_count(&self) -> usize {
        self.available_count.saturating_sub(self.returned_count)
    }

    pub fn is_complete(&self) -> bool {
        self.returned_count == self.available_count
    }

    pub fn is_empty(&self) -> bool {
        self.available_count == 0
    }

    pub fn is_truncated(&self) -> bool {
        self.returned_count < self.available_count
    }
}

/// Compact omitted-item counts for blended recall sources and totals.
///
/// This gives automation a machine-readable summary of what recent-window or
/// query limits left behind without requiring callers to diff returned-vs-
/// available coverage counts by hand.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallOmissionCounts {
    pub recent_entry_count: usize,
    pub transcript_hit_count: usize,
    pub memory_hit_count: usize,
    pub query_hit_count: usize,
    pub total_item_count: usize,
}

impl ContextRecallOmissionCounts {
    pub fn has_omissions(&self) -> bool {
        self.total_item_count > 0
    }

    pub fn is_empty(&self) -> bool {
        self.total_item_count == 0
    }
}

/// Compact summary of whether recall limits clipped any source.
///
/// Unlike [`ContextRecallCoverage`], this payload focuses on omission pressure
/// and truncation flags instead of returned-vs-available counts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallLimitPressure {
    pub recent_entries_truncated: bool,
    pub transcript_hits_truncated: bool,
    pub memory_hits_truncated: bool,
    pub omission_counts: ContextRecallOmissionCounts,
}

impl ContextRecallLimitPressure {
    pub fn from_inspection(inspection: &ContextRecallInspection) -> Self {
        Self {
            recent_entries_truncated: inspection.recent_entries_truncated(),
            transcript_hits_truncated: inspection.transcript_hits_truncated(),
            memory_hits_truncated: inspection.memory_hits_truncated(),
            omission_counts: inspection.omission_counts(),
        }
    }

    pub fn from_coverage(coverage: &ContextRecallCoverage) -> Self {
        Self {
            recent_entries_truncated: coverage.recent_entries.is_truncated(),
            transcript_hits_truncated: coverage.transcript_hits.is_truncated(),
            memory_hits_truncated: coverage.memory_hits.is_truncated(),
            omission_counts: coverage.omission_counts(),
        }
    }

    pub fn query_hits_truncated(&self) -> bool {
        self.transcript_hits_truncated || self.memory_hits_truncated
    }

    pub fn has_omissions(&self) -> bool {
        self.omission_counts.has_omissions()
    }

    pub fn is_complete(&self) -> bool {
        !self.recent_entries_truncated
            && !self.transcript_hits_truncated
            && !self.memory_hits_truncated
    }

    pub fn is_empty(&self) -> bool {
        self.is_complete() && self.omission_counts.is_empty()
    }
}

/// Compact returned-vs-available coverage summary for blended recall.
///
/// Unlike [`ContextRecallInspection`], this payload omits the original request
/// and collapses source totals into machine-readable coverage counts that are
/// easy to ship through automation and audit trails.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextRecallCoverage {
    pub recent_entries: ContextRecallCoverageCounts,
    pub transcript_hits: ContextRecallCoverageCounts,
    pub memory_hits: ContextRecallCoverageCounts,
    pub query_hits: ContextRecallCoverageCounts,
    pub total_items: ContextRecallCoverageCounts,
}

impl ContextRecallCoverage {
    pub fn from_inspection(inspection: &ContextRecallInspection) -> Self {
        let recent_entries = ContextRecallCoverageCounts {
            returned_count: inspection.report.source_counts.recent_entry_count,
            available_count: inspection.availability.total_recent_entry_count,
        };
        let transcript_hits = ContextRecallCoverageCounts {
            returned_count: inspection.report.source_counts.transcript_hit_count,
            available_count: inspection.availability.total_transcript_match_count,
        };
        let memory_hits = ContextRecallCoverageCounts {
            returned_count: inspection.returned_memory_hit_count(),
            available_count: inspection.availability.total_memory_match_count,
        };
        let query_hits = ContextRecallCoverageCounts {
            returned_count: inspection.returned_query_hit_count(),
            available_count: inspection.matched_query_hit_count(),
        };
        let total_items = ContextRecallCoverageCounts {
            returned_count: inspection.returned_total_item_count(),
            available_count: inspection.matched_total_item_count(),
        };

        Self {
            recent_entries,
            transcript_hits,
            memory_hits,
            query_hits,
            total_items,
        }
    }

    pub fn omitted_total_item_count(&self) -> usize {
        self.total_items.omitted_count()
    }

    /// Returns a compact omitted-item summary for each recall source.
    pub fn omission_counts(&self) -> ContextRecallOmissionCounts {
        ContextRecallOmissionCounts {
            recent_entry_count: self.recent_entries.omitted_count(),
            transcript_hit_count: self.transcript_hits.omitted_count(),
            memory_hit_count: self.memory_hits.omitted_count(),
            query_hit_count: self.query_hits.omitted_count(),
            total_item_count: self.total_items.omitted_count(),
        }
    }

    pub fn has_omissions(&self) -> bool {
        self.omitted_total_item_count() > 0
    }

    pub fn is_complete(&self) -> bool {
        self.recent_entries.is_complete()
            && self.transcript_hits.is_complete()
            && self.memory_hits.is_complete()
            && self.query_hits.is_complete()
            && self.total_items.is_complete()
    }

    pub fn is_empty(&self) -> bool {
        self.total_items.is_empty()
    }

    /// Returns a compact limit-pressure summary for recall sources and totals.
    pub fn limit_pressure(&self) -> ContextRecallLimitPressure {
        ContextRecallLimitPressure::from_coverage(self)
    }
}

impl ContextRecallBundle {
    /// Returns a bounded set of lightweight transcript provenance refs for the
    /// evidence used by this blended recall bundle.
    pub fn source_transcript_spans(&self) -> Vec<TranscriptSpanRef> {
        let mut refs = Vec::new();

        if let Some(range) = transcript_range_for_entries(&self.recent_entries) {
            upsert_context_recall_transcript_span_ref(
                &mut refs,
                TranscriptSpanRef {
                    session_id: self.recent_entries[0].session_id.clone(),
                    range,
                    reason: Some("recent_window".to_string()),
                },
            );
        }

        for hit in &self.transcript_hits {
            upsert_context_recall_transcript_span_ref(
                &mut refs,
                TranscriptSpanRef {
                    session_id: hit.session_id.clone(),
                    range: hit.range.clone(),
                    reason: Some("query_match".to_string()),
                },
            );
        }

        for topic_session in self
            .active_topic_sessions
            .iter()
            .filter(|topic_session| matches!(topic_session.status, TopicSessionStatus::Active))
        {
            for span in &topic_session.linked_transcript_spans {
                upsert_context_recall_transcript_span_ref(
                    &mut refs,
                    TranscriptSpanRef {
                        session_id: span.session_id.clone(),
                        range: span.range.clone(),
                        reason: merge_context_recall_transcript_span_reasons(
                            span.reason.as_deref(),
                            Some("active_topic_session"),
                        ),
                    },
                );
            }
        }

        refs.sort_by(|left, right| {
            right
                .range
                .end_sequence
                .cmp(&left.range.end_sequence)
                .then_with(|| right.range.start_sequence.cmp(&left.range.start_sequence))
                .then_with(|| left.session_id.0.cmp(&right.session_id.0))
        });
        refs.truncate(DEFAULT_CONTEXT_RECALL_TRANSCRIPT_PROVENANCE_LIMIT);
        refs
    }

    /// Returns a compact per-source count summary for this recall bundle.
    pub fn source_counts(&self) -> ContextRecallSourceCounts {
        ContextRecallSourceCounts {
            recent_entry_count: self.recent_entries.len(),
            transcript_hit_count: self.transcript_hits.len(),
            durable_memory_hit_count: self.durable_memory_hits.len(),
            summary_hit_count: self.summary_hits.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.source_counts().is_empty()
    }

    pub fn query_hit_count(&self) -> usize {
        self.source_counts().query_hit_count()
    }

    pub fn total_item_count(&self) -> usize {
        self.source_counts().total_item_count()
    }

    pub fn has_query_matches(&self) -> bool {
        self.source_counts().has_query_matches()
    }

    pub fn active_topic_session_count(&self) -> usize {
        self.active_topic_sessions.len()
    }

    pub fn active_neuron_count(&self) -> usize {
        self.active_neurons.len()
    }

    pub fn ensure_budget(&mut self) {
        if self.budget.max_items == 0 {
            self.budget = ContextBudget::from_request(&self.request);
        }
    }

    /// Returns a compact summary of transcript provenance attached to this
    /// recall bundle.
    pub fn transcript_provenance_summary(&self) -> ContextRecallTranscriptProvenanceSummary {
        ContextRecallTranscriptProvenanceSummary::from_span_refs(&self.source_transcript_spans())
    }

    /// Returns a payload-light report that preserves request, counts, and
    /// truncation state for diagnostics and automation.
    pub fn report(&self) -> ContextRecallReport {
        ContextRecallReport::from_bundle(self)
    }

    /// Returns a payload-light inspection view that pairs returned counts with
    /// pre-limit availability counts.
    pub fn inspection(&self, availability: ContextRecallAvailability) -> ContextRecallInspection {
        ContextRecallInspection::from_bundle(self, availability)
    }
}

fn transcript_range_for_entries(entries: &[TranscriptEntry]) -> Option<TranscriptRange> {
    let start = entries.first()?.sequence;
    let end = entries.last()?.sequence;
    Some(TranscriptRange {
        start_sequence: start,
        end_sequence: end,
    })
}

fn upsert_context_recall_transcript_span_ref(
    refs: &mut Vec<TranscriptSpanRef>,
    incoming: TranscriptSpanRef,
) {
    if let Some(existing) = refs.iter_mut().find(|existing| {
        existing.session_id == incoming.session_id && existing.range == incoming.range
    }) {
        existing.reason = merge_context_recall_transcript_span_reasons(
            existing.reason.as_deref(),
            incoming.reason.as_deref(),
        );
        return;
    }

    refs.push(incoming);
}

fn merge_context_recall_transcript_span_reasons(
    existing: Option<&str>,
    incoming: Option<&str>,
) -> Option<String> {
    let mut merged = Vec::new();
    let mut seen = BTreeSet::new();

    for reason in [existing, incoming].into_iter().flatten() {
        for part in reason
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
        {
            if seen.insert(part.to_string()) {
                merged.push(part.to_string());
            }
        }
    }

    (!merged.is_empty()).then(|| merged.join(", "))
}

/// Aggregate counts that describe a portable transcript snapshot.
///
/// This stays intentionally storage-agnostic so diagnostics, export/import
/// tooling, and contract tests can reason about transcript state without
/// binding to a concrete backend implementation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptSnapshotStats {
    pub total_entry_count: usize,
    pub session_count: usize,
    pub message_count: usize,
    pub tool_call_count: usize,
    pub tool_result_count: usize,
    pub approval_count: usize,
    pub summary_count: usize,
    pub event_count: usize,
}

impl TranscriptSnapshotStats {
    pub fn from_entries(entries: &[TranscriptEntry]) -> Self {
        let mut stats = Self::default();
        let mut session_ids = BTreeSet::new();

        for entry in entries {
            stats.total_entry_count += 1;

            let session_id = entry.session_id.0.trim();
            if !session_id.is_empty() {
                session_ids.insert(session_id.to_string());
            }

            match entry.kind {
                TranscriptEntryKind::Message => stats.message_count += 1,
                TranscriptEntryKind::ToolCall => stats.tool_call_count += 1,
                TranscriptEntryKind::ToolResult => stats.tool_result_count += 1,
                TranscriptEntryKind::Approval => stats.approval_count += 1,
                TranscriptEntryKind::Summary => stats.summary_count += 1,
                TranscriptEntryKind::Event => stats.event_count += 1,
            }
        }

        stats.session_count = session_ids.len();
        stats
    }

    pub fn is_empty(&self) -> bool {
        self.total_entry_count == 0
    }
}

/// Compact transcript metadata for snapshot manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotTranscriptDescriptor {
    pub entry_id: String,
    pub session_id: SessionId,
    pub sequence: u64,
    pub kind: TranscriptEntryKind,
    pub content_bytes: usize,
}

/// Portable manifest that summarizes a transcript snapshot without embedding
/// the full entry contents.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptSnapshotManifest {
    pub stats: TranscriptSnapshotStats,
    #[serde(default)]
    pub entries: Vec<SnapshotTranscriptDescriptor>,
}

impl TranscriptSnapshotManifest {
    pub fn from_entries(entries: &[TranscriptEntry]) -> Self {
        let mut entry_descriptors = entries
            .iter()
            .map(|entry| SnapshotTranscriptDescriptor {
                entry_id: entry.entry_id.clone(),
                session_id: entry.session_id.clone(),
                sequence: entry.sequence,
                kind: entry.kind,
                content_bytes: entry.content.len(),
            })
            .collect::<Vec<_>>();
        entry_descriptors.sort_by(|left, right| {
            left.session_id
                .0
                .cmp(&right.session_id.0)
                .then(left.sequence.cmp(&right.sequence))
                .then(left.entry_id.cmp(&right.entry_id))
        });

        Self {
            stats: TranscriptSnapshotStats::from_entries(entries),
            entries: entry_descriptors,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stats.is_empty()
    }
}

/// Compact per-session transcript inventory derived from a portable transcript
/// snapshot.
///
/// This gives doctor, export/import, and CLI tooling a payload-light way to
/// inspect transcript occupancy by session without needing to load or diff the
/// full entry list.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptSessionInventory {
    pub total_entry_count: usize,
    pub blank_session_id_entry_count: usize,
    #[serde(default)]
    pub sessions: Vec<TranscriptSessionDescriptor>,
}

/// Per-session transcript inventory row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptSessionDescriptor {
    pub session_id: SessionId,
    pub entry_count: usize,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub message_count: usize,
    pub tool_call_count: usize,
    pub tool_result_count: usize,
    pub approval_count: usize,
    pub summary_count: usize,
    pub event_count: usize,
}

impl TranscriptSessionInventory {
    pub fn from_entries(entries: &[TranscriptEntry]) -> Self {
        #[derive(Default)]
        struct SessionAccumulator {
            entry_count: usize,
            first_sequence: Option<u64>,
            last_sequence: Option<u64>,
            message_count: usize,
            tool_call_count: usize,
            tool_result_count: usize,
            approval_count: usize,
            summary_count: usize,
            event_count: usize,
        }

        let mut blank_session_id_entry_count = 0;
        let mut by_session = BTreeMap::<String, SessionAccumulator>::new();

        for entry in entries {
            let session_id = entry.session_id.0.trim();
            if session_id.is_empty() {
                blank_session_id_entry_count += 1;
                continue;
            }

            let accumulator = by_session.entry(session_id.to_string()).or_default();
            accumulator.entry_count += 1;
            accumulator.first_sequence = Some(
                accumulator
                    .first_sequence
                    .map_or(entry.sequence, |current| current.min(entry.sequence)),
            );
            accumulator.last_sequence = Some(
                accumulator
                    .last_sequence
                    .map_or(entry.sequence, |current| current.max(entry.sequence)),
            );

            match entry.kind {
                TranscriptEntryKind::Message => accumulator.message_count += 1,
                TranscriptEntryKind::ToolCall => accumulator.tool_call_count += 1,
                TranscriptEntryKind::ToolResult => accumulator.tool_result_count += 1,
                TranscriptEntryKind::Approval => accumulator.approval_count += 1,
                TranscriptEntryKind::Summary => accumulator.summary_count += 1,
                TranscriptEntryKind::Event => accumulator.event_count += 1,
            }
        }

        let sessions = by_session
            .into_iter()
            .map(|(session_id, accumulator)| TranscriptSessionDescriptor {
                session_id: SessionId(session_id),
                entry_count: accumulator.entry_count,
                first_sequence: accumulator.first_sequence.unwrap_or_default(),
                last_sequence: accumulator.last_sequence.unwrap_or_default(),
                message_count: accumulator.message_count,
                tool_call_count: accumulator.tool_call_count,
                tool_result_count: accumulator.tool_result_count,
                approval_count: accumulator.approval_count,
                summary_count: accumulator.summary_count,
                event_count: accumulator.event_count,
            })
            .collect();

        Self {
            total_entry_count: entries.len(),
            blank_session_id_entry_count,
            sessions,
        }
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    pub fn inventoried_entry_count(&self) -> usize {
        self.sessions
            .iter()
            .map(|session| session.entry_count)
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.total_entry_count == 0
    }
}

/// Duplicate transcript sequence occupancy inside a single session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptSequenceCollision {
    pub session_id: SessionId,
    pub sequence: u64,
    #[serde(default)]
    pub entry_ids: Vec<String>,
}

/// Integrity-focused summary of a portable transcript snapshot.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptSnapshotIntegrityReport {
    #[serde(default)]
    pub duplicate_entry_ids: Vec<String>,
    #[serde(default)]
    pub duplicate_sequence_collisions: Vec<TranscriptSequenceCollision>,
    pub blank_entry_id_count: usize,
    pub blank_session_id_count: usize,
    pub blank_content_count: usize,
}

impl TranscriptSnapshotIntegrityReport {
    pub fn from_entries(entries: &[TranscriptEntry]) -> Self {
        let duplicate_entry_ids = duplicate_non_blank_values(
            entries
                .iter()
                .map(|entry| entry.entry_id.trim().to_string()),
        );

        let mut sequence_collisions = BTreeMap::<(String, u64), Vec<String>>::new();
        for entry in entries {
            let session_id = entry.session_id.0.trim();
            if session_id.is_empty() {
                continue;
            }

            sequence_collisions
                .entry((session_id.to_string(), entry.sequence))
                .or_default()
                .push(entry.entry_id.trim().to_string());
        }

        let duplicate_sequence_collisions = sequence_collisions
            .into_iter()
            .filter_map(|((session_id, sequence), entry_ids)| {
                (entry_ids.len() > 1).then_some(TranscriptSequenceCollision {
                    session_id: SessionId(session_id),
                    sequence,
                    entry_ids,
                })
            })
            .collect();

        Self {
            duplicate_entry_ids,
            duplicate_sequence_collisions,
            blank_entry_id_count: entries
                .iter()
                .filter(|entry| entry.entry_id.trim().is_empty())
                .count(),
            blank_session_id_count: entries
                .iter()
                .filter(|entry| entry.session_id.0.trim().is_empty())
                .count(),
            blank_content_count: entries
                .iter()
                .filter(|entry| entry.content.trim().is_empty())
                .count(),
        }
    }

    pub fn issue_count(&self) -> usize {
        self.duplicate_entry_ids.len()
            + self.duplicate_sequence_collisions.len()
            + self.blank_entry_id_count
            + self.blank_session_id_count
            + self.blank_content_count
    }

    pub fn is_clean(&self) -> bool {
        self.issue_count() == 0
    }
}

/// Aggregate counts that describe a portable session+memory snapshot.
///
/// This stays intentionally storage-agnostic so doctor reports, export/import
/// flows, and lightweight tooling can reason about memory state without binding
/// to a concrete backend implementation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MemorySnapshotStats {
    pub session_count: usize,
    pub active_session_count: usize,
    pub archived_session_count: usize,
    pub total_memory_count: usize,
    pub session_memory_count: usize,
    pub long_term_memory_count: usize,
}

impl MemorySnapshotStats {
    pub fn from_records(sessions: &[SessionRecord], memories: &[MemoryRecord]) -> Self {
        let archived_session_count = sessions
            .iter()
            .filter(|record| record.archived_at_unix_ms.is_some())
            .count();
        let session_memory_count = memories
            .iter()
            .filter(|record| record.scope == MemoryScope::Session)
            .count();
        let long_term_memory_count = memories
            .iter()
            .filter(|record| record.scope == MemoryScope::LongTerm)
            .count();

        Self {
            session_count: sessions.len(),
            active_session_count: sessions.len().saturating_sub(archived_session_count),
            archived_session_count,
            total_memory_count: memories.len(),
            session_memory_count,
            long_term_memory_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.session_count == 0 && self.total_memory_count == 0
    }
}

/// Compact per-agent session inventory derived from portable session records.
///
/// This gives doctor, audit, and export/import tooling a storage-agnostic way
/// to inspect session occupancy by agent without loading the full session
/// payload set into a custom report shape.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionAgentInventory {
    pub total_session_count: usize,
    pub blank_agent_id_session_count: usize,
    #[serde(default)]
    pub agents: Vec<SessionAgentDescriptor>,
}

/// Per-agent session inventory row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionAgentDescriptor {
    pub agent_id: AgentId,
    pub session_count: usize,
    pub active_session_count: usize,
    pub archived_session_count: usize,
    pub latest_activity_unix_ms: u64,
}

impl SessionAgentInventory {
    pub fn from_records(sessions: &[SessionRecord]) -> Self {
        #[derive(Default)]
        struct AgentAccumulator {
            session_count: usize,
            active_session_count: usize,
            archived_session_count: usize,
            latest_activity_unix_ms: u64,
        }

        let mut blank_agent_id_session_count = 0;
        let mut by_agent = BTreeMap::<String, AgentAccumulator>::new();

        for record in sessions {
            let agent_id = record.agent_id.0.trim();
            if agent_id.is_empty() {
                blank_agent_id_session_count += 1;
                continue;
            }

            let accumulator = by_agent.entry(agent_id.to_string()).or_default();
            accumulator.session_count += 1;
            if record.archived_at_unix_ms.is_some() {
                accumulator.archived_session_count += 1;
            } else {
                accumulator.active_session_count += 1;
            }
            accumulator.latest_activity_unix_ms = accumulator
                .latest_activity_unix_ms
                .max(record.last_active_unix_ms);
        }

        let agents = by_agent
            .into_iter()
            .map(|(agent_id, accumulator)| SessionAgentDescriptor {
                agent_id: AgentId(agent_id),
                session_count: accumulator.session_count,
                active_session_count: accumulator.active_session_count,
                archived_session_count: accumulator.archived_session_count,
                latest_activity_unix_ms: accumulator.latest_activity_unix_ms,
            })
            .collect();

        Self {
            total_session_count: sessions.len(),
            blank_agent_id_session_count,
            agents,
        }
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn inventoried_session_count(&self) -> usize {
        self.agents.iter().map(|agent| agent.session_count).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.total_session_count == 0
    }
}

/// Compact session metadata for snapshot manifests.
///
/// Manifests are intended for diagnostics, audit trails, and export/import
/// planning, where callers need to inspect the shape of a snapshot without
/// loading the full session history or memory payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotSessionDescriptor {
    pub session_id: SessionId,
    pub title: String,
    pub archived: bool,
}

/// Compact memory metadata for snapshot manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotMemoryDescriptor {
    pub id: String,
    pub scope: MemoryScope,
    pub content_bytes: usize,
}

/// Portable manifest that summarizes a session+memory snapshot without
/// embedding the full memory contents.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MemorySnapshotManifest {
    pub stats: MemorySnapshotStats,
    #[serde(default)]
    pub sessions: Vec<SnapshotSessionDescriptor>,
    #[serde(default)]
    pub memories: Vec<SnapshotMemoryDescriptor>,
}

impl MemorySnapshotManifest {
    pub fn from_records(sessions: &[SessionRecord], memories: &[MemoryRecord]) -> Self {
        let mut session_descriptors = sessions
            .iter()
            .map(|record| SnapshotSessionDescriptor {
                session_id: record.session_id.clone(),
                title: record.title.clone(),
                archived: record.archived_at_unix_ms.is_some(),
            })
            .collect::<Vec<_>>();
        session_descriptors.sort_by(|left, right| left.session_id.0.cmp(&right.session_id.0));

        let mut memory_descriptors = memories
            .iter()
            .map(|record| SnapshotMemoryDescriptor {
                id: record.id.clone(),
                scope: record.scope,
                content_bytes: record.content.len(),
            })
            .collect::<Vec<_>>();
        memory_descriptors.sort_by(|left, right| left.id.cmp(&right.id));

        Self {
            stats: MemorySnapshotStats::from_records(sessions, memories),
            sessions: session_descriptors,
            memories: memory_descriptors,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stats.is_empty()
    }
}

/// Integrity-focused summary of a portable session+memory snapshot.
///
/// This is intentionally additive to the manifest/stats layer: callers can use
/// it for doctor checks, export/import preflight validation, or audit tooling
/// without binding to a concrete store implementation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MemorySnapshotIntegrityReport {
    #[serde(default)]
    pub duplicate_session_ids: Vec<SessionId>,
    #[serde(default)]
    pub duplicate_memory_ids: Vec<String>,
    pub blank_session_id_count: usize,
    pub blank_memory_id_count: usize,
    pub blank_session_title_count: usize,
    pub blank_memory_content_count: usize,
}

impl MemorySnapshotIntegrityReport {
    pub fn from_records(sessions: &[SessionRecord], memories: &[MemoryRecord]) -> Self {
        let duplicate_session_ids = duplicate_non_blank_values(
            sessions
                .iter()
                .map(|record| record.session_id.0.trim().to_string()),
        )
        .into_iter()
        .map(SessionId)
        .collect();
        let duplicate_memory_ids =
            duplicate_non_blank_values(memories.iter().map(|record| record.id.trim().to_string()));

        Self {
            duplicate_session_ids,
            duplicate_memory_ids,
            blank_session_id_count: sessions
                .iter()
                .filter(|record| record.session_id.0.trim().is_empty())
                .count(),
            blank_memory_id_count: memories
                .iter()
                .filter(|record| record.id.trim().is_empty())
                .count(),
            blank_session_title_count: sessions
                .iter()
                .filter(|record| record.title.trim().is_empty())
                .count(),
            blank_memory_content_count: memories
                .iter()
                .filter(|record| record.content.trim().is_empty())
                .count(),
        }
    }

    pub fn issue_count(&self) -> usize {
        self.duplicate_session_ids.len()
            + self.duplicate_memory_ids.len()
            + self.blank_session_id_count
            + self.blank_memory_id_count
            + self.blank_session_title_count
            + self.blank_memory_content_count
    }

    pub fn is_clean(&self) -> bool {
        self.issue_count() == 0
    }
}

/// Combined audit view over portable session, memory, and transcript state.
///
/// This gives callers a single machine-readable contract for storage preflight,
/// export/import validation, and lightweight doctor checks without forcing them
/// to manually stitch together stats and integrity reports across layers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotAuditReport {
    pub memory_stats: MemorySnapshotStats,
    pub memory_integrity: MemorySnapshotIntegrityReport,
    pub transcript_stats: TranscriptSnapshotStats,
    pub transcript_integrity: TranscriptSnapshotIntegrityReport,
}

impl SnapshotAuditReport {
    pub fn from_records_and_entries(
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> Self {
        Self {
            memory_stats: MemorySnapshotStats::from_records(sessions, memories),
            memory_integrity: MemorySnapshotIntegrityReport::from_records(sessions, memories),
            transcript_stats: TranscriptSnapshotStats::from_entries(transcripts),
            transcript_integrity: TranscriptSnapshotIntegrityReport::from_entries(transcripts),
        }
    }

    pub fn memory_issue_count(&self) -> usize {
        self.memory_integrity.issue_count()
    }

    pub fn transcript_issue_count(&self) -> usize {
        self.transcript_integrity.issue_count()
    }

    pub fn issue_count(&self) -> usize {
        self.memory_issue_count() + self.transcript_issue_count()
    }

    pub fn issue_domain_count(&self) -> usize {
        usize::from(self.touches_memory()) + usize::from(self.touches_transcripts())
    }

    /// Returns a compact machine-readable issue-count summary for automation
    /// that does not need the full stats and integrity payloads.
    pub fn issue_summary(&self) -> SnapshotIssueSummary {
        SnapshotIssueSummary::from_audit_report(self)
    }

    pub fn touches_memory(&self) -> bool {
        self.memory_issue_count() > 0
    }

    pub fn touches_transcripts(&self) -> bool {
        self.transcript_issue_count() > 0
    }

    pub fn is_clean(&self) -> bool {
        self.issue_count() == 0
    }

    pub fn is_empty(&self) -> bool {
        self.memory_stats.is_empty() && self.transcript_stats.is_empty()
    }
}

/// Compact issue-count summary derived from snapshot audit or inspection
/// payloads.
///
/// This lets automation carry the cross-domain integrity posture of a
/// snapshot without embedding the full stats, manifests, or integrity
/// reports.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotIssueSummary {
    pub memory_issue_count: usize,
    pub transcript_issue_count: usize,
    pub total_issue_count: usize,
    pub issue_domain_count: usize,
}

impl SnapshotIssueSummary {
    pub fn from_audit_report(report: &SnapshotAuditReport) -> Self {
        Self {
            memory_issue_count: report.memory_issue_count(),
            transcript_issue_count: report.transcript_issue_count(),
            total_issue_count: report.issue_count(),
            issue_domain_count: report.issue_domain_count(),
        }
    }

    pub fn from_inspection(inspection: &SnapshotInspectionBundle) -> Self {
        Self {
            memory_issue_count: inspection.memory_issue_count(),
            transcript_issue_count: inspection.transcript_issue_count(),
            total_issue_count: inspection.issue_count(),
            issue_domain_count: inspection.issue_domain_count(),
        }
    }

    pub fn touches_memory(&self) -> bool {
        self.memory_issue_count > 0
    }

    pub fn touches_transcripts(&self) -> bool {
        self.transcript_issue_count > 0
    }

    pub fn has_issues(&self) -> bool {
        self.total_issue_count > 0
    }

    pub fn is_clean(&self) -> bool {
        !self.has_issues()
    }
}

/// Compact inspection bundle for portable session, memory, and transcript
/// snapshots.
///
/// Unlike [`SnapshotAuditReport`], this envelope keeps the manifest views for
/// both storage domains alongside their integrity reports so tooling can show a
/// stable inventory without loading the full payload-bearing snapshot.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotInspectionBundle {
    pub memory_manifest: MemorySnapshotManifest,
    pub memory_integrity: MemorySnapshotIntegrityReport,
    pub transcript_manifest: TranscriptSnapshotManifest,
    pub transcript_integrity: TranscriptSnapshotIntegrityReport,
}

impl SnapshotInspectionBundle {
    pub fn from_records_and_entries(
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> Self {
        Self {
            memory_manifest: MemorySnapshotManifest::from_records(sessions, memories),
            memory_integrity: MemorySnapshotIntegrityReport::from_records(sessions, memories),
            transcript_manifest: TranscriptSnapshotManifest::from_entries(transcripts),
            transcript_integrity: TranscriptSnapshotIntegrityReport::from_entries(transcripts),
        }
    }

    /// Reconstructs the lighter-weight audit view from an inspection bundle.
    ///
    /// This lets export/import tooling keep a single manifest-oriented payload
    /// on disk while still deriving the aggregate health summary used by doctor
    /// checks and automation.
    pub fn audit_report(&self) -> SnapshotAuditReport {
        SnapshotAuditReport {
            memory_stats: self.memory_manifest.stats.clone(),
            memory_integrity: self.memory_integrity.clone(),
            transcript_stats: self.transcript_manifest.stats.clone(),
            transcript_integrity: self.transcript_integrity.clone(),
        }
    }

    /// Returns `true` when this inspection bundle still reflects the supplied
    /// portable snapshot payloads.
    pub fn matches_records_and_entries(
        &self,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> bool {
        self == &Self::from_records_and_entries(sessions, memories, transcripts)
    }

    /// Returns a section-level drift report describing which inspection views,
    /// if any, no longer match the supplied portable snapshot payloads.
    pub fn drift_report(
        &self,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> SnapshotInspectionDriftReport {
        SnapshotInspectionDriftReport::from_bundle_and_records(
            self,
            sessions,
            memories,
            transcripts,
        )
    }

    /// Returns a compact domain-level drift summary for this inspection
    /// bundle relative to the supplied portable snapshot payloads.
    pub fn drift_impact_against_records(
        &self,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> SnapshotInspectionDriftImpact {
        self.drift_report(sessions, memories, transcripts).impact()
    }

    pub fn memory_issue_count(&self) -> usize {
        self.memory_integrity.issue_count()
    }

    pub fn transcript_issue_count(&self) -> usize {
        self.transcript_integrity.issue_count()
    }

    pub fn issue_count(&self) -> usize {
        self.memory_issue_count() + self.transcript_issue_count()
    }

    pub fn issue_domain_count(&self) -> usize {
        usize::from(self.touches_memory()) + usize::from(self.touches_transcripts())
    }

    /// Returns a compact machine-readable issue-count summary for automation
    /// that does not need the full manifest and integrity payloads.
    pub fn issue_summary(&self) -> SnapshotIssueSummary {
        SnapshotIssueSummary::from_inspection(self)
    }

    /// Returns a compact readiness summary that combines issue counts with
    /// inspection-drift impact relative to the supplied snapshot payloads.
    pub fn health_against_records(
        &self,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> SnapshotInspectionHealth {
        SnapshotInspectionHealth::from_bundle_and_records(self, sessions, memories, transcripts)
    }

    pub fn touches_memory(&self) -> bool {
        self.memory_issue_count() > 0
    }

    pub fn touches_transcripts(&self) -> bool {
        self.transcript_issue_count() > 0
    }

    pub fn is_clean(&self) -> bool {
        self.issue_count() == 0
    }

    pub fn is_empty(&self) -> bool {
        self.memory_manifest.is_empty() && self.transcript_manifest.is_empty()
    }
}

/// Named inspection-bundle section used by snapshot drift reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotInspectionSection {
    MemoryManifest,
    MemoryIntegrity,
    TranscriptManifest,
    TranscriptIntegrity,
}

/// Section-level drift summary for a persisted inspection bundle.
///
/// This lets automation and import/export tooling distinguish between a fully
/// aligned envelope and one where only part of the derived inspection view has
/// drifted away from the full snapshot payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotInspectionDriftReport {
    #[serde(default)]
    pub mismatched_sections: Vec<SnapshotInspectionSection>,
}

impl SnapshotInspectionDriftReport {
    pub fn from_bundle_and_records(
        inspection: &SnapshotInspectionBundle,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> Self {
        let expected =
            SnapshotInspectionBundle::from_records_and_entries(sessions, memories, transcripts);
        let mut mismatched_sections = Vec::new();

        if inspection.memory_manifest != expected.memory_manifest {
            mismatched_sections.push(SnapshotInspectionSection::MemoryManifest);
        }
        if inspection.memory_integrity != expected.memory_integrity {
            mismatched_sections.push(SnapshotInspectionSection::MemoryIntegrity);
        }
        if inspection.transcript_manifest != expected.transcript_manifest {
            mismatched_sections.push(SnapshotInspectionSection::TranscriptManifest);
        }
        if inspection.transcript_integrity != expected.transcript_integrity {
            mismatched_sections.push(SnapshotInspectionSection::TranscriptIntegrity);
        }

        Self {
            mismatched_sections,
        }
    }

    pub fn mismatch_count(&self) -> usize {
        self.mismatched_sections.len()
    }

    pub fn mismatches(&self, section: SnapshotInspectionSection) -> bool {
        self.mismatched_sections.contains(&section)
    }

    pub fn memory_mismatch_count(&self) -> usize {
        self.mismatched_sections
            .iter()
            .filter(|section| {
                matches!(
                    section,
                    SnapshotInspectionSection::MemoryManifest
                        | SnapshotInspectionSection::MemoryIntegrity
                )
            })
            .count()
    }

    pub fn transcript_mismatch_count(&self) -> usize {
        self.mismatched_sections
            .iter()
            .filter(|section| {
                matches!(
                    section,
                    SnapshotInspectionSection::TranscriptManifest
                        | SnapshotInspectionSection::TranscriptIntegrity
                )
            })
            .count()
    }

    pub fn changed_domain_count(&self) -> usize {
        usize::from(self.touches_memory()) + usize::from(self.touches_transcripts())
    }

    pub fn touches_memory(&self) -> bool {
        self.memory_mismatch_count() > 0
    }

    pub fn touches_transcripts(&self) -> bool {
        self.transcript_mismatch_count() > 0
    }

    /// Returns a compact domain-level drift summary for automation that does
    /// not need per-section names.
    pub fn impact(&self) -> SnapshotInspectionDriftImpact {
        SnapshotInspectionDriftImpact::from_report(self)
    }

    pub fn is_aligned(&self) -> bool {
        self.mismatched_sections.is_empty()
    }
}

/// Compact domain-level summary derived from a section-level inspection drift
/// report.
///
/// This lets automation and doctor-style checks answer whether inspection
/// drift touches memory-derived views, transcript-derived views, or both,
/// without carrying the individual mismatched section names.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotInspectionDriftImpact {
    pub mismatch_count: usize,
    pub memory_mismatch_count: usize,
    pub transcript_mismatch_count: usize,
}

impl SnapshotInspectionDriftImpact {
    pub fn from_report(report: &SnapshotInspectionDriftReport) -> Self {
        Self {
            mismatch_count: report.mismatch_count(),
            memory_mismatch_count: report.memory_mismatch_count(),
            transcript_mismatch_count: report.transcript_mismatch_count(),
        }
    }

    pub fn changed_domain_count(&self) -> usize {
        usize::from(self.touches_memory()) + usize::from(self.touches_transcripts())
    }

    pub fn touches_memory(&self) -> bool {
        self.memory_mismatch_count > 0
    }

    pub fn touches_transcripts(&self) -> bool {
        self.transcript_mismatch_count > 0
    }

    pub fn is_aligned(&self) -> bool {
        self.mismatch_count == 0
    }
}

/// Compact snapshot-inspection readiness summary.
///
/// This combines the issue posture of an inspection bundle with its drift
/// impact relative to a payload-bearing snapshot so automation can answer
/// "is this inspection clean and aligned?" without carrying manifests,
/// integrity reports, or section-level mismatch names.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotInspectionHealth {
    pub issue_summary: SnapshotIssueSummary,
    pub drift_impact: SnapshotInspectionDriftImpact,
}

impl SnapshotInspectionHealth {
    pub fn from_bundle_and_records(
        inspection: &SnapshotInspectionBundle,
        sessions: &[SessionRecord],
        memories: &[MemoryRecord],
        transcripts: &[TranscriptEntry],
    ) -> Self {
        Self {
            issue_summary: inspection.issue_summary(),
            drift_impact: inspection
                .drift_report(sessions, memories, transcripts)
                .impact(),
        }
    }

    pub fn issue_count(&self) -> usize {
        self.issue_summary.total_issue_count
    }

    pub fn mismatch_count(&self) -> usize {
        self.drift_impact.mismatch_count
    }

    pub fn touches_memory(&self) -> bool {
        self.issue_summary.touches_memory() || self.drift_impact.touches_memory()
    }

    pub fn touches_transcripts(&self) -> bool {
        self.issue_summary.touches_transcripts() || self.drift_impact.touches_transcripts()
    }

    pub fn changed_domain_count(&self) -> usize {
        usize::from(self.touches_memory()) + usize::from(self.touches_transcripts())
    }

    pub fn has_issues(&self) -> bool {
        self.issue_summary.has_issues()
    }

    pub fn has_drift(&self) -> bool {
        !self.drift_impact.is_aligned()
    }

    pub fn inspection_aligned(&self) -> bool {
        self.drift_impact.is_aligned()
    }

    pub fn is_clean(&self) -> bool {
        self.issue_summary.is_clean()
    }

    pub fn is_ready(&self) -> bool {
        self.is_clean() && self.inspection_aligned()
    }
}

/// Machine-readable restore delta counts used by preview automation and CLI
/// summaries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct RestoreDeltaCounts {
    pub added_count: usize,
    pub removed_count: usize,
    pub updated_count: usize,
    pub unchanged_count: usize,
}

impl RestoreDeltaCounts {
    pub fn change_count(&self) -> usize {
        self.added_count + self.removed_count + self.updated_count
    }

    pub fn has_additions(&self) -> bool {
        self.added_count > 0
    }

    pub fn has_removals(&self) -> bool {
        self.removed_count > 0
    }

    pub fn has_updates(&self) -> bool {
        self.updated_count > 0
    }

    pub fn has_changes(&self) -> bool {
        self.change_count() > 0
    }

    /// Returns `true` when this delta modifies or removes existing records.
    pub fn touches_existing_records(&self) -> bool {
        self.has_removals() || self.has_updates()
    }

    /// Returns `true` when this delta adds new records without updating or
    /// removing any existing records.
    pub fn is_additive_only(&self) -> bool {
        self.has_additions() && !self.touches_existing_records()
    }

    pub fn is_empty(&self) -> bool {
        self.change_count() == 0
    }
}

/// Named snapshot restore domain used by compact impact summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotRestoreDomain {
    Sessions,
    Memories,
    Transcripts,
}

/// Compact per-domain restore impact summary.
///
/// Unlike the identifier-bearing restore deltas on [`SnapshotRestorePreview`],
/// this struct keeps only aggregate counts for one restore domain so automation
/// can present stable summaries without loading the full changed-id lists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRestoreDomainImpact {
    pub domain: SnapshotRestoreDomain,
    pub counts: RestoreDeltaCounts,
}

impl SnapshotRestoreDomainImpact {
    pub fn change_count(&self) -> usize {
        self.counts.change_count()
    }

    pub fn has_changes(&self) -> bool {
        self.counts.has_changes()
    }

    pub fn has_additions(&self) -> bool {
        self.counts.has_additions()
    }

    pub fn has_removals(&self) -> bool {
        self.counts.has_removals()
    }

    pub fn has_updates(&self) -> bool {
        self.counts.has_updates()
    }

    pub fn touches_existing_records(&self) -> bool {
        self.counts.touches_existing_records()
    }

    pub fn is_additive_only(&self) -> bool {
        self.counts.is_additive_only()
    }
}

const SNAPSHOT_RESTORE_DOMAIN_COUNT: usize = 3;

/// Restore-preview change summary for session records keyed by `session_id`.
///
/// When either side contains duplicate session ids, this remains a best-effort
/// diff and callers should consult the paired integrity reports before using
/// the identifier lists as an exhaustive inventory.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionRestoreDelta {
    #[serde(default)]
    pub added_session_ids: Vec<SessionId>,
    #[serde(default)]
    pub removed_session_ids: Vec<SessionId>,
    #[serde(default)]
    pub updated_session_ids: Vec<SessionId>,
    pub unchanged_count: usize,
}

impl SessionRestoreDelta {
    pub fn change_count(&self) -> usize {
        self.added_session_ids.len()
            + self.removed_session_ids.len()
            + self.updated_session_ids.len()
    }

    pub fn counts(&self) -> RestoreDeltaCounts {
        RestoreDeltaCounts {
            added_count: self.added_session_ids.len(),
            removed_count: self.removed_session_ids.len(),
            updated_count: self.updated_session_ids.len(),
            unchanged_count: self.unchanged_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.change_count() == 0
    }
}

/// Restore-preview change summary for memory records keyed by `id`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MemoryRestoreDelta {
    #[serde(default)]
    pub added_memory_ids: Vec<String>,
    #[serde(default)]
    pub removed_memory_ids: Vec<String>,
    #[serde(default)]
    pub updated_memory_ids: Vec<String>,
    pub unchanged_count: usize,
}

impl MemoryRestoreDelta {
    pub fn change_count(&self) -> usize {
        self.added_memory_ids.len() + self.removed_memory_ids.len() + self.updated_memory_ids.len()
    }

    pub fn counts(&self) -> RestoreDeltaCounts {
        RestoreDeltaCounts {
            added_count: self.added_memory_ids.len(),
            removed_count: self.removed_memory_ids.len(),
            updated_count: self.updated_memory_ids.len(),
            unchanged_count: self.unchanged_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.change_count() == 0
    }
}

/// Restore-preview change summary for transcript entries keyed by `entry_id`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptRestoreDelta {
    #[serde(default)]
    pub added_entry_ids: Vec<String>,
    #[serde(default)]
    pub removed_entry_ids: Vec<String>,
    #[serde(default)]
    pub updated_entry_ids: Vec<String>,
    pub unchanged_count: usize,
}

impl TranscriptRestoreDelta {
    pub fn change_count(&self) -> usize {
        self.added_entry_ids.len() + self.removed_entry_ids.len() + self.updated_entry_ids.len()
    }

    pub fn counts(&self) -> RestoreDeltaCounts {
        RestoreDeltaCounts {
            added_count: self.added_entry_ids.len(),
            removed_count: self.removed_entry_ids.len(),
            updated_count: self.updated_entry_ids.len(),
            unchanged_count: self.unchanged_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.change_count() == 0
    }
}

/// Portable preflight view of what a full snapshot restore would change.
///
/// This complements [`SnapshotInspectionBundle`] and [`SnapshotAuditReport`]
/// with an additive diff surface so automation can preview replace-style
/// restores before mutating a concrete store.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestorePreview {
    pub current_audit: SnapshotAuditReport,
    pub incoming_audit: SnapshotAuditReport,
    pub session_delta: SessionRestoreDelta,
    pub memory_delta: MemoryRestoreDelta,
    pub transcript_delta: TranscriptRestoreDelta,
}

impl SnapshotRestorePreview {
    pub fn from_records_and_entries(
        current_sessions: &[SessionRecord],
        current_memories: &[MemoryRecord],
        current_transcripts: &[TranscriptEntry],
        incoming_sessions: &[SessionRecord],
        incoming_memories: &[MemoryRecord],
        incoming_transcripts: &[TranscriptEntry],
    ) -> Self {
        let current_session_map = current_sessions
            .iter()
            .map(|record| (record.session_id.0.clone(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let incoming_session_map = incoming_sessions
            .iter()
            .map(|record| (record.session_id.0.clone(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let (added_session_ids, removed_session_ids, updated_session_ids, session_unchanged_count) =
            keyed_restore_delta(current_session_map, incoming_session_map);

        let current_memory_map = current_memories
            .iter()
            .map(|record| (record.id.clone(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let incoming_memory_map = incoming_memories
            .iter()
            .map(|record| (record.id.clone(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let (added_memory_ids, removed_memory_ids, updated_memory_ids, memory_unchanged_count) =
            keyed_restore_delta(current_memory_map, incoming_memory_map);

        let current_transcript_map = current_transcripts
            .iter()
            .map(|entry| (entry.entry_id.clone(), entry.clone()))
            .collect::<BTreeMap<_, _>>();
        let incoming_transcript_map = incoming_transcripts
            .iter()
            .map(|entry| (entry.entry_id.clone(), entry.clone()))
            .collect::<BTreeMap<_, _>>();
        let (added_entry_ids, removed_entry_ids, updated_entry_ids, transcript_unchanged_count) =
            keyed_restore_delta(current_transcript_map, incoming_transcript_map);

        Self {
            current_audit: SnapshotAuditReport::from_records_and_entries(
                current_sessions,
                current_memories,
                current_transcripts,
            ),
            incoming_audit: SnapshotAuditReport::from_records_and_entries(
                incoming_sessions,
                incoming_memories,
                incoming_transcripts,
            ),
            session_delta: SessionRestoreDelta {
                added_session_ids: added_session_ids.into_iter().map(SessionId).collect(),
                removed_session_ids: removed_session_ids.into_iter().map(SessionId).collect(),
                updated_session_ids: updated_session_ids.into_iter().map(SessionId).collect(),
                unchanged_count: session_unchanged_count,
            },
            memory_delta: MemoryRestoreDelta {
                added_memory_ids,
                removed_memory_ids,
                updated_memory_ids,
                unchanged_count: memory_unchanged_count,
            },
            transcript_delta: TranscriptRestoreDelta {
                added_entry_ids,
                removed_entry_ids,
                updated_entry_ids,
                unchanged_count: transcript_unchanged_count,
            },
        }
    }

    pub fn change_count(&self) -> usize {
        self.session_delta.change_count()
            + self.memory_delta.change_count()
            + self.transcript_delta.change_count()
    }

    /// Returns a compact automation-friendly summary of restore impact.
    pub fn impact(&self) -> SnapshotRestoreImpact {
        SnapshotRestoreImpact::from_preview(self)
    }

    /// Returns a payload-light readiness summary for restore planning.
    pub fn readiness(&self) -> SnapshotRestoreReadiness {
        SnapshotRestoreReadiness::from_preview(self)
    }

    /// Returns a compact safety summary for restore planning.
    pub fn safety(&self) -> SnapshotRestoreSafety {
        SnapshotRestoreSafety::from_preview(self)
    }

    /// Returns a compact domain-shape summary for restore planning.
    pub fn mutation_profile(&self) -> SnapshotRestoreMutationProfile {
        SnapshotRestoreMutationProfile::from_preview(self)
    }

    /// Returns the restore domains whose delta counts include real changes.
    pub fn changed_domains(&self) -> Vec<SnapshotRestoreDomain> {
        self.domain_impacts()
            .into_iter()
            .filter_map(|impact| (!impact.counts.is_empty()).then_some(impact.domain))
            .collect()
    }

    /// Returns aggregate counts for each restore domain in stable domain order.
    pub fn domain_impacts(&self) -> Vec<SnapshotRestoreDomainImpact> {
        vec![
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: self.session_delta.counts(),
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: self.memory_delta.counts(),
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: self.transcript_delta.counts(),
            },
        ]
    }

    /// Returns the number of restore domains touched by this preview.
    pub fn changed_domain_count(&self) -> usize {
        self.changed_domains().len()
    }

    /// Returns `true` when this preview includes changes for `domain`.
    pub fn touches(&self, domain: SnapshotRestoreDomain) -> bool {
        self.changed_domains().contains(&domain)
    }

    /// Returns the compact delta counts for one restore domain.
    pub fn impact_for(&self, domain: SnapshotRestoreDomain) -> Option<SnapshotRestoreDomainImpact> {
        self.domain_impacts()
            .into_iter()
            .find(|impact| impact.domain == domain)
    }

    /// Returns aggregate added/removed/updated/unchanged counts across all
    /// restore domains.
    pub fn change_totals(&self) -> RestoreDeltaCounts {
        let session = self.session_delta.counts();
        let memory = self.memory_delta.counts();
        let transcript = self.transcript_delta.counts();

        RestoreDeltaCounts {
            added_count: session.added_count + memory.added_count + transcript.added_count,
            removed_count: session.removed_count + memory.removed_count + transcript.removed_count,
            updated_count: session.updated_count + memory.updated_count + transcript.updated_count,
            unchanged_count: session.unchanged_count
                + memory.unchanged_count
                + transcript.unchanged_count,
        }
    }

    pub fn has_integrity_issues(&self) -> bool {
        !self.current_audit.is_clean() || !self.incoming_audit.is_clean()
    }

    pub fn is_noop(&self) -> bool {
        self.change_count() == 0
    }
}

/// Compact impact summary derived from a full restore preview.
///
/// This keeps the aggregate counts, touched domains, and integrity posture in
/// one portable payload for CLI summaries and automation that do not need the
/// per-record identifier lists carried by [`SnapshotRestorePreview`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestoreImpact {
    pub change_totals: RestoreDeltaCounts,
    #[serde(default)]
    pub changed_domains: Vec<SnapshotRestoreDomain>,
    #[serde(default)]
    pub domain_impacts: Vec<SnapshotRestoreDomainImpact>,
    pub current_issue_count: usize,
    pub incoming_issue_count: usize,
}

impl SnapshotRestoreImpact {
    pub fn from_preview(preview: &SnapshotRestorePreview) -> Self {
        let domain_impacts = preview.domain_impacts();
        let changed_domains = preview.changed_domains();

        Self {
            change_totals: preview.change_totals(),
            changed_domains,
            domain_impacts,
            current_issue_count: preview.current_audit.issue_count(),
            incoming_issue_count: preview.incoming_audit.issue_count(),
        }
    }

    pub fn change_count(&self) -> usize {
        self.change_totals.change_count()
    }

    pub fn has_additions(&self) -> bool {
        self.change_totals.has_additions()
    }

    pub fn has_removals(&self) -> bool {
        self.change_totals.has_removals()
    }

    pub fn has_updates(&self) -> bool {
        self.change_totals.has_updates()
    }

    pub fn touches_existing_records(&self) -> bool {
        self.change_totals.touches_existing_records()
    }

    pub fn is_additive_only(&self) -> bool {
        self.change_totals.is_additive_only()
    }

    pub fn changed_domain_count(&self) -> usize {
        self.changed_domains.len()
    }

    pub fn total_issue_count(&self) -> usize {
        self.current_issue_count + self.incoming_issue_count
    }

    pub fn touches(&self, domain: SnapshotRestoreDomain) -> bool {
        self.changed_domains.contains(&domain)
    }

    pub fn impact_for(
        &self,
        domain: SnapshotRestoreDomain,
    ) -> Option<&SnapshotRestoreDomainImpact> {
        self.domain_impacts
            .iter()
            .find(|impact| impact.domain == domain)
    }

    pub fn has_integrity_issues(&self) -> bool {
        self.total_issue_count() > 0
    }

    pub fn is_noop(&self) -> bool {
        self.change_count() == 0
    }

    /// Returns a payload-light readiness summary derived from this impact.
    pub fn readiness(&self) -> SnapshotRestoreReadiness {
        SnapshotRestoreReadiness::from_impact(self)
    }

    /// Returns a compact safety summary derived from this impact.
    pub fn safety(&self) -> SnapshotRestoreSafety {
        SnapshotRestoreSafety::from_impact(self)
    }

    /// Returns a compact domain-shape summary derived from this impact.
    pub fn mutation_profile(&self) -> SnapshotRestoreMutationProfile {
        SnapshotRestoreMutationProfile::from_impact(self)
    }
}

/// Compact restore-planning domain summary.
///
/// Unlike [`SnapshotRestoreImpact`], this keeps only the count of restore
/// domains that changed, how many of those domains are additive-only versus
/// touching existing records, and whether removals or integrity issues are
/// present. This is intended for low-blast-radius automation that needs to
/// quickly answer "how much rewrite pressure does this restore create?"
/// without carrying per-domain vectors or changed identifier lists.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestoreMutationProfile {
    pub changed_domain_count: usize,
    pub unchanged_domain_count: usize,
    pub addition_domain_count: usize,
    pub additive_only_domain_count: usize,
    pub existing_record_domain_count: usize,
    pub removal_domain_count: usize,
    pub current_issue_count: usize,
    pub incoming_issue_count: usize,
}

impl SnapshotRestoreMutationProfile {
    pub fn from_preview(preview: &SnapshotRestorePreview) -> Self {
        Self::from_impact(&preview.impact())
    }

    pub fn from_impact(impact: &SnapshotRestoreImpact) -> Self {
        let domain_impacts = &impact.domain_impacts;
        let changed_domain_count = impact.changed_domain_count();

        let domain_count = domain_impacts
            .len()
            .max(changed_domain_count)
            .max(SNAPSHOT_RESTORE_DOMAIN_COUNT);

        let count_or_fallback = |count: usize, fallback: bool| {
            if domain_impacts.is_empty() {
                usize::from(fallback)
            } else {
                count
            }
        };

        let addition_domain_count = count_or_fallback(
            domain_impacts
                .iter()
                .filter(|impact| impact.has_additions())
                .count(),
            impact.has_additions(),
        );
        let additive_only_domain_count = count_or_fallback(
            domain_impacts
                .iter()
                .filter(|impact| impact.has_changes() && impact.is_additive_only())
                .count(),
            impact.is_additive_only(),
        );
        let existing_record_domain_count = count_or_fallback(
            domain_impacts
                .iter()
                .filter(|impact| impact.touches_existing_records())
                .count(),
            impact.touches_existing_records(),
        );
        let removal_domain_count = count_or_fallback(
            domain_impacts
                .iter()
                .filter(|impact| impact.has_removals())
                .count(),
            impact.has_removals(),
        );

        Self {
            changed_domain_count,
            unchanged_domain_count: domain_count.saturating_sub(changed_domain_count),
            addition_domain_count,
            additive_only_domain_count,
            existing_record_domain_count,
            removal_domain_count,
            current_issue_count: impact.current_issue_count,
            incoming_issue_count: impact.incoming_issue_count,
        }
    }

    pub fn total_issue_count(&self) -> usize {
        self.current_issue_count + self.incoming_issue_count
    }

    pub fn has_changes(&self) -> bool {
        self.changed_domain_count > 0
    }

    pub fn has_additive_domains(&self) -> bool {
        self.additive_only_domain_count > 0
    }

    pub fn touches_existing_records(&self) -> bool {
        self.existing_record_domain_count > 0
    }

    pub fn has_removals(&self) -> bool {
        self.removal_domain_count > 0
    }

    pub fn is_additive_only(&self) -> bool {
        self.has_changes() && !self.touches_existing_records()
    }

    pub fn has_integrity_issues(&self) -> bool {
        self.total_issue_count() > 0
    }

    pub fn is_noop(&self) -> bool {
        !self.has_changes()
    }

    pub fn is_ready(&self) -> bool {
        !self.has_integrity_issues()
    }
}

/// Compact restore-planning readiness summary.
///
/// Unlike [`SnapshotRestoreImpact`], this keeps only aggregate change counts,
/// the number of changed domains, and the integrity posture of the current and
/// incoming snapshots. It is intended for automation that needs to answer
/// "is this restore plan clean and how much does it change?" without carrying
/// per-domain vectors or record identifiers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestoreReadiness {
    pub change_totals: RestoreDeltaCounts,
    pub changed_domain_count: usize,
    pub current_issue_count: usize,
    pub incoming_issue_count: usize,
}

impl SnapshotRestoreReadiness {
    pub fn from_preview(preview: &SnapshotRestorePreview) -> Self {
        Self {
            change_totals: preview.change_totals(),
            changed_domain_count: preview.changed_domain_count(),
            current_issue_count: preview.current_audit.issue_count(),
            incoming_issue_count: preview.incoming_audit.issue_count(),
        }
    }

    pub fn from_impact(impact: &SnapshotRestoreImpact) -> Self {
        Self {
            change_totals: impact.change_totals.clone(),
            changed_domain_count: impact.changed_domain_count(),
            current_issue_count: impact.current_issue_count,
            incoming_issue_count: impact.incoming_issue_count,
        }
    }

    pub fn change_count(&self) -> usize {
        self.change_totals.change_count()
    }

    pub fn has_additions(&self) -> bool {
        self.change_totals.has_additions()
    }

    pub fn has_removals(&self) -> bool {
        self.change_totals.has_removals()
    }

    pub fn has_updates(&self) -> bool {
        self.change_totals.has_updates()
    }

    pub fn touches_existing_records(&self) -> bool {
        self.change_totals.touches_existing_records()
    }

    pub fn is_additive_only(&self) -> bool {
        self.change_totals.is_additive_only()
    }

    pub fn total_issue_count(&self) -> usize {
        self.current_issue_count + self.incoming_issue_count
    }

    pub fn has_changes(&self) -> bool {
        !self.is_noop()
    }

    pub fn has_integrity_issues(&self) -> bool {
        self.total_issue_count() > 0
    }

    pub fn is_noop(&self) -> bool {
        self.change_count() == 0
    }

    pub fn is_ready(&self) -> bool {
        !self.has_integrity_issues()
    }

    /// Returns a compact safety summary derived from this readiness payload.
    pub fn safety(&self) -> SnapshotRestoreSafety {
        SnapshotRestoreSafety::from_readiness(self)
    }
}

/// Compact restore-planning safety summary.
///
/// Unlike [`SnapshotRestoreReadiness`], this persists the derived gating
/// booleans that low-blast-radius automation often wants to inspect directly,
/// including whether the preview changes anything at all, whether it is purely
/// additive, whether it would touch existing records, and whether integrity
/// issues keep the plan from being ready.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestoreSafety {
    pub change_totals: RestoreDeltaCounts,
    pub changed_domain_count: usize,
    pub current_issue_count: usize,
    pub incoming_issue_count: usize,
    pub has_changes: bool,
    pub touches_existing_records: bool,
    pub additive_only: bool,
    pub has_integrity_issues: bool,
}

impl SnapshotRestoreSafety {
    pub fn from_preview(preview: &SnapshotRestorePreview) -> Self {
        Self::from_readiness(&preview.readiness())
    }

    pub fn from_impact(impact: &SnapshotRestoreImpact) -> Self {
        Self::from_readiness(&impact.readiness())
    }

    pub fn from_readiness(readiness: &SnapshotRestoreReadiness) -> Self {
        Self {
            change_totals: readiness.change_totals.clone(),
            changed_domain_count: readiness.changed_domain_count,
            current_issue_count: readiness.current_issue_count,
            incoming_issue_count: readiness.incoming_issue_count,
            has_changes: readiness.has_changes(),
            touches_existing_records: readiness.touches_existing_records(),
            additive_only: readiness.is_additive_only(),
            has_integrity_issues: readiness.has_integrity_issues(),
        }
    }

    pub fn change_count(&self) -> usize {
        self.change_totals.change_count()
    }

    pub fn has_additions(&self) -> bool {
        self.change_totals.has_additions()
    }

    pub fn has_removals(&self) -> bool {
        self.change_totals.has_removals()
    }

    pub fn has_updates(&self) -> bool {
        self.change_totals.has_updates()
    }

    pub fn total_issue_count(&self) -> usize {
        self.current_issue_count + self.incoming_issue_count
    }

    pub fn is_noop(&self) -> bool {
        !self.has_changes
    }

    pub fn is_ready(&self) -> bool {
        !self.has_integrity_issues
    }
}

fn duplicate_non_blank_values(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut counts = BTreeMap::<String, usize>::new();

    for value in values {
        if value.is_empty() {
            continue;
        }
        *counts.entry(value).or_default() += 1;
    }

    counts
        .into_iter()
        .filter_map(|(value, count)| (count > 1).then_some(value))
        .collect()
}

fn keyed_restore_delta<V: PartialEq>(
    current: BTreeMap<String, V>,
    incoming: BTreeMap<String, V>,
) -> (Vec<String>, Vec<String>, Vec<String>, usize) {
    let mut keys = current.keys().cloned().collect::<BTreeSet<_>>();
    keys.extend(incoming.keys().cloned());

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut updated = Vec::new();
    let mut unchanged_count = 0;

    for key in keys {
        match (current.get(&key), incoming.get(&key)) {
            (None, Some(_)) => added.push(key),
            (Some(_), None) => removed.push(key),
            (Some(current_value), Some(incoming_value)) => {
                if current_value == incoming_value {
                    unchanged_count += 1;
                } else {
                    updated.push(key);
                }
            }
            (None, None) => {}
        }
    }

    (added, removed, updated, unchanged_count)
}

/// Session persistence boundary.
///
/// Implementations may expose additional helper methods, but these two calls
/// define the minimum runtime dependency on session storage.
pub trait SessionStore: Send + Sync {
    async fn create(&self, record: SessionRecord) -> Result<(), crate::MemoryError>;
    async fn get(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<SessionRecord>, crate::MemoryError>;
}

/// Transcript persistence boundary used by the runtime.
pub trait TranscriptStore: Send + Sync {
    async fn append(&self, entry: TranscriptEntry) -> Result<(), crate::MemoryError>;
    async fn query(
        &self,
        query: TranscriptQuery,
    ) -> Result<TranscriptQueryReport, crate::MemoryError>;
}

/// Memory persistence boundary used by the runtime.
pub trait MemoryStore: Send + Sync {
    async fn put(&self, record: MemoryRecord) -> Result<(), crate::MemoryError>;
    async fn search(&self, query: MemoryQuery) -> Result<Vec<MemoryRecord>, crate::MemoryError>;
}

/// Optional report-bearing extension for memory retrieval backends.
///
/// This keeps [`MemoryStore`] small for simple adapters while giving richer
/// backends and automation a stable contract for matched counts and truncation
/// metadata, mirroring the report envelope already used by [`TranscriptStore`].
pub trait MemoryReportStore: MemoryStore {
    async fn search_report(
        &self,
        query: MemoryQuery,
    ) -> Result<MemoryQueryReport, crate::MemoryError>;
}

#[cfg(test)]
mod tests {
    use std::{future::Future, pin::Pin};

    use super::*;

    fn sample_transcript_entry(sequence: u64, content: &str) -> TranscriptEntry {
        TranscriptEntry {
            entry_id: format!("entry-{}", sequence),
            session_id: SessionId("session-42".into()),
            sequence,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content: content.into(),
            created_at_unix_ms: 100 + sequence,
            tool_name: None,
            correlation_id: Some("corr-1".into()),
            summary_of_range: None,
        }
    }

    struct StubMemoryReportStore;

    impl MemoryStore for StubMemoryReportStore {
        async fn put(&self, _record: MemoryRecord) -> Result<(), crate::MemoryError> {
            Ok(())
        }

        async fn search(
            &self,
            _query: MemoryQuery,
        ) -> Result<Vec<MemoryRecord>, crate::MemoryError> {
            Ok(Vec::new())
        }
    }

    impl MemoryReportStore for StubMemoryReportStore {
        async fn search_report(
            &self,
            query: MemoryQuery,
        ) -> Result<MemoryQueryReport, crate::MemoryError> {
            Ok(MemoryQueryReport::from_hits(query, 0, Vec::new()))
        }
    }

    fn assert_memory_report_store<T: MemoryReportStore>() {}

    fn reported_search_future<'a, T: MemoryReportStore + ?Sized>(
        store: &'a T,
        query: MemoryQuery,
    ) -> Pin<Box<dyn Future<Output = Result<MemoryQueryReport, crate::MemoryError>> + 'a>> {
        Box::pin(store.search_report(query))
    }

    #[test]
    fn memory_report_store_trait_supports_report_queries() {
        assert_memory_report_store::<StubMemoryReportStore>();

        let _future = reported_search_future(
            &StubMemoryReportStore,
            MemoryQuery {
                text: "snapshot".into(),
                limit: 2,
            },
        );
    }

    #[test]
    fn session_record_roundtrips_through_json() {
        let record = SessionRecord {
            session_id: SessionId("session-42".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("stabilize contracts".into()),
            archived_at_unix_ms: None,
        };

        let json = serde_json::to_string(&record).expect("session record should serialize");
        let parsed: SessionRecord =
            serde_json::from_str(&json).expect("session record should deserialize");

        assert_eq!(parsed, record);
    }

    #[test]
    fn memory_query_roundtrips_through_json() {
        let query = MemoryQuery {
            text: "doctor snapshot".into(),
            limit: 5,
        };

        let json = serde_json::to_string(&query).expect("memory query should serialize");
        let parsed: MemoryQuery =
            serde_json::from_str(&json).expect("memory query should deserialize");

        assert_eq!(parsed, query);
    }

    #[test]
    fn memory_query_report_tracks_counts_and_truncation() {
        let report = MemoryQueryReport::from_hits(
            MemoryQuery {
                text: "snapshot".into(),
                limit: 1,
            },
            2,
            vec![MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::Session,
                content: "snapshot ok".into(),
            }],
        );

        assert_eq!(report.query.text, "snapshot");
        assert_eq!(report.query.limit, 1);
        assert_eq!(report.matched_count, 2);
        assert_eq!(report.returned_count, 1);
        assert!(report.truncated);
        assert_eq!(report.hits.len(), 1);
        assert!(!report.is_empty());
    }

    #[test]
    fn query_report_coverage_roundtrips_through_json() {
        let coverage = QueryReportCoverage {
            returned_count: 1,
            matched_count: 3,
        };

        let json = serde_json::to_string(&coverage).expect("coverage should serialize");
        let parsed: QueryReportCoverage =
            serde_json::from_str(&json).expect("coverage should deserialize");

        assert_eq!(parsed, coverage);
        assert_eq!(parsed.omitted_count(), 2);
        assert!(parsed.is_truncated());
        assert!(!parsed.is_complete());
        assert!(!parsed.is_empty());
    }

    #[test]
    fn query_report_limit_pressure_deserializes_from_sparse_json() {
        let parsed: QueryReportLimitPressure = serde_json::from_str("{}")
            .expect("sparse limit pressure should deserialize with defaults");

        assert_eq!(parsed, QueryReportLimitPressure::default());
        assert!(parsed.is_complete());
        assert!(parsed.is_empty());
    }

    #[test]
    fn memory_query_report_exposes_coverage_and_limit_pressure() {
        let report = MemoryQueryReport::from_hits(
            MemoryQuery {
                text: "snapshot".into(),
                limit: 1,
            },
            3,
            vec![MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::Session,
                content: "snapshot ok".into(),
            }],
        );

        assert!(report.has_hits());
        assert_eq!(report.omitted_count(), 2);
        assert!(!report.is_complete());
        assert_eq!(
            report.coverage(),
            QueryReportCoverage {
                returned_count: 1,
                matched_count: 3,
            }
        );
        assert_eq!(
            report.limit_pressure(),
            QueryReportLimitPressure {
                truncated: true,
                omitted_count: 2,
            }
        );
        assert!(!report.limit_pressure().is_complete());
        assert!(!report.limit_pressure().is_empty());
    }

    #[test]
    fn memory_query_report_roundtrips_through_json() {
        let report = MemoryQueryReport::from_hits(
            MemoryQuery {
                text: "contract".into(),
                limit: 2,
            },
            1,
            vec![MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "contract ready".into(),
            }],
        );

        let json = serde_json::to_string(&report).expect("memory query report should serialize");
        let parsed: MemoryQueryReport =
            serde_json::from_str(&json).expect("memory query report should deserialize");

        assert_eq!(parsed, report);
    }

    #[test]
    fn memory_provider_plane_native_default_is_contract_ready() {
        let report = MemoryProviderPlaneReport::native_default();

        assert_eq!(report.provider_count, 2);
        assert_eq!(report.active_provider_count, 1);
        assert_eq!(report.external_provider_count, 1);
        assert_eq!(report.active_external_provider_count, 0);
        assert!(report.builtin_present);
        assert!(report.exactly_one_external_active_or_none);
        assert!(report.context_fencing_required);
        assert!(report.all_active_providers_prefetch);
        assert!(report.all_active_providers_sync);
        assert!(report.provenance_required);
        assert!(report.deletion_path_available);
        assert!(report.contract_ready());
        assert!(
            report
                .capabilities
                .contains(&MemoryProviderCapability::SemanticSearch)
        );
        assert!(
            report
                .capabilities
                .contains(&MemoryProviderCapability::Conclusions)
        );
    }

    #[test]
    fn memory_provider_plane_rejects_multiple_active_external_providers() {
        let report = MemoryProviderPlaneReport::from_providers(vec![
            MemoryProviderDescriptor::builtin(),
            MemoryProviderDescriptor::external_slot("external-a", MemoryProviderStatus::Active),
            MemoryProviderDescriptor::external_slot("external-b", MemoryProviderStatus::Active),
        ]);

        assert_eq!(report.active_external_provider_count, 2);
        assert!(!report.exactly_one_external_active_or_none);
        assert!(!report.contract_ready());
    }

    #[test]
    fn memory_provider_plane_roundtrips_through_json() {
        let report = MemoryProviderPlaneReport::native_default();
        let json = serde_json::to_string(&report).expect("provider plane should serialize");
        let parsed: MemoryProviderPlaneReport =
            serde_json::from_str(&json).expect("provider plane should deserialize");

        assert_eq!(parsed, report);
        assert!(json.contains("context_fencing_required"));
        assert!(json.contains("external-user-modeling-slot"));
    }

    #[test]
    fn transcript_entry_roundtrips_through_json() {
        let entry = TranscriptEntry {
            entry_id: "entry-1".into(),
            session_id: SessionId("session-42".into()),
            sequence: 7,
            kind: TranscriptEntryKind::Summary,
            role: Some(MessageRole::Assistant),
            content: "condensed summary".into(),
            created_at_unix_ms: 77,
            tool_name: Some("write".into()),
            correlation_id: Some("corr-42".into()),
            summary_of_range: Some(TranscriptRange {
                start_sequence: 1,
                end_sequence: 6,
            }),
        };

        let json = serde_json::to_string(&entry).expect("transcript entry should serialize");
        let parsed: TranscriptEntry =
            serde_json::from_str(&json).expect("transcript entry should deserialize");

        assert_eq!(parsed, entry);
        assert!(
            parsed
                .summary_of_range
                .as_ref()
                .expect("summary range should be present")
                .contains(4)
        );
    }

    #[test]
    fn transcript_query_report_tracks_counts_and_truncation() {
        let report = TranscriptQueryReport::from_hits(
            TranscriptQuery {
                session_id: Some(SessionId("session-42".into())),
                text: "approval".into(),
                limit: 1,
            },
            2,
            vec![TranscriptSpan {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 3,
                    end_sequence: 4,
                },
                entry_count: 2,
                excerpt: Some("approval was requested".into()),
                entries: vec![
                    sample_transcript_entry(3, "please approve"),
                    sample_transcript_entry(4, "approval granted"),
                ],
            }],
        );

        assert_eq!(report.query.text, "approval");
        assert_eq!(report.matched_count, 2);
        assert_eq!(report.returned_count, 1);
        assert!(report.truncated);
        assert!(!report.is_empty());
        assert_eq!(report.hits[0].entry_count, 2);
        assert!(!report.hits[0].is_empty());
    }

    #[test]
    fn transcript_query_report_exposes_coverage_and_limit_pressure() {
        let report = TranscriptQueryReport::from_hits(
            TranscriptQuery {
                session_id: Some(SessionId("session-42".into())),
                text: "approval".into(),
                limit: 1,
            },
            2,
            vec![TranscriptSpan::from_entry(sample_transcript_entry(
                3,
                "approval granted",
            ))],
        );

        assert!(report.has_hits());
        assert_eq!(report.omitted_count(), 1);
        assert!(!report.is_complete());
        assert_eq!(
            report.coverage(),
            QueryReportCoverage {
                returned_count: 1,
                matched_count: 2,
            }
        );
        assert_eq!(
            report.limit_pressure(),
            QueryReportLimitPressure {
                truncated: true,
                omitted_count: 1,
            }
        );
        assert!(report.limit_pressure().truncated);
    }

    #[test]
    fn transcript_entry_matches_query_with_optional_session_filter() {
        let entry = sample_transcript_entry(3, "approval granted");

        assert!(entry.matches_query(&TranscriptQuery {
            session_id: None,
            text: "approval".into(),
            limit: 5,
        }));
        assert!(entry.matches_query(&TranscriptQuery {
            session_id: Some(SessionId("session-42".into())),
            text: "granted".into(),
            limit: 5,
        }));
        assert!(!entry.matches_query(&TranscriptQuery {
            session_id: Some(SessionId("session-7".into())),
            text: "granted".into(),
            limit: 5,
        }));
        assert!(!entry.matches_query(&TranscriptQuery {
            session_id: None,
            text: "timeout".into(),
            limit: 5,
        }));
    }

    #[test]
    fn transcript_span_from_entry_builds_single_entry_range() {
        let entry = sample_transcript_entry(11, "snapshot restored");

        let span = TranscriptSpan::from_entry(entry.clone());

        assert_eq!(span.session_id, entry.session_id);
        assert_eq!(span.range.start_sequence, 11);
        assert_eq!(span.range.end_sequence, 11);
        assert_eq!(span.entry_count, 1);
        assert_eq!(span.excerpt.as_deref(), Some("snapshot restored"));
        assert_eq!(span.entries, vec![entry]);
        assert!(!span.is_empty());
    }

    #[test]
    fn context_recall_bundle_roundtrips_through_json() {
        let bundle = ContextRecallBundle {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("tool failure".into()),
                recent_window_limit: 8,
                transcript_limit: 3,
                memory_limit: 2,
                allow_cross_session: true,
            },
            recent_entries: vec![sample_transcript_entry(10, "tool failed with timeout")],
            transcript_hits: vec![TranscriptSpan {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 8,
                    end_sequence: 10,
                },
                entry_count: 3,
                excerpt: Some("failure span".into()),
                entries: vec![
                    sample_transcript_entry(8, "run tool"),
                    sample_transcript_entry(9, "tool timeout"),
                    sample_transcript_entry(10, "retry requested"),
                ],
            }],
            durable_memory_hits: vec![MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "user prefers retry with bounded timeout".into(),
            }],
            summary_hits: vec![MemoryRecord {
                id: "memory-2".into(),
                scope: MemoryScope::Session,
                content: "earlier tool failure cluster summary".into(),
            }],
            active_topic_sessions: vec![],
            active_neurons: Vec::new(),
            budget: ContextBudget::default(),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: false,
        };

        let json = serde_json::to_string(&bundle).expect("context recall bundle should serialize");
        let parsed: ContextRecallBundle =
            serde_json::from_str(&json).expect("context recall bundle should deserialize");

        assert_eq!(parsed, bundle);
        assert!(!parsed.is_empty());
        assert_eq!(parsed.active_topic_session_count(), 0);
        assert_eq!(parsed.transcript_hits[0].range.start_sequence, 8);
        assert_eq!(
            parsed.source_counts(),
            ContextRecallSourceCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 1,
            }
        );
    }

    #[test]
    fn context_recall_source_counts_roundtrip_and_totals_stay_compact() {
        let counts = ContextRecallSourceCounts {
            recent_entry_count: 2,
            transcript_hit_count: 3,
            durable_memory_hit_count: 1,
            summary_hit_count: 4,
        };

        let json = serde_json::to_string(&counts).expect("source counts should serialize");
        let parsed: ContextRecallSourceCounts =
            serde_json::from_str(&json).expect("source counts should deserialize");

        assert_eq!(parsed, counts);
        assert_eq!(parsed.query_hit_count(), 8);
        assert_eq!(parsed.total_item_count(), 10);
        assert!(parsed.has_query_matches());
        assert!(!parsed.is_empty());
    }

    #[test]
    fn context_recall_source_counts_deserialize_from_sparse_json() {
        let parsed: ContextRecallSourceCounts = serde_json::from_str("{}")
            .expect("sparse source counts should deserialize with defaults");

        assert_eq!(parsed, ContextRecallSourceCounts::default());
        assert_eq!(parsed.query_hit_count(), 0);
        assert_eq!(parsed.total_item_count(), 0);
        assert!(!parsed.has_query_matches());
        assert!(parsed.is_empty());
    }

    #[test]
    fn context_recall_report_roundtrips_through_json() {
        let report = ContextRecallReport {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("tool failure".into()),
                recent_window_limit: 8,
                transcript_limit: 3,
                memory_limit: 2,
                allow_cross_session: true,
            },
            source_counts: ContextRecallSourceCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 1,
            },
            truncated: false,
        };

        let json = serde_json::to_string(&report).expect("context recall report should serialize");
        let parsed: ContextRecallReport =
            serde_json::from_str(&json).expect("context recall report should deserialize");

        assert_eq!(parsed, report);
        assert_eq!(parsed.query_hit_count(), 3);
        assert_eq!(parsed.total_item_count(), 4);
        assert!(parsed.has_query_matches());
        assert!(!parsed.is_empty());
    }

    #[test]
    fn context_recall_availability_roundtrips_through_json() {
        let availability = ContextRecallAvailability {
            total_recent_entry_count: 5,
            total_transcript_match_count: 3,
            total_memory_match_count: 2,
        };

        let json = serde_json::to_string(&availability).expect("availability should serialize");
        let parsed: ContextRecallAvailability =
            serde_json::from_str(&json).expect("availability should deserialize");

        assert_eq!(parsed, availability);
        assert_eq!(parsed.query_match_count(), 5);
        assert_eq!(parsed.total_item_count(), 10);
        assert!(parsed.has_query_matches());
        assert!(!parsed.is_empty());
    }

    #[test]
    fn context_recall_availability_deserializes_from_sparse_json() {
        let parsed: ContextRecallAvailability = serde_json::from_str("{}")
            .expect("sparse availability should deserialize with defaults");

        assert_eq!(parsed, ContextRecallAvailability::default());
        assert_eq!(parsed.query_match_count(), 0);
        assert_eq!(parsed.total_item_count(), 0);
        assert!(!parsed.has_query_matches());
        assert!(parsed.is_empty());
    }

    #[test]
    fn context_recall_source_availability_roundtrips_through_json() {
        let availability = ContextRecallSourceAvailability {
            recent_entry_count: 5,
            transcript_match_count: 3,
            durable_memory_match_count: 2,
            summary_memory_match_count: 4,
        };

        let json =
            serde_json::to_string(&availability).expect("source availability should serialize");
        let parsed: ContextRecallSourceAvailability =
            serde_json::from_str(&json).expect("source availability should deserialize");

        assert_eq!(parsed, availability);
        assert_eq!(parsed.memory_match_count(), 6);
        assert_eq!(parsed.query_match_count(), 9);
        assert_eq!(parsed.total_item_count(), 14);
        assert!(parsed.has_query_matches());
        assert!(!parsed.is_empty());
    }

    #[test]
    fn context_recall_source_availability_deserializes_from_sparse_json() {
        let parsed: ContextRecallSourceAvailability = serde_json::from_str("{}")
            .expect("sparse source availability should deserialize with defaults");

        assert_eq!(parsed, ContextRecallSourceAvailability::default());
        assert_eq!(parsed.memory_match_count(), 0);
        assert_eq!(parsed.query_match_count(), 0);
        assert_eq!(parsed.total_item_count(), 0);
        assert!(!parsed.has_query_matches());
        assert!(parsed.is_empty());
    }

    #[test]
    fn context_recall_inspection_tracks_availability_and_limit_pressure() {
        let bundle = ContextRecallBundle {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("tool failure".into()),
                recent_window_limit: 2,
                transcript_limit: 1,
                memory_limit: 1,
                allow_cross_session: true,
            },
            recent_entries: vec![
                sample_transcript_entry(9, "tool timeout"),
                sample_transcript_entry(10, "retry requested"),
            ],
            transcript_hits: vec![TranscriptSpan::from_entry(sample_transcript_entry(
                8,
                "tool failure span",
            ))],
            durable_memory_hits: vec![MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "retry guidance".into(),
            }],
            summary_hits: vec![],
            active_topic_sessions: vec![],
            active_neurons: Vec::new(),
            budget: ContextBudget::default(),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: true,
        };

        let inspection = bundle.inspection(ContextRecallAvailability {
            total_recent_entry_count: 4,
            total_transcript_match_count: 3,
            total_memory_match_count: 2,
        });

        assert_eq!(inspection.report, bundle.report());
        assert_eq!(inspection.source_transcript_spans.len(), 2);
        assert!(inspection.source_transcript_spans.iter().any(|span| {
            span.session_id.0 == "session-42"
                && span.range.start_sequence == 9
                && span.range.end_sequence == 10
                && span.reason.as_deref() == Some("recent_window")
        }));
        assert!(inspection.source_transcript_spans.iter().any(|span| {
            span.session_id.0 == "session-42"
                && span.range.start_sequence == 8
                && span.range.end_sequence == 8
                && span.reason.as_deref() == Some("query_match")
        }));
        assert_eq!(inspection.returned_memory_hit_count(), 1);
        assert_eq!(inspection.returned_query_hit_count(), 2);
        assert_eq!(inspection.returned_total_item_count(), 4);
        assert_eq!(inspection.omitted_recent_entry_count(), 2);
        assert_eq!(inspection.omitted_transcript_hit_count(), 2);
        assert_eq!(inspection.omitted_memory_hit_count(), 1);
        assert_eq!(inspection.omitted_query_hit_count(), 3);
        assert_eq!(inspection.omitted_total_item_count(), 5);
        assert_eq!(
            inspection.omission_counts(),
            ContextRecallOmissionCounts {
                recent_entry_count: 2,
                transcript_hit_count: 2,
                memory_hit_count: 1,
                query_hit_count: 3,
                total_item_count: 5,
            }
        );
        assert_eq!(inspection.matched_query_hit_count(), 5);
        assert_eq!(inspection.matched_total_item_count(), 9);
        assert!(inspection.has_query_matches());
        assert!(inspection.has_omissions());
        assert!(inspection.recent_entries_truncated());
        assert!(inspection.transcript_hits_truncated());
        assert!(inspection.memory_hits_truncated());
        assert!(!inspection.is_complete());
        assert!(!inspection.is_empty());
    }

    #[test]
    fn context_recall_inspection_roundtrips_through_json() {
        let inspection = ContextRecallInspection {
            report: ContextRecallReport {
                request: ContextRecallRequest {
                    session_id: SessionId("session-42".into()),
                    query_text: Some("timeout".into()),
                    recent_window_limit: 4,
                    transcript_limit: 2,
                    memory_limit: 2,
                    allow_cross_session: true,
                },
                source_counts: ContextRecallSourceCounts {
                    recent_entry_count: 2,
                    transcript_hit_count: 1,
                    durable_memory_hit_count: 1,
                    summary_hit_count: 0,
                },
                truncated: false,
            },
            availability: ContextRecallAvailability {
                total_recent_entry_count: 2,
                total_transcript_match_count: 1,
                total_memory_match_count: 1,
            },
            source_transcript_spans: vec![TranscriptSpanRef {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 7,
                    end_sequence: 8,
                },
                reason: Some("recent_window, query_match".into()),
            }],
        };

        let json = serde_json::to_string(&inspection).expect("inspection should serialize");
        let parsed: ContextRecallInspection =
            serde_json::from_str(&json).expect("inspection should deserialize");

        assert_eq!(parsed, inspection);
        assert_eq!(parsed.omitted_recent_entry_count(), 0);
        assert_eq!(parsed.omitted_transcript_hit_count(), 0);
        assert_eq!(parsed.omitted_memory_hit_count(), 0);
        assert_eq!(parsed.omitted_query_hit_count(), 0);
        assert_eq!(parsed.omitted_total_item_count(), 0);
        assert!(!parsed.has_omissions());
        assert!(parsed.is_complete());
        assert!(!parsed.recent_entries_truncated());
        assert!(!parsed.transcript_hits_truncated());
        assert!(!parsed.memory_hits_truncated());
        assert_eq!(parsed.source_transcript_spans.len(), 1);
    }

    #[test]
    fn context_recall_transcript_provenance_summary_tracks_sessions_and_reasons() {
        let inspection = ContextRecallInspection {
            report: ContextRecallReport {
                request: ContextRecallRequest {
                    session_id: SessionId("session-42".into()),
                    query_text: Some("timeout".into()),
                    recent_window_limit: 4,
                    transcript_limit: 2,
                    memory_limit: 2,
                    allow_cross_session: true,
                },
                source_counts: ContextRecallSourceCounts {
                    recent_entry_count: 2,
                    transcript_hit_count: 1,
                    durable_memory_hit_count: 1,
                    summary_hit_count: 0,
                },
                truncated: false,
            },
            availability: ContextRecallAvailability {
                total_recent_entry_count: 2,
                total_transcript_match_count: 1,
                total_memory_match_count: 1,
            },
            source_transcript_spans: vec![
                TranscriptSpanRef {
                    session_id: SessionId("session-42".into()),
                    range: TranscriptRange {
                        start_sequence: 7,
                        end_sequence: 8,
                    },
                    reason: Some("recent_window, query_match".into()),
                },
                TranscriptSpanRef {
                    session_id: SessionId("session-99".into()),
                    range: TranscriptRange {
                        start_sequence: 4,
                        end_sequence: 4,
                    },
                    reason: Some("active_topic_session, query_match".into()),
                },
                TranscriptSpanRef {
                    session_id: SessionId(" ".into()),
                    range: TranscriptRange {
                        start_sequence: 1,
                        end_sequence: 1,
                    },
                    reason: None,
                },
            ],
        };

        let summary = inspection.transcript_provenance_summary();

        assert_eq!(
            summary,
            ContextRecallTranscriptProvenanceSummary {
                span_count: 3,
                session_count: 2,
                spans_with_reason_count: 2,
                distinct_reason_count: 3,
            }
        );
        assert!(summary.has_spans());
        assert!(summary.has_reasons());
        assert!(!summary.is_empty());
    }

    #[test]
    fn context_recall_bundle_transcript_provenance_summary_matches_inspection_summary() {
        let bundle = ContextRecallBundle {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("tool failure".into()),
                recent_window_limit: 2,
                transcript_limit: 1,
                memory_limit: 1,
                allow_cross_session: true,
            },
            recent_entries: vec![
                sample_transcript_entry(9, "tool timeout"),
                sample_transcript_entry(10, "retry requested"),
            ],
            transcript_hits: vec![TranscriptSpan::from_entry(sample_transcript_entry(
                8,
                "tool failure span",
            ))],
            durable_memory_hits: vec![],
            summary_hits: vec![],
            active_topic_sessions: vec![],
            active_neurons: Vec::new(),
            budget: ContextBudget::default(),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: true,
        };
        let inspection = bundle.inspection(ContextRecallAvailability {
            total_recent_entry_count: 4,
            total_transcript_match_count: 3,
            total_memory_match_count: 0,
        });

        assert_eq!(
            bundle.transcript_provenance_summary(),
            inspection.transcript_provenance_summary()
        );
    }

    #[test]
    fn context_recall_coverage_counts_track_omissions_and_completeness() {
        let counts = ContextRecallCoverageCounts {
            returned_count: 2,
            available_count: 5,
        };

        assert_eq!(counts.omitted_count(), 3);
        assert!(counts.is_truncated());
        assert!(!counts.is_complete());
        assert!(!counts.is_empty());
    }

    #[test]
    fn context_recall_coverage_rolls_up_inspection_counts() {
        let inspection = ContextRecallInspection {
            report: ContextRecallReport {
                request: ContextRecallRequest {
                    session_id: SessionId("session-42".into()),
                    query_text: Some("timeout".into()),
                    recent_window_limit: 2,
                    transcript_limit: 1,
                    memory_limit: 1,
                    allow_cross_session: true,
                },
                source_counts: ContextRecallSourceCounts {
                    recent_entry_count: 2,
                    transcript_hit_count: 1,
                    durable_memory_hit_count: 1,
                    summary_hit_count: 0,
                },
                truncated: true,
            },
            availability: ContextRecallAvailability {
                total_recent_entry_count: 4,
                total_transcript_match_count: 3,
                total_memory_match_count: 2,
            },
            source_transcript_spans: vec![TranscriptSpanRef {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 9,
                    end_sequence: 10,
                },
                reason: Some("recent_window".into()),
            }],
        };

        let coverage = inspection.coverage();

        assert_eq!(
            coverage,
            ContextRecallCoverage {
                recent_entries: ContextRecallCoverageCounts {
                    returned_count: 2,
                    available_count: 4,
                },
                transcript_hits: ContextRecallCoverageCounts {
                    returned_count: 1,
                    available_count: 3,
                },
                memory_hits: ContextRecallCoverageCounts {
                    returned_count: 1,
                    available_count: 2,
                },
                query_hits: ContextRecallCoverageCounts {
                    returned_count: 2,
                    available_count: 5,
                },
                total_items: ContextRecallCoverageCounts {
                    returned_count: 4,
                    available_count: 9,
                },
            }
        );
        assert_eq!(coverage.omitted_total_item_count(), 5);
        assert_eq!(
            coverage.omission_counts(),
            ContextRecallOmissionCounts {
                recent_entry_count: 2,
                transcript_hit_count: 2,
                memory_hit_count: 1,
                query_hit_count: 3,
                total_item_count: 5,
            }
        );
        assert!(coverage.has_omissions());
        assert!(!coverage.is_complete());
        assert!(!coverage.is_empty());
    }

    #[test]
    fn context_recall_omission_counts_roundtrip_through_json() {
        let counts = ContextRecallOmissionCounts {
            recent_entry_count: 2,
            transcript_hit_count: 1,
            memory_hit_count: 3,
            query_hit_count: 4,
            total_item_count: 6,
        };

        let json = serde_json::to_string(&counts).expect("omission counts should serialize");
        let parsed: ContextRecallOmissionCounts =
            serde_json::from_str(&json).expect("omission counts should deserialize");

        assert_eq!(parsed, counts);
        assert!(parsed.has_omissions());
        assert!(!parsed.is_empty());
    }

    #[test]
    fn context_recall_omission_counts_deserialize_from_sparse_json() {
        let parsed: ContextRecallOmissionCounts = serde_json::from_str("{}")
            .expect("sparse omission counts should deserialize with defaults");

        assert_eq!(parsed, ContextRecallOmissionCounts::default());
        assert!(!parsed.has_omissions());
        assert!(parsed.is_empty());
    }

    #[test]
    fn context_recall_limit_pressure_rolls_up_inspection_flags_and_omissions() {
        let inspection = ContextRecallInspection {
            report: ContextRecallReport {
                request: ContextRecallRequest {
                    session_id: SessionId("session-42".into()),
                    query_text: Some("timeout".into()),
                    recent_window_limit: 2,
                    transcript_limit: 1,
                    memory_limit: 1,
                    allow_cross_session: true,
                },
                source_counts: ContextRecallSourceCounts {
                    recent_entry_count: 2,
                    transcript_hit_count: 1,
                    durable_memory_hit_count: 1,
                    summary_hit_count: 0,
                },
                truncated: true,
            },
            availability: ContextRecallAvailability {
                total_recent_entry_count: 4,
                total_transcript_match_count: 3,
                total_memory_match_count: 2,
            },
            source_transcript_spans: vec![],
        };

        let pressure = inspection.limit_pressure();

        assert_eq!(
            pressure,
            ContextRecallLimitPressure {
                recent_entries_truncated: true,
                transcript_hits_truncated: true,
                memory_hits_truncated: true,
                omission_counts: ContextRecallOmissionCounts {
                    recent_entry_count: 2,
                    transcript_hit_count: 2,
                    memory_hit_count: 1,
                    query_hit_count: 3,
                    total_item_count: 5,
                },
            }
        );
        assert!(pressure.query_hits_truncated());
        assert!(pressure.has_omissions());
        assert!(!pressure.is_complete());
        assert!(!pressure.is_empty());
    }

    #[test]
    fn context_recall_limit_pressure_roundtrips_through_json() {
        let pressure = ContextRecallLimitPressure {
            recent_entries_truncated: false,
            transcript_hits_truncated: true,
            memory_hits_truncated: false,
            omission_counts: ContextRecallOmissionCounts {
                recent_entry_count: 0,
                transcript_hit_count: 2,
                memory_hit_count: 0,
                query_hit_count: 2,
                total_item_count: 2,
            },
        };

        let json = serde_json::to_string(&pressure).expect("limit pressure should serialize");
        let parsed: ContextRecallLimitPressure =
            serde_json::from_str(&json).expect("limit pressure should deserialize");

        assert_eq!(parsed, pressure);
        assert!(parsed.query_hits_truncated());
        assert!(parsed.has_omissions());
        assert!(!parsed.is_complete());
        assert!(!parsed.is_empty());
    }

    #[test]
    fn context_recall_limit_pressure_deserializes_from_sparse_json() {
        let parsed: ContextRecallLimitPressure = serde_json::from_str("{}")
            .expect("sparse limit pressure should deserialize with defaults");

        assert_eq!(parsed, ContextRecallLimitPressure::default());
        assert!(!parsed.query_hits_truncated());
        assert!(!parsed.has_omissions());
        assert!(parsed.is_complete());
        assert!(parsed.is_empty());
    }

    #[test]
    fn context_recall_bundle_builds_bounded_transcript_provenance_refs() {
        let bundle = ContextRecallBundle {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("tool failure".into()),
                recent_window_limit: 3,
                transcript_limit: 2,
                memory_limit: 2,
                allow_cross_session: true,
            },
            recent_entries: vec![
                sample_transcript_entry(8, "run tool"),
                sample_transcript_entry(9, "tool timeout"),
                sample_transcript_entry(10, "retry requested"),
            ],
            transcript_hits: vec![TranscriptSpan {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 8,
                    end_sequence: 10,
                },
                entry_count: 3,
                excerpt: Some("failure span".into()),
                entries: vec![
                    sample_transcript_entry(8, "run tool"),
                    sample_transcript_entry(9, "tool timeout"),
                    sample_transcript_entry(10, "retry requested"),
                ],
            }],
            durable_memory_hits: vec![],
            summary_hits: vec![],
            active_topic_sessions: vec![TopicSession {
                topic_session_id: "topic-session-bootstrap:session-42".into(),
                topic_id: crate::TopicId("topic-session-42".into()),
                topic_label: crate::TopicLabel("tool failure".into()),
                topic_embedding: None,
                linked_surface_session_ids: vec![SessionId("session-42".into())],
                linked_transcript_spans: vec![TranscriptSpanRef {
                    session_id: SessionId("session-42".into()),
                    range: TranscriptRange {
                        start_sequence: 8,
                        end_sequence: 10,
                    },
                    reason: Some("query_match".into()),
                }],
                open_loops: vec![],
                entities: BTreeMap::new(),
                graph_edges: vec![],
                durable_memory_refs: vec![],
                status: TopicSessionStatus::Active,
                created_at_unix_ms: 100,
                last_active_unix_ms: 110,
            }],
            active_neurons: Vec::new(),
            budget: ContextBudget::default(),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: false,
        };

        let refs = bundle.source_transcript_spans();

        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].session_id.0, "session-42");
        assert_eq!(refs[0].range.start_sequence, 8);
        assert_eq!(refs[0].range.end_sequence, 10);
        assert_eq!(
            refs[0].reason.as_deref(),
            Some("recent_window, query_match, active_topic_session")
        );
    }

    #[test]
    fn context_recall_coverage_roundtrips_through_json() {
        let coverage = ContextRecallCoverage {
            recent_entries: ContextRecallCoverageCounts {
                returned_count: 2,
                available_count: 2,
            },
            transcript_hits: ContextRecallCoverageCounts {
                returned_count: 1,
                available_count: 1,
            },
            memory_hits: ContextRecallCoverageCounts {
                returned_count: 3,
                available_count: 3,
            },
            query_hits: ContextRecallCoverageCounts {
                returned_count: 4,
                available_count: 4,
            },
            total_items: ContextRecallCoverageCounts {
                returned_count: 6,
                available_count: 6,
            },
        };

        let json = serde_json::to_string(&coverage).expect("coverage should serialize");
        let parsed: ContextRecallCoverage =
            serde_json::from_str(&json).expect("coverage should deserialize");

        assert_eq!(parsed, coverage);
        assert!(!parsed.has_omissions());
        assert!(parsed.is_complete());
        assert!(!parsed.is_empty());
    }

    #[test]
    fn context_recall_coverage_deserializes_from_sparse_json() {
        let parsed: ContextRecallCoverage =
            serde_json::from_str("{}").expect("sparse coverage should deserialize with defaults");

        assert_eq!(parsed, ContextRecallCoverage::default());
        assert_eq!(parsed.omitted_total_item_count(), 0);
        assert!(!parsed.has_omissions());
        assert!(parsed.is_complete());
        assert!(parsed.is_empty());
    }

    #[test]
    fn context_recall_coverage_limit_pressure_matches_coverage_omissions() {
        let coverage = ContextRecallCoverage {
            recent_entries: ContextRecallCoverageCounts {
                returned_count: 2,
                available_count: 4,
            },
            transcript_hits: ContextRecallCoverageCounts {
                returned_count: 1,
                available_count: 3,
            },
            memory_hits: ContextRecallCoverageCounts {
                returned_count: 1,
                available_count: 2,
            },
            query_hits: ContextRecallCoverageCounts {
                returned_count: 2,
                available_count: 5,
            },
            total_items: ContextRecallCoverageCounts {
                returned_count: 4,
                available_count: 9,
            },
        };

        let pressure = coverage.limit_pressure();

        assert_eq!(
            pressure,
            ContextRecallLimitPressure {
                recent_entries_truncated: true,
                transcript_hits_truncated: true,
                memory_hits_truncated: true,
                omission_counts: ContextRecallOmissionCounts {
                    recent_entry_count: 2,
                    transcript_hit_count: 2,
                    memory_hit_count: 1,
                    query_hit_count: 3,
                    total_item_count: 5,
                },
            }
        );
        assert!(pressure.query_hits_truncated());
        assert!(pressure.has_omissions());
        assert!(!pressure.is_complete());
    }

    #[test]
    fn context_recall_request_normalizes_blank_queries_and_builds_queries() {
        let request = ContextRecallRequest {
            session_id: SessionId("session-42".into()),
            query_text: Some("   ".into()),
            recent_window_limit: 8,
            transcript_limit: 3,
            memory_limit: 2,
            allow_cross_session: true,
        };

        assert_eq!(request.normalized_query_text(), None);
        assert!(!request.has_query_text());
        assert_eq!(
            request.transcript_query(),
            TranscriptQuery {
                session_id: Some(SessionId("session-42".into())),
                text: String::new(),
                limit: 3,
            }
        );
        assert_eq!(
            request.memory_query(),
            MemoryQuery {
                text: String::new(),
                limit: 2,
            }
        );
    }

    #[test]
    fn context_recall_request_cross_session_flag_does_not_change_portable_queries() {
        let session_scoped = ContextRecallRequest {
            session_id: SessionId("session-42".into()),
            query_text: Some(" timeout ".into()),
            recent_window_limit: 8,
            transcript_limit: 3,
            memory_limit: 2,
            allow_cross_session: false,
        };
        let mut cross_session = session_scoped.clone();
        cross_session.allow_cross_session = true;

        assert_eq!(
            session_scoped.transcript_query(),
            cross_session.transcript_query()
        );
        assert_eq!(session_scoped.memory_query(), cross_session.memory_query());
        assert_eq!(
            cross_session.transcript_query(),
            TranscriptQuery {
                session_id: Some(SessionId("session-42".into())),
                text: "timeout".into(),
                limit: 3,
            }
        );
        assert_eq!(
            cross_session.memory_query(),
            MemoryQuery {
                text: "timeout".into(),
                limit: 2,
            }
        );
    }

    #[test]
    fn context_recall_bundle_reports_query_hit_and_total_item_counts() {
        let bundle = ContextRecallBundle {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("tool failure".into()),
                recent_window_limit: 8,
                transcript_limit: 3,
                memory_limit: 2,
                allow_cross_session: true,
            },
            recent_entries: vec![sample_transcript_entry(10, "tool failed with timeout")],
            transcript_hits: vec![TranscriptSpan {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 8,
                    end_sequence: 10,
                },
                entry_count: 3,
                excerpt: Some("failure span".into()),
                entries: vec![
                    sample_transcript_entry(8, "run tool"),
                    sample_transcript_entry(9, "tool timeout"),
                    sample_transcript_entry(10, "retry requested"),
                ],
            }],
            durable_memory_hits: vec![MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "user prefers retry with bounded timeout".into(),
            }],
            summary_hits: vec![MemoryRecord {
                id: "memory-2".into(),
                scope: MemoryScope::Session,
                content: "earlier tool failure cluster summary".into(),
            }],
            active_topic_sessions: vec![],
            active_neurons: Vec::new(),
            budget: ContextBudget::default(),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: false,
        };

        assert_eq!(
            bundle.source_counts(),
            ContextRecallSourceCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 1,
            }
        );

        assert_eq!(bundle.query_hit_count(), 3);
        assert_eq!(bundle.total_item_count(), 4);
        assert!(bundle.has_query_matches());
        assert_eq!(bundle.report().request, bundle.request);
        assert_eq!(bundle.report().source_counts, bundle.source_counts());
        assert!(!bundle.report().truncated);
    }

    #[test]
    fn transcript_snapshot_stats_roll_up_entry_kinds_and_sessions() {
        let entries = vec![
            sample_transcript_entry(1, "user message"),
            TranscriptEntry {
                entry_id: "entry-2".into(),
                session_id: SessionId("session-42".into()),
                sequence: 2,
                kind: TranscriptEntryKind::ToolCall,
                role: Some(MessageRole::Assistant),
                content: "call write".into(),
                created_at_unix_ms: 102,
                tool_name: Some("write".into()),
                correlation_id: Some("corr-1".into()),
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "entry-3".into(),
                session_id: SessionId("session-42".into()),
                sequence: 3,
                kind: TranscriptEntryKind::ToolResult,
                role: Some(MessageRole::Tool),
                content: "write ok".into(),
                created_at_unix_ms: 103,
                tool_name: Some("write".into()),
                correlation_id: Some("corr-1".into()),
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "entry-4".into(),
                session_id: SessionId("session-42".into()),
                sequence: 4,
                kind: TranscriptEntryKind::Approval,
                role: Some(MessageRole::Assistant),
                content: "approval granted".into(),
                created_at_unix_ms: 104,
                tool_name: None,
                correlation_id: Some("corr-2".into()),
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "entry-5".into(),
                session_id: SessionId("session-77".into()),
                sequence: 1,
                kind: TranscriptEntryKind::Summary,
                role: Some(MessageRole::Assistant),
                content: "session summary".into(),
                created_at_unix_ms: 105,
                tool_name: None,
                correlation_id: Some("corr-3".into()),
                summary_of_range: Some(TranscriptRange {
                    start_sequence: 1,
                    end_sequence: 4,
                }),
            },
            TranscriptEntry {
                entry_id: "entry-6".into(),
                session_id: SessionId("session-77".into()),
                sequence: 2,
                kind: TranscriptEntryKind::Event,
                role: None,
                content: "session archived".into(),
                created_at_unix_ms: 106,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
        ];

        let stats = TranscriptSnapshotStats::from_entries(&entries);

        assert_eq!(stats.total_entry_count, 6);
        assert_eq!(stats.session_count, 2);
        assert_eq!(stats.message_count, 1);
        assert_eq!(stats.tool_call_count, 1);
        assert_eq!(stats.tool_result_count, 1);
        assert_eq!(stats.approval_count, 1);
        assert_eq!(stats.summary_count, 1);
        assert_eq!(stats.event_count, 1);
        assert!(!stats.is_empty());
    }

    #[test]
    fn transcript_snapshot_manifest_sorts_entries_and_tracks_sizes() {
        let manifest = TranscriptSnapshotManifest::from_entries(&[
            TranscriptEntry {
                entry_id: "entry-b".into(),
                session_id: SessionId("session-z".into()),
                sequence: 5,
                kind: TranscriptEntryKind::ToolResult,
                role: Some(MessageRole::Tool),
                content: "tool result payload".into(),
                created_at_unix_ms: 200,
                tool_name: Some("write".into()),
                correlation_id: None,
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "entry-a".into(),
                session_id: SessionId("session-a".into()),
                sequence: 2,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::User),
                content: "hello".into(),
                created_at_unix_ms: 100,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "entry-c".into(),
                session_id: SessionId("session-a".into()),
                sequence: 1,
                kind: TranscriptEntryKind::Event,
                role: None,
                content: "created".into(),
                created_at_unix_ms: 99,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
        ]);

        assert_eq!(manifest.stats.total_entry_count, 3);
        assert_eq!(manifest.stats.session_count, 2);
        assert_eq!(manifest.entries.len(), 3);
        assert_eq!(manifest.entries[0].entry_id, "entry-c");
        assert_eq!(manifest.entries[0].session_id.0, "session-a");
        assert_eq!(manifest.entries[0].sequence, 1);
        assert_eq!(manifest.entries[0].content_bytes, "created".len());
        assert_eq!(manifest.entries[1].entry_id, "entry-a");
        assert_eq!(manifest.entries[2].entry_id, "entry-b");
        assert!(!manifest.is_empty());
    }

    #[test]
    fn transcript_snapshot_manifest_roundtrips_through_json() {
        let manifest = TranscriptSnapshotManifest::from_entries(&[sample_transcript_entry(
            1,
            "manifest payload",
        )]);

        let json = serde_json::to_string(&manifest).expect("manifest should serialize");
        let parsed: TranscriptSnapshotManifest =
            serde_json::from_str(&json).expect("manifest should deserialize");

        assert_eq!(parsed, manifest);
    }

    #[test]
    fn transcript_snapshot_manifest_deserializes_from_sparse_json() {
        let parsed: TranscriptSnapshotManifest =
            serde_json::from_str("{}").expect("sparse manifest should deserialize with defaults");

        assert_eq!(parsed, TranscriptSnapshotManifest::default());
        assert!(parsed.is_empty());
    }

    #[test]
    fn transcript_session_inventory_rolls_up_ranges_and_entry_kinds_per_session() {
        let inventory = TranscriptSessionInventory::from_entries(&[
            TranscriptEntry {
                entry_id: "entry-b".into(),
                session_id: SessionId("session-z".into()),
                sequence: 8,
                kind: TranscriptEntryKind::ToolResult,
                role: Some(MessageRole::Tool),
                content: "tool result payload".into(),
                created_at_unix_ms: 200,
                tool_name: Some("write".into()),
                correlation_id: None,
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "entry-a".into(),
                session_id: SessionId("session-a".into()),
                sequence: 3,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::User),
                content: "hello".into(),
                created_at_unix_ms: 100,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "entry-c".into(),
                session_id: SessionId("session-a".into()),
                sequence: 9,
                kind: TranscriptEntryKind::Summary,
                role: Some(MessageRole::Assistant),
                content: "summary".into(),
                created_at_unix_ms: 300,
                tool_name: None,
                correlation_id: None,
                summary_of_range: Some(TranscriptRange {
                    start_sequence: 1,
                    end_sequence: 8,
                }),
            },
            TranscriptEntry {
                entry_id: "entry-d".into(),
                session_id: SessionId("   ".into()),
                sequence: 1,
                kind: TranscriptEntryKind::Event,
                role: None,
                content: "missing session".into(),
                created_at_unix_ms: 400,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
        ]);

        assert_eq!(inventory.total_entry_count, 4);
        assert_eq!(inventory.blank_session_id_entry_count, 1);
        assert_eq!(inventory.session_count(), 2);
        assert_eq!(inventory.inventoried_entry_count(), 3);
        assert!(!inventory.is_empty());
        assert_eq!(inventory.sessions[0].session_id.0, "session-a");
        assert_eq!(inventory.sessions[0].entry_count, 2);
        assert_eq!(inventory.sessions[0].first_sequence, 3);
        assert_eq!(inventory.sessions[0].last_sequence, 9);
        assert_eq!(inventory.sessions[0].message_count, 1);
        assert_eq!(inventory.sessions[0].summary_count, 1);
        assert_eq!(inventory.sessions[1].session_id.0, "session-z");
        assert_eq!(inventory.sessions[1].entry_count, 1);
        assert_eq!(inventory.sessions[1].tool_result_count, 1);
    }

    #[test]
    fn transcript_session_inventory_roundtrips_through_json() {
        let inventory = TranscriptSessionInventory::from_entries(&[
            sample_transcript_entry(1, "first"),
            sample_transcript_entry(2, "second"),
        ]);

        let json = serde_json::to_string(&inventory).expect("inventory should serialize");
        let parsed: TranscriptSessionInventory =
            serde_json::from_str(&json).expect("inventory should deserialize");

        assert_eq!(parsed, inventory);
    }

    #[test]
    fn transcript_session_inventory_deserializes_from_sparse_json() {
        let parsed: TranscriptSessionInventory =
            serde_json::from_str("{}").expect("sparse inventory should deserialize with defaults");

        assert_eq!(parsed, TranscriptSessionInventory::default());
        assert_eq!(parsed.session_count(), 0);
        assert_eq!(parsed.inventoried_entry_count(), 0);
        assert!(parsed.is_empty());
    }

    #[test]
    fn transcript_snapshot_integrity_report_detects_duplicates_blank_and_collisions() {
        let report = TranscriptSnapshotIntegrityReport::from_entries(&[
            TranscriptEntry {
                entry_id: "entry-1".into(),
                session_id: SessionId("session-1".into()),
                sequence: 1,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::User),
                content: "hello".into(),
                created_at_unix_ms: 1,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "entry-1".into(),
                session_id: SessionId("session-1".into()),
                sequence: 1,
                kind: TranscriptEntryKind::ToolResult,
                role: Some(MessageRole::Tool),
                content: "result".into(),
                created_at_unix_ms: 2,
                tool_name: Some("write".into()),
                correlation_id: None,
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "   ".into(),
                session_id: SessionId("   ".into()),
                sequence: 2,
                kind: TranscriptEntryKind::Event,
                role: None,
                content: "   ".into(),
                created_at_unix_ms: 3,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
        ]);

        assert_eq!(report.duplicate_entry_ids, vec!["entry-1".to_string()]);
        assert_eq!(report.blank_entry_id_count, 1);
        assert_eq!(report.blank_session_id_count, 1);
        assert_eq!(report.blank_content_count, 1);
        assert_eq!(report.duplicate_sequence_collisions.len(), 1);
        assert_eq!(
            report.duplicate_sequence_collisions[0].session_id.0,
            "session-1"
        );
        assert_eq!(report.duplicate_sequence_collisions[0].sequence, 1);
        assert_eq!(
            report.duplicate_sequence_collisions[0].entry_ids,
            vec!["entry-1".to_string(), "entry-1".to_string()]
        );
        assert_eq!(report.issue_count(), 5);
        assert!(!report.is_clean());
    }

    #[test]
    fn clean_transcript_snapshot_integrity_report_has_no_issues() {
        let report = TranscriptSnapshotIntegrityReport::from_entries(&[
            sample_transcript_entry(1, "approval granted"),
            TranscriptEntry {
                entry_id: "entry-2".into(),
                session_id: SessionId("session-99".into()),
                sequence: 1,
                kind: TranscriptEntryKind::Summary,
                role: Some(MessageRole::Assistant),
                content: "clean summary".into(),
                created_at_unix_ms: 2,
                tool_name: None,
                correlation_id: Some("corr-2".into()),
                summary_of_range: Some(TranscriptRange {
                    start_sequence: 1,
                    end_sequence: 1,
                }),
            },
        ]);

        assert_eq!(report, TranscriptSnapshotIntegrityReport::default());
        assert_eq!(report.issue_count(), 0);
        assert!(report.is_clean());
    }

    #[test]
    fn memory_snapshot_stats_roll_up_sessions_and_memories() {
        let sessions = vec![
            SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Active foundation lane".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("stabilize memory contracts".into()),
                archived_at_unix_ms: None,
            },
            SessionRecord {
                session_id: SessionId("session-2".into()),
                agent_id: AgentId("builder".into()),
                title: "Archived foundation lane".into(),
                created_at_unix_ms: 3,
                last_active_unix_ms: 4,
                last_user_intent_summary: None,
                archived_at_unix_ms: Some(5),
            },
        ];
        let memories = vec![
            MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::Session,
                content: "doctor snapshot ready".into(),
            },
            MemoryRecord {
                id: "memory-2".into(),
                scope: MemoryScope::LongTerm,
                content: "memory contract exported".into(),
            },
        ];

        let stats = MemorySnapshotStats::from_records(&sessions, &memories);

        assert_eq!(stats.session_count, 2);
        assert_eq!(stats.active_session_count, 1);
        assert_eq!(stats.archived_session_count, 1);
        assert_eq!(stats.total_memory_count, 2);
        assert_eq!(stats.session_memory_count, 1);
        assert_eq!(stats.long_term_memory_count, 1);
        assert!(!stats.is_empty());
    }

    #[test]
    fn empty_memory_snapshot_stats_report_empty_state() {
        let stats = MemorySnapshotStats::from_records(&[], &[]);

        assert_eq!(stats, MemorySnapshotStats::default());
        assert!(stats.is_empty());
    }

    #[test]
    fn session_agent_inventory_rolls_up_sessions_by_agent() {
        let inventory = SessionAgentInventory::from_records(&[
            SessionRecord {
                session_id: SessionId("session-2".into()),
                agent_id: AgentId("reviewer".into()),
                title: "Reviewer lane".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 40,
                last_user_intent_summary: None,
                archived_at_unix_ms: Some(50),
            },
            SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Builder lane".into(),
                created_at_unix_ms: 2,
                last_active_unix_ms: 10,
                last_user_intent_summary: Some("stabilize contracts".into()),
                archived_at_unix_ms: None,
            },
            SessionRecord {
                session_id: SessionId("session-3".into()),
                agent_id: AgentId("builder".into()),
                title: "Builder archive".into(),
                created_at_unix_ms: 3,
                last_active_unix_ms: 25,
                last_user_intent_summary: None,
                archived_at_unix_ms: Some(30),
            },
            SessionRecord {
                session_id: SessionId("session-4".into()),
                agent_id: AgentId("   ".into()),
                title: "Blank agent".into(),
                created_at_unix_ms: 4,
                last_active_unix_ms: 60,
                last_user_intent_summary: None,
                archived_at_unix_ms: None,
            },
        ]);

        assert_eq!(inventory.total_session_count, 4);
        assert_eq!(inventory.blank_agent_id_session_count, 1);
        assert_eq!(inventory.agent_count(), 2);
        assert_eq!(inventory.inventoried_session_count(), 3);
        assert!(!inventory.is_empty());
        assert_eq!(inventory.agents[0].agent_id.0, "builder");
        assert_eq!(inventory.agents[0].session_count, 2);
        assert_eq!(inventory.agents[0].active_session_count, 1);
        assert_eq!(inventory.agents[0].archived_session_count, 1);
        assert_eq!(inventory.agents[0].latest_activity_unix_ms, 25);
        assert_eq!(inventory.agents[1].agent_id.0, "reviewer");
        assert_eq!(inventory.agents[1].session_count, 1);
        assert_eq!(inventory.agents[1].active_session_count, 0);
        assert_eq!(inventory.agents[1].archived_session_count, 1);
        assert_eq!(inventory.agents[1].latest_activity_unix_ms, 40);
    }

    #[test]
    fn session_agent_inventory_roundtrips_through_json() {
        let inventory = SessionAgentInventory::from_records(&[
            SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Foundation lane".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("inventory roundtrip".into()),
                archived_at_unix_ms: None,
            },
            SessionRecord {
                session_id: SessionId("session-2".into()),
                agent_id: AgentId("reviewer".into()),
                title: "Review lane".into(),
                created_at_unix_ms: 3,
                last_active_unix_ms: 4,
                last_user_intent_summary: None,
                archived_at_unix_ms: Some(5),
            },
        ]);

        let json = serde_json::to_string(&inventory).expect("inventory should serialize");
        let parsed: SessionAgentInventory =
            serde_json::from_str(&json).expect("inventory should deserialize");

        assert_eq!(parsed, inventory);
    }

    #[test]
    fn session_agent_inventory_deserializes_from_sparse_json() {
        let parsed: SessionAgentInventory =
            serde_json::from_str("{}").expect("sparse inventory should deserialize with defaults");

        assert_eq!(parsed, SessionAgentInventory::default());
        assert_eq!(parsed.agent_count(), 0);
        assert_eq!(parsed.inventoried_session_count(), 0);
        assert!(parsed.is_empty());
    }

    #[test]
    fn memory_snapshot_manifest_sorts_records_and_tracks_content_sizes() {
        let manifest = MemorySnapshotManifest::from_records(
            &[
                SessionRecord {
                    session_id: SessionId("session-b".into()),
                    agent_id: AgentId("builder".into()),
                    title: "Later session".into(),
                    created_at_unix_ms: 1,
                    last_active_unix_ms: 2,
                    last_user_intent_summary: None,
                    archived_at_unix_ms: Some(5),
                },
                SessionRecord {
                    session_id: SessionId("session-a".into()),
                    agent_id: AgentId("builder".into()),
                    title: "Earlier session".into(),
                    created_at_unix_ms: 3,
                    last_active_unix_ms: 4,
                    last_user_intent_summary: Some("audit manifest".into()),
                    archived_at_unix_ms: None,
                },
            ],
            &[
                MemoryRecord {
                    id: "memory-z".into(),
                    scope: MemoryScope::LongTerm,
                    content: "doctor export manifest".into(),
                },
                MemoryRecord {
                    id: "memory-a".into(),
                    scope: MemoryScope::Session,
                    content: "snapshot ok".into(),
                },
            ],
        );

        assert_eq!(manifest.stats.session_count, 2);
        assert_eq!(manifest.stats.archived_session_count, 1);
        assert_eq!(manifest.sessions.len(), 2);
        assert_eq!(manifest.sessions[0].session_id.0, "session-a");
        assert!(!manifest.sessions[0].archived);
        assert_eq!(manifest.sessions[1].session_id.0, "session-b");
        assert!(manifest.sessions[1].archived);
        assert_eq!(manifest.memories.len(), 2);
        assert_eq!(manifest.memories[0].id, "memory-a");
        assert_eq!(manifest.memories[0].content_bytes, "snapshot ok".len());
        assert_eq!(manifest.memories[1].id, "memory-z");
        assert_eq!(
            manifest.memories[1].content_bytes,
            "doctor export manifest".len()
        );
        assert!(!manifest.is_empty());
    }

    #[test]
    fn memory_snapshot_manifest_roundtrips_through_json() {
        let manifest = MemorySnapshotManifest::from_records(
            &[SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Foundation lane".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("inspect snapshot manifest".into()),
                archived_at_unix_ms: None,
            }],
            &[MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "export manifest ready".into(),
            }],
        );

        let json = serde_json::to_string(&manifest).expect("manifest should serialize");
        let parsed: MemorySnapshotManifest =
            serde_json::from_str(&json).expect("manifest should deserialize");

        assert_eq!(parsed, manifest);
    }

    #[test]
    fn memory_snapshot_manifest_deserializes_from_sparse_json() {
        let parsed: MemorySnapshotManifest =
            serde_json::from_str("{}").expect("sparse manifest should deserialize with defaults");

        assert_eq!(parsed, MemorySnapshotManifest::default());
        assert!(parsed.is_empty());
    }

    #[test]
    fn memory_snapshot_integrity_report_detects_duplicate_and_blank_fields() {
        let report = MemorySnapshotIntegrityReport::from_records(
            &[
                SessionRecord {
                    session_id: SessionId("session-1".into()),
                    agent_id: AgentId("builder".into()),
                    title: "Foundation".into(),
                    created_at_unix_ms: 1,
                    last_active_unix_ms: 2,
                    last_user_intent_summary: Some("audit snapshot".into()),
                    archived_at_unix_ms: None,
                },
                SessionRecord {
                    session_id: SessionId("session-1".into()),
                    agent_id: AgentId("builder".into()),
                    title: "   ".into(),
                    created_at_unix_ms: 3,
                    last_active_unix_ms: 4,
                    last_user_intent_summary: None,
                    archived_at_unix_ms: None,
                },
                SessionRecord {
                    session_id: SessionId("   ".into()),
                    agent_id: AgentId("builder".into()),
                    title: "Needs title".into(),
                    created_at_unix_ms: 5,
                    last_active_unix_ms: 6,
                    last_user_intent_summary: None,
                    archived_at_unix_ms: None,
                },
            ],
            &[
                MemoryRecord {
                    id: "memory-1".into(),
                    scope: MemoryScope::Session,
                    content: "contract ready".into(),
                },
                MemoryRecord {
                    id: "memory-1".into(),
                    scope: MemoryScope::LongTerm,
                    content: "   ".into(),
                },
                MemoryRecord {
                    id: " ".into(),
                    scope: MemoryScope::LongTerm,
                    content: "manifest payload".into(),
                },
            ],
        );

        assert_eq!(
            report.duplicate_session_ids,
            vec![SessionId("session-1".into())]
        );
        assert_eq!(report.duplicate_memory_ids, vec!["memory-1".to_string()]);
        assert_eq!(report.blank_session_id_count, 1);
        assert_eq!(report.blank_memory_id_count, 1);
        assert_eq!(report.blank_session_title_count, 1);
        assert_eq!(report.blank_memory_content_count, 1);
        assert_eq!(report.issue_count(), 6);
        assert!(!report.is_clean());
    }

    #[test]
    fn clean_memory_snapshot_integrity_report_has_no_issues() {
        let report = MemorySnapshotIntegrityReport::from_records(
            &[SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Foundation lane".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("stabilize contracts".into()),
                archived_at_unix_ms: None,
            }],
            &[MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "snapshot contract ready".into(),
            }],
        );

        assert_eq!(report, MemorySnapshotIntegrityReport::default());
        assert_eq!(report.issue_count(), 0);
        assert!(report.is_clean());
    }

    #[test]
    fn memory_snapshot_integrity_report_roundtrips_through_json() {
        let report = MemorySnapshotIntegrityReport {
            duplicate_session_ids: vec![SessionId("session-1".into())],
            duplicate_memory_ids: vec!["memory-1".into()],
            blank_session_id_count: 1,
            blank_memory_id_count: 0,
            blank_session_title_count: 2,
            blank_memory_content_count: 3,
        };

        let json = serde_json::to_string(&report).expect("integrity report should serialize");
        let parsed: MemorySnapshotIntegrityReport =
            serde_json::from_str(&json).expect("integrity report should deserialize");

        assert_eq!(parsed, report);
    }

    #[test]
    fn snapshot_audit_report_rolls_up_memory_and_transcript_health() {
        let report = SnapshotAuditReport::from_records_and_entries(
            &[
                SessionRecord {
                    session_id: SessionId("session-1".into()),
                    agent_id: AgentId("builder".into()),
                    title: "Foundation".into(),
                    created_at_unix_ms: 1,
                    last_active_unix_ms: 2,
                    last_user_intent_summary: Some("audit combined snapshot".into()),
                    archived_at_unix_ms: None,
                },
                SessionRecord {
                    session_id: SessionId("session-1".into()),
                    agent_id: AgentId("builder".into()),
                    title: "   ".into(),
                    created_at_unix_ms: 3,
                    last_active_unix_ms: 4,
                    last_user_intent_summary: None,
                    archived_at_unix_ms: None,
                },
            ],
            &[
                MemoryRecord {
                    id: "memory-1".into(),
                    scope: MemoryScope::LongTerm,
                    content: "manifest payload".into(),
                },
                MemoryRecord {
                    id: "memory-1".into(),
                    scope: MemoryScope::Session,
                    content: "   ".into(),
                },
            ],
            &[
                TranscriptEntry {
                    entry_id: "entry-1".into(),
                    session_id: SessionId("session-1".into()),
                    sequence: 1,
                    kind: TranscriptEntryKind::Message,
                    role: Some(MessageRole::User),
                    content: "hello".into(),
                    created_at_unix_ms: 1,
                    tool_name: None,
                    correlation_id: None,
                    summary_of_range: None,
                },
                TranscriptEntry {
                    entry_id: "entry-1".into(),
                    session_id: SessionId("session-1".into()),
                    sequence: 1,
                    kind: TranscriptEntryKind::ToolResult,
                    role: Some(MessageRole::Tool),
                    content: "result".into(),
                    created_at_unix_ms: 2,
                    tool_name: Some("write".into()),
                    correlation_id: None,
                    summary_of_range: None,
                },
            ],
        );

        assert_eq!(report.memory_stats.session_count, 2);
        assert_eq!(report.memory_stats.total_memory_count, 2);
        assert_eq!(report.transcript_stats.total_entry_count, 2);
        assert_eq!(report.memory_integrity.issue_count(), 4);
        assert_eq!(report.transcript_integrity.issue_count(), 2);
        assert_eq!(report.memory_issue_count(), 4);
        assert_eq!(report.transcript_issue_count(), 2);
        assert_eq!(report.issue_count(), 6);
        assert_eq!(report.issue_domain_count(), 2);
        assert!(report.touches_memory());
        assert!(report.touches_transcripts());
        assert!(!report.is_clean());
        assert!(!report.is_empty());
    }

    #[test]
    fn snapshot_audit_report_roundtrips_through_json() {
        let report = SnapshotAuditReport::from_records_and_entries(
            &[SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Foundation lane".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("combined audit".into()),
                archived_at_unix_ms: None,
            }],
            &[MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "snapshot contract ready".into(),
            }],
            &[sample_transcript_entry(1, "snapshot captured")],
        );

        let json = serde_json::to_string(&report).expect("audit report should serialize");
        let parsed: SnapshotAuditReport =
            serde_json::from_str(&json).expect("audit report should deserialize");

        assert_eq!(parsed, report);
        assert!(parsed.is_clean());
    }

    #[test]
    fn snapshot_audit_report_deserializes_from_sparse_json() {
        let parsed: SnapshotAuditReport = serde_json::from_str("{}")
            .expect("sparse audit report should deserialize with defaults");

        assert_eq!(parsed, SnapshotAuditReport::default());
        assert!(parsed.is_empty());
        assert!(parsed.is_clean());
    }

    #[test]
    fn snapshot_issue_summary_compacts_audit_issue_counts() {
        let report = SnapshotAuditReport::from_records_and_entries(
            &[SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: " ".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("compact audit issue summary".into()),
                archived_at_unix_ms: None,
            }],
            &[MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: " ".into(),
            }],
            &[TranscriptEntry {
                entry_id: "entry-1".into(),
                session_id: SessionId("session-1".into()),
                sequence: 1,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::Assistant),
                content: " ".into(),
                created_at_unix_ms: 3,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            }],
        );

        let summary = report.issue_summary();

        assert_eq!(summary.memory_issue_count, 2);
        assert_eq!(summary.transcript_issue_count, 1);
        assert_eq!(summary.total_issue_count, 3);
        assert_eq!(summary.issue_domain_count, 2);
        assert!(summary.touches_memory());
        assert!(summary.touches_transcripts());
        assert!(summary.has_issues());
        assert!(!summary.is_clean());
    }

    #[test]
    fn snapshot_issue_summary_matches_bundle_and_audit_views() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("issue summary alignment".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "payload".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];

        let report =
            SnapshotAuditReport::from_records_and_entries(&sessions, &memories, &transcripts);
        let bundle =
            SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);

        assert_eq!(
            report.issue_summary(),
            SnapshotIssueSummary::from_audit_report(&report)
        );
        assert_eq!(
            bundle.issue_summary(),
            SnapshotIssueSummary::from_inspection(&bundle)
        );
        assert_eq!(report.issue_summary(), bundle.issue_summary());
        assert!(report.issue_summary().is_clean());
    }

    #[test]
    fn snapshot_inspection_bundle_drift_impact_matches_report_helper() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("drift impact alignment".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "snapshot payload".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
        let canonical =
            SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);
        let drifted = SnapshotInspectionBundle {
            memory_manifest: MemorySnapshotManifest::default(),
            ..canonical.clone()
        };

        assert_eq!(
            canonical.drift_impact_against_records(&sessions, &memories, &transcripts),
            canonical
                .drift_report(&sessions, &memories, &transcripts)
                .impact()
        );
        assert!(
            canonical
                .drift_impact_against_records(&sessions, &memories, &transcripts)
                .is_aligned()
        );

        let impact = drifted.drift_impact_against_records(&sessions, &memories, &transcripts);

        assert_eq!(
            impact,
            drifted
                .drift_report(&sessions, &memories, &transcripts)
                .impact()
        );
        assert_eq!(impact.mismatch_count, 1);
        assert_eq!(impact.memory_mismatch_count, 1);
        assert_eq!(impact.transcript_mismatch_count, 0);
        assert_eq!(impact.changed_domain_count(), 1);
        assert!(impact.touches_memory());
        assert!(!impact.touches_transcripts());
        assert!(!impact.is_aligned());
    }

    #[test]
    fn snapshot_issue_summary_roundtrips_through_json() {
        let summary = SnapshotIssueSummary {
            memory_issue_count: 2,
            transcript_issue_count: 1,
            total_issue_count: 3,
            issue_domain_count: 2,
        };

        let json = serde_json::to_string(&summary).expect("issue summary should serialize");
        let parsed: SnapshotIssueSummary =
            serde_json::from_str(&json).expect("issue summary should deserialize");

        assert_eq!(parsed, summary);
        assert!(parsed.has_issues());
        assert!(!parsed.is_clean());
    }

    #[test]
    fn snapshot_issue_summary_deserializes_from_sparse_json() {
        let parsed: SnapshotIssueSummary = serde_json::from_str("{}")
            .expect("sparse issue summary should deserialize with defaults");

        assert_eq!(parsed, SnapshotIssueSummary::default());
        assert!(!parsed.has_issues());
        assert!(parsed.is_clean());
    }

    #[test]
    fn snapshot_inspection_bundle_keeps_manifests_and_integrity_reports_aligned() {
        let bundle = SnapshotInspectionBundle::from_records_and_entries(
            &[SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Foundation lane".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("inspect snapshot".into()),
                archived_at_unix_ms: None,
            }],
            &[MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::Session,
                content: "manifest payload".into(),
            }],
            &[sample_transcript_entry(1, "snapshot captured")],
        );

        assert_eq!(bundle.memory_manifest.stats.session_count, 1);
        assert_eq!(bundle.memory_manifest.stats.total_memory_count, 1);
        assert_eq!(bundle.transcript_manifest.stats.total_entry_count, 1);
        assert_eq!(
            bundle.memory_integrity,
            MemorySnapshotIntegrityReport::default()
        );
        assert_eq!(
            bundle.transcript_integrity,
            TranscriptSnapshotIntegrityReport::default()
        );
        assert_eq!(bundle.memory_issue_count(), 0);
        assert_eq!(bundle.transcript_issue_count(), 0);
        assert_eq!(bundle.issue_count(), 0);
        assert_eq!(bundle.issue_domain_count(), 0);
        assert_eq!(bundle.issue_summary(), SnapshotIssueSummary::default());
        assert!(!bundle.touches_memory());
        assert!(!bundle.touches_transcripts());
        assert!(bundle.is_clean());
        assert!(!bundle.is_empty());
    }

    #[test]
    fn snapshot_inspection_bundle_roundtrips_through_json() {
        let bundle = SnapshotInspectionBundle::from_records_and_entries(
            &[SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Foundation lane".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("inspect snapshot".into()),
                archived_at_unix_ms: Some(3),
            }],
            &[MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "snapshot contract ready".into(),
            }],
            &[sample_transcript_entry(1, "snapshot captured")],
        );

        let json = serde_json::to_string(&bundle).expect("inspection bundle should serialize");
        let parsed: SnapshotInspectionBundle =
            serde_json::from_str(&json).expect("inspection bundle should deserialize");

        assert_eq!(parsed, bundle);
        assert!(parsed.is_clean());
    }

    #[test]
    fn snapshot_inspection_bundle_deserializes_from_sparse_json() {
        let parsed: SnapshotInspectionBundle = serde_json::from_str("{}")
            .expect("sparse inspection bundle should deserialize with defaults");

        assert_eq!(parsed, SnapshotInspectionBundle::default());
        assert!(parsed.is_empty());
        assert!(parsed.is_clean());
    }

    #[test]
    fn snapshot_inspection_bundle_reconstructs_audit_report() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("reconstruct audit report".into()),
            archived_at_unix_ms: Some(3),
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "snapshot contract ready".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];

        let bundle =
            SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);
        let report = bundle.audit_report();

        assert_eq!(
            report,
            SnapshotAuditReport::from_records_and_entries(&sessions, &memories, &transcripts)
        );
        assert_eq!(bundle.memory_issue_count(), report.memory_issue_count());
        assert_eq!(
            bundle.transcript_issue_count(),
            report.transcript_issue_count()
        );
        assert_eq!(bundle.issue_summary(), report.issue_summary());
        assert_eq!(bundle.issue_domain_count(), report.issue_domain_count());
        assert_eq!(bundle.touches_memory(), report.touches_memory());
        assert_eq!(bundle.touches_transcripts(), report.touches_transcripts());
        assert!(report.is_clean());
    }

    #[test]
    fn snapshot_inspection_bundle_matches_records_and_entries_only_when_aligned() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("match inspection bundle".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "inspection payload".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
        let bundle =
            SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);

        assert!(bundle.matches_records_and_entries(&sessions, &memories, &transcripts));

        let drifted_transcripts = vec![sample_transcript_entry(2, "snapshot changed")];

        assert!(!bundle.matches_records_and_entries(&sessions, &memories, &drifted_transcripts,));
    }

    #[test]
    fn snapshot_inspection_drift_report_is_empty_for_aligned_bundle() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("aligned inspection bundle".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "inspection payload".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
        let bundle =
            SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);

        let drift = bundle.drift_report(&sessions, &memories, &transcripts);

        assert_eq!(drift, SnapshotInspectionDriftReport::default());
        assert_eq!(drift.mismatch_count(), 0);
        assert!(drift.is_aligned());
        assert!(!drift.mismatches(SnapshotInspectionSection::MemoryManifest));
    }

    #[test]
    fn snapshot_inspection_drift_report_identifies_mismatched_sections() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("drift inspection bundle".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "snapshot payload".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
        let drifted = SnapshotInspectionBundle {
            memory_manifest: MemorySnapshotManifest::default(),
            memory_integrity: MemorySnapshotIntegrityReport::default(),
            transcript_manifest: TranscriptSnapshotManifest::default(),
            transcript_integrity: TranscriptSnapshotIntegrityReport::default(),
        };

        let drift = drifted.drift_report(&sessions, &memories, &transcripts);

        assert_eq!(
            drift.mismatched_sections,
            vec![
                SnapshotInspectionSection::MemoryManifest,
                SnapshotInspectionSection::TranscriptManifest,
            ]
        );
        assert_eq!(drift.mismatch_count(), 2);
        assert!(!drift.is_aligned());
        assert!(drift.mismatches(SnapshotInspectionSection::MemoryManifest));
        assert!(!drift.mismatches(SnapshotInspectionSection::MemoryIntegrity));
        assert!(drift.mismatches(SnapshotInspectionSection::TranscriptManifest));
        assert!(!drift.mismatches(SnapshotInspectionSection::TranscriptIntegrity));
    }

    #[test]
    fn snapshot_inspection_drift_report_roundtrips_through_json() {
        let report = SnapshotInspectionDriftReport {
            mismatched_sections: vec![
                SnapshotInspectionSection::MemoryIntegrity,
                SnapshotInspectionSection::TranscriptIntegrity,
            ],
        };

        let json = serde_json::to_string(&report).expect("drift report should serialize");
        let parsed: SnapshotInspectionDriftReport =
            serde_json::from_str(&json).expect("drift report should deserialize");

        assert_eq!(parsed, report);
        assert_eq!(parsed.mismatch_count(), 2);
        assert!(!parsed.is_aligned());
    }

    #[test]
    fn snapshot_inspection_drift_report_exposes_domain_level_counts_and_impact() {
        let report = SnapshotInspectionDriftReport {
            mismatched_sections: vec![
                SnapshotInspectionSection::MemoryManifest,
                SnapshotInspectionSection::MemoryIntegrity,
                SnapshotInspectionSection::TranscriptManifest,
            ],
        };

        assert_eq!(report.memory_mismatch_count(), 2);
        assert_eq!(report.transcript_mismatch_count(), 1);
        assert_eq!(report.changed_domain_count(), 2);
        assert!(report.touches_memory());
        assert!(report.touches_transcripts());

        assert_eq!(
            report.impact(),
            SnapshotInspectionDriftImpact {
                mismatch_count: 3,
                memory_mismatch_count: 2,
                transcript_mismatch_count: 1,
            }
        );
    }

    #[test]
    fn snapshot_inspection_drift_impact_roundtrips_through_json() {
        let impact = SnapshotInspectionDriftImpact {
            mismatch_count: 2,
            memory_mismatch_count: 0,
            transcript_mismatch_count: 2,
        };

        let json = serde_json::to_string(&impact).expect("drift impact should serialize");
        let parsed: SnapshotInspectionDriftImpact =
            serde_json::from_str(&json).expect("drift impact should deserialize");

        assert_eq!(parsed, impact);
        assert_eq!(parsed.changed_domain_count(), 1);
        assert!(!parsed.touches_memory());
        assert!(parsed.touches_transcripts());
        assert!(!parsed.is_aligned());
    }

    #[test]
    fn snapshot_inspection_drift_impact_deserializes_from_sparse_json() {
        let parsed: SnapshotInspectionDriftImpact = serde_json::from_str("{}")
            .expect("sparse drift impact should deserialize with defaults");

        assert_eq!(parsed, SnapshotInspectionDriftImpact::default());
        assert_eq!(parsed.changed_domain_count(), 0);
        assert!(!parsed.touches_memory());
        assert!(!parsed.touches_transcripts());
        assert!(parsed.is_aligned());
    }

    #[test]
    fn snapshot_inspection_drift_report_deserializes_from_sparse_json() {
        let parsed: SnapshotInspectionDriftReport = serde_json::from_str("{}")
            .expect("sparse drift report should deserialize with defaults");

        assert_eq!(parsed, SnapshotInspectionDriftReport::default());
        assert_eq!(parsed.mismatch_count(), 0);
        assert!(parsed.is_aligned());
    }

    #[test]
    fn snapshot_inspection_health_is_ready_when_bundle_is_clean_and_aligned() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("inspection health ready".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "inspection payload".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
        let bundle =
            SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);

        let health = bundle.health_against_records(&sessions, &memories, &transcripts);

        assert_eq!(health.issue_summary, SnapshotIssueSummary::default());
        assert_eq!(
            health.drift_impact,
            SnapshotInspectionDriftImpact::default()
        );
        assert_eq!(health.issue_count(), 0);
        assert_eq!(health.mismatch_count(), 0);
        assert_eq!(health.changed_domain_count(), 0);
        assert!(!health.touches_memory());
        assert!(!health.touches_transcripts());
        assert!(!health.has_issues());
        assert!(!health.has_drift());
        assert!(health.inspection_aligned());
        assert!(health.is_clean());
        assert!(health.is_ready());
    }

    #[test]
    fn snapshot_inspection_health_tracks_issue_domains_even_without_drift() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("inspection health issues".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: " ".into(),
        }];
        let transcripts = vec![TranscriptEntry {
            entry_id: "entry-1".into(),
            session_id: SessionId("session-1".into()),
            sequence: 1,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::Assistant),
            content: " ".into(),
            created_at_unix_ms: 3,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        }];
        let bundle =
            SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);

        let health = bundle.health_against_records(&sessions, &memories, &transcripts);

        assert_eq!(health.issue_count(), 2);
        assert_eq!(health.mismatch_count(), 0);
        assert_eq!(health.changed_domain_count(), 2);
        assert!(health.touches_memory());
        assert!(health.touches_transcripts());
        assert!(health.has_issues());
        assert!(!health.has_drift());
        assert!(health.inspection_aligned());
        assert!(!health.is_clean());
        assert!(!health.is_ready());
    }

    #[test]
    fn snapshot_inspection_health_tracks_drift_domains_even_without_issues() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("inspection health drift".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "snapshot payload".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];

        let health = SnapshotInspectionBundle::default().health_against_records(
            &sessions,
            &memories,
            &transcripts,
        );

        assert_eq!(health.issue_count(), 0);
        assert_eq!(health.mismatch_count(), 2);
        assert_eq!(health.changed_domain_count(), 2);
        assert!(health.touches_memory());
        assert!(health.touches_transcripts());
        assert!(!health.has_issues());
        assert!(health.has_drift());
        assert!(!health.inspection_aligned());
        assert!(health.is_clean());
        assert!(!health.is_ready());
    }

    #[test]
    fn snapshot_inspection_health_roundtrips_through_json() {
        let health = SnapshotInspectionHealth {
            issue_summary: SnapshotIssueSummary {
                memory_issue_count: 1,
                transcript_issue_count: 0,
                total_issue_count: 1,
                issue_domain_count: 1,
            },
            drift_impact: SnapshotInspectionDriftImpact {
                mismatch_count: 2,
                memory_mismatch_count: 1,
                transcript_mismatch_count: 1,
            },
        };

        let json = serde_json::to_string(&health).expect("inspection health should serialize");
        let parsed: SnapshotInspectionHealth =
            serde_json::from_str(&json).expect("inspection health should deserialize");

        assert_eq!(parsed, health);
        assert_eq!(parsed.issue_count(), 1);
        assert_eq!(parsed.mismatch_count(), 2);
        assert_eq!(parsed.changed_domain_count(), 2);
        assert!(parsed.touches_memory());
        assert!(parsed.touches_transcripts());
        assert!(parsed.has_issues());
        assert!(parsed.has_drift());
        assert!(!parsed.inspection_aligned());
        assert!(!parsed.is_clean());
        assert!(!parsed.is_ready());
    }

    #[test]
    fn snapshot_restore_preview_classifies_added_removed_updated_and_unchanged_records() {
        let current_sessions = vec![
            SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Current title".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("current".into()),
                archived_at_unix_ms: None,
            },
            SessionRecord {
                session_id: SessionId("session-2".into()),
                agent_id: AgentId("builder".into()),
                title: "Unchanged session".into(),
                created_at_unix_ms: 3,
                last_active_unix_ms: 4,
                last_user_intent_summary: None,
                archived_at_unix_ms: None,
            },
        ];
        let incoming_sessions = vec![
            SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Updated title".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 20,
                last_user_intent_summary: Some("incoming".into()),
                archived_at_unix_ms: None,
            },
            SessionRecord {
                session_id: SessionId("session-2".into()),
                agent_id: AgentId("builder".into()),
                title: "Unchanged session".into(),
                created_at_unix_ms: 3,
                last_active_unix_ms: 4,
                last_user_intent_summary: None,
                archived_at_unix_ms: None,
            },
            SessionRecord {
                session_id: SessionId("session-3".into()),
                agent_id: AgentId("builder".into()),
                title: "Added session".into(),
                created_at_unix_ms: 5,
                last_active_unix_ms: 6,
                last_user_intent_summary: Some("added".into()),
                archived_at_unix_ms: Some(7),
            },
        ];
        let current_memories = vec![
            MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::Session,
                content: "unchanged".into(),
            },
            MemoryRecord {
                id: "memory-2".into(),
                scope: MemoryScope::LongTerm,
                content: "removed".into(),
            },
            MemoryRecord {
                id: "memory-3".into(),
                scope: MemoryScope::LongTerm,
                content: "before update".into(),
            },
        ];
        let incoming_memories = vec![
            MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::Session,
                content: "unchanged".into(),
            },
            MemoryRecord {
                id: "memory-3".into(),
                scope: MemoryScope::LongTerm,
                content: "after update".into(),
            },
            MemoryRecord {
                id: "memory-4".into(),
                scope: MemoryScope::Session,
                content: "added".into(),
            },
        ];
        let current_transcripts = vec![
            sample_transcript_entry(1, "unchanged transcript"),
            sample_transcript_entry(2, "removed transcript"),
            sample_transcript_entry(3, "before update"),
        ];
        let incoming_transcripts = vec![
            sample_transcript_entry(1, "unchanged transcript"),
            TranscriptEntry {
                content: "after update".into(),
                ..sample_transcript_entry(3, "before update")
            },
            sample_transcript_entry(4, "added transcript"),
        ];

        let preview = SnapshotRestorePreview::from_records_and_entries(
            &current_sessions,
            &current_memories,
            &current_transcripts,
            &incoming_sessions,
            &incoming_memories,
            &incoming_transcripts,
        );

        assert_eq!(
            preview.session_delta.added_session_ids,
            vec![SessionId("session-3".into())]
        );
        assert!(preview.session_delta.removed_session_ids.is_empty());
        assert_eq!(
            preview.session_delta.updated_session_ids,
            vec![SessionId("session-1".into())]
        );
        assert_eq!(preview.session_delta.unchanged_count, 1);

        assert_eq!(
            preview.memory_delta.added_memory_ids,
            vec!["memory-4".to_string()]
        );
        assert_eq!(
            preview.memory_delta.removed_memory_ids,
            vec!["memory-2".to_string()]
        );
        assert_eq!(
            preview.memory_delta.updated_memory_ids,
            vec!["memory-3".to_string()]
        );
        assert_eq!(preview.memory_delta.unchanged_count, 1);

        assert_eq!(
            preview.transcript_delta.added_entry_ids,
            vec!["entry-4".to_string()]
        );
        assert_eq!(
            preview.transcript_delta.removed_entry_ids,
            vec!["entry-2".to_string()]
        );
        assert_eq!(
            preview.transcript_delta.updated_entry_ids,
            vec!["entry-3".to_string()]
        );
        assert_eq!(preview.transcript_delta.unchanged_count, 1);

        assert_eq!(preview.change_count(), 8);
        assert!(!preview.is_noop());
        assert!(!preview.has_integrity_issues());
    }

    #[test]
    fn snapshot_restore_preview_detects_noop_restore() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("noop restore".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "same payload".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "same transcript")];

        let preview = SnapshotRestorePreview::from_records_and_entries(
            &sessions,
            &memories,
            &transcripts,
            &sessions,
            &memories,
            &transcripts,
        );

        assert_eq!(preview.change_count(), 0);
        assert_eq!(
            preview.change_totals(),
            RestoreDeltaCounts {
                added_count: 0,
                removed_count: 0,
                updated_count: 0,
                unchanged_count: 3,
            }
        );
        assert!(preview.is_noop());
        assert!(!preview.has_integrity_issues());
    }

    #[test]
    fn restore_delta_counts_roundtrip_through_json() {
        let counts = RestoreDeltaCounts {
            added_count: 2,
            removed_count: 1,
            updated_count: 3,
            unchanged_count: 4,
        };

        let json = serde_json::to_string(&counts).expect("restore delta counts should serialize");
        let parsed: RestoreDeltaCounts =
            serde_json::from_str(&json).expect("restore delta counts should deserialize");

        assert_eq!(parsed, counts);
        assert_eq!(parsed.change_count(), 6);
        assert!(!parsed.is_empty());
    }

    #[test]
    fn restore_delta_counts_deserialize_from_sparse_json() {
        let parsed: RestoreDeltaCounts = serde_json::from_str("{}")
            .expect("sparse restore delta counts should deserialize with defaults");

        assert_eq!(parsed, RestoreDeltaCounts::default());
        assert_eq!(parsed.change_count(), 0);
        assert!(parsed.is_empty());
    }

    #[test]
    fn snapshot_restore_preview_change_totals_roll_up_each_domain() {
        let preview = SnapshotRestorePreview {
            current_audit: SnapshotAuditReport::default(),
            incoming_audit: SnapshotAuditReport::default(),
            session_delta: SessionRestoreDelta {
                added_session_ids: vec![SessionId("session-2".into())],
                removed_session_ids: vec![SessionId("session-3".into())],
                updated_session_ids: vec![SessionId("session-1".into())],
                unchanged_count: 4,
            },
            memory_delta: MemoryRestoreDelta {
                added_memory_ids: vec!["memory-2".into()],
                removed_memory_ids: vec![],
                updated_memory_ids: vec!["memory-1".into()],
                unchanged_count: 5,
            },
            transcript_delta: TranscriptRestoreDelta {
                added_entry_ids: vec!["entry-3".into()],
                removed_entry_ids: vec!["entry-4".into()],
                updated_entry_ids: vec![],
                unchanged_count: 6,
            },
        };

        assert_eq!(
            preview.session_delta.counts(),
            RestoreDeltaCounts {
                added_count: 1,
                removed_count: 1,
                updated_count: 1,
                unchanged_count: 4,
            }
        );
        assert_eq!(
            preview.memory_delta.counts(),
            RestoreDeltaCounts {
                added_count: 1,
                removed_count: 0,
                updated_count: 1,
                unchanged_count: 5,
            }
        );
        assert_eq!(
            preview.transcript_delta.counts(),
            RestoreDeltaCounts {
                added_count: 1,
                removed_count: 1,
                updated_count: 0,
                unchanged_count: 6,
            }
        );
        assert_eq!(
            preview.change_totals(),
            RestoreDeltaCounts {
                added_count: 3,
                removed_count: 2,
                updated_count: 2,
                unchanged_count: 15,
            }
        );
        assert_eq!(
            preview.change_totals().change_count(),
            preview.change_count()
        );
        assert_eq!(
            preview.domain_impacts(),
            vec![
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Sessions,
                    counts: RestoreDeltaCounts {
                        added_count: 1,
                        removed_count: 1,
                        updated_count: 1,
                        unchanged_count: 4,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Memories,
                    counts: RestoreDeltaCounts {
                        added_count: 1,
                        removed_count: 0,
                        updated_count: 1,
                        unchanged_count: 5,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Transcripts,
                    counts: RestoreDeltaCounts {
                        added_count: 1,
                        removed_count: 1,
                        updated_count: 0,
                        unchanged_count: 6,
                    },
                },
            ]
        );
        assert_eq!(
            preview.changed_domains(),
            vec![
                SnapshotRestoreDomain::Sessions,
                SnapshotRestoreDomain::Memories,
                SnapshotRestoreDomain::Transcripts,
            ]
        );
        assert_eq!(preview.changed_domain_count(), 3);
        assert!(preview.touches(SnapshotRestoreDomain::Sessions));
        assert!(preview.touches(SnapshotRestoreDomain::Memories));
        assert!(preview.touches(SnapshotRestoreDomain::Transcripts));
        assert_eq!(
            preview.impact_for(SnapshotRestoreDomain::Memories),
            Some(SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 5,
                },
            })
        );
    }

    #[test]
    fn snapshot_restore_impact_compacts_changed_domains_counts_and_issue_flags() {
        let preview = SnapshotRestorePreview {
            current_audit: SnapshotAuditReport {
                memory_integrity: MemorySnapshotIntegrityReport {
                    blank_session_title_count: 1,
                    ..MemorySnapshotIntegrityReport::default()
                },
                ..SnapshotAuditReport::default()
            },
            incoming_audit: SnapshotAuditReport {
                transcript_integrity: TranscriptSnapshotIntegrityReport {
                    blank_content_count: 2,
                    ..TranscriptSnapshotIntegrityReport::default()
                },
                ..SnapshotAuditReport::default()
            },
            session_delta: SessionRestoreDelta {
                updated_session_ids: vec![SessionId("session-1".into())],
                unchanged_count: 2,
                ..SessionRestoreDelta::default()
            },
            memory_delta: MemoryRestoreDelta {
                added_memory_ids: vec!["memory-2".into()],
                unchanged_count: 1,
                ..MemoryRestoreDelta::default()
            },
            transcript_delta: TranscriptRestoreDelta {
                unchanged_count: 3,
                ..TranscriptRestoreDelta::default()
            },
        };

        let impact = preview.impact();

        assert_eq!(impact.change_totals, preview.change_totals());
        assert_eq!(
            impact.changed_domains,
            vec![
                SnapshotRestoreDomain::Sessions,
                SnapshotRestoreDomain::Memories,
            ]
        );
        assert_eq!(
            impact.domain_impacts,
            vec![
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Sessions,
                    counts: RestoreDeltaCounts {
                        added_count: 0,
                        removed_count: 0,
                        updated_count: 1,
                        unchanged_count: 2,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Memories,
                    counts: RestoreDeltaCounts {
                        added_count: 1,
                        removed_count: 0,
                        updated_count: 0,
                        unchanged_count: 1,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Transcripts,
                    counts: RestoreDeltaCounts {
                        added_count: 0,
                        removed_count: 0,
                        updated_count: 0,
                        unchanged_count: 3,
                    },
                },
            ]
        );
        assert_eq!(impact.changed_domain_count(), 2);
        assert!(impact.touches(SnapshotRestoreDomain::Sessions));
        assert!(impact.touches(SnapshotRestoreDomain::Memories));
        assert!(!impact.touches(SnapshotRestoreDomain::Transcripts));
        assert_eq!(
            impact
                .impact_for(SnapshotRestoreDomain::Memories)
                .expect("memory impact should be present")
                .counts,
            RestoreDeltaCounts {
                added_count: 1,
                removed_count: 0,
                updated_count: 0,
                unchanged_count: 1,
            }
        );
        assert_eq!(impact.current_issue_count, 1);
        assert_eq!(impact.incoming_issue_count, 2);
        assert_eq!(impact.total_issue_count(), 3);
        assert!(impact.has_integrity_issues());
        assert_eq!(impact.change_count(), 2);
        assert!(!impact.is_noop());
    }

    #[test]
    fn snapshot_restore_impact_roundtrips_through_json() {
        let impact = SnapshotRestoreImpact {
            change_totals: RestoreDeltaCounts {
                added_count: 1,
                removed_count: 2,
                updated_count: 3,
                unchanged_count: 4,
            },
            changed_domains: vec![
                SnapshotRestoreDomain::Memories,
                SnapshotRestoreDomain::Transcripts,
            ],
            domain_impacts: vec![
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Sessions,
                    counts: RestoreDeltaCounts {
                        added_count: 0,
                        removed_count: 0,
                        updated_count: 0,
                        unchanged_count: 1,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Memories,
                    counts: RestoreDeltaCounts {
                        added_count: 1,
                        removed_count: 0,
                        updated_count: 1,
                        unchanged_count: 2,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Transcripts,
                    counts: RestoreDeltaCounts {
                        added_count: 0,
                        removed_count: 2,
                        updated_count: 2,
                        unchanged_count: 1,
                    },
                },
            ],
            current_issue_count: 5,
            incoming_issue_count: 6,
        };

        let json = serde_json::to_string(&impact).expect("restore impact should serialize");
        let parsed: SnapshotRestoreImpact =
            serde_json::from_str(&json).expect("restore impact should deserialize");

        assert_eq!(parsed, impact);
        assert_eq!(parsed.change_count(), 6);
        assert_eq!(parsed.changed_domain_count(), 2);
        assert_eq!(parsed.domain_impacts.len(), 3);
        assert_eq!(parsed.total_issue_count(), 11);
        assert!(parsed.has_integrity_issues());
    }

    #[test]
    fn snapshot_restore_readiness_matches_preview_and_impact_helpers() {
        let preview = SnapshotRestorePreview {
            current_audit: SnapshotAuditReport {
                memory_integrity: MemorySnapshotIntegrityReport {
                    blank_session_title_count: 1,
                    ..MemorySnapshotIntegrityReport::default()
                },
                ..SnapshotAuditReport::default()
            },
            incoming_audit: SnapshotAuditReport {
                transcript_integrity: TranscriptSnapshotIntegrityReport {
                    blank_content_count: 2,
                    ..TranscriptSnapshotIntegrityReport::default()
                },
                ..SnapshotAuditReport::default()
            },
            session_delta: SessionRestoreDelta {
                updated_session_ids: vec![SessionId("session-1".into())],
                unchanged_count: 2,
                ..SessionRestoreDelta::default()
            },
            memory_delta: MemoryRestoreDelta {
                added_memory_ids: vec!["memory-2".into()],
                unchanged_count: 1,
                ..MemoryRestoreDelta::default()
            },
            transcript_delta: TranscriptRestoreDelta {
                unchanged_count: 3,
                ..TranscriptRestoreDelta::default()
            },
        };

        let readiness = preview.readiness();

        assert_eq!(readiness, SnapshotRestoreReadiness::from_preview(&preview));
        assert_eq!(readiness, preview.impact().readiness());
        assert_eq!(readiness.change_totals, preview.change_totals());
        assert_eq!(readiness.changed_domain_count, 2);
        assert_eq!(readiness.change_count(), 2);
        assert!(readiness.has_changes());
        assert_eq!(readiness.current_issue_count, 1);
        assert_eq!(readiness.incoming_issue_count, 2);
        assert_eq!(readiness.total_issue_count(), 3);
        assert!(readiness.has_integrity_issues());
        assert!(!readiness.is_noop());
        assert!(!readiness.is_ready());
    }

    #[test]
    fn snapshot_restore_readiness_roundtrips_through_json() {
        let readiness = SnapshotRestoreReadiness {
            change_totals: RestoreDeltaCounts {
                added_count: 1,
                removed_count: 2,
                updated_count: 3,
                unchanged_count: 4,
            },
            changed_domain_count: 2,
            current_issue_count: 5,
            incoming_issue_count: 6,
        };

        let json = serde_json::to_string(&readiness).expect("restore readiness should serialize");
        let parsed: SnapshotRestoreReadiness =
            serde_json::from_str(&json).expect("restore readiness should deserialize");

        assert_eq!(parsed, readiness);
        assert_eq!(parsed.change_count(), 6);
        assert_eq!(parsed.total_issue_count(), 11);
        assert!(parsed.has_changes());
        assert!(parsed.has_integrity_issues());
        assert!(!parsed.is_ready());
    }

    #[test]
    fn snapshot_restore_readiness_deserializes_from_sparse_json() {
        let parsed: SnapshotRestoreReadiness = serde_json::from_str("{}")
            .expect("sparse restore readiness should deserialize with defaults");

        assert_eq!(parsed, SnapshotRestoreReadiness::default());
        assert_eq!(parsed.change_count(), 0);
        assert_eq!(parsed.changed_domain_count, 0);
        assert_eq!(parsed.total_issue_count(), 0);
        assert!(!parsed.has_changes());
        assert!(!parsed.has_integrity_issues());
        assert!(parsed.is_noop());
        assert!(parsed.is_ready());
    }

    #[test]
    fn snapshot_restore_safety_matches_preview_impact_and_readiness() {
        let preview = SnapshotRestorePreview {
            current_audit: SnapshotAuditReport {
                memory_integrity: MemorySnapshotIntegrityReport {
                    blank_memory_content_count: 1,
                    ..MemorySnapshotIntegrityReport::default()
                },
                ..SnapshotAuditReport::default()
            },
            incoming_audit: SnapshotAuditReport {
                transcript_integrity: TranscriptSnapshotIntegrityReport {
                    blank_content_count: 2,
                    ..TranscriptSnapshotIntegrityReport::default()
                },
                ..SnapshotAuditReport::default()
            },
            session_delta: SessionRestoreDelta {
                added_session_ids: vec![SessionId("session-2".into())],
                unchanged_count: 1,
                ..SessionRestoreDelta::default()
            },
            memory_delta: MemoryRestoreDelta {
                updated_memory_ids: vec!["memory-1".into()],
                unchanged_count: 2,
                ..MemoryRestoreDelta::default()
            },
            transcript_delta: TranscriptRestoreDelta {
                unchanged_count: 3,
                ..TranscriptRestoreDelta::default()
            },
        };

        let readiness = preview.readiness();
        let safety = preview.safety();

        assert_eq!(safety, SnapshotRestoreSafety::from_preview(&preview));
        assert_eq!(safety, preview.impact().safety());
        assert_eq!(safety, readiness.safety());
        assert_eq!(safety.change_totals, preview.change_totals());
        assert_eq!(safety.changed_domain_count, 2);
        assert_eq!(safety.change_count(), 2);
        assert!(safety.has_additions());
        assert!(safety.has_updates());
        assert!(!safety.has_removals());
        assert!(safety.has_changes);
        assert!(safety.touches_existing_records);
        assert!(!safety.additive_only);
        assert_eq!(safety.total_issue_count(), 3);
        assert!(safety.has_integrity_issues);
        assert!(!safety.is_ready());
        assert!(!safety.is_noop());
    }

    #[test]
    fn snapshot_restore_safety_roundtrips_through_json() {
        let safety = SnapshotRestoreSafety {
            change_totals: RestoreDeltaCounts {
                added_count: 1,
                removed_count: 2,
                updated_count: 0,
                unchanged_count: 4,
            },
            changed_domain_count: 2,
            current_issue_count: 3,
            incoming_issue_count: 5,
            has_changes: true,
            touches_existing_records: true,
            additive_only: false,
            has_integrity_issues: true,
        };

        let json = serde_json::to_string(&safety).expect("restore safety should serialize");
        let parsed: SnapshotRestoreSafety =
            serde_json::from_str(&json).expect("restore safety should deserialize");

        assert_eq!(parsed, safety);
        assert_eq!(parsed.change_count(), 3);
        assert_eq!(parsed.total_issue_count(), 8);
        assert!(parsed.has_changes);
        assert!(parsed.touches_existing_records);
        assert!(parsed.has_integrity_issues);
        assert!(!parsed.is_ready());
    }

    #[test]
    fn snapshot_restore_safety_deserializes_from_sparse_json() {
        let parsed: SnapshotRestoreSafety = serde_json::from_str("{}")
            .expect("sparse restore safety should deserialize with defaults");

        assert_eq!(parsed, SnapshotRestoreSafety::default());
        assert_eq!(parsed.change_count(), 0);
        assert_eq!(parsed.changed_domain_count, 0);
        assert_eq!(parsed.total_issue_count(), 0);
        assert!(!parsed.has_changes);
        assert!(!parsed.touches_existing_records);
        assert!(!parsed.additive_only);
        assert!(!parsed.has_integrity_issues);
        assert!(parsed.is_ready());
        assert!(parsed.is_noop());
    }

    #[test]
    fn snapshot_restore_impact_deserializes_without_domain_impacts_field() {
        let parsed: SnapshotRestoreImpact = serde_json::from_str(
            r#"{"change_totals":{"added_count":1},"changed_domains":["sessions"]}"#,
        )
        .expect("legacy restore impact should deserialize");

        assert_eq!(
            parsed,
            SnapshotRestoreImpact {
                change_totals: RestoreDeltaCounts {
                    added_count: 1,
                    ..RestoreDeltaCounts::default()
                },
                changed_domains: vec![SnapshotRestoreDomain::Sessions],
                domain_impacts: Vec::new(),
                current_issue_count: 0,
                incoming_issue_count: 0,
            }
        );
    }

    #[test]
    fn snapshot_restore_impact_deserializes_from_sparse_json() {
        let parsed: SnapshotRestoreImpact = serde_json::from_str("{}")
            .expect("sparse restore impact should deserialize with defaults");

        assert_eq!(parsed, SnapshotRestoreImpact::default());
        assert_eq!(parsed.change_count(), 0);
        assert_eq!(parsed.changed_domain_count(), 0);
        assert_eq!(parsed.total_issue_count(), 0);
        assert!(parsed.is_noop());
        assert!(!parsed.has_integrity_issues());
    }

    #[test]
    fn snapshot_restore_preview_deserializes_from_sparse_json() {
        let parsed: SnapshotRestorePreview = serde_json::from_str("{}")
            .expect("sparse restore preview should deserialize with defaults");

        assert_eq!(parsed, SnapshotRestorePreview::default());
        assert_eq!(parsed.change_count(), 0);
        assert!(parsed.changed_domains().is_empty());
        assert_eq!(parsed.changed_domain_count(), 0);
        assert!(!parsed.touches(SnapshotRestoreDomain::Sessions));
        assert_eq!(
            parsed.impact_for(SnapshotRestoreDomain::Transcripts),
            Some(SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts::default(),
            })
        );
        assert!(parsed.is_noop());
        assert!(!parsed.has_integrity_issues());
    }

    #[test]
    fn noop_restore_preview_keeps_zeroed_domain_impacts_in_stable_order() {
        let preview = SnapshotRestorePreview::default();

        assert_eq!(
            preview.domain_impacts(),
            vec![
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Sessions,
                    counts: RestoreDeltaCounts::default(),
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Memories,
                    counts: RestoreDeltaCounts::default(),
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Transcripts,
                    counts: RestoreDeltaCounts::default(),
                },
            ]
        );
        assert!(preview.changed_domains().is_empty());
        assert_eq!(
            preview.impact_for(SnapshotRestoreDomain::Sessions),
            Some(SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts::default(),
            })
        );
        assert_eq!(
            preview.impact_for(SnapshotRestoreDomain::Memories),
            Some(SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts::default(),
            })
        );
        assert_eq!(
            preview.impact_for(SnapshotRestoreDomain::Transcripts),
            Some(SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts::default(),
            })
        );
    }

    #[test]
    fn noop_restore_impact_keeps_zeroed_domain_impacts_in_stable_order() {
        let impact = SnapshotRestorePreview::default().impact();

        assert!(impact.changed_domains.is_empty());
        assert_eq!(
            impact.domain_impacts,
            vec![
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Sessions,
                    counts: RestoreDeltaCounts::default(),
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Memories,
                    counts: RestoreDeltaCounts::default(),
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Transcripts,
                    counts: RestoreDeltaCounts::default(),
                },
            ]
        );
        assert_eq!(
            impact
                .impact_for(SnapshotRestoreDomain::Sessions)
                .expect("session impact should be present"),
            &SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts::default(),
            }
        );
        assert_eq!(
            impact
                .impact_for(SnapshotRestoreDomain::Memories)
                .expect("memory impact should be present"),
            &SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts::default(),
            }
        );
        assert_eq!(
            impact
                .impact_for(SnapshotRestoreDomain::Transcripts)
                .expect("transcript impact should be present"),
            &SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts::default(),
            }
        );
        assert!(impact.is_noop());
    }

    #[test]
    fn restore_delta_counts_expose_additive_only_and_existing_record_flags() {
        let additive = RestoreDeltaCounts {
            added_count: 2,
            removed_count: 0,
            updated_count: 0,
            unchanged_count: 1,
        };
        let mixed = RestoreDeltaCounts {
            added_count: 1,
            removed_count: 1,
            updated_count: 1,
            unchanged_count: 0,
        };

        assert!(additive.has_additions());
        assert!(!additive.has_removals());
        assert!(!additive.has_updates());
        assert!(additive.has_changes());
        assert!(additive.is_additive_only());
        assert!(!additive.touches_existing_records());

        assert!(mixed.has_additions());
        assert!(mixed.has_removals());
        assert!(mixed.has_updates());
        assert!(mixed.has_changes());
        assert!(!mixed.is_additive_only());
        assert!(mixed.touches_existing_records());

        assert!(!RestoreDeltaCounts::default().has_changes());
        assert!(!RestoreDeltaCounts::default().is_additive_only());
        assert!(!RestoreDeltaCounts::default().touches_existing_records());
    }

    #[test]
    fn restore_domain_impact_delegates_change_shape_helpers() {
        let impact = SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Memories,
            counts: RestoreDeltaCounts {
                added_count: 1,
                removed_count: 0,
                updated_count: 0,
                unchanged_count: 2,
            },
        };

        assert_eq!(impact.change_count(), 1);
        assert!(impact.has_changes());
        assert!(impact.has_additions());
        assert!(!impact.has_removals());
        assert!(!impact.has_updates());
        assert!(impact.is_additive_only());
        assert!(!impact.touches_existing_records());
    }

    #[test]
    fn restore_impact_and_readiness_surface_change_shape_flags() {
        let additive_impact = SnapshotRestoreImpact {
            change_totals: RestoreDeltaCounts {
                added_count: 3,
                removed_count: 0,
                updated_count: 0,
                unchanged_count: 4,
            },
            changed_domains: vec![SnapshotRestoreDomain::Sessions],
            domain_impacts: vec![SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts {
                    added_count: 3,
                    removed_count: 0,
                    updated_count: 0,
                    unchanged_count: 4,
                },
            }],
            current_issue_count: 0,
            incoming_issue_count: 0,
        };
        let mixed_impact = SnapshotRestoreImpact {
            change_totals: RestoreDeltaCounts {
                added_count: 1,
                removed_count: 1,
                updated_count: 1,
                unchanged_count: 0,
            },
            changed_domains: vec![
                SnapshotRestoreDomain::Sessions,
                SnapshotRestoreDomain::Memories,
                SnapshotRestoreDomain::Transcripts,
            ],
            domain_impacts: Vec::new(),
            current_issue_count: 0,
            incoming_issue_count: 0,
        };
        let additive_readiness = additive_impact.readiness();
        let mixed_readiness = mixed_impact.readiness();

        assert!(additive_impact.has_additions());
        assert!(!additive_impact.has_removals());
        assert!(!additive_impact.has_updates());
        assert!(additive_impact.is_additive_only());
        assert!(!additive_impact.touches_existing_records());

        assert!(additive_readiness.has_additions());
        assert!(!additive_readiness.has_removals());
        assert!(!additive_readiness.has_updates());
        assert!(additive_readiness.is_additive_only());
        assert!(!additive_readiness.touches_existing_records());

        assert!(mixed_impact.has_additions());
        assert!(mixed_impact.has_removals());
        assert!(mixed_impact.has_updates());
        assert!(!mixed_impact.is_additive_only());
        assert!(mixed_impact.touches_existing_records());

        assert!(mixed_readiness.has_additions());
        assert!(mixed_readiness.has_removals());
        assert!(mixed_readiness.has_updates());
        assert!(!mixed_readiness.is_additive_only());
        assert!(mixed_readiness.touches_existing_records());
    }

    #[test]
    fn restore_mutation_profile_summarizes_domain_shape() {
        let impact = SnapshotRestoreImpact {
            change_totals: RestoreDeltaCounts {
                added_count: 3,
                removed_count: 1,
                updated_count: 1,
                unchanged_count: 5,
            },
            changed_domains: vec![
                SnapshotRestoreDomain::Sessions,
                SnapshotRestoreDomain::Memories,
            ],
            domain_impacts: vec![
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Sessions,
                    counts: RestoreDeltaCounts {
                        added_count: 2,
                        removed_count: 0,
                        updated_count: 0,
                        unchanged_count: 1,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Memories,
                    counts: RestoreDeltaCounts {
                        added_count: 1,
                        removed_count: 1,
                        updated_count: 1,
                        unchanged_count: 0,
                    },
                },
                SnapshotRestoreDomainImpact {
                    domain: SnapshotRestoreDomain::Transcripts,
                    counts: RestoreDeltaCounts {
                        added_count: 0,
                        removed_count: 0,
                        updated_count: 0,
                        unchanged_count: 4,
                    },
                },
            ],
            current_issue_count: 1,
            incoming_issue_count: 2,
        };

        let profile = impact.mutation_profile();

        assert_eq!(profile.changed_domain_count, 2);
        assert_eq!(profile.unchanged_domain_count, 1);
        assert_eq!(profile.addition_domain_count, 2);
        assert_eq!(profile.additive_only_domain_count, 1);
        assert_eq!(profile.existing_record_domain_count, 1);
        assert_eq!(profile.removal_domain_count, 1);
        assert_eq!(profile.total_issue_count(), 3);
        assert!(profile.has_changes());
        assert!(profile.has_additive_domains());
        assert!(profile.touches_existing_records());
        assert!(profile.has_removals());
        assert!(!profile.is_additive_only());
        assert!(profile.has_integrity_issues());
        assert!(!profile.is_ready());
    }

    #[test]
    fn restore_mutation_profile_matches_preview_helper() {
        let current_sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Current".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: None,
            archived_at_unix_ms: None,
        }];
        let incoming_sessions = vec![
            current_sessions[0].clone(),
            SessionRecord {
                session_id: SessionId("session-2".into()),
                agent_id: AgentId("builder".into()),
                title: "Added".into(),
                created_at_unix_ms: 3,
                last_active_unix_ms: 4,
                last_user_intent_summary: Some("additive restore".into()),
                archived_at_unix_ms: None,
            },
        ];
        let current_memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "current payload".into(),
        }];
        let incoming_memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "updated payload".into(),
        }];
        let preview = SnapshotRestorePreview::from_records_and_entries(
            &current_sessions,
            &current_memories,
            &[],
            &incoming_sessions,
            &incoming_memories,
            &[],
        );

        let profile = preview.mutation_profile();

        assert_eq!(profile, preview.impact().mutation_profile());
        assert_eq!(profile.changed_domain_count, 2);
        assert_eq!(profile.unchanged_domain_count, 1);
        assert_eq!(profile.addition_domain_count, 1);
        assert_eq!(profile.additive_only_domain_count, 1);
        assert_eq!(profile.existing_record_domain_count, 1);
        assert_eq!(profile.removal_domain_count, 0);
        assert!(profile.has_changes());
        assert!(profile.has_additive_domains());
        assert!(profile.touches_existing_records());
        assert!(!profile.has_removals());
        assert!(!profile.is_additive_only());
        assert!(profile.is_ready());
    }

    #[test]
    fn restore_mutation_profile_roundtrips_through_json() {
        let profile = SnapshotRestoreMutationProfile {
            changed_domain_count: 2,
            unchanged_domain_count: 1,
            addition_domain_count: 2,
            additive_only_domain_count: 1,
            existing_record_domain_count: 1,
            removal_domain_count: 1,
            current_issue_count: 3,
            incoming_issue_count: 5,
        };

        let json = serde_json::to_string(&profile).expect("mutation profile should serialize");
        let parsed: SnapshotRestoreMutationProfile =
            serde_json::from_str(&json).expect("mutation profile should deserialize");

        assert_eq!(parsed, profile);
        assert_eq!(parsed.total_issue_count(), 8);
        assert!(parsed.has_changes());
        assert!(parsed.has_additive_domains());
        assert!(parsed.touches_existing_records());
        assert!(parsed.has_removals());
        assert!(!parsed.is_additive_only());
        assert!(parsed.has_integrity_issues());
        assert!(!parsed.is_noop());
        assert!(!parsed.is_ready());
    }

    #[test]
    fn restore_mutation_profile_deserializes_from_sparse_json() {
        let parsed: SnapshotRestoreMutationProfile = serde_json::from_str("{}")
            .expect("sparse mutation profile should deserialize with defaults");

        assert_eq!(parsed, SnapshotRestoreMutationProfile::default());
        assert_eq!(parsed.changed_domain_count, 0);
        assert_eq!(parsed.unchanged_domain_count, 0);
        assert_eq!(parsed.total_issue_count(), 0);
        assert!(!parsed.has_changes());
        assert!(!parsed.has_additive_domains());
        assert!(!parsed.touches_existing_records());
        assert!(!parsed.has_removals());
        assert!(parsed.is_noop());
        assert!(parsed.is_ready());
    }

    #[test]
    fn restore_mutation_profile_falls_back_to_impact_flags_without_domain_impacts() {
        let impact = SnapshotRestoreImpact {
            change_totals: RestoreDeltaCounts {
                added_count: 2,
                removed_count: 0,
                updated_count: 1,
                unchanged_count: 0,
            },
            changed_domains: vec![
                SnapshotRestoreDomain::Sessions,
                SnapshotRestoreDomain::Memories,
            ],
            domain_impacts: Vec::new(),
            current_issue_count: 0,
            incoming_issue_count: 0,
        };

        let profile = impact.mutation_profile();

        assert_eq!(profile.changed_domain_count, 2);
        assert_eq!(profile.unchanged_domain_count, 1);
        assert_eq!(profile.addition_domain_count, 1);
        assert_eq!(profile.additive_only_domain_count, 0);
        assert_eq!(profile.existing_record_domain_count, 1);
        assert_eq!(profile.removal_domain_count, 0);
        assert!(profile.has_changes());
        assert!(!profile.has_additive_domains());
        assert!(profile.touches_existing_records());
        assert!(!profile.has_removals());
        assert!(!profile.is_additive_only());
        assert!(profile.is_ready());
    }

    #[test]
    fn query_report_limit_pressure_from_coverage_tracks_completion_state() {
        let truncated = QueryReportLimitPressure::from_coverage(&QueryReportCoverage {
            returned_count: 1,
            matched_count: 3,
        });
        let complete = QueryReportLimitPressure::from_coverage(&QueryReportCoverage {
            returned_count: 2,
            matched_count: 2,
        });

        assert_eq!(
            truncated,
            QueryReportLimitPressure {
                truncated: true,
                omitted_count: 2,
            }
        );
        assert!(!truncated.is_complete());
        assert!(!truncated.is_empty());

        assert_eq!(complete, QueryReportLimitPressure::default());
        assert!(complete.is_complete());
        assert!(complete.is_empty());
    }

    #[test]
    fn context_recall_report_from_bundle_preserves_compact_counts() {
        let request = ContextRecallRequest {
            session_id: SessionId("session-1".into()),
            query_text: Some("timeout".into()),
            recent_window_limit: 2,
            transcript_limit: 1,
            memory_limit: 2,
            allow_cross_session: true,
        };
        let bundle = ContextRecallBundle {
            request: request.clone(),
            recent_entries: vec![sample_transcript_entry(2, "timeout surfaced")],
            transcript_hits: vec![TranscriptSpan::from_entry(sample_transcript_entry(
                3,
                "timeout resolved",
            ))],
            durable_memory_hits: vec![MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "timeout retry guidance".into(),
            }],
            summary_hits: vec![MemoryRecord {
                id: "memory-2".into(),
                scope: MemoryScope::Session,
                content: "timeout summary".into(),
            }],
            active_topic_sessions: Vec::new(),
            active_neurons: Vec::new(),
            budget: ContextBudget::default(),
            ranked_items: Vec::new(),
            omitted_by_budget: 0,
            truncated: true,
        };

        let report = ContextRecallReport::from_bundle(&bundle);

        assert_eq!(report.request, request);
        assert_eq!(report.source_counts, bundle.source_counts());
        assert_eq!(report.query_hit_count(), 3);
        assert_eq!(report.total_item_count(), 4);
        assert!(report.has_query_matches());
        assert!(!report.is_empty());
        assert!(report.truncated);
    }

    #[test]
    fn context_recall_transcript_provenance_summary_splits_reason_lists() {
        let summary = ContextRecallTranscriptProvenanceSummary::from_span_refs(&[
            crate::TranscriptSpanRef {
                session_id: SessionId("session-1".into()),
                range: TranscriptRange {
                    start_sequence: 1,
                    end_sequence: 2,
                },
                reason: Some("recent_window, query_match".into()),
            },
            crate::TranscriptSpanRef {
                session_id: SessionId("session-1".into()),
                range: TranscriptRange {
                    start_sequence: 3,
                    end_sequence: 3,
                },
                reason: Some("query_match, active_topic_session".into()),
            },
            crate::TranscriptSpanRef {
                session_id: SessionId("session-2".into()),
                range: TranscriptRange {
                    start_sequence: 1,
                    end_sequence: 1,
                },
                reason: Some("   ".into()),
            },
            crate::TranscriptSpanRef {
                session_id: SessionId("   ".into()),
                range: TranscriptRange {
                    start_sequence: 4,
                    end_sequence: 4,
                },
                reason: None,
            },
        ]);

        assert_eq!(
            summary,
            ContextRecallTranscriptProvenanceSummary {
                span_count: 4,
                session_count: 2,
                spans_with_reason_count: 2,
                distinct_reason_count: 3,
            }
        );
        assert!(summary.has_spans());
        assert!(summary.has_reasons());
        assert!(!summary.is_empty());
    }

    #[test]
    fn snapshot_inspection_drift_impact_and_health_constructors_match_helpers() {
        let sessions = vec![SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("verify inspection health".into()),
            archived_at_unix_ms: None,
        }];
        let memories = vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "snapshot payload".into(),
        }];
        let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
        let inspection =
            SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);
        let report = SnapshotInspectionDriftReport {
            mismatched_sections: vec![
                SnapshotInspectionSection::MemoryManifest,
                SnapshotInspectionSection::TranscriptManifest,
                SnapshotInspectionSection::TranscriptIntegrity,
            ],
        };

        let impact = SnapshotInspectionDriftImpact::from_report(&report);
        let health = SnapshotInspectionHealth::from_bundle_and_records(
            &inspection,
            &sessions,
            &memories,
            &transcripts,
        );

        assert_eq!(
            impact,
            SnapshotInspectionDriftImpact {
                mismatch_count: 3,
                memory_mismatch_count: 1,
                transcript_mismatch_count: 2,
            }
        );
        assert_eq!(impact.changed_domain_count(), 2);
        assert!(impact.touches_memory());
        assert!(impact.touches_transcripts());
        assert!(!impact.is_aligned());

        assert_eq!(
            health,
            inspection.health_against_records(&sessions, &memories, &transcripts)
        );
        assert_eq!(health.issue_count(), 0);
        assert_eq!(health.mismatch_count(), 0);
        assert!(health.inspection_aligned());
        assert!(health.is_clean());
        assert!(health.is_ready());
    }

    #[test]
    fn snapshot_restore_readiness_and_safety_constructors_preserve_flags() {
        let impact = SnapshotRestoreImpact {
            change_totals: RestoreDeltaCounts {
                added_count: 1,
                removed_count: 1,
                updated_count: 0,
                unchanged_count: 2,
            },
            changed_domains: vec![
                SnapshotRestoreDomain::Sessions,
                SnapshotRestoreDomain::Memories,
            ],
            domain_impacts: vec![SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 0,
                    updated_count: 0,
                    unchanged_count: 1,
                },
            }],
            current_issue_count: 1,
            incoming_issue_count: 2,
        };

        let readiness = SnapshotRestoreReadiness::from_impact(&impact);
        let safety_from_impact = SnapshotRestoreSafety::from_impact(&impact);
        let safety_from_readiness = SnapshotRestoreSafety::from_readiness(&readiness);

        assert_eq!(readiness, impact.readiness());
        assert_eq!(safety_from_impact, impact.safety());
        assert_eq!(safety_from_readiness, readiness.safety());
        assert_eq!(readiness.change_count(), 2);
        assert_eq!(readiness.changed_domain_count, 2);
        assert!(readiness.has_changes());
        assert!(readiness.has_additions());
        assert!(readiness.has_removals());
        assert!(!readiness.has_updates());
        assert!(readiness.touches_existing_records());
        assert!(!readiness.is_additive_only());
        assert!(readiness.has_integrity_issues());
        assert!(!readiness.is_ready());

        assert!(safety_from_impact.has_changes);
        assert!(safety_from_impact.touches_existing_records);
        assert!(!safety_from_impact.additive_only);
        assert!(safety_from_impact.has_integrity_issues);
        assert!(!safety_from_impact.is_ready());
    }
}
