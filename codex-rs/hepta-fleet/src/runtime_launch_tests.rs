use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::AuthorityGrant;

use crate::AgentLifecycle;
use crate::AgentManifest;
use crate::FleetRegistry;
use crate::ReleaseId;
use crate::ResourceBudget;
use crate::RuntimeLaunchBinding;
use crate::RuntimeLaunchBindingError;
use crate::WorkspaceBinding;
use codex_hepta_paths::HeptaFleetRoot;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

fn starting_record() -> (
    tempfile::TempDir,
    FleetRegistry,
    codex_hepta_contracts::AgentId,
) {
    let temp = tempfile::tempdir().expect("temporary root");
    let root = temp.path().canonicalize().expect("canonical temp root");
    let fleet_path = root.join("fleet");
    let fleet_root = HeptaFleetRoot::parse(fleet_path).expect("fleet root");
    let registry = FleetRegistry::initialize(fleet_root.clone()).expect("registry");
    let workspace = root.join("workspace");
    std::fs::create_dir(&workspace).expect("workspace");
    let agent_id = AgentId::parse(AGENT_ID).expect("agent id");
    let manifest = AgentManifest::new(
        agent_id.clone(),
        WorkspaceBinding::new(workspace, &fleet_root).expect("workspace binding"),
        ResourceBudget::local_default(),
    )
    .expect("manifest");
    registry.register(manifest).expect("register");
    registry
        .compare_and_transition(&agent_id, 0, AgentLifecycle::Starting)
        .expect("starting");
    (temp, registry, agent_id)
}

#[test]
fn starting_launch_binds_target_release_and_closed_authority() {
    let (_temp, registry, agent_id) = starting_record();
    let record = registry
        .load()
        .expect("snapshot")
        .agent(&agent_id)
        .cloned()
        .expect("record");
    let authority = AuthorityGrant::agent_local(agent_id.clone(), 1).expect("authority");
    let binding = RuntimeLaunchBinding::for_starting(
        &record,
        ReleaseId::parse("release-v1").expect("release"),
        &authority,
    )
    .expect("launch binding");
    assert_eq!(binding.release_id().as_str(), "release-v1");
    assert_eq!(binding.runtime_authority().authority_epoch(), 1);
    assert_eq!(binding.runtime_authority().owner_epoch(), 1);
    assert_eq!(binding.runtime_authority().generation(), 1);
    assert_eq!(
        binding.runtime_authority().authority_grant_sha256(),
        &authority.digest()
    );
    assert!(!authority.allows(AuthorityAction::ExternalEffect));
}

#[test]
fn release_and_resource_drift_change_the_fence() {
    let (_temp, registry, agent_id) = starting_record();
    let record = registry
        .load()
        .expect("snapshot")
        .agent(&agent_id)
        .cloned()
        .expect("record");
    let authority = AuthorityGrant::agent_local(agent_id.clone(), 1).expect("authority");
    let first = RuntimeLaunchBinding::for_starting(
        &record,
        ReleaseId::parse("release-v1").expect("release"),
        &authority,
    )
    .expect("first binding");
    let second = RuntimeLaunchBinding::for_starting(
        &record,
        ReleaseId::parse("release-v2").expect("release"),
        &authority,
    )
    .expect("second binding");
    assert_ne!(
        first.runtime_authority().fencing_token_sha256(),
        second.runtime_authority().fencing_token_sha256()
    );
    assert_ne!(first.digest(), second.digest());
}

#[test]
fn nonstarting_and_wrong_generation_fail_closed() {
    let (_temp, registry, agent_id) = starting_record();
    let record = registry
        .load()
        .expect("snapshot")
        .agent(&agent_id)
        .cloned()
        .expect("record");
    let wrong = AuthorityGrant::agent_local(agent_id.clone(), 2).expect("authority");
    assert!(matches!(
        RuntimeLaunchBinding::for_starting(
            &record,
            ReleaseId::parse("release-v1").expect("release"),
            &wrong
        ),
        Err(RuntimeLaunchBindingError::GenerationMismatch)
    ));
}
