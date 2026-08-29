use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::RuntimeBootstrapTrustRoot;
use codex_hepta_paths::HeptaFleetRoot;

use crate::AgentManifest;
use crate::FleetRegistry;
use crate::ReleaseId;
use crate::ResourceBudget;
use crate::WorkspaceBinding;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const SOURCE_COMMIT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SOURCE_TREE: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const PUBLIC_KEY: [u8; 32] = [
    0xea, 0x4a, 0x6c, 0x63, 0xe2, 0x9c, 0x52, 0x0a, 0xbe, 0xf5, 0x50, 0x7b, 0x13, 0x2e,
    0xc5, 0xf9, 0x95, 0x47, 0x76, 0xae, 0xbe, 0xbe, 0x7b, 0x92, 0x42, 0x1e, 0xea, 0x69,
    0x14, 0x46, 0xd2, 0x2c,
];

fn setup() -> (
    tempfile::TempDir,
    FleetRegistry,
    AgentId,
    ReleaseId,
    std::path::PathBuf,
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

    let source = root.join("hepta-agentd-source");
    std::fs::write(&source, b"test-agentd-binary").expect("source binary");
    let release_id = ReleaseId::parse("release-v1").expect("release id");
    let release = registry
        .install_release(release_id.clone(), &source, Vec::new())
        .expect("install release");
    registry
        .allow_release(&agent_id, &release_id)
        .expect("allow release");
    (temp, registry, agent_id, release_id, release.program)
}

#[test]
fn trust_root_is_immutable_and_selector_bound() {
    let (_temp, registry, _agent_id, _release_id, _program) = setup();
    let trust = RuntimeBootstrapTrustRoot::new("runtime-key", 7, PUBLIC_KEY)
        .expect("trust root");
    registry
        .install_runtime_bootstrap_trust_root(&trust)
        .expect("install trust root");
    registry
        .install_runtime_bootstrap_trust_root(&trust)
        .expect("idempotent identical trust root");
    let resolved = registry
        .resolve_runtime_bootstrap_trust_root("runtime-key", 7)
        .expect("resolve trust root");
    assert_eq!(resolved, trust);
    assert!(registry
        .resolve_runtime_bootstrap_trust_root("runtime-key", 8)
        .is_err());
}

#[test]
fn release_provenance_binds_source_manifest_and_binary() {
    let (_temp, registry, agent_id, release_id, program) = setup();
    let installed = registry
        .install_runtime_release_provenance(
            &agent_id,
            &release_id,
            SOURCE_COMMIT,
            SOURCE_TREE,
        )
        .expect("install provenance");
    let resolved = registry
        .resolve_runtime_release_for_program(&agent_id, &program)
        .expect("resolve by exact program");
    assert_eq!(resolved.release_id, release_id);
    assert_eq!(resolved.provenance, installed);
    assert_eq!(resolved.program, program.canonicalize().expect("canonical program"));
}

#[test]
fn unknown_program_and_noncanonical_source_identity_fail_closed() {
    let (temp, registry, agent_id, release_id, _program) = setup();
    assert!(registry
        .install_runtime_release_provenance(
            &agent_id,
            &release_id,
            SOURCE_COMMIT.to_uppercase(),
            SOURCE_TREE,
        )
        .is_err());
    let unknown = temp.path().join("unknown-agentd");
    std::fs::write(&unknown, b"unknown").expect("unknown program");
    assert!(registry
        .resolve_runtime_release_for_program(&agent_id, &unknown)
        .is_err());
}
