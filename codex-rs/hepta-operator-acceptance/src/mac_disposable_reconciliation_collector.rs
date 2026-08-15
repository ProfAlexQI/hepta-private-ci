//! Read-only restart collector for disposable macOS disk-image lifecycles.
//!
//! This module has no effect primitive. It holds every observed IOMedia and
//! filesystem descriptor across canonical receipt persistence, performs a
//! final replay, and only then releases typed lifecycle observations.

use crate::AcceptanceError;
use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::mac_disposable_lifecycle::DisposableAuthorityV2;
use crate::mac_disposable_lifecycle::FreshAbsenceObservationV2;
use crate::mac_disposable_lifecycle::ReconciliationMatchV2;
use crate::mac_disposable_lifecycle::ReconciliationSnapshotV2;
use crate::mac_disposable_lifecycle::reconciliation_snapshot_sha256;
use crate::mac_iomedia_identity::DiskImageBackingIdentityV2;
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
pub struct RestartCollectorBindingsV3 {
    pub backing_identity_sha256: String,
    pub baseline_inventory_sha256: String,
    pub boot_session_uuid: String,
    pub collector_policy_sha256: String,
    pub mountpoint_underlying_sha256: String,
    pub operation_nonce: String,
    pub restart_epoch_nonce: String,
    pub restart_started_monotonic_nanoseconds: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestartCollectorPolicyV3 {
    pub artifacts: Vec<PreparedArtifactBindingV3>,
    pub artifact_root: String,
    pub artifact_root_identity: StableDirectoryIdentityV3,
    pub authority: DisposableAuthorityV2,
    pub backing_path: String,
    pub max_iomedia_objects: usize,
    pub max_mount_entries: usize,
    pub mountpoint: String,
    pub protected_roots: Vec<String>,
    pub receipt_root: String,
    pub receipt_root_identity: StableDirectoryIdentityV3,
    pub schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestartBaselineInventoryV3 {
    pub authority: DisposableAuthorityV2,
    pub boot_session_uuid: String,
    pub registry_entry_ids: Vec<String>,
    pub schema: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CollectorPurposeV3 {
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
pub struct FilesystemObjectBindingV3 {
    pub birthtime_nanoseconds: i64,
    pub birthtime_seconds: i64,
    pub ctime_nanoseconds: i64,
    pub ctime_seconds: i64,
    pub dev: u64,
    pub flags: u32,
    pub generation: u32,
    pub gid: u32,
    pub inode: u64,
    pub mode: u32,
    pub mtime_nanoseconds: i64,
    pub mtime_seconds: i64,
    pub nlink: u64,
    pub size: u64,
    pub uid: u32,
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
pub(crate) enum FinalizedRestartObservationV3 {
    ReconciliationSnapshot(ReconciliationSnapshotV2),
    FreshAbsence(FreshAbsenceObservationV2),
}

#[derive(Clone, Debug)]
pub struct LiveRestartCollectorRequestV3<'a> {
    pub artifact_root: &'a Path,
    pub baseline: &'a RestartBaselineInventoryV3,
    pub bindings: &'a RestartCollectorBindingsV3,
    pub mountpoint_identity: &'a MountpointIdentityV3,
    pub policy: &'a RestartCollectorPolicyV3,
    pub prepared_backing: &'a DiskImageBackingIdentityV2,
    pub receipt_root: &'a Path,
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
    mountpoint: Option<HeldDirectoryV3>,
    mounts: Vec<MountBindingV3>,
    prepared_backing: DiskImageBackingIdentityV2,
}

pub struct PendingRestartObservationV3 {
    existing_receipts: ReceiptRootSnapshotV3,
    guard: LiveReplayGuardV3,
    receipt: RestartCollectorReceiptV3,
    receipt_directory: HeldDirectoryV3,
}

struct ValidatedExistingReceiptV3 {
    binding: FilesystemObjectBindingV3,
    bytes: Vec<u8>,
    file: File,
    name: String,
    receipt: RestartCollectorReceiptV3,
}

struct ReceiptRootSnapshotV3 {
    aggregate_bytes: usize,
    entries: Vec<ValidatedExistingReceiptV3>,
    roster: Vec<String>,
}

struct DurableCollectorReceiptV3 {
    bytes: Vec<u8>,
    directory: File,
    directory_binding: FilesystemObjectBindingV3,
    existing_receipts: ReceiptRootSnapshotV3,
    file: File,
    file_binding: FilesystemObjectBindingV3,
    final_name: String,
    path: PathBuf,
    roster: Vec<String>,
    temporary_name: String,
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

impl RestartCollectorPolicyV3 {
    pub fn new(
        backing_path: &Path,
        mountpoint: &Path,
        artifact_root: &Path,
        receipt_root: &Path,
        artifacts: &[PreparedArtifactBindingV3],
        protected_roots: &[&Path],
    ) -> Result<Self, RestartCollectorErrorV3> {
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
        let policy = Self {
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
        Ok(policy)
    }

    pub fn sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        validate_policy(self)?;
        Ok(sha256(&canonical_json(self)?))
    }
}

impl RestartBaselineInventoryV3 {
    pub fn from_inventory(
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

    pub fn sha256(&self) -> Result<String, RestartCollectorErrorV3> {
        validate_baseline(self)?;
        Ok(sha256(&canonical_json(self)?))
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

pub fn capture_live_restart_baseline_v3()
-> Result<RestartBaselineInventoryV3, RestartCollectorErrorV3> {
    let held = capture_restart_iomedia_inventory_v3()?;
    let baseline = RestartBaselineInventoryV3::from_inventory(held.report())?;
    held.revalidate_after_persistence()?;
    Ok(baseline)
}

pub fn capture_live_backing_identity_v2(
    path: &Path,
) -> Result<DiskImageBackingIdentityV2, RestartCollectorErrorV3> {
    let held = hold_disk_image_backing(path)?;
    let identity = held.identity()?;
    held.revalidate_identity_after_persistence(&identity)?;
    Ok(identity)
}

pub fn collect_reconciliation_snapshot_v3(
    request: LiveRestartCollectorRequestV3<'_>,
) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3> {
    collect_live(request, CollectorPurposeV3::ReconciliationSnapshot, None)
}

pub fn collect_fresh_absence_v3(
    request: LiveRestartCollectorRequestV3<'_>,
    snapshot: &ReconciliationSnapshotV2,
) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3> {
    validate_reconciliation_snapshot_shape_v3(snapshot)?;
    if snapshot.match_result != ReconciliationMatchV2::Zero
        || snapshot.operation_nonce != request.bindings.operation_nonce
        || snapshot.restart_epoch_nonce != request.bindings.restart_epoch_nonce
        || snapshot.boot_session_uuid != request.bindings.boot_session_uuid
        || snapshot.collector_policy_sha256 != request.bindings.collector_policy_sha256
        || snapshot.backing_identity_sha256 != request.bindings.backing_identity_sha256
        || snapshot.mountpoint_underlying_sha256 != request.bindings.mountpoint_underlying_sha256
    {
        return Err(invalid(
            "fresh absence requires the exact current-epoch Zero reconciliation snapshot",
        ));
    }
    let snapshot_sha = reconciliation_snapshot_sha256(snapshot)
        .map_err(|error| invalid(format!("reconciliation snapshot digest failed: {error}")))?;
    collect_live(
        request,
        CollectorPurposeV3::FreshAbsence,
        Some((snapshot, snapshot_sha)),
    )
}

fn collect_live(
    request: LiveRestartCollectorRequestV3<'_>,
    purpose: CollectorPurposeV3,
    prior_snapshot: Option<(&ReconciliationSnapshotV2, String)>,
) -> Result<PendingRestartObservationV3, RestartCollectorErrorV3> {
    validate_request(&request)?;
    // Hold both prepared roots from the first live-collection boundary through
    // durable persistence and final replay. Stable policy identities reject a
    // same-path replacement that occurred after preparation, while the full
    // bindings below reject any later metadata or roster churn.
    let artifact_root = HeldDirectoryV3::capture(request.artifact_root, "artifact root")?;
    let receipt_directory = HeldDirectoryV3::capture(request.receipt_root, "receipt root")?;
    let artifact_root_roster =
        list_directory(artifact_root.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES)?;
    let receipt_root_roster =
        list_directory(receipt_directory.file.as_raw_fd(), MAX_RECEIPT_FILES)?;
    artifact_root.revalidate("artifact root")?;
    receipt_directory.revalidate("receipt root")?;
    if !request
        .policy
        .artifact_root_identity
        .matches_binding(&artifact_root.binding, artifact_root_roster.len())
        || !request
            .policy
            .receipt_root_identity
            .matches_binding(&receipt_directory.binding, receipt_root_roster.len())
    {
        return Err(invalid(
            "live collector roots differ from their prepared stable identities",
        ));
    }
    validate_receipt_directory(&receipt_directory.binding)?;
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

    let iomedia = capture_restart_iomedia_inventory_v3()?;
    if iomedia.report().boot_session_uuid != request.bindings.boot_session_uuid {
        return Err(invalid("restart IOMedia inventory belongs to another boot"));
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
        None
    } else {
        let held = HeldDirectoryV3::capture(Path::new(&request.policy.mountpoint), "mountpoint")?;
        if mountpoint_identity_from_held(&held)? != *request.mountpoint_identity {
            return Err(invalid(
                "restart mountpoint differs from the prepared underlying identity",
            ));
        }
        Some(held)
    };

    let existing_receipts = capture_receipt_root_closed_world(&receipt_directory)?;
    if let Some((snapshot, _)) = prior_snapshot.as_ref() {
        validate_prior_snapshot_receipt(
            snapshot,
            &request.bindings.baseline_inventory_sha256,
            &existing_receipts,
        )?;
    }
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
    let baseline_restored = current_baseline == *request.baseline;
    let mount_evidence = MountEvidenceV3 {
        authority: DisposableAuthorityV2::none(),
        mountpoint_underlying_revalidated: mountpoint.is_some(),
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
    Ok(PendingRestartObservationV3 {
        existing_receipts,
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
        receipt_directory,
    })
}

impl PendingRestartObservationV3 {
    pub fn receipt(&self) -> &RestartCollectorReceiptV3 {
        &self.receipt
    }

    pub(crate) fn persist_and_retain(
        self,
    ) -> Result<RetainedCollectorObservationV3, RestartCollectorErrorV3> {
        self.persist_and_retain_inner(|| Ok(()))
    }

    #[cfg(test)]
    fn persist_and_retain_with_hook<F>(
        self,
        after_persistence: F,
    ) -> Result<RetainedCollectorObservationV3, RestartCollectorErrorV3>
    where
        F: FnOnce() -> Result<(), RestartCollectorErrorV3>,
    {
        self.persist_and_retain_inner(after_persistence)
    }

    fn persist_and_retain_inner<F>(
        self,
        after_persistence: F,
    ) -> Result<RetainedCollectorObservationV3, RestartCollectorErrorV3>
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
        self.receipt_directory.revalidate("receipt root")?;
        let durable = DurableCollectorReceiptV3::persist(
            &self.receipt_directory,
            self.existing_receipts,
            &self.receipt,
            bytes.clone(),
            &receipt_sha256,
        )?;
        after_persistence()?;

        // No typed lifecycle observation exists before every held descriptor
        // and the complete mount table have survived this post-persistence
        // replay.
        self.guard.revalidate(&self.receipt)?;
        durable.revalidate()?;
        let decoded: RestartCollectorReceiptV3 = serde_json::from_slice(&bytes)
            .map_err(|error| invalid(format!("persisted receipt JSON failed: {error}")))?;
        if decoded != self.receipt || canonical_json(&decoded)? != bytes {
            return Err(invalid(
                "canonical restart collector receipt failed final replay",
            ));
        }

        let purpose = self.receipt.purpose;
        let match_result = self.receipt.match_result;
        let observation = match purpose {
            CollectorPurposeV3::ReconciliationSnapshot => {
                FinalizedRestartObservationV3::ReconciliationSnapshot(
                    reconciliation_snapshot_from_receipt(&self.receipt, &receipt_sha256)?,
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
                )?)
            }
        };
        let evidence = RetainedCollectorEvidenceV3 {
            durable,
            guard: self.guard,
            observation,
            receipt: self.receipt,
            receipt_sha256,
            _not_send_or_sync: PhantomData,
        };
        match (purpose, match_result) {
            (CollectorPurposeV3::ReconciliationSnapshot, ReconciliationMatchV2::Zero) => {
                Ok(RetainedCollectorObservationV3::Reconciliation(
                    RetainedCollectorMatchV3::Zero(RetainedZeroMatchV3 { evidence }),
                ))
            }
            (
                CollectorPurposeV3::ReconciliationSnapshot,
                ReconciliationMatchV2::Unique { mounted: false },
            ) => Ok(RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueAttached(RetainedUniqueMatchV3 {
                    evidence,
                    _state: PhantomData,
                }),
            )),
            (
                CollectorPurposeV3::ReconciliationSnapshot,
                ReconciliationMatchV2::Unique { mounted: true },
            ) => Ok(RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::UniqueMounted(RetainedUniqueMatchV3 {
                    evidence,
                    _state: PhantomData,
                }),
            )),
            (
                CollectorPurposeV3::ReconciliationSnapshot,
                ReconciliationMatchV2::Ambiguous { .. },
            ) => Ok(RetainedCollectorObservationV3::Reconciliation(
                RetainedCollectorMatchV3::Ambiguous(RetainedAmbiguousMatchV3 { evidence }),
            )),
            (CollectorPurposeV3::FreshAbsence, ReconciliationMatchV2::Zero) => Ok(
                RetainedCollectorObservationV3::FreshAbsence(RetainedFreshAbsenceV3 { evidence }),
            ),
            (CollectorPurposeV3::FreshAbsence, _) => Err(invalid(
                "FreshAbsence retained evidence must have an exact Zero match",
            )),
        }
    }
}

impl RetainedCollectorEvidenceV3 {
    fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        self.guard.revalidate(&self.receipt)?;
        self.durable.revalidate()?;
        if sha256(&canonical_json(&self.receipt)?) != self.receipt_sha256 {
            return Err(invalid(
                "retained collector receipt digest changed after final replay",
            ));
        }
        let expected = match self.receipt.purpose {
            CollectorPurposeV3::ReconciliationSnapshot => {
                FinalizedRestartObservationV3::ReconciliationSnapshot(
                    reconciliation_snapshot_from_receipt(&self.receipt, &self.receipt_sha256)?,
                )
            }
            CollectorPurposeV3::FreshAbsence => FinalizedRestartObservationV3::FreshAbsence(
                fresh_absence_from_receipt(&self.receipt, &self.receipt_sha256)?,
            ),
        };
        if self.observation != expected {
            return Err(invalid(
                "retained collector observation differs from its durable receipt",
            ));
        }
        Ok(())
    }
}

impl RetainedCollectorObservationV3 {
    pub(crate) fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        match self {
            Self::Reconciliation(match_result) => match match_result {
                RetainedCollectorMatchV3::Zero(value) => value.evidence.revalidate(),
                RetainedCollectorMatchV3::UniqueAttached(value) => value.evidence.revalidate(),
                RetainedCollectorMatchV3::UniqueMounted(value) => value.evidence.revalidate(),
                RetainedCollectorMatchV3::Ambiguous(value) => value.evidence.revalidate(),
            },
            Self::FreshAbsence(value) => value.evidence.revalidate(),
        }
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
    Ok(())
}

fn reconciliation_snapshot_from_receipt(
    receipt: &RestartCollectorReceiptV3,
    receipt_sha256: &str,
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
) -> Result<FreshAbsenceObservationV2, RestartCollectorErrorV3> {
    validate_receipt(receipt)?;
    if receipt.purpose != CollectorPurposeV3::FreshAbsence
        || receipt.match_result != ReconciliationMatchV2::Zero
        || !receipt.baseline_restored
        || !receipt.operation_artifacts_absent
        || !receipt.mount_evidence.no_nested_mounts
        || !receipt.mount_evidence.mountpoint_underlying_revalidated
        || receipt.reconciliation_snapshot_sha256.is_none()
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
    fn revalidate(&self, directory: &HeldDirectoryV3) -> Result<(), RestartCollectorErrorV3> {
        directory.revalidate("receipt root")?;
        if list_directory(directory.file.as_raw_fd(), MAX_RECEIPT_FILES)? != self.roster
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
        let receipt_root = path_text(&directory.path, "receipt root")?;
        for entry in &self.entries {
            entry.revalidate(directory.file.as_raw_fd(), &receipt_root)?;
        }
        directory.revalidate("receipt root")
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

fn capture_receipt_root_closed_world(
    directory: &HeldDirectoryV3,
) -> Result<ReceiptRootSnapshotV3, RestartCollectorErrorV3> {
    directory.revalidate("receipt root")?;
    validate_receipt_directory(&directory.binding)?;
    let roster = list_directory(directory.file.as_raw_fd(), MAX_RECEIPT_FILES)?;
    if roster.len() >= MAX_RECEIPT_FILES {
        return Err(invalid(
            "collector receipt root has no bounded descriptor slot for a new receipt",
        ));
    }
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
            file,
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
        let snapshot = reconciliation_snapshot_from_receipt(&candidate.receipt, receipt_sha256)?;
        let snapshot_sha256 = reconciliation_snapshot_sha256(&snapshot)
            .map_err(|error| invalid(format!("prior snapshot digest failed: {error}")))?;
        if &snapshot_sha256 == expected
            && snapshot.match_result == ReconciliationMatchV2::Zero
            && snapshot.operation_nonce == receipt.operation_nonce
            && snapshot.restart_epoch_nonce == receipt.restart_epoch_nonce
            && snapshot.boot_session_uuid == receipt.boot_session_uuid
            && snapshot.collector_policy_sha256 == receipt.collector_policy_sha256
            && snapshot.backing_identity_sha256 == receipt.backing_identity_sha256
            && snapshot.mountpoint_underlying_sha256 == receipt.mountpoint_underlying_sha256
            && candidate.receipt.baseline_inventory_sha256 == receipt.baseline_inventory_sha256
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

fn validate_prior_snapshot_receipt(
    snapshot: &ReconciliationSnapshotV2,
    baseline_inventory_sha256: &str,
    receipts: &ReceiptRootSnapshotV3,
) -> Result<(), RestartCollectorErrorV3> {
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
        || reconciliation_snapshot_from_receipt(&entry.receipt, &snapshot.collector_receipt_sha256)?
            != *snapshot
    {
        return Err(invalid(
            "prior durable reconciliation receipt does not match the baseline or supplied snapshot",
        ));
    }
    Ok(())
}

impl DurableCollectorReceiptV3 {
    fn persist(
        held_directory: &HeldDirectoryV3,
        existing_receipts: ReceiptRootSnapshotV3,
        receipt: &RestartCollectorReceiptV3,
        bytes: Vec<u8>,
        receipt_sha256: &str,
    ) -> Result<Self, RestartCollectorErrorV3> {
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
        existing_receipts.revalidate(held_directory)?;
        let path = held_directory.path.clone();
        let directory = held_directory.file.try_clone()?;
        let before = fstat_binding(directory.as_raw_fd(), "collector receipt directory")?;
        if before != held_directory.binding {
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
        let roster_before = existing_receipts.roster.clone();
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
        existing_receipts
            .revalidate_entries(directory.as_raw_fd(), &path_text(&path, "receipt root")?)?;
        Ok(Self {
            bytes,
            directory,
            directory_binding: after,
            existing_receipts,
            file,
            file_binding,
            final_name,
            path,
            roster,
            temporary_name,
        })
    }

    fn revalidate(&self) -> Result<(), RestartCollectorErrorV3> {
        let directory = fstat_binding(self.directory.as_raw_fd(), "collector receipt directory")?;
        if self.directory_binding != directory
            || directory != lstat_binding(&self.path, "receipt root")?
            || list_directory(self.directory.as_raw_fd(), MAX_RECEIPT_FILES)? != self.roster
        {
            return Err(invalid(
                "collector receipt directory changed before final replay",
            ));
        }
        verify_fd_binding_secure(
            self.directory.as_raw_fd(),
            &self.directory_binding,
            "collector receipt directory",
        )?;
        if lstat_binding(&self.path, "receipt root")? != self.directory_binding {
            return Err(invalid(
                "collector receipt directory pathname changed during final ACL/xattr replay",
            ));
        }
        let receipt_root = path_text(&self.path, "receipt root")?;
        self.existing_receipts
            .revalidate_entries(self.directory.as_raw_fd(), &receipt_root)?;
        let named = fstatat_binding(
            self.directory.as_raw_fd(),
            &self.final_name,
            "collector receipt pathname",
        )?;
        let held = fstat_binding(self.file.as_raw_fd(), "held collector receipt")?;
        if named != self.file_binding
            || held != self.file_binding
            || read_fd_exact(&self.file, &held)? != self.bytes
            || self.roster.binary_search(&self.temporary_name).is_ok()
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
        validate_fresh_receipt_prior_relationship(&receipt, &self.existing_receipts.entries)?;
        verify_fd_binding_secure(
            self.directory.as_raw_fd(),
            &self.directory_binding,
            "collector receipt directory",
        )?;
        if lstat_binding(&self.path, "receipt root")? != self.directory_binding {
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
        if let Some(mountpoint) = &self.mountpoint {
            mountpoint.revalidate("mountpoint")?;
        }
        if mount_table_snapshot()? != self.mounts
            || current_boot_session_uuid()? != receipt.boot_session_uuid
        {
            return Err(invalid(
                "mount table or boot changed after restart receipt persistence",
            ));
        }
        validate_receipt(receipt)
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
        || request.baseline.boot_session_uuid != request.bindings.boot_session_uuid
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
                || path_is_at_or_below(&policy.backing_path, root)
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
        || !valid_nonce(&receipt.operation_nonce)
        || !valid_nonce(&receipt.restart_epoch_nonce)
        || receipt.monotonic_before_nanoseconds == 0
        || receipt.monotonic_after_nanoseconds < receipt.monotonic_before_nanoseconds
        || receipt.boot_session_uuid != receipt.iomedia_inventory.boot_session_uuid
        || receipt.baseline_inventory.boot_session_uuid != receipt.boot_session_uuid
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
    if current_baseline.sha256()? != receipt.post_inventory_sha256
        || receipt.baseline_restored != (current_baseline == receipt.baseline_inventory)
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
