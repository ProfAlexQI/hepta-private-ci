use serde::Deserialize;
use serde::Serialize;

use super::CONTEXT_MEMORY_NAMESPACE_POLICY_SCHEMA_VERSION;
use super::ContextRecallInspection;
use super::ContextRecallSourceAvailability;

/// Coarse payload-light memory taxonomy class.
///
/// These buckets mirror common long-context systems without storing memory
/// contents, transcript text, ranked payloads, or source identifiers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryTaxonomyClass {
    Semantic,
    Episodic,
    Procedural,
    Control,
    Transcript,
    #[default]
    Unknown,
}

impl ContextMemoryTaxonomyClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::Episodic => "episodic",
            Self::Procedural => "procedural",
            Self::Control => "control",
            Self::Transcript => "transcript",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light returned-vs-available counts for one memory taxonomy class.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTaxonomyBucket {
    pub class: ContextMemoryTaxonomyClass,
    pub source_count: usize,
    pub returned_count: usize,
    pub available_count: usize,
    pub omitted_count: usize,
    pub provenance_span_count: usize,
}

impl ContextMemoryTaxonomyBucket {
    pub fn has_count_integrity(&self) -> bool {
        !self.class.is_unknown()
            && self.returned_count <= self.available_count
            && self.omitted_count == self.available_count.saturating_sub(self.returned_count)
            && (self.source_count > 0
                || (self.returned_count == 0
                    && self.available_count == 0
                    && self.omitted_count == 0
                    && self.provenance_span_count == 0))
    }

    pub fn is_empty(&self) -> bool {
        self.source_count == 0
            && self.returned_count == 0
            && self.available_count == 0
            && self.omitted_count == 0
            && self.provenance_span_count == 0
    }
}

/// Compact memory taxonomy report for recall diagnostics and future memory
/// formation planning.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTaxonomyReport {
    pub buckets: Vec<ContextMemoryTaxonomyBucket>,
}

impl ContextMemoryTaxonomyReport {
    pub fn from_recall_inspection(
        inspection: &ContextRecallInspection,
        source_availability: &ContextRecallSourceAvailability,
        memory_control_omitted_count: usize,
    ) -> Self {
        let mut buckets = Vec::new();
        let counts = &inspection.report.source_counts;

        push_nonempty_bucket(
            &mut buckets,
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Semantic,
                source_count: usize::from(source_availability.durable_memory_match_count > 0),
                returned_count: counts.durable_memory_hit_count,
                available_count: source_availability.durable_memory_match_count,
                omitted_count: source_availability
                    .durable_memory_match_count
                    .saturating_sub(counts.durable_memory_hit_count),
                provenance_span_count: 0,
            },
        );

        push_nonempty_bucket(
            &mut buckets,
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Episodic,
                source_count: usize::from(source_availability.summary_memory_match_count > 0),
                returned_count: counts.summary_hit_count,
                available_count: source_availability.summary_memory_match_count,
                omitted_count: source_availability
                    .summary_memory_match_count
                    .saturating_sub(counts.summary_hit_count),
                provenance_span_count: 0,
            },
        );

        push_nonempty_bucket(
            &mut buckets,
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Control,
                source_count: usize::from(memory_control_omitted_count > 0),
                returned_count: 0,
                available_count: memory_control_omitted_count,
                omitted_count: memory_control_omitted_count,
                provenance_span_count: 0,
            },
        );

        let transcript_source_count = usize::from(source_availability.recent_entry_count > 0)
            + usize::from(source_availability.transcript_match_count > 0);
        let transcript_returned_count = counts.recent_entry_count + counts.transcript_hit_count;
        let transcript_available_count =
            source_availability.recent_entry_count + source_availability.transcript_match_count;
        let provenance_span_count = inspection.transcript_provenance_summary().span_count;
        push_nonempty_bucket(
            &mut buckets,
            ContextMemoryTaxonomyBucket {
                class: ContextMemoryTaxonomyClass::Transcript,
                source_count: transcript_source_count,
                returned_count: transcript_returned_count,
                available_count: transcript_available_count,
                omitted_count: transcript_available_count.saturating_sub(transcript_returned_count),
                provenance_span_count,
            },
        );

        Self { buckets }
    }

    pub fn has_count_integrity(&self) -> bool {
        self.buckets
            .iter()
            .all(ContextMemoryTaxonomyBucket::has_count_integrity)
    }

    pub fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }
}

fn push_nonempty_bucket(
    buckets: &mut Vec<ContextMemoryTaxonomyBucket>,
    bucket: ContextMemoryTaxonomyBucket,
) {
    if !bucket.is_empty() {
        buckets.push(bucket);
    }
}

/// Fixed long-term memory namespace names for shadow policy review.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryNamespace {
    Core,
    Session,
    Procedural,
    Semantic,
    Episodic,
    Archival,
    #[default]
    Unknown,
}

impl ContextMemoryNamespace {
    pub const REQUIRED: [Self; 6] = [
        Self::Core,
        Self::Session,
        Self::Procedural,
        Self::Semantic,
        Self::Episodic,
        Self::Archival,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::Session => "session",
            Self::Procedural => "procedural",
            Self::Semantic => "semantic",
            Self::Episodic => "episodic",
            Self::Archival => "archival",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Controlled owner for a memory namespace policy block.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryNamespaceOwner {
    OperatorPolicy,
    SessionRuntime,
    ProcedurePolicy,
    SemanticMemory,
    EpisodicMemory,
    ArchivalStore,
    #[default]
    Unknown,
}

impl ContextMemoryNamespaceOwner {
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Coarse retention model for a memory namespace.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryNamespaceTtlPolicy {
    Indefinite,
    SessionBound,
    Rolling,
    Archival,
    #[default]
    Unknown,
}

impl ContextMemoryNamespaceTtlPolicy {
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Controlled privacy tier for a namespace policy block.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryNamespacePrivacyTier {
    OperatorControlled,
    UserPrivate,
    WorkspacePrivate,
    ArchivalPrivate,
    #[default]
    Unknown,
}

impl ContextMemoryNamespacePrivacyTier {
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Redaction rule required before any future namespace write can commit.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryNamespaceRedactionPolicy {
    RequiredBeforeWrite,
    MetadataOnly,
    OperatorReview,
    #[default]
    Unknown,
}

impl ContextMemoryNamespaceRedactionPolicy {
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Write-mode state for the namespace policy surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryNamespaceWritePolicy {
    ShadowProposalOnly,
    #[default]
    Unknown,
}

impl ContextMemoryNamespaceWritePolicy {
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// One payload-light policy block for a future memory namespace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryNamespacePolicyBlock {
    pub namespace: ContextMemoryNamespace,
    pub owner: ContextMemoryNamespaceOwner,
    pub ttl_policy: ContextMemoryNamespaceTtlPolicy,
    pub ttl_turns: u32,
    pub privacy_tier: ContextMemoryNamespacePrivacyTier,
    pub redaction_policy: ContextMemoryNamespaceRedactionPolicy,
    pub write_policy: ContextMemoryNamespaceWritePolicy,
    pub budget_tokens: u32,
    pub propose_write_required: bool,
    pub policy_approval_required: bool,
    pub operator_approval_required: bool,
    pub shadow_wal_required: bool,
    pub readback_required: bool,
    pub canary_required: bool,
    pub supersede_supported: bool,
    pub tombstone_supported: bool,
    pub rollback_supported: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
}

impl Default for ContextMemoryNamespacePolicyBlock {
    fn default() -> Self {
        Self {
            namespace: ContextMemoryNamespace::Unknown,
            owner: ContextMemoryNamespaceOwner::Unknown,
            ttl_policy: ContextMemoryNamespaceTtlPolicy::Unknown,
            ttl_turns: 0,
            privacy_tier: ContextMemoryNamespacePrivacyTier::Unknown,
            redaction_policy: ContextMemoryNamespaceRedactionPolicy::Unknown,
            write_policy: ContextMemoryNamespaceWritePolicy::Unknown,
            budget_tokens: 0,
            propose_write_required: false,
            policy_approval_required: false,
            operator_approval_required: false,
            shadow_wal_required: false,
            readback_required: false,
            canary_required: false,
            supersede_supported: false,
            tombstone_supported: false,
            rollback_supported: false,
            production_write: false,
            graph_write: false,
            hot_path_write: false,
            prompt_assembly_change: false,
            runtime_activation: false,
        }
    }
}

impl ContextMemoryNamespacePolicyBlock {
    pub fn has_policy_integrity(&self) -> bool {
        !self.namespace.is_unknown()
            && !self.owner.is_unknown()
            && !self.ttl_policy.is_unknown()
            && self.has_ttl_integrity()
            && !self.privacy_tier.is_unknown()
            && !self.redaction_policy.is_unknown()
            && self.write_policy == ContextMemoryNamespaceWritePolicy::ShadowProposalOnly
            && self.budget_tokens > 0
            && self.propose_write_required
            && self.policy_approval_required
            && self.operator_approval_required
            && self.shadow_wal_required
            && self.readback_required
            && self.canary_required
            && self.supersede_supported
            && self.tombstone_supported
            && self.rollback_supported
            && !self.production_write
            && !self.graph_write
            && !self.hot_path_write
            && !self.prompt_assembly_change
            && !self.runtime_activation
    }

    fn has_ttl_integrity(&self) -> bool {
        match self.ttl_policy {
            ContextMemoryNamespaceTtlPolicy::Indefinite => self.ttl_turns == 0,
            ContextMemoryNamespaceTtlPolicy::SessionBound
            | ContextMemoryNamespaceTtlPolicy::Rolling
            | ContextMemoryNamespaceTtlPolicy::Archival => self.ttl_turns > 0,
            ContextMemoryNamespaceTtlPolicy::Unknown => false,
        }
    }
}

/// Payload-light shadow policy report for future memory namespace blocks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryNamespacePolicyReport {
    pub schema_version: u32,
    pub blocks: Vec<ContextMemoryNamespacePolicyBlock>,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
}

impl Default for ContextMemoryNamespacePolicyReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_NAMESPACE_POLICY_SCHEMA_VERSION,
            blocks: Vec::new(),
            production_write: false,
            graph_write: false,
            hot_path_write: false,
            prompt_assembly_change: false,
            runtime_activation: false,
        }
    }
}

impl ContextMemoryNamespacePolicyReport {
    pub fn seeded() -> Self {
        Self {
            blocks: vec![
                namespace_policy_block(
                    ContextMemoryNamespace::Core,
                    ContextMemoryNamespaceOwner::OperatorPolicy,
                    ContextMemoryNamespaceTtlPolicy::Indefinite,
                    0,
                    ContextMemoryNamespacePrivacyTier::OperatorControlled,
                    ContextMemoryNamespaceRedactionPolicy::OperatorReview,
                    1024,
                ),
                namespace_policy_block(
                    ContextMemoryNamespace::Session,
                    ContextMemoryNamespaceOwner::SessionRuntime,
                    ContextMemoryNamespaceTtlPolicy::SessionBound,
                    256,
                    ContextMemoryNamespacePrivacyTier::UserPrivate,
                    ContextMemoryNamespaceRedactionPolicy::RequiredBeforeWrite,
                    2048,
                ),
                namespace_policy_block(
                    ContextMemoryNamespace::Procedural,
                    ContextMemoryNamespaceOwner::ProcedurePolicy,
                    ContextMemoryNamespaceTtlPolicy::Rolling,
                    4096,
                    ContextMemoryNamespacePrivacyTier::WorkspacePrivate,
                    ContextMemoryNamespaceRedactionPolicy::MetadataOnly,
                    1536,
                ),
                namespace_policy_block(
                    ContextMemoryNamespace::Semantic,
                    ContextMemoryNamespaceOwner::SemanticMemory,
                    ContextMemoryNamespaceTtlPolicy::Rolling,
                    8192,
                    ContextMemoryNamespacePrivacyTier::UserPrivate,
                    ContextMemoryNamespaceRedactionPolicy::RequiredBeforeWrite,
                    4096,
                ),
                namespace_policy_block(
                    ContextMemoryNamespace::Episodic,
                    ContextMemoryNamespaceOwner::EpisodicMemory,
                    ContextMemoryNamespaceTtlPolicy::Rolling,
                    2048,
                    ContextMemoryNamespacePrivacyTier::UserPrivate,
                    ContextMemoryNamespaceRedactionPolicy::RequiredBeforeWrite,
                    3072,
                ),
                namespace_policy_block(
                    ContextMemoryNamespace::Archival,
                    ContextMemoryNamespaceOwner::ArchivalStore,
                    ContextMemoryNamespaceTtlPolicy::Archival,
                    32768,
                    ContextMemoryNamespacePrivacyTier::ArchivalPrivate,
                    ContextMemoryNamespaceRedactionPolicy::OperatorReview,
                    8192,
                ),
            ],
            ..Self::default()
        }
    }

    pub fn namespace_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn operator_approval_required_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.operator_approval_required)
            .count()
    }

    pub fn shadow_wal_required_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.shadow_wal_required)
            .count()
    }

    pub fn readback_required_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.readback_required)
            .count()
    }

    pub fn canary_required_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.canary_required)
            .count()
    }

    pub fn rollback_supported_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.rollback_supported)
            .count()
    }

    pub fn production_write_count(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.production_write)
            .count()
    }

    pub fn graph_write_count(&self) -> usize {
        self.blocks.iter().filter(|block| block.graph_write).count()
    }

    pub fn has_policy_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_NAMESPACE_POLICY_SCHEMA_VERSION
            && self.required_namespaces_present_once()
            && self
                .blocks
                .iter()
                .all(ContextMemoryNamespacePolicyBlock::has_policy_integrity)
            && !self.production_write
            && !self.graph_write
            && !self.hot_path_write
            && !self.prompt_assembly_change
            && !self.runtime_activation
    }

    fn required_namespaces_present_once(&self) -> bool {
        if self.blocks.len() != ContextMemoryNamespace::REQUIRED.len() {
            return false;
        }

        let mut actual = self
            .blocks
            .iter()
            .map(|block| block.namespace.as_str())
            .collect::<Vec<_>>();
        actual.sort_unstable();
        actual.dedup();

        let mut expected = ContextMemoryNamespace::REQUIRED
            .iter()
            .map(|namespace| namespace.as_str())
            .collect::<Vec<_>>();
        expected.sort_unstable();

        actual == expected
    }
}

fn namespace_policy_block(
    namespace: ContextMemoryNamespace,
    owner: ContextMemoryNamespaceOwner,
    ttl_policy: ContextMemoryNamespaceTtlPolicy,
    ttl_turns: u32,
    privacy_tier: ContextMemoryNamespacePrivacyTier,
    redaction_policy: ContextMemoryNamespaceRedactionPolicy,
    budget_tokens: u32,
) -> ContextMemoryNamespacePolicyBlock {
    ContextMemoryNamespacePolicyBlock {
        namespace,
        owner,
        ttl_policy,
        ttl_turns,
        privacy_tier,
        redaction_policy,
        write_policy: ContextMemoryNamespaceWritePolicy::ShadowProposalOnly,
        budget_tokens,
        propose_write_required: true,
        policy_approval_required: true,
        operator_approval_required: true,
        shadow_wal_required: true,
        readback_required: true,
        canary_required: true,
        supersede_supported: true,
        tombstone_supported: true,
        rollback_supported: true,
        production_write: false,
        graph_write: false,
        hot_path_write: false,
        prompt_assembly_change: false,
        runtime_activation: false,
    }
}
