use std::fs::File;
use std::sync::Arc;

use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::ProductGraph;
use codex_hepta_contracts::RuntimeAuthorityContext;
use codex_hepta_contracts::RuntimeInstanceGraph;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::AgentRecord;

use crate::AgentdConfig;
use crate::AgentdError;
use crate::AgentdIdentity;
use crate::AgentdState;
use crate::automation_service::AgentAutomationService;
use crate::memory_service::AgentMemoryService;
use crate::runtime_profile::RuntimeProfileContract;

const EVENT_CAPACITY: usize = 128;

pub(crate) struct AgentRuntimeComposition {
    identity: AgentdIdentity,
    state: Arc<AgentdState>,
    memory_service: AgentMemoryService,
    automation_service: AgentAutomationService,
    authority: AuthorityGrant,
    runtime_authority: RuntimeAuthorityContext,
    product_graph: ProductGraph,
    runtime_instance: RuntimeInstanceGraph,
    writer_lock: File,
}

pub(crate) struct AgentRuntimeParts {
    pub(crate) identity: AgentdIdentity,
    pub(crate) state: Arc<AgentdState>,
    pub(crate) memory_service: AgentMemoryService,
    pub(crate) automation_service: AgentAutomationService,
    pub(crate) authority: AuthorityGrant,
    pub(crate) runtime_authority: RuntimeAuthorityContext,
    pub(crate) product_graph: ProductGraph,
    pub(crate) runtime_instance: RuntimeInstanceGraph,
    pub(crate) writer_lock: File,
}

impl AgentRuntimeComposition {
    pub(crate) async fn open(config: AgentdConfig) -> Result<Self, AgentdError> {
        let (identity, registry, writer_lock) = config.into_parts();
        let snapshot = registry.load()?;
        let record = snapshot
            .agent(&identity.agent_id)
            .cloned()
            .ok_or_else(|| {
                AgentdError::GenerationFenced(format!(
                    "agent {} disappeared before product composition",
                    identity.agent_id
                ))
            })?;
        let authority = authority_for_identity(&identity)?;
        authority
            .validate_binding(&identity.agent_id, identity.spawn_generation)
            .map_err(|error| {
                AgentdError::Protocol(format!("validate Agent authority binding: {error}"))
            })?;
        let runtime_authority = runtime_authority_context(&record, &identity, &authority)?;
        let product_graph = ProductGraph::agent_local(&authority).map_err(|error| {
            AgentdError::Protocol(format!("validate Agent product graph: {error}"))
        })?;
        let runtime_profile = RuntimeProfileContract::for_authority(&authority).map_err(|error| {
            AgentdError::Protocol(format!("validate Agent runtime profile: {error}"))
        })?;
        runtime_profile
            .validate_product_graph(&product_graph)
            .map_err(|error| {
                AgentdError::Protocol(format!("bind runtime profile to product graph: {error}"))
            })?;

        let federation_owner_layouts = snapshot
            .agents
            .into_values()
            .filter(|candidate| candidate.manifest.agent_id != identity.agent_id)
            .map(|candidate| candidate.layout)
            .collect::<Vec<_>>();
        let state = Arc::new(AgentdState::new(
            identity.clone(),
            registry,
            EVENT_CAPACITY,
        )?);
        let memory_service = AgentMemoryService::open(
            state.as_ref(),
            &identity,
            federation_owner_layouts,
            &authority,
            &runtime_authority,
        )
        .await?;
        let automation_service = AgentAutomationService::open(
            state.as_ref(),
            &identity,
            &authority,
            &runtime_authority,
        )
        .await?;
        runtime_profile
            .validate_composed_services(
                memory_service.is_available(),
                automation_service.is_available(),
            )
            .map_err(|error| {
                AgentdError::Protocol(format!("validate composed runtime services: {error}"))
            })?;
        let runtime_instance = RuntimeInstanceGraph::agent_composed(
            &authority,
            &product_graph,
            memory_service.is_available(),
            automation_service.is_available(),
        )
        .map_err(|error| {
            AgentdError::Protocol(format!("validate Agent runtime instance graph: {error}"))
        })?;

        Ok(Self {
            identity,
            state,
            memory_service,
            automation_service,
            authority,
            runtime_authority,
            product_graph,
            runtime_instance,
            writer_lock,
        })
    }

    pub(crate) fn into_parts(self) -> AgentRuntimeParts {
        AgentRuntimeParts {
            identity: self.identity,
            state: self.state,
            memory_service: self.memory_service,
            automation_service: self.automation_service,
            authority: self.authority,
            runtime_authority: self.runtime_authority,
            product_graph: self.product_graph,
            runtime_instance: self.runtime_instance,
            writer_lock: self.writer_lock,
        }
    }
}

pub(crate) fn authority_for_identity(
    identity: &AgentdIdentity,
) -> Result<AuthorityGrant, AgentdError> {
    #[cfg(feature = "qualification-cognitive-write")]
    let authority = AuthorityGrant::qualification_cognitive_write(
        identity.agent_id.clone(),
        identity.spawn_generation,
    );

    #[cfg(not(feature = "qualification-cognitive-write"))]
    let authority =
        AuthorityGrant::agent_local(identity.agent_id.clone(), identity.spawn_generation);

    authority.map_err(|error| AgentdError::Protocol(format!("build Agent authority: {error}")))
}

fn runtime_authority_context(
    record: &AgentRecord,
    identity: &AgentdIdentity,
    authority: &AuthorityGrant,
) -> Result<RuntimeAuthorityContext, AgentdError> {
    if record.lifecycle.lifecycle != AgentLifecycle::Starting
        || record.lifecycle.generation != identity.spawn_generation
    {
        return Err(AgentdError::GenerationFenced(format!(
            "runtime authority requires Starting generation {}, found {:?} generation {}",
            identity.spawn_generation,
            record.lifecycle.lifecycle,
            record.lifecycle.generation
        )));
    }
    let authority_epoch = record
        .release_state
        .generation
        .checked_add(1)
        .ok_or_else(|| AgentdError::Protocol("release authority epoch overflow".to_string()))?;
    let owner_epoch = record.lifecycle.generation;
    let fencing_token_sha256 = runtime_fencing_token(record, identity, authority);
    RuntimeAuthorityContext::new(
        identity.agent_id.clone(),
        authority_epoch,
        owner_epoch,
        identity.spawn_generation,
        fencing_token_sha256,
        authority.digest(),
    )
    .and_then(|context| {
        context.validate_grant(authority)?;
        Ok(context)
    })
    .map_err(|error| {
        AgentdError::Protocol(format!("build lifecycle-owned runtime authority: {error}"))
    })
}

fn runtime_fencing_token(
    record: &AgentRecord,
    identity: &AgentdIdentity,
    authority: &AuthorityGrant,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:agent-runtime-fence:v1");
    frame(&mut bytes, identity.agent_id.as_str().as_bytes());
    frame(&mut bytes, &record.lifecycle.schema_version.to_be_bytes());
    frame(&mut bytes, &record.lifecycle.generation.to_be_bytes());
    frame(
        &mut bytes,
        lifecycle_name(record.lifecycle.lifecycle).as_bytes(),
    );
    frame(&mut bytes, &record.release_state.schema_version.to_be_bytes());
    frame(&mut bytes, &record.release_state.generation.to_be_bytes());
    frame(
        &mut bytes,
        record
            .release_state
            .current
            .as_ref()
            .map(|release| release.as_str())
            .unwrap_or("")
            .as_bytes(),
    );
    frame(
        &mut bytes,
        record
            .release_state
            .previous
            .as_ref()
            .map(|release| release.as_str())
            .unwrap_or("")
            .as_bytes(),
    );
    frame(&mut bytes, authority.digest().as_str().as_bytes());
    frame(
        &mut bytes,
        &record.manifest.resources.max_concurrent_turns.to_be_bytes(),
    );
    frame(
        &mut bytes,
        &record.manifest.resources.max_tool_processes.to_be_bytes(),
    );
    frame(
        &mut bytes,
        &record.manifest.resources.turn_queue_capacity.to_be_bytes(),
    );
    Sha256Digest::for_bytes(&bytes)
}

fn lifecycle_name(lifecycle: AgentLifecycle) -> &'static str {
    match lifecycle {
        AgentLifecycle::Stopped => "stopped",
        AgentLifecycle::Starting => "starting",
        AgentLifecycle::Running => "running",
        AgentLifecycle::Draining => "draining",
        AgentLifecycle::Failed => "failed",
    }
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityAction;
    use codex_hepta_contracts::ProductComponentId;
    use codex_hepta_contracts::RuntimeAuthorityProfile;
    use codex_hepta_contracts::RuntimeServiceRequirement;
    use codex_hepta_contracts::RuntimeServiceState;
    use codex_hepta_fleet::AgentLifecycle;
    use codex_hepta_fleet::AgentManifest;
    use codex_hepta_fleet::FleetRegistry;
    use codex_hepta_fleet::ResourceBudget;
    use codex_hepta_fleet::WorkspaceBinding;
    use codex_hepta_paths::HeptaFleetRoot;

    use super::AgentRuntimeComposition;
    use crate::AgentdConfig;
    use crate::runtime_profile::RuntimeProfileContract;
    use crate::runtime_profile::RuntimeServiceId;
    use crate::runtime_profile::RuntimeServiceRequirement as ProfileRequirement;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    #[tokio::test]
    async fn real_product_graph_opens_agent_private_services_without_external_authority() {
        let temp = tempfile::tempdir().expect("temporary root");
        let root = temp
            .path()
            .canonicalize()
            .expect("canonical temporary root");
        let fleet_path = root.join("fleet");
        let fleet_root = HeptaFleetRoot::parse(fleet_path.clone()).expect("valid fleet root");
        let registry = FleetRegistry::initialize(fleet_root.clone()).expect("initialize registry");
        let workspace = root.join("workspace");
        std::fs::create_dir(&workspace).expect("create workspace");
        let agent_id = AgentId::parse(AGENT_ID).expect("valid agent id");
        let binding =
            WorkspaceBinding::new(workspace.clone(), &fleet_root).expect("bind workspace");
        let manifest =
            AgentManifest::new(agent_id.clone(), binding, ResourceBudget::local_default())
                .expect("valid manifest");
        let record = registry.register(manifest).expect("register agent");
        registry
            .compare_and_transition(&agent_id, 0, AgentLifecycle::Starting)
            .expect("start generation");
        let config = AgentdConfig::load(
            fleet_path,
            agent_id,
            1,
            record.layout.home_root().to_path_buf(),
            record.layout.run_root().to_path_buf(),
            record.layout.home_root().to_path_buf(),
            workspace,
        )
        .expect("load Agentd configuration");

        let composition = AgentRuntimeComposition::open(config)
            .await
            .expect("open real Agent product composition");
        let parts = composition.into_parts();
        let runtime_profile = RuntimeProfileContract::for_authority(&parts.authority)
            .expect("runtime profile must bind authority");
        assert!(parts.memory_service.is_available());
        assert!(parts.automation_service.is_available());
        assert_eq!(
            parts.memory_service.cognitive_write_enabled(),
            parts.authority.allows(AuthorityAction::WriteCognitiveState)
        );
        assert!(parts.product_graph.validate().is_ok());
        assert_eq!(runtime_profile.profile(), RuntimeAuthorityProfile::AgentLocal);
        assert_eq!(
            runtime_profile
                .policy(RuntimeServiceId::MemoryRuntime)
                .expect("Memory runtime policy")
                .requirement,
            ProfileRequirement::Optional
        );
        assert!(parts.authority.is_product_closed());
        assert!(!parts.authority.allows(AuthorityAction::ExternalEffect));
        assert!(!parts.authority.allows(AuthorityAction::PromoteRelease));
        assert_eq!(parts.runtime_authority.authority_epoch(), 1);
        assert_eq!(parts.runtime_authority.owner_epoch(), 1);
        assert_eq!(parts.runtime_authority.generation(), 1);
        assert!(!parts.runtime_instance.ready());
        assert_eq!(
            parts
                .runtime_instance
                .component_status(ProductComponentId::AppServer)
                .map(|status| (status.requirement, status.state)),
            Some((
                RuntimeServiceRequirement::Required,
                RuntimeServiceState::Starting,
            ))
        );
        assert!(parts.state.automation_is_available().expect("state lock"));
    }
}
