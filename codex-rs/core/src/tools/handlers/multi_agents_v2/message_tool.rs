//! Shared argument parsing and dispatch for the v2 text-only agent messaging tools.
//!
//! `send_message` and `followup_task` share the same submission path and differ only in whether the
//! resulting `InterAgentCommunication` should wake the target immediately.

use super::*;
use crate::session::turn_context::TurnContext;
use crate::tools::context::FunctionToolOutput;
use crate::tools::handlers::multi_agents_common::tool_output_json_text;
use crate::tools::handlers::work_graph_admission::WorkGraphAgentCardManifestObservation;
use crate::tools::handlers::work_graph_admission::WorkGraphRoleManifestShadowDecision;
use crate::tools::handlers::work_graph_admission::build_agent_card_manifest_shadow_decision;
use crate::tools::handlers::work_graph_admission::configured_agent_role_manifest_source;
use crate::tools::handlers::work_graph_admission::subagent_handoff_agent_card_manifest;
use crate::turn_timing::now_unix_timestamp_ms;
use codex_protocol::protocol::InterAgentCommunication;
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum MessageDeliveryMode {
    QueueOnly,
    TriggerTurn,
}

impl MessageDeliveryMode {
    /// Returns whether the produced communication should start a turn immediately.
    fn apply(self, communication: InterAgentCommunication) -> InterAgentCommunication {
        match self {
            Self::QueueOnly => InterAgentCommunication {
                trigger_turn: false,
                ..communication
            },
            Self::TriggerTurn => InterAgentCommunication {
                trigger_turn: true,
                ..communication
            },
        }
    }

    fn tool_name(self) -> &'static str {
        match self {
            Self::QueueOnly => "send_message",
            Self::TriggerTurn => "followup_task",
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MessageToolShadowResult {
    work_graph_handoff_shadow_decision: WorkGraphRoleManifestShadowDecision,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Input for the MultiAgentV2 `send_message` tool.
pub(crate) struct SendMessageArgs {
    pub(crate) target: String,
    pub(crate) message: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Input for the MultiAgentV2 `followup_task` tool.
pub(crate) struct FollowupTaskArgs {
    pub(crate) target: String,
    pub(crate) message: String,
}

fn message_content(message: String) -> Result<String, FunctionCallError> {
    if message.trim().is_empty() {
        return Err(FunctionCallError::RespondToModel(
            "Empty message can't be sent to an agent".to_string(),
        ));
    }
    Ok(message)
}

/// Handles the shared MultiAgentV2 plain-text message flow for both `send_message` and `followup_task`.
pub(crate) async fn handle_message_string_tool(
    invocation: ToolInvocation,
    mode: MessageDeliveryMode,
    target: String,
    message: String,
) -> Result<FunctionToolOutput, FunctionCallError> {
    let prompt = message_content(message)?;
    let ToolInvocation {
        session,
        turn,
        call_id,
        ..
    } = invocation;
    let receiver_thread_id = resolve_agent_target(&session, &turn, &target).await?;
    let receiver_agent = session
        .services
        .agent_control
        .get_agent_metadata(receiver_thread_id)
        .unwrap_or_default();
    if mode == MessageDeliveryMode::TriggerTurn
        && receiver_agent
            .agent_path
            .as_ref()
            .is_some_and(AgentPath::is_root)
    {
        return Err(FunctionCallError::RespondToModel(
            "Tasks can't be assigned to the root agent".to_string(),
        ));
    }
    session
        .send_event(
            &turn,
            CollabAgentInteractionBeginEvent {
                call_id: call_id.clone(),
                started_at_ms: now_unix_timestamp_ms(),
                sender_thread_id: session.conversation_id,
                receiver_thread_id,
                prompt: prompt.clone(),
            }
            .into(),
        )
        .await;
    let receiver_agent_path = receiver_agent.agent_path.clone().ok_or_else(|| {
        FunctionCallError::RespondToModel("target agent is missing an agent_path".to_string())
    })?;
    let handoff_shadow_decision = build_handoff_role_manifest_shadow_decision(
        mode,
        &turn,
        receiver_agent.agent_role.as_ref(),
    );
    let communication = InterAgentCommunication::new(
        turn.session_source
            .get_agent_path()
            .unwrap_or_else(AgentPath::root),
        receiver_agent_path,
        Vec::new(),
        prompt.clone(),
        /*trigger_turn*/ true,
    );
    let result = session
        .services
        .agent_control
        .send_inter_agent_communication(receiver_thread_id, mode.apply(communication))
        .await
        .map_err(|err| collab_agent_error(receiver_thread_id, err));
    let status = session
        .services
        .agent_control
        .get_status(receiver_thread_id)
        .await;
    session
        .send_event(
            &turn,
            CollabAgentInteractionEndEvent {
                call_id,
                completed_at_ms: now_unix_timestamp_ms(),
                sender_thread_id: session.conversation_id,
                receiver_thread_id,
                receiver_agent_nickname: receiver_agent.agent_nickname,
                receiver_agent_role: receiver_agent.agent_role,
                prompt,
                status,
            }
            .into(),
        )
        .await;
    result?;

    Ok(FunctionToolOutput::from_text(
        tool_output_json_text(
            &MessageToolShadowResult {
                work_graph_handoff_shadow_decision: handoff_shadow_decision,
            },
            mode.tool_name(),
        ),
        Some(true),
    ))
}

fn build_handoff_role_manifest_shadow_decision(
    mode: MessageDeliveryMode,
    turn: &TurnContext,
    receiver_agent_role: Option<&String>,
) -> WorkGraphRoleManifestShadowDecision {
    let requested_role = receiver_agent_role
        .map(String::as_str)
        .map(str::trim)
        .filter(|role| !role.is_empty());
    let configured_role = requested_role.and_then(|role| turn.config.agent_roles.get(role));
    let role_declared = requested_role.is_none() || configured_role.is_some();
    let role_description_present = requested_role.is_none()
        || configured_role
            .and_then(|role| role.description.as_deref())
            .is_some_and(|description| !description.trim().is_empty());

    build_agent_card_manifest_shadow_decision(
        subagent_handoff_agent_card_manifest(mode.tool_name()),
        WorkGraphAgentCardManifestObservation {
            role_name: requested_role.map(str::to_string),
            role_declared,
            role_description_present,
            configured_manifest_source: configured_agent_role_manifest_source(
                requested_role,
                configured_role.is_some(),
                configured_role.is_some_and(|role| role.config_file.is_some()),
                configured_role.and_then(|role| role.agent_card_manifest_source.as_deref()),
            ),
            configured_manifest_version: configured_role
                .and_then(|role| role.agent_card_manifest_version.clone()),
            configured_manifest_overlay: configured_role
                .and_then(|role| role.agent_card_manifest.clone()),
            budget_present: turn
                .config
                .agent_max_threads
                .is_none_or(|max_threads| max_threads > 0),
            output_contract_present: None,
            result_contract_present: None,
            verifier_present: None,
            reducer_present: None,
            attempted_tool: Some(mode.tool_name()),
            observed_lane: Some("subagent_handoff"),
        },
    )
}
