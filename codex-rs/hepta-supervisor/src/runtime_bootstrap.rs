use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::ProductGraph;
use codex_hepta_contracts::RuntimeBootstrapDocument;
use codex_hepta_contracts::RuntimeBootstrapEnvelope;
use codex_hepta_contracts::RuntimeBootstrapEnvelopeFields;
use codex_hepta_contracts::RuntimeBootstrapReservation;
use codex_hepta_contracts::RuntimeBootstrapSignature;
use codex_hepta_contracts::RuntimeBootstrapTrustRoot;
use codex_hepta_contracts::RuntimeProfileBinding;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::runtime_bootstrap_claim_file_name;
use codex_hepta_contracts::runtime_bootstrap_document_file_name;
use codex_hepta_contracts::runtime_bootstrap_reservation_file_name;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::FleetRegistryError;
use codex_hepta_fleet::RuntimeLaunchBinding;
use codex_hepta_fleet::allowed_runtime_release_for_program;
use ed25519_dalek::Signer as _;
use ed25519_dalek::SigningKey;
use uuid::Uuid;

use crate::ProcessDriverError;
use crate::SpawnSpec;

pub const RUNTIME_BOOTSTRAP_DEFAULT_LIFETIME_SECONDS: u64 = 120;
const MAX_BOOTSTRAP_FILE_BYTES: usize = 64 * 1024;
static PUBLISH_SEQUENCE: AtomicU64 = AtomicU64::new(1);
static PROCESS_RUNTIME_BOOTSTRAP_ISSUER: OnceLock<RuntimeBootstrapIssuer> = OnceLock::new();

/// Supervisor-owned signer for one bounded Agentd startup identity.
///
/// The document only authenticates the local closed-world runtime profile. It
/// does not mint model, provider, tool, external-effect, fleet, operator or
/// release-promotion capabilities.
pub struct RuntimeBootstrapIssuer {
    signing_key: SigningKey,
    trust_root: RuntimeBootstrapTrustRoot,
    lifetime_seconds: u64,
}

impl RuntimeBootstrapIssuer {
    pub fn new(
        signer_key_id: impl Into<String>,
        signer_epoch: u64,
        signing_key: SigningKey,
        lifetime_seconds: u64,
    ) -> Result<Self, ProcessDriverError> {
        if lifetime_seconds == 0
            || lifetime_seconds > codex_hepta_contracts::RUNTIME_BOOTSTRAP_MAX_LIFETIME_SECONDS
        {
            return Err(ProcessDriverError::new(
                "runtime bootstrap lifetime is outside the contract bound",
            ));
        }
        let trust_root = RuntimeBootstrapTrustRoot::new(
            signer_key_id,
            signer_epoch,
            signing_key.verifying_key().to_bytes(),
        )
        .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        Ok(Self {
            signing_key,
            trust_root,
            lifetime_seconds,
        })
    }

    pub fn trust_root(&self) -> &RuntimeBootstrapTrustRoot {
        &self.trust_root
    }

    pub fn prepare_spawn(
        &self,
        registry: &FleetRegistry,
        spec: &SpawnSpec,
        issued_at_unix_seconds: u64,
    ) -> Result<RuntimeBootstrapDocument, ProcessDriverError> {
        let pinned = registry
            .resolve_runtime_bootstrap_trust_root(
                self.trust_root.signer_key_id(),
                self.trust_root.signer_epoch(),
            )
            .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        if pinned != self.trust_root {
            return Err(ProcessDriverError::new(
                "runtime bootstrap signer does not match the pinned trust root",
            ));
        }
        let record = registry
            .load()
            .map_err(|error| ProcessDriverError::new(error.to_string()))?
            .agent(&spec.agent_id)
            .cloned()
            .ok_or_else(|| ProcessDriverError::new("runtime bootstrap agent is not registered"))?;
        if record.lifecycle.generation != spec.generation
            || record.manifest.workspace.as_path() != spec.workspace
            || record.layout.home_root() != spec.home_root
            || record.layout.run_root() != spec.run_root
            || record.layout.agentd_control_socket() != spec.control_socket
        {
            return Err(ProcessDriverError::new(
                "runtime bootstrap spawn facts drifted from the fleet registry",
            ));
        }
        let release = registry
            .resolve_runtime_release_for_program(&spec.agent_id, &spec.command.program)
            .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        let authority = AuthorityGrant::agent_local(spec.agent_id.clone(), spec.generation)
            .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        let profile = RuntimeProfileBinding::for_authority(&authority)
            .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        let graph = ProductGraph::agent_local(&authority)
            .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        let launch = RuntimeLaunchBinding::for_starting(
            &record,
            release.release_id.clone(),
            &authority,
        )
        .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        let expires_at_unix_seconds = issued_at_unix_seconds
            .checked_add(self.lifetime_seconds)
            .ok_or_else(|| ProcessDriverError::new("runtime bootstrap expiry overflow"))?;
        let nonce_sha256 = runtime_bootstrap_nonce(
            spec,
            &release.provenance.agentd_binary_sha256,
            profile.profile_sha256(),
            launch.digest(),
            issued_at_unix_seconds,
        );
        let envelope = RuntimeBootstrapEnvelope::new(RuntimeBootstrapEnvelopeFields {
            subject_agent_id: spec.agent_id.clone(),
            release_id: release.release_id.as_str().to_string(),
            source_commit: release.provenance.source_commit,
            source_tree: release.provenance.source_tree,
            binary_sha256: release.provenance.agentd_binary_sha256,
            runtime_profile: profile.profile_name().to_string(),
            runtime_profile_sha256: profile.profile_sha256().clone(),
            authority_grant_sha256: authority.digest(),
            product_graph_sha256: graph.digest(),
            authority_epoch: launch.runtime_authority().authority_epoch(),
            owner_epoch: launch.runtime_authority().owner_epoch(),
            generation: launch.runtime_authority().generation(),
            fencing_token_sha256: launch
                .runtime_authority()
                .fencing_token_sha256()
                .clone(),
            signer_key_id: self.trust_root.signer_key_id().to_string(),
            signer_epoch: self.trust_root.signer_epoch(),
            issued_at_unix_seconds,
            not_before_unix_seconds: issued_at_unix_seconds,
            expires_at_unix_seconds,
            nonce_sha256,
        })
        .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        let signature = self.signing_key.sign(&envelope.signing_bytes());
        let signature = RuntimeBootstrapSignature::new(
            self.trust_root.signer_key_id(),
            self.trust_root.signer_epoch(),
            envelope.digest(),
            STANDARD.encode(signature.to_bytes()),
        )
        .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        let document = RuntimeBootstrapDocument::new(envelope, signature)
            .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        publish_bootstrap_handoff(&spec.run_root, &document)?;
        Ok(document)
    }
}

/// Installs the one process-wide bootstrap issuer before the supervisor daemon
/// starts. The issuer cannot be replaced or reconfigured after installation.
pub fn install_process_runtime_bootstrap_issuer(
    issuer: RuntimeBootstrapIssuer,
) -> Result<(), ProcessDriverError> {
    PROCESS_RUNTIME_BOOTSTRAP_ISSUER
        .set(issuer)
        .map_err(|_| ProcessDriverError::new("runtime bootstrap issuer is already installed"))
}

pub fn process_runtime_bootstrap_issuer_installed() -> bool {
    PROCESS_RUNTIME_BOOTSTRAP_ISSUER.get().is_some()
}

/// Enforces the configured bootstrap policy at the unique Agentd spawn seam.
///
/// A provenance-bound release may never be spawned without the pinned signer.
/// Legacy/unversioned launches remain on the existing closed local grant until
/// their release is explicitly enrolled in the provenance registry.
pub fn prepare_runtime_bootstrap_for_spawn(
    registry: &FleetRegistry,
    spec: &SpawnSpec,
) -> Result<Option<RuntimeBootstrapDocument>, ProcessDriverError> {
    if let Some(issuer) = PROCESS_RUNTIME_BOOTSTRAP_ISSUER.get() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ProcessDriverError::new("system clock is before the Unix epoch"))?
            .as_secs();
        return issuer.prepare_spawn(registry, spec, now).map(Some);
    }

    let allowed = allowed_runtime_release_for_program(
        registry,
        &spec.agent_id,
        &spec.command.program,
    )
    .map_err(|error| ProcessDriverError::new(error.to_string()))?;
    let Some(allowed) = allowed else {
        return Ok(None);
    };
    match registry.resolve_runtime_release_provenance(&spec.agent_id, &allowed.release_id) {
        Ok(_) => Err(ProcessDriverError::new(format!(
            "provenance-bound release {} requires a process runtime bootstrap issuer",
            allowed.release_id
        ))),
        Err(FleetRegistryError::Io(error)) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(ProcessDriverError::new(error.to_string())),
    }
}

fn publish_bootstrap_handoff(
    run_root: &Path,
    document: &RuntimeBootstrapDocument,
) -> Result<(), ProcessDriverError> {
    validate_physical_directory(run_root)?;
    let generation = document.envelope.generation();
    let document_path = run_root.join(runtime_bootstrap_document_file_name(generation));
    let reservation_path = run_root.join(runtime_bootstrap_reservation_file_name(generation));
    let claim_path = run_root.join(runtime_bootstrap_claim_file_name(generation));
    for path in [&document_path, &reservation_path, &claim_path] {
        if physical_path_exists(path)? {
            return Err(ProcessDriverError::new(format!(
                "runtime bootstrap generation already has durable state: {}",
                path.display()
            )));
        }
    }
    let reservation = RuntimeBootstrapReservation::new(document)
        .map_err(|error| ProcessDriverError::new(error.to_string()))?;
    let mut reservation_bytes = serde_json::to_vec(&reservation)
        .map_err(|error| ProcessDriverError::new(error.to_string()))?;
    reservation_bytes.push(b'\n');
    let document_bytes = document
        .encode()
        .map_err(|error| ProcessDriverError::new(error.to_string()))?;
    publish_owner_only(run_root, &reservation_path, &reservation_bytes)?;
    if let Err(error) = publish_owner_only(run_root, &document_path, &document_bytes) {
        // The fsynced reservation is intentionally retained. A partially
        // published handoff is never silently retried or reinterpreted.
        return Err(error);
    }
    Ok(())
}

fn publish_owner_only(
    parent: &Path,
    final_path: &Path,
    bytes: &[u8],
) -> Result<(), ProcessDriverError> {
    if bytes.is_empty() || bytes.len() > MAX_BOOTSTRAP_FILE_BYTES {
        return Err(ProcessDriverError::new(
            "runtime bootstrap durable object exceeds its byte bound",
        ));
    }
    let parent_metadata = validate_physical_directory(parent)?;
    let temp_path = parent.join(format!(
        ".runtime-bootstrap-{}-{}.tmp",
        std::process::id(),
        PUBLISH_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
    }
    let mut file = options.open(&temp_path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    set_file_owner_read_only(&file)?;
    file.sync_all()?;
    let held = file.metadata()?;
    validate_owner_only_metadata(&temp_path, &held, &parent_metadata, 1)?;
    let path_metadata = std::fs::symlink_metadata(&temp_path)?;
    if physical_identity(&held) != physical_identity(&path_metadata) {
        return Err(ProcessDriverError::new(
            "runtime bootstrap temporary path drifted before publication",
        ));
    }
    match std::fs::hard_link(&temp_path, final_path) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            let _ = std::fs::remove_file(&temp_path);
            return Err(ProcessDriverError::new(format!(
                "runtime bootstrap durable object already exists: {}",
                final_path.display()
            )));
        }
        Err(error) => {
            let _ = std::fs::remove_file(&temp_path);
            return Err(error.into());
        }
    }
    let linked = std::fs::symlink_metadata(final_path)?;
    validate_owner_only_metadata(final_path, &linked, &parent_metadata, 2)?;
    if physical_identity(&held) != physical_identity(&linked) {
        return Err(ProcessDriverError::new(
            "runtime bootstrap published path does not bind the fsynced inode",
        ));
    }
    std::fs::remove_file(&temp_path)?;
    sync_directory(parent)?;
    let published = std::fs::symlink_metadata(final_path)?;
    validate_owner_only_metadata(final_path, &published, &parent_metadata, 1)?;
    if physical_identity(&held) != physical_identity(&published) {
        return Err(ProcessDriverError::new(
            "runtime bootstrap published inode drifted after directory sync",
        ));
    }
    Ok(())
}

fn physical_path_exists(path: &Path) -> Result<bool, ProcessDriverError> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

fn runtime_bootstrap_nonce(
    spec: &SpawnSpec,
    binary_sha256: &Sha256Digest,
    profile_sha256: &Sha256Digest,
    launch_sha256: &Sha256Digest,
    issued_at_unix_seconds: u64,
) -> Sha256Digest {
    let random = Uuid::new_v4();
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:runtime-bootstrap-nonce:v1");
    frame(&mut bytes, random.as_bytes());
    frame(&mut bytes, spec.agent_id.as_str().as_bytes());
    frame(&mut bytes, &spec.generation.to_be_bytes());
    frame(&mut bytes, binary_sha256.as_str().as_bytes());
    frame(&mut bytes, profile_sha256.as_str().as_bytes());
    frame(&mut bytes, launch_sha256.as_str().as_bytes());
    frame(&mut bytes, &issued_at_unix_seconds.to_be_bytes());
    Sha256Digest::for_bytes(&bytes)
}

fn validate_physical_directory(path: &Path) -> Result<std::fs::Metadata, ProcessDriverError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
        return Err(ProcessDriverError::new(format!(
            "runtime bootstrap parent is not a physical directory: {}",
            path.display()
        )));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(ProcessDriverError::new(format!(
                "runtime bootstrap parent is not owner-only: {}",
                path.display()
            )));
        }
    }
    Ok(metadata)
}

fn validate_owner_only_metadata(
    path: &Path,
    metadata: &std::fs::Metadata,
    parent_metadata: &std::fs::Metadata,
    expected_links: u64,
) -> Result<(), ProcessDriverError> {
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err(ProcessDriverError::new(format!(
            "runtime bootstrap object is not a physical regular file: {}",
            path.display()
        )));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.uid() != parent_metadata.uid()
            || metadata.nlink() != expected_links
            || metadata.permissions().mode() & 0o777 != 0o400
        {
            return Err(ProcessDriverError::new(format!(
                "runtime bootstrap object is not owner-bound, owner-read-only, or link-exact: {}",
                path.display()
            )));
        }
    }
    #[cfg(not(unix))]
    {
        let _ = parent_metadata;
        let _ = expected_links;
        if !metadata.permissions().readonly() {
            return Err(ProcessDriverError::new(format!(
                "runtime bootstrap object is not read-only: {}",
                path.display()
            )));
        }
    }
    Ok(())
}

#[cfg(unix)]
fn set_file_owner_read_only(file: &File) -> Result<(), ProcessDriverError> {
    use std::os::unix::fs::PermissionsExt as _;
    file.set_permissions(std::fs::Permissions::from_mode(0o400))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_file_owner_read_only(file: &File) -> Result<(), ProcessDriverError> {
    let mut permissions = file.metadata()?.permissions();
    permissions.set_readonly(true);
    file.set_permissions(permissions)?;
    Ok(())
}

#[cfg(unix)]
fn physical_identity(metadata: &std::fs::Metadata) -> (u64, u64, u64) {
    use std::os::unix::fs::MetadataExt as _;
    (metadata.dev(), metadata.ino(), metadata.len())
}

#[cfg(not(unix))]
fn physical_identity(metadata: &std::fs::Metadata) -> (u64,) {
    (metadata.len(),)
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), ProcessDriverError> {
    File::open(path)?.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<(), ProcessDriverError> {
    Ok(())
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
#[path = "runtime_bootstrap_tests.rs"]
mod tests;
