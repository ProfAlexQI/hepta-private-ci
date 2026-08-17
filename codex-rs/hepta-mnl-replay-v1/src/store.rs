use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::os::fd::OwnedFd;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use codex_hepta_mnl_trust_v1::PreparedCopyAckReplayClaimV1;
use codex_hepta_mnl_trust_v1::PreparedPreRunReplayClaimV1;
use sha2::Digest;

use crate::ReplayStoreErrorV1;
use crate::ReplayStoreResultV1;
use crate::error::invalid;
use crate::error::io_error;
use crate::secure_fs::FileIdentityV1;
use crate::secure_fs::fsync_directory;
use crate::secure_fs::identity_for_fd;
use crate::secure_fs::open_child_directory;
use crate::secure_fs::open_claim_file;
use crate::secure_fs::open_root_directory;
use crate::secure_fs::probe_leaf_exists;
use crate::secure_fs::rename_noreplace;
use crate::secure_fs::require_claim_identity;
use crate::secure_fs::require_directory_identity;
use crate::secure_fs::same_directory_identity;
use crate::secure_fs::validate_absolute_store_root;
use crate::secure_fs::validate_leaf;

const STORE_IDENTITY_DOMAIN: &[u8] = b"hepta.mnl.replay-store-identity.v1\0";
const MAX_RECORD_BYTES: usize = 256 * 1024;

/// Opaque policy required to reach filesystem operations.
///
/// There is deliberately no public constructor. Production construction is
/// absent, while unit tests can construct a scoped policy inside this crate.
#[derive(Debug)]
pub struct ReplayStorePolicyV1 {
    expected_gid: u32,
    expected_uid: u32,
    namespace_leaf: String,
    root_path: PathBuf,
}

/// Retained, identity-checked replay-store descriptors.
#[derive(Debug)]
pub struct ReplayStoreAnchorV1 {
    expected_gid: u32,
    expected_uid: u32,
    identity_sha256: String,
    namespace: OwnedFd,
    namespace_identity: FileIdentityV1,
    namespace_leaf: String,
    root: OwnedFd,
    root_identity: FileIdentityV1,
    root_path: PathBuf,
}

/// Opaque observation that exact claim bytes were atomically published and
/// read back through retained descriptors.
///
/// This token is neither a freshness grant nor live runner authority. Full
/// crash/reboot qualification and the production policy remain absent.
#[derive(Debug)]
pub struct DurableReplayPublicationInspectionV1 {
    final_file: File,
    final_leaf_name: String,
    full_binding_sha256: String,
    namespace: OwnedFd,
    replay_slot_sha256: String,
    root: OwnedFd,
    store_identity_sha256: String,
}

#[derive(Debug)]
struct ReplayRecordMaterialV1<'a> {
    final_leaf_name: &'a str,
    full_binding_sha256: &'a str,
    record_bytes: &'a [u8],
    replay_slot_sha256: &'a str,
    store_identity_sha256: &'a str,
}

impl ReplayStoreAnchorV1 {
    pub fn identity_sha256(&self) -> &str {
        &self.identity_sha256
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }
}

impl DurableReplayPublicationInspectionV1 {
    pub fn final_leaf_name(&self) -> &str {
        &self.final_leaf_name
    }

    pub fn full_binding_sha256(&self) -> &str {
        &self.full_binding_sha256
    }

    pub fn replay_slot_sha256(&self) -> &str {
        &self.replay_slot_sha256
    }

    pub fn store_identity_sha256(&self) -> &str {
        &self.store_identity_sha256
    }

    pub const fn exact_publication_read_back_observed(&self) -> bool {
        true
    }

    pub const fn crash_reboot_qualified(&self) -> bool {
        false
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }

    pub fn retained_descriptor_count(&self) -> usize {
        let _ = (&self.root, &self.namespace, &self.final_file);
        3
    }
}

pub fn open_production_replay_store() -> ReplayStoreResultV1<ReplayStoreAnchorV1> {
    let policy = compiled_production_replay_store_policy().ok_or(ReplayStoreErrorV1::Blocked(
        "compiled production replay-store policy is absent",
    ))?;
    open_replay_store(&policy)
}

pub fn open_replay_store(policy: &ReplayStorePolicyV1) -> ReplayStoreResultV1<ReplayStoreAnchorV1> {
    let root_path = validate_absolute_store_root(&policy.root_path)?;
    validate_leaf(&policy.namespace_leaf, "replay-store namespace")?;
    let root = open_root_directory(&root_path)?;
    let root_identity = identity_for_fd(&root)?;
    require_directory_identity(
        root_identity,
        policy.expected_uid,
        policy.expected_gid,
        "replay-store root",
    )?;
    let namespace = open_child_directory(&root, &policy.namespace_leaf)?;
    let namespace_identity = identity_for_fd(&namespace)?;
    require_directory_identity(
        namespace_identity,
        policy.expected_uid,
        policy.expected_gid,
        "replay-store namespace",
    )?;
    if root_identity.device != namespace_identity.device {
        return Err(ReplayStoreErrorV1::IdentityMismatch(
            "replay-store namespace crossed a mount below its retained root".to_string(),
        ));
    }
    let identity_sha256 = derive_store_identity_sha256(
        &root_path,
        &policy.namespace_leaf,
        root_identity,
        namespace_identity,
    )?;
    Ok(ReplayStoreAnchorV1 {
        expected_gid: policy.expected_gid,
        expected_uid: policy.expected_uid,
        identity_sha256,
        namespace,
        namespace_identity,
        namespace_leaf: policy.namespace_leaf.clone(),
        root,
        root_identity,
        root_path,
    })
}

pub fn publish_pre_run_claim_once(
    store: &ReplayStoreAnchorV1,
    claim: &PreparedPreRunReplayClaimV1,
) -> ReplayStoreResultV1<DurableReplayPublicationInspectionV1> {
    publish_record_once(
        store,
        ReplayRecordMaterialV1 {
            final_leaf_name: claim.final_leaf_name(),
            full_binding_sha256: claim.full_binding_sha256(),
            record_bytes: claim.record_bytes(),
            replay_slot_sha256: claim.replay_slot_sha256(),
            store_identity_sha256: claim.pre_run_replay_store_identity_sha256(),
        },
    )
}

pub fn publish_copy_ack_claim_once(
    store: &ReplayStoreAnchorV1,
    claim: &PreparedCopyAckReplayClaimV1,
) -> ReplayStoreResultV1<DurableReplayPublicationInspectionV1> {
    publish_record_once(
        store,
        ReplayRecordMaterialV1 {
            final_leaf_name: claim.final_leaf_name(),
            full_binding_sha256: claim.full_binding_sha256(),
            record_bytes: claim.record_bytes(),
            replay_slot_sha256: claim.replay_slot_sha256(),
            store_identity_sha256: claim.copy_replay_store_identity_sha256(),
        },
    )
}

fn compiled_production_replay_store_policy() -> Option<ReplayStorePolicyV1> {
    None
}

fn publish_record_once(
    store: &ReplayStoreAnchorV1,
    material: ReplayRecordMaterialV1<'_>,
) -> ReplayStoreResultV1<DurableReplayPublicationInspectionV1> {
    validate_record_material(store, &material)?;
    revalidate_store_anchor(store)?;

    let incoming_leaf = format!("{}.incoming-v1", material.replay_slot_sha256);
    if probe_leaf_exists(&store.namespace, material.final_leaf_name) {
        return Err(ReplayStoreErrorV1::ExistingFinalBlocksReplay);
    }
    if probe_leaf_exists(&store.namespace, &incoming_leaf) {
        return Err(ReplayStoreErrorV1::IncomingResidueBlocks);
    }

    let mut incoming = crate::secure_fs::create_claim_file(&store.namespace, &incoming_leaf)
        .map_err(|error| {
            if probe_leaf_exists(&store.namespace, material.final_leaf_name) {
                ReplayStoreErrorV1::ExistingFinalBlocksReplay
            } else if probe_leaf_exists(&store.namespace, &incoming_leaf) {
                ReplayStoreErrorV1::IncomingResidueBlocks
            } else {
                error
            }
        })?;
    incoming
        .write_all(material.record_bytes)
        .map_err(|error| io_error("write exact replay claim bytes", error))?;
    incoming
        .flush()
        .map_err(|error| io_error("flush replay claim bytes", error))?;
    incoming
        .sync_all()
        .map_err(|error| io_error("fsync replay claim file", error))?;
    let incoming_identity = identity_for_fd(&incoming)?;
    require_claim_identity(
        incoming_identity,
        store.expected_uid,
        store.expected_gid,
        material.record_bytes.len(),
        "incoming replay claim",
    )?;

    if let Err(error) = rename_noreplace(&store.namespace, &incoming_leaf, material.final_leaf_name)
    {
        if probe_leaf_exists(&store.namespace, material.final_leaf_name) {
            return Err(ReplayStoreErrorV1::ExistingFinalBlocksReplay);
        }
        return Err(ReplayStoreErrorV1::RaceDetected(format!(
            "atomic no-replace publication failed and incoming state remains: {error}",
        )));
    }
    fsync_directory(&store.namespace)?;

    let mut final_file = open_claim_file(&store.namespace, material.final_leaf_name)?;
    let final_identity = identity_for_fd(&final_file)?;
    require_claim_identity(
        final_identity,
        store.expected_uid,
        store.expected_gid,
        material.record_bytes.len(),
        "read-back replay claim",
    )?;
    if final_identity != incoming_identity {
        return Err(ReplayStoreErrorV1::RaceDetected(
            "read-back claim identity differs from the atomically renamed inode".to_string(),
        ));
    }
    let read_limit = u64::try_from(material.record_bytes.len())
        .map_err(|_| invalid("replay claim byte length is not representable"))?
        .checked_add(1)
        .ok_or_else(|| invalid("replay claim read bound overflow"))?;
    let mut read_back = Vec::with_capacity(material.record_bytes.len());
    Read::by_ref(&mut final_file)
        .take(read_limit)
        .read_to_end(&mut read_back)
        .map_err(|error| io_error("read back exact replay claim bytes", error))?;
    if read_back != material.record_bytes || sha256_hex(&read_back) != material.full_binding_sha256
    {
        return Err(ReplayStoreErrorV1::IdentityMismatch(
            "read-back replay claim bytes or digest differ from the prepared record".to_string(),
        ));
    }
    revalidate_store_anchor(store)?;
    let named_final = open_claim_file(&store.namespace, material.final_leaf_name)?;
    let named_final_identity = identity_for_fd(&named_final)?;
    if named_final_identity != final_identity {
        return Err(ReplayStoreErrorV1::RaceDetected(
            "canonical final replay claim no longer names the read-back inode".to_string(),
        ));
    }
    drop(named_final);

    let root = rustix::io::dup(&store.root)
        .map_err(|error| crate::error::syscall_error("duplicate retained replay root", error))?;
    let namespace = rustix::io::dup(&store.namespace).map_err(|error| {
        crate::error::syscall_error("duplicate retained replay namespace", error)
    })?;
    Ok(DurableReplayPublicationInspectionV1 {
        final_file,
        final_leaf_name: material.final_leaf_name.to_string(),
        full_binding_sha256: material.full_binding_sha256.to_string(),
        namespace,
        replay_slot_sha256: material.replay_slot_sha256.to_string(),
        root,
        store_identity_sha256: store.identity_sha256.clone(),
    })
}

fn validate_record_material(
    store: &ReplayStoreAnchorV1,
    material: &ReplayRecordMaterialV1<'_>,
) -> ReplayStoreResultV1<()> {
    validate_sha256(material.replay_slot_sha256, "replay slot")?;
    validate_sha256(material.full_binding_sha256, "full replay binding")?;
    validate_sha256(material.store_identity_sha256, "replay-store identity")?;
    if material.store_identity_sha256 != store.identity_sha256 {
        return Err(ReplayStoreErrorV1::IdentityMismatch(
            "prepared claim targets a different replay-store identity".to_string(),
        ));
    }
    if material.final_leaf_name != format!("{}.claim-v1", material.replay_slot_sha256) {
        return Err(invalid("prepared replay final leaf is not deterministic"));
    }
    if material.record_bytes.is_empty() || material.record_bytes.len() > MAX_RECORD_BYTES {
        return Err(invalid("replay record byte length is outside its bound"));
    }
    if sha256_hex(material.record_bytes) != material.full_binding_sha256 {
        return Err(invalid(
            "prepared replay record digest differs from its exact bytes",
        ));
    }
    Ok(())
}

fn revalidate_store_anchor(store: &ReplayStoreAnchorV1) -> ReplayStoreResultV1<()> {
    let retained_root = identity_for_fd(&store.root)?;
    let retained_namespace = identity_for_fd(&store.namespace)?;
    require_directory_identity(
        retained_root,
        store.expected_uid,
        store.expected_gid,
        "retained replay-store root",
    )?;
    require_directory_identity(
        retained_namespace,
        store.expected_uid,
        store.expected_gid,
        "retained replay-store namespace",
    )?;
    if !same_directory_identity(retained_root, store.root_identity)
        || !same_directory_identity(retained_namespace, store.namespace_identity)
    {
        return Err(ReplayStoreErrorV1::RaceDetected(
            "retained replay-store descriptor identity drifted".to_string(),
        ));
    }
    let reopened_root = open_root_directory(&store.root_path)?;
    let reopened_root_identity = identity_for_fd(&reopened_root)?;
    let reopened_namespace = open_child_directory(&reopened_root, &store.namespace_leaf)?;
    let reopened_namespace_identity = identity_for_fd(&reopened_namespace)?;
    if !same_directory_identity(reopened_root_identity, store.root_identity)
        || !same_directory_identity(reopened_namespace_identity, store.namespace_identity)
    {
        return Err(ReplayStoreErrorV1::RaceDetected(
            "canonical replay-store path no longer names the retained identity".to_string(),
        ));
    }
    Ok(())
}

fn derive_store_identity_sha256(
    root_path: &std::path::Path,
    namespace_leaf: &str,
    root_identity: FileIdentityV1,
    namespace_identity: FileIdentityV1,
) -> ReplayStoreResultV1<String> {
    let mut frame = Vec::new();
    frame.extend_from_slice(STORE_IDENTITY_DOMAIN);
    append_length_prefixed(&mut frame, root_path.as_os_str().as_bytes())?;
    append_length_prefixed(&mut frame, namespace_leaf.as_bytes())?;
    append_identity(&mut frame, root_identity);
    append_identity(&mut frame, namespace_identity);
    Ok(sha256_hex(&frame))
}

fn append_identity(output: &mut Vec<u8>, identity: FileIdentityV1) {
    output.extend_from_slice(&identity.device.to_be_bytes());
    output.extend_from_slice(&identity.inode.to_be_bytes());
    output.extend_from_slice(&identity.uid.to_be_bytes());
    output.extend_from_slice(&identity.gid.to_be_bytes());
    output.extend_from_slice(&identity.mode.to_be_bytes());
    output.extend_from_slice(&identity.link_count.to_be_bytes());
}

fn append_length_prefixed(output: &mut Vec<u8>, value: &[u8]) -> ReplayStoreResultV1<()> {
    let length = u64::try_from(value.len())
        .map_err(|_| invalid("replay-store identity field length is not representable"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn validate_sha256(value: &str, label: &str) -> ReplayStoreResultV1<()> {
    if value.len() != 64
        || value.bytes().all(|byte| byte == b'0')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!("{label} is not canonical SHA-256")));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

#[cfg(test)]
impl ReplayStorePolicyV1 {
    pub(crate) fn for_tests(
        root_path: PathBuf,
        namespace_leaf: impl Into<String>,
        expected_uid: u32,
        expected_gid: u32,
    ) -> Self {
        Self {
            expected_gid,
            expected_uid,
            namespace_leaf: namespace_leaf.into(),
            root_path,
        }
    }
}

#[cfg(test)]
pub(crate) fn publish_material_for_tests(
    store: &ReplayStoreAnchorV1,
    replay_slot_sha256: &str,
    record_bytes: &[u8],
    store_identity_sha256: &str,
) -> ReplayStoreResultV1<DurableReplayPublicationInspectionV1> {
    let full_binding_sha256 = sha256_hex(record_bytes);
    let final_leaf_name = format!("{replay_slot_sha256}.claim-v1");
    publish_record_once(
        store,
        ReplayRecordMaterialV1 {
            final_leaf_name: &final_leaf_name,
            full_binding_sha256: &full_binding_sha256,
            record_bytes,
            replay_slot_sha256,
            store_identity_sha256,
        },
    )
}
