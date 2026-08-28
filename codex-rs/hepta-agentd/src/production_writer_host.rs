//! Explicit Agentd/host seam for the production durable writer.
//!
//! The host must supply an externally verified authority lease and verifier.
//! Nothing in Agentd startup installs this capability automatically; the
//! default runtime remains read-only. A dispatcher target is likewise an
//! explicit attachment and dispatch fails closed while it is absent.

use std::fmt;
use std::sync::Arc;

use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_memory::CognitiveStore;
use codex_hepta_memory::ProductionAuthorityLease;
use codex_hepta_memory::ProductionAuthorityVerifier;
use codex_hepta_memory::ProductionDispatchReceipt;
use codex_hepta_memory::ProductionDurableWriter;
use codex_hepta_memory::ProductionOutboxDispatcher;
use codex_hepta_memory::ProductionOutboxTarget;
use codex_hepta_memory::ProductionQueuedReceipt;
use codex_hepta_memory::ProductionWriterError;

use crate::AgentdConfig;
use crate::AgentdError;
use crate::production_authority_adapter::ProductionCognitiveWriteAuthorization;

/// Host-owned production writer handle. Constructing this value does not
/// mutate Agentd's runtime configuration; callers explicitly attach/use it.
#[derive(Clone)]
pub struct AgentdProductionWriterHost {
    writer: Arc<ProductionDurableWriter>,
    cognitive_write: Authorized<CognitiveWriteCapability>,
    dispatcher: Option<ProductionOutboxDispatcher>,
}

impl fmt::Debug for AgentdProductionWriterHost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentdProductionWriterHost")
            .field("writer", &self.writer)
            .field(
                "cognitive_write_grant_sha256",
                self.cognitive_write.grant_sha256(),
            )
            .field("dispatcher_attached", &self.dispatcher.is_some())
            .finish()
    }
}

impl AgentdProductionWriterHost {
    /// Verify and mint the typed cognitive-write witness before opening the
    /// Cognitive store. The durable writer repeats its legacy verifier check
    /// before lease mutation, preserving backward compatibility while the
    /// typed authority kernel becomes the canonical host boundary.
    pub async fn open<V>(
        config: &AgentdConfig,
        authority: ProductionAuthorityLease,
        verifier: &V,
        lease_id: impl Into<String>,
        lease_generation: u64,
    ) -> Result<Self, AgentdError>
    where
        V: ProductionAuthorityVerifier + ?Sized,
    {
        let authorization = ProductionCognitiveWriteAuthorization::verify(
            authority,
            verifier,
            &config.identity().agent_id,
            lease_generation,
        )?;
        let (authority, cognitive_write) = authorization.into_parts();
        let store = CognitiveStore::open(&config.identity().layout)
            .await
            .map_err(|error| {
                AgentdError::Protocol(format!("open production cognitive store: {error}"))
            })?;
        let writer =
            ProductionDurableWriter::open(store, authority, verifier, lease_id, lease_generation)
                .await?;
        Ok(Self {
            writer: Arc::new(writer),
            cognitive_write,
            dispatcher: None,
        })
    }

    /// Build a host handle around an already-open Agentd-owned store. This is
    /// useful when the runtime has already attached a CognitiveStore and keeps
    /// the same mandatory external verifier contract.
    pub async fn open_with_store<V>(
        store: CognitiveStore,
        authority: ProductionAuthorityLease,
        verifier: &V,
        lease_id: impl Into<String>,
        lease_generation: u64,
    ) -> Result<Self, ProductionWriterError>
    where
        V: ProductionAuthorityVerifier + ?Sized,
    {
        let authorization = ProductionCognitiveWriteAuthorization::verify(
            authority,
            verifier,
            store.owner_agent_id(),
            lease_generation,
        )?;
        let (authority, cognitive_write) = authorization.into_parts();
        let writer =
            ProductionDurableWriter::open(store, authority, verifier, lease_id, lease_generation)
                .await?;
        Ok(Self {
            writer: Arc::new(writer),
            cognitive_write,
            dispatcher: None,
        })
    }

    pub fn writer(&self) -> Arc<ProductionDurableWriter> {
        Arc::clone(&self.writer)
    }

    pub fn cognitive_write_capability(&self) -> &Authorized<CognitiveWriteCapability> {
        &self.cognitive_write
    }

    /// Attach the provider/host target explicitly. Replacing a target is
    /// allowed only through a new host handle, avoiding an in-flight target
    /// swap behind the writer's back.
    pub fn attach_target(mut self, target: Arc<dyn ProductionOutboxTarget>) -> Self {
        self.dispatcher = Some(ProductionOutboxDispatcher::attach(target));
        self
    }

    pub fn has_target(&self) -> bool {
        self.dispatcher.is_some()
    }

    pub async fn dispatch(
        &self,
        receipt: ProductionQueuedReceipt,
    ) -> Result<ProductionDispatchReceipt, AgentdError> {
        let dispatcher = self.dispatcher.as_ref().ok_or_else(|| {
            AgentdError::Protocol(
                "production outbox dispatcher is not explicitly attached".to_string(),
            )
        })?;
        Ok(dispatcher.dispatch(self.writer.as_ref(), receipt).await?)
    }
}
