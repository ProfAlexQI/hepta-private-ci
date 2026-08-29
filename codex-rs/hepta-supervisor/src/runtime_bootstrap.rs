use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;
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
use codex_hepta_fleet::RuntimeLaunchBinding;
use ed25519_dalek::Signer as _;
use ed25519_dalek::SigningKey;
use uuid::Uuid;

use crate::AdoptSpec;
use crate::Adoption;
use crate::MatrixAdoptSpec;
use crate::MatrixSpawnSpec;
use crate::ProcessDriver;
use crate::ProcessDriverError;
use crate::SpawnSpec;
use crate::driver::SpawnedProcess;

pub const RUNTIME_BOOTSTRAP_DEFAULT_LIFETIME_SECONDS: u64 = 120;
const MAX_BOOTSTRAP_FILE_BYTES: usize = 64 * 1024;
static PUBLISH_SEQUENCE: AtomicU64 = AtomicU64::new(1);

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

/// Process-driver decorator that publishes the signed bootstrap before the
/// child process exists, then delegates all lifecycle operations unchanged.
pub struct RuntimeBootstrapProcessDriver<D> {
    inner: D,
    registry: FleetRegistry,
    issuer: Option<RuntimeBootstrapIssuer>,
}

impl<D> RuntimeBootstrapProcessDriver<D> {
    pub fn passthrough(inner: D, registry: FleetRegistry) -> Self {
        Self {
            inner,
            registry,
            issuer: None,
        }
    }

    pub fn with_issuer(
        inner: D,
        registry: FleetRegistry,
        issuer: RuntimeBootstrapIssuer,
    ) -> Self {
        Self {
            inner,
            registry,
            issuer: Some(issuer),
        }
    }

    pub fn issuer(&self) -> Option<&RuntimeBootstrapIssuer> {
        self.issuer.as_ref()
    }
}

impl<D: ProcessDriver> ProcessDriver for RuntimeBootstrapProcessDriver<D> {
    type Process = D::Process;

    fn spawn(
        &mut self,
        spec: &SpawnSpec,
    ) -> Result<SpawnedProcess<Self::Process>, ProcessDriverError> {
        if let Some(issuer) = self.issuer.as_ref() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| ProcessDriverError::new("system clock is before the Unix epoch"))?
                .as_secs();
            issuer.prepare_spawn(&self.registry, spec, now)?;
        }
        self.inner.spawn(spec)
    }

    fn adopt(&mut self, spec: &AdoptSpec) -> Result<Adoption<Self::Process>, ProcessDriverError> {
        self.inner.adopt(spec)
    }

    fn spawn_matrixd(
        &mut self,
        spec: &MatrixSpawnSpec,
    ) -> Result<SpawnedProcess<Self::Process>, ProcessDriverError> {
        self.inner.spawn_matrixd(spec)
    }

    fn adopt_matrixd(
        &mut self,
        spec: &MatrixAdoptSpec,
    ) -> Result<Adoption<Self::Process>, ProcessDriverError> {
        self.inner.adopt_matrixd(spec)
    }
}

fn publish_bootstrap_handoff(
    run_root: &Path,
    document: &RuntimeBootstrapDocument,
) -> Result<(), ProcessDriverError> {
    let generation = document.envelope.generation();
    let document_path = run_root.join(runtime_bootstrap_document_file_name(generation));
    let reservation_path = run_root.join(runtime_bootstrap_reservation_file_name(generation));
    let claim_path = run_root.join(runtime_bootstrap_claim_file_name(generation));
    for path in [&document_path, &reservation_path, &claim_path] {
        if path.exists() {
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
    set_owner_read_only(&temp_path)?;
    File::open(&temp_path)?.sync_all()?;
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
    let _ = std::fs::remove_file(temp_path);
    sync_directory(parent)
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

#[cfg(unix)]
fn set_owner_read_only(path: &Path) -> Result<(), ProcessDriverError> {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o400))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_owner_read_only(path: &Path) -> Result<(), ProcessDriverError> {
    let mut permissions = std::fs::metadata(path)?.permissions();
    permissions.set_readonly(true);
    std::fs::set_permissions(path, permissions)?;
    Ok(())
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
