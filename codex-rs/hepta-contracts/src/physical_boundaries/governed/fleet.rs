use serde::Serialize;

use super::GOVERNED_BOUNDARY_SCHEMA_VERSION;
use super::GovernedBoundaryIntent;
use super::core::GovernedBoundaryAdapter;
use super::core::GovernedBoundaryCore;
use super::core::GovernedBoundaryError;
use super::core::GovernedBoundaryOutcome;
use super::core::frame;
use super::core::frame_digest;
use super::core::payload_len;
use super::core::require_nonzero;
use super::core::validate_digest;
use super::core::validate_final_payload;
use super::core::validate_identity;
use crate::Authorized;
use crate::FleetMutationCapability;
use crate::OperationId;
use crate::PhysicalCapabilityKind;
use crate::PhysicalUseVerifier;
use crate::PhysicalUseWindow;
use crate::RevocationRevision;
use crate::Sha256Digest;
use crate::VerifiedUseWitness;

/// Exact Fleet registry mutation identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FleetMutationIntent {
    schema_version: u32,
    operation_id: OperationId,
    registry_revision: u64,
    release_id: String,
    owner_epoch: u64,
    process_generation: u64,
    immutable_release_identity_sha256: Sha256Digest,
    expected_prior_registry_sha256: Sha256Digest,
    final_payload_sha256: Sha256Digest,
    final_payload_bytes: u64,
}

impl FleetMutationIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationId,
        registry_revision: u64,
        release_id: impl Into<String>,
        owner_epoch: u64,
        process_generation: u64,
        immutable_release_identity_sha256: Sha256Digest,
        expected_prior_registry_sha256: Sha256Digest,
        final_payload: &[u8],
    ) -> Result<Self, GovernedBoundaryError> {
        let intent = Self {
            schema_version: GOVERNED_BOUNDARY_SCHEMA_VERSION,
            operation_id,
            registry_revision,
            release_id: release_id.into(),
            owner_epoch,
            process_generation,
            immutable_release_identity_sha256,
            expected_prior_registry_sha256,
            final_payload_sha256: Sha256Digest::for_bytes(final_payload),
            final_payload_bytes: payload_len(final_payload)?,
        };
        intent.validate()?;
        Ok(intent)
    }

    pub const fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    pub const fn final_payload_sha256(&self) -> &Sha256Digest {
        &self.final_payload_sha256
    }

    pub const fn final_payload_bytes(&self) -> u64 {
        self.final_payload_bytes
    }

    pub fn validate_final_payload(
        &self,
        final_payload: &[u8],
    ) -> Result<(), GovernedBoundaryError> {
        self.validate()?;
        validate_final_payload(
            &self.final_payload_sha256,
            self.final_payload_bytes,
            final_payload,
        )
    }

    pub fn physical_payload_sha256(&self) -> Result<Sha256Digest, GovernedBoundaryError> {
        self.validate()?;
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:fleet-mutation-physical-payload:v1");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.operation_id.as_str().as_bytes());
        frame(&mut bytes, &self.registry_revision.to_be_bytes());
        frame(&mut bytes, self.release_id.as_bytes());
        frame(&mut bytes, &self.owner_epoch.to_be_bytes());
        frame(&mut bytes, &self.process_generation.to_be_bytes());
        frame_digest(&mut bytes, &self.immutable_release_identity_sha256);
        frame_digest(&mut bytes, &self.expected_prior_registry_sha256);
        frame_digest(&mut bytes, &self.final_payload_sha256);
        frame(&mut bytes, &self.final_payload_bytes.to_be_bytes());
        Ok(Sha256Digest::for_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), GovernedBoundaryError> {
        if self.schema_version != GOVERNED_BOUNDARY_SCHEMA_VERSION {
            return Err(GovernedBoundaryError::SchemaVersion);
        }
        require_nonzero("Fleet registry revision", self.registry_revision)?;
        validate_identity("Fleet release id", &self.release_id)?;
        require_nonzero("Fleet owner epoch", self.owner_epoch)?;
        require_nonzero("Fleet process generation", self.process_generation)?;
        validate_digest(
            "immutable Fleet release identity",
            &self.immutable_release_identity_sha256,
        )?;
        validate_digest(
            "expected prior Fleet registry",
            &self.expected_prior_registry_sha256,
        )?;
        validate_digest("Fleet final payload", &self.final_payload_sha256)?;
        super::core::validate_payload_size(self.final_payload_bytes)
    }
}

pub struct CheckedFleetMutation<A, V>
where
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    core: GovernedBoundaryCore<FleetMutationCapability, A, V>,
}

impl<A, V> CheckedFleetMutation<A, V>
where
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    pub fn new(
        adapter: A,
        capability: Authorized<FleetMutationCapability>,
        verifier: V,
    ) -> Result<Self, GovernedBoundaryError> {
        Ok(Self {
            core: GovernedBoundaryCore::new(adapter, capability, verifier)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn execute_once<N, S, P>(
        &mut self,
        intent: &FleetMutationIntent,
        final_payload: &[u8],
        expected_revocation_revision: RevocationRevision,
        window: PhysicalUseWindow,
        now_unix_seconds: &N,
        claim_once: &S,
        persist_witness: P,
    ) -> Result<(GovernedBoundaryOutcome, VerifiedUseWitness), GovernedBoundaryError>
    where
        N: Fn() -> Result<u64, String> + Sync + ?Sized,
        S: Fn(
                PhysicalCapabilityKind,
                &Sha256Digest,
                &Sha256Digest,
                &Sha256Digest,
                &Sha256Digest,
                u64,
            ) -> Result<(u64, Sha256Digest), String>
            + Sync
            + ?Sized,
        P: FnOnce(&VerifiedUseWitness) -> Result<(), String>,
    {
        let boundary = GovernedBoundaryIntent::FleetMutation(intent.clone());
        self.core
            .cross_once(
                &boundary,
                final_payload,
                expected_revocation_revision,
                window,
                now_unix_seconds,
                claim_once,
                persist_witness,
            )
            .await
    }
}
