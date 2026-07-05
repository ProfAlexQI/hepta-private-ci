use crate::context::EXTENSION_DEVELOPER_CAPABILITIES_CLOSE_TAG;
use crate::context::EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::APPS_INSTRUCTIONS_CLOSE_TAG;
use codex_protocol::protocol::APPS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::PLUGINS_INSTRUCTIONS_CLOSE_TAG;
use codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::SKILLS_INSTRUCTIONS_CLOSE_TAG;
use codex_protocol::protocol::SKILLS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::TurnContextCompressionStageKind;
use codex_protocol::protocol::TurnContextTier;
use codex_protocol::protocol::stable_turn_context_manifest_replay_hash;

use super::ContextCompressionEvidence;
use super::ContextContribution;
use super::ContextTruncationEvidence;
use super::classification::classify_contribution;
use super::classification::contribution_source;
use super::classification::estimate_manifest_content_tokens;
use super::classification::manifest_content_identity;
use super::classification::manifest_content_text;
use super::classification::source_role;
use super::policy::ContextCompressionPlan;
use super::policy::ContextTruncationPlan;
use super::policy::source_aware_tool_defragment_source;
use super::policy::source_aware_tool_prune_source;
use super::policy::source_is_omitted;
use super::selected_snippet::LIVE_RECALL_SELECTED_SNIPPETS_FOOTER;
use super::selected_snippet::LIVE_RECALL_SELECTED_SNIPPETS_HEADER;

pub(super) struct ContextRewriteResult {
    pub(super) context_items: Vec<ResponseItem>,
    pub(super) truncation: Option<ContextTruncationEvidence>,
    pub(super) compressions: Vec<ContextCompressionEvidence>,
}

pub(super) fn rewrite_context_items_for_assembly(
    context_items: &[ResponseItem],
    omitted_sources: &[String],
    truncation_plan: Option<&ContextTruncationPlan<'_>>,
    compression_plans: &[ContextCompressionPlan<'_>],
) -> ContextRewriteResult {
    let mut truncation = None;
    let mut compressions: Vec<ContextCompressionEvidence> = Vec::new();
    let context_items = context_items
        .iter()
        .enumerate()
        .filter_map(|(item_index, item)| match item {
            ResponseItem::Message {
                id,
                role,
                content,
                phase,
            } => {
                let slot = source_role(role).to_string();
                let rewritten_content = content
                    .iter()
                    .enumerate()
                    .filter_map(|(content_index, content_item)| {
                        let source = manifest_content_identity(content_item).map(|_| {
                            let classification = classify_contribution(&slot, content_item);
                            contribution_source(
                                &slot,
                                classification.source_id,
                                item_index,
                                content_index,
                                content.len(),
                            )
                        });
                        if source
                            .as_ref()
                            .is_some_and(|source| source_is_omitted(omitted_sources, source))
                        {
                            return None;
                        }

                        if let Some(plan) = compression_plans.iter().find(|plan| {
                            source.as_ref() == Some(&plan.contribution.source)
                                && !compressions.iter().any(|compression| {
                                    compression.source == plan.contribution.source
                                })
                        }) && let Some((compressed_content_item, compression_evidence)) =
                            compress_content_item_for_budget(role, content_item, plan)
                        {
                            compressions.push(compression_evidence);
                            return Some(compressed_content_item);
                        }

                        if truncation.is_none()
                            && let Some(plan) = truncation_plan
                            && source.as_ref() == Some(&plan.contribution.source)
                            && let Some((truncated_content_item, truncation_evidence)) =
                                truncate_content_item_for_budget(role, content_item, plan)
                        {
                            truncation = Some(truncation_evidence);
                            return Some(truncated_content_item);
                        }

                        Some(content_item.clone())
                    })
                    .collect::<Vec<_>>();

                (!rewritten_content.is_empty()).then(|| ResponseItem::Message {
                    id: id.clone(),
                    role: role.clone(),
                    content: rewritten_content,
                    phase: phase.clone(),
                })
            }
            _ => Some(item.clone()),
        })
        .collect();

    ContextRewriteResult {
        context_items,
        truncation,
        compressions,
    }
}

pub(super) fn filter_context_items_by_omitted_sources(
    context_items: &[ResponseItem],
    omitted_sources: &[String],
) -> Vec<ResponseItem> {
    if omitted_sources.is_empty() {
        return context_items.to_vec();
    }

    context_items
        .iter()
        .enumerate()
        .filter_map(|(item_index, item)| match item {
            ResponseItem::Message {
                id,
                role,
                content,
                phase,
            } => {
                let slot = source_role(role).to_string();
                let filtered_content = content
                    .iter()
                    .enumerate()
                    .filter_map(|(content_index, content_item)| {
                        let source = manifest_content_identity(content_item).map(|_| {
                            let classification = classify_contribution(&slot, content_item);
                            contribution_source(
                                &slot,
                                classification.source_id,
                                item_index,
                                content_index,
                                content.len(),
                            )
                        });
                        let should_omit = source
                            .as_ref()
                            .is_some_and(|source| source_is_omitted(omitted_sources, source));
                        (!should_omit).then(|| content_item.clone())
                    })
                    .collect::<Vec<_>>();

                (!filtered_content.is_empty()).then(|| ResponseItem::Message {
                    id: id.clone(),
                    role: role.clone(),
                    content: filtered_content,
                    phase: phase.clone(),
                })
            }
            _ => Some(item.clone()),
        })
        .collect()
}

fn compress_content_item_for_budget(
    role: &str,
    content_item: &ContentItem,
    plan: &ContextCompressionPlan<'_>,
) -> Option<(ContentItem, ContextCompressionEvidence)> {
    let original_text = manifest_content_text(content_item)?;
    let compressed_text = compressed_context_text_for_budget(original_text, plan)?;
    let compressed_content_item = match content_item {
        ContentItem::InputText { .. } => ContentItem::InputText {
            text: compressed_text,
        },
        ContentItem::OutputText { .. } => ContentItem::OutputText {
            text: compressed_text,
        },
        _ => return None,
    };
    let identity = manifest_content_identity(&compressed_content_item)?;
    let text_hash = stable_turn_context_manifest_replay_hash(&identity);
    let estimated_tokens = estimate_manifest_content_tokens(role, &compressed_content_item);
    if estimated_tokens >= plan.contribution.estimated_tokens {
        return None;
    }

    Some((
        compressed_content_item,
        ContextCompressionEvidence {
            source: plan.contribution.source.clone(),
            source_id: plan.contribution.source_id,
            kind: plan.kind,
            original_text_hash: plan.contribution.text_hash.clone(),
            text_hash,
            estimated_tokens,
            original_estimated_tokens: plan.contribution.estimated_tokens,
        },
    ))
}

fn compressed_context_text_for_budget(
    text: &str,
    plan: &ContextCompressionPlan<'_>,
) -> Option<String> {
    match plan.kind {
        TurnContextCompressionStageKind::Summary => {
            summarized_context_text_for_budget(text, plan.contribution)
        }
        TurnContextCompressionStageKind::Defragment => {
            defragmented_context_text_for_budget(text, plan.contribution)
        }
        TurnContextCompressionStageKind::Prune => {
            pruned_context_text_for_budget(text, plan.contribution)
        }
        TurnContextCompressionStageKind::Rewrite | TurnContextCompressionStageKind::Unknown => None,
    }
}

fn summarized_context_text_for_budget(
    text: &str,
    contribution: &ContextContribution,
) -> Option<String> {
    if contribution.tier != TurnContextTier::RetrievedSnippets
        || contribution.source_id != "selected_context_recall"
    {
        return None;
    }
    let summarized = format!(
        "{LIVE_RECALL_SELECTED_SNIPPETS_HEADER}\n[context summarized for budget]\n{LIVE_RECALL_SELECTED_SNIPPETS_FOOTER}"
    );
    (summarized.chars().count() < text.chars().count()).then_some(summarized)
}

fn defragmented_context_text_for_budget(
    text: &str,
    contribution: &ContextContribution,
) -> Option<String> {
    if !source_aware_tool_defragment_source(contribution) {
        return None;
    }
    let (open_tag, close_tag) = match contribution.source_id {
        "available_plugins" => (
            PLUGINS_INSTRUCTIONS_OPEN_TAG,
            PLUGINS_INSTRUCTIONS_CLOSE_TAG,
        ),
        "apps" => (APPS_INSTRUCTIONS_OPEN_TAG, APPS_INSTRUCTIONS_CLOSE_TAG),
        "available_skills" => (SKILLS_INSTRUCTIONS_OPEN_TAG, SKILLS_INSTRUCTIONS_CLOSE_TAG),
        _ => return None,
    };
    let defragmented = format!("{open_tag}\n[context defragmented for budget]\n{close_tag}");
    (defragmented.chars().count() < text.chars().count()).then_some(defragmented)
}

fn pruned_context_text_for_budget(
    text: &str,
    contribution: &ContextContribution,
) -> Option<String> {
    if !source_aware_tool_prune_source(contribution) {
        return None;
    }
    let pruned = format!(
        "{EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG}\n[context pruned for budget]\n{EXTENSION_DEVELOPER_CAPABILITIES_CLOSE_TAG}"
    );
    (pruned.chars().count() < text.chars().count()).then_some(pruned)
}

fn truncate_content_item_for_budget(
    role: &str,
    content_item: &ContentItem,
    plan: &ContextTruncationPlan<'_>,
) -> Option<(ContentItem, ContextTruncationEvidence)> {
    let original_text = manifest_content_text(content_item)?;
    let truncated_text = truncated_context_text_for_budget(
        original_text,
        plan.contribution.source_id,
        plan.remaining_over_budget,
        plan.contribution.estimated_tokens,
    )?;
    let truncated_content_item = match content_item {
        ContentItem::InputText { .. } => ContentItem::InputText {
            text: truncated_text,
        },
        ContentItem::OutputText { .. } => ContentItem::OutputText {
            text: truncated_text,
        },
        _ => return None,
    };
    let identity = manifest_content_identity(&truncated_content_item)?;
    let text_hash = stable_turn_context_manifest_replay_hash(&identity);
    let estimated_tokens = estimate_manifest_content_tokens(role, &truncated_content_item);
    if estimated_tokens >= plan.contribution.estimated_tokens {
        return None;
    }

    Some((
        truncated_content_item,
        ContextTruncationEvidence {
            source: plan.contribution.source.clone(),
            source_id: plan.contribution.source_id,
            text_hash,
            estimated_tokens,
            original_estimated_tokens: plan.contribution.estimated_tokens,
        },
    ))
}

fn truncated_context_text_for_budget(
    text: &str,
    source_id: &str,
    remaining_over_budget: u32,
    original_estimated_tokens: u32,
) -> Option<String> {
    if source_id == "selected_context_recall" {
        let truncated = format!(
            "{LIVE_RECALL_SELECTED_SNIPPETS_HEADER}\n[context truncated for budget]\n{LIVE_RECALL_SELECTED_SNIPPETS_FOOTER}"
        );
        return (truncated.chars().count() < text.chars().count()).then_some(truncated);
    }

    let original_chars = text.chars().count();
    let suffix = "\n[context truncated for budget]";
    if original_chars <= suffix.chars().count() + 1 {
        return None;
    }
    let target_tokens = original_estimated_tokens
        .saturating_sub(remaining_over_budget)
        .max(1);
    let target_chars = ((original_chars as u64 * target_tokens as u64)
        / u64::from(original_estimated_tokens.max(1))) as usize;
    let prefix_chars = target_chars
        .saturating_sub(suffix.chars().count())
        .clamp(1, original_chars.saturating_sub(1));
    let mut truncated = text.chars().take(prefix_chars).collect::<String>();
    truncated.push_str(suffix);

    (truncated.chars().count() < original_chars).then_some(truncated)
}
