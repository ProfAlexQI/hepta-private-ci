#[cfg(target_os = "linux")]
use super::NativeSysErrorV8;
#[cfg(target_os = "linux")]
use super::NativeSysResultV8;

#[cfg(target_os = "linux")]
use super::io_error;

// `FS_IOC_GETFLAGS` and the flag values are a stable Linux UAPI.  libc does
// not expose the ioctl request or every flag on all supported targets, so keep
// the exact values beside the filesystem-specific policy that consumes them.
#[cfg(target_os = "linux")]
const FS_IOC_GETFLAGS_V8: libc::c_ulong = 0x8008_6601;
const FS_INDEX_FL_V8: u32 = 0x0000_1000;
const FS_EXTENT_FL_V8: u32 = 0x0008_0000;
const FS_EA_INODE_FL_V8: u32 = 0x0020_0000;
const FS_INLINE_DATA_FL_V8: u32 = 0x1000_0000;

pub(crate) const EXT_SUPER_MAGIC_V8: u32 = 0x0000_ef53;
pub(crate) const TMPFS_MAGIC_V8: u32 = 0x0102_1994;
pub(crate) const XFS_SUPER_MAGIC_V8: u32 = 0x5846_5342;
pub(crate) const OVERLAYFS_SUPER_MAGIC_V8: u32 = 0x794c_7630;
pub(crate) const BTRFS_SUPER_MAGIC_V8: u32 = 0x9123_683e;

/// Exact descriptor-derived security metadata for a trusted durable-state
/// node.  The inode flags are admitted only under the compiled filesystem
/// policy; no caller can supply an allow mask.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TrustedNodeMetadataV8 {
    filesystem_type: u32,
    inode_flags: u32,
    inode_flags_allow_mask: u32,
    mount_flags: u64,
    mount_id: u64,
}

impl TrustedNodeMetadataV8 {
    pub(crate) fn mount_id(self) -> u64 {
        self.mount_id
    }

    pub(crate) fn matches_filesystem_domain(self, other: Self) -> bool {
        self.filesystem_type == other.filesystem_type
            && self.inode_flags_allow_mask == other.inode_flags_allow_mask
            && self.mount_flags == other.mount_flags
            && self.mount_id == other.mount_id
    }
}

/// The only inode bits accepted for each supported local filesystem.
///
/// ext-family directories can acquire htree, extent, EA-inode, or inline-data
/// representation bits as a filesystem-managed consequence of normal writes.
/// Overlayfs may expose those same backing-inode representation bits.  Every
/// user-policy bit (immutable, append, compression, no-COW, project inherit,
/// casefold, verity, DAX, and so on) is deliberately absent.  tmpfs, XFS, and
/// Btrfs must report zero until a separately reviewed exact policy exists.
pub(crate) const TRUSTED_FILESYSTEM_FLAG_POLICIES_V8: [(u32, u32); 5] = [
    (
        EXT_SUPER_MAGIC_V8,
        FS_INDEX_FL_V8 | FS_EXTENT_FL_V8 | FS_EA_INODE_FL_V8 | FS_INLINE_DATA_FL_V8,
    ),
    (TMPFS_MAGIC_V8, 0),
    (XFS_SUPER_MAGIC_V8, 0),
    (
        OVERLAYFS_SUPER_MAGIC_V8,
        FS_INDEX_FL_V8 | FS_EXTENT_FL_V8 | FS_EA_INODE_FL_V8 | FS_INLINE_DATA_FL_V8,
    ),
    (BTRFS_SUPER_MAGIC_V8, 0),
];

pub(crate) fn trusted_inode_flag_allow_mask_v8(filesystem_type: u32) -> Option<u32> {
    TRUSTED_FILESYSTEM_FLAG_POLICIES_V8
        .iter()
        .find_map(|(observed, mask)| (*observed == filesystem_type).then_some(*mask))
}

#[cfg(target_os = "linux")]
pub(crate) fn observe_trusted_node_metadata_v8(
    fd: libc::c_int,
) -> NativeSysResultV8<TrustedNodeMetadataV8> {
    if fd < 0 {
        return Err(NativeSysErrorV8::InvalidInput(
            "trusted metadata descriptor must be nonnegative".to_string(),
        ));
    }

    // ACLs are represented by system.posix_acl_* xattrs on every admitted
    // local filesystem.  The v8 durable namespace has an exact empty-xattr
    // policy, so both ACLs and arbitrary user/security xattrs fail closed.  A
    // second descriptor query after every other probe detects concurrent
    // introduction/removal during the observation window.
    require_empty_xattr_roster_v8(fd)?;

    // SAFETY: zero initializes a writable statfs buffer.
    let mut filesystem: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: fd is live and `filesystem` remains writable for the call.
    if unsafe { libc::fstatfs(fd, &mut filesystem) } != 0 {
        return Err(io_error(
            "fstatfs trusted durable-state descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    let filesystem_type = filesystem.f_type as u32;
    let inode_flags_allow_mask =
        trusted_inode_flag_allow_mask_v8(filesystem_type).ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch(format!(
                "trusted durable-state filesystem type 0x{filesystem_type:08x} is not compiled"
            ))
        })?;

    // SAFETY: zero initializes a writable statvfs buffer.
    let mut mount: libc::statvfs = unsafe { std::mem::zeroed() };
    // SAFETY: fd is live and `mount` remains writable for the call.
    if unsafe { libc::fstatvfs(fd, &mut mount) } != 0 {
        return Err(io_error(
            "fstatvfs trusted durable-state descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    let mount_flags = mount.f_flag as u64;
    if mount_flags & libc::ST_RDONLY as u64 != 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "trusted durable-state filesystem is read-only".to_string(),
        ));
    }

    let mut inode_flags: libc::c_int = 0;
    // SAFETY: the request is the stable FS_IOC_GETFLAGS UAPI; fd is live and
    // the pointer names one writable C int for the duration of the ioctl.
    if unsafe { libc::ioctl(fd, FS_IOC_GETFLAGS_V8, &mut inode_flags) } != 0 {
        return Err(io_error(
            "FS_IOC_GETFLAGS trusted durable-state descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    let inode_flags = inode_flags as u32;
    if inode_flags & !inode_flags_allow_mask != 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "trusted durable-state inode flags 0x{inode_flags:08x} exceed filesystem 0x{filesystem_type:08x} allow-mask 0x{inode_flags_allow_mask:08x}"
        )));
    }

    let mount_id = statx_mount_id_for_fd_v8(fd)?;
    require_empty_xattr_roster_v8(fd)?;
    Ok(TrustedNodeMetadataV8 {
        filesystem_type,
        inode_flags,
        inode_flags_allow_mask,
        mount_flags,
        mount_id,
    })
}

#[cfg(target_os = "linux")]
fn require_empty_xattr_roster_v8(fd: libc::c_int) -> NativeSysResultV8<()> {
    // SAFETY: a null buffer and zero length request only the exact byte count
    // for the live descriptor's NUL-separated xattr-name roster.
    let length = unsafe { libc::flistxattr(fd, std::ptr::null_mut(), 0) };
    if length < 0 {
        return Err(io_error(
            "flistxattr trusted durable-state descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    if length != 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "trusted durable-state descriptor has an unapproved xattr or ACL".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn statx_mount_id_for_fd_v8(fd: libc::c_int) -> NativeSysResultV8<u64> {
    // SAFETY: zero initializes the kernel output structure.
    let mut statx: libc::statx = unsafe { std::mem::zeroed() };
    // SAFETY: AT_EMPTY_PATH binds the observation to fd; the empty C string
    // and writable output remain live and the kernel retains neither pointer.
    if unsafe {
        libc::statx(
            fd,
            c"".as_ptr(),
            libc::AT_EMPTY_PATH | libc::AT_NO_AUTOMOUNT | libc::AT_SYMLINK_NOFOLLOW,
            libc::STATX_BASIC_STATS | libc::STATX_MNT_ID,
            &mut statx,
        )
    } != 0
    {
        return Err(io_error(
            "statx trusted durable-state descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    if statx.stx_mask & libc::STATX_MNT_ID == 0 || statx.stx_mnt_id == 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "trusted durable-state descriptor lacks a statx mount id".to_string(),
        ));
    }
    Ok(statx.stx_mnt_id)
}
