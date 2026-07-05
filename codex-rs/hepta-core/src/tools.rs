use serde::Deserialize;
use serde::Serialize;

use crate::runtime_types::CorrelationId;
use crate::runtime_types::SessionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskTier {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionProfile {
    FullAccess,
    ReadOnlyTools,
    NoTools,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilesystemScope {
    WorkspaceOnly,
    AnyPath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritePathScope {
    ArtifactsOnly,
    WorkspaceOnly,
    AnyPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathCapabilityGate {
    pub id: String,
    pub tool_name: String,
    pub argument_name: String,
    pub scope: FilesystemScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolExecutionMetadata {
    pub read_only: bool,
    pub destructive: bool,
    pub idempotent: bool,
    pub produces_structured_output: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub input_schema_json: String,
    pub output_schema_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolContext {
    pub session_id: Option<SessionId>,
    pub correlation_id: Option<CorrelationId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallRequest {
    pub name: String,
    pub input_json: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolResult {
    pub content: String,
    pub structured_json: Option<String>,
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn risk_tier(&self) -> RiskTier;
    fn execution_metadata(&self) -> ToolExecutionMetadata;
    fn schema(&self) -> ToolSchema;
    async fn invoke(
        &self,
        ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, crate::ToolError>;
}
