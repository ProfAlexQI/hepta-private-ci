use codex_hepta_contracts::AgentId;
use codex_hepta_paths::HeptaFleetRoot;

use crate::AgentManifest;
use crate::FleetRegistry;
use crate::ReleaseId;
use crate::ResourceBudget;
use crate::WorkspaceBinding;
use crate::allowed_runtime_release_for_program;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

#[test]
fn exact_allowed_program_resolves_without_requiring_provenance() {
    let temp = tempfile::tempdir().expect("temporary root");
    let root = temp.path().canonicalize().expect("canonical root");
    let fleet_root = HeptaFleetRoot::parse(root.join("fleet")).expect("fleet root");
    let registry = FleetRegistry::initialize(fleet_root.clone()).expect("registry");
    let workspace = root.join("workspace");
    std::fs::create_dir(&workspace).expect("workspace");
    let agent_id = AgentId::parse(AGENT_ID).expect("agent id");
    registry
        .register(
            AgentManifest::new(
                agent_id.clone(),
                WorkspaceBinding::new(workspace, &fleet_root).expect("workspace binding"),
                ResourceBudget::local_default(),
            )
            .expect("manifest"),
        )
        .expect("register");
    let source = root.join("source-agentd");
    std::fs::write(&source, b"agentd").expect("source");
    let release_id = ReleaseId::parse("release-v1").expect("release id");
    let installed = registry
        .install_release(release_id.clone(), &source, Vec::new())
        .expect("release");
    registry
        .allow_release(&agent_id, &release_id)
        .expect("allow");

    let resolved = allowed_runtime_release_for_program(
        &registry,
        &agent_id,
        &installed.program,
    )
    .expect("lookup")
    .expect("allowed release");
    assert_eq!(resolved.release_id, release_id);
    assert_eq!(resolved.program, installed.program.canonicalize().expect("canonical"));

    let unknown = root.join("unknown-agentd");
    std::fs::write(&unknown, b"unknown").expect("unknown");
    assert!(allowed_runtime_release_for_program(&registry, &agent_id, &unknown)
        .expect("lookup")
        .is_none());
}
