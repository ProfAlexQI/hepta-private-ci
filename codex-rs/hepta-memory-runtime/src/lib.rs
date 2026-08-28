#![forbid(unsafe_code)]

use std::fmt;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityError;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_contracts::MemoryReadCapability;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_memory::CognitiveRuntime;
use codex_hepta_paths::HeptaAgentLayout;

/// Physical product boundary between Agent composition and the legacy
/// `codex-hepta-memory` implementation crate.
///
/// This facade owns no database and invents no authority. It accepts the exact
/// composition-root grant, opens the existing Agent-private runtime, and
/// exposes only the bounded runtime operations needed by Agentd. Store, KG,
/// compact, retrieval, and learning extraction can proceed behind this stable
/// product boundary without growing Agentd's dependency surface.
#[derive(Clone)]
pub struct AgentMemoryRuntime {
    owner_agent_id: AgentId,
    authority_grant_sha256: Sha256Digest,
    inner: CognitiveRuntime,
}

impl AgentMemoryRuntime {
    pub async fn open(
        owner_agent_id: AgentId,
        layout: &HeptaAgentLayout,
        authority: &AuthorityGrant,
    ) -> Result<Self, MemoryRuntimeBoundaryError> {
        authority
            .validate_binding(&owner_agent_id, authority.generation())
            .map_err(MemoryRuntimeBoundaryError::Authority)?;
        let _memory_read = authority
            .authorize::<MemoryReadCapability>()
            .map_err(MemoryRuntimeBoundaryError::Authority)?;
        let inner = CognitiveRuntime::open_agent_owned(layout, authority)
            .await
            .map_err(MemoryRuntimeBoundaryError::Authority)?;
        Ok(Self {
            owner_agent_id,
            authority_grant_sha256: authority.digest(),
            inner,
        })
    }

    pub async fn with_discovered_federation(
        self,
        owner_layouts: Vec<HeptaAgentLayout>,
        observed_at_unix_seconds: i64,
        authority: &AuthorityGrant,
    ) -> Result<Self, MemoryRuntimeBoundaryError> {
        self.verify_authority(authority)?;
        let inner = self
            .inner
            .with_discovered_federation(
                self.owner_agent_id.clone(),
                owner_layouts,
                observed_at_unix_seconds,
                authority,
            )
            .await
            .map_err(MemoryRuntimeBoundaryError::Authority)?;
        Ok(Self { inner, ..self })
    }

    pub fn require_cognitive_write(
        &self,
        authority: &AuthorityGrant,
    ) -> Result<Authorized<CognitiveWriteCapability>, MemoryRuntimeBoundaryError> {
        self.verify_authority(authority)?;
        authority
            .authorize::<CognitiveWriteCapability>()
            .map_err(MemoryRuntimeBoundaryError::Authority)
    }

    pub fn cognitive_write_store_available(
        &self,
        authority: &AuthorityGrant,
    ) -> Result<bool, MemoryRuntimeBoundaryError> {
        let _write = self.require_cognitive_write(authority)?;
        self.inner
            .cognitive_write_store_available(authority)
            .map_err(MemoryRuntimeBoundaryError::Authority)
    }

    pub fn cognitive_runtime(&self) -> &CognitiveRuntime {
        &self.inner
    }

    pub fn into_cognitive_runtime(self) -> CognitiveRuntime {
        self.inner
    }

    pub fn owner_agent_id(&self) -> &AgentId {
        &self.owner_agent_id
    }

    pub fn authority_grant_sha256(&self) -> &Sha256Digest {
        &self.authority_grant_sha256
    }

    fn verify_authority(
        &self,
        authority: &AuthorityGrant,
    ) -> Result<(), MemoryRuntimeBoundaryError> {
        authority
            .validate_binding(&self.owner_agent_id, authority.generation())
            .map_err(MemoryRuntimeBoundaryError::Authority)?;
        if authority.digest() != self.authority_grant_sha256 {
            return Err(MemoryRuntimeBoundaryError::AuthorityDrift);
        }
        Ok(())
    }
}

impl fmt::Debug for AgentMemoryRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentMemoryRuntime")
            .field("owner_agent_id", &self.owner_agent_id)
            .field("authority_grant_sha256", &self.authority_grant_sha256)
            .field("inner", &self.inner)
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryRuntimeBoundaryError {
    Authority(AuthorityError),
    AuthorityDrift,
}

impl fmt::Display for MemoryRuntimeBoundaryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authority(error) => write!(formatter, "memory runtime authority rejected: {error}"),
            Self::AuthorityDrift => {
                formatter.write_str("memory runtime authority grant digest drifted")
            }
        }
    }
}

impl std::error::Error for MemoryRuntimeBoundaryError {}

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityAction;
    use codex_hepta_contracts::AuthorityGrant;

    use super::AgentMemoryRuntime;
    use super::MemoryRuntimeBoundaryError;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn agent_id() -> AgentId {
        AgentId::parse(AGENT_ID)
            .unwrap_or_else(|error| panic!("test AgentId must parse: {error}"))
    }

    fn absent_runtime(authority: &AuthorityGrant) -> AgentMemoryRuntime {
        AgentMemoryRuntime {
            owner_agent_id: agent_id(),
            authority_grant_sha256: authority.digest(),
            inner: codex_hepta_memory::CognitiveRuntime::Absent,
        }
    }

    #[test]
    fn agent_local_facade_cannot_mint_cognitive_write() {
        let authority = AuthorityGrant::agent_local(agent_id(), 1)
            .unwrap_or_else(|error| panic!("Agent authority must be valid: {error}"));
        let runtime = absent_runtime(&authority);
        assert!(matches!(
            runtime.require_cognitive_write(&authority),
            Err(MemoryRuntimeBoundaryError::Authority(
                codex_hepta_contracts::AuthorityError::ActionDenied(
                    AuthorityAction::WriteCognitiveState
                )
            ))
        ));
    }

    #[test]
    fn facade_rejects_changed_authority_digest() {
        let first = AuthorityGrant::agent_local(agent_id(), 1)
            .unwrap_or_else(|error| panic!("first authority must be valid: {error}"));
        let second = AuthorityGrant::agent_local(agent_id(), 2)
            .unwrap_or_else(|error| panic!("second authority must be valid: {error}"));
        let runtime = absent_runtime(&first);
        assert!(matches!(
            runtime.require_cognitive_write(&second),
            Err(MemoryRuntimeBoundaryError::AuthorityDrift)
        ));
    }

    #[test]
    fn qualification_facade_requires_store_after_typed_authorization() {
        let authority = AuthorityGrant::qualification_cognitive_write(agent_id(), 1)
            .unwrap_or_else(|error| panic!("qualification authority must be valid: {error}"));
        let runtime = absent_runtime(&authority);
        assert_eq!(
            runtime.cognitive_write_store_available(&authority),
            Ok(false)
        );
    }
}
