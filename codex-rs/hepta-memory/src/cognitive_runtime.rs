use std::fmt;
use std::sync::Arc;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityError;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_contracts::MemoryReadCapability;
use codex_hepta_paths::HeptaAgentLayout;

use crate::CognitiveCompactError;
use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::CompactCheckpoint;
use crate::CompactCommitDecision;
use crate::CompactLease;
use crate::CompactParentSnapshot;
use crate::FederatedRecallSet;
use crate::RehydrationPlan;

/// Sanitized reason why an owning runtime could not open its Cognitive Plane.
///
/// This type deliberately carries no filesystem paths, database text, or raw
/// error strings, so it is safe to expose through health and tool surfaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CognitiveUnavailableReason {
    InvalidStoreConfiguration,
    AccessDenied,
    RevisionConflict,
    CorruptStore,
    StorageUnavailable,
}

impl CognitiveUnavailableReason {
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidStoreConfiguration => "invalid_store_configuration",
            Self::AccessDenied => "access_denied",
            Self::RevisionConflict => "revision_conflict",
            Self::CorruptStore => "corrupt_store",
            Self::StorageUnavailable => "storage_unavailable",
        }
    }
}

impl From<&CognitiveStoreError> for CognitiveUnavailableReason {
    fn from(error: &CognitiveStoreError) -> Self {
        match error {
            CognitiveStoreError::Invalid(_) => Self::InvalidStoreConfiguration,
            CognitiveStoreError::AccessDenied(_) => Self::AccessDenied,
            CognitiveStoreError::Conflict(_) => Self::RevisionConflict,
            CognitiveStoreError::Corrupt(_) => Self::CorruptStore,
            CognitiveStoreError::Unavailable(_) => Self::StorageUnavailable,
        }
    }
}

/// Process-scoped Cognitive Plane capability supplied to Codex extensions.
///
/// Plain Codex uses `Absent`. An owning Hepta agent uses `Available` after a
/// successful open, or `Unavailable` when opening failed. The latter remains
/// distinct from absence so typed tools can stay visible and report a bounded
/// unavailable state while normal Codex execution continues.
#[derive(Clone, Default)]
pub enum CognitiveRuntime {
    #[default]
    Absent,
    Available(Arc<CognitiveStore>),
    AvailableFederated {
        store: Arc<CognitiveStore>,
        federation: Arc<FederatedRecallSet>,
    },
    Unavailable(CognitiveUnavailableReason),
}

impl CognitiveRuntime {
    pub fn from_open_result(result: Result<CognitiveStore, CognitiveStoreError>) -> Self {
        match result {
            Ok(store) => Self::Available(Arc::new(store)),
            Err(error) => Self::Unavailable(CognitiveUnavailableReason::from(&error)),
        }
    }

    /// Opens the Agent-private memory runtime through the unified authority
    /// kernel. Store failures remain a typed unavailable runtime so the normal
    /// read-only Agent profile can degrade without inventing write authority.
    pub async fn open_agent_owned(
        layout: &HeptaAgentLayout,
        authority: &AuthorityGrant,
    ) -> Result<Self, AuthorityError> {
        let _memory_read = authority.authorize::<MemoryReadCapability>()?;
        Ok(Self::from_open_result(CognitiveStore::open(layout).await))
    }

    /// Discovers read-only federation sources while preserving the existing
    /// owning store. The caller must fence its lifecycle generation before and
    /// after this await; this facade deliberately does not own fleet state.
    pub async fn with_discovered_federation(
        self,
        owner_agent_id: AgentId,
        owner_layouts: Vec<HeptaAgentLayout>,
        observed_at_unix_seconds: i64,
        authority: &AuthorityGrant,
    ) -> Result<Self, AuthorityError> {
        let _memory_read = authority.authorize::<MemoryReadCapability>()?;
        if self.available_store().is_none() || owner_layouts.is_empty() {
            return Ok(self);
        }
        let federation =
            FederatedRecallSet::discover(owner_agent_id, owner_layouts, observed_at_unix_seconds)
                .await;
        Ok(self.with_federation(federation))
    }

    /// Verifies the typed write capability and reports whether an owning store
    /// is present. It does not create a writer, mutate SQLite, or grant any
    /// effect/provider/fleet authority.
    pub fn cognitive_write_store_available(
        &self,
        authority: &AuthorityGrant,
    ) -> Result<bool, AuthorityError> {
        let _cognitive_write = authority.authorize::<CognitiveWriteCapability>()?;
        Ok(self.available_store().is_some())
    }

    pub fn available_store(&self) -> Option<&Arc<CognitiveStore>> {
        match self {
            Self::Available(store) => Some(store),
            Self::AvailableFederated { store, .. } => Some(store),
            Self::Absent | Self::Unavailable(_) => None,
        }
    }

    pub fn with_federation(self, federation: FederatedRecallSet) -> Self {
        if federation.is_empty() {
            return self;
        }
        match self {
            Self::Available(store) => Self::AvailableFederated {
                store,
                federation: Arc::new(federation),
            },
            Self::AvailableFederated { store, .. } => Self::AvailableFederated {
                store,
                federation: Arc::new(federation),
            },
            Self::Absent | Self::Unavailable(_) => self,
        }
    }

    pub fn federation(&self) -> Option<&Arc<FederatedRecallSet>> {
        match self {
            Self::AvailableFederated { federation, .. } => Some(federation),
            Self::Absent | Self::Available(_) | Self::Unavailable(_) => None,
        }
    }

    pub fn unavailable_reason(&self) -> Option<CognitiveUnavailableReason> {
        match self {
            Self::Unavailable(reason) => Some(*reason),
            Self::Absent | Self::Available(_) | Self::AvailableFederated { .. } => None,
        }
    }

    /// Captures a typed, local-development-only pre-compact lease envelope.
    ///
    /// This is a read-only contract seam: it does not acquire a lease, append
    /// an event, mutate SQLite, or authorize compaction. The Agent-local
    /// authoritative writer must perform the real CAS transaction.
    pub fn pre_compact(
        &self,
        snapshot: CompactParentSnapshot,
    ) -> Result<CompactLease, CognitiveCompactError> {
        self.require_compact_runtime()?;
        Ok(CompactLease::from_snapshot(snapshot))
    }

    /// Builds a typed post-compact rehydration plan without writing state.
    ///
    /// A plan starts as NotStarted; callers must execute and acknowledge
    /// rehydration through the owning event store before treating it as
    /// complete.
    pub fn post_compact(
        &self,
        checkpoint: &CompactCheckpoint,
        expected_revision: u64,
    ) -> Result<RehydrationPlan, CognitiveCompactError> {
        self.require_compact_runtime()?;
        checkpoint.rehydration_plan(expected_revision)
    }

    /// Performs read-only parent CAS and generation/fence validation for a
    /// checkpoint. No commit or projection write occurs here.
    pub fn validate_compact_commit(
        &self,
        checkpoint: &CompactCheckpoint,
        current: &CompactParentSnapshot,
    ) -> Result<CompactCommitDecision, CognitiveCompactError> {
        self.require_compact_runtime()?;
        Ok(checkpoint.validate_against(current))
    }

    fn require_compact_runtime(&self) -> Result<(), CognitiveCompactError> {
        match self {
            Self::Absent => Err(CognitiveCompactError::RuntimeAbsent),
            Self::Unavailable(reason) => {
                Err(CognitiveCompactError::RuntimeUnavailable { reason: *reason })
            }
            Self::Available(_) | Self::AvailableFederated { .. } => Ok(()),
        }
    }
}

impl fmt::Debug for CognitiveRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absent => formatter.write_str("CognitiveRuntime::Absent"),
            Self::Available(_) => formatter.write_str("CognitiveRuntime::Available(<owned store>)"),
            Self::AvailableFederated { .. } => formatter.write_str(
                "CognitiveRuntime::AvailableFederated(<owned store>, <read-only sources>)",
            ),
            Self::Unavailable(reason) => formatter
                .debug_tuple("CognitiveRuntime::Unavailable")
                .field(reason)
                .finish(),
        }
    }
}

#[cfg(test)]
mod authority_tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityAction;
    use codex_hepta_contracts::AuthorityError;
    use codex_hepta_contracts::AuthorityGrant;

    use super::CognitiveRuntime;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn agent_id() -> AgentId {
        match AgentId::parse(AGENT_ID) {
            Ok(agent_id) => agent_id,
            Err(error) => panic!("test AgentId must parse: {error}"),
        }
    }

    #[test]
    fn agent_local_runtime_cannot_acquire_cognitive_write() {
        let authority = match AuthorityGrant::agent_local(agent_id(), 1) {
            Ok(authority) => authority,
            Err(error) => panic!("agent authority must be valid: {error}"),
        };
        assert!(matches!(
            CognitiveRuntime::Absent.cognitive_write_store_available(&authority),
            Err(AuthorityError::ActionDenied(AuthorityAction::WriteCognitiveState))
        ));
    }

    #[test]
    fn qualification_write_still_requires_an_open_store() {
        let authority = match AuthorityGrant::qualification_cognitive_write(agent_id(), 1) {
            Ok(authority) => authority,
            Err(error) => panic!("qualification authority must be valid: {error}"),
        };
        assert_eq!(
            CognitiveRuntime::Absent.cognitive_write_store_available(&authority),
            Ok(false)
        );
    }
}
