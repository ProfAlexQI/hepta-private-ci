use std::collections::BTreeSet;
use std::fmt;
use std::marker::PhantomData;

use serde::Serialize;

use crate::AgentId;
use crate::Sha256Digest;

pub const AUTHORITY_KERNEL_SCHEMA_VERSION: u32 = 1;

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

    pub fn agent_local(
        subject_agent_id: AgentId,
        generation: u64,
    ) -> Result<Self, AuthorityError> {
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
            grant_sha256: self.digest(),
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
        frame(&mut bytes, b"hepta:authority-grant:v1");
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
            RuntimeAuthorityProfile::SnapshotReadOnly => [AuthorityAction::ReadMemory]
                .into_iter()
                .collect(),
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
capability!(ProviderDispatchCapability, AuthorityAction::DispatchProvider);
capability!(ExternalEffectCapability, AuthorityAction::ExternalEffect);
capability!(FleetMutationCapability, AuthorityAction::MutateFleet);
capability!(OperatorAcceptanceCapability, AuthorityAction::AcceptOperator);
capability!(ReleasePromotionCapability, AuthorityAction::PromoteRelease);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authorized<C>
where
    C: AuthorityCapability,
{
    grant_sha256: Sha256Digest,
    action: AuthorityAction,
    marker: PhantomData<C>,
}

impl<C> Authorized<C>
where
    C: AuthorityCapability,
{
    pub fn grant_sha256(&self) -> &Sha256Digest {
        &self.grant_sha256
    }

    pub fn action(&self) -> AuthorityAction {
        self.action
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
            Err(AuthorityError::ActionDenied(AuthorityAction::WriteCognitiveState))
        ));
        assert!(matches!(
            grant.authorize::<ExternalEffectCapability>(),
            Err(AuthorityError::ActionDenied(AuthorityAction::ExternalEffect))
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
    fn zero_generation_is_rejected() {
        assert!(matches!(
            AuthorityGrant::agent_local(agent_id(), 0),
            Err(AuthorityError::ZeroGeneration)
        ));
    }
}
