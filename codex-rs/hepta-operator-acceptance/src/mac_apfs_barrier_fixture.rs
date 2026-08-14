//! Disposable APFS unmount-barrier qualification harness.
//!
//! This harness never grants migration, cutover, aggregate, production, ref,
//! remote, or deletion authority.  Its only positive result is a privileged
//! *mechanism* receipt for a disposable, dedicated APFS image.  The real state
//! snapshot/release/bridge lifecycle must consume that mechanism in a separate
//! end-to-end fixture before any Mac migration gate can pass.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::CString;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::darwin::fs::MetadataExt as DarwinMetadataExt;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::RawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use serde::Deserialize;
use serde::Serialize;

use crate::AcceptanceError;
use crate::durable::MAX_ARTIFACT_BYTES;
use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::mac_privileged_broker::AuthenticatedPeerV1;
use crate::mac_privileged_broker::NamespacePolicy;
use crate::mac_privileged_broker::ObjectBindingV1;
use crate::mac_privileged_broker::SealedPublicationV1;
use crate::mac_privileged_broker::prepared_tree_replay_sha256;
use crate::mac_privileged_broker::qualify_prepared_directory;
use crate::mac_privileged_broker::verify_sealed_publication;

const SCHEMA: &str = "hepta_mac_apfs_unmount_barrier_fixture_v1";
const RESULT_NAME: &str = "RESULT.json";
const SHA256SUMS_NAME: &str = "SHA256SUMS";
const MODES_NAME: &str = "MODES.tsv";
const IMAGE_NAME: &str = "barrier.dmg";
const MOUNTPOINT_NAME: &str = "mountpoint";
const LOGS_NAME: &str = "logs";
const TOOLS_NAME: &str = "executed-tools";
const OBLIGATION_PREFIX: &str = "hepta-apfs-obligation-v1-";
const HDIUTIL: &str = "/usr/bin/hdiutil";
const DISKUTIL: &str = "/usr/sbin/diskutil";
const LAUNCHCTL: &str = "/bin/launchctl";
const PLUTIL: &str = "/usr/bin/plutil";
const LIVE_ROOT_PREFIX: &str = ".hepta-privileged-qualification-v1-";
const IMAGE_BYTES: &str = "512m";
const COMMAND_TIMEOUT: Duration = Duration::from_secs(45);
const HOLDER_READY_TIMEOUT: Duration = Duration::from_secs(10);
const EXPECTED_T5_UUID: &str = "fb804d1b-24cb-4d6e-aea7-a9e180807758";
const LIVE_PRODUCER_UID: u32 = 499;
const LIVE_PRODUCER_GID: u32 = 499;
const MNT_RDONLY: u64 = 0x0000_0001;
const MNT_NOEXEC: u64 = 0x0000_0004;
const MNT_NOSUID: u64 = 0x0000_0008;
const MNT_NODEV: u64 = 0x0000_0010;
const MNT_IGNORE_OWNERSHIP: u64 = 0x0020_0000;
const MNT_NOATIME: u64 = 0x1000_0000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApfsFixturePlanV1 {
    pub aggregate_authority: bool,
    pub cutover_authority: bool,
    pub deletion_authority: bool,
    pub execution: bool,
    pub production_authority: bool,
    pub refs_authority: bool,
    pub remote_authority: bool,
    pub required_flow: Vec<String>,
    pub schema: String,
    pub scope: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileIdentityV1 {
    pub binding: ObjectBindingV1,
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PathIdentityV1 {
    pub binding: ObjectBindingV1,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommandReceiptV1 {
    pub arguments: Vec<String>,
    pub child_pid: i32,
    pub duration_milliseconds: u64,
    pub exit_code: i32,
    pub label: String,
    pub stderr_path: String,
    pub stderr_sha256: String,
    pub stderr_size: u64,
    pub stdout_path: String,
    pub stdout_sha256: String,
    pub stdout_size: u64,
    pub tool_after: FileIdentityV1,
    pub tool_ancestor_chain_after: Vec<PathIdentityV1>,
    pub tool_ancestor_chain_before: Vec<PathIdentityV1>,
    pub tool_before: FileIdentityV1,
    pub tool_copy_path: String,
    pub tool_process_group_id: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatFsFactsV1 {
    pub filesystem_id: [i32; 2],
    pub filesystem_type: String,
    pub mount_flags: u64,
    pub mount_from: String,
    pub mount_on: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DescriptorObservationV1 {
    pub absolute_path: String,
    pub fd_binding: ObjectBindingV1,
    pub parent_binding: ObjectBindingV1,
    pub path_binding_after: ObjectBindingV1,
    pub path_binding_before: ObjectBindingV1,
    pub schema: String,
    pub statfs: StatFsFactsV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HolderKindV1 {
    ReadWriteFd,
    SharedWritableMappingOnly,
    CurrentWorkingDirectoryOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessIdentityV1 {
    pub effective_gid: u32,
    pub effective_uid: u32,
    pub parent_pid: i32,
    pub pid: i32,
    pub process_group_id: i32,
    pub real_gid: u32,
    pub real_uid: u32,
    pub start_microseconds: u64,
    pub start_seconds: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawUnmountReceiptV1 {
    pub duration_microseconds: u64,
    pub errno: i32,
    pub flags: i32,
    pub mountpoint: String,
    pub rc: i32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MountPhaseV1 {
    ReadWrite,
    ReadOnly,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageDigestPhaseV1 {
    BeforeReadOnlyAttach,
    AfterReadOnlyDetach,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePhaseV1 {
    After,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VolumeStatePhaseV1 {
    BeforeReadOnlyNegatives,
    AfterReadOnlyNegatives,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnmountPhaseV1 {
    Baseline,
    Holder,
    ReadOnlyFinal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HolderCycleReceiptV1 {
    pub clean_unmount_after_release: RawUnmountReceiptV1,
    pub holder: ProcessIdentityV1,
    pub holder_after_busy: ProcessIdentityV1,
    pub holder_kind: HolderKindV1,
    pub holder_release_wait_status: i32,
    pub mount_still_same_after_busy: bool,
    pub mount_statfs_after_busy: StatFsFactsV1,
    pub mount_statfs_before: StatFsFactsV1,
    pub unmount_with_holder: RawUnmountReceiptV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountFactsV1 {
    pub apfs_container_uuid: String,
    pub device_identifier: String,
    pub filesystem_id: [i32; 2],
    pub filesystem_type: String,
    pub global_permissions_enabled: bool,
    pub media_writable: bool,
    pub mount_flags: u64,
    pub mount_from: String,
    pub mount_on: String,
    pub owner_sentinel_gid: u32,
    pub owner_sentinel_uid: u32,
    pub physical_store_identifier: String,
    pub volume_uuid: String,
    pub volume_writable: bool,
    pub whole_disk_identifier: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ErrnoNegativeV1 {
    pub credentials_after: KernelCredentialsV1,
    pub credentials_before: KernelCredentialsV1,
    pub child_wait_status: Option<i32>,
    pub observed_errno: i32,
    pub operation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KernelCredentialsV1 {
    pub effective_gid: u32,
    pub effective_uid: u32,
    pub pid: i32,
    pub real_gid: u32,
    pub real_uid: u32,
    pub supplementary_groups: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationEpochV1 {
    pub boot_session_uuid: String,
    pub challenge_sha256: String,
    pub helper_executable: FileIdentityV1,
    pub monotonic_nanoseconds: u64,
    pub mount_parent_before: ObjectBindingV1,
    pub mountpoint_underlying_before: ObjectBindingV1,
    pub operation_nonce: String,
    pub schema: String,
    pub source_before: DescriptorObservationV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EpochChallengeMaterialV1 {
    pub boot_session_uuid: String,
    pub helper_executable_sha256: String,
    pub monotonic_nanoseconds: u64,
    pub mountpoint_underlying_before: ObjectBindingV1,
    pub operation_nonce: String,
    pub schema: String,
    pub source_binding: ObjectBindingV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiskInventoryV1 {
    pub all_disks: Vec<String>,
    pub all_whole_disks: Vec<String>,
    pub command_receipt_sha256: String,
    pub hdiutil_backing_paths: Vec<String>,
    pub hdiutil_info_command_sha256: String,
    pub schema: String,
    pub t5_apfs_container_reference: String,
    pub t5_device_identifier: String,
    pub t5_parent_whole_disk: String,
    pub t5_physical_store_identifier: String,
    pub t5_volume_uuid: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiskNodeV1 {
    pub device_identifier: String,
    pub device_node: String,
    pub disk_image: bool,
    pub parent_whole_disk: String,
    pub size: u64,
    pub virtual_or_physical: String,
    pub whole: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttachedTopologyV1 {
    pub apfs_container: DiskNodeV1,
    pub apfs_container_uuid: String,
    pub apfs_volume: DiskNodeV1,
    pub apfs_volume_uuid: String,
    pub hdiutil_info_command_sha256: String,
    pub image_backing_after: FileIdentityV1,
    pub image_backing_before: FileIdentityV1,
    pub image_path_from_hdiutil: String,
    pub physical_store: DiskNodeV1,
    pub pre_attach_inventory_sha256: String,
    pub schema: String,
    pub whole_disk: DiskNodeV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiskArbitrationTerminalV1 {
    pub devnode_lstat_errno: i32,
    pub diskutil_info_command_sha256: String,
    pub diskutil_info_exit_code: i32,
    pub hdiutil_info_command_sha256: String,
    pub mountpoint_underlying_after: ObjectBindingV1,
    pub nested_mounts_after: Vec<StatFsFactsV1>,
    pub post_inventory: DiskInventoryV1,
    pub schema: String,
    pub whole_disk_identifier: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationDispositionV1 {
    Active,
    ReconcileRequired,
    Quarantined,
    Reconciled,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum AttachmentObligationEventV1 {
    Prepared {
        image_backing: FileIdentityV1,
        mountpoint_underlying: ObjectBindingV1,
        nested_mounts_before: Vec<StatFsFactsV1>,
        namespace_statfs: StatFsFactsV1,
        pre_attach_inventory: DiskInventoryV1,
    },
    AttachStarted {
        phase: MountPhaseV1,
    },
    Attached {
        phase: MountPhaseV1,
        topology: AttachedTopologyV1,
    },
    MountStarted {
        phase: MountPhaseV1,
        volume_identifier: String,
    },
    Mounted {
        mountpoint_statfs: StatFsFactsV1,
        phase: MountPhaseV1,
    },
    UnmountStarted {
        phase: MountPhaseV1,
    },
    Unmounted {
        phase: MountPhaseV1,
        receipt: RawUnmountReceiptV1,
    },
    DetachStarted {
        phase: MountPhaseV1,
        whole_disk_identifier: String,
    },
    DiskArbitrationGone {
        phase: MountPhaseV1,
        terminal: DiskArbitrationTerminalV1,
    },
    ReconcileRequired {
        reason_sha256: String,
    },
    Quarantined {
        cross_boot: bool,
        reason_sha256: String,
    },
    TerminalReconciled {
        post_inventory: DiskInventoryV1,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttachmentObligationRecordV1 {
    pub authority_granted: bool,
    pub boot_session_uuid: String,
    pub challenge_sha256: String,
    pub disposition: ObligationDispositionV1,
    pub epoch_receipt_sha256: String,
    pub event: AttachmentObligationEventV1,
    pub operation_nonce: String,
    pub previous_record_sha256: Option<String>,
    pub schema: String,
    pub sequence: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttachmentObligationVerificationV1 {
    pub authority_granted: bool,
    pub boot_session_uuid: String,
    pub current_boot: bool,
    pub disposition: ObligationDispositionV1,
    pub operation_nonce: String,
    pub records: usize,
    pub requires_privileged_reconciliation: bool,
    pub schema: String,
    pub terminal_record_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum NativeFactV1 {
    AttachedTopology {
        phase: MountPhaseV1,
        topology: AttachedTopologyV1,
    },
    Command(CommandReceiptV1),
    HolderCycle(HolderCycleReceiptV1),
    ImageDigest {
        phase: ImageDigestPhaseV1,
        sha256: String,
    },
    Mount {
        facts: MountFactsV1,
        phase: MountPhaseV1,
    },
    MutationNegative(ErrnoNegativeV1),
    RawUnmount {
        mountpoint_statfs_before: StatFsFactsV1,
        phase: UnmountPhaseV1,
        receipt: RawUnmountReceiptV1,
    },
    Source {
        observation: DescriptorObservationV1,
        phase: SourcePhaseV1,
    },
    Terminal {
        final_detached: bool,
        mount_parent_after: ObjectBindingV1,
        mountpoint_underlying_after: ObjectBindingV1,
    },
    DiskArbitrationTerminal {
        phase: MountPhaseV1,
        terminal: DiskArbitrationTerminalV1,
    },
    VolumeStateDigest {
        phase: VolumeStatePhaseV1,
        sha256: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeFactReceiptV1 {
    pub boot_session_uuid: String,
    pub challenge_sha256: String,
    pub epoch_receipt_sha256: String,
    pub fact: NativeFactV1,
    pub label: String,
    pub operation_nonce: String,
    pub schema: String,
    pub sequence: u32,
    pub source_binding: ObjectBindingV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawReceiptReferenceV1 {
    pub label: String,
    pub path: String,
    pub sequence: u32,
    pub sha256: String,
    pub size: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApfsFixtureResultV1 {
    pub aggregate_authority: bool,
    pub attachment_obligation_directory: String,
    pub attachment_obligation_terminal_sha256: String,
    pub cutover_authority: bool,
    pub deletion_authority: bool,
    pub epoch_receipt_path: String,
    pub epoch_receipt_sha256: String,
    pub execution_kind: String,
    pub operation_nonce: String,
    pub production_authority: bool,
    pub raw_receipts: Vec<RawReceiptReferenceV1>,
    pub refs_authority: bool,
    pub remote_authority: bool,
    pub schema: String,
    pub scope: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApfsFixtureVerificationV1 {
    pub authority_granted: bool,
    pub boot_session_uuid: String,
    pub command_receipts: usize,
    pub epoch_receipt_sha256: String,
    pub manifest_entries: usize,
    pub modes_entries: usize,
    pub operation_nonce: String,
    pub raw_receipts: usize,
    pub result_sha256: String,
    pub schema: String,
    pub tree_replay_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct HdiutilPlist {
    #[serde(rename = "system-entities")]
    system_entities: Vec<HdiutilEntity>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct HdiutilEntity {
    #[serde(rename = "content-hint")]
    content_hint: Option<String>,
    #[serde(rename = "dev-entry")]
    dev_entry: String,
    #[serde(rename = "volume-kind")]
    volume_kind: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DiskInfoPlist {
    #[serde(rename = "DeviceIdentifier")]
    device_identifier: String,
    #[serde(rename = "FilesystemType")]
    filesystem_type: String,
    #[serde(rename = "GlobalPermissionsEnabled")]
    global_permissions_enabled: bool,
    #[serde(rename = "VolumeUUID")]
    volume_uuid: String,
    #[serde(rename = "WritableMedia")]
    writable_media: bool,
    #[serde(rename = "WritableVolume")]
    writable_volume: bool,
}

#[derive(Debug, Deserialize)]
struct DiskInfoPhysicalStorePlist {
    #[serde(rename = "DeviceIdentifier")]
    device_identifier: String,
}

#[derive(Debug, Deserialize)]
struct DiskNodeInfoPlist {
    #[serde(rename = "APFSContainerReference", default)]
    apfs_container_reference: Option<String>,
    #[serde(rename = "APFSPhysicalStores", default)]
    apfs_physical_stores: Vec<DiskInfoPhysicalStorePlist>,
    #[serde(rename = "DeviceIdentifier")]
    device_identifier: String,
    #[serde(rename = "DeviceNode", default)]
    device_node: Option<String>,
    #[serde(rename = "DiskImage", default)]
    disk_image: Option<bool>,
    #[serde(rename = "ParentWholeDisk", default)]
    parent_whole_disk: Option<String>,
    #[serde(rename = "TotalSize", default)]
    total_size: Option<u64>,
    #[serde(rename = "VirtualOrPhysical", default)]
    virtual_or_physical: Option<String>,
    #[serde(rename = "VolumeUUID", default)]
    volume_uuid: Option<String>,
    #[serde(rename = "Whole", default)]
    whole: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct DiskListPlist {
    #[serde(rename = "AllDisks")]
    all_disks: Vec<String>,
    #[serde(rename = "WholeDisks")]
    whole_disks: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct HdiutilInfoPlist {
    images: Vec<HdiutilImageInfoPlist>,
}

#[derive(Debug, Deserialize)]
struct HdiutilImageInfoPlist {
    #[serde(rename = "image-path")]
    image_path: String,
    #[serde(rename = "system-entities")]
    system_entities: Vec<HdiutilEntity>,
}

#[derive(Debug, Deserialize)]
struct ApfsListPlist {
    #[serde(rename = "Containers")]
    containers: Vec<ApfsContainerPlist>,
}

#[derive(Debug, Deserialize)]
struct ApfsContainerPlist {
    #[serde(rename = "APFSContainerUUID")]
    apfs_container_uuid: String,
    #[serde(rename = "ContainerReference")]
    container_reference: String,
    #[serde(rename = "DesignatedPhysicalStore")]
    designated_physical_store: String,
    #[serde(rename = "Volumes")]
    volumes: Vec<ApfsVolumePlist>,
}

#[derive(Debug, Deserialize)]
struct ApfsVolumePlist {
    #[serde(rename = "APFSVolumeUUID")]
    apfs_volume_uuid: String,
    #[serde(rename = "DeviceIdentifier")]
    device_identifier: String,
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Clone, Debug)]
struct AttachedImage {
    apfs_container_uuid: String,
    container_identifier: String,
    physical_store_identifier: String,
    topology: Option<AttachedTopologyV1>,
    volume_identifier: String,
    volume_name: String,
    volume_uuid: String,
    whole_disk_identifier: String,
}

pub fn plan() -> ApfsFixturePlanV1 {
    ApfsFixturePlanV1 {
        aggregate_authority: false,
        cutover_authority: false,
        deletion_authority: false,
        execution: false,
        production_authority: false,
        refs_authority: false,
        remote_authority: false,
        required_flow: vec![
            "root_owned_underlying_mountpoint".to_string(),
            "owners_enabled_rw_mount".to_string(),
            "raw_clean_unmount_baseline".to_string(),
            "rw_fd_mmap_cwd_each_ebusy_then_clean_unmount".to_string(),
            "unmounted_gap_producer_eacces".to_string(),
            "same_uuid_read_only_media_remount".to_string(),
            "producer_and_root_erofs_mutation_matrix".to_string(),
            "final_clean_unmount_nonforced_detach".to_string(),
            "typed_closed_world_no_replace_seal".to_string(),
        ],
        schema: "hepta_mac_apfs_unmount_barrier_plan_v1".to_string(),
        scope: "disposable_privileged_mechanism_only_no_migration_authority".to_string(),
    }
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

fn require_digest(value: &str, label: &str) -> Result<(), AcceptanceError> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(invalid(format!("{label} is not a SHA-256 digest")));
    }
    Ok(())
}

fn require_nonce(value: &str) -> Result<(), AcceptanceError> {
    require_digest(value, "APFS fixture nonce")
}

fn require_uuid(value: &str, label: &str) -> Result<(), AcceptanceError> {
    let bytes = value.as_bytes();
    if bytes.len() != 36
        || bytes.iter().enumerate().any(|(index, byte)| match index {
            8 | 13 | 18 | 23 => *byte != b'-',
            _ => !byte.is_ascii_hexdigit(),
        })
    {
        return Err(invalid(format!("{label} is not a UUID")));
    }
    Ok(())
}

fn read_bounded(path: &Path, limit: u64) -> Result<Vec<u8>, AcceptanceError> {
    let path_before = fs::symlink_metadata(path)?;
    if !path_before.file_type().is_file() || path_before.nlink() != 1 || path_before.len() > limit {
        return Err(invalid(format!(
            "bounded file {} has invalid type, link count, or size",
            path.display()
        )));
    }
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)?;
    let fd_before = file.metadata()?;
    if binding_from_metadata(&path_before) != binding_from_metadata(&fd_before) {
        return Err(invalid(format!(
            "bounded file {} changed before descriptor replay",
            path.display()
        )));
    }
    let mut bytes = Vec::with_capacity(path_before.len() as usize);
    file.read_to_end(&mut bytes)?;
    let fd_after = file.metadata()?;
    let path_after = fs::symlink_metadata(path)?;
    if bytes.len() as u64 != path_before.len()
        || binding_from_metadata(&path_before) != binding_from_metadata(&fd_after)
        || binding_from_metadata(&path_before) != binding_from_metadata(&path_after)
    {
        return Err(invalid(format!(
            "bounded file {} changed while reading",
            path.display()
        )));
    }
    Ok(bytes)
}

fn binding_from_metadata(metadata: &fs::Metadata) -> ObjectBindingV1 {
    ObjectBindingV1 {
        ctime_nanoseconds: metadata.st_ctime_nsec(),
        ctime_seconds: metadata.st_ctime(),
        dev: metadata.dev(),
        flags: metadata.st_flags(),
        gid: metadata.gid(),
        inode: metadata.ino(),
        mode: metadata.mode() & 0o7777,
        mtime_nanoseconds: metadata.st_mtime_nsec(),
        mtime_seconds: metadata.st_mtime(),
        nlink: metadata.nlink(),
        size: metadata.size(),
        uid: metadata.uid(),
    }
}

fn binding(path: &Path) -> Result<ObjectBindingV1, AcceptanceError> {
    let metadata = fs::symlink_metadata(path)?;
    let file_type = metadata.file_type();
    if !file_type.is_file() && !file_type.is_dir() {
        return Err(invalid(format!(
            "fixture path {} is a symlink or special node",
            path.display()
        )));
    }
    Ok(binding_from_metadata(&metadata))
}

fn file_identity(path: &Path) -> Result<FileIdentityV1, AcceptanceError> {
    let binding_before = binding(path)?;
    if binding_before.nlink != 1 || binding_before.size > 128 * 1024 * 1024 {
        return Err(invalid(format!(
            "executable {} has invalid size or link count",
            path.display()
        )));
    }
    let bytes = read_bounded(path, 128 * 1024 * 1024)?;
    let after = binding(path)?;
    if binding_before != after {
        return Err(invalid(format!(
            "executable {} changed while hashing",
            path.display()
        )));
    }
    Ok(FileIdentityV1 {
        binding: binding_before,
        path: path
            .to_str()
            .ok_or_else(|| invalid("executable path is not UTF-8"))?
            .to_string(),
        sha256: sha256(&bytes),
    })
}

fn fixed_system_tool(path: &Path) -> bool {
    [HDIUTIL, DISKUTIL, LAUNCHCTL, PLUTIL]
        .into_iter()
        .any(|expected| path == Path::new(expected))
}

fn validate_system_tool_binding(
    binding: &ObjectBindingV1,
    kind: &str,
) -> Result<(), AcceptanceError> {
    if binding.uid != 0
        || binding.gid != 0
        || binding.mode != 0o755
        || binding.mode & 0o7000 != 0
        || (kind == "regular" && binding.nlink != 1)
    {
        return Err(invalid(format!(
            "system tool {kind} is not exact root:wheel mode 0755 without special bits"
        )));
    }
    Ok(())
}

fn system_tool_ancestor_chain(path: &Path) -> Result<Vec<PathIdentityV1>, AcceptanceError> {
    if !path.is_absolute() || !fixed_system_tool(path) {
        return Err(invalid(
            "command tool path is outside the fixed system allowlist",
        ));
    }
    let mut ancestors = Vec::new();
    let mut current = path
        .parent()
        .ok_or_else(|| invalid("system tool has no parent"))?;
    loop {
        ancestors.push(current.to_path_buf());
        if current == Path::new("/") {
            break;
        }
        current = current
            .parent()
            .ok_or_else(|| invalid("system tool ancestor chain did not reach root"))?;
    }
    ancestors.reverse();
    let mut identities = Vec::new();
    for ancestor in ancestors {
        let metadata = fs::symlink_metadata(&ancestor)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(invalid(format!(
                "system tool ancestor {} is not a real directory",
                ancestor.display()
            )));
        }
        let observed = binding_from_metadata(&metadata);
        validate_system_tool_binding(&observed, "ancestor")?;
        identities.push(PathIdentityV1 {
            binding: observed,
            path: ancestor
                .to_str()
                .ok_or_else(|| invalid("system tool ancestor path is not UTF-8"))?
                .to_string(),
        });
    }
    Ok(identities)
}

fn validate_system_tool_identity(identity: &FileIdentityV1) -> Result<(), AcceptanceError> {
    if !fixed_system_tool(Path::new(&identity.path)) {
        return Err(invalid(
            "system tool identity path is outside the fixed allowlist",
        ));
    }
    require_digest(&identity.sha256, "system tool byte pin")?;
    validate_system_tool_binding(&identity.binding, "regular")
}

fn validate_root_owned_executable(
    identity: &FileIdentityV1,
    expected_sha256: &str,
) -> Result<(), AcceptanceError> {
    require_digest(expected_sha256, "installed executable pin")?;
    if identity.sha256 != expected_sha256
        || identity.binding.uid != 0
        || identity.binding.gid != 0
        || identity.binding.mode != 0o555
        || identity.binding.nlink != 1
    {
        return Err(invalid(format!(
            "installed executable {} differs from root:wheel 0555 byte pin",
            identity.path
        )));
    }
    Ok(())
}

fn write_new(path: &Path, bytes: &[u8], mode: u32) -> Result<(), AcceptanceError> {
    if bytes.len() as u64 > MAX_ARTIFACT_BYTES {
        return Err(invalid("fixture artifact exceeds the fixed size bound"));
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

fn capture_tool(staging: &Path, identity: &FileIdentityV1) -> Result<String, AcceptanceError> {
    require_digest(&identity.sha256, "executed tool")?;
    let relative = format!("{TOOLS_NAME}/{}", identity.sha256);
    let destination = staging.join(&relative);
    let bytes = read_bounded(Path::new(&identity.path), 128 * 1024 * 1024)?;
    if sha256(&bytes) != identity.sha256 {
        return Err(invalid(
            "executed tool bytes differ from their descriptor identity",
        ));
    }
    match fs::symlink_metadata(&destination) {
        Ok(metadata) => {
            if !metadata.is_file()
                || metadata.nlink() != 1
                || read_bounded(&destination, 128 * 1024 * 1024)? != bytes
            {
                return Err(invalid(
                    "existing executed-tool copy differs from its byte pin",
                ));
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            write_new(&destination, &bytes, 0o400)?;
            File::open(staging.join(TOOLS_NAME))?.sync_all()?;
        }
        Err(error) => return Err(error.into()),
    }
    Ok(relative)
}

fn run_command(
    staging: &Path,
    sequence: usize,
    label: &str,
    tool_path: &Path,
    arguments: &[OsString],
) -> Result<CommandReceiptV1, AcceptanceError> {
    let tool_before = file_identity(tool_path)?;
    validate_system_tool_identity(&tool_before)?;
    let tool_ancestor_chain_before = system_tool_ancestor_chain(tool_path)?;
    let tool_copy_path = capture_tool(staging, &tool_before)?;
    let logs = staging.join(LOGS_NAME);
    let stdout_relative = format!("{LOGS_NAME}/{sequence:03}-{label}.stdout");
    let stderr_relative = format!("{LOGS_NAME}/{sequence:03}-{label}.stderr");
    let stdout_path = staging.join(&stdout_relative);
    let stderr_path = staging.join(&stderr_relative);
    let stdout_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&stdout_path)?;
    let stderr_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&stderr_path)?;
    if binding(&logs)?.mode != 0o700 {
        return Err(invalid("fixture log directory mode changed"));
    }
    let started = Instant::now();
    let mut command = Command::new(tool_path);
    command
        .args(arguments)
        .env_clear()
        .env("LC_ALL", "C")
        .env("LANG", "C")
        .env("PATH", "/usr/bin:/usr/sbin:/bin:/sbin")
        .env("HOME", "/var/empty")
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file));
    command.process_group(0);
    let mut child = command.spawn()?;
    let child_pid = i32::try_from(child.id()).map_err(|_| invalid("command PID overflowed"))?;
    let tool_process_group_id = unsafe { libc::getpgid(child_pid) };
    if child_pid <= 1 || tool_process_group_id != child_pid {
        let _ = terminate_process_group(&mut child, child_pid);
        return Err(invalid(
            "command did not start in its own kernel process group",
        ));
    }
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() > COMMAND_TIMEOUT {
            terminate_process_group(&mut child, tool_process_group_id)?;
            return Err(invalid(format!("command {label} exceeded its deadline")));
        }
        thread::sleep(Duration::from_millis(10));
    };
    sync_file(&stdout_path)?;
    sync_file(&stderr_path)?;
    let stdout = read_bounded(&stdout_path, MAX_ARTIFACT_BYTES)?;
    let stderr = read_bounded(&stderr_path, MAX_ARTIFACT_BYTES)?;
    let tool_after = file_identity(tool_path)?;
    validate_system_tool_identity(&tool_after)?;
    let tool_ancestor_chain_after = system_tool_ancestor_chain(tool_path)?;
    if tool_before != tool_after || tool_ancestor_chain_before != tool_ancestor_chain_after {
        return Err(invalid(format!(
            "command tool or root-owned ancestor chain {label} changed during execution"
        )));
    }
    let exit_code = exit_code(status)?;
    Ok(CommandReceiptV1 {
        arguments: arguments
            .iter()
            .map(|argument| {
                argument
                    .to_str()
                    .ok_or_else(|| invalid("command argument is not UTF-8"))
                    .map(str::to_string)
            })
            .collect::<Result<_, _>>()?,
        child_pid,
        duration_milliseconds: started.elapsed().as_millis() as u64,
        exit_code,
        label: label.to_string(),
        stderr_path: stderr_relative,
        stderr_sha256: sha256(&stderr),
        stderr_size: stderr.len() as u64,
        stdout_path: stdout_relative,
        stdout_sha256: sha256(&stdout),
        stdout_size: stdout.len() as u64,
        tool_after,
        tool_ancestor_chain_after,
        tool_ancestor_chain_before,
        tool_before,
        tool_copy_path,
        tool_process_group_id,
    })
}

fn terminate_process_group(
    child: &mut std::process::Child,
    pgid: i32,
) -> Result<(), AcceptanceError> {
    if pgid <= 1 {
        return Err(invalid("refusing to terminate an invalid process group"));
    }
    let term_rc = unsafe { libc::kill(-pgid, libc::SIGTERM) };
    if term_rc != 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH) {
        return Err(std::io::Error::last_os_error().into());
    }
    let grace_started = Instant::now();
    while grace_started.elapsed() < Duration::from_secs(2) {
        if child.try_wait()?.is_some() {
            let group_rc = unsafe { libc::kill(-pgid, 0) };
            if group_rc != 0 && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
            {
                return Ok(());
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    let kill_rc = unsafe { libc::kill(-pgid, libc::SIGKILL) };
    if kill_rc != 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH) {
        return Err(std::io::Error::last_os_error().into());
    }
    let _ = child.wait()?;
    let group_rc = unsafe { libc::kill(-pgid, 0) };
    if group_rc == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH) {
        return Err(invalid(
            "timed-out command process group remained live after SIGKILL",
        ));
    }
    Ok(())
}

fn sync_file(path: &Path) -> Result<(), AcceptanceError> {
    File::open(path)?.sync_all()?;
    Ok(())
}

fn exit_code(status: ExitStatus) -> Result<i32, AcceptanceError> {
    status
        .code()
        .ok_or_else(|| invalid("fixture command terminated by signal"))
}

fn command_stdout(staging: &Path, receipt: &CommandReceiptV1) -> Result<Vec<u8>, AcceptanceError> {
    let bytes = read_bounded(&staging.join(&receipt.stdout_path), MAX_ARTIFACT_BYTES)?;
    if bytes.len() as u64 != receipt.stdout_size || sha256(&bytes) != receipt.stdout_sha256 {
        return Err(invalid(format!(
            "command {} stdout differs from its receipt",
            receipt.label
        )));
    }
    Ok(bytes)
}

fn convert_plist<T: for<'de> Deserialize<'de>>(
    staging: &Path,
    sequence: &mut usize,
    label: &str,
    input: &CommandReceiptV1,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<T, AcceptanceError> {
    let input_path = staging.join(&input.stdout_path);
    let arguments = vec![
        OsString::from("-convert"),
        OsString::from("json"),
        OsString::from("-o"),
        OsString::from("-"),
        input_path.as_os_str().to_os_string(),
    ];
    let receipt = run_command(staging, *sequence, label, Path::new(PLUTIL), &arguments)?;
    *sequence += 1;
    if receipt.exit_code != 0 {
        return Err(invalid(format!("plutil conversion {label} failed")));
    }
    let bytes = command_stdout(staging, &receipt)?;
    let parsed = serde_json::from_slice(&bytes)
        .map_err(|error| invalid(format!("converted system plist is malformed: {error}")))?;
    commands.push(receipt);
    Ok(parsed)
}

fn command_receipt_sha256(receipt: &CommandReceiptV1) -> Result<String, AcceptanceError> {
    Ok(sha256(&canonical_json(receipt)?))
}

fn inventory_sha256(inventory: &DiskInventoryV1) -> Result<String, AcceptanceError> {
    validate_disk_inventory(inventory)?;
    Ok(sha256(&canonical_json(inventory)?))
}

fn collect_disk_inventory(
    staging: &Path,
    sequence: &mut usize,
    stage: &str,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<DiskInventoryV1, AcceptanceError> {
    require_label(stage)?;
    let list_label = format!("disk-list-{stage}");
    let list_receipt = run_command(
        staging,
        *sequence,
        &list_label,
        Path::new(DISKUTIL),
        &[OsString::from("list"), OsString::from("-plist")],
    )?;
    *sequence += 1;
    if list_receipt.exit_code != 0 {
        return Err(invalid(
            "diskutil list failed before topology qualification",
        ));
    }
    commands.push(list_receipt.clone());
    let list: DiskListPlist = convert_plist(
        staging,
        sequence,
        &format!("parse-{list_label}"),
        &list_receipt,
        commands,
    )?;

    let t5_label = format!("disk-node-info-t5-{stage}");
    let t5_receipt = run_command(
        staging,
        *sequence,
        &t5_label,
        Path::new(DISKUTIL),
        &[
            OsString::from("info"),
            OsString::from("-plist"),
            OsString::from("/Volumes/T5"),
        ],
    )?;
    *sequence += 1;
    if t5_receipt.exit_code != 0 {
        return Err(invalid(
            "diskutil T5 info failed before topology qualification",
        ));
    }
    commands.push(t5_receipt.clone());
    let t5: DiskNodeInfoPlist = convert_plist(
        staging,
        sequence,
        &format!("parse-{t5_label}"),
        &t5_receipt,
        commands,
    )?;
    let hdi_label = format!("hdi-info-inventory-{stage}");
    let hdi_receipt = run_command(
        staging,
        *sequence,
        &hdi_label,
        Path::new(HDIUTIL),
        &[OsString::from("info"), OsString::from("-plist")],
    )?;
    *sequence += 1;
    if hdi_receipt.exit_code != 0 {
        return Err(invalid("hdiutil inventory failed"));
    }
    commands.push(hdi_receipt.clone());
    let hdi: HdiutilInfoPlist = convert_plist(
        staging,
        sequence,
        &format!("parse-{hdi_label}"),
        &hdi_receipt,
        commands,
    )?;
    build_disk_inventory(list, t5, hdi, &list_receipt, &t5_receipt, &hdi_receipt)
}

fn build_disk_inventory(
    list: DiskListPlist,
    t5: DiskNodeInfoPlist,
    hdi: HdiutilInfoPlist,
    list_receipt: &CommandReceiptV1,
    t5_receipt: &CommandReceiptV1,
    hdi_receipt: &CommandReceiptV1,
) -> Result<DiskInventoryV1, AcceptanceError> {
    let mut all_disks = list.all_disks;
    let mut all_whole_disks = list.whole_disks;
    all_disks.sort();
    all_disks.dedup();
    all_whole_disks.sort();
    all_whole_disks.dedup();
    let mut hdiutil_backing_paths = hdi
        .images
        .into_iter()
        .map(|image| image.image_path)
        .collect::<Vec<_>>();
    hdiutil_backing_paths.sort();
    hdiutil_backing_paths.dedup();
    if t5.apfs_physical_stores.len() != 1 {
        return Err(invalid(
            "T5 diskutil info does not expose exactly one APFS physical store",
        ));
    }
    let inventory_command_receipt_sha256 = sha256(&canonical_json(&[
        command_receipt_sha256(&list_receipt)?,
        command_receipt_sha256(&t5_receipt)?,
        command_receipt_sha256(hdi_receipt)?,
    ])?);
    let inventory = DiskInventoryV1 {
        all_disks,
        all_whole_disks,
        command_receipt_sha256: inventory_command_receipt_sha256,
        hdiutil_backing_paths,
        hdiutil_info_command_sha256: command_receipt_sha256(hdi_receipt)?,
        schema: "hepta_mac_disk_inventory_v1".to_string(),
        t5_apfs_container_reference: t5
            .apfs_container_reference
            .ok_or_else(|| invalid("T5 info omitted APFS container reference"))?,
        t5_device_identifier: t5.device_identifier,
        t5_parent_whole_disk: t5
            .parent_whole_disk
            .ok_or_else(|| invalid("T5 info omitted parent whole disk"))?,
        t5_physical_store_identifier: t5.apfs_physical_stores[0].device_identifier.clone(),
        t5_volume_uuid: t5
            .volume_uuid
            .ok_or_else(|| invalid("T5 info omitted volume UUID"))?
            .to_ascii_lowercase(),
    };
    validate_disk_inventory(&inventory)?;
    Ok(inventory)
}

fn disk_node_from_info(
    info: DiskNodeInfoPlist,
    expect_whole: bool,
) -> Result<DiskNodeV1, AcceptanceError> {
    let node = DiskNodeV1 {
        device_identifier: info.device_identifier.clone(),
        device_node: info
            .device_node
            .ok_or_else(|| invalid("disk node info omitted DeviceNode"))?,
        disk_image: info
            .disk_image
            .ok_or_else(|| invalid("disk node info omitted DiskImage"))?,
        parent_whole_disk: info
            .parent_whole_disk
            .unwrap_or_else(|| info.device_identifier.clone()),
        size: info
            .total_size
            .ok_or_else(|| invalid("disk node info omitted TotalSize"))?,
        virtual_or_physical: info
            .virtual_or_physical
            .ok_or_else(|| invalid("disk node info omitted VirtualOrPhysical"))?,
        whole: info
            .whole
            .ok_or_else(|| invalid("disk node info omitted Whole"))?,
    };
    validate_disk_node(&node, expect_whole)?;
    Ok(node)
}

fn collect_disk_node(
    staging: &Path,
    sequence: &mut usize,
    label: &str,
    identifier: &str,
    expect_whole: bool,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<DiskNodeV1, AcceptanceError> {
    let receipt = run_command(
        staging,
        *sequence,
        label,
        Path::new(DISKUTIL),
        &[
            OsString::from("info"),
            OsString::from("-plist"),
            OsString::from(identifier),
        ],
    )?;
    *sequence += 1;
    if receipt.exit_code != 0 {
        return Err(invalid(format!("diskutil node info {label} failed")));
    }
    commands.push(receipt.clone());
    let info: DiskNodeInfoPlist = convert_plist(
        staging,
        sequence,
        &format!("parse-{label}"),
        &receipt,
        commands,
    )?;
    let node = disk_node_from_info(info, expect_whole)?;
    if node.device_identifier != identifier {
        return Err(invalid("diskutil node info changed the requested BSD name"));
    }
    Ok(node)
}

fn parse_attached_image(
    attach: &HdiutilPlist,
    apfs: &ApfsListPlist,
    expected_name: &str,
) -> Result<AttachedImage, AcceptanceError> {
    let whole_disks = attach
        .system_entities
        .iter()
        .filter(|entity| entity.content_hint.as_deref() == Some("GUID_partition_scheme"))
        .collect::<Vec<_>>();
    let physical_stores = attach
        .system_entities
        .iter()
        .filter(|entity| entity.content_hint.as_deref() == Some("Apple_APFS"))
        .collect::<Vec<_>>();
    let apfs_volumes = attach
        .system_entities
        .iter()
        .filter(|entity| entity.volume_kind.as_deref() == Some("apfs"))
        .collect::<Vec<_>>();
    if whole_disks.len() != 1 || physical_stores.len() != 1 || apfs_volumes.len() != 1 {
        return Err(invalid(
            "disk image attachment topology is not a single APFS volume",
        ));
    }
    if apfs.containers.len() != 1 {
        return Err(invalid("disk image has more than one APFS container"));
    }
    let container = &apfs.containers[0];
    if container.volumes.len() != 1 {
        return Err(invalid(
            "disk image APFS container has more than one volume",
        ));
    }
    let volume = &container.volumes[0];
    let whole = strip_device_prefix(&whole_disks[0].dev_entry);
    let physical = strip_device_prefix(&physical_stores[0].dev_entry);
    let attached_volume = strip_device_prefix(&apfs_volumes[0].dev_entry);
    if physical != container.designated_physical_store
        || attached_volume != volume.device_identifier
        || volume.name != expected_name
        || !container.container_reference.starts_with("disk")
        || !whole.starts_with("disk")
    {
        return Err(invalid(
            "APFS device, container, volume, or name lineage is inconsistent",
        ));
    }
    require_uuid(&container.apfs_container_uuid, "APFS container UUID")?;
    require_uuid(&volume.apfs_volume_uuid, "APFS volume UUID")?;
    Ok(AttachedImage {
        apfs_container_uuid: container.apfs_container_uuid.to_ascii_lowercase(),
        container_identifier: container.container_reference.clone(),
        physical_store_identifier: physical.to_string(),
        topology: None,
        volume_identifier: volume.device_identifier.clone(),
        volume_name: volume.name.clone(),
        volume_uuid: volume.apfs_volume_uuid.to_ascii_lowercase(),
        whole_disk_identifier: whole.to_string(),
    })
}

fn strip_device_prefix(value: &str) -> &str {
    value.strip_prefix("/dev/").unwrap_or(value)
}

fn validate_disk_inventory(inventory: &DiskInventoryV1) -> Result<(), AcceptanceError> {
    require_digest(
        &inventory.command_receipt_sha256,
        "disk inventory command receipt",
    )?;
    require_digest(
        &inventory.hdiutil_info_command_sha256,
        "disk inventory hdiutil receipt",
    )?;
    require_uuid(&inventory.t5_volume_uuid, "T5 APFS volume")?;
    if inventory.schema != "hepta_mac_disk_inventory_v1"
        || inventory.t5_volume_uuid != EXPECTED_T5_UUID
        || inventory.all_disks.is_empty()
        || inventory.all_whole_disks.is_empty()
        || !inventory.all_disks.windows(2).all(|pair| pair[0] < pair[1])
        || !inventory
            .all_whole_disks
            .windows(2)
            .all(|pair| pair[0] < pair[1])
        || inventory
            .all_whole_disks
            .iter()
            .any(|disk| !inventory.all_disks.contains(disk))
        || !inventory
            .hdiutil_backing_paths
            .windows(2)
            .all(|pair| pair[0] < pair[1])
        || inventory
            .hdiutil_backing_paths
            .iter()
            .any(|path| !Path::new(path).is_absolute())
        || [
            &inventory.t5_device_identifier,
            &inventory.t5_parent_whole_disk,
            &inventory.t5_apfs_container_reference,
            &inventory.t5_physical_store_identifier,
        ]
        .into_iter()
        .any(|disk| !valid_disk_identifier(disk) || !inventory.all_disks.contains(disk))
    {
        return Err(invalid(
            "disk inventory does not exactly bind the pre-existing T5 device topology",
        ));
    }
    Ok(())
}

fn validate_disk_node(node: &DiskNodeV1, expect_whole: bool) -> Result<(), AcceptanceError> {
    if !valid_disk_identifier(&node.device_identifier)
        || node.device_node != format!("/dev/{}", node.device_identifier)
        || node.whole != expect_whole
        || !node.disk_image
        || node.virtual_or_physical != "Virtual"
        || node.size == 0
        || (!expect_whole && !valid_disk_identifier(&node.parent_whole_disk))
    {
        return Err(invalid(
            "attached disk node is not an exact virtual disk-image topology fact",
        ));
    }
    Ok(())
}

fn validate_attached_topology_shape(topology: &AttachedTopologyV1) -> Result<(), AcceptanceError> {
    require_digest(
        &topology.hdiutil_info_command_sha256,
        "hdiutil topology command",
    )?;
    require_digest(
        &topology.pre_attach_inventory_sha256,
        "pre-attach disk inventory",
    )?;
    require_uuid(&topology.apfs_container_uuid, "attached APFS container")?;
    require_uuid(&topology.apfs_volume_uuid, "attached APFS volume")?;
    validate_disk_node(&topology.whole_disk, true)?;
    validate_disk_node(&topology.physical_store, false)?;
    validate_disk_node(&topology.apfs_container, true)?;
    validate_disk_node(&topology.apfs_volume, false)?;
    let identifiers = [
        &topology.whole_disk.device_identifier,
        &topology.physical_store.device_identifier,
        &topology.apfs_container.device_identifier,
        &topology.apfs_volume.device_identifier,
    ];
    if topology.schema != "hepta_mac_attached_apfs_topology_v1"
        || topology.image_backing_before != topology.image_backing_after
        || topology.image_backing_before.path != topology.image_path_from_hdiutil
        || identifiers
            .iter()
            .enumerate()
            .any(|(index, value)| identifiers[..index].contains(value))
        || topology.physical_store.parent_whole_disk != topology.whole_disk.device_identifier
        || topology.apfs_container.parent_whole_disk != topology.apfs_container.device_identifier
        || topology.apfs_volume.parent_whole_disk != topology.apfs_container.device_identifier
    {
        return Err(invalid(
            "image inode/backing, whole, slice, container, and volume chain is inconsistent",
        ));
    }
    Ok(())
}

fn c_path(path: &Path) -> Result<CString, AcceptanceError> {
    CString::new(path.as_os_str().as_bytes())
        .map_err(|_| invalid(format!("path {} contains NUL", path.display())))
}

fn fixed_c_string(bytes: &[libc::c_char]) -> Result<String, AcceptanceError> {
    let nul = bytes
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| invalid("statfs fixed string is not terminated"))?;
    let raw = bytes[..nul]
        .iter()
        .map(|byte| *byte as u8)
        .collect::<Vec<_>>();
    String::from_utf8(raw).map_err(|_| invalid("statfs fixed string is not UTF-8"))
}

fn statfs_from_raw(stat: &libc::statfs) -> Result<StatFsFactsV1, AcceptanceError> {
    Ok(StatFsFactsV1 {
        filesystem_id: unsafe { std::mem::transmute::<libc::fsid_t, [i32; 2]>(stat.f_fsid) },
        filesystem_type: fixed_c_string(&stat.f_fstypename)?,
        mount_flags: stat.f_flags as u64,
        mount_from: fixed_c_string(&stat.f_mntfromname)?,
        mount_on: fixed_c_string(&stat.f_mntonname)?,
    })
}

fn statfs_facts(path: &Path) -> Result<StatFsFactsV1, AcceptanceError> {
    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(c_path(path)?.as_ptr(), &mut stat) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    statfs_from_raw(&stat)
}

fn fstatfs_facts(fd: RawFd) -> Result<StatFsFactsV1, AcceptanceError> {
    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::fstatfs(fd, &mut stat) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    statfs_from_raw(&stat)
}

fn nested_mounts_below(root: &Path) -> Result<Vec<StatFsFactsV1>, AcceptanceError> {
    if !root.is_absolute() {
        return Err(invalid("nested-mount root is not absolute"));
    }
    let count = unsafe { libc::getfsstat(std::ptr::null_mut(), 0, libc::MNT_NOWAIT) };
    if count < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let capacity = usize::try_from(count)
        .map_err(|_| invalid("mount-table count overflowed"))?
        .saturating_add(16);
    let mut mounts = vec![unsafe { std::mem::zeroed::<libc::statfs>() }; capacity];
    let buffer_bytes = mounts
        .len()
        .checked_mul(std::mem::size_of::<libc::statfs>())
        .and_then(|size| i32::try_from(size).ok())
        .ok_or_else(|| invalid("mount-table buffer overflowed"))?;
    let observed = unsafe { libc::getfsstat(mounts.as_mut_ptr(), buffer_bytes, libc::MNT_NOWAIT) };
    if observed < 0 || observed as usize > mounts.len() {
        return Err(std::io::Error::last_os_error().into());
    }
    mounts.truncate(observed as usize);
    let root_text = root
        .to_str()
        .ok_or_else(|| invalid("nested-mount root is not UTF-8"))?;
    let prefix = format!("{root_text}/");
    let mut facts = mounts
        .iter()
        .map(statfs_from_raw)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|mount| mount.mount_on == root_text || mount.mount_on.starts_with(&prefix))
        .collect::<Vec<_>>();
    facts.sort_by(|left, right| left.mount_on.cmp(&right.mount_on));
    Ok(facts)
}

fn mounted_facts(
    staging: &Path,
    sequence: &mut usize,
    mountpoint: &Path,
    attached: &AttachedImage,
    expected_uid: u32,
    expected_gid: u32,
    label: &str,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<MountFactsV1, AcceptanceError> {
    let arguments = vec![
        OsString::from("info"),
        OsString::from("-plist"),
        mountpoint.as_os_str().to_os_string(),
    ];
    let receipt = run_command(staging, *sequence, label, Path::new(DISKUTIL), &arguments)?;
    *sequence += 1;
    if receipt.exit_code != 0 {
        return Err(invalid("diskutil info failed"));
    }
    commands.push(receipt.clone());
    let info: DiskInfoPlist = convert_plist(
        staging,
        sequence,
        &format!("parse-{label}"),
        &receipt,
        commands,
    )?;
    let statfs = statfs_facts(mountpoint)?;
    let sentinel = binding(&mountpoint.join("owner-sentinel"))?;
    if sentinel.uid != expected_uid || sentinel.gid != expected_gid {
        return Err(invalid("APFS ownership sentinel changed across mount"));
    }
    if info.device_identifier != attached.volume_identifier
        || info.volume_uuid.to_ascii_lowercase() != attached.volume_uuid
        || info.filesystem_type != "apfs"
        || statfs.filesystem_type != "apfs"
        || statfs.mount_on != mountpoint.to_string_lossy()
        || !statfs.mount_from.ends_with(&attached.volume_identifier)
    {
        return Err(invalid(
            "mounted APFS identity differs from attached image lineage",
        ));
    }
    Ok(MountFactsV1 {
        apfs_container_uuid: attached.apfs_container_uuid.clone(),
        device_identifier: info.device_identifier,
        filesystem_id: statfs.filesystem_id,
        filesystem_type: statfs.filesystem_type,
        global_permissions_enabled: info.global_permissions_enabled,
        media_writable: info.writable_media,
        mount_flags: statfs.mount_flags,
        mount_from: statfs.mount_from,
        mount_on: statfs.mount_on,
        owner_sentinel_gid: sentinel.gid,
        owner_sentinel_uid: sentinel.uid,
        physical_store_identifier: attached.physical_store_identifier.clone(),
        volume_uuid: info.volume_uuid.to_ascii_lowercase(),
        volume_writable: info.writable_volume,
        whole_disk_identifier: attached.whole_disk_identifier.clone(),
    })
}

fn raw_unmount(mountpoint: &Path) -> Result<RawUnmountReceiptV1, AcceptanceError> {
    if !mountpoint.is_absolute() {
        return Err(invalid("raw unmount target is not absolute"));
    }
    let mountpoint_text = mountpoint
        .to_str()
        .ok_or_else(|| invalid("raw unmount target is not UTF-8"))?
        .to_string();
    let path = c_path(mountpoint)?;
    const FLAGS: i32 = 0;
    let started = Instant::now();
    unsafe { *libc::__error() = 0 };
    let rc = unsafe { libc::unmount(path.as_ptr(), FLAGS) };
    let errno = if rc == 0 {
        0
    } else {
        unsafe { *libc::__error() }
    };
    Ok(RawUnmountReceiptV1 {
        duration_microseconds: started.elapsed().as_micros() as u64,
        errno,
        flags: FLAGS,
        mountpoint: mountpoint_text,
        rc,
    })
}

fn validate_mount_flags(facts: &MountFactsV1, read_only: bool) -> Result<(), AcceptanceError> {
    let required = MNT_NOEXEC | MNT_NOSUID | MNT_NODEV | MNT_NOATIME;
    if !facts.global_permissions_enabled
        || facts.mount_flags & MNT_IGNORE_OWNERSHIP != 0
        || facts.mount_flags & required != required
        || (facts.mount_flags & MNT_RDONLY != 0) != read_only
        || facts.volume_writable == read_only
        || (read_only && facts.media_writable)
    {
        return Err(invalid(
            "APFS mount flags, ownership semantics, or writable state are invalid",
        ));
    }
    Ok(())
}

fn statfs_matches_mount(statfs: &StatFsFactsV1, mount: &MountFactsV1) -> bool {
    statfs.filesystem_id == mount.filesystem_id
        && statfs.filesystem_type == mount.filesystem_type
        && statfs.mount_flags == mount.mount_flags
        && statfs.mount_from == mount.mount_from
        && statfs.mount_on == mount.mount_on
}

fn mount_volume(
    staging: &Path,
    sequence: &mut usize,
    mountpoint: &Path,
    volume_identifier: &str,
    read_only: bool,
    label: &str,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<(), AcceptanceError> {
    let mut arguments = vec![OsString::from("mount")];
    if read_only {
        arguments.push(OsString::from("readOnly"));
    }
    arguments.extend([
        OsString::from("nobrowse"),
        OsString::from("-mountOptions"),
        OsString::from("owners,nodev,nosuid,noexec,noatime"),
        OsString::from("-mountPoint"),
        mountpoint.as_os_str().to_os_string(),
        OsString::from(volume_identifier),
    ]);
    let receipt = run_command(staging, *sequence, label, Path::new(DISKUTIL), &arguments)?;
    *sequence += 1;
    if receipt.exit_code != 0 {
        return Err(invalid(format!("{label} failed")));
    }
    commands.push(receipt);
    Ok(())
}

fn attach_image(
    staging: &Path,
    sequence: &mut usize,
    image: &Path,
    read_only: bool,
    expected_volume_name: &str,
    pre_attach_inventory: &DiskInventoryV1,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<AttachedImage, AcceptanceError> {
    validate_disk_inventory(pre_attach_inventory)?;
    let image_backing_before = file_identity(image)?;
    let mut arguments = vec![OsString::from("attach")];
    if read_only {
        arguments.push(OsString::from("-readonly"));
    }
    arguments.extend([
        OsString::from("-owners"),
        OsString::from("on"),
        OsString::from("-nomount"),
        OsString::from("-plist"),
        image.as_os_str().to_os_string(),
    ]);
    let receipt = run_command(
        staging,
        *sequence,
        if read_only {
            "attach-read-only"
        } else {
            "attach-read-write"
        },
        Path::new(HDIUTIL),
        &arguments,
    )?;
    *sequence += 1;
    if receipt.exit_code != 0 {
        return Err(invalid("hdiutil attach failed"));
    }
    let parse_label = if read_only {
        "parse-attach-read-only"
    } else {
        "parse-attach-read-write"
    };
    commands.push(receipt.clone());
    let attach: HdiutilPlist = convert_plist(staging, sequence, parse_label, &receipt, commands)?;
    let volume_entity = attach
        .system_entities
        .iter()
        .find(|entity| entity.volume_kind.as_deref() == Some("apfs"))
        .ok_or_else(|| invalid("hdiutil attach omitted its APFS volume entity"))?;
    let container = volume_entity
        .dev_entry
        .strip_prefix("/dev/")
        .unwrap_or(&volume_entity.dev_entry)
        .trim_end_matches(|character: char| character.is_ascii_digit())
        .trim_end_matches('s')
        .to_string();
    if !container.starts_with("disk") {
        return Err(invalid("APFS container identifier is malformed"));
    }
    let apfs_arguments = vec![
        OsString::from("apfs"),
        OsString::from("list"),
        OsString::from("-plist"),
        OsString::from(&container),
    ];
    let apfs_receipt = run_command(
        staging,
        *sequence,
        if read_only {
            "apfs-list-read-only"
        } else {
            "apfs-list-read-write"
        },
        Path::new(DISKUTIL),
        &apfs_arguments,
    )?;
    *sequence += 1;
    if apfs_receipt.exit_code != 0 {
        return Err(invalid("diskutil apfs list failed"));
    }
    let parse_apfs_label = if read_only {
        "parse-apfs-read-only"
    } else {
        "parse-apfs-read-write"
    };
    commands.push(apfs_receipt.clone());
    let apfs: ApfsListPlist =
        convert_plist(staging, sequence, parse_apfs_label, &apfs_receipt, commands)?;
    let mut attached = parse_attached_image(&attach, &apfs, expected_volume_name)?;
    let suffix = if read_only { "read-only" } else { "read-write" };

    let hdi_info_label = format!("hdi-info-{suffix}");
    let hdi_info_receipt = run_command(
        staging,
        *sequence,
        &hdi_info_label,
        Path::new(HDIUTIL),
        &[OsString::from("info"), OsString::from("-plist")],
    )?;
    *sequence += 1;
    if hdi_info_receipt.exit_code != 0 {
        return Err(invalid("hdiutil info failed after image attachment"));
    }
    commands.push(hdi_info_receipt.clone());
    let hdi_info: HdiutilInfoPlist = convert_plist(
        staging,
        sequence,
        &format!("parse-{hdi_info_label}"),
        &hdi_info_receipt,
        commands,
    )?;
    let canonical_image = image.canonicalize()?;
    if canonical_image != image {
        return Err(invalid("attached image path is not canonical"));
    }
    let matching_images = hdi_info
        .images
        .iter()
        .filter(|candidate| Path::new(&candidate.image_path) == image)
        .collect::<Vec<_>>();
    if matching_images.len() != 1 || matching_images[0].system_entities != attach.system_entities {
        return Err(invalid(
            "hdiutil info does not bind the exact image backing to the attach topology",
        ));
    }

    let whole_disk = collect_disk_node(
        staging,
        sequence,
        &format!("disk-node-info-{suffix}-whole"),
        &attached.whole_disk_identifier,
        true,
        commands,
    )?;
    let physical_store = collect_disk_node(
        staging,
        sequence,
        &format!("disk-node-info-{suffix}-physical"),
        &attached.physical_store_identifier,
        false,
        commands,
    )?;
    let apfs_container = collect_disk_node(
        staging,
        sequence,
        &format!("disk-node-info-{suffix}-container"),
        &attached.container_identifier,
        true,
        commands,
    )?;
    let apfs_volume = collect_disk_node(
        staging,
        sequence,
        &format!("disk-node-info-{suffix}-volume"),
        &attached.volume_identifier,
        false,
        commands,
    )?;
    let image_backing_after = file_identity(image)?;
    let topology = AttachedTopologyV1 {
        apfs_container,
        apfs_container_uuid: attached.apfs_container_uuid.clone(),
        apfs_volume,
        apfs_volume_uuid: attached.volume_uuid.clone(),
        hdiutil_info_command_sha256: command_receipt_sha256(&hdi_info_receipt)?,
        image_backing_after,
        image_backing_before,
        image_path_from_hdiutil: matching_images[0].image_path.clone(),
        physical_store,
        pre_attach_inventory_sha256: inventory_sha256(pre_attach_inventory)?,
        schema: "hepta_mac_attached_apfs_topology_v1".to_string(),
        whole_disk,
    };
    validate_attached_topology_shape(&topology)?;
    let forbidden = [
        &pre_attach_inventory.t5_device_identifier,
        &pre_attach_inventory.t5_parent_whole_disk,
        &pre_attach_inventory.t5_apfs_container_reference,
        &pre_attach_inventory.t5_physical_store_identifier,
    ];
    for identifier in [
        &topology.whole_disk.device_identifier,
        &topology.physical_store.device_identifier,
        &topology.apfs_container.device_identifier,
        &topology.apfs_volume.device_identifier,
    ] {
        if pre_attach_inventory.all_disks.contains(identifier) || forbidden.contains(&identifier) {
            return Err(invalid(
                "attached topology intersects T5 or a pre-existing disk",
            ));
        }
    }
    if topology.apfs_volume_uuid == pre_attach_inventory.t5_volume_uuid {
        return Err(invalid("disposable APFS volume UUID aliases T5"));
    }
    if pre_attach_inventory
        .hdiutil_backing_paths
        .contains(&topology.image_path_from_hdiutil)
    {
        return Err(invalid(
            "disposable image backing was already attached before the obligation",
        ));
    }
    attached.topology = Some(topology);
    Ok(attached)
}

fn detach_image(
    staging: &Path,
    sequence: &mut usize,
    attached: &AttachedImage,
    label: &str,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<(), AcceptanceError> {
    let arguments = vec![
        OsString::from("detach"),
        OsString::from(format!("/dev/{}", attached.whole_disk_identifier)),
    ];
    let receipt = run_command(staging, *sequence, label, Path::new(HDIUTIL), &arguments)?;
    *sequence += 1;
    if receipt.exit_code != 0
        || receipt
            .arguments
            .iter()
            .any(|argument| argument == "-force" || argument == "force")
    {
        return Err(invalid("non-forced hdiutil detach failed"));
    }
    commands.push(receipt);
    Ok(())
}

fn same_inventory_topology(left: &DiskInventoryV1, right: &DiskInventoryV1) -> bool {
    left.all_disks == right.all_disks
        && left.all_whole_disks == right.all_whole_disks
        && left.hdiutil_backing_paths == right.hdiutil_backing_paths
        && left.t5_apfs_container_reference == right.t5_apfs_container_reference
        && left.t5_device_identifier == right.t5_device_identifier
        && left.t5_parent_whole_disk == right.t5_parent_whole_disk
        && left.t5_physical_store_identifier == right.t5_physical_store_identifier
        && left.t5_volume_uuid == right.t5_volume_uuid
}

fn same_image_backing_inode(left: &FileIdentityV1, right: &FileIdentityV1) -> bool {
    left.path == right.path
        && left.binding.dev == right.binding.dev
        && left.binding.inode == right.binding.inode
        && left.binding.uid == right.binding.uid
        && left.binding.gid == right.binding.gid
        && left.binding.mode == right.binding.mode
        && left.binding.flags == right.binding.flags
        && left.binding.nlink == right.binding.nlink
        && left.binding.size == right.binding.size
}

fn validate_disk_arbitration_terminal(
    terminal: &DiskArbitrationTerminalV1,
    baseline: &DiskInventoryV1,
    expected_mountpoint_underlying: &ObjectBindingV1,
) -> Result<(), AcceptanceError> {
    validate_disk_inventory(&terminal.post_inventory)?;
    require_digest(
        &terminal.diskutil_info_command_sha256,
        "DiskArbitration terminal command",
    )?;
    require_digest(
        &terminal.hdiutil_info_command_sha256,
        "hdiutil terminal command",
    )?;
    if terminal.schema != "hepta_mac_diskarbitration_terminal_v1"
        || !valid_disk_identifier(&terminal.whole_disk_identifier)
        || terminal.diskutil_info_exit_code == 0
        || terminal.devnode_lstat_errno != libc::ENOENT
        || &terminal.mountpoint_underlying_after != expected_mountpoint_underlying
        || !terminal.nested_mounts_after.is_empty()
        || !same_inventory_topology(&terminal.post_inventory, baseline)
    {
        return Err(invalid(
            "DiskArbitration terminal does not prove disappearance and inventory restoration",
        ));
    }
    Ok(())
}

fn confirm_disk_arbitration_terminal(
    staging: &Path,
    namespace: &Path,
    sequence: &mut usize,
    phase: MountPhaseV1,
    attached: &AttachedImage,
    baseline: &DiskInventoryV1,
    expected_mountpoint_underlying: &ObjectBindingV1,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<DiskArbitrationTerminalV1, AcceptanceError> {
    let suffix = match phase {
        MountPhaseV1::ReadWrite => "read-write",
        MountPhaseV1::ReadOnly => "read-only",
    };
    let hdi_label = format!("hdi-info-after-detach-{suffix}");
    let hdi_receipt = run_command(
        staging,
        *sequence,
        &hdi_label,
        Path::new(HDIUTIL),
        &[OsString::from("info"), OsString::from("-plist")],
    )?;
    *sequence += 1;
    if hdi_receipt.exit_code != 0 {
        return Err(invalid("hdiutil terminal inventory failed"));
    }
    commands.push(hdi_receipt.clone());
    let hdi_info: HdiutilInfoPlist = convert_plist(
        staging,
        sequence,
        &format!("parse-{hdi_label}"),
        &hdi_receipt,
        commands,
    )?;
    let image_path = &attached
        .topology
        .as_ref()
        .ok_or_else(|| invalid("attached topology is missing at detach terminal"))?
        .image_path_from_hdiutil;
    if hdi_info
        .images
        .iter()
        .any(|candidate| &candidate.image_path == image_path)
    {
        return Err(invalid(
            "hdiutil terminal inventory still contains the disposable image",
        ));
    }

    let diskutil_label = format!("disk-arbitration-confirm-{suffix}");
    let device_node = format!("/dev/{}", attached.whole_disk_identifier);
    let diskutil_receipt = run_command(
        staging,
        *sequence,
        &diskutil_label,
        Path::new(DISKUTIL),
        &[
            OsString::from("info"),
            OsString::from("-plist"),
            OsString::from(&device_node),
        ],
    )?;
    *sequence += 1;
    if diskutil_receipt.exit_code == 0 {
        return Err(invalid(
            "DiskArbitration still resolves the detached whole disk",
        ));
    }
    commands.push(diskutil_receipt.clone());
    let devnode_lstat_errno = match fs::symlink_metadata(&device_node) {
        Ok(_) => 0,
        Err(error) => error.raw_os_error().unwrap_or(libc::EIO),
    };
    let post_inventory = collect_disk_inventory(
        staging,
        sequence,
        &format!("post-{suffix}-detach"),
        commands,
    )?;
    let terminal = DiskArbitrationTerminalV1 {
        devnode_lstat_errno,
        diskutil_info_command_sha256: command_receipt_sha256(&diskutil_receipt)?,
        diskutil_info_exit_code: diskutil_receipt.exit_code,
        hdiutil_info_command_sha256: command_receipt_sha256(&hdi_receipt)?,
        mountpoint_underlying_after: binding(&staging.join(MOUNTPOINT_NAME))?,
        nested_mounts_after: nested_mounts_below(namespace)?,
        post_inventory,
        schema: "hepta_mac_diskarbitration_terminal_v1".to_string(),
        whole_disk_identifier: attached.whole_disk_identifier.clone(),
    };
    validate_disk_arbitration_terminal(&terminal, baseline, expected_mountpoint_underlying)?;
    Ok(terminal)
}

#[repr(C)]
struct AttrList {
    bitmapcount: u16,
    reserved: u16,
    commonattr: u32,
    volattr: u32,
    dirattr: u32,
    fileattr: u32,
    forkattr: u32,
}

#[repr(C)]
struct VolumeUuidBuffer {
    length: u32,
    uuid: [u8; 16],
}

const ATTR_BIT_MAP_COUNT: u16 = 5;
const ATTR_VOL_UUID: u32 = 0x0004_0000;
const ATTR_VOL_INFO: u32 = 0x8000_0000;
const T5_VOLUME_UUID: [u8; 16] = [
    0xfb, 0x80, 0x4d, 0x1b, 0x24, 0xcb, 0x4d, 0x6e, 0xae, 0xa7, 0xa9, 0xe1, 0x80, 0x80, 0x77, 0x58,
];

/// Opaque authorization assembled entirely from broker-native facts.  It is
/// deliberately not serializable and has no public fields, so a caller cannot
/// manufacture a privileged result from JSON or a boolean flag.
pub struct BrokerNativeExecution<'a> {
    helper: FileIdentityV1,
    namespace: PathBuf,
    operation_nonce: String,
    peer: &'a AuthenticatedPeerV1,
    policy: &'a NamespacePolicy,
    source: PathBuf,
}

/// Validate, but do not execute, a future disposable privileged fixture.
///
/// This constructor has no side effects.  It requires the already-installed
/// broker policy and authenticated connected peer, verifies the helper byte
/// pin, descriptor-opens the fixed T5 namespace, and binds the source path.
/// The returned value remains mechanism-only; executing and sealing it can
/// never grant migration authority.
pub fn authorize_broker_native_execution<'a>(
    namespace: &Path,
    source: &Path,
    operation_nonce: &str,
    peer: &'a AuthenticatedPeerV1,
    policy: &'a NamespacePolicy,
) -> Result<BrokerNativeExecution<'a>, AcceptanceError> {
    require_nonce(operation_nonce)?;
    if unsafe { libc::geteuid() } != 0
        || unsafe { libc::getegid() } != 0
        || !policy.is_privileged_qualification_mode()
        || !policy.validates_authenticated_peer(peer)
        || policy.target_producer_uid() != LIVE_PRODUCER_UID
        || policy.target_producer_gid() != LIVE_PRODUCER_GID
    {
        return Err(invalid(
            "broker-native fixture authorization requires root:wheel and the fixed _hepta UID/GID",
        ));
    }
    let expected_namespace = Path::new("/Volumes/T5")
        .join(format!("{LIVE_ROOT_PREFIX}{operation_nonce}"))
        .join("publication");
    if namespace != expected_namespace {
        return Err(invalid(
            "broker-native fixture namespace is not the fixed per-operation T5 publication root",
        ));
    }
    verify_canonical_directory(namespace, 0, 0, 0o700, "fixture publication namespace")?;
    verify_t5_descriptor(namespace)?;
    if !nested_mounts_below(namespace)?.is_empty() {
        return Err(invalid(
            "fixture publication namespace already contains a nested mount",
        ));
    }

    if !source.is_absolute() {
        return Err(invalid("fixture source path is not canonical and absolute"));
    }
    let canonical_source = source.canonicalize().map_err(AcceptanceError::from)?;
    if canonical_source.as_path() != source {
        return Err(invalid("fixture source path is not canonical and absolute"));
    }
    let _ = observe_descriptor(&canonical_source)?;
    let helper_path = Path::new(policy.helper_executable_path());
    if std::env::current_exe()?.as_os_str().as_bytes() != helper_path.as_os_str().as_bytes() {
        return Err(invalid(
            "running helper path differs from the fixed installed policy",
        ));
    }
    let helper = file_identity(helper_path)?;
    validate_root_owned_executable(&helper, policy.helper_executable_sha256())?;
    Ok(BrokerNativeExecution {
        helper,
        namespace: namespace.to_path_buf(),
        operation_nonce: operation_nonce.to_string(),
        peer,
        policy,
        source: canonical_source,
    })
}

fn verify_canonical_directory(
    path: &Path,
    uid: u32,
    gid: u32,
    mode: u32,
    label: &str,
) -> Result<(), AcceptanceError> {
    if !path.is_absolute() || path.canonicalize()? != path {
        return Err(invalid(format!(
            "{label} is not an absolute canonical path"
        )));
    }
    let observed = binding(path)?;
    if !fs::symlink_metadata(path)?.is_dir()
        || observed.uid != uid
        || observed.gid != gid
        || observed.mode != mode
    {
        return Err(invalid(format!(
            "{label} is not a real {uid}:{gid} mode {mode:04o} directory"
        )));
    }
    Ok(())
}

fn verify_t5_descriptor(path: &Path) -> Result<(), AcceptanceError> {
    if !path.is_absolute() || path.canonicalize()? != path || !path.starts_with("/Volumes/T5/") {
        return Err(invalid(
            "fixture namespace is not an absolute canonical T5 descendant",
        ));
    }
    let t5 = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open("/Volumes/T5")?;
    let namespace = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open(path)?;
    let t5_statfs = fstatfs_facts(t5.as_raw_fd())?;
    let namespace_statfs = fstatfs_facts(namespace.as_raw_fd())?;
    if t5_statfs.filesystem_type != "apfs"
        || t5_statfs.mount_on != "/Volumes/T5"
        || t5_statfs.mount_flags & MNT_IGNORE_OWNERSHIP != 0
        || namespace_statfs.filesystem_id != t5_statfs.filesystem_id
        || namespace_statfs.filesystem_type != t5_statfs.filesystem_type
        || namespace_statfs.mount_from != t5_statfs.mount_from
        || namespace_statfs.mount_on != "/Volumes/T5"
        || namespace_statfs.mount_flags != t5_statfs.mount_flags
        || namespace.metadata()?.dev() != t5.metadata()?.dev()
    {
        return Err(invalid(
            "fixture namespace is a nested mount or differs from ownership-enabled T5 APFS",
        ));
    }
    let t5_uuid = volume_uuid_for_fd(t5.as_raw_fd())?;
    let namespace_uuid = volume_uuid_for_fd(namespace.as_raw_fd())?;
    if t5_uuid != namespace_uuid || t5_uuid != T5_VOLUME_UUID {
        return Err(invalid(
            "fixture namespace descriptor differs from the exact T5 volume UUID",
        ));
    }
    Ok(())
}

fn volume_uuid_for_fd(fd: RawFd) -> Result<[u8; 16], AcceptanceError> {
    let mut attributes = AttrList {
        bitmapcount: ATTR_BIT_MAP_COUNT,
        reserved: 0,
        commonattr: 0,
        volattr: ATTR_VOL_INFO | ATTR_VOL_UUID,
        dirattr: 0,
        fileattr: 0,
        forkattr: 0,
    };
    let mut buffer = VolumeUuidBuffer {
        length: 0,
        uuid: [0; 16],
    };
    if unsafe {
        libc::fgetattrlist(
            fd,
            (&mut attributes as *mut AttrList).cast(),
            (&mut buffer as *mut VolumeUuidBuffer).cast(),
            std::mem::size_of::<VolumeUuidBuffer>(),
            0,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    if buffer.length as usize != std::mem::size_of::<VolumeUuidBuffer>() {
        return Err(invalid(
            "volume UUID descriptor response has invalid length",
        ));
    }
    Ok(buffer.uuid)
}

fn boot_session_uuid() -> Result<String, AcceptanceError> {
    let name = CString::new("kern.bootsessionuuid").expect("fixed sysctl name");
    let mut length = 0_usize;
    if unsafe {
        libc::sysctlbyname(
            name.as_ptr(),
            std::ptr::null_mut(),
            &mut length,
            std::ptr::null_mut(),
            0,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    if !(2..=128).contains(&length) {
        return Err(invalid("boot session UUID sysctl length is invalid"));
    }
    let mut bytes = vec![0_u8; length];
    if unsafe {
        libc::sysctlbyname(
            name.as_ptr(),
            bytes.as_mut_ptr().cast(),
            &mut length,
            std::ptr::null_mut(),
            0,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    bytes.truncate(length);
    if bytes.last() == Some(&0) {
        bytes.pop();
    }
    let uuid = String::from_utf8(bytes)
        .map_err(|_| invalid("boot session UUID is not UTF-8"))?
        .to_ascii_lowercase();
    require_uuid(&uuid, "boot session UUID")?;
    Ok(uuid)
}

fn monotonic_nanoseconds() -> Result<u64, AcceptanceError> {
    let mut time: libc::timespec = unsafe { std::mem::zeroed() };
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_RAW, &mut time) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if time.tv_sec < 0 || time.tv_nsec < 0 {
        return Err(invalid("monotonic clock returned a negative epoch"));
    }
    (time.tv_sec as u64)
        .checked_mul(1_000_000_000)
        .and_then(|seconds| seconds.checked_add(time.tv_nsec as u64))
        .ok_or_else(|| invalid("monotonic epoch overflowed"))
}

fn observe_descriptor(path: &Path) -> Result<DescriptorObservationV1, AcceptanceError> {
    if !path.is_absolute() || path.canonicalize()? != path {
        return Err(invalid(
            "descriptor observation path is not canonical and absolute",
        ));
    }
    let parent = path
        .parent()
        .ok_or_else(|| invalid("descriptor observation has no parent"))?;
    let name = path
        .file_name()
        .ok_or_else(|| invalid("descriptor observation cannot target filesystem root"))?;
    let parent_fd = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open(parent)?;
    let before = binding(path)?;
    let name = CString::new(name.as_bytes())
        .map_err(|_| invalid("descriptor observation name contains NUL"))?;
    let fd = unsafe {
        libc::openat(
            parent_fd.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    let fd_binding = binding_from_metadata(&file.metadata()?);
    let after = binding(path)?;
    if before != fd_binding || before != after {
        return Err(invalid(
            "descriptor-observed source changed during openat replay",
        ));
    }
    Ok(DescriptorObservationV1 {
        absolute_path: path
            .to_str()
            .ok_or_else(|| invalid("descriptor observation path is not UTF-8"))?
            .to_string(),
        fd_binding,
        parent_binding: binding_from_metadata(&parent_fd.metadata()?),
        path_binding_after: after,
        path_binding_before: before,
        schema: "hepta_mac_descriptor_observation_v1".to_string(),
        statfs: fstatfs_facts(file.as_raw_fd())?,
    })
}

fn validate_descriptor_observation(
    observation: &DescriptorObservationV1,
) -> Result<(), AcceptanceError> {
    if observation.schema != "hepta_mac_descriptor_observation_v1"
        || !Path::new(&observation.absolute_path).is_absolute()
        || observation.fd_binding != observation.path_binding_before
        || observation.fd_binding != observation.path_binding_after
        || observation.fd_binding.nlink == 0
        || observation.statfs.filesystem_type.is_empty()
        || observation.statfs.mount_on.is_empty()
        || observation.statfs.mount_from.is_empty()
    {
        return Err(invalid(
            "descriptor observation is not an exact stable openat/fstat replay",
        ));
    }
    Ok(())
}

fn challenge_for_epoch(epoch: &OperationEpochV1) -> Result<String, AcceptanceError> {
    let material = EpochChallengeMaterialV1 {
        boot_session_uuid: epoch.boot_session_uuid.clone(),
        helper_executable_sha256: epoch.helper_executable.sha256.clone(),
        monotonic_nanoseconds: epoch.monotonic_nanoseconds,
        mountpoint_underlying_before: epoch.mountpoint_underlying_before.clone(),
        operation_nonce: epoch.operation_nonce.clone(),
        schema: "hepta_mac_apfs_epoch_challenge_material_v1".to_string(),
        source_binding: epoch.source_before.fd_binding.clone(),
    };
    Ok(sha256(&canonical_json(&material)?))
}

fn validate_epoch(epoch: &OperationEpochV1) -> Result<(), AcceptanceError> {
    require_nonce(&epoch.operation_nonce)?;
    require_uuid(&epoch.boot_session_uuid, "epoch boot session UUID")?;
    require_digest(&epoch.challenge_sha256, "epoch challenge")?;
    require_digest(&epoch.helper_executable.sha256, "epoch helper")?;
    validate_descriptor_observation(&epoch.source_before)?;
    if epoch.schema != "hepta_mac_apfs_operation_epoch_v1"
        || epoch.monotonic_nanoseconds == 0
        || epoch.mount_parent_before.uid != 0
        || epoch.mount_parent_before.gid != 0
        || epoch.mount_parent_before.mode != 0o700
        || epoch.mountpoint_underlying_before.uid != 0
        || epoch.mountpoint_underlying_before.gid != 0
        || epoch.mountpoint_underlying_before.mode != 0o700
        || challenge_for_epoch(epoch)? != epoch.challenge_sha256
    {
        return Err(invalid(
            "operation epoch does not bind exact boot/helper/source/mountpoint facts",
        ));
    }
    Ok(())
}

struct ReceiptWriter {
    boot_session_uuid: String,
    challenge_sha256: String,
    epoch_receipt_sha256: String,
    operation_nonce: String,
    raw_root: PathBuf,
    receipts: Vec<RawReceiptReferenceV1>,
    sequence: u32,
    source_binding: ObjectBindingV1,
}

impl ReceiptWriter {
    fn new(staging: &Path, epoch: &OperationEpochV1) -> Result<Self, AcceptanceError> {
        validate_epoch(epoch)?;
        let epoch_bytes = canonical_json(epoch)?;
        let epoch_path = staging.join("raw/000-epoch.json");
        write_new(&epoch_path, &epoch_bytes, 0o400)?;
        File::open(staging.join("raw"))?.sync_all()?;
        Ok(Self {
            boot_session_uuid: epoch.boot_session_uuid.clone(),
            challenge_sha256: epoch.challenge_sha256.clone(),
            epoch_receipt_sha256: sha256(&epoch_bytes),
            operation_nonce: epoch.operation_nonce.clone(),
            raw_root: staging.join("raw"),
            receipts: Vec::new(),
            sequence: 1,
            source_binding: epoch.source_before.fd_binding.clone(),
        })
    }

    fn append(
        &mut self,
        label: &str,
        fact: NativeFactV1,
    ) -> Result<RawReceiptReferenceV1, AcceptanceError> {
        require_label(label)?;
        let receipt = NativeFactReceiptV1 {
            boot_session_uuid: self.boot_session_uuid.clone(),
            challenge_sha256: self.challenge_sha256.clone(),
            epoch_receipt_sha256: self.epoch_receipt_sha256.clone(),
            fact,
            label: label.to_string(),
            operation_nonce: self.operation_nonce.clone(),
            schema: "hepta_mac_apfs_native_fact_receipt_v1".to_string(),
            sequence: self.sequence,
            source_binding: self.source_binding.clone(),
        };
        let bytes = canonical_json(&receipt)?;
        let relative = format!("raw/{:03}-{label}.json", self.sequence);
        write_new(
            &self
                .raw_root
                .join(format!("{:03}-{label}.json", self.sequence)),
            &bytes,
            0o400,
        )?;
        File::open(&self.raw_root)?.sync_all()?;
        let reference = RawReceiptReferenceV1 {
            label: label.to_string(),
            path: relative,
            sequence: self.sequence,
            sha256: sha256(&bytes),
            size: bytes.len() as u64,
        };
        self.receipts.push(reference.clone());
        self.sequence = self
            .sequence
            .checked_add(1)
            .ok_or_else(|| invalid("native fact sequence overflowed"))?;
        Ok(reference)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ObligationState {
    attached_phase: Option<MountPhaseV1>,
    attached_volume: Option<String>,
    attached_whole: Option<String>,
    completed_read_only: bool,
    completed_read_write: bool,
    detach_started: bool,
    disposition: ObligationDispositionV1,
    mount_started: bool,
    mounted: bool,
    mounted_on: Option<String>,
    prepared: bool,
    terminal: bool,
    unmount_started: bool,
}

impl ObligationState {
    fn empty() -> Self {
        Self {
            attached_phase: None,
            attached_volume: None,
            attached_whole: None,
            completed_read_only: false,
            completed_read_write: false,
            detach_started: false,
            disposition: ObligationDispositionV1::Active,
            mount_started: false,
            mounted: false,
            mounted_on: None,
            prepared: false,
            terminal: false,
            unmount_started: false,
        }
    }

    fn apply(
        &mut self,
        event: &AttachmentObligationEventV1,
        disposition: ObligationDispositionV1,
    ) -> Result<(), AcceptanceError> {
        if self.terminal {
            return Err(invalid("attachment obligation is already terminal"));
        }
        match event {
            AttachmentObligationEventV1::Prepared {
                nested_mounts_before,
                ..
            } => {
                if self.prepared
                    || !nested_mounts_before.is_empty()
                    || disposition != ObligationDispositionV1::Active
                {
                    return Err(invalid("attachment obligation Prepared is out of order"));
                }
                self.prepared = true;
            }
            AttachmentObligationEventV1::AttachStarted { phase } => {
                let expected_phase = if !self.completed_read_write {
                    MountPhaseV1::ReadWrite
                } else {
                    MountPhaseV1::ReadOnly
                };
                if !self.prepared
                    || self.disposition != ObligationDispositionV1::Active
                    || self.attached_phase.is_some()
                    || self.detach_started
                    || self.mounted
                    || *phase != expected_phase
                    || self.completed_read_only
                    || disposition != ObligationDispositionV1::Active
                {
                    return Err(invalid(
                        "attachment obligation AttachStarted is out of order",
                    ));
                }
                self.attached_phase = Some(*phase);
            }
            AttachmentObligationEventV1::Attached { phase, topology } => {
                validate_attached_topology_shape(topology)?;
                if self.attached_phase != Some(*phase)
                    || self.attached_whole.is_some()
                    || disposition != ObligationDispositionV1::Active
                {
                    return Err(invalid("attachment obligation Attached is out of order"));
                }
                self.attached_whole = Some(topology.whole_disk.device_identifier.clone());
                self.attached_volume = Some(topology.apfs_volume.device_identifier.clone());
            }
            AttachmentObligationEventV1::MountStarted {
                phase,
                volume_identifier,
            } => {
                if self.attached_phase != Some(*phase)
                    || self.attached_volume.as_deref() != Some(volume_identifier)
                    || self.attached_whole.is_none()
                    || self.mount_started
                    || self.mounted
                    || self.unmount_started
                    || self.detach_started
                    || disposition != ObligationDispositionV1::Active
                {
                    return Err(invalid(
                        "attachment obligation MountStarted is out of order",
                    ));
                }
                self.mount_started = true;
            }
            AttachmentObligationEventV1::Mounted {
                mountpoint_statfs,
                phase,
            } => {
                let expected_volume = self
                    .attached_volume
                    .as_deref()
                    .ok_or_else(|| invalid("attachment obligation mounted without a volume"))?;
                if self.attached_phase != Some(*phase)
                    || !self.mount_started
                    || self.mounted
                    || self.unmount_started
                    || self.detach_started
                    || mountpoint_statfs.filesystem_type != "apfs"
                    || !mountpoint_statfs.mount_from.ends_with(expected_volume)
                    || !Path::new(&mountpoint_statfs.mount_on).is_absolute()
                    || !Path::new(&mountpoint_statfs.mount_on).starts_with("/Volumes/T5/")
                    || mountpoint_statfs.mount_flags
                        & (MNT_NODEV | MNT_NOSUID | MNT_NOEXEC | MNT_NOATIME)
                        != (MNT_NODEV | MNT_NOSUID | MNT_NOEXEC | MNT_NOATIME)
                    || mountpoint_statfs.mount_flags & MNT_IGNORE_OWNERSHIP != 0
                    || (*phase == MountPhaseV1::ReadOnly
                        && mountpoint_statfs.mount_flags & MNT_RDONLY == 0)
                    || (*phase == MountPhaseV1::ReadWrite
                        && mountpoint_statfs.mount_flags & MNT_RDONLY != 0)
                    || disposition != ObligationDispositionV1::Active
                {
                    return Err(invalid("attachment obligation Mounted is out of order"));
                }
                self.mount_started = false;
                self.mounted = true;
                self.mounted_on = Some(mountpoint_statfs.mount_on.clone());
            }
            AttachmentObligationEventV1::UnmountStarted { phase } => {
                if self.attached_phase != Some(*phase)
                    || self.mount_started
                    || !self.mounted
                    || self.unmount_started
                    || self.detach_started
                    || disposition != ObligationDispositionV1::Active
                {
                    return Err(invalid(
                        "attachment obligation UnmountStarted is out of order",
                    ));
                }
                self.unmount_started = true;
            }
            AttachmentObligationEventV1::Unmounted { phase, receipt } => {
                if self.attached_phase != Some(*phase)
                    || !self.mounted
                    || !self.unmount_started
                    || self.mounted_on.as_deref() != Some(receipt.mountpoint.as_str())
                    || receipt.rc != 0
                    || receipt.errno != 0
                    || receipt.flags != 0
                    || disposition != ObligationDispositionV1::Active
                {
                    return Err(invalid("attachment obligation Unmounted is out of order"));
                }
                self.mounted = false;
                self.mounted_on = None;
                self.unmount_started = false;
            }
            AttachmentObligationEventV1::DetachStarted {
                phase,
                whole_disk_identifier,
            } => {
                if self.attached_phase != Some(*phase)
                    || self.attached_whole.as_deref() != Some(whole_disk_identifier)
                    || self.mount_started
                    || self.mounted
                    || self.unmount_started
                    || self.detach_started
                    || disposition != ObligationDispositionV1::Active
                {
                    return Err(invalid(
                        "attachment obligation DetachStarted is out of order",
                    ));
                }
                self.detach_started = true;
            }
            AttachmentObligationEventV1::DiskArbitrationGone { phase, terminal } => {
                if self.attached_phase != Some(*phase)
                    || self.attached_whole.as_deref() != Some(&terminal.whole_disk_identifier)
                    || !self.detach_started
                    || self.mount_started
                    || self.mounted
                    || self.unmount_started
                    || terminal.diskutil_info_exit_code == 0
                    || terminal.devnode_lstat_errno != libc::ENOENT
                    || !terminal.nested_mounts_after.is_empty()
                    || disposition != ObligationDispositionV1::Active
                {
                    return Err(invalid(
                        "attachment obligation DiskArbitrationGone is out of order",
                    ));
                }
                match phase {
                    MountPhaseV1::ReadWrite => self.completed_read_write = true,
                    MountPhaseV1::ReadOnly => self.completed_read_only = true,
                }
                self.attached_phase = None;
                self.attached_whole = None;
                self.attached_volume = None;
                self.detach_started = false;
            }
            AttachmentObligationEventV1::ReconcileRequired { reason_sha256 } => {
                require_digest(reason_sha256, "reconcile-required reason")?;
                if !self.prepared
                    || self.disposition != ObligationDispositionV1::Active
                    || disposition != ObligationDispositionV1::ReconcileRequired
                {
                    return Err(invalid("ReconcileRequired is out of order"));
                }
                self.disposition = ObligationDispositionV1::ReconcileRequired;
            }
            AttachmentObligationEventV1::Quarantined { reason_sha256, .. } => {
                require_digest(reason_sha256, "quarantine reason")?;
                if self.disposition != ObligationDispositionV1::ReconcileRequired
                    || disposition != ObligationDispositionV1::Quarantined
                {
                    return Err(invalid("Quarantined is out of order"));
                }
                self.disposition = ObligationDispositionV1::Quarantined;
                self.terminal = true;
            }
            AttachmentObligationEventV1::TerminalReconciled { .. } => {
                if !self.completed_read_write
                    || !self.completed_read_only
                    || self.attached_phase.is_some()
                    || self.mounted
                    || self.mount_started
                    || self.unmount_started
                    || self.detach_started
                    || self.mounted_on.is_some()
                    || self.disposition != ObligationDispositionV1::Active
                    || disposition != ObligationDispositionV1::Reconciled
                {
                    return Err(invalid("TerminalReconciled is out of order"));
                }
                self.disposition = ObligationDispositionV1::Reconciled;
                self.terminal = true;
            }
        }
        if !matches!(
            event,
            AttachmentObligationEventV1::ReconcileRequired { .. }
                | AttachmentObligationEventV1::Quarantined { .. }
                | AttachmentObligationEventV1::TerminalReconciled { .. }
        ) && disposition != self.disposition
        {
            return Err(invalid("attachment obligation disposition drifted"));
        }
        Ok(())
    }
}

struct AttachmentObligationJournal {
    boot_session_uuid: String,
    challenge_sha256: String,
    directory: PathBuf,
    epoch_receipt_sha256: String,
    namespace: PathBuf,
    operation_nonce: String,
    previous_record_sha256: Option<String>,
    sequence: u32,
    state: ObligationState,
}

impl AttachmentObligationJournal {
    fn create(
        namespace: &Path,
        operation_nonce: &str,
        epoch: &OperationEpochV1,
        epoch_receipt_sha256: &str,
        prepared: AttachmentObligationEventV1,
    ) -> Result<Self, AcceptanceError> {
        require_nonce(operation_nonce)?;
        require_digest(epoch_receipt_sha256, "obligation epoch receipt")?;
        let directory = namespace.join(format!("{OBLIGATION_PREFIX}{operation_nonce}"));
        fs::create_dir(&directory)?;
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))?;
        File::open(&directory)?.sync_all()?;
        File::open(namespace)?.sync_all()?;
        let mut journal = Self {
            boot_session_uuid: epoch.boot_session_uuid.clone(),
            challenge_sha256: epoch.challenge_sha256.clone(),
            directory,
            epoch_receipt_sha256: epoch_receipt_sha256.to_string(),
            namespace: namespace.to_path_buf(),
            operation_nonce: operation_nonce.to_string(),
            previous_record_sha256: None,
            sequence: 1,
            state: ObligationState::empty(),
        };
        journal.append(prepared, ObligationDispositionV1::Active)?;
        Ok(journal)
    }

    fn append(
        &mut self,
        event: AttachmentObligationEventV1,
        disposition: ObligationDispositionV1,
    ) -> Result<String, AcceptanceError> {
        verify_t5_descriptor(&self.namespace)?;
        let mut next_state = self.state.clone();
        next_state.apply(&event, disposition)?;
        let record = AttachmentObligationRecordV1 {
            authority_granted: false,
            boot_session_uuid: self.boot_session_uuid.clone(),
            challenge_sha256: self.challenge_sha256.clone(),
            disposition,
            epoch_receipt_sha256: self.epoch_receipt_sha256.clone(),
            event,
            operation_nonce: self.operation_nonce.clone(),
            previous_record_sha256: self.previous_record_sha256.clone(),
            schema: "hepta_mac_attachment_obligation_record_v1".to_string(),
            sequence: self.sequence,
        };
        let bytes = canonical_json(&record)?;
        let digest = sha256(&bytes);
        write_new(
            &self.directory.join(format!("{:08}.json", self.sequence)),
            &bytes,
            0o400,
        )?;
        File::open(&self.directory)?.sync_all()?;
        File::open(&self.namespace)?.sync_all()?;
        self.previous_record_sha256 = Some(digest.clone());
        self.sequence = self
            .sequence
            .checked_add(1)
            .ok_or_else(|| invalid("attachment obligation sequence overflowed"))?;
        self.state = next_state;
        Ok(digest)
    }

    fn quarantine(&mut self, reason: &str, cross_boot: bool) {
        if self.state.terminal {
            return;
        }
        let reason_sha256 = sha256(reason.as_bytes());
        if self.state.disposition == ObligationDispositionV1::Active {
            let _ = self.append(
                AttachmentObligationEventV1::ReconcileRequired {
                    reason_sha256: reason_sha256.clone(),
                },
                ObligationDispositionV1::ReconcileRequired,
            );
        }
        if self.state.disposition == ObligationDispositionV1::ReconcileRequired {
            let _ = self.append(
                AttachmentObligationEventV1::Quarantined {
                    cross_boot,
                    reason_sha256,
                },
                ObligationDispositionV1::Quarantined,
            );
        }
    }

    fn relative_directory(&self) -> String {
        self.directory
            .file_name()
            .expect("obligation directory component")
            .to_string_lossy()
            .to_string()
    }
}

struct AttachmentGuard {
    active_topology: Option<AttachedTopologyV1>,
    armed: bool,
    baseline_inventory: DiskInventoryV1,
    expected_mountpoint_underlying: ObjectBindingV1,
    image: PathBuf,
    journal: AttachmentObligationJournal,
    mounted: bool,
    mountpoint: PathBuf,
}

impl AttachmentGuard {
    fn new(
        journal: AttachmentObligationJournal,
        baseline_inventory: DiskInventoryV1,
        image: PathBuf,
        mountpoint: PathBuf,
        expected_mountpoint_underlying: ObjectBindingV1,
    ) -> Self {
        Self {
            active_topology: None,
            armed: true,
            baseline_inventory,
            expected_mountpoint_underlying,
            image,
            journal,
            mounted: false,
            mountpoint,
        }
    }

    fn attach_started(&mut self, phase: MountPhaseV1) -> Result<(), AcceptanceError> {
        self.journal.append(
            AttachmentObligationEventV1::AttachStarted { phase },
            ObligationDispositionV1::Active,
        )?;
        Ok(())
    }

    fn attached(
        &mut self,
        phase: MountPhaseV1,
        topology: &AttachedTopologyV1,
    ) -> Result<(), AcceptanceError> {
        validate_attached_topology_shape(topology)?;
        self.journal.append(
            AttachmentObligationEventV1::Attached {
                phase,
                topology: topology.clone(),
            },
            ObligationDispositionV1::Active,
        )?;
        self.active_topology = Some(topology.clone());
        Ok(())
    }

    fn mount_started(
        &mut self,
        phase: MountPhaseV1,
        volume_identifier: &str,
    ) -> Result<(), AcceptanceError> {
        self.journal.append(
            AttachmentObligationEventV1::MountStarted {
                phase,
                volume_identifier: volume_identifier.to_string(),
            },
            ObligationDispositionV1::Active,
        )?;
        Ok(())
    }

    fn mounted(
        &mut self,
        phase: MountPhaseV1,
        statfs: StatFsFactsV1,
    ) -> Result<(), AcceptanceError> {
        if statfs.mount_on != self.mountpoint.to_string_lossy() {
            return Err(invalid("mounted obligation statfs targets another path"));
        }
        let nested = nested_mounts_below(&self.journal.namespace)?;
        if nested.as_slice() != [statfs.clone()] {
            return Err(invalid(
                "mounted obligation namespace does not contain exactly the proven mount",
            ));
        }
        self.journal.append(
            AttachmentObligationEventV1::Mounted {
                mountpoint_statfs: statfs,
                phase,
            },
            ObligationDispositionV1::Active,
        )?;
        self.mounted = true;
        Ok(())
    }

    fn unmount_started(&mut self, phase: MountPhaseV1) -> Result<(), AcceptanceError> {
        self.journal.append(
            AttachmentObligationEventV1::UnmountStarted { phase },
            ObligationDispositionV1::Active,
        )?;
        Ok(())
    }

    fn unmounted(
        &mut self,
        phase: MountPhaseV1,
        receipt: &RawUnmountReceiptV1,
    ) -> Result<(), AcceptanceError> {
        validate_unmount(receipt, false, &self.mountpoint)?;
        if !nested_mounts_below(&self.journal.namespace)?.is_empty() {
            return Err(invalid(
                "unmounted obligation namespace still contains a nested mount",
            ));
        }
        self.journal.append(
            AttachmentObligationEventV1::Unmounted {
                phase,
                receipt: receipt.clone(),
            },
            ObligationDispositionV1::Active,
        )?;
        self.mounted = false;
        Ok(())
    }

    fn detach_started(
        &mut self,
        phase: MountPhaseV1,
        whole_disk_identifier: &str,
    ) -> Result<(), AcceptanceError> {
        self.journal.append(
            AttachmentObligationEventV1::DetachStarted {
                phase,
                whole_disk_identifier: whole_disk_identifier.to_string(),
            },
            ObligationDispositionV1::Active,
        )?;
        Ok(())
    }

    fn disk_arbitration_gone(
        &mut self,
        phase: MountPhaseV1,
        terminal: &DiskArbitrationTerminalV1,
    ) -> Result<(), AcceptanceError> {
        validate_disk_arbitration_terminal(
            terminal,
            &self.baseline_inventory,
            &self.expected_mountpoint_underlying,
        )?;
        self.journal.append(
            AttachmentObligationEventV1::DiskArbitrationGone {
                phase,
                terminal: terminal.clone(),
            },
            ObligationDispositionV1::Active,
        )?;
        self.active_topology = None;
        Ok(())
    }

    fn finish(&mut self, post_inventory: DiskInventoryV1) -> Result<String, AcceptanceError> {
        if !nested_mounts_below(&self.journal.namespace)?.is_empty()
            || !same_inventory_topology(&post_inventory, &self.baseline_inventory)
        {
            return Err(invalid(
                "terminal obligation retains a nested mount or inventory differs from pre-attach",
            ));
        }
        let digest = self.journal.append(
            AttachmentObligationEventV1::TerminalReconciled { post_inventory },
            ObligationDispositionV1::Reconciled,
        )?;
        self.armed = false;
        Ok(digest)
    }

    fn fail_closed(&mut self, reason: &str) {
        if !self.armed {
            return;
        }
        let cross_boot = boot_session_uuid()
            .map(|current| current != self.journal.boot_session_uuid)
            .unwrap_or(true);
        let mut cleanup = "cleanup-not-attempted".to_string();
        if !cross_boot {
            cleanup = self.best_effort_nonforced_cleanup();
        }
        self.journal
            .quarantine(&format!("{reason}; {cleanup}"), cross_boot);
        self.armed = false;
    }

    fn best_effort_nonforced_cleanup(&mut self) -> String {
        if self.mounted {
            match statfs_facts(&self.mountpoint) {
                Ok(statfs)
                    if statfs.mount_on == self.mountpoint.to_string_lossy()
                        && self.active_topology.as_ref().is_some_and(|topology| {
                            statfs
                                .mount_from
                                .ends_with(&topology.apfs_volume.device_identifier)
                        }) =>
                {
                    match raw_unmount(&self.mountpoint) {
                        Ok(receipt)
                            if receipt.rc == 0 && receipt.errno == 0 && receipt.flags == 0 =>
                        {
                            self.mounted = false;
                        }
                        Ok(receipt) => {
                            return format!(
                                "nonforced-unmount-failed-rc{}-errno{}",
                                receipt.rc, receipt.errno
                            );
                        }
                        Err(error) => return format!("nonforced-unmount-error-{error}"),
                    }
                }
                Ok(_) => return "mountpoint-topology-ambiguous".to_string(),
                Err(error) => return format!("mountpoint-statfs-error-{error}"),
            }
        }
        let Some(topology) = self.active_topology.as_ref() else {
            return "no-proven-active-topology-safe-quarantine".to_string();
        };
        let whole = &topology.whole_disk.device_identifier;
        if self.baseline_inventory.all_disks.contains(whole)
            || [
                &self.baseline_inventory.t5_device_identifier,
                &self.baseline_inventory.t5_parent_whole_disk,
                &self.baseline_inventory.t5_apfs_container_reference,
                &self.baseline_inventory.t5_physical_store_identifier,
            ]
            .contains(&whole)
        {
            return "refused-preexisting-or-t5-detach".to_string();
        }
        match file_identity(&self.image) {
            Ok(identity) if identity == topology.image_backing_after => {}
            Ok(_) => return "image-backing-binding-changed".to_string(),
            Err(error) => return format!("image-backing-replay-error-{error}"),
        }
        let arguments = [
            OsString::from("detach"),
            OsString::from(format!("/dev/{whole}")),
        ];
        match run_reconcile_command(Path::new(HDIUTIL), &arguments) {
            Ok(0) => "nonforced-detach-issued-terminal-quarantine".to_string(),
            Ok(exit_code) => format!("nonforced-detach-failed-exit-{exit_code}"),
            Err(error) => format!("nonforced-detach-error-{error}"),
        }
    }
}

impl Drop for AttachmentGuard {
    fn drop(&mut self) {
        if self.armed {
            self.fail_closed("attachment guard dropped before terminal reconciliation");
        }
    }
}

fn run_reconcile_command(tool: &Path, arguments: &[OsString]) -> Result<i32, AcceptanceError> {
    let before = file_identity(tool)?;
    validate_system_tool_identity(&before)?;
    let ancestors = system_tool_ancestor_chain(tool)?;
    let mut command = Command::new(tool);
    command
        .args(arguments)
        .env_clear()
        .env("LC_ALL", "C")
        .env("LANG", "C")
        .env("PATH", "/usr/bin:/usr/sbin:/bin:/sbin")
        .env("HOME", "/var/empty")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command.process_group(0);
    let mut child = command.spawn()?;
    let pid = i32::try_from(child.id()).map_err(|_| invalid("reconcile PID overflowed"))?;
    if pid <= 1 || unsafe { libc::getpgid(pid) } != pid {
        let _ = terminate_process_group(&mut child, pid);
        return Err(invalid("reconcile command lacks an isolated process group"));
    }
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() > COMMAND_TIMEOUT {
            terminate_process_group(&mut child, pid)?;
            return Err(invalid("reconcile command exceeded its deadline"));
        }
        thread::sleep(Duration::from_millis(10));
    };
    if file_identity(tool)? != before || system_tool_ancestor_chain(tool)? != ancestors {
        return Err(invalid(
            "reconcile command tool or ancestor chain changed during execution",
        ));
    }
    exit_code(status)
}

fn require_label(label: &str) -> Result<(), AcceptanceError> {
    if label.is_empty()
        || label.len() > 96
        || !label
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(invalid(
            "native receipt label is not a fixed safe component",
        ));
    }
    Ok(())
}

struct HolderProcess {
    active: bool,
    identity: ProcessIdentityV1,
    pid: libc::pid_t,
    release: File,
}

impl Drop for HolderProcess {
    fn drop(&mut self) {
        if self.active && self.pid > 1 {
            let _ = terminate_and_reap_pid(self.pid);
        }
    }
}

fn pipe_pair() -> Result<(File, File), AcceptanceError> {
    let mut fds = [-1; 2];
    if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) })
}

fn wait_ready(ready: &mut File) -> Result<(i32, i32), AcceptanceError> {
    let started = Instant::now();
    let mut poll = libc::pollfd {
        fd: ready.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    loop {
        let rc = unsafe { libc::poll(&mut poll, 1, 50) };
        if rc < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        if rc > 0 {
            let mut bytes = [0_u8; 8];
            ready.read_exact(&mut bytes)?;
            let pid = i32::from_ne_bytes(bytes[..4].try_into().expect("fixed PID bytes"));
            let parent_pid = i32::from_ne_bytes(bytes[4..].try_into().expect("fixed PPID bytes"));
            if pid <= 1 || parent_pid <= 1 {
                return Err(invalid("holder readiness kernel identities are invalid"));
            }
            return Ok((pid, parent_pid));
        }
        if started.elapsed() > HOLDER_READY_TIMEOUT {
            return Err(invalid(
                "holder did not reach its descriptor barrier deadline",
            ));
        }
    }
}

fn child_holder(
    kind: HolderKindV1,
    mountpoint: &Path,
    ready: &mut File,
    release: &mut File,
) -> i32 {
    let payload = mountpoint.join("payload");
    let payload_c = match c_path(&payload) {
        Ok(value) => value,
        Err(_) => return 91,
    };
    let mountpoint_c = match c_path(mountpoint) {
        Ok(value) => value,
        Err(_) => return 92,
    };
    let fd = match kind {
        HolderKindV1::CurrentWorkingDirectoryOnly => -1,
        _ => unsafe { libc::open(payload_c.as_ptr(), libc::O_RDWR | libc::O_CLOEXEC) },
    };
    if !matches!(kind, HolderKindV1::CurrentWorkingDirectoryOnly) && fd < 0 {
        return 93;
    }
    let mapping = if matches!(kind, HolderKindV1::SharedWritableMappingOnly) {
        let mapping = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                4096,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                0,
            )
        };
        if mapping == libc::MAP_FAILED {
            unsafe { libc::close(fd) };
            return 94;
        }
        unsafe { libc::close(fd) };
        mapping
    } else {
        std::ptr::null_mut()
    };
    if matches!(kind, HolderKindV1::CurrentWorkingDirectoryOnly)
        && unsafe { libc::chdir(mountpoint_c.as_ptr()) } != 0
    {
        return 95;
    }
    let mut identity = Vec::with_capacity(8);
    identity.extend_from_slice(&(unsafe { libc::getpid() }).to_ne_bytes());
    identity.extend_from_slice(&(unsafe { libc::getppid() }).to_ne_bytes());
    if ready.write_all(&identity).is_err() {
        return 96;
    }
    let mut byte = [0_u8; 1];
    if release.read_exact(&mut byte).is_err() || byte != [1] {
        return 97;
    }
    if matches!(kind, HolderKindV1::SharedWritableMappingOnly) {
        if unsafe { libc::munmap(mapping, 4096) } != 0 {
            return 98;
        }
    } else if fd >= 0 {
        unsafe { libc::close(fd) };
    }
    0
}

fn spawn_holder(kind: HolderKindV1, mountpoint: &Path) -> Result<HolderProcess, AcceptanceError> {
    let (mut ready_read, mut ready_write) = pipe_pair()?;
    let (mut release_read, release_write) = pipe_pair()?;
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if pid == 0 {
        drop(ready_read);
        drop(release_write);
        let rc = child_holder(kind, mountpoint, &mut ready_write, &mut release_read);
        unsafe { libc::_exit(rc) };
    }
    drop(ready_write);
    drop(release_read);
    let ready_identity = match wait_ready(&mut ready_read) {
        Ok(identity) => identity,
        Err(error) => {
            terminate_and_reap_pid(pid)?;
            return Err(error);
        }
    };
    let identity = match kernel_process_identity(pid) {
        Ok(identity) => identity,
        Err(error) => {
            terminate_and_reap_pid(pid)?;
            return Err(error);
        }
    };
    if ready_identity != (identity.pid, identity.parent_pid)
        || identity.parent_pid != unsafe { libc::getpid() }
    {
        terminate_and_reap_pid(pid)?;
        return Err(invalid(
            "holder readiness identity differs from proc_pidinfo",
        ));
    }
    Ok(HolderProcess {
        active: true,
        identity,
        pid,
        release: release_write,
    })
}

fn kernel_process_identity(pid: i32) -> Result<ProcessIdentityV1, AcceptanceError> {
    let mut info: libc::proc_bsdinfo = unsafe { std::mem::zeroed() };
    let expected_size = i32::try_from(std::mem::size_of::<libc::proc_bsdinfo>())
        .map_err(|_| invalid("proc_bsdinfo size overflowed"))?;
    let observed_size = unsafe {
        libc::proc_pidinfo(
            pid,
            libc::PROC_PIDTBSDINFO,
            0,
            (&mut info as *mut libc::proc_bsdinfo).cast(),
            expected_size,
        )
    };
    let process_group_id = unsafe { libc::getpgid(pid) };
    if observed_size != expected_size
        || info.pbi_pid != pid as u32
        || info.pbi_start_tvsec == 0
        || info.pbi_start_tvusec >= 1_000_000
        || process_group_id <= 1
    {
        return Err(invalid(
            "proc_pidinfo did not return an exact live holder start identity",
        ));
    }
    Ok(ProcessIdentityV1 {
        effective_gid: info.pbi_gid,
        effective_uid: info.pbi_uid,
        parent_pid: info.pbi_ppid as i32,
        pid,
        process_group_id,
        real_gid: info.pbi_rgid,
        real_uid: info.pbi_ruid,
        start_microseconds: info.pbi_start_tvusec,
        start_seconds: info.pbi_start_tvsec,
    })
}

fn terminate_and_reap_pid(pid: i32) -> Result<i32, AcceptanceError> {
    if pid <= 1 {
        return Err(invalid("refusing to terminate invalid holder PID"));
    }
    let term_rc = unsafe { libc::kill(pid, libc::SIGTERM) };
    if term_rc != 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH) {
        return Err(std::io::Error::last_os_error().into());
    }
    let started = Instant::now();
    let mut wait_status = 0;
    while started.elapsed() < Duration::from_secs(2) {
        let waited = unsafe { libc::waitpid(pid, &mut wait_status, libc::WNOHANG) };
        if waited == pid {
            return Ok(wait_status);
        }
        if waited < 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
            return Err(std::io::Error::last_os_error().into());
        }
        thread::sleep(Duration::from_millis(10));
    }
    let kill_rc = unsafe { libc::kill(pid, libc::SIGKILL) };
    if kill_rc != 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH) {
        return Err(std::io::Error::last_os_error().into());
    }
    if unsafe { libc::waitpid(pid, &mut wait_status, 0) } != pid {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(wait_status)
}

fn waitpid_with_deadline(pid: i32) -> Result<i32, AcceptanceError> {
    let started = Instant::now();
    let mut wait_status = 0;
    loop {
        let waited = unsafe { libc::waitpid(pid, &mut wait_status, libc::WNOHANG) };
        if waited == pid {
            return Ok(wait_status);
        }
        if waited < 0 && std::io::Error::last_os_error().raw_os_error() != Some(libc::EINTR) {
            return Err(std::io::Error::last_os_error().into());
        }
        if started.elapsed() > HOLDER_READY_TIMEOUT {
            let _ = terminate_and_reap_pid(pid)?;
            return Err(invalid("holder did not exit before its wait deadline"));
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn holder_cycle(
    kind: HolderKindV1,
    mountpoint: &Path,
) -> Result<HolderCycleReceiptV1, AcceptanceError> {
    let mount_statfs_before = statfs_facts(mountpoint)?;
    let mut holder = spawn_holder(kind, mountpoint)?;
    let unmount_with_holder = raw_unmount(mountpoint)?;
    let mount_statfs_after_busy = statfs_facts(mountpoint)?;
    let mount_still_same_after_busy = mount_statfs_before == mount_statfs_after_busy;
    let holder_after_busy = kernel_process_identity(holder.pid)?;
    if holder.identity != holder_after_busy {
        let _ = terminate_and_reap_pid(holder.pid)?;
        return Err(invalid("holder PID start identity changed across EBUSY"));
    }
    holder.release.write_all(&[1])?;
    let wait_status = waitpid_with_deadline(holder.pid)?;
    holder.active = false;
    let clean_unmount_after_release = raw_unmount(mountpoint)?;
    Ok(HolderCycleReceiptV1 {
        clean_unmount_after_release,
        holder: holder.identity.clone(),
        holder_after_busy,
        holder_kind: kind,
        holder_release_wait_status: wait_status,
        mount_still_same_after_busy,
        mount_statfs_after_busy,
        mount_statfs_before,
        unmount_with_holder,
    })
}

fn kernel_credentials() -> Result<KernelCredentialsV1, AcceptanceError> {
    let group_count = unsafe { libc::getgroups(0, std::ptr::null_mut()) };
    if group_count < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let mut supplementary_groups = vec![0 as libc::gid_t; group_count as usize];
    if group_count > 0
        && unsafe { libc::getgroups(group_count, supplementary_groups.as_mut_ptr()) } != group_count
    {
        return Err(std::io::Error::last_os_error().into());
    }
    supplementary_groups.sort_unstable();
    supplementary_groups.dedup();
    Ok(KernelCredentialsV1 {
        effective_gid: unsafe { libc::getegid() },
        effective_uid: unsafe { libc::geteuid() },
        pid: unsafe { libc::getpid() },
        real_gid: unsafe { libc::getgid() },
        real_uid: unsafe { libc::getuid() },
        supplementary_groups,
    })
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ChildMutationMessageV1 {
    credentials_after: KernelCredentialsV1,
    credentials_before: KernelCredentialsV1,
    observed_errno: i32,
}

fn child_as_producer_negative<F>(
    operation: &str,
    gid: u32,
    uid: u32,
    action: F,
) -> Result<ErrnoNegativeV1, AcceptanceError>
where
    F: FnOnce() -> i32,
{
    let (read_end, mut write_end) = pipe_pair()?;
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if pid == 0 {
        drop(read_end);
        if unsafe { libc::setgroups(0, std::ptr::null()) } != 0
            || unsafe { libc::setgid(gid) } != 0
            || unsafe { libc::setuid(uid) } != 0
        {
            unsafe { libc::_exit(98) };
        }
        let before = match kernel_credentials() {
            Ok(credentials) => credentials,
            Err(_) => unsafe { libc::_exit(97) },
        };
        let observed_errno = action();
        let after = match kernel_credentials() {
            Ok(credentials) => credentials,
            Err(_) => unsafe { libc::_exit(96) },
        };
        let message = ChildMutationMessageV1 {
            credentials_after: after,
            credentials_before: before,
            observed_errno,
        };
        let bytes = match canonical_json(&message) {
            Ok(bytes) if bytes.len() <= 4096 => bytes,
            _ => unsafe { libc::_exit(95) },
        };
        let exit = if write_end.write_all(&bytes).is_ok() {
            0
        } else {
            99
        };
        unsafe { libc::_exit(exit) };
    }
    drop(write_end);
    let mut bytes = Vec::new();
    read_end.take(4097).read_to_end(&mut bytes)?;
    let mut wait_status = 0;
    if unsafe { libc::waitpid(pid, &mut wait_status, 0) } != pid || wait_status != 0 {
        return Err(invalid("producer mutation probe did not exit cleanly"));
    }
    if bytes.is_empty() || bytes.len() > 4096 {
        return Err(invalid("producer mutation probe message is not bounded"));
    }
    let message: ChildMutationMessageV1 = serde_json::from_slice(&bytes)
        .map_err(|error| invalid(format!("producer mutation probe is malformed: {error}")))?;
    if canonical_json(&message)? != bytes {
        return Err(invalid("producer mutation probe is not canonical JSON"));
    }
    Ok(ErrnoNegativeV1 {
        credentials_after: message.credentials_after,
        credentials_before: message.credentials_before,
        child_wait_status: Some(wait_status),
        observed_errno: message.observed_errno,
        operation: operation.to_string(),
    })
}

fn root_negative<F>(operation: &str, action: F) -> Result<ErrnoNegativeV1, AcceptanceError>
where
    F: FnOnce() -> i32,
{
    let credentials_before = kernel_credentials()?;
    let observed_errno = action();
    let credentials_after = kernel_credentials()?;
    Ok(ErrnoNegativeV1 {
        credentials_after,
        credentials_before,
        child_wait_status: None,
        observed_errno,
        operation: operation.to_string(),
    })
}

fn create_errno(path: &Path) -> i32 {
    let path = match c_path(path) {
        Ok(path) => path,
        Err(_) => return libc::EINVAL,
    };
    unsafe { *libc::__error() = 0 };
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_CLOEXEC,
            0o600,
        )
    };
    if fd >= 0 {
        unsafe { libc::close(fd) };
        return 0;
    }
    unsafe { *libc::__error() }
}

fn readonly_mutation_errno(operation: &str, mountpoint: &Path) -> i32 {
    let payload = mountpoint.join("payload");
    let alternate = mountpoint.join("payload-renamed");
    let created = mountpoint.join(format!("negative-{operation}"));
    let payload_c = c_path(&payload).ok();
    let alternate_c = c_path(&alternate).ok();
    unsafe { *libc::__error() = 0 };
    let rc = match operation {
        "create" => return create_errno(&created),
        "truncate" => payload_c
            .as_ref()
            .map_or(-1, |path| unsafe { libc::truncate(path.as_ptr(), 0) }),
        "write" => {
            let fd = payload_c.as_ref().map_or(-1, |path| unsafe {
                libc::open(path.as_ptr(), libc::O_WRONLY | libc::O_CLOEXEC)
            });
            if fd < 0 {
                -1
            } else {
                let rc = unsafe { libc::pwrite(fd, b"x".as_ptr().cast(), 1, 0) as i32 };
                unsafe { libc::close(fd) };
                rc
            }
        }
        "chmod" => {
            let fd = payload_c.as_ref().map_or(-1, |path| unsafe {
                libc::open(
                    path.as_ptr(),
                    libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
                )
            });
            if fd < 0 {
                -1
            } else {
                let rc = unsafe { libc::fchmod(fd, 0o400) };
                unsafe { libc::close(fd) };
                rc
            }
        }
        "rename" => match (payload_c.as_ref(), alternate_c.as_ref()) {
            (Some(from), Some(to)) => unsafe { libc::rename(from.as_ptr(), to.as_ptr()) },
            _ => -1,
        },
        "unlink" => payload_c
            .as_ref()
            .map_or(-1, |path| unsafe { libc::unlink(path.as_ptr()) }),
        "setxattr" => payload_c.as_ref().map_or(-1, |path| {
            let name = b"com.hepta.readonly-negative\0";
            unsafe {
                libc::setxattr(
                    path.as_ptr(),
                    name.as_ptr().cast(),
                    b"x".as_ptr().cast(),
                    1,
                    0,
                    0,
                )
            }
        }),
        _ => return libc::EINVAL,
    };
    if rc >= 0 {
        0
    } else {
        unsafe { *libc::__error() }
    }
}

fn stable_file_sha256(path: &Path) -> Result<String, AcceptanceError> {
    use sha2::Digest;
    use sha2::Sha256;

    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)?;
    let before = binding_from_metadata(&file.metadata()?);
    if before.nlink != 1 || !fs::symlink_metadata(path)?.is_file() {
        return Err(invalid(
            "stream-hashed artifact is not a unique regular file",
        ));
    }
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 128 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let fd_after = binding_from_metadata(&file.metadata()?);
    let path_after = binding(path)?;
    if before != fd_after || before != path_after {
        return Err(invalid(
            "stream-hashed artifact changed during descriptor replay",
        ));
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn volume_state_digest(mountpoint: &Path) -> Result<String, AcceptanceError> {
    #[derive(Serialize)]
    #[serde(deny_unknown_fields)]
    struct VolumeState<'a> {
        payload_binding: ObjectBindingV1,
        payload_sha256: String,
        schema: &'a str,
        sentinel_binding: ObjectBindingV1,
    }
    let payload = mountpoint.join("payload");
    let state = VolumeState {
        payload_binding: binding(&payload)?,
        payload_sha256: stable_file_sha256(&payload)?,
        schema: "hepta_mac_apfs_volume_state_v1",
        sentinel_binding: binding(&mountpoint.join("owner-sentinel"))?,
    };
    Ok(sha256(&canonical_json(&state)?))
}

fn create_volume_payload(
    mountpoint: &Path,
    producer_uid: u32,
    producer_gid: u32,
) -> Result<(), AcceptanceError> {
    let payload_path = mountpoint.join("payload");
    let mut payload = OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(&payload_path)?;
    payload.set_len(4096)?;
    payload.seek(SeekFrom::Start(0))?;
    payload.write_all(b"hepta disposable APFS holder payload\n")?;
    if unsafe { libc::fchown(payload.as_raw_fd(), producer_uid, producer_gid) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    payload.sync_all()?;

    let sentinel_path = mountpoint.join("owner-sentinel");
    let mut sentinel = OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(&sentinel_path)?;
    sentinel.write_all(b"ownership enabled\n")?;
    if unsafe { libc::fchown(sentinel.as_raw_fd(), producer_uid, producer_gid) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    sentinel.sync_all()?;
    File::open(mountpoint)?.sync_all()?;
    Ok(())
}

fn append_new_commands(
    commands: &[CommandReceiptV1],
    appended: &mut usize,
    writer: &mut ReceiptWriter,
) -> Result<(), AcceptanceError> {
    while *appended < commands.len() {
        let receipt = commands[*appended].clone();
        let label = receipt.label.clone();
        writer.append(&label, NativeFactV1::Command(receipt))?;
        *appended += 1;
    }
    Ok(())
}

fn create_image(
    staging: &Path,
    sequence: &mut usize,
    volume_name: &str,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<(), AcceptanceError> {
    require_label(volume_name)?;
    let image = staging.join(IMAGE_NAME);
    if fs::symlink_metadata(&image).is_ok() {
        return Err(invalid("disposable APFS image name already exists"));
    }
    let arguments = vec![
        OsString::from("create"),
        OsString::from("-size"),
        OsString::from(IMAGE_BYTES),
        OsString::from("-type"),
        OsString::from("UDRW"),
        OsString::from("-fs"),
        OsString::from("APFSX"),
        OsString::from("-volname"),
        OsString::from(volume_name),
        image.as_os_str().to_os_string(),
    ];
    if arguments.iter().any(|argument| argument == "-ov") {
        return Err(invalid("hdiutil create may not replace an existing image"));
    }
    let receipt = run_command(
        staging,
        *sequence,
        "create-disposable-image",
        Path::new(HDIUTIL),
        &arguments,
    )?;
    *sequence += 1;
    if receipt.exit_code != 0 {
        return Err(invalid("hdiutil create failed"));
    }
    commands.push(receipt);
    sync_file(&image)?;
    File::open(staging)?.sync_all()?;
    Ok(())
}

fn inspect_launchd(
    staging: &Path,
    sequence: &mut usize,
    commands: &mut Vec<CommandReceiptV1>,
) -> Result<(), AcceptanceError> {
    let arguments = vec![OsString::from("print"), OsString::from("system")];
    let receipt = run_command(
        staging,
        *sequence,
        "launchd-system-inspection",
        Path::new(LAUNCHCTL),
        &arguments,
    )?;
    *sequence += 1;
    if receipt.exit_code != 0 || receipt.stdout_size == 0 {
        return Err(invalid("read-only launchd system inspection failed"));
    }
    commands.push(receipt);
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutedApfsFixtureV1 {
    pub authority_granted: bool,
    pub operation_nonce: String,
    pub publication: SealedPublicationV1,
    pub result_sha256: String,
    pub schema: String,
    pub scope: String,
}

/// Execute and seal one disposable APFS-image mechanism fixture.
///
/// The only way to call this mutating path is with the opaque value returned
/// by `authorize_broker_native_execution`.  It never targets the real T5 APFS
/// container: the mounted filesystem is always a newly-created UDRW image and
/// every unmount/detach is non-forced.  The result and broker publication both
/// carry zero authority.
pub fn execute_and_seal_disposable_fixture(
    authorization: BrokerNativeExecution<'_>,
) -> Result<ExecutedApfsFixtureV1, AcceptanceError> {
    if unsafe { libc::geteuid() } != 0
        || unsafe { libc::getegid() } != 0
        || !authorization.policy.is_privileged_qualification_mode()
        || !authorization
            .policy
            .validates_authenticated_peer(authorization.peer)
        || authorization.policy.target_producer_uid() != LIVE_PRODUCER_UID
        || authorization.policy.target_producer_gid() != LIVE_PRODUCER_GID
    {
        return Err(invalid(
            "broker-native execution lost its fixed root/_hepta identity",
        ));
    }
    verify_canonical_directory(
        &authorization.namespace,
        0,
        0,
        0o700,
        "fixture publication namespace",
    )?;
    verify_t5_descriptor(&authorization.namespace)?;
    if !nested_mounts_below(&authorization.namespace)?.is_empty() {
        return Err(invalid(
            "fixture publication namespace gained a nested mount before execution",
        ));
    }
    let helper_path = Path::new(authorization.policy.helper_executable_path());
    let helper_now = file_identity(helper_path)?;
    validate_root_owned_executable(&helper_now, authorization.policy.helper_executable_sha256())?;
    if helper_now != authorization.helper
        || std::env::current_exe()?.as_os_str().as_bytes() != helper_path.as_os_str().as_bytes()
    {
        return Err(invalid(
            "broker helper path, inode, metadata, or bytes changed after authorization",
        ));
    }
    let staging_name = format!(".incoming-apfs-fixture-{}", authorization.operation_nonce);
    let final_name = format!("apfs-fixture-{}", authorization.operation_nonce);
    if !staging_name
        .strip_prefix(".incoming-apfs-fixture-")
        .is_some_and(|nonce| nonce == authorization.operation_nonce)
    {
        return Err(invalid(
            "fixture staging name is outside the fixed protocol",
        ));
    }
    require_label(&final_name)?;
    let staging = authorization.namespace.join(&staging_name);
    let final_path = authorization.namespace.join(&final_name);
    if fs::symlink_metadata(&staging).is_ok() || fs::symlink_metadata(&final_path).is_ok() {
        return Err(invalid(
            "fixture staging or final no-replace name is already occupied",
        ));
    }
    fs::create_dir(&staging)?;
    fs::set_permissions(&staging, fs::Permissions::from_mode(0o700))?;
    for name in [LOGS_NAME, TOOLS_NAME, "raw", MOUNTPOINT_NAME] {
        fs::create_dir(staging.join(name))?;
        fs::set_permissions(staging.join(name), fs::Permissions::from_mode(0o700))?;
    }
    File::open(&staging)?.sync_all()?;
    File::open(&authorization.namespace)?.sync_all()?;

    let source_before = observe_descriptor(&authorization.source)?;
    let boot_session_uuid = boot_session_uuid()?;
    let monotonic_nanoseconds = monotonic_nanoseconds()?;
    let mut epoch = OperationEpochV1 {
        boot_session_uuid,
        challenge_sha256: String::new(),
        helper_executable: authorization.helper.clone(),
        monotonic_nanoseconds,
        mount_parent_before: binding(&staging)?,
        mountpoint_underlying_before: binding(&staging.join(MOUNTPOINT_NAME))?,
        operation_nonce: authorization.operation_nonce.clone(),
        schema: "hepta_mac_apfs_operation_epoch_v1".to_string(),
        source_before,
    };
    epoch.challenge_sha256 = challenge_for_epoch(&epoch)?;
    let mut writer = ReceiptWriter::new(&staging, &epoch)?;
    let epoch_receipt_sha256 = writer.epoch_receipt_sha256.clone();

    let mut sequence = 0_usize;
    let mut commands = Vec::new();
    let mut appended_commands = 0_usize;
    let volume_name = format!("hepta-apfs-{}", &authorization.operation_nonce[..16]);
    create_image(&staging, &mut sequence, &volume_name, &mut commands)?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;

    let image = staging.join(IMAGE_NAME);
    let mountpoint = staging.join(MOUNTPOINT_NAME);
    let pre_attach_inventory =
        collect_disk_inventory(&staging, &mut sequence, "pre-attach", &mut commands)?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    let obligation = AttachmentObligationJournal::create(
        &authorization.namespace,
        &authorization.operation_nonce,
        &epoch,
        &epoch_receipt_sha256,
        AttachmentObligationEventV1::Prepared {
            image_backing: file_identity(&image)?,
            mountpoint_underlying: epoch.mountpoint_underlying_before.clone(),
            nested_mounts_before: nested_mounts_below(&authorization.namespace)?,
            namespace_statfs: statfs_facts(&authorization.namespace)?,
            pre_attach_inventory: pre_attach_inventory.clone(),
        },
    )?;
    let mut attachment_guard = AttachmentGuard::new(
        obligation,
        pre_attach_inventory.clone(),
        image.clone(),
        mountpoint.clone(),
        epoch.mountpoint_underlying_before.clone(),
    );
    attachment_guard.attach_started(MountPhaseV1::ReadWrite)?;
    let attached_rw = attach_image(
        &staging,
        &mut sequence,
        &image,
        false,
        &volume_name,
        &pre_attach_inventory,
        &mut commands,
    )?;
    let rw_topology = attached_rw
        .topology
        .as_ref()
        .ok_or_else(|| invalid("read-write attach omitted its proven topology"))?
        .clone();
    attachment_guard.attached(MountPhaseV1::ReadWrite, &rw_topology)?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    writer.append(
        "attached-topology-read-write",
        NativeFactV1::AttachedTopology {
            phase: MountPhaseV1::ReadWrite,
            topology: rw_topology,
        },
    )?;
    attachment_guard.mount_started(MountPhaseV1::ReadWrite, &attached_rw.volume_identifier)?;
    mount_volume(
        &staging,
        &mut sequence,
        &mountpoint,
        &attached_rw.volume_identifier,
        false,
        "mount-read-write-initial",
        &mut commands,
    )?;
    attachment_guard.mounted(MountPhaseV1::ReadWrite, statfs_facts(&mountpoint)?)?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    create_volume_payload(&mountpoint, LIVE_PRODUCER_UID, LIVE_PRODUCER_GID)?;
    let rw_facts = mounted_facts(
        &staging,
        &mut sequence,
        &mountpoint,
        &attached_rw,
        LIVE_PRODUCER_UID,
        LIVE_PRODUCER_GID,
        "disk-info-read-write",
        &mut commands,
    )?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    validate_mount_flags(&rw_facts, false)?;
    writer.append(
        "mount-facts-read-write",
        NativeFactV1::Mount {
            facts: rw_facts.clone(),
            phase: MountPhaseV1::ReadWrite,
        },
    )?;
    inspect_launchd(&staging, &mut sequence, &mut commands)?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;

    let baseline_statfs = statfs_facts(&mountpoint)?;
    attachment_guard.unmount_started(MountPhaseV1::ReadWrite)?;
    let baseline = raw_unmount(&mountpoint)?;
    attachment_guard.unmounted(MountPhaseV1::ReadWrite, &baseline)?;
    writer.append(
        "raw-unmount-baseline",
        NativeFactV1::RawUnmount {
            mountpoint_statfs_before: baseline_statfs,
            phase: UnmountPhaseV1::Baseline,
            receipt: baseline,
        },
    )?;

    for (kind, label, mount_label) in [
        (
            HolderKindV1::ReadWriteFd,
            "holder-read-write-fd",
            "mount-read-write-for-fd-holder",
        ),
        (
            HolderKindV1::SharedWritableMappingOnly,
            "holder-shared-writable-mapping-only",
            "mount-read-write-for-mapping-holder",
        ),
        (
            HolderKindV1::CurrentWorkingDirectoryOnly,
            "holder-current-working-directory-only",
            "mount-read-write-for-cwd-holder",
        ),
    ] {
        attachment_guard.mount_started(MountPhaseV1::ReadWrite, &attached_rw.volume_identifier)?;
        mount_volume(
            &staging,
            &mut sequence,
            &mountpoint,
            &attached_rw.volume_identifier,
            false,
            mount_label,
            &mut commands,
        )?;
        attachment_guard.mounted(MountPhaseV1::ReadWrite, statfs_facts(&mountpoint)?)?;
        append_new_commands(&commands, &mut appended_commands, &mut writer)?;
        attachment_guard.unmount_started(MountPhaseV1::ReadWrite)?;
        let cycle = holder_cycle(kind, &mountpoint)?;
        attachment_guard.unmounted(MountPhaseV1::ReadWrite, &cycle.clean_unmount_after_release)?;
        writer.append(label, NativeFactV1::HolderCycle(cycle))?;
    }

    let gap_path = mountpoint.join("producer-gap-negative");
    let gap_negative = child_as_producer_negative(
        "unmounted_gap_create",
        LIVE_PRODUCER_GID,
        LIVE_PRODUCER_UID,
        || create_errno(&gap_path),
    )?;
    writer.append(
        "mutation-unmounted-gap-producer-create",
        NativeFactV1::MutationNegative(gap_negative),
    )?;

    attachment_guard.detach_started(MountPhaseV1::ReadWrite, &attached_rw.whole_disk_identifier)?;
    detach_image(
        &staging,
        &mut sequence,
        &attached_rw,
        "detach-read-write-nonforced",
        &mut commands,
    )?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    let rw_terminal = confirm_disk_arbitration_terminal(
        &staging,
        &authorization.namespace,
        &mut sequence,
        MountPhaseV1::ReadWrite,
        &attached_rw,
        &pre_attach_inventory,
        &epoch.mountpoint_underlying_before,
        &mut commands,
    )?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    attachment_guard.disk_arbitration_gone(MountPhaseV1::ReadWrite, &rw_terminal)?;
    writer.append(
        "disk-arbitration-terminal-read-write",
        NativeFactV1::DiskArbitrationTerminal {
            phase: MountPhaseV1::ReadWrite,
            terminal: rw_terminal,
        },
    )?;
    let image_before_ro = stable_file_sha256(&image)?;
    writer.append(
        "image-before-read-only-attach",
        NativeFactV1::ImageDigest {
            phase: ImageDigestPhaseV1::BeforeReadOnlyAttach,
            sha256: image_before_ro,
        },
    )?;

    attachment_guard.attach_started(MountPhaseV1::ReadOnly)?;
    let attached_ro = attach_image(
        &staging,
        &mut sequence,
        &image,
        true,
        &volume_name,
        &pre_attach_inventory,
        &mut commands,
    )?;
    let ro_topology = attached_ro
        .topology
        .as_ref()
        .ok_or_else(|| invalid("read-only attach omitted its proven topology"))?
        .clone();
    attachment_guard.attached(MountPhaseV1::ReadOnly, &ro_topology)?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    writer.append(
        "attached-topology-read-only",
        NativeFactV1::AttachedTopology {
            phase: MountPhaseV1::ReadOnly,
            topology: ro_topology,
        },
    )?;
    if attached_ro.volume_uuid != attached_rw.volume_uuid
        || attached_ro.apfs_container_uuid != attached_rw.apfs_container_uuid
    {
        return Err(invalid(
            "read-only attachment changed APFS volume or container UUID",
        ));
    }
    attachment_guard.mount_started(MountPhaseV1::ReadOnly, &attached_ro.volume_identifier)?;
    mount_volume(
        &staging,
        &mut sequence,
        &mountpoint,
        &attached_ro.volume_identifier,
        true,
        "mount-read-only",
        &mut commands,
    )?;
    attachment_guard.mounted(MountPhaseV1::ReadOnly, statfs_facts(&mountpoint)?)?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    let ro_facts = mounted_facts(
        &staging,
        &mut sequence,
        &mountpoint,
        &attached_ro,
        LIVE_PRODUCER_UID,
        LIVE_PRODUCER_GID,
        "disk-info-read-only",
        &mut commands,
    )?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    validate_mount_flags(&ro_facts, true)?;
    writer.append(
        "mount-facts-read-only",
        NativeFactV1::Mount {
            facts: ro_facts,
            phase: MountPhaseV1::ReadOnly,
        },
    )?;

    let before_negatives = volume_state_digest(&mountpoint)?;
    writer.append(
        "volume-state-before-read-only-negatives",
        NativeFactV1::VolumeStateDigest {
            phase: VolumeStatePhaseV1::BeforeReadOnlyNegatives,
            sha256: before_negatives,
        },
    )?;
    for uid in [0, LIVE_PRODUCER_UID] {
        for operation in [
            "create", "truncate", "write", "chmod", "rename", "unlink", "setxattr",
        ] {
            let negative = if uid == 0 {
                root_negative(&format!("read_only_{operation}"), || {
                    readonly_mutation_errno(operation, &mountpoint)
                })?
            } else {
                child_as_producer_negative(
                    &format!("read_only_{operation}"),
                    LIVE_PRODUCER_GID,
                    LIVE_PRODUCER_UID,
                    || readonly_mutation_errno(operation, &mountpoint),
                )?
            };
            writer.append(
                &format!("mutation-read-only-{uid}-{operation}"),
                NativeFactV1::MutationNegative(negative),
            )?;
        }
    }
    let after_negatives = volume_state_digest(&mountpoint)?;
    writer.append(
        "volume-state-after-read-only-negatives",
        NativeFactV1::VolumeStateDigest {
            phase: VolumeStatePhaseV1::AfterReadOnlyNegatives,
            sha256: after_negatives,
        },
    )?;
    let ro_unmount_statfs = statfs_facts(&mountpoint)?;
    attachment_guard.unmount_started(MountPhaseV1::ReadOnly)?;
    let ro_unmount = raw_unmount(&mountpoint)?;
    attachment_guard.unmounted(MountPhaseV1::ReadOnly, &ro_unmount)?;
    writer.append(
        "raw-unmount-read-only-final",
        NativeFactV1::RawUnmount {
            mountpoint_statfs_before: ro_unmount_statfs,
            phase: UnmountPhaseV1::ReadOnlyFinal,
            receipt: ro_unmount,
        },
    )?;
    attachment_guard.detach_started(MountPhaseV1::ReadOnly, &attached_ro.whole_disk_identifier)?;
    detach_image(
        &staging,
        &mut sequence,
        &attached_ro,
        "detach-read-only-nonforced",
        &mut commands,
    )?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    let ro_terminal = confirm_disk_arbitration_terminal(
        &staging,
        &authorization.namespace,
        &mut sequence,
        MountPhaseV1::ReadOnly,
        &attached_ro,
        &pre_attach_inventory,
        &epoch.mountpoint_underlying_before,
        &mut commands,
    )?;
    append_new_commands(&commands, &mut appended_commands, &mut writer)?;
    attachment_guard.disk_arbitration_gone(MountPhaseV1::ReadOnly, &ro_terminal)?;
    writer.append(
        "disk-arbitration-terminal-read-only",
        NativeFactV1::DiskArbitrationTerminal {
            phase: MountPhaseV1::ReadOnly,
            terminal: ro_terminal.clone(),
        },
    )?;
    let attachment_obligation_directory = attachment_guard.journal.relative_directory();
    let attachment_obligation_terminal_sha256 =
        attachment_guard.finish(ro_terminal.post_inventory.clone())?;
    let image_after_ro = stable_file_sha256(&image)?;
    writer.append(
        "image-after-read-only-detach",
        NativeFactV1::ImageDigest {
            phase: ImageDigestPhaseV1::AfterReadOnlyDetach,
            sha256: image_after_ro,
        },
    )?;

    let source_after = observe_descriptor(&authorization.source)?;
    writer.append(
        "source-after",
        NativeFactV1::Source {
            observation: source_after,
            phase: SourcePhaseV1::After,
        },
    )?;
    writer.append(
        "terminal-detached",
        NativeFactV1::Terminal {
            final_detached: true,
            mount_parent_after: binding(&staging)?,
            mountpoint_underlying_after: binding(&mountpoint)?,
        },
    )?;

    let result = ApfsFixtureResultV1 {
        aggregate_authority: false,
        attachment_obligation_directory,
        attachment_obligation_terminal_sha256,
        cutover_authority: false,
        deletion_authority: false,
        epoch_receipt_path: "raw/000-epoch.json".to_string(),
        epoch_receipt_sha256,
        execution_kind: "broker_native_disposable_apfs_image_only".to_string(),
        operation_nonce: authorization.operation_nonce.clone(),
        production_authority: false,
        raw_receipts: writer.receipts,
        refs_authority: false,
        remote_authority: false,
        schema: SCHEMA.to_string(),
        scope: "privileged_mechanism_only_no_migration_authority".to_string(),
    };
    let result_bytes = canonical_json(&result)?;
    let result_sha256 = sha256(&result_bytes);
    write_new(&staging.join(RESULT_NAME), &result_bytes, 0o400)?;
    seal_inner_tree(&staging, &result)?;
    let replay = prepared_tree_replay_sha256(&staging, authorization.policy)?;
    let publication = qualify_prepared_directory(
        &authorization.namespace,
        &staging_name,
        &final_name,
        &replay,
        &authorization.operation_nonce,
        authorization.peer,
        authorization.policy,
    )?;
    if publication.qualification_receipt.live_authority
        || publication.qualification_receipt.aggregate_authority
        || publication.qualification_receipt.cutover_authority
        || publication.qualification_receipt.deletion_authority
        || publication.qualification_receipt.production_authority
        || publication.qualification_receipt.refs_authority
        || publication.qualification_receipt.remote_authority
        || publication.publication_receipt.authority_granted
    {
        return Err(invalid("broker publication unexpectedly granted authority"));
    }
    Ok(ExecutedApfsFixtureV1 {
        authority_granted: false,
        operation_nonce: authorization.operation_nonce,
        publication,
        result_sha256,
        schema: "hepta_mac_executed_apfs_fixture_v1".to_string(),
        scope: "disposable_privileged_mechanism_only".to_string(),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum InventoryKind {
    Directory,
    Regular,
}

#[derive(Clone, Debug)]
struct InventoryEntry {
    binding: ObjectBindingV1,
    kind: InventoryKind,
    path: String,
}

fn relative_utf8(path: &Path, root: &Path) -> Result<String, AcceptanceError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| invalid("inventory path escaped its root"))?;
    if relative.as_os_str().is_empty() {
        return Ok(".".to_string());
    }
    let value = relative
        .to_str()
        .ok_or_else(|| invalid("inventory path is not UTF-8"))?
        .to_string();
    validate_relative_path(&value)?;
    Ok(value)
}

fn validate_relative_path(path: &str) -> Result<(), AcceptanceError> {
    if path.is_empty()
        || path.starts_with('/')
        || path.split('/').any(|component| {
            component.is_empty()
                || component == "."
                || component == ".."
                || component.contains('\0')
        })
    {
        return Err(invalid("fixture receipt path is not a safe relative path"));
    }
    Ok(())
}

fn scan_inventory(root: &Path) -> Result<BTreeMap<String, InventoryEntry>, AcceptanceError> {
    fn visit(
        root: &Path,
        current: &Path,
        depth: usize,
        entries: &mut BTreeMap<String, InventoryEntry>,
    ) -> Result<(), AcceptanceError> {
        if depth > 8 || entries.len() > 65_536 {
            return Err(invalid("fixture inventory exceeds its depth or node bound"));
        }
        let metadata = fs::symlink_metadata(current)?;
        let kind = if metadata.is_dir() {
            InventoryKind::Directory
        } else if metadata.is_file() && metadata.nlink() == 1 {
            InventoryKind::Regular
        } else {
            return Err(invalid(format!(
                "fixture inventory contains a symlink, special node, or hardlink: {}",
                current.display()
            )));
        };
        let path = relative_utf8(current, root)?;
        if entries
            .insert(
                path.clone(),
                InventoryEntry {
                    binding: binding_from_metadata(&metadata),
                    kind: kind.clone(),
                    path,
                },
            )
            .is_some()
        {
            return Err(invalid("fixture inventory contains a duplicate path"));
        }
        if kind == InventoryKind::Directory {
            let mut children = fs::read_dir(current)?.collect::<Result<Vec<_>, _>>()?;
            children.sort_by_key(|entry| entry.file_name());
            for child in children {
                visit(root, &child.path(), depth + 1, entries)?;
            }
        }
        Ok(())
    }

    let mut entries = BTreeMap::new();
    visit(root, root, 0, &mut entries)?;
    Ok(entries)
}

fn expected_mode_for(path: &str, kind: &InventoryKind) -> u32 {
    match kind {
        InventoryKind::Regular => 0o400,
        InventoryKind::Directory if path == MOUNTPOINT_NAME => 0o700,
        InventoryKind::Directory => 0o500,
    }
}

fn render_modes(entries: &BTreeMap<String, InventoryEntry>) -> Vec<u8> {
    let mut output = String::new();
    for entry in entries.values() {
        let kind = match entry.kind {
            InventoryKind::Directory => "directory",
            InventoryKind::Regular => "regular",
        };
        output.push_str(&format!(
            "{kind}\t{:04o}\t{}\n",
            expected_mode_for(&entry.path, &entry.kind),
            entry.path
        ));
    }
    output.into_bytes()
}

fn sync_tree(root: &Path) -> Result<(), AcceptanceError> {
    let entries = scan_inventory(root)?;
    for entry in entries.values().rev() {
        File::open(root.join(&entry.path))
            .or_else(|error| {
                if entry.path == "." {
                    File::open(root)
                } else {
                    Err(error)
                }
            })?
            .sync_all()?;
    }
    Ok(())
}

fn seal_inner_tree(staging: &Path, _result: &ApfsFixtureResultV1) -> Result<(), AcceptanceError> {
    if fs::symlink_metadata(staging.join(MODES_NAME)).is_ok()
        || fs::symlink_metadata(staging.join(SHA256SUMS_NAME)).is_ok()
    {
        return Err(invalid(
            "fixture inventory names already exist before no-replace seal",
        ));
    }
    let before = scan_inventory(staging)?;
    for entry in before.values() {
        if entry.kind == InventoryKind::Regular {
            fs::set_permissions(
                if entry.path == "." {
                    staging.to_path_buf()
                } else {
                    staging.join(&entry.path)
                },
                fs::Permissions::from_mode(0o400),
            )?;
        }
    }
    let mut predicted = scan_inventory(staging)?;
    for name in [MODES_NAME, SHA256SUMS_NAME] {
        predicted.insert(
            name.to_string(),
            InventoryEntry {
                binding: ObjectBindingV1 {
                    ctime_nanoseconds: 0,
                    ctime_seconds: 0,
                    dev: 0,
                    flags: 0,
                    gid: 0,
                    inode: 0,
                    mode: 0o400,
                    mtime_nanoseconds: 0,
                    mtime_seconds: 0,
                    nlink: 1,
                    size: 0,
                    uid: 0,
                },
                kind: InventoryKind::Regular,
                path: name.to_string(),
            },
        );
    }
    let modes = render_modes(&predicted);
    write_new(&staging.join(MODES_NAME), &modes, 0o400)?;

    let with_modes = scan_inventory(staging)?;
    let mut sums = String::new();
    for entry in with_modes.values() {
        if entry.kind == InventoryKind::Regular && entry.path != SHA256SUMS_NAME {
            let digest = stable_file_sha256(&staging.join(&entry.path))?;
            sums.push_str(&format!("{digest}  {}\n", entry.path));
        }
    }
    write_new(&staging.join(SHA256SUMS_NAME), sums.as_bytes(), 0o400)?;

    let sealed = scan_inventory(staging)?;
    if render_modes(&sealed) != modes {
        return Err(invalid(
            "fixture tree changed while materializing MODES.tsv",
        ));
    }
    for entry in sealed.values() {
        let path = if entry.path == "." {
            staging.to_path_buf()
        } else {
            staging.join(&entry.path)
        };
        fs::set_permissions(
            path,
            fs::Permissions::from_mode(expected_mode_for(&entry.path, &entry.kind)),
        )?;
    }
    sync_tree(staging)?;
    Ok(())
}

fn read_canonical<T: for<'de> Deserialize<'de> + Serialize>(
    path: &Path,
    label: &str,
) -> Result<(T, Vec<u8>), AcceptanceError> {
    let bytes = read_bounded(path, MAX_ARTIFACT_BYTES)?;
    let parsed: T = serde_json::from_slice(&bytes)
        .map_err(|error| invalid(format!("{label} is malformed JSON: {error}")))?;
    if canonical_json(&parsed)? != bytes {
        return Err(invalid(format!("{label} is not canonical JSON")));
    }
    Ok((parsed, bytes))
}

fn replay_attachment_obligation_records(
    records: &[(AttachmentObligationRecordV1, Vec<u8>)],
    current_boot_session_uuid: &str,
) -> Result<AttachmentObligationVerificationV1, AcceptanceError> {
    if records.is_empty() {
        return Err(invalid("attachment obligation contains no records"));
    }
    let first = &records[0].0;
    require_nonce(&first.operation_nonce)?;
    require_uuid(&first.boot_session_uuid, "obligation boot session")?;
    require_digest(&first.challenge_sha256, "obligation challenge")?;
    require_digest(&first.epoch_receipt_sha256, "obligation epoch")?;
    let mut state = ObligationState::empty();
    let mut previous = None;
    let mut baseline = None;
    let mut prepared_image = None;
    let mut read_write_topology = None;
    for (index, (record, bytes)) in records.iter().enumerate() {
        let expected_sequence = (index + 1) as u32;
        if record.schema != "hepta_mac_attachment_obligation_record_v1"
            || record.authority_granted
            || record.sequence != expected_sequence
            || record.operation_nonce != first.operation_nonce
            || record.boot_session_uuid != first.boot_session_uuid
            || record.challenge_sha256 != first.challenge_sha256
            || record.epoch_receipt_sha256 != first.epoch_receipt_sha256
            || record.previous_record_sha256 != previous
            || canonical_json(record)? != *bytes
        {
            return Err(invalid(
                "attachment obligation record differs from its exact hash-chain envelope",
            ));
        }
        match &record.event {
            AttachmentObligationEventV1::Prepared {
                image_backing,
                mountpoint_underlying,
                nested_mounts_before,
                pre_attach_inventory,
                namespace_statfs,
                ..
            } => {
                validate_disk_inventory(pre_attach_inventory)?;
                require_digest(&image_backing.sha256, "Prepared image backing")?;
                if !Path::new(&image_backing.path).is_absolute()
                    || !Path::new(&image_backing.path).starts_with("/Volumes/T5/")
                    || image_backing.binding.uid != 0
                    || image_backing.binding.gid != 0
                    || image_backing.binding.nlink != 1
                    || mountpoint_underlying.uid != 0
                    || mountpoint_underlying.gid != 0
                    || mountpoint_underlying.mode != 0o700
                    || namespace_statfs.filesystem_type != "apfs"
                    || namespace_statfs.mount_from.is_empty()
                    || namespace_statfs.mount_on != "/Volumes/T5"
                    || namespace_statfs.mount_flags & MNT_IGNORE_OWNERSHIP != 0
                    || !nested_mounts_before.is_empty()
                {
                    return Err(invalid(
                        "obligation Prepared is not bound to non-nested ownership-enabled T5",
                    ));
                }
                baseline = Some(pre_attach_inventory.clone());
                prepared_image = Some(image_backing.clone());
            }
            AttachmentObligationEventV1::Attached { phase, topology } => {
                let baseline = baseline
                    .as_ref()
                    .ok_or_else(|| invalid("Attached precedes Prepared inventory"))?;
                let prepared_image = prepared_image
                    .as_ref()
                    .ok_or_else(|| invalid("Attached precedes Prepared image binding"))?;
                let image_phase_binding_valid = match phase {
                    MountPhaseV1::ReadWrite => topology.image_backing_before == *prepared_image,
                    MountPhaseV1::ReadOnly => read_write_topology.as_ref().is_some_and(
                        |read_write: &AttachedTopologyV1| {
                            same_image_backing_inode(
                                &read_write.image_backing_before,
                                &topology.image_backing_before,
                            ) && read_write.apfs_container_uuid == topology.apfs_container_uuid
                                && read_write.apfs_volume_uuid == topology.apfs_volume_uuid
                        },
                    ),
                };
                if !image_phase_binding_valid
                    || topology.pre_attach_inventory_sha256 != inventory_sha256(baseline)?
                    || [
                        &topology.whole_disk.device_identifier,
                        &topology.physical_store.device_identifier,
                        &topology.apfs_container.device_identifier,
                        &topology.apfs_volume.device_identifier,
                    ]
                    .into_iter()
                    .any(|identifier| {
                        baseline.all_disks.contains(identifier)
                            || identifier == &baseline.t5_device_identifier
                            || identifier == &baseline.t5_parent_whole_disk
                            || identifier == &baseline.t5_apfs_container_reference
                            || identifier == &baseline.t5_physical_store_identifier
                    })
                    || baseline
                        .hdiutil_backing_paths
                        .contains(&topology.image_path_from_hdiutil)
                    || topology.apfs_volume_uuid == baseline.t5_volume_uuid
                {
                    return Err(invalid(
                        "obligation Attached topology aliases T5 or a pre-existing disk",
                    ));
                }
                if *phase == MountPhaseV1::ReadWrite {
                    read_write_topology = Some(topology.clone());
                }
            }
            AttachmentObligationEventV1::DiskArbitrationGone { terminal, .. } => {
                validate_disk_arbitration_terminal(
                    terminal,
                    baseline
                        .as_ref()
                        .ok_or_else(|| invalid("terminal precedes Prepared inventory"))?,
                    match &records[0].0.event {
                        AttachmentObligationEventV1::Prepared {
                            mountpoint_underlying,
                            ..
                        } => mountpoint_underlying,
                        _ => return Err(invalid("first obligation record is not Prepared")),
                    },
                )?;
            }
            AttachmentObligationEventV1::TerminalReconciled { post_inventory } => {
                if !same_inventory_topology(
                    post_inventory,
                    baseline
                        .as_ref()
                        .ok_or_else(|| invalid("terminal precedes Prepared inventory"))?,
                ) {
                    return Err(invalid(
                        "terminal obligation inventory differs from Prepared inventory",
                    ));
                }
            }
            _ => {}
        }
        state.apply(&record.event, record.disposition)?;
        previous = Some(sha256(bytes));
    }
    let terminal_record_sha256 = previous.expect("nonempty obligation");
    let current_boot = first.boot_session_uuid == current_boot_session_uuid;
    let requires_privileged_reconciliation = !current_boot
        || state.disposition != ObligationDispositionV1::Reconciled
        || !state.terminal;
    Ok(AttachmentObligationVerificationV1 {
        authority_granted: false,
        boot_session_uuid: first.boot_session_uuid.clone(),
        current_boot,
        disposition: state.disposition,
        operation_nonce: first.operation_nonce.clone(),
        records: records.len(),
        requires_privileged_reconciliation,
        schema: "hepta_mac_attachment_obligation_verification_v1".to_string(),
        terminal_record_sha256,
    })
}

/// Inspect a durable attachment obligation without granting any authority.
/// Prior-boot, active, reconcile-required, or quarantined chains remain an
/// explicit manual privileged-reconciliation requirement.
pub fn inspect_attachment_obligation(
    namespace: &Path,
    operation_nonce: &str,
) -> Result<AttachmentObligationVerificationV1, AcceptanceError> {
    require_nonce(operation_nonce)?;
    verify_t5_descriptor(namespace)?;
    let directory = namespace.join(format!("{OBLIGATION_PREFIX}{operation_nonce}"));
    verify_canonical_directory(&directory, 0, 0, 0o700, "attachment obligation directory")?;
    let mut entries = fs::read_dir(&directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    let mut records = Vec::new();
    for (index, entry) in entries.into_iter().enumerate() {
        let expected_name = format!("{:08}.json", index + 1);
        if entry.file_name().as_os_str() != std::ffi::OsStr::new(&expected_name) {
            return Err(invalid(
                "attachment obligation directory is not a closed contiguous record set",
            ));
        }
        let metadata = entry.metadata()?;
        if !metadata.is_file()
            || metadata.uid() != 0
            || metadata.gid() != 0
            || metadata.mode() & 0o7777 != 0o400
            || metadata.nlink() != 1
        {
            return Err(invalid(
                "attachment obligation record is not root:wheel 0400 unique regular",
            ));
        }
        records.push(read_canonical::<AttachmentObligationRecordV1>(
            &entry.path(),
            "attachment obligation record",
        )?);
    }
    replay_attachment_obligation_records(&records, &boot_session_uuid()?)
}

fn parse_sums(bytes: &[u8]) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let text = std::str::from_utf8(bytes).map_err(|_| invalid("SHA256SUMS is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(invalid("SHA256SUMS is empty or not newline terminated"));
    }
    let mut sums = BTreeMap::new();
    for line in text.lines() {
        let (digest, path) = line
            .split_once("  ")
            .ok_or_else(|| invalid("SHA256SUMS line is malformed"))?;
        require_digest(digest, "SHA256SUMS entry")?;
        validate_relative_path(path)?;
        if sums.insert(path.to_string(), digest.to_string()).is_some() {
            return Err(invalid("SHA256SUMS contains a duplicate path"));
        }
    }
    Ok(sums)
}

fn verify_manifest_and_modes(
    root: &Path,
    expected_paths: &BTreeSet<String>,
) -> Result<(usize, usize, String), AcceptanceError> {
    let inventory = scan_inventory(root)?;
    let actual_paths = inventory.keys().cloned().collect::<BTreeSet<_>>();
    if &actual_paths != expected_paths {
        return Err(invalid(
            "fixture tree differs from its typed closed-world path set",
        ));
    }
    for entry in inventory.values() {
        if entry.binding.mode != expected_mode_for(&entry.path, &entry.kind)
            || (entry.kind == InventoryKind::Regular && entry.binding.nlink != 1)
        {
            return Err(invalid(format!(
                "fixture node {} differs from its exact mode/link contract",
                entry.path
            )));
        }
    }
    let modes = read_bounded(&root.join(MODES_NAME), MAX_ARTIFACT_BYTES)?;
    if modes != render_modes(&inventory) {
        return Err(invalid("MODES.tsv does not exactly replay the closed tree"));
    }
    let sums_bytes = read_bounded(&root.join(SHA256SUMS_NAME), MAX_ARTIFACT_BYTES)?;
    let sums = parse_sums(&sums_bytes)?;
    let expected_sum_paths = inventory
        .values()
        .filter(|entry| entry.kind == InventoryKind::Regular && entry.path != SHA256SUMS_NAME)
        .map(|entry| entry.path.clone())
        .collect::<BTreeSet<_>>();
    if sums.keys().cloned().collect::<BTreeSet<_>>() != expected_sum_paths {
        return Err(invalid(
            "SHA256SUMS does not cover exactly every non-manifest regular file",
        ));
    }
    for (path, expected) in &sums {
        if stable_file_sha256(&root.join(path))? != *expected {
            return Err(invalid(format!(
                "fixture artifact {path} differs from SHA256SUMS"
            )));
        }
    }
    #[derive(Serialize)]
    struct Replay<'a> {
        modes_sha256: String,
        schema: &'a str,
        sums: &'a BTreeMap<String, String>,
    }
    let replay = sha256(&canonical_json(&Replay {
        modes_sha256: sha256(&modes),
        schema: "hepta_mac_apfs_inner_tree_replay_v1",
        sums: &sums,
    })?);
    Ok((sums.len(), inventory.len(), replay))
}

fn expected_executable_command_labels() -> BTreeSet<String> {
    expected_fact_labels()
        .into_iter()
        .filter(|label| command_tool(label).is_ok())
        .collect()
}

fn append_inventory_labels(labels: &mut Vec<String>, stage: &str) {
    labels.extend([
        format!("disk-list-{stage}"),
        format!("parse-disk-list-{stage}"),
        format!("disk-node-info-t5-{stage}"),
        format!("parse-disk-node-info-t5-{stage}"),
        format!("hdi-info-inventory-{stage}"),
        format!("parse-hdi-info-inventory-{stage}"),
    ]);
}

fn append_attach_labels(labels: &mut Vec<String>, suffix: &str) {
    labels.extend([
        format!("attach-{suffix}"),
        format!("parse-attach-{suffix}"),
        format!("apfs-list-{suffix}"),
        format!("parse-apfs-{suffix}"),
        format!("hdi-info-{suffix}"),
        format!("parse-hdi-info-{suffix}"),
    ]);
    for kind in ["whole", "physical", "container", "volume"] {
        labels.push(format!("disk-node-info-{suffix}-{kind}"));
        labels.push(format!("parse-disk-node-info-{suffix}-{kind}"));
    }
}

fn append_diskarbitration_terminal_labels(labels: &mut Vec<String>, suffix: &str) {
    labels.extend([
        format!("hdi-info-after-detach-{suffix}"),
        format!("parse-hdi-info-after-detach-{suffix}"),
        format!("disk-arbitration-confirm-{suffix}"),
    ]);
    append_inventory_labels(labels, &format!("post-{suffix}-detach"));
}

fn expected_fact_labels() -> Vec<String> {
    let mut labels = strings(&["create-disposable-image"]);
    append_inventory_labels(&mut labels, "pre-attach");
    append_attach_labels(&mut labels, "read-write");
    labels.extend(strings(&[
        "attached-topology-read-write",
        "mount-read-write-initial",
        "disk-info-read-write",
        "parse-disk-info-read-write",
        "mount-facts-read-write",
        "launchd-system-inspection",
        "raw-unmount-baseline",
        "mount-read-write-for-fd-holder",
        "holder-read-write-fd",
        "mount-read-write-for-mapping-holder",
        "holder-shared-writable-mapping-only",
        "mount-read-write-for-cwd-holder",
        "holder-current-working-directory-only",
        "mutation-unmounted-gap-producer-create",
        "detach-read-write-nonforced",
    ]));
    append_diskarbitration_terminal_labels(&mut labels, "read-write");
    labels.extend(strings(&[
        "disk-arbitration-terminal-read-write",
        "image-before-read-only-attach",
    ]));
    append_attach_labels(&mut labels, "read-only");
    labels.extend(strings(&[
        "attached-topology-read-only",
        "mount-read-only",
        "disk-info-read-only",
        "parse-disk-info-read-only",
        "mount-facts-read-only",
        "volume-state-before-read-only-negatives",
    ]));
    for uid in [0, LIVE_PRODUCER_UID] {
        for operation in [
            "create", "truncate", "write", "chmod", "rename", "unlink", "setxattr",
        ] {
            labels.push(format!("mutation-read-only-{uid}-{operation}"));
        }
    }
    labels.extend(strings(&[
        "volume-state-after-read-only-negatives",
        "raw-unmount-read-only-final",
        "detach-read-only-nonforced",
    ]));
    append_diskarbitration_terminal_labels(&mut labels, "read-only");
    labels.extend(strings(&[
        "disk-arbitration-terminal-read-only",
        "image-after-read-only-detach",
        "source-after",
        "terminal-detached",
    ]));
    labels
}

fn command_tool(label: &str) -> Result<&'static str, AcceptanceError> {
    if label.starts_with("parse-") {
        Ok(PLUTIL)
    } else if label.starts_with("attach-")
        || label.starts_with("detach-")
        || label.starts_with("hdi-info-")
        || label == "create-disposable-image"
    {
        Ok(HDIUTIL)
    } else if label.starts_with("apfs-list-")
        || label.starts_with("disk-info-")
        || label.starts_with("disk-list-")
        || label.starts_with("disk-node-info-")
        || label.starts_with("disk-arbitration-confirm-")
        || label.starts_with("mount-")
    {
        Ok(DISKUTIL)
    } else if label == "launchd-system-inspection" {
        Ok(LAUNCHCTL)
    } else {
        Err(invalid(format!("unexpected fixture command label {label}")))
    }
}

fn verify_command_artifacts(
    root: &Path,
    receipt: &CommandReceiptV1,
) -> Result<(), AcceptanceError> {
    require_label(&receipt.label)?;
    let disk_arbitration_negative = receipt.label.starts_with("disk-arbitration-confirm-");
    if (disk_arbitration_negative && receipt.exit_code == 0)
        || (!disk_arbitration_negative && receipt.exit_code != 0)
        || receipt.child_pid <= 1
        || receipt.tool_process_group_id != receipt.child_pid
        || receipt.duration_milliseconds > COMMAND_TIMEOUT.as_millis() as u64
        || receipt.tool_before != receipt.tool_after
        || receipt.tool_ancestor_chain_before != receipt.tool_ancestor_chain_after
        || receipt.tool_before.path != command_tool(&receipt.label)?
    {
        return Err(invalid(format!(
            "command {} does not replay an exact successful root-owned tool invocation",
            receipt.label
        )));
    }
    validate_system_tool_identity(&receipt.tool_before)?;
    if system_tool_ancestor_chain(Path::new(&receipt.tool_before.path))?
        != receipt.tool_ancestor_chain_before
    {
        return Err(invalid(
            "command tool root-owned ancestor pins are no longer current",
        ));
    }
    require_digest(&receipt.stdout_sha256, "command stdout")?;
    require_digest(&receipt.stderr_sha256, "command stderr")?;
    let expected_tool_copy = format!("{TOOLS_NAME}/{}", receipt.tool_before.sha256);
    if receipt.tool_copy_path != expected_tool_copy {
        return Err(invalid("command tool-copy path is not digest addressed"));
    }
    for (stream, path, size, digest) in [
        (
            "stdout",
            &receipt.stdout_path,
            receipt.stdout_size,
            &receipt.stdout_sha256,
        ),
        (
            "stderr",
            &receipt.stderr_path,
            receipt.stderr_size,
            &receipt.stderr_sha256,
        ),
    ] {
        validate_relative_path(path)?;
        if !path.starts_with(&format!("{LOGS_NAME}/"))
            || !path.ends_with(&format!("-{}.{stream}", receipt.label))
        {
            return Err(invalid("command stream path is not fixed by its label"));
        }
        let bytes = read_bounded(&root.join(path), MAX_ARTIFACT_BYTES)?;
        if bytes.len() as u64 != size || sha256(&bytes) != *digest {
            return Err(invalid(format!(
                "command {} {stream} differs from its raw receipt",
                receipt.label
            )));
        }
    }
    let tool_copy = read_bounded(&root.join(&receipt.tool_copy_path), 128 * 1024 * 1024)?;
    if sha256(&tool_copy) != receipt.tool_before.sha256 {
        return Err(invalid(
            "executed tool copy differs from its before/after byte identity",
        ));
    }
    Ok(())
}

fn valid_disk_identifier(value: &str) -> bool {
    value.starts_with("disk")
        && value.len() <= 32
        && value[4..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b's')
        && value[4..].bytes().any(|byte| byte.is_ascii_digit())
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn verify_command_shape(
    root: &Path,
    nonce: &str,
    receipt: &CommandReceiptV1,
    commands: &BTreeMap<String, CommandReceiptV1>,
) -> Result<(), AcceptanceError> {
    let image = root.join(IMAGE_NAME).to_string_lossy().to_string();
    let mountpoint = root.join(MOUNTPOINT_NAME).to_string_lossy().to_string();
    let volume_name = format!("hepta-apfs-{}", &nonce[..16]);
    let args = &receipt.arguments;
    match receipt.label.as_str() {
        "create-disposable-image" => {
            let expected = vec![
                "create".to_string(),
                "-size".to_string(),
                IMAGE_BYTES.to_string(),
                "-type".to_string(),
                "UDRW".to_string(),
                "-fs".to_string(),
                "APFSX".to_string(),
                "-volname".to_string(),
                volume_name,
                image,
            ];
            if args != &expected || args.iter().any(|argument| argument == "-ov") {
                return Err(invalid(
                    "hdiutil create argv is not the exact no-replace UDRW APFSX form",
                ));
            }
        }
        "attach-read-write" => {
            if args != &strings(&["attach", "-owners", "on", "-nomount", "-plist", &image]) {
                return Err(invalid("read-write attach argv is not exact"));
            }
        }
        "attach-read-only" => {
            if args
                != &strings(&[
                    "attach",
                    "-readonly",
                    "-owners",
                    "on",
                    "-nomount",
                    "-plist",
                    &image,
                ])
            {
                return Err(invalid("read-only attach argv is not exact"));
            }
        }
        label if label.starts_with("hdi-info-") => {
            if args != &strings(&["info", "-plist"]) {
                return Err(invalid("hdiutil info argv is not exact"));
            }
        }
        "launchd-system-inspection" => {
            if args != &strings(&["print", "system"]) || receipt.stdout_size == 0 {
                return Err(invalid(
                    "launchd inspection is not the exact read-only system print",
                ));
            }
        }
        label if label.starts_with("parse-") => {
            let input_label = label.strip_prefix("parse-").expect("matched prefix");
            let input = commands
                .get(input_label)
                .ok_or_else(|| invalid("plutil receipt has no exact source command"))?;
            let input_path = root.join(&input.stdout_path).to_string_lossy().to_string();
            if args != &strings(&["-convert", "json", "-o", "-", &input_path])
                || receipt.stdout_size == 0
            {
                return Err(invalid(
                    "plutil conversion argv is not bound to exact raw stdout",
                ));
            }
        }
        label if label.starts_with("apfs-list-") => {
            if args.len() != 4
                || args[0..3] != strings(&["apfs", "list", "-plist"])
                || !valid_disk_identifier(&args[3])
            {
                return Err(invalid("diskutil apfs list argv is malformed"));
            }
        }
        label if label.starts_with("disk-list-") => {
            if args != &strings(&["list", "-plist"]) {
                return Err(invalid("diskutil inventory argv is not exact"));
            }
        }
        label if label.starts_with("disk-node-info-") => {
            if args.len() != 3
                || args[0..2] != strings(&["info", "-plist"])
                || if label.starts_with("disk-node-info-t5-") {
                    args[2] != "/Volumes/T5"
                } else {
                    !valid_disk_identifier(&args[2])
                }
            {
                return Err(invalid("diskutil topology node argv is not exact"));
            }
        }
        label if label.starts_with("disk-arbitration-confirm-") => {
            if args.len() != 3
                || args[0..2] != strings(&["info", "-plist"])
                || !args[2].starts_with("/dev/disk")
                || !valid_disk_identifier(args[2].trim_start_matches("/dev/"))
            {
                return Err(invalid(
                    "DiskArbitration disappearance probe argv is not exact",
                ));
            }
        }
        label if label.starts_with("disk-info-") => {
            if args != &strings(&["info", "-plist", &mountpoint]) {
                return Err(invalid(
                    "diskutil info argv is not bound to the fixture mountpoint",
                ));
            }
        }
        label if label.starts_with("mount-") => {
            let read_only = label == "mount-read-only";
            let mut expected = vec!["mount".to_string()];
            if read_only {
                expected.push("readOnly".to_string());
            }
            expected.extend(strings(&[
                "nobrowse",
                "-mountOptions",
                "owners,nodev,nosuid,noexec,noatime",
                "-mountPoint",
                &mountpoint,
            ]));
            if args.len() != expected.len() + 1
                || args[..expected.len()] != expected
                || !valid_disk_identifier(&args[expected.len()])
            {
                return Err(invalid(
                    "diskutil mount argv is not the exact ownership-safe form",
                ));
            }
        }
        label if label.starts_with("detach-") => {
            if args.len() != 2
                || args[0] != "detach"
                || !args[1].starts_with("/dev/disk")
                || args
                    .iter()
                    .any(|argument| argument == "-force" || argument == "force")
            {
                return Err(invalid("hdiutil detach is not exact and non-forced"));
            }
        }
        _ => return Err(invalid("fixture command label is outside the exact flow")),
    }
    Ok(())
}

fn validate_unmount(
    receipt: &RawUnmountReceiptV1,
    busy: bool,
    expected_mountpoint: &Path,
) -> Result<(), AcceptanceError> {
    if receipt.flags != 0
        || receipt.mountpoint != expected_mountpoint.to_string_lossy()
        || receipt.duration_microseconds == 0
        || if busy {
            receipt.rc != -1 || receipt.errno != libc::EBUSY
        } else {
            receipt.rc != 0 || receipt.errno != 0
        }
    {
        return Err(invalid(
            "raw unmount rc/errno does not match its non-forced causal phase",
        ));
    }
    Ok(())
}

fn validate_holder_cycle(
    cycle: &HolderCycleReceiptV1,
    expected_mountpoint: &Path,
) -> Result<(), AcceptanceError> {
    validate_unmount(&cycle.unmount_with_holder, true, expected_mountpoint)?;
    validate_unmount(
        &cycle.clean_unmount_after_release,
        false,
        expected_mountpoint,
    )?;
    if cycle.holder != cycle.holder_after_busy
        || cycle.holder.pid <= 1
        || cycle.holder.parent_pid <= 1
        || cycle.holder.process_group_id <= 1
        || cycle.holder.real_uid != 0
        || cycle.holder.effective_uid != 0
        || cycle.holder.real_gid != 0
        || cycle.holder.effective_gid != 0
        || cycle.holder.start_seconds == 0
        || cycle.holder.start_microseconds >= 1_000_000
        || cycle.holder_release_wait_status != 0
        || !cycle.mount_still_same_after_busy
        || cycle.mount_statfs_before != cycle.mount_statfs_after_busy
        || cycle.mount_statfs_before.filesystem_type != "apfs"
    {
        return Err(invalid(
            "holder cycle does not causally bind EBUSY to one live volume reference",
        ));
    }
    Ok(())
}

fn validate_mutation_negative(negative: &ErrnoNegativeV1) -> Result<u32, AcceptanceError> {
    if negative.credentials_before != negative.credentials_after
        || negative.credentials_before.pid <= 1
    {
        return Err(invalid(
            "mutation probe credentials changed or lack a kernel PID",
        ));
    }
    let credentials = &negative.credentials_before;
    let expected_errno = match credentials.effective_uid {
        0 => {
            if credentials.real_uid != 0
                || credentials.real_gid != 0
                || credentials.effective_gid != 0
                || negative.child_wait_status.is_some()
                || !negative.operation.starts_with("read_only_")
            {
                return Err(invalid(
                    "root mutation probe lacks exact in-process root credentials",
                ));
            }
            libc::EROFS
        }
        LIVE_PRODUCER_UID => {
            if credentials.real_uid != LIVE_PRODUCER_UID
                || credentials.real_gid != LIVE_PRODUCER_GID
                || credentials.effective_gid != LIVE_PRODUCER_GID
                || !credentials.supplementary_groups.is_empty()
                || negative.child_wait_status != Some(0)
            {
                return Err(invalid(
                    "producer mutation probe lacks exact dropped kernel credentials",
                ));
            }
            if negative.operation == "unmounted_gap_create" {
                libc::EACCES
            } else if negative.operation.starts_with("read_only_") {
                libc::EROFS
            } else {
                return Err(invalid(
                    "producer mutation operation is outside the fixture",
                ));
            }
        }
        _ => return Err(invalid("mutation probe ran under an unexpected UID")),
    };
    if negative.observed_errno != expected_errno {
        return Err(invalid(
            "mutation probe raw errno differs from its credential-bound phase",
        ));
    }
    Ok(credentials.effective_uid)
}

fn command_json<T: for<'de> Deserialize<'de>>(
    root: &Path,
    commands: &BTreeMap<String, CommandReceiptV1>,
    label: &str,
) -> Result<T, AcceptanceError> {
    let receipt = commands
        .get(label)
        .ok_or_else(|| invalid(format!("missing converted command {label}")))?;
    let bytes = read_bounded(&root.join(&receipt.stdout_path), MAX_ARTIFACT_BYTES)?;
    serde_json::from_slice(&bytes)
        .map_err(|error| invalid(format!("converted command {label} is malformed: {error}")))
}

fn inventory_from_commands(
    root: &Path,
    commands: &BTreeMap<String, CommandReceiptV1>,
    stage: &str,
) -> Result<DiskInventoryV1, AcceptanceError> {
    let list_label = format!("disk-list-{stage}");
    let t5_label = format!("disk-node-info-t5-{stage}");
    let hdi_label = format!("hdi-info-inventory-{stage}");
    let list: DiskListPlist = command_json(root, commands, &format!("parse-{list_label}"))?;
    let t5: DiskNodeInfoPlist = command_json(root, commands, &format!("parse-{t5_label}"))?;
    let hdi: HdiutilInfoPlist = command_json(root, commands, &format!("parse-{hdi_label}"))?;
    build_disk_inventory(
        list,
        t5,
        hdi,
        &commands[&list_label],
        &commands[&t5_label],
        &commands[&hdi_label],
    )
}

fn disk_node_from_commands(
    root: &Path,
    commands: &BTreeMap<String, CommandReceiptV1>,
    label: &str,
    expect_whole: bool,
) -> Result<DiskNodeV1, AcceptanceError> {
    let info: DiskNodeInfoPlist = command_json(root, commands, &format!("parse-{label}"))?;
    disk_node_from_info(info, expect_whole)
}

fn same_backing_object_after_publication(
    sealed: &FileIdentityV1,
    current: &FileIdentityV1,
) -> bool {
    sealed.sha256 == current.sha256
        && sealed.binding.dev == current.binding.dev
        && sealed.binding.inode == current.binding.inode
        && sealed.binding.uid == current.binding.uid
        && sealed.binding.gid == current.binding.gid
        && sealed.binding.flags == current.binding.flags
        && sealed.binding.mtime_seconds == current.binding.mtime_seconds
        && sealed.binding.mtime_nanoseconds == current.binding.mtime_nanoseconds
        && sealed.binding.nlink == current.binding.nlink
        && sealed.binding.size == current.binding.size
}

fn reconstruct_attached_topology(
    root: &Path,
    commands: &BTreeMap<String, CommandReceiptV1>,
    suffix: &str,
    operation_nonce: &str,
    fact: &AttachedTopologyV1,
) -> Result<(), AcceptanceError> {
    let attach: HdiutilPlist = command_json(root, commands, &format!("parse-attach-{suffix}"))?;
    let apfs: ApfsListPlist = command_json(root, commands, &format!("parse-apfs-{suffix}"))?;
    let parsed = parse_attached_image(
        &attach,
        &apfs,
        &format!("hepta-apfs-{}", &operation_nonce[..16]),
    )?;
    let hdi: HdiutilInfoPlist = command_json(root, commands, &format!("parse-hdi-info-{suffix}"))?;
    let matching = hdi
        .images
        .iter()
        .filter(|image| image.image_path == fact.image_path_from_hdiutil)
        .collect::<Vec<_>>();
    let whole = disk_node_from_commands(
        root,
        commands,
        &format!("disk-node-info-{suffix}-whole"),
        true,
    )?;
    let physical = disk_node_from_commands(
        root,
        commands,
        &format!("disk-node-info-{suffix}-physical"),
        false,
    )?;
    let container = disk_node_from_commands(
        root,
        commands,
        &format!("disk-node-info-{suffix}-container"),
        true,
    )?;
    let volume = disk_node_from_commands(
        root,
        commands,
        &format!("disk-node-info-{suffix}-volume"),
        false,
    )?;
    let current_image = file_identity(&root.join(IMAGE_NAME))?;
    if matching.len() != 1
        || matching[0].system_entities != attach.system_entities
        || parsed.whole_disk_identifier != whole.device_identifier
        || parsed.physical_store_identifier != physical.device_identifier
        || parsed.container_identifier != container.device_identifier
        || parsed.volume_identifier != volume.device_identifier
        || parsed.apfs_container_uuid != fact.apfs_container_uuid
        || parsed.volume_uuid != fact.apfs_volume_uuid
        || command_receipt_sha256(&commands[&format!("hdi-info-{suffix}")])?
            != fact.hdiutil_info_command_sha256
        || whole != fact.whole_disk
        || physical != fact.physical_store
        || container != fact.apfs_container
        || volume != fact.apfs_volume
        || !same_backing_object_after_publication(&fact.image_backing_after, &current_image)
    {
        return Err(invalid(
            "sealed raw receipts do not reconstruct the attached image topology",
        ));
    }
    Ok(())
}

fn validate_result_authority(result: &ApfsFixtureResultV1) -> Result<(), AcceptanceError> {
    require_nonce(&result.operation_nonce)?;
    require_digest(&result.epoch_receipt_sha256, "fixture epoch receipt")?;
    require_digest(
        &result.attachment_obligation_terminal_sha256,
        "attachment obligation terminal",
    )?;
    let expected_obligation = format!("{OBLIGATION_PREFIX}{}", result.operation_nonce);
    if result.schema != SCHEMA
        || result.scope != "privileged_mechanism_only_no_migration_authority"
        || result.execution_kind != "broker_native_disposable_apfs_image_only"
        || result.attachment_obligation_directory != expected_obligation
        || result.epoch_receipt_path != "raw/000-epoch.json"
        || result.aggregate_authority
        || result.cutover_authority
        || result.deletion_authority
        || result.production_authority
        || result.refs_authority
        || result.remote_authority
    {
        return Err(invalid(
            "fixture RESULT is not the exact closed no-authority schema",
        ));
    }
    Ok(())
}

/// Independently replay the sealed inner fixture from raw receipts.
///
/// This verifier is intentionally usable without privilege.  It does not
/// trust summary booleans: it parses every epoch/fact receipt, re-hashes every
/// stdout/stderr/tool copy/image, validates every argv and raw unmount rc/errno,
/// reconstructs the APFS state machine, and checks a typed closed-world tree.
pub fn verify_disposable_fixture_tree(
    root: &Path,
    expected_result_sha256: &str,
) -> Result<ApfsFixtureVerificationV1, AcceptanceError> {
    require_digest(expected_result_sha256, "externally pinned fixture RESULT")?;
    if !root.is_absolute() || root.canonicalize()? != root {
        return Err(invalid("fixture root is not canonical and absolute"));
    }
    let (result, result_bytes) =
        read_canonical::<ApfsFixtureResultV1>(&root.join(RESULT_NAME), "fixture RESULT")?;
    let result_sha256 = sha256(&result_bytes);
    if result_sha256 != expected_result_sha256 {
        return Err(invalid(
            "fixture RESULT differs from its external digest pin",
        ));
    }
    validate_result_authority(&result)?;

    let (epoch, epoch_bytes) = read_canonical::<OperationEpochV1>(
        &root.join(&result.epoch_receipt_path),
        "fixture operation epoch",
    )?;
    validate_epoch(&epoch)?;
    let epoch_sha256 = sha256(&epoch_bytes);
    if epoch_sha256 != result.epoch_receipt_sha256
        || epoch.operation_nonce != result.operation_nonce
    {
        return Err(invalid(
            "fixture epoch differs from RESULT operation binding",
        ));
    }

    let mut expected_paths = [
        ".",
        IMAGE_NAME,
        LOGS_NAME,
        MODES_NAME,
        MOUNTPOINT_NAME,
        RESULT_NAME,
        SHA256SUMS_NAME,
        TOOLS_NAME,
        "raw",
        "raw/000-epoch.json",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<BTreeSet<_>>();
    let mut reference_labels = BTreeSet::new();
    let mut commands = BTreeMap::new();
    let mut attached_topologies = BTreeMap::new();
    let mut disk_arbitration_terminals = BTreeMap::new();
    let mut mounts = BTreeMap::new();
    let mut holders = BTreeMap::new();
    let mut images = BTreeMap::new();
    let mut mutations = BTreeMap::new();
    let mut raw_unmounts = BTreeMap::new();
    let mut source_after = None;
    let mut terminal = None;
    let mut volume_states = BTreeMap::new();
    let fixture_mountpoint = root.join(MOUNTPOINT_NAME);

    if result
        .raw_receipts
        .iter()
        .map(|reference| reference.label.clone())
        .collect::<Vec<_>>()
        != expected_fact_labels()
    {
        return Err(invalid(
            "raw fact receipts differ from the exact executable state-machine order",
        ));
    }

    for (index, reference) in result.raw_receipts.iter().enumerate() {
        let expected_sequence = (index + 1) as u32;
        require_label(&reference.label)?;
        require_digest(&reference.sha256, "raw fact reference")?;
        let expected_path = format!("raw/{expected_sequence:03}-{}.json", reference.label);
        if reference.sequence != expected_sequence
            || reference.path != expected_path
            || !reference_labels.insert(reference.label.clone())
        {
            return Err(invalid(
                "raw fact references are not unique contiguous typed paths",
            ));
        }
        expected_paths.insert(reference.path.clone());
        let (receipt, bytes) = read_canonical::<NativeFactReceiptV1>(
            &root.join(&reference.path),
            "native fact receipt",
        )?;
        if bytes.len() as u64 != reference.size
            || sha256(&bytes) != reference.sha256
            || receipt.schema != "hepta_mac_apfs_native_fact_receipt_v1"
            || receipt.sequence != reference.sequence
            || receipt.label != reference.label
            || receipt.operation_nonce != epoch.operation_nonce
            || receipt.boot_session_uuid != epoch.boot_session_uuid
            || receipt.challenge_sha256 != epoch.challenge_sha256
            || receipt.epoch_receipt_sha256 != epoch_sha256
            || receipt.source_binding != epoch.source_before.fd_binding
        {
            return Err(invalid(
                "native fact receipt differs from epoch/result/raw reference",
            ));
        }
        match receipt.fact {
            NativeFactV1::AttachedTopology { phase, topology } => {
                validate_attached_topology_shape(&topology)?;
                if attached_topologies.insert(phase, topology).is_some() {
                    return Err(invalid("fixture contains a duplicate attached topology"));
                }
            }
            NativeFactV1::Command(command) => {
                if command.label != reference.label
                    || commands.insert(command.label.clone(), command).is_some()
                {
                    return Err(invalid(
                        "command receipt label is duplicate or envelope-shaped",
                    ));
                }
            }
            NativeFactV1::HolderCycle(cycle) => {
                validate_holder_cycle(&cycle, &fixture_mountpoint)?;
                if holders.insert(cycle.holder_kind, cycle).is_some() {
                    return Err(invalid("fixture contains a duplicate holder kind"));
                }
            }
            NativeFactV1::ImageDigest { phase, sha256 } => {
                require_digest(&sha256, "APFS image")?;
                if images.insert(phase, sha256).is_some() {
                    return Err(invalid("fixture contains a duplicate image digest phase"));
                }
            }
            NativeFactV1::Mount { facts, phase } => {
                validate_mount_flags(&facts, phase == MountPhaseV1::ReadOnly)?;
                require_uuid(&facts.apfs_container_uuid, "mount APFS container")?;
                require_uuid(&facts.volume_uuid, "mount APFS volume")?;
                if mounts.insert(phase, facts).is_some() {
                    return Err(invalid("fixture contains a duplicate mount phase"));
                }
            }
            NativeFactV1::MutationNegative(negative) => {
                let uid = validate_mutation_negative(&negative)?;
                if mutations
                    .insert((uid, negative.operation.clone()), negative)
                    .is_some()
                {
                    return Err(invalid("mutation negative is duplicate"));
                }
            }
            NativeFactV1::RawUnmount {
                mountpoint_statfs_before,
                phase,
                receipt,
            } => {
                if mountpoint_statfs_before.filesystem_type != "apfs" {
                    return Err(invalid(
                        "raw unmount was not issued against APFS statfs facts",
                    ));
                }
                validate_unmount(&receipt, false, &fixture_mountpoint)?;
                if raw_unmounts
                    .insert(phase, (mountpoint_statfs_before, receipt))
                    .is_some()
                {
                    return Err(invalid("fixture contains a duplicate raw unmount phase"));
                }
            }
            NativeFactV1::Source { observation, phase } => {
                validate_descriptor_observation(&observation)?;
                if phase != SourcePhaseV1::After || source_after.replace(observation).is_some() {
                    return Err(invalid(
                        "fixture source-after receipt is duplicate or misphased",
                    ));
                }
            }
            NativeFactV1::Terminal {
                final_detached,
                mount_parent_after,
                mountpoint_underlying_after,
            } => {
                if terminal
                    .replace((
                        final_detached,
                        mount_parent_after,
                        mountpoint_underlying_after,
                    ))
                    .is_some()
                {
                    return Err(invalid("fixture contains a duplicate terminal fact"));
                }
            }
            NativeFactV1::DiskArbitrationTerminal {
                phase,
                terminal: fact,
            } => {
                if disk_arbitration_terminals.insert(phase, fact).is_some() {
                    return Err(invalid(
                        "fixture contains a duplicate DiskArbitration terminal",
                    ));
                }
            }
            NativeFactV1::VolumeStateDigest { phase, sha256 } => {
                require_digest(&sha256, "APFS volume state")?;
                if volume_states.insert(phase, sha256).is_some() {
                    return Err(invalid("fixture contains a duplicate volume-state phase"));
                }
            }
        }
    }

    if commands.keys().cloned().collect::<BTreeSet<_>>() != expected_executable_command_labels() {
        return Err(invalid(
            "fixture command receipts differ from the exact executable flow",
        ));
    }
    for command in commands.values() {
        verify_command_artifacts(root, command)?;
        verify_command_shape(root, &result.operation_nonce, command, &commands)?;
        expected_paths.insert(command.stdout_path.clone());
        expected_paths.insert(command.stderr_path.clone());
        expected_paths.insert(command.tool_copy_path.clone());
    }
    let pre_attach_inventory = inventory_from_commands(root, &commands, "pre-attach")?;
    for (phase, suffix) in [
        (MountPhaseV1::ReadWrite, "read-write"),
        (MountPhaseV1::ReadOnly, "read-only"),
    ] {
        let topology = attached_topologies
            .get(&phase)
            .ok_or_else(|| invalid("missing attached topology"))?;
        if topology.pre_attach_inventory_sha256 != inventory_sha256(&pre_attach_inventory)? {
            return Err(invalid(
                "attached topology differs from the raw pre-attach inventory",
            ));
        }
        reconstruct_attached_topology(root, &commands, suffix, &result.operation_nonce, topology)?;

        let terminal = disk_arbitration_terminals
            .get(&phase)
            .ok_or_else(|| invalid("missing DiskArbitration terminal"))?;
        let post_inventory =
            inventory_from_commands(root, &commands, &format!("post-{suffix}-detach"))?;
        let hdi_after: HdiutilInfoPlist = command_json(
            root,
            &commands,
            &format!("parse-hdi-info-after-detach-{suffix}"),
        )?;
        let da_command = &commands[&format!("disk-arbitration-confirm-{suffix}")];
        if hdi_after
            .images
            .iter()
            .any(|image| image.image_path == topology.image_path_from_hdiutil)
            || terminal.post_inventory != post_inventory
            || terminal.hdiutil_info_command_sha256
                != command_receipt_sha256(&commands[&format!("hdi-info-after-detach-{suffix}")])?
            || terminal.diskutil_info_command_sha256 != command_receipt_sha256(da_command)?
            || terminal.diskutil_info_exit_code != da_command.exit_code
            || da_command.arguments.get(2)
                != Some(&format!("/dev/{}", topology.whole_disk.device_identifier))
        {
            return Err(invalid(
                "raw hdiutil/diskutil receipts do not reconstruct terminal disappearance",
            ));
        }
        validate_disk_arbitration_terminal(
            terminal,
            &pre_attach_inventory,
            &epoch.mountpoint_underlying_before,
        )?;
    }

    let expected_holders = [
        HolderKindV1::ReadWriteFd,
        HolderKindV1::SharedWritableMappingOnly,
        HolderKindV1::CurrentWorkingDirectoryOnly,
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    if holders.keys().copied().collect::<BTreeSet<_>>() != expected_holders {
        return Err(invalid(
            "fixture does not contain exactly FD, mapping-only, and cwd holders",
        ));
    }
    let expected_mount_phases = [MountPhaseV1::ReadWrite, MountPhaseV1::ReadOnly]
        .into_iter()
        .collect::<BTreeSet<_>>();
    if attached_topologies.keys().copied().collect::<BTreeSet<_>>() != expected_mount_phases
        || disk_arbitration_terminals
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != expected_mount_phases
    {
        return Err(invalid(
            "fixture lacks exact RW/RO attached topology and DiskArbitration terminals",
        ));
    }
    if raw_unmounts.keys().copied().collect::<BTreeSet<_>>()
        != [UnmountPhaseV1::Baseline, UnmountPhaseV1::ReadOnlyFinal]
            .into_iter()
            .collect()
    {
        return Err(invalid(
            "fixture does not contain exact baseline and read-only clean unmounts",
        ));
    }
    if images.keys().copied().collect::<BTreeSet<_>>()
        != [
            ImageDigestPhaseV1::BeforeReadOnlyAttach,
            ImageDigestPhaseV1::AfterReadOnlyDetach,
        ]
        .into_iter()
        .collect()
        || images[&ImageDigestPhaseV1::BeforeReadOnlyAttach]
            != images[&ImageDigestPhaseV1::AfterReadOnlyDetach]
        || stable_file_sha256(&root.join(IMAGE_NAME))?
            != images[&ImageDigestPhaseV1::AfterReadOnlyDetach]
    {
        return Err(invalid(
            "APFS image bytes changed across read-only attach/detach",
        ));
    }
    if volume_states.keys().copied().collect::<BTreeSet<_>>()
        != [
            VolumeStatePhaseV1::BeforeReadOnlyNegatives,
            VolumeStatePhaseV1::AfterReadOnlyNegatives,
        ]
        .into_iter()
        .collect()
        || volume_states[&VolumeStatePhaseV1::BeforeReadOnlyNegatives]
            != volume_states[&VolumeStatePhaseV1::AfterReadOnlyNegatives]
    {
        return Err(invalid(
            "read-only mutation matrix changed the mounted volume state",
        ));
    }

    let mut expected_mutations = BTreeSet::new();
    expected_mutations.insert((LIVE_PRODUCER_UID, "unmounted_gap_create".to_string()));
    for uid in [0, LIVE_PRODUCER_UID] {
        for operation in [
            "create", "truncate", "write", "chmod", "rename", "unlink", "setxattr",
        ] {
            expected_mutations.insert((uid, format!("read_only_{operation}")));
        }
    }
    if mutations.keys().cloned().collect::<BTreeSet<_>>() != expected_mutations
        || mutations[&(LIVE_PRODUCER_UID, "unmounted_gap_create".to_string())].observed_errno
            != libc::EACCES
        || mutations
            .iter()
            .filter(|((_, operation), _)| operation.starts_with("read_only_"))
            .any(|(_, negative)| negative.observed_errno != libc::EROFS)
    {
        return Err(invalid(
            "fixture mutation negatives differ from exact producer/root errno matrix",
        ));
    }

    let rw = mounts
        .get(&MountPhaseV1::ReadWrite)
        .ok_or_else(|| invalid("missing read-write mount facts"))?;
    let ro = mounts
        .get(&MountPhaseV1::ReadOnly)
        .ok_or_else(|| invalid("missing read-only mount facts"))?;
    if rw.volume_uuid != ro.volume_uuid
        || rw.apfs_container_uuid != ro.apfs_container_uuid
        || rw.owner_sentinel_uid != LIVE_PRODUCER_UID
        || rw.owner_sentinel_gid != LIVE_PRODUCER_GID
        || ro.owner_sentinel_uid != LIVE_PRODUCER_UID
        || ro.owner_sentinel_gid != LIVE_PRODUCER_GID
        || rw.mount_on != root.join(MOUNTPOINT_NAME).to_string_lossy()
        || ro.mount_on != root.join(MOUNTPOINT_NAME).to_string_lossy()
    {
        return Err(invalid(
            "read-write/read-only mounts do not preserve UUID, mountpoint, and ownership",
        ));
    }
    let baseline_statfs = &raw_unmounts[&UnmountPhaseV1::Baseline].0;
    let read_only_final_statfs = &raw_unmounts[&UnmountPhaseV1::ReadOnlyFinal].0;
    if !statfs_matches_mount(baseline_statfs, rw)
        || !statfs_matches_mount(read_only_final_statfs, ro)
        || holders.values().any(|cycle| {
            !statfs_matches_mount(&cycle.mount_statfs_before, rw)
                || !statfs_matches_mount(&cycle.mount_statfs_after_busy, rw)
        })
    {
        return Err(invalid(
            "raw unmount and EBUSY holder facts are not bound to the exact RW/RO APFS mounts",
        ));
    }
    let rw_info: DiskInfoPlist = command_json(root, &commands, "parse-disk-info-read-write")?;
    let ro_info: DiskInfoPlist = command_json(root, &commands, "parse-disk-info-read-only")?;
    for (info, facts) in [(&rw_info, rw), (&ro_info, ro)] {
        if info.device_identifier != facts.device_identifier
            || info.filesystem_type != facts.filesystem_type
            || info.global_permissions_enabled != facts.global_permissions_enabled
            || info.volume_uuid.to_ascii_lowercase() != facts.volume_uuid
            || info.writable_media != facts.media_writable
            || info.writable_volume != facts.volume_writable
        {
            return Err(invalid(
                "typed mount facts differ from raw diskutil/plutil stdout",
            ));
        }
    }
    for (suffix, facts) in [("read-write", rw), ("read-only", ro)] {
        let attach: HdiutilPlist =
            command_json(root, &commands, &format!("parse-attach-{suffix}"))?;
        let apfs: ApfsListPlist = command_json(root, &commands, &format!("parse-apfs-{suffix}"))?;
        let attached = parse_attached_image(
            &attach,
            &apfs,
            &format!("hepta-apfs-{}", &result.operation_nonce[..16]),
        )?;
        let apfs_list = &commands[&format!("apfs-list-{suffix}")];
        let mount_labels: &[&str] = if suffix == "read-write" {
            &[
                "mount-read-write-initial",
                "mount-read-write-for-fd-holder",
                "mount-read-write-for-mapping-holder",
                "mount-read-write-for-cwd-holder",
            ]
        } else {
            &["mount-read-only"]
        };
        let expected_detach = format!("/dev/{}", attached.whole_disk_identifier);
        if apfs_list.arguments.get(3) != Some(&attached.container_identifier)
            || mount_labels
                .iter()
                .any(|label| commands[*label].arguments.last() != Some(&attached.volume_identifier))
            || commands[&format!("detach-{suffix}-nonforced")]
                .arguments
                .get(1)
                != Some(&expected_detach)
            || attached.volume_name != format!("hepta-apfs-{}", &result.operation_nonce[..16])
            || attached.volume_uuid != facts.volume_uuid
            || attached.apfs_container_uuid != facts.apfs_container_uuid
            || attached.volume_identifier != facts.device_identifier
            || attached.physical_store_identifier != facts.physical_store_identifier
            || attached.whole_disk_identifier != facts.whole_disk_identifier
        {
            return Err(invalid(
                "typed mount lineage differs from raw attach/APFS plist stdout",
            ));
        }
    }

    let source_after = source_after.ok_or_else(|| invalid("fixture omitted source-after facts"))?;
    if source_after != epoch.source_before {
        return Err(invalid(
            "source descriptor/statfs binding changed across the complete fixture",
        ));
    }
    let (final_detached, mount_parent_after, mountpoint_after) =
        terminal.ok_or_else(|| invalid("fixture omitted terminal detached fact"))?;
    if !final_detached
        || mount_parent_after.dev != epoch.mount_parent_before.dev
        || mount_parent_after.inode != epoch.mount_parent_before.inode
        || mountpoint_after.dev != epoch.mountpoint_underlying_before.dev
        || mountpoint_after.inode != epoch.mountpoint_underlying_before.inode
        || mountpoint_after.uid != 0
        || mountpoint_after.gid != 0
        || mountpoint_after.mode != 0o700
    {
        return Err(invalid(
            "terminal fact does not replay the detached underlying mountpoint inode",
        ));
    }

    let (manifest_entries, modes_entries, tree_replay_sha256) =
        verify_manifest_and_modes(root, &expected_paths)?;
    Ok(ApfsFixtureVerificationV1 {
        authority_granted: false,
        boot_session_uuid: epoch.boot_session_uuid,
        command_receipts: commands.len(),
        epoch_receipt_sha256: epoch_sha256,
        manifest_entries,
        modes_entries,
        operation_nonce: result.operation_nonce,
        raw_receipts: result.raw_receipts.len(),
        result_sha256,
        schema: "hepta_mac_apfs_fixture_verification_v1".to_string(),
        tree_replay_sha256,
    })
}

/// Verify both the broker no-replace publication and the independently sealed
/// APFS fact tree.  A prior-boot receipt or a source that changed since the
/// operation is rejected.  Even a successful return explicitly grants no
/// authority.
pub fn verify_sealed_disposable_fixture(
    namespace: &Path,
    operation_nonce: &str,
    expected_result_sha256: &str,
    policy: &NamespacePolicy,
) -> Result<(SealedPublicationV1, ApfsFixtureVerificationV1), AcceptanceError> {
    let publication = verify_sealed_publication(namespace, operation_nonce, policy)?;
    if publication.publication_receipt.authority_granted
        || publication.qualification_receipt.live_authority
        || publication.qualification_receipt.aggregate_authority
        || publication.qualification_receipt.cutover_authority
        || publication.qualification_receipt.deletion_authority
        || publication.qualification_receipt.production_authority
        || publication.qualification_receipt.refs_authority
        || publication.qualification_receipt.remote_authority
    {
        return Err(invalid(
            "sealed fixture broker receipt unexpectedly grants authority",
        ));
    }
    let expected_final = format!("apfs-fixture-{operation_nonce}");
    if publication.publication_receipt.final_name != expected_final {
        return Err(invalid(
            "sealed fixture publication final name differs from operation nonce",
        ));
    }
    let root = namespace.join(&expected_final);
    let verification = verify_disposable_fixture_tree(&root, expected_result_sha256)?;
    let (result, _) = read_canonical::<ApfsFixtureResultV1>(
        &root.join(RESULT_NAME),
        "sealed fixture RESULT for obligation binding",
    )?;
    let expected_obligation_directory = format!("{OBLIGATION_PREFIX}{operation_nonce}");
    if result.attachment_obligation_directory != expected_obligation_directory {
        return Err(invalid(
            "sealed fixture RESULT points outside its exact attachment obligation",
        ));
    }
    let obligation = inspect_attachment_obligation(namespace, operation_nonce)?;
    if obligation.authority_granted
        || !obligation.current_boot
        || obligation.requires_privileged_reconciliation
        || obligation.disposition != ObligationDispositionV1::Reconciled
        || obligation.terminal_record_sha256 != result.attachment_obligation_terminal_sha256
    {
        return Err(invalid(
            "sealed fixture attachment obligation is not current-boot terminal reconciled",
        ));
    }
    if verification.operation_nonce != operation_nonce
        || verification.boot_session_uuid != boot_session_uuid()?
    {
        return Err(invalid(
            "sealed fixture is from another operation or prior boot session",
        ));
    }
    let (epoch, _) = read_canonical::<OperationEpochV1>(
        &root.join("raw/000-epoch.json"),
        "sealed fixture operation epoch",
    )?;
    if observe_descriptor(Path::new(&epoch.source_before.absolute_path))? != epoch.source_before {
        return Err(invalid(
            "sealed fixture source binding is no longer current",
        ));
    }
    Ok((publication, verification))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::os::fd::AsRawFd;
    use std::os::unix::net::UnixStream;

    use crate::mac_privileged_broker::authenticate_connected_peer;

    use super::*;

    fn inert_result() -> ApfsFixtureResultV1 {
        let operation_nonce = "b".repeat(64);
        ApfsFixtureResultV1 {
            aggregate_authority: false,
            attachment_obligation_directory: format!("{OBLIGATION_PREFIX}{operation_nonce}"),
            attachment_obligation_terminal_sha256: "c".repeat(64),
            cutover_authority: false,
            deletion_authority: false,
            epoch_receipt_path: "raw/000-epoch.json".to_string(),
            epoch_receipt_sha256: "a".repeat(64),
            execution_kind: "broker_native_disposable_apfs_image_only".to_string(),
            operation_nonce,
            production_authority: false,
            raw_receipts: Vec::new(),
            refs_authority: false,
            remote_authority: false,
            schema: SCHEMA.to_string(),
            scope: "privileged_mechanism_only_no_migration_authority".to_string(),
        }
    }

    fn command_receipt(label: &str, arguments: &[&str]) -> CommandReceiptV1 {
        let binding = ObjectBindingV1 {
            ctime_nanoseconds: 0,
            ctime_seconds: 0,
            dev: 0,
            flags: 0,
            gid: 0,
            inode: 1,
            mode: 0o555,
            mtime_nanoseconds: 0,
            mtime_seconds: 0,
            nlink: 1,
            size: 1,
            uid: 0,
        };
        let tool = FileIdentityV1 {
            binding,
            path: command_tool(label).expect("known tool").to_string(),
            sha256: "d".repeat(64),
        };
        CommandReceiptV1 {
            arguments: strings(arguments),
            child_pid: 42,
            duration_milliseconds: 1,
            exit_code: 0,
            label: label.to_string(),
            stderr_path: format!("logs/000-{label}.stderr"),
            stderr_sha256: "e".repeat(64),
            stderr_size: 0,
            stdout_path: format!("logs/000-{label}.stdout"),
            stdout_sha256: "f".repeat(64),
            stdout_size: 1,
            tool_after: tool.clone(),
            tool_ancestor_chain_after: Vec::new(),
            tool_ancestor_chain_before: Vec::new(),
            tool_before: tool,
            tool_copy_path: format!("executed-tools/{}", "d".repeat(64)),
            tool_process_group_id: 42,
        }
    }

    fn test_binding(mode: u32, uid: u32, gid: u32, nlink: u64, inode: u64) -> ObjectBindingV1 {
        ObjectBindingV1 {
            ctime_nanoseconds: 0,
            ctime_seconds: 1,
            dev: 42,
            flags: 0,
            gid,
            inode,
            mode,
            mtime_nanoseconds: 0,
            mtime_seconds: 1,
            nlink,
            size: 1024,
            uid,
        }
    }

    fn test_inventory() -> DiskInventoryV1 {
        DiskInventoryV1 {
            all_disks: strings(&["disk2", "disk2s2", "disk3", "disk3s1"]),
            all_whole_disks: strings(&["disk2", "disk3"]),
            command_receipt_sha256: "1".repeat(64),
            hdiutil_backing_paths: Vec::new(),
            hdiutil_info_command_sha256: "2".repeat(64),
            schema: "hepta_mac_disk_inventory_v1".to_string(),
            t5_apfs_container_reference: "disk3".to_string(),
            t5_device_identifier: "disk3s1".to_string(),
            t5_parent_whole_disk: "disk2".to_string(),
            t5_physical_store_identifier: "disk2s2".to_string(),
            t5_volume_uuid: EXPECTED_T5_UUID.to_string(),
        }
    }

    fn test_image(modified: bool) -> FileIdentityV1 {
        let mut binding = test_binding(0o600, 0, 0, 1, 9001);
        if modified {
            binding.ctime_seconds = 2;
            binding.mtime_seconds = 2;
        }
        FileIdentityV1 {
            binding,
            path: "/Volumes/T5/hepta-vnext/artifacts/.incoming-test/barrier.dmg".to_string(),
            sha256: if modified {
                "4".repeat(64)
            } else {
                "3".repeat(64)
            },
        }
    }

    fn test_disk_node(identifier: &str, parent: &str, whole: bool) -> DiskNodeV1 {
        DiskNodeV1 {
            device_identifier: identifier.to_string(),
            device_node: format!("/dev/{identifier}"),
            disk_image: true,
            parent_whole_disk: parent.to_string(),
            size: 1_073_741_824,
            virtual_or_physical: "Virtual".to_string(),
            whole,
        }
    }

    fn test_topology(phase: MountPhaseV1) -> AttachedTopologyV1 {
        let image = test_image(phase == MountPhaseV1::ReadOnly);
        AttachedTopologyV1 {
            apfs_container: test_disk_node("disk10", "disk10", true),
            apfs_container_uuid: "11111111-1111-4111-8111-111111111111".to_string(),
            apfs_volume: test_disk_node("disk10s1", "disk10", false),
            apfs_volume_uuid: "22222222-2222-4222-8222-222222222222".to_string(),
            hdiutil_info_command_sha256: "5".repeat(64),
            image_backing_after: image.clone(),
            image_backing_before: image.clone(),
            image_path_from_hdiutil: image.path,
            physical_store: test_disk_node("disk9s1", "disk9", false),
            pre_attach_inventory_sha256: inventory_sha256(&test_inventory())
                .expect("test inventory digest"),
            schema: "hepta_mac_attached_apfs_topology_v1".to_string(),
            whole_disk: test_disk_node("disk9", "disk9", true),
        }
    }

    fn test_underlying_mountpoint() -> ObjectBindingV1 {
        test_binding(0o700, 0, 0, 2, 8001)
    }

    fn test_mount_statfs(phase: MountPhaseV1) -> StatFsFactsV1 {
        StatFsFactsV1 {
            filesystem_id: [71, 72],
            filesystem_type: "apfs".to_string(),
            mount_flags: MNT_NODEV
                | MNT_NOSUID
                | MNT_NOEXEC
                | MNT_NOATIME
                | if phase == MountPhaseV1::ReadOnly {
                    MNT_RDONLY
                } else {
                    0
                },
            mount_from: "/dev/disk10s1".to_string(),
            mount_on: "/Volumes/T5/hepta-vnext/artifacts/.incoming-test/mount".to_string(),
        }
    }

    fn test_unmount() -> RawUnmountReceiptV1 {
        RawUnmountReceiptV1 {
            duration_microseconds: 1,
            errno: 0,
            flags: 0,
            mountpoint: "/Volumes/T5/hepta-vnext/artifacts/.incoming-test/mount".to_string(),
            rc: 0,
        }
    }

    fn test_terminal() -> DiskArbitrationTerminalV1 {
        DiskArbitrationTerminalV1 {
            devnode_lstat_errno: libc::ENOENT,
            diskutil_info_command_sha256: "6".repeat(64),
            diskutil_info_exit_code: 1,
            hdiutil_info_command_sha256: "7".repeat(64),
            mountpoint_underlying_after: test_underlying_mountpoint(),
            nested_mounts_after: Vec::new(),
            post_inventory: test_inventory(),
            schema: "hepta_mac_diskarbitration_terminal_v1".to_string(),
            whole_disk_identifier: "disk9".to_string(),
        }
    }

    fn prepared_event() -> AttachmentObligationEventV1 {
        AttachmentObligationEventV1::Prepared {
            image_backing: test_image(false),
            mountpoint_underlying: test_underlying_mountpoint(),
            nested_mounts_before: Vec::new(),
            namespace_statfs: StatFsFactsV1 {
                filesystem_id: [1, 2],
                filesystem_type: "apfs".to_string(),
                mount_flags: 0,
                mount_from: "/dev/disk2s2".to_string(),
                mount_on: "/Volumes/T5".to_string(),
            },
            pre_attach_inventory: test_inventory(),
        }
    }

    fn successful_obligation_events() -> Vec<(AttachmentObligationEventV1, ObligationDispositionV1)>
    {
        let mut events = vec![(prepared_event(), ObligationDispositionV1::Active)];
        for phase in [MountPhaseV1::ReadWrite, MountPhaseV1::ReadOnly] {
            events.extend([
                (
                    AttachmentObligationEventV1::AttachStarted { phase },
                    ObligationDispositionV1::Active,
                ),
                (
                    AttachmentObligationEventV1::Attached {
                        phase,
                        topology: test_topology(phase),
                    },
                    ObligationDispositionV1::Active,
                ),
                (
                    AttachmentObligationEventV1::MountStarted {
                        phase,
                        volume_identifier: "disk10s1".to_string(),
                    },
                    ObligationDispositionV1::Active,
                ),
                (
                    AttachmentObligationEventV1::Mounted {
                        mountpoint_statfs: test_mount_statfs(phase),
                        phase,
                    },
                    ObligationDispositionV1::Active,
                ),
                (
                    AttachmentObligationEventV1::UnmountStarted { phase },
                    ObligationDispositionV1::Active,
                ),
                (
                    AttachmentObligationEventV1::Unmounted {
                        phase,
                        receipt: test_unmount(),
                    },
                    ObligationDispositionV1::Active,
                ),
                (
                    AttachmentObligationEventV1::DetachStarted {
                        phase,
                        whole_disk_identifier: "disk9".to_string(),
                    },
                    ObligationDispositionV1::Active,
                ),
                (
                    AttachmentObligationEventV1::DiskArbitrationGone {
                        phase,
                        terminal: test_terminal(),
                    },
                    ObligationDispositionV1::Active,
                ),
            ]);
        }
        events.push((
            AttachmentObligationEventV1::TerminalReconciled {
                post_inventory: test_inventory(),
            },
            ObligationDispositionV1::Reconciled,
        ));
        events
    }

    fn obligation_records(
        events: Vec<(AttachmentObligationEventV1, ObligationDispositionV1)>,
    ) -> Vec<(AttachmentObligationRecordV1, Vec<u8>)> {
        let mut previous = None;
        events
            .into_iter()
            .enumerate()
            .map(|(index, (event, disposition))| {
                let record = AttachmentObligationRecordV1 {
                    authority_granted: false,
                    boot_session_uuid: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".to_string(),
                    challenge_sha256: "8".repeat(64),
                    disposition,
                    epoch_receipt_sha256: "9".repeat(64),
                    event,
                    operation_nonce: "a".repeat(64),
                    previous_record_sha256: previous.clone(),
                    schema: "hepta_mac_attachment_obligation_record_v1".to_string(),
                    sequence: u32::try_from(index + 1).expect("test sequence"),
                };
                let bytes = canonical_json(&record).expect("canonical obligation record");
                previous = Some(sha256(&bytes));
                (record, bytes)
            })
            .collect()
    }

    #[test]
    fn fixture_plan_is_explicitly_inert() {
        let plan = plan();
        assert!(!plan.execution);
        assert!(!plan.aggregate_authority);
        assert!(!plan.cutover_authority);
        assert!(!plan.deletion_authority);
        assert!(!plan.production_authority);
        assert!(!plan.refs_authority);
        assert!(!plan.remote_authority);
        assert_eq!(
            plan.scope,
            "disposable_privileged_mechanism_only_no_migration_authority"
        );
    }

    #[test]
    fn rootless_peer_cannot_authorize_privileged_execution() {
        if unsafe { libc::geteuid() } == 0 || unsafe { libc::getegid() } == 0 {
            return;
        }
        let policy = NamespacePolicy::mechanism_only_current_user().expect("mechanism policy");
        let (left, _right) = UnixStream::pair().expect("socket pair");
        let peer = authenticate_connected_peer(left.as_raw_fd(), &policy)
            .expect("kernel-authenticated rootless peer");
        assert!(
            authorize_broker_native_execution(
                Path::new("/Volumes/T5/.hepta-privileged-qualification-v1-placeholder/publication"),
                Path::new("/"),
                &"c".repeat(64),
                &peer,
                &policy,
            )
            .is_err()
        );
    }

    #[test]
    fn raw_unmount_requires_exact_nonforced_rc_and_errno() {
        let mountpoint = Path::new("/private/tmp/hepta-test-mountpoint");
        let clean = RawUnmountReceiptV1 {
            duration_microseconds: 1,
            errno: 0,
            flags: 0,
            mountpoint: mountpoint.to_string_lossy().to_string(),
            rc: 0,
        };
        let busy = RawUnmountReceiptV1 {
            duration_microseconds: 1,
            errno: libc::EBUSY,
            flags: 0,
            mountpoint: mountpoint.to_string_lossy().to_string(),
            rc: -1,
        };
        assert!(validate_unmount(&clean, false, mountpoint).is_ok());
        assert!(validate_unmount(&busy, true, mountpoint).is_ok());
        assert!(validate_unmount(&clean, true, mountpoint).is_err());
        assert!(validate_unmount(&busy, false, mountpoint).is_err());

        let mut forced = clean.clone();
        forced.flags = libc::MNT_FORCE;
        assert!(validate_unmount(&forced, false, mountpoint).is_err());

        let mut wrong_errno = busy;
        wrong_errno.errno = libc::EACCES;
        assert!(validate_unmount(&wrong_errno, true, mountpoint).is_err());
    }

    #[test]
    fn mutation_errno_is_bound_to_kernel_credentials() {
        let producer = KernelCredentialsV1 {
            effective_gid: LIVE_PRODUCER_GID,
            effective_uid: LIVE_PRODUCER_UID,
            pid: 42,
            real_gid: LIVE_PRODUCER_GID,
            real_uid: LIVE_PRODUCER_UID,
            supplementary_groups: Vec::new(),
        };
        let negative = ErrnoNegativeV1 {
            credentials_after: producer.clone(),
            credentials_before: producer,
            child_wait_status: Some(0),
            observed_errno: libc::EROFS,
            operation: "read_only_write".to_string(),
        };
        assert_eq!(
            validate_mutation_negative(&negative).expect("producer EROFS"),
            LIVE_PRODUCER_UID
        );

        let mut forged_uid = negative.clone();
        forged_uid.credentials_before.effective_uid = 0;
        assert!(validate_mutation_negative(&forged_uid).is_err());

        let mut forged_errno = negative.clone();
        forged_errno.observed_errno = libc::EACCES;
        assert!(validate_mutation_negative(&forged_errno).is_err());

        let mut forged_groups = negative;
        forged_groups.credentials_before.supplementary_groups = vec![0];
        forged_groups.credentials_after.supplementary_groups = vec![0];
        assert!(validate_mutation_negative(&forged_groups).is_err());
    }

    #[test]
    fn exact_flow_and_t5_uuid_pins_are_stable() {
        let labels = expected_fact_labels();
        assert_eq!(labels.len(), 97);
        assert_eq!(
            labels.first().map(String::as_str),
            Some("create-disposable-image")
        );
        assert_eq!(labels.last().map(String::as_str), Some("terminal-detached"));
        assert_eq!(EXPECTED_T5_UUID, "fb804d1b-24cb-4d6e-aea7-a9e180807758");
        assert_eq!(T5_VOLUME_UUID[15], 0x58);
        assert!(labels.contains(&"mutation-read-only-0-chmod".to_string()));
        assert!(labels.contains(&format!("mutation-read-only-{LIVE_PRODUCER_UID}-chmod")));
    }

    #[test]
    fn system_tool_binding_requires_exact_root_wheel_0755() {
        let exact = test_binding(0o755, 0, 0, 1, 1);
        assert!(validate_system_tool_binding(&exact, "regular").is_ok());

        for (label, mut forged) in [
            ("0555", test_binding(0o555, 0, 0, 1, 1)),
            ("0775", test_binding(0o775, 0, 0, 1, 1)),
            ("setuid", test_binding(0o4755, 0, 0, 1, 1)),
            ("non-root-owner", test_binding(0o755, 501, 0, 1, 1)),
            ("non-wheel-group", test_binding(0o755, 0, 20, 1, 1)),
            ("hard-linked", test_binding(0o755, 0, 0, 2, 1)),
        ] {
            forged.size = exact.size;
            assert!(
                validate_system_tool_binding(&forged, "regular").is_err(),
                "forged system-tool mode {label} was accepted"
            );
        }
    }

    #[test]
    fn mac_system_tools_and_ancestors_match_the_real_0755_contract() {
        for tool in [HDIUTIL, DISKUTIL, LAUNCHCTL, PLUTIL] {
            let identity =
                file_identity(Path::new(tool)).expect("fixed macOS system tool identity");
            validate_system_tool_identity(&identity).expect("root:wheel 0755 exact-byte tool");
            let ancestors = system_tool_ancestor_chain(Path::new(tool))
                .expect("root-owned system-tool ancestor pins");
            assert!(!ancestors.is_empty());
            assert_eq!(
                ancestors.first().map(|entry| entry.path.as_str()),
                Some("/")
            );
            assert_eq!(
                ancestors.last().map(|entry| entry.path.as_str()),
                Path::new(tool).parent().and_then(Path::to_str)
            );
        }
    }

    #[test]
    fn obligation_success_replays_and_prior_boot_never_authorizes() {
        let records = obligation_records(successful_obligation_events());
        let current =
            replay_attachment_obligation_records(&records, "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa")
                .expect("current-boot successful obligation");
        assert_eq!(current.disposition, ObligationDispositionV1::Reconciled);
        assert!(!current.authority_granted);
        assert!(current.current_boot);
        assert!(!current.requires_privileged_reconciliation);
        assert_eq!(
            current.terminal_record_sha256,
            sha256(&records.last().expect("terminal record").1)
        );

        let prior_boot =
            replay_attachment_obligation_records(&records, "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb")
                .expect("prior-boot obligation remains inspectable");
        assert!(!prior_boot.authority_granted);
        assert!(!prior_boot.current_boot);
        assert!(prior_boot.requires_privileged_reconciliation);
    }

    #[test]
    fn obligation_state_machine_rejects_skips_and_duplicate_unmount() {
        let mut empty = ObligationState::empty();
        assert!(
            empty
                .apply(
                    &AttachmentObligationEventV1::AttachStarted {
                        phase: MountPhaseV1::ReadWrite,
                    },
                    ObligationDispositionV1::Active,
                )
                .is_err()
        );

        let mut state = ObligationState::empty();
        state
            .apply(&prepared_event(), ObligationDispositionV1::Active)
            .expect("Prepared");
        state
            .apply(
                &AttachmentObligationEventV1::AttachStarted {
                    phase: MountPhaseV1::ReadWrite,
                },
                ObligationDispositionV1::Active,
            )
            .expect("AttachStarted");
        state
            .apply(
                &AttachmentObligationEventV1::Attached {
                    phase: MountPhaseV1::ReadWrite,
                    topology: test_topology(MountPhaseV1::ReadWrite),
                },
                ObligationDispositionV1::Active,
            )
            .expect("Attached");
        state
            .apply(
                &AttachmentObligationEventV1::MountStarted {
                    phase: MountPhaseV1::ReadWrite,
                    volume_identifier: "disk10s1".to_string(),
                },
                ObligationDispositionV1::Active,
            )
            .expect("MountStarted");
        state
            .apply(
                &AttachmentObligationEventV1::Mounted {
                    mountpoint_statfs: test_mount_statfs(MountPhaseV1::ReadWrite),
                    phase: MountPhaseV1::ReadWrite,
                },
                ObligationDispositionV1::Active,
            )
            .expect("Mounted");
        assert!(
            state
                .apply(
                    &AttachmentObligationEventV1::DetachStarted {
                        phase: MountPhaseV1::ReadWrite,
                        whole_disk_identifier: "disk9".to_string(),
                    },
                    ObligationDispositionV1::Active,
                )
                .is_err()
        );
        state
            .apply(
                &AttachmentObligationEventV1::UnmountStarted {
                    phase: MountPhaseV1::ReadWrite,
                },
                ObligationDispositionV1::Active,
            )
            .expect("first UnmountStarted");
        assert!(
            state
                .apply(
                    &AttachmentObligationEventV1::UnmountStarted {
                        phase: MountPhaseV1::ReadWrite,
                    },
                    ObligationDispositionV1::Active,
                )
                .is_err()
        );
        state
            .apply(
                &AttachmentObligationEventV1::Unmounted {
                    phase: MountPhaseV1::ReadWrite,
                    receipt: test_unmount(),
                },
                ObligationDispositionV1::Active,
            )
            .expect("Unmounted");
        assert!(
            state
                .apply(
                    &AttachmentObligationEventV1::DiskArbitrationGone {
                        phase: MountPhaseV1::ReadWrite,
                        terminal: test_terminal(),
                    },
                    ObligationDispositionV1::Active,
                )
                .is_err()
        );
    }

    #[test]
    fn obligation_replay_rejects_hash_order_image_and_t5_alias_tampering() {
        let records = obligation_records(successful_obligation_events());

        let mut bad_hash = records.clone();
        bad_hash[5].0.previous_record_sha256 = Some("f".repeat(64));
        bad_hash[5].1 = canonical_json(&bad_hash[5].0).expect("tampered canonical record");
        assert!(
            replay_attachment_obligation_records(
                &bad_hash,
                "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            )
            .is_err()
        );

        let mut bad_order = records.clone();
        bad_order.swap(3, 4);
        assert!(
            replay_attachment_obligation_records(
                &bad_order,
                "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            )
            .is_err()
        );

        let mut image_events = successful_obligation_events();
        if let AttachmentObligationEventV1::Attached { topology, .. } = &mut image_events[2].0 {
            topology.image_backing_before.sha256 = "a".repeat(64);
            topology.image_backing_after.sha256 = "a".repeat(64);
        } else {
            panic!("expected first Attached event");
        }
        assert!(
            replay_attachment_obligation_records(
                &obligation_records(image_events),
                "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            )
            .is_err()
        );

        let mut alias_events = successful_obligation_events();
        if let AttachmentObligationEventV1::Attached { topology, .. } = &mut alias_events[2].0 {
            topology.whole_disk = test_disk_node("disk2", "disk2", true);
            topology.physical_store.parent_whole_disk = "disk2".to_string();
        } else {
            panic!("expected first Attached event");
        }
        assert!(
            replay_attachment_obligation_records(
                &obligation_records(alias_events),
                "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            )
            .is_err()
        );
    }

    #[test]
    fn incomplete_attachment_is_quarantined_without_authority() {
        let reason = "a".repeat(64);
        let events = vec![
            (prepared_event(), ObligationDispositionV1::Active),
            (
                AttachmentObligationEventV1::AttachStarted {
                    phase: MountPhaseV1::ReadWrite,
                },
                ObligationDispositionV1::Active,
            ),
            (
                AttachmentObligationEventV1::ReconcileRequired {
                    reason_sha256: reason.clone(),
                },
                ObligationDispositionV1::ReconcileRequired,
            ),
            (
                AttachmentObligationEventV1::Quarantined {
                    cross_boot: false,
                    reason_sha256: reason,
                },
                ObligationDispositionV1::Quarantined,
            ),
        ];
        let records = obligation_records(events.clone());
        let verification =
            replay_attachment_obligation_records(&records, "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa")
                .expect("quarantine is an inspectable fail-closed terminal");
        assert_eq!(
            verification.disposition,
            ObligationDispositionV1::Quarantined
        );
        assert!(!verification.authority_granted);
        assert!(verification.requires_privileged_reconciliation);

        let mut post_quarantine = events;
        post_quarantine.push((
            AttachmentObligationEventV1::AttachStarted {
                phase: MountPhaseV1::ReadWrite,
            },
            ObligationDispositionV1::Active,
        ));
        assert!(
            replay_attachment_obligation_records(
                &obligation_records(post_quarantine),
                "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            )
            .is_err()
        );
    }

    #[test]
    fn disk_arbitration_terminal_rejects_live_node_nested_mount_and_inventory_drift() {
        let baseline = test_inventory();
        let underlying = test_underlying_mountpoint();
        let terminal = test_terminal();
        assert!(validate_disk_arbitration_terminal(&terminal, &baseline, &underlying).is_ok());

        let mut still_live = terminal.clone();
        still_live.diskutil_info_exit_code = 0;
        assert!(validate_disk_arbitration_terminal(&still_live, &baseline, &underlying).is_err());
        let mut devnode_present = terminal.clone();
        devnode_present.devnode_lstat_errno = 0;
        assert!(
            validate_disk_arbitration_terminal(&devnode_present, &baseline, &underlying).is_err()
        );
        let mut nested = terminal.clone();
        nested
            .nested_mounts_after
            .push(test_mount_statfs(MountPhaseV1::ReadWrite));
        assert!(validate_disk_arbitration_terminal(&nested, &baseline, &underlying).is_err());
        let mut drift = terminal;
        drift.post_inventory.all_disks.push("disk9".to_string());
        drift
            .post_inventory
            .all_whole_disks
            .push("disk9".to_string());
        assert!(validate_disk_arbitration_terminal(&drift, &baseline, &underlying).is_err());
    }

    #[test]
    fn attached_topology_rejects_backing_and_layer_aliases() {
        let topology = test_topology(MountPhaseV1::ReadWrite);
        assert!(validate_attached_topology_shape(&topology).is_ok());

        let mut changed_backing = topology.clone();
        changed_backing.image_backing_after.sha256 = "a".repeat(64);
        assert!(validate_attached_topology_shape(&changed_backing).is_err());

        let mut duplicate_layer = topology.clone();
        duplicate_layer.apfs_volume = duplicate_layer.physical_store.clone();
        duplicate_layer.apfs_volume.parent_whole_disk = "disk10".to_string();
        assert!(validate_attached_topology_shape(&duplicate_layer).is_err());

        let mut wrong_parent = topology;
        wrong_parent.physical_store.parent_whole_disk = "disk8".to_string();
        assert!(validate_attached_topology_shape(&wrong_parent).is_err());
    }

    #[test]
    fn attach_mount_and_detach_argv_are_exact_and_nonforced() {
        let root = Path::new("/private/tmp/hepta-fixture-root");
        let nonce = "a".repeat(64);
        let image = root.join(IMAGE_NAME).to_string_lossy().to_string();
        let mountpoint = root.join(MOUNTPOINT_NAME).to_string_lossy().to_string();
        let commands = BTreeMap::new();

        let attach = command_receipt(
            "attach-read-write",
            &["attach", "-owners", "on", "-nomount", "-plist", &image],
        );
        assert!(verify_command_shape(root, &nonce, &attach, &commands).is_ok());
        let mut auto_mount = attach;
        auto_mount
            .arguments
            .retain(|argument| argument != "-nomount");
        assert!(verify_command_shape(root, &nonce, &auto_mount, &commands).is_err());

        let mount = command_receipt(
            "mount-read-only",
            &[
                "mount",
                "readOnly",
                "nobrowse",
                "-mountOptions",
                "owners,nodev,nosuid,noexec,noatime",
                "-mountPoint",
                &mountpoint,
                "disk9s1",
            ],
        );
        assert!(verify_command_shape(root, &nonce, &mount, &commands).is_ok());
        let mut no_owners = mount;
        no_owners.arguments[4] = "noowners,nodev,nosuid,noexec,noatime".to_string();
        assert!(verify_command_shape(root, &nonce, &no_owners, &commands).is_err());

        let detach = command_receipt("detach-read-only-nonforced", &["detach", "/dev/disk9"]);
        assert!(verify_command_shape(root, &nonce, &detach, &commands).is_ok());
        let forced = command_receipt(
            "detach-read-only-nonforced",
            &["detach", "-force", "/dev/disk9"],
        );
        assert!(verify_command_shape(root, &nonce, &forced, &commands).is_err());
    }

    #[test]
    fn result_schema_rejects_every_authority_grant() {
        let result = inert_result();
        assert!(validate_result_authority(&result).is_ok());

        for grant in [
            "aggregate",
            "cutover",
            "deletion",
            "production",
            "refs",
            "remote",
        ] {
            let mut forged = result.clone();
            match grant {
                "aggregate" => forged.aggregate_authority = true,
                "cutover" => forged.cutover_authority = true,
                "deletion" => forged.deletion_authority = true,
                "production" => forged.production_authority = true,
                "refs" => forged.refs_authority = true,
                "remote" => forged.remote_authority = true,
                _ => unreachable!(),
            }
            assert!(
                validate_result_authority(&forged).is_err(),
                "forged {grant} authority was accepted"
            );
        }
    }
}
