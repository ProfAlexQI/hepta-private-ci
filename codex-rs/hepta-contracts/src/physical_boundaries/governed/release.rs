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
use super::core::validate_identity;
use crate::Authorized;
use crate::OperationId;
use crate::PhysicalCapabilityKind;
use crate::PhysicalUseVerifier;
use crate::PhysicalUseWindow;
use crate::ReleasePromotionCapability;
use crate::RevocationRevision;
use crate::Sha256Digest;
use crate::VerifiedUseWitness;

/// Exact promotion/release decision intent. All required external evidence is
/// digest-bound; this source cannot manufacture any of those receipts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleasePromotionIntent {
    schema_version: u32,
    operation_id: OperationId,
    candidate: CandidateIdentity,
    release_id: String,
    promotion_target: String,
    release_manifest_sha256: Sha256Digest,
    artifact_set_sha256: Sha256Digest,
    sbom_sha256: Sha256Digest,
    migration_compatibility_sha256: Sha256Digest,
    rollback_evidence_sha256: Sha256Digest,
    independent_review_receipt_sha256: Sha256Digest,
    operator_acceptance_receipt_sha256: Sha256Digest,
    release_policy_revision: u64,
    final_payload_sha256: Sha256Digest,
    final_payload_bytes: u64,
}

impl ReleasePromotionIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationId,
        candidate: CandidateIdentity,
        release_id: impl Into<String>,
        promotion_target: impl Into<String>,
        release_manifest_sha256: Sha256Digest,
        artifact_set_sha256: Sha256Digest,
        sbom_sha256: Sha256Digest,
        migration_compatibility_sha256: Sha256Digest,
        rollback_evidence_sha256: Sha256Digest,
        independent_review_receipt_sha256: Sha256Digest,
        operator_acceptance_receipt_sha256: Sha256Digest,
        release_policy_revision: u64,
        final_payload: &[u8],
    ) -> Result<Self, GovernedBoundaryError> {
        let intent = Self {
            schema_version: GOVERNED_BOUNDARY_SCHEMA_VERSION,
            operation_id,
            candidate,
            release_id: release_id.into(),
            promotion_target: promotion_target.into(),
            release_manifest_sha256,
            artifact_set_sha256,
            sbom_sha256,
            migration_compatibility_sha256,
            rollback_evidence_sha256,
            independent_review_receipt_sha256,
            operator_acceptance_receipt_sha256,
            release_policy_revision,
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
            b"hepta:release-promotion-physical-payload:v1",
        );
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.operation_id.as_str().as_bytes());
        frame_digest(&mut bytes, &candidate_sha256);
        frame(&mut bytes, self.release_id.as_bytes());
        frame(&mut bytes, self.promotion_target.as_bytes());
        frame_digest(&mut bytes, &self.release_manifest_sha256);
        frame_digest(&mut bytes, &self.artifact_set_sha256);
        frame_digest(&mut bytes, &self.sbom_sha256);
        frame_digest(&mut bytes, &self.migration_compatibility_sha256);
        frame_digest(&mut bytes, &self.rollback_evidence_sha256);
        frame_digest(&mut bytes, &self.independent_review_receipt_sha256);
        frame_digest(&mut bytes, &self.operator_acceptance_receipt_sha256);
        frame(&mut bytes, &self.release_policy_revision.to_be_bytes());
        frame_digest(&mut bytes, &self.final_payload_sha256);
        frame(&mut bytes, &self.final_payload_bytes.to_be_bytes());
        Ok(Sha256Digest::for_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), GovernedBoundaryError> {
        if self.schema_version != GOVERNED_BOUNDARY_SCHEMA_VERSION {
            return Err(GovernedBoundaryError::SchemaVersion);
        }
        self.candidate.digest()?;
        validate_identity("release id", &self.release_id)?;
        validate_identity("promotion target", &self.promotion_target)?;
        for (field, digest) in [
            ("release manifest", &self.release_manifest_sha256),
            ("release artifact set", &self.artifact_set_sha256),
            ("release SBOM", &self.sbom_sha256),
            (
                "migration compatibility evidence",
                &self.migration_compatibility_sha256,
            ),
            ("rollback evidence", &self.rollback_evidence_sha256),
            (
                "independent review receipt",
                &self.independent_review_receipt_sha256,
            ),
            (
                "operator acceptance receipt",
                &self.operator_acceptance_receipt_sha256,
            ),
            ("release final payload", &self.final_payload_sha256),
        ] {
            validate_digest(field, digest)?;
        }
        require_nonzero("release policy revision", self.release_policy_revision)?;
        super::core::validate_payload_size(self.final_payload_bytes)
    }
}

pub struct CheckedReleasePromotion<A, V>
where
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    core: GovernedBoundaryCore<ReleasePromotionCapability, A, V>,
}

impl<A, V> CheckedReleasePromotion<A, V>
where
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    pub fn new(
        adapter: A,
        capability: Authorized<ReleasePromotionCapability>,
        verifier: V,
    ) -> Result<Self, GovernedBoundaryError> {
        Ok(Self {
            core: GovernedBoundaryCore::new(adapter, capability, verifier)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn execute_once<N, S, P>(
        &mut self,
        intent: &ReleasePromotionIntent,
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
        let boundary = GovernedBoundaryIntent::ReleasePromotion(intent.clone());
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
