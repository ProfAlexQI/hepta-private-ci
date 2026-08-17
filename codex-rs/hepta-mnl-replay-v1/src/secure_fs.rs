use std::fs::File;
use std::os::fd::OwnedFd;
use std::os::unix::ffi::OsStrExt;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use rustix::fs::FileType;
use rustix::fs::Mode;
use rustix::fs::OFlags;
use rustix::fs::ResolveFlags;

use crate::ReplayStoreErrorV1;
use crate::ReplayStoreResultV1;
use crate::error::invalid;
use crate::error::syscall_error;

pub(crate) const DIRECTORY_MODE: u32 = 0o700;
pub(crate) const CLAIM_MODE: u32 = 0o600;

const DIRECTORY_OPEN_FLAGS: OFlags = OFlags::RDONLY
    .union(OFlags::DIRECTORY)
    .union(OFlags::NOFOLLOW)
    .union(OFlags::CLOEXEC);
const ROOT_RESOLVE_FLAGS: ResolveFlags = ResolveFlags::BENEATH
    .union(ResolveFlags::NO_SYMLINKS)
    .union(ResolveFlags::NO_MAGICLINKS);
const DESCENDANT_RESOLVE_FLAGS: ResolveFlags = ROOT_RESOLVE_FLAGS.union(ResolveFlags::NO_XDEV);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FileIdentityV1 {
    pub(crate) device: u64,
    pub(crate) inode: u64,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) mode: u32,
    pub(crate) link_count: u64,
    pub(crate) size: i64,
    pub(crate) file_type: FileType,
}

pub(crate) fn validate_absolute_store_root(root: &Path) -> ReplayStoreResultV1<PathBuf> {
    if !root.is_absolute() || root == Path::new("/") {
        return Err(invalid(
            "replay-store root must be an absolute non-root path",
        ));
    }
    let mut canonical = PathBuf::from("/");
    let mut saw_normal = false;
    for component in root.components() {
        match component {
            Component::RootDir => {}
            Component::Normal(value) => {
                saw_normal = true;
                canonical.push(value);
            }
            _ => {
                return Err(invalid(
                    "replay-store root must contain only canonical absolute components",
                ));
            }
        }
    }
    if !saw_normal
        || root.as_os_str().as_bytes().contains(&0)
        || canonical.as_os_str().as_bytes() != root.as_os_str().as_bytes()
    {
        return Err(invalid("replay-store root path is not canonical"));
    }
    Ok(root.to_path_buf())
}

pub(crate) fn same_directory_identity(left: FileIdentityV1, right: FileIdentityV1) -> bool {
    left.device == right.device
        && left.inode == right.inode
        && left.uid == right.uid
        && left.gid == right.gid
        && left.mode == right.mode
        && left.link_count == right.link_count
        && left.file_type == right.file_type
}

pub(crate) fn validate_leaf(leaf: &str, label: &str) -> ReplayStoreResultV1<()> {
    if leaf.is_empty()
        || leaf.len() > 128
        || leaf.as_bytes().contains(&0)
        || !leaf.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_' | b'.')
        })
        || leaf == "."
        || leaf == ".."
    {
        return Err(invalid(format!(
            "{label} must be one canonical relative leaf",
        )));
    }
    Ok(())
}

pub(crate) fn open_root_directory(root: &Path) -> ReplayStoreResultV1<OwnedFd> {
    let root = validate_absolute_store_root(root)?;
    let slash = rustix::fs::open(
        "/",
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map_err(|error| syscall_error("open fixed filesystem root", error))?;
    let relative = root
        .strip_prefix("/")
        .map_err(|_| invalid("replay-store root cannot be made relative to slash"))?;
    rustix::fs::openat2(
        &slash,
        relative,
        DIRECTORY_OPEN_FLAGS,
        Mode::empty(),
        ROOT_RESOLVE_FLAGS,
    )
    .map_err(|error| syscall_error("openat2 replay-store root", error))
}

pub(crate) fn open_child_directory(parent: &OwnedFd, leaf: &str) -> ReplayStoreResultV1<OwnedFd> {
    validate_leaf(leaf, "replay-store namespace")?;
    rustix::fs::openat2(
        parent,
        leaf,
        DIRECTORY_OPEN_FLAGS,
        Mode::empty(),
        DESCENDANT_RESOLVE_FLAGS,
    )
    .map_err(|error| syscall_error("openat2 replay-store namespace", error))
}

pub(crate) fn probe_leaf_exists(parent: &OwnedFd, leaf: &str) -> bool {
    validate_leaf(leaf, "replay claim leaf").is_err()
        || rustix::fs::openat2(
            parent,
            leaf,
            OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
            DESCENDANT_RESOLVE_FLAGS,
        )
        .is_ok()
        || rustix::fs::statat(parent, leaf, rustix::fs::AtFlags::SYMLINK_NOFOLLOW).is_ok()
}

pub(crate) fn create_claim_file(parent: &OwnedFd, leaf: &str) -> ReplayStoreResultV1<File> {
    validate_leaf(leaf, "incoming replay claim leaf")?;
    let descriptor = rustix::fs::openat2(
        parent,
        leaf,
        OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::RUSR | Mode::WUSR,
        DESCENDANT_RESOLVE_FLAGS,
    )
    .map_err(|error| syscall_error("openat2 exclusive replay claim", error))?;
    rustix::fs::fchmod(&descriptor, Mode::RUSR | Mode::WUSR)
        .map_err(|error| syscall_error("fchmod replay claim to 0600", error))?;
    Ok(File::from(descriptor))
}

pub(crate) fn open_claim_file(parent: &OwnedFd, leaf: &str) -> ReplayStoreResultV1<File> {
    validate_leaf(leaf, "final replay claim leaf")?;
    let descriptor = rustix::fs::openat2(
        parent,
        leaf,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
        DESCENDANT_RESOLVE_FLAGS,
    )
    .map_err(|error| syscall_error("openat2 final replay claim", error))?;
    Ok(File::from(descriptor))
}

pub(crate) fn open_readonly_file_beneath(
    parent: &OwnedFd,
    leaf: &str,
    label: &str,
) -> ReplayStoreResultV1<File> {
    validate_leaf(leaf, label)?;
    let descriptor = rustix::fs::openat2(
        parent,
        leaf,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
        DESCENDANT_RESOLVE_FLAGS,
    )
    .map_err(|error| syscall_error("openat2 fixed read-only file", error))?;
    Ok(File::from(descriptor))
}

pub(crate) fn identity_for_fd<Fd: std::os::fd::AsFd>(
    descriptor: Fd,
) -> ReplayStoreResultV1<FileIdentityV1> {
    let stat = rustix::fs::fstat(descriptor)
        .map_err(|error| syscall_error("fstat replay-store object", error))?;
    Ok(FileIdentityV1 {
        device: stat.st_dev,
        inode: stat.st_ino,
        uid: stat.st_uid,
        gid: stat.st_gid,
        mode: Mode::from_raw_mode(stat.st_mode).bits(),
        link_count: stat.st_nlink,
        size: stat.st_size,
        file_type: FileType::from_raw_mode(stat.st_mode),
    })
}

pub(crate) fn require_directory_identity(
    identity: FileIdentityV1,
    expected_uid: u32,
    expected_gid: u32,
    label: &str,
) -> ReplayStoreResultV1<()> {
    if identity.file_type != FileType::Directory
        || identity.mode != DIRECTORY_MODE
        || identity.uid != expected_uid
        || identity.gid != expected_gid
        || identity.link_count < 2
    {
        return Err(ReplayStoreErrorV1::IdentityMismatch(format!(
            "{label} is not an owned 0700 directory",
        )));
    }
    Ok(())
}

pub(crate) fn require_claim_identity(
    identity: FileIdentityV1,
    expected_uid: u32,
    expected_gid: u32,
    expected_size: usize,
    label: &str,
) -> ReplayStoreResultV1<()> {
    let expected_size = i64::try_from(expected_size)
        .map_err(|_| invalid("replay claim byte length is not representable"))?;
    if identity.file_type != FileType::RegularFile
        || identity.mode != CLAIM_MODE
        || identity.uid != expected_uid
        || identity.gid != expected_gid
        || identity.link_count != 1
        || identity.size != expected_size
    {
        return Err(ReplayStoreErrorV1::IdentityMismatch(format!(
            "{label} is not the exact owned 0600 regular file",
        )));
    }
    Ok(())
}

pub(crate) fn rename_noreplace(
    parent: &OwnedFd,
    incoming: &str,
    final_leaf: &str,
) -> ReplayStoreResultV1<()> {
    validate_leaf(incoming, "incoming replay claim leaf")?;
    validate_leaf(final_leaf, "final replay claim leaf")?;
    rustix::fs::renameat_with(
        parent,
        incoming,
        parent,
        final_leaf,
        rustix::fs::RenameFlags::NOREPLACE,
    )
    .map_err(|error| syscall_error("renameat2 replay claim RENAME_NOREPLACE", error))
}

pub(crate) fn fsync_directory(directory: &OwnedFd) -> ReplayStoreResultV1<()> {
    rustix::fs::fsync(directory)
        .map_err(|error| syscall_error("fsync replay-store directory", error))
}
