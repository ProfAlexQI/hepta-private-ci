use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::TurnContextRecallSelectedSnippetEnvelope;

use super::ContextRecallSelectedSnippetEnvelope;

pub(super) const LIVE_RECALL_SELECTED_SNIPPETS_HEADER: &str = "<selected_context_recall>";
pub(super) const LIVE_RECALL_SELECTED_SNIPPETS_FOOTER: &str = "</selected_context_recall>";

const LIVE_RECALL_SELECTED_SNIPPET_FORBIDDEN_SUBSTRINGS: &[&str] = &[
    "[hepta-memory:",
    "memory_id",
    "neuron_id",
    "per origin",
    "per-origin",
    "per_source",
    "query payload",
    "query_payload",
    "rank explanation",
    "raw ranked payload",
    "raw_ranked_payload",
    "score reason",
    "score_reason",
    "source id",
    "source lane",
    "source memory id",
    "source-memory-id",
    "source_id",
    "source_lane",
    "source_memory_ids",
    "topic_id",
];

pub(crate) fn build_recall_selected_snippets_live_context_item(
    selected_snippets: Option<&ContextRecallSelectedSnippetEnvelope>,
) -> Option<ResponseItem> {
    let envelope = &selected_snippets?.envelope;
    if envelope.snippets.is_empty() || !selected_snippet_envelope_is_manifest_safe(envelope) {
        return None;
    }

    let mut text = String::new();
    text.push_str(LIVE_RECALL_SELECTED_SNIPPETS_HEADER);
    text.push('\n');
    text.push_str("Bounded recall snippets selected for this turn:\n");
    for snippet in &envelope.snippets {
        text.push_str("- snippet_hash=");
        text.push_str(&snippet.snippet_hash);
        text.push_str(" text: ");
        text.push_str(&snippet.text);
        if snippet.redacted {
            text.push_str(" [redacted]");
        }
        if snippet.truncated {
            text.push_str(" [truncated]");
        }
        text.push('\n');
    }
    text.push_str(LIVE_RECALL_SELECTED_SNIPPETS_FOOTER);

    Some(ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText { text }],
        phase: None,
    })
}

pub(super) fn selected_snippet_envelope_is_manifest_safe(
    envelope: &TurnContextRecallSelectedSnippetEnvelope,
) -> bool {
    envelope.has_shadow_integrity()
        && envelope
            .snippets
            .iter()
            .all(|snippet| selected_snippet_text_is_live_prompt_safe(&snippet.text))
}

fn selected_snippet_text_is_live_prompt_safe(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    !LIVE_RECALL_SELECTED_SNIPPET_FORBIDDEN_SUBSTRINGS
        .iter()
        .any(|needle| text.contains(needle))
}
