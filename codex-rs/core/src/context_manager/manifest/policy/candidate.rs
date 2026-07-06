use crate::context_manager::source_registry::source_aware_omit_priority;
use codex_protocol::protocol::TurnContextDecisionEntry;
use codex_protocol::protocol::TurnContextDecisionKind;
use codex_protocol::protocol::stable_turn_context_manifest_replay_hash;

use super::super::ContextContribution;
use super::super::ContextContributionLedger;

struct SourceAwareBudgetCandidate<'a> {
    contribution: &'a ContextContribution,
    priority: u8,
}

pub(super) struct SourceAwareBudgetOmissionDecision {
    pub(super) omitted_sources: Vec<String>,
    pub(super) remaining_over_budget: u32,
    pub(super) decisions: Vec<TurnContextDecisionEntry>,
}

pub(super) fn source_aware_budget_candidate_decisions(
    ledger: &ContextContributionLedger,
    strategy: &str,
    estimated_tokens: u32,
    budget_tokens: u32,
) -> Vec<TurnContextDecisionEntry> {
    let mut decisions = Vec::new();
    let mut remaining_over_budget = estimated_tokens.saturating_sub(budget_tokens);
    let mut omitted_candidate_sources = Vec::new();
    for candidate in source_aware_omission_candidates(ledger) {
        if remaining_over_budget == 0 {
            break;
        }
        remaining_over_budget =
            remaining_over_budget.saturating_sub(candidate.contribution.estimated_tokens);
        omitted_candidate_sources.push(candidate.contribution.source.clone());
        decisions.push(budget_candidate_decision(
            candidate.contribution,
            TurnContextDecisionKind::CandidateOmit {
                source_id: candidate.contribution.source_id.to_string(),
                priority: u32::from(candidate.priority),
                tokens: candidate.contribution.estimated_tokens,
            },
            strategy,
            budget_tokens,
            estimated_tokens,
        ));
    }

    if remaining_over_budget > 0
        && let Some(candidate) =
            source_aware_truncation_candidate(ledger, &omitted_candidate_sources)
    {
        decisions.push(budget_candidate_decision(
            candidate,
            TurnContextDecisionKind::CandidateTruncate {
                source_id: candidate.source_id.to_string(),
                remaining_over_budget,
                tokens: candidate.estimated_tokens,
            },
            strategy,
            budget_tokens,
            estimated_tokens,
        ));
    }

    decisions
}

pub(super) fn source_aware_budget_omission_decision(
    ledger: &ContextContributionLedger,
    strategy: &str,
    estimated_tokens: u32,
    budget_tokens: u32,
) -> SourceAwareBudgetOmissionDecision {
    let mut decisions = Vec::new();
    let mut omitted_sources = Vec::new();
    let mut remaining_over_budget = estimated_tokens.saturating_sub(budget_tokens);
    let max_omittable_entries = ledger.contributions().len().saturating_sub(1);

    for candidate in source_aware_omission_candidates(ledger) {
        if remaining_over_budget == 0 || omitted_sources.len() >= max_omittable_entries {
            break;
        }
        remaining_over_budget =
            remaining_over_budget.saturating_sub(candidate.contribution.estimated_tokens);
        omitted_sources.push(candidate.contribution.source.clone());
        decisions.push(budget_candidate_decision(
            candidate.contribution,
            TurnContextDecisionKind::Omitted {
                source_id: candidate.contribution.source_id.to_string(),
                priority: u32::from(candidate.priority),
                tokens: candidate.contribution.estimated_tokens,
            },
            strategy,
            budget_tokens,
            estimated_tokens,
        ));
    }

    SourceAwareBudgetOmissionDecision {
        omitted_sources,
        remaining_over_budget,
        decisions,
    }
}

pub(super) fn source_aware_omission_candidate_contributions(
    ledger: &ContextContributionLedger,
) -> Vec<&ContextContribution> {
    source_aware_omission_candidates(ledger)
        .into_iter()
        .map(|candidate| candidate.contribution)
        .collect()
}

fn source_aware_omission_candidates(
    ledger: &ContextContributionLedger,
) -> Vec<SourceAwareBudgetCandidate<'_>> {
    let mut candidates = ledger
        .contributions()
        .iter()
        .filter_map(|contribution| {
            source_aware_budget_candidate_priority(contribution).map(|priority| {
                SourceAwareBudgetCandidate {
                    contribution,
                    priority,
                }
            })
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        left.priority
            .cmp(&right.priority)
            .then_with(|| left.contribution.source.cmp(&right.contribution.source))
    });
    candidates
}

pub(in crate::context_manager::manifest) fn source_aware_budget_candidate_priority(
    contribution: &ContextContribution,
) -> Option<u8> {
    source_aware_omit_priority(contribution.tier, contribution.source_id)
}

pub(super) fn source_aware_truncation_candidate<'a>(
    ledger: &'a ContextContributionLedger,
    omitted_candidate_sources: &[String],
) -> Option<&'a ContextContribution> {
    ledger
        .contributions()
        .iter()
        .filter(|contribution| source_aware_budget_candidate_priority(contribution).is_some())
        .filter(|contribution| {
            !omitted_candidate_sources
                .iter()
                .any(|source| source == &contribution.source)
        })
        .max_by(|left, right| {
            left.estimated_tokens
                .cmp(&right.estimated_tokens)
                .then_with(|| right.source.cmp(&left.source))
        })
}

pub(super) fn budget_candidate_decision(
    contribution: &ContextContribution,
    decision_kind: TurnContextDecisionKind,
    strategy: &str,
    budget_tokens: u32,
    estimated_tokens: u32,
) -> TurnContextDecisionEntry {
    let decision = decision_kind.to_legacy_decision_string();
    let reason_hash = stable_turn_context_manifest_replay_hash(&format!(
        "{}:{}:{}:{}:{}:{}:{}:{}",
        strategy,
        contribution.slot,
        contribution.source_id,
        contribution.policy_class,
        contribution.estimated_tokens,
        estimated_tokens,
        budget_tokens,
        decision
    ));
    TurnContextDecisionEntry::from_kind(
        contribution.source.clone(),
        decision_kind,
        Some(reason_hash),
    )
}
