use std::fs::File;
use std::future::Future;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_automation::AutomationError;
use codex_hepta_automation::AutomationStore;
use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::ProductGraph;
use codex_hepta_memory::CognitiveRuntime;

use crate::AgentdConfig;
use crate::AgentdError;
use crate::AgentdIdentity;
use crate::AgentdState;

const EVENT_CAPACITY: usize = 128;

pub(crate) struct AgentRuntimeComposition {
    identity: AgentdIdentity,
    state: Arc<AgentdState>,
    cognitive_runtime: CognitiveRuntime,
    automation_store: Option<AutomationStore>,
    authority: AuthorityGrant,
    product_graph: ProductGraph,
    writer_lock: File,
}

pub(crate) struct AgentRuntimeParts {
    pub(crate) identity: AgentdIdentity,
    pub(crate) state: Arc<AgentdState>,
    pub(crate) cognitive_runtime: CognitiveRuntime,
    pub(crate) automation_store: Option<AutomationStore>,
    pub(crate) authority: AuthorityGrant,
    pub(crate) product_graph: ProductGraph,
    pub(crate) writer_lock: File,
}

impl AgentRuntimeComposition {
    pub(crate) async fn open(config: AgentdConfig) -> Result<Self, AgentdError> {
        let (identity, registry, writer_lock) = config.into_parts();
        let authority = authority_for_identity(&identity)?;
        authority
            .validate_binding(&identity.agent_id, identity.spawn_generation)
            .map_err(|error| {
                AgentdError::Protocol(format!("validate Agent authority binding: {error}"))
            })?;
        let product_graph = ProductGraph::agent_local(&authority).map_err(|error| {
            AgentdError::Protocol(format!("validate Agent product graph: {error}"))
        })?;

        let federation_owner_layouts = registry
            .load()?
            .agents
            .into_values()
            .filter(|record| record.manifest.agent_id != identity.agent_id)
            .map(|record| record.layout)
            .collect::<Vec<_>>();
        let state = Arc::new(AgentdState::new(
            identity.clone(),
            registry,
            EVENT_CAPACITY,
        )?);

        state.refresh_generation()?;
        let mut cognitive_runtime =
            CognitiveRuntime::open_agent_owned(&identity.layout, &authority)
                .await
                .map_err(|error| {
                    AgentdError::Protocol(format!("open Agent memory runtime: {error}"))
                })?;
        state.refresh_generation()?;
        if authority.allows(AuthorityAction::WriteCognitiveState)
            && !cognitive_runtime
                .cognitive_write_store_available(&authority)
                .map_err(|error| {
                    AgentdError::Protocol(format!("bind cognitive write capability: {error}"))
                })?
        {
            return Err(AgentdError::QualificationCognitiveRuntimeUnavailable);
        }
        if let Some(store) = cognitive_runtime.available_store() {
            state.attach_cognitive_store(Arc::clone(store))?;
        }

        if cognitive_runtime.available_store().is_some() && !federation_owner_layouts.is_empty() {
            state.refresh_generation()?;
            cognitive_runtime = cognitive_runtime
                .with_discovered_federation(
                    identity.agent_id.clone(),
                    federation_owner_layouts,
                    now_unix_seconds()?,
                    &authority,
                )
                .await
                .map_err(|error| {
                    AgentdError::Protocol(format!("discover memory federation: {error}"))
                })?;
            state.refresh_generation()?;
        }

        let automation_layout = identity.layout.clone();
        let automation_store = open_automation_store_after_generation_fence(&state, || async move {
            AutomationStore::open(&automation_layout).await
        })
        .await?;
        if let Some(store) = automation_store.as_ref() {
            state.attach_automation_store(store.clone())?;
        }

        Ok(Self {
            identity,
            state,
            cognitive_runtime,
            automation_store,
            authority,
            product_graph,
            writer_lock,
        })
    }

    pub(crate) fn into_parts(self) -> AgentRuntimeParts {
        AgentRuntimeParts {
            identity: self.identity,
            state: self.state,
            cognitive_runtime: self.cognitive_runtime,
            automation_store: self.automation_store,
            authority: self.authority,
            product_graph: self.product_graph,
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

async fn open_automation_store_after_generation_fence<Open, OpenFuture>(
    state: &AgentdState,
    open: Open,
) -> Result<Option<AutomationStore>, AgentdError>
where
    Open: FnOnce() -> OpenFuture,
    OpenFuture: Future<Output = Result<AutomationStore, AutomationError>>,
{
    state.refresh_generation()?;
    let opened = open().await;
    state.refresh_generation()?;
    match opened {
        Ok(store) => Ok(Some(store)),
        Err(AutomationError::Unavailable | AutomationError::Corrupt) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn now_unix_seconds() -> Result<i64, AgentdError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| AgentdError::Protocol(error.to_string()))?
        .as_secs();
    i64::try_from(now).map_err(|_| AgentdError::Protocol("system clock overflow".to_string()))
}

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::AuthorityAction;
    use codex_hepta_fleet::AgentLifecycle;
    use codex_hepta_fleet::AgentManifest;
    use codex_hepta_fleet::FleetRegistry;
    use codex_hepta_fleet::ResourceBudget;
    use codex_hepta_fleet::WorkspaceBinding;
    use codex_hepta_paths::HeptaFleetRoot;

    use super::AgentRuntimeComposition;
    use crate::AgentdConfig;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    #[tokio::test]
    async fn real_product_graph_opens_agent_private_stores_without_external_authority() {
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
        let binding = WorkspaceBinding::new(workspace.clone(), &fleet_root).expect("bind workspace");
        let manifest = AgentManifest::new(
            agent_id.clone(),
            binding,
            ResourceBudget::local_default(),
        )
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
        assert!(parts.cognitive_runtime.available_store().is_some());
        assert!(parts.automation_store.is_some());
        assert!(parts.product_graph.validate().is_ok());
        assert!(parts.authority.is_product_closed());
        assert!(!parts.authority.allows(AuthorityAction::ExternalEffect));
        assert!(!parts.authority.allows(AuthorityAction::PromoteRelease));
        assert!(parts.state.automation_is_available().expect("state lock"));
    }
}
