use std::cell::Cell;
use std::ffi::OsStr;
use std::marker::PhantomData;
#[cfg(target_os = "linux")]
use std::path::Path;
use std::sync::Arc;

use super::DirectoryAnchorV8;
use super::FileIdentityV8;
use super::NativeSysResultV8;
#[cfg(not(target_os = "linux"))]
use super::unsupported;

#[cfg(target_os = "linux")]
use super::NativeSysErrorV8;
#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::FromRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt;

/// Process-lifetime singleton lock for one anchored state root. Dropping the
/// only token closes the descriptor and releases the kernel flock.
#[derive(Debug)]
pub struct StateRootLockV8 {
    identity: FileIdentityV8,
    leaf: Box<OsStr>,
    owner_pid: u32,
    session: Arc<StateRootLockSessionV8>,
    state_root_identity: FileIdentityV8,
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    /// Linux flock is tied to the open file description; the same token or a
    /// duplicated/inherited descriptor can therefore be shared by sibling
    /// threads or a fork. Keeping the token `!Sync`, together with mutable
    /// borrows and PID/session binding, preserves one writer in safe Rust.
    not_sync: PhantomData<Cell<()>>,
}

/// Process-memory identity for one successful lock acquisition. The Arc
/// allocation remains alive in any fresh active-attempt token, so dropping
/// and reacquiring the same lock inode—even in the same PID—cannot recreate
/// the old session identity.
#[derive(Debug)]
pub(crate) struct StateRootLockSessionV8 {
    _private: (),
}

impl StateRootLockV8 {
    pub fn identity(&self) -> FileIdentityV8 {
        self.identity
    }

    pub fn leaf(&self) -> &OsStr {
        &self.leaf
    }

    pub fn owner_pid(&self) -> u32 {
        self.owner_pid
    }

    pub(crate) fn session_handle(&self) -> Arc<StateRootLockSessionV8> {
        Arc::clone(&self.session)
    }

    pub(crate) fn matches_session(&self, session: &Arc<StateRootLockSessionV8>) -> bool {
        Arc::ptr_eq(&self.session, session)
    }

    pub fn state_root_identity(&self) -> FileIdentityV8 {
        self.state_root_identity
    }

    /// Reopens the lock name through the exact state-root anchor and proves it
    /// still names this live flocked inode. Call immediately before every
    /// durable state transition.
    pub fn revalidate_for_root(&self, state_root: &DirectoryAnchorV8) -> NativeSysResultV8<()> {
        revalidate_lock_impl(self, state_root)
    }
}

pub fn acquire_state_root_lock_v8(
    state_root: &DirectoryAnchorV8,
    leaf: &OsStr,
) -> NativeSysResultV8<StateRootLockV8> {
    acquire_state_root_lock_impl(state_root, leaf)
}

#[cfg(target_os = "linux")]
fn acquire_state_root_lock_impl(
    state_root: &DirectoryAnchorV8,
    leaf: &OsStr,
) -> NativeSysResultV8<StateRootLockV8> {
    let leaf_cstring = validate_leaf(leaf)?;
    // SAFETY: an all-zero `open_how` is the kernel-defined baseline; every
    // used field is then assigned explicitly.
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(libc::O_RDWR | libc::O_CREAT | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .map_err(|_| NativeSysErrorV8::InvalidInput("invalid lock open flags".to_string()))?;
    how.mode = 0o600;
    how.resolve = libc::RESOLVE_BENEATH
        | libc::RESOLVE_NO_SYMLINKS
        | libc::RESOLVE_NO_MAGICLINKS
        | libc::RESOLVE_NO_XDEV;
    // SAFETY: both pointers remain live for the syscall; the kernel retains
    // neither, and a successful call returns a new owned descriptor.
    let raw_fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            state_root.raw_fd(),
            leaf_cstring.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if raw_fd < 0 {
        return Err(super::io_error(
            "openat2 state-root singleton lock",
            std::io::Error::last_os_error(),
        ));
    }
    let raw_fd = libc::c_int::try_from(raw_fd).map_err(|_| {
        NativeSysErrorV8::InvalidInput("openat2 returned an invalid lock descriptor".to_string())
    })?;
    // SAFETY: the successful openat2 returned a unique descriptor.
    let descriptor = unsafe { OwnedFd::from_raw_fd(raw_fd) };
    let identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
    // SAFETY: zero initializes a valid stat buffer, filled by fstat on success.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat points to writable storage for the duration of fstat.
    if unsafe { libc::fstat(descriptor.as_raw_fd(), &mut stat) } != 0 {
        return Err(super::io_error(
            "fstat state-root singleton lock",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: geteuid/getegid have no pointer arguments or preconditions.
    let expected_uid = unsafe { libc::geteuid() };
    // SAFETY: see above.
    let expected_gid = unsafe { libc::getegid() };
    // SAFETY: getpid has no pointer arguments or preconditions.
    let owner_pid = unsafe { libc::getpid() };
    let owner_pid = u32::try_from(owner_pid).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch(
            "state-root lock owner PID is not a positive u32".to_string(),
        )
    })?;
    if owner_pid == 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "state-root lock owner PID is zero".to_string(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFREG
        || identity.owner_uid() != expected_uid
        || identity.owner_gid() != expected_gid
        || identity.mode() != 0o600
        || identity.link_count() != 1
        || identity.size() != 0
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "state-root lock must be an empty 0600 single-link regular file owned by the process"
                .to_string(),
        ));
    }
    // SAFETY: flock receives a live descriptor and scalar flags only.
    if unsafe { libc::flock(descriptor.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(super::io_error(
            "acquire state-root singleton flock",
            std::io::Error::last_os_error(),
        ));
    }
    state_root.sync_directory()?;
    Ok(StateRootLockV8 {
        identity,
        leaf: leaf.into(),
        owner_pid,
        session: Arc::new(StateRootLockSessionV8 { _private: () }),
        state_root_identity: state_root.identity(),
        descriptor,
        not_sync: PhantomData,
    })
}

#[cfg(target_os = "linux")]
fn revalidate_lock_impl(
    lock: &StateRootLockV8,
    state_root: &DirectoryAnchorV8,
) -> NativeSysResultV8<()> {
    if !state_root
        .identity()
        .matches_stable_directory(lock.state_root_identity)
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "singleton lock is being used with a different state root".to_string(),
        ));
    }
    // A forked child inherits the same open file description and therefore
    // the same flock. It must not inherit writer authority from the parent's
    // in-memory token.
    // SAFETY: getpid has no pointer arguments or preconditions.
    let current_pid = unsafe { libc::getpid() };
    if u32::try_from(current_pid).ok() != Some(lock.owner_pid) {
        return Err(NativeSysErrorV8::RaceDetected(
            "state-root lock token was inherited by a different process".to_string(),
        ));
    }
    let open_identity = super::openat2::identity_for_fd(lock.descriptor.as_raw_fd())?;
    if open_identity != lock.identity || open_identity.link_count() != 1 {
        return Err(NativeSysErrorV8::RaceDetected(
            "live singleton lock inode was changed or unlinked".to_string(),
        ));
    }
    let named = state_root.open_regular_readonly_beneath(Path::new(lock.leaf.as_ref()))?;
    if named.identity() != lock.identity {
        return Err(NativeSysErrorV8::RaceDetected(
            "singleton lock pathname no longer names the flocked inode".to_string(),
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn acquire_state_root_lock_impl(
    _state_root: &DirectoryAnchorV8,
    _leaf: &OsStr,
) -> NativeSysResultV8<StateRootLockV8> {
    Err(unsupported("state-root singleton lock"))
}

#[cfg(not(target_os = "linux"))]
fn revalidate_lock_impl(
    _lock: &StateRootLockV8,
    _state_root: &DirectoryAnchorV8,
) -> NativeSysResultV8<()> {
    Err(unsupported("revalidate state-root singleton lock"))
}

#[cfg(target_os = "linux")]
fn validate_leaf(leaf: &OsStr) -> NativeSysResultV8<CString> {
    let bytes = leaf.as_bytes();
    if bytes.is_empty() || bytes == b"." || bytes == b".." || bytes.contains(&b'/') {
        return Err(NativeSysErrorV8::InvalidInput(
            "state-root lock must be one non-empty leaf".to_string(),
        ));
    }
    CString::new(bytes)
        .map_err(|_| NativeSysErrorV8::InvalidInput("lock leaf contains NUL".to_string()))
}
