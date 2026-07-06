use crate::context_manager::manifest::ContextContribution;
use crate::context_manager::manifest::ContextContributionLedger;
use crate::context_manager::source_registry::context_source_registry_entries;
use crate::context_manager::source_registry::source_aware_omit_priority;
use codex_protocol::protocol::TurnContextAdaptiveBudgetAllocation;
use codex_protocol::protocol::TurnContextBudgetAllocationAction;
use codex_protocol::protocol::TurnContextCompressionStageKind;

#[derive(Clone, Debug)]
struct AdaptiveBudgetSource {
    source_id: &'static str,
    tier: codex_protocol::protocol::TurnContextTier,
    budget_class: &'static str,
    input_tokens: u32,
    reserve_tokens: u32,
    proposed_budget_tokens: u32,
    omit_priority: Option<u8>,
    compression_kind: Option<TurnContextCompressionStageKind>,
    estimated_compressed_tokens: Option<u32>,
    current_heuristic_action: TurnContextBudgetAllocationAction,
}

pub(crate) fn adaptive_budget_allocations_for_pressure(
    ledger: &ContextContributionLedger,
    estimated_tokens: u32,
    budget_tokens: u32,
) -> Vec<TurnContextAdaptiveBudgetAllocation> {
    let heuristic_drop_source_ids =
        current_heuristic_drop_source_ids(ledger, estimated_tokens, budget_tokens);
    let mut sources = context_source_registry_entries()
        .iter()
        .filter_map(|registry_entry| {
            let input_tokens = ledger
                .contributions()
                .iter()
                .filter(|contribution| contribution.source_id == registry_entry.source_id)
                .fold(0_u32, |tokens, contribution| {
                    tokens.saturating_add(contribution.estimated_tokens)
                });
            if input_tokens == 0 {
                return None;
            }

            let compression_kind = registry_entry.default_compression_kind();
            let estimated_compressed_tokens = compression_kind
                .map(|kind| estimated_compression_output_tokens(kind, input_tokens));
            let reserve_tokens = if registry_entry.omit_priority.is_none() {
                input_tokens
            } else {
                estimated_compressed_tokens.unwrap_or(0).min(input_tokens)
            };
            let current_heuristic_action =
                if heuristic_drop_source_ids.contains(&registry_entry.source_id) {
                    TurnContextBudgetAllocationAction::Drop
                } else if compression_kind.is_some() {
                    TurnContextBudgetAllocationAction::Compress
                } else {
                    TurnContextBudgetAllocationAction::Keep
                };

            Some(AdaptiveBudgetSource {
                source_id: registry_entry.source_id,
                tier: registry_entry.tier,
                budget_class: registry_entry.budget_class.as_str(),
                input_tokens,
                reserve_tokens,
                proposed_budget_tokens: if registry_entry.omit_priority.is_none() {
                    reserve_tokens
                } else {
                    0
                },
                omit_priority: registry_entry.omit_priority,
                compression_kind,
                estimated_compressed_tokens,
                current_heuristic_action,
            })
        })
        .collect::<Vec<_>>();

    let reserved_tokens = sources.iter().fold(0_u32, |tokens, source| {
        tokens.saturating_add(source.proposed_budget_tokens)
    });
    let mut remaining_tokens = budget_tokens.saturating_sub(reserved_tokens);
    let mut flexible_indexes = sources
        .iter()
        .enumerate()
        .filter_map(|(index, source)| source.omit_priority.map(|priority| (index, priority)))
        .collect::<Vec<_>>();
    flexible_indexes.sort_by(|left, right| {
        left.1
            .cmp(&right.1)
            .then_with(|| sources[left.0].source_id.cmp(sources[right.0].source_id))
    });

    for (index, _) in flexible_indexes.iter().copied() {
        if remaining_tokens == 0 {
            break;
        }
        let source = &mut sources[index];
        let granted_tokens = remaining_tokens.min(source.reserve_tokens);
        source.proposed_budget_tokens =
            source.proposed_budget_tokens.saturating_add(granted_tokens);
        remaining_tokens = remaining_tokens.saturating_sub(granted_tokens);
    }

    for (index, _) in flexible_indexes {
        if remaining_tokens == 0 {
            break;
        }
        let source = &mut sources[index];
        let wanted_tokens = source
            .input_tokens
            .saturating_sub(source.proposed_budget_tokens);
        let granted_tokens = remaining_tokens.min(wanted_tokens);
        source.proposed_budget_tokens =
            source.proposed_budget_tokens.saturating_add(granted_tokens);
        remaining_tokens = remaining_tokens.saturating_sub(granted_tokens);
    }

    sources
        .into_iter()
        .map(|source| {
            let overflow_tokens = source
                .input_tokens
                .saturating_sub(source.proposed_budget_tokens);
            let would_drop = source.proposed_budget_tokens == 0 && overflow_tokens > 0;
            let would_compress =
                !would_drop && source.compression_kind.is_some() && overflow_tokens > 0;
            let proposed_action = if would_drop {
                TurnContextBudgetAllocationAction::Drop
            } else if would_compress {
                TurnContextBudgetAllocationAction::Compress
            } else {
                TurnContextBudgetAllocationAction::Keep
            };

            TurnContextAdaptiveBudgetAllocation {
                tier: source.tier,
                source_id: source.source_id.to_string(),
                budget_class: source.budget_class.to_string(),
                input_tokens: source.input_tokens,
                reserve_tokens: source.reserve_tokens,
                proposed_budget_tokens: source.proposed_budget_tokens,
                overflow_tokens,
                omit_priority: source.omit_priority.map(u32::from),
                compression_kind: source.compression_kind,
                estimated_compressed_tokens: source.estimated_compressed_tokens,
                current_heuristic_action: source.current_heuristic_action,
                proposed_action,
                would_drop,
                would_compress,
            }
        })
        .collect()
}

pub(crate) fn estimated_compression_output_tokens(
    kind: TurnContextCompressionStageKind,
    input_tokens: u32,
) -> u32 {
    if input_tokens <= 1 {
        return input_tokens;
    }
    let numerator = match kind {
        TurnContextCompressionStageKind::Summary => 40,
        TurnContextCompressionStageKind::Prune => 50,
        TurnContextCompressionStageKind::Defragment => 70,
        TurnContextCompressionStageKind::Rewrite => 75,
        TurnContextCompressionStageKind::Unknown => return input_tokens,
    };
    let rounded = (u64::from(input_tokens) * numerator).div_ceil(100);
    u32::try_from(rounded)
        .unwrap_or(u32::MAX)
        .clamp(1, input_tokens.saturating_sub(1))
}

fn current_heuristic_drop_source_ids(
    ledger: &ContextContributionLedger,
    estimated_tokens: u32,
    budget_tokens: u32,
) -> Vec<&'static str> {
    let mut source_ids = Vec::new();
    let mut remaining_over_budget = estimated_tokens.saturating_sub(budget_tokens);
    for contribution in current_heuristic_omission_candidates(ledger) {
        if remaining_over_budget == 0 {
            break;
        }
        remaining_over_budget = remaining_over_budget.saturating_sub(contribution.estimated_tokens);
        if !source_ids.contains(&contribution.source_id) {
            source_ids.push(contribution.source_id);
        }
    }
    source_ids
}

fn current_heuristic_omission_candidates(
    ledger: &ContextContributionLedger,
) -> Vec<&ContextContribution> {
    let mut candidates = ledger
        .contributions()
        .iter()
        .filter(|contribution| source_aware_budget_candidate_priority(contribution).is_some())
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        source_aware_budget_candidate_priority(left)
            .cmp(&source_aware_budget_candidate_priority(right))
            .then_with(|| left.source.cmp(&right.source))
    });
    candidates
}

fn source_aware_budget_candidate_priority(contribution: &ContextContribution) -> Option<u8> {
    source_aware_omit_priority(contribution.tier, contribution.source_id)
}
