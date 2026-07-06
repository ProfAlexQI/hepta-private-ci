use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::TurnContextDecisionEntry;
use codex_protocol::protocol::TurnContextManifestEntry;
use codex_protocol::protocol::TurnContextTier;
use codex_protocol::protocol::stable_turn_context_manifest_replay_hash;

use super::classification::classify_contribution;
use super::classification::contribution_source;
use super::classification::estimate_manifest_content_tokens;
use super::classification::manifest_content_identity;
use super::classification::source_role;
use super::policy::source_is_omitted;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContextTruncationEvidence {
    pub(crate) source: String,
    pub(crate) source_id: &'static str,
    pub(crate) text_hash: String,
    pub(crate) estimated_tokens: u32,
    pub(crate) original_estimated_tokens: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContextCompressionEvidence {
    pub(crate) source: String,
    pub(crate) source_id: &'static str,
    pub(crate) kind: codex_protocol::protocol::TurnContextCompressionStageKind,
    pub(crate) original_text_hash: String,
    pub(crate) text_hash: String,
    pub(crate) estimated_tokens: u32,
    pub(crate) original_estimated_tokens: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContextContributionLedger {
    contributions: Vec<ContextContribution>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContextContribution {
    pub(crate) role: String,
    pub(crate) slot: String,
    pub(crate) source_id: &'static str,
    pub(crate) source: String,
    pub(crate) replay_key: String,
    pub(crate) text_hash: String,
    pub(crate) estimated_tokens: u32,
    pub(crate) policy_class: &'static str,
    pub(crate) include_reason: &'static str,
    pub(crate) tier: TurnContextTier,
}

impl ContextContributionLedger {
    pub(crate) fn contributions(&self) -> &[ContextContribution] {
        &self.contributions
    }

    pub(crate) fn from_response_items(context_items: &[ResponseItem]) -> Self {
        let mut contributions = Vec::new();

        for (item_index, item) in context_items.iter().enumerate() {
            let ResponseItem::Message { role, content, .. } = item else {
                continue;
            };
            let slot = source_role(role).to_string();
            for (content_index, content_item) in content.iter().enumerate() {
                let Some(identity) = manifest_content_identity(content_item) else {
                    continue;
                };
                let classification = classify_contribution(&slot, content_item);
                let text_hash = stable_turn_context_manifest_replay_hash(&identity);
                let source = contribution_source(
                    &slot,
                    classification.source_id,
                    item_index,
                    content_index,
                    content.len(),
                );
                let replay_key = format!("{source}:{text_hash}");
                contributions.push(ContextContribution {
                    role: role.clone(),
                    slot: slot.clone(),
                    source_id: classification.source_id,
                    source,
                    replay_key,
                    text_hash,
                    estimated_tokens: estimate_manifest_content_tokens(role, content_item),
                    policy_class: classification.policy_class,
                    include_reason: classification.include_reason,
                    tier: classification.tier,
                });
            }
        }

        Self { contributions }
    }

    pub(super) fn estimated_tokens(&self) -> u32 {
        self.contributions
            .iter()
            .map(|contribution| contribution.estimated_tokens)
            .fold(0_u32, u32::saturating_add)
    }

    pub(super) fn manifest_entries_excluding(
        &self,
        omitted_sources: &[String],
        truncation: Option<&ContextTruncationEvidence>,
        compressions: &[ContextCompressionEvidence],
    ) -> Vec<TurnContextManifestEntry> {
        self.contributions
            .iter()
            .filter(|contribution| !source_is_omitted(omitted_sources, &contribution.source))
            .map(|contribution| contribution.manifest_entry_with_rewrites(truncation, compressions))
            .collect()
    }

    pub(super) fn decision_entries_excluding(
        &self,
        omitted_sources: &[String],
        _truncation: Option<&ContextTruncationEvidence>,
    ) -> Vec<TurnContextDecisionEntry> {
        self.contributions
            .iter()
            .filter(|contribution| !source_is_omitted(omitted_sources, &contribution.source))
            .map(ContextContribution::decision_entry)
            .collect()
    }

    pub(super) fn estimated_tokens_excluding(
        &self,
        omitted_sources: &[String],
        truncation: Option<&ContextTruncationEvidence>,
        compressions: &[ContextCompressionEvidence],
    ) -> u32 {
        self.contributions
            .iter()
            .filter(|contribution| !source_is_omitted(omitted_sources, &contribution.source))
            .map(|contribution| {
                contribution.estimated_tokens_with_rewrites(truncation, compressions)
            })
            .fold(0_u32, u32::saturating_add)
    }
}

fn compression_evidence_for_source<'a>(
    compressions: &'a [ContextCompressionEvidence],
    source: &str,
) -> Option<&'a ContextCompressionEvidence> {
    compressions
        .iter()
        .find(|compression| compression.source == source)
}

impl ContextContribution {
    fn manifest_entry_with_rewrites(
        &self,
        truncation: Option<&ContextTruncationEvidence>,
        compressions: &[ContextCompressionEvidence],
    ) -> TurnContextManifestEntry {
        if let Some(truncation) = truncation
            && truncation.source == self.source
        {
            return TurnContextManifestEntry {
                role: self.role.clone(),
                tier: self.tier,
                replay_key: format!("{}:{}", self.source, truncation.text_hash),
                source: self.source.clone(),
                text_hash: truncation.text_hash.clone(),
                estimated_tokens: truncation.estimated_tokens,
            };
        }
        if let Some(compression) = compression_evidence_for_source(compressions, &self.source) {
            return TurnContextManifestEntry {
                role: self.role.clone(),
                tier: self.tier,
                replay_key: format!("{}:{}", self.source, compression.text_hash),
                source: self.source.clone(),
                text_hash: compression.text_hash.clone(),
                estimated_tokens: compression.estimated_tokens,
            };
        }

        TurnContextManifestEntry {
            role: self.role.clone(),
            tier: self.tier,
            replay_key: self.replay_key.clone(),
            source: self.source.clone(),
            text_hash: self.text_hash.clone(),
            estimated_tokens: self.estimated_tokens,
        }
    }

    fn estimated_tokens_with_rewrites(
        &self,
        truncation: Option<&ContextTruncationEvidence>,
        compressions: &[ContextCompressionEvidence],
    ) -> u32 {
        if let Some(truncation) = truncation
            && truncation.source == self.source
        {
            return truncation.estimated_tokens;
        }
        if let Some(compression) = compression_evidence_for_source(compressions, &self.source) {
            return compression.estimated_tokens;
        }
        self.estimated_tokens
    }

    fn decision_entry(&self) -> TurnContextDecisionEntry {
        TurnContextDecisionEntry::included(
            self.source.clone(),
            self.policy_class,
            Some(stable_turn_context_manifest_replay_hash(&format!(
                "{}:{}:{}:{}",
                self.slot, self.source_id, self.policy_class, self.include_reason
            ))),
        )
    }
}
