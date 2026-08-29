use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::RuntimeBootstrapTrustRoot;
use codex_hepta_paths::HeptaFleetRoot;

use crate::AgentManifest;
use crate::FleetRegistry;
use crate::ReleaseId;
use crate::ResourceBudget;
use crate::WorkspaceBinding;

use super::RELEASE_PROVENANCE_DIRECTORY;
use super::TRUST_ROOT_DIRECTORY;
use super::trust_root_file_name;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const SOURCE_COMMIT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SOURCE_TREE: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const PUBLIC_KEY: [u8; 32] = [
    0xea, 0x4a, 0x6c, 0x63, 0xe2, 0x9c, 0x52, 0x0a, 0xbe, 0xf5, 0x50, 0x7b, 0x13, 0x2e, 0xc5, 0xf9,
    0x95, 0x47, 0x76, 0xae, 0xbe, 0xbe, 0x7b, 0x92, 0x42, 0x1e, 0xea, 0x69, 0x14, 0x46, 0xd2, 0x2c,
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

fn trust_path(registry: &FleetRegistry) -> std::path::PathBuf {
    registry
        .layout()
        .state_root()
        .join(TRUST_ROOT_DIRECTORY)
        .join(trust_root_file_name("runtime-key", 7))
}

fn provenance_path(
    registry: &FleetRegistry,
    agent_id: &AgentId,
    release_id: &ReleaseId,
) -> std::path::PathBuf {
    registry
        .layout()
        .state_root()
        .join(RELEASE_PROVENANCE_DIRECTORY)
        .join(agent_id.as_str())
        .join(format!("{}.json", release_id.as_str()))
}

fn trust_root() -> RuntimeBootstrapTrustRoot {
    RuntimeBootstrapTrustRoot::new("runtime-key", 7, PUBLIC_KEY).expect("trust root")
}

#[test]
fn trust_root_is_immutable_selector_bound_and_physically_sealed() {
    let (_temp, registry, _agent_id, _release_id, _program) = setup();
    let trust = trust_root();
    registry
        .install_runtime_bootstrap_trust_root(&trust)
        .expect("install trust root");
    registry
        .install_runtime_bootstrap_trust_root(&trust)
        .expect("idempotent identical trust root");
    assert_registry_object(&trust_path(&registry));
    let resolved = registry
        .resolve_runtime_bootstrap_trust_root("runtime-key", 7)
        .expect("resolve trust root");
    assert_eq!(resolved, trust);
    assert!(
        registry
            .resolve_runtime_bootstrap_trust_root("runtime-key", 8)
            .is_err()
    );
}

#[test]
fn release_provenance_binds_source_manifest_binary_and_physical_identity() {
    let (_temp, registry, agent_id, release_id, program) = setup();
    let installed = registry
        .install_runtime_release_provenance(&agent_id, &release_id, SOURCE_COMMIT, SOURCE_TREE)
        .expect("install provenance");
    assert_registry_object(&provenance_path(&registry, &agent_id, &release_id));
    let resolved = registry
        .resolve_runtime_release_for_program(&agent_id, &program)
        .expect("resolve by exact program");
    assert_eq!(resolved.release_id, release_id);
    assert_eq!(resolved.provenance, installed);
    assert_eq!(
        resolved.program,
        program.canonicalize().expect("canonical program")
    );
}

#[test]
fn unknown_program_and_noncanonical_source_identity_fail_closed() {
    let (temp, registry, agent_id, release_id, _program) = setup();
    assert!(
        registry
            .install_runtime_release_provenance(
                &agent_id,
                &release_id,
                SOURCE_COMMIT.to_uppercase(),
                SOURCE_TREE,
            )
            .is_err()
    );
    let unknown = temp.path().join("unknown-agentd");
    std::fs::write(&unknown, b"unknown").expect("unknown program");
    assert!(
        registry
            .resolve_runtime_release_for_program(&agent_id, &unknown)
            .is_err()
    );
}

#[cfg(unix)]
#[test]
fn trust_root_wrong_mode_and_unsafe_hardlink_fail_closed() {
    use std::os::unix::fs::PermissionsExt as _;

    let (_temp, registry, _agent_id, _release_id, _program) = setup();
    registry
        .install_runtime_bootstrap_trust_root(&trust_root())
        .expect("install trust root");
    let path = trust_path(&registry);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).expect("wrong mode");
    assert!(
        registry
            .resolve_runtime_bootstrap_trust_root("runtime-key", 7)
            .is_err()
    );

    let (_temp, registry, _agent_id, _release_id, _program) = setup();
    registry
        .install_runtime_bootstrap_trust_root(&trust_root())
        .expect("install trust root");
    let path = trust_path(&registry);
    std::fs::hard_link(&path, path.with_extension("alias")).expect("hardlink alias");
    assert!(
        registry
            .resolve_runtime_bootstrap_trust_root("runtime-key", 7)
            .is_err()
    );
}

#[cfg(unix)]
#[test]
fn provenance_symlink_is_rejected_without_following_target() {
    use std::os::unix::fs::symlink;

    let (_temp, registry, agent_id, release_id, _program) = setup();
    registry
        .install_runtime_release_provenance(&agent_id, &release_id, SOURCE_COMMIT, SOURCE_TREE)
        .expect("install provenance");
    let path = provenance_path(&registry, &agent_id, &release_id);
    let target = path.with_extension("retained");
    std::fs::rename(&path, &target).expect("retain target");
    symlink(&target, &path).expect("install symlink");
    assert!(
        registry
            .resolve_runtime_release_provenance(&agent_id, &release_id)
            .is_err()
    );
}

#[cfg(unix)]
fn assert_registry_object(path: &std::path::Path) {
    use std::os::unix::fs::MetadataExt as _;
    use std::os::unix::fs::PermissionsExt as _;

    let parent = path.parent().expect("parent");
    let parent_metadata = std::fs::symlink_metadata(parent).expect("parent metadata");
    let metadata = std::fs::symlink_metadata(path).expect("registry object metadata");
    assert!(metadata.file_type().is_file());
    assert!(!metadata.file_type().is_symlink());
    assert_eq!(metadata.uid(), parent_metadata.uid());
    assert_eq!(metadata.nlink(), 1);
    assert_eq!(metadata.permissions().mode() & 0o777, 0o400);
}

#[cfg(not(unix))]
fn assert_registry_object(path: &std::path::Path) {
    assert!(
        std::fs::metadata(path)
            .expect("registry object metadata")
            .permissions()
            .readonly()
    );
}
