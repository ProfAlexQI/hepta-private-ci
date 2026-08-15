//! Descriptor-retained V3 effect-issue persistence for inert macOS effects.
//!
//! This module owns no effect primitive and grants no authority.  It binds one
//! canonical, typed command to one already-durable V2 issued-or-uncertain
//! lifecycle record and to the most recent durable collector observation.
//! New-format operations must create `effect-issues-v3` before their operation
//! directory is published.  S2 will supply that construction ordering when it
//! integrates this module; a published new-format operation with no directory
//! is therefore blocking, while untouched historical V2 operations remain
//! outside this module and may have no such directory.

use crate::AcceptanceError;
use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::mac_disposable_lifecycle::DisposableAuthorityV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleEventV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleRecordV2;
use crate::mac_disposable_lifecycle::EffectPurposeV2;
use crate::mac_disposable_lifecycle::LifecycleErrorV2;
use crate::mac_disposable_lifecycle::inspect_lifecycle_v2;
use crate::mac_iomedia_identity::current_boot_session_uuid;
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EffectPurposeV3 {
    ForwardFlow,
    RestartReconciliation,
}

/// Closed, inert command vocabulary.  Every string is a digest rather than a
/// caller-selected path or executable name; this module cannot execute it.
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
                mountpoint_underlying_sha256,
                volume_identity_sha256,
                ..
            } => {
                require_digest(mountpoint_underlying_sha256, "mountpoint underlying digest")?;
                require_digest(volume_identity_sha256, "volume identity digest")?;
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
    schema: String,
    schema_version: u32,
    unique_binding_sha256: Option<String>,
}

/// Data-only epoch binding.  It is not an authority capability: integration
/// must derive it from the authenticated pre-runner before calling persist.
pub(crate) struct EffectEpochEvidenceV3 {
    boot_session_uuid: String,
    process_epoch_nonce: String,
    process_epoch_sha256: String,
    runner_epoch_nonce: String,
    runner_epoch_sha256: String,
}

impl EffectEpochEvidenceV3 {
    pub(crate) fn bind_current_boot(
        boot_session_uuid: &str,
        process_epoch_nonce: &str,
        process_epoch_sha256: &str,
        runner_epoch_nonce: &str,
        runner_epoch_sha256: &str,
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
        Ok(Self {
            boot_session_uuid: boot_session_uuid.to_string(),
            process_epoch_nonce: process_epoch_nonce.to_string(),
            process_epoch_sha256: process_epoch_sha256.to_string(),
            runner_epoch_nonce: runner_epoch_nonce.to_string(),
            runner_epoch_sha256: runner_epoch_sha256.to_string(),
        })
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

/// Exact canonical V2 replay product.  Fields are private so caller-provided
/// effect IDs, tips, or collector digests cannot stand in for lifecycle bytes.
pub(crate) struct VerifiedLifecycleIssueRosterV3 {
    issues: Vec<LifecycleIssueBindingV3>,
    operation_nonce: String,
    terminal_record_sha256: String,
}

impl VerifiedLifecycleIssueRosterV3 {
    pub(crate) fn replay(records: &[Vec<u8>]) -> Result<Self, DurableEffectIssueStoreErrorV3> {
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
            terminal_record_sha256: inspection.terminal_record_sha256,
        })
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    pub(crate) fn terminal_record_sha256(&self) -> &str {
        &self.terminal_record_sha256
    }
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

impl DurableEffectIssueStoreV3 {
    /// Create the mandatory empty issue directory for a new-format operation.
    /// S2 integration must call this before publishing the operation directory.
    pub(crate) fn create_new(
        operation_directory: &File,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<Self, DurableEffectIssueStoreErrorV3> {
        if !lifecycle.issues.is_empty() {
            return Err(invalid(
                "new V3 issue directory must exist before the first V2 issue",
            ));
        }
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
            operation_nonce: lifecycle.operation_nonce.clone(),
            poisoned: false,
            _not_send_or_sync: PhantomData,
        })
    }

    /// Reopen a mandatory new-format issue directory.  The entire V2 and V3
    /// rosters are replayed as a bijection.  Reopened issues remain evidence
    /// of issued-or-uncertain work; this API intentionally yields no fresh
    /// retained issue capability and therefore cannot authorize a resend.
    pub(crate) fn open_existing_required(
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
            unique_binding_sha256,
            |_| Ok(()),
        )
    }

    fn persist_with_hook<'store, F>(
        &'store mut self,
        lifecycle: &VerifiedLifecycleIssueRosterV3,
        command: ExactDisposableCommandV3,
        epochs: EffectEpochEvidenceV3,
        unique_binding_sha256: Option<String>,
        mut hook: F,
    ) -> Result<RetainedDurableEffectIssueV3<'store>, DurableEffectIssueStoreErrorV3>
    where
        F: FnMut(PublishCutpointV3) -> io::Result<()>,
    {
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
        if let Some(unique_binding_sha256) = &unique_binding_sha256 {
            require_digest(unique_binding_sha256, "unique collector binding digest")?;
        }
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
            schema: ISSUE_SCHEMA_V3.to_string(),
            schema_version: 3,
            unique_binding_sha256,
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
            hook(PublishCutpointV3::FinalReplayed)?;
            Ok((
                IssueFileCapsuleV3 {
                    binding: final_binding,
                    bytes,
                    file,
                    name: final_name,
                    record,
                    record_sha256,
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

    pub(crate) fn replayed_issue(&self, effect_id: u64) -> Option<&IssuedEffectRecordV3> {
        self.issues
            .iter()
            .find(|issue| issue.record.effect_id == effect_id)
            .map(|issue| &issue.record)
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

    pub(crate) fn runner_epoch_sha256(&self) -> &str {
        &self.runner_epoch_sha256
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
        if self.process_epoch_nonce == self.runner_epoch_nonce
            || self.process_epoch_sha256 == self.runner_epoch_sha256
        {
            return Err(invalid("process and runner epochs alias each other"));
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

    pub(crate) fn record_sha256(&self) -> &str {
        &self.store.issues[self.index].record_sha256
    }

    pub(crate) fn record(&self) -> &IssuedEffectRecordV3 {
        &self.store.issues[self.index].record
    }

    pub(crate) fn record_canonical_bytes(&self) -> &[u8] {
        &self.store.issues[self.index].bytes
    }

    pub(crate) fn revalidate(&self) -> Result<(), DurableEffectIssueStoreErrorV3> {
        self.store.revalidate_files()
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
