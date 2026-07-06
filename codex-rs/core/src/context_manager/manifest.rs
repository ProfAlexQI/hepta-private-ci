#[cfg(test)]
use crate::context::EXTENSION_CONTEXTUAL_USER_OPEN_TAG;
#[cfg(test)]
use crate::context::EXTENSION_DEVELOPER_CAPABILITIES_CLOSE_TAG;
#[cfg(test)]
use crate::context::EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG;
#[cfg(test)]
use crate::context::EXTENSION_DEVELOPER_POLICY_OPEN_TAG;
#[cfg(test)]
use crate::context::EXTENSION_SEPARATE_DEVELOPER_OPEN_TAG;
#[cfg(test)]
use crate::context_manager::budget_planner::estimated_compression_output_tokens;
use codex_extension_api::ExtensionData;
#[cfg(test)]
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
#[cfg(test)]
use codex_protocol::protocol::APPS_INSTRUCTIONS_OPEN_TAG;
#[cfg(test)]
use codex_protocol::protocol::COLLABORATION_MODE_OPEN_TAG;
#[cfg(test)]
use codex_protocol::protocol::ENVIRONMENT_CONTEXT_OPEN_TAG;
#[cfg(test)]
use codex_protocol::protocol::PLUGINS_INSTRUCTIONS_CLOSE_TAG;
#[cfg(test)]
use codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG;
#[cfg(test)]
use codex_protocol::protocol::REALTIME_CONVERSATION_OPEN_TAG;
#[cfg(test)]
use codex_protocol::protocol::SKILLS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::TURN_CONTEXT_MANIFEST_VERSION;
#[cfg(test)]
use codex_protocol::protocol::TurnContextBudgetAllocationAction;
#[cfg(test)]
use codex_protocol::protocol::TurnContextCompressionCandidateReason;
use codex_protocol::protocol::TurnContextCompressionLossCheckStatus;
use codex_protocol::protocol::TurnContextCompressionProtectedTierInvariant;
use codex_protocol::protocol::TurnContextCompressionStage;
#[cfg(test)]
use codex_protocol::protocol::TurnContextCompressionStageKind;
use codex_protocol::protocol::TurnContextDecisionEntry;
use codex_protocol::protocol::TurnContextDecisionKind;
use codex_protocol::protocol::TurnContextManifestItem;
#[cfg(test)]
use codex_protocol::protocol::TurnContextMemoryFormationCandidateType;
#[cfg(test)]
use codex_protocol::protocol::TurnContextMemoryFormationReceipt;
#[cfg(test)]
use codex_protocol::protocol::TurnContextMemoryTaxonomyBucket;
#[cfg(test)]
use codex_protocol::protocol::TurnContextMemoryTaxonomyClass;
#[cfg(test)]
use codex_protocol::protocol::TurnContextMemoryTemporalFact;
#[cfg(test)]
use codex_protocol::protocol::TurnContextMemoryTemporalFactType;
#[cfg(test)]
use codex_protocol::protocol::TurnContextRecallSelectedSnippetEnvelope;
#[cfg(test)]
use codex_protocol::protocol::TurnContextRecallSelectionSummary;
#[cfg(test)]
use codex_protocol::protocol::TurnContextTier;
use codex_protocol::protocol::stable_turn_context_manifest_replay_hash;

mod classification;
mod ledger;
mod options;
mod policy;
mod rewrite;
mod selected_recall;
mod selected_snippet;

#[cfg(test)]
use classification::manifest_content_text;
use ledger::ContextCompressionEvidence;
pub(crate) use ledger::ContextContribution;
pub(crate) use ledger::ContextContributionLedger;
use ledger::ContextTruncationEvidence;
pub(crate) use options::TurnContextManifestOptions;
use options::apply_turn_context_manifest_options;
pub(crate) use options::turn_context_manifest_options_from_extension_data;
use policy::ContextAssemblyDecision;
pub(crate) use policy::ContextAssemblyPolicy;
#[cfg(test)]
use policy::source_aware_budget_candidate_priority;
use rewrite::filter_context_items_by_omitted_sources;
use rewrite::rewrite_context_items_for_assembly;
#[allow(unused_imports)]
pub(crate) use selected_recall::ContextRecallProviderRollup;
pub(crate) use selected_recall::ContextRecallSelectedSnippetEnvelope;
#[cfg(test)]
use selected_snippet::LIVE_RECALL_SELECTED_SNIPPETS_FOOTER;
#[cfg(test)]
use selected_snippet::LIVE_RECALL_SELECTED_SNIPPETS_HEADER;
pub(crate) use selected_snippet::build_recall_selected_snippets_live_context_item;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum TurnContextAssemblyPolicyOptIn {
    SourceAwareCompression,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TurnContextAssemblyPolicyOptInGate {
    Disabled,
    SourceAwareCompressionCanary,
}

#[allow(dead_code)]
pub(crate) fn insert_source_aware_compression_policy_opt_in_marker(extension_data: &ExtensionData) {
    extension_data.insert(TurnContextAssemblyPolicyOptIn::SourceAwareCompression);
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ContextAssemblyResult {
    pub(crate) context_items: Vec<ResponseItem>,
    pub(crate) context_manifest: Option<TurnContextManifestItem>,
}

pub(crate) fn turn_context_assembly_policy_from_extension_data(
    extension_data: &ExtensionData,
    model_context_window: Option<i64>,
    opt_in_gate: TurnContextAssemblyPolicyOptInGate,
) -> ContextAssemblyPolicy {
    match (
        opt_in_gate,
        extension_data
            .get::<TurnContextAssemblyPolicyOptIn>()
            .as_deref(),
    ) {
        (
            TurnContextAssemblyPolicyOptInGate::SourceAwareCompressionCanary,
            Some(TurnContextAssemblyPolicyOptIn::SourceAwareCompression),
        ) => ContextAssemblyPolicy::source_aware_compression_for_model_context_window(
            model_context_window,
        ),
        (
            TurnContextAssemblyPolicyOptInGate::Disabled,
            Some(TurnContextAssemblyPolicyOptIn::SourceAwareCompression),
        )
        | (TurnContextAssemblyPolicyOptInGate::Disabled, None)
        | (TurnContextAssemblyPolicyOptInGate::SourceAwareCompressionCanary, None) => {
            ContextAssemblyPolicy::from_model_context_window(model_context_window)
        }
    }
}

#[cfg(test)]
pub(crate) fn build_turn_context_manifest(
    context_items: &[ResponseItem],
) -> Option<TurnContextManifestItem> {
    build_turn_context_manifest_with_policy(context_items, &ContextAssemblyPolicy::default())
}

pub(crate) fn build_turn_context_manifest_with_policy(
    context_items: &[ResponseItem],
    assembly_policy: &ContextAssemblyPolicy,
) -> Option<TurnContextManifestItem> {
    let ledger = ContextContributionLedger::from_response_items(context_items);
    let assembly_decision = assembly_policy.apply(&ledger);
    build_turn_context_manifest_from_ledger(&ledger, assembly_decision, None, &[])
}

fn build_turn_context_manifest_from_ledger(
    ledger: &ContextContributionLedger,
    mut assembly_decision: ContextAssemblyDecision,
    truncation: Option<&ContextTruncationEvidence>,
    compressions: &[ContextCompressionEvidence],
) -> Option<TurnContextManifestItem> {
    let entries = ledger.manifest_entries_excluding(
        &assembly_decision.omitted_sources,
        truncation,
        compressions,
    );

    (!entries.is_empty()).then(|| {
        let estimated_tokens = ledger.estimated_tokens_excluding(
            &assembly_decision.omitted_sources,
            truncation,
            compressions,
        );
        let mut decision_ledger =
            ledger.decision_entries_excluding(&assembly_decision.omitted_sources, truncation);
        if let Some(truncation) = truncation {
            assembly_decision.truncated = true;
            assembly_decision.decisions.retain(|entry| {
                !(entry.source == truncation.source && entry.kind().is_candidate_truncation())
            });
            decision_ledger.push(truncation_decision_entry(
                truncation,
                assembly_decision.budget_tokens,
            ));
        }
        let compression_stages = compressions
            .iter()
            .map(|compression| {
                assembly_decision
                    .compression_candidates
                    .retain(|candidate| {
                        !(candidate.source_id == compression.source_id
                            && candidate.kind == compression.kind)
                    });
                TurnContextCompressionStage {
                    kind: compression.kind,
                    input_tokens: compression.original_estimated_tokens,
                    output_tokens: compression.estimated_tokens,
                    affected_entries: 1,
                    loss_check_status: Some(
                        TurnContextCompressionLossCheckStatus::MarkerBoundaryOnly,
                    ),
                    rollback_source_text_hash: Some(compression.original_text_hash.clone()),
                    protected_tier_invariant: Some(
                        TurnContextCompressionProtectedTierInvariant::Preserved,
                    ),
                }
            })
            .collect::<Vec<_>>();
        decision_ledger.extend(assembly_decision.decisions);

        TurnContextManifestItem {
            version: TURN_CONTEXT_MANIFEST_VERSION,
            estimated_tokens,
            ledger_hash: None,
            budget_tokens: assembly_decision.budget_tokens,
            omitted_entries: assembly_decision.omitted_entries,
            omitted_sources: assembly_decision.omitted_sources,
            truncated: assembly_decision.truncated,
            decision_ledger_hash: None,
            decision_ledger,
            recall_selection: None,
            recall_selected_snippets: None,
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
            compression_candidates: assembly_decision.compression_candidates,
            adaptive_budget_allocations: assembly_decision.adaptive_budget_allocations,
            compression_stages,
            entries,
        }
        .with_refreshed_ledger_hash()
    })
}

fn truncation_decision_entry(
    truncation: &ContextTruncationEvidence,
    budget_tokens: Option<u32>,
) -> TurnContextDecisionEntry {
    let decision_kind = TurnContextDecisionKind::Truncated {
        source_id: truncation.source_id.to_string(),
        original_tokens: truncation.original_estimated_tokens,
        tokens: truncation.estimated_tokens,
    };
    let decision = decision_kind.to_legacy_decision_string();
    let reason_hash = stable_turn_context_manifest_replay_hash(&format!(
        "{}:{}:{}:{}:{}",
        truncation.source,
        truncation.source_id,
        truncation.text_hash,
        budget_tokens.unwrap_or(0),
        decision
    ));
    TurnContextDecisionEntry::from_kind(truncation.source.clone(), decision_kind, Some(reason_hash))
}

#[cfg(test)]
pub(crate) fn resolve_turn_context_manifest(
    context_items: &[ResponseItem],
    previous_manifest: Option<&TurnContextManifestItem>,
    options: &TurnContextManifestOptions,
) -> Option<TurnContextManifestItem> {
    resolve_turn_context_manifest_with_policy(
        context_items,
        previous_manifest,
        options,
        &ContextAssemblyPolicy::default(),
    )
}

pub(crate) fn resolve_turn_context_manifest_with_policy(
    context_items: &[ResponseItem],
    previous_manifest: Option<&TurnContextManifestItem>,
    options: &TurnContextManifestOptions,
    assembly_policy: &ContextAssemblyPolicy,
) -> Option<TurnContextManifestItem> {
    let mut manifest = build_turn_context_manifest_with_policy(context_items, assembly_policy)
        .or_else(|| previous_manifest.cloned())?;
    apply_turn_context_manifest_options(&mut manifest, options);
    Some(manifest)
}

pub(crate) fn assemble_turn_context_with_policy(
    context_items: &[ResponseItem],
    previous_manifest: Option<&TurnContextManifestItem>,
    options: &TurnContextManifestOptions,
    assembly_policy: &ContextAssemblyPolicy,
) -> ContextAssemblyResult {
    if assembly_policy.requires_paired_rewrite() {
        return assemble_turn_context_with_rewrites(
            context_items,
            previous_manifest,
            options,
            assembly_policy,
        );
    }

    let context_manifest = resolve_turn_context_manifest_with_policy(
        context_items,
        previous_manifest,
        options,
        assembly_policy,
    );
    let filtered_context_items = context_manifest
        .as_ref()
        .map(|manifest| {
            filter_context_items_by_omitted_sources(context_items, &manifest.omitted_sources)
        })
        .unwrap_or_else(|| context_items.to_vec());

    ContextAssemblyResult {
        context_items: filtered_context_items,
        context_manifest,
    }
}

fn assemble_turn_context_with_rewrites(
    context_items: &[ResponseItem],
    previous_manifest: Option<&TurnContextManifestItem>,
    options: &TurnContextManifestOptions,
    assembly_policy: &ContextAssemblyPolicy,
) -> ContextAssemblyResult {
    let ledger = ContextContributionLedger::from_response_items(context_items);
    if ledger.contributions().is_empty() {
        return ContextAssemblyResult {
            context_items: context_items.to_vec(),
            context_manifest: previous_manifest.cloned(),
        };
    }

    let assembly_decision = assembly_policy.apply(&ledger);
    let compression_candidates =
        assembly_policy.compression_candidates(&ledger, &assembly_decision.omitted_sources);
    let truncation_candidate =
        assembly_policy.truncation_candidate(&ledger, &assembly_decision.omitted_sources);
    let rewrite_result = rewrite_context_items_for_assembly(
        context_items,
        &assembly_decision.omitted_sources,
        truncation_candidate.as_ref(),
        &compression_candidates,
    );
    let mut context_manifest = build_turn_context_manifest_from_ledger(
        &ledger,
        assembly_decision,
        rewrite_result.truncation.as_ref(),
        &rewrite_result.compressions,
    )
    .or_else(|| previous_manifest.cloned());
    if let Some(manifest) = context_manifest.as_mut() {
        apply_turn_context_manifest_options(manifest, options);
    }

    ContextAssemblyResult {
        context_items: rewrite_result.context_items,
        context_manifest,
    }
}

#[cfg(test)]
mod tests;
