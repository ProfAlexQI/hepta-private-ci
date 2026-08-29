use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;

use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::AuthorityError;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::ProductComponentId;
use codex_hepta_contracts::ProductGraph;
use codex_hepta_contracts::ProductGraphError;
use codex_hepta_contracts::RuntimeAuthorityProfile;
use codex_hepta_contracts::Sha256Digest;

pub(crate) const RUNTIME_PROFILE_CONTRACT_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum RuntimeServiceId {
    Supervisor,
    Agentd,
    AppServer,
    MemoryRuntime,
    AutomationRuntime,
    MatrixIngress,
    ProviderEffectAdapter,
}

impl RuntimeServiceId {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Supervisor => "supervisor",
            Self::Agentd => "agentd",
            Self::AppServer => "app_server",
            Self::MemoryRuntime => "memory_runtime",
            Self::AutomationRuntime => "automation_runtime",
            Self::MatrixIngress => "matrix_ingress",
            Self::ProviderEffectAdapter => "provider_effect_adapter",
        }
    }

    const fn product_component(self) -> ProductComponentId {
        match self {
            Self::Supervisor => ProductComponentId::Supervisor,
            Self::Agentd => ProductComponentId::Agentd,
            Self::AppServer => ProductComponentId::AppServer,
            Self::MemoryRuntime => ProductComponentId::MemoryRuntime,
            Self::AutomationRuntime => ProductComponentId::AutomationRuntime,
            Self::MatrixIngress => ProductComponentId::MatrixIngress,
            Self::ProviderEffectAdapter => ProductComponentId::ProviderEffectAdapter,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RuntimeServicePlacement {
    ControlPlane,
    InProcess,
    AdapterProcess,
    DormantBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RuntimeServiceRequirement {
    Required,
    Optional,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RuntimeFailureMode {
    FailClosed,
    Degraded,
    NotStarted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeServicePolicy {
    pub(crate) service: RuntimeServiceId,
    pub(crate) placement: RuntimeServicePlacement,
    pub(crate) requirement: RuntimeServiceRequirement,
    pub(crate) failure_mode: RuntimeFailureMode,
    pub(crate) readiness_required: bool,
}

impl RuntimeServicePolicy {
    const fn required(
        service: RuntimeServiceId,
        placement: RuntimeServicePlacement,
        readiness_required: bool,
    ) -> Self {
        Self {
            service,
            placement,
            requirement: RuntimeServiceRequirement::Required,
            failure_mode: RuntimeFailureMode::FailClosed,
            readiness_required,
        }
    }

    const fn optional(
        service: RuntimeServiceId,
        placement: RuntimeServicePlacement,
    ) -> Self {
        Self {
            service,
            placement,
            requirement: RuntimeServiceRequirement::Optional,
            failure_mode: RuntimeFailureMode::Degraded,
            readiness_required: false,
        }
    }

    const fn disabled(
        service: RuntimeServiceId,
        placement: RuntimeServicePlacement,
    ) -> Self {
        Self {
            service,
            placement,
            requirement: RuntimeServiceRequirement::Disabled,
            failure_mode: RuntimeFailureMode::NotStarted,
            readiness_required: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeProfileContract {
    schema_version: u32,
    profile: RuntimeAuthorityProfile,
    authority_grant_sha256: Sha256Digest,
    services: Vec<RuntimeServicePolicy>,
}

impl RuntimeProfileContract {
    pub(crate) fn for_authority(
        authority: &AuthorityGrant,
    ) -> Result<Self, RuntimeProfileContractError> {
        authority
            .validate_binding(authority.subject_agent_id(), authority.generation())
            .map_err(RuntimeProfileContractError::Authority)?;
        let escaped = authority.dangerous_actions();
        if !escaped.is_empty() {
            return Err(RuntimeProfileContractError::EscapedAuthority(escaped));
        }
        validate_action_set(authority)?;

        let services = match authority.profile() {
            RuntimeAuthorityProfile::SnapshotReadOnly => vec![
                RuntimeServicePolicy::required(
                    RuntimeServiceId::Supervisor,
                    RuntimeServicePlacement::ControlPlane,
                    true,
                ),
                RuntimeServicePolicy::disabled(
                    RuntimeServiceId::Agentd,
                    RuntimeServicePlacement::InProcess,
                ),
                RuntimeServicePolicy::disabled(
                    RuntimeServiceId::AppServer,
                    RuntimeServicePlacement::InProcess,
                ),
                RuntimeServicePolicy::required(
                    RuntimeServiceId::MemoryRuntime,
                    RuntimeServicePlacement::InProcess,
                    true,
                ),
                RuntimeServicePolicy::disabled(
                    RuntimeServiceId::AutomationRuntime,
                    RuntimeServicePlacement::InProcess,
                ),
                RuntimeServicePolicy::disabled(
                    RuntimeServiceId::MatrixIngress,
                    RuntimeServicePlacement::AdapterProcess,
                ),
                RuntimeServicePolicy::disabled(
                    RuntimeServiceId::ProviderEffectAdapter,
                    RuntimeServicePlacement::DormantBoundary,
                ),
            ],
            RuntimeAuthorityProfile::AgentLocal => vec![
                RuntimeServicePolicy::required(
                    RuntimeServiceId::Supervisor,
                    RuntimeServicePlacement::ControlPlane,
                    true,
                ),
                RuntimeServicePolicy::required(
                    RuntimeServiceId::Agentd,
                    RuntimeServicePlacement::InProcess,
                    true,
                ),
                RuntimeServicePolicy::required(
                    RuntimeServiceId::AppServer,
                    RuntimeServicePlacement::InProcess,
                    true,
                ),
                RuntimeServicePolicy::optional(
                    RuntimeServiceId::MemoryRuntime,
                    RuntimeServicePlacement::InProcess,
                ),
                RuntimeServicePolicy::optional(
                    RuntimeServiceId::AutomationRuntime,
                    RuntimeServicePlacement::InProcess,
                ),
                RuntimeServicePolicy::optional(
                    RuntimeServiceId::MatrixIngress,
                    RuntimeServicePlacement::AdapterProcess,
                ),
                RuntimeServicePolicy::disabled(
                    RuntimeServiceId::ProviderEffectAdapter,
                    RuntimeServicePlacement::DormantBoundary,
                ),
            ],
            RuntimeAuthorityProfile::QualificationCognitiveWrite => vec![
                RuntimeServicePolicy::required(
                    RuntimeServiceId::Supervisor,
                    RuntimeServicePlacement::ControlPlane,
                    true,
                ),
                RuntimeServicePolicy::required(
                    RuntimeServiceId::Agentd,
                    RuntimeServicePlacement::InProcess,
                    true,
                ),
                RuntimeServicePolicy::required(
                    RuntimeServiceId::AppServer,
                    RuntimeServicePlacement::InProcess,
                    true,
                ),
                RuntimeServicePolicy::required(
                    RuntimeServiceId::MemoryRuntime,
                    RuntimeServicePlacement::InProcess,
                    true,
                ),
                RuntimeServicePolicy::optional(
                    RuntimeServiceId::AutomationRuntime,
                    RuntimeServicePlacement::InProcess,
                ),
                RuntimeServicePolicy::disabled(
                    RuntimeServiceId::MatrixIngress,
                    RuntimeServicePlacement::AdapterProcess,
                ),
                RuntimeServicePolicy::disabled(
                    RuntimeServiceId::ProviderEffectAdapter,
                    RuntimeServicePlacement::DormantBoundary,
                ),
            ],
        };

        let contract = Self {
            schema_version: RUNTIME_PROFILE_CONTRACT_SCHEMA_VERSION,
            profile: authority.profile(),
            authority_grant_sha256: authority.digest(),
            services,
        };
        contract.validate()?;
        Ok(contract)
    }

    pub(crate) fn profile(&self) -> RuntimeAuthorityProfile {
        self.profile
    }

    pub(crate) fn policy(
        &self,
        service: RuntimeServiceId,
    ) -> Result<&RuntimeServicePolicy, RuntimeProfileContractError> {
        self.services
            .iter()
            .find(|policy| policy.service == service)
            .ok_or(RuntimeProfileContractError::MissingService(service))
    }

    pub(crate) fn validate_product_graph(
        &self,
        graph: &ProductGraph,
    ) -> Result<(), RuntimeProfileContractError> {
        if self.profile == RuntimeAuthorityProfile::SnapshotReadOnly {
            return Err(RuntimeProfileContractError::SnapshotDoesNotHostProductGraph);
        }
        graph
            .validate()
            .map_err(RuntimeProfileContractError::ProductGraph)?;
        if graph.authority_grant_sha256() != &self.authority_grant_sha256 {
            return Err(RuntimeProfileContractError::AuthorityDigestDrift);
        }
        let components = graph.components().iter().copied().collect::<BTreeSet<_>>();
        for service in all_services() {
            if !components.contains(&service.product_component()) {
                return Err(RuntimeProfileContractError::MissingGraphComponent(service));
            }
        }
        Ok(())
    }

    pub(crate) fn validate_composed_services(
        &self,
        memory_available: bool,
        automation_available: bool,
    ) -> Result<(), RuntimeProfileContractError> {
        self.validate_availability(RuntimeServiceId::MemoryRuntime, memory_available)?;
        self.validate_availability(RuntimeServiceId::AutomationRuntime, automation_available)?;
        Ok(())
    }

    fn validate_availability(
        &self,
        service: RuntimeServiceId,
        available: bool,
    ) -> Result<(), RuntimeProfileContractError> {
        match (self.policy(service)?.requirement, available) {
            (RuntimeServiceRequirement::Required, false) => {
                Err(RuntimeProfileContractError::RequiredServiceUnavailable(service))
            }
            (RuntimeServiceRequirement::Disabled, true) => {
                Err(RuntimeProfileContractError::DisabledServiceStarted(service))
            }
            _ => Ok(()),
        }
    }

    fn validate(&self) -> Result<(), RuntimeProfileContractError> {
        if self.schema_version != RUNTIME_PROFILE_CONTRACT_SCHEMA_VERSION {
            return Err(RuntimeProfileContractError::InvalidPolicy(
                "schema version drifted",
            ));
        }
        let by_service = self
            .services
            .iter()
            .map(|policy| (policy.service, *policy))
            .collect::<BTreeMap<_, _>>();
        if by_service.len() != self.services.len() {
            return Err(RuntimeProfileContractError::DuplicateService);
        }
        for service in all_services() {
            let policy = by_service
                .get(&service)
                .ok_or(RuntimeProfileContractError::MissingService(service))?;
            match policy.requirement {
                RuntimeServiceRequirement::Required => {
                    if policy.failure_mode != RuntimeFailureMode::FailClosed {
                        return Err(RuntimeProfileContractError::InvalidPolicy(
                            "required service must fail closed",
                        ));
                    }
                }
                RuntimeServiceRequirement::Optional => {
                    if policy.failure_mode != RuntimeFailureMode::Degraded
                        || policy.readiness_required
                    {
                        return Err(RuntimeProfileContractError::InvalidPolicy(
                            "optional service must degrade and cannot gate readiness",
                        ));
                    }
                }
                RuntimeServiceRequirement::Disabled => {
                    if policy.failure_mode != RuntimeFailureMode::NotStarted
                        || policy.readiness_required
                    {
                        return Err(RuntimeProfileContractError::InvalidPolicy(
                            "disabled service must remain not-started",
                        ));
                    }
                }
            }
            if policy.placement == RuntimeServicePlacement::DormantBoundary
                && policy.requirement != RuntimeServiceRequirement::Disabled
            {
                return Err(RuntimeProfileContractError::InvalidPolicy(
                    "dormant boundary cannot be enabled by a local runtime profile",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RuntimeProfileContractError {
    Authority(AuthorityError),
    ProductGraph(ProductGraphError),
    ActionSetDrift,
    EscapedAuthority(Vec<AuthorityAction>),
    DuplicateService,
    MissingService(RuntimeServiceId),
    InvalidPolicy(&'static str),
    AuthorityDigestDrift,
    MissingGraphComponent(RuntimeServiceId),
    RequiredServiceUnavailable(RuntimeServiceId),
    DisabledServiceStarted(RuntimeServiceId),
    SnapshotDoesNotHostProductGraph,
}

impl fmt::Display for RuntimeProfileContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authority(error) => write!(formatter, "runtime profile authority failed: {error}"),
            Self::ProductGraph(error) => write!(formatter, "runtime profile graph failed: {error}"),
            Self::ActionSetDrift => formatter.write_str("runtime profile authority action set drifted"),
            Self::EscapedAuthority(actions) => {
                write!(formatter, "runtime profile received escaped authority: {actions:?}")
            }
            Self::DuplicateService => formatter.write_str("runtime profile has duplicate services"),
            Self::MissingService(service) => {
                write!(formatter, "runtime profile is missing service {}", service.as_str())
            }
            Self::InvalidPolicy(reason) => write!(formatter, "runtime profile policy is invalid: {reason}"),
            Self::AuthorityDigestDrift => {
                formatter.write_str("runtime profile authority digest drifted from product graph")
            }
            Self::MissingGraphComponent(service) => write!(
                formatter,
                "runtime profile service {} is missing from the product graph",
                service.as_str()
            ),
            Self::RequiredServiceUnavailable(service) => write!(
                formatter,
                "required runtime service {} is unavailable",
                service.as_str()
            ),
            Self::DisabledServiceStarted(service) => write!(
                formatter,
                "disabled runtime service {} was started",
                service.as_str()
            ),
            Self::SnapshotDoesNotHostProductGraph => formatter
                .write_str("snapshot_read_only does not host the Agent product graph"),
        }
    }
}

impl std::error::Error for RuntimeProfileContractError {}

fn validate_action_set(authority: &AuthorityGrant) -> Result<(), RuntimeProfileContractError> {
    let expected = match authority.profile() {
        RuntimeAuthorityProfile::SnapshotReadOnly => {
            [AuthorityAction::ReadMemory].into_iter().collect::<BTreeSet<_>>()
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
    if authority.actions().collect::<BTreeSet<_>>() != expected {
        return Err(RuntimeProfileContractError::ActionSetDrift);
    }
    Ok(())
}

const fn all_services() -> [RuntimeServiceId; 7] {
    [
        RuntimeServiceId::Supervisor,
        RuntimeServiceId::Agentd,
        RuntimeServiceId::AppServer,
        RuntimeServiceId::MemoryRuntime,
        RuntimeServiceId::AutomationRuntime,
        RuntimeServiceId::MatrixIngress,
        RuntimeServiceId::ProviderEffectAdapter,
    ]
}

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityGrant;
    use codex_hepta_contracts::ProductGraph;
    use codex_hepta_contracts::RuntimeAuthorityProfile;

    use super::RuntimeProfileContract;
    use super::RuntimeProfileContractError;
    use super::RuntimeServiceId;
    use super::RuntimeServiceRequirement;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn agent_id() -> AgentId {
        AgentId::parse(AGENT_ID)
            .unwrap_or_else(|error| panic!("test AgentId must parse: {error}"))
    }

    #[test]
    fn agent_local_profile_declares_optional_memory_and_automation() {
        let authority = AuthorityGrant::agent_local(agent_id(), 1)
            .unwrap_or_else(|error| panic!("authority must build: {error}"));
        let contract = RuntimeProfileContract::for_authority(&authority)
            .unwrap_or_else(|error| panic!("profile must build: {error}"));
        let graph = ProductGraph::agent_local(&authority)
            .unwrap_or_else(|error| panic!("graph must build: {error}"));
        assert_eq!(contract.profile(), RuntimeAuthorityProfile::AgentLocal);
        assert_eq!(
            contract
                .policy(RuntimeServiceId::MemoryRuntime)
                .expect("Memory policy")
                .requirement,
            RuntimeServiceRequirement::Optional
        );
        assert_eq!(
            contract
                .policy(RuntimeServiceId::AutomationRuntime)
                .expect("Automation policy")
                .requirement,
            RuntimeServiceRequirement::Optional
        );
        assert!(contract.validate_product_graph(&graph).is_ok());
        assert!(contract.validate_composed_services(false, false).is_ok());
    }

    #[test]
    fn qualification_profile_requires_memory_and_keeps_effects_disabled() {
        let authority = AuthorityGrant::qualification_cognitive_write(agent_id(), 2)
            .unwrap_or_else(|error| panic!("authority must build: {error}"));
        let contract = RuntimeProfileContract::for_authority(&authority)
            .unwrap_or_else(|error| panic!("profile must build: {error}"));
        assert_eq!(
            contract
                .policy(RuntimeServiceId::MemoryRuntime)
                .expect("Memory policy")
                .requirement,
            RuntimeServiceRequirement::Required
        );
        assert_eq!(
            contract
                .policy(RuntimeServiceId::ProviderEffectAdapter)
                .expect("Provider policy")
                .requirement,
            RuntimeServiceRequirement::Disabled
        );
        assert!(matches!(
            contract.validate_composed_services(false, true),
            Err(RuntimeProfileContractError::RequiredServiceUnavailable(
                RuntimeServiceId::MemoryRuntime
            ))
        ));
        assert!(contract.validate_composed_services(true, false).is_ok());
    }

    #[test]
    fn snapshot_profile_is_valid_but_cannot_host_agent_product_graph() {
        let snapshot = AuthorityGrant::snapshot_read_only(agent_id(), 3)
            .unwrap_or_else(|error| panic!("snapshot authority must build: {error}"));
        let contract = RuntimeProfileContract::for_authority(&snapshot)
            .unwrap_or_else(|error| panic!("snapshot profile must build: {error}"));
        let local = AuthorityGrant::agent_local(agent_id(), 3)
            .unwrap_or_else(|error| panic!("local authority must build: {error}"));
        let graph = ProductGraph::agent_local(&local)
            .unwrap_or_else(|error| panic!("graph must build: {error}"));
        assert!(matches!(
            contract.validate_product_graph(&graph),
            Err(RuntimeProfileContractError::SnapshotDoesNotHostProductGraph)
        ));
    }
}
