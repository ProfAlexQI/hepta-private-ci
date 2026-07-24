/// Stable schema material describing one callable tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSchema {
    /// Stable tool name exposed to planners and providers.
    pub name: String,
    /// Human-readable description of the tool contract.
    pub description: String,
    /// Canonical JSON Schema for the tool input.
    pub input_schema_json: String,
    /// Canonical JSON Schema for the tool output.
    pub output_schema_json: String,
}
