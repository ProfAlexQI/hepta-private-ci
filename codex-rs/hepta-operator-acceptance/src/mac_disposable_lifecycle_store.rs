//! Durable, no-replace storage for the inert macOS disposable lifecycle.
//!
//! This module owns no effect capability.  It only creates an operation
//! directory and persists canonical lifecycle records before any caller may
//! consider sending an effect request.

use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::mac_disposable_effect_issue_store::DurableEffectIssueStoreErrorV3;
use crate::mac_disposable_effect_issue_store::DurableEffectIssueStoreV3;
use crate::mac_disposable_effect_issue_store::EffectEpochEvidenceV3;
use crate::mac_disposable_effect_issue_store::ExactDisposableCommandV3;
use crate::mac_disposable_effect_issue_store::IssuedEffectRecordV3;
use crate::mac_disposable_effect_issue_store::RetainedEffectIssueReplaySinkV3;
use crate::mac_disposable_effect_issue_store::VerifiedLifecycleIssueRosterV3;
use crate::mac_disposable_lifecycle::CallbackOutcomeV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleEventV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleJournalV2;
use crate::mac_disposable_lifecycle::EffectPurposeV2;
#[cfg(test)]
use crate::mac_disposable_lifecycle::FreshAbsenceObservationV2;
use crate::mac_disposable_lifecycle::LifecycleErrorV2;
use crate::mac_disposable_lifecycle::LifecycleProcessModeV2;
use crate::mac_disposable_lifecycle::PreparedCollectorManifestBindingV3;
#[cfg(test)]
use crate::mac_disposable_lifecycle::ReconciliationSnapshotV2;
use crate::mac_disposable_lifecycle::TerminalDispositionV2;
use crate::mac_disposable_lifecycle::inspect_lifecycle_v2;
use crate::mac_disposable_reconciliation_collector::RestartCollectorErrorV3;
use crate::mac_disposable_reconciliation_collector::RetainedCollectorAppendEventV3;
use crate::mac_disposable_reconciliation_collector::RetainedCollectorMountDeltaV3;
use crate::mac_disposable_reconciliation_collector::RetainedCollectorObservationV3;
use crate::mac_disposable_reconciliation_collector::RetainedPreparedCollectorCapabilityV3;
use crate::mac_disposable_reconciliation_collector::RetainedTerminalAbsenceV3;
use crate::mac_disposable_reconciliation_collector::UnmountingV3;
use crate::mac_inert_one_shot_runner::AuthenticatedDispatchedRunnerV3;
use crate::mac_inert_one_shot_runner::AuthenticatedPreRunnerV3;
use crate::mac_inert_one_shot_runner::FreshProcessEpochV3;
use crate::mac_inert_one_shot_runner::InertDispatchReceiptV3;
use crate::mac_inert_one_shot_runner::InertRunnerErrorV3;
use crate::mac_inert_one_shot_runner::IssuedRunnerDispatchFailureV3;
use crate::mac_inert_one_shot_runner::RecoveredRunnerDeathProofV3;
use crate::mac_inert_one_shot_runner::RunnerIssueReadSealV3;
use crate::mac_inert_one_shot_runner::SameSupervisorRunnerDeathProofV3;
use crate::mac_inert_one_shot_runner::SealedRunnerDispatchV3;
use crate::mac_privileged_disposable_control::BlockingOperationV3;
use crate::mac_privileged_disposable_control::CompletedOperationV3;
use crate::mac_privileged_disposable_control::FreshAdmissionV3;
use crate::mac_privileged_disposable_control::LifecycleRecordAppendSinkV3;
use crate::mac_privileged_disposable_control::PendingUnmountDeltaV3;
use crate::mac_privileged_disposable_control::PrivilegedDisposableControlErrorV2;
use crate::mac_privileged_disposable_control::RetainedControlCensusV3;
use crate::mac_privileged_disposable_control::S1PreparedManifestReadSealV3;
use crate::mac_privileged_disposable_control::StableMountStateV3;
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

const MAX_RECORD_BYTES: usize = 1024 * 1024;
const MAX_RECORDS: usize = 256;
const MAX_TOTAL_RECORD_BYTES: usize = 64 * 1024 * 1024;
const RENAME_EXCL: libc::c_uint = 0x0000_0004;
const PREPARED_MANIFEST_NAME_V3: &str = "prepared-collector-manifest-v3.json";
const PREPARED_MANIFEST_TEMPORARY_NAME_V3: &str = ".incoming-prepared-collector-manifest-v3.json";

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
    #[error(transparent)]
    EffectIssue(#[from] DurableEffectIssueStoreErrorV3),
    #[error(transparent)]
    Runner(#[from] InertRunnerErrorV3),
    #[error(transparent)]
    Collector(#[from] RestartCollectorErrorV3),
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
    PreparedManifestTemporaryCreated,
    PreparedManifestBytesWritten,
    PreparedManifestFileSynced,
    PreparedManifestRenamed,
    PreparedManifestDirectorySynced,
    PreparedManifestFinalReopened,
    PreparedManifestFinalRevalidated,
    TemporarySynced,
    Renamed,
    ParentSynced,
    FinalReopened,
    FinalRevalidated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Binding {
    birthtime_nsec: i64,
    birthtime_sec: i64,
    ctime_nsec: i64,
    ctime_sec: i64,
    dev: u64,
    flags: u32,
    generation: u32,
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
        self.birthtime_nsec == other.birthtime_nsec
            && self.birthtime_sec == other.birthtime_sec
            && self.dev == other.dev
            && self.flags == other.flags
            && self.generation == other.generation
            && self.gid == other.gid
            && self.ino == other.ino
            && self.mode == other.mode
            && self.mtime_nsec == other.mtime_nsec
            && self.mtime_sec == other.mtime_sec
            && self.nlink == other.nlink
            && self.size == other.size
            && self.uid == other.uid
    }

    fn same_directory_across_record_append(self, other: Self) -> bool {
        self.birthtime_nsec == other.birthtime_nsec
            && self.birthtime_sec == other.birthtime_sec
            && self.dev == other.dev
            && self.flags == other.flags
            && self.generation == other.generation
            && self.gid == other.gid
            && self.ino == other.ino
            && self.mode == other.mode
            && self.nlink == other.nlink
            && self.uid == other.uid
    }
}

struct RecordCapsule {
    binding: Binding,
    bytes: Vec<u8>,
    file: File,
    name: String,
}

struct PreparedManifestCapsuleV3 {
    binding: Binding,
    bytes: Vec<u8>,
    digest: String,
    file: File,
}

/// Private sibling seal for the only lifecycle-store reads of the retained
/// prepared collector capability.  Other crate modules can name this type but
/// cannot construct it, so the collector exposes no free-standing manifest,
/// nonce, or digest projection.
pub(crate) struct PreparedCollectorLifecycleSealV3 {
    _private: (),
}

/// Opaque one-shot S2-to-S1 transfer of the exact fixed manifest descriptor.
/// S1 can open it only with its own private seal; the intermediate issue-store
/// handoff cannot inspect or replace any field.
pub(crate) struct PreparedManifestS1TransferV3 {
    binding: Binding,
    bytes: Vec<u8>,
    digest: String,
    file: File,
}

impl PreparedManifestS1TransferV3 {
    fn retain(capsule: &PreparedManifestCapsuleV3) -> io::Result<Self> {
        Ok(Self {
            binding: capsule.binding,
            bytes: capsule.bytes.clone(),
            digest: capsule.digest.clone(),
            file: capsule.file.try_clone()?,
        })
    }

    pub(crate) fn into_s1_parts(
        self,
        _seal: S1PreparedManifestReadSealV3,
    ) -> (File, Vec<u8>, String, (u64, u64)) {
        (
            self.file,
            self.bytes,
            self.digest,
            (self.binding.dev, self.binding.ino),
        )
    }
}

/// Ephemeral sealed view used only for the sibling V3 issue module.  There is
/// no public constructor or wrapper outlet, so production issue replay cannot
/// be invoked with an arbitrary `File`, nonce, UID/GID, or byte vector.
pub(crate) struct RetainedLifecycleIssueSourceV3<'a> {
    directory: &'a File,
    expected_gid: u32,
    expected_uid: u32,
    operation_nonce: &'a str,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl RetainedLifecycleIssueSourceV3<'_> {
    fn new<'a>(
        directory: &'a File,
        operation_nonce: &'a str,
        expected_uid: u32,
        expected_gid: u32,
    ) -> RetainedLifecycleIssueSourceV3<'a> {
        RetainedLifecycleIssueSourceV3 {
            directory,
            expected_gid,
            expected_uid,
            operation_nonce,
            _not_send_or_sync: PhantomData,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test<'a>(
        directory: &'a File,
        operation_nonce: &'a str,
        expected_uid: u32,
        expected_gid: u32,
    ) -> RetainedLifecycleIssueSourceV3<'a> {
        Self::new(directory, operation_nonce, expected_uid, expected_gid)
    }

    pub(crate) fn directory(&self) -> &File {
        self.directory
    }

    pub(crate) fn expected_gid(&self) -> u32 {
        self.expected_gid
    }

    pub(crate) fn expected_uid(&self) -> u32 {
        self.expected_uid
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        self.operation_nonce
    }
}

/// One-shot S1-to-S2 transfer of the exact retained V3 issue directory and
/// issue file capsules.  Only the sealed restart wiring can construct it; the
/// sibling issue module consumes it without reopening a path by name.
pub(crate) struct RetainedEffectIssueSourceV3 {
    directory: File,
    directory_inode: (u64, u64),
    records: Vec<(String, Vec<u8>, File, (u64, u64))>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl RetainedEffectIssueSourceV3 {
    fn new(
        directory: File,
        directory_inode: (u64, u64),
        records: Vec<(String, Vec<u8>, File, (u64, u64))>,
    ) -> Self {
        Self {
            directory,
            directory_inode,
            records,
            _not_send_or_sync: PhantomData,
        }
    }

    pub(crate) fn transfer(
        self,
        sink: RetainedEffectIssueReplaySinkV3<'_, '_>,
    ) -> Result<DurableEffectIssueStoreV3, DurableEffectIssueStoreErrorV3> {
        sink.consume(self.directory, self.directory_inode, self.records)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OperationFormatV3 {
    LegacyV2,
    RequiredEffectIssuesV3,
    RequiredPreparedManifestV3,
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
pub(crate) struct ReconciliationLifecycleEventV3(DisposableLifecycleEventV2);

/// The only fresh-process production append currently admitted by S2.  Later
/// effect records must come from a retained persisted-issue capability rather
/// than from a caller-supplied lifecycle DTO.
pub(crate) struct FreshPreparedLifecycleEventV3(DisposableLifecycleEventV2);

/// Terminal closure is deliberately not part of the ordinary reconciliation
/// append vocabulary.  Persisting it consumes the blocking store and its S1
/// census into a completed typestate.
pub(crate) struct ReconciliationTerminalEventV3(DisposableLifecycleEventV2);

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

    #[cfg(test)]
    pub fn snapshot_observed(snapshot: ReconciliationSnapshotV2) -> Self {
        Self(DisposableLifecycleEventV2::ReconciliationSnapshotObserved { snapshot })
    }

    fn unmount_issued(effect_id: u64) -> Self {
        Self(DisposableLifecycleEventV2::UnmountIssuedOrUncertain {
            effect_id,
            purpose: EffectPurposeV2::Reconciliation,
        })
    }

    fn unmount_callback(effect_id: u64, outcome: CallbackOutcomeV2) -> Self {
        Self(DisposableLifecycleEventV2::UnmountCallbackObserved { effect_id, outcome })
    }

    fn unmount_observed(effect_id: u64, mount_absence_sha256: String) -> Self {
        Self(DisposableLifecycleEventV2::UnmountObserved {
            effect_id,
            mount_absence_sha256,
        })
    }

    fn eject_issued(effect_id: u64) -> Self {
        Self(DisposableLifecycleEventV2::EjectIssuedOrUncertain {
            effect_id,
            purpose: EffectPurposeV2::Reconciliation,
        })
    }

    fn eject_callback(effect_id: u64, outcome: CallbackOutcomeV2) -> Self {
        Self(DisposableLifecycleEventV2::EjectCallbackObserved { effect_id, outcome })
    }

    fn eject_observed(effect_id: u64, iomedia_absence_sha256: String) -> Self {
        Self(DisposableLifecycleEventV2::EjectObserved {
            effect_id,
            iomedia_absence_sha256,
        })
    }

    #[cfg(test)]
    pub fn fresh_absence_observed(observation: FreshAbsenceObservationV2) -> Self {
        Self(DisposableLifecycleEventV2::FreshAbsenceObserved { observation })
    }

    pub fn manual_intervention(reason_sha256: String) -> Self {
        Self(DisposableLifecycleEventV2::ManualIntervention { reason_sha256 })
    }

    pub fn quarantined(reason_sha256: String) -> Self {
        Self(DisposableLifecycleEventV2::Quarantined { reason_sha256 })
    }
}

impl FreshPreparedLifecycleEventV3 {
    fn new(
        baseline_inventory_sha256: String,
        backing_identity_sha256: String,
        boot_session_uuid: String,
        collector_policy_sha256: String,
        mountpoint_underlying_sha256: String,
    ) -> Self {
        Self(DisposableLifecycleEventV2::OperationPrepared {
            baseline_inventory_sha256,
            backing_identity_sha256,
            boot_session_uuid,
            collector_policy_sha256,
            mountpoint_underlying_sha256,
        })
    }

    fn new_bound(
        baseline_inventory_sha256: String,
        backing_identity_sha256: String,
        boot_session_uuid: String,
        collector_policy_sha256: String,
        mountpoint_underlying_sha256: String,
        prepared_manifest: PreparedCollectorManifestBindingV3,
    ) -> Self {
        Self(
            DisposableLifecycleEventV2::OperationPreparedWithManifestV3 {
                baseline_inventory_sha256,
                backing_identity_sha256,
                boot_session_uuid,
                collector_policy_sha256,
                mountpoint_underlying_sha256,
                prepared_manifest,
            },
        )
    }
}

impl ReconciliationTerminalEventV3 {
    fn from_retained_absence(
        absence: &RetainedTerminalAbsenceV3<'_>,
    ) -> Result<Self, DurableLifecycleStoreErrorV3> {
        absence.revalidate().map_err(|error| {
            invalid(format!(
                "retained terminal absence proof failed replay: {error}"
            ))
        })?;
        Ok(Self(DisposableLifecycleEventV2::TerminalAbsenceProved {
            disposition: TerminalDispositionV2::Aborted,
            fresh_absence_sha256: absence.fresh_absence_sha256().to_string(),
        }))
    }

    #[cfg(test)]
    pub(crate) fn absence_proved(
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
    format: OperationFormatV3,
    operation_nonce: String,
    parent: File,
    parent_binding: Binding,
    poisoned: bool,
    prepared_manifest: Option<PreparedManifestCapsuleV3>,
    records: Vec<RecordCapsule>,
    temporary_name: String,
    _mode: PhantomData<fn() -> M>,
}

/// Fresh S2 store whose parent directory was admitted by one consumed S1
/// retained census.  The census and global lock outlive every append.
pub(crate) struct CensusBoundDurableLifecycleStoreV3<'a> {
    census: RetainedControlCensusV3<'a, FreshAdmissionV3, StableMountStateV3>,
    journal: DisposableLifecycleJournalV2,
    poisoned: bool,
    prepared: Option<RetainedPreparedCollectorCapabilityV3>,
    issues: DurableEffectIssueStoreV3,
    store: DurableLifecycleStoreV3<FreshProcessStoreV3>,
}

/// Reconciliation store opened only from the exact blocking S1 census.  It
/// retains that census, the current process epoch, and the replay-derived
/// journal for its entire lifetime; callers cannot substitute a reconstructed
/// journal with matching bytes.
pub(crate) struct ReconciliationOperationStoreV3<'a, 'e> {
    census: RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3>,
    epoch: &'e FreshProcessEpochV3,
    journal: DisposableLifecycleJournalV2,
    poisoned: bool,
    prepared: Option<RetainedPreparedCollectorCapabilityV3>,
    collector: Option<RetainedCollectorObservationV3>,
    issues: DurableEffectIssueStoreV3,
    store: DurableLifecycleStoreV3<ReconciliationOnlyStoreV3>,
}

/// No callback DTO is accepted at the mount-delta boundary.  The runner bridge
/// will eventually construct this token only after validating the exact issued
/// record and a successful authenticated callback.  This inert lane
/// deliberately provides no production constructor.
pub(crate) struct RetainedSuccessfulUnmountCallbackV3 {
    command_sha256: String,
    effect_id: u64,
    issued_record_sha256: String,
    operation_nonce: String,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub(crate) struct AwaitingUnmountCallbackV3;
pub(crate) struct AwaitingUnmountObservationV3;

/// Whole-operation pending-unmount typestate.  It continues to own the exact
/// S1 census, S2 journal and issue store, and the retained pre-effect collector
/// delta.  It has no authority or descriptor outlet.
pub(crate) struct PendingUnmountReconciliationOperationStoreV3<'a, 'e, S> {
    census: RetainedControlCensusV3<'a, BlockingOperationV3, PendingUnmountDeltaV3>,
    command_sha256: String,
    delta: RetainedCollectorMountDeltaV3<UnmountingV3>,
    effect_id: u64,
    epoch: &'e FreshProcessEpochV3,
    issued_record_sha256: String,
    issues: DurableEffectIssueStoreV3,
    journal: DisposableLifecycleJournalV2,
    poisoned: bool,
    prepared: Option<RetainedPreparedCollectorCapabilityV3>,
    store: DurableLifecycleStoreV3<ReconciliationOnlyStoreV3>,
    _state: PhantomData<fn() -> S>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Whole-operation retained issue capability.  It borrows the S1 census,
/// wrapper-owned V2 journal/record descriptors, bound collector evidence and
/// V3 issue store together; no sub-capability or descriptor can be extracted.
pub(crate) struct RetainedOperationEffectIssueV3<'store, 'a, 'e> {
    effect_id: u64,
    record: IssuedEffectRecordV3,
    record_canonical_bytes: Vec<u8>,
    record_sha256: String,
    store: &'store mut ReconciliationOperationStoreV3<'a, 'e>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Token whose private constructor confines typed durable-issue reads to this
/// whole-operation owner. Issue-store APIs accept it by reference but cannot
/// mint it, so they expose no crate-wide record/bytes/digest projection.
pub(crate) struct OperationIssueReadSealV3 {
    _private: (),
}

/// Opaque one-shot handoff from the whole-operation issue capability to the
/// runner module. Its contents can be opened only with the runner-private
/// `RunnerIssueReadSealV3`.
pub(crate) struct SealedRunnerIssueMaterialV3 {
    record: IssuedEffectRecordV3,
    record_canonical_bytes: Vec<u8>,
    record_sha256: String,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Private proof that the exact V2/V3 issue pair was durably replayed,
/// adopted by S1, and revalidated while the whole operation store was held.
/// The control layer may consume this type but cannot construct it.
pub(crate) struct PersistedIssueLeaseSealV3 {
    _private: (),
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// One-shot proof that the latest issued-or-uncertain V2 record and its exact
/// retained V3 issue were replayed as a bijection and are still S1-adopted.
/// Only this module constructs it; S1 consumes it while re-proving the global
/// lease before a recovered death proof can exist.
pub(crate) struct RecoveredIssueVerifierSealV3 {
    _private: (),
    _not_send_or_sync: PhantomData<Rc<()>>,
}

struct PersistedOperationIssueSealV3 {
    effect_id: u64,
    record: IssuedEffectRecordV3,
    record_canonical_bytes: Vec<u8>,
    record_sha256: String,
    lease_seal: PersistedIssueLeaseSealV3,
}

/// The only production-capable S2-to-runner handoff.  `issue` is the actual
/// exclusive borrow of the whole reconciliation store; `dispatch` embeds the
/// exact durable issue bytes and the S1-derived lease.  Neither component has
/// a raw descriptor or digest projection.
pub(crate) struct PersistedIssuedRunnerGrantV3<'store, 'a, 'e> {
    issue: RetainedOperationEffectIssueV3<'store, 'a, 'e>,
    dispatch: Option<SealedRunnerDispatchV3>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Inert dispatch acknowledgement plus the still-live whole-store borrow.
/// This is not a privileged callback-success token.
pub(crate) struct IssuedEffectSessionV3<'store, 'a, 'e> {
    runner: Option<AuthenticatedDispatchedRunnerV3>,
    grant: Option<PersistedIssuedRunnerGrantV3<'store, 'a, 'e>>,
    death_proven: bool,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// A post-durability dispatch failure retains the same whole-store grant until
/// a composite death proof exists.  Drop poisons the in-memory wrapper if
/// fail-closed cleanup cannot prove the runner dead.
pub(crate) struct IssuedEffectDispatchFailureV3<'store, 'a, 'e> {
    error: String,
    runner_failure: Option<IssuedRunnerDispatchFailureV3>,
    grant: Option<PersistedIssuedRunnerGrantV3<'store, 'a, 'e>>,
    death_proven: bool,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub(crate) struct IssuedEffectDeathProvedV3<'store, 'a, 'e> {
    proof: SameSupervisorRunnerDeathProofV3,
    receipt: Option<InertDispatchReceiptV3>,
    _grant: PersistedIssuedRunnerGrantV3<'store, 'a, 'e>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Unforgeable acknowledgement that the reconciliation store has retained
/// and replayed the exact lifecycle record produced by one append.
pub(crate) struct RetainedLifecycleRecordAppendV3 {
    bytes: Vec<u8>,
    directory: File,
    directory_binding: Binding,
    digest: String,
    expected_gid: u32,
    expected_uid: u32,
    name: String,
    operation_name: String,
    record: File,
    record_binding: Binding,
    sequence: u32,
    s1_adopted: bool,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl RetainedLifecycleRecordAppendV3 {
    fn retain<M: StoreModeV3>(
        store: &DurableLifecycleStoreV3<M>,
        expected_digest: &str,
    ) -> Result<Self, DurableLifecycleStoreErrorV3> {
        store.revalidate()?;
        let record = store
            .records
            .last()
            .ok_or_else(|| invalid("durable append did not retain a final record capsule"))?;
        let digest = sha256(&record.bytes);
        if digest != expected_digest {
            return Err(invalid(
                "durable append digest differs from its retained final record bytes",
            ));
        }
        let sequence = u32::try_from(store.records.len())
            .map_err(|_| invalid("lifecycle record sequence overflowed"))?;
        let retained = Self {
            bytes: record.bytes.clone(),
            directory: store.directory.try_clone()?,
            directory_binding: store.directory_binding,
            digest,
            expected_gid: store.expected_gid,
            expected_uid: store.expected_uid,
            name: record.name.clone(),
            operation_name: store.final_name.clone(),
            record: record.file.try_clone()?,
            record_binding: record.binding,
            sequence,
            s1_adopted: false,
            _not_send_or_sync: PhantomData,
        };
        retained.revalidate_exact()?;
        Ok(retained)
    }

    pub(crate) fn digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn sequence(&self) -> u32 {
        self.sequence
    }

    pub(crate) fn revalidate(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        self.revalidate_exact()?;
        if !self.s1_adopted {
            return Err(invalid(
                "lifecycle append capsule has not been adopted by the retained S1 census",
            ));
        }
        Ok(())
    }

    fn revalidate_exact(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        let directory = validate_directory(
            &self.directory,
            self.expected_uid,
            self.expected_gid,
            0o700,
            None,
            "retained lifecycle append operation directory",
        )?;
        let record = validate_regular(
            &self.record,
            self.expected_uid,
            self.expected_gid,
            0o400,
            Some(directory.dev),
            Some(self.bytes.len()),
            "retained lifecycle append record",
        )?;
        if !self
            .directory_binding
            .same_directory_across_record_append(directory)
            || record != self.record_binding
            || named_binding(self.directory.as_raw_fd(), &self.name)? != record
            || read_stable(&self.record, record)? != self.bytes
            || sha256(&self.bytes) != self.digest
            || self.name != record_name(self.sequence as usize)?
        {
            return Err(invalid(
                "retained lifecycle append capsule changed during exact replay",
            ));
        }
        Ok(())
    }

    fn adopt_into_s1(
        &mut self,
        sink: LifecycleRecordAppendSinkV3<'_, '_>,
    ) -> Result<(), DurableLifecycleStoreErrorV3> {
        self.revalidate_exact()?;
        if self.s1_adopted {
            return Err(invalid(
                "retained lifecycle append capsule was already adopted by S1",
            ));
        }
        sink.retain(
            self.directory.try_clone()?,
            self.record.try_clone()?,
            self.bytes.clone(),
            self.operation_name.clone(),
            self.name.clone(),
            self.digest.clone(),
            self.sequence,
        )?;
        self.s1_adopted = true;
        self.revalidate()
    }

    pub(crate) fn require_s1_adopted(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        self.revalidate_exact()?;
        if !self.s1_adopted {
            return Err(invalid(
                "lifecycle append capsule has not been adopted by the retained S1 census",
            ));
        }
        Ok(())
    }
}

impl RetainedSuccessfulUnmountCallbackV3 {
    fn revalidate_against(
        &self,
        effect_id: u64,
        operation_nonce: &str,
        command_sha256: &str,
        issued_record_sha256: &str,
    ) -> Result<(), DurableLifecycleStoreErrorV3> {
        if self.effect_id != effect_id
            || self.operation_nonce != operation_nonce
            || self.command_sha256 != command_sha256
            || self.issued_record_sha256 != issued_record_sha256
        {
            return Err(invalid(
                "successful unmount callback belongs to another operation, issue, or command",
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    fn for_test(
        effect_id: u64,
        operation_nonce: &str,
        command_sha256: &str,
        issued_record_sha256: &str,
    ) -> Self {
        Self {
            command_sha256: command_sha256.to_string(),
            effect_id,
            issued_record_sha256: issued_record_sha256.to_string(),
            operation_nonce: operation_nonce.to_string(),
            _not_send_or_sync: PhantomData,
        }
    }
}

/// Terminally completed reconciliation retains the exact S1 descriptors,
/// replay journal, current epoch, and durable S2 store.  There is no append or
/// fresh-admission method on this state.
pub(crate) struct CompletedReconciliationOperationStoreV3<'a, 'e> {
    census: RetainedControlCensusV3<'a, CompletedOperationV3, StableMountStateV3>,
    epoch: &'e FreshProcessEpochV3,
    issues: DurableEffectIssueStoreV3,
    journal: DisposableLifecycleJournalV2,
    prepared: Option<RetainedPreparedCollectorCapabilityV3>,
    store: DurableLifecycleStoreV3<ReconciliationOnlyStoreV3>,
}

/// Unforgeable one-shot connector constructed only by the census-bound S2
/// entrypoint.  It is consumed by S1 and never exposes its descriptor input.
pub(crate) struct FreshCensusStoreWiringV3 {
    operation_nonce: String,
    prepared: Option<RetainedPreparedCollectorCapabilityV3>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Opaque result of creating a fresh durable store.  Only S1 can admit its
/// final name and bind it to the consumed census.
pub(crate) struct PreparedFreshCensusStoreV3 {
    issues: DurableEffectIssueStoreV3,
    journal: DisposableLifecycleJournalV2,
    prepared: Option<RetainedPreparedCollectorCapabilityV3>,
    store: DurableLifecycleStoreV3<FreshProcessStoreV3>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// Unforgeable one-shot restart connector.  The test hook models a path swap
/// after S1 cloned its exact retained descriptors but before S2 replay.
pub(crate) struct ExistingCensusStoreWiringV3<'e, 'h> {
    before_replay: Option<Box<dyn FnOnce() -> io::Result<()> + 'h>>,
    epoch: &'e FreshProcessEpochV3,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub type ReconciliationDurableLifecycleStoreV3 = DurableLifecycleStoreV3<ReconciliationOnlyStoreV3>;

impl FreshCensusStoreWiringV3 {
    #[cfg(test)]
    fn new(operation_nonce: &str) -> Self {
        Self {
            operation_nonce: operation_nonce.to_string(),
            prepared: None,
            _not_send_or_sync: PhantomData,
        }
    }

    fn from_prepared(
        prepared: RetainedPreparedCollectorCapabilityV3,
    ) -> Result<Self, DurableLifecycleStoreErrorV3> {
        let seal = PreparedCollectorLifecycleSealV3 { _private: () };
        let operation_nonce = prepared.lifecycle_manifest(&seal)?.0.to_string();
        Ok(Self {
            operation_nonce,
            prepared: Some(prepared),
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn wire(
        self,
        operations: File,
        expected_operations_inode: (u64, u64),
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<PreparedFreshCensusStoreV3, DurableLifecycleStoreErrorV3> {
        let parent_binding = binding(&operations)?;
        if (parent_binding.dev, parent_binding.ino) != expected_operations_inode {
            return Err(invalid(
                "fresh S2 parent descriptor differs from the exact S1 operations inode",
            ));
        }
        let journal = DisposableLifecycleJournalV2::new(&self.operation_nonce)?;
        let seal = PreparedCollectorLifecycleSealV3 { _private: () };
        let manifest_bytes = self
            .prepared
            .as_ref()
            .map(|prepared| prepared.lifecycle_manifest(&seal).map(|(_, bytes)| bytes))
            .transpose()?;
        let (store, issues) = DurableLifecycleStoreV3::create_new_format_with_hook(
            &operations,
            &self.operation_nonce,
            expected_uid,
            expected_gid,
            manifest_bytes,
            |_| Ok(()),
        )?;
        Ok(PreparedFreshCensusStoreV3 {
            issues,
            journal,
            prepared: self.prepared,
            store,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl PreparedFreshCensusStoreV3 {
    pub(crate) fn bind<'a>(
        self,
        mut census: RetainedControlCensusV3<'a, FreshAdmissionV3, StableMountStateV3>,
    ) -> Result<CensusBoundDurableLifecycleStoreV3<'a>, DurableLifecycleStoreErrorV3> {
        let operation_directory = self.store.directory.try_clone()?;
        let final_name = self.store.final_name.clone();
        let prepared_manifest = self
            .store
            .prepared_manifest
            .as_ref()
            .map(PreparedManifestS1TransferV3::retain)
            .transpose()?;
        let sink = census.fresh_operation_admission_sink()?;
        self.issues.transfer_prepared_operation_to_s1(
            sink,
            operation_directory,
            final_name,
            prepared_manifest,
        )?;
        Ok(CensusBoundDurableLifecycleStoreV3 {
            census,
            issues: self.issues,
            journal: self.journal,
            poisoned: false,
            prepared: self.prepared,
            store: self.store,
        })
    }
}

impl<'e> ExistingCensusStoreWiringV3<'e, 'static> {
    fn new(epoch: &'e FreshProcessEpochV3) -> Self {
        Self {
            before_replay: None,
            epoch,
            _not_send_or_sync: PhantomData,
        }
    }
}

#[cfg(test)]
impl<'e, 'h> ExistingCensusStoreWiringV3<'e, 'h> {
    fn with_hook<F>(epoch: &'e FreshProcessEpochV3, before_replay: F) -> Self
    where
        F: FnOnce() -> io::Result<()> + 'h,
    {
        Self {
            before_replay: Some(Box::new(before_replay)),
            epoch,
            _not_send_or_sync: PhantomData,
        }
    }
}

impl<'e, 'h> ExistingCensusStoreWiringV3<'e, 'h> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn wire<'a>(
        mut self,
        census: RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3>,
        operations: File,
        expected_operations_inode: (u64, u64),
        directory: File,
        expected_operation_inode: (u64, u64),
        records: Vec<(String, Vec<u8>, File, (u64, u64))>,
        effect_issues: Option<(File, (u64, u64), Vec<(String, Vec<u8>, File, (u64, u64))>)>,
        prepared_manifest: Option<(Vec<u8>, File, (u64, u64))>,
        operation_name: String,
        operation_nonce: String,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<ReconciliationOperationStoreV3<'a, 'e>, DurableLifecycleStoreErrorV3> {
        self.epoch
            .validate_current()
            .map_err(|error| invalid(format!("fresh process epoch is not current: {error}")))?;
        if let Some(before_replay) = self.before_replay.take() {
            before_replay()?;
        }
        let effect_issues = effect_issues.map(|(directory, directory_inode, records)| {
            RetainedEffectIssueSourceV3::new(directory, directory_inode, records)
        });
        let store = DurableLifecycleStoreV3::<FreshProcessStoreV3>::
            open_existing_from_retained_descriptors(
                operations,
                expected_operations_inode,
                directory,
                expected_operation_inode,
                records,
                effect_issues.as_ref(),
                prepared_manifest,
                &operation_name,
                &operation_nonce,
                expected_uid,
                expected_gid,
            )?;
        let exact_prepared_format = store.format == OperationFormatV3::RequiredPreparedManifestV3;
        #[cfg(test)]
        let compatible_test_format = store.format == OperationFormatV3::RequiredEffectIssuesV3;
        #[cfg(not(test))]
        let compatible_test_format = false;
        if !exact_prepared_format && !compatible_test_format {
            return Err(invalid(
                "operation without the exact prepared manifest is blocking and cannot enter the production V3 effect path",
            ));
        }
        let prepared = if let Some(manifest) = store.prepared_manifest.as_ref() {
            let seal = PreparedCollectorLifecycleSealV3 { _private: () };
            Some(
                RetainedPreparedCollectorCapabilityV3::reopen_from_lifecycle_manifest(
                    &operation_nonce,
                    &manifest.bytes,
                    &manifest.digest,
                    &seal,
                )?,
            )
        } else {
            None
        };
        let lifecycle = VerifiedLifecycleIssueRosterV3::capture_from_s2(store.issue_source())?;
        let issues = DurableEffectIssueStoreV3::open_existing_from_retained_s1(
            store.issue_source(),
            effect_issues.ok_or_else(|| {
                invalid("new-format operation lacks retained S1 V3 issue descriptors")
            })?,
            &lifecycle,
        )?;
        let journal = store.resume_for_reconciliation()?;
        census.revalidate()?;
        self.epoch.validate_current().map_err(|error| {
            invalid(format!(
                "fresh process epoch changed during exact descriptor replay: {error}"
            ))
        })?;
        Ok(ReconciliationOperationStoreV3 {
            census,
            epoch: self.epoch,
            collector: None,
            issues,
            journal,
            poisoned: false,
            prepared,
            store,
        })
    }
}

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
        Self::create_inner(
            operations,
            operation_nonce,
            expected_uid,
            expected_gid,
            OperationFormatV3::LegacyV2,
            None,
            &mut hook,
        )
        .map(|(store, issues)| {
            debug_assert!(issues.is_none());
            store
        })
    }

    fn create_new_format_with_hook<F>(
        operations: &File,
        operation_nonce: &str,
        expected_uid: u32,
        expected_gid: u32,
        prepared_manifest_bytes: Option<&[u8]>,
        mut hook: F,
    ) -> Result<(Self, DurableEffectIssueStoreV3), DurableLifecycleStoreErrorV3>
    where
        F: FnMut(CreateCutpointV3) -> io::Result<()>,
    {
        let (store, issues) = Self::create_inner(
            operations,
            operation_nonce,
            expected_uid,
            expected_gid,
            if prepared_manifest_bytes.is_some() {
                OperationFormatV3::RequiredPreparedManifestV3
            } else {
                OperationFormatV3::RequiredEffectIssuesV3
            },
            prepared_manifest_bytes,
            &mut hook,
        )?;
        Ok((
            store,
            issues.ok_or_else(|| invalid("new-format operation lacks its retained issue store"))?,
        ))
    }

    fn create_inner<F>(
        operations: &File,
        operation_nonce: &str,
        expected_uid: u32,
        expected_gid: u32,
        format: OperationFormatV3,
        prepared_manifest_bytes: Option<&[u8]>,
        hook: &mut F,
    ) -> Result<(Self, Option<DurableEffectIssueStoreV3>), DurableLifecycleStoreErrorV3>
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
        let mut temporary_binding = validate_directory(
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
        let issues = if matches!(
            format,
            OperationFormatV3::RequiredEffectIssuesV3
                | OperationFormatV3::RequiredPreparedManifestV3
        ) {
            let issues = DurableEffectIssueStoreV3::create_prepublication(
                RetainedLifecycleIssueSourceV3::new(
                    &temporary,
                    operation_nonce,
                    expected_uid,
                    expected_gid,
                ),
            )?;
            let roster = read_directory_names(temporary.as_raw_fd(), MAX_RECORDS + 1)?;
            if roster != ["effect-issues-v3"] {
                return Err(invalid(
                    "new-format incoming operation lacks its exact mandatory issue directory",
                ));
            }
            temporary_binding = validate_directory(
                &temporary,
                expected_uid,
                expected_gid,
                0o700,
                Some(parent_binding.dev),
                "temporary operation directory after issue-root creation",
            )?;
            Some(issues)
        } else {
            None
        };
        let prepared_manifest = match prepared_manifest_bytes {
            Some(bytes) => Some(publish_prepared_manifest(
                &temporary,
                bytes,
                expected_uid,
                expected_gid,
                parent_binding.dev,
                hook,
            )?),
            None => None,
        };
        if (format == OperationFormatV3::RequiredPreparedManifestV3) != prepared_manifest.is_some()
        {
            return Err(invalid(
                "prepared-manifest operation format differs from its fixed sidecar",
            ));
        }
        if prepared_manifest.is_some() {
            temporary_binding = validate_directory(
                &temporary,
                expected_uid,
                expected_gid,
                0o700,
                Some(parent_binding.dev),
                "temporary operation directory after prepared-manifest publication",
            )?;
        }
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
        let expected_roster = match format {
            OperationFormatV3::LegacyV2 => Vec::new(),
            OperationFormatV3::RequiredEffectIssuesV3 => vec!["effect-issues-v3".to_string()],
            OperationFormatV3::RequiredPreparedManifestV3 => vec![
                "effect-issues-v3".to_string(),
                PREPARED_MANIFEST_NAME_V3.to_string(),
            ],
        };
        if !temporary_binding.stable_across_rename(directory_binding)
            || read_directory_names(directory.as_raw_fd(), MAX_RECORDS + 2)? != expected_roster
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
            format,
            operation_nonce: operation_nonce.to_string(),
            parent: operations.try_clone()?,
            parent_binding: binding(operations)?,
            poisoned: false,
            prepared_manifest,
            records: Vec::new(),
            temporary_name,
            _mode: PhantomData,
        };
        store.revalidate()?;
        hook(CreateCutpointV3::FinalRevalidated)?;
        store.revalidate()?;
        Ok((store, issues))
    }

    #[cfg(test)]
    pub fn open_existing(
        operations: &File,
        operation_nonce: &str,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<ReconciliationDurableLifecycleStoreV3, DurableLifecycleStoreErrorV3> {
        Self::open_existing_inner(operations, operation_nonce, expected_uid, expected_gid)
    }

    #[cfg(test)]
    fn open_existing_inner(
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
        let names = read_directory_names(directory.as_raw_fd(), MAX_RECORDS + 2)?;
        let (format, record_names) = classify_operation_roster(&names)?;
        if record_names.is_empty() {
            return Err(invalid("operation directory has no lifecycle records"));
        }
        let mut records = Vec::with_capacity(record_names.len());
        let mut total_bytes = 0usize;
        for (index, name) in record_names.iter().enumerate() {
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
        let prepared_manifest = if format == OperationFormatV3::RequiredPreparedManifestV3 {
            let capsule = open_record(
                directory.as_raw_fd(),
                PREPARED_MANIFEST_NAME_V3,
                expected_uid,
                expected_gid,
                directory_binding.dev,
            )?;
            Some(PreparedManifestCapsuleV3 {
                digest: sha256(&capsule.bytes),
                binding: capsule.binding,
                bytes: capsule.bytes,
                file: capsule.file,
            })
        } else {
            None
        };
        let store = DurableLifecycleStoreV3::<ReconciliationOnlyStoreV3> {
            directory,
            directory_binding,
            expected_gid,
            expected_uid,
            final_name,
            format,
            operation_nonce: operation_nonce.to_string(),
            parent: operations.try_clone()?,
            parent_binding: binding(operations)?,
            poisoned: false,
            prepared_manifest,
            records,
            temporary_name,
            _mode: PhantomData,
        };
        store.revalidate()?;
        Ok(store)
    }

    #[allow(clippy::too_many_arguments)]
    fn open_existing_from_retained_descriptors(
        operations: File,
        expected_operations_inode: (u64, u64),
        directory: File,
        expected_operation_inode: (u64, u64),
        retained_records: Vec<(String, Vec<u8>, File, (u64, u64))>,
        retained_effect_issues: Option<&RetainedEffectIssueSourceV3>,
        retained_prepared_manifest: Option<(Vec<u8>, File, (u64, u64))>,
        operation_name: &str,
        operation_nonce: &str,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Result<ReconciliationDurableLifecycleStoreV3, DurableLifecycleStoreErrorV3> {
        require_nonce(operation_nonce)?;
        let final_name = format!("operation-{operation_nonce}");
        if operation_name != final_name {
            return Err(invalid(
                "retained S1 operation name differs from its exact lifecycle nonce",
            ));
        }
        let temporary_name = format!(".incoming-operation-{operation_nonce}");
        let parent_binding = validate_directory(
            &operations,
            expected_uid,
            expected_gid,
            0o700,
            None,
            "retained S1 operations directory",
        )?;
        if (parent_binding.dev, parent_binding.ino) != expected_operations_inode {
            return Err(invalid(
                "S2 operations descriptor differs from the exact retained S1 inode",
            ));
        }
        require_absent(operations.as_raw_fd(), &temporary_name)?;
        let directory_binding = validate_directory(
            &directory,
            expected_uid,
            expected_gid,
            0o700,
            Some(parent_binding.dev),
            "retained S1 operation directory",
        )?;
        if (directory_binding.dev, directory_binding.ino) != expected_operation_inode
            || named_binding(operations.as_raw_fd(), &final_name)? != directory_binding
        {
            return Err(invalid(
                "selected operation path no longer names the exact retained S1 directory",
            ));
        }
        if retained_records.is_empty() {
            return Err(invalid(
                "operation directory has no retained lifecycle records",
            ));
        }
        let names = read_directory_names(directory.as_raw_fd(), MAX_RECORDS + 2)?;
        let mut expected_names = retained_records
            .iter()
            .map(|(name, _, _, _)| name.clone())
            .collect::<Vec<_>>();
        let format = match (
            retained_effect_issues.is_some(),
            retained_prepared_manifest.is_some(),
        ) {
            (true, true) => {
                expected_names.push("effect-issues-v3".to_string());
                expected_names.push(PREPARED_MANIFEST_NAME_V3.to_string());
                OperationFormatV3::RequiredPreparedManifestV3
            }
            (true, false) => {
                expected_names.push("effect-issues-v3".to_string());
                OperationFormatV3::RequiredEffectIssuesV3
            }
            (false, false) => OperationFormatV3::LegacyV2,
            (false, true) => {
                return Err(invalid(
                    "prepared manifest exists without the mandatory V3 issue root",
                ));
            }
        };
        expected_names.sort();
        if names != expected_names {
            return Err(invalid(
                "operation roster differs from the full retained S1 record capsules",
            ));
        }
        let mut records = Vec::with_capacity(retained_records.len());
        let mut total_bytes = 0usize;
        for (index, (name, bytes, file, expected_inode)) in retained_records.into_iter().enumerate()
        {
            if name != record_name(index + 1)? {
                return Err(invalid(
                    "retained operation roster contains an unknown name or sequence gap",
                ));
            }
            let record_binding = validate_regular(
                &file,
                expected_uid,
                expected_gid,
                0o400,
                Some(directory_binding.dev),
                Some(bytes.len()),
                "retained S1 lifecycle record",
            )?;
            if (record_binding.dev, record_binding.ino) != expected_inode
                || named_binding(directory.as_raw_fd(), &name)? != record_binding
                || read_stable(&file, record_binding)? != bytes
            {
                return Err(invalid(
                    "S2 lifecycle capsule differs from the exact retained S1 descriptor",
                ));
            }
            total_bytes = total_bytes
                .checked_add(bytes.len())
                .ok_or_else(|| invalid("lifecycle byte count overflowed"))?;
            if total_bytes > MAX_TOTAL_RECORD_BYTES {
                return Err(invalid("lifecycle bytes exceed the fixed total bound"));
            }
            records.push(RecordCapsule {
                binding: record_binding,
                bytes,
                file,
                name,
            });
        }
        let lifecycle = records
            .iter()
            .map(|record| record.bytes.clone())
            .collect::<Vec<_>>();
        let inspection = inspect_lifecycle_v2(&lifecycle)?;
        if inspection.operation_nonce != operation_nonce || !inspection.blocks_new_operations {
            return Err(invalid(
                "retained S1 target is not the exact selected blocking lifecycle",
            ));
        }
        let prepared_manifest = retained_prepared_manifest
            .map(|(bytes, file, expected_inode)| {
                let binding = validate_regular(
                    &file,
                    expected_uid,
                    expected_gid,
                    0o400,
                    Some(directory_binding.dev),
                    Some(bytes.len()),
                    "retained S1 prepared collector manifest",
                )?;
                if (binding.dev, binding.ino) != expected_inode
                    || named_binding(directory.as_raw_fd(), PREPARED_MANIFEST_NAME_V3)? != binding
                    || read_stable(&file, binding)? != bytes
                {
                    return Err(invalid(
                        "S2 prepared manifest differs from the exact retained S1 descriptor",
                    ));
                }
                Ok(PreparedManifestCapsuleV3 {
                    binding,
                    digest: sha256(&bytes),
                    bytes,
                    file,
                })
            })
            .transpose()?;
        let store = DurableLifecycleStoreV3::<ReconciliationOnlyStoreV3> {
            directory,
            directory_binding,
            expected_gid,
            expected_uid,
            final_name,
            format,
            operation_nonce: operation_nonce.to_string(),
            parent: operations,
            parent_binding,
            poisoned: false,
            prepared_manifest,
            records,
            temporary_name,
            _mode: PhantomData,
        };
        store.revalidate()?;
        Ok(store)
    }

    #[cfg(test)]
    pub fn append(
        &mut self,
        journal: &mut DisposableLifecycleJournalV2,
        event: DisposableLifecycleEventV2,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        self.append_mode_with_hook(journal, event, |_| Ok(()))
    }
}

impl<'a> CensusBoundDurableLifecycleStoreV3<'a> {
    #[cfg(test)]
    pub(crate) fn create(
        census: RetainedControlCensusV3<'a, FreshAdmissionV3, StableMountStateV3>,
        operation_nonce: &str,
    ) -> Result<Self, DurableLifecycleStoreErrorV3> {
        census.wire_fresh_store(FreshCensusStoreWiringV3::new(operation_nonce))
    }

    pub(crate) fn create_prepared(
        census: RetainedControlCensusV3<'a, FreshAdmissionV3, StableMountStateV3>,
        prepared: RetainedPreparedCollectorCapabilityV3,
    ) -> Result<Self, DurableLifecycleStoreErrorV3> {
        let wiring = FreshCensusStoreWiringV3::from_prepared(prepared)?;
        census.wire_fresh_store(wiring)
    }

    #[cfg(test)]
    pub(crate) fn persist_prepared(
        &mut self,
        event: FreshPreparedLifecycleEventV3,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        self.append_sealed(event.0)
    }

    pub(crate) fn persist_retained_prepared(
        &mut self,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        let prepared = self.prepared.as_ref().ok_or_else(|| {
            invalid("production prepared append lacks its retained collector capability")
        })?;
        let seal = PreparedCollectorLifecycleSealV3 { _private: () };
        let (
            baseline_inventory_sha256,
            backing_identity_sha256,
            boot_session_uuid,
            collector_policy_sha256,
            mountpoint_underlying_sha256,
        ) = prepared.lifecycle_prepared_fields(&seal)?;
        let manifest =
            self.store.prepared_manifest.as_ref().ok_or_else(|| {
                invalid("production prepared append lost its exact sidecar capsule")
            })?;
        self.append_sealed(
            FreshPreparedLifecycleEventV3::new_bound(
                baseline_inventory_sha256,
                backing_identity_sha256,
                boot_session_uuid,
                collector_policy_sha256,
                mountpoint_underlying_sha256,
                PreparedCollectorManifestBindingV3 {
                    birthtime_nanoseconds: manifest.binding.birthtime_nsec,
                    birthtime_seconds: manifest.binding.birthtime_sec,
                    dev: manifest.binding.dev,
                    generation: manifest.binding.generation,
                    inode: manifest.binding.ino,
                    sha256: manifest.digest.clone(),
                },
            )
            .0,
        )
    }

    #[cfg(test)]
    pub(crate) fn append(
        &mut self,
        event: DisposableLifecycleEventV2,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        self.append_sealed(event)
    }

    fn append_sealed(
        &mut self,
        event: DisposableLifecycleEventV2,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        if self.poisoned {
            return Err(invalid(
                "census-bound store is poisoned; restart reconciliation is required",
            ));
        }
        if let Err(error) = self.revalidate_issues() {
            self.poisoned = true;
            return Err(error);
        }
        if let Err(error) = self.census.revalidate() {
            self.poisoned = true;
            return Err(error.into());
        }
        let digest = self
            .store
            .append_mode_with_hook(&mut self.journal, event, |_| Ok(()))?;
        let mut append = RetainedLifecycleRecordAppendV3::retain(&self.store, &digest)?;
        let sink = self.census.fresh_lifecycle_record_sink()?;
        if let Err(error) = append.adopt_into_s1(sink) {
            self.poisoned = true;
            return Err(error);
        }
        append.require_s1_adopted()?;
        if let Err(error) = self.revalidate_issues() {
            self.poisoned = true;
            return Err(error);
        }
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

    fn revalidate_issues(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        if self.store.records.is_empty() {
            self.issues.revalidate_prepared_empty()?;
        } else {
            let lifecycle =
                VerifiedLifecycleIssueRosterV3::capture_from_s2(self.store.issue_source())?;
            self.issues.revalidate_required(&lifecycle)?;
        }
        Ok(())
    }
}

impl<'a, 'e> ReconciliationOperationStoreV3<'a, 'e> {
    /// Production restart entrypoint.  The raw `File + nonce` opener remains
    /// test-only; production must retain the full S1 census and a current
    /// process epoch while replaying or appending reconciliation records.
    pub(crate) fn open_existing(
        census: RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3>,
        epoch: &'e FreshProcessEpochV3,
    ) -> Result<Self, DurableLifecycleStoreErrorV3> {
        census.wire_existing_store(ExistingCensusStoreWiringV3::new(epoch))
    }

    #[cfg(test)]
    fn open_existing_with_hook<F>(
        census: RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3>,
        epoch: &'e FreshProcessEpochV3,
        before_replay: F,
    ) -> Result<Self, DurableLifecycleStoreErrorV3>
    where
        F: FnOnce() -> io::Result<()>,
    {
        census.wire_existing_store(ExistingCensusStoreWiringV3::with_hook(epoch, before_replay))
    }

    #[cfg(test)]
    pub(crate) fn append_reconciliation(
        &mut self,
        event: ReconciliationLifecycleEventV3,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        self.append_reconciliation_inner(event, false)
            .map(|append| append.digest().to_string())
    }

    pub(crate) fn append_retained_collector(
        &mut self,
        mut retained: RetainedCollectorObservationV3,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        if self.collector.is_some() {
            return Err(invalid(
                "operation already owns a retained collector observation",
            ));
        }
        let (operation_nonce, event) = {
            let capability = retained.append_capability().map_err(|error| {
                invalid(format!(
                    "retained collector evidence failed replay: {error}"
                ))
            })?;
            let event = match capability.event() {
                RetainedCollectorAppendEventV3::ReconciliationSnapshot(snapshot) => {
                    DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
                        snapshot: (*snapshot).clone(),
                    }
                }
                RetainedCollectorAppendEventV3::FreshAbsence(observation) => {
                    DisposableLifecycleEventV2::FreshAbsenceObserved {
                        observation: (*observation).clone(),
                    }
                }
            };
            (capability.operation_nonce().to_string(), event)
        };
        if operation_nonce != self.store.operation_nonce() {
            return Err(invalid(
                "retained collector operation differs from the reconciliation store",
            ));
        }
        let append =
            self.append_reconciliation_inner(ReconciliationLifecycleEventV3(event), false)?;
        let digest = append.digest().to_string();
        if let Err(error) = retained.bind_lifecycle_record(append) {
            self.poisoned = true;
            return Err(invalid(format!(
                "durable collector append could not bind its retained record: {error}"
            )));
        }
        retained.revalidate_bound().map_err(|error| {
            self.poisoned = true;
            invalid(format!(
                "retained collector evidence changed after lifecycle append: {error}"
            ))
        })?;
        self.collector = Some(retained);
        Ok(digest)
    }

    /// Persist one reconciliation issue as a single S2-owned transaction:
    /// exact bound collector -> durable V2 issued tip -> durable V3 issue ->
    /// S1 admission of that exact issue descriptor.  Any cut after the V2
    /// append is issued-or-uncertain and poisons this wrapper.
    fn persist_reconciliation_issue_inner(
        &mut self,
        command: ExactDisposableCommandV3,
        epochs: EffectEpochEvidenceV3,
    ) -> Result<PersistedOperationIssueSealV3, DurableLifecycleStoreErrorV3> {
        if self.poisoned {
            return Err(invalid(
                "reconciliation operation store is poisoned; exact restart replay is required",
            ));
        }
        let collector = self.collector.as_ref().ok_or_else(|| {
            invalid("effect issue requires one wrapper-owned retained collector observation")
        })?;
        collector.revalidate_bound().map_err(|error| {
            invalid(format!(
                "wrapper-owned collector failed replay before issue: {error}"
            ))
        })?;
        let effect_id = self
            .journal
            .last_effect_id()
            .checked_add(1)
            .ok_or_else(|| invalid("effect ID overflowed"))?;
        let event = match &command {
            ExactDisposableCommandV3::UnmountVolume { .. } => {
                DisposableLifecycleEventV2::UnmountIssuedOrUncertain {
                    effect_id,
                    purpose: EffectPurposeV2::Reconciliation,
                }
            }
            ExactDisposableCommandV3::EjectImage { .. } => {
                DisposableLifecycleEventV2::EjectIssuedOrUncertain {
                    effect_id,
                    purpose: EffectPurposeV2::Reconciliation,
                }
            }
            _ => {
                return Err(invalid(
                    "restart reconciliation may issue only unmount or eject commands",
                ));
            }
        };
        let issued_append =
            self.append_reconciliation_inner(ReconciliationLifecycleEventV3(event), true)?;
        let post_durability = (|| {
            issued_append.require_s1_adopted()?;
            let lifecycle =
                VerifiedLifecycleIssueRosterV3::capture_from_s2(self.store.issue_source())?;
            let collector_binding = self
                .collector
                .as_ref()
                .expect("collector is preserved across the issued append")
                .issue_binding()
                .map_err(|error| {
                    invalid(format!(
                        "retained collector could not seal the effect issue: {error}"
                    ))
                })?;
            let mut retained =
                self.issues
                    .persist_bound(&lifecycle, &collector_binding, command, epochs)?;
            let issue_read = OperationIssueReadSealV3 { _private: () };
            let (effect_id, record, record_canonical_bytes, record_sha256) = {
                let effect_id = retained.effect_id();
                retained.revalidate()?;
                let sink = self.census.selected_effect_issue_sink()?;
                retained.adopt_into_s1(sink)?;
                retained.require_s1_adopted()?;
                retained.revalidate()?;
                (
                    effect_id,
                    retained.sealed_record(&issue_read).clone(),
                    retained.sealed_record_canonical_bytes(&issue_read).to_vec(),
                    retained.sealed_record_sha256(&issue_read).to_string(),
                )
            };
            self.revalidate_issue_state(effect_id)?;
            Ok(PersistedOperationIssueSealV3 {
                effect_id,
                record,
                record_canonical_bytes,
                record_sha256,
                lease_seal: PersistedIssueLeaseSealV3 {
                    _private: (),
                    _not_send_or_sync: PhantomData,
                },
            })
        })();
        if post_durability.is_err() {
            self.poisoned = true;
        }
        post_durability
    }

    #[cfg(test)]
    pub(crate) fn persist_reconciliation_issue<'store>(
        &'store mut self,
        command: ExactDisposableCommandV3,
        epochs: EffectEpochEvidenceV3,
    ) -> Result<RetainedOperationEffectIssueV3<'store, 'a, 'e>, DurableLifecycleStoreErrorV3> {
        let persisted = self.persist_reconciliation_issue_inner(command, epochs)?;
        Ok(RetainedOperationEffectIssueV3 {
            effect_id: persisted.effect_id,
            record: persisted.record,
            record_canonical_bytes: persisted.record_canonical_bytes,
            record_sha256: persisted.record_sha256,
            store: self,
            _not_send_or_sync: PhantomData,
        })
    }

    /// Persist, replay, and S1-adopt an exact issue before minting the only
    /// runner grant. The epoch evidence is derived internally from the live
    /// authenticated pre-runner; callers cannot supply nonce/digest strings.
    pub(crate) fn persist_runner_grant<'store>(
        &'store mut self,
        runner: &AuthenticatedPreRunnerV3,
        command: ExactDisposableCommandV3,
    ) -> Result<PersistedIssuedRunnerGrantV3<'store, 'a, 'e>, DurableLifecycleStoreErrorV3> {
        let epoch_binding = runner.bind_effect_epoch(self.epoch)?;
        let epochs = EffectEpochEvidenceV3::from_authenticated(epoch_binding)?;
        let persisted = self.persist_reconciliation_issue_inner(command, epochs)?;
        if let Err(error) = self.revalidate_issue_state(persisted.effect_id) {
            self.poisoned = true;
            return Err(error);
        }
        let dispatch_epoch = match runner.bind_effect_epoch(self.epoch) {
            Ok(epoch) => epoch,
            Err(error) => {
                self.poisoned = true;
                return Err(error.into());
            }
        };
        let lease = match self.census.duplicate_control_lease(persisted.lease_seal) {
            Ok(lease) => lease,
            Err(error) => {
                self.poisoned = true;
                return Err(error.into());
            }
        };
        let mut issue = RetainedOperationEffectIssueV3 {
            effect_id: persisted.effect_id,
            record: persisted.record,
            record_canonical_bytes: persisted.record_canonical_bytes,
            record_sha256: persisted.record_sha256,
            store: self,
            _not_send_or_sync: PhantomData,
        };
        let dispatch = match SealedRunnerDispatchV3::from_retained_issue(
            runner,
            &dispatch_epoch,
            &issue,
            lease,
        ) {
            Ok(dispatch) => dispatch,
            Err(error) => {
                issue.poison_after_unproved_runner_drop();
                return Err(error.into());
            }
        };
        Ok(PersistedIssuedRunnerGrantV3 {
            issue,
            dispatch: Some(dispatch),
            _not_send_or_sync: PhantomData,
        })
    }

    /// Produce the distinct fresh-supervisor death proof for the latest exact
    /// issued-or-uncertain effect. The proof borrows this entire operation,
    /// retains a re-proved S1 global lease, and cannot be confused with the
    /// kqueue/pipe/waitpid proof emitted by a live same-supervisor handle.
    pub(crate) fn recover_latest_runner_death<'store>(
        &'store mut self,
    ) -> Result<RecoveredRunnerDeathProofV3<'store>, DurableLifecycleStoreErrorV3> {
        self.recover_latest_runner_death_inner(|| Ok(()))
    }

    #[cfg(test)]
    fn recover_latest_runner_death_with_hook<'store, F>(
        &'store mut self,
        after_absence_before_final_revalidate: F,
    ) -> Result<RecoveredRunnerDeathProofV3<'store>, DurableLifecycleStoreErrorV3>
    where
        F: FnOnce() -> io::Result<()>,
    {
        self.recover_latest_runner_death_inner(after_absence_before_final_revalidate)
    }

    fn recover_latest_runner_death_inner<'store, F>(
        &'store mut self,
        after_absence_before_final_revalidate: F,
    ) -> Result<RecoveredRunnerDeathProofV3<'store>, DurableLifecycleStoreErrorV3>
    where
        F: FnOnce() -> io::Result<()>,
    {
        if self.poisoned {
            return Err(invalid(
                "poisoned reconciliation store cannot mint a recovered runner proof",
            ));
        }
        let effect_id = self.journal.last_effect_id();
        if effect_id == 0 {
            return Err(invalid(
                "recovered runner proof requires one exact issued-or-uncertain effect",
            ));
        }
        self.revalidate_recovery_issue_state(effect_id)?;
        let issue_read = OperationIssueReadSealV3 { _private: () };
        let record = self
            .issues
            .replayed_issue_sealed(effect_id, &issue_read)
            .ok_or_else(|| invalid("latest recovered effect lost its exact V3 issue"))?
            .clone();
        if record.effect_id() != effect_id || record.operation_nonce() != self.operation_nonce() {
            return Err(invalid(
                "recovered issue differs from the exact blocking operation or effect",
            ));
        }
        let issued_record_sha256 = sha256(&canonical_json(&record).map_err(|error| {
            invalid(format!(
                "recovered issue is not canonical after retained replay: {error}"
            ))
        })?);
        let issue_seal = RecoveredIssueVerifierSealV3 {
            _private: (),
            _not_send_or_sync: PhantomData,
        };
        let control_lease = self.census.seal_recovered_control_lease(issue_seal)?;
        RecoveredRunnerDeathProofV3::from_exact_replay(
            control_lease,
            self.epoch,
            record.boot_session_uuid().to_string(),
            issued_record_sha256,
            record.command_sha256().to_string(),
            record.effect_id(),
            record.operation_nonce().to_string(),
            record.purpose(),
            record.supervisor_pid(),
            record.supervisor_parent_pid(),
            record.supervisor_kernel_start_microseconds(),
            record.runner_pid(),
            record.runner_kernel_start_microseconds(),
            || {
                after_absence_before_final_revalidate().map_err(|error| error.to_string())?;
                self.revalidate_recovery_issue_state(effect_id)
                    .map_err(|error| error.to_string())
            },
        )
        .map_err(Into::into)
    }

    /// Consume the stable reconciliation wrapper into a pending unmount using
    /// only its latest exact S2-owned issue and its wrapper-owned bound
    /// UniqueMounted collector.  No caller-supplied effect ID, command bytes,
    /// digest, or mount-table DTO is accepted.
    pub(crate) fn begin_latest_unmount_delta(
        mut self,
    ) -> Result<
        PendingUnmountReconciliationOperationStoreV3<'a, 'e, AwaitingUnmountCallbackV3>,
        DurableLifecycleStoreErrorV3,
    > {
        if self.poisoned {
            return Err(invalid(
                "poisoned reconciliation store cannot begin an unmount delta",
            ));
        }
        let effect_id = self.journal.last_effect_id();
        if effect_id == 0 {
            return Err(invalid(
                "unmount delta requires the latest durable reconciliation issue",
            ));
        }
        self.revalidate_issue_state(effect_id)?;
        let (command, command_sha256, issued_record_sha256) = {
            let issue_read = OperationIssueReadSealV3 { _private: () };
            let issue = self
                .issues
                .replayed_issue_sealed(effect_id, &issue_read)
                .ok_or_else(|| invalid("latest durable effect has no retained V3 issue"))?;
            let record_sha256 = sha256(&canonical_json(issue).map_err(|error| {
                invalid(format!(
                    "latest retained V3 issue is not canonical after exact replay: {error}"
                ))
            })?);
            (
                issue.command().clone(),
                issue.command_sha256().to_string(),
                record_sha256,
            )
        };
        let collector = self
            .collector
            .take()
            .ok_or_else(|| invalid("latest unmount issue lost its retained collector"))?;
        let delta = collector.into_unmount_delta(&command).map_err(|error| {
            invalid(format!(
                "latest durable unmount issue cannot seal an exact mount delta: {error}"
            ))
        })?;
        delta.revalidate_live_pending().map_err(|error| {
            invalid(format!(
                "exact unmount delta changed before S1 entered PendingUnmount: {error}"
            ))
        })?;
        let ReconciliationOperationStoreV3 {
            census,
            epoch,
            journal,
            poisoned,
            prepared,
            collector: _,
            issues,
            store,
        } = self;
        let census = census.begin_unmount_delta(delta.sealed_plan())?;
        let pending = PendingUnmountReconciliationOperationStoreV3 {
            census,
            command_sha256,
            delta,
            effect_id,
            epoch,
            issued_record_sha256,
            issues,
            journal,
            poisoned,
            prepared,
            store,
            _state: PhantomData,
            _not_send_or_sync: PhantomData,
        };
        pending.revalidate_pending_issue()?;
        Ok(pending)
    }

    fn revalidate_issue_state(&self, effect_id: u64) -> Result<(), DurableLifecycleStoreErrorV3> {
        self.census.revalidate()?;
        self.epoch.validate_current().map_err(|error| {
            invalid(format!(
                "fresh process epoch changed during issue replay: {error}"
            ))
        })?;
        self.collector
            .as_ref()
            .ok_or_else(|| invalid("retained issue lost its collector capability"))?
            .issue_binding()
            .map_err(|error| invalid(format!("retained issue collector replay failed: {error}")))?
            .revalidate()
            .map_err(|error| invalid(format!("retained issue collector changed: {error}")))?;
        let lifecycle = VerifiedLifecycleIssueRosterV3::capture_from_s2(self.store.issue_source())?;
        self.issues.revalidate_required(&lifecycle)?;
        self.issues.revalidate_s1_adopted()?;
        let issue_read = OperationIssueReadSealV3 { _private: () };
        if self
            .issues
            .replayed_issue_sealed(effect_id, &issue_read)
            .is_none()
        {
            return Err(invalid(
                "retained V3 issue disappeared from the exact bijection",
            ));
        }
        Ok(())
    }

    fn revalidate_recovery_issue_state(
        &self,
        effect_id: u64,
    ) -> Result<(), DurableLifecycleStoreErrorV3> {
        self.census.revalidate()?;
        self.epoch.validate_current().map_err(|error| {
            invalid(format!(
                "fresh process epoch changed during recovered issue replay: {error}"
            ))
        })?;
        let lifecycle = VerifiedLifecycleIssueRosterV3::capture_from_s2(self.store.issue_source())?;
        self.issues.revalidate_required(&lifecycle)?;
        self.issues.revalidate_s1_adopted()?;
        let issue_read = OperationIssueReadSealV3 { _private: () };
        if self
            .issues
            .replayed_issue_sealed(effect_id, &issue_read)
            .is_none()
        {
            return Err(invalid(
                "recovered V3 issue disappeared from the exact V2/V3 bijection",
            ));
        }
        Ok(())
    }

    fn append_reconciliation_inner(
        &mut self,
        event: ReconciliationLifecycleEventV3,
        preserve_collector: bool,
    ) -> Result<RetainedLifecycleRecordAppendV3, DurableLifecycleStoreErrorV3> {
        self.append_reconciliation_inner_with_adoption_hook(event, preserve_collector, || Ok(()))
    }

    #[cfg(test)]
    fn append_reconciliation_with_s1_adoption_hook<F>(
        &mut self,
        event: ReconciliationLifecycleEventV3,
        hook: F,
    ) -> Result<String, DurableLifecycleStoreErrorV3>
    where
        F: FnOnce() -> io::Result<()>,
    {
        self.append_reconciliation_inner_with_adoption_hook(event, false, hook)
            .map(|append| append.digest().to_string())
    }

    fn append_reconciliation_inner_with_adoption_hook<F>(
        &mut self,
        event: ReconciliationLifecycleEventV3,
        preserve_collector: bool,
        hook: F,
    ) -> Result<RetainedLifecycleRecordAppendV3, DurableLifecycleStoreErrorV3>
    where
        F: FnOnce() -> io::Result<()>,
    {
        if self.poisoned {
            return Err(invalid(
                "reconciliation operation store is poisoned; a fresh exact census is required",
            ));
        }
        if let Err(error) = self.revalidate_existing_issues() {
            self.poisoned = true;
            return Err(error);
        }
        if let Err(error) = self.census.revalidate() {
            self.poisoned = true;
            return Err(error.into());
        }
        if let Err(error) = self.epoch.validate_current() {
            self.poisoned = true;
            return Err(invalid(format!(
                "fresh process epoch changed before reconciliation append: {error}"
            )));
        }
        if matches!(
            &event.0,
            DisposableLifecycleEventV2::TerminalAbsenceProved { .. }
        ) {
            return Err(invalid(
                "terminal closure must consume Blocking into the completed typestate",
            ));
        }
        let digest = self.store.append_reconciliation(&mut self.journal, event)?;
        let mut append = RetainedLifecycleRecordAppendV3::retain(&self.store, &digest)?;
        if let Err(error) = hook() {
            self.poisoned = true;
            return Err(error.into());
        }
        let sink = self.census.selected_lifecycle_record_sink()?;
        if let Err(error) = append.adopt_into_s1(sink) {
            self.poisoned = true;
            return Err(error);
        }
        append.require_s1_adopted()?;
        if let Err(error) = self.epoch.validate_current() {
            self.poisoned = true;
            return Err(invalid(format!(
                "fresh process epoch changed after reconciliation append: {error}"
            )));
        }
        if !preserve_collector {
            if let Err(error) = self.revalidate_existing_issues() {
                self.poisoned = true;
                return Err(error);
            }
        }
        if !preserve_collector {
            self.collector = None;
        }
        Ok(append)
    }

    pub(crate) fn complete_reconciliation_from_retained_absence(
        self,
    ) -> Result<CompletedReconciliationOperationStoreV3<'a, 'e>, DurableLifecycleStoreErrorV3> {
        let event = {
            let retained = self.collector.as_ref().ok_or_else(|| {
                invalid("terminal closure requires a wrapper-owned retained FreshAbsence")
            })?;
            let absence = retained.terminal_absence().map_err(|error| {
                invalid(format!("retained terminal absence is invalid: {error}"))
            })?;
            if absence.operation_nonce() != self.store.operation_nonce() {
                return Err(invalid(
                    "terminal FreshAbsence operation differs from the reconciliation store",
                ));
            }
            ReconciliationTerminalEventV3::from_retained_absence(&absence)?
        };
        self.complete_reconciliation_inner(event)
    }

    #[cfg(test)]
    pub(crate) fn complete_reconciliation(
        self,
        event: ReconciliationTerminalEventV3,
    ) -> Result<CompletedReconciliationOperationStoreV3<'a, 'e>, DurableLifecycleStoreErrorV3> {
        self.complete_reconciliation_inner(event)
    }

    fn complete_reconciliation_inner(
        mut self,
        event: ReconciliationTerminalEventV3,
    ) -> Result<CompletedReconciliationOperationStoreV3<'a, 'e>, DurableLifecycleStoreErrorV3> {
        if self.poisoned {
            return Err(invalid(
                "poisoned reconciliation store cannot perform terminal completion",
            ));
        }
        self.revalidate_existing_issues()?;
        self.census.revalidate()?;
        self.epoch.validate_current().map_err(|error| {
            invalid(format!(
                "fresh process epoch changed before terminal append: {error}"
            ))
        })?;
        let digest = self
            .store
            .append_reconciliation_terminal(&mut self.journal, event)?;
        let mut append = RetainedLifecycleRecordAppendV3::retain(&self.store, &digest)?;
        let sink = self.census.selected_terminal_record_sink()?;
        append.adopt_into_s1(sink)?;
        let census = self.census.complete_selected_lifecycle_append(&append)?;
        let lifecycle = VerifiedLifecycleIssueRosterV3::capture_from_s2(self.store.issue_source())?;
        self.issues.revalidate_required(&lifecycle)?;
        self.epoch.validate_current().map_err(|error| {
            invalid(format!(
                "fresh process epoch changed after terminal append: {error}"
            ))
        })?;
        Ok(CompletedReconciliationOperationStoreV3 {
            census,
            epoch: self.epoch,
            issues: self.issues,
            journal: self.journal,
            prepared: self.prepared,
            store: self.store,
        })
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        self.store.operation_nonce()
    }

    pub(crate) fn poisoned(&self) -> bool {
        self.poisoned || self.store.poisoned()
    }

    fn revalidate_existing_issues(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        let lifecycle = VerifiedLifecycleIssueRosterV3::capture_from_s2(self.store.issue_source())?;
        self.issues.revalidate_required(&lifecycle)?;
        self.issues.revalidate_s1_adopted()?;
        Ok(())
    }
}

impl<S> PendingUnmountReconciliationOperationStoreV3<'_, '_, S> {
    fn revalidate_pending_issue(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        if self.poisoned || self.store.poisoned() || self.issues.poisoned() {
            return Err(invalid(
                "pending-unmount operation is poisoned; exact restart replay is required",
            ));
        }
        self.census.revalidate()?;
        self.epoch.validate_current().map_err(|error| {
            invalid(format!(
                "fresh process epoch changed during pending-unmount replay: {error}"
            ))
        })?;
        self.delta.revalidate_live_pending().map_err(|error| {
            invalid(format!(
                "pending-unmount collector delta failed exact live replay: {error}"
            ))
        })?;
        let lifecycle = VerifiedLifecycleIssueRosterV3::capture_from_s2(self.store.issue_source())?;
        self.issues.revalidate_required(&lifecycle)?;
        self.issues.revalidate_s1_adopted()?;
        let issue_read = OperationIssueReadSealV3 { _private: () };
        let issue = self
            .issues
            .replayed_issue_sealed(self.effect_id, &issue_read)
            .ok_or_else(|| invalid("pending-unmount V3 issue disappeared"))?;
        let issued_record_sha256 = sha256(&canonical_json(issue).map_err(|error| {
            invalid(format!(
                "pending-unmount retained V3 issue is not canonical after exact replay: {error}"
            ))
        })?);
        let plan = self.delta.sealed_plan();
        if issue.operation_nonce() != self.store.operation_nonce()
            || issue.command_sha256() != self.command_sha256
            || plan.operation_nonce() != self.store.operation_nonce()
            || plan.command_sha256() != self.command_sha256
            || issued_record_sha256 != self.issued_record_sha256
        {
            return Err(invalid(
                "pending-unmount operation, command, collector delta, or retained issue diverged",
            ));
        }
        Ok(())
    }

    fn append_pending_unmount(
        &mut self,
        event: ReconciliationLifecycleEventV3,
    ) -> Result<RetainedLifecycleRecordAppendV3, DurableLifecycleStoreErrorV3> {
        self.revalidate_pending_issue()?;
        let valid_event = matches!(
            &event.0,
            DisposableLifecycleEventV2::UnmountCallbackObserved {
                effect_id,
                outcome: CallbackOutcomeV2::Succeeded,
            } if *effect_id == self.effect_id
        ) || matches!(
            &event.0,
            DisposableLifecycleEventV2::UnmountObserved { effect_id, .. }
                if *effect_id == self.effect_id
        );
        if !valid_event {
            return Err(invalid(
                "pending-unmount typestate accepts only its exact successful callback or observation",
            ));
        }
        let digest = match self.store.append_reconciliation(&mut self.journal, event) {
            Ok(digest) => digest,
            Err(error) => {
                self.poisoned = true;
                return Err(error);
            }
        };
        let mut append = RetainedLifecycleRecordAppendV3::retain(&self.store, &digest)?;
        let sink = self.census.selected_lifecycle_record_sink()?;
        if let Err(error) = append.adopt_into_s1(sink) {
            self.poisoned = true;
            return Err(error);
        }
        append.require_s1_adopted()?;
        if let Err(error) = self.revalidate_pending_issue() {
            self.poisoned = true;
            return Err(error);
        }
        Ok(append)
    }
}

impl<'a, 'e> PendingUnmountReconciliationOperationStoreV3<'a, 'e, AwaitingUnmountCallbackV3> {
    /// Append callback success only from the sealed runner token.  There is no
    /// production constructor for that token in this inert lane.
    pub(crate) fn append_successful_callback(
        mut self,
        callback: RetainedSuccessfulUnmountCallbackV3,
    ) -> Result<
        PendingUnmountReconciliationOperationStoreV3<'a, 'e, AwaitingUnmountObservationV3>,
        DurableLifecycleStoreErrorV3,
    > {
        callback.revalidate_against(
            self.effect_id,
            self.store.operation_nonce(),
            &self.command_sha256,
            &self.issued_record_sha256,
        )?;
        let append =
            self.append_pending_unmount(ReconciliationLifecycleEventV3::unmount_callback(
                self.effect_id,
                CallbackOutcomeV2::Succeeded,
            ))?;
        append.require_s1_adopted()?;
        let PendingUnmountReconciliationOperationStoreV3 {
            census,
            command_sha256,
            delta,
            effect_id,
            epoch,
            issued_record_sha256,
            issues,
            journal,
            poisoned,
            prepared,
            store,
            _state: _,
            _not_send_or_sync: _,
        } = self;
        Ok(PendingUnmountReconciliationOperationStoreV3 {
            census,
            command_sha256,
            delta,
            effect_id,
            epoch,
            issued_record_sha256,
            issues,
            journal,
            poisoned,
            prepared,
            store,
            _state: PhantomData,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl<'a, 'e> PendingUnmountReconciliationOperationStoreV3<'a, 'e, AwaitingUnmountObservationV3> {
    /// Durable post-unmount collection and S1 adoption are inseparable from the
    /// PendingUnmount -> Stable transition.  A plain observation DTO or digest
    /// cannot reach this method.
    pub(crate) fn append_retained_observation_and_advance(
        mut self,
        mut next: RetainedCollectorObservationV3,
    ) -> Result<ReconciliationOperationStoreV3<'a, 'e>, DurableLifecycleStoreErrorV3> {
        let (operation_nonce, mount_absence_sha256) = {
            let observation = self.delta.seal_observation(&next).map_err(|error| {
                invalid(format!(
                    "post-unmount collector cannot seal the exact observation: {error}"
                ))
            })?;
            (
                observation.operation_nonce().to_string(),
                observation.mount_evidence_sha256().to_string(),
            )
        };
        if operation_nonce != self.store.operation_nonce() {
            return Err(invalid(
                "post-unmount collector belongs to another operation",
            ));
        }
        let append = self.append_pending_unmount(
            ReconciliationLifecycleEventV3::unmount_observed(self.effect_id, mount_absence_sha256),
        )?;
        if let Err(error) = next.bind_lifecycle_record(append) {
            self.poisoned = true;
            return Err(invalid(format!(
                "post-unmount collector could not bind its exact S1-adopted lifecycle capsule: {error}"
            )));
        }
        next.revalidate_bound().map_err(|error| {
            self.poisoned = true;
            invalid(format!(
                "post-unmount collector changed after durable lifecycle adoption: {error}"
            ))
        })?;

        let PendingUnmountReconciliationOperationStoreV3 {
            census,
            command_sha256: _,
            delta,
            effect_id: _,
            epoch,
            issued_record_sha256: _,
            issues,
            journal,
            poisoned,
            prepared,
            store,
            _state: _,
            _not_send_or_sync: _,
        } = self;
        let advance = delta.seal_advance(&next).map_err(|error| {
            invalid(format!(
                "post-unmount retained evidence cannot seal the S1 stable advance: {error}"
            ))
        })?;
        let census = census.advance_unmount_delta(advance)?;
        let stable = ReconciliationOperationStoreV3 {
            census,
            epoch,
            journal,
            poisoned,
            prepared,
            collector: Some(next),
            issues,
            store,
        };
        stable.census.revalidate()?;
        stable.epoch.validate_current().map_err(|error| {
            invalid(format!(
                "fresh process epoch changed after unmount-delta advance: {error}"
            ))
        })?;
        stable.revalidate_existing_issues()?;
        stable
            .collector
            .as_ref()
            .expect("stable unmount transition retains its post-effect collector")
            .revalidate_bound()
            .map_err(|error| {
                invalid(format!(
                    "post-unmount collector changed after stable transition: {error}"
                ))
            })?;
        Ok(stable)
    }
}

impl RetainedOperationEffectIssueV3<'_, '_, '_> {
    pub(crate) fn effect_id(&self) -> u64 {
        self.effect_id
    }

    pub(crate) fn seal_runner_issue(
        &self,
        _runner: &RunnerIssueReadSealV3,
    ) -> Result<SealedRunnerIssueMaterialV3, DurableLifecycleStoreErrorV3> {
        self.revalidate()?;
        Ok(SealedRunnerIssueMaterialV3 {
            record: self.record.clone(),
            record_canonical_bytes: self.record_canonical_bytes.clone(),
            record_sha256: self.record_sha256.clone(),
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn poison_after_unproved_runner_drop(&mut self) {
        self.store.poisoned = true;
    }

    pub(crate) fn revalidate(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        self.store.revalidate_issue_state(self.effect_id)
    }
}

impl SealedRunnerIssueMaterialV3 {
    pub(crate) fn into_runner_parts(
        self,
        _runner: RunnerIssueReadSealV3,
    ) -> (IssuedEffectRecordV3, Vec<u8>, String) {
        (self.record, self.record_canonical_bytes, self.record_sha256)
    }
}

impl<'store, 'a, 'e> PersistedIssuedRunnerGrantV3<'store, 'a, 'e> {
    pub(crate) fn effect_id(&self) -> u64 {
        self.issue.effect_id()
    }

    pub(crate) fn revalidate(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        self.issue.revalidate()
    }

    pub(crate) fn dispatch(
        mut self,
        runner: AuthenticatedPreRunnerV3,
        timeout: std::time::Duration,
    ) -> Result<IssuedEffectSessionV3<'store, 'a, 'e>, IssuedEffectDispatchFailureV3<'store, 'a, 'e>>
    {
        if let Err(error) = self.issue.revalidate() {
            self.issue.poison_after_unproved_runner_drop();
            drop(runner);
            return Err(IssuedEffectDispatchFailureV3 {
                error: format!("exact S2 issue changed before dispatch: {error}"),
                runner_failure: None,
                grant: Some(self),
                death_proven: true,
                _not_send_or_sync: PhantomData,
            });
        }
        let dispatch = self
            .dispatch
            .take()
            .expect("persisted runner grant is one-shot and still sealed");
        match runner.dispatch_sealed(dispatch, timeout) {
            Ok(runner) => Ok(IssuedEffectSessionV3 {
                runner: Some(runner),
                grant: Some(self),
                death_proven: false,
                _not_send_or_sync: PhantomData,
            }),
            Err(failure) => {
                let error = failure.error().to_string();
                let death_proven = failure.has_death_proof();
                Err(IssuedEffectDispatchFailureV3 {
                    error,
                    runner_failure: Some(failure),
                    grant: Some(self),
                    death_proven,
                    _not_send_or_sync: PhantomData,
                })
            }
        }
    }
}

impl<'store, 'a, 'e> IssuedEffectSessionV3<'store, 'a, 'e> {
    pub(crate) fn receipt(&self) -> &InertDispatchReceiptV3 {
        self.runner
            .as_ref()
            .expect("live issued session retains its runner")
            .receipt()
    }

    pub(crate) fn ensure_death_proof(
        &mut self,
        timeout: std::time::Duration,
    ) -> Result<(), InertRunnerErrorV3> {
        self.runner
            .as_mut()
            .ok_or_else(|| {
                InertRunnerErrorV3::Invalid(
                    "issued session runner was already consumed".to_string(),
                )
            })?
            .ensure_death_proof(timeout)
    }

    pub(crate) fn finish_after_death_proof(
        mut self,
        timeout: std::time::Duration,
    ) -> Result<IssuedEffectDeathProvedV3<'store, 'a, 'e>, InertRunnerErrorV3> {
        self.ensure_death_proof(timeout)?;
        let mut runner = self
            .runner
            .take()
            .expect("issued session retains its runner until death proof");
        let proof = runner.take_death_proof()?;
        let receipt = runner.receipt().clone();
        let grant = self
            .grant
            .take()
            .expect("issued session retains its whole-store grant");
        self.death_proven = true;
        Ok(IssuedEffectDeathProvedV3 {
            proof,
            receipt: Some(receipt),
            _grant: grant,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl Drop for IssuedEffectSessionV3<'_, '_, '_> {
    fn drop(&mut self) {
        if self.death_proven {
            return;
        }
        let proved = self
            .runner
            .as_mut()
            .map(|runner| runner.ensure_death_proof(std::time::Duration::from_secs(5)))
            .transpose()
            .is_ok();
        if !proved {
            if let Some(grant) = self.grant.as_mut() {
                grant.issue.poison_after_unproved_runner_drop();
            }
        }
    }
}

impl<'store, 'a, 'e> IssuedEffectDispatchFailureV3<'store, 'a, 'e> {
    pub(crate) fn error(&self) -> &str {
        &self.error
    }

    pub(crate) fn finish_after_death_proof(
        mut self,
        timeout: std::time::Duration,
    ) -> Result<IssuedEffectDeathProvedV3<'store, 'a, 'e>, InertRunnerErrorV3> {
        let failure = self.runner_failure.as_mut().ok_or_else(|| {
            InertRunnerErrorV3::Invalid(
                "pre-dispatch rejection has no issued runner death receipt".to_string(),
            )
        })?;
        failure.ensure_death_proof(timeout)?;
        let proof = failure.take_death_proof()?;
        let grant = self
            .grant
            .take()
            .expect("failed dispatch retains its whole-store grant");
        self.death_proven = true;
        Ok(IssuedEffectDeathProvedV3 {
            proof,
            receipt: None,
            _grant: grant,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl Drop for IssuedEffectDispatchFailureV3<'_, '_, '_> {
    fn drop(&mut self) {
        if self.death_proven {
            return;
        }
        let proved = self
            .runner_failure
            .as_mut()
            .map(|failure| failure.ensure_death_proof(std::time::Duration::from_secs(5)))
            .transpose()
            .is_ok();
        if !proved {
            if let Some(grant) = self.grant.as_mut() {
                grant.issue.poison_after_unproved_runner_drop();
            }
        }
    }
}

impl IssuedEffectDeathProvedV3<'_, '_, '_> {
    pub(crate) fn proof(&self) -> &SameSupervisorRunnerDeathProofV3 {
        &self.proof
    }

    pub(crate) fn receipt(&self) -> Option<&InertDispatchReceiptV3> {
        self.receipt.as_ref()
    }

    pub(crate) fn into_same_supervisor_proof(self) -> SameSupervisorRunnerDeathProofV3 {
        self.proof
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

    fn append_reconciliation(
        &mut self,
        journal: &mut DisposableLifecycleJournalV2,
        event: ReconciliationLifecycleEventV3,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        self.append_mode_with_hook(journal, event.0, |_| Ok(()))
    }

    fn append_reconciliation_terminal(
        &mut self,
        journal: &mut DisposableLifecycleJournalV2,
        event: ReconciliationTerminalEventV3,
    ) -> Result<String, DurableLifecycleStoreErrorV3> {
        self.append_mode_with_hook(journal, event.0, |_| Ok(()))
    }
}

impl CompletedReconciliationOperationStoreV3<'_, '_> {
    #[cfg(test)]
    fn revalidate(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        self.census.revalidate()?;
        self.store.revalidate()?;
        let lifecycle = VerifiedLifecycleIssueRosterV3::capture_from_s2(self.store.issue_source())?;
        self.issues.revalidate_required(&lifecycle)?;
        self.epoch
            .validate_current()
            .map_err(|error| invalid(format!("completed epoch changed: {error}")))?;
        if !matches!(
            self.journal.disposition(),
            crate::mac_disposable_lifecycle::LifecycleDispositionV2::TerminalCompleted
                | crate::mac_disposable_lifecycle::LifecycleDispositionV2::TerminalAborted
        ) {
            return Err(invalid(
                "completed operation retained a nonterminal replay journal",
            ));
        }
        Ok(())
    }
}

impl<M: StoreModeV3> DurableLifecycleStoreV3<M> {
    pub fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    pub fn poisoned(&self) -> bool {
        self.poisoned
    }

    fn issue_source(&self) -> RetainedLifecycleIssueSourceV3<'_> {
        RetainedLifecycleIssueSourceV3::new(
            &self.directory,
            &self.operation_nonce,
            self.expected_uid,
            self.expected_gid,
        )
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
        self.validate_prepared_append_event(&event)?;
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
        // From this point onward an error may follow a durable filesystem
        // mutation.  Fail closed until the complete publish, descriptor
        // retention, and exact replay sequence has succeeded.
        self.poisoned = true;
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
            Ok(digest) => {
                self.poisoned = false;
                Ok(digest)
            }
            Err(error) => Err(error.into()),
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
        let names = read_directory_names(self.directory.as_raw_fd(), MAX_RECORDS + 2)?;
        let mut expected = (1..=self.records.len())
            .map(record_name)
            .collect::<Result<Vec<_>, _>>()?;
        if matches!(
            self.format,
            OperationFormatV3::RequiredEffectIssuesV3
                | OperationFormatV3::RequiredPreparedManifestV3
        ) {
            expected.push("effect-issues-v3".to_string());
        }
        if self.format == OperationFormatV3::RequiredPreparedManifestV3 {
            expected.push(PREPARED_MANIFEST_NAME_V3.to_string());
        }
        expected.sort();
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
        match (&self.format, &self.prepared_manifest) {
            (OperationFormatV3::RequiredPreparedManifestV3, Some(manifest)) => {
                require_absent(
                    self.directory.as_raw_fd(),
                    PREPARED_MANIFEST_TEMPORARY_NAME_V3,
                )?;
                if named_binding(self.directory.as_raw_fd(), PREPARED_MANIFEST_NAME_V3)?
                    != manifest.binding
                    || binding(&manifest.file)? != manifest.binding
                    || read_stable(&manifest.file, manifest.binding)? != manifest.bytes
                    || sha256(&manifest.bytes) != manifest.digest
                {
                    return Err(invalid(
                        "durable prepared collector manifest changed during replay",
                    ));
                }
                validate_regular(
                    &manifest.file,
                    self.expected_uid,
                    self.expected_gid,
                    0o400,
                    Some(self.directory_binding.dev),
                    Some(manifest.bytes.len()),
                    "prepared collector manifest replay",
                )?;
            }
            (OperationFormatV3::RequiredPreparedManifestV3, None) => {
                return Err(invalid(
                    "prepared-manifest operation lost its retained manifest capsule",
                ));
            }
            (_, Some(_)) => {
                return Err(invalid(
                    "operation format does not admit a prepared manifest capsule",
                ));
            }
            (_, None) => {}
        }
        self.validate_prepared_lifecycle_binding()?;
        Ok(())
    }

    fn validate_prepared_append_event(
        &self,
        event: &DisposableLifecycleEventV2,
    ) -> Result<(), DurableLifecycleStoreErrorV3> {
        if !self.records.is_empty() {
            return Ok(());
        }
        match (self.format, event, self.prepared_manifest.as_ref()) {
            (
                OperationFormatV3::RequiredPreparedManifestV3,
                DisposableLifecycleEventV2::OperationPreparedWithManifestV3 {
                    prepared_manifest,
                    ..
                },
                Some(retained),
            ) if prepared_manifest
                == &PreparedCollectorManifestBindingV3 {
                    birthtime_nanoseconds: retained.binding.birthtime_nsec,
                    birthtime_seconds: retained.binding.birthtime_sec,
                    dev: retained.binding.dev,
                    generation: retained.binding.generation,
                    inode: retained.binding.ino,
                    sha256: retained.digest.clone(),
                } =>
            {
                Ok(())
            }
            (OperationFormatV3::RequiredPreparedManifestV3, _, _) => Err(invalid(
                "prepared-manifest operation requires an exact sidecar-bound first lifecycle record",
            )),
            (_, DisposableLifecycleEventV2::OperationPreparedWithManifestV3 { .. }, _) => {
                Err(invalid(
                    "sidecar-bound prepared record requires the exact prepared-manifest operation format",
                ))
            }
            _ => Ok(()),
        }
    }

    fn validate_prepared_lifecycle_binding(&self) -> Result<(), DurableLifecycleStoreErrorV3> {
        if self.records.is_empty() {
            return Ok(());
        }
        let lifecycle = self
            .records
            .iter()
            .map(|record| record.bytes.clone())
            .collect::<Vec<_>>();
        let inspection = inspect_lifecycle_v2(&lifecycle)?;
        match (self.format, self.prepared_manifest.as_ref()) {
            (OperationFormatV3::RequiredPreparedManifestV3, Some(retained)) => {
                let expected = PreparedCollectorManifestBindingV3 {
                    birthtime_nanoseconds: retained.binding.birthtime_nsec,
                    birthtime_seconds: retained.binding.birthtime_sec,
                    dev: retained.binding.dev,
                    generation: retained.binding.generation,
                    inode: retained.binding.ino,
                    sha256: retained.digest.clone(),
                };
                if inspection.prepared_manifest.as_ref() != Some(&expected) {
                    return Err(invalid(
                        "lifecycle first record does not bind the exact retained prepared manifest",
                    ));
                }
            }
            (_, None) if inspection.prepared_manifest.is_none() => {}
            _ => {
                return Err(invalid(
                    "lifecycle prepared-manifest binding differs from the operation format",
                ));
            }
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

fn publish_prepared_manifest(
    directory: &File,
    bytes: &[u8],
    expected_uid: u32,
    expected_gid: u32,
    expected_dev: u64,
    hook: &mut impl FnMut(CreateCutpointV3) -> io::Result<()>,
) -> Result<PreparedManifestCapsuleV3, DurableLifecycleStoreErrorV3> {
    if bytes.is_empty() || bytes.len() > MAX_RECORD_BYTES {
        return Err(invalid(
            "prepared collector manifest byte length is outside the fixed bound",
        ));
    }
    require_absent(directory.as_raw_fd(), PREPARED_MANIFEST_TEMPORARY_NAME_V3)?;
    require_absent(directory.as_raw_fd(), PREPARED_MANIFEST_NAME_V3)?;
    let mut temporary = createat_file(
        directory.as_raw_fd(),
        PREPARED_MANIFEST_TEMPORARY_NAME_V3,
        0o400,
    )?;
    hook(CreateCutpointV3::PreparedManifestTemporaryCreated)?;
    temporary.write_all(bytes)?;
    hook(CreateCutpointV3::PreparedManifestBytesWritten)?;
    temporary.sync_all()?;
    hook(CreateCutpointV3::PreparedManifestFileSynced)?;
    let temporary_binding = validate_regular(
        &temporary,
        expected_uid,
        expected_gid,
        0o400,
        Some(expected_dev),
        Some(bytes.len()),
        "temporary prepared collector manifest",
    )?;
    if read_stable(&temporary, temporary_binding)? != bytes {
        return Err(invalid(
            "temporary prepared collector manifest differs after fsync",
        ));
    }
    rename_noreplace(
        directory.as_raw_fd(),
        PREPARED_MANIFEST_TEMPORARY_NAME_V3,
        directory.as_raw_fd(),
        PREPARED_MANIFEST_NAME_V3,
    )?;
    hook(CreateCutpointV3::PreparedManifestRenamed)?;
    directory.sync_all()?;
    hook(CreateCutpointV3::PreparedManifestDirectorySynced)?;
    let file = openat_regular(directory.as_raw_fd(), PREPARED_MANIFEST_NAME_V3)?;
    hook(CreateCutpointV3::PreparedManifestFinalReopened)?;
    let binding = validate_regular(
        &file,
        expected_uid,
        expected_gid,
        0o400,
        Some(expected_dev),
        Some(bytes.len()),
        "final prepared collector manifest",
    )?;
    if !temporary_binding.stable_across_rename(binding)
        || named_binding(directory.as_raw_fd(), PREPARED_MANIFEST_NAME_V3)? != binding
        || read_stable(&file, binding)? != bytes
    {
        return Err(invalid(
            "final prepared collector manifest differs from its fsynced temporary inode",
        ));
    }
    directory.sync_all()?;
    hook(CreateCutpointV3::PreparedManifestFinalRevalidated)?;
    Ok(PreparedManifestCapsuleV3 {
        binding,
        bytes: bytes.to_vec(),
        digest: sha256(bytes),
        file,
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
        birthtime_nsec: stat.st_birthtime_nsec,
        birthtime_sec: stat.st_birthtime,
        ctime_nsec: stat.st_ctime_nsec,
        ctime_sec: stat.st_ctime,
        dev: stat.st_dev as u64,
        flags: stat.st_flags,
        generation: stat.st_gen,
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

fn classify_operation_roster(
    names: &[String],
) -> Result<(OperationFormatV3, Vec<String>), DurableLifecycleStoreErrorV3> {
    let mut records = Vec::new();
    let mut issue_roots = 0usize;
    let mut prepared_manifests = 0usize;
    for name in names {
        if name == "effect-issues-v3" {
            issue_roots += 1;
        } else if name == PREPARED_MANIFEST_NAME_V3 {
            prepared_manifests += 1;
        } else if name.contains("effect-issues-v3")
            || name.contains("prepared-collector-manifest-v3")
            || !name.ends_with(".json")
        {
            return Err(invalid(
                "operation roster contains a missing, temporary, aliased, or unknown V3 entry",
            ));
        } else {
            records.push(name.clone());
        }
    }
    if issue_roots > 1 || prepared_manifests > 1 {
        return Err(invalid(
            "operation roster contains duplicate mandatory V3 entries",
        ));
    }
    records.sort();
    let format = match (issue_roots, prepared_manifests) {
        (1, 1) => OperationFormatV3::RequiredPreparedManifestV3,
        (1, 0) => OperationFormatV3::RequiredEffectIssuesV3,
        (0, 0) => OperationFormatV3::LegacyV2,
        (0, 1) => {
            return Err(invalid(
                "prepared collector manifest exists without the mandatory V3 issue root",
            ));
        }
        _ => unreachable!("duplicate mandatory entries were rejected"),
    };
    Ok((format, records))
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
