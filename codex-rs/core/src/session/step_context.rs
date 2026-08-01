use std::sync::Arc;

use codex_mcp::McpBinding;

use crate::session::turn_context::TurnContext;
use crate::tools::router::ToolRouter;

/// Request-scoped state that may change between model sampling requests.
pub(crate) struct StepContext {
    pub(crate) turn: Arc<TurnContext>,
    /// The exact MCP catalog and published-manager generation for this step.
    pub(crate) mcp: Arc<McpBinding>,
    /// The finalized tool plan advertised and executed for this step.
    pub(crate) tool_router: Arc<ToolRouter>,
}

impl StepContext {
    pub(crate) fn new(
        turn: Arc<TurnContext>,
        mcp: Arc<McpBinding>,
        tool_router: Arc<ToolRouter>,
    ) -> Self {
        Self {
            turn,
            mcp,
            tool_router,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test(turn: Arc<TurnContext>, tool_router: Arc<ToolRouter>) -> Self {
        Self::new(
            turn,
            Arc::new(McpBinding::from_tools(
                0,
                &[],
                &[],
                /*has_servers*/ false,
            )),
            tool_router,
        )
    }
}
