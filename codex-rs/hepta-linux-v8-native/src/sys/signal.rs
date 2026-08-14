use super::NativeSysErrorV8;
use super::NativeSysResultV8;
use super::ProcessIdentityV8;
use super::ProcessObservationV8;
#[cfg(target_os = "linux")]
use super::ProcessRuntimeBindingV8;

#[cfg(target_os = "linux")]
use super::ProcessCaptureBudgetV8;
#[cfg(target_os = "linux")]
use super::ProcfsRootV8;
#[cfg(target_os = "linux")]
use super::observe_process_exact_with_root_and_budget_v8;
#[cfg(not(target_os = "linux"))]
use super::unsupported;
#[cfg(target_os = "linux")]
use std::collections::BTreeSet;
#[cfg(target_os = "linux")]
use std::time::Duration;
#[cfg(target_os = "linux")]
use std::time::Instant;

#[cfg(target_os = "linux")]
const MIN_SHARED_PROCESS_GROUP_MEMBERS_V8: usize = 2;
#[cfg(target_os = "linux")]
const MAX_SHARED_PROCESS_GROUP_MEMBERS_V8: usize = 128;
#[cfg(target_os = "linux")]
const SIGNAL_STATE_TIMEOUT_V8: Duration = Duration::from_secs(5);
#[cfg(target_os = "linux")]
const SIGNAL_STATE_POLL_V8: Duration = Duration::from_millis(10);
#[cfg(target_os = "linux")]
const MAX_GROUP_EXECUTABLE_HASH_BYTES_V8: u64 = 1024 * 1024 * 1024;

/// The only signals admitted by the process-group controller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessGroupSignalV8 {
    Stop,
    Continue,
}

/// Exact complete process-group observation backed by one pidfd per member.
/// Construction enumerates the fixed procfs mount before and after capture;
/// caller-supplied partial membership is rejected.
#[derive(Debug)]
pub struct ProcessGroupObservationV8 {
    process_group_id: u32,
    session_id: u32,
    members: Vec<ProcessObservationV8>,
}

#[cfg(target_os = "linux")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProcessGroupRuntimeMemberBindingV8 {
    pub(crate) identity: ProcessIdentityV8,
    pub(crate) runtime: ProcessRuntimeBindingV8,
}

impl ProcessGroupObservationV8 {
    pub fn process_group_id(&self) -> u32 {
        self.process_group_id
    }

    pub fn session_id(&self) -> u32 {
        self.session_id
    }

    pub fn member_identities(&self) -> Vec<ProcessIdentityV8> {
        self.members
            .iter()
            .map(|member| member.identity().clone())
            .collect()
    }

    pub fn revalidate(&self) -> NativeSysResultV8<Vec<ProcessIdentityV8>> {
        revalidate_complete_group_impl_v8(self)
    }

    /// Metadata/pidfd-only closure check for post-effect runner-scope states.
    /// It never recomputes an ELF digest.
    #[cfg(target_os = "linux")]
    pub(crate) fn revalidate_fast_for_scope_v8(&self) -> NativeSysResultV8<Vec<ProcessIdentityV8>> {
        revalidate_scope_closure_fast_v8(self)
    }

    pub(crate) fn member_pids_v8(&self) -> Vec<u32> {
        self.members.iter().map(ProcessObservationV8::pid).collect()
    }

    /// Pre-effect-only exact runner closure: the global PGID set, global SID
    /// set and caller-bound pidfd roster must all be identical, then UID/GID,
    /// argv and cwd identities are captured for every member.
    #[cfg(target_os = "linux")]
    pub(crate) fn observe_runtime_closure_v8(
        &self,
    ) -> NativeSysResultV8<Vec<ProcessGroupRuntimeMemberBindingV8>> {
        let root = ProcfsRootV8::open_fixed()?;
        let expected = self.member_pids_v8();
        let (group_before, session_before) =
            root.process_group_and_session_pids(self.process_group_id, self.session_id)?;
        if group_before != expected || session_before != expected {
            return Err(NativeSysErrorV8::RaceDetected(
                "runner scope is not the complete global PGID and SID closure".to_string(),
            ));
        }
        let mut bindings = Vec::with_capacity(self.members.len());
        for member in &self.members {
            bindings.push(ProcessGroupRuntimeMemberBindingV8 {
                identity: member.revalidate_fast_with_root(&root)?,
                runtime: root.observe_process_runtime_binding_v8(member)?,
            });
        }
        bindings.sort_by_key(|binding| binding.identity.pid());
        let (group_after, session_after) =
            root.process_group_and_session_pids(self.process_group_id, self.session_id)?;
        if group_after != expected || session_after != expected {
            return Err(NativeSysErrorV8::RaceDetected(
                "runner global PGID/SID closure changed during runtime binding".to_string(),
            ));
        }
        Ok(bindings)
    }
}

/// Opaque single-use plan. There is no API accepting a raw signal number or a
/// raw negative PID, and consumption prevents duplicate execution in safe Rust.
#[derive(Debug)]
pub struct ProcessGroupSignalPlanV8 {
    signal: ProcessGroupSignalV8,
    group: ProcessGroupObservationV8,
    planned_snapshot: Vec<ProcessIdentityV8>,
    #[cfg(target_os = "linux")]
    restore_after_issued_stop: bool,
}

impl ProcessGroupSignalPlanV8 {
    pub fn signal(&self) -> ProcessGroupSignalV8 {
        self.signal
    }

    pub fn process_group_id(&self) -> u32 {
        self.group.process_group_id
    }

    pub fn planned_snapshot(&self) -> &[ProcessIdentityV8] {
        &self.planned_snapshot
    }
}

/// Before/after identities returned only after the group signal, full-group
/// enumeration, pidfd checks, and target-state transition all succeeded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessGroupSignalExecutionV8 {
    signal: ProcessGroupSignalV8,
    process_group_id: u32,
    before: Vec<ProcessIdentityV8>,
    after: Vec<ProcessIdentityV8>,
}

/// STOP success retains every pidfd and is the normal single-use route to the
/// one permitted SIGCONT. Cloned identities alone are never a restore token.
#[derive(Debug)]
pub struct StoppedProcessGroupV8 {
    group: ProcessGroupObservationV8,
    stop_execution: ProcessGroupSignalExecutionV8,
}

impl StoppedProcessGroupV8 {
    pub fn stop_execution(&self) -> &ProcessGroupSignalExecutionV8 {
        &self.stop_execution
    }

    pub fn member_identities(&self) -> Vec<ProcessIdentityV8> {
        self.group.member_identities()
    }
}

#[derive(Debug)]
pub enum ProcessGroupSignalSuccessV8 {
    Stopped(StoppedProcessGroupV8),
    Continued(ProcessGroupSignalExecutionV8),
}

/// Failure before `kill(-pgid, signal)` retains the still-unexecuted plan.
#[derive(Debug)]
pub struct PreSignalFailureV8 {
    plan: ProcessGroupSignalPlanV8,
    cause: NativeSysErrorV8,
}

impl PreSignalFailureV8 {
    pub fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub fn into_plan(self) -> ProcessGroupSignalPlanV8 {
        self.plan
    }
}

/// STOP was definitely issued. The exact group/pidfds remain owned here until
/// the obligation is consumed into the unique CONT recovery path.
#[derive(Debug)]
pub struct PostStopRestoreObligationV8 {
    group: ProcessGroupObservationV8,
    before: Vec<ProcessIdentityV8>,
    cause: NativeSysErrorV8,
}

impl PostStopRestoreObligationV8 {
    pub fn signal_issued(&self) -> bool {
        true
    }

    pub fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub fn before(&self) -> &[ProcessIdentityV8] {
        &self.before
    }

    pub fn member_identities(&self) -> Vec<ProcessIdentityV8> {
        self.group.member_identities()
    }
}

/// CONT was definitely issued but final proof failed. It preserves the exact
/// pidfds for quarantine/forensics and deliberately exposes no implicit resend.
#[derive(Debug)]
pub struct PostContinueRecoveryObligationV8 {
    group: ProcessGroupObservationV8,
    before: Vec<ProcessIdentityV8>,
    cause: NativeSysErrorV8,
}

impl PostContinueRecoveryObligationV8 {
    pub fn signal_issued(&self) -> bool {
        true
    }

    pub fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub fn before(&self) -> &[ProcessIdentityV8] {
        &self.before
    }

    pub fn member_identities(&self) -> Vec<ProcessIdentityV8> {
        self.group.member_identities()
    }

    pub fn revalidate_for_quarantine(&self) -> NativeSysResultV8<Vec<ProcessIdentityV8>> {
        self.group.revalidate()
    }
}

#[derive(Debug)]
pub enum ProcessGroupSignalFailureV8 {
    BeforeSignal(PreSignalFailureV8),
    PostStop(PostStopRestoreObligationV8),
    PostContinue(PostContinueRecoveryObligationV8),
}

/// Planning failures retain the owning token, so even a failed revalidation
/// cannot accidentally discard the only safe restore route.
#[derive(Debug)]
pub struct StoppedContinuePlanFailureV8 {
    stopped: StoppedProcessGroupV8,
    cause: NativeSysErrorV8,
}

impl StoppedContinuePlanFailureV8 {
    pub fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub fn into_stopped(self) -> StoppedProcessGroupV8 {
        self.stopped
    }
}

#[derive(Debug)]
pub struct RestoreContinuePlanFailureV8 {
    obligation: PostStopRestoreObligationV8,
    cause: NativeSysErrorV8,
}

impl RestoreContinuePlanFailureV8 {
    pub fn cause(&self) -> &NativeSysErrorV8 {
        &self.cause
    }

    pub fn into_obligation(self) -> PostStopRestoreObligationV8 {
        self.obligation
    }
}

impl ProcessGroupSignalExecutionV8 {
    pub fn signal(&self) -> ProcessGroupSignalV8 {
        self.signal
    }

    pub fn process_group_id(&self) -> u32 {
        self.process_group_id
    }

    pub fn before(&self) -> &[ProcessIdentityV8] {
        &self.before
    }

    pub fn after(&self) -> &[ProcessIdentityV8] {
        &self.after
    }
}

/// Captures exactly the caller-bound membership of one shared process group.
/// A singleton group, PID 0/1, the calling process, the caller's own process
/// group, duplicate PIDs, mixed PGIDs/sessions, or any unlisted live group
/// member is rejected.
pub fn observe_exact_process_group_v8(
    expected_pids: &[u32],
) -> NativeSysResultV8<ProcessGroupObservationV8> {
    observe_exact_process_group_impl_v8(expected_pids)
}

/// Revalidates a complete exact group and creates a single-use STOP/CONT plan.
/// STOP rejects an already-stopped member; CONT requires every member stopped.
pub fn plan_stop_process_group_v8(
    group: ProcessGroupObservationV8,
) -> NativeSysResultV8<ProcessGroupSignalPlanV8> {
    plan_stop_process_group_impl_v8(group)
}

/// Consumes a successful STOP token into its unique normal CONT plan. If
/// revalidation fails, the error retains and returns the stopped token.
pub fn plan_continue_stopped_process_group_v8(
    stopped: StoppedProcessGroupV8,
) -> Result<ProcessGroupSignalPlanV8, StoppedContinuePlanFailureV8> {
    plan_continue_stopped_impl_v8(stopped)
}

/// Consumes a post-STOP failure obligation into the unique recovery CONT plan.
/// Mixed running/stopped states are accepted because STOP may have taken effect
/// only partially; dead/zombie or identity drift still fail with the obligation
/// retained in the error.
pub fn plan_restore_post_stop_v8(
    obligation: PostStopRestoreObligationV8,
) -> Result<ProcessGroupSignalPlanV8, RestoreContinuePlanFailureV8> {
    plan_restore_post_stop_impl_v8(obligation)
}

/// Revalidates immediately before issuing exactly one `kill(-pgid, signal)`
/// syscall, then polls exact full-group snapshots until every member reaches
/// the required state. This primitive conveys no operator or run authority.
pub fn execute_process_group_signal_v8(
    plan: ProcessGroupSignalPlanV8,
) -> Result<ProcessGroupSignalSuccessV8, ProcessGroupSignalFailureV8> {
    execute_process_group_signal_impl_v8(plan)
}

#[cfg(target_os = "linux")]
fn observe_exact_process_group_impl_v8(
    expected_pids: &[u32],
) -> NativeSysResultV8<ProcessGroupObservationV8> {
    // SAFETY: getpid/getpgrp have no pointer arguments or preconditions.
    let current_pid = unsafe { libc::getpid() };
    // SAFETY: see above.
    let current_group = unsafe { libc::getpgrp() };
    let current_pid = u32::try_from(current_pid).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch("current pid is not a positive u32".to_string())
    })?;
    let current_group = u32::try_from(current_group).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch(
            "current process group is not a positive u32".to_string(),
        )
    })?;
    let expected = validate_expected_group_inputs_v8(expected_pids, current_pid)?;

    let root = ProcfsRootV8::open_fixed()?;
    let mut budget = ProcessCaptureBudgetV8::new(
        expected.len(),
        MAX_GROUP_EXECUTABLE_HASH_BYTES_V8,
        Duration::from_secs(10),
    )?;
    let mut members = Vec::with_capacity(expected.len());
    for pid in &expected {
        members.push(observe_process_exact_with_root_and_budget_v8(
            &root,
            *pid,
            &mut budget,
        )?);
    }
    members.sort_by_key(ProcessObservationV8::pid);
    let first = members.first().ok_or_else(|| {
        NativeSysErrorV8::InvalidInput("shared process group is empty".to_string())
    })?;
    let process_group_id = first.identity().process_group_id();
    let session_id = first.identity().session_id();
    if process_group_id <= 1 || process_group_id == current_group {
        return Err(NativeSysErrorV8::InvalidInput(
            "target process group must be greater than one and distinct from the caller group"
                .to_string(),
        ));
    }
    for member in &members {
        if member.identity().process_group_id() != process_group_id {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "target members do not share one exact process group".to_string(),
            ));
        }
        if member.identity().session_id() != session_id {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "target process-group members do not share one session".to_string(),
            ));
        }
    }

    let group = ProcessGroupObservationV8 {
        process_group_id,
        session_id,
        members,
    };
    let observed = revalidate_complete_group_impl_v8(&group)?;
    let observed_pids: Vec<u32> = observed.iter().map(ProcessIdentityV8::pid).collect();
    if observed_pids != expected {
        return Err(NativeSysErrorV8::RaceDetected(
            "complete process-group membership differs from caller-bound membership".to_string(),
        ));
    }
    Ok(group)
}

#[cfg(not(target_os = "linux"))]
fn observe_exact_process_group_impl_v8(
    _expected_pids: &[u32],
) -> NativeSysResultV8<ProcessGroupObservationV8> {
    Err(unsupported("observe exact shared process group"))
}

#[cfg(target_os = "linux")]
fn plan_stop_process_group_impl_v8(
    group: ProcessGroupObservationV8,
) -> NativeSysResultV8<ProcessGroupSignalPlanV8> {
    let signal = ProcessGroupSignalV8::Stop;
    let planned_snapshot = revalidate_complete_group_impl_v8(&group)?;
    require_signal_precondition_v8(signal, &planned_snapshot)?;
    Ok(ProcessGroupSignalPlanV8 {
        signal,
        group,
        planned_snapshot,
        restore_after_issued_stop: false,
    })
}

#[cfg(not(target_os = "linux"))]
fn plan_stop_process_group_impl_v8(
    _group: ProcessGroupObservationV8,
) -> NativeSysResultV8<ProcessGroupSignalPlanV8> {
    Err(unsupported("plan exact process-group STOP"))
}

#[cfg(target_os = "linux")]
fn plan_continue_stopped_impl_v8(
    stopped: StoppedProcessGroupV8,
) -> Result<ProcessGroupSignalPlanV8, StoppedContinuePlanFailureV8> {
    let StoppedProcessGroupV8 {
        group,
        stop_execution,
    } = stopped;
    let planned_snapshot = match revalidate_complete_group_impl_v8(&group) {
        Ok(snapshot) => snapshot,
        Err(cause) => {
            return Err(StoppedContinuePlanFailureV8 {
                stopped: StoppedProcessGroupV8 {
                    group,
                    stop_execution,
                },
                cause,
            });
        }
    };
    if let Err(cause) =
        require_signal_precondition_v8(ProcessGroupSignalV8::Continue, &planned_snapshot)
    {
        return Err(StoppedContinuePlanFailureV8 {
            stopped: StoppedProcessGroupV8 {
                group,
                stop_execution,
            },
            cause,
        });
    }
    Ok(ProcessGroupSignalPlanV8 {
        signal: ProcessGroupSignalV8::Continue,
        group,
        planned_snapshot,
        restore_after_issued_stop: false,
    })
}

#[cfg(not(target_os = "linux"))]
fn plan_continue_stopped_impl_v8(
    stopped: StoppedProcessGroupV8,
) -> Result<ProcessGroupSignalPlanV8, StoppedContinuePlanFailureV8> {
    Err(StoppedContinuePlanFailureV8 {
        stopped,
        cause: unsupported("plan CONT from stopped process-group token"),
    })
}

#[cfg(target_os = "linux")]
fn plan_restore_post_stop_impl_v8(
    obligation: PostStopRestoreObligationV8,
) -> Result<ProcessGroupSignalPlanV8, RestoreContinuePlanFailureV8> {
    let PostStopRestoreObligationV8 {
        group,
        before,
        cause: original_cause,
    } = obligation;
    let planned_snapshot = match revalidate_complete_group_impl_v8(&group) {
        Ok(snapshot) => snapshot,
        Err(cause) => {
            return Err(RestoreContinuePlanFailureV8 {
                obligation: PostStopRestoreObligationV8 {
                    group,
                    before,
                    cause: original_cause,
                },
                cause,
            });
        }
    };
    if let Err(cause) = require_recovery_continue_precondition_v8(&planned_snapshot) {
        return Err(RestoreContinuePlanFailureV8 {
            obligation: PostStopRestoreObligationV8 {
                group,
                before,
                cause: original_cause,
            },
            cause,
        });
    }
    Ok(ProcessGroupSignalPlanV8 {
        signal: ProcessGroupSignalV8::Continue,
        group,
        planned_snapshot,
        restore_after_issued_stop: true,
    })
}

#[cfg(not(target_os = "linux"))]
fn plan_restore_post_stop_impl_v8(
    obligation: PostStopRestoreObligationV8,
) -> Result<ProcessGroupSignalPlanV8, RestoreContinuePlanFailureV8> {
    Err(RestoreContinuePlanFailureV8 {
        obligation,
        cause: unsupported("plan recovery CONT after issued STOP"),
    })
}

#[cfg(target_os = "linux")]
fn execute_process_group_signal_impl_v8(
    plan: ProcessGroupSignalPlanV8,
) -> Result<ProcessGroupSignalSuccessV8, ProcessGroupSignalFailureV8> {
    execute_process_group_signal_core_v8(plan, None)
}

#[cfg(target_os = "linux")]
fn execute_process_group_signal_core_v8(
    plan: ProcessGroupSignalPlanV8,
    #[cfg(test)] fault: Option<SignalPostIssueTestFaultV8>,
    #[cfg(not(test))] _fault: Option<()>,
) -> Result<ProcessGroupSignalSuccessV8, ProcessGroupSignalFailureV8> {
    let before = match revalidate_complete_group_impl_v8(&plan.group) {
        Ok(before) => before,
        Err(cause) => {
            return Err(ProcessGroupSignalFailureV8::BeforeSignal(
                PreSignalFailureV8 { plan, cause },
            ));
        }
    };
    if let Err(cause) = require_same_stable_group_v8(&plan.planned_snapshot, &before) {
        return Err(ProcessGroupSignalFailureV8::BeforeSignal(
            PreSignalFailureV8 { plan, cause },
        ));
    }
    let precondition = if plan.restore_after_issued_stop {
        require_recovery_continue_precondition_v8(&before)
    } else {
        require_signal_precondition_v8(plan.signal, &before)
    };
    if let Err(cause) = precondition {
        return Err(ProcessGroupSignalFailureV8::BeforeSignal(
            PreSignalFailureV8 { plan, cause },
        ));
    }

    let pgid = match i32::try_from(plan.group.process_group_id) {
        Ok(pgid) => pgid,
        Err(_) => {
            return Err(ProcessGroupSignalFailureV8::BeforeSignal(
                PreSignalFailureV8 {
                    plan,
                    cause: NativeSysErrorV8::InvalidInput(
                        "process group id does not fit pid_t".to_string(),
                    ),
                },
            ));
        }
    };
    let native_signal = match plan.signal {
        ProcessGroupSignalV8::Stop => libc::SIGSTOP,
        ProcessGroupSignalV8::Continue => libc::SIGCONT,
    };
    // SAFETY: the negative value names the already validated external process
    // group and `native_signal` is restricted to SIGSTOP/SIGCONT.
    let rc = unsafe { libc::kill(-pgid, native_signal) };
    if rc != 0 {
        let cause = super::io_error(
            "issue one exact process-group SIGSTOP/SIGCONT",
            std::io::Error::last_os_error(),
        );
        return Err(ProcessGroupSignalFailureV8::BeforeSignal(
            PreSignalFailureV8 { plan, cause },
        ));
    }

    #[cfg(test)]
    if let Some(fault) = fault {
        let cause = match fault {
            SignalPostIssueTestFaultV8::Timeout => NativeSysErrorV8::RaceDetected(
                "injected post-signal timeout after syscall".to_string(),
            ),
            SignalPostIssueTestFaultV8::IdentityDrift => NativeSysErrorV8::RaceDetected(
                "injected post-signal identity drift after syscall".to_string(),
            ),
        };
        return Err(post_signal_failure_v8(plan, before, cause));
    }

    let deadline = Instant::now() + SIGNAL_STATE_TIMEOUT_V8;
    loop {
        let fast_after = match revalidate_complete_group_fast_v8(&plan.group) {
            Ok(after) => after,
            Err(error) => {
                let cause = NativeSysErrorV8::RaceDetected(format!(
                    "post-signal fast identity revalidation failed: {error}"
                ));
                return Err(post_signal_failure_v8(plan, before, cause));
            }
        };
        if let Err(error) = require_same_stable_group_v8(&before, &fast_after) {
            let cause = NativeSysErrorV8::RaceDetected(format!(
                "post-signal fast stable membership changed: {error}"
            ));
            return Err(post_signal_failure_v8(plan, before, cause));
        }
        let fast_ready = match signal_postcondition_met_v8(plan.signal, &fast_after) {
            Ok(ready) => ready,
            Err(cause) => return Err(post_signal_failure_v8(plan, before, cause)),
        };
        if fast_ready {
            // Exactly one final full executable-digest revalidation closes
            // in-place tamper after the metadata-only poll loop.
            let after = match revalidate_complete_group_impl_v8(&plan.group) {
                Ok(after) => after,
                Err(error) => {
                    let cause = NativeSysErrorV8::RaceDetected(format!(
                        "final post-signal digest revalidation failed: {error}"
                    ));
                    return Err(post_signal_failure_v8(plan, before, cause));
                }
            };
            if let Err(error) = require_same_stable_group_v8(&before, &after) {
                let cause = NativeSysErrorV8::RaceDetected(format!(
                    "final post-signal digest identity changed: {error}"
                ));
                return Err(post_signal_failure_v8(plan, before, cause));
            }
            match signal_postcondition_met_v8(plan.signal, &after) {
                Ok(true) => {}
                Ok(false) => {
                    let cause = NativeSysErrorV8::RaceDetected(
                        "signal target state changed during final digest revalidation".to_string(),
                    );
                    return Err(post_signal_failure_v8(plan, before, cause));
                }
                Err(cause) => return Err(post_signal_failure_v8(plan, before, cause)),
            }
            let execution = ProcessGroupSignalExecutionV8 {
                signal: plan.signal,
                process_group_id: plan.group.process_group_id,
                before,
                after,
            };
            return Ok(match plan.signal {
                ProcessGroupSignalV8::Stop => {
                    ProcessGroupSignalSuccessV8::Stopped(StoppedProcessGroupV8 {
                        group: plan.group,
                        stop_execution: execution,
                    })
                }
                ProcessGroupSignalV8::Continue => ProcessGroupSignalSuccessV8::Continued(execution),
            });
        }
        if Instant::now() >= deadline {
            let cause = NativeSysErrorV8::RaceDetected(format!(
                "group signal was issued but members did not reach {:?} state before timeout",
                plan.signal
            ));
            return Err(post_signal_failure_v8(plan, before, cause));
        }
        std::thread::sleep(SIGNAL_STATE_POLL_V8);
    }
}

#[cfg(not(target_os = "linux"))]
fn execute_process_group_signal_impl_v8(
    plan: ProcessGroupSignalPlanV8,
) -> Result<ProcessGroupSignalSuccessV8, ProcessGroupSignalFailureV8> {
    Err(ProcessGroupSignalFailureV8::BeforeSignal(
        PreSignalFailureV8 {
            plan,
            cause: unsupported("execute exact process-group signal"),
        },
    ))
}

#[cfg(target_os = "linux")]
fn post_signal_failure_v8(
    plan: ProcessGroupSignalPlanV8,
    before: Vec<ProcessIdentityV8>,
    cause: NativeSysErrorV8,
) -> ProcessGroupSignalFailureV8 {
    match plan.signal {
        ProcessGroupSignalV8::Stop => {
            ProcessGroupSignalFailureV8::PostStop(PostStopRestoreObligationV8 {
                group: plan.group,
                before,
                cause,
            })
        }
        ProcessGroupSignalV8::Continue => {
            ProcessGroupSignalFailureV8::PostContinue(PostContinueRecoveryObligationV8 {
                group: plan.group,
                before,
                cause,
            })
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
#[derive(Clone, Copy, Debug)]
pub(super) enum SignalPostIssueTestFaultV8 {
    Timeout,
    IdentityDrift,
}

#[cfg(all(test, target_os = "linux"))]
pub(super) fn execute_process_group_signal_with_fault_for_test_v8(
    plan: ProcessGroupSignalPlanV8,
    fault: SignalPostIssueTestFaultV8,
) -> Result<ProcessGroupSignalSuccessV8, ProcessGroupSignalFailureV8> {
    execute_process_group_signal_core_v8(plan, Some(fault))
}

#[cfg(target_os = "linux")]
fn revalidate_complete_group_impl_v8(
    group: &ProcessGroupObservationV8,
) -> NativeSysResultV8<Vec<ProcessIdentityV8>> {
    let root = ProcfsRootV8::open_fixed()?;
    let expected: Vec<u32> = group
        .members
        .iter()
        .map(ProcessObservationV8::pid)
        .collect();
    let before_pids = root.process_group_pids(group.process_group_id)?;
    if before_pids != expected {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "process group {} membership is partial or changed before revalidation",
            group.process_group_id
        )));
    }
    let mut identities = Vec::with_capacity(group.members.len());
    for member in &group.members {
        let identity = member.revalidate()?;
        if identity.process_group_id() != group.process_group_id
            || identity.session_id() != group.session_id
        {
            return Err(NativeSysErrorV8::RaceDetected(format!(
                "pid {} left its exact process group or session",
                identity.pid()
            )));
        }
        identities.push(identity);
    }
    identities.sort_by_key(ProcessIdentityV8::pid);
    let after_pids = root.process_group_pids(group.process_group_id)?;
    if after_pids != expected {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "process group {} membership changed during revalidation",
            group.process_group_id
        )));
    }
    Ok(identities)
}

#[cfg(target_os = "linux")]
fn revalidate_complete_group_fast_v8(
    group: &ProcessGroupObservationV8,
) -> NativeSysResultV8<Vec<ProcessIdentityV8>> {
    let root = ProcfsRootV8::open_fixed()?;
    let expected: Vec<u32> = group
        .members
        .iter()
        .map(ProcessObservationV8::pid)
        .collect();
    let before_pids = root.process_group_pids(group.process_group_id)?;
    if before_pids != expected {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "process group {} membership is partial or changed before fast revalidation",
            group.process_group_id
        )));
    }
    let mut identities = Vec::with_capacity(group.members.len());
    for member in &group.members {
        let identity = member.revalidate_fast_with_root(&root)?;
        if identity.process_group_id() != group.process_group_id
            || identity.session_id() != group.session_id
        {
            return Err(NativeSysErrorV8::RaceDetected(format!(
                "pid {} left its exact process group or session during fast revalidation",
                identity.pid()
            )));
        }
        identities.push(identity);
    }
    identities.sort_by_key(ProcessIdentityV8::pid);
    let after_pids = root.process_group_pids(group.process_group_id)?;
    if after_pids != expected {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "process group {} membership changed during fast revalidation",
            group.process_group_id
        )));
    }
    Ok(identities)
}

/// Scope-only fast revalidation strengthens the reusable process-group
/// primitive with an exact global session closure. Ordinary process groups
/// may legitimately share a session with an external controller, so this
/// stronger invariant must not leak into the generic STOP/CONT API.
#[cfg(target_os = "linux")]
fn revalidate_scope_closure_fast_v8(
    group: &ProcessGroupObservationV8,
) -> NativeSysResultV8<Vec<ProcessIdentityV8>> {
    let root = ProcfsRootV8::open_fixed()?;
    let expected: Vec<u32> = group
        .members
        .iter()
        .map(ProcessObservationV8::pid)
        .collect();
    let (group_before, session_before) =
        root.process_group_and_session_pids(group.process_group_id, group.session_id)?;
    if group_before != expected || session_before != expected {
        return Err(NativeSysErrorV8::RaceDetected(
            "runner scope is not the complete global PGID and SID closure before fast revalidation"
                .to_string(),
        ));
    }
    let identities = revalidate_complete_group_fast_v8(group)?;
    let (group_after, session_after) =
        root.process_group_and_session_pids(group.process_group_id, group.session_id)?;
    if group_after != expected || session_after != expected {
        return Err(NativeSysErrorV8::RaceDetected(
            "runner global PGID/SID closure changed during fast revalidation".to_string(),
        ));
    }
    Ok(identities)
}

#[cfg(not(target_os = "linux"))]
fn revalidate_complete_group_impl_v8(
    _group: &ProcessGroupObservationV8,
) -> NativeSysResultV8<Vec<ProcessIdentityV8>> {
    Err(unsupported("revalidate exact shared process group"))
}

#[cfg(target_os = "linux")]
fn validate_expected_group_inputs_v8(
    expected_pids: &[u32],
    current_pid: u32,
) -> NativeSysResultV8<Vec<u32>> {
    if !(MIN_SHARED_PROCESS_GROUP_MEMBERS_V8..=MAX_SHARED_PROCESS_GROUP_MEMBERS_V8)
        .contains(&expected_pids.len())
    {
        return Err(NativeSysErrorV8::InvalidInput(format!(
            "shared process group must contain {MIN_SHARED_PROCESS_GROUP_MEMBERS_V8}..={MAX_SHARED_PROCESS_GROUP_MEMBERS_V8} exact members"
        )));
    }
    let mut unique = BTreeSet::new();
    for pid in expected_pids {
        if *pid <= 1 || *pid > libc::pid_t::MAX as u32 {
            return Err(NativeSysErrorV8::InvalidInput(
                "target pid must fit pid_t and be greater than one".to_string(),
            ));
        }
        if *pid == current_pid {
            return Err(NativeSysErrorV8::InvalidInput(
                "signal controller may not target its own process".to_string(),
            ));
        }
        if !unique.insert(*pid) {
            return Err(NativeSysErrorV8::InvalidInput(
                "target process list contains a duplicate pid".to_string(),
            ));
        }
    }
    Ok(unique.into_iter().collect())
}

#[cfg(target_os = "linux")]
fn require_signal_precondition_v8(
    signal: ProcessGroupSignalV8,
    identities: &[ProcessIdentityV8],
) -> NativeSysResultV8<()> {
    if identities.is_empty() {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "signal plan has no exact process identities".to_string(),
        ));
    }
    match signal {
        ProcessGroupSignalV8::Stop => {
            if identities
                .iter()
                .any(|identity| identity.state().is_stopped())
            {
                return Err(NativeSysErrorV8::InvalidInput(
                    "duplicate SIGSTOP is forbidden for an already-stopped member".to_string(),
                ));
            }
            if identities.iter().any(|identity| {
                matches!(
                    identity.state(),
                    super::ProcessStateV8::Zombie | super::ProcessStateV8::Dead
                )
            }) {
                return Err(NativeSysErrorV8::IdentityMismatch(
                    "SIGSTOP plan contains a dead or zombie member".to_string(),
                ));
            }
        }
        ProcessGroupSignalV8::Continue => {
            if identities
                .iter()
                .any(|identity| !identity.state().is_stopped())
            {
                return Err(NativeSysErrorV8::InvalidInput(
                    "SIGCONT requires every exact group member to be stopped".to_string(),
                ));
            }
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_recovery_continue_precondition_v8(
    identities: &[ProcessIdentityV8],
) -> NativeSysResultV8<()> {
    if identities.is_empty()
        || identities.iter().any(|identity| {
            matches!(
                identity.state(),
                super::ProcessStateV8::Zombie | super::ProcessStateV8::Dead
            )
        })
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "post-STOP recovery CONT requires a nonempty live exact group".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn signal_postcondition_met_v8(
    signal: ProcessGroupSignalV8,
    identities: &[ProcessIdentityV8],
) -> NativeSysResultV8<bool> {
    if identities.iter().any(|identity| {
        matches!(
            identity.state(),
            super::ProcessStateV8::Zombie | super::ProcessStateV8::Dead
        )
    }) {
        return Err(NativeSysErrorV8::RaceDetected(
            "group member became dead or zombie after signal".to_string(),
        ));
    }
    Ok(match signal {
        ProcessGroupSignalV8::Stop => identities
            .iter()
            .all(|identity| identity.state().is_stopped()),
        ProcessGroupSignalV8::Continue => identities
            .iter()
            .all(|identity| !identity.state().is_stopped()),
    })
}

#[cfg(target_os = "linux")]
fn require_same_stable_group_v8(
    expected: &[ProcessIdentityV8],
    observed: &[ProcessIdentityV8],
) -> NativeSysResultV8<()> {
    if expected.len() != observed.len()
        || expected
            .iter()
            .zip(observed)
            .any(|(left, right)| !left.has_same_stable_identity(right))
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "stable process-group identity differs from the exact snapshot".to_string(),
        ));
    }
    Ok(())
}

#[cfg(all(test, target_os = "linux"))]
pub(super) fn validate_expected_group_inputs_for_test_v8(
    expected_pids: &[u32],
    current_pid: u32,
) -> NativeSysResultV8<Vec<u32>> {
    validate_expected_group_inputs_v8(expected_pids, current_pid)
}
