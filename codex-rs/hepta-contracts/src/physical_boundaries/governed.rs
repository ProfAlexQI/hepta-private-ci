//! Governed secret, Matrix, fleet, operator-acceptance, and release boundaries.
//!
//! The module is physically split by boundary class. Each intent binds its own
//! final facts, while the private core reuses the common B0 verified-use token,
//! trusted clock, current revocation check, durable single-use claim, and
//! caller-owned witness persistence.
//!
//! Source presence never issues operator acceptance, promotion, or release.
//! Those facts can only be returned by an externally governed adapter after the
//! corresponding checked boundary has consumed independently issued authority.

mod core;
pub mod fleet;
pub mod matrix;
pub mod operator;
pub mod release;
pub mod secret;

use serde::Serialize;

use crate::OperationId;
use crate::PhysicalCapabilityKind;
use crate::Sha256Digest;

pub use core::GovernedBoundaryAdapter;
pub use core::GovernedBoundaryDispatch;
pub use core::GovernedBoundaryError;
pub use core::GovernedBoundaryFuture;
pub use core::GovernedBoundaryOutcome;
pub use fleet::CheckedFleetMutation;
pub use fleet::FleetMutationIntent;
pub use matrix::CheckedMatrixSend;
pub use matrix::MatrixSendIntent;
pub use operator::CheckedOperatorAcceptance;
pub use operator::OperatorAcceptanceIntent;
pub use release::CheckedReleasePromotion;
pub use release::ReleasePromotionIntent;
pub use secret::CheckedSecretOperation;
pub use secret::SecretOperationIntent;

pub const GOVERNED_BOUNDARY_SCHEMA_VERSION: u32 = 1;
pub const B3_GOVERNED_BOUNDARIES_RUNTIME_REGISTERED: bool = false;
pub const B3_GOVERNED_BOUNDARIES_PRODUCTION_CALLER: bool = false;
pub const B3_GOVERNED_BOUNDARIES_PRODUCTION_WRITER: bool = false;
pub const B3_GOVERNED_BOUNDARIES_SECRET_OPERATION: bool = false;
pub const B3_GOVERNED_BOUNDARIES_MATRIX_SEND: bool = false;
pub const B3_GOVERNED_BOUNDARIES_FLEET_MUTATION: bool = false;
pub const B3_GOVERNED_BOUNDARIES_OPERATOR_ACCEPTANCE: bool = false;
pub const B3_GOVERNED_BOUNDARIES_PROMOTION: bool = false;
pub const B3_GOVERNED_BOUNDARIES_RELEASE: bool = false;

/// Exact source candidate identity shared by operator and release intents.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateIdentity {
    commit: String,
    tree: String,
}

impl CandidateIdentity {
    pub fn new(
        commit: impl Into<String>,
        tree: impl Into<String>,
    ) -> Result<Self, GovernedBoundaryError> {
        let identity = Self {
            commit: commit.into(),
            tree: tree.into(),
        };
        identity.validate()?;
        Ok(identity)
    }

    pub fn commit(&self) -> &str {
        &self.commit
    }

    pub fn tree(&self) -> &str {
        &self.tree
    }

    pub fn digest(&self) -> Result<Sha256Digest, GovernedBoundaryError> {
        self.validate()?;
        let mut bytes = Vec::new();
        core::frame(&mut bytes, b"hepta:candidate-identity:v1");
        core::frame(&mut bytes, self.commit.as_bytes());
        core::frame(&mut bytes, self.tree.as_bytes());
        Ok(Sha256Digest::for_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), GovernedBoundaryError> {
        if !is_lower_hex_40(&self.commit) || !is_lower_hex_40(&self.tree) {
            return Err(GovernedBoundaryError::CandidateIdentityInvalid);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "boundary", rename_all = "snake_case")]
pub enum GovernedBoundaryIntent {
    SecretOperation(SecretOperationIntent),
    MatrixSend(MatrixSendIntent),
    FleetMutation(FleetMutationIntent),
    OperatorAcceptance(OperatorAcceptanceIntent),
    ReleasePromotion(ReleasePromotionIntent),
}

impl GovernedBoundaryIntent {
    pub const fn kind(&self) -> PhysicalCapabilityKind {
        match self {
            Self::SecretOperation(_) => PhysicalCapabilityKind::SecretOperation,
            Self::MatrixSend(_) => PhysicalCapabilityKind::MatrixSend,
            Self::FleetMutation(_) => PhysicalCapabilityKind::FleetMutation,
            Self::OperatorAcceptance(_) => PhysicalCapabilityKind::OperatorAcceptance,
            Self::ReleasePromotion(_) => PhysicalCapabilityKind::ReleasePromotion,
        }
    }

    pub const fn operation_id(&self) -> &OperationId {
        match self {
            Self::SecretOperation(intent) => intent.operation_id(),
            Self::MatrixSend(intent) => intent.operation_id(),
            Self::FleetMutation(intent) => intent.operation_id(),
            Self::OperatorAcceptance(intent) => intent.operation_id(),
            Self::ReleasePromotion(intent) => intent.operation_id(),
        }
    }

    pub const fn final_payload_sha256(&self) -> &Sha256Digest {
        match self {
            Self::SecretOperation(intent) => intent.final_payload_sha256(),
            Self::MatrixSend(intent) => intent.final_payload_sha256(),
            Self::FleetMutation(intent) => intent.final_payload_sha256(),
            Self::OperatorAcceptance(intent) => intent.final_payload_sha256(),
            Self::ReleasePromotion(intent) => intent.final_payload_sha256(),
        }
    }

    pub const fn final_payload_bytes(&self) -> u64 {
        match self {
            Self::SecretOperation(intent) => intent.final_payload_bytes(),
            Self::MatrixSend(intent) => intent.final_payload_bytes(),
            Self::FleetMutation(intent) => intent.final_payload_bytes(),
            Self::OperatorAcceptance(intent) => intent.final_payload_bytes(),
            Self::ReleasePromotion(intent) => intent.final_payload_bytes(),
        }
    }

    pub fn validate_final_payload(
        &self,
        final_payload: &[u8],
    ) -> Result<(), GovernedBoundaryError> {
        match self {
            Self::SecretOperation(intent) => intent.validate_final_payload(final_payload),
            Self::MatrixSend(intent) => intent.validate_final_payload(final_payload),
            Self::FleetMutation(intent) => intent.validate_final_payload(final_payload),
            Self::OperatorAcceptance(intent) => intent.validate_final_payload(final_payload),
            Self::ReleasePromotion(intent) => intent.validate_final_payload(final_payload),
        }
    }

    pub fn physical_payload_sha256(&self) -> Result<Sha256Digest, GovernedBoundaryError> {
        match self {
            Self::SecretOperation(intent) => intent.physical_payload_sha256(),
            Self::MatrixSend(intent) => intent.physical_payload_sha256(),
            Self::FleetMutation(intent) => intent.physical_payload_sha256(),
            Self::OperatorAcceptance(intent) => intent.physical_payload_sha256(),
            Self::ReleasePromotion(intent) => intent.physical_payload_sha256(),
        }
    }
}

fn is_lower_hex_40(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
