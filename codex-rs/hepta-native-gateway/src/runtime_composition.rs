use std::env;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use hepta_memory::DurableIntegrityKey;
use hepta_runtime::RuntimeKernel;
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

const OUTCOME_DATABASE_ENV: &str = "HEPTA_RUNTIME_OUTCOME_DATABASE";
const INTEGRITY_KEY_FILE_ENV: &str = "HEPTA_RUNTIME_INTEGRITY_KEY_FILE";
const OUTCOME_MODE_ENV: &str = "HEPTA_RUNTIME_OUTCOME_MODE";
const OPEN_EXISTING_MODE: &str = "open-existing";
const BOOTSTRAP_NEW_MODE: &str = "bootstrap-new";
#[cfg(unix)]
const PRIVATE_FILE_MODE: u32 = 0o600;
const ENCODED_KEY_BYTES: usize = 64;
const ENCODED_KEY_WITH_NEWLINE_BYTES: usize = ENCODED_KEY_BYTES + 1;

pub struct NativeGatewayRuntime {
    kernel: RuntimeKernel,
    outcome_mode: RuntimeOutcomeMode,
}

impl fmt::Debug for NativeGatewayRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeGatewayRuntime")
            .field("outcome_mode", &self.outcome_mode.as_str())
            .field("integrity_key", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuntimeOutcomeMode {
    OpenExisting,
    BootstrapNew,
}

impl RuntimeOutcomeMode {
    fn parse(value: &str) -> Result<Self> {
        match value {
            OPEN_EXISTING_MODE => Ok(Self::OpenExisting),
            BOOTSTRAP_NEW_MODE => Ok(Self::BootstrapNew),
            _ => anyhow::bail!(
                "{OUTCOME_MODE_ENV} must be {OPEN_EXISTING_MODE} or {BOOTSTRAP_NEW_MODE}"
            ),
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::OpenExisting => OPEN_EXISTING_MODE,
            Self::BootstrapNew => BOOTSTRAP_NEW_MODE,
        }
    }
}

struct RuntimeCompositionConfig {
    outcome_database: PathBuf,
    integrity_key_file: PathBuf,
    outcome_mode: RuntimeOutcomeMode,
}

impl RuntimeCompositionConfig {
    fn from_env() -> Result<Self> {
        let outcome_database = required_absolute_path(OUTCOME_DATABASE_ENV)?;
        let integrity_key_file = required_absolute_path(INTEGRITY_KEY_FILE_ENV)?;
        let outcome_mode = env::var(OUTCOME_MODE_ENV)
            .ok()
            .map(|value| RuntimeOutcomeMode::parse(value.trim()))
            .transpose()?
            .unwrap_or(RuntimeOutcomeMode::OpenExisting);
        Ok(Self {
            outcome_database,
            integrity_key_file,
            outcome_mode,
        })
    }
}

impl NativeGatewayRuntime {
    pub fn from_env() -> Result<Self> {
        Self::open(RuntimeCompositionConfig::from_env()?)
    }

    fn open(config: RuntimeCompositionConfig) -> Result<Self> {
        let integrity_key = read_integrity_key(&config.integrity_key_file)?;
        let kernel = match config.outcome_mode {
            RuntimeOutcomeMode::OpenExisting => {
                RuntimeKernel::open_with_durable_outcomes(&config.outcome_database, integrity_key)
            }
            RuntimeOutcomeMode::BootstrapNew => RuntimeKernel::bootstrap_with_durable_outcomes(
                &config.outcome_database,
                integrity_key,
            ),
        }
        .with_context(|| {
            format!(
                "initialize keyed RuntimeKernel with {} durable outcomes",
                config.outcome_mode.as_str()
            )
        })?;
        Ok(Self {
            kernel,
            outcome_mode: config.outcome_mode,
        })
    }

    pub(crate) fn validate_readiness(&self) -> Result<()> {
        self.kernel
            .model_selection()
            .map(|_| ())
            .map_err(|error| anyhow::anyhow!("attached RuntimeKernel readiness failed: {error}"))
    }

    pub(crate) const fn outcome_mode(&self) -> &'static str {
        self.outcome_mode.as_str()
    }
}

fn required_absolute_path(env_name: &str) -> Result<PathBuf> {
    let path = env::var_os(env_name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .with_context(|| format!("{env_name} is required for --serve-ui"))?;
    if !path.is_absolute() {
        anyhow::bail!("{env_name} must be an absolute path");
    }
    Ok(path)
}

#[cfg(unix)]
fn read_integrity_key(path: &Path) -> Result<DurableIntegrityKey> {
    if !path.is_absolute() {
        anyhow::bail!("{INTEGRITY_KEY_FILE_ENV} must be an absolute path");
    }
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .context("open RuntimeKernel integrity key file")?;
    validate_private_key_file(&file)?;
    let mut encoded = Zeroizing::new(Vec::with_capacity(ENCODED_KEY_WITH_NEWLINE_BYTES));
    file.by_ref()
        .take((ENCODED_KEY_WITH_NEWLINE_BYTES + 1) as u64)
        .read_to_end(&mut encoded)
        .context("read RuntimeKernel integrity key file")?;
    decode_integrity_key(&encoded)
}

#[cfg(not(unix))]
fn read_integrity_key(_path: &Path) -> Result<DurableIntegrityKey> {
    anyhow::bail!("keyed RuntimeKernel composition requires Unix secure-file semantics")
}

#[cfg(unix)]
fn validate_private_key_file(file: &File) -> Result<()> {
    let metadata = file
        .metadata()
        .context("inspect opened RuntimeKernel integrity key file")?;
    if !metadata.file_type().is_file() {
        anyhow::bail!("RuntimeKernel integrity key must be a regular file");
    }
    let effective_uid = effective_uid();
    if metadata.uid() != effective_uid {
        anyhow::bail!("RuntimeKernel integrity key must be owned by the effective user");
    }
    if metadata.nlink() != 1 {
        anyhow::bail!("RuntimeKernel integrity key must have exactly one hard link");
    }
    let mode = metadata.mode() & 0o7777;
    if mode != PRIVATE_FILE_MODE {
        anyhow::bail!("RuntimeKernel integrity key must have mode 0o600");
    }
    if !matches!(
        metadata.len() as usize,
        ENCODED_KEY_BYTES | ENCODED_KEY_WITH_NEWLINE_BYTES
    ) {
        anyhow::bail!("RuntimeKernel integrity key must contain 64 lowercase hex bytes");
    }
    Ok(())
}

fn decode_integrity_key(encoded: &[u8]) -> Result<DurableIntegrityKey> {
    let encoded = match encoded {
        value if value.len() == ENCODED_KEY_BYTES => value,
        value
            if value.len() == ENCODED_KEY_WITH_NEWLINE_BYTES
                && value[ENCODED_KEY_BYTES] == b'\n' =>
        {
            &value[..ENCODED_KEY_BYTES]
        }
        _ => anyhow::bail!("RuntimeKernel integrity key must be canonical lowercase hex"),
    };
    let mut bytes = [0_u8; 32];
    for (index, pair) in encoded.chunks_exact(2).enumerate() {
        bytes[index] = (decode_hex_nibble(pair[0])? << 4) | decode_hex_nibble(pair[1])?;
    }
    Ok(DurableIntegrityKey::from_bytes(bytes))
}

fn decode_hex_nibble(value: u8) -> Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => anyhow::bail!("RuntimeKernel integrity key must be canonical lowercase hex"),
    }
}

#[cfg(unix)]
fn effective_uid() -> u32 {
    // SAFETY: `geteuid` has no preconditions and reads process credentials.
    unsafe { libc::geteuid() }
}

#[cfg(all(test, unix))]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use tempfile::tempdir;

    use super::*;

    fn write_key(path: &Path, encoded: &[u8]) {
        fs::write(path, encoded).expect("write key");
        fs::set_permissions(path, fs::Permissions::from_mode(PRIVATE_FILE_MODE))
            .expect("set key permissions");
    }

    #[test]
    fn keyed_runtime_bootstraps_then_opens_existing_database() {
        let root = tempdir().expect("tempdir");
        fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))
            .expect("set root permissions");
        let key_path = root.path().join("runtime.key");
        let database_path = root.path().join("outcomes.sqlite3");
        write_key(
            &key_path,
            b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\n",
        );

        let bootstrap = NativeGatewayRuntime::open(RuntimeCompositionConfig {
            outcome_database: database_path.clone(),
            integrity_key_file: key_path.clone(),
            outcome_mode: RuntimeOutcomeMode::BootstrapNew,
        })
        .expect("bootstrap keyed runtime");
        bootstrap.validate_readiness().expect("bootstrap readiness");
        drop(bootstrap);

        let opened = NativeGatewayRuntime::open(RuntimeCompositionConfig {
            outcome_database: database_path,
            integrity_key_file: key_path,
            outcome_mode: RuntimeOutcomeMode::OpenExisting,
        })
        .expect("open keyed runtime");
        assert_eq!(opened.outcome_mode(), OPEN_EXISTING_MODE);
        opened.validate_readiness().expect("open readiness");
    }

    #[test]
    fn keyed_runtime_rejects_non_private_key_file() {
        let root = tempdir().expect("tempdir");
        let key_path = root.path().join("runtime.key");
        write_key(
            &key_path,
            b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        );
        fs::set_permissions(&key_path, fs::Permissions::from_mode(0o644))
            .expect("relax key permissions");

        let error = read_integrity_key(&key_path).expect_err("unsafe key must fail");
        assert!(
            error
                .to_string()
                .contains("integrity key must have mode 0o600")
        );
    }

    #[test]
    fn keyed_runtime_rejects_noncanonical_key_encoding() {
        let error = decode_integrity_key(
            b"000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F",
        )
        .expect_err("uppercase key must fail");
        assert!(error.to_string().contains("canonical lowercase hex"));
    }
}
