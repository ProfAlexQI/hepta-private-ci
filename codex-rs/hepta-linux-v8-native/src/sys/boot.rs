#[cfg(any(target_os = "linux", test))]
use super::NativeSysErrorV8;
use super::NativeSysResultV8;

#[cfg(target_os = "linux")]
use super::ProcfsRootV8;
#[cfg(not(target_os = "linux"))]
use super::unsupported;

const BOOT_ID_CANONICAL_BYTES_V8: usize = 36;

/// Canonical lowercase UUID read from the fixed procfs boot-id source.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BootIdV8 {
    bytes: [u8; BOOT_ID_CANONICAL_BYTES_V8],
}

impl BootIdV8 {
    pub fn as_str(&self) -> &str {
        // SAFETY: construction accepts only lowercase ASCII UUID bytes.
        unsafe { std::str::from_utf8_unchecked(&self.bytes) }
    }
}

impl std::fmt::Display for BootIdV8 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Reads `/proc/sys/kernel/random/boot_id` twice through the exact procfs
/// anchor and returns only a byte-stable canonical UUID observation.
pub fn observe_boot_id_v8() -> NativeSysResultV8<BootIdV8> {
    observe_boot_id_impl_v8()
}

#[cfg(target_os = "linux")]
fn observe_boot_id_impl_v8() -> NativeSysResultV8<BootIdV8> {
    let procfs = ProcfsRootV8::open_fixed()?;
    let before = procfs.read_regular_beneath("sys/kernel/random/boot_id", 37)?;
    let before = parse_boot_id_v8(&before)?;
    let after = procfs.read_regular_beneath("sys/kernel/random/boot_id", 37)?;
    let after = parse_boot_id_v8(&after)?;
    procfs.revalidate()?;
    if before != after {
        return Err(NativeSysErrorV8::RaceDetected(
            "kernel boot id changed during anchored observation".to_string(),
        ));
    }
    Ok(after)
}

#[cfg(not(target_os = "linux"))]
fn observe_boot_id_impl_v8() -> NativeSysResultV8<BootIdV8> {
    Err(unsupported("observe fixed procfs boot id"))
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_boot_id_v8(source: &[u8]) -> NativeSysResultV8<BootIdV8> {
    let canonical = match source {
        bytes if bytes.len() == BOOT_ID_CANONICAL_BYTES_V8 => bytes,
        bytes
            if bytes.len() == BOOT_ID_CANONICAL_BYTES_V8 + 1
                && bytes[BOOT_ID_CANONICAL_BYTES_V8] == b'\n' =>
        {
            &bytes[..BOOT_ID_CANONICAL_BYTES_V8]
        }
        _ => {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "boot id must be exactly one canonical UUID plus at most one LF".to_string(),
            ));
        }
    };
    for (index, byte) in canonical.iter().copied().enumerate() {
        let is_separator = matches!(index, 8 | 13 | 18 | 23);
        let is_lower_hex = byte.is_ascii_digit() || matches!(byte, b'a'..=b'f');
        let is_valid = if is_separator {
            byte == b'-'
        } else {
            is_lower_hex
        };
        if !is_valid {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "boot id is not a canonical lowercase UUID".to_string(),
            ));
        }
    }
    if canonical
        .iter()
        .copied()
        .filter(|byte| *byte != b'-')
        .all(|byte| byte == b'0')
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "boot id must not be the nil UUID".to_string(),
        ));
    }
    let mut bytes = [0_u8; BOOT_ID_CANONICAL_BYTES_V8];
    bytes.copy_from_slice(canonical);
    Ok(BootIdV8 { bytes })
}
