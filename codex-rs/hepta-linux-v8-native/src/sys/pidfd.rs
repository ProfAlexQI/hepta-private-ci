use super::NativeSysResultV8;

#[cfg(target_os = "linux")]
use super::ProcfsRootV8;

#[cfg(target_os = "linux")]
use super::NativeSysErrorV8;
#[cfg(target_os = "linux")]
use super::io_error;
#[cfg(not(target_os = "linux"))]
use super::unsupported;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::FromRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;

/// A pidfd paired with the `/proc/<pid>/stat` start-time observed immediately
/// before and after `pidfd_open`.
#[derive(Debug)]
pub struct PidHandleV8 {
    pid: u32,
    start_ticks: u64,
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
}

impl PidHandleV8 {
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn start_ticks(&self) -> u64 {
        self.start_ticks
    }

    /// Returns true only when poll(2) reports that the process represented by
    /// this pidfd has exited.
    pub fn is_exited(&self) -> NativeSysResultV8<bool> {
        is_exited_impl(self)
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn raw_fd(&self) -> libc::c_int {
        self.descriptor.as_raw_fd()
    }
}

/// Opens a pidfd only if the process start-time is stable across the syscall.
/// This closes the pathname-PID reuse window without sending any signal.
pub fn pidfd_open_verified(pid: u32) -> NativeSysResultV8<PidHandleV8> {
    pidfd_open_verified_impl(pid)
}

/// Opens a pidfd while retaining the caller's already verified procfs root for
/// both sides of the PID-reuse check.  This is the only constructor used by
/// multi-process observations, so one proof can never splice observations
/// from two procfs mounts or PID namespaces.
#[cfg(target_os = "linux")]
pub(crate) fn pidfd_open_verified_with_procfs_v8(
    procfs: &ProcfsRootV8,
    pid: u32,
) -> NativeSysResultV8<PidHandleV8> {
    pidfd_open_verified_with_procfs_impl_v8(procfs, pid)
}

/// Binds a pidfd returned directly by a kernel peer-identity API to the
/// numeric PID observed through that same connected socket. Unlike
/// [`pidfd_open_verified`], this never reopens a caller-supplied numeric PID.
#[cfg(target_os = "linux")]
pub(crate) fn bind_kernel_peer_pidfd_verified(
    descriptor: OwnedFd,
    expected_pid: u32,
) -> NativeSysResultV8<PidHandleV8> {
    let procfs = ProcfsRootV8::open_fixed()?;
    bind_kernel_peer_pidfd_verified_with_procfs_v8(&procfs, descriptor, expected_pid)
}

#[cfg(target_os = "linux")]
pub(crate) fn bind_kernel_peer_pidfd_verified_with_procfs_v8(
    procfs: &ProcfsRootV8,
    descriptor: OwnedFd,
    expected_pid: u32,
) -> NativeSysResultV8<PidHandleV8> {
    if expected_pid == 0 || expected_pid > libc::pid_t::MAX as u32 {
        return Err(NativeSysErrorV8::InvalidInput(
            "peer pid must fit a positive pid_t".to_string(),
        ));
    }
    verify_pidfd_cloexec(descriptor.as_raw_fd())?;
    verify_pidfd_target(procfs, descriptor.as_raw_fd(), expected_pid)?;
    let start_before = procfs.read_process_stat(expected_pid)?.start_ticks;
    verify_pidfd_target(procfs, descriptor.as_raw_fd(), expected_pid)?;
    let start_after = procfs.read_process_stat(expected_pid)?.start_ticks;
    if start_before != start_after {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "peer pid {expected_pid} start-time changed while binding kernel pidfd"
        )));
    }
    let handle = PidHandleV8 {
        pid: expected_pid,
        start_ticks: start_after,
        descriptor,
    };
    if handle.is_exited()? {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "peer pid {expected_pid} exited while binding kernel pidfd"
        )));
    }
    verify_pidfd_target(procfs, handle.raw_fd(), expected_pid)?;
    procfs.revalidate()?;
    Ok(handle)
}

/// Binds a pidfd written atomically by clone3(CLONE_PIDFD) to the child PID
/// returned by that same clone3 syscall, reusing the caller's retained procfs
/// root.  No numeric pidfd reopen is performed.
#[cfg(target_os = "linux")]
pub(crate) fn bind_clone3_pidfd_with_procfs_v8(
    procfs: &ProcfsRootV8,
    descriptor: OwnedFd,
    expected_pid: u32,
) -> NativeSysResultV8<PidHandleV8> {
    bind_kernel_peer_pidfd_verified_with_procfs_v8(procfs, descriptor, expected_pid)
}

#[cfg(target_os = "linux")]
fn pidfd_open_verified_impl(pid: u32) -> NativeSysResultV8<PidHandleV8> {
    let procfs = ProcfsRootV8::open_fixed()?;
    pidfd_open_verified_with_procfs_impl_v8(&procfs, pid)
}

#[cfg(target_os = "linux")]
fn pidfd_open_verified_with_procfs_impl_v8(
    procfs: &ProcfsRootV8,
    pid: u32,
) -> NativeSysResultV8<PidHandleV8> {
    if pid == 0 || pid > libc::pid_t::MAX as u32 {
        return Err(NativeSysErrorV8::InvalidInput(
            "pid must fit a positive pid_t".to_string(),
        ));
    }
    let start_before = procfs.read_process_stat(pid)?.start_ticks;
    // SAFETY: pidfd_open receives only scalar arguments and returns a fresh
    // descriptor on success.
    let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid, 0_u32) };
    if fd < 0 {
        return Err(io_error("pidfd_open", std::io::Error::last_os_error()));
    }
    let fd = libc::c_int::try_from(fd).map_err(|_| {
        NativeSysErrorV8::InvalidInput("pidfd_open returned an invalid descriptor".to_string())
    })?;
    // SAFETY: a successful pidfd_open returns a uniquely owned descriptor.
    let descriptor = unsafe { OwnedFd::from_raw_fd(fd) };
    let start_after = procfs.read_process_stat(pid)?.start_ticks;
    if start_before != start_after {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "pid {pid} start-time changed across pidfd_open"
        )));
    }
    let handle = PidHandleV8 {
        pid,
        start_ticks: start_after,
        descriptor,
    };
    if handle.is_exited()? {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "pid {pid} exited during pidfd verification"
        )));
    }
    verify_pidfd_target(procfs, handle.raw_fd(), pid)?;
    procfs.revalidate()?;
    Ok(handle)
}

#[cfg(not(target_os = "linux"))]
fn pidfd_open_verified_impl(_pid: u32) -> NativeSysResultV8<PidHandleV8> {
    Err(unsupported("pidfd_open"))
}

#[cfg(target_os = "linux")]
fn is_exited_impl(handle: &PidHandleV8) -> NativeSysResultV8<bool> {
    let mut poll_fd = libc::pollfd {
        fd: handle.raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    // SAFETY: `poll_fd` is writable for one element and the zero timeout does
    // not block.
    let rc = unsafe { libc::poll(&mut poll_fd, 1, 0) };
    if rc < 0 {
        return Err(io_error("poll pidfd", std::io::Error::last_os_error()));
    }
    if rc == 0 {
        return Ok(false);
    }
    if poll_fd.revents & libc::POLLNVAL != 0 {
        return Err(NativeSysErrorV8::RaceDetected(
            "pidfd became invalid".to_string(),
        ));
    }
    Ok(poll_fd.revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR) != 0)
}

#[cfg(not(target_os = "linux"))]
fn is_exited_impl(_handle: &PidHandleV8) -> NativeSysResultV8<bool> {
    Err(unsupported("poll pidfd"))
}

#[cfg(target_os = "linux")]
fn verify_pidfd_cloexec(fd: libc::c_int) -> NativeSysResultV8<()> {
    // SAFETY: F_GETFD reads flags from a live descriptor and takes no pointer.
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags < 0 {
        return Err(io_error(
            "fcntl F_GETFD kernel peer pidfd",
            std::io::Error::last_os_error(),
        ));
    }
    if flags & libc::FD_CLOEXEC == 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "kernel peer pidfd is missing FD_CLOEXEC".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn verify_pidfd_target(
    procfs: &ProcfsRootV8,
    fd: libc::c_int,
    expected_pid: u32,
) -> NativeSysResultV8<()> {
    // SAFETY: getpid has no pointer arguments or preconditions.  A numeric
    // path deliberately avoids procfs's magic `self` link.
    let observer_pid = unsafe { libc::getpid() };
    let observer_pid = u32::try_from(observer_pid).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch("observer pid does not fit positive u32".to_string())
    })?;
    let fdinfo = procfs.read_regular_beneath(&format!("{observer_pid}/fdinfo/{fd}"), 16 * 1024)?;
    let fdinfo = std::str::from_utf8(&fdinfo).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch("pidfd fdinfo is not UTF-8/ASCII".to_string())
    })?;
    let observed_pid = parse_pidfd_fdinfo_pid(fdinfo)?;
    if observed_pid != expected_pid {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "kernel peer pidfd identifies pid {observed_pid}, expected {expected_pid}"
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub(super) fn parse_pidfd_fdinfo_pid(fdinfo: &str) -> NativeSysResultV8<u32> {
    let mut observed = None;
    for line in fdinfo.lines() {
        let Some(value) = line.strip_prefix("Pid:") else {
            continue;
        };
        if observed.is_some() {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "pidfd fdinfo contains multiple Pid fields".to_string(),
            ));
        }
        let value = value.trim();
        if value == "-1" {
            return Err(NativeSysErrorV8::RaceDetected(
                "kernel peer pidfd target has exited".to_string(),
            ));
        }
        let pid = value.parse::<u32>().map_err(|_| {
            NativeSysErrorV8::IdentityMismatch(
                "pidfd fdinfo Pid is not a positive unsigned integer".to_string(),
            )
        })?;
        if pid == 0 || pid > libc::pid_t::MAX as u32 {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "pidfd fdinfo Pid does not fit a positive pid_t".to_string(),
            ));
        }
        observed = Some(pid);
    }
    observed.ok_or_else(|| {
        NativeSysErrorV8::IdentityMismatch("pidfd fdinfo lacks an exact Pid field".to_string())
    })
}

#[cfg(target_os = "linux")]
pub(super) fn parse_proc_start_ticks(stat: &str) -> NativeSysResultV8<u64> {
    // comm is parenthesized and may itself contain spaces or parentheses, so
    // split after its final ')' rather than using a naive whole-line split.
    let closing = stat.rfind(')').ok_or_else(|| {
        NativeSysErrorV8::IdentityMismatch("process stat lacks closing comm delimiter".to_string())
    })?;
    let after_comm = stat.get(closing + 1..).ok_or_else(|| {
        NativeSysErrorV8::IdentityMismatch("process stat is truncated".to_string())
    })?;
    // `after_comm` begins at field 3 (state); starttime is field 22, index 19.
    let start = after_comm.split_whitespace().nth(19).ok_or_else(|| {
        NativeSysErrorV8::IdentityMismatch("process stat lacks field 22 starttime".to_string())
    })?;
    start.parse::<u64>().map_err(|_| {
        NativeSysErrorV8::IdentityMismatch(
            "process stat starttime is not an unsigned integer".to_string(),
        )
    })
}
