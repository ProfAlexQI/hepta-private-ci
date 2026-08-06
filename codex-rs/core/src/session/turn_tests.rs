use super::*;
use codex_extension_api::ExtensionData;
use codex_extension_api::TurnItemContributor;
use codex_protocol::ResponseItemId;
use codex_protocol::items::AgentMessageContent;
use pretty_assertions::assert_eq;
use sha2::Sha256;
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

fn prompt_only_context(enabled: bool) -> PromptOnlyInputContext {
    PromptOnlyInputContext {
        thread_id: "thread-1".to_string(),
        turn_id: "turn-1".to_string(),
        cwd: std::path::PathBuf::from("/workspace"),
        model_context_window: Some(128_000),
        host_authority_enabled: enabled,
    }
}

fn prompt_only_proposal(content: &str) -> PromptOnlyInputProposal {
    PromptOnlyInputProposal {
        schema_version: PROMPT_ONLY_INPUT_PROPOSAL_SCHEMA_VERSION,
        source: codex_extension_api::PromptOnlyInputSource::parse("hepta_memory_same_thread_v1")
            .expect("source"),
        thread_id: "thread-1".to_string(),
        turn_id: "turn-1".to_string(),
        source_binding_sha256: codex_extension_api::ModelProviderSha256Digest::parse(
            "a".repeat(64),
        )
        .expect("source binding"),
        content_sha256: codex_extension_api::ModelProviderSha256Digest::parse(format!(
            "{:x}",
            Sha256::digest(content.as_bytes())
        ))
        .expect("content binding"),
        content: content.to_string(),
        claimed_token_count: u32::try_from(content.len()).expect("bounded content"),
    }
}

#[test]
fn post_sampling_token_estimate_is_disabled_by_always_on_sinks() {
    let feedback = codex_feedback::CodexFeedback::new();
    let subscriber = tracing_subscriber::registry()
        .with(feedback.logger_layer())
        .with(tracing_subscriber::fmt::layer().with_filter(codex_state::log_db::default_filter()));

    tracing::subscriber::with_default(subscriber, || {
        assert!(!tracing::event_enabled!(
            target: POST_SAMPLING_TOKEN_ESTIMATE_TARGET,
            tracing::Level::TRACE,
            turn_id,
            estimated_token_count,
            message
        ));
    });
}

#[test]
fn prompt_only_proposal_requires_exact_host_binding() {
    let input = prompt_only_context(true);
    let proposal = prompt_only_proposal("reviewed memory summary");
    validate_prompt_only_input_proposal(&input, &proposal).expect("exact proposal");

    let mut disabled = input.clone();
    disabled.host_authority_enabled = false;
    assert!(validate_prompt_only_input_proposal(&disabled, &proposal).is_err());

    let mut substituted = proposal;
    substituted.content.push_str(" substituted");
    assert!(validate_prompt_only_input_proposal(&input, &substituted).is_err());
}

#[test]
fn prompt_only_policy_binding_changes_with_source_or_budget() {
    let proposal = prompt_only_proposal("reviewed memory summary");
    let original = prompt_only_policy_binding(&proposal).expect("original binding");
    let mut source_drift = proposal.clone();
    source_drift.source_binding_sha256 =
        codex_extension_api::ModelProviderSha256Digest::parse("b".repeat(64))
            .expect("source drift");
    let mut budget_drift = proposal;
    budget_drift.claimed_token_count += 1;

    assert_ne!(
        original,
        prompt_only_policy_binding(&source_drift).expect("source binding")
    );
    assert_ne!(
        original,
        prompt_only_policy_binding(&budget_drift).expect("budget binding")
    );
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
