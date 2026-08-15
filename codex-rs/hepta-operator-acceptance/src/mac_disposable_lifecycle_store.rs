//! Durable, no-replace storage for the inert macOS disposable lifecycle.
//!
//! This module owns no effect capability.  It only creates an operation
//! directory and persists canonical lifecycle records before any caller may
//! consider sending an effect request.

use crate::durable::sha256;
use crate::mac_disposable_lifecycle::CallbackOutcomeV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleEventV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleJournalV2;
use crate::mac_disposable_lifecycle::EffectPurposeV2;
use crate::mac_disposable_lifecycle::FreshAbsenceObservationV2;
use crate::mac_disposable_lifecycle::LifecycleErrorV2;
use crate::mac_disposable_lifecycle::LifecycleProcessModeV2;
use crate::mac_disposable_lifecycle::ReconciliationSnapshotV2;
use crate::mac_disposable_lifecycle::TerminalDispositionV2;
#[cfg(test)]
use crate::mac_disposable_lifecycle::inspect_lifecycle_v2;
use crate::mac_privileged_disposable_control::PrivilegedDisposableControlErrorV2;
use crate::mac_privileged_disposable_control::RetainedControlCensusV3;
use crate::mac_privileged_disposable_control::RunnerControlLeaseSourceV3;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::Write;
use std::marker::PhantomData;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::RawFd;
use thiserror::Error;

const MAX_RECORD_BYTES: usize = 1024 * 1024;
const MAX_RECORDS: usize = 256;
const MAX_TOTAL_RECORD_BYTES: usize = 64 * 1024 * 1024;
const RENAME_EXCL: libc::c_uint = 0x0000_0004;

#[derive(Debug, Error)]
pub enum DurableLifecycleStoreErrorV3 {
    #[error(transparent)]
    Control(#[from] PrivilegedDisposableControlErrorV2),
    #[error("invalid durable lifecycle store: {0}")]
    Invalid(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Lifecycle(#[from] LifecycleErrorV2),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PublishCutpointV3 {
    TemporaryCreated,
    BytesWritten,
    FileSynced,
    Renamed,
    ParentSynced,
    FinalReopened,
    FinalRevalidated,
    CapsuleRetained,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CreateCutpointV3 {
    TemporaryCreated,
    TemporaryOpened,
    TemporarySynced,
    Renamed,
    ParentSynced,
    FinalReopened,
    FinalRevalidated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Binding {
    ctime_nsec: i64,
    ctime_sec: i64,
    dev: u64,
    flags: u32,
    gid: u32,
    ino: u64,
    mode: u32,
    mtime_nsec: i64,
    mtime_sec: i64,
    nlink: u64,
    size: u64,
    uid: u32,
}

impl Binding {
    fn stable_across_rename(self, other: Self) -> bool {
        self.dev == other.dev
            && self.flags == other.flags
            && self.gid == other.gid
            && self.ino == other.ino
            && self.mode == other.mode
            && self.mtime_nsec == other.mtime_nsec
            && self.mtime_sec == other.mtime_sec
            && self.nlink == other.nlink
            && self.size == other.size
            && self.uid == other.uid
    }
}

struct RecordCapsule {
    binding: Binding,
    bytes: Vec<u8>,
    file: File,
    name: String,
}

#[derive(Debug)]
pub(crate) struct FreshProcessStoreV3;

#[derive(Debug)]
pub(crate) struct ReconciliationOnlyStoreV3;

pub(crate) trait StoreModeV3 {
    const JOURNAL_MODE: LifecycleProcessModeV2;
}

impl StoreModeV3 for FreshProcessStoreV3 {
    const JOURNAL_MODE: LifecycleProcessModeV2 = LifecycleProcessModeV2::FreshProcess;
}

impl StoreModeV3 for ReconciliationOnlyStoreV3 {
    const JOURNAL_MODE: LifecycleProcessModeV2 = LifecycleProcessModeV2::RestartReconcileOnly;
}

/// Closed reconciliation event vocabulary.  There is deliberately no create,
/// attach, or mount constructor and the wrapped V2 event is private.
pub struct ReconciliationLifecycleEventV3(DisposableLifecycleEventV2);

#[allow(dead_code)]
impl ReconciliationLifecycleEventV3 {
    pub fn restart_started(
        boot_session_uuid: String,
        collector_policy_sha256: String,
        monotonic_nanoseconds: u64,
        restart_epoch_nonce: String,
    ) -> Self {
        Self(DisposableLifecycleEventV2::RestartReconciliationStarted {
            boot_session_uuid,
            collector_policy_sha256,
            monotonic_nanoseconds,
            restart_epoch_nonce,
        })
    }

    pub fn snapshot_observed(snapshot: ReconciliationSnapshotV2) -> Self {
        Self(DisposableLifecycleEventV2::ReconciliationSnapshotObserved { snapshot })
    }

    pub fn unmount_issued(effect_id: u64) -> Self {
        Self(DisposableLifecycleEventV2::UnmountIssuedOrUncertain {
            effect_id,
            purpose: EffectPurposeV2::Reconciliation,
        })
    }

    pub fn unmount_callback(effect_id: u64, outcome: CallbackOutcomeV2) -> Self {
        Self(DisposableLifecycleEventV2::UnmountCallbackObserved { effect_id, outcome })
    }

    pub fn unmount_observed(effect_id: u64, mount_absence_sha256: String) -> Self {
        Self(DisposableLifecycleEventV2::UnmountObserved {
            effect_id,
            mount_absence_sha256,
        })
    }

    pub fn eject_issued(effect_id: u64) -> Self {
        Self(DisposableLifecycleEventV2::EjectIssuedOrUncertain {
            effect_id,
            purpose: EffectPurposeV2::Reconciliation,
        })
    }

    pub fn eject_callback(effect_id: u64, outcome: CallbackOutcomeV2) -> Self {
        Self(DisposableLifecycleEventV2::EjectCallbackObserved { effect_id, outcome })
    }

    pub fn eject_observed(effect_id: u64, iomedia_absence_sha256: String) -> Self {
        Self(DisposableLifecycleEventV2::EjectObserved {
            effect_id,
            iomedia_absence_sha256,
        })
    }

    pub fn fresh_absence_observed(observation: FreshAbsenceObservationV2) -> Self {
        Self(DisposableLifecycleEventV2::FreshAbsenceObserved { observation })
    }

    pub fn manual_intervention(reason_sha256: String) -> Self {
        Self(DisposableLifecycleEventV2::ManualIntervention { reason_sha256 })
    }

    pub fn quarantined(reason_sha256: String) -> Self {
        Self(DisposableLifecycleEventV2::Quarantined { reason_sha256 })
    }

    pub fn terminal_absence_proved(
        disposition: TerminalDispositionV2,
        fresh_absence_sha256: String,
    ) -> Self {
        Self(DisposableLifecycleEventV2::TerminalAbsenceProved {
            disposition,
            fresh_absence_sha256,
        })
    }
}

/// A descriptor-retained operation directory.  Errors during persistence
/// poison this object; restart must reopen and replay the directory.
pub struct DurableLifecycleStoreV3<M = FreshProcessStoreV3> {
    directory: File,
    directory_binding: Binding,
    expected_gid: u32,
    expected_uid: u32,
    final_name: String,
    operation_nonce: String,
    parent: File,
    parent_binding: Binding,
    poisoned: bool,
    records: Vec<RecordCapsule>,
    temporary_name: String,
    _mode: PhantomData<fn() -> M>,
}

/// Fresh S2 store whose parent directory was admitted by one consumed S1
/// retained census.  The census and global lock outlive every append.
pub(crate) struct CensusBoundDurableLifecycleStoreV3<'a> {
    census: RetainedControlCensusV3<'a>,
    poisoned: bool,
    store: DurableLifecycleStoreV3<FreshProcessStoreV3>,
}

pub type ReconciliationDurableLifecycleStoreV3 = DurableLifecycleStoreV3<ReconciliationOnlyStoreV3>;

impl DurableLifecycleStoreV3<FreshProcessStoreV3> {
    #[cfg(test)]
    pub fn create(
        operations: &File,
        operation_nonce: &str,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<Self, DurableLifecycleStoreErrorV3> {
        Self::create_with_hook(
            operations,
            operation_nonce,
            expected_uid,
            expected_gid,
            |_| Ok(()),
        )
    }

    fn create_with_hook<F>(
        operations: &File,
        operation_nonce: &str,
        expected_uid: u32,
        expected_gid: u32,
        mut hook: F,
    ) -> Result<Self, DurableLifecycleStoreErrorV3>
    where
        F: FnMut(CreateCutpointV3) -> io::Result<()>,
    {
        require_nonce(operation_nonce)?;
        let parent_binding = validate_directory(
            operations,
            expected_uid,
            expected_gid,
            0o700,
            None,
            "operations directory",
        )?;
        let final_name = format!("operation-{operation_nonce}");
        let temporary_name = format!(".incoming-operation-{operation_nonce}");
        require_absent(operations.as_raw_fd(), &temporary_name)?;
        require_absent(operations.as_raw_fd(), &final_name)?;
        let temporary_component = component(&temporary_name)?;
        if unsafe { libc::mkdirat(operations.as_raw_fd(), temporary_component.as_ptr(), 0o700) }
            != 0
        {
            return Err(io::Error::last_os_error().into());
        }
        hook(CreateCutpointV3::TemporaryCreated)?;
        let temporary = openat_directory(operations.as_raw_fd(), &temporary_name)?;
        let temporary_binding = validate_directory(
            &temporary,
            expected_uid,
            expected_gid,
            0o700,
            Some(parent_binding.dev),
            "temporary operation directory",
        )?;
        if !read_directory_names(temporary.as_raw_fd(), MAX_RECORDS)?.is_empty() {
            return Err(invalid("temporary operation directory is not empty"));
        }
        hook(CreateCutpointV3::TemporaryOpened)?;
        temporary.sync_all()?;
        hook(CreateCutpointV3::TemporarySynced)?;
        rename_noreplace(
            operations.as_raw_fd(),
            &temporary_name,
            operations.as_raw_fd(),
            &final_name,
        )?;
        hook(CreateCutpointV3::Renamed)?;
        operations.sync_all()?;
        hook(CreateCutpointV3::ParentSynced)?;
        let directory = openat_directory(operations.as_raw_fd(), &final_name)?;
        hook(CreateCutpointV3::FinalReopened)?;
        let directory_binding = validate_directory(
            &directory,
            expected_uid,
            expected_gid,
            0o700,
            Some(parent_binding.dev),
            "final operation directory",
        )?;
        if !temporary_binding.stable_across_rename(directory_binding)
            || !read_directory_names(directory.as_raw_fd(), MAX_RECORDS)?.is_empty()
        {
            return Err(invalid(
                "final operation directory is not the exact empty temporary inode",
            ));
        }
        operations.sync_all()?;
        let store = Self {
            directory,
            directory_binding,
            expected_gid,
            expected_uid,
            final_name,
            operation_nonce: operation_nonce.to_string(),
            parent: operations.try_clone()?,
            parent_binding: binding(operations)?,
            poisoned: false,
            records: Vec::new(),
            temporary_name,
            _mode: PhantomData,
        };
        store.revalidate()?;
        hook(CreateCutpointV3::FinalRevalidated)?;
        store.revalidate()?;
        Ok(store)
    }

    #[cfg(test)]
    pub fn open_existing(
        operations: &File,
        operation_nonce: &str,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<ReconciliationDurableLifecycleStoreV3, DurableLifecycleStoreErrorV3> {
        require_nonce(operation_nonce)?;
        let parent_binding = validate_directory(
            operations,
            expected_uid,
            expected_gid,
            0o700,
            None,
            "operations directory",
        )?;
        let final_name = format!("operation-{operation_nonce}");
        let temporary_name = format!(".incoming-operation-{operation_nonce}");
        match require_absent(operations.as_raw_fd(), &temporary_name) {
            Ok(()) => {}
            Err(DurableLifecycleStoreErrorV3::Invalid(_)) => {
                return Err(invalid(
                    "operation has a crash-temporary directory; reconciliation is required",
                ));
            }
            Err(error) => return Err(error),
        }
        let directory = openat_directory(operations.as_raw_fd(), &final_name)?;
        let directory_binding = validate_directory(
            &directory,
            expected_uid,
            expected_gid,
            0o700,
            Some(parent_binding.dev),
            "operation directory",
        )?;
        let names = read_directory_names(directory.as_raw_fd(), MAX_RECORDS)?;
        if names.is_empty() {
            return Err(invalid("operation directory has no lifecycle records"));
        }
        let mut records = Vec::with_capacity(names.len());
        let mut total_bytes = 0usize;
        for (index, name) in names.iter().enumerate() {
            if name != &record_name(index + 1)? {
                return Err(invalid(
                    "operation roster contains a temporary, unknown, or gap entry",
                ));
            }
            let record = open_record(
                directory.as_raw_fd(),
                name,
                expected_uid,
                expected_gid,
                directory_binding.dev,
            )?;
            total_bytes = total_bytes
                .checked_add(record.bytes.len())
                .ok_or_else(|| invalid("lifecycle byte count overflowed"))?;
            if total_bytes > MAX_TOTAL_RECORD_BYTES {
                return Err(invalid("lifecycle bytes exceed the fixed total bound"));
            }
            records.push(record);
        }
        let bytes = records
            .iter()
            .map(|record| record.bytes.clone())
            .collect::<Vec<_>>();
        let inspection = inspect_lifecycle_v2(&bytes)?;
        if inspection.operation_nonce != operation_nonce {
            return Err(invalid(
                "operation directory nonce differs from lifecycle nonce",
            ));
        }
        let store = DurableLifecycleStoreV3::<ReconciliationOnlyStoreV3> {
            directory,
            directory_binding,
            expected_gid,
            expected_uid,
            final_name,
            operation_nonce: operation_nonce.to_string(),
            parent: operations.try_clone()?,
            parent_binding: binding(operations)?,
            poisoned: false,
            records,
            temporary_name,
            _mode: PhantomData,
        };
        store.revalidate()?;
        Ok(store)
    }

    pub fn append(
        &mut self,
        journal: &mut DisposableLifecycleJournalV2,
        event: DisposableLifecycleEventV2,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        self.append_mode_with_hook(journal, event, |_| Ok(()))
    }
}

impl<'a> CensusBoundDurableLifecycleStoreV3<'a> {
    pub(crate) fn create(
        mut census: RetainedControlCensusV3<'a>,
        operation_nonce: &str,
    ) -> Result<Self, DurableLifecycleStoreErrorV3> {
        let binding = census.prepare_store_creation()?;
        let store = DurableLifecycleStoreV3::create_with_hook(
            binding.operations(),
            operation_nonce,
            binding.expected_uid(),
            binding.expected_gid(),
            |_| Ok(()),
        )?;
        census.admit_fresh_operation(&store.final_name)?;
        Ok(Self {
            census,
            poisoned: false,
            store,
        })
    }

    pub(crate) fn append(
        &mut self,
        journal: &mut DisposableLifecycleJournalV2,
        event: DisposableLifecycleEventV2,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        if self.poisoned {
            return Err(invalid(
                "census-bound store is poisoned; restart reconciliation is required",
            ));
        }
        if let Err(error) = self.census.revalidate() {
            self.poisoned = true;
            return Err(error.into());
        }
        let digest = self.store.append(journal, event)?;
        if let Err(error) = self.census.revalidate() {
            self.poisoned = true;
            return Err(error.into());
        }
        Ok(digest)
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        self.store.operation_nonce()
    }

    pub(crate) fn poisoned(&self) -> bool {
        self.poisoned || self.store.poisoned()
    }

    pub(crate) fn runner_lease_source(
        &self,
    ) -> Result<RunnerControlLeaseSourceV3, DurableLifecycleStoreErrorV3> {
        Ok(self.census.runner_lease_source()?)
    }
}

impl DurableLifecycleStoreV3<ReconciliationOnlyStoreV3> {
    pub fn resume_for_reconciliation(
        &self,
    ) -> Result<DisposableLifecycleJournalV2, DurableLifecycleStoreErrorV3> {
        self.revalidate()?;
        let bytes = self
            .records
            .iter()
            .map(|record| record.bytes.clone())
            .collect::<Vec<_>>();
        Ok(DisposableLifecycleJournalV2::resume_for_reconciliation(
            &bytes,
        )?)
    }

    pub fn append_reconciliation(
        &mut self,
        journal: &mut DisposableLifecycleJournalV2,
        event: ReconciliationLifecycleEventV3,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        self.append_mode_with_hook(journal, event.0, |_| Ok(()))
    }
}

impl<M: StoreModeV3> DurableLifecycleStoreV3<M> {
    pub fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    pub fn poisoned(&self) -> bool {
        self.poisoned
    }

    fn append_mode_with_hook<F>(
        &mut self,
        journal: &mut DisposableLifecycleJournalV2,
        event: DisposableLifecycleEventV2,
        mut hook: F,
    ) -> Result<String, DurableLifecycleStoreErrorV3>
    where
        F: FnMut(PublishCutpointV3) -> io::Result<()>,
    {
        if self.poisoned {
            return Err(invalid(
                "store persistence is issued-or-uncertain; descriptor replay is required",
            ));
        }
        if journal.process_mode() != M::JOURNAL_MODE {
            return Err(invalid(
                "lifecycle journal process mode differs from the durable store typestate",
            ));
        }
        let expected_previous = self.records.last().map(|record| sha256(&record.bytes));
        if journal.operation_nonce() != self.operation_nonce
            || journal.record_count() != self.records.len()
            || journal.terminal_record_sha256() != expected_previous.as_deref()
        {
            return Err(invalid(
                "in-memory lifecycle journal differs from the descriptor-retained durable chain",
            ));
        }
        if let Err(error) = self.revalidate() {
            self.poisoned = true;
            return Err(error);
        }
        let expected_sequence = self
            .records
            .len()
            .checked_add(1)
            .ok_or_else(|| invalid("record count overflowed"))?;
        if expected_sequence > MAX_RECORDS {
            return Err(invalid("record count exceeds the fixed bound"));
        }
        let expected_uid = self.expected_uid;
        let expected_gid = self.expected_gid;
        let expected_dev = self.directory_binding.dev;
        let prior_total_bytes = self
            .records
            .iter()
            .try_fold(0usize, |total, record| {
                total.checked_add(record.bytes.len())
            })
            .ok_or_else(|| invalid("lifecycle byte count overflowed"))?;
        let operation_nonce = self.operation_nonce.clone();
        let result = journal.append_with(event, |record, bytes| {
            if usize::try_from(record.sequence).ok() != Some(expected_sequence)
                || record.operation_nonce != operation_nonce
                || record.previous_record_sha256.as_deref() != expected_previous.as_deref()
            {
                return Err(io::Error::other(
                    "journal identity, predecessor, or sequence differs from durable store",
                ));
            }
            if prior_total_bytes
                .checked_add(bytes.len())
                .is_none_or(|total| total > MAX_TOTAL_RECORD_BYTES)
            {
                return Err(io::Error::other(
                    "lifecycle bytes exceed the fixed total bound",
                ));
            }
            let capsule = publish_record(
                &self.directory,
                record.sequence,
                bytes,
                expected_uid,
                expected_gid,
                expected_dev,
                &mut hook,
            )
            .map_err(|error| io::Error::other(error.to_string()))?;
            self.records.push(capsule);
            hook(PublishCutpointV3::CapsuleRetained)?;
            self.directory_binding =
                binding(&self.directory).map_err(|error| io::Error::other(error.to_string()))?;
            self.revalidate()
                .map_err(|error| io::Error::other(error.to_string()))?;
            Ok(())
        });
        match result {
            Ok(digest) => Ok(digest),
            Err(error) => {
                if matches!(error, LifecycleErrorV2::Persistence(_)) {
                    self.poisoned = true;
                }
                Err(error.into())
            }
        }
    }

    fn revalidate(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        self.revalidate_with_hook(|| Ok(()))
    }

    fn revalidate_with_hook<F>(&self, hook: F) -> Result<(), DurableLifecycleStoreErrorV3>
    where
        F: FnOnce() -> io::Result<()>,
    {
        let parent_before = binding(&self.parent)?;
        let named_directory_before = named_binding(self.parent.as_raw_fd(), &self.final_name)?;
        let directory_before = binding(&self.directory)?;
        require_absent(self.parent.as_raw_fd(), &self.temporary_name)?;
        if parent_before != self.parent_binding
            || named_directory_before != directory_before
            || directory_before != self.directory_binding
        {
            return Err(invalid("retained operation descriptor identity changed"));
        }
        validate_directory(
            &self.parent,
            self.expected_uid,
            self.expected_gid,
            0o700,
            None,
            "operations directory replay",
        )?;
        validate_directory(
            &self.directory,
            self.expected_uid,
            self.expected_gid,
            0o700,
            Some(self.parent_binding.dev),
            "operation directory replay",
        )?;
        self.revalidate_roster_and_records()?;
        hook()?;
        require_absent(self.parent.as_raw_fd(), &self.temporary_name)?;
        self.revalidate_roster_and_records()?;
        let parent_after = binding(&self.parent)?;
        let named_directory_after = named_binding(self.parent.as_raw_fd(), &self.final_name)?;
        let directory_after = binding(&self.directory)?;
        if parent_after != parent_before
            || named_directory_after != named_directory_before
            || directory_after != directory_before
            || parent_after != self.parent_binding
            || named_directory_after != directory_after
            || directory_after != self.directory_binding
        {
            return Err(invalid(
                "retained operation descriptor changed during replay validation",
            ));
        }
        Ok(())
    }

    fn revalidate_roster_and_records(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        let names = read_directory_names(self.directory.as_raw_fd(), MAX_RECORDS)?;
        let expected = (1..=self.records.len())
            .map(record_name)
            .collect::<Result<Vec<_>, _>>()?;
        if names != expected {
            return Err(invalid(
                "operation roster changed or contains a crash-temporary entry",
            ));
        }
        for record in &self.records {
            if named_binding(self.directory.as_raw_fd(), &record.name)? != record.binding
                || binding(&record.file)? != record.binding
                || read_stable(&record.file, record.binding)? != record.bytes
            {
                return Err(invalid("durable lifecycle record changed during replay"));
            }
            validate_regular(
                &record.file,
                self.expected_uid,
                self.expected_gid,
                0o400,
                Some(self.directory_binding.dev),
                Some(record.bytes.len()),
                "lifecycle record replay",
            )?;
        }
        Ok(())
    }
}

fn publish_record<F>(
    directory: &File,
    sequence: u32,
    bytes: &[u8],
    expected_uid: u32,
    expected_gid: u32,
    expected_dev: u64,
    hook: &mut F,
) -> Result<RecordCapsule, DurableLifecycleStoreErrorV3>
where
    F: FnMut(PublishCutpointV3) -> io::Result<()>,
{
    if bytes.is_empty() || bytes.len() > MAX_RECORD_BYTES {
        return Err(invalid("record byte length is outside the fixed bound"));
    }
    let final_name = record_name(sequence as usize)?;
    let temporary_name = format!(".incoming-{final_name}");
    require_absent(directory.as_raw_fd(), &temporary_name)?;
    require_absent(directory.as_raw_fd(), &final_name)?;
    let mut temporary = createat_file(directory.as_raw_fd(), &temporary_name, 0o400)?;
    hook(PublishCutpointV3::TemporaryCreated)?;
    temporary.write_all(bytes)?;
    hook(PublishCutpointV3::BytesWritten)?;
    temporary.sync_all()?;
    hook(PublishCutpointV3::FileSynced)?;
    let temporary_binding = validate_regular(
        &temporary,
        expected_uid,
        expected_gid,
        0o400,
        Some(expected_dev),
        Some(bytes.len()),
        "temporary lifecycle record",
    )?;
    if read_stable(&temporary, temporary_binding)? != bytes {
        return Err(invalid("temporary lifecycle bytes differ after fsync"));
    }
    rename_noreplace(
        directory.as_raw_fd(),
        &temporary_name,
        directory.as_raw_fd(),
        &final_name,
    )?;
    hook(PublishCutpointV3::Renamed)?;
    directory.sync_all()?;
    hook(PublishCutpointV3::ParentSynced)?;
    let final_file = openat_regular(directory.as_raw_fd(), &final_name)?;
    hook(PublishCutpointV3::FinalReopened)?;
    let final_binding = validate_regular(
        &final_file,
        expected_uid,
        expected_gid,
        0o400,
        Some(expected_dev),
        Some(bytes.len()),
        "final lifecycle record",
    )?;
    if !temporary_binding.stable_across_rename(final_binding)
        || read_stable(&final_file, final_binding)? != bytes
    {
        return Err(invalid(
            "final lifecycle record is not the exact temporary inode and bytes",
        ));
    }
    directory.sync_all()?;
    hook(PublishCutpointV3::FinalRevalidated)?;
    Ok(RecordCapsule {
        binding: final_binding,
        bytes: bytes.to_vec(),
        file: final_file,
        name: final_name,
    })
}

fn open_record(
    directory_fd: RawFd,
    name: &str,
    expected_uid: u32,
    expected_gid: u32,
    expected_dev: u64,
) -> Result<RecordCapsule, DurableLifecycleStoreErrorV3> {
    let file = openat_regular(directory_fd, name)?;
    let record_binding = validate_regular(
        &file,
        expected_uid,
        expected_gid,
        0o400,
        Some(expected_dev),
        None,
        "existing lifecycle record",
    )?;
    let bytes = read_stable(&file, record_binding)?;
    Ok(RecordCapsule {
        binding: record_binding,
        bytes,
        file,
        name: name.to_string(),
    })
}

fn validate_directory(
    file: &File,
    expected_uid: u32,
    expected_gid: u32,
    expected_mode: u32,
    expected_dev: Option<u64>,
    label: &str,
) -> Result<Binding, DurableLifecycleStoreErrorV3> {
    let observed = binding(file)?;
    if observed.mode & u32::from(libc::S_IFMT) != u32::from(libc::S_IFDIR)
        || observed.mode & 0o7777 != expected_mode
        || observed.uid != expected_uid
        || observed.gid != expected_gid
        || observed.flags != 0
        || expected_dev.is_some_and(|dev| observed.dev != dev)
    {
        return Err(invalid(format!("{label} metadata is not exact")));
    }
    verify_acl_absent(file.as_raw_fd())?;
    verify_xattrs_empty(file.as_raw_fd())?;
    if binding(file)? != observed {
        return Err(invalid(format!(
            "{label} metadata changed during validation"
        )));
    }
    Ok(observed)
}

fn validate_regular(
    file: &File,
    expected_uid: u32,
    expected_gid: u32,
    expected_mode: u32,
    expected_dev: Option<u64>,
    expected_size: Option<usize>,
    label: &str,
) -> Result<Binding, DurableLifecycleStoreErrorV3> {
    let observed = binding(file)?;
    if observed.mode & u32::from(libc::S_IFMT) != u32::from(libc::S_IFREG)
        || observed.mode & 0o7777 != expected_mode
        || observed.uid != expected_uid
        || observed.gid != expected_gid
        || observed.nlink != 1
        || observed.flags != 0
        || observed.size as usize > MAX_RECORD_BYTES
        || expected_dev.is_some_and(|dev| observed.dev != dev)
        || expected_size.is_some_and(|size| observed.size as usize != size)
    {
        return Err(invalid(format!("{label} metadata is not exact")));
    }
    verify_acl_absent(file.as_raw_fd())?;
    verify_xattrs_empty(file.as_raw_fd())?;
    if binding(file)? != observed {
        return Err(invalid(format!(
            "{label} metadata changed during validation"
        )));
    }
    Ok(observed)
}

fn binding(file: &File) -> Result<Binding, DurableLifecycleStoreErrorV3> {
    let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    if unsafe { libc::fstat(file.as_raw_fd(), stat.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(binding_from_stat(unsafe { stat.assume_init() }))
}

fn named_binding(parent_fd: RawFd, name: &str) -> Result<Binding, DurableLifecycleStoreErrorV3> {
    let name = component(name)?;
    let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    if unsafe {
        libc::fstatat(
            parent_fd,
            name.as_ptr(),
            stat.as_mut_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    } != 0
    {
        return Err(io::Error::last_os_error().into());
    }
    Ok(binding_from_stat(unsafe { stat.assume_init() }))
}

fn binding_from_stat(stat: libc::stat) -> Binding {
    Binding {
        ctime_nsec: stat.st_ctime_nsec,
        ctime_sec: stat.st_ctime,
        dev: stat.st_dev as u64,
        flags: stat.st_flags,
        gid: stat.st_gid,
        ino: stat.st_ino,
        mode: u32::from(stat.st_mode),
        mtime_nsec: stat.st_mtime_nsec,
        mtime_sec: stat.st_mtime,
        nlink: stat.st_nlink as u64,
        size: stat.st_size.max(0) as u64,
        uid: stat.st_uid,
    }
}

fn read_stable(file: &File, expected: Binding) -> Result<Vec<u8>, DurableLifecycleStoreErrorV3> {
    let length = usize::try_from(expected.size).map_err(|_| invalid("record size overflowed"))?;
    if length > MAX_RECORD_BYTES {
        return Err(invalid("record exceeds the fixed byte bound"));
    }
    let mut bytes = vec![0_u8; length];
    let mut offset = 0usize;
    while offset < length {
        let read = unsafe {
            libc::pread(
                file.as_raw_fd(),
                bytes[offset..].as_mut_ptr().cast(),
                length - offset,
                offset as libc::off_t,
            )
        };
        if read < 0 {
            return Err(io::Error::last_os_error().into());
        }
        if read == 0 {
            return Err(invalid("record was truncated during descriptor read"));
        }
        offset += read as usize;
    }
    if binding(file)? != expected {
        return Err(invalid("record metadata changed during descriptor read"));
    }
    verify_acl_absent(file.as_raw_fd())?;
    verify_xattrs_empty(file.as_raw_fd())?;
    Ok(bytes)
}

fn require_absent(parent_fd: RawFd, name: &str) -> Result<(), DurableLifecycleStoreErrorV3> {
    let name = component(name)?;
    let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    if unsafe {
        libc::fstatat(
            parent_fd,
            name.as_ptr(),
            stat.as_mut_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    } == 0
    {
        return Err(invalid("durable no-replace name is already occupied"));
    }
    let error = io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::ENOENT) {
        Ok(())
    } else {
        Err(error.into())
    }
}

fn openat_directory(parent_fd: RawFd, name: &str) -> Result<File, DurableLifecycleStoreErrorV3> {
    let name = component(name)?;
    let fd = unsafe {
        libc::openat(
            parent_fd,
            name.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    file_from_fd(fd)
}

fn openat_regular(parent_fd: RawFd, name: &str) -> Result<File, DurableLifecycleStoreErrorV3> {
    let name = component(name)?;
    let fd = unsafe {
        libc::openat(
            parent_fd,
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    file_from_fd(fd)
}

fn createat_file(
    parent_fd: RawFd,
    name: &str,
    mode: u32,
) -> Result<File, DurableLifecycleStoreErrorV3> {
    let name = component(name)?;
    let fd = unsafe {
        libc::openat(
            parent_fd,
            name.as_ptr(),
            libc::O_RDWR | libc::O_CREAT | libc::O_EXCL | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            mode as libc::c_uint,
        )
    };
    file_from_fd(fd)
}

fn file_from_fd(fd: RawFd) -> Result<File, DurableLifecycleStoreErrorV3> {
    if fd < 0 {
        Err(io::Error::last_os_error().into())
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

fn rename_noreplace(
    source_parent_fd: RawFd,
    source_name: &str,
    destination_parent_fd: RawFd,
    destination_name: &str,
) -> Result<(), DurableLifecycleStoreErrorV3> {
    let source_name = component(source_name)?;
    let destination_name = component(destination_name)?;
    if unsafe {
        renameatx_np(
            source_parent_fd,
            source_name.as_ptr(),
            destination_parent_fd,
            destination_name.as_ptr(),
            RENAME_EXCL,
        )
    } != 0
    {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}

fn read_directory_names(
    fd: RawFd,
    maximum: usize,
) -> Result<Vec<String>, DurableLifecycleStoreErrorV3> {
    // `dup(2)` would share the retained directory's seek offset.  Reopen `.`
    // relative to the retained descriptor so every bounded census starts from
    // an independent open-file description.
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
            let error = unsafe { *libc::__error() };
            let close_result = unsafe { libc::closedir(directory) };
            if error != 0 {
                return Err(io::Error::from_raw_os_error(error).into());
            }
            if close_result != 0 {
                return Err(io::Error::last_os_error().into());
            }
            break;
        }
        let name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }
            .to_str()
            .map_err(|_| invalid("directory entry is not UTF-8"))?;
        if name == "." || name == ".." {
            continue;
        }
        if names.len() == maximum {
            unsafe { libc::closedir(directory) };
            return Err(invalid("directory entry bound exceeded"));
        }
        names.push(name.to_string());
    }
    names.sort();
    Ok(names)
}

fn verify_acl_absent(fd: RawFd) -> Result<(), DurableLifecycleStoreErrorV3> {
    const ACL_FIRST_ENTRY: libc::c_int = 0;
    const ACL_TYPE_EXTENDED: libc::c_int = 0x0000_0100;
    let acl = unsafe { acl_get_fd_np(fd, ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ENOENT) {
            return Ok(());
        }
        return Err(error.into());
    }
    let mut entry = std::ptr::null_mut();
    let result = unsafe { acl_get_entry(acl, ACL_FIRST_ENTRY, &mut entry) };
    let error = io::Error::last_os_error();
    if unsafe { acl_free(acl) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    match result {
        0 => Err(invalid("durable lifecycle node has an extended ACL")),
        -1 if error.raw_os_error() == Some(libc::EINVAL) => Ok(()),
        _ => Err(error.into()),
    }
}

fn verify_xattrs_empty(fd: RawFd) -> Result<(), DurableLifecycleStoreErrorV3> {
    let count = unsafe { libc::flistxattr(fd, std::ptr::null_mut(), 0, 0) };
    if count < 0 {
        return Err(io::Error::last_os_error().into());
    }
    if count != 0 {
        return Err(invalid("durable lifecycle node has extended attributes"));
    }
    Ok(())
}

fn component(name: &str) -> Result<CString, DurableLifecycleStoreErrorV3> {
    if name.is_empty() || name == "." || name == ".." || name.as_bytes().contains(&b'/') {
        return Err(invalid("node name is not one safe path component"));
    }
    CString::new(name).map_err(|_| invalid("node name contains NUL"))
}

fn record_name(sequence: usize) -> Result<String, DurableLifecycleStoreErrorV3> {
    if sequence == 0 || sequence > MAX_RECORDS {
        return Err(invalid("record sequence is outside the fixed bound"));
    }
    Ok(format!("{sequence:08}.json"))
}

fn require_nonce(value: &str) -> Result<(), DurableLifecycleStoreErrorV3> {
    if value.len() != 64
        || !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
        || value.as_bytes().iter().all(|byte| *byte == b'0')
    {
        return Err(invalid("operation nonce is not non-nil lowercase 64-hex"));
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> DurableLifecycleStoreErrorV3 {
    DurableLifecycleStoreErrorV3::Invalid(message.into())
}

unsafe extern "C" {
    fn acl_free(object: *mut libc::c_void) -> libc::c_int;
    fn acl_get_entry(
        acl: *mut libc::c_void,
        entry_id: libc::c_int,
        entry: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn acl_get_fd_np(fd: libc::c_int, acl_type: libc::c_int) -> *mut libc::c_void;
    fn renameatx_np(
        from_fd: libc::c_int,
        from: *const libc::c_char,
        to_fd: libc::c_int,
        to: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
}

#[cfg(test)]
#[path = "mac_disposable_lifecycle_store_tests.rs"]
mod tests;
