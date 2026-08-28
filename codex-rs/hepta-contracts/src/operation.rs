use std::fmt;

use serde::Serialize;

use crate::AgentId;
use crate::AuthorityAction;
use crate::ProductComponentId;
use crate::Sha256Digest;

pub const OPERATION_CONTRACT_SCHEMA_VERSION: u32 = 2;
const MAX_IDENTIFIER_BYTES: usize = 256;
const MAX_COMMAND_BYTES: u64 = 1024 * 1024;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct OperationId(String);

impl OperationId {
    pub fn parse(value: impl Into<String>) -> Result<Self, OperationContractError> {
        parse_identifier(value.into(), "operation id").map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn parse(value: impl Into<String>) -> Result<Self, OperationContractError> {
        parse_identifier(value.into(), "idempotency key").map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationPhase {
    IntentAppended,
    SourceCommitted,
    OutboxPending,
    DeliveryClaimed,
    DestinationCommitted,
    Acknowledged,
    Indeterminate,
    ReconciledApplied,
    ReconciledNotApplied,
    Quarantined,
}

impl OperationPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IntentAppended => "intent_appended",
            Self::SourceCommitted => "source_committed",
            Self::OutboxPending => "outbox_pending",
            Self::DeliveryClaimed => "delivery_claimed",
            Self::DestinationCommitted => "destination_committed",
            Self::Acknowledged => "acknowledged",
            Self::Indeterminate => "indeterminate",
            Self::ReconciledApplied => "reconciled_applied",
            Self::ReconciledNotApplied => "reconciled_not_applied",
            Self::Quarantined => "quarantined",
        }
    }

    pub const fn terminal(self) -> bool {
        matches!(
            self,
            Self::Acknowledged
                | Self::ReconciledApplied
                | Self::ReconciledNotApplied
                | Self::Quarantined
        )
    }

    pub const fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::IntentAppended, Self::SourceCommitted)
                | (Self::SourceCommitted, Self::OutboxPending)
                | (Self::OutboxPending, Self::DeliveryClaimed)
                | (Self::DeliveryClaimed, Self::DestinationCommitted)
                | (Self::DeliveryClaimed, Self::Indeterminate)
                | (Self::DestinationCommitted, Self::Acknowledged)
                | (Self::Indeterminate, Self::ReconciledApplied)
                | (Self::Indeterminate, Self::ReconciledNotApplied)
                | (Self::Indeterminate, Self::Quarantined)
        )
    }
}

/// Digest-bound operation identity between two product component owners.
///
/// The Agent and component are independent identity axes. This permits a
/// same-Agent Automation Runtime → App Server operation while still rejecting
/// an identical source/destination owner.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationBinding {
    pub schema_version: u32,
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub source_owner_agent_id: AgentId,
    pub source_component: ProductComponentId,
    pub destination_owner_agent_id: AgentId,
    pub destination_component: ProductComponentId,
    pub action: AuthorityAction,
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub fencing_token_sha256: Sha256Digest,
    pub command_sha256: Sha256Digest,
    pub command_bytes: u64,
}

impl OperationBinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationId,
        idempotency_key: IdempotencyKey,
        source_owner_agent_id: AgentId,
        source_component: ProductComponentId,
        destination_owner_agent_id: AgentId,
        destination_component: ProductComponentId,
        action: AuthorityAction,
        authority_epoch: u64,
        owner_epoch: u64,
        generation: u64,
        fencing_token_sha256: Sha256Digest,
        command_sha256: Sha256Digest,
        command_bytes: u64,
    ) -> Result<Self, OperationContractError> {
        if source_owner_agent_id == destination_owner_agent_id
            && source_component == destination_component
        {
            return Err(OperationContractError::SameOwner);
        }
        if authority_epoch == 0 || owner_epoch == 0 || generation == 0 {
            return Err(OperationContractError::ZeroFence);
        }
        if command_bytes == 0 || command_bytes > MAX_COMMAND_BYTES {
            return Err(OperationContractError::CommandSize);
        }
        Ok(Self {
            schema_version: OPERATION_CONTRACT_SCHEMA_VERSION,
            operation_id,
            idempotency_key,
            source_owner_agent_id,
            source_component,
            destination_owner_agent_id,
            destination_component,
            action,
            authority_epoch,
            owner_epoch,
            generation,
            fencing_token_sha256,
            command_sha256,
            command_bytes,
        })
    }

    pub fn digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:cross-owner-operation:v2");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.operation_id.as_str().as_bytes());
        frame(&mut bytes, self.idempotency_key.as_str().as_bytes());
        frame(
            &mut bytes,
            self.source_owner_agent_id.as_str().as_bytes(),
        );
        frame(&mut bytes, self.source_component.as_str().as_bytes());
        frame(
            &mut bytes,
            self.destination_owner_agent_id.as_str().as_bytes(),
        );
        frame(&mut bytes, self.destination_component.as_str().as_bytes());
        frame(&mut bytes, self.action.as_str().as_bytes());
        frame(&mut bytes, &self.authority_epoch.to_be_bytes());
        frame(&mut bytes, &self.owner_epoch.to_be_bytes());
        frame(&mut bytes, &self.generation.to_be_bytes());
        frame(
            &mut bytes,
            self.fencing_token_sha256.as_str().as_bytes(),
        );
        frame(&mut bytes, self.command_sha256.as_str().as_bytes());
        frame(&mut bytes, &self.command_bytes.to_be_bytes());
        Sha256Digest::for_bytes(&bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutboxEnvelope {
    pub binding: OperationBinding,
    pub binding_sha256: Sha256Digest,
    pub sequence: u64,
    pub phase: OperationPhase,
}

impl OutboxEnvelope {
    pub fn pending(
        binding: OperationBinding,
        sequence: u64,
    ) -> Result<Self, OperationContractError> {
        if sequence == 0 {
            return Err(OperationContractError::ZeroSequence);
        }
        let binding_sha256 = binding.digest();
        Ok(Self {
            binding,
            binding_sha256,
            sequence,
            phase: OperationPhase::OutboxPending,
        })
    }

    pub fn validate(&self) -> Result<(), OperationContractError> {
        if self.sequence == 0 {
            return Err(OperationContractError::ZeroSequence);
        }
        if self.phase != OperationPhase::OutboxPending
            || self.binding_sha256 != self.binding.digest()
        {
            return Err(OperationContractError::BindingDrift);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DestinationAcknowledgement {
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub binding_sha256: Sha256Digest,
    pub destination_receipt_sha256: Sha256Digest,
    pub sequence: u64,
    pub phase: OperationPhase,
}

impl DestinationAcknowledgement {
    pub fn committed(
        envelope: &OutboxEnvelope,
        destination_receipt_sha256: Sha256Digest,
    ) -> Result<Self, OperationContractError> {
        envelope.validate()?;
        Ok(Self {
            operation_id: envelope.binding.operation_id.clone(),
            idempotency_key: envelope.binding.idempotency_key.clone(),
            binding_sha256: envelope.binding_sha256.clone(),
            destination_receipt_sha256,
            sequence: envelope.sequence,
            phase: OperationPhase::DestinationCommitted,
        })
    }

    pub fn validate_against(
        &self,
        envelope: &OutboxEnvelope,
    ) -> Result<(), OperationContractError> {
        envelope.validate()?;
        if self.operation_id != envelope.binding.operation_id
            || self.idempotency_key != envelope.binding.idempotency_key
            || self.binding_sha256 != envelope.binding_sha256
            || self.sequence != envelope.sequence
            || self.phase != OperationPhase::DestinationCommitted
        {
            return Err(OperationContractError::AcknowledgementDrift);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryDecision {
    RetryBeforeDelivery,
    LookupOnly,
    AdoptAcknowledgement,
    Quarantine,
    Terminal,
}

pub fn recovery_decision(
    phase: OperationPhase,
    delivery_may_have_crossed_boundary: bool,
) -> RecoveryDecision {
    if phase.terminal() {
        return RecoveryDecision::Terminal;
    }
    match phase {
        OperationPhase::IntentAppended
        | OperationPhase::SourceCommitted
        | OperationPhase::OutboxPending
            if !delivery_may_have_crossed_boundary => RecoveryDecision::RetryBeforeDelivery,
        OperationPhase::DestinationCommitted => RecoveryDecision::AdoptAcknowledgement,
        OperationPhase::DeliveryClaimed | OperationPhase::Indeterminate => {
            RecoveryDecision::LookupOnly
        }
        OperationPhase::IntentAppended
        | OperationPhase::SourceCommitted
        | OperationPhase::OutboxPending => RecoveryDecision::LookupOnly,
        OperationPhase::Acknowledged
        | OperationPhase::ReconciledApplied
        | OperationPhase::ReconciledNotApplied
        | OperationPhase::Quarantined => RecoveryDecision::Terminal,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationContractError {
    InvalidIdentifier(&'static str),
    SameOwner,
    ZeroFence,
    CommandSize,
    ZeroSequence,
    BindingDrift,
    AcknowledgementDrift,
    InvalidTransition {
        current: OperationPhase,
        requested: OperationPhase,
    },
}

impl fmt::Display for OperationContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier(label) => write!(
                formatter,
                "{label} must contain 1..={MAX_IDENTIFIER_BYTES} non-NUL bytes"
            ),
            Self::SameOwner => formatter.write_str(
                "cross-owner operation source and destination component owners must differ",
            ),
            Self::ZeroFence => formatter.write_str(
                "authority epoch, owner epoch, and generation must be non-zero",
            ),
            Self::CommandSize => write!(
                formatter,
                "command size must be between 1 and {MAX_COMMAND_BYTES} bytes",
            ),
            Self::ZeroSequence => formatter.write_str("operation sequence must be non-zero"),
            Self::BindingDrift => formatter.write_str("outbox binding digest drifted"),
            Self::AcknowledgementDrift => {
                formatter.write_str("destination acknowledgement does not match the outbox")
            }
            Self::InvalidTransition { current, requested } => write!(
                formatter,
                "operation cannot transition from {} to {}",
                current.as_str(),
                requested.as_str(),
            ),
        }
    }
}

impl std::error::Error for OperationContractError {}

pub fn validate_transition(
    current: OperationPhase,
    requested: OperationPhase,
) -> Result<(), OperationContractError> {
    if current.can_transition_to(requested) {
        Ok(())
    } else {
        Err(OperationContractError::InvalidTransition { current, requested })
    }
}

fn parse_identifier(
    value: String,
    label: &'static str,
) -> Result<String, OperationContractError> {
    if value.trim().is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || value.as_bytes().contains(&0)
    {
        return Err(OperationContractError::InvalidIdentifier(label));
    }
    Ok(value)
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
mod tests {
    use super::*;

    const AGENT: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn agent() -> AgentId {
        match AgentId::parse(AGENT) {
            Ok(agent_id) => agent_id,
            Err(error) => panic!("test AgentId must parse: {error}"),
        }
    }

    fn binding() -> OperationBinding {
        match OperationBinding::new(
            OperationId::parse("operation:test")
                .unwrap_or_else(|error| panic!("operation id must parse: {error}")),
            IdempotencyKey::parse("idempotency:test")
                .unwrap_or_else(|error| panic!("idempotency key must parse: {error}")),
            agent(),
            ProductComponentId::AutomationRuntime,
            agent(),
            ProductComponentId::AppServer,
            AuthorityAction::MutateAutomation,
            1,
            7,
            9,
            Sha256Digest::for_bytes(b"fence"),
            Sha256Digest::for_bytes(b"command"),
            7,
        ) {
            Ok(binding) => binding,
            Err(error) => panic!("operation binding must be valid: {error}"),
        }
    }

    #[test]
    fn same_agent_distinct_components_are_cross_owner() {
        let value = binding();
        assert_eq!(
            value.source_owner_agent_id,
            value.destination_owner_agent_id
        );
        assert_ne!(value.source_component, value.destination_component);
    }

    #[test]
    fn identical_component_owner_is_rejected() {
        let result = OperationBinding::new(
            OperationId::parse("operation:same-owner")
                .unwrap_or_else(|error| panic!("operation id must parse: {error}")),
            IdempotencyKey::parse("idempotency:same-owner")
                .unwrap_or_else(|error| panic!("idempotency key must parse: {error}")),
            agent(),
            ProductComponentId::AutomationRuntime,
            agent(),
            ProductComponentId::AutomationRuntime,
            AuthorityAction::MutateAutomation,
            1,
            1,
            1,
            Sha256Digest::for_bytes(b"fence"),
            Sha256Digest::for_bytes(b"command"),
            7,
        );
        assert!(matches!(result, Err(OperationContractError::SameOwner)));
    }

    #[test]
    fn binding_digest_covers_owner_component_fence_and_payload() {
        let first = binding();
        let mut changed = first.clone();
        changed.generation += 1;
        assert_ne!(first.digest(), changed.digest());
        changed = first.clone();
        changed.destination_component = ProductComponentId::MatrixIngress;
        assert_ne!(first.digest(), changed.digest());
        changed = first.clone();
        changed.command_sha256 = Sha256Digest::for_bytes(b"changed-command");
        assert_ne!(first.digest(), changed.digest());
    }

    #[test]
    fn acknowledgement_requires_exact_outbox_identity() {
        let envelope = OutboxEnvelope::pending(binding(), 1)
            .unwrap_or_else(|error| panic!("outbox must be valid: {error}"));
        let acknowledgement = DestinationAcknowledgement::committed(
            &envelope,
            Sha256Digest::for_bytes(b"receipt"),
        )
        .unwrap_or_else(|error| panic!("acknowledgement must be valid: {error}"));
        assert!(acknowledgement.validate_against(&envelope).is_ok());
        let mut changed = acknowledgement;
        changed.sequence += 1;
        assert!(matches!(
            changed.validate_against(&envelope),
            Err(OperationContractError::AcknowledgementDrift)
        ));
    }

    #[test]
    fn recovery_never_blindly_retries_after_delivery_boundary() {
        assert_eq!(
            recovery_decision(OperationPhase::OutboxPending, false),
            RecoveryDecision::RetryBeforeDelivery
        );
        assert_eq!(
            recovery_decision(OperationPhase::OutboxPending, true),
            RecoveryDecision::LookupOnly
        );
        assert_eq!(
            recovery_decision(OperationPhase::DeliveryClaimed, true),
            RecoveryDecision::LookupOnly
        );
        assert_eq!(
            recovery_decision(OperationPhase::DestinationCommitted, true),
            RecoveryDecision::AdoptAcknowledgement
        );
    }

    #[test]
    fn transition_graph_rejects_skips_and_terminal_reopen() {
        assert!(validate_transition(
            OperationPhase::IntentAppended,
            OperationPhase::SourceCommitted
        )
        .is_ok());
        assert!(matches!(
            validate_transition(
                OperationPhase::IntentAppended,
                OperationPhase::DestinationCommitted
            ),
            Err(OperationContractError::InvalidTransition { .. })
        ));
        assert!(matches!(
            validate_transition(
                OperationPhase::Acknowledged,
                OperationPhase::OutboxPending
            ),
            Err(OperationContractError::InvalidTransition { .. })
        ));
    }
}
