//! Canonical operation wrapper for provider-backed external effects.
//!
//! The existing provider effect journal remains the provider-specific
//! no-blind-retry state machine. This module binds that state machine to the
//! product-wide operation identity, owner epochs, generation fence, and a
//! separately verified `Authorized<ExternalEffectCapability>`.
//!
//! Merely constructing a provider operation grants no authority. Every
//! physical dispatch and provider status lookup through
//! [`ProviderOperationCoordinator`] revalidates the external capability.

use std::fmt;

use crate::AgentId;
use crate::AuthorityAction;
use crate::AuthorityError;
use crate::Authorized;
use crate::DestinationAcknowledgement;
use crate::ExternalEffectCapability;
use crate::IdempotencyKey;
use crate::OperationBinding;
use crate::OperationContractError;
use crate::OperationId;
use crate::OperationPhase;
use crate::OutboxEnvelope;
use crate::ProductComponentId;
use crate::ProviderEffectAck;
use crate::ProviderEffectAckStatus;
use crate::ProviderEffectAdapter;
use crate::ProviderEffectCoordinator;
use crate::ProviderEffectCoordinatorError;
use crate::ProviderEffectDispatchReceipt;
use crate::ProviderEffectIntent;
use crate::ProviderEffectKey;
use crate::ProviderEffectState;
use crate::RecoveryDecision;
use crate::Sha256Digest;
use crate::recovery_decision;

pub const PROVIDER_OPERATION_SCHEMA_VERSION: u32 = 1;
const PROVIDER_OPERATION_SEQUENCE: u64 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderOperationRecord {
    pub schema_version: u32,
    pub envelope: OutboxEnvelope,
    pub phase: OperationPhase,
    pub destination_receipt_sha256: Option<Sha256Digest>,
}

impl ProviderOperationRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        owner_agent_id: AgentId,
        intent: &ProviderEffectIntent,
        authority_epoch: u64,
        owner_epoch: u64,
        generation: u64,
        fencing_token_sha256: Sha256Digest,
        command_bytes: u64,
    ) -> Result<Self, ProviderOperationError> {
        intent.validate()?;
        let operation_id = provider_operation_id(&intent.key)?;
        let idempotency_key = IdempotencyKey::parse(intent.key.as_str().to_string())?;
        let binding = OperationBinding::new(
            operation_id,
            idempotency_key,
            owner_agent_id.clone(),
            ProductComponentId::AppServer,
            owner_agent_id,
            ProductComponentId::ProviderEffectAdapter,
            AuthorityAction::ExternalEffect,
            authority_epoch,
            owner_epoch,
            generation,
            fencing_token_sha256,
            intent.payload_sha256.clone(),
            command_bytes,
        )?;
        let envelope = OutboxEnvelope::pending(binding, PROVIDER_OPERATION_SEQUENCE)?;
        Ok(Self {
            schema_version: PROVIDER_OPERATION_SCHEMA_VERSION,
            envelope,
            phase: OperationPhase::OutboxPending,
            destination_receipt_sha256: None,
        })
    }

    pub fn validate_for(
        &self,
        intent: &ProviderEffectIntent,
    ) -> Result<(), ProviderOperationError> {
        intent.validate()?;
        self.envelope.validate()?;
        if self.schema_version != PROVIDER_OPERATION_SCHEMA_VERSION
            || self.envelope.binding.source_component != ProductComponentId::AppServer
            || self.envelope.binding.destination_component
                != ProductComponentId::ProviderEffectAdapter
            || self.envelope.binding.action != AuthorityAction::ExternalEffect
            || self.envelope.binding.idempotency_key.as_str() != intent.key.as_str()
            || self.envelope.binding.command_sha256 != intent.payload_sha256
            || self.envelope.binding.operation_id != provider_operation_id(&intent.key)?
        {
            return Err(ProviderOperationError::BindingDrift);
        }
        if matches!(
            self.phase,
            OperationPhase::Acknowledged | OperationPhase::ReconciledApplied
        ) != self.destination_receipt_sha256.is_some()
        {
            return Err(ProviderOperationError::BindingDrift);
        }
        Ok(())
    }

    pub fn recovery_decision(&self) -> RecoveryDecision {
        recovery_decision(self.phase, self.phase != OperationPhase::OutboxPending)
    }

    fn claim_delivery(&mut self) -> Result<(), ProviderOperationError> {
        match self.phase {
            OperationPhase::OutboxPending => {
                self.transition(OperationPhase::DeliveryClaimed)?;
                Ok(())
            }
            // HEPTA_PROVIDER_SINGLE_WINNER_CLAIM_V1: a claimed or
            // settled provider operation may only use status lookup/reconcile.
            OperationPhase::DeliveryClaimed
            | OperationPhase::Indeterminate
            | OperationPhase::Acknowledged
            | OperationPhase::ReconciledApplied
            | OperationPhase::ReconciledNotApplied
            | OperationPhase::Quarantined => Err(ProviderOperationError::DeliveryAlreadyClaimed),
            _ => Err(ProviderOperationError::BindingDrift),
        }
    }

    fn settle_dispatch(
        &mut self,
        state: ProviderEffectState,
        latest_ack: Option<&ProviderEffectAck>,
    ) -> Result<(), ProviderOperationError> {
        match state {
            ProviderEffectState::Completed => {
                let ack = latest_ack.ok_or(ProviderOperationError::MissingProviderAck)?;
                if ack.status != ProviderEffectAckStatus::Completed {
                    return Err(ProviderOperationError::BindingDrift);
                }
                let receipt = provider_ack_digest(ack);
                match self.phase {
                    OperationPhase::DeliveryClaimed => {
                        self.transition(OperationPhase::DestinationCommitted)?;
                        let acknowledgement =
                            DestinationAcknowledgement::committed(&self.envelope, receipt.clone())?;
                        acknowledgement.validate_against(&self.envelope)?;
                        self.transition(OperationPhase::Acknowledged)?;
                        self.destination_receipt_sha256 = Some(receipt);
                    }
                    OperationPhase::Indeterminate => {
                        self.transition(OperationPhase::ReconciledApplied)?;
                        self.destination_receipt_sha256 = Some(receipt);
                    }
                    OperationPhase::Acknowledged | OperationPhase::ReconciledApplied => {
                        if self.destination_receipt_sha256.as_ref() != Some(&receipt) {
                            return Err(ProviderOperationError::BindingDrift);
                        }
                    }
                    _ => return Err(ProviderOperationError::BindingDrift),
                }
            }
            ProviderEffectState::Rejected => {
                let ack = latest_ack.ok_or(ProviderOperationError::MissingProviderAck)?;
                if ack.status != ProviderEffectAckStatus::Rejected {
                    return Err(ProviderOperationError::BindingDrift);
                }
                match self.phase {
                    OperationPhase::DeliveryClaimed => {
                        self.transition(OperationPhase::Indeterminate)?;
                        self.transition(OperationPhase::ReconciledNotApplied)?;
                    }
                    OperationPhase::Indeterminate => {
                        self.transition(OperationPhase::ReconciledNotApplied)?;
                    }
                    OperationPhase::ReconciledNotApplied => {}
                    _ => return Err(ProviderOperationError::BindingDrift),
                }
            }
            ProviderEffectState::Pending
            | ProviderEffectState::Accepted
            | ProviderEffectState::Indeterminate => match self.phase {
                OperationPhase::DeliveryClaimed => {
                    self.transition(OperationPhase::Indeterminate)?;
                }
                OperationPhase::Indeterminate => {}
                _ if self.phase.terminal() => {}
                _ => return Err(ProviderOperationError::BindingDrift),
            },
        }
        Ok(())
    }

    fn transition(&mut self, requested: OperationPhase) -> Result<(), ProviderOperationError> {
        if !self.phase.can_transition_to(requested) {
            return Err(OperationContractError::InvalidTransition {
                current: self.phase,
                requested,
            }
            .into());
        }
        self.phase = requested;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderOperationDispatchReceipt {
    pub provider: ProviderEffectDispatchReceipt,
    pub operation_phase: OperationPhase,
}

/// Authority-gated integration of the canonical operation state and the
/// provider-specific coordinator.
pub struct ProviderOperationCoordinator<A>
where
    A: ProviderEffectAdapter,
{
    provider: ProviderEffectCoordinator<A>,
    operation: ProviderOperationRecord,
    external_effect: Authorized<ExternalEffectCapability>,
}

impl<A> ProviderOperationCoordinator<A>
where
    A: ProviderEffectAdapter,
{
    pub fn new(
        adapter: A,
        operation: ProviderOperationRecord,
        external_effect: Authorized<ExternalEffectCapability>,
        observed_at_unix_seconds: u64,
    ) -> Result<Self, ProviderOperationError> {
        validate_effect_authority(&operation, &external_effect, observed_at_unix_seconds)?;
        Ok(Self {
            provider: ProviderEffectCoordinator::new(adapter),
            operation,
            external_effect,
        })
    }

    pub fn with_provider(
        provider: ProviderEffectCoordinator<A>,
        operation: ProviderOperationRecord,
        external_effect: Authorized<ExternalEffectCapability>,
        observed_at_unix_seconds: u64,
    ) -> Result<Self, ProviderOperationError> {
        validate_effect_authority(&operation, &external_effect, observed_at_unix_seconds)?;
        Ok(Self {
            provider,
            operation,
            external_effect,
        })
    }

    pub fn operation(&self) -> &ProviderOperationRecord {
        &self.operation
    }

    pub fn provider(&self) -> &ProviderEffectCoordinator<A> {
        &self.provider
    }

    pub fn into_parts(
        self,
    ) -> (
        ProviderEffectCoordinator<A>,
        ProviderOperationRecord,
        Authorized<ExternalEffectCapability>,
    ) {
        (self.provider, self.operation, self.external_effect)
    }

    pub async fn dispatch_once(
        &mut self,
        intent: ProviderEffectIntent,
        observed_at_unix_seconds: u64,
    ) -> Result<ProviderOperationDispatchReceipt, ProviderOperationError> {
        self.operation.validate_for(&intent)?;
        self.validate_authority(observed_at_unix_seconds)?;
        self.operation.claim_delivery()?;
        let provider = match self.provider.dispatch_once(intent.clone()).await {
            Ok(receipt) => receipt,
            Err(error) => {
                self.operation
                    .settle_dispatch(ProviderEffectState::Indeterminate, None)?;
                return Err(error.into());
            }
        };
        let latest_ack = self.provider.journal().acknowledgements(&intent.key).last();
        self.operation.settle_dispatch(provider.state, latest_ack)?;
        Ok(ProviderOperationDispatchReceipt {
            provider,
            operation_phase: self.operation.phase,
        })
    }

    pub async fn dispatch_once_with_payload(
        &mut self,
        intent: ProviderEffectIntent,
        wire_payload: &[u8],
        observed_at_unix_seconds: u64,
    ) -> Result<ProviderOperationDispatchReceipt, ProviderOperationError> {
        self.operation.validate_for(&intent)?;
        self.validate_authority(observed_at_unix_seconds)?;
        self.operation.claim_delivery()?;
        let provider = match self
            .provider
            .dispatch_once_with_payload(intent.clone(), wire_payload)
            .await
        {
            Ok(receipt) => receipt,
            Err(error) => {
                self.operation
                    .settle_dispatch(ProviderEffectState::Indeterminate, None)?;
                return Err(error.into());
            }
        };
        let latest_ack = self.provider.journal().acknowledgements(&intent.key).last();
        self.operation.settle_dispatch(provider.state, latest_ack)?;
        Ok(ProviderOperationDispatchReceipt {
            provider,
            operation_phase: self.operation.phase,
        })
    }

    pub async fn reconcile(
        &mut self,
        intent: &ProviderEffectIntent,
        observed_at_unix_seconds: u64,
    ) -> Result<ProviderEffectState, ProviderOperationError> {
        self.operation.validate_for(intent)?;
        self.validate_authority(observed_at_unix_seconds)?;
        if !matches!(
            self.operation.phase,
            OperationPhase::Indeterminate
                | OperationPhase::Acknowledged
                | OperationPhase::ReconciledApplied
                | OperationPhase::ReconciledNotApplied
        ) {
            return Err(ProviderOperationError::LookupBeforeBoundary);
        }
        let state = self.provider.reconcile(&intent.key).await?;
        let latest_ack = self.provider.journal().acknowledgements(&intent.key).last();
        self.operation.settle_dispatch(state, latest_ack)?;
        Ok(state)
    }

    fn validate_authority(
        &self,
        observed_at_unix_seconds: u64,
    ) -> Result<(), ProviderOperationError> {
        validate_effect_authority(
            &self.operation,
            &self.external_effect,
            observed_at_unix_seconds,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderOperationError {
    ProviderBinding(crate::ProviderEffectBindingError),
    ProviderCoordinator(ProviderEffectCoordinatorError),
    Operation(OperationContractError),
    Authority(AuthorityError),
    BindingDrift,
    MissingProviderAck,
    ExternalAuthorityRequired,
    DeliveryAlreadyClaimed,
    LookupBeforeBoundary,
}

impl fmt::Display for ProviderOperationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProviderBinding(error) => write!(formatter, "provider binding failed: {error:?}"),
            Self::ProviderCoordinator(error) => {
                write!(formatter, "provider coordinator failed: {error:?}")
            }
            Self::Operation(error) => write!(formatter, "operation contract failed: {error}"),
            Self::Authority(error) => write!(formatter, "effect authority failed: {error}"),
            Self::BindingDrift => formatter.write_str("provider operation binding drifted"),
            Self::MissingProviderAck => {
                formatter.write_str("provider terminal state has no exact acknowledgement")
            }
            Self::ExternalAuthorityRequired => formatter
                .write_str("provider operation requires externally verified effect authority"),
            Self::DeliveryAlreadyClaimed => formatter
                .write_str("provider delivery boundary was already claimed; reconcile instead"),
            Self::LookupBeforeBoundary => formatter
                .write_str("provider lookup is forbidden before a delivery boundary is crossed"),
        }
    }
}

impl std::error::Error for ProviderOperationError {}

impl From<crate::ProviderEffectBindingError> for ProviderOperationError {
    fn from(error: crate::ProviderEffectBindingError) -> Self {
        Self::ProviderBinding(error)
    }
}

impl From<ProviderEffectCoordinatorError> for ProviderOperationError {
    fn from(error: ProviderEffectCoordinatorError) -> Self {
        Self::ProviderCoordinator(error)
    }
}

impl From<OperationContractError> for ProviderOperationError {
    fn from(error: OperationContractError) -> Self {
        Self::Operation(error)
    }
}

impl From<AuthorityError> for ProviderOperationError {
    fn from(error: AuthorityError) -> Self {
        Self::Authority(error)
    }
}

fn validate_effect_authority(
    operation: &ProviderOperationRecord,
    external_effect: &Authorized<ExternalEffectCapability>,
    observed_at_unix_seconds: u64,
) -> Result<(), ProviderOperationError> {
    if !external_effect.is_external()
        || external_effect.action() != AuthorityAction::ExternalEffect
        || external_effect.subject_agent_id() != &operation.envelope.binding.source_owner_agent_id
        || external_effect.generation() != operation.envelope.binding.generation
    {
        return Err(ProviderOperationError::ExternalAuthorityRequired);
    }
    let binding = external_effect
        .external_lease_binding()
        .ok_or(ProviderOperationError::ExternalAuthorityRequired)?;
    if binding.authority_epoch() != operation.envelope.binding.authority_epoch
        || binding.owner_epoch() != operation.envelope.binding.owner_epoch
        || binding.fencing_token_sha256() != &operation.envelope.binding.fencing_token_sha256
    {
        return Err(ProviderOperationError::ExternalAuthorityRequired);
    }
    if binding.is_expired_at(observed_at_unix_seconds) {
        return Err(AuthorityError::LeaseExpired {
            deadline: binding.expires_at_unix_seconds(),
        }
        .into());
    }
    Ok(())
}

fn provider_operation_id(key: &ProviderEffectKey) -> Result<OperationId, OperationContractError> {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:provider-operation:v1");
    frame(&mut bytes, key.as_str().as_bytes());
    OperationId::parse(format!(
        "provider:effect:v1:{}",
        Sha256Digest::for_bytes(&bytes).as_str()
    ))
}

fn provider_ack_digest(ack: &ProviderEffectAck) -> Sha256Digest {
    let status = match ack.status {
        ProviderEffectAckStatus::Accepted => b"accepted".as_slice(),
        ProviderEffectAckStatus::Completed => b"completed".as_slice(),
        ProviderEffectAckStatus::Rejected => b"rejected".as_slice(),
    };
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:provider-ack:v1");
    frame(&mut bytes, ack.key.as_str().as_bytes());
    frame(&mut bytes, ack.payload_sha256.as_str().as_bytes());
    frame(
        &mut bytes,
        ack.provider_operation_id_sha256.as_str().as_bytes(),
    );
    frame(&mut bytes, status);
    Sha256Digest::for_bytes(&bytes)
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AuthorityLeaseBinding;
    use crate::CapabilityVerificationRequest;
    use crate::CapabilityVerifier;
    use crate::ProviderEffectDispatch;
    use crate::ProviderEffectFuture;
    use crate::ProviderEffectIdempotencyCapability;
    use crate::ProviderEffectLookup;
    use crate::authorize_verified_capability;

    const AGENT: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn agent() -> AgentId {
        AgentId::parse(AGENT).unwrap_or_else(|error| panic!("AgentId must parse: {error}"))
    }

    fn intent(payload: &[u8]) -> ProviderEffectIntent {
        let key = ProviderEffectKey::parse(format!(
            "provider-effect:v1:{}",
            Sha256Digest::for_bytes(b"provider-occurrence").as_str()
        ))
        .unwrap_or_else(|error| panic!("provider key must parse: {error:?}"));
        ProviderEffectIntent::new(key, Sha256Digest::for_bytes(payload))
    }

    fn operation(intent: &ProviderEffectIntent) -> ProviderOperationRecord {
        ProviderOperationRecord::new(
            agent(),
            intent,
            7,
            11,
            3,
            Sha256Digest::for_bytes(b"effect-fence"),
            64,
        )
        .unwrap_or_else(|error| panic!("provider operation must build: {error}"))
    }

    struct ExactEffectVerifier;

    impl CapabilityVerifier for ExactEffectVerifier {
        fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
            if request.action() != AuthorityAction::ExternalEffect {
                return Err("wrong effect action".to_string());
            }
            Ok(())
        }
    }

    fn effect_authority(generation: u64, expiry: u64) -> Authorized<ExternalEffectCapability> {
        let binding = AuthorityLeaseBinding::new(
            agent(),
            Sha256Digest::for_bytes(b"signed-effect-grant"),
            7,
            11,
            generation,
            Sha256Digest::for_bytes(b"effect-fence"),
            expiry,
        )
        .unwrap_or_else(|error| panic!("effect binding must build: {error}"));
        authorize_verified_capability::<ExternalEffectCapability, _>(
            binding,
            &agent(),
            generation,
            100,
            &ExactEffectVerifier,
        )
        .unwrap_or_else(|error| panic!("effect authority must verify: {error}"))
    }

    #[derive(Clone)]
    struct CompleteAdapter {
        intent: ProviderEffectIntent,
    }

    impl ProviderEffectAdapter for CompleteAdapter {
        fn capability(&self) -> ProviderEffectIdempotencyCapability {
            ProviderEffectIdempotencyCapability::KeyAndStatusLookup
        }

        fn dispatch<'a>(
            &'a self,
            intent: &'a ProviderEffectIntent,
        ) -> ProviderEffectFuture<'a, ProviderEffectDispatch> {
            let ack = ProviderEffectAck::new(
                intent.key.clone(),
                intent.payload_sha256.clone(),
                Sha256Digest::for_bytes(b"provider-operation"),
                ProviderEffectAckStatus::Completed,
            );
            Box::pin(std::future::ready(ProviderEffectDispatch::Ack(ack)))
        }

        fn lookup<'a>(
            &'a self,
            _key: &'a ProviderEffectKey,
        ) -> ProviderEffectFuture<'a, ProviderEffectLookup> {
            let ack = ProviderEffectAck::new(
                self.intent.key.clone(),
                self.intent.payload_sha256.clone(),
                Sha256Digest::for_bytes(b"provider-operation"),
                ProviderEffectAckStatus::Completed,
            );
            Box::pin(std::future::ready(ProviderEffectLookup::Ack(ack)))
        }
    }

    #[test]
    fn operation_binds_app_server_to_dormant_provider_adapter() {
        let intent = intent(b"payload");
        let operation = operation(&intent);
        assert!(operation.validate_for(&intent).is_ok());
        assert_eq!(
            operation.envelope.binding.source_component,
            ProductComponentId::AppServer
        );
        assert_eq!(
            operation.envelope.binding.destination_component,
            ProductComponentId::ProviderEffectAdapter
        );
        assert_eq!(
            operation.envelope.binding.action,
            AuthorityAction::ExternalEffect
        );
        assert_eq!(
            operation.recovery_decision(),
            RecoveryDecision::RetryBeforeDelivery
        );
    }

    #[test]
    fn changed_payload_and_local_effect_authority_fail_closed() {
        let first = intent(b"payload-a");
        let changed =
            ProviderEffectIntent::new(first.key.clone(), Sha256Digest::for_bytes(b"payload-b"));
        let operation = operation(&first);
        assert_eq!(
            operation.validate_for(&changed),
            Err(ProviderOperationError::BindingDrift)
        );
        let local = crate::AuthorityGrant::qualification_cognitive_write(agent(), 3)
            .unwrap_or_else(|error| panic!("qualification grant must build: {error}"));
        assert!(matches!(
            local.authorize::<ExternalEffectCapability>(),
            Err(AuthorityError::ActionDenied(
                AuthorityAction::ExternalEffect
            ))
        ));
    }

    #[tokio::test]
    async fn bound_coordinator_requires_effect_authority_and_acknowledges_exact_completion() {
        let intent = intent(b"payload");
        let operation = operation(&intent);
        assert!(matches!(
            ProviderOperationCoordinator::new(
                CompleteAdapter {
                    intent: intent.clone(),
                },
                operation.clone(),
                effect_authority(4, 500),
                100,
            ),
            Err(ProviderOperationError::ExternalAuthorityRequired)
        ));

        let mut coordinator = ProviderOperationCoordinator::new(
            CompleteAdapter {
                intent: intent.clone(),
            },
            operation,
            effect_authority(3, 500),
            100,
        )
        .unwrap_or_else(|error| panic!("bound coordinator must build: {error}"));
        let receipt = coordinator
            .dispatch_once(intent.clone(), 101)
            .await
            .unwrap_or_else(|error| panic!("dispatch must settle: {error}"));
        assert!(receipt.provider.physical_dispatch_attempted);
        assert_eq!(receipt.provider.state, ProviderEffectState::Completed);
        assert_eq!(receipt.operation_phase, OperationPhase::Acknowledged);
        assert_eq!(
            coordinator.operation().recovery_decision(),
            RecoveryDecision::Terminal
        );
        assert_eq!(
            coordinator.dispatch_once(intent, 102).await,
            Err(ProviderOperationError::DeliveryAlreadyClaimed),
        );
    }

    #[test]
    fn expired_effect_authority_is_rejected_before_dispatch() {
        let intent = intent(b"payload");
        let result = ProviderOperationCoordinator::new(
            CompleteAdapter {
                intent: intent.clone(),
            },
            operation(&intent),
            effect_authority(3, 200),
            200,
        );
        assert!(matches!(
            result,
            Err(ProviderOperationError::Authority(
                AuthorityError::LeaseExpired { deadline: 200 }
            ))
        ));
    }
}
