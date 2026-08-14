use std::path::Path;

use crate::DirectoryAnchorV8;
use crate::FileIdentityV8;
use crate::NativeErrorV8;
use crate::StateRootLockV8;
use crate::invalid;

use super::FreshActiveAttemptPublicationV8;
use super::NONCE_CLAIMS_DIRECTORY_V8;
use super::NonceClaimRecordV8;
use super::PublishedRecordV8;

const MAX_NONCE_CLAIM_BYTES_V8: u64 = 64 * 1024;

/// Opaque proof that these exact claim bytes won the no-replace publication.
/// This is not execution authority: a caller must separately match every
/// binding against an opaque verified signed authority before any side effect.
#[derive(Debug)]
pub struct FreshDurableNoncePublicationV8 {
    attempt_identity_sha256: String,
    barrier_generation: u64,
    claim_sha256: String,
    machine_id_sha256: String,
    namespace: String,
    nonce: String,
    publication: PublishedRecordV8,
    signature_sha256: String,
    statement_sha256: String,
}

impl FreshDurableNoncePublicationV8 {
    pub fn attempt_identity_sha256(&self) -> &str {
        &self.attempt_identity_sha256
    }

    pub fn barrier_generation(&self) -> u64 {
        self.barrier_generation
    }

    pub fn claim_sha256(&self) -> &str {
        &self.claim_sha256
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }

    pub fn publication(&self) -> &PublishedRecordV8 {
        &self.publication
    }

    pub fn machine_id_sha256(&self) -> &str {
        &self.machine_id_sha256
    }

    pub fn signature_sha256(&self) -> &str {
        &self.signature_sha256
    }

    pub fn statement_sha256(&self) -> &str {
        &self.statement_sha256
    }
}

/// An exact committed record was already present. This branch deliberately
/// has no conversion into [`FreshDurableNoncePublicationV8`], so replay cannot
/// authorize the external side effect a second time.
#[derive(Debug)]
pub struct ExistingDurableNonceClaimV8 {
    claim_sha256: String,
    identity: FileIdentityV8,
    namespace: String,
    nonce: String,
}

impl ExistingDurableNonceClaimV8 {
    pub fn claim_sha256(&self) -> &str {
        &self.claim_sha256
    }

    pub fn identity(&self) -> FileIdentityV8 {
        self.identity
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }
}

#[derive(Debug)]
pub enum DurableNonceClaimOutcomeV8 {
    FreshPublication(FreshDurableNoncePublicationV8),
    ExactReplay(ExistingDurableNonceClaimV8),
}

pub fn claim_nonce_durably_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    active_attempt: &FreshActiveAttemptPublicationV8,
    record: &NonceClaimRecordV8,
) -> Result<DurableNonceClaimOutcomeV8, NativeErrorV8> {
    claim_nonce_durably_observed_v8(state_root, state_root_lock, active_attempt, record, |_| {})
}

pub(super) fn claim_nonce_durably_observed_v8<F>(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    active_attempt: &FreshActiveAttemptPublicationV8,
    record: &NonceClaimRecordV8,
    observe: F,
) -> Result<DurableNonceClaimOutcomeV8, NativeErrorV8>
where
    F: FnMut(super::DurablePublicationCheckpointV8),
{
    record.validate()?;
    if state_root_lock.state_root_identity() != state_root.identity() {
        return Err(invalid(
            "durable nonce claim lock belongs to a different state root",
        ));
    }
    if !active_attempt.matches_lock(state_root_lock)
        || !active_attempt.matches_state_root(state_root.identity())
        || active_attempt.attempt_identity_sha256() != record.attempt_identity_sha256
        || active_attempt.barrier_generation() != record.barrier_generation
        || active_attempt.machine_id_sha256() != record.machine_id_sha256
    {
        return Err(invalid(
            "durable nonce claim does not match the sole fresh active attempt",
        ));
    }
    state_root_lock.revalidate_for_root(state_root)?;
    let canonical = record.canonical_bytes()?;
    let claim_sha256 = record.sha256()?;
    let claims_directory =
        state_root.open_directory_beneath(Path::new(NONCE_CLAIMS_DIRECTORY_V8))?;
    let final_leaf = format!("{}.claim", record.nonce);
    let final_name = std::ffi::OsString::from(&final_leaf);

    let claim_names = claims_directory.list_leaf_names_bounded(super::MAX_NONCE_CLAIM_LEAVES_V8)?;
    if claim_names.iter().any(|name| name == &final_name) {
        let existing = claims_directory.open_regular_readonly_beneath(Path::new(&final_leaf))?;
        let identity = existing.identity();
        let directory_identity = claims_directory.identity();
        if identity.owner_uid() != directory_identity.owner_uid()
            || identity.owner_gid() != directory_identity.owner_gid()
            || identity.mode() != 0o600
            || identity.link_count() != 1
        {
            return Err(invalid(
                "existing durable nonce claim identity is not exact",
            ));
        }
        let existing_bytes = existing.read_all(MAX_NONCE_CLAIM_BYTES_V8)?;
        if existing_bytes != canonical {
            return Err(invalid(
                "global nonce is already claimed by different exact bytes",
            ));
        }
        state_root_lock.revalidate_for_root(state_root)?;
        return Ok(DurableNonceClaimOutcomeV8::ExactReplay(
            ExistingDurableNonceClaimV8 {
                claim_sha256,
                identity,
                namespace: record.namespace.clone(),
                nonce: record.nonce.clone(),
            },
        ));
    }
    super::require_inventory_capacity_v8(
        "nonce claim directory",
        claim_names.len(),
        super::MAX_NONCE_CLAIM_LEAVES_V8,
    )?;

    state_root_lock.revalidate_for_root(state_root)?;
    let publication = super::publish_record_noreplace_observed_v8(
        &claims_directory,
        &final_leaf,
        &record.nonce,
        &canonical,
        observe,
    )?;
    state_root_lock.revalidate_for_root(state_root)?;
    Ok(DurableNonceClaimOutcomeV8::FreshPublication(
        FreshDurableNoncePublicationV8 {
            attempt_identity_sha256: record.attempt_identity_sha256.clone(),
            barrier_generation: record.barrier_generation,
            claim_sha256,
            machine_id_sha256: record.machine_id_sha256.clone(),
            namespace: record.namespace.clone(),
            nonce: record.nonce.clone(),
            publication,
            signature_sha256: record.signature_sha256.clone(),
            statement_sha256: record.statement_sha256.clone(),
        },
    ))
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::fs;

    use super::*;
    use crate::ActiveAttemptPublicationOutcomeV8;
    use crate::ActiveAttemptRequestV8;
    use crate::NONCE_CLAIM_SCHEMA_V8;
    use crate::acquire_state_root_lock_v8;
    use crate::publish_active_attempt_durably_v8;

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    fn record(namespace: &str) -> NonceClaimRecordV8 {
        NonceClaimRecordV8 {
            attempt_identity_sha256: digest('1'),
            barrier_generation: 7,
            machine_id_sha256: digest('2'),
            namespace: namespace.to_string(),
            nonce: digest('3'),
            schema: NONCE_CLAIM_SCHEMA_V8.to_string(),
            signature_sha256: digest('4'),
            statement_sha256: digest('5'),
        }
    }

    fn temporary_state_root() -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "hepta-linux-v8-nonce-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        fs::create_dir(root.join(NONCE_CLAIMS_DIRECTORY_V8)).unwrap();
        root
    }

    #[test]
    fn only_first_global_claim_returns_fresh_authority() {
        let root = temporary_state_root();
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock =
            acquire_state_root_lock_v8(&anchor, std::ffi::OsStr::new("state.lock")).unwrap();
        let claim = record("hepta-linux-v8-execution");
        let active = match publish_active_attempt_durably_v8(
            &anchor,
            &mut lock,
            &ActiveAttemptRequestV8::new(
                claim.attempt_identity_sha256.clone(),
                claim.barrier_generation,
                "01234567-89ab-cdef-0123-456789abcdef".to_string(),
                claim.machine_id_sha256.clone(),
                digest('6'),
            )
            .unwrap(),
            &digest('7'),
        )
        .unwrap()
        {
            ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        assert!(matches!(
            claim_nonce_durably_v8(&anchor, &mut lock, &active, &claim).unwrap(),
            DurableNonceClaimOutcomeV8::FreshPublication(_)
        ));
        assert!(matches!(
            claim_nonce_durably_v8(&anchor, &mut lock, &active, &claim).unwrap(),
            DurableNonceClaimOutcomeV8::ExactReplay(_)
        ));

        let conflicting = record("hepta-linux-v8-break-glass");
        assert!(claim_nonce_durably_v8(&anchor, &mut lock, &active, &conflicting).is_err());

        let mut wrong_attempt = record("hepta-linux-v8-execution");
        wrong_attempt.attempt_identity_sha256 = digest('8');
        wrong_attempt.nonce = digest('9');
        assert!(claim_nonce_durably_v8(&anchor, &mut lock, &active, &wrong_attempt).is_err());
        drop(lock);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn existing_claim_mode_or_bytes_drift_fails_closed() {
        let root = temporary_state_root();
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock =
            acquire_state_root_lock_v8(&anchor, std::ffi::OsStr::new("state.lock")).unwrap();
        let claim = record("hepta-linux-v8-execution");
        let active = match publish_active_attempt_durably_v8(
            &anchor,
            &mut lock,
            &ActiveAttemptRequestV8::new(
                claim.attempt_identity_sha256.clone(),
                claim.barrier_generation,
                "01234567-89ab-cdef-0123-456789abcdef".to_string(),
                claim.machine_id_sha256.clone(),
                digest('6'),
            )
            .unwrap(),
            &digest('7'),
        )
        .unwrap()
        {
            ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        claim_nonce_durably_v8(&anchor, &mut lock, &active, &claim).unwrap();
        let claim_path = root
            .join(NONCE_CLAIMS_DIRECTORY_V8)
            .join(format!("{}.claim", claim.nonce));
        fs::write(&claim_path, b"tampered").unwrap();
        assert!(claim_nonce_durably_v8(&anchor, &mut lock, &active, &claim).is_err());
        drop(lock);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reacquired_lock_cannot_reuse_an_old_fresh_active_token() {
        let root = temporary_state_root();
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock =
            acquire_state_root_lock_v8(&anchor, std::ffi::OsStr::new("state.lock")).unwrap();
        let claim = record("hepta-linux-v8-execution");
        let active = match publish_active_attempt_durably_v8(
            &anchor,
            &mut lock,
            &ActiveAttemptRequestV8::new(
                claim.attempt_identity_sha256.clone(),
                claim.barrier_generation,
                "01234567-89ab-cdef-0123-456789abcdef".to_string(),
                claim.machine_id_sha256.clone(),
                digest('6'),
            )
            .unwrap(),
            &digest('7'),
        )
        .unwrap()
        {
            ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        drop(lock);
        let mut reacquired =
            acquire_state_root_lock_v8(&anchor, std::ffi::OsStr::new("state.lock")).unwrap();
        assert!(claim_nonce_durably_v8(&anchor, &mut reacquired, &active, &claim).is_err());
        drop(reacquired);
        fs::remove_dir_all(root).unwrap();
    }
}
