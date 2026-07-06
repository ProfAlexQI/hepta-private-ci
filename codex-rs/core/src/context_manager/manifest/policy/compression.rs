use crate::context_manager::budget_planner::estimated_compression_output_tokens;
use crate::context_manager::source_registry::source_aware_compression_kind;
use codex_protocol::protocol::TurnContextCompressionCandidate;
use codex_protocol::protocol::TurnContextCompressionCandidateReason;
use codex_protocol::protocol::TurnContextCompressionStageKind;
use codex_protocol::protocol::TurnContextTier;

use super::super::ContextContribution;
use super::super::ContextContributionLedger;
use super::ContextCompressionPlan;
use super::candidate::source_aware_omission_candidate_contributions;
use super::source_is_omitted;

pub(super) fn source_aware_compression_plans<'a>(
    ledger: &'a ContextContributionLedger,
    omitted_sources: &[String],
    summarize_retrieved_snippets: bool,
    defragment_tool_context: bool,
    prune_tool_context: bool,
) -> Vec<ContextCompressionPlan<'a>> {
    let mut plans = Vec::new();

    if summarize_retrieved_snippets
        && let Some(contribution) = ledger.contributions().iter().find(|contribution| {
            contribution.tier == TurnContextTier::RetrievedSnippets
                && contribution.source_id == "selected_context_recall"
                && !source_is_omitted(omitted_sources, &contribution.source)
        })
    {
        plans.push(ContextCompressionPlan {
            contribution,
            kind: TurnContextCompressionStageKind::Summary,
        });
    }

    if defragment_tool_context
        && let Some(contribution) = ledger
            .contributions()
            .iter()
            .filter(|contribution| source_aware_tool_defragment_source(contribution))
            .filter(|contribution| !source_is_omitted(omitted_sources, &contribution.source))
            .max_by(|left, right| {
                left.estimated_tokens
                    .cmp(&right.estimated_tokens)
                    .then_with(|| right.source.cmp(&left.source))
            })
    {
        plans.push(ContextCompressionPlan {
            contribution,
            kind: TurnContextCompressionStageKind::Defragment,
        });
    }

    if prune_tool_context
        && let Some(contribution) = ledger
            .contributions()
            .iter()
            .find(|contribution| source_aware_tool_prune_source(contribution))
            .filter(|contribution| !source_is_omitted(omitted_sources, &contribution.source))
    {
        plans.push(ContextCompressionPlan {
            contribution,
            kind: TurnContextCompressionStageKind::Prune,
        });
    }

    plans
}

pub(in crate::context_manager::manifest) fn source_aware_tool_defragment_source(
    contribution: &ContextContribution,
) -> bool {
    source_aware_compression_kind(contribution.tier, contribution.source_id)
        == Some(TurnContextCompressionStageKind::Defragment)
}

pub(in crate::context_manager::manifest) fn source_aware_tool_prune_source(
    contribution: &ContextContribution,
) -> bool {
    source_aware_compression_kind(contribution.tier, contribution.source_id)
        == Some(TurnContextCompressionStageKind::Prune)
}

pub(super) fn source_aware_compression_candidates(
    ledger: &ContextContributionLedger,
) -> Vec<TurnContextCompressionCandidate> {
    source_aware_omission_candidate_contributions(ledger)
        .into_iter()
        .filter_map(compression_candidate_for_contribution)
        .collect()
}

fn compression_candidate_for_contribution(
    contribution: &ContextContribution,
) -> Option<TurnContextCompressionCandidate> {
    let kind = source_aware_compression_kind(contribution.tier, contribution.source_id)?;
    let estimated_output_tokens =
        estimated_compression_output_tokens(kind, contribution.estimated_tokens);
    if estimated_output_tokens >= contribution.estimated_tokens {
        return None;
    }

    Some(TurnContextCompressionCandidate {
        kind,
        tier: contribution.tier,
        source_id: contribution.source_id.to_string(),
        input_tokens: contribution.estimated_tokens,
        estimated_output_tokens,
        affected_entries: 1,
        not_executed_reason: TurnContextCompressionCandidateReason::BudgetPressureDryRun,
    })
}
