use super::BTRFS_SUPER_MAGIC_V8;
use super::EXT_SUPER_MAGIC_V8;
use super::OVERLAYFS_SUPER_MAGIC_V8;
use super::TMPFS_MAGIC_V8;
use super::TRUSTED_FILESYSTEM_FLAG_POLICIES_V8;
use super::XFS_SUPER_MAGIC_V8;
use super::trusted_inode_flag_allow_mask_v8;

#[test]
fn filesystem_flag_policy_is_closed_world_and_filesystem_specific() {
    assert_eq!(TRUSTED_FILESYSTEM_FLAG_POLICIES_V8.len(), 5);
    assert_ne!(
        trusted_inode_flag_allow_mask_v8(EXT_SUPER_MAGIC_V8),
        trusted_inode_flag_allow_mask_v8(TMPFS_MAGIC_V8)
    );
    assert_ne!(
        trusted_inode_flag_allow_mask_v8(OVERLAYFS_SUPER_MAGIC_V8),
        trusted_inode_flag_allow_mask_v8(XFS_SUPER_MAGIC_V8)
    );
    assert_eq!(trusted_inode_flag_allow_mask_v8(TMPFS_MAGIC_V8), Some(0));
    assert_eq!(
        trusted_inode_flag_allow_mask_v8(XFS_SUPER_MAGIC_V8),
        Some(0)
    );
    assert_eq!(
        trusted_inode_flag_allow_mask_v8(BTRFS_SUPER_MAGIC_V8),
        Some(0)
    );
    assert_eq!(trusted_inode_flag_allow_mask_v8(0), None);
    assert_eq!(trusted_inode_flag_allow_mask_v8(0xdead_beef), None);
}

#[test]
fn forbidden_policy_bits_are_never_smuggled_by_a_generic_zero_assumption() {
    const FS_IMMUTABLE_FL: u32 = 0x0000_0010;
    const FS_APPEND_FL: u32 = 0x0000_0020;
    const FS_NOCOW_FL: u32 = 0x0080_0000;
    const FS_PROJINHERIT_FL: u32 = 0x2000_0000;
    for (filesystem, allow_mask) in TRUSTED_FILESYSTEM_FLAG_POLICIES_V8 {
        assert_eq!(
            allow_mask & (FS_IMMUTABLE_FL | FS_APPEND_FL | FS_NOCOW_FL | FS_PROJINHERIT_FL),
            0,
            "filesystem 0x{filesystem:08x} admits a policy-changing inode bit"
        );
    }
}
