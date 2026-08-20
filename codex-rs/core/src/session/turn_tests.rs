use super::*;
use codex_extension_api::ExtensionData;
use codex_extension_api::TurnItemContributor;
use codex_protocol::ResponseItemId;
use codex_protocol::items::AgentMessageContent;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use tracing_subscriber::prelude::*;

use crate::environment_selection::EnvironmentConfigOrigin;
use crate::environment_selection::ThreadEnvironments;
use crate::environment_selection::TurnEnvironmentState;

#[test]
fn turn_recovery_does_not_claim_provider_output_arrival_exactly_once() {
    assert!(!TURN_RECOVERY_PROVIDER_OUTPUT_ARRIVAL_EXACTLY_ONCE);
}

#[test]
fn hepta_turn_recovery_blocks_only_pre_turn_auto_compaction() {
    assert!(hepta_turn_recovery_blocks_auto_compact(
        true,
        CompactionPhase::PreTurn,
    ));
    assert!(!hepta_turn_recovery_blocks_auto_compact(
        false,
        CompactionPhase::PreTurn,
    ));
    assert!(!hepta_turn_recovery_blocks_auto_compact(
        true,
        CompactionPhase::MidTurn,
    ));
    assert!(!hepta_turn_recovery_blocks_auto_compact(
        true,
        CompactionPhase::StandaloneTurn,
    ));
}

#[tokio::test]
async fn turn_recovery_checkpoint_accepts_only_nonempty_from_thread_environments() {
    let (_session, turn_context) = crate::session::tests::make_session_and_context().await;
    let primary = turn_context
        .environments
        .primary()
        .expect("ready primary environment")
        .clone();

    let mut first = primary.clone();
    first.selection.environment_id = "thread-environment-b".to_string();
    first.selection.workspace_roots = vec![first.selection.cwd.clone()];
    let mut second = primary.clone();
    second.selection.environment_id = "thread-environment-a".to_string();
    second.selection.workspace_roots = Vec::new();
    let from_thread = TurnEnvironmentSnapshot {
        environments: vec![
            TurnEnvironmentState::Ready(first.clone()),
            TurnEnvironmentState::Ready(second.clone()),
        ],
    };
    let expected = [first, second]
        .into_iter()
        .map(
            |environment| codex_history::TurnRecoveryEnvironmentSelection {
                environment_id: environment.selection.environment_id,
                cwd: environment.selection.cwd.to_string(),
                workspace_roots: environment
                    .selection
                    .workspace_roots
                    .into_iter()
                    .map(|root| root.to_string())
                    .collect(),
            },
        )
        .collect::<Vec<_>>();
    assert_eq!(
        turn_recovery_environment_selections(&from_thread),
        Some(expected)
    );

    let mut owner_ready = primary.clone();
    owner_ready.config_origin = EnvironmentConfigOrigin::Owner;
    assert_eq!(
        turn_recovery_environment_selections(&TurnEnvironmentSnapshot {
            environments: vec![TurnEnvironmentState::Ready(owner_ready)],
        }),
        None
    );

    let thread_environment_config = primary.config().clone();
    let environment_manager = Arc::new(codex_exec_server::EnvironmentManager::default_for_tests());
    let mut owner_pending_selection = primary.selection();
    owner_pending_selection.config = EnvironmentConfigState::Pending;
    let owner_pending = ThreadEnvironments::new(
        Arc::clone(&environment_manager),
        crate::shell::default_user_shell(),
        thread_environment_config.clone(),
        crate::shell_snapshot::ShellSnapshot::disabled(),
        TurnEnvironmentSnapshot::default(),
        /*non_blocking_snapshots*/ true,
    );
    owner_pending.update_selections(
        std::slice::from_ref(&owner_pending_selection),
        &thread_environment_config,
    );
    let owner_pending = owner_pending.snapshot().await;
    assert!(owner_pending.starting().next().is_some());
    assert_eq!(turn_recovery_environment_selections(&owner_pending), None);

    let mut owner_failed_selection = primary.selection();
    owner_failed_selection.config = EnvironmentConfigState::Failed("owner failed".to_string());
    let owner_failed = ThreadEnvironments::new(
        environment_manager,
        crate::shell::default_user_shell(),
        thread_environment_config.clone(),
        crate::shell_snapshot::ShellSnapshot::disabled(),
        TurnEnvironmentSnapshot::default(),
        /*non_blocking_snapshots*/ true,
    );
    owner_failed.update_selections(
        std::slice::from_ref(&owner_failed_selection),
        &thread_environment_config,
    );
    let owner_failed = owner_failed.snapshot().await;
    assert!(owner_failed.environments.is_empty());
    assert_eq!(turn_recovery_environment_selections(&owner_failed), None);

    assert_eq!(
        turn_recovery_environment_selections(&TurnEnvironmentSnapshot::default()),
        None
    );
}

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

struct PostSamplingTokenEstimateCallsite;

static POST_SAMPLING_TOKEN_ESTIMATE_CALLSITE: PostSamplingTokenEstimateCallsite =
    PostSamplingTokenEstimateCallsite;
static POST_SAMPLING_TOKEN_ESTIMATE_METADATA: tracing::Metadata<'static> = tracing::metadata! {
    name: "post_sampling_token_estimate",
    target: POST_SAMPLING_TOKEN_ESTIMATE_TARGET,
    level: tracing::Level::TRACE,
    fields: &["turn_id", "estimated_token_count", "message"],
    callsite: &POST_SAMPLING_TOKEN_ESTIMATE_CALLSITE,
    kind: tracing::metadata::Kind::EVENT,
};

impl tracing::Callsite for PostSamplingTokenEstimateCallsite {
    fn set_interest(&self, _interest: tracing::subscriber::Interest) {}

    fn metadata(&self) -> &tracing::Metadata<'_> {
        &POST_SAMPLING_TOKEN_ESTIMATE_METADATA
    }
}

#[test]
fn post_sampling_token_estimate_is_disabled_by_always_on_sinks() {
    let feedback = codex_feedback::CodexFeedback::new();
    let subscriber = tracing_subscriber::registry()
        .with(feedback.logger_layer())
        .with(tracing_subscriber::fmt::layer().with_filter(codex_state::log_db::default_filter()));

    tracing::subscriber::with_default(subscriber, || {
        tracing::callsite::rebuild_interest_cache();
        assert!(!tracing::event_enabled!(
            target: POST_SAMPLING_TOKEN_ESTIMATE_TARGET,
            tracing::Level::TRACE,
            turn_id,
            estimated_token_count,
            message
        ));
    });
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
