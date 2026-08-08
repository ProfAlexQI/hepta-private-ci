use super::*;
use codex_extension_api::ExtensionData;
use codex_extension_api::TurnItemContributor;
use codex_protocol::ResponseItemId;
use codex_protocol::items::AgentMessageContent;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use tracing_subscriber::prelude::*;

struct RewriteAgentMessageContributor;

impl TurnItemContributor for RewriteAgentMessageContributor {
    fn contribute<'a>(
        &'a self,
        _thread_store: &'a ExtensionData,
        _turn_store: &'a ExtensionData,
        item: &'a mut TurnItem,
    ) -> codex_extension_api::ExtensionFuture<'a, Result<(), String>> {
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
        id: Some(ResponseItemId::with_suffix("msg", "1")),
        role: "assistant".to_string(),
        content: vec![ContentItem::OutputText {
            text: text.to_string(),
        }],
        phase: None,
        internal_chat_message_metadata_passthrough: None,
    }
}

#[tokio::test]
async fn post_sampling_token_estimate_is_filtered_from_always_on_sinks() {
    let codex_home = tempfile::tempdir().expect("create isolated state directory");
    let runtime = codex_state::StateRuntime::init(
        codex_state::SqliteConfig::new_for_testing(
            codex_utils_absolute_path::AbsolutePathBuf::try_from(codex_home.path())
                .expect("temporary state directory should be absolute"),
        ),
        "test-provider".to_string(),
    )
    .await
    .expect("initialize isolated state runtime");
    let feedback = codex_feedback::CodexFeedback::new();
    let log_db = codex_state::log_db::start(Arc::clone(&runtime));
    let subscriber = tracing_subscriber::registry()
        .with(feedback.logger_layer())
        .with(
            log_db
                .clone()
                .with_filter(codex_state::log_db::default_filter()),
        );

    tracing::subscriber::with_default(subscriber, || {
        tracing::trace!(
            target: POST_SAMPLING_TOKEN_ESTIMATE_TARGET,
            turn_id = "test-turn",
            estimated_token_count = 42_u64,
            "filtered post sampling token estimate"
        );
        tracing::trace!(
            target: "codex_core::turn_filter_control",
            "retained turn filter control"
        );
    });
    log_db.flush().await;

    let log_rows = runtime
        .query_logs(&codex_state::LogQuery {
            include_threadless: true,
            ..Default::default()
        })
        .await
        .expect("query isolated log database");
    assert!(
        !log_rows
            .iter()
            .any(|row| row.target == POST_SAMPLING_TOKEN_ESTIMATE_TARGET)
    );
    assert!(log_rows.iter().any(|row| {
        row.target == "codex_core::turn_filter_control"
            && row
                .message
                .as_deref()
                .is_some_and(|message| message.contains("retained turn filter control"))
    }));
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
