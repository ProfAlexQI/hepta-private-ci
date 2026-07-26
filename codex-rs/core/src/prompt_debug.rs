use std::collections::HashSet;
use std::sync::Arc;

use codex_exec_server::EnvironmentManager;
use codex_exec_server::ExecServerRuntimePaths;
use codex_login::AuthManager;
use codex_protocol::error::CodexErr;
use codex_protocol::error::Result as CodexResult;
use codex_protocol::models::ResponseInputItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::SessionSource;
use codex_protocol::user_input::UserInput;
use tokio_util::sync::CancellationToken;

use crate::config::Config;
use crate::resolve_installation_id;
use crate::session::session::Session;
use crate::session::turn::build_prompt;
use crate::session::turn::built_tools;
use crate::state_db_bridge::StateDbHandle;
use crate::thread_manager::ThreadManager;
use crate::thread_manager::thread_store_from_config;

/// Build the model-visible `input` list for a single debug turn.
#[doc(hidden)]
pub async fn build_prompt_input(
    mut config: Config,
    input: Vec<UserInput>,
    state_db: Option<StateDbHandle>,
) -> CodexResult<Vec<ResponseItem>> {
    config.ephemeral = true;

    let auth_manager =
        AuthManager::shared_from_config(&config, /*enable_codex_api_key_env*/ false).await;

    let local_runtime_paths = ExecServerRuntimePaths::from_optional_paths(
        config.codex_self_exe.clone(),
        config.codex_linux_sandbox_exe.clone(),
    )?;

    let thread_store = thread_store_from_config(&config, state_db.clone());
    let installation_id = resolve_installation_id(&config.codex_home).await?;
    let thread_manager = ThreadManager::new(
        &config,
        Arc::clone(&auth_manager),
        SessionSource::Exec,
        Arc::new(
            EnvironmentManager::from_codex_home(config.codex_home.clone(), local_runtime_paths)
                .await
                .map_err(|err| CodexErr::Fatal(err.to_string()))?,
        ),
        crate::default_thread_extension_registry(),
        /*analytics_events_client*/ None,
        thread_store,
        state_db.clone(),
        installation_id,
        /*attestation_provider*/ None,
    );
    let thread = thread_manager.start_thread(config).await?;

    let output = build_prompt_input_from_session(thread.thread.codex.session.as_ref(), input).await;
    let shutdown = thread.thread.shutdown_and_wait().await;
    let _removed = thread_manager.remove_thread(&thread.thread_id).await;

    shutdown?;
    output
}

pub(crate) async fn build_prompt_input_from_session(
    sess: &Session,
    input: Vec<UserInput>,
) -> CodexResult<Vec<ResponseItem>> {
    build_prompt_input_from_session_with_manifest_options(sess, input, None).await
}

async fn build_prompt_input_from_session_with_manifest_options(
    sess: &Session,
    input: Vec<UserInput>,
    manifest_options: Option<crate::context_manager::manifest::TurnContextManifestOptions>,
) -> CodexResult<Vec<ResponseItem>> {
    let turn_context = sess.new_default_turn().await;
    if let Some(manifest_options) = manifest_options {
        sess.record_context_updates_and_set_reference_context_item_with_manifest_options(
            turn_context.as_ref(),
            manifest_options,
        )
        .await;
    } else {
        sess.record_context_updates_and_set_reference_context_item(turn_context.as_ref())
            .await;
    }

    if !input.is_empty() {
        let input_item = ResponseInputItem::from(input);
        let response_item = ResponseItem::from(input_item);
        sess.record_conversation_items(turn_context.as_ref(), std::slice::from_ref(&response_item))
            .await;
    }

    let prompt_input = sess
        .clone_history()
        .await
        .for_prompt(&turn_context.model_info.input_modalities);
    let router = built_tools(
        sess,
        turn_context.as_ref(),
        &prompt_input,
        &HashSet::new(),
        Some(turn_context.turn_skills.outcome.as_ref()),
        &CancellationToken::new(),
    )
    .await?;
    let base_instructions = sess.get_base_instructions().await;
    let prompt = build_prompt(
        prompt_input,
        router.as_ref(),
        turn_context.as_ref(),
        base_instructions,
    );

    Ok(prompt.get_formatted_input())
}

#[cfg(test)]
mod tests {
    use codex_protocol::models::ContentItem;
    use codex_protocol::protocol::TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION;
    use codex_protocol::protocol::TurnContextRecallSelectedSnippet;
    use codex_protocol::protocol::TurnContextRecallSelectedSnippetEnvelope;
    use codex_protocol::protocol::TurnContextRecallSelectedSnippetSafety;
    use codex_protocol::protocol::TurnContextRecallSelectionSummary;
    use codex_protocol::user_input::UserInput;

    use super::*;

    #[tokio::test]
    async fn build_prompt_input_from_session_consumes_context_manifest_without_shadow_leak()
    -> CodexResult<()> {
        let (session, _turn_context) = crate::session::tests::make_session_and_context().await;
        let selected_snippets = test_selected_snippet_envelope();
        assert!(selected_snippets.has_shadow_integrity());

        let input = build_prompt_input_from_session_with_manifest_options(
            &session,
            vec![UserInput::Text {
                text: "hello with turn-scoped context manifest".to_string(),
                text_elements: Vec::new(),
            }],
            Some(
                crate::context_manager::manifest::TurnContextManifestOptions {
                    recall_provider_rollup: Some(
                        crate::context_manager::manifest::ContextRecallProviderRollup {
                            recall_selection: test_recall_selection_summary(),
                        },
                    ),
                    recall_selected_snippets: Some(
                        crate::context_manager::manifest::ContextRecallSelectedSnippetEnvelope {
                            envelope: selected_snippets,
                        },
                    ),
                    ..Default::default()
                },
            ),
        )
        .await?;

        let prompt_json = serde_json::to_string(&input).expect("prompt input should serialize");
        assert!(prompt_json.contains("<selected_context_recall>"));
        assert!(prompt_json.contains("fedcba9876543210"));
        assert!(prompt_json.contains("[redacted-query] bounded memory"));
        assert!(prompt_json.contains("hello with turn-scoped context manifest"));

        for forbidden in [
            "recall_selection",
            "recall_selected_snippets",
            "selected_snippet_count",
            "returned_source_count",
            "ranked_item_count",
            "omitted_by_budget_count",
            "source-memory-id",
            "source_id",
            "[hepta-memory:",
            "needle",
            "raw_ranked_payload_exposed",
            "origin_identifiers_exposed",
        ] {
            assert!(
                !prompt_json.contains(forbidden),
                "prompt input leaked shadow manifest field or source payload: {forbidden}"
            );
        }

        let user_message = ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: "hello with turn-scoped context manifest".to_string(),
            }],
            phase: None,
        };
        assert_eq!(input.last(), Some(&user_message));

        Ok(())
    }

    fn test_recall_selection_summary() -> TurnContextRecallSelectionSummary {
        TurnContextRecallSelectionSummary {
            returned_source_count: 4,
            selected_source_count: 2,
            ranked_source_count: 2,
            returned_unselected_source_count: 2,
            source_diversity_met: true,
            source_diversity_target: 2,
            max_per_source: 1,
            ranked_item_count: 5,
            omitted_by_budget_count: 1,
            memory_control_omitted_count: 1,
            low_trust_ranked_item_count: 1,
            low_recency_ranked_item_count: 0,
        }
    }

    fn test_selected_snippet_envelope() -> TurnContextRecallSelectedSnippetEnvelope {
        TurnContextRecallSelectedSnippetEnvelope {
            version: TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
            max_snippets: 4,
            max_snippet_chars: 120,
            selected_snippet_count: 1,
            omitted_snippet_count: 2,
            redacted_snippet_count: 1,
            truncated_snippet_count: 0,
            snippets: vec![TurnContextRecallSelectedSnippet {
                snippet_hash: "fedcba9876543210".into(),
                text: "[redacted-query] bounded memory".into(),
                estimated_tokens: 8,
                redacted: true,
                truncated: false,
            }],
            safety: TurnContextRecallSelectedSnippetSafety {
                ready_for_shadow_handoff: true,
                bounded: true,
                origin_identifiers_exposed: false,
                raw_ranked_payload_exposed: false,
                rank_explanation_exposed: false,
                control_marker_exposed: false,
                query_payload_exposed: false,
                per_origin_list_exposed: false,
            },
        }
    }
}
