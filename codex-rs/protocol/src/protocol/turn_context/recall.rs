use super::common::is_false;
use super::common::is_stable_manifest_replay_hash;
use super::common::is_zero_u32;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextRecallSelectionSummary {
    pub returned_source_count: u32,
    pub selected_source_count: u32,
    pub ranked_source_count: u32,
    pub returned_unselected_source_count: u32,
    pub source_diversity_met: bool,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub source_diversity_target: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub max_per_source: u32,
    pub ranked_item_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub omitted_by_budget_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub memory_control_omitted_count: u32,
    pub low_trust_ranked_item_count: u32,
    pub low_recency_ranked_item_count: u32,
}

impl TurnContextRecallSelectionSummary {
    pub fn returned_unselected_source_count_matches(&self) -> bool {
        self.returned_unselected_source_count
            == self
                .returned_source_count
                .saturating_sub(self.selected_source_count)
    }

    pub fn source_diversity_target_matches(&self) -> bool {
        self.source_diversity_target == 0
            || self.source_diversity_met
                == (self.selected_source_count >= self.source_diversity_target)
    }

    pub fn has_count_integrity(&self) -> bool {
        self.selected_source_count <= self.returned_source_count
            && self.ranked_source_count <= self.selected_source_count
            && self.ranked_source_count <= self.ranked_item_count
            && (self.ranked_item_count == 0 || self.ranked_source_count > 0)
            && self.returned_unselected_source_count_matches()
            && self.source_diversity_target_matches()
            && self.low_trust_ranked_item_count <= self.ranked_item_count
            && self.low_recency_ranked_item_count <= self.ranked_item_count
    }
}

pub const TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextRecallSelectedSnippetEnvelope {
    pub version: u32,
    pub max_snippets: u32,
    pub max_snippet_chars: u32,
    pub selected_snippet_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub omitted_snippet_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub redacted_snippet_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub truncated_snippet_count: u32,
    pub snippets: Vec<TurnContextRecallSelectedSnippet>,
    pub safety: TurnContextRecallSelectedSnippetSafety,
}

impl TurnContextRecallSelectedSnippetEnvelope {
    pub fn counts_match(&self) -> bool {
        self.selected_snippet_count == u32::try_from(self.snippets.len()).unwrap_or(u32::MAX)
            && self.redacted_snippet_count
                == u32::try_from(
                    self.snippets
                        .iter()
                        .filter(|snippet| snippet.redacted)
                        .count(),
                )
                .unwrap_or(u32::MAX)
            && self.truncated_snippet_count
                == u32::try_from(
                    self.snippets
                        .iter()
                        .filter(|snippet| snippet.truncated)
                        .count(),
                )
                .unwrap_or(u32::MAX)
    }

    pub fn bounds_match(&self) -> bool {
        self.selected_snippet_count <= self.max_snippets
            && self.snippets.len() <= usize::try_from(self.max_snippets).unwrap_or(usize::MAX)
            && self.snippets.iter().all(|snippet| {
                !snippet.text.is_empty()
                    && snippet.text.chars().count()
                        <= usize::try_from(self.max_snippet_chars).unwrap_or(usize::MAX)
                    && is_stable_manifest_replay_hash(&snippet.snippet_hash)
            })
    }

    pub fn safety_matches(&self) -> bool {
        let forbidden_exposure = self.safety.origin_identifiers_exposed
            || self.safety.raw_ranked_payload_exposed
            || self.safety.rank_explanation_exposed
            || self.safety.control_marker_exposed
            || self.safety.query_payload_exposed
            || self.safety.per_origin_list_exposed
            || self
                .snippets
                .iter()
                .any(|snippet| snippet.text.contains("[hepta-memory:"));
        self.safety.bounded == self.bounds_match()
            && self.safety.ready_for_shadow_handoff == (self.safety.bounded && !forbidden_exposure)
            && self.safety.ready_for_shadow_handoff
    }

    pub fn has_shadow_integrity(&self) -> bool {
        self.version == TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION
            && self.counts_match()
            && self.bounds_match()
            && self.safety_matches()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextRecallSelectedSnippet {
    pub snippet_hash: String,
    pub text: String,
    pub estimated_tokens: u32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub redacted: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub truncated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextRecallSelectedSnippetSafety {
    pub ready_for_shadow_handoff: bool,
    pub bounded: bool,
    pub origin_identifiers_exposed: bool,
    pub raw_ranked_payload_exposed: bool,
    pub rank_explanation_exposed: bool,
    pub control_marker_exposed: bool,
    pub query_payload_exposed: bool,
    pub per_origin_list_exposed: bool,
}
