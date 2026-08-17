use std::io::Read;
use std::os::fd::AsFd;
use std::os::fd::OwnedFd;
use std::path::Path;

use rustix::fs::FileType;
use sha2::Digest;

use crate::ReplayStoreErrorV1;
use crate::ReplayStoreResultV1;
use crate::error::invalid;
use crate::error::io_error;
use crate::error::syscall_error;
use crate::secure_fs::FileIdentityV1;
use crate::secure_fs::identity_for_fd;
use crate::secure_fs::open_child_directory;
use crate::secure_fs::open_fixed_child_directory_across_mount;
use crate::secure_fs::open_readonly_file_beneath;
use crate::secure_fs::open_root_directory;

const BOOT_ID_BYTES: usize = 36;
const BOOT_ID_FILE_BYTES: usize = BOOT_ID_BYTES + 1;
const BOOT_ID_READ_LIMIT: u64 = (BOOT_ID_FILE_BYTES + 1) as u64;
const BOOT_ID_SHA256_DOMAIN: &[u8] = b"hepta.mnl.linux-boot-id.v1\0";
const PROC_ROOT_MODE: u32 = 0o555;
const PROC_BOOT_ID_MODE: u32 = 0o444;

#[derive(Debug)]
pub(crate) struct FixedProcfsBootIdSourceV1 {
    proc_root: OwnedFd,
    proc_root_identity: ProcfsDirectoryIdentityV1,
    proc_sys_root: OwnedFd,
    proc_sys_root_identity: ProcfsDirectoryIdentityV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProcfsDirectoryIdentityV1 {
    file: FileIdentityV1,
    mount_id: u64,
}

impl FixedProcfsBootIdSourceV1 {
    pub(crate) fn open() -> ReplayStoreResultV1<Self> {
        let proc_root = open_root_directory(Path::new("/proc"))?;
        let proc_root_identity = procfs_directory_identity_for_fd(&proc_root)?;
        require_proc_root(proc_root_identity.file)?;
        require_procfs(&proc_root)?;
        // Container runtimes commonly expose /proc/sys as its own read-only
        // procfs mount. Open that exact compile-time path as a second fixed
        // root, then restore NO_XDEV for every descendant below it.
        let proc_sys_root = open_fixed_child_directory_across_mount(&proc_root, "sys")?;
        let proc_sys_root_identity = procfs_directory_identity_for_fd(&proc_sys_root)?;
        require_proc_sys_root(proc_sys_root_identity.file)?;
        require_procfs(&proc_sys_root)?;
        Ok(Self {
            proc_root,
            proc_root_identity,
            proc_sys_root,
            proc_sys_root_identity,
        })
    }

    pub(crate) fn observe_sha256(&self) -> ReplayStoreResultV1<String> {
        let proc_root_identity = procfs_directory_identity_for_fd(&self.proc_root)?;
        require_proc_root(proc_root_identity.file)?;
        require_procfs(&self.proc_root)?;
        if !same_procfs_directory_identity(proc_root_identity, self.proc_root_identity) {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "retained procfs root identity drifted before boot-id read".to_string(),
            ));
        }
        let proc_sys_root_identity = procfs_directory_identity_for_fd(&self.proc_sys_root)?;
        require_proc_sys_root(proc_sys_root_identity.file)?;
        require_procfs(&self.proc_sys_root)?;
        if !same_procfs_directory_identity(proc_sys_root_identity, self.proc_sys_root_identity) {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "retained procfs sys root identity drifted before boot-id read".to_string(),
            ));
        }
        let kernel = open_child_directory(&self.proc_sys_root, "kernel")?;
        let random = open_child_directory(&kernel, "random")?;
        let mut boot_id = open_readonly_file_beneath(&random, "boot_id", "kernel boot-id leaf")?;
        let identity_before = identity_for_fd(&boot_id)?;
        require_proc_boot_id(identity_before)?;
        let mut bytes = Vec::with_capacity(BOOT_ID_FILE_BYTES);
        Read::by_ref(&mut boot_id)
            .take(BOOT_ID_READ_LIMIT)
            .read_to_end(&mut bytes)
            .map_err(|error| io_error("read fixed procfs boot id", error))?;
        let identity_after = identity_for_fd(&boot_id)?;
        if identity_before != identity_after {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "procfs boot-id identity changed during bounded read".to_string(),
            ));
        }
        derive_linux_boot_id_sha256(&bytes)
    }

    pub(crate) fn revalidate(&self) -> ReplayStoreResultV1<()> {
        let retained = procfs_directory_identity_for_fd(&self.proc_root)?;
        require_proc_root(retained.file)?;
        require_procfs(&self.proc_root)?;
        if !same_procfs_directory_identity(retained, self.proc_root_identity) {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "retained procfs root identity drifted".to_string(),
            ));
        }
        let retained_sys = procfs_directory_identity_for_fd(&self.proc_sys_root)?;
        require_proc_sys_root(retained_sys.file)?;
        require_procfs(&self.proc_sys_root)?;
        if !same_procfs_directory_identity(retained_sys, self.proc_sys_root_identity) {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "retained procfs sys root identity drifted".to_string(),
            ));
        }
        let reopened = open_root_directory(Path::new("/proc"))?;
        let reopened_identity = procfs_directory_identity_for_fd(&reopened)?;
        require_proc_root(reopened_identity.file)?;
        require_procfs(&reopened)?;
        if !same_procfs_directory_identity(reopened_identity, self.proc_root_identity) {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "canonical /proc path no longer names the retained procfs root".to_string(),
            ));
        }
        let reopened_sys = open_fixed_child_directory_across_mount(&reopened, "sys")?;
        let reopened_sys_identity = procfs_directory_identity_for_fd(&reopened_sys)?;
        require_proc_sys_root(reopened_sys_identity.file)?;
        require_procfs(&reopened_sys)?;
        if !same_procfs_directory_identity(reopened_sys_identity, self.proc_sys_root_identity) {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "canonical /proc/sys path no longer names the retained procfs sys root".to_string(),
            ));
        }
        Ok(())
    }
}

fn procfs_directory_identity_for_fd<Fd: AsFd>(
    descriptor: Fd,
) -> ReplayStoreResultV1<ProcfsDirectoryIdentityV1> {
    let descriptor = descriptor.as_fd();
    let file = identity_for_fd(descriptor)?;
    let statx = rustix::fs::statx(
        descriptor,
        "",
        rustix::fs::AtFlags::EMPTY_PATH | rustix::fs::AtFlags::NO_AUTOMOUNT,
        rustix::fs::StatxFlags::MNT_ID,
    )
    .map_err(|error| syscall_error("statx fixed procfs mount identity", error))?;
    if !rustix::fs::StatxFlags::from_bits_retain(statx.stx_mask)
        .contains(rustix::fs::StatxFlags::MNT_ID)
        || statx.stx_mnt_id == 0
    {
        return Err(ReplayStoreErrorV1::IdentityMismatch(
            "fixed procfs mount identity is unavailable".to_string(),
        ));
    }
    Ok(ProcfsDirectoryIdentityV1 {
        file,
        mount_id: statx.stx_mnt_id,
    })
}

fn same_procfs_directory_identity(
    left: ProcfsDirectoryIdentityV1,
    right: ProcfsDirectoryIdentityV1,
) -> bool {
    left.file.device == right.file.device
        && left.file.inode == right.file.inode
        && left.file.uid == right.file.uid
        && left.file.gid == right.file.gid
        && left.file.mode == right.file.mode
        && left.file.file_type == right.file.file_type
        && left.mount_id == right.mount_id
}

pub fn derive_linux_boot_id_sha256(bytes: &[u8]) -> ReplayStoreResultV1<String> {
    let canonical = parse_boot_id(bytes)?;
    let mut frame = Vec::with_capacity(BOOT_ID_SHA256_DOMAIN.len() + 8 + canonical.len());
    frame.extend_from_slice(BOOT_ID_SHA256_DOMAIN);
    frame.extend_from_slice(&(canonical.len() as u64).to_be_bytes());
    frame.extend_from_slice(canonical);
    Ok(format!("{:x}", sha2::Sha256::digest(&frame)))
}

fn parse_boot_id(bytes: &[u8]) -> ReplayStoreResultV1<&[u8]> {
    if bytes.len() != BOOT_ID_FILE_BYTES || bytes[BOOT_ID_BYTES] != b'\n' {
        return Err(invalid(
            "kernel boot id must be exactly one canonical lowercase UUID plus LF",
        ));
    }
    let canonical = &bytes[..BOOT_ID_BYTES];
    for (index, byte) in canonical.iter().copied().enumerate() {
        let separator = matches!(index, 8 | 13 | 18 | 23);
        let valid = if separator {
            byte == b'-'
        } else {
            byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')
        };
        if !valid {
            return Err(invalid("kernel boot id is not a canonical lowercase UUID"));
        }
    }
    if canonical
        .iter()
        .copied()
        .filter(|byte| *byte != b'-')
        .all(|byte| byte == b'0')
    {
        return Err(invalid("kernel boot id must not be the nil UUID"));
    }
    Ok(canonical)
}

fn require_proc_root(identity: FileIdentityV1) -> ReplayStoreResultV1<()> {
    if identity.file_type != FileType::Directory
        || identity.mode != PROC_ROOT_MODE
        || identity.uid != 0
        || identity.gid != 0
        || identity.link_count < 2
    {
        return Err(ReplayStoreErrorV1::IdentityMismatch(
            "fixed /proc root is not the exact root-owned 0555 directory".to_string(),
        ));
    }
    Ok(())
}

fn require_proc_sys_root(identity: FileIdentityV1) -> ReplayStoreResultV1<()> {
    if identity.file_type != FileType::Directory
        || identity.mode != PROC_ROOT_MODE
        || identity.uid != 0
        || identity.gid != 0
        || identity.link_count == 0
    {
        return Err(ReplayStoreErrorV1::IdentityMismatch(
            "fixed /proc/sys root is not the exact root-owned 0555 directory on retained procfs"
                .to_string(),
        ));
    }
    Ok(())
}

fn require_proc_boot_id(identity: FileIdentityV1) -> ReplayStoreResultV1<()> {
    if identity.file_type != FileType::RegularFile
        || identity.mode != PROC_BOOT_ID_MODE
        || identity.uid != 0
        || identity.gid != 0
        || identity.link_count != 1
    {
        return Err(ReplayStoreErrorV1::IdentityMismatch(
            "fixed procfs boot-id is not the exact root-owned 0444 pseudo-file".to_string(),
        ));
    }
    Ok(())
}

fn require_procfs(descriptor: &OwnedFd) -> ReplayStoreResultV1<()> {
    let filesystem = rustix::fs::fstatfs(descriptor)
        .map_err(|error| syscall_error("fstatfs fixed procfs root", error))?;
    if filesystem.f_type != rustix::fs::PROC_SUPER_MAGIC {
        return Err(ReplayStoreErrorV1::IdentityMismatch(
            "fixed /proc root is not procfs".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_id_parser_and_digest_are_exact() {
        let canonical = b"550e8400-e29b-41d4-a716-446655440000\n";
        let digest = derive_linux_boot_id_sha256(canonical).expect("canonical boot id");
        assert_eq!(digest.len(), 64);
        assert_eq!(
            digest,
            derive_linux_boot_id_sha256(canonical).expect("stable digest")
        );
        for invalid_bytes in [
            b"550E8400-e29b-41d4-a716-446655440000\n".as_slice(),
            b"550e8400e29b41d4a716446655440000\n".as_slice(),
            b"00000000-0000-0000-0000-000000000000\n".as_slice(),
            b"550e8400-e29b-41d4-a716-446655440000".as_slice(),
            b"550e8400-e29b-41d4-a716-446655440000\r\n".as_slice(),
        ] {
            assert!(derive_linux_boot_id_sha256(invalid_bytes).is_err());
        }
    }

    #[test]
    fn fixed_procfs_observation_is_stable() {
        let source = FixedProcfsBootIdSourceV1::open().expect("open fixed procfs");
        assert_ne!(source.proc_root_identity.mount_id, 0);
        assert_ne!(source.proc_sys_root_identity.mount_id, 0);

        let mut positive_nlink_drift = source.proc_root_identity;
        positive_nlink_drift.file.link_count = positive_nlink_drift
            .file
            .link_count
            .checked_add(1)
            .expect("procfs nlink test increment");
        assert!(same_procfs_directory_identity(
            positive_nlink_drift,
            source.proc_root_identity,
        ));

        let mut mount_drift = source.proc_root_identity;
        mount_drift.mount_id = mount_drift
            .mount_id
            .checked_add(1)
            .expect("procfs mount-id test increment");
        assert!(!same_procfs_directory_identity(
            mount_drift,
            source.proc_root_identity,
        ));
        for mutate in [
            |identity: &mut ProcfsDirectoryIdentityV1| identity.file.device ^= 1,
            |identity: &mut ProcfsDirectoryIdentityV1| identity.file.inode ^= 1,
            |identity: &mut ProcfsDirectoryIdentityV1| identity.file.mode ^= 0o100,
        ] {
            let mut drift = source.proc_root_identity;
            mutate(&mut drift);
            assert!(!same_procfs_directory_identity(
                drift,
                source.proc_root_identity,
            ));
        }
        let mut zero_nlink = source.proc_root_identity.file;
        zero_nlink.link_count = 0;
        assert!(require_proc_root(zero_nlink).is_err());

        let before = source.observe_sha256().expect("first boot observation");
        let after = source.observe_sha256().expect("second boot observation");
        assert_eq!(before, after);
        source.revalidate().expect("revalidate fixed procfs");
    }
}
