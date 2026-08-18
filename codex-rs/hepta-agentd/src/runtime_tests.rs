use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::AgentManifest;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::ResourceBudget;
use codex_hepta_fleet::WorkspaceBinding;
use codex_hepta_memory::CognitiveRuntime;
use codex_hepta_memory::CognitiveStoreError;
use codex_hepta_paths::HeptaFleetRoot;

use super::AgentdIdentity;
use super::AgentdState;
use super::EVENT_CAPACITY;
use super::open_cognitive_runtime_after_generation_fence;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

struct RuntimeFixture {
    _temp: tempfile::TempDir,
    state: Arc<AgentdState>,
    registry: FleetRegistry,
    identity: AgentdIdentity,
}

fn runtime_fixture() -> RuntimeFixture {
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
    let workspace = workspace.canonicalize().expect("canonical workspace");
    let agent_id = AgentId::parse(AGENT_ID).expect("valid agent id");
    let binding = WorkspaceBinding::new(&workspace, &fleet_root).expect("workspace binding");
    let manifest = AgentManifest::new(agent_id.clone(), binding, ResourceBudget::local_default())
        .expect("agent manifest");
    let record = registry.register(manifest).expect("register agent");
    registry
        .compare_and_transition(&agent_id, 0, AgentLifecycle::Starting)
        .expect("start generation");
    let identity = AgentdIdentity {
        agent_id,
        layout: record.layout.clone(),
        spawn_generation: 1,
        fleet_root: fleet_path,
        workspace,
        home_root: record.layout.home_root().to_path_buf(),
        run_root: record.layout.run_root().to_path_buf(),
        control_socket: record.layout.agentd_control_socket().to_path_buf(),
        app_server_socket: record.layout.app_server_socket().to_path_buf(),
    };
    let state = Arc::new(
        AgentdState::new(identity.clone(), registry.clone(), EVENT_CAPACITY).expect("agent state"),
    );
    RuntimeFixture {
        _temp: temp,
        state,
        registry,
        identity,
    }
}

#[tokio::test]
async fn unavailable_cognitive_store_degrades_without_leaking_open_error() {
    let fixture = runtime_fixture();
    let runtime = open_cognitive_runtime_after_generation_fence(&fixture.state, || async {
        Err(CognitiveStoreError::Unavailable(
            "/private/raw/cognitive.sqlite: secret detail".to_string(),
        ))
    })
    .await
    .expect("store outage must not block agent execution");

    let CognitiveRuntime::Unavailable(reason) = runtime else {
        panic!("store outage must remain distinguishable from absence");
    };
    assert_eq!(reason.code(), "storage_unavailable");
    assert!(!format!("{reason:?}").contains("/private/raw"));
}

#[tokio::test]
async fn stale_generation_is_fenced_before_cognitive_store_open() {
    let fixture = runtime_fixture();
    let agent_id = &fixture.identity.agent_id;
    fixture
        .registry
        .compare_and_transition(agent_id, 1, AgentLifecycle::Running)
        .expect("running");
    fixture
        .registry
        .compare_and_transition(agent_id, 2, AgentLifecycle::Draining)
        .expect("draining");
    fixture
        .registry
        .compare_and_transition(agent_id, 3, AgentLifecycle::Stopped)
        .expect("stopped");
    fixture
        .registry
        .compare_and_transition(agent_id, 4, AgentLifecycle::Starting)
        .expect("new starting generation");
    let opened = Arc::new(AtomicBool::new(false));
    let opened_by_call = Arc::clone(&opened);
    let result =
        open_cognitive_runtime_after_generation_fence(&fixture.state, move || async move {
            opened_by_call.store(true, Ordering::SeqCst);
            Err(CognitiveStoreError::Unavailable("must not run".to_string()))
        })
        .await;

    assert!(matches!(
        result,
        Err(crate::AgentdError::GenerationFenced(_))
    ));
    assert!(!opened.load(Ordering::SeqCst));
    assert_eq!(
        std::fs::read_dir(fixture.identity.layout.cognitive_root())
            .expect("cognitive root")
            .count(),
        0,
        "a stale generation must not touch the cognitive database"
    );
}

#[tokio::test]
async fn generation_change_during_open_is_fenced_before_serving() {
    let fixture = runtime_fixture();
    let registry = fixture.registry.clone();
    let agent_id = fixture.identity.agent_id.clone();
    let opened = Arc::new(AtomicBool::new(false));
    let opened_by_call = Arc::clone(&opened);
    let result =
        open_cognitive_runtime_after_generation_fence(&fixture.state, move || async move {
            opened_by_call.store(true, Ordering::SeqCst);
            registry
                .compare_and_transition(&agent_id, 1, AgentLifecycle::Failed)
                .expect("concurrent generation change");
            Err(CognitiveStoreError::Unavailable(
                "simulated outage".to_string(),
            ))
        })
        .await;

    assert!(opened.load(Ordering::SeqCst));
    assert!(matches!(
        result,
        Err(crate::AgentdError::GenerationFenced(_))
    ));
}
