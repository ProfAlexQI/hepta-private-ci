use super::common::TURN_CONTEXT_MANIFEST_VERSION;
use super::common::TurnContextTier;
use super::common::is_false;
use super::common::is_stable_manifest_replay_hash;
use super::common::is_zero_u32;
use super::compression::TurnContextAdaptiveBudgetAllocation;
use super::compression::TurnContextCompressionCandidate;
use super::compression::TurnContextCompressionStage;
use super::decision::TurnContextDecisionEntry;
use super::decision::TurnContextDecisionLedgerSummary;
use super::decision::summarize_turn_context_decision_ledger;
use super::memory::TurnContextMemoryFormationReceipt;
use super::memory::TurnContextMemoryTaxonomyBucket;
use super::memory::TurnContextMemoryTemporalFact;
use super::recall::TurnContextRecallSelectedSnippetEnvelope;
use super::recall::TurnContextRecallSelectionSummary;
use super::stable_hash::StableManifestReplayHash;
use super::stable_hash::compute_decision_ledger_hash;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextManifestEntry {
    pub role: String,
    #[serde(default, skip_serializing_if = "TurnContextTier::is_unknown")]
    pub tier: TurnContextTier,
    pub source: String,
    pub replay_key: String,
    /// Stable 16-hex replay identity for the entry text. This is not a
    /// cryptographic trust digest and must not be used for approval integrity.
    pub text_hash: String,
    pub estimated_tokens: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextManifestItem {
    pub version: u32,
    pub estimated_tokens: u32,
    /// Stable 16-hex replay identity over the payload-light manifest fields.
    /// This is not a cryptographic trust digest and must not be used for
    /// operator approval, release, or activation integrity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub ledger_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub budget_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub omitted_entries: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub omitted_sources: Vec<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub truncated: bool,
    /// Stable 16-hex replay identity over the payload-light decision ledger.
    /// This is not a cryptographic trust digest and must not be used for
    /// operator approval, release, or activation integrity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub decision_ledger_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decision_ledger: Vec<TurnContextDecisionEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub recall_selection: Option<TurnContextRecallSelectionSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub recall_selected_snippets: Option<TurnContextRecallSelectedSnippetEnvelope>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memory_taxonomy: Vec<TurnContextMemoryTaxonomyBucket>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memory_formation_receipts: Vec<TurnContextMemoryFormationReceipt>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memory_temporal_facts: Vec<TurnContextMemoryTemporalFact>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compression_candidates: Vec<TurnContextCompressionCandidate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adaptive_budget_allocations: Vec<TurnContextAdaptiveBudgetAllocation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compression_stages: Vec<TurnContextCompressionStage>,
    pub entries: Vec<TurnContextManifestEntry>,
}

impl TurnContextManifestItem {
    pub fn with_refreshed_ledger_hash(mut self) -> Self {
        self.refresh_ledger_hash();
        self
    }

    pub fn refresh_ledger_hash(&mut self) {
        self.ledger_hash = Some(self.compute_ledger_hash());
        self.decision_ledger_hash = (!self.decision_ledger.is_empty())
            .then(|| compute_decision_ledger_hash(&self.decision_ledger));
    }

    pub fn has_supported_version(&self) -> bool {
        self.version == TURN_CONTEXT_MANIFEST_VERSION
    }

    pub fn ledger_hash_matches_manifest(&self) -> bool {
        self.ledger_hash
            .as_deref()
            .is_none_or(|hash| hash == self.compute_ledger_hash())
    }

    pub fn ledger_hash_is_compatible(&self) -> bool {
        self.ledger_hash
            .as_deref()
            .is_none_or(is_stable_manifest_replay_hash)
            && self.ledger_hash_matches_manifest()
    }

    pub fn entries_have_replay_integrity(&self) -> bool {
        !self.entries.is_empty()
            && self.entries.iter().all(|entry| {
                !entry.role.is_empty()
                    && !entry.source.is_empty()
                    && !entry.replay_key.is_empty()
                    && is_stable_manifest_replay_hash(&entry.text_hash)
                    && entry.replay_key.ends_with(&format!(":{}", entry.text_hash))
            })
    }

    pub fn decision_ledger_has_integrity(&self) -> bool {
        self.decision_ledger.iter().all(|entry| {
            !entry.source.is_empty()
                && !entry.decision.is_empty()
                && entry
                    .reason_hash
                    .as_deref()
                    .is_none_or(is_stable_manifest_replay_hash)
                && entry.kind().is_known()
        })
    }

    pub fn decision_ledger_summary(&self) -> TurnContextDecisionLedgerSummary {
        summarize_turn_context_decision_ledger(&self.decision_ledger)
    }

    pub fn decision_ledger_hash_is_compatible(&self) -> bool {
        match (
            self.decision_ledger.is_empty(),
            self.decision_ledger_hash.as_deref(),
        ) {
            (true, None) => true,
            (true, Some(hash)) => is_stable_manifest_replay_hash(hash),
            (false, Some(hash)) => {
                is_stable_manifest_replay_hash(hash)
                    && hash == compute_decision_ledger_hash(&self.decision_ledger)
            }
            (false, None) => true,
        }
    }

    pub fn recall_selection_has_integrity(&self) -> bool {
        self.recall_selection
            .as_ref()
            .is_none_or(TurnContextRecallSelectionSummary::has_count_integrity)
    }

    pub fn recall_selected_snippets_have_integrity(&self) -> bool {
        self.recall_selected_snippets
            .as_ref()
            .is_none_or(TurnContextRecallSelectedSnippetEnvelope::has_shadow_integrity)
    }

    pub fn memory_taxonomy_has_integrity(&self) -> bool {
        self.memory_taxonomy
            .iter()
            .all(TurnContextMemoryTaxonomyBucket::has_payload_light_integrity)
    }

    pub fn memory_formation_receipts_have_integrity(&self) -> bool {
        self.memory_formation_receipts
            .iter()
            .all(TurnContextMemoryFormationReceipt::has_payload_light_integrity)
    }

    pub fn memory_temporal_facts_have_integrity(&self) -> bool {
        self.memory_temporal_facts
            .iter()
            .all(TurnContextMemoryTemporalFact::has_payload_light_integrity)
    }

    pub fn compression_stages_have_integrity(&self) -> bool {
        self.compression_stages
            .iter()
            .all(TurnContextCompressionStage::has_payload_light_integrity)
    }

    pub fn compression_candidates_have_integrity(&self) -> bool {
        self.compression_candidates
            .iter()
            .all(TurnContextCompressionCandidate::has_payload_light_integrity)
    }

    pub fn adaptive_budget_allocations_have_integrity(&self) -> bool {
        self.adaptive_budget_allocations
            .iter()
            .all(TurnContextAdaptiveBudgetAllocation::has_payload_light_integrity)
    }

    pub fn has_replay_integrity(&self) -> bool {
        self.has_supported_version()
            && self.entries_have_replay_integrity()
            && self.ledger_hash_is_compatible()
            && self.decision_ledger_has_integrity()
            && self.decision_ledger_hash_is_compatible()
            && self.recall_selection_has_integrity()
            && self.recall_selected_snippets_have_integrity()
            && self.memory_taxonomy_has_integrity()
            && self.memory_formation_receipts_have_integrity()
            && self.memory_temporal_facts_have_integrity()
            && self.compression_candidates_have_integrity()
            && self.adaptive_budget_allocations_have_integrity()
            && self.compression_stages_have_integrity()
    }

    fn compute_ledger_hash(&self) -> String {
        let mut hash = StableManifestReplayHash::new();
        hash.update_u32(self.version);
        hash.update_u32(self.estimated_tokens);
        hash.update_option_u32(self.budget_tokens);
        hash.update_u32(self.omitted_entries);
        hash.update_vec_str(&self.omitted_sources);
        hash.update_bool(self.truncated);
        hash.update_vec_decisions(&self.decision_ledger);
        hash.update_recall_selection(self.recall_selection.as_ref());
        hash.update_recall_selected_snippets(self.recall_selected_snippets.as_ref());
        hash.update_memory_taxonomy(&self.memory_taxonomy);
        hash.update_memory_formation_receipts(&self.memory_formation_receipts);
        hash.update_memory_temporal_facts(&self.memory_temporal_facts);
        hash.update_compression_candidates(&self.compression_candidates);
        hash.update_adaptive_budget_allocations(&self.adaptive_budget_allocations);
        hash.update_compression_stages(&self.compression_stages);
        hash.update_vec_entries(&self.entries);
        hash.finish()
    }
}
