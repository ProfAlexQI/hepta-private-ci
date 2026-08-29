use crate::Digest;
use crate::InferError;
use crate::Result;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoritySnapshot {
    pub qualification_only: bool,
    pub production_listener: bool,
    pub production_writer: bool,
    pub provider_effect: bool,
    pub external_effect: bool,
    pub shared_kg_write: bool,
    pub memory_write: bool,
    pub route_write: bool,
    pub fleet_write: bool,
    pub model_npu: bool,
    pub remote_inference: bool,
    pub automatic_model_install: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl AuthoritySnapshot {
    pub const fn qualification_only_closed() -> Self {
        Self {
            qualification_only: true,
            production_listener: false,
            production_writer: false,
            provider_effect: false,
            external_effect: false,
            shared_kg_write: false,
            memory_write: false,
            route_write: false,
            fleet_write: false,
            model_npu: false,
            remote_inference: false,
            automatic_model_install: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    pub fn validate_closed(&self) -> Result<()> {
        if self == &Self::qualification_only_closed() {
            Ok(())
        } else {
            Err(InferError::AuthorityEscalation)
        }
    }
}

impl Default for AuthoritySnapshot {
    fn default() -> Self {
        Self::qualification_only_closed()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InferenceRequest {
    pub identity: crate::RequestIdentity,
    pub agent_generation: u64,
    pub request_generation: u64,
    pub cancel_generation: u64,
    pub deadline_unix_ms: u64,
    pub model_tuple_digest: Digest,
    pub policy_digest: Digest,
    pub resource_budget_id: crate::ResourceBudgetId,
    pub prompt_digest: Digest,
    pub prompt_byte_length: u64,
    pub output_token_limit: u32,
    pub authority: AuthoritySnapshot,
}

impl InferenceRequest {
    pub fn validate_shape(&self) -> Result<()> {
        self.authority.validate_closed()?;
        if self.agent_generation == 0 || self.request_generation == 0 {
            return Err(InferError::InvalidGeneration);
        }
        if self.prompt_byte_length == 0 {
            return Err(InferError::EmptyPrompt);
        }
        if self.output_token_limit == 0 {
            return Err(InferError::EmptyOutputLimit);
        }
        Ok(())
    }
}
