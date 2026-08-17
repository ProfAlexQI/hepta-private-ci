use std::io::Read;
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
use crate::secure_fs::open_readonly_file_beneath;
use crate::secure_fs::open_root_directory;
use crate::secure_fs::same_directory_identity;

const BOOT_ID_BYTES: usize = 36;
const BOOT_ID_FILE_BYTES: usize = BOOT_ID_BYTES + 1;
const BOOT_ID_READ_LIMIT: u64 = (BOOT_ID_FILE_BYTES + 1) as u64;
const BOOT_ID_SHA256_DOMAIN: &[u8] = b"hepta.mnl.linux-boot-id.v1\0";
const PROC_ROOT_MODE: u32 = 0o555;
const PROC_BOOT_ID_MODE: u32 = 0o444;

#[derive(Debug)]
pub(crate) struct FixedProcfsBootIdSourceV1 {
    proc_root: OwnedFd,
    proc_root_identity: FileIdentityV1,
}

impl FixedProcfsBootIdSourceV1 {
    pub(crate) fn open() -> ReplayStoreResultV1<Self> {
        let proc_root = open_root_directory(Path::new("/proc"))?;
        let proc_root_identity = identity_for_fd(&proc_root)?;
        require_proc_root(proc_root_identity)?;
        require_procfs(&proc_root)?;
        Ok(Self {
            proc_root,
            proc_root_identity,
        })
    }

    pub(crate) fn observe_sha256(&self) -> ReplayStoreResultV1<String> {
        require_proc_root(identity_for_fd(&self.proc_root)?)?;
        require_procfs(&self.proc_root)?;
        let sys = open_child_directory(&self.proc_root, "sys")?;
        let kernel = open_child_directory(&sys, "kernel")?;
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
        let retained = identity_for_fd(&self.proc_root)?;
        require_proc_root(retained)?;
        require_procfs(&self.proc_root)?;
        if !same_directory_identity(retained, self.proc_root_identity) {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "retained procfs root identity drifted".to_string(),
            ));
        }
        let reopened = open_root_directory(Path::new("/proc"))?;
        let reopened_identity = identity_for_fd(&reopened)?;
        require_proc_root(reopened_identity)?;
        require_procfs(&reopened)?;
        if !same_directory_identity(reopened_identity, self.proc_root_identity) {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "canonical /proc path no longer names the retained procfs root".to_string(),
            ));
        }
        Ok(())
    }
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
        let before = source.observe_sha256().expect("first boot observation");
        let after = source.observe_sha256().expect("second boot observation");
        assert_eq!(before, after);
        source.revalidate().expect("revalidate fixed procfs");
    }
}
