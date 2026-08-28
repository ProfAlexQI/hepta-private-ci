use std::fmt;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityError;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_contracts::Sha256Digest;

const MAX_CAPABILITY_ID_BYTES: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyProductionLeaseEvidence {
    capability_id: String,
    owner_agent_id: AgentId,
    authority_epoch: u64,
    owner_epoch: u64,
    generation: u64,
    expires_at_unix_seconds: u64,
    lease_head_sha256: Sha256Digest,
    verifier_receipt_sha256: Sha256Digest,
}

impl LegacyProductionLeaseEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        capability_id: impl Into<String>,
        owner_agent_id: AgentId,
        authority_epoch: u64,
        owner_epoch: u64,
        generation: u64,
        expires_at_unix_seconds: u64,
        lease_head_sha256: Sha256Digest,
        verifier_receipt_sha256: Sha256Digest,
    ) -> Result<Self, LegacyAuthorityBridgeError> {
        let capability_id = capability_id.into();
        if capability_id.trim().is_empty()
            || capability_id.len() > MAX_CAPABILITY_ID_BYTES
            || capability_id.as_bytes().contains(&0)
        {
            return Err(LegacyAuthorityBridgeError::InvalidCapabilityId);
        }
        if authority_epoch == 0 || owner_epoch == 0 || generation == 0 {
            return Err(LegacyAuthorityBridgeError::ZeroFence);
        }
        Ok(Self {
            capability_id,
            owner_agent_id,
            authority_epoch,
            owner_epoch,
            generation,
            expires_at_unix_seconds,
            lease_head_sha256,
            verifier_receipt_sha256,
        })
    }

    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    pub fn owner_agent_id(&self) -> &AgentId {
        &self.owner_agent_id
    }

    pub fn authority_epoch(&self) -> u64 {
        self.authority_epoch
    }

    pub fn owner_epoch(&self) -> u64 {
        self.owner_epoch
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn expires_at_unix_seconds(&self) -> u64 {
        self.expires_at_unix_seconds
    }

    pub fn lease_head_sha256(&self) -> &Sha256Digest {
        &self.lease_head_sha256
    }

    pub fn verifier_receipt_sha256(&self) -> &Sha256Digest {
        &self.verifier_receipt_sha256
    }

    pub fn digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:legacy-production-lease-evidence:v1");
        frame(&mut bytes, self.capability_id.as_bytes());
        frame(&mut bytes, self.owner_agent_id.as_str().as_bytes());
        frame(&mut bytes, &self.authority_epoch.to_be_bytes());
        frame(&mut bytes, &self.owner_epoch.to_be_bytes());
        frame(&mut bytes, &self.generation.to_be_bytes());
        frame(&mut bytes, &self.expires_at_unix_seconds.to_be_bytes());
        frame(&mut bytes, self.lease_head_sha256.as_str().as_bytes());
        frame(
            &mut bytes,
            self.verifier_receipt_sha256.as_str().as_bytes(),
        );
        Sha256Digest::for_bytes(&bytes)
    }
}

pub trait LegacyProductionAuthorityVerifier: Send + Sync {
    fn verifier_id(&self) -> &str;

    fn verify(
        &self,
        evidence: &LegacyProductionLeaseEvidence,
        observed_at_unix_seconds: u64,
    ) -> Result<(), LegacyAuthorityBridgeError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedProductionCognitiveWrite {
    authorized: Authorized<CognitiveWriteCapability>,
    evidence_sha256: Sha256Digest,
    verifier_id_sha256: Sha256Digest,
}

impl VerifiedProductionCognitiveWrite {
    pub fn authorized(&self) -> &Authorized<CognitiveWriteCapability> {
        &self.authorized
    }

    pub fn evidence_sha256(&self) -> &Sha256Digest {
        &self.evidence_sha256
    }

    pub fn verifier_id_sha256(&self) -> &Sha256Digest {
        &self.verifier_id_sha256
    }
}

pub fn adopt_verified_legacy_cognitive_write<V>(
    authority: &AuthorityGrant,
    evidence: &LegacyProductionLeaseEvidence,
    verifier: &V,
    observed_at_unix_seconds: u64,
) -> Result<VerifiedProductionCognitiveWrite, LegacyAuthorityBridgeError>
where
    V: LegacyProductionAuthorityVerifier,
{
    authority
        .validate_binding(evidence.owner_agent_id(), evidence.generation())
        .map_err(LegacyAuthorityBridgeError::Authority)?;
    if observed_at_unix_seconds >= evidence.expires_at_unix_seconds() {
        return Err(LegacyAuthorityBridgeError::Expired);
    }
    let verifier_id = verifier.verifier_id();
    if verifier_id.trim().is_empty()
        || verifier_id.len() > MAX_CAPABILITY_ID_BYTES
        || verifier_id.as_bytes().contains(&0)
    {
        return Err(LegacyAuthorityBridgeError::InvalidVerifierId);
    }
    verifier.verify(evidence, observed_at_unix_seconds)?;
    let authorized = authority
        .authorize::<CognitiveWriteCapability>()
        .map_err(LegacyAuthorityBridgeError::Authority)?;
    Ok(VerifiedProductionCognitiveWrite {
        authorized,
        evidence_sha256: evidence.digest(),
        verifier_id_sha256: Sha256Digest::for_bytes(verifier_id.as_bytes()),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LegacyAuthorityBridgeError {
    Authority(AuthorityError),
    InvalidCapabilityId,
    InvalidVerifierId,
    ZeroFence,
    Expired,
    VerificationRejected,
}

impl fmt::Display for LegacyAuthorityBridgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authority(error) => write!(formatter, "typed authority rejected: {error}"),
            Self::InvalidCapabilityId => formatter.write_str(
                "legacy capability id must contain 1..=256 non-NUL bytes",
            ),
            Self::InvalidVerifierId => formatter
                .write_str("legacy verifier id must contain 1..=256 non-NUL bytes"),
            Self::ZeroFence => formatter.write_str(
                "legacy authority epoch, owner epoch, and generation must be non-zero",
            ),
            Self::Expired => formatter.write_str("legacy production lease evidence expired"),
            Self::VerificationRejected => {
                formatter.write_str("legacy production authority verifier rejected evidence")
            }
        }
    }
}

impl std::error::Error for LegacyAuthorityBridgeError {}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityAction;
    use codex_hepta_contracts::AuthorityGrant;
    use codex_hepta_contracts::Sha256Digest;

    use super::LegacyAuthorityBridgeError;
    use super::LegacyProductionAuthorityVerifier;
    use super::LegacyProductionLeaseEvidence;
    use super::adopt_verified_legacy_cognitive_write;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    struct Verifier {
        expected_evidence_sha256: Sha256Digest,
        accept: bool,
    }

    impl LegacyProductionAuthorityVerifier for Verifier {
        fn verifier_id(&self) -> &str {
            "test-legacy-production-verifier"
        }

        fn verify(
            &self,
            evidence: &LegacyProductionLeaseEvidence,
            _observed_at_unix_seconds: u64,
        ) -> Result<(), LegacyAuthorityBridgeError> {
            if self.accept && evidence.digest() == self.expected_evidence_sha256 {
                Ok(())
            } else {
                Err(LegacyAuthorityBridgeError::VerificationRejected)
            }
        }
    }

    fn agent_id() -> AgentId {
        AgentId::parse(AGENT_ID)
            .unwrap_or_else(|error| panic!("test AgentId must parse: {error}"))
    }

    fn evidence(generation: u64, expiry: u64) -> LegacyProductionLeaseEvidence {
        LegacyProductionLeaseEvidence::new(
            "hepta-agentd:production-cognitive-writer:v1",
            agent_id(),
            7,
            11,
            generation,
            expiry,
            Sha256Digest::for_bytes(b"lease-head"),
            Sha256Digest::for_bytes(b"verifier-receipt"),
        )
        .unwrap_or_else(|error| panic!("legacy evidence must be valid: {error}"))
    }

    #[test]
    fn verified_legacy_evidence_becomes_only_cognitive_write() {
        let authority = AuthorityGrant::qualification_cognitive_write(agent_id(), 3)
            .unwrap_or_else(|error| panic!("authority must be valid: {error}"));
        let evidence = evidence(3, 100);
        let verifier = Verifier {
            expected_evidence_sha256: evidence.digest(),
            accept: true,
        };
        let witness = adopt_verified_legacy_cognitive_write(
            &authority,
            &evidence,
            &verifier,
            50,
        )
        .unwrap_or_else(|error| panic!("verified evidence must be adopted: {error}"));
        assert_eq!(
            witness.authorized().action(),
            AuthorityAction::WriteCognitiveState
        );
        assert!(!authority.allows(AuthorityAction::ExternalEffect));
        assert!(!authority.allows(AuthorityAction::InvokeModel));
        assert!(!authority.allows(AuthorityAction::PromoteRelease));
    }

    #[test]
    fn stale_generation_and_expiry_fail_closed() {
        let authority = AuthorityGrant::qualification_cognitive_write(agent_id(), 3)
            .unwrap_or_else(|error| panic!("authority must be valid: {error}"));
        let stale = evidence(2, 100);
        let stale_verifier = Verifier {
            expected_evidence_sha256: stale.digest(),
            accept: true,
        };
        assert!(matches!(
            adopt_verified_legacy_cognitive_write(
                &authority,
                &stale,
                &stale_verifier,
                50
            ),
            Err(LegacyAuthorityBridgeError::Authority(_))
        ));

        let expired = evidence(3, 50);
        let expired_verifier = Verifier {
            expected_evidence_sha256: expired.digest(),
            accept: true,
        };
        assert!(matches!(
            adopt_verified_legacy_cognitive_write(
                &authority,
                &expired,
                &expired_verifier,
                50
            ),
            Err(LegacyAuthorityBridgeError::Expired)
        ));
    }

    #[test]
    fn verifier_rejection_never_yields_typed_authority() {
        let authority = AuthorityGrant::qualification_cognitive_write(agent_id(), 3)
            .unwrap_or_else(|error| panic!("authority must be valid: {error}"));
        let evidence = evidence(3, 100);
        let verifier = Verifier {
            expected_evidence_sha256: evidence.digest(),
            accept: false,
        };
        assert!(matches!(
            adopt_verified_legacy_cognitive_write(
                &authority,
                &evidence,
                &verifier,
                50
            ),
            Err(LegacyAuthorityBridgeError::VerificationRejected)
        ));
    }
}
