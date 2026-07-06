use crate::context::ExtensionPromptFragment;
use crate::context::ExtensionPromptSlot;
use codex_protocol::protocol::TurnContextItem;
use codex_protocol::protocol::TurnContextManifestItem;

use super::manifest_text_hash;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct ExtensionContextSections {
    pub(crate) developer_policy: Vec<String>,
    pub(crate) developer_capabilities: Vec<String>,
    pub(crate) separate_developer: Vec<String>,
    pub(crate) contextual_user: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(super) struct ExtensionContextUpdateSections {
    pub(super) developer: Vec<String>,
    pub(super) contextual_user: Vec<String>,
}

pub(super) fn build_extension_context_update_items(
    previous: Option<&TurnContextItem>,
    next: ExtensionContextSections,
) -> ExtensionContextUpdateSections {
    let Some(previous_manifest) = previous.and_then(|item| item.context_manifest.as_ref()) else {
        return ExtensionContextUpdateSections::default();
    };

    let mut updates = ExtensionContextUpdateSections::default();
    for spec in [
        ExtensionContextUpdateSpec {
            slot: ExtensionPromptSlot::DeveloperPolicy,
            source_slot: "developer",
            next_texts: next.developer_policy,
        },
        ExtensionContextUpdateSpec {
            slot: ExtensionPromptSlot::DeveloperCapabilities,
            source_slot: "developer",
            next_texts: next.developer_capabilities,
        },
        ExtensionContextUpdateSpec {
            slot: ExtensionPromptSlot::SeparateDeveloper,
            source_slot: "developer",
            next_texts: next.separate_developer,
        },
        ExtensionContextUpdateSpec {
            slot: ExtensionPromptSlot::ContextualUser,
            source_slot: "contextual_user",
            next_texts: next.contextual_user,
        },
    ] {
        let target = if spec.source_slot == "contextual_user" {
            &mut updates.contextual_user
        } else {
            &mut updates.developer
        };
        target.extend(build_extension_context_update_item(previous_manifest, spec));
    }
    updates
}

struct ExtensionContextUpdateSpec {
    slot: ExtensionPromptSlot,
    source_slot: &'static str,
    next_texts: Vec<String>,
}

fn build_extension_context_update_item(
    previous_manifest: &TurnContextManifestItem,
    spec: ExtensionContextUpdateSpec,
) -> Vec<String> {
    let previous_hashes =
        previous_extension_context_hashes(previous_manifest, spec.source_slot, spec.slot);
    if previous_hashes.is_empty() {
        return Vec::new();
    }

    let clear_text = ExtensionPromptFragment::cleared(spec.slot).render();
    let clear_hashes = vec![manifest_text_hash(&clear_text)];
    if spec.next_texts.is_empty() {
        return (previous_hashes != clear_hashes)
            .then_some(vec![clear_text])
            .unwrap_or_default();
    }

    let next_hashes = spec
        .next_texts
        .iter()
        .map(|text| manifest_text_hash(text))
        .collect::<Vec<_>>();
    if previous_hashes == next_hashes {
        Vec::new()
    } else {
        spec.next_texts
    }
}

fn previous_extension_context_hashes(
    previous_manifest: &TurnContextManifestItem,
    source_slot: &str,
    prompt_slot: ExtensionPromptSlot,
) -> Vec<String> {
    let source_prefix = format!("turn_context:{source_slot}:{}:", prompt_slot.source_id());
    previous_manifest
        .entries
        .iter()
        .filter(|entry| entry.source.starts_with(&source_prefix))
        .map(|entry| entry.text_hash.clone())
        .collect()
}
