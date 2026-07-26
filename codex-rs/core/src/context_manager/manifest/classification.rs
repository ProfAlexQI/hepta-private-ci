use crate::context::EXTENSION_CONTEXTUAL_USER_OPEN_TAG;
use crate::context::EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG;
use crate::context::EXTENSION_DEVELOPER_POLICY_OPEN_TAG;
use crate::context::EXTENSION_SEPARATE_DEVELOPER_OPEN_TAG;
use crate::context_manager::estimate_response_item_model_visible_bytes;
use crate::context_manager::source_registry::context_source_registry_entry;
use crate::session::multi_agents;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::APPS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::COLLABORATION_MODE_OPEN_TAG;
use codex_protocol::protocol::ENVIRONMENT_CONTEXT_OPEN_TAG;
use codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::REALTIME_CONVERSATION_OPEN_TAG;
use codex_protocol::protocol::SKILLS_INSTRUCTIONS_OPEN_TAG;
use codex_protocol::protocol::TurnContextTier;
use codex_utils_output_truncation::approx_tokens_from_byte_count_i64;

use super::selected_snippet::LIVE_RECALL_SELECTED_SNIPPETS_HEADER;

pub(super) fn manifest_content_identity(content_item: &ContentItem) -> Option<String> {
    let mut identity = String::new();
    match content_item {
        ContentItem::InputText { text } | ContentItem::OutputText { text } => {
            identity.push_str("text:");
            identity.push_str(text);
            identity.push('\n');
        }
        other => {
            identity.push_str("content:");
            identity.push_str(&serde_json::to_string(other).unwrap_or_default());
            identity.push('\n');
        }
    }

    (!identity.is_empty()).then_some(identity)
}

pub(super) fn source_role(role: &str) -> &str {
    match role {
        "user" => "contextual_user",
        "developer" => "developer",
        other => other,
    }
}

pub(super) fn contribution_source(
    slot: &str,
    source_id: &str,
    item_index: usize,
    content_index: usize,
    content_len: usize,
) -> String {
    if content_len == 1 {
        format!("turn_context:{slot}:{source_id}:{item_index}")
    } else {
        format!("turn_context:{slot}:{source_id}:{item_index}:{content_index}")
    }
}

pub(super) fn model_context_window_to_budget_tokens(model_context_window: i64) -> Option<u32> {
    if model_context_window <= 0 {
        return None;
    }
    Some(u32::try_from(model_context_window).unwrap_or(u32::MAX))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ContributionClassification {
    pub(super) source_id: &'static str,
    pub(super) policy_class: &'static str,
    pub(super) include_reason: &'static str,
    pub(super) tier: TurnContextTier,
}

fn registered_contribution_classification(
    source_id: &'static str,
    policy_class: &'static str,
    include_reason: &'static str,
) -> ContributionClassification {
    let registry_entry = match context_source_registry_entry(source_id) {
        Some(registry_entry) => registry_entry,
        None => panic!("context source registry must cover manifest classifier source"),
    };
    ContributionClassification {
        source_id: registry_entry.source_id,
        policy_class,
        include_reason,
        tier: registry_entry.tier,
    }
}

pub(super) fn classify_contribution(
    slot: &str,
    content_item: &ContentItem,
) -> ContributionClassification {
    let Some(text) = manifest_content_text(content_item) else {
        return registered_contribution_classification(
            "non_text_content",
            "always_include_context",
            "model_visible_non_text_context_fragment",
        );
    };
    let trimmed = text.trim_start();

    if trimmed.starts_with("<model_switch>") {
        return registered_contribution_classification(
            "model_switch",
            "session_state_diff",
            "model_instructions_changed",
        );
    }
    if trimmed.starts_with(multi_agents::MULTI_AGENT_USAGE_HINT_OPEN_TAG) {
        return registered_contribution_classification(
            multi_agents::MULTI_AGENT_USAGE_HINT_SOURCE_ID,
            "always_include_developer",
            "multi_agent_usage_hint",
        );
    }
    if trimmed.starts_with("<permissions instructions>") {
        return registered_contribution_classification(
            "permissions",
            "always_include_safety_policy",
            "permission_profile_and_approval_policy",
        );
    }
    if trimmed.starts_with(COLLABORATION_MODE_OPEN_TAG) {
        return registered_contribution_classification(
            "collaboration_mode",
            "always_include_developer",
            "collaboration_mode_instructions",
        );
    }
    if trimmed.starts_with(REALTIME_CONVERSATION_OPEN_TAG) {
        return registered_contribution_classification("realtime", "turn_state", "realtime_state");
    }
    if trimmed.starts_with("<personality_spec>") {
        return registered_contribution_classification(
            "personality",
            "model_behavior",
            "personality_setting",
        );
    }
    if trimmed.starts_with(APPS_INSTRUCTIONS_OPEN_TAG) {
        return registered_contribution_classification(
            "apps",
            "capability_inventory",
            "enabled_apps",
        );
    }
    if trimmed.starts_with(SKILLS_INSTRUCTIONS_OPEN_TAG) {
        return registered_contribution_classification(
            "available_skills",
            "capability_inventory",
            "available_skills",
        );
    }
    if trimmed.starts_with(PLUGINS_INSTRUCTIONS_OPEN_TAG) {
        return registered_contribution_classification(
            "available_plugins",
            "capability_inventory",
            "available_plugins",
        );
    }
    if trimmed.starts_with(EXTENSION_DEVELOPER_POLICY_OPEN_TAG) {
        return registered_contribution_classification(
            "extension_developer_policy",
            "extension_prompt",
            "extension_developer_policy",
        );
    }
    if trimmed.starts_with(EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG) {
        return registered_contribution_classification(
            "extension_developer_capabilities",
            "extension_prompt",
            "extension_developer_capabilities",
        );
    }
    if trimmed.starts_with(EXTENSION_SEPARATE_DEVELOPER_OPEN_TAG) {
        return registered_contribution_classification(
            "extension_separate_developer",
            "extension_prompt",
            "extension_separate_developer",
        );
    }
    if trimmed.starts_with(EXTENSION_CONTEXTUAL_USER_OPEN_TAG) {
        return registered_contribution_classification(
            "extension_contextual_user",
            "extension_prompt",
            "extension_contextual_user",
        );
    }
    if trimmed.starts_with(ENVIRONMENT_CONTEXT_OPEN_TAG) {
        return registered_contribution_classification(
            "environment",
            "turn_environment",
            "runtime_environment",
        );
    }
    if trimmed.starts_with("# AGENTS.md instructions for ")
        || trimmed.starts_with("<user_instructions>")
    {
        return registered_contribution_classification(
            "user_instructions",
            "always_include_contextual_user",
            "workspace_user_instructions",
        );
    }
    if trimmed.starts_with(LIVE_RECALL_SELECTED_SNIPPETS_HEADER) {
        return registered_contribution_classification(
            "selected_context_recall",
            "bounded_recall",
            "selected_snippet_shadow_handoff",
        );
    }

    match slot {
        "developer" => registered_contribution_classification(
            "developer_instructions",
            "always_include_developer",
            "developer_or_extension_context",
        ),
        "contextual_user" => registered_contribution_classification(
            "contextual_user",
            "always_include_contextual_user",
            "contextual_user_fragment",
        ),
        _ => registered_contribution_classification(
            "context",
            "always_include_context",
            "model_visible_turn_context_fragment",
        ),
    }
}

pub(super) fn estimate_manifest_content_tokens(role: &str, content_item: &ContentItem) -> u32 {
    let item = ResponseItem::Message {
        id: None,
        role: role.to_string(),
        content: vec![content_item.clone()],
        phase: None,
    };
    let estimated_bytes = estimate_response_item_model_visible_bytes(&item).max(0);
    let estimated_tokens = approx_tokens_from_byte_count_i64(estimated_bytes).max(0);
    u32::try_from(estimated_tokens).unwrap_or(u32::MAX)
}

pub(super) fn manifest_content_text(content_item: &ContentItem) -> Option<&str> {
    match content_item {
        ContentItem::InputText { text } | ContentItem::OutputText { text } => Some(text.as_str()),
        _ => None,
    }
}
