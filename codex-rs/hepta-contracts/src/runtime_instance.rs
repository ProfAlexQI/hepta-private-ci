use std::collections::BTreeMap;
use std::fmt;

use serde::Serialize;

use crate::AgentId;
use crate::AuthorityGrant;
use crate::ProductComponentId;
use crate::ProductGraph;
use crate::RuntimeAuthorityProfile;
use crate::Sha256Digest;

pub const RUNTIME_INSTANCE_GRAPH_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeServiceRequirement {
    Required,
    Optional,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeServiceState {
    Starting,
    Ready,
    Degraded,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeComponentStatus {
    pub component: ProductComponentId,
    pub requirement: RuntimeServiceRequirement,
    pub state: RuntimeServiceState,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInstanceGraph {
    schema_version: u32,
    subject_agent_id: AgentId,
    generation: u64,
    profile: RuntimeAuthorityProfile,
    authority_grant_sha256: Sha256Digest,
    canonical_product_graph_sha256: Sha256Digest,
    components: Vec<RuntimeComponentStatus>,
}

impl RuntimeInstanceGraph {
    pub fn agent_composed(
        authority: &AuthorityGrant,
        product_graph: &ProductGraph,
        memory_available: bool,
        automation_available: bool,
    ) -> Result<Self, RuntimeInstanceGraphError> {
        if !matches!(
            authority.profile(),
            RuntimeAuthorityProfile::AgentLocal
                | RuntimeAuthorityProfile::QualificationCognitiveWrite
        ) {
            return Err(RuntimeInstanceGraphError::UnsupportedProfile);
        }
        if product_graph.authority_grant_sha256() != &authority.digest() {
            return Err(RuntimeInstanceGraphError::AuthorityDrift);
        }

        let memory_requirement = match authority.profile() {
            RuntimeAuthorityProfile::QualificationCognitiveWrite => {
                RuntimeServiceRequirement::Required
            }
            RuntimeAuthorityProfile::AgentLocal => RuntimeServiceRequirement::Optional,
            RuntimeAuthorityProfile::SnapshotReadOnly => {
                return Err(RuntimeInstanceGraphError::UnsupportedProfile);
            }
        };
        if memory_requirement == RuntimeServiceRequirement::Required && !memory_available {
            return Err(RuntimeInstanceGraphError::RequiredUnavailable(
                ProductComponentId::MemoryRuntime,
            ));
        }

        let components = vec![
            RuntimeComponentStatus {
                component: ProductComponentId::Supervisor,
                requirement: RuntimeServiceRequirement::Required,
                state: RuntimeServiceState::Ready,
            },
            RuntimeComponentStatus {
                component: ProductComponentId::Agentd,
                requirement: RuntimeServiceRequirement::Required,
                state: RuntimeServiceState::Ready,
            },
            RuntimeComponentStatus {
                component: ProductComponentId::AppServer,
                requirement: RuntimeServiceRequirement::Required,
                state: RuntimeServiceState::Starting,
            },
            RuntimeComponentStatus {
                component: ProductComponentId::MemoryRuntime,
                requirement: memory_requirement,
                state: if memory_available {
                    RuntimeServiceState::Ready
                } else {
                    RuntimeServiceState::Degraded
                },
            },
            RuntimeComponentStatus {
                component: ProductComponentId::AutomationRuntime,
                requirement: RuntimeServiceRequirement::Optional,
                state: if automation_available {
                    RuntimeServiceState::Ready
                } else {
                    RuntimeServiceState::Degraded
                },
            },
            RuntimeComponentStatus {
                component: ProductComponentId::MatrixIngress,
                requirement: RuntimeServiceRequirement::Disabled,
                state: RuntimeServiceState::Disabled,
            },
            RuntimeComponentStatus {
                component: ProductComponentId::ProviderEffectAdapter,
                requirement: RuntimeServiceRequirement::Disabled,
                state: RuntimeServiceState::Disabled,
            },
        ];

        let graph = Self {
            schema_version: RUNTIME_INSTANCE_GRAPH_SCHEMA_VERSION,
            subject_agent_id: authority.subject_agent_id().clone(),
            generation: authority.generation(),
            profile: authority.profile(),
            authority_grant_sha256: authority.digest(),
            canonical_product_graph_sha256: product_graph.digest(),
            components,
        };
        graph.validate_against(authority, product_graph)?;
        Ok(graph)
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

    pub fn authority_grant_sha256(&self) -> &Sha256Digest {
        &self.authority_grant_sha256
    }

    pub fn canonical_product_graph_sha256(&self) -> &Sha256Digest {
        &self.canonical_product_graph_sha256
    }

    pub fn components(&self) -> &[RuntimeComponentStatus] {
        &self.components
    }

    pub fn component_status(
        &self,
        component: ProductComponentId,
    ) -> Option<RuntimeComponentStatus> {
        self.components
            .iter()
            .copied()
            .find(|status| status.component == component)
    }

    pub fn mark_ready(
        &mut self,
        component: ProductComponentId,
    ) -> Result<(), RuntimeInstanceGraphError> {
        let status = self
            .components
            .iter_mut()
            .find(|status| status.component == component)
            .ok_or(RuntimeInstanceGraphError::UnknownComponent(component))?;
        if status.requirement == RuntimeServiceRequirement::Disabled {
            return Err(RuntimeInstanceGraphError::DisabledComponentActivated(
                component,
            ));
        }
        status.state = RuntimeServiceState::Ready;
        Ok(())
    }

    pub fn mark_degraded(
        &mut self,
        component: ProductComponentId,
    ) -> Result<(), RuntimeInstanceGraphError> {
        let status = self
            .components
            .iter_mut()
            .find(|status| status.component == component)
            .ok_or(RuntimeInstanceGraphError::UnknownComponent(component))?;
        if status.requirement != RuntimeServiceRequirement::Optional {
            return Err(RuntimeInstanceGraphError::RequiredUnavailable(component));
        }
        status.state = RuntimeServiceState::Degraded;
        Ok(())
    }

    pub fn ready(&self) -> bool {
        self.components.iter().all(|status| match status.requirement {
            RuntimeServiceRequirement::Required => status.state == RuntimeServiceState::Ready,
            RuntimeServiceRequirement::Optional => matches!(
                status.state,
                RuntimeServiceState::Ready | RuntimeServiceState::Degraded
            ),
            RuntimeServiceRequirement::Disabled => {
                status.state == RuntimeServiceState::Disabled
            }
        })
    }

    pub fn validate_against(
        &self,
        authority: &AuthorityGrant,
        product_graph: &ProductGraph,
    ) -> Result<(), RuntimeInstanceGraphError> {
        if self.schema_version != RUNTIME_INSTANCE_GRAPH_SCHEMA_VERSION
            || self.subject_agent_id != *authority.subject_agent_id()
            || self.generation != authority.generation()
            || self.profile != authority.profile()
            || self.authority_grant_sha256 != authority.digest()
            || self.canonical_product_graph_sha256 != product_graph.digest()
        {
            return Err(RuntimeInstanceGraphError::AuthorityDrift);
        }

        let statuses = self
            .components
            .iter()
            .map(|status| (status.component, *status))
            .collect::<BTreeMap<_, _>>();
        if statuses.len() != self.components.len()
            || statuses.len() != product_graph.components().len()
            || product_graph
                .components()
                .iter()
                .any(|component| !statuses.contains_key(component))
        {
            return Err(RuntimeInstanceGraphError::TopologyDrift);
        }
        for status in &self.components {
            match (status.requirement, status.state) {
                (
                    RuntimeServiceRequirement::Required,
                    RuntimeServiceState::Starting | RuntimeServiceState::Ready,
                )
                | (
                    RuntimeServiceRequirement::Optional,
                    RuntimeServiceState::Ready | RuntimeServiceState::Degraded,
                )
                | (
                    RuntimeServiceRequirement::Disabled,
                    RuntimeServiceState::Disabled,
                ) => {}
                (RuntimeServiceRequirement::Required, _) => {
                    return Err(RuntimeInstanceGraphError::RequiredUnavailable(
                        status.component,
                    ));
                }
                (RuntimeServiceRequirement::Optional, _) => {
                    return Err(RuntimeInstanceGraphError::InvalidOptionalState(
                        status.component,
                    ));
                }
                (RuntimeServiceRequirement::Disabled, _) => {
                    return Err(RuntimeInstanceGraphError::DisabledComponentActivated(
                        status.component,
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:runtime-instance-graph:v1");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.subject_agent_id.as_str().as_bytes());
        frame(&mut bytes, &self.generation.to_be_bytes());
        frame(&mut bytes, profile_name(self.profile).as_bytes());
        frame(&mut bytes, self.authority_grant_sha256.as_str().as_bytes());
        frame(
            &mut bytes,
            self.canonical_product_graph_sha256.as_str().as_bytes(),
        );
        for status in &self.components {
            frame(&mut bytes, status.component.as_str().as_bytes());
            frame(&mut bytes, requirement_name(status.requirement).as_bytes());
            frame(&mut bytes, state_name(status.state).as_bytes());
        }
        Sha256Digest::for_bytes(&bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeInstanceGraphError {
    UnsupportedProfile,
    AuthorityDrift,
    TopologyDrift,
    RequiredUnavailable(ProductComponentId),
    InvalidOptionalState(ProductComponentId),
    DisabledComponentActivated(ProductComponentId),
    UnknownComponent(ProductComponentId),
}

impl fmt::Display for RuntimeInstanceGraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedProfile => {
                formatter.write_str("runtime profile cannot host an Agent instance graph")
            }
            Self::AuthorityDrift => {
                formatter.write_str("runtime instance authority or graph binding drifted")
            }
            Self::TopologyDrift => {
                formatter.write_str("runtime instance components drifted from the product graph")
            }
            Self::RequiredUnavailable(component) => write!(
                formatter,
                "required runtime component {} is unavailable",
                component.as_str()
            ),
            Self::InvalidOptionalState(component) => write!(
                formatter,
                "optional runtime component {} has an invalid state",
                component.as_str()
            ),
            Self::DisabledComponentActivated(component) => write!(
                formatter,
                "disabled runtime component {} was activated",
                component.as_str()
            ),
            Self::UnknownComponent(component) => write!(
                formatter,
                "runtime component {} is not part of this instance",
                component.as_str()
            ),
        }
    }
}

impl std::error::Error for RuntimeInstanceGraphError {}

fn profile_name(profile: RuntimeAuthorityProfile) -> &'static str {
    match profile {
        RuntimeAuthorityProfile::SnapshotReadOnly => "snapshot_read_only",
        RuntimeAuthorityProfile::AgentLocal => "agent_local",
        RuntimeAuthorityProfile::QualificationCognitiveWrite => {
            "qualification_cognitive_write"
        }
    }
}

fn requirement_name(requirement: RuntimeServiceRequirement) -> &'static str {
    match requirement {
        RuntimeServiceRequirement::Required => "required",
        RuntimeServiceRequirement::Optional => "optional",
        RuntimeServiceRequirement::Disabled => "disabled",
    }
}

fn state_name(state: RuntimeServiceState) -> &'static str {
    match state {
        RuntimeServiceState::Starting => "starting",
        RuntimeServiceState::Ready => "ready",
        RuntimeServiceState::Degraded => "degraded",
        RuntimeServiceState::Disabled => "disabled",
    }
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
mod tests {
    use crate::AuthorityGrant;
    use crate::ProductComponentId;
    use crate::ProductGraph;

    use super::*;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    fn agent_id() -> AgentId {
        AgentId::parse(AGENT_ID).unwrap_or_else(|error| panic!("AgentId must parse: {error}"))
    }

    #[test]
    fn local_instance_distinguishes_optional_degradation_from_readiness() {
        let authority = AuthorityGrant::agent_local(agent_id(), 3)
            .unwrap_or_else(|error| panic!("authority must build: {error}"));
        let product_graph = ProductGraph::agent_local(&authority)
            .unwrap_or_else(|error| panic!("product graph must build: {error}"));
        let mut instance = RuntimeInstanceGraph::agent_composed(
            &authority,
            &product_graph,
            false,
            false,
        )
        .unwrap_or_else(|error| panic!("instance must build: {error}"));
        assert!(!instance.ready());
        assert_eq!(
            instance
                .component_status(ProductComponentId::MemoryRuntime)
                .map(|status| (status.requirement, status.state)),
            Some((
                RuntimeServiceRequirement::Optional,
                RuntimeServiceState::Degraded,
            ))
        );
        instance
            .mark_ready(ProductComponentId::AppServer)
            .unwrap_or_else(|error| panic!("App Server must become ready: {error}"));
        assert!(instance.ready());
        instance
            .validate_against(&authority, &product_graph)
            .unwrap_or_else(|error| panic!("instance must validate: {error}"));
    }

    #[test]
    fn qualification_instance_requires_memory() {
        let authority = AuthorityGrant::qualification_cognitive_write(agent_id(), 3)
            .unwrap_or_else(|error| panic!("authority must build: {error}"));
        let product_graph = ProductGraph::agent_local(&authority)
            .unwrap_or_else(|error| panic!("product graph must build: {error}"));
        assert_eq!(
            RuntimeInstanceGraph::agent_composed(&authority, &product_graph, false, true),
            Err(RuntimeInstanceGraphError::RequiredUnavailable(
                ProductComponentId::MemoryRuntime,
            ))
        );
    }

    #[test]
    fn disabled_component_cannot_become_ready() {
        let authority = AuthorityGrant::agent_local(agent_id(), 3)
            .unwrap_or_else(|error| panic!("authority must build: {error}"));
        let product_graph = ProductGraph::agent_local(&authority)
            .unwrap_or_else(|error| panic!("product graph must build: {error}"));
        let mut instance = RuntimeInstanceGraph::agent_composed(
            &authority,
            &product_graph,
            true,
            true,
        )
        .unwrap_or_else(|error| panic!("instance must build: {error}"));
        assert_eq!(
            instance.mark_ready(ProductComponentId::ProviderEffectAdapter),
            Err(RuntimeInstanceGraphError::DisabledComponentActivated(
                ProductComponentId::ProviderEffectAdapter,
            ))
        );
    }
}
