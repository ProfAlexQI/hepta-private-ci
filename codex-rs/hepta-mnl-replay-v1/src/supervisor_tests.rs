use std::cell::RefCell;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::rc::Rc;

use codex_hepta_mnl_trust_v1::ReplayPlatformScopeV1;
use serde::Serialize;
use sha2::Digest;
use static_assertions::assert_not_impl_any;
use static_assertions::const_assert;

use crate::PRODUCTION_WALL_CLOCK_SUPERVISOR_POLICY_AVAILABLE;
use crate::PreRunClockPublicationInspectionV1;
use crate::ReplayStoreAnchorV1;
use crate::ReplayStoreErrorV1;
use crate::ReplayStorePolicyV1;
use crate::clock::ClockSampleV1;
use crate::clock::sample_for_tests;
use crate::open_replay_store;
use crate::store::publish_material_for_tests;
use crate::supervisor::ClockBootObservationV1;
use crate::supervisor::ClockBootSourceV1;
use crate::supervisor::LinuxClockBootSourceV1;
use crate::supervisor::PreRunClockClaimBindingV1;
use crate::supervisor::inspect_sequence_with_source_and_publisher;
use crate::supervisor::require_production_wall_clock_supervisor_policy;

assert_not_impl_any!(PreRunClockPublicationInspectionV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);
const_assert!(!PRODUCTION_WALL_CLOCK_SUPERVISOR_POLICY_AVAILABLE);

#[test]
fn production_clock_policy_is_absent() {
    let error = require_production_wall_clock_supervisor_policy()
        .expect_err("production clock policy must remain absent");
    assert!(matches!(error, ReplayStoreErrorV1::Blocked(_)));
}

#[test]
fn exact_clock_publish_clock_order_is_observed_without_authority() {
    let fixture = StoreFixture::new("ordered").expect("ordered fixture");
    let store = fixture.open().expect("open ordered store");
    let slot = digest('a');
    let record = b"ordered replay claim";
    let full_binding = sha256(record);
    let boot = digest('b');
    let binding = binding(&store, &slot, &full_binding, &boot, 100, 102);
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut source = FakeSource::new(
        Rc::clone(&events),
        vec![
            observation(&boot, sample_for_tests(10, 1, 100, 1, 10, 2)),
            observation(&boot, sample_for_tests(11, 1, 101, 1, 11, 2)),
        ],
    );
    let publish_events = Rc::clone(&events);
    let inspected = inspect_sequence_with_source_and_publisher(&binding, &mut source, || {
        publish_events.borrow_mut().push("publish");
        publish_material_for_tests(&store, &slot, record, store.identity_sha256())
    })
    .expect("ordered clock-publication observation");

    assert_eq!(
        events.borrow().as_slice(),
        ["observe", "revalidate", "publish", "observe", "revalidate"]
    );
    assert_eq!(inspected.boot_id_sha256(), boot);
    assert_eq!(inspected.replay_slot_sha256(), slot);
    assert_eq!(inspected.full_binding_sha256(), full_binding);
    assert_eq!(inspected.retained_descriptor_count(), 3);
    assert!(inspected.exact_publication_read_back_observed());
    assert!(inspected.wall_clock_publication_sequence_observed());
    assert!(!inspected.launch_grant_available());
    assert!(!inspected.launch_performed());
    assert!(!inspected.crash_reboot_qualified());
    assert!(!inspected.authorizes_live());
}

#[test]
fn pre_publication_failures_do_not_publish() {
    for (label, platform, boot, sample) in [
        (
            "too-early",
            ReplayPlatformScopeV1::Nix,
            digest('b'),
            sample_for_tests(10, 1, 99, 999_999_999, 10, 2),
        ),
        (
            "wrong-boot",
            ReplayPlatformScopeV1::Nix,
            digest('c'),
            sample_for_tests(10, 1, 100, 0, 10, 2),
        ),
        (
            "wrong-scope",
            ReplayPlatformScopeV1::LinuxPhase1,
            digest('b'),
            sample_for_tests(10, 1, 100, 0, 10, 2),
        ),
    ] {
        let fixture = StoreFixture::new(label).expect("pre-failure fixture");
        let store = fixture.open().expect("open pre-failure store");
        let slot = digest('d');
        let record = b"must not publish";
        let binding = PreRunClockClaimBindingV1::for_tests(
            digest('b'),
            100,
            102,
            platform,
            slot.clone(),
            sha256(record),
            store.identity_sha256().to_string(),
        );
        let events = Rc::new(RefCell::new(Vec::new()));
        let mut source = FakeSource::new(Rc::clone(&events), vec![observation(&boot, sample)]);
        let published = Rc::new(RefCell::new(false));
        let publication_flag = Rc::clone(&published);
        assert!(
            inspect_sequence_with_source_and_publisher(&binding, &mut source, || {
                *publication_flag.borrow_mut() = true;
                publish_material_for_tests(&store, &slot, record, store.identity_sha256())
            })
            .is_err()
        );
        assert!(!*published.borrow());
        assert!(!fixture.final_path(&slot).exists());
    }
}

#[test]
fn every_post_publication_clock_or_boot_failure_is_uncertain_and_claim_survives() {
    let cases = [
        (
            "expiry",
            digest('b'),
            sample_for_tests(11, 0, 102, 0, 11, 1),
        ),
        (
            "realtime-rollback",
            digest('b'),
            sample_for_tests(11, 0, 100, 0, 11, 1),
        ),
        (
            "boottime-rollback",
            digest('b'),
            sample_for_tests(9, 0, 101, 0, 9, 1),
        ),
        (
            "boot-transplant",
            digest('c'),
            sample_for_tests(11, 0, 101, 0, 11, 1),
        ),
    ];
    for (label, post_boot, post_sample) in cases {
        let fixture = StoreFixture::new(label).expect("post-failure fixture");
        let store = fixture.open().expect("open post-failure store");
        let slot = digest('e');
        let record = format!("post failure {label}");
        let binding = binding(
            &store,
            &slot,
            &sha256(record.as_bytes()),
            &digest('b'),
            100,
            102,
        );
        let events = Rc::new(RefCell::new(Vec::new()));
        let mut source = FakeSource::new(
            Rc::clone(&events),
            vec![
                observation(&digest('b'), sample_for_tests(10, 0, 101, 0, 10, 1)),
                observation(&post_boot, post_sample),
            ],
        );
        let publish_events = Rc::clone(&events);
        let error = inspect_sequence_with_source_and_publisher(&binding, &mut source, || {
            publish_events.borrow_mut().push("publish");
            publish_material_for_tests(&store, &slot, record.as_bytes(), store.identity_sha256())
        })
        .expect_err("post-publication failure must be uncertain");
        assert!(matches!(
            error,
            ReplayStoreErrorV1::UncertainAfterPublicationAttempt(_)
        ));
        assert_eq!(
            fs::read(fixture.final_path(&slot)).expect("durable claim survives"),
            record.as_bytes()
        );
        let replay =
            publish_material_for_tests(&store, &slot, record.as_bytes(), store.identity_sha256())
                .expect_err("published nonce must remain consumed");
        assert!(matches!(
            replay,
            ReplayStoreErrorV1::ExistingFinalBlocksReplay
        ));
    }
}

#[test]
fn every_publisher_error_is_conservatively_uncertain() {
    let fixture = StoreFixture::new("publisher-error").expect("publisher-error fixture");
    let store = fixture.open().expect("open publisher-error store");
    let slot = digest('8');
    let record = b"publisher error";
    let boot = digest('b');
    let binding = binding(&store, &slot, &sha256(record), &boot, 100, 102);
    let events = Rc::new(RefCell::new(Vec::new()));
    let mut source = FakeSource::new(
        events,
        vec![observation(&boot, sample_for_tests(10, 0, 100, 0, 10, 1))],
    );
    let error = inspect_sequence_with_source_and_publisher(&binding, &mut source, || {
        Err(ReplayStoreErrorV1::Syscall {
            operation: "injected publisher failure",
            errno: 5,
        })
    })
    .expect_err("publisher failure must be uncertain");
    assert!(matches!(
        error,
        ReplayStoreErrorV1::UncertainAfterPublicationAttempt(_)
    ));
}

#[test]
fn real_linux_boottime_realtime_and_fixed_procfs_sequence_is_observed() {
    let fixture = StoreFixture::new("real-clock").expect("real-clock fixture");
    let store = fixture.open().expect("open real-clock store");
    let mut source = LinuxClockBootSourceV1::open().expect("fixed Linux clock source");
    let initial = source.observe().expect("initial real clock observation");
    let now = u64::try_from(initial.sample.realtime.tv_sec).expect("nonnegative realtime");
    let not_before = now.saturating_sub(1);
    let expires_at = now.checked_add(60).expect("bounded test expiry");
    let slot = digest('f');
    let record = b"real Linux clock sequence";
    let binding = binding(
        &store,
        &slot,
        &sha256(record),
        &initial.boot_id_sha256,
        not_before,
        expires_at,
    );
    let inspected = inspect_sequence_with_source_and_publisher(&binding, &mut source, || {
        publish_material_for_tests(&store, &slot, record, store.identity_sha256())
    })
    .expect("real Linux clock-publication sequence");
    assert!(inspected.pre_realtime_seconds() <= inspected.post_realtime_seconds());
    assert!(inspected.pre_boottime_before_seconds() <= inspected.post_boottime_after_seconds());
    assert!(!inspected.authorizes_live());
}

struct FakeSource {
    events: Rc<RefCell<Vec<&'static str>>>,
    observations: std::collections::VecDeque<ClockBootObservationV1>,
}

impl FakeSource {
    fn new(
        events: Rc<RefCell<Vec<&'static str>>>,
        observations: Vec<ClockBootObservationV1>,
    ) -> Self {
        Self {
            events,
            observations: observations.into(),
        }
    }
}

impl ClockBootSourceV1 for FakeSource {
    fn observe(&mut self) -> crate::ReplayStoreResultV1<ClockBootObservationV1> {
        self.events.borrow_mut().push("observe");
        self.observations
            .pop_front()
            .ok_or_else(|| ReplayStoreErrorV1::Invalid("fake clock sample exhausted".to_string()))
    }

    fn revalidate(&self) -> crate::ReplayStoreResultV1<()> {
        self.events.borrow_mut().push("revalidate");
        Ok(())
    }
}

fn observation(boot: &str, sample: ClockSampleV1) -> ClockBootObservationV1 {
    ClockBootObservationV1::for_tests(boot.to_string(), sample)
}

fn binding(
    store: &ReplayStoreAnchorV1,
    slot: &str,
    full_binding: &str,
    boot: &str,
    not_before: u64,
    expires_at: u64,
) -> PreRunClockClaimBindingV1 {
    PreRunClockClaimBindingV1::for_tests(
        boot.to_string(),
        not_before,
        expires_at,
        ReplayPlatformScopeV1::Nix,
        slot.to_string(),
        full_binding.to_string(),
        store.identity_sha256().to_string(),
    )
}

struct StoreFixture {
    _temporary: tempfile::TempDir,
    expected_gid: u32,
    expected_uid: u32,
    namespace: String,
    root: std::path::PathBuf,
}

impl StoreFixture {
    fn new(label: &str) -> std::io::Result<Self> {
        let temporary = tempfile::tempdir()?;
        let root = temporary.path().join(format!("clock-store-{label}"));
        fs::create_dir(&root)?;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
        let namespace = "claims-v1".to_string();
        fs::create_dir(root.join(&namespace))?;
        fs::set_permissions(root.join(&namespace), fs::Permissions::from_mode(0o700))?;
        let metadata = fs::symlink_metadata(&root)?;
        Ok(Self {
            _temporary: temporary,
            expected_gid: metadata.gid(),
            expected_uid: metadata.uid(),
            namespace,
            root,
        })
    }

    fn open(&self) -> crate::ReplayStoreResultV1<ReplayStoreAnchorV1> {
        open_replay_store(&ReplayStorePolicyV1::for_tests(
            self.root.clone(),
            self.namespace.clone(),
            self.expected_uid,
            self.expected_gid,
        ))
    }

    fn final_path(&self, slot: &str) -> std::path::PathBuf {
        self.root
            .join(&self.namespace)
            .join(format!("{slot}.claim-v1"))
    }
}

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}
