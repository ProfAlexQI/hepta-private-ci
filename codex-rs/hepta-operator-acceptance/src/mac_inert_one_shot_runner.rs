//! Inert, fail-closed process boundary for macOS disposable effects.
//!
//! Threat model and boundary:
//! - V2 lifecycle records remain unchanged.  V3 records bind an exact command
//!   to a fresh process epoch, one runner epoch, and the current journal tip.
//! - Persistence is invoked before a command can cross the pipe.  Any error,
//!   panic, timeout, or channel loss leaves the command issued-or-uncertain.
//! - The child dispatch target is deliberately inert: it hashes and counts the
//!   admitted bytes.  This module imports no Disk Arbitration, mount, image,
//!   eject, privilege, or service-management API.
//! - A process epoch is a non-cloneable in-process capability bound to the boot
//!   UUID and the kernel's PID/start-time identity.  A fork-inherited value
//!   fails validation.
//! - Darwin has no atomic CLOEXEC pipe/kqueue creation.  A fresh executable
//!   must therefore prove it has exactly one kernel thread, preallocate a
//!   bounded one-shot FD pool, reserve the fixed child targets, and only then
//!   permit runner use.  No runtime runner spawn creates anonymous FDs.
//! - A runner accepts one exact command.  Same-supervisor sequential
//!   reconciliation requires a non-serializable death proof produced from the
//!   original live handle after kqueue NOTE_EXIT, pipe EOF, waitpid, and kernel
//!   identity checks all agree.  Fresh-supervisor recovery remains sealed
//!   until the S1 durable bridge can construct its distinct proof type.

use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::mac_disposable_lifecycle::DisposableAuthorityV2;
use rand::TryRngCore;
use rand::rngs::OsRng;
use serde::Deserialize;
use serde::Serialize;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::RawFd;
use std::process::Child;
#[cfg(test)]
use std::process::Command;
use std::process::ExitStatus;
#[cfg(test)]
use std::process::Stdio;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::time::Duration;
use thiserror::Error;

const ISSUE_SCHEMA_V3: &str = "hepta_mac_disposable_effect_issue_v3";
const ENVELOPE_SCHEMA_V3: &str = "hepta_mac_inert_runner_envelope_v3";
const PROCESS_EPOCH_SCHEMA_V3: &str = "hepta_mac_process_epoch_v3";
const RUNNER_HELLO_SCHEMA_V3: &str = "hepta_mac_inert_runner_hello_v3";
const DISPATCH_RECEIPT_SCHEMA_V3: &str = "hepta_mac_inert_dispatch_receipt_v3";
const DEATH_RECEIPT_SCHEMA_V3: &str = "hepta_mac_runner_death_receipt_v3";
const MAX_COMMAND_BYTES_V3: usize = 256 * 1024;
const MAX_FRAME_BYTES_V3: usize = 2 * 1024 * 1024;
const PROC_PIDTBSDINFO: libc::c_int = 3;
const PROC_PIDTASKINFO_V3: libc::c_int = 4;
const CHILD_COMMAND_FD_V3: RawFd = 900;
const CHILD_RESPONSE_FD_V3: RawFd = 901;
const CHILD_DEATH_FD_V3: RawFd = 902;
const CHILD_LEASE_FD_V3: RawFd = 903;
const CHILD_FIXED_FDS_V3: [RawFd; 4] = [
    CHILD_COMMAND_FD_V3,
    CHILD_RESPONSE_FD_V3,
    CHILD_DEATH_FD_V3,
    CHILD_LEASE_FD_V3,
];
const PREALLOCATED_RUNNER_SLOTS_V3: usize = 64;
const PREALLOCATED_SLOT_FDS_V3: usize = 7;
const PREALLOCATED_FDS_V3: usize = PREALLOCATED_RUNNER_SLOTS_V3 * PREALLOCATED_SLOT_FDS_V3;
const BOOTSTRAP_UNINITIALIZED_V3: i32 = 0;
const BOOTSTRAP_INITIALIZING_V3: i32 = 1;
const BOOTSTRAP_READY_V3: i32 = 2;
const BOOTSTRAP_FAILED_V3: i32 = -1;
const SKIP_PREMAIN_BOOTSTRAP_ENV_V3: &str = "HEPTA_SKIP_RUNNER_PREMAIN_BOOTSTRAP_V3";
const FRAME_HEADER_BYTES_V3: usize = 16;
const CLEANUP_TIMEOUT_V3: Duration = Duration::from_secs(5);
const CHILD_DEADLINE_ENV_V3: &str = "HEPTA_INERT_RUNNER_DEADLINE_NS_V3";
const F_SETNOSIGPIPE_V3: libc::c_int = 73;

static BOOTSTRAP_STATUS_V3: AtomicI32 = AtomicI32::new(BOOTSTRAP_UNINITIALIZED_V3);
static BOOTSTRAP_PID_V3: AtomicU32 = AtomicU32::new(0);
static BOOTSTRAP_NONCE_V3: [AtomicU8; 32] = [const { AtomicU8::new(0) }; 32];
static PREALLOCATED_SLOT_TAKEN_V3: [AtomicU8; PREALLOCATED_RUNNER_SLOTS_V3] =
    [const { AtomicU8::new(0) }; PREALLOCATED_RUNNER_SLOTS_V3];
static PREALLOCATED_FD_TABLE_V3: [AtomicI32; PREALLOCATED_FDS_V3] =
    [const { AtomicI32::new(-1) }; PREALLOCATED_FDS_V3];
static FIXED_TARGET_RESERVATIONS_V3: [AtomicI32; 4] = [const { AtomicI32::new(-1) }; 4];

#[derive(Debug, Error)]
pub enum InertRunnerErrorV3 {
    #[error("invalid inert one-shot runner state: {0}")]
    Invalid(String),
    #[error("effect issue persistence is uncertain: {0}")]
    PersistenceUncertain(String),
    #[error("inert one-shot runner timed out; issue remains uncertain")]
    TimeoutIssuedOrUncertain,
    #[error("inert one-shot runner did not complete its authenticated startup")]
    StartupFailed,
    #[error("inert one-shot runner channel was lost; issue remains uncertain: {0}")]
    ChannelLostIssuedOrUncertain(String),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectPurposeV3 {
    ForwardFlow,
    RestartReconciliation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EffectIssueContextV3 {
    FreshProcess,
    RestartReconciliation {
        prior_runner_death_proof_sha256: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssuedEffectRecordV3 {
    pub authority: DisposableAuthorityV2,
    pub command_sha256: String,
    pub effect_id: u64,
    pub issue_context: EffectIssueContextV3,
    pub journal_tip_before_sha256: Option<String>,
    pub operation_nonce: String,
    pub previous_record_sha256: Option<String>,
    pub process_epoch_sha256: String,
    pub purpose: EffectPurposeV3,
    pub runner_epoch_sha256: String,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerCommandEnvelopeV3 {
    pub command_sha256: String,
    pub effect_id: u64,
    pub issued_record_sha256: String,
    pub journal_tip_before_sha256: Option<String>,
    pub operation_nonce: String,
    pub previous_record_sha256: Option<String>,
    pub process_epoch_sha256: String,
    pub purpose: EffectPurposeV3,
    pub runner_epoch_sha256: String,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ProcessEpochBindingV3 {
    boot_session_uuid: String,
    bootstrap_pool_sha256: String,
    kernel_start_microseconds: u64,
    nonce: String,
    parent_pid: u32,
    pid: u32,
    schema: String,
    schema_version: u32,
}

/// Non-cloneable and process-local by construction.  Serializing the public
/// binding never recreates this capability.
pub struct FreshProcessEpochV3 {
    binding: ProcessEpochBindingV3,
    binding_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct RunnerHelloRequestV3 {
    challenge: String,
    process_epoch: ProcessEpochBindingV3,
    process_epoch_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct RunnerHelloV3 {
    challenge_sha256: String,
    parent_kernel_start_microseconds: u64,
    parent_pid: u32,
    process_epoch_sha256: String,
    runner_kernel_start_microseconds: u64,
    runner_nonce: String,
    runner_pid: u32,
    schema: String,
    schema_version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct RunnerWireCommandV3 {
    command: Vec<u8>,
    envelope: RunnerCommandEnvelopeV3,
    issued_record: IssuedEffectRecordV3,
}

/// Private typestate produced only after the persistence callback returns.
/// Its name remains issued-or-uncertain because durability does not prove
/// whether a later channel loss allowed the child to see the command.
struct DurablyIssuedOrUncertainV3 {
    record: IssuedEffectRecordV3,
    record_bytes: Vec<u8>,
    record_sha256: String,
}

/// Sealed receipt returned by the durable-store bridge.  There is no
/// production constructor in this lane; S2 must add the only constructor
/// after no-replace publish, fsync, descriptor retention, and final replay.
pub struct DurableIssuePersistenceReceiptV3 {
    issued_record_sha256: String,
    operation_nonce: String,
    runner_epoch_sha256: String,
}

impl DurableIssuePersistenceReceiptV3 {
    #[cfg(test)]
    fn for_test(record: &IssuedEffectRecordV3, bytes: &[u8]) -> Self {
        Self {
            issued_record_sha256: sha256(bytes),
            operation_nonce: record.operation_nonce.clone(),
            runner_epoch_sha256: record.runner_epoch_sha256.clone(),
        }
    }

    fn validate(
        &self,
        record: &IssuedEffectRecordV3,
        expected_sha256: &str,
    ) -> Result<(), InertRunnerErrorV3> {
        if self.issued_record_sha256 != expected_sha256
            || self.operation_nonce != record.operation_nonce
            || self.runner_epoch_sha256 != record.runner_epoch_sha256
        {
            return Err(invalid(
                "durable issue receipt differs from the exact issued record",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DurableIssuedBindingV3 {
    command_sha256: String,
    effect_id: u64,
    issued_record_sha256: String,
    journal_tip_before_sha256: Option<String>,
    operation_nonce: String,
    purpose: EffectPurposeV3,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InertDispatchReceiptV3 {
    pub authority: DisposableAuthorityV2,
    pub command_sha256: String,
    pub dispatch_count: u32,
    pub issued_record_sha256: String,
    pub runner_epoch_sha256: String,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PriorRunnerDeathReceiptV3 {
    pub authority: DisposableAuthorityV2,
    pub command_sha256: String,
    pub death_pipe_eof_observed: bool,
    pub effect_id: u64,
    pub exit_status: i32,
    pub issued_record_sha256: String,
    pub journal_tip_before_sha256: Option<String>,
    pub kernel_identity_absent: bool,
    pub kqueue_note_exit_observed: bool,
    pub operation_nonce: String,
    pub process_epoch_sha256: String,
    pub purpose: EffectPurposeV3,
    pub runner_epoch_sha256: String,
    pub runner_kernel_start_microseconds: u64,
    pub runner_pid: u32,
    pub schema: String,
    pub schema_version: u32,
    pub waitpid_observed: bool,
}

/// Only the original live handle can construct this same-supervisor proof.
/// A copied JSON receipt is evidence, not a restart capability.
#[derive(Debug)]
pub struct SameSupervisorRunnerDeathProofV3 {
    receipt: PriorRunnerDeathReceiptV3,
    receipt_sha256: String,
}

/// Fresh-supervisor recovery is intentionally sealed until S1 supplies a
/// descriptor-retained durable bridge.  This type has no constructor here.
pub struct RecoveredRunnerDeathProofV3 {
    _receipt: PriorRunnerDeathReceiptV3,
    _s1_durable_bridge_sha256: String,
    _seal: RecoveredProofSealV3,
}

struct RecoveredProofSealV3;

/// Owned, non-serializable FD reserved for a future borrow-tied, one-shot S2
/// persisted-issued grant.  That grant must perform the exact S1 revalidation
/// while exclusively borrowing the durable store and must bind operation,
/// effect, command, tip, issued-record, and runner-epoch digests.  There is no
/// production constructor here, no direct S1 outlet, and no `From<File>` path.
pub struct RetainedControlLeaseV3 {
    descriptor: File,
}

impl RetainedControlLeaseV3 {
    #[cfg(test)]
    fn for_test(descriptor: File) -> Self {
        Self { descriptor }
    }

    fn prepare_for_child(&self) -> Result<File, InertRunnerErrorV3> {
        let source_flags = unsafe { libc::fcntl(self.descriptor.as_raw_fd(), libc::F_GETFD) };
        if source_flags < 0 {
            return Err(io::Error::last_os_error().into());
        }
        if source_flags & libc::FD_CLOEXEC == 0 {
            return Err(invalid(
                "retained control lease is not CLOEXEC in the parent",
            ));
        }
        // F_DUPFD_CLOEXEC avoids even a transient inheritable duplicate in a
        // multithreaded supervisor. Only the post-fork child-local dup2 target
        // is ever made inheritable.
        let duplicate =
            unsafe { libc::fcntl(self.descriptor.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 0) };
        if duplicate < 0 {
            return Err(io::Error::last_os_error().into());
        }
        Ok(unsafe { File::from_raw_fd(duplicate) })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunnerStateV3 {
    Ready,
    IssuedOrUncertain,
    Reaped,
}

/// Supervisor-owned live handle.  It is intentionally not Clone.
pub struct LiveInertRunnerV3 {
    child: Child,
    command_pipe: File,
    death_pipe: File,
    kqueue: File,
    durable_issued_binding: Option<DurableIssuedBindingV3>,
    process_epoch_sha256: String,
    retained_death_proof: Option<SameSupervisorRunnerDeathProofV3>,
    response_pipe: File,
    runner_epoch_sha256: String,
    runner_identity: KernelProcessIdentityV3,
    session_deadline: AbsoluteDeadlineV3,
    state: RunnerStateV3,
}

struct PreallocatedRunnerSlotV3 {
    child_command_read: File,
    parent_command_write: File,
    parent_response_read: File,
    child_response_write: File,
    parent_death_read: File,
    child_death_write: File,
    kqueue: File,
}

#[derive(Clone, Copy, Debug)]
struct AbsoluteDeadlineV3 {
    monotonic_nanoseconds: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct KernelProcessIdentityV3 {
    parent_pid: u32,
    pid: u32,
    start_microseconds: u64,
}

#[repr(C)]
struct ProcBsdInfoV3 {
    flags: u32,
    status: u32,
    xstatus: u32,
    pid: u32,
    ppid: u32,
    uid: u32,
    gid: u32,
    ruid: u32,
    rgid: u32,
    svuid: u32,
    svgid: u32,
    reserved: u32,
    comm: [libc::c_char; 16],
    name: [libc::c_char; 32],
    nfiles: u32,
    pgid: u32,
    pjobc: u32,
    tdev: u32,
    tpgid: u32,
    nice: i32,
    start_seconds: u64,
    start_microseconds: u64,
}

unsafe extern "C" {
    fn proc_pidinfo(
        pid: libc::c_int,
        flavor: libc::c_int,
        arg: u64,
        buffer: *mut libc::c_void,
        buffersize: libc::c_int,
    ) -> libc::c_int;
}

#[cfg(test)]
#[used]
#[unsafe(link_section = "__DATA,__mod_init_func")]
static TEST_PREMAIN_RUNNER_POOL_BOOTSTRAP_V3: extern "C" fn() = {
    extern "C" fn initialize() {
        let skip_name = b"HEPTA_SKIP_RUNNER_PREMAIN_BOOTSTRAP_V3\0";
        if unsafe { libc::getenv(skip_name.as_ptr().cast()) }.is_null() {
            let _ = ensure_preallocated_runner_pool();
        }
    }
    initialize
};

fn ensure_preallocated_runner_pool() -> Result<(), InertRunnerErrorV3> {
    loop {
        match BOOTSTRAP_STATUS_V3.load(Ordering::Acquire) {
            BOOTSTRAP_READY_V3 => return validate_preallocated_runner_pool(),
            BOOTSTRAP_FAILED_V3 => {
                return Err(invalid(
                    "single-thread runner FD pool bootstrap previously failed",
                ));
            }
            BOOTSTRAP_INITIALIZING_V3 => {
                return Err(invalid(
                    "runner FD pool bootstrap is unexpectedly concurrent",
                ));
            }
            BOOTSTRAP_UNINITIALIZED_V3 => {
                require_single_kernel_thread("before runner FD pool bootstrap")?;
                if BOOTSTRAP_STATUS_V3
                    .compare_exchange(
                        BOOTSTRAP_UNINITIALIZED_V3,
                        BOOTSTRAP_INITIALIZING_V3,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_err()
                {
                    continue;
                }
                match build_preallocated_runner_pool() {
                    Ok(()) => {
                        BOOTSTRAP_STATUS_V3.store(BOOTSTRAP_READY_V3, Ordering::Release);
                        return validate_preallocated_runner_pool();
                    }
                    Err(error) => {
                        BOOTSTRAP_STATUS_V3.store(BOOTSTRAP_FAILED_V3, Ordering::Release);
                        return Err(error);
                    }
                }
            }
            _ => return Err(invalid("runner FD pool bootstrap state is invalid")),
        }
    }
}

fn build_preallocated_runner_pool() -> Result<(), InertRunnerErrorV3> {
    // This is the only function allowed to call pipe(2) or kqueue(2).  The
    // kernel thread-count checks plus a full signal mask prove no concurrent
    // exec actor exists while CLOEXEC is applied. Runtime runner launches only
    // consume these one-shot slots.
    let mut descriptors = [-1; PREALLOCATED_FDS_V3];
    let mut reservations = [-1; 4];
    let previous_signals = block_all_signals()?;
    let result = (|| -> Result<[u8; 32], InertRunnerErrorV3> {
        require_single_kernel_thread("during runner FD pool bootstrap")?;
        let null_path = b"/dev/null\0";
        let null_fd =
            unsafe { libc::open(null_path.as_ptr().cast(), libc::O_RDONLY | libc::O_CLOEXEC) };
        if null_fd < 0 {
            return Err(io::Error::last_os_error().into());
        }
        if null_fd >= CHILD_COMMAND_FD_V3 {
            unsafe {
                libc::close(null_fd);
            }
            return Err(invalid(
                "fixed child FD range was reached before bootstrap reservations",
            ));
        }
        for (index, target) in CHILD_FIXED_FDS_V3.into_iter().enumerate() {
            let duplicate = unsafe { libc::fcntl(null_fd, libc::F_DUPFD_CLOEXEC, target) };
            if duplicate != target {
                if duplicate >= 0 {
                    unsafe {
                        libc::close(duplicate);
                    }
                }
                unsafe {
                    libc::close(null_fd);
                }
                return Err(invalid(
                    "fixed child FD reservation is occupied or outside the descriptor limit",
                ));
            }
            reservations[index] = duplicate;
        }
        unsafe {
            libc::close(null_fd);
        }

        for slot in 0..PREALLOCATED_RUNNER_SLOTS_V3 {
            let base = slot * PREALLOCATED_SLOT_FDS_V3;
            for pair in 0..3 {
                let offset = base + pair * 2;
                let mut pipe = [-1; 2];
                if unsafe { libc::pipe(pipe.as_mut_ptr()) } != 0 {
                    return Err(io::Error::last_os_error().into());
                }
                descriptors[offset] = pipe[0];
                descriptors[offset + 1] = pipe[1];
                set_close_on_exec(pipe[0])?;
                set_close_on_exec(pipe[1])?;
            }
            let kqueue = unsafe { libc::kqueue() };
            if kqueue < 0 {
                return Err(io::Error::last_os_error().into());
            }
            descriptors[base + 6] = kqueue;
            set_close_on_exec(kqueue)?;
        }

        require_single_kernel_thread("after runner FD pool bootstrap")?;
        let mut nonce = [0u8; 32];
        if unsafe { libc::getentropy(nonce.as_mut_ptr().cast(), nonce.len()) } != 0 {
            return Err(io::Error::last_os_error().into());
        }
        Ok(nonce)
    })();
    let nonce = match result {
        Ok(nonce) => {
            if let Err(error) = restore_signal_mask(&previous_signals) {
                close_raw_descriptors(descriptors.into_iter().chain(reservations));
                return Err(error);
            }
            nonce
        }
        Err(error) => {
            close_raw_descriptors(descriptors.into_iter().chain(reservations));
            restore_signal_mask(&previous_signals)?;
            return Err(error);
        }
    };
    for (destination, byte) in BOOTSTRAP_NONCE_V3.iter().zip(nonce) {
        destination.store(byte, Ordering::Relaxed);
    }
    BOOTSTRAP_PID_V3.store(unsafe { libc::getpid() } as u32, Ordering::Relaxed);
    for (destination, descriptor) in PREALLOCATED_FD_TABLE_V3.iter().zip(descriptors) {
        destination.store(descriptor, Ordering::Relaxed);
    }
    for (destination, descriptor) in FIXED_TARGET_RESERVATIONS_V3.iter().zip(reservations) {
        destination.store(descriptor, Ordering::Relaxed);
    }
    Ok(())
}

fn block_all_signals() -> Result<libc::sigset_t, InertRunnerErrorV3> {
    let mut all = MaybeUninit::<libc::sigset_t>::zeroed();
    if unsafe { libc::sigfillset(all.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    let all = unsafe { all.assume_init() };
    let mut previous = MaybeUninit::<libc::sigset_t>::zeroed();
    let rc = unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, &all, previous.as_mut_ptr()) };
    if rc != 0 {
        return Err(io::Error::from_raw_os_error(rc).into());
    }
    Ok(unsafe { previous.assume_init() })
}

fn restore_signal_mask(previous: &libc::sigset_t) -> Result<(), InertRunnerErrorV3> {
    let rc = unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, previous, std::ptr::null_mut()) };
    if rc != 0 {
        return Err(io::Error::from_raw_os_error(rc).into());
    }
    Ok(())
}

fn close_raw_descriptors(descriptors: impl IntoIterator<Item = RawFd>) {
    for descriptor in descriptors {
        if descriptor >= 0 {
            unsafe {
                libc::close(descriptor);
            }
        }
    }
}

fn validate_preallocated_runner_pool() -> Result<(), InertRunnerErrorV3> {
    if BOOTSTRAP_PID_V3.load(Ordering::Acquire) != unsafe { libc::getpid() } as u32 {
        return Err(invalid(
            "runner FD pool belongs to another process or a fork descendant",
        ));
    }
    for (reservation, expected) in FIXED_TARGET_RESERVATIONS_V3.iter().zip(CHILD_FIXED_FDS_V3) {
        if reservation.load(Ordering::Acquire) != expected {
            return Err(invalid("fixed child FD reservation identity changed"));
        }
        let flags = unsafe { libc::fcntl(expected, libc::F_GETFD) };
        if flags < 0 || flags & libc::FD_CLOEXEC == 0 {
            return Err(invalid("fixed child FD reservation was closed or changed"));
        }
    }
    Ok(())
}

fn require_single_kernel_thread(label: &str) -> Result<(), InertRunnerErrorV3> {
    let mut task = MaybeUninit::<libc::proc_taskinfo>::zeroed();
    let expected = std::mem::size_of::<libc::proc_taskinfo>();
    let received = unsafe {
        proc_pidinfo(
            libc::getpid(),
            PROC_PIDTASKINFO_V3,
            0,
            task.as_mut_ptr().cast(),
            expected as libc::c_int,
        )
    };
    if received != expected as libc::c_int {
        return Err(io::Error::last_os_error().into());
    }
    let task = unsafe { task.assume_init() };
    if task.pti_threadnum != 1 {
        return Err(invalid(format!(
            "{label} requires exactly one kernel thread; observed {}",
            task.pti_threadnum,
        )));
    }
    Ok(())
}

fn bootstrap_pool_sha256() -> Result<String, InertRunnerErrorV3> {
    validate_preallocated_runner_pool()?;
    let mut nonce = [0u8; 32];
    for (destination, source) in nonce.iter_mut().zip(&BOOTSTRAP_NONCE_V3) {
        *destination = source.load(Ordering::Acquire);
    }
    Ok(sha256(&nonce))
}

fn take_preallocated_runner_slot() -> Result<PreallocatedRunnerSlotV3, InertRunnerErrorV3> {
    validate_preallocated_runner_pool()?;
    for slot in 0..PREALLOCATED_RUNNER_SLOTS_V3 {
        if PREALLOCATED_SLOT_TAKEN_V3[slot]
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            continue;
        }
        let base = slot * PREALLOCATED_SLOT_FDS_V3;
        let mut descriptors = [-1; PREALLOCATED_SLOT_FDS_V3];
        for (offset, destination) in descriptors.iter_mut().enumerate() {
            *destination = PREALLOCATED_FD_TABLE_V3[base + offset].swap(-1, Ordering::AcqRel);
        }
        if descriptors.iter().any(|descriptor| *descriptor < 0) {
            for descriptor in descriptors {
                if descriptor >= 0 {
                    unsafe {
                        libc::close(descriptor);
                    }
                }
            }
            return Err(invalid("preallocated runner FD slot is incomplete"));
        }
        let [
            command_read,
            command_write,
            response_read,
            response_write,
            death_read,
            death_write,
            kqueue,
        ] = descriptors;
        return Ok(PreallocatedRunnerSlotV3 {
            child_command_read: unsafe { File::from_raw_fd(command_read) },
            parent_command_write: unsafe { File::from_raw_fd(command_write) },
            parent_response_read: unsafe { File::from_raw_fd(response_read) },
            child_response_write: unsafe { File::from_raw_fd(response_write) },
            parent_death_read: unsafe { File::from_raw_fd(death_read) },
            child_death_write: unsafe { File::from_raw_fd(death_write) },
            kqueue: unsafe { File::from_raw_fd(kqueue) },
        });
    }
    Err(invalid("bounded preallocated runner FD pool is exhausted"))
}

impl FreshProcessEpochV3 {
    pub fn establish() -> Result<Self, InertRunnerErrorV3> {
        ensure_preallocated_runner_pool()?;
        let identity = kernel_process_identity(unsafe { libc::getpid() } as u32)?;
        let binding = ProcessEpochBindingV3 {
            boot_session_uuid: boot_session_uuid()?,
            bootstrap_pool_sha256: bootstrap_pool_sha256()?,
            kernel_start_microseconds: identity.start_microseconds,
            nonce: random_hex(32)?,
            parent_pid: identity.parent_pid,
            pid: identity.pid,
            schema: PROCESS_EPOCH_SCHEMA_V3.to_string(),
            schema_version: 3,
        };
        let binding_sha256 = digest_canonical(&binding)?;
        let epoch = Self {
            binding,
            binding_sha256,
        };
        epoch.validate_current()?;
        Ok(epoch)
    }

    pub fn sha256(&self) -> &str {
        &self.binding_sha256
    }

    pub fn validate_current(&self) -> Result<(), InertRunnerErrorV3> {
        let identity = kernel_process_identity(unsafe { libc::getpid() } as u32)?;
        if identity.pid != self.binding.pid
            || identity.parent_pid != self.binding.parent_pid
            || identity.start_microseconds != self.binding.kernel_start_microseconds
            || boot_session_uuid()? != self.binding.boot_session_uuid
            || bootstrap_pool_sha256()? != self.binding.bootstrap_pool_sha256
            || digest_canonical(&self.binding)? != self.binding_sha256
        {
            return Err(invalid(
                "fresh process epoch was inherited, replaced, or changed",
            ));
        }
        Ok(())
    }
}

impl AbsoluteDeadlineV3 {
    fn after(duration: Duration) -> Result<Self, InertRunnerErrorV3> {
        let delta = u64::try_from(duration.as_nanos())
            .map_err(|_| invalid("runner deadline duration overflows nanoseconds"))?;
        let monotonic_nanoseconds = monotonic_nanoseconds()?
            .checked_add(delta)
            .ok_or_else(|| invalid("runner absolute deadline overflowed"))?;
        Ok(Self {
            monotonic_nanoseconds,
        })
    }

    fn from_environment() -> Result<Self, InertRunnerErrorV3> {
        let value = std::env::var(CHILD_DEADLINE_ENV_V3)
            .map_err(|_| invalid("missing child session deadline"))?;
        let monotonic_nanoseconds = value
            .parse::<u64>()
            .map_err(|_| invalid("invalid child session deadline"))?;
        let deadline = Self {
            monotonic_nanoseconds,
        };
        deadline.remaining_nanoseconds()?;
        Ok(deadline)
    }

    fn min(self, other: Self) -> Self {
        Self {
            monotonic_nanoseconds: self.monotonic_nanoseconds.min(other.monotonic_nanoseconds),
        }
    }

    fn remaining_nanoseconds(self) -> Result<u64, InertRunnerErrorV3> {
        self.monotonic_nanoseconds
            .checked_sub(monotonic_nanoseconds()?)
            .filter(|remaining| *remaining > 0)
            .ok_or_else(frame_deadline_expired)
    }

    fn poll_timeout_milliseconds(self) -> Result<libc::c_int, InertRunnerErrorV3> {
        let remaining = self.remaining_nanoseconds()?;
        let rounded_up = remaining.saturating_add(999_999) / 1_000_000;
        Ok(rounded_up.min(libc::c_int::MAX as u64) as libc::c_int)
    }
}

impl IssuedEffectRecordV3 {
    fn validate(&self) -> Result<(), InertRunnerErrorV3> {
        require_nonce(&self.operation_nonce, "operation nonce")?;
        require_sha256(&self.command_sha256, "command digest")?;
        require_sha256(&self.process_epoch_sha256, "process epoch digest")?;
        require_sha256(&self.runner_epoch_sha256, "runner epoch digest")?;
        require_optional_sha256(&self.previous_record_sha256, "predecessor digest")?;
        require_optional_sha256(&self.journal_tip_before_sha256, "journal tip digest")?;
        if self.previous_record_sha256 != self.journal_tip_before_sha256 {
            return Err(invalid(
                "record predecessor differs from the journal tip bound at issue",
            ));
        }
        if self.schema != ISSUE_SCHEMA_V3 || self.schema_version != 3 || self.authority.any() {
            return Err(invalid("V3 issue schema or no-authority boundary changed"));
        }
        match (&self.purpose, &self.issue_context) {
            (EffectPurposeV3::ForwardFlow, EffectIssueContextV3::FreshProcess) => {}
            (
                EffectPurposeV3::RestartReconciliation,
                EffectIssueContextV3::RestartReconciliation {
                    prior_runner_death_proof_sha256,
                },
            ) => require_sha256(
                prior_runner_death_proof_sha256,
                "prior runner death proof digest",
            )?,
            _ => return Err(invalid("effect purpose and issue context disagree")),
        }
        Ok(())
    }
}

impl RunnerCommandEnvelopeV3 {
    fn validate_against(
        &self,
        record: &IssuedEffectRecordV3,
        command: &[u8],
    ) -> Result<(), InertRunnerErrorV3> {
        record.validate()?;
        require_command_size(command)?;
        if self.schema != ENVELOPE_SCHEMA_V3 || self.schema_version != 3 {
            return Err(invalid("runner command envelope schema changed"));
        }
        let issued_record_sha256 = digest_canonical(record)?;
        if self.issued_record_sha256 != issued_record_sha256
            || self.command_sha256 != sha256(command)
            || self.command_sha256 != record.command_sha256
            || self.effect_id != record.effect_id
            || self.journal_tip_before_sha256 != record.journal_tip_before_sha256
            || self.operation_nonce != record.operation_nonce
            || self.previous_record_sha256 != record.previous_record_sha256
            || self.process_epoch_sha256 != record.process_epoch_sha256
            || self.purpose != record.purpose
            || self.runner_epoch_sha256 != record.runner_epoch_sha256
        {
            return Err(invalid(
                "runner envelope, issue record, journal tip, or command bytes drifted",
            ));
        }
        Ok(())
    }
}

impl SameSupervisorRunnerDeathProofV3 {
    pub fn receipt(&self) -> &PriorRunnerDeathReceiptV3 {
        &self.receipt
    }

    pub fn sha256(&self) -> &str {
        &self.receipt_sha256
    }

    fn validate(&self) -> Result<(), InertRunnerErrorV3> {
        self.receipt.validate()?;
        if digest_canonical(&self.receipt)? != self.receipt_sha256 {
            return Err(invalid("same-supervisor death proof digest changed"));
        }
        Ok(())
    }
}

impl PriorRunnerDeathReceiptV3 {
    fn validate(&self) -> Result<(), InertRunnerErrorV3> {
        require_sha256(&self.command_sha256, "death receipt command digest")?;
        require_sha256(
            &self.issued_record_sha256,
            "death receipt issued-record digest",
        )?;
        require_optional_sha256(&self.journal_tip_before_sha256, "death receipt journal tip")?;
        require_nonce(&self.operation_nonce, "death receipt operation nonce")?;
        require_sha256(&self.process_epoch_sha256, "death receipt process epoch")?;
        require_sha256(&self.runner_epoch_sha256, "death receipt runner epoch")?;
        if self.schema != DEATH_RECEIPT_SCHEMA_V3
            || self.schema_version != 3
            || self.authority.any()
            || !self.death_pipe_eof_observed
            || !self.kernel_identity_absent
            || !self.kqueue_note_exit_observed
            || !self.waitpid_observed
            || self.runner_pid == 0
            || self.runner_kernel_start_microseconds == 0
        {
            return Err(invalid(
                "death receipt schema, no-authority boundary, or composite signals changed",
            ));
        }
        Ok(())
    }
}

impl LiveInertRunnerV3 {
    /// Test-only launcher for an independent inert process-group member.
    /// Production integration must instead consume the future borrow-tied,
    /// exact S2 persisted-issued grant described on `RetainedControlLeaseV3`.
    #[cfg(test)]
    fn spawn_program(
        epoch: &FreshProcessEpochV3,
        lease: &RetainedControlLeaseV3,
        program: &std::path::Path,
        arguments: &[&str],
        startup_timeout: Duration,
    ) -> Result<Self, InertRunnerErrorV3> {
        epoch.validate_current()?;
        let startup_deadline = AbsoluteDeadlineV3::after(startup_timeout)?;
        let session_deadline =
            AbsoluteDeadlineV3::after(startup_timeout.max(Duration::from_secs(30)))?;
        let PreallocatedRunnerSlotV3 {
            child_command_read,
            parent_command_write,
            parent_response_read,
            child_response_write,
            parent_death_read,
            child_death_write,
            kqueue,
        } = take_preallocated_runner_slot()?;
        let child_lease = lease.prepare_for_child()?;
        let source_fds = [
            child_command_read.as_raw_fd(),
            child_response_write.as_raw_fd(),
            child_death_write.as_raw_fd(),
            child_lease.as_raw_fd(),
        ];
        let target_fds = CHILD_FIXED_FDS_V3;
        reject_child_fd_aliases(&source_fds, &target_fds)?;
        set_nonblocking(parent_command_write.as_raw_fd())?;
        set_no_sigpipe(parent_command_write.as_raw_fd())?;
        set_nonblocking(parent_response_read.as_raw_fd())?;
        set_nonblocking(parent_death_read.as_raw_fd())?;

        let mut command = Command::new(program);
        command
            .args(arguments)
            .env(
                "HEPTA_INERT_RUNNER_COMMAND_FD_V3",
                CHILD_COMMAND_FD_V3.to_string(),
            )
            .env(
                "HEPTA_INERT_RUNNER_RESPONSE_FD_V3",
                CHILD_RESPONSE_FD_V3.to_string(),
            )
            .env(
                "HEPTA_INERT_RUNNER_DEATH_FD_V3",
                CHILD_DEATH_FD_V3.to_string(),
            )
            .env(
                "HEPTA_INERT_RUNNER_LEASE_FD_V3",
                CHILD_LEASE_FD_V3.to_string(),
            )
            .env(
                CHILD_DEADLINE_ENV_V3,
                session_deadline.monotonic_nanoseconds.to_string(),
            )
            .env(SKIP_PREMAIN_BOOTSTRAP_ENV_V3, "1")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        use std::os::unix::process::CommandExt;
        unsafe {
            command.pre_exec(move || {
                // This closure runs after fork and before exec.  Keep it to
                // async-signal-safe syscalls only.  Parent FD flags remain
                // CLOEXEC throughout; only these child-local dup2 targets are
                // inheritable across exec.
                for (source, target) in source_fds.into_iter().zip(target_fds) {
                    if libc::dup2(source, target) < 0 {
                        return Err(io::Error::last_os_error());
                    }
                }
                for source in source_fds {
                    libc::close(source);
                }
                if libc::setpgid(0, 0) != 0 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let mut child = command.spawn()?;
        drop(child_command_read);
        drop(child_response_write);
        drop(child_death_write);
        drop(child_lease);

        let startup = (|| {
            let challenge = random_hex(32)?;
            let request = RunnerHelloRequestV3 {
                challenge: challenge.clone(),
                process_epoch: epoch.binding.clone(),
                process_epoch_sha256: epoch.binding_sha256.clone(),
            };
            write_canonical_frame_until(&parent_command_write, &request, startup_deadline)?;
            let (hello_bytes, hello_deadline) =
                read_frame_until(&parent_response_read, startup_deadline)
                    .map_err(|_| InertRunnerErrorV3::StartupFailed)?;
            if hello_deadline.monotonic_nanoseconds != startup_deadline.monotonic_nanoseconds {
                return Err(invalid("runner hello changed the startup deadline"));
            }
            let hello: RunnerHelloV3 = parse_canonical(&hello_bytes, "runner hello")?;
            let identity = kernel_process_identity(child.id())?;
            if hello.schema != RUNNER_HELLO_SCHEMA_V3
                || hello.schema_version != 3
                || hello.challenge_sha256 != sha256(challenge.as_bytes())
                || hello.parent_pid != epoch.binding.pid
                || hello.parent_kernel_start_microseconds != epoch.binding.kernel_start_microseconds
                || hello.process_epoch_sha256 != epoch.binding_sha256
                || hello.runner_pid != child.id()
                || hello.runner_pid != identity.pid
                || hello.runner_kernel_start_microseconds != identity.start_microseconds
                || unsafe { libc::getpgid(child.id() as libc::pid_t) } != child.id() as libc::pid_t
            {
                return Err(invalid("runner hello or independent process group changed"));
            }
            require_nonce(&hello.runner_nonce, "runner nonce")?;
            let runner_epoch_sha256 = digest_canonical(&hello)?;
            register_process_exit(kqueue.as_raw_fd(), child.id())?;
            Ok((identity, runner_epoch_sha256))
        })();
        let (identity, runner_epoch_sha256) = match startup {
            Ok(startup) => startup,
            Err(error) => {
                terminate_group_and_reap(&mut child);
                return Err(error);
            }
        };
        Ok(Self {
            child,
            command_pipe: parent_command_write,
            death_pipe: parent_death_read,
            durable_issued_binding: None,
            kqueue,
            process_epoch_sha256: epoch.binding_sha256.clone(),
            retained_death_proof: None,
            response_pipe: parent_response_read,
            runner_epoch_sha256,
            runner_identity: identity,
            session_deadline,
            state: RunnerStateV3::Ready,
        })
    }

    pub fn runner_epoch_sha256(&self) -> &str {
        &self.runner_epoch_sha256
    }

    #[allow(clippy::too_many_arguments)]
    pub fn issue_fresh_with<F>(
        &mut self,
        epoch: &FreshProcessEpochV3,
        operation_nonce: &str,
        effect_id: u64,
        previous_record_sha256: Option<String>,
        command: &[u8],
        timeout: Duration,
        persist: F,
    ) -> Result<InertDispatchReceiptV3, InertRunnerErrorV3>
    where
        F: FnOnce(&IssuedEffectRecordV3, &[u8]) -> io::Result<DurableIssuePersistenceReceiptV3>,
    {
        self.issue_with(
            epoch,
            operation_nonce,
            effect_id,
            previous_record_sha256,
            command,
            timeout,
            EffectPurposeV3::ForwardFlow,
            EffectIssueContextV3::FreshProcess,
            persist,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn issue_same_supervisor_reconciliation_with<F>(
        &mut self,
        epoch: &FreshProcessEpochV3,
        death_proof: SameSupervisorRunnerDeathProofV3,
        operation_nonce: &str,
        effect_id: u64,
        previous_record_sha256: Option<String>,
        command: &[u8],
        timeout: Duration,
        persist: F,
    ) -> Result<InertDispatchReceiptV3, InertRunnerErrorV3>
    where
        F: FnOnce(&IssuedEffectRecordV3, &[u8]) -> io::Result<DurableIssuePersistenceReceiptV3>,
    {
        death_proof.validate()?;
        if death_proof.receipt.process_epoch_sha256 != epoch.binding_sha256 {
            return Err(invalid(
                "prior runner death proof belongs to another supervisor epoch",
            ));
        }
        if death_proof.receipt.operation_nonce != operation_nonce
            || previous_record_sha256.as_deref()
                != Some(death_proof.receipt.issued_record_sha256.as_str())
            || effect_id <= death_proof.receipt.effect_id
        {
            return Err(invalid(
                "same-supervisor death proof was transplanted across operation, journal tip, or effect order",
            ));
        }
        self.issue_with(
            epoch,
            operation_nonce,
            effect_id,
            previous_record_sha256,
            command,
            timeout,
            EffectPurposeV3::RestartReconciliation,
            EffectIssueContextV3::RestartReconciliation {
                prior_runner_death_proof_sha256: death_proof.receipt_sha256.clone(),
            },
            persist,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn issue_with<F>(
        &mut self,
        epoch: &FreshProcessEpochV3,
        operation_nonce: &str,
        effect_id: u64,
        previous_record_sha256: Option<String>,
        command: &[u8],
        timeout: Duration,
        purpose: EffectPurposeV3,
        issue_context: EffectIssueContextV3,
        persist: F,
    ) -> Result<InertDispatchReceiptV3, InertRunnerErrorV3>
    where
        F: FnOnce(&IssuedEffectRecordV3, &[u8]) -> io::Result<DurableIssuePersistenceReceiptV3>,
    {
        epoch.validate_current()?;
        if self.state != RunnerStateV3::Ready {
            return Err(invalid(
                "runner epoch already accepted or may have accepted one command",
            ));
        }
        if self.process_epoch_sha256 != epoch.binding_sha256 {
            return Err(invalid("runner belongs to another process epoch"));
        }
        require_command_size(command)?;
        let deadline = AbsoluteDeadlineV3::after(timeout)?.min(self.session_deadline);
        deadline.remaining_nanoseconds()?;
        let record = IssuedEffectRecordV3 {
            authority: DisposableAuthorityV2::none(),
            command_sha256: sha256(command),
            effect_id,
            issue_context,
            journal_tip_before_sha256: previous_record_sha256.clone(),
            operation_nonce: operation_nonce.to_string(),
            previous_record_sha256,
            process_epoch_sha256: epoch.binding_sha256.clone(),
            purpose,
            runner_epoch_sha256: self.runner_epoch_sha256.clone(),
            schema: ISSUE_SCHEMA_V3.to_string(),
            schema_version: 3,
        };
        record.validate()?;
        let issued = self.persist_issue(record, persist)?;
        let envelope = RunnerCommandEnvelopeV3 {
            command_sha256: issued.record.command_sha256.clone(),
            effect_id,
            issued_record_sha256: issued.record_sha256.clone(),
            journal_tip_before_sha256: issued.record.journal_tip_before_sha256.clone(),
            operation_nonce: operation_nonce.to_string(),
            previous_record_sha256: issued.record.previous_record_sha256.clone(),
            process_epoch_sha256: epoch.binding_sha256.clone(),
            purpose,
            runner_epoch_sha256: self.runner_epoch_sha256.clone(),
            schema: ENVELOPE_SCHEMA_V3.to_string(),
            schema_version: 3,
        };
        envelope.validate_against(&issued.record, command)?;
        debug_assert_eq!(sha256(&issued.record_bytes), issued.record_sha256);
        let wire = RunnerWireCommandV3 {
            command: command.to_vec(),
            envelope,
            issued_record: issued.record,
        };
        let dispatch = (|| {
            write_canonical_frame_until(&self.command_pipe, &wire, deadline)?;
            let (bytes, response_deadline) = read_frame_until(&self.response_pipe, deadline)?;
            if response_deadline.monotonic_nanoseconds != deadline.monotonic_nanoseconds {
                return Err(invalid("runner response changed the dispatch deadline"));
            }
            let receipt: InertDispatchReceiptV3 = parse_canonical(&bytes, "dispatch receipt")?;
            if receipt.schema != DISPATCH_RECEIPT_SCHEMA_V3
                || receipt.schema_version != 3
                || receipt.authority.any()
                || receipt.dispatch_count != 1
                || receipt.runner_epoch_sha256 != self.runner_epoch_sha256
                || receipt.issued_record_sha256 != wire.envelope.issued_record_sha256
                || receipt.command_sha256 != wire.envelope.command_sha256
            {
                return Err(invalid("inert dispatch receipt changed bindings"));
            }
            Ok(receipt)
        })();
        match dispatch {
            Ok(receipt) => Ok(receipt),
            Err(error) => {
                let result = post_issue_error(error);
                if let Err(proof_error) = self.terminate_and_retain_proof(CLEANUP_TIMEOUT_V3) {
                    return Err(InertRunnerErrorV3::ChannelLostIssuedOrUncertain(format!(
                        "{result}; death evidence retention failed: {proof_error}",
                    )));
                }
                Err(result)
            }
        }
    }

    fn persist_issue<F>(
        &mut self,
        record: IssuedEffectRecordV3,
        persist: F,
    ) -> Result<DurablyIssuedOrUncertainV3, InertRunnerErrorV3>
    where
        F: FnOnce(&IssuedEffectRecordV3, &[u8]) -> io::Result<DurableIssuePersistenceReceiptV3>,
    {
        let record_bytes = canonical_bytes(&record)?;
        let record_sha256 = sha256(&record_bytes);
        let durable_binding = DurableIssuedBindingV3 {
            command_sha256: record.command_sha256.clone(),
            effect_id: record.effect_id,
            issued_record_sha256: record_sha256.clone(),
            journal_tip_before_sha256: record.journal_tip_before_sha256.clone(),
            operation_nonce: record.operation_nonce.clone(),
            purpose: record.purpose,
        };
        self.state = RunnerStateV3::IssuedOrUncertain;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            persist(&record, &record_bytes)
        })) {
            Ok(Ok(receipt)) => {
                receipt
                    .validate(&record, &record_sha256)
                    .map_err(|error| InertRunnerErrorV3::PersistenceUncertain(error.to_string()))?;
                self.durable_issued_binding = Some(durable_binding);
                Ok(DurablyIssuedOrUncertainV3 {
                    record,
                    record_bytes,
                    record_sha256,
                })
            }
            Ok(Err(error)) => Err(InertRunnerErrorV3::PersistenceUncertain(error.to_string())),
            Err(_) => Err(InertRunnerErrorV3::PersistenceUncertain(
                "persistence callback panicked".to_string(),
            )),
        }
    }

    pub fn prove_dead(
        mut self,
        timeout: Duration,
    ) -> Result<SameSupervisorRunnerDeathProofV3, InertRunnerErrorV3> {
        if self.retained_death_proof.is_none() {
            self.collect_death_proof(AbsoluteDeadlineV3::after(timeout)?)?;
        }
        self.retained_death_proof
            .take()
            .ok_or_else(|| invalid("runner death proof was not retained"))
    }

    pub fn terminate_and_prove_dead(
        mut self,
        timeout: Duration,
    ) -> Result<SameSupervisorRunnerDeathProofV3, InertRunnerErrorV3> {
        self.terminate_and_retain_proof(timeout)?;
        self.retained_death_proof
            .take()
            .ok_or_else(|| invalid("terminated runner death proof was not retained"))
    }

    fn terminate_and_retain_proof(&mut self, timeout: Duration) -> Result<(), InertRunnerErrorV3> {
        if self.retained_death_proof.is_some() {
            return Ok(());
        }
        if self.durable_issued_binding.is_none() {
            return Err(invalid(
                "runner has no durably persisted issued record to bind a death proof",
            ));
        }
        let pid = self.child.id() as libc::pid_t;
        let rc = unsafe { libc::kill(-pid, libc::SIGKILL) };
        if rc != 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() != Some(libc::ESRCH) {
                return Err(error.into());
            }
        }
        self.collect_death_proof(AbsoluteDeadlineV3::after(timeout)?)
    }

    fn collect_death_proof(
        &mut self,
        deadline: AbsoluteDeadlineV3,
    ) -> Result<(), InertRunnerErrorV3> {
        if self.retained_death_proof.is_some() {
            return Ok(());
        }
        let issued = self.durable_issued_binding.clone().ok_or_else(|| {
            invalid("runner never acquired a durably persisted issued-record binding")
        })?;
        // Do not call `try_wait` before consuming NOTE_EXIT: reaping first can
        // detach the process knote on macOS.  The death pipe is the non-reaping
        // readiness signal; both waits consume the same absolute deadline.
        wait_for_events_until(
            self.death_pipe.as_raw_fd(),
            libc::POLLIN | libc::POLLHUP,
            deadline,
        )?;
        wait_for_events_until(self.kqueue.as_raw_fd(), libc::POLLIN, deadline)?;
        let kqueue_note_exit_observed =
            observe_kqueue_exit(self.kqueue.as_raw_fd(), self.runner_identity.pid)?;
        let death_pipe_eof_observed = read_eof_until(&self.death_pipe, deadline)?;
        let status = self.child.wait()?;
        let kernel_identity_absent = match kernel_process_identity(self.runner_identity.pid) {
            Ok(identity) => identity != self.runner_identity,
            Err(InertRunnerErrorV3::Io(error)) if error.raw_os_error() == Some(libc::ESRCH) => true,
            Err(_) => false,
        };
        if !kqueue_note_exit_observed || !death_pipe_eof_observed || !kernel_identity_absent {
            return Err(invalid(format!(
                "old runner death signals did not agree: kqueue_note_exit={kqueue_note_exit_observed}, death_pipe_eof={death_pipe_eof_observed}, kernel_identity_absent={kernel_identity_absent}",
            )));
        }
        let receipt = PriorRunnerDeathReceiptV3 {
            authority: DisposableAuthorityV2::none(),
            command_sha256: issued.command_sha256,
            death_pipe_eof_observed,
            effect_id: issued.effect_id,
            exit_status: exit_status_code(status),
            issued_record_sha256: issued.issued_record_sha256,
            journal_tip_before_sha256: issued.journal_tip_before_sha256,
            kernel_identity_absent,
            kqueue_note_exit_observed,
            operation_nonce: issued.operation_nonce,
            process_epoch_sha256: self.process_epoch_sha256.clone(),
            purpose: issued.purpose,
            runner_epoch_sha256: self.runner_epoch_sha256.clone(),
            runner_kernel_start_microseconds: self.runner_identity.start_microseconds,
            runner_pid: self.runner_identity.pid,
            schema: DEATH_RECEIPT_SCHEMA_V3.to_string(),
            schema_version: 3,
            waitpid_observed: true,
        };
        receipt.validate()?;
        let receipt_sha256 = digest_canonical(&receipt)?;
        self.state = RunnerStateV3::Reaped;
        self.retained_death_proof = Some(SameSupervisorRunnerDeathProofV3 {
            receipt,
            receipt_sha256,
        });
        Ok(())
    }
}

impl Drop for LiveInertRunnerV3 {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            terminate_group_and_reap(&mut self.child);
        }
    }
}

/// Child entry point for a dedicated inert runner executable (and tests).
/// It consumes only inherited pipes and the retained lease descriptor.
pub fn run_inert_child_from_environment() -> Result<(), InertRunnerErrorV3> {
    run_inert_child_with_behavior(Duration::ZERO, InertChildResponseV3::Complete)
}

fn run_inert_child_with_delay(delay: Duration) -> Result<(), InertRunnerErrorV3> {
    run_inert_child_with_behavior(delay, InertChildResponseV3::Complete)
}

#[derive(Clone, Copy)]
enum InertChildResponseV3 {
    Complete,
    DropWithoutReceipt,
    PartialReceiptThenStall,
    StallBeforeCommand,
}

fn run_inert_child_with_behavior(
    delay: Duration,
    response_behavior: InertChildResponseV3,
) -> Result<(), InertRunnerErrorV3> {
    let session_deadline = AbsoluteDeadlineV3::from_environment()?;
    let command_fd = inherited_fd("HEPTA_INERT_RUNNER_COMMAND_FD_V3")?;
    let response_fd = inherited_fd("HEPTA_INERT_RUNNER_RESPONSE_FD_V3")?;
    let death_fd = inherited_fd("HEPTA_INERT_RUNNER_DEATH_FD_V3")?;
    let lease_fd = inherited_fd("HEPTA_INERT_RUNNER_LEASE_FD_V3")?;
    let command_pipe = unsafe { File::from_raw_fd(command_fd) };
    let response_pipe = unsafe { File::from_raw_fd(response_fd) };
    let _death_pipe = unsafe { File::from_raw_fd(death_fd) };
    let _retained_control_lease = unsafe { File::from_raw_fd(lease_fd) };
    set_nonblocking(command_pipe.as_raw_fd())?;
    set_nonblocking(response_pipe.as_raw_fd())?;
    set_no_sigpipe(response_pipe.as_raw_fd())?;

    let (request_bytes, startup_deadline) = read_frame_until(&command_pipe, session_deadline)?;
    let request: RunnerHelloRequestV3 = parse_canonical(&request_bytes, "runner hello request")?;
    if digest_canonical(&request.process_epoch)? != request.process_epoch_sha256
        || request.process_epoch.schema != PROCESS_EPOCH_SCHEMA_V3
        || request.process_epoch.schema_version != 3
        || request.process_epoch.boot_session_uuid != boot_session_uuid()?
    {
        return Err(invalid("parent process epoch binding changed"));
    }
    let parent_identity = kernel_process_identity(unsafe { libc::getppid() } as u32)?;
    if parent_identity.pid != request.process_epoch.pid
        || parent_identity.start_microseconds != request.process_epoch.kernel_start_microseconds
    {
        return Err(invalid("pipe peer differs from the process epoch owner"));
    }
    let identity = kernel_process_identity(unsafe { libc::getpid() } as u32)?;
    let hello = RunnerHelloV3 {
        challenge_sha256: sha256(request.challenge.as_bytes()),
        parent_kernel_start_microseconds: parent_identity.start_microseconds,
        parent_pid: parent_identity.pid,
        process_epoch_sha256: request.process_epoch_sha256,
        runner_kernel_start_microseconds: identity.start_microseconds,
        runner_nonce: random_hex(32)?,
        runner_pid: identity.pid,
        schema: RUNNER_HELLO_SCHEMA_V3.to_string(),
        schema_version: 3,
    };
    let runner_epoch_sha256 = digest_canonical(&hello)?;
    write_canonical_frame_until(&response_pipe, &hello, startup_deadline)?;

    if matches!(response_behavior, InertChildResponseV3::StallBeforeCommand) {
        std::thread::sleep(Duration::from_secs(5));
        return Ok(());
    }

    let (wire_bytes, dispatch_deadline) = read_frame_until(&command_pipe, session_deadline)?;
    let wire: RunnerWireCommandV3 = parse_canonical(&wire_bytes, "runner wire command")?;
    wire.envelope
        .validate_against(&wire.issued_record, &wire.command)?;
    if wire.envelope.process_epoch_sha256 != hello.process_epoch_sha256
        || wire.envelope.runner_epoch_sha256 != runner_epoch_sha256
    {
        return Err(invalid("command targets another process or runner epoch"));
    }
    if !delay.is_zero() {
        std::thread::sleep(delay);
    }
    if matches!(response_behavior, InertChildResponseV3::DropWithoutReceipt) {
        return Ok(());
    }
    let receipt = InertDispatchReceiptV3 {
        authority: DisposableAuthorityV2::none(),
        command_sha256: sha256(&wire.command),
        dispatch_count: 1,
        issued_record_sha256: wire.envelope.issued_record_sha256,
        runner_epoch_sha256,
        schema: DISPATCH_RECEIPT_SCHEMA_V3.to_string(),
        schema_version: 3,
    };
    if matches!(
        response_behavior,
        InertChildResponseV3::PartialReceiptThenStall
    ) {
        let prefix = &dispatch_deadline.monotonic_nanoseconds.to_be_bytes()[..4];
        let written = unsafe {
            libc::write(
                response_pipe.as_raw_fd(),
                prefix.as_ptr().cast(),
                prefix.len(),
            )
        };
        if written != prefix.len() as isize {
            return Err(io::Error::last_os_error().into());
        }
        std::thread::sleep(Duration::from_secs(5));
        return Ok(());
    }
    write_canonical_frame_until(&response_pipe, &receipt, dispatch_deadline)?;
    Ok(())
}

fn kernel_process_identity(pid: u32) -> Result<KernelProcessIdentityV3, InertRunnerErrorV3> {
    let mut info = MaybeUninit::<ProcBsdInfoV3>::zeroed();
    let expected = std::mem::size_of::<ProcBsdInfoV3>();
    let received = unsafe {
        proc_pidinfo(
            pid as libc::c_int,
            PROC_PIDTBSDINFO,
            0,
            info.as_mut_ptr().cast(),
            expected as libc::c_int,
        )
    };
    if received == 0 {
        return Err(io::Error::from_raw_os_error(libc::ESRCH).into());
    }
    if received < 0 {
        let error = io::Error::last_os_error();
        return Err(error.into());
    }
    if received as usize != expected {
        return Err(invalid("kernel process identity has an unexpected size"));
    }
    let info = unsafe { info.assume_init() };
    if info.pid != pid || info.start_seconds == 0 {
        return Err(invalid(
            "kernel process identity disagrees with requested PID",
        ));
    }
    let start_microseconds = info
        .start_seconds
        .checked_mul(1_000_000)
        .and_then(|value| value.checked_add(info.start_microseconds))
        .ok_or_else(|| invalid("kernel process start identity overflowed"))?;
    Ok(KernelProcessIdentityV3 {
        parent_pid: info.ppid,
        pid: info.pid,
        start_microseconds,
    })
}

fn boot_session_uuid() -> Result<String, InertRunnerErrorV3> {
    let name = CString::new("kern.bootsessionuuid").expect("static sysctl name");
    let mut length = 0usize;
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
        return Err(io::Error::last_os_error().into());
    }
    if !(2..=128).contains(&length) {
        return Err(invalid("boot session UUID has an invalid length"));
    }
    let mut bytes = vec![0u8; length];
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
        return Err(io::Error::last_os_error().into());
    }
    bytes.truncate(length);
    if bytes.last() == Some(&0) {
        bytes.pop();
    }
    let uuid = String::from_utf8(bytes).map_err(|_| invalid("boot session UUID is not UTF-8"))?;
    if uuid.len() != 36
        || !uuid
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() || byte == b'-')
    {
        return Err(invalid("boot session UUID shape changed"));
    }
    Ok(uuid)
}

fn set_close_on_exec(fd: RawFd) -> Result<(), InertRunnerErrorV3> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags < 0 || unsafe { libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC) } < 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}

fn set_nonblocking(fd: RawFd) -> Result<(), InertRunnerErrorV3> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags < 0 || unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}

fn set_no_sigpipe(fd: RawFd) -> Result<(), InertRunnerErrorV3> {
    if unsafe { libc::fcntl(fd, F_SETNOSIGPIPE_V3, 1) } < 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}

fn reject_child_fd_aliases(
    sources: &[RawFd; 4],
    targets: &[RawFd; 4],
) -> Result<(), InertRunnerErrorV3> {
    for (index, source) in sources.iter().enumerate() {
        if *source < 0
            || sources[..index].contains(source)
            || targets.contains(source)
            || unsafe { libc::fcntl(*source, libc::F_GETFD) } < 0
        {
            return Err(invalid(
                "child FD sources are closed, aliased, or collide with fixed targets",
            ));
        }
        let flags = unsafe { libc::fcntl(*source, libc::F_GETFD) };
        if flags & libc::FD_CLOEXEC == 0 {
            return Err(invalid("parent child-source FD lost CLOEXEC"));
        }
    }
    for target in targets {
        let flags = unsafe { libc::fcntl(*target, libc::F_GETFD) };
        if flags < 0 || flags & libc::FD_CLOEXEC == 0 {
            return Err(invalid(
                "fixed child FD reservation is closed or lost CLOEXEC in the parent",
            ));
        }
    }
    Ok(())
}

fn inherited_fd(name: &str) -> Result<RawFd, InertRunnerErrorV3> {
    let value = std::env::var(name).map_err(|_| invalid(format!("missing inherited {name}")))?;
    let fd = value
        .parse::<RawFd>()
        .map_err(|_| invalid(format!("invalid inherited {name}")))?;
    if fd < 0 || unsafe { libc::fcntl(fd, libc::F_GETFD) } < 0 {
        return Err(invalid(format!("closed inherited {name}")));
    }
    Ok(fd)
}

fn register_process_exit(descriptor: RawFd, pid: u32) -> Result<(), InertRunnerErrorV3> {
    let change = libc::kevent {
        ident: pid as libc::uintptr_t,
        filter: libc::EVFILT_PROC,
        flags: libc::EV_ADD | libc::EV_ENABLE | libc::EV_ONESHOT,
        fflags: libc::NOTE_EXIT,
        data: 0,
        udata: std::ptr::null_mut(),
    };
    let rc = unsafe {
        libc::kevent(
            descriptor,
            &change,
            1,
            std::ptr::null_mut(),
            0,
            std::ptr::null(),
        )
    };
    if rc != 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}

fn observe_kqueue_exit(descriptor: RawFd, expected_pid: u32) -> Result<bool, InertRunnerErrorV3> {
    let mut event = MaybeUninit::<libc::kevent>::zeroed();
    let timeout = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let rc = unsafe {
        libc::kevent(
            descriptor,
            std::ptr::null(),
            0,
            event.as_mut_ptr(),
            1,
            &timeout,
        )
    };
    if rc < 0 {
        return Err(io::Error::last_os_error().into());
    }
    if rc == 0 {
        return Ok(false);
    }
    let event = unsafe { event.assume_init() };
    validate_kqueue_exit_event(&event, expected_pid)
}

fn validate_kqueue_exit_event(
    event: &libc::kevent,
    expected_pid: u32,
) -> Result<bool, InertRunnerErrorV3> {
    if event.flags & libc::EV_ERROR != 0 {
        return Err(invalid("runner kqueue returned EV_ERROR"));
    }
    Ok(event.ident == expected_pid as libc::uintptr_t
        && event.filter == libc::EVFILT_PROC
        && event.fflags & libc::NOTE_EXIT != 0)
}

fn wait_for_events_until(
    fd: RawFd,
    events: libc::c_short,
    deadline: AbsoluteDeadlineV3,
) -> Result<libc::c_short, InertRunnerErrorV3> {
    let mut descriptor = libc::pollfd {
        fd,
        events,
        revents: 0,
    };
    loop {
        descriptor.revents = 0;
        let milliseconds = deadline.poll_timeout_milliseconds()?;
        let rc = unsafe { libc::poll(&mut descriptor, 1, milliseconds) };
        if rc > 0 {
            if descriptor.revents & (libc::POLLERR | libc::POLLNVAL) != 0 {
                return Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "runner pipe reported POLLERR/POLLNVAL",
                )
                .into());
            }
            if descriptor.revents & (events | libc::POLLHUP) != 0 {
                return Ok(descriptor.revents);
            }
            continue;
        }
        if rc == 0 {
            return Err(frame_deadline_expired());
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error.into());
        }
    }
}

fn terminate_group_and_reap(child: &mut Child) {
    let pid = child.id() as libc::pid_t;
    unsafe {
        libc::kill(-pid, libc::SIGKILL);
    }
    let _ = child.wait();
}

fn exit_status_code(status: ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    status
        .code()
        .unwrap_or_else(|| 128 + status.signal().unwrap_or(0))
}

fn monotonic_nanoseconds() -> Result<u64, InertRunnerErrorV3> {
    let mut value = MaybeUninit::<libc::timespec>::zeroed();
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_RAW, value.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error().into());
    }
    let value = unsafe { value.assume_init() };
    if value.tv_sec < 0 || value.tv_nsec < 0 || value.tv_nsec >= 1_000_000_000 {
        return Err(invalid("kernel monotonic clock returned an invalid value"));
    }
    (value.tv_sec as u64)
        .checked_mul(1_000_000_000)
        .and_then(|seconds| seconds.checked_add(value.tv_nsec as u64))
        .ok_or_else(|| invalid("kernel monotonic clock overflowed"))
}

fn frame_deadline_expired() -> InertRunnerErrorV3 {
    io::Error::new(
        io::ErrorKind::TimedOut,
        "runner frame absolute deadline expired",
    )
    .into()
}

fn post_issue_error(error: InertRunnerErrorV3) -> InertRunnerErrorV3 {
    if matches!(&error, InertRunnerErrorV3::Io(io_error) if io_error.kind() == io::ErrorKind::TimedOut)
    {
        InertRunnerErrorV3::TimeoutIssuedOrUncertain
    } else {
        InertRunnerErrorV3::ChannelLostIssuedOrUncertain(error.to_string())
    }
}

fn write_frame_until(
    writer: &File,
    bytes: &[u8],
    deadline: AbsoluteDeadlineV3,
) -> Result<(), InertRunnerErrorV3> {
    if bytes.len() > MAX_FRAME_BYTES_V3 {
        return Err(invalid("runner frame exceeds the bounded size"));
    }
    deadline.remaining_nanoseconds()?;
    let mut header = [0u8; FRAME_HEADER_BYTES_V3];
    header[..8].copy_from_slice(&deadline.monotonic_nanoseconds.to_be_bytes());
    header[8..].copy_from_slice(&(bytes.len() as u64).to_be_bytes());
    write_all_until(writer.as_raw_fd(), &header, deadline)?;
    write_all_until(writer.as_raw_fd(), bytes, deadline)?;
    Ok(())
}

fn read_frame_until(
    reader: &File,
    ambient_deadline: AbsoluteDeadlineV3,
) -> Result<(Vec<u8>, AbsoluteDeadlineV3), InertRunnerErrorV3> {
    let mut header = [0u8; FRAME_HEADER_BYTES_V3];
    read_exact_until(reader.as_raw_fd(), &mut header, ambient_deadline)?;
    let frame_deadline = AbsoluteDeadlineV3 {
        monotonic_nanoseconds: u64::from_be_bytes(
            header[..8].try_into().expect("fixed deadline header width"),
        ),
    };
    if frame_deadline.monotonic_nanoseconds > ambient_deadline.monotonic_nanoseconds {
        return Err(invalid(
            "frame deadline exceeds the ambient session deadline",
        ));
    }
    frame_deadline.remaining_nanoseconds()?;
    let length = u64::from_be_bytes(
        header[8..]
            .try_into()
            .expect("fixed frame-length header width"),
    );
    if length == 0 || length > MAX_FRAME_BYTES_V3 as u64 {
        return Err(invalid("runner frame length is outside the bound"));
    }
    let mut bytes = vec![0u8; length as usize];
    read_exact_until(reader.as_raw_fd(), &mut bytes, frame_deadline)?;
    Ok((bytes, frame_deadline))
}

fn write_canonical_frame_until<T: Serialize>(
    writer: &File,
    value: &T,
    deadline: AbsoluteDeadlineV3,
) -> Result<(), InertRunnerErrorV3> {
    write_frame_until(writer, &canonical_bytes(value)?, deadline)
}

fn write_all_until(
    fd: RawFd,
    mut bytes: &[u8],
    deadline: AbsoluteDeadlineV3,
) -> Result<(), InertRunnerErrorV3> {
    while !bytes.is_empty() {
        deadline.remaining_nanoseconds()?;
        let written = unsafe { libc::write(fd, bytes.as_ptr().cast(), bytes.len()) };
        if written > 0 {
            bytes = &bytes[written as usize..];
            continue;
        }
        if written == 0 {
            return Err(io::Error::new(io::ErrorKind::WriteZero, "runner pipe write zero").into());
        }
        let error = io::Error::last_os_error();
        if error.kind() == io::ErrorKind::Interrupted {
            continue;
        }
        if error.kind() == io::ErrorKind::WouldBlock {
            wait_for_events_until(fd, libc::POLLOUT, deadline)?;
            continue;
        }
        return Err(error.into());
    }
    Ok(())
}

fn read_exact_until(
    fd: RawFd,
    mut bytes: &mut [u8],
    deadline: AbsoluteDeadlineV3,
) -> Result<(), InertRunnerErrorV3> {
    while !bytes.is_empty() {
        deadline.remaining_nanoseconds()?;
        let read = unsafe { libc::read(fd, bytes.as_mut_ptr().cast(), bytes.len()) };
        if read > 0 {
            let (_, remaining) = bytes.split_at_mut(read as usize);
            bytes = remaining;
            continue;
        }
        if read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "runner frame ended before its declared length",
            )
            .into());
        }
        let error = io::Error::last_os_error();
        if error.kind() == io::ErrorKind::Interrupted {
            continue;
        }
        if error.kind() == io::ErrorKind::WouldBlock {
            wait_for_events_until(fd, libc::POLLIN, deadline)?;
            continue;
        }
        return Err(error.into());
    }
    Ok(())
}

fn read_eof_until(reader: &File, deadline: AbsoluteDeadlineV3) -> Result<bool, InertRunnerErrorV3> {
    let mut probe = [0u8; 1];
    loop {
        deadline.remaining_nanoseconds()?;
        let read = unsafe { libc::read(reader.as_raw_fd(), probe.as_mut_ptr().cast(), 1) };
        if read == 0 {
            return Ok(true);
        }
        if read > 0 {
            return Ok(false);
        }
        let error = io::Error::last_os_error();
        if error.kind() == io::ErrorKind::Interrupted {
            continue;
        }
        if error.kind() == io::ErrorKind::WouldBlock {
            wait_for_events_until(reader.as_raw_fd(), libc::POLLIN, deadline)?;
            continue;
        }
        return Err(error.into());
    }
}

fn parse_canonical<T>(bytes: &[u8], label: &str) -> Result<T, InertRunnerErrorV3>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let value = serde_json::from_slice::<T>(bytes)
        .map_err(|error| invalid(format!("{label} is invalid JSON: {error}")))?;
    if canonical_bytes(&value)? != bytes {
        return Err(invalid(format!("{label} is not canonical JSON")));
    }
    Ok(value)
}

fn canonical_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, InertRunnerErrorV3> {
    canonical_json(value).map_err(|error| invalid(error.to_string()))
}

fn digest_canonical<T: Serialize>(value: &T) -> Result<String, InertRunnerErrorV3> {
    Ok(sha256(&canonical_bytes(value)?))
}

fn random_hex(bytes: usize) -> Result<String, InertRunnerErrorV3> {
    let mut value = vec![0u8; bytes];
    OsRng
        .try_fill_bytes(&mut value)
        .map_err(|error| invalid(format!("OS randomness failed: {error}")))?;
    Ok(value.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn require_nonce(value: &str, label: &str) -> Result<(), InertRunnerErrorV3> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(invalid(format!("{label} must be 32-byte lowercase hex")));
    }
    if value.bytes().any(|byte| byte.is_ascii_uppercase()) {
        return Err(invalid(format!("{label} must be lowercase")));
    }
    Ok(())
}

fn require_command_size(command: &[u8]) -> Result<(), InertRunnerErrorV3> {
    if command.is_empty() || command.len() > MAX_COMMAND_BYTES_V3 {
        return Err(invalid("command size is outside the inert runner bound"));
    }
    Ok(())
}

fn require_sha256(value: &str, label: &str) -> Result<(), InertRunnerErrorV3> {
    require_nonce(value, label)
}

fn require_optional_sha256(value: &Option<String>, label: &str) -> Result<(), InertRunnerErrorV3> {
    if let Some(value) = value {
        require_sha256(value, label)?;
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> InertRunnerErrorV3 {
    InertRunnerErrorV3::Invalid(message.into())
}

#[cfg(test)]
#[path = "mac_inert_one_shot_runner_tests.rs"]
mod tests;
