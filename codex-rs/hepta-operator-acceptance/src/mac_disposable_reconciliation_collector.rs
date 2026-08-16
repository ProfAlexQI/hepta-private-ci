//! Read-only restart collector for disposable macOS disk-image lifecycles.
//!
//! This module has no effect primitive. It holds every observed IOMedia and
//! filesystem descriptor across canonical receipt persistence, performs a
//! final replay, and only then releases typed lifecycle observations.

use crate::AcceptanceError;
use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::mac_disposable_effect_issue_store::ExactDisposableCommandV3;
use crate::mac_disposable_effect_issue_store::ExactMountBindingCommandV3;
use crate::mac_disposable_effect_issue_store::ExactMountDeltaCommandViewV3;
use crate::mac_disposable_effect_issue_store::IssuePlanReadSealV3;
use crate::mac_disposable_effect_issue_store::PersistedEjectEffectV3;
use crate::mac_disposable_effect_issue_store::PersistedUnmountEffectV3;
use crate::mac_disposable_lifecycle::CollectorReceiptFileBindingV3;
use crate::mac_disposable_lifecycle::DisposableAuthorityV2;
use crate::mac_disposable_lifecycle::FreshAbsenceObservationV2;
use crate::mac_disposable_lifecycle::PostEffectCollectorBindingV3;
use crate::mac_disposable_lifecycle::ReconciliationMatchV2;
use crate::mac_disposable_lifecycle::ReconciliationSnapshotV2;
use crate::mac_disposable_lifecycle::collector_receipt_file_roster_v3;
use crate::mac_disposable_lifecycle::fresh_absence_sha256;
use crate::mac_disposable_lifecycle::reconciliation_snapshot_sha256;
use crate::mac_disposable_lifecycle_store::ActiveRestartCollectorEpochV3;
use crate::mac_disposable_lifecycle_store::ActiveRestartCollectorSeedV3;
use crate::mac_disposable_lifecycle_store::ActiveRestartEpochV3;
use crate::mac_disposable_lifecycle_store::EjectExpectationArmSealV3;
use crate::mac_disposable_lifecycle_store::PreparedCollectorLifecycleSealV3;
use crate::mac_disposable_lifecycle_store::ReconciliationOperationStoreV3;
use crate::mac_disposable_lifecycle_store::RetainedLifecycleRecordAppendV3;
use crate::mac_disposable_lifecycle_store::SuccessfulIssuedEffectTransitionSealV3;
use crate::mac_iomedia_identity::DiskImageBackingIdentityV2;
use crate::mac_iomedia_identity::ExactDiskImageBackingIdentityV3;
pub use crate::mac_iomedia_identity::FilesystemObjectBindingV3;
use crate::mac_iomedia_identity::HeldDiskImageBacking;
use crate::mac_iomedia_identity::HeldRestartIOMediaInventoryV3;
use crate::mac_iomedia_identity::RestartDiskImageCandidateV3;
use crate::mac_iomedia_identity::RestartIOMediaInventoryV3;
use crate::mac_iomedia_identity::capture_restart_iomedia_inventory_v3;
use crate::mac_iomedia_identity::current_boot_session_uuid;
use crate::mac_iomedia_identity::hold_disk_image_backing;
use crate::mac_iomedia_identity::restart_disk_image_backing_matches_prepared_v3;
use crate::mac_iomedia_identity::validate_disk_image_backing_identity_v2;
use crate::mac_iomedia_identity::validate_restart_iomedia_inventory_v3;
use crate::mac_privileged_disposable_control::S1AdoptedCollectorPairV3;
use crate::mac_privileged_disposable_control::S1CollectorReceiptAppendReadSealV3;
use crate::mac_privileged_disposable_control::S1CollectorReceiptRegistrySealV3;
use crate::mac_privileged_disposable_control::S1PreparedManifestReadSealV3;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::Write;
use std::marker::PhantomData;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::RawFd;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use thiserror::Error;

const RECEIPT_SCHEMA: &str = "hepta_mac_restart_collector_receipt_v3";
const POLICY_SCHEMA: &str = "hepta_mac_restart_collector_policy_v3";
const BASELINE_SCHEMA: &str = "hepta_mac_restart_baseline_inventory_v3";
const MOUNTPOINT_SCHEMA: &str = "hepta_mac_restart_mountpoint_identity_v3";
const ARTIFACT_SCHEMA: &str = "hepta_mac_restart_artifact_evidence_v3";
const MOUNT_SCHEMA: &str = "hepta_mac_restart_mount_evidence_v3";
const PREPARED_COLLECTOR_MANIFEST_SCHEMA: &str = "hepta_mac_prepared_collector_manifest_v3";
const PREPARED_COLLECTOR_PROFILE_SCHEMA: &str = "hepta_mac_prepared_collector_profile_v3";
const MAX_MOUNT_ENTRIES: usize = 4096;
const MAX_MOUNT_STRING_BYTES: usize = 4096;
const MAX_ARTIFACT_ENTRIES: usize = 4096;
const MAX_ARTIFACT_BINDINGS: usize = 10;
const MAX_PROTECTED_ROOTS: usize = 16;
const MAX_RECEIPT_FILES: usize = 64;
const MAX_RECEIPT_BYTES: usize = 16 * 1024 * 1024;
const MAX_RECEIPT_AGGREGATE_BYTES: usize = 64 * 1024 * 1024;
const RENAME_EXCL: libc::c_uint = 0x0000_0004;
const ACL_TYPE_EXTENDED: libc::c_int = 0x0000_0100;
const ACL_FIRST_ENTRY: libc::c_int = 0;

unsafe extern "C" {
    fn renameatx_np(
        from_dirfd: libc::c_int,
        from: *const libc::c_char,
        to_dirfd: libc::c_int,
        to: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn acl_free(object: *mut libc::c_void) -> libc::c_int;
    fn acl_get_entry(
        acl: *mut libc::c_void,
        entry_id: libc::c_int,
        entry: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn acl_get_fd_np(fd: libc::c_int, acl_type: libc::c_int) -> *mut libc::c_void;
}

#[derive(Debug, Error)]
pub enum RestartCollectorErrorV3 {
    #[error("invalid macOS restart collector: {0}")]
    Invalid(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Acceptance(#[from] AcceptanceError),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RestartCollectorBindingsV3 {
    backing_identity_sha256: String,
    baseline_inventory_sha256: String,
    boot_session_uuid: String,
    collector_policy_sha256: String,
    mountpoint_underlying_sha256: String,
    operation_nonce: String,
    restart_epoch_nonce: String,
    restart_started_monotonic_nanoseconds: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RestartCollectorPolicyV3 {
    artifacts: Vec<PreparedArtifactBindingV3>,
    artifact_root: String,
    artifact_root_identity: StableDirectoryIdentityV3,
    authority: DisposableAuthorityV2,
    backing_path: String,
    max_iomedia_objects: usize,
    max_mount_entries: usize,
    mountpoint: String,
    protected_roots: Vec<String>,
    receipt_root: String,
    receipt_root_identity: StableDirectoryIdentityV3,
    schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RestartBaselineInventoryV3 {
    authority: DisposableAuthorityV2,
    boot_session_uuid: String,
    registry_entry_ids: Vec<String>,
    schema: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CollectorPurposeV3 {
    ReconciliationSnapshot,
    FreshAbsence,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountBindingV3 {
    pub filesystem_id: [i32; 2],
    pub filesystem_type: String,
    pub mount_flags: u64,
    pub mount_from: String,
    pub mount_on: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StableDirectoryIdentityV3 {
    pub birthtime_nanoseconds: i64,
    pub birthtime_seconds: i64,
    pub dev: u64,
    pub flags: u32,
    pub generation: u32,
    pub gid: u32,
    pub inode: u64,
    pub mode: u32,
    pub nlink: u64,
    pub roster_entries: u64,
    pub uid: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ArtifactRoleV3 {
    BackingImage,
    MountpointUnderlying,
    DiskImageDevice,
    PhysicalWhole,
    PhysicalStore,
    ApfsContainer,
    ApfsVolume,
    CollectorReceipt,
    LifecycleRecord,
    EffectIssueRecord,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PreparedArtifactBindingV3 {
    basename: String,
    role: ArtifactRoleV3,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountpointIdentityV3 {
    pub authority: DisposableAuthorityV2,
    pub binding: FilesystemObjectBindingV3,
    pub path: String,
    pub schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatchingDiskImageGroupV3 {
    pub candidate: RestartDiskImageCandidateV3,
    pub member_bsd_names: Vec<String>,
    pub member_registry_entry_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ArtifactEvidenceV3 {
    artifacts: Vec<PreparedArtifactBindingV3>,
    artifact_root: String,
    authority: DisposableAuthorityV2,
    operation_artifacts_absent: bool,
    root_binding: FilesystemObjectBindingV3,
    roster: Vec<String>,
    schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct MountEvidenceV3 {
    authority: DisposableAuthorityV2,
    mountpoint_underlying_revalidated: bool,
    mounts_after: Vec<MountBindingV3>,
    mounts_before: Vec<MountBindingV3>,
    no_nested_mounts: bool,
    schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestartCollectorReceiptV3 {
    artifact_evidence: ArtifactEvidenceV3,
    pub artifact_evidence_sha256: String,
    pub authority: DisposableAuthorityV2,
    backing_identity: DiskImageBackingIdentityV2,
    pub backing_identity_sha256: String,
    baseline_inventory: RestartBaselineInventoryV3,
    pub baseline_inventory_sha256: String,
    pub baseline_restored: bool,
    pub boot_session_uuid: String,
    collector_policy: RestartCollectorPolicyV3,
    pub collector_policy_sha256: String,
    current_expected_absence_inventory: Option<RestartIOMediaInventoryV3>,
    current_expected_absence_inventory_sha256: Option<String>,
    pub iomedia_evidence_sha256: String,
    pub iomedia_inventory: RestartIOMediaInventoryV3,
    pub match_result: ReconciliationMatchV2,
    pub matching_groups: Vec<MatchingDiskImageGroupV3>,
    pub monotonic_after_nanoseconds: u64,
    pub monotonic_before_nanoseconds: u64,
    pub mount_evidence_sha256: String,
    mount_evidence: MountEvidenceV3,
    mountpoint_underlying: MountpointIdentityV3,
    pub mountpoint_underlying_sha256: String,
    pub operation_artifacts_absent: bool,
    pub operation_nonce: String,
    pub post_inventory_sha256: String,
    purpose: CollectorPurposeV3,
    pub reconciliation_snapshot_sha256: Option<String>,
    pub restart_epoch_nonce: String,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Debug, Eq, PartialEq)]
enum FinalizedRestartObservationV3 {
    ReconciliationSnapshot(ReconciliationSnapshotV2),
    FreshAbsence(FreshAbsenceObservationV2),
}

#[derive(Clone, Debug)]
struct LiveRestartCollectorRequestV3<'a> {
    artifact_root: &'a Path,
    baseline: &'a RestartBaselineInventoryV3,
    bindings: &'a RestartCollectorBindingsV3,
    mountpoint_identity: &'a MountpointIdentityV3,
    policy: &'a RestartCollectorPolicyV3,
    prepared_backing: &'a DiskImageBackingIdentityV2,
    receipt_root: &'a Path,
}

/// Canonical durable description captured while the fresh operation still
/// owns every prepared object.  It is data, not a capability: production
/// collection must additionally retain and replay the descriptor-backed
/// [`RetainedPreparedCollectorCapabilityV3`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PreparedCollectorManifestV3 {
    artifact_root_initial_roster: Vec<String>,
    authority: DisposableAuthorityV2,
    backing: DiskImageBackingIdentityV2,
    backing_exact: ExactDiskImageBackingIdentityV3,
    baseline: RestartBaselineInventoryV3,
    mountpoint: MountpointIdentityV3,
    operation_nonce: String,
    policy: RestartCollectorPolicyV3,
    prepared_boot_session_uuid: String,
    profile_sha256: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    receipt_root_initial_binding: Option<FilesystemObjectBindingV3>,
    receipt_root_initial_roster: Vec<String>,
    schema: String,
    schema_version: u32,
}

enum PreparedBaselineGuardV3 {
    Captured(HeldRestartIOMediaInventoryV3),
    DurableCommitment,
}

/// Non-serializable prepared collector authority.  The canonical manifest is
/// useful only while these exact live descriptors continue to replay.  A
/// restart may reconstruct this capability solely from an exact durable
/// manifest; there is no digest-only or DTO-only production constructor.
pub(crate) struct RetainedPreparedCollectorCapabilityV3 {
    artifact_root: HeldDirectoryV3,
    backing: HeldDiskImageBacking,
    baseline_guard: PreparedBaselineGuardV3,
    manifest: PreparedCollectorManifestV3,
    manifest_bytes: Vec<u8>,
    manifest_sha256: String,
    mountpoint: UnderlyingMountpointGuardV3,
    operation_nonce: String,
    initial_receipt_root_owner: Option<RetainedCollectorReceiptRootOwnerV3>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct PreparedCollectorProfileV3<'a> {
    artifacts: &'a [PreparedArtifactBindingV3],
    schema: &'static str,
}

struct HeldDirectoryV3 {
    binding: FilesystemObjectBindingV3,
    file: File,
    path: PathBuf,
}

struct LiveReplayGuardV3 {
    artifact_evidence: ArtifactEvidenceV3,
    artifact_root: HeldDirectoryV3,
    backing: HeldDiskImageBacking,
    iomedia: HeldRestartIOMediaInventoryV3,
    mountpoint: UnderlyingMountpointGuardV3,
    mounts: Vec<MountBindingV3>,
    prepared_backing: DiskImageBackingIdentityV2,
}

enum UnderlyingMountpointGuardV3 {
    Held(HeldDirectoryV3),
    DeferredWhileMounted {
        basename: String,
        expected: MountpointIdentityV3,
        parent: HeldDirectoryV3,
    },
}

pub(crate) struct PendingRestartObservationV3 {
    guard: LiveReplayGuardV3,
    receipt: RestartCollectorReceiptV3,
    receipt_root_owner: RetainedCollectorReceiptRootOwnerV3,
}

struct ValidatedExistingReceiptV3 {
    binding: FilesystemObjectBindingV3,
    bytes: Vec<u8>,
    expected_lifecycle_binding: Option<CollectorReceiptFileBindingV3>,
    file: File,
    lifecycle_binding: Option<CollectorReceiptFileBindingV3>,
    name: String,
    receipt: RestartCollectorReceiptV3,
}

struct ReceiptRootSnapshotV3 {
    aggregate_bytes: usize,
    entries: Vec<ValidatedExistingReceiptV3>,
    roster: Vec<String>,
}

/// The sole linear owner of the receipt-root generation. It retains every
/// exact entry descriptor and the full current roster. Publishing consumes a
/// generation and returns its one successor; individual receipt capsules do
/// not freeze or police the global roster.
struct RetainedReceiptRootV3 {
    current_binding: FilesystemObjectBindingV3,
    directory: File,
    initial_binding: FilesystemObjectBindingV3,
    path: PathBuf,
    snapshot: ReceiptRootSnapshotV3,
    stable_identity: StableDirectoryIdentityV3,
}

/// The sole S2 owner of the external collector-receipt namespace.
///
/// The whole operation store keeps this value between publications. A
/// collection moves it into the pending/unadopted transaction and successful
/// paired S1 adoption returns the unique successor alongside the positive
/// observation. Observations retain their own exact receipt and lifecycle
/// capsules, but never own or freeze a historical copy of the full root.
pub(crate) struct RetainedCollectorReceiptRootOwnerV3 {
    root: RetainedReceiptRootV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Opaque S1 mirror of one exact external receipt-root generation. It exposes
/// no path or descriptor; S1 captures it from the exact lifecycle census and
/// can only revalidate that retained generation.
pub(crate) struct S1RetainedCollectorReceiptRootV3 {
    root: RetainedReceiptRootV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

struct DurableCollectorReceiptV3 {
    bytes: Vec<u8>,
    directory: File,
    directory_identity: StableDirectoryIdentityV3,
    file: File,
    file_binding: FilesystemObjectBindingV3,
    final_name: String,
    path: PathBuf,
    root_after_binding: FilesystemObjectBindingV3,
    root_generation_ordinal: u32,
}

/// Receipt publication has completed, but neither the lifecycle record nor
/// the S1 receipt-root mirror has adopted the new generation.  This is not a
/// positive collector observation and exposes no issue, mount, or terminal
/// capability.
struct UnadoptedCollectorGenerationCoreV3 {
    before_root_binding: FilesystemObjectBindingV3,
    durable: DurableCollectorReceiptV3,
    expected_lifecycle_binding: CollectorReceiptFileBindingV3,
    guard: LiveReplayGuardV3,
    observation: FinalizedRestartObservationV3,
    receipt: RestartCollectorReceiptV3,
    receipt_root_owner: RetainedCollectorReceiptRootOwnerV3,
    receipt_sha256: String,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// The sole S2 owner of one durable-but-unadopted G -> G+1 collector
/// generation.  Consuming this value can mint exactly one S1 transfer; there
/// is no borrowed or repeatable descriptor-transfer API.
pub(crate) struct UnadoptedCollectorGenerationV3 {
    core: UnadoptedCollectorGenerationCoreV3,
}

/// The S2 generation after its one S1 descriptor transfer has been consumed.
/// It still owns the original complete S2 root generation and can become a
/// positive observation only by consuming an exact paired-adoption token.
pub(crate) struct UnadoptedCollectorGenerationAfterTransferV3 {
    core: UnadoptedCollectorGenerationCoreV3,
}

/// One-shot transfer of only the newly published receipt entry.  S1 already
/// retains the root directory and every prior entry, so cloning the complete
/// root would create a repeatable capability outlet without adding evidence.
pub(crate) struct S1CollectorReceiptAppendTransferV3 {
    core: UnadoptedCollectorGenerationCoreV3,
    new_bytes: Vec<u8>,
    new_receipt: File,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Prevalidated, allocation-free commit material for advancing the S1 mirror
/// from one exact generation to its sole successor.
pub(crate) struct S1CollectorReceiptAppendCommitV3 {
    after_binding: FilesystemObjectBindingV3,
    entry: ValidatedExistingReceiptV3,
    entry_insert_index: usize,
    next_aggregate_bytes: usize,
    next_roster: Vec<String>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Only this module may open the opaque S1 paired-adoption acknowledgement.
pub(crate) struct S1CollectorPairAdoptionReadSealV3 {
    _private: (),
}

impl PreparedArtifactBindingV3 {
    pub(crate) fn new(
        role: ArtifactRoleV3,
        basename: &str,
    ) -> Result<Self, RestartCollectorErrorV3> {
        let binding = Self {
            basename: validate_child_name(basename)?.to_string(),
            role,
        };
        validate_artifact_bindings(std::slice::from_ref(&binding), false)?;
        Ok(binding)
    }
}

pub(crate) struct AttachedV3;
pub(crate) struct MountedV3;

struct RetainedCollectorEvidenceV3 {
    durable: DurableCollectorReceiptV3,
    guard: LiveReplayGuardV3,
    lifecycle_record: Option<RetainedLifecycleRecordAppendV3>,
    observation: FinalizedRestartObservationV3,
    receipt: RestartCollectorReceiptV3,
    receipt_sha256: String,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub(crate) struct RetainedZeroMatchV3 {
    evidence: RetainedCollectorEvidenceV3,
}

pub(crate) struct RetainedUniqueMatchV3<S> {
    evidence: RetainedCollectorEvidenceV3,
    _state: PhantomData<S>,
}

pub(crate) struct RetainedAmbiguousMatchV3 {
    evidence: RetainedCollectorEvidenceV3,
}

pub(crate) struct RetainedFreshAbsenceV3 {
    evidence: RetainedCollectorEvidenceV3,
}

pub(crate) enum RetainedCollectorMatchV3 {
    Zero(RetainedZeroMatchV3),
    UniqueAttached(RetainedUniqueMatchV3<AttachedV3>),
    UniqueMounted(RetainedUniqueMatchV3<MountedV3>),
    Ambiguous(RetainedAmbiguousMatchV3),
}

pub(crate) enum RetainedCollectorObservationV3 {
    Reconciliation(RetainedCollectorMatchV3),
    FreshAbsence(RetainedFreshAbsenceV3),
}

/// Linear owner for the complete collector lineage of one restart epoch.
///
/// `First` is a marker rather than a second observation, so the first receipt
/// has exactly one retained descriptor owner.  Later observations advance only
/// `current`; the original admission snapshot remains live and is replayed
/// across every completed mount-table delta.
pub(crate) struct RetainedCollectorLineageV3 {
    current: RetainedCollectorCurrentV3,
    first: RetainedCollectorObservationV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

enum RetainedCollectorCurrentV3 {
    First,
    MountDelta {
        direction: MountDeltaDirectionV3,
        expected_after: Vec<MountBindingV3>,
        observation: RetainedCollectorObservationV3,
    },
    EjectedZero {
        binding: EjectExpectationBindingV3,
        observation: RetainedCollectorObservationV3,
        prior: Box<RetainedCollectorCurrentV3>,
    },
    FreshAbsence(RetainedCollectorObservationV3),
}

pub(crate) struct MountingV3;
pub(crate) struct UnmountingV3;

/// Linear pending mount-table transition.  It owns the complete prior
/// collector lineage, so the first admission receipt cannot be dropped while
/// a post-effect observation replaces only the current evidence.
pub(crate) struct RetainedCollectorMountDeltaV3<K> {
    after: Vec<MountBindingV3>,
    before: Vec<MountBindingV3>,
    command_sha256: String,
    operation_nonce: String,
    prior: RetainedCollectorLineageV3,
    target: MountBindingV3,
    _kind: PhantomData<K>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Borrowed sealed view consumed by S1 when it changes only the mount
/// typestate.  It exposes no descriptor and cannot outlive the linear retained
/// collector transition which owns all evidence.
pub(crate) struct SealedMountDeltaPlanV3<'a, K> {
    delta: &'a RetainedCollectorMountDeltaV3<K>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub(crate) struct SealedMountDeltaAdvanceV3<'a, K> {
    delta: &'a RetainedCollectorMountDeltaV3<K>,
    next: &'a RetainedCollectorObservationV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Borrowed proof that one persisted post-effect collector receipt is the
/// exact expected-after observation for a linear delta and has not yet been
/// appended to lifecycle storage.  S2 may inspect only its derived digest;
/// callers cannot construct it from a digest or mount-table DTO.
/// Borrowed proof that one durable-but-unadopted receipt is the exact
/// expected-after collector result for a linear mount delta.  It exposes only
/// the sealed lifecycle projection needed to build the paired append; no
/// positive observation exists until S1 adopts that exact pair.
pub(crate) struct SealedUnadoptedMountDeltaObservationV3<'a, K> {
    delta: &'a RetainedCollectorMountDeltaV3<K>,
    next: &'a UnadoptedCollectorGenerationV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Private mint seal for a post-effect lifecycle binding.  The lifecycle
/// schema may deserialize historical bytes, but no other production module can
/// construct a new binding from caller-supplied strings.
pub(crate) struct PostEffectCollectorBindingSealV3 {
    _private: (),
}

/// Private mint seal proving the lifecycle receipt projection came from the
/// final reopened collector receipt descriptor rather than caller data.
pub(crate) struct CollectorReceiptFileBindingSealV3 {
    _private: (),
}

#[derive(Clone, Copy)]
enum MountDeltaDirectionV3 {
    Mount,
    Unmount,
}

pub(crate) enum RetainedCollectorAppendEventV3<'a> {
    ReconciliationSnapshot(&'a ReconciliationSnapshotV2),
    FreshAbsence(&'a FreshAbsenceObservationV2),
}

/// Borrowed append material from a durable-but-unadopted generation.  It is
/// only a projection for constructing the exact next lifecycle record; it is
/// not positive collector evidence.
pub(crate) struct UnadoptedCollectorAppendV3<'a> {
    event: RetainedCollectorAppendEventV3<'a>,
    operation_nonce: &'a str,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Sealed, descriptor-backed input for one effect issue.  The fields cannot
/// be caller supplied: construction requires a still-live, lifecycle-bound
/// unique collector observation and every use replays that observation.
pub(crate) struct RetainedCollectorIssueBindingV3<'a> {
    receipt_root_owner: &'a RetainedCollectorReceiptRootOwnerV3,
    retained: &'a RetainedCollectorObservationV3,
    boot_session_uuid: String,
    lifecycle_record_sha256: String,
    lifecycle_record_sequence: u32,
    operation_nonce: String,
    receipt_sha256: String,
    unique_binding_sha256: String,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Owned, descriptor-backed plan for the only unmount which the current
/// retained unique-mounted observation admits.  Construction consumes both
/// the receipt-root generation owner and the complete collector lineage, so a
/// sibling cannot retain one evidence-bearing copy while issuing from
/// another.  The exact command and all of its digests remain private.
pub(crate) struct SealedUnmountEffectPlanV3 {
    core: SealedCollectorEffectPlanCoreV3,
}

/// Owned, descriptor-backed plan for the only eject which the current
/// retained unique-attached observation admits.  As with the unmount plan,
/// there is no constructor from a command, effect kind, group digest, mount
/// binding, or serialized projection.
pub(crate) struct SealedEjectEffectPlanV3 {
    core: SealedCollectorEffectPlanCoreV3,
}

/// Exact pre-dispatch eject expectation.  It replaces the live unique-attached
/// effect plan once the issue is durable and before the command is dispatched.
/// Pending replay admits only the exact retained before inventory or its
/// internally derived expected-after inventory; it never requires the ejected
/// IOMedia descriptors to remain live after a successful effect.
pub(crate) struct ArmedEjectExpectationV3 {
    core: ArmedEjectExpectationCoreV3,
    receipt_root_owner: RetainedCollectorReceiptRootOwnerV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

struct ArmedEjectExpectationCoreV3 {
    binding: EjectExpectationBindingV3,
    lineage: RetainedCollectorLineageV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

#[derive(Debug, Eq, PartialEq)]
struct EjectExpectationBindingV3 {
    before_inventory: RestartIOMediaInventoryV3,
    command_sha256: String,
    disk_image_group_sha256: String,
    expected_after_inventory: RestartIOMediaInventoryV3,
    expected_after_inventory_sha256: String,
    provenance: SealedCollectorEffectPlanProvenanceV3,
    unchanged_mounts: Vec<MountBindingV3>,
}

#[derive(Clone, Copy)]
enum EjectInventoryEndpointV3 {
    Pending,
    ExpectedAfter,
}

/// Live post-eject collection plus the exact consumed pre-effect expectation.
/// This is not positive evidence until its receipt and lifecycle record are
/// durably paired and adopted by S1.
pub(crate) struct PendingEjectCollectorObservationV3 {
    expectation: ArmedEjectExpectationCoreV3,
    pending: PendingRestartObservationV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub(crate) struct UnadoptedEjectObservationV3 {
    expectation: ArmedEjectExpectationCoreV3,
    generation: UnadoptedCollectorGenerationV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub(crate) struct EjectObservationAfterTransferV3 {
    expectation: ArmedEjectExpectationCoreV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub(crate) struct SealedUnadoptedEjectObservationV3<'a> {
    expectation: &'a ArmedEjectExpectationCoreV3,
    generation: &'a UnadoptedCollectorGenerationV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

struct SealedCollectorEffectPlanCoreV3 {
    command: ExactDisposableCommandV3,
    command_canonical_bytes: Vec<u8>,
    command_sha256: String,
    lineage: RetainedCollectorLineageV3,
    provenance: SealedCollectorEffectPlanProvenanceV3,
    receipt_root_owner: RetainedCollectorReceiptRootOwnerV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SealedCollectorEffectPlanProvenanceV3 {
    boot_session_uuid: String,
    collector_receipt_sha256: String,
    lifecycle_record_sequence: u32,
    lifecycle_record_sha256: String,
    operation_nonce: String,
    restart_epoch_nonce: String,
    specific: SealedCollectorEffectSpecificProvenanceV3,
    unique_binding_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SealedCollectorEffectSpecificProvenanceV3 {
    Unmount {
        mounted_binding: MountBindingV3,
        mounted_binding_sha256: String,
    },
    Eject {
        disk_image_group_sha256: String,
    },
}

#[derive(Clone, Copy)]
enum SealedCollectorEffectPlanKindV3 {
    Unmount,
    Eject,
}

struct DerivedCollectorEffectPlanV3 {
    command: ExactDisposableCommandV3,
    command_canonical_bytes: Vec<u8>,
    command_sha256: String,
    provenance: SealedCollectorEffectPlanProvenanceV3,
}

/// Seal-gated borrowed issue material.  The effect store may inspect this
/// value only with its own private read seal; retaining or serializing the
/// material is impossible and the underlying owner/lineage stay in the owned
/// plan until the issued runner reaches a terminal proof state.
pub(crate) struct SealedCollectorEffectIssuePlanV3<'a, K> {
    collector_binding: RetainedCollectorIssueBindingV3<'a>,
    command: &'a ExactDisposableCommandV3,
    plan: &'a SealedCollectorEffectPlanCoreV3,
    _kind: PhantomData<K>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Borrowed terminal token sealed by one exact retained FreshAbsence
/// observation and its adopted lifecycle-record capsule.
pub(crate) struct RetainedTerminalAbsenceV3<'a> {
    fresh_absence_sha256: String,
    operation_nonce: String,
    receipt_root_owner: &'a RetainedCollectorReceiptRootOwnerV3,
    retained: &'a RetainedCollectorObservationV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl RestartCollectorPolicyV3 {
    #[cfg(test)]
    fn new(
        backing_path: &Path,
        mountpoint: &Path,
        artifact_root: &Path,
        receipt_root: &Path,
        artifacts: &[PreparedArtifactBindingV3],
        protected_roots: &[&Path],
    ) -> Result<Self, RestartCollectorErrorV3> {
        capture_policy_and_roots(
            backing_path,
            mountpoint,
            artifact_root,
            receipt_root,
            artifacts,
            protected_roots,
        )
        .map(|(policy, _, _)| policy)
    }

    fn sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        validate_policy(self)?;
        Ok(sha256(&canonical_json(self)?))
    }
}

#[allow(clippy::too_many_arguments)]
fn capture_policy_and_roots(
    backing_path: &Path,
    mountpoint: &Path,
    artifact_root: &Path,
    receipt_root: &Path,
    artifacts: &[PreparedArtifactBindingV3],
    protected_roots: &[&Path],
) -> Result<(RestartCollectorPolicyV3, HeldDirectoryV3, HeldDirectoryV3), RestartCollectorErrorV3> {
    let backing_path = canonical_input_path(backing_path, "backing path", false)?;
    let mountpoint = canonical_input_path(mountpoint, "mountpoint", true)?;
    let artifact_root = canonical_input_path(artifact_root, "artifact root", true)?;
    let receipt_root = canonical_input_path(receipt_root, "receipt root", true)?;
    if protected_roots.len() > MAX_PROTECTED_ROOTS.saturating_sub(2) {
        return Err(invalid("protected-root roster exceeds its bound"));
    }
    if artifacts.len() > MAX_ARTIFACT_BINDINGS {
        return Err(invalid("prepared artifact roster exceeds its bound"));
    }
    let mut artifacts = artifacts.to_vec();
    artifacts.sort();
    validate_artifact_bindings(&artifacts, true)?;
    let mut roots = protected_roots
        .iter()
        .map(|path| canonical_input_path(path, "protected root", true))
        .collect::<Result<Vec<_>, _>>()?;
    roots.push(artifact_root.clone());
    roots.push(receipt_root.clone());
    roots.sort();
    roots.dedup();
    let artifact_root_held =
        HeldDirectoryV3::capture(Path::new(&artifact_root), "prepared artifact root")?;
    let receipt_root_held =
        HeldDirectoryV3::capture(Path::new(&receipt_root), "prepared receipt root")?;
    validate_receipt_directory(&receipt_root_held.binding)?;
    let artifact_root_roster =
        list_directory(artifact_root_held.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES)?;
    let receipt_root_roster =
        list_directory(receipt_root_held.file.as_raw_fd(), MAX_RECEIPT_FILES)?;
    artifact_root_held.revalidate("prepared artifact root")?;
    receipt_root_held.revalidate("prepared receipt root")?;
    let policy = RestartCollectorPolicyV3 {
        artifacts,
        artifact_root,
        artifact_root_identity: StableDirectoryIdentityV3::from_binding(
            &artifact_root_held.binding,
            artifact_root_roster.len(),
        ),
        authority: DisposableAuthorityV2::none(),
        backing_path,
        max_iomedia_objects: 256,
        max_mount_entries: MAX_MOUNT_ENTRIES,
        mountpoint,
        protected_roots: roots,
        receipt_root,
        receipt_root_identity: StableDirectoryIdentityV3::from_binding(
            &receipt_root_held.binding,
            receipt_root_roster.len(),
        ),
        schema: POLICY_SCHEMA.to_string(),
    };
    validate_policy(&policy)?;
    Ok((policy, artifact_root_held, receipt_root_held))
}

impl RetainedPreparedCollectorCapabilityV3 {
    #[allow(clippy::too_many_arguments)]
    fn capture(
        operation_nonce: &str,
        backing_path: &Path,
        mountpoint: &Path,
        artifact_root: &Path,
        receipt_root: &Path,
        backing_artifact_basename: &str,
        protected_roots: &[&Path],
    ) -> Result<Self, RestartCollectorErrorV3> {
        if !valid_nonce(operation_nonce) {
            return Err(invalid("prepared collector operation nonce is malformed"));
        }
        let artifacts = prepared_backing_profile(backing_artifact_basename)?;
        let (policy, artifact_root, receipt_root) = capture_policy_and_roots(
            backing_path,
            mountpoint,
            artifact_root,
            receipt_root,
            &artifacts,
            protected_roots,
        )?;
        let baseline_guard = capture_restart_iomedia_inventory_v3()?;
        let baseline = RestartBaselineInventoryV3::from_inventory(baseline_guard.report())?;
        let backing = hold_disk_image_backing(Path::new(&policy.backing_path))?;
        let backing_identity = backing.identity()?;
        let backing_exact = backing.exact_identity_v3()?;
        validate_prepared_backing_artifact(&artifact_root, &policy, &backing_exact)?;
        let mountpoint_held =
            HeldDirectoryV3::capture(Path::new(&policy.mountpoint), "mountpoint")?;
        let mountpoint = mountpoint_identity_from_held(&mountpoint_held)?;
        let profile_sha256 = prepared_profile_sha256(&policy.artifacts)?;
        let prepared_boot_session_uuid = baseline.boot_session_uuid.clone();
        let artifact_root_initial_roster =
            list_directory(artifact_root.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES)?;
        let expected_artifact_roster = policy
            .artifacts
            .iter()
            .map(|artifact| artifact.basename.clone())
            .collect::<Vec<_>>();
        if artifact_root_initial_roster != expected_artifact_roster {
            return Err(invalid(
                "fresh prepared artifact root differs from its exact required profile roster",
            ));
        }
        let receipt_root_initial_roster =
            list_directory(receipt_root.file.as_raw_fd(), MAX_RECEIPT_FILES)?;
        let manifest = PreparedCollectorManifestV3 {
            artifact_root_initial_roster,
            authority: DisposableAuthorityV2::none(),
            backing: backing_identity,
            backing_exact,
            baseline,
            mountpoint,
            operation_nonce: operation_nonce.to_string(),
            policy,
            prepared_boot_session_uuid,
            profile_sha256,
            receipt_root_initial_binding: Some(receipt_root.binding),
            receipt_root_initial_roster,
            schema: PREPARED_COLLECTOR_MANIFEST_SCHEMA.to_string(),
            schema_version: 3,
        };
        let (manifest_bytes, manifest_sha256) = canonical_prepared_manifest(&manifest)?;
        let receipt_root =
            RetainedCollectorReceiptRootOwnerV3::from_root(RetainedReceiptRootV3::from_held(
                receipt_root,
                manifest.policy.receipt_root_identity,
                manifest.receipt_root_initial_binding.ok_or_else(|| {
                    invalid("new prepared collector manifest lost its initial receipt-root binding")
                })?,
            )?);
        let retained = Self {
            artifact_root,
            backing,
            baseline_guard: PreparedBaselineGuardV3::Captured(baseline_guard),
            manifest,
            manifest_bytes,
            manifest_sha256,
            mountpoint: UnderlyingMountpointGuardV3::Held(mountpoint_held),
            operation_nonce: operation_nonce.to_string(),
            initial_receipt_root_owner: Some(receipt_root),
            _not_send_or_sync: PhantomData,
        };
        retained.revalidate()?;
        Ok(retained)
    }

    fn reopen_from_exact_manifest(
        operation_nonce: &str,
        manifest_bytes: &[u8],
        expected_manifest_sha256: &str,
        expected_profile_sha256: &str,
    ) -> Result<Self, RestartCollectorErrorV3> {
        if !valid_nonce(operation_nonce)
            || !valid_digest(expected_manifest_sha256)
            || !valid_digest(expected_profile_sha256)
            || manifest_bytes.is_empty()
            || manifest_bytes.len() > MAX_RECEIPT_BYTES
        {
            return Err(invalid(
                "durable prepared collector commitment is malformed or oversized",
            ));
        }
        let manifest: PreparedCollectorManifestV3 = serde_json::from_slice(manifest_bytes)
            .map_err(|error| {
                invalid(format!("prepared collector manifest JSON failed: {error}"))
            })?;
        let (canonical, manifest_sha256) = canonical_prepared_manifest_bytes(&manifest)?;
        if canonical != manifest_bytes
            || manifest_sha256 != expected_manifest_sha256
            || manifest.profile_sha256 != expected_profile_sha256
            || manifest.operation_nonce != operation_nonce
        {
            return Err(invalid(
                "prepared collector manifest differs from its exact durable commitment",
            ));
        }
        let receipt_root_initial_binding =
            manifest.receipt_root_initial_binding.ok_or_else(|| {
                invalid("legacy prepared collector manifest cannot enter active receipt generation")
            })?;
        validate_prepared_manifest(&manifest)?;
        let artifact_root =
            HeldDirectoryV3::capture(Path::new(&manifest.policy.artifact_root), "artifact root")?;
        let receipt_root =
            HeldDirectoryV3::capture(Path::new(&manifest.policy.receipt_root), "receipt root")?;
        validate_prepared_artifact_root(
            &artifact_root,
            &manifest.policy.artifact_root_identity,
            &manifest.artifact_root_initial_roster,
            &manifest.policy.artifacts,
        )?;
        validate_prepared_receipt_root(&receipt_root, &manifest.policy.receipt_root_identity)?;
        validate_receipt_directory(&receipt_root.binding)?;
        let backing = hold_disk_image_backing(Path::new(&manifest.policy.backing_path))?;
        let live_backing = backing.identity()?;
        if !prepared_backing_candidate_matches(&live_backing, &manifest.backing)?
            || backing.exact_identity_v3()? != manifest.backing_exact
        {
            return Err(invalid(
                "live backing differs from the exact durable prepared identity",
            ));
        }
        validate_prepared_backing_artifact(
            &artifact_root,
            &manifest.policy,
            &manifest.backing_exact,
        )?;
        let mountpoint = reopen_prepared_mountpoint(&manifest.mountpoint)?;
        let receipt_root =
            RetainedCollectorReceiptRootOwnerV3::from_root(RetainedReceiptRootV3::from_held(
                receipt_root,
                manifest.policy.receipt_root_identity,
                receipt_root_initial_binding,
            )?);
        let retained = Self {
            artifact_root,
            backing,
            baseline_guard: PreparedBaselineGuardV3::DurableCommitment,
            manifest,
            manifest_bytes: canonical,
            manifest_sha256,
            mountpoint,
            operation_nonce: operation_nonce.to_string(),
            initial_receipt_root_owner: Some(receipt_root),
            _not_send_or_sync: PhantomData,
        };
        retained.revalidate()?;
        Ok(retained)
    }

    fn manifest_bytes(&self) -> &[u8] {
        &self.manifest_bytes
    }

    fn manifest_sha256(&self) -> &str {
        &self.manifest_sha256
    }

    fn profile_sha256(&self) -> &str {
        &self.manifest.profile_sha256
    }

    fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    fn baseline_inventory_sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        self.manifest.baseline.sha256()
    }

    fn backing_identity_sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        Ok(sha256(&canonical_json(&self.manifest.backing)?))
    }

    fn collector_policy_sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        self.manifest.policy.sha256()
    }

    fn mountpoint_underlying_sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        self.manifest.mountpoint.sha256()
    }

    fn boot_session_uuid(&self) -> &str {
        &self.manifest.baseline.boot_session_uuid
    }

    pub(crate) fn lifecycle_manifest<'a>(
        &'a self,
        _seal: &PreparedCollectorLifecycleSealV3,
    ) -> Result<(&'a str, &'a [u8]), RestartCollectorErrorV3> {
        self.revalidate()?;
        Ok((&self.operation_nonce, &self.manifest_bytes))
    }

    pub(crate) fn lifecycle_prepared_fields(
        &self,
        _seal: &PreparedCollectorLifecycleSealV3,
    ) -> Result<(String, String, String, String, String), RestartCollectorErrorV3> {
        self.revalidate()?;
        Ok((
            self.baseline_inventory_sha256()?,
            self.backing_identity_sha256()?,
            self.boot_session_uuid().to_string(),
            self.collector_policy_sha256()?,
            self.mountpoint_underlying_sha256()?,
        ))
    }

    pub(crate) fn lifecycle_initial_receipt_root_binding(
        &self,
        _seal: &PreparedCollectorLifecycleSealV3,
    ) -> Result<FilesystemObjectBindingV3, RestartCollectorErrorV3> {
        self.revalidate()?;
        let initial = self.manifest.receipt_root_initial_binding.ok_or_else(|| {
            invalid("legacy prepared manifest has no exact initial receipt-root endpoint")
        })?;
        let root = self.initial_receipt_root_owner.as_ref().ok_or_else(|| {
            invalid("prepared initial receipt-root owner was already transferred")
        })?;
        if !root.root.snapshot.roster.is_empty() || root.root.current_binding != initial {
            return Err(invalid(
                "prepared lifecycle can bind only the exact empty initial receipt-root endpoint",
            ));
        }
        Ok(initial)
    }

    pub(crate) fn reopen_from_lifecycle_manifest(
        operation_nonce: &str,
        manifest_bytes: &[u8],
        expected_manifest_sha256: &str,
        _seal: &PreparedCollectorLifecycleSealV3,
    ) -> Result<Self, RestartCollectorErrorV3> {
        let parsed: PreparedCollectorManifestV3 =
            serde_json::from_slice(manifest_bytes).map_err(|error| {
                invalid(format!(
                    "prepared collector manifest JSON failed before sealed replay: {error}"
                ))
            })?;
        Self::reopen_from_exact_manifest(
            operation_nonce,
            manifest_bytes,
            expected_manifest_sha256,
            &parsed.profile_sha256,
        )
    }

    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        if !valid_nonce(&self.operation_nonce)
            || self.manifest.operation_nonce != self.operation_nonce
        {
            return Err(invalid("retained prepared collector operation changed"));
        }
        validate_prepared_manifest(&self.manifest)?;
        let (bytes, digest) = canonical_prepared_manifest(&self.manifest)?;
        if bytes != self.manifest_bytes || digest != self.manifest_sha256 {
            return Err(invalid(
                "retained prepared collector manifest changed after capture",
            ));
        }
        validate_prepared_artifact_root(
            &self.artifact_root,
            &self.manifest.policy.artifact_root_identity,
            &self.manifest.artifact_root_initial_roster,
            &self.manifest.policy.artifacts,
        )?;
        if let Some(receipt_root) = &self.initial_receipt_root_owner {
            receipt_root.revalidate_for_prepared(&self.manifest)?;
        }
        self.backing
            .revalidate_identity_after_persistence(&self.manifest.backing)?;
        if self.backing.exact_identity_v3()? != self.manifest.backing_exact {
            return Err(invalid(
                "retained prepared backing full component binding changed",
            ));
        }
        validate_prepared_backing_artifact(
            &self.artifact_root,
            &self.manifest.policy,
            &self.manifest.backing_exact,
        )?;
        self.mountpoint.revalidate()?;
        if let PreparedBaselineGuardV3::Captured(baseline) = &self.baseline_guard {
            baseline.revalidate_after_persistence()?;
            if RestartBaselineInventoryV3::from_inventory(baseline.report())?
                != self.manifest.baseline
            {
                return Err(invalid(
                    "held baseline inventory changed after prepared capture",
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn revalidate_receipt_root_against_lifecycle(
        &self,
        lifecycle_records: &[Vec<u8>],
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        self.initial_receipt_root_owner
            .as_ref()
            .ok_or_else(|| invalid("prepared initial receipt-root owner was already transferred"))?
            .revalidate_lifecycle_records(lifecycle_records)
    }

    pub(crate) fn bind_receipt_root_to_lifecycle(
        &mut self,
        lifecycle_records: &[Vec<u8>],
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        self.initial_receipt_root_owner
            .as_mut()
            .ok_or_else(|| invalid("prepared initial receipt-root owner was already transferred"))?
            .bind_lifecycle_records(lifecycle_records)?;
        self.revalidate()
    }

    /// Move the unique G0/reopened receipt-root owner into the whole S2
    /// operation store. This handoff is one-shot; prepared evidence remains
    /// replayable afterwards but can no longer publish or recapture a root.
    pub(crate) fn take_initial_receipt_root_owner(
        &mut self,
    ) -> Result<RetainedCollectorReceiptRootOwnerV3, RestartCollectorErrorV3> {
        self.revalidate()?;
        let owner = self.initial_receipt_root_owner.take().ok_or_else(|| {
            invalid("prepared initial receipt-root owner was already transferred")
        })?;
        owner.revalidate_for_prepared(&self.manifest)?;
        Ok(owner)
    }

    pub(crate) fn collect_reconciliation_from_active(
        &self,
        seed: ActiveRestartCollectorSeedV3<'_>,
        receipt_root_owner: RetainedCollectorReceiptRootOwnerV3,
    ) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3> {
        self.collect_reconciliation_from_active_with_hook(seed, receipt_root_owner, |_| Ok(()))
    }

    pub(crate) fn collect_reconciliation_from_active_with_hook<H>(
        &self,
        seed: ActiveRestartCollectorSeedV3<'_>,
        receipt_root_owner: RetainedCollectorReceiptRootOwnerV3,
        after_iomedia_capture: H,
    ) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3>
    where
        H: FnOnce(&mut HeldRestartIOMediaInventoryV3) -> Result<(), RestartCollectorErrorV3>,
    {
        self.revalidate()?;
        seed.revalidate_for_prepared(
            &self.operation_nonce,
            &self.manifest_sha256,
            &self.manifest.profile_sha256,
        )?;
        let bindings = RestartCollectorBindingsV3 {
            backing_identity_sha256: self.backing_identity_sha256()?,
            baseline_inventory_sha256: self.baseline_inventory_sha256()?,
            boot_session_uuid: seed.boot_session_uuid().to_string(),
            collector_policy_sha256: self.collector_policy_sha256()?,
            mountpoint_underlying_sha256: self.mountpoint_underlying_sha256()?,
            operation_nonce: seed.operation_nonce().to_string(),
            restart_epoch_nonce: seed.restart_epoch_nonce().to_string(),
            restart_started_monotonic_nanoseconds: seed.restart_started_monotonic_nanoseconds(),
        };
        let request = LiveRestartCollectorRequestV3 {
            artifact_root: &self.artifact_root.path,
            baseline: &self.manifest.baseline,
            bindings: &bindings,
            mountpoint_identity: &self.manifest.mountpoint,
            policy: &self.manifest.policy,
            prepared_backing: &self.manifest.backing,
            receipt_root: Path::new(&self.manifest.policy.receipt_root),
        };
        receipt_root_owner.revalidate_for_prepared(&self.manifest)?;
        collect_live_with_root(
            request,
            CollectorPurposeV3::ReconciliationSnapshot,
            None,
            Some(&seed),
            receipt_root_owner,
            after_iomedia_capture,
        )
    }

    /// Collect the exact post-eject reconciliation endpoint.  Unlike the
    /// admission and post-unmount paths this deliberately uses the active
    /// epoch rather than the admission live-before seed: a successful eject
    /// must remove the retained disk-image group, so requiring the original
    /// live inventory here would make the positive transition unreachable.
    pub(crate) fn collect_reconciliation_after_eject(
        &self,
        epoch: ActiveRestartCollectorEpochV3<'_>,
        armed: ArmedEjectExpectationV3,
    ) -> Result<PendingEjectCollectorObservationV3, RestartCollectorErrorV3> {
        self.revalidate()?;
        epoch.revalidate_for_prepared(
            &self.operation_nonce,
            &self.manifest_sha256,
            &self.manifest.profile_sha256,
        )?;
        armed.revalidate_pending()?;
        let ArmedEjectExpectationV3 {
            core,
            receipt_root_owner,
            _not_send_or_sync: _,
        } = armed;
        core.revalidate_with_owner(&receipt_root_owner, EjectInventoryEndpointV3::Pending)?;
        let bindings = RestartCollectorBindingsV3 {
            backing_identity_sha256: self.backing_identity_sha256()?,
            baseline_inventory_sha256: self.baseline_inventory_sha256()?,
            boot_session_uuid: epoch.boot_session_uuid().to_string(),
            collector_policy_sha256: self.collector_policy_sha256()?,
            mountpoint_underlying_sha256: self.mountpoint_underlying_sha256()?,
            operation_nonce: epoch.operation_nonce().to_string(),
            restart_epoch_nonce: epoch.restart_epoch_nonce().to_string(),
            restart_started_monotonic_nanoseconds: epoch.restart_started_monotonic_nanoseconds(),
        };
        receipt_root_owner.revalidate_for_prepared(&self.manifest)?;
        let pending = collect_live_with_root(
            LiveRestartCollectorRequestV3 {
                artifact_root: &self.artifact_root.path,
                baseline: &self.manifest.baseline,
                bindings: &bindings,
                mountpoint_identity: &self.manifest.mountpoint,
                policy: &self.manifest.policy,
                prepared_backing: &self.manifest.backing,
                receipt_root: Path::new(&self.manifest.policy.receipt_root),
            },
            CollectorPurposeV3::ReconciliationSnapshot,
            None,
            None,
            receipt_root_owner,
            |_| Ok(()),
        )?;
        epoch.revalidate_for_prepared(
            &self.operation_nonce,
            &self.manifest_sha256,
            &self.manifest.profile_sha256,
        )?;
        core.validate_pending_observation(&pending)?;
        Ok(PendingEjectCollectorObservationV3 {
            expectation: core,
            pending,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn collect_fresh_absence_from_active(
        &self,
        epoch: ActiveRestartCollectorEpochV3<'_>,
        snapshot: &ReconciliationSnapshotV2,
        receipt_root_owner: RetainedCollectorReceiptRootOwnerV3,
    ) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3> {
        self.revalidate()?;
        epoch.revalidate_for_prepared(
            &self.operation_nonce,
            &self.manifest_sha256,
            &self.manifest.profile_sha256,
        )?;
        let bindings = RestartCollectorBindingsV3 {
            backing_identity_sha256: self.backing_identity_sha256()?,
            baseline_inventory_sha256: self.baseline_inventory_sha256()?,
            boot_session_uuid: epoch.boot_session_uuid().to_string(),
            collector_policy_sha256: self.collector_policy_sha256()?,
            mountpoint_underlying_sha256: self.mountpoint_underlying_sha256()?,
            operation_nonce: epoch.operation_nonce().to_string(),
            restart_epoch_nonce: epoch.restart_epoch_nonce().to_string(),
            restart_started_monotonic_nanoseconds: epoch.restart_started_monotonic_nanoseconds(),
        };
        validate_reconciliation_snapshot_shape_v3(snapshot)?;
        if snapshot.current_expected_absence_inventory_sha256.is_none()
            || snapshot.operation_nonce != bindings.operation_nonce
            || snapshot.restart_epoch_nonce != bindings.restart_epoch_nonce
            || snapshot.boot_session_uuid != bindings.boot_session_uuid
            || snapshot.collector_policy_sha256 != bindings.collector_policy_sha256
            || snapshot.backing_identity_sha256 != bindings.backing_identity_sha256
            || snapshot.mountpoint_underlying_sha256 != bindings.mountpoint_underlying_sha256
        {
            return Err(invalid(
                "FreshAbsence retained predictable snapshot differs from the active admitted epoch",
            ));
        }
        let snapshot_sha = reconciliation_snapshot_sha256(snapshot)
            .map_err(|error| invalid(format!("reconciliation snapshot digest failed: {error}")))?;
        receipt_root_owner.revalidate_for_prepared(&self.manifest)?;
        collect_live_with_root(
            LiveRestartCollectorRequestV3 {
                artifact_root: &self.artifact_root.path,
                baseline: &self.manifest.baseline,
                bindings: &bindings,
                mountpoint_identity: &self.manifest.mountpoint,
                policy: &self.manifest.policy,
                prepared_backing: &self.manifest.backing,
                receipt_root: Path::new(&self.manifest.policy.receipt_root),
            },
            CollectorPurposeV3::FreshAbsence,
            Some((snapshot, snapshot_sha)),
            None,
            receipt_root_owner,
            |_| Ok(()),
        )
    }
}

pub(crate) fn lifecycle_manifest_initial_receipt_root_binding(
    manifest_bytes: &[u8],
    _seal: &PreparedCollectorLifecycleSealV3,
) -> Result<Option<FilesystemObjectBindingV3>, RestartCollectorErrorV3> {
    prepared_manifest_initial_receipt_root_binding(manifest_bytes)
}

pub(crate) fn s1_manifest_initial_receipt_root_binding(
    manifest_bytes: &[u8],
    _seal: &S1PreparedManifestReadSealV3,
) -> Result<Option<FilesystemObjectBindingV3>, RestartCollectorErrorV3> {
    prepared_manifest_initial_receipt_root_binding(manifest_bytes)
}

fn prepared_manifest_initial_receipt_root_binding(
    manifest_bytes: &[u8],
) -> Result<Option<FilesystemObjectBindingV3>, RestartCollectorErrorV3> {
    let projection: serde_json::Value = serde_json::from_slice(manifest_bytes)
        .map_err(|error| invalid(format!("prepared collector manifest JSON failed: {error}")))?;
    let object = projection
        .as_object()
        .ok_or_else(|| invalid("prepared collector manifest JSON is not an object"))?;
    if !object.contains_key("receipt_root_initial_binding") {
        // Historical prepared sidecars predate the external receipt-root
        // generation contract. They remain exact opaque lifecycle evidence,
        // but can never mint a live root owner or enter the new forward path.
        return Ok(None);
    }
    let manifest: PreparedCollectorManifestV3 = serde_json::from_slice(manifest_bytes)
        .map_err(|error| invalid(format!("prepared collector manifest JSON failed: {error}")))?;
    validate_prepared_manifest(&manifest)?;
    manifest
        .receipt_root_initial_binding
        .map(Some)
        .ok_or_else(|| invalid("prepared collector manifest explicitly nulls its receipt root"))
}

fn prepared_backing_profile(
    basename: &str,
) -> Result<Vec<PreparedArtifactBindingV3>, RestartCollectorErrorV3> {
    let artifacts = vec![PreparedArtifactBindingV3 {
        basename: validate_child_name(basename)?.to_string(),
        role: ArtifactRoleV3::BackingImage,
    }];
    validate_artifact_bindings(&artifacts, true)?;
    Ok(artifacts)
}

fn prepared_profile_sha256(
    artifacts: &[PreparedArtifactBindingV3],
) -> Result<String, RestartCollectorErrorV3> {
    validate_artifact_bindings(artifacts, true)?;
    Ok(sha256(&canonical_json(&PreparedCollectorProfileV3 {
        artifacts,
        schema: PREPARED_COLLECTOR_PROFILE_SCHEMA,
    })?))
}

fn validate_prepared_backing_identity(
    identity: &ExactDiskImageBackingIdentityV3,
) -> Result<(), RestartCollectorErrorV3> {
    if identity.schema != "hepta_mac_exact_disk_image_backing_identity_v3"
        || identity.authority_granted
        || !Path::new(&identity.canonical_path).is_absolute()
        || identity.opened_components.is_empty()
        || !valid_digest(&identity.content_sha256)
        || identity.opened_components.last().is_none_or(|component| {
            component.directory || component.path != identity.canonical_path
        })
    {
        return Err(invalid("exact prepared backing identity is malformed"));
    }
    for (index, component) in identity.opened_components.iter().enumerate() {
        if !Path::new(&component.path).is_absolute()
            || component.directory != (index + 1 != identity.opened_components.len())
        {
            return Err(invalid(
                "exact prepared backing component path or kind is malformed",
            ));
        }
        validate_filesystem_binding_shape(
            &component.binding,
            component.directory,
            "exact prepared backing component",
        )?;
    }
    Ok(())
}

fn canonical_prepared_manifest(
    manifest: &PreparedCollectorManifestV3,
) -> Result<(Vec<u8>, String), RestartCollectorErrorV3> {
    validate_prepared_manifest(manifest)?;
    canonical_prepared_manifest_bytes(manifest)
}

fn canonical_prepared_manifest_bytes(
    manifest: &PreparedCollectorManifestV3,
) -> Result<(Vec<u8>, String), RestartCollectorErrorV3> {
    let bytes = canonical_json(manifest)?;
    if bytes.is_empty() || bytes.len() > MAX_RECEIPT_BYTES {
        return Err(invalid(
            "canonical prepared collector manifest exceeds its bound",
        ));
    }
    let digest = sha256(&bytes);
    Ok((bytes, digest))
}

fn validate_prepared_manifest(
    manifest: &PreparedCollectorManifestV3,
) -> Result<(), RestartCollectorErrorV3> {
    let expected_artifact_roster = manifest
        .policy
        .artifacts
        .iter()
        .map(|artifact| artifact.basename.clone())
        .collect::<Vec<_>>();
    let expected_backing_path = Path::new(&manifest.policy.artifact_root)
        .join(
            manifest
                .policy
                .artifacts
                .first()
                .map(|artifact| artifact.basename.as_str())
                .unwrap_or_default(),
        )
        .to_str()
        .unwrap_or_default()
        .to_string();
    if manifest.schema != PREPARED_COLLECTOR_MANIFEST_SCHEMA
        || manifest.schema_version != 3
        || manifest.authority.any()
        || !valid_nonce(&manifest.operation_nonce)
        || !valid_uuid(&manifest.prepared_boot_session_uuid)
        || manifest.prepared_boot_session_uuid != manifest.baseline.boot_session_uuid
        || manifest.profile_sha256 != prepared_profile_sha256(&manifest.policy.artifacts)?
        || manifest.backing.canonical_path != manifest.policy.backing_path
        || manifest.backing_exact.canonical_path != manifest.policy.backing_path
        || manifest.backing_exact.canonical_path != manifest.backing.canonical_path
        || manifest.mountpoint.path != manifest.policy.mountpoint
        || !manifest.receipt_root_initial_roster.is_empty()
        || manifest
            .receipt_root_initial_binding
            .is_some_and(|binding| {
                validate_receipt_directory(&binding).is_err()
                    || !manifest
                        .policy
                        .receipt_root_identity
                        .matches_binding(&binding, 0)
            })
        || manifest.artifact_root_initial_roster != expected_artifact_roster
        || manifest.policy.backing_path != expected_backing_path
    {
        return Err(invalid(
            "prepared collector manifest is malformed or grants authority",
        ));
    }
    validate_policy(&manifest.policy)?;
    validate_baseline(&manifest.baseline)?;
    validate_disk_image_backing_identity_v2(&manifest.backing)?;
    validate_prepared_backing_identity(&manifest.backing_exact)?;
    if manifest.backing_exact.content_sha256
        != manifest
            .backing
            .opened_components
            .last()
            .and_then(|component| component.fd_binding.content_sha256.as_deref())
            .unwrap_or_default()
    {
        return Err(invalid(
            "exact prepared backing content digest differs from its IOMedia projection",
        ));
    }
    if manifest.backing.opened_components.len() != manifest.backing_exact.opened_components.len()
        || manifest
            .backing
            .opened_components
            .iter()
            .zip(&manifest.backing_exact.opened_components)
            .any(|(legacy, exact)| {
                let binding = &exact.binding;
                legacy.directory != exact.directory
                    || legacy.path != exact.path
                    || legacy.fd_binding.ctime_nanoseconds != binding.ctime_nanoseconds
                    || legacy.fd_binding.ctime_seconds != binding.ctime_seconds
                    || legacy.fd_binding.dev != binding.dev
                    || legacy.fd_binding.flags != binding.flags
                    || legacy.fd_binding.gid != binding.gid
                    || legacy.fd_binding.inode != binding.inode
                    || legacy.fd_binding.mode != binding.mode
                    || legacy.fd_binding.mtime_nanoseconds != binding.mtime_nanoseconds
                    || legacy.fd_binding.mtime_seconds != binding.mtime_seconds
                    || legacy.fd_binding.nlink != binding.nlink
                    || legacy.fd_binding.size != binding.size
                    || legacy.fd_binding.uid != binding.uid
            })
    {
        return Err(invalid(
            "exact prepared backing components differ from their V2 matching projection",
        ));
    }
    validate_mountpoint_identity(&manifest.mountpoint)?;
    Ok(())
}

fn validate_prepared_artifact_root(
    root: &HeldDirectoryV3,
    expected: &StableDirectoryIdentityV3,
    initial_roster: &[String],
    artifacts: &[PreparedArtifactBindingV3],
) -> Result<(), RestartCollectorErrorV3> {
    root.revalidate("artifact root")?;
    let roster = list_directory(root.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES)?;
    if !expected.matches_binding(&root.binding, roster.len()) {
        return Err(invalid(
            "retained prepared artifact root differs from its exact stable identity",
        ));
    }
    let required = artifacts
        .iter()
        .map(|artifact| artifact.basename.clone())
        .collect::<Vec<_>>();
    if initial_roster != required || roster != required {
        return Err(invalid(
            "retained prepared artifact root contains an unprepared roster delta",
        ));
    }
    Ok(())
}

fn validate_prepared_backing_artifact(
    root: &HeldDirectoryV3,
    policy: &RestartCollectorPolicyV3,
    backing: &ExactDiskImageBackingIdentityV3,
) -> Result<(), RestartCollectorErrorV3> {
    let basename = policy
        .artifacts
        .iter()
        .find(|artifact| artifact.role == ArtifactRoleV3::BackingImage)
        .ok_or_else(|| invalid("prepared profile lacks its required BackingImage role"))?;
    let final_binding = backing
        .opened_components
        .last()
        .ok_or_else(|| invalid("exact prepared backing has no terminal component"))?
        .binding;
    if fstatat_binding(
        root.file.as_raw_fd(),
        &basename.basename,
        "prepared BackingImage pathname",
    )? != final_binding
        || Path::new(&policy.artifact_root).join(&basename.basename)
            != Path::new(&backing.canonical_path)
    {
        return Err(invalid(
            "prepared BackingImage role does not name the exact held backing inode",
        ));
    }
    Ok(())
}

fn validate_prepared_receipt_root(
    root: &HeldDirectoryV3,
    expected: &StableDirectoryIdentityV3,
) -> Result<(), RestartCollectorErrorV3> {
    root.revalidate("receipt root")?;
    let roster = list_directory(root.file.as_raw_fd(), MAX_RECEIPT_FILES)?;
    if !expected.matches_binding(&root.binding, roster.len()) {
        return Err(invalid(
            "retained prepared receipt root differs from its exact stable identity",
        ));
    }
    capture_receipt_root_closed_world(root)?;
    Ok(())
}

fn prepared_backing_candidate_matches(
    candidate: &DiskImageBackingIdentityV2,
    prepared: &DiskImageBackingIdentityV2,
) -> Result<bool, RestartCollectorErrorV3> {
    validate_disk_image_backing_identity_v2(candidate)?;
    validate_disk_image_backing_identity_v2(prepared)?;
    if candidate.canonical_path != prepared.canonical_path
        || candidate.opened_components.len() != prepared.opened_components.len()
    {
        return Ok(false);
    }
    let last_index = candidate.opened_components.len().saturating_sub(1);
    for (index, (candidate, prepared)) in candidate
        .opened_components
        .iter()
        .zip(&prepared.opened_components)
        .enumerate()
    {
        if candidate.directory != prepared.directory
            || candidate.path != prepared.path
            || candidate.fd_binding.dev != prepared.fd_binding.dev
            || candidate.fd_binding.inode != prepared.fd_binding.inode
            || candidate.fd_binding.mode != prepared.fd_binding.mode
            || candidate.fd_binding.uid != prepared.fd_binding.uid
            || candidate.fd_binding.gid != prepared.fd_binding.gid
            || candidate.fd_binding.flags != prepared.fd_binding.flags
            || candidate.fd_binding.nlink != prepared.fd_binding.nlink
            || (index == last_index && candidate.fd_binding != prepared.fd_binding)
        {
            return Ok(false);
        }
    }
    Ok(true)
}

fn reopen_prepared_mountpoint(
    expected: &MountpointIdentityV3,
) -> Result<UnderlyingMountpointGuardV3, RestartCollectorErrorV3> {
    let mounts = mount_table_snapshot()?;
    if mounts.iter().any(|mount| mount.mount_on == expected.path) {
        UnderlyingMountpointGuardV3::capture_deferred(expected)
    } else {
        let held = HeldDirectoryV3::capture(Path::new(&expected.path), "mountpoint")?;
        if mountpoint_identity_from_held(&held)? != *expected {
            return Err(invalid(
                "live mountpoint differs from the exact durable prepared identity",
            ));
        }
        Ok(UnderlyingMountpointGuardV3::Held(held))
    }
}

impl RestartBaselineInventoryV3 {
    pub(crate) fn from_inventory(
        inventory: &RestartIOMediaInventoryV3,
    ) -> Result<Self, RestartCollectorErrorV3> {
        validate_restart_iomedia_inventory_v3(inventory)?;
        Ok(Self {
            authority: DisposableAuthorityV2::none(),
            boot_session_uuid: inventory.boot_session_uuid.clone(),
            registry_entry_ids: inventory
                .objects
                .iter()
                .map(|object| object.provenance.registry_entry_id.clone())
                .collect(),
            schema: BASELINE_SCHEMA.to_string(),
        })
    }

    pub(crate) fn sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        validate_baseline(self)?;
        Ok(sha256(&canonical_json(self)?))
    }

    pub(crate) fn boot_session_uuid(&self) -> &str {
        &self.boot_session_uuid
    }
}

impl MountpointIdentityV3 {
    pub fn capture(path: &Path) -> Result<Self, RestartCollectorErrorV3> {
        let held = HeldDirectoryV3::capture(path, "mountpoint")?;
        let identity = Self {
            authority: DisposableAuthorityV2::none(),
            binding: held.binding,
            path: path_text(&held.path, "mountpoint")?,
            schema: MOUNTPOINT_SCHEMA.to_string(),
        };
        validate_mountpoint_identity(&identity)?;
        Ok(identity)
    }

    pub fn sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        validate_mountpoint_identity(self)?;
        Ok(sha256(&canonical_json(self)?))
    }
}

#[cfg(test)]
fn capture_live_restart_baseline_v3() -> Result<RestartBaselineInventoryV3, RestartCollectorErrorV3>
{
    let held = capture_restart_iomedia_inventory_v3()?;
    let baseline = RestartBaselineInventoryV3::from_inventory(held.report())?;
    held.revalidate_after_persistence()?;
    Ok(baseline)
}

#[cfg(test)]
fn capture_live_backing_identity_v2(
    path: &Path,
) -> Result<DiskImageBackingIdentityV2, RestartCollectorErrorV3> {
    let held = hold_disk_image_backing(path)?;
    let identity = held.identity()?;
    held.revalidate_identity_after_persistence(&identity)?;
    Ok(identity)
}

#[cfg(test)]
fn collect_reconciliation_snapshot_v3(
    request: LiveRestartCollectorRequestV3<'_>,
) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3> {
    collect_live(
        request,
        CollectorPurposeV3::ReconciliationSnapshot,
        None,
        None,
        |_| Ok(()),
    )
}

#[cfg(test)]
fn collect_fresh_absence_v3(
    request: LiveRestartCollectorRequestV3<'_>,
    snapshot: &ReconciliationSnapshotV2,
) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3> {
    validate_reconciliation_snapshot_shape_v3(snapshot)?;
    if snapshot.current_expected_absence_inventory_sha256.is_none()
        || snapshot.operation_nonce != request.bindings.operation_nonce
        || snapshot.restart_epoch_nonce != request.bindings.restart_epoch_nonce
        || snapshot.boot_session_uuid != request.bindings.boot_session_uuid
        || snapshot.collector_policy_sha256 != request.bindings.collector_policy_sha256
        || snapshot.backing_identity_sha256 != request.bindings.backing_identity_sha256
        || snapshot.mountpoint_underlying_sha256 != request.bindings.mountpoint_underlying_sha256
    {
        return Err(invalid(
            "fresh absence requires one exact current-epoch predictable reconciliation snapshot",
        ));
    }
    let snapshot_sha = reconciliation_snapshot_sha256(snapshot)
        .map_err(|error| invalid(format!("reconciliation snapshot digest failed: {error}")))?;
    collect_live(
        request,
        CollectorPurposeV3::FreshAbsence,
        Some((snapshot, snapshot_sha)),
        None,
        |_| Ok(()),
    )
}

#[cfg(test)]
fn collect_live<H>(
    request: LiveRestartCollectorRequestV3<'_>,
    purpose: CollectorPurposeV3,
    prior_snapshot: Option<(&ReconciliationSnapshotV2, String)>,
    active_seed: Option<&ActiveRestartCollectorSeedV3<'_>>,
    after_iomedia_capture: H,
) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3>
where
    H: FnOnce(&mut HeldRestartIOMediaInventoryV3) -> Result<(), RestartCollectorErrorV3>,
{
    // Preserve the rootless test harness's original fail-closed ordering: a
    // same-path replacement is first rejected against the prepared policy.
    // Production collection enters through a retained prepared capability and
    // never uses this test-only DTO path.
    validate_request(&request)?;
    let receipt_root_held = HeldDirectoryV3::capture(request.receipt_root, "receipt root")?;
    let receipt_root_initial = receipt_root_held.binding;
    let receipt_root =
        RetainedCollectorReceiptRootOwnerV3::from_root(RetainedReceiptRootV3::from_held(
            receipt_root_held,
            request.policy.receipt_root_identity,
            receipt_root_initial,
        )?);
    collect_live_with_root(
        request,
        purpose,
        prior_snapshot,
        active_seed,
        receipt_root,
        after_iomedia_capture,
    )
}

fn collect_live_with_root<H>(
    request: LiveRestartCollectorRequestV3<'_>,
    purpose: CollectorPurposeV3,
    prior_snapshot: Option<(&ReconciliationSnapshotV2, String)>,
    active_seed: Option<&ActiveRestartCollectorSeedV3<'_>>,
    receipt_root_owner: RetainedCollectorReceiptRootOwnerV3,
    after_iomedia_capture: H,
) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3>
where
    H: FnOnce(&mut HeldRestartIOMediaInventoryV3) -> Result<(), RestartCollectorErrorV3>,
{
    validate_request(&request)?;
    if let Some(seed) = active_seed {
        seed.revalidate_live()?;
    }
    // Hold both prepared roots from the first live-collection boundary through
    // durable persistence and final replay. Stable policy identities reject a
    // same-path replacement that occurred after preparation, while the full
    // bindings below reject any later metadata or roster churn.
    let artifact_root = HeldDirectoryV3::capture(request.artifact_root, "artifact root")?;
    let artifact_root_roster =
        list_directory(artifact_root.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES)?;
    artifact_root.revalidate("artifact root")?;
    if !request
        .policy
        .artifact_root_identity
        .matches_binding(&artifact_root.binding, artifact_root_roster.len())
        || receipt_root_owner.root.stable_identity != request.policy.receipt_root_identity
        || receipt_root_owner.root.path != request.receipt_root
    {
        return Err(invalid(
            "live collector roots differ from their prepared stable identities",
        ));
    }
    receipt_root_owner.revalidate()?;
    let before = monotonic_nanoseconds()?;
    if before <= request.bindings.restart_started_monotonic_nanoseconds
        || prior_snapshot
            .as_ref()
            .is_some_and(|(snapshot, _)| before <= snapshot.monotonic_after_nanoseconds)
    {
        return Err(invalid(
            "restart collector window is not later than its bound epoch or snapshot",
        ));
    }
    let mounts_before = mount_table_snapshot()?;
    reject_nested_mounts(&mounts_before, request.policy)?;

    let mut iomedia = capture_restart_iomedia_inventory_v3()?;
    after_iomedia_capture(&mut iomedia)?;
    if iomedia.report().boot_session_uuid != request.bindings.boot_session_uuid {
        return Err(invalid("restart IOMedia inventory belongs to another boot"));
    }
    if let Some(seed) = active_seed {
        seed.require_exact_live_inventory(iomedia.report())?;
    }
    let matching_groups = classify_matching_groups(iomedia.report(), request.prepared_backing)?;
    let (match_result, mountpoint_is_mounted) =
        classify_mount_state(&matching_groups, &mounts_before, request.policy)?;

    let backing = hold_disk_image_backing(Path::new(&request.policy.backing_path))?;
    if backing.identity()? != *request.prepared_backing {
        return Err(invalid(
            "restart backing descriptor differs from the prepared backing identity",
        ));
    }

    let mountpoint = if mountpoint_is_mounted {
        UnderlyingMountpointGuardV3::capture_deferred(request.mountpoint_identity)?
    } else {
        let held = HeldDirectoryV3::capture(Path::new(&request.policy.mountpoint), "mountpoint")?;
        if mountpoint_identity_from_held(&held)? != *request.mountpoint_identity {
            return Err(invalid(
                "restart mountpoint differs from the prepared underlying identity",
            ));
        }
        UnderlyingMountpointGuardV3::Held(held)
    };

    let prior_expected_absence = if let Some((snapshot, _)) = prior_snapshot.as_ref() {
        Some(validate_prior_snapshot_receipt(
            snapshot,
            &request.bindings.baseline_inventory_sha256,
            &receipt_root_owner.root.snapshot,
        )?)
    } else {
        None
    };
    let artifact_roster = list_directory(artifact_root.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES)?;
    let operation_artifacts_absent = request
        .policy
        .artifacts
        .iter()
        .all(|artifact| artifact_roster.binary_search(&artifact.basename).is_err());
    let artifact_evidence = ArtifactEvidenceV3 {
        artifacts: request.policy.artifacts.clone(),
        artifact_root: request.policy.artifact_root.clone(),
        authority: DisposableAuthorityV2::none(),
        operation_artifacts_absent,
        root_binding: artifact_root.binding,
        roster: artifact_roster,
        schema: ARTIFACT_SCHEMA.to_string(),
    };

    let mounts_after = mount_table_snapshot()?;
    reject_nested_mounts(&mounts_after, request.policy)?;
    if mounts_after != mounts_before {
        return Err(invalid(
            "bounded mount table changed during restart collection",
        ));
    }
    let after = monotonic_nanoseconds()?;
    if after < before || current_boot_session_uuid()? != request.bindings.boot_session_uuid {
        return Err(invalid(
            "restart collector changed boot or monotonic window",
        ));
    }

    let current_baseline = RestartBaselineInventoryV3::from_inventory(iomedia.report())?;
    let post_inventory_sha256 = current_baseline.sha256()?;
    let current_expected_absence_inventory =
        derive_current_expected_absence_v3(iomedia.report(), &match_result, &matching_groups)?;
    let current_expected_absence_inventory_sha256 = current_expected_absence_inventory
        .as_ref()
        .map(|inventory| canonical_json(inventory).map(|bytes| sha256(&bytes)))
        .transpose()?;
    let baseline_restored = match purpose {
        CollectorPurposeV3::ReconciliationSnapshot => current_expected_absence_inventory
            .as_ref()
            .is_some_and(|expected| expected == iomedia.report()),
        CollectorPurposeV3::FreshAbsence => prior_expected_absence
            .as_ref()
            .is_some_and(|expected| expected == iomedia.report()),
    };
    let mount_evidence = MountEvidenceV3 {
        authority: DisposableAuthorityV2::none(),
        mountpoint_underlying_revalidated: matches!(
            &mountpoint,
            UnderlyingMountpointGuardV3::Held(_)
        ),
        mounts_after: mounts_after.clone(),
        mounts_before: mounts_before.clone(),
        no_nested_mounts: true,
        schema: MOUNT_SCHEMA.to_string(),
    };
    let artifact_evidence_sha256 = sha256(&canonical_json(&artifact_evidence)?);
    let iomedia_evidence_sha256 = sha256(&canonical_json(iomedia.report())?);
    let mount_evidence_sha256 = sha256(&canonical_json(&mount_evidence)?);
    let receipt = RestartCollectorReceiptV3 {
        artifact_evidence,
        artifact_evidence_sha256,
        authority: DisposableAuthorityV2::none(),
        backing_identity: request.prepared_backing.clone(),
        backing_identity_sha256: request.bindings.backing_identity_sha256.clone(),
        baseline_inventory: request.baseline.clone(),
        baseline_inventory_sha256: request.bindings.baseline_inventory_sha256.clone(),
        baseline_restored,
        boot_session_uuid: request.bindings.boot_session_uuid.clone(),
        collector_policy: request.policy.clone(),
        collector_policy_sha256: request.bindings.collector_policy_sha256.clone(),
        current_expected_absence_inventory,
        current_expected_absence_inventory_sha256,
        iomedia_evidence_sha256,
        iomedia_inventory: iomedia.report().clone(),
        match_result,
        matching_groups,
        monotonic_after_nanoseconds: after,
        monotonic_before_nanoseconds: before,
        mount_evidence,
        mount_evidence_sha256,
        mountpoint_underlying: request.mountpoint_identity.clone(),
        mountpoint_underlying_sha256: request.bindings.mountpoint_underlying_sha256.clone(),
        operation_artifacts_absent,
        operation_nonce: request.bindings.operation_nonce.clone(),
        post_inventory_sha256,
        purpose,
        reconciliation_snapshot_sha256: prior_snapshot.map(|(_, digest)| digest),
        restart_epoch_nonce: request.bindings.restart_epoch_nonce.clone(),
        schema: RECEIPT_SCHEMA.to_string(),
        schema_version: 3,
    };
    validate_receipt(&receipt)?;
    if purpose == CollectorPurposeV3::FreshAbsence
        && (receipt.match_result != ReconciliationMatchV2::Zero
            || !receipt.baseline_restored
            || !receipt.operation_artifacts_absent
            || !receipt.mount_evidence.no_nested_mounts
            || !receipt.mount_evidence.mountpoint_underlying_revalidated)
    {
        return Err(invalid(
            "FreshAbsence collection did not observe exact Zero baseline restoration",
        ));
    }
    if let Some(seed) = active_seed {
        seed.revalidate_live()?;
        seed.require_exact_live_inventory(iomedia.report())?;
    }
    Ok(PendingRestartObservationV3 {
        guard: LiveReplayGuardV3 {
            artifact_evidence: receipt.artifact_evidence.clone(),
            artifact_root,
            backing,
            iomedia,
            mountpoint,
            mounts: mounts_after,
            prepared_backing: request.prepared_backing.clone(),
        },
        receipt,
        receipt_root_owner,
    })
}

impl PendingRestartObservationV3 {
    #[cfg(test)]
    fn receipt(&self) -> &RestartCollectorReceiptV3 {
        &self.receipt
    }

    #[cfg(test)]
    pub(crate) fn persist_and_retain(
        self,
    ) -> Result<UnadoptedCollectorGenerationV3, RestartCollectorErrorV3> {
        self.persist_and_retain_inner(|| Ok(()))
    }

    pub(crate) fn persist_for_unmount_delta(
        self,
        delta: &RetainedCollectorMountDeltaV3<UnmountingV3>,
    ) -> Result<UnadoptedCollectorGenerationV3, RestartCollectorErrorV3> {
        let unadopted = self.persist_and_retain_inner(|| Ok(()))?;
        delta.validate_unadopted_observation(&unadopted, MountDeltaDirectionV3::Unmount)?;
        Ok(unadopted)
    }

    pub(crate) fn persist_and_append(
        self,
        operation: &mut ReconciliationOperationStoreV3<'_, '_, ActiveRestartEpochV3>,
    ) -> Result<(), RestartCollectorErrorV3> {
        operation.arm_collector_persistence().map_err(|error| {
            invalid(format!(
                "could not arm the collector persistence transaction: {error}"
            ))
        })?;
        let unadopted = match self.persist_and_retain_inner(|| Ok(())) {
            Ok(unadopted) => unadopted,
            Err(error) => {
                return Err(error);
            }
        };
        operation
            .append_unadopted_collector_armed(unadopted)
            .map_err(|error| {
                invalid(format!(
                    "durable lifecycle append rejected unadopted collector generation: {error}"
                ))
            })?;
        Ok(())
    }

    #[cfg(test)]
    fn persist_and_retain_with_hook<F>(
        self,
        after_persistence: F,
    ) -> Result<UnadoptedCollectorGenerationV3, RestartCollectorErrorV3>
    where
        F: FnOnce() -> Result<(), RestartCollectorErrorV3>,
    {
        self.persist_and_retain_inner(after_persistence)
    }

    fn persist_and_retain_inner<F>(
        self,
        after_persistence: F,
    ) -> Result<UnadoptedCollectorGenerationV3, RestartCollectorErrorV3>
    where
        F: FnOnce() -> Result<(), RestartCollectorErrorV3>,
    {
        validate_receipt(&self.receipt)?;
        let bytes = canonical_json(&self.receipt)?;
        if bytes.len() > MAX_RECEIPT_BYTES {
            return Err(invalid(
                "canonical restart collector receipt exceeds its bound",
            ));
        }
        let receipt_sha256 = sha256(&bytes);
        self.receipt_root_owner.revalidate()?;
        let before_root_binding = self.receipt_root_owner.root.current_binding;
        let RetainedCollectorReceiptRootOwnerV3 {
            root: receipt_root,
            _not_send_or_sync: _,
        } = self.receipt_root_owner;
        let (durable, receipt_root) = DurableCollectorReceiptV3::persist(
            receipt_root,
            &self.receipt,
            bytes.clone(),
            &receipt_sha256,
        )?;
        let receipt_root_owner = RetainedCollectorReceiptRootOwnerV3::from_root(receipt_root);
        after_persistence()?;

        // No typed lifecycle observation exists before every held descriptor
        // and the complete mount table have survived this post-persistence
        // replay.
        self.guard.revalidate(&self.receipt)?;
        let expected_lifecycle_binding = durable.lifecycle_binding();
        durable.revalidate_unadopted(&receipt_root_owner.root, &expected_lifecycle_binding)?;
        let decoded: RestartCollectorReceiptV3 = serde_json::from_slice(&bytes)
            .map_err(|error| invalid(format!("persisted receipt JSON failed: {error}")))?;
        if decoded != self.receipt || canonical_json(&decoded)? != bytes {
            return Err(invalid(
                "canonical restart collector receipt failed final replay",
            ));
        }

        let purpose = self.receipt.purpose;
        let match_result = self.receipt.match_result;
        let receipt_file = durable.lifecycle_binding();
        let observation = match purpose {
            CollectorPurposeV3::ReconciliationSnapshot => {
                FinalizedRestartObservationV3::ReconciliationSnapshot(
                    reconciliation_snapshot_from_receipt(
                        &self.receipt,
                        &receipt_sha256,
                        receipt_file.clone(),
                    )?,
                )
            }
            CollectorPurposeV3::FreshAbsence => {
                if self.receipt.match_result != ReconciliationMatchV2::Zero
                    || !self.receipt.baseline_restored
                    || !self.receipt.operation_artifacts_absent
                    || !self.receipt.mount_evidence.no_nested_mounts
                    || !self
                        .receipt
                        .mount_evidence
                        .mountpoint_underlying_revalidated
                    || self.receipt.reconciliation_snapshot_sha256.is_none()
                {
                    return Err(invalid(
                        "FreshAbsence requires Zero, exact baseline, no mount, and absent artifacts",
                    ));
                }
                FinalizedRestartObservationV3::FreshAbsence(fresh_absence_from_receipt(
                    &self.receipt,
                    &receipt_sha256,
                    receipt_file,
                )?)
            }
        };
        let core = UnadoptedCollectorGenerationCoreV3 {
            before_root_binding,
            durable,
            guard: self.guard,
            expected_lifecycle_binding,
            observation,
            receipt: self.receipt,
            receipt_root_owner,
            receipt_sha256,
            _not_send_or_sync: PhantomData,
        };
        let unadopted = UnadoptedCollectorGenerationV3 { core };
        unadopted.revalidate()?;
        match (purpose, match_result) {
            (CollectorPurposeV3::FreshAbsence, ReconciliationMatchV2::Zero)
            | (CollectorPurposeV3::ReconciliationSnapshot, _) => Ok(unadopted),
            (CollectorPurposeV3::FreshAbsence, _) => Err(invalid(
                "FreshAbsence unadopted generation must have an exact Zero match",
            )),
        }
    }
}

impl PendingEjectCollectorObservationV3 {
    pub(crate) fn persist_and_retain(
        self,
    ) -> Result<UnadoptedEjectObservationV3, RestartCollectorErrorV3> {
        self.expectation
            .validate_pending_observation(&self.pending)?;
        let generation = self.pending.persist_and_retain_inner(|| Ok(()))?;
        self.expectation.validate_unadopted(&generation)?;
        Ok(UnadoptedEjectObservationV3 {
            expectation: self.expectation,
            generation,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl UnadoptedCollectorGenerationCoreV3 {
    fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.guard.revalidate(&self.receipt)?;
        self.durable.revalidate_unadopted(
            &self.receipt_root_owner.root,
            &self.expected_lifecycle_binding,
        )?;
        if self.before_root_binding == self.receipt_root_owner.root.current_binding
            || !same_directory_object(
                self.before_root_binding,
                self.receipt_root_owner.root.current_binding,
            )
            || self.before_root_binding.nlink.checked_add(1)
                != Some(self.receipt_root_owner.root.current_binding.nlink)
            || self.expected_lifecycle_binding != self.durable.lifecycle_binding()
            || sha256(&canonical_json(&self.receipt)?) != self.receipt_sha256
        {
            return Err(invalid(
                "unadopted collector generation changed its exact G -> G+1 binding",
            ));
        }
        let expected = match self.receipt.purpose {
            CollectorPurposeV3::ReconciliationSnapshot => {
                FinalizedRestartObservationV3::ReconciliationSnapshot(
                    reconciliation_snapshot_from_receipt(
                        &self.receipt,
                        &self.receipt_sha256,
                        self.expected_lifecycle_binding.clone(),
                    )?,
                )
            }
            CollectorPurposeV3::FreshAbsence => {
                FinalizedRestartObservationV3::FreshAbsence(fresh_absence_from_receipt(
                    &self.receipt,
                    &self.receipt_sha256,
                    self.expected_lifecycle_binding.clone(),
                )?)
            }
        };
        if self.observation != expected {
            return Err(invalid(
                "unadopted collector projection differs from its durable receipt",
            ));
        }
        Ok(())
    }

    fn append_material(&self) -> Result<UnadoptedCollectorAppendV3<'_>, RestartCollectorErrorV3> {
        self.revalidate()?;
        let event = match &self.observation {
            FinalizedRestartObservationV3::ReconciliationSnapshot(snapshot) => {
                RetainedCollectorAppendEventV3::ReconciliationSnapshot(snapshot)
            }
            FinalizedRestartObservationV3::FreshAbsence(observation) => {
                RetainedCollectorAppendEventV3::FreshAbsence(observation)
            }
        };
        Ok(UnadoptedCollectorAppendV3 {
            event,
            operation_nonce: &self.receipt.operation_nonce,
            _not_send_or_sync: PhantomData,
        })
    }

    fn into_positive(
        mut self,
        append: RetainedLifecycleRecordAppendV3,
        adopted_binding: CollectorReceiptFileBindingV3,
    ) -> Result<
        (
            RetainedCollectorReceiptRootOwnerV3,
            RetainedCollectorObservationV3,
        ),
        RestartCollectorErrorV3,
    > {
        self.revalidate()?;
        if adopted_binding != self.expected_lifecycle_binding {
            return Err(invalid(
                "S1 adopted collector binding differs from the unadopted generation",
            ));
        }
        self.receipt_root_owner
            .root
            .adopt_unadopted_tail(adopted_binding)?;
        self.durable.revalidate(&self.receipt_root_owner.root)?;
        let purpose = self.receipt.purpose;
        let match_result = self.receipt.match_result;
        let evidence = RetainedCollectorEvidenceV3 {
            durable: self.durable,
            guard: self.guard,
            lifecycle_record: Some(append),
            observation: self.observation,
            receipt: self.receipt,
            receipt_sha256: self.receipt_sha256,
            _not_send_or_sync: PhantomData,
        };
        let retained = match (purpose, match_result) {
            (CollectorPurposeV3::ReconciliationSnapshot, ReconciliationMatchV2::Zero) => {
                RetainedCollectorObservationV3::Reconciliation(RetainedCollectorMatchV3::Zero(
                    RetainedZeroMatchV3 { evidence },
                ))
            }
            (
                CollectorPurposeV3::ReconciliationSnapshot,
                ReconciliationMatchV2::Unique { mounted: false },
            ) => RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueAttached(RetainedUniqueMatchV3 {
                    evidence,
                    _state: PhantomData,
                }),
            ),
            (
                CollectorPurposeV3::ReconciliationSnapshot,
                ReconciliationMatchV2::Unique { mounted: true },
            ) => RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueMounted(RetainedUniqueMatchV3 {
                    evidence,
                    _state: PhantomData,
                }),
            ),
            (
                CollectorPurposeV3::ReconciliationSnapshot,
                ReconciliationMatchV2::Ambiguous { .. },
            ) => RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::Ambiguous(RetainedAmbiguousMatchV3 { evidence }),
            ),
            (CollectorPurposeV3::FreshAbsence, ReconciliationMatchV2::Zero) => {
                RetainedCollectorObservationV3::FreshAbsence(RetainedFreshAbsenceV3 { evidence })
            }
            (CollectorPurposeV3::FreshAbsence, _) => {
                return Err(invalid(
                    "FreshAbsence adopted evidence must have an exact Zero match",
                ));
            }
        };
        retained.revalidate_bound()?;
        self.receipt_root_owner.revalidate_observation(&retained)?;
        Ok((self.receipt_root_owner, retained))
    }
}

impl UnadoptedCollectorGenerationV3 {
    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.core.revalidate()
    }

    pub(crate) fn append_material(
        &self,
    ) -> Result<UnadoptedCollectorAppendV3<'_>, RestartCollectorErrorV3> {
        self.core.append_material()
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.core.receipt.operation_nonce
    }

    pub(crate) fn purpose(&self) -> CollectorPurposeV3 {
        self.core.receipt.purpose
    }

    pub(crate) fn validate_fresh_absence_successor(
        &self,
        prior: &RetainedCollectorObservationV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        let prior_snapshot = prior.snapshot_for_fresh_absence()?;
        if self.core.receipt.purpose != CollectorPurposeV3::FreshAbsence
            || self.core.receipt.match_result != ReconciliationMatchV2::Zero
        {
            return Err(invalid(
                "an unadopted collector successor is not exact FreshAbsence",
            ));
        }
        let expected_snapshot_sha256 = reconciliation_snapshot_sha256(prior_snapshot)
            .map_err(|error| invalid(format!("retained snapshot digest failed: {error}")))?;
        if self.core.receipt.reconciliation_snapshot_sha256.as_deref()
            != Some(expected_snapshot_sha256.as_str())
            || self.core.receipt.operation_nonce != prior_snapshot.operation_nonce
            || self.core.receipt.restart_epoch_nonce != prior_snapshot.restart_epoch_nonce
            || self.core.receipt.boot_session_uuid != prior_snapshot.boot_session_uuid
            || self.core.receipt.backing_identity_sha256 != prior_snapshot.backing_identity_sha256
            || self.core.receipt.collector_policy_sha256 != prior_snapshot.collector_policy_sha256
            || self.core.receipt.mountpoint_underlying_sha256
                != prior_snapshot.mountpoint_underlying_sha256
        {
            return Err(invalid(
                "FreshAbsence receipt is not the exact successor of the retained predictable snapshot",
            ));
        }
        Ok(())
    }

    pub(crate) fn into_s1_transfer(
        self,
    ) -> Result<S1CollectorReceiptAppendTransferV3, RestartCollectorErrorV3> {
        self.core.revalidate()?;
        let new_receipt = self.core.durable.file.try_clone()?;
        let new_bytes = self.core.durable.bytes.clone();
        Ok(S1CollectorReceiptAppendTransferV3 {
            core: self.core,
            new_bytes,
            new_receipt,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl S1CollectorReceiptAppendTransferV3 {
    pub(crate) fn into_s1_parts(
        self,
        _seal: S1CollectorReceiptAppendReadSealV3,
    ) -> (
        UnadoptedCollectorGenerationAfterTransferV3,
        FilesystemObjectBindingV3,
        FilesystemObjectBindingV3,
        CollectorReceiptFileBindingV3,
        File,
        Vec<u8>,
    ) {
        let Self {
            core,
            new_bytes,
            new_receipt,
            _not_send_or_sync: _,
        } = self;
        let before = core.before_root_binding;
        let after = core.receipt_root_owner.root.current_binding;
        let binding = core.expected_lifecycle_binding.clone();
        (
            UnadoptedCollectorGenerationAfterTransferV3 { core },
            before,
            after,
            binding,
            new_receipt,
            new_bytes,
        )
    }
}

impl UnadoptedCollectorGenerationAfterTransferV3 {
    pub(crate) fn bind_adopted_pair(
        self,
        append: RetainedLifecycleRecordAppendV3,
        adoption: S1AdoptedCollectorPairV3,
    ) -> Result<
        (
            RetainedCollectorReceiptRootOwnerV3,
            RetainedCollectorObservationV3,
        ),
        RestartCollectorErrorV3,
    > {
        self.core.revalidate()?;
        append.require_s1_adopted().map_err(|error| {
            invalid(format!(
                "collector pair lifecycle capsule lacks exact S1 adoption: {error}"
            ))
        })?;
        append.revalidate().map_err(|error| {
            invalid(format!(
                "collector pair lifecycle capsule failed exact replay: {error}"
            ))
        })?;
        let (operation_name, lifecycle_sha256, sequence, collector_binding) =
            adoption.into_collector_parts(S1CollectorPairAdoptionReadSealV3 { _private: () });
        if operation_name != format!("operation-{}", self.core.receipt.operation_nonce)
            || lifecycle_sha256 != append.digest()
            || sequence != append.sequence()
            || collector_binding != self.core.expected_lifecycle_binding
        {
            return Err(invalid(
                "S1 collector-pair acknowledgement differs from the unadopted generation",
            ));
        }
        self.core.into_positive(append, collector_binding)
    }
}

impl UnadoptedEjectObservationV3 {
    pub(crate) fn sealed_observation(
        &self,
    ) -> Result<SealedUnadoptedEjectObservationV3<'_>, RestartCollectorErrorV3> {
        self.expectation.validate_unadopted(&self.generation)?;
        Ok(SealedUnadoptedEjectObservationV3 {
            expectation: &self.expectation,
            generation: &self.generation,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn into_s1_transfer(
        self,
    ) -> Result<
        (
            EjectObservationAfterTransferV3,
            S1CollectorReceiptAppendTransferV3,
        ),
        RestartCollectorErrorV3,
    > {
        self.expectation.validate_unadopted(&self.generation)?;
        let transfer = self.generation.into_s1_transfer()?;
        Ok((
            EjectObservationAfterTransferV3 {
                expectation: self.expectation,
                _not_send_or_sync: PhantomData,
            },
            transfer,
        ))
    }
}

impl SealedUnadoptedEjectObservationV3<'_> {
    fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.expectation.validate_unadopted(self.generation)
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.expectation.binding.provenance.operation_nonce
    }

    pub(crate) fn iomedia_absence_sha256(&self) -> &str {
        &self.generation.core.receipt.iomedia_evidence_sha256
    }

    pub(crate) fn post_effect_collector_binding(
        &self,
    ) -> Result<PostEffectCollectorBindingV3, RestartCollectorErrorV3> {
        self.revalidate()?;
        let evidence = &self.generation.core;
        let binding = PostEffectCollectorBindingV3::from_retained_collector(
            PostEffectCollectorBindingSealV3 { _private: () },
            evidence.receipt.boot_session_uuid.clone(),
            evidence.expected_lifecycle_binding.clone(),
            evidence.receipt_sha256.clone(),
            self.expectation.lineage.first_snapshot_sha256()?,
            evidence.receipt.iomedia_evidence_sha256.clone(),
            evidence.receipt.operation_nonce.clone(),
            evidence.receipt.restart_epoch_nonce.clone(),
        );
        if binding.operation_nonce() != self.operation_nonce()
            || binding.observation_sha256() != self.iomedia_absence_sha256()
        {
            return Err(invalid(
                "unadopted eject collector binding changed across its sealed observation",
            ));
        }
        Ok(binding)
    }
}

impl EjectObservationAfterTransferV3 {
    pub(crate) fn bind_adopted_pair(
        self,
        generation: UnadoptedCollectorGenerationAfterTransferV3,
        append: RetainedLifecycleRecordAppendV3,
        adoption: S1AdoptedCollectorPairV3,
    ) -> Result<
        (
            RetainedCollectorReceiptRootOwnerV3,
            RetainedCollectorLineageV3,
        ),
        RestartCollectorErrorV3,
    > {
        let (receipt_root_owner, next) = generation.bind_adopted_pair(append, adoption)?;
        self.expectation
            .revalidate_with_owner(&receipt_root_owner, EjectInventoryEndpointV3::ExpectedAfter)?;
        validate_eject_successor_shape(
            &self.expectation.lineage.first,
            self.expectation.prior_observation(),
            &self.expectation.binding,
            &next.evidence().receipt,
            &next.evidence().guard,
        )?;
        let ArmedEjectExpectationCoreV3 {
            binding,
            lineage,
            _not_send_or_sync: _,
        } = self.expectation;
        let lineage = lineage.into_ejected_zero(binding, next)?;
        receipt_root_owner.revalidate_lineage(&lineage)?;
        Ok((receipt_root_owner, lineage))
    }
}

impl RetainedCollectorEvidenceV3 {
    fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.guard.revalidate(&self.receipt)?;
        self.revalidate_retained_capsule()
    }

    fn revalidate_retained_capsule(&self) -> Result<(), RestartCollectorErrorV3> {
        self.durable.revalidate_entry()?;
        if sha256(&canonical_json(&self.receipt)?) != self.receipt_sha256 {
            return Err(invalid(
                "retained collector receipt digest changed after final replay",
            ));
        }
        let expected = match self.receipt.purpose {
            CollectorPurposeV3::ReconciliationSnapshot => {
                FinalizedRestartObservationV3::ReconciliationSnapshot(
                    reconciliation_snapshot_from_receipt(
                        &self.receipt,
                        &self.receipt_sha256,
                        self.durable.lifecycle_binding(),
                    )?,
                )
            }
            CollectorPurposeV3::FreshAbsence => {
                FinalizedRestartObservationV3::FreshAbsence(fresh_absence_from_receipt(
                    &self.receipt,
                    &self.receipt_sha256,
                    self.durable.lifecycle_binding(),
                )?)
            }
        };
        if self.observation != expected {
            return Err(invalid(
                "retained collector observation differs from its durable receipt",
            ));
        }
        if let Some(append) = &self.lifecycle_record {
            append.require_s1_adopted().map_err(|error| {
                invalid(format!(
                    "retained collector lifecycle capsule lost its S1 adoption: {error}"
                ))
            })?;
            append.revalidate().map_err(|error| {
                invalid(format!(
                    "retained collector lifecycle capsule changed: {error}"
                ))
            })?;
            if !valid_digest(append.digest()) || append.sequence() == 0 {
                return Err(invalid(
                    "retained collector lifecycle-record binding is malformed",
                ));
            }
        }
        Ok(())
    }

    fn revalidate_across_mount_delta(
        &self,
        expected_after: &[MountBindingV3],
        direction: MountDeltaDirectionV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.durable.revalidate_entry()?;
        let record = self
            .lifecycle_record
            .as_ref()
            .ok_or_else(|| invalid("pre-delta collector lacks its exact lifecycle capsule"))?;
        record.revalidate().map_err(|error| {
            invalid(format!(
                "pre-delta lifecycle capsule failed descriptor replay: {error}"
            ))
        })?;
        if sha256(&canonical_json(&self.receipt)?) != self.receipt_sha256 {
            return Err(invalid("pre-delta durable collector receipt changed"));
        }
        self.guard
            .revalidate_across_mount_delta(&self.receipt, expected_after, direction)
    }
}

impl RetainedCollectorObservationV3 {
    fn evidence(&self) -> &RetainedCollectorEvidenceV3 {
        match self {
            Self::Reconciliation(match_result) => match match_result {
                RetainedCollectorMatchV3::Zero(value) => &value.evidence,
                RetainedCollectorMatchV3::UniqueAttached(value) => &value.evidence,
                RetainedCollectorMatchV3::UniqueMounted(value) => &value.evidence,
                RetainedCollectorMatchV3::Ambiguous(value) => &value.evidence,
            },
            Self::FreshAbsence(value) => &value.evidence,
        }
    }

    #[cfg(test)]
    fn evidence_mut(&mut self) -> &mut RetainedCollectorEvidenceV3 {
        match self {
            Self::Reconciliation(match_result) => match match_result {
                RetainedCollectorMatchV3::Zero(value) => &mut value.evidence,
                RetainedCollectorMatchV3::UniqueAttached(value) => &mut value.evidence,
                RetainedCollectorMatchV3::UniqueMounted(value) => &mut value.evidence,
                RetainedCollectorMatchV3::Ambiguous(value) => &mut value.evidence,
            },
            Self::FreshAbsence(value) => &mut value.evidence,
        }
    }

    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.evidence().revalidate()
    }

    pub(crate) fn revalidate_bound(&self) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        if self.evidence().lifecycle_record.is_none() {
            return Err(invalid(
                "retained collector observation lacks its durable lifecycle-record binding",
            ));
        }
        Ok(())
    }

    pub(crate) fn snapshot_for_fresh_absence(
        &self,
    ) -> Result<&ReconciliationSnapshotV2, RestartCollectorErrorV3> {
        self.revalidate_bound()?;
        let evidence = match self {
            Self::Reconciliation(RetainedCollectorMatchV3::Zero(value)) => &value.evidence,
            Self::Reconciliation(RetainedCollectorMatchV3::UniqueAttached(value)) => {
                &value.evidence
            }
            Self::Reconciliation(RetainedCollectorMatchV3::UniqueMounted(value)) => &value.evidence,
            Self::Reconciliation(RetainedCollectorMatchV3::Ambiguous(_))
            | Self::FreshAbsence(_) => {
                return Err(invalid(
                    "FreshAbsence requires an exact retained predictable reconciliation state",
                ));
            }
        };
        match &evidence.observation {
            FinalizedRestartObservationV3::ReconciliationSnapshot(snapshot)
                if snapshot.current_expected_absence_inventory_sha256.is_some() =>
            {
                Ok(snapshot)
            }
            _ => Err(invalid(
                "retained predictable typestate differs from its reconciliation snapshot",
            )),
        }
    }

    pub(crate) fn validate_fresh_absence_successor(
        &self,
        prior: &Self,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_bound()?;
        let prior_snapshot = prior.snapshot_for_fresh_absence()?;
        let next = match self {
            Self::FreshAbsence(value) => &value.evidence,
            Self::Reconciliation(_) => {
                return Err(invalid(
                    "a retained collector successor is not FreshAbsence",
                ));
            }
        };
        let expected_snapshot_sha256 = reconciliation_snapshot_sha256(prior_snapshot)
            .map_err(|error| invalid(format!("retained snapshot digest failed: {error}")))?;
        if next.receipt.reconciliation_snapshot_sha256.as_deref()
            != Some(expected_snapshot_sha256.as_str())
            || next.receipt.operation_nonce != prior_snapshot.operation_nonce
            || next.receipt.restart_epoch_nonce != prior_snapshot.restart_epoch_nonce
            || next.receipt.boot_session_uuid != prior_snapshot.boot_session_uuid
            || next.receipt.backing_identity_sha256 != prior_snapshot.backing_identity_sha256
            || next.receipt.collector_policy_sha256 != prior_snapshot.collector_policy_sha256
            || next.receipt.mountpoint_underlying_sha256
                != prior_snapshot.mountpoint_underlying_sha256
        {
            return Err(invalid(
                "FreshAbsence receipt is not the exact successor of the retained predictable snapshot",
            ));
        }
        Ok(())
    }

    fn issue_binding<'a>(
        &'a self,
        receipt_root_owner: &'a RetainedCollectorReceiptRootOwnerV3,
    ) -> Result<RetainedCollectorIssueBindingV3<'a>, RestartCollectorErrorV3> {
        receipt_root_owner.revalidate_observation(self)?;
        match self {
            Self::Reconciliation(RetainedCollectorMatchV3::UniqueAttached(_))
            | Self::Reconciliation(RetainedCollectorMatchV3::UniqueMounted(_)) => {}
            Self::Reconciliation(RetainedCollectorMatchV3::Zero(_))
            | Self::Reconciliation(RetainedCollectorMatchV3::Ambiguous(_))
            | Self::FreshAbsence(_) => {
                return Err(invalid(
                    "only one exact retained unique collector match may bind an effect issue",
                ));
            }
        }
        let evidence = self.evidence();
        let lifecycle_record = evidence
            .lifecycle_record
            .as_ref()
            .ok_or_else(|| invalid("collector lifecycle-record capsule is absent"))?;
        let lifecycle_record_sha256 = lifecycle_record.digest().to_string();
        let lifecycle_record_sequence = lifecycle_record.sequence();
        let unique_binding_sha256 = unique_collector_binding_sha256(&evidence.receipt)?;
        Ok(RetainedCollectorIssueBindingV3 {
            receipt_root_owner,
            retained: self,
            boot_session_uuid: evidence.receipt.boot_session_uuid.clone(),
            lifecycle_record_sha256,
            lifecycle_record_sequence,
            operation_nonce: evidence.receipt.operation_nonce.clone(),
            receipt_sha256: evidence.receipt_sha256.clone(),
            unique_binding_sha256,
            _not_send_or_sync: PhantomData,
        })
    }

    fn terminal_absence<'a>(
        &'a self,
        receipt_root_owner: &'a RetainedCollectorReceiptRootOwnerV3,
    ) -> Result<RetainedTerminalAbsenceV3<'a>, RestartCollectorErrorV3> {
        receipt_root_owner.revalidate_observation(self)?;
        let evidence = match self {
            Self::FreshAbsence(value) => &value.evidence,
            Self::Reconciliation(_) => {
                return Err(invalid(
                    "terminal closure requires an exact retained FreshAbsence observation",
                ));
            }
        };
        let observation = match &evidence.observation {
            FinalizedRestartObservationV3::FreshAbsence(observation) => observation,
            FinalizedRestartObservationV3::ReconciliationSnapshot(_) => {
                return Err(invalid(
                    "terminal FreshAbsence typestate contains a reconciliation snapshot",
                ));
            }
        };
        let token = RetainedTerminalAbsenceV3 {
            fresh_absence_sha256: fresh_absence_sha256(observation).map_err(|error| {
                invalid(format!("terminal FreshAbsence digest failed: {error}"))
            })?,
            operation_nonce: evidence.receipt.operation_nonce.clone(),
            receipt_root_owner,
            retained: self,
            _not_send_or_sync: PhantomData,
        };
        token.revalidate()?;
        Ok(token)
    }
}

impl RetainedCollectorLineageV3 {
    pub(crate) fn from_first(
        first: RetainedCollectorObservationV3,
    ) -> Result<Self, RestartCollectorErrorV3> {
        first.revalidate_bound()?;
        if !matches!(&first, RetainedCollectorObservationV3::Reconciliation(_)) {
            return Err(invalid(
                "collector lineage must begin with one exact reconciliation snapshot",
            ));
        }
        let lineage = Self {
            current: RetainedCollectorCurrentV3::First,
            first,
            _not_send_or_sync: PhantomData,
        };
        lineage.revalidate_bound()?;
        Ok(lineage)
    }

    fn current_observation(&self) -> &RetainedCollectorObservationV3 {
        match &self.current {
            RetainedCollectorCurrentV3::First => &self.first,
            RetainedCollectorCurrentV3::MountDelta { observation, .. }
            | RetainedCollectorCurrentV3::EjectedZero { observation, .. }
            | RetainedCollectorCurrentV3::FreshAbsence(observation) => observation,
        }
    }

    fn first_snapshot_raw(&self) -> Result<&ReconciliationSnapshotV2, RestartCollectorErrorV3> {
        let evidence = match &self.first {
            RetainedCollectorObservationV3::Reconciliation(RetainedCollectorMatchV3::Zero(
                value,
            )) => &value.evidence,
            RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueAttached(value),
            ) => &value.evidence,
            RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueMounted(value),
            ) => &value.evidence,
            RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::Ambiguous(value),
            ) => &value.evidence,
            RetainedCollectorObservationV3::FreshAbsence(_) => {
                return Err(invalid(
                    "collector lineage first owner is not a reconciliation snapshot",
                ));
            }
        };
        match &evidence.observation {
            FinalizedRestartObservationV3::ReconciliationSnapshot(snapshot) => Ok(snapshot),
            FinalizedRestartObservationV3::FreshAbsence(_) => Err(invalid(
                "collector lineage first owner changed observation kind",
            )),
        }
    }

    pub(crate) fn revalidate_bound(&self) -> Result<(), RestartCollectorErrorV3> {
        match &self.current {
            RetainedCollectorCurrentV3::First => self.first.revalidate_bound(),
            RetainedCollectorCurrentV3::MountDelta {
                direction,
                expected_after,
                observation,
            } => {
                self.first
                    .evidence()
                    .revalidate_across_mount_delta(expected_after, *direction)?;
                observation.revalidate_bound()?;
                validate_lineage_successor(&self.first, observation, expected_after, *direction)
            }
            RetainedCollectorCurrentV3::EjectedZero {
                binding,
                observation,
                prior,
            } => {
                self.revalidate_current_retained_capsules(prior)?;
                observation.revalidate_bound()?;
                let prior_observation = current_observation_for(&self.first, prior);
                prior_observation.evidence().guard.revalidate_eject_stable(
                    &prior_observation.evidence().receipt,
                    &binding.unchanged_mounts,
                )?;
                validate_eject_successor_shape(
                    &self.first,
                    prior_observation,
                    binding,
                    &observation.evidence().receipt,
                    &observation.evidence().guard,
                )
            }
            RetainedCollectorCurrentV3::FreshAbsence(observation) => {
                // FreshAbsence may legitimately follow eject, so the first
                // live IOMedia guard no longer describes current reality.  Its
                // immutable receipt and adopted V2 capsule remain retained;
                // the current absence observation owns the live replay.
                self.first.evidence().revalidate_retained_capsule()?;
                observation.revalidate_bound()?;
                observation.validate_fresh_absence_successor(&self.first)
            }
        }
    }

    fn revalidate_retained_capsules(&self) -> Result<(), RestartCollectorErrorV3> {
        self.first.evidence().revalidate_retained_capsule()?;
        self.revalidate_current_retained_capsules(&self.current)
    }

    fn revalidate_receipt_entries(
        &self,
        root: &RetainedReceiptRootV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.first.evidence().durable.revalidate(root)?;
        revalidate_current_receipt_entries(root, &self.current)
    }

    fn revalidate_current_retained_capsules(
        &self,
        current: &RetainedCollectorCurrentV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        match current {
            RetainedCollectorCurrentV3::First => Ok(()),
            RetainedCollectorCurrentV3::MountDelta {
                direction,
                expected_after,
                observation,
            } => {
                observation.evidence().revalidate_retained_capsule()?;
                validate_lineage_successor(&self.first, observation, expected_after, *direction)
            }
            RetainedCollectorCurrentV3::EjectedZero {
                binding,
                observation,
                prior,
            } => {
                self.revalidate_current_retained_capsules(prior)?;
                observation.evidence().revalidate_retained_capsule()?;
                validate_eject_successor_shape(
                    &self.first,
                    current_observation_for(&self.first, prior),
                    binding,
                    &observation.evidence().receipt,
                    &observation.evidence().guard,
                )
            }
            RetainedCollectorCurrentV3::FreshAbsence(observation) => {
                observation.evidence().revalidate_retained_capsule()?;
                observation.validate_fresh_absence_successor(&self.first)
            }
        }
    }

    fn revalidate_across_mount_delta(
        &self,
        expected_after: &[MountBindingV3],
        direction: MountDeltaDirectionV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        if !matches!(self.current, RetainedCollectorCurrentV3::First) {
            return Err(invalid(
                "this inert lineage admits only one mount-table delta from its first snapshot",
            ));
        }
        self.first
            .evidence()
            .revalidate_across_mount_delta(expected_after, direction)
    }

    fn issue_binding<'a>(
        &'a self,
        receipt_root_owner: &'a RetainedCollectorReceiptRootOwnerV3,
    ) -> Result<RetainedCollectorIssueBindingV3<'a>, RestartCollectorErrorV3> {
        self.revalidate_bound()?;
        self.current_observation().issue_binding(receipt_root_owner)
    }

    pub(crate) fn snapshot_for_fresh_absence(
        &self,
    ) -> Result<&ReconciliationSnapshotV2, RestartCollectorErrorV3> {
        self.revalidate_bound()?;
        let snapshot = self.first_snapshot_raw()?;
        if snapshot.current_expected_absence_inventory_sha256.is_none()
            || matches!(
                snapshot.match_result,
                ReconciliationMatchV2::Ambiguous { .. }
            )
        {
            return Err(invalid(
                "FreshAbsence requires the first exact predictable reconciliation snapshot",
            ));
        }
        Ok(snapshot)
    }

    pub(crate) fn advance_fresh_absence(
        self,
        next: RetainedCollectorObservationV3,
    ) -> Result<Self, RestartCollectorErrorV3> {
        next.validate_fresh_absence_successor(&self.first)?;
        let lineage = Self {
            current: RetainedCollectorCurrentV3::FreshAbsence(next),
            first: self.first,
            _not_send_or_sync: PhantomData,
        };
        lineage.revalidate_bound()?;
        Ok(lineage)
    }

    pub(crate) fn validate_fresh_absence_successor(
        &self,
        next: &RetainedCollectorObservationV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_bound()?;
        next.validate_fresh_absence_successor(&self.first)
    }

    fn terminal_absence<'a>(
        &'a self,
        receipt_root_owner: &'a RetainedCollectorReceiptRootOwnerV3,
    ) -> Result<RetainedTerminalAbsenceV3<'a>, RestartCollectorErrorV3> {
        self.revalidate_bound()?;
        self.current_observation()
            .terminal_absence(receipt_root_owner)
    }

    fn first_snapshot_sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        reconciliation_snapshot_sha256(self.first_snapshot_raw()?)
            .map_err(|error| invalid(format!("first reconciliation digest failed: {error}")))
    }

    fn into_mount_delta_current(
        self,
        direction: MountDeltaDirectionV3,
        expected_after: Vec<MountBindingV3>,
        next: RetainedCollectorObservationV3,
    ) -> Result<Self, RestartCollectorErrorV3> {
        if !matches!(self.current, RetainedCollectorCurrentV3::First) {
            return Err(invalid(
                "mount-table lineage current marker was already advanced",
            ));
        }
        validate_lineage_successor(&self.first, &next, &expected_after, direction)?;
        let lineage = Self {
            current: RetainedCollectorCurrentV3::MountDelta {
                direction,
                expected_after,
                observation: next,
            },
            first: self.first,
            _not_send_or_sync: PhantomData,
        };
        lineage.revalidate_bound()?;
        Ok(lineage)
    }

    fn into_ejected_zero(
        self,
        binding: EjectExpectationBindingV3,
        next: RetainedCollectorObservationV3,
    ) -> Result<Self, RestartCollectorErrorV3> {
        let prior_observation = current_observation_for(&self.first, &self.current);
        validate_eject_successor_shape(
            &self.first,
            prior_observation,
            &binding,
            &next.evidence().receipt,
            &next.evidence().guard,
        )?;
        let lineage = Self {
            current: RetainedCollectorCurrentV3::EjectedZero {
                binding,
                observation: next,
                prior: Box::new(self.current),
            },
            first: self.first,
            _not_send_or_sync: PhantomData,
        };
        lineage.revalidate_bound()?;
        Ok(lineage)
    }
}

fn current_observation_for<'a>(
    first: &'a RetainedCollectorObservationV3,
    current: &'a RetainedCollectorCurrentV3,
) -> &'a RetainedCollectorObservationV3 {
    match current {
        RetainedCollectorCurrentV3::First => first,
        RetainedCollectorCurrentV3::MountDelta { observation, .. }
        | RetainedCollectorCurrentV3::EjectedZero { observation, .. }
        | RetainedCollectorCurrentV3::FreshAbsence(observation) => observation,
    }
}

fn revalidate_current_receipt_entries(
    root: &RetainedReceiptRootV3,
    current: &RetainedCollectorCurrentV3,
) -> Result<(), RestartCollectorErrorV3> {
    match current {
        RetainedCollectorCurrentV3::First => Ok(()),
        RetainedCollectorCurrentV3::MountDelta { observation, .. }
        | RetainedCollectorCurrentV3::FreshAbsence(observation) => {
            observation.evidence().durable.revalidate(root)
        }
        RetainedCollectorCurrentV3::EjectedZero {
            observation, prior, ..
        } => {
            revalidate_current_receipt_entries(root, prior)?;
            observation.evidence().durable.revalidate(root)
        }
    }
}

fn validate_lineage_successor(
    prior: &RetainedCollectorObservationV3,
    next: &RetainedCollectorObservationV3,
    expected_after: &[MountBindingV3],
    direction: MountDeltaDirectionV3,
) -> Result<(), RestartCollectorErrorV3> {
    let prior = prior.evidence();
    let next = next.evidence();
    let expected_match = match direction {
        MountDeltaDirectionV3::Mount => ReconciliationMatchV2::Unique { mounted: true },
        MountDeltaDirectionV3::Unmount => ReconciliationMatchV2::Unique { mounted: false },
    };
    if next.receipt.purpose != CollectorPurposeV3::ReconciliationSnapshot
        || next.receipt.match_result != expected_match
        || next.receipt.mount_evidence.mounts_before != expected_after
        || next.receipt.mount_evidence.mounts_after != expected_after
        || next.guard.mounts != expected_after
        || next.receipt.operation_nonce != prior.receipt.operation_nonce
        || next.receipt.boot_session_uuid != prior.receipt.boot_session_uuid
        || next.receipt.restart_epoch_nonce != prior.receipt.restart_epoch_nonce
        || next.receipt.collector_policy_sha256 != prior.receipt.collector_policy_sha256
        || next.receipt.backing_identity_sha256 != prior.receipt.backing_identity_sha256
        || next.receipt.mountpoint_underlying_sha256 != prior.receipt.mountpoint_underlying_sha256
        || next.receipt.matching_groups != prior.receipt.matching_groups
    {
        return Err(invalid(
            "collector lineage current observation is not the exact successor of its first snapshot",
        ));
    }
    Ok(())
}

impl RetainedTerminalAbsenceV3<'_> {
    pub(crate) fn fresh_absence_sha256(&self) -> &str {
        &self.fresh_absence_sha256
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.receipt_root_owner
            .revalidate_observation(self.retained)?;
        let evidence = match self.retained {
            RetainedCollectorObservationV3::FreshAbsence(value) => &value.evidence,
            RetainedCollectorObservationV3::Reconciliation(_) => {
                return Err(invalid(
                    "terminal absence token lost its FreshAbsence owner",
                ));
            }
        };
        let observation = match &evidence.observation {
            FinalizedRestartObservationV3::FreshAbsence(observation) => observation,
            FinalizedRestartObservationV3::ReconciliationSnapshot(_) => {
                return Err(invalid("terminal absence token changed observation kind"));
            }
        };
        if evidence.receipt.operation_nonce != self.operation_nonce
            || fresh_absence_sha256(observation)
                .map_err(|error| invalid(format!("terminal FreshAbsence replay failed: {error}")))?
                != self.fresh_absence_sha256
        {
            return Err(invalid(
                "terminal absence token changed during retained descriptor replay",
            ));
        }
        Ok(())
    }
}

impl RetainedCollectorIssueBindingV3<'_> {
    pub(crate) fn boot_session_uuid(&self) -> &str {
        &self.boot_session_uuid
    }

    pub(crate) fn lifecycle_record_sha256(&self) -> &str {
        &self.lifecycle_record_sha256
    }

    pub(crate) fn lifecycle_record_sequence(&self) -> u32 {
        self.lifecycle_record_sequence
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    pub(crate) fn receipt_sha256(&self) -> &str {
        &self.receipt_sha256
    }

    pub(crate) fn unique_binding_sha256(&self) -> &str {
        &self.unique_binding_sha256
    }

    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.receipt_root_owner
            .revalidate_observation(self.retained)?;
        let evidence = self.retained.evidence();
        if evidence.receipt.boot_session_uuid != self.boot_session_uuid
            || evidence.receipt.operation_nonce != self.operation_nonce
            || evidence.receipt_sha256 != self.receipt_sha256
            || evidence
                .lifecycle_record
                .as_ref()
                .map(RetainedLifecycleRecordAppendV3::digest)
                != Some(self.lifecycle_record_sha256.as_str())
            || evidence
                .lifecycle_record
                .as_ref()
                .map(RetainedLifecycleRecordAppendV3::sequence)
                != Some(self.lifecycle_record_sequence)
            || unique_collector_binding_sha256(&evidence.receipt)? != self.unique_binding_sha256
        {
            return Err(invalid(
                "retained collector issue binding changed during descriptor replay",
            ));
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct UniqueCollectorBindingDigestV3<'a> {
    backing_identity_sha256: &'a str,
    boot_session_uuid: &'a str,
    iomedia_evidence_sha256: &'a str,
    match_result: &'a ReconciliationMatchV2,
    matching_groups: &'a [MatchingDiskImageGroupV3],
    mount_evidence_sha256: &'a str,
    mountpoint_underlying_sha256: &'a str,
    operation_nonce: &'a str,
    receipt_sha256: String,
    restart_epoch_nonce: &'a str,
    schema: &'static str,
}

fn unique_collector_binding_sha256(
    receipt: &RestartCollectorReceiptV3,
) -> Result<String, RestartCollectorErrorV3> {
    if !matches!(receipt.match_result, ReconciliationMatchV2::Unique { .. })
        || receipt.matching_groups.len() != 1
    {
        return Err(invalid(
            "collector receipt is not one exact unique disk-image binding",
        ));
    }
    let receipt_sha256 = sha256(&canonical_json(receipt)?);
    Ok(sha256(&canonical_json(&UniqueCollectorBindingDigestV3 {
        backing_identity_sha256: &receipt.backing_identity_sha256,
        boot_session_uuid: &receipt.boot_session_uuid,
        iomedia_evidence_sha256: &receipt.iomedia_evidence_sha256,
        match_result: &receipt.match_result,
        matching_groups: &receipt.matching_groups,
        mount_evidence_sha256: &receipt.mount_evidence_sha256,
        mountpoint_underlying_sha256: &receipt.mountpoint_underlying_sha256,
        operation_nonce: &receipt.operation_nonce,
        receipt_sha256,
        restart_epoch_nonce: &receipt.restart_epoch_nonce,
        schema: "hepta_mac_unique_collector_binding_v3",
    })?))
}

fn derive_collector_effect_plan(
    receipt_root_owner: &RetainedCollectorReceiptRootOwnerV3,
    lineage: &RetainedCollectorLineageV3,
    kind: SealedCollectorEffectPlanKindV3,
) -> Result<DerivedCollectorEffectPlanV3, RestartCollectorErrorV3> {
    receipt_root_owner.revalidate_lineage(lineage)?;
    let current = lineage.current_observation();
    let evidence = current.evidence();
    if evidence.receipt.authority.any() {
        return Err(invalid(
            "sealed collector effect plan cannot inherit any authority bit",
        ));
    }
    let group = match (kind, current) {
        (
            SealedCollectorEffectPlanKindV3::Unmount,
            RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueMounted(unique),
            ),
        ) => exact_unique_group(&unique.evidence.receipt)?,
        (
            SealedCollectorEffectPlanKindV3::Eject,
            RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueAttached(unique),
            ),
        ) => exact_unique_group(&unique.evidence.receipt)?,
        (SealedCollectorEffectPlanKindV3::Unmount, _) => {
            return Err(invalid(
                "sealed unmount plan requires the retained unique-mounted current observation",
            ));
        }
        (SealedCollectorEffectPlanKindV3::Eject, _) => {
            return Err(invalid(
                "sealed eject plan requires the retained unique-attached current observation",
            ));
        }
    };
    let collector_binding = receipt_root_owner.issue_binding(lineage)?;
    collector_binding.revalidate()?;
    let (command, specific) = match kind {
        SealedCollectorEffectPlanKindV3::Unmount => {
            let mounted_binding = exact_unmount_target(&evidence.receipt, group)?;
            let mounted_binding_sha256 = sha256(&canonical_json(&mounted_binding)?);
            (
                ExactDisposableCommandV3::UnmountVolume {
                    mounted_binding_sha256: mounted_binding_sha256.clone(),
                },
                SealedCollectorEffectSpecificProvenanceV3::Unmount {
                    mounted_binding,
                    mounted_binding_sha256,
                },
            )
        }
        SealedCollectorEffectPlanKindV3::Eject => {
            let disk_image_group_sha256 = exact_eject_target(&evidence.receipt, group)?;
            (
                ExactDisposableCommandV3::EjectImage {
                    disk_image_group_sha256: disk_image_group_sha256.clone(),
                },
                SealedCollectorEffectSpecificProvenanceV3::Eject {
                    disk_image_group_sha256,
                },
            )
        }
    };
    let command_canonical_bytes = canonical_json(&command)?;
    let command_sha256 = sha256(&command_canonical_bytes);
    Ok(DerivedCollectorEffectPlanV3 {
        command,
        command_canonical_bytes,
        command_sha256,
        provenance: SealedCollectorEffectPlanProvenanceV3 {
            boot_session_uuid: collector_binding.boot_session_uuid.clone(),
            collector_receipt_sha256: collector_binding.receipt_sha256.clone(),
            lifecycle_record_sequence: collector_binding.lifecycle_record_sequence,
            lifecycle_record_sha256: collector_binding.lifecycle_record_sha256.clone(),
            operation_nonce: collector_binding.operation_nonce.clone(),
            restart_epoch_nonce: evidence.receipt.restart_epoch_nonce.clone(),
            specific,
            unique_binding_sha256: collector_binding.unique_binding_sha256.clone(),
        },
    })
}

fn exact_unmount_target(
    receipt: &RestartCollectorReceiptV3,
    group: &MatchingDiskImageGroupV3,
) -> Result<MountBindingV3, RestartCollectorErrorV3> {
    let group_mount_count = receipt
        .mount_evidence
        .mounts_after
        .iter()
        .filter(|mount| group_source_matches(group, &mount.mount_from))
        .count();
    let targets = receipt
        .mount_evidence
        .mounts_after
        .iter()
        .filter(|mount| {
            mount.mount_on == receipt.collector_policy.mountpoint
                && group_source_matches(group, &mount.mount_from)
        })
        .cloned()
        .collect::<Vec<_>>();
    let [target] = targets.as_slice() else {
        return Err(invalid(
            "unique-mounted collector does not retain exactly one group-bound mount entry",
        ));
    };
    if group_mount_count != 1 {
        return Err(invalid(
            "unique-mounted collector retains an additional disk-image group mount alias",
        ));
    }
    validate_mount_binding_shape(target)?;
    Ok(target.clone())
}

fn exact_eject_target(
    receipt: &RestartCollectorReceiptV3,
    group: &MatchingDiskImageGroupV3,
) -> Result<String, RestartCollectorErrorV3> {
    if receipt
        .mount_evidence
        .mounts_after
        .iter()
        .any(|mount| group_source_matches(group, &mount.mount_from))
    {
        return Err(invalid(
            "sealed eject plan requires the unique disk-image group to be fully unmounted",
        ));
    }
    unique_volume_identity_sha256(group)
}

fn derive_eject_expectation_binding(
    lineage: &RetainedCollectorLineageV3,
    command_sha256: &str,
    provenance: &SealedCollectorEffectPlanProvenanceV3,
) -> Result<EjectExpectationBindingV3, RestartCollectorErrorV3> {
    lineage.revalidate_retained_capsules()?;
    let current = lineage.current_observation();
    let evidence = current.evidence();
    let unique = match current {
        RetainedCollectorObservationV3::Reconciliation(
            RetainedCollectorMatchV3::UniqueAttached(unique),
        ) => unique,
        _ => {
            return Err(invalid(
                "eject expectation requires the retained unique-attached current observation",
            ));
        }
    };
    let group = exact_unique_group(&unique.evidence.receipt)?;
    let disk_image_group_sha256 = exact_eject_target(&evidence.receipt, group)?;
    let SealedCollectorEffectSpecificProvenanceV3::Eject {
        disk_image_group_sha256: provenance_group_sha256,
    } = &provenance.specific
    else {
        return Err(invalid(
            "eject expectation inherited non-eject effect provenance",
        ));
    };
    let lifecycle_record = evidence
        .lifecycle_record
        .as_ref()
        .ok_or_else(|| invalid("eject expectation lacks its prior lifecycle capsule"))?;
    let unique_binding_sha256 = unique_collector_binding_sha256(&evidence.receipt)?;
    if !valid_digest(command_sha256)
        || provenance_group_sha256 != &disk_image_group_sha256
        || provenance.boot_session_uuid != evidence.receipt.boot_session_uuid
        || provenance.collector_receipt_sha256 != evidence.receipt_sha256
        || provenance.lifecycle_record_sequence != lifecycle_record.sequence()
        || provenance.lifecycle_record_sha256 != lifecycle_record.digest()
        || provenance.operation_nonce != evidence.receipt.operation_nonce
        || provenance.restart_epoch_nonce != evidence.receipt.restart_epoch_nonce
        || provenance.unique_binding_sha256 != unique_binding_sha256
    {
        return Err(invalid(
            "eject expectation differs from the exact durable issue provenance",
        ));
    }
    let before_inventory = evidence.receipt.iomedia_inventory.clone();
    let expected_after_inventory = evidence
        .receipt
        .current_expected_absence_inventory
        .as_ref()
        .ok_or_else(|| invalid("unique-attached eject source has no exact expected absence"))?
        .clone();
    let expected_after_inventory_sha256 = sha256(&canonical_json(&expected_after_inventory)?);
    if evidence
        .receipt
        .current_expected_absence_inventory_sha256
        .as_deref()
        != Some(expected_after_inventory_sha256.as_str())
    {
        return Err(invalid(
            "eject expected-after inventory differs from its source receipt digest",
        ));
    }
    validate_eject_inventory_endpoint(
        &before_inventory,
        &expected_after_inventory,
        group,
        &before_inventory,
        EjectInventoryEndpointV3::Pending,
    )?;
    let first = lineage.first.evidence();
    let first_expected = first
        .receipt
        .current_expected_absence_inventory
        .as_ref()
        .ok_or_else(|| invalid("first reconciliation receipt has no exact expected absence"))?;
    let first_expected_sha256 = first
        .receipt
        .current_expected_absence_inventory_sha256
        .as_deref()
        .ok_or_else(|| invalid("first reconciliation receipt has no expected-absence digest"))?;
    if first_expected != &expected_after_inventory
        || first_expected_sha256 != expected_after_inventory_sha256
    {
        return Err(invalid(
            "eject expected-after inventory differs from the immutable first snapshot",
        ));
    }
    Ok(EjectExpectationBindingV3 {
        before_inventory,
        command_sha256: command_sha256.to_string(),
        disk_image_group_sha256,
        expected_after_inventory,
        expected_after_inventory_sha256,
        provenance: provenance.clone(),
        unchanged_mounts: evidence.receipt.mount_evidence.mounts_after.clone(),
    })
}

fn validate_eject_inventory_endpoint(
    before_inventory: &RestartIOMediaInventoryV3,
    expected_after_inventory: &RestartIOMediaInventoryV3,
    group: &MatchingDiskImageGroupV3,
    observed: &RestartIOMediaInventoryV3,
    endpoint: EjectInventoryEndpointV3,
) -> Result<(), RestartCollectorErrorV3> {
    validate_restart_iomedia_inventory_v3(before_inventory)?;
    validate_restart_iomedia_inventory_v3(expected_after_inventory)?;
    validate_restart_iomedia_inventory_v3(observed)?;
    let derived = derive_current_expected_absence_v3(
        before_inventory,
        &ReconciliationMatchV2::Unique { mounted: false },
        std::slice::from_ref(group),
    )?
    .ok_or_else(|| invalid("unique eject source did not derive one expected-after inventory"))?;
    if derived != *expected_after_inventory
        || before_inventory.boot_session_uuid != expected_after_inventory.boot_session_uuid
        || observed.boot_session_uuid != before_inventory.boot_session_uuid
    {
        return Err(invalid(
            "eject expected-after inventory is not the exact before inventory minus its unique group",
        ));
    }
    let matches = match endpoint {
        EjectInventoryEndpointV3::Pending => {
            observed == before_inventory || observed == expected_after_inventory
        }
        EjectInventoryEndpointV3::ExpectedAfter => observed == expected_after_inventory,
    };
    if !matches {
        return Err(invalid(
            "eject inventory observed neither the admitted endpoint nor its exact expected-after state",
        ));
    }
    Ok(())
}

fn validate_eject_mount_endpoint(
    unchanged_mounts: &[MountBindingV3],
    observed_before: &[MountBindingV3],
    observed_after: &[MountBindingV3],
    live_mounts: &[MountBindingV3],
    mountpoint_underlying_revalidated: bool,
    no_nested_mounts: bool,
) -> Result<(), RestartCollectorErrorV3> {
    for mount in unchanged_mounts
        .iter()
        .chain(observed_before)
        .chain(observed_after)
        .chain(live_mounts)
    {
        validate_mount_binding_shape(mount)?;
    }
    if observed_before != unchanged_mounts
        || observed_after != unchanged_mounts
        || live_mounts != unchanged_mounts
        || !mountpoint_underlying_revalidated
        || !no_nested_mounts
    {
        return Err(invalid(
            "post-eject mount census differs from its exact unchanged endpoint",
        ));
    }
    Ok(())
}

fn validate_derived_collector_effect_plan(
    command: &ExactDisposableCommandV3,
    command_canonical_bytes: &[u8],
    command_sha256: &str,
    provenance: &SealedCollectorEffectPlanProvenanceV3,
    derived: &DerivedCollectorEffectPlanV3,
) -> Result<(), RestartCollectorErrorV3> {
    if command != &derived.command
        || command_canonical_bytes != derived.command_canonical_bytes
        || command_sha256 != derived.command_sha256
        || provenance != &derived.provenance
        || canonical_json(command)? != command_canonical_bytes
        || sha256(command_canonical_bytes) != command_sha256
        || !valid_digest(command_sha256)
    {
        return Err(invalid(
            "sealed collector effect plan changed command or exact retained provenance",
        ));
    }
    Ok(())
}

impl ArmedEjectExpectationCoreV3 {
    fn prior_observation(&self) -> &RetainedCollectorObservationV3 {
        self.lineage.current_observation()
    }

    fn revalidate_binding(&self) -> Result<(), RestartCollectorErrorV3> {
        let derived = derive_eject_expectation_binding(
            &self.lineage,
            &self.binding.command_sha256,
            &self.binding.provenance,
        )?;
        if derived != self.binding {
            return Err(invalid(
                "armed eject expectation changed its exact retained endpoints",
            ));
        }
        Ok(())
    }

    fn revalidate_with_owner(
        &self,
        receipt_root_owner: &RetainedCollectorReceiptRootOwnerV3,
        endpoint: EjectInventoryEndpointV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        receipt_root_owner.revalidate_lineage_retained_capsules(&self.lineage)?;
        self.revalidate_binding()?;
        let prior = self.prior_observation();
        let evidence = prior.evidence();
        evidence
            .guard
            .revalidate_eject_stable(&evidence.receipt, &self.binding.unchanged_mounts)?;
        let group = exact_unique_group(&evidence.receipt)?;
        if unique_volume_identity_sha256(group)? != self.binding.disk_image_group_sha256 {
            return Err(invalid(
                "armed eject expectation changed its unique disk-image group",
            ));
        }
        let current = capture_restart_iomedia_inventory_v3()?;
        current.revalidate_after_persistence()?;
        validate_eject_inventory_endpoint(
            &self.binding.before_inventory,
            &self.binding.expected_after_inventory,
            group,
            current.report(),
            endpoint,
        )?;
        current.revalidate_after_persistence()?;
        receipt_root_owner.revalidate_lineage_retained_capsules(&self.lineage)
    }

    fn validate_pending_observation(
        &self,
        pending: &PendingRestartObservationV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_with_owner(
            &pending.receipt_root_owner,
            EjectInventoryEndpointV3::ExpectedAfter,
        )?;
        pending.guard.revalidate(&pending.receipt)?;
        validate_eject_successor_shape(
            &self.lineage.first,
            self.prior_observation(),
            &self.binding,
            &pending.receipt,
            &pending.guard,
        )?;
        pending.receipt_root_owner.revalidate()
    }

    fn validate_unadopted(
        &self,
        generation: &UnadoptedCollectorGenerationV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        generation.revalidate()?;
        generation
            .core
            .receipt_root_owner
            .revalidate_lineage_retained_capsules(&self.lineage)?;
        self.revalidate_binding()?;
        validate_eject_successor_shape(
            &self.lineage.first,
            self.prior_observation(),
            &self.binding,
            &generation.core.receipt,
            &generation.core.guard,
        )
    }
}

impl ArmedEjectExpectationV3 {
    pub(crate) fn revalidate_pending(&self) -> Result<(), RestartCollectorErrorV3> {
        self.core
            .revalidate_with_owner(&self.receipt_root_owner, EjectInventoryEndpointV3::Pending)
    }

    pub(crate) fn validate_issued_binding(
        &self,
        operation_nonce: &str,
        command_sha256: &str,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_pending()?;
        if operation_nonce != self.core.binding.provenance.operation_nonce
            || command_sha256 != self.core.binding.command_sha256
        {
            return Err(invalid(
                "issued eject binding differs from its armed collector expectation",
            ));
        }
        Ok(())
    }
}

fn validate_eject_successor_shape(
    first: &RetainedCollectorObservationV3,
    prior: &RetainedCollectorObservationV3,
    binding: &EjectExpectationBindingV3,
    next: &RestartCollectorReceiptV3,
    next_guard: &LiveReplayGuardV3,
) -> Result<(), RestartCollectorErrorV3> {
    let prior = prior.evidence();
    let first = first.evidence();
    let group = exact_unique_group(&prior.receipt)?;
    validate_eject_inventory_endpoint(
        &binding.before_inventory,
        &binding.expected_after_inventory,
        group,
        &next.iomedia_inventory,
        EjectInventoryEndpointV3::ExpectedAfter,
    )?;
    validate_eject_mount_endpoint(
        &binding.unchanged_mounts,
        &next.mount_evidence.mounts_before,
        &next.mount_evidence.mounts_after,
        &next_guard.mounts,
        next.mount_evidence.mountpoint_underlying_revalidated,
        next.mount_evidence.no_nested_mounts,
    )?;
    let expected_after_sha256 = sha256(&canonical_json(&binding.expected_after_inventory)?);
    if expected_after_sha256 != binding.expected_after_inventory_sha256
        || unique_volume_identity_sha256(group)? != binding.disk_image_group_sha256
        || next.purpose != CollectorPurposeV3::ReconciliationSnapshot
        || next.match_result != ReconciliationMatchV2::Zero
        || !next.matching_groups.is_empty()
        || next.current_expected_absence_inventory.as_ref()
            != Some(&binding.expected_after_inventory)
        || next.current_expected_absence_inventory_sha256.as_deref()
            != Some(binding.expected_after_inventory_sha256.as_str())
        || next.iomedia_evidence_sha256 != binding.expected_after_inventory_sha256
        || next.operation_nonce != binding.provenance.operation_nonce
        || next.operation_nonce != prior.receipt.operation_nonce
        || next.boot_session_uuid != binding.provenance.boot_session_uuid
        || next.boot_session_uuid != prior.receipt.boot_session_uuid
        || next.restart_epoch_nonce != binding.provenance.restart_epoch_nonce
        || next.restart_epoch_nonce != prior.receipt.restart_epoch_nonce
        || next.collector_policy_sha256 != prior.receipt.collector_policy_sha256
        || next.backing_identity_sha256 != prior.receipt.backing_identity_sha256
        || next.mountpoint_underlying_sha256 != prior.receipt.mountpoint_underlying_sha256
        || next.baseline_inventory_sha256 != prior.receipt.baseline_inventory_sha256
        || next.artifact_evidence_sha256 != prior.receipt.artifact_evidence_sha256
        || next.monotonic_before_nanoseconds <= prior.receipt.monotonic_after_nanoseconds
        || next.reconciliation_snapshot_sha256.is_some()
        || first.receipt.current_expected_absence_inventory.as_ref()
            != Some(&binding.expected_after_inventory)
        || first
            .receipt
            .current_expected_absence_inventory_sha256
            .as_deref()
            != Some(binding.expected_after_inventory_sha256.as_str())
    {
        return Err(invalid(
            "post-eject collector is not the exact Zero successor of its armed expectation",
        ));
    }
    Ok(())
}

impl UnadoptedCollectorAppendV3<'_> {
    pub(crate) fn event(&self) -> &RetainedCollectorAppendEventV3<'_> {
        &self.event
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        self.operation_nonce
    }
}

#[cfg(test)]
impl RetainedCollectorObservationV3 {
    fn observation_for_test(&self) -> &FinalizedRestartObservationV3 {
        match self {
            Self::Reconciliation(match_result) => match match_result {
                RetainedCollectorMatchV3::Zero(value) => &value.evidence.observation,
                RetainedCollectorMatchV3::UniqueAttached(value) => &value.evidence.observation,
                RetainedCollectorMatchV3::UniqueMounted(value) => &value.evidence.observation,
                RetainedCollectorMatchV3::Ambiguous(value) => &value.evidence.observation,
            },
            Self::FreshAbsence(value) => &value.evidence.observation,
        }
    }
}

pub(crate) fn validate_reconciliation_snapshot_shape_v3(
    snapshot: &ReconciliationSnapshotV2,
) -> Result<(), RestartCollectorErrorV3> {
    if !valid_digest(&snapshot.backing_identity_sha256)
        || !valid_uuid(&snapshot.boot_session_uuid)
        || !valid_digest(&snapshot.collector_policy_sha256)
        || !valid_digest(&snapshot.collector_receipt_sha256)
        || !valid_digest(&snapshot.iomedia_evidence_sha256)
        || !valid_digest(&snapshot.mount_evidence_sha256)
        || !valid_digest(&snapshot.mountpoint_underlying_sha256)
        || !valid_nonce(&snapshot.operation_nonce)
        || !valid_nonce(&snapshot.restart_epoch_nonce)
        || snapshot.monotonic_before_nanoseconds == 0
        || snapshot.monotonic_after_nanoseconds < snapshot.monotonic_before_nanoseconds
        || matches!(
            snapshot.match_result,
            ReconciliationMatchV2::Ambiguous {
                matching_objects: 0 | 1
            }
        )
    {
        return Err(invalid("reconciliation snapshot shape is malformed"));
    }
    match (
        snapshot.match_result,
        snapshot
            .current_expected_absence_inventory_sha256
            .as_deref(),
    ) {
        (ReconciliationMatchV2::Zero, Some(expected))
            if valid_digest(expected) && expected == snapshot.iomedia_evidence_sha256 => {}
        (ReconciliationMatchV2::Unique { .. }, Some(expected))
            if valid_digest(expected) && expected != snapshot.iomedia_evidence_sha256 => {}
        (ReconciliationMatchV2::Ambiguous { .. }, None) => {}
        _ => {
            return Err(invalid(
                "reconciliation snapshot current-boot expected absence is malformed",
            ));
        }
    }
    Ok(())
}

fn reconciliation_snapshot_from_receipt(
    receipt: &RestartCollectorReceiptV3,
    receipt_sha256: &str,
    receipt_file: CollectorReceiptFileBindingV3,
) -> Result<ReconciliationSnapshotV2, RestartCollectorErrorV3> {
    validate_receipt(receipt)?;
    if receipt.purpose != CollectorPurposeV3::ReconciliationSnapshot
        || !valid_digest(receipt_sha256)
        || sha256(&canonical_json(receipt)?) != receipt_sha256
    {
        return Err(invalid(
            "reconciliation snapshot does not project from the exact durable receipt",
        ));
    }
    let snapshot = ReconciliationSnapshotV2 {
        backing_identity_sha256: receipt.backing_identity_sha256.clone(),
        boot_session_uuid: receipt.boot_session_uuid.clone(),
        collector_policy_sha256: receipt.collector_policy_sha256.clone(),
        collector_receipt_sha256: receipt_sha256.to_string(),
        collector_receipt_file: Some(receipt_file),
        current_expected_absence_inventory_sha256: receipt
            .current_expected_absence_inventory_sha256
            .clone(),
        iomedia_evidence_sha256: receipt.iomedia_evidence_sha256.clone(),
        match_result: receipt.match_result,
        monotonic_after_nanoseconds: receipt.monotonic_after_nanoseconds,
        monotonic_before_nanoseconds: receipt.monotonic_before_nanoseconds,
        mount_evidence_sha256: receipt.mount_evidence_sha256.clone(),
        mountpoint_underlying_sha256: receipt.mountpoint_underlying_sha256.clone(),
        operation_nonce: receipt.operation_nonce.clone(),
        restart_epoch_nonce: receipt.restart_epoch_nonce.clone(),
    };
    validate_reconciliation_snapshot_shape_v3(&snapshot)?;
    Ok(snapshot)
}

fn fresh_absence_from_receipt(
    receipt: &RestartCollectorReceiptV3,
    receipt_sha256: &str,
    receipt_file: CollectorReceiptFileBindingV3,
) -> Result<FreshAbsenceObservationV2, RestartCollectorErrorV3> {
    validate_receipt(receipt)?;
    if receipt.purpose != CollectorPurposeV3::FreshAbsence
        || receipt.match_result != ReconciliationMatchV2::Zero
        || !receipt.baseline_restored
        || !receipt.operation_artifacts_absent
        || !receipt.mount_evidence.no_nested_mounts
        || !receipt.mount_evidence.mountpoint_underlying_revalidated
        || receipt.reconciliation_snapshot_sha256.is_none()
        || receipt.current_expected_absence_inventory_sha256.as_deref()
            != Some(receipt.iomedia_evidence_sha256.as_str())
        || !valid_digest(receipt_sha256)
        || sha256(&canonical_json(receipt)?) != receipt_sha256
    {
        return Err(invalid(
            "fresh absence does not project from the exact durable receipt",
        ));
    }
    Ok(FreshAbsenceObservationV2 {
        artifact_evidence_sha256: receipt.artifact_evidence_sha256.clone(),
        baseline_inventory_sha256: receipt.baseline_inventory_sha256.clone(),
        backing_identity_sha256: receipt.backing_identity_sha256.clone(),
        boot_session_uuid: receipt.boot_session_uuid.clone(),
        collector_policy_sha256: receipt.collector_policy_sha256.clone(),
        collector_receipt_sha256: receipt_sha256.to_string(),
        collector_receipt_file: Some(receipt_file),
        current_expected_absence_inventory_sha256: receipt
            .current_expected_absence_inventory_sha256
            .clone(),
        iomedia_evidence_sha256: receipt.iomedia_evidence_sha256.clone(),
        monotonic_after_nanoseconds: receipt.monotonic_after_nanoseconds,
        monotonic_before_nanoseconds: receipt.monotonic_before_nanoseconds,
        mount_evidence_sha256: receipt.mount_evidence_sha256.clone(),
        mountpoint_underlying_sha256: receipt.mountpoint_underlying_sha256.clone(),
        no_matching_iomedia: true,
        no_nested_mounts: true,
        operation_nonce: receipt.operation_nonce.clone(),
        operation_artifacts_absent: true,
        post_inventory_sha256: receipt.post_inventory_sha256.clone(),
        reconciliation_snapshot_sha256: receipt.reconciliation_snapshot_sha256.clone(),
        restart_epoch_nonce: Some(receipt.restart_epoch_nonce.clone()),
    })
}

impl ValidatedExistingReceiptV3 {
    fn lifecycle_binding(&self) -> Result<CollectorReceiptFileBindingV3, RestartCollectorErrorV3> {
        self.lifecycle_binding.clone().ok_or_else(|| {
            invalid("existing collector receipt lacks its durable root-generation binding")
        })
    }

    fn revalidate(
        &self,
        directory_fd: RawFd,
        receipt_root: &str,
    ) -> Result<(), RestartCollectorErrorV3> {
        verify_fd_binding_secure(self.file.as_raw_fd(), &self.binding, "existing receipt")?;
        if fstatat_binding(directory_fd, &self.name, "existing receipt pathname")? != self.binding
            || read_fd_exact(&self.file, &self.binding)? != self.bytes
            || receipt_name_digest(&self.name).is_none_or(|digest| sha256(&self.bytes) != digest)
            || self.receipt.collector_policy.receipt_root != receipt_root
            || canonical_json(&self.receipt)? != self.bytes
        {
            return Err(invalid(
                "existing collector receipt changed or is not bound to this receipt root",
            ));
        }
        validate_receipt(&self.receipt)?;
        verify_fd_binding_secure(self.file.as_raw_fd(), &self.binding, "existing receipt")?;
        if fstatat_binding(directory_fd, &self.name, "existing receipt pathname")? != self.binding {
            return Err(invalid(
                "existing collector receipt pathname changed during final replay",
            ));
        }
        Ok(())
    }
}

impl ReceiptRootSnapshotV3 {
    fn revalidate(&self, directory: &File, path: &Path) -> Result<(), RestartCollectorErrorV3> {
        if list_directory(directory.as_raw_fd(), MAX_RECEIPT_FILES)? != self.roster
            || self.entries.len() != self.roster.len()
            || self
                .entries
                .iter()
                .try_fold(0usize, |total, entry| total.checked_add(entry.bytes.len()))
                != Some(self.aggregate_bytes)
            || self.aggregate_bytes > MAX_RECEIPT_AGGREGATE_BYTES
        {
            return Err(invalid(
                "collector receipt root changed after its closed-world capture",
            ));
        }
        let receipt_root = path_text(path, "receipt root")?;
        for entry in &self.entries {
            entry.revalidate(directory.as_raw_fd(), &receipt_root)?;
        }
        Ok(())
    }

    fn revalidate_entries(
        &self,
        directory_fd: RawFd,
        receipt_root: &str,
    ) -> Result<(), RestartCollectorErrorV3> {
        for entry in &self.entries {
            entry.revalidate(directory_fd, receipt_root)?;
        }
        Ok(())
    }
}

impl RetainedReceiptRootV3 {
    fn from_held(
        held: HeldDirectoryV3,
        stable_identity: StableDirectoryIdentityV3,
        initial_binding: FilesystemObjectBindingV3,
    ) -> Result<Self, RestartCollectorErrorV3> {
        held.revalidate("receipt root")?;
        validate_receipt_directory(&held.binding)?;
        let snapshot = capture_receipt_root_closed_world(&held)?;
        if !stable_identity.matches_binding(&held.binding, snapshot.roster.len())
            || !stable_root_object_matches(&stable_identity, &initial_binding)
            || validate_receipt_directory(&initial_binding).is_err()
        {
            return Err(invalid(
                "receipt-root generation differs from its prepared stable identity",
            ));
        }
        let retained = Self {
            current_binding: held.binding,
            directory: held.file,
            initial_binding,
            path: held.path,
            snapshot,
            stable_identity,
        };
        retained.revalidate()?;
        Ok(retained)
    }

    fn capture(
        path: &Path,
        stable_identity: StableDirectoryIdentityV3,
        initial_binding: FilesystemObjectBindingV3,
    ) -> Result<Self, RestartCollectorErrorV3> {
        Self::from_held(
            HeldDirectoryV3::capture(path, "receipt root")?,
            stable_identity,
            initial_binding,
        )
    }

    fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        let descriptor = fstat_binding(self.directory.as_raw_fd(), "receipt root")?;
        let named = lstat_binding(&self.path, "receipt root")?;
        if descriptor != self.current_binding
            || named != self.current_binding
            || !self
                .stable_identity
                .matches_binding(&descriptor, self.snapshot.roster.len())
        {
            return Err(invalid(
                "retained receipt-root generation changed identity or metadata",
            ));
        }
        validate_receipt_directory(&descriptor)?;
        verify_fd_binding_secure(self.directory.as_raw_fd(), &descriptor, "receipt root")?;
        self.snapshot.revalidate(&self.directory, &self.path)?;
        let after = fstat_binding(self.directory.as_raw_fd(), "receipt root")?;
        if after != descriptor || lstat_binding(&self.path, "receipt root")? != descriptor {
            return Err(invalid(
                "receipt-root generation changed during closed-world replay",
            ));
        }
        Ok(())
    }

    fn revalidate_retained_generation_chain(&self) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_generation_chain(None)
    }

    fn revalidate_unadopted_generation_chain(
        &self,
        expected_tail: &CollectorReceiptFileBindingV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_generation_chain(Some(expected_tail))
    }

    fn revalidate_generation_chain(
        &self,
        expected_tail: Option<&CollectorReceiptFileBindingV3>,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        let mut generations = self
            .snapshot
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let is_tail = index + 1 == self.snapshot.entries.len();
                let binding = match (
                    entry.lifecycle_binding.as_ref(),
                    entry.expected_lifecycle_binding.as_ref(),
                ) {
                    (Some(binding), None) => binding,
                    (None, Some(binding))
                        if is_tail && expected_tail.is_some_and(|tail| tail == binding) =>
                    {
                        binding
                    }
                    _ => {
                        return Err(invalid(
                            "receipt-root owner lost or ambiguously adopted a generation binding",
                        ));
                    }
                };
                if binding.final_basename() != entry.name
                    || binding.canonical_sha256() != sha256(&entry.bytes)
                    || binding.exact_binding() != entry.binding
                {
                    return Err(invalid(
                        "receipt-root owner generation differs from its exact receipt capsule",
                    ));
                }
                Ok(binding)
            })
            .collect::<Result<Vec<_>, RestartCollectorErrorV3>>()?;
        if expected_tail.is_some()
            != self.snapshot.entries.last().is_some_and(|entry| {
                entry.lifecycle_binding.is_none()
                    && entry.expected_lifecycle_binding.as_ref() == expected_tail
            })
        {
            return Err(invalid(
                "receipt-root owner unadopted tail does not match its one-shot delta",
            ));
        }
        generations.sort_by_key(|binding| binding.root_generation_ordinal());
        let mut prior_root = self.initial_binding;
        for (index, binding) in generations.into_iter().enumerate() {
            let root_after = binding.root_after();
            if usize::try_from(binding.root_generation_ordinal()).ok() != Some(index + 1)
                || !same_directory_object(prior_root, root_after)
                || prior_root.nlink.checked_add(1) != Some(root_after.nlink)
            {
                return Err(invalid(
                    "receipt-root owner retained generation chain is not exact and contiguous",
                ));
            }
            prior_root = root_after;
        }
        if prior_root != self.current_binding {
            return Err(invalid(
                "receipt-root owner current full endpoint differs from its retained chain",
            ));
        }
        self.revalidate()
    }

    fn adopt_unadopted_tail(
        &mut self,
        binding: CollectorReceiptFileBindingV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_unadopted_generation_chain(&binding)?;
        let entry = self
            .snapshot
            .entries
            .iter_mut()
            .find(|entry| {
                entry.expected_lifecycle_binding.as_ref() == Some(&binding)
                    && entry.lifecycle_binding.is_none()
            })
            .ok_or_else(|| invalid("unadopted receipt-root tail disappeared"))?;
        entry.expected_lifecycle_binding = None;
        entry.lifecycle_binding = Some(binding);
        self.revalidate_retained_generation_chain()
    }

    fn revalidate_lifecycle_records(
        &self,
        lifecycle_records: &[Vec<u8>],
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        self.revalidate_lifecycle_references_only(lifecycle_records)?;
        self.revalidate()
    }

    fn revalidate_lifecycle_references_only(
        &self,
        lifecycle_records: &[Vec<u8>],
    ) -> Result<(), RestartCollectorErrorV3> {
        let references = if lifecycle_records.is_empty() {
            Vec::new()
        } else {
            collector_receipt_file_roster_v3(lifecycle_records).map_err(|error| {
                invalid(format!(
                    "lifecycle collector-receipt roster failed replay: {error}"
                ))
            })?
        };
        if references.len() != self.snapshot.entries.len()
            || references.len() != self.snapshot.roster.len()
        {
            return Err(invalid(
                "lifecycle collector references and receipt-root roster are not bijective",
            ));
        }
        let mut prior_root = self.initial_binding;
        for (index, reference) in references.iter().enumerate() {
            let entry_index = self
                .snapshot
                .entries
                .binary_search_by(|entry| entry.name.as_str().cmp(reference.final_basename()))
                .map_err(|_| {
                    invalid("lifecycle collector reference has no exact retained receipt entry")
                })?;
            let entry = &self.snapshot.entries[entry_index];
            let root_after = reference.root_after();
            if usize::try_from(reference.root_generation_ordinal()).ok() != Some(index + 1)
                || !same_directory_object(prior_root, root_after)
                || prior_root.nlink.checked_add(1) != Some(root_after.nlink)
                || entry.name != reference.final_basename()
                || reference.canonical_sha256() != sha256(&entry.bytes)
                || reference.exact_binding() != entry.binding
                || entry
                    .lifecycle_binding
                    .as_ref()
                    .is_some_and(|retained| retained != reference)
                || entry
                    .expected_lifecycle_binding
                    .as_ref()
                    .is_some_and(|expected| expected != reference)
            {
                return Err(invalid(
                    "lifecycle collector reference differs from its exact retained receipt inode",
                ));
            }
            prior_root = root_after;
        }
        if prior_root != self.current_binding {
            return Err(invalid(
                "live receipt-root full binding differs from the last durable generation endpoint",
            ));
        }
        Ok(())
    }

    fn bind_lifecycle_records(
        &mut self,
        lifecycle_records: &[Vec<u8>],
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_lifecycle_records(lifecycle_records)?;
        let references = if lifecycle_records.is_empty() {
            Vec::new()
        } else {
            collector_receipt_file_roster_v3(lifecycle_records).map_err(|error| {
                invalid(format!(
                    "lifecycle collector-receipt roster failed replay: {error}"
                ))
            })?
        };
        for reference in references {
            let index = self
                .snapshot
                .entries
                .binary_search_by(|entry| entry.name.as_str().cmp(reference.final_basename()))
                .map_err(|_| invalid("lifecycle receipt binding disappeared during retention"))?;
            if self.snapshot.entries[index]
                .expected_lifecycle_binding
                .as_ref()
                .is_some_and(|expected| expected != &reference)
            {
                return Err(invalid(
                    "lifecycle binding differs from the unadopted receipt generation",
                ));
            }
            self.snapshot.entries[index].expected_lifecycle_binding = None;
            self.snapshot.entries[index].lifecycle_binding = Some(reference);
        }
        self.revalidate_lifecycle_records(lifecycle_records)
    }
}

impl RetainedCollectorReceiptRootOwnerV3 {
    fn from_root(root: RetainedReceiptRootV3) -> Self {
        Self {
            root,
            _not_send_or_sync: PhantomData,
        }
    }

    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.root.revalidate_retained_generation_chain()
    }

    fn revalidate_for_prepared(
        &self,
        manifest: &PreparedCollectorManifestV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        if self.root.stable_identity != manifest.policy.receipt_root_identity
            || self.root.path != Path::new(&manifest.policy.receipt_root)
            || manifest.receipt_root_initial_binding != Some(self.root.initial_binding)
        {
            return Err(invalid(
                "receipt-root owner differs from the exact prepared manifest",
            ));
        }
        validate_receipt_directory(&self.root.current_binding)
    }

    pub(crate) fn revalidate_lifecycle_records(
        &self,
        lifecycle_records: &[Vec<u8>],
    ) -> Result<(), RestartCollectorErrorV3> {
        self.root.revalidate_lifecycle_records(lifecycle_records)
    }

    pub(crate) fn bind_lifecycle_records(
        &mut self,
        lifecycle_records: &[Vec<u8>],
    ) -> Result<(), RestartCollectorErrorV3> {
        self.root.bind_lifecycle_records(lifecycle_records)
    }

    pub(crate) fn revalidate_observation(
        &self,
        observation: &RetainedCollectorObservationV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        observation.revalidate_bound()?;
        observation.evidence().durable.revalidate(&self.root)
    }

    pub(crate) fn revalidate_lineage(
        &self,
        lineage: &RetainedCollectorLineageV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        lineage.revalidate_bound()?;
        lineage.revalidate_receipt_entries(&self.root)?;
        self.revalidate()
    }

    fn revalidate_lineage_retained_capsules(
        &self,
        lineage: &RetainedCollectorLineageV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate()?;
        lineage.revalidate_retained_capsules()?;
        lineage.revalidate_receipt_entries(&self.root)?;
        self.revalidate()
    }

    pub(crate) fn revalidate_mount_delta<K>(
        &self,
        delta: &RetainedCollectorMountDeltaV3<K>,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_lineage(&delta.prior)
    }

    fn issue_binding<'a>(
        &'a self,
        lineage: &'a RetainedCollectorLineageV3,
    ) -> Result<RetainedCollectorIssueBindingV3<'a>, RestartCollectorErrorV3> {
        self.revalidate_lineage(lineage)?;
        lineage.issue_binding(self)
    }

    pub(crate) fn terminal_absence<'a>(
        &'a self,
        lineage: &'a RetainedCollectorLineageV3,
    ) -> Result<RetainedTerminalAbsenceV3<'a>, RestartCollectorErrorV3> {
        self.revalidate_lineage(lineage)?;
        lineage.terminal_absence(self)
    }

    pub(crate) fn into_sealed_unmount_effect_plan(
        self,
        lineage: RetainedCollectorLineageV3,
    ) -> Result<SealedUnmountEffectPlanV3, RestartCollectorErrorV3> {
        Ok(SealedUnmountEffectPlanV3 {
            core: SealedCollectorEffectPlanCoreV3::new(
                self,
                lineage,
                SealedCollectorEffectPlanKindV3::Unmount,
            )?,
        })
    }

    pub(crate) fn into_sealed_eject_effect_plan(
        self,
        lineage: RetainedCollectorLineageV3,
    ) -> Result<SealedEjectEffectPlanV3, RestartCollectorErrorV3> {
        Ok(SealedEjectEffectPlanV3 {
            core: SealedCollectorEffectPlanCoreV3::new(
                self,
                lineage,
                SealedCollectorEffectPlanKindV3::Eject,
            )?,
        })
    }
}

impl SealedCollectorEffectPlanCoreV3 {
    fn new(
        receipt_root_owner: RetainedCollectorReceiptRootOwnerV3,
        lineage: RetainedCollectorLineageV3,
        kind: SealedCollectorEffectPlanKindV3,
    ) -> Result<Self, RestartCollectorErrorV3> {
        let derived = derive_collector_effect_plan(&receipt_root_owner, &lineage, kind)?;
        let plan = Self {
            command: derived.command,
            command_canonical_bytes: derived.command_canonical_bytes,
            command_sha256: derived.command_sha256,
            lineage,
            provenance: derived.provenance,
            receipt_root_owner,
            _not_send_or_sync: PhantomData,
        };
        plan.revalidate(kind)?;
        Ok(plan)
    }

    fn revalidate(
        &self,
        kind: SealedCollectorEffectPlanKindV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        let derived = derive_collector_effect_plan(&self.receipt_root_owner, &self.lineage, kind)?;
        validate_derived_collector_effect_plan(
            &self.command,
            &self.command_canonical_bytes,
            &self.command_sha256,
            &self.provenance,
            &derived,
        )
    }

    fn issue_plan<K>(
        &self,
        kind: SealedCollectorEffectPlanKindV3,
    ) -> Result<SealedCollectorEffectIssuePlanV3<'_, K>, RestartCollectorErrorV3> {
        self.revalidate(kind)?;
        let collector_binding = self.receipt_root_owner.issue_binding(&self.lineage)?;
        let issue = SealedCollectorEffectIssuePlanV3 {
            collector_binding,
            command: &self.command,
            plan: self,
            _kind: PhantomData,
            _not_send_or_sync: PhantomData,
        };
        issue.revalidate_inner(kind)?;
        Ok(issue)
    }
}

impl SealedUnmountEffectPlanV3 {
    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.core
            .revalidate(SealedCollectorEffectPlanKindV3::Unmount)
    }

    pub(crate) fn issue_plan(
        &self,
        _seal: &IssuePlanReadSealV3,
    ) -> Result<
        SealedCollectorEffectIssuePlanV3<'_, PersistedUnmountEffectV3>,
        RestartCollectorErrorV3,
    > {
        self.core
            .issue_plan(SealedCollectorEffectPlanKindV3::Unmount)
    }

    /// Recover the exact mount-delta owner after the issued runner has been
    /// positively death-proved.  This consumes the whole effect plan and uses
    /// only its internally derived command; no caller command or digest enters
    /// the transition.
    pub(crate) fn into_unmount_delta(
        self,
        _seal: &SuccessfulIssuedEffectTransitionSealV3,
    ) -> Result<
        (
            RetainedCollectorReceiptRootOwnerV3,
            RetainedCollectorMountDeltaV3<UnmountingV3>,
        ),
        RestartCollectorErrorV3,
    > {
        self.revalidate()?;
        let SealedCollectorEffectPlanCoreV3 {
            command,
            lineage,
            receipt_root_owner,
            ..
        } = self.core;
        let delta = lineage.into_unmount_delta(&command)?;
        receipt_root_owner.revalidate_mount_delta(&delta)?;
        Ok((receipt_root_owner, delta))
    }
}

impl SealedEjectEffectPlanV3 {
    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.core.revalidate(SealedCollectorEffectPlanKindV3::Eject)
    }

    pub(crate) fn issue_plan(
        &self,
        _seal: &IssuePlanReadSealV3,
    ) -> Result<SealedCollectorEffectIssuePlanV3<'_, PersistedEjectEffectV3>, RestartCollectorErrorV3>
    {
        self.core.issue_plan(SealedCollectorEffectPlanKindV3::Eject)
    }

    /// Consume the pre-effect live plan after exact issue persistence and
    /// before dispatch.  The returned expectation permits only the retained
    /// before inventory or its exact derived expected-after endpoint; no
    /// positive path can later demand that ejected IOMedia remain live.
    pub(crate) fn into_armed_expectation(
        self,
        _seal: &EjectExpectationArmSealV3,
    ) -> Result<ArmedEjectExpectationV3, RestartCollectorErrorV3> {
        self.revalidate()?;
        let SealedCollectorEffectPlanCoreV3 {
            command,
            command_canonical_bytes,
            command_sha256,
            lineage,
            provenance,
            receipt_root_owner,
            _not_send_or_sync: _,
        } = self.core;
        let ExactDisposableCommandV3::EjectImage {
            disk_image_group_sha256,
        } = &command
        else {
            return Err(invalid(
                "sealed eject plan changed command kind before arming",
            ));
        };
        if canonical_json(&command)? != command_canonical_bytes
            || sha256(&command_canonical_bytes) != command_sha256
        {
            return Err(invalid(
                "sealed eject command changed before expectation arming",
            ));
        }
        let binding = derive_eject_expectation_binding(&lineage, &command_sha256, &provenance)?;
        if disk_image_group_sha256 != &binding.disk_image_group_sha256 {
            return Err(invalid(
                "eject command group differs from its armed expected-after inventory",
            ));
        }
        let armed = ArmedEjectExpectationV3 {
            core: ArmedEjectExpectationCoreV3 {
                binding,
                lineage,
                _not_send_or_sync: PhantomData,
            },
            receipt_root_owner,
            _not_send_or_sync: PhantomData,
        };
        armed.revalidate_pending()?;
        Ok(armed)
    }
}

impl<K> SealedCollectorEffectIssuePlanV3<'_, K> {
    fn revalidate_inner(
        &self,
        kind: SealedCollectorEffectPlanKindV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.plan.revalidate(kind)?;
        self.collector_binding.revalidate()?;
        if canonical_json(self.command)? != self.plan.command_canonical_bytes
            || sha256(&self.plan.command_canonical_bytes) != self.plan.command_sha256
        {
            return Err(invalid(
                "sealed collector effect issue command changed after plan derivation",
            ));
        }
        Ok(())
    }

    pub(crate) fn command_for_issue(
        &self,
        _seal: &IssuePlanReadSealV3,
    ) -> &ExactDisposableCommandV3 {
        self.command
    }

    pub(crate) fn collector_binding_for_issue(
        &self,
        _seal: &IssuePlanReadSealV3,
    ) -> &RetainedCollectorIssueBindingV3<'_> {
        &self.collector_binding
    }
}

impl SealedCollectorEffectIssuePlanV3<'_, PersistedUnmountEffectV3> {
    pub(crate) fn revalidate_for_issue(
        &self,
        _seal: &IssuePlanReadSealV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_inner(SealedCollectorEffectPlanKindV3::Unmount)
    }
}

impl SealedCollectorEffectIssuePlanV3<'_, PersistedEjectEffectV3> {
    pub(crate) fn revalidate_for_issue(
        &self,
        _seal: &IssuePlanReadSealV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_inner(SealedCollectorEffectPlanKindV3::Eject)
    }
}

impl S1RetainedCollectorReceiptRootV3 {
    pub(crate) fn retained_descriptor_count(&self) -> usize {
        1 + self.root.snapshot.entries.len()
    }

    pub(crate) fn retained_bytes(&self) -> usize {
        self.root.snapshot.aggregate_bytes
    }

    pub(crate) fn capture_from_exact_lifecycle(
        _seal: &S1CollectorReceiptRegistrySealV3,
        prepared_manifest_bytes: &[u8],
        lifecycle_records: &[Vec<u8>],
    ) -> Result<Self, RestartCollectorErrorV3> {
        let manifest: PreparedCollectorManifestV3 = serde_json::from_slice(prepared_manifest_bytes)
            .map_err(|error| {
                invalid(format!(
                    "S1 prepared collector manifest JSON failed: {error}"
                ))
            })?;
        validate_prepared_manifest(&manifest)?;
        let mut root = RetainedReceiptRootV3::capture(
            Path::new(&manifest.policy.receipt_root),
            manifest.policy.receipt_root_identity,
            manifest.receipt_root_initial_binding.ok_or_else(|| {
                invalid("legacy prepared manifest has no durable initial receipt-root endpoint")
            })?,
        )?;
        root.bind_lifecycle_records(lifecycle_records)?;
        Ok(Self {
            root,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn revalidate(
        &self,
        lifecycle_records: &[Vec<u8>],
    ) -> Result<(), RestartCollectorErrorV3> {
        self.root.revalidate_lifecycle_records(lifecycle_records)
    }

    pub(crate) fn preflight_exact_append(
        &mut self,
        before_lifecycle: &[Vec<u8>],
        before: FilesystemObjectBindingV3,
        after: FilesystemObjectBindingV3,
        binding: CollectorReceiptFileBindingV3,
        new_file: File,
        bytes: Vec<u8>,
    ) -> Result<S1CollectorReceiptAppendCommitV3, RestartCollectorErrorV3> {
        if self.root.current_binding != before
            || before == after
            || !same_directory_object(before, after)
            || before.nlink.checked_add(1) != Some(after.nlink)
            || binding.root_after() != after
            || usize::try_from(binding.root_generation_ordinal()).ok()
                != Some(self.root.snapshot.roster.len() + 1)
            || binding.canonical_sha256() != sha256(&bytes)
            || binding.final_basename() != format!("collector-{}.json", binding.canonical_sha256())
        {
            return Err(invalid(
                "S1 collector receipt transfer is not the exact next root generation",
            ));
        }
        let live_root = fstat_binding(self.root.directory.as_raw_fd(), "S1 receipt root")?;
        if live_root != after
            || lstat_binding(&self.root.path, "S1 receipt root")? != after
            || !self
                .root
                .stable_identity
                .matches_binding(&after, self.root.snapshot.roster.len() + 1)
        {
            return Err(invalid(
                "S1 retained receipt root differs from the transferred successor endpoint",
            ));
        }
        validate_receipt_directory(&after)?;
        verify_fd_binding_secure(self.root.directory.as_raw_fd(), &after, "S1 receipt root")?;
        let root_text = path_text(&self.root.path, "S1 receipt root")?;
        self.root
            .snapshot
            .revalidate_entries(self.root.directory.as_raw_fd(), &root_text)?;
        self.root
            .revalidate_lifecycle_references_only(before_lifecycle)?;
        let exact_binding = binding.exact_binding();
        let held = fstat_binding(new_file.as_raw_fd(), "S1 transferred collector receipt")?;
        if held != exact_binding
            || fstatat_binding(
                self.root.directory.as_raw_fd(),
                binding.final_basename(),
                "S1 transferred collector receipt pathname",
            )? != exact_binding
            || read_fd_exact(&new_file, &held)? != bytes
            || self.root.snapshot.entries.iter().any(|entry| {
                entry.name == binding.final_basename()
                    || (entry.binding.dev, entry.binding.inode)
                        == (exact_binding.dev, exact_binding.inode)
            })
        {
            return Err(invalid(
                "S1 transferred collector receipt differs from its final exact inode",
            ));
        }
        verify_fd_binding_secure(
            new_file.as_raw_fd(),
            &exact_binding,
            "S1 transferred collector receipt",
        )?;
        let receipt: RestartCollectorReceiptV3 = serde_json::from_slice(&bytes)
            .map_err(|error| invalid(format!("S1 transferred receipt JSON failed: {error}")))?;
        if canonical_json(&receipt)? != bytes || receipt.collector_policy.receipt_root != root_text
        {
            return Err(invalid(
                "S1 transferred collector receipt is noncanonical or belongs to another root",
            ));
        }
        validate_receipt(&receipt)?;
        validate_fresh_receipt_prior_relationship(&receipt, &self.root.snapshot.entries)?;
        let new_name = binding.final_basename().to_string();
        let entry_insert_index = self
            .root
            .snapshot
            .entries
            .binary_search_by(|entry| entry.name.cmp(&new_name))
            .err()
            .ok_or_else(|| invalid("S1 receipt-root successor duplicates an existing entry"))?;
        let mut next_roster = self.root.snapshot.roster.clone();
        next_roster.push(new_name.clone());
        next_roster.sort();
        if next_roster.windows(2).any(|pair| pair[0] == pair[1])
            || list_directory(self.root.directory.as_raw_fd(), MAX_RECEIPT_FILES)? != next_roster
        {
            return Err(invalid(
                "S1 receipt-root successor roster is not old roster plus one exact receipt",
            ));
        }
        let next_aggregate_bytes =
            checked_receipt_aggregate_bytes(self.root.snapshot.aggregate_bytes, bytes.len())?;
        self.root
            .snapshot
            .entries
            .try_reserve(1)
            .map_err(|error| invalid(format!("S1 receipt entry reserve failed: {error}")))?;
        self.root
            .snapshot
            .roster
            .try_reserve(1)
            .map_err(|error| invalid(format!("S1 receipt roster reserve failed: {error}")))?;
        Ok(S1CollectorReceiptAppendCommitV3 {
            after_binding: after,
            entry: ValidatedExistingReceiptV3 {
                binding: exact_binding,
                bytes,
                expected_lifecycle_binding: None,
                file: new_file,
                lifecycle_binding: Some(binding),
                name: new_name,
                receipt,
            },
            entry_insert_index,
            next_aggregate_bytes,
            next_roster,
            _not_send_or_sync: PhantomData,
        })
    }

    /// No allocation, syscall, serialization, or replay is permitted here.
    /// All fallible work belongs to `preflight_exact_append`.
    pub(crate) fn commit_exact_append(&mut self, plan: S1CollectorReceiptAppendCommitV3) {
        self.root.snapshot.aggregate_bytes = plan.next_aggregate_bytes;
        self.root.snapshot.roster = plan.next_roster;
        self.root
            .snapshot
            .entries
            .insert(plan.entry_insert_index, plan.entry);
        self.root.current_binding = plan.after_binding;
    }

    pub(crate) fn s1_census_projection(
        &self,
        _seal: &S1CollectorReceiptRegistrySealV3,
    ) -> (PathBuf, u64, u64, Vec<(String, u64, u64)>) {
        (
            self.root.path.clone(),
            self.root.current_binding.dev,
            self.root.current_binding.inode,
            self.root
                .snapshot
                .entries
                .iter()
                .map(|entry| (entry.name.clone(), entry.binding.dev, entry.binding.inode))
                .collect(),
        )
    }

    pub(crate) fn s1_initial_binding(
        &self,
        _seal: &S1CollectorReceiptRegistrySealV3,
    ) -> FilesystemObjectBindingV3 {
        self.root.initial_binding
    }
}

fn capture_receipt_root_closed_world(
    directory: &HeldDirectoryV3,
) -> Result<ReceiptRootSnapshotV3, RestartCollectorErrorV3> {
    directory.revalidate("receipt root")?;
    validate_receipt_directory(&directory.binding)?;
    let roster = list_directory(directory.file.as_raw_fd(), MAX_RECEIPT_FILES)?;
    if roster
        .iter()
        .any(|name| name.starts_with(".incoming-collector-") || !valid_final_receipt_name(name))
    {
        return Err(invalid(
            "collector receipt root contains a noncanonical or uncertain entry",
        ));
    }
    let receipt_root = path_text(&directory.path, "receipt root")?;
    let mut aggregate_bytes = 0usize;
    let mut entries = Vec::with_capacity(roster.len());
    for name in &roster {
        let file = openat_existing_regular(directory.file.as_raw_fd(), name)?;
        let binding = fstat_binding(file.as_raw_fd(), "existing collector receipt")?;
        let size = usize::try_from(binding.size)
            .ok()
            .filter(|size| *size <= MAX_RECEIPT_BYTES)
            .ok_or_else(|| invalid("existing collector receipt exceeds its size bound"))?;
        aggregate_bytes = checked_receipt_aggregate_bytes(aggregate_bytes, size)?;
        validate_receipt_file(&binding, directory.binding.uid, directory.binding.gid, size)?;
        verify_fd_binding_secure(file.as_raw_fd(), &binding, "existing collector receipt")?;
        if fstatat_binding(
            directory.file.as_raw_fd(),
            name,
            "existing collector receipt pathname",
        )? != binding
        {
            return Err(invalid(
                "existing collector receipt descriptor differs from its pathname",
            ));
        }
        let bytes = read_fd_exact(&file, &binding)?;
        let digest = receipt_name_digest(name)
            .ok_or_else(|| invalid("existing collector receipt name is malformed"))?;
        if sha256(&bytes) != digest {
            return Err(invalid(
                "existing collector receipt content digest differs from its filename",
            ));
        }
        let receipt: RestartCollectorReceiptV3 = serde_json::from_slice(&bytes)
            .map_err(|error| invalid(format!("existing collector receipt JSON failed: {error}")))?;
        if canonical_json(&receipt)? != bytes
            || receipt.collector_policy.receipt_root != receipt_root
        {
            return Err(invalid(
                "existing collector receipt is noncanonical or belongs to another root",
            ));
        }
        validate_receipt(&receipt)?;
        verify_fd_binding_secure(file.as_raw_fd(), &binding, "existing collector receipt")?;
        if fstatat_binding(
            directory.file.as_raw_fd(),
            name,
            "existing collector receipt pathname",
        )? != binding
        {
            return Err(invalid(
                "existing collector receipt changed during closed-world validation",
            ));
        }
        entries.push(ValidatedExistingReceiptV3 {
            binding,
            bytes,
            expected_lifecycle_binding: None,
            file,
            lifecycle_binding: None,
            name: name.clone(),
            receipt,
        });
    }
    for entry in &entries {
        validate_fresh_receipt_prior_relationship(&entry.receipt, &entries)?;
    }
    directory.revalidate("receipt root")?;
    if list_directory(directory.file.as_raw_fd(), MAX_RECEIPT_FILES)? != roster {
        return Err(invalid(
            "collector receipt roster changed during closed-world validation",
        ));
    }
    Ok(ReceiptRootSnapshotV3 {
        aggregate_bytes,
        entries,
        roster,
    })
}

fn validate_fresh_receipt_prior_relationship(
    receipt: &RestartCollectorReceiptV3,
    existing: &[ValidatedExistingReceiptV3],
) -> Result<(), RestartCollectorErrorV3> {
    if receipt.purpose != CollectorPurposeV3::FreshAbsence {
        return Ok(());
    }
    let expected = receipt
        .reconciliation_snapshot_sha256
        .as_ref()
        .ok_or_else(|| invalid("FreshAbsence receipt omits its prior snapshot digest"))?;
    let mut matches = 0usize;
    for candidate in existing {
        if candidate.receipt.purpose != CollectorPurposeV3::ReconciliationSnapshot {
            continue;
        }
        let Some(receipt_sha256) = receipt_name_digest(&candidate.name) else {
            continue;
        };
        let snapshot = reconciliation_snapshot_from_receipt(
            &candidate.receipt,
            receipt_sha256,
            candidate.lifecycle_binding()?,
        )?;
        let snapshot_sha256 = reconciliation_snapshot_sha256(&snapshot)
            .map_err(|error| invalid(format!("prior snapshot digest failed: {error}")))?;
        let exact_expected_absence = match (
            candidate
                .receipt
                .current_expected_absence_inventory
                .as_ref(),
            candidate
                .receipt
                .current_expected_absence_inventory_sha256
                .as_deref(),
        ) {
            (Some(expected_inventory), Some(expected_sha256)) => {
                validate_exact_expected_absence_inventory(
                    expected_inventory,
                    expected_sha256,
                    &receipt.iomedia_inventory,
                    &receipt.iomedia_evidence_sha256,
                )
                .is_ok()
            }
            _ => false,
        };
        if &snapshot_sha256 == expected
            && snapshot.operation_nonce == receipt.operation_nonce
            && snapshot.restart_epoch_nonce == receipt.restart_epoch_nonce
            && snapshot.boot_session_uuid == receipt.boot_session_uuid
            && snapshot.collector_policy_sha256 == receipt.collector_policy_sha256
            && snapshot.backing_identity_sha256 == receipt.backing_identity_sha256
            && snapshot.mountpoint_underlying_sha256 == receipt.mountpoint_underlying_sha256
            && candidate.receipt.baseline_inventory_sha256 == receipt.baseline_inventory_sha256
            && exact_expected_absence
            && snapshot.monotonic_after_nanoseconds < receipt.monotonic_before_nanoseconds
        {
            matches += 1;
        }
    }
    if matches != 1 {
        return Err(invalid(
            "FreshAbsence receipt does not have exactly one durable prior snapshot projection",
        ));
    }
    Ok(())
}

fn validate_exact_expected_absence_inventory(
    expected_inventory: &RestartIOMediaInventoryV3,
    expected_sha256: &str,
    current_inventory: &RestartIOMediaInventoryV3,
    current_sha256: &str,
) -> Result<(), RestartCollectorErrorV3> {
    validate_restart_iomedia_inventory_v3(expected_inventory)?;
    validate_restart_iomedia_inventory_v3(current_inventory)?;
    if !valid_digest(expected_sha256)
        || !valid_digest(current_sha256)
        || sha256(&canonical_json(expected_inventory)?) != expected_sha256
        || sha256(&canonical_json(current_inventory)?) != current_sha256
        || expected_inventory != current_inventory
        || expected_sha256 != current_sha256
    {
        return Err(invalid(
            "current inventory differs from the exact full expected-absence inventory",
        ));
    }
    Ok(())
}

fn validate_prior_snapshot_receipt(
    snapshot: &ReconciliationSnapshotV2,
    baseline_inventory_sha256: &str,
    receipts: &ReceiptRootSnapshotV3,
) -> Result<RestartIOMediaInventoryV3, RestartCollectorErrorV3> {
    validate_reconciliation_snapshot_shape_v3(snapshot)?;
    if !valid_digest(baseline_inventory_sha256) {
        return Err(invalid(
            "prior reconciliation baseline binding is malformed",
        ));
    }
    let name = format!("collector-{}.json", snapshot.collector_receipt_sha256);
    let entry = receipts
        .entries
        .iter()
        .find(|entry| entry.name == name)
        .ok_or_else(|| invalid("prior reconciliation snapshot receipt is absent"))?;
    if entry.receipt.baseline_inventory_sha256 != baseline_inventory_sha256
        || reconciliation_snapshot_from_receipt(
            &entry.receipt,
            &snapshot.collector_receipt_sha256,
            entry.lifecycle_binding()?,
        )? != *snapshot
    {
        return Err(invalid(
            "prior durable reconciliation receipt does not match the baseline or supplied snapshot",
        ));
    }
    let expected = entry
        .receipt
        .current_expected_absence_inventory
        .as_ref()
        .ok_or_else(|| invalid("prior receipt has no exact current-boot expected absence"))?;
    let expected_sha256 = entry
        .receipt
        .current_expected_absence_inventory_sha256
        .as_deref()
        .ok_or_else(|| invalid("prior receipt has no expected-absence digest"))?;
    validate_restart_iomedia_inventory_v3(expected)?;
    if sha256(&canonical_json(expected)?) != expected_sha256 {
        return Err(invalid(
            "prior current-boot expected absence differs from its durable digest",
        ));
    }
    Ok(expected.clone())
}

impl DurableCollectorReceiptV3 {
    fn lifecycle_binding(&self) -> CollectorReceiptFileBindingV3 {
        CollectorReceiptFileBindingV3::from_retained_collector(
            CollectorReceiptFileBindingSealV3 { _private: () },
            sha256(&self.bytes),
            self.final_name.clone(),
            self.file_binding,
            self.root_after_binding,
            self.root_generation_ordinal,
        )
    }

    fn persist(
        mut root: RetainedReceiptRootV3,
        receipt: &RestartCollectorReceiptV3,
        bytes: Vec<u8>,
        receipt_sha256: &str,
    ) -> Result<(Self, RetainedReceiptRootV3), RestartCollectorErrorV3> {
        root.revalidate()?;
        let existing_receipts = &root.snapshot;
        validate_receipt(receipt)?;
        validate_fresh_receipt_prior_relationship(receipt, &existing_receipts.entries)?;
        if sha256(&bytes) != receipt_sha256 || canonical_json(receipt)? != bytes {
            return Err(invalid(
                "collector persistence input differs from its canonical receipt",
            ));
        }
        if checked_receipt_aggregate_bytes(existing_receipts.aggregate_bytes, bytes.len()).is_err()
            || existing_receipts.roster.len() >= MAX_RECEIPT_FILES
        {
            return Err(invalid(
                "collector receipt persistence exceeds its aggregate byte or descriptor budget",
            ));
        }
        root.revalidate_retained_generation_chain()?;
        let path = root.path.clone();
        let directory = root.directory.try_clone()?;
        let before = fstat_binding(directory.as_raw_fd(), "collector receipt directory")?;
        if before != root.current_binding {
            return Err(invalid(
                "collector receipt directory changed before persistence",
            ));
        }
        validate_receipt_directory(&before)?;
        verify_fd_binding_secure(
            directory.as_raw_fd(),
            &before,
            "collector receipt directory",
        )?;
        let roster_before = root.snapshot.roster.clone();
        existing_receipts
            .revalidate_entries(directory.as_raw_fd(), &path_text(&path, "receipt root")?)?;
        let final_name = format!("collector-{receipt_sha256}.json");
        let temporary_name = format!(".incoming-collector-{receipt_sha256}");
        if roster_before.binary_search(&final_name).is_ok()
            || roster_before.binary_search(&temporary_name).is_ok()
        {
            return Err(invalid(
                "collector receipt already exists or has uncertain persistence",
            ));
        }
        let temporary = openat_new_private(
            directory.as_raw_fd(),
            &temporary_name,
            before.uid,
            before.gid,
        )?;
        let mut temporary = temporary;
        temporary.write_all(&bytes)?;
        temporary.sync_all()?;
        let temporary_binding = fstat_binding(temporary.as_raw_fd(), "temporary receipt")?;
        validate_receipt_file(&temporary_binding, before.uid, before.gid, bytes.len())?;
        verify_fd_binding_secure(
            temporary.as_raw_fd(),
            &temporary_binding,
            "temporary receipt",
        )?;
        if fstatat_binding(
            directory.as_raw_fd(),
            &temporary_name,
            "temporary receipt pathname",
        )? != temporary_binding
            || read_fd_exact(&temporary, &temporary_binding)? != bytes
        {
            return Err(invalid("temporary collector receipt bytes changed"));
        }
        let from = CString::new(temporary_name.as_str())
            .map_err(|_| invalid("temporary collector receipt name contains NUL"))?;
        let to = CString::new(final_name.as_str())
            .map_err(|_| invalid("final collector receipt name contains NUL"))?;
        if unsafe {
            renameatx_np(
                directory.as_raw_fd(),
                from.as_ptr(),
                directory.as_raw_fd(),
                to.as_ptr(),
                RENAME_EXCL,
            )
        } != 0
        {
            return Err(io::Error::last_os_error().into());
        }
        directory.sync_all()?;
        let file = openat_existing_regular(directory.as_raw_fd(), &final_name)?;
        let file_binding = fstat_binding(file.as_raw_fd(), "final collector receipt")?;
        let renamed_binding = fstat_binding(temporary.as_raw_fd(), "renamed temporary receipt")?;
        validate_receipt_file(&file_binding, before.uid, before.gid, bytes.len())?;
        validate_receipt_file(&renamed_binding, before.uid, before.gid, bytes.len())?;
        verify_fd_binding_secure(
            temporary.as_raw_fd(),
            &renamed_binding,
            "renamed temporary receipt",
        )?;
        verify_fd_binding_secure(file.as_raw_fd(), &file_binding, "final collector receipt")?;
        if !same_receipt_object_across_rename(temporary_binding, renamed_binding)
            || file_binding != renamed_binding
            || fstatat_binding(
                directory.as_raw_fd(),
                &final_name,
                "final collector receipt pathname",
            )? != file_binding
            || read_fd_exact(&temporary, &renamed_binding)? != bytes
            || read_fd_exact(&file, &file_binding)? != bytes
        {
            return Err(invalid(
                "final collector receipt bytes changed after rename",
            ));
        }
        let mut roster = roster_before;
        roster.push(final_name.clone());
        roster.sort();
        if list_directory(directory.as_raw_fd(), MAX_RECEIPT_FILES)? != roster {
            return Err(invalid(
                "collector receipt directory roster changed during persistence",
            ));
        }
        let after = fstat_binding(directory.as_raw_fd(), "collector receipt directory")?;
        if !same_directory_object(before, after) || after != lstat_binding(&path, "receipt root")? {
            return Err(invalid(
                "collector receipt directory changed identity during persistence",
            ));
        }
        verify_fd_binding_secure(directory.as_raw_fd(), &after, "collector receipt directory")?;
        if lstat_binding(&path, "receipt root")? != after {
            return Err(invalid(
                "collector receipt directory pathname changed during ACL/xattr replay",
            ));
        }
        root.snapshot
            .revalidate_entries(directory.as_raw_fd(), &path_text(&path, "receipt root")?)?;
        let root_entry_file = file.try_clone()?;
        let decoded: RestartCollectorReceiptV3 = serde_json::from_slice(&bytes)
            .map_err(|error| invalid(format!("final collector receipt JSON failed: {error}")))?;
        root.snapshot.aggregate_bytes =
            checked_receipt_aggregate_bytes(root.snapshot.aggregate_bytes, bytes.len())?;
        let expected_lifecycle_binding = CollectorReceiptFileBindingV3::from_retained_collector(
            CollectorReceiptFileBindingSealV3 { _private: () },
            receipt_sha256.to_string(),
            final_name.clone(),
            file_binding,
            after,
            u32::try_from(root.snapshot.roster.len() + 1)
                .map_err(|_| invalid("collector receipt generation ordinal exceeds u32"))?,
        );
        root.snapshot.entries.push(ValidatedExistingReceiptV3 {
            binding: file_binding,
            bytes: bytes.clone(),
            expected_lifecycle_binding: Some(expected_lifecycle_binding.clone()),
            file: root_entry_file,
            lifecycle_binding: None,
            name: final_name.clone(),
            receipt: decoded,
        });
        root.snapshot
            .entries
            .sort_by(|left, right| left.name.cmp(&right.name));
        root.snapshot.roster = roster;
        root.current_binding = after;
        root.revalidate()?;
        let durable = Self {
            bytes,
            directory,
            directory_identity: root.stable_identity,
            file,
            file_binding,
            final_name,
            path,
            root_after_binding: after,
            root_generation_ordinal: u32::try_from(root.snapshot.roster.len())
                .map_err(|_| invalid("collector receipt generation ordinal exceeds u32"))?,
        };
        durable.revalidate_unadopted(&root, &expected_lifecycle_binding)?;
        Ok((durable, root))
    }

    fn revalidate(&self, owner: &RetainedReceiptRootV3) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_against_owner(owner, None)
    }

    fn revalidate_unadopted(
        &self,
        owner: &RetainedReceiptRootV3,
        expected_lifecycle_binding: &CollectorReceiptFileBindingV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        if self.lifecycle_binding() != *expected_lifecycle_binding {
            return Err(invalid(
                "unadopted collector receipt differs from its expected lifecycle projection",
            ));
        }
        self.revalidate_against_owner(owner, Some(expected_lifecycle_binding))
    }

    fn revalidate_against_owner(
        &self,
        owner: &RetainedReceiptRootV3,
        expected_lifecycle_binding: Option<&CollectorReceiptFileBindingV3>,
    ) -> Result<(), RestartCollectorErrorV3> {
        match expected_lifecycle_binding {
            Some(expected) => owner.revalidate_unadopted_generation_chain(expected)?,
            None => owner.revalidate_retained_generation_chain()?,
        }
        if owner.path != self.path
            || owner.stable_identity != self.directory_identity
            || self.root_generation_ordinal == 0
        {
            return Err(invalid(
                "collector receipt root changed identity before capsule replay",
            ));
        }
        let lifecycle_binding = self.lifecycle_binding();
        let owner_matches = owner.snapshot.entries.iter().filter(|entry| {
            entry.name == self.final_name
                && entry.binding == self.file_binding
                && entry.bytes == self.bytes
                && match expected_lifecycle_binding {
                    Some(expected) => {
                        expected == &lifecycle_binding
                            && entry.lifecycle_binding.is_none()
                            && entry.expected_lifecycle_binding.as_ref() == Some(expected)
                    }
                    None => {
                        entry.expected_lifecycle_binding.is_none()
                            && entry.lifecycle_binding.as_ref() == Some(&lifecycle_binding)
                    }
                }
        });
        if owner_matches.count() != 1 {
            return Err(invalid(
                "unique receipt-root owner does not retain this exact historical capsule",
            ));
        }
        self.revalidate_entry()?;
        match expected_lifecycle_binding {
            Some(expected) => owner.revalidate_unadopted_generation_chain(expected),
            None => owner.revalidate_retained_generation_chain(),
        }
    }

    /// Replay only this immutable receipt capsule and its stable directory
    /// anchor. The current full directory endpoint is deliberately owned and
    /// checked by `RetainedCollectorReceiptRootOwnerV3`; a later legitimate
    /// generation must not invalidate an earlier observation.
    fn revalidate_entry(&self) -> Result<(), RestartCollectorErrorV3> {
        let directory = fstat_binding(self.directory.as_raw_fd(), "collector receipt directory")?;
        let named_directory = lstat_binding(&self.path, "receipt root")?;
        if !stable_root_object_matches(&self.directory_identity, &directory)
            || !same_directory_object(directory, named_directory)
            || !stable_root_object_matches(&self.directory_identity, &self.root_after_binding)
            || !same_directory_object(self.root_after_binding, directory)
            || self.root_generation_ordinal == 0
        {
            return Err(invalid(
                "collector receipt root changed identity before capsule replay",
            ));
        }
        verify_fd_binding_secure(
            self.directory.as_raw_fd(),
            &directory,
            "collector receipt directory",
        )?;
        let receipt_root = path_text(&self.path, "receipt root")?;
        let named = fstatat_binding(
            self.directory.as_raw_fd(),
            &self.final_name,
            "collector receipt pathname",
        )?;
        let held = fstat_binding(self.file.as_raw_fd(), "held collector receipt")?;
        if named != self.file_binding
            || held != self.file_binding
            || read_fd_exact(&self.file, &held)? != self.bytes
        {
            return Err(invalid(
                "durable collector receipt failed exact final replay",
            ));
        }
        verify_fd_binding_secure(
            self.file.as_raw_fd(),
            &self.file_binding,
            "held collector receipt",
        )?;
        if fstatat_binding(
            self.directory.as_raw_fd(),
            &self.final_name,
            "collector receipt pathname",
        )? != self.file_binding
            || receipt_name_digest(&self.final_name)
                .is_none_or(|digest| sha256(&self.bytes) != digest)
        {
            return Err(invalid(
                "durable collector receipt filename or binding changed",
            ));
        }
        let receipt: RestartCollectorReceiptV3 = serde_json::from_slice(&self.bytes)
            .map_err(|error| invalid(format!("final collector receipt JSON failed: {error}")))?;
        if canonical_json(&receipt)? != self.bytes
            || receipt.collector_policy.receipt_root != receipt_root
        {
            return Err(invalid(
                "final collector receipt is noncanonical or belongs to another root",
            ));
        }
        validate_receipt(&receipt)?;
        verify_fd_binding_secure(
            self.directory.as_raw_fd(),
            &directory,
            "collector receipt directory",
        )?;
        let directory_after = fstat_binding(self.directory.as_raw_fd(), "receipt root")?;
        if !stable_root_object_matches(&self.directory_identity, &directory_after)
            || !same_directory_object(directory_after, lstat_binding(&self.path, "receipt root")?)
        {
            return Err(invalid(
                "collector receipt directory changed across final replay",
            ));
        }
        Ok(())
    }
}

fn receipt_name_digest(value: &str) -> Option<&str> {
    value
        .strip_prefix("collector-")
        .and_then(|value| value.strip_suffix(".json"))
        .filter(|value| valid_digest(value))
}

fn valid_final_receipt_name(value: &str) -> bool {
    receipt_name_digest(value).is_some()
}

fn checked_receipt_aggregate_bytes(
    current: usize,
    additional: usize,
) -> Result<usize, RestartCollectorErrorV3> {
    current
        .checked_add(additional)
        .filter(|total| *total <= MAX_RECEIPT_AGGREGATE_BYTES)
        .ok_or_else(|| invalid("collector receipt aggregate exceeds its byte bound"))
}

fn validate_receipt_directory(
    binding: &FilesystemObjectBindingV3,
) -> Result<(), RestartCollectorErrorV3> {
    if binding.mode & libc::S_IFMT as u32 != libc::S_IFDIR as u32
        || binding.mode & 0o7777 != 0o700
        || binding.uid != unsafe { libc::geteuid() }
        || binding.gid != unsafe { libc::getegid() }
        || binding.flags != 0
    {
        return Err(invalid(
            "collector receipt directory type, owner, mode, or flags are invalid",
        ));
    }
    Ok(())
}

fn validate_receipt_file(
    binding: &FilesystemObjectBindingV3,
    uid: u32,
    gid: u32,
    size: usize,
) -> Result<(), RestartCollectorErrorV3> {
    if binding.mode & libc::S_IFMT as u32 != libc::S_IFREG as u32
        || binding.mode & 0o7777 != 0o600
        || binding.uid != uid
        || binding.gid != gid
        || binding.flags != 0
        || binding.nlink != 1
        || usize::try_from(binding.size).ok() != Some(size)
    {
        return Err(invalid(
            "collector receipt file type, owner, mode, flags, links, or size are invalid",
        ));
    }
    Ok(())
}

fn same_directory_object(
    before: FilesystemObjectBindingV3,
    after: FilesystemObjectBindingV3,
) -> bool {
    before.birthtime_nanoseconds == after.birthtime_nanoseconds
        && before.birthtime_seconds == after.birthtime_seconds
        && before.dev == after.dev
        && before.flags == after.flags
        && before.generation == after.generation
        && before.gid == after.gid
        && before.inode == after.inode
        && before.mode == after.mode
        && before.uid == after.uid
}

fn stable_root_object_matches(
    expected: &StableDirectoryIdentityV3,
    binding: &FilesystemObjectBindingV3,
) -> bool {
    expected.birthtime_nanoseconds == binding.birthtime_nanoseconds
        && expected.birthtime_seconds == binding.birthtime_seconds
        && expected.dev == binding.dev
        && expected.flags == binding.flags
        && expected.generation == binding.generation
        && expected.gid == binding.gid
        && expected.inode == binding.inode
        && expected.mode == binding.mode
        && expected.uid == binding.uid
}

fn same_receipt_object_across_rename(
    before: FilesystemObjectBindingV3,
    after: FilesystemObjectBindingV3,
) -> bool {
    let before_ctime = (before.ctime_seconds, before.ctime_nanoseconds);
    let after_ctime = (after.ctime_seconds, after.ctime_nanoseconds);
    before.birthtime_nanoseconds == after.birthtime_nanoseconds
        && before.birthtime_seconds == after.birthtime_seconds
        && before.dev == after.dev
        && before.flags == after.flags
        && before.generation == after.generation
        && before.gid == after.gid
        && before.inode == after.inode
        && before.mode == after.mode
        && before.mtime_seconds == after.mtime_seconds
        && before.mtime_nanoseconds == after.mtime_nanoseconds
        && before.nlink == after.nlink
        && before.size == after.size
        && before.uid == after.uid
        && after_ctime >= before_ctime
}

fn openat_new_private(
    directory_fd: RawFd,
    name: &str,
    uid: u32,
    gid: u32,
) -> Result<File, RestartCollectorErrorV3> {
    let name_text = name;
    let name =
        CString::new(name_text).map_err(|_| invalid("collector receipt name contains NUL"))?;
    let fd = unsafe {
        libc::openat(
            directory_fd,
            name.as_ptr(),
            libc::O_CREAT | libc::O_EXCL | libc::O_RDWR | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            0o600,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error().into());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    let binding = fstat_binding(file.as_raw_fd(), "new collector receipt")?;
    validate_receipt_file(&binding, uid, gid, 0)?;
    verify_fd_binding_secure(file.as_raw_fd(), &binding, "new collector receipt")?;
    if fstatat_binding(directory_fd, name_text, "new collector receipt pathname")? != binding {
        return Err(invalid(
            "new collector receipt pathname differs from its held descriptor",
        ));
    }
    Ok(file)
}

fn openat_existing_regular(
    directory_fd: RawFd,
    name: &str,
) -> Result<File, RestartCollectorErrorV3> {
    let name = CString::new(name).map_err(|_| invalid("collector receipt name contains NUL"))?;
    let fd = unsafe {
        libc::openat(
            directory_fd,
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn read_fd_exact(
    file: &File,
    binding: &FilesystemObjectBindingV3,
) -> Result<Vec<u8>, RestartCollectorErrorV3> {
    let size = usize::try_from(binding.size)
        .ok()
        .filter(|size| *size <= MAX_RECEIPT_BYTES)
        .ok_or_else(|| invalid("collector receipt size exceeds its bound"))?;
    let mut bytes = vec![0_u8; size];
    let mut offset = 0usize;
    while offset < size {
        let count = unsafe {
            libc::pread(
                file.as_raw_fd(),
                bytes[offset..].as_mut_ptr().cast(),
                size - offset,
                offset as libc::off_t,
            )
        };
        if count < 0 {
            return Err(io::Error::last_os_error().into());
        }
        if count == 0 {
            return Err(invalid("collector receipt was truncated during replay"));
        }
        offset += count as usize;
    }
    if fstat_binding(file.as_raw_fd(), "collector receipt after read")? != *binding {
        return Err(invalid("collector receipt changed while it was read"));
    }
    Ok(bytes)
}

fn verify_no_extended_metadata(fd: RawFd, label: &str) -> Result<(), RestartCollectorErrorV3> {
    let xattr_bytes = unsafe { libc::flistxattr(fd, std::ptr::null_mut(), 0, 0) };
    if xattr_bytes < 0 {
        return Err(io::Error::last_os_error().into());
    }
    if xattr_bytes != 0 {
        return Err(invalid(format!(
            "{label} has extended attributes; collector evidence requires none"
        )));
    }
    let acl = unsafe { acl_get_fd_np(fd, ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ENOENT) {
            return Ok(());
        }
        return Err(error.into());
    }
    let mut entry = std::ptr::null_mut();
    let entry_rc = unsafe { acl_get_entry(acl, ACL_FIRST_ENTRY, &mut entry) };
    let entry_error = io::Error::last_os_error();
    if unsafe { acl_free(acl) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    match entry_rc {
        0 => Err(invalid(format!(
            "{label} has an extended ACL; collector evidence requires none"
        ))),
        -1 if entry_error.raw_os_error() == Some(libc::EINVAL) => Ok(()),
        _ => Err(entry_error.into()),
    }
}

fn verify_fd_binding_secure(
    fd: RawFd,
    expected: &FilesystemObjectBindingV3,
    label: &str,
) -> Result<(), RestartCollectorErrorV3> {
    if fstat_binding(fd, label)? != *expected {
        return Err(invalid(format!("{label} changed before ACL/xattr replay")));
    }
    verify_no_extended_metadata(fd, label)?;
    if fstat_binding(fd, label)? != *expected {
        return Err(invalid(format!("{label} changed during ACL/xattr replay")));
    }
    Ok(())
}

fn fstatat_binding(
    directory_fd: RawFd,
    name: &str,
    _label: &str,
) -> Result<FilesystemObjectBindingV3, RestartCollectorErrorV3> {
    let name = CString::new(name).map_err(|_| invalid("collector receipt name contains NUL"))?;
    let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
    if unsafe {
        libc::fstatat(
            directory_fd,
            name.as_ptr(),
            stat.as_mut_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    } != 0
    {
        return Err(io::Error::last_os_error().into());
    }
    binding_from_stat(unsafe { stat.assume_init() })
}

fn classify_matching_groups(
    inventory: &RestartIOMediaInventoryV3,
    prepared_backing: &DiskImageBackingIdentityV2,
) -> Result<Vec<MatchingDiskImageGroupV3>, RestartCollectorErrorV3> {
    validate_restart_iomedia_inventory_v3(inventory)?;
    validate_disk_image_backing_identity_v2(prepared_backing)?;
    let mut groups = BTreeMap::<(String, String), MatchingDiskImageGroupV3>::new();
    let mut all_device_urls = BTreeMap::<String, String>::new();
    for object in &inventory.objects {
        let Some(candidate) = &object.candidate else {
            continue;
        };
        if all_device_urls
            .insert(
                candidate.disk_image_device.registry_entry_id.clone(),
                candidate.disk_image_url.clone(),
            )
            .is_some_and(|url| url != candidate.disk_image_url)
        {
            return Err(invalid(
                "one AppleDiskImageDevice has multiple DiskImageURL values",
            ));
        }
        if !restart_disk_image_backing_matches_prepared_v3(
            &candidate.backing_identity,
            prepared_backing,
        )? {
            continue;
        }
        let key = (
            candidate.disk_image_device.registry_entry_id.clone(),
            candidate.disk_image_url.clone(),
        );
        let group = groups
            .entry(key)
            .or_insert_with(|| MatchingDiskImageGroupV3 {
                candidate: candidate.clone(),
                member_bsd_names: Vec::new(),
                member_registry_entry_ids: Vec::new(),
            });
        if group.candidate != *candidate {
            return Err(invalid(
                "one AppleDiskImageDevice has inconsistent URL or ancestry evidence",
            ));
        }
        group
            .member_bsd_names
            .push(object.provenance.bsd_name.clone());
        group
            .member_registry_entry_ids
            .push(object.provenance.registry_entry_id.clone());
    }
    let mut seen_devices = BTreeMap::<String, String>::new();
    let mut result = groups.into_values().collect::<Vec<_>>();
    for group in &mut result {
        group.member_bsd_names.sort();
        group.member_bsd_names.dedup();
        group.member_registry_entry_ids.sort();
        group.member_registry_entry_ids.dedup();
        if group.member_bsd_names.is_empty()
            || group.member_registry_entry_ids.is_empty()
            || seen_devices
                .insert(
                    group.candidate.disk_image_device.registry_entry_id.clone(),
                    group.candidate.disk_image_url.clone(),
                )
                .is_some_and(|url| url != group.candidate.disk_image_url)
        {
            return Err(invalid(
                "restart match group is empty or one device has multiple URLs",
            ));
        }
    }
    Ok(result)
}

fn derive_current_expected_absence_v3(
    current: &RestartIOMediaInventoryV3,
    match_result: &ReconciliationMatchV2,
    matching_groups: &[MatchingDiskImageGroupV3],
) -> Result<Option<RestartIOMediaInventoryV3>, RestartCollectorErrorV3> {
    validate_restart_iomedia_inventory_v3(current)?;
    match match_result {
        ReconciliationMatchV2::Zero => {
            if !matching_groups.is_empty() {
                return Err(invalid(
                    "Zero reconciliation unexpectedly owns a matching IOMedia group",
                ));
            }
            Ok(Some(current.clone()))
        }
        ReconciliationMatchV2::Unique { .. } => {
            let [group] = matching_groups else {
                return Err(invalid(
                    "Unique reconciliation does not own exactly one matching IOMedia group",
                ));
            };
            let members = group
                .member_registry_entry_ids
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>();
            if members.len() != group.member_registry_entry_ids.len()
                || !members.iter().all(|member| {
                    current
                        .objects
                        .binary_search_by(|object| {
                            object.provenance.registry_entry_id.as_str().cmp(member)
                        })
                        .is_ok()
                })
            {
                return Err(invalid(
                    "Unique matching group is not an exact subset of the current inventory",
                ));
            }
            let mut absence = current.clone();
            absence
                .objects
                .retain(|object| !members.contains(&object.provenance.registry_entry_id));
            if current.objects.len().checked_sub(absence.objects.len()) != Some(members.len()) {
                return Err(invalid(
                    "current expected absence did not remove the exact Unique group",
                ));
            }
            validate_restart_iomedia_inventory_v3(&absence)?;
            Ok(Some(absence))
        }
        ReconciliationMatchV2::Ambiguous { .. } => {
            if matching_groups.len() < 2 {
                return Err(invalid(
                    "Ambiguous reconciliation has fewer than two matching groups",
                ));
            }
            Ok(None)
        }
    }
}

fn classify_mount_state(
    groups: &[MatchingDiskImageGroupV3],
    mounts: &[MountBindingV3],
    policy: &RestartCollectorPolicyV3,
) -> Result<(ReconciliationMatchV2, bool), RestartCollectorErrorV3> {
    let sources = groups
        .iter()
        .flat_map(|group| group.member_bsd_names.iter())
        .map(|bsd_name| format!("/dev/{bsd_name}"))
        .collect::<BTreeSet<_>>();
    let matching_mounts = mounts
        .iter()
        .filter(|mount| sources.contains(&mount.mount_from))
        .collect::<Vec<_>>();
    if matching_mounts
        .iter()
        .any(|mount| mount.mount_on != policy.mountpoint)
    {
        return Err(invalid(
            "matching disk image is mounted outside the policy-bound mountpoint",
        ));
    }
    let target_mounts = mounts
        .iter()
        .filter(|mount| mount.mount_on == policy.mountpoint)
        .collect::<Vec<_>>();
    if target_mounts
        .iter()
        .any(|mount| !sources.contains(&mount.mount_from))
    {
        return Err(invalid(
            "policy-bound mountpoint is occupied by a nonmatching filesystem",
        ));
    }
    if target_mounts.len() > 1 || matching_mounts.len() > 1 {
        return Err(invalid(
            "restart mount table has multiple bindings for the target disk image",
        ));
    }
    let mounted = !matching_mounts.is_empty();
    let result = match groups.len() {
        0 if mounted || !target_mounts.is_empty() => {
            return Err(invalid("Zero match cannot own a mount-table entry"));
        }
        0 => ReconciliationMatchV2::Zero,
        1 => ReconciliationMatchV2::Unique { mounted },
        count => ReconciliationMatchV2::Ambiguous {
            matching_objects: u32::try_from(count)
                .map_err(|_| invalid("restart match count exceeds u32"))?,
        },
    };
    Ok((result, mounted))
}

impl LiveReplayGuardV3 {
    fn revalidate(
        &self,
        receipt: &RestartCollectorReceiptV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_non_mount_evidence(receipt)?;
        self.mountpoint.revalidate()?;
        if mount_table_snapshot()? != self.mounts {
            return Err(invalid(
                "mount table changed after restart receipt persistence",
            ));
        }
        Ok(())
    }

    /// Replay the persistent evidence which must remain exact across eject
    /// without touching the pre-effect held IOMedia descriptors.  The latter
    /// are expected to disappear; current IOMedia is checked separately
    /// against the sealed before/expected-after endpoints.
    fn revalidate_eject_stable(
        &self,
        receipt: &RestartCollectorReceiptV3,
        unchanged_mounts: &[MountBindingV3],
    ) -> Result<(), RestartCollectorErrorV3> {
        if self.mounts != unchanged_mounts
            || receipt.mount_evidence.mounts_before != unchanged_mounts
            || receipt.mount_evidence.mounts_after != unchanged_mounts
        {
            return Err(invalid(
                "eject expectation changed the exact stable mount census",
            ));
        }
        for mount in unchanged_mounts {
            validate_mount_binding_shape(mount)?;
        }
        if unchanged_mounts.len() > MAX_MOUNT_ENTRIES
            || unchanged_mounts.windows(2).any(|pair| pair[0] >= pair[1])
        {
            return Err(invalid(
                "eject stable mount census is oversized or not strictly sorted",
            ));
        }
        self.mountpoint.revalidate()?;
        if mount_table_snapshot()? != unchanged_mounts {
            return Err(invalid(
                "mount table changed while the eject expectation was pending",
            ));
        }
        self.backing
            .revalidate_identity_after_persistence(&self.prepared_backing)?;
        self.artifact_root.revalidate("artifact root")?;
        let roster = list_directory(self.artifact_root.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES)?;
        let absent = self
            .artifact_evidence
            .artifacts
            .iter()
            .all(|artifact| roster.binary_search(&artifact.basename).is_err());
        if roster != self.artifact_evidence.roster
            || absent != self.artifact_evidence.operation_artifacts_absent
            || self.artifact_root.binding != self.artifact_evidence.root_binding
        {
            return Err(invalid(
                "operation-artifact census changed while eject was pending",
            ));
        }
        if current_boot_session_uuid()? != receipt.boot_session_uuid {
            return Err(invalid("boot changed while eject was pending"));
        }
        validate_receipt(receipt)
    }

    fn revalidate_across_mount_delta(
        &self,
        receipt: &RestartCollectorReceiptV3,
        expected_after: &[MountBindingV3],
        direction: MountDeltaDirectionV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        self.revalidate_non_mount_evidence(receipt)?;
        if expected_after.len() > MAX_MOUNT_ENTRIES {
            return Err(invalid("expected-after mount census exceeds its bound"));
        }
        for mount in expected_after {
            validate_mount_binding_shape(mount)?;
        }
        if expected_after.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(invalid(
                "expected-after mount census is not a strictly sorted exact roster",
            ));
        }
        match direction {
            MountDeltaDirectionV3::Mount => {
                if exact_added_mount(&self.mounts, expected_after).is_none() {
                    return Err(invalid(
                        "mount transition is not exactly the retained census plus one entry",
                    ));
                }
                let UnderlyingMountpointGuardV3::Held(mountpoint) = &self.mountpoint else {
                    return Err(invalid(
                        "mount transition lacks the retained underlying mountpoint descriptor",
                    ));
                };
                // The pathname now resolves through the mounted filesystem.
                // Revalidate only the descriptor that was captured before the
                // effect; pathname replay is intentionally deferred until the
                // filesystem is later unmounted.
                mountpoint.revalidate_descriptor_only("underlying mountpoint hidden by mount")?;
            }
            MountDeltaDirectionV3::Unmount => {
                if exact_removed_mount(&self.mounts, expected_after).is_none() {
                    return Err(invalid(
                        "unmount transition is not exactly the retained census minus one entry",
                    ));
                }
                if !matches!(
                    &self.mountpoint,
                    UnderlyingMountpointGuardV3::DeferredWhileMounted { .. }
                ) {
                    return Err(invalid(
                        "unmount transition lacks a deferred underlying mountpoint guard",
                    ));
                }
                // This reopens through the retained parent dirfd with
                // O_NOFOLLOW_ANY and compares the full prepared identity.
                // It is the first point where the previously hidden pathname
                // may be trusted again.
                self.mountpoint.reopen_underlying_after_unmount()?;
            }
        }
        if mount_table_snapshot()? != expected_after {
            return Err(invalid(
                "mount table differs from the exact expected-after census",
            ));
        }
        Ok(())
    }

    fn revalidate_non_mount_evidence(
        &self,
        receipt: &RestartCollectorReceiptV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        if self.mounts != receipt.mount_evidence.mounts_after {
            return Err(invalid(
                "retained mount census differs from the persisted receipt",
            ));
        }
        for mount in &self.mounts {
            validate_mount_binding_shape(mount)?;
        }
        if self.mounts.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(invalid(
                "retained mount census is not a strictly sorted exact roster",
            ));
        }
        self.iomedia.revalidate_after_persistence()?;
        if self.iomedia.report() != &receipt.iomedia_inventory {
            return Err(invalid(
                "held restart inventory differs from the persisted receipt",
            ));
        }
        self.backing
            .revalidate_identity_after_persistence(&self.prepared_backing)?;
        self.artifact_root.revalidate("artifact root")?;
        let roster = list_directory(self.artifact_root.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES)?;
        let absent = self
            .artifact_evidence
            .artifacts
            .iter()
            .all(|artifact| roster.binary_search(&artifact.basename).is_err());
        if roster != self.artifact_evidence.roster
            || absent != self.artifact_evidence.operation_artifacts_absent
            || self.artifact_root.binding != self.artifact_evidence.root_binding
        {
            return Err(invalid(
                "operation-artifact census changed after receipt persistence",
            ));
        }
        if current_boot_session_uuid()? != receipt.boot_session_uuid {
            return Err(invalid("boot changed after restart receipt persistence"));
        }
        validate_receipt(receipt)
    }
}

impl UnderlyingMountpointGuardV3 {
    fn capture_deferred(expected: &MountpointIdentityV3) -> Result<Self, RestartCollectorErrorV3> {
        validate_mountpoint_identity(expected)?;
        let mountpoint = Path::new(&expected.path);
        let parent_path = mountpoint
            .parent()
            .filter(|path| path.is_absolute())
            .ok_or_else(|| invalid("prepared mountpoint has no absolute parent"))?;
        let basename = mountpoint
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| invalid("prepared mountpoint has no bounded UTF-8 basename"))?;
        validate_child_name(basename)?;
        let parent = HeldDirectoryV3::capture(parent_path, "mountpoint parent")?;
        if parent.path.join(basename) != mountpoint {
            return Err(invalid(
                "retained mountpoint parent and basename do not reconstruct the prepared path",
            ));
        }
        Ok(Self::DeferredWhileMounted {
            basename: basename.to_string(),
            expected: expected.clone(),
            parent,
        })
    }

    fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        match self {
            Self::Held(mountpoint) => mountpoint.revalidate("mountpoint"),
            Self::DeferredWhileMounted {
                basename,
                expected,
                parent,
            } => {
                parent.revalidate("mountpoint parent")?;
                validate_child_name(basename)?;
                validate_mountpoint_identity(expected)?;
                if parent.path.join(basename) != Path::new(&expected.path) {
                    return Err(invalid(
                        "deferred underlying mountpoint guard changed its prepared path",
                    ));
                }
                Ok(())
            }
        }
    }

    fn reopen_underlying_after_unmount(&self) -> Result<HeldDirectoryV3, RestartCollectorErrorV3> {
        match self {
            Self::Held(mountpoint) => {
                mountpoint.revalidate("mountpoint")?;
                HeldDirectoryV3::capture(&mountpoint.path, "mountpoint")
            }
            Self::DeferredWhileMounted {
                basename,
                expected,
                parent,
            } => {
                parent.revalidate("mountpoint parent")?;
                let held = HeldDirectoryV3::capture_child(
                    parent,
                    basename,
                    "underlying mountpoint after unmount",
                )?;
                if mountpoint_identity_from_held(&held)? != *expected {
                    return Err(invalid(
                        "underlying mountpoint changed while hidden by the mounted filesystem",
                    ));
                }
                Ok(held)
            }
        }
    }
}

impl HeldDirectoryV3 {
    fn capture(path: &Path, label: &str) -> Result<Self, RestartCollectorErrorV3> {
        let canonical = canonical_input_path(path, label, true)?;
        let c_path = CString::new(canonical.as_bytes())
            .map_err(|_| invalid(format!("{label} path contains NUL")))?;
        let fd = unsafe {
            libc::open(
                c_path.as_ptr(),
                libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW_ANY | libc::O_DIRECTORY,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error().into());
        }
        let file = unsafe { File::from_raw_fd(fd) };
        let binding = fstat_binding(file.as_raw_fd(), label)?;
        let path_binding = lstat_binding(Path::new(&canonical), label)?;
        if binding != path_binding || binding.mode & libc::S_IFMT as u32 != libc::S_IFDIR as u32 {
            return Err(invalid(format!(
                "{label} changed during descriptor capture or is not a directory"
            )));
        }
        verify_fd_binding_secure(file.as_raw_fd(), &binding, label)?;
        if lstat_binding(Path::new(&canonical), label)? != binding {
            return Err(invalid(format!(
                "{label} changed while ACL/xattr absence was verified"
            )));
        }
        Ok(Self {
            binding,
            file,
            path: PathBuf::from(canonical),
        })
    }

    fn capture_child(
        parent: &HeldDirectoryV3,
        basename: &str,
        label: &str,
    ) -> Result<Self, RestartCollectorErrorV3> {
        parent.revalidate("mountpoint parent")?;
        validate_child_name(basename)?;
        let before = fstatat_binding(parent.file.as_raw_fd(), basename, label)?;
        if before.mode & libc::S_IFMT as u32 != libc::S_IFDIR as u32 {
            return Err(invalid(format!("{label} is not a directory")));
        }
        let name = CString::new(basename)
            .map_err(|_| invalid(format!("{label} basename contains NUL")))?;
        let fd = unsafe {
            libc::openat(
                parent.file.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW_ANY | libc::O_DIRECTORY,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error().into());
        }
        let file = unsafe { File::from_raw_fd(fd) };
        let binding = fstat_binding(file.as_raw_fd(), label)?;
        let after = fstatat_binding(parent.file.as_raw_fd(), basename, label)?;
        let path = parent.path.join(basename);
        if before != binding || before != after || before != lstat_binding(&path, label)? {
            return Err(invalid(format!(
                "{label} changed across retained-parent openat"
            )));
        }
        verify_fd_binding_secure(file.as_raw_fd(), &binding, label)?;
        parent.revalidate("mountpoint parent")?;
        if fstatat_binding(parent.file.as_raw_fd(), basename, label)? != binding
            || lstat_binding(&path, label)? != binding
        {
            return Err(invalid(format!("{label} changed during ACL/xattr replay")));
        }
        Ok(Self {
            binding,
            file,
            path,
        })
    }

    fn revalidate(&self, label: &str) -> Result<(), RestartCollectorErrorV3> {
        if fstat_binding(self.file.as_raw_fd(), label)? != self.binding
            || lstat_binding(&self.path, label)? != self.binding
            || canonical_input_path(&self.path, label, true)? != path_text(&self.path, label)?
        {
            return Err(invalid(format!(
                "held {label} descriptor or pathname changed"
            )));
        }
        verify_fd_binding_secure(self.file.as_raw_fd(), &self.binding, label)?;
        if lstat_binding(&self.path, label)? != self.binding {
            return Err(invalid(format!(
                "held {label} pathname changed during ACL/xattr replay"
            )));
        }
        Ok(())
    }

    fn revalidate_descriptor_only(&self, label: &str) -> Result<(), RestartCollectorErrorV3> {
        if fstat_binding(self.file.as_raw_fd(), label)? != self.binding {
            return Err(invalid(format!("held {label} descriptor changed")));
        }
        verify_fd_binding_secure(self.file.as_raw_fd(), &self.binding, label)?;
        if fstat_binding(self.file.as_raw_fd(), label)? != self.binding {
            return Err(invalid(format!(
                "held {label} descriptor changed during ACL/xattr replay"
            )));
        }
        Ok(())
    }
}

impl StableDirectoryIdentityV3 {
    fn from_binding(binding: &FilesystemObjectBindingV3, roster_entries: usize) -> Self {
        Self {
            birthtime_nanoseconds: binding.birthtime_nanoseconds,
            birthtime_seconds: binding.birthtime_seconds,
            dev: binding.dev,
            flags: binding.flags,
            generation: binding.generation,
            gid: binding.gid,
            inode: binding.inode,
            mode: binding.mode,
            nlink: binding.nlink,
            roster_entries: roster_entries as u64,
            uid: binding.uid,
        }
    }

    fn matches_binding(&self, binding: &FilesystemObjectBindingV3, roster_entries: usize) -> bool {
        // APFS reports a directory nlink that grows with every roster entry,
        // including regular files. Bind the raw prepared nlink plus prepared
        // roster cardinality and compare their invariant base so legitimate
        // receipt publication does not look like a directory replacement.
        self.birthtime_nanoseconds == binding.birthtime_nanoseconds
            && self.birthtime_seconds == binding.birthtime_seconds
            && self.dev == binding.dev
            && self.flags == binding.flags
            && self.generation == binding.generation
            && self.gid == binding.gid
            && self.inode == binding.inode
            && self.mode == binding.mode
            && self.uid == binding.uid
            && self.nlink.checked_sub(self.roster_entries)
                == binding.nlink.checked_sub(roster_entries as u64)
    }
}

fn mountpoint_identity_from_held(
    held: &HeldDirectoryV3,
) -> Result<MountpointIdentityV3, RestartCollectorErrorV3> {
    let identity = MountpointIdentityV3 {
        authority: DisposableAuthorityV2::none(),
        binding: held.binding,
        path: path_text(&held.path, "mountpoint")?,
        schema: MOUNTPOINT_SCHEMA.to_string(),
    };
    validate_mountpoint_identity(&identity)?;
    Ok(identity)
}

fn validate_request(
    request: &LiveRestartCollectorRequestV3<'_>,
) -> Result<(), RestartCollectorErrorV3> {
    validate_bindings(request.bindings)?;
    validate_policy(request.policy)?;
    validate_baseline(request.baseline)?;
    validate_disk_image_backing_identity_v2(request.prepared_backing)?;
    validate_mountpoint_identity(request.mountpoint_identity)?;
    if canonical_input_path(request.artifact_root, "artifact root", true)?
        != request.policy.artifact_root
        || canonical_input_path(request.receipt_root, "receipt root", true)?
            != request.policy.receipt_root
        || request.prepared_backing.canonical_path != request.policy.backing_path
        || request.mountpoint_identity.path != request.policy.mountpoint
        || request.policy.sha256()? != request.bindings.collector_policy_sha256
        || request.baseline.sha256()? != request.bindings.baseline_inventory_sha256
        || sha256(&canonical_json(request.prepared_backing)?)
            != request.bindings.backing_identity_sha256
        || request.mountpoint_identity.sha256()? != request.bindings.mountpoint_underlying_sha256
        || current_boot_session_uuid()? != request.bindings.boot_session_uuid
    {
        return Err(invalid(
            "restart request differs from its prepared policy, backing, baseline, or mountpoint bindings",
        ));
    }
    Ok(())
}

fn validate_bindings(bindings: &RestartCollectorBindingsV3) -> Result<(), RestartCollectorErrorV3> {
    if !valid_digest(&bindings.backing_identity_sha256)
        || !valid_digest(&bindings.baseline_inventory_sha256)
        || !valid_uuid(&bindings.boot_session_uuid)
        || !valid_digest(&bindings.collector_policy_sha256)
        || !valid_digest(&bindings.mountpoint_underlying_sha256)
        || !valid_nonce(&bindings.operation_nonce)
        || !valid_nonce(&bindings.restart_epoch_nonce)
        || bindings.restart_started_monotonic_nanoseconds == 0
    {
        return Err(invalid("restart collector bindings are malformed"));
    }
    Ok(())
}

fn validate_artifact_bindings(
    artifacts: &[PreparedArtifactBindingV3],
    require_complete_collector_profile: bool,
) -> Result<(), RestartCollectorErrorV3> {
    if artifacts.len() > MAX_ARTIFACT_BINDINGS
        || artifacts.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(invalid(
            "prepared artifact roster is oversized, duplicated, or noncanonical",
        ));
    }
    let mut roles = BTreeSet::new();
    let mut basenames = BTreeSet::new();
    for artifact in artifacts {
        validate_child_name(&artifact.basename)?;
        if !roles.insert(artifact.role) || !basenames.insert(artifact.basename.as_str()) {
            return Err(invalid(
                "prepared artifact roster duplicates or aliases a role or basename",
            ));
        }
    }
    if require_complete_collector_profile
        && (artifacts.len() != 1 || artifacts[0].role != ArtifactRoleV3::BackingImage)
    {
        return Err(invalid(
            "collector requires exactly one prepared BackingImage artifact binding",
        ));
    }
    Ok(())
}

fn validate_policy(policy: &RestartCollectorPolicyV3) -> Result<(), RestartCollectorErrorV3> {
    if policy.schema != POLICY_SCHEMA
        || policy.authority.any()
        || policy.max_iomedia_objects != 256
        || policy.max_mount_entries != MAX_MOUNT_ENTRIES
        || policy.artifacts.len() > MAX_ARTIFACT_BINDINGS
        || policy.protected_roots.is_empty()
        || policy.protected_roots.len() > MAX_PROTECTED_ROOTS
        || policy
            .protected_roots
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
    {
        return Err(invalid(
            "restart collector policy is malformed or grants authority",
        ));
    }
    validate_artifact_bindings(&policy.artifacts, true)?;
    let backing_artifact = policy
        .artifacts
        .iter()
        .find(|artifact| artifact.role == ArtifactRoleV3::BackingImage)
        .ok_or_else(|| invalid("collector policy omits its exact BackingImage role"))?;
    if Path::new(&policy.artifact_root).join(&backing_artifact.basename)
        != Path::new(&policy.backing_path)
    {
        return Err(invalid(
            "collector policy backing path differs from its prepared BackingImage binding",
        ));
    }
    for (path, label, directory) in [
        (&policy.backing_path, "policy backing path", false),
        (&policy.mountpoint, "policy mountpoint", true),
        (&policy.artifact_root, "policy artifact root", true),
        (&policy.receipt_root, "policy receipt root", true),
    ] {
        if canonical_input_path(Path::new(path), label, directory)? != *path {
            return Err(invalid(format!("{label} changed after policy capture")));
        }
    }
    for root in &policy.protected_roots {
        if canonical_input_path(Path::new(root), "policy protected root", true)? != *root {
            return Err(invalid("policy protected root changed after capture"));
        }
    }
    validate_stable_directory_identity(&policy.artifact_root_identity, "artifact root")?;
    validate_stable_directory_identity(&policy.receipt_root_identity, "receipt root")?;
    let artifact_root =
        HeldDirectoryV3::capture(Path::new(&policy.artifact_root), "policy artifact root")?;
    let artifact_root_roster =
        list_directory(artifact_root.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES)?;
    artifact_root.revalidate("policy artifact root")?;
    if !policy
        .artifact_root_identity
        .matches_binding(&artifact_root.binding, artifact_root_roster.len())
    {
        return Err(invalid(
            "policy artifact root differs from its prepared stable identity",
        ));
    }
    let receipt_root =
        HeldDirectoryV3::capture(Path::new(&policy.receipt_root), "policy receipt root")?;
    let receipt_root_roster = list_directory(receipt_root.file.as_raw_fd(), MAX_RECEIPT_FILES)?;
    receipt_root.revalidate("policy receipt root")?;
    if !policy
        .receipt_root_identity
        .matches_binding(&receipt_root.binding, receipt_root_roster.len())
    {
        return Err(invalid(
            "policy receipt root differs from its prepared stable identity",
        ));
    }
    validate_receipt_directory(&receipt_root.binding)?;
    if !policy
        .protected_roots
        .iter()
        .any(|root| root == &policy.artifact_root)
        || !policy
            .protected_roots
            .iter()
            .any(|root| root == &policy.receipt_root)
    {
        return Err(invalid(
            "restart collector policy does not protect its artifact root",
        ));
    }
    if paths_overlap(&policy.artifact_root, &policy.receipt_root)
        || paths_overlap(&policy.backing_path, &policy.mountpoint)
        || policy.protected_roots.iter().any(|root| {
            paths_overlap(root, &policy.mountpoint)
                || (root != &policy.artifact_root
                    && path_is_at_or_below(&policy.backing_path, root))
        })
    {
        return Err(invalid(
            "restart collector backing, mountpoint, and protected roots are not topologically disjoint",
        ));
    }
    Ok(())
}

fn validate_stable_directory_identity(
    identity: &StableDirectoryIdentityV3,
    label: &str,
) -> Result<(), RestartCollectorErrorV3> {
    if identity.dev == 0
        || identity.inode == 0
        || identity.nlink == 0
        || identity.roster_entries > MAX_ARTIFACT_ENTRIES as u64
        || identity
            .nlink
            .checked_sub(identity.roster_entries)
            .is_none()
        || identity.mode & libc::S_IFMT as u32 != libc::S_IFDIR as u32
        || !(0..1_000_000_000).contains(&identity.birthtime_nanoseconds)
    {
        return Err(invalid(format!(
            "prepared {label} stable identity is malformed"
        )));
    }
    Ok(())
}

fn validate_baseline(baseline: &RestartBaselineInventoryV3) -> Result<(), RestartCollectorErrorV3> {
    if baseline.schema != BASELINE_SCHEMA
        || baseline.authority.any()
        || !valid_uuid(&baseline.boot_session_uuid)
        || baseline.registry_entry_ids.is_empty()
        || baseline.registry_entry_ids.len() > 256
        || baseline
            .registry_entry_ids
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        || baseline
            .registry_entry_ids
            .iter()
            .any(|id| !valid_registry_id(id))
    {
        return Err(invalid("restart baseline inventory is malformed"));
    }
    Ok(())
}

fn validate_mountpoint_identity(
    identity: &MountpointIdentityV3,
) -> Result<(), RestartCollectorErrorV3> {
    if identity.schema != MOUNTPOINT_SCHEMA
        || identity.authority.any()
        || !Path::new(&identity.path).is_absolute()
        || identity.binding.dev == 0
        || identity.binding.inode == 0
        || identity.binding.nlink == 0
        || identity.binding.mode & libc::S_IFMT as u32 != libc::S_IFDIR as u32
        || !(0..1_000_000_000).contains(&identity.binding.birthtime_nanoseconds)
        || !(0..1_000_000_000).contains(&identity.binding.ctime_nanoseconds)
        || !(0..1_000_000_000).contains(&identity.binding.mtime_nanoseconds)
    {
        return Err(invalid("mountpoint underlying identity is malformed"));
    }
    Ok(())
}

fn validate_filesystem_binding_shape(
    binding: &FilesystemObjectBindingV3,
    directory: bool,
    label: &str,
) -> Result<(), RestartCollectorErrorV3> {
    let expected_type = if directory {
        libc::S_IFDIR
    } else {
        libc::S_IFREG
    } as u32;
    if binding.dev == 0
        || binding.inode == 0
        || binding.nlink == 0
        || binding.mode & libc::S_IFMT as u32 != expected_type
        || !(0..1_000_000_000).contains(&binding.birthtime_nanoseconds)
        || !(0..1_000_000_000).contains(&binding.ctime_nanoseconds)
        || !(0..1_000_000_000).contains(&binding.mtime_nanoseconds)
    {
        return Err(invalid(format!("{label} binding shape is malformed")));
    }
    Ok(())
}

fn validate_receipt(receipt: &RestartCollectorReceiptV3) -> Result<(), RestartCollectorErrorV3> {
    validate_disk_image_backing_identity_v2(&receipt.backing_identity)?;
    validate_baseline(&receipt.baseline_inventory)?;
    validate_policy(&receipt.collector_policy)?;
    validate_mountpoint_identity(&receipt.mountpoint_underlying)?;
    let derived_current_expected_absence = derive_current_expected_absence_v3(
        &receipt.iomedia_inventory,
        &receipt.match_result,
        &receipt.matching_groups,
    )?;
    let derived_current_expected_absence_sha256 = derived_current_expected_absence
        .as_ref()
        .map(|inventory| canonical_json(inventory).map(|bytes| sha256(&bytes)))
        .transpose()?;
    if receipt.schema != RECEIPT_SCHEMA
        || receipt.schema_version != 3
        || receipt.authority.any()
        || !valid_digest(&receipt.artifact_evidence_sha256)
        || !valid_digest(&receipt.backing_identity_sha256)
        || !valid_digest(&receipt.baseline_inventory_sha256)
        || !valid_uuid(&receipt.boot_session_uuid)
        || !valid_digest(&receipt.collector_policy_sha256)
        || !valid_digest(&receipt.iomedia_evidence_sha256)
        || !valid_digest(&receipt.mount_evidence_sha256)
        || !valid_digest(&receipt.mountpoint_underlying_sha256)
        || !valid_digest(&receipt.post_inventory_sha256)
        || receipt.current_expected_absence_inventory != derived_current_expected_absence
        || receipt.current_expected_absence_inventory_sha256
            != derived_current_expected_absence_sha256
        || !valid_nonce(&receipt.operation_nonce)
        || !valid_nonce(&receipt.restart_epoch_nonce)
        || receipt.monotonic_before_nanoseconds == 0
        || receipt.monotonic_after_nanoseconds < receipt.monotonic_before_nanoseconds
        || receipt.boot_session_uuid != receipt.iomedia_inventory.boot_session_uuid
        || receipt.backing_identity.canonical_path != receipt.collector_policy.backing_path
        || receipt.mountpoint_underlying.path != receipt.collector_policy.mountpoint
        || receipt.artifact_evidence.authority.any()
        || receipt.artifact_evidence.schema != ARTIFACT_SCHEMA
        || receipt.mount_evidence.authority.any()
        || receipt.mount_evidence.schema != MOUNT_SCHEMA
        || sha256(&canonical_json(&receipt.backing_identity)?) != receipt.backing_identity_sha256
        || receipt.baseline_inventory.sha256()? != receipt.baseline_inventory_sha256
        || receipt.collector_policy.sha256()? != receipt.collector_policy_sha256
        || receipt.mountpoint_underlying.sha256()? != receipt.mountpoint_underlying_sha256
        || receipt.operation_artifacts_absent
            != receipt.artifact_evidence.operation_artifacts_absent
        || receipt.artifact_evidence.artifacts != receipt.collector_policy.artifacts
        || receipt.artifact_evidence.artifact_root != receipt.collector_policy.artifact_root
        || !receipt
            .collector_policy
            .artifact_root_identity
            .matches_binding(
                &receipt.artifact_evidence.root_binding,
                receipt.artifact_evidence.roster.len(),
            )
        || sha256(&canonical_json(&receipt.artifact_evidence)?) != receipt.artifact_evidence_sha256
        || sha256(&canonical_json(&receipt.iomedia_inventory)?) != receipt.iomedia_evidence_sha256
        || sha256(&canonical_json(&receipt.mount_evidence)?) != receipt.mount_evidence_sha256
        || receipt.mount_evidence.mounts_before != receipt.mount_evidence.mounts_after
        || !receipt.mount_evidence.no_nested_mounts
        || receipt.artifact_evidence.roster.len() > MAX_ARTIFACT_ENTRIES
        || receipt
            .artifact_evidence
            .roster
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        || receipt.operation_artifacts_absent
            != receipt.artifact_evidence.artifacts.iter().all(|artifact| {
                receipt
                    .artifact_evidence
                    .roster
                    .binary_search(&artifact.basename)
                    .is_err()
            })
    {
        return Err(invalid(
            "restart collector receipt is malformed, inconsistent, or grants authority",
        ));
    }
    validate_filesystem_binding_shape(
        &receipt.artifact_evidence.root_binding,
        true,
        "artifact root",
    )?;
    for name in &receipt.artifact_evidence.roster {
        validate_child_name(name)?;
    }
    validate_restart_iomedia_inventory_v3(&receipt.iomedia_inventory)?;
    let current_baseline = RestartBaselineInventoryV3::from_inventory(&receipt.iomedia_inventory)?;
    let reconciliation_baseline_restored = derived_current_expected_absence
        .as_ref()
        .is_some_and(|expected| expected == &receipt.iomedia_inventory);
    if current_baseline.sha256()? != receipt.post_inventory_sha256
        || (receipt.purpose == CollectorPurposeV3::ReconciliationSnapshot
            && receipt.baseline_restored != reconciliation_baseline_restored)
    {
        return Err(invalid(
            "restart collector baseline restoration claim is inconsistent",
        ));
    }
    let expected_groups =
        classify_matching_groups(&receipt.iomedia_inventory, &receipt.backing_identity)?;
    let (expected_match, mounted) = classify_mount_state(
        &expected_groups,
        &receipt.mount_evidence.mounts_before,
        &receipt.collector_policy,
    )?;
    reject_nested_mounts(
        &receipt.mount_evidence.mounts_before,
        &receipt.collector_policy,
    )?;
    if expected_groups != receipt.matching_groups
        || expected_match != receipt.match_result
        || receipt.mount_evidence.mountpoint_underlying_revalidated == mounted
    {
        return Err(invalid(
            "restart collector match or mountpoint replay claim is inconsistent",
        ));
    }
    match (receipt.purpose, &receipt.reconciliation_snapshot_sha256) {
        (CollectorPurposeV3::ReconciliationSnapshot, None) => {}
        (CollectorPurposeV3::FreshAbsence, Some(digest))
            if valid_digest(digest)
                && receipt.match_result == ReconciliationMatchV2::Zero
                && receipt.baseline_restored
                && receipt.operation_artifacts_absent
                && receipt.mount_evidence.no_nested_mounts
                && receipt.mount_evidence.mountpoint_underlying_revalidated => {}
        _ => return Err(invalid("restart collector purpose binding is malformed")),
    }
    match receipt.match_result {
        ReconciliationMatchV2::Zero if receipt.matching_groups.is_empty() => {}
        ReconciliationMatchV2::Unique { .. } if receipt.matching_groups.len() == 1 => {}
        ReconciliationMatchV2::Ambiguous { matching_objects }
            if matching_objects >= 2
                && usize::try_from(matching_objects).ok()
                    == Some(receipt.matching_groups.len()) => {}
        _ => return Err(invalid("restart collector match result is inconsistent")),
    }
    Ok(())
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_nonce(value: &str) -> bool {
    valid_digest(value) && value.bytes().any(|byte| byte != b'0')
}

fn valid_registry_id(value: &str) -> bool {
    value.len() == 16
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        && value != "0000000000000000"
}

fn valid_uuid(value: &str) -> bool {
    value.len() == 36
        && value
            .bytes()
            .any(|byte| byte.is_ascii_hexdigit() && byte != b'0')
        && value.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte),
        })
}

fn invalid(message: impl Into<String>) -> RestartCollectorErrorV3 {
    RestartCollectorErrorV3::Invalid(message.into())
}

fn path_text(path: &Path, label: &str) -> Result<String, RestartCollectorErrorV3> {
    path.to_str()
        .filter(|value| !value.is_empty() && value.len() <= MAX_MOUNT_STRING_BYTES)
        .map(str::to_string)
        .ok_or_else(|| invalid(format!("{label} path is not bounded UTF-8")))
}

fn canonical_input_path(
    path: &Path,
    label: &str,
    directory: bool,
) -> Result<String, RestartCollectorErrorV3> {
    if !path.is_absolute() {
        return Err(invalid(format!("{label} is not absolute")));
    }
    let canonical = std::fs::canonicalize(path)?;
    if canonical != path {
        return Err(invalid(format!(
            "{label} has a symlink or noncanonical component"
        )));
    }
    let metadata = std::fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink()
        || (directory && !metadata.is_dir())
        || (!directory && !metadata.is_file())
    {
        return Err(invalid(format!("{label} has the wrong file type")));
    }
    path_text(&canonical, label)
}

fn path_is_at_or_below(path: &str, root: &str) -> bool {
    Path::new(path) == Path::new(root) || Path::new(path).starts_with(Path::new(root))
}

fn paths_overlap(left: &str, right: &str) -> bool {
    path_is_at_or_below(left, right) || path_is_at_or_below(right, left)
}

fn validate_child_name(value: &str) -> Result<&str, RestartCollectorErrorV3> {
    if value.is_empty()
        || value.len() > 255
        || value == "."
        || value == ".."
        || value.contains(['/', '\0'])
    {
        return Err(invalid("artifact name is not one bounded child name"));
    }
    Ok(value)
}

fn monotonic_nanoseconds() -> Result<u64, RestartCollectorErrorV3> {
    let mut value = std::mem::MaybeUninit::<libc::timespec>::zeroed();
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_RAW, value.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    let value = unsafe { value.assume_init() };
    if value.tv_sec < 0 || value.tv_nsec < 0 || value.tv_nsec >= 1_000_000_000 {
        return Err(invalid("monotonic clock returned an invalid value"));
    }
    (value.tv_sec as u64)
        .checked_mul(1_000_000_000)
        .and_then(|seconds| seconds.checked_add(value.tv_nsec as u64))
        .filter(|value| *value != 0)
        .ok_or_else(|| invalid("monotonic clock overflowed or returned zero"))
}

fn binding_from_stat(
    stat: libc::stat,
) -> Result<FilesystemObjectBindingV3, RestartCollectorErrorV3> {
    if stat.st_size < 0
        || !(0..1_000_000_000).contains(&stat.st_birthtime_nsec)
        || !(0..1_000_000_000).contains(&stat.st_ctime_nsec)
        || !(0..1_000_000_000).contains(&stat.st_mtime_nsec)
    {
        return Err(invalid("filesystem identity has invalid stat fields"));
    }
    let binding = FilesystemObjectBindingV3 {
        birthtime_nanoseconds: stat.st_birthtime_nsec,
        birthtime_seconds: stat.st_birthtime,
        ctime_nanoseconds: stat.st_ctime_nsec,
        ctime_seconds: stat.st_ctime,
        dev: stat.st_dev as u64,
        flags: stat.st_flags,
        generation: stat.st_gen,
        gid: stat.st_gid,
        inode: stat.st_ino,
        mode: stat.st_mode as u32,
        mtime_nanoseconds: stat.st_mtime_nsec,
        mtime_seconds: stat.st_mtime,
        nlink: stat.st_nlink as u64,
        size: stat.st_size as u64,
        uid: stat.st_uid,
    };
    if binding.dev == 0 || binding.inode == 0 || binding.nlink == 0 {
        return Err(invalid("filesystem identity has a zero stable field"));
    }
    Ok(binding)
}

fn fstat_binding(
    fd: RawFd,
    _label: &str,
) -> Result<FilesystemObjectBindingV3, RestartCollectorErrorV3> {
    let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
    if unsafe { libc::fstat(fd, stat.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    binding_from_stat(unsafe { stat.assume_init() })
}

fn lstat_binding(
    path: &Path,
    label: &str,
) -> Result<FilesystemObjectBindingV3, RestartCollectorErrorV3> {
    let c_path = CString::new(path.as_os_str().as_encoded_bytes())
        .map_err(|_| invalid(format!("{label} path contains NUL")))?;
    let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
    if unsafe { libc::lstat(c_path.as_ptr(), stat.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    binding_from_stat(unsafe { stat.assume_init() })
}

fn list_directory(fd: RawFd, maximum: usize) -> Result<Vec<String>, RestartCollectorErrorV3> {
    let dot = c".";
    let reopened = unsafe {
        libc::openat(
            fd,
            dot.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    if reopened < 0 {
        return Err(io::Error::last_os_error().into());
    }
    let directory = unsafe { libc::fdopendir(reopened) };
    if directory.is_null() {
        unsafe { libc::close(reopened) };
        return Err(io::Error::last_os_error().into());
    }
    let mut names = Vec::new();
    loop {
        unsafe { *libc::__error() = 0 };
        let entry = unsafe { libc::readdir(directory) };
        if entry.is_null() {
            let errno = unsafe { *libc::__error() };
            let close_rc = unsafe { libc::closedir(directory) };
            if errno != 0 {
                return Err(io::Error::from_raw_os_error(errno).into());
            }
            if close_rc != 0 {
                return Err(io::Error::last_os_error().into());
            }
            break;
        }
        let bytes = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
        let name = std::str::from_utf8(bytes).map_err(|_| {
            unsafe { libc::closedir(directory) };
            invalid("artifact directory contains a non-UTF-8 name")
        })?;
        if name == "." || name == ".." {
            continue;
        }
        if names.len() == maximum {
            unsafe { libc::closedir(directory) };
            return Err(invalid("artifact directory exceeds its roster bound"));
        }
        names.push(name.to_string());
    }
    names.sort();
    Ok(names)
}

fn fixed_c_string(bytes: &[libc::c_char], label: &str) -> Result<String, RestartCollectorErrorV3> {
    let nul = bytes
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| invalid(format!("{label} is not NUL-terminated")))?;
    if nul == 0 || nul > MAX_MOUNT_STRING_BYTES {
        return Err(invalid(format!("{label} is empty or exceeds its bound")));
    }
    let raw = bytes[..nul]
        .iter()
        .map(|byte| *byte as u8)
        .collect::<Vec<_>>();
    String::from_utf8(raw).map_err(|_| invalid(format!("{label} is not UTF-8")))
}

fn mount_binding(filesystem: &libc::statfs) -> Result<MountBindingV3, RestartCollectorErrorV3> {
    Ok(MountBindingV3 {
        filesystem_id: unsafe { std::mem::transmute::<libc::fsid_t, [i32; 2]>(filesystem.f_fsid) },
        filesystem_type: fixed_c_string(&filesystem.f_fstypename, "mount-table filesystem type")?,
        mount_flags: filesystem.f_flags as u64,
        mount_from: fixed_c_string(&filesystem.f_mntfromname, "mount-table source")?,
        mount_on: fixed_c_string(&filesystem.f_mntonname, "mount-table target")?,
    })
}

fn mount_table_snapshot() -> Result<Vec<MountBindingV3>, RestartCollectorErrorV3> {
    let count = unsafe { libc::getfsstat(std::ptr::null_mut(), 0, libc::MNT_NOWAIT) };
    if count < 0 {
        return Err(io::Error::last_os_error().into());
    }
    let count = usize::try_from(count).map_err(|_| invalid("mount count overflowed"))?;
    if count > MAX_MOUNT_ENTRIES {
        return Err(invalid("mount table exceeds its closed-world bound"));
    }
    let capacity = count
        .checked_add(16)
        .filter(|value| *value <= MAX_MOUNT_ENTRIES)
        .ok_or_else(|| invalid("mount table growth allowance exceeds its bound"))?;
    let mut values = vec![unsafe { std::mem::zeroed::<libc::statfs>() }; capacity];
    let bytes = values
        .len()
        .checked_mul(std::mem::size_of::<libc::statfs>())
        .and_then(|value| i32::try_from(value).ok())
        .ok_or_else(|| invalid("mount-table buffer size overflowed"))?;
    let observed = unsafe { libc::getfsstat(values.as_mut_ptr(), bytes, libc::MNT_NOWAIT) };
    if observed < 0 {
        return Err(io::Error::last_os_error().into());
    }
    let observed = usize::try_from(observed).map_err(|_| invalid("mount count overflowed"))?;
    if observed > values.len() {
        return Err(invalid("mount table grew beyond its bounded snapshot"));
    }
    values.truncate(observed);
    let mut mounts = values
        .iter()
        .map(mount_binding)
        .collect::<Result<Vec<_>, _>>()?;
    mounts.sort();
    if mounts.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(invalid("mount table contains a duplicate binding"));
    }
    Ok(mounts)
}

fn reject_nested_mounts(
    mounts: &[MountBindingV3],
    policy: &RestartCollectorPolicyV3,
) -> Result<(), RestartCollectorErrorV3> {
    for root in &policy.protected_roots {
        let prefix = format!("{}/", root.trim_end_matches('/'));
        if mounts
            .iter()
            .any(|mount| mount.mount_on == *root || mount.mount_on.starts_with(&prefix))
        {
            return Err(invalid(format!(
                "nested mount exists at or below protected root {root}"
            )));
        }
    }
    let prefix = format!("{}/", policy.mountpoint.trim_end_matches('/'));
    if mounts
        .iter()
        .any(|mount| mount.mount_on.starts_with(&prefix))
    {
        return Err(invalid(
            "nested mount exists below the policy-bound mountpoint",
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "mac_disposable_reconciliation_collector_tests.rs"]
mod tests;
impl RetainedCollectorLineageV3 {
    pub(crate) fn into_mount_delta(
        self,
        command: &ExactDisposableCommandV3,
    ) -> Result<RetainedCollectorMountDeltaV3<MountingV3>, RestartCollectorErrorV3> {
        self.revalidate_bound()?;
        let ExactMountDeltaCommandViewV3::Mount {
            binding,
            mountpoint_underlying_sha256,
            read_only,
            volume_identity_sha256,
        } = command
            .mount_delta_view()
            .ok_or_else(|| invalid("mount delta requires the exact durable mount command"))?
        else {
            return Err(invalid("mount delta cannot consume an unmount command"));
        };
        let current = self.current_observation();
        let evidence = current.evidence();
        let unique = match current {
            RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueAttached(unique),
            ) => unique,
            _ => {
                return Err(invalid(
                    "mount delta requires one retained unique-attached collector match",
                ));
            }
        };
        let group = exact_unique_group(&unique.evidence.receipt)?;
        let target = mount_binding_from_command(binding);
        validate_mount_binding_shape(&target)?;
        if mountpoint_underlying_sha256 != evidence.receipt.mountpoint_underlying_sha256
            || volume_identity_sha256 != unique_volume_identity_sha256(group)?
            || target.mount_on != evidence.receipt.collector_policy.mountpoint
            || !group_source_matches(group, &target.mount_from)
            || ((target.mount_flags & libc::MNT_RDONLY as u64) != 0) != read_only
        {
            return Err(invalid(
                "mount command, unique volume, mountpoint, access flags, or exact entry differ",
            ));
        }
        let before = evidence.receipt.mount_evidence.mounts_after.clone();
        if before != evidence.guard.mounts
            || before.iter().any(|mount| {
                mount.mount_on == target.mount_on || group_source_matches(group, &mount.mount_from)
            })
        {
            return Err(invalid(
                "unique-attached evidence already owns or aliases the target mount entry",
            ));
        }
        let mut after = before.clone();
        after.push(target.clone());
        after.sort();
        if after.windows(2).any(|pair| pair[0] == pair[1])
            || exact_added_mount(&before, &after) != Some(&target)
        {
            return Err(invalid(
                "mount expected-after is not exactly before plus one target entry",
            ));
        }
        Ok(RetainedCollectorMountDeltaV3 {
            after,
            before,
            command_sha256: sha256(&canonical_json(command)?),
            operation_nonce: evidence.receipt.operation_nonce.clone(),
            prior: self,
            target,
            _kind: PhantomData,
            _not_send_or_sync: PhantomData,
        })
    }

    fn into_unmount_delta(
        self,
        command: &ExactDisposableCommandV3,
    ) -> Result<RetainedCollectorMountDeltaV3<UnmountingV3>, RestartCollectorErrorV3> {
        self.revalidate_bound()?;
        let ExactMountDeltaCommandViewV3::Unmount {
            mounted_binding_sha256,
        } = command
            .mount_delta_view()
            .ok_or_else(|| invalid("unmount delta requires the exact durable unmount command"))?
        else {
            return Err(invalid("unmount delta cannot consume a mount command"));
        };
        let current = self.current_observation();
        let evidence = current.evidence();
        let unique = match current {
            RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueMounted(unique),
            ) => unique,
            _ => {
                return Err(invalid(
                    "unmount delta requires one retained unique-mounted collector match",
                ));
            }
        };
        let group = exact_unique_group(&unique.evidence.receipt)?;
        let before = evidence.receipt.mount_evidence.mounts_after.clone();
        if before != evidence.guard.mounts {
            return Err(invalid(
                "unique-mounted retained guard differs from its exact receipt snapshot",
            ));
        }
        let targets = before
            .iter()
            .filter(|mount| {
                mount.mount_on == evidence.receipt.collector_policy.mountpoint
                    && group_source_matches(group, &mount.mount_from)
            })
            .cloned()
            .collect::<Vec<_>>();
        let [target] = targets.as_slice() else {
            return Err(invalid(
                "unique-mounted evidence does not have exactly one target mount entry",
            ));
        };
        if sha256(&canonical_json(target)?) != mounted_binding_sha256 {
            return Err(invalid(
                "unmount command digest differs from the exact mounted binding",
            ));
        }
        let mut after = before.clone();
        let index = after
            .binary_search(target)
            .map_err(|_| invalid("target mount is absent from the sorted exact census"))?;
        after.remove(index);
        if exact_removed_mount(&before, &after) != Some(target) {
            return Err(invalid(
                "unmount expected-after is not exactly before minus one target entry",
            ));
        }
        Ok(RetainedCollectorMountDeltaV3 {
            after,
            before,
            command_sha256: sha256(&canonical_json(command)?),
            operation_nonce: evidence.receipt.operation_nonce.clone(),
            prior: self,
            target: target.clone(),
            _kind: PhantomData,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl<K> RetainedCollectorMountDeltaV3<K> {
    pub(crate) fn sealed_plan(&self) -> SealedMountDeltaPlanV3<'_, K> {
        SealedMountDeltaPlanV3 {
            delta: self,
            _not_send_or_sync: PhantomData,
        }
    }

    pub(crate) fn revalidate_snapshot(
        &self,
        observed: &[MountBindingV3],
    ) -> Result<(), RestartCollectorErrorV3> {
        if observed != self.before && observed != self.after {
            return Err(invalid(
                "pending mount census observed neither exact before nor exact expected-after",
            ));
        }
        if observed == self.before {
            self.prior.revalidate_bound()?;
        } else {
            let direction = if exact_added_mount(&self.before, &self.after).is_some() {
                MountDeltaDirectionV3::Mount
            } else if exact_removed_mount(&self.before, &self.after).is_some() {
                MountDeltaDirectionV3::Unmount
            } else {
                return Err(invalid("pending mount delta shape changed"));
            };
            self.prior
                .revalidate_across_mount_delta(&self.after, direction)?;
        }
        Ok(())
    }

    pub(crate) fn revalidate_live_pending(&self) -> Result<(), RestartCollectorErrorV3> {
        let observed = mount_table_snapshot()?;
        self.revalidate_snapshot(&observed)
    }

    fn validate_post_delta(
        &self,
        next: &RetainedCollectorObservationV3,
        direction: MountDeltaDirectionV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        next.revalidate_bound()?;
        let prior = self.prior.current_observation().evidence();
        let next_evidence = next.evidence();
        let expected_match = match direction {
            MountDeltaDirectionV3::Mount => ReconciliationMatchV2::Unique { mounted: true },
            MountDeltaDirectionV3::Unmount => ReconciliationMatchV2::Unique { mounted: false },
        };
        let expected_underlying_revalidated = matches!(direction, MountDeltaDirectionV3::Unmount);
        if next_evidence.receipt.purpose != CollectorPurposeV3::ReconciliationSnapshot
            || next_evidence.receipt.match_result != expected_match
            || next_evidence
                .receipt
                .mount_evidence
                .mountpoint_underlying_revalidated
                != expected_underlying_revalidated
            || next_evidence.receipt.mount_evidence.mounts_before != self.after
            || next_evidence.receipt.mount_evidence.mounts_after != self.after
            || next_evidence.guard.mounts != self.after
            || next_evidence.receipt.operation_nonce != self.operation_nonce
            || next_evidence.receipt.operation_nonce != prior.receipt.operation_nonce
            || next_evidence.receipt.boot_session_uuid != prior.receipt.boot_session_uuid
            || next_evidence.receipt.restart_epoch_nonce != prior.receipt.restart_epoch_nonce
            || next_evidence.receipt.collector_policy_sha256
                != prior.receipt.collector_policy_sha256
            || next_evidence.receipt.backing_identity_sha256
                != prior.receipt.backing_identity_sha256
            || next_evidence.receipt.mountpoint_underlying_sha256
                != prior.receipt.mountpoint_underlying_sha256
            || next_evidence.receipt.matching_groups != prior.receipt.matching_groups
        {
            return Err(invalid(
                "post-delta retained collector does not bind the exact operation, epoch, unique group, or expected mount snapshot",
            ));
        }
        self.prior
            .revalidate_across_mount_delta(&self.after, direction)?;
        if mount_table_snapshot()? != self.after {
            return Err(invalid(
                "mount table changed after post-delta collector final replay",
            ));
        }
        Ok(())
    }

    fn validate_unadopted_observation(
        &self,
        next: &UnadoptedCollectorGenerationV3,
        direction: MountDeltaDirectionV3,
    ) -> Result<(), RestartCollectorErrorV3> {
        next.revalidate()?;
        let prior = self.prior.current_observation().evidence();
        let next_evidence = &next.core;
        let expected_match = match direction {
            MountDeltaDirectionV3::Mount => ReconciliationMatchV2::Unique { mounted: true },
            MountDeltaDirectionV3::Unmount => ReconciliationMatchV2::Unique { mounted: false },
        };
        let expected_underlying_revalidated = matches!(direction, MountDeltaDirectionV3::Unmount);
        if next_evidence.receipt.purpose != CollectorPurposeV3::ReconciliationSnapshot
            || next_evidence.receipt.match_result != expected_match
            || next_evidence
                .receipt
                .mount_evidence
                .mountpoint_underlying_revalidated
                != expected_underlying_revalidated
            || next_evidence.receipt.mount_evidence.mounts_before != self.after
            || next_evidence.receipt.mount_evidence.mounts_after != self.after
            || next_evidence.guard.mounts != self.after
            || next_evidence.receipt.operation_nonce != self.operation_nonce
            || next_evidence.receipt.operation_nonce != prior.receipt.operation_nonce
            || next_evidence.receipt.boot_session_uuid != prior.receipt.boot_session_uuid
            || next_evidence.receipt.restart_epoch_nonce != prior.receipt.restart_epoch_nonce
            || next_evidence.receipt.collector_policy_sha256
                != prior.receipt.collector_policy_sha256
            || next_evidence.receipt.backing_identity_sha256
                != prior.receipt.backing_identity_sha256
            || next_evidence.receipt.mountpoint_underlying_sha256
                != prior.receipt.mountpoint_underlying_sha256
            || next_evidence.receipt.matching_groups != prior.receipt.matching_groups
        {
            return Err(invalid(
                "unadopted post-delta collector does not bind the exact operation, epoch, unique group, or expected mount snapshot",
            ));
        }
        self.prior
            .revalidate_across_mount_delta(&self.after, direction)?;
        if mount_table_snapshot()? != self.after {
            return Err(invalid(
                "mount table changed after unadopted post-delta collector final replay",
            ));
        }
        Ok(())
    }
}

impl RetainedCollectorMountDeltaV3<MountingV3> {
    pub(crate) fn seal_unadopted_observation<'a>(
        &'a self,
        next: &'a UnadoptedCollectorGenerationV3,
    ) -> Result<SealedUnadoptedMountDeltaObservationV3<'a, MountingV3>, RestartCollectorErrorV3>
    {
        self.validate_unadopted_observation(next, MountDeltaDirectionV3::Mount)?;
        Ok(SealedUnadoptedMountDeltaObservationV3 {
            delta: self,
            next,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn seal_advance<'a>(
        &'a self,
        next: &'a RetainedCollectorObservationV3,
    ) -> Result<SealedMountDeltaAdvanceV3<'a, MountingV3>, RestartCollectorErrorV3> {
        self.validate_post_delta(next, MountDeltaDirectionV3::Mount)?;
        Ok(SealedMountDeltaAdvanceV3 {
            delta: self,
            next,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl RetainedCollectorMountDeltaV3<UnmountingV3> {
    pub(crate) fn seal_unadopted_observation<'a>(
        &'a self,
        next: &'a UnadoptedCollectorGenerationV3,
    ) -> Result<SealedUnadoptedMountDeltaObservationV3<'a, UnmountingV3>, RestartCollectorErrorV3>
    {
        self.validate_unadopted_observation(next, MountDeltaDirectionV3::Unmount)?;
        Ok(SealedUnadoptedMountDeltaObservationV3 {
            delta: self,
            next,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn seal_advance<'a>(
        &'a self,
        next: &'a RetainedCollectorObservationV3,
    ) -> Result<SealedMountDeltaAdvanceV3<'a, UnmountingV3>, RestartCollectorErrorV3> {
        self.validate_post_delta(next, MountDeltaDirectionV3::Unmount)?;
        Ok(SealedMountDeltaAdvanceV3 {
            delta: self,
            next,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn into_advanced_lineage(
        self,
        next: RetainedCollectorObservationV3,
    ) -> Result<RetainedCollectorLineageV3, RestartCollectorErrorV3> {
        self.validate_post_delta(&next, MountDeltaDirectionV3::Unmount)?;
        let RetainedCollectorMountDeltaV3 { after, prior, .. } = self;
        prior.into_mount_delta_current(MountDeltaDirectionV3::Unmount, after, next)
    }
}

impl<K> SealedMountDeltaPlanV3<'_, K> {
    pub(crate) fn before(&self) -> &[MountBindingV3] {
        &self.delta.before
    }

    pub(crate) fn expected_after(&self) -> &[MountBindingV3] {
        &self.delta.after
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.delta.operation_nonce
    }

    pub(crate) fn command_sha256(&self) -> &str {
        &self.delta.command_sha256
    }

    pub(crate) fn target(&self) -> &MountBindingV3 {
        &self.delta.target
    }
}

impl<K> SealedMountDeltaAdvanceV3<'_, K> {
    pub(crate) fn before(&self) -> &[MountBindingV3] {
        &self.delta.before
    }

    pub(crate) fn expected_after(&self) -> &[MountBindingV3] {
        &self.delta.after
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.delta.operation_nonce
    }

    pub(crate) fn command_sha256(&self) -> &str {
        &self.delta.command_sha256
    }

    pub(crate) fn target(&self) -> &MountBindingV3 {
        &self.delta.target
    }

    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.next.revalidate_bound()?;
        if self.next.evidence().receipt.mount_evidence.mounts_after != self.delta.after {
            return Err(invalid("sealed mount advance changed after construction"));
        }
        Ok(())
    }
}

impl<K> SealedUnadoptedMountDeltaObservationV3<'_, K> {
    pub(crate) fn mount_evidence_sha256(&self) -> &str {
        &self.next.core.receipt.mount_evidence_sha256
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.delta.operation_nonce
    }

    pub(crate) fn post_effect_collector_binding(
        &self,
    ) -> Result<PostEffectCollectorBindingV3, RestartCollectorErrorV3> {
        self.next.revalidate()?;
        let evidence = &self.next.core;
        let binding = PostEffectCollectorBindingV3::from_retained_collector(
            PostEffectCollectorBindingSealV3 { _private: () },
            evidence.receipt.boot_session_uuid.clone(),
            evidence.expected_lifecycle_binding.clone(),
            evidence.receipt_sha256.clone(),
            self.delta.prior.first_snapshot_sha256()?,
            evidence.receipt.mount_evidence_sha256.clone(),
            evidence.receipt.operation_nonce.clone(),
            evidence.receipt.restart_epoch_nonce.clone(),
        );
        if binding.operation_nonce() != self.delta.operation_nonce
            || binding.observation_sha256() != self.mount_evidence_sha256()
        {
            return Err(invalid(
                "unadopted post-effect collector binding changed across sealed observation",
            ));
        }
        Ok(binding)
    }
}

fn mount_binding_from_command(binding: &ExactMountBindingCommandV3) -> MountBindingV3 {
    MountBindingV3 {
        filesystem_id: binding.filesystem_id(),
        filesystem_type: binding.filesystem_type().to_string(),
        mount_flags: binding.mount_flags(),
        mount_from: binding.mount_from().to_string(),
        mount_on: binding.mount_on().to_string(),
    }
}

fn validate_mount_binding_shape(binding: &MountBindingV3) -> Result<(), RestartCollectorErrorV3> {
    for (value, label, absolute) in [
        (&binding.filesystem_type, "mount filesystem type", false),
        (&binding.mount_from, "mount source", false),
        (&binding.mount_on, "mount target", true),
    ] {
        if value.is_empty()
            || value.len() > MAX_MOUNT_STRING_BYTES
            || value.as_bytes().contains(&0)
            || (absolute && !value.starts_with('/'))
        {
            return Err(invalid(format!("{label} is malformed")));
        }
    }
    Ok(())
}

fn exact_unique_group(
    receipt: &RestartCollectorReceiptV3,
) -> Result<&MatchingDiskImageGroupV3, RestartCollectorErrorV3> {
    let [group] = receipt.matching_groups.as_slice() else {
        return Err(invalid(
            "retained unique collector receipt is not exactly unique",
        ));
    };
    Ok(group)
}

fn unique_volume_identity_sha256(
    group: &MatchingDiskImageGroupV3,
) -> Result<String, RestartCollectorErrorV3> {
    Ok(sha256(&canonical_json(group)?))
}

fn group_source_matches(group: &MatchingDiskImageGroupV3, source: &str) -> bool {
    group
        .member_bsd_names
        .iter()
        .any(|name| source == name || source == format!("/dev/{name}"))
}

fn exact_added_mount<'a>(
    before: &'a [MountBindingV3],
    after: &'a [MountBindingV3],
) -> Option<&'a MountBindingV3> {
    if !strictly_sorted_mounts(before)
        || !strictly_sorted_mounts(after)
        || after.len() != before.len().checked_add(1)?
    {
        return None;
    }
    let added = after
        .iter()
        .filter(|entry| before.binary_search(entry).is_err())
        .collect::<Vec<_>>();
    (added.len() == 1
        && before
            .iter()
            .all(|entry| after.binary_search(entry).is_ok()))
    .then_some(added[0])
}

fn exact_removed_mount<'a>(
    before: &'a [MountBindingV3],
    after: &[MountBindingV3],
) -> Option<&'a MountBindingV3> {
    if !strictly_sorted_mounts(before)
        || !strictly_sorted_mounts(after)
        || before.len() != after.len().checked_add(1)?
    {
        return None;
    }
    let removed = before
        .iter()
        .filter(|entry| after.binary_search(entry).is_err())
        .collect::<Vec<_>>();
    (removed.len() == 1
        && after
            .iter()
            .all(|entry| before.binary_search(entry).is_ok()))
    .then_some(removed[0])
}

fn strictly_sorted_mounts(mounts: &[MountBindingV3]) -> bool {
    mounts.windows(2).all(|pair| pair[0] < pair[1])
}
