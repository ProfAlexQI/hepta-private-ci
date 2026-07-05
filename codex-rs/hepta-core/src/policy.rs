use serde::Deserialize;
use serde::Serialize;

use crate::ModelRef;
use crate::runtime_types::SessionId;
use crate::tools::RiskTier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequirement {
    None,
    Ask,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub requirement: ApprovalRequirement,
    pub reason: String,
    pub matched_rule_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyRule {
    pub id: String,
    pub session_id: Option<String>,
    pub provider_name: Option<String>,
    pub tool_name: Option<String>,
    pub risk_tier: Option<RiskTier>,
    pub requirement: ApprovalRequirement,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyEvaluationContext {
    pub session_id: Option<SessionId>,
    pub model: Option<ModelRef>,
    pub tool_name: String,
    pub risk_tier: RiskTier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionRule {
    pub target: String,
    pub requirement: ApprovalRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandboxProfile {
    pub name: String,
    pub allow_network: bool,
}

pub trait PolicyEngine: Send + Sync {
    async fn evaluate_tool(
        &self,
        context: PolicyEvaluationContext,
    ) -> Result<PolicyDecision, crate::PolicyError>;
}
