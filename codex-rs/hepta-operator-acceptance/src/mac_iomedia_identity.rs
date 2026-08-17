//! Read-only macOS IOMedia object identity capture and replay.
//!
//! A BSD disk name is diagnostic metadata, not durable identity.  This module
//! binds each attached APFS node to the boot-scoped 64-bit IORegistry entry ID
//! of its IOMedia object and re-resolves that object through IOKit before any
//! future effect implementation may act.  The returned object deliberately
//! exposes no unmount/eject primitive and keeps both IOMedia references and
//! the DADiskRef alive through RAII.

#[cfg(target_os = "macos")]
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::AcceptanceError;
use crate::durable::canonical_json;
use crate::durable::sha256;

const IDENTITY_SCHEMA: &str = "hepta_mac_iomedia_registry_identity_v1";
const TOPOLOGY_SCHEMA: &str = "hepta_mac_attached_iomedia_topology_v1";
const PROVENANCE_SCHEMA: &str = "hepta_mac_iomedia_registry_provenance_v2";
const REGISTRY_INVENTORY_SCHEMA: &str = "hepta_mac_iomedia_registry_inventory_v2";
const FOUR_NODE_TOPOLOGY_SCHEMA: &str = "hepta_mac_iomedia_four_node_topology_v2";
const PROVENANCE_TOPOLOGY_SCHEMA: &str = "hepta_mac_attached_iomedia_topology_v2";
const BACKING_SCHEMA: &str = "hepta_mac_disk_image_backing_provenance_v1";
const BACKING_IDENTITY_SCHEMA: &str = "hepta_mac_disk_image_backing_identity_v2";
const EXACT_BACKING_IDENTITY_SCHEMA: &str = "hepta_mac_exact_disk_image_backing_identity_v3";
const UNLINKED_BACKING_SCHEMA: &str = "hepta_mac_unlinked_disk_image_backing_v3";
const BACKING_PATH_ABSENCE_SCHEMA: &str = "hepta_mac_backing_path_absence_v3";
const UNLINKED_BACKING_KIND: &str = "held_inode_namespace_unlinked";
const BACKING_PATH_ABSENCE_KIND: &str = "namespace_absent";
const RESTART_BACKING_IDENTITY_SCHEMA: &str = "hepta_mac_restart_disk_image_backing_identity_v3";
const RESTART_INVENTORY_SCHEMA: &str = "hepta_mac_restart_iomedia_inventory_v3";
const MAX_IOMEDIA_OBJECTS: usize = 256;
const MAX_ANCESTOR_DEPTH: usize = 64;
const MAX_CF_STRING_BYTES: usize = 16 * 1024;
const MAX_BACKING_COMPONENTS: usize = 128;
const MAX_BACKING_FILE_BYTES: u64 = 1024 * 1024 * 1024;
const EXPECTED_T5_VOLUME_UUID: &str = "fb804d1b-24cb-4d6e-aea7-a9e180807758";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IOMediaRegistryIdentityV1 {
    pub authority_granted: bool,
    pub bsd_name: String,
    /// Canonical fixed-width lower-hex u64.  JSON numbers are forbidden so
    /// readers cannot silently round IDs above JavaScript's 53-bit range.
    pub registry_entry_id: String,
    pub schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttachedIOMediaTopologyV1 {
    pub apfs_container: IOMediaRegistryIdentityV1,
    pub apfs_volume: IOMediaRegistryIdentityV1,
    pub authority_granted: bool,
    pub boot_session_uuid: String,
    pub physical_store: IOMediaRegistryIdentityV1,
    pub physical_whole: IOMediaRegistryIdentityV1,
    pub schema: String,
}

/// A typed snapshot of the properties exported by an IOMedia registry node.
/// `None` means the property was genuinely absent; absence is never rewritten
/// as `false`, zero, or an empty string.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IOMediaRegistryPropertiesV2 {
    pub content: Option<String>,
    pub ejectable: Option<bool>,
    pub leaf: Option<bool>,
    pub preferred_block_size: Option<u64>,
    pub removable: Option<bool>,
    pub size: Option<u64>,
    pub whole: Option<bool>,
    pub writable: Option<bool>,
}

/// The independent Disk Arbitration view of the same IOMedia node.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiskArbitrationPropertiesV2 {
    pub block_size: Option<u64>,
    pub content: Option<String>,
    pub ejectable: Option<bool>,
    pub internal: Option<bool>,
    pub leaf: Option<bool>,
    pub media_uuid: Option<String>,
    pub removable: Option<bool>,
    pub size: Option<u64>,
    pub whole: Option<bool>,
    pub writable: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IORegistryAncestorV1 {
    pub class_name: String,
    pub registry_entry_id: String,
    pub registry_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IOMediaRegistryProvenanceV2 {
    pub ancestry: Vec<IORegistryAncestorV1>,
    pub authority_granted: bool,
    pub bsd_name: String,
    pub conforms_to_iomedia: bool,
    pub disk_arbitration: DiskArbitrationPropertiesV2,
    pub iomedia: IOMediaRegistryPropertiesV2,
    pub registry_entry_id: String,
    pub registry_path: String,
    /// DADiskCopyWholeDisk is retained and replayed during capture.  Its
    /// boot-scoped registry identity is recorded here, not treated as an
    /// effect target.
    pub whole_disk: IOMediaRegistryIdentityV1,
    pub schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IOMediaRegistryInventoryV2 {
    pub all_registry_entry_ids: Vec<String>,
    pub authority_granted: bool,
    pub boot_session_uuid: String,
    pub capture_monotonic_nanoseconds: u64,
    pub schema: String,
    pub t5_apfs_container: IOMediaRegistryProvenanceV2,
    pub t5_apfs_volume: IOMediaRegistryProvenanceV2,
    pub t5_physical_store: IOMediaRegistryProvenanceV2,
    pub t5_physical_whole: IOMediaRegistryProvenanceV2,
    pub t5_volume_uuid: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IOMediaFourNodeTopologyV2 {
    pub apfs_container: IOMediaRegistryProvenanceV2,
    pub apfs_volume: IOMediaRegistryProvenanceV2,
    pub authority_granted: bool,
    pub boot_session_uuid: String,
    pub physical_store: IOMediaRegistryProvenanceV2,
    pub physical_whole: IOMediaRegistryProvenanceV2,
    pub schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackingObjectBindingV1 {
    pub content_sha256: Option<String>,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackingPathComponentV1 {
    pub directory: bool,
    pub fd_binding: BackingObjectBindingV1,
    pub path: String,
    pub path_binding_after: BackingObjectBindingV1,
    pub path_binding_before: BackingObjectBindingV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiskImageBackingProvenanceV1 {
    pub authority_granted: bool,
    pub canonical_path: String,
    pub disk_image_device: IORegistryAncestorV1,
    pub disk_image_device_ancestor_count: u32,
    pub disk_image_url: String,
    pub disk_image_url_ancestor_count: u32,
    pub opened_components: Vec<BackingPathComponentV1>,
    pub path_authority_granted: bool,
    pub schema: String,
}

/// Prepared, path-only identity of a disk-image backing file.  Unlike
/// `DiskImageBackingProvenanceV1`, this deliberately excludes boot-scoped
/// IORegistry ancestry so it can be replayed by a fresh process before a
/// lingering attachment has been classified.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiskImageBackingIdentityV2 {
    pub authority_granted: bool,
    pub canonical_path: String,
    pub opened_components: Vec<BackingPathComponentV1>,
    pub path_authority_granted: bool,
    pub schema: String,
}

/// Full-stat, descriptor-backed prepared identity.  V2 intentionally omitted
/// APFS generation and birthtime; this V3 projection is minted only while the
/// complete held descriptor chain and each retained-parent pathname agree.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FilesystemObjectBindingV3 {
    pub birthtime_nanoseconds: i64,
    pub birthtime_seconds: i64,
    pub ctime_nanoseconds: i64,
    pub ctime_seconds: i64,
    pub dev: u64,
    pub flags: u32,
    pub generation: u32,
    pub gid: u32,
    pub inode: u64,
    pub mode: u32,
    pub mtime_nanoseconds: i64,
    pub mtime_seconds: i64,
    pub nlink: u64,
    pub size: u64,
    pub uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactBackingPathComponentV3 {
    pub binding: FilesystemObjectBindingV3,
    pub directory: bool,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactDiskImageBackingIdentityV3 {
    pub authority_granted: bool,
    pub canonical_path: String,
    pub content_sha256: String,
    pub opened_components: Vec<ExactBackingPathComponentV3>,
    pub schema: String,
}

/// Canonical full-stat projection of the retained backing file on one side of
/// the namespace-unlink observation.  `rdev` is recorded separately from the
/// older prepared identity so the transition cannot silently change any
/// terminal inode field that V3 is required to bind.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnlinkedBackingFileStateV3 {
    pub birthtime_nanoseconds: i64,
    pub birthtime_seconds: i64,
    pub ctime_nanoseconds: i64,
    pub ctime_seconds: i64,
    pub dev: u64,
    pub flags: u32,
    pub generation: u32,
    pub gid: u32,
    pub inode: u64,
    pub mode: u32,
    pub mtime_nanoseconds: i64,
    pub mtime_seconds: i64,
    pub nlink: u64,
    pub rdev: u64,
    pub size: u64,
    pub uid: u32,
}

/// Serializable evidence for a namespace unlink that was performed by an
/// external actor while this process continuously retained the original file
/// descriptor.  This is evidence only and never grants path or effect
/// authority.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnlinkedBackingBindingV3 {
    pub authority_granted: bool,
    pub canonical_path: String,
    pub content_sha256: String,
    pub initial_file: UnlinkedBackingFileStateV3,
    pub kind: String,
    pub opened_ancestors_after: Vec<ExactBackingPathComponentV3>,
    pub opened_ancestors_before: Vec<ExactBackingPathComponentV3>,
    pub post_unlink_file: UnlinkedBackingFileStateV3,
    pub prepared_backing_sha256: String,
    pub schema: String,
}

/// Canonical evidence that the prepared basename is currently absent beneath
/// the exact retained ancestor chain.  It intentionally contains no terminal
/// inode or link-count assertion and therefore cannot be confused with proof
/// that the formerly prepared inode was globally deleted.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackingPathAbsenceBindingV3 {
    pub authority_granted: bool,
    pub basename: String,
    pub canonical_path: String,
    pub kind: String,
    pub observed_ancestors: Vec<ExactBackingPathComponentV3>,
    pub prepared_ancestors: Vec<ExactBackingPathComponentV3>,
    pub prepared_backing_sha256: String,
    pub schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestartDiskImageBackingIdentityV3 {
    pub authority_granted: bool,
    pub canonical_path: String,
    pub file_binding: BackingObjectBindingV1,
    pub schema: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestartDiskImageCandidateV3 {
    pub backing_identity: RestartDiskImageBackingIdentityV3,
    pub canonical_backing_path: String,
    pub disk_image_device: IORegistryAncestorV1,
    pub disk_image_url: String,
    pub disk_image_url_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestartIOMediaObjectV3 {
    pub authority_granted: bool,
    pub candidate: Option<RestartDiskImageCandidateV3>,
    pub provenance: IOMediaRegistryProvenanceV2,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestartIOMediaInventoryV3 {
    pub authority_granted: bool,
    pub boot_session_uuid: String,
    pub objects: Vec<RestartIOMediaObjectV3>,
    pub schema: String,
}

fn valid_restart_bsd_name(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("disk") else {
        return false;
    };
    let components = rest.split('s').collect::<Vec<_>>();
    !components.is_empty()
        && components.iter().enumerate().all(|(index, component)| {
            !component.is_empty()
                && component.bytes().all(|byte| byte.is_ascii_digit())
                && !component.starts_with('0')
                || (index == 0 && *component == "0")
        })
}

pub fn validate_restart_iomedia_inventory_v3(
    inventory: &RestartIOMediaInventoryV3,
) -> Result<(), AcceptanceError> {
    if inventory.schema != RESTART_INVENTORY_SCHEMA
        || inventory.authority_granted
        || !valid_uuid(&inventory.boot_session_uuid)
        || inventory.objects.is_empty()
        || inventory.objects.len() > MAX_IOMEDIA_OBJECTS
    {
        return Err(invalid(
            "restart IOMedia inventory is malformed or grants authority",
        ));
    }
    let mut previous_id: Option<&str> = None;
    let mut bsd_names = BTreeSet::new();
    for object in &inventory.objects {
        let node = &object.provenance;
        let first = node.ancestry.first();
        let terminal = node.ancestry.last();
        if object.authority_granted
            || node.schema != PROVENANCE_SCHEMA
            || node.authority_granted
            || !node.conforms_to_iomedia
            || node.bsd_name.len() > 256
            || !valid_restart_bsd_name(&node.bsd_name)
            || parse_registry_entry_id(&node.registry_entry_id).is_err()
            || !valid_registry_path(&node.registry_path)
            || node.ancestry.is_empty()
            || node.ancestry.len() > MAX_ANCESTOR_DEPTH
            || first.is_none_or(|ancestor| {
                ancestor.registry_entry_id != node.registry_entry_id
                    || ancestor.registry_path.as_deref() != Some(node.registry_path.as_str())
            })
            || terminal.is_none_or(|ancestor| {
                ancestor.class_name != "IORegistryEntry" || ancestor.registry_path.is_some()
            })
            || node.whole_disk.schema != IDENTITY_SCHEMA
            || node.whole_disk.authority_granted
            || !valid_restart_bsd_name(&node.whole_disk.bsd_name)
            || parse_registry_entry_id(&node.whole_disk.registry_entry_id).is_err()
            || !bsd_names.insert(node.bsd_name.as_str())
            || previous_id.is_some_and(|previous| previous >= node.registry_entry_id.as_str())
        {
            return Err(invalid(
                "restart IOMedia object is malformed, duplicated, or out of order",
            ));
        }
        previous_id = Some(&node.registry_entry_id);
        let mut ancestry_ids = BTreeSet::new();
        let mut ancestry_paths = BTreeSet::new();
        for (index, ancestor) in node.ancestry.iter().enumerate() {
            let terminal_root = index + 1 == node.ancestry.len()
                && ancestor.class_name == "IORegistryEntry"
                && ancestor.registry_path.is_none();
            if parse_registry_entry_id(&ancestor.registry_entry_id).is_err()
                || !valid_class_name(&ancestor.class_name)
                || (!terminal_root
                    && ancestor
                        .registry_path
                        .as_deref()
                        .is_none_or(|path| !valid_registry_path(path)))
                || !ancestry_ids.insert(ancestor.registry_entry_id.as_str())
                || ancestor
                    .registry_path
                    .as_deref()
                    .is_some_and(|path| !ancestry_paths.insert(path))
            {
                return Err(invalid(
                    "restart IOMedia ancestry is malformed, cyclic, or aliased",
                ));
            }
        }
        if node.iomedia.preferred_block_size != node.disk_arbitration.block_size
            || node.iomedia.content != node.disk_arbitration.content
            || node.iomedia.ejectable != node.disk_arbitration.ejectable
            || node.iomedia.leaf != node.disk_arbitration.leaf
            || node.iomedia.removable != node.disk_arbitration.removable
            || node.iomedia.size != node.disk_arbitration.size
            || node.iomedia.whole != node.disk_arbitration.whole
            || node.iomedia.writable != node.disk_arbitration.writable
        {
            return Err(invalid(
                "restart IOMedia and Disk Arbitration properties disagree",
            ));
        }
        let device_ancestors = node
            .ancestry
            .iter()
            .filter(|ancestor| ancestor.class_name == "AppleDiskImageDevice")
            .collect::<Vec<_>>();
        match &object.candidate {
            None if device_ancestors.is_empty() => {}
            Some(candidate)
                if device_ancestors.len() == 1
                    && device_ancestors[0] == &candidate.disk_image_device
                    && strict_file_url_path(&candidate.disk_image_url)?
                        == candidate.disk_image_url_path
                    && validate_restart_disk_image_backing_identity_v3(
                        &candidate.backing_identity,
                    )
                    .is_ok()
                    && candidate.backing_identity.canonical_path
                        == candidate.canonical_backing_path => {}
            _ => {
                return Err(invalid(
                    "restart IOMedia object has an ambiguous disk-image candidate",
                ));
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AttachedIOMediaTopologyV2 {
    pub apfs_container: IOMediaRegistryProvenanceV2,
    pub apfs_volume: IOMediaRegistryProvenanceV2,
    pub authority_granted: bool,
    pub backing: DiskImageBackingProvenanceV1,
    pub boot_session_uuid: String,
    pub fresh_t5: IOMediaFourNodeTopologyV2,
    pub physical_store: IOMediaRegistryProvenanceV2,
    pub physical_whole: IOMediaRegistryProvenanceV2,
    pub pre_attach_inventory: IOMediaRegistryInventoryV2,
    pub schema: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DiskImageCandidateObservation {
    device: IORegistryAncestorV1,
    url: String,
}

fn select_unique_disk_image_candidate(
    observations: &[(usize, Vec<DiskImageCandidateObservation>)],
) -> Result<DiskImageCandidateObservation, AcceptanceError> {
    if observations.len() != 4
        || observations
            .iter()
            .any(|(device_count, candidates)| *device_count != 1 || candidates.len() != 1)
    {
        return Err(invalid(
            "attached topology does not have exactly one typed DiskImageURL ancestor per node",
        ));
    }
    let first = observations[0].1[0].clone();
    if observations
        .iter()
        .any(|(_, candidates)| candidates[0] != first)
    {
        return Err(invalid(
            "attached topology has multiple distinct DiskImageURL ancestors",
        ));
    }
    strict_file_url_path(&first.url)?;
    Ok(first)
}

#[derive(Clone, Copy, Debug)]
pub struct ExpectedIOMediaTopology<'a> {
    pub apfs_container: &'a str,
    pub apfs_volume: &'a str,
    pub physical_store: &'a str,
    pub physical_whole: &'a str,
}

impl<'a> ExpectedIOMediaTopology<'a> {
    fn ordered(self) -> [(&'static str, &'a str); 4] {
        [
            ("physical whole", self.physical_whole),
            ("physical store", self.physical_store),
            ("APFS container", self.apfs_container),
            ("APFS volume", self.apfs_volume),
        ]
    }
}

impl AttachedIOMediaTopologyV1 {
    fn ordered(&self) -> [(&'static str, &IOMediaRegistryIdentityV1); 4] {
        [
            ("physical whole", &self.physical_whole),
            ("physical store", &self.physical_store),
            ("APFS container", &self.apfs_container),
            ("APFS volume", &self.apfs_volume),
        ]
    }
}

impl AttachedIOMediaTopologyV2 {
    pub(crate) fn ordered(&self) -> [(&'static str, &IOMediaRegistryProvenanceV2); 4] {
        [
            ("physical whole", &self.physical_whole),
            ("physical store", &self.physical_store),
            ("APFS container", &self.apfs_container),
            ("APFS volume", &self.apfs_volume),
        ]
    }
}

impl IOMediaRegistryInventoryV2 {
    fn ordered_t5(&self) -> [(&'static str, &IOMediaRegistryProvenanceV2); 4] {
        [
            ("T5 physical whole", &self.t5_physical_whole),
            ("T5 physical store", &self.t5_physical_store),
            ("T5 APFS container", &self.t5_apfs_container),
            ("T5 APFS volume", &self.t5_apfs_volume),
        ]
    }
}

impl IOMediaFourNodeTopologyV2 {
    fn ordered(&self) -> [(&'static str, &IOMediaRegistryProvenanceV2); 4] {
        [
            ("physical whole", &self.physical_whole),
            ("physical store", &self.physical_store),
            ("APFS container", &self.apfs_container),
            ("APFS volume", &self.apfs_volume),
        ]
    }
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

fn valid_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36
        && bytes
            .iter()
            .any(|byte| byte.is_ascii_hexdigit() && *byte != b'0')
        && bytes.iter().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => *byte == b'-',
            _ => byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase(),
        })
}

fn valid_bsd_name(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("disk") else {
        return false;
    };
    let (whole, slice) = match rest.split_once('s') {
        Some((whole, slice)) if !slice.contains('s') => (whole, Some(slice)),
        Some(_) => return false,
        None => (rest, None),
    };
    !whole.is_empty()
        && whole.bytes().all(|byte| byte.is_ascii_digit())
        && (whole == "0" || !whole.starts_with('0'))
        && slice.is_none_or(|slice| {
            !slice.is_empty()
                && !slice.starts_with('0')
                && slice.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn parse_registry_entry_id(value: &str) -> Result<u64, AcceptanceError> {
    if value.len() != 16
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(invalid(
            "IOMedia registry entry ID is not canonical 16-digit lower hex",
        ));
    }
    let parsed = u64::from_str_radix(value, 16)
        .map_err(|_| invalid("IOMedia registry entry ID does not fit u64"))?;
    if parsed == 0 {
        return Err(invalid("IOMedia registry entry ID is zero"));
    }
    Ok(parsed)
}

fn validate_registry_identity(
    identity: &IOMediaRegistryIdentityV1,
    expected_bsd_name: &str,
    label: &str,
) -> Result<(), AcceptanceError> {
    if identity.schema != IDENTITY_SCHEMA
        || identity.authority_granted
        || parse_registry_entry_id(&identity.registry_entry_id).is_err()
        || !valid_bsd_name(&identity.bsd_name)
        || identity.bsd_name != expected_bsd_name
    {
        return Err(invalid(format!(
            "{label} IOMedia identity is malformed, grants authority, or binds the wrong BSD name"
        )));
    }
    Ok(())
}

pub fn validate_iomedia_topology_identity_shape(
    identity: &AttachedIOMediaTopologyV1,
    expected: ExpectedIOMediaTopology<'_>,
) -> Result<(), AcceptanceError> {
    if identity.schema != TOPOLOGY_SCHEMA
        || identity.authority_granted
        || !valid_uuid(&identity.boot_session_uuid)
    {
        return Err(invalid(
            "IOMedia topology identity is malformed or grants authority",
        ));
    }
    let actual = identity.ordered();
    let expected = expected.ordered();
    let mut registry_entry_ids = BTreeSet::new();
    let mut bsd_names = BTreeSet::new();
    for (index, ((label, node), (expected_label, expected_bsd_name))) in
        actual.into_iter().zip(expected).enumerate()
    {
        if label != expected_label {
            return Err(invalid("IOMedia topology role order changed"));
        }
        validate_registry_identity(node, expected_bsd_name, label)?;
        if !registry_entry_ids.insert(node.registry_entry_id.as_str())
            || !bsd_names.insert(node.bsd_name.as_str())
        {
            return Err(invalid(format!(
                "IOMedia topology node {index} aliases another BSD name or registry entry ID"
            )));
        }
    }
    Ok(())
}

fn validate_iomedia_topology_identity_against_boot(
    identity: &AttachedIOMediaTopologyV1,
    expected: ExpectedIOMediaTopology<'_>,
    current_boot_session_uuid: &str,
) -> Result<(), AcceptanceError> {
    validate_iomedia_topology_identity_shape(identity, expected)?;
    if !valid_uuid(current_boot_session_uuid)
        || identity.boot_session_uuid != current_boot_session_uuid
    {
        return Err(invalid(
            "IOMedia topology identity belongs to another boot session",
        ));
    }
    Ok(())
}

pub fn validate_iomedia_topology_identity_current_boot(
    identity: &AttachedIOMediaTopologyV1,
    expected: ExpectedIOMediaTopology<'_>,
) -> Result<(), AcceptanceError> {
    validate_iomedia_topology_identity_against_boot(
        identity,
        expected,
        &current_boot_session_uuid()?,
    )
}

fn strict_file_url_path(value: &str) -> Result<String, AcceptanceError> {
    if value.len() > MAX_CF_STRING_BYTES || !value.starts_with("file:///") {
        return Err(invalid("DiskImageURL is not a bounded local file URL"));
    }
    let encoded_path = &value["file://".len()..];
    if encoded_path.contains(['?', '#']) || !encoded_path.starts_with('/') {
        return Err(invalid("DiskImageURL contains a host, query, or fragment"));
    }
    let raw = encoded_path.as_bytes();
    let mut decoded = Vec::with_capacity(raw.len());
    let mut index = 0;
    while index < raw.len() {
        let byte = raw[index];
        if byte == b'%' {
            if index + 2 >= raw.len()
                || !raw[index + 1].is_ascii_hexdigit()
                || !raw[index + 2].is_ascii_hexdigit()
                || raw[index + 1].is_ascii_lowercase()
                || raw[index + 2].is_ascii_lowercase()
            {
                return Err(invalid("DiskImageURL has a noncanonical percent escape"));
            }
            let hex = |digit: u8| -> u8 {
                match digit {
                    b'0'..=b'9' => digit - b'0',
                    b'A'..=b'F' => digit - b'A' + 10,
                    _ => unreachable!("validated uppercase hex"),
                }
            };
            let decoded_byte = (hex(raw[index + 1]) << 4) | hex(raw[index + 2]);
            if decoded_byte == 0
                || decoded_byte == b'%'
                || decoded_byte == b'/'
                || decoded_byte.is_ascii_alphanumeric()
                || matches!(decoded_byte, b'-' | b'.' | b'_' | b'~')
            {
                return Err(invalid(
                    "DiskImageURL percent-encodes NUL, a separator, or an unreserved byte",
                ));
            }
            decoded.push(decoded_byte);
            index += 3;
        } else {
            if !(byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'.' | b'_' | b'~')) {
                return Err(invalid("DiskImageURL contains a noncanonical raw byte"));
            }
            decoded.push(byte);
            index += 1;
        }
    }
    let path = String::from_utf8(decoded)
        .map_err(|_| invalid("DiskImageURL path is not unambiguous UTF-8"))?;
    let parsed = Path::new(&path);
    if !parsed.is_absolute()
        || parsed.components().count() > MAX_BACKING_COMPONENTS
        || parsed.components().any(|component| {
            !matches!(
                component,
                std::path::Component::RootDir | std::path::Component::Normal(_)
            )
        })
        || path.ends_with('/')
    {
        return Err(invalid(
            "DiskImageURL path is not a bounded canonical absolute path",
        ));
    }
    Ok(path)
}

fn valid_registry_path(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_CF_STRING_BYTES
        && value.starts_with("IOService:/")
        && !value.contains('\0')
}

fn valid_class_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

#[derive(Clone, Copy)]
enum IOMediaRole {
    PhysicalWhole,
    PhysicalStore,
    ApfsContainer,
    ApfsVolume,
}

impl IOMediaRole {
    fn expected_class(self) -> &'static str {
        match self {
            Self::PhysicalWhole | Self::PhysicalStore => "IOMedia",
            Self::ApfsContainer => "AppleAPFSMedia",
            Self::ApfsVolume => "AppleAPFSVolume",
        }
    }

    fn expected_leaf(self) -> bool {
        matches!(self, Self::ApfsVolume)
    }

    fn expected_whole(self) -> bool {
        matches!(self, Self::PhysicalWhole | Self::ApfsContainer)
    }
}

fn validate_provenance_node(
    node: &IOMediaRegistryProvenanceV2,
    expected_bsd_name: &str,
    role: IOMediaRole,
    label: &str,
) -> Result<(), AcceptanceError> {
    if node.schema != PROVENANCE_SCHEMA
        || node.authority_granted
        || !node.conforms_to_iomedia
        || node.bsd_name != expected_bsd_name
        || !valid_bsd_name(&node.bsd_name)
        || parse_registry_entry_id(&node.registry_entry_id).is_err()
        || !valid_registry_path(&node.registry_path)
        || node.ancestry.is_empty()
        || node.ancestry.len() > MAX_ANCESTOR_DEPTH
        || node.ancestry.last().is_none_or(|ancestor| {
            ancestor.class_name != "IORegistryEntry" || ancestor.registry_path.is_some()
        })
    {
        return Err(invalid(format!("{label} IOMedia provenance is malformed")));
    }
    let first = &node.ancestry[0];
    if first.registry_entry_id != node.registry_entry_id
        || first.registry_path.as_deref() != Some(node.registry_path.as_str())
        || first.class_name != role.expected_class()
    {
        return Err(invalid(format!(
            "{label} IOMedia ancestry does not begin at the captured node"
        )));
    }
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for (index, ancestor) in node.ancestry.iter().enumerate() {
        let terminal_root = index + 1 == node.ancestry.len()
            && ancestor.class_name == "IORegistryEntry"
            && ancestor.registry_path.is_none();
        if parse_registry_entry_id(&ancestor.registry_entry_id).is_err()
            || (!terminal_root
                && ancestor
                    .registry_path
                    .as_deref()
                    .is_none_or(|path| !valid_registry_path(path)))
            || !valid_class_name(&ancestor.class_name)
            || (ancestor.class_name == "IORegistryEntry" && !terminal_root)
            || !ids.insert(ancestor.registry_entry_id.as_str())
            || ancestor
                .registry_path
                .as_deref()
                .is_some_and(|path| !paths.insert(path))
        {
            return Err(invalid(format!(
                "{label} IOMedia ancestry is malformed or cyclic"
            )));
        }
    }
    let iomedia = &node.iomedia;
    let da = &node.disk_arbitration;
    if iomedia.whole != Some(role.expected_whole())
        || da.whole != Some(role.expected_whole())
        || iomedia.leaf != Some(role.expected_leaf())
        || da.leaf != Some(role.expected_leaf())
        || iomedia.writable.is_none()
        || iomedia.writable != da.writable
        || iomedia.ejectable.is_none()
        || iomedia.ejectable != da.ejectable
        || iomedia.removable.is_none()
        || iomedia.removable != da.removable
        || iomedia.size.is_none()
        || iomedia.size != da.size
        || iomedia.preferred_block_size.is_none()
        || iomedia.preferred_block_size != da.block_size
        || iomedia.content.is_none()
        || iomedia.content != da.content
        || iomedia.size == Some(0)
        || iomedia.preferred_block_size == Some(0)
        || iomedia
            .content
            .as_ref()
            .is_some_and(|content| content.is_empty() || content.len() > MAX_CF_STRING_BYTES)
        || da.media_uuid.as_ref().is_some_and(|uuid| !valid_uuid(uuid))
        || da.internal == Some(true)
    {
        return Err(invalid(format!(
            "{label} IOMedia and Disk Arbitration properties are absent, invalid, or disagree"
        )));
    }
    validate_registry_identity(&node.whole_disk, &node.whole_disk.bsd_name, label)?;
    Ok(())
}

fn ancestry_position(node: &IOMediaRegistryProvenanceV2, registry_entry_id: &str) -> Option<usize> {
    node.ancestry
        .iter()
        .position(|ancestor| ancestor.registry_entry_id == registry_entry_id)
}

fn validate_four_node_relationships(
    physical_whole: &IOMediaRegistryProvenanceV2,
    physical_store: &IOMediaRegistryProvenanceV2,
    apfs_container: &IOMediaRegistryProvenanceV2,
    apfs_volume: &IOMediaRegistryProvenanceV2,
    label: &str,
) -> Result<(), AcceptanceError> {
    if physical_whole.whole_disk.registry_entry_id != physical_whole.registry_entry_id
        || physical_whole.whole_disk.bsd_name != physical_whole.bsd_name
        || physical_store.whole_disk.registry_entry_id != physical_whole.registry_entry_id
        || physical_store.whole_disk.bsd_name != physical_whole.bsd_name
        || apfs_container.whole_disk.registry_entry_id != apfs_container.registry_entry_id
        || apfs_container.whole_disk.bsd_name != apfs_container.bsd_name
        || apfs_volume.whole_disk.registry_entry_id != apfs_container.registry_entry_id
        || apfs_volume.whole_disk.bsd_name != apfs_container.bsd_name
    {
        return Err(invalid(format!(
            "{label} DADiskCopyWholeDisk semantics do not match whole/store/container/volume roles"
        )));
    }
    let store_whole = ancestry_position(physical_store, &physical_whole.registry_entry_id);
    let container_store = ancestry_position(apfs_container, &physical_store.registry_entry_id);
    let container_whole = ancestry_position(apfs_container, &physical_whole.registry_entry_id);
    let volume_container = ancestry_position(apfs_volume, &apfs_container.registry_entry_id);
    let volume_store = ancestry_position(apfs_volume, &physical_store.registry_entry_id);
    let volume_whole = ancestry_position(apfs_volume, &physical_whole.registry_entry_id);
    if !matches!(store_whole, Some(index) if index > 0)
        || !matches!((container_store, container_whole), (Some(store), Some(whole)) if store > 0 && store < whole)
        || !matches!((volume_container, volume_store, volume_whole), (Some(container), Some(store), Some(whole)) if container > 0 && container < store && store < whole)
    {
        return Err(invalid(format!(
            "{label} IOService ancestry does not prove volume -> container -> store -> physical whole"
        )));
    }
    let exact_bridge =
        |node: &IOMediaRegistryProvenanceV2, bridge: &str, parent: &IOMediaRegistryProvenanceV2| {
            node.ancestry
                .get(1)
                .is_some_and(|entry| entry.class_name == bridge)
                && node
                    .ancestry
                    .get(2..)
                    .is_some_and(|suffix| suffix == parent.ancestry)
        };
    if !exact_bridge(physical_store, "IOGUIDPartitionScheme", physical_whole)
        || !exact_bridge(apfs_container, "AppleAPFSContainerScheme", physical_store)
        || !exact_bridge(apfs_volume, "AppleAPFSContainer", apfs_container)
    {
        return Err(invalid(format!(
            "{label} IOService ancestry does not have the exact APFS and GUID bridge classes"
        )));
    }
    Ok(())
}

fn validate_four_node_topology_shape(
    topology: &IOMediaFourNodeTopologyV2,
    expected: ExpectedIOMediaTopology<'_>,
    label: &str,
) -> Result<(), AcceptanceError> {
    if topology.schema != FOUR_NODE_TOPOLOGY_SCHEMA
        || topology.authority_granted
        || !valid_uuid(&topology.boot_session_uuid)
    {
        return Err(invalid(format!(
            "{label} four-node IOMedia topology is malformed"
        )));
    }
    for (index, ((role, node), (_, expected_bsd_name))) in topology
        .ordered()
        .into_iter()
        .zip(expected.ordered())
        .enumerate()
    {
        let media_role = match index {
            0 => IOMediaRole::PhysicalWhole,
            1 => IOMediaRole::PhysicalStore,
            2 => IOMediaRole::ApfsContainer,
            3 => IOMediaRole::ApfsVolume,
            _ => unreachable!("fixed four-node topology"),
        };
        validate_provenance_node(
            node,
            expected_bsd_name,
            media_role,
            &format!("{label} {role}"),
        )?;
    }
    validate_four_node_relationships(
        &topology.physical_whole,
        &topology.physical_store,
        &topology.apfs_container,
        &topology.apfs_volume,
        label,
    )
}

pub fn validate_iomedia_registry_inventory_shape(
    inventory: &IOMediaRegistryInventoryV2,
    expected: ExpectedIOMediaTopology<'_>,
) -> Result<(), AcceptanceError> {
    if inventory.schema != REGISTRY_INVENTORY_SCHEMA
        || inventory.authority_granted
        || !valid_uuid(&inventory.boot_session_uuid)
        || inventory.capture_monotonic_nanoseconds == 0
        || inventory.all_registry_entry_ids.is_empty()
        || inventory.all_registry_entry_ids.len() > MAX_IOMEDIA_OBJECTS
        || !inventory
            .all_registry_entry_ids
            .windows(2)
            .all(|pair| pair[0] < pair[1])
        || inventory
            .all_registry_entry_ids
            .iter()
            .any(|id| parse_registry_entry_id(id).is_err())
        || inventory.t5_volume_uuid != EXPECTED_T5_VOLUME_UUID
    {
        return Err(invalid(
            "pre-attach IOMedia registry inventory is malformed",
        ));
    }
    let expected_nodes = expected.ordered();
    let mut t5_ids = BTreeSet::new();
    for (index, ((label, node), (_, expected_bsd_name))) in inventory
        .ordered_t5()
        .into_iter()
        .zip(expected_nodes)
        .enumerate()
    {
        let media_role = match index {
            0 => IOMediaRole::PhysicalWhole,
            1 => IOMediaRole::PhysicalStore,
            2 => IOMediaRole::ApfsContainer,
            3 => IOMediaRole::ApfsVolume,
            _ => unreachable!("fixed four-node topology"),
        };
        validate_provenance_node(node, expected_bsd_name, media_role, label)?;
        if !inventory
            .all_registry_entry_ids
            .contains(&node.registry_entry_id)
        {
            return Err(invalid(format!(
                "{label} is absent from the full pre-attach registry ID set"
            )));
        }
        if !t5_ids.insert(node.registry_entry_id.as_str()) {
            return Err(invalid("T5 IOMedia roles alias the same registry entry ID"));
        }
    }
    if inventory
        .t5_apfs_volume
        .disk_arbitration
        .media_uuid
        .as_deref()
        != Some(EXPECTED_T5_VOLUME_UUID)
    {
        return Err(invalid(
            "T5 APFS volume Disk Arbitration UUID does not match the canonical T5 volume",
        ));
    }
    validate_four_node_relationships(
        &inventory.t5_physical_whole,
        &inventory.t5_physical_store,
        &inventory.t5_apfs_container,
        &inventory.t5_apfs_volume,
        "T5",
    )
}

fn validate_backing_shape(backing: &DiskImageBackingProvenanceV1) -> Result<(), AcceptanceError> {
    let decoded_path = strict_file_url_path(&backing.disk_image_url)?;
    if backing.schema != BACKING_SCHEMA
        || backing.authority_granted
        || backing.path_authority_granted
        || backing.disk_image_device.class_name != "AppleDiskImageDevice"
        || backing.disk_image_device_ancestor_count != 1
        || backing.disk_image_url_ancestor_count != 1
        || parse_registry_entry_id(&backing.disk_image_device.registry_entry_id).is_err()
        || backing
            .disk_image_device
            .registry_path
            .as_deref()
            .is_none_or(|path| !valid_registry_path(path))
        || decoded_path != backing.canonical_path
        || backing.opened_components.len() < 2
        || backing.opened_components.len() > MAX_BACKING_COMPONENTS
    {
        return Err(invalid(
            "disk-image backing provenance is malformed or ambiguous",
        ));
    }
    let path = Path::new(&backing.canonical_path);
    if !path.is_absolute()
        || backing
            .opened_components
            .first()
            .map(|component| component.path.as_str())
            != Some("/")
        || backing
            .opened_components
            .last()
            .map(|component| component.path.as_str())
            != Some(backing.canonical_path.as_str())
    {
        return Err(invalid(
            "disk-image backing component chain has wrong endpoints",
        ));
    }
    for (index, component) in backing.opened_components.iter().enumerate() {
        let binding = &component.fd_binding;
        let expected_directory = index + 1 != backing.opened_components.len();
        let expected_type = if expected_directory {
            libc::S_IFDIR
        } else {
            libc::S_IFREG
        } as u32;
        let content_binding_is_valid = if expected_directory {
            binding.content_sha256.is_none()
        } else {
            binding.content_sha256.as_deref().is_some_and(valid_sha256)
        };
        if component.directory != expected_directory
            || binding.dev == 0
            || binding.inode == 0
            || binding.nlink == 0
            || !(0..1_000_000_000).contains(&binding.ctime_nanoseconds)
            || !(0..1_000_000_000).contains(&binding.mtime_nanoseconds)
            || binding.mode & libc::S_IFMT as u32 != expected_type
            || !content_binding_is_valid
            || component.path_binding_before != component.fd_binding
            || component.path_binding_after != component.fd_binding
            || (index > 0
                && Path::new(&component.path).parent()
                    != Some(Path::new(&backing.opened_components[index - 1].path)))
        {
            return Err(invalid(
                "disk-image backing component binding is unsafe or discontinuous",
            ));
        }
    }
    let file = &backing
        .opened_components
        .last()
        .expect("checked length")
        .fd_binding;
    if file.nlink != 1 {
        return Err(invalid(
            "disk-image backing file does not have exactly one link",
        ));
    }
    if file.size == 0 || file.size > MAX_BACKING_FILE_BYTES {
        return Err(invalid(
            "disk-image backing file size is outside the bounded digest range",
        ));
    }
    Ok(())
}

pub fn validate_disk_image_backing_identity_v2(
    backing: &DiskImageBackingIdentityV2,
) -> Result<(), AcceptanceError> {
    if backing.schema != BACKING_IDENTITY_SCHEMA
        || backing.authority_granted
        || backing.path_authority_granted
        || backing.opened_components.len() < 2
        || backing.opened_components.len() > MAX_BACKING_COMPONENTS
    {
        return Err(invalid(
            "disk-image backing identity is malformed or grants authority",
        ));
    }
    let path = Path::new(&backing.canonical_path);
    if !path.is_absolute()
        || backing
            .opened_components
            .first()
            .map(|component| component.path.as_str())
            != Some("/")
        || backing
            .opened_components
            .last()
            .map(|component| component.path.as_str())
            != Some(backing.canonical_path.as_str())
    {
        return Err(invalid(
            "disk-image backing identity component chain has wrong endpoints",
        ));
    }
    for (index, component) in backing.opened_components.iter().enumerate() {
        let binding = &component.fd_binding;
        let expected_directory = index + 1 != backing.opened_components.len();
        let expected_type = if expected_directory {
            libc::S_IFDIR
        } else {
            libc::S_IFREG
        } as u32;
        let content_binding_is_valid = if expected_directory {
            binding.content_sha256.is_none()
        } else {
            binding.content_sha256.as_deref().is_some_and(valid_sha256)
        };
        if component.directory != expected_directory
            || binding.dev == 0
            || binding.inode == 0
            || binding.nlink == 0
            || !(0..1_000_000_000).contains(&binding.ctime_nanoseconds)
            || !(0..1_000_000_000).contains(&binding.mtime_nanoseconds)
            || binding.mode & libc::S_IFMT as u32 != expected_type
            || !content_binding_is_valid
            || component.path_binding_before != component.fd_binding
            || component.path_binding_after != component.fd_binding
            || (index > 0
                && Path::new(&component.path).parent()
                    != Some(Path::new(&backing.opened_components[index - 1].path)))
        {
            return Err(invalid(
                "disk-image backing identity component binding is unsafe or discontinuous",
            ));
        }
    }
    let file = &backing
        .opened_components
        .last()
        .expect("checked length")
        .fd_binding;
    if file.nlink != 1 || file.size == 0 || file.size > MAX_BACKING_FILE_BYTES {
        return Err(invalid(
            "disk-image backing identity file has an unsafe link count or size",
        ));
    }
    Ok(())
}

fn bounded_canonical_absolute_path_shape(path: &str) -> bool {
    let path = Path::new(path);
    path.as_os_str().as_encoded_bytes().len() <= MAX_CF_STRING_BYTES
        && !path.as_os_str().as_encoded_bytes().contains(&0)
        && path.is_absolute()
        && path.components().count() <= MAX_BACKING_COMPONENTS
        && path.components().all(|component| {
            matches!(
                component,
                std::path::Component::RootDir | std::path::Component::Normal(_)
            )
        })
}

fn validate_full_binding_v3(
    binding: &FilesystemObjectBindingV3,
    directory: bool,
    allow_zero_links: bool,
    label: &str,
) -> Result<(), AcceptanceError> {
    let expected_type = if directory {
        libc::S_IFDIR
    } else {
        libc::S_IFREG
    } as u32;
    if binding.dev == 0
        || binding.inode == 0
        || (!allow_zero_links && binding.nlink == 0)
        || !(0..1_000_000_000).contains(&binding.birthtime_nanoseconds)
        || !(0..1_000_000_000).contains(&binding.ctime_nanoseconds)
        || !(0..1_000_000_000).contains(&binding.mtime_nanoseconds)
        || binding.mode & libc::S_IFMT as u32 != expected_type
        || (!directory && (binding.size == 0 || binding.size > MAX_BACKING_FILE_BYTES))
    {
        return Err(invalid(format!("{label} full binding is malformed")));
    }
    Ok(())
}

fn same_parent_binding_except_namespace_delta(
    before: &FilesystemObjectBindingV3,
    after: &FilesystemObjectBindingV3,
) -> bool {
    before.birthtime_nanoseconds == after.birthtime_nanoseconds
        && before.birthtime_seconds == after.birthtime_seconds
        && before.dev == after.dev
        && before.flags == after.flags
        && before.generation == after.generation
        && before.gid == after.gid
        && before.inode == after.inode
        && before.mode == after.mode
        && before.uid == after.uid
}

fn validate_exact_ancestor_roster(
    canonical_path: &str,
    ancestors: &[ExactBackingPathComponentV3],
    label: &str,
) -> Result<(), AcceptanceError> {
    if ancestors.is_empty()
        || ancestors.len() >= MAX_BACKING_COMPONENTS
        || ancestors.first().map(|component| component.path.as_str()) != Some("/")
        || ancestors.last().map(|component| Path::new(&component.path))
            != Path::new(canonical_path).parent()
    {
        return Err(invalid(format!(
            "{label} ancestor chain has wrong endpoints"
        )));
    }
    for (index, component) in ancestors.iter().enumerate() {
        validate_full_binding_v3(&component.binding, true, false, label)?;
        if !component.directory
            || (index > 0
                && Path::new(&component.path).parent()
                    != Some(Path::new(&ancestors[index - 1].path)))
        {
            return Err(invalid(format!(
                "{label} ancestor chain is unsafe or discontinuous"
            )));
        }
    }
    Ok(())
}

pub fn validate_exact_disk_image_backing_identity_v3(
    backing: &ExactDiskImageBackingIdentityV3,
) -> Result<(), AcceptanceError> {
    if backing.schema != EXACT_BACKING_IDENTITY_SCHEMA
        || backing.authority_granted
        || !bounded_canonical_absolute_path_shape(&backing.canonical_path)
        || !valid_sha256(&backing.content_sha256)
        || backing.opened_components.len() < 2
        || backing.opened_components.len() > MAX_BACKING_COMPONENTS
        || backing
            .opened_components
            .first()
            .map(|component| component.path.as_str())
            != Some("/")
        || backing
            .opened_components
            .last()
            .map(|component| component.path.as_str())
            != Some(backing.canonical_path.as_str())
    {
        return Err(invalid(
            "exact disk-image backing identity is malformed or grants authority",
        ));
    }
    for (index, component) in backing.opened_components.iter().enumerate() {
        let directory = index + 1 != backing.opened_components.len();
        validate_full_binding_v3(
            &component.binding,
            directory,
            false,
            "exact backing component",
        )?;
        if component.directory != directory
            || (index > 0
                && Path::new(&component.path).parent()
                    != Some(Path::new(&backing.opened_components[index - 1].path)))
        {
            return Err(invalid(
                "exact disk-image backing component chain is unsafe or discontinuous",
            ));
        }
    }
    if backing
        .opened_components
        .last()
        .expect("validated exact backing roster")
        .binding
        .nlink
        != 1
    {
        return Err(invalid(
            "exact disk-image backing terminal does not have one namespace link",
        ));
    }
    Ok(())
}

fn unlinked_state_as_full_binding(state: &UnlinkedBackingFileStateV3) -> FilesystemObjectBindingV3 {
    FilesystemObjectBindingV3 {
        birthtime_nanoseconds: state.birthtime_nanoseconds,
        birthtime_seconds: state.birthtime_seconds,
        ctime_nanoseconds: state.ctime_nanoseconds,
        ctime_seconds: state.ctime_seconds,
        dev: state.dev,
        flags: state.flags,
        generation: state.generation,
        gid: state.gid,
        inode: state.inode,
        mode: state.mode,
        mtime_nanoseconds: state.mtime_nanoseconds,
        mtime_seconds: state.mtime_seconds,
        nlink: state.nlink,
        size: state.size,
        uid: state.uid,
    }
}

fn same_unlinked_terminal_except_ctime_and_nlink(
    before: &UnlinkedBackingFileStateV3,
    after: &UnlinkedBackingFileStateV3,
) -> bool {
    before.birthtime_nanoseconds == after.birthtime_nanoseconds
        && before.birthtime_seconds == after.birthtime_seconds
        && before.dev == after.dev
        && before.flags == after.flags
        && before.generation == after.generation
        && before.gid == after.gid
        && before.inode == after.inode
        && before.mode == after.mode
        && before.mtime_nanoseconds == after.mtime_nanoseconds
        && before.mtime_seconds == after.mtime_seconds
        && before.rdev == after.rdev
        && before.size == after.size
        && before.uid == after.uid
}

pub fn validate_unlinked_backing_binding_v3(
    binding: &UnlinkedBackingBindingV3,
) -> Result<(), AcceptanceError> {
    if binding.schema != UNLINKED_BACKING_SCHEMA
        || binding.kind != UNLINKED_BACKING_KIND
        || binding.authority_granted
        || !bounded_canonical_absolute_path_shape(&binding.canonical_path)
        || !valid_sha256(&binding.content_sha256)
        || !valid_sha256(&binding.prepared_backing_sha256)
    {
        return Err(invalid(
            "unlinked backing binding is malformed or grants authority",
        ));
    }
    validate_exact_ancestor_roster(
        &binding.canonical_path,
        &binding.opened_ancestors_before,
        "unlinked backing initial",
    )?;
    validate_exact_ancestor_roster(
        &binding.canonical_path,
        &binding.opened_ancestors_after,
        "unlinked backing post",
    )?;
    if binding.opened_ancestors_before.len() != binding.opened_ancestors_after.len() {
        return Err(invalid("unlinked backing ancestor roster length changed"));
    }
    let final_ancestor = binding.opened_ancestors_before.len() - 1;
    for (index, (before, after)) in binding
        .opened_ancestors_before
        .iter()
        .zip(&binding.opened_ancestors_after)
        .enumerate()
    {
        if before.path != after.path
            || before.directory != after.directory
            || (index != final_ancestor && before.binding != after.binding)
            || (index == final_ancestor
                && !same_parent_binding_except_namespace_delta(&before.binding, &after.binding))
        {
            return Err(invalid(format!(
                "unlinked backing ancestor {index} changed outside the parent namespace delta: before={before:?} after={after:?}"
            )));
        }
    }
    validate_full_binding_v3(
        &unlinked_state_as_full_binding(&binding.initial_file),
        false,
        false,
        "unlinked backing initial file",
    )?;
    validate_full_binding_v3(
        &unlinked_state_as_full_binding(&binding.post_unlink_file),
        false,
        true,
        "unlinked backing post file",
    )?;
    if binding.initial_file.nlink != 1
        || binding.post_unlink_file.nlink != 0
        || !same_unlinked_terminal_except_ctime_and_nlink(
            &binding.initial_file,
            &binding.post_unlink_file,
        )
    {
        return Err(invalid(
            "unlinked backing terminal is not the exact retained one-link to zero-link transition",
        ));
    }
    Ok(())
}

pub fn validate_backing_path_absence_binding_v3(
    binding: &BackingPathAbsenceBindingV3,
) -> Result<(), AcceptanceError> {
    if binding.schema != BACKING_PATH_ABSENCE_SCHEMA
        || binding.kind != BACKING_PATH_ABSENCE_KIND
        || binding.authority_granted
        || !bounded_canonical_absolute_path_shape(&binding.canonical_path)
        || binding.basename.is_empty()
        || binding.basename.as_bytes().contains(&0)
        || binding.basename.as_bytes().contains(&b'/')
        || !valid_sha256(&binding.prepared_backing_sha256)
    {
        return Err(invalid(
            "backing path-absence binding is malformed or grants authority",
        ));
    }
    validate_exact_ancestor_roster(
        &binding.canonical_path,
        &binding.prepared_ancestors,
        "prepared backing absence",
    )?;
    validate_exact_ancestor_roster(
        &binding.canonical_path,
        &binding.observed_ancestors,
        "observed backing absence",
    )?;
    if Path::new(&binding.canonical_path)
        .file_name()
        .and_then(|name| name.to_str())
        != Some(binding.basename.as_str())
        || binding.prepared_ancestors.len() != binding.observed_ancestors.len()
    {
        return Err(invalid(
            "backing path-absence binding does not match its prepared basename",
        ));
    }
    let final_ancestor = binding.prepared_ancestors.len() - 1;
    for (index, (before, after)) in binding
        .prepared_ancestors
        .iter()
        .zip(&binding.observed_ancestors)
        .enumerate()
    {
        if before.path != after.path
            || before.directory != after.directory
            || (index != final_ancestor && before.binding != after.binding)
            || (index == final_ancestor
                && !same_parent_binding_except_namespace_delta(&before.binding, &after.binding))
        {
            return Err(invalid(format!(
                "backing path-absence ancestor {index} differs from the exact prepared chain: before={before:?} after={after:?}"
            )));
        }
    }
    Ok(())
}

pub fn validate_restart_disk_image_backing_identity_v3(
    backing: &RestartDiskImageBackingIdentityV3,
) -> Result<(), AcceptanceError> {
    let binding = &backing.file_binding;
    let path = Path::new(&backing.canonical_path);
    if backing.schema != RESTART_BACKING_IDENTITY_SCHEMA
        || backing.authority_granted
        || backing.canonical_path.len() > MAX_CF_STRING_BYTES
        || !path.is_absolute()
        || path.components().count() > MAX_BACKING_COMPONENTS
        || path.components().any(|component| {
            !matches!(
                component,
                std::path::Component::RootDir | std::path::Component::Normal(_)
            )
        })
        || binding.content_sha256.is_some()
        || binding.dev == 0
        || binding.inode == 0
        || binding.nlink == 0
        || binding.size == 0
        || binding.mode & libc::S_IFMT as u32 != libc::S_IFREG as u32
        || !(0..1_000_000_000).contains(&binding.ctime_nanoseconds)
        || !(0..1_000_000_000).contains(&binding.mtime_nanoseconds)
    {
        return Err(invalid(
            "restart DiskImageURL backing identity is malformed or grants authority",
        ));
    }
    Ok(())
}

pub fn restart_disk_image_backing_matches_prepared_v3(
    candidate: &RestartDiskImageBackingIdentityV3,
    prepared: &DiskImageBackingIdentityV2,
) -> Result<bool, AcceptanceError> {
    validate_restart_disk_image_backing_identity_v3(candidate)?;
    validate_disk_image_backing_identity_v2(prepared)?;
    let prepared = &prepared
        .opened_components
        .last()
        .expect("validated prepared backing")
        .fd_binding;
    let candidate = &candidate.file_binding;
    Ok(candidate.ctime_nanoseconds == prepared.ctime_nanoseconds
        && candidate.ctime_seconds == prepared.ctime_seconds
        && candidate.dev == prepared.dev
        && candidate.flags == prepared.flags
        && candidate.gid == prepared.gid
        && candidate.inode == prepared.inode
        && candidate.mode == prepared.mode
        && candidate.mtime_nanoseconds == prepared.mtime_nanoseconds
        && candidate.mtime_seconds == prepared.mtime_seconds
        && candidate.nlink == prepared.nlink
        && candidate.size == prepared.size
        && candidate.uid == prepared.uid)
}

pub fn project_disk_image_backing_identity_v2(
    backing: &DiskImageBackingProvenanceV1,
) -> Result<DiskImageBackingIdentityV2, AcceptanceError> {
    validate_backing_shape(backing)?;
    let identity = DiskImageBackingIdentityV2 {
        authority_granted: false,
        canonical_path: backing.canonical_path.clone(),
        opened_components: backing.opened_components.clone(),
        path_authority_granted: false,
        schema: BACKING_IDENTITY_SCHEMA.to_string(),
    };
    validate_disk_image_backing_identity_v2(&identity)?;
    Ok(identity)
}

pub fn validate_iomedia_topology_provenance_shape(
    topology: &AttachedIOMediaTopologyV2,
    expected: ExpectedIOMediaTopology<'_>,
) -> Result<(), AcceptanceError> {
    if topology.schema != PROVENANCE_TOPOLOGY_SCHEMA
        || topology.authority_granted
        || !valid_uuid(&topology.boot_session_uuid)
        || topology.boot_session_uuid != topology.pre_attach_inventory.boot_session_uuid
    {
        return Err(invalid("attached IOMedia provenance topology is malformed"));
    }
    validate_iomedia_registry_inventory_shape(
        &topology.pre_attach_inventory,
        ExpectedIOMediaTopology {
            apfs_container: &topology.pre_attach_inventory.t5_apfs_container.bsd_name,
            apfs_volume: &topology.pre_attach_inventory.t5_apfs_volume.bsd_name,
            physical_store: &topology.pre_attach_inventory.t5_physical_store.bsd_name,
            physical_whole: &topology.pre_attach_inventory.t5_physical_whole.bsd_name,
        },
    )?;
    validate_four_node_topology_shape(
        &topology.fresh_t5,
        ExpectedIOMediaTopology {
            apfs_container: &topology.pre_attach_inventory.t5_apfs_container.bsd_name,
            apfs_volume: &topology.pre_attach_inventory.t5_apfs_volume.bsd_name,
            physical_store: &topology.pre_attach_inventory.t5_physical_store.bsd_name,
            physical_whole: &topology.pre_attach_inventory.t5_physical_whole.bsd_name,
        },
        "fresh T5",
    )?;
    if topology.fresh_t5.boot_session_uuid != topology.boot_session_uuid
        || topology.fresh_t5.physical_whole != topology.pre_attach_inventory.t5_physical_whole
        || topology.fresh_t5.physical_store != topology.pre_attach_inventory.t5_physical_store
        || topology.fresh_t5.apfs_container != topology.pre_attach_inventory.t5_apfs_container
        || topology.fresh_t5.apfs_volume != topology.pre_attach_inventory.t5_apfs_volume
    {
        return Err(invalid(
            "fresh T5 IOMedia topology differs from the pre-attach held inventory",
        ));
    }
    let expected_nodes = expected.ordered();
    let mut attached_ids = BTreeSet::new();
    for (index, ((label, node), (_, expected_bsd_name))) in topology
        .ordered()
        .into_iter()
        .zip(expected_nodes)
        .enumerate()
    {
        let media_role = match index {
            0 => IOMediaRole::PhysicalWhole,
            1 => IOMediaRole::PhysicalStore,
            2 => IOMediaRole::ApfsContainer,
            3 => IOMediaRole::ApfsVolume,
            _ => unreachable!("fixed four-node topology"),
        };
        validate_provenance_node(node, expected_bsd_name, media_role, label)?;
        if !attached_ids.insert(node.registry_entry_id.as_str())
            || topology
                .pre_attach_inventory
                .all_registry_entry_ids
                .contains(&node.registry_entry_id)
        {
            return Err(invalid(
                "attached IOMedia registry ID aliases another role or a pre-existing object",
            ));
        }
    }
    validate_four_node_relationships(
        &topology.physical_whole,
        &topology.physical_store,
        &topology.apfs_container,
        &topology.apfs_volume,
        "attached disk image",
    )?;
    validate_backing_shape(&topology.backing)?;
    if !topology
        .physical_whole
        .ancestry
        .get(1)
        .is_some_and(|entry| entry.class_name == "IOBlockStorageDriver")
        || topology.physical_whole.ancestry.get(2) != Some(&topology.backing.disk_image_device)
    {
        return Err(invalid(
            "attached physical whole does not lead directly through IOBlockStorageDriver to the captured disk-image device",
        ));
    }
    for (_, node) in topology.ordered() {
        let matches = node
            .ancestry
            .iter()
            .filter(|ancestor| {
                ancestor.registry_entry_id == topology.backing.disk_image_device.registry_entry_id
                    && ancestor.class_name == "AppleDiskImageDevice"
            })
            .count();
        if matches != 1 {
            return Err(invalid(
                "attached node does not have the unique captured AppleDiskImageDevice ancestor",
            ));
        }
    }
    Ok(())
}

fn registry_identity(bsd_name: String, registry_entry_id: u64) -> IOMediaRegistryIdentityV1 {
    IOMediaRegistryIdentityV1 {
        authority_granted: false,
        bsd_name,
        registry_entry_id: format!("{registry_entry_id:016x}"),
        schema: IDENTITY_SCHEMA.to_string(),
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::ffi::OsStr;
    use std::fs::File;
    use std::marker::PhantomData;
    use std::os::fd::AsRawFd;
    use std::os::fd::FromRawFd;
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::os::raw::c_uint;
    use std::os::raw::c_void;
    use std::os::unix::ffi::OsStrExt;
    use std::path::PathBuf;
    use std::rc::Rc;

    use sha2::Digest;
    use sha2::Sha256;

    use super::*;

    type CFAllocatorRef = *const c_void;
    type CFBooleanRef = *const c_void;
    type CFDictionaryRef = *const c_void;
    type CFIndex = isize;
    type CFMutableDictionaryRef = *mut c_void;
    type CFNumberRef = *const c_void;
    type CFStringRef = *const c_void;
    type CFTypeRef = *const c_void;
    type CFTypeId = usize;
    type CFUUIDRef = *const c_void;
    type DADiskRef = *const c_void;
    type DASessionRef = *const c_void;
    type IoIterator = c_uint;
    type IoObject = c_uint;
    type IoRegistryEntry = c_uint;
    type IoService = c_uint;
    type KernReturn = c_int;
    type MachPort = c_uint;

    const IO_OBJECT_NULL: IoObject = 0;
    const KERN_SUCCESS: KernReturn = 0;
    const K_IO_RETURN_NO_DEVICE: KernReturn = 0xe000_02c0_u32 as i32;
    const K_IOMAIN_PORT_DEFAULT: MachPort = 0;
    const K_CF_NUMBER_SINT64_TYPE: c_int = 4;
    const K_CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFBooleanGetTypeID() -> CFTypeId;
        fn CFBooleanGetValue(boolean: CFBooleanRef) -> u8;
        fn CFDictionaryGetValue(dictionary: CFDictionaryRef, key: *const c_void) -> *const c_void;
        fn CFGetTypeID(value: CFTypeRef) -> CFTypeId;
        fn CFNumberGetTypeID() -> CFTypeId;
        fn CFNumberGetValue(number: CFNumberRef, number_type: c_int, value: *mut c_void) -> u8;
        fn CFRelease(value: CFTypeRef);
        fn CFRetain(value: CFTypeRef) -> CFTypeRef;
        fn CFStringCreateWithCString(
            allocator: CFAllocatorRef,
            string: *const c_char,
            encoding: u32,
        ) -> CFStringRef;
        fn CFStringGetCString(
            string: CFStringRef,
            buffer: *mut c_char,
            buffer_size: CFIndex,
            encoding: u32,
        ) -> u8;
        fn CFStringGetLength(string: CFStringRef) -> CFIndex;
        fn CFStringGetMaximumSizeForEncoding(length: CFIndex, encoding: u32) -> CFIndex;
        fn CFStringGetTypeID() -> CFTypeId;
        fn CFUUIDCreateString(allocator: CFAllocatorRef, uuid: CFUUIDRef) -> CFStringRef;
        fn CFUUIDGetTypeID() -> CFTypeId;
    }

    #[link(name = "DiskArbitration", kind = "framework")]
    unsafe extern "C" {
        fn DADiskCopyDescription(disk: DADiskRef) -> CFDictionaryRef;
        fn DASessionCreate(allocator: CFAllocatorRef) -> DASessionRef;
        fn DADiskCreateFromIOMedia(
            allocator: CFAllocatorRef,
            session: DASessionRef,
            media: IoService,
        ) -> DADiskRef;
        fn DADiskCopyIOMedia(disk: DADiskRef) -> IoService;
        fn DADiskCopyWholeDisk(disk: DADiskRef) -> DADiskRef;
        fn DADiskGetBSDName(disk: DADiskRef) -> *const c_char;

        static kDADiskDescriptionDeviceInternalKey: CFStringRef;
        static kDADiskDescriptionMediaBlockSizeKey: CFStringRef;
        static kDADiskDescriptionMediaContentKey: CFStringRef;
        static kDADiskDescriptionMediaEjectableKey: CFStringRef;
        static kDADiskDescriptionMediaLeafKey: CFStringRef;
        static kDADiskDescriptionMediaRemovableKey: CFStringRef;
        static kDADiskDescriptionMediaSizeKey: CFStringRef;
        static kDADiskDescriptionMediaUUIDKey: CFStringRef;
        static kDADiskDescriptionMediaWholeKey: CFStringRef;
        static kDADiskDescriptionMediaWritableKey: CFStringRef;
    }

    #[link(name = "IOKit", kind = "framework")]
    unsafe extern "C" {
        fn IOIteratorNext(iterator: IoIterator) -> IoObject;
        fn IOObjectConformsTo(object: IoObject, class_name: *const c_char) -> libc::boolean_t;
        fn IOObjectCopyClass(object: IoObject) -> CFStringRef;
        fn IOObjectRelease(object: IoObject) -> KernReturn;
        fn IORegistryEntryGetRegistryEntryID(
            entry: IoRegistryEntry,
            entry_id: *mut u64,
        ) -> KernReturn;
        fn IORegistryEntryCopyPath(entry: IoRegistryEntry, plane: *const c_char) -> CFStringRef;
        fn IORegistryEntryCreateCFProperty(
            entry: IoRegistryEntry,
            key: CFStringRef,
            allocator: CFAllocatorRef,
            options: u32,
        ) -> CFTypeRef;
        fn IORegistryEntryGetParentEntry(
            entry: IoRegistryEntry,
            plane: *const c_char,
            parent: *mut IoRegistryEntry,
        ) -> KernReturn;
        fn IORegistryEntryIDMatching(entry_id: u64) -> CFMutableDictionaryRef;
        fn IOServiceGetMatchingService(main_port: MachPort, matching: CFDictionaryRef)
        -> IoService;
        fn IOServiceGetMatchingServices(
            main_port: MachPort,
            matching: CFDictionaryRef,
            existing: *mut IoIterator,
        ) -> KernReturn;
        fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;
    }

    pub(super) fn current_boot_session_uuid_impl() -> Result<String, AcceptanceError> {
        let name = CString::new("kern.bootsessionuuid").expect("fixed sysctl name");
        let mut length = 0_usize;
        if unsafe {
            libc::sysctlbyname(
                name.as_ptr(),
                std::ptr::null_mut(),
                &mut length,
                std::ptr::null_mut(),
                0,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        if !(2..=128).contains(&length) {
            return Err(invalid("boot session UUID sysctl length is invalid"));
        }
        let mut bytes = vec![0_u8; length];
        if unsafe {
            libc::sysctlbyname(
                name.as_ptr(),
                bytes.as_mut_ptr().cast(),
                &mut length,
                std::ptr::null_mut(),
                0,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        bytes.truncate(length);
        if bytes.last() == Some(&0) {
            bytes.pop();
        }
        let value = String::from_utf8(bytes)
            .map_err(|_| invalid("boot session UUID is not UTF-8"))?
            .to_ascii_lowercase();
        if !valid_uuid(&value) {
            return Err(invalid("boot session UUID is malformed"));
        }
        Ok(value)
    }

    fn monotonic_nanoseconds() -> Result<u64, AcceptanceError> {
        let mut time = std::mem::MaybeUninit::<libc::timespec>::zeroed();
        if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_RAW, time.as_mut_ptr()) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let time = unsafe { time.assume_init() };
        if time.tv_sec < 0 || time.tv_nsec < 0 {
            return Err(invalid("monotonic clock returned a negative epoch"));
        }
        (time.tv_sec as u64)
            .checked_mul(1_000_000_000)
            .and_then(|seconds| seconds.checked_add(time.tv_nsec as u64))
            .filter(|value| *value != 0)
            .ok_or_else(|| invalid("monotonic epoch overflowed or was zero"))
    }

    struct Session(DASessionRef);

    impl Session {
        fn create() -> Result<Self, AcceptanceError> {
            let session = unsafe { DASessionCreate(std::ptr::null()) };
            if session.is_null() {
                return Err(invalid("DiskArbitration session creation failed"));
            }
            Ok(Self(session))
        }
    }

    impl Drop for Session {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    impl Clone for Session {
        fn clone(&self) -> Self {
            unsafe { CFRetain(self.0) };
            Self(self.0)
        }
    }

    struct CfOwned(CFTypeRef);

    impl CfOwned {
        fn new(value: CFTypeRef, label: &str) -> Result<Self, AcceptanceError> {
            if value.is_null() {
                return Err(invalid(format!(
                    "{label} returned no CoreFoundation object"
                )));
            }
            Ok(Self(value))
        }

        fn as_dictionary(&self) -> CFDictionaryRef {
            self.0.cast()
        }

        fn as_string(&self) -> CFStringRef {
            self.0.cast()
        }
    }

    impl Drop for CfOwned {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    struct IoObjectGuard(IoObject);

    impl IoObjectGuard {
        fn new(object: IoObject, label: &str) -> Result<Self, AcceptanceError> {
            if object == IO_OBJECT_NULL {
                return Err(invalid(format!("{label} returned no IOKit object")));
            }
            Ok(Self(object))
        }
    }

    impl Drop for IoObjectGuard {
        fn drop(&mut self) {
            if self.0 != IO_OBJECT_NULL {
                let _ = unsafe { IOObjectRelease(self.0) };
            }
        }
    }

    struct DADiskGuard(DADiskRef);

    impl DADiskGuard {
        fn new(disk: DADiskRef) -> Result<Self, AcceptanceError> {
            if disk.is_null() {
                return Err(invalid("DADiskCreateFromIOMedia returned no disk"));
            }
            Ok(Self(disk))
        }
    }

    impl Drop for DADiskGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    fn cf_string(value: CFStringRef, label: &str) -> Result<String, AcceptanceError> {
        if value.is_null() || unsafe { CFGetTypeID(value.cast()) } != unsafe { CFStringGetTypeID() }
        {
            return Err(invalid(format!("{label} is not a CFString")));
        }
        let length = unsafe { CFStringGetLength(value) };
        if length < 0 || length as usize > MAX_CF_STRING_BYTES {
            return Err(invalid(format!(
                "{label} exceeds the CFString length bound"
            )));
        }
        let maximum =
            unsafe { CFStringGetMaximumSizeForEncoding(length, K_CF_STRING_ENCODING_UTF8) };
        if maximum < 0 || maximum as usize >= MAX_CF_STRING_BYTES {
            return Err(invalid(format!("{label} exceeds the UTF-8 byte bound")));
        }
        let mut bytes = vec![0_u8; maximum as usize + 1];
        if unsafe {
            CFStringGetCString(
                value,
                bytes.as_mut_ptr().cast(),
                bytes.len() as CFIndex,
                K_CF_STRING_ENCODING_UTF8,
            )
        } == 0
        {
            return Err(invalid(format!("{label} is not representable as UTF-8")));
        }
        let nul = bytes
            .iter()
            .position(|byte| *byte == 0)
            .ok_or_else(|| invalid(format!("{label} UTF-8 conversion was not terminated")))?;
        bytes.truncate(nul);
        String::from_utf8(bytes).map_err(|_| invalid(format!("{label} is not UTF-8")))
    }

    fn cf_key(value: &str) -> Result<CfOwned, AcceptanceError> {
        let value = CString::new(value).expect("fixed IOKit property key");
        CfOwned::new(
            unsafe {
                CFStringCreateWithCString(
                    std::ptr::null(),
                    value.as_ptr(),
                    K_CF_STRING_ENCODING_UTF8,
                )
            }
            .cast(),
            "CFStringCreateWithCString",
        )
    }

    fn registry_property(
        entry: IoRegistryEntry,
        key: &str,
    ) -> Result<Option<CfOwned>, AcceptanceError> {
        let key = cf_key(key)?;
        let value =
            unsafe { IORegistryEntryCreateCFProperty(entry, key.as_string(), std::ptr::null(), 0) };
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(CfOwned(value)))
        }
    }

    fn typed_boolean(value: CFTypeRef, label: &str) -> Result<bool, AcceptanceError> {
        if value.is_null() || unsafe { CFGetTypeID(value) } != unsafe { CFBooleanGetTypeID() } {
            return Err(invalid(format!("{label} is not a CFBoolean")));
        }
        Ok(unsafe { CFBooleanGetValue(value.cast()) } != 0)
    }

    fn typed_u64(value: CFTypeRef, label: &str) -> Result<u64, AcceptanceError> {
        if value.is_null() || unsafe { CFGetTypeID(value) } != unsafe { CFNumberGetTypeID() } {
            return Err(invalid(format!("{label} is not a CFNumber")));
        }
        let mut number = 0_i64;
        if unsafe {
            CFNumberGetValue(
                value.cast(),
                K_CF_NUMBER_SINT64_TYPE,
                (&mut number as *mut i64).cast(),
            )
        } == 0
            || number < 0
        {
            return Err(invalid(format!(
                "{label} is negative or does not fit signed 64-bit"
            )));
        }
        Ok(number as u64)
    }

    fn optional_registry_boolean(
        entry: IoRegistryEntry,
        key: &str,
    ) -> Result<Option<bool>, AcceptanceError> {
        registry_property(entry, key)?
            .map(|value| typed_boolean(value.0, key))
            .transpose()
    }

    fn optional_registry_u64(
        entry: IoRegistryEntry,
        key: &str,
    ) -> Result<Option<u64>, AcceptanceError> {
        registry_property(entry, key)?
            .map(|value| typed_u64(value.0, key))
            .transpose()
    }

    fn optional_registry_string(
        entry: IoRegistryEntry,
        key: &str,
    ) -> Result<Option<String>, AcceptanceError> {
        registry_property(entry, key)?
            .map(|value| cf_string(value.as_string(), key))
            .transpose()
    }

    fn dictionary_value(dictionary: CFDictionaryRef, key: CFStringRef) -> Option<CFTypeRef> {
        if dictionary.is_null() || key.is_null() {
            return None;
        }
        let value = unsafe { CFDictionaryGetValue(dictionary, key.cast()) };
        (!value.is_null()).then_some(value.cast())
    }

    fn optional_da_boolean(
        dictionary: CFDictionaryRef,
        key: CFStringRef,
        label: &str,
    ) -> Result<Option<bool>, AcceptanceError> {
        dictionary_value(dictionary, key)
            .map(|value| typed_boolean(value, label))
            .transpose()
    }

    fn optional_da_u64(
        dictionary: CFDictionaryRef,
        key: CFStringRef,
        label: &str,
    ) -> Result<Option<u64>, AcceptanceError> {
        dictionary_value(dictionary, key)
            .map(|value| typed_u64(value, label))
            .transpose()
    }

    fn optional_da_string(
        dictionary: CFDictionaryRef,
        key: CFStringRef,
        label: &str,
    ) -> Result<Option<String>, AcceptanceError> {
        dictionary_value(dictionary, key)
            .map(|value| cf_string(value.cast(), label))
            .transpose()
    }

    fn optional_da_uuid(
        dictionary: CFDictionaryRef,
        key: CFStringRef,
        label: &str,
    ) -> Result<Option<String>, AcceptanceError> {
        let Some(value) = dictionary_value(dictionary, key) else {
            return Ok(None);
        };
        if unsafe { CFGetTypeID(value) } != unsafe { CFUUIDGetTypeID() } {
            return Err(invalid(format!("{label} is not a CFUUID")));
        }
        let string = CfOwned::new(
            unsafe { CFUUIDCreateString(std::ptr::null(), value.cast()) }.cast(),
            label,
        )?;
        let uuid = cf_string(string.as_string(), label)?.to_ascii_lowercase();
        if !valid_uuid(&uuid) {
            return Err(invalid(format!("{label} is malformed")));
        }
        Ok(Some(uuid))
    }

    fn registry_class(entry: IoRegistryEntry) -> Result<String, AcceptanceError> {
        let class = CfOwned::new(
            unsafe { IOObjectCopyClass(entry) }.cast(),
            "IOObjectCopyClass",
        )?;
        let class = cf_string(class.as_string(), "IORegistry class")?;
        if !valid_class_name(&class) {
            return Err(invalid("IORegistry class name is malformed"));
        }
        Ok(class)
    }

    fn registry_path(entry: IoRegistryEntry) -> Result<Option<String>, AcceptanceError> {
        let plane = CString::new("IOService").expect("fixed IORegistry plane");
        let raw_path = unsafe { IORegistryEntryCopyPath(entry, plane.as_ptr()) };
        if raw_path.is_null() {
            return Ok(None);
        }
        let path = CfOwned(raw_path.cast());
        let path = cf_string(path.as_string(), "IORegistry path")?;
        if !valid_registry_path(&path) {
            return Err(invalid("IORegistry path is malformed"));
        }
        Ok(Some(path))
    }

    struct HeldAncestry {
        disk_image_candidates: Vec<DiskImageCandidateObservation>,
        disk_image_device_count: usize,
        report: Vec<IORegistryAncestorV1>,
        _retained: Vec<IoObjectGuard>,
    }

    impl HeldAncestry {
        fn revalidate_retained(&self, label: &str) -> Result<(), AcceptanceError> {
            if self.report.len() != self._retained.len() + 1 {
                return Err(invalid(format!(
                    "{label} held ancestry descriptor count changed"
                )));
            }
            for (index, (object, expected)) in self
                ._retained
                .iter()
                .zip(self.report.iter().skip(1))
                .enumerate()
            {
                let actual = IORegistryAncestorV1 {
                    class_name: registry_class(object.0)?,
                    registry_entry_id: format!(
                        "{:016x}",
                        registry_id(object.0, &format!("{label} ancestor {index}"))?
                    ),
                    registry_path: registry_path(object.0)?,
                };
                if &actual != expected {
                    return Err(invalid(format!(
                        "{label} exact held ancestor {index} changed"
                    )));
                }
            }
            Ok(())
        }
    }

    fn ancestor_chain(entry: IoRegistryEntry) -> Result<HeldAncestry, AcceptanceError> {
        let plane = CString::new("IOService").expect("fixed IORegistry plane");
        let mut ancestry = Vec::new();
        let mut candidates = Vec::new();
        let mut disk_image_device_count = 0_usize;
        let mut retained = Vec::new();
        let mut current = entry;
        let mut seen = BTreeSet::new();
        loop {
            if ancestry.len() == MAX_ANCESTOR_DEPTH {
                return Err(invalid("IOService ancestry exceeds the depth bound"));
            }
            let registry_entry_id = registry_id(current, "IOService ancestor")?;
            if !seen.insert(registry_entry_id) {
                return Err(invalid("IOService ancestry contains a registry-ID cycle"));
            }
            let class_name = registry_class(current)?;
            let captured_path = registry_path(current).map_err(|error| {
                invalid(format!(
                    "IORegistry path capture failed for {class_name} {registry_entry_id:016x}: {error}"
                ))
            })?;
            let has_captured_path = captured_path.is_some();
            let ancestor = IORegistryAncestorV1 {
                class_name: class_name.clone(),
                registry_entry_id: format!("{registry_entry_id:016x}"),
                registry_path: captured_path,
            };
            if class_name == "AppleDiskImageDevice" {
                disk_image_device_count += 1;
            }
            if let Some(property) = registry_property(current, "DiskImageURL")? {
                if class_name != "AppleDiskImageDevice" {
                    return Err(invalid(
                        "DiskImageURL exists on a non-AppleDiskImageDevice ancestor",
                    ));
                }
                candidates.push(DiskImageCandidateObservation {
                    device: ancestor.clone(),
                    url: cf_string(property.as_string(), "DiskImageURL")?,
                });
            }
            ancestry.push(ancestor);
            let mut parent = IO_OBJECT_NULL;
            let rc = unsafe { IORegistryEntryGetParentEntry(current, plane.as_ptr(), &mut parent) };
            if rc == K_IO_RETURN_NO_DEVICE {
                if parent != IO_OBJECT_NULL {
                    return Err(invalid(
                        "IORegistry parent termination returned a stray object",
                    ));
                }
                if class_name != "IORegistryEntry" || has_captured_path {
                    return Err(invalid(
                        "IORegistry ancestry terminal must be a pathless IORegistryEntry root",
                    ));
                }
                break;
            }
            if rc != KERN_SUCCESS || parent == IO_OBJECT_NULL {
                return Err(invalid(format!(
                    "IORegistryEntryGetParentEntry failed with IOKit status 0x{rc:x}"
                )));
            }
            if !has_captured_path {
                return Err(invalid(
                    "nonterminal IORegistry ancestry entry has no registry path",
                ));
            }
            retained.push(IoObjectGuard(parent));
            current = parent;
        }
        Ok(HeldAncestry {
            disk_image_candidates: candidates,
            disk_image_device_count,
            report: ancestry,
            _retained: retained,
        })
    }

    pub struct ResolvedIOMediaObject {
        report: IOMediaRegistryIdentityV1,
        // Rust drops fields in declaration order. Keep the dependent DADisk
        // media, DADiskRef, and matched IOMedia ahead of the DASession so the
        // session is always the final CoreFoundation object released.
        _disk_media: IoObjectGuard,
        _whole_disk_media: IoObjectGuard,
        _whole_disk: DADiskGuard,
        _disk: DADiskGuard,
        _matched_media: IoObjectGuard,
        _session: Session,
        _not_send_or_sync: PhantomData<Rc<()>>,
    }

    pub struct HeldDiskImageBacking {
        canonical_path: String,
        component_names: Vec<CString>,
        component_paths: Vec<String>,
        files: Vec<File>,
        path_bindings_before: Vec<BackingObjectBindingV1>,
        path_bindings_full: Vec<FilesystemObjectBindingV3>,
        terminal_initial: UnlinkedBackingFileStateV3,
        _not_send_or_sync: PhantomData<Rc<()>>,
    }

    /// Opaque proof capsule retaining the original, now-zero-link backing
    /// inode plus the exact descriptor-anchored ancestor chain.  It exposes
    /// only canonical evidence and replay, never a descriptor or an effect.
    #[must_use = "held unlinked backing requires post-persistence revalidation"]
    pub struct HeldUnlinkedDiskImageBackingV3 {
        binding: UnlinkedBackingBindingV3,
        component_names: Vec<CString>,
        files: Vec<File>,
        _not_send_or_sync: PhantomData<Rc<()>>,
    }

    /// Opaque restart-only proof that the prepared basename is absent beneath
    /// the same exact ancestor chain.  No descriptor for the historical file
    /// is retained or reconstructed.
    #[must_use = "held backing path absence requires post-persistence revalidation"]
    pub struct HeldBackingPathAbsenceV3 {
        basename: CString,
        binding: BackingPathAbsenceBindingV3,
        component_names: Vec<CString>,
        files: Vec<File>,
        _not_send_or_sync: PhantomData<Rc<()>>,
    }

    /// A bounded full-registry restart census. Every IOMedia, DADisk,
    /// whole-disk, and ancestry handle remains live until the caller has
    /// persisted its canonical collector receipt and requests final replay.
    #[must_use = "restart IOMedia handles must outlive collector receipt persistence"]
    pub struct HeldRestartIOMediaInventoryV3 {
        held_backings: Vec<HeldRestartCandidateBackingV3>,
        held_nodes: Vec<HeldRestartNodeV3>,
        report: RestartIOMediaInventoryV3,
        _not_send_or_sync: PhantomData<Rc<()>>,
    }

    struct HeldRestartCandidateBackingV3 {
        canonical_path: String,
        disk_image_url: String,
        disk_image_url_path: String,
        file: File,
        identity: RestartDiskImageBackingIdentityV3,
    }

    struct HeldRestartNodeV3 {
        candidate: Option<RestartDiskImageCandidateV3>,
        captured: CapturedNode,
    }

    /// Opaque, non-serializable capsule retaining both exact pre-attach T5
    /// captures. The report can be cloned into canonical receipts, but the
    /// IOKit, Disk Arbitration, and ancestry handles remain owned here.
    #[must_use = "held pre-attach descriptors must outlive attached receipt persistence"]
    pub struct HeldPreAttachIOMediaInventory {
        report: IOMediaRegistryInventoryV2,
        t5_capture: CapturedFour,
        t5_replay: CapturedFour,
        _not_send_or_sync: PhantomData<Rc<()>>,
    }

    /// Opaque, non-serializable capsule retaining the exact attached and T5
    /// descriptor sets, the pre-attach capsule, and the backing descriptor
    /// chain until the caller has durably persisted canonical receipt bytes
    /// and explicitly requested the final replay.
    #[must_use = "held attached descriptors require post-persistence revalidation"]
    pub struct HeldAttachedIOMediaTopologyV2<'a> {
        report: AttachedIOMediaTopologyV2,
        pre_attach: &'a HeldPreAttachIOMediaInventory,
        held_backing: HeldDiskImageBacking,
        fresh_t5_capture: CapturedFour,
        attached: CapturedFour,
        attached_replay: CapturedFour,
        t5_replay: CapturedFour,
        _not_send_or_sync: PhantomData<Rc<()>>,
    }

    fn backing_binding_from_stat(
        stat: &libc::stat,
        label: &str,
    ) -> Result<BackingObjectBindingV1, AcceptanceError> {
        Ok(legacy_backing_binding(
            backing_full_binding_from_stat(stat, label)?,
            None,
        ))
    }

    fn backing_full_binding_from_stat(
        stat: &libc::stat,
        label: &str,
    ) -> Result<FilesystemObjectBindingV3, AcceptanceError> {
        if stat.st_size < 0
            || !(0..1_000_000_000).contains(&stat.st_birthtime_nsec)
            || !(0..1_000_000_000).contains(&stat.st_ctime_nsec)
            || !(0..1_000_000_000).contains(&stat.st_mtime_nsec)
        {
            return Err(invalid(format!("{label} stat fields are invalid")));
        }
        let binding = FilesystemObjectBindingV3 {
            birthtime_nanoseconds: stat.st_birthtime_nsec,
            birthtime_seconds: stat.st_birthtime,
            ctime_nanoseconds: stat.st_ctime_nsec,
            ctime_seconds: stat.st_ctime,
            dev: stat.st_dev as u64,
            flags: stat.st_flags,
            generation: stat.st_gen,
            gid: stat.st_gid,
            inode: stat.st_ino,
            mode: stat.st_mode as u32,
            mtime_nanoseconds: stat.st_mtime_nsec,
            mtime_seconds: stat.st_mtime,
            nlink: stat.st_nlink as u64,
            size: stat.st_size as u64,
            uid: stat.st_uid,
        };
        if binding.dev == 0 || binding.inode == 0 || binding.nlink == 0 {
            return Err(invalid(format!("{label} full stat binding is invalid")));
        }
        Ok(binding)
    }

    fn legacy_backing_binding(
        binding: FilesystemObjectBindingV3,
        content_sha256: Option<&str>,
    ) -> BackingObjectBindingV1 {
        BackingObjectBindingV1 {
            content_sha256: content_sha256.map(str::to_string),
            ctime_nanoseconds: binding.ctime_nanoseconds,
            ctime_seconds: binding.ctime_seconds,
            dev: binding.dev,
            flags: binding.flags,
            gid: binding.gid,
            inode: binding.inode,
            mode: binding.mode,
            mtime_nanoseconds: binding.mtime_nanoseconds,
            mtime_seconds: binding.mtime_seconds,
            nlink: binding.nlink,
            size: binding.size,
            uid: binding.uid,
        }
    }

    fn backing_full_fstat(
        fd: c_int,
        label: &str,
    ) -> Result<FilesystemObjectBindingV3, AcceptanceError> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
        if unsafe { libc::fstat(fd, stat.as_mut_ptr()) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        backing_full_binding_from_stat(&unsafe { stat.assume_init() }, label)
    }

    fn backing_full_fstatat(
        directory_fd: c_int,
        name: &CStr,
        label: &str,
    ) -> Result<FilesystemObjectBindingV3, AcceptanceError> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
        if unsafe {
            libc::fstatat(
                directory_fd,
                name.as_ptr(),
                stat.as_mut_ptr(),
                libc::AT_SYMLINK_NOFOLLOW,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        backing_full_binding_from_stat(&unsafe { stat.assume_init() }, label)
    }

    fn unlinked_file_state_from_stat(
        stat: &libc::stat,
        label: &str,
    ) -> Result<UnlinkedBackingFileStateV3, AcceptanceError> {
        if stat.st_size < 0
            || !(0..1_000_000_000).contains(&stat.st_birthtime_nsec)
            || !(0..1_000_000_000).contains(&stat.st_ctime_nsec)
            || !(0..1_000_000_000).contains(&stat.st_mtime_nsec)
        {
            return Err(invalid(format!("{label} stat fields are invalid")));
        }
        let state = UnlinkedBackingFileStateV3 {
            birthtime_nanoseconds: stat.st_birthtime_nsec,
            birthtime_seconds: stat.st_birthtime,
            ctime_nanoseconds: stat.st_ctime_nsec,
            ctime_seconds: stat.st_ctime,
            dev: stat.st_dev as u64,
            flags: stat.st_flags,
            generation: stat.st_gen,
            gid: stat.st_gid,
            inode: stat.st_ino,
            mode: stat.st_mode as u32,
            mtime_nanoseconds: stat.st_mtime_nsec,
            mtime_seconds: stat.st_mtime,
            nlink: stat.st_nlink as u64,
            rdev: stat.st_rdev as u64,
            size: stat.st_size as u64,
            uid: stat.st_uid,
        };
        validate_full_binding_v3(&unlinked_state_as_full_binding(&state), false, true, label)?;
        Ok(state)
    }

    fn unlinked_file_fstat(
        fd: c_int,
        label: &str,
    ) -> Result<UnlinkedBackingFileStateV3, AcceptanceError> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
        if unsafe { libc::fstat(fd, stat.as_mut_ptr()) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        unlinked_file_state_from_stat(&unsafe { stat.assume_init() }, label)
    }

    fn require_backing_path_absent(
        parent_fd: c_int,
        basename: &CStr,
        label: &str,
    ) -> Result<(), AcceptanceError> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
        if unsafe {
            libc::fstatat(
                parent_fd,
                basename.as_ptr(),
                stat.as_mut_ptr(),
                libc::AT_SYMLINK_NOFOLLOW,
            )
        } == 0
        {
            return Err(invalid(format!(
                "{label} basename exists instead of being namespace-absent"
            )));
        }
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() != Some(libc::ENOENT) {
            return Err(error.into());
        }
        Ok(())
    }

    fn capture_held_ancestor_bindings(
        files: &[File],
        component_names: &[CString],
        component_paths: &[String],
        label: &str,
    ) -> Result<Vec<ExactBackingPathComponentV3>, AcceptanceError> {
        if files.is_empty()
            || files.len() != component_names.len()
            || files.len() != component_paths.len()
        {
            return Err(invalid(format!(
                "{label} held ancestor descriptor roster changed"
            )));
        }
        let mut observed = Vec::with_capacity(files.len());
        for index in 0..files.len() {
            let fd_binding = backing_full_fstat(files[index].as_raw_fd(), label)?;
            let path_binding = if index == 0 {
                backing_full_fstatat(libc::AT_FDCWD, &component_names[index], label)?
            } else {
                backing_full_fstatat(files[index - 1].as_raw_fd(), &component_names[index], label)?
            };
            validate_full_binding_v3(&fd_binding, true, false, label)?;
            if fd_binding != path_binding {
                return Err(invalid(format!(
                    "{label} descriptor and retained-parent pathname disagree"
                )));
            }
            observed.push(ExactBackingPathComponentV3 {
                binding: fd_binding,
                directory: true,
                path: component_paths[index].clone(),
            });
        }
        Ok(observed)
    }

    fn backing_fstat(fd: c_int, label: &str) -> Result<BackingObjectBindingV1, AcceptanceError> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
        if unsafe { libc::fstat(fd, stat.as_mut_ptr()) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let binding = backing_binding_from_stat(&unsafe { stat.assume_init() }, label)?;
        if binding.dev == 0 || binding.inode == 0 || binding.nlink == 0 {
            return Err(invalid(format!("{label} fstat binding is invalid")));
        }
        Ok(binding)
    }

    fn held_fd_content_sha256(
        fd: c_int,
        size: u64,
        label: &str,
    ) -> Result<String, AcceptanceError> {
        if size == 0 || size > MAX_BACKING_FILE_BYTES || size > i64::MAX as u64 {
            return Err(invalid(format!(
                "{label} size is outside the bounded digest range"
            )));
        }
        let mut digest = Sha256::new();
        let mut buffer = [0_u8; 128 * 1024];
        let mut offset = 0_u64;
        while offset < size {
            let remaining = usize::try_from((size - offset).min(buffer.len() as u64))
                .expect("bounded digest chunk fits usize");
            let read = unsafe {
                libc::pread(
                    fd,
                    buffer.as_mut_ptr().cast(),
                    remaining,
                    offset as libc::off_t,
                )
            };
            if read < 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            if read == 0 {
                return Err(invalid(format!(
                    "{label} reached EOF before its bound size"
                )));
            }
            let read = read as usize;
            digest.update(&buffer[..read]);
            offset += read as u64;
        }
        Ok(format!("{:x}", digest.finalize()))
    }

    fn with_content_sha256(
        mut binding: BackingObjectBindingV1,
        content_sha256: Option<&str>,
    ) -> BackingObjectBindingV1 {
        binding.content_sha256 = content_sha256.map(str::to_string);
        binding
    }

    fn backing_fstatat(
        directory_fd: c_int,
        name: &CStr,
        label: &str,
    ) -> Result<BackingObjectBindingV1, AcceptanceError> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::zeroed();
        if unsafe {
            libc::fstatat(
                directory_fd,
                name.as_ptr(),
                stat.as_mut_ptr(),
                libc::AT_SYMLINK_NOFOLLOW,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        let binding = backing_binding_from_stat(&unsafe { stat.assume_init() }, label)?;
        if binding.dev == 0 || binding.inode == 0 || binding.nlink == 0 {
            return Err(invalid(format!("{label} fstatat binding is invalid")));
        }
        Ok(binding)
    }

    fn require_backing_kind(
        binding: &BackingObjectBindingV1,
        directory: bool,
        label: &str,
    ) -> Result<(), AcceptanceError> {
        let expected = if directory {
            libc::S_IFDIR
        } else {
            libc::S_IFREG
        } as u32;
        if binding.mode & libc::S_IFMT as u32 != expected {
            return Err(invalid(format!("{label} has the wrong file type")));
        }
        Ok(())
    }

    impl HeldDiskImageBacking {
        pub(super) fn capture(path: &Path) -> Result<Self, AcceptanceError> {
            let path_text = path
                .to_str()
                .ok_or_else(|| invalid("disk-image backing path is not UTF-8"))?;
            if path_text.len() > MAX_CF_STRING_BYTES
                || !path.is_absolute()
                || path.components().count() > MAX_BACKING_COMPONENTS
                || path.components().any(|component| {
                    !matches!(
                        component,
                        std::path::Component::RootDir | std::path::Component::Normal(_)
                    )
                })
            {
                return Err(invalid(
                    "disk-image backing path is not a bounded canonical absolute path",
                ));
            }
            let canonical = std::fs::canonicalize(path)?;
            if canonical != path {
                return Err(invalid(
                    "disk-image backing path has a symlink or noncanonical component",
                ));
            }

            let root_name = CString::new("/").expect("fixed root path");
            let root_before_full =
                backing_full_fstatat(libc::AT_FDCWD, &root_name, "disk-image root before open")?;
            let root_before = legacy_backing_binding(root_before_full, None);
            let root_fd = unsafe {
                libc::open(
                    root_name.as_ptr(),
                    libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_DIRECTORY,
                )
            };
            if root_fd < 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            let root = unsafe { File::from_raw_fd(root_fd) };
            let root_fd_binding_full = backing_full_fstat(root.as_raw_fd(), "disk-image root fd")?;
            let root_after_full =
                backing_full_fstatat(libc::AT_FDCWD, &root_name, "disk-image root after open")?;
            if root_before_full != root_fd_binding_full || root_before_full != root_after_full {
                return Err(invalid("disk-image root changed across descriptor open"));
            }
            require_backing_kind(&root_before, true, "disk-image root")?;

            let mut files = vec![root];
            let mut component_names = vec![root_name];
            let mut component_paths = vec!["/".to_string()];
            let mut path_bindings_before = vec![root_before];
            let mut path_bindings_full = vec![root_before_full];
            let normal_components = path
                .components()
                .filter_map(|component| match component {
                    std::path::Component::Normal(name) => Some(name),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let mut current_path = PathBuf::from("/");
            for (index, name) in normal_components.iter().enumerate() {
                let name = CString::new(name.as_bytes())
                    .map_err(|_| invalid("disk-image backing component contains NUL"))?;
                let directory = index + 1 != normal_components.len();
                let parent_fd = files.last().expect("root retained").as_raw_fd();
                let before_full =
                    backing_full_fstatat(parent_fd, &name, "backing component before open")?;
                let before = legacy_backing_binding(before_full, None);
                require_backing_kind(&before, directory, "disk-image backing component")?;
                let mut flags =
                    libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK;
                if directory {
                    flags |= libc::O_DIRECTORY;
                }
                let fd = unsafe { libc::openat(parent_fd, name.as_ptr(), flags) };
                if fd < 0 {
                    return Err(std::io::Error::last_os_error().into());
                }
                let file = unsafe { File::from_raw_fd(fd) };
                let fd_binding_full = backing_full_fstat(file.as_raw_fd(), "backing component fd")?;
                let after_full =
                    backing_full_fstatat(parent_fd, &name, "backing component after open")?;
                if before_full != fd_binding_full || before_full != after_full {
                    return Err(invalid(
                        "disk-image backing component changed across openat",
                    ));
                }
                let stable_binding = if directory {
                    before
                } else {
                    let content_sha256 = held_fd_content_sha256(
                        file.as_raw_fd(),
                        before.size,
                        "disk-image backing file",
                    )?;
                    let fd_after_digest =
                        backing_full_fstat(file.as_raw_fd(), "backing file fd after digest")?;
                    let path_after_digest = backing_full_fstatat(
                        parent_fd,
                        &name,
                        "backing file pathname after digest",
                    )?;
                    if before_full != fd_after_digest || before_full != path_after_digest {
                        return Err(invalid(
                            "disk-image backing file changed while hashing the held descriptor",
                        ));
                    }
                    with_content_sha256(before, Some(&content_sha256))
                };
                current_path.push(OsStr::from_bytes(name.to_bytes()));
                component_paths.push(
                    current_path
                        .to_str()
                        .ok_or_else(|| invalid("disk-image component path is not UTF-8"))?
                        .to_string(),
                );
                component_names.push(name);
                path_bindings_before.push(stable_binding);
                path_bindings_full.push(before_full);
                files.push(file);
            }
            if component_paths.last().map(String::as_str) != Some(path_text)
                || path_bindings_before
                    .last()
                    .is_none_or(|binding| binding.nlink != 1)
            {
                return Err(invalid(
                    "disk-image backing descriptor chain does not end at a single-link regular file",
                ));
            }
            let terminal_initial = unlinked_file_fstat(
                files.last().expect("captured backing terminal").as_raw_fd(),
                "disk-image backing initial terminal",
            )?;
            if terminal_initial.nlink != 1
                || path_bindings_full.last().copied()
                    != Some(unlinked_state_as_full_binding(&terminal_initial))
            {
                return Err(invalid(
                    "disk-image backing initial terminal changed before capture completed",
                ));
            }
            Ok(Self {
                canonical_path: path_text.to_string(),
                component_names,
                component_paths,
                files,
                path_bindings_before,
                path_bindings_full,
                terminal_initial,
                _not_send_or_sync: PhantomData,
            })
        }

        pub fn identity(&self) -> Result<DiskImageBackingIdentityV2, AcceptanceError> {
            self.replay_identity()
        }

        pub fn exact_identity_v3(
            &self,
        ) -> Result<ExactDiskImageBackingIdentityV3, AcceptanceError> {
            let legacy = self.replay_identity()?;
            if self.path_bindings_full.len() != self.files.len()
                || self.component_paths.len() != self.files.len()
            {
                return Err(invalid(
                    "held exact backing descriptor roster changed internally",
                ));
            }
            let content_sha256 = legacy
                .opened_components
                .last()
                .and_then(|component| component.fd_binding.content_sha256.clone())
                .ok_or_else(|| invalid("held backing file lacks its exact content digest"))?;
            Ok(ExactDiskImageBackingIdentityV3 {
                authority_granted: false,
                canonical_path: self.canonical_path.clone(),
                content_sha256,
                opened_components: self
                    .path_bindings_full
                    .iter()
                    .zip(&self.component_paths)
                    .enumerate()
                    .map(|(index, (binding, path))| ExactBackingPathComponentV3 {
                        binding: *binding,
                        directory: index + 1 != self.files.len(),
                        path: path.clone(),
                    })
                    .collect(),
                schema: EXACT_BACKING_IDENTITY_SCHEMA.to_string(),
            })
        }

        /// Observe a namespace transition performed by an external actor.
        /// This method never removes a path: it consumes the present capsule
        /// only after the retained parent proves `ENOENT`, synchronizes that
        /// directory, and proves that the retained terminal inode made the
        /// exact one-link to zero-link transition.
        pub fn observe_namespace_unlinked(
            self,
        ) -> Result<HeldUnlinkedDiskImageBackingV3, AcceptanceError> {
            if self.files.len() < 2
                || self.files.len() != self.component_names.len()
                || self.files.len() != self.component_paths.len()
                || self.files.len() != self.path_bindings_full.len()
                || self.files.len() != self.path_bindings_before.len()
            {
                return Err(invalid(
                    "held disk-image backing roster changed before namespace observation",
                ));
            }
            let terminal_index = self.files.len() - 1;
            let parent = &self.files[terminal_index - 1];
            let basename = &self.component_names[terminal_index];
            require_backing_path_absent(
                parent.as_raw_fd(),
                basename,
                "disk-image backing unlink observation",
            )?;
            parent.sync_all()?;

            let opened_ancestors_before = self.path_bindings_full[..terminal_index]
                .iter()
                .zip(&self.component_paths[..terminal_index])
                .map(|(binding, path)| ExactBackingPathComponentV3 {
                    binding: *binding,
                    directory: true,
                    path: path.clone(),
                })
                .collect::<Vec<_>>();
            let opened_ancestors_after = capture_held_ancestor_bindings(
                &self.files[..terminal_index],
                &self.component_names[..terminal_index],
                &self.component_paths[..terminal_index],
                "disk-image backing post-unlink ancestor",
            )?;
            require_backing_path_absent(
                parent.as_raw_fd(),
                basename,
                "disk-image backing post-sync unlink observation",
            )?;

            let terminal = &self.files[terminal_index];
            let post_unlink_file = unlinked_file_fstat(
                terminal.as_raw_fd(),
                "disk-image backing post-unlink terminal",
            )?;
            let content_sha256 = held_fd_content_sha256(
                terminal.as_raw_fd(),
                post_unlink_file.size,
                "disk-image backing post-unlink content",
            )?;
            let terminal_after_digest = unlinked_file_fstat(
                terminal.as_raw_fd(),
                "disk-image backing post-unlink terminal after digest",
            )?;
            let initial_content_sha256 = self
                .path_bindings_before
                .last()
                .and_then(|binding| binding.content_sha256.clone())
                .ok_or_else(|| invalid("held backing lacks its initial content digest"))?;
            if terminal_after_digest != post_unlink_file || content_sha256 != initial_content_sha256
            {
                return Err(invalid(
                    "retained backing terminal metadata or content changed across unlink observation",
                ));
            }
            require_backing_path_absent(
                parent.as_raw_fd(),
                basename,
                "disk-image backing final unlink observation",
            )?;

            let prepared = ExactDiskImageBackingIdentityV3 {
                authority_granted: false,
                canonical_path: self.canonical_path.clone(),
                content_sha256: initial_content_sha256,
                opened_components: self
                    .path_bindings_full
                    .iter()
                    .zip(&self.component_paths)
                    .enumerate()
                    .map(|(index, (binding, path))| ExactBackingPathComponentV3 {
                        binding: *binding,
                        directory: index + 1 != self.files.len(),
                        path: path.clone(),
                    })
                    .collect(),
                schema: EXACT_BACKING_IDENTITY_SCHEMA.to_string(),
            };
            validate_exact_disk_image_backing_identity_v3(&prepared)?;
            let prepared_backing_sha256 = sha256(&canonical_json(&prepared)?);

            let binding = UnlinkedBackingBindingV3 {
                authority_granted: false,
                canonical_path: self.canonical_path.clone(),
                content_sha256,
                initial_file: self.terminal_initial,
                kind: UNLINKED_BACKING_KIND.to_string(),
                opened_ancestors_after,
                opened_ancestors_before,
                post_unlink_file,
                prepared_backing_sha256,
                schema: UNLINKED_BACKING_SCHEMA.to_string(),
            };
            validate_unlinked_backing_binding_v3(&binding)?;
            Ok(HeldUnlinkedDiskImageBackingV3 {
                binding,
                component_names: self.component_names,
                files: self.files,
                _not_send_or_sync: PhantomData,
            })
        }

        pub fn revalidate_identity_after_persistence(
            &self,
            expected: &DiskImageBackingIdentityV2,
        ) -> Result<(), AcceptanceError> {
            validate_disk_image_backing_identity_v2(expected)?;
            if self.replay_identity()? != *expected {
                return Err(invalid(
                    "held disk-image backing identity changed after receipt persistence",
                ));
            }
            Ok(())
        }

        fn replay_identity(&self) -> Result<DiskImageBackingIdentityV2, AcceptanceError> {
            if std::fs::canonicalize(&self.canonical_path)? != Path::new(&self.canonical_path) {
                return Err(invalid(
                    "held disk-image backing path is no longer canonical",
                ));
            }
            let mut opened_components = Vec::with_capacity(self.files.len());
            for index in 0..self.files.len() {
                let directory = index + 1 != self.files.len();
                let fd_metadata_full = backing_full_fstat(
                    self.files[index].as_raw_fd(),
                    "held backing component replay",
                )?;
                let path_metadata_full = if index == 0 {
                    backing_full_fstatat(
                        libc::AT_FDCWD,
                        &self.component_names[index],
                        "held root pathname replay",
                    )?
                } else {
                    backing_full_fstatat(
                        self.files[index - 1].as_raw_fd(),
                        &self.component_names[index],
                        "held backing pathname replay",
                    )?
                };
                let expected_full = self.path_bindings_full[index];
                if expected_full != fd_metadata_full || expected_full != path_metadata_full {
                    return Err(invalid(
                        "held backing full-stat component changed before provenance publication",
                    ));
                }
                let fd_metadata = legacy_backing_binding(fd_metadata_full, None);
                let path_metadata = legacy_backing_binding(path_metadata_full, None);
                let mut expected_metadata = self.path_bindings_before[index].clone();
                expected_metadata.content_sha256 = None;
                if expected_metadata != fd_metadata || expected_metadata != path_metadata {
                    return Err(invalid(
                        "held backing component changed before provenance publication",
                    ));
                }
                let content_sha256 = if directory {
                    None
                } else {
                    let digest = held_fd_content_sha256(
                        self.files[index].as_raw_fd(),
                        fd_metadata.size,
                        "held backing file replay",
                    )?;
                    let fd_after_digest_full = backing_full_fstat(
                        self.files[index].as_raw_fd(),
                        "held backing file after replay digest",
                    )?;
                    let path_after_digest_full = if index == 0 {
                        unreachable!("root component is always a directory")
                    } else {
                        backing_full_fstatat(
                            self.files[index - 1].as_raw_fd(),
                            &self.component_names[index],
                            "held backing pathname after replay digest",
                        )?
                    };
                    if expected_full != fd_after_digest_full
                        || expected_full != path_after_digest_full
                    {
                        return Err(invalid(
                            "held backing file changed while replaying its content digest",
                        ));
                    }
                    Some(digest)
                };
                let fd_binding = with_content_sha256(fd_metadata, content_sha256.as_deref());
                let path_binding_after =
                    with_content_sha256(path_metadata, content_sha256.as_deref());
                if self.path_bindings_before[index] != fd_binding
                    || self.path_bindings_before[index] != path_binding_after
                {
                    return Err(invalid(
                        "held backing component metadata or content changed before provenance publication",
                    ));
                }
                opened_components.push(BackingPathComponentV1 {
                    directory,
                    fd_binding,
                    path: self.component_paths[index].clone(),
                    path_binding_after,
                    path_binding_before: self.path_bindings_before[index].clone(),
                });
            }
            let identity = DiskImageBackingIdentityV2 {
                authority_granted: false,
                canonical_path: self.canonical_path.clone(),
                opened_components,
                path_authority_granted: false,
                schema: BACKING_IDENTITY_SCHEMA.to_string(),
            };
            validate_disk_image_backing_identity_v2(&identity)?;
            Ok(identity)
        }

        fn finish(
            &self,
            candidate: &DiskImageCandidateObservation,
            disk_image_device_count: usize,
            disk_image_url_count: usize,
        ) -> Result<DiskImageBackingProvenanceV1, AcceptanceError> {
            if strict_file_url_path(&candidate.url)? != self.canonical_path {
                return Err(invalid(
                    "DiskImageURL does not resolve to the held canonical backing path",
                ));
            }
            let identity = self.replay_identity()?;
            let report = DiskImageBackingProvenanceV1 {
                authority_granted: false,
                canonical_path: identity.canonical_path,
                disk_image_device: candidate.device.clone(),
                disk_image_device_ancestor_count: disk_image_device_count as u32,
                disk_image_url: candidate.url.clone(),
                disk_image_url_ancestor_count: disk_image_url_count as u32,
                opened_components: identity.opened_components,
                path_authority_granted: false,
                schema: BACKING_SCHEMA.to_string(),
            };
            validate_backing_shape(&report)?;
            Ok(report)
        }
    }

    impl HeldUnlinkedDiskImageBackingV3 {
        pub fn binding(&self) -> &UnlinkedBackingBindingV3 {
            &self.binding
        }

        pub fn revalidate_after_persistence(&self) -> Result<(), AcceptanceError> {
            validate_unlinked_backing_binding_v3(&self.binding)?;
            let ancestor_count = self.binding.opened_ancestors_after.len();
            if self.files.len() != ancestor_count + 1
                || self.component_names.len() != self.files.len()
            {
                return Err(invalid(
                    "held unlinked backing descriptor roster changed after persistence",
                ));
            }
            let parent = &self.files[ancestor_count - 1];
            let basename = &self.component_names[ancestor_count];
            require_backing_path_absent(
                parent.as_raw_fd(),
                basename,
                "held unlinked backing replay",
            )?;
            parent.sync_all()?;
            let paths = self
                .binding
                .opened_ancestors_after
                .iter()
                .map(|component| component.path.clone())
                .collect::<Vec<_>>();
            let observed = capture_held_ancestor_bindings(
                &self.files[..ancestor_count],
                &self.component_names[..ancestor_count],
                &paths,
                "held unlinked backing ancestor replay",
            )?;
            if observed != self.binding.opened_ancestors_after {
                return Err(invalid(
                    "held unlinked backing ancestor changed after persistence",
                ));
            }
            require_backing_path_absent(
                parent.as_raw_fd(),
                basename,
                "held unlinked backing post-sync replay",
            )?;
            let terminal = self.files.last().expect("validated terminal descriptor");
            let terminal_state = unlinked_file_fstat(
                terminal.as_raw_fd(),
                "held unlinked backing terminal replay",
            )?;
            if terminal_state != self.binding.post_unlink_file {
                return Err(invalid(
                    "held unlinked backing terminal changed after persistence",
                ));
            }
            let content_sha256 = held_fd_content_sha256(
                terminal.as_raw_fd(),
                terminal_state.size,
                "held unlinked backing content replay",
            )?;
            if content_sha256 != self.binding.content_sha256
                || unlinked_file_fstat(
                    terminal.as_raw_fd(),
                    "held unlinked backing terminal after replay digest",
                )? != self.binding.post_unlink_file
            {
                return Err(invalid(
                    "held unlinked backing metadata or content changed during replay",
                ));
            }
            require_backing_path_absent(
                parent.as_raw_fd(),
                basename,
                "held unlinked backing final replay",
            )
        }
    }

    impl HeldBackingPathAbsenceV3 {
        /// Recover only the absence of the prepared basename.  The returned
        /// capsule retains `/` through the direct parent and binds the full
        /// exact prepared identity digest, but never opens or reconstructs the
        /// historical terminal inode. Raw DTO recovery is deliberately
        /// test-only: production integration must supply a sealed retained
        /// prepared capability rather than caller-authored serializable data.
        fn recover_from_exact_prepared_inner(
            prepared: &ExactDiskImageBackingIdentityV3,
        ) -> Result<Self, AcceptanceError> {
            validate_exact_disk_image_backing_identity_v3(prepared)?;
            let prepared_backing_sha256 = sha256(&canonical_json(prepared)?);
            let terminal_index = prepared.opened_components.len() - 1;
            let prepared_ancestors = prepared.opened_components[..terminal_index].to_vec();
            let basename_text = Path::new(&prepared.canonical_path)
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| invalid("prepared backing basename is not UTF-8"))?;
            let basename = CString::new(basename_text.as_bytes())
                .map_err(|_| invalid("prepared backing basename contains NUL"))?;

            let mut files = Vec::with_capacity(prepared_ancestors.len());
            let mut component_names = Vec::with_capacity(prepared_ancestors.len());
            let mut opened_ancestors = Vec::with_capacity(prepared_ancestors.len());
            for (index, expected) in prepared_ancestors.iter().enumerate() {
                let name = if index == 0 {
                    CString::new("/").expect("fixed root path")
                } else {
                    let component = Path::new(&expected.path)
                        .file_name()
                        .ok_or_else(|| invalid("prepared ancestor lacks a basename"))?;
                    CString::new(component.as_bytes())
                        .map_err(|_| invalid("prepared ancestor basename contains NUL"))?
                };
                let parent_fd = files
                    .last()
                    .map_or(libc::AT_FDCWD, |parent: &File| parent.as_raw_fd());
                let before = backing_full_fstatat(
                    parent_fd,
                    &name,
                    "recovered backing ancestor before open",
                )?;
                let fd = unsafe {
                    libc::openat(
                        parent_fd,
                        name.as_ptr(),
                        libc::O_RDONLY
                            | libc::O_CLOEXEC
                            | libc::O_NOFOLLOW
                            | libc::O_NONBLOCK
                            | libc::O_DIRECTORY,
                    )
                };
                if fd < 0 {
                    return Err(std::io::Error::last_os_error().into());
                }
                let file = unsafe { File::from_raw_fd(fd) };
                let fd_binding =
                    backing_full_fstat(file.as_raw_fd(), "recovered backing ancestor descriptor")?;
                let after = backing_full_fstatat(
                    parent_fd,
                    &name,
                    "recovered backing ancestor after open",
                )?;
                let is_parent = index + 1 == prepared_ancestors.len();
                if before != fd_binding
                    || before != after
                    || (!is_parent && before != expected.binding)
                    || (is_parent
                        && !same_parent_binding_except_namespace_delta(&expected.binding, &before))
                {
                    return Err(invalid(
                        "recovered backing ancestor differs from the exact prepared chain",
                    ));
                }
                opened_ancestors.push(ExactBackingPathComponentV3 {
                    binding: before,
                    directory: true,
                    path: expected.path.clone(),
                });
                component_names.push(name);
                files.push(file);
            }

            let parent = files.last().expect("validated prepared ancestor roster");
            require_backing_path_absent(
                parent.as_raw_fd(),
                &basename,
                "recovered prepared backing basename",
            )?;
            parent.sync_all()?;
            let ancestor_paths = prepared_ancestors
                .iter()
                .map(|component| component.path.clone())
                .collect::<Vec<_>>();
            let observed_ancestors = capture_held_ancestor_bindings(
                &files,
                &component_names,
                &ancestor_paths,
                "recovered backing post-sync ancestor",
            )?;
            if observed_ancestors != opened_ancestors {
                return Err(invalid(
                    "recovered backing ancestor changed across parent synchronization",
                ));
            }
            require_backing_path_absent(
                parent.as_raw_fd(),
                &basename,
                "recovered prepared backing post-sync basename",
            )?;

            let binding = BackingPathAbsenceBindingV3 {
                authority_granted: false,
                basename: basename_text.to_string(),
                canonical_path: prepared.canonical_path.clone(),
                kind: BACKING_PATH_ABSENCE_KIND.to_string(),
                observed_ancestors,
                prepared_ancestors,
                prepared_backing_sha256,
                schema: BACKING_PATH_ABSENCE_SCHEMA.to_string(),
            };
            validate_backing_path_absence_binding_v3(&binding)?;
            Ok(Self {
                basename,
                binding,
                component_names,
                files,
                _not_send_or_sync: PhantomData,
            })
        }

        pub(crate) fn recover_from_exact_prepared_sealed(
            _seal: crate::mac_disposable_reconciliation_collector::PreparedBackingAbsenceRecoverySealV3,
            prepared: &ExactDiskImageBackingIdentityV3,
        ) -> Result<Self, AcceptanceError> {
            Self::recover_from_exact_prepared_inner(prepared)
        }

        #[cfg(test)]
        pub fn recover_from_exact_prepared_for_test(
            prepared: &ExactDiskImageBackingIdentityV3,
        ) -> Result<Self, AcceptanceError> {
            Self::recover_from_exact_prepared_inner(prepared)
        }

        pub fn binding(&self) -> &BackingPathAbsenceBindingV3 {
            &self.binding
        }

        pub fn revalidate_after_persistence(&self) -> Result<(), AcceptanceError> {
            validate_backing_path_absence_binding_v3(&self.binding)?;
            if self.files.len() != self.binding.observed_ancestors.len()
                || self.files.len() != self.component_names.len()
            {
                return Err(invalid(
                    "held recovered backing-absence roster changed after persistence",
                ));
            }
            let parent = self.files.last().expect("validated held parent");
            require_backing_path_absent(
                parent.as_raw_fd(),
                &self.basename,
                "held recovered backing-absence replay",
            )?;
            parent.sync_all()?;
            let paths = self
                .binding
                .observed_ancestors
                .iter()
                .map(|component| component.path.clone())
                .collect::<Vec<_>>();
            let observed = capture_held_ancestor_bindings(
                &self.files,
                &self.component_names,
                &paths,
                "held recovered backing-absence ancestor replay",
            )?;
            if observed != self.binding.observed_ancestors {
                return Err(invalid(
                    "held recovered backing-absence ancestor changed after persistence",
                ));
            }
            require_backing_path_absent(
                parent.as_raw_fd(),
                &self.basename,
                "held recovered backing-absence final replay",
            )
        }
    }

    impl ResolvedIOMediaObject {
        pub fn identity(&self) -> &IOMediaRegistryIdentityV1 {
            &self.report
        }
    }

    fn registry_id(object: IoObject, label: &str) -> Result<u64, AcceptanceError> {
        let mut registry_entry_id = 0_u64;
        let rc = unsafe { IORegistryEntryGetRegistryEntryID(object, &mut registry_entry_id) };
        if rc != KERN_SUCCESS || registry_entry_id == 0 {
            return Err(invalid(format!(
                "{label} registry entry ID lookup failed with IOKit status 0x{rc:x}"
            )));
        }
        Ok(registry_entry_id)
    }

    fn require_iomedia(object: IoObject, label: &str) -> Result<(), AcceptanceError> {
        let class = CString::new("IOMedia").expect("fixed IOKit class");
        if unsafe { IOObjectConformsTo(object, class.as_ptr()) } == 0 {
            return Err(invalid(format!(
                "{label} registry object does not conform to IOMedia"
            )));
        }
        Ok(())
    }

    fn disk_bsd_name(disk: DADiskRef) -> Result<String, AcceptanceError> {
        let pointer = unsafe { DADiskGetBSDName(disk) };
        if pointer.is_null() {
            return Err(invalid("DADiskRef has no BSD name"));
        }
        let name = unsafe { CStr::from_ptr(pointer) }
            .to_str()
            .map_err(|_| invalid("DADiskRef BSD name is not UTF-8"))?
            .to_string();
        Ok(name)
    }

    fn iomedia_properties(
        media: IoRegistryEntry,
    ) -> Result<IOMediaRegistryPropertiesV2, AcceptanceError> {
        Ok(IOMediaRegistryPropertiesV2 {
            content: optional_registry_string(media, "Content")?,
            ejectable: optional_registry_boolean(media, "Ejectable")?,
            leaf: optional_registry_boolean(media, "Leaf")?,
            preferred_block_size: optional_registry_u64(media, "Preferred Block Size")?,
            removable: optional_registry_boolean(media, "Removable")?,
            size: optional_registry_u64(media, "Size")?,
            whole: optional_registry_boolean(media, "Whole")?,
            writable: optional_registry_boolean(media, "Writable")?,
        })
    }

    fn disk_arbitration_properties(
        disk: DADiskRef,
    ) -> Result<DiskArbitrationPropertiesV2, AcceptanceError> {
        let description = CfOwned::new(
            unsafe { DADiskCopyDescription(disk) }.cast(),
            "DADiskCopyDescription",
        )?;
        let dictionary = description.as_dictionary();
        Ok(DiskArbitrationPropertiesV2 {
            block_size: optional_da_u64(
                dictionary,
                unsafe { kDADiskDescriptionMediaBlockSizeKey },
                "DA media block size",
            )?,
            content: optional_da_string(
                dictionary,
                unsafe { kDADiskDescriptionMediaContentKey },
                "DA media content",
            )?,
            ejectable: optional_da_boolean(
                dictionary,
                unsafe { kDADiskDescriptionMediaEjectableKey },
                "DA media ejectable",
            )?,
            internal: optional_da_boolean(
                dictionary,
                unsafe { kDADiskDescriptionDeviceInternalKey },
                "DA device internal",
            )?,
            leaf: optional_da_boolean(
                dictionary,
                unsafe { kDADiskDescriptionMediaLeafKey },
                "DA media leaf",
            )?,
            media_uuid: optional_da_uuid(
                dictionary,
                unsafe { kDADiskDescriptionMediaUUIDKey },
                "DA media UUID",
            )?,
            removable: optional_da_boolean(
                dictionary,
                unsafe { kDADiskDescriptionMediaRemovableKey },
                "DA media removable",
            )?,
            size: optional_da_u64(
                dictionary,
                unsafe { kDADiskDescriptionMediaSizeKey },
                "DA media size",
            )?,
            whole: optional_da_boolean(
                dictionary,
                unsafe { kDADiskDescriptionMediaWholeKey },
                "DA media whole",
            )?,
            writable: optional_da_boolean(
                dictionary,
                unsafe { kDADiskDescriptionMediaWritableKey },
                "DA media writable",
            )?,
        })
    }

    struct CapturedNode {
        ancestry: HeldAncestry,
        report: IOMediaRegistryProvenanceV2,
        _resolved: ResolvedIOMediaObject,
    }

    struct ReplayedCapturedNode {
        ancestry: HeldAncestry,
        report: IOMediaRegistryProvenanceV2,
    }

    fn provenance_from_resolved(
        object: ResolvedIOMediaObject,
    ) -> Result<CapturedNode, AcceptanceError> {
        let ancestry = ancestor_chain(object._matched_media.0)?;
        let whole_bsd_name = disk_bsd_name(object._whole_disk.0)?;
        let whole_registry_entry_id =
            registry_id(object._whole_disk_media.0, "DADiskCopyWholeDisk IOMedia")?;
        let report = IOMediaRegistryProvenanceV2 {
            ancestry: ancestry.report.clone(),
            authority_granted: false,
            bsd_name: object.report.bsd_name.clone(),
            conforms_to_iomedia: true,
            disk_arbitration: disk_arbitration_properties(object._disk.0)?,
            iomedia: iomedia_properties(object._matched_media.0)?,
            registry_entry_id: object.report.registry_entry_id.clone(),
            registry_path: registry_path(object._matched_media.0)?.ok_or_else(|| {
                invalid("resolved IOMedia registry entry has no IOService registry path")
            })?,
            whole_disk: registry_identity(whole_bsd_name, whole_registry_entry_id),
            schema: PROVENANCE_SCHEMA.to_string(),
        };
        Ok(CapturedNode {
            ancestry,
            report,
            _resolved: object,
        })
    }

    fn replay_provenance_from_held(
        captured: &CapturedNode,
        label: &str,
    ) -> Result<ReplayedCapturedNode, AcceptanceError> {
        let object = &captured._resolved;
        captured.ancestry.revalidate_retained(label)?;
        require_iomedia(object._matched_media.0, label)?;
        require_iomedia(object._disk_media.0, label)?;
        require_iomedia(object._whole_disk_media.0, label)?;

        let expected_registry_entry_id =
            parse_registry_entry_id(&captured.report.registry_entry_id)?;
        if registry_id(object._matched_media.0, label)? != expected_registry_entry_id
            || registry_id(object._disk_media.0, label)? != expected_registry_entry_id
            || disk_bsd_name(object._disk.0)? != captured.report.bsd_name
        {
            return Err(invalid(format!(
                "{label} exact held IOMedia or DADisk identity changed"
            )));
        }

        let replayed_disk_media = IoObjectGuard::new(
            unsafe { DADiskCopyIOMedia(object._disk.0) },
            "held DADiskCopyIOMedia replay",
        )?;
        require_iomedia(replayed_disk_media.0, label)?;
        if registry_id(replayed_disk_media.0, label)? != expected_registry_entry_id {
            return Err(invalid(format!(
                "{label} held DADiskCopyIOMedia replay changed registry identity"
            )));
        }

        let held_whole_registry_entry_id = registry_id(object._whole_disk_media.0, label)?;
        let held_whole_bsd_name = disk_bsd_name(object._whole_disk.0)?;
        let replayed_whole_disk = DADiskGuard::new(unsafe { DADiskCopyWholeDisk(object._disk.0) })?;
        let replayed_whole_media = IoObjectGuard::new(
            unsafe { DADiskCopyIOMedia(replayed_whole_disk.0) },
            "held DADiskCopyWholeDisk/DADiskCopyIOMedia replay",
        )?;
        require_iomedia(replayed_whole_media.0, label)?;
        let replayed_whole_registry_entry_id = registry_id(replayed_whole_media.0, label)?;
        let replayed_whole_bsd_name = disk_bsd_name(replayed_whole_disk.0)?;
        if held_whole_registry_entry_id != replayed_whole_registry_entry_id
            || held_whole_bsd_name != replayed_whole_bsd_name
            || captured.report.whole_disk
                != registry_identity(
                    replayed_whole_bsd_name.clone(),
                    replayed_whole_registry_entry_id,
                )
        {
            return Err(invalid(format!(
                "{label} exact held whole-disk replay changed identity"
            )));
        }

        let ancestry = ancestor_chain(object._matched_media.0)?;
        let report = IOMediaRegistryProvenanceV2 {
            ancestry: ancestry.report.clone(),
            authority_granted: false,
            bsd_name: captured.report.bsd_name.clone(),
            conforms_to_iomedia: true,
            disk_arbitration: disk_arbitration_properties(object._disk.0)?,
            iomedia: iomedia_properties(object._matched_media.0)?,
            registry_entry_id: format!("{expected_registry_entry_id:016x}"),
            registry_path: registry_path(object._matched_media.0)?.ok_or_else(|| {
                invalid(format!(
                    "{label} exact held IOMedia has no IOService registry path"
                ))
            })?,
            whole_disk: registry_identity(
                replayed_whole_bsd_name,
                replayed_whole_registry_entry_id,
            ),
            schema: PROVENANCE_SCHEMA.to_string(),
        };
        if report != captured.report {
            return Err(invalid(format!(
                "{label} exact held IOMedia descriptors changed"
            )));
        }
        Ok(ReplayedCapturedNode { ancestry, report })
    }

    fn describe_media(
        session: &Session,
        media: IoObjectGuard,
        expected_registry_entry_id: u64,
        expected_bsd_name: &str,
    ) -> Result<ResolvedIOMediaObject, AcceptanceError> {
        require_iomedia(media.0, "matched")?;
        let matched_registry_entry_id = registry_id(media.0, "matched IOMedia")?;
        if matched_registry_entry_id != expected_registry_entry_id {
            return Err(invalid(
                "IORegistryEntryIDMatching resolved a different registry entry ID",
            ));
        }
        let disk = DADiskGuard::new(unsafe {
            DADiskCreateFromIOMedia(std::ptr::null(), session.0, media.0)
        })?;
        let bsd_name = disk_bsd_name(disk.0)?;
        if bsd_name != expected_bsd_name {
            return Err(invalid(
                "resolved IOMedia registry ID now has a different BSD name",
            ));
        }
        let disk_media =
            IoObjectGuard::new(unsafe { DADiskCopyIOMedia(disk.0) }, "DADiskCopyIOMedia")?;
        require_iomedia(disk_media.0, "DADisk replay")?;
        let replayed_registry_entry_id = registry_id(disk_media.0, "DADisk IOMedia replay")?;
        if replayed_registry_entry_id != expected_registry_entry_id {
            return Err(invalid(
                "DADiskCopyIOMedia replay changed the registry entry ID",
            ));
        }
        let whole_disk = DADiskGuard::new(unsafe { DADiskCopyWholeDisk(disk.0) })?;
        let whole_disk_media = IoObjectGuard::new(
            unsafe { DADiskCopyIOMedia(whole_disk.0) },
            "DADiskCopyWholeDisk/DADiskCopyIOMedia",
        )?;
        require_iomedia(whole_disk_media.0, "DADisk whole-disk replay")?;
        Ok(ResolvedIOMediaObject {
            report: registry_identity(bsd_name, replayed_registry_entry_id),
            _disk_media: disk_media,
            _whole_disk_media: whole_disk_media,
            _whole_disk: whole_disk,
            _disk: disk,
            _matched_media: media,
            _session: session.clone(),
            _not_send_or_sync: PhantomData,
        })
    }

    pub fn resolve(
        registry_entry_id: &str,
        expected_boot_session_uuid: &str,
        expected_bsd_name: &str,
    ) -> Result<ResolvedIOMediaObject, AcceptanceError> {
        let registry_entry_id = parse_registry_entry_id(registry_entry_id)?;
        if !valid_uuid(expected_boot_session_uuid) {
            return Err(invalid("IOMedia resolution input is malformed"));
        }
        if !valid_bsd_name(expected_bsd_name) {
            return Err(invalid("IOMedia resolution expected BSD name is malformed"));
        }
        let current_boot = current_boot_session_uuid_impl()?;
        if current_boot != expected_boot_session_uuid {
            return Err(invalid(
                "IOMedia registry entry ID belongs to a different boot session",
            ));
        }
        let matching = unsafe { IORegistryEntryIDMatching(registry_entry_id) };
        if matching.is_null() {
            return Err(invalid(
                "IORegistryEntryIDMatching could not construct a matching dictionary",
            ));
        }
        let media = IoObjectGuard::new(
            unsafe { IOServiceGetMatchingService(K_IOMAIN_PORT_DEFAULT, matching) },
            "IOServiceGetMatchingService",
        )?;
        let session = Session::create()?;
        describe_media(&session, media, registry_entry_id, expected_bsd_name)
    }

    pub fn enumerate() -> Result<Vec<IOMediaRegistryIdentityV1>, AcceptanceError> {
        let class = CString::new("IOMedia").expect("fixed IOKit class");
        let matching = unsafe { IOServiceMatching(class.as_ptr()) };
        if matching.is_null() {
            return Err(invalid("IOServiceMatching could not match IOMedia"));
        }
        let mut iterator = IO_OBJECT_NULL;
        let rc =
            unsafe { IOServiceGetMatchingServices(K_IOMAIN_PORT_DEFAULT, matching, &mut iterator) };
        if rc != KERN_SUCCESS || iterator == IO_OBJECT_NULL {
            return Err(invalid(format!(
                "IOServiceGetMatchingServices failed with IOKit status 0x{rc:x}"
            )));
        }
        let iterator = IoObjectGuard(iterator);
        let session = Session::create()?;
        let boot = current_boot_session_uuid_impl()?;
        let mut identities = BTreeMap::new();
        let mut registry_entry_ids = BTreeSet::new();
        loop {
            let object = unsafe { IOIteratorNext(iterator.0) };
            if object == IO_OBJECT_NULL {
                break;
            }
            let media = IoObjectGuard(object);
            require_iomedia(media.0, "enumerated")?;
            let registry_entry_id = registry_id(media.0, "enumerated IOMedia")?;
            if registry_entry_ids.len() == MAX_IOMEDIA_OBJECTS {
                return Err(invalid("IOMedia enumeration exceeds the object bound"));
            }
            if !registry_entry_ids.insert(registry_entry_id) {
                return Err(invalid(
                    "IOMedia enumeration returned a duplicate registry entry ID",
                ));
            }
            let disk = DADiskGuard::new(unsafe {
                DADiskCreateFromIOMedia(std::ptr::null(), session.0, media.0)
            })?;
            let bsd_name = disk_bsd_name(disk.0)?;
            // APFS snapshot/synthesized media can use nested names such as
            // disk3s1s1. They are real IOMedia objects but can never be one of
            // this fixture's four exact diskN/diskNsN topology roles.
            if !valid_bsd_name(&bsd_name) {
                continue;
            }
            let disk_media = IoObjectGuard::new(
                unsafe { DADiskCopyIOMedia(disk.0) },
                "enumerated DADiskCopyIOMedia",
            )?;
            require_iomedia(disk_media.0, "enumerated DADisk replay")?;
            if registry_id(disk_media.0, "enumerated DADisk IOMedia replay")? != registry_entry_id {
                return Err(invalid(
                    "enumerated DADiskCopyIOMedia changed the registry entry ID",
                ));
            }
            if identities
                .insert(
                    bsd_name.clone(),
                    registry_identity(bsd_name, registry_entry_id),
                )
                .is_some()
            {
                return Err(invalid("IOMedia enumeration returned a duplicate BSD name"));
            }
        }
        if identities.is_empty() {
            return Err(invalid(
                "IOMedia enumeration returned no objects; capability is unavailable",
            ));
        }
        for identity in identities.values() {
            let replay = resolve(
                identity.registry_entry_id.as_str(),
                // Stored IDs are canonical strings; the resolver parses to
                // u64 only immediately before the IOKit call.
                &boot,
                identity.bsd_name.as_str(),
            )?;
            if replay.identity() != identity {
                return Err(invalid("IOMedia enumeration replay changed identity"));
            }
        }
        Ok(identities.into_values().collect())
    }

    fn enumerate_all_registry_ids() -> Result<Vec<String>, AcceptanceError> {
        let class = CString::new("IOMedia").expect("fixed IOKit class");
        let matching = unsafe { IOServiceMatching(class.as_ptr()) };
        if matching.is_null() {
            return Err(invalid("IOServiceMatching could not match IOMedia"));
        }
        let mut iterator = IO_OBJECT_NULL;
        let rc =
            unsafe { IOServiceGetMatchingServices(K_IOMAIN_PORT_DEFAULT, matching, &mut iterator) };
        if rc != KERN_SUCCESS || iterator == IO_OBJECT_NULL {
            return Err(invalid(format!(
                "IOServiceGetMatchingServices failed with IOKit status 0x{rc:x}"
            )));
        }
        let iterator = IoObjectGuard(iterator);
        let mut ids = BTreeSet::new();
        loop {
            let object = unsafe { IOIteratorNext(iterator.0) };
            if object == IO_OBJECT_NULL {
                break;
            }
            let media = IoObjectGuard(object);
            require_iomedia(media.0, "full inventory")?;
            if ids.len() == MAX_IOMEDIA_OBJECTS {
                return Err(invalid("full IOMedia inventory exceeds the object bound"));
            }
            let id = registry_id(media.0, "full inventory IOMedia")?;
            if !ids.insert(format!("{id:016x}")) {
                return Err(invalid(
                    "full IOMedia inventory returned a duplicate registry entry ID",
                ));
            }
        }
        if ids.is_empty() {
            return Err(invalid("full IOMedia inventory returned no objects"));
        }
        Ok(ids.into_iter().collect())
    }

    struct RestartCandidateObservationV3 {
        device: IORegistryAncestorV1,
        disk_image_url: String,
        disk_image_url_path: String,
    }

    fn restart_candidate_observation(
        ancestry: &HeldAncestry,
    ) -> Result<Option<RestartCandidateObservationV3>, AcceptanceError> {
        match (
            ancestry.disk_image_device_count,
            ancestry.disk_image_candidates.as_slice(),
        ) {
            (0, []) => Ok(None),
            (1, [candidate]) => {
                if candidate.device.class_name != "AppleDiskImageDevice"
                    || parse_registry_entry_id(&candidate.device.registry_entry_id).is_err()
                    || candidate
                        .device
                        .registry_path
                        .as_deref()
                        .is_none_or(|path| !valid_registry_path(path))
                {
                    return Err(invalid(
                        "restart candidate has malformed AppleDiskImageDevice ancestry",
                    ));
                }
                Ok(Some(RestartCandidateObservationV3 {
                    device: candidate.device.clone(),
                    disk_image_url: candidate.url.clone(),
                    disk_image_url_path: strict_file_url_path(&candidate.url)?,
                }))
            }
            _ => Err(invalid(
                "restart IOMedia ancestry has an ambiguous DiskImageURL/device relationship",
            )),
        }
    }

    fn canonical_restart_url_path(path: &str) -> Result<String, AcceptanceError> {
        let canonical = std::fs::canonicalize(Path::new(path))?;
        canonical
            .to_str()
            .filter(|value| !value.is_empty() && value.len() <= MAX_CF_STRING_BYTES)
            .map(str::to_string)
            .ok_or_else(|| invalid("DiskImageURL realpath is not bounded UTF-8"))
    }

    fn capture_restart_url_backing(
        canonical_path: &str,
        disk_image_url: &str,
        disk_image_url_path: &str,
    ) -> Result<HeldRestartCandidateBackingV3, AcceptanceError> {
        let path = CString::new(canonical_path)
            .map_err(|_| invalid("DiskImageURL realpath contains NUL"))?;
        let before = backing_fstatat(libc::AT_FDCWD, &path, "DiskImageURL backing before open")?;
        require_backing_kind(&before, false, "DiskImageURL backing")?;
        let fd = unsafe {
            libc::open(
                path.as_ptr(),
                libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
            )
        };
        if fd < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let file = unsafe { File::from_raw_fd(fd) };
        let held = backing_fstat(file.as_raw_fd(), "held DiskImageURL backing")?;
        let after = backing_fstatat(libc::AT_FDCWD, &path, "DiskImageURL backing after open")?;
        require_backing_kind(&held, false, "DiskImageURL backing")?;
        if before != held || held != after {
            return Err(invalid(
                "DiskImageURL backing changed across descriptor capture",
            ));
        }
        let identity = RestartDiskImageBackingIdentityV3 {
            authority_granted: false,
            canonical_path: canonical_path.to_string(),
            file_binding: held,
            schema: RESTART_BACKING_IDENTITY_SCHEMA.to_string(),
        };
        validate_restart_disk_image_backing_identity_v3(&identity)?;
        Ok(HeldRestartCandidateBackingV3 {
            canonical_path: canonical_path.to_string(),
            disk_image_url: disk_image_url.to_string(),
            disk_image_url_path: disk_image_url_path.to_string(),
            file,
            identity,
        })
    }

    #[cfg(test)]
    pub(super) fn capture_restart_url_backing_identity_for_test(
        disk_image_url: &str,
    ) -> Result<RestartDiskImageBackingIdentityV3, AcceptanceError> {
        let disk_image_url_path = strict_file_url_path(disk_image_url)?;
        let canonical_path = canonical_restart_url_path(&disk_image_url_path)?;
        let held =
            capture_restart_url_backing(&canonical_path, disk_image_url, &disk_image_url_path)?;
        if canonical_restart_url_path(&disk_image_url_path)? != canonical_path
            || backing_fstat(held.file.as_raw_fd(), "test DiskImageURL backing replay")?
                != held.identity.file_binding
        {
            return Err(invalid(
                "test DiskImageURL backing changed across retained replay",
            ));
        }
        Ok(held.identity.clone())
    }

    fn restart_candidate_with_backing(
        observation: RestartCandidateObservationV3,
        backing: &HeldRestartCandidateBackingV3,
    ) -> Result<RestartDiskImageCandidateV3, AcceptanceError> {
        if observation.disk_image_url != backing.disk_image_url
            || observation.disk_image_url_path != backing.disk_image_url_path
            || canonical_restart_url_path(&observation.disk_image_url_path)?
                != backing.canonical_path
        {
            return Err(invalid(
                "DiskImageURL alias changed while its live backing was retained",
            ));
        }
        Ok(RestartDiskImageCandidateV3 {
            backing_identity: backing.identity.clone(),
            canonical_backing_path: backing.canonical_path.clone(),
            disk_image_device: observation.device,
            disk_image_url: observation.disk_image_url,
            disk_image_url_path: observation.disk_image_url_path,
        })
    }

    pub fn capture_restart_inventory() -> Result<HeldRestartIOMediaInventoryV3, AcceptanceError> {
        let boot = current_boot_session_uuid_impl()?;
        let class = CString::new("IOMedia").expect("fixed IOKit class");
        let matching = unsafe { IOServiceMatching(class.as_ptr()) };
        if matching.is_null() {
            return Err(invalid("IOServiceMatching could not match IOMedia"));
        }
        let mut iterator = IO_OBJECT_NULL;
        let rc =
            unsafe { IOServiceGetMatchingServices(K_IOMAIN_PORT_DEFAULT, matching, &mut iterator) };
        if rc != KERN_SUCCESS || iterator == IO_OBJECT_NULL {
            return Err(invalid(format!(
                "IOServiceGetMatchingServices failed with IOKit status 0x{rc:x}"
            )));
        }
        let iterator = IoObjectGuard(iterator);
        let session = Session::create()?;
        let mut nodes = BTreeMap::<String, CapturedNode>::new();
        loop {
            let object = unsafe { IOIteratorNext(iterator.0) };
            if object == IO_OBJECT_NULL {
                break;
            }
            if nodes.len() == MAX_IOMEDIA_OBJECTS {
                return Err(invalid(
                    "restart IOMedia enumeration exceeds the object bound",
                ));
            }
            let media = IoObjectGuard(object);
            require_iomedia(media.0, "restart census")?;
            let id = registry_id(media.0, "restart census IOMedia")?;
            let disk = DADiskGuard::new(unsafe {
                DADiskCreateFromIOMedia(std::ptr::null(), session.0, media.0)
            })?;
            let bsd_name = disk_bsd_name(disk.0)?;
            let resolved = describe_media(&session, media, id, &bsd_name)?;
            let captured = provenance_from_resolved(resolved)?;
            let key = format!("{id:016x}");
            if nodes.insert(key, captured).is_some() {
                return Err(invalid(
                    "restart IOMedia enumeration returned a duplicate registry ID",
                ));
            }
        }
        if nodes.is_empty() {
            return Err(invalid("restart IOMedia enumeration returned no objects"));
        }
        let all_ids = enumerate_all_registry_ids()?;
        if nodes.keys().cloned().collect::<Vec<_>>() != all_ids
            || current_boot_session_uuid_impl()? != boot
        {
            return Err(invalid(
                "restart full IOMedia census or boot changed during capture",
            ));
        }
        let mut backing_by_url = BTreeMap::<String, HeldRestartCandidateBackingV3>::new();
        let mut held_nodes = Vec::with_capacity(nodes.len());
        for captured in nodes.into_values() {
            let candidate = match restart_candidate_observation(&captured.ancestry)? {
                None => None,
                Some(observation) => {
                    let canonical_path =
                        canonical_restart_url_path(&observation.disk_image_url_path)?;
                    if !backing_by_url.contains_key(&observation.disk_image_url) {
                        backing_by_url.insert(
                            observation.disk_image_url.clone(),
                            capture_restart_url_backing(
                                &canonical_path,
                                &observation.disk_image_url,
                                &observation.disk_image_url_path,
                            )?,
                        );
                    }
                    let backing = backing_by_url
                        .get(&observation.disk_image_url)
                        .expect("restart backing inserted");
                    if backing.canonical_path != canonical_path {
                        return Err(invalid(
                            "one DiskImageURL resolved to multiple live backing paths",
                        ));
                    }
                    Some(restart_candidate_with_backing(observation, backing)?)
                }
            };
            held_nodes.push(HeldRestartNodeV3 {
                candidate,
                captured,
            });
        }
        let objects = held_nodes
            .iter()
            .map(|node| RestartIOMediaObjectV3 {
                authority_granted: false,
                candidate: node.candidate.clone(),
                provenance: node.captured.report.clone(),
            })
            .collect::<Vec<_>>();
        let report = RestartIOMediaInventoryV3 {
            authority_granted: false,
            boot_session_uuid: boot,
            objects,
            schema: RESTART_INVENTORY_SCHEMA.to_string(),
        };
        validate_restart_iomedia_inventory_v3(&report)?;
        Ok(HeldRestartIOMediaInventoryV3 {
            held_backings: backing_by_url.into_values().collect(),
            held_nodes,
            report,
            _not_send_or_sync: PhantomData,
        })
    }

    impl HeldRestartIOMediaInventoryV3 {
        pub fn report(&self) -> &RestartIOMediaInventoryV3 {
            &self.report
        }

        #[cfg(test)]
        pub(crate) fn poison_boot_session_for_test(&mut self) {
            self.report.boot_session_uuid = "00000000-0000-0000-0000-000000000000".to_string();
        }

        #[cfg(test)]
        pub(crate) fn substitute_valid_property_for_test(&mut self) {
            let object = self
                .report
                .objects
                .first_mut()
                .expect("captured restart inventory is nonempty");
            object.provenance.disk_arbitration.internal =
                Some(!object.provenance.disk_arbitration.internal.unwrap_or(false));
        }

        pub fn revalidate_after_persistence(&self) -> Result<(), AcceptanceError> {
            if current_boot_session_uuid_impl()? != self.report.boot_session_uuid
                || self.held_nodes.len() != self.report.objects.len()
            {
                return Err(invalid(
                    "restart IOMedia census changed boot or descriptor cardinality",
                ));
            }
            for backing in &self.held_backings {
                if canonical_restart_url_path(&backing.disk_image_url_path)?
                    != backing.canonical_path
                {
                    return Err(invalid(
                        "DiskImageURL realpath changed after receipt persistence",
                    ));
                }
                let path = CString::new(backing.canonical_path.as_str())
                    .map_err(|_| invalid("DiskImageURL realpath contains NUL"))?;
                let held =
                    backing_fstat(backing.file.as_raw_fd(), "held DiskImageURL backing replay")?;
                let named =
                    backing_fstatat(libc::AT_FDCWD, &path, "named DiskImageURL backing replay")?;
                if held != backing.identity.file_binding || named != backing.identity.file_binding {
                    return Err(invalid(
                        "held DiskImageURL backing changed after receipt persistence",
                    ));
                }
                validate_restart_disk_image_backing_identity_v3(&backing.identity)?;
            }
            for (index, (held, expected)) in
                self.held_nodes.iter().zip(&self.report.objects).enumerate()
            {
                held.captured
                    .ancestry
                    .revalidate_retained(&format!("restart held node {index}"))?;
                let replayed = replay_provenance_from_held(
                    &held.captured,
                    &format!("restart held node {index}"),
                )?;
                let replayed_candidate = match restart_candidate_observation(&replayed.ancestry)? {
                    None => None,
                    Some(observation) => {
                        let backing = self
                            .held_backings
                            .iter()
                            .find(|backing| backing.disk_image_url == observation.disk_image_url)
                            .ok_or_else(|| {
                                invalid("replayed DiskImageURL has no retained backing")
                            })?;
                        Some(restart_candidate_with_backing(observation, backing)?)
                    }
                };
                if replayed.report != expected.provenance
                    || replayed_candidate != expected.candidate
                    || held.candidate != expected.candidate
                {
                    return Err(invalid(
                        "restart held IOMedia provenance changed after receipt persistence",
                    ));
                }
            }
            let expected_ids = self
                .report
                .objects
                .iter()
                .map(|object| object.provenance.registry_entry_id.clone())
                .collect::<Vec<_>>();
            if enumerate_all_registry_ids()? != expected_ids
                || current_boot_session_uuid_impl()? != self.report.boot_session_uuid
            {
                return Err(invalid(
                    "restart full IOMedia inventory changed after receipt persistence",
                ));
            }
            validate_restart_iomedia_inventory_v3(&self.report)
        }
    }

    struct CapturedFour {
        held_nodes: Vec<CapturedNode>,
        topology: IOMediaFourNodeTopologyV2,
    }

    struct ReplayedFour {
        held_ancestries: Vec<HeldAncestry>,
        topology: IOMediaFourNodeTopologyV2,
    }

    impl CapturedFour {
        fn replay_exact(&self, label: &str) -> Result<ReplayedFour, AcceptanceError> {
            if self.held_nodes.len() != 4 {
                return Err(invalid(format!(
                    "{label} held topology does not contain exactly four nodes"
                )));
            }
            let replayed = self
                .held_nodes
                .iter()
                .enumerate()
                .map(|(index, node)| {
                    replay_provenance_from_held(node, &format!("{label} node {index}"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let topology = IOMediaFourNodeTopologyV2 {
                apfs_container: replayed[2].report.clone(),
                apfs_volume: replayed[3].report.clone(),
                authority_granted: false,
                boot_session_uuid: self.topology.boot_session_uuid.clone(),
                physical_store: replayed[1].report.clone(),
                physical_whole: replayed[0].report.clone(),
                schema: FOUR_NODE_TOPOLOGY_SCHEMA.to_string(),
            };
            let expected = ExpectedIOMediaTopology {
                apfs_container: &self.topology.apfs_container.bsd_name,
                apfs_volume: &self.topology.apfs_volume.bsd_name,
                physical_store: &self.topology.physical_store.bsd_name,
                physical_whole: &self.topology.physical_whole.bsd_name,
            };
            validate_four_node_topology_shape(&topology, expected, label)?;
            if topology != self.topology {
                return Err(invalid(format!(
                    "{label} exact held four-node topology changed"
                )));
            }
            Ok(ReplayedFour {
                held_ancestries: replayed.into_iter().map(|node| node.ancestry).collect(),
                topology,
            })
        }
    }

    fn capture_four(
        expected: ExpectedIOMediaTopology<'_>,
        boot_session_uuid: &str,
        label: &str,
    ) -> Result<CapturedFour, AcceptanceError> {
        let identities = enumerate()?
            .into_iter()
            .map(|identity| (identity.bsd_name.clone(), identity))
            .collect::<BTreeMap<_, _>>();
        let capture_node = |role: &str, bsd_name: &str| -> Result<CapturedNode, AcceptanceError> {
            let identity = identities.get(bsd_name).ok_or_else(|| {
                invalid(format!(
                    "fresh IOMedia graph has no exact {label} {role} node {bsd_name}"
                ))
            })?;
            let resolved = resolve(
                &identity.registry_entry_id,
                boot_session_uuid,
                &identity.bsd_name,
            )?;
            provenance_from_resolved(resolved)
        };
        let physical_whole = capture_node("physical whole", expected.physical_whole)?;
        let physical_store = capture_node("physical store", expected.physical_store)?;
        let apfs_container = capture_node("APFS container", expected.apfs_container)?;
        let apfs_volume = capture_node("APFS volume", expected.apfs_volume)?;
        let topology = IOMediaFourNodeTopologyV2 {
            apfs_container: apfs_container.report.clone(),
            apfs_volume: apfs_volume.report.clone(),
            authority_granted: false,
            boot_session_uuid: boot_session_uuid.to_string(),
            physical_store: physical_store.report.clone(),
            physical_whole: physical_whole.report.clone(),
            schema: FOUR_NODE_TOPOLOGY_SCHEMA.to_string(),
        };
        validate_four_node_topology_shape(&topology, expected, label)?;
        Ok(CapturedFour {
            held_nodes: vec![physical_whole, physical_store, apfs_container, apfs_volume],
            topology,
        })
    }

    fn unique_disk_image_candidate_from_ancestries<'a>(
        ancestries: impl IntoIterator<Item = &'a HeldAncestry>,
    ) -> Result<DiskImageCandidateObservation, AcceptanceError> {
        let observations = ancestries
            .into_iter()
            .map(|ancestry| {
                (
                    ancestry.disk_image_device_count,
                    ancestry.disk_image_candidates.clone(),
                )
            })
            .collect::<Vec<_>>();
        select_unique_disk_image_candidate(&observations)
    }

    fn unique_disk_image_candidate(
        captured: &CapturedFour,
    ) -> Result<DiskImageCandidateObservation, AcceptanceError> {
        unique_disk_image_candidate_from_ancestries(
            captured.held_nodes.iter().map(|node| &node.ancestry),
        )
    }

    fn unique_replayed_disk_image_candidate(
        replayed: &ReplayedFour,
    ) -> Result<DiskImageCandidateObservation, AcceptanceError> {
        unique_disk_image_candidate_from_ancestries(replayed.held_ancestries.iter())
    }

    fn require_no_disk_image_ancestry(
        captured: &CapturedFour,
        label: &str,
    ) -> Result<(), AcceptanceError> {
        if captured.held_nodes.len() != 4
            || captured.held_nodes.iter().any(|node| {
                node.ancestry.disk_image_device_count != 0
                    || !node.ancestry.disk_image_candidates.is_empty()
            })
        {
            return Err(invalid(format!(
                "{label} unexpectedly descends from a disk-image device"
            )));
        }
        Ok(())
    }

    fn require_no_replayed_disk_image_ancestry(
        replayed: &ReplayedFour,
        label: &str,
    ) -> Result<(), AcceptanceError> {
        if replayed.held_ancestries.len() != 4
            || replayed.held_ancestries.iter().any(|ancestry| {
                ancestry.disk_image_device_count != 0 || !ancestry.disk_image_candidates.is_empty()
            })
        {
            return Err(invalid(format!(
                "{label} unexpectedly descends from a disk-image device"
            )));
        }
        Ok(())
    }

    pub fn capture_inventory(
        expected_t5: ExpectedIOMediaTopology<'_>,
    ) -> Result<HeldPreAttachIOMediaInventory, AcceptanceError> {
        let boot = current_boot_session_uuid_impl()?;
        let all_before = enumerate_all_registry_ids()?;
        let t5_capture = capture_four(expected_t5, &boot, "T5")?;
        require_no_disk_image_ancestry(&t5_capture, "T5")?;
        let all_after = enumerate_all_registry_ids()?;
        if all_before != all_after || current_boot_session_uuid_impl()? != boot {
            return Err(invalid(
                "pre-attach IOMedia inventory or boot changed during capture",
            ));
        }
        let t5_replay = capture_four(expected_t5, &boot, "T5 replay")?;
        require_no_disk_image_ancestry(&t5_replay, "T5 replay")?;
        if t5_replay.topology != t5_capture.topology
            || enumerate_all_registry_ids()? != all_before
            || current_boot_session_uuid_impl()? != boot
        {
            return Err(invalid(
                "pre-attach T5 descriptors or full registry inventory changed on exact replay",
            ));
        }
        let t5_volume_uuid = t5_capture
            .topology
            .apfs_volume
            .disk_arbitration
            .media_uuid
            .clone()
            .ok_or_else(|| invalid("T5 APFS volume has no Disk Arbitration UUID"))?;
        let inventory = IOMediaRegistryInventoryV2 {
            all_registry_entry_ids: all_before,
            authority_granted: false,
            boot_session_uuid: boot,
            capture_monotonic_nanoseconds: monotonic_nanoseconds()?,
            schema: REGISTRY_INVENTORY_SCHEMA.to_string(),
            t5_apfs_container: t5_capture.topology.apfs_container.clone(),
            t5_apfs_volume: t5_capture.topology.apfs_volume.clone(),
            t5_physical_store: t5_capture.topology.physical_store.clone(),
            t5_physical_whole: t5_capture.topology.physical_whole.clone(),
            t5_volume_uuid,
        };
        validate_iomedia_registry_inventory_shape(&inventory, expected_t5)?;
        Ok(HeldPreAttachIOMediaInventory {
            report: inventory,
            t5_capture,
            t5_replay,
            _not_send_or_sync: PhantomData,
        })
    }

    pub fn capture_v2<'a>(
        expected: ExpectedIOMediaTopology<'_>,
        pre_attach: &'a HeldPreAttachIOMediaInventory,
        held_backing: HeldDiskImageBacking,
    ) -> Result<HeldAttachedIOMediaTopologyV2<'a>, AcceptanceError> {
        let pre_attach_inventory = &pre_attach.report;
        let boot = current_boot_session_uuid_impl()?;
        if boot != pre_attach_inventory.boot_session_uuid {
            return Err(invalid(
                "pre-attach IOMedia inventory belongs to another boot session",
            ));
        }
        let t5_expected = ExpectedIOMediaTopology {
            apfs_container: &pre_attach_inventory.t5_apfs_container.bsd_name,
            apfs_volume: &pre_attach_inventory.t5_apfs_volume.bsd_name,
            physical_store: &pre_attach_inventory.t5_physical_store.bsd_name,
            physical_whole: &pre_attach_inventory.t5_physical_whole.bsd_name,
        };
        validate_iomedia_registry_inventory_shape(pre_attach_inventory, t5_expected)?;
        let fresh_t5_capture = capture_four(t5_expected, &boot, "fresh T5")?;
        require_no_disk_image_ancestry(&fresh_t5_capture, "fresh T5")?;
        if fresh_t5_capture.topology.physical_whole != pre_attach_inventory.t5_physical_whole
            || fresh_t5_capture.topology.physical_store != pre_attach_inventory.t5_physical_store
            || fresh_t5_capture.topology.apfs_container != pre_attach_inventory.t5_apfs_container
            || fresh_t5_capture.topology.apfs_volume != pre_attach_inventory.t5_apfs_volume
        {
            return Err(invalid(
                "fresh T5 capture differs from the pre-attach registry inventory",
            ));
        }
        let attached = capture_four(expected, &boot, "attached disk image")?;
        let candidate = unique_disk_image_candidate(&attached)?;
        let backing = held_backing.finish(&candidate, 1, 1)?;
        let current_ids = enumerate_all_registry_ids()?;
        let mut expected_current_ids = pre_attach_inventory
            .all_registry_entry_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        for (_, node) in attached.topology.ordered() {
            if !expected_current_ids.insert(node.registry_entry_id.clone()) {
                return Err(invalid(
                    "attached IOMedia registry ID aliases the immediate pre-attach inventory",
                ));
            }
        }
        if current_ids != expected_current_ids.into_iter().collect::<Vec<_>>() {
            return Err(invalid(
                "current IOMedia IDs are not exactly pre-attach IDs plus the four attached nodes",
            ));
        }
        let topology = AttachedIOMediaTopologyV2 {
            apfs_container: attached.topology.apfs_container.clone(),
            apfs_volume: attached.topology.apfs_volume.clone(),
            authority_granted: false,
            backing,
            boot_session_uuid: boot.clone(),
            fresh_t5: fresh_t5_capture.topology.clone(),
            physical_store: attached.topology.physical_store.clone(),
            physical_whole: attached.topology.physical_whole.clone(),
            pre_attach_inventory: pre_attach_inventory.clone(),
            schema: PROVENANCE_TOPOLOGY_SCHEMA.to_string(),
        };
        validate_iomedia_topology_provenance_shape(&topology, expected)?;
        let attached_replay = capture_four(expected, &boot, "attached exact replay")?;
        let t5_replay = capture_four(t5_expected, &boot, "fresh T5 exact replay")?;
        require_no_disk_image_ancestry(&t5_replay, "fresh T5 exact replay")?;
        let replay_candidate = unique_disk_image_candidate(&attached_replay)?;
        if attached_replay.topology != attached.topology
            || t5_replay.topology != fresh_t5_capture.topology
            || replay_candidate.device != candidate.device
            || replay_candidate.url != candidate.url
        {
            return Err(invalid(
                "attached or T5 held IOMedia topology changed on final descriptor replay",
            ));
        }
        let mut final_expected_ids = pre_attach_inventory
            .all_registry_entry_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        for (_, node) in topology.ordered() {
            final_expected_ids.insert(node.registry_entry_id.clone());
        }
        if enumerate_all_registry_ids()? != final_expected_ids.into_iter().collect::<Vec<_>>() {
            return Err(invalid(
                "full IOMedia registry inventory changed during final replay",
            ));
        }
        let backing_replay = held_backing.finish(&replay_candidate, 1, 1)?;
        if backing_replay != topology.backing {
            return Err(invalid(
                "disk-image URL or held backing descriptor changed on final replay",
            ));
        }
        validate_iomedia_topology_provenance_shape(&topology, expected)?;
        if current_boot_session_uuid_impl()? != boot {
            return Err(invalid("boot changed during attached IOMedia capture"));
        }
        Ok(HeldAttachedIOMediaTopologyV2 {
            report: topology,
            pre_attach,
            held_backing,
            fresh_t5_capture,
            attached,
            attached_replay,
            t5_replay,
            _not_send_or_sync: PhantomData,
        })
    }

    impl HeldPreAttachIOMediaInventory {
        pub fn report(&self) -> &IOMediaRegistryInventoryV2 {
            &self.report
        }

        fn revalidate_exact(&self) -> Result<(), AcceptanceError> {
            if current_boot_session_uuid_impl()? != self.report.boot_session_uuid {
                return Err(invalid(
                    "pre-attach held IOMedia capsule belongs to another boot session",
                ));
            }
            let expected = ExpectedIOMediaTopology {
                apfs_container: &self.report.t5_apfs_container.bsd_name,
                apfs_volume: &self.report.t5_apfs_volume.bsd_name,
                physical_store: &self.report.t5_physical_store.bsd_name,
                physical_whole: &self.report.t5_physical_whole.bsd_name,
            };
            validate_iomedia_registry_inventory_shape(&self.report, expected)?;
            let first = self
                .t5_capture
                .replay_exact("pre-attach held T5 final replay")?;
            let second = self
                .t5_replay
                .replay_exact("pre-attach held T5 duplicate final replay")?;
            require_no_replayed_disk_image_ancestry(&first, "pre-attach held T5 final replay")?;
            require_no_replayed_disk_image_ancestry(
                &second,
                "pre-attach held T5 duplicate final replay",
            )?;
            if first.topology != second.topology
                || first.topology.physical_whole != self.report.t5_physical_whole
                || first.topology.physical_store != self.report.t5_physical_store
                || first.topology.apfs_container != self.report.t5_apfs_container
                || first.topology.apfs_volume != self.report.t5_apfs_volume
            {
                return Err(invalid(
                    "pre-attach held T5 descriptors changed before receipt publication",
                ));
            }
            Ok(())
        }
    }

    impl<'a> HeldAttachedIOMediaTopologyV2<'a> {
        pub fn report(&self) -> &AttachedIOMediaTopologyV2 {
            &self.report
        }

        /// Replays every retained descriptor after the caller has generated
        /// and durably appended canonical receipt bytes. This grants no
        /// effect authority and exposes no effect primitive.
        pub fn revalidate_after_persistence(&self) -> Result<(), AcceptanceError> {
            let boot = current_boot_session_uuid_impl()?;
            if boot != self.report.boot_session_uuid
                || self.report.pre_attach_inventory != *self.pre_attach.report()
            {
                return Err(invalid(
                    "held attached IOMedia capsule changed boot or pre-attach report",
                ));
            }
            self.pre_attach.revalidate_exact()?;

            let fresh_t5 = self
                .fresh_t5_capture
                .replay_exact("fresh T5 post-persistence replay")?;
            let t5_replay = self
                .t5_replay
                .replay_exact("fresh T5 duplicate post-persistence replay")?;
            require_no_replayed_disk_image_ancestry(&fresh_t5, "fresh T5 post-persistence replay")?;
            require_no_replayed_disk_image_ancestry(
                &t5_replay,
                "fresh T5 duplicate post-persistence replay",
            )?;
            if fresh_t5.topology != self.report.fresh_t5
                || t5_replay.topology != self.report.fresh_t5
            {
                return Err(invalid(
                    "held fresh T5 descriptors changed after canonical receipt generation",
                ));
            }

            let attached = self
                .attached
                .replay_exact("attached post-persistence replay")?;
            let attached_replay = self
                .attached_replay
                .replay_exact("attached duplicate post-persistence replay")?;
            let reported_attached = IOMediaFourNodeTopologyV2 {
                apfs_container: self.report.apfs_container.clone(),
                apfs_volume: self.report.apfs_volume.clone(),
                authority_granted: false,
                boot_session_uuid: self.report.boot_session_uuid.clone(),
                physical_store: self.report.physical_store.clone(),
                physical_whole: self.report.physical_whole.clone(),
                schema: FOUR_NODE_TOPOLOGY_SCHEMA.to_string(),
            };
            if attached.topology != reported_attached
                || attached_replay.topology != reported_attached
            {
                return Err(invalid(
                    "held attached descriptors changed after canonical receipt generation",
                ));
            }
            let candidate = unique_replayed_disk_image_candidate(&attached)?;
            let replay_candidate = unique_replayed_disk_image_candidate(&attached_replay)?;
            if candidate != replay_candidate
                || candidate.device != self.report.backing.disk_image_device
                || candidate.url != self.report.backing.disk_image_url
            {
                return Err(invalid(
                    "held DiskImageURL ancestry changed after canonical receipt generation",
                ));
            }
            let backing = self.held_backing.finish(&candidate, 1, 1)?;
            if backing != self.report.backing {
                return Err(invalid(
                    "held disk-image backing metadata or content changed after canonical receipt generation",
                ));
            }

            let mut expected_current_ids = self
                .pre_attach
                .report()
                .all_registry_entry_ids
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>();
            for (_, node) in self.report.ordered() {
                if !expected_current_ids.insert(node.registry_entry_id.clone()) {
                    return Err(invalid(
                        "attached registry identity aliases the held pre-attach inventory",
                    ));
                }
            }
            if enumerate_all_registry_ids()? != expected_current_ids.into_iter().collect::<Vec<_>>()
            {
                return Err(invalid(
                    "full IOMedia registry inventory changed before receipt publication",
                ));
            }
            let expected = ExpectedIOMediaTopology {
                apfs_container: &self.report.apfs_container.bsd_name,
                apfs_volume: &self.report.apfs_volume.bsd_name,
                physical_store: &self.report.physical_store.bsd_name,
                physical_whole: &self.report.physical_whole.bsd_name,
            };
            validate_iomedia_topology_provenance_shape(&self.report, expected)?;
            if current_boot_session_uuid_impl()? != boot {
                return Err(invalid(
                    "boot changed during post-persistence held-descriptor replay",
                ));
            }
            Ok(())
        }
    }

    pub fn capture(
        expected: ExpectedIOMediaTopology<'_>,
    ) -> Result<AttachedIOMediaTopologyV1, AcceptanceError> {
        let boot = current_boot_session_uuid_impl()?;
        let identities = enumerate()?
            .into_iter()
            .map(|identity| (identity.bsd_name.clone(), identity))
            .collect::<BTreeMap<_, _>>();
        let find = |label: &str, bsd_name: &str| {
            identities.get(bsd_name).cloned().ok_or_else(|| {
                invalid(format!(
                    "fresh IOMedia graph has no exact {label} node {bsd_name}"
                ))
            })
        };
        let topology = AttachedIOMediaTopologyV1 {
            apfs_container: find("APFS container", expected.apfs_container)?,
            apfs_volume: find("APFS volume", expected.apfs_volume)?,
            authority_granted: false,
            boot_session_uuid: boot.clone(),
            physical_store: find("physical store", expected.physical_store)?,
            physical_whole: find("physical whole", expected.physical_whole)?,
            schema: TOPOLOGY_SCHEMA.to_string(),
        };
        validate_iomedia_topology_identity_current_boot(&topology, expected)?;
        let mut retained = Vec::new();
        for (_, node) in topology.ordered() {
            retained.push(resolve(
                node.registry_entry_id.as_str(),
                &boot,
                node.bsd_name.as_str(),
            )?);
        }
        if retained
            .iter()
            .any(|object| object.identity().authority_granted)
        {
            return Err(invalid(
                "read-only IOMedia capture unexpectedly granted authority",
            ));
        }
        // These handles intentionally live only through the capture
        // consistency check. This schema grants no effect authority. A future
        // isolated effect runner must fresh-resolve all four IDs and retain
        // every handle through its callback and terminal observation.
        Ok(topology)
    }

    #[cfg(test)]
    mod live_tests {
        use std::fs::OpenOptions;
        use std::io::Seek;
        use std::io::SeekFrom;
        use std::io::Write;
        use std::os::unix::fs::MetadataExt;
        use std::os::unix::fs::PermissionsExt;
        use std::os::unix::fs::symlink;

        use super::*;

        const UNLINKED_FIXTURE_BYTES: &[u8] = b"held-backing-v3";

        fn make_present_backing_fixture() -> (tempfile::TempDir, PathBuf, File, HeldDiskImageBacking)
        {
            let directory = tempfile::tempdir().expect("create unlinked backing directory");
            let parent = directory.path().join("outer").join("prepared");
            std::fs::create_dir_all(&parent).expect("create prepared backing ancestor chain");
            let path = parent.join("image.dmg");
            std::fs::write(&path, UNLINKED_FIXTURE_BYTES).expect("write prepared backing fixture");
            let path = path
                .canonicalize()
                .expect("canonical prepared backing path");
            let external = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&path)
                .expect("open external mutation handle");
            let held = HeldDiskImageBacking::capture(&path)
                .expect("capture exact prepared backing descriptors");
            (directory, path, external, held)
        }

        fn make_unlinked_backing_fixture() -> (
            tempfile::TempDir,
            PathBuf,
            File,
            ExactDiskImageBackingIdentityV3,
            HeldUnlinkedDiskImageBackingV3,
        ) {
            let (directory, path, external, held) = make_present_backing_fixture();
            let prepared = held.exact_identity_v3().expect("exact prepared identity");
            std::fs::remove_file(&path).expect("external test-only backing unlink");
            let unlinked = held
                .observe_namespace_unlinked()
                .expect("observe externally unlinked retained backing");
            (directory, path, external, prepared, unlinked)
        }

        #[test]
        fn rootless_held_backing_replay_detects_same_inode_content_drift() {
            let directory = tempfile::tempdir().expect("create rootless backing directory");
            let path = directory.path().join("image.dmg");
            std::fs::write(&path, b"held-backing-before").expect("write rootless backing fixture");
            let path = path
                .canonicalize()
                .expect("canonical rootless backing path");
            let path_text = path.to_str().expect("UTF-8 rootless backing path");
            let candidate = DiskImageCandidateObservation {
                device: IORegistryAncestorV1 {
                    class_name: "AppleDiskImageDevice".to_string(),
                    registry_entry_id: "0000000000000001".to_string(),
                    registry_path: Some("IOService:/AppleDiskImageDevice".to_string()),
                },
                url: format!("file://{path_text}"),
            };
            let backing = HeldDiskImageBacking::capture(&path)
                .expect("capture rootless held backing descriptors");
            let report = backing
                .finish(&candidate, 1, 1)
                .expect("replay unchanged rootless held backing");
            let file = &report
                .opened_components
                .last()
                .expect("final backing component")
                .fd_binding;
            assert_eq!(file.size, b"held-backing-before".len() as u64);
            let expected_digest = format!("{:x}", Sha256::digest(b"held-backing-before"));
            assert_eq!(
                file.content_sha256.as_deref(),
                Some(expected_digest.as_str())
            );

            std::fs::write(&path, b"held-backing-after!")
                .expect("mutate same-inode rootless backing fixture");
            assert_eq!(
                std::fs::metadata(&path)
                    .expect("mutated backing metadata")
                    .ino(),
                file.inode
            );
            assert!(backing.finish(&candidate, 1, 1).is_err());
        }

        #[test]
        fn live_unlink_retains_exact_zero_link_inode_and_replays_after_persistence() {
            let (_directory, path, _external, prepared, unlinked) = make_unlinked_backing_fixture();
            let binding = unlinked.binding();
            assert_eq!(binding.kind, UNLINKED_BACKING_KIND);
            assert!(!binding.authority_granted);
            assert_eq!(binding.canonical_path, path.to_str().expect("UTF-8 path"));
            assert_eq!(binding.initial_file.nlink, 1);
            assert_eq!(binding.post_unlink_file.nlink, 0);
            assert_eq!(
                binding.prepared_backing_sha256,
                sha256(&canonical_json(&prepared).expect("canonical prepared bytes"))
            );
            assert_eq!(
                binding.content_sha256,
                format!("{:x}", Sha256::digest(UNLINKED_FIXTURE_BYTES))
            );
            validate_unlinked_backing_binding_v3(binding).expect("canonical unlink evidence");
            unlinked
                .revalidate_after_persistence()
                .expect("post-persistence unlinked replay");
        }

        #[test]
        fn live_unlinked_replay_rejects_same_name_file_symlink_and_hardlink_recreation() {
            for replacement in ["same-bytes", "symlink", "hardlink"] {
                let (_directory, path, _external, _prepared, unlinked) =
                    make_unlinked_backing_fixture();
                match replacement {
                    "same-bytes" => std::fs::write(&path, UNLINKED_FIXTURE_BYTES)
                        .expect("recreate same-byte pathname"),
                    "symlink" => {
                        symlink("missing-target", &path).expect("recreate pathname as a symlink")
                    }
                    "hardlink" => {
                        let donor = path.with_file_name("donor.dmg");
                        std::fs::write(&donor, UNLINKED_FIXTURE_BYTES)
                            .expect("write hardlink donor");
                        std::fs::hard_link(&donor, &path).expect("recreate pathname as a hardlink");
                    }
                    _ => unreachable!("closed replacement roster"),
                }
                assert!(
                    unlinked.revalidate_after_persistence().is_err(),
                    "{replacement} recreation was accepted as namespace absence"
                );
            }
        }

        #[test]
        fn live_unlinked_replay_rejects_ancestor_content_and_metadata_mutation() {
            {
                let (_directory, path, _external, _prepared, unlinked) =
                    make_unlinked_backing_fixture();
                let outer = path
                    .parent()
                    .and_then(Path::parent)
                    .expect("outer ancestor");
                let renamed = outer.with_file_name("outer-replaced");
                std::fs::rename(outer, &renamed).expect("rename retained non-parent ancestor");
                assert!(unlinked.revalidate_after_persistence().is_err());
            }

            {
                let (_directory, _path, mut external, _prepared, unlinked) =
                    make_unlinked_backing_fixture();
                external
                    .seek(SeekFrom::Start(0))
                    .expect("seek retained external file");
                external
                    .write_all(&vec![b'X'; UNLINKED_FIXTURE_BYTES.len()])
                    .expect("mutate retained backing content");
                external.sync_all().expect("sync mutated backing content");
                assert!(unlinked.revalidate_after_persistence().is_err());
            }

            {
                let (_directory, _path, external, _prepared, unlinked) =
                    make_unlinked_backing_fixture();
                external
                    .set_permissions(std::fs::Permissions::from_mode(0o600))
                    .expect("mutate retained backing metadata");
                external.sync_all().expect("sync mutated backing metadata");
                assert!(unlinked.revalidate_after_persistence().is_err());
            }
        }

        #[test]
        fn recovered_path_absence_requires_exact_prepared_ancestors_and_enoent() {
            let (_directory, path, _external, held) = make_present_backing_fixture();
            let prepared = held.exact_identity_v3().expect("exact prepared identity");
            assert!(
                HeldBackingPathAbsenceV3::recover_from_exact_prepared_for_test(&prepared).is_err()
            );
            drop(held);
            std::fs::remove_file(&path).expect("external test-only backing unlink");
            let recovered =
                HeldBackingPathAbsenceV3::recover_from_exact_prepared_for_test(&prepared)
                    .expect("recover exact prepared basename absence");
            assert_eq!(recovered.binding().kind, BACKING_PATH_ABSENCE_KIND);
            assert_eq!(
                recovered.binding().prepared_backing_sha256,
                sha256(&canonical_json(&prepared).expect("canonical prepared bytes"))
            );
            recovered
                .revalidate_after_persistence()
                .expect("replay recovered basename absence");

            let mut nul_path = recovered.binding().clone();
            nul_path.canonical_path.push('\0');
            assert!(validate_backing_path_absence_binding_v3(&nul_path).is_err());
            let mut nul_basename = recovered.binding().clone();
            nul_basename.basename.push('\0');
            assert!(validate_backing_path_absence_binding_v3(&nul_basename).is_err());
        }

        #[test]
        fn recovered_path_absence_rejects_recreation_and_non_parent_ancestor_mutation() {
            {
                let (_directory, path, _external, held) = make_present_backing_fixture();
                let prepared = held.exact_identity_v3().expect("exact prepared identity");
                drop(held);
                std::fs::remove_file(&path).expect("external test-only backing unlink");
                std::fs::write(&path, UNLINKED_FIXTURE_BYTES).expect("recreate prepared pathname");
                assert!(
                    HeldBackingPathAbsenceV3::recover_from_exact_prepared_for_test(&prepared)
                        .is_err()
                );
            }

            {
                let (_directory, path, _external, held) = make_present_backing_fixture();
                let prepared = held.exact_identity_v3().expect("exact prepared identity");
                drop(held);
                std::fs::remove_file(&path).expect("external test-only backing unlink");
                let outer = path
                    .parent()
                    .and_then(Path::parent)
                    .expect("prepared non-parent ancestor");
                std::fs::set_permissions(outer, std::fs::Permissions::from_mode(0o711))
                    .expect("mutate prepared non-parent ancestor");
                assert!(
                    HeldBackingPathAbsenceV3::recover_from_exact_prepared_for_test(&prepared)
                        .is_err()
                );
            }
        }

        #[test]
        fn production_backing_helpers_contain_no_namespace_removal_primitive() {
            let source = include_str!("mac_iomedia_identity.rs");
            let live_start = source
                .find("    #[cfg(test)]\n    mod live_tests {")
                .expect("live-test source boundary");
            let live_end = source[live_start..]
                .find("\n}\n\n#[cfg(target_os = \"macos\")]\npub use")
                .map(|offset| live_start + offset)
                .expect("platform source boundary");
            let portable_tests = source
                .find("\n#[cfg(test)]\nmod tests {")
                .expect("portable-test source boundary");
            let production = format!(
                "{}{}",
                &source[..live_start],
                &source[live_end..portable_tests]
            );
            for forbidden in [
                "libc::unlink(",
                "libc::unlinkat(",
                "std::fs::remove_file(",
                "fs::remove_file(",
            ] {
                assert!(
                    !production.contains(forbidden),
                    "production source contains forbidden removal primitive {forbidden}"
                );
            }
        }

        fn capture_all_nodes(boot: &str) -> Result<Vec<CapturedNode>, AcceptanceError> {
            enumerate()?
                .into_iter()
                .map(|identity| {
                    provenance_from_resolved(resolve(
                        &identity.registry_entry_id,
                        boot,
                        &identity.bsd_name,
                    )?)
                })
                .collect()
        }

        fn node_by_id<'a>(
            nodes: &'a [CapturedNode],
            id: &str,
            label: &str,
        ) -> Result<&'a CapturedNode, AcceptanceError> {
            let matches = nodes
                .iter()
                .filter(|node| node.report.registry_entry_id == id)
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return Err(invalid(format!(
                    "live corpus has {} matches for {label} registry ID",
                    matches.len()
                )));
            }
            Ok(matches[0])
        }

        #[test]
        fn rootless_live_v2_t5_and_optional_preexisting_disk_image_corpus() {
            match std::env::var("HEPTA_REQUIRE_T5_IOMEDIA_LIVE_CORPUS") {
                Ok(value) if value == "1" => {}
                Ok(value) => {
                    panic!("HEPTA_REQUIRE_T5_IOMEDIA_LIVE_CORPUS must be exactly 1, got {value:?}")
                }
                Err(_) => {
                    eprintln!(
                        "SKIP live T5 corpus: set HEPTA_REQUIRE_T5_IOMEDIA_LIVE_CORPUS=1 for qualification"
                    );
                    return;
                }
            }
            let boot = current_boot_session_uuid_impl().expect("live boot UUID");
            let nodes = capture_all_nodes(&boot).expect("capture live IOMedia corpus");
            let t5_volumes = nodes
                .iter()
                .filter(|node| {
                    node.report.disk_arbitration.media_uuid.as_deref()
                        == Some(EXPECTED_T5_VOLUME_UUID)
                })
                .collect::<Vec<_>>();
            assert_eq!(t5_volumes.len(), 1, "canonical T5 volume must be unique");
            let t5_volume = t5_volumes[0];
            let t5_container = node_by_id(
                &nodes,
                &t5_volume.report.whole_disk.registry_entry_id,
                "T5 APFS container",
            )
            .expect("T5 container by DA whole disk");
            let t5_store_id = &t5_container
                .report
                .ancestry
                .get(2)
                .expect("T5 container store ancestor")
                .registry_entry_id;
            let t5_store =
                node_by_id(&nodes, t5_store_id, "T5 physical store").expect("T5 store node");
            let t5_whole = node_by_id(
                &nodes,
                &t5_store.report.whole_disk.registry_entry_id,
                "T5 physical whole",
            )
            .expect("T5 physical whole node");
            let t5_names = [
                t5_whole.report.bsd_name.clone(),
                t5_store.report.bsd_name.clone(),
                t5_container.report.bsd_name.clone(),
                t5_volume.report.bsd_name.clone(),
            ];
            let t5_expected = ExpectedIOMediaTopology {
                apfs_container: &t5_names[2],
                apfs_volume: &t5_names[3],
                physical_store: &t5_names[1],
                physical_whole: &t5_names[0],
            };
            let mut baseline: Option<IOMediaRegistryInventoryV2> = None;
            for iteration in 0..5 {
                let held_inventory = capture_inventory(t5_expected)
                    .unwrap_or_else(|error| panic!("live T5 capture {iteration} failed: {error}"));
                let inventory = held_inventory.report().clone();
                let encoded = serde_json::to_vec(held_inventory.report())
                    .expect("serialize held pre-attach report while descriptors remain live");
                assert!(!encoded.is_empty());
                held_inventory
                    .revalidate_exact()
                    .expect("revalidate held pre-attach descriptors after report bytes");
                assert!(!inventory.authority_granted);
                assert!(inventory.all_registry_entry_ids.len() >= nodes.len());
                if let Some(previous) = baseline.as_ref() {
                    let mut expected = previous.clone();
                    expected.capture_monotonic_nanoseconds =
                        inventory.capture_monotonic_nanoseconds;
                    assert_eq!(inventory, expected, "T5 capture {iteration} drifted");
                    assert!(
                        inventory.capture_monotonic_nanoseconds
                            > previous.capture_monotonic_nanoseconds
                    );
                }
                baseline = Some(inventory);
            }

            let image_chain = nodes
                .iter()
                .filter(|node| {
                    node.report.ancestry[0].class_name == "IOMedia"
                        && node.report.iomedia.whole == Some(true)
                        && node.report.iomedia.leaf == Some(false)
                        && node.ancestry.disk_image_device_count == 1
                        && node.ancestry.disk_image_candidates.len() == 1
                })
                .find_map(|image_whole| {
                    let image_store = nodes.iter().find(|node| {
                        node.report.ancestry.get(2..)
                            == Some(image_whole.report.ancestry.as_slice())
                            && node.report.ancestry.get(1).is_some_and(|ancestor| {
                                ancestor.class_name == "IOGUIDPartitionScheme"
                            })
                    })?;
                    let image_container = nodes.iter().find(|node| {
                        node.report.ancestry.get(2..)
                            == Some(image_store.report.ancestry.as_slice())
                            && node.report.ancestry.get(1).is_some_and(|ancestor| {
                                ancestor.class_name == "AppleAPFSContainerScheme"
                            })
                    })?;
                    let image_volume =
                        nodes.iter().find(|node| {
                            node.report.ancestry.get(2..)
                                == Some(image_container.report.ancestry.as_slice())
                                && node.report.ancestry.get(1).is_some_and(|ancestor| {
                                    ancestor.class_name == "AppleAPFSContainer"
                                })
                        })?;
                    Some((image_whole, image_store, image_container, image_volume))
                });
            let Some((image_whole, image_store, image_container, image_volume)) = image_chain
            else {
                eprintln!(
                    "SKIP corpus-only disk-image check: no pre-existing image has a complete exact APFS four-node chain"
                );
                return;
            };
            let image_names = [
                image_whole.report.bsd_name.clone(),
                image_store.report.bsd_name.clone(),
                image_container.report.bsd_name.clone(),
                image_volume.report.bsd_name.clone(),
            ];
            let image_expected = ExpectedIOMediaTopology {
                apfs_container: &image_names[2],
                apfs_volume: &image_names[3],
                physical_store: &image_names[1],
                physical_whole: &image_names[0],
            };
            let first = capture_four(image_expected, &boot, "pre-existing disk-image corpus")
                .expect("capture exact live disk-image four-node chain");
            let candidate =
                unique_disk_image_candidate(&first).expect("unique live DiskImageURL candidate");
            let path = strict_file_url_path(&candidate.url).expect("strict live DiskImageURL");
            let metadata = std::fs::metadata(&path).expect("live disk-image backing metadata");
            if !metadata.is_file()
                || metadata.nlink() != 1
                || metadata.len() == 0
                || metadata.len() > MAX_BACKING_FILE_BYTES
            {
                eprintln!(
                    "SKIP corpus-only disk-image check: backing is not a single-link regular file within the {MAX_BACKING_FILE_BYTES}-byte digest bound"
                );
                return;
            }
            let backing = HeldDiskImageBacking::capture(Path::new(&path))
                .expect("hold live disk-image backing through openat");
            let first_backing = backing
                .finish(&candidate, 1, 1)
                .expect("first live backing replay");
            let replay = capture_four(image_expected, &boot, "pre-existing image exact replay")
                .expect("replay exact live disk-image chain");
            let replay_candidate =
                unique_disk_image_candidate(&replay).expect("replay unique live DiskImageURL");
            assert_eq!(replay.topology, first.topology);
            assert_eq!(replay_candidate, candidate);
            assert_eq!(
                backing
                    .finish(&replay_candidate, 1, 1)
                    .expect("final live backing replay"),
                first_backing
            );
            assert!(!first_backing.authority_granted);
            assert!(!first_backing.path_authority_granted);
            assert_eq!(
                current_boot_session_uuid_impl().expect("final boot UUID"),
                boot
            );
        }
    }
}

#[cfg(target_os = "macos")]
pub use platform::HeldAttachedIOMediaTopologyV2;
#[cfg(target_os = "macos")]
pub use platform::HeldBackingPathAbsenceV3;
#[cfg(target_os = "macos")]
pub use platform::HeldDiskImageBacking;
#[cfg(target_os = "macos")]
pub use platform::HeldPreAttachIOMediaInventory;
#[cfg(target_os = "macos")]
pub use platform::HeldRestartIOMediaInventoryV3;
#[cfg(target_os = "macos")]
pub use platform::HeldUnlinkedDiskImageBackingV3;
#[cfg(target_os = "macos")]
pub use platform::ResolvedIOMediaObject;

#[cfg(not(target_os = "macos"))]
#[must_use = "held attached descriptors require post-persistence revalidation"]
pub struct HeldAttachedIOMediaTopologyV2<'a> {
    _unsupported: std::marker::PhantomData<&'a HeldPreAttachIOMediaInventory>,
}

#[cfg(not(target_os = "macos"))]
pub struct HeldDiskImageBacking {
    _unsupported: (),
}

#[cfg(not(target_os = "macos"))]
#[must_use = "held unlinked backing requires post-persistence revalidation"]
pub struct HeldUnlinkedDiskImageBackingV3 {
    _unsupported: (),
    _not_send_or_sync: std::marker::PhantomData<std::rc::Rc<()>>,
}

#[cfg(not(target_os = "macos"))]
#[must_use = "held backing path absence requires post-persistence revalidation"]
pub struct HeldBackingPathAbsenceV3 {
    _unsupported: (),
    _not_send_or_sync: std::marker::PhantomData<std::rc::Rc<()>>,
}

#[cfg(not(target_os = "macos"))]
#[must_use = "held pre-attach descriptors must outlive attached receipt persistence"]
pub struct HeldPreAttachIOMediaInventory {
    _unsupported: (),
}

#[cfg(not(target_os = "macos"))]
#[must_use = "restart IOMedia handles must outlive collector receipt persistence"]
pub struct HeldRestartIOMediaInventoryV3 {
    _unsupported: (),
}

#[cfg(not(target_os = "macos"))]
pub struct ResolvedIOMediaObject {
    _unsupported: (),
}

pub fn hold_disk_image_backing(path: &Path) -> Result<HeldDiskImageBacking, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::HeldDiskImageBacking::capture(path)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = path;
        Err(invalid(
            "disk-image backing descriptor capture is unsupported outside macOS",
        ))
    }
}

pub fn capture_restart_iomedia_inventory_v3()
-> Result<HeldRestartIOMediaInventoryV3, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::capture_restart_inventory()
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(invalid(
            "restart IOMedia inventory capture is unsupported outside macOS",
        ))
    }
}

#[cfg(all(test, target_os = "macos"))]
pub(crate) fn capture_restart_disk_image_url_identity_for_test(
    disk_image_url: &str,
) -> Result<RestartDiskImageBackingIdentityV3, AcceptanceError> {
    platform::capture_restart_url_backing_identity_for_test(disk_image_url)
}

pub fn capture_pre_attach_iomedia_inventory(
    expected_t5: ExpectedIOMediaTopology<'_>,
) -> Result<HeldPreAttachIOMediaInventory, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::capture_inventory(expected_t5)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = expected_t5;
        Err(invalid(
            "pre-attach IOMedia inventory capture is unsupported outside macOS",
        ))
    }
}

pub fn capture_attached_iomedia_topology_v2<'a>(
    expected: ExpectedIOMediaTopology<'_>,
    pre_attach_inventory: &'a HeldPreAttachIOMediaInventory,
    held_backing: HeldDiskImageBacking,
) -> Result<HeldAttachedIOMediaTopologyV2<'a>, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::capture_v2(expected, pre_attach_inventory, held_backing)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (expected, pre_attach_inventory, held_backing);
        Err(invalid(
            "attached IOMedia provenance capture is unsupported outside macOS",
        ))
    }
}

pub fn project_iomedia_identity_v1(
    topology: &AttachedIOMediaTopologyV2,
) -> Result<AttachedIOMediaTopologyV1, AcceptanceError> {
    let expected = ExpectedIOMediaTopology {
        apfs_container: &topology.apfs_container.bsd_name,
        apfs_volume: &topology.apfs_volume.bsd_name,
        physical_store: &topology.physical_store.bsd_name,
        physical_whole: &topology.physical_whole.bsd_name,
    };
    validate_iomedia_topology_provenance_shape(topology, expected)?;
    let identity = AttachedIOMediaTopologyV1 {
        apfs_container: registry_identity(
            topology.apfs_container.bsd_name.clone(),
            parse_registry_entry_id(&topology.apfs_container.registry_entry_id)?,
        ),
        apfs_volume: registry_identity(
            topology.apfs_volume.bsd_name.clone(),
            parse_registry_entry_id(&topology.apfs_volume.registry_entry_id)?,
        ),
        authority_granted: false,
        boot_session_uuid: topology.boot_session_uuid.clone(),
        physical_store: registry_identity(
            topology.physical_store.bsd_name.clone(),
            parse_registry_entry_id(&topology.physical_store.registry_entry_id)?,
        ),
        physical_whole: registry_identity(
            topology.physical_whole.bsd_name.clone(),
            parse_registry_entry_id(&topology.physical_whole.registry_entry_id)?,
        ),
        schema: TOPOLOGY_SCHEMA.to_string(),
    };
    validate_iomedia_topology_identity_shape(&identity, expected)?;
    Ok(identity)
}

#[cfg(not(target_os = "macos"))]
impl HeldPreAttachIOMediaInventory {
    pub fn report(&self) -> &IOMediaRegistryInventoryV2 {
        unreachable!("non-macOS capture never constructs a held pre-attach capsule")
    }
}

#[cfg(not(target_os = "macos"))]
impl HeldDiskImageBacking {
    pub fn identity(&self) -> Result<DiskImageBackingIdentityV2, AcceptanceError> {
        Err(invalid(
            "held disk-image backing replay is unsupported outside macOS",
        ))
    }

    pub fn exact_identity_v3(&self) -> Result<ExactDiskImageBackingIdentityV3, AcceptanceError> {
        Err(invalid(
            "held exact disk-image backing replay is unsupported outside macOS",
        ))
    }

    pub fn revalidate_identity_after_persistence(
        &self,
        _expected: &DiskImageBackingIdentityV2,
    ) -> Result<(), AcceptanceError> {
        Err(invalid(
            "held disk-image backing replay is unsupported outside macOS",
        ))
    }

    pub fn observe_namespace_unlinked(
        self,
    ) -> Result<HeldUnlinkedDiskImageBackingV3, AcceptanceError> {
        Err(invalid(
            "held backing namespace-unlink observation is unsupported outside macOS",
        ))
    }
}

#[cfg(not(target_os = "macos"))]
impl HeldUnlinkedDiskImageBackingV3 {
    pub fn binding(&self) -> &UnlinkedBackingBindingV3 {
        unreachable!("non-macOS capture never constructs a held unlinked backing capsule")
    }

    pub fn revalidate_after_persistence(&self) -> Result<(), AcceptanceError> {
        Err(invalid(
            "held unlinked backing replay is unsupported outside macOS",
        ))
    }
}

#[cfg(not(target_os = "macos"))]
impl HeldBackingPathAbsenceV3 {
    #[cfg(test)]
    pub fn recover_from_exact_prepared_for_test(
        _prepared: &ExactDiskImageBackingIdentityV3,
    ) -> Result<Self, AcceptanceError> {
        Err(invalid(
            "backing path-absence recovery is unsupported outside macOS",
        ))
    }

    pub fn binding(&self) -> &BackingPathAbsenceBindingV3 {
        unreachable!("non-macOS capture never constructs a held backing-absence capsule")
    }

    pub fn revalidate_after_persistence(&self) -> Result<(), AcceptanceError> {
        Err(invalid(
            "held backing path-absence replay is unsupported outside macOS",
        ))
    }
}

#[cfg(not(target_os = "macos"))]
impl HeldRestartIOMediaInventoryV3 {
    pub fn report(&self) -> &RestartIOMediaInventoryV3 {
        unreachable!("non-macOS capture never constructs a restart inventory")
    }

    #[cfg(test)]
    pub(crate) fn poison_boot_session_for_test(&mut self) {}

    #[cfg(test)]
    pub(crate) fn substitute_valid_property_for_test(&mut self) {}

    pub fn revalidate_after_persistence(&self) -> Result<(), AcceptanceError> {
        Err(invalid(
            "restart IOMedia replay is unsupported outside macOS",
        ))
    }
}

#[cfg(not(target_os = "macos"))]
impl<'a> HeldAttachedIOMediaTopologyV2<'a> {
    pub fn report(&self) -> &AttachedIOMediaTopologyV2 {
        unreachable!("non-macOS capture never constructs a held attached capsule")
    }

    pub fn revalidate_after_persistence(&self) -> Result<(), AcceptanceError> {
        Err(invalid(
            "held IOMedia descriptor replay is unsupported outside macOS",
        ))
    }
}

#[cfg(not(target_os = "macos"))]
impl ResolvedIOMediaObject {
    pub fn identity(&self) -> &IOMediaRegistryIdentityV1 {
        unreachable!("non-macOS resolver never constructs an IOMedia object")
    }
}

pub fn current_boot_session_uuid() -> Result<String, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::current_boot_session_uuid_impl()
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(invalid(
            "IOMedia boot-session identity is unsupported outside macOS",
        ))
    }
}

pub fn enumerate_iomedia_registry_identities()
-> Result<Vec<IOMediaRegistryIdentityV1>, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::enumerate()
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(invalid("IOMedia enumeration is unsupported outside macOS"))
    }
}

pub fn resolve_iomedia_registry_identity(
    registry_entry_id: &str,
    expected_boot_session_uuid: &str,
    expected_bsd_name: &str,
) -> Result<ResolvedIOMediaObject, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::resolve(
            registry_entry_id,
            expected_boot_session_uuid,
            expected_bsd_name,
        )
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (
            registry_entry_id,
            expected_boot_session_uuid,
            expected_bsd_name,
        );
        Err(invalid(
            "IOMedia registry-entry resolution is unsupported outside macOS",
        ))
    }
}

pub fn capture_attached_iomedia_topology(
    expected: ExpectedIOMediaTopology<'_>,
) -> Result<AttachedIOMediaTopologyV1, AcceptanceError> {
    #[cfg(target_os = "macos")]
    {
        platform::capture(expected)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = expected;
        Err(invalid(
            "attached IOMedia topology capture is unsupported outside macOS",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_not_impl {
        ($type:ty, $trait:path) => {
            const _: fn() = || {
                trait AmbiguousIfImpl<A> {
                    fn marker() {}
                }
                impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
                impl<T: ?Sized + $trait> AmbiguousIfImpl<u8> for T {}
                let _ = <$type as AmbiguousIfImpl<_>>::marker;
            };
        };
    }

    type HeldUnlinkedBackingForCompileAssertions = HeldUnlinkedDiskImageBackingV3;
    type HeldPathAbsenceForCompileAssertions = HeldBackingPathAbsenceV3;
    assert_not_impl!(HeldUnlinkedBackingForCompileAssertions, Clone);
    assert_not_impl!(HeldUnlinkedBackingForCompileAssertions, Send);
    assert_not_impl!(HeldUnlinkedBackingForCompileAssertions, Sync);
    assert_not_impl!(HeldUnlinkedBackingForCompileAssertions, serde::Serialize);
    assert_not_impl!(
        HeldUnlinkedBackingForCompileAssertions,
        std::os::fd::AsRawFd
    );
    assert_not_impl!(HeldUnlinkedBackingForCompileAssertions, From<std::fs::File>);
    assert_not_impl!(HeldPathAbsenceForCompileAssertions, Clone);
    assert_not_impl!(HeldPathAbsenceForCompileAssertions, Send);
    assert_not_impl!(HeldPathAbsenceForCompileAssertions, Sync);
    assert_not_impl!(HeldPathAbsenceForCompileAssertions, serde::Serialize);
    assert_not_impl!(HeldPathAbsenceForCompileAssertions, std::os::fd::AsRawFd);
    assert_not_impl!(HeldPathAbsenceForCompileAssertions, From<std::fs::File>);
    assert_not_impl!(
        HeldPathAbsenceForCompileAssertions,
        From<ExactDiskImageBackingIdentityV3>
    );
    assert_not_impl!(
        HeldPathAbsenceForCompileAssertions,
        From<&'static ExactDiskImageBackingIdentityV3>
    );

    #[test]
    fn raw_prepared_absence_recovery_is_test_only() {
        let source = include_str!("mac_iomedia_identity.rs");
        let production_name = ["pub fn recover_from_exact_", "prepared("].concat();
        assert!(
            !source.contains(&production_name),
            "the caller-authored DTO recovery API must not exist"
        );
        let test_name = ["pub fn recover_from_exact_", "prepared_for_test("].concat();
        let lines = source.lines().collect::<Vec<_>>();
        let definitions = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.contains(&test_name))
            .collect::<Vec<_>>();
        assert_eq!(definitions.len(), 2, "macOS and fail-closed stubs only");
        for (index, _) in definitions {
            assert!(index > 0);
            assert_eq!(
                lines[index - 1].trim(),
                "#[cfg(test)]",
                "raw prepared DTO recovery escaped its test-only boundary"
            );
        }
    }

    fn test_identity(name: &str, id: u64) -> IOMediaRegistryIdentityV1 {
        registry_identity(name.to_string(), id)
    }

    fn test_topology(boot: &str) -> AttachedIOMediaTopologyV1 {
        AttachedIOMediaTopologyV1 {
            apfs_container: test_identity("disk10", 103),
            apfs_volume: test_identity("disk10s1", 104),
            authority_granted: false,
            boot_session_uuid: boot.to_string(),
            physical_store: test_identity("disk9s1", 102),
            physical_whole: test_identity("disk9", 101),
            schema: TOPOLOGY_SCHEMA.to_string(),
        }
    }

    fn expected() -> ExpectedIOMediaTopology<'static> {
        ExpectedIOMediaTopology {
            apfs_container: "disk10",
            apfs_volume: "disk10s1",
            physical_store: "disk9s1",
            physical_whole: "disk9",
        }
    }

    fn test_ancestor(class_name: &str, id: u64) -> IORegistryAncestorV1 {
        IORegistryAncestorV1 {
            class_name: class_name.to_string(),
            registry_entry_id: format!("{id:016x}"),
            registry_path: (class_name != "IORegistryEntry")
                .then(|| format!("IOService:/test-{id}")),
        }
    }

    fn test_properties(role: IOMediaRole) -> IOMediaRegistryPropertiesV2 {
        IOMediaRegistryPropertiesV2 {
            content: Some("41504653-0000-11AA-AA11-00306543ECAC".to_string()),
            ejectable: Some(false),
            leaf: Some(role.expected_leaf()),
            preferred_block_size: Some(4096),
            removable: Some(false),
            size: Some(1024 * 1024),
            whole: Some(role.expected_whole()),
            writable: Some(true),
        }
    }

    fn test_da_properties(
        role: IOMediaRole,
        media_uuid: Option<&str>,
    ) -> DiskArbitrationPropertiesV2 {
        DiskArbitrationPropertiesV2 {
            block_size: Some(4096),
            content: Some("41504653-0000-11AA-AA11-00306543ECAC".to_string()),
            ejectable: Some(false),
            internal: None,
            leaf: Some(role.expected_leaf()),
            media_uuid: media_uuid.map(str::to_string),
            removable: Some(false),
            size: Some(1024 * 1024),
            whole: Some(role.expected_whole()),
            writable: Some(true),
        }
    }

    fn test_provenance_node(
        name: &str,
        id: u64,
        role: IOMediaRole,
        ancestry: Vec<IORegistryAncestorV1>,
        whole_name: &str,
        whole_id: u64,
        media_uuid: Option<&str>,
    ) -> IOMediaRegistryProvenanceV2 {
        IOMediaRegistryProvenanceV2 {
            registry_path: ancestry[0]
                .registry_path
                .clone()
                .expect("IOMedia test node has a registry path"),
            ancestry,
            authority_granted: false,
            bsd_name: name.to_string(),
            conforms_to_iomedia: true,
            disk_arbitration: test_da_properties(role, media_uuid),
            iomedia: test_properties(role),
            registry_entry_id: format!("{id:016x}"),
            whole_disk: test_identity(whole_name, whole_id),
            schema: PROVENANCE_SCHEMA.to_string(),
        }
    }

    fn test_four(
        base: u64,
        physical_whole_name: &str,
        physical_store_name: &str,
        container_name: &str,
        volume_name: &str,
        volume_uuid: &str,
        disk_image: bool,
    ) -> IOMediaFourNodeTopologyV2 {
        let whole_id = base;
        let store_id = base + 1;
        let container_id = base + 2;
        let volume_id = base + 3;
        let mut whole_ancestry = vec![test_ancestor("IOMedia", whole_id)];
        if disk_image {
            whole_ancestry.extend([
                test_ancestor("IOBlockStorageDriver", base + 100),
                test_ancestor("AppleDiskImageDevice", base + 101),
                test_ancestor("IORegistryEntry", base + 102),
            ]);
        } else {
            whole_ancestry.extend([
                test_ancestor("IOBlockStorageDriver", base + 100),
                test_ancestor("IOUSBMassStorageDriver", base + 101),
                test_ancestor("IORegistryEntry", base + 102),
            ]);
        }
        let whole = test_provenance_node(
            physical_whole_name,
            whole_id,
            IOMediaRole::PhysicalWhole,
            whole_ancestry,
            physical_whole_name,
            whole_id,
            None,
        );
        let mut store_ancestry = vec![
            test_ancestor("IOMedia", store_id),
            test_ancestor("IOGUIDPartitionScheme", base + 110),
        ];
        store_ancestry.extend(whole.ancestry.clone());
        let store = test_provenance_node(
            physical_store_name,
            store_id,
            IOMediaRole::PhysicalStore,
            store_ancestry,
            physical_whole_name,
            whole_id,
            Some("11111111-1111-4111-8111-111111111111"),
        );
        let mut container_ancestry = vec![
            test_ancestor("AppleAPFSMedia", container_id),
            test_ancestor("AppleAPFSContainerScheme", base + 120),
        ];
        container_ancestry.extend(store.ancestry.clone());
        let container = test_provenance_node(
            container_name,
            container_id,
            IOMediaRole::ApfsContainer,
            container_ancestry,
            container_name,
            container_id,
            Some("22222222-2222-4222-8222-222222222222"),
        );
        let mut volume_ancestry = vec![
            test_ancestor("AppleAPFSVolume", volume_id),
            test_ancestor("AppleAPFSContainer", base + 130),
        ];
        volume_ancestry.extend(container.ancestry.clone());
        let volume = test_provenance_node(
            volume_name,
            volume_id,
            IOMediaRole::ApfsVolume,
            volume_ancestry,
            container_name,
            container_id,
            Some(volume_uuid),
        );
        IOMediaFourNodeTopologyV2 {
            apfs_container: container,
            apfs_volume: volume,
            authority_granted: false,
            boot_session_uuid: "11111111-1111-4111-8111-111111111111".to_string(),
            physical_store: store,
            physical_whole: whole,
            schema: FOUR_NODE_TOPOLOGY_SCHEMA.to_string(),
        }
    }

    fn test_binding(mode: u32, inode: u64) -> BackingObjectBindingV1 {
        BackingObjectBindingV1 {
            content_sha256: None,
            ctime_nanoseconds: 0,
            ctime_seconds: 1,
            dev: 1,
            flags: 0,
            gid: 0,
            inode,
            mode,
            mtime_nanoseconds: 0,
            mtime_seconds: 1,
            nlink: 1,
            size: 4096,
            uid: 0,
        }
    }

    fn test_backing(device: IORegistryAncestorV1) -> DiskImageBackingProvenanceV1 {
        let component = |path: &str,
                         directory: bool,
                         mut binding: BackingObjectBindingV1|
         -> BackingPathComponentV1 {
            if !directory {
                binding.content_sha256 = Some("11".repeat(32));
            }
            BackingPathComponentV1 {
                directory,
                fd_binding: binding.clone(),
                path: path.to_string(),
                path_binding_after: binding.clone(),
                path_binding_before: binding,
            }
        };
        DiskImageBackingProvenanceV1 {
            authority_granted: false,
            canonical_path: "/Library/image.dmg".to_string(),
            disk_image_device: device,
            disk_image_device_ancestor_count: 1,
            disk_image_url: "file:///Library/image.dmg".to_string(),
            disk_image_url_ancestor_count: 1,
            opened_components: vec![
                component("/", true, test_binding(libc::S_IFDIR as u32 | 0o755, 1)),
                component(
                    "/Library",
                    true,
                    test_binding(libc::S_IFDIR as u32 | 0o755, 2),
                ),
                component(
                    "/Library/image.dmg",
                    false,
                    test_binding(libc::S_IFREG as u32 | 0o644, 3),
                ),
            ],
            path_authority_granted: false,
            schema: BACKING_SCHEMA.to_string(),
        }
    }

    fn test_v2_topology() -> AttachedIOMediaTopologyV2 {
        let t5 = test_four(
            11,
            "disk6",
            "disk6s2",
            "disk7",
            "disk7s1",
            EXPECTED_T5_VOLUME_UUID,
            false,
        );
        let attached = test_four(
            101,
            "disk9",
            "disk9s1",
            "disk10",
            "disk10s1",
            "33333333-3333-4333-8333-333333333333",
            true,
        );
        let mut all_registry_entry_ids = t5
            .ordered()
            .into_iter()
            .map(|(_, node)| node.registry_entry_id.clone())
            .collect::<Vec<_>>();
        all_registry_entry_ids.push("00000000000000ff".to_string());
        all_registry_entry_ids.sort();
        let inventory = IOMediaRegistryInventoryV2 {
            all_registry_entry_ids,
            authority_granted: false,
            boot_session_uuid: t5.boot_session_uuid.clone(),
            capture_monotonic_nanoseconds: 1,
            schema: REGISTRY_INVENTORY_SCHEMA.to_string(),
            t5_apfs_container: t5.apfs_container.clone(),
            t5_apfs_volume: t5.apfs_volume.clone(),
            t5_physical_store: t5.physical_store.clone(),
            t5_physical_whole: t5.physical_whole.clone(),
            t5_volume_uuid: EXPECTED_T5_VOLUME_UUID.to_string(),
        };
        AttachedIOMediaTopologyV2 {
            apfs_container: attached.apfs_container,
            apfs_volume: attached.apfs_volume,
            authority_granted: false,
            backing: test_backing(attached.physical_whole.ancestry[2].clone()),
            boot_session_uuid: attached.boot_session_uuid,
            fresh_t5: t5,
            physical_store: attached.physical_store,
            physical_whole: attached.physical_whole,
            pre_attach_inventory: inventory,
            schema: PROVENANCE_TOPOLOGY_SCHEMA.to_string(),
        }
    }

    #[test]
    fn topology_rejects_zero_duplicate_wrong_boot_and_wrong_node_bindings() {
        let boot = "11111111-1111-4111-8111-111111111111";
        assert!(
            validate_iomedia_topology_identity_against_boot(&test_topology(boot), expected(), boot)
                .is_ok()
        );

        let mut zero = test_topology(boot);
        zero.physical_whole.registry_entry_id = "0000000000000000".to_string();
        assert!(validate_iomedia_topology_identity_against_boot(&zero, expected(), boot).is_err());

        let mut duplicate = test_topology(boot);
        duplicate.apfs_volume.registry_entry_id =
            duplicate.apfs_container.registry_entry_id.clone();
        assert!(
            validate_iomedia_topology_identity_against_boot(&duplicate, expected(), boot).is_err()
        );

        let wrong_boot = "22222222-2222-4222-8222-222222222222";
        assert!(
            validate_iomedia_topology_identity_against_boot(
                &test_topology(boot),
                expected(),
                wrong_boot,
            )
            .is_err()
        );

        let mut wrong_name = test_topology(boot);
        wrong_name.physical_store.bsd_name = "disk11s1".to_string();
        assert!(
            validate_iomedia_topology_identity_against_boot(&wrong_name, expected(), boot).is_err()
        );

        let mut wrong_role = test_topology(boot);
        std::mem::swap(&mut wrong_role.physical_store, &mut wrong_role.apfs_volume);
        assert!(
            validate_iomedia_topology_identity_against_boot(&wrong_role, expected(), boot).is_err()
        );
    }

    #[test]
    fn boot_uuid_and_bsd_name_shapes_are_closed_world() {
        assert!(!valid_uuid("00000000-0000-0000-0000-000000000000"));
        for valid in ["disk0", "disk1", "disk1s1", "disk12s34"] {
            assert!(valid_bsd_name(valid), "{valid}");
        }
        for invalid in [
            "disk1s",
            "disk01",
            "disk01s1",
            "disk1s00",
            "disk1s0",
            "disk1s1s2",
        ] {
            assert!(!valid_bsd_name(invalid), "{invalid}");
        }
    }

    #[test]
    fn registry_entry_ids_are_canonical_strings_not_json_numbers() {
        let boot = "11111111-1111-4111-8111-111111111111";
        let topology = test_topology(boot);
        let encoded = serde_json::to_string(&topology).expect("serialize topology identity");
        assert!(encoded.contains(r#""registry_entry_id":"0000000000000065""#));
        assert!(!encoded.contains(r#""registry_entry_id":101"#));

        let mut short = topology.clone();
        short.physical_whole.registry_entry_id = "65".to_string();
        assert!(validate_iomedia_topology_identity_shape(&short, expected()).is_err());
        let mut uppercase = topology;
        uppercase.physical_whole.registry_entry_id = "00000000000000AF".to_string();
        assert!(validate_iomedia_topology_identity_shape(&uppercase, expected()).is_err());
    }

    #[test]
    fn strict_disk_image_url_parser_rejects_authority_and_decoding_ambiguity() {
        assert_eq!(
            strict_file_url_path("file:///Library/A%20B.dmg").expect("canonical file URL"),
            "/Library/A B.dmg"
        );
        for invalid_url in [
            "https:///Library/image.dmg",
            "file://host/Library/image.dmg",
            "file:///Library/image.dmg?query",
            "file:///Library/image.dmg#fragment",
            "file:///Library/%2fimage.dmg",
            "file:///Library/%2Fimage.dmg",
            "file:///Library/%69mage.dmg",
            "file:///Library/%25.dmg",
            "file:///Library/%252F.dmg",
            "file:///Library/%ZZ.dmg",
            "file:///Library/../image.dmg",
            "file:///Library/image.dmg/",
        ] {
            assert!(strict_file_url_path(invalid_url).is_err(), "{invalid_url}");
        }
    }

    #[test]
    fn v2_provenance_accepts_role_exact_apple_apfs_volume_and_rejects_drift() {
        let topology = test_v2_topology();
        assert_eq!(
            topology.apfs_volume.ancestry[0].class_name,
            "AppleAPFSVolume"
        );
        assert!(validate_iomedia_topology_provenance_shape(&topology, expected()).is_ok());
        assert_eq!(
            project_iomedia_identity_v1(&topology).expect("v1 projection"),
            test_topology("11111111-1111-4111-8111-111111111111")
        );

        let mut wrong_volume_class = topology.clone();
        wrong_volume_class.apfs_volume.ancestry[0].class_name = "IOMedia".to_string();
        assert!(
            validate_iomedia_topology_provenance_shape(&wrong_volume_class, expected()).is_err()
        );

        let mut missing_required_property = topology.clone();
        missing_required_property.apfs_volume.iomedia.leaf = None;
        assert!(
            validate_iomedia_topology_provenance_shape(&missing_required_property, expected())
                .is_err()
        );

        let mut source_disagreement = topology.clone();
        source_disagreement.apfs_volume.disk_arbitration.writable = Some(false);
        assert!(
            validate_iomedia_topology_provenance_shape(&source_disagreement, expected()).is_err()
        );

        let mut internal = topology.clone();
        internal.physical_whole.disk_arbitration.internal = Some(true);
        assert!(validate_iomedia_topology_provenance_shape(&internal, expected()).is_err());

        let mut forked_suffix = topology.clone();
        forked_suffix
            .apfs_volume
            .ancestry
            .push(test_ancestor("IOService", 9999));
        assert!(validate_iomedia_topology_provenance_shape(&forked_suffix, expected()).is_err());

        let mut inserted_suffix = topology.clone();
        inserted_suffix
            .apfs_container
            .ancestry
            .insert(3, test_ancestor("IOService", 9998));
        assert!(validate_iomedia_topology_provenance_shape(&inserted_suffix, expected()).is_err());

        let mut preexisting_alias = topology.clone();
        preexisting_alias
            .pre_attach_inventory
            .all_registry_entry_ids
            .push(preexisting_alias.physical_whole.registry_entry_id.clone());
        preexisting_alias
            .pre_attach_inventory
            .all_registry_entry_ids
            .sort();
        assert!(
            validate_iomedia_topology_provenance_shape(&preexisting_alias, expected()).is_err()
        );

        let mut wrong_t5_uuid = topology.clone();
        wrong_t5_uuid.pre_attach_inventory.t5_volume_uuid =
            "44444444-4444-4444-8444-444444444444".to_string();
        assert!(validate_iomedia_topology_provenance_shape(&wrong_t5_uuid, expected()).is_err());
    }

    #[test]
    fn raw_disk_image_candidates_and_backing_triplet_fail_closed() {
        let topology = test_v2_topology();
        let candidate = DiskImageCandidateObservation {
            device: topology.backing.disk_image_device.clone(),
            url: topology.backing.disk_image_url.clone(),
        };
        let observations = (0..4)
            .map(|_| (1, vec![candidate.clone()]))
            .collect::<Vec<_>>();
        assert_eq!(
            select_unique_disk_image_candidate(&observations).expect("unique raw candidate"),
            candidate
        );

        let mut missing = observations.clone();
        missing[0] = (1, Vec::new());
        assert!(select_unique_disk_image_candidate(&missing).is_err());

        let mut multiple = observations.clone();
        multiple[0] = (2, vec![candidate.clone(), candidate.clone()]);
        assert!(select_unique_disk_image_candidate(&multiple).is_err());

        let mut distinct = observations;
        distinct[3].1[0].url = "file:///Library/other.dmg".to_string();
        assert!(select_unique_disk_image_candidate(&distinct).is_err());

        let mut path_drift = topology.clone();
        path_drift.backing.opened_components[2]
            .path_binding_after
            .inode += 1;
        assert!(validate_iomedia_topology_provenance_shape(&path_drift, expected()).is_err());

        let mut missing_content_digest = topology.clone();
        missing_content_digest.backing.opened_components[2]
            .fd_binding
            .content_sha256 = None;
        assert!(
            validate_iomedia_topology_provenance_shape(&missing_content_digest, expected())
                .is_err()
        );

        let mut metadata_drift = topology.clone();
        metadata_drift.backing.opened_components[2]
            .path_binding_after
            .mtime_nanoseconds += 1;
        assert!(validate_iomedia_topology_provenance_shape(&metadata_drift, expected()).is_err());

        let mut digest_drift = topology.clone();
        digest_drift.backing.opened_components[2]
            .path_binding_after
            .content_sha256 = Some("22".repeat(32));
        assert!(validate_iomedia_topology_provenance_shape(&digest_drift, expected()).is_err());

        let mut reported_ambiguity = topology.clone();
        reported_ambiguity.backing.disk_image_url_ancestor_count = 2;
        assert!(
            validate_iomedia_topology_provenance_shape(&reported_ambiguity, expected()).is_err()
        );

        let mut authority = topology;
        authority.backing.path_authority_granted = true;
        assert!(validate_iomedia_topology_provenance_shape(&authority, expected()).is_err());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn rootless_live_iomedia_enumeration_and_registry_id_replay_is_authority_free() {
        let boot = current_boot_session_uuid().expect("current boot session UUID");
        let identities = enumerate_iomedia_registry_identities()
            .expect("read-only IOMedia enumeration must be available");
        assert!(identities.len() >= 4);
        let mut ids = BTreeSet::new();
        for identity in identities {
            assert!(!identity.authority_granted);
            assert!(ids.insert(identity.registry_entry_id.clone()));
            let object = resolve_iomedia_registry_identity(
                identity.registry_entry_id.as_str(),
                &boot,
                &identity.bsd_name,
            )
            .expect("registry ID must replay through DADiskRef");
            assert_eq!(object.identity(), &identity);
            assert!(!object.identity().authority_granted);
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn live_resolution_rejects_not_found_wrong_boot_and_reused_name_mismatch() {
        let boot = current_boot_session_uuid().expect("current boot session UUID");
        let identities = enumerate_iomedia_registry_identities()
            .expect("read-only IOMedia enumeration must be available");
        let first = identities.first().expect("at least one IOMedia object");

        assert!(
            resolve_iomedia_registry_identity("ffffffffffffffff", &boot, &first.bsd_name).is_err()
        );
        assert!(
            resolve_iomedia_registry_identity(
                first.registry_entry_id.as_str(),
                "22222222-2222-4222-8222-222222222222",
                &first.bsd_name,
            )
            .is_err()
        );
        let forged_name = if first.bsd_name == "disk999" {
            "disk998"
        } else {
            "disk999"
        };
        assert!(
            resolve_iomedia_registry_identity(
                first.registry_entry_id.as_str(),
                &boot,
                forged_name,
            )
            .is_err()
        );
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn non_macos_identity_operations_fail_closed_as_unsupported() {
        assert!(current_boot_session_uuid().is_err());
        assert!(enumerate_iomedia_registry_identities().is_err());
        assert!(resolve_iomedia_registry_identity("0000000000000001", "x", "disk1").is_err());
        assert!(capture_attached_iomedia_topology(expected()).is_err());
    }
}
