//! Explicit Agentd/host seam for the production durable writer.
//!
//! The host must supply an externally verified cognitive-write lease and a
//! separately verified external-effect capability. Nothing in Agentd startup
//! installs either capability automatically; the default runtime remains
//! read-only and has no provider/effect authority.

use std::fmt;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CapabilityUseVerifier;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_contracts::ExternalEffectCapability;
use codex_hepta_contracts::RuntimeAuthorityContext;
use codex_hepta_contracts::verify_capability_use;
use codex_hepta_memory::CognitiveStore;
use codex_hepta_memory::LocalOutcomeState;
use codex_hepta_memory::ProductionAuthorityLease;
use codex_hepta_memory::ProductionAuthorityVerifier;
use codex_hepta_memory::ProductionDispatchReceipt;
use codex_hepta_memory::ProductionDurableWriter;
use codex_hepta_memory::ProductionOutboxDispatcher;
use codex_hepta_memory::ProductionOutboxTarget;
use codex_hepta_memory::ProductionQueuedReceipt;
use codex_hepta_memory_runtime::AuthorizedProductionWriter;

use crate::AgentdConfig;
use crate::AgentdError;
use crate::production_authority_adapter::ProductionCognitiveWriteAuthorization;

/// Host-owned production writer handle. Constructing this value does not
/// mutate Agentd's runtime configuration; callers explicitly attach/use it.
#[derive(Clone)]
pub struct AgentdProductionWriterHost {
    writer: Arc<ProductionDurableWriter>,
    cognitive_write: Authorized<CognitiveWriteCapability>,
    external_effect: Option<Authorized<ExternalEffectCapability>>,
    effect_runtime_authority: Option<RuntimeAuthorityContext>,
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
            .field(
                "external_effect_grant_sha256",
                &self
                    .external_effect
                    .as_ref()
                    .map(|capability| capability.grant_sha256()),
            )
            .field(
                "effect_runtime_authority_sha256",
                &self
                    .effect_runtime_authority
                    .as_ref()
                    .map(RuntimeAuthorityContext::digest),
            )
            .field("dispatcher_attached", &self.dispatcher.is_some())
            .finish()
    }
}

impl AgentdProductionWriterHost {
    /// Verify and mint the typed cognitive-write witness before the Memory
    /// runtime opens the raw durable writer. Agentd never calls the raw opener.
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
        let runtime = AuthorizedProductionWriter::open(
            &config.identity().layout,
            authorization,
            verifier,
            lease_id,
            lease_generation,
        )
        .await
        .map_err(|error| {
            AgentdError::Protocol(format!(
                "open authorized production Memory runtime: {error}"
            ))
        })?;
        Ok(Self::from_runtime(runtime))
    }

    /// Build a host handle around an already-open Agent-owned store. The
    /// Memory runtime still owns the raw writer opening and validates that the
    /// store owner matches the externally verified capability.
    pub async fn open_with_store<V>(
        store: CognitiveStore,
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
            store.owner_agent_id(),
            lease_generation,
        )?;
        let runtime = AuthorizedProductionWriter::open_with_store(
            store,
            authorization,
            verifier,
            lease_id,
            lease_generation,
        )
        .await
        .map_err(|error| {
            AgentdError::Protocol(format!(
                "open authorized production Memory runtime: {error}"
            ))
        })?;
        Ok(Self::from_runtime(runtime))
    }

    fn from_runtime(runtime: AuthorizedProductionWriter) -> Self {
        let (writer, cognitive_write) = runtime.into_parts();
        Self {
            writer,
            cognitive_write,
            external_effect: None,
            effect_runtime_authority: None,
            dispatcher: None,
        }
    }

    /// Append a durable local intent through the host-owned writer. This path
    /// requires cognitive-write authority but does not cross an external
    /// effect boundary.
    pub async fn admit(
        &self,
        occurrence_key: impl Into<String>,
        topic: impl Into<String>,
        payload_json: impl Into<String>,
    ) -> Result<ProductionQueuedReceipt, AgentdError> {
        Ok(self
            .writer
            .admit(occurrence_key, topic, payload_json)
            .await?)
    }

    pub async fn status(
        &self,
        occurrence_key: impl Into<String>,
    ) -> Result<LocalOutcomeState, AgentdError> {
        Ok(self.writer.status(occurrence_key).await?)
    }

    pub fn cognitive_write_capability(&self) -> &Authorized<CognitiveWriteCapability> {
        &self.cognitive_write
    }

    /// Attach an effect target only with a separately verified typed
    /// `ExternalEffectCapability`. The capability must be external, bind the
    /// same Agent, generation, authority epoch, owner epoch and fencing token
    /// as the cognitive writer, and remain unexpired.
    pub fn attach_target(
        mut self,
        target: Arc<dyn ProductionOutboxTarget>,
        external_effect: Authorized<ExternalEffectCapability>,
    ) -> Result<Self, AgentdError> {
        validate_external_effect_capability(
            &self.cognitive_write,
            &external_effect,
            now_unix_seconds()?,
        )?;
        let binding = external_effect.external_lease_binding().ok_or_else(|| {
            AgentdError::Protocol(
                "production external-effect capability is not externally bound".to_string(),
            )
        })?;
        let effect_runtime_authority = RuntimeAuthorityContext::from_external_binding(binding)
            .map_err(|error| {
                AgentdError::Protocol(format!(
                    "bind production effect runtime authority: {error}"
                ))
            })?;
        self.external_effect = Some(external_effect);
        self.effect_runtime_authority = Some(effect_runtime_authority);
        self.dispatcher = Some(ProductionOutboxDispatcher::attach(target));
        Ok(self)
    }

    pub fn has_target(&self) -> bool {
        self.dispatcher.is_some()
            && self.external_effect.is_some()
            && self.effect_runtime_authority.is_some()
    }

    /// Cross the physical provider boundary only after a current use verifier
    /// rechecks revocation, epoch and policy state for this exact capability.
    pub async fn dispatch<V>(
        &self,
        receipt: ProductionQueuedReceipt,
        verifier: &V,
    ) -> Result<ProductionDispatchReceipt, AgentdError>
    where
        V: CapabilityUseVerifier + ?Sized,
    {
        let external_effect = self.external_effect.as_ref().ok_or_else(|| {
            AgentdError::Protocol(
                "production external-effect capability is not explicitly attached".to_string(),
            )
        })?;
        let runtime_authority = self.effect_runtime_authority.as_ref().ok_or_else(|| {
            AgentdError::Protocol(
                "production effect runtime authority is not explicitly attached".to_string(),
            )
        })?;
        let now_unix_seconds = now_unix_seconds()?;
        validate_external_effect_capability(
            &self.cognitive_write,
            external_effect,
            now_unix_seconds,
        )?;
        verify_capability_use(
            external_effect,
            runtime_authority,
            now_unix_seconds,
            verifier,
        )
        .map_err(|error| {
            AgentdError::Protocol(format!(
                "production external-effect use rejected: {error}"
            ))
        })?;
        let dispatcher = self.dispatcher.as_ref().ok_or_else(|| {
            AgentdError::Protocol(
                "production outbox dispatcher is not explicitly attached".to_string(),
            )
        })?;
        Ok(dispatcher.dispatch(self.writer.as_ref(), receipt).await?)
    }
}

fn validate_external_effect_capability(
    cognitive_write: &Authorized<CognitiveWriteCapability>,
    external_effect: &Authorized<ExternalEffectCapability>,
    now_unix_seconds: u64,
) -> Result<(), AgentdError> {
    let cognitive_binding = cognitive_write.external_lease_binding().ok_or_else(|| {
        AgentdError::Protocol(
            "production cognitive-write capability must come from an external verified lease"
                .to_string(),
        )
    })?;
    let effect_binding = external_effect.external_lease_binding().ok_or_else(|| {
        AgentdError::Protocol(
            "production external-effect capability must come from an external verified lease"
                .to_string(),
        )
    })?;
    if external_effect.subject_agent_id() != cognitive_write.subject_agent_id() {
        return Err(AgentdError::Protocol(
            "production external-effect capability belongs to another Agent".to_string(),
        ));
    }
    if external_effect.generation() != cognitive_write.generation() {
        return Err(AgentdError::GenerationFenced(
            "production external-effect capability generation does not match cognitive writer"
                .to_string(),
        ));
    }
    if effect_binding.authority_epoch() != cognitive_binding.authority_epoch()
        || effect_binding.owner_epoch() != cognitive_binding.owner_epoch()
        || effect_binding.fencing_token_sha256() != cognitive_binding.fencing_token_sha256()
    {
        return Err(AgentdError::GenerationFenced(
            "production external-effect capability fence family does not match cognitive writer"
                .to_string(),
        ));
    }
    if cognitive_binding.is_expired_at(now_unix_seconds) {
        return Err(AgentdError::Protocol(format!(
            "production cognitive-write capability expired at {}",
            cognitive_binding.expires_at_unix_seconds()
        )));
    }
    if effect_binding.is_expired_at(now_unix_seconds) {
        return Err(AgentdError::Protocol(format!(
            "production external-effect capability expired at {}",
            effect_binding.expires_at_unix_seconds()
        )));
    }
    Ok(())
}

fn now_unix_seconds() -> Result<u64, AgentdError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| AgentdError::Protocol(format!("system clock failed: {error}")))
}

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityAction;
    use codex_hepta_contracts::AuthorityLeaseBinding;
    use codex_hepta_contracts::CapabilityVerificationRequest;
    use codex_hepta_contracts::CapabilityVerifier;
    use codex_hepta_contracts::CognitiveWriteCapability;
    use codex_hepta_contracts::ExternalEffectCapability;
    use codex_hepta_contracts::Sha256Digest;
    use codex_hepta_contracts::authorize_verified_capability;

    use super::validate_external_effect_capability;

    const OWNER_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
    const OTHER_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c13";

    fn agent_id(value: &str) -> AgentId {
        match AgentId::parse(value) {
            Ok(agent_id) => agent_id,
            Err(error) => panic!("test AgentId must parse: {error}"),
        }
    }

    fn binding(
        agent_id: AgentId,
        generation: u64,
        authority_epoch: u64,
        owner_epoch: u64,
        expiry: u64,
    ) -> AuthorityLeaseBinding {
        match AuthorityLeaseBinding::new(
            agent_id,
            Sha256Digest::for_bytes(b"signed-capability-grant"),
            authority_epoch,
            owner_epoch,
            generation,
            Sha256Digest::for_bytes(b"fencing-token"),
            expiry,
        ) {
            Ok(binding) => binding,
            Err(error) => panic!("test binding must be valid: {error}"),
        }
    }

    struct ExactActionVerifier(AuthorityAction);

    impl CapabilityVerifier for ExactActionVerifier {
        fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
            if request.action() != self.0 {
                return Err("unexpected capability action".to_string());
            }
            Ok(())
        }
    }

    #[test]
    fn effect_target_requires_matching_live_external_lease_family() {
        let owner = agent_id(OWNER_ID);
        let cognitive_write = match authorize_verified_capability::<CognitiveWriteCapability, _>(
            binding(owner.clone(), 3, 7, 11, 500),
            &owner,
            3,
            100,
            &ExactActionVerifier(AuthorityAction::WriteCognitiveState),
        ) {
            Ok(capability) => capability,
            Err(error) => panic!("cognitive write must authorize: {error}"),
        };
        let external_effect = match authorize_verified_capability::<ExternalEffectCapability, _>(
            binding(owner.clone(), 3, 7, 11, 500),
            &owner,
            3,
            100,
            &ExactActionVerifier(AuthorityAction::ExternalEffect),
        ) {
            Ok(capability) => capability,
            Err(error) => panic!("external effect must authorize: {error}"),
        };
        assert!(
            validate_external_effect_capability(&cognitive_write, &external_effect, 100).is_ok()
        );
        assert!(
            validate_external_effect_capability(&cognitive_write, &external_effect, 500).is_err()
        );

        let other = agent_id(OTHER_ID);
        let other_effect = match authorize_verified_capability::<ExternalEffectCapability, _>(
            binding(other.clone(), 3, 7, 11, 500),
            &other,
            3,
            100,
            &ExactActionVerifier(AuthorityAction::ExternalEffect),
        ) {
            Ok(capability) => capability,
            Err(error) => panic!("other effect must authorize: {error}"),
        };
        assert!(validate_external_effect_capability(&cognitive_write, &other_effect, 100).is_err());

        let newer_effect = match authorize_verified_capability::<ExternalEffectCapability, _>(
            binding(owner.clone(), 4, 7, 11, 500),
            &owner,
            4,
            100,
            &ExactActionVerifier(AuthorityAction::ExternalEffect),
        ) {
            Ok(capability) => capability,
            Err(error) => panic!("newer effect must authorize: {error}"),
        };
        assert!(validate_external_effect_capability(&cognitive_write, &newer_effect, 100).is_err());

        let changed_epoch_effect = match authorize_verified_capability::<ExternalEffectCapability, _>(
            binding(owner.clone(), 3, 8, 11, 500),
            &owner,
            3,
            100,
            &ExactActionVerifier(AuthorityAction::ExternalEffect),
        ) {
            Ok(capability) => capability,
            Err(error) => panic!("changed-epoch effect must authorize alone: {error}"),
        };
        assert!(
            validate_external_effect_capability(&cognitive_write, &changed_epoch_effect, 100)
                .is_err()
        );
    }
}
