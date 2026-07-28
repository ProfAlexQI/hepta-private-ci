use super::*;
use crate::ModelsManagerConfig;
use codex_protocol::openai_models::ModelTokenBudgetConfig;
use pretty_assertions::assert_eq;

fn model_token_budget() -> ModelTokenBudgetConfig {
    ModelTokenBudgetConfig {
        reminder_threshold_tokens: 6_144,
        reminder_message_template: "Wrap up with {n_remaining} tokens left.".to_string(),
        guidance_message: "Preserve durable state before rollover.".to_string(),
        auto_compact_fallback_prompt: "Record the remaining state.".to_string(),
        auto_compact_fallback_buffer_tokens: 16_384,
    }
}

#[test]
fn reasoning_summaries_override_true_enables_support() {
    let model = model_info_from_slug("unknown-model");
    let config = ModelsManagerConfig {
        model_supports_reasoning_summaries: Some(true),
        ..Default::default()
    };

    let updated = with_config_overrides(model.clone(), &config);
    let mut expected = model;
    expected.supports_reasoning_summaries = true;

    assert_eq!(updated, expected);
}

#[test]
fn reasoning_summaries_override_false_does_not_disable_support() {
    let mut model = model_info_from_slug("unknown-model");
    model.supports_reasoning_summaries = true;
    let config = ModelsManagerConfig {
        model_supports_reasoning_summaries: Some(false),
        ..Default::default()
    };

    let updated = with_config_overrides(model.clone(), &config);

    assert_eq!(updated, model);
}

#[test]
fn reasoning_summaries_override_false_is_noop_when_model_is_false() {
    let model = model_info_from_slug("unknown-model");
    let config = ModelsManagerConfig {
        model_supports_reasoning_summaries: Some(false),
        ..Default::default()
    };

    let updated = with_config_overrides(model.clone(), &config);

    assert_eq!(updated, model);
}

#[test]
fn model_context_window_override_clamps_to_max_context_window() {
    let mut model = model_info_from_slug("unknown-model");
    model.context_window = Some(273_000);
    model.max_context_window = Some(400_000);
    let config = ModelsManagerConfig {
        model_context_window: Some(500_000),
        ..Default::default()
    };

    let updated = with_config_overrides(model.clone(), &config);
    let mut expected = model;
    expected.context_window = Some(400_000);

    assert_eq!(updated, expected);
}

#[test]
fn model_context_window_uses_model_value_without_override() {
    let mut model = model_info_from_slug("unknown-model");
    model.context_window = Some(273_000);
    model.max_context_window = Some(400_000);
    let config = ModelsManagerConfig::default();

    let updated = with_config_overrides(model.clone(), &config);

    assert_eq!(updated, model);
}

#[test]
fn base_instruction_override_preserves_model_owned_token_budget() {
    let mut model = model_info_from_slug("gpt-5.2-codex");
    let token_budget = model_token_budget();
    model
        .model_messages
        .as_mut()
        .expect("fallback model should have personality metadata")
        .token_budget = Some(token_budget.clone());
    let config = ModelsManagerConfig {
        base_instructions: Some("explicit instructions".to_string()),
        personality_enabled: true,
        ..Default::default()
    };

    let updated = with_config_overrides(model, &config);

    assert_eq!(updated.base_instructions, "explicit instructions");
    assert_eq!(
        updated.model_messages,
        Some(ModelMessages {
            instructions_template: None,
            instructions_variables: None,
            token_budget: Some(token_budget),
        })
    );
}

#[test]
fn disabled_personality_preserves_model_owned_token_budget() {
    let mut model = model_info_from_slug("gpt-5.2-codex");
    let token_budget = model_token_budget();
    model
        .model_messages
        .as_mut()
        .expect("fallback model should have personality metadata")
        .token_budget = Some(token_budget.clone());
    let config = ModelsManagerConfig {
        personality_enabled: false,
        ..Default::default()
    };

    let updated = with_config_overrides(model, &config);

    assert_eq!(
        updated.model_messages,
        Some(ModelMessages {
            instructions_template: None,
            instructions_variables: None,
            token_budget: Some(token_budget),
        })
    );
}
