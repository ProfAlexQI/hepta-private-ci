use super::NativeSysErrorV8;
use super::NativeSysResultV8;
#[cfg(all(test, target_os = "linux"))]
use super::ProcessObservationV8;
use crate::DurableRunnerRestoreIntentV8;
use crate::DurableRunnerStopIntentV8;
use crate::RunnerRestoreEffectEvidenceV8;
use crate::RunnerRestoreEvidencePhaseV8;
#[cfg(target_os = "linux")]
use crate::RunnerRestoreProcessEvidenceV8;
use crate::RunnerStopEffectEvidenceV8;
use crate::RunnerStopEvidencePhaseV8;
#[cfg(target_os = "linux")]
use crate::RunnerStopProcessEvidenceV8;

#[cfg(target_os = "linux")]
use super::FileIdentityV8;
#[cfg(target_os = "linux")]
use super::io_error;
#[cfg(not(target_os = "linux"))]
use super::unsupported;
#[cfg(target_os = "linux")]
use sha2::Digest as _;
#[cfg(target_os = "linux")]
use std::cell::Cell;
#[cfg(any(target_os = "linux", test))]
use std::collections::BTreeSet;
#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::marker::PhantomData;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::FromRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;
#[cfg(all(test, target_os = "linux"))]
use std::os::unix::ffi::OsStrExt as _;
#[cfg(all(test, target_os = "linux"))]
use std::path::Path;
#[cfg(target_os = "linux")]
use std::path::PathBuf;
#[cfg(target_os = "linux")]
use std::sync::Arc;
use std::time::Duration;
#[cfg(target_os = "linux")]
use std::time::Instant;

#[cfg(target_os = "linux")]
const CGROUP2_SUPER_MAGIC_V8: libc::c_long = 0x6367_7270;
#[cfg(target_os = "linux")]
const CGROUP_CONTROL_MAX_BYTES_V8: usize = 64 * 1024;
pub const ADMISSIOND_CGROUP_RELATIVE_PATH_V8: &str =
    "system.slice/hepta-linux-v8-admissiond.service";
pub const ADMISSIOND_CGROUP_ABSOLUTE_PATH_V8: &str =
    "/sys/fs/cgroup/system.slice/hepta-linux-v8-admissiond.service";
#[cfg(target_os = "linux")]
const MAX_CGROUP_WAIT_V8: Duration = Duration::from_secs(60);
#[cfg(target_os = "linux")]
const CGROUP_WAIT_POLL_V8: Duration = Duration::from_millis(10);
#[cfg(target_os = "linux")]
const MAX_RUNNER_SCOPE_WAIT_V8: Duration = Duration::from_secs(10);
#[cfg(target_os = "linux")]
const MAX_RUNNER_ARTIFACTS_V8: usize = 32;
#[cfg(target_os = "linux")]
const MAX_RUNNER_ARTIFACT_HASH_BYTES_V8: u64 = 256 * 1024 * 1024;
#[cfg(target_os = "linux")]
const MAX_RUNNER_ARTIFACT_ELAPSED_V8: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CgroupTypeV8 {
    Domain,
    DomainThreaded,
    DomainInvalid,
    Threaded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CgroupEventsV8 {
    populated: bool,
    frozen: bool,
}

impl CgroupEventsV8 {
    pub fn populated(self) -> bool {
        self.populated
    }

    pub fn frozen(self) -> bool {
        self.frozen
    }
}

/// Descriptor-anchored proof of the fixed unified cgroup-v2 root. The root
/// cgroup type and exact parent `cgroup.subtree_control` set are retained and
/// revalidated before every state-changing child operation.
#[derive(Debug)]
pub struct CgroupV2RootV8 {
    #[cfg(target_os = "linux")]
    inner: Arc<CgroupRootInnerV8>,
    cgroup_type: CgroupTypeV8,
    controllers: Vec<String>,
    subtree_control: Vec<String>,
    delegated_parent_path: String,
}

impl CgroupV2RootV8 {
    pub fn cgroup_type(&self) -> CgroupTypeV8 {
        self.cgroup_type
    }

    pub fn subtree_control(&self) -> &[String] {
        &self.subtree_control
    }

    pub fn controllers(&self) -> &[String] {
        &self.controllers
    }

    pub fn delegated_parent_path(&self) -> &str {
        &self.delegated_parent_path
    }

    pub fn revalidate(&self) -> NativeSysResultV8<()> {
        revalidate_cgroup_root_impl_v8(self)
    }
}

/// Single-writer ownership of admissiond's delegated namespace.  It consumes
/// the root proof, is bound to the acquiring PID, and is deliberately `!Sync`.
/// A live candidate leaf always owns this lease until descriptor-bound cleanup
/// and name-absence proof return it.
#[must_use = "dropping the namespace lease abandons candidate cleanup authority"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupNamespaceLeaseV8 {
    root: CgroupV2RootV8,
    #[cfg(target_os = "linux")]
    owner_pid: u32,
    #[cfg(target_os = "linux")]
    not_sync: PhantomData<Cell<()>>,
}

impl CandidateCgroupNamespaceLeaseV8 {
    pub(crate) fn parent_path(&self) -> &str {
        self.root.delegated_parent_path()
    }

    fn revalidate(&self) -> NativeSysResultV8<()> {
        #[cfg(not(target_os = "linux"))]
        {
            return Err(unsupported("revalidate candidate cgroup namespace lease"));
        }
        #[cfg(target_os = "linux")]
        {
            // SAFETY: getpid has no pointer arguments or preconditions.
            let pid = unsafe { libc::getpid() };
            if u32::try_from(pid).ok() != Some(self.owner_pid) {
                return Err(NativeSysErrorV8::RaceDetected(
                    "candidate cgroup namespace lease crossed a process boundary".to_string(),
                ));
            }
            self.root.revalidate()
        }
    }
}

/// Fresh exact nondelegated child plus the namespace lease that makes its
/// parent/name binding exclusive for the entire lifecycle.
#[must_use = "a created candidate cgroup must be cleaned through its owning token"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupLeafV8 {
    lease: CandidateCgroupNamespaceLeaseV8,
    leaf: CgroupV2LeafV8,
}

impl CandidateCgroupLeafV8 {
    pub(crate) fn leaf(&self) -> &str {
        self.leaf.leaf()
    }

    pub(crate) fn absolute_path(&self) -> &str {
        self.leaf.absolute_path()
    }
}

#[must_use = "post-mkdir state owns the only safe cleanup route"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupCreateIssuedOrUncertainV8 {
    lease: CandidateCgroupNamespaceLeaseV8,
    leaf_name: String,
    leaf: Option<CgroupV2LeafV8>,
    #[cfg(target_os = "linux")]
    unvalidated_descriptor: Option<OwnedFd>,
    cause: NativeSysErrorV8,
}

impl CandidateCgroupCreateIssuedOrUncertainV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(crate) fn leaf_name(&self) -> &str {
        &self.leaf_name
    }
}

#[must_use = "create failure retains either retry authority or cleanup obligation"]
#[derive(Debug)]
pub(crate) enum CandidateCgroupCreateFailureV8 {
    BeforeEffect {
        lease: CandidateCgroupNamespaceLeaseV8,
        cause: NativeSysErrorV8,
    },
    IssuedOrUncertain(CandidateCgroupCreateIssuedOrUncertainV8),
}

#[must_use = "create recovery failure retains the post-mkdir obligation"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupCreateRecoveryFailureV8 {
    obligation: CandidateCgroupCreateIssuedOrUncertainV8,
    cause: NativeSysErrorV8,
}

impl CandidateCgroupCreateRecoveryFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(crate) fn into_obligation(self) -> CandidateCgroupCreateIssuedOrUncertainV8 {
        self.obligation
    }
}

#[must_use = "empty-leaf validation failure retains candidate cleanup authority"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupPrepareCleanupFailureV8 {
    candidate: CandidateCgroupLeafV8,
    cause: NativeSysErrorV8,
}

impl CandidateCgroupPrepareCleanupFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(crate) fn into_candidate(self) -> CandidateCgroupLeafV8 {
        self.candidate
    }
}

/// Parent-side token returned only after clone3 created a child directly in
/// the exact leaf and the child was observed blocked behind its start pipe.
#[must_use = "the clone3 child remains blocked until released or recovered"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupStartBarrierV8 {
    candidate: CandidateCgroupLeafV8,
    child: super::PidHandleV8,
    #[cfg(target_os = "linux")]
    release: OwnedFd,
}

impl CandidateCgroupStartBarrierV8 {
    pub(crate) fn child_pid(&self) -> u32 {
        self.child.pid()
    }
}

/// Child-side marker.  The child receives it only after the parent writes the
/// one release byte.  All inherited control descriptors are CLOEXEC; callers
/// must immediately replace the child with an already verified executable.
#[must_use = "a released clone3 child must immediately exec or _exit"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupChildReleasedV8 {
    _private: (),
}

#[must_use = "both clone3 branches require explicit handling"]
#[derive(Debug)]
pub(crate) enum CandidateCgroupCloneOutcomeV8 {
    Parent(CandidateCgroupStartBarrierV8),
    Child(CandidateCgroupChildReleasedV8),
}

#[must_use = "a successful clone3 may own a blocked child requiring recovery"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupCloneIssuedOrUncertainV8 {
    candidate: CandidateCgroupLeafV8,
    child_pid: u32,
    verified_child: Option<super::PidHandleV8>,
    #[cfg(target_os = "linux")]
    raw_pidfd: Option<OwnedFd>,
    #[cfg(target_os = "linux")]
    release: OwnedFd,
    cause: NativeSysErrorV8,
}

impl CandidateCgroupCloneIssuedOrUncertainV8 {
    pub(crate) fn child_pid(&self) -> u32 {
        self.child_pid
    }

    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }
}

#[must_use = "clone failure retains the leaf or a live-child recovery token"]
#[derive(Debug)]
pub(crate) enum CandidateCgroupCloneFailureV8 {
    BeforeEffect {
        candidate: CandidateCgroupLeafV8,
        cause: NativeSysErrorV8,
    },
    IssuedOrUncertain(CandidateCgroupCloneIssuedOrUncertainV8),
}

impl CandidateCgroupCloneFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        match self {
            Self::BeforeEffect { cause, .. } => cause,
            Self::IssuedOrUncertain(obligation) => obligation.cause(),
        }
    }

    pub(crate) fn into_before_effect_candidate(self) -> Option<CandidateCgroupLeafV8> {
        match self {
            Self::BeforeEffect { candidate, .. } => Some(candidate),
            Self::IssuedOrUncertain(_) => None,
        }
    }

    pub(crate) fn into_issued(self) -> Option<CandidateCgroupCloneIssuedOrUncertainV8> {
        match self {
            Self::BeforeEffect { .. } => None,
            Self::IssuedOrUncertain(obligation) => Some(obligation),
        }
    }
}

#[must_use = "release failure may have made the child runnable"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupReleaseIssuedOrUncertainV8 {
    barrier: CandidateCgroupStartBarrierV8,
    cause: NativeSysErrorV8,
}

impl CandidateCgroupReleaseIssuedOrUncertainV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }
}

#[must_use = "a running candidate owns the leaf until kill/wait/cleanup"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupRunningV8 {
    candidate: CandidateCgroupLeafV8,
    child_pid: u32,
    child: Option<super::PidHandleV8>,
}

impl CandidateCgroupRunningV8 {
    pub(crate) fn child_pid(&self) -> u32 {
        self.child_pid
    }

    pub(crate) fn absolute_path(&self) -> &str {
        self.candidate.absolute_path()
    }
}

#[cfg(all(test, target_os = "linux"))]
#[must_use = "explicit fixture population failure retains candidate cleanup authority"]
#[derive(Debug)]
pub(super) struct CandidateCgroupTestPopulationFailureV8 {
    candidate: CandidateCgroupLeafV8,
    effect_issued_or_uncertain: bool,
    cause: NativeSysErrorV8,
}

#[cfg(all(test, target_os = "linux"))]
impl CandidateCgroupTestPopulationFailureV8 {
    pub(super) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(super) fn effect_issued_or_uncertain(&self) -> bool {
        self.effect_issued_or_uncertain
    }

    pub(super) fn into_candidate(self) -> CandidateCgroupLeafV8 {
        self.candidate
    }
}

#[must_use = "a populated candidate owns kill/wait/cleanup authority"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupPopulationV8 {
    candidate: CandidateCgroupLeafV8,
    child_pid: u32,
    verified_child: Option<super::PidHandleV8>,
    #[cfg(target_os = "linux")]
    raw_pidfd: Option<OwnedFd>,
    #[cfg(target_os = "linux")]
    blocked_release_guard: Option<OwnedFd>,
}

#[must_use = "kill planning failure retains the populated candidate"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupKillBeforeEffectV8 {
    population: CandidateCgroupPopulationV8,
    cause: NativeSysErrorV8,
}

impl CandidateCgroupKillBeforeEffectV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(crate) fn into_population(self) -> CandidateCgroupPopulationV8 {
        self.population
    }
}

/// Whether write(2) returned success or an ambiguous error, this token owns
/// the exact leaf and child handles until a bounded empty wait succeeds.
#[must_use = "cgroup.kill was issued or uncertain; wait or quarantine explicitly"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupKillIssuedOrUncertainV8 {
    population: CandidateCgroupPopulationV8,
    cause: Option<NativeSysErrorV8>,
}

impl CandidateCgroupKillIssuedOrUncertainV8 {
    pub(crate) fn issue_was_uncertain(&self) -> bool {
        self.cause.is_some()
    }

    pub(crate) fn cause(&self) -> Option<&NativeSysErrorV8> {
        self.cause.as_ref()
    }
}

#[must_use = "kill failure retains either an unissued target or post-issue obligation"]
#[derive(Debug)]
pub(crate) enum CandidateCgroupKillFailureV8 {
    BeforeEffect(CandidateCgroupKillBeforeEffectV8),
    IssuedOrUncertain(CandidateCgroupKillIssuedOrUncertainV8),
}

#[must_use = "empty wait failure returns the still-live kill obligation"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupWaitFailureV8 {
    issued: CandidateCgroupKillIssuedOrUncertainV8,
    cause: NativeSysErrorV8,
}

impl CandidateCgroupWaitFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(crate) fn into_issued(self) -> CandidateCgroupKillIssuedOrUncertainV8 {
        self.issued
    }
}

#[must_use = "an empty candidate leaf still requires descriptor-bound cleanup"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupEmptyV8 {
    candidate: CandidateCgroupLeafV8,
    empty: CgroupEmptyProofV8,
}

impl CandidateCgroupEmptyV8 {
    pub(crate) fn proof(&self) -> &CgroupEmptyProofV8 {
        &self.empty
    }
}

#[must_use = "cleanup failure retains the exact empty leaf or post-rmdir obligation"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupCleanupFailureV8 {
    empty: CandidateCgroupEmptyV8,
    cleanup_effect_issued_or_uncertain: bool,
    cause: NativeSysErrorV8,
}

impl CandidateCgroupCleanupFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(crate) fn cleanup_effect_issued_or_uncertain(&self) -> bool {
        self.cleanup_effect_issued_or_uncertain
    }

    pub(crate) fn into_empty(self) -> CandidateCgroupEmptyV8 {
        self.empty
    }
}

#[must_use = "terminal containment evidence and namespace lease bind one completed attempt"]
#[derive(Debug)]
pub(crate) struct CandidateCgroupTerminalV8 {
    lease: CandidateCgroupNamespaceLeaseV8,
    empty: CgroupEmptyProofV8,
    absence: CgroupLeafAbsenceProofV8,
}

impl CandidateCgroupTerminalV8 {
    pub(crate) fn empty_proof(&self) -> &CgroupEmptyProofV8 {
        &self.empty
    }

    pub(crate) fn absence_proof(&self) -> &CgroupLeafAbsenceProofV8 {
        &self.absence
    }

    pub(crate) fn into_namespace_lease(self) -> CandidateCgroupNamespaceLeaseV8 {
        self.lease
    }
}

impl CandidateCgroupCreateFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        match self {
            Self::BeforeEffect { cause, .. } => cause,
            Self::IssuedOrUncertain(obligation) => obligation.cause(),
        }
    }

    pub(crate) fn into_before_effect_lease(self) -> Option<CandidateCgroupNamespaceLeaseV8> {
        match self {
            Self::BeforeEffect { lease, .. } => Some(lease),
            Self::IssuedOrUncertain(_) => None,
        }
    }

    pub(crate) fn into_issued(self) -> Option<CandidateCgroupCreateIssuedOrUncertainV8> {
        match self {
            Self::BeforeEffect { .. } => None,
            Self::IssuedOrUncertain(obligation) => Some(obligation),
        }
    }
}

/// Descriptor-bound existing systemd scope.  Unlike candidate containment it
/// is never created or removed here; it exists solely for the independent
/// freeze/STOP/unfreeze runner-pause state machine.
#[must_use = "an observed runner scope must remain descriptor-bound through restore"]
#[derive(Debug)]
pub(crate) struct ExistingSystemdScopeCgroupV8 {
    #[cfg(target_os = "linux")]
    cgroup_root: OwnedFd,
    #[cfg(target_os = "linux")]
    cgroup_root_identity: FileIdentityV8,
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    #[cfg(target_os = "linux")]
    identity: FileIdentityV8,
    #[cfg(target_os = "linux")]
    observer: super::ProcfsObserverBindingV8,
    #[cfg(target_os = "linux")]
    backend: ExistingScopeBackendV8,
    control_group: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TrustedRunnerArtifactV8 {
    pub(crate) absolute_path: String,
    pub(crate) device: u64,
    pub(crate) inode: u64,
    pub(crate) size: u64,
    pub(crate) mode: u32,
    pub(crate) owner_uid: u32,
    pub(crate) owner_gid: u32,
    pub(crate) sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TrustedRunnerDirectoryV8 {
    pub(crate) absolute_path: String,
    pub(crate) device: u64,
    pub(crate) inode: u64,
    pub(crate) mode: u32,
    pub(crate) owner_uid: u32,
    pub(crate) owner_gid: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RunnerScopeProcessManifestV8 {
    pub(crate) pid: u32,
    pub(crate) start_ticks: u64,
    pub(crate) parent_pid: u32,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) process_group_id: u32,
    pub(crate) session_id: u32,
    pub(crate) executable_sha256: [u8; 32],
    pub(crate) argv_sha256: [u8; 32],
    pub(crate) cwd_device: u64,
    pub(crate) cwd_inode: u64,
}

/// Trusted expected systemd/process/artifact closure.  Construction alone is
/// not authority; the future runtime must derive it from the verified signed
/// attempt before calling the crate-private observation boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RunnerScopeTrustedManifestV8 {
    pub(crate) boot_id: super::BootIdV8,
    pub(crate) unit_name: String,
    pub(crate) delegate: bool,
    pub(crate) control_group: String,
    pub(crate) main_pid: u32,
    pub(crate) fragment: TrustedRunnerArtifactV8,
    pub(crate) workroot: TrustedRunnerDirectoryV8,
    pub(crate) artifacts: Vec<TrustedRunnerArtifactV8>,
    pub(crate) processes: Vec<RunnerScopeProcessManifestV8>,
}

#[must_use = "an observed runner scope owns the only pause/restore route"]
#[derive(Debug)]
pub(crate) struct ObservedRunnerScopeV8 {
    manifest: RunnerScopeTrustedManifestV8,
    scope: ExistingSystemdScopeCgroupV8,
    group: super::ProcessGroupObservationV8,
    initial: Vec<super::ProcessIdentityV8>,
    runner_stop_intent: Option<DurableRunnerStopIntentV8>,
}

impl ObservedRunnerScopeV8 {
    pub(crate) fn unit_name(&self) -> &str {
        &self.manifest.unit_name
    }

    pub(crate) fn capture_runner_stop_manifest_v8(
        &self,
    ) -> NativeSysResultV8<RunnerStopEffectEvidenceV8> {
        capture_runner_stop_evidence_impl_v8(self, RunnerStopEvidencePhaseV8::PreEffect, None, None)
    }
}

#[must_use = "freeze failure retains either the unmodified scope or freeze obligation"]
#[derive(Debug)]
pub(crate) enum RunnerScopeFreezeFailureV8 {
    IntentConflict {
        observed: ObservedRunnerScopeV8,
        unbound_intent: DurableRunnerStopIntentV8,
        cause: NativeSysErrorV8,
    },
    BeforeEffect {
        observed: ObservedRunnerScopeV8,
        cause: NativeSysErrorV8,
    },
    IssuedOrUncertain(RunnerScopeFreezeIssuedOrUncertainV8),
}

#[must_use = "cgroup.freeze=1 was issued or uncertain; wait or recover explicitly"]
#[derive(Debug)]
pub(crate) struct RunnerScopeFreezeIssuedOrUncertainV8 {
    observed: ObservedRunnerScopeV8,
    cause: Option<NativeSysErrorV8>,
}

impl RunnerScopeFreezeFailureV8 {
    pub(crate) fn cause(&self) -> Option<&NativeSysErrorV8> {
        match self {
            Self::IntentConflict { cause, .. } | Self::BeforeEffect { cause, .. } => Some(cause),
            Self::IssuedOrUncertain(issued) => issued.cause(),
        }
    }

    pub(crate) fn into_before_effect(self) -> Option<ObservedRunnerScopeV8> {
        match self {
            Self::BeforeEffect { observed, .. } => Some(observed),
            Self::IntentConflict { .. } | Self::IssuedOrUncertain(_) => None,
        }
    }

    pub(crate) fn into_issued(self) -> Option<RunnerScopeFreezeIssuedOrUncertainV8> {
        match self {
            Self::IntentConflict { .. } | Self::BeforeEffect { .. } => None,
            Self::IssuedOrUncertain(issued) => Some(issued),
        }
    }

    pub(crate) fn into_intent_conflict(
        self,
    ) -> Option<(ObservedRunnerScopeV8, DurableRunnerStopIntentV8)> {
        match self {
            Self::IntentConflict {
                observed,
                unbound_intent,
                ..
            } => Some((observed, unbound_intent)),
            Self::BeforeEffect { .. } | Self::IssuedOrUncertain(_) => None,
        }
    }
}

impl RunnerScopeFreezeIssuedOrUncertainV8 {
    pub(crate) fn issue_was_uncertain(&self) -> bool {
        self.cause.is_some()
    }

    pub(crate) fn cause(&self) -> Option<&NativeSysErrorV8> {
        self.cause.as_ref()
    }

    pub(crate) fn intent_record_sha256(&self) -> Option<&str> {
        self.observed
            .runner_stop_intent
            .as_ref()
            .map(DurableRunnerStopIntentV8::intent_record_sha256)
    }
}

#[must_use = "freeze wait failure returns the freeze obligation"]
#[derive(Debug)]
pub(crate) struct RunnerScopeFreezeWaitFailureV8 {
    issued: RunnerScopeFreezeIssuedOrUncertainV8,
    cause: NativeSysErrorV8,
}

impl RunnerScopeFreezeWaitFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(crate) fn into_issued(self) -> RunnerScopeFreezeIssuedOrUncertainV8 {
        self.issued
    }
}

#[must_use = "freeze abort failure retains the freeze obligation"]
#[derive(Debug)]
pub(crate) enum RunnerScopeFreezeAbortFailureV8 {
    BeforeEffect {
        issued: RunnerScopeFreezeIssuedOrUncertainV8,
        cause: NativeSysErrorV8,
    },
    IssuedOrUncertain(RunnerScopeFreezeAbortIssuedOrUncertainV8),
}

#[must_use = "freeze-abort unfreeze write was issued or uncertain"]
#[derive(Debug)]
pub(crate) struct RunnerScopeFreezeAbortIssuedOrUncertainV8 {
    observed: ObservedRunnerScopeV8,
    cause: Option<NativeSysErrorV8>,
}

impl RunnerScopeFreezeAbortFailureV8 {
    pub(crate) fn cause(&self) -> Option<&NativeSysErrorV8> {
        match self {
            Self::BeforeEffect { cause, .. } => Some(cause),
            Self::IssuedOrUncertain(issued) => issued.cause(),
        }
    }

    pub(crate) fn into_before_effect_issued(self) -> Option<RunnerScopeFreezeIssuedOrUncertainV8> {
        match self {
            Self::BeforeEffect { issued, .. } => Some(issued),
            Self::IssuedOrUncertain(_) => None,
        }
    }

    pub(crate) fn into_issued(self) -> Option<RunnerScopeFreezeAbortIssuedOrUncertainV8> {
        match self {
            Self::BeforeEffect { .. } => None,
            Self::IssuedOrUncertain(issued) => Some(issued),
        }
    }
}

impl RunnerScopeFreezeAbortIssuedOrUncertainV8 {
    pub(crate) fn issue_was_uncertain(&self) -> bool {
        self.cause.is_some()
    }

    pub(crate) fn cause(&self) -> Option<&NativeSysErrorV8> {
        self.cause.as_ref()
    }
}

#[must_use = "freeze-abort wait failure retains explicit recovery ownership"]
#[derive(Debug)]
pub(crate) struct RunnerScopeFreezeAbortWaitFailureV8 {
    issued: RunnerScopeFreezeAbortIssuedOrUncertainV8,
    cause: NativeSysErrorV8,
}

impl RunnerScopeFreezeAbortWaitFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(crate) fn into_issued(self) -> RunnerScopeFreezeAbortIssuedOrUncertainV8 {
        self.issued
    }
}

#[must_use = "a frozen runner scope must be stopped then unfrozen or recovered"]
#[derive(Debug)]
pub(crate) struct FrozenRunnerScopeV8 {
    observed: ObservedRunnerScopeV8,
}

#[must_use = "STOP planning failure retains the frozen scope"]
#[derive(Debug)]
pub(crate) struct RunnerScopeStopBeforeEffectV8 {
    frozen: FrozenRunnerScopeV8,
    cause: NativeSysErrorV8,
}

impl RunnerScopeStopBeforeEffectV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub(crate) fn into_frozen(self) -> FrozenRunnerScopeV8 {
        self.frozen
    }
}

#[must_use = "SIGSTOP was issued or uncertain; unfreeze and restore explicitly"]
#[derive(Debug)]
pub(crate) struct RunnerScopeStopIssuedOrUncertainV8 {
    observed: ObservedRunnerScopeV8,
    cause: Option<NativeSysErrorV8>,
}

impl RunnerScopeStopIssuedOrUncertainV8 {
    pub(crate) fn issue_was_uncertain(&self) -> bool {
        self.cause.is_some()
    }

    pub(crate) fn cause(&self) -> Option<&NativeSysErrorV8> {
        self.cause.as_ref()
    }
}

#[must_use = "unfreeze failure retains STOP recovery ownership"]
#[derive(Debug)]
pub(crate) enum RunnerScopeUnfreezeFailureV8 {
    BeforeEffect {
        stopped: RunnerScopeStopIssuedOrUncertainV8,
        cause: NativeSysErrorV8,
    },
    IssuedOrUncertain(RunnerScopeUnfreezeIssuedOrUncertainV8),
}

#[must_use = "cgroup.freeze=0 was issued or uncertain after STOP"]
#[derive(Debug)]
pub(crate) struct RunnerScopeUnfreezeIssuedOrUncertainV8 {
    observed: ObservedRunnerScopeV8,
    stop_cause: Option<NativeSysErrorV8>,
    unfreeze_cause: Option<NativeSysErrorV8>,
}

impl RunnerScopeUnfreezeFailureV8 {
    pub(crate) fn cause(&self) -> Option<&NativeSysErrorV8> {
        match self {
            Self::BeforeEffect { cause, .. } => Some(cause),
            Self::IssuedOrUncertain(issued) => issued.unfreeze_cause.as_ref(),
        }
    }

    pub(crate) fn into_before_effect_stopped(self) -> Option<RunnerScopeStopIssuedOrUncertainV8> {
        match self {
            Self::BeforeEffect { stopped, .. } => Some(stopped),
            Self::IssuedOrUncertain(_) => None,
        }
    }

    pub(crate) fn into_issued(self) -> Option<RunnerScopeUnfreezeIssuedOrUncertainV8> {
        match self {
            Self::BeforeEffect { .. } => None,
            Self::IssuedOrUncertain(issued) => Some(issued),
        }
    }
}

impl RunnerScopeUnfreezeIssuedOrUncertainV8 {
    pub(crate) fn stop_issue_was_uncertain(&self) -> bool {
        self.stop_cause.is_some()
    }

    pub(crate) fn unfreeze_issue_was_uncertain(&self) -> bool {
        self.unfreeze_cause.is_some()
    }
}

#[must_use = "unfreeze/T verification failure retains the unique CONT recovery route"]
#[derive(Debug)]
pub(crate) struct RunnerScopeUnfreezeWaitFailureV8 {
    issued: RunnerScopeUnfreezeIssuedOrUncertainV8,
    cause: NativeSysErrorV8,
}

impl RunnerScopeUnfreezeWaitFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }
}

#[must_use = "verified stopped scope must be restored with its unique CONT token"]
#[derive(Debug)]
pub(crate) struct StoppedRunnerScopeV8 {
    observed: ObservedRunnerScopeV8,
    stopped: Vec<super::ProcessIdentityV8>,
}

impl StoppedRunnerScopeV8 {
    pub(crate) fn runner_stop_intent(&self) -> Option<&DurableRunnerStopIntentV8> {
        self.observed.runner_stop_intent.as_ref()
    }

    pub(crate) fn capture_runner_stop_observation_v8(
        &self,
    ) -> NativeSysResultV8<RunnerStopEffectEvidenceV8> {
        let intent = self.observed.runner_stop_intent.as_ref().ok_or_else(|| {
            NativeSysErrorV8::InvalidInput(
                "stopped runner scope lacks its durable STOP intent".to_string(),
            )
        })?;
        capture_runner_stop_evidence_impl_v8(
            &self.observed,
            RunnerStopEvidencePhaseV8::StoppedObservation,
            Some(intent.effect_manifest_sha256()),
            Some(intent.intent_record_sha256()),
        )
    }
}

/// Exact stopped closure whose RunnerStop intent has been closed by one
/// durable EffectObserved record. Only this type can enter the ordinary CONT
/// planner; raw stopped/recovery tokens cannot bypass durable replay.
#[must_use = "durably observed stopped scope retains the unique ordinary CONT route"]
#[derive(Debug)]
pub(crate) struct DurablyObservedStoppedRunnerScopeV8 {
    stopped: StoppedRunnerScopeV8,
    observation_record_sha256: String,
}

impl DurablyObservedStoppedRunnerScopeV8 {
    pub(crate) fn bind_durable_observation(
        mut stopped: StoppedRunnerScopeV8,
        expected_intent_record_sha256: &str,
        observation_record_sha256: String,
    ) -> Result<Self, StoppedRunnerScopeV8> {
        let matches = stopped
            .observed
            .runner_stop_intent
            .as_ref()
            .is_some_and(|intent| intent.intent_record_sha256() == expected_intent_record_sha256);
        if !matches {
            return Err(stopped);
        }
        stopped.observed.runner_stop_intent = None;
        Ok(Self {
            stopped,
            observation_record_sha256,
        })
    }

    pub(crate) fn observation_record_sha256(&self) -> &str {
        &self.observation_record_sha256
    }

    pub(crate) fn capture_runner_restore_manifest_v8(
        &self,
    ) -> NativeSysResultV8<RunnerRestoreEffectEvidenceV8> {
        capture_runner_restore_evidence_impl_v8(
            &self.stopped.observed,
            RunnerRestoreEvidencePhaseV8::StoppedPreEffect,
            &self.observation_record_sha256,
            None,
            None,
        )
    }
}

/// Exact stopped closure plus the durable RunnerRestore intent that alone may
/// enter the ordinary SIGCONT planner.
#[must_use = "restore-authorized stopped scope retains the only ordinary CONT route"]
#[derive(Debug)]
pub(crate) struct RestoreAuthorizedStoppedRunnerScopeV8 {
    stopped: DurablyObservedStoppedRunnerScopeV8,
    restore_intent: DurableRunnerRestoreIntentV8,
}

impl RestoreAuthorizedStoppedRunnerScopeV8 {
    pub(crate) fn bind_durable_intent(
        stopped: DurablyObservedStoppedRunnerScopeV8,
        expected_stopped_observation_record_sha256: &str,
        restore_intent: DurableRunnerRestoreIntentV8,
    ) -> Result<
        Self,
        (
            DurablyObservedStoppedRunnerScopeV8,
            DurableRunnerRestoreIntentV8,
        ),
    > {
        let current_manifest = stopped.capture_runner_restore_manifest_v8();
        let binding_matches = current_manifest
            .map_err(|_| ())
            .and_then(|current| {
                RunnerRestoreEffectEvidenceV8::decode_exact(restore_intent.effect_manifest_bytes())
                    .map_err(|_| ())
                    .and_then(|archived| {
                        Ok(
                            archived.phase == RunnerRestoreEvidencePhaseV8::StoppedPreEffect
                                && stopped.observation_record_sha256()
                                    == expected_stopped_observation_record_sha256
                                && restore_intent.stopped_observation_record_sha256()
                                    == expected_stopped_observation_record_sha256
                                && archived.stopped_observation_record_sha256
                                    == expected_stopped_observation_record_sha256
                                && archived.sha256().map_err(|_| ())?
                                    == restore_intent.effect_manifest_sha256()
                                && archived.scope_binding_bytes().map_err(|_| ())?
                                    == restore_intent.scope_binding_bytes()
                                && current.scope_binding_bytes().map_err(|_| ())?
                                    == restore_intent.scope_binding_bytes(),
                        )
                    })
            })
            .unwrap_or(false);
        if !binding_matches {
            return Err((stopped, restore_intent));
        }
        Ok(Self {
            stopped,
            restore_intent,
        })
    }

    pub(crate) fn restore_intent(&self) -> &DurableRunnerRestoreIntentV8 {
        &self.restore_intent
    }
}

#[must_use = "CONT plan is single-use and owns all pidfds"]
#[derive(Debug)]
pub(crate) struct RunnerScopeContinuePlanV8 {
    observed: ObservedRunnerScopeV8,
    before: Vec<super::ProcessIdentityV8>,
    recovery: bool,
    runner_restore_intent: Option<DurableRunnerRestoreIntentV8>,
    runner_stop_observation_record_sha256: Option<String>,
}

#[must_use = "CONT planning failure retains the stopped/recovery token"]
#[derive(Debug)]
pub(crate) enum RunnerScopeContinuePlanFailureV8 {
    Stopped {
        stopped: RestoreAuthorizedStoppedRunnerScopeV8,
        cause: NativeSysErrorV8,
    },
    Recovery {
        failure: RunnerScopeUnfreezeWaitFailureV8,
        cause: NativeSysErrorV8,
    },
}

impl RunnerScopeContinuePlanFailureV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        match self {
            Self::Stopped { cause, .. } | Self::Recovery { cause, .. } => cause,
        }
    }
}

#[must_use = "CONT issue/final-proof failure is quarantine evidence, never a resend token"]
#[derive(Debug)]
pub(crate) struct RunnerScopePostContinueQuarantineV8 {
    observed: ObservedRunnerScopeV8,
    before: Vec<super::ProcessIdentityV8>,
    runner_restore_intent: Option<DurableRunnerRestoreIntentV8>,
    runner_stop_observation_record_sha256: Option<String>,
    cause: NativeSysErrorV8,
}

impl RunnerScopePostContinueQuarantineV8 {
    pub(crate) fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn revalidate_for_quarantine(
        &self,
    ) -> NativeSysResultV8<Vec<super::ProcessIdentityV8>> {
        let (identities, _) =
            revalidate_observed_runner_scope_fast_v8(&self.observed, Some(false))?;
        Ok(identities)
    }

    pub(crate) fn before(&self) -> &[super::ProcessIdentityV8] {
        &self.before
    }

    pub(crate) fn runner_restore_intent(&self) -> Option<&DurableRunnerRestoreIntentV8> {
        self.runner_restore_intent.as_ref()
    }
}

#[must_use = "runner CONT evidence must be recorded by its caller"]
#[derive(Debug)]
pub(crate) struct RunnerScopeContinueExecutionV8 {
    observed: ObservedRunnerScopeV8,
    pub(crate) process_group_id: u32,
    pub(crate) before: Vec<super::ProcessIdentityV8>,
    pub(crate) after: Vec<super::ProcessIdentityV8>,
    pub(crate) recovery: bool,
    runner_restore_intent: Option<DurableRunnerRestoreIntentV8>,
    runner_stop_observation_record_sha256: Option<String>,
    unresolved_runner_stop_intent: Option<DurableRunnerStopIntentV8>,
}

impl RunnerScopeContinueExecutionV8 {
    pub(crate) fn unresolved_runner_stop_intent(&self) -> Option<&DurableRunnerStopIntentV8> {
        self.unresolved_runner_stop_intent.as_ref()
    }

    pub(crate) fn runner_restore_intent(&self) -> Option<&DurableRunnerRestoreIntentV8> {
        self.runner_restore_intent.as_ref()
    }

    pub(crate) fn capture_runner_restore_observation_v8(
        &self,
    ) -> NativeSysResultV8<RunnerRestoreEffectEvidenceV8> {
        let intent = self.runner_restore_intent.as_ref().ok_or_else(|| {
            NativeSysErrorV8::InvalidInput(
                "post-CONT execution lacks a durable RESTORE intent".to_string(),
            )
        })?;
        let stopped_observation_record_sha256 = self
            .runner_stop_observation_record_sha256
            .as_deref()
            .ok_or_else(|| {
                NativeSysErrorV8::InvalidInput(
                    "post-CONT execution lacks its runner STOP observation binding".to_string(),
                )
            })?;
        capture_runner_restore_evidence_impl_v8(
            &self.observed,
            RunnerRestoreEvidencePhaseV8::RunningObservation,
            stopped_observation_record_sha256,
            Some(intent.effect_manifest_sha256()),
            Some(intent.intent_record_sha256()),
        )
    }
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExistingScopeBackendV8 {
    Production,
    #[cfg(test)]
    ExplicitTest,
}

impl ExistingSystemdScopeCgroupV8 {
    pub(crate) fn control_group(&self) -> &str {
        &self.control_group
    }

    #[cfg(target_os = "linux")]
    fn revalidate(&self) -> NativeSysResultV8<()> {
        revalidate_existing_systemd_scope_v8(self)
    }

    #[cfg(target_os = "linux")]
    fn roster_and_events(&self) -> NativeSysResultV8<(Vec<u32>, CgroupEventsV8)> {
        self.revalidate()?;
        let pids = parse_cgroup_procs_v8(&read_existing_scope_control_v8(self, "cgroup.procs")?)?;
        let events =
            parse_cgroup_events_v8(&read_existing_scope_control_v8(self, "cgroup.events")?)?;
        self.revalidate()?;
        Ok((pids, events))
    }
}

/// Exact root-owned nondelegated cgroup-v2 leaf. It is not a run authority;
/// callers must provide separate durable admission and operator authority.
#[derive(Debug)]
pub struct CgroupV2LeafV8 {
    #[cfg(target_os = "linux")]
    root: Arc<CgroupRootInnerV8>,
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    #[cfg(target_os = "linux")]
    identity: FileIdentityV8,
    leaf: String,
    absolute_path: String,
}

impl CgroupV2LeafV8 {
    pub fn leaf(&self) -> &str {
        &self.leaf
    }

    pub fn absolute_path(&self) -> &str {
        &self.absolute_path
    }

    pub fn revalidate_empty(&self) -> NativeSysResultV8<CgroupEmptyProofV8> {
        revalidate_empty_leaf_impl_v8(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgroupEmptyProofV8 {
    leaf: String,
    events: CgroupEventsV8,
    parent_device: u64,
    parent_inode: u64,
    device: u64,
    inode: u64,
    owner_uid: u32,
    owner_gid: u32,
    delegated_controller_count: u32,
    observed_process_count: u32,
    observation_sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgroupLeafAbsenceProofV8 {
    absolute_path: String,
    parent_device: u64,
    parent_inode: u64,
    removed_device: u64,
    removed_inode: u64,
    child_link_count_after_cleanup: u64,
    name_absent: bool,
    observation_sha256: [u8; 32],
}

impl CgroupLeafAbsenceProofV8 {
    pub fn absolute_path(&self) -> &str {
        &self.absolute_path
    }

    pub fn removed_device(&self) -> u64 {
        self.removed_device
    }

    pub fn removed_inode(&self) -> u64 {
        self.removed_inode
    }

    pub fn parent_device(&self) -> u64 {
        self.parent_device
    }

    pub fn parent_inode(&self) -> u64 {
        self.parent_inode
    }

    pub fn child_link_count_after_cleanup(&self) -> u64 {
        self.child_link_count_after_cleanup
    }

    pub fn name_absent(&self) -> bool {
        self.name_absent
    }

    pub fn observation_sha256(&self) -> [u8; 32] {
        self.observation_sha256
    }
}

impl CgroupEmptyProofV8 {
    pub fn leaf(&self) -> &str {
        &self.leaf
    }

    pub fn events(&self) -> CgroupEventsV8 {
        self.events
    }

    pub fn parent_device(&self) -> u64 {
        self.parent_device
    }

    pub fn parent_inode(&self) -> u64 {
        self.parent_inode
    }

    pub fn child_device(&self) -> u64 {
        self.device
    }

    pub fn child_inode(&self) -> u64 {
        self.inode
    }

    pub fn owner_uid(&self) -> u32 {
        self.owner_uid
    }

    pub fn owner_gid(&self) -> u32 {
        self.owner_gid
    }

    pub fn delegated_controller_count(&self) -> u32 {
        self.delegated_controller_count
    }

    pub fn observed_process_count(&self) -> u32 {
        self.observed_process_count
    }

    pub fn observation_sha256(&self) -> [u8; 32] {
        self.observation_sha256
    }
}

/// Probes only `/sys/fs/cgroup`, verifies the cgroup2 superblock and fixed
/// root ownership/mode, and parses exact type/subtree-control evidence.
pub fn probe_fixed_cgroup_v2_root_v8() -> NativeSysResultV8<CgroupV2RootV8> {
    probe_fixed_cgroup_v2_root_impl_v8()
}

pub(crate) fn open_existing_systemd_scope_cgroup_v8(
    control_group: &str,
) -> NativeSysResultV8<ExistingSystemdScopeCgroupV8> {
    open_existing_systemd_scope_impl_v8(control_group)
}

pub(crate) fn observe_runner_scope_v8(
    manifest: RunnerScopeTrustedManifestV8,
) -> NativeSysResultV8<ObservedRunnerScopeV8> {
    observe_runner_scope_impl_v8(manifest)
}

pub(crate) fn issue_runner_scope_freeze_v8(
    mut observed: ObservedRunnerScopeV8,
    intent: DurableRunnerStopIntentV8,
) -> Result<RunnerScopeFreezeIssuedOrUncertainV8, RunnerScopeFreezeFailureV8> {
    if observed.runner_stop_intent.is_some() {
        return Err(RunnerScopeFreezeFailureV8::IntentConflict {
            observed,
            unbound_intent: intent,
            cause: NativeSysErrorV8::InvalidInput(
                "runner scope already owns a different durable STOP obligation".to_string(),
            ),
        });
    }
    let current_manifest = match observed.capture_runner_stop_manifest_v8() {
        Ok(manifest) => manifest,
        Err(cause) => {
            return Err(RunnerScopeFreezeFailureV8::IntentConflict {
                observed,
                unbound_intent: intent,
                cause,
            });
        }
    };
    let binding_matches = RunnerStopEffectEvidenceV8::decode_exact(intent.effect_manifest_bytes())
        .and_then(|archived| {
            Ok(archived.phase == RunnerStopEvidencePhaseV8::PreEffect
                && archived.sha256()? == intent.effect_manifest_sha256()
                && archived.scope_binding_bytes()? == intent.scope_binding_bytes()
                && current_manifest.scope_binding_bytes()? == intent.scope_binding_bytes())
        })
        .unwrap_or(false);
    if !binding_matches {
        return Err(RunnerScopeFreezeFailureV8::IntentConflict {
            observed,
            unbound_intent: intent,
            cause: NativeSysErrorV8::IdentityMismatch(
                "runner scope differs from the exact durable STOP manifest binding".to_string(),
            ),
        });
    }
    observed.runner_stop_intent = Some(intent);
    issue_runner_scope_freeze_impl_v8(observed)
}

pub(crate) fn retry_runner_scope_freeze_v8(
    observed: ObservedRunnerScopeV8,
) -> Result<RunnerScopeFreezeIssuedOrUncertainV8, RunnerScopeFreezeFailureV8> {
    if observed.runner_stop_intent.is_none() {
        return Err(RunnerScopeFreezeFailureV8::BeforeEffect {
            observed,
            cause: NativeSysErrorV8::InvalidInput(
                "runner freeze retry lacks its durable STOP obligation".to_string(),
            ),
        });
    }
    issue_runner_scope_freeze_impl_v8(observed)
}

pub(crate) fn wait_runner_scope_frozen_v8(
    issued: RunnerScopeFreezeIssuedOrUncertainV8,
    timeout: Duration,
) -> Result<FrozenRunnerScopeV8, RunnerScopeFreezeWaitFailureV8> {
    wait_runner_scope_frozen_impl_v8(issued, timeout)
}

pub(crate) fn abort_runner_scope_freeze_v8(
    issued: RunnerScopeFreezeIssuedOrUncertainV8,
) -> Result<RunnerScopeFreezeAbortIssuedOrUncertainV8, RunnerScopeFreezeAbortFailureV8> {
    abort_runner_scope_freeze_impl_v8(issued)
}

pub(crate) fn wait_runner_scope_freeze_aborted_v8(
    issued: RunnerScopeFreezeAbortIssuedOrUncertainV8,
    timeout: Duration,
) -> Result<ObservedRunnerScopeV8, RunnerScopeFreezeAbortWaitFailureV8> {
    wait_runner_scope_freeze_aborted_impl_v8(issued, timeout)
}

pub(crate) fn issue_runner_scope_stop_v8(
    frozen: FrozenRunnerScopeV8,
) -> Result<RunnerScopeStopIssuedOrUncertainV8, RunnerScopeStopBeforeEffectV8> {
    issue_runner_scope_stop_impl_v8(frozen)
}

pub(crate) fn issue_runner_scope_unfreeze_v8(
    stopped: RunnerScopeStopIssuedOrUncertainV8,
) -> Result<RunnerScopeUnfreezeIssuedOrUncertainV8, RunnerScopeUnfreezeFailureV8> {
    issue_runner_scope_unfreeze_impl_v8(stopped)
}

pub(crate) fn wait_runner_scope_unfrozen_stopped_v8(
    issued: RunnerScopeUnfreezeIssuedOrUncertainV8,
    timeout: Duration,
) -> Result<StoppedRunnerScopeV8, RunnerScopeUnfreezeWaitFailureV8> {
    wait_runner_scope_unfrozen_stopped_impl_v8(issued, timeout)
}

pub(crate) fn plan_runner_scope_continue_v8(
    stopped: RestoreAuthorizedStoppedRunnerScopeV8,
) -> Result<RunnerScopeContinuePlanV8, RunnerScopeContinuePlanFailureV8> {
    plan_runner_scope_continue_impl_v8(stopped)
}

pub(crate) fn plan_runner_scope_recovery_continue_v8(
    failure: RunnerScopeUnfreezeWaitFailureV8,
) -> Result<RunnerScopeContinuePlanV8, RunnerScopeContinuePlanFailureV8> {
    plan_runner_scope_recovery_continue_impl_v8(failure)
}

pub(crate) fn execute_runner_scope_continue_v8(
    plan: RunnerScopeContinuePlanV8,
) -> Result<RunnerScopeContinueExecutionV8, RunnerScopePostContinueQuarantineV8> {
    execute_runner_scope_continue_impl_v8(plan)
}

/// Test-only post-effect fault point. It proves that an already issued CONT
/// becomes quarantine evidence and never another signal authority.
#[cfg(all(test, target_os = "linux"))]
pub(super) fn execute_runner_scope_continue_after_issue_fault_for_test_v8(
    plan: RunnerScopeContinuePlanV8,
) -> Result<RunnerScopeContinueExecutionV8, RunnerScopePostContinueQuarantineV8> {
    execute_runner_scope_continue_core_v8(plan, RunnerContinueFaultV8::AfterIssue)
}

/// Test-only partial STOP fault point. It sends SIGSTOP to exactly one
/// pidfd-bound member after the frozen full-group closure has been revalidated,
/// then returns the same issued/uncertain recovery token used for an ambiguous
/// process-group signal outcome. This exercises mixed T/non-T recovery without
/// granting any raw-PID production API.
#[cfg(all(test, target_os = "linux"))]
pub(super) fn issue_runner_scope_partial_stop_fault_for_test_v8(
    frozen: FrozenRunnerScopeV8,
) -> Result<RunnerScopeStopIssuedOrUncertainV8, RunnerScopeStopBeforeEffectV8> {
    issue_runner_scope_stop_core_v8(frozen, RunnerStopFaultV8::PartialMember)
}

/// Test-only semantic seam. Kernel lifecycle tests do not construct a durable
/// state root; this consumes the test intent only after the exact stopped
/// closure exists, so the production planner still has no raw bypass.
#[cfg(all(test, target_os = "linux"))]
pub(super) fn mark_runner_stop_observed_for_test_v8(
    stopped: StoppedRunnerScopeV8,
) -> NativeSysResultV8<DurablyObservedStoppedRunnerScopeV8> {
    let intent_record_sha256 = stopped
        .observed
        .runner_stop_intent
        .as_ref()
        .ok_or_else(|| {
            NativeSysErrorV8::InvalidInput(
                "runner stopped fixture lacks a durable STOP intent".to_string(),
            )
        })?
        .intent_record_sha256()
        .to_string();
    DurablyObservedStoppedRunnerScopeV8::bind_durable_observation(
        stopped,
        &intent_record_sha256,
        "5".repeat(64),
    )
    .map_err(|_| {
        NativeSysErrorV8::IdentityMismatch(
            "runner stopped fixture lost its durable STOP intent".to_string(),
        )
    })
}

/// Test-only seam that models an already durable RunnerRestore intent without
/// granting production code a way around the journal publication boundary.
#[cfg(all(test, target_os = "linux"))]
pub(super) fn mark_runner_restore_intent_for_test_v8(
    stopped: DurablyObservedStoppedRunnerScopeV8,
) -> NativeSysResultV8<RestoreAuthorizedStoppedRunnerScopeV8> {
    let expected_stopped_observation_record_sha256 =
        stopped.observation_record_sha256().to_string();
    let intent = DurableRunnerRestoreIntentV8::test_only_for_stopped(&stopped)?;
    RestoreAuthorizedStoppedRunnerScopeV8::bind_durable_intent(
        stopped,
        &expected_stopped_observation_record_sha256,
        intent,
    )
    .map_err(|_| {
        NativeSysErrorV8::IdentityMismatch(
            "runner stopped fixture lost its durable RESTORE binding".to_string(),
        )
    })
}

/// Consumes the parent proof into one process-bound admission namespace lease.
/// Acquisition is read-only and refuses any pre-existing `hepta-v8-*` child,
/// so an abandoned prior obligation blocks a new candidate rather than being
/// silently overwritten.
pub(crate) fn acquire_candidate_cgroup_namespace_lease_v8(
    root: CgroupV2RootV8,
) -> NativeSysResultV8<CandidateCgroupNamespaceLeaseV8> {
    acquire_candidate_namespace_lease_impl_v8(root)
}

/// Creates the one exact attempt-derived child.  Every result owns the lease:
/// a pre-mkdir failure can retry, while any successful mkdir returns either a
/// bound child or an explicit issued/uncertain cleanup obligation.
pub(crate) fn create_candidate_cgroup_leaf_v8(
    lease: CandidateCgroupNamespaceLeaseV8,
    attempt_sha256: [u8; 32],
) -> Result<CandidateCgroupLeafV8, CandidateCgroupCreateFailureV8> {
    create_candidate_leaf_impl_v8(lease, attempt_sha256)
}

/// Test-only post-mkdir fault point. The successful mkdir and retained child
/// descriptor must emerge solely as an issued/uncertain recovery obligation.
#[cfg(all(test, target_os = "linux"))]
pub(super) fn create_candidate_cgroup_leaf_after_bind_fault_for_test_v8(
    lease: CandidateCgroupNamespaceLeaseV8,
    attempt_sha256: [u8; 32],
) -> Result<CandidateCgroupLeafV8, CandidateCgroupCreateFailureV8> {
    create_candidate_leaf_core_v8(lease, attempt_sha256, CandidateCreateFaultV8::AfterBind)
}

/// Rebinds a post-mkdir create obligation through its retained lease and exact
/// attempt-derived name. No name-only unlink is ever attempted; failure keeps
/// the obligation intact for retry/quarantine.
pub(crate) fn recover_candidate_cgroup_create_v8(
    obligation: CandidateCgroupCreateIssuedOrUncertainV8,
) -> Result<CandidateCgroupLeafV8, CandidateCgroupCreateRecoveryFailureV8> {
    recover_candidate_create_impl_v8(obligation)
}

/// Converts an exact, still-empty candidate into the ordinary descriptor-bound
/// cleanup chain. This is the only pre-clone abandon path.
pub(crate) fn prepare_candidate_cgroup_empty_for_cleanup_v8(
    candidate: CandidateCgroupLeafV8,
) -> Result<CandidateCgroupEmptyV8, CandidateCgroupPrepareCleanupFailureV8> {
    prepare_candidate_empty_for_cleanup_impl_v8(candidate)
}

/// Creates a child with clone3(CLONE_INTO_CGROUP|CLONE_PIDFD).  The child
/// blocks on an inherited pipe before returning to caller code; there is no
/// fork+numeric-attach fallback on ENOSYS, EINVAL, EPERM, or any other error.
pub(crate) fn clone_candidate_into_cgroup_with_barrier_v8(
    candidate: CandidateCgroupLeafV8,
) -> Result<CandidateCgroupCloneOutcomeV8, CandidateCgroupCloneFailureV8> {
    clone_candidate_with_barrier_impl_v8(candidate)
}

/// Releases exactly one already-contained child.  Once write(2) is attempted,
/// every failure retains an issued/uncertain token because the byte may have
/// made the child runnable.
pub(crate) fn release_candidate_cgroup_start_barrier_v8(
    barrier: CandidateCgroupStartBarrierV8,
) -> Result<CandidateCgroupRunningV8, CandidateCgroupReleaseIssuedOrUncertainV8> {
    release_candidate_start_barrier_impl_v8(barrier)
}

/// Explicit regular-file fixture adapter. It is compiled only into Linux
/// tests, never provides a production numeric-attach fallback, and retains a
/// consumptive obligation if either simulated control write may have landed.
#[cfg(all(test, target_os = "linux"))]
pub(super) fn populate_candidate_cgroup_for_test_v8(
    candidate: CandidateCgroupLeafV8,
    process: &ProcessObservationV8,
) -> Result<CandidateCgroupRunningV8, CandidateCgroupTestPopulationFailureV8> {
    populate_candidate_for_test_impl_v8(candidate, process)
}

pub(crate) fn issue_candidate_cgroup_kill_v8(
    running: CandidateCgroupRunningV8,
) -> Result<CandidateCgroupKillIssuedOrUncertainV8, CandidateCgroupKillFailureV8> {
    issue_candidate_kill_impl_v8(CandidateCgroupPopulationV8 {
        child_pid: running.child_pid,
        verified_child: running.child,
        #[cfg(target_os = "linux")]
        raw_pidfd: None,
        #[cfg(target_os = "linux")]
        blocked_release_guard: None,
        candidate: running.candidate,
    })
}

pub(crate) fn recover_candidate_clone_by_kill_v8(
    obligation: CandidateCgroupCloneIssuedOrUncertainV8,
) -> Result<CandidateCgroupKillIssuedOrUncertainV8, CandidateCgroupKillFailureV8> {
    issue_candidate_kill_impl_v8(CandidateCgroupPopulationV8 {
        candidate: obligation.candidate,
        child_pid: obligation.child_pid,
        verified_child: obligation.verified_child,
        #[cfg(target_os = "linux")]
        raw_pidfd: obligation.raw_pidfd,
        #[cfg(target_os = "linux")]
        blocked_release_guard: Some(obligation.release),
    })
}

pub(crate) fn recover_candidate_release_by_kill_v8(
    obligation: CandidateCgroupReleaseIssuedOrUncertainV8,
) -> Result<CandidateCgroupKillIssuedOrUncertainV8, CandidateCgroupKillFailureV8> {
    let CandidateCgroupStartBarrierV8 {
        candidate,
        child,
        #[cfg(target_os = "linux")]
        release,
    } = obligation.barrier;
    issue_candidate_kill_impl_v8(CandidateCgroupPopulationV8 {
        child_pid: child.pid(),
        verified_child: Some(child),
        #[cfg(target_os = "linux")]
        raw_pidfd: None,
        #[cfg(target_os = "linux")]
        blocked_release_guard: Some(release),
        candidate,
    })
}

pub(crate) fn wait_candidate_cgroup_empty_v8(
    issued: CandidateCgroupKillIssuedOrUncertainV8,
    timeout: Duration,
) -> Result<CandidateCgroupEmptyV8, CandidateCgroupWaitFailureV8> {
    wait_candidate_empty_impl_v8(issued, timeout)
}

pub(crate) fn cleanup_candidate_cgroup_v8(
    empty: CandidateCgroupEmptyV8,
) -> Result<CandidateCgroupTerminalV8, CandidateCgroupCleanupFailureV8> {
    cleanup_candidate_impl_v8(empty)
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
struct CgroupRootInnerV8 {
    descriptor: OwnedFd,
    identity: FileIdentityV8,
    reopen_path: PathBuf,
    backend: CgroupBackendV8,
    production_anchor: Option<CgroupProductionAnchorV8>,
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
struct CgroupProductionAnchorV8 {
    descriptor: OwnedFd,
    identity: FileIdentityV8,
    relative_parent: String,
    observer: super::ProcfsObserverBindingV8,
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CgroupBackendV8 {
    Production,
    #[cfg(test)]
    ExplicitTest,
}

#[cfg(target_os = "linux")]
fn probe_fixed_cgroup_v2_root_impl_v8() -> NativeSysResultV8<CgroupV2RootV8> {
    let inner = open_production_cgroup_parent_v8()?;
    build_cgroup_root_proof_v8(inner)
}

#[cfg(not(target_os = "linux"))]
fn probe_fixed_cgroup_v2_root_impl_v8() -> NativeSysResultV8<CgroupV2RootV8> {
    Err(unsupported("probe fixed cgroup-v2 root"))
}

#[cfg(target_os = "linux")]
fn build_cgroup_root_proof_v8(inner: CgroupRootInnerV8) -> NativeSysResultV8<CgroupV2RootV8> {
    let inner = Arc::new(inner);
    revalidate_root_inner_v8(&inner)?;
    let cgroup_type = parse_cgroup_type_v8(&read_control_v8(&inner, "cgroup.type")?)?;
    if cgroup_type != CgroupTypeV8::Domain {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "cgroup-v2 parent must be exact domain type".to_string(),
        ));
    }
    let controllers = parse_subtree_control_v8(&read_control_v8(&inner, "cgroup.controllers")?)?;
    let subtree_control =
        parse_subtree_control_v8(&read_control_v8(&inner, "cgroup.subtree_control")?)?;
    verify_control_file_policy_v8(&inner, "cgroup.type")?;
    verify_control_file_policy_v8(&inner, "cgroup.controllers")?;
    verify_control_file_policy_v8(&inner, "cgroup.subtree_control")?;
    Ok(CgroupV2RootV8 {
        delegated_parent_path: inner.reopen_path.to_string_lossy().into_owned(),
        inner,
        cgroup_type,
        controllers,
        subtree_control,
    })
}

#[cfg(target_os = "linux")]
fn revalidate_cgroup_root_impl_v8(root: &CgroupV2RootV8) -> NativeSysResultV8<()> {
    revalidate_root_inner_v8(&root.inner)?;
    let observed_type = parse_cgroup_type_v8(&read_control_v8(&root.inner, "cgroup.type")?)?;
    let observed_controllers =
        parse_subtree_control_v8(&read_control_v8(&root.inner, "cgroup.controllers")?)?;
    let observed_subtree =
        parse_subtree_control_v8(&read_control_v8(&root.inner, "cgroup.subtree_control")?)?;
    if observed_type != root.cgroup_type
        || observed_controllers != root.controllers
        || observed_subtree != root.subtree_control
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "cgroup-v2 delegated parent type/controllers/subtree_control changed after probe"
                .to_string(),
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn revalidate_cgroup_root_impl_v8(_root: &CgroupV2RootV8) -> NativeSysResultV8<()> {
    Err(unsupported("revalidate cgroup-v2 root"))
}

#[cfg(target_os = "linux")]
fn acquire_candidate_namespace_lease_impl_v8(
    root: CgroupV2RootV8,
) -> NativeSysResultV8<CandidateCgroupNamespaceLeaseV8> {
    root.revalidate()?;
    let existing = candidate_child_names_v8(&root.inner)?;
    if !existing.is_empty() {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "candidate namespace is not empty; retained obligations: {}",
            existing.join(",")
        )));
    }
    // SAFETY: getpid has no pointer arguments or preconditions.
    let owner_pid = unsafe { libc::getpid() };
    let owner_pid = u32::try_from(owner_pid).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch(
            "candidate namespace lease owner pid is not positive".to_string(),
        )
    })?;
    root.revalidate()?;
    Ok(CandidateCgroupNamespaceLeaseV8 {
        root,
        owner_pid,
        not_sync: PhantomData,
    })
}

#[cfg(not(target_os = "linux"))]
fn acquire_candidate_namespace_lease_impl_v8(
    _root: CgroupV2RootV8,
) -> NativeSysResultV8<CandidateCgroupNamespaceLeaseV8> {
    Err(unsupported("acquire candidate cgroup namespace lease"))
}

#[cfg(target_os = "linux")]
fn create_candidate_leaf_impl_v8(
    lease: CandidateCgroupNamespaceLeaseV8,
    attempt_sha256: [u8; 32],
) -> Result<CandidateCgroupLeafV8, CandidateCgroupCreateFailureV8> {
    create_candidate_leaf_core_v8(lease, attempt_sha256, CandidateCreateFaultV8::None)
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CandidateCreateFaultV8 {
    None,
    #[cfg(test)]
    AfterBind,
}

#[cfg(target_os = "linux")]
fn create_candidate_leaf_core_v8(
    lease: CandidateCgroupNamespaceLeaseV8,
    attempt_sha256: [u8; 32],
    fault: CandidateCreateFaultV8,
) -> Result<CandidateCgroupLeafV8, CandidateCgroupCreateFailureV8> {
    #[cfg(not(test))]
    let _ = fault;
    let before = |lease, cause| CandidateCgroupCreateFailureV8::BeforeEffect { lease, cause };
    if attempt_sha256 == [0; 32] {
        return Err(before(
            lease,
            NativeSysErrorV8::InvalidInput(
                "candidate attempt digest must not be the all-zero sentinel".to_string(),
            ),
        ));
    }
    if let Err(cause) = lease.revalidate() {
        return Err(before(lease, cause));
    }
    match candidate_child_names_v8(&lease.root.inner) {
        Ok(existing) if existing.is_empty() => {}
        Ok(existing) => {
            return Err(before(
                lease,
                NativeSysErrorV8::IdentityMismatch(format!(
                    "candidate namespace already contains: {}",
                    existing.join(",")
                )),
            ));
        }
        Err(cause) => return Err(before(lease, cause)),
    }
    let leaf_name = format!("hepta-v8-{}", lowercase_hex_v8(&attempt_sha256));
    if let Err(cause) = validate_cgroup_leaf_v8(&leaf_name) {
        return Err(before(lease, cause));
    }
    let leaf_c = match CString::new(leaf_name.as_str()) {
        Ok(value) => value,
        Err(_) => {
            return Err(before(
                lease,
                NativeSysErrorV8::InvalidInput("candidate cgroup leaf contains NUL".to_string()),
            ));
        }
    };
    // SAFETY: the lease's parent descriptor and leaf C string remain live;
    // mkdirat retains neither.  A syscall error is a before-effect outcome;
    // every successful return below owns the post-mkdir namespace/name.
    if unsafe {
        libc::mkdirat(
            lease.root.inner.descriptor.as_raw_fd(),
            leaf_c.as_ptr(),
            0o755,
        )
    } != 0
    {
        return Err(before(
            lease,
            io_error(
                "mkdirat exclusive attempt-derived cgroup-v2 child",
                std::io::Error::last_os_error(),
            ),
        ));
    }

    let descriptor = match open_beneath_v8(
        lease.root.inner.descriptor.as_raw_fd(),
        &leaf_name,
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
    ) {
        Ok(descriptor) => descriptor,
        Err(cause) => {
            return Err(CandidateCgroupCreateFailureV8::IssuedOrUncertain(
                CandidateCgroupCreateIssuedOrUncertainV8 {
                    lease,
                    leaf_name,
                    leaf: None,
                    unvalidated_descriptor: None,
                    cause,
                },
            ));
        }
    };
    let identity = match super::openat2::identity_for_fd(descriptor.as_raw_fd()) {
        Ok(identity) => identity,
        Err(cause) => {
            return Err(CandidateCgroupCreateFailureV8::IssuedOrUncertain(
                CandidateCgroupCreateIssuedOrUncertainV8 {
                    lease,
                    leaf_name,
                    leaf: None,
                    unvalidated_descriptor: Some(descriptor),
                    cause,
                },
            ));
        }
    };
    let leaf = CgroupV2LeafV8 {
        absolute_path: format!(
            "{}/{}",
            lease.root.delegated_parent_path.trim_end_matches('/'),
            leaf_name
        ),
        root: Arc::clone(&lease.root.inner),
        descriptor,
        identity,
        leaf: leaf_name.clone(),
    };

    #[cfg(test)]
    if lease.root.inner.backend == CgroupBackendV8::ExplicitTest
        && let Err(cause) = materialize_test_cgroup_files_v8(&lease.root.inner, &leaf_name)
    {
        return Err(CandidateCgroupCreateFailureV8::IssuedOrUncertain(
            CandidateCgroupCreateIssuedOrUncertainV8 {
                lease,
                leaf_name,
                leaf: Some(leaf),
                unvalidated_descriptor: None,
                cause,
            },
        ));
    }

    #[cfg(test)]
    if fault == CandidateCreateFaultV8::AfterBind {
        return Err(CandidateCgroupCreateFailureV8::IssuedOrUncertain(
            CandidateCgroupCreateIssuedOrUncertainV8 {
                lease,
                leaf_name,
                leaf: Some(leaf),
                unvalidated_descriptor: None,
                cause: NativeSysErrorV8::RaceDetected(
                    "injected post-mkdir candidate bind fault".to_string(),
                ),
            },
        ));
    }

    if let Err(cause) =
        require_cgroup_directory_policy_v8(&lease.root.inner, leaf.descriptor.as_raw_fd(), identity)
            .and_then(|()| revalidate_empty_leaf_impl_v8(&leaf).map(|_| ()))
            .and_then(|()| lease.revalidate())
    {
        return Err(CandidateCgroupCreateFailureV8::IssuedOrUncertain(
            CandidateCgroupCreateIssuedOrUncertainV8 {
                lease,
                leaf_name,
                leaf: Some(leaf),
                unvalidated_descriptor: None,
                cause,
            },
        ));
    }
    Ok(CandidateCgroupLeafV8 { lease, leaf })
}

#[cfg(not(target_os = "linux"))]
fn create_candidate_leaf_impl_v8(
    lease: CandidateCgroupNamespaceLeaseV8,
    _attempt_sha256: [u8; 32],
) -> Result<CandidateCgroupLeafV8, CandidateCgroupCreateFailureV8> {
    Err(CandidateCgroupCreateFailureV8::BeforeEffect {
        lease,
        cause: unsupported("create attempt-derived cgroup-v2 child"),
    })
}

#[cfg(target_os = "linux")]
fn recover_candidate_create_impl_v8(
    mut obligation: CandidateCgroupCreateIssuedOrUncertainV8,
) -> Result<CandidateCgroupLeafV8, CandidateCgroupCreateRecoveryFailureV8> {
    let fail = |obligation, cause| CandidateCgroupCreateRecoveryFailureV8 { obligation, cause };
    if let Err(cause) = obligation.lease.revalidate() {
        return Err(fail(obligation, cause));
    }
    if let Err(cause) = validate_cgroup_leaf_v8(&obligation.leaf_name) {
        return Err(fail(obligation, cause));
    }
    if obligation.leaf.is_none() {
        let descriptor = match obligation.unvalidated_descriptor.take() {
            Some(descriptor) => descriptor,
            None => match open_beneath_v8(
                obligation.lease.root.inner.descriptor.as_raw_fd(),
                &obligation.leaf_name,
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            ) {
                Ok(descriptor) => descriptor,
                Err(cause) => return Err(fail(obligation, cause)),
            },
        };
        let identity = match super::openat2::identity_for_fd(descriptor.as_raw_fd()) {
            Ok(identity) => identity,
            Err(cause) => {
                obligation.unvalidated_descriptor = Some(descriptor);
                return Err(fail(obligation, cause));
            }
        };
        let named = match open_beneath_v8(
            obligation.lease.root.inner.descriptor.as_raw_fd(),
            &obligation.leaf_name,
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
        ) {
            Ok(named) => named,
            Err(cause) => {
                obligation.unvalidated_descriptor = Some(descriptor);
                return Err(fail(obligation, cause));
            }
        };
        let named_identity = match super::openat2::identity_for_fd(named.as_raw_fd()) {
            Ok(identity) => identity,
            Err(cause) => {
                obligation.unvalidated_descriptor = Some(descriptor);
                return Err(fail(obligation, cause));
            }
        };
        if !named_identity.matches_stable_directory(identity) {
            obligation.unvalidated_descriptor = Some(descriptor);
            return Err(fail(
                obligation,
                NativeSysErrorV8::RaceDetected(
                    "post-mkdir candidate name no longer resolves to its retained descriptor"
                        .to_string(),
                ),
            ));
        }
        obligation.leaf = Some(CgroupV2LeafV8 {
            absolute_path: format!(
                "{}/{}",
                obligation
                    .lease
                    .root
                    .delegated_parent_path
                    .trim_end_matches('/'),
                obligation.leaf_name
            ),
            root: Arc::clone(&obligation.lease.root.inner),
            descriptor,
            identity,
            leaf: obligation.leaf_name.clone(),
        });
    }
    let leaf = match obligation.leaf.take() {
        Some(leaf) => leaf,
        None => {
            return Err(fail(
                obligation,
                NativeSysErrorV8::RaceDetected(
                    "post-mkdir candidate recovery has no retained descriptor".to_string(),
                ),
            ));
        }
    };
    let validation = require_cgroup_directory_policy_v8(
        &obligation.lease.root.inner,
        leaf.descriptor.as_raw_fd(),
        leaf.identity,
    )
    .and_then(|()| revalidate_empty_leaf_impl_v8(&leaf).map(|_| ()))
    .and_then(|()| obligation.lease.revalidate());
    if let Err(cause) = validation {
        obligation.leaf = Some(leaf);
        return Err(fail(obligation, cause));
    }
    Ok(CandidateCgroupLeafV8 {
        lease: obligation.lease,
        leaf,
    })
}

#[cfg(not(target_os = "linux"))]
fn recover_candidate_create_impl_v8(
    obligation: CandidateCgroupCreateIssuedOrUncertainV8,
) -> Result<CandidateCgroupLeafV8, CandidateCgroupCreateRecoveryFailureV8> {
    Err(CandidateCgroupCreateRecoveryFailureV8 {
        obligation,
        cause: unsupported("recover post-mkdir candidate cgroup"),
    })
}

#[cfg(target_os = "linux")]
fn prepare_candidate_empty_for_cleanup_impl_v8(
    candidate: CandidateCgroupLeafV8,
) -> Result<CandidateCgroupEmptyV8, CandidateCgroupPrepareCleanupFailureV8> {
    if let Err(cause) = candidate.lease.revalidate() {
        return Err(CandidateCgroupPrepareCleanupFailureV8 { candidate, cause });
    }
    let empty = match revalidate_empty_leaf_impl_v8(&candidate.leaf) {
        Ok(empty) => empty,
        Err(cause) => {
            return Err(CandidateCgroupPrepareCleanupFailureV8 { candidate, cause });
        }
    };
    Ok(CandidateCgroupEmptyV8 { candidate, empty })
}

#[cfg(not(target_os = "linux"))]
fn prepare_candidate_empty_for_cleanup_impl_v8(
    candidate: CandidateCgroupLeafV8,
) -> Result<CandidateCgroupEmptyV8, CandidateCgroupPrepareCleanupFailureV8> {
    Err(CandidateCgroupPrepareCleanupFailureV8 {
        candidate,
        cause: unsupported("prepare empty candidate cgroup cleanup"),
    })
}

#[cfg(target_os = "linux")]
fn clone_candidate_with_barrier_impl_v8(
    candidate: CandidateCgroupLeafV8,
) -> Result<CandidateCgroupCloneOutcomeV8, CandidateCgroupCloneFailureV8> {
    let before =
        |candidate, cause| CandidateCgroupCloneFailureV8::BeforeEffect { candidate, cause };
    if let Err(cause) = candidate.lease.revalidate() {
        return Err(before(candidate, cause));
    }
    if let Err(cause) = revalidate_empty_leaf_impl_v8(&candidate.leaf) {
        return Err(before(candidate, cause));
    }
    #[cfg(test)]
    if candidate.leaf.root.backend == CgroupBackendV8::ExplicitTest {
        return Err(before(
            candidate,
            NativeSysErrorV8::InvalidInput(
                "clone3 INTO_CGROUP never falls back to an explicit-test/numeric attach"
                    .to_string(),
            ),
        ));
    }

    let mut pipe = [-1_i32; 2];
    // SAFETY: pipe points to two writable descriptors; O_CLOEXEC is a scalar.
    if unsafe { libc::pipe2(pipe.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        return Err(before(
            candidate,
            io_error(
                "create candidate clone3 start barrier pipe",
                std::io::Error::last_os_error(),
            ),
        ));
    }
    // SAFETY: successful pipe2 returns two uniquely owned descriptors.
    let start_read = unsafe { OwnedFd::from_raw_fd(pipe[0]) };
    // SAFETY: see above.
    let release = unsafe { OwnedFd::from_raw_fd(pipe[1]) };
    let mut pidfd_slot = -1_i32;
    // SAFETY: zero is the kernel-defined baseline for clone_args.
    let mut arguments: libc::clone_args = unsafe { std::mem::zeroed() };
    const CLONE_INTO_CGROUP_U64_V8: u64 = 1_u64 << 33;
    arguments.flags = u64::try_from(libc::CLONE_PIDFD).unwrap_or(0) | CLONE_INTO_CGROUP_U64_V8;
    arguments.pidfd = (&mut pidfd_slot as *mut libc::c_int) as usize as u64;
    arguments.exit_signal = u64::try_from(libc::SIGCHLD).unwrap_or(0);
    arguments.cgroup = u64::try_from(candidate.leaf.descriptor.as_raw_fd()).unwrap_or(0);
    // SAFETY: arguments points to a fully initialized clone_args.  No stack,
    // VM, file-table, signal-handler, or thread sharing flags are admitted.
    let cloned = unsafe {
        libc::syscall(
            libc::SYS_clone3,
            &arguments as *const libc::clone_args,
            std::mem::size_of::<libc::clone_args>(),
        )
    };
    if cloned < 0 {
        return Err(before(
            candidate,
            io_error(
                "clone3 candidate directly into exact cgroup",
                std::io::Error::last_os_error(),
            ),
        ));
    }
    if cloned == 0 {
        drop(release);
        if pidfd_slot >= 0 {
            // SAFETY: the child closes its inherited clone pidfd integer; the
            // parent owns an independent descriptor table after clone3.
            unsafe { libc::close(pidfd_slot) };
        }
        let mut byte = [0_u8; 1];
        // SAFETY: buffer is writable and the read-end remains live.  EINTR is
        // fail-closed: the child exits rather than running before release.
        let read = unsafe { libc::read(start_read.as_raw_fd(), byte.as_mut_ptr().cast(), 1) };
        if read != 1 || byte != [0xa5] {
            // SAFETY: _exit performs no userspace cleanup in the post-clone
            // child and therefore cannot run duplicated parent destructors.
            unsafe { libc::_exit(126) };
        }
        drop(start_read);
        drop(candidate);
        return Ok(CandidateCgroupCloneOutcomeV8::Child(
            CandidateCgroupChildReleasedV8 { _private: () },
        ));
    }

    drop(start_read);
    let child_pid = match u32::try_from(cloned) {
        Ok(pid) if pid > 1 && pid <= i32::MAX as u32 => pid,
        _ => {
            // A successful clone with an unrepresentable PID is post-effect;
            // there is no numeric fallback or implicit signal here.
            let raw_pidfd = if pidfd_slot >= 0 {
                // SAFETY: successful CLONE_PIDFD wrote a uniquely owned fd.
                Some(unsafe { OwnedFd::from_raw_fd(pidfd_slot) })
            } else {
                None
            };
            return Err(CandidateCgroupCloneFailureV8::IssuedOrUncertain(
                CandidateCgroupCloneIssuedOrUncertainV8 {
                    candidate,
                    child_pid: 0,
                    verified_child: None,
                    raw_pidfd,
                    release,
                    cause: NativeSysErrorV8::RaceDetected(
                        "clone3 returned an invalid child pid".to_string(),
                    ),
                },
            ));
        }
    };
    if pidfd_slot < 0 {
        // No descriptor exists to own; retain the exact leaf and blocked-child
        // release pipe as the recovery boundary.
        return Err(CandidateCgroupCloneFailureV8::IssuedOrUncertain(
            CandidateCgroupCloneIssuedOrUncertainV8 {
                candidate,
                child_pid,
                verified_child: None,
                raw_pidfd: None,
                release,
                cause: NativeSysErrorV8::RaceDetected(
                    "successful clone3 did not return a pidfd".to_string(),
                ),
            },
        ));
    }
    // SAFETY: successful CLONE_PIDFD wrote a uniquely owned descriptor.
    let raw_pidfd = unsafe { OwnedFd::from_raw_fd(pidfd_slot) };
    // SAFETY: F_DUPFD_CLOEXEC duplicates a live descriptor without changing
    // the original recovery handle.
    let duplicate = unsafe { libc::fcntl(raw_pidfd.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 3) };
    if duplicate < 0 {
        return Err(CandidateCgroupCloneFailureV8::IssuedOrUncertain(
            CandidateCgroupCloneIssuedOrUncertainV8 {
                candidate,
                child_pid,
                verified_child: None,
                raw_pidfd: Some(raw_pidfd),
                release,
                cause: io_error(
                    "duplicate clone3 pidfd for anchored verification",
                    std::io::Error::last_os_error(),
                ),
            },
        ));
    }
    // SAFETY: fcntl returned a new uniquely owned descriptor.
    let duplicate = unsafe { OwnedFd::from_raw_fd(duplicate) };
    let procfs = match super::ProcfsRootV8::open_fixed() {
        Ok(procfs) => procfs,
        Err(cause) => {
            return Err(CandidateCgroupCloneFailureV8::IssuedOrUncertain(
                CandidateCgroupCloneIssuedOrUncertainV8 {
                    candidate,
                    child_pid,
                    verified_child: None,
                    raw_pidfd: Some(raw_pidfd),
                    release,
                    cause,
                },
            ));
        }
    };
    let child = match super::bind_clone3_pidfd_with_procfs_v8(&procfs, duplicate, child_pid) {
        Ok(child) => child,
        Err(cause) => {
            return Err(CandidateCgroupCloneFailureV8::IssuedOrUncertain(
                CandidateCgroupCloneIssuedOrUncertainV8 {
                    candidate,
                    child_pid,
                    verified_child: None,
                    raw_pidfd: Some(raw_pidfd),
                    release,
                    cause,
                },
            ));
        }
    };
    let validation = (|| {
        revalidate_nondelegated_leaf_v8(&candidate.leaf)?;
        let pids = parse_cgroup_procs_v8(&read_leaf_control_v8(&candidate.leaf, "cgroup.procs")?)?;
        let events =
            parse_cgroup_events_v8(&read_leaf_control_v8(&candidate.leaf, "cgroup.events")?)?;
        if pids != [child_pid] || !events.populated || events.frozen || child.is_exited()? {
            return Err(NativeSysErrorV8::RaceDetected(
                "clone3 child is not the exact live blocked member of its nondelegated leaf"
                    .to_string(),
            ));
        }
        Ok(())
    })();
    if let Err(cause) = validation {
        return Err(CandidateCgroupCloneFailureV8::IssuedOrUncertain(
            CandidateCgroupCloneIssuedOrUncertainV8 {
                candidate,
                child_pid,
                verified_child: Some(child),
                raw_pidfd: Some(raw_pidfd),
                release,
                cause,
            },
        ));
    }
    drop(raw_pidfd);
    Ok(CandidateCgroupCloneOutcomeV8::Parent(
        CandidateCgroupStartBarrierV8 {
            candidate,
            child,
            release,
        },
    ))
}

#[cfg(not(target_os = "linux"))]
fn clone_candidate_with_barrier_impl_v8(
    candidate: CandidateCgroupLeafV8,
) -> Result<CandidateCgroupCloneOutcomeV8, CandidateCgroupCloneFailureV8> {
    Err(CandidateCgroupCloneFailureV8::BeforeEffect {
        candidate,
        cause: unsupported("clone3 candidate directly into cgroup"),
    })
}

#[cfg(target_os = "linux")]
fn release_candidate_start_barrier_impl_v8(
    barrier: CandidateCgroupStartBarrierV8,
) -> Result<CandidateCgroupRunningV8, CandidateCgroupReleaseIssuedOrUncertainV8> {
    let byte = [0xa5_u8; 1];
    // SAFETY: byte and descriptor remain live; write retains no pointer.
    let written = unsafe {
        libc::write(
            barrier.release.as_raw_fd(),
            byte.as_ptr().cast(),
            byte.len(),
        )
    };
    if written != 1 {
        let cause = if written < 0 {
            io_error(
                "release exact candidate start barrier",
                std::io::Error::last_os_error(),
            )
        } else {
            NativeSysErrorV8::RaceDetected(
                "candidate start barrier release write was partial".to_string(),
            )
        };
        return Err(CandidateCgroupReleaseIssuedOrUncertainV8 { barrier, cause });
    }
    if let Err(cause) = revalidate_nondelegated_leaf_v8(&barrier.candidate.leaf).and_then(|()| {
        let pids = parse_cgroup_procs_v8(&read_leaf_control_v8(
            &barrier.candidate.leaf,
            "cgroup.procs",
        )?)?;
        if !pids.contains(&barrier.child.pid()) || barrier.child.is_exited()? {
            return Err(NativeSysErrorV8::RaceDetected(
                "released candidate child is no longer live in its exact leaf".to_string(),
            ));
        }
        Ok(())
    }) {
        return Err(CandidateCgroupReleaseIssuedOrUncertainV8 { barrier, cause });
    }
    let CandidateCgroupStartBarrierV8 {
        candidate,
        child,
        release: _,
    } = barrier;
    Ok(CandidateCgroupRunningV8 {
        candidate,
        child_pid: child.pid(),
        child: Some(child),
    })
}

#[cfg(not(target_os = "linux"))]
fn release_candidate_start_barrier_impl_v8(
    barrier: CandidateCgroupStartBarrierV8,
) -> Result<CandidateCgroupRunningV8, CandidateCgroupReleaseIssuedOrUncertainV8> {
    Err(CandidateCgroupReleaseIssuedOrUncertainV8 {
        barrier,
        cause: unsupported("release candidate cgroup start barrier"),
    })
}

#[cfg(target_os = "linux")]
fn issue_candidate_kill_impl_v8(
    population: CandidateCgroupPopulationV8,
) -> Result<CandidateCgroupKillIssuedOrUncertainV8, CandidateCgroupKillFailureV8> {
    let before = |population, cause| {
        CandidateCgroupKillFailureV8::BeforeEffect(CandidateCgroupKillBeforeEffectV8 {
            population,
            cause,
        })
    };
    if let Err(cause) = population.candidate.lease.revalidate() {
        return Err(before(population, cause));
    }
    if let Err(cause) = revalidate_nondelegated_leaf_v8(&population.candidate.leaf) {
        return Err(before(population, cause));
    }
    let pids = match read_leaf_control_v8(&population.candidate.leaf, "cgroup.procs")
        .and_then(|bytes| parse_cgroup_procs_v8(&bytes))
    {
        Ok(pids) => pids,
        Err(cause) => return Err(before(population, cause)),
    };
    let events = match read_leaf_control_v8(&population.candidate.leaf, "cgroup.events")
        .and_then(|bytes| parse_cgroup_events_v8(&bytes))
    {
        Ok(events) => events,
        Err(cause) => return Err(before(population, cause)),
    };
    if pids.is_empty()
        || !events.populated
        || (population.child_pid != 0 && !pids.contains(&population.child_pid))
    {
        return Err(before(
            population,
            NativeSysErrorV8::IdentityMismatch(
                "candidate kill target is not the exact populated child cgroup".to_string(),
            ),
        ));
    }
    let control = match open_leaf_control_for_write_v8(&population.candidate.leaf, "cgroup.kill") {
        Ok(control) => control,
        Err(cause) => return Err(before(population, cause)),
    };
    match write_open_control_once_v8(&control, b"1\n") {
        Ok(()) => Ok(CandidateCgroupKillIssuedOrUncertainV8 {
            population,
            cause: None,
        }),
        Err(cause) => Err(CandidateCgroupKillFailureV8::IssuedOrUncertain(
            CandidateCgroupKillIssuedOrUncertainV8 {
                population,
                cause: Some(cause),
            },
        )),
    }
}

#[cfg(not(target_os = "linux"))]
fn issue_candidate_kill_impl_v8(
    population: CandidateCgroupPopulationV8,
) -> Result<CandidateCgroupKillIssuedOrUncertainV8, CandidateCgroupKillFailureV8> {
    Err(CandidateCgroupKillFailureV8::BeforeEffect(
        CandidateCgroupKillBeforeEffectV8 {
            population,
            cause: unsupported("issue candidate cgroup.kill"),
        },
    ))
}

#[cfg(target_os = "linux")]
fn wait_candidate_empty_impl_v8(
    issued: CandidateCgroupKillIssuedOrUncertainV8,
    timeout: Duration,
) -> Result<CandidateCgroupEmptyV8, CandidateCgroupWaitFailureV8> {
    let fail = |issued, cause| CandidateCgroupWaitFailureV8 { issued, cause };
    if timeout.is_zero() || timeout > MAX_CGROUP_WAIT_V8 {
        return Err(fail(
            issued,
            NativeSysErrorV8::InvalidInput(format!(
                "candidate empty wait must be non-zero and at most {} seconds",
                MAX_CGROUP_WAIT_V8.as_secs()
            )),
        ));
    }
    let deadline = Instant::now() + timeout;
    let issued = issued;
    loop {
        let observation = (|| {
            revalidate_nondelegated_leaf_v8(&issued.population.candidate.leaf)?;
            let events = parse_cgroup_events_v8(&read_leaf_control_v8(
                &issued.population.candidate.leaf,
                "cgroup.events",
            )?)?;
            let pids = parse_cgroup_procs_v8(&read_leaf_control_v8(
                &issued.population.candidate.leaf,
                "cgroup.procs",
            )?)?;
            Ok::<_, NativeSysErrorV8>((events, pids))
        })();
        let (events, pids) = match observation {
            Ok(observation) => observation,
            Err(cause) => return Err(fail(issued, cause)),
        };
        if !events.populated && pids.is_empty() {
            if let Some(child) = &issued.population.verified_child {
                match child.is_exited() {
                    Ok(true) => {}
                    Ok(false) => {
                        return Err(fail(
                            issued,
                            NativeSysErrorV8::RaceDetected(
                                "candidate cgroup is empty but its exact child pidfd is live"
                                    .to_string(),
                            ),
                        ));
                    }
                    Err(cause) => return Err(fail(issued, cause)),
                }
            }
            let empty =
                match build_cgroup_empty_proof_v8(&issued.population.candidate.leaf, events, &pids)
                {
                    Ok(empty) => empty,
                    Err(cause) => return Err(fail(issued, cause)),
                };
            let CandidateCgroupKillIssuedOrUncertainV8 {
                population,
                cause: _,
            } = issued;
            let CandidateCgroupPopulationV8 {
                candidate,
                child_pid: _,
                verified_child: _,
                raw_pidfd: _,
                blocked_release_guard: _,
            } = population;
            return Ok(CandidateCgroupEmptyV8 { candidate, empty });
        }
        if Instant::now() >= deadline {
            return Err(fail(
                issued,
                NativeSysErrorV8::RaceDetected(
                    "candidate cgroup.kill was issued/uncertain but leaf did not become empty"
                        .to_string(),
                ),
            ));
        }
        std::thread::sleep(CGROUP_WAIT_POLL_V8);
    }
}

#[cfg(not(target_os = "linux"))]
fn wait_candidate_empty_impl_v8(
    issued: CandidateCgroupKillIssuedOrUncertainV8,
    _timeout: Duration,
) -> Result<CandidateCgroupEmptyV8, CandidateCgroupWaitFailureV8> {
    Err(CandidateCgroupWaitFailureV8 {
        issued,
        cause: unsupported("wait candidate cgroup empty"),
    })
}

#[cfg(target_os = "linux")]
fn cleanup_candidate_impl_v8(
    empty: CandidateCgroupEmptyV8,
) -> Result<CandidateCgroupTerminalV8, CandidateCgroupCleanupFailureV8> {
    let fail = |empty, cleanup_effect_issued_or_uncertain, cause| CandidateCgroupCleanupFailureV8 {
        empty,
        cleanup_effect_issued_or_uncertain,
        cause,
    };
    if let Err(cause) = empty.candidate.lease.revalidate() {
        return Err(fail(empty, false, cause));
    }
    let observed = match revalidate_empty_leaf_impl_v8(&empty.candidate.leaf) {
        Ok(observed) => observed,
        Err(cause) => return Err(fail(empty, false, cause)),
    };
    if observed != empty.empty {
        return Err(fail(
            empty,
            false,
            NativeSysErrorV8::RaceDetected(
                "candidate empty proof changed before cleanup".to_string(),
            ),
        ));
    }
    if let Err(cause) = revalidate_leaf_identity_v8(&empty.candidate.leaf) {
        return Err(fail(empty, false, cause));
    }

    let mut cleanup_effect_issued_or_uncertain = false;
    #[cfg(test)]
    if empty.candidate.leaf.root.backend == CgroupBackendV8::ExplicitTest {
        for control in [
            "cgroup.type",
            "cgroup.subtree_control",
            "cgroup.events",
            "cgroup.procs",
            "cgroup.kill",
        ] {
            cleanup_effect_issued_or_uncertain = true;
            if let Err(cause) = unlink_leaf_control_for_test_v8(&empty.candidate.leaf, control) {
                return Err(fail(empty, true, cause));
            }
        }
    }

    let leaf_c = match CString::new(empty.candidate.leaf.leaf.as_str()) {
        Ok(leaf_c) => leaf_c,
        Err(_) => {
            return Err(fail(
                empty,
                cleanup_effect_issued_or_uncertain,
                NativeSysErrorV8::InvalidInput("candidate cgroup leaf contains NUL".to_string()),
            ));
        }
    };
    // SAFETY: the namespace lease's parent descriptor and exact leaf name
    // remain live.  Once unlinkat is attempted, every error is conservatively
    // an issued/uncertain cleanup obligation.
    if unsafe {
        libc::unlinkat(
            empty.candidate.lease.root.inner.descriptor.as_raw_fd(),
            leaf_c.as_ptr(),
            libc::AT_REMOVEDIR,
        )
    } != 0
    {
        return Err(fail(
            empty,
            true,
            io_error(
                "unlinkat exact candidate cgroup child",
                std::io::Error::last_os_error(),
            ),
        ));
    }

    let post_identity =
        match super::openat2::identity_for_fd(empty.candidate.leaf.descriptor.as_raw_fd()) {
            Ok(identity) => identity,
            Err(cause) => return Err(fail(empty, true, cause)),
        };
    if post_identity.link_count() != 0 {
        return Err(fail(
            empty,
            true,
            NativeSysErrorV8::RaceDetected(
                "removed candidate cgroup descriptor still has a directory link".to_string(),
            ),
        ));
    }
    let name_absent = match open_beneath_v8(
        empty.candidate.lease.root.inner.descriptor.as_raw_fd(),
        &empty.candidate.leaf.leaf,
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
    ) {
        Err(error) if is_enoent_v8(&error) => true,
        Err(cause) => return Err(fail(empty, true, cause)),
        Ok(_) => {
            return Err(fail(
                empty,
                true,
                NativeSysErrorV8::RaceDetected(
                    "removed candidate cgroup name unexpectedly resolves".to_string(),
                ),
            ));
        }
    };
    if let Err(cause) = empty.candidate.lease.revalidate() {
        return Err(fail(empty, true, cause));
    }
    match candidate_child_names_v8(&empty.candidate.lease.root.inner) {
        Ok(names) if names.is_empty() => {}
        Ok(names) => {
            return Err(fail(
                empty,
                true,
                NativeSysErrorV8::RaceDetected(format!(
                    "candidate namespace is not empty after cleanup: {}",
                    names.join(",")
                )),
            ));
        }
        Err(cause) => return Err(fail(empty, true, cause)),
    }

    let CandidateCgroupEmptyV8 {
        candidate,
        empty: empty_proof,
    } = empty;
    let CandidateCgroupLeafV8 { lease, leaf } = candidate;
    let absence = CgroupLeafAbsenceProofV8 {
        absolute_path: leaf.absolute_path,
        parent_device: leaf.root.identity.device(),
        parent_inode: leaf.root.identity.inode(),
        removed_device: leaf.identity.device(),
        removed_inode: leaf.identity.inode(),
        child_link_count_after_cleanup: post_identity.link_count(),
        name_absent,
        observation_sha256: cgroup_absence_observation_sha256_v8(
            leaf.root.identity,
            leaf.identity,
            post_identity.link_count(),
            name_absent,
        ),
    };
    Ok(CandidateCgroupTerminalV8 {
        lease,
        empty: empty_proof,
        absence,
    })
}

#[cfg(not(target_os = "linux"))]
fn cleanup_candidate_impl_v8(
    empty: CandidateCgroupEmptyV8,
) -> Result<CandidateCgroupTerminalV8, CandidateCgroupCleanupFailureV8> {
    Err(CandidateCgroupCleanupFailureV8 {
        empty,
        cleanup_effect_issued_or_uncertain: false,
        cause: unsupported("cleanup candidate cgroup"),
    })
}

#[cfg(target_os = "linux")]
fn revalidate_empty_leaf_impl_v8(leaf: &CgroupV2LeafV8) -> NativeSysResultV8<CgroupEmptyProofV8> {
    revalidate_nondelegated_leaf_v8(leaf)?;
    let events = parse_cgroup_events_v8(&read_leaf_control_v8(leaf, "cgroup.events")?)?;
    let pids = parse_cgroup_procs_v8(&read_leaf_control_v8(leaf, "cgroup.procs")?)?;
    if events.populated || !pids.is_empty() {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "cgroup leaf is not empty in both events and procs".to_string(),
        ));
    }
    build_cgroup_empty_proof_v8(leaf, events, &pids)
}

#[cfg(not(target_os = "linux"))]
fn revalidate_empty_leaf_impl_v8(_leaf: &CgroupV2LeafV8) -> NativeSysResultV8<CgroupEmptyProofV8> {
    Err(unsupported("revalidate empty cgroup-v2 leaf"))
}

#[cfg(target_os = "linux")]
fn build_cgroup_empty_proof_v8(
    leaf: &CgroupV2LeafV8,
    events: CgroupEventsV8,
    pids: &[u32],
) -> NativeSysResultV8<CgroupEmptyProofV8> {
    let delegated =
        parse_subtree_control_v8(&read_leaf_control_v8(leaf, "cgroup.subtree_control")?)?;
    if events.populated || !pids.is_empty() || !delegated.is_empty() {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "candidate cgroup terminal proof is populated or delegated".to_string(),
        ));
    }
    let delegated_controller_count = u32::try_from(delegated.len()).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch("delegated controller count exceeds u32".to_string())
    })?;
    let observed_process_count = u32::try_from(pids.len()).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch("observed cgroup process count exceeds u32".to_string())
    })?;
    Ok(CgroupEmptyProofV8 {
        leaf: leaf.leaf.clone(),
        events,
        parent_device: leaf.root.identity.device(),
        parent_inode: leaf.root.identity.inode(),
        device: leaf.identity.device(),
        inode: leaf.identity.inode(),
        owner_uid: leaf.identity.owner_uid(),
        owner_gid: leaf.identity.owner_gid(),
        delegated_controller_count,
        observed_process_count,
        observation_sha256: cgroup_empty_observation_sha256_v8(
            leaf.root.identity,
            leaf.identity,
            delegated_controller_count,
            observed_process_count,
            events,
        ),
    })
}

#[cfg(target_os = "linux")]
fn cgroup_empty_observation_sha256_v8(
    parent: FileIdentityV8,
    child: FileIdentityV8,
    delegated_controller_count: u32,
    observed_process_count: u32,
    events: CgroupEventsV8,
) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"hepta-linux-v8-cgroup-empty-observation-v1\0");
    for value in [
        parent.device(),
        parent.inode(),
        child.device(),
        child.inode(),
        u64::from(child.owner_uid()),
        u64::from(child.owner_gid()),
        u64::from(delegated_controller_count),
        u64::from(observed_process_count),
        u64::from(events.populated),
        u64::from(events.frozen),
    ] {
        hasher.update(value.to_be_bytes());
    }
    hasher.finalize().into()
}

#[cfg(target_os = "linux")]
fn cgroup_absence_observation_sha256_v8(
    parent: FileIdentityV8,
    child: FileIdentityV8,
    child_link_count_after_cleanup: u64,
    name_absent: bool,
) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(b"hepta-linux-v8-cgroup-cleanup-absence-v1\0");
    for value in [
        parent.device(),
        parent.inode(),
        child.device(),
        child.inode(),
        child_link_count_after_cleanup,
        u64::from(name_absent),
    ] {
        hasher.update(value.to_be_bytes());
    }
    hasher.finalize().into()
}

#[cfg(target_os = "linux")]
fn revalidate_nondelegated_leaf_v8(leaf: &CgroupV2LeafV8) -> NativeSysResultV8<()> {
    revalidate_leaf_identity_v8(leaf)?;
    let cgroup_type = parse_cgroup_type_v8(&read_leaf_control_v8(leaf, "cgroup.type")?)?;
    let subtree = parse_subtree_control_v8(&read_leaf_control_v8(leaf, "cgroup.subtree_control")?)?;
    if cgroup_type != CgroupTypeV8::Domain || !subtree.is_empty() {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "cgroup leaf is delegated, threaded, or not exact domain type".to_string(),
        ));
    }
    for control in [
        "cgroup.type",
        "cgroup.subtree_control",
        "cgroup.events",
        "cgroup.procs",
        "cgroup.kill",
    ] {
        let descriptor = open_beneath_v8(
            leaf.descriptor.as_raw_fd(),
            control,
            libc::O_RDONLY | libc::O_CLOEXEC,
        )?;
        require_control_descriptor_policy_v8(&leaf.root, descriptor.as_raw_fd(), control)?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn revalidate_leaf_identity_v8(leaf: &CgroupV2LeafV8) -> NativeSysResultV8<()> {
    revalidate_root_inner_v8(&leaf.root)?;
    let open_identity = super::openat2::identity_for_fd(leaf.descriptor.as_raw_fd())?;
    if !open_identity.matches_stable_directory(leaf.identity) {
        return Err(NativeSysErrorV8::RaceDetected(
            "opened cgroup leaf identity changed or was unlinked".to_string(),
        ));
    }
    require_cgroup_directory_policy_v8(&leaf.root, leaf.descriptor.as_raw_fd(), open_identity)?;
    let named = open_beneath_v8(
        leaf.root.descriptor.as_raw_fd(),
        &leaf.leaf,
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
    )?;
    let named_identity = super::openat2::identity_for_fd(named.as_raw_fd())?;
    if !named_identity.matches_stable_directory(leaf.identity) {
        return Err(NativeSysErrorV8::RaceDetected(
            "cgroup leaf pathname no longer names the opened inode".to_string(),
        ));
    }
    Ok(())
}

#[cfg(all(test, target_os = "linux"))]
fn open_cgroup_root_v8(
    path: &Path,
    backend: CgroupBackendV8,
) -> NativeSysResultV8<CgroupRootInnerV8> {
    let path_c = CString::new(path.as_os_str().as_encoded_bytes())
        .map_err(|_| NativeSysErrorV8::InvalidInput("cgroup root path contains NUL".to_string()))?;
    // SAFETY: the path remains live and open retains no pointer.
    let raw_fd = unsafe {
        libc::open(
            path_c.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if raw_fd < 0 {
        return Err(io_error(
            "open cgroup-v2 root",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: successful open returned a uniquely owned descriptor.
    let descriptor = unsafe { OwnedFd::from_raw_fd(raw_fd) };
    let identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
    let inner = CgroupRootInnerV8 {
        descriptor,
        identity,
        reopen_path: path.to_path_buf(),
        backend,
        production_anchor: None,
    };
    require_cgroup_directory_policy_v8(&inner, inner.descriptor.as_raw_fd(), identity)?;
    Ok(inner)
}

#[cfg(target_os = "linux")]
fn observe_runner_scope_impl_v8(
    manifest: RunnerScopeTrustedManifestV8,
) -> NativeSysResultV8<ObservedRunnerScopeV8> {
    validate_runner_scope_manifest_v8(&manifest)?;
    if super::observe_boot_id_v8()? != manifest.boot_id {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "runner scope trusted systemd boot binding differs from the kernel boot".to_string(),
        ));
    }
    verify_trusted_runner_filesystem_v8(&manifest)?;
    let scope = open_existing_systemd_scope_impl_v8(&manifest.control_group)?;
    let expected_pids: Vec<u32> = manifest
        .processes
        .iter()
        .map(|process| process.pid)
        .collect();
    let (roster_before, events_before) = scope.roster_and_events()?;
    if roster_before != expected_pids || !events_before.populated || events_before.frozen {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "systemd ControlGroup roster does not match the trusted runner manifest".to_string(),
        ));
    }
    let group = super::observe_exact_process_group_v8(&expected_pids)?;
    let runtime = group.observe_runtime_closure_v8()?;
    if runtime.len() != manifest.processes.len() {
        return Err(NativeSysErrorV8::RaceDetected(
            "runner runtime binding count differs from trusted manifest".to_string(),
        ));
    }
    for (observed, expected) in runtime.iter().zip(&manifest.processes) {
        if observed.identity.pid() != expected.pid
            || observed.identity.start_ticks() != expected.start_ticks
            || observed.identity.parent_pid() != expected.parent_pid
            || observed.identity.process_group_id() != expected.process_group_id
            || observed.identity.session_id() != expected.session_id
            || observed.identity.executable().sha256() != expected.executable_sha256
            || observed.runtime.uid != expected.uid
            || observed.runtime.gid != expected.gid
            || observed.runtime.argv_sha256 != expected.argv_sha256
            || observed.runtime.cwd_device != expected.cwd_device
            || observed.runtime.cwd_inode != expected.cwd_inode
        {
            return Err(NativeSysErrorV8::IdentityMismatch(format!(
                "runner pid {} differs from trusted uid/gid/topology/argv/cwd/executable closure",
                expected.pid
            )));
        }
    }
    if group.process_group_id() != manifest.processes[0].process_group_id
        || group.session_id() != manifest.processes[0].session_id
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "trusted runner PGID/SID differs from the exact global closure".to_string(),
        ));
    }
    let initial = group.revalidate_fast_for_scope_v8()?;
    let (roster_after, events_after) = scope.roster_and_events()?;
    if roster_after != expected_pids || events_after != events_before {
        return Err(NativeSysErrorV8::RaceDetected(
            "runner ControlGroup roster/events changed during trusted observation".to_string(),
        ));
    }
    Ok(ObservedRunnerScopeV8 {
        manifest,
        scope,
        group,
        initial,
        runner_stop_intent: None,
    })
}

#[cfg(not(target_os = "linux"))]
fn observe_runner_scope_impl_v8(
    _manifest: RunnerScopeTrustedManifestV8,
) -> NativeSysResultV8<ObservedRunnerScopeV8> {
    Err(unsupported("observe trusted runner systemd scope"))
}

#[cfg(target_os = "linux")]
fn validate_runner_scope_manifest_v8(
    manifest: &RunnerScopeTrustedManifestV8,
) -> NativeSysResultV8<()> {
    validate_absolute_control_group_v8(&manifest.control_group)?;
    if manifest.delegate
        || !manifest.unit_name.ends_with(".service")
        || manifest.unit_name.len() > 256
        || !manifest.unit_name.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'@' | b':' | b'\\')
        })
        || manifest.processes.len() != 6
        || manifest.artifacts.len() + 1 > MAX_RUNNER_ARTIFACTS_V8
    {
        return Err(NativeSysErrorV8::InvalidInput(
            "trusted runner systemd unit/delegate/exact-six-process/artifact bounds are invalid"
                .to_string(),
        ));
    }
    let mut pids = BTreeSet::new();
    let mut previous = None;
    for process in &manifest.processes {
        if process.pid <= 1
            || process.start_ticks == 0
            || process.process_group_id <= 1
            || process.session_id <= 1
            || process.executable_sha256 == [0; 32]
            || process.argv_sha256 == [0; 32]
            || process.cwd_device == 0
            || process.cwd_inode == 0
            || previous.is_some_and(|pid| pid >= process.pid)
            || !pids.insert(process.pid)
        {
            return Err(NativeSysErrorV8::InvalidInput(
                "trusted runner process manifest is unsorted, duplicate, or incomplete".to_string(),
            ));
        }
        previous = Some(process.pid);
    }
    if !pids.contains(&manifest.main_pid)
        || manifest.processes.iter().any(|process| {
            process.process_group_id != manifest.processes[0].process_group_id
                || process.session_id != manifest.processes[0].session_id
        })
    {
        return Err(NativeSysErrorV8::InvalidInput(
            "systemd MainPID/runner PGID/SID is not bound to the exact manifest".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn verify_trusted_runner_filesystem_v8(
    manifest: &RunnerScopeTrustedManifestV8,
) -> NativeSysResultV8<()> {
    let root_path = c"/";
    // SAFETY: static path remains live; open retains no pointer.
    let raw_root = unsafe {
        libc::open(
            root_path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if raw_root < 0 {
        return Err(io_error(
            "open filesystem root for trusted runner artifacts",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: successful open returned a uniquely owned descriptor.
    let root = unsafe { OwnedFd::from_raw_fd(raw_root) };
    let root_identity = super::openat2::identity_for_fd(root.as_raw_fd())?;
    if root_identity.owner_uid() != 0
        || root_identity.owner_gid() != 0
        || root_identity.mode() & 0o022 != 0
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "filesystem root for trusted runner artifacts is not protected/root-owned".to_string(),
        ));
    }
    verify_trusted_runner_directory_v8(&root, &manifest.workroot)?;
    let deadline = Instant::now() + MAX_RUNNER_ARTIFACT_ELAPSED_V8;
    let mut remaining = MAX_RUNNER_ARTIFACT_HASH_BYTES_V8;
    let mut paths = BTreeSet::new();
    for artifact in std::iter::once(&manifest.fragment).chain(&manifest.artifacts) {
        if !paths.insert(artifact.absolute_path.as_str()) {
            return Err(NativeSysErrorV8::InvalidInput(
                "trusted runner artifact manifest contains a duplicate path".to_string(),
            ));
        }
        remaining = remaining.checked_sub(artifact.size).ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch(
                "trusted runner artifacts exceed aggregate hash-byte bound".to_string(),
            )
        })?;
        if Instant::now() > deadline {
            return Err(NativeSysErrorV8::RaceDetected(
                "trusted runner artifact capture exceeded elapsed-time bound".to_string(),
            ));
        }
        verify_trusted_runner_artifact_v8(&root, artifact)?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn open_trusted_absolute_v8(
    root: &OwnedFd,
    absolute: &str,
    flags: libc::c_int,
) -> NativeSysResultV8<OwnedFd> {
    if !absolute.starts_with('/') || absolute == "/" || absolute.len() > 4096 {
        return Err(NativeSysErrorV8::InvalidInput(
            "trusted runner path is not a bounded absolute path".to_string(),
        ));
    }
    open_beneath_v8(root.as_raw_fd(), absolute.trim_start_matches('/'), flags)
}

#[cfg(target_os = "linux")]
fn verify_trusted_runner_directory_v8(
    root: &OwnedFd,
    expected: &TrustedRunnerDirectoryV8,
) -> NativeSysResultV8<()> {
    let descriptor = open_trusted_absolute_v8(
        root,
        &expected.absolute_path,
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
    )?;
    let identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
    if identity.device() != expected.device
        || identity.inode() != expected.inode
        || identity.mode() != expected.mode
        || identity.owner_uid() != expected.owner_uid
        || identity.owner_gid() != expected.owner_gid
        || identity.link_count() == 0
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "trusted runner workroot directory identity differs from manifest".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn verify_trusted_runner_artifact_v8(
    root: &OwnedFd,
    expected: &TrustedRunnerArtifactV8,
) -> NativeSysResultV8<()> {
    if expected.size == 0
        || expected.size > MAX_RUNNER_ARTIFACT_HASH_BYTES_V8
        || expected.sha256 == [0; 32]
    {
        return Err(NativeSysErrorV8::InvalidInput(
            "trusted runner artifact size/digest is invalid".to_string(),
        ));
    }
    let descriptor = open_trusted_absolute_v8(
        root,
        &expected.absolute_path,
        libc::O_RDONLY | libc::O_CLOEXEC,
    )?;
    let before = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and descriptor remains live.
    if unsafe { libc::fstat(descriptor.as_raw_fd(), &mut stat) } != 0 {
        return Err(io_error(
            "fstat trusted runner artifact",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFREG
        || before.device() != expected.device
        || before.inode() != expected.inode
        || before.size() != expected.size
        || before.mode() != expected.mode
        || before.owner_uid() != expected.owner_uid
        || before.owner_gid() != expected.owner_gid
        || before.link_count() == 0
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "trusted runner script/Listener/fragment identity differs from manifest".to_string(),
        ));
    }
    let digest = sha256_exact_open_fd_v8(descriptor.as_raw_fd(), expected.size)?;
    let after = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
    if before != after || digest != expected.sha256 {
        return Err(NativeSysErrorV8::RaceDetected(
            "trusted runner artifact changed during bounded hashing".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn sha256_exact_open_fd_v8(fd: libc::c_int, size: u64) -> NativeSysResultV8<[u8; 32]> {
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut offset = 0_u64;
    while offset < size {
        let request = usize::try_from((size - offset).min(buffer.len() as u64)).map_err(|_| {
            NativeSysErrorV8::InvalidInput("runner artifact hash request overflow".to_string())
        })?;
        // SAFETY: buffer is writable and descriptor remains live.
        let read = unsafe {
            libc::pread(
                fd,
                buffer.as_mut_ptr().cast(),
                request,
                i64::try_from(offset).map_err(|_| {
                    NativeSysErrorV8::InvalidInput(
                        "runner artifact hash offset overflow".to_string(),
                    )
                })?,
            )
        };
        if read <= 0 {
            return Err(if read < 0 {
                io_error(
                    "pread trusted runner artifact",
                    std::io::Error::last_os_error(),
                )
            } else {
                NativeSysErrorV8::RaceDetected(
                    "trusted runner artifact became shorter while hashing".to_string(),
                )
            });
        }
        let read = usize::try_from(read).map_err(|_| {
            NativeSysErrorV8::InvalidInput("runner artifact read length overflow".to_string())
        })?;
        hasher.update(&buffer[..read]);
        let read_u64 = u64::try_from(read).map_err(|_| {
            NativeSysErrorV8::InvalidInput("runner artifact read length overflow".to_string())
        })?;
        offset = offset.checked_add(read_u64).ok_or_else(|| {
            NativeSysErrorV8::InvalidInput("runner artifact hash offset overflow".to_string())
        })?;
    }
    Ok(hasher.finalize().into())
}

#[cfg(target_os = "linux")]
fn capture_runner_stop_evidence_impl_v8(
    observed: &ObservedRunnerScopeV8,
    phase: RunnerStopEvidencePhaseV8,
    intent_manifest_sha256: Option<&str>,
    intent_record_sha256: Option<&str>,
) -> NativeSysResultV8<RunnerStopEffectEvidenceV8> {
    let observation_started_boottime_ns = clock_nanoseconds_v8(libc::CLOCK_BOOTTIME)?;
    let observation_started_monotonic_ns = clock_nanoseconds_v8(libc::CLOCK_MONOTONIC)?;
    let expected_frozen = match phase {
        RunnerStopEvidencePhaseV8::PreEffect => Some(false),
        RunnerStopEvidencePhaseV8::StoppedObservation => Some(false),
    };
    let (identities, _) = revalidate_observed_runner_scope_fast_v8(observed, expected_frozen)?;
    let phase_states_match = match phase {
        RunnerStopEvidencePhaseV8::PreEffect => identities.iter().all(|identity| {
            !identity.state().is_stopped()
                && !matches!(
                    identity.state(),
                    super::ProcessStateV8::Zombie | super::ProcessStateV8::Dead
                )
        }),
        RunnerStopEvidencePhaseV8::StoppedObservation => identities
            .iter()
            .all(|identity| identity.state().is_stopped()),
    };
    if !phase_states_match {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "runner STOP typed evidence process states differ from the requested phase".to_string(),
        ));
    }
    let mut processes = Vec::with_capacity(identities.len());
    for identity in &identities {
        let runtime = observed
            .manifest
            .processes
            .iter()
            .find(|expected| expected.pid == identity.pid())
            .ok_or_else(|| {
                NativeSysErrorV8::IdentityMismatch(
                    "runner STOP evidence process is absent from the trusted manifest".to_string(),
                )
            })?;
        processes.push(RunnerStopProcessEvidenceV8 {
            pid: identity.pid(),
            pidfd_bound: true,
            start_ticks: identity.start_ticks(),
            parent_pid: identity.parent_pid(),
            process_group_id: identity.process_group_id(),
            session_id: identity.session_id(),
            state: identity.state().as_proc_stat_byte(),
            executable_device: identity.executable().device(),
            executable_inode: identity.executable().inode(),
            executable_size: identity.executable().size(),
            executable_sha256: lower_hex_digest_v8(identity.executable().sha256()),
            uid: runtime.uid,
            gid: runtime.gid,
            argv_sha256: lower_hex_digest_v8(runtime.argv_sha256),
            cwd_device: runtime.cwd_device,
            cwd_inode: runtime.cwd_inode,
        });
    }
    processes.sort_by_key(|process| process.pid);
    let (after, _) = revalidate_observed_runner_scope_fast_v8(observed, expected_frozen)?;
    if after.len() != identities.len()
        || after.iter().zip(&identities).any(|(current, prior)| {
            !prior.has_same_stable_identity(current)
                || current.state().is_stopped() != prior.state().is_stopped()
        })
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "runner STOP closure changed during typed evidence capture".to_string(),
        ));
    }
    let observation_completed_monotonic_ns = clock_nanoseconds_v8(libc::CLOCK_MONOTONIC)?;
    let observation_completed_boottime_ns = clock_nanoseconds_v8(libc::CLOCK_BOOTTIME)?;
    Ok(RunnerStopEffectEvidenceV8 {
        schema: "hepta-linux-v8-runner-stop-evidence-v1".to_string(),
        phase,
        boot_id: observed.manifest.boot_id.to_string(),
        unit_name: observed.manifest.unit_name.clone(),
        control_group: observed.manifest.control_group.clone(),
        main_pid: observed.manifest.main_pid,
        cgroup_device: observed.scope.identity.device(),
        cgroup_inode: observed.scope.identity.inode(),
        cgroup_mount_id: observed.scope.observer.cgroup_mount_id,
        proc_mount_id: observed.scope.observer.proc_mount_id,
        pid_namespace_device: observed.scope.observer.pid_namespace_device,
        pid_namespace_inode: observed.scope.observer.pid_namespace_inode,
        cgroup_namespace_device: observed.scope.observer.cgroup_namespace_device,
        cgroup_namespace_inode: observed.scope.observer.cgroup_namespace_inode,
        mount_namespace_device: observed.scope.observer.mount_namespace_device,
        mount_namespace_inode: observed.scope.observer.mount_namespace_inode,
        process_group_id: observed.group.process_group_id(),
        session_id: observed.group.session_id(),
        observation_started_boottime_ns,
        observation_completed_boottime_ns,
        observation_started_monotonic_ns,
        observation_completed_monotonic_ns,
        intent_manifest_sha256: intent_manifest_sha256.map(str::to_string),
        intent_record_sha256: intent_record_sha256.map(str::to_string),
        processes,
    })
}

#[cfg(not(target_os = "linux"))]
fn capture_runner_stop_evidence_impl_v8(
    _observed: &ObservedRunnerScopeV8,
    _phase: RunnerStopEvidencePhaseV8,
    _intent_manifest_sha256: Option<&str>,
    _intent_record_sha256: Option<&str>,
) -> NativeSysResultV8<RunnerStopEffectEvidenceV8> {
    Err(unsupported("capture runner STOP typed evidence"))
}

#[cfg(target_os = "linux")]
fn capture_runner_restore_evidence_impl_v8(
    observed: &ObservedRunnerScopeV8,
    phase: RunnerRestoreEvidencePhaseV8,
    stopped_observation_record_sha256: &str,
    intent_manifest_sha256: Option<&str>,
    intent_record_sha256: Option<&str>,
) -> NativeSysResultV8<RunnerRestoreEffectEvidenceV8> {
    let observation_started_boottime_ns = clock_nanoseconds_v8(libc::CLOCK_BOOTTIME)?;
    let observation_started_monotonic_ns = clock_nanoseconds_v8(libc::CLOCK_MONOTONIC)?;
    let (identities, _) = revalidate_observed_runner_scope_fast_v8(observed, Some(false))?;
    let phase_states_match = match phase {
        RunnerRestoreEvidencePhaseV8::StoppedPreEffect => identities
            .iter()
            .all(|identity| identity.state().is_stopped()),
        RunnerRestoreEvidencePhaseV8::RunningObservation => identities.iter().all(|identity| {
            !identity.state().is_stopped()
                && !matches!(
                    identity.state(),
                    super::ProcessStateV8::Zombie
                        | super::ProcessStateV8::Dead
                        | super::ProcessStateV8::Idle
                )
        }),
    };
    if !phase_states_match {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "runner RESTORE typed evidence process states differ from the requested phase"
                .to_string(),
        ));
    }
    let mut processes = Vec::with_capacity(identities.len());
    for identity in &identities {
        let runtime = observed
            .manifest
            .processes
            .iter()
            .find(|expected| expected.pid == identity.pid())
            .ok_or_else(|| {
                NativeSysErrorV8::IdentityMismatch(
                    "runner RESTORE evidence process is absent from the trusted manifest"
                        .to_string(),
                )
            })?;
        processes.push(RunnerRestoreProcessEvidenceV8 {
            pid: identity.pid(),
            pidfd_bound: true,
            start_ticks: identity.start_ticks(),
            parent_pid: identity.parent_pid(),
            process_group_id: identity.process_group_id(),
            session_id: identity.session_id(),
            state: identity.state().as_proc_stat_byte(),
            executable_device: identity.executable().device(),
            executable_inode: identity.executable().inode(),
            executable_size: identity.executable().size(),
            executable_sha256: lower_hex_digest_v8(identity.executable().sha256()),
            uid: runtime.uid,
            gid: runtime.gid,
            argv_sha256: lower_hex_digest_v8(runtime.argv_sha256),
            cwd_device: runtime.cwd_device,
            cwd_inode: runtime.cwd_inode,
        });
    }
    processes.sort_by_key(|process| process.pid);
    let (after, _) = revalidate_observed_runner_scope_fast_v8(observed, Some(false))?;
    if after.len() != identities.len()
        || after.iter().zip(&identities).any(|(current, prior)| {
            !prior.has_same_stable_identity(current)
                || current.state().is_stopped() != prior.state().is_stopped()
        })
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "runner RESTORE closure changed during typed evidence capture".to_string(),
        ));
    }
    let observation_completed_monotonic_ns = clock_nanoseconds_v8(libc::CLOCK_MONOTONIC)?;
    let observation_completed_boottime_ns = clock_nanoseconds_v8(libc::CLOCK_BOOTTIME)?;
    Ok(RunnerRestoreEffectEvidenceV8 {
        schema: "hepta-linux-v8-runner-restore-evidence-v1".to_string(),
        phase,
        boot_id: observed.manifest.boot_id.to_string(),
        unit_name: observed.manifest.unit_name.clone(),
        control_group: observed.manifest.control_group.clone(),
        main_pid: observed.manifest.main_pid,
        cgroup_device: observed.scope.identity.device(),
        cgroup_inode: observed.scope.identity.inode(),
        cgroup_mount_id: observed.scope.observer.cgroup_mount_id,
        proc_mount_id: observed.scope.observer.proc_mount_id,
        pid_namespace_device: observed.scope.observer.pid_namespace_device,
        pid_namespace_inode: observed.scope.observer.pid_namespace_inode,
        cgroup_namespace_device: observed.scope.observer.cgroup_namespace_device,
        cgroup_namespace_inode: observed.scope.observer.cgroup_namespace_inode,
        mount_namespace_device: observed.scope.observer.mount_namespace_device,
        mount_namespace_inode: observed.scope.observer.mount_namespace_inode,
        process_group_id: observed.group.process_group_id(),
        session_id: observed.group.session_id(),
        stopped_observation_record_sha256: stopped_observation_record_sha256.to_string(),
        observation_started_boottime_ns,
        observation_completed_boottime_ns,
        observation_started_monotonic_ns,
        observation_completed_monotonic_ns,
        intent_manifest_sha256: intent_manifest_sha256.map(str::to_string),
        intent_record_sha256: intent_record_sha256.map(str::to_string),
        processes,
    })
}

#[cfg(not(target_os = "linux"))]
fn capture_runner_restore_evidence_impl_v8(
    _observed: &ObservedRunnerScopeV8,
    _phase: RunnerRestoreEvidencePhaseV8,
    _stopped_observation_record_sha256: &str,
    _intent_manifest_sha256: Option<&str>,
    _intent_record_sha256: Option<&str>,
) -> NativeSysResultV8<RunnerRestoreEffectEvidenceV8> {
    Err(unsupported("capture runner RESTORE typed evidence"))
}

#[cfg(target_os = "linux")]
fn clock_nanoseconds_v8(clock: libc::clockid_t) -> NativeSysResultV8<u64> {
    // SAFETY: zero initializes writable timespec storage.
    let mut timestamp: libc::timespec = unsafe { std::mem::zeroed() };
    // SAFETY: timestamp remains writable for the duration of clock_gettime.
    if unsafe { libc::clock_gettime(clock, &mut timestamp) } != 0 {
        return Err(io_error(
            "clock_gettime runner STOP evidence",
            std::io::Error::last_os_error(),
        ));
    }
    let seconds = u64::try_from(timestamp.tv_sec).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch(
            "runner STOP evidence clock seconds are negative".to_string(),
        )
    })?;
    let nanoseconds = u64::try_from(timestamp.tv_nsec).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch(
            "runner STOP evidence clock nanoseconds are negative".to_string(),
        )
    })?;
    seconds
        .checked_mul(1_000_000_000)
        .and_then(|value| value.checked_add(nanoseconds))
        .filter(|value| *value != 0)
        .ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch(
                "runner STOP evidence clock value overflows or is zero".to_string(),
            )
        })
}

fn lower_hex_digest_v8(digest: [u8; 32]) -> String {
    let mut output = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut output, "{byte:02x}");
    }
    output
}

#[cfg(target_os = "linux")]
fn revalidate_observed_runner_scope_fast_v8(
    observed: &ObservedRunnerScopeV8,
    expected_frozen: Option<bool>,
) -> NativeSysResultV8<(Vec<super::ProcessIdentityV8>, CgroupEventsV8)> {
    let expected_pids = observed.group.member_pids_v8();
    let (roster, events) = observed.scope.roster_and_events()?;
    if roster != expected_pids
        || !events.populated
        || expected_frozen.is_some_and(|expected| events.frozen != expected)
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "runner scope cgroup roster/populated/frozen state differs from typestate".to_string(),
        ));
    }
    let identities = observed.group.revalidate_fast_for_scope_v8()?;
    if identities.len() != observed.initial.len()
        || identities
            .iter()
            .zip(&observed.initial)
            .any(|(current, initial)| !initial.has_same_stable_identity(current))
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "runner scope stable process closure changed after observation".to_string(),
        ));
    }
    Ok((identities, events))
}

#[cfg(target_os = "linux")]
fn issue_runner_scope_freeze_impl_v8(
    observed: ObservedRunnerScopeV8,
) -> Result<RunnerScopeFreezeIssuedOrUncertainV8, RunnerScopeFreezeFailureV8> {
    let before = |observed, cause| RunnerScopeFreezeFailureV8::BeforeEffect { observed, cause };
    let (identities, _) = match revalidate_observed_runner_scope_fast_v8(&observed, Some(false)) {
        Ok(value) => value,
        Err(cause) => return Err(before(observed, cause)),
    };
    if identities.iter().any(|identity| {
        identity.state().is_stopped()
            || matches!(
                identity.state(),
                super::ProcessStateV8::Zombie | super::ProcessStateV8::Dead
            )
    }) {
        return Err(before(
            observed,
            NativeSysErrorV8::IdentityMismatch(
                "runner freeze requires every exact member live and not already stopped"
                    .to_string(),
            ),
        ));
    }
    let control = match open_existing_scope_freeze_for_write_v8(&observed.scope) {
        Ok(control) => control,
        Err(cause) => return Err(before(observed, cause)),
    };
    if let Err(cause) = write_open_control_once_v8(&control, b"1\n") {
        return Err(RunnerScopeFreezeFailureV8::IssuedOrUncertain(
            RunnerScopeFreezeIssuedOrUncertainV8 {
                observed,
                cause: Some(cause),
            },
        ));
    }
    #[cfg(test)]
    if observed.scope.backend == ExistingScopeBackendV8::ExplicitTest
        && let Err(cause) = write_existing_scope_test_control_v8(
            &observed.scope,
            "cgroup.events",
            b"populated 1\nfrozen 1\n",
        )
    {
        return Err(RunnerScopeFreezeFailureV8::IssuedOrUncertain(
            RunnerScopeFreezeIssuedOrUncertainV8 {
                observed,
                cause: Some(cause),
            },
        ));
    }
    Ok(RunnerScopeFreezeIssuedOrUncertainV8 {
        observed,
        cause: None,
    })
}

#[cfg(not(target_os = "linux"))]
fn issue_runner_scope_freeze_impl_v8(
    observed: ObservedRunnerScopeV8,
) -> Result<RunnerScopeFreezeIssuedOrUncertainV8, RunnerScopeFreezeFailureV8> {
    Err(RunnerScopeFreezeFailureV8::BeforeEffect {
        observed,
        cause: unsupported("issue runner cgroup freeze"),
    })
}

#[cfg(target_os = "linux")]
fn wait_runner_scope_frozen_impl_v8(
    issued: RunnerScopeFreezeIssuedOrUncertainV8,
    timeout: Duration,
) -> Result<FrozenRunnerScopeV8, RunnerScopeFreezeWaitFailureV8> {
    if timeout.is_zero() || timeout > MAX_RUNNER_SCOPE_WAIT_V8 {
        return Err(RunnerScopeFreezeWaitFailureV8 {
            issued,
            cause: NativeSysErrorV8::InvalidInput(
                "runner freeze wait is outside the frozen bound".to_string(),
            ),
        });
    }
    let deadline = Instant::now() + timeout;
    let issued = issued;
    loop {
        match revalidate_observed_runner_scope_fast_v8(&issued.observed, None) {
            Ok((_, events)) if events.frozen => {
                return Ok(FrozenRunnerScopeV8 {
                    observed: issued.observed,
                });
            }
            Ok(_) if Instant::now() >= deadline => {
                return Err(RunnerScopeFreezeWaitFailureV8 {
                    issued,
                    cause: NativeSysErrorV8::RaceDetected(
                        "runner scope did not report frozen before timeout".to_string(),
                    ),
                });
            }
            Err(cause) => return Err(RunnerScopeFreezeWaitFailureV8 { issued, cause }),
            _ => std::thread::sleep(CGROUP_WAIT_POLL_V8),
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn wait_runner_scope_frozen_impl_v8(
    issued: RunnerScopeFreezeIssuedOrUncertainV8,
    _timeout: Duration,
) -> Result<FrozenRunnerScopeV8, RunnerScopeFreezeWaitFailureV8> {
    Err(RunnerScopeFreezeWaitFailureV8 {
        issued,
        cause: unsupported("wait runner cgroup frozen"),
    })
}

#[cfg(target_os = "linux")]
fn abort_runner_scope_freeze_impl_v8(
    issued: RunnerScopeFreezeIssuedOrUncertainV8,
) -> Result<RunnerScopeFreezeAbortIssuedOrUncertainV8, RunnerScopeFreezeAbortFailureV8> {
    let control = match open_existing_scope_freeze_for_write_v8(&issued.observed.scope) {
        Ok(control) => control,
        Err(cause) => {
            return Err(RunnerScopeFreezeAbortFailureV8::BeforeEffect { issued, cause });
        }
    };
    let RunnerScopeFreezeIssuedOrUncertainV8 { observed, cause: _ } = issued;
    let mut cause = write_open_control_once_v8(&control, b"0\n").err();
    #[cfg(test)]
    if cause.is_none()
        && observed.scope.backend == ExistingScopeBackendV8::ExplicitTest
        && let Err(error) = write_existing_scope_test_control_v8(
            &observed.scope,
            "cgroup.events",
            b"populated 1\nfrozen 0\n",
        )
    {
        cause = Some(error);
    }
    let token = RunnerScopeFreezeAbortIssuedOrUncertainV8 { observed, cause };
    if token.cause.is_some() {
        Err(RunnerScopeFreezeAbortFailureV8::IssuedOrUncertain(token))
    } else {
        Ok(token)
    }
}

#[cfg(not(target_os = "linux"))]
fn abort_runner_scope_freeze_impl_v8(
    issued: RunnerScopeFreezeIssuedOrUncertainV8,
) -> Result<RunnerScopeFreezeAbortIssuedOrUncertainV8, RunnerScopeFreezeAbortFailureV8> {
    Err(RunnerScopeFreezeAbortFailureV8::BeforeEffect {
        issued,
        cause: unsupported("abort runner cgroup freeze"),
    })
}

#[cfg(target_os = "linux")]
fn wait_runner_scope_freeze_aborted_impl_v8(
    issued: RunnerScopeFreezeAbortIssuedOrUncertainV8,
    timeout: Duration,
) -> Result<ObservedRunnerScopeV8, RunnerScopeFreezeAbortWaitFailureV8> {
    if timeout.is_zero() || timeout > MAX_RUNNER_SCOPE_WAIT_V8 {
        return Err(RunnerScopeFreezeAbortWaitFailureV8 {
            issued,
            cause: NativeSysErrorV8::InvalidInput(
                "runner freeze-abort wait is outside the frozen bound".to_string(),
            ),
        });
    }
    let deadline = Instant::now() + timeout;
    let issued = issued;
    loop {
        match revalidate_observed_runner_scope_fast_v8(&issued.observed, None) {
            Ok((identities, events))
                if !events.frozen
                    && identities
                        .iter()
                        .all(|identity| !identity.state().is_stopped()) =>
            {
                return Ok(issued.observed);
            }
            Ok(_) if Instant::now() >= deadline => {
                return Err(RunnerScopeFreezeAbortWaitFailureV8 {
                    issued,
                    cause: NativeSysErrorV8::RaceDetected(
                        "runner freeze abort left an unexpectedly stopped member".to_string(),
                    ),
                });
            }
            Err(cause) => return Err(RunnerScopeFreezeAbortWaitFailureV8 { issued, cause }),
            _ => std::thread::sleep(CGROUP_WAIT_POLL_V8),
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn wait_runner_scope_freeze_aborted_impl_v8(
    issued: RunnerScopeFreezeAbortIssuedOrUncertainV8,
    _timeout: Duration,
) -> Result<ObservedRunnerScopeV8, RunnerScopeFreezeAbortWaitFailureV8> {
    Err(RunnerScopeFreezeAbortWaitFailureV8 {
        issued,
        cause: unsupported("wait runner freeze abort"),
    })
}

#[cfg(target_os = "linux")]
fn issue_runner_scope_stop_impl_v8(
    frozen: FrozenRunnerScopeV8,
) -> Result<RunnerScopeStopIssuedOrUncertainV8, RunnerScopeStopBeforeEffectV8> {
    issue_runner_scope_stop_core_v8(frozen, RunnerStopFaultV8::None)
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunnerStopFaultV8 {
    None,
    #[cfg(test)]
    PartialMember,
}

#[cfg(target_os = "linux")]
fn issue_runner_scope_stop_core_v8(
    frozen: FrozenRunnerScopeV8,
    fault: RunnerStopFaultV8,
) -> Result<RunnerScopeStopIssuedOrUncertainV8, RunnerScopeStopBeforeEffectV8> {
    #[cfg(not(test))]
    let _ = fault;
    let identities = match revalidate_observed_runner_scope_fast_v8(&frozen.observed, Some(true)) {
        Ok((identities, _)) => identities,
        Err(cause) => return Err(RunnerScopeStopBeforeEffectV8 { frozen, cause }),
    };
    if identities.iter().any(|identity| {
        identity.state().is_stopped()
            || matches!(
                identity.state(),
                super::ProcessStateV8::Zombie | super::ProcessStateV8::Dead
            )
    }) {
        return Err(RunnerScopeStopBeforeEffectV8 {
            frozen,
            cause: NativeSysErrorV8::IdentityMismatch(
                "frozen runner STOP requires every member live and not already stopped".to_string(),
            ),
        });
    }
    #[cfg(test)]
    let partial_pid = if fault == RunnerStopFaultV8::PartialMember {
        let pid = match identities.first().map(super::ProcessIdentityV8::pid) {
            Some(pid) => pid,
            None => {
                return Err(RunnerScopeStopBeforeEffectV8 {
                    frozen,
                    cause: NativeSysErrorV8::IdentityMismatch(
                        "runner partial-STOP fixture has no exact member".to_string(),
                    ),
                });
            }
        };
        match i32::try_from(pid) {
            Ok(pid) => Some(pid),
            Err(_) => {
                return Err(RunnerScopeStopBeforeEffectV8 {
                    frozen,
                    cause: NativeSysErrorV8::InvalidInput(
                        "runner partial-STOP fixture pid does not fit pid_t".to_string(),
                    ),
                });
            }
        }
    } else {
        None
    };
    #[cfg(not(test))]
    let partial_pid: Option<i32> = None;

    let target = match partial_pid {
        Some(pid) => pid,
        None => match i32::try_from(frozen.observed.group.process_group_id()) {
            Ok(pgid) => -pgid,
            Err(_) => {
                return Err(RunnerScopeStopBeforeEffectV8 {
                    frozen,
                    cause: NativeSysErrorV8::InvalidInput(
                        "runner process group does not fit pid_t".to_string(),
                    ),
                });
            }
        },
    };
    // SAFETY: production uses the exact frozen negative PGID. The positive
    // PID branch exists only in tests and is selected from the retained exact
    // member observations. Only SIGSTOP is admitted.
    let rc = unsafe { libc::kill(target, libc::SIGSTOP) };
    let cause = if rc != 0 {
        Some(io_error(
            "issue SIGSTOP to exact frozen runner target",
            std::io::Error::last_os_error(),
        ))
    } else if partial_pid.is_some() {
        Some(NativeSysErrorV8::RaceDetected(
            "injected partial runner SIGSTOP outcome after one member".to_string(),
        ))
    } else {
        None
    };
    Ok(RunnerScopeStopIssuedOrUncertainV8 {
        observed: frozen.observed,
        cause,
    })
}

#[cfg(not(target_os = "linux"))]
fn issue_runner_scope_stop_impl_v8(
    frozen: FrozenRunnerScopeV8,
) -> Result<RunnerScopeStopIssuedOrUncertainV8, RunnerScopeStopBeforeEffectV8> {
    Err(RunnerScopeStopBeforeEffectV8 {
        frozen,
        cause: unsupported("issue frozen runner SIGSTOP"),
    })
}

#[cfg(target_os = "linux")]
fn issue_runner_scope_unfreeze_impl_v8(
    stopped: RunnerScopeStopIssuedOrUncertainV8,
) -> Result<RunnerScopeUnfreezeIssuedOrUncertainV8, RunnerScopeUnfreezeFailureV8> {
    if let Err(cause) = revalidate_observed_runner_scope_fast_v8(&stopped.observed, Some(true)) {
        return Err(RunnerScopeUnfreezeFailureV8::BeforeEffect { stopped, cause });
    }
    let control = match open_existing_scope_freeze_for_write_v8(&stopped.observed.scope) {
        Ok(control) => control,
        Err(cause) => {
            return Err(RunnerScopeUnfreezeFailureV8::BeforeEffect { stopped, cause });
        }
    };
    let RunnerScopeStopIssuedOrUncertainV8 {
        observed,
        cause: stop_cause,
    } = stopped;
    let mut unfreeze_cause = write_open_control_once_v8(&control, b"0\n").err();
    #[cfg(test)]
    if unfreeze_cause.is_none()
        && observed.scope.backend == ExistingScopeBackendV8::ExplicitTest
        && let Err(error) = write_existing_scope_test_control_v8(
            &observed.scope,
            "cgroup.events",
            b"populated 1\nfrozen 0\n",
        )
    {
        unfreeze_cause = Some(error);
    }
    let token = RunnerScopeUnfreezeIssuedOrUncertainV8 {
        observed,
        stop_cause,
        unfreeze_cause,
    };
    if token.unfreeze_cause.is_some() {
        Err(RunnerScopeUnfreezeFailureV8::IssuedOrUncertain(token))
    } else {
        Ok(token)
    }
}

#[cfg(not(target_os = "linux"))]
fn issue_runner_scope_unfreeze_impl_v8(
    stopped: RunnerScopeStopIssuedOrUncertainV8,
) -> Result<RunnerScopeUnfreezeIssuedOrUncertainV8, RunnerScopeUnfreezeFailureV8> {
    Err(RunnerScopeUnfreezeFailureV8::BeforeEffect {
        stopped,
        cause: unsupported("issue runner cgroup unfreeze after STOP"),
    })
}

#[cfg(target_os = "linux")]
fn wait_runner_scope_unfrozen_stopped_impl_v8(
    issued: RunnerScopeUnfreezeIssuedOrUncertainV8,
    timeout: Duration,
) -> Result<StoppedRunnerScopeV8, RunnerScopeUnfreezeWaitFailureV8> {
    if timeout.is_zero() || timeout > MAX_RUNNER_SCOPE_WAIT_V8 {
        return Err(RunnerScopeUnfreezeWaitFailureV8 {
            issued,
            cause: NativeSysErrorV8::InvalidInput(
                "runner unfreeze/T wait is outside the frozen bound".to_string(),
            ),
        });
    }
    let deadline = Instant::now() + timeout;
    let issued = issued;
    loop {
        match revalidate_observed_runner_scope_fast_v8(&issued.observed, None) {
            Ok((identities, events))
                if !events.frozen
                    && identities
                        .iter()
                        .all(|identity| identity.state().is_stopped()) =>
            {
                return Ok(StoppedRunnerScopeV8 {
                    observed: issued.observed,
                    stopped: identities,
                });
            }
            Ok(_) if Instant::now() >= deadline => {
                return Err(RunnerScopeUnfreezeWaitFailureV8 {
                    issued,
                    cause: NativeSysErrorV8::RaceDetected(
                        "unfrozen runner members did not all reach T before timeout".to_string(),
                    ),
                });
            }
            Err(cause) => return Err(RunnerScopeUnfreezeWaitFailureV8 { issued, cause }),
            _ => std::thread::sleep(CGROUP_WAIT_POLL_V8),
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn wait_runner_scope_unfrozen_stopped_impl_v8(
    issued: RunnerScopeUnfreezeIssuedOrUncertainV8,
    _timeout: Duration,
) -> Result<StoppedRunnerScopeV8, RunnerScopeUnfreezeWaitFailureV8> {
    Err(RunnerScopeUnfreezeWaitFailureV8 {
        issued,
        cause: unsupported("wait runner unfrozen and stopped"),
    })
}

#[cfg(target_os = "linux")]
fn plan_runner_scope_continue_impl_v8(
    stopped: RestoreAuthorizedStoppedRunnerScopeV8,
) -> Result<RunnerScopeContinuePlanV8, RunnerScopeContinuePlanFailureV8> {
    let binding_matches = stopped
        .stopped
        .capture_runner_restore_manifest_v8()
        .map_err(|_| ())
        .and_then(|current| {
            RunnerRestoreEffectEvidenceV8::decode_exact(
                stopped.restore_intent.effect_manifest_bytes(),
            )
            .map_err(|_| ())
            .and_then(|archived| {
                Ok(
                    archived.phase == RunnerRestoreEvidencePhaseV8::StoppedPreEffect
                        && archived.stopped_observation_record_sha256
                            == stopped.stopped.observation_record_sha256
                        && stopped.restore_intent.stopped_observation_record_sha256()
                            == stopped.stopped.observation_record_sha256
                        && archived.sha256().map_err(|_| ())?
                            == stopped.restore_intent.effect_manifest_sha256()
                        && archived.scope_binding_bytes().map_err(|_| ())?
                            == stopped.restore_intent.scope_binding_bytes()
                        && current.scope_binding_bytes().map_err(|_| ())?
                            == stopped.restore_intent.scope_binding_bytes(),
                )
            })
        })
        .unwrap_or(false);
    if !binding_matches {
        return Err(RunnerScopeContinuePlanFailureV8::Stopped {
            stopped,
            cause: NativeSysErrorV8::IdentityMismatch(
                "runner RESTORE plan differs from its exact durable intent binding".to_string(),
            ),
        });
    }
    let identities = match revalidate_observed_runner_scope_fast_v8(
        &stopped.stopped.stopped.observed,
        Some(false),
    ) {
        Ok((identities, _)) => identities,
        Err(cause) => {
            return Err(RunnerScopeContinuePlanFailureV8::Stopped { stopped, cause });
        }
    };
    if identities.len() != stopped.stopped.stopped.stopped.len()
        || identities
            .iter()
            .zip(&stopped.stopped.stopped.stopped)
            .any(|(current, prior)| !prior.has_same_stable_identity(current))
        || identities
            .iter()
            .any(|identity| !identity.state().is_stopped())
    {
        return Err(RunnerScopeContinuePlanFailureV8::Stopped {
            stopped,
            cause: NativeSysErrorV8::IdentityMismatch(
                "normal runner CONT requires every exact member stopped".to_string(),
            ),
        });
    }
    let RestoreAuthorizedStoppedRunnerScopeV8 {
        stopped,
        restore_intent,
    } = stopped;
    let runner_stop_observation_record_sha256 = stopped.observation_record_sha256;
    let stopped = stopped.stopped;
    Ok(RunnerScopeContinuePlanV8 {
        observed: stopped.observed,
        before: identities,
        recovery: false,
        runner_restore_intent: Some(restore_intent),
        runner_stop_observation_record_sha256: Some(runner_stop_observation_record_sha256),
    })
}

#[cfg(not(target_os = "linux"))]
fn plan_runner_scope_continue_impl_v8(
    stopped: RestoreAuthorizedStoppedRunnerScopeV8,
) -> Result<RunnerScopeContinuePlanV8, RunnerScopeContinuePlanFailureV8> {
    Err(RunnerScopeContinuePlanFailureV8::Stopped {
        stopped,
        cause: unsupported("plan runner CONT"),
    })
}

#[cfg(target_os = "linux")]
fn plan_runner_scope_recovery_continue_impl_v8(
    failure: RunnerScopeUnfreezeWaitFailureV8,
) -> Result<RunnerScopeContinuePlanV8, RunnerScopeContinuePlanFailureV8> {
    let identities =
        match revalidate_observed_runner_scope_fast_v8(&failure.issued.observed, Some(false)) {
            Ok((identities, _)) => identities,
            Err(cause) => {
                return Err(RunnerScopeContinuePlanFailureV8::Recovery { failure, cause });
            }
        };
    if identities.iter().any(|identity| {
        matches!(
            identity.state(),
            super::ProcessStateV8::Zombie | super::ProcessStateV8::Dead
        )
    }) {
        return Err(RunnerScopeContinuePlanFailureV8::Recovery {
            failure,
            cause: NativeSysErrorV8::IdentityMismatch(
                "runner recovery CONT contains a dead/zombie member".to_string(),
            ),
        });
    }
    Ok(RunnerScopeContinuePlanV8 {
        observed: failure.issued.observed,
        before: identities,
        recovery: true,
        runner_restore_intent: None,
        runner_stop_observation_record_sha256: None,
    })
}

#[cfg(not(target_os = "linux"))]
fn plan_runner_scope_recovery_continue_impl_v8(
    failure: RunnerScopeUnfreezeWaitFailureV8,
) -> Result<RunnerScopeContinuePlanV8, RunnerScopeContinuePlanFailureV8> {
    Err(RunnerScopeContinuePlanFailureV8::Recovery {
        failure,
        cause: unsupported("plan runner recovery CONT"),
    })
}

#[cfg(target_os = "linux")]
fn execute_runner_scope_continue_impl_v8(
    plan: RunnerScopeContinuePlanV8,
) -> Result<RunnerScopeContinueExecutionV8, RunnerScopePostContinueQuarantineV8> {
    execute_runner_scope_continue_core_v8(plan, RunnerContinueFaultV8::None)
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunnerContinueFaultV8 {
    None,
    #[cfg(test)]
    AfterIssue,
}

#[cfg(target_os = "linux")]
fn execute_runner_scope_continue_core_v8(
    mut plan: RunnerScopeContinuePlanV8,
    fault: RunnerContinueFaultV8,
) -> Result<RunnerScopeContinueExecutionV8, RunnerScopePostContinueQuarantineV8> {
    #[cfg(not(test))]
    let _ = fault;
    let quarantine =
        |observed, before, runner_restore_intent, runner_stop_observation_record_sha256, cause| {
            RunnerScopePostContinueQuarantineV8 {
                observed,
                before,
                runner_restore_intent,
                runner_stop_observation_record_sha256,
                cause,
            }
        };
    let current = match revalidate_observed_runner_scope_fast_v8(&plan.observed, Some(false)) {
        Ok((identities, _)) => identities,
        Err(cause) => {
            return Err(quarantine(
                plan.observed,
                plan.before,
                plan.runner_restore_intent,
                plan.runner_stop_observation_record_sha256,
                cause,
            ));
        }
    };
    if current.len() != plan.before.len()
        || current
            .iter()
            .zip(&plan.before)
            .any(|(left, right)| !left.has_same_stable_identity(right))
        || (!plan.recovery
            && current
                .iter()
                .any(|identity| !identity.state().is_stopped()))
    {
        return Err(quarantine(
            plan.observed,
            plan.before,
            plan.runner_restore_intent,
            plan.runner_stop_observation_record_sha256,
            NativeSysErrorV8::RaceDetected(
                "runner CONT precondition changed after single-use planning".to_string(),
            ),
        ));
    }
    let pgid = match i32::try_from(plan.observed.group.process_group_id()) {
        Ok(pgid) => pgid,
        Err(_) => {
            return Err(quarantine(
                plan.observed,
                plan.before,
                plan.runner_restore_intent,
                plan.runner_stop_observation_record_sha256,
                NativeSysErrorV8::InvalidInput(
                    "runner CONT process group does not fit pid_t".to_string(),
                ),
            ));
        }
    };
    // SAFETY: negative pgid is exact/pidfd-bound; only SIGCONT is admitted.
    if unsafe { libc::kill(-pgid, libc::SIGCONT) } != 0 {
        return Err(quarantine(
            plan.observed,
            plan.before,
            plan.runner_restore_intent,
            plan.runner_stop_observation_record_sha256,
            io_error(
                "issue unique runner scope SIGCONT",
                std::io::Error::last_os_error(),
            ),
        ));
    }
    #[cfg(test)]
    if fault == RunnerContinueFaultV8::AfterIssue {
        return Err(quarantine(
            plan.observed,
            plan.before,
            plan.runner_restore_intent,
            plan.runner_stop_observation_record_sha256,
            NativeSysErrorV8::RaceDetected(
                "injected post-CONT proof fault; signal must not be reissued".to_string(),
            ),
        ));
    }
    let deadline = Instant::now() + MAX_RUNNER_SCOPE_WAIT_V8;
    loop {
        match revalidate_observed_runner_scope_fast_v8(&plan.observed, Some(false)) {
            Ok((after, _)) if after.iter().all(|identity| !identity.state().is_stopped()) => {
                let process_group_id = plan.observed.group.process_group_id();
                let unresolved_runner_stop_intent = plan.observed.runner_stop_intent.take();
                return Ok(RunnerScopeContinueExecutionV8 {
                    observed: plan.observed,
                    process_group_id,
                    before: plan.before,
                    after,
                    recovery: plan.recovery,
                    runner_restore_intent: plan.runner_restore_intent,
                    runner_stop_observation_record_sha256: plan
                        .runner_stop_observation_record_sha256,
                    unresolved_runner_stop_intent,
                });
            }
            Ok(_) if Instant::now() >= deadline => {
                return Err(quarantine(
                    plan.observed,
                    plan.before,
                    plan.runner_restore_intent,
                    plan.runner_stop_observation_record_sha256,
                    NativeSysErrorV8::RaceDetected(
                        "runner CONT was issued but a member remained stopped".to_string(),
                    ),
                ));
            }
            Err(cause) => {
                return Err(quarantine(
                    plan.observed,
                    plan.before,
                    plan.runner_restore_intent,
                    plan.runner_stop_observation_record_sha256,
                    cause,
                ));
            }
            _ => std::thread::sleep(CGROUP_WAIT_POLL_V8),
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn execute_runner_scope_continue_impl_v8(
    plan: RunnerScopeContinuePlanV8,
) -> Result<RunnerScopeContinueExecutionV8, RunnerScopePostContinueQuarantineV8> {
    Err(RunnerScopePostContinueQuarantineV8 {
        observed: plan.observed,
        before: plan.before,
        runner_restore_intent: plan.runner_restore_intent,
        runner_stop_observation_record_sha256: plan.runner_stop_observation_record_sha256,
        cause: unsupported("execute runner scope CONT"),
    })
}

#[cfg(target_os = "linux")]
fn open_existing_systemd_scope_impl_v8(
    control_group: &str,
) -> NativeSysResultV8<ExistingSystemdScopeCgroupV8> {
    validate_absolute_control_group_v8(control_group)?;
    let procfs = super::ProcfsRootV8::open_fixed()?;
    let observer = procfs.observer_binding()?;
    let cgroup_root = open_fixed_unified_cgroup_root_v8()?;
    let cgroup_root_identity = super::openat2::identity_for_fd(cgroup_root.as_raw_fd())?;
    require_unified_cgroup_root_v8(cgroup_root.as_raw_fd(), cgroup_root_identity)?;
    let mount_id =
        procfs.mount_id_for_observer_fd(observer.observer_tid, cgroup_root.as_raw_fd())?;
    if mount_id != observer.cgroup_mount_id {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "existing systemd scope cgroup root differs from numeric mountinfo".to_string(),
        ));
    }
    let relative = control_group.trim_start_matches('/');
    let descriptor = open_beneath_v8(
        cgroup_root.as_raw_fd(),
        relative,
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
    )?;
    let identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
    require_existing_scope_directory_policy_v8(
        descriptor.as_raw_fd(),
        identity,
        ExistingScopeBackendV8::Production,
    )?;
    let scope = ExistingSystemdScopeCgroupV8 {
        cgroup_root,
        cgroup_root_identity,
        descriptor,
        identity,
        observer,
        backend: ExistingScopeBackendV8::Production,
        control_group: control_group.to_string(),
    };
    scope.revalidate()?;
    let cgroup_type =
        parse_cgroup_type_v8(&read_existing_scope_control_v8(&scope, "cgroup.type")?)?;
    let subtree = parse_subtree_control_v8(&read_existing_scope_control_v8(
        &scope,
        "cgroup.subtree_control",
    )?)?;
    let (pids, events) = scope.roster_and_events()?;
    if cgroup_type != CgroupTypeV8::Domain
        || !subtree.is_empty()
        || pids.is_empty()
        || !events.populated
        || events.frozen
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "existing systemd scope is delegated, empty, frozen, or not a domain".to_string(),
        ));
    }
    for control in [
        "cgroup.type",
        "cgroup.subtree_control",
        "cgroup.events",
        "cgroup.procs",
        "cgroup.freeze",
    ] {
        let descriptor = open_beneath_v8(
            scope.descriptor.as_raw_fd(),
            control,
            libc::O_RDONLY | libc::O_CLOEXEC,
        )?;
        require_existing_scope_control_policy_v8(descriptor.as_raw_fd(), scope.backend, control)?;
    }
    Ok(scope)
}

#[cfg(not(target_os = "linux"))]
fn open_existing_systemd_scope_impl_v8(
    _control_group: &str,
) -> NativeSysResultV8<ExistingSystemdScopeCgroupV8> {
    Err(unsupported("open existing systemd scope cgroup"))
}

#[cfg(target_os = "linux")]
fn revalidate_existing_systemd_scope_v8(
    scope: &ExistingSystemdScopeCgroupV8,
) -> NativeSysResultV8<()> {
    let root_identity = super::openat2::identity_for_fd(scope.cgroup_root.as_raw_fd())?;
    if !root_identity.matches_stable_directory(scope.cgroup_root_identity) {
        return Err(NativeSysErrorV8::RaceDetected(
            "existing scope cgroup root identity changed".to_string(),
        ));
    }
    let scope_identity = super::openat2::identity_for_fd(scope.descriptor.as_raw_fd())?;
    if !scope_identity.matches_stable_directory(scope.identity) {
        return Err(NativeSysErrorV8::RaceDetected(
            "existing systemd scope descriptor identity changed or was unlinked".to_string(),
        ));
    }
    require_existing_scope_directory_policy_v8(
        scope.descriptor.as_raw_fd(),
        scope_identity,
        scope.backend,
    )?;
    let named = open_beneath_v8(
        scope.cgroup_root.as_raw_fd(),
        scope.control_group.trim_start_matches('/'),
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
    )?;
    let named_identity = super::openat2::identity_for_fd(named.as_raw_fd())?;
    if !named_identity.matches_stable_directory(scope.identity) {
        return Err(NativeSysErrorV8::RaceDetected(
            "existing systemd scope pathname changed identity".to_string(),
        ));
    }
    match scope.backend {
        ExistingScopeBackendV8::Production => {
            require_unified_cgroup_root_v8(scope.cgroup_root.as_raw_fd(), root_identity)?;
            let procfs = super::ProcfsRootV8::open_fixed()?;
            let observer = procfs.observer_binding()?;
            if observer != scope.observer {
                return Err(NativeSysErrorV8::RaceDetected(
                    "existing scope observer mount/namespace binding changed".to_string(),
                ));
            }
            let mount_id = procfs
                .mount_id_for_observer_fd(observer.observer_tid, scope.cgroup_root.as_raw_fd())?;
            if mount_id != observer.cgroup_mount_id {
                return Err(NativeSysErrorV8::RaceDetected(
                    "existing scope cgroup root mount id changed".to_string(),
                ));
            }
        }
        #[cfg(test)]
        ExistingScopeBackendV8::ExplicitTest => {}
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_existing_scope_directory_policy_v8(
    fd: libc::c_int,
    identity: FileIdentityV8,
    backend: ExistingScopeBackendV8,
) -> NativeSysResultV8<()> {
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and fd remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat existing systemd scope",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFDIR || identity.mode() & 0o022 != 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "existing systemd scope must be a protected directory".to_string(),
        ));
    }
    match backend {
        ExistingScopeBackendV8::Production => {
            if identity.owner_uid() != 0 || identity.owner_gid() != 0 {
                return Err(NativeSysErrorV8::IdentityMismatch(
                    "production existing systemd scope must be root-owned".to_string(),
                ));
            }
            require_cgroup2_magic_v8(fd)
        }
        #[cfg(test)]
        ExistingScopeBackendV8::ExplicitTest => Ok(()),
    }
}

#[cfg(target_os = "linux")]
fn require_existing_scope_control_policy_v8(
    fd: libc::c_int,
    backend: ExistingScopeBackendV8,
    label: &str,
) -> NativeSysResultV8<()> {
    let identity = super::openat2::identity_for_fd(fd)?;
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and fd remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat existing scope control",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFREG || identity.mode() & 0o022 != 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "existing scope control {label} is not a protected regular file"
        )));
    }
    match backend {
        ExistingScopeBackendV8::Production => {
            if identity.owner_uid() != 0 || identity.owner_gid() != 0 {
                return Err(NativeSysErrorV8::IdentityMismatch(format!(
                    "production existing scope control {label} is not root-owned"
                )));
            }
            require_cgroup2_magic_v8(fd)
        }
        #[cfg(test)]
        ExistingScopeBackendV8::ExplicitTest => Ok(()),
    }
}

#[cfg(target_os = "linux")]
fn read_existing_scope_control_v8(
    scope: &ExistingSystemdScopeCgroupV8,
    control: &str,
) -> NativeSysResultV8<Vec<u8>> {
    scope.revalidate()?;
    let descriptor = open_beneath_v8(
        scope.descriptor.as_raw_fd(),
        control,
        libc::O_RDONLY | libc::O_CLOEXEC,
    )?;
    require_existing_scope_control_policy_v8(descriptor.as_raw_fd(), scope.backend, control)?;
    let bytes = read_descriptor_bounded_v8(descriptor.as_raw_fd(), CGROUP_CONTROL_MAX_BYTES_V8)?;
    scope.revalidate()?;
    Ok(bytes)
}

#[cfg(target_os = "linux")]
fn open_existing_scope_freeze_for_write_v8(
    scope: &ExistingSystemdScopeCgroupV8,
) -> NativeSysResultV8<OwnedFd> {
    scope.revalidate()?;
    let descriptor = open_beneath_v8(
        scope.descriptor.as_raw_fd(),
        "cgroup.freeze",
        libc::O_WRONLY | libc::O_CLOEXEC,
    )?;
    require_existing_scope_control_policy_v8(
        descriptor.as_raw_fd(),
        scope.backend,
        "cgroup.freeze",
    )?;
    Ok(descriptor)
}

#[cfg(all(test, target_os = "linux"))]
fn write_existing_scope_test_control_v8(
    scope: &ExistingSystemdScopeCgroupV8,
    control: &str,
    bytes: &[u8],
) -> NativeSysResultV8<()> {
    if scope.backend != ExistingScopeBackendV8::ExplicitTest || control != "cgroup.events" {
        return Err(NativeSysErrorV8::InvalidInput(
            "explicit scope fixture may only synthesize cgroup.events transitions".to_string(),
        ));
    }
    let descriptor = open_beneath_v8(
        scope.descriptor.as_raw_fd(),
        control,
        libc::O_WRONLY | libc::O_CLOEXEC,
    )?;
    require_existing_scope_control_policy_v8(descriptor.as_raw_fd(), scope.backend, control)?;
    // SAFETY: explicit test controls are regular files held through a verified
    // descriptor. Production cgroupfs controls never reach this helper.
    if unsafe { libc::ftruncate(descriptor.as_raw_fd(), 0) } != 0 {
        return Err(io_error(
            "truncate explicit runner scope event fixture",
            std::io::Error::last_os_error(),
        ));
    }
    write_open_control_once_v8(&descriptor, bytes)
}

#[cfg(target_os = "linux")]
fn open_production_cgroup_parent_v8() -> NativeSysResultV8<CgroupRootInnerV8> {
    let procfs = super::ProcfsRootV8::open_fixed()?;
    let observer = procfs.observer_binding()?;
    let anchor = open_fixed_unified_cgroup_root_v8()?;
    let anchor_identity = super::openat2::identity_for_fd(anchor.as_raw_fd())?;
    require_unified_cgroup_root_v8(anchor.as_raw_fd(), anchor_identity)?;
    let cgroup_mount_id =
        procfs.mount_id_for_observer_fd(observer.observer_tid, anchor.as_raw_fd())?;
    if cgroup_mount_id != observer.cgroup_mount_id {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "fixed cgroup descriptor mount id {cgroup_mount_id} differs from numeric mountinfo id {}",
            observer.cgroup_mount_id
        )));
    }
    let self_path = observe_numeric_cgroup_path_v8(&procfs, observer.observer_pid)?;
    if self_path.trim_start_matches('/') != ADMISSIOND_CGROUP_RELATIVE_PATH_V8 {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "current process cgroup {self_path} is not the compiled admissiond service cgroup /{ADMISSIOND_CGROUP_RELATIVE_PATH_V8}"
        )));
    }
    let relative_parent = ADMISSIOND_CGROUP_RELATIVE_PATH_V8.to_string();
    let descriptor = open_beneath_v8(
        anchor.as_raw_fd(),
        &relative_parent,
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
    )?;
    let identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
    let inner = CgroupRootInnerV8 {
        descriptor,
        identity,
        reopen_path: PathBuf::from(ADMISSIOND_CGROUP_ABSOLUTE_PATH_V8),
        backend: CgroupBackendV8::Production,
        production_anchor: Some(CgroupProductionAnchorV8 {
            descriptor: anchor,
            identity: anchor_identity,
            relative_parent,
            observer,
        }),
    };
    require_cgroup_directory_policy_v8(&inner, inner.descriptor.as_raw_fd(), identity)?;
    let parent_pids = parse_cgroup_procs_v8(&read_control_v8(&inner, "cgroup.procs")?)?;
    if !parent_pids.contains(&observer.observer_pid) {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "compiled admissiond service cgroup does not contain the numeric observer pid"
                .to_string(),
        ));
    }
    Ok(inner)
}

#[cfg(target_os = "linux")]
fn open_fixed_unified_cgroup_root_v8() -> NativeSysResultV8<OwnedFd> {
    let path = c"/sys/fs/cgroup";
    // SAFETY: the fixed path remains live and open retains no pointer.
    let raw_fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if raw_fd < 0 {
        return Err(io_error(
            "open fixed unified cgroup root",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: successful open returned a uniquely owned descriptor.
    Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
}

#[cfg(target_os = "linux")]
fn require_unified_cgroup_root_v8(
    fd: libc::c_int,
    identity: FileIdentityV8,
) -> NativeSysResultV8<()> {
    // The unified root deliberately has no cgroup.type file. Its trust boundary
    // is the fixed cgroup2 mount, root ownership, and non-writable mode; all
    // type/controller checks occur at admissiond's delegated service cgroup.
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and fd remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat unified cgroup root",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFDIR
        || identity.owner_uid() != 0
        || identity.owner_gid() != 0
        || identity.mode() & 0o022 != 0
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "fixed unified cgroup root must be a protected root-owned directory".to_string(),
        ));
    }
    require_cgroup2_magic_v8(fd)
}

#[cfg(target_os = "linux")]
fn observe_numeric_cgroup_path_v8(
    procfs: &super::ProcfsRootV8,
    pid: u32,
) -> NativeSysResultV8<String> {
    let relative = format!("{pid}/cgroup");
    let before = procfs.read_regular_beneath(&relative, 4096)?;
    let after = procfs.read_regular_beneath(&relative, 4096)?;
    if before != after {
        return Err(NativeSysErrorV8::RaceDetected(
            "numeric observer cgroup changed across anchored reads".to_string(),
        ));
    }
    let path = parse_self_cgroup_path_v8(&after)?;
    procfs.revalidate()?;
    Ok(path)
}

#[cfg(target_os = "linux")]
fn revalidate_root_inner_v8(root: &CgroupRootInnerV8) -> NativeSysResultV8<()> {
    let observed = super::openat2::identity_for_fd(root.descriptor.as_raw_fd())?;
    if !observed.matches_stable_directory(root.identity) {
        return Err(NativeSysErrorV8::RaceDetected(
            "opened cgroup-v2 root identity changed or was unlinked".to_string(),
        ));
    }
    require_cgroup_directory_policy_v8(root, root.descriptor.as_raw_fd(), observed)?;
    match (&root.backend, &root.production_anchor) {
        (CgroupBackendV8::Production, Some(anchor)) => {
            require_unified_cgroup_root_v8(anchor.descriptor.as_raw_fd(), anchor.identity)?;
            let procfs = super::ProcfsRootV8::open_fixed()?;
            let observer = procfs.observer_binding()?;
            if observer != anchor.observer {
                return Err(NativeSysErrorV8::RaceDetected(
                    "numeric observer proc/cgroup mount or namespace binding changed".to_string(),
                ));
            }
            let cgroup_mount_id = procfs
                .mount_id_for_observer_fd(observer.observer_tid, anchor.descriptor.as_raw_fd())?;
            if cgroup_mount_id != observer.cgroup_mount_id {
                return Err(NativeSysErrorV8::RaceDetected(
                    "retained unified cgroup descriptor changed mount binding".to_string(),
                ));
            }
            let reopened_anchor = open_fixed_unified_cgroup_root_v8()?;
            let reopened_anchor_identity =
                super::openat2::identity_for_fd(reopened_anchor.as_raw_fd())?;
            require_unified_cgroup_root_v8(reopened_anchor.as_raw_fd(), reopened_anchor_identity)?;
            if !reopened_anchor_identity.matches_stable_directory(anchor.identity) {
                return Err(NativeSysErrorV8::RaceDetected(
                    "fixed unified cgroup root identity changed".to_string(),
                ));
            }
            let self_path = observe_numeric_cgroup_path_v8(&procfs, observer.observer_pid)?;
            if self_path.trim_start_matches('/') != ADMISSIOND_CGROUP_RELATIVE_PATH_V8
                || anchor.relative_parent != ADMISSIOND_CGROUP_RELATIVE_PATH_V8
            {
                return Err(NativeSysErrorV8::RaceDetected(
                    "current process left the compiled admissiond service cgroup".to_string(),
                ));
            }
            let named = open_beneath_v8(
                anchor.descriptor.as_raw_fd(),
                &anchor.relative_parent,
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )?;
            let named_identity = super::openat2::identity_for_fd(named.as_raw_fd())?;
            if !named_identity.matches_stable_directory(root.identity) {
                return Err(NativeSysErrorV8::RaceDetected(
                    "compiled admissiond cgroup pathname changed identity".to_string(),
                ));
            }
            let parent_pids = parse_cgroup_procs_v8(&read_control_v8(root, "cgroup.procs")?)?;
            if !parent_pids.contains(&observer.observer_pid) {
                return Err(NativeSysErrorV8::RaceDetected(
                    "numeric observer left the retained admissiond cgroup parent".to_string(),
                ));
            }
        }
        #[cfg(test)]
        (CgroupBackendV8::ExplicitTest, None) => {
            let reopened = open_cgroup_root_v8(&root.reopen_path, root.backend)?;
            if !reopened.identity.matches_stable_directory(root.identity) {
                return Err(NativeSysErrorV8::RaceDetected(
                    "explicit test cgroup root pathname changed identity".to_string(),
                ));
            }
        }
        _ => {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "cgroup root backend/anchor binding is internally inconsistent".to_string(),
            ));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_cgroup_directory_policy_v8(
    root: &CgroupRootInnerV8,
    fd: libc::c_int,
    identity: FileIdentityV8,
) -> NativeSysResultV8<()> {
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and fd remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat cgroup directory",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFDIR || identity.mode() & 0o022 != 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "cgroup directory must be a non-group/world-writable directory".to_string(),
        ));
    }
    match root.backend {
        CgroupBackendV8::Production => {
            if identity.owner_uid() != 0 || identity.owner_gid() != 0 {
                return Err(NativeSysErrorV8::IdentityMismatch(
                    "production cgroup directory must be root-owned".to_string(),
                ));
            }
            require_cgroup2_magic_v8(fd)?;
        }
        #[cfg(test)]
        CgroupBackendV8::ExplicitTest => {
            // SAFETY: geteuid/getegid have no pointer arguments.
            let uid = unsafe { libc::geteuid() };
            // SAFETY: see above.
            let gid = unsafe { libc::getegid() };
            if identity.owner_uid() != uid || identity.owner_gid() != gid {
                return Err(NativeSysErrorV8::IdentityMismatch(
                    "explicit test cgroup directory must be owned by the test process".to_string(),
                ));
            }
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_cgroup2_magic_v8(fd: libc::c_int) -> NativeSysResultV8<()> {
    // SAFETY: zero initializes a writable statfs buffer.
    let mut statfs: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: statfs is writable and fd remains live.
    if unsafe { libc::fstatfs(fd, &mut statfs) } != 0 {
        return Err(io_error(
            "fstatfs cgroup-v2 descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    if statfs.f_type != CGROUP2_SUPER_MAGIC_V8 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "production cgroup descriptor is not on a cgroup2 superblock".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn verify_control_file_policy_v8(
    root: &CgroupRootInnerV8,
    relative: &str,
) -> NativeSysResultV8<()> {
    let descriptor = open_beneath_v8(
        root.descriptor.as_raw_fd(),
        relative,
        libc::O_RDONLY | libc::O_CLOEXEC,
    )?;
    require_control_descriptor_policy_v8(root, descriptor.as_raw_fd(), relative)
}

#[cfg(target_os = "linux")]
fn require_control_descriptor_policy_v8(
    root: &CgroupRootInnerV8,
    fd: libc::c_int,
    label: &str,
) -> NativeSysResultV8<()> {
    let identity = super::openat2::identity_for_fd(fd)?;
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and descriptor remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat cgroup control file",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFREG || identity.mode() & 0o022 != 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "cgroup control file {label} is not a protected regular file"
        )));
    }
    match root.backend {
        CgroupBackendV8::Production => {
            if identity.owner_uid() != 0 || identity.owner_gid() != 0 {
                return Err(NativeSysErrorV8::IdentityMismatch(format!(
                    "production cgroup control file {label} is not root-owned"
                )));
            }
            require_cgroup2_magic_v8(fd)?;
        }
        #[cfg(test)]
        CgroupBackendV8::ExplicitTest => {}
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn open_beneath_v8(
    root_fd: libc::c_int,
    relative: &str,
    flags: libc::c_int,
) -> NativeSysResultV8<OwnedFd> {
    validate_relative_cgroup_path_v8(relative)?;
    let path = CString::new(relative).map_err(|_| {
        NativeSysErrorV8::InvalidInput("cgroup relative path contains NUL".to_string())
    })?;
    // SAFETY: zero is the kernel-defined baseline; fields are set explicitly.
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(flags | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .map_err(|_| NativeSysErrorV8::InvalidInput("invalid cgroup open flags".to_string()))?;
    how.resolve = libc::RESOLVE_BENEATH
        | libc::RESOLVE_NO_SYMLINKS
        | libc::RESOLVE_NO_MAGICLINKS
        | libc::RESOLVE_NO_XDEV;
    // SAFETY: pointers remain live and openat2 retains neither.
    let raw_fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            root_fd,
            path.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if raw_fd < 0 {
        return Err(io_error(
            "openat2 beneath cgroup-v2 root",
            std::io::Error::last_os_error(),
        ));
    }
    let raw_fd = libc::c_int::try_from(raw_fd).map_err(|_| {
        NativeSysErrorV8::InvalidInput("openat2 returned an invalid cgroup descriptor".to_string())
    })?;
    // SAFETY: successful openat2 returned a uniquely owned descriptor.
    Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
}

#[cfg(target_os = "linux")]
fn read_control_v8(root: &CgroupRootInnerV8, relative: &str) -> NativeSysResultV8<Vec<u8>> {
    let descriptor = open_beneath_v8(
        root.descriptor.as_raw_fd(),
        relative,
        libc::O_RDONLY | libc::O_CLOEXEC,
    )?;
    verify_control_file_policy_v8(root, relative)?;
    read_descriptor_bounded_v8(descriptor.as_raw_fd(), CGROUP_CONTROL_MAX_BYTES_V8)
}

#[cfg(target_os = "linux")]
fn read_leaf_control_v8(leaf: &CgroupV2LeafV8, control: &str) -> NativeSysResultV8<Vec<u8>> {
    revalidate_leaf_identity_v8(leaf)?;
    let descriptor = open_beneath_v8(
        leaf.descriptor.as_raw_fd(),
        control,
        libc::O_RDONLY | libc::O_CLOEXEC,
    )?;
    require_control_descriptor_policy_v8(&leaf.root, descriptor.as_raw_fd(), control)?;
    let bytes = read_descriptor_bounded_v8(descriptor.as_raw_fd(), CGROUP_CONTROL_MAX_BYTES_V8)?;
    revalidate_leaf_identity_v8(leaf)?;
    Ok(bytes)
}

#[cfg(target_os = "linux")]
fn read_descriptor_bounded_v8(fd: libc::c_int, maximum: usize) -> NativeSysResultV8<Vec<u8>> {
    let mut output = Vec::new();
    let mut buffer = [0_u8; 4096];
    loop {
        let remaining = maximum.saturating_add(1).saturating_sub(output.len());
        if remaining == 0 {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "cgroup control file exceeds frozen read bound".to_string(),
            ));
        }
        // SAFETY: buffer is writable, descriptor is live, and read retains no
        // pointer. Every control file is freshly opened with offset zero.
        let read =
            unsafe { libc::read(fd, buffer.as_mut_ptr().cast(), remaining.min(buffer.len())) };
        if read < 0 {
            return Err(io_error(
                "read cgroup control file",
                std::io::Error::last_os_error(),
            ));
        }
        if read == 0 {
            break;
        }
        let read = usize::try_from(read).map_err(|_| {
            NativeSysErrorV8::InvalidInput("cgroup read returned invalid length".to_string())
        })?;
        output.extend_from_slice(&buffer[..read]);
    }
    if output.len() > maximum {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "cgroup control file exceeds frozen read bound".to_string(),
        ));
    }
    Ok(output)
}

#[cfg(target_os = "linux")]
fn open_leaf_control_for_write_v8(
    leaf: &CgroupV2LeafV8,
    control: &str,
) -> NativeSysResultV8<OwnedFd> {
    revalidate_leaf_identity_v8(leaf)?;
    let descriptor = open_beneath_v8(
        leaf.descriptor.as_raw_fd(),
        control,
        libc::O_WRONLY | libc::O_CLOEXEC,
    )?;
    require_control_descriptor_policy_v8(&leaf.root, descriptor.as_raw_fd(), control)?;
    Ok(descriptor)
}

#[cfg(target_os = "linux")]
fn write_open_control_once_v8(descriptor: &OwnedFd, bytes: &[u8]) -> NativeSysResultV8<()> {
    if bytes.is_empty() || bytes.len() > 64 {
        return Err(NativeSysErrorV8::InvalidInput(
            "cgroup control write is empty or exceeds frozen bound".to_string(),
        ));
    }
    // SAFETY: bytes and descriptor remain live and write retains no pointer.
    let written =
        unsafe { libc::write(descriptor.as_raw_fd(), bytes.as_ptr().cast(), bytes.len()) };
    if written < 0 {
        return Err(io_error(
            "single write to opened cgroup control file",
            std::io::Error::last_os_error(),
        ));
    }
    if usize::try_from(written).ok() != Some(bytes.len()) {
        return Err(NativeSysErrorV8::RaceDetected(
            "single opened cgroup control write was partial; outcome is ambiguous".to_string(),
        ));
    }
    Ok(())
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_cgroup_type_v8(bytes: &[u8]) -> NativeSysResultV8<CgroupTypeV8> {
    match canonical_control_line_v8(bytes, "cgroup.type")? {
        "domain" => Ok(CgroupTypeV8::Domain),
        "domain threaded" => Ok(CgroupTypeV8::DomainThreaded),
        "domain invalid" => Ok(CgroupTypeV8::DomainInvalid),
        "threaded" => Ok(CgroupTypeV8::Threaded),
        _ => Err(NativeSysErrorV8::IdentityMismatch(
            "cgroup.type contains an unknown value".to_string(),
        )),
    }
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_subtree_control_v8(bytes: &[u8]) -> NativeSysResultV8<Vec<String>> {
    let line = canonical_control_line_v8(bytes, "cgroup.subtree_control")?;
    if line.is_empty() {
        return Ok(Vec::new());
    }
    let mut controllers = BTreeSet::new();
    for controller in line.split(' ') {
        if controller.is_empty()
            || !controller
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "cgroup.subtree_control has a noncanonical controller".to_string(),
            ));
        }
        if !controllers.insert(controller.to_string()) {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "cgroup.subtree_control contains a duplicate controller".to_string(),
            ));
        }
    }
    Ok(controllers.into_iter().collect())
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_cgroup_events_v8(bytes: &[u8]) -> NativeSysResultV8<CgroupEventsV8> {
    let text = canonical_multiline_control_v8(bytes, "cgroup.events")?;
    let mut populated = None;
    let mut frozen = None;
    for line in text.lines() {
        let mut fields = line.split(' ');
        let key = fields.next().unwrap_or_default();
        let value = fields.next().unwrap_or_default();
        if fields.next().is_some() || !matches!(value, "0" | "1") {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "cgroup.events row is not canonical key/bool".to_string(),
            ));
        }
        let parsed = value == "1";
        match key {
            "populated" if populated.replace(parsed).is_none() => {}
            "frozen" if frozen.replace(parsed).is_none() => {}
            _ => {
                return Err(NativeSysErrorV8::IdentityMismatch(
                    "cgroup.events has duplicate or unknown keys".to_string(),
                ));
            }
        }
    }
    Ok(CgroupEventsV8 {
        populated: populated.ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch(
                "cgroup.events lacks exact populated field".to_string(),
            )
        })?,
        frozen: frozen.ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch("cgroup.events lacks exact frozen field".to_string())
        })?,
    })
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_cgroup_procs_v8(bytes: &[u8]) -> NativeSysResultV8<Vec<u32>> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let text = canonical_multiline_control_v8(bytes, "cgroup.procs")?;
    let mut pids = BTreeSet::new();
    for line in text.lines() {
        if line.is_empty()
            || (line.len() > 1 && line.starts_with('0'))
            || !line.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "cgroup.procs contains a noncanonical pid".to_string(),
            ));
        }
        let pid = line.parse::<u32>().map_err(|_| {
            NativeSysErrorV8::IdentityMismatch("cgroup.procs pid does not fit u32".to_string())
        })?;
        if pid == 0 || pid > i32::MAX as u32 || !pids.insert(pid) {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "cgroup.procs contains invalid or duplicate pid".to_string(),
            ));
        }
    }
    Ok(pids.into_iter().collect())
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_self_cgroup_path_v8(bytes: &[u8]) -> NativeSysResultV8<String> {
    if bytes.is_empty()
        || bytes.contains(&0)
        || !bytes.ends_with(b"\n")
        || bytes[..bytes.len() - 1].contains(&b'\n')
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "/proc/self/cgroup must contain exactly one LF-terminated v2 row".to_string(),
        ));
    }
    let row = std::str::from_utf8(&bytes[..bytes.len() - 1]).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch("/proc/self/cgroup row is not ASCII".to_string())
    })?;
    let path = row.strip_prefix("0::").ok_or_else(|| {
        NativeSysErrorV8::IdentityMismatch(
            "/proc/self/cgroup is not an exact unified-v2 0:: row".to_string(),
        )
    })?;
    if !path.starts_with('/')
        || path.len() > 2048
        || path.contains("//")
        || (path != "/"
            && path
                .split('/')
                .skip(1)
                .any(|component| component.is_empty() || matches!(component, "." | "..")))
        || !path.bytes().all(|byte| {
            byte == b'/'
                || byte.is_ascii_alphanumeric()
                || matches!(byte, b'_' | b'-' | b'.' | b'@' | b':' | b'\\')
        })
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "/proc/self/cgroup v2 path is not canonical beneath the unified root".to_string(),
        ));
    }
    Ok(path.to_string())
}

#[cfg(any(target_os = "linux", test))]
fn canonical_control_line_v8<'a>(
    bytes: &'a [u8],
    field: &'static str,
) -> NativeSysResultV8<&'a str> {
    let canonical = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    if canonical.contains(&b'\n') || canonical.contains(&0) {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "{field} must be exactly one NUL-free line"
        )));
    }
    std::str::from_utf8(canonical)
        .map_err(|_| NativeSysErrorV8::IdentityMismatch(format!("{field} is not ASCII")))
}

#[cfg(any(target_os = "linux", test))]
fn canonical_multiline_control_v8<'a>(
    bytes: &'a [u8],
    field: &'static str,
) -> NativeSysResultV8<&'a str> {
    if bytes.is_empty() || bytes.contains(&0) || !bytes.ends_with(b"\n") {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "{field} must be non-empty, NUL-free, and LF-terminated"
        )));
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|_| NativeSysErrorV8::IdentityMismatch(format!("{field} is not ASCII")))?;
    if text.contains("\n\n") {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "{field} contains an empty row"
        )));
    }
    Ok(text)
}

#[cfg(target_os = "linux")]
fn validate_cgroup_leaf_v8(leaf: &str) -> NativeSysResultV8<()> {
    if !leaf.starts_with("hepta-v8-")
        || leaf.len() > 128
        || leaf
            .bytes()
            .any(|byte| !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'))
    {
        return Err(NativeSysErrorV8::InvalidInput(
            "cgroup child must be one lowercase hepta-v8-* leaf of at most 128 bytes".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn lowercase_hex_v8(bytes: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

#[cfg(target_os = "linux")]
fn candidate_child_names_v8(root: &CgroupRootInnerV8) -> NativeSysResultV8<Vec<String>> {
    revalidate_root_inner_v8(root)?;
    let dot = c".";
    // SAFETY: the root descriptor and static name remain live; openat returns
    // an independently offset descriptor on success.
    let enumeration_fd = unsafe {
        libc::openat(
            root.descriptor.as_raw_fd(),
            dot.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if enumeration_fd < 0 {
        return Err(io_error(
            "open independent cgroup parent enumeration descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    let identity = match super::openat2::identity_for_fd(enumeration_fd) {
        Ok(identity) => identity,
        Err(error) => {
            // SAFETY: fdopendir has not consumed the descriptor.
            unsafe { libc::close(enumeration_fd) };
            return Err(error);
        }
    };
    if !identity.matches_stable_directory(root.identity) {
        // SAFETY: fdopendir has not consumed the descriptor.
        unsafe { libc::close(enumeration_fd) };
        return Err(NativeSysErrorV8::RaceDetected(
            "cgroup namespace enumeration descriptor differs from parent".to_string(),
        ));
    }
    // SAFETY: fdopendir consumes the descriptor on success.
    let stream = unsafe { libc::fdopendir(enumeration_fd) };
    if stream.is_null() {
        // SAFETY: fdopendir failed and did not consume the descriptor.
        unsafe { libc::close(enumeration_fd) };
        return Err(io_error(
            "fdopendir candidate cgroup namespace",
            std::io::Error::last_os_error(),
        ));
    }
    let stream = CgroupDirectoryStreamV8(stream);
    let mut candidates = BTreeSet::new();
    let mut total_entries = 0_usize;
    loop {
        // SAFETY: errno is thread local and the stream is exclusively owned.
        unsafe { *libc::__errno_location() = 0 };
        // SAFETY: stream remains live for the readdir call.
        let entry = unsafe { libc::readdir(stream.0) };
        if entry.is_null() {
            // SAFETY: errno is read immediately from this thread.
            let errno = unsafe { *libc::__errno_location() };
            if errno != 0 {
                return Err(io_error(
                    "readdir candidate cgroup namespace",
                    std::io::Error::from_raw_os_error(errno),
                ));
            }
            break;
        }
        total_entries = total_entries.checked_add(1).ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch(
                "candidate cgroup namespace entry count overflow".to_string(),
            )
        })?;
        if total_entries > 4096 {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "candidate cgroup namespace exceeds 4096 entries".to_string(),
            ));
        }
        // SAFETY: d_name is NUL-terminated and valid until the next readdir.
        let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
        if !name.starts_with(b"hepta-v8-") {
            continue;
        }
        let name = std::str::from_utf8(name).map_err(|_| {
            NativeSysErrorV8::IdentityMismatch(
                "candidate cgroup name is not canonical ASCII".to_string(),
            )
        })?;
        validate_cgroup_leaf_v8(name)?;
        if !candidates.insert(name.to_string()) {
            return Err(NativeSysErrorV8::RaceDetected(
                "candidate cgroup enumeration returned a duplicate name".to_string(),
            ));
        }
    }
    revalidate_root_inner_v8(root)?;
    Ok(candidates.into_iter().collect())
}

#[cfg(target_os = "linux")]
struct CgroupDirectoryStreamV8(*mut libc::DIR);

#[cfg(target_os = "linux")]
impl Drop for CgroupDirectoryStreamV8 {
    fn drop(&mut self) {
        // SAFETY: wrapper uniquely owns the live DIR pointer.
        unsafe { libc::closedir(self.0) };
    }
}

#[cfg(target_os = "linux")]
fn validate_relative_cgroup_path_v8(relative: &str) -> NativeSysResultV8<()> {
    if relative.is_empty()
        || relative.starts_with('/')
        || relative
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(NativeSysErrorV8::InvalidInput(
            "cgroup relative path is not canonical beneath its anchor".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn validate_absolute_control_group_v8(control_group: &str) -> NativeSysResultV8<()> {
    if !control_group.starts_with('/')
        || control_group == "/"
        || control_group.len() > 2048
        || control_group.contains("//")
        || control_group
            .split('/')
            .skip(1)
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
        || !control_group.bytes().all(|byte| {
            byte == b'/'
                || byte.is_ascii_alphanumeric()
                || matches!(byte, b'_' | b'-' | b'.' | b'@' | b':' | b'\\')
        })
    {
        return Err(NativeSysErrorV8::InvalidInput(
            "systemd ControlGroup is not one canonical absolute cgroup-v2 path".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn is_enoent_v8(error: &NativeSysErrorV8) -> bool {
    matches!(
        error,
        NativeSysErrorV8::Io { source, .. } if source.raw_os_error() == Some(libc::ENOENT)
    )
}

#[cfg(all(test, target_os = "linux"))]
pub(super) fn open_explicit_test_cgroup_root_v8(path: &Path) -> NativeSysResultV8<CgroupV2RootV8> {
    if path.starts_with("/sys/fs/cgroup") {
        return Err(NativeSysErrorV8::InvalidInput(
            "explicit test cgroup root may not be the production cgroup namespace".to_string(),
        ));
    }
    let inner = open_cgroup_root_v8(path, CgroupBackendV8::ExplicitTest)?;
    build_cgroup_root_proof_v8(inner)
}

#[cfg(all(test, target_os = "linux"))]
pub(super) fn open_explicit_test_systemd_scope_v8(
    root_path: &Path,
    control_group: &str,
) -> NativeSysResultV8<ExistingSystemdScopeCgroupV8> {
    if root_path.starts_with("/sys/fs/cgroup") {
        return Err(NativeSysErrorV8::InvalidInput(
            "explicit runner scope fixture may not be the production cgroup namespace".to_string(),
        ));
    }
    validate_absolute_control_group_v8(control_group)?;
    let root_c = CString::new(root_path.as_os_str().as_bytes()).map_err(|_| {
        NativeSysErrorV8::InvalidInput("explicit runner scope root contains NUL".to_string())
    })?;
    // SAFETY: path remains live; open retains no pointer.
    let root_fd = unsafe {
        libc::open(
            root_c.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if root_fd < 0 {
        return Err(io_error(
            "open explicit runner scope root",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: successful open returned a uniquely owned descriptor.
    let cgroup_root = unsafe { OwnedFd::from_raw_fd(root_fd) };
    let cgroup_root_identity = super::openat2::identity_for_fd(cgroup_root.as_raw_fd())?;
    if cgroup_root_identity.mode() & 0o022 != 0 || cgroup_root_identity.link_count() == 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "explicit runner scope root is writable by group/other or unlinked".to_string(),
        ));
    }
    let descriptor = open_beneath_v8(
        cgroup_root.as_raw_fd(),
        control_group.trim_start_matches('/'),
        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
    )?;
    let identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
    require_existing_scope_directory_policy_v8(
        descriptor.as_raw_fd(),
        identity,
        ExistingScopeBackendV8::ExplicitTest,
    )?;
    let procfs = super::ProcfsRootV8::open_fixed()?;
    let observer = procfs.observer_binding()?;
    let scope = ExistingSystemdScopeCgroupV8 {
        cgroup_root,
        cgroup_root_identity,
        descriptor,
        identity,
        observer,
        backend: ExistingScopeBackendV8::ExplicitTest,
        control_group: control_group.to_string(),
    };
    scope.revalidate()?;
    for control in [
        "cgroup.type",
        "cgroup.subtree_control",
        "cgroup.events",
        "cgroup.procs",
        "cgroup.freeze",
    ] {
        let descriptor = open_beneath_v8(
            scope.descriptor.as_raw_fd(),
            control,
            libc::O_RDONLY | libc::O_CLOEXEC,
        )?;
        require_existing_scope_control_policy_v8(descriptor.as_raw_fd(), scope.backend, control)?;
    }
    let cgroup_type =
        parse_cgroup_type_v8(&read_existing_scope_control_v8(&scope, "cgroup.type")?)?;
    let subtree = parse_subtree_control_v8(&read_existing_scope_control_v8(
        &scope,
        "cgroup.subtree_control",
    )?)?;
    let (pids, events) = scope.roster_and_events()?;
    if cgroup_type != CgroupTypeV8::Domain
        || !subtree.is_empty()
        || pids.is_empty()
        || !events.populated
        || events.frozen
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "explicit runner scope fixture is delegated, empty, frozen, or not a domain"
                .to_string(),
        ));
    }
    Ok(scope)
}

#[cfg(all(test, target_os = "linux"))]
pub(super) fn observe_explicit_test_runner_scope_v8(
    scope: ExistingSystemdScopeCgroupV8,
    expected_pids: &[u32],
) -> NativeSysResultV8<ObservedRunnerScopeV8> {
    if scope.backend != ExistingScopeBackendV8::ExplicitTest {
        return Err(NativeSysErrorV8::InvalidInput(
            "explicit runner observation requires the regular-file scope backend".to_string(),
        ));
    }
    let mut expected = expected_pids.to_vec();
    expected.sort_unstable();
    if expected != expected_pids || expected.len() < 2 {
        return Err(NativeSysErrorV8::InvalidInput(
            "explicit runner scope pids must be sorted and contain at least two members"
                .to_string(),
        ));
    }
    let (roster, events) = scope.roster_and_events()?;
    if roster != expected || !events.populated || events.frozen {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "explicit runner scope roster/events differ from the requested closure".to_string(),
        ));
    }
    let group = super::observe_exact_process_group_v8(&expected)?;
    let runtime = group.observe_runtime_closure_v8()?;
    let initial = group.revalidate_fast_for_scope_v8()?;
    let processes = runtime
        .into_iter()
        .map(|binding| RunnerScopeProcessManifestV8 {
            pid: binding.identity.pid(),
            start_ticks: binding.identity.start_ticks(),
            parent_pid: binding.identity.parent_pid(),
            uid: binding.runtime.uid,
            gid: binding.runtime.gid,
            process_group_id: binding.identity.process_group_id(),
            session_id: binding.identity.session_id(),
            executable_sha256: binding.identity.executable().sha256(),
            argv_sha256: binding.runtime.argv_sha256,
            cwd_device: binding.runtime.cwd_device,
            cwd_inode: binding.runtime.cwd_inode,
        })
        .collect();
    let manifest = RunnerScopeTrustedManifestV8 {
        boot_id: super::observe_boot_id_v8()?,
        unit_name: "explicit-runner-scope-fixture.service".to_string(),
        delegate: false,
        control_group: scope.control_group.clone(),
        main_pid: expected[0],
        fragment: TrustedRunnerArtifactV8 {
            absolute_path: "/explicit-test-only/systemd-fragment".to_string(),
            device: 1,
            inode: 1,
            size: 1,
            mode: 0o600,
            owner_uid: 0,
            owner_gid: 0,
            sha256: [1; 32],
        },
        workroot: TrustedRunnerDirectoryV8 {
            absolute_path: "/explicit-test-only/workroot".to_string(),
            device: 1,
            inode: 1,
            mode: 0o700,
            owner_uid: 0,
            owner_gid: 0,
        },
        artifacts: Vec::new(),
        processes,
    };
    Ok(ObservedRunnerScopeV8 {
        manifest,
        scope,
        group,
        initial,
        runner_stop_intent: None,
    })
}

#[cfg(all(test, target_os = "linux"))]
fn materialize_test_cgroup_files_v8(root: &CgroupRootInnerV8, leaf: &str) -> NativeSysResultV8<()> {
    for (name, bytes) in [
        ("cgroup.type", b"domain\n".as_slice()),
        ("cgroup.subtree_control", b"".as_slice()),
        ("cgroup.events", b"populated 0\nfrozen 0\n".as_slice()),
        ("cgroup.procs", b"".as_slice()),
        ("cgroup.kill", b"".as_slice()),
    ] {
        let relative = format!("{leaf}/{name}");
        let path = CString::new(relative.as_str()).map_err(|_| {
            NativeSysErrorV8::InvalidInput("test cgroup path contains NUL".to_string())
        })?;
        // SAFETY: path and bytes remain live; openat retains no pointers.
        let fd = unsafe {
            libc::openat(
                root.descriptor.as_raw_fd(),
                path.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o644,
            )
        };
        if fd < 0 {
            return Err(io_error(
                "create explicit test cgroup control file",
                std::io::Error::last_os_error(),
            ));
        }
        // SAFETY: successful openat returned a uniquely owned descriptor.
        let descriptor = unsafe { OwnedFd::from_raw_fd(fd) };
        if !bytes.is_empty() {
            // SAFETY: bytes and descriptor remain live.
            let written =
                unsafe { libc::write(descriptor.as_raw_fd(), bytes.as_ptr().cast(), bytes.len()) };
            if usize::try_from(written).ok() != Some(bytes.len()) {
                return Err(io_error(
                    "write explicit test cgroup control file",
                    std::io::Error::last_os_error(),
                ));
            }
        }
    }
    Ok(())
}

#[cfg(all(test, target_os = "linux"))]
fn populate_candidate_for_test_impl_v8(
    candidate: CandidateCgroupLeafV8,
    process: &ProcessObservationV8,
) -> Result<CandidateCgroupRunningV8, CandidateCgroupTestPopulationFailureV8> {
    let fail =
        |candidate, effect_issued_or_uncertain, cause| CandidateCgroupTestPopulationFailureV8 {
            candidate,
            effect_issued_or_uncertain,
            cause,
        };
    if candidate.leaf.root.backend != CgroupBackendV8::ExplicitTest {
        return Err(fail(
            candidate,
            false,
            NativeSysErrorV8::InvalidInput(
                "numeric candidate population is restricted to explicit regular-file fixtures"
                    .to_string(),
            ),
        ));
    }
    if let Err(cause) = candidate.lease.revalidate() {
        return Err(fail(candidate, false, cause));
    }
    if let Err(cause) = revalidate_empty_leaf_impl_v8(&candidate.leaf) {
        return Err(fail(candidate, false, cause));
    }
    // SAFETY: getpid has no pointer arguments or preconditions.
    let self_pid = unsafe { libc::getpid() };
    if u32::try_from(self_pid).ok() == Some(process.pid()) {
        return Err(fail(
            candidate,
            false,
            NativeSysErrorV8::InvalidInput(
                "explicit candidate fixture may not target its controller process".to_string(),
            ),
        ));
    }
    let before = match process.revalidate() {
        Ok(identity) => identity,
        Err(cause) => return Err(fail(candidate, false, cause)),
    };
    let procs = format!("{}\n", before.pid());
    if let Err(cause) =
        write_leaf_test_control_v8(&candidate.leaf, "cgroup.procs", procs.as_bytes())
    {
        return Err(fail(candidate, true, cause));
    }
    if let Err(cause) =
        write_leaf_test_control_v8(&candidate.leaf, "cgroup.events", b"populated 1\nfrozen 0\n")
    {
        return Err(fail(candidate, true, cause));
    }
    let after = match process.revalidate() {
        Ok(identity) => identity,
        Err(cause) => return Err(fail(candidate, true, cause)),
    };
    if !before.has_same_stable_identity(&after) {
        return Err(fail(
            candidate,
            true,
            NativeSysErrorV8::RaceDetected(
                "explicit candidate fixture process changed across simulated population"
                    .to_string(),
            ),
        ));
    }
    match read_leaf_control_v8(&candidate.leaf, "cgroup.procs")
        .and_then(|bytes| parse_cgroup_procs_v8(&bytes))
    {
        Ok(pids) if pids == [before.pid()] => {}
        Ok(_) => {
            return Err(fail(
                candidate,
                true,
                NativeSysErrorV8::RaceDetected(
                    "explicit candidate fixture did not retain exact membership".to_string(),
                ),
            ));
        }
        Err(cause) => return Err(fail(candidate, true, cause)),
    }
    Ok(CandidateCgroupRunningV8 {
        candidate,
        child_pid: before.pid(),
        child: None,
    })
}

#[cfg(all(test, target_os = "linux"))]
fn write_leaf_test_control_v8(
    leaf: &CgroupV2LeafV8,
    control: &str,
    bytes: &[u8],
) -> NativeSysResultV8<()> {
    if leaf.root.backend != CgroupBackendV8::ExplicitTest
        || !matches!(control, "cgroup.procs" | "cgroup.events")
    {
        return Err(NativeSysErrorV8::InvalidInput(
            "explicit candidate fixture control is not admitted".to_string(),
        ));
    }
    let descriptor = open_beneath_v8(
        leaf.descriptor.as_raw_fd(),
        control,
        libc::O_WRONLY | libc::O_CLOEXEC,
    )?;
    require_control_descriptor_policy_v8(&leaf.root, descriptor.as_raw_fd(), control)?;
    // SAFETY: explicit fixture controls are regular files, never cgroupfs.
    if unsafe { libc::ftruncate(descriptor.as_raw_fd(), 0) } != 0 {
        return Err(io_error(
            "truncate explicit candidate fixture control",
            std::io::Error::last_os_error(),
        ));
    }
    write_open_control_once_v8(&descriptor, bytes)
}

#[cfg(all(test, target_os = "linux"))]
fn unlink_leaf_control_for_test_v8(leaf: &CgroupV2LeafV8, control: &str) -> NativeSysResultV8<()> {
    let path = CString::new(control).map_err(|_| {
        NativeSysErrorV8::InvalidInput("test cgroup control leaf contains NUL".to_string())
    })?;
    // SAFETY: the exact leaf descriptor and control name remain live;
    // unlinkat retains neither.
    if unsafe { libc::unlinkat(leaf.descriptor.as_raw_fd(), path.as_ptr(), 0) } != 0 {
        return Err(io_error(
            "unlink explicit test control relative to exact cgroup leaf",
            std::io::Error::last_os_error(),
        ));
    }
    Ok(())
}
