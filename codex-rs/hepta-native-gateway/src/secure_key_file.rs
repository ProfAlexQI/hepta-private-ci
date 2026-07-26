use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use zeroize::Zeroizing;

#[cfg(unix)]
use std::fs::File;
#[cfg(unix)]
use std::fs::OpenOptions;
#[cfg(unix)]
use std::io::Read;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

pub(crate) const PRIVATE_FILE_MODE: u32 = 0o600;
const ENCODED_KEY_BYTES: usize = 64;
const ENCODED_KEY_WITH_NEWLINE_BYTES: usize = ENCODED_KEY_BYTES + 1;

#[cfg(unix)]
pub(crate) fn read_private_key(
    path: &Path,
    environment_name: &str,
    purpose: &str,
) -> Result<Zeroizing<[u8; 32]>> {
    if !path.is_absolute() {
        anyhow::bail!("{environment_name} must be an absolute path");
    }
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .with_context(|| format!("open {purpose} key file"))?;
    validate_private_key_file(&file, purpose)?;
    let mut encoded = Zeroizing::new(Vec::with_capacity(ENCODED_KEY_WITH_NEWLINE_BYTES));
    file.by_ref()
        .take((ENCODED_KEY_WITH_NEWLINE_BYTES + 1) as u64)
        .read_to_end(&mut encoded)
        .with_context(|| format!("read {purpose} key file"))?;
    decode_private_key(&encoded, purpose)
}

#[cfg(not(unix))]
pub(crate) fn read_private_key(
    _path: &Path,
    _environment_name: &str,
    purpose: &str,
) -> Result<Zeroizing<[u8; 32]>> {
    anyhow::bail!("{purpose} requires Unix secure-file semantics")
}

#[cfg(unix)]
fn validate_private_key_file(file: &File, purpose: &str) -> Result<()> {
    let metadata = file
        .metadata()
        .with_context(|| format!("inspect opened {purpose} key file"))?;
    if !metadata.file_type().is_file() {
        anyhow::bail!("{purpose} key must be a regular file");
    }
    if metadata.uid() != effective_uid() {
        anyhow::bail!("{purpose} key must be owned by the effective user");
    }
    if metadata.nlink() != 1 {
        anyhow::bail!("{purpose} key must have exactly one hard link");
    }
    let mode = metadata.mode() & 0o7777;
    if mode != PRIVATE_FILE_MODE {
        anyhow::bail!("{purpose} key must have mode 0o600");
    }
    if !matches!(
        metadata.len() as usize,
        ENCODED_KEY_BYTES | ENCODED_KEY_WITH_NEWLINE_BYTES
    ) {
        anyhow::bail!("{purpose} key must contain 64 lowercase hex bytes");
    }
    Ok(())
}

fn decode_private_key(encoded: &[u8], purpose: &str) -> Result<Zeroizing<[u8; 32]>> {
    let encoded = match encoded {
        value if value.len() == ENCODED_KEY_BYTES => value,
        value
            if value.len() == ENCODED_KEY_WITH_NEWLINE_BYTES
                && value[ENCODED_KEY_BYTES] == b'\n' =>
        {
            &value[..ENCODED_KEY_BYTES]
        }
        _ => anyhow::bail!("{purpose} key must be canonical lowercase hex"),
    };
    let mut bytes = Zeroizing::new([0_u8; 32]);
    for (index, pair) in encoded.chunks_exact(2).enumerate() {
        bytes[index] =
            (decode_hex_nibble(pair[0], purpose)? << 4) | decode_hex_nibble(pair[1], purpose)?;
    }
    Ok(bytes)
}

fn decode_hex_nibble(value: u8, purpose: &str) -> Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => anyhow::bail!("{purpose} key must be canonical lowercase hex"),
    }
}

#[cfg(unix)]
fn effective_uid() -> u32 {
    // SAFETY: `geteuid` has no preconditions and reads process credentials.
    unsafe { libc::geteuid() }
}

#[cfg(all(test, unix))]
#[path = "../tests/unit/secure_key_file.rs"]
mod tests;
