//! Immutable MCP catalog state bound to one model sampling request.

use std::collections::HashSet;

use codex_protocol::ToolName;

use crate::ToolInfo;

/// The exact MCP catalog advertised for one model sampling request.
///
/// Core binds this snapshot to the published manager generation and holds the
/// matching manager read guard while an advertised MCP call executes. This
/// keeps catalog construction, direct execution, deferred execution, and MCP
/// resource helpers on one immutable authority boundary.
#[derive(Debug)]
pub struct McpBinding {
    generation: u64,
    has_servers: bool,
    tools: Vec<ToolInfo>,
    tool_names: HashSet<ToolName>,
}

impl McpBinding {
    pub fn from_tools(
        generation: u64,
        direct_tools: &[ToolInfo],
        deferred_tools: &[ToolInfo],
        has_servers: bool,
    ) -> Self {
        let tools = direct_tools
            .iter()
            .chain(deferred_tools)
            .cloned()
            .collect::<Vec<_>>();
        let mut tool_names = tools
            .iter()
            .map(ToolInfo::canonical_tool_name)
            .collect::<HashSet<_>>();
        if has_servers {
            tool_names.extend([
                ToolName::plain("list_mcp_resources"),
                ToolName::plain("list_mcp_resource_templates"),
                ToolName::plain("read_mcp_resource"),
            ]);
        }
        Self {
            generation,
            has_servers,
            tools,
            tool_names,
        }
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn has_servers(&self) -> bool {
        self.has_servers
    }

    pub fn tools(&self) -> &[ToolInfo] {
        &self.tools
    }

    pub fn contains(&self, tool_name: &ToolName) -> bool {
        self.tool_names.contains(tool_name)
    }
}
