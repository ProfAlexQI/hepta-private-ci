use std::ffi::OsString;

use codex_hepta_contracts::AgentId;
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
use codex_hepta_supervisor::AgentCommand;
use codex_hepta_supervisor::RuntimeBootstrapIssuer;
use codex_hepta_supervisor::SpawnSpec;
use ed25519_dalek::SigningKey;

use super::RuntimeBootstrapAdmission;
use super::consume_runtime_bootstrap;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const SOURCE_COMMIT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SOURCE_TREE: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

struct Fixture {
    _temp: tempfile::TempDir,
    registry: FleetRegistry,
    record: codex_hepta_fleet::AgentRecord,
    program: std::path::PathBuf,
    issuer: RuntimeBootstrapIssuer,
    spec: SpawnSpec,
}

fn fixture(provenance: bool) -> Fixture {
    let temp = tempfile::tempdir().expect("temporary root");
    let root = temp.path().canonicalize().expect("canonical temp root");
    let fleet_root = HeptaFleetRoot::parse(root.join("fleet")).expect("fleet root");
    let registry = FleetRegistry::initialize(fleet_root.clone()).expect("registry");
    let workspace = root.join("workspace");
    std::fs::create_dir(&workspace).expect("workspace");
    let agent_id = AgentId::parse(AGENT_ID).expect("agent id");
    let initial = registry
        .register(
            AgentManifest::new(
                agent_id.clone(),
                WorkspaceBinding::new(workspace.clone(), &fleet_root)
                    .expect("workspace binding"),
                ResourceBudget::local_default(),
            )
            .expect("manifest"),
        )
        .expect("register");
    let source = root.join("hepta-agentd-source");
    std::fs::write(&source, b"test-agentd-binary").expect("source binary");
    let release_id = ReleaseId::parse("release-v1").expect("release id");
    let release = registry
        .install_release(release_id.clone(), &source, Vec::new())
        .expect("install release");
    registry
        .allow_release(&agent_id, &release_id)
        .expect("allow release");
    if provenance {
        registry
            .install_runtime_release_provenance(
                &agent_id,
                &release_id,
                SOURCE_COMMIT,
                SOURCE_TREE,
            )
            .expect("provenance");
    }
    let starting = registry
        .compare_and_transition(&agent_id, 0, AgentLifecycle::Starting)
        .expect("starting");
    let record = registry
        .load()
        .expect("snapshot")
        .agent(&agent_id)
        .cloned()
        .expect("record");
    let issuer = RuntimeBootstrapIssuer::new(
        "runtime-bootstrap-key",
        3,
        SigningKey::from_bytes(&[7_u8; 32]),
        120,
    )
    .expect("issuer");
    registry
        .install_runtime_bootstrap_trust_root(issuer.trust_root())
        .expect("trust root");
    let spec = SpawnSpec {
        agent_id,
        generation: starting.generation,
        fleet_root: registry.layout().fleet_root().as_path().to_path_buf(),
        workspace,
        home_root: initial.layout.home_root().to_path_buf(),
        run_root: initial.layout.run_root().to_path_buf(),
        control_socket: initial.layout.agentd_control_socket().to_path_buf(),
        logs_root: initial.layout.logs_root().to_path_buf(),
        command: AgentCommand::new(release.program.clone(), Vec::<OsString>::new())
            .expect("command"),
    };
    Fixture {
        _temp: temp,
        registry,
        record,
        program: release.program,
        issuer,
        spec,
    }
}

fn document_path(fixture: &Fixture, generation: u64) -> std::path::PathBuf {
    fixture
        .record
        .layout
        .run_root()
        .join(runtime_bootstrap_document_file_name(generation))
}

fn reservation_path(fixture: &Fixture, generation: u64) -> std::path::PathBuf {
    fixture
        .record
        .layout
        .run_root()
        .join(runtime_bootstrap_reservation_file_name(generation))
}

fn claim_path(fixture: &Fixture, generation: u64) -> std::path::PathBuf {
    fixture
        .record
        .layout
        .run_root()
        .join(runtime_bootstrap_claim_file_name(generation))
}

#[test]
fn valid_bootstrap_is_claimed_once_before_service_start() {
    let fixture = fixture(true);
    fixture
        .issuer
        .prepare_spawn(&fixture.registry, &fixture.spec, 100)
        .expect("prepare bootstrap");
    let admission = consume_runtime_bootstrap(
        &fixture.registry,
        &fixture.record,
        &fixture.program,
        150,
    )
    .expect("consume bootstrap");
    assert!(matches!(
        admission,
        RuntimeBootstrapAdmission::Verified {
            release_id,
            signer_epoch: 3,
            ..
        } if release_id == "release-v1"
    ));
    let generation = fixture.record.lifecycle.generation;
    assert!(claim_path(&fixture, generation).is_file());
    assert!(!document_path(&fixture, generation).exists());
    assert!(!reservation_path(&fixture, generation).exists());
    assert!(consume_runtime_bootstrap(
        &fixture.registry,
        &fixture.record,
        &fixture.program,
        151
    )
    .is_err());
}

#[test]
fn provenance_bound_release_without_handoff_fails_closed() {
    let fixture = fixture(true);
    assert!(consume_runtime_bootstrap(
        &fixture.registry,
        &fixture.record,
        &fixture.program,
        150
    )
    .is_err());
}

#[test]
fn unprovenanced_legacy_release_remains_closed_world_without_handoff() {
    let fixture = fixture(false);
    assert_eq!(
        consume_runtime_bootstrap(
            &fixture.registry,
            &fixture.record,
            &fixture.program,
            150
        )
        .expect("legacy admission"),
        RuntimeBootstrapAdmission::LocalClosedWorld
    );
}

#[test]
fn expired_or_tampered_handoff_never_creates_a_claim() {
    for tamper in [false, true] {
        let fixture = fixture(true);
        fixture
            .issuer
            .prepare_spawn(&fixture.registry, &fixture.spec, 100)
            .expect("prepare bootstrap");
        let generation = fixture.record.lifecycle.generation;
        let document_path = document_path(&fixture, generation);
        if tamper {
            make_writable(&document_path);
            let mut bytes = std::fs::read(&document_path).expect("read document");
            let index = bytes
                .iter()
                .position(|byte| *byte == b'a')
                .expect("document contains source oid");
            bytes[index] = b'c';
            std::fs::write(&document_path, bytes).expect("tamper document");
            make_owner_read_only(&document_path);
        }
        let observed = if tamper { 150 } else { 220 };
        assert!(consume_runtime_bootstrap(
            &fixture.registry,
            &fixture.record,
            &fixture.program,
            observed
        )
        .is_err());
        assert!(!claim_path(&fixture, generation).exists());
    }
}

#[test]
fn published_handoff_with_wrong_mode_is_recovery_required_before_claim() {
    let fixture = fixture(true);
    fixture
        .issuer
        .prepare_spawn(&fixture.registry, &fixture.spec, 100)
        .expect("published handoff");
    let generation = fixture.record.lifecycle.generation;
    make_writable(&document_path(&fixture, generation));
    assert!(consume_runtime_bootstrap(
        &fixture.registry,
        &fixture.record,
        &fixture.program,
        150
    )
    .is_err());
    assert!(!claim_path(&fixture, generation).exists());
}

#[test]
fn partial_reservation_without_document_is_recovery_required() {
    let fixture = fixture(true);
    fixture
        .issuer
        .prepare_spawn(&fixture.registry, &fixture.spec, 100)
        .expect("published handoff");
    let generation = fixture.record.lifecycle.generation;
    std::fs::remove_file(document_path(&fixture, generation)).expect("remove document");
    assert!(consume_runtime_bootstrap(
        &fixture.registry,
        &fixture.record,
        &fixture.program,
        150
    )
    .is_err());
    assert!(reservation_path(&fixture, generation).exists());
    assert!(!claim_path(&fixture, generation).exists());
}

#[cfg(unix)]
#[test]
fn symlink_handoff_is_rejected_before_claim() {
    use std::os::unix::fs::symlink;

    let fixture = fixture(true);
    fixture
        .issuer
        .prepare_spawn(&fixture.registry, &fixture.spec, 100)
        .expect("published handoff");
    let generation = fixture.record.lifecycle.generation;
    let document = document_path(&fixture, generation);
    let target = fixture
        .record
        .layout
        .run_root()
        .join("runtime-bootstrap-symlink-target");
    std::fs::rename(&document, &target).expect("retain document target");
    symlink(&target, &document).expect("install symlink");
    assert!(consume_runtime_bootstrap(
        &fixture.registry,
        &fixture.record,
        &fixture.program,
        150
    )
    .is_err());
    assert!(!claim_path(&fixture, generation).exists());
}

#[cfg(unix)]
#[test]
fn hardlink_handoff_is_rejected_before_claim() {
    let fixture = fixture(true);
    fixture
        .issuer
        .prepare_spawn(&fixture.registry, &fixture.spec, 100)
        .expect("published handoff");
    let generation = fixture.record.lifecycle.generation;
    let document = document_path(&fixture, generation);
    let alias = fixture
        .record
        .layout
        .run_root()
        .join("runtime-bootstrap-hardlink-alias");
    std::fs::hard_link(&document, alias).expect("install hardlink");
    assert!(consume_runtime_bootstrap(
        &fixture.registry,
        &fixture.record,
        &fixture.program,
        150
    )
    .is_err());
    assert!(!claim_path(&fixture, generation).exists());
}

#[test]
fn claimed_handoff_crash_is_recovery_required_and_never_replayed() {
    let fixture = fixture(true);
    fixture
        .issuer
        .prepare_spawn(&fixture.registry, &fixture.spec, 100)
        .expect("published handoff");
    let generation = fixture.record.lifecycle.generation;
    std::fs::hard_link(
        reservation_path(&fixture, generation),
        claim_path(&fixture, generation),
    )
    .expect("simulate durable claim before cleanup");
    assert!(consume_runtime_bootstrap(
        &fixture.registry,
        &fixture.record,
        &fixture.program,
        150
    )
    .is_err());
    assert!(document_path(&fixture, generation).exists());
    assert!(reservation_path(&fixture, generation).exists());
    assert!(claim_path(&fixture, generation).exists());
}

#[test]
fn retained_old_claim_does_not_authorize_but_does_not_block_a_fresh_generation() {
    let fixture = fixture(true);
    fixture
        .issuer
        .prepare_spawn(&fixture.registry, &fixture.spec, 100)
        .expect("first generation handoff");
    consume_runtime_bootstrap(
        &fixture.registry,
        &fixture.record,
        &fixture.program,
        150,
    )
    .expect("first generation claim");
    let old_generation = fixture.record.lifecycle.generation;

    let failed = fixture
        .registry
        .compare_and_transition(
            &fixture.spec.agent_id,
            old_generation,
            AgentLifecycle::Failed,
        )
        .expect("fence old generation");
    let starting = fixture
        .registry
        .compare_and_transition(
            &fixture.spec.agent_id,
            failed.generation,
            AgentLifecycle::Starting,
        )
        .expect("fresh generation");
    let mut fresh_spec = fixture.spec.clone();
    fresh_spec.generation = starting.generation;
    let fresh_record = fixture
        .registry
        .load()
        .expect("fresh snapshot")
        .agent(&fixture.spec.agent_id)
        .cloned()
        .expect("fresh record");
    fixture
        .issuer
        .prepare_spawn(&fixture.registry, &fresh_spec, 200)
        .expect("fresh generation handoff");
    let admission = consume_runtime_bootstrap(
        &fixture.registry,
        &fresh_record,
        &fixture.program,
        250,
    )
    .expect("fresh generation claim");
    assert!(admission.is_verified());
    assert!(claim_path(&fixture, old_generation).exists());
    assert!(claim_path(&fixture, starting.generation).exists());
}

#[cfg(unix)]
fn make_writable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
        .expect("writable mode");
}

#[cfg(not(unix))]
fn make_writable(path: &std::path::Path) {
    let mut permissions = std::fs::metadata(path).expect("metadata").permissions();
    permissions.set_readonly(false);
    std::fs::set_permissions(path, permissions).expect("writable mode");
}

#[cfg(unix)]
fn make_owner_read_only(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o400))
        .expect("read-only mode");
}

#[cfg(not(unix))]
fn make_owner_read_only(path: &std::path::Path) {
    let mut permissions = std::fs::metadata(path).expect("metadata").permissions();
    permissions.set_readonly(true);
    std::fs::set_permissions(path, permissions).expect("read-only mode");
}
