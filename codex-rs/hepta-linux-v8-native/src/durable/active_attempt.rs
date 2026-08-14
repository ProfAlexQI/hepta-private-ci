use std::ffi::OsString;
use std::path::Path;
use std::sync::Arc;

use sha2::Digest as _;

use crate::DirectoryAnchorV8;
use crate::FileIdentityV8;
use crate::NativeErrorV8;
use crate::StateRootLockSessionV8;
use crate::StateRootLockV8;
use crate::invalid;

use super::PublishedRecordV8;
use super::validate_boot_id_v8;
use super::validate_digest;

pub const ACTIVE_ATTEMPT_LEAF_V8: &str = "active-attempt.binding";
const ACTIVE_ATTEMPT_SCHEMA_V8: &[u8] = b"hepta-linux-v8-active-attempt-v1\0";
const ACTIVE_ATTEMPT_RECORD_BYTES_V8: usize =
    ACTIVE_ATTEMPT_SCHEMA_V8.len() + 64 + 8 + 64 + 36 + 64 + 8 + 8 + 4 + 4 + 4;

/// Structurally valid request to create the one active attempt for a state
/// root. It is not authority; the caller must separately bind it to a verified
/// one-shot authority and trusted state-root profile.
#[derive(Debug, Eq, PartialEq)]
pub struct ActiveAttemptRequestV8 {
    attempt_identity_sha256: String,
    barrier_generation: u64,
    boot_id: String,
    machine_id_sha256: String,
    restore_plan_sha256: String,
}

impl ActiveAttemptRequestV8 {
    pub fn new(
        attempt_identity_sha256: String,
        barrier_generation: u64,
        boot_id: String,
        machine_id_sha256: String,
        restore_plan_sha256: String,
    ) -> Result<Self, NativeErrorV8> {
        let request = Self {
            attempt_identity_sha256,
            barrier_generation,
            boot_id,
            machine_id_sha256,
            restore_plan_sha256,
        };
        request.validate()?;
        Ok(request)
    }

    fn validate(&self) -> Result<(), NativeErrorV8> {
        validate_digest("active attempt", &self.attempt_identity_sha256)?;
        validate_digest("active machine", &self.machine_id_sha256)?;
        validate_digest("active restore plan", &self.restore_plan_sha256)?;
        validate_boot_id_v8(&self.boot_id)?;
        if self.barrier_generation == 0 {
            return Err(invalid("active attempt generation must be non-zero"));
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ActiveAttemptRecordV8 {
    attempt_identity_sha256: String,
    barrier_generation: u64,
    boot_id: String,
    machine_id_sha256: String,
    restore_plan_sha256: String,
    state_root_device: u64,
    state_root_inode: u64,
    state_root_mode: u32,
    state_root_owner_gid: u32,
    state_root_owner_uid: u32,
}

impl ActiveAttemptRecordV8 {
    fn from_request(request: &ActiveAttemptRequestV8, root: FileIdentityV8) -> Self {
        Self {
            attempt_identity_sha256: request.attempt_identity_sha256.clone(),
            barrier_generation: request.barrier_generation,
            boot_id: request.boot_id.clone(),
            machine_id_sha256: request.machine_id_sha256.clone(),
            restore_plan_sha256: request.restore_plan_sha256.clone(),
            state_root_device: root.device(),
            state_root_inode: root.inode(),
            state_root_mode: root.mode(),
            state_root_owner_gid: root.owner_gid(),
            state_root_owner_uid: root.owner_uid(),
        }
    }

    fn validate(&self) -> Result<(), NativeErrorV8> {
        validate_digest("active attempt", &self.attempt_identity_sha256)?;
        validate_digest("active machine", &self.machine_id_sha256)?;
        validate_digest("active restore plan", &self.restore_plan_sha256)?;
        validate_boot_id_v8(&self.boot_id)?;
        if self.barrier_generation == 0
            || self.state_root_device == 0
            || self.state_root_inode == 0
            || self.state_root_mode > 0o7777
        {
            return Err(invalid("active attempt scalar binding is malformed"));
        }
        Ok(())
    }

    fn canonical_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate()?;
        let mut bytes = Vec::with_capacity(ACTIVE_ATTEMPT_RECORD_BYTES_V8);
        bytes.extend_from_slice(ACTIVE_ATTEMPT_SCHEMA_V8);
        bytes.extend_from_slice(self.attempt_identity_sha256.as_bytes());
        bytes.extend_from_slice(&self.barrier_generation.to_be_bytes());
        bytes.extend_from_slice(self.machine_id_sha256.as_bytes());
        bytes.extend_from_slice(self.boot_id.as_bytes());
        bytes.extend_from_slice(self.restore_plan_sha256.as_bytes());
        bytes.extend_from_slice(&self.state_root_device.to_be_bytes());
        bytes.extend_from_slice(&self.state_root_inode.to_be_bytes());
        bytes.extend_from_slice(&self.state_root_owner_uid.to_be_bytes());
        bytes.extend_from_slice(&self.state_root_owner_gid.to_be_bytes());
        bytes.extend_from_slice(&self.state_root_mode.to_be_bytes());
        debug_assert_eq!(bytes.len(), ACTIVE_ATTEMPT_RECORD_BYTES_V8);
        Ok(bytes)
    }

    fn decode_exact(bytes: &[u8]) -> Result<Self, NativeErrorV8> {
        if bytes.len() != ACTIVE_ATTEMPT_RECORD_BYTES_V8
            || !bytes.starts_with(ACTIVE_ATTEMPT_SCHEMA_V8)
        {
            return Err(invalid(
                "active attempt record length or schema is not exact",
            ));
        }
        let mut offset = ACTIVE_ATTEMPT_SCHEMA_V8.len();
        let attempt_identity_sha256 = take_ascii(bytes, &mut offset, 64, "active attempt")?;
        let barrier_generation = take_u64(bytes, &mut offset)?;
        let machine_id_sha256 = take_ascii(bytes, &mut offset, 64, "active machine")?;
        let boot_id = take_ascii(bytes, &mut offset, 36, "active boot id")?;
        let restore_plan_sha256 = take_ascii(bytes, &mut offset, 64, "active restore plan")?;
        let state_root_device = take_u64(bytes, &mut offset)?;
        let state_root_inode = take_u64(bytes, &mut offset)?;
        let state_root_owner_uid = take_u32(bytes, &mut offset)?;
        let state_root_owner_gid = take_u32(bytes, &mut offset)?;
        let state_root_mode = take_u32(bytes, &mut offset)?;
        if offset != bytes.len() {
            return Err(invalid("active attempt record has trailing bytes"));
        }
        let record = Self {
            attempt_identity_sha256,
            barrier_generation,
            boot_id,
            machine_id_sha256,
            restore_plan_sha256,
            state_root_device,
            state_root_inode,
            state_root_mode,
            state_root_owner_gid,
            state_root_owner_uid,
        };
        record.validate()?;
        if record.canonical_bytes()? != bytes {
            return Err(invalid("active attempt record is not canonical"));
        }
        Ok(record)
    }

    fn sha256(&self) -> Result<String, NativeErrorV8> {
        Ok(format!(
            "{:x}",
            sha2::Sha256::digest(self.canonical_bytes()?)
        ))
    }

    fn matches_root(&self, root: FileIdentityV8) -> bool {
        self.state_root_device == root.device()
            && self.state_root_inode == root.inode()
            && self.state_root_owner_uid == root.owner_uid()
            && self.state_root_owner_gid == root.owner_gid()
            && self.state_root_mode == root.mode()
    }
}

/// Opaque proof that this process won the no-replace publication of the sole
/// active-attempt binding. It is intentionally not Clone or Deserialize;
/// after restart, the existing leaf can only produce the recovery branch.
#[derive(Debug)]
pub struct FreshActiveAttemptPublicationV8 {
    creator_pid: u32,
    lock_session: Arc<StateRootLockSessionV8>,
    record: ActiveAttemptRecordV8,
    record_sha256: String,
    publication: PublishedRecordV8,
}

impl FreshActiveAttemptPublicationV8 {
    pub fn attempt_identity_sha256(&self) -> &str {
        &self.record.attempt_identity_sha256
    }

    pub fn barrier_generation(&self) -> u64 {
        self.record.barrier_generation
    }

    pub fn boot_id(&self) -> &str {
        &self.record.boot_id
    }

    pub fn machine_id_sha256(&self) -> &str {
        &self.record.machine_id_sha256
    }

    pub fn restore_plan_sha256(&self) -> &str {
        &self.record.restore_plan_sha256
    }

    pub fn record_sha256(&self) -> &str {
        &self.record_sha256
    }

    pub fn publication(&self) -> &PublishedRecordV8 {
        &self.publication
    }

    pub(crate) fn matches_state_root(&self, root: FileIdentityV8) -> bool {
        self.record.matches_root(root)
    }

    pub(crate) fn matches_lock(&self, lock: &StateRootLockV8) -> bool {
        self.creator_pid == lock.owner_pid() && lock.matches_session(&self.lock_session)
    }
}

/// Evidence that an active attempt already existed. This type deliberately
/// cannot become a fresh token; admission after process restart must recover,
/// abandon, and quarantine instead of continuing the qualification.
#[derive(Debug)]
pub struct ExistingActiveAttemptV8 {
    record: ActiveAttemptRecordV8,
    record_sha256: String,
    identity: FileIdentityV8,
}

impl ExistingActiveAttemptV8 {
    pub fn attempt_identity_sha256(&self) -> &str {
        &self.record.attempt_identity_sha256
    }

    pub fn barrier_generation(&self) -> u64 {
        self.record.barrier_generation
    }

    pub fn boot_id(&self) -> &str {
        &self.record.boot_id
    }

    pub fn machine_id_sha256(&self) -> &str {
        &self.record.machine_id_sha256
    }

    pub fn restore_plan_sha256(&self) -> &str {
        &self.record.restore_plan_sha256
    }

    pub fn record_sha256(&self) -> &str {
        &self.record_sha256
    }

    pub fn identity(&self) -> FileIdentityV8 {
        self.identity
    }
}

#[derive(Debug)]
pub enum ActiveAttemptPublicationOutcomeV8 {
    Fresh(FreshActiveAttemptPublicationV8),
    ExistingRequiresRecovery(ExistingActiveAttemptV8),
}

pub fn publish_active_attempt_durably_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    request: &ActiveAttemptRequestV8,
    publication_nonce: &str,
) -> Result<ActiveAttemptPublicationOutcomeV8, NativeErrorV8> {
    publish_active_attempt_durably_observed_v8(
        state_root,
        state_root_lock,
        request,
        publication_nonce,
        |_| {},
    )
}

pub(super) fn publish_active_attempt_durably_observed_v8<F>(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    request: &ActiveAttemptRequestV8,
    publication_nonce: &str,
    observe: F,
) -> Result<ActiveAttemptPublicationOutcomeV8, NativeErrorV8>
where
    F: FnMut(super::DurablePublicationCheckpointV8),
{
    request.validate()?;
    validate_digest("active publication nonce", publication_nonce)?;
    if !state_root_lock
        .state_root_identity()
        .matches_stable_directory(state_root.identity())
    {
        return Err(invalid(
            "active attempt lock belongs to a different state root",
        ));
    }
    state_root_lock.revalidate_for_root(state_root)?;
    let record = ActiveAttemptRecordV8::from_request(request, state_root.identity());
    let canonical = record.canonical_bytes()?;
    let leaf = OsString::from(ACTIVE_ATTEMPT_LEAF_V8);
    let before = state_root.list_leaf_names_bounded(super::MAX_STATE_ROOT_LEAVES_V8)?;
    state_root_lock.revalidate_for_root(state_root)?;
    let after = state_root.list_leaf_names_bounded(super::MAX_STATE_ROOT_LEAVES_V8)?;
    if before != after {
        return Err(invalid(
            "state-root inventory changed while checking active attempt",
        ));
    }
    if after.iter().any(|name| name == &leaf) {
        let existing = read_existing_active_attempt(state_root)?;
        state_root_lock.revalidate_for_root(state_root)?;
        return Ok(existing);
    }
    let incoming_prefix = format!(".{ACTIVE_ATTEMPT_LEAF_V8}.");
    if after.iter().any(|name| {
        name.to_str()
            .is_some_and(|name| name.starts_with(&incoming_prefix) && name.ends_with(".incoming"))
    }) {
        return Err(invalid(
            "interrupted active-attempt publication requires recovery",
        ));
    }
    super::require_inventory_capacity_v8(
        "state root",
        after.len(),
        super::MAX_STATE_ROOT_LEAVES_V8,
    )?;

    let publication = super::publish_record_noreplace_observed_v8(
        state_root,
        ACTIVE_ATTEMPT_LEAF_V8,
        publication_nonce,
        &canonical,
        observe,
    )?;
    state_root_lock.revalidate_for_root(state_root)?;
    let reopened = state_root.open_regular_readonly_beneath(Path::new(ACTIVE_ATTEMPT_LEAF_V8))?;
    let reopened_bytes = reopened.read_all(ACTIVE_ATTEMPT_RECORD_BYTES_V8 as u64)?;
    let reopened_record = ActiveAttemptRecordV8::decode_exact(&reopened_bytes)?;
    if reopened.identity() != publication.identity()
        || reopened_record != record
        || !reopened_record.matches_root(state_root.identity())
    {
        return Err(invalid(
            "published active attempt did not reopen as the exact state-root binding",
        ));
    }
    state_root_lock.revalidate_for_root(state_root)?;
    Ok(ActiveAttemptPublicationOutcomeV8::Fresh(
        FreshActiveAttemptPublicationV8 {
            creator_pid: state_root_lock.owner_pid(),
            lock_session: state_root_lock.session_handle(),
            record_sha256: record.sha256()?,
            record,
            publication,
        },
    ))
}

fn read_existing_active_attempt(
    state_root: &DirectoryAnchorV8,
) -> Result<ActiveAttemptPublicationOutcomeV8, NativeErrorV8> {
    let existing = state_root.open_regular_readonly_beneath(Path::new(ACTIVE_ATTEMPT_LEAF_V8))?;
    let identity = existing.identity();
    let root = state_root.identity();
    if identity.owner_uid() != root.owner_uid()
        || identity.owner_gid() != root.owner_gid()
        || identity.mode() != 0o600
        || identity.link_count() != 1
    {
        return Err(invalid("existing active attempt identity is not exact"));
    }
    let bytes = existing.read_all(ACTIVE_ATTEMPT_RECORD_BYTES_V8 as u64)?;
    let record = ActiveAttemptRecordV8::decode_exact(&bytes)?;
    if !record.matches_root(root) {
        return Err(invalid(
            "existing active attempt is bound to a different state root identity",
        ));
    }
    Ok(ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(
        ExistingActiveAttemptV8 {
            record_sha256: record.sha256()?,
            record,
            identity,
        },
    ))
}

fn take_ascii(
    bytes: &[u8],
    offset: &mut usize,
    length: usize,
    label: &str,
) -> Result<String, NativeErrorV8> {
    let end = offset
        .checked_add(length)
        .ok_or_else(|| invalid("active attempt decode offset overflow"))?;
    let value = bytes
        .get(*offset..end)
        .ok_or_else(|| invalid(format!("active attempt {label} is truncated")))?;
    *offset = end;
    if !value.is_ascii() {
        return Err(invalid(format!("active attempt {label} is not ASCII")));
    }
    String::from_utf8(value.to_vec()).map_err(|_| invalid("active attempt ASCII decode failed"))
}

fn take_u64(bytes: &[u8], offset: &mut usize) -> Result<u64, NativeErrorV8> {
    let end = offset
        .checked_add(8)
        .ok_or_else(|| invalid("active attempt decode offset overflow"))?;
    let value: [u8; 8] = bytes
        .get(*offset..end)
        .ok_or_else(|| invalid("active attempt scalar is truncated"))?
        .try_into()
        .map_err(|_| invalid("active attempt scalar length is invalid"))?;
    *offset = end;
    Ok(u64::from_be_bytes(value))
}

fn take_u32(bytes: &[u8], offset: &mut usize) -> Result<u32, NativeErrorV8> {
    let end = offset
        .checked_add(4)
        .ok_or_else(|| invalid("active attempt decode offset overflow"))?;
    let value: [u8; 4] = bytes
        .get(*offset..end)
        .ok_or_else(|| invalid("active attempt scalar is truncated"))?
        .try_into()
        .map_err(|_| invalid("active attempt scalar length is invalid"))?;
    *offset = end;
    Ok(u32::from_be_bytes(value))
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::ffi::OsStr;
    use std::fs;

    use super::*;
    use crate::acquire_state_root_lock_v8;

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    fn request() -> ActiveAttemptRequestV8 {
        ActiveAttemptRequestV8::new(
            digest('1'),
            7,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            digest('2'),
            digest('3'),
        )
        .unwrap()
    }

    fn temporary_state_root() -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "hepta-linux-v8-active-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        root
    }

    #[test]
    fn only_first_process_lifetime_gets_fresh_token() {
        let root = temporary_state_root();
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        let first = publish_active_attempt_durably_v8(&anchor, &mut lock, &request(), &digest('4'))
            .unwrap();
        let fresh = match first {
            ActiveAttemptPublicationOutcomeV8::Fresh(fresh) => fresh,
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        assert_eq!(fresh.barrier_generation(), 7);
        assert!(fresh.matches_state_root(anchor.identity()));

        drop(lock);
        let mut restarted_lock =
            acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        let second = publish_active_attempt_durably_v8(
            &anchor,
            &mut restarted_lock,
            &request(),
            &digest('5'),
        )
        .unwrap();
        let existing = match second {
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(existing) => existing,
            ActiveAttemptPublicationOutcomeV8::Fresh(_) => panic!(),
        };
        assert_eq!(existing.attempt_identity_sha256(), digest('1'));
        assert_eq!(existing.record_sha256(), fresh.record_sha256());
        drop(restarted_lock);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn different_attempt_or_generation_never_reactivates_existing_leaf() {
        let root = temporary_state_root();
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        publish_active_attempt_durably_v8(&anchor, &mut lock, &request(), &digest('4')).unwrap();
        let different = ActiveAttemptRequestV8::new(
            digest('5'),
            8,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            digest('6'),
            digest('7'),
        )
        .unwrap();
        assert!(matches!(
            publish_active_attempt_durably_v8(&anchor, &mut lock, &different, &digest('8'))
                .unwrap(),
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_)
        ));
        drop(lock);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn tampered_existing_record_fails_closed() {
        let root = temporary_state_root();
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        publish_active_attempt_durably_v8(&anchor, &mut lock, &request(), &digest('4')).unwrap();
        fs::write(root.join(ACTIVE_ATTEMPT_LEAF_V8), b"tampered").unwrap();
        assert!(
            publish_active_attempt_durably_v8(&anchor, &mut lock, &request(), &digest('5'))
                .is_err()
        );
        drop(lock);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn interrupted_active_incoming_never_yields_fresh_authority() {
        let root = temporary_state_root();
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        fs::write(
            root.join(format!(
                ".{ACTIVE_ATTEMPT_LEAF_V8}.{}.incoming",
                digest('4')
            )),
            b"interrupted",
        )
        .unwrap();
        assert!(
            publish_active_attempt_durably_v8(&anchor, &mut lock, &request(), &digest('5'))
                .is_err()
        );
        drop(lock);
        fs::remove_dir_all(root).unwrap();
    }
}
