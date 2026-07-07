use super::*;
use codex_extension_api::ExtensionData;
use codex_extension_api::TurnItemContributionFuture;
use codex_extension_api::TurnItemContributor;
use codex_protocol::items::AgentMessageContent;
use pretty_assertions::assert_eq;
use std::sync::Arc;

struct RewriteAgentMessageContributor;

impl TurnItemContributor for RewriteAgentMessageContributor {
    fn contribute<'a>(
        &'a self,
        _thread_store: &'a ExtensionData,
        _turn_store: &'a ExtensionData,
        item: &'a mut TurnItem,
    ) -> TurnItemContributionFuture<'a> {
        Box::pin(async move {
            if let TurnItem::AgentMessage(agent_message) = item {
                agent_message.content = vec![AgentMessageContent::Text {
                    text: "plan contributed assistant text".to_string(),
                }];
            }
            Ok(())
        })
    }
}

fn assistant_output_text(text: &str) -> ResponseItem {
    ResponseItem::Message {
        id: Some("msg-1".to_string()),
        role: "assistant".to_string(),
        content: vec![ContentItem::OutputText {
            text: text.to_string(),
        }],
        phase: None,
    }
}

#[tokio::test]
async fn plan_mode_uses_contributed_turn_item_for_last_agent_message() {
    let (mut session, turn_context) = crate::session::tests::make_session_and_context().await;
    let mut builder = codex_extension_api::ExtensionRegistryBuilder::new();
    builder.turn_item_contributor(Arc::new(RewriteAgentMessageContributor));
    session.services.extensions = Arc::new(builder.build());
    let turn_store = ExtensionData::new(turn_context.sub_id.clone());
    let mut state = PlanModeStreamState::new(&turn_context.sub_id);
    let mut last_agent_message = None;
    let item = assistant_output_text("original assistant text");

    let handled = handle_assistant_item_done_in_plan_mode(
        &session,
        &turn_context,
        &turn_store,
        &item,
        &mut state,
        /*previously_active_item*/ None,
        &mut last_agent_message,
    )
    .await;

    assert!(handled);
    assert_eq!(
        last_agent_message.as_deref(),
        Some("plan contributed assistant text")
    );
}

#[tokio::test]
async fn previous_model_for_pre_sampling_compact_prefers_reference_context_item() {
    let (_, turn_context) = crate::session::tests::make_session_and_context().await;
    let mut reference_context_item = turn_context.to_turn_context_item();
    reference_context_item.model = "durable-reference-model".to_string();
    let previous_turn_settings = PreviousTurnSettings {
        model: "stale-previous-settings-model".to_string(),
        realtime_active: Some(false),
    };

    let previous_model = previous_model_for_pre_sampling_compact(
        Some(&reference_context_item),
        Some(&previous_turn_settings),
    );

    assert_eq!(previous_model.as_deref(), Some("durable-reference-model"));
}

#[tokio::test]
async fn previous_model_for_pre_sampling_compact_falls_back_to_previous_turn_settings() {
    let previous_turn_settings = PreviousTurnSettings {
        model: "legacy-previous-settings-model".to_string(),
        realtime_active: Some(false),
    };

    let previous_model =
        previous_model_for_pre_sampling_compact(None, Some(&previous_turn_settings));

    assert_eq!(
        previous_model.as_deref(),
        Some("legacy-previous-settings-model")
    );
}

#[test]
fn project_pre_sampling_total_usage_tokens_counts_pending_context_and_user_input() {
    let current_usage_tokens = 90;
    let pending_context_update_tokens = 7;
    let pending_user_input_tokens = 5;

    let projected_total_usage_tokens = project_pre_sampling_total_usage_tokens(
        current_usage_tokens,
        pending_context_update_tokens,
        pending_user_input_tokens,
    );

    assert!(current_usage_tokens < 100);
    assert!(projected_total_usage_tokens >= 100);
    assert_eq!(projected_total_usage_tokens, 102);
}

#[test]
fn estimate_pending_user_input_tokens_counts_text_without_reading_local_images() {
    let input = vec![
        UserInput::Text {
            text: "Summarize the current context state.".to_string(),
            text_elements: Vec::new(),
        },
        UserInput::LocalImage {
            path: "/tmp/hepta-context-nonexistent-image.png".into(),
            detail: None,
        },
    ];

    let estimated_tokens = estimate_pending_user_input_tokens(&input);

    assert!(estimated_tokens > 0);
}
