use serde::Serialize;

use super::CandidateIdentity;
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
use crate::Authorized;
use crate::OperationId;
use crate::OperatorAcceptanceCapability;
use crate::PhysicalCapabilityKind;
use crate::PhysicalUseVerifier;
use crate::PhysicalUseWindow;
use crate::RevocationRevision;
use crate::Sha256Digest;
use crate::VerifiedUseWitness;

/// Intent to record an independently issued, exact-candidate operator decision.
///
/// This value is not the decision itself. The externally governed adapter must
/// verify the challenge, identity, validity and evidence before returning a
/// digest-bound external receipt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorAcceptanceIntent {
    schema_version: u32,
    operation_id: OperationId,
    candidate: CandidateIdentity,
    complete_evidence_manifest_sha256: Sha256Digest,
    acceptance_policy_revision: u64,
    implementer_identity_sha256: Sha256Digest,
    independent_reviewer_identity_sha256: Sha256Digest,
    review_challenge_sha256: Sha256Digest,
    issued_at_unix_seconds: u64,
    expires_at_unix_seconds: u64,
    final_payload_sha256: Sha256Digest,
    final_payload_bytes: u64,
}

impl OperatorAcceptanceIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationId,
        candidate: CandidateIdentity,
        complete_evidence_manifest_sha256: Sha256Digest,
        acceptance_policy_revision: u64,
        implementer_identity_sha256: Sha256Digest,
        independent_reviewer_identity_sha256: Sha256Digest,
        review_challenge_sha256: Sha256Digest,
        issued_at_unix_seconds: u64,
        expires_at_unix_seconds: u64,
        final_payload: &[u8],
    ) -> Result<Self, GovernedBoundaryError> {
        let intent = Self {
            schema_version: GOVERNED_BOUNDARY_SCHEMA_VERSION,
            operation_id,
            candidate,
            complete_evidence_manifest_sha256,
            acceptance_policy_revision,
            implementer_identity_sha256,
            independent_reviewer_identity_sha256,
            review_challenge_sha256,
            issued_at_unix_seconds,
            expires_at_unix_seconds,
            final_payload_sha256: Sha256Digest::for_bytes(final_payload),
            final_payload_bytes: payload_len(final_payload)?,
        };
        intent.validate()?;
        Ok(intent)
    }

    pub const fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    pub const fn candidate(&self) -> &CandidateIdentity {
        &self.candidate
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
        let candidate_sha256 = self.candidate.digest()?;
        let mut bytes = Vec::new();
        frame(
            &mut bytes,
            b"hepta:operator-acceptance-physical-payload:v1",
        );
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.operation_id.as_str().as_bytes());
        frame_digest(&mut bytes, &candidate_sha256);
        frame_digest(&mut bytes, &self.complete_evidence_manifest_sha256);
        frame(
            &mut bytes,
            &self.acceptance_policy_revision.to_be_bytes(),
        );
        frame_digest(&mut bytes, &self.implementer_identity_sha256);
        frame_digest(
            &mut bytes,
            &self.independent_reviewer_identity_sha256,
        );
        frame_digest(&mut bytes, &self.review_challenge_sha256);
        frame(&mut bytes, &self.issued_at_unix_seconds.to_be_bytes());
        frame(&mut bytes, &self.expires_at_unix_seconds.to_be_bytes());
        frame_digest(&mut bytes, &self.final_payload_sha256);
        frame(&mut bytes, &self.final_payload_bytes.to_be_bytes());
        Ok(Sha256Digest::for_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), GovernedBoundaryError> {
        if self.schema_version != GOVERNED_BOUNDARY_SCHEMA_VERSION {
            return Err(GovernedBoundaryError::SchemaVersion);
        }
        self.candidate.digest()?;
        validate_digest(
            "complete operator evidence manifest",
            &self.complete_evidence_manifest_sha256,
        )?;
        require_nonzero(
            "operator acceptance policy revision",
            self.acceptance_policy_revision,
        )?;
        validate_digest(
            "operator implementer identity",
            &self.implementer_identity_sha256,
        )?;
        validate_digest(
            "independent operator reviewer identity",
            &self.independent_reviewer_identity_sha256,
        )?;
        if self.implementer_identity_sha256 == self.independent_reviewer_identity_sha256 {
            return Err(GovernedBoundaryError::InvalidIdentity(
                "independent operator reviewer identity",
            ));
        }
        validate_digest("operator review challenge", &self.review_challenge_sha256)?;
        if self.issued_at_unix_seconds == 0
            || self.expires_at_unix_seconds <= self.issued_at_unix_seconds
        {
            return Err(GovernedBoundaryError::InvalidDeadline);
        }
        validate_digest("operator final payload", &self.final_payload_sha256)?;
        super::core::validate_payload_size(self.final_payload_bytes)
    }
}

pub struct CheckedOperatorAcceptance<A, V>
where
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    core: GovernedBoundaryCore<OperatorAcceptanceCapability, A, V>,
}

impl<A, V> CheckedOperatorAcceptance<A, V>
where
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    pub fn new(
        adapter: A,
        capability: Authorized<OperatorAcceptanceCapability>,
        verifier: V,
    ) -> Result<Self, GovernedBoundaryError> {
        Ok(Self {
            core: GovernedBoundaryCore::new(adapter, capability, verifier)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn execute_once<N, S, P>(
        &mut self,
        intent: &OperatorAcceptanceIntent,
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
        let boundary = GovernedBoundaryIntent::OperatorAcceptance(intent.clone());
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
