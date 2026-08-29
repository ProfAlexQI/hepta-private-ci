use std::collections::BTreeSet;
use std::fmt;
use std::marker::PhantomData;

use serde::Serialize;

use crate::AgentId;
use crate::Sha256Digest;

pub const AUTHORITY_KERNEL_SCHEMA_VERSION: u32 = 2;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeAuthorityProfile {
    SnapshotReadOnly,
    AgentLocal,
    QualificationCognitiveWrite,
}

impl RuntimeAuthorityProfile {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotReadOnly => "snapshot_read_only",
            Self::AgentLocal => "agent_local",
            Self::QualificationCognitiveWrite => "qualification_cognitive_write",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityAction {
    ServeSession,
    ReadMemory,
    MutateMemoryFederation,
    MutateAutomation,
    WriteCognitiveState,
    InvokeModel,
    DispatchProvider,
    ExternalEffect,
    MutateFleet,
    AcceptOperator,
    PromoteRelease,
}

impl AuthorityAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServeSession => "serve_session",
            Self::ReadMemory => "read_memory",
            Self::MutateMemoryFederation => "mutate_memory_federation",
            Self::MutateAutomation => "mutate_automation",
            Self::WriteCognitiveState => "write_cognitive_state",
            Self::InvokeModel => "invoke_model",
            Self::DispatchProvider => "dispatch_provider",
            Self::ExternalEffect => "external_effect",
            Self::MutateFleet => "mutate_fleet",
            Self::AcceptOperator => "accept_operator",
            Self::PromoteRelease => "promote_release",
        }
    }

    pub const fn is_release_or_external(self) -> bool {
        matches!(
            self,
            Self::InvokeModel
                | Self::DispatchProvider
                | Self::ExternalEffect
                | Self::MutateFleet
                | Self::AcceptOperator
                | Self::PromoteRelease
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityError {
    ZeroGeneration,
    SubjectMismatch,
    GenerationMismatch,
    ActionDenied(AuthorityAction),
    ProfileInvariant,
    InvalidLeaseBinding(&'static str),
    LeaseExpired { deadline: u64 },
    VerificationRejected(String),
}

impl fmt::Display for AuthorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroGeneration => formatter.write_str("authority generation must be non-zero"),
            Self::SubjectMismatch => formatter.write_str("authority subject does not match"),
            Self::GenerationMismatch => formatter.write_str("authority generation does not match"),
            Self::ActionDenied(action) => {
                write!(formatter, "authority action {} is denied", action.as_str())
            }
            Self::ProfileInvariant => {
                formatter.write_str("authority profile violates the closed-world action set")
            }
            Self::InvalidLeaseBinding(reason) => {
                write!(formatter, "authority lease binding is invalid: {reason}")
            }
            Self::LeaseExpired { deadline } => {
                write!(formatter, "authority lease expired at {deadline}")
            }
            Self::VerificationRejected(reason) => {
                write!(
                    formatter,
                    "authority verifier rejected capability: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for AuthorityError {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityGrant {
    schema_version: u32,
    subject_agent_id: AgentId,
    generation: u64,
    profile: RuntimeAuthorityProfile,
    actions: BTreeSet<AuthorityAction>,
}

impl AuthorityGrant {
    pub fn snapshot_read_only(
        subject_agent_id: AgentId,
        generation: u64,
    ) -> Result<Self, AuthorityError> {
        Self::new(
            subject_agent_id,
            generation,
            RuntimeAuthorityProfile::SnapshotReadOnly,
            [AuthorityAction::ReadMemory],
        )
    }

    pub fn agent_local(subject_agent_id: AgentId, generation: u64) -> Result<Self, AuthorityError> {
        Self::new(
            subject_agent_id,
            generation,
            RuntimeAuthorityProfile::AgentLocal,
            [
                AuthorityAction::ServeSession,
                AuthorityAction::ReadMemory,
                AuthorityAction::MutateMemoryFederation,
                AuthorityAction::MutateAutomation,
            ],
        )
    }

    pub fn qualification_cognitive_write(
        subject_agent_id: AgentId,
        generation: u64,
    ) -> Result<Self, AuthorityError> {
        Self::new(
            subject_agent_id,
            generation,
            RuntimeAuthorityProfile::QualificationCognitiveWrite,
            [
                AuthorityAction::ServeSession,
                AuthorityAction::ReadMemory,
                AuthorityAction::MutateMemoryFederation,
                AuthorityAction::MutateAutomation,
                AuthorityAction::WriteCognitiveState,
            ],
        )
    }

    fn new<const N: usize>(
        subject_agent_id: AgentId,
        generation: u64,
        profile: RuntimeAuthorityProfile,
        actions: [AuthorityAction; N],
    ) -> Result<Self, AuthorityError> {
        if generation == 0 {
            return Err(AuthorityError::ZeroGeneration);
        }
        let grant = Self {
            schema_version: AUTHORITY_KERNEL_SCHEMA_VERSION,
            subject_agent_id,
            generation,
            profile,
            actions: actions.into_iter().collect(),
        };
        grant.validate()?;
        Ok(grant)
    }

    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn subject_agent_id(&self) -> &AgentId {
        &self.subject_agent_id
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn profile(&self) -> RuntimeAuthorityProfile {
        self.profile
    }

    pub fn actions(&self) -> impl ExactSizeIterator<Item = AuthorityAction> + '_ {
        self.actions.iter().copied()
    }

    pub fn allows(&self, action: AuthorityAction) -> bool {
        self.actions.contains(&action)
    }

    pub fn authorize<C>(&self) -> Result<Authorized<C>, AuthorityError>
    where
        C: AuthorityCapability,
    {
        if !self.allows(C::ACTION) {
            return Err(AuthorityError::ActionDenied(C::ACTION));
        }
        Ok(Authorized {
            source: AuthorizationSource::Local {
                grant_sha256: self.digest(),
                subject_agent_id: self.subject_agent_id.clone(),
                generation: self.generation,
            },
            action: C::ACTION,
            marker: PhantomData,
        })
    }

    pub fn validate_binding(
        &self,
        subject_agent_id: &AgentId,
        generation: u64,
    ) -> Result<(), AuthorityError> {
        if &self.subject_agent_id != subject_agent_id {
            return Err(AuthorityError::SubjectMismatch);
        }
        if self.generation != generation {
            return Err(AuthorityError::GenerationMismatch);
        }
        self.validate()
    }

    pub fn dangerous_actions(&self) -> Vec<AuthorityAction> {
        self.actions
            .iter()
            .copied()
            .filter(|action| action.is_release_or_external())
            .collect()
    }

    pub fn is_product_closed(&self) -> bool {
        self.dangerous_actions().is_empty()
    }

    pub fn digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:authority-grant:v2");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.subject_agent_id.as_str().as_bytes());
        frame(&mut bytes, &self.generation.to_be_bytes());
        frame(&mut bytes, self.profile.as_str().as_bytes());
        for action in &self.actions {
            frame(&mut bytes, action.as_str().as_bytes());
        }
        Sha256Digest::for_bytes(&bytes)
    }

    fn validate(&self) -> Result<(), AuthorityError> {
        if self.schema_version != AUTHORITY_KERNEL_SCHEMA_VERSION || self.generation == 0 {
            return Err(AuthorityError::ProfileInvariant);
        }
        let expected: BTreeSet<AuthorityAction> = match self.profile {
            RuntimeAuthorityProfile::SnapshotReadOnly => {
                [AuthorityAction::ReadMemory].into_iter().collect()
            }
            RuntimeAuthorityProfile::AgentLocal => [
                AuthorityAction::ServeSession,
                AuthorityAction::ReadMemory,
                AuthorityAction::MutateMemoryFederation,
                AuthorityAction::MutateAutomation,
            ]
            .into_iter()
            .collect(),
            RuntimeAuthorityProfile::QualificationCognitiveWrite => [
                AuthorityAction::ServeSession,
                AuthorityAction::ReadMemory,
                AuthorityAction::MutateMemoryFederation,
                AuthorityAction::MutateAutomation,
                AuthorityAction::WriteCognitiveState,
            ]
            .into_iter()
            .collect(),
        };
        if self.actions != expected || !self.is_product_closed() {
            return Err(AuthorityError::ProfileInvariant);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityLeaseBinding {
    subject_agent_id: AgentId,
    grant_sha256: Sha256Digest,
    authority_epoch: u64,
    owner_epoch: u64,
    generation: u64,
    fencing_token_sha256: Sha256Digest,
    expires_at_unix_seconds: u64,
}

impl AuthorityLeaseBinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subject_agent_id: AgentId,
        grant_sha256: Sha256Digest,
        authority_epoch: u64,
        owner_epoch: u64,
        generation: u64,
        fencing_token_sha256: Sha256Digest,
        expires_at_unix_seconds: u64,
    ) -> Result<Self, AuthorityError> {
        if authority_epoch == 0 {
            return Err(AuthorityError::InvalidLeaseBinding(
                "authority epoch must be non-zero",
            ));
        }
        if owner_epoch == 0 {
            return Err(AuthorityError::InvalidLeaseBinding(
                "owner epoch must be non-zero",
            ));
        }
        if generation == 0 {
            return Err(AuthorityError::ZeroGeneration);
        }
        if expires_at_unix_seconds == 0 {
            return Err(AuthorityError::InvalidLeaseBinding(
                "expiry must be non-zero",
            ));
        }
        Ok(Self {
            subject_agent_id,
            grant_sha256,
            authority_epoch,
            owner_epoch,
            generation,
            fencing_token_sha256,
            expires_at_unix_seconds,
        })
    }

    pub fn subject_agent_id(&self) -> &AgentId {
        &self.subject_agent_id
    }

    pub fn grant_sha256(&self) -> &Sha256Digest {
        &self.grant_sha256
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

    pub fn expires_at_unix_seconds(&self) -> u64 {
        self.expires_at_unix_seconds
    }

    pub fn is_expired_at(&self, observed_at_unix_seconds: u64) -> bool {
        observed_at_unix_seconds >= self.expires_at_unix_seconds
    }

    pub fn digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:authority-lease-binding:v1");
        frame(&mut bytes, self.subject_agent_id.as_str().as_bytes());
        frame(&mut bytes, self.grant_sha256.as_str().as_bytes());
        frame(&mut bytes, &self.authority_epoch.to_be_bytes());
        frame(&mut bytes, &self.owner_epoch.to_be_bytes());
        frame(&mut bytes, &self.generation.to_be_bytes());
        frame(&mut bytes, self.fencing_token_sha256.as_str().as_bytes());
        frame(&mut bytes, &self.expires_at_unix_seconds.to_be_bytes());
        Sha256Digest::for_bytes(&bytes)
    }
}

#[derive(Debug)]
pub struct CapabilityVerificationRequest<'a> {
    action: AuthorityAction,
    binding: &'a AuthorityLeaseBinding,
    expected_agent_id: &'a AgentId,
    expected_generation: u64,
    observed_at_unix_seconds: u64,
}

impl<'a> CapabilityVerificationRequest<'a> {
    pub fn action(&self) -> AuthorityAction {
        self.action
    }

    pub fn binding(&self) -> &'a AuthorityLeaseBinding {
        self.binding
    }

    pub fn expected_agent_id(&self) -> &'a AgentId {
        self.expected_agent_id
    }

    pub fn expected_generation(&self) -> u64 {
        self.expected_generation
    }

    pub fn observed_at_unix_seconds(&self) -> u64 {
        self.observed_at_unix_seconds
    }
}

pub trait CapabilityVerifier: Send + Sync {
    fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String>;
}

impl<F> CapabilityVerifier for F
where
    F: for<'a> Fn(&CapabilityVerificationRequest<'a>) -> Result<(), String> + Send + Sync,
{
    fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
        self(request)
    }
}

pub fn authorize_verified_capability<C, V>(
    binding: AuthorityLeaseBinding,
    expected_agent_id: &AgentId,
    expected_generation: u64,
    observed_at_unix_seconds: u64,
    verifier: &V,
) -> Result<Authorized<C>, AuthorityError>
where
    C: AuthorityCapability,
    V: CapabilityVerifier + ?Sized,
{
    if binding.subject_agent_id() != expected_agent_id {
        return Err(AuthorityError::SubjectMismatch);
    }
    if binding.generation() != expected_generation {
        return Err(AuthorityError::GenerationMismatch);
    }
    if binding.is_expired_at(observed_at_unix_seconds) {
        return Err(AuthorityError::LeaseExpired {
            deadline: binding.expires_at_unix_seconds(),
        });
    }
    {
        let request = CapabilityVerificationRequest {
            action: C::ACTION,
            binding: &binding,
            expected_agent_id,
            expected_generation,
            observed_at_unix_seconds,
        };
        verifier
            .verify(&request)
            .map_err(AuthorityError::VerificationRejected)?;
    }
    Ok(Authorized {
        source: AuthorizationSource::ExternalLease(binding),
        action: C::ACTION,
        marker: PhantomData,
    })
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

pub trait AuthorityCapability {
    const ACTION: AuthorityAction;
}

macro_rules! capability {
    ($name:ident, $action:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;

        impl AuthorityCapability for $name {
            const ACTION: AuthorityAction = $action;
        }
    };
}

capability!(SessionServeCapability, AuthorityAction::ServeSession);
capability!(MemoryReadCapability, AuthorityAction::ReadMemory);
capability!(
    MemoryFederationMutationCapability,
    AuthorityAction::MutateMemoryFederation
);
capability!(
    AutomationMutationCapability,
    AuthorityAction::MutateAutomation
);
capability!(
    CognitiveWriteCapability,
    AuthorityAction::WriteCognitiveState
);
capability!(ModelInvocationCapability, AuthorityAction::InvokeModel);
capability!(
    ProviderDispatchCapability,
    AuthorityAction::DispatchProvider
);
capability!(ExternalEffectCapability, AuthorityAction::ExternalEffect);
capability!(FleetMutationCapability, AuthorityAction::MutateFleet);
capability!(
    OperatorAcceptanceCapability,
    AuthorityAction::AcceptOperator
);
capability!(ReleasePromotionCapability, AuthorityAction::PromoteRelease);

#[derive(Clone, Debug, Eq, PartialEq)]
enum AuthorizationSource {
    Local {
        grant_sha256: Sha256Digest,
        subject_agent_id: AgentId,
        generation: u64,
    },
    ExternalLease(AuthorityLeaseBinding),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authorized<C>
where
    C: AuthorityCapability,
{
    source: AuthorizationSource,
    action: AuthorityAction,
    marker: PhantomData<C>,
}

impl<C> Authorized<C>
where
    C: AuthorityCapability,
{
    pub fn grant_sha256(&self) -> &Sha256Digest {
        match &self.source {
            AuthorizationSource::Local { grant_sha256, .. } => grant_sha256,
            AuthorizationSource::ExternalLease(binding) => binding.grant_sha256(),
        }
    }

    pub fn subject_agent_id(&self) -> &AgentId {
        match &self.source {
            AuthorizationSource::Local {
                subject_agent_id, ..
            } => subject_agent_id,
            AuthorizationSource::ExternalLease(binding) => binding.subject_agent_id(),
        }
    }

    pub fn generation(&self) -> u64 {
        match &self.source {
            AuthorizationSource::Local { generation, .. } => *generation,
            AuthorizationSource::ExternalLease(binding) => binding.generation(),
        }
    }

    pub fn action(&self) -> AuthorityAction {
        self.action
    }

    pub fn external_lease_binding(&self) -> Option<&AuthorityLeaseBinding> {
        match &self.source {
            AuthorizationSource::Local { .. } => None,
            AuthorizationSource::ExternalLease(binding) => Some(binding),
        }
    }

    pub fn is_external(&self) -> bool {
        self.external_lease_binding().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn agent_id() -> AgentId {
        match AgentId::parse(AGENT_ID) {
            Ok(agent_id) => agent_id,
            Err(error) => panic!("test AgentId must parse: {error}"),
        }
    }

    fn lease_binding(generation: u64, expiry: u64) -> AuthorityLeaseBinding {
        match AuthorityLeaseBinding::new(
            agent_id(),
            Sha256Digest::for_bytes(b"signed-supervisor-grant"),
            7,
            11,
            generation,
            Sha256Digest::for_bytes(b"opaque-fencing-token"),
            expiry,
        ) {
            Ok(binding) => binding,
            Err(error) => panic!("test lease binding must be valid: {error}"),
        }
    }

    struct AllowCognitiveWrite;

    impl CapabilityVerifier for AllowCognitiveWrite {
        fn verify(&self, request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
            if request.action() != AuthorityAction::WriteCognitiveState {
                return Err("unexpected capability action".to_string());
            }
            if request.binding().subject_agent_id() != request.expected_agent_id()
                || request.binding().generation() != request.expected_generation()
            {
                return Err("binding drift".to_string());
            }
            Ok(())
        }
    }

    struct DenyCapability;

    impl CapabilityVerifier for DenyCapability {
        fn verify(&self, _request: &CapabilityVerificationRequest<'_>) -> Result<(), String> {
            Err("signed grant scope denied".to_string())
        }
    }

    #[test]
    fn agent_local_profile_is_closed_and_typed() {
        let grant = match AuthorityGrant::agent_local(agent_id(), 7) {
            Ok(grant) => grant,
            Err(error) => panic!("agent-local grant must be valid: {error}"),
        };
        assert!(grant.is_product_closed());
        assert!(grant.authorize::<SessionServeCapability>().is_ok());
        assert!(grant.authorize::<MemoryReadCapability>().is_ok());
        assert!(grant.authorize::<AutomationMutationCapability>().is_ok());
        assert!(matches!(
            grant.authorize::<CognitiveWriteCapability>(),
            Err(AuthorityError::ActionDenied(
                AuthorityAction::WriteCognitiveState
            ))
        ));
        assert!(matches!(
            grant.authorize::<ExternalEffectCapability>(),
            Err(AuthorityError::ActionDenied(
                AuthorityAction::ExternalEffect
            ))
        ));
    }

    #[test]
    fn qualification_profile_adds_only_cognitive_write() {
        let local = match AuthorityGrant::agent_local(agent_id(), 9) {
            Ok(grant) => grant,
            Err(error) => panic!("agent-local grant must be valid: {error}"),
        };
        let qualification = match AuthorityGrant::qualification_cognitive_write(agent_id(), 9) {
            Ok(grant) => grant,
            Err(error) => panic!("qualification grant must be valid: {error}"),
        };
        let local_actions: BTreeSet<_> = local.actions().collect();
        let qualification_actions: BTreeSet<_> = qualification.actions().collect();
        let delta: Vec<_> = qualification_actions
            .difference(&local_actions)
            .copied()
            .collect();
        assert_eq!(delta, vec![AuthorityAction::WriteCognitiveState]);
        assert!(qualification.is_product_closed());
    }

    #[test]
    fn grant_digest_binds_subject_generation_and_profile() {
        let first = match AuthorityGrant::agent_local(agent_id(), 1) {
            Ok(grant) => grant,
            Err(error) => panic!("first grant must be valid: {error}"),
        };
        let second = match AuthorityGrant::agent_local(agent_id(), 2) {
            Ok(grant) => grant,
            Err(error) => panic!("second grant must be valid: {error}"),
        };
        let qualification = match AuthorityGrant::qualification_cognitive_write(agent_id(), 1) {
            Ok(grant) => grant,
            Err(error) => panic!("qualification grant must be valid: {error}"),
        };
        assert_ne!(first.digest(), second.digest());
        assert_ne!(first.digest(), qualification.digest());
    }

    #[test]
    fn externally_verified_lease_mints_only_the_requested_typed_capability() {
        let binding = lease_binding(3, 500);
        let authorized = match authorize_verified_capability::<CognitiveWriteCapability, _>(
            binding,
            &agent_id(),
            3,
            100,
            &AllowCognitiveWrite,
        ) {
            Ok(authorized) => authorized,
            Err(error) => panic!("external cognitive write must authorize: {error}"),
        };
        assert_eq!(authorized.action(), AuthorityAction::WriteCognitiveState);
        assert_eq!(authorized.generation(), 3);
        assert!(authorized.is_external());
        assert!(authorized.external_lease_binding().is_some());
    }

    #[test]
    fn external_lease_rejects_expiry_and_verifier_denial() {
        assert!(matches!(
            authorize_verified_capability::<CognitiveWriteCapability, _>(
                lease_binding(4, 100),
                &agent_id(),
                4,
                100,
                &AllowCognitiveWrite,
            ),
            Err(AuthorityError::LeaseExpired { deadline: 100 })
        ));
        assert!(matches!(
            authorize_verified_capability::<CognitiveWriteCapability, _>(
                lease_binding(4, 200),
                &agent_id(),
                4,
                100,
                &DenyCapability,
            ),
            Err(AuthorityError::VerificationRejected(reason))
                if reason == "signed grant scope denied"
        ));
    }

    #[test]
    fn zero_generation_is_rejected() {
        assert!(matches!(
            AuthorityGrant::agent_local(agent_id(), 0),
            Err(AuthorityError::ZeroGeneration)
        ));
        assert!(matches!(
            AuthorityLeaseBinding::new(
                agent_id(),
                Sha256Digest::for_bytes(b"grant"),
                1,
                1,
                0,
                Sha256Digest::for_bytes(b"fence"),
                10,
            ),
            Err(AuthorityError::ZeroGeneration)
        ));
    }
}
