//! Fail-closed Linux syscall boundaries used by the v8 native components.
//!
//! None of the opaque tokens in this module can be deserialized or assembled
//! from caller-supplied booleans. On non-Linux targets every constructor and
//! operation returns [`NativeSysErrorV8::UnsupportedPlatform`].

mod boot;
mod cgroup;
mod execveat;
mod lock;
mod machine_id;
mod metadata;
mod openat2;
mod pidfd;
mod procfs;
mod rename_noreplace;
mod signal;

pub use boot::*;
pub use cgroup::*;
pub use execveat::*;
pub use lock::*;
pub use machine_id::*;
pub(crate) use metadata::*;
pub use openat2::*;
pub use pidfd::*;
pub use procfs::*;
pub use rename_noreplace::*;
pub use signal::*;

#[cfg(test)]
mod boot_tests;
#[cfg(test)]
mod cgroup_tests;
#[cfg(all(test, target_os = "linux"))]
mod execveat_tests;
#[cfg(all(test, target_os = "linux"))]
mod lock_tests;
#[cfg(test)]
mod machine_id_tests;
#[cfg(test)]
mod metadata_tests;
#[cfg(all(test, target_os = "linux"))]
mod openat2_tests;
#[cfg(all(test, target_os = "linux"))]
mod pidfd_tests;
#[cfg(test)]
mod procfs_tests;
#[cfg(all(test, target_os = "linux"))]
mod rename_noreplace_tests;
#[cfg(test)]
mod signal_tests;

// Fork-based execveat tests can transiently inherit a live flock descriptor
// from another parallel test until the child execs. Serialize only those
// process-wide descriptor-lifetime fixtures; production code has no such
// global lock or test-only coupling.
#[cfg(all(test, target_os = "linux"))]
pub(crate) static PROCESS_FD_LIFETIME_TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[derive(Debug, thiserror::Error)]
pub enum NativeSysErrorV8 {
    #[error("Linux syscall boundary is unsupported on this platform: {0}")]
    UnsupportedPlatform(&'static str),
    #[error("invalid native syscall input: {0}")]
    InvalidInput(String),
    #[error("native identity mismatch: {0}")]
    IdentityMismatch(String),
    #[error("native race detected: {0}")]
    RaceDetected(String),
    #[error("{operation}: {source}")]
    Io {
        operation: &'static str,
        #[source]
        source: std::io::Error,
    },
}

pub type NativeSysResultV8<T> = Result<T, NativeSysErrorV8>;

#[cfg(target_os = "linux")]
pub(super) fn io_error(operation: &'static str, source: std::io::Error) -> NativeSysErrorV8 {
    NativeSysErrorV8::Io { operation, source }
}

#[cfg(not(target_os = "linux"))]
pub(super) fn unsupported(operation: &'static str) -> NativeSysErrorV8 {
    NativeSysErrorV8::UnsupportedPlatform(operation)
}
