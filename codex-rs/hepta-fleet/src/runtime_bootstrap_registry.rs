use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::RuntimeBootstrapTrustRoot;
use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;
use sha2::Sha256;

use crate::FleetRegistry;
use crate::FleetRegistryError;
use crate::ReleaseId;

pub const RUNTIME_RELEASE_PROVENANCE_SCHEMA_VERSION: u32 = 1;
pub const RUNTIME_BOOTSTRAP_REGISTRY_MAX_BYTES: u64 = 32 * 1024;
const TRUST_ROOT_DIRECTORY: &str = "runtime-bootstrap-trust-v1";
const RELEASE_PROVENANCE_DIRECTORY: &str = "runtime-release-provenance-v1";
const RELEASE_MANIFEST_FILE: &str = "release.json";
static PUBLISH_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeReleaseProvenance {
    pub schema_version: u32,
    pub agent_id: AgentId,
    pub release_id: ReleaseId,
    pub source_commit: String,
    pub source_tree: String,
    pub release_manifest_sha256: Sha256Digest,
    pub agentd_binary_sha256: Sha256Digest,
}

impl RuntimeReleaseProvenance {
    pub fn validate(&self) -> Result<(), FleetRegistryError> {
        if self.schema_version != RUNTIME_RELEASE_PROVENANCE_SCHEMA_VERSION {
            return Err(FleetRegistryError::Corrupt(
                "runtime release provenance schema drifted".to_string(),
            ));
        }
        validate_git_oid(&self.source_commit, "source commit")?;
        validate_git_oid(&self.source_tree, "source tree")?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedRuntimeRelease {
    pub release_id: ReleaseId,
    pub program: PathBuf,
    pub provenance: RuntimeReleaseProvenance,
}

impl FleetRegistry {
    /// Installs one immutable, globally pinned public verification key for
    /// runtime bootstrap documents. The corresponding signing key is never
    /// accepted by this registry.
    pub fn install_runtime_bootstrap_trust_root(
        &self,
        trust_root: &RuntimeBootstrapTrustRoot,
    ) -> Result<(), FleetRegistryError> {
        trust_root
            .validate()
            .map_err(|error| FleetRegistryError::Invalid(error.to_string()))?;
        let directory = self.layout().state_root().join(TRUST_ROOT_DIRECTORY);
        ensure_private_directory(&directory)?;
        let path = directory.join(trust_root_file_name(
            trust_root.signer_key_id(),
            trust_root.signer_epoch(),
        ));
        publish_immutable_json(&path, trust_root)
    }

    pub fn resolve_runtime_bootstrap_trust_root(
        &self,
        signer_key_id: &str,
        signer_epoch: u64,
    ) -> Result<RuntimeBootstrapTrustRoot, FleetRegistryError> {
        if signer_epoch == 0 {
            return Err(FleetRegistryError::Invalid(
                "runtime bootstrap signer epoch must be non-zero".to_string(),
            ));
        }
        let directory = self.layout().state_root().join(TRUST_ROOT_DIRECTORY);
        validate_private_directory(&directory)?;
        let path = directory.join(trust_root_file_name(signer_key_id, signer_epoch));
        let trust_root: RuntimeBootstrapTrustRoot = read_bounded_json(&path)?;
        trust_root
            .validate()
            .map_err(|error| FleetRegistryError::Corrupt(error.to_string()))?;
        if trust_root.signer_key_id() != signer_key_id
            || trust_root.signer_epoch() != signer_epoch
        {
            return Err(FleetRegistryError::Corrupt(
                "runtime bootstrap trust-root selector mismatch".to_string(),
            ));
        }
        Ok(trust_root)
    }

    /// Binds an allowed immutable release to the exact source commit/tree,
    /// fleet release manifest and Agentd binary bytes used for bootstrap.
    pub fn install_runtime_release_provenance(
        &self,
        agent_id: &AgentId,
        release_id: &ReleaseId,
        source_commit: impl Into<String>,
        source_tree: impl Into<String>,
    ) -> Result<RuntimeReleaseProvenance, FleetRegistryError> {
        let source_commit = source_commit.into();
        let source_tree = source_tree.into();
        validate_git_oid(&source_commit, "source commit")?;
        validate_git_oid(&source_tree, "source tree")?;
        let release = self.resolve_release(agent_id, release_id)?;
        let manifest = self
            .layout()
            .releases_root()
            .join(release_id.as_str())
            .join(RELEASE_MANIFEST_FILE);
        let provenance = RuntimeReleaseProvenance {
            schema_version: RUNTIME_RELEASE_PROVENANCE_SCHEMA_VERSION,
            agent_id: agent_id.clone(),
            release_id: release_id.clone(),
            source_commit,
            source_tree,
            release_manifest_sha256: sha256_file(&manifest)?,
            agentd_binary_sha256: sha256_file(&release.program)?,
        };
        provenance.validate()?;
        let directory = self
            .layout()
            .state_root()
            .join(RELEASE_PROVENANCE_DIRECTORY)
            .join(agent_id.as_str());
        ensure_private_directory(&directory)?;
        let path = directory.join(format!("{}.json", release_id.as_str()));
        publish_immutable_json(&path, &provenance)?;
        Ok(provenance)
    }

    pub fn resolve_runtime_release_provenance(
        &self,
        agent_id: &AgentId,
        release_id: &ReleaseId,
    ) -> Result<RuntimeReleaseProvenance, FleetRegistryError> {
        let path = self
            .layout()
            .state_root()
            .join(RELEASE_PROVENANCE_DIRECTORY)
            .join(agent_id.as_str())
            .join(format!("{}.json", release_id.as_str()));
        let provenance: RuntimeReleaseProvenance = read_bounded_json(&path)?;
        provenance.validate()?;
        if provenance.agent_id != *agent_id || provenance.release_id != *release_id {
            return Err(FleetRegistryError::Corrupt(
                "runtime release provenance selector mismatch".to_string(),
            ));
        }
        let release = self.resolve_release(agent_id, release_id)?;
        let manifest = self
            .layout()
            .releases_root()
            .join(release_id.as_str())
            .join(RELEASE_MANIFEST_FILE);
        if provenance.release_manifest_sha256 != sha256_file(&manifest)?
            || provenance.agentd_binary_sha256 != sha256_file(&release.program)?
        {
            return Err(FleetRegistryError::Corrupt(
                "runtime release bytes drifted from provenance".to_string(),
            ));
        }
        Ok(provenance)
    }

    /// Resolves an executing Agentd path back to exactly one allowed immutable
    /// release and revalidates its source/binary provenance.
    pub fn resolve_runtime_release_for_program(
        &self,
        agent_id: &AgentId,
        program: &Path,
    ) -> Result<ResolvedRuntimeRelease, FleetRegistryError> {
        let actual = program.canonicalize().map_err(|error| {
            FleetRegistryError::Invalid(format!(
                "runtime executable cannot be canonicalized: {error}"
            ))
        })?;
        let mut resolved = None;
        for release_id in self.allowed_releases(agent_id)? {
            let release = self.resolve_release(agent_id, &release_id)?;
            let candidate = release.program.canonicalize()?;
            if candidate != actual {
                continue;
            }
            if resolved.is_some() {
                return Err(FleetRegistryError::Corrupt(
                    "runtime executable resolves to multiple release identities".to_string(),
                ));
            }
            let provenance = self.resolve_runtime_release_provenance(agent_id, &release_id)?;
            resolved = Some(ResolvedRuntimeRelease {
                release_id,
                program: candidate,
                provenance,
            });
        }
        resolved.ok_or_else(|| {
            FleetRegistryError::Invalid(
                "runtime executable is not an allowed provenance-bound release".to_string(),
            )
        })
    }
}

fn trust_root_file_name(signer_key_id: &str, signer_epoch: u64) -> String {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:runtime-bootstrap-trust-selector:v1");
    frame(&mut bytes, signer_key_id.as_bytes());
    frame(&mut bytes, &signer_epoch.to_be_bytes());
    format!("{}-{signer_epoch:020}.json", Sha256Digest::for_bytes(&bytes).as_str())
}

fn validate_git_oid(value: &str, label: &str) -> Result<(), FleetRegistryError> {
    if value.len() != 40
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(FleetRegistryError::Invalid(format!(
            "runtime {label} must be a lowercase 40-hex object id"
        )));
    }
    Ok(())
}

fn publish_immutable_json<T: Serialize>(
    path: &Path,
    value: &T,
) -> Result<(), FleetRegistryError> {
    let parent = path.parent().ok_or_else(|| {
        FleetRegistryError::Invalid("runtime bootstrap registry path has no parent".to_string())
    })?;
    validate_private_directory(parent)?;
    let mut bytes = serde_json::to_vec(value)
        .map_err(|error| FleetRegistryError::Corrupt(error.to_string()))?;
    bytes.push(b'\n');
    if bytes.len() as u64 > RUNTIME_BOOTSTRAP_REGISTRY_MAX_BYTES {
        return Err(FleetRegistryError::Invalid(
            "runtime bootstrap registry object exceeds its bound".to_string(),
        ));
    }
    if path.exists() {
        let actual = read_bounded(path)?;
        if actual == bytes {
            return Ok(());
        }
        return Err(FleetRegistryError::Corrupt(format!(
            "immutable runtime bootstrap registry object changed: {}",
            path.display()
        )));
    }
    let temp = parent.join(format!(
        ".runtime-bootstrap-registry-{}-{}.tmp",
        std::process::id(),
        PUBLISH_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = secure_create_new(&temp)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    match std::fs::hard_link(&temp, path) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            let _ = std::fs::remove_file(&temp);
            return publish_immutable_json(path, value);
        }
        Err(error) => {
            let _ = std::fs::remove_file(&temp);
            return Err(error.into());
        }
    }
    let _ = std::fs::remove_file(temp);
    sync_directory(parent)
}

fn read_bounded_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, FleetRegistryError> {
    let bytes = read_bounded(path)?;
    serde_json::from_slice(&bytes).map_err(|error| {
        FleetRegistryError::Corrupt(format!(
            "invalid runtime bootstrap registry JSON {}: {error}",
            path.display()
        ))
    })
}

fn read_bounded(path: &Path) -> Result<Vec<u8>, FleetRegistryError> {
    let before = secure_metadata(path)?;
    if before.len() > RUNTIME_BOOTSTRAP_REGISTRY_MAX_BYTES {
        return Err(FleetRegistryError::Corrupt(
            "runtime bootstrap registry object exceeds its bound".to_string(),
        ));
    }
    let mut file = secure_open_read(path)?;
    let mut bytes = Vec::with_capacity(before.len() as usize);
    file.take(RUNTIME_BOOTSTRAP_REGISTRY_MAX_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > RUNTIME_BOOTSTRAP_REGISTRY_MAX_BYTES {
        return Err(FleetRegistryError::Corrupt(
            "runtime bootstrap registry object exceeds its bound".to_string(),
        ));
    }
    let after = secure_metadata(path)?;
    if metadata_identity(&before) != metadata_identity(&after) {
        return Err(FleetRegistryError::Corrupt(
            "runtime bootstrap registry object changed while reading".to_string(),
        ));
    }
    Ok(bytes)
}

fn ensure_private_directory(path: &Path) -> Result<(), FleetRegistryError> {
    std::fs::create_dir_all(path)?;
    set_private_directory_mode(path)?;
    validate_private_directory(path)
}

fn validate_private_directory(path: &Path) -> Result<(), FleetRegistryError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
        return Err(FleetRegistryError::Corrupt(format!(
            "runtime bootstrap registry root is not a physical directory: {}",
            path.display()
        )));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(FleetRegistryError::Corrupt(format!(
                "runtime bootstrap registry root is not owner-only: {}",
                path.display()
            )));
        }
    }
    Ok(())
}

fn secure_metadata(path: &Path) -> Result<std::fs::Metadata, FleetRegistryError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err(FleetRegistryError::Corrupt(format!(
            "runtime bootstrap registry object is not a regular file: {}",
            path.display()
        )));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.nlink() != 1 || metadata.permissions().mode() & 0o022 != 0 {
            return Err(FleetRegistryError::Corrupt(format!(
                "runtime bootstrap registry object has unsafe links or permissions: {}",
                path.display()
            )));
        }
    }
    Ok(metadata)
}

fn secure_create_new(path: &Path) -> Result<File, FleetRegistryError> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
    }
    Ok(options.open(path)?)
}

fn secure_open_read(path: &Path) -> Result<File, FleetRegistryError> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    Ok(options.open(path)?)
}

#[cfg(unix)]
fn metadata_identity(metadata: &std::fs::Metadata) -> (u64, u64, i64, i64, u64) {
    use std::os::unix::fs::MetadataExt as _;
    (
        metadata.dev(),
        metadata.ino(),
        metadata.ctime(),
        metadata.ctime_nsec(),
        metadata.len(),
    )
}

#[cfg(not(unix))]
fn metadata_identity(metadata: &std::fs::Metadata) -> (u64, Option<std::time::SystemTime>) {
    (metadata.len(), metadata.modified().ok())
}

fn sha256_file(path: &Path) -> Result<Sha256Digest, FleetRegistryError> {
    let before = secure_metadata(path)?;
    let mut file = secure_open_read(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let after = secure_metadata(path)?;
    if metadata_identity(&before) != metadata_identity(&after) {
        return Err(FleetRegistryError::Corrupt(format!(
            "runtime release file changed while hashing: {}",
            path.display()
        )));
    }
    Ok(Sha256Digest::from_sha256_output(hasher.finalize()))
}

#[cfg(unix)]
fn set_private_directory_mode(path: &Path) -> Result<(), FleetRegistryError> {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_private_directory_mode(_path: &Path) -> Result<(), FleetRegistryError> {
    Ok(())
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), FleetRegistryError> {
    File::open(path)?.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<(), FleetRegistryError> {
    Ok(())
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
#[path = "runtime_bootstrap_registry_tests.rs"]
mod tests;
