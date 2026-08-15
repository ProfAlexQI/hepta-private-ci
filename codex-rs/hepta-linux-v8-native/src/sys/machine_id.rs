#[cfg(target_os = "linux")]
use sha2::Digest as _;

use super::DirectoryAnchorV8;
use super::FileIdentityV8;
#[cfg(any(target_os = "linux", test))]
use super::NativeSysErrorV8;
use super::NativeSysResultV8;
use super::VerifiedFileFdV8;

#[cfg(not(target_os = "linux"))]
use super::unsupported;
#[cfg(target_os = "linux")]
use std::path::Path;

#[cfg(any(target_os = "linux", test))]
const MACHINE_ID_CANONICAL_BYTES_V8: usize = 32;
#[cfg(target_os = "linux")]
const MACHINE_ID_MAX_SOURCE_BYTES_V8: u64 = 33;

/// Opaque observation of the fixed Linux machine-id source.
///
/// This token is intentionally neither cloneable nor deserializable. Its
/// digest is SHA-256 over the canonical 32 lowercase hexadecimal bytes; a
/// single trailing LF in the source file is accepted but excluded from the
/// digest. The retained identity is metadata from the opened descriptor.
/// This observation is not target-host authorization or install authority.
#[derive(Debug)]
pub struct ObservedMachineIdV8 {
    etc_anchor: DirectoryAnchorV8,
    machine_id_sha256: String,
    source: VerifiedFileFdV8,
    source_bytes: Vec<u8>,
    source_identity: FileIdentityV8,
}

impl ObservedMachineIdV8 {
    pub fn machine_id_sha256(&self) -> &str {
        &self.machine_id_sha256
    }

    pub fn source_identity(&self) -> FileIdentityV8 {
        self.source_identity
    }

    /// Replays the fixed pathname and exact bytes while retaining the
    /// original `/etc` and `machine-id` descriptors. This remains identity
    /// evidence only and cannot be converted into host or execution authority.
    pub(crate) fn revalidate_descriptor_bound_v8(&self) -> NativeSysResultV8<()> {
        revalidate_machine_id_descriptor_bound_impl_v8(self)
    }
}

/// Observes `/etc/machine-id` through the mandatory anchored openat2 policy.
///
/// Linux accepts only an exact root-owned mode-0755 `/etc` anchor and a
/// root-owned, single-link, exact mode-0444 regular `machine-id` file.
pub fn observe_machine_id_v8() -> NativeSysResultV8<ObservedMachineIdV8> {
    observe_machine_id_impl_v8()
}

#[cfg(target_os = "linux")]
fn observe_machine_id_impl_v8() -> NativeSysResultV8<ObservedMachineIdV8> {
    let etc = DirectoryAnchorV8::open(Path::new("/etc"))?;
    require_root_owned_etc_anchor_v8(etc.identity())?;

    let source = etc.open_regular_readonly_beneath(Path::new("machine-id"))?;
    let source_identity = source.identity();
    validate_machine_id_source_identity_v8(source_identity)?;
    let source_bytes = source.read_all(MACHINE_ID_MAX_SOURCE_BYTES_V8)?;
    let canonical = parse_machine_id_bytes_v8(&source_bytes)?;

    // Re-resolve both the fixed anchor and leaf after the read. A rename or
    // pathname substitution must not leave a valid-looking observation of an
    // inode that `/etc/machine-id` no longer names.
    let reopened_etc = DirectoryAnchorV8::open(Path::new("/etc"))?;
    require_root_owned_etc_anchor_v8(reopened_etc.identity())?;
    if !reopened_etc
        .identity()
        .matches_stable_directory(etc.identity())
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "fixed /etc anchor identity changed during machine-id observation".to_string(),
        ));
    }
    let reopened = reopened_etc.open_regular_readonly_beneath(Path::new("machine-id"))?;
    validate_machine_id_source_identity_v8(reopened.identity())?;
    if reopened.identity() != source_identity {
        return Err(NativeSysErrorV8::RaceDetected(
            "fixed /etc/machine-id identity changed during observation".to_string(),
        ));
    }
    if reopened.read_all(MACHINE_ID_MAX_SOURCE_BYTES_V8)? != source_bytes {
        return Err(NativeSysErrorV8::RaceDetected(
            "fixed /etc/machine-id bytes changed during observation".to_string(),
        ));
    }

    let observed = ObservedMachineIdV8 {
        etc_anchor: etc,
        machine_id_sha256: format!("{:x}", sha2::Sha256::digest(canonical)),
        source,
        source_bytes,
        source_identity,
    };
    observed.revalidate_descriptor_bound_v8()?;
    Ok(observed)
}

#[cfg(not(target_os = "linux"))]
fn observe_machine_id_impl_v8() -> NativeSysResultV8<ObservedMachineIdV8> {
    Err(unsupported("observe fixed /etc/machine-id"))
}

#[cfg(target_os = "linux")]
fn revalidate_machine_id_descriptor_bound_impl_v8(
    observed: &ObservedMachineIdV8,
) -> NativeSysResultV8<()> {
    observed.etc_anchor.revalidate_identity()?;
    require_root_owned_etc_anchor_v8(observed.etc_anchor.identity())?;
    observed.source.revalidate_identity()?;
    validate_machine_id_source_identity_v8(observed.source.identity())?;
    if observed.source.identity() != observed.source_identity {
        return Err(NativeSysErrorV8::RaceDetected(
            "retained /etc/machine-id descriptor identity drifted".to_string(),
        ));
    }
    let retained_before = observed.source.read_all(MACHINE_ID_MAX_SOURCE_BYTES_V8)?;
    let canonical = parse_machine_id_bytes_v8(&retained_before)?;
    if retained_before != observed.source_bytes
        || format!("{:x}", sha2::Sha256::digest(canonical)) != observed.machine_id_sha256
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "retained /etc/machine-id bytes differ from the pinned observation".to_string(),
        ));
    }

    let named_etc = DirectoryAnchorV8::open(Path::new("/etc"))?;
    require_root_owned_etc_anchor_v8(named_etc.identity())?;
    if !named_etc
        .identity()
        .matches_stable_directory(observed.etc_anchor.identity())
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "fixed /etc pathname no longer names the retained anchor".to_string(),
        ));
    }
    let named_source = named_etc.open_regular_readonly_beneath(Path::new("machine-id"))?;
    validate_machine_id_source_identity_v8(named_source.identity())?;
    if named_source.identity() != observed.source_identity
        || named_source.read_all(MACHINE_ID_MAX_SOURCE_BYTES_V8)? != retained_before
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "fixed /etc/machine-id pathname differs from the retained descriptor".to_string(),
        ));
    }

    let retained_after = observed.source.read_all(MACHINE_ID_MAX_SOURCE_BYTES_V8)?;
    observed.source.revalidate_identity()?;
    observed.etc_anchor.revalidate_identity()?;
    if retained_after != retained_before {
        return Err(NativeSysErrorV8::RaceDetected(
            "retained /etc/machine-id bytes changed during descriptor replay".to_string(),
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn revalidate_machine_id_descriptor_bound_impl_v8(
    _observed: &ObservedMachineIdV8,
) -> NativeSysResultV8<()> {
    Err(unsupported(
        "revalidate retained /etc/machine-id descriptor",
    ))
}

#[cfg(target_os = "linux")]
fn require_root_owned_etc_anchor_v8(identity: FileIdentityV8) -> NativeSysResultV8<()> {
    if identity.owner_uid() != 0 || identity.owner_gid() != 0 || identity.mode() != 0o755 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "fixed /etc anchor must be root-owned mode 0755".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn validate_machine_id_source_identity_v8(identity: FileIdentityV8) -> NativeSysResultV8<()> {
    if identity.owner_uid() != 0
        || identity.owner_gid() != 0
        || identity.link_count() != 1
        || !machine_id_mode_is_frozen_read_only_v8(identity.mode())
    {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "fixed /etc/machine-id must be root-owned, single-link, and mode 0444; observed uid={}:gid={} mode={:04o} nlink={}",
            identity.owner_uid(),
            identity.owner_gid(),
            identity.mode(),
            identity.link_count()
        )));
    }
    Ok(())
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn machine_id_mode_is_frozen_read_only_v8(mode: u32) -> bool {
    mode == 0o444
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_machine_id_bytes_v8(
    source: &[u8],
) -> NativeSysResultV8<[u8; MACHINE_ID_CANONICAL_BYTES_V8]> {
    let canonical = match source {
        bytes if bytes.len() == MACHINE_ID_CANONICAL_BYTES_V8 => bytes,
        bytes
            if bytes.len() == MACHINE_ID_CANONICAL_BYTES_V8 + 1
                && bytes[MACHINE_ID_CANONICAL_BYTES_V8] == b'\n' =>
        {
            &bytes[..MACHINE_ID_CANONICAL_BYTES_V8]
        }
        _ => {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "fixed /etc/machine-id must contain exactly 32 bytes plus at most one trailing LF"
                    .to_string(),
            ));
        }
    };
    if canonical.iter().all(|byte| *byte == b'0')
        || !canonical
            .iter()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "fixed /etc/machine-id must be non-zero lowercase hexadecimal".to_string(),
        ));
    }

    let mut exact = [0_u8; MACHINE_ID_CANONICAL_BYTES_V8];
    exact.copy_from_slice(canonical);
    Ok(exact)
}
