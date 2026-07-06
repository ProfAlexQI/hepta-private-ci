use crate::context_manager::budget_planner::adaptive_budget_allocations_for_pressure;
use codex_protocol::protocol::TurnContextAdaptiveBudgetAllocation;
use codex_protocol::protocol::TurnContextCompressionCandidate;
use codex_protocol::protocol::TurnContextCompressionStageKind;
use codex_protocol::protocol::TurnContextDecisionEntry;
use codex_protocol::protocol::TurnContextDecisionKind;
use codex_protocol::protocol::stable_turn_context_manifest_replay_hash;

use super::ContextContribution;
use super::ContextContributionLedger;
use super::classification::model_context_window_to_budget_tokens;

mod candidate;
mod compression;

use candidate::budget_candidate_decision;
use candidate::source_aware_budget_candidate_decisions;
#[cfg(test)]
pub(super) use candidate::source_aware_budget_candidate_priority;
use candidate::source_aware_budget_omission_decision;
use candidate::source_aware_truncation_candidate;
use compression::source_aware_compression_candidates;
use compression::source_aware_compression_plans;
pub(super) use compression::source_aware_tool_defragment_source;
pub(super) use compression::source_aware_tool_prune_source;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContextAssemblyPolicy {
    pub(super) budget_tokens: Option<u32>,
    pub(super) strategy: &'static str,
    pub(super) omit_low_priority_context: bool,
    pub(super) truncate_low_priority_context: bool,
    pub(super) summarize_retrieved_snippets: bool,
    pub(super) defragment_tool_context: bool,
    pub(super) prune_tool_context: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ContextAssemblyDecision {
    pub(super) budget_tokens: Option<u32>,
    pub(super) omitted_entries: u32,
    pub(super) omitted_sources: Vec<String>,
    pub(super) truncated: bool,
    pub(super) compression_candidates: Vec<TurnContextCompressionCandidate>,
    pub(super) adaptive_budget_allocations: Vec<TurnContextAdaptiveBudgetAllocation>,
    pub(super) decisions: Vec<TurnContextDecisionEntry>,
}

pub(super) struct ContextTruncationPlan<'a> {
    pub(super) contribution: &'a ContextContribution,
    pub(super) remaining_over_budget: u32,
}

pub(super) struct ContextCompressionPlan<'a> {
    pub(super) contribution: &'a ContextContribution,
    pub(super) kind: TurnContextCompressionStageKind,
}

impl Default for ContextAssemblyPolicy {
    fn default() -> Self {
        Self {
            budget_tokens: None,
            strategy: "non_omitting_replay_baseline",
            omit_low_priority_context: false,
            truncate_low_priority_context: false,
            summarize_retrieved_snippets: false,
            defragment_tool_context: false,
            prune_tool_context: false,
        }
    }
}

impl ContextAssemblyPolicy {
    pub(crate) fn from_model_context_window(model_context_window: Option<i64>) -> Self {
        Self {
            budget_tokens: model_context_window.and_then(model_context_window_to_budget_tokens),
            ..Self::default()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn source_aware_omission_for_model_context_window(
        model_context_window: Option<i64>,
    ) -> Self {
        Self {
            budget_tokens: model_context_window.and_then(model_context_window_to_budget_tokens),
            strategy: "source_aware_omission",
            omit_low_priority_context: true,
            truncate_low_priority_context: false,
            summarize_retrieved_snippets: false,
            defragment_tool_context: false,
            prune_tool_context: false,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn source_aware_omission_and_truncation_for_model_context_window(
        model_context_window: Option<i64>,
    ) -> Self {
        Self {
            budget_tokens: model_context_window.and_then(model_context_window_to_budget_tokens),
            strategy: "source_aware_omission_and_truncation",
            omit_low_priority_context: true,
            truncate_low_priority_context: true,
            summarize_retrieved_snippets: false,
            defragment_tool_context: false,
            prune_tool_context: false,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn source_aware_summary_for_model_context_window(
        model_context_window: Option<i64>,
    ) -> Self {
        Self {
            budget_tokens: model_context_window.and_then(model_context_window_to_budget_tokens),
            strategy: "source_aware_summary",
            omit_low_priority_context: false,
            truncate_low_priority_context: false,
            summarize_retrieved_snippets: true,
            defragment_tool_context: false,
            prune_tool_context: false,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn source_aware_tool_defragment_for_model_context_window(
        model_context_window: Option<i64>,
    ) -> Self {
        Self {
            budget_tokens: model_context_window.and_then(model_context_window_to_budget_tokens),
            strategy: "source_aware_tool_defragment",
            omit_low_priority_context: false,
            truncate_low_priority_context: false,
            summarize_retrieved_snippets: false,
            defragment_tool_context: true,
            prune_tool_context: false,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn source_aware_tool_prune_for_model_context_window(
        model_context_window: Option<i64>,
    ) -> Self {
        Self {
            budget_tokens: model_context_window.and_then(model_context_window_to_budget_tokens),
            strategy: "source_aware_tool_prune",
            omit_low_priority_context: false,
            truncate_low_priority_context: false,
            summarize_retrieved_snippets: false,
            defragment_tool_context: false,
            prune_tool_context: true,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn source_aware_compression_for_model_context_window(
        model_context_window: Option<i64>,
    ) -> Self {
        Self {
            budget_tokens: model_context_window.and_then(model_context_window_to_budget_tokens),
            strategy: "source_aware_compression",
            omit_low_priority_context: false,
            truncate_low_priority_context: false,
            summarize_retrieved_snippets: true,
            defragment_tool_context: true,
            prune_tool_context: true,
        }
    }

    pub(super) fn apply(&self, ledger: &ContextContributionLedger) -> ContextAssemblyDecision {
        let estimated_tokens = ledger.estimated_tokens();
        let budget_tokens = self.budget_tokens.or(Some(estimated_tokens));
        let budget_state = if budget_tokens.is_some_and(|budget| estimated_tokens > budget) {
            "budget_exceeded"
        } else {
            "within_budget"
        };
        let mut decisions = Vec::new();
        let reason_hash = stable_turn_context_manifest_replay_hash(&format!(
            "{}:{}:{}:{}",
            self.strategy,
            budget_state,
            estimated_tokens,
            budget_tokens.unwrap_or(0)
        ));
        decisions.push(TurnContextDecisionEntry::policy(
            "turn_context:assembly_policy",
            self.strategy,
            budget_state,
            Some(reason_hash),
        ));
        let mut compression_candidates = Vec::new();
        let mut adaptive_budget_allocations = Vec::new();
        if let Some(budget_tokens) = budget_tokens
            && estimated_tokens > budget_tokens
        {
            compression_candidates = source_aware_compression_candidates(ledger);
            adaptive_budget_allocations =
                adaptive_budget_allocations_for_pressure(ledger, estimated_tokens, budget_tokens);
            if self.omit_low_priority_context {
                let omission_decision = self.source_aware_budget_omission_decision(
                    ledger,
                    estimated_tokens,
                    budget_tokens,
                );
                let omitted_entries =
                    u32::try_from(omission_decision.omitted_sources.len()).unwrap_or(u32::MAX);
                let omitted_sources = omission_decision.omitted_sources;
                decisions.extend(omission_decision.decisions);
                if omission_decision.remaining_over_budget > 0
                    && let Some(candidate) =
                        source_aware_truncation_candidate(ledger, &omitted_sources)
                {
                    decisions.push(budget_candidate_decision(
                        candidate,
                        TurnContextDecisionKind::CandidateTruncate {
                            source_id: candidate.source_id.to_string(),
                            remaining_over_budget: omission_decision.remaining_over_budget,
                            tokens: candidate.estimated_tokens,
                        },
                        self.strategy,
                        budget_tokens,
                        estimated_tokens,
                    ));
                }

                return ContextAssemblyDecision {
                    budget_tokens: Some(budget_tokens),
                    omitted_entries,
                    omitted_sources,
                    truncated: false,
                    compression_candidates,
                    adaptive_budget_allocations,
                    decisions,
                };
            }

            decisions.extend(source_aware_budget_candidate_decisions(
                ledger,
                self.strategy,
                estimated_tokens,
                budget_tokens,
            ));
        }

        ContextAssemblyDecision {
            budget_tokens,
            omitted_entries: 0,
            omitted_sources: Vec::new(),
            truncated: false,
            compression_candidates,
            adaptive_budget_allocations,
            decisions,
        }
    }

    pub(super) fn truncation_candidate<'a>(
        &self,
        ledger: &'a ContextContributionLedger,
        omitted_sources: &[String],
    ) -> Option<ContextTruncationPlan<'a>> {
        if !self.truncate_low_priority_context {
            return None;
        }
        let budget_tokens = self.budget_tokens?;
        let estimated_after_omission =
            ledger.estimated_tokens_excluding(omitted_sources, None, &[]);
        if estimated_after_omission <= budget_tokens {
            return None;
        }
        let contribution = source_aware_truncation_candidate(ledger, omitted_sources)?;

        Some(ContextTruncationPlan {
            contribution,
            remaining_over_budget: estimated_after_omission.saturating_sub(budget_tokens),
        })
    }

    pub(super) fn compression_candidates<'a>(
        &self,
        ledger: &'a ContextContributionLedger,
        omitted_sources: &[String],
    ) -> Vec<ContextCompressionPlan<'a>> {
        if !self.summarize_retrieved_snippets
            && !self.defragment_tool_context
            && !self.prune_tool_context
        {
            return Vec::new();
        }
        let Some(budget_tokens) = self.budget_tokens else {
            return Vec::new();
        };
        let estimated_after_omission =
            ledger.estimated_tokens_excluding(omitted_sources, None, &[]);
        if estimated_after_omission <= budget_tokens {
            return Vec::new();
        }
        source_aware_compression_plans(
            ledger,
            omitted_sources,
            self.summarize_retrieved_snippets,
            self.defragment_tool_context,
            self.prune_tool_context,
        )
    }

    pub(super) fn requires_paired_rewrite(&self) -> bool {
        self.truncate_low_priority_context
            || self.summarize_retrieved_snippets
            || self.defragment_tool_context
            || self.prune_tool_context
    }

    fn source_aware_budget_omission_decision(
        &self,
        ledger: &ContextContributionLedger,
        estimated_tokens: u32,
        budget_tokens: u32,
    ) -> candidate::SourceAwareBudgetOmissionDecision {
        source_aware_budget_omission_decision(
            ledger,
            self.strategy,
            estimated_tokens,
            budget_tokens,
        )
    }
}

pub(super) fn source_is_omitted(omitted_sources: &[String], source: &str) -> bool {
    omitted_sources
        .iter()
        .any(|omitted_source| omitted_source == source)
}
