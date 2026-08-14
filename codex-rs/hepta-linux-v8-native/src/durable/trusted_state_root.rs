use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use sha2::Digest as _;

use crate::DirectoryAnchorV8;
use crate::FileIdentityV8;
use crate::NativeErrorV8;
use crate::StateRootLockV8;
use crate::TRUSTED_FILESYSTEM_FLAG_POLICIES_V8;
use crate::TrustedNodeMetadataV8;
use crate::invalid;
use crate::observe_machine_id_v8;
use crate::open_existing_state_root_lock_v8;

use super::ACTIVE_ATTEMPT_LEAF_V8;
use super::ATTEMPTS_DIRECTORY_V8;
use super::JOURNAL_DIRECTORY_V8;
use super::NONCE_CLAIMS_DIRECTORY_V8;
use super::QUARANTINE_DIRECTORY_V8;
use super::validate_digest;

pub const INSTALL_EPOCH_DIRECTORY_V8: &str = "install-epoch";
pub const STATE_ROOT_LOCK_LEAF_V8: &str = "state.lock";
pub const STATE_ROOT_PROFILE_ID_V8: &str = "hepta-linux-v8-state-root-profile-v1";

const STATE_ROOT_PROFILE_DOMAIN_V8: &[u8] = b"hepta_linux_v8_state_root_profile_binding_v1\0";
const TRUSTED_STATE_ROOT_BINDING_DOMAIN_V8: &[u8] =
    b"hepta_linux_v8_trusted_state_root_binding_v1\0";
const STATE_ROOT_LAYOUT_DOMAIN_V8: &[u8] = b"hepta_linux_v8_state_root_layout_manifest_v1\0";
const MAX_TOP_LEVEL_LEAVES_V8: usize = 32;

const REQUIRED_DIRECTORIES_V8: [(&str, u32); 5] = [
    (ATTEMPTS_DIRECTORY_V8, 0o700),
    (INSTALL_EPOCH_DIRECTORY_V8, 0o700),
    (JOURNAL_DIRECTORY_V8, 0o700),
    (NONCE_CLAIMS_DIRECTORY_V8, 0o700),
    (QUARANTINE_DIRECTORY_V8, 0o700),
];

/// Compile-time state-root and target-machine policy. There is intentionally
/// no public constructor: artifacts and callers cannot nominate their own
/// path, profile, layout, owner, or machine trust root.
#[derive(Debug, Eq, PartialEq)]
struct FrozenStateRootProfileV8 {
    gid: u32,
    layout_manifest_sha256: String,
    machine_id_sha256: String,
    mode: u32,
    path: PathBuf,
    profile_sha256: String,
    uid: u32,
}

impl FrozenStateRootProfileV8 {
    fn validate(&self, production: bool) -> Result<(), NativeErrorV8> {
        let path = self
            .path
            .to_str()
            .ok_or_else(|| invalid("frozen state-root path is not UTF-8"))?;
        validate_canonical_absolute_path_v8(path)?;
        if self.mode != 0o700 || (production && (self.uid != 0 || self.gid != 0)) {
            return Err(invalid("frozen state-root owner or mode is not exact"));
        }
        validate_digest("frozen state-root machine", &self.machine_id_sha256)?;
        validate_digest(
            "frozen state-root layout manifest",
            &self.layout_manifest_sha256,
        )?;
        validate_digest("frozen state-root profile", &self.profile_sha256)?;
        let expected = state_root_profile_sha256_v8(
            path,
            self.uid,
            self.gid,
            self.mode,
            &self.layout_manifest_sha256,
        )?;
        if self.profile_sha256 != expected
            || self.layout_manifest_sha256 != state_root_layout_manifest_sha256_v8()
        {
            return Err(invalid(
                "frozen state-root profile differs from the compiled canonical layout",
            ));
        }
        Ok(())
    }
}

/// Opaque process-lifetime proof that the exact compiled path, target machine,
/// filesystem identities, closed layout, and live singleton lock all match.
/// It is only a durable-state substrate and grants no install, activation,
/// provider-I/O, attempt, or qualification authority.
#[derive(Debug)]
pub struct TrustedStateRootV8 {
    anchor: DirectoryAnchorV8,
    binding_sha256: String,
    install_epoch_anchor: DirectoryAnchorV8,
    layout_manifest_sha256: String,
    lock: StateRootLockV8,
    machine_id_sha256: String,
    machine_id_source_identity: FileIdentityV8,
    path: PathBuf,
    profile_sha256: String,
    required_directory_identities: Vec<(&'static str, FileIdentityV8, TrustedNodeMetadataV8)>,
    root_identity: FileIdentityV8,
    root_metadata: TrustedNodeMetadataV8,
}

impl TrustedStateRootV8 {
    pub fn binding_sha256(&self) -> &str {
        &self.binding_sha256
    }

    pub fn identity(&self) -> FileIdentityV8 {
        self.root_identity
    }

    pub fn layout_manifest_sha256(&self) -> &str {
        &self.layout_manifest_sha256
    }

    pub fn lock_identity(&self) -> FileIdentityV8 {
        self.lock.identity()
    }

    pub fn machine_id_sha256(&self) -> &str {
        &self.machine_id_sha256
    }

    pub fn profile_sha256(&self) -> &str {
        &self.profile_sha256
    }

    pub(crate) fn machine_id_source_identity_v8(&self) -> FileIdentityV8 {
        self.machine_id_source_identity
    }

    pub(crate) fn current_root_identity_v8(&self) -> Result<FileIdentityV8, NativeErrorV8> {
        Ok(self.anchor.current_identity()?)
    }

    pub(crate) fn current_root_metadata_v8(&self) -> Result<TrustedNodeMetadataV8, NativeErrorV8> {
        Ok(self.anchor.trusted_node_metadata()?)
    }

    pub(crate) fn descriptor_replay_handles_v8(&self) -> (&DirectoryAnchorV8, &StateRootLockV8) {
        (&self.anchor, &self.lock)
    }

    pub fn revalidate(&self) -> Result<(), NativeErrorV8> {
        self.anchor.revalidate_identity()?;
        let root_metadata = self.anchor.trusted_node_metadata()?;
        if !root_metadata.matches_filesystem_domain(self.root_metadata) {
            return Err(invalid(
                "trusted state-root filesystem or mount domain changed after pinning",
            ));
        }
        self.install_epoch_anchor.revalidate_identity()?;
        self.lock.revalidate_for_root(&self.anchor)?;
        let named = DirectoryAnchorV8::open(&self.path)?;
        if !named
            .identity()
            .matches_stable_directory(self.root_identity)
        {
            return Err(invalid(
                "compiled state-root pathname no longer names the trusted inode",
            ));
        }
        if named.trusted_node_metadata()? != root_metadata {
            return Err(invalid(
                "compiled state-root pathname metadata differs from the retained descriptor",
            ));
        }
        let machine = observe_machine_id_v8()?;
        if machine.machine_id_sha256() != self.machine_id_sha256
            || machine.source_identity() != self.machine_id_source_identity
        {
            return Err(invalid(
                "fixed machine-id source changed after state-root trust establishment",
            ));
        }
        let observed = verify_top_level_layout_v8(
            &self.anchor,
            &self.lock,
            self.root_identity,
            root_metadata,
            Some(&self.required_directory_identities),
        )?;
        if !matches_pinned_directory_roster_v8(&self.required_directory_identities, &observed) {
            return Err(invalid(
                "state-root required-directory identities changed after pinning",
            ));
        }
        let install_identity = required_identity_v8(&observed, INSTALL_EPOCH_DIRECTORY_V8)?;
        if !self
            .install_epoch_anchor
            .identity()
            .matches_stable_directory(install_identity)
        {
            return Err(invalid(
                "pinned install-epoch directory differs from the trusted layout",
            ));
        }
        Ok(())
    }

    pub(crate) fn split_for_store_v8(
        &mut self,
    ) -> (&DirectoryAnchorV8, &DirectoryAnchorV8, &mut StateRootLockV8) {
        (&self.anchor, &self.install_epoch_anchor, &mut self.lock)
    }
}

/// Production entrypoint. It deliberately fails before observing or mutating
/// any host path until independently published profile bytes are compiled.
/// No caller-supplied path, machine, profile, or layout is accepted.
pub fn open_production_trusted_state_root_v8() -> Result<TrustedStateRootV8, NativeErrorV8> {
    let profile = required_frozen_state_root_profile_v8()?;
    open_trusted_state_root_with_profile_v8(profile, true)
}

fn required_frozen_state_root_profile_v8() -> Result<FrozenStateRootProfileV8, NativeErrorV8> {
    Err(invalid(
        "production state-root profile is not independently published",
    ))
}

fn open_trusted_state_root_with_profile_v8(
    profile: FrozenStateRootProfileV8,
    production: bool,
) -> Result<TrustedStateRootV8, NativeErrorV8> {
    profile.validate(production)?;
    let machine_before = observe_machine_id_v8()?;
    if machine_before.machine_id_sha256() != profile.machine_id_sha256 {
        return Err(invalid(
            "actual fixed machine-id differs from the compiled state-root binding",
        ));
    }
    let (effective_uid, effective_gid) = effective_ids_v8()?;
    if effective_uid != profile.uid || effective_gid != profile.gid {
        return Err(invalid(
            "state-root opener credential differs from the frozen owner",
        ));
    }

    let anchor = DirectoryAnchorV8::open(&profile.path)?;
    let root_identity = anchor.identity();
    let root_metadata = anchor.trusted_node_metadata()?;
    require_directory_identity_v8(
        "state root",
        root_identity,
        profile.uid,
        profile.gid,
        profile.mode,
        None,
    )?;
    preflight_top_level_layout_v8(&anchor, root_identity, root_metadata)?;

    let machine_after_preflight = observe_machine_id_v8()?;
    if machine_after_preflight.machine_id_sha256() != profile.machine_id_sha256
        || machine_after_preflight.source_identity() != machine_before.source_identity()
    {
        return Err(invalid(
            "fixed machine-id changed during state-root preflight",
        ));
    }
    let lock = open_existing_state_root_lock_v8(&anchor, OsStr::new(STATE_ROOT_LOCK_LEAF_V8))?;
    let required_directory_identities =
        verify_top_level_layout_v8(&anchor, &lock, root_identity, root_metadata, None)?;
    let install_epoch_anchor =
        anchor.open_directory_beneath(Path::new(INSTALL_EPOCH_DIRECTORY_V8))?;
    if install_epoch_anchor.identity()
        != required_identity_v8(&required_directory_identities, INSTALL_EPOCH_DIRECTORY_V8)?
    {
        return Err(invalid(
            "pinned install-epoch anchor differs from the verified layout",
        ));
    }
    let named = DirectoryAnchorV8::open(&profile.path)?;
    if !named.identity().matches_stable_directory(root_identity) {
        return Err(invalid(
            "compiled state-root pathname changed during trust establishment",
        ));
    }
    if named.trusted_node_metadata()? != root_metadata {
        return Err(invalid(
            "compiled state-root pathname metadata changed during trust establishment",
        ));
    }
    let machine_final = observe_machine_id_v8()?;
    if machine_final.machine_id_sha256() != profile.machine_id_sha256
        || machine_final.source_identity() != machine_before.source_identity()
    {
        return Err(invalid(
            "fixed machine-id changed during state-root trust establishment",
        ));
    }
    let binding_sha256 = trusted_state_root_binding_sha256_v8(
        &profile.machine_id_sha256,
        &profile.profile_sha256,
        &profile.layout_manifest_sha256,
    )?;

    Ok(TrustedStateRootV8 {
        anchor,
        binding_sha256,
        install_epoch_anchor,
        layout_manifest_sha256: profile.layout_manifest_sha256,
        lock,
        machine_id_sha256: profile.machine_id_sha256,
        machine_id_source_identity: machine_final.source_identity(),
        path: profile.path,
        profile_sha256: profile.profile_sha256,
        required_directory_identities,
        root_identity,
        root_metadata,
    })
}

fn preflight_top_level_layout_v8(
    anchor: &DirectoryAnchorV8,
    root_identity: FileIdentityV8,
    root_metadata: TrustedNodeMetadataV8,
) -> Result<(), NativeErrorV8> {
    let (before_names, before_directories, before_active) =
        collect_top_level_layout_v8(anchor, root_identity, root_metadata, false)?;
    let (after_names, after_directories, after_active) =
        collect_top_level_layout_v8(anchor, root_identity, root_metadata, false)?;
    if before_names != after_names
        || !same_directory_identity_roster_v8(&before_directories, &after_directories)
        || before_active != after_active
    {
        return Err(invalid(
            "state-root layout changed during read-only preflight",
        ));
    }
    Ok(())
}

fn verify_top_level_layout_v8(
    anchor: &DirectoryAnchorV8,
    lock: &StateRootLockV8,
    root_identity: FileIdentityV8,
    root_metadata: TrustedNodeMetadataV8,
    expected_directories: Option<&[(&'static str, FileIdentityV8, TrustedNodeMetadataV8)]>,
) -> Result<Vec<(&'static str, FileIdentityV8, TrustedNodeMetadataV8)>, NativeErrorV8> {
    lock.revalidate_for_root(anchor)?;
    let (before_names, before_directories, before_active) =
        collect_top_level_layout_v8(anchor, root_identity, root_metadata, true)?;
    lock.revalidate_for_root(anchor)?;
    let (after_names, after_directories, after_active) =
        collect_top_level_layout_v8(anchor, root_identity, root_metadata, true)?;
    if before_names != after_names
        || !same_directory_identity_roster_v8(&before_directories, &after_directories)
        || before_active != after_active
    {
        return Err(invalid(
            "state-root names or inode identities changed during verification",
        ));
    }
    if expected_directories
        .is_some_and(|expected| !matches_pinned_directory_roster_v8(expected, &before_directories))
    {
        return Err(invalid(
            "state-root required-directory identities differ from the pinned layout",
        ));
    }
    lock.revalidate_for_root(anchor)?;
    Ok(before_directories)
}

type TopLevelLayoutObservationV8 = (
    Vec<std::ffi::OsString>,
    Vec<(&'static str, FileIdentityV8, TrustedNodeMetadataV8)>,
    Option<(FileIdentityV8, TrustedNodeMetadataV8)>,
);

fn collect_top_level_layout_v8(
    anchor: &DirectoryAnchorV8,
    root_identity: FileIdentityV8,
    root_metadata: TrustedNodeMetadataV8,
    require_lock: bool,
) -> Result<TopLevelLayoutObservationV8, NativeErrorV8> {
    anchor.revalidate_identity()?;
    let names = anchor.list_leaf_names_bounded(MAX_TOP_LEVEL_LEAVES_V8)?;
    for name in &names {
        let Some(name) = name.to_str() else {
            return Err(invalid("state root contains a non-UTF-8 top-level leaf"));
        };
        if name.starts_with('.') && name.ends_with(".incoming") {
            return Err(invalid(
                "state root contains an interrupted top-level publication",
            ));
        }
        let known = name == STATE_ROOT_LOCK_LEAF_V8
            || name == ACTIVE_ATTEMPT_LEAF_V8
            || REQUIRED_DIRECTORIES_V8
                .iter()
                .any(|(required, _)| name == *required);
        if !known {
            return Err(invalid("state root contains an unknown top-level leaf"));
        }
    }
    if require_lock
        && !names
            .iter()
            .any(|name| name == OsStr::new(STATE_ROOT_LOCK_LEAF_V8))
    {
        return Err(invalid("state root singleton lock leaf disappeared"));
    }

    let mut directories = Vec::with_capacity(REQUIRED_DIRECTORIES_V8.len());
    for (relative, mode) in REQUIRED_DIRECTORIES_V8 {
        if !names.iter().any(|name| name == OsStr::new(relative)) {
            return Err(invalid(format!(
                "state root is missing required directory {relative}",
            )));
        }
        let directory = anchor.open_directory_beneath(Path::new(relative))?;
        require_directory_identity_v8(
            relative,
            directory.identity(),
            root_identity.owner_uid(),
            root_identity.owner_gid(),
            mode,
            Some(root_identity.device()),
        )?;
        let metadata = directory.trusted_node_metadata()?;
        if !metadata.matches_filesystem_domain(root_metadata) {
            return Err(invalid(format!(
                "{relative} directory differs from the trusted filesystem or mount domain",
            )));
        }
        directories.push((relative, directory.identity(), metadata));
    }
    let active = if names
        .iter()
        .any(|name| name == OsStr::new(ACTIVE_ATTEMPT_LEAF_V8))
    {
        let active = anchor.open_regular_readonly_beneath(Path::new(ACTIVE_ATTEMPT_LEAF_V8))?;
        let identity = active.identity();
        if identity.owner_uid() != root_identity.owner_uid()
            || identity.owner_gid() != root_identity.owner_gid()
            || identity.mode() != 0o600
            || identity.link_count() != 1
            || identity.device() != root_identity.device()
        {
            return Err(invalid(
                "state-root active-attempt leaf identity is not exact",
            ));
        }
        let metadata = active.trusted_node_metadata()?;
        if !metadata.matches_filesystem_domain(root_metadata) {
            return Err(invalid(
                "state-root active-attempt leaf differs from the trusted filesystem or mount domain",
            ));
        }
        Some((identity, metadata))
    } else {
        None
    };
    anchor.revalidate_identity()?;
    Ok((names, directories, active))
}

fn required_identity_v8(
    identities: &[(&'static str, FileIdentityV8, TrustedNodeMetadataV8)],
    name: &str,
) -> Result<FileIdentityV8, NativeErrorV8> {
    identities
        .iter()
        .find_map(|(observed, identity, _)| (*observed == name).then_some(*identity))
        .ok_or_else(|| invalid(format!("required state-root identity {name} is absent")))
}

fn same_directory_identity_roster_v8(
    left: &[(&'static str, FileIdentityV8, TrustedNodeMetadataV8)],
    right: &[(&'static str, FileIdentityV8, TrustedNodeMetadataV8)],
) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(
            |(
                (left_name, left_identity, left_metadata),
                (right_name, right_identity, right_metadata),
            )| {
                left_name == right_name
                    && left_identity.matches_stable_directory(*right_identity)
                    && left_metadata == right_metadata
            },
        )
}

fn matches_pinned_directory_roster_v8(
    pinned: &[(&'static str, FileIdentityV8, TrustedNodeMetadataV8)],
    observed: &[(&'static str, FileIdentityV8, TrustedNodeMetadataV8)],
) -> bool {
    pinned.len() == observed.len()
        && pinned.iter().zip(observed).all(
            |(
                (pinned_name, pinned_identity, pinned_metadata),
                (observed_name, observed_identity, observed_metadata),
            )| {
                pinned_name == observed_name
                    && pinned_identity.matches_stable_directory(*observed_identity)
                    && pinned_metadata.matches_filesystem_domain(*observed_metadata)
            },
        )
}

fn require_directory_identity_v8(
    label: &str,
    identity: FileIdentityV8,
    expected_uid: u32,
    expected_gid: u32,
    expected_mode: u32,
    expected_device: Option<u64>,
) -> Result<(), NativeErrorV8> {
    if identity.device() == 0
        || identity.inode() == 0
        || identity.link_count() == 0
        || identity.owner_uid() != expected_uid
        || identity.owner_gid() != expected_gid
        || identity.mode() != expected_mode
        || expected_device.is_some_and(|device| identity.device() != device)
    {
        return Err(invalid(format!(
            "{label} directory identity differs from the frozen state-root layout",
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn effective_ids_v8() -> Result<(u32, u32), NativeErrorV8> {
    // SAFETY: geteuid/getegid have no pointer arguments or preconditions.
    Ok((unsafe { libc::geteuid() }, unsafe { libc::getegid() }))
}

#[cfg(not(target_os = "linux"))]
fn effective_ids_v8() -> Result<(u32, u32), NativeErrorV8> {
    Err(crate::NativeSysErrorV8::UnsupportedPlatform("open trusted state root").into())
}

pub fn state_root_layout_manifest_sha256_v8() -> String {
    let mut bytes = STATE_ROOT_LAYOUT_DOMAIN_V8.to_vec();
    append_field_v8(&mut bytes, "policy.top_level_closed_world", b"true");
    append_field_v8(&mut bytes, "policy.incoming_top_level_forbidden", b"true");
    append_field_v8(
        &mut bytes,
        "policy.required_directories_same_device",
        b"true",
    );
    append_field_v8(&mut bytes, "policy.required_identities_pinned", b"true");
    append_field_v8(&mut bytes, "policy.statx_mount_id_required", b"true");
    append_field_v8(&mut bytes, "policy.xattr_acl_allowlist", b"empty");
    append_field_v8(
        &mut bytes,
        "policy.filesystem_inode_flags_exact_allow_masks",
        b"true",
    );
    append_field_v8(
        &mut bytes,
        "policy.root_and_directories_exact_owner_mode",
        b"true",
    );
    append_field_v8(&mut bytes, "policy.lock_empty_single_link_mode", b"0600");
    for (name, mode) in REQUIRED_DIRECTORIES_V8 {
        append_layout_entry_v8(&mut bytes, "required_directory", name, mode);
    }
    append_layout_entry_v8(&mut bytes, "required_leaf", STATE_ROOT_LOCK_LEAF_V8, 0o600);
    append_layout_entry_v8(&mut bytes, "optional_leaf", ACTIVE_ATTEMPT_LEAF_V8, 0o600);
    for (filesystem_type, inode_flags_allow_mask) in TRUSTED_FILESYSTEM_FLAG_POLICIES_V8 {
        append_u64_v8(&mut bytes, "filesystem.type", u64::from(filesystem_type));
        append_u64_v8(
            &mut bytes,
            "filesystem.inode_flags_allow_mask",
            u64::from(inode_flags_allow_mask),
        );
    }
    format!("{:x}", sha2::Sha256::digest(bytes))
}

/// Canonical profile hash shared with the qualification contract. This pure
/// helper is not a profile constructor and grants no trust by itself.
pub fn state_root_profile_sha256_v8(
    path: &str,
    uid: u32,
    gid: u32,
    mode: u32,
    layout_manifest_sha256: &str,
) -> Result<String, NativeErrorV8> {
    validate_canonical_absolute_path_v8(path)?;
    if mode != 0o700 {
        return Err(invalid("state-root profile mode is malformed"));
    }
    validate_digest("state-root profile layout", layout_manifest_sha256)?;
    let mut bytes = STATE_ROOT_PROFILE_DOMAIN_V8.to_vec();
    append_field_v8(
        &mut bytes,
        "profile_id",
        STATE_ROOT_PROFILE_ID_V8.as_bytes(),
    );
    append_u64_v8(&mut bytes, "profile_revision", 1);
    append_field_v8(&mut bytes, "path", path.as_bytes());
    append_u64_v8(&mut bytes, "uid", u64::from(uid));
    append_u64_v8(&mut bytes, "gid", u64::from(gid));
    append_u64_v8(&mut bytes, "mode", u64::from(mode));
    append_field_v8(
        &mut bytes,
        "layout_manifest_sha256",
        layout_manifest_sha256.as_bytes(),
    );
    Ok(format!("{:x}", sha2::Sha256::digest(bytes)))
}

pub fn trusted_state_root_binding_sha256_v8(
    machine_id_sha256: &str,
    profile_sha256: &str,
    layout_manifest_sha256: &str,
) -> Result<String, NativeErrorV8> {
    validate_digest("trusted-root machine", machine_id_sha256)?;
    validate_digest("trusted-root profile", profile_sha256)?;
    validate_digest("trusted-root layout", layout_manifest_sha256)?;
    let mut bytes = TRUSTED_STATE_ROOT_BINDING_DOMAIN_V8.to_vec();
    append_field_v8(
        &mut bytes,
        "machine_id_sha256",
        machine_id_sha256.as_bytes(),
    );
    append_field_v8(&mut bytes, "profile_sha256", profile_sha256.as_bytes());
    append_field_v8(
        &mut bytes,
        "layout_manifest_sha256",
        layout_manifest_sha256.as_bytes(),
    );
    Ok(format!("{:x}", sha2::Sha256::digest(bytes)))
}

fn validate_canonical_absolute_path_v8(path: &str) -> Result<(), NativeErrorV8> {
    if path.len() < 2
        || !path.starts_with('/')
        || path.ends_with('/')
        || path.contains("//")
        || path
            .split('/')
            .skip(1)
            .any(|part| part.is_empty() || part == "." || part == "..")
        || path.contains('\0')
    {
        return Err(invalid(
            "state-root path must be one canonical non-root absolute path",
        ));
    }
    Ok(())
}

fn append_layout_entry_v8(bytes: &mut Vec<u8>, disposition: &str, name: &str, mode: u32) {
    append_field_v8(bytes, "entry.disposition", disposition.as_bytes());
    append_field_v8(bytes, "entry.name", name.as_bytes());
    append_u64_v8(bytes, "entry.mode", u64::from(mode));
}

fn append_field_v8(bytes: &mut Vec<u8>, name: &str, value: &[u8]) {
    bytes.extend_from_slice(&(name.len() as u64).to_be_bytes());
    bytes.extend_from_slice(name.as_bytes());
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}

fn append_u64_v8(bytes: &mut Vec<u8>, name: &str, value: u64) {
    append_field_v8(bytes, name, &value.to_be_bytes());
}

#[cfg(all(test, target_os = "linux"))]
pub(crate) fn open_test_trusted_state_root_v8(
    path: &Path,
) -> Result<TrustedStateRootV8, NativeErrorV8> {
    // SAFETY: geteuid/getegid have no pointer arguments or preconditions.
    let uid = unsafe { libc::geteuid() };
    // SAFETY: see above.
    let gid = unsafe { libc::getegid() };
    let machine_id_sha256 = observe_machine_id_v8()?.machine_id_sha256().to_string();
    let layout_manifest_sha256 = state_root_layout_manifest_sha256_v8();
    let path_string = path
        .to_str()
        .ok_or_else(|| invalid("test state-root path is not UTF-8"))?;
    let profile_sha256 =
        state_root_profile_sha256_v8(path_string, uid, gid, 0o700, &layout_manifest_sha256)?;
    open_trusted_state_root_with_profile_v8(
        FrozenStateRootProfileV8 {
            gid,
            layout_manifest_sha256,
            machine_id_sha256,
            mode: 0o700,
            path: path.to_path_buf(),
            profile_sha256,
            uid,
        },
        false,
    )
}

#[cfg(test)]
#[path = "trusted_state_root_tests.rs"]
mod tests;
