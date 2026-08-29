use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Read;
use std::path::Path;

use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::ProductGraph;
use codex_hepta_contracts::RuntimeBootstrapDocument;
use codex_hepta_contracts::RuntimeBootstrapExpectation;
use codex_hepta_contracts::RuntimeBootstrapReservation;
use codex_hepta_contracts::RuntimeProfileBinding;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::runtime_bootstrap_claim_file_name;
use codex_hepta_contracts::runtime_bootstrap_document_file_name;
use codex_hepta_contracts::runtime_bootstrap_reservation_file_name;
use codex_hepta_contracts::verify_runtime_bootstrap;
use codex_hepta_fleet::AgentRecord;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_fleet::FleetRegistryError;
use codex_hepta_fleet::RuntimeLaunchBinding;
use codex_hepta_fleet::RuntimeReleaseProvenance;
use codex_hepta_fleet::allowed_runtime_release_for_program;

use crate::AgentdError;

const MAX_BOOTSTRAP_FILE_BYTES: u64 = 64 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeBootstrapAdmission {
    /// Direct/unversioned or legacy catalog startup with no installed
    /// provenance and no bootstrap state. Its authority remains the existing
    /// local closed-world grant.
    LocalClosedWorld,
    Verified {
        release_id: String,
        document_sha256: Sha256Digest,
        nonce_sha256: Sha256Digest,
        signer_key_id: String,
        signer_epoch: u64,
    },
}

impl RuntimeBootstrapAdmission {
    pub const fn is_verified(&self) -> bool {
        matches!(self, Self::Verified { .. })
    }
}

pub(crate) fn consume_runtime_bootstrap(
    registry: &FleetRegistry,
    record: &AgentRecord,
    executable: &Path,
    observed_at_unix_seconds: u64,
) -> Result<RuntimeBootstrapAdmission, AgentdError> {
    let generation = record.lifecycle.generation;
    let document_path = record
        .layout
        .run_root()
        .join(runtime_bootstrap_document_file_name(generation));
    let reservation_path = record
        .layout
        .run_root()
        .join(runtime_bootstrap_reservation_file_name(generation));
    let claim_path = record
        .layout
        .run_root()
        .join(runtime_bootstrap_claim_file_name(generation));
    let document_exists = physical_path_exists(&document_path)?;
    let reservation_exists = physical_path_exists(&reservation_path)?;
    let claim_exists = physical_path_exists(&claim_path)?;

    if claim_exists {
        return Err(AgentdError::GenerationFenced(format!(
            "runtime bootstrap generation {generation} was already consumed"
        )));
    }
    if document_exists != reservation_exists {
        return Err(AgentdError::GenerationFenced(format!(
            "runtime bootstrap generation {generation} has a partial durable handoff"
        )));
    }

    let allowed = allowed_runtime_release_for_program(
        registry,
        &record.manifest.agent_id,
        executable,
    )?;
    let Some(allowed) = allowed else {
        if document_exists || reservation_exists {
            return Err(AgentdError::GenerationFenced(
                "runtime bootstrap exists for an executable outside the allowed release catalog"
                    .to_string(),
            ));
        }
        return Ok(RuntimeBootstrapAdmission::LocalClosedWorld);
    };

    let provenance = match registry.resolve_runtime_release_provenance(
        &record.manifest.agent_id,
        &allowed.release_id,
    ) {
        Ok(provenance) => provenance,
        Err(FleetRegistryError::Io(error)) if error.kind() == ErrorKind::NotFound => {
            if document_exists || reservation_exists {
                return Err(AgentdError::GenerationFenced(
                    "runtime bootstrap handoff has no installed release provenance".to_string(),
                ));
            }
            return Ok(RuntimeBootstrapAdmission::LocalClosedWorld);
        }
        Err(error) => return Err(error.into()),
    };
    if !document_exists || !reservation_exists {
        return Err(AgentdError::GenerationFenced(format!(
            "provenance-bound release {} has no signed runtime bootstrap",
            allowed.release_id
        )));
    }

    let document = RuntimeBootstrapDocument::decode(&read_owner_only(&document_path)?)
        .map_err(|error| AgentdError::GenerationFenced(error.to_string()))?;
    let reservation: RuntimeBootstrapReservation =
        serde_json::from_slice(&read_owner_only(&reservation_path)?).map_err(|error| {
            AgentdError::GenerationFenced(format!(
                "runtime bootstrap reservation decode failed: {error}"
            ))
        })?;
    reservation
        .validate()
        .map_err(|error| AgentdError::GenerationFenced(error.to_string()))?;
    validate_reservation(&reservation, &document, record)?;

    let trust_root = registry.resolve_runtime_bootstrap_trust_root(
        document.envelope.signer_key_id(),
        document.envelope.signer_epoch(),
    )?;
    let authority = AuthorityGrant::agent_local(
        record.manifest.agent_id.clone(),
        record.lifecycle.generation,
    )
    .map_err(|error| AgentdError::GenerationFenced(error.to_string()))?;
    let profile = RuntimeProfileBinding::for_authority(&authority)
        .map_err(|error| AgentdError::GenerationFenced(error.to_string()))?;
    let graph = ProductGraph::agent_local(&authority)
        .map_err(|error| AgentdError::GenerationFenced(error.to_string()))?;
    let launch = RuntimeLaunchBinding::for_starting(
        record,
        allowed.release_id.clone(),
        &authority,
    )
    .map_err(|error| AgentdError::GenerationFenced(error.to_string()))?;
    let expected = bootstrap_expectation(
        record,
        &allowed.release_id,
        &provenance,
        &authority,
        &profile,
        &graph,
        &launch,
        document.envelope.signer_key_id(),
        document.envelope.signer_epoch(),
    );
    let verified = verify_runtime_bootstrap(
        &document,
        &expected,
        observed_at_unix_seconds,
        &trust_root
            .verifier()
            .map_err(|error| AgentdError::GenerationFenced(error.to_string()))?,
    )
    .map_err(|error| AgentdError::GenerationFenced(error.to_string()))?;

    claim_reservation(&reservation_path, &claim_path, record.layout.run_root())?;

    // The claim is durable before this second read. Any drift after the claim
    // is retained as consumed/recovery-required rather than retried.
    let current = registry
        .load()?
        .agent(&record.manifest.agent_id)
        .cloned()
        .ok_or_else(|| AgentdError::GenerationFenced("agent disappeared after claim".to_string()))?;
    let current_provenance = registry.resolve_runtime_release_provenance(
        &record.manifest.agent_id,
        &allowed.release_id,
    )?;
    if current.lifecycle != record.lifecycle
        || current.release_state != record.release_state
        || current.manifest != record.manifest
        || current_provenance != provenance
    {
        return Err(AgentdError::GenerationFenced(
            "runtime bootstrap fleet facts drifted after nonce claim".to_string(),
        ));
    }

    std::fs::remove_file(&document_path)?;
    std::fs::remove_file(&reservation_path)?;
    sync_directory(record.layout.run_root())?;
    Ok(RuntimeBootstrapAdmission::Verified {
        release_id: allowed.release_id.to_string(),
        document_sha256: verified.document_sha256().clone(),
        nonce_sha256: verified.nonce_sha256().clone(),
        signer_key_id: document.envelope.signer_key_id().to_string(),
        signer_epoch: document.envelope.signer_epoch(),
    })
}

#[allow(clippy::too_many_arguments)]
fn bootstrap_expectation(
    record: &AgentRecord,
    release_id: &codex_hepta_fleet::ReleaseId,
    provenance: &RuntimeReleaseProvenance,
    authority: &AuthorityGrant,
    profile: &RuntimeProfileBinding,
    graph: &ProductGraph,
    launch: &RuntimeLaunchBinding,
    signer_key_id: &str,
    signer_epoch: u64,
) -> RuntimeBootstrapExpectation {
    RuntimeBootstrapExpectation {
        subject_agent_id: record.manifest.agent_id.clone(),
        release_id: release_id.to_string(),
        source_commit: provenance.source_commit.clone(),
        source_tree: provenance.source_tree.clone(),
        binary_sha256: provenance.agentd_binary_sha256.clone(),
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
        signer_key_id: signer_key_id.to_string(),
        signer_epoch,
    }
}

fn validate_reservation(
    reservation: &RuntimeBootstrapReservation,
    document: &RuntimeBootstrapDocument,
    record: &AgentRecord,
) -> Result<(), AgentdError> {
    if reservation.subject_agent_id != record.manifest.agent_id
        || reservation.generation != record.lifecycle.generation
        || reservation.envelope_sha256 != document.digest()
        || reservation.nonce_sha256 != *document.envelope.nonce_sha256()
    {
        return Err(AgentdError::GenerationFenced(
            "runtime bootstrap reservation does not bind the signed handoff".to_string(),
        ));
    }
    Ok(())
}

fn claim_reservation(
    reservation_path: &Path,
    claim_path: &Path,
    run_root: &Path,
) -> Result<(), AgentdError> {
    match std::fs::hard_link(reservation_path, claim_path) {
        Ok(()) => sync_directory(run_root),
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            Err(AgentdError::GenerationFenced(
                "runtime bootstrap nonce was already claimed".to_string(),
            ))
        }
        Err(error) => Err(error.into()),
    }
}

fn physical_path_exists(path: &Path) -> Result<bool, AgentdError> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

fn read_owner_only(path: &Path) -> Result<Vec<u8>, AgentdError> {
    let before = secure_metadata(path)?;
    if before.len() == 0 || before.len() > MAX_BOOTSTRAP_FILE_BYTES {
        return Err(AgentdError::GenerationFenced(format!(
            "runtime bootstrap object is outside its byte bound: {}",
            path.display()
        )));
    }
    let mut options = OpenOptions::new();
    options.read(true);
    let mut file = options.open(path)?;
    let opened = file.metadata()?;
    if metadata_identity(&before) != metadata_identity(&opened) {
        return Err(AgentdError::GenerationFenced(
            "runtime bootstrap path changed before open".to_string(),
        ));
    }
    let mut bytes = Vec::with_capacity(before.len() as usize);
    file.take(MAX_BOOTSTRAP_FILE_BYTES + 1)
        .read_to_end(&mut bytes)?;
    let after = secure_metadata(path)?;
    if metadata_identity(&before) != metadata_identity(&after)
        || bytes.len() as u64 > MAX_BOOTSTRAP_FILE_BYTES
    {
        return Err(AgentdError::GenerationFenced(
            "runtime bootstrap object changed while reading".to_string(),
        ));
    }
    Ok(bytes)
}

fn secure_metadata(path: &Path) -> Result<std::fs::Metadata, AgentdError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err(AgentdError::GenerationFenced(format!(
            "runtime bootstrap object is not a physical regular file: {}",
            path.display()
        )));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.nlink() != 1 || metadata.permissions().mode() & 0o777 != 0o400 {
            return Err(AgentdError::GenerationFenced(format!(
                "runtime bootstrap object is not single-link owner-read-only: {}",
                path.display()
            )));
        }
    }
    #[cfg(not(unix))]
    if !metadata.permissions().readonly() {
        return Err(AgentdError::GenerationFenced(format!(
            "runtime bootstrap object is not read-only: {}",
            path.display()
        )));
    }
    Ok(metadata)
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

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), AgentdError> {
    File::open(path)?.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<(), AgentdError> {
    Ok(())
}

#[cfg(test)]
#[path = "runtime_bootstrap_tests.rs"]
mod tests;
