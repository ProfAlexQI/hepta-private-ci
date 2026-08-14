use super::NativeSysErrorV8;
use super::NativeSysResultV8;
#[cfg(not(target_os = "linux"))]
use super::unsupported;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;

#[cfg(target_os = "linux")]
use super::io_error;
#[cfg(target_os = "linux")]
use std::ffi::CStr;
#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::FromRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::IntoRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt;

/// Metadata read from the opened descriptor, never from a pathname after open.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileIdentityV8 {
    device: u64,
    inode: u64,
    owner_uid: u32,
    owner_gid: u32,
    mode: u32,
    link_count: u64,
    size: u64,
}

impl FileIdentityV8 {
    pub fn device(&self) -> u64 {
        self.device
    }

    pub fn inode(&self) -> u64 {
        self.inode
    }

    pub fn owner_uid(&self) -> u32 {
        self.owner_uid
    }

    pub fn owner_gid(&self) -> u32 {
        self.owner_gid
    }

    /// Permission and special bits only; the file-type bits are excluded.
    pub fn mode(&self) -> u32 {
        self.mode
    }

    pub fn link_count(&self) -> u64 {
        self.link_count
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

/// An exact directory descriptor used as the anchor for all relative opens.
///
/// The descriptor is intentionally not exposed through `AsRawFd`: sibling
/// native code must use the operations in this module instead of silently
/// weakening the resolution policy.
#[derive(Debug)]
pub struct DirectoryAnchorV8 {
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    identity: FileIdentityV8,
}

impl DirectoryAnchorV8 {
    /// Opens a caller-selected directory anchor without following any path
    /// component. This establishes resolution safety, not trust: a live
    /// consumer must separately match the anchor with a frozen, root-owned
    /// state-root policy. All descendants are opened with openat2.
    pub fn open(path: &Path) -> NativeSysResultV8<Self> {
        open_anchor_impl(path)
    }

    pub fn identity(&self) -> FileIdentityV8 {
        self.identity
    }

    pub fn open_directory_beneath(&self, relative: &Path) -> NativeSysResultV8<Self> {
        open_directory_beneath_impl(self, relative)
    }

    pub fn open_regular_readonly_beneath(
        &self,
        relative: &Path,
    ) -> NativeSysResultV8<VerifiedFileFdV8> {
        open_regular_beneath_impl(self, relative)
    }

    /// Creates one direct child as a new 0600 regular file. It never follows a
    /// symlink and never opens or replaces an existing name.
    pub fn create_regular_leaf_exclusive(
        &self,
        leaf: &OsStr,
    ) -> NativeSysResultV8<CreatedRegularLeafV8> {
        create_regular_leaf_exclusive_impl(self, leaf)
    }

    /// Makes prior directory-entry changes durable. The anchor is deliberately
    /// opened read-only (not O_PATH) so this fsync is meaningful.
    pub fn sync_directory(&self) -> NativeSysResultV8<()> {
        sync_directory_impl(self)
    }

    /// Lists exactly one directory level through an independently opened
    /// anchored descriptor. Names are returned as raw platform strings in
    /// deterministic byte order; `.` and `..` are never included.
    pub fn list_leaf_names(&self) -> NativeSysResultV8<Vec<OsString>> {
        list_leaf_names_impl(self, usize::MAX)
    }

    /// Lists at most `maximum_entries` direct leaves. The enumeration aborts
    /// as soon as the next leaf would exceed the frozen caller budget, before
    /// allocating or reading the rest of an adversarial directory.
    pub fn list_leaf_names_bounded(
        &self,
        maximum_entries: usize,
    ) -> NativeSysResultV8<Vec<OsString>> {
        if maximum_entries == 0 {
            return Err(NativeSysErrorV8::InvalidInput(
                "anchored directory entry budget must be non-zero".to_string(),
            ));
        }
        list_leaf_names_impl(self, maximum_entries)
    }

    #[cfg(target_os = "linux")]
    pub(super) fn raw_fd(&self) -> libc::c_int {
        self.descriptor.as_raw_fd()
    }
}

/// A just-created leaf that has not yet been durably populated.
///
/// The only state transition consumes this token and yields
/// [`SyncedRegularLeafV8`], preventing an unsynced token from being mistaken
/// for a durable record.
#[derive(Debug)]
pub struct CreatedRegularLeafV8 {
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    leaf: Box<OsStr>,
}

impl CreatedRegularLeafV8 {
    pub fn leaf(&self) -> &OsStr {
        &self.leaf
    }

    pub fn write_all_and_sync(self, bytes: &[u8]) -> NativeSysResultV8<SyncedRegularLeafV8> {
        self.write_all_without_sync(bytes)?.sync_and_revalidate()
    }

    pub(crate) fn write_all_without_sync(
        self,
        bytes: &[u8],
    ) -> NativeSysResultV8<WrittenRegularLeafV8> {
        write_all_without_sync_impl(self, bytes)
    }
}

/// A fully written leaf that has not yet crossed the file-fsync boundary.
/// It is crate-private so no external caller can mistake it for durability.
#[derive(Debug)]
pub(crate) struct WrittenRegularLeafV8 {
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    #[cfg(target_os = "linux")]
    leaf: Box<OsStr>,
    #[cfg(target_os = "linux")]
    expected_size: u64,
}

impl WrittenRegularLeafV8 {
    pub(crate) fn sync_and_revalidate(self) -> NativeSysResultV8<SyncedRegularLeafV8> {
        sync_and_revalidate_written_impl(self)
    }
}

/// An opaque proof that a newly created 0600 leaf was fully written, fsynced,
/// and revalidated through its still-open descriptor.
#[derive(Debug)]
pub struct SyncedRegularLeafV8 {
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    leaf: Box<OsStr>,
    identity: FileIdentityV8,
}

impl SyncedRegularLeafV8 {
    pub fn leaf(&self) -> &OsStr {
        &self.leaf
    }

    pub fn identity(&self) -> FileIdentityV8 {
        self.identity
    }

    /// Revalidates the open descriptor after any later publication step.
    pub fn revalidate(&self) -> NativeSysResultV8<()> {
        revalidate_synced_leaf_impl(self)
    }
}

/// A regular file reached through the mandatory openat2 policy.
#[derive(Debug)]
pub struct VerifiedFileFdV8 {
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    identity: FileIdentityV8,
}

impl VerifiedFileFdV8 {
    pub fn identity(&self) -> FileIdentityV8 {
        self.identity
    }

    /// Reads the descriptor while retaining the original verified descriptor.
    pub fn read_all(&self, maximum_bytes: u64) -> NativeSysResultV8<Vec<u8>> {
        read_all_impl(self, maximum_bytes)
    }

    #[cfg(target_os = "linux")]
    pub(super) fn raw_fd(&self) -> libc::c_int {
        self.descriptor.as_raw_fd()
    }

    #[cfg(target_os = "linux")]
    pub(super) fn revalidate_identity(&self) -> NativeSysResultV8<()> {
        let observed = identity_for_fd(self.descriptor.as_raw_fd())?;
        if observed != self.identity {
            return Err(NativeSysErrorV8::RaceDetected(
                "opened file metadata changed after verification".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
const REQUIRED_RESOLVE_FLAGS: u64 = libc::RESOLVE_BENEATH
    | libc::RESOLVE_NO_SYMLINKS
    | libc::RESOLVE_NO_MAGICLINKS
    | libc::RESOLVE_NO_XDEV;

#[cfg(target_os = "linux")]
fn open_anchor_impl(path: &Path) -> NativeSysResultV8<DirectoryAnchorV8> {
    if !path.is_absolute() || path == Path::new("/") {
        return Err(NativeSysErrorV8::InvalidInput(
            "directory anchor must be a non-root absolute path".to_string(),
        ));
    }
    let root_path = c"/";
    // SAFETY: the static root path remains live and open retains no pointer.
    let root_fd = unsafe {
        libc::open(
            root_path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if root_fd < 0 {
        return Err(io_error(
            "open root directory for anchored resolution",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: a successful open returned a uniquely owned descriptor.
    let root = unsafe { OwnedFd::from_raw_fd(root_fd) };
    let relative = path
        .strip_prefix(Path::new("/"))
        .map_err(|_| NativeSysErrorV8::InvalidInput("anchor is not below root".to_string()))?;
    let descriptor = openat2_fixed(
        root.as_raw_fd(),
        relative,
        libc::O_RDONLY | libc::O_DIRECTORY,
    )?;
    let identity = identity_for_fd(descriptor.as_raw_fd())?;
    require_directory(descriptor.as_raw_fd())?;
    Ok(DirectoryAnchorV8 {
        descriptor,
        identity,
    })
}

#[cfg(target_os = "linux")]
fn create_regular_leaf_exclusive_impl(
    anchor: &DirectoryAnchorV8,
    leaf: &OsStr,
) -> NativeSysResultV8<CreatedRegularLeafV8> {
    let leaf = validate_leaf(leaf)?;
    // SAFETY: an all-zero `open_how` is the kernel-defined baseline; fields
    // are then set explicitly. This also tolerates future non-exhaustive libc
    // bindings without leaving any extension bytes uninitialized.
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(
        libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
    )
    .map_err(|_| NativeSysErrorV8::InvalidInput("invalid create flags".to_string()))?;
    how.mode = 0o600;
    how.resolve = REQUIRED_RESOLVE_FLAGS;
    // SAFETY: the leaf and open_how pointers remain live for the syscall. The
    // leaf is a single component and the kernel does not retain either pointer.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            anchor.raw_fd(),
            leaf.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if fd < 0 {
        return Err(io_error(
            "openat2 exclusive 0600 leaf creation",
            std::io::Error::last_os_error(),
        ));
    }
    let fd = libc::c_int::try_from(fd).map_err(|_| {
        NativeSysErrorV8::InvalidInput("openat2 returned an invalid descriptor".to_string())
    })?;
    // SAFETY: a non-negative descriptor returned by openat2 is uniquely owned.
    let descriptor = unsafe { OwnedFd::from_raw_fd(fd) };
    require_regular(descriptor.as_raw_fd())?;
    Ok(CreatedRegularLeafV8 {
        descriptor,
        leaf: OsStr::from_bytes(leaf.as_bytes()).into(),
    })
}

#[cfg(not(target_os = "linux"))]
fn create_regular_leaf_exclusive_impl(
    _anchor: &DirectoryAnchorV8,
    _leaf: &OsStr,
) -> NativeSysResultV8<CreatedRegularLeafV8> {
    Err(unsupported("exclusive anchored leaf creation"))
}

#[cfg(target_os = "linux")]
fn write_all_without_sync_impl(
    created: CreatedRegularLeafV8,
    bytes: &[u8],
) -> NativeSysResultV8<WrittenRegularLeafV8> {
    let CreatedRegularLeafV8 { descriptor, leaf } = created;
    let mut file = File::from(descriptor);
    file.write_all(bytes)
        .map_err(|source| io_error("write exclusive anchored leaf", source))?;
    let expected_size = u64::try_from(bytes.len())
        .map_err(|_| NativeSysErrorV8::InvalidInput("record bytes do not fit u64".to_string()))?;
    Ok(WrittenRegularLeafV8 {
        descriptor: file.into(),
        leaf,
        expected_size,
    })
}

#[cfg(not(target_os = "linux"))]
fn write_all_without_sync_impl(
    _created: CreatedRegularLeafV8,
    _bytes: &[u8],
) -> NativeSysResultV8<WrittenRegularLeafV8> {
    Err(unsupported("write exclusive anchored leaf"))
}

#[cfg(target_os = "linux")]
fn sync_and_revalidate_written_impl(
    written: WrittenRegularLeafV8,
) -> NativeSysResultV8<SyncedRegularLeafV8> {
    let WrittenRegularLeafV8 {
        descriptor,
        leaf,
        expected_size,
    } = written;
    let file = File::from(descriptor);
    file.sync_all()
        .map_err(|source| io_error("fsync exclusive anchored leaf", source))?;
    let descriptor: OwnedFd = file.into();
    require_regular(descriptor.as_raw_fd())?;
    let identity = identity_for_fd(descriptor.as_raw_fd())?;
    // 0600 is unaffected by any umask. Ownership must match the creating
    // process, and a durable record is never allowed to be hard-linked.
    // SAFETY: geteuid has no preconditions or pointer arguments.
    let expected_uid = unsafe { libc::geteuid() };
    // SAFETY: getegid has no preconditions or pointer arguments.
    let expected_gid = unsafe { libc::getegid() };
    if identity.owner_uid != expected_uid
        || identity.owner_gid != expected_gid
        || identity.mode != 0o600
        || identity.link_count != 1
        || identity.size != expected_size
    {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "created leaf identity is uid={}:gid={} mode={:04o} nlink={} size={}, expected uid={expected_uid}:gid={expected_gid} mode=0600 nlink=1 size={expected_size}",
            identity.owner_uid,
            identity.owner_gid,
            identity.mode,
            identity.link_count,
            identity.size
        )));
    }
    Ok(SyncedRegularLeafV8 {
        descriptor,
        leaf,
        identity,
    })
}

#[cfg(not(target_os = "linux"))]
fn sync_and_revalidate_written_impl(
    _written: WrittenRegularLeafV8,
) -> NativeSysResultV8<SyncedRegularLeafV8> {
    Err(unsupported("fsync and revalidate written anchored leaf"))
}

#[cfg(target_os = "linux")]
fn revalidate_synced_leaf_impl(synced: &SyncedRegularLeafV8) -> NativeSysResultV8<()> {
    let observed = identity_for_fd(synced.descriptor.as_raw_fd())?;
    if observed != synced.identity {
        return Err(NativeSysErrorV8::RaceDetected(
            "synced leaf identity changed after fsync".to_string(),
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn revalidate_synced_leaf_impl(_synced: &SyncedRegularLeafV8) -> NativeSysResultV8<()> {
    Err(unsupported("revalidate synced anchored leaf"))
}

#[cfg(target_os = "linux")]
fn sync_directory_impl(anchor: &DirectoryAnchorV8) -> NativeSysResultV8<()> {
    // SAFETY: the descriptor is owned by the live anchor and fsync retains no
    // reference to it.
    let rc = unsafe { libc::fsync(anchor.raw_fd()) };
    if rc != 0 {
        return Err(io_error(
            "fsync anchored directory",
            std::io::Error::last_os_error(),
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn sync_directory_impl(_anchor: &DirectoryAnchorV8) -> NativeSysResultV8<()> {
    Err(unsupported("fsync anchored directory"))
}

#[cfg(target_os = "linux")]
fn list_leaf_names_impl(
    anchor: &DirectoryAnchorV8,
    maximum_entries: usize,
) -> NativeSysResultV8<Vec<OsString>> {
    let directory = open_current_directory_fixed(anchor)?;
    let raw_fd = directory.into_raw_fd();
    // SAFETY: `raw_fd` is a live directory descriptor transferred to DIR on
    // success. On failure fdopendir leaves ownership with the caller.
    let stream = unsafe { libc::fdopendir(raw_fd) };
    if stream.is_null() {
        let source = std::io::Error::last_os_error();
        // SAFETY: fdopendir failed, so raw_fd remains owned by this function.
        unsafe { libc::close(raw_fd) };
        return Err(io_error("fdopendir anchored directory", source));
    }

    let mut names = Vec::new();
    loop {
        // SAFETY: Linux exposes thread-local errno through this function. It is
        // reset so a null readdir result can distinguish EOF from an error.
        unsafe { *libc::__errno_location() = 0 };
        // SAFETY: `stream` remains open and is accessed only by this thread.
        let entry = unsafe { libc::readdir(stream) };
        if entry.is_null() {
            // SAFETY: read the same thread-local errno set by readdir.
            let errno = unsafe { *libc::__errno_location() };
            if errno != 0 {
                // SAFETY: stream is live and closed exactly once here.
                unsafe { libc::closedir(stream) };
                return Err(io_error(
                    "readdir anchored directory",
                    std::io::Error::from_raw_os_error(errno),
                ));
            }
            break;
        }
        // SAFETY: d_name is NUL-terminated within the live dirent returned by
        // readdir and is copied before the next call.
        let bytes = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
        if bytes == b"." || bytes == b".." {
            continue;
        }
        if bytes.is_empty() || bytes.contains(&b'/') {
            // SAFETY: stream is live and closed exactly once here.
            unsafe { libc::closedir(stream) };
            return Err(NativeSysErrorV8::IdentityMismatch(
                "readdir returned an invalid leaf name".to_string(),
            ));
        }
        names.push(OsStr::from_bytes(bytes).to_owned());
        if names.len() > maximum_entries {
            // SAFETY: stream is live and closed exactly once here.
            unsafe { libc::closedir(stream) };
            return Err(NativeSysErrorV8::IdentityMismatch(format!(
                "anchored directory exceeds the {maximum_entries}-entry budget"
            )));
        }
    }
    // SAFETY: stream is live and this is its single closing operation.
    if unsafe { libc::closedir(stream) } != 0 {
        return Err(io_error(
            "closedir anchored directory",
            std::io::Error::last_os_error(),
        ));
    }
    names.sort();
    Ok(names)
}

#[cfg(not(target_os = "linux"))]
fn list_leaf_names_impl(
    _anchor: &DirectoryAnchorV8,
    _maximum_entries: usize,
) -> NativeSysResultV8<Vec<OsString>> {
    Err(unsupported("list anchored directory leaves"))
}

#[cfg(target_os = "linux")]
fn open_current_directory_fixed(anchor: &DirectoryAnchorV8) -> NativeSysResultV8<OwnedFd> {
    // SAFETY: see the fixed open_how construction above.
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags =
        u64::try_from(libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .map_err(|_| NativeSysErrorV8::InvalidInput("invalid directory flags".to_string()))?;
    how.resolve = REQUIRED_RESOLVE_FLAGS;
    let current = c".";
    // SAFETY: `current` and `how` remain live for the syscall and the kernel
    // retains neither pointer.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            anchor.raw_fd(),
            current.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if fd < 0 {
        return Err(io_error(
            "openat2 anchored directory for scan",
            std::io::Error::last_os_error(),
        ));
    }
    let fd = libc::c_int::try_from(fd).map_err(|_| {
        NativeSysErrorV8::InvalidInput("openat2 returned an invalid descriptor".to_string())
    })?;
    // SAFETY: a successful openat2 returned a new descriptor uniquely owned.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

#[cfg(not(target_os = "linux"))]
fn open_anchor_impl(_path: &Path) -> NativeSysResultV8<DirectoryAnchorV8> {
    Err(unsupported("open directory anchor"))
}

#[cfg(target_os = "linux")]
fn open_directory_beneath_impl(
    anchor: &DirectoryAnchorV8,
    relative: &Path,
) -> NativeSysResultV8<DirectoryAnchorV8> {
    let descriptor = openat2_fixed(
        anchor.raw_fd(),
        relative,
        libc::O_RDONLY | libc::O_DIRECTORY,
    )?;
    require_directory(descriptor.as_raw_fd())?;
    let identity = identity_for_fd(descriptor.as_raw_fd())?;
    Ok(DirectoryAnchorV8 {
        descriptor,
        identity,
    })
}

#[cfg(not(target_os = "linux"))]
fn open_directory_beneath_impl(
    _anchor: &DirectoryAnchorV8,
    _relative: &Path,
) -> NativeSysResultV8<DirectoryAnchorV8> {
    Err(unsupported("openat2 directory beneath anchor"))
}

#[cfg(target_os = "linux")]
fn open_regular_beneath_impl(
    anchor: &DirectoryAnchorV8,
    relative: &Path,
) -> NativeSysResultV8<VerifiedFileFdV8> {
    let descriptor = openat2_fixed(anchor.raw_fd(), relative, libc::O_RDONLY)?;
    require_regular(descriptor.as_raw_fd())?;
    let identity = identity_for_fd(descriptor.as_raw_fd())?;
    Ok(VerifiedFileFdV8 {
        descriptor,
        identity,
    })
}

#[cfg(not(target_os = "linux"))]
fn open_regular_beneath_impl(
    _anchor: &DirectoryAnchorV8,
    _relative: &Path,
) -> NativeSysResultV8<VerifiedFileFdV8> {
    Err(unsupported("openat2 regular file beneath anchor"))
}

#[cfg(target_os = "linux")]
fn openat2_fixed(
    directory_fd: libc::c_int,
    relative: &Path,
    access_flags: libc::c_int,
) -> NativeSysResultV8<OwnedFd> {
    validate_relative_path(relative)?;
    let relative = path_to_cstring(relative, "relative openat2 path contains NUL")?;
    // SAFETY: see the fixed open_how construction above.
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(access_flags | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .map_err(|_| NativeSysErrorV8::InvalidInput("invalid open flags".to_string()))?;
    how.resolve = REQUIRED_RESOLVE_FLAGS;
    // SAFETY: the path and open_how pointers remain valid for the syscall and
    // the kernel copies both structures before returning.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            directory_fd,
            relative.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if fd < 0 {
        let source = std::io::Error::last_os_error();
        return Err(io_error("openat2 with mandatory resolve policy", source));
    }
    let fd = libc::c_int::try_from(fd).map_err(|_| {
        NativeSysErrorV8::InvalidInput("openat2 returned an invalid descriptor".to_string())
    })?;
    // SAFETY: a non-negative descriptor returned by openat2 is uniquely owned.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

#[cfg(target_os = "linux")]
fn validate_relative_path(relative: &Path) -> NativeSysResultV8<()> {
    use std::path::Component;

    if relative.as_os_str().is_empty() || relative.is_absolute() {
        return Err(NativeSysErrorV8::InvalidInput(
            "openat2 path must be a non-empty relative path".to_string(),
        ));
    }
    for component in relative.components() {
        match component {
            Component::Normal(_) => {}
            _ => {
                return Err(NativeSysErrorV8::InvalidInput(
                    "openat2 path may contain only normal relative components".to_string(),
                ));
            }
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn path_to_cstring(path: &Path, message: &'static str) -> NativeSysResultV8<CString> {
    CString::new(path.as_os_str().as_bytes())
        .map_err(|_| NativeSysErrorV8::InvalidInput(message.to_string()))
}

#[cfg(target_os = "linux")]
fn validate_leaf(leaf: &OsStr) -> NativeSysResultV8<CString> {
    let bytes = leaf.as_bytes();
    if bytes.is_empty() || bytes == b"." || bytes == b".." || bytes.contains(&b'/') {
        return Err(NativeSysErrorV8::InvalidInput(
            "anchored name must be a non-empty single leaf component".to_string(),
        ));
    }
    CString::new(bytes)
        .map_err(|_| NativeSysErrorV8::InvalidInput("anchored leaf contains NUL".to_string()))
}

#[cfg(target_os = "linux")]
fn require_directory(fd: libc::c_int) -> NativeSysResultV8<()> {
    let stat = stat_for_fd(fd)?;
    if stat.st_mode & libc::S_IFMT != libc::S_IFDIR {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "descriptor is not a directory".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_regular(fd: libc::c_int) -> NativeSysResultV8<()> {
    let stat = stat_for_fd(fd)?;
    if stat.st_mode & libc::S_IFMT != libc::S_IFREG {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "descriptor is not a regular file".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn stat_for_fd(fd: libc::c_int) -> NativeSysResultV8<libc::stat> {
    // SAFETY: zero is a valid initial representation for `stat`, and fstat
    // initializes it before a successful return.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: `stat` points to writable storage for the duration of fstat.
    let rc = unsafe { libc::fstat(fd, &mut stat) };
    if rc != 0 {
        return Err(io_error(
            "fstat verified descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    Ok(stat)
}

#[cfg(target_os = "linux")]
pub(super) fn identity_for_fd(fd: libc::c_int) -> NativeSysResultV8<FileIdentityV8> {
    let stat = stat_for_fd(fd)?;
    Ok(FileIdentityV8 {
        device: stat.st_dev,
        inode: stat.st_ino,
        owner_uid: stat.st_uid,
        owner_gid: stat.st_gid,
        mode: stat.st_mode & 0o7777,
        link_count: stat.st_nlink,
        size: u64::try_from(stat.st_size).map_err(|_| {
            NativeSysErrorV8::IdentityMismatch("descriptor has a negative size".to_string())
        })?,
    })
}

#[cfg(target_os = "linux")]
fn read_all_impl(file: &VerifiedFileFdV8, maximum_bytes: u64) -> NativeSysResultV8<Vec<u8>> {
    if file.identity.size > maximum_bytes {
        return Err(NativeSysErrorV8::InvalidInput(format!(
            "verified file size {} exceeds maximum {maximum_bytes}",
            file.identity.size
        )));
    }
    let length = usize::try_from(file.identity.size).map_err(|_| {
        NativeSysErrorV8::InvalidInput("verified file does not fit address space".to_string())
    })?;
    let mut bytes = vec![0_u8; length];
    let mut offset = 0_usize;
    while offset < bytes.len() {
        let offset_i64 = i64::try_from(offset).map_err(|_| {
            NativeSysErrorV8::InvalidInput("verified file offset exceeds off_t".to_string())
        })?;
        // SAFETY: the remaining slice is writable, the descriptor is live,
        // and pread does not mutate its shared file offset.
        let read = unsafe {
            libc::pread(
                file.raw_fd(),
                bytes[offset..].as_mut_ptr().cast(),
                bytes.len() - offset,
                offset_i64,
            )
        };
        if read < 0 {
            return Err(io_error(
                "pread verified file descriptor",
                std::io::Error::last_os_error(),
            ));
        }
        if read == 0 {
            return Err(NativeSysErrorV8::RaceDetected(
                "verified file became shorter while reading".to_string(),
            ));
        }
        offset = offset
            .checked_add(usize::try_from(read).map_err(|_| {
                NativeSysErrorV8::InvalidInput("pread returned an invalid length".to_string())
            })?)
            .ok_or_else(|| {
                NativeSysErrorV8::InvalidInput("verified file offset overflow".to_string())
            })?;
    }
    file.revalidate_identity()?;
    Ok(bytes)
}

#[cfg(not(target_os = "linux"))]
fn read_all_impl(_file: &VerifiedFileFdV8, _maximum_bytes: u64) -> NativeSysResultV8<Vec<u8>> {
    Err(unsupported("read openat2-verified file"))
}
