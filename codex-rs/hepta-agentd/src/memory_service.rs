use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_contracts::MemoryReadCapability;
use codex_hepta_memory::CognitiveRuntime;

use crate::AgentdError;
use crate::AgentdIdentity;
use crate::AgentdState;

pub(crate) struct AgentMemoryService {
    runtime: CognitiveRuntime,
    _read: Authorized<MemoryReadCapability>,
    _cognitive_write: Option<Authorized<CognitiveWriteCapability>>,
}

impl AgentMemoryService {
    pub(crate) async fn open(
        state: &AgentdState,
        identity: &AgentdIdentity,
        federation_owner_layouts: Vec<codex_hepta_paths::HeptaAgentLayout>,
        authority: &AuthorityGrant,
    ) -> Result<Self, AgentdError> {
        authority
            .validate_binding(&identity.agent_id, identity.spawn_generation)
            .map_err(|error| {
                AgentdError::Protocol(format!("validate Memory authority binding: {error}"))
            })?;
        let read = authority.authorize::<MemoryReadCapability>().map_err(|error| {
            AgentdError::Protocol(format!("authorize Memory read service: {error}"))
        })?;
        let cognitive_write = if authority.allows(AuthorityAction::WriteCognitiveState) {
            Some(
                authority
                    .authorize::<CognitiveWriteCapability>()
                    .map_err(|error| {
                        AgentdError::Protocol(format!(
                            "authorize Memory cognitive-write service: {error}"
                        ))
                    })?,
            )
        } else {
            None
        };

        state.refresh_generation()?;
        let mut runtime = CognitiveRuntime::open_agent_owned(&identity.layout, authority)
            .await
            .map_err(|error| {
                AgentdError::Protocol(format!("open Agent Memory runtime: {error}"))
            })?;
        state.refresh_generation()?;

        if cognitive_write.is_some()
            && !runtime
                .cognitive_write_store_available(authority)
                .map_err(|error| {
                    AgentdError::Protocol(format!("bind Memory cognitive write: {error}"))
                })?
        {
            return Err(AgentdError::QualificationCognitiveRuntimeUnavailable);
        }
        if let Some(store) = runtime.available_store() {
            state.attach_cognitive_store(Arc::clone(store))?;
        }

        if runtime.available_store().is_some() && !federation_owner_layouts.is_empty() {
            state.refresh_generation()?;
            runtime = runtime
                .with_discovered_federation(
                    identity.agent_id.clone(),
                    federation_owner_layouts,
                    now_unix_seconds()?,
                    authority,
                )
                .await
                .map_err(|error| {
                    AgentdError::Protocol(format!("discover Memory federation: {error}"))
                })?;
            state.refresh_generation()?;
        }

        Ok(Self {
            runtime,
            _read: read,
            _cognitive_write: cognitive_write,
        })
    }

    #[cfg(test)]
    pub(crate) fn is_available(&self) -> bool {
        self.runtime.available_store().is_some()
    }

    #[cfg(test)]
    pub(crate) fn cognitive_write_enabled(&self) -> bool {
        self._cognitive_write.is_some()
    }

    pub(crate) fn into_runtime(self) -> CognitiveRuntime {
        self.runtime
    }
}

fn now_unix_seconds() -> Result<i64, AgentdError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| AgentdError::Protocol(error.to_string()))?
        .as_secs();
    i64::try_from(now).map_err(|_| AgentdError::Protocol("system clock overflow".to_string()))
}
