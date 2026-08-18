use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::FleetRegistryError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{message}")]
pub struct ProcessDriverError {
    message: String,
}

impl ProcessDriverError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<std::io::Error> for ProcessDriverError {
    fn from(error: std::io::Error) -> Self {
        Self::new(error.to_string())
    }
}

#[derive(Debug, Error)]
pub enum SupervisorError {
    #[error("invalid supervisor value: {0}")]
    Invalid(String),
    #[error("unknown fleet agent {0}")]
    UnknownAgent(AgentId),
    #[error("agent {0} already has an active child")]
    AlreadyActive(AgentId),
    #[error("agent {0} has no previous child command")]
    NoPreviousCommand(AgentId),
    #[error("agent {0} has an unresolved process lease")]
    UnresolvedLease(AgentId),
    #[error("corrupt supervisor process lease: {0}")]
    CorruptLease(String),
    #[error("process driver failed for agent {agent_id}: {message}")]
    Driver { agent_id: AgentId, message: String },
    #[error("generation fence rejected agent {agent_id}: runtime {runtime}, registry {registry}")]
    GenerationFence {
        agent_id: AgentId,
        runtime: u64,
        registry: u64,
    },
    #[error(transparent)]
    Registry(#[from] FleetRegistryError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
