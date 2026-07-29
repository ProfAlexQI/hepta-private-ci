use crate::function_tool::FunctionCallError;
use crate::session::session::Session;
use crate::session::turn_context::TurnContext;
use crate::tools::context::SharedTurnDiffTracker;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::registry::AnyToolResult;
use crate::tools::registry::ToolArgumentDiffConsumer;
use crate::tools::registry::ToolRegistry;
use crate::tools::spec_plan::build_tool_router;
use codex_mcp::ToolInfo;
use codex_protocol::dynamic_tools::DynamicToolSpec;
use codex_protocol::models::ResponseItem;
use codex_protocol::models::SearchToolCallParams;
use codex_tools::DiscoverableTool;
use codex_tools::ToolCall as ExtensionToolCall;
use codex_tools::ToolExecutor;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use codex_tools::ToolsConfig;
use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::instrument;

pub use crate::tools::context::ToolCallSource;

#[derive(Clone, Debug)]
pub struct ToolCall {
    pub tool_name: ToolName,
    pub call_id: String,
    pub payload: ToolPayload,
}

pub(crate) fn direct_source_for_response_item(item: &ResponseItem) -> ToolCallSource {
    if let ResponseItem::FunctionCall {
        name,
        namespace,
        encrypted_function_args: Some(encrypted_function_args),
        ..
    } = item
        && namespace.as_deref() == Some("collaboration")
        && matches!(
            name.as_str(),
            "spawn_agent" | "send_message" | "followup_task"
        )
        && encrypted_function_args.is_empty()
    {
        ToolCallSource::DirectPlaintextMessage
    } else {
        ToolCallSource::Direct
    }
}

pub(crate) fn tool_log_payload<'a>(
    payload: &'a ToolPayload,
    source: &ToolCallSource,
) -> Cow<'a, str> {
    if matches!(source, ToolCallSource::DirectPlaintextMessage) {
        Cow::Borrowed("[plaintext arguments]")
    } else {
        payload.log_payload()
    }
}

pub struct ToolRouter {
    registry: ToolRegistry,
    model_visible_specs: Vec<ToolSpec>,
    mcp_generation: Option<u64>,
    mcp_tool_names: HashSet<ToolName>,
}

pub(crate) struct ToolRouterParams<'a> {
    pub(crate) mcp_tools: Option<Vec<ToolInfo>>,
    pub(crate) deferred_mcp_tools: Option<Vec<ToolInfo>>,
    pub(crate) discoverable_tools: Option<Vec<DiscoverableTool>>,
    pub(crate) extension_tool_executors: Vec<Arc<dyn ToolExecutor<ExtensionToolCall>>>,
    pub(crate) dynamic_tools: &'a [DynamicToolSpec],
}

impl ToolRouter {
    pub fn from_config(config: &ToolsConfig, params: ToolRouterParams<'_>) -> Self {
        build_tool_router(config, params)
    }

    pub(crate) fn from_parts(registry: ToolRegistry, model_visible_specs: Vec<ToolSpec>) -> Self {
        Self {
            registry,
            model_visible_specs,
            mcp_generation: None,
            mcp_tool_names: HashSet::new(),
        }
    }

    pub(crate) fn bind_mcp_generation(
        mut self,
        generation: u64,
        tool_names: HashSet<ToolName>,
    ) -> Self {
        self.mcp_generation = Some(generation);
        self.mcp_tool_names = tool_names;
        self
    }

    pub fn model_visible_specs(&self) -> Vec<ToolSpec> {
        self.model_visible_specs.clone()
    }

    pub(crate) fn create_diff_consumer(
        &self,
        tool_name: &ToolName,
    ) -> Option<Box<dyn ToolArgumentDiffConsumer>> {
        self.registry.create_diff_consumer(tool_name)
    }

    pub fn tool_supports_parallel(&self, call: &ToolCall) -> bool {
        self.registry
            .supports_parallel_tool_calls(&call.tool_name)
            .unwrap_or(false)
    }

    #[instrument(level = "trace", skip_all, err)]
    pub fn build_tool_call(item: ResponseItem) -> Result<Option<ToolCall>, FunctionCallError> {
        match item {
            ResponseItem::FunctionCall {
                name,
                namespace,
                arguments,
                call_id,
                ..
            } => {
                let tool_name = ToolName::new(namespace, name);
                Ok(Some(ToolCall {
                    tool_name,
                    call_id,
                    payload: ToolPayload::Function { arguments },
                }))
            }
            ResponseItem::ToolSearchCall {
                call_id: Some(call_id),
                execution,
                arguments,
                ..
            } if execution == "client" => {
                let arguments: SearchToolCallParams =
                    serde_json::from_value(arguments).map_err(|err| {
                        FunctionCallError::RespondToModel(format!(
                            "failed to parse tool_search arguments: {err}"
                        ))
                    })?;
                Ok(Some(ToolCall {
                    tool_name: ToolName::plain("tool_search"),
                    call_id,
                    payload: ToolPayload::ToolSearch { arguments },
                }))
            }
            ResponseItem::ToolSearchCall { .. } => Ok(None),
            ResponseItem::CustomToolCall {
                name,
                input,
                call_id,
                ..
            } => Ok(Some(ToolCall {
                tool_name: ToolName::plain(name),
                call_id,
                payload: ToolPayload::Custom { input },
            })),
            _ => Ok(None),
        }
    }

    #[instrument(level = "trace", skip_all, err)]
    pub async fn dispatch_tool_call_with_code_mode_result(
        &self,
        session: Arc<Session>,
        turn: Arc<TurnContext>,
        cancellation_token: CancellationToken,
        tracker: SharedTurnDiffTracker,
        call: ToolCall,
        source: ToolCallSource,
    ) -> Result<AnyToolResult, FunctionCallError> {
        let ToolCall {
            tool_name,
            call_id,
            payload,
        } = call;
        let manager = Arc::clone(&session.services.mcp_connection_manager);
        let mcp_generation_guard = if self.mcp_tool_names.contains(&tool_name) {
            if !turn.config.config_generation().is_current() {
                return Err(FunctionCallError::RespondToModel(
                    "MCP configuration source generation is stale; retry on the current runtime"
                        .to_string(),
                ));
            }
            let guard = manager.read().await;
            if self.mcp_generation != Some(guard.generation()) {
                return Err(FunctionCallError::RespondToModel(
                    "MCP tool catalog generation is stale; retry on the current runtime"
                        .to_string(),
                ));
            }
            Some(guard)
        } else {
            None
        };

        let invocation = ToolInvocation {
            session,
            turn,
            cancellation_token,
            tracker,
            call_id,
            tool_name,
            source,
            payload,
        };

        let result = self.registry.dispatch_any(invocation).await;
        drop(mcp_generation_guard);
        result
    }
}

pub(crate) fn extension_tool_executors(
    session: &Session,
    step_store: &codex_extension_api::ExtensionData,
) -> Vec<Arc<dyn ToolExecutor<ExtensionToolCall>>> {
    session
        .services
        .extensions
        .tool_contributors()
        .iter()
        .flat_map(|contributor| {
            contributor.tools_for_step(
                &session.services.session_extension_data,
                &session.services.thread_extension_data,
                step_store,
            )
        })
        .collect()
}

#[cfg(test)]
#[path = "router_tests.rs"]
mod tests;
