use super::*;
use crate::agent::control::SpawnAgentForkMode;
use crate::agent::control::SpawnAgentOptions;
use crate::agent::control::render_input_preview;
use crate::agent::next_thread_spawn_depth;
use crate::agent::role::DEFAULT_ROLE_NAME;
use crate::agent::role::apply_role_to_config;
use crate::tools::handlers::multi_agents_spec::SpawnAgentToolOptions;
use crate::tools::handlers::multi_agents_spec::create_spawn_agent_tool_v2;
use crate::tools::handlers::work_graph_admission::WorkGraphAdmissionShadowDecision;
use crate::tools::handlers::work_graph_admission::WorkGraphAdmissionShadowInput;
use crate::tools::handlers::work_graph_admission::WorkGraphAgentCardManifestObservation;
use crate::tools::handlers::work_graph_admission::WorkGraphRoleManifestShadowDecision;
use crate::tools::handlers::work_graph_admission::build_agent_card_manifest_shadow_decision;
use crate::tools::handlers::work_graph_admission::build_work_graph_admission_shadow_decision;
use crate::tools::handlers::work_graph_admission::configured_agent_role_manifest_source;
use crate::tools::handlers::work_graph_admission::subagent_spawn_agent_card_manifest;
use crate::turn_timing::now_unix_timestamp_ms;
use codex_protocol::AgentPath;
use codex_protocol::protocol::InterAgentCommunication;
use codex_protocol::protocol::Op;
use codex_tools::ToolSpec;

#[derive(Default)]
pub(crate) struct Handler {
    options: SpawnAgentToolOptions,
}

impl Handler {
    pub(crate) fn new(options: SpawnAgentToolOptions) -> Self {
        Self { options }
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolInvocation> for Handler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain("spawn_agent")
    }

    fn spec(&self) -> Option<ToolSpec> {
        Some(create_spawn_agent_tool_v2(self.options.clone()))
    }

    async fn handle(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn crate::tools::context::ToolOutput>, FunctionCallError> {
        handle_spawn_agent(invocation).await.map(boxed_tool_output)
    }
}

async fn handle_spawn_agent(
    invocation: ToolInvocation,
) -> Result<SpawnAgentResult, FunctionCallError> {
    let ToolInvocation {
        session,
        turn,
        payload,
        call_id,
        ..
    } = invocation;
    let arguments = function_arguments(payload)?;
    let args: SpawnAgentArgs = parse_arguments(&arguments)?;
    let fork_mode = args.fork_mode()?;
    let role_name = args
        .agent_type
        .as_deref()
        .map(str::trim)
        .filter(|role| !role.is_empty());

    let initial_operation = parse_collab_input(Some(args.message), /*items*/ None)?;
    let prompt = render_input_preview(&initial_operation);

    let session_source = turn.session_source.clone();
    let child_depth = next_thread_spawn_depth(&session_source);
    let role_manifest_shadow_decision = build_spawn_agent_role_manifest_shadow_decision(
        "spawn_agent_v2",
        role_name,
        turn.as_ref(),
        child_depth,
    );
    let admission_shadow_decision =
        build_work_graph_admission_shadow_decision(WorkGraphAdmissionShadowInput {
            source_surface_id: "spawn_agent_v2",
            task_id: Some(format!(
                "spawn-agent:{}:{}",
                session.conversation_id, args.task_name
            )),
            job_id: None,
            role_manifest_shadow_decision,
            requested_concurrency: 1,
            item_count: Some(1),
            child_depth,
            max_depth: turn.config.agent_max_depth,
            max_threads: turn.config.agent_max_threads,
            enforce_depth_limit: false,
            state_db_required: false,
            state_db_available: session.state_db().is_some(),
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: true,
            result_contract_present: false,
            reducer_required: false,
            reducer_present: false,
            side_effect_class: "local_subagent_spawn",
        });
    session
        .send_event(
            &turn,
            CollabAgentSpawnBeginEvent {
                call_id: call_id.clone(),
                started_at_ms: now_unix_timestamp_ms(),
                sender_thread_id: session.conversation_id,
                prompt: prompt.clone(),
                model: args.model.clone().unwrap_or_default(),
                reasoning_effort: args.reasoning_effort.unwrap_or_default(),
            }
            .into(),
        )
        .await;
    let mut config =
        build_agent_spawn_config(&session.get_base_instructions().await, turn.as_ref())?;
    if matches!(fork_mode, Some(SpawnAgentForkMode::FullHistory)) {
        reject_full_fork_spawn_overrides(role_name, args.model.as_deref(), args.reasoning_effort)?;
    } else {
        apply_requested_spawn_agent_model_overrides(
            &session,
            turn.as_ref(),
            &mut config,
            args.model.as_deref(),
            args.reasoning_effort,
        )
        .await?;
        apply_role_to_config(&mut config, role_name)
            .await
            .map_err(FunctionCallError::RespondToModel)?;
    }
    apply_spawn_agent_service_tier(
        &session,
        &mut config,
        turn.config.service_tier.as_deref(),
        args.service_tier.as_deref(),
    )
    .await?;
    apply_spawn_agent_runtime_overrides(&mut config, turn.as_ref())?;
    apply_spawn_agent_overrides(&mut config, child_depth);

    let spawn_source = thread_spawn_source(
        session.conversation_id,
        &turn.session_source,
        child_depth,
        role_name,
        Some(args.task_name.clone()),
    )?;
    let result = Box::pin(
        session.services.agent_control.spawn_agent_with_metadata(
            config,
            match (spawn_source.get_agent_path(), initial_operation) {
                (Some(recipient), Op::UserInput { items, .. })
                    if items
                        .iter()
                        .all(|item| matches!(item, UserInput::Text { .. })) =>
                {
                    Op::InterAgentCommunication {
                        communication: InterAgentCommunication::new(
                            turn.session_source
                                .get_agent_path()
                                .unwrap_or_else(AgentPath::root),
                            recipient,
                            Vec::new(),
                            prompt.clone(),
                            /*trigger_turn*/ true,
                        ),
                    }
                }
                (_, initial_operation) => initial_operation,
            },
            Some(spawn_source),
            SpawnAgentOptions {
                fork_parent_spawn_call_id: fork_mode.as_ref().map(|_| call_id.clone()),
                fork_mode,
                environments: Some(turn.environments.to_selections()),
            },
        ),
    )
    .await
    .map_err(collab_spawn_error);
    let (new_thread_id, new_agent_metadata, status) = match &result {
        Ok(spawned_agent) => (
            Some(spawned_agent.thread_id),
            Some(spawned_agent.metadata.clone()),
            spawned_agent.status.clone(),
        ),
        Err(_) => (None, None, AgentStatus::NotFound),
    };
    let agent_snapshot = match new_thread_id {
        Some(thread_id) => {
            session
                .services
                .agent_control
                .get_agent_config_snapshot(thread_id)
                .await
        }
        None => None,
    };
    let (new_agent_path, new_agent_nickname, new_agent_role) =
        match (&agent_snapshot, new_agent_metadata) {
            (Some(snapshot), _) => (
                snapshot.session_source.get_agent_path().map(String::from),
                snapshot.session_source.get_nickname(),
                snapshot.session_source.get_agent_role(),
            ),
            (None, Some(metadata)) => (
                metadata.agent_path.map(String::from),
                metadata.agent_nickname,
                metadata.agent_role,
            ),
            (None, None) => (None, None, None),
        };
    let effective_model = agent_snapshot
        .as_ref()
        .map(|snapshot| snapshot.model.clone())
        .unwrap_or_else(|| args.model.clone().unwrap_or_default());
    let effective_reasoning_effort = agent_snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.reasoning_effort)
        .unwrap_or(args.reasoning_effort.unwrap_or_default());
    let nickname = new_agent_nickname.clone();
    session
        .send_event(
            &turn,
            CollabAgentSpawnEndEvent {
                call_id,
                completed_at_ms: now_unix_timestamp_ms(),
                sender_thread_id: session.conversation_id,
                new_thread_id,
                new_agent_nickname,
                new_agent_role,
                prompt,
                model: effective_model,
                reasoning_effort: effective_reasoning_effort,
                status,
            }
            .into(),
        )
        .await;
    let _ = result?;
    let role_tag = role_name.unwrap_or(DEFAULT_ROLE_NAME);
    turn.session_telemetry.counter(
        "codex.multi_agent.spawn",
        /*inc*/ 1,
        &[("role", role_tag)],
    );
    let task_name = new_agent_path.ok_or_else(|| {
        FunctionCallError::RespondToModel(
            "spawned agent is missing a canonical task name".to_string(),
        )
    })?;

    let hide_agent_metadata = turn.config.multi_agent_v2.hide_spawn_agent_metadata;
    if hide_agent_metadata {
        Ok(SpawnAgentResult::HiddenMetadata {
            task_name,
            admission_shadow_decision,
        })
    } else {
        Ok(SpawnAgentResult::WithNickname {
            task_name,
            nickname,
            admission_shadow_decision,
        })
    }
}

fn build_spawn_agent_role_manifest_shadow_decision(
    source_surface_id: &'static str,
    role_name: Option<&str>,
    turn: &crate::TurnContext,
    child_depth: i32,
) -> WorkGraphRoleManifestShadowDecision {
    let requested_role = role_name.map(str::to_string);
    let configured_role = role_name.and_then(|role| turn.config.agent_roles.get(role));
    let role_declared = role_name.is_none() || configured_role.is_some();
    let role_description_present = role_name.is_none()
        || configured_role
            .and_then(|role| role.description.as_deref())
            .is_some_and(|description| !description.trim().is_empty());
    build_agent_card_manifest_shadow_decision(
        subagent_spawn_agent_card_manifest(source_surface_id),
        WorkGraphAgentCardManifestObservation {
            role_name: requested_role,
            role_declared,
            role_description_present,
            configured_manifest_source: configured_agent_role_manifest_source(
                role_name,
                configured_role.is_some(),
                configured_role.is_some_and(|role| role.config_file.is_some()),
                configured_role.and_then(|role| role.agent_card_manifest_source.as_deref()),
            ),
            configured_manifest_version: configured_role
                .and_then(|role| role.agent_card_manifest_version.clone()),
            configured_manifest_overlay: configured_role
                .and_then(|role| role.agent_card_manifest.clone()),
            budget_present: child_depth <= turn.config.agent_max_depth
                && turn
                    .config
                    .agent_max_threads
                    .is_none_or(|max_threads| max_threads > 0),
            output_contract_present: None,
            result_contract_present: None,
            verifier_present: None,
            reducer_present: None,
            attempted_tool: None,
            observed_lane: Some("subagent"),
        },
    )
}

impl CoreToolRuntime for Handler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SpawnAgentArgs {
    message: String,
    task_name: String,
    agent_type: Option<String>,
    model: Option<String>,
    reasoning_effort: Option<ReasoningEffort>,
    service_tier: Option<String>,
    fork_turns: Option<String>,
    fork_context: Option<bool>,
}

impl SpawnAgentArgs {
    fn fork_mode(&self) -> Result<Option<SpawnAgentForkMode>, FunctionCallError> {
        if self.fork_context.is_some() {
            return Err(FunctionCallError::RespondToModel(
                "fork_context is not supported in MultiAgentV2; use fork_turns instead".to_string(),
            ));
        }

        let fork_turns = self
            .fork_turns
            .as_deref()
            .map(str::trim)
            .filter(|fork_turns| !fork_turns.is_empty())
            .unwrap_or("all");

        if fork_turns.eq_ignore_ascii_case("none") {
            return Ok(None);
        }
        if fork_turns.eq_ignore_ascii_case("all") {
            return Ok(Some(SpawnAgentForkMode::FullHistory));
        }

        let last_n_turns = fork_turns.parse::<usize>().map_err(|_| {
            FunctionCallError::RespondToModel(
                "fork_turns must be `none`, `all`, or a positive integer string".to_string(),
            )
        })?;
        if last_n_turns == 0 {
            return Err(FunctionCallError::RespondToModel(
                "fork_turns must be `none`, `all`, or a positive integer string".to_string(),
            ));
        }

        Ok(Some(SpawnAgentForkMode::LastNTurns(last_n_turns)))
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(crate) enum SpawnAgentResult {
    WithNickname {
        task_name: String,
        nickname: Option<String>,
        admission_shadow_decision: WorkGraphAdmissionShadowDecision,
    },
    HiddenMetadata {
        task_name: String,
        admission_shadow_decision: WorkGraphAdmissionShadowDecision,
    },
}

impl ToolOutput for SpawnAgentResult {
    fn log_preview(&self) -> String {
        tool_output_json_text(self, "spawn_agent")
    }

    fn success_for_logging(&self) -> bool {
        true
    }

    fn to_response_item(&self, call_id: &str, payload: &ToolPayload) -> ResponseInputItem {
        tool_output_response_item(call_id, payload, self, Some(true), "spawn_agent")
    }

    fn code_mode_result(&self, _payload: &ToolPayload) -> JsonValue {
        tool_output_code_mode_result(self, "spawn_agent")
    }
}
