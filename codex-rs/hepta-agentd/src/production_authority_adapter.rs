//! Agentd integration seam for the Memory runtime production adapter.
//!
//! The actual legacy-lease attenuation is owned by
//! `codex-hepta-memory-runtime`; Agentd only consumes the resulting typed
//! cognitive-write authorization before it opens the durable writer host.

pub(crate) use codex_hepta_memory_runtime::ProductionCognitiveWriteAuthorization;

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

    const OWNER_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
    const OTHER_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c13";

    fn agent_id(value: &str) -> AgentId {
        AgentId::parse(value)
            .unwrap_or_else(|error| panic!("test AgentId must parse: {error}"))
    }

    fn lease() -> ProductionAuthorityLease {
        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|error| panic!("clock must work: {error}"))
            .as_secs()
            + 3_600;
        ProductionAuthorityLease::from_verified_parts(
            agent_id(OWNER_ID),
            Sha256Digest::for_bytes(b"signed-production-grant"),
            5,
            8,
            expires_at,
            ProductionAuthorityToken::from_verified_bytes(
                b"externally-verified-supervisor-token".to_vec(),
            )
            .unwrap_or_else(|error| panic!("test token must be valid: {error}")),
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
    fn agentd_consumes_external_typed_cognitive_write_only() {
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
    }

    #[test]
    fn agentd_never_bypasses_verifier_or_agent_binding() {
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
    }
}
