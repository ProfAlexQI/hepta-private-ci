use std::ffi::OsString;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::RuntimeBootstrapDocument;
use codex_hepta_contracts::runtime_bootstrap_claim_file_name;
use codex_hepta_contracts::runtime_bootstrap_document_file_name;
use codex_hepta_contracts::runtime_bootstrap_reservation_file_name;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::AgentManifest;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::ReleaseId;
use codex_hepta_fleet::ResourceBudget;
use codex_hepta_fleet::WorkspaceBinding;
use codex_hepta_paths::HeptaFleetRoot;
use ed25519_dalek::SigningKey;

use crate::AgentCommand;
use crate::RuntimeBootstrapIssuer;
use crate::SpawnSpec;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const SOURCE_COMMIT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SOURCE_TREE: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

struct Fixture {
    _temp: tempfile::TempDir,
    registry: FleetRegistry,
    spec: SpawnSpec,
    issuer: RuntimeBootstrapIssuer,
}

fn fixture(install_provenance: bool, install_trust: bool) -> Fixture {
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
        WorkspaceBinding::new(workspace.clone(), &fleet_root).expect("workspace binding"),
        ResourceBudget::local_default(),
    )
    .expect("manifest");
    let record = registry.register(manifest).expect("register");

    let source = root.join("hepta-agentd-source");
    std::fs::write(&source, b"test-agentd-binary").expect("source binary");
    let release_id = ReleaseId::parse("release-v1").expect("release id");
    let release = registry
        .install_release(release_id.clone(), &source, Vec::new())
        .expect("install release");
    registry
        .allow_release(&agent_id, &release_id)
        .expect("allow release");
    if install_provenance {
        registry
            .install_runtime_release_provenance(&agent_id, &release_id, SOURCE_COMMIT, SOURCE_TREE)
            .expect("install provenance");
    }
    let starting = registry
        .compare_and_transition(&agent_id, 0, AgentLifecycle::Starting)
        .expect("starting");
    let issuer = RuntimeBootstrapIssuer::new(
        "runtime-bootstrap-key",
        3,
        SigningKey::from_bytes(&[7_u8; 32]),
        120,
    )
    .expect("issuer");
    if install_trust {
        registry
            .install_runtime_bootstrap_trust_root(issuer.trust_root())
            .expect("install trust root");
    }
    let spec = SpawnSpec {
        agent_id,
        generation: starting.generation,
        fleet_root: registry.layout().fleet_root().as_path().to_path_buf(),
        workspace,
        home_root: record.layout.home_root().to_path_buf(),
        run_root: record.layout.run_root().to_path_buf(),
        control_socket: record.layout.agentd_control_socket().to_path_buf(),
        logs_root: record.layout.logs_root().to_path_buf(),
        command: AgentCommand::new(release.program, Vec::<OsString>::new()).expect("command"),
    };
    Fixture {
        _temp: temp,
        registry,
        spec,
        issuer,
    }
}

#[test]
fn issuer_publishes_reservation_then_signed_handoff_for_exact_generation() {
    let fixture = fixture(true, true);
    let document = fixture
        .issuer
        .prepare_spawn(&fixture.registry, &fixture.spec, 100)
        .expect("prepare bootstrap");
    let generation = fixture.spec.generation;
    let document_path = fixture
        .spec
        .run_root
        .join(runtime_bootstrap_document_file_name(generation));
    let reservation_path = fixture
        .spec
        .run_root
        .join(runtime_bootstrap_reservation_file_name(generation));
    let claim_path = fixture
        .spec
        .run_root
        .join(runtime_bootstrap_claim_file_name(generation));
    assert!(reservation_path.is_file());
    assert!(document_path.is_file());
    assert!(!claim_path.exists());
    assert_owner_bound_single_link_read_only(&fixture.spec.run_root, &reservation_path);
    assert_owner_bound_single_link_read_only(&fixture.spec.run_root, &document_path);
    let on_disk =
        RuntimeBootstrapDocument::decode(&std::fs::read(document_path).expect("read handoff"))
            .expect("decode handoff");
    assert_eq!(on_disk, document);
    assert_eq!(document.envelope.release_id(), "release-v1");
    assert_eq!(document.envelope.source_commit(), SOURCE_COMMIT);
    assert_eq!(document.envelope.source_tree(), SOURCE_TREE);
    assert_eq!(document.envelope.generation(), generation);
    assert!(
        fixture
            .issuer
            .prepare_spawn(&fixture.registry, &fixture.spec, 101)
            .is_err()
    );
}

#[test]
fn missing_provenance_or_trust_root_fails_before_handoff_publication() {
    for fixture in [fixture(false, true), fixture(true, false)] {
        assert!(
            fixture
                .issuer
                .prepare_spawn(&fixture.registry, &fixture.spec, 100)
                .is_err()
        );
        assert!(
            !fixture
                .spec
                .run_root
                .join(runtime_bootstrap_document_file_name(
                    fixture.spec.generation
                ))
                .exists()
        );
    }
}

#[test]
fn partial_reservation_is_retained_and_blocks_reinterpretation() {
    let fixture = fixture(true, true);
    let reservation_path = fixture
        .spec
        .run_root
        .join(runtime_bootstrap_reservation_file_name(
            fixture.spec.generation,
        ));
    std::fs::write(&reservation_path, b"partial\n").expect("write partial reservation");
    assert!(
        fixture
            .issuer
            .prepare_spawn(&fixture.registry, &fixture.spec, 100)
            .is_err()
    );
    assert_eq!(
        std::fs::read(&reservation_path).expect("read retained reservation"),
        b"partial\n"
    );
    assert!(
        !fixture
            .spec
            .run_root
            .join(runtime_bootstrap_document_file_name(
                fixture.spec.generation
            ))
            .exists()
    );
}

#[cfg(unix)]
#[test]
fn broken_symlink_state_blocks_reservation_and_document_publication() {
    use std::os::unix::fs::symlink;

    let fixture = fixture(true, true);
    let document_path = fixture
        .spec
        .run_root
        .join(runtime_bootstrap_document_file_name(
            fixture.spec.generation,
        ));
    symlink(
        fixture.spec.run_root.join("missing-bootstrap-target"),
        &document_path,
    )
    .expect("install broken symlink");
    assert!(
        fixture
            .issuer
            .prepare_spawn(&fixture.registry, &fixture.spec, 100)
            .is_err()
    );
    assert!(
        std::fs::symlink_metadata(&document_path)
            .expect("symlink retained")
            .file_type()
            .is_symlink()
    );
    assert!(
        !fixture
            .spec
            .run_root
            .join(runtime_bootstrap_reservation_file_name(
                fixture.spec.generation
            ))
            .exists()
    );
}

#[cfg(unix)]
fn assert_owner_bound_single_link_read_only(parent: &std::path::Path, path: &std::path::Path) {
    use std::os::unix::fs::MetadataExt as _;
    use std::os::unix::fs::PermissionsExt as _;

    let parent_metadata = std::fs::symlink_metadata(parent).expect("parent metadata");
    let metadata = std::fs::symlink_metadata(path).expect("object metadata");
    assert!(metadata.file_type().is_file());
    assert!(!metadata.file_type().is_symlink());
    assert_eq!(metadata.uid(), parent_metadata.uid());
    assert_eq!(metadata.nlink(), 1);
    assert_eq!(metadata.permissions().mode() & 0o777, 0o400);
}

#[cfg(not(unix))]
fn assert_owner_bound_single_link_read_only(_parent: &std::path::Path, path: &std::path::Path) {
    assert!(
        std::fs::metadata(path)
            .expect("object metadata")
            .permissions()
            .readonly()
    );
}
