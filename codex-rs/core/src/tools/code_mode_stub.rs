#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value as JsonValue;

use crate::function_tool::FunctionCallError;
use crate::session::session::Session;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::SharedTurnDiffTracker;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::parallel::StepToolPlan;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use codex_tools::ToolName;
use codex_tools::ToolSpec;

pub(crate) const PUBLIC_TOOL_NAME: &str = "exec";
pub(crate) const WAIT_TOOL_NAME: &str = "wait";
pub(crate) const DEFAULT_WAIT_YIELD_TIME_MS: u64 = 1000;

pub(crate) fn is_exec_tool_name(tool_name: &ToolName) -> bool {
    tool_name.namespace.is_none() && tool_name.name == PUBLIC_TOOL_NAME
}

pub(crate) struct CodeModeTurnWorker;

pub(crate) struct CodeModeService;

impl CodeModeService {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn stored_values(&self) -> HashMap<String, JsonValue> {
        HashMap::new()
    }

    pub(crate) async fn replace_stored_values(&self, _values: HashMap<String, JsonValue>) {}

    pub(crate) fn allocate_cell_id(&self) -> String {
        "code-mode-disabled".to_string()
    }

    pub(crate) async fn shutdown(&self) -> Result<(), String> {
        Ok(())
    }

    pub(crate) async fn start_turn_worker(
        &self,
        _session: &Arc<Session>,
        _step_tool_plan: Arc<StepToolPlan>,
        _tracker: SharedTurnDiffTracker,
    ) -> Option<CodeModeTurnWorker> {
        None
    }
}

pub struct CodeModeExecuteHandler {
    spec: ToolSpec,
}

impl CodeModeExecuteHandler {
    pub(crate) fn new(spec: ToolSpec, _nested_tool_specs: Vec<ToolSpec>) -> Self {
        Self { spec }
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolInvocation> for CodeModeExecuteHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(PUBLIC_TOOL_NAME)
    }

    fn spec(&self) -> Option<ToolSpec> {
        Some(self.spec.clone())
    }

    async fn handle(
        &self,
        _invocation: ToolInvocation,
    ) -> Result<Box<dyn crate::tools::context::ToolOutput>, FunctionCallError> {
        Ok(boxed_tool_output(FunctionToolOutput::from_text(
            "code mode is disabled in this Hepta build".to_string(),
            Some(false),
        )))
    }
}

impl CoreToolRuntime for CodeModeExecuteHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Custom { .. })
    }
}

pub struct CodeModeWaitHandler;

#[async_trait::async_trait]
impl ToolExecutor<ToolInvocation> for CodeModeWaitHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(WAIT_TOOL_NAME)
    }

    async fn handle(
        &self,
        _invocation: ToolInvocation,
    ) -> Result<Box<dyn crate::tools::context::ToolOutput>, FunctionCallError> {
        Ok(boxed_tool_output(FunctionToolOutput::from_text(
            "code mode is disabled in this Hepta build".to_string(),
            Some(false),
        )))
    }
}

impl CoreToolRuntime for CodeModeWaitHandler {}
