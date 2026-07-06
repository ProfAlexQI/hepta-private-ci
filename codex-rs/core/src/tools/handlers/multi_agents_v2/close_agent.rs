use super::*;
use crate::session::turn_context::TurnContext;
use crate::tools::handlers::multi_agents_spec::create_close_agent_tool_v2;
use crate::tools::handlers::work_graph_admission::WorkGraphAgentCardManifestObservation;
use crate::tools::handlers::work_graph_admission::WorkGraphRoleManifestShadowDecision;
use crate::tools::handlers::work_graph_admission::build_agent_card_manifest_shadow_decision;
use crate::tools::handlers::work_graph_admission::configured_agent_role_manifest_source;
use crate::tools::handlers::work_graph_admission::subagent_lifecycle_agent_card_manifest;
use crate::turn_timing::now_unix_timestamp_ms;
use codex_tools::ToolSpec;

pub(crate) struct Handler;

#[async_trait::async_trait]
impl ToolExecutor<ToolInvocation> for Handler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain("close_agent")
    }

    fn spec(&self) -> Option<ToolSpec> {
        Some(create_close_agent_tool_v2())
    }

    async fn handle(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn crate::tools::context::ToolOutput>, FunctionCallError> {
        handle_close_agent(invocation).await.map(boxed_tool_output)
    }
}

async fn handle_close_agent(
    invocation: ToolInvocation,
) -> Result<CloseAgentResult, FunctionCallError> {
    let ToolInvocation {
        session,
        turn,
        payload,
        call_id,
        ..
    } = invocation;
    let arguments = function_arguments(payload)?;
    let args: CloseAgentArgs = parse_arguments(&arguments)?;
    let agent_id = resolve_agent_target(&session, &turn, &args.target).await?;
    let receiver_agent = session
        .services
        .agent_control
        .get_agent_metadata(agent_id)
        .unwrap_or_default();
    let lifecycle_shadow_decision = build_lifecycle_role_manifest_shadow_decision(
        "close_agent",
        &turn,
        receiver_agent.agent_role.as_ref(),
    );
    if receiver_agent
        .agent_path
        .as_ref()
        .is_some_and(AgentPath::is_root)
    {
        return Err(FunctionCallError::RespondToModel(
            "root is not a spawned agent".to_string(),
        ));
    }
    session
        .send_event(
            &turn,
            CollabCloseBeginEvent {
                call_id: call_id.clone(),
                started_at_ms: now_unix_timestamp_ms(),
                sender_thread_id: session.conversation_id,
                receiver_thread_id: agent_id,
            }
            .into(),
        )
        .await;
    let status = match session
        .services
        .agent_control
        .subscribe_status(agent_id)
        .await
    {
        Ok(mut status_rx) => status_rx.borrow_and_update().clone(),
        Err(err) => {
            let status = session.services.agent_control.get_status(agent_id).await;
            session
                .send_event(
                    &turn,
                    CollabCloseEndEvent {
                        call_id: call_id.clone(),
                        completed_at_ms: now_unix_timestamp_ms(),
                        sender_thread_id: session.conversation_id,
                        receiver_thread_id: agent_id,
                        receiver_agent_nickname: receiver_agent.agent_nickname.clone(),
                        receiver_agent_role: receiver_agent.agent_role.clone(),
                        status,
                    }
                    .into(),
                )
                .await;
            return Err(collab_agent_error(agent_id, err));
        }
    };
    let result = session
        .services
        .agent_control
        .close_agent(agent_id)
        .await
        .map_err(|err| collab_agent_error(agent_id, err))
        .map(|_| ());
    session
        .send_event(
            &turn,
            CollabCloseEndEvent {
                call_id,
                completed_at_ms: now_unix_timestamp_ms(),
                sender_thread_id: session.conversation_id,
                receiver_thread_id: agent_id,
                receiver_agent_nickname: receiver_agent.agent_nickname,
                receiver_agent_role: receiver_agent.agent_role,
                status: status.clone(),
            }
            .into(),
        )
        .await;
    result?;

    Ok(CloseAgentResult {
        previous_status: status,
        work_graph_lifecycle_shadow_decision: Some(lifecycle_shadow_decision),
    })
}

impl CoreToolRuntime for Handler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CloseAgentArgs {
    target: String,
}

fn build_lifecycle_role_manifest_shadow_decision(
    source_surface_id: &'static str,
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
        subagent_lifecycle_agent_card_manifest(source_surface_id),
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
            attempted_tool: Some(source_surface_id),
            observed_lane: Some("subagent_lifecycle"),
        },
    )
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CloseAgentResult {
    pub(crate) previous_status: AgentStatus,
    #[serde(default, skip_deserializing)]
    pub(crate) work_graph_lifecycle_shadow_decision: Option<WorkGraphRoleManifestShadowDecision>,
}

impl ToolOutput for CloseAgentResult {
    fn log_preview(&self) -> String {
        tool_output_json_text(self, "close_agent")
    }

    fn success_for_logging(&self) -> bool {
        true
    }

    fn to_response_item(&self, call_id: &str, payload: &ToolPayload) -> ResponseInputItem {
        tool_output_response_item(call_id, payload, self, Some(true), "close_agent")
    }

    fn code_mode_result(&self, _payload: &ToolPayload) -> JsonValue {
        tool_output_code_mode_result(self, "close_agent")
    }
}
