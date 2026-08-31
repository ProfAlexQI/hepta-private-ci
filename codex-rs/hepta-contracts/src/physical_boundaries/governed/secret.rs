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
use crate::ExternalEffectCapability;
use crate::OperationId;
use crate::PhysicalCapabilityKind;
use crate::PhysicalUseVerifier;
use crate::PhysicalUseWindow;
use crate::RevocationRevision;
use crate::Sha256Digest;
use crate::VerifiedUseWitness;

/// Exact, raw-secret-free operation presented to a secret provider boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SecretOperationIntent {
    schema_version: u32,
    operation_id: OperationId,
    opaque_secret_ref_sha256: Sha256Digest,
    provider_id: String,
    profile_id: String,
    token_family_id: String,
    purpose: String,
    audience: String,
    expected_secret_revision: u64,
    operation_deadline_unix_seconds: u64,
    final_payload_sha256: Sha256Digest,
    final_payload_bytes: u64,
}

impl SecretOperationIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationId,
        opaque_secret_ref_sha256: Sha256Digest,
        provider_id: impl Into<String>,
        profile_id: impl Into<String>,
        token_family_id: impl Into<String>,
        purpose: impl Into<String>,
        audience: impl Into<String>,
        expected_secret_revision: u64,
        operation_deadline_unix_seconds: u64,
        final_payload: &[u8],
    ) -> Result<Self, GovernedBoundaryError> {
        let intent = Self {
            schema_version: GOVERNED_BOUNDARY_SCHEMA_VERSION,
            operation_id,
            opaque_secret_ref_sha256,
            provider_id: provider_id.into(),
            profile_id: profile_id.into(),
            token_family_id: token_family_id.into(),
            purpose: purpose.into(),
            audience: audience.into(),
            expected_secret_revision,
            operation_deadline_unix_seconds,
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
        frame(&mut bytes, b"hepta:secret-operation-physical-payload:v1");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.operation_id.as_str().as_bytes());
        frame_digest(&mut bytes, &self.opaque_secret_ref_sha256);
        frame(&mut bytes, self.provider_id.as_bytes());
        frame(&mut bytes, self.profile_id.as_bytes());
        frame(&mut bytes, self.token_family_id.as_bytes());
        frame(&mut bytes, self.purpose.as_bytes());
        frame(&mut bytes, self.audience.as_bytes());
        frame(&mut bytes, &self.expected_secret_revision.to_be_bytes());
        frame(
            &mut bytes,
            &self.operation_deadline_unix_seconds.to_be_bytes(),
        );
        frame_digest(&mut bytes, &self.final_payload_sha256);
        frame(&mut bytes, &self.final_payload_bytes.to_be_bytes());
        Ok(Sha256Digest::for_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), GovernedBoundaryError> {
        if self.schema_version != GOVERNED_BOUNDARY_SCHEMA_VERSION {
            return Err(GovernedBoundaryError::SchemaVersion);
        }
        validate_digest("opaque SecretRef", &self.opaque_secret_ref_sha256)?;
        validate_identity("secret provider id", &self.provider_id)?;
        validate_identity("secret profile id", &self.profile_id)?;
        validate_identity("secret token family", &self.token_family_id)?;
        validate_identity("secret purpose", &self.purpose)?;
        validate_identity("secret audience", &self.audience)?;
        require_nonzero("expected secret revision", self.expected_secret_revision)?;
        if self.operation_deadline_unix_seconds == 0 {
            return Err(GovernedBoundaryError::InvalidDeadline);
        }
        validate_digest("secret final payload", &self.final_payload_sha256)?;
        super::core::validate_payload_size(self.final_payload_bytes)
    }
}

/// Secret operation boundary. Raw secret bytes are absent from the intent,
/// witness, outcome and error surface.
pub struct CheckedSecretOperation<A, V>
where
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    core: GovernedBoundaryCore<ExternalEffectCapability, A, V>,
}

impl<A, V> CheckedSecretOperation<A, V>
where
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    pub fn new(
        adapter: A,
        capability: Authorized<ExternalEffectCapability>,
        verifier: V,
    ) -> Result<Self, GovernedBoundaryError> {
        Ok(Self {
            core: GovernedBoundaryCore::new(adapter, capability, verifier)?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn execute_once<N, S, P>(
        &mut self,
        intent: &SecretOperationIntent,
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
        let boundary = GovernedBoundaryIntent::SecretOperation(intent.clone());
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
