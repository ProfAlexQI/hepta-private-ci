use codex_extension_api::ExtensionData;
use codex_protocol::protocol::TurnContextManifestItem;
use codex_protocol::protocol::TurnContextMemoryFormationReceipt;
use codex_protocol::protocol::TurnContextMemoryTaxonomyBucket;
use codex_protocol::protocol::TurnContextMemoryTemporalFact;

use super::selected_recall::ContextRecallProviderRollup;
use super::selected_recall::ContextRecallSelectedSnippetEnvelope;
use super::selected_recall::SelectedRecallControllerDecision;
use super::selected_recall::apply_selected_recall_controller_decision;
use super::selected_recall::selected_recall_controller_decision_from_extension_data;

#[derive(Clone, Debug, Default)]
pub(crate) struct TurnContextManifestOptions {
    pub(crate) recall_provider_rollup: Option<ContextRecallProviderRollup>,
    pub(crate) recall_selected_snippets: Option<ContextRecallSelectedSnippetEnvelope>,
    pub(crate) memory_taxonomy: Vec<TurnContextMemoryTaxonomyBucket>,
    pub(crate) memory_formation_receipts: Vec<TurnContextMemoryFormationReceipt>,
    pub(crate) memory_temporal_facts: Vec<TurnContextMemoryTemporalFact>,
}

pub(crate) fn turn_context_manifest_options_from_extension_data(
    extension_data: &ExtensionData,
) -> TurnContextManifestOptions {
    let selected_recall = selected_recall_controller_decision_from_extension_data(extension_data);
    TurnContextManifestOptions {
        recall_provider_rollup: selected_recall.recall_provider_rollup,
        recall_selected_snippets: selected_recall.recall_selected_snippets,
        memory_taxonomy: extension_data
            .get::<Vec<TurnContextMemoryTaxonomyBucket>>()
            .filter(|buckets| {
                buckets
                    .iter()
                    .all(TurnContextMemoryTaxonomyBucket::has_payload_light_integrity)
            })
            .map(|buckets| (*buckets).clone())
            .unwrap_or_default(),
        memory_formation_receipts: extension_data
            .get::<Vec<TurnContextMemoryFormationReceipt>>()
            .filter(|receipts| {
                receipts
                    .iter()
                    .all(TurnContextMemoryFormationReceipt::has_payload_light_integrity)
            })
            .map(|receipts| (*receipts).clone())
            .unwrap_or_default(),
        memory_temporal_facts: extension_data
            .get::<Vec<TurnContextMemoryTemporalFact>>()
            .filter(|facts| {
                facts
                    .iter()
                    .all(TurnContextMemoryTemporalFact::has_payload_light_integrity)
            })
            .map(|facts| (*facts).clone())
            .unwrap_or_default(),
    }
}

pub(super) fn apply_turn_context_manifest_options(
    manifest: &mut TurnContextManifestItem,
    options: &TurnContextManifestOptions,
) {
    apply_selected_recall_controller_decision(
        manifest,
        &SelectedRecallControllerDecision {
            recall_provider_rollup: options.recall_provider_rollup.clone(),
            recall_selected_snippets: options.recall_selected_snippets.clone(),
            ..SelectedRecallControllerDecision::default()
        },
    );
    if !options.memory_taxonomy.is_empty()
        && options
            .memory_taxonomy
            .iter()
            .all(TurnContextMemoryTaxonomyBucket::has_payload_light_integrity)
    {
        manifest.memory_taxonomy = options.memory_taxonomy.clone();
        manifest.refresh_ledger_hash();
    }
    if !options.memory_formation_receipts.is_empty()
        && options
            .memory_formation_receipts
            .iter()
            .all(TurnContextMemoryFormationReceipt::has_payload_light_integrity)
    {
        manifest.memory_formation_receipts = options.memory_formation_receipts.clone();
        manifest.refresh_ledger_hash();
    }
    if !options.memory_temporal_facts.is_empty()
        && options
            .memory_temporal_facts
            .iter()
            .all(TurnContextMemoryTemporalFact::has_payload_light_integrity)
    {
        manifest.memory_temporal_facts = options.memory_temporal_facts.clone();
        manifest.refresh_ledger_hash();
    }
}
