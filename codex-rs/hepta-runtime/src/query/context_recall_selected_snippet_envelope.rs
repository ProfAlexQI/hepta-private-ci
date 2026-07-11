use hepta_core::ContextRecallItem;

use super::RuntimeContextRecallSelectedSnippet;
use super::RuntimeContextRecallSelectedSnippetEnvelope;
use super::RuntimeContextRecallSelectedSnippetSafety;
use super::RuntimeContextRecallSlice;

const ENVELOPE_VERSION: u32 = 1;
pub(super) const DEFAULT_MAX_SNIPPETS: usize = 4;
pub(super) const DEFAULT_MAX_SNIPPET_CHARS: usize = 120;

pub(super) fn build(
    slice: &RuntimeContextRecallSlice,
    query_text: Option<&str>,
) -> RuntimeContextRecallSelectedSnippetEnvelope {
    build_with_limits(
        slice,
        query_text,
        DEFAULT_MAX_SNIPPETS,
        DEFAULT_MAX_SNIPPET_CHARS,
    )
}

fn build_with_limits(
    slice: &RuntimeContextRecallSlice,
    query_text: Option<&str>,
    max_snippets: usize,
    max_snippet_chars: usize,
) -> RuntimeContextRecallSelectedSnippetEnvelope {
    let query_text = query_text.map(str::trim).filter(|text| !text.is_empty());
    let candidate_count = slice.bundle.ranked_items.len();
    let mut snippets = Vec::new();
    let mut redacted_snippet_count = 0;
    let mut truncated_snippet_count = 0;

    for item in &slice.bundle.ranked_items {
        if snippets.len() >= max_snippets {
            break;
        }
        let Some(snippet) = snippet_from_ranked_item(item, query_text, max_snippet_chars) else {
            continue;
        };
        if snippet.redacted {
            redacted_snippet_count += 1;
        }
        if snippet.truncated {
            truncated_snippet_count += 1;
        }
        snippets.push(snippet);
    }

    let omitted_snippet_count = candidate_count.saturating_sub(snippets.len());
    let safety = safety_gate(&snippets, query_text, max_snippets, max_snippet_chars);

    RuntimeContextRecallSelectedSnippetEnvelope {
        version: ENVELOPE_VERSION,
        max_snippets,
        max_snippet_chars,
        selected_snippet_count: snippets.len(),
        omitted_snippet_count,
        redacted_snippet_count,
        truncated_snippet_count,
        snippets,
        safety,
    }
}

fn snippet_from_ranked_item(
    item: &ContextRecallItem,
    query_text: Option<&str>,
    max_snippet_chars: usize,
) -> Option<RuntimeContextRecallSelectedSnippet> {
    let text = normalize_snippet_text(&item.summary);
    if text.is_empty() {
        return None;
    }
    let (text, redacted) = redact_shadow_snippet(&text, query_text);
    let (text, truncated) = truncate_chars(&text, max_snippet_chars);

    Some(RuntimeContextRecallSelectedSnippet {
        snippet_hash: stable_snippet_hash(&text),
        estimated_tokens: estimate_snippet_tokens(&text),
        text,
        redacted,
        truncated,
    })
}

fn safety_gate(
    snippets: &[RuntimeContextRecallSelectedSnippet],
    query_text: Option<&str>,
    max_snippets: usize,
    max_snippet_chars: usize,
) -> RuntimeContextRecallSelectedSnippetSafety {
    let bounded = snippets.len() <= max_snippets
        && snippets
            .iter()
            .all(|snippet| snippet.text.chars().count() <= max_snippet_chars);
    let control_marker_exposed = snippets
        .iter()
        .any(|snippet| snippet.text.contains("[hepta-memory:"));
    let query_payload_exposed =
        query_text.is_some_and(|query| snippets.iter().any(|snippet| snippet.text.contains(query)));
    let origin_identifiers_exposed = false;
    let raw_ranked_payload_exposed = false;
    let rank_explanation_exposed = false;
    let per_origin_list_exposed = false;
    let ready_for_shadow_handoff = bounded
        && !origin_identifiers_exposed
        && !raw_ranked_payload_exposed
        && !rank_explanation_exposed
        && !control_marker_exposed
        && !query_payload_exposed
        && !per_origin_list_exposed;

    RuntimeContextRecallSelectedSnippetSafety {
        ready_for_shadow_handoff,
        bounded,
        origin_identifiers_exposed,
        raw_ranked_payload_exposed,
        rank_explanation_exposed,
        control_marker_exposed,
        query_payload_exposed,
        per_origin_list_exposed,
    }
}

fn normalize_snippet_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn redact_shadow_snippet(text: &str, query_text: Option<&str>) -> (String, bool) {
    let mut redacted = false;
    let mut text = text.replace("[hepta-memory:tombstone]", "[redacted-memory-control]");
    let control_redacted = text.contains("[redacted-memory-control]");
    text = text.replace("[hepta-memory:conflict]", "[redacted-memory-control]");
    redacted |= control_redacted || text.contains("[redacted-memory-control]");
    if let Some(query_text) = query_text
        && text.contains(query_text)
    {
        text = text.replace(query_text, "[redacted-query]");
        redacted = true;
    }
    (text, redacted)
}

fn truncate_chars(text: &str, max_chars: usize) -> (String, bool) {
    if max_chars == 0 {
        return (String::new(), !text.is_empty());
    }
    let mut chars = text.chars();
    let truncated = text.chars().count() > max_chars;
    let text = chars.by_ref().take(max_chars).collect::<String>();
    (text, truncated)
}

fn estimate_snippet_tokens(text: &str) -> u32 {
    let token_estimate = text.len().saturating_add(3) / 4;
    u32::try_from(token_estimate).unwrap_or(u32::MAX)
}

fn stable_snippet_hash(text: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in b"hepta-runtime-recall-snippet-v1:" {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}
