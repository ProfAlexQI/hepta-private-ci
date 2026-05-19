use crate::model::ModelRef;
use crate::runtime_types::ThinkingLevel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentConfig {
    pub id: String,
    pub primary_model: ModelRef,
    pub thinking_default: ThinkingLevel,
}
