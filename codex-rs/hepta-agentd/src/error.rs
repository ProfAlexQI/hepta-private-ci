use codex_hepta_automation::AutomationError;
use codex_hepta_fleet::FleetRegistryError;

#[derive(Debug, thiserror::Error)]
pub enum AgentdError {
    #[error("invalid agentd configuration: {0}")]
    Invalid(String),
    #[error("agentd generation fenced: {0}")]
    GenerationFenced(String),
    #[error("agentd protocol error: {0}")]
    Protocol(String),
    #[error(transparent)]
    Fleet(#[from] FleetRegistryError),
    #[error(transparent)]
    Automation(#[from] AutomationError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
