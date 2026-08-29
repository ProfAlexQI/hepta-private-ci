use std::fmt;
use std::sync::Arc;

use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_memory::CognitiveStore;
use codex_hepta_memory::CognitiveStoreError;
use codex_hepta_memory::ProductionAuthorityVerifier;
use codex_hepta_memory::ProductionDurableWriter;
use codex_hepta_memory::ProductionWriterError;
use codex_hepta_paths::HeptaAgentLayout;

use crate::ProductionCognitiveWriteAuthorization;

/// Runtime-owned handle proving that the legacy externally verified lease was
/// attenuated to cognitive-write authority before the raw durable writer was
/// opened.
///
/// Agentd consumes this handle; it cannot call `ProductionDurableWriter::open`
/// directly. Provider/effect authority is deliberately absent and remains a
/// separate host boundary.
#[derive(Clone)]
pub struct AuthorizedProductionWriter {
    writer: Arc<ProductionDurableWriter>,
    cognitive_write: Authorized<CognitiveWriteCapability>,
}

impl AuthorizedProductionWriter {
    pub async fn open<V>(
        layout: &HeptaAgentLayout,
        authorization: ProductionCognitiveWriteAuthorization,
        verifier: &V,
        lease_id: impl Into<String>,
        lease_generation: u64,
    ) -> Result<Self, ProductionWriterRuntimeError>
    where
        V: ProductionAuthorityVerifier + ?Sized,
    {
        if authorization.capability().generation() != lease_generation {
            return Err(ProductionWriterRuntimeError::GenerationMismatch {
                capability: authorization.capability().generation(),
                requested: lease_generation,
            });
        }
        let store = CognitiveStore::open(layout)
            .await
            .map_err(ProductionWriterRuntimeError::Store)?;
        Self::open_with_store(
            store,
            authorization,
            verifier,
            lease_id,
            lease_generation,
        )
        .await
    }

    pub async fn open_with_store<V>(
        store: CognitiveStore,
        authorization: ProductionCognitiveWriteAuthorization,
        verifier: &V,
        lease_id: impl Into<String>,
        lease_generation: u64,
    ) -> Result<Self, ProductionWriterRuntimeError>
    where
        V: ProductionAuthorityVerifier + ?Sized,
    {
        if authorization.capability().subject_agent_id() != store.owner_agent_id() {
            return Err(ProductionWriterRuntimeError::AgentMismatch);
        }
        if authorization.capability().generation() != lease_generation {
            return Err(ProductionWriterRuntimeError::GenerationMismatch {
                capability: authorization.capability().generation(),
                requested: lease_generation,
            });
        }
        if !authorization.capability().is_external() {
            return Err(ProductionWriterRuntimeError::LocalAuthorityRejected);
        }
        let (authority, cognitive_write) = authorization.into_parts();
        let writer = ProductionDurableWriter::open(
            store,
            authority,
            verifier,
            lease_id,
            lease_generation,
        )
        .await
        .map_err(ProductionWriterRuntimeError::Writer)?;
        Ok(Self {
            writer: Arc::new(writer),
            cognitive_write,
        })
    }

    pub fn writer(&self) -> Arc<ProductionDurableWriter> {
        Arc::clone(&self.writer)
    }

    pub fn cognitive_write_capability(&self) -> &Authorized<CognitiveWriteCapability> {
        &self.cognitive_write
    }

    pub fn into_parts(
        self,
    ) -> (
        Arc<ProductionDurableWriter>,
        Authorized<CognitiveWriteCapability>,
    ) {
        (self.writer, self.cognitive_write)
    }
}

impl fmt::Debug for AuthorizedProductionWriter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthorizedProductionWriter")
            .field("writer", &self.writer)
            .field(
                "cognitive_write_grant_sha256",
                self.cognitive_write.grant_sha256(),
            )
            .field("generation", &self.cognitive_write.generation())
            .finish()
    }
}

#[derive(Debug)]
pub enum ProductionWriterRuntimeError {
    Store(CognitiveStoreError),
    Writer(ProductionWriterError),
    AgentMismatch,
    GenerationMismatch { capability: u64, requested: u64 },
    LocalAuthorityRejected,
}

impl fmt::Display for ProductionWriterRuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Store(error) => write!(formatter, "open cognitive store: {error}"),
            Self::Writer(error) => write!(formatter, "open durable writer: {error}"),
            Self::AgentMismatch => formatter.write_str(
                "typed cognitive-write capability does not match the cognitive store owner",
            ),
            Self::GenerationMismatch {
                capability,
                requested,
            } => write!(
                formatter,
                "typed cognitive-write generation {capability} does not match requested generation {requested}"
            ),
            Self::LocalAuthorityRejected => formatter.write_str(
                "production durable writer requires externally verified cognitive-write authority",
            ),
        }
    }
}

impl std::error::Error for ProductionWriterRuntimeError {}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::Sha256Digest;
    use codex_hepta_memory::ProductionAuthorityLease;
    use codex_hepta_memory::ProductionAuthorityToken;
    use codex_hepta_memory::ProductionAuthorityVerifier;
    use codex_hepta_paths::HeptaFleetRoot;

    use crate::ProductionCognitiveWriteAuthorization;

    use super::AuthorizedProductionWriter;
    use super::ProductionWriterRuntimeError;

    const OWNER_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn must<T, E>(result: Result<T, E>, label: &str) -> T
    where
        E: Display,
    {
        match result {
            Ok(value) => value,
            Err(error) => panic!("{label}: {error}"),
        }
    }

    fn agent_id() -> AgentId {
        must(AgentId::parse(OWNER_ID), "test AgentId must parse")
    }

    fn lease() -> ProductionAuthorityLease {
        let expiry = must(
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH),
            "clock must work",
        )
        .as_secs()
            + 3_600;
        must(
            ProductionAuthorityLease::from_verified_parts(
                agent_id(),
                Sha256Digest::for_bytes(b"signed-production-grant"),
                4,
                9,
                expiry,
                must(
                    ProductionAuthorityToken::from_verified_bytes(b"opaque-token".to_vec()),
                    "token must be valid",
                ),
            ),
            "lease must be valid",
        )
    }

    fn layout() -> (tempfile::TempDir, codex_hepta_paths::HeptaAgentLayout) {
        let temp = must(tempfile::tempdir(), "create temporary root");
        let fleet_root_path = temp.path().join("fleet");
        must(
            std::fs::create_dir_all(&fleet_root_path),
            "create fleet root",
        );
        let canonical = must(fleet_root_path.canonicalize(), "canonicalize fleet root");
        let fleet_root = must(HeptaFleetRoot::parse(canonical), "parse fleet root");
        let layout = fleet_root.layout().agent(&agent_id());
        (temp, layout)
    }

    struct AllowVerifier;

    impl ProductionAuthorityVerifier for AllowVerifier {
        fn verify(
            &self,
            authority: &ProductionAuthorityLease,
            expected_agent: &AgentId,
        ) -> Result<(), String> {
            if &authority.agent_id != expected_agent {
                return Err("Agent mismatch".to_string());
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn typed_runtime_handle_is_required_before_raw_writer_open() {
        let (_temp, layout) = layout();
        let authorization = must(
            ProductionCognitiveWriteAuthorization::verify(
                lease(),
                &AllowVerifier,
                &agent_id(),
                1,
            ),
            "create typed authorization",
        );
        let runtime = must(
            AuthorizedProductionWriter::open(
                &layout,
                authorization,
                &AllowVerifier,
                "production:runtime:test",
                1,
            )
            .await,
            "open typed production writer runtime",
        );
        assert!(runtime.cognitive_write_capability().is_external());
        assert_eq!(runtime.cognitive_write_capability().generation(), 1);
        must(runtime.writer().release().await, "release writer lease");
    }

    #[tokio::test]
    async fn generation_drift_is_rejected_before_store_open() {
        let (_temp, layout) = layout();
        let authorization = must(
            ProductionCognitiveWriteAuthorization::verify(
                lease(),
                &AllowVerifier,
                &agent_id(),
                3,
            ),
            "create typed authorization",
        );
        let result = AuthorizedProductionWriter::open(
            &layout,
            authorization,
            &AllowVerifier,
            "production:runtime:generation-drift",
            4,
        )
        .await;
        assert!(matches!(
            result,
            Err(ProductionWriterRuntimeError::GenerationMismatch {
                capability: 3,
                requested: 4,
            })
        ));
    }
}
