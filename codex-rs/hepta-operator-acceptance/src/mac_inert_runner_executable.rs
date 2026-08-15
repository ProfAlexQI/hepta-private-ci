//! Executable and descriptor identity shared by the supervisor and the
//! minimal inert child build.
//!
//! This module intentionally contains no packaging digest, fixed path, or
//! launcher constructor.  A child-only build can therefore link the protocol
//! without embedding the supervisor's expected hash into the object whose
//! bytes that hash authenticates.

use crate::durable::sha256;
use serde::Deserialize;
use serde::Serialize;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::RawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::ExitStatus;
use std::ptr;
use std::time::Duration;

const MAX_FIXED_RUNNER_BYTES_V3: u64 = 512 * 1024 * 1024;
const CHILD_REAP_TIMEOUT_V3: Duration = Duration::from_secs(5);
#[cfg(test)]
const TEST_FORCE_BOUNDED_REAP_FAILURE_ENV_V3: &str = "HEPTA_TEST_FORCE_BOUNDED_REAP_FAILURE_V3";

#[derive(Clone, Copy)]
struct ChildReapDeadlineV3 {
    monotonic_nanoseconds: u64,
}

impl ChildReapDeadlineV3 {
    fn after(timeout: Duration) -> io::Result<Self> {
        Self::after_from(monotonic_nanoseconds(), timeout)
    }

    fn after_from(now: io::Result<u64>, timeout: Duration) -> io::Result<Self> {
        let timeout = u64::try_from(timeout.as_nanos())
            .map_err(|_| invalid("child reap timeout exceeds the monotonic clock range"))?;
        let monotonic_nanoseconds = now?
            .checked_add(timeout)
            .ok_or_else(|| invalid("child reap deadline overflowed"))?;
        Ok(Self {
            monotonic_nanoseconds,
        })
    }

    fn expired(&self) -> io::Result<bool> {
        self.expired_at(monotonic_nanoseconds())
    }

    fn expired_at(&self, now: io::Result<u64>) -> io::Result<bool> {
        Ok(now? >= self.monotonic_nanoseconds)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FixedFileIdentityV3 {
    pub(crate) birthtime_nanoseconds: i64,
    pub(crate) ctime_nanoseconds: i64,
    pub(crate) device: u64,
    pub(crate) file_type_and_mode: u16,
    pub(crate) flags: u32,
    pub(crate) generation: u32,
    pub(crate) group: u32,
    pub(crate) inode: u64,
    pub(crate) link_count: u64,
    pub(crate) mtime_nanoseconds: i64,
    pub(crate) owner: u32,
    pub(crate) rdev: u64,
    pub(crate) size: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FixedExecutableEvidenceV3 {
    pub(crate) identity: FixedFileIdentityV3,
    pub(crate) sha256: String,
}

impl FixedExecutableEvidenceV3 {
    pub(crate) fn sha256(&self) -> &str {
        &self.sha256
    }
}

/// Minimal, fail-closed owner for a PID returned by `posix_spawn`.
pub(crate) struct SpawnedInertChildV3 {
    pub(crate) pid: libc::pid_t,
    pub(crate) reaped: Option<ExitStatus>,
}

impl SpawnedInertChildV3 {
    pub(crate) fn id(&self) -> u32 {
        self.pid as u32
    }

    pub(crate) fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        if let Some(status) = self.reaped {
            return Ok(Some(status));
        }
        let mut raw_status = 0;
        let rc = unsafe { libc::waitpid(self.pid, &mut raw_status, libc::WNOHANG) };
        if rc == self.pid {
            let status = ExitStatus::from_raw(raw_status);
            self.reaped = Some(status);
            return Ok(Some(status));
        }
        if rc == 0 {
            return Ok(None);
        }
        if rc < 0 {
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                return Ok(None);
            }
            return Err(error);
        }
        Err(invalid("waitpid returned another process identifier"))
    }

    pub(crate) fn terminate_group_and_reap(&mut self) -> io::Result<()> {
        if self.try_wait()?.is_some() {
            return Ok(());
        }
        let deadline = ChildReapDeadlineV3::after(CHILD_REAP_TIMEOUT_V3)?;
        self.terminate_group_and_reap_until(deadline)
    }

    fn terminate_group_and_reap_until(&mut self, deadline: ChildReapDeadlineV3) -> io::Result<()> {
        let group_kill = send_sigkill_allow_absent(-self.pid);
        // The exact child PID remains reserved to this unreaped owner and
        // therefore cannot have been reused.  Kill it directly as well as
        // its intended process group: if process-group establishment ever
        // drifted or raced, group-only cleanup would otherwise wait for a
        // still-live runner and could outlast the protocol deadline.
        let child_kill = send_sigkill_allow_absent(self.pid);
        #[cfg(test)]
        if std::env::var_os(TEST_FORCE_BOUNDED_REAP_FAILURE_ENV_V3).is_some() {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "injected bounded inert runner reap failure",
            ));
        }
        loop {
            if deadline.expired()? {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "killed inert runner was not reaped before cleanup deadline",
                ));
            }
            if self.try_wait()?.is_some() {
                group_kill?;
                child_kill?;
                return Ok(());
            }
            let delay = libc::timespec {
                tv_sec: 0,
                tv_nsec: 1_000_000,
            };
            unsafe {
                libc::nanosleep(&delay, ptr::null_mut());
            }
        }
    }
}

fn send_sigkill_allow_absent(pid: libc::pid_t) -> io::Result<()> {
    if unsafe { libc::kill(pid, libc::SIGKILL) } == 0 {
        return Ok(());
    }
    let error = io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::ESRCH) {
        Ok(())
    } else {
        Err(error)
    }
}

impl Drop for SpawnedInertChildV3 {
    fn drop(&mut self) {
        if self.reaped.is_none() && self.terminate_group_and_reap().is_err() {
            // Losing the only PID owner would permit an untracked runner or
            // zombie to survive.  Fail-stop the supervisor instead.
            std::process::abort();
        }
    }
}

pub(crate) fn inspect_current_executable() -> io::Result<FixedExecutableEvidenceV3> {
    let mut length = 0u32;
    unsafe {
        _NSGetExecutablePath(ptr::null_mut(), &mut length);
    }
    if length == 0 || length > libc::PATH_MAX as u32 * 4 {
        return Err(invalid("current executable path length is invalid"));
    }
    let mut bytes = vec![0u8; length as usize];
    if unsafe { _NSGetExecutablePath(bytes.as_mut_ptr().cast(), &mut length) } != 0 {
        return Err(invalid("current executable path changed while reading"));
    }
    let path = CStr::from_bytes_until_nul(&bytes)
        .map_err(|_| invalid("current executable path is not NUL terminated"))?;
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    executable_evidence(&file)
}

pub(crate) fn inspect_executable_path(path: &Path) -> io::Result<FixedExecutableEvidenceV3> {
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| invalid("executable path contains NUL"))?;
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    executable_evidence(&file)
}

pub(crate) fn executable_evidence(file: &File) -> io::Result<FixedExecutableEvidenceV3> {
    let before = file_identity(file.as_raw_fd())?;
    if before.size <= 0 || before.size as u64 > MAX_FIXED_RUNNER_BYTES_V3 {
        return Err(invalid("fixed runner size is outside the closed bound"));
    }
    let mut bytes = vec![0u8; before.size as usize];
    let mut offset = 0usize;
    while offset < bytes.len() {
        let count = unsafe {
            libc::pread(
                file.as_raw_fd(),
                bytes[offset..].as_mut_ptr().cast(),
                bytes.len() - offset,
                offset as libc::off_t,
            )
        };
        if count > 0 {
            offset += count as usize;
            continue;
        }
        if count == 0 {
            return Err(invalid("fixed runner was truncated while hashing"));
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
    let after = file_identity(file.as_raw_fd())?;
    if before != after {
        return Err(invalid("fixed runner changed while hashing"));
    }
    Ok(FixedExecutableEvidenceV3 {
        identity: before,
        sha256: sha256(&bytes),
    })
}

pub(crate) fn file_identity(fd: RawFd) -> io::Result<FixedFileIdentityV3> {
    let mut status = MaybeUninit::<libc::stat>::zeroed();
    if unsafe { libc::fstat(fd, status.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(identity_from_stat(unsafe { status.assume_init() }))
}

pub(crate) fn path_identity(path: &CStr) -> io::Result<FixedFileIdentityV3> {
    let mut status = MaybeUninit::<libc::stat>::zeroed();
    if unsafe { libc::stat(path.as_ptr(), status.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(identity_from_stat(unsafe { status.assume_init() }))
}

fn identity_from_stat(status: libc::stat) -> FixedFileIdentityV3 {
    FixedFileIdentityV3 {
        birthtime_nanoseconds: status
            .st_birthtime
            .saturating_mul(1_000_000_000)
            .saturating_add(status.st_birthtime_nsec),
        ctime_nanoseconds: status
            .st_ctime
            .saturating_mul(1_000_000_000)
            .saturating_add(status.st_ctime_nsec),
        device: status.st_dev as u64,
        file_type_and_mode: status.st_mode,
        flags: status.st_flags,
        generation: status.st_gen,
        group: status.st_gid,
        inode: status.st_ino,
        link_count: status.st_nlink as u64,
        mtime_nanoseconds: status
            .st_mtime
            .saturating_mul(1_000_000_000)
            .saturating_add(status.st_mtime_nsec),
        owner: status.st_uid,
        rdev: status.st_rdev as u64,
        size: status.st_size,
    }
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn monotonic_nanoseconds() -> io::Result<u64> {
    let mut value = MaybeUninit::<libc::timespec>::zeroed();
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_RAW, value.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    let value = unsafe { value.assume_init() };
    if value.tv_sec < 0 || value.tv_nsec < 0 {
        return Err(invalid("monotonic clock returned an invalid value"));
    }
    (value.tv_sec as u64)
        .checked_mul(1_000_000_000)
        .and_then(|seconds| seconds.checked_add(value.tv_nsec as u64))
        .ok_or_else(|| invalid("monotonic clock overflowed"))
}

unsafe extern "C" {
    fn _NSGetExecutablePath(buffer: *mut libc::c_char, size: *mut u32) -> libc::c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_reap_deadline_rejects_clock_failure_and_overflow() {
        assert!(
            ChildReapDeadlineV3::after_from(
                Err(io::Error::other("injected monotonic clock failure")),
                CHILD_REAP_TIMEOUT_V3,
            )
            .is_err()
        );
        assert!(ChildReapDeadlineV3::after_from(Ok(u64::MAX), Duration::from_nanos(1)).is_err());
        let deadline = ChildReapDeadlineV3 {
            monotonic_nanoseconds: 10,
        };
        assert!(
            deadline
                .expired_at(Err(io::Error::other("injected deadline recheck failure")))
                .is_err()
        );
    }

    #[test]
    fn child_owner_source_has_no_blocking_waitpid_or_optional_deadline() {
        let source = include_str!("mac_inert_runner_executable.rs");
        let blocking_method = ["pub(crate) fn ", "wait("].concat();
        let blocking_waitpid = ["waitpid(self.pid, &mut raw_status, ", "0)"].concat();
        let optional_clock = ["monotonic_nanoseconds()", ".ok()"].concat();
        assert!(!source.contains(&blocking_method));
        assert!(!source.contains(&blocking_waitpid));
        assert!(!source.contains(&optional_clock));
    }
}
