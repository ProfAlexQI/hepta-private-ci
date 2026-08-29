use std::fmt;

use serde::Serialize;

use crate::AgentId;
use crate::AuthorityAction;
use crate::AuthorityCapability;
use crate::AuthorityError;
use crate::AuthorityGrant;
use crate::AuthorityLeaseBinding;
use crate::Authorized;
use crate::Sha256Digest;

pub const RUNTIME_AUTHORITY_CONTEXT_SCHEMA_VERSION: u32 = 1;

/// Runtime-owned authority identity derived from durable control-plane state.
///
/// `authority_epoch` tracks the selected release lineage, `owner_epoch` tracks
/// the supervisor lifecycle owner, and `generation` binds the concrete Agentd
/// process. The fencing token must be derived from the exact durable release,
/// lifecycle and local authority inputs by the composition root.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeAuthorityContext {
    schema_version: u32,
    subject_agent_id: AgentId,
    authority_epoch: u64,
    owner_epoch: u64,
    generation: u64,
    fencing_token_sha256: Sha256Digest,
    authority_grant_sha256: Sha256Digest,
}

impl RuntimeAuthorityContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subject_agent_id: AgentId,
        authority_epoch: u64,
        owner_epoch: u64,
        generation: u64,
        fencing_token_sha256: Sha256Digest,
        authority_grant_sha256: Sha256Digest,
    ) -> Result<Self, AuthorityError> {
        if authority_epoch == 0 {
            return Err(AuthorityError::InvalidLeaseBinding(
                "runtime authority epoch must be non-zero",
            ));
        }
        if owner_epoch == 0 {
            return Err(AuthorityError::InvalidLeaseBinding(
                "runtime owner epoch must be non-zero",
            ));
        }
        if generation == 0 {
            return Err(AuthorityError::ZeroGeneration);
        }
        Ok(Self {
            schema_version: RUNTIME_AUTHORITY_CONTEXT_SCHEMA_VERSION,
            subject_agent_id,
            authority_epoch,
            owner_epoch,
            generation,
            fencing_token_sha256,
            authority_grant_sha256,
        })
    }

    pub fn from_external_binding(binding: &AuthorityLeaseBinding) -> Result<Self, AuthorityError> {
        Self::new(
            binding.subject_agent_id().clone(),
            binding.authority_epoch(),
            binding.owner_epoch(),
            binding.generation(),
            binding.fencing_token_sha256().clone(),
            binding.grant_sha256().clone(),
        )
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn subject_agent_id(&self) -> &AgentId {
        &self.subject_agent_id
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

    pub fn fencing_token_sha256(&self) -> &Sha256Digest {
        &self.fencing_token_sha256
    }

    pub fn authority_grant_sha256(&self) -> &Sha256Digest {
        &self.authority_grant_sha256
    }

    pub fn validate_grant(&self, grant: &AuthorityGrant) -> Result<(), AuthorityError> {
        if self.schema_version != RUNTIME_AUTHORITY_CONTEXT_SCHEMA_VERSION {
            return Err(AuthorityError::ProfileInvariant);
        }
        grant.validate_binding(&self.subject_agent_id, self.generation)?;
        if grant.digest() != self.authority_grant_sha256 {
            return Err(AuthorityError::VerificationRejected(
                "runtime authority context does not match the local grant".to_string(),
            ));
        }
        Ok(())
    }

    pub fn digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:runtime-authority-context:v1");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.subject_agent_id.as_str().as_bytes());
        frame(&mut bytes, &self.authority_epoch.to_be_bytes());
        frame(&mut bytes, &self.owner_epoch.to_be_bytes());
        frame(&mut bytes, &self.generation.to_be_bytes());
        frame(&mut bytes, self.fencing_token_sha256.as_str().as_bytes());
        frame(&mut bytes, self.authority_grant_sha256.as_str().as_bytes());
        Sha256Digest::for_bytes(&bytes)
    }
}

/// Exact request presented to the current capability-use verifier.
///
/// The verifier is invoked on every use, not only when an external capability
/// is first minted. A production implementation can therefore reject a
/// revoked grant, advanced epoch, superseded owner or policy change before the
/// physical effect boundary is crossed.
#[derive(Debug)]
pub struct CapabilityUseVerificationRequest<'a> {
    action: AuthorityAction,
    subject_agent_id: &'a AgentId,
    generation: u64,
    grant_sha256: &'a Sha256Digest,
    external_binding: Option<&'a AuthorityLeaseBinding>,
    runtime_context: &'a RuntimeAuthorityContext,
    observed_at_unix_seconds: u64,
}

impl<'a> CapabilityUseVerificationRequest<'a> {
    pub fn action(&self) -> AuthorityAction {
        self.action
    }

    pub fn subject_agent_id(&self) -> &'a AgentId {
        self.subject_agent_id
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn grant_sha256(&self) -> &'a Sha256Digest {
        self.grant_sha256
    }

    pub fn external_binding(&self) -> Option<&'a AuthorityLeaseBinding> {
        self.external_binding
    }

    pub fn runtime_context(&self) -> &'a RuntimeAuthorityContext {
        self.runtime_context
    }

    pub fn observed_at_unix_seconds(&self) -> u64 {
        self.observed_at_unix_seconds
    }
}

pub trait CapabilityUseVerifier: Send + Sync {
    fn verify_use(&self, request: &CapabilityUseVerificationRequest<'_>) -> Result<(), String>;
}

impl<F> CapabilityUseVerifier for F
where
    F: for<'a> Fn(&CapabilityUseVerificationRequest<'a>) -> Result<(), String> + Send + Sync,
{
    fn verify_use(&self, request: &CapabilityUseVerificationRequest<'_>) -> Result<(), String> {
        self(request)
    }
}

pub fn verify_capability_use<C, V>(
    capability: &Authorized<C>,
    runtime_context: &RuntimeAuthorityContext,
    observed_at_unix_seconds: u64,
    verifier: &V,
) -> Result<(), AuthorityError>
where
    C: AuthorityCapability,
    V: CapabilityUseVerifier + ?Sized,
{
    if runtime_context.schema_version != RUNTIME_AUTHORITY_CONTEXT_SCHEMA_VERSION {
        return Err(AuthorityError::ProfileInvariant);
    }
    if capability.action() != C::ACTION {
        return Err(AuthorityError::ActionDenied(C::ACTION));
    }
    if capability.subject_agent_id() != runtime_context.subject_agent_id() {
        return Err(AuthorityError::SubjectMismatch);
    }
    if capability.generation() != runtime_context.generation() {
        return Err(AuthorityError::GenerationMismatch);
    }

    let external_binding = capability.external_lease_binding();
    match external_binding {
        Some(binding) => {
            if binding.grant_sha256() != runtime_context.authority_grant_sha256()
                || binding.authority_epoch() != runtime_context.authority_epoch()
                || binding.owner_epoch() != runtime_context.owner_epoch()
                || binding.fencing_token_sha256() != runtime_context.fencing_token_sha256()
            {
                return Err(AuthorityError::VerificationRejected(
                    "external capability drifted from the current runtime authority context"
                        .to_string(),
                ));
            }
            if binding.is_expired_at(observed_at_unix_seconds) {
                return Err(AuthorityError::LeaseExpired {
                    deadline: binding.expires_at_unix_seconds(),
                });
            }
        }
        None => {
            if capability.grant_sha256() != runtime_context.authority_grant_sha256() {
                return Err(AuthorityError::VerificationRejected(
                    "local capability drifted from the current runtime authority context"
                        .to_string(),
                ));
            }
        }
    }

    verifier
        .verify_use(&CapabilityUseVerificationRequest {
            action: capability.action(),
            subject_agent_id: capability.subject_agent_id(),
            generation: capability.generation(),
            grant_sha256: capability.grant_sha256(),
            external_binding,
            runtime_context,
            observed_at_unix_seconds,
        })
        .map_err(AuthorityError::VerificationRejected)
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

impl fmt::Display for RuntimeAuthorityContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "runtime-authority:{}:{}:{}:{}",
            self.subject_agent_id, self.authority_epoch, self.owner_epoch, self.generation
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use crate::AuthorityAction;
    use crate::AuthorityGrant;
    use crate::AuthorityLeaseBinding;
    use crate::CapabilityVerificationRequest;
    use crate::CapabilityVerifier;
    use crate::CognitiveWriteCapability;
    use crate::ExternalEffectCapability;
    use crate::Sha256Digest;
    use crate::authorize_verified_capability;

    use super::*;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn agent_id() -> AgentId {
        AgentId::parse(AGENT_ID).unwrap_or_else(|error| panic!("AgentId must parse: {error}"))
    }

    struct MintVerifier;

    impl CapabilityVerifier for MintVerifier {
        fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
            if request.action() != AuthorityAction::ExternalEffect {
                return Err("unexpected mint action".to_string());
            }
            Ok(())
        }
    }

    #[test]
    fn local_capability_is_bound_to_the_exact_runtime_context() {
        let grant = AuthorityGrant::qualification_cognitive_write(agent_id(), 3)
            .unwrap_or_else(|error| panic!("grant must build: {error}"));
        let context = RuntimeAuthorityContext::new(
            agent_id(),
            7,
            11,
            3,
            Sha256Digest::for_bytes(b"runtime-fence"),
            grant.digest(),
        )
        .unwrap_or_else(|error| panic!("context must build: {error}"));
        context
            .validate_grant(&grant)
            .unwrap_or_else(|error| panic!("grant must validate: {error}"));
        let capability = grant
            .authorize::<CognitiveWriteCapability>()
            .unwrap_or_else(|error| panic!("capability must authorize: {error}"));
        verify_capability_use(
            &capability,
            &context,
            100,
            &|request: &CapabilityUseVerificationRequest<'_>| {
                if request.external_binding().is_some() {
                    return Err("local capability unexpectedly external".to_string());
                }
                Ok(())
            },
        )
        .unwrap_or_else(|error| panic!("local use must verify: {error}"));

        let changed = RuntimeAuthorityContext::new(
            agent_id(),
            7,
            12,
            3,
            Sha256Digest::for_bytes(b"changed-fence"),
            Sha256Digest::for_bytes(b"changed-grant"),
        )
        .unwrap_or_else(|error| panic!("changed context must build: {error}"));
        assert!(
            verify_capability_use(
                &capability,
                &changed,
                100,
                &|_: &CapabilityUseVerificationRequest<'_>| Ok(())
            )
            .is_err()
        );
    }

    #[test]
    fn external_capability_is_reverified_on_every_use_and_revocation_fails_closed() {
        let grant_digest = Sha256Digest::for_bytes(b"signed-effect-grant");
        let fence = Sha256Digest::for_bytes(b"effect-fence");
        let binding =
            AuthorityLeaseBinding::new(agent_id(), grant_digest.clone(), 7, 11, 3, fence, 500)
                .unwrap_or_else(|error| panic!("binding must build: {error}"));
        let capability = authorize_verified_capability::<ExternalEffectCapability, _>(
            binding.clone(),
            &agent_id(),
            3,
            100,
            &MintVerifier,
        )
        .unwrap_or_else(|error| panic!("capability must mint: {error}"));
        let context = RuntimeAuthorityContext::from_external_binding(&binding)
            .unwrap_or_else(|error| panic!("context must build: {error}"));
        let calls = AtomicUsize::new(0);
        let verifier = |request: &CapabilityUseVerificationRequest<'_>| {
            calls.fetch_add(1, Ordering::SeqCst);
            if request.observed_at_unix_seconds() >= 200 {
                Err("grant was revoked at the current authority epoch".to_string())
            } else {
                Ok(())
            }
        };
        verify_capability_use(&capability, &context, 101, &verifier)
            .unwrap_or_else(|error| panic!("first use must verify: {error}"));
        verify_capability_use(&capability, &context, 102, &verifier)
            .unwrap_or_else(|error| panic!("second use must verify: {error}"));
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert!(matches!(
            verify_capability_use(&capability, &context, 200, &verifier),
            Err(AuthorityError::VerificationRejected(reason))
                if reason.contains("revoked")
        ));
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn changed_epoch_fence_and_expiry_are_rejected_before_use_verifier() {
        let binding = AuthorityLeaseBinding::new(
            agent_id(),
            Sha256Digest::for_bytes(b"signed-effect-grant"),
            7,
            11,
            3,
            Sha256Digest::for_bytes(b"effect-fence"),
            200,
        )
        .unwrap_or_else(|error| panic!("binding must build: {error}"));
        let capability = authorize_verified_capability::<ExternalEffectCapability, _>(
            binding,
            &agent_id(),
            3,
            100,
            &MintVerifier,
        )
        .unwrap_or_else(|error| panic!("capability must mint: {error}"));
        let changed = RuntimeAuthorityContext::new(
            agent_id(),
            8,
            11,
            3,
            Sha256Digest::for_bytes(b"changed-fence"),
            Sha256Digest::for_bytes(b"signed-effect-grant"),
        )
        .unwrap_or_else(|error| panic!("context must build: {error}"));
        let calls = AtomicUsize::new(0);
        assert!(
            verify_capability_use(
                &capability,
                &changed,
                100,
                &|_: &CapabilityUseVerificationRequest<'_>| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                },
            )
            .is_err()
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }
}
