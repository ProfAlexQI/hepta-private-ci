//! Darwin-native, privilege-separated publication and admission-barrier core.
//!
//! This module is deliberately not an installer or a launchd service.  Its
//! live constructors require an already installed `root:wheel` helper, an
//! already provisioned producer-unwritable namespace, and an authenticated
//! connected Unix-domain socket.  Rootless constructors exist only under
//! `cfg(test)` and every rootless receipt says that it grants no authority.
//!
//! The implementation avoids pathname-only publication.  It keeps the
//! namespace directory descriptor open, traverses descendants with
//! `openat(O_NOFOLLOW)` and `fstatat(AT_SYMLINK_NOFOLLOW)`, publishes with
//! `renameatx_np(RENAME_EXCL)`, reopens the final child, compares the exact
//! `(dev, inode)` pair, and replays the complete tree before returning.
//!
//! The barrier journal is not an advisory file lock.  In a live profile it is
//! writable only by the root helper.  Its state machine will not release until
//! source isolation, zero-handle proof, snapshot, canary, cutover, rollback,
//! fresh re-isolation, a fresh snapshot, and recutover have all been durably
//! chained.  Moving the source into the root-only quarantine and scanning all
//! outstanding descriptors/maps/fileports are separate privileged ceremony
//! steps; callers cannot replace those facts with booleans that skip phases.

use std::collections::BTreeSet;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::RawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::path::Component;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::AcceptanceError;
use crate::durable::MAX_ARTIFACT_BYTES;
use crate::durable::canonical_json;
use crate::durable::sha256;

const RENAME_EXCL: libc::c_uint = 0x0000_0004;
const SOL_LOCAL: libc::c_int = 0;
const LOCAL_PEERPID: libc::c_int = 0x002;
const LOCAL_PEERTOKEN: libc::c_int = 0x006;
const MAX_NODES: usize = 65_536;
const MAX_DEPTH: usize = 64;
const RECORD_SUFFIX: &str = ".json";
const RECORD_DIGITS: usize = 20;
const RECOVERY_RECORD_NAME: &str = "recovery-terminal.json";
const LIVE_PRODUCER_NAME: &str = "_hepta";
const LIVE_PRODUCER_UID: u32 = 499;
const LIVE_PRODUCER_GID: u32 = 499;
const LIVE_OPERATOR_UID: u32 = 501;
const LIVE_LEGACY_WRITER_UID: u32 = 501;
const LIVE_INSTALL_CONFIG_PATH: &str =
    "/Library/PrivilegedHelperTools/hepta-privileged-broker-policy-v1.json";
const LIVE_T5_ROOT_PREFIX: &str = ".hepta-privileged-qualification-v1-";
const T5_VOLUME_UUID: [u8; 16] = [
    0xfb, 0x80, 0x4d, 0x1b, 0x24, 0xcb, 0x4d, 0x6e, 0xae, 0xa7, 0xa9, 0xe1, 0x80, 0x80, 0x77, 0x58,
];
const ATTR_BIT_MAP_COUNT: u16 = 5;
const ATTR_VOL_UUID: u32 = 0x0004_0000;
const ATTR_VOL_INFO: u32 = 0x8000_0000;
const MNT_IGNORE_OWNERSHIP: u64 = 0x0020_0000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QualificationMode {
    Live,
    MechanismOnly,
}

#[derive(Clone, Debug)]
pub struct NamespacePolicy {
    broker_uid: u32,
    broker_gid: u32,
    legacy_writer_uid: u32,
    operator_uid: u32,
    client_executable_path: String,
    client_executable_sha256: String,
    helper_executable_path: String,
    helper_executable_sha256: String,
    producer_account_sha256: String,
    producer_uid: u32,
    producer_groups: BTreeSet<u32>,
    mode: QualificationMode,
}

impl NamespacePolicy {
    /// Construct the only policy accepted for live qualification.
    pub fn live() -> Result<Self, AcceptanceError> {
        if unsafe { libc::geteuid() } != 0 || unsafe { libc::getegid() } != 0 {
            return Err(invalid(
                "live privilege-broker policy requires effective root:wheel",
            ));
        }
        let config_file = open_trusted_install_regular(
            Path::new(LIVE_INSTALL_CONFIG_PATH),
            0o444,
            "live broker install config",
        )?;
        let config_bytes = read_open_file(&config_file)?;
        let config: InstalledBrokerPolicyV1 =
            serde_json::from_slice(&config_bytes).map_err(|error| {
                invalid(format!("live broker install config is malformed: {error}"))
            })?;
        if canonical_json(&config)? != config_bytes {
            return Err(invalid(
                "live broker install config is not exact canonical JSON",
            ));
        }
        config.validate()?;

        let producer = resolve_producer_account(LIVE_PRODUCER_UID)?;
        if LIVE_OPERATOR_UID == LIVE_PRODUCER_UID
            || LIVE_LEGACY_WRITER_UID == LIVE_PRODUCER_UID
            || producer.groups.contains(&0)
            || producer.name != LIVE_PRODUCER_NAME
            || producer.uid != LIVE_PRODUCER_UID
            || producer.gid != LIVE_PRODUCER_GID
            || producer.home != "/var/empty"
            || !matches!(producer.shell.as_str(), "/usr/bin/false" | "/bin/false")
        {
            return Err(invalid(
                "dedicated producer account identity, shell, home, or wheel exclusion is invalid",
            ));
        }
        let helper = open_trusted_install_regular(
            Path::new(&config.helper_executable_path),
            0o555,
            "installed broker helper",
        )?;
        let helper_snapshot = snapshot_fd(helper.as_raw_fd(), "installed broker helper")?;
        let helper_sha256 = hash_open_file(&helper, helper_snapshot.size())?;
        let current_executable = std::env::current_exe()?;
        if current_executable.as_os_str().as_bytes() != config.helper_executable_path.as_bytes()
            || helper_sha256 != config.helper_executable_sha256
        {
            return Err(invalid(
                "running helper path or bytes differ from the fixed root-owned install config",
            ));
        }
        let client = open_trusted_install_regular(
            Path::new(&config.client_executable_path),
            0o555,
            "installed broker client",
        )?;
        let client_snapshot = snapshot_fd(client.as_raw_fd(), "installed broker client")?;
        let client_sha256 = hash_open_file(&client, client_snapshot.size())?;
        if client_sha256 != config.client_executable_sha256 {
            return Err(invalid(
                "installed client bytes differ from the fixed root-owned install config",
            ));
        }
        Ok(Self {
            broker_uid: 0,
            broker_gid: 0,
            legacy_writer_uid: LIVE_LEGACY_WRITER_UID,
            operator_uid: LIVE_OPERATOR_UID,
            client_executable_path: config.client_executable_path,
            client_executable_sha256: config.client_executable_sha256,
            helper_executable_path: config.helper_executable_path,
            helper_executable_sha256: config.helper_executable_sha256,
            producer_account_sha256: producer.sha256()?,
            producer_uid: LIVE_PRODUCER_UID,
            producer_groups: producer.groups,
            mode: QualificationMode::Live,
        })
    }

    #[cfg(test)]
    fn rootless_test() -> Self {
        Self::mechanism_only_current_user().expect("rootless test policy")
    }

    /// Construct an explicitly non-authoritative policy for disposable
    /// mechanism qualification.  It cannot emit live authority even if the
    /// caller later gains privilege.
    pub fn mechanism_only_current_user() -> Result<Self, AcceptanceError> {
        let uid = unsafe { libc::geteuid() };
        let gid = unsafe { libc::getegid() };
        if uid == 0 || gid == 0 {
            return Err(invalid(
                "mechanism-only current-user policy refuses privileged execution",
            ));
        }
        let current_executable = std::env::current_exe()?;
        let current_executable_string = current_executable
            .to_str()
            .ok_or_else(|| invalid("mechanism-only executable path is not UTF-8"))?
            .to_string();
        let executable = open_absolute_regular(&current_executable)?;
        let executable_snapshot = snapshot_fd(executable.as_raw_fd(), "mechanism-only helper")?;
        let executable_sha256 = hash_open_file(&executable, executable_snapshot.size())?;
        Ok(Self {
            broker_uid: uid,
            broker_gid: gid,
            legacy_writer_uid: uid,
            operator_uid: uid,
            client_executable_path: current_executable_string.clone(),
            client_executable_sha256: executable_sha256.clone(),
            helper_executable_path: current_executable_string,
            helper_executable_sha256: executable_sha256,
            producer_account_sha256: "0".repeat(64),
            producer_uid: uid.saturating_add(1),
            producer_groups: BTreeSet::new(),
            mode: QualificationMode::MechanismOnly,
        })
    }

    fn privileged_mode(&self) -> bool {
        self.mode == QualificationMode::Live
    }

    pub(crate) fn is_live_authority(&self) -> bool {
        false
    }

    pub(crate) fn is_privileged_qualification_mode(&self) -> bool {
        self.privileged_mode()
    }

    pub(crate) fn helper_executable_path(&self) -> &str {
        &self.helper_executable_path
    }

    pub(crate) fn helper_executable_sha256(&self) -> &str {
        &self.helper_executable_sha256
    }

    pub(crate) fn validates_authenticated_peer(&self, peer: &AuthenticatedPeerV1) -> bool {
        peer.effective_uid == self.operator_uid
            && peer.executable_path == self.client_executable_path
            && peer.executable_sha256 == self.client_executable_sha256
            && peer.audit_session_id != 0
            && peer.pid > 1
            && peer.pid_version >= 0
            && require_digest(&peer.audit_token_sha256, "policy peer audit token").is_ok()
    }

    pub(crate) fn target_producer_uid(&self) -> u32 {
        self.producer_uid
    }

    pub(crate) fn target_producer_gid(&self) -> u32 {
        self.producer_groups
            .iter()
            .copied()
            .find(|gid| *gid == LIVE_PRODUCER_GID)
            .unwrap_or(LIVE_PRODUCER_GID)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct InstalledBrokerPolicyV1 {
    client_executable_path: String,
    client_executable_sha256: String,
    helper_executable_path: String,
    helper_executable_sha256: String,
    legacy_writer_uid: u32,
    operator_uid: u32,
    producer_gid: u32,
    producer_uid: u32,
    schema: String,
}

impl InstalledBrokerPolicyV1 {
    fn validate(&self) -> Result<(), AcceptanceError> {
        require_digest(
            &self.client_executable_sha256,
            "configured client executable",
        )?;
        require_digest(
            &self.helper_executable_sha256,
            "configured helper executable",
        )?;
        if self.schema != "hepta_mac_privileged_broker_install_policy_v1"
            || self.operator_uid != LIVE_OPERATOR_UID
            || self.legacy_writer_uid != LIVE_LEGACY_WRITER_UID
            || self.producer_uid != LIVE_PRODUCER_UID
            || self.producer_gid != LIVE_PRODUCER_GID
            || !Path::new(&self.client_executable_path).is_absolute()
            || !Path::new(&self.helper_executable_path).is_absolute()
            || self.client_executable_path == self.helper_executable_path
        {
            return Err(invalid(
                "fixed live install policy has invalid schema, identities, or executable paths",
            ));
        }
        Ok(())
    }
}

/// Flush and replay a prepared tree for an external digest pin.  This is
/// exposed only to the disposable mechanism harness; live publication still
/// repeats every check under the root broker.
pub fn prepared_tree_replay_sha256(
    path: &Path,
    policy: &NamespacePolicy,
) -> Result<String, AcceptanceError> {
    let root = open_absolute_directory(path)?;
    let snapshot = snapshot_fd(root.as_raw_fd(), ".")?;
    verify_broker_owned_tree_root(&snapshot, policy)?;
    sync_open_tree(root.as_raw_fd())?;
    let replay = replay_tree(root.as_raw_fd())?;
    verify_broker_owned_replay(&replay, policy)?;
    replay.digest()
}

struct OpenedNamespace {
    namespace: File,
    /// Existing ancestors, ordered from the namespace parent back to `/`.
    /// Keeping every descriptor alive prevents later path re-resolution from
    /// silently switching the durability domain.
    durability_ancestors: Vec<File>,
}

impl OpenedNamespace {
    fn sync_durability_chain(&self) -> Result<(), AcceptanceError> {
        self.namespace.sync_all()?;
        for ancestor in &self.durability_ancestors {
            ancestor.sync_all()?;
        }
        Ok(())
    }
}

#[repr(C)]
struct AttrList {
    bitmapcount: u16,
    reserved: u16,
    commonattr: u32,
    volattr: u32,
    dirattr: u32,
    fileattr: u32,
    forkattr: u32,
}

#[repr(C)]
struct VolumeUuidBuffer {
    length: u32,
    uuid: [u8; 16],
}

#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
struct ProducerAccountV1 {
    gid: u32,
    groups: BTreeSet<u32>,
    home: String,
    name: String,
    schema: String,
    shell: String,
    uid: u32,
}

impl ProducerAccountV1 {
    fn sha256(&self) -> Result<String, AcceptanceError> {
        Ok(sha256(&canonical_json(self)?))
    }
}

fn resolve_producer_account(uid: u32) -> Result<ProducerAccountV1, AcceptanceError> {
    let mut buffer_size = unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) };
    if buffer_size < 1 {
        buffer_size = 16_384;
    }
    if buffer_size > 1_048_576 {
        return Err(invalid("passwd lookup buffer bound is unreasonable"));
    }
    let mut buffer = vec![0_u8; buffer_size as usize];
    let mut password: libc::passwd = unsafe { std::mem::zeroed() };
    let mut result = std::ptr::null_mut();
    let rc = unsafe {
        libc::getpwuid_r(
            uid,
            &mut password,
            buffer.as_mut_ptr().cast(),
            buffer.len(),
            &mut result,
        )
    };
    if rc != 0 {
        return Err(std::io::Error::from_raw_os_error(rc).into());
    }
    if result.is_null() {
        return Err(invalid("dedicated producer UID has no passwd record"));
    }
    let field = |pointer: *const libc::c_char, label: &str| -> Result<String, AcceptanceError> {
        if pointer.is_null() {
            return Err(invalid(format!("producer passwd {label} is null")));
        }
        unsafe { CStr::from_ptr(pointer) }
            .to_str()
            .map(str::to_owned)
            .map_err(|_| invalid(format!("producer passwd {label} is not UTF-8")))
    };
    let name = field(password.pw_name, "name")?;
    let home = field(password.pw_dir, "home")?;
    let shell = field(password.pw_shell, "shell")?;
    if name.is_empty() || name.contains('\0') {
        return Err(invalid("dedicated producer account name is malformed"));
    }

    let name_c = CString::new(name.as_bytes())
        .map_err(|_| invalid("dedicated producer account name contains NUL"))?;
    let primary_gid = libc::c_int::try_from(password.pw_gid)
        .map_err(|_| invalid("producer primary GID exceeds getgrouplist ABI"))?;
    let mut count: libc::c_int = 32;
    let mut groups = vec![0 as libc::c_int; count as usize];
    let mut groups_rc = unsafe {
        libc::getgrouplist(
            name_c.as_ptr(),
            primary_gid,
            groups.as_mut_ptr(),
            &mut count,
        )
    };
    if groups_rc < 0 {
        if count <= 0 || count > 1024 {
            return Err(invalid("producer supplementary group count is invalid"));
        }
        groups.resize(count as usize, 0);
        groups_rc = unsafe {
            libc::getgrouplist(
                name_c.as_ptr(),
                primary_gid,
                groups.as_mut_ptr(),
                &mut count,
            )
        };
    }
    if groups_rc < 0 || count <= 0 || count as usize > groups.len() {
        return Err(invalid("producer supplementary groups could not be closed"));
    }
    groups.truncate(count as usize);
    let groups = groups
        .into_iter()
        .map(|group| {
            u32::try_from(group).map_err(|_| invalid("producer supplementary GID is negative"))
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if !groups.contains(&password.pw_gid) {
        return Err(invalid("producer group closure omits its primary group"));
    }
    Ok(ProducerAccountV1 {
        gid: password.pw_gid,
        groups,
        home,
        name,
        schema: "hepta_mac_dedicated_producer_account_v1".to_string(),
        shell,
        uid,
    })
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AuditToken {
    val: [u32; 8],
}

#[derive(Clone, Debug)]
pub struct AuthenticatedPeerV1 {
    audit_session_id: u32,
    audit_token_sha256: String,
    effective_gid: u32,
    effective_uid: u32,
    executable_path: String,
    executable_sha256: String,
    pid: i32,
    pid_version: i32,
}

/// Authenticate one already-connected Unix-domain peer using kernel facts.
///
/// The caller-supplied request contains no identity fields.  UID/GID/PID are
/// decoded from `LOCAL_PEERTOKEN`, then cross-checked against `getpeereid` and
/// `LOCAL_PEERPID`.  The peer executable must be installed in a root-owned,
/// producer-unwritable path for a live result; its exact bytes are externally
/// pinned because this host currently has no usable code-signing identity.
pub fn authenticate_connected_peer(
    socket_fd: RawFd,
    policy: &NamespacePolicy,
) -> Result<AuthenticatedPeerV1, AcceptanceError> {
    require_digest(&policy.client_executable_sha256, "policy client executable")?;
    let token = peer_audit_token(socket_fd)?;
    let token_uid = unsafe { audit_token_to_euid(token) };
    let token_gid = unsafe { audit_token_to_egid(token) };
    let token_pid = unsafe { audit_token_to_pid(token) };
    let token_asid = unsafe { audit_token_to_asid(token) };
    let token_pid_version = unsafe { audit_token_to_pidversion(token) };

    let mut peer_uid = 0;
    let mut peer_gid = 0;
    if unsafe { libc::getpeereid(socket_fd, &mut peer_uid, &mut peer_gid) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let peer_pid = peer_pid(socket_fd)?;
    if token_uid != peer_uid
        || token_gid != peer_gid
        || token_pid != peer_pid
        || token_uid != policy.operator_uid
    {
        return Err(invalid(
            "kernel peer credential and audit-token identities do not match the external pin",
        ));
    }

    let executable_path = executable_path_for_token(token)?;
    if executable_path.as_bytes() != policy.client_executable_path.as_bytes() {
        return Err(invalid(
            "connected peer executable path differs from the installed policy",
        ));
    }
    let executable = if policy.privileged_mode() {
        open_trusted_install_regular(
            Path::new(&executable_path),
            0o555,
            "connected peer executable",
        )?
    } else {
        open_absolute_regular(Path::new(&executable_path))?
    };
    let executable_snapshot = snapshot_fd(executable.as_raw_fd(), &executable_path)?;
    if executable_snapshot.kind != NodeKind::RegularFile || executable_snapshot.nlink() != 1 {
        return Err(invalid(
            "installed client must be one regular unaliased file",
        ));
    }
    let executable_sha256 = hash_open_file(&executable, executable_snapshot.size())?;
    if executable_sha256 != policy.client_executable_sha256 {
        return Err(invalid(
            "connected peer executable differs from its exact byte pin",
        ));
    }

    Ok(AuthenticatedPeerV1 {
        audit_session_id: token_asid,
        audit_token_sha256: audit_token_sha256(token),
        effective_gid: token_gid,
        effective_uid: token_uid,
        executable_path,
        executable_sha256,
        pid: token_pid,
        pid_version: token_pid_version,
    })
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum NodeKind {
    Directory,
    RegularFile,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectBindingV1 {
    pub ctime_nanoseconds: i64,
    pub ctime_seconds: i64,
    pub dev: u64,
    pub flags: u32,
    pub gid: u32,
    pub inode: u64,
    pub mode: u32,
    pub mtime_nanoseconds: i64,
    pub mtime_seconds: i64,
    pub nlink: u64,
    pub size: u64,
    pub uid: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Snapshot {
    binding: ObjectBindingV1,
    kind: NodeKind,
}

impl Snapshot {
    fn dev(&self) -> u64 {
        self.binding.dev
    }

    fn inode(&self) -> u64 {
        self.binding.inode
    }

    fn mode(&self) -> u32 {
        self.binding.mode
    }

    fn uid(&self) -> u32 {
        self.binding.uid
    }

    fn gid(&self) -> u32 {
        self.binding.gid
    }

    fn nlink(&self) -> u64 {
        self.binding.nlink
    }

    fn size(&self) -> u64 {
        self.binding.size
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct TreeNodeV1 {
    acl_sha256: String,
    binding: ObjectBindingV1,
    content_sha256: Option<String>,
    kind: NodeKind,
    path: String,
    xattrs_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct TreeReplayV1 {
    nodes: Vec<TreeNodeV1>,
    schema: String,
}

impl TreeReplayV1 {
    fn digest(&self) -> Result<String, AcceptanceError> {
        // Darwin updates the moved directory's ctime during rename even though
        // it is the exact same inode.  Exclude only the root ctime from the
        // externally pinned replay identity; all descendant ctimes and every
        // other root field remain bound.
        let mut stable = self.clone();
        let root = stable
            .nodes
            .iter_mut()
            .find(|node| node.path == ".")
            .ok_or_else(|| invalid("tree replay has no root node"))?;
        root.binding.ctime_seconds = 0;
        root.binding.ctime_nanoseconds = 0;
        Ok(sha256(&canonical_json(&stable)?))
    }

    fn equivalent_after_root_rename(&self, after: &Self) -> bool {
        let mut expected = self.clone();
        let mut observed = after.clone();
        let expected_root = expected.nodes.iter_mut().find(|node| node.path == ".");
        let observed_root = observed.nodes.iter_mut().find(|node| node.path == ".");
        match (expected_root, observed_root) {
            (Some(expected_root), Some(observed_root)) => {
                expected_root.binding.ctime_seconds = 0;
                expected_root.binding.ctime_nanoseconds = 0;
                observed_root.binding.ctime_seconds = 0;
                observed_root.binding.ctime_nanoseconds = 0;
                expected == observed
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationReceiptV1 {
    pub authority_granted: bool,
    pub final_name: String,
    pub final_root: ObjectBindingV1,
    pub peer_audit_token_sha256: String,
    pub post_publish_replay_sha256: String,
    pub pre_publish_replay_sha256: String,
    pub producer_uid: u32,
    pub producer_account_sha256: String,
    pub publisher_effective_gid: u32,
    pub publisher_effective_uid: u32,
    pub rename_exclusive: bool,
    pub root_owned_producer_unwritable_namespace: bool,
    pub schema: String,
    pub staging_root: ObjectBindingV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct PublicationPreparedV1 {
    client_executable_sha256: String,
    expected_replay_sha256: String,
    final_name: String,
    helper_executable_sha256: String,
    namespace_root: ObjectBindingV1,
    operation_nonce: String,
    peer_audit_token_sha256: String,
    producer_account_sha256: String,
    producer_uid: u32,
    schema: String,
    staging_name: String,
    staging_root: ObjectBindingV1,
    trusted_operator_uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrivilegedQualificationReceiptV1 {
    pub aggregate_authority: bool,
    pub cutover_authority: bool,
    pub deletion_authority: bool,
    pub live_authority: bool,
    pub operation_nonce: String,
    pub prepared_record_sha256: String,
    pub production_authority: bool,
    pub publication_receipt_sha256: String,
    pub recovered_orphan_final: bool,
    pub recovery_challenge_sha256: Option<String>,
    pub recovery_peer_audit_session_id: Option<u32>,
    pub recovery_peer_audit_token_sha256: Option<String>,
    pub refs_authority: bool,
    pub remote_authority: bool,
    pub schema: String,
    pub scope: String,
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct PublicationRecoveryChallengeV1<'a> {
    client_executable_sha256: &'a str,
    final_root: &'a ObjectBindingV1,
    helper_executable_sha256: &'a str,
    operation_nonce: &'a str,
    original_peer_audit_token_sha256: &'a str,
    prepared_record_sha256: &'a str,
    publication_receipt_sha256: &'a str,
    recovery_peer_audit_session_id: u32,
    recovery_peer_audit_token_sha256: &'a str,
    schema: &'static str,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SealedPublicationV1 {
    pub prepared_record_name: String,
    pub prepared_record_sha256: String,
    pub publication_receipt: PublicationReceiptV1,
    pub publication_receipt_name: String,
    pub publication_receipt_sha256: String,
    pub qualification_receipt: PrivilegedQualificationReceiptV1,
    pub qualification_receipt_name: String,
    pub qualification_receipt_sha256: String,
    pub schema: String,
}

/// Crash-consistent entrypoint for a prepared-directory publication.
///
/// A durable prepared record is published before the exclusive rename.  The
/// mechanism receipt and terminal qualification receipt are then published as
/// separate no-replace, fsynced records.  Every record remains explicitly
/// mechanism-only and grants no migration authority.
pub fn qualify_prepared_directory(
    namespace: &Path,
    staging_name: &str,
    final_name: &str,
    expected_replay_sha256: &str,
    operation_nonce: &str,
    peer: &AuthenticatedPeerV1,
    policy: &NamespacePolicy,
) -> Result<SealedPublicationV1, AcceptanceError> {
    let (prepared, prepared_sha256) = prepare_publication(
        namespace,
        staging_name,
        final_name,
        expected_replay_sha256,
        operation_nonce,
        peer,
        policy,
    )?;
    let core = publish_prepared_directory(
        namespace,
        staging_name,
        final_name,
        expected_replay_sha256,
        peer,
        policy,
    )?;
    seal_publication(namespace, &prepared, &prepared_sha256, core, None, policy)
}

/// Recover only a publication whose durable precommit proves that the
/// exclusive rename already completed.  A still-present staging name is never
/// renamed implicitly during recovery.
pub fn recover_prepared_publication(
    namespace: &Path,
    operation_nonce: &str,
    peer: &AuthenticatedPeerV1,
    policy: &NamespacePolicy,
) -> Result<SealedPublicationV1, AcceptanceError> {
    require_nonce(operation_nonce, "publication operation nonce")?;
    let opened_namespace = open_publication_namespace(namespace, policy)?;
    let namespace_fd = &opened_namespace.namespace;
    verify_private_namespace(namespace_fd, policy)?;
    reject_operation_temporary_files(namespace_fd.as_raw_fd(), operation_nonce)?;
    let prepared_name = publication_record_name(operation_nonce, "prepared")?;
    let (prepared, prepared_bytes) = read_canonical_at::<PublicationPreparedV1>(
        namespace_fd.as_raw_fd(),
        &prepared_name,
        "publication prepared record",
    )?;
    validate_prepared(&prepared, operation_nonce, policy)?;
    let prepared_sha256 = sha256(&prepared_bytes);

    let terminal_name = publication_record_name(operation_nonce, "terminal-receipt")?;
    if optional_snapshot(namespace_fd.as_raw_fd(), &terminal_name)?.is_some() {
        return verify_sealed_publication(namespace, operation_nonce, policy);
    }

    let staging = optional_snapshot(namespace_fd.as_raw_fd(), &prepared.staging_name)?;
    let final_snapshot = optional_snapshot(namespace_fd.as_raw_fd(), &prepared.final_name)?;
    match (staging, final_snapshot) {
        (None, Some(final_snapshot)) => {
            if final_snapshot.dev() != prepared.staging_root.dev
                || final_snapshot.inode() != prepared.staging_root.inode
            {
                return Err(invalid(
                    "orphan final is not the exact precommitted staging inode",
                ));
            }
            let final_fd = openat_directory(namespace_fd.as_raw_fd(), &prepared.final_name)?;
            let replay = replay_tree(final_fd.as_raw_fd())?;
            if replay.digest()? != prepared.expected_replay_sha256 {
                return Err(invalid("orphan final differs from the prepared replay pin"));
            }
            validate_fresh_publication_recovery_peer(&prepared, peer, policy)?;
            let receipt = publication_receipt_from_prepared(&prepared, final_snapshot);
            seal_publication(
                namespace,
                &prepared,
                &prepared_sha256,
                receipt,
                Some(peer),
                policy,
            )
        }
        (Some(_), None) => Err(invalid(
            "prepared publication has not renamed; recovery will not repeat rename implicitly",
        )),
        (Some(_), Some(_)) => Err(invalid(
            "prepared publication has both staging and final names; quarantine is required",
        )),
        (None, None) => Err(invalid(
            "prepared publication lost both staging and final names; quarantine is required",
        )),
    }
}

/// Independently verify the full prepared → mechanism → terminal chain and
/// replay the published inode.  A terminal receipt is never accepted alone.
pub fn verify_sealed_publication(
    namespace: &Path,
    operation_nonce: &str,
    policy: &NamespacePolicy,
) -> Result<SealedPublicationV1, AcceptanceError> {
    require_nonce(operation_nonce, "publication operation nonce")?;
    let opened_namespace = open_publication_namespace(namespace, policy)?;
    let namespace_fd = &opened_namespace.namespace;
    verify_private_namespace(namespace_fd, policy)?;
    reject_operation_temporary_files(namespace_fd.as_raw_fd(), operation_nonce)?;
    let prepared_name = publication_record_name(operation_nonce, "prepared")?;
    let (prepared, prepared_bytes) = read_canonical_at::<PublicationPreparedV1>(
        namespace_fd.as_raw_fd(),
        &prepared_name,
        "publication prepared record",
    )?;
    validate_prepared(&prepared, operation_nonce, policy)?;
    let prepared_sha256 = sha256(&prepared_bytes);
    if optional_snapshot(namespace_fd.as_raw_fd(), &prepared.staging_name)?.is_some() {
        return Err(invalid(
            "sealed publication unexpectedly retains its staging name",
        ));
    }
    let final_fd = openat_directory(namespace_fd.as_raw_fd(), &prepared.final_name)?;
    let final_snapshot = snapshot_fd(final_fd.as_raw_fd(), &prepared.final_name)?;
    if final_snapshot.dev() != prepared.staging_root.dev
        || final_snapshot.inode() != prepared.staging_root.inode
        || replay_tree(final_fd.as_raw_fd())?.digest()? != prepared.expected_replay_sha256
    {
        return Err(invalid(
            "sealed publication final inode or descriptor replay is invalid",
        ));
    }

    let publication_name = publication_record_name(operation_nonce, "mechanism-receipt")?;
    let (publication_receipt, publication_bytes) = read_canonical_at::<PublicationReceiptV1>(
        namespace_fd.as_raw_fd(),
        &publication_name,
        "publication mechanism receipt",
    )?;
    let expected_publication = publication_receipt_from_prepared(&prepared, final_snapshot);
    if publication_receipt != expected_publication || publication_receipt.authority_granted {
        return Err(invalid(
            "publication mechanism receipt differs from prepared and final facts",
        ));
    }
    let publication_sha256 = sha256(&publication_bytes);

    let qualification_name = publication_record_name(operation_nonce, "terminal-receipt")?;
    let (qualification_receipt, qualification_bytes) =
        read_canonical_at::<PrivilegedQualificationReceiptV1>(
            namespace_fd.as_raw_fd(),
            &qualification_name,
            "privileged qualification terminal receipt",
        )?;
    let expected_qualification = build_qualification_receipt(
        &prepared,
        &prepared_sha256,
        &publication_receipt,
        &publication_sha256,
        qualification_receipt.recovery_peer_audit_session_id,
        qualification_receipt
            .recovery_peer_audit_token_sha256
            .as_deref(),
    )?;
    if qualification_receipt != expected_qualification {
        return Err(invalid(
            "terminal qualification receipt is not the exact closed-authority chain",
        ));
    }
    Ok(SealedPublicationV1 {
        prepared_record_name: prepared_name,
        prepared_record_sha256: prepared_sha256,
        publication_receipt,
        publication_receipt_name: publication_name,
        publication_receipt_sha256: publication_sha256,
        qualification_receipt,
        qualification_receipt_name: qualification_name,
        qualification_receipt_sha256: sha256(&qualification_bytes),
        schema: "hepta_mac_privileged_sealed_publication_v1".to_string(),
    })
}

fn prepare_publication(
    namespace: &Path,
    staging_name: &str,
    final_name: &str,
    expected_replay_sha256: &str,
    operation_nonce: &str,
    peer: &AuthenticatedPeerV1,
    policy: &NamespacePolicy,
) -> Result<(PublicationPreparedV1, String), AcceptanceError> {
    require_component(staging_name)?;
    require_component(final_name)?;
    require_digest(expected_replay_sha256, "expected tree replay")?;
    require_nonce(operation_nonce, "publication operation nonce")?;
    require_digest(&policy.helper_executable_sha256, "policy helper executable")?;
    if peer.effective_uid != policy.operator_uid {
        return Err(invalid(
            "prepared publication peer is not the trusted operator",
        ));
    }
    let opened_namespace = open_publication_namespace(namespace, policy)?;
    let namespace_fd = &opened_namespace.namespace;
    verify_private_namespace(namespace_fd, policy)?;
    reject_operation_temporary_files(namespace_fd.as_raw_fd(), operation_nonce)?;
    let namespace_root = snapshot_fd(namespace_fd.as_raw_fd(), "namespace")?;
    let staging = openat_directory(namespace_fd.as_raw_fd(), staging_name)?;
    let staging_root = snapshot_fd(staging.as_raw_fd(), staging_name)?;
    verify_broker_owned_tree_root(&staging_root, policy)?;
    sync_open_tree(staging.as_raw_fd())?;
    let replay = replay_tree(staging.as_raw_fd())?;
    verify_broker_owned_replay(&replay, policy)?;
    if replay.digest()? != expected_replay_sha256 {
        return Err(invalid("precommit replay differs from its external pin"));
    }
    if optional_snapshot(namespace_fd.as_raw_fd(), final_name)?.is_some() {
        return Err(invalid(
            "publication final name already exists before precommit",
        ));
    }
    let prepared = PublicationPreparedV1 {
        client_executable_sha256: peer.executable_sha256.clone(),
        expected_replay_sha256: expected_replay_sha256.to_string(),
        final_name: final_name.to_string(),
        helper_executable_sha256: policy.helper_executable_sha256.clone(),
        namespace_root: namespace_root.binding,
        operation_nonce: operation_nonce.to_string(),
        peer_audit_token_sha256: peer.audit_token_sha256.clone(),
        producer_account_sha256: policy.producer_account_sha256.clone(),
        producer_uid: policy.producer_uid,
        schema: "hepta_mac_privileged_publication_prepared_v1".to_string(),
        staging_name: staging_name.to_string(),
        staging_root: staging_root.binding,
        trusted_operator_uid: policy.operator_uid,
    };
    let bytes = canonical_json(&prepared)?;
    let name = publication_record_name(operation_nonce, "prepared")?;
    durable_publish_bytes(
        namespace_fd.as_raw_fd(),
        &publication_temporary_name(operation_nonce, "prepared")?,
        &name,
        &bytes,
    )?;
    opened_namespace.sync_durability_chain()?;
    Ok((prepared, sha256(&bytes)))
}

fn seal_publication(
    namespace: &Path,
    prepared: &PublicationPreparedV1,
    prepared_sha256: &str,
    mut publication_receipt: PublicationReceiptV1,
    recovery_peer: Option<&AuthenticatedPeerV1>,
    policy: &NamespacePolicy,
) -> Result<SealedPublicationV1, AcceptanceError> {
    require_digest(prepared_sha256, "prepared record")?;
    let opened_namespace = open_publication_namespace(namespace, policy)?;
    let namespace_fd = &opened_namespace.namespace;
    publication_receipt.authority_granted = false;
    let publication_bytes = canonical_json(&publication_receipt)?;
    let publication_sha256 = sha256(&publication_bytes);
    let publication_name = publication_record_name(&prepared.operation_nonce, "mechanism-receipt")?;
    persist_or_verify_canonical(
        namespace_fd.as_raw_fd(),
        &publication_temporary_name(&prepared.operation_nonce, "mechanism")?,
        &publication_name,
        &publication_bytes,
    )?;

    let qualification_receipt = build_qualification_receipt(
        prepared,
        prepared_sha256,
        &publication_receipt,
        &publication_sha256,
        recovery_peer.map(|peer| peer.audit_session_id),
        recovery_peer.map(|peer| peer.audit_token_sha256.as_str()),
    )?;
    let qualification_bytes = canonical_json(&qualification_receipt)?;
    let qualification_sha256 = sha256(&qualification_bytes);
    let qualification_name =
        publication_record_name(&prepared.operation_nonce, "terminal-receipt")?;
    persist_or_verify_canonical(
        namespace_fd.as_raw_fd(),
        &publication_temporary_name(&prepared.operation_nonce, "terminal")?,
        &qualification_name,
        &qualification_bytes,
    )?;

    let final_fd = openat_directory(namespace_fd.as_raw_fd(), &prepared.final_name)?;
    let final_snapshot = snapshot_fd(final_fd.as_raw_fd(), &prepared.final_name)?;
    if final_snapshot.dev() != prepared.staging_root.dev
        || final_snapshot.inode() != prepared.staging_root.inode
        || replay_tree(final_fd.as_raw_fd())?.digest()? != prepared.expected_replay_sha256
    {
        return Err(invalid(
            "final publication changed while durable terminal receipt was committed",
        ));
    }
    opened_namespace.sync_durability_chain()?;
    let expected = SealedPublicationV1 {
        prepared_record_name: publication_record_name(&prepared.operation_nonce, "prepared")?,
        prepared_record_sha256: prepared_sha256.to_string(),
        publication_receipt,
        publication_receipt_name: publication_name,
        publication_receipt_sha256: publication_sha256,
        qualification_receipt,
        qualification_receipt_name: qualification_name,
        qualification_receipt_sha256: qualification_sha256,
        schema: "hepta_mac_privileged_sealed_publication_v1".to_string(),
    };
    let verified = verify_sealed_publication(namespace, &prepared.operation_nonce, policy)?;
    if verified != expected {
        return Err(invalid(
            "independent sealed-publication verifier disagrees with committed result",
        ));
    }
    Ok(verified)
}

fn validate_fresh_publication_recovery_peer(
    prepared: &PublicationPreparedV1,
    peer: &AuthenticatedPeerV1,
    policy: &NamespacePolicy,
) -> Result<(), AcceptanceError> {
    if peer.effective_uid != policy.operator_uid
        || peer.executable_sha256 != policy.client_executable_sha256
        || peer.executable_path != policy.client_executable_path
        || peer.audit_token_sha256 == prepared.peer_audit_token_sha256
    {
        return Err(invalid(
            "orphan-final recovery requires a fresh authenticated installed-policy peer",
        ));
    }
    require_digest(
        &peer.audit_token_sha256,
        "fresh publication recovery peer audit token",
    )
}

fn build_qualification_receipt(
    prepared: &PublicationPreparedV1,
    prepared_sha256: &str,
    publication_receipt: &PublicationReceiptV1,
    publication_sha256: &str,
    recovery_peer_audit_session_id: Option<u32>,
    recovery_peer_audit_token_sha256: Option<&str>,
) -> Result<PrivilegedQualificationReceiptV1, AcceptanceError> {
    require_digest(prepared_sha256, "qualification prepared record")?;
    require_digest(publication_sha256, "qualification publication receipt")?;
    let recovery_challenge_sha256 = match (
        recovery_peer_audit_session_id,
        recovery_peer_audit_token_sha256,
    ) {
        (None, None) => None,
        (Some(session), Some(token)) => {
            require_digest(token, "qualification recovery peer audit token")?;
            if token == prepared.peer_audit_token_sha256 {
                return Err(invalid(
                    "publication recovery reused the original peer audit token",
                ));
            }
            Some(sha256(&canonical_json(&PublicationRecoveryChallengeV1 {
                client_executable_sha256: &prepared.client_executable_sha256,
                final_root: &publication_receipt.final_root,
                helper_executable_sha256: &prepared.helper_executable_sha256,
                operation_nonce: &prepared.operation_nonce,
                original_peer_audit_token_sha256: &prepared.peer_audit_token_sha256,
                prepared_record_sha256: prepared_sha256,
                publication_receipt_sha256: publication_sha256,
                recovery_peer_audit_session_id: session,
                recovery_peer_audit_token_sha256: token,
                schema: "hepta_mac_publication_recovery_challenge_v1",
            })?))
        }
        _ => {
            return Err(invalid(
                "publication recovery session and audit token must be present together",
            ));
        }
    };
    Ok(PrivilegedQualificationReceiptV1 {
        aggregate_authority: false,
        cutover_authority: false,
        deletion_authority: false,
        live_authority: false,
        operation_nonce: prepared.operation_nonce.clone(),
        prepared_record_sha256: prepared_sha256.to_string(),
        production_authority: false,
        publication_receipt_sha256: publication_sha256.to_string(),
        recovered_orphan_final: recovery_challenge_sha256.is_some(),
        recovery_challenge_sha256,
        recovery_peer_audit_session_id,
        recovery_peer_audit_token_sha256: recovery_peer_audit_token_sha256.map(str::to_string),
        refs_authority: false,
        remote_authority: false,
        schema: "hepta_mac_privileged_qualification_receipt_v1".to_string(),
        scope: "disposable_privilege_separation_mechanism_only".to_string(),
    })
}

fn publication_receipt_from_prepared(
    prepared: &PublicationPreparedV1,
    final_snapshot: Snapshot,
) -> PublicationReceiptV1 {
    PublicationReceiptV1 {
        authority_granted: false,
        final_name: prepared.final_name.clone(),
        final_root: final_snapshot.binding,
        peer_audit_token_sha256: prepared.peer_audit_token_sha256.clone(),
        post_publish_replay_sha256: prepared.expected_replay_sha256.clone(),
        pre_publish_replay_sha256: prepared.expected_replay_sha256.clone(),
        producer_account_sha256: prepared.producer_account_sha256.clone(),
        producer_uid: prepared.producer_uid,
        publisher_effective_gid: unsafe { libc::getegid() },
        publisher_effective_uid: unsafe { libc::geteuid() },
        rename_exclusive: true,
        root_owned_producer_unwritable_namespace: unsafe { libc::geteuid() } == 0,
        schema: "hepta_mac_privileged_publication_receipt_v1".to_string(),
        staging_root: prepared.staging_root.clone(),
    }
}

/// Publish an already broker-owned staging directory.
///
/// Ingress copy is intentionally outside this function.  A live caller must
/// first descriptor-copy producer input into this root-owned namespace and
/// externally pin the resulting replay digest.  Publishing a directory still
/// writable by the producer is rejected.
fn publish_prepared_directory(
    namespace: &Path,
    staging_name: &str,
    final_name: &str,
    expected_replay_sha256: &str,
    peer: &AuthenticatedPeerV1,
    policy: &NamespacePolicy,
) -> Result<PublicationReceiptV1, AcceptanceError> {
    publish_prepared_directory_with_hook(
        namespace,
        staging_name,
        final_name,
        expected_replay_sha256,
        peer,
        policy,
        || Ok(()),
    )
}

fn publish_prepared_directory_with_hook<F>(
    namespace: &Path,
    staging_name: &str,
    final_name: &str,
    expected_replay_sha256: &str,
    peer: &AuthenticatedPeerV1,
    policy: &NamespacePolicy,
    after_rename: F,
) -> Result<PublicationReceiptV1, AcceptanceError>
where
    F: FnOnce() -> Result<(), AcceptanceError>,
{
    require_component(staging_name)?;
    require_component(final_name)?;
    if staging_name == final_name || !staging_name.starts_with(".incoming-") {
        return Err(invalid(
            "publication child names do not match the fixed protocol",
        ));
    }
    require_digest(expected_replay_sha256, "expected tree replay")?;
    if peer.effective_uid != policy.operator_uid {
        return Err(invalid(
            "authenticated trusted operator differs from namespace policy",
        ));
    }

    let opened_namespace = open_publication_namespace(namespace, policy)?;
    let namespace_fd = &opened_namespace.namespace;
    verify_private_namespace(namespace_fd, policy)?;
    let namespace_pre = snapshot_fd(namespace_fd.as_raw_fd(), ".")?;
    let staging = openat_directory(namespace_fd.as_raw_fd(), staging_name)?;
    let staging_snapshot = snapshot_fd(staging.as_raw_fd(), staging_name)?;
    verify_broker_owned_tree_root(&staging_snapshot, policy)?;
    sync_open_tree(staging.as_raw_fd())?;
    let pre = replay_tree(staging.as_raw_fd())?;
    verify_broker_owned_replay(&pre, policy)?;
    let pre_digest = pre.digest()?;
    if pre_digest != expected_replay_sha256 {
        return Err(invalid(
            "prepared staging replay differs from its external pin",
        ));
    }

    rename_noreplace(
        namespace_fd.as_raw_fd(),
        staging_name,
        namespace_fd.as_raw_fd(),
        final_name,
    )?;
    opened_namespace.sync_durability_chain()?;
    after_rename()?;

    let published = openat_directory(namespace_fd.as_raw_fd(), final_name)?;
    let published_snapshot = snapshot_fd(published.as_raw_fd(), final_name)?;
    if staging_snapshot.dev() != published_snapshot.dev()
        || staging_snapshot.inode() != published_snapshot.inode()
    {
        return Err(invalid(
            "published path is not the exact staging inode after exclusive rename",
        ));
    }
    let post = replay_tree(published.as_raw_fd())?;
    if !pre.equivalent_after_root_rename(&post) {
        return Err(invalid(
            "published tree differs during post-publish full replay",
        ));
    }
    let post_digest = post.digest()?;
    let namespace_post = snapshot_fd(namespace_fd.as_raw_fd(), ".")?;
    if namespace_pre.dev() != namespace_post.dev()
        || namespace_pre.inode() != namespace_post.inode()
    {
        return Err(invalid(
            "publication namespace inode changed during publication",
        ));
    }
    opened_namespace.sync_durability_chain()?;

    Ok(PublicationReceiptV1 {
        authority_granted: false,
        final_name: final_name.to_string(),
        final_root: published_snapshot.binding,
        peer_audit_token_sha256: peer.audit_token_sha256.clone(),
        post_publish_replay_sha256: post_digest,
        pre_publish_replay_sha256: pre_digest,
        producer_uid: policy.producer_uid,
        producer_account_sha256: policy.producer_account_sha256.clone(),
        publisher_effective_gid: unsafe { libc::getegid() },
        publisher_effective_uid: unsafe { libc::geteuid() },
        rename_exclusive: true,
        root_owned_producer_unwritable_namespace: policy.privileged_mode(),
        schema: "hepta_mac_privileged_publication_receipt_v1".to_string(),
        staging_root: staging_snapshot.binding,
    })
}

fn publication_record_name(operation_nonce: &str, kind: &str) -> Result<String, AcceptanceError> {
    require_nonce(operation_nonce, "publication operation nonce")?;
    require_component(kind)?;
    let name = format!("hepta-operation-{operation_nonce}.{kind}.json");
    require_component(&name)?;
    Ok(name)
}

fn publication_temporary_name(
    operation_nonce: &str,
    kind: &str,
) -> Result<String, AcceptanceError> {
    require_nonce(operation_nonce, "publication operation nonce")?;
    require_component(kind)?;
    let name = format!(".incoming-{operation_nonce}-{kind}");
    require_component(&name)?;
    Ok(name)
}

fn reject_operation_temporary_files(
    namespace_fd: RawFd,
    operation_nonce: &str,
) -> Result<(), AcceptanceError> {
    for kind in ["prepared", "mechanism", "terminal"] {
        let name = publication_temporary_name(operation_nonce, kind)?;
        if optional_snapshot(namespace_fd, &name)?.is_some() {
            return Err(invalid(
                "publication has an ambiguous durable temporary record; privileged reconciliation is required",
            ));
        }
    }
    Ok(())
}

fn validate_prepared(
    prepared: &PublicationPreparedV1,
    operation_nonce: &str,
    policy: &NamespacePolicy,
) -> Result<(), AcceptanceError> {
    if prepared.schema != "hepta_mac_privileged_publication_prepared_v1"
        || prepared.operation_nonce != operation_nonce
        || prepared.helper_executable_sha256 != policy.helper_executable_sha256
        || prepared.client_executable_sha256 != policy.client_executable_sha256
        || prepared.trusted_operator_uid != policy.operator_uid
        || prepared.producer_uid != policy.producer_uid
        || prepared.producer_account_sha256 != policy.producer_account_sha256
    {
        return Err(invalid(
            "prepared publication differs from helper, peer, operator, or producer pins",
        ));
    }
    require_component(&prepared.staging_name)?;
    require_component(&prepared.final_name)?;
    require_digest(&prepared.expected_replay_sha256, "prepared tree replay")?;
    require_digest(
        &prepared.helper_executable_sha256,
        "prepared helper executable",
    )?;
    require_digest(
        &prepared.client_executable_sha256,
        "prepared client executable",
    )?;
    require_digest(
        &prepared.producer_account_sha256,
        "prepared producer account",
    )?;
    require_digest(
        &prepared.peer_audit_token_sha256,
        "prepared original peer audit token",
    )?;
    validate_binding(&prepared.namespace_root)?;
    validate_binding(&prepared.staging_root)
}

fn durable_publish_bytes(
    parent_fd: RawFd,
    temporary_name: &str,
    final_name: &str,
    bytes: &[u8],
) -> Result<ObjectBindingV1, AcceptanceError> {
    if bytes.is_empty() || bytes.len() as u64 > MAX_ARTIFACT_BYTES {
        return Err(invalid("durable record size is outside the fixed bound"));
    }
    if optional_snapshot(parent_fd, temporary_name)?.is_some()
        || optional_snapshot(parent_fd, final_name)?.is_some()
    {
        return Err(invalid(
            "durable no-replace record name is already occupied",
        ));
    }
    let mut temporary = createat_file(parent_fd, temporary_name, 0o400)?;
    temporary.write_all(bytes)?;
    temporary.sync_all()?;
    let temporary_snapshot = snapshot_fd(temporary.as_raw_fd(), temporary_name)?;
    if temporary_snapshot.kind != NodeKind::RegularFile
        || temporary_snapshot.mode() != 0o400
        || temporary_snapshot.nlink() != 1
    {
        return Err(invalid("durable temporary record metadata is invalid"));
    }
    rename_noreplace(parent_fd, temporary_name, parent_fd, final_name)?;
    let parent = duplicate_fd(parent_fd)?;
    parent.sync_all()?;
    let final_file = openat_regular(parent_fd, final_name)?;
    let final_snapshot = snapshot_fd(final_file.as_raw_fd(), final_name)?;
    if final_snapshot.dev() != temporary_snapshot.dev()
        || final_snapshot.inode() != temporary_snapshot.inode()
        || read_open_file(&final_file)? != bytes
    {
        return Err(invalid(
            "durable record is not the exact temporary inode and bytes after rename",
        ));
    }
    parent.sync_all()?;
    Ok(final_snapshot.binding)
}

fn persist_or_verify_canonical(
    parent_fd: RawFd,
    temporary_name: &str,
    final_name: &str,
    expected: &[u8],
) -> Result<(), AcceptanceError> {
    if optional_snapshot(parent_fd, temporary_name)?.is_some() {
        return Err(invalid(
            "durable record has a crash-temporary sibling; reconciliation is required",
        ));
    }
    if optional_snapshot(parent_fd, final_name)?.is_some() {
        let existing = openat_regular(parent_fd, final_name)?;
        if read_open_file(&existing)? != expected {
            return Err(invalid(
                "existing durable record differs from recovery result",
            ));
        }
        return Ok(());
    }
    durable_publish_bytes(parent_fd, temporary_name, final_name, expected)?;
    Ok(())
}

fn read_canonical_at<T: DeserializeOwned + Serialize>(
    parent_fd: RawFd,
    name: &str,
    label: &str,
) -> Result<(T, Vec<u8>), AcceptanceError> {
    let file = openat_regular(parent_fd, name)?;
    let bytes = read_open_file(&file)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|error| invalid(format!("{label} is invalid JSON: {error}")))?;
    let parsed: T = serde_json::from_value(value)
        .map_err(|error| invalid(format!("{label} is malformed: {error}")))?;
    if canonical_json(&parsed)? != bytes {
        return Err(invalid(format!("{label} is not canonical JSON")));
    }
    Ok((parsed, bytes))
}

fn optional_snapshot(parent_fd: RawFd, name: &str) -> Result<Option<Snapshot>, AcceptanceError> {
    match fstatat_snapshot(parent_fd, name) {
        Ok(snapshot) => Ok(Some(snapshot)),
        Err(AcceptanceError::Io(error)) if error.raw_os_error() == Some(libc::ENOENT) => Ok(None),
        Err(error) => Err(error),
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ProcessVisibleHolderDiagnosticsV1 {
    advisory_only: bool,
    all_pid_gt_zero_scanned_as_root: bool,
    cwd_or_root_vnode_count: u64,
    enumeration_non_atomic_limitation_bound: bool,
    external_holder_count: u64,
    fileport_vnode_count: u64,
    kernel_pid_zero_region_excluded: bool,
    mapped_vnode_count: u64,
    open_vnode_count: u64,
    opaque_vm_submap_count: u64,
    pid_identity_reuse_detected: bool,
    pid_start_identities_sha256: String,
    producer_process_count: u64,
    region_vnode_race_limitation_bound: bool,
    scan_complete: bool,
    scan_one_receipt_sha256: String,
    scan_two_receipt_sha256: String,
    unknown_or_inaccessible_process_count: u64,
    vm_submap_limitation_bound: bool,
    writable_shared_mapping_count: u64,
    writable_vnode_fd_count: u64,
}

impl ProcessVisibleHolderDiagnosticsV1 {
    fn validate(&self) -> Result<(), AcceptanceError> {
        require_digest(
            &self.scan_one_receipt_sha256,
            "first handle-drain scan receipt",
        )?;
        require_digest(
            &self.scan_two_receipt_sha256,
            "second handle-drain scan receipt",
        )?;
        require_digest(
            &self.pid_start_identities_sha256,
            "PID start identity inventory",
        )?;
        if self.scan_one_receipt_sha256 == self.scan_two_receipt_sha256
            || !self.advisory_only
            || !self.scan_complete
            || !self.all_pid_gt_zero_scanned_as_root
            || !self.kernel_pid_zero_region_excluded
            || !self.vm_submap_limitation_bound
            || !self.region_vnode_race_limitation_bound
            || !self.enumeration_non_atomic_limitation_bound
            || self.pid_identity_reuse_detected
            || self.open_vnode_count != 0
            || self.mapped_vnode_count != 0
            || self.fileport_vnode_count != 0
            || self.cwd_or_root_vnode_count != 0
            || self.external_holder_count != 0
            || self.writable_vnode_fd_count != 0
            || self.writable_shared_mapping_count != 0
            || self.unknown_or_inaccessible_process_count != 0
            || self.producer_process_count != 0
        {
            return Err(invalid(
                "process-visible holder diagnostics are incomplete, overclaim kernel coverage, or contain an observed external holder",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ApfsUnmountBarrierProofV1 {
    barrier_boot_session_uuid: String,
    barrier_epoch_nonce: String,
    clean_unmount_receipt_sha256: String,
    clean_unmount_succeeded: bool,
    forced_unmount: bool,
    kernel_wide_holder_gate: bool,
    mount_parent_after: ObjectBindingV1,
    mount_parent_before: ObjectBindingV1,
    mountpoint_underlying_after: ObjectBindingV1,
    mountpoint_underlying_before: ObjectBindingV1,
    operation_challenge_sha256: String,
    owners_enabled_after: bool,
    owners_enabled_before: bool,
    read_only_remount_receipt_sha256: String,
    read_only_volume_after: bool,
    schema: String,
    source_binding_after: ObjectBindingV1,
    source_binding_before: ObjectBindingV1,
    source_closure_sha256: String,
    volume_uuid_after: String,
    volume_uuid_before: String,
    writable_media_after: bool,
    writable_volume_after: bool,
}

impl ApfsUnmountBarrierProofV1 {
    fn validate(&self) -> Result<(), AcceptanceError> {
        require_digest(
            &self.clean_unmount_receipt_sha256,
            "APFS clean-unmount receipt",
        )?;
        require_digest(
            &self.read_only_remount_receipt_sha256,
            "APFS read-only remount receipt",
        )?;
        require_digest(
            &self.operation_challenge_sha256,
            "APFS barrier operation challenge",
        )?;
        require_digest(&self.source_closure_sha256, "APFS source closure")?;
        require_nonce(&self.barrier_epoch_nonce, "APFS barrier epoch nonce")?;
        require_uuid(
            &self.barrier_boot_session_uuid,
            "APFS barrier boot session UUID",
        )?;
        require_uuid(&self.volume_uuid_before, "APFS barrier volume UUID")?;
        require_uuid(&self.volume_uuid_after, "APFS remounted volume UUID")?;
        validate_binding(&self.mount_parent_before)?;
        validate_binding(&self.mount_parent_after)?;
        validate_binding(&self.mountpoint_underlying_before)?;
        validate_binding(&self.mountpoint_underlying_after)?;
        validate_binding(&self.source_binding_before)?;
        validate_binding(&self.source_binding_after)?;
        if self.schema != "hepta_mac_apfs_unmount_barrier_proof_v1"
            || !self.clean_unmount_succeeded
            || self.forced_unmount
            || !self.kernel_wide_holder_gate
            || !self.owners_enabled_before
            || !self.owners_enabled_after
            || !self.read_only_volume_after
            || self.writable_media_after
            || self.writable_volume_after
            || self.volume_uuid_before != self.volume_uuid_after
            || self.mount_parent_before.dev != self.mount_parent_after.dev
            || self.mount_parent_before.inode != self.mount_parent_after.inode
            || self.mountpoint_underlying_before.dev != self.mountpoint_underlying_after.dev
            || self.mountpoint_underlying_before.inode != self.mountpoint_underlying_after.inode
            || self.mountpoint_underlying_before.uid != 0
            || self.mountpoint_underlying_before.gid != 0
            || self.mountpoint_underlying_before.mode != 0o700
            || self.mountpoint_underlying_after.uid != 0
            || self.mountpoint_underlying_after.gid != 0
            || self.mountpoint_underlying_after.mode != 0o700
            || self.source_binding_before.dev != self.source_binding_after.dev
            || self.source_binding_before.inode != self.source_binding_after.inode
        {
            return Err(invalid(
                "APFS barrier requires a clean non-forced kernel unmount and UUID-stable owners-enabled read-only remount",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct HandleDrainProofV1 {
    apfs_unmount_barrier: ApfsUnmountBarrierProofV1,
    content_topology_after_sha256: String,
    content_topology_before_sha256: String,
    isolated_exact_replay_sha256: String,
    original_exact_replay_sha256: String,
    process_visible_diagnostics: ProcessVisibleHolderDiagnosticsV1,
    writer_launchd_disabled_and_absent: bool,
}

impl HandleDrainProofV1 {
    fn validate(&self) -> Result<(), AcceptanceError> {
        self.apfs_unmount_barrier.validate()?;
        self.process_visible_diagnostics.validate()?;
        require_digest(
            &self.content_topology_before_sha256,
            "pre-isolation content/topology closure",
        )?;
        require_digest(
            &self.content_topology_after_sha256,
            "post-isolation content/topology closure",
        )?;
        require_digest(
            &self.original_exact_replay_sha256,
            "original exact metadata replay",
        )?;
        require_digest(
            &self.isolated_exact_replay_sha256,
            "isolated exact metadata replay",
        )?;
        if self.content_topology_before_sha256 != self.content_topology_after_sha256
            || !self.writer_launchd_disabled_and_absent
        {
            return Err(invalid(
                "source content/topology changed or legacy writer admission remains enabled",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct OwnershipConversionV1 {
    legacy_writer_uid: u32,
    quarantine_gid: u32,
    quarantine_uid: u32,
    schema: String,
    target_producer_uid: u32,
}

impl OwnershipConversionV1 {
    fn validate(&self, record: &BarrierRecordV1) -> Result<(), AcceptanceError> {
        if self.schema != "hepta_mac_source_ownership_conversion_v1"
            || self.legacy_writer_uid != record.legacy_writer_uid
            || self.quarantine_uid != 0
            || self.quarantine_gid != 0
            || self.target_producer_uid != record.target_producer_uid
            || self.legacy_writer_uid == self.target_producer_uid
            || self.target_producer_uid == self.quarantine_uid
        {
            return Err(invalid(
                "source ownership conversion does not bind legacy, root quarantine, and dedicated target identities",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum BarrierEventV1 {
    Acquired {
        barrier_root: ObjectBindingV1,
        client_executable_sha256: String,
        dedicated_volume_uuid: String,
        helper_executable_sha256: String,
        mountpoint_underlying: ObjectBindingV1,
        operation_challenge_sha256: String,
        source_binding: ObjectBindingV1,
    },
    SourceIsolated {
        drain: HandleDrainProofV1,
        isolated: ObjectBindingV1,
        metadata_receipt_sha256: String,
        original: ObjectBindingV1,
        ownership_conversion: OwnershipConversionV1,
    },
    SnapshotPublished {
        snapshot_receipt_sha256: String,
        source_binding: ObjectBindingV1,
    },
    CanaryPassed {
        canary_receipt_sha256: String,
        snapshot_receipt_sha256: String,
    },
    CutoverPassed {
        cutover_receipt_sha256: String,
    },
    RollbackRestored {
        restored: ObjectBindingV1,
        restored_replay_sha256: String,
        rollback_receipt_sha256: String,
    },
    RecutoverSourceIsolated {
        drain: HandleDrainProofV1,
        freshness_nonce: String,
        isolated: ObjectBindingV1,
        metadata_receipt_sha256: String,
        ownership_conversion: OwnershipConversionV1,
    },
    RecutoverSnapshotPublished {
        snapshot_receipt_sha256: String,
        source_binding: ObjectBindingV1,
    },
    RecutoverPassed {
        recutover_receipt_sha256: String,
    },
    Released {
        terminal_aggregate_sha256: String,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BarrierPhaseV1 {
    Acquired,
    SourceIsolated,
    SnapshotPublished,
    CanaryPassed,
    CutoverPassed,
    RollbackRestored,
    RecutoverSourceIsolated,
    RecutoverSnapshotPublished,
    RecutoverPassed,
    Released,
}

impl BarrierEventV1 {
    fn phase(&self) -> BarrierPhaseV1 {
        match self {
            Self::Acquired { .. } => BarrierPhaseV1::Acquired,
            Self::SourceIsolated { .. } => BarrierPhaseV1::SourceIsolated,
            Self::SnapshotPublished { .. } => BarrierPhaseV1::SnapshotPublished,
            Self::CanaryPassed { .. } => BarrierPhaseV1::CanaryPassed,
            Self::CutoverPassed { .. } => BarrierPhaseV1::CutoverPassed,
            Self::RollbackRestored { .. } => BarrierPhaseV1::RollbackRestored,
            Self::RecutoverSourceIsolated { .. } => BarrierPhaseV1::RecutoverSourceIsolated,
            Self::RecutoverSnapshotPublished { .. } => BarrierPhaseV1::RecutoverSnapshotPublished,
            Self::RecutoverPassed { .. } => BarrierPhaseV1::RecutoverPassed,
            Self::Released { .. } => BarrierPhaseV1::Released,
        }
    }
}

const PHASES: [BarrierPhaseV1; 10] = [
    BarrierPhaseV1::Acquired,
    BarrierPhaseV1::SourceIsolated,
    BarrierPhaseV1::SnapshotPublished,
    BarrierPhaseV1::CanaryPassed,
    BarrierPhaseV1::CutoverPassed,
    BarrierPhaseV1::RollbackRestored,
    BarrierPhaseV1::RecutoverSourceIsolated,
    BarrierPhaseV1::RecutoverSnapshotPublished,
    BarrierPhaseV1::RecutoverPassed,
    BarrierPhaseV1::Released,
];

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct BarrierChallengeMaterialV1<'a> {
    barrier_root: &'a ObjectBindingV1,
    boot_session_uuid: &'a str,
    client_executable_sha256: &'a str,
    dedicated_volume_uuid: &'a str,
    epoch_nonce: &'a str,
    helper_executable_sha256: &'a str,
    mountpoint_underlying: &'a ObjectBindingV1,
    peer_audit_token_sha256: &'a str,
    schema: &'static str,
    source_binding: &'a ObjectBindingV1,
}

#[allow(clippy::too_many_arguments)]
fn barrier_operation_challenge(
    epoch_nonce: &str,
    boot_session_uuid: &str,
    peer_audit_token_sha256: &str,
    client_executable_sha256: &str,
    helper_executable_sha256: &str,
    barrier_root: &ObjectBindingV1,
    dedicated_volume_uuid: &str,
    mountpoint_underlying: &ObjectBindingV1,
    source_binding: &ObjectBindingV1,
) -> Result<String, AcceptanceError> {
    require_nonce(epoch_nonce, "barrier epoch nonce")?;
    require_uuid(dedicated_volume_uuid, "dedicated APFS volume UUID")?;
    require_digest(peer_audit_token_sha256, "barrier peer audit token")?;
    require_digest(client_executable_sha256, "barrier client executable")?;
    require_digest(helper_executable_sha256, "barrier helper executable")?;
    Ok(sha256(&canonical_json(&BarrierChallengeMaterialV1 {
        barrier_root,
        boot_session_uuid,
        client_executable_sha256,
        dedicated_volume_uuid,
        epoch_nonce,
        helper_executable_sha256,
        mountpoint_underlying,
        peer_audit_token_sha256,
        schema: "hepta_mac_writer_barrier_challenge_material_v1",
        source_binding,
    })?))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct BarrierRecordV1 {
    pub boot_session_uuid: String,
    pub epoch_nonce: String,
    pub event: BarrierEventV1,
    pub operator_audit_session_id: u32,
    pub peer_audit_token_sha256: String,
    pub previous_record_sha256: Option<String>,
    pub target_producer_uid: u32,
    pub schema: String,
    pub sequence: u64,
    pub legacy_writer_uid: u32,
    pub trusted_operator_uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct BarrierEnvelopeV1 {
    record: BarrierRecordV1,
    record_sha256: String,
    schema: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum BarrierRecoveryDispositionV1 {
    RestoreLegacy,
    QuarantineOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct BarrierRecoveryReceiptV1 {
    action_receipt_sha256: String,
    aggregate_authority: bool,
    boot_session_uuid: String,
    disposition: BarrierRecoveryDispositionV1,
    forward_authority: bool,
    grant_authority: bool,
    original_epoch_peer_audit_token_sha256: String,
    prior_phase: BarrierPhaseV1,
    prior_terminal_record_sha256: String,
    production_authority: bool,
    quarantined_binding: Option<ObjectBindingV1>,
    recovery_peer_audit_session_id: u32,
    recovery_peer_audit_token_sha256: String,
    restored_binding: Option<ObjectBindingV1>,
    schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BarrierVerificationV1 {
    pub admission_release_authority: bool,
    pub admission_state: String,
    pub admissions_closed: bool,
    pub current_phase: BarrierPhaseV1,
    pub epoch_nonce: String,
    pub live_authority: bool,
    pub record_count: usize,
    pub recovery_disposition: String,
    pub schema: String,
    pub terminal_record_sha256: String,
}

/// Descriptor-bound durable journal for the non-advisory writer barrier.
pub struct BarrierJournal {
    directory: File,
    durability_ancestors: Vec<File>,
    policy: NamespacePolicy,
}

impl BarrierJournal {
    pub fn open(directory: &Path, policy: NamespacePolicy) -> Result<Self, AcceptanceError> {
        let opened = open_barrier_namespace(directory, &policy)?;
        verify_private_namespace(&opened.namespace, &policy)?;
        Ok(Self {
            directory: opened.namespace,
            durability_ancestors: opened.durability_ancestors,
            policy,
        })
    }

    fn begin(
        &self,
        epoch_nonce: &str,
        peer: &AuthenticatedPeerV1,
        barrier_root: ObjectBindingV1,
        dedicated_volume_uuid: &str,
        mountpoint_underlying: ObjectBindingV1,
        source_binding: ObjectBindingV1,
    ) -> Result<BarrierVerificationV1, AcceptanceError> {
        if !read_dir_names(self.directory.as_raw_fd())?.is_empty() {
            return Err(invalid("barrier journal is not empty"));
        }
        require_nonce(epoch_nonce, "barrier epoch nonce")?;
        if peer.effective_uid != self.policy.operator_uid {
            return Err(invalid(
                "barrier peer differs from trusted operator UID pin",
            ));
        }
        let boot_session_uuid = current_boot_session_uuid()?;
        let operation_challenge_sha256 = barrier_operation_challenge(
            epoch_nonce,
            &boot_session_uuid,
            &peer.audit_token_sha256,
            &peer.executable_sha256,
            &self.policy.helper_executable_sha256,
            &barrier_root,
            dedicated_volume_uuid,
            &mountpoint_underlying,
            &source_binding,
        )?;
        let event = BarrierEventV1::Acquired {
            barrier_root,
            client_executable_sha256: peer.executable_sha256.clone(),
            dedicated_volume_uuid: dedicated_volume_uuid.to_string(),
            helper_executable_sha256: self.policy.helper_executable_sha256.clone(),
            mountpoint_underlying,
            operation_challenge_sha256,
            source_binding,
        };
        self.append_record(BarrierRecordV1 {
            boot_session_uuid,
            epoch_nonce: epoch_nonce.to_string(),
            event,
            operator_audit_session_id: peer.audit_session_id,
            peer_audit_token_sha256: peer.audit_token_sha256.clone(),
            previous_record_sha256: None,
            target_producer_uid: self.policy.producer_uid,
            schema: "hepta_mac_writer_barrier_record_v1".to_string(),
            sequence: 1,
            legacy_writer_uid: self.policy.legacy_writer_uid,
            trusted_operator_uid: self.policy.operator_uid,
        })?;
        self.verify()
    }

    fn transition(
        &self,
        peer: &AuthenticatedPeerV1,
        event: BarrierEventV1,
    ) -> Result<BarrierVerificationV1, AcceptanceError> {
        if self.policy.privileged_mode() && event.phase() == BarrierPhaseV1::Released {
            return Err(invalid(
                "privileged mechanism journal cannot release admission or grant migration authority",
            ));
        }
        if optional_snapshot(self.directory.as_raw_fd(), RECOVERY_RECORD_NAME)?.is_some() {
            return Err(invalid(
                "recovered barrier epoch cannot transition forward or grant authority",
            ));
        }
        let envelopes = self.read_and_validate()?;
        let last = envelopes
            .last()
            .ok_or_else(|| invalid("barrier transition requires an acquired epoch"))?;
        if current_boot_session_uuid()? != last.record.boot_session_uuid {
            return Err(invalid(
                "barrier transition belongs to a prior boot and cannot be appended",
            ));
        }
        if last.record.event.phase() == BarrierPhaseV1::Released {
            return Err(invalid("released barrier epoch is immutable"));
        }
        if peer.effective_uid != last.record.trusted_operator_uid
            || peer.audit_token_sha256 != last.record.peer_audit_token_sha256
        {
            return Err(invalid(
                "barrier transition peer differs from acquired audit identity",
            ));
        }
        let sequence = last
            .record
            .sequence
            .checked_add(1)
            .ok_or_else(|| invalid("barrier sequence overflow"))?;
        self.append_record(BarrierRecordV1 {
            boot_session_uuid: last.record.boot_session_uuid.clone(),
            epoch_nonce: last.record.epoch_nonce.clone(),
            event,
            operator_audit_session_id: last.record.operator_audit_session_id,
            peer_audit_token_sha256: last.record.peer_audit_token_sha256.clone(),
            previous_record_sha256: Some(last.record_sha256.clone()),
            target_producer_uid: last.record.target_producer_uid,
            schema: "hepta_mac_writer_barrier_record_v1".to_string(),
            sequence,
            legacy_writer_uid: last.record.legacy_writer_uid,
            trusted_operator_uid: last.record.trusted_operator_uid,
        })?;
        self.verify()
    }

    pub fn verify(&self) -> Result<BarrierVerificationV1, AcceptanceError> {
        let envelopes = self.read_and_validate()?;
        let last = envelopes
            .last()
            .ok_or_else(|| invalid("barrier journal is empty"))?;
        if current_boot_session_uuid()? != last.record.boot_session_uuid {
            return Err(invalid(
                "barrier journal belongs to a prior boot and is fail-closed",
            ));
        }
        let phase = last.record.event.phase();
        let recovery = self.read_recovery(last)?;
        Ok(BarrierVerificationV1 {
            admission_release_authority: false,
            admission_state: "mechanism_intent_only_no_native_holder".to_string(),
            admissions_closed: false,
            current_phase: phase,
            epoch_nonce: last.record.epoch_nonce.clone(),
            live_authority: false,
            record_count: envelopes.len(),
            recovery_disposition: match recovery.as_ref().map(|receipt| receipt.disposition) {
                Some(BarrierRecoveryDispositionV1::RestoreLegacy) => {
                    "recovery_only_restore_legacy_no_forward_authority".to_string()
                }
                Some(BarrierRecoveryDispositionV1::QuarantineOnly) => {
                    "recovery_only_quarantine_no_forward_authority".to_string()
                }
                None if phase == BarrierPhaseV1::Released => "terminal_released".to_string(),
                None => "fail_closed_keep_namespace_isolated_and_require_privileged_reconciliation"
                    .to_string(),
            },
            schema: "hepta_mac_writer_barrier_verification_v1".to_string(),
            terminal_record_sha256: last.record_sha256.clone(),
        })
    }

    /// Persist the result of a separately executed recovery-only action.  This
    /// method is private so an IPC caller cannot manufacture action facts.  A
    /// future installed broker may call it only after its native restore or
    /// quarantine primitive has completed and independently replayed state.
    fn record_recovery_terminal(
        &self,
        recovery_peer: &AuthenticatedPeerV1,
        disposition: BarrierRecoveryDispositionV1,
        action_receipt_sha256: &str,
        restored_binding: Option<ObjectBindingV1>,
        quarantined_binding: Option<ObjectBindingV1>,
    ) -> Result<BarrierVerificationV1, AcceptanceError> {
        require_digest(action_receipt_sha256, "barrier recovery action receipt")?;
        if optional_snapshot(self.directory.as_raw_fd(), RECOVERY_RECORD_NAME)?.is_some() {
            return Err(invalid("barrier recovery terminal already exists"));
        }
        let envelopes = self.read_and_validate()?;
        let last = envelopes
            .last()
            .ok_or_else(|| invalid("barrier recovery requires an acquired epoch"))?;
        if last.record.event.phase() == BarrierPhaseV1::Released {
            return Err(invalid("released barrier does not permit recovery"));
        }
        if recovery_peer.effective_uid != last.record.trusted_operator_uid
            || recovery_peer.audit_token_sha256 == last.record.peer_audit_token_sha256
            || current_boot_session_uuid()? != last.record.boot_session_uuid
        {
            return Err(invalid(
                "recovery requires a fresh trusted-operator peer in the original boot session",
            ));
        }
        let receipt = BarrierRecoveryReceiptV1 {
            action_receipt_sha256: action_receipt_sha256.to_string(),
            aggregate_authority: false,
            boot_session_uuid: last.record.boot_session_uuid.clone(),
            disposition,
            forward_authority: false,
            grant_authority: false,
            original_epoch_peer_audit_token_sha256: last.record.peer_audit_token_sha256.clone(),
            prior_phase: last.record.event.phase(),
            prior_terminal_record_sha256: last.record_sha256.clone(),
            production_authority: false,
            quarantined_binding,
            recovery_peer_audit_session_id: recovery_peer.audit_session_id,
            recovery_peer_audit_token_sha256: recovery_peer.audit_token_sha256.clone(),
            restored_binding,
            schema: "hepta_mac_writer_barrier_recovery_receipt_v1".to_string(),
        };
        validate_recovery_receipt(&receipt, &envelopes, recovery_peer)?;
        durable_publish_bytes(
            self.directory.as_raw_fd(),
            ".incoming-recovery-terminal",
            RECOVERY_RECORD_NAME,
            &canonical_json(&receipt)?,
        )?;
        self.sync_durability_chain()?;
        self.verify()
    }

    fn read_recovery(
        &self,
        last: &BarrierEnvelopeV1,
    ) -> Result<Option<BarrierRecoveryReceiptV1>, AcceptanceError> {
        if optional_snapshot(self.directory.as_raw_fd(), RECOVERY_RECORD_NAME)?.is_none() {
            return Ok(None);
        }
        let (receipt, _) = read_canonical_at::<BarrierRecoveryReceiptV1>(
            self.directory.as_raw_fd(),
            RECOVERY_RECORD_NAME,
            "barrier recovery terminal",
        )?;
        let envelopes = self.read_and_validate()?;
        let observed_last = envelopes
            .last()
            .ok_or_else(|| invalid("barrier recovery has no prior journal"))?;
        if observed_last != last {
            return Err(invalid("barrier recovery prior journal changed"));
        }
        validate_recovery_receipt_shape(&receipt, observed_last)?;
        validate_recovery_binding(&receipt, &envelopes)?;
        Ok(Some(receipt))
    }

    fn append_record(&self, record: BarrierRecordV1) -> Result<(), AcceptanceError> {
        validate_record_shape(&record)?;
        let record_bytes = canonical_json(&record)?;
        let envelope = BarrierEnvelopeV1 {
            record,
            record_sha256: sha256(&record_bytes),
            schema: "hepta_mac_writer_barrier_envelope_v1".to_string(),
        };
        // Reject malformed or out-of-order evidence before publishing even a
        // temporary journal entry.  A bad request must not poison recovery.
        let mut candidate = self.read_and_validate()?;
        candidate.push(envelope.clone());
        validate_barrier_chain(&candidate)?;
        let bytes = canonical_json(&envelope)?;
        let final_name = record_name(envelope.record.sequence);
        let temporary_name = format!(".incoming-{final_name}");
        let mut file = createat_file(self.directory.as_raw_fd(), &temporary_name, 0o400)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        rename_noreplace(
            self.directory.as_raw_fd(),
            &temporary_name,
            self.directory.as_raw_fd(),
            &final_name,
        )?;
        self.sync_durability_chain()?;
        let reopened = openat_regular(self.directory.as_raw_fd(), &final_name)?;
        if read_open_file(&reopened)? != bytes {
            return Err(invalid("durable barrier record differs after publication"));
        }
        Ok(())
    }

    fn sync_durability_chain(&self) -> Result<(), AcceptanceError> {
        self.directory.sync_all()?;
        for ancestor in &self.durability_ancestors {
            ancestor.sync_all()?;
        }
        Ok(())
    }

    fn read_and_validate(&self) -> Result<Vec<BarrierEnvelopeV1>, AcceptanceError> {
        let names = read_dir_names(self.directory.as_raw_fd())?
            .into_iter()
            .filter(|name| name.as_bytes() != RECOVERY_RECORD_NAME.as_bytes())
            .collect::<Vec<_>>();
        if names.is_empty() {
            return Ok(Vec::new());
        }
        let mut envelopes = Vec::with_capacity(names.len());
        for (index, name) in names.into_iter().enumerate() {
            let name = name
                .into_string()
                .map_err(|_| invalid("barrier record name is not UTF-8"))?;
            let sequence = (index + 1) as u64;
            if name != record_name(sequence) {
                return Err(invalid(
                    "barrier journal contains a gap, temporary file, or unknown entry",
                ));
            }
            let file = openat_regular(self.directory.as_raw_fd(), &name)?;
            let bytes = read_open_file(&file)?;
            let value: serde_json::Value = serde_json::from_slice(&bytes)
                .map_err(|error| invalid(format!("barrier envelope is invalid JSON: {error}")))?;
            let envelope: BarrierEnvelopeV1 = serde_json::from_value(value)
                .map_err(|error| invalid(format!("barrier envelope is malformed: {error}")))?;
            if canonical_json(&envelope)? != bytes
                || envelope.schema != "hepta_mac_writer_barrier_envelope_v1"
                || envelope.record.sequence != sequence
            {
                return Err(invalid(
                    "barrier envelope is not canonical or sequence-bound",
                ));
            }
            let record_bytes = canonical_json(&envelope.record)?;
            if sha256(&record_bytes) != envelope.record_sha256 {
                return Err(invalid("barrier record digest mismatch"));
            }
            validate_record_shape(&envelope.record)?;
            envelopes.push(envelope);
        }
        validate_barrier_chain(&envelopes)?;
        Ok(envelopes)
    }
}

fn validate_barrier_chain(envelopes: &[BarrierEnvelopeV1]) -> Result<(), AcceptanceError> {
    let first = envelopes
        .first()
        .ok_or_else(|| invalid("barrier chain is empty"))?;
    if first.record.previous_record_sha256.is_some() {
        return Err(invalid("first barrier record has a predecessor"));
    }
    let epoch = &first.record.epoch_nonce;
    let boot_session_uuid = &first.record.boot_session_uuid;
    let peer = &first.record.peer_audit_token_sha256;
    let operator_audit_session_id = first.record.operator_audit_session_id;
    let producer = first.record.target_producer_uid;
    let legacy_writer = first.record.legacy_writer_uid;
    let operator = first.record.trusted_operator_uid;
    for (index, envelope) in envelopes.iter().enumerate() {
        if index >= PHASES.len()
            || envelope.record.event.phase() != PHASES[index]
            || envelope.record.epoch_nonce != *epoch
            || envelope.record.boot_session_uuid != *boot_session_uuid
            || envelope.record.peer_audit_token_sha256 != *peer
            || envelope.record.operator_audit_session_id != operator_audit_session_id
            || envelope.record.target_producer_uid != producer
            || envelope.record.legacy_writer_uid != legacy_writer
            || envelope.record.trusted_operator_uid != operator
        {
            return Err(invalid("barrier record order or epoch identity is invalid"));
        }
        let expected_previous = index
            .checked_sub(1)
            .map(|previous| envelopes[previous].record_sha256.as_str());
        if envelope.record.previous_record_sha256.as_deref() != expected_previous {
            return Err(invalid("barrier digest chain is broken"));
        }
        validate_transition_evidence(envelopes, index)?;
    }
    Ok(())
}

fn validate_recovery_receipt(
    receipt: &BarrierRecoveryReceiptV1,
    envelopes: &[BarrierEnvelopeV1],
    recovery_peer: &AuthenticatedPeerV1,
) -> Result<(), AcceptanceError> {
    let last = envelopes
        .last()
        .ok_or_else(|| invalid("barrier recovery has no prior record"))?;
    validate_recovery_receipt_shape(receipt, last)?;
    if receipt.recovery_peer_audit_session_id != recovery_peer.audit_session_id
        || receipt.recovery_peer_audit_token_sha256 != recovery_peer.audit_token_sha256
    {
        return Err(invalid(
            "barrier recovery receipt differs from the authenticated fresh peer",
        ));
    }
    validate_recovery_binding(receipt, envelopes)
}

fn validate_recovery_receipt_shape(
    receipt: &BarrierRecoveryReceiptV1,
    last: &BarrierEnvelopeV1,
) -> Result<(), AcceptanceError> {
    if receipt.schema != "hepta_mac_writer_barrier_recovery_receipt_v1"
        || receipt.aggregate_authority
        || receipt.forward_authority
        || receipt.grant_authority
        || receipt.production_authority
        || receipt.boot_session_uuid != last.record.boot_session_uuid
        || receipt.prior_phase != last.record.event.phase()
        || receipt.prior_terminal_record_sha256 != last.record_sha256
        || receipt.original_epoch_peer_audit_token_sha256 != last.record.peer_audit_token_sha256
        || receipt.recovery_peer_audit_token_sha256
            == receipt.original_epoch_peer_audit_token_sha256
    {
        return Err(invalid(
            "barrier recovery receipt is not a closed-authority fresh-peer terminal",
        ));
    }
    require_uuid(&receipt.boot_session_uuid, "recovery boot session UUID")?;
    require_digest(
        &receipt.action_receipt_sha256,
        "barrier recovery action receipt",
    )?;
    require_digest(
        &receipt.recovery_peer_audit_token_sha256,
        "barrier recovery peer audit token",
    )?;
    Ok(())
}

fn validate_recovery_binding(
    receipt: &BarrierRecoveryReceiptV1,
    envelopes: &[BarrierEnvelopeV1],
) -> Result<(), AcceptanceError> {
    let initial = envelopes
        .iter()
        .find_map(|envelope| match &envelope.record.event {
            BarrierEventV1::SourceIsolated {
                isolated, original, ..
            } => Some((original, isolated)),
            _ => None,
        });
    let latest_isolated =
        envelopes
            .iter()
            .rev()
            .find_map(|envelope| match &envelope.record.event {
                BarrierEventV1::RecutoverSourceIsolated { isolated, .. }
                | BarrierEventV1::SourceIsolated { isolated, .. } => Some(isolated),
                _ => None,
            });
    let Some((original, _)) = initial else {
        return Err(invalid(
            "barrier recovery cannot attest source state before native isolation evidence",
        ));
    };
    match receipt.disposition {
        BarrierRecoveryDispositionV1::RestoreLegacy => {
            let restored = receipt
                .restored_binding
                .as_ref()
                .ok_or_else(|| invalid("restore recovery omitted restored binding"))?;
            if receipt.quarantined_binding.is_some() || restored != original {
                return Err(invalid(
                    "restore recovery did not reproduce the exact original binding",
                ));
            }
        }
        BarrierRecoveryDispositionV1::QuarantineOnly => {
            let quarantined = receipt
                .quarantined_binding
                .as_ref()
                .ok_or_else(|| invalid("quarantine recovery omitted quarantined binding"))?;
            if receipt.restored_binding.is_some() || Some(quarantined) != latest_isolated {
                return Err(invalid(
                    "quarantine recovery differs from the latest isolated binding",
                ));
            }
        }
    }
    Ok(())
}

fn validate_transition_evidence(
    envelopes: &[BarrierEnvelopeV1],
    index: usize,
) -> Result<(), AcceptanceError> {
    match &envelopes[index].record.event {
        BarrierEventV1::Acquired {
            barrier_root,
            client_executable_sha256,
            dedicated_volume_uuid,
            helper_executable_sha256,
            mountpoint_underlying,
            operation_challenge_sha256,
            source_binding,
        } => {
            validate_binding(barrier_root)?;
            validate_binding(mountpoint_underlying)?;
            validate_binding(source_binding)?;
            require_digest(client_executable_sha256, "client executable")?;
            require_digest(helper_executable_sha256, "helper executable")?;
            require_digest(operation_challenge_sha256, "barrier operation challenge")?;
            require_uuid(dedicated_volume_uuid, "dedicated APFS volume UUID")?;
            let expected = barrier_operation_challenge(
                &envelopes[index].record.epoch_nonce,
                &envelopes[index].record.boot_session_uuid,
                &envelopes[index].record.peer_audit_token_sha256,
                client_executable_sha256,
                helper_executable_sha256,
                barrier_root,
                dedicated_volume_uuid,
                mountpoint_underlying,
                source_binding,
            )?;
            if expected != *operation_challenge_sha256 {
                return Err(invalid(
                    "barrier operation challenge does not bind acquired facts",
                ));
            }
        }
        BarrierEventV1::SourceIsolated {
            drain,
            isolated,
            metadata_receipt_sha256,
            original,
            ownership_conversion,
        } => {
            drain.validate()?;
            ownership_conversion.validate(&envelopes[index].record)?;
            require_digest(metadata_receipt_sha256, "source metadata receipt")?;
            validate_binding(original)?;
            validate_binding(isolated)?;
            validate_ownership_isolation_transform(
                original,
                isolated,
                envelopes[index].record.legacy_writer_uid,
            )?;
            let acquired = match &envelopes[0].record.event {
                BarrierEventV1::Acquired {
                    dedicated_volume_uuid,
                    mountpoint_underlying,
                    operation_challenge_sha256,
                    source_binding,
                    ..
                } => (
                    dedicated_volume_uuid,
                    mountpoint_underlying,
                    operation_challenge_sha256,
                    source_binding,
                ),
                _ => return Err(invalid("source isolation lacks acquired APFS facts")),
            };
            if drain.apfs_unmount_barrier.barrier_boot_session_uuid
                != envelopes[index].record.boot_session_uuid
                || drain.apfs_unmount_barrier.barrier_epoch_nonce
                    != envelopes[index].record.epoch_nonce
                || drain.apfs_unmount_barrier.operation_challenge_sha256 != *acquired.2
                || drain
                    .apfs_unmount_barrier
                    .volume_uuid_before
                    .to_ascii_lowercase()
                    != acquired.0.to_ascii_lowercase()
                || drain.apfs_unmount_barrier.mountpoint_underlying_before != *acquired.1
                || drain.apfs_unmount_barrier.source_binding_before != *acquired.3
                || drain.apfs_unmount_barrier.source_binding_before != *original
                || drain.apfs_unmount_barrier.source_binding_after != *isolated
                || drain.apfs_unmount_barrier.source_closure_sha256
                    != drain.content_topology_before_sha256
            {
                return Err(invalid(
                    "APFS unmount proof is not bound to acquired epoch, boot, challenge, mountpoint, volume, or source closure",
                ));
            }
        }
        BarrierEventV1::SnapshotPublished {
            snapshot_receipt_sha256,
            source_binding,
        } => {
            require_digest(snapshot_receipt_sha256, "snapshot receipt")?;
            validate_binding(source_binding)?;
            let isolated = match &envelopes[index - 1].record.event {
                BarrierEventV1::SourceIsolated { isolated, .. } => isolated,
                _ => return Err(invalid("snapshot lacks source-isolation predecessor")),
            };
            if source_binding.dev != isolated.dev || source_binding.inode != isolated.inode {
                return Err(invalid(
                    "snapshot is not bound to the isolated source inode",
                ));
            }
        }
        BarrierEventV1::CanaryPassed {
            canary_receipt_sha256,
            snapshot_receipt_sha256,
        } => {
            require_digest(canary_receipt_sha256, "canary receipt")?;
            require_digest(snapshot_receipt_sha256, "snapshot receipt")?;
            let prior = match &envelopes[index - 1].record.event {
                BarrierEventV1::SnapshotPublished {
                    snapshot_receipt_sha256,
                    ..
                } => snapshot_receipt_sha256,
                _ => return Err(invalid("canary lacks snapshot predecessor")),
            };
            if prior != snapshot_receipt_sha256 {
                return Err(invalid("canary is bound to a different snapshot"));
            }
        }
        BarrierEventV1::CutoverPassed {
            cutover_receipt_sha256,
        } => require_digest(cutover_receipt_sha256, "cutover receipt")?,
        BarrierEventV1::RollbackRestored {
            restored,
            restored_replay_sha256,
            rollback_receipt_sha256,
        } => {
            validate_binding(restored)?;
            require_digest(restored_replay_sha256, "rollback restored replay")?;
            require_digest(rollback_receipt_sha256, "rollback receipt")?;
            let (original, original_replay) = match &envelopes[1].record.event {
                BarrierEventV1::SourceIsolated {
                    drain, original, ..
                } => (original, &drain.original_exact_replay_sha256),
                _ => return Err(invalid("rollback lacks original source binding")),
            };
            if restored.dev != original.dev
                || restored.inode != original.inode
                || restored.uid != original.uid
                || restored.gid != original.gid
                || restored.mode != original.mode
                || restored_replay_sha256 != original_replay
            {
                return Err(invalid(
                    "rollback did not restore the original source binding and descriptor replay",
                ));
            }
        }
        BarrierEventV1::RecutoverSourceIsolated {
            drain,
            freshness_nonce,
            isolated,
            metadata_receipt_sha256,
            ownership_conversion,
        } => {
            drain.validate()?;
            ownership_conversion.validate(&envelopes[index].record)?;
            require_nonce(freshness_nonce, "recutover freshness nonce")?;
            validate_binding(isolated)?;
            require_digest(metadata_receipt_sha256, "recutover metadata receipt")?;
            if isolated.uid != 0 || isolated.gid != 0 || isolated.mode != 0o700 {
                return Err(invalid("recutover source is not isolated under root:wheel"));
            }
            let (acquired_volume, acquired_mountpoint, acquired_challenge) =
                match &envelopes[0].record.event {
                    BarrierEventV1::Acquired {
                        dedicated_volume_uuid,
                        mountpoint_underlying,
                        operation_challenge_sha256,
                        ..
                    } => (
                        dedicated_volume_uuid,
                        mountpoint_underlying,
                        operation_challenge_sha256,
                    ),
                    _ => return Err(invalid("recutover lacks acquired APFS facts")),
                };
            let (initial_original, restored) =
                match (&envelopes[1].record.event, &envelopes[5].record.event) {
                    (
                        BarrierEventV1::SourceIsolated { original, .. },
                        BarrierEventV1::RollbackRestored { restored, .. },
                    ) => (original, restored),
                    _ => return Err(invalid("recutover lacks original and restored bindings")),
                };
            validate_ownership_isolation_transform(
                restored,
                isolated,
                envelopes[index].record.legacy_writer_uid,
            )?;
            if isolated.dev != initial_original.dev
                || isolated.inode != initial_original.inode
                || drain.apfs_unmount_barrier.barrier_boot_session_uuid
                    != envelopes[index].record.boot_session_uuid
                || drain.apfs_unmount_barrier.barrier_epoch_nonce
                    != envelopes[index].record.epoch_nonce
                || drain.apfs_unmount_barrier.operation_challenge_sha256 != *acquired_challenge
                || drain
                    .apfs_unmount_barrier
                    .volume_uuid_before
                    .to_ascii_lowercase()
                    != acquired_volume.to_ascii_lowercase()
                || drain.apfs_unmount_barrier.mountpoint_underlying_before != *acquired_mountpoint
                || drain.apfs_unmount_barrier.source_binding_before != *restored
                || drain.apfs_unmount_barrier.source_binding_after != *isolated
                || drain.apfs_unmount_barrier.source_closure_sha256
                    != drain.content_topology_before_sha256
            {
                return Err(invalid(
                    "recutover APFS proof changed inode or is not bound to the acquired boot, epoch, challenge, mountpoint, volume, and restored source",
                ));
            }
            let first = match &envelopes[1].record.event {
                BarrierEventV1::SourceIsolated {
                    drain,
                    metadata_receipt_sha256,
                    ..
                } => (drain, metadata_receipt_sha256),
                _ => return Err(invalid("recutover lacks initial isolation")),
            };
            if first.0.process_visible_diagnostics.scan_one_receipt_sha256
                == drain.process_visible_diagnostics.scan_one_receipt_sha256
                || first.0.process_visible_diagnostics.scan_two_receipt_sha256
                    == drain.process_visible_diagnostics.scan_two_receipt_sha256
                || first.0.apfs_unmount_barrier.clean_unmount_receipt_sha256
                    == drain.apfs_unmount_barrier.clean_unmount_receipt_sha256
                || first
                    .0
                    .apfs_unmount_barrier
                    .read_only_remount_receipt_sha256
                    == drain.apfs_unmount_barrier.read_only_remount_receipt_sha256
                || first.0.content_topology_before_sha256 != drain.content_topology_before_sha256
                || first.0.original_exact_replay_sha256 != drain.original_exact_replay_sha256
                || first.1 == metadata_receipt_sha256
            {
                return Err(invalid("recutover reused stale isolation evidence"));
            }
        }
        BarrierEventV1::RecutoverSnapshotPublished {
            snapshot_receipt_sha256,
            source_binding,
        } => {
            require_digest(snapshot_receipt_sha256, "recutover snapshot receipt")?;
            validate_binding(source_binding)?;
            let (isolated, initial_snapshot) = match (
                &envelopes[index - 1].record.event,
                &envelopes[2].record.event,
            ) {
                (
                    BarrierEventV1::RecutoverSourceIsolated { isolated, .. },
                    BarrierEventV1::SnapshotPublished {
                        snapshot_receipt_sha256,
                        ..
                    },
                ) => (isolated, snapshot_receipt_sha256),
                _ => return Err(invalid("recutover snapshot lacks required predecessors")),
            };
            if source_binding.dev != isolated.dev
                || source_binding.inode != isolated.inode
                || snapshot_receipt_sha256 == initial_snapshot
            {
                return Err(invalid("recutover snapshot is stale or source-unbound"));
            }
        }
        BarrierEventV1::RecutoverPassed {
            recutover_receipt_sha256,
        } => require_digest(recutover_receipt_sha256, "recutover receipt")?,
        BarrierEventV1::Released {
            terminal_aggregate_sha256,
        } => require_digest(terminal_aggregate_sha256, "terminal aggregate")?,
    }
    Ok(())
}

fn validate_record_shape(record: &BarrierRecordV1) -> Result<(), AcceptanceError> {
    if record.schema != "hepta_mac_writer_barrier_record_v1"
        || record.sequence == 0
        || record.target_producer_uid == 0
        || record.legacy_writer_uid == 0
        || record.trusted_operator_uid == 0
        || record.trusted_operator_uid == record.target_producer_uid
        || record.legacy_writer_uid == record.target_producer_uid
    {
        return Err(invalid("barrier record schema or identity is invalid"));
    }
    require_uuid(&record.boot_session_uuid, "barrier boot session UUID")?;
    require_nonce(&record.epoch_nonce, "barrier epoch nonce")?;
    require_digest(&record.peer_audit_token_sha256, "peer audit token")
}

fn validate_binding(binding: &ObjectBindingV1) -> Result<(), AcceptanceError> {
    if binding.dev == 0 || binding.inode == 0 || binding.nlink == 0 {
        return Err(invalid("object binding is incomplete"));
    }
    Ok(())
}

fn validate_ownership_isolation_transform(
    original: &ObjectBindingV1,
    isolated: &ObjectBindingV1,
    legacy_writer_uid: u32,
) -> Result<(), AcceptanceError> {
    if original.dev != isolated.dev
        || original.inode != isolated.inode
        || original.flags != isolated.flags
        || original.mtime_seconds != isolated.mtime_seconds
        || original.mtime_nanoseconds != isolated.mtime_nanoseconds
        || original.nlink != isolated.nlink
        || original.size != isolated.size
        || original.uid != legacy_writer_uid
        || isolated.uid != 0
        || isolated.gid != 0
        || isolated.mode != 0o700
    {
        return Err(invalid(
            "source isolation changed facts outside the unique legacy-owner to root:wheel 0700 transform",
        ));
    }
    Ok(())
}

fn verify_private_namespace(file: &File, policy: &NamespacePolicy) -> Result<(), AcceptanceError> {
    if policy.privileged_mode()
        && resolve_producer_account(policy.producer_uid)?.sha256()?
            != policy.producer_account_sha256
    {
        return Err(invalid(
            "dedicated producer passwd/group identity changed before live mutation",
        ));
    }
    let snapshot = snapshot_fd(file.as_raw_fd(), "namespace")?;
    if snapshot.kind != NodeKind::Directory
        || snapshot.uid() != policy.broker_uid
        || snapshot.gid() != policy.broker_gid
        || snapshot.mode() != 0o700
        || policy.producer_uid == policy.broker_uid
        || policy.producer_groups.contains(&policy.broker_gid)
    {
        return Err(invalid(
            "publication namespace is not broker-owned mode 0700 and producer-unwritable",
        ));
    }
    verify_acl_absent(file.as_raw_fd())?;
    Ok(())
}

fn verify_broker_owned_tree_root(
    snapshot: &Snapshot,
    policy: &NamespacePolicy,
) -> Result<(), AcceptanceError> {
    if snapshot.kind != NodeKind::Directory
        || snapshot.uid() != policy.broker_uid
        || snapshot.gid() != policy.broker_gid
        || snapshot.mode() & 0o022 != 0
    {
        return Err(invalid(
            "staging root is not broker-owned and producer-unwritable",
        ));
    }
    Ok(())
}

fn verify_broker_owned_replay(
    replay: &TreeReplayV1,
    policy: &NamespacePolicy,
) -> Result<(), AcceptanceError> {
    for node in &replay.nodes {
        if node.binding.uid != policy.broker_uid
            || node.binding.gid != policy.broker_gid
            || node.binding.mode & 0o022 != 0
        {
            return Err(invalid(format!(
                "prepared tree node {} is producer-writable or has wrong owner",
                node.path
            )));
        }
    }
    Ok(())
}

fn replay_tree(root_fd: RawFd) -> Result<TreeReplayV1, AcceptanceError> {
    let mut nodes = Vec::new();
    scan_open_node(root_fd, ".", 0, &mut nodes)?;
    nodes.sort_by(|left, right| left.path.cmp(&right.path));
    if nodes.is_empty() || nodes.len() > MAX_NODES {
        return Err(invalid("tree replay node count is outside the fixed bound"));
    }
    Ok(TreeReplayV1 {
        nodes,
        schema: "hepta_mac_descriptor_tree_replay_v1".to_string(),
    })
}

/// Flush every payload inode before its containing directory, then flush the
/// staging root.  Publication can therefore rely on the directory rename for
/// namespace durability instead of merely syncing the top-level name.
fn sync_open_tree(root_fd: RawFd) -> Result<(), AcceptanceError> {
    let root = snapshot_fd(root_fd, ".")?;
    let mut visited = 0_usize;
    sync_open_node(root_fd, root.dev(), 0, &mut visited)?;
    if visited == 0 || visited > MAX_NODES {
        return Err(invalid(
            "payload fsync traversal node count is outside the fixed bound",
        ));
    }
    Ok(())
}

fn sync_open_node(
    fd: RawFd,
    root_dev: u64,
    depth: usize,
    visited: &mut usize,
) -> Result<Snapshot, AcceptanceError> {
    if depth > MAX_DEPTH || *visited >= MAX_NODES {
        return Err(invalid(
            "payload fsync traversal exceeds its depth or node bound",
        ));
    }
    *visited += 1;
    let before = snapshot_fd(fd, "payload fsync node")?;
    if before.dev() != root_dev {
        return Err(invalid(
            "payload fsync traversal rejects cross-device descendants",
        ));
    }
    match before.kind {
        NodeKind::Directory => {
            for name in read_dir_names(fd)? {
                let name = name
                    .into_string()
                    .map_err(|_| invalid("payload fsync name is not UTF-8"))?;
                require_component(&name)?;
                let path_before = fstatat_snapshot(fd, &name)?;
                let child = match path_before.kind {
                    NodeKind::Directory => openat_directory(fd, &name)?,
                    NodeKind::RegularFile => openat_regular(fd, &name)?,
                };
                let opened_before = snapshot_fd(child.as_raw_fd(), &name)?;
                if path_before != opened_before {
                    return Err(invalid(
                        "payload child changed between fstatat and openat during fsync",
                    ));
                }
                let child_after = sync_open_node(child.as_raw_fd(), root_dev, depth + 1, visited)?;
                if fstatat_snapshot(fd, &name)? != child_after {
                    return Err(invalid(
                        "payload child changed before parent fsync completed",
                    ));
                }
            }
        }
        NodeKind::RegularFile if before.nlink() != 1 => {
            return Err(invalid(
                "payload fsync traversal rejects hardlinked regular files",
            ));
        }
        NodeKind::RegularFile => {}
    }
    duplicate_fd(fd)?.sync_all()?;
    let after = snapshot_fd(fd, "payload fsync node")?;
    if after != before {
        return Err(invalid("payload node changed during bottom-up fsync"));
    }
    Ok(after)
}

fn scan_open_node(
    fd: RawFd,
    path: &str,
    depth: usize,
    nodes: &mut Vec<TreeNodeV1>,
) -> Result<Snapshot, AcceptanceError> {
    if depth > MAX_DEPTH || nodes.len() >= MAX_NODES {
        return Err(invalid("tree replay exceeds its depth or node bound"));
    }
    let before = snapshot_fd(fd, path)?;
    let before_acl = acl_sha256(fd)?;
    let before_xattrs = xattrs_sha256(fd)?;
    let content_sha256 = match before.kind {
        NodeKind::Directory => {
            for name in read_dir_names(fd)? {
                let name = name
                    .into_string()
                    .map_err(|_| invalid("tree replay name is not UTF-8"))?;
                require_component(&name)?;
                let child_path = if path == "." {
                    name.clone()
                } else {
                    format!("{path}/{name}")
                };
                let path_before = fstatat_snapshot(fd, &name)?;
                let child = match path_before.kind {
                    NodeKind::Directory => openat_directory(fd, &name)?,
                    NodeKind::RegularFile => openat_regular(fd, &name)?,
                };
                let opened_before = snapshot_fd(child.as_raw_fd(), &child_path)?;
                if path_before != opened_before {
                    return Err(invalid("tree child changed between fstatat and openat"));
                }
                let scanned = scan_open_node(child.as_raw_fd(), &child_path, depth + 1, nodes)?;
                let path_after = fstatat_snapshot(fd, &name)?;
                if path_after != scanned {
                    return Err(invalid("tree child changed before parent replay completed"));
                }
            }
            None
        }
        NodeKind::RegularFile => {
            if before.nlink() != 1 {
                return Err(invalid("tree replay rejects hardlinked regular files"));
            }
            let file = duplicate_fd(fd)?;
            Some(hash_open_file(&file, before.size())?)
        }
    };
    let after_acl = acl_sha256(fd)?;
    let after_xattrs = xattrs_sha256(fd)?;
    let after = snapshot_fd(fd, path)?;
    if after != before || after_acl != before_acl || after_xattrs != before_xattrs {
        return Err(invalid("tree node changed during descriptor replay"));
    }
    nodes.push(TreeNodeV1 {
        acl_sha256: before_acl,
        binding: before.binding.clone(),
        content_sha256,
        kind: before.kind,
        path: path.to_string(),
        xattrs_sha256: before_xattrs,
    });
    Ok(after)
}

fn snapshot_fd(fd: RawFd, path: &str) -> Result<Snapshot, AcceptanceError> {
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    snapshot_from_stat(stat, path)
}

fn fstatat_snapshot(parent_fd: RawFd, name: &str) -> Result<Snapshot, AcceptanceError> {
    let name = c_component(name)?;
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    if unsafe {
        libc::fstatat(
            parent_fd,
            name.as_ptr(),
            &mut stat,
            libc::AT_SYMLINK_NOFOLLOW,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    snapshot_from_stat(stat, name.to_str().unwrap_or("node"))
}

fn snapshot_from_stat(stat: libc::stat, path: &str) -> Result<Snapshot, AcceptanceError> {
    let file_type = stat.st_mode & libc::S_IFMT;
    let kind = if file_type == libc::S_IFDIR {
        NodeKind::Directory
    } else if file_type == libc::S_IFREG {
        NodeKind::RegularFile
    } else {
        return Err(invalid(format!(
            "tree path {path} is a symlink or unsupported special node"
        )));
    };
    if stat.st_size < 0 {
        return Err(invalid("tree node has a negative size"));
    }
    Ok(Snapshot {
        binding: ObjectBindingV1 {
            ctime_nanoseconds: stat.st_ctime_nsec,
            ctime_seconds: stat.st_ctime,
            dev: stat.st_dev as u64,
            flags: stat.st_flags,
            gid: stat.st_gid,
            inode: stat.st_ino,
            mode: (stat.st_mode as u32) & 0o7777,
            mtime_nanoseconds: stat.st_mtime_nsec,
            mtime_seconds: stat.st_mtime,
            nlink: stat.st_nlink as u64,
            size: stat.st_size as u64,
            uid: stat.st_uid,
        },
        kind,
    })
}

fn open_publication_namespace(
    path: &Path,
    policy: &NamespacePolicy,
) -> Result<OpenedNamespace, AcceptanceError> {
    if !policy.privileged_mode() {
        return Ok(OpenedNamespace {
            namespace: open_absolute_directory(path)?,
            durability_ancestors: Vec::new(),
        });
    }

    let components = path.components().collect::<Vec<_>>();
    let [
        Component::RootDir,
        Component::Normal(volumes),
        Component::Normal(t5),
        Component::Normal(root_name),
        Component::Normal(publication),
    ] = components.as_slice()
    else {
        return Err(invalid(
            "live publication namespace must be /Volumes/T5/<fixed-root>/publication",
        ));
    };
    if volumes.as_bytes() != b"Volumes"
        || t5.as_bytes() != b"T5"
        || publication.as_bytes() != b"publication"
    {
        return Err(invalid(
            "live publication namespace is outside the fixed T5 layout",
        ));
    }
    let root_name = root_name
        .to_str()
        .ok_or_else(|| invalid("live qualification root name is not UTF-8"))?;
    let root_nonce = root_name
        .strip_prefix(LIVE_T5_ROOT_PREFIX)
        .ok_or_else(|| invalid("live qualification root name has the wrong fixed prefix"))?;
    require_nonce(root_nonce, "live qualification root nonce")?;

    let filesystem_root = open_absolute_directory(Path::new("/"))?;
    let volumes = openat_directory(filesystem_root.as_raw_fd(), "Volumes")?;
    let t5 = openat_directory(volumes.as_raw_fd(), "T5")?;
    verify_t5_volume_uuid(t5.as_raw_fd())?;
    let qualification_root = openat_directory(t5.as_raw_fd(), root_name)?;
    verify_private_namespace(&qualification_root, policy)?;
    let namespace = openat_directory(qualification_root.as_raw_fd(), "publication")?;

    Ok(OpenedNamespace {
        namespace,
        durability_ancestors: vec![qualification_root, t5, volumes, filesystem_root],
    })
}

fn open_barrier_namespace(
    path: &Path,
    policy: &NamespacePolicy,
) -> Result<OpenedNamespace, AcceptanceError> {
    if !policy.privileged_mode() {
        return Ok(OpenedNamespace {
            namespace: open_absolute_directory(path)?,
            durability_ancestors: Vec::new(),
        });
    }

    let components = path.components().collect::<Vec<_>>();
    let [
        Component::RootDir,
        Component::Normal(volumes),
        Component::Normal(t5),
        Component::Normal(root_name),
        Component::Normal(barrier),
    ] = components.as_slice()
    else {
        return Err(invalid(
            "live barrier namespace must be /Volumes/T5/<fixed-root>/barrier-journal",
        ));
    };
    if volumes.as_bytes() != b"Volumes"
        || t5.as_bytes() != b"T5"
        || barrier.as_bytes() != b"barrier-journal"
    {
        return Err(invalid(
            "live barrier namespace is outside the fixed T5 layout",
        ));
    }
    let root_name = root_name
        .to_str()
        .ok_or_else(|| invalid("live qualification root name is not UTF-8"))?;
    let root_nonce = root_name
        .strip_prefix(LIVE_T5_ROOT_PREFIX)
        .ok_or_else(|| invalid("live qualification root name has the wrong fixed prefix"))?;
    require_nonce(root_nonce, "live qualification root nonce")?;

    let filesystem_root = open_absolute_directory(Path::new("/"))?;
    let volumes = openat_directory(filesystem_root.as_raw_fd(), "Volumes")?;
    let t5 = openat_directory(volumes.as_raw_fd(), "T5")?;
    verify_t5_volume_uuid(t5.as_raw_fd())?;
    let qualification_root = openat_directory(t5.as_raw_fd(), root_name)?;
    verify_private_namespace(&qualification_root, policy)?;
    let namespace = openat_directory(qualification_root.as_raw_fd(), "barrier-journal")?;

    Ok(OpenedNamespace {
        namespace,
        durability_ancestors: vec![qualification_root, t5, volumes, filesystem_root],
    })
}

fn verify_t5_volume_uuid(fd: RawFd) -> Result<(), AcceptanceError> {
    let mut filesystem: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::fstatfs(fd, &mut filesystem) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let filesystem_type = unsafe { CStr::from_ptr(filesystem.f_fstypename.as_ptr()) }
        .to_str()
        .map_err(|_| invalid("T5 filesystem type is not UTF-8"))?;
    let mounted_on = unsafe { CStr::from_ptr(filesystem.f_mntonname.as_ptr()) }
        .to_str()
        .map_err(|_| invalid("T5 mountpoint is not UTF-8"))?;
    validate_t5_mount_semantics(filesystem.f_flags.into(), filesystem_type, mounted_on)?;

    let mut attributes = AttrList {
        bitmapcount: ATTR_BIT_MAP_COUNT,
        reserved: 0,
        commonattr: 0,
        volattr: ATTR_VOL_INFO | ATTR_VOL_UUID,
        dirattr: 0,
        fileattr: 0,
        forkattr: 0,
    };
    let mut buffer = VolumeUuidBuffer {
        length: 0,
        uuid: [0; 16],
    };
    if unsafe {
        libc::fgetattrlist(
            fd,
            (&mut attributes as *mut AttrList).cast(),
            (&mut buffer as *mut VolumeUuidBuffer).cast(),
            std::mem::size_of::<VolumeUuidBuffer>(),
            0,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    if buffer.length as usize != std::mem::size_of::<VolumeUuidBuffer>()
        || buffer.uuid != T5_VOLUME_UUID
    {
        return Err(invalid(
            "live publication volume UUID differs from the canonical T5 pin",
        ));
    }
    Ok(())
}

fn validate_t5_mount_semantics(
    flags: u64,
    filesystem_type: &str,
    mounted_on: &str,
) -> Result<(), AcceptanceError> {
    if flags & MNT_IGNORE_OWNERSHIP != 0
        || filesystem_type != "apfs"
        || mounted_on.as_bytes() != b"/Volumes/T5"
    {
        return Err(invalid(
            "canonical T5 must be APFS mounted exactly at /Volumes/T5 with ownership enabled",
        ));
    }
    Ok(())
}

/// Open an installed regular file only through root-owned, ACL-free directory
/// descriptors.  No component may be group/world writable, and the final
/// inode must be root:wheel with the exact requested mode.  This closes the
/// `proc_pidpath` pathname-replacement gap for an unprivileged operator: the
/// pathname that is hashed cannot be rebound without crossing the root TCB.
fn open_trusted_install_regular(
    path: &Path,
    expected_mode: u32,
    label: &str,
) -> Result<File, AcceptanceError> {
    let components = path.components().collect::<Vec<_>>();
    if components.len() < 2 || components[0] != Component::RootDir {
        return Err(invalid(format!(
            "{label} path must be absolute and non-root"
        )));
    }
    let mut names = Vec::with_capacity(components.len() - 1);
    for component in &components[1..] {
        match component {
            Component::Normal(name) => names.push(
                name.to_str()
                    .ok_or_else(|| invalid(format!("{label} path is not UTF-8")))?
                    .to_string(),
            ),
            _ => {
                return Err(invalid(format!(
                    "{label} path contains a relative, dot, or parent component"
                )));
            }
        }
    }
    let final_name = names
        .pop()
        .ok_or_else(|| invalid(format!("{label} path omits a final component")))?;
    let mut directory = open_absolute_directory(Path::new("/"))?;
    verify_trusted_install_directory(&directory, label)?;
    for name in names {
        let path_snapshot = fstatat_snapshot(directory.as_raw_fd(), &name)?;
        let child = openat_directory(directory.as_raw_fd(), &name)?;
        let opened_snapshot = snapshot_fd(child.as_raw_fd(), &name)?;
        if path_snapshot != opened_snapshot {
            return Err(invalid(format!(
                "{label} ancestor changed between descriptor checks"
            )));
        }
        verify_trusted_install_directory(&child, label)?;
        directory = child;
    }
    let path_snapshot = fstatat_snapshot(directory.as_raw_fd(), &final_name)?;
    let file = openat_regular(directory.as_raw_fd(), &final_name)?;
    let opened_snapshot = snapshot_fd(file.as_raw_fd(), label)?;
    if path_snapshot != opened_snapshot
        || opened_snapshot.kind != NodeKind::RegularFile
        || opened_snapshot.uid() != 0
        || opened_snapshot.gid() != 0
        || opened_snapshot.mode() != expected_mode
        || opened_snapshot.nlink() != 1
    {
        return Err(invalid(format!(
            "{label} is not the exact root:wheel unaliased installed inode and mode"
        )));
    }
    verify_acl_absent(file.as_raw_fd())?;
    Ok(file)
}

fn verify_trusted_install_directory(file: &File, label: &str) -> Result<(), AcceptanceError> {
    let snapshot = snapshot_fd(file.as_raw_fd(), label)?;
    if snapshot.kind != NodeKind::Directory
        || snapshot.uid() != 0
        || snapshot.gid() != 0
        || snapshot.mode() & 0o022 != 0
    {
        return Err(invalid(format!(
            "{label} has a non-root-owned or writable pathname ancestor"
        )));
    }
    verify_acl_absent(file.as_raw_fd())
}

fn open_absolute_directory(path: &Path) -> Result<File, AcceptanceError> {
    if !path.is_absolute() {
        return Err(invalid("broker namespace path must be absolute"));
    }
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| invalid("absolute directory contains NUL"))?;
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    file_from_fd(fd)
}

fn open_absolute_regular(path: &Path) -> Result<File, AcceptanceError> {
    if !path.is_absolute() {
        return Err(invalid("installed executable path must be absolute"));
    }
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| invalid("installed executable path contains NUL"))?;
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    file_from_fd(fd)
}

fn openat_directory(parent_fd: RawFd, name: &str) -> Result<File, AcceptanceError> {
    let name = c_component(name)?;
    let fd = unsafe {
        libc::openat(
            parent_fd,
            name.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    file_from_fd(fd)
}

fn openat_regular(parent_fd: RawFd, name: &str) -> Result<File, AcceptanceError> {
    let name = c_component(name)?;
    let fd = unsafe {
        libc::openat(
            parent_fd,
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    let file = file_from_fd(fd)?;
    if snapshot_fd(file.as_raw_fd(), name.to_str().unwrap_or("file"))?.kind != NodeKind::RegularFile
    {
        return Err(invalid(
            "openat regular-file operation yielded another node type",
        ));
    }
    Ok(file)
}

fn createat_file(parent_fd: RawFd, name: &str, mode: u32) -> Result<File, AcceptanceError> {
    let name = c_component(name)?;
    let fd = unsafe {
        libc::openat(
            parent_fd,
            name.as_ptr(),
            libc::O_RDWR | libc::O_CREAT | libc::O_EXCL | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            mode as libc::c_uint,
        )
    };
    file_from_fd(fd)
}

fn duplicate_fd(fd: RawFd) -> Result<File, AcceptanceError> {
    file_from_fd(unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) })
}

fn file_from_fd(fd: libc::c_int) -> Result<File, AcceptanceError> {
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn rename_noreplace(
    source_parent_fd: RawFd,
    source_name: &str,
    destination_parent_fd: RawFd,
    destination_name: &str,
) -> Result<(), AcceptanceError> {
    let source_name = c_component(source_name)?;
    let destination_name = c_component(destination_name)?;
    let rc = unsafe {
        renameatx_np(
            source_parent_fd,
            source_name.as_ptr(),
            destination_parent_fd,
            destination_name.as_ptr(),
            RENAME_EXCL,
        )
    };
    if rc != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}

fn read_dir_names(fd: RawFd) -> Result<Vec<OsString>, AcceptanceError> {
    let duplicate = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
    if duplicate < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let directory = unsafe { libc::fdopendir(duplicate) };
    if directory.is_null() {
        unsafe { libc::close(duplicate) };
        return Err(std::io::Error::last_os_error().into());
    }
    if unsafe { libc::lseek(duplicate, 0, libc::SEEK_SET) } < 0 {
        let error = std::io::Error::last_os_error();
        unsafe { libc::closedir(directory) };
        return Err(error.into());
    }
    let mut names = Vec::new();
    loop {
        set_errno(0);
        let entry = unsafe { libc::readdir(directory) };
        if entry.is_null() {
            let error = get_errno();
            let close_rc = unsafe { libc::closedir(directory) };
            if error != 0 {
                return Err(std::io::Error::from_raw_os_error(error).into());
            }
            if close_rc != 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            break;
        }
        let raw = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
        if raw != b"." && raw != b".." {
            names.push(OsString::from_vec(raw.to_vec()));
        }
    }
    names.sort();
    Ok(names)
}

fn acl_sha256(fd: RawFd) -> Result<String, AcceptanceError> {
    const ACL_TYPE_EXTENDED: libc::c_int = 0x0000_0100;
    let acl = unsafe { acl_get_fd_np(fd, ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ENOENT) {
            return Ok(sha256(b"acl:none\n"));
        }
        return Err(error.into());
    }
    let mut length = 0;
    let text = unsafe { acl_to_text(acl, &mut length) };
    if text.is_null() || length < 0 {
        let error = std::io::Error::last_os_error();
        unsafe { acl_free(acl) };
        return Err(error.into());
    }
    let bytes = unsafe { std::slice::from_raw_parts(text.cast::<u8>(), length as usize) };
    let digest = sha256(bytes);
    if unsafe { acl_free(text.cast()) } != 0 || unsafe { acl_free(acl) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(digest)
}

fn verify_acl_absent(fd: RawFd) -> Result<(), AcceptanceError> {
    const ACL_FIRST_ENTRY: libc::c_int = 0;
    const ACL_TYPE_EXTENDED: libc::c_int = 0x0000_0100;
    let acl = unsafe { acl_get_fd_np(fd, ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ENOENT) {
            return Ok(());
        }
        return Err(error.into());
    }
    let mut entry = std::ptr::null_mut();
    let rc = unsafe { acl_get_entry(acl, ACL_FIRST_ENTRY, &mut entry) };
    let error = std::io::Error::last_os_error();
    if unsafe { acl_free(acl) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    match rc {
        0 => Err(invalid("broker namespace has an extended ACL")),
        -1 if error.raw_os_error() == Some(libc::EINVAL) => Ok(()),
        _ => Err(error.into()),
    }
}

fn xattrs_sha256(fd: RawFd) -> Result<String, AcceptanceError> {
    let required = unsafe { libc::flistxattr(fd, std::ptr::null_mut(), 0, 0) };
    if required < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let mut names = vec![0_u8; required as usize];
    if required > 0 {
        let received = unsafe { libc::flistxattr(fd, names.as_mut_ptr().cast(), names.len(), 0) };
        if received < 0 || received as usize != names.len() {
            return Err(if received < 0 {
                std::io::Error::last_os_error().into()
            } else {
                invalid("xattr name inventory changed while read")
            });
        }
    }
    let mut parsed = names
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    parsed.sort();
    let mut inventory = Vec::new();
    for raw_name in parsed {
        let name = CString::new(raw_name.clone())
            .map_err(|_| invalid("xattr name contains an embedded NUL"))?;
        let size = unsafe { libc::fgetxattr(fd, name.as_ptr(), std::ptr::null_mut(), 0, 0, 0) };
        if size < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let mut value = vec![0_u8; size as usize];
        if size > 0 {
            let received = unsafe {
                libc::fgetxattr(
                    fd,
                    name.as_ptr(),
                    value.as_mut_ptr().cast(),
                    value.len(),
                    0,
                    0,
                )
            };
            if received < 0 || received as usize != value.len() {
                return Err(if received < 0 {
                    std::io::Error::last_os_error().into()
                } else {
                    invalid("xattr value changed while read")
                });
            }
        }
        inventory.extend_from_slice(&(raw_name.len() as u64).to_be_bytes());
        inventory.extend_from_slice(&raw_name);
        inventory.extend_from_slice(&(value.len() as u64).to_be_bytes());
        inventory.extend_from_slice(&value);
    }
    Ok(sha256(&inventory))
}

fn hash_open_file(file: &File, expected_size: u64) -> Result<String, AcceptanceError> {
    if expected_size > MAX_ARTIFACT_BYTES {
        return Err(invalid("tree file exceeds the 2 GiB bound"));
    }
    let bytes = read_open_file(file)?;
    if bytes.len() as u64 != expected_size {
        return Err(invalid("tree file size changed while read"));
    }
    Ok(sha256(&bytes))
}

fn read_open_file(file: &File) -> Result<Vec<u8>, AcceptanceError> {
    let mut file = file.try_clone()?;
    file.seek(SeekFrom::Start(0))?;
    let mut bytes = Vec::new();
    file.take(MAX_ARTIFACT_BYTES.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_ARTIFACT_BYTES {
        return Err(invalid("file exceeds the 2 GiB bound"));
    }
    Ok(bytes)
}

fn peer_audit_token(socket_fd: RawFd) -> Result<AuditToken, AcceptanceError> {
    let mut token = AuditToken { val: [0; 8] };
    let mut length = std::mem::size_of::<AuditToken>() as libc::socklen_t;
    if unsafe {
        libc::getsockopt(
            socket_fd,
            SOL_LOCAL,
            LOCAL_PEERTOKEN,
            (&mut token as *mut AuditToken).cast(),
            &mut length,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    if length as usize != std::mem::size_of::<AuditToken>() {
        return Err(invalid("kernel returned a truncated peer audit token"));
    }
    Ok(token)
}

fn peer_pid(socket_fd: RawFd) -> Result<i32, AcceptanceError> {
    let mut pid: libc::pid_t = 0;
    let mut length = std::mem::size_of::<libc::pid_t>() as libc::socklen_t;
    if unsafe {
        libc::getsockopt(
            socket_fd,
            SOL_LOCAL,
            LOCAL_PEERPID,
            (&mut pid as *mut libc::pid_t).cast(),
            &mut length,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    if length as usize != std::mem::size_of::<libc::pid_t>() {
        return Err(invalid("kernel returned a truncated peer PID"));
    }
    Ok(pid)
}

fn executable_path_for_token(mut token: AuditToken) -> Result<String, AcceptanceError> {
    const PROC_PIDPATHINFO_MAXSIZE: usize = 4_096;
    let mut path = vec![0_u8; PROC_PIDPATHINFO_MAXSIZE];
    let length =
        unsafe { proc_pidpath_audittoken(&mut token, path.as_mut_ptr().cast(), path.len() as u32) };
    if length <= 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let length = length as usize;
    if length >= path.len() {
        return Err(invalid("peer executable path is truncated"));
    }
    path.truncate(length);
    if path.last() == Some(&0) {
        path.pop();
    }
    String::from_utf8(path).map_err(|_| invalid("peer executable path is not UTF-8"))
}

fn audit_token_sha256(token: AuditToken) -> String {
    let mut bytes = Vec::with_capacity(32);
    for value in token.val {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    sha256(&bytes)
}

fn record_name(sequence: u64) -> String {
    format!("{sequence:0RECORD_DIGITS$}{RECORD_SUFFIX}")
}

fn c_component(name: &str) -> Result<CString, AcceptanceError> {
    require_component(name)?;
    CString::new(name).map_err(|_| invalid("path component contains NUL"))
}

fn require_component(name: &str) -> Result<(), AcceptanceError> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.len() > 240
        || name.contains('/')
        || name.contains('\0')
        || name.contains('\n')
        || name.contains('\r')
        || name.contains('\t')
    {
        return Err(invalid("fixed-protocol path component is unsafe"));
    }
    Ok(())
}

fn require_digest(value: &str, label: &str) -> Result<(), AcceptanceError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(invalid(format!("{label} SHA-256 is malformed")));
    }
    Ok(())
}

fn require_nonce(value: &str, label: &str) -> Result<(), AcceptanceError> {
    require_digest(value, label)
}

fn require_uuid(value: &str, label: &str) -> Result<(), AcceptanceError> {
    let bytes = value.as_bytes();
    if bytes.len() != 36
        || bytes.iter().enumerate().any(|(index, byte)| match index {
            8 | 13 | 18 | 23 => *byte != b'-',
            _ => !byte.is_ascii_hexdigit(),
        })
    {
        return Err(invalid(format!("{label} is not a canonical UUID")));
    }
    Ok(())
}

fn current_boot_session_uuid() -> Result<String, AcceptanceError> {
    let name = CString::new("kern.bootsessionuuid").expect("literal has no NUL");
    let mut required = 0_usize;
    if unsafe {
        libc::sysctlbyname(
            name.as_ptr(),
            std::ptr::null_mut(),
            &mut required,
            std::ptr::null_mut(),
            0,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    if !(37..=128).contains(&required) {
        return Err(invalid("boot session UUID sysctl size is invalid"));
    }
    let mut bytes = vec![0_u8; required];
    if unsafe {
        libc::sysctlbyname(
            name.as_ptr(),
            bytes.as_mut_ptr().cast(),
            &mut required,
            std::ptr::null_mut(),
            0,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    bytes.truncate(required);
    let trailing_nul = bytes
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| invalid("boot session UUID sysctl omitted its terminator"))?;
    if bytes[trailing_nul..].iter().any(|byte| *byte != 0) {
        return Err(invalid("boot session UUID sysctl has trailing bytes"));
    }
    let value = std::str::from_utf8(&bytes[..trailing_nul])
        .map_err(|_| invalid("boot session UUID sysctl is not UTF-8"))?
        .to_ascii_lowercase();
    require_uuid(&value, "boot session UUID")?;
    Ok(value)
}

fn get_errno() -> libc::c_int {
    unsafe { *libc::__error() }
}

fn set_errno(value: libc::c_int) {
    unsafe { *libc::__error() = value };
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

#[link(name = "bsm")]
unsafe extern "C" {
    fn audit_token_to_asid(token: AuditToken) -> u32;
    fn audit_token_to_egid(token: AuditToken) -> u32;
    fn audit_token_to_euid(token: AuditToken) -> u32;
    fn audit_token_to_pid(token: AuditToken) -> libc::pid_t;
    fn audit_token_to_pidversion(token: AuditToken) -> libc::c_int;
}

unsafe extern "C" {
    fn acl_free(object: *mut libc::c_void) -> libc::c_int;
    fn acl_get_entry(
        acl: *mut libc::c_void,
        entry_id: libc::c_int,
        entry_p: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn acl_get_fd_np(fd: libc::c_int, acl_type: libc::c_int) -> *mut libc::c_void;
    fn acl_to_text(acl: *mut libc::c_void, length: *mut libc::ssize_t) -> *mut libc::c_char;
    fn proc_pidpath_audittoken(
        token: *mut AuditToken,
        buffer: *mut libc::c_void,
        buffer_size: u32,
    ) -> libc::c_int;
    fn renameatx_np(
        fromfd: libc::c_int,
        from: *const libc::c_char,
        tofd: libc::c_int,
        to: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::net::UnixStream;

    use tempfile::TempDir;

    use super::*;

    fn private_root() -> (TempDir, NamespacePolicy) {
        let root = tempfile::Builder::new()
            .prefix("hepta-mac-priv-broker-")
            .tempdir()
            .expect("private root");
        fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))
            .expect("private root mode");
        (root, NamespacePolicy::rootless_test())
    }

    fn peer(policy: &NamespacePolicy) -> AuthenticatedPeerV1 {
        AuthenticatedPeerV1 {
            audit_session_id: 7,
            audit_token_sha256: "a".repeat(64),
            effective_gid: unsafe { libc::getegid() },
            effective_uid: policy.operator_uid,
            executable_path: policy.client_executable_path.clone(),
            executable_sha256: policy.client_executable_sha256.clone(),
            pid: 123,
            pid_version: 4,
        }
    }

    fn fixture(root: &Path, name: &str, policy: &NamespacePolicy) {
        let staging = root.join(name);
        fs::create_dir(&staging).expect("staging");
        fs::write(staging.join("payload"), b"sealed payload\n").expect("payload");
        fs::set_permissions(staging.join("payload"), fs::Permissions::from_mode(0o400))
            .expect("payload mode");
        fs::set_permissions(&staging, fs::Permissions::from_mode(0o500)).expect("staging mode");
        let replay = replay_tree(
            open_absolute_directory(&staging)
                .expect("open staging")
                .as_raw_fd(),
        )
        .expect("replay");
        verify_broker_owned_replay(&replay, policy).expect("owned replay");
    }

    fn replay_digest(path: &Path) -> String {
        let file = open_absolute_directory(path).expect("open replay root");
        replay_tree(file.as_raw_fd())
            .expect("replay")
            .digest()
            .expect("digest")
    }

    fn binding(uid: u32, gid: u32, mode: u32) -> ObjectBindingV1 {
        ObjectBindingV1 {
            ctime_nanoseconds: 2,
            ctime_seconds: 1,
            dev: 7,
            flags: 0,
            gid,
            inode: 11,
            mode,
            mtime_nanoseconds: 4,
            mtime_seconds: 3,
            nlink: 1,
            size: 0,
            uid,
        }
    }

    const TEST_VOLUME_UUID: &str = "92f75e11-f0e7-4187-9340-1b807579568b";

    fn begin_barrier(
        journal: &BarrierJournal,
        nonce: &str,
        peer: &AuthenticatedPeerV1,
        source: &ObjectBindingV1,
    ) -> Result<BarrierVerificationV1, AcceptanceError> {
        journal.begin(
            nonce,
            peer,
            binding(0, 0, 0o700),
            TEST_VOLUME_UUID,
            binding(0, 0, 0o700),
            source.clone(),
        )
    }

    fn drain_for(
        journal: &BarrierJournal,
        digit: char,
        source_before: &ObjectBindingV1,
        source_after: &ObjectBindingV1,
    ) -> HandleDrainProofV1 {
        let next =
            char::from_digit((digit.to_digit(16).expect("hex") + 1) % 16, 16).expect("next hex");
        let envelopes = journal.read_and_validate().expect("read acquired facts");
        let acquired = envelopes.first().expect("acquired envelope");
        let (volume_uuid, mountpoint, challenge) = match &acquired.record.event {
            BarrierEventV1::Acquired {
                dedicated_volume_uuid,
                mountpoint_underlying,
                operation_challenge_sha256,
                ..
            } => (
                dedicated_volume_uuid.clone(),
                mountpoint_underlying.clone(),
                operation_challenge_sha256.clone(),
            ),
            _ => panic!("first record is acquired"),
        };
        HandleDrainProofV1 {
            apfs_unmount_barrier: ApfsUnmountBarrierProofV1 {
                barrier_boot_session_uuid: acquired.record.boot_session_uuid.clone(),
                barrier_epoch_nonce: acquired.record.epoch_nonce.clone(),
                clean_unmount_receipt_sha256: digit.to_string().repeat(64),
                clean_unmount_succeeded: true,
                forced_unmount: false,
                kernel_wide_holder_gate: true,
                mount_parent_after: binding(0, 0, 0o700),
                mount_parent_before: binding(0, 0, 0o700),
                mountpoint_underlying_after: mountpoint.clone(),
                mountpoint_underlying_before: mountpoint,
                operation_challenge_sha256: challenge,
                owners_enabled_after: true,
                owners_enabled_before: true,
                read_only_remount_receipt_sha256: next.to_string().repeat(64),
                read_only_volume_after: true,
                schema: "hepta_mac_apfs_unmount_barrier_proof_v1".to_string(),
                source_binding_after: source_after.clone(),
                source_binding_before: source_before.clone(),
                source_closure_sha256: "f".repeat(64),
                volume_uuid_after: volume_uuid.clone(),
                volume_uuid_before: volume_uuid,
                writable_media_after: false,
                writable_volume_after: false,
            },
            content_topology_after_sha256: "f".repeat(64),
            content_topology_before_sha256: "f".repeat(64),
            isolated_exact_replay_sha256: next.to_string().repeat(64),
            original_exact_replay_sha256: "d".repeat(64),
            process_visible_diagnostics: ProcessVisibleHolderDiagnosticsV1 {
                advisory_only: true,
                all_pid_gt_zero_scanned_as_root: true,
                cwd_or_root_vnode_count: 0,
                enumeration_non_atomic_limitation_bound: true,
                external_holder_count: 0,
                fileport_vnode_count: 0,
                kernel_pid_zero_region_excluded: true,
                mapped_vnode_count: 0,
                opaque_vm_submap_count: 7,
                open_vnode_count: 0,
                pid_identity_reuse_detected: false,
                pid_start_identities_sha256: "e".repeat(64),
                producer_process_count: 0,
                region_vnode_race_limitation_bound: true,
                scan_complete: true,
                scan_one_receipt_sha256: digit.to_string().repeat(64),
                scan_two_receipt_sha256: next.to_string().repeat(64),
                unknown_or_inaccessible_process_count: 0,
                vm_submap_limitation_bound: true,
                writable_shared_mapping_count: 0,
                writable_vnode_fd_count: 0,
            },
            writer_launchd_disabled_and_absent: true,
        }
    }

    fn conversion(policy: &NamespacePolicy) -> OwnershipConversionV1 {
        OwnershipConversionV1 {
            legacy_writer_uid: policy.legacy_writer_uid,
            quarantine_gid: 0,
            quarantine_uid: 0,
            schema: "hepta_mac_source_ownership_conversion_v1".to_string(),
            target_producer_uid: policy.producer_uid,
        }
    }

    fn advance_to_cutover(
        journal: &BarrierJournal,
        peer: &AuthenticatedPeerV1,
        policy: &NamespacePolicy,
        original: &ObjectBindingV1,
        isolated: &ObjectBindingV1,
    ) {
        journal
            .transition(
                peer,
                BarrierEventV1::SourceIsolated {
                    drain: drain_for(journal, '2', original, isolated),
                    isolated: isolated.clone(),
                    metadata_receipt_sha256: "3".repeat(64),
                    original: original.clone(),
                    ownership_conversion: conversion(policy),
                },
            )
            .expect("source isolated");
        journal
            .transition(
                peer,
                BarrierEventV1::SnapshotPublished {
                    snapshot_receipt_sha256: "4".repeat(64),
                    source_binding: isolated.clone(),
                },
            )
            .expect("snapshot");
        journal
            .transition(
                peer,
                BarrierEventV1::CanaryPassed {
                    canary_receipt_sha256: "5".repeat(64),
                    snapshot_receipt_sha256: "4".repeat(64),
                },
            )
            .expect("canary");
        journal
            .transition(
                peer,
                BarrierEventV1::CutoverPassed {
                    cutover_receipt_sha256: "6".repeat(64),
                },
            )
            .expect("cutover");
    }

    #[test]
    fn kernel_peer_token_uid_gid_pid_and_exact_bytes_are_cross_checked() {
        let (left, _right) = UnixStream::pair().expect("socket pair");
        let executable = std::env::current_exe().expect("current executable");
        let file = open_absolute_regular(&executable).expect("open current executable");
        let size = file.metadata().expect("metadata").len();
        let digest = hash_open_file(&file, size).expect("hash executable");
        let policy = NamespacePolicy::rootless_test();
        assert_eq!(policy.client_executable_sha256, digest);
        let authenticated = authenticate_connected_peer(left.as_raw_fd(), &policy)
            .expect("authenticate socket peer");
        assert_eq!(authenticated.effective_uid, unsafe { libc::geteuid() });
        assert_eq!(authenticated.pid, std::process::id() as i32);
        assert_eq!(authenticated.executable_sha256, digest);
        let mut wrong_uid = policy.clone();
        wrong_uid.operator_uid = unsafe { libc::geteuid() }.saturating_add(1);
        assert!(authenticate_connected_peer(left.as_raw_fd(), &wrong_uid).is_err());
        let mut wrong_digest = policy;
        wrong_digest.client_executable_sha256 = "0".repeat(64);
        assert!(authenticate_connected_peer(left.as_raw_fd(), &wrong_digest).is_err());
    }

    #[test]
    fn atomic_exclusive_publish_and_post_publish_inode_replay_fail_closed() {
        let (root, policy) = private_root();
        let peer = peer(&policy);
        fixture(root.path(), ".incoming-first", &policy);
        let digest = replay_digest(&root.path().join(".incoming-first"));
        let receipt = publish_prepared_directory(
            root.path(),
            ".incoming-first",
            "hepta-final",
            &digest,
            &peer,
            &policy,
        )
        .expect("exclusive publication");
        assert_eq!(receipt.pre_publish_replay_sha256, digest);
        assert_eq!(
            receipt.pre_publish_replay_sha256,
            receipt.post_publish_replay_sha256
        );
        assert!(receipt.rename_exclusive);
        assert!(!receipt.authority_granted);

        fixture(root.path(), ".incoming-second", &policy);
        let second = replay_digest(&root.path().join(".incoming-second"));
        assert!(
            publish_prepared_directory(
                root.path(),
                ".incoming-second",
                "hepta-final",
                &second,
                &peer,
                &policy,
            )
            .is_err()
        );
        assert!(root.path().join(".incoming-second").is_dir());

        fixture(root.path(), ".incoming-raced", &policy);
        let raced = replay_digest(&root.path().join(".incoming-raced"));
        let root_path = root.path().to_path_buf();
        assert!(
            publish_prepared_directory_with_hook(
                root.path(),
                ".incoming-raced",
                "hepta-raced",
                &raced,
                &peer,
                &policy,
                move || {
                    fs::rename(root_path.join("hepta-raced"), root_path.join("displaced"))?;
                    fs::create_dir(root_path.join("hepta-raced"))?;
                    fs::set_permissions(
                        root_path.join("hepta-raced"),
                        fs::Permissions::from_mode(0o500),
                    )?;
                    Ok(())
                },
            )
            .is_err()
        );
    }

    #[test]
    fn descriptor_replay_rejects_symlink_hardlink_and_external_digest_drift() {
        let (root, policy) = private_root();
        let peer = peer(&policy);
        fixture(root.path(), ".incoming-symlink", &policy);
        fs::set_permissions(
            root.path().join(".incoming-symlink"),
            fs::Permissions::from_mode(0o700),
        )
        .expect("open symlink fixture for mutation");
        std::os::unix::fs::symlink("payload", root.path().join(".incoming-symlink/link"))
            .expect("symlink");
        fs::set_permissions(
            root.path().join(".incoming-symlink"),
            fs::Permissions::from_mode(0o500),
        )
        .expect("reseal symlink fixture");
        assert!(replay_digest_result(&root.path().join(".incoming-symlink")).is_err());

        fixture(root.path(), ".incoming-hardlink", &policy);
        fs::set_permissions(
            root.path().join(".incoming-hardlink"),
            fs::Permissions::from_mode(0o700),
        )
        .expect("open hardlink fixture for mutation");
        fs::hard_link(
            root.path().join(".incoming-hardlink/payload"),
            root.path().join(".incoming-hardlink/alias"),
        )
        .expect("hardlink");
        fs::set_permissions(
            root.path().join(".incoming-hardlink"),
            fs::Permissions::from_mode(0o500),
        )
        .expect("reseal hardlink fixture");
        assert!(replay_digest_result(&root.path().join(".incoming-hardlink")).is_err());

        fixture(root.path(), ".incoming-pin", &policy);
        assert!(
            publish_prepared_directory(
                root.path(),
                ".incoming-pin",
                "hepta-pin",
                &"9".repeat(64),
                &peer,
                &policy,
            )
            .is_err()
        );
    }

    #[test]
    fn durable_publication_has_precommit_mechanism_and_terminal_receipts() {
        let (root, policy) = private_root();
        let peer = peer(&policy);
        fixture(root.path(), ".incoming-durable", &policy);
        let digest = replay_digest(&root.path().join(".incoming-durable"));
        let operation_nonce = "1".repeat(64);
        let sealed = qualify_prepared_directory(
            root.path(),
            ".incoming-durable",
            "hepta-durable",
            &digest,
            &operation_nonce,
            &peer,
            &policy,
        )
        .expect("durable publication");
        assert!(!sealed.publication_receipt.authority_granted);
        assert!(!sealed.qualification_receipt.live_authority);
        assert!(root.path().join(&sealed.prepared_record_name).is_file());
        assert!(root.path().join(&sealed.publication_receipt_name).is_file());
        assert!(
            root.path()
                .join(&sealed.qualification_receipt_name)
                .is_file()
        );
        let recovered = recover_prepared_publication(root.path(), &operation_nonce, &peer, &policy)
            .expect("idempotent terminal recovery");
        assert_eq!(recovered, sealed);
    }

    #[test]
    fn independent_verifier_rejects_terminal_only_mechanism_tamper_and_final_tamper() {
        let (terminal_root, terminal_policy) = private_root();
        let terminal_peer = peer(&terminal_policy);
        fixture(
            terminal_root.path(),
            ".incoming-terminal-only",
            &terminal_policy,
        );
        let terminal_digest = replay_digest(&terminal_root.path().join(".incoming-terminal-only"));
        let terminal_nonce = "3".repeat(64);
        let terminal = qualify_prepared_directory(
            terminal_root.path(),
            ".incoming-terminal-only",
            "hepta-terminal-only",
            &terminal_digest,
            &terminal_nonce,
            &terminal_peer,
            &terminal_policy,
        )
        .expect("terminal-only fixture");
        fs::remove_file(terminal_root.path().join(terminal.prepared_record_name))
            .expect("remove prepared record");
        assert!(
            verify_sealed_publication(terminal_root.path(), &terminal_nonce, &terminal_policy,)
                .is_err()
        );

        let (mechanism_root, mechanism_policy) = private_root();
        let mechanism_peer = peer(&mechanism_policy);
        fixture(
            mechanism_root.path(),
            ".incoming-mechanism-tamper",
            &mechanism_policy,
        );
        let mechanism_digest =
            replay_digest(&mechanism_root.path().join(".incoming-mechanism-tamper"));
        let mechanism_nonce = "4".repeat(64);
        let mechanism = qualify_prepared_directory(
            mechanism_root.path(),
            ".incoming-mechanism-tamper",
            "hepta-mechanism-tamper",
            &mechanism_digest,
            &mechanism_nonce,
            &mechanism_peer,
            &mechanism_policy,
        )
        .expect("mechanism-tamper fixture");
        let mechanism_path = mechanism_root
            .path()
            .join(mechanism.publication_receipt_name);
        fs::set_permissions(&mechanism_path, fs::Permissions::from_mode(0o600))
            .expect("make mechanism writable");
        fs::write(&mechanism_path, b"{}").expect("tamper mechanism");
        assert!(
            verify_sealed_publication(mechanism_root.path(), &mechanism_nonce, &mechanism_policy,)
                .is_err()
        );

        let (payload_root, payload_policy) = private_root();
        let payload_peer = peer(&payload_policy);
        fixture(
            payload_root.path(),
            ".incoming-payload-tamper",
            &payload_policy,
        );
        let payload_digest = replay_digest(&payload_root.path().join(".incoming-payload-tamper"));
        let payload_nonce = "5".repeat(64);
        qualify_prepared_directory(
            payload_root.path(),
            ".incoming-payload-tamper",
            "hepta-payload-tamper",
            &payload_digest,
            &payload_nonce,
            &payload_peer,
            &payload_policy,
        )
        .expect("payload-tamper fixture");
        let final_root = payload_root.path().join("hepta-payload-tamper");
        fs::set_permissions(&final_root, fs::Permissions::from_mode(0o700))
            .expect("make final root writable");
        let payload = final_root.join("payload");
        fs::set_permissions(&payload, fs::Permissions::from_mode(0o600))
            .expect("make final payload writable");
        fs::write(payload, b"tampered\n").expect("tamper final payload");
        assert!(
            verify_sealed_publication(payload_root.path(), &payload_nonce, &payload_policy,)
                .is_err()
        );
    }

    #[test]
    fn recovery_never_repeats_rename_and_seals_only_exact_orphan_final() {
        let (root, policy) = private_root();
        let peer = peer(&policy);
        fixture(root.path(), ".incoming-recovery", &policy);
        let digest = replay_digest(&root.path().join(".incoming-recovery"));
        let operation_nonce = "2".repeat(64);
        let (_prepared, _prepared_sha256) = prepare_publication(
            root.path(),
            ".incoming-recovery",
            "hepta-recovery",
            &digest,
            &operation_nonce,
            &peer,
            &policy,
        )
        .expect("precommit");
        assert!(
            recover_prepared_publication(root.path(), &operation_nonce, &peer, &policy,).is_err()
        );
        assert!(root.path().join(".incoming-recovery").is_dir());
        assert!(!root.path().join("hepta-recovery").exists());

        let namespace = open_absolute_directory(root.path()).expect("namespace");
        rename_noreplace(
            namespace.as_raw_fd(),
            ".incoming-recovery",
            namespace.as_raw_fd(),
            "hepta-recovery",
        )
        .expect("simulated rename before crash");
        namespace.sync_all().expect("sync simulated rename");
        assert!(
            recover_prepared_publication(root.path(), &operation_nonce, &peer, &policy).is_err()
        );
        let mut recovery_peer = peer.clone();
        recovery_peer.audit_session_id = peer.audit_session_id.saturating_add(1);
        recovery_peer.audit_token_sha256 = "e".repeat(64);
        let recovered =
            recover_prepared_publication(root.path(), &operation_nonce, &recovery_peer, &policy)
                .expect("seal exact orphan final with fresh peer");
        assert!(!recovered.qualification_receipt.live_authority);
        assert!(recovered.qualification_receipt.recovered_orphan_final);
        assert_eq!(
            recovered
                .qualification_receipt
                .recovery_peer_audit_token_sha256
                .as_deref(),
            Some(recovery_peer.audit_token_sha256.as_str())
        );
        assert!(
            root.path()
                .join(&recovered.qualification_receipt_name)
                .is_file()
        );
        assert!(verify_sealed_publication(root.path(), &operation_nonce, &policy).is_ok());

        let terminal_path = root.path().join(&recovered.qualification_receipt_name);
        let mut terminal: PrivilegedQualificationReceiptV1 =
            serde_json::from_slice(&fs::read(&terminal_path).expect("read recovery terminal"))
                .expect("parse recovery terminal");
        terminal.recovery_challenge_sha256 = Some("0".repeat(64));
        fs::set_permissions(&terminal_path, fs::Permissions::from_mode(0o600))
            .expect("open recovery terminal for negative tamper");
        fs::write(
            &terminal_path,
            canonical_json(&terminal).expect("canonical tampered terminal"),
        )
        .expect("write tampered terminal");
        fs::set_permissions(&terminal_path, fs::Permissions::from_mode(0o400))
            .expect("reseal tampered terminal");
        assert!(verify_sealed_publication(root.path(), &operation_nonce, &policy).is_err());
    }

    fn replay_digest_result(path: &Path) -> Result<String, AcceptanceError> {
        let file = open_absolute_directory(path)?;
        replay_tree(file.as_raw_fd())?.digest()
    }

    #[test]
    fn barrier_requires_exact_full_cycle_and_fresh_recutover_evidence() {
        let (root, policy) = private_root();
        let peer = peer(&policy);
        let journal = BarrierJournal::open(root.path(), policy.clone()).expect("journal");
        let original = binding(policy.legacy_writer_uid, unsafe { libc::getegid() }, 0o700);
        let isolated = binding(0, 0, 0o700);
        let begun = begin_barrier(&journal, &"1".repeat(64), &peer, &original).expect("begin");
        assert!(!begun.admissions_closed);
        assert_eq!(
            begun.admission_state,
            "mechanism_intent_only_no_native_holder"
        );
        assert!(!begun.live_authority);
        assert_eq!(begun.current_phase, BarrierPhaseV1::Acquired);

        assert!(
            journal
                .transition(
                    &peer,
                    BarrierEventV1::SnapshotPublished {
                        snapshot_receipt_sha256: "d".repeat(64),
                        source_binding: isolated.clone(),
                    },
                )
                .is_err()
        );

        journal
            .transition(
                &peer,
                BarrierEventV1::SourceIsolated {
                    drain: drain_for(&journal, '2', &original, &isolated),
                    isolated: isolated.clone(),
                    metadata_receipt_sha256: "3".repeat(64),
                    original: original.clone(),
                    ownership_conversion: conversion(&policy),
                },
            )
            .expect("source isolated");
        journal
            .transition(
                &peer,
                BarrierEventV1::SnapshotPublished {
                    snapshot_receipt_sha256: "4".repeat(64),
                    source_binding: isolated.clone(),
                },
            )
            .expect("snapshot");
        journal
            .transition(
                &peer,
                BarrierEventV1::CanaryPassed {
                    canary_receipt_sha256: "5".repeat(64),
                    snapshot_receipt_sha256: "4".repeat(64),
                },
            )
            .expect("canary");
        journal
            .transition(
                &peer,
                BarrierEventV1::CutoverPassed {
                    cutover_receipt_sha256: "6".repeat(64),
                },
            )
            .expect("cutover");
        journal
            .transition(
                &peer,
                BarrierEventV1::RollbackRestored {
                    restored: original.clone(),
                    restored_replay_sha256: "d".repeat(64),
                    rollback_receipt_sha256: "7".repeat(64),
                },
            )
            .expect("rollback");

        assert!(
            journal
                .transition(
                    &peer,
                    BarrierEventV1::RecutoverSourceIsolated {
                        drain: drain_for(&journal, '2', &original, &isolated),
                        freshness_nonce: "8".repeat(64),
                        isolated: isolated.clone(),
                        metadata_receipt_sha256: "3".repeat(64),
                        ownership_conversion: conversion(&policy),
                    },
                )
                .is_err()
        );
        journal
            .transition(
                &peer,
                BarrierEventV1::RecutoverSourceIsolated {
                    drain: drain_for(&journal, '8', &original, &isolated),
                    freshness_nonce: "9".repeat(64),
                    isolated: isolated.clone(),
                    metadata_receipt_sha256: "a".repeat(64),
                    ownership_conversion: conversion(&policy),
                },
            )
            .expect("fresh isolation");
        assert!(
            journal
                .transition(
                    &peer,
                    BarrierEventV1::RecutoverSnapshotPublished {
                        snapshot_receipt_sha256: "4".repeat(64),
                        source_binding: isolated.clone(),
                    },
                )
                .is_err()
        );
        journal
            .transition(
                &peer,
                BarrierEventV1::RecutoverSnapshotPublished {
                    snapshot_receipt_sha256: "b".repeat(64),
                    source_binding: isolated,
                },
            )
            .expect("fresh snapshot");
        journal
            .transition(
                &peer,
                BarrierEventV1::RecutoverPassed {
                    recutover_receipt_sha256: "c".repeat(64),
                },
            )
            .expect("recutover");
        let released = journal
            .transition(
                &peer,
                BarrierEventV1::Released {
                    terminal_aggregate_sha256: "d".repeat(64),
                },
            )
            .expect("release");
        assert!(!released.admissions_closed);
        assert_eq!(released.record_count, PHASES.len());
        assert_eq!(released.current_phase, BarrierPhaseV1::Released);
    }

    #[test]
    fn barrier_crash_artifact_and_tamper_fail_closed() {
        let (root, policy) = private_root();
        let peer = peer(&policy);
        let journal = BarrierJournal::open(root.path(), policy.clone()).expect("journal");
        let source = binding(policy.legacy_writer_uid, unsafe { libc::getegid() }, 0o700);
        begin_barrier(&journal, &"1".repeat(64), &peer, &source).expect("begin");
        fs::write(
            root.path().join(".incoming-00000000000000000002.json"),
            b"partial",
        )
        .expect("partial crash record");
        assert!(journal.verify().is_err());
        fs::remove_file(root.path().join(".incoming-00000000000000000002.json"))
            .expect("remove fixture");
        fs::set_permissions(
            root.path().join("00000000000000000001.json"),
            fs::Permissions::from_mode(0o600),
        )
        .expect("make tamper fixture writable");
        fs::write(root.path().join("00000000000000000001.json"), b"{}").expect("tamper record");
        assert!(journal.verify().is_err());
    }

    #[test]
    fn fresh_peer_recovery_is_terminal_and_cannot_move_forward_or_grant() {
        let (root, policy) = private_root();
        let peer = peer(&policy);
        let journal = BarrierJournal::open(root.path(), policy.clone()).expect("journal");
        let original = binding(policy.legacy_writer_uid, unsafe { libc::getegid() }, 0o700);
        let isolated = binding(0, 0, 0o700);
        begin_barrier(&journal, &"a".repeat(64), &peer, &original).expect("begin");
        journal
            .transition(
                &peer,
                BarrierEventV1::SourceIsolated {
                    drain: drain_for(&journal, 'c', &original, &isolated),
                    isolated: isolated.clone(),
                    metadata_receipt_sha256: "d".repeat(64),
                    original,
                    ownership_conversion: conversion(&policy),
                },
            )
            .expect("isolate");
        assert!(
            journal
                .record_recovery_terminal(
                    &peer,
                    BarrierRecoveryDispositionV1::QuarantineOnly,
                    &"e".repeat(64),
                    None,
                    Some(isolated.clone()),
                )
                .is_err()
        );

        let mut recovery_peer = peer.clone();
        recovery_peer.audit_session_id = peer.audit_session_id.saturating_add(1);
        recovery_peer.audit_token_sha256 = "f".repeat(64);
        let recovered = journal
            .record_recovery_terminal(
                &recovery_peer,
                BarrierRecoveryDispositionV1::QuarantineOnly,
                &"e".repeat(64),
                None,
                Some(isolated.clone()),
            )
            .expect("fresh recovery terminal");
        assert!(!recovered.admissions_closed);
        assert!(!recovered.live_authority);
        assert_eq!(
            recovered.recovery_disposition,
            "recovery_only_quarantine_no_forward_authority"
        );
        assert!(
            journal
                .transition(
                    &peer,
                    BarrierEventV1::SnapshotPublished {
                        snapshot_receipt_sha256: "1".repeat(64),
                        source_binding: isolated,
                    },
                )
                .is_err()
        );
    }

    #[test]
    fn ownership_isolation_separates_stable_content_from_exact_metadata_replays() {
        let (root, policy) = private_root();
        let initial_peer = peer(&policy);
        let journal = BarrierJournal::open(root.path(), policy.clone()).expect("journal");
        let original = binding(policy.legacy_writer_uid, unsafe { libc::getegid() }, 0o755);
        let isolated = binding(0, 0, 0o700);
        begin_barrier(&journal, &"1".repeat(64), &initial_peer, &original).expect("begin");
        let proof = drain_for(&journal, '2', &original, &isolated);
        assert_eq!(
            proof.content_topology_before_sha256,
            proof.content_topology_after_sha256
        );
        assert_ne!(
            proof.original_exact_replay_sha256,
            proof.isolated_exact_replay_sha256
        );
        journal
            .transition(
                &initial_peer,
                BarrierEventV1::SourceIsolated {
                    drain: proof,
                    isolated: isolated.clone(),
                    metadata_receipt_sha256: "3".repeat(64),
                    original: original.clone(),
                    ownership_conversion: conversion(&policy),
                },
            )
            .expect("allowed ownership-only transform");

        let (content_root, content_policy) = private_root();
        let content_peer = peer(&content_policy);
        let content_journal =
            BarrierJournal::open(content_root.path(), content_policy.clone()).expect("journal");
        begin_barrier(&content_journal, &"4".repeat(64), &content_peer, &original)
            .expect("begin content negative");
        let mut changed_content = drain_for(&content_journal, '5', &original, &isolated);
        changed_content.content_topology_after_sha256 = "0".repeat(64);
        assert!(
            content_journal
                .transition(
                    &content_peer,
                    BarrierEventV1::SourceIsolated {
                        drain: changed_content,
                        isolated: isolated.clone(),
                        metadata_receipt_sha256: "6".repeat(64),
                        original: original.clone(),
                        ownership_conversion: conversion(&content_policy),
                    },
                )
                .is_err()
        );

        let (metadata_root, metadata_policy) = private_root();
        let metadata_peer = peer(&metadata_policy);
        let metadata_journal =
            BarrierJournal::open(metadata_root.path(), metadata_policy.clone()).expect("journal");
        begin_barrier(
            &metadata_journal,
            &"7".repeat(64),
            &metadata_peer,
            &original,
        )
        .expect("begin metadata negative");
        let mut forbidden = isolated.clone();
        forbidden.mtime_seconds = forbidden.mtime_seconds.saturating_add(1);
        assert!(
            metadata_journal
                .transition(
                    &metadata_peer,
                    BarrierEventV1::SourceIsolated {
                        drain: drain_for(&metadata_journal, '8', &original, &forbidden),
                        isolated: forbidden,
                        metadata_receipt_sha256: "9".repeat(64),
                        original,
                        ownership_conversion: conversion(&metadata_policy),
                    },
                )
                .is_err()
        );
    }

    #[test]
    fn rollback_replay_and_recutover_inode_must_match_original() {
        let (root, policy) = private_root();
        let peer = peer(&policy);
        let journal = BarrierJournal::open(root.path(), policy.clone()).expect("journal");
        let original = binding(policy.legacy_writer_uid, unsafe { libc::getegid() }, 0o700);
        let isolated = binding(0, 0, 0o700);
        begin_barrier(&journal, &"1".repeat(64), &peer, &original).expect("begin");
        advance_to_cutover(&journal, &peer, &policy, &original, &isolated);
        assert!(
            journal
                .transition(
                    &peer,
                    BarrierEventV1::RollbackRestored {
                        restored: original.clone(),
                        restored_replay_sha256: "0".repeat(64),
                        rollback_receipt_sha256: "7".repeat(64),
                    },
                )
                .is_err()
        );
        journal
            .transition(
                &peer,
                BarrierEventV1::RollbackRestored {
                    restored: original.clone(),
                    restored_replay_sha256: "d".repeat(64),
                    rollback_receipt_sha256: "7".repeat(64),
                },
            )
            .expect("exact original rollback replay");
        let mut replacement = isolated;
        replacement.inode = replacement.inode.saturating_add(1);
        assert!(
            journal
                .transition(
                    &peer,
                    BarrierEventV1::RecutoverSourceIsolated {
                        drain: drain_for(&journal, '8', &original, &replacement),
                        freshness_nonce: "9".repeat(64),
                        isolated: replacement,
                        metadata_receipt_sha256: "a".repeat(64),
                        ownership_conversion: conversion(&policy),
                    },
                )
                .is_err()
        );
    }

    #[test]
    fn transition_rechecks_current_boot_before_append() {
        let (root, policy) = private_root();
        let peer = peer(&policy);
        let journal = BarrierJournal::open(root.path(), policy.clone()).expect("journal");
        let source = binding(policy.legacy_writer_uid, unsafe { libc::getegid() }, 0o700);
        begin_barrier(&journal, &"1".repeat(64), &peer, &source).expect("begin");
        let mut envelope = journal.read_and_validate().expect("read journal").remove(0);
        envelope.record.boot_session_uuid = "11111111-1111-4111-8111-111111111111".to_string();
        let (barrier_root, client, volume, helper, mountpoint, source_binding) =
            match &envelope.record.event {
                BarrierEventV1::Acquired {
                    barrier_root,
                    client_executable_sha256,
                    dedicated_volume_uuid,
                    helper_executable_sha256,
                    mountpoint_underlying,
                    source_binding,
                    ..
                } => (
                    barrier_root.clone(),
                    client_executable_sha256.clone(),
                    dedicated_volume_uuid.clone(),
                    helper_executable_sha256.clone(),
                    mountpoint_underlying.clone(),
                    source_binding.clone(),
                ),
                _ => panic!("acquired"),
            };
        let challenge = barrier_operation_challenge(
            &envelope.record.epoch_nonce,
            &envelope.record.boot_session_uuid,
            &envelope.record.peer_audit_token_sha256,
            &client,
            &helper,
            &barrier_root,
            &volume,
            &mountpoint,
            &source_binding,
        )
        .expect("recompute synthetic prior-boot challenge");
        match &mut envelope.record.event {
            BarrierEventV1::Acquired {
                operation_challenge_sha256,
                ..
            } => *operation_challenge_sha256 = challenge,
            _ => unreachable!(),
        }
        envelope.record_sha256 =
            sha256(&canonical_json(&envelope.record).expect("canonical prior-boot record"));
        let bytes = canonical_json(&envelope).expect("canonical prior-boot envelope");
        let record_path = root.path().join(record_name(1));
        fs::set_permissions(&record_path, fs::Permissions::from_mode(0o600))
            .expect("open synthetic prior-boot record");
        fs::write(&record_path, bytes).expect("write synthetic prior-boot record");
        fs::set_permissions(&record_path, fs::Permissions::from_mode(0o400))
            .expect("reseal synthetic prior-boot record");
        let error = journal
            .transition(
                &peer,
                BarrierEventV1::SourceIsolated {
                    drain: drain_for(&journal, '2', &source, &binding(0, 0, 0o700)),
                    isolated: binding(0, 0, 0o700),
                    metadata_receipt_sha256: "3".repeat(64),
                    original: source,
                    ownership_conversion: conversion(&policy),
                },
            )
            .expect_err("prior-boot append must fail");
        assert!(error.to_string().contains("prior boot"));
    }

    #[test]
    fn t5_noowners_and_untrusted_install_paths_fail_closed() {
        assert!(validate_t5_mount_semantics(0, "apfs", "/Volumes/T5").is_ok());
        assert!(validate_t5_mount_semantics(MNT_IGNORE_OWNERSHIP, "apfs", "/Volumes/T5").is_err());
        let executable = std::env::current_exe().expect("current executable");
        assert!(
            open_trusted_install_regular(&executable, 0o555, "rootless test executable").is_err()
        );
    }

    #[test]
    fn live_policy_rejects_non_root_and_wheel_member_producer() {
        if unsafe { libc::geteuid() } != 0 {
            assert!(NamespacePolicy::live().is_err());
        }
        assert!(NamespacePolicy::live().is_err());
    }
}
