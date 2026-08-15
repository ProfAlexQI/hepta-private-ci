//! Descriptor-bound, no-authority control-plane inspection for disposable
//! privileged macOS qualification operations.
//!
//! Production code can only open the fixed root.  It never creates that root,
//! never invokes an effect, and never turns a clean roster into authority.

use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::mac_apfs_barrier_fixture::ApfsFixtureResultV1;
use crate::mac_apfs_barrier_fixture::AttachmentObligationEventV1;
use crate::mac_apfs_barrier_fixture::AttachmentObligationRecordV1;
use crate::mac_apfs_barrier_fixture::AttachmentObligationVerificationV1;
use crate::mac_apfs_barrier_fixture::ObligationDispositionV1;
use crate::mac_apfs_barrier_fixture::replay_attachment_obligation_records;
use crate::mac_apfs_barrier_fixture::verify_disposable_fixture_tree;
use crate::mac_disposable_lifecycle::DisposableAuthorityV2;
use crate::mac_disposable_lifecycle::FreshAbsenceObservationV2;
use crate::mac_disposable_lifecycle::LifecycleDispatchV2;
use crate::mac_disposable_lifecycle::LifecycleDispositionV2;
use crate::mac_disposable_lifecycle::LifecycleInspectionV2;
use crate::mac_disposable_lifecycle::dispatch_lifecycle_records;
use crate::mac_disposable_lifecycle::fresh_absence_sha256;
use crate::mac_disposable_lifecycle::validate_fresh_absence_shape;
use crate::mac_disposable_lifecycle_store::CensusBoundDurableLifecycleStoreV3;
use crate::mac_disposable_lifecycle_store::DurableLifecycleStoreErrorV3;
use crate::mac_disposable_lifecycle_store::ExistingCensusStoreWiringV3;
use crate::mac_disposable_lifecycle_store::FreshCensusStoreWiringV3;
use crate::mac_disposable_lifecycle_store::ReconciliationOperationStoreV3;
use crate::mac_disposable_lifecycle_store::RetainedLifecycleRecordAppendV3;
use crate::mac_disposable_reconciliation_collector::MountBindingV3;
use crate::mac_disposable_reconciliation_collector::MountingV3;
use crate::mac_disposable_reconciliation_collector::SealedMountDeltaAdvanceV3;
use crate::mac_disposable_reconciliation_collector::SealedMountDeltaPlanV3;
use crate::mac_disposable_reconciliation_collector::UnmountingV3;
use crate::mac_iomedia_identity::current_boot_session_uuid;
use crate::mac_privileged_broker::BarrierJournal;
use crate::mac_privileged_broker::BarrierPhaseV1;
use crate::mac_privileged_broker::NamespacePolicy;
use crate::mac_privileged_broker::verify_sealed_publication;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::marker::PhantomData;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::RawFd;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use thiserror::Error;

pub const PRIVILEGED_DISPOSABLE_CONTROL_ROOT_V2: &str =
    "/Volumes/T5/.hepta-privileged-disposable-v2";
pub const MAX_CONTROL_OPERATIONS_V2: usize = 128;
pub const MAX_OPERATION_RECORDS_V2: usize = 256;
const MAX_RECORD_BYTES: usize = 1024 * 1024;
const MAX_TOTAL_RECORD_BYTES: usize = 64 * 1024 * 1024;
const LOCK_NAME: &str = "global.lock";
const OPERATIONS_NAME: &str = "operations";
const PUBLICATION_NAME: &str = "publication";
const OPERATION_PREFIX: &str = "operation-";
const EFFECT_ISSUE_ROOT_V3: &str = "effect-issues-v3";
const MAX_EFFECT_ISSUES_V3: usize = 256;
const HISTORICAL_ROOT_PREFIX: &str = ".hepta-privileged-qualification-v1-";
const HISTORICAL_OBLIGATION_PREFIX: &str = "attachment-obligation-";
const LEGACY_CLOSURE_PREFIX: &str = "legacy-closure-";
const EXPECTED_T5_UUID: [u8; 16] = [
    0xfb, 0x80, 0x4d, 0x1b, 0x24, 0xcb, 0x4d, 0x6e, 0xae, 0xa7, 0xa9, 0xe1, 0x80, 0x80, 0x77, 0x58,
];
const MAX_VOLUME_ENTRIES: usize = 4096;
const MAX_HISTORICAL_ROOT_ENTRIES: usize = 512;
const MAX_HISTORICAL_ROOTS: usize = 64;
const MAX_RETAINED_FDS: usize = 2048;
const MAX_DESCRIPTOR_DEPTH: usize = 64;
const MAX_MOUNT_ENTRIES: usize = 4096;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrivilegedDisposablePolicyV2 {
    pub admission_authority: bool,
    pub authority: DisposableAuthorityV2,
    pub control_root: String,
    pub max_operation_records: usize,
    pub max_operations: usize,
    pub schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrivilegedDisposableExecutionV2 {
    pub admission_authority: bool,
    pub authority: DisposableAuthorityV2,
    pub blocking_operation_nonces: Vec<String>,
    pub closed_world_roster_verified: bool,
    pub completed_operation_nonces: Vec<String>,
    pub historical_closure_bindings: Vec<String>,
    pub historical_roots_scanned: usize,
    pub legacy_v1_verified_but_awaiting_v2_closure: Vec<String>,
    pub new_operation_precondition_satisfied: bool,
    pub operation_count: usize,
    pub schema: String,
    pub storage_precondition_satisfied: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyClosureAttestationV2 {
    pub authority: DisposableAuthorityV2,
    pub fresh_absence: FreshAbsenceObservationV2,
    pub fresh_absence_sha256: String,
    pub historical_boot_session_uuid: String,
    pub historical_operation_nonce: String,
    pub historical_root_ctime_nanoseconds: i64,
    pub historical_root_ctime_seconds: i64,
    pub historical_root_dev: u64,
    pub historical_root_inode: u64,
    pub historical_root_name: String,
    pub historical_terminal_record_sha256: String,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Debug, Error)]
pub enum PrivilegedDisposableControlErrorV2 {
    #[error("invalid privileged-disposable control root: {0}")]
    Invalid(String),
    #[error("privileged-disposable process lock is already held")]
    LockBusy,
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Identity {
    ctime_nsec: i64,
    ctime_sec: i64,
    dev: u64,
    flags: u32,
    gid: u32,
    ino: u64,
    mode: u16,
    mtime_nsec: i64,
    mtime_sec: i64,
    nlink: u64,
    size: i64,
    uid: u32,
}

impl Identity {
    fn same_directory_across_record_append(self, after: Self) -> bool {
        self.dev == after.dev
            && self.flags == after.flags
            && self.gid == after.gid
            && self.ino == after.ino
            && self.mode == after.mode
            && self.nlink.checked_add(1) == Some(after.nlink)
            && self.uid == after.uid
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FilesystemBinding {
    dev: u64,
    filesystem_id: [i32; 2],
    filesystem_type: String,
    mount_flags: u64,
    mount_from: String,
    mount_on: String,
}

type MountBinding = MountBindingV3;

#[repr(C)]
struct VolumeUuidBuffer {
    length: u32,
    uuid: [u8; 16],
}

struct RecordCapsule {
    bytes: Vec<u8>,
    file: File,
    identity: Identity,
    name: String,
}

struct OperationCapsule {
    directory: File,
    effect_issues: Option<EffectIssueRootCapsuleV3>,
    identity: Identity,
    name: String,
    record_names: Vec<String>,
    records: Vec<RecordCapsule>,
    roster: Vec<String>,
}

struct EffectIssueRootCapsuleV3 {
    directory: File,
    identity: Identity,
    issues: Vec<RecordCapsule>,
    name: String,
    roster: Vec<String>,
}

struct HistoricalRootCapsule {
    barrier: File,
    barrier_nodes: Vec<HeldNode>,
    barrier_identity: Identity,
    barrier_roster: Vec<String>,
    directory: File,
    identity: Identity,
    name: String,
    obligations: Vec<OperationCapsule>,
    other_nodes: Vec<HeldNode>,
    publication: File,
    publication_identity: Identity,
    publication_roster: Vec<String>,
    roster: Vec<String>,
}

struct HeldNode {
    bytes: Option<Vec<u8>>,
    children: Vec<HeldNode>,
    file: File,
    identity: Identity,
    name: String,
    roster: Vec<String>,
}

struct LegacyClosureExpectation {
    attestation_name: String,
    backing_identity_sha256: String,
    baseline_inventory_sha256: String,
    boot_session_uuid: String,
    mountpoint_underlying_sha256: String,
    operation_nonce: String,
    root_identity: Identity,
    root_name: String,
    semantics_replayed: bool,
    terminal_record_sha256: String,
}

struct HistoricalScan {
    awaiting_closure: Vec<String>,
    blockers: Vec<String>,
    closure_expectations: Vec<LegacyClosureExpectation>,
    roots: Vec<HistoricalRootCapsule>,
    volume_roster: Vec<String>,
}

#[derive(Debug)]
struct RetainedFdBudget {
    retained: usize,
    remaining: usize,
}

impl RetainedFdBudget {
    fn after_existing(
        maximum: usize,
        existing: usize,
    ) -> Result<Self, PrivilegedDisposableControlErrorV2> {
        let remaining = maximum
            .checked_sub(existing)
            .ok_or_else(|| invalid("retained descriptor budget exceeded by fixed descriptors"))?;
        Ok(Self {
            retained: existing,
            remaining,
        })
    }

    /// Reserve before every open which will remain alive in the returned
    /// assessment.  A failed open terminates the census, so reservations never
    /// need to be refunded or reused by another fixture.
    fn reserve(&mut self, label: &str) -> Result<(), PrivilegedDisposableControlErrorV2> {
        self.remaining = self.remaining.checked_sub(1).ok_or_else(|| {
            invalid(format!(
                "retained descriptor budget exhausted before opening {label}"
            ))
        })?;
        self.retained = self
            .retained
            .checked_add(1)
            .ok_or_else(|| invalid("retained descriptor counter overflowed"))?;
        Ok(())
    }

    fn retained(&self) -> usize {
        self.retained
    }
}

#[derive(Default)]
struct CensusRegistry {
    identities: BTreeMap<(u64, u64), String>,
}

impl CensusRegistry {
    fn insert(
        &mut self,
        label: &str,
        identity: Identity,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        if let Some(existing) = self
            .identities
            .insert((identity.dev, identity.ino), label.to_string())
        {
            return Err(invalid(format!(
                "closed-world census aliases {existing} and {label} to one inode"
            )));
        }
        Ok(())
    }
}

impl LegacyClosureExpectation {
    fn binding_key(&self) -> String {
        format!(
            "{}/{}:{}:{}:{}:{}:{}",
            self.root_name,
            self.operation_nonce,
            self.root_identity.dev,
            self.root_identity.ino,
            self.root_identity.ctime_sec,
            self.root_identity.ctime_nsec,
            self.terminal_record_sha256,
        )
    }
}

struct AncestorChain {
    filesystem_root: File,
    filesystem_root_identity: Identity,
    t5: File,
    t5_identity: Identity,
    volumes: File,
    volumes_identity: Identity,
}

/// Live descriptors are intentionally a different, non-serializable type
/// from the no-authority policy receipt.
pub struct LivePrivilegedDisposablePolicyV2 {
    assessment_claimed: AtomicBool,
    ancestors: Option<AncestorChain>,
    expected_gid: u32,
    expected_uid: u32,
    filesystem: FilesystemBinding,
    lock: File,
    lock_identity: Identity,
    operations: File,
    operations_identity: Identity,
    policy: PrivilegedDisposablePolicyV2,
    publication: File,
    publication_identity: Identity,
    root: File,
    root_identity: Identity,
    root_name: String,
    volume: File,
    volume_identity: Identity,
    volume_path: PathBuf,
}

/// A read-only assessment retains every operation and record descriptor until
/// it is dropped.  It is evidence, not an execution token.
pub struct LivePrivilegedDisposableExecutionV2<'a> {
    _boot_session_uuid: String,
    _exact_blocking_receipt: Vec<String>,
    _exact_completed_receipt: Vec<String>,
    _historical_roots: Vec<HistoricalRootCapsule>,
    _mounts: Vec<MountBinding>,
    _operation_blockers: Vec<String>,
    _operation_names: Vec<String>,
    _operations: Vec<OperationCapsule>,
    _policy: &'a LivePrivilegedDisposablePolicyV2,
    _protected_roots: Vec<PathBuf>,
    _publication_roster: Vec<String>,
    _publication_records: Vec<RecordCapsule>,
    _retained_fd_count: usize,
    _retained_record_bytes: usize,
    receipt: PrivilegedDisposableExecutionV2,
    _volume_roster: Vec<String>,
}

/// Fresh admission may publish exactly one operation directory into the
/// otherwise exact retained roster.
pub(crate) struct FreshAdmissionV3 {
    admitted_operation_name: Option<String>,
}

/// Restart reconciliation is bound to one exact blocking operation while the
/// census continues to retain every other completed and blocking capsule.
pub(crate) struct BlockingOperationV3 {
    operation_identity: Identity,
    operation_name: String,
    operation_nonce: String,
}

/// A selected blocker may enter this state only through one durable terminal
/// append whose replay changes the exact selected operation to completed.
/// The state remains descriptor-retained and deliberately has no conversion
/// back into fresh admission.
pub(crate) struct CompletedOperationV3 {
    operation_identity: Identity,
    operation_name: String,
    operation_nonce: String,
    terminal_record_sha256: String,
}

/// The only mount state admitted by S1.  Later effect orchestration must move
/// this into an explicit pending-delta typestate rather than silently changing
/// the expected full mount table.
pub(crate) struct StableMountStateV3;

pub(crate) struct PendingMountDeltaV3 {
    command_sha256: String,
    expected_after: Vec<MountBinding>,
    target: MountBinding,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub(crate) struct PendingUnmountDeltaV3 {
    command_sha256: String,
    expected_after: Vec<MountBinding>,
    target: MountBinding,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

pub(crate) trait MountCensusStateV3 {
    fn validate_current(
        &self,
        before: &[MountBinding],
        current: &[MountBinding],
    ) -> Result<(), PrivilegedDisposableControlErrorV2>;
}

impl MountCensusStateV3 for StableMountStateV3 {
    fn validate_current(
        &self,
        before: &[MountBinding],
        current: &[MountBinding],
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        if current != before {
            return Err(invalid("stable exact mount census changed"));
        }
        Ok(())
    }
}

macro_rules! impl_pending_mount_state {
    ($state:ty, $label:literal) => {
        impl MountCensusStateV3 for $state {
            fn validate_current(
                &self,
                before: &[MountBinding],
                current: &[MountBinding],
            ) -> Result<(), PrivilegedDisposableControlErrorV2> {
                if current != before && current != self.expected_after {
                    return Err(invalid(concat!(
                        $label,
                        " observed neither exact before nor exact expected-after"
                    )));
                }
                Ok(())
            }
        }
    };
}

impl_pending_mount_state!(PendingMountDeltaV3, "pending mount delta");
impl_pending_mount_state!(PendingUnmountDeltaV3, "pending unmount delta");

struct ExactMountCensusV3<M> {
    expected_full_snapshot: Vec<MountBinding>,
    state: M,
}

/// Non-serializable closed-world census consumed by S2.  It retains every
/// descriptor from the S1 assessment and the process-global flock.  The
/// admission and mount state parameters cannot be reconstructed from a file or
/// from a serialized receipt.
pub(crate) struct RetainedControlCensusV3<'a, A, M> {
    assessment: LivePrivilegedDisposableExecutionV2<'a>,
    admission: A,
    exact_mounts: ExactMountCensusV3<M>,
    operations_identity: Identity,
    operations_roster: Vec<String>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// One-shot sink through which S2 transfers the exact freshly published
/// operation and mandatory issue-root descriptors back into the retained S1
/// census.  The sink never yields either descriptor to its caller.
pub(crate) struct FreshOperationAdmissionSinkV3<'c, 'a> {
    census: &'c mut RetainedControlCensusV3<'a, FreshAdmissionV3, StableMountStateV3>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

enum LifecycleRecordAppendTargetV3<'c, 'a> {
    Fresh(&'c mut RetainedControlCensusV3<'a, FreshAdmissionV3, StableMountStateV3>),
    Blocking(&'c mut RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3>),
    PendingMount(&'c mut RetainedControlCensusV3<'a, BlockingOperationV3, PendingMountDeltaV3>),
    PendingUnmount(&'c mut RetainedControlCensusV3<'a, BlockingOperationV3, PendingUnmountDeltaV3>),
}

/// One-shot S1 sink for one already-durable S2 lifecycle record descriptor.
/// `terminal` is sealed by the constructor selected from the census typestate,
/// never supplied by an external caller.
pub(crate) struct LifecycleRecordAppendSinkV3<'c, 'a> {
    target: LifecycleRecordAppendTargetV3<'c, 'a>,
    terminal: bool,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

/// One-shot S1 sink for the exact final V3 issue descriptor retained by S2.
pub(crate) struct EffectIssueAppendSinkV3<'c, 'a> {
    census: &'c mut RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3>,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl LivePrivilegedDisposablePolicyV2 {
    /// Open and retain the fixed control root and lock until process exit.
    pub fn open_fixed_for_process() -> Result<&'static Self, PrivilegedDisposableControlErrorV2> {
        let opened = open_fixed_root()?;
        verify_t5(&opened.ancestors.t5)?;
        let filesystem = filesystem_binding(&opened.ancestors.t5)?;
        let live = Self::open_from_root(
            opened.root,
            opened.ancestors.t5.try_clone()?,
            Some(opened.ancestors),
            filesystem,
            /*expected_uid*/ 0,
            /*expected_gid*/ 0,
            PRIVILEGED_DISPOSABLE_CONTROL_ROOT_V2,
            ".hepta-privileged-disposable-v2",
        )?;
        Ok(Box::leak(Box::new(live)))
    }

    pub fn policy(&self) -> &PrivilegedDisposablePolicyV2 {
        &self.policy
    }

    pub fn assess_read_only(
        &self,
    ) -> Result<LivePrivilegedDisposableExecutionV2<'_>, PrivilegedDisposableControlErrorV2> {
        self.assess_read_only_inner(MAX_RETAINED_FDS, || Ok(()))
    }

    #[cfg(test)]
    fn assess_read_only_with_hook<F>(
        &self,
        before_final_revalidation: F,
    ) -> Result<LivePrivilegedDisposableExecutionV2<'_>, PrivilegedDisposableControlErrorV2>
    where
        F: FnOnce() -> Result<(), PrivilegedDisposableControlErrorV2>,
    {
        self.assess_read_only_inner(MAX_RETAINED_FDS, before_final_revalidation)
    }

    #[cfg(test)]
    fn assess_read_only_with_fd_limit(
        &self,
        retained_fd_limit: usize,
    ) -> Result<LivePrivilegedDisposableExecutionV2<'_>, PrivilegedDisposableControlErrorV2> {
        self.assess_read_only_inner(retained_fd_limit, || Ok(()))
    }

    fn assess_read_only_inner<F>(
        &self,
        retained_fd_limit: usize,
        before_final_revalidation: F,
    ) -> Result<LivePrivilegedDisposableExecutionV2<'_>, PrivilegedDisposableControlErrorV2>
    where
        F: FnOnce() -> Result<(), PrivilegedDisposableControlErrorV2>,
    {
        if self
            .assessment_claimed
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(invalid(
                "this retained policy/flock already consumed its one S1 assessment",
            ));
        }
        self.revalidate_control_root()?;
        let boot_session_uuid = current_boot_session_uuid()
            .map_err(|error| invalid(format!("cannot bind S1 census to current boot: {error}")))?;
        let mounts_before = mount_table_snapshot()?;
        let operation_names =
            list_directory(self.operations.as_raw_fd(), MAX_CONTROL_OPERATIONS_V2)?;
        let mut capsules = Vec::with_capacity(operation_names.len());
        let mut blocking = Vec::new();
        let mut completed = Vec::new();
        let mut total_bytes = 0usize;
        let fixed_descriptors = 5usize + self.ancestors.as_ref().map_or(0, |_| 3);
        let mut retained_fds =
            RetainedFdBudget::after_existing(retained_fd_limit, fixed_descriptors)?;
        for name in &operation_names {
            let nonce = operation_nonce(name)?;
            let capsule = self.open_operation(name, &mut total_bytes, &mut retained_fds)?;
            let bytes = capsule
                .records
                .iter()
                .map(|record| record.bytes.clone())
                .collect::<Vec<_>>();
            let dispatch = dispatch_lifecycle_records(&bytes)
                .map_err(|error| invalid(format!("{name} lifecycle is invalid: {error}")))?;
            match dispatch {
                LifecycleDispatchV2::V2(inspection) => {
                    if inspection.operation_nonce != nonce {
                        return Err(invalid(
                            "operation directory nonce differs from lifecycle nonce",
                        ));
                    }
                    if inspection.blocks_new_operations {
                        blocking.push(nonce.to_string());
                    } else {
                        completed.push(nonce.to_string());
                    }
                }
                LifecycleDispatchV2::HistoricalV1(_) => {
                    return Err(invalid(
                        "v2 operations roster contains a historical v1 journal",
                    ));
                }
            }
            capsules.push(capsule);
        }
        let operation_blockers = blocking.clone();
        let historical = self.scan_historical_roots(&mut total_bytes, &mut retained_fds)?;
        let (historical_closed, publication_records, publication_roster) = self
            .verify_legacy_closures(
                &historical.closure_expectations,
                &mut total_bytes,
                &mut retained_fds,
            )?;
        let awaiting = historical
            .awaiting_closure
            .iter()
            .filter(|binding| !historical_closed.contains(binding))
            .cloned()
            .collect::<Vec<_>>();
        blocking.extend(historical.blockers.iter().cloned());
        blocking.extend(awaiting.iter().cloned());
        before_final_revalidation()?;
        self.revalidate_control_root()?;
        if list_directory(self.operations.as_raw_fd(), MAX_CONTROL_OPERATIONS_V2)?
            != operation_names
        {
            return Err(invalid("operations roster changed during assessment"));
        }
        for capsule in &capsules {
            capsule.revalidate(
                self.operations.as_raw_fd(),
                self.expected_uid,
                self.expected_gid,
                &self.filesystem,
            )?;
        }
        for root in &historical.roots {
            root.revalidate(
                self.volume.as_raw_fd(),
                self.expected_uid,
                self.expected_gid,
                &self.filesystem,
            )?;
        }
        if list_directory(self.volume.as_raw_fd(), MAX_VOLUME_ENTRIES)? != historical.volume_roster
        {
            return Err(invalid("T5 root roster changed after historical census"));
        }
        if list_directory(self.publication.as_raw_fd(), MAX_CONTROL_OPERATIONS_V2)?
            != publication_roster
        {
            return Err(invalid(
                "closure publication roster changed during assessment",
            ));
        }
        for record in &publication_records {
            record.revalidate(
                self.publication.as_raw_fd(),
                self.expected_uid,
                self.expected_gid,
                &self.filesystem,
            )?;
        }
        let retained_fds_from_capsules = fixed_descriptors
            + capsules
                .iter()
                .map(|capsule| {
                    1 + capsule.records.len()
                        + capsule
                            .effect_issues
                            .as_ref()
                            .map_or(0, |root| 1 + root.issues.len())
                })
                .sum::<usize>()
            + historical
                .roots
                .iter()
                .map(HistoricalRootCapsule::descriptor_count)
                .sum::<usize>()
            + publication_records.len();
        if retained_fds.retained() != retained_fds_from_capsules {
            return Err(invalid(
                "incremental retained descriptor budget disagrees with final capsule census",
            ));
        }
        let mut aliases = CensusRegistry::default();
        aliases.insert("control root", self.root_identity)?;
        aliases.insert("control lock", self.lock_identity)?;
        aliases.insert("operations directory", self.operations_identity)?;
        aliases.insert("closure publication directory", self.publication_identity)?;
        for capsule in &capsules {
            capsule.register(&mut aliases, "operations")?;
        }
        for root in &historical.roots {
            root.register(&mut aliases)?;
        }
        for record in &publication_records {
            record.register(&mut aliases, "closure publication")?;
        }

        let protected_roots = std::iter::once(PathBuf::from(&self.policy.control_root))
            .chain(
                historical
                    .roots
                    .iter()
                    .map(|root| self.volume_path.join(&root.name)),
            )
            .collect::<Vec<_>>();
        reject_nested_mounts(&mounts_before, &protected_roots)?;
        let mounts_after = mount_table_snapshot()?;
        reject_nested_mounts(&mounts_after, &protected_roots)?;
        if mounts_after != mounts_before {
            return Err(invalid(
                "bounded mount table changed during storage assessment",
            ));
        }
        if current_boot_session_uuid()
            .map_err(|error| invalid(format!("cannot revalidate S1 boot binding: {error}")))?
            != boot_session_uuid
        {
            return Err(invalid("boot session changed during storage assessment"));
        }
        let new_operation_precondition_satisfied = blocking.is_empty() && awaiting.is_empty();
        let receipt = PrivilegedDisposableExecutionV2 {
            admission_authority: false,
            authority: DisposableAuthorityV2::none(),
            blocking_operation_nonces: blocking.clone(),
            closed_world_roster_verified: true,
            completed_operation_nonces: completed.clone(),
            historical_closure_bindings: historical_closed,
            historical_roots_scanned: historical.roots.len(),
            legacy_v1_verified_but_awaiting_v2_closure: awaiting,
            new_operation_precondition_satisfied,
            operation_count: operation_names.len(),
            schema: "hepta_mac_privileged_disposable_execution_v2".to_string(),
            storage_precondition_satisfied: true,
        };
        Ok(LivePrivilegedDisposableExecutionV2 {
            _boot_session_uuid: boot_session_uuid,
            _exact_blocking_receipt: blocking,
            _exact_completed_receipt: completed,
            _historical_roots: historical.roots,
            _mounts: mounts_after,
            _operation_blockers: operation_blockers,
            _operation_names: operation_names,
            _operations: capsules,
            _policy: self,
            _protected_roots: protected_roots,
            _publication_roster: publication_roster,
            _publication_records: publication_records,
            _retained_fd_count: retained_fds.retained(),
            _retained_record_bytes: total_bytes,
            receipt,
            _volume_roster: historical.volume_roster,
        })
    }

    fn open_from_root(
        root: File,
        volume: File,
        ancestors: Option<AncestorChain>,
        filesystem: FilesystemBinding,
        expected_uid: u32,
        expected_gid: u32,
        control_root: &str,
        root_name: &str,
    ) -> Result<Self, PrivilegedDisposableControlErrorV2> {
        let control_root_path = Path::new(control_root);
        let volume_path = control_root_path
            .parent()
            .ok_or_else(|| invalid("control root has no volume parent"))?
            .to_path_buf();
        if !control_root_path.is_absolute() || !volume_path.is_absolute() {
            return Err(invalid("control root and volume parent must be absolute"));
        }
        validate_filesystem(&volume, &filesystem, "control volume")?;
        let root_identity = validate_directory(
            &root,
            expected_uid,
            expected_gid,
            0o700,
            &filesystem,
            "control root",
        )?;
        require_names(
            &list_directory(root.as_raw_fd(), 3)?,
            &[LOCK_NAME, OPERATIONS_NAME, PUBLICATION_NAME],
            "control root",
        )?;
        let lock = openat_node(root.as_raw_fd(), LOCK_NAME, libc::O_RDWR)?;
        let lock_identity = validate_regular(
            &lock,
            expected_uid,
            expected_gid,
            0o600,
            Some(0),
            &filesystem,
            "control lock",
        )?;
        let lock_rc = unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
        if lock_rc != 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::EWOULDBLOCK) {
                return Err(PrivilegedDisposableControlErrorV2::LockBusy);
            }
            return Err(error.into());
        }
        let operations = openat_node(
            root.as_raw_fd(),
            OPERATIONS_NAME,
            libc::O_RDONLY | libc::O_DIRECTORY,
        )?;
        let operations_identity = validate_directory(
            &operations,
            expected_uid,
            expected_gid,
            0o700,
            &filesystem,
            "operations directory",
        )?;
        let publication = openat_node(
            root.as_raw_fd(),
            PUBLICATION_NAME,
            libc::O_RDONLY | libc::O_DIRECTORY,
        )?;
        let publication_identity = validate_directory(
            &publication,
            expected_uid,
            expected_gid,
            0o700,
            &filesystem,
            "publication directory",
        )?;
        let volume_identity = identity(&volume)?;
        Ok(Self {
            assessment_claimed: AtomicBool::new(false),
            ancestors,
            expected_gid,
            expected_uid,
            filesystem,
            lock,
            lock_identity,
            operations,
            operations_identity,
            policy: PrivilegedDisposablePolicyV2 {
                admission_authority: false,
                authority: DisposableAuthorityV2::none(),
                control_root: control_root.to_string(),
                max_operation_records: MAX_OPERATION_RECORDS_V2,
                max_operations: MAX_CONTROL_OPERATIONS_V2,
                schema: "hepta_mac_privileged_disposable_policy_v2".to_string(),
            },
            publication,
            publication_identity,
            root,
            root_identity,
            root_name: root_name.to_string(),
            volume,
            volume_identity,
            volume_path,
        })
    }

    fn open_operation(
        &self,
        name: &str,
        total_bytes: &mut usize,
        retained_fds: &mut RetainedFdBudget,
    ) -> Result<OperationCapsule, PrivilegedDisposableControlErrorV2> {
        self.open_record_set(self.operations.as_raw_fd(), name, total_bytes, retained_fds)
    }

    fn open_record_set(
        &self,
        parent_fd: RawFd,
        name: &str,
        total_bytes: &mut usize,
        retained_fds: &mut RetainedFdBudget,
    ) -> Result<OperationCapsule, PrivilegedDisposableControlErrorV2> {
        retained_fds.reserve("operation or obligation directory")?;
        let directory = openat_node(parent_fd, name, libc::O_RDONLY | libc::O_DIRECTORY)?;
        let identity = validate_directory(
            &directory,
            self.expected_uid,
            self.expected_gid,
            0o700,
            &self.filesystem,
            "operation directory",
        )?;
        let roster = list_directory(directory.as_raw_fd(), MAX_OPERATION_RECORDS_V2 + 1)?;
        let issue_like = roster
            .iter()
            .filter(|entry| entry.contains(EFFECT_ISSUE_ROOT_V3))
            .collect::<Vec<_>>();
        if issue_like.len() > 1
            || issue_like
                .first()
                .is_some_and(|entry| entry.as_str() != EFFECT_ISSUE_ROOT_V3)
        {
            return Err(invalid(
                "operation contains a duplicate, temporary, or aliased V3 issue root",
            ));
        }
        let record_names = roster
            .iter()
            .filter(|entry| entry.as_str() != EFFECT_ISSUE_ROOT_V3)
            .cloned()
            .collect::<Vec<_>>();
        if record_names.is_empty() {
            return Err(invalid("operation has no lifecycle records"));
        }
        let mut records = Vec::with_capacity(record_names.len());
        for (index, record_name) in record_names.iter().enumerate() {
            if record_name != &format!("{:08}.json", index + 1) {
                return Err(invalid(
                    "operation records contain an unknown, temporary, or gap name",
                ));
            }
            retained_fds.reserve("operation or obligation record")?;
            let file = openat_node(directory.as_raw_fd(), record_name, libc::O_RDONLY)?;
            let record_identity = validate_regular(
                &file,
                self.expected_uid,
                self.expected_gid,
                0o400,
                Some(MAX_RECORD_BYTES as i64),
                &self.filesystem,
                "lifecycle record",
            )?;
            let bytes = read_stable(&file, record_identity)?;
            *total_bytes = total_bytes
                .checked_add(bytes.len())
                .ok_or_else(|| invalid("lifecycle byte budget overflowed"))?;
            if *total_bytes > MAX_TOTAL_RECORD_BYTES {
                return Err(invalid("lifecycle byte budget exceeded"));
            }
            records.push(RecordCapsule {
                bytes,
                file,
                identity: record_identity,
                name: record_name.clone(),
            });
        }
        let effect_issues = if issue_like.is_empty() {
            None
        } else {
            Some(open_effect_issue_root(
                directory.as_raw_fd(),
                total_bytes,
                retained_fds,
                self.expected_uid,
                self.expected_gid,
                &self.filesystem,
            )?)
        };
        let capsule = OperationCapsule {
            directory,
            effect_issues,
            identity,
            name: name.to_string(),
            record_names,
            records,
            roster,
        };
        capsule.revalidate(
            parent_fd,
            self.expected_uid,
            self.expected_gid,
            &self.filesystem,
        )?;
        Ok(capsule)
    }

    fn scan_historical_roots(
        &self,
        total_bytes: &mut usize,
        retained_fds: &mut RetainedFdBudget,
    ) -> Result<HistoricalScan, PrivilegedDisposableControlErrorV2> {
        let current_boot_session_uuid = current_boot_session_uuid().map_err(|error| {
            invalid(format!(
                "current boot session UUID could not be captured for historical replay: {error}"
            ))
        })?;
        let volume_roster = list_directory(self.volume.as_raw_fd(), MAX_VOLUME_ENTRIES)?;
        let historical_names = volume_roster
            .iter()
            .filter(|name| name.starts_with(HISTORICAL_ROOT_PREFIX))
            .collect::<Vec<_>>();
        if historical_names.len() > MAX_HISTORICAL_ROOTS {
            return Err(invalid("historical root count exceeds the global cap"));
        }
        let mut roots = Vec::new();
        let mut blockers = Vec::new();
        let mut awaiting_closure = Vec::new();
        let mut closure_expectations = Vec::new();
        for root_name in historical_names {
            let root_nonce = root_name
                .strip_prefix(HISTORICAL_ROOT_PREFIX)
                .expect("filtered historical prefix");
            require_hex_nonce(root_nonce, "historical root nonce")?;
            retained_fds.reserve("historical qualification root")?;
            let directory = openat_node(
                self.volume.as_raw_fd(),
                root_name,
                libc::O_RDONLY | libc::O_DIRECTORY,
            )?;
            let root_identity = validate_directory(
                &directory,
                self.expected_uid,
                self.expected_gid,
                0o700,
                &self.filesystem,
                "historical qualification root",
            )?;
            let roster = list_directory(directory.as_raw_fd(), MAX_HISTORICAL_ROOT_ENTRIES)?;
            require_names(
                &roster,
                &["barrier-journal", "publication"],
                "historical qualification root",
            )?;
            retained_fds.reserve("historical publication directory")?;
            let publication = openat_node(
                directory.as_raw_fd(),
                "publication",
                libc::O_RDONLY | libc::O_DIRECTORY,
            )?;
            let publication_identity = validate_directory(
                &publication,
                self.expected_uid,
                self.expected_gid,
                0o700,
                &self.filesystem,
                "historical publication directory",
            )?;
            retained_fds.reserve("historical barrier journal")?;
            let barrier = openat_node(
                directory.as_raw_fd(),
                "barrier-journal",
                libc::O_RDONLY | libc::O_DIRECTORY,
            )?;
            let barrier_identity = validate_directory(
                &barrier,
                self.expected_uid,
                self.expected_gid,
                0o700,
                &self.filesystem,
                "historical barrier journal",
            )?;
            let (barrier_roster, barrier_nodes) =
                self.open_barrier_journal(&barrier, total_bytes, retained_fds)?;
            let publication_roster =
                list_directory(publication.as_raw_fd(), MAX_HISTORICAL_ROOT_ENTRIES)?;
            if publication_roster.is_empty() {
                return Err(invalid(format!("historical root {root_name} is empty")));
            }
            let mut obligation_names = Vec::new();
            let mut other_nodes = Vec::new();
            for name in &publication_roster {
                if let Some(nonce) = name.strip_prefix(HISTORICAL_OBLIGATION_PREFIX) {
                    require_hex_nonce(nonce, "historical obligation nonce")?;
                    if nonce != root_nonce {
                        return Err(invalid(
                            "historical obligation nonce differs from root nonce",
                        ));
                    }
                    obligation_names.push(name.clone());
                } else if let Some(nonce) = name.strip_prefix("apfs-fixture-") {
                    require_hex_nonce(nonce, "historical fixture nonce")?;
                    if nonce != root_nonce {
                        return Err(invalid("historical fixture nonce differs from root nonce"));
                    }
                    other_nodes.push(self.open_sealed_fixture(
                        publication.as_raw_fd(),
                        name,
                        self.expected_uid,
                        self.expected_gid,
                        retained_fds,
                    )?);
                } else if let Some(nonce) = historical_publication_record_nonce(name) {
                    if nonce != root_nonce {
                        return Err(invalid(
                            "historical publication receipt nonce differs from root nonce",
                        ));
                    }
                    retained_fds.reserve("historical publication receipt")?;
                    let file = openat_node(publication.as_raw_fd(), name, libc::O_RDONLY)?;
                    let identity = validate_regular(
                        &file,
                        self.expected_uid,
                        self.expected_gid,
                        0o400,
                        Some(MAX_RECORD_BYTES as i64),
                        &self.filesystem,
                        "historical publication receipt",
                    )?;
                    let bytes = read_stable(&file, identity)?;
                    *total_bytes = total_bytes
                        .checked_add(bytes.len())
                        .ok_or_else(|| invalid("historical byte budget overflowed"))?;
                    if *total_bytes > MAX_TOTAL_RECORD_BYTES {
                        return Err(invalid("historical byte budget exceeded"));
                    }
                    other_nodes.push(HeldNode {
                        bytes: Some(bytes),
                        children: Vec::new(),
                        file,
                        identity,
                        name: name.clone(),
                        roster: Vec::new(),
                    });
                } else {
                    return Err(invalid(format!(
                        "historical root {root_name} contains unknown or temporary entry {name}"
                    )));
                }
            }
            if obligation_names != [format!("{HISTORICAL_OBLIGATION_PREFIX}{root_nonce}")] {
                return Err(invalid(format!(
                    "historical root {root_name} must contain exactly its own obligation"
                )));
            }
            let mut obligations = Vec::new();
            let mut obligation_verification = None;
            let mut root_expectation = None;
            for obligation_name in obligation_names {
                let operation_nonce = obligation_name
                    .strip_prefix(HISTORICAL_OBLIGATION_PREFIX)
                    .expect("validated obligation prefix");
                let capsule = self.open_record_set(
                    publication.as_raw_fd(),
                    &obligation_name,
                    total_bytes,
                    retained_fds,
                )?;
                let decoded = capsule
                    .records
                    .iter()
                    .map(|record| {
                        serde_json::from_slice::<AttachmentObligationRecordV1>(&record.bytes)
                            .map(|parsed| (parsed, record.bytes.clone()))
                            .map_err(|error| {
                                invalid(format!("historical v1 JSON is invalid: {error}"))
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let verification =
                    replay_attachment_obligation_records(&decoded, &current_boot_session_uuid)
                        .map_err(|error| {
                            invalid(format!("frozen historical v1 replay failed: {error}"))
                        })?;
                if verification.operation_nonce != operation_nonce {
                    return Err(invalid("historical directory and record nonces differ"));
                }
                obligation_verification = Some(verification.clone());
                match verification.disposition {
                    ObligationDispositionV1::Active
                    | ObligationDispositionV1::ReconcileRequired
                    | ObligationDispositionV1::Quarantined => {
                        blockers.push(format!("{root_name}/{operation_nonce}"));
                    }
                    ObligationDispositionV1::Reconciled
                        if historical_obligation_closure_eligible(&verification) =>
                    {
                        let AttachmentObligationEventV1::Prepared {
                            image_backing,
                            mountpoint_underlying,
                            pre_attach_inventory,
                            ..
                        } = &decoded[0].0.event
                        else {
                            return Err(invalid("frozen v1 replay did not begin with Prepared"));
                        };
                        let attestation_name =
                            format!("{LEGACY_CLOSURE_PREFIX}{root_nonce}-{operation_nonce}.json");
                        let expectation = LegacyClosureExpectation {
                            attestation_name,
                            backing_identity_sha256: typed_sha256(image_backing)?,
                            baseline_inventory_sha256: typed_sha256(pre_attach_inventory)?,
                            boot_session_uuid: decoded[0].0.boot_session_uuid.clone(),
                            mountpoint_underlying_sha256: typed_sha256(mountpoint_underlying)?,
                            operation_nonce: operation_nonce.to_string(),
                            root_identity,
                            root_name: root_name.clone(),
                            semantics_replayed: false,
                            terminal_record_sha256: verification.terminal_record_sha256,
                        };
                        root_expectation = Some(expectation);
                    }
                    ObligationDispositionV1::Reconciled => {
                        blockers.push(
                            historical_obligation_semantic_blocker(root_name, &verification)
                                .expect("ineligible reconciled obligation has a blocker"),
                        );
                    }
                }
                obligations.push(capsule);
            }
            let semantic_blockers = self.verify_historical_v1_semantics(
                root_name,
                root_nonce,
                &publication_roster,
                &other_nodes,
                obligation_verification
                    .as_ref()
                    .expect("historical root has exactly one replayed obligation"),
            );
            let semantics_replayed = semantic_blockers.is_empty();
            blockers.extend(semantic_blockers);
            if let Some(mut expectation) = root_expectation {
                expectation.semantics_replayed = semantics_replayed;
                awaiting_closure.push(expectation.binding_key());
                closure_expectations.push(expectation);
            }
            roots.push(HistoricalRootCapsule {
                barrier,
                barrier_nodes,
                barrier_identity,
                barrier_roster,
                directory,
                identity: root_identity,
                name: root_name.clone(),
                obligations,
                other_nodes,
                publication,
                publication_identity,
                publication_roster,
                roster,
            });
        }
        if list_directory(self.volume.as_raw_fd(), MAX_VOLUME_ENTRIES)? != volume_roster {
            return Err(invalid("T5 root roster changed during historical census"));
        }
        Ok(HistoricalScan {
            awaiting_closure,
            blockers,
            closure_expectations,
            roots,
            volume_roster,
        })
    }

    fn open_barrier_journal(
        &self,
        barrier: &File,
        total_bytes: &mut usize,
        retained_fds: &mut RetainedFdBudget,
    ) -> Result<(Vec<String>, Vec<HeldNode>), PrivilegedDisposableControlErrorV2> {
        let roster = list_directory(barrier.as_raw_fd(), MAX_HISTORICAL_ROOT_ENTRIES)?;
        let mut nodes = Vec::with_capacity(roster.len());
        let mut sequence = 1usize;
        for name in &roster {
            if name == "recovery-terminal.json" {
                continue;
            }
            if name != &format!("{sequence:020}.json") {
                return Err(invalid(
                    "historical barrier journal contains a gap, temporary, or unknown entry",
                ));
            }
            sequence = sequence
                .checked_add(1)
                .ok_or_else(|| invalid("historical barrier sequence overflowed"))?;
        }
        if sequence == 1 {
            return Err(invalid("historical barrier journal has no durable records"));
        }
        if roster
            .iter()
            .filter(|name| name.as_str() == "recovery-terminal.json")
            .count()
            > 1
        {
            return Err(invalid(
                "historical barrier journal has duplicate recovery terminals",
            ));
        }
        for name in &roster {
            retained_fds.reserve("historical barrier record")?;
            let file = openat_node(barrier.as_raw_fd(), name, libc::O_RDONLY)?;
            let node_identity = validate_regular(
                &file,
                self.expected_uid,
                self.expected_gid,
                0o400,
                Some(MAX_RECORD_BYTES as i64),
                &self.filesystem,
                "historical barrier record",
            )?;
            let bytes = read_stable(&file, node_identity)?;
            *total_bytes = total_bytes
                .checked_add(bytes.len())
                .ok_or_else(|| invalid("historical barrier byte budget overflowed"))?;
            if *total_bytes > MAX_TOTAL_RECORD_BYTES {
                return Err(invalid("historical barrier byte budget exceeded"));
            }
            nodes.push(HeldNode {
                bytes: Some(bytes),
                children: Vec::new(),
                file,
                identity: node_identity,
                name: name.clone(),
                roster: Vec::new(),
            });
        }
        Ok((roster, nodes))
    }

    fn open_sealed_fixture(
        &self,
        parent_fd: RawFd,
        name: &str,
        expected_uid: u32,
        expected_gid: u32,
        retained_fds: &mut RetainedFdBudget,
    ) -> Result<HeldNode, PrivilegedDisposableControlErrorV2> {
        open_sealed_fixture_node(
            parent_fd,
            name,
            ".",
            0,
            retained_fds,
            expected_uid,
            expected_gid,
            &self.filesystem,
        )
    }

    fn verify_historical_v1_semantics(
        &self,
        root_name: &str,
        root_nonce: &str,
        publication_roster: &[String],
        other_nodes: &[HeldNode],
        obligation: &AttachmentObligationVerificationV1,
    ) -> Vec<String> {
        let mut blockers = Vec::new();
        let root_path = self.volume_path.join(root_name);
        let publication_path = root_path.join("publication");
        let fixture_name = format!("apfs-fixture-{root_nonce}");
        let fixture_path = publication_path.join(&fixture_name);
        let semantic_policy = if self.expected_uid == 0 && self.expected_gid == 0 {
            NamespacePolicy::live()
        } else {
            NamespacePolicy::mechanism_only_current_user()
        };
        let policy = match semantic_policy {
            Ok(policy) => Some(policy),
            Err(error) => {
                blockers.push(format!(
                    "{root_name}:historical_v1_policy_semantics_unproved:{error}"
                ));
                None
            }
        };

        if let Some(policy) = policy.as_ref() {
            match BarrierJournal::open(&root_path.join("barrier-journal"), policy.clone())
                .and_then(|journal| journal.verify())
            {
                Ok(verification)
                    if !verification.admission_release_authority
                        && !verification.live_authority
                        && verification.current_phase == BarrierPhaseV1::Released
                        && verification.epoch_nonce == root_nonce
                        && verification.recovery_disposition == "terminal_released" => {}
                Ok(_) => blockers.push(format!(
                    "{root_name}:historical_v1_barrier_not_exact_terminal_released"
                )),
                Err(error) => blockers.push(format!(
                    "{root_name}:historical_v1_barrier_semantics_unproved:{error}"
                )),
            }
        }

        let mut expected_publication_records = [
            format!("hepta-operation-{root_nonce}.prepared.json"),
            format!("hepta-operation-{root_nonce}.mechanism-receipt.json"),
            format!("hepta-operation-{root_nonce}.terminal-receipt.json"),
        ];
        expected_publication_records.sort();
        let publication_records = publication_roster
            .iter()
            .filter(|name| historical_publication_record_nonce(name).is_some())
            .cloned()
            .collect::<Vec<_>>();
        if publication_records != expected_publication_records {
            blockers.push(format!(
                "{root_name}:historical_v1_publication_exact_three_record_roster_unproved"
            ));
        } else if let Some(policy) = policy.as_ref() {
            match verify_sealed_publication(&publication_path, root_nonce, policy) {
                Ok(sealed)
                    if !sealed.publication_receipt.authority_granted
                        && sealed.publication_receipt.final_name == fixture_name
                        && !sealed.qualification_receipt.live_authority
                        && !sealed.qualification_receipt.aggregate_authority
                        && !sealed.qualification_receipt.cutover_authority
                        && !sealed.qualification_receipt.deletion_authority
                        && !sealed.qualification_receipt.production_authority
                        && !sealed.qualification_receipt.refs_authority
                        && !sealed.qualification_receipt.remote_authority
                        && sealed.qualification_receipt.operation_nonce == root_nonce => {}
                Ok(_) => blockers.push(format!(
                    "{root_name}:historical_v1_publication_chain_not_exact_no_authority"
                )),
                Err(error) => blockers.push(format!(
                    "{root_name}:historical_v1_publication_semantics_unproved:{error}"
                )),
            }
        }

        let matching_fixtures = other_nodes
            .iter()
            .filter(|node| node.name == fixture_name)
            .collect::<Vec<_>>();
        if matching_fixtures.len() != 1 {
            blockers.push(format!(
                "{root_name}:historical_v1_fixture_exact_singleton_unproved"
            ));
        } else {
            let result = matching_fixtures[0]
                .child_named("RESULT.json")
                .ok_or_else(|| invalid("sealed fixture has no held RESULT.json"))
                .and_then(|node| node.read_regular_bytes());
            match result {
                Ok(bytes) => {
                    let decoded = serde_json::from_slice::<ApfsFixtureResultV1>(&bytes)
                        .map_err(|error| invalid(format!("fixture RESULT JSON failed: {error}")))
                        .and_then(|decoded| {
                            if canonical_json(&decoded).map_err(|error| {
                                invalid(format!("fixture RESULT canonicalization failed: {error}"))
                            })? != bytes
                            {
                                return Err(invalid("fixture RESULT is not canonical JSON"));
                            }
                            Ok(decoded)
                        });
                    match decoded {
                        Ok(result) => {
                            let result_sha256 = crate::durable::sha256(&bytes);
                            match verify_disposable_fixture_tree(&fixture_path, &result_sha256) {
                                Ok(verification)
                                    if !verification.authority_granted
                                        && verification.operation_nonce == root_nonce
                                        && verification.operation_nonce == obligation.operation_nonce
                                        && verification.boot_session_uuid
                                            == obligation.boot_session_uuid
                                        && result.operation_nonce == root_nonce
                                        && result.attachment_obligation_terminal_sha256
                                            == obligation.terminal_record_sha256 => {}
                                Ok(_) => blockers.push(format!(
                                    "{root_name}:historical_v1_fixture_nonce_boot_or_terminal_binding_failed"
                                )),
                                Err(error) => blockers.push(format!(
                                    "{root_name}:historical_v1_fixture_semantics_unproved:{error}"
                                )),
                            }
                        }
                        Err(error) => blockers.push(format!(
                            "{root_name}:historical_v1_fixture_result_semantics_unproved:{error}"
                        )),
                    }
                }
                Err(error) => blockers.push(format!(
                    "{root_name}:historical_v1_fixture_result_semantics_unproved:{error}"
                )),
            }
        }
        blockers
    }

    fn verify_legacy_closures(
        &self,
        expectations: &[LegacyClosureExpectation],
        total_bytes: &mut usize,
        retained_fds: &mut RetainedFdBudget,
    ) -> Result<(Vec<String>, Vec<RecordCapsule>, Vec<String>), PrivilegedDisposableControlErrorV2>
    {
        let publication_roster =
            list_directory(self.publication.as_raw_fd(), MAX_CONTROL_OPERATIONS_V2)?;
        if publication_roster.iter().any(|name| {
            !expectations
                .iter()
                .any(|expectation| &expectation.attestation_name == name)
        }) {
            return Err(invalid(
                "publication is not the exact closed-world legacy-closure roster",
            ));
        }
        let mut closed = Vec::new();
        let mut records = Vec::new();
        for name in &publication_roster {
            let expectation = expectations
                .iter()
                .find(|expectation| &expectation.attestation_name == name)
                .expect("publication roster was checked above");
            retained_fds.reserve("legacy closure record")?;
            let file = openat_node(
                self.publication.as_raw_fd(),
                &expectation.attestation_name,
                libc::O_RDONLY,
            )?;
            let record_identity = validate_regular(
                &file,
                self.expected_uid,
                self.expected_gid,
                0o400,
                Some(MAX_RECORD_BYTES as i64),
                &self.filesystem,
                "legacy closure attestation",
            )?;
            let bytes = read_stable(&file, record_identity)?;
            *total_bytes = total_bytes
                .checked_add(bytes.len())
                .ok_or_else(|| invalid("closure byte budget overflowed"))?;
            if *total_bytes > MAX_TOTAL_RECORD_BYTES {
                return Err(invalid("closure byte budget exceeded"));
            }
            let attestation: LegacyClosureAttestationV2 = serde_json::from_slice(&bytes)
                .map_err(|error| invalid(format!("legacy closure JSON is invalid: {error}")))?;
            if canonical_json(&attestation)
                .map_err(|error| invalid(format!("legacy closure JSON failed: {error}")))?
                != bytes
            {
                return Err(invalid("legacy closure attestation is not canonical JSON"));
            }
            validate_fresh_absence_shape(&attestation.fresh_absence)
                .map_err(|error| invalid(format!("legacy fresh absence is invalid: {error}")))?;
            if attestation.schema != "hepta_mac_legacy_closure_attestation_v2"
                || attestation.schema_version != 2
                || attestation.authority.any()
                || attestation.historical_root_name != expectation.root_name
                || attestation.historical_operation_nonce != expectation.operation_nonce
                || attestation.historical_boot_session_uuid != expectation.boot_session_uuid
                || attestation.historical_terminal_record_sha256
                    != expectation.terminal_record_sha256
                || attestation.historical_root_dev != expectation.root_identity.dev
                || attestation.historical_root_inode != expectation.root_identity.ino
                || attestation.historical_root_ctime_seconds != expectation.root_identity.ctime_sec
                || attestation.historical_root_ctime_nanoseconds
                    != expectation.root_identity.ctime_nsec
                || attestation.fresh_absence.operation_nonce != expectation.operation_nonce
                || attestation.fresh_absence.baseline_inventory_sha256
                    != expectation.baseline_inventory_sha256
                || attestation.fresh_absence.backing_identity_sha256
                    != expectation.backing_identity_sha256
                || attestation.fresh_absence.mountpoint_underlying_sha256
                    != expectation.mountpoint_underlying_sha256
                || fresh_absence_sha256(&attestation.fresh_absence)
                    .map_err(|error| invalid(format!("legacy absence digest failed: {error}")))?
                    != attestation.fresh_absence_sha256
            {
                return Err(invalid(
                    "legacy closure does not exactly bind v1 terminal and independent absence",
                ));
            }
            if named_identity(self.publication.as_raw_fd(), &expectation.attestation_name)?
                != record_identity
                || read_stable(&file, record_identity)? != bytes
            {
                return Err(invalid("legacy closure changed during descriptor replay"));
            }
            if expectation.semantics_replayed {
                closed.push(expectation.binding_key());
            }
            records.push(RecordCapsule {
                bytes,
                file,
                identity: record_identity,
                name: expectation.attestation_name.clone(),
            });
        }
        Ok((closed, records, publication_roster))
    }

    fn revalidate_control_root(&self) -> Result<(), PrivilegedDisposableControlErrorV2> {
        self.revalidate_control_root_with_operations(self.operations_identity)
    }

    fn revalidate_control_root_with_operations(
        &self,
        operations_identity: Identity,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        if validate_directory(
            &self.root,
            self.expected_uid,
            self.expected_gid,
            0o700,
            &self.filesystem,
            "control root",
        )? != self.root_identity
            || validate_directory(
                &self.operations,
                self.expected_uid,
                self.expected_gid,
                0o700,
                &self.filesystem,
                "operations directory",
            )? != operations_identity
            || validate_regular(
                &self.lock,
                self.expected_uid,
                self.expected_gid,
                0o600,
                Some(0),
                &self.filesystem,
                "control lock",
            )? != self.lock_identity
            || validate_directory(
                &self.publication,
                self.expected_uid,
                self.expected_gid,
                0o700,
                &self.filesystem,
                "closure publication directory",
            )? != self.publication_identity
            || identity(&self.volume)? != self.volume_identity
            || named_identity(self.volume.as_raw_fd(), &self.root_name)? != self.root_identity
            || named_identity(self.root.as_raw_fd(), LOCK_NAME)? != self.lock_identity
            || named_identity(self.root.as_raw_fd(), OPERATIONS_NAME)? != operations_identity
            || named_identity(self.root.as_raw_fd(), PUBLICATION_NAME)? != self.publication_identity
        {
            return Err(invalid("retained control descriptor identity changed"));
        }
        validate_filesystem(&self.volume, &self.filesystem, "control volume")?;
        if let Some(ancestors) = &self.ancestors {
            verify_t5(&ancestors.t5)?;
            if filesystem_binding(&ancestors.t5)? != self.filesystem
                || identity(&ancestors.filesystem_root)? != ancestors.filesystem_root_identity
                || identity(&ancestors.volumes)? != ancestors.volumes_identity
                || identity(&ancestors.t5)? != ancestors.t5_identity
                || named_identity(ancestors.filesystem_root.as_raw_fd(), "Volumes")?
                    != ancestors.volumes_identity
                || named_identity(ancestors.volumes.as_raw_fd(), "T5")? != ancestors.t5_identity
            {
                return Err(invalid("retained fixed-root ancestor chain changed"));
            }
        }
        require_names(
            &list_directory(self.root.as_raw_fd(), 3)?,
            &[LOCK_NAME, OPERATIONS_NAME, PUBLICATION_NAME],
            "control root",
        )
    }

    #[cfg(test)]
    pub(crate) fn create_for_test(root: &Path) -> Result<Self, PrivilegedDisposableControlErrorV2> {
        use std::fs;
        use std::fs::OpenOptions;
        use std::os::unix::fs::OpenOptionsExt;
        use std::os::unix::fs::PermissionsExt;

        if !root.exists() {
            fs::create_dir(root)?;
            fs::set_permissions(root, fs::Permissions::from_mode(0o700))?;
            fs::create_dir(root.join(OPERATIONS_NAME))?;
            fs::set_permissions(
                root.join(OPERATIONS_NAME),
                fs::Permissions::from_mode(0o700),
            )?;
            fs::create_dir(root.join(PUBLICATION_NAME))?;
            fs::set_permissions(
                root.join(PUBLICATION_NAME),
                fs::Permissions::from_mode(0o700),
            )?;
            let lock = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
                .open(root.join(LOCK_NAME))?;
            lock.sync_all()?;
        }
        let root_file = open_path_directory(root)?;
        let volume_path = root
            .parent()
            .ok_or_else(|| invalid("test root has no volume parent"))?;
        let volume = open_path_directory(volume_path)?;
        let filesystem = filesystem_binding(&volume)?;
        Self::open_from_root(
            root_file,
            volume,
            None,
            filesystem,
            unsafe { libc::geteuid() },
            unsafe { libc::getegid() },
            root.to_str()
                .ok_or_else(|| invalid("test root is not UTF-8"))?,
            root.file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| invalid("test root name is not UTF-8"))?,
        )
    }
}

impl<'a> LivePrivilegedDisposableExecutionV2<'a> {
    pub fn receipt(&self) -> &PrivilegedDisposableExecutionV2 {
        &self.receipt
    }

    fn revalidate_for_operations<M: MountCensusStateV3>(
        &self,
        operations_identity: Identity,
        operations_roster: &[String],
        exact_mounts: &[MountBinding],
        mount_state: &M,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        self._policy
            .revalidate_control_root_with_operations(operations_identity)?;
        if current_boot_session_uuid()
            .map_err(|error| invalid(format!("cannot revalidate retained S1 boot: {error}")))?
            != self._boot_session_uuid
        {
            return Err(invalid("boot session changed after retained census"));
        }
        if list_directory(
            self._policy.operations.as_raw_fd(),
            MAX_CONTROL_OPERATIONS_V2,
        )? != operations_roster
        {
            return Err(invalid(
                "operations roster changed outside the admitted census transition",
            ));
        }
        for capsule in &self._operations {
            capsule.revalidate(
                self._policy.operations.as_raw_fd(),
                self._policy.expected_uid,
                self._policy.expected_gid,
                &self._policy.filesystem,
            )?;
        }
        for root in &self._historical_roots {
            root.revalidate(
                self._policy.volume.as_raw_fd(),
                self._policy.expected_uid,
                self._policy.expected_gid,
                &self._policy.filesystem,
            )?;
        }
        if list_directory(self._policy.volume.as_raw_fd(), MAX_VOLUME_ENTRIES)?
            != self._volume_roster
        {
            return Err(invalid("T5 root roster changed after retained census"));
        }
        if list_directory(
            self._policy.publication.as_raw_fd(),
            MAX_CONTROL_OPERATIONS_V2,
        )? != self._publication_roster
        {
            return Err(invalid(
                "closure publication roster changed after retained census",
            ));
        }
        for record in &self._publication_records {
            record.revalidate(
                self._policy.publication.as_raw_fd(),
                self._policy.expected_uid,
                self._policy.expected_gid,
                &self._policy.filesystem,
            )?;
        }
        let mounts = mount_table_snapshot()?;
        reject_nested_mounts(&mounts, &self._protected_roots)?;
        mount_state.validate_current(exact_mounts, &mounts)?;
        Ok(())
    }

    fn validate_retained_receipt(&self) -> Result<(), PrivilegedDisposableControlErrorV2> {
        if !self.receipt.storage_precondition_satisfied
            || !self.receipt.closed_world_roster_verified
            || self.receipt.admission_authority
            || self.receipt.authority.any()
            || self.receipt.operation_count != self._operation_names.len()
            || self.receipt.blocking_operation_nonces != self._exact_blocking_receipt
            || self.receipt.completed_operation_nonces != self._exact_completed_receipt
        {
            return Err(invalid(
                "serialized receipt differs from the exact descriptor-retained census",
            ));
        }
        Ok(())
    }

    pub(crate) fn into_fresh_control_census(
        self,
    ) -> Result<
        RetainedControlCensusV3<'a, FreshAdmissionV3, StableMountStateV3>,
        PrivilegedDisposableControlErrorV2,
    > {
        self.validate_retained_receipt()?;
        if !self.receipt.new_operation_precondition_satisfied
            || !self.receipt.blocking_operation_nonces.is_empty()
            || !self
                .receipt
                .legacy_v1_verified_but_awaiting_v2_closure
                .is_empty()
        {
            return Err(invalid(
                "fresh store census is not a closed-world no-authority precondition",
            ));
        }
        self.revalidate_for_operations(
            self._policy.operations_identity,
            &self._operation_names,
            &self._mounts,
            &StableMountStateV3,
        )?;
        let operations_identity = self._policy.operations_identity;
        let operations_roster = self._operation_names.clone();
        let expected_full_snapshot = self._mounts.clone();
        Ok(RetainedControlCensusV3 {
            assessment: self,
            admission: FreshAdmissionV3 {
                admitted_operation_name: None,
            },
            exact_mounts: ExactMountCensusV3 {
                expected_full_snapshot,
                state: StableMountStateV3,
            },
            operations_identity,
            operations_roster,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn into_blocking_control_census(
        self,
        target_operation_nonce: &str,
    ) -> Result<
        RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3>,
        PrivilegedDisposableControlErrorV2,
    > {
        let target_name = format!("{OPERATION_PREFIX}{target_operation_nonce}");
        operation_nonce(&target_name)?;
        self.validate_retained_receipt()?;
        let completed_count = self
            ._exact_completed_receipt
            .iter()
            .filter(|nonce| nonce.as_str() == target_operation_nonce)
            .count();
        if completed_count != 0 {
            return Err(invalid(
                "completed operation cannot be selected for restart reconciliation",
            ));
        }
        if self.receipt.new_operation_precondition_satisfied {
            return Err(invalid(
                "blocking census cannot be selected from a fresh-operation receipt",
            ));
        }
        let blocking_count = self
            ._operation_blockers
            .iter()
            .filter(|nonce| nonce.as_str() == target_operation_nonce)
            .count();
        if blocking_count != 1 {
            return Err(invalid(
                "restart target is unknown, duplicated, or not an exact blocking operation",
            ));
        }
        let mut matching_capsules = self
            ._operations
            .iter()
            .filter(|capsule| capsule.name == target_name);
        let target = matching_capsules
            .next()
            .ok_or_else(|| invalid("blocking target has no retained operation capsule"))?;
        let target_identity = target.identity;
        if matching_capsules.next().is_some() {
            return Err(invalid("blocking target capsule is duplicated"));
        }
        self.revalidate_for_operations(
            self._policy.operations_identity,
            &self._operation_names,
            &self._mounts,
            &StableMountStateV3,
        )?;
        let operations_identity = self._policy.operations_identity;
        let operations_roster = self._operation_names.clone();
        let expected_full_snapshot = self._mounts.clone();
        Ok(RetainedControlCensusV3 {
            assessment: self,
            admission: BlockingOperationV3 {
                operation_identity: target_identity,
                operation_name: target_name,
                operation_nonce: target_operation_nonce.to_string(),
            },
            exact_mounts: ExactMountCensusV3 {
                expected_full_snapshot,
                state: StableMountStateV3,
            },
            operations_identity,
            operations_roster,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl<A, M: MountCensusStateV3> RetainedControlCensusV3<'_, A, M> {
    pub(crate) fn revalidate(&self) -> Result<(), PrivilegedDisposableControlErrorV2> {
        self.assessment.validate_retained_receipt()?;
        self.assessment.revalidate_for_operations(
            self.operations_identity,
            &self.operations_roster,
            &self.exact_mounts.expected_full_snapshot,
            &self.exact_mounts.state,
        )
    }
}

impl FreshOperationAdmissionSinkV3<'_, '_> {
    pub(crate) fn retain(
        self,
        operation_directory: File,
        final_name: String,
        issue_directory: File,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        let census = self.census;
        if census.admission.admitted_operation_name.is_some() {
            return Err(invalid(
                "retained census already admitted its one fresh operation",
            ));
        }
        operation_nonce(&final_name)?;
        let next_fd_count = census
            .assessment
            ._retained_fd_count
            .checked_add(2)
            .ok_or_else(|| invalid("fresh retained descriptor count overflowed"))?;
        if next_fd_count > MAX_RETAINED_FDS {
            return Err(invalid(
                "fresh operation capsule exceeds retained descriptor bound",
            ));
        }
        let expected_uid = census.assessment._policy.expected_uid;
        let expected_gid = census.assessment._policy.expected_gid;
        let filesystem = &census.assessment._policy.filesystem;
        let operations_identity = validate_directory(
            &census.assessment._policy.operations,
            expected_uid,
            expected_gid,
            0o700,
            filesystem,
            "operations directory after exact fresh publication",
        )?;
        if named_identity(census.assessment._policy.root.as_raw_fd(), OPERATIONS_NAME)?
            != operations_identity
        {
            return Err(invalid(
                "operations directory changed during exact fresh admission",
            ));
        }
        let operation_identity = validate_directory(
            &operation_directory,
            expected_uid,
            expected_gid,
            0o700,
            filesystem,
            "freshly published operation directory",
        )?;
        if named_identity(
            census.assessment._policy.operations.as_raw_fd(),
            &final_name,
        )? != operation_identity
            || list_directory(
                operation_directory.as_raw_fd(),
                MAX_OPERATION_RECORDS_V2 + 1,
            )? != [EFFECT_ISSUE_ROOT_V3]
        {
            return Err(invalid(
                "fresh operation path or initial roster differs from its retained descriptor",
            ));
        }
        let issue_identity = validate_directory(
            &issue_directory,
            expected_uid,
            expected_gid,
            0o700,
            filesystem,
            "fresh mandatory V3 issue root",
        )?;
        if named_identity(operation_directory.as_raw_fd(), EFFECT_ISSUE_ROOT_V3)? != issue_identity
            || !list_directory(issue_directory.as_raw_fd(), MAX_EFFECT_ISSUES_V3)?.is_empty()
        {
            return Err(invalid(
                "fresh mandatory V3 issue root differs from its retained empty descriptor",
            ));
        }
        let mut expected = census.operations_roster.clone();
        expected.push(final_name.clone());
        expected.sort();
        if expected.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(invalid("fresh operation aliases an existing roster entry"));
        }
        census.assessment._operations.push(OperationCapsule {
            directory: operation_directory,
            effect_issues: Some(EffectIssueRootCapsuleV3 {
                directory: issue_directory,
                identity: issue_identity,
                issues: Vec::new(),
                name: EFFECT_ISSUE_ROOT_V3.to_string(),
                roster: Vec::new(),
            }),
            identity: operation_identity,
            name: final_name.clone(),
            record_names: Vec::new(),
            records: Vec::new(),
            roster: vec![EFFECT_ISSUE_ROOT_V3.to_string()],
        });
        census.assessment._retained_fd_count = next_fd_count;
        census.operations_identity = operations_identity;
        census.operations_roster = expected;
        census.admission.admitted_operation_name = Some(final_name);
        census.revalidate()
    }
}

impl LifecycleRecordAppendSinkV3<'_, '_> {
    pub(crate) fn retain(
        self,
        s2_directory: File,
        record: File,
        bytes: Vec<u8>,
        operation_name: String,
        record_name: String,
        record_sha256: String,
        sequence: u32,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        let (inspection, selected_nonce) = match self.target {
            LifecycleRecordAppendTargetV3::Fresh(census) => {
                let selected = census
                    .admission
                    .admitted_operation_name
                    .clone()
                    .ok_or_else(|| invalid("fresh S1 census has no exact admitted operation"))?;
                if selected != operation_name {
                    return Err(invalid(
                        "S2 lifecycle capsule differs from the freshly admitted operation",
                    ));
                }
                let inspection = absorb_transferred_lifecycle_record(
                    census,
                    &s2_directory,
                    record,
                    bytes,
                    &operation_name,
                    &record_name,
                    &record_sha256,
                    sequence,
                )?;
                (inspection, operation_nonce(&operation_name)?.to_string())
            }
            LifecycleRecordAppendTargetV3::Blocking(census) => {
                if census.admission.operation_name != operation_name {
                    return Err(invalid(
                        "S2 lifecycle capsule differs from the selected blocking operation",
                    ));
                }
                let selected_nonce = census.admission.operation_nonce.clone();
                let inspection = absorb_transferred_lifecycle_record(
                    census,
                    &s2_directory,
                    record,
                    bytes,
                    &operation_name,
                    &record_name,
                    &record_sha256,
                    sequence,
                )?;
                let target = census
                    .assessment
                    ._operations
                    .iter()
                    .find(|capsule| capsule.name == operation_name)
                    .ok_or_else(|| invalid("selected blocking capsule disappeared"))?;
                census.admission.operation_identity = target.identity;
                (inspection, selected_nonce)
            }
            LifecycleRecordAppendTargetV3::PendingMount(census) => {
                if census.admission.operation_name != operation_name {
                    return Err(invalid(
                        "S2 lifecycle capsule differs from the pending-mount operation",
                    ));
                }
                let selected_nonce = census.admission.operation_nonce.clone();
                let inspection = absorb_transferred_lifecycle_record(
                    census,
                    &s2_directory,
                    record,
                    bytes,
                    &operation_name,
                    &record_name,
                    &record_sha256,
                    sequence,
                )?;
                let target = census
                    .assessment
                    ._operations
                    .iter()
                    .find(|capsule| capsule.name == operation_name)
                    .ok_or_else(|| invalid("pending-mount operation capsule disappeared"))?;
                census.admission.operation_identity = target.identity;
                (inspection, selected_nonce)
            }
            LifecycleRecordAppendTargetV3::PendingUnmount(census) => {
                if census.admission.operation_name != operation_name {
                    return Err(invalid(
                        "S2 lifecycle capsule differs from the pending-unmount operation",
                    ));
                }
                let selected_nonce = census.admission.operation_nonce.clone();
                let inspection = absorb_transferred_lifecycle_record(
                    census,
                    &s2_directory,
                    record,
                    bytes,
                    &operation_name,
                    &record_name,
                    &record_sha256,
                    sequence,
                )?;
                let target = census
                    .assessment
                    ._operations
                    .iter()
                    .find(|capsule| capsule.name == operation_name)
                    .ok_or_else(|| invalid("pending-unmount operation capsule disappeared"))?;
                census.admission.operation_identity = target.identity;
                (inspection, selected_nonce)
            }
        };
        if inspection.operation_nonce != selected_nonce {
            return Err(invalid(
                "exact transferred lifecycle record changed the operation nonce",
            ));
        }
        if self.terminal {
            if inspection.blocks_new_operations
                || !matches!(
                    inspection.disposition,
                    LifecycleDispositionV2::TerminalCompleted
                        | LifecycleDispositionV2::TerminalAborted
                )
            {
                return Err(invalid(
                    "terminal lifecycle sink requires the exact durable terminal transition",
                ));
            }
        } else if !inspection.blocks_new_operations {
            return Err(invalid(
                "ordinary lifecycle sink cannot absorb a terminal transition",
            ));
        }
        Ok(())
    }
}

impl EffectIssueAppendSinkV3<'_, '_> {
    pub(crate) fn retain(
        self,
        s2_issue_directory: File,
        issue: File,
        bytes: Vec<u8>,
        operation_name: String,
        issue_name: String,
        issue_sha256: String,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        let census = self.census;
        if census.admission.operation_name != operation_name {
            return Err(invalid(
                "S2 effect-issue capsule differs from the selected blocking operation",
            ));
        }
        let next_fd_count = census
            .assessment
            ._retained_fd_count
            .checked_add(1)
            .ok_or_else(|| invalid("retained descriptor count overflowed"))?;
        if next_fd_count > MAX_RETAINED_FDS {
            return Err(invalid(
                "appended V3 effect issue exceeds retained descriptor bound",
            ));
        }
        let remaining_bytes = MAX_TOTAL_RECORD_BYTES
            .checked_sub(census.assessment._retained_record_bytes)
            .ok_or_else(|| invalid("retained record bytes already exceed the global bound"))?;
        let target_index = census
            .assessment
            ._operations
            .iter()
            .position(|capsule| capsule.name == operation_name)
            .ok_or_else(|| invalid("selected blocking capsule disappeared"))?;
        if census
            .assessment
            ._operations
            .iter()
            .skip(target_index + 1)
            .any(|capsule| capsule.name == operation_name)
        {
            return Err(invalid("selected blocking capsule is duplicated"));
        }
        let operation_fd = census.assessment._operations[target_index]
            .directory
            .as_raw_fd();
        let expected_uid = census.assessment._policy.expected_uid;
        let expected_gid = census.assessment._policy.expected_gid;
        let filesystem = census.assessment._policy.filesystem.clone();
        let appended_bytes = census.assessment._operations[target_index]
            .effect_issues
            .as_mut()
            .ok_or_else(|| {
                invalid("selected operation lacks the mandatory retained V3 issue root")
            })?
            .absorb_exact_append(
                operation_fd,
                expected_uid,
                expected_gid,
                &filesystem,
                s2_issue_directory,
                issue,
                bytes,
                &issue_name,
                &issue_sha256,
                remaining_bytes,
            )?;
        census.assessment._retained_fd_count = next_fd_count;
        census.assessment._retained_record_bytes = census
            .assessment
            ._retained_record_bytes
            .checked_add(appended_bytes)
            .ok_or_else(|| invalid("retained record byte count overflowed"))?;
        census.revalidate()
    }
}

fn absorb_transferred_lifecycle_record<A, M: MountCensusStateV3>(
    census: &mut RetainedControlCensusV3<'_, A, M>,
    s2_directory: &File,
    record: File,
    bytes: Vec<u8>,
    operation_name: &str,
    record_name: &str,
    record_sha256: &str,
    sequence: u32,
) -> Result<LifecycleInspectionV2, PrivilegedDisposableControlErrorV2> {
    let next_fd_count = census
        .assessment
        ._retained_fd_count
        .checked_add(1)
        .ok_or_else(|| invalid("retained descriptor count overflowed"))?;
    if next_fd_count > MAX_RETAINED_FDS {
        return Err(invalid(
            "appended lifecycle record exceeds retained descriptor bound",
        ));
    }
    let remaining_bytes = MAX_TOTAL_RECORD_BYTES
        .checked_sub(census.assessment._retained_record_bytes)
        .ok_or_else(|| invalid("retained lifecycle bytes already exceed the global bound"))?;
    let target_index = census
        .assessment
        ._operations
        .iter()
        .position(|capsule| capsule.name == operation_name)
        .ok_or_else(|| invalid("transferred lifecycle operation capsule disappeared"))?;
    if census
        .assessment
        ._operations
        .iter()
        .skip(target_index + 1)
        .any(|capsule| capsule.name == operation_name)
    {
        return Err(invalid(
            "transferred lifecycle operation capsule is duplicated",
        ));
    }
    let expected_uid = census.assessment._policy.expected_uid;
    let expected_gid = census.assessment._policy.expected_gid;
    let filesystem = census.assessment._policy.filesystem.clone();
    let s2_identity = validate_directory(
        s2_directory,
        expected_uid,
        expected_gid,
        0o700,
        &filesystem,
        "S2 retained lifecycle operation directory",
    )?;
    let target_identity = census.assessment._operations[target_index].identity;
    if s2_identity.dev != target_identity.dev || s2_identity.ino != target_identity.ino {
        return Err(invalid(
            "S2 lifecycle capsule directory is not the exact S1-retained operation inode",
        ));
    }
    let parent_fd = census.assessment._policy.operations.as_raw_fd();
    let (appended_bytes, _) = census.assessment._operations[target_index]
        .absorb_exact_record_append(
            parent_fd,
            expected_uid,
            expected_gid,
            &filesystem,
            record,
            bytes,
            record_name,
            record_sha256,
            sequence,
            remaining_bytes,
        )?;
    let lifecycle = census.assessment._operations[target_index]
        .records
        .iter()
        .map(|record| record.bytes.clone())
        .collect::<Vec<_>>();
    let inspection = match dispatch_lifecycle_records(&lifecycle)
        .map_err(|error| invalid(format!("appended target lifecycle is invalid: {error}")))?
    {
        LifecycleDispatchV2::V2(inspection) => inspection,
        LifecycleDispatchV2::HistoricalV1(_) => {
            return Err(invalid(
                "selected v2 operation changed into a historical lifecycle",
            ));
        }
    };
    census.assessment._retained_fd_count = next_fd_count;
    census.assessment._retained_record_bytes = census
        .assessment
        ._retained_record_bytes
        .checked_add(appended_bytes)
        .ok_or_else(|| invalid("retained lifecycle byte count overflowed"))?;
    census.revalidate()?;
    Ok(inspection)
}

impl<'a> RetainedControlCensusV3<'a, FreshAdmissionV3, StableMountStateV3> {
    pub(crate) fn fresh_operation_admission_sink(
        &mut self,
    ) -> Result<FreshOperationAdmissionSinkV3<'_, 'a>, PrivilegedDisposableControlErrorV2> {
        if self.admission.admitted_operation_name.is_some() {
            return Err(invalid("fresh operation was already admitted"));
        }
        Ok(FreshOperationAdmissionSinkV3 {
            census: self,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn fresh_lifecycle_record_sink(
        &mut self,
    ) -> Result<LifecycleRecordAppendSinkV3<'_, 'a>, PrivilegedDisposableControlErrorV2> {
        Ok(LifecycleRecordAppendSinkV3 {
            target: LifecycleRecordAppendTargetV3::Fresh(self),
            terminal: false,
            _not_send_or_sync: PhantomData,
        })
    }

    /// Consume the fresh S1 census into the one concrete S2 wiring object.
    /// No `File`, raw descriptor, or cloneable descriptor-bearing bundle is
    /// ever returned to the caller between S1 revalidation and S2 creation.
    pub(crate) fn wire_fresh_store(
        mut self,
        wiring: FreshCensusStoreWiringV3,
    ) -> Result<CensusBoundDurableLifecycleStoreV3<'a>, DurableLifecycleStoreErrorV3> {
        if self.admission.admitted_operation_name.is_some() {
            return Err(invalid("retained census already admitted its one fresh operation").into());
        }
        self.revalidate()?;
        let expected_gid = self.assessment._policy.expected_gid;
        let expected_uid = self.assessment._policy.expected_uid;
        let operations = self.assessment._policy.operations.try_clone()?;
        let expected_operations_inode =
            (self.operations_identity.dev, self.operations_identity.ino);
        self.revalidate()?;
        let prepared = wiring.wire(
            operations,
            expected_operations_inode,
            expected_uid,
            expected_gid,
        )?;
        prepared.bind(self)
    }
}

impl<'a> RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3> {
    pub(crate) fn begin_mount_delta(
        self,
        plan: SealedMountDeltaPlanV3<'_, MountingV3>,
    ) -> Result<
        RetainedControlCensusV3<'a, BlockingOperationV3, PendingMountDeltaV3>,
        PrivilegedDisposableControlErrorV2,
    > {
        self.revalidate()?;
        if plan.operation_nonce() != self.admission.operation_nonce
            || plan.before() != self.exact_mounts.expected_full_snapshot
        {
            return Err(invalid(
                "mount plan differs from the exact blocking operation or stable S1 census",
            ));
        }
        Ok(RetainedControlCensusV3 {
            assessment: self.assessment,
            admission: self.admission,
            exact_mounts: ExactMountCensusV3 {
                expected_full_snapshot: self.exact_mounts.expected_full_snapshot,
                state: PendingMountDeltaV3 {
                    command_sha256: plan.command_sha256().to_string(),
                    expected_after: plan.expected_after().to_vec(),
                    target: plan.target().clone(),
                    _not_send_or_sync: PhantomData,
                },
            },
            operations_identity: self.operations_identity,
            operations_roster: self.operations_roster,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn begin_unmount_delta(
        self,
        plan: SealedMountDeltaPlanV3<'_, UnmountingV3>,
    ) -> Result<
        RetainedControlCensusV3<'a, BlockingOperationV3, PendingUnmountDeltaV3>,
        PrivilegedDisposableControlErrorV2,
    > {
        self.revalidate()?;
        if plan.operation_nonce() != self.admission.operation_nonce
            || plan.before() != self.exact_mounts.expected_full_snapshot
        {
            return Err(invalid(
                "unmount plan differs from the exact blocking operation or stable S1 census",
            ));
        }
        Ok(RetainedControlCensusV3 {
            assessment: self.assessment,
            admission: self.admission,
            exact_mounts: ExactMountCensusV3 {
                expected_full_snapshot: self.exact_mounts.expected_full_snapshot,
                state: PendingUnmountDeltaV3 {
                    command_sha256: plan.command_sha256().to_string(),
                    expected_after: plan.expected_after().to_vec(),
                    target: plan.target().clone(),
                    _not_send_or_sync: PhantomData,
                },
            },
            operations_identity: self.operations_identity,
            operations_roster: self.operations_roster,
            _not_send_or_sync: PhantomData,
        })
    }

    #[cfg(test)]
    pub(crate) fn selected_record_count(&self) -> usize {
        self.assessment
            ._operations
            .iter()
            .find(|capsule| capsule.name == self.admission.operation_name)
            .map_or(0, |capsule| capsule.records.len())
    }

    #[cfg(test)]
    pub(crate) fn selected_effect_issue_count(&self) -> usize {
        self.assessment
            ._operations
            .iter()
            .find(|capsule| capsule.name == self.admission.operation_name)
            .and_then(|capsule| capsule.effect_issues.as_ref())
            .map_or(0, |root| root.issues.len())
    }

    /// Consume this census into the one concrete reconciliation wiring object.
    /// S2 receives clones of the already-retained operation and record
    /// descriptors; it never reopens the selected path from nonce bytes.
    pub(crate) fn wire_existing_store<'e, 'h>(
        self,
        wiring: ExistingCensusStoreWiringV3<'e, 'h>,
    ) -> Result<ReconciliationOperationStoreV3<'a, 'e>, DurableLifecycleStoreErrorV3> {
        self.revalidate()?;
        if named_identity(
            self.assessment._policy.operations.as_raw_fd(),
            &self.admission.operation_name,
        )? != self.admission.operation_identity
        {
            return Err(
                invalid("selected blocking operation inode changed after retained census").into(),
            );
        }
        let target = self
            .assessment
            ._operations
            .iter()
            .find(|capsule| capsule.name == self.admission.operation_name)
            .ok_or_else(|| invalid("selected blocking operation capsule disappeared"))?;
        if self
            .assessment
            ._operations
            .iter()
            .filter(|capsule| capsule.name == self.admission.operation_name)
            .count()
            != 1
        {
            return Err(invalid("selected blocking operation capsule is duplicated").into());
        }
        let operations = self.assessment._policy.operations.try_clone()?;
        let directory = target.directory.try_clone()?;
        let records = target
            .records
            .iter()
            .map(|record| {
                Ok((
                    record.name.clone(),
                    record.bytes.clone(),
                    record.file.try_clone()?,
                    (record.identity.dev, record.identity.ino),
                ))
            })
            .collect::<Result<Vec<_>, io::Error>>()?;
        let effect_issues = target
            .effect_issues
            .as_ref()
            .map(|root| {
                let records = root
                    .issues
                    .iter()
                    .map(|record| {
                        Ok((
                            record.name.clone(),
                            record.bytes.clone(),
                            record.file.try_clone()?,
                            (record.identity.dev, record.identity.ino),
                        ))
                    })
                    .collect::<Result<Vec<_>, io::Error>>()?;
                Ok::<_, io::Error>((
                    root.directory.try_clone()?,
                    (root.identity.dev, root.identity.ino),
                    records,
                ))
            })
            .transpose()?;
        let expected_operations_inode =
            (self.operations_identity.dev, self.operations_identity.ino);
        let expected_operation_inode = (
            self.admission.operation_identity.dev,
            self.admission.operation_identity.ino,
        );
        let expected_uid = self.assessment._policy.expected_uid;
        let expected_gid = self.assessment._policy.expected_gid;
        let operation_name = self.admission.operation_name.clone();
        let operation_nonce = self.admission.operation_nonce.clone();
        self.revalidate()?;
        wiring.wire(
            self,
            operations,
            expected_operations_inode,
            directory,
            expected_operation_inode,
            records,
            effect_issues,
            operation_name,
            operation_nonce,
            expected_uid,
            expected_gid,
        )
    }

    pub(crate) fn selected_lifecycle_record_sink(
        &mut self,
    ) -> Result<LifecycleRecordAppendSinkV3<'_, 'a>, PrivilegedDisposableControlErrorV2> {
        Ok(LifecycleRecordAppendSinkV3 {
            target: LifecycleRecordAppendTargetV3::Blocking(self),
            terminal: false,
            _not_send_or_sync: PhantomData,
        })
    }

    /// Test-only compatibility projection used to exercise malformed path
    /// states. Production append admission always consumes an S2-retained
    /// descriptor capsule through `selected_lifecycle_record_sink`.
    #[cfg(test)]
    pub(crate) fn admit_selected_lifecycle_append(
        &mut self,
        expected_record_sha256: &str,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        let operation_name = self.admission.operation_name.clone();
        let target = self
            .assessment
            ._operations
            .iter()
            .find(|capsule| capsule.name == operation_name)
            .ok_or_else(|| invalid("selected blocking capsule disappeared"))?;
        let sequence = target
            .records
            .len()
            .checked_add(1)
            .ok_or_else(|| invalid("test lifecycle sequence overflowed"))?;
        let record_name = format!("{sequence:08}.json");
        let record = openat_node(target.directory.as_raw_fd(), &record_name, libc::O_RDONLY)?;
        let identity = validate_regular(
            &record,
            self.assessment._policy.expected_uid,
            self.assessment._policy.expected_gid,
            0o400,
            Some(MAX_RECORD_BYTES as i64),
            &self.assessment._policy.filesystem,
            "test lifecycle append record",
        )?;
        let bytes = read_stable(&record, identity)?;
        let directory = target.directory.try_clone()?;
        let sequence = u32::try_from(sequence)
            .map_err(|_| invalid("test lifecycle sequence overflowed u32"))?;
        let sink = self.selected_lifecycle_record_sink()?;
        sink.retain(
            directory,
            record,
            bytes,
            operation_name,
            record_name,
            expected_record_sha256.to_string(),
            sequence,
        )
    }

    pub(crate) fn selected_terminal_record_sink(
        &mut self,
    ) -> Result<LifecycleRecordAppendSinkV3<'_, 'a>, PrivilegedDisposableControlErrorV2> {
        Ok(LifecycleRecordAppendSinkV3 {
            target: LifecycleRecordAppendTargetV3::Blocking(self),
            terminal: true,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn selected_effect_issue_sink(
        &mut self,
    ) -> Result<EffectIssueAppendSinkV3<'_, 'a>, PrivilegedDisposableControlErrorV2> {
        Ok(EffectIssueAppendSinkV3 {
            census: self,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn complete_selected_lifecycle_append(
        mut self,
        append: &RetainedLifecycleRecordAppendV3,
    ) -> Result<
        RetainedControlCensusV3<'a, CompletedOperationV3, StableMountStateV3>,
        PrivilegedDisposableControlErrorV2,
    > {
        append.require_s1_adopted().map_err(|error| {
            invalid(format!(
                "terminal append capsule is not S1-retained: {error}"
            ))
        })?;
        let inspection = self.selected_lifecycle_inspection()?;
        if inspection.operation_nonce != self.admission.operation_nonce
            || inspection.blocks_new_operations
            || !matches!(
                inspection.disposition,
                LifecycleDispositionV2::TerminalCompleted | LifecycleDispositionV2::TerminalAborted
            )
        {
            return Err(invalid(
                "completed transition requires the exact durable terminal lifecycle record",
            ));
        }
        if remove_exact_once(
            &mut self.assessment._operation_blockers,
            &self.admission.operation_nonce,
        )? == 0
            || remove_exact_once(
                &mut self.assessment._exact_blocking_receipt,
                &self.admission.operation_nonce,
            )? == 0
            || self
                .assessment
                ._exact_completed_receipt
                .iter()
                .any(|nonce| nonce == &self.admission.operation_nonce)
        {
            return Err(invalid(
                "selected terminal transition disagrees with retained blocker classification",
            ));
        }
        self.assessment
            ._exact_completed_receipt
            .push(self.admission.operation_nonce.clone());
        self.assessment._exact_completed_receipt.sort();
        self.assessment.receipt.blocking_operation_nonces =
            self.assessment._exact_blocking_receipt.clone();
        self.assessment.receipt.completed_operation_nonces =
            self.assessment._exact_completed_receipt.clone();
        self.assessment.receipt.new_operation_precondition_satisfied =
            self.assessment._exact_blocking_receipt.is_empty()
                && self
                    .assessment
                    .receipt
                    .legacy_v1_verified_but_awaiting_v2_closure
                    .is_empty();
        self.revalidate()?;
        let admission = CompletedOperationV3 {
            operation_identity: self.admission.operation_identity,
            operation_name: self.admission.operation_name,
            operation_nonce: self.admission.operation_nonce,
            terminal_record_sha256: append.digest().to_string(),
        };
        Ok(RetainedControlCensusV3 {
            assessment: self.assessment,
            admission,
            exact_mounts: self.exact_mounts,
            operations_identity: self.operations_identity,
            operations_roster: self.operations_roster,
            _not_send_or_sync: PhantomData,
        })
    }

    fn selected_lifecycle_inspection(
        &self,
    ) -> Result<LifecycleInspectionV2, PrivilegedDisposableControlErrorV2> {
        let target_index = self
            .assessment
            ._operations
            .iter()
            .position(|capsule| capsule.name == self.admission.operation_name)
            .ok_or_else(|| invalid("selected blocking capsule disappeared"))?;
        if self
            .assessment
            ._operations
            .iter()
            .skip(target_index + 1)
            .any(|capsule| capsule.name == self.admission.operation_name)
        {
            return Err(invalid("selected blocking capsule is duplicated"));
        }
        let target = &self.assessment._operations[target_index];
        let lifecycle = target
            .records
            .iter()
            .map(|record| record.bytes.clone())
            .collect::<Vec<_>>();
        let inspection = match dispatch_lifecycle_records(&lifecycle)
            .map_err(|error| invalid(format!("appended target lifecycle is invalid: {error}")))?
        {
            LifecycleDispatchV2::V2(inspection) => inspection,
            LifecycleDispatchV2::HistoricalV1(_) => {
                return Err(invalid(
                    "selected v2 operation changed into a historical lifecycle",
                ));
            }
        };
        Ok(inspection)
    }
}

impl<'a> RetainedControlCensusV3<'a, BlockingOperationV3, PendingMountDeltaV3> {
    pub(crate) fn selected_lifecycle_record_sink(
        &mut self,
    ) -> Result<LifecycleRecordAppendSinkV3<'_, 'a>, PrivilegedDisposableControlErrorV2> {
        self.revalidate()?;
        Ok(LifecycleRecordAppendSinkV3 {
            target: LifecycleRecordAppendTargetV3::PendingMount(self),
            terminal: false,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn advance_mount_delta(
        self,
        advance: SealedMountDeltaAdvanceV3<'_, MountingV3>,
    ) -> Result<
        RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3>,
        PrivilegedDisposableControlErrorV2,
    > {
        self.revalidate()?;
        advance.revalidate().map_err(|error| {
            invalid(format!(
                "post-mount collector could not seal the S1 advance: {error}"
            ))
        })?;
        if advance.operation_nonce() != self.admission.operation_nonce
            || advance.before() != self.exact_mounts.expected_full_snapshot
            || advance.expected_after() != self.exact_mounts.state.expected_after
            || advance.command_sha256() != self.exact_mounts.state.command_sha256
            || advance.target() != &self.exact_mounts.state.target
        {
            return Err(invalid(
                "post-mount advance differs from the exact pending S1 transition",
            ));
        }
        let current = mount_table_snapshot()?;
        reject_nested_mounts(&current, &self.assessment._protected_roots)?;
        if current != self.exact_mounts.state.expected_after {
            return Err(invalid(
                "post-mount S1 census is not the exact durable expected-after snapshot",
            ));
        }
        self.revalidate()?;
        advance.revalidate().map_err(|error| {
            invalid(format!(
                "post-mount collector changed during final S1 replay: {error}"
            ))
        })?;
        Ok(RetainedControlCensusV3 {
            assessment: self.assessment,
            admission: self.admission,
            exact_mounts: ExactMountCensusV3 {
                expected_full_snapshot: self.exact_mounts.state.expected_after,
                state: StableMountStateV3,
            },
            operations_identity: self.operations_identity,
            operations_roster: self.operations_roster,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl<'a> RetainedControlCensusV3<'a, BlockingOperationV3, PendingUnmountDeltaV3> {
    pub(crate) fn selected_lifecycle_record_sink(
        &mut self,
    ) -> Result<LifecycleRecordAppendSinkV3<'_, 'a>, PrivilegedDisposableControlErrorV2> {
        self.revalidate()?;
        Ok(LifecycleRecordAppendSinkV3 {
            target: LifecycleRecordAppendTargetV3::PendingUnmount(self),
            terminal: false,
            _not_send_or_sync: PhantomData,
        })
    }

    pub(crate) fn advance_unmount_delta(
        self,
        advance: SealedMountDeltaAdvanceV3<'_, UnmountingV3>,
    ) -> Result<
        RetainedControlCensusV3<'a, BlockingOperationV3, StableMountStateV3>,
        PrivilegedDisposableControlErrorV2,
    > {
        self.revalidate()?;
        advance.revalidate().map_err(|error| {
            invalid(format!(
                "post-unmount collector could not seal the S1 advance: {error}"
            ))
        })?;
        if advance.operation_nonce() != self.admission.operation_nonce
            || advance.before() != self.exact_mounts.expected_full_snapshot
            || advance.expected_after() != self.exact_mounts.state.expected_after
            || advance.command_sha256() != self.exact_mounts.state.command_sha256
            || advance.target() != &self.exact_mounts.state.target
        {
            return Err(invalid(
                "post-unmount advance differs from the exact pending S1 transition",
            ));
        }
        let current = mount_table_snapshot()?;
        reject_nested_mounts(&current, &self.assessment._protected_roots)?;
        if current != self.exact_mounts.state.expected_after {
            return Err(invalid(
                "post-unmount S1 census is not the exact durable expected-after snapshot",
            ));
        }
        self.revalidate()?;
        advance.revalidate().map_err(|error| {
            invalid(format!(
                "post-unmount collector changed during final S1 replay: {error}"
            ))
        })?;
        Ok(RetainedControlCensusV3 {
            assessment: self.assessment,
            admission: self.admission,
            exact_mounts: ExactMountCensusV3 {
                expected_full_snapshot: self.exact_mounts.state.expected_after,
                state: StableMountStateV3,
            },
            operations_identity: self.operations_identity,
            operations_roster: self.operations_roster,
            _not_send_or_sync: PhantomData,
        })
    }
}

impl RetainedControlCensusV3<'_, CompletedOperationV3, StableMountStateV3> {
    #[cfg(test)]
    pub(crate) fn completed_binding(&self) -> (&str, &str, (u64, u64)) {
        (
            &self.admission.operation_nonce,
            &self.admission.terminal_record_sha256,
            (
                self.admission.operation_identity.dev,
                self.admission.operation_identity.ino,
            ),
        )
    }

    #[cfg(test)]
    pub(crate) fn completed_operation_name(&self) -> &str {
        &self.admission.operation_name
    }

    #[cfg(test)]
    pub(crate) fn completed_receipt(&self) -> &PrivilegedDisposableExecutionV2 {
        &self.assessment.receipt
    }
}

fn remove_exact_once(
    values: &mut Vec<String>,
    selected: &str,
) -> Result<usize, PrivilegedDisposableControlErrorV2> {
    let count = values
        .iter()
        .filter(|value| value.as_str() == selected)
        .count();
    if count > 1 {
        return Err(invalid("retained operation classification is duplicated"));
    }
    values.retain(|value| value != selected);
    Ok(count)
}

impl OperationCapsule {
    fn absorb_exact_record_append(
        &mut self,
        parent_fd: RawFd,
        expected_uid: u32,
        expected_gid: u32,
        filesystem: &FilesystemBinding,
        file: File,
        bytes: Vec<u8>,
        exact_name: &str,
        expected_record_sha256: &str,
        exact_sequence: u32,
        maximum_appended_bytes: usize,
    ) -> Result<(usize, Identity), PrivilegedDisposableControlErrorV2> {
        require_hex_nonce(expected_record_sha256, "appended lifecycle record SHA-256")?;
        let next_sequence = self
            .record_names
            .len()
            .checked_add(1)
            .ok_or_else(|| invalid("operation record sequence overflowed"))?;
        if next_sequence > MAX_OPERATION_RECORDS_V2 {
            return Err(invalid("operation record count exceeds retained bound"));
        }
        let next_name = format!("{next_sequence:08}.json");
        if exact_name != next_name || usize::try_from(exact_sequence).ok() != Some(next_sequence) {
            return Err(invalid(
                "S2 lifecycle capsule name or sequence differs from the exact next record",
            ));
        }
        let mut expected_roster = self.record_names.clone();
        expected_roster.push(next_name.clone());
        if self.effect_issues.is_some() {
            expected_roster.push(EFFECT_ISSUE_ROOT_V3.to_string());
        }
        expected_roster.sort();
        let after_identity = validate_directory(
            &self.directory,
            expected_uid,
            expected_gid,
            0o700,
            filesystem,
            "selected operation directory after lifecycle append",
        )?;
        let named_after = named_identity(parent_fd, &self.name)?;
        let roster_after =
            list_directory(self.directory.as_raw_fd(), MAX_OPERATION_RECORDS_V2 + 1)?;
        if !self
            .identity
            .same_directory_across_record_append(after_identity)
            || named_after != after_identity
            || roster_after != expected_roster
        {
            return Err(invalid(format!(
                "selected operation changed by more than one exact lifecycle append: before={:?} after={after_identity:?} named={named_after:?} expected_roster={expected_roster:?} actual_roster={roster_after:?}",
                self.identity,
            )));
        }
        for record in &self.records {
            record.revalidate(
                self.directory.as_raw_fd(),
                expected_uid,
                expected_gid,
                filesystem,
            )?;
        }
        let record_identity = validate_regular(
            &file,
            expected_uid,
            expected_gid,
            0o400,
            Some(MAX_RECORD_BYTES as i64),
            filesystem,
            "appended lifecycle record",
        )?;
        if self.records.iter().any(|record| {
            record.identity.dev == record_identity.dev && record.identity.ino == record_identity.ino
        }) {
            return Err(invalid(
                "appended lifecycle record aliases an already retained record",
            ));
        }
        let descriptor_bytes = read_stable(&file, record_identity)?;
        if named_identity(self.directory.as_raw_fd(), &next_name)? != record_identity
            || descriptor_bytes != bytes
        {
            return Err(invalid(
                "S2 lifecycle capsule differs from the exact final pathname or descriptor bytes",
            ));
        }
        if descriptor_bytes.len() > maximum_appended_bytes {
            return Err(invalid(
                "appended lifecycle record exceeds retained global byte budget",
            ));
        }
        if sha256(&descriptor_bytes) != expected_record_sha256 {
            return Err(invalid(
                "appended lifecycle record differs from the S2 durable digest",
            ));
        }
        let appended_bytes = descriptor_bytes.len();
        self.identity = after_identity;
        self.record_names.push(next_name.clone());
        self.roster = expected_roster;
        self.records.push(RecordCapsule {
            bytes: descriptor_bytes,
            file,
            identity: record_identity,
            name: next_name,
        });
        self.revalidate(parent_fd, expected_uid, expected_gid, filesystem)?;
        Ok((appended_bytes, record_identity))
    }

    fn revalidate(
        &self,
        parent_fd: RawFd,
        expected_uid: u32,
        expected_gid: u32,
        filesystem: &FilesystemBinding,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        if named_identity(parent_fd, &self.name)? != self.identity
            || validate_directory(
                &self.directory,
                expected_uid,
                expected_gid,
                0o700,
                filesystem,
                "operation directory",
            )? != self.identity
            || list_directory(self.directory.as_raw_fd(), MAX_OPERATION_RECORDS_V2 + 1)?
                != self.roster
        {
            return Err(invalid(
                "operation directory changed during descriptor replay",
            ));
        }
        for record in &self.records {
            record.revalidate(
                self.directory.as_raw_fd(),
                expected_uid,
                expected_gid,
                filesystem,
            )?;
        }
        if let Some(effect_issues) = &self.effect_issues {
            effect_issues.revalidate(
                self.directory.as_raw_fd(),
                expected_uid,
                expected_gid,
                filesystem,
            )?;
        }
        Ok(())
    }

    fn register(
        &self,
        registry: &mut CensusRegistry,
        prefix: &str,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        let path = format!("{prefix}/{}", self.name);
        registry.insert(&path, self.identity)?;
        for record in &self.records {
            record.register(registry, &path)?;
        }
        if let Some(effect_issues) = &self.effect_issues {
            effect_issues.register(registry, &path)?;
        }
        Ok(())
    }
}

fn open_effect_issue_root(
    operation_fd: RawFd,
    total_bytes: &mut usize,
    retained_fds: &mut RetainedFdBudget,
    expected_uid: u32,
    expected_gid: u32,
    filesystem: &FilesystemBinding,
) -> Result<EffectIssueRootCapsuleV3, PrivilegedDisposableControlErrorV2> {
    retained_fds.reserve("V3 effect-issue directory")?;
    let directory = openat_node(
        operation_fd,
        EFFECT_ISSUE_ROOT_V3,
        libc::O_RDONLY | libc::O_DIRECTORY,
    )?;
    let identity = validate_directory(
        &directory,
        expected_uid,
        expected_gid,
        0o700,
        filesystem,
        "V3 effect-issue directory",
    )?;
    if named_identity(operation_fd, EFFECT_ISSUE_ROOT_V3)? != identity {
        return Err(invalid(
            "V3 effect-issue root differs from its retained descriptor",
        ));
    }
    let roster = list_directory(directory.as_raw_fd(), MAX_EFFECT_ISSUES_V3)?;
    let mut issues = Vec::with_capacity(roster.len());
    for name in &roster {
        if !valid_effect_issue_name(name) {
            return Err(invalid(
                "V3 effect-issue root contains a noncanonical entry",
            ));
        }
        retained_fds.reserve("V3 effect-issue record")?;
        let file = openat_node(directory.as_raw_fd(), name, libc::O_RDONLY)?;
        let issue_identity = validate_regular(
            &file,
            expected_uid,
            expected_gid,
            0o400,
            Some(MAX_RECORD_BYTES as i64),
            filesystem,
            "V3 effect-issue record",
        )?;
        if named_identity(directory.as_raw_fd(), name)? != issue_identity {
            return Err(invalid(
                "V3 effect-issue pathname differs from its descriptor",
            ));
        }
        let bytes = read_stable(&file, issue_identity)?;
        *total_bytes = total_bytes
            .checked_add(bytes.len())
            .ok_or_else(|| invalid("V3 effect-issue byte budget overflowed"))?;
        if *total_bytes > MAX_TOTAL_RECORD_BYTES {
            return Err(invalid("V3 effect-issue byte budget exceeded"));
        }
        let name_digest = name
            .strip_suffix(".json")
            .and_then(|value| value.rsplit_once('-'))
            .map(|(_, digest)| digest)
            .ok_or_else(|| invalid("V3 effect-issue filename is malformed"))?;
        if sha256(&bytes) != name_digest {
            return Err(invalid(
                "V3 effect-issue bytes differ from their filename digest",
            ));
        }
        issues.push(RecordCapsule {
            bytes,
            file,
            identity: issue_identity,
            name: name.clone(),
        });
    }
    Ok(EffectIssueRootCapsuleV3 {
        directory,
        identity,
        issues,
        name: EFFECT_ISSUE_ROOT_V3.to_string(),
        roster,
    })
}

fn valid_effect_issue_name(name: &str) -> bool {
    let Some(value) = name
        .strip_prefix("effect-")
        .and_then(|v| v.strip_suffix(".json"))
    else {
        return false;
    };
    let Some((effect, digest)) = value.split_once('-') else {
        return false;
    };
    effect.len() == 20
        && effect.as_bytes().iter().all(u8::is_ascii_digit)
        && effect != "00000000000000000000"
        && digest.len() == 64
        && digest
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

impl EffectIssueRootCapsuleV3 {
    fn revalidate(
        &self,
        operation_fd: RawFd,
        expected_uid: u32,
        expected_gid: u32,
        filesystem: &FilesystemBinding,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        if named_identity(operation_fd, &self.name)? != self.identity
            || validate_directory(
                &self.directory,
                expected_uid,
                expected_gid,
                0o700,
                filesystem,
                "retained V3 effect-issue directory",
            )? != self.identity
            || list_directory(self.directory.as_raw_fd(), MAX_EFFECT_ISSUES_V3)? != self.roster
        {
            return Err(invalid(
                "retained V3 effect-issue directory changed during replay",
            ));
        }
        for issue in &self.issues {
            issue.revalidate(
                self.directory.as_raw_fd(),
                expected_uid,
                expected_gid,
                filesystem,
            )?;
        }
        Ok(())
    }

    fn absorb_exact_append(
        &mut self,
        operation_fd: RawFd,
        expected_uid: u32,
        expected_gid: u32,
        filesystem: &FilesystemBinding,
        s2_directory: File,
        file: File,
        bytes: Vec<u8>,
        expected_name: &str,
        expected_sha256: &str,
        maximum_appended_bytes: usize,
    ) -> Result<usize, PrivilegedDisposableControlErrorV2> {
        if !valid_effect_issue_name(expected_name) {
            return Err(invalid("appended V3 issue filename is not canonical"));
        }
        require_hex_nonce(expected_sha256, "appended V3 issue SHA-256")?;
        let name_sha256 = expected_name
            .strip_suffix(".json")
            .and_then(|value| value.rsplit_once('-'))
            .map(|(_, digest)| digest)
            .ok_or_else(|| invalid("appended V3 issue filename is malformed"))?;
        if name_sha256 != expected_sha256 {
            return Err(invalid(
                "appended V3 issue filename digest differs from its retained digest",
            ));
        }
        let mut expected_roster = self.roster.clone();
        expected_roster.push(expected_name.to_string());
        expected_roster.sort();
        let after_identity = validate_directory(
            &self.directory,
            expected_uid,
            expected_gid,
            0o700,
            filesystem,
            "V3 effect-issue directory after append",
        )?;
        let s2_directory_identity = validate_directory(
            &s2_directory,
            expected_uid,
            expected_gid,
            0o700,
            filesystem,
            "S2 retained V3 effect-issue directory",
        )?;
        if !self
            .identity
            .same_directory_across_record_append(after_identity)
            || s2_directory_identity != after_identity
            || named_identity(operation_fd, &self.name)? != after_identity
            || list_directory(self.directory.as_raw_fd(), MAX_EFFECT_ISSUES_V3)? != expected_roster
        {
            return Err(invalid(
                "V3 effect-issue root changed by more than one exact append",
            ));
        }
        for issue in &self.issues {
            issue.revalidate(
                self.directory.as_raw_fd(),
                expected_uid,
                expected_gid,
                filesystem,
            )?;
        }
        let identity = validate_regular(
            &file,
            expected_uid,
            expected_gid,
            0o400,
            Some(MAX_RECORD_BYTES as i64),
            filesystem,
            "appended V3 effect-issue record",
        )?;
        let descriptor_bytes = read_stable(&file, identity)?;
        if descriptor_bytes != bytes
            || bytes.len() > maximum_appended_bytes
            || sha256(&bytes) != expected_sha256
            || named_identity(self.directory.as_raw_fd(), expected_name)? != identity
            || self.issues.iter().any(|issue| {
                issue.identity.dev == identity.dev && issue.identity.ino == identity.ino
            })
        {
            return Err(invalid(
                "appended V3 effect-issue record failed exact retained replay",
            ));
        }
        let appended_bytes = bytes.len();
        self.identity = after_identity;
        self.roster = expected_roster;
        self.issues.push(RecordCapsule {
            bytes,
            file,
            identity,
            name: expected_name.to_string(),
        });
        self.revalidate(operation_fd, expected_uid, expected_gid, filesystem)?;
        Ok(appended_bytes)
    }

    fn register(
        &self,
        registry: &mut CensusRegistry,
        prefix: &str,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        let path = format!("{prefix}/{}", self.name);
        registry.insert(&path, self.identity)?;
        for issue in &self.issues {
            issue.register(registry, &path)?;
        }
        Ok(())
    }
}

impl RecordCapsule {
    fn revalidate(
        &self,
        parent_fd: RawFd,
        expected_uid: u32,
        expected_gid: u32,
        filesystem: &FilesystemBinding,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        if named_identity(parent_fd, &self.name)? != self.identity
            || validate_regular(
                &self.file,
                expected_uid,
                expected_gid,
                0o400,
                Some(MAX_RECORD_BYTES as i64),
                filesystem,
                "lifecycle or closure record",
            )? != self.identity
            || read_stable(&self.file, self.identity)? != self.bytes
        {
            return Err(invalid("record changed during descriptor replay"));
        }
        Ok(())
    }

    fn register(
        &self,
        registry: &mut CensusRegistry,
        prefix: &str,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        registry.insert(&format!("{prefix}/{}", self.name), self.identity)
    }
}

impl HistoricalRootCapsule {
    fn revalidate(
        &self,
        volume_fd: RawFd,
        expected_uid: u32,
        expected_gid: u32,
        filesystem: &FilesystemBinding,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        if named_identity(volume_fd, &self.name)? != self.identity
            || validate_directory(
                &self.directory,
                expected_uid,
                expected_gid,
                0o700,
                filesystem,
                "historical qualification root",
            )? != self.identity
            || validate_directory(
                &self.publication,
                expected_uid,
                expected_gid,
                0o700,
                filesystem,
                "historical publication directory",
            )? != self.publication_identity
            || validate_directory(
                &self.barrier,
                expected_uid,
                expected_gid,
                0o700,
                filesystem,
                "historical barrier journal",
            )? != self.barrier_identity
            || named_identity(self.directory.as_raw_fd(), "publication")?
                != self.publication_identity
            || named_identity(self.directory.as_raw_fd(), "barrier-journal")?
                != self.barrier_identity
            || list_directory(self.directory.as_raw_fd(), MAX_HISTORICAL_ROOT_ENTRIES)?
                != self.roster
            || list_directory(self.publication.as_raw_fd(), MAX_HISTORICAL_ROOT_ENTRIES)?
                != self.publication_roster
            || list_directory(self.barrier.as_raw_fd(), MAX_HISTORICAL_ROOT_ENTRIES)?
                != self.barrier_roster
        {
            return Err(invalid("historical root changed during descriptor replay"));
        }
        for obligation in &self.obligations {
            obligation.revalidate(
                self.publication.as_raw_fd(),
                expected_uid,
                expected_gid,
                filesystem,
            )?;
        }
        for node in &self.barrier_nodes {
            node.revalidate(
                self.barrier.as_raw_fd(),
                expected_uid,
                expected_gid,
                filesystem,
            )?;
        }
        for node in &self.other_nodes {
            node.revalidate(
                self.publication.as_raw_fd(),
                expected_uid,
                expected_gid,
                filesystem,
            )?;
        }
        Ok(())
    }

    fn descriptor_count(&self) -> usize {
        3 + self
            .barrier_nodes
            .iter()
            .map(HeldNode::descriptor_count)
            .sum::<usize>()
            + self
                .other_nodes
                .iter()
                .map(HeldNode::descriptor_count)
                .sum::<usize>()
            + self
                .obligations
                .iter()
                .map(|obligation| 1 + obligation.records.len())
                .sum::<usize>()
    }

    fn register(
        &self,
        registry: &mut CensusRegistry,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        registry.insert(&self.name, self.identity)?;
        let publication_path = format!("{}/publication", self.name);
        registry.insert(&publication_path, self.publication_identity)?;
        let barrier_path = format!("{}/barrier-journal", self.name);
        registry.insert(&barrier_path, self.barrier_identity)?;
        for node in &self.barrier_nodes {
            node.register(registry, &barrier_path)?;
        }
        for obligation in &self.obligations {
            obligation.register(registry, &publication_path)?;
        }
        for node in &self.other_nodes {
            node.register(registry, &publication_path)?;
        }
        Ok(())
    }
}

impl HeldNode {
    fn child_named(&self, name: &str) -> Option<&HeldNode> {
        self.children.iter().find(|child| child.name == name)
    }

    fn read_regular_bytes(&self) -> Result<Vec<u8>, PrivilegedDisposableControlErrorV2> {
        if self.identity.mode & libc::S_IFMT as u16 != libc::S_IFREG as u16 {
            return Err(invalid(
                "held fixture semantic object is not a regular file",
            ));
        }
        read_stable(&self.file, self.identity)
    }

    fn revalidate(
        &self,
        parent_fd: RawFd,
        expected_uid: u32,
        expected_gid: u32,
        filesystem: &FilesystemBinding,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        if named_identity(parent_fd, &self.name)? != self.identity {
            return Err(invalid("held historical node name changed during replay"));
        }
        let file_type = self.identity.mode & libc::S_IFMT as u16;
        let mode = self.identity.mode & 0o7777;
        if file_type == libc::S_IFDIR as u16 {
            if validate_directory(
                &self.file,
                expected_uid,
                expected_gid,
                mode,
                filesystem,
                "held historical directory",
            )? != self.identity
                || list_directory(self.file.as_raw_fd(), MAX_HISTORICAL_ROOT_ENTRIES)?
                    != self.roster
            {
                return Err(invalid("held historical directory changed during replay"));
            }
            for child in &self.children {
                child.revalidate(
                    self.file.as_raw_fd(),
                    expected_uid,
                    expected_gid,
                    filesystem,
                )?;
            }
        } else if file_type == libc::S_IFREG as u16 {
            if validate_regular(
                &self.file,
                expected_uid,
                expected_gid,
                mode,
                self.bytes.as_ref().map(|_| MAX_RECORD_BYTES as i64),
                filesystem,
                "held historical regular file",
            )? != self.identity
                || self.bytes.as_ref().is_some_and(|bytes| {
                    read_stable(&self.file, self.identity).ok().as_ref() != Some(bytes)
                })
            {
                return Err(invalid(
                    "held historical regular file changed during replay",
                ));
            }
        } else {
            return Err(invalid("held historical node became a special node"));
        }
        Ok(())
    }

    fn descriptor_count(&self) -> usize {
        1 + self
            .children
            .iter()
            .map(HeldNode::descriptor_count)
            .sum::<usize>()
    }

    fn register(
        &self,
        registry: &mut CensusRegistry,
        prefix: &str,
    ) -> Result<(), PrivilegedDisposableControlErrorV2> {
        let path = format!("{prefix}/{}", self.name);
        registry.insert(&path, self.identity)?;
        for child in &self.children {
            child.register(registry, &path)?;
        }
        Ok(())
    }
}

struct OpenedFixedRoot {
    ancestors: AncestorChain,
    root: File,
}

fn open_fixed_root() -> Result<OpenedFixedRoot, PrivilegedDisposableControlErrorV2> {
    let filesystem_root = open_path_directory(Path::new("/"))?;
    let volumes = openat_node(
        filesystem_root.as_raw_fd(),
        "Volumes",
        libc::O_RDONLY | libc::O_DIRECTORY,
    )?;
    let t5 = openat_node(
        volumes.as_raw_fd(),
        "T5",
        libc::O_RDONLY | libc::O_DIRECTORY,
    )?;
    let root = openat_node(
        t5.as_raw_fd(),
        ".hepta-privileged-disposable-v2",
        libc::O_RDONLY | libc::O_DIRECTORY,
    )?;
    let ancestors = AncestorChain {
        filesystem_root_identity: identity(&filesystem_root)?,
        volumes_identity: identity(&volumes)?,
        t5_identity: identity(&t5)?,
        filesystem_root,
        t5,
        volumes,
    };
    Ok(OpenedFixedRoot { ancestors, root })
}

fn open_path_directory(path: &Path) -> Result<File, PrivilegedDisposableControlErrorV2> {
    let path = CString::new(path.as_os_str().as_encoded_bytes())
        .map_err(|_| invalid("path contains NUL"))?;
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    file_from_fd(fd)
}

fn openat_node(
    parent_fd: RawFd,
    name: &str,
    access: libc::c_int,
) -> Result<File, PrivilegedDisposableControlErrorV2> {
    let name = CString::new(name).map_err(|_| invalid("node name contains NUL"))?;
    let fd = unsafe {
        libc::openat(
            parent_fd,
            name.as_ptr(),
            access | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    file_from_fd(fd)
}

fn file_from_fd(fd: RawFd) -> Result<File, PrivilegedDisposableControlErrorV2> {
    if fd < 0 {
        Err(io::Error::last_os_error().into())
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

fn list_directory(
    fd: RawFd,
    maximum: usize,
) -> Result<Vec<String>, PrivilegedDisposableControlErrorV2> {
    let dot = c".";
    let duplicate = unsafe {
        libc::openat(
            fd,
            dot.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    if duplicate < 0 {
        return Err(io::Error::last_os_error().into());
    }
    let directory = unsafe { libc::fdopendir(duplicate) };
    if directory.is_null() {
        unsafe { libc::close(duplicate) };
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
        let raw = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
        let name = match std::str::from_utf8(raw) {
            Ok(name) => name,
            Err(_) => {
                unsafe { libc::closedir(directory) };
                return Err(invalid("directory entry is not UTF-8"));
            }
        };
        if name == "." || name == ".." {
            continue;
        }
        if names.len() == maximum {
            unsafe { libc::closedir(directory) };
            return Err(invalid("closed-world directory entry limit exceeded"));
        }
        names.push(name.to_string());
    }
    names.sort();
    Ok(names)
}

fn open_sealed_fixture_node(
    parent_fd: RawFd,
    name: &str,
    relative: &str,
    depth: usize,
    retained_fds: &mut RetainedFdBudget,
    expected_uid: u32,
    expected_gid: u32,
    filesystem: &FilesystemBinding,
) -> Result<HeldNode, PrivilegedDisposableControlErrorV2> {
    if depth > MAX_DESCRIPTOR_DEPTH {
        return Err(invalid(
            "sealed fixture descriptor census exceeds its depth bound",
        ));
    }
    let path_identity = named_identity(parent_fd, name)?;
    let file_type = path_identity.mode & libc::S_IFMT as u16;
    if file_type == libc::S_IFDIR as u16 {
        retained_fds.reserve("historical sealed fixture directory")?;
        let file = openat_node(parent_fd, name, libc::O_RDONLY | libc::O_DIRECTORY)?;
        let expected_mode = if relative == "mountpoint" {
            0o700
        } else {
            0o500
        };
        let node_identity = validate_directory(
            &file,
            expected_uid,
            expected_gid,
            expected_mode,
            filesystem,
            "historical sealed fixture directory",
        )?;
        if node_identity != path_identity {
            return Err(invalid(
                "sealed fixture directory changed between fstatat and openat",
            ));
        }
        let roster = list_directory(file.as_raw_fd(), MAX_HISTORICAL_ROOT_ENTRIES)?;
        let mut children = Vec::with_capacity(roster.len());
        for child_name in &roster {
            let child_relative = if relative == "." {
                child_name.clone()
            } else {
                format!("{relative}/{child_name}")
            };
            children.push(open_sealed_fixture_node(
                file.as_raw_fd(),
                child_name,
                &child_relative,
                depth + 1,
                retained_fds,
                expected_uid,
                expected_gid,
                filesystem,
            )?);
        }
        Ok(HeldNode {
            bytes: None,
            children,
            file,
            identity: node_identity,
            name: name.to_string(),
            roster,
        })
    } else if file_type == libc::S_IFREG as u16 {
        if relative == "." {
            return Err(invalid("historical sealed fixture root is not a directory"));
        }
        retained_fds.reserve("historical sealed fixture file")?;
        let file = openat_node(parent_fd, name, libc::O_RDONLY)?;
        let node_identity = validate_regular(
            &file,
            expected_uid,
            expected_gid,
            0o400,
            None,
            filesystem,
            "historical sealed fixture file",
        )?;
        if node_identity != path_identity {
            return Err(invalid(
                "sealed fixture file changed between fstatat and openat",
            ));
        }
        Ok(HeldNode {
            bytes: None,
            children: Vec::new(),
            file,
            identity: node_identity,
            name: name.to_string(),
            roster: Vec::new(),
        })
    } else {
        Err(invalid(
            "sealed fixture contains a symlink or unsupported special node",
        ))
    }
}

fn validate_directory(
    file: &File,
    uid: u32,
    gid: u32,
    mode: u16,
    filesystem: &FilesystemBinding,
    label: &str,
) -> Result<Identity, PrivilegedDisposableControlErrorV2> {
    let identity = identity(file)?;
    if identity.mode & libc::S_IFMT as u16 != libc::S_IFDIR as u16
        || identity.mode & 0o7777 != mode
        || identity.uid != uid
        || identity.gid != gid
        || identity.flags != 0
    {
        return Err(invalid(format!(
            "{label} type, owner, mode, or flags are invalid"
        )));
    }
    validate_filesystem(file, filesystem, label)?;
    verify_acl_absent(file.as_raw_fd(), label)?;
    verify_xattrs_empty(file.as_raw_fd(), label)?;
    Ok(identity)
}

fn validate_regular(
    file: &File,
    uid: u32,
    gid: u32,
    mode: u16,
    maximum_size: Option<i64>,
    filesystem: &FilesystemBinding,
    label: &str,
) -> Result<Identity, PrivilegedDisposableControlErrorV2> {
    let identity = identity(file)?;
    if identity.mode & libc::S_IFMT as u16 != libc::S_IFREG as u16
        || identity.mode & 0o7777 != mode
        || identity.uid != uid
        || identity.gid != gid
        || identity.flags != 0
        || identity.nlink != 1
        || identity.size < 0
        || maximum_size.is_some_and(|maximum| identity.size > maximum)
    {
        return Err(invalid(format!(
            "{label} type, owner, mode, flags, link count, or size is invalid"
        )));
    }
    validate_filesystem(file, filesystem, label)?;
    verify_acl_absent(file.as_raw_fd(), label)?;
    verify_xattrs_empty(file.as_raw_fd(), label)?;
    Ok(identity)
}

fn identity(file: &File) -> Result<Identity, PrivilegedDisposableControlErrorV2> {
    let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    if unsafe { libc::fstat(file.as_raw_fd(), stat.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(identity_from_stat(unsafe { stat.assume_init() }))
}

fn named_identity(
    parent_fd: RawFd,
    name: &str,
) -> Result<Identity, PrivilegedDisposableControlErrorV2> {
    let name = CString::new(name).map_err(|_| invalid("node name contains NUL"))?;
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
    Ok(identity_from_stat(unsafe { stat.assume_init() }))
}

fn identity_from_stat(stat: libc::stat) -> Identity {
    Identity {
        ctime_nsec: stat.st_ctime_nsec,
        ctime_sec: stat.st_ctime,
        dev: stat.st_dev as u64,
        flags: stat.st_flags,
        gid: stat.st_gid,
        ino: stat.st_ino,
        mode: stat.st_mode as u16,
        mtime_nsec: stat.st_mtime_nsec,
        mtime_sec: stat.st_mtime,
        nlink: stat.st_nlink as u64,
        size: stat.st_size,
        uid: stat.st_uid,
    }
}

fn read_stable(
    file: &File,
    expected: Identity,
) -> Result<Vec<u8>, PrivilegedDisposableControlErrorV2> {
    let length = usize::try_from(expected.size).map_err(|_| invalid("negative record size"))?;
    if length > MAX_RECORD_BYTES {
        return Err(invalid("record size limit exceeded"));
    }
    let mut bytes = vec![0; length];
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
    if identity(file)? != expected {
        return Err(invalid("record identity changed during descriptor read"));
    }
    Ok(bytes)
}

fn operation_nonce(name: &str) -> Result<&str, PrivilegedDisposableControlErrorV2> {
    let nonce = name
        .strip_prefix(OPERATION_PREFIX)
        .ok_or_else(|| invalid("operations roster contains an unknown or temporary entry"))?;
    if nonce.len() != 64
        || !nonce
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        return Err(invalid("operation directory nonce is malformed"));
    }
    Ok(nonce)
}

fn require_hex_nonce(value: &str, label: &str) -> Result<(), PrivilegedDisposableControlErrorV2> {
    if value.len() != 64
        || !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        return Err(invalid(format!(
            "{label} is not 64 lowercase hexadecimal characters"
        )));
    }
    Ok(())
}

fn historical_publication_record_nonce(name: &str) -> Option<&str> {
    let Some(remainder) = name.strip_prefix("hepta-operation-") else {
        return None;
    };
    [
        ".prepared.json",
        ".mechanism-receipt.json",
        ".terminal-receipt.json",
    ]
    .iter()
    .find_map(|suffix| {
        remainder
            .strip_suffix(suffix)
            .filter(|nonce| require_hex_nonce(nonce, "historical publication nonce").is_ok())
    })
}

fn historical_obligation_closure_eligible(
    verification: &AttachmentObligationVerificationV1,
) -> bool {
    !verification.authority_granted
        && verification.current_boot
        && verification.disposition == ObligationDispositionV1::Reconciled
        && !verification.requires_privileged_reconciliation
}

fn historical_obligation_semantic_blocker(
    root_name: &str,
    verification: &AttachmentObligationVerificationV1,
) -> Option<String> {
    if !historical_obligation_closure_eligible(verification) {
        Some(format!(
            "{root_name}/{}:historical_v1_prior_boot_or_reconciliation_required",
            verification.operation_nonce
        ))
    } else {
        None
    }
}

fn typed_sha256<T: Serialize>(value: &T) -> Result<String, PrivilegedDisposableControlErrorV2> {
    canonical_json(value)
        .map(|bytes| crate::durable::sha256(&bytes))
        .map_err(|error| invalid(format!("typed digest serialization failed: {error}")))
}

fn require_names(
    actual: &[String],
    expected: &[&str],
    label: &str,
) -> Result<(), PrivilegedDisposableControlErrorV2> {
    let mut expected = expected
        .iter()
        .map(|name| (*name).to_string())
        .collect::<Vec<_>>();
    expected.sort();
    if actual != expected {
        return Err(invalid(format!(
            "{label} is not an exact closed-world roster"
        )));
    }
    Ok(())
}

fn fixed_c_string(
    bytes: &[libc::c_char],
    label: &str,
) -> Result<String, PrivilegedDisposableControlErrorV2> {
    let nul = bytes
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| invalid(format!("{label} is not NUL terminated")))?;
    let raw = bytes[..nul]
        .iter()
        .map(|byte| *byte as u8)
        .collect::<Vec<_>>();
    String::from_utf8(raw).map_err(|_| invalid(format!("{label} is not UTF-8")))
}

fn filesystem_binding(
    file: &File,
) -> Result<FilesystemBinding, PrivilegedDisposableControlErrorV2> {
    let mut filesystem = std::mem::MaybeUninit::<libc::statfs>::uninit();
    if unsafe { libc::fstatfs(file.as_raw_fd(), filesystem.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    let filesystem = unsafe { filesystem.assume_init() };
    Ok(FilesystemBinding {
        dev: identity(file)?.dev,
        filesystem_id: unsafe { std::mem::transmute::<libc::fsid_t, [i32; 2]>(filesystem.f_fsid) },
        filesystem_type: fixed_c_string(&filesystem.f_fstypename, "filesystem type")?,
        mount_flags: filesystem.f_flags as u64,
        mount_from: fixed_c_string(&filesystem.f_mntfromname, "mount source")?,
        mount_on: fixed_c_string(&filesystem.f_mntonname, "mountpoint")?,
    })
}

fn validate_filesystem(
    file: &File,
    expected: &FilesystemBinding,
    label: &str,
) -> Result<(), PrivilegedDisposableControlErrorV2> {
    if &filesystem_binding(file)? != expected {
        return Err(invalid(format!(
            "{label} is on a nested mount or differs from the canonical control filesystem"
        )));
    }
    Ok(())
}

fn mount_binding(
    filesystem: &libc::statfs,
) -> Result<MountBinding, PrivilegedDisposableControlErrorV2> {
    Ok(MountBinding {
        filesystem_id: unsafe { std::mem::transmute::<libc::fsid_t, [i32; 2]>(filesystem.f_fsid) },
        filesystem_type: fixed_c_string(&filesystem.f_fstypename, "mount-table filesystem type")?,
        mount_flags: filesystem.f_flags as u64,
        mount_from: fixed_c_string(&filesystem.f_mntfromname, "mount-table source")?,
        mount_on: fixed_c_string(&filesystem.f_mntonname, "mount-table target")?,
    })
}

fn mount_table_snapshot() -> Result<Vec<MountBinding>, PrivilegedDisposableControlErrorV2> {
    let count = unsafe { libc::getfsstat(std::ptr::null_mut(), 0, libc::MNT_NOWAIT) };
    if count < 0 {
        return Err(io::Error::last_os_error().into());
    }
    let count = usize::try_from(count).map_err(|_| invalid("mount-table count overflowed"))?;
    if count > MAX_MOUNT_ENTRIES {
        return Err(invalid("mount-table count exceeds the closed-world bound"));
    }
    let capacity = count
        .checked_add(16)
        .ok_or_else(|| invalid("mount-table capacity overflowed"))?;
    if capacity > MAX_MOUNT_ENTRIES {
        return Err(invalid("mount-table growth allowance exceeds the bound"));
    }
    let mut mounts = vec![unsafe { std::mem::zeroed::<libc::statfs>() }; capacity];
    let buffer_bytes = mounts
        .len()
        .checked_mul(std::mem::size_of::<libc::statfs>())
        .and_then(|size| i32::try_from(size).ok())
        .ok_or_else(|| invalid("mount-table buffer size overflowed"))?;
    let observed = unsafe { libc::getfsstat(mounts.as_mut_ptr(), buffer_bytes, libc::MNT_NOWAIT) };
    if observed < 0 || observed as usize > mounts.len() {
        return Err(if observed < 0 {
            io::Error::last_os_error().into()
        } else {
            invalid("mount table grew beyond its bounded snapshot")
        });
    }
    mounts.truncate(observed as usize);
    let mut snapshot = mounts
        .iter()
        .map(mount_binding)
        .collect::<Result<Vec<_>, _>>()?;
    snapshot.sort();
    Ok(snapshot)
}

fn reject_nested_mounts(
    mounts: &[MountBinding],
    protected_roots: &[PathBuf],
) -> Result<(), PrivilegedDisposableControlErrorV2> {
    for root in protected_roots {
        if !root.is_absolute() {
            return Err(invalid("protected mount-table root is not absolute"));
        }
        let root = root
            .to_str()
            .ok_or_else(|| invalid("protected mount-table root is not UTF-8"))?;
        let prefix = format!("{}/", root.trim_end_matches('/'));
        if mounts
            .iter()
            .any(|mount| mount.mount_on == root || mount.mount_on.starts_with(&prefix))
        {
            return Err(invalid(format!(
                "nested mount exists at or below protected root {root}"
            )));
        }
    }
    Ok(())
}

fn verify_acl_absent(fd: RawFd, label: &str) -> Result<(), PrivilegedDisposableControlErrorV2> {
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
    let rc = unsafe { acl_get_entry(acl, ACL_FIRST_ENTRY, &mut entry) };
    let error = io::Error::last_os_error();
    if unsafe { acl_free(acl) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    match rc {
        0 => Err(invalid(format!("{label} has an extended ACL"))),
        -1 if error.raw_os_error() == Some(libc::EINVAL) => Ok(()),
        _ => Err(error.into()),
    }
}

fn verify_xattrs_empty(fd: RawFd, label: &str) -> Result<(), PrivilegedDisposableControlErrorV2> {
    let count = unsafe { libc::flistxattr(fd, std::ptr::null_mut(), 0, 0) };
    if count < 0 {
        return Err(io::Error::last_os_error().into());
    }
    if count != 0 {
        return Err(invalid(format!("{label} has extended attributes")));
    }
    Ok(())
}

fn verify_t5(t5: &File) -> Result<(), PrivilegedDisposableControlErrorV2> {
    let mut filesystem = std::mem::MaybeUninit::<libc::statfs>::uninit();
    if unsafe { libc::fstatfs(t5.as_raw_fd(), filesystem.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    let filesystem = unsafe { filesystem.assume_init() };
    let filesystem_type = unsafe { CStr::from_ptr(filesystem.f_fstypename.as_ptr()) }
        .to_str()
        .map_err(|_| invalid("T5 filesystem type is not UTF-8"))?;
    let mounted_on = unsafe { CStr::from_ptr(filesystem.f_mntonname.as_ptr()) }
        .to_str()
        .map_err(|_| invalid("T5 mountpoint is not UTF-8"))?;
    if filesystem_type != "apfs"
        || mounted_on != "/Volumes/T5"
        || filesystem.f_flags & libc::MNT_IGNORE_OWNERSHIP as u32 != 0
    {
        return Err(invalid(
            "T5 is not ownership-enabled APFS mounted exactly at /Volumes/T5",
        ));
    }
    let mut attributes = libc::attrlist {
        bitmapcount: libc::ATTR_BIT_MAP_COUNT,
        reserved: 0,
        commonattr: 0,
        volattr: libc::ATTR_VOL_INFO | libc::ATTR_VOL_UUID,
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
            t5.as_raw_fd(),
            (&mut attributes as *mut libc::attrlist).cast(),
            (&mut buffer as *mut VolumeUuidBuffer).cast(),
            std::mem::size_of::<VolumeUuidBuffer>(),
            0,
        )
    } != 0
    {
        return Err(io::Error::last_os_error().into());
    }
    if buffer.length as usize != std::mem::size_of::<VolumeUuidBuffer>()
        || buffer.uuid != EXPECTED_T5_UUID
    {
        return Err(invalid("T5 volume UUID differs from the canonical pin"));
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> PrivilegedDisposableControlErrorV2 {
    PrivilegedDisposableControlErrorV2::Invalid(message.into())
}

unsafe extern "C" {
    fn acl_free(object: *mut libc::c_void) -> libc::c_int;
    fn acl_get_entry(
        acl: *mut libc::c_void,
        entry_id: libc::c_int,
        entry_p: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn acl_get_fd_np(fd: libc::c_int, acl_type: libc::c_int) -> *mut libc::c_void;
}

#[cfg(test)]
#[path = "mac_privileged_disposable_control_tests.rs"]
mod tests;
