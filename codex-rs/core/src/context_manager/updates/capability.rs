use codex_protocol::protocol::APPS_INSTRUCTIONS_CLOSE_TAG;
use codex_protocol::protocol::APPS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::PLUGINS_INSTRUCTIONS_CLOSE_TAG;
use codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::SKILLS_INSTRUCTIONS_CLOSE_TAG;
use codex_protocol::protocol::SKILLS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::TurnContextItem;
use codex_protocol::protocol::TurnContextManifestItem;

use super::manifest_text_hash;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct CapabilityContextSections {
    pub(crate) apps: Option<String>,
    pub(crate) available_skills: Option<String>,
    pub(crate) available_plugins: Option<String>,
}

pub(super) fn build_capability_inventory_update_items(
    previous: Option<&TurnContextItem>,
    next: CapabilityContextSections,
) -> Vec<String> {
    let Some(previous_manifest) = previous.and_then(|item| item.context_manifest.as_ref()) else {
        return Vec::new();
    };

    [
        CapabilityContextUpdateSpec {
            source_id: "apps",
            next_text: next.apps,
            clear_text: capability_inventory_clear_text(
                APPS_INSTRUCTIONS_OPEN_TAG,
                APPS_INSTRUCTIONS_CLOSE_TAG,
                "Apps/connectors",
            ),
        },
        CapabilityContextUpdateSpec {
            source_id: "available_skills",
            next_text: next.available_skills,
            clear_text: capability_inventory_clear_text(
                SKILLS_INSTRUCTIONS_OPEN_TAG,
                SKILLS_INSTRUCTIONS_CLOSE_TAG,
                "Available skills",
            ),
        },
        CapabilityContextUpdateSpec {
            source_id: "available_plugins",
            next_text: next.available_plugins,
            clear_text: capability_inventory_clear_text(
                PLUGINS_INSTRUCTIONS_OPEN_TAG,
                PLUGINS_INSTRUCTIONS_CLOSE_TAG,
                "Available plugins",
            ),
        },
    ]
    .into_iter()
    .filter_map(|spec| build_capability_inventory_update_item(previous_manifest, spec))
    .collect()
}

struct CapabilityContextUpdateSpec {
    source_id: &'static str,
    next_text: Option<String>,
    clear_text: String,
}

fn build_capability_inventory_update_item(
    previous_manifest: &TurnContextManifestItem,
    spec: CapabilityContextUpdateSpec,
) -> Option<String> {
    let previous_hash = previous_capability_context_hash(previous_manifest, spec.source_id);
    let clear_hash = manifest_text_hash(&spec.clear_text);
    match (previous_hash, spec.next_text) {
        (Some(previous_hash), None) if previous_hash == clear_hash => None,
        (Some(_), None) => Some(spec.clear_text),
        (Some(previous_hash), Some(next_text))
            if previous_hash == manifest_text_hash(&next_text) =>
        {
            None
        }
        (Some(_), Some(next_text)) => Some(next_text),
        (None, Some(_)) | (None, None) => None,
    }
}

fn previous_capability_context_hash(
    previous_manifest: &TurnContextManifestItem,
    source_id: &str,
) -> Option<String> {
    let source_prefix = format!("turn_context:developer:{source_id}:");
    previous_manifest
        .entries
        .iter()
        .find(|entry| entry.source.starts_with(&source_prefix))
        .map(|entry| entry.text_hash.clone())
}

fn capability_inventory_clear_text(open_tag: &str, close_tag: &str, label: &str) -> String {
    format!(
        "{open_tag}\n{label} capability inventory was cleared. Do not continue assuming previously injected {label} capabilities are available.\n{close_tag}"
    )
}
