//! Descriptor-retained V3 effect-issue persistence for inert macOS effects.
//!
//! This module owns no effect primitive and grants no authority.  It binds one
//! canonical, typed command to one already-durable V2 issued-or-uncertain
//! lifecycle record and to the most recent durable collector observation.
//! New-format operations create `effect-issues-v3` through the S2 operation
//! wrapper before their outer operation directory is published.  A published
//! new-format operation with no exact issue directory is therefore blocking;
//! untouched historical V2 operations remain outside the V3 effect path.

use crate::AcceptanceError;
use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::mac_disposable_lifecycle::DisposableAuthorityV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleEventV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleRecordV2;
use crate::mac_disposable_lifecycle::EffectPurposeV2;
use crate::mac_disposable_lifecycle::LifecycleErrorV2;
use crate::mac_disposable_lifecycle::inspect_lifecycle_v2;
use crate::mac_disposable_lifecycle_store::OperationIssueReadSealV3;
use crate::mac_disposable_lifecycle_store::PreparedManifestS1TransferV3;
use crate::mac_disposable_lifecycle_store::RestartAdmissionRootS1TransferV3;
use crate::mac_disposable_lifecycle_store::RetainedEffectIssueSourceV3;
use crate::mac_disposable_lifecycle_store::RetainedLifecycleIssueSourceV3;
use crate::mac_disposable_reconciliation_collector::RetainedCollectorIssueBindingV3;
use crate::mac_disposable_reconciliation_collector::SealedCollectorEffectIssuePlanV3;
use crate::mac_disposable_reconciliation_collector::SealedEjectEffectPlanV3;
use crate::mac_disposable_reconciliation_collector::SealedUnmountEffectPlanV3;
use crate::mac_inert_one_shot_runner::AuthenticatedEffectEpochBindingV3;
use crate::mac_inert_one_shot_runner::RunnerIssueReadSealV3;
#[cfg(test)]
use crate::mac_iomedia_identity::current_boot_session_uuid;
use crate::mac_privileged_disposable_control::EffectIssueAppendSinkV3;
use crate::mac_privileged_disposable_control::FreshOperationAdmissionSinkV3;
use crate::mac_privileged_disposable_control::S1EffectIssueReadSealV3;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::Write;
use std::marker::PhantomData;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::RawFd;
use std::rc::Rc;
use thiserror::Error;

const ISSUE_SCHEMA_V3: &str = "hepta_mac_disposable_durable_effect_issue_v3";
const ISSUE_DIRECTORY_NAME_V3: &str = "effect-issues-v3";
const ISSUE_DIRECTORY_TEMPORARY_NAME_V3: &str = ".incoming-effect-issues-v3";
const PREPARED_MANIFEST_NAME_V3: &str = "prepared-collector-manifest-v3.json";
const RESTART_ADMISSION_DIRECTORY_NAME_V3: &str = "restart-admissions-v3";
const MAX_ISSUES_V3: usize = 256;
const MAX_ISSUE_BYTES_V3: usize = 1024 * 1024;
const MAX_LIFECYCLE_RECORDS_V3: usize = 256;
const MAX_TOTAL_LIFECYCLE_BYTES_V3: usize = 64 * 1024 * 1024;
const MAX_COMMAND_BYTES_V3: usize = 256 * 1024;
const MAX_TOTAL_ISSUE_BYTES_V3: usize = 64 * 1024 * 1024;
const MAX_OPERATION_ENTRIES_V3: usize = 1024;
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
pub(crate) enum DurableEffectIssueStoreErrorV3 {
    #[error("invalid durable V3 effect-issue store: {0}")]
    Invalid(String),
    #[error("V3 effect-issue persistence is issued-or-uncertain: {0}")]
    PersistenceUncertain(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Lifecycle(#[from] LifecycleErrorV2),
    #[error(transparent)]
    Acceptance(#[from] AcceptanceError),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EffectKindV3 {
    Create,
    Attach,
    Mount,
    Unmount,
    Eject,
}

mod issued_effect_kind_seal_v3 {
    pub trait Sealed {}
}

/// Type-level identity for one exact persisted unmount issue.  This marker has
/// no value constructor and grants no effect authority; it only prevents an
/// owned issue key from being transplanted into the eject path.
pub(crate) struct PersistedUnmountEffectV3 {
    _private: (),
}

/// Type-level identity for one exact persisted eject issue.  Like the unmount
/// marker, it is an inert compile-time discriminator only.
pub(crate) struct PersistedEjectEffectV3 {
    _private: (),
}

impl issued_effect_kind_seal_v3::Sealed for PersistedUnmountEffectV3 {}
impl issued_effect_kind_seal_v3::Sealed for PersistedEjectEffectV3 {}

trait IssuedEffectKindSpecV3: issued_effect_kind_seal_v3::Sealed {
    const KIND: EffectKindV3;
}

impl IssuedEffectKindSpecV3 for PersistedUnmountEffectV3 {
    const KIND: EffectKindV3 = EffectKindV3::Unmount;
}

impl IssuedEffectKindSpecV3 for PersistedEjectEffectV3 {
    const KIND: EffectKindV3 = EffectKindV3::Eject;
}

/// Read seal shared only with the collector's opaque effect-plan projection.
/// The collector can require this type to reveal a derived command, but no
/// other module can construct the seal or turn caller strings into a plan.
pub(crate) struct IssuePlanReadSealV3 {
    _private: (),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EffectPurposeV3 {
    ForwardFlow,
    RestartReconciliation,
}

/// Exact statfs binding expected after one inert mount command.  Keeping the
/// full entry in the durable command lets S1 admit exactly one before/after
/// mount-table pair; it never guesses a filesystem ID or accepts an observed
/// third state after dispatch.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ExactMountBindingCommandV3 {
    filesystem_id: [i32; 2],
    filesystem_type: String,
    mount_flags: u64,
    mount_from: String,
    mount_on: String,
}

pub(crate) enum ExactMountDeltaCommandViewV3<'a> {
    Mount {
        binding: &'a ExactMountBindingCommandV3,
        mountpoint_underlying_sha256: &'a str,
        read_only: bool,
        volume_identity_sha256: &'a str,
    },
    Unmount {
        mounted_binding_sha256: &'a str,
    },
}

/// Closed, inert command vocabulary.  Path-shaped strings exist only inside
/// an exact prepared statfs binding; there is no executable name, argument
/// vector, or effect primitive in this module.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum ExactDisposableCommandV3 {
    CreateImage {
        prepared_image_sha256: String,
        size_bytes: u64,
    },
    AttachImage {
        backing_identity_sha256: String,
        read_only: bool,
    },
    MountVolume {
        expected_binding: ExactMountBindingCommandV3,
        mountpoint_underlying_sha256: String,
        read_only: bool,
        volume_identity_sha256: String,
    },
    UnmountVolume {
        mounted_binding_sha256: String,
    },
    EjectImage {
        disk_image_group_sha256: String,
    },
}

impl ExactDisposableCommandV3 {
    fn kind(&self) -> EffectKindV3 {
        match self {
            Self::CreateImage { .. } => EffectKindV3::Create,
            Self::AttachImage { .. } => EffectKindV3::Attach,
            Self::MountVolume { .. } => EffectKindV3::Mount,
            Self::UnmountVolume { .. } => EffectKindV3::Unmount,
            Self::EjectImage { .. } => EffectKindV3::Eject,
        }
    }

    fn validate(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        match self {
            Self::CreateImage {
                prepared_image_sha256,
                size_bytes,
            } => {
                require_digest(prepared_image_sha256, "prepared image digest")?;
                if *size_bytes == 0 {
                    return Err(invalid("create-image size must be nonzero"));
                }
            }
            Self::AttachImage {
                backing_identity_sha256,
                ..
            } => require_digest(backing_identity_sha256, "backing identity digest")?,
            Self::MountVolume {
                expected_binding,
                mountpoint_underlying_sha256,
                read_only,
                volume_identity_sha256,
            } => {
                expected_binding.validate()?;
                require_digest(mountpoint_underlying_sha256, "mountpoint underlying digest")?;
                require_digest(volume_identity_sha256, "volume identity digest")?;
                if ((expected_binding.mount_flags & libc::MNT_RDONLY as u64) != 0) != *read_only {
                    return Err(invalid(
                        "mount access mode differs from the exact expected statfs flags",
                    ));
                }
            }
            Self::UnmountVolume {
                mounted_binding_sha256,
            } => require_digest(mounted_binding_sha256, "mounted binding digest")?,
            Self::EjectImage {
                disk_image_group_sha256,
            } => require_digest(disk_image_group_sha256, "disk-image group digest")?,
        }
        if canonical_json(self)?.len() > MAX_COMMAND_BYTES_V3 {
            return Err(invalid("typed command exceeds its fixed byte bound"));
        }
        Ok(())
    }

    pub(crate) fn mount_delta_view(&self) -> Option<ExactMountDeltaCommandViewV3<'_>> {
        match self {
            Self::MountVolume {
                expected_binding,
                mountpoint_underlying_sha256,
                read_only,
                volume_identity_sha256,
            } => Some(ExactMountDeltaCommandViewV3::Mount {
                binding: expected_binding,
                mountpoint_underlying_sha256,
                read_only: *read_only,
                volume_identity_sha256,
            }),
            Self::UnmountVolume {
                mounted_binding_sha256,
            } => Some(ExactMountDeltaCommandViewV3::Unmount {
                mounted_binding_sha256,
            }),
            _ => None,
        }
    }
}

impl ExactMountBindingCommandV3 {
    fn validate(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        for (value, label) in [
            (&self.filesystem_type, "mount filesystem type"),
            (&self.mount_from, "mount source"),
            (&self.mount_on, "mount target"),
        ] {
            if value.is_empty()
                || value.len() > 4096
                || value.as_bytes().contains(&0)
                || (label == "mount target" && !value.starts_with('/'))
            {
                return Err(invalid(format!("{label} is malformed")));
            }
        }
        Ok(())
    }

    pub(crate) fn filesystem_id(&self) -> [i32; 2] {
        self.filesystem_id
    }

    pub(crate) fn filesystem_type(&self) -> &str {
        &self.filesystem_type
    }

    pub(crate) fn mount_flags(&self) -> u64 {
        self.mount_flags
    }

    pub(crate) fn mount_from(&self) -> &str {
        &self.mount_from
    }

    pub(crate) fn mount_on(&self) -> &str {
        &self.mount_on
    }

    #[cfg(test)]
    pub(crate) fn for_test(
        filesystem_id: [i32; 2],
        filesystem_type: &str,
        mount_flags: u64,
        mount_from: &str,
        mount_on: &str,
    ) -> Self {
        Self {
            filesystem_id,
            filesystem_type: filesystem_type.to_string(),
            mount_flags,
            mount_from: mount_from.to_string(),
            mount_on: mount_on.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IssuedEffectRecordV3 {
    authority: DisposableAuthorityV2,
    boot_session_uuid: String,
    command: ExactDisposableCommandV3,
    command_canonical_json: String,
    command_sha256: String,
    effect_id: u64,
    effect_kind: EffectKindV3,
    lifecycle_issue_record_sha256: String,
    lifecycle_issue_sequence: u32,
    lifecycle_tip_before_sha256: String,
    operation_nonce: String,
    prior_collector_lifecycle_record_sha256: String,
    prior_collector_lifecycle_sequence: u32,
    prior_collector_receipt_sha256: String,
    process_epoch_nonce: String,
    process_epoch_sha256: String,
    purpose: EffectPurposeV3,
    runner_epoch_nonce: String,
    runner_epoch_sha256: String,
    runner_hello_sha256: String,
    runner_kernel_start_microseconds: u64,
    runner_pid: u32,
    runner_pre_hello_fd_census_sha256: String,
    runner_transport_sha256: String,
    schema: String,
    schema_version: u32,
    supervisor_kernel_start_microseconds: u64,
    supervisor_parent_pid: u32,
    supervisor_pid: u32,
    unique_binding_sha256: Option<String>,
}

/// Data-only epoch binding.  Production deliberately has no constructor yet:
/// the runner integration must seal it from the fresh process epoch and the
/// authenticated pre-runner before this inert persistence path becomes usable.
pub(crate) struct EffectEpochEvidenceV3 {
    boot_session_uuid: String,
    process_epoch_nonce: String,
    process_epoch_sha256: String,
    runner_epoch_nonce: String,
    runner_epoch_sha256: String,
    runner_hello_sha256: String,
    runner_kernel_start_microseconds: u64,
    runner_pid: u32,
    runner_pre_hello_fd_census_sha256: String,
    runner_transport_sha256: String,
    supervisor_kernel_start_microseconds: u64,
    supervisor_parent_pid: u32,
    supervisor_pid: u32,
}

impl EffectEpochEvidenceV3 {
    pub(crate) fn from_authenticated(
        binding: AuthenticatedEffectEpochBindingV3,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        binding.validate_current().map_err(|error| {
            invalid(format!(
                "authenticated runner epoch failed current replay: {error}"
            ))
        })?;
        let runner_transport_sha256 = binding.transport_sha256().map_err(|error| {
            invalid(format!(
                "authenticated runner transport could not be sealed: {error}"
            ))
        })?;
        let evidence = Self {
            boot_session_uuid: binding.boot_session_uuid().to_string(),
            process_epoch_nonce: binding.process_epoch_nonce().to_string(),
            process_epoch_sha256: binding.process_epoch_sha256().to_string(),
            runner_epoch_nonce: binding.runner_epoch_nonce().to_string(),
            runner_epoch_sha256: binding.runner_epoch_sha256().to_string(),
            runner_hello_sha256: binding.runner_hello_sha256().to_string(),
            runner_kernel_start_microseconds: binding.runner_kernel_start_microseconds(),
            runner_pid: binding.runner_pid(),
            runner_pre_hello_fd_census_sha256: binding.pre_hello_fd_census_sha256().to_string(),
            runner_transport_sha256,
            supervisor_kernel_start_microseconds: binding.supervisor_kernel_start_microseconds(),
            supervisor_parent_pid: binding.supervisor_parent_pid(),
            supervisor_pid: binding.supervisor_pid(),
        };
        evidence.validate()?;
        Ok(evidence)
    }

    #[cfg(test)]
    pub(crate) fn bind_current_boot(
        boot_session_uuid: &str,
        process_epoch_nonce: &str,
        process_epoch_sha256: &str,
        runner_epoch_nonce: &str,
        runner_epoch_sha256: &str,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        Self::bind_current_boot_with_identities(
            boot_session_uuid,
            process_epoch_nonce,
            process_epoch_sha256,
            runner_epoch_nonce,
            runner_epoch_sha256,
            2,
            3,
            2,
            1,
            1,
        )
    }

    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn bind_current_boot_with_identities(
        boot_session_uuid: &str,
        process_epoch_nonce: &str,
        process_epoch_sha256: &str,
        runner_epoch_nonce: &str,
        runner_epoch_sha256: &str,
        supervisor_pid: u32,
        supervisor_parent_pid: u32,
        supervisor_kernel_start_microseconds: u64,
        runner_pid: u32,
        runner_kernel_start_microseconds: u64,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        require_uuid(boot_session_uuid, "effect issue boot session UUID")?;
        require_nonce(process_epoch_nonce, "process epoch nonce")?;
        require_digest(process_epoch_sha256, "process epoch digest")?;
        require_nonce(runner_epoch_nonce, "runner epoch nonce")?;
        require_digest(runner_epoch_sha256, "runner epoch digest")?;
        if current_boot_session_uuid()? != boot_session_uuid
            || process_epoch_nonce == runner_epoch_nonce
            || process_epoch_sha256 == runner_epoch_sha256
        {
            return Err(invalid(
                "effect epochs are stale, aliased, or not bound to the current boot",
            ));
        }
        let evidence = Self {
            boot_session_uuid: boot_session_uuid.to_string(),
            process_epoch_nonce: process_epoch_nonce.to_string(),
            process_epoch_sha256: process_epoch_sha256.to_string(),
            runner_epoch_nonce: runner_epoch_nonce.to_string(),
            runner_epoch_sha256: runner_epoch_sha256.to_string(),
            runner_hello_sha256: runner_epoch_sha256.to_string(),
            runner_kernel_start_microseconds,
            runner_pid,
            runner_pre_hello_fd_census_sha256: sha256(b"test-pre-hello-fd-census"),
            runner_transport_sha256: sha256(b"test-runner-transport"),
            supervisor_kernel_start_microseconds,
            supervisor_parent_pid,
            supervisor_pid,
        };
        evidence.validate()?;
        Ok(evidence)
    }

    fn validate(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        require_uuid(&self.boot_session_uuid, "effect issue boot session UUID")?;
        require_nonce(&self.process_epoch_nonce, "process epoch nonce")?;
        require_digest(&self.process_epoch_sha256, "process epoch digest")?;
        require_nonce(&self.runner_epoch_nonce, "runner epoch nonce")?;
        require_digest(&self.runner_epoch_sha256, "runner epoch digest")?;
        require_digest(&self.runner_hello_sha256, "runner hello digest")?;
        require_digest(
            &self.runner_pre_hello_fd_census_sha256,
            "runner pre-hello FD census digest",
        )?;
        require_digest(&self.runner_transport_sha256, "runner transport digest")?;
        if self.process_epoch_nonce == self.runner_epoch_nonce
            || self.process_epoch_sha256 == self.runner_epoch_sha256
            || self.runner_epoch_sha256 != self.runner_hello_sha256
            || self.runner_pid == 0
            || self.runner_kernel_start_microseconds == 0
            || self.supervisor_pid == 0
            || self.supervisor_kernel_start_microseconds == 0
            || self.supervisor_pid == self.runner_pid
            || self.supervisor_parent_pid == self.supervisor_pid
        {
            return Err(invalid(
                "effect epochs, runner identity, hello, or transport binding is invalid",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CollectorObservationBindingV3 {
    boot_session_uuid: String,
    receipt_sha256: String,
    record_sha256: String,
    sequence: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LifecycleIssueBindingV3 {
    effect_id: u64,
    kind: EffectKindV3,
    operation_nonce: String,
    previous_tip_sha256: String,
    prior_collector: Option<CollectorObservationBindingV3>,
    purpose: EffectPurposeV3,
    record_sha256: String,
    sequence: u32,
}

struct RetainedLifecycleRecordV3 {
    binding: FilesystemBindingV3,
    file: File,
    name: String,
    record_sha256: String,
    size: usize,
}

enum LifecycleReplaySourceV3 {
    Retained {
        operation_directory: File,
        operation_directory_binding: FilesystemBindingV3,
        records: Vec<RetainedLifecycleRecordV3>,
        roster: Vec<String>,
    },
    /// Short-lived semantic replay over bytes whose exact descriptors,
    /// pathnames, and closed-world rosters are retained and revalidated by
    /// S1.  This source is created only behind `S1EffectIssueReadSealV3` and
    /// never escapes the S1 bijection verifier.
    S1DescriptorRetained,
    #[cfg(test)]
    Synthetic,
}

/// Exact canonical V2 replay product.  Fields are private so caller-provided
/// effect IDs, tips, or collector digests cannot stand in for lifecycle bytes.
pub(crate) struct VerifiedLifecycleIssueRosterV3 {
    issues: Vec<LifecycleIssueBindingV3>,
    operation_nonce: String,
    source: LifecycleReplaySourceV3,
    terminal_record_sha256: String,
}

impl VerifiedLifecycleIssueRosterV3 {
    pub(crate) fn capture_from_s2(
        source: RetainedLifecycleIssueSourceV3<'_>,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        let captured = Self::capture_directory(
            source.directory(),
            source.expected_uid(),
            source.expected_gid(),
        )?;
        if captured.operation_nonce() != source.operation_nonce() {
            return Err(invalid(
                "sealed S2 operation nonce differs from its retained lifecycle replay",
            ));
        }
        Ok(captured)
    }

    /// Capture canonical V2 numbered records directly from their retained
    /// operation directory.  Production callers cannot substitute an
    /// in-memory journal or self-reported digest for this descriptor roster.
    #[cfg(test)]
    pub(crate) fn capture(
        operation_directory: &File,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        Self::capture_directory(operation_directory, expected_uid, expected_gid)
    }

    fn capture_directory(
        operation_directory: &File,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        require_current_owner(expected_uid, expected_gid)?;
        let operation_directory = operation_directory.try_clone()?;
        let operation_directory_binding = validate_directory(
            &operation_directory,
            expected_uid,
            expected_gid,
            None,
            "V2 lifecycle operation directory",
        )?;
        let roster = lifecycle_record_roster(operation_directory.as_raw_fd())?;
        if roster.is_empty() || roster.len() > MAX_LIFECYCLE_RECORDS_V3 {
            return Err(invalid(
                "V2 lifecycle descriptor roster is empty or exceeds its bound",
            ));
        }
        let mut aggregate = 0usize;
        let mut bytes = Vec::with_capacity(roster.len());
        let mut retained = Vec::with_capacity(roster.len());
        for (index, name) in roster.iter().enumerate() {
            if lifecycle_record_name(index + 1) != *name {
                return Err(invalid(
                    "V2 lifecycle descriptor roster is not contiguous and canonical",
                ));
            }
            let file = openat_issue_file(operation_directory.as_raw_fd(), name)?;
            let initial = binding(&file)?;
            let size = usize::try_from(initial.size)
                .ok()
                .filter(|size| *size > 0 && *size <= MAX_ISSUE_BYTES_V3)
                .ok_or_else(|| invalid("V2 lifecycle record size is outside its bound"))?;
            aggregate = aggregate
                .checked_add(size)
                .filter(|total| *total <= MAX_TOTAL_LIFECYCLE_BYTES_V3)
                .ok_or_else(|| invalid("V2 lifecycle descriptor aggregate exceeds its bound"))?;
            let binding = validate_issue_file(
                &file,
                expected_uid,
                expected_gid,
                operation_directory_binding.dev,
                size,
                "retained V2 lifecycle record",
            )?;
            if binding != initial
                || named_binding(operation_directory.as_raw_fd(), name)? != binding
            {
                return Err(invalid(
                    "V2 lifecycle record pathname differs from its retained descriptor",
                ));
            }
            let record_bytes = read_exact_file(&file, binding)?;
            let record_sha256 = sha256(&record_bytes);
            bytes.push(record_bytes);
            retained.push(RetainedLifecycleRecordV3 {
                binding,
                file,
                name: name.clone(),
                record_sha256,
                size,
            });
        }
        if lifecycle_record_roster(operation_directory.as_raw_fd())? != roster
            || !same_directory_object(
                validate_directory(
                    &operation_directory,
                    expected_uid,
                    expected_gid,
                    None,
                    "V2 lifecycle operation directory final capture",
                )?,
                operation_directory_binding,
            )
        {
            return Err(invalid(
                "V2 lifecycle descriptor roster changed during capture",
            ));
        }
        let source = LifecycleReplaySourceV3::Retained {
            operation_directory,
            operation_directory_binding,
            records: retained,
            roster,
        };
        let result = Self::replay_with_source(&bytes, source)?;
        result.revalidate()?;
        Ok(result)
    }

    #[cfg(test)]
    pub(crate) fn replay(records: &[Vec<u8>]) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        Self::replay_with_source(records, LifecycleReplaySourceV3::Synthetic)
    }

    fn replay_with_source(
        records: &[Vec<u8>],
        source: LifecycleReplaySourceV3,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        if records.len() > MAX_LIFECYCLE_RECORDS_V3 {
            return Err(invalid("lifecycle roster exceeds its fixed replay bound"));
        }
        records.iter().try_fold(0usize, |total, bytes| {
            if bytes.is_empty() || bytes.len() > MAX_ISSUE_BYTES_V3 {
                return Err(invalid("lifecycle record exceeds the issue replay bound"));
            }
            total
                .checked_add(bytes.len())
                .filter(|total| *total <= MAX_TOTAL_LIFECYCLE_BYTES_V3)
                .ok_or_else(|| invalid("lifecycle replay exceeds its aggregate byte bound"))
        })?;
        let inspection = inspect_lifecycle_v2(records)?;
        let mut issues = Vec::new();
        let mut latest_collector = None;
        for bytes in records {
            let record: DisposableLifecycleRecordV2 = serde_json::from_slice(bytes)
                .map_err(|error| invalid(format!("lifecycle record JSON failed: {error}")))?;
            if canonical_json(&record)? != *bytes
                || record.operation_nonce != inspection.operation_nonce
            {
                return Err(invalid(
                    "lifecycle issue input is not the exact canonical replay",
                ));
            }
            let record_sha256 = sha256(bytes);
            match &record.event {
                DisposableLifecycleEventV2::ReconciliationSnapshotObserved { snapshot } => {
                    latest_collector = Some(CollectorObservationBindingV3 {
                        boot_session_uuid: snapshot.boot_session_uuid.clone(),
                        receipt_sha256: snapshot.collector_receipt_sha256.clone(),
                        record_sha256,
                        sequence: record.sequence,
                    });
                }
                DisposableLifecycleEventV2::FreshAbsenceObserved { observation } => {
                    latest_collector = Some(CollectorObservationBindingV3 {
                        boot_session_uuid: observation.boot_session_uuid.clone(),
                        receipt_sha256: observation.collector_receipt_sha256.clone(),
                        record_sha256,
                        sequence: record.sequence,
                    });
                }
                DisposableLifecycleEventV2::UnmountObserved {
                    collector: Some(collector),
                    ..
                }
                | DisposableLifecycleEventV2::EjectObserved {
                    collector: Some(collector),
                    ..
                } => {
                    // The post-effect receipt is admitted only by lifecycle
                    // replay against the active V3 restart lineage.  Bind the
                    // next issue to both that receipt and this exact retained
                    // V2 observation record, never to a digest-only DTO.
                    latest_collector = Some(CollectorObservationBindingV3 {
                        boot_session_uuid: collector.boot_session_uuid().to_string(),
                        receipt_sha256: collector.collector_receipt_sha256().to_string(),
                        record_sha256,
                        sequence: record.sequence,
                    });
                }
                DisposableLifecycleEventV2::RestartReconciliationStarted { .. }
                | DisposableLifecycleEventV2::CreateObserved { .. }
                | DisposableLifecycleEventV2::AttachObserved { .. }
                | DisposableLifecycleEventV2::MountObserved { .. }
                | DisposableLifecycleEventV2::UnmountObserved {
                    collector: None, ..
                }
                | DisposableLifecycleEventV2::EjectObserved {
                    collector: None, ..
                } => {
                    // A collector receipt binds one exact pre-effect state.
                    // Once durable state advances (or a new restart epoch
                    // starts), that receipt may never authorize another
                    // command by merely remaining the most recent snapshot.
                    latest_collector = None;
                }
                event => {
                    if let Some((effect_id, kind, purpose)) = issued_binding(event) {
                        let previous_tip_sha256 = record
                            .previous_record_sha256
                            .clone()
                            .ok_or_else(|| invalid("issued lifecycle record has no predecessor"))?;
                        issues.push(LifecycleIssueBindingV3 {
                            effect_id,
                            kind,
                            operation_nonce: record.operation_nonce.clone(),
                            previous_tip_sha256,
                            prior_collector: latest_collector.clone(),
                            purpose,
                            record_sha256,
                            sequence: record.sequence,
                        });
                    }
                }
            }
        }
        if issues.len() > MAX_ISSUES_V3 {
            return Err(invalid("issued lifecycle roster exceeds its fixed bound"));
        }
        Ok(Self {
            issues,
            operation_nonce: inspection.operation_nonce,
            source,
            terminal_record_sha256: inspection.terminal_record_sha256,
        })
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    pub(crate) fn terminal_record_sha256(&self) -> &str {
        &self.terminal_record_sha256
    }

    fn revalidate(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        match &self.source {
            LifecycleReplaySourceV3::Retained {
                operation_directory,
                operation_directory_binding,
                records,
                roster,
            } => {
                let operation = validate_directory(
                    operation_directory,
                    operation_directory_binding.uid,
                    operation_directory_binding.gid,
                    None,
                    "retained V2 lifecycle operation directory",
                )?;
                if !same_directory_object(operation, *operation_directory_binding)
                    || lifecycle_record_roster(operation_directory.as_raw_fd())? != *roster
                {
                    return Err(invalid("retained V2 lifecycle roster changed"));
                }
                for record in records {
                    if binding(&record.file)? != record.binding
                        || named_binding(operation_directory.as_raw_fd(), &record.name)?
                            != record.binding
                        || validate_issue_file(
                            &record.file,
                            operation_directory_binding.uid,
                            operation_directory_binding.gid,
                            operation_directory_binding.dev,
                            record.size,
                            "retained V2 lifecycle record replay",
                        )? != record.binding
                        || sha256(&read_exact_file(&record.file, record.binding)?)
                            != record.record_sha256
                        || named_binding(operation_directory.as_raw_fd(), &record.name)?
                            != record.binding
                    {
                        return Err(invalid(
                            "retained V2 lifecycle record changed during replay",
                        ));
                    }
                }
                if lifecycle_record_roster(operation_directory.as_raw_fd())? != *roster
                    || !same_directory_object(
                        validate_directory(
                            operation_directory,
                            operation_directory_binding.uid,
                            operation_directory_binding.gid,
                            None,
                            "retained V2 lifecycle operation directory final replay",
                        )?,
                        *operation_directory_binding,
                    )
                {
                    return Err(invalid(
                        "retained V2 lifecycle roster changed across final replay",
                    ));
                }
                Ok(())
            }
            LifecycleReplaySourceV3::S1DescriptorRetained => Ok(()),
            #[cfg(test)]
            LifecycleReplaySourceV3::Synthetic => Ok(()),
        }
    }
}

/// Verify the complete V2 issued-event to V3 issue-file bijection over exact
/// descriptor-retained S1 capsules.  Raw JSON never produces a capability:
/// only S1 can construct the seal, this function returns `Result<()>`, and S1
/// revalidates every descriptor/name/byte roster before and after capture.
pub(crate) fn validate_effect_issue_bijection_for_s1(
    lifecycle_records: &[Vec<u8>],
    issue_records: &[(&str, &[u8])],
    operation_nonce: &str,
    _seal: S1EffectIssueReadSealV3,
) -> Result<(), DurableEffectIssueStoreErrorV3> {
    let lifecycle = VerifiedLifecycleIssueRosterV3::replay_with_source(
        lifecycle_records,
        LifecycleReplaySourceV3::S1DescriptorRetained,
    )?;
    if lifecycle.operation_nonce != operation_nonce {
        return Err(invalid(
            "S1 operation nonce differs from its exact lifecycle issue roster",
        ));
    }
    if issue_records.len() != lifecycle.issues.len() {
        return Err(invalid(
            "S1 V2 issued roster and retained V3 issue roster are not an exact bijection",
        ));
    }

    let mut by_effect = BTreeMap::new();
    for (name, bytes) in issue_records {
        if bytes.is_empty() || bytes.len() > MAX_ISSUE_BYTES_V3 {
            return Err(invalid(
                "S1 retained V3 issue size is outside its fixed bound",
            ));
        }
        let (effect_id, name_sha256) = parse_issue_name(name)
            .ok_or_else(|| invalid("S1 retained V3 issue name is noncanonical"))?;
        if by_effect.contains_key(&effect_id) {
            return Err(invalid(
                "S1 retained V3 issue roster duplicates an effect ID",
            ));
        }
        if sha256(bytes) != name_sha256 {
            return Err(invalid(
                "S1 retained V3 issue bytes differ from their filename digest",
            ));
        }
        let expected = lifecycle
            .issues
            .iter()
            .find(|issue| issue.effect_id == effect_id)
            .ok_or_else(|| invalid("S1 retained V3 issue is orphaned from V2 lifecycle"))?;
        decode_and_validate_record(bytes, name, expected)?;
        by_effect.insert(effect_id, ());
    }

    for expected in &lifecycle.issues {
        if by_effect.remove(&expected.effect_id).is_none() {
            return Err(invalid(
                "S1 retained V3 issue is missing for a V2 issued record",
            ));
        }
    }
    if !by_effect.is_empty() {
        return Err(invalid("S1 retained V3 issue roster contains orphans"));
    }
    lifecycle.revalidate()
}

fn issued_binding(
    event: &DisposableLifecycleEventV2,
) -> Option<(u64, EffectKindV3, EffectPurposeV3)> {
    match event {
        DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id } => Some((
            *effect_id,
            EffectKindV3::Create,
            EffectPurposeV3::ForwardFlow,
        )),
        DisposableLifecycleEventV2::AttachIssuedOrUncertain { effect_id } => Some((
            *effect_id,
            EffectKindV3::Attach,
            EffectPurposeV3::ForwardFlow,
        )),
        DisposableLifecycleEventV2::MountIssuedOrUncertain { effect_id } => Some((
            *effect_id,
            EffectKindV3::Mount,
            EffectPurposeV3::ForwardFlow,
        )),
        DisposableLifecycleEventV2::UnmountIssuedOrUncertain { effect_id, purpose } => {
            Some((*effect_id, EffectKindV3::Unmount, map_purpose(*purpose)))
        }
        DisposableLifecycleEventV2::EjectIssuedOrUncertain { effect_id, purpose } => {
            Some((*effect_id, EffectKindV3::Eject, map_purpose(*purpose)))
        }
        _ => None,
    }
}

fn map_purpose(purpose: EffectPurposeV2) -> EffectPurposeV3 {
    match purpose {
        EffectPurposeV2::ForwardFlow => EffectPurposeV3::ForwardFlow,
        EffectPurposeV2::Reconciliation => EffectPurposeV3::RestartReconciliation,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PublishCutpointV3 {
    TemporaryCreated,
    BytesWritten,
    FileSynced,
    Renamed,
    DirectorySynced,
    FinalReopened,
    FinalReplayed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FilesystemBindingV3 {
    birthtime_nanoseconds: i64,
    birthtime_seconds: i64,
    ctime_nanoseconds: i64,
    ctime_seconds: i64,
    dev: u64,
    flags: u32,
    generation: u32,
    gid: u32,
    inode: u64,
    mode: u32,
    mtime_nanoseconds: i64,
    mtime_seconds: i64,
    nlink: u64,
    size: u64,
    uid: u32,
}

struct IssueFileCapsuleV3 {
    binding: FilesystemBindingV3,
    bytes: Vec<u8>,
    file: File,
    name: String,
    record: IssuedEffectRecordV3,
    record_sha256: String,
    s1_adopted: bool,
}

/// Owns the exact issue directory roster and every issue descriptor.  It is
/// intentionally process-local and cannot yield raw descriptors.
pub(crate) struct DurableEffectIssueStoreV3 {
    directory: File,
    directory_binding: FilesystemBindingV3,
    expected_gid: u32,
    expected_uid: u32,
    issues: Vec<IssueFileCapsuleV3>,
    operation_directory: File,
    operation_directory_binding: FilesystemBindingV3,
    operation_nonce: String,
    poisoned: bool,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// One normal-path issue capability.  The borrowed store keeps the issue file
/// and the whole closed-world directory roster alive.  No constructor exists
/// for a replayed/crash-uncertain issue.
pub(crate) struct RetainedDurableEffectIssueV3<'store> {
    index: usize,
    store: &'store mut DurableEffectIssueStoreV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Owned identity of one exact V2/V3 issue pair after the V3 file has been
/// durably replayed and admitted into S1.  It deliberately owns no descriptor
/// and exposes no record, command, or digest projection.  A later consumer
/// must move it through `AdoptedIssuedEffectUseSinkV3`, which replays the whole
/// issue store and the exact retained inode before producing runner material.
pub(crate) struct AdoptedIssuedEffectKeyV3<K> {
    identity: AdoptedIssuedEffectIdentityV3<K>,
    plan: SealedOwnedCollectorEffectPlanV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

struct AdoptedIssuedEffectIdentityV3<K> {
    effect_id: u64,
    issue_binding: FilesystemBindingV3,
    issue_name: String,
    operation_nonce: String,
    record: IssuedEffectRecordV3,
    record_canonical_bytes: Vec<u8>,
    record_sha256: String,
    _kind: PhantomData<fn() -> K>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

enum SealedOwnedCollectorEffectPlanV3 {
    Unmount(SealedUnmountEffectPlanV3),
    Eject(SealedEjectEffectPlanV3),
    #[cfg(test)]
    Synthetic,
}

/// One-use whole-store replay boundary for an adopted issue key.  Construction
/// requires the lifecycle-store-private read seal; holding a copied digest is
/// therefore insufficient to use an owned key after its short persistence
/// borrow has ended.
pub(crate) struct AdoptedIssuedEffectUseSinkV3<'store> {
    store: &'store mut DurableEffectIssueStoreV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Exact inert dispatch material produced only after a moved adopted key has
/// been revalidated against the complete issue store.  Its parts remain sealed
/// to the whole-operation owner; this module has no runner or effect primitive.
pub(crate) struct OwnedIssuedEffectDispatchMaterialV3<K> {
    record: IssuedEffectRecordV3,
    record_canonical_bytes: Vec<u8>,
    record_sha256: String,
    _kind: PhantomData<fn() -> K>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// One linear post-replay pair.  It keeps the exact collector plan beside the
/// runner material, so dispatch can move the material while the grant retains
/// the plan until callback/observation typestate consumes it.
pub(crate) struct AdoptedIssuedEffectDispatchV3<K> {
    material: OwnedIssuedEffectDispatchMaterialV3<K>,
    plan: SealedOwnedCollectorEffectPlanV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl<K> AdoptedIssuedEffectKeyV3<K> {
    /// Validate the lifecycle owner's expected effect ID without exposing an
    /// ID/digest projection that could be detached from this owned key.
    pub(crate) fn require_effect_id(
        &self,
        expected_effect_id: u64,
        _seal: &OperationIssueReadSealV3,
    ) -> Result<(), DurableEffectIssueStoreErrorV3> {
        if expected_effect_id == 0 || self.identity.effect_id != expected_effect_id {
            return Err(invalid(
                "owned adopted issue differs from the lifecycle-selected effect ID",
            ));
        }
        Ok(())
    }
}

/// Sealed one-shot consumer for S1-retained restart issue descriptors. Its
/// fields and constructor stay private to this module, so the lifecycle
/// source cannot be opened into a raw `File` tuple by another crate caller.
pub(crate) struct RetainedEffectIssueReplaySinkV3<'l, 's> {
    lifecycle: &'l VerifiedLifecycleIssueRosterV3,
    source: RetainedLifecycleIssueSourceV3<'s>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl RetainedEffectIssueReplaySinkV3<'_, '_> {
    pub(crate) fn consume(
        self,
        directory: File,
        expected_directory_inode: (u64, u64),
        retained_issues: Vec<(String, Vec<u8>, File, (u64, u64))>,
    ) -> Result<DurableEffectIssueStoreV3, DurableEffectIssueStoreErrorV3> {
        DurableEffectIssueStoreV3::open_existing_from_retained_parts(
            self.source,
            directory,
            expected_directory_inode,
            retained_issues,
            self.lifecycle,
        )
    }
}

enum CollectorIssueInputV3<'a> {
    Retained(&'a RetainedCollectorIssueBindingV3<'a>),
    #[cfg(test)]
    Synthetic(Option<String>),
}

impl CollectorIssueInputV3<'_> {
    fn unique_binding_sha256(
        &self,
        target: &LifecycleIssueBindingV3,
        epochs: &EffectEpochEvidenceV3,
    ) -> Result<String, DurableEffectIssueStoreErrorV3> {
        let prior = target
            .prior_collector
            .as_ref()
            .ok_or_else(|| invalid("V3 issue has no preceding durable collector observation"))?;
        match self {
            Self::Retained(binding) => {
                binding.revalidate().map_err(|error| {
                    invalid(format!(
                        "retained collector issue binding failed replay: {error}"
                    ))
                })?;
                if binding.operation_nonce() != target.operation_nonce
                    || binding.boot_session_uuid() != epochs.boot_session_uuid
                    || binding.receipt_sha256() != prior.receipt_sha256
                    || binding.lifecycle_record_sha256() != prior.record_sha256
                    || binding.lifecycle_record_sequence() != prior.sequence
                {
                    return Err(invalid(
                        "retained collector capability differs from the exact V2 predecessor",
                    ));
                }
                Ok(binding.unique_binding_sha256().to_string())
            }
            #[cfg(test)]
            Self::Synthetic(value) => value
                .clone()
                .ok_or_else(|| invalid("test collector binding digest is absent")),
        }
    }

    fn revalidate(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        match self {
            Self::Retained(binding) => binding.revalidate().map_err(|error| {
                invalid(format!(
                    "retained collector issue binding failed replay: {error}"
                ))
            }),
            #[cfg(test)]
            Self::Synthetic(_) => Ok(()),
        }
    }
}

impl DurableEffectIssueStoreV3 {
    #[cfg(test)]
    pub(crate) fn create_new(
        operation_directory: &File,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        lifecycle.revalidate()?;
        if !lifecycle.issues.is_empty() {
            return Err(invalid(
                "new V3 issue directory must exist before the first V2 issue",
            ));
        }
        let store = Self::create_prepublication(RetainedLifecycleIssueSourceV3::for_test(
            operation_directory,
            lifecycle.operation_nonce(),
            expected_uid,
            expected_gid,
        ))?;
        lifecycle.revalidate()?;
        Ok(store)
    }

    /// Create the mandatory empty issue directory for a new-format operation.
    /// S2 calls this while the outer operation directory still has its
    /// `.incoming-operation-*` name; no production raw lifecycle roster is
    /// accepted here.
    pub(crate) fn create_prepublication(
        source: RetainedLifecycleIssueSourceV3<'_>,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        let operation_directory = source.directory();
        let operation_nonce = source.operation_nonce();
        let expected_uid = source.expected_uid();
        let expected_gid = source.expected_gid();
        require_nonce(operation_nonce, "new V3 operation nonce")?;
        require_current_owner(expected_uid, expected_gid)?;
        let operation_directory = operation_directory.try_clone()?;
        let operation_before = validate_directory(
            &operation_directory,
            expected_uid,
            expected_gid,
            None,
            "new V3 operation directory",
        )?;
        reject_issue_directory_aliases(operation_directory.as_raw_fd(), false)?;
        require_absent(
            operation_directory.as_raw_fd(),
            ISSUE_DIRECTORY_TEMPORARY_NAME_V3,
        )?;
        require_absent(operation_directory.as_raw_fd(), ISSUE_DIRECTORY_NAME_V3)?;
        mkdirat_private(
            operation_directory.as_raw_fd(),
            ISSUE_DIRECTORY_TEMPORARY_NAME_V3,
        )?;
        let temporary = openat_directory(
            operation_directory.as_raw_fd(),
            ISSUE_DIRECTORY_TEMPORARY_NAME_V3,
        )?;
        if unsafe { libc::fchmod(temporary.as_raw_fd(), 0o700) } != 0 {
            return Err(io::Error::last_os_error().into());
        }
        let temporary_binding = validate_directory(
            &temporary,
            expected_uid,
            expected_gid,
            Some(operation_before.dev),
            "temporary V3 issue directory",
        )?;
        if !list_directory(temporary.as_raw_fd(), 0)?.is_empty()
            || named_binding(
                operation_directory.as_raw_fd(),
                ISSUE_DIRECTORY_TEMPORARY_NAME_V3,
            )? != temporary_binding
        {
            return Err(invalid(
                "temporary V3 issue directory changed before publication",
            ));
        }
        temporary.sync_all()?;
        rename_noreplace(
            operation_directory.as_raw_fd(),
            ISSUE_DIRECTORY_TEMPORARY_NAME_V3,
            operation_directory.as_raw_fd(),
            ISSUE_DIRECTORY_NAME_V3,
        )?;
        operation_directory.sync_all()?;
        let directory = openat_directory(operation_directory.as_raw_fd(), ISSUE_DIRECTORY_NAME_V3)?;
        let directory_binding = validate_directory(
            &directory,
            expected_uid,
            expected_gid,
            Some(operation_before.dev),
            "published V3 issue directory",
        )?;
        let renamed_temporary = binding(&temporary)?;
        if !same_object_across_rename(temporary_binding, renamed_temporary)
            || directory_binding != renamed_temporary
            || named_binding(operation_directory.as_raw_fd(), ISSUE_DIRECTORY_NAME_V3)?
                != directory_binding
            || !list_directory(directory.as_raw_fd(), 0)?.is_empty()
        {
            return Err(invalid(
                "published V3 issue directory failed final descriptor replay",
            ));
        }
        reject_issue_directory_aliases(operation_directory.as_raw_fd(), true)?;
        let operation_directory_binding = validate_directory(
            &operation_directory,
            expected_uid,
            expected_gid,
            None,
            "V3 operation directory after issue-root publication",
        )?;
        if named_binding(operation_directory.as_raw_fd(), ISSUE_DIRECTORY_NAME_V3)?
            != directory_binding
            || validate_directory(
                &directory,
                expected_uid,
                expected_gid,
                Some(operation_directory_binding.dev),
                "published V3 issue directory final replay",
            )? != directory_binding
            || !list_directory(directory.as_raw_fd(), 0)?.is_empty()
        {
            return Err(invalid(
                "published V3 issue directory changed across final replay",
            ));
        }
        reject_issue_directory_aliases(operation_directory.as_raw_fd(), true)?;
        Ok(Self {
            directory,
            directory_binding,
            expected_gid,
            expected_uid,
            issues: Vec::new(),
            operation_directory,
            operation_directory_binding,
            operation_nonce: operation_nonce.to_string(),
            poisoned: false,
            _not_send_or_sync: PhantomData,
        })
    }

    /// Reopen a mandatory new-format issue directory.  The entire V2 and V3
    /// rosters are replayed as a bijection.  Reopened issues remain evidence
    /// of issued-or-uncertain work; this API intentionally yields no fresh
    /// retained issue capability and therefore cannot authorize a resend.
    #[cfg(test)]
    pub(crate) fn open_existing_required(
        operation_directory: &File,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        Self::open_existing_directory(operation_directory, lifecycle, expected_uid, expected_gid)
    }

    /// Consume the exact S1-retained issue directory and file descriptors
    /// transferred through the sealed S2 restart wiring.  No path reopen or
    /// caller-provided format flag can substitute for these capsules.
    pub(crate) fn open_existing_from_retained_s1(
        source: RetainedLifecycleIssueSourceV3<'_>,
        retained: RetainedEffectIssueSourceV3,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        retained.transfer(RetainedEffectIssueReplaySinkV3 {
            lifecycle,
            source,
            _not_send_or_sync: PhantomData,
        })
    }

    fn open_existing_from_retained_parts(
        source: RetainedLifecycleIssueSourceV3<'_>,
        directory: File,
        expected_directory_inode: (u64, u64),
        retained_issues: Vec<(String, Vec<u8>, File, (u64, u64))>,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        if source.operation_nonce() != lifecycle.operation_nonce() {
            return Err(invalid(
                "sealed S2 operation differs from the lifecycle issue roster",
            ));
        }
        let expected_uid = source.expected_uid();
        let expected_gid = source.expected_gid();
        require_current_owner(expected_uid, expected_gid)?;
        let operation_directory = source.directory().try_clone()?;
        let operation_directory_binding = validate_directory(
            &operation_directory,
            expected_uid,
            expected_gid,
            None,
            "retained S1 V3 operation directory",
        )?;
        require_absent(
            operation_directory.as_raw_fd(),
            ISSUE_DIRECTORY_TEMPORARY_NAME_V3,
        )?;
        reject_issue_directory_aliases(operation_directory.as_raw_fd(), true)?;
        let directory_binding = validate_directory(
            &directory,
            expected_uid,
            expected_gid,
            Some(operation_directory_binding.dev),
            "retained S1 V3 issue directory",
        )?;
        if (directory_binding.dev, directory_binding.inode) != expected_directory_inode
            || named_binding(operation_directory.as_raw_fd(), ISSUE_DIRECTORY_NAME_V3)?
                != directory_binding
        {
            return Err(invalid(
                "S2 V3 issue directory differs from the exact retained S1 descriptor",
            ));
        }
        let issues = replay_retained_issue_directory(
            &directory,
            directory_binding,
            expected_uid,
            expected_gid,
            retained_issues,
            lifecycle,
        )?;
        let store = Self {
            directory,
            directory_binding,
            expected_gid,
            expected_uid,
            issues,
            operation_directory,
            operation_directory_binding,
            operation_nonce: lifecycle.operation_nonce.clone(),
            poisoned: false,
            _not_send_or_sync: PhantomData,
        };
        store.revalidate_against(lifecycle)?;
        Ok(store)
    }

    fn open_existing_directory(
        operation_directory: &File,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        require_current_owner(expected_uid, expected_gid)?;
        let operation_directory = operation_directory.try_clone()?;
        let operation_directory_binding = validate_directory(
            &operation_directory,
            expected_uid,
            expected_gid,
            None,
            "existing V3 operation directory",
        )?;
        require_absent(
            operation_directory.as_raw_fd(),
            ISSUE_DIRECTORY_TEMPORARY_NAME_V3,
        )?;
        reject_issue_directory_aliases(operation_directory.as_raw_fd(), true)?;
        let directory = openat_directory(operation_directory.as_raw_fd(), ISSUE_DIRECTORY_NAME_V3)?;
        let directory_binding = validate_directory(
            &directory,
            expected_uid,
            expected_gid,
            Some(operation_directory_binding.dev),
            "existing V3 issue directory",
        )?;
        if named_binding(operation_directory.as_raw_fd(), ISSUE_DIRECTORY_NAME_V3)?
            != directory_binding
        {
            return Err(invalid(
                "existing V3 issue directory differs from its retained descriptor",
            ));
        }
        let issues = replay_issue_directory(
            &directory,
            directory_binding,
            expected_uid,
            expected_gid,
            lifecycle,
        )?;
        let store = Self {
            directory,
            directory_binding,
            expected_gid,
            expected_uid,
            issues,
            operation_directory,
            operation_directory_binding,
            operation_nonce: lifecycle.operation_nonce.clone(),
            poisoned: false,
            _not_send_or_sync: PhantomData,
        };
        store.revalidate_against(lifecycle)?;
        Ok(store)
    }

    #[cfg(test)]
    pub(crate) fn persist<'store>(
        &'store mut self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        command: ExactDisposableCommandV3,
        epochs: EffectEpochEvidenceV3,
        unique_binding_sha256: Option<String>,
    ) -> Result<RetainedDurableEffectIssueV3<'store>, DurableEffectIssueStoreErrorV3> {
        self.persist_with_hook(
            lifecycle,
            command,
            epochs,
            CollectorIssueInputV3::Synthetic(unique_binding_sha256),
            |_| Ok(()),
        )
    }

    #[cfg(test)]
    pub(crate) fn persist_bound<'store>(
        &'store mut self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        collector: &RetainedCollectorIssueBindingV3<'_>,
        command: ExactDisposableCommandV3,
        epochs: EffectEpochEvidenceV3,
    ) -> Result<RetainedDurableEffectIssueV3<'store>, DurableEffectIssueStoreErrorV3> {
        self.persist_with_hook(
            lifecycle,
            command,
            epochs,
            CollectorIssueInputV3::Retained(collector),
            |_| Ok(()),
        )
    }

    /// Persist and S1-adopt the exact unmount derived by the owned collector
    /// plan.  No command, effect kind, purpose, collector digest, or unique
    /// binding is accepted from the caller.
    pub(crate) fn persist_unmount_adopted(
        &mut self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        plan: SealedUnmountEffectPlanV3,
        epochs: EffectEpochEvidenceV3,
        s1: EffectIssueAppendSinkV3<'_, '_>,
    ) -> Result<AdoptedIssuedEffectKeyV3<PersistedUnmountEffectV3>, DurableEffectIssueStoreErrorV3>
    {
        let identity = {
            let read = IssuePlanReadSealV3 { _private: () };
            let issue_plan = match plan.issue_plan(&read) {
                Ok(issue_plan) => issue_plan,
                Err(error) => {
                    self.poisoned = true;
                    return Err(invalid(format!(
                        "sealed unmount collector plan failed issue projection: {error}"
                    )));
                }
            };
            if let Err(error) = issue_plan.revalidate_for_issue(&read) {
                self.poisoned = true;
                return Err(invalid(format!(
                    "sealed unmount collector plan failed pre-persistence replay: {error}"
                )));
            }
            let identity =
                self.persist_typed_adopted_inner(lifecycle, &issue_plan, &read, epochs, s1)?;
            if let Err(error) = issue_plan.revalidate_for_issue(&read) {
                self.poisoned = true;
                return Err(DurableEffectIssueStoreErrorV3::PersistenceUncertain(
                    format!("sealed unmount collector plan changed after issue adoption: {error}"),
                ));
            }
            identity
        };
        Ok(AdoptedIssuedEffectKeyV3 {
            identity,
            plan: SealedOwnedCollectorEffectPlanV3::Unmount(plan),
            _not_send_or_sync: PhantomData,
        })
    }

    /// Eject counterpart of `persist_unmount_adopted`.  Keeping two
    /// concrete entrypoints makes the plan marker part of the function type and
    /// prevents dynamic kind selection at the durability boundary.
    pub(crate) fn persist_eject_adopted(
        &mut self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        plan: SealedEjectEffectPlanV3,
        epochs: EffectEpochEvidenceV3,
        s1: EffectIssueAppendSinkV3<'_, '_>,
    ) -> Result<AdoptedIssuedEffectKeyV3<PersistedEjectEffectV3>, DurableEffectIssueStoreErrorV3>
    {
        let identity = {
            let read = IssuePlanReadSealV3 { _private: () };
            let issue_plan = match plan.issue_plan(&read) {
                Ok(issue_plan) => issue_plan,
                Err(error) => {
                    self.poisoned = true;
                    return Err(invalid(format!(
                        "sealed eject collector plan failed issue projection: {error}"
                    )));
                }
            };
            if let Err(error) = issue_plan.revalidate_for_issue(&read) {
                self.poisoned = true;
                return Err(invalid(format!(
                    "sealed eject collector plan failed pre-persistence replay: {error}"
                )));
            }
            let identity =
                self.persist_typed_adopted_inner(lifecycle, &issue_plan, &read, epochs, s1)?;
            if let Err(error) = issue_plan.revalidate_for_issue(&read) {
                self.poisoned = true;
                return Err(DurableEffectIssueStoreErrorV3::PersistenceUncertain(
                    format!("sealed eject collector plan changed after issue adoption: {error}"),
                ));
            }
            identity
        };
        Ok(AdoptedIssuedEffectKeyV3 {
            identity,
            plan: SealedOwnedCollectorEffectPlanV3::Eject(plan),
            _not_send_or_sync: PhantomData,
        })
    }

    fn persist_typed_adopted_inner<K>(
        &mut self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        plan: &SealedCollectorEffectIssuePlanV3<'_, K>,
        read: &IssuePlanReadSealV3,
        epochs: EffectEpochEvidenceV3,
        s1: EffectIssueAppendSinkV3<'_, '_>,
    ) -> Result<AdoptedIssuedEffectIdentityV3<K>, DurableEffectIssueStoreErrorV3>
    where
        K: IssuedEffectKindSpecV3,
    {
        let command = plan.command_for_issue(read).clone();
        let collector = plan.collector_binding_for_issue(read);
        let mut retained = self.persist_with_hook(
            lifecycle,
            command,
            epochs,
            CollectorIssueInputV3::Retained(collector),
            |_| Ok(()),
        )?;
        if let Err(error) = retained.adopt_into_s1(s1) {
            retained.store.poisoned = true;
            return Err(DurableEffectIssueStoreErrorV3::PersistenceUncertain(
                format!("exact V3 issue was durable but S1 adoption failed: {error}"),
            ));
        }
        if let Err(error) = retained.require_s1_adopted() {
            retained.store.poisoned = true;
            return Err(DurableEffectIssueStoreErrorV3::PersistenceUncertain(
                format!("exact V3 issue lost its S1 adoption: {error}"),
            ));
        }
        retained.into_adopted_identity()
    }

    fn persist_with_hook<'store, F>(
        &'store mut self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        command: ExactDisposableCommandV3,
        epochs: EffectEpochEvidenceV3,
        collector: CollectorIssueInputV3<'_>,
        mut hook: F,
    ) -> Result<RetainedDurableEffectIssueV3<'store>, DurableEffectIssueStoreErrorV3>
    where
        F: FnMut(PublishCutpointV3) -> io::Result<()>,
    {
        epochs.validate()?;
        if self.poisoned {
            return Err(invalid(
                "issue store is poisoned; exact restart replay is required",
            ));
        }
        if let Err(error) = self.revalidate_files() {
            self.poisoned = true;
            return Err(error);
        }
        self.validate_existing_lifecycle_prefix(lifecycle)?;
        let target = lifecycle
            .issues
            .get(self.issues.len())
            .ok_or_else(|| invalid("no next durable V2 issued record exists"))?;
        if lifecycle.issues.len() != self.issues.len() + 1 {
            return Err(invalid(
                "exactly one new V2 issued record must precede one V3 issue",
            ));
        }
        if lifecycle.terminal_record_sha256 != target.record_sha256 {
            return Err(invalid(
                "the V2 issued record must be the durable lifecycle tip at V3 publication",
            ));
        }
        let prior_collector = target
            .prior_collector
            .as_ref()
            .ok_or_else(|| invalid("V3 issue has no preceding durable collector observation"))?;
        if prior_collector.sequence >= target.sequence
            || prior_collector.boot_session_uuid != epochs.boot_session_uuid
        {
            return Err(invalid(
                "collector observation does not precede this issue in the current boot",
            ));
        }
        command.validate()?;
        if command.kind() != target.kind {
            return Err(invalid(
                "typed command kind differs from the durable V2 issued event",
            ));
        }
        let unique_binding_sha256 = collector.unique_binding_sha256(target, &epochs)?;
        require_digest(&unique_binding_sha256, "unique collector binding digest")?;
        let command_bytes = canonical_json(&command)?;
        if command_bytes.len() > MAX_COMMAND_BYTES_V3 {
            return Err(invalid("typed command exceeds its fixed byte bound"));
        }
        let command_canonical_json = String::from_utf8(command_bytes.clone())
            .map_err(|_| invalid("canonical typed command is not UTF-8 JSON"))?;
        let record = IssuedEffectRecordV3 {
            authority: DisposableAuthorityV2::none(),
            boot_session_uuid: epochs.boot_session_uuid,
            command,
            command_canonical_json,
            command_sha256: sha256(&command_bytes),
            effect_id: target.effect_id,
            effect_kind: target.kind,
            lifecycle_issue_record_sha256: target.record_sha256.clone(),
            lifecycle_issue_sequence: target.sequence,
            lifecycle_tip_before_sha256: target.previous_tip_sha256.clone(),
            operation_nonce: self.operation_nonce.clone(),
            prior_collector_lifecycle_record_sha256: prior_collector.record_sha256.clone(),
            prior_collector_lifecycle_sequence: prior_collector.sequence,
            prior_collector_receipt_sha256: prior_collector.receipt_sha256.clone(),
            process_epoch_nonce: epochs.process_epoch_nonce,
            process_epoch_sha256: epochs.process_epoch_sha256,
            purpose: target.purpose,
            runner_epoch_nonce: epochs.runner_epoch_nonce,
            runner_epoch_sha256: epochs.runner_epoch_sha256,
            runner_hello_sha256: epochs.runner_hello_sha256,
            runner_kernel_start_microseconds: epochs.runner_kernel_start_microseconds,
            runner_pid: epochs.runner_pid,
            runner_pre_hello_fd_census_sha256: epochs.runner_pre_hello_fd_census_sha256,
            runner_transport_sha256: epochs.runner_transport_sha256,
            schema: ISSUE_SCHEMA_V3.to_string(),
            schema_version: 3,
            supervisor_kernel_start_microseconds: epochs.supervisor_kernel_start_microseconds,
            supervisor_parent_pid: epochs.supervisor_parent_pid,
            supervisor_pid: epochs.supervisor_pid,
            unique_binding_sha256: Some(unique_binding_sha256),
        };
        record.validate_against(target)?;
        let bytes = canonical_json(&record)?;
        if bytes.is_empty() || bytes.len() > MAX_ISSUE_BYTES_V3 {
            return Err(invalid("canonical V3 issue exceeds its fixed byte bound"));
        }
        let previous_total = aggregate_issue_bytes(&self.issues)?;
        previous_total
            .checked_add(bytes.len())
            .filter(|total| *total <= MAX_TOTAL_ISSUE_BYTES_V3)
            .ok_or_else(|| invalid("V3 issue aggregate exceeds its fixed byte bound"))?;
        let record_sha256 = sha256(&bytes);
        let final_name = issue_name(record.effect_id, &record_sha256);
        let temporary_name = format!(".incoming-{final_name}");
        let result = (|| {
            require_absent(self.directory.as_raw_fd(), &temporary_name)?;
            require_absent(self.directory.as_raw_fd(), &final_name)?;
            let mut temporary = createat_issue_file(
                self.directory.as_raw_fd(),
                &temporary_name,
                self.expected_uid,
                self.expected_gid,
                self.directory_binding.dev,
            )?;
            hook(PublishCutpointV3::TemporaryCreated)?;
            temporary.write_all(&bytes)?;
            hook(PublishCutpointV3::BytesWritten)?;
            temporary.sync_all()?;
            hook(PublishCutpointV3::FileSynced)?;
            let temporary_binding = validate_issue_file(
                &temporary,
                self.expected_uid,
                self.expected_gid,
                self.directory_binding.dev,
                bytes.len(),
                "temporary V3 issue",
            )?;
            if named_binding(self.directory.as_raw_fd(), &temporary_name)? != temporary_binding
                || read_exact_file(&temporary, temporary_binding)? != bytes
            {
                return Err(invalid("temporary V3 issue changed before publication"));
            }
            rename_noreplace(
                self.directory.as_raw_fd(),
                &temporary_name,
                self.directory.as_raw_fd(),
                &final_name,
            )?;
            hook(PublishCutpointV3::Renamed)?;
            self.directory.sync_all()?;
            hook(PublishCutpointV3::DirectorySynced)?;
            let file = openat_issue_file(self.directory.as_raw_fd(), &final_name)?;
            hook(PublishCutpointV3::FinalReopened)?;
            let final_binding = validate_issue_file(
                &file,
                self.expected_uid,
                self.expected_gid,
                self.directory_binding.dev,
                bytes.len(),
                "final V3 issue",
            )?;
            let renamed_binding = binding(&temporary)?;
            if !same_object_across_rename(temporary_binding, renamed_binding)
                || final_binding != renamed_binding
                || named_binding(self.directory.as_raw_fd(), &final_name)? != final_binding
                || read_exact_file(&temporary, renamed_binding)? != bytes
                || read_exact_file(&file, final_binding)? != bytes
            {
                return Err(invalid("final V3 issue failed exact descriptor replay"));
            }
            let mut expected_names = self
                .issues
                .iter()
                .map(|issue| issue.name.clone())
                .collect::<Vec<_>>();
            expected_names.push(final_name.clone());
            expected_names.sort();
            if list_directory(self.directory.as_raw_fd(), MAX_ISSUES_V3)? != expected_names {
                return Err(invalid(
                    "V3 issue directory roster changed during publication",
                ));
            }
            let directory_binding = validate_directory(
                &self.directory,
                self.expected_uid,
                self.expected_gid,
                Some(self.operation_directory_binding.dev),
                "V3 issue directory after publication",
            )?;
            if named_binding(
                self.operation_directory.as_raw_fd(),
                ISSUE_DIRECTORY_NAME_V3,
            )? != directory_binding
            {
                return Err(invalid(
                    "V3 issue directory identity changed during publication",
                ));
            }
            let decoded = decode_and_validate_record(&bytes, &final_name, target)?;
            if decoded != record {
                return Err(invalid("final V3 issue differs from the issued value"));
            }
            for issue in &self.issues {
                if binding(&issue.file)? != issue.binding
                    || named_binding(self.directory.as_raw_fd(), &issue.name)? != issue.binding
                    || read_exact_file(&issue.file, issue.binding)? != issue.bytes
                {
                    return Err(invalid(
                        "an earlier retained V3 issue changed during publication",
                    ));
                }
            }
            if validate_issue_file(
                &file,
                self.expected_uid,
                self.expected_gid,
                directory_binding.dev,
                bytes.len(),
                "final V3 issue second replay",
            )? != final_binding
                || named_binding(self.directory.as_raw_fd(), &final_name)? != final_binding
                || read_exact_file(&file, final_binding)? != bytes
                || list_directory(self.directory.as_raw_fd(), MAX_ISSUES_V3)? != expected_names
                || validate_directory(
                    &self.directory,
                    self.expected_uid,
                    self.expected_gid,
                    Some(self.operation_directory_binding.dev),
                    "V3 issue directory final publication replay",
                )? != directory_binding
                || named_binding(
                    self.operation_directory.as_raw_fd(),
                    ISSUE_DIRECTORY_NAME_V3,
                )? != directory_binding
            {
                return Err(invalid(
                    "V3 issue or directory changed across final publication replay",
                ));
            }
            lifecycle.revalidate()?;
            collector.revalidate()?;
            hook(PublishCutpointV3::FinalReplayed)?;
            Ok((
                IssueFileCapsuleV3 {
                    binding: final_binding,
                    bytes,
                    file,
                    name: final_name,
                    record,
                    record_sha256,
                    s1_adopted: false,
                },
                directory_binding,
            ))
        })();
        let (capsule, directory_binding) = match result {
            Ok(value) => value,
            Err(error) => {
                self.poisoned = true;
                return Err(DurableEffectIssueStoreErrorV3::PersistenceUncertain(
                    error.to_string(),
                ));
            }
        };
        self.directory_binding = directory_binding;
        self.issues.push(capsule);
        let index = self.issues.len() - 1;
        Ok(RetainedDurableEffectIssueV3 {
            index,
            store: self,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn replayed_issue_sealed(
        &self,
        effect_id: u64,
        _seal: &OperationIssueReadSealV3,
    ) -> Option<&IssuedEffectRecordV3> {
        self.issues
            .iter()
            .find(|issue| issue.record.effect_id == effect_id)
            .map(|issue| &issue.record)
    }

    #[cfg(test)]
    pub(crate) fn replayed_issue(&self, effect_id: u64) -> Option<&IssuedEffectRecordV3> {
        self.issues
            .iter()
            .find(|issue| issue.record.effect_id == effect_id)
            .map(|issue| &issue.record)
    }

    pub(crate) fn revalidate_s1_adopted(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.revalidate_files()?;
        if self.issues.iter().any(|issue| !issue.s1_adopted) {
            return Err(invalid(
                "V3 issue roster contains a record not adopted by the retained S1 census",
            ));
        }
        Ok(())
    }

    /// Seal one later use of an owned adopted key to this exact live issue
    /// store.  The caller must itself own the whole-operation read seal; this
    /// prevents a digest-only projection from becoming runner material.
    pub(crate) fn adopted_issue_use_sink<'store>(
        &'store mut self,
        _seal: &OperationIssueReadSealV3,
    ) -> Result<AdoptedIssuedEffectUseSinkV3<'store>, DurableEffectIssueStoreErrorV3> {
        if let Err(error) = self.revalidate_s1_adopted() {
            self.poisoned = true;
            return Err(error);
        }
        Ok(AdoptedIssuedEffectUseSinkV3 {
            store: self,
            _not_send_or_sync: PhantomData,
        })
    }

    #[cfg(test)]
    fn adopted_issue_use_sink_for_test(
        &mut self,
    ) -> Result<AdoptedIssuedEffectUseSinkV3<'_>, DurableEffectIssueStoreErrorV3> {
        if let Err(error) = self.revalidate_s1_adopted() {
            self.poisoned = true;
            return Err(error);
        }
        Ok(AdoptedIssuedEffectUseSinkV3 {
            store: self,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn revalidate_required(
        &self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
    ) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.revalidate_against(lifecycle)
    }

    pub(crate) fn revalidate_prepared_empty(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.revalidate_files()?;
        if !self.issues.is_empty() {
            return Err(invalid(
                "pre-issue operation unexpectedly owns a V3 issue record",
            ));
        }
        Ok(())
    }

    pub(crate) fn transfer_prepared_operation_to_s1(
        &self,
        sink: FreshOperationAdmissionSinkV3<'_, '_>,
        operation_directory: File,
        final_name: String,
        prepared_manifest: Option<PreparedManifestS1TransferV3>,
        restart_admissions: Option<RestartAdmissionRootS1TransferV3>,
    ) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.revalidate_prepared_empty()?;
        sink.retain(
            operation_directory,
            final_name,
            self.directory.try_clone()?,
            prepared_manifest,
            restart_admissions,
        )
        .map_err(|error| {
            invalid(format!(
                "S1 rejected the exact freshly published operation capsule: {error}"
            ))
        })
    }

    pub(crate) fn poisoned(&self) -> bool {
        self.poisoned
    }

    fn revalidate_existing_prefix(
        &self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
    ) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.revalidate_files()?;
        self.validate_existing_lifecycle_prefix(lifecycle)
    }

    fn validate_existing_lifecycle_prefix(
        &self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
    ) -> Result<(), DurableEffectIssueStoreErrorV3> {
        lifecycle.revalidate()?;
        if lifecycle.operation_nonce != self.operation_nonce
            || lifecycle.issues.len() < self.issues.len()
        {
            return Err(invalid(
                "lifecycle replay differs from the retained issue store",
            ));
        }
        for (capsule, expected) in self.issues.iter().zip(&lifecycle.issues) {
            capsule.record.validate_against(expected)?;
        }
        Ok(())
    }

    fn revalidate_against(
        &self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
    ) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.revalidate_existing_prefix(lifecycle)?;
        if lifecycle.issues.len() != self.issues.len() {
            return Err(invalid(
                "V2 issued roster and V3 issue roster are not an exact bijection",
            ));
        }
        Ok(())
    }

    fn revalidate_files(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        let operation = validate_directory(
            &self.operation_directory,
            self.expected_uid,
            self.expected_gid,
            None,
            "retained V3 operation directory",
        )?;
        let directory = validate_directory(
            &self.directory,
            self.expected_uid,
            self.expected_gid,
            Some(operation.dev),
            "retained V3 issue directory",
        )?;
        if !same_directory_object(operation, self.operation_directory_binding)
            || directory != self.directory_binding
            || named_binding(
                self.operation_directory.as_raw_fd(),
                ISSUE_DIRECTORY_NAME_V3,
            )? != directory
        {
            return Err(invalid("retained V3 issue directory identity changed"));
        }
        require_absent(
            self.operation_directory.as_raw_fd(),
            ISSUE_DIRECTORY_TEMPORARY_NAME_V3,
        )?;
        reject_issue_directory_aliases(self.operation_directory.as_raw_fd(), true)?;
        let mut expected_names = self
            .issues
            .iter()
            .map(|issue| issue.name.clone())
            .collect::<Vec<_>>();
        expected_names.sort();
        if list_directory(self.directory.as_raw_fd(), MAX_ISSUES_V3)? != expected_names {
            return Err(invalid("retained V3 issue roster changed"));
        }
        for issue in &self.issues {
            if binding(&issue.file)? != issue.binding
                || named_binding(self.directory.as_raw_fd(), &issue.name)? != issue.binding
                || read_exact_file(&issue.file, issue.binding)? != issue.bytes
                || sha256(&issue.bytes) != issue.record_sha256
            {
                return Err(invalid("retained V3 issue file changed"));
            }
            validate_issue_file(
                &issue.file,
                self.expected_uid,
                self.expected_gid,
                directory.dev,
                issue.bytes.len(),
                "retained V3 issue file",
            )?;
            if named_binding(self.directory.as_raw_fd(), &issue.name)? != issue.binding {
                return Err(invalid(
                    "retained V3 issue pathname changed during final replay",
                ));
            }
        }
        if list_directory(self.directory.as_raw_fd(), MAX_ISSUES_V3)? != expected_names
            || validate_directory(
                &self.directory,
                self.expected_uid,
                self.expected_gid,
                Some(operation.dev),
                "retained V3 issue directory final replay",
            )? != directory
            || named_binding(
                self.operation_directory.as_raw_fd(),
                ISSUE_DIRECTORY_NAME_V3,
            )? != directory
            || !same_directory_object(
                validate_directory(
                    &self.operation_directory,
                    self.expected_uid,
                    self.expected_gid,
                    None,
                    "retained V3 operation directory final replay",
                )?,
                self.operation_directory_binding,
            )
        {
            return Err(invalid(
                "retained V3 issue store changed across final replay",
            ));
        }
        Ok(())
    }
}

impl IssuedEffectRecordV3 {
    pub(crate) fn boot_session_uuid(&self) -> &str {
        &self.boot_session_uuid
    }

    pub(crate) fn command(&self) -> &ExactDisposableCommandV3 {
        &self.command
    }

    pub(crate) fn command_canonical_bytes(&self) -> &[u8] {
        self.command_canonical_json.as_bytes()
    }

    pub(crate) fn command_sha256(&self) -> &str {
        &self.command_sha256
    }

    pub(crate) fn effect_id(&self) -> u64 {
        self.effect_id
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    pub(crate) fn process_epoch_sha256(&self) -> &str {
        &self.process_epoch_sha256
    }

    pub(crate) fn lifecycle_tip_before_sha256(&self) -> &str {
        &self.lifecycle_tip_before_sha256
    }

    pub(crate) fn purpose(&self) -> EffectPurposeV3 {
        self.purpose
    }

    pub(crate) fn runner_epoch_sha256(&self) -> &str {
        &self.runner_epoch_sha256
    }

    pub(crate) fn runner_pid(&self) -> u32 {
        self.runner_pid
    }

    pub(crate) fn runner_kernel_start_microseconds(&self) -> u64 {
        self.runner_kernel_start_microseconds
    }

    pub(crate) fn runner_hello_sha256(&self) -> &str {
        &self.runner_hello_sha256
    }

    pub(crate) fn runner_pre_hello_fd_census_sha256(&self) -> &str {
        &self.runner_pre_hello_fd_census_sha256
    }

    pub(crate) fn runner_transport_sha256(&self) -> &str {
        &self.runner_transport_sha256
    }

    pub(crate) fn supervisor_pid(&self) -> u32 {
        self.supervisor_pid
    }

    pub(crate) fn supervisor_parent_pid(&self) -> u32 {
        self.supervisor_parent_pid
    }

    pub(crate) fn supervisor_kernel_start_microseconds(&self) -> u64 {
        self.supervisor_kernel_start_microseconds
    }

    fn validate_against(
        &self,
        expected: &LifecycleIssueBindingV3,
    ) -> Result<(), DurableEffectIssueStoreErrorV3> {
        if self.schema != ISSUE_SCHEMA_V3
            || self.schema_version != 3
            || self.authority.any()
            || self.effect_id == 0
            || self.effect_id != expected.effect_id
            || self.effect_kind != expected.kind
            || self.operation_nonce != expected.operation_nonce
            || self.purpose != expected.purpose
            || self.lifecycle_issue_sequence != expected.sequence
            || self.lifecycle_issue_record_sha256 != expected.record_sha256
            || self.lifecycle_tip_before_sha256 != expected.previous_tip_sha256
        {
            return Err(invalid(
                "V3 issue differs from its exact V2 issued-or-uncertain record",
            ));
        }
        require_nonce(&self.operation_nonce, "effect issue operation nonce")?;
        require_uuid(&self.boot_session_uuid, "effect issue boot session UUID")?;
        require_digest(&self.command_sha256, "typed command digest")?;
        require_digest(
            &self.lifecycle_issue_record_sha256,
            "lifecycle issue record digest",
        )?;
        require_digest(
            &self.lifecycle_tip_before_sha256,
            "lifecycle tip before issue",
        )?;
        require_digest(
            &self.prior_collector_lifecycle_record_sha256,
            "prior collector lifecycle record digest",
        )?;
        require_digest(
            &self.prior_collector_receipt_sha256,
            "prior collector receipt digest",
        )?;
        require_nonce(&self.process_epoch_nonce, "process epoch nonce")?;
        require_digest(&self.process_epoch_sha256, "process epoch digest")?;
        require_nonce(&self.runner_epoch_nonce, "runner epoch nonce")?;
        require_digest(&self.runner_epoch_sha256, "runner epoch digest")?;
        require_digest(&self.runner_hello_sha256, "runner hello digest")?;
        require_digest(
            &self.runner_pre_hello_fd_census_sha256,
            "runner pre-hello FD census digest",
        )?;
        require_digest(&self.runner_transport_sha256, "runner transport digest")?;
        if self.process_epoch_nonce == self.runner_epoch_nonce
            || self.process_epoch_sha256 == self.runner_epoch_sha256
            || self.runner_epoch_sha256 != self.runner_hello_sha256
            || self.runner_pid == 0
            || self.runner_kernel_start_microseconds == 0
            || self.supervisor_pid == 0
            || self.supervisor_kernel_start_microseconds == 0
            || self.supervisor_pid == self.runner_pid
            || self.supervisor_parent_pid == self.supervisor_pid
        {
            return Err(invalid(
                "process/runner epochs, runner identity, hello, or transport binding is invalid",
            ));
        }
        if let Some(unique_binding_sha256) = &self.unique_binding_sha256 {
            require_digest(unique_binding_sha256, "unique collector binding digest")?;
        }
        let prior = expected
            .prior_collector
            .as_ref()
            .ok_or_else(|| invalid("V2 issue has no preceding durable collector observation"))?;
        if self.prior_collector_lifecycle_sequence != prior.sequence
            || self.prior_collector_lifecycle_record_sha256 != prior.record_sha256
            || self.prior_collector_receipt_sha256 != prior.receipt_sha256
            || self.boot_session_uuid != prior.boot_session_uuid
            || prior.sequence >= self.lifecycle_issue_sequence
        {
            return Err(invalid(
                "V3 issue differs from its exact prior collector observation",
            ));
        }
        self.command.validate()?;
        let command_bytes = canonical_json(&self.command)?;
        if self.command.kind() != self.effect_kind
            || self.command_canonical_json.as_bytes() != command_bytes
            || self.command_sha256 != sha256(&command_bytes)
        {
            return Err(invalid(
                "typed command value, canonical bytes, kind, or digest drifted",
            ));
        }
        Ok(())
    }
}

impl RetainedDurableEffectIssueV3<'_> {
    pub(crate) fn effect_id(&self) -> u64 {
        self.store.issues[self.index].record.effect_id
    }

    pub(crate) fn sealed_record_sha256(&self, _seal: &OperationIssueReadSealV3) -> &str {
        &self.store.issues[self.index].record_sha256
    }

    pub(crate) fn sealed_record(&self, _seal: &OperationIssueReadSealV3) -> &IssuedEffectRecordV3 {
        &self.store.issues[self.index].record
    }

    pub(crate) fn sealed_record_canonical_bytes(&self, _seal: &OperationIssueReadSealV3) -> &[u8] {
        &self.store.issues[self.index].bytes
    }

    #[cfg(test)]
    pub(crate) fn record_sha256(&self) -> &str {
        &self.store.issues[self.index].record_sha256
    }

    #[cfg(test)]
    pub(crate) fn record(&self) -> &IssuedEffectRecordV3 {
        &self.store.issues[self.index].record
    }

    #[cfg(test)]
    pub(crate) fn record_canonical_bytes(&self) -> &[u8] {
        &self.store.issues[self.index].bytes
    }

    pub(crate) fn revalidate(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.store.revalidate_files()
    }

    pub(crate) fn adopt_into_s1(
        &mut self,
        sink: EffectIssueAppendSinkV3<'_, '_>,
    ) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.revalidate()?;
        if self.store.issues[self.index].s1_adopted {
            return Err(invalid("V3 issue capsule was already adopted by S1"));
        }
        let issue = &self.store.issues[self.index];
        sink.retain(
            self.store.directory.try_clone()?,
            issue.file.try_clone()?,
            issue.bytes.clone(),
            self.store.operation_nonce.clone(),
            issue.name.clone(),
            issue.record_sha256.clone(),
        )
        .map_err(|error| invalid(format!("S1 rejected the exact V3 issue capsule: {error}")))?;
        self.store.issues[self.index].s1_adopted = true;
        self.revalidate()
    }

    pub(crate) fn require_s1_adopted(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.revalidate()?;
        if !self.store.issues[self.index].s1_adopted {
            return Err(invalid("V3 issue capsule has not been adopted by S1"));
        }
        Ok(())
    }

    #[cfg(test)]
    fn into_adopted_key_for_test<K>(
        self,
    ) -> Result<AdoptedIssuedEffectKeyV3<K>, DurableEffectIssueStoreErrorV3>
    where
        K: IssuedEffectKindSpecV3,
    {
        // Production reaches this state only through the retained S1 append
        // sink.  Unit tests use the explicit test-only edge to isolate owned
        // key replay without constructing the full S1 assessment fixture.
        self.store.issues[self.index].s1_adopted = true;
        Ok(AdoptedIssuedEffectKeyV3 {
            identity: self.into_adopted_identity()?,
            plan: SealedOwnedCollectorEffectPlanV3::Synthetic,
            _not_send_or_sync: PhantomData,
        })
    }

    fn into_adopted_identity<K>(
        self,
    ) -> Result<AdoptedIssuedEffectIdentityV3<K>, DurableEffectIssueStoreErrorV3>
    where
        K: IssuedEffectKindSpecV3,
    {
        if let Err(error) = self.require_s1_adopted() {
            self.store.poisoned = true;
            return Err(DurableEffectIssueStoreErrorV3::PersistenceUncertain(
                format!("owned issue key could not retain exact S1 adoption: {error}"),
            ));
        }
        let built = (|| {
            let issue = &self.store.issues[self.index];
            let decoded: IssuedEffectRecordV3 =
                serde_json::from_slice(&issue.bytes).map_err(|error| {
                    invalid(format!("adopted V3 issue bytes no longer decode: {error}"))
                })?;
            if issue.record.effect_kind != K::KIND
                || issue.record.command.kind() != K::KIND
                || issue.record.unique_binding_sha256.is_none()
                || decoded != issue.record
                || canonical_json(&decoded)? != issue.bytes
                || sha256(&issue.bytes) != issue.record_sha256
                || issue_name(issue.record.effect_id, &issue.record_sha256) != issue.name
            {
                return Err(invalid(
                    "adopted V3 issue kind, command, or collector binding differs from its typed plan",
                ));
            }
            Ok(AdoptedIssuedEffectIdentityV3 {
                effect_id: issue.record.effect_id,
                issue_binding: issue.binding,
                issue_name: issue.name.clone(),
                operation_nonce: self.store.operation_nonce.clone(),
                record: issue.record.clone(),
                record_canonical_bytes: issue.bytes.clone(),
                record_sha256: issue.record_sha256.clone(),
                _kind: PhantomData,
                _not_send_or_sync: PhantomData,
            })
        })();
        match built {
            Ok(key) => Ok(key),
            Err(error) => {
                self.store.poisoned = true;
                Err(DurableEffectIssueStoreErrorV3::PersistenceUncertain(
                    error.to_string(),
                ))
            }
        }
    }
}

impl AdoptedIssuedEffectUseSinkV3<'_> {
    pub(crate) fn consume_unmount(
        self,
        key: AdoptedIssuedEffectKeyV3<PersistedUnmountEffectV3>,
    ) -> Result<
        AdoptedIssuedEffectDispatchV3<PersistedUnmountEffectV3>,
        DurableEffectIssueStoreErrorV3,
    > {
        let AdoptedIssuedEffectKeyV3 { identity, plan, .. } = key;
        let plan = match plan {
            SealedOwnedCollectorEffectPlanV3::Unmount(plan) => plan,
            SealedOwnedCollectorEffectPlanV3::Eject(_) => {
                self.store.poisoned = true;
                return Err(invalid(
                    "typed unmount issue key carried an eject collector plan",
                ));
            }
            #[cfg(test)]
            SealedOwnedCollectorEffectPlanV3::Synthetic => {
                self.store.poisoned = true;
                return Err(invalid(
                    "test-only synthetic issue key entered production unmount dispatch",
                ));
            }
        };
        let read = IssuePlanReadSealV3 { _private: () };
        let issue_plan = match plan.issue_plan(&read) {
            Ok(issue_plan) => issue_plan,
            Err(error) => {
                self.store.poisoned = true;
                return Err(invalid(format!(
                    "sealed unmount collector plan failed dispatch projection: {error}"
                )));
            }
        };
        if let Err(error) = issue_plan.revalidate_for_issue(&read) {
            self.store.poisoned = true;
            return Err(invalid(format!(
                "sealed unmount collector plan failed dispatch replay: {error}"
            )));
        }
        let material = self.consume_inner(identity, &issue_plan, &read)?;
        Ok(AdoptedIssuedEffectDispatchV3 {
            material,
            plan: SealedOwnedCollectorEffectPlanV3::Unmount(plan),
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn consume_eject(
        self,
        key: AdoptedIssuedEffectKeyV3<PersistedEjectEffectV3>,
    ) -> Result<AdoptedIssuedEffectDispatchV3<PersistedEjectEffectV3>, DurableEffectIssueStoreErrorV3>
    {
        let AdoptedIssuedEffectKeyV3 { identity, plan, .. } = key;
        let plan = match plan {
            SealedOwnedCollectorEffectPlanV3::Eject(plan) => plan,
            SealedOwnedCollectorEffectPlanV3::Unmount(_) => {
                self.store.poisoned = true;
                return Err(invalid(
                    "typed eject issue key carried an unmount collector plan",
                ));
            }
            #[cfg(test)]
            SealedOwnedCollectorEffectPlanV3::Synthetic => {
                self.store.poisoned = true;
                return Err(invalid(
                    "test-only synthetic issue key entered production eject dispatch",
                ));
            }
        };
        let read = IssuePlanReadSealV3 { _private: () };
        let issue_plan = match plan.issue_plan(&read) {
            Ok(issue_plan) => issue_plan,
            Err(error) => {
                self.store.poisoned = true;
                return Err(invalid(format!(
                    "sealed eject collector plan failed dispatch projection: {error}"
                )));
            }
        };
        if let Err(error) = issue_plan.revalidate_for_issue(&read) {
            self.store.poisoned = true;
            return Err(invalid(format!(
                "sealed eject collector plan failed dispatch replay: {error}"
            )));
        }
        let material = self.consume_inner(identity, &issue_plan, &read)?;
        Ok(AdoptedIssuedEffectDispatchV3 {
            material,
            plan: SealedOwnedCollectorEffectPlanV3::Eject(plan),
            _not_send_or_sync: PhantomData,
        })
    }

    fn consume_inner<K>(
        self,
        identity: AdoptedIssuedEffectIdentityV3<K>,
        plan: &SealedCollectorEffectIssuePlanV3<'_, K>,
        read: &IssuePlanReadSealV3,
    ) -> Result<OwnedIssuedEffectDispatchMaterialV3<K>, DurableEffectIssueStoreErrorV3>
    where
        K: IssuedEffectKindSpecV3,
    {
        let replayed = (|| {
            self.store.revalidate_s1_adopted()?;
            let collector = plan.collector_binding_for_issue(read);
            collector.revalidate().map_err(|error| {
                invalid(format!(
                    "sealed collector binding failed owned-key replay: {error}"
                ))
            })?;
            let plan_command = plan.command_for_issue(read);
            let plan_command_bytes = canonical_json(plan_command)?;
            if identity.record.command != *plan_command
                || identity.record.command_canonical_json.as_bytes() != plan_command_bytes
                || identity.record.command_sha256 != sha256(&plan_command_bytes)
                || identity.record.operation_nonce != collector.operation_nonce()
                || identity.record.boot_session_uuid != collector.boot_session_uuid()
                || identity.record.prior_collector_lifecycle_record_sha256
                    != collector.lifecycle_record_sha256()
                || identity.record.prior_collector_lifecycle_sequence
                    != collector.lifecycle_record_sequence()
                || identity.record.prior_collector_receipt_sha256 != collector.receipt_sha256()
                || identity.record.unique_binding_sha256.as_deref()
                    != Some(collector.unique_binding_sha256())
            {
                return Err(invalid(
                    "owned adopted issue differs from its still-retained sealed collector plan",
                ));
            }
            let mut matches = self
                .store
                .issues
                .iter()
                .filter(|issue| issue.record.effect_id == identity.effect_id);
            let issue = matches.next().ok_or_else(|| {
                invalid("owned adopted issue is absent from the retained issue store")
            })?;
            if matches.next().is_some()
                || self.store.operation_nonce != identity.operation_nonce
                || issue.binding != identity.issue_binding
                || issue.name != identity.issue_name
                || issue.bytes != identity.record_canonical_bytes
                || issue.record_sha256 != identity.record_sha256
                || issue.record != identity.record
                || issue.record.effect_kind != K::KIND
                || issue.record.command.kind() != K::KIND
                || issue.record.unique_binding_sha256.is_none()
                || !issue.s1_adopted
            {
                return Err(invalid(
                    "owned adopted issue changed or was transplanted across whole-store replay",
                ));
            }
            Ok(OwnedIssuedEffectDispatchMaterialV3 {
                record: identity.record,
                record_canonical_bytes: identity.record_canonical_bytes,
                record_sha256: identity.record_sha256,
                _kind: PhantomData,
                _not_send_or_sync: PhantomData,
            })
        })();
        match replayed {
            Ok(material) => Ok(material),
            Err(error) => {
                self.store.poisoned = true;
                Err(error)
            }
        }
    }

    #[cfg(test)]
    fn consume_for_test<K>(
        self,
        key: AdoptedIssuedEffectKeyV3<K>,
    ) -> Result<OwnedIssuedEffectDispatchMaterialV3<K>, DurableEffectIssueStoreErrorV3>
    where
        K: IssuedEffectKindSpecV3,
    {
        let AdoptedIssuedEffectKeyV3 { identity, plan, .. } = key;
        if !matches!(plan, SealedOwnedCollectorEffectPlanV3::Synthetic) {
            self.store.poisoned = true;
            return Err(invalid(
                "production collector plan entered the test-only issue sink",
            ));
        }
        let replayed = (|| {
            self.store.revalidate_s1_adopted()?;
            let mut matches = self
                .store
                .issues
                .iter()
                .filter(|issue| issue.record.effect_id == identity.effect_id);
            let issue = matches.next().ok_or_else(|| {
                invalid("owned adopted issue is absent from the retained issue store")
            })?;
            if matches.next().is_some()
                || self.store.operation_nonce != identity.operation_nonce
                || issue.binding != identity.issue_binding
                || issue.name != identity.issue_name
                || issue.bytes != identity.record_canonical_bytes
                || issue.record_sha256 != identity.record_sha256
                || issue.record != identity.record
                || issue.record.effect_kind != K::KIND
                || issue.record.command.kind() != K::KIND
                || issue.record.unique_binding_sha256.is_none()
                || !issue.s1_adopted
            {
                return Err(invalid(
                    "owned adopted issue changed or was transplanted across whole-store replay",
                ));
            }
            Ok(OwnedIssuedEffectDispatchMaterialV3 {
                record: identity.record,
                record_canonical_bytes: identity.record_canonical_bytes,
                record_sha256: identity.record_sha256,
                _kind: PhantomData,
                _not_send_or_sync: PhantomData,
            })
        })();
        match replayed {
            Ok(material) => Ok(material),
            Err(error) => {
                self.store.poisoned = true;
                Err(error)
            }
        }
    }
}

impl AdoptedIssuedEffectDispatchV3<PersistedUnmountEffectV3> {
    pub(crate) fn into_parts(
        self,
    ) -> Result<
        (
            OwnedIssuedEffectDispatchMaterialV3<PersistedUnmountEffectV3>,
            SealedUnmountEffectPlanV3,
        ),
        DurableEffectIssueStoreErrorV3,
    > {
        match self.plan {
            SealedOwnedCollectorEffectPlanV3::Unmount(plan) => Ok((self.material, plan)),
            SealedOwnedCollectorEffectPlanV3::Eject(_) => Err(invalid(
                "typed unmount dispatch carried an eject collector plan",
            )),
            #[cfg(test)]
            SealedOwnedCollectorEffectPlanV3::Synthetic => Err(invalid(
                "test-only synthetic issue plan entered production unmount dispatch",
            )),
        }
    }
}

impl AdoptedIssuedEffectDispatchV3<PersistedEjectEffectV3> {
    pub(crate) fn into_parts(
        self,
    ) -> Result<
        (
            OwnedIssuedEffectDispatchMaterialV3<PersistedEjectEffectV3>,
            SealedEjectEffectPlanV3,
        ),
        DurableEffectIssueStoreErrorV3,
    > {
        match self.plan {
            SealedOwnedCollectorEffectPlanV3::Eject(plan) => Ok((self.material, plan)),
            SealedOwnedCollectorEffectPlanV3::Unmount(_) => Err(invalid(
                "typed eject dispatch carried an unmount collector plan",
            )),
            #[cfg(test)]
            SealedOwnedCollectorEffectPlanV3::Synthetic => Err(invalid(
                "test-only synthetic issue plan entered production eject dispatch",
            )),
        }
    }
}

impl<K> OwnedIssuedEffectDispatchMaterialV3<K> {
    /// The marker `K` remains on the argument at the runner call boundary, so
    /// unmount and eject dispatch cannot share a dynamically selected command
    /// path before the runner has checked all durable process/runner identities
    /// in the record.  No lifecycle- or crate-wide raw-parts outlet exists.
    pub(crate) fn into_runner_parts(
        self,
        _seal: RunnerIssueReadSealV3,
    ) -> (IssuedEffectRecordV3, Vec<u8>, String) {
        (self.record, self.record_canonical_bytes, self.record_sha256)
    }

    #[cfg(test)]
    fn test_record(&self) -> &IssuedEffectRecordV3 {
        &self.record
    }
}

fn replay_issue_directory(
    directory: &File,
    directory_binding: FilesystemBindingV3,
    expected_uid: u32,
    expected_gid: u32,
    lifecycle: &VerifiedLifecycleIssueRosterV3,
) -> Result<Vec<IssueFileCapsuleV3>, DurableEffectIssueStoreErrorV3> {
    let names = list_directory(directory.as_raw_fd(), MAX_ISSUES_V3)?;
    if names.len() != lifecycle.issues.len() {
        return Err(invalid(
            "V2 issued roster and V3 issue roster are not an exact bijection",
        ));
    }
    let mut by_effect = BTreeMap::new();
    let mut aggregate = 0usize;
    for name in names {
        let (effect_id, name_sha256) = parse_issue_name(&name)
            .ok_or_else(|| invalid("V3 issue directory contains a noncanonical entry"))?;
        if by_effect.contains_key(&effect_id) {
            return Err(invalid("V3 issue roster contains a duplicate effect ID"));
        }
        let file = openat_issue_file(directory.as_raw_fd(), &name)?;
        let initial = binding(&file)?;
        let size = usize::try_from(initial.size)
            .ok()
            .filter(|size| *size > 0 && *size <= MAX_ISSUE_BYTES_V3)
            .ok_or_else(|| invalid("V3 issue file size is outside its bound"))?;
        aggregate = aggregate
            .checked_add(size)
            .filter(|total| *total <= MAX_TOTAL_ISSUE_BYTES_V3)
            .ok_or_else(|| invalid("V3 issue aggregate exceeds its fixed byte bound"))?;
        let binding = validate_issue_file(
            &file,
            expected_uid,
            expected_gid,
            directory_binding.dev,
            size,
            "replayed V3 issue file",
        )?;
        if binding != initial || named_binding(directory.as_raw_fd(), &name)? != binding {
            return Err(invalid(
                "replayed V3 issue pathname differs from its descriptor",
            ));
        }
        let bytes = read_exact_file(&file, binding)?;
        let record_sha256 = sha256(&bytes);
        if record_sha256 != name_sha256 {
            return Err(invalid("V3 issue digest differs from its filename"));
        }
        let expected = lifecycle
            .issues
            .iter()
            .find(|issue| issue.effect_id == effect_id)
            .ok_or_else(|| invalid("V3 issue is orphaned from the V2 issued roster"))?;
        let record = decode_and_validate_record(&bytes, &name, expected)?;
        by_effect.insert(
            effect_id,
            IssueFileCapsuleV3 {
                binding,
                bytes,
                file,
                name,
                record,
                record_sha256,
                s1_adopted: false,
            },
        );
    }
    let mut issues = Vec::with_capacity(lifecycle.issues.len());
    for expected in &lifecycle.issues {
        issues.push(
            by_effect
                .remove(&expected.effect_id)
                .ok_or_else(|| invalid("V3 issue is missing for a V2 issued record"))?,
        );
    }
    if !by_effect.is_empty() {
        return Err(invalid("V3 issue roster contains orphan entries"));
    }
    Ok(issues)
}

fn replay_retained_issue_directory(
    directory: &File,
    directory_binding: FilesystemBindingV3,
    expected_uid: u32,
    expected_gid: u32,
    retained_issues: Vec<(String, Vec<u8>, File, (u64, u64))>,
    lifecycle: &VerifiedLifecycleIssueRosterV3,
) -> Result<Vec<IssueFileCapsuleV3>, DurableEffectIssueStoreErrorV3> {
    let names = list_directory(directory.as_raw_fd(), MAX_ISSUES_V3)?;
    let retained_names = retained_issues
        .iter()
        .map(|(name, _, _, _)| name.clone())
        .collect::<Vec<_>>();
    if names != retained_names || names.len() != lifecycle.issues.len() {
        return Err(invalid(
            "V3 issue roster differs from the exact retained S1 capsules or V2 issued roster",
        ));
    }
    let mut by_effect = BTreeMap::new();
    let mut aggregate = 0usize;
    for (name, retained_bytes, file, expected_inode) in retained_issues {
        let (effect_id, name_sha256) = parse_issue_name(&name)
            .ok_or_else(|| invalid("retained S1 V3 issue name is noncanonical"))?;
        if by_effect.contains_key(&effect_id) {
            return Err(invalid(
                "retained S1 V3 issue roster duplicates an effect ID",
            ));
        }
        let size = retained_bytes.len();
        if size == 0 || size > MAX_ISSUE_BYTES_V3 {
            return Err(invalid("retained S1 V3 issue size is outside its bound"));
        }
        aggregate = aggregate
            .checked_add(size)
            .filter(|total| *total <= MAX_TOTAL_ISSUE_BYTES_V3)
            .ok_or_else(|| invalid("retained S1 V3 issue aggregate exceeds its bound"))?;
        let issue_binding = validate_issue_file(
            &file,
            expected_uid,
            expected_gid,
            directory_binding.dev,
            size,
            "retained S1 V3 issue file",
        )?;
        if (issue_binding.dev, issue_binding.inode) != expected_inode
            || named_binding(directory.as_raw_fd(), &name)? != issue_binding
            || read_exact_file(&file, issue_binding)? != retained_bytes
        {
            return Err(invalid(
                "S2 V3 issue capsule differs from the exact retained S1 descriptor",
            ));
        }
        let record_sha256 = sha256(&retained_bytes);
        if record_sha256 != name_sha256 {
            return Err(invalid(
                "retained S1 V3 issue bytes differ from their filename digest",
            ));
        }
        let expected = lifecycle
            .issues
            .iter()
            .find(|issue| issue.effect_id == effect_id)
            .ok_or_else(|| invalid("retained S1 V3 issue is orphaned from V2"))?;
        let record = decode_and_validate_record(&retained_bytes, &name, expected)?;
        by_effect.insert(
            effect_id,
            IssueFileCapsuleV3 {
                binding: issue_binding,
                bytes: retained_bytes,
                file,
                name,
                record,
                record_sha256,
                s1_adopted: true,
            },
        );
    }
    let mut issues = Vec::with_capacity(lifecycle.issues.len());
    for expected in &lifecycle.issues {
        issues.push(
            by_effect
                .remove(&expected.effect_id)
                .ok_or_else(|| invalid("retained S1 V3 issue is missing for a V2 issue"))?,
        );
    }
    if !by_effect.is_empty() {
        return Err(invalid("retained S1 V3 issue roster contains orphans"));
    }
    Ok(issues)
}

fn decode_and_validate_record(
    bytes: &[u8],
    name: &str,
    expected: &LifecycleIssueBindingV3,
) -> Result<IssuedEffectRecordV3, DurableEffectIssueStoreErrorV3> {
    let record: IssuedEffectRecordV3 = serde_json::from_slice(bytes)
        .map_err(|error| invalid(format!("V3 issue JSON failed: {error}")))?;
    if canonical_json(&record)? != bytes {
        return Err(invalid("V3 issue JSON is not canonical"));
    }
    let digest = sha256(bytes);
    if issue_name(record.effect_id, &digest) != name {
        return Err(invalid(
            "V3 issue filename does not bind its effect and digest",
        ));
    }
    record.validate_against(expected)?;
    Ok(record)
}

fn aggregate_issue_bytes(
    issues: &[IssueFileCapsuleV3],
) -> Result<usize, DurableEffectIssueStoreErrorV3> {
    issues.iter().try_fold(0usize, |total, issue| {
        total
            .checked_add(issue.bytes.len())
            .filter(|total| *total <= MAX_TOTAL_ISSUE_BYTES_V3)
            .ok_or_else(|| invalid("V3 issue aggregate exceeds its fixed byte bound"))
    })
}

fn issue_name(effect_id: u64, digest: &str) -> String {
    format!("effect-{effect_id:020}-{digest}.json")
}

fn parse_issue_name(name: &str) -> Option<(u64, String)> {
    let value = name.strip_prefix("effect-")?.strip_suffix(".json")?;
    let (effect, digest) = value.split_once('-')?;
    if effect.len() != 20
        || !effect.as_bytes().iter().all(u8::is_ascii_digit)
        || !valid_digest(digest)
    {
        return None;
    }
    let effect_id = effect.parse::<u64>().ok().filter(|value| *value != 0)?;
    if format!("{effect_id:020}") != effect {
        return None;
    }
    Some((effect_id, digest.to_string()))
}

fn lifecycle_record_name(sequence: usize) -> String {
    format!("{sequence:08}.json")
}

fn parse_lifecycle_record_name(name: &str) -> Option<usize> {
    let sequence = name.strip_suffix(".json")?;
    if sequence.len() != 8 || !sequence.as_bytes().iter().all(u8::is_ascii_digit) {
        return None;
    }
    let sequence = sequence.parse::<usize>().ok().filter(|value| *value != 0)?;
    (lifecycle_record_name(sequence) == name).then_some(sequence)
}

fn lifecycle_record_roster(
    operation_directory_fd: RawFd,
) -> Result<Vec<String>, DurableEffectIssueStoreErrorV3> {
    let names = list_directory(operation_directory_fd, MAX_OPERATION_ENTRIES_V3)?;
    let mut records = Vec::new();
    for name in names {
        if name == ISSUE_DIRECTORY_NAME_V3
            || name == PREPARED_MANIFEST_NAME_V3
            || name == RESTART_ADMISSION_DIRECTORY_NAME_V3
        {
            continue;
        }
        if name.contains("effect-issues-v3")
            || name.contains("prepared-collector-manifest-v3")
            || name.contains("restart-admissions-v3")
            || parse_lifecycle_record_name(&name).is_none()
        {
            return Err(invalid(
                "operation contains a noncanonical lifecycle or V3 sidecar entry",
            ));
        }
        records.push(name);
    }
    records.sort();
    Ok(records)
}

fn reject_issue_directory_aliases(
    operation_directory_fd: RawFd,
    final_expected: bool,
) -> Result<(), DurableEffectIssueStoreErrorV3> {
    let names = list_directory(operation_directory_fd, MAX_OPERATION_ENTRIES_V3)?;
    let issue_like = names
        .iter()
        .filter(|name| name.contains("effect-issues-v3"))
        .collect::<Vec<_>>();
    if (!final_expected && !issue_like.is_empty())
        || (final_expected
            && (issue_like.len() != 1 || issue_like[0].as_str() != ISSUE_DIRECTORY_NAME_V3))
    {
        return Err(invalid(
            "operation contains a missing, duplicate, temporary, or aliased V3 issue directory",
        ));
    }
    Ok(())
}

fn createat_issue_file(
    directory_fd: RawFd,
    name: &str,
    expected_uid: u32,
    expected_gid: u32,
    expected_dev: u64,
) -> Result<File, DurableEffectIssueStoreErrorV3> {
    let name = cstring(name, "V3 issue filename")?;
    let fd = unsafe {
        libc::openat(
            directory_fd,
            name.as_ptr(),
            libc::O_CREAT | libc::O_EXCL | libc::O_RDWR | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            0o400,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error().into());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    if unsafe { libc::fchmod(file.as_raw_fd(), 0o400) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    validate_issue_file(
        &file,
        expected_uid,
        expected_gid,
        expected_dev,
        0,
        "new V3 issue file",
    )?;
    Ok(file)
}

fn openat_issue_file(
    directory_fd: RawFd,
    name: &str,
) -> Result<File, DurableEffectIssueStoreErrorV3> {
    let name = cstring(name, "V3 issue filename")?;
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

fn mkdirat_private(directory_fd: RawFd, name: &str) -> Result<(), DurableEffectIssueStoreErrorV3> {
    let name = cstring(name, "V3 issue directory name")?;
    if unsafe { libc::mkdirat(directory_fd, name.as_ptr(), 0o700) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}

fn openat_directory(
    directory_fd: RawFd,
    name: &str,
) -> Result<File, DurableEffectIssueStoreErrorV3> {
    let name = cstring(name, "V3 issue directory name")?;
    let fd = unsafe {
        libc::openat(
            directory_fd,
            name.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn rename_noreplace(
    from_directory_fd: RawFd,
    from: &str,
    to_directory_fd: RawFd,
    to: &str,
) -> Result<(), DurableEffectIssueStoreErrorV3> {
    let from = cstring(from, "temporary V3 issue name")?;
    let to = cstring(to, "final V3 issue name")?;
    if unsafe {
        renameatx_np(
            from_directory_fd,
            from.as_ptr(),
            to_directory_fd,
            to.as_ptr(),
            RENAME_EXCL,
        )
    } != 0
    {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}

fn require_absent(directory_fd: RawFd, name: &str) -> Result<(), DurableEffectIssueStoreErrorV3> {
    let name = cstring(name, "V3 issue pathname")?;
    let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
    if unsafe {
        libc::fstatat(
            directory_fd,
            name.as_ptr(),
            stat.as_mut_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    } == 0
    {
        return Err(invalid("unexpected V3 issue pathname already exists"));
    }
    let error = io::Error::last_os_error();
    if error.raw_os_error() != Some(libc::ENOENT) {
        return Err(error.into());
    }
    Ok(())
}

fn named_binding(
    directory_fd: RawFd,
    name: &str,
) -> Result<FilesystemBindingV3, DurableEffectIssueStoreErrorV3> {
    let name = cstring(name, "V3 issue pathname")?;
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

fn binding(file: &File) -> Result<FilesystemBindingV3, DurableEffectIssueStoreErrorV3> {
    let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
    if unsafe { libc::fstat(file.as_raw_fd(), stat.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    binding_from_stat(unsafe { stat.assume_init() })
}

fn binding_from_stat(
    stat: libc::stat,
) -> Result<FilesystemBindingV3, DurableEffectIssueStoreErrorV3> {
    if stat.st_size < 0
        || !(0..1_000_000_000).contains(&stat.st_birthtime_nsec)
        || !(0..1_000_000_000).contains(&stat.st_ctime_nsec)
        || !(0..1_000_000_000).contains(&stat.st_mtime_nsec)
    {
        return Err(invalid("filesystem binding contains invalid stat fields"));
    }
    let value = FilesystemBindingV3 {
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
    if value.dev == 0 || value.inode == 0 || value.nlink == 0 {
        return Err(invalid("filesystem binding contains a zero stable field"));
    }
    Ok(value)
}

fn same_object_across_rename(before: FilesystemBindingV3, after: FilesystemBindingV3) -> bool {
    before.birthtime_nanoseconds == after.birthtime_nanoseconds
        && before.birthtime_seconds == after.birthtime_seconds
        && before.dev == after.dev
        && before.flags == after.flags
        && before.generation == after.generation
        && before.gid == after.gid
        && before.inode == after.inode
        && before.mode == after.mode
        && before.mtime_nanoseconds == after.mtime_nanoseconds
        && before.mtime_seconds == after.mtime_seconds
        && before.nlink == after.nlink
        && before.size == after.size
        && before.uid == after.uid
        && (after.ctime_seconds, after.ctime_nanoseconds)
            >= (before.ctime_seconds, before.ctime_nanoseconds)
}

fn same_directory_object(before: FilesystemBindingV3, after: FilesystemBindingV3) -> bool {
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

fn validate_directory(
    file: &File,
    expected_uid: u32,
    expected_gid: u32,
    expected_dev: Option<u64>,
    label: &str,
) -> Result<FilesystemBindingV3, DurableEffectIssueStoreErrorV3> {
    let before = binding(file)?;
    if before.mode & libc::S_IFMT as u32 != libc::S_IFDIR as u32
        || before.mode & 0o7777 != 0o700
        || before.uid != expected_uid
        || before.gid != expected_gid
        || before.flags != 0
        || expected_dev.is_some_and(|dev| before.dev != dev)
    {
        return Err(invalid(format!(
            "{label} type, owner, mode, flags, or filesystem is invalid"
        )));
    }
    verify_no_extended_metadata(file.as_raw_fd(), label)?;
    if binding(file)? != before {
        return Err(invalid(format!("{label} changed during ACL/xattr replay")));
    }
    Ok(before)
}

fn validate_issue_file(
    file: &File,
    expected_uid: u32,
    expected_gid: u32,
    expected_dev: u64,
    expected_size: usize,
    label: &str,
) -> Result<FilesystemBindingV3, DurableEffectIssueStoreErrorV3> {
    let before = binding(file)?;
    if before.mode & libc::S_IFMT as u32 != libc::S_IFREG as u32
        || before.mode & 0o7777 != 0o400
        || before.uid != expected_uid
        || before.gid != expected_gid
        || before.dev != expected_dev
        || before.flags != 0
        || before.nlink != 1
        || usize::try_from(before.size).ok() != Some(expected_size)
    {
        return Err(invalid(format!(
            "{label} type, owner, mode, flags, links, filesystem, or size is invalid"
        )));
    }
    verify_no_extended_metadata(file.as_raw_fd(), label)?;
    if binding(file)? != before {
        return Err(invalid(format!("{label} changed during ACL/xattr replay")));
    }
    Ok(before)
}

fn verify_no_extended_metadata(
    fd: RawFd,
    label: &str,
) -> Result<(), DurableEffectIssueStoreErrorV3> {
    let xattr_bytes = unsafe { libc::flistxattr(fd, std::ptr::null_mut(), 0, 0) };
    if xattr_bytes < 0 {
        return Err(io::Error::last_os_error().into());
    }
    if xattr_bytes != 0 {
        return Err(invalid(format!(
            "{label} has extended attributes; V3 issue evidence requires none"
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
            "{label} has an extended ACL; V3 issue evidence requires none"
        ))),
        -1 if entry_error.raw_os_error() == Some(libc::EINVAL) => Ok(()),
        _ => Err(entry_error.into()),
    }
}

fn read_exact_file(
    file: &File,
    expected: FilesystemBindingV3,
) -> Result<Vec<u8>, DurableEffectIssueStoreErrorV3> {
    let size = usize::try_from(expected.size)
        .ok()
        .filter(|size| *size <= MAX_ISSUE_BYTES_V3)
        .ok_or_else(|| invalid("V3 issue file exceeds its read bound"))?;
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
            return Err(invalid("V3 issue file was truncated during replay"));
        }
        offset += count as usize;
    }
    if binding(file)? != expected {
        return Err(invalid("V3 issue file changed while it was read"));
    }
    Ok(bytes)
}

fn list_directory(
    fd: RawFd,
    maximum: usize,
) -> Result<Vec<String>, DurableEffectIssueStoreErrorV3> {
    let reopened = unsafe {
        libc::openat(
            fd,
            c".".as_ptr(),
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
            invalid("V3 issue directory contains a non-UTF-8 entry")
        })?;
        if name == "." || name == ".." {
            continue;
        }
        if names.len() == maximum {
            unsafe { libc::closedir(directory) };
            return Err(invalid("directory exceeds its fixed roster bound"));
        }
        names.push(name.to_string());
    }
    names.sort();
    Ok(names)
}

fn require_current_owner(
    expected_uid: u32,
    expected_gid: u32,
) -> Result<(), DurableEffectIssueStoreErrorV3> {
    if expected_uid != unsafe { libc::geteuid() } || expected_gid != unsafe { libc::getegid() } {
        return Err(invalid(
            "V3 issue store owner differs from the current effective identity",
        ));
    }
    Ok(())
}

fn require_nonce(value: &str, label: &str) -> Result<(), DurableEffectIssueStoreErrorV3> {
    if !valid_digest(value) {
        return Err(invalid(format!(
            "{label} is not 64 lowercase hexadecimal characters"
        )));
    }
    Ok(())
}

fn require_digest(value: &str, label: &str) -> Result<(), DurableEffectIssueStoreErrorV3> {
    if !valid_digest(value) {
        return Err(invalid(format!(
            "{label} is not a lowercase SHA-256 digest"
        )));
    }
    Ok(())
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

fn require_uuid(value: &str, label: &str) -> Result<(), DurableEffectIssueStoreErrorV3> {
    let bytes = value.as_bytes();
    if bytes.len() != 36
        || !bytes
            .iter()
            .any(|byte| byte.is_ascii_hexdigit() && *byte != b'0')
        || !bytes.iter().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => *byte == b'-',
            _ => byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase(),
        })
    {
        return Err(invalid(format!(
            "{label} is not a canonical lowercase non-nil UUID"
        )));
    }
    Ok(())
}

fn cstring(value: &str, label: &str) -> Result<CString, DurableEffectIssueStoreErrorV3> {
    CString::new(value).map_err(|_| invalid(format!("{label} contains NUL")))
}

fn invalid(message: impl Into<String>) -> DurableEffectIssueStoreErrorV3 {
    DurableEffectIssueStoreErrorV3::Invalid(message.into())
}

#[cfg(test)]
#[path = "mac_disposable_effect_issue_store_tests.rs"]
mod tests;
