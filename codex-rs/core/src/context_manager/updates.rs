use crate::context::CollaborationModeInstructions;
use crate::context::ContextualUserFragment;
use crate::context::EnvironmentContext;
use crate::context::ModelSwitchInstructions;
use crate::context::PermissionsInstructions;
use crate::context::PersonalitySpecInstructions;
use crate::context::RealtimeEndInstructions;
use crate::context::RealtimeStartInstructions;
use crate::context::RealtimeStartWithInstructions;
use crate::context::UserInstructions;
use crate::session::PreviousTurnSettings;
use crate::session::turn_context::TurnContext;
use crate::shell::Shell;
use codex_execpolicy::Policy;
use codex_features::Feature;
use codex_protocol::config_types::Personality;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::openai_models::ModelInfo;
use codex_protocol::protocol::TurnContextItem;
use codex_protocol::protocol::stable_turn_context_manifest_replay_hash;

mod capability;
mod extension;

pub(crate) use capability::CapabilityContextSections;
use capability::build_capability_inventory_update_items;
pub(crate) use extension::ExtensionContextSections;
use extension::build_extension_context_update_items;

fn build_environment_update_item(
    previous: Option<&TurnContextItem>,
    next: &TurnContext,
    shell: &Shell,
) -> Option<String> {
    if !next.config.include_environment_context {
        return None;
    }

    let prev = previous?;
    let prev_context = EnvironmentContext::from_turn_context_item(prev, shell.name().to_string());
    let next_context = EnvironmentContext::from_turn_context(next, shell);
    if prev_context.equals_except_shell(&next_context) {
        return None;
    }

    Some(EnvironmentContext::diff_from_turn_context_item(prev, &next_context).render())
}

fn build_permissions_update_item(
    previous: Option<&TurnContextItem>,
    next: &TurnContext,
    exec_policy: &Policy,
) -> Option<String> {
    if !next.config.include_permissions_instructions {
        return None;
    }

    let prev = previous?;
    if prev.permission_profile() == next.permission_profile()
        && prev.approval_policy == next.approval_policy.value()
    {
        return None;
    }

    Some(
        PermissionsInstructions::from_permission_profile(
            &next.permission_profile,
            next.approval_policy.value(),
            next.config.approvals_reviewer,
            exec_policy,
            #[allow(deprecated)]
            &next.cwd,
            next.features.enabled(Feature::ExecPermissionApprovals),
            next.features.enabled(Feature::RequestPermissionsTool),
        )
        .render(),
    )
}

fn build_collaboration_mode_update_item(
    previous: Option<&TurnContextItem>,
    next: &TurnContext,
) -> Option<String> {
    if !next.config.include_collaboration_mode_instructions {
        return None;
    }

    let prev = previous?;
    if prev.collaboration_mode.as_ref() == Some(&next.collaboration_mode) {
        return None;
    }

    if let Some(next_instructions) =
        CollaborationModeInstructions::from_collaboration_mode(&next.collaboration_mode)
    {
        return Some(next_instructions.render());
    }

    prev.collaboration_mode
        .as_ref()
        .and_then(CollaborationModeInstructions::from_collaboration_mode)
        .map(|_| CollaborationModeInstructions::cleared().render())
}

pub(crate) fn build_realtime_update_item(
    previous: Option<&TurnContextItem>,
    previous_turn_settings: Option<&PreviousTurnSettings>,
    next: &TurnContext,
) -> Option<String> {
    match (
        previous.and_then(|item| item.realtime_active),
        next.realtime_active,
    ) {
        (Some(true), false) => Some(RealtimeEndInstructions::new("inactive").render()),
        (Some(false), true) | (None, true) => Some(
            if let Some(instructions) = next
                .config
                .experimental_realtime_start_instructions
                .as_deref()
            {
                RealtimeStartWithInstructions::new(instructions).render()
            } else {
                RealtimeStartInstructions.render()
            },
        ),
        (Some(true), true) | (Some(false), false) => None,
        (None, false) => previous_turn_settings
            .and_then(|settings| settings.realtime_active)
            .filter(|realtime_active| *realtime_active)
            .map(|_| RealtimeEndInstructions::new("inactive").render()),
    }
}

pub(crate) fn build_initial_realtime_item(
    previous: Option<&TurnContextItem>,
    previous_turn_settings: Option<&PreviousTurnSettings>,
    next: &TurnContext,
) -> Option<String> {
    build_realtime_update_item(previous, previous_turn_settings, next)
}

fn build_personality_update_item(
    previous: Option<&TurnContextItem>,
    next: &TurnContext,
    personality_feature_enabled: bool,
) -> Option<String> {
    if !personality_feature_enabled {
        return None;
    }
    let previous = previous?;
    if next.model_info.slug != previous.model {
        return None;
    }

    if let Some(personality) = next.personality
        && next.personality != previous.personality
    {
        let model_info = &next.model_info;
        let personality_message = personality_message_for(model_info, personality);
        personality_message.map(|message| PersonalitySpecInstructions::new(message).render())
    } else {
        None
    }
}

pub(crate) fn personality_message_for(
    model_info: &ModelInfo,
    personality: Personality,
) -> Option<String> {
    model_info
        .model_messages
        .as_ref()
        .and_then(|spec| spec.get_personality_message(Some(personality)))
        .filter(|message| !message.is_empty())
}

pub(crate) fn build_model_instructions_update_item(
    previous_turn_settings: Option<&PreviousTurnSettings>,
    next: &TurnContext,
) -> Option<String> {
    let previous_turn_settings = previous_turn_settings?;
    if previous_turn_settings.model == next.model_info.slug {
        return None;
    }

    let model_instructions = next.model_info.get_model_instructions(next.personality);
    if model_instructions.is_empty() {
        return None;
    }

    Some(ModelSwitchInstructions::new(model_instructions).render())
}

fn build_developer_instructions_update_item(
    previous: Option<&TurnContextItem>,
    next: &TurnContext,
) -> Option<String> {
    let prev = previous?;
    let previous_instructions = prev
        .developer_instructions
        .as_deref()
        .filter(|instructions| !instructions.is_empty());
    let next_instructions = next
        .developer_instructions
        .as_deref()
        .filter(|instructions| !instructions.is_empty());

    if previous_instructions == next_instructions {
        return None;
    }

    next_instructions.map_or_else(
        || {
            previous_instructions.map(|_| {
                "Developer instructions were cleared. Do not continue applying previously injected developer instructions.".to_string()
            })
        },
        |instructions| Some(instructions.to_string()),
    )
}

fn build_user_instructions_update_item(
    previous: Option<&TurnContextItem>,
    next: &TurnContext,
) -> Option<String> {
    let prev = previous?;
    let previous_instructions = prev
        .user_instructions
        .as_deref()
        .filter(|instructions| !instructions.is_empty());
    let next_instructions = next
        .user_instructions
        .as_deref()
        .filter(|instructions| !instructions.is_empty());

    if previous_instructions == next_instructions {
        return None;
    }

    let text = next_instructions.unwrap_or(
        "Workspace/user instructions were cleared. Do not continue applying previously injected workspace or user instructions.",
    );
    Some(
        UserInstructions {
            text: text.to_string(),
            #[allow(deprecated)]
            directory: next.cwd.to_string_lossy().into_owned(),
        }
        .render(),
    )
}

fn manifest_text_hash(text: &str) -> String {
    stable_turn_context_manifest_replay_hash(&format!("text:{text}\n"))
}

pub(crate) fn build_developer_update_item(text_sections: Vec<String>) -> Option<ResponseItem> {
    build_text_message("developer", text_sections)
}

pub(crate) fn build_contextual_user_message(text_sections: Vec<String>) -> Option<ResponseItem> {
    build_text_message("user", text_sections)
}

fn build_text_message(role: &str, text_sections: Vec<String>) -> Option<ResponseItem> {
    if text_sections.is_empty() {
        return None;
    }

    let content = text_sections
        .into_iter()
        .map(|text| ContentItem::InputText { text })
        .collect();

    Some(ResponseItem::Message {
        id: None,
        role: role.to_string(),
        content,
        phase: None,
    })
}

pub(crate) fn build_settings_update_items(
    previous: Option<&TurnContextItem>,
    previous_turn_settings: Option<&PreviousTurnSettings>,
    next: &TurnContext,
    shell: &Shell,
    exec_policy: &Policy,
    personality_feature_enabled: bool,
    capability_sections: CapabilityContextSections,
    extension_sections: ExtensionContextSections,
) -> Vec<ResponseItem> {
    // TODO(ccunningham): build_settings_update_items still does not cover every
    // model-visible item emitted by build_initial_context. Persist the remaining
    // inputs or add explicit replay events so fork/resume can diff everything
    // deterministically.
    let mut contextual_user_sections: Vec<String> = [
        build_user_instructions_update_item(previous, next),
        build_environment_update_item(previous, next, shell),
    ]
    .into_iter()
    .flatten()
    .collect();
    let mut developer_update_sections: Vec<String> = [
        // Keep model-switch instructions first so model-specific guidance is read before
        // any other context diffs on this turn.
        build_model_instructions_update_item(previous_turn_settings, next),
        build_permissions_update_item(previous, next, exec_policy),
        build_developer_instructions_update_item(previous, next),
        build_collaboration_mode_update_item(previous, next),
        build_realtime_update_item(previous, previous_turn_settings, next),
        build_personality_update_item(previous, next, personality_feature_enabled),
    ]
    .into_iter()
    .flatten()
    .collect();
    developer_update_sections.extend(build_capability_inventory_update_items(
        previous,
        capability_sections,
    ));
    let extension_update_sections =
        build_extension_context_update_items(previous, extension_sections);
    developer_update_sections.extend(extension_update_sections.developer);
    contextual_user_sections.extend(extension_update_sections.contextual_user);

    let mut items = Vec::with_capacity(2);
    if let Some(developer_message) = build_developer_update_item(developer_update_sections) {
        items.push(developer_message);
    }
    if let Some(contextual_user_message) = build_contextual_user_message(contextual_user_sections) {
        items.push(contextual_user_message);
    }
    items
}
