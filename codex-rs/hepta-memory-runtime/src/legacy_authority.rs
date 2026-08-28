//! Compatibility adapter from the legacy production lease to typed authority.
//!
//! This module does not trust a local qualification profile and does not mint
//! authority from evidence booleans. It binds the exact legacy lease fields to
//! `AuthorityLeaseBinding`, invokes the mandatory external
//! `ProductionAuthorityVerifier`, and only then returns
//! `Authorized<CognitiveWriteCapability>`.

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::AuthorityLeaseBinding;
use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::CapabilityVerificationRequest;
use codex_hepta_contracts::CapabilityVerifier;
use codex_hepta_contracts::CognitiveWriteCapability;
use codex_hepta_contracts::authorize_verified_capability;
use codex_hepta_memory::ProductionAuthorityLease;
use codex_hepta_memory::ProductionAuthorityVerifier;
use codex_hepta_memory::ProductionWriterError;

#[derive(Clone, Debug)]
pub struct ProductionCognitiveWriteAuthorization {
    lease: ProductionAuthorityLease,
    capability: Authorized<CognitiveWriteCapability>,
}

impl ProductionCognitiveWriteAuthorization {
    pub fn verify<V>(
        lease: ProductionAuthorityLease,
        verifier: &V,
        expected_agent: &AgentId,
        lease_generation: u64,
    ) -> Result<Self, ProductionWriterError>
    where
        V: ProductionAuthorityVerifier + ?Sized,
    {
        let binding = AuthorityLeaseBinding::new(
            lease.agent_id.clone(),
            lease.grant_digest.clone(),
            lease.authority_epoch,
            lease.owner_epoch,
            lease_generation,
            lease.fencing_token_digest()?,
            lease.lease_expires_at_unix_seconds,
        )
        .map_err(|error| {
            ProductionWriterError::AuthorityRejected(format!(
                "typed cognitive-write binding rejected: {error}"
            ))
        })?;
        let adapter = LegacyProductionCapabilityVerifier {
            lease: &lease,
            verifier,
        };
        let capability = authorize_verified_capability::<CognitiveWriteCapability, _>(
            binding,
            expected_agent,
            lease_generation,
            now_unix_seconds()?,
            &adapter,
        )
        .map_err(|error| {
            ProductionWriterError::AuthorityRejected(format!(
                "typed cognitive-write authorization rejected: {error}"
            ))
        })?;
        Ok(Self { lease, capability })
    }

    pub fn lease(&self) -> &ProductionAuthorityLease {
        &self.lease
    }

    pub fn capability(&self) -> &Authorized<CognitiveWriteCapability> {
        &self.capability
    }

    pub fn into_parts(
        self,
    ) -> (
        ProductionAuthorityLease,
        Authorized<CognitiveWriteCapability>,
    ) {
        (self.lease, self.capability)
    }
}

struct LegacyProductionCapabilityVerifier<'a, V>
where
    V: ProductionAuthorityVerifier + ?Sized,
{
    lease: &'a ProductionAuthorityLease,
    verifier: &'a V,
}

impl<V> CapabilityVerifier for LegacyProductionCapabilityVerifier<'_, V>
where
    V: ProductionAuthorityVerifier + ?Sized,
{
    fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
        if request.action() != AuthorityAction::WriteCognitiveState {
            return Err(
                "legacy production lease may only mint cognitive-write authority".to_string(),
            );
        }
        let binding = request.binding();
        let fencing_token_sha256 = self
            .lease
            .fencing_token_digest()
            .map_err(|error| error.to_string())?;
        if binding.subject_agent_id() != &self.lease.agent_id
            || binding.grant_sha256() != &self.lease.grant_digest
            || binding.authority_epoch() != self.lease.authority_epoch
            || binding.owner_epoch() != self.lease.owner_epoch
            || binding.generation() != request.expected_generation()
            || binding.expires_at_unix_seconds()
                != self.lease.lease_expires_at_unix_seconds
            || binding.fencing_token_sha256() != &fencing_token_sha256
        {
            return Err("legacy production lease drifted from typed binding".to_string());
        }
        self.verifier
            .verify(self.lease, request.expected_agent_id())
    }
}

fn now_unix_seconds() -> Result<u64, ProductionWriterError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| ProductionWriterError::Invalid(format!("system clock failed: {error}")))
}

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityAction;
    use codex_hepta_contracts::Sha256Digest;
    use codex_hepta_memory::ProductionAuthorityLease;
    use codex_hepta_memory::ProductionAuthorityToken;
    use codex_hepta_memory::ProductionAuthorityVerifier;
    use codex_hepta_memory::ProductionWriterError;

    use super::ProductionCognitiveWriteAuthorization;
    use super::now_unix_seconds;

    const OWNER_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
    const OTHER_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c13";

    fn agent_id(value: &str) -> AgentId {
        AgentId::parse(value)
            .unwrap_or_else(|error| panic!("test AgentId must parse: {error}"))
    }

    fn lease() -> ProductionAuthorityLease {
        let token = ProductionAuthorityToken::from_verified_bytes(
            b"externally-verified-supervisor-token".to_vec(),
        )
        .unwrap_or_else(|error| panic!("test token must be valid: {error}"));
        ProductionAuthorityLease::from_verified_parts(
            agent_id(OWNER_ID),
            Sha256Digest::for_bytes(b"signed-production-grant"),
            5,
            8,
            now_unix_seconds().unwrap_or_else(|error| panic!("clock must work: {error}")) + 3_600,
            token,
        )
        .unwrap_or_else(|error| panic!("test lease must be valid: {error}"))
    }

    struct AllowVerifier;

    impl ProductionAuthorityVerifier for AllowVerifier {
        fn verify(
            &self,
            authority: &ProductionAuthorityLease,
            expected_agent: &AgentId,
        ) -> Result<(), String> {
            if &authority.agent_id != expected_agent {
                return Err("signed grant Agent mismatch".to_string());
            }
            Ok(())
        }
    }

    struct DenyVerifier;

    impl ProductionAuthorityVerifier for DenyVerifier {
        fn verify(
            &self,
            _authority: &ProductionAuthorityLease,
            _expected_agent: &AgentId,
        ) -> Result<(), String> {
            Err("independent verifier denied grant scope".to_string())
        }
    }

    #[test]
    fn verified_legacy_lease_mints_external_cognitive_write_only() {
        let authorization = ProductionCognitiveWriteAuthorization::verify(
            lease(),
            &AllowVerifier,
            &agent_id(OWNER_ID),
            3,
        )
        .unwrap_or_else(|error| panic!("legacy lease must adapt: {error}"));
        assert_eq!(
            authorization.capability().action(),
            AuthorityAction::WriteCognitiveState
        );
        assert!(authorization.capability().is_external());
        assert_eq!(authorization.capability().generation(), 3);
        assert_eq!(
            authorization.capability().grant_sha256(),
            &authorization.lease().grant_digest
        );
    }

    #[test]
    fn verifier_denial_and_agent_or_generation_mismatch_fail_closed() {
        assert!(matches!(
            ProductionCognitiveWriteAuthorization::verify(
                lease(),
                &DenyVerifier,
                &agent_id(OWNER_ID),
                3,
            ),
            Err(ProductionWriterError::AuthorityRejected(reason))
                if reason.contains("independent verifier denied grant scope")
        ));
        assert!(matches!(
            ProductionCognitiveWriteAuthorization::verify(
                lease(),
                &AllowVerifier,
                &agent_id(OTHER_ID),
                3,
            ),
            Err(ProductionWriterError::AuthorityRejected(reason))
                if reason.contains("subject does not match")
        ));
        assert!(matches!(
            ProductionCognitiveWriteAuthorization::verify(
                lease(),
                &AllowVerifier,
                &agent_id(OWNER_ID),
                0,
            ),
            Err(ProductionWriterError::AuthorityRejected(reason))
                if reason.contains("generation must be non-zero")
        ));
    }
}
