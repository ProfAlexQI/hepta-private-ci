//! Revision-8 frozen-tool sealing and publication.
//!
//! This module deliberately keeps a directory descriptor open from the first
//! observation through inventory generation or publication. Every descendant
//! is reopened with `openat(O_NOFOLLOW)`, typed with `fstatat`, and compared to
//! the opened inode. Publication uses a same-parent no-replace rename and then
//! replays the complete tree from the published descriptor. The sibling
//! publication result is useful only when its own digest is pinned outside the
//! published directory.

use std::collections::BTreeMap;
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
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::AcceptanceError;
use crate::durable::MAX_ARTIFACT_BYTES;
use crate::durable::canonical_json;
use crate::durable::secure_root;
use crate::durable::sha256;

const SHA256SUMS: &str = "SHA256SUMS";
const MODES: &str = "MODES.tsv";
const METADATA: &str = "METADATA.tsv";
const ACL: &str = "ACL.tsv";
const XATTRS: &str = "XATTRS.tsv";
const INVENTORIES: [&str; 5] = [SHA256SUMS, MODES, METADATA, ACL, XATTRS];
const ACCEPTANCE_BINARY: &str = "bin/hepta-operator-acceptance-v3";
const PUBLISHER_BINARY: &str = "bin/hepta-operator-acceptance-freeze-tool-v1";
const SOURCE_IDENTITY: &str = "SOURCE-IDENTITY.json";
const MAX_NODES: usize = 8_192;
const MAX_DEPTH: usize = 32;
const MAX_FIXED_POINT_ATTEMPTS: usize = 16;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenToolSourceIdentityV1 {
    pub head: String,
    pub parent: String,
    pub remote_tracking: String,
    pub schema: String,
    pub schema_version: u32,
    pub tree: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InventoryIdentityV1 {
    pub rows: usize,
    pub sha256: String,
    pub size_bytes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedStagingV1 {
    pub acl: InventoryIdentityV1,
    pub metadata: InventoryIdentityV1,
    pub modes: InventoryIdentityV1,
    pub root_dev: u64,
    pub root_inode: u64,
    pub root_name: String,
    pub schema: String,
    pub sha256sums: InventoryIdentityV1,
    pub xattrs: InventoryIdentityV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationResultV1 {
    pub acceptance_binary_sha256: String,
    pub acceptance_binary_size_bytes: u64,
    pub acl_rows: usize,
    pub acl_sha256: String,
    pub automatic_transition: bool,
    pub local_remote_tracking_only: bool,
    pub metadata_rows: usize,
    pub metadata_sha256: String,
    pub modes_rows: usize,
    pub modes_sha256: String,
    pub parent_post_dev: u64,
    pub parent_post_inode: u64,
    pub parent_pre_dev: u64,
    pub parent_pre_inode: u64,
    pub production_authority: bool,
    pub promotion_authority: bool,
    pub publication_result_name: String,
    pub published_post_dev: u64,
    pub published_post_inode: u64,
    pub published_root_name: String,
    pub publisher_binary_sha256: String,
    pub publisher_binary_size_bytes: u64,
    pub refs_authority: bool,
    pub rename_no_replace: bool,
    pub rename_rc: i32,
    pub schema: String,
    pub schema_version: u32,
    pub sha256sums_rows: usize,
    pub sha256sums_sha256: String,
    pub source_head: String,
    pub source_identity_sha256: String,
    pub source_parent: String,
    pub source_tree: String,
    pub staging_pre_dev: u64,
    pub staging_pre_inode: u64,
    pub tamper_evident_requires_external_pin: bool,
    pub xattrs_rows: usize,
    pub xattrs_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublishedFrozenToolV1 {
    pub publication_result: PublicationResultV1,
    pub publication_result_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Snapshot {
    dev: u64,
    digest: Option<String>,
    gid: u32,
    inode: u64,
    kind: NodeKind,
    mode: u32,
    nlink: u64,
    path: String,
    size: u64,
    uid: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NodeKind {
    Directory,
    RegularFile,
}

struct GeneratedInventories {
    acl: Vec<u8>,
    metadata: Vec<u8>,
    modes: Vec<u8>,
    sha256sums: Vec<u8>,
    xattrs: Vec<u8>,
}

impl GeneratedInventories {
    fn get(&self, name: &str) -> &[u8] {
        match name {
            SHA256SUMS => &self.sha256sums,
            MODES => &self.modes,
            METADATA => &self.metadata,
            ACL => &self.acl,
            XATTRS => &self.xattrs,
            _ => unreachable!("fixed inventory name"),
        }
    }
}

pub fn run_cli(arguments: Vec<OsString>) -> Result<String, String> {
    const USAGE: &str = "usage:\n  hepta-operator-acceptance-freeze-tool-v1 seal --execute <absolute-tools-parent> <incoming-name>\n  hepta-operator-acceptance-freeze-tool-v1 publish --execute <absolute-tools-parent> <incoming-name> <final-name> <SHA256SUMS-sha256> <MODES-sha256> <METADATA-sha256> <ACL-sha256> <XATTRS-sha256>";
    if arguments.len() == 2
        && matches!(
            arguments.get(1).and_then(|value| value.to_str()),
            Some("--help" | "-h" | "help")
        )
    {
        return Ok(USAGE.to_string());
    }
    let command = arguments
        .get(1)
        .and_then(|value| value.to_str())
        .ok_or_else(|| USAGE.to_string())?;
    let result = match command {
        "seal"
            if arguments.len() == 5
                && arguments.get(2).and_then(|value| value.to_str()) == Some("--execute") =>
        {
            let parent = Path::new(&arguments[3]);
            let incoming = utf8(&arguments[4], "incoming name")?;
            serde_json::to_string(&seal_staging(parent, incoming).map_err(|e| e.to_string())?)
        }
        "publish"
            if arguments.len() == 11
                && arguments.get(2).and_then(|value| value.to_str()) == Some("--execute") =>
        {
            let parent = Path::new(&arguments[3]);
            let incoming = utf8(&arguments[4], "incoming name")?;
            let final_name = utf8(&arguments[5], "final name")?;
            let expected = INVENTORIES
                .iter()
                .zip(arguments[6..].iter())
                .map(|(name, digest)| Ok((*name, utf8(digest, "inventory digest")?.to_string())))
                .collect::<Result<BTreeMap<_, _>, String>>()?;
            serde_json::to_string(
                &publish_staging(parent, incoming, final_name, &expected)
                    .map_err(|e| e.to_string())?,
            )
        }
        _ => return Err(USAGE.to_string()),
    }
    .map_err(|error| error.to_string())?;
    Ok(result)
}

fn utf8<'a>(value: &'a OsString, label: &str) -> Result<&'a str, String> {
    value
        .to_str()
        .ok_or_else(|| format!("{label} must be UTF-8"))
}

pub fn seal_staging(
    parent: &Path,
    incoming_name: &str,
) -> Result<SealedStagingV1, AcceptanceError> {
    require_safe_name(incoming_name, true)?;
    let parent = secure_root(parent, "frozen-tool parent")?;
    let parent_fd = open_absolute_directory(&parent)?;
    let root = openat_directory(parent_fd.as_raw_fd(), incoming_name)?;
    let initial = snapshot_fd(root.as_raw_fd(), ".", None)?;
    if initial.mode != 0o700 {
        return Err(invalid(
            "incoming frozen-tool root must start mode 0700 before sealing",
        ));
    }

    let mut inventory_files = BTreeMap::new();
    for name in INVENTORIES {
        inventory_files.insert(name, createat_file(root.as_raw_fd(), name, 0o600)?);
    }
    normalize_modes(root.as_raw_fd(), ".", 0)?;

    let mut converged = false;
    for _ in 0..MAX_FIXED_POINT_ATTEMPTS {
        let before = scan_tree(root.as_raw_fd())?;
        let generated = generate_inventories(&before)?;
        for name in INVENTORIES {
            let file = inventory_files
                .get_mut(name)
                .ok_or_else(|| invalid("inventory descriptor is absent"))?;
            let current = read_file(file)?;
            if current != generated.get(name) {
                rewrite_open_file(file, generated.get(name))?;
            }
        }
        for file in inventory_files.values() {
            file.sync_all()?;
        }
        let after = scan_tree(root.as_raw_fd())?;
        let expected = generate_inventories(&after)?;
        let exact = INVENTORIES.iter().all(|name| {
            inventory_files
                .get(name)
                .and_then(|file| read_file_ref(file).ok())
                .is_some_and(|bytes| bytes == expected.get(name))
        });
        if exact {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(invalid(
            "frozen-tool inventories did not reach a joint fixed point",
        ));
    }

    sync_tree(root.as_raw_fd(), 0)?;
    root.sync_all()?;
    parent_fd.sync_all()?;
    let first = verify_sealed_tree(root.as_raw_fd())?;
    let second = verify_sealed_tree(root.as_raw_fd())?;
    if first != second {
        return Err(invalid(
            "frozen-tool staging changed across terminal descriptor replay",
        ));
    }
    let root_snapshot = snapshot_fd(root.as_raw_fd(), ".", None)?;
    Ok(SealedStagingV1 {
        acl: inventory_identity(root.as_raw_fd(), ACL)?,
        metadata: inventory_identity(root.as_raw_fd(), METADATA)?,
        modes: inventory_identity(root.as_raw_fd(), MODES)?,
        root_dev: root_snapshot.dev,
        root_inode: root_snapshot.inode,
        root_name: incoming_name.to_string(),
        schema: "hepta_operator_acceptance_frozen_tool_staging_v1".to_string(),
        sha256sums: inventory_identity(root.as_raw_fd(), SHA256SUMS)?,
        xattrs: inventory_identity(root.as_raw_fd(), XATTRS)?,
    })
}

pub fn publish_staging(
    parent: &Path,
    incoming_name: &str,
    final_name: &str,
    expected_digests: &BTreeMap<&str, String>,
) -> Result<PublishedFrozenToolV1, AcceptanceError> {
    require_safe_name(incoming_name, true)?;
    require_safe_name(final_name, false)?;
    require_expected_digests(expected_digests)?;
    let parent = secure_root(parent, "frozen-tool parent")?;
    let parent_fd = open_absolute_directory(&parent)?;
    let parent_pre = snapshot_fd(parent_fd.as_raw_fd(), ".", None)?;
    let root = openat_directory(parent_fd.as_raw_fd(), incoming_name)?;
    let staging_pre = snapshot_fd(root.as_raw_fd(), ".", None)?;
    let inventories_pre = verify_sealed_tree(root.as_raw_fd())?;
    for name in INVENTORIES {
        if expected_digests.get(name).map(String::as_str)
            != Some(
                inventories_pre
                    .get(name)
                    .expect("fixed inventory")
                    .sha256
                    .as_str(),
            )
        {
            return Err(invalid(format!(
                "sealed staging {name} differs from its external pin"
            )));
        }
    }
    let source_bytes = read_relative_file(root.as_raw_fd(), SOURCE_IDENTITY)?;
    let source: FrozenToolSourceIdentityV1 = strict_source_identity(&source_bytes)?;
    let acceptance_binary = read_relative_file(root.as_raw_fd(), ACCEPTANCE_BINARY)?;
    let publisher_binary = read_relative_file(root.as_raw_fd(), PUBLISHER_BINARY)?;
    let source_identity_sha256 = sha256(&source_bytes);
    let acceptance_binary_sha256 = sha256(&acceptance_binary);
    let publisher_binary_sha256 = sha256(&publisher_binary);

    // Reserve the sibling result path before the tree can become visible at its
    // final name. Holding this descriptor across the rename prevents a same-UID
    // contender from installing a forged result between publication and result
    // creation. A crash can leave an empty reservation, which fails closed and
    // permanently blocks reuse of this final identity.
    let result_name = format!("{final_name}.PUBLICATION-RESULT.json");
    require_safe_name(&result_name, false)?;
    let mut result_file = createat_file(parent_fd.as_raw_fd(), &result_name, 0o400)?;
    parent_fd.sync_all()?;

    let rename_rc = rename_noreplace(
        parent_fd.as_raw_fd(),
        incoming_name,
        parent_fd.as_raw_fd(),
        final_name,
    )?;
    parent_fd.sync_all()?;
    let published = openat_directory(parent_fd.as_raw_fd(), final_name)?;
    let published_post = snapshot_fd(published.as_raw_fd(), ".", None)?;
    if staging_pre.dev != published_post.dev || staging_pre.inode != published_post.inode {
        return Err(invalid(
            "published frozen-tool root is not the exact staged inode",
        ));
    }
    if read_relative_file(published.as_raw_fd(), SOURCE_IDENTITY)? != source_bytes
        || read_relative_file(published.as_raw_fd(), ACCEPTANCE_BINARY)? != acceptance_binary
        || read_relative_file(published.as_raw_fd(), PUBLISHER_BINARY)? != publisher_binary
    {
        return Err(invalid(
            "published source or binary differs from its pre-publication descriptor read",
        ));
    }
    let inventories_post = verify_sealed_tree(published.as_raw_fd())?;
    if inventories_post != inventories_pre {
        return Err(invalid(
            "published frozen-tool tree differs from the sealed staging replay",
        ));
    }
    let parent_post = snapshot_fd(parent_fd.as_raw_fd(), ".", None)?;
    if !same_identity(&parent_pre, &parent_post) {
        return Err(invalid(
            "frozen-tool parent inode changed during publication",
        ));
    }

    let identity = |name| inventories_post.get(name).expect("fixed inventory");
    let result = PublicationResultV1 {
        acceptance_binary_sha256,
        acceptance_binary_size_bytes: acceptance_binary.len() as u64,
        acl_rows: identity(ACL).rows,
        acl_sha256: identity(ACL).sha256.clone(),
        automatic_transition: false,
        local_remote_tracking_only: true,
        metadata_rows: identity(METADATA).rows,
        metadata_sha256: identity(METADATA).sha256.clone(),
        modes_rows: identity(MODES).rows,
        modes_sha256: identity(MODES).sha256.clone(),
        parent_post_dev: parent_post.dev,
        parent_post_inode: parent_post.inode,
        parent_pre_dev: parent_pre.dev,
        parent_pre_inode: parent_pre.inode,
        production_authority: false,
        promotion_authority: false,
        publication_result_name: result_name.clone(),
        published_post_dev: published_post.dev,
        published_post_inode: published_post.inode,
        published_root_name: final_name.to_string(),
        publisher_binary_sha256,
        publisher_binary_size_bytes: publisher_binary.len() as u64,
        refs_authority: false,
        rename_no_replace: true,
        rename_rc,
        schema: "hepta_operator_acceptance_frozen_tool_publication_result_v1".to_string(),
        schema_version: 1,
        sha256sums_rows: identity(SHA256SUMS).rows,
        sha256sums_sha256: identity(SHA256SUMS).sha256.clone(),
        source_head: source.head,
        source_identity_sha256,
        source_parent: source.parent,
        source_tree: source.tree,
        staging_pre_dev: staging_pre.dev,
        staging_pre_inode: staging_pre.inode,
        tamper_evident_requires_external_pin: true,
        xattrs_rows: identity(XATTRS).rows,
        xattrs_sha256: identity(XATTRS).sha256.clone(),
    };
    let result_bytes = canonical_json(&result)?;
    result_file.write_all(&result_bytes)?;
    result_file.sync_all()?;
    let result_stat = fstatat_snapshot(parent_fd.as_raw_fd(), &result_name)?;
    let result_fd_stat = snapshot_fd(result_file.as_raw_fd(), &result_name, None)?;
    if !same_node(&result_stat, &result_fd_stat) {
        return Err(invalid(
            "publication-result path differs from its no-replace descriptor",
        ));
    }
    parent_fd.sync_all()?;
    let published_replay = verify_sealed_tree(published.as_raw_fd())?;
    if published_replay != inventories_post {
        return Err(invalid(
            "published frozen-tool tree changed while the result was committed",
        ));
    }
    Ok(PublishedFrozenToolV1 {
        publication_result: result,
        publication_result_sha256: sha256(&result_bytes),
    })
}

fn require_safe_name(name: &str, incoming: bool) -> Result<(), AcceptanceError> {
    let expected_prefix = if incoming { ".incoming-" } else { "hepta-" };
    if !name.starts_with(expected_prefix)
        || name.len() > 240
        || name.contains('/')
        || name.contains('\0')
        || name == "."
        || name == ".."
    {
        return Err(invalid("frozen-tool root name is not a safe fixed child"));
    }
    Ok(())
}

fn require_expected_digests(expected: &BTreeMap<&str, String>) -> Result<(), AcceptanceError> {
    if expected.len() != INVENTORIES.len() {
        return Err(invalid("publication requires exactly five inventory pins"));
    }
    for name in INVENTORIES {
        let digest = expected
            .get(name)
            .ok_or_else(|| invalid(format!("publication omits {name} pin")))?;
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        {
            return Err(invalid(format!("publication {name} pin is malformed")));
        }
    }
    Ok(())
}

fn inventory_identity(root_fd: RawFd, name: &str) -> Result<InventoryIdentityV1, AcceptanceError> {
    let bytes = read_relative_file(root_fd, name)?;
    Ok(InventoryIdentityV1 {
        rows: line_count(&bytes)?,
        sha256: sha256(&bytes),
        size_bytes: bytes.len(),
    })
}

fn line_count(bytes: &[u8]) -> Result<usize, AcceptanceError> {
    if bytes.is_empty() || !bytes.ends_with(b"\n") || bytes.contains(&b'\r') {
        return Err(invalid(
            "frozen-tool inventory must be nonempty canonical LF text",
        ));
    }
    Ok(bytes.iter().filter(|byte| **byte == b'\n').count())
}

fn strict_source_identity(bytes: &[u8]) -> Result<FrozenToolSourceIdentityV1, AcceptanceError> {
    let value: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|error| invalid(format!("source identity is invalid JSON: {error}")))?;
    let source: FrozenToolSourceIdentityV1 = serde_json::from_value(value)
        .map_err(|error| invalid(format!("source identity is malformed: {error}")))?;
    if canonical_json(&source)? != bytes
        || source.schema != "hepta_operator_acceptance_frozen_tool_source_v1"
        || source.schema_version != 1
        || source.remote_tracking != "local_remote_tracking_only"
        || !git_oid(&source.head)
        || !git_oid(&source.tree)
        || !git_oid(&source.parent)
    {
        return Err(invalid(
            "source identity differs from the frozen-tool local tracking contract",
        ));
    }
    Ok(source)
}

fn git_oid(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn generate_inventories(
    nodes: &BTreeMap<String, Snapshot>,
) -> Result<GeneratedInventories, AcceptanceError> {
    for required in INVENTORIES {
        let node = nodes
            .get(required)
            .ok_or_else(|| invalid(format!("frozen-tool inventory file is absent: {required}")))?;
        if node.kind != NodeKind::RegularFile {
            return Err(invalid("frozen-tool inventory path is not a regular file"));
        }
    }

    let acl = absence_inventory(nodes, "acl")?;
    let xattrs = absence_inventory(nodes, "xattrs")?;
    let mut predicted = nodes.clone();
    update_predicted_file(&mut predicted, ACL, &acl)?;
    update_predicted_file(&mut predicted, XATTRS, &xattrs)?;

    let manifest_size = predicted
        .values()
        .filter(|node| node.kind == NodeKind::RegularFile && node.path != SHA256SUMS)
        .map(|node| 64 + 2 + node.path.len() + 1)
        .sum::<usize>() as u64;
    predicted
        .get_mut(SHA256SUMS)
        .expect("required manifest")
        .size = manifest_size;

    let mut modes = Vec::new();
    let mut metadata = Vec::new();
    for _ in 0..MAX_FIXED_POINT_ATTEMPTS {
        modes = modes_bytes(&predicted)?;
        predicted.get_mut(MODES).expect("required modes").size = modes.len() as u64;
        metadata = metadata_bytes(&predicted)?;
        predicted.get_mut(METADATA).expect("required metadata").size = metadata.len() as u64;
        let next_modes = modes_bytes(&predicted)?;
        let next_metadata = metadata_bytes(&predicted)?;
        if next_modes == modes && next_metadata == metadata {
            modes = next_modes;
            metadata = next_metadata;
            break;
        }
        modes = next_modes;
        metadata = next_metadata;
    }
    if modes.len() as u64 != predicted[MODES].size
        || metadata.len() as u64 != predicted[METADATA].size
    {
        return Err(invalid(
            "frozen-tool mode/metadata inventory sizes did not converge",
        ));
    }

    update_predicted_file(&mut predicted, MODES, &modes)?;
    update_predicted_file(&mut predicted, METADATA, &metadata)?;
    let mut sha256sums = Vec::new();
    for node in predicted.values() {
        if node.kind != NodeKind::RegularFile || node.path == SHA256SUMS {
            continue;
        }
        let digest = match node.path.as_str() {
            MODES => sha256(&modes),
            METADATA => sha256(&metadata),
            ACL => sha256(&acl),
            XATTRS => sha256(&xattrs),
            _ => node
                .digest
                .as_deref()
                .ok_or_else(|| invalid("regular file digest is absent"))?
                .to_string(),
        };
        writeln!(&mut sha256sums, "{digest}  {}", node.path)?;
    }
    if sha256sums.len() as u64 != predicted[SHA256SUMS].size {
        return Err(invalid("frozen-tool SHA256SUMS predicted size drifted"));
    }
    Ok(GeneratedInventories {
        acl,
        metadata,
        modes,
        sha256sums,
        xattrs,
    })
}

fn update_predicted_file(
    nodes: &mut BTreeMap<String, Snapshot>,
    name: &str,
    bytes: &[u8],
) -> Result<(), AcceptanceError> {
    let node = nodes
        .get_mut(name)
        .ok_or_else(|| invalid(format!("inventory node is absent: {name}")))?;
    node.size = bytes.len() as u64;
    node.digest = Some(sha256(bytes));
    Ok(())
}

fn absence_inventory(
    nodes: &BTreeMap<String, Snapshot>,
    label: &str,
) -> Result<Vec<u8>, AcceptanceError> {
    let mut bytes = Vec::new();
    for path in nodes.keys() {
        validate_tsv_path(path)?;
        writeln!(&mut bytes, "{path}\tnone")?;
    }
    if label != "acl" && label != "xattrs" {
        return Err(invalid("unknown absence inventory"));
    }
    Ok(bytes)
}

fn modes_bytes(nodes: &BTreeMap<String, Snapshot>) -> Result<Vec<u8>, AcceptanceError> {
    let mut bytes = Vec::new();
    for node in nodes.values() {
        validate_tsv_path(&node.path)?;
        let size = match node.kind {
            NodeKind::Directory => "-".to_string(),
            NodeKind::RegularFile => node.size.to_string(),
        };
        writeln!(
            &mut bytes,
            "{}\t{:03o}\t{size}\t{}",
            node.kind.label(),
            node.mode,
            display_path(&node.path)
        )?;
    }
    Ok(bytes)
}

fn metadata_bytes(nodes: &BTreeMap<String, Snapshot>) -> Result<Vec<u8>, AcceptanceError> {
    let mut bytes = Vec::new();
    for node in nodes.values() {
        validate_tsv_path(&node.path)?;
        writeln!(
            &mut bytes,
            "{}\t{}\t{}\t{}\t{}\t{}\t{:03o}\t{}\t{}",
            node.kind.label(),
            node.dev,
            node.inode,
            node.uid,
            node.gid,
            node.nlink,
            node.mode,
            node.size,
            display_path(&node.path)
        )?;
    }
    Ok(bytes)
}

fn display_path(path: &str) -> String {
    if path == "." {
        ".".to_string()
    } else {
        format!("./{path}")
    }
}

fn validate_tsv_path(path: &str) -> Result<(), AcceptanceError> {
    if path.is_empty()
        || path.contains('\t')
        || path.contains('\n')
        || path.contains('\r')
        || path.contains('\0')
        || (path != "."
            && path
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == ".."))
    {
        return Err(invalid("frozen-tool inventory path is unsafe"));
    }
    Ok(())
}

impl NodeKind {
    fn label(self) -> &'static str {
        match self {
            Self::Directory => "Directory",
            Self::RegularFile => "Regular File",
        }
    }
}

fn verify_sealed_tree(
    root_fd: RawFd,
) -> Result<BTreeMap<&'static str, InventoryIdentityV1>, AcceptanceError> {
    let first = scan_tree(root_fd)?;
    let generated = generate_inventories(&first)?;
    let mut identities = BTreeMap::new();
    for name in INVENTORIES {
        let actual = read_relative_file(root_fd, name)?;
        if actual != generated.get(name) {
            return Err(invalid(format!(
                "frozen-tool {name} differs from descriptor-derived inventory"
            )));
        }
        identities.insert(
            name,
            InventoryIdentityV1 {
                rows: line_count(&actual)?,
                sha256: sha256(&actual),
                size_bytes: actual.len(),
            },
        );
    }
    validate_sealed_modes(&first)?;
    let second = scan_tree(root_fd)?;
    if first != second {
        return Err(invalid(
            "frozen-tool tree changed across complete descriptor replay",
        ));
    }
    Ok(identities)
}

fn validate_sealed_modes(nodes: &BTreeMap<String, Snapshot>) -> Result<(), AcceptanceError> {
    for node in nodes.values() {
        let expected = match node.kind {
            NodeKind::Directory => 0o500,
            NodeKind::RegularFile
                if matches!(node.path.as_str(), ACCEPTANCE_BINARY | PUBLISHER_BINARY) =>
            {
                0o500
            }
            NodeKind::RegularFile => 0o400,
        };
        if node.mode != expected || (node.kind == NodeKind::RegularFile && node.nlink != 1) {
            return Err(invalid(format!(
                "frozen-tool node mode/topology differs: {}",
                node.path
            )));
        }
    }
    Ok(())
}

fn scan_tree(root_fd: RawFd) -> Result<BTreeMap<String, Snapshot>, AcceptanceError> {
    let mut nodes = BTreeMap::new();
    let root = snapshot_fd(root_fd, ".", None)?;
    if root.kind != NodeKind::Directory {
        return Err(invalid("frozen-tool root descriptor is not a directory"));
    }
    verify_no_extended_metadata(root_fd)?;
    nodes.insert(".".to_string(), root);
    scan_directory(root_fd, "", 0, &mut nodes)?;
    if nodes.len() > MAX_NODES {
        return Err(invalid("frozen-tool tree exceeds the node bound"));
    }
    Ok(nodes)
}

fn scan_directory(
    directory_fd: RawFd,
    relative_directory: &str,
    depth: usize,
    nodes: &mut BTreeMap<String, Snapshot>,
) -> Result<(), AcceptanceError> {
    if depth > MAX_DEPTH {
        return Err(invalid("frozen-tool tree exceeds the depth bound"));
    }
    for name in read_dir_names(directory_fd)? {
        let name = name
            .into_string()
            .map_err(|_| invalid("frozen-tool entry name is not UTF-8"))?;
        validate_component(&name)?;
        let relative = if relative_directory.is_empty() || relative_directory == "." {
            name.clone()
        } else {
            format!("{relative_directory}/{name}")
        };
        validate_tsv_path(&relative)?;
        let path_snapshot = fstatat_snapshot(directory_fd, &name)?;
        let opened = match path_snapshot.kind {
            NodeKind::Directory => openat_directory(directory_fd, &name)?,
            NodeKind::RegularFile => openat_regular(directory_fd, &name, libc::O_RDONLY)?,
        };
        let mut descriptor_snapshot = snapshot_fd(opened.as_raw_fd(), &relative, None)?;
        if !same_node(&path_snapshot, &descriptor_snapshot) {
            return Err(invalid(
                "frozen-tool path differs from its opened nofollow inode",
            ));
        }
        verify_no_extended_metadata(opened.as_raw_fd())?;
        match descriptor_snapshot.kind {
            NodeKind::Directory => {
                if nodes
                    .insert(relative.clone(), descriptor_snapshot)
                    .is_some()
                {
                    return Err(invalid("frozen-tool tree contains a duplicate path"));
                }
                scan_directory(opened.as_raw_fd(), &relative, depth + 1, nodes)?;
            }
            NodeKind::RegularFile => {
                if descriptor_snapshot.nlink != 1 {
                    return Err(invalid("frozen-tool tree contains a hardlinked file"));
                }
                descriptor_snapshot.digest = Some(hash_open_file(&opened, &descriptor_snapshot)?);
                let after_path = fstatat_snapshot(directory_fd, &name)?;
                let after_fd = snapshot_fd(opened.as_raw_fd(), &relative, None)?;
                if !same_node(&descriptor_snapshot, &after_path)
                    || !same_node(&descriptor_snapshot, &after_fd)
                {
                    return Err(invalid("frozen-tool file changed while it was hashed"));
                }
                if nodes.insert(relative, descriptor_snapshot).is_some() {
                    return Err(invalid("frozen-tool tree contains a duplicate path"));
                }
            }
        }
        if nodes.len() > MAX_NODES {
            return Err(invalid("frozen-tool tree exceeds the node bound"));
        }
    }
    Ok(())
}

fn normalize_modes(
    directory_fd: RawFd,
    relative_directory: &str,
    depth: usize,
) -> Result<(), AcceptanceError> {
    if depth > MAX_DEPTH {
        return Err(invalid("frozen-tool tree exceeds the depth bound"));
    }
    for name in read_dir_names(directory_fd)? {
        let name = name
            .into_string()
            .map_err(|_| invalid("frozen-tool entry name is not UTF-8"))?;
        validate_component(&name)?;
        let relative = if relative_directory == "." {
            name.clone()
        } else {
            format!("{relative_directory}/{name}")
        };
        let snapshot = fstatat_snapshot(directory_fd, &name)?;
        let opened = match snapshot.kind {
            NodeKind::Directory => openat_directory(directory_fd, &name)?,
            NodeKind::RegularFile => openat_regular(directory_fd, &name, libc::O_RDONLY)?,
        };
        let expected_mode = match snapshot.kind {
            NodeKind::Directory => 0o500,
            NodeKind::RegularFile
                if matches!(relative.as_str(), ACCEPTANCE_BINARY | PUBLISHER_BINARY) =>
            {
                0o500
            }
            NodeKind::RegularFile => 0o400,
        };
        fchmod_fd(opened.as_raw_fd(), expected_mode)?;
        if snapshot.kind == NodeKind::Directory {
            normalize_modes(opened.as_raw_fd(), &relative, depth + 1)?;
            opened.sync_all()?;
        }
    }
    fchmod_fd(directory_fd, 0o500)?;
    Ok(())
}

fn sync_tree(directory_fd: RawFd, depth: usize) -> Result<(), AcceptanceError> {
    if depth > MAX_DEPTH {
        return Err(invalid("frozen-tool tree exceeds the depth bound"));
    }
    for name in read_dir_names(directory_fd)? {
        let name = name
            .into_string()
            .map_err(|_| invalid("frozen-tool entry name is not UTF-8"))?;
        validate_component(&name)?;
        let snapshot = fstatat_snapshot(directory_fd, &name)?;
        let opened = match snapshot.kind {
            NodeKind::Directory => openat_directory(directory_fd, &name)?,
            NodeKind::RegularFile => openat_regular(directory_fd, &name, libc::O_RDONLY)?,
        };
        if snapshot.kind == NodeKind::Directory {
            sync_tree(opened.as_raw_fd(), depth + 1)?;
        }
        opened.sync_all()?;
    }
    Ok(())
}

fn read_relative_file(root_fd: RawFd, relative: &str) -> Result<Vec<u8>, AcceptanceError> {
    validate_tsv_path(relative)?;
    let (parents, name) = relative
        .rsplit_once('/')
        .map_or(("", relative), |(parents, name)| (parents, name));
    let mut directory = duplicate_fd(root_fd)?;
    if !parents.is_empty() {
        for component in parents.split('/') {
            directory = openat_directory(directory.as_raw_fd(), component)?;
        }
    }
    let mut file = openat_regular(directory.as_raw_fd(), name, libc::O_RDONLY)?;
    read_file(&mut file)
}

fn hash_open_file(file: &File, snapshot: &Snapshot) -> Result<String, AcceptanceError> {
    if snapshot.size > MAX_ARTIFACT_BYTES {
        return Err(invalid("frozen-tool file exceeds the 2 GiB bound"));
    }
    let bytes = read_file_ref(file)?;
    if bytes.len() as u64 != snapshot.size {
        return Err(invalid("frozen-tool file size changed while read"));
    }
    Ok(sha256(&bytes))
}

fn read_file(file: &mut File) -> Result<Vec<u8>, AcceptanceError> {
    file.seek(SeekFrom::Start(0))?;
    let size = file.metadata()?.len();
    if size > MAX_ARTIFACT_BYTES {
        return Err(invalid("frozen-tool file exceeds the 2 GiB bound"));
    }
    let mut bytes = Vec::with_capacity(size as usize);
    file.take(MAX_ARTIFACT_BYTES.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_ARTIFACT_BYTES {
        return Err(invalid("frozen-tool file exceeds the 2 GiB bound"));
    }
    Ok(bytes)
}

fn read_file_ref(file: &File) -> Result<Vec<u8>, AcceptanceError> {
    let mut copy = file.try_clone()?;
    read_file(&mut copy)
}

fn rewrite_open_file(file: &mut File, bytes: &[u8]) -> Result<(), AcceptanceError> {
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

fn open_absolute_directory(path: &Path) -> Result<File, AcceptanceError> {
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| invalid("frozen-tool parent contains NUL"))?;
    // SAFETY: path is a live C string and open returns a uniquely owned fd.
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    file_from_fd(fd)
}

fn openat_directory(parent_fd: RawFd, name: &str) -> Result<File, AcceptanceError> {
    let name = c_component(name)?;
    // SAFETY: parent_fd is open and name is a live single-component C string.
    let fd = unsafe {
        libc::openat(
            parent_fd,
            name.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    file_from_fd(fd)
}

fn openat_regular(
    parent_fd: RawFd,
    name: &str,
    access: libc::c_int,
) -> Result<File, AcceptanceError> {
    let name = c_component(name)?;
    // SAFETY: parent_fd is open and name is a live single-component C string.
    let fd = unsafe {
        libc::openat(
            parent_fd,
            name.as_ptr(),
            access | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    let file = file_from_fd(fd)?;
    let snapshot = snapshot_fd(file.as_raw_fd(), name.to_str().unwrap_or("file"), None)?;
    if snapshot.kind != NodeKind::RegularFile {
        return Err(invalid("nofollow file open did not yield a regular file"));
    }
    Ok(file)
}

fn createat_file(parent_fd: RawFd, name: &str, mode: u32) -> Result<File, AcceptanceError> {
    let name = c_component(name)?;
    // SAFETY: parent_fd is open and O_EXCL establishes no-replace creation.
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
    // SAFETY: fcntl duplicates the open descriptor and transfers ownership.
    let duplicate = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
    file_from_fd(duplicate)
}

fn file_from_fd(fd: libc::c_int) -> Result<File, AcceptanceError> {
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: fd is a fresh uniquely owned descriptor on the success path.
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn c_component(name: &str) -> Result<CString, AcceptanceError> {
    validate_component(name)?;
    CString::new(name).map_err(|_| invalid("frozen-tool component contains NUL"))
}

fn validate_component(name: &str) -> Result<(), AcceptanceError> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.contains('/')
        || name.contains('\t')
        || name.contains('\n')
        || name.contains('\r')
        || name.contains('\0')
    {
        return Err(invalid("frozen-tool path component is unsafe"));
    }
    Ok(())
}

fn snapshot_fd(
    fd: RawFd,
    path: &str,
    expected: Option<NodeKind>,
) -> Result<Snapshot, AcceptanceError> {
    // SAFETY: stat is initialized by fstat on success.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: fd is open and stat is writable.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    snapshot_from_stat(stat, path, expected)
}

fn fstatat_snapshot(parent_fd: RawFd, name: &str) -> Result<Snapshot, AcceptanceError> {
    let name = c_component(name)?;
    // SAFETY: stat is initialized by fstatat on success.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: parent_fd is open, name is live, and AT_SYMLINK_NOFOLLOW rejects
    // substituting a link for the intended node.
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
    snapshot_from_stat(stat, name.to_str().unwrap_or("node"), None)
}

fn snapshot_from_stat(
    stat: libc::stat,
    path: &str,
    expected: Option<NodeKind>,
) -> Result<Snapshot, AcceptanceError> {
    let file_type = (stat.st_mode as libc::mode_t) & libc::S_IFMT;
    let kind = if file_type == libc::S_IFDIR {
        NodeKind::Directory
    } else if file_type == libc::S_IFREG {
        NodeKind::RegularFile
    } else {
        return Err(invalid(
            "frozen-tool tree contains a symlink or special node",
        ));
    };
    if expected.is_some_and(|expected| expected != kind) || stat.st_size < 0 {
        return Err(invalid("frozen-tool node type or size changed"));
    }
    Ok(Snapshot {
        dev: stat.st_dev as u64,
        digest: None,
        gid: stat.st_gid,
        inode: stat.st_ino as u64,
        kind,
        mode: (stat.st_mode as u32) & 0o7777,
        nlink: stat.st_nlink as u64,
        path: path.to_string(),
        size: stat.st_size as u64,
        uid: stat.st_uid,
    })
}

fn same_node(left: &Snapshot, right: &Snapshot) -> bool {
    left.dev == right.dev
        && left.inode == right.inode
        && left.kind == right.kind
        && left.mode == right.mode
        && left.nlink == right.nlink
        && left.size == right.size
        && left.uid == right.uid
        && left.gid == right.gid
}

fn same_identity(left: &Snapshot, right: &Snapshot) -> bool {
    left.dev == right.dev && left.inode == right.inode && left.kind == right.kind
}

fn fchmod_fd(fd: RawFd, mode: u32) -> Result<(), AcceptanceError> {
    // SAFETY: fd is open and mode is restricted by the caller.
    if unsafe { libc::fchmod(fd, mode as libc::mode_t) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}

fn read_dir_names(fd: RawFd) -> Result<Vec<OsString>, AcceptanceError> {
    // SAFETY: dup creates a descriptor that fdopendir owns on success.
    let duplicate = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
    if duplicate < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: duplicate is a fresh directory descriptor.
    let directory = unsafe { libc::fdopendir(duplicate) };
    if directory.is_null() {
        // SAFETY: fdopendir did not take ownership on failure.
        unsafe { libc::close(duplicate) };
        return Err(std::io::Error::last_os_error().into());
    }
    // fdopendir inherits the open-file description's current offset. Every
    // traversal must replay from the beginning even when an earlier scan used
    // a dup of the same long-lived descriptor.
    // SAFETY: duplicate is owned by the live DIR stream and remains valid.
    if unsafe { libc::lseek(duplicate, 0, libc::SEEK_SET) } < 0 {
        let error = std::io::Error::last_os_error();
        // SAFETY: directory is live and owned by this function.
        unsafe { libc::closedir(directory) };
        return Err(error.into());
    }
    let mut names = Vec::new();
    loop {
        set_errno(0);
        // SAFETY: directory is live until the unconditional closedir below.
        let entry = unsafe { libc::readdir(directory) };
        if entry.is_null() {
            let error = get_errno();
            // SAFETY: directory is live and owned by this function.
            let close_rc = unsafe { libc::closedir(directory) };
            if error != 0 {
                return Err(std::io::Error::from_raw_os_error(error).into());
            }
            if close_rc != 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            break;
        }
        // SAFETY: d_name is NUL terminated for a live dirent.
        let raw = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
        if raw == b"." || raw == b".." {
            continue;
        }
        names.push(OsString::from_vec(raw.to_vec()));
    }
    names.sort();
    Ok(names)
}

#[cfg(target_os = "macos")]
fn get_errno() -> libc::c_int {
    // SAFETY: __error returns the calling thread's errno pointer.
    unsafe { *libc::__error() }
}

#[cfg(target_os = "macos")]
fn set_errno(value: libc::c_int) {
    // SAFETY: __error returns the calling thread's errno pointer.
    unsafe { *libc::__error() = value };
}

#[cfg(target_os = "macos")]
fn verify_no_extended_metadata(fd: RawFd) -> Result<(), AcceptanceError> {
    const ACL_FIRST_ENTRY: libc::c_int = 0;
    const ACL_TYPE_EXTENDED: libc::c_int = 0x0000_0100;
    // SAFETY: fd is live and a null buffer requests only the name-list size.
    let xattr_bytes = unsafe { libc::flistxattr(fd, std::ptr::null_mut(), 0, 0) };
    if xattr_bytes < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if xattr_bytes != 0 {
        return Err(invalid(
            "frozen-tool node has extended attributes; the sealed profile requires none",
        ));
    }

    // SAFETY: acl_get_fd_np returns a separately owned ACL object.
    let acl = unsafe { acl_get_fd_np(fd, ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ENOENT) {
            return Ok(());
        }
        return Err(error.into());
    }
    let mut entry = std::ptr::null_mut();
    // SAFETY: acl is live and entry is writable.
    let entry_rc = unsafe { acl_get_entry(acl, ACL_FIRST_ENTRY, &mut entry) };
    let entry_error = std::io::Error::last_os_error();
    // SAFETY: acl is owned by this function and released exactly once.
    if unsafe { acl_free(acl) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    match entry_rc {
        0 => Err(invalid(
            "frozen-tool node has an extended ACL; the sealed profile requires none",
        )),
        -1 if entry_error.raw_os_error() == Some(libc::EINVAL) => Ok(()),
        _ => Err(entry_error.into()),
    }
}

#[cfg(target_os = "linux")]
fn verify_no_extended_metadata(fd: RawFd) -> Result<(), AcceptanceError> {
    // Linux stores POSIX ACLs as xattrs, so an empty fd-local list closes both.
    // SAFETY: fd is live and a null buffer requests only the name-list size.
    let xattr_bytes = unsafe { libc::flistxattr(fd, std::ptr::null_mut(), 0) };
    if xattr_bytes < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if xattr_bytes != 0 {
        return Err(invalid("frozen-tool node has an ACL or extended attribute"));
    }
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn verify_no_extended_metadata(_fd: RawFd) -> Result<(), AcceptanceError> {
    Err(invalid(
        "frozen-tool ACL/xattr inventory is unsupported on this platform",
    ))
}

#[cfg(target_os = "macos")]
fn rename_noreplace(
    source_parent_fd: RawFd,
    source_name: &str,
    destination_parent_fd: RawFd,
    destination_name: &str,
) -> Result<i32, AcceptanceError> {
    const RENAME_EXCL: libc::c_uint = 0x0000_0004;
    let source_name = c_component(source_name)?;
    let destination_name = c_component(destination_name)?;
    // SAFETY: both parent descriptors and C strings are live; RENAME_EXCL is
    // the macOS atomic no-replace contract.
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
    Ok(rc)
}

#[cfg(target_os = "linux")]
fn rename_noreplace(
    source_parent_fd: RawFd,
    source_name: &str,
    destination_parent_fd: RawFd,
    destination_name: &str,
) -> Result<i32, AcceptanceError> {
    const RENAME_NOREPLACE: libc::c_uint = 1;
    let source_name = c_component(source_name)?;
    let destination_name = c_component(destination_name)?;
    // SAFETY: syscall arguments are live and renameat2 performs one atomic
    // no-replace operation on the two descriptor-relative children.
    let rc = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            source_parent_fd,
            source_name.as_ptr(),
            destination_parent_fd,
            destination_name.as_ptr(),
            RENAME_NOREPLACE,
        )
    };
    if rc != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(rc as i32)
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn rename_noreplace(
    _source_parent_fd: RawFd,
    _source_name: &str,
    _destination_parent_fd: RawFd,
    _destination_name: &str,
) -> Result<i32, AcceptanceError> {
    Err(invalid(
        "frozen-tool atomic no-replace publication is unsupported",
    ))
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn acl_free(object: *mut libc::c_void) -> libc::c_int;
    fn acl_get_entry(
        acl: *mut libc::c_void,
        entry_id: libc::c_int,
        entry_p: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn acl_get_fd_np(fd: libc::c_int, acl_type: libc::c_int) -> *mut libc::c_void;
    fn renameatx_np(
        fromfd: libc::c_int,
        from: *const libc::c_char,
        tofd: libc::c_int,
        to: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use tempfile::TempDir;

    use super::*;

    fn private_parent() -> TempDir {
        let temporary = tempfile::Builder::new()
            .prefix("hepta-rev8-publisher-")
            .tempdir()
            .expect("private temporary parent");
        fs::set_permissions(temporary.path(), fs::Permissions::from_mode(0o700))
            .expect("private parent mode");
        temporary
    }

    fn fixture(parent: &Path, name: &str) {
        let root = parent.join(name);
        fs::create_dir(&root).expect("incoming root");
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("root mode");
        fs::create_dir(root.join("bin")).expect("bin");
        fs::set_permissions(root.join("bin"), fs::Permissions::from_mode(0o700)).expect("bin mode");
        fs::write(root.join(ACCEPTANCE_BINARY), b"acceptance-v3\n").expect("acceptance binary");
        fs::write(root.join(PUBLISHER_BINARY), b"publisher-v1\n").expect("publisher binary");
        fs::write(
            root.join(SOURCE_IDENTITY),
            canonical_json(&FrozenToolSourceIdentityV1 {
                head: "1".repeat(40),
                parent: "2".repeat(40),
                remote_tracking: "local_remote_tracking_only".to_string(),
                schema: "hepta_operator_acceptance_frozen_tool_source_v1".to_string(),
                schema_version: 1,
                tree: "3".repeat(40),
            })
            .expect("source JSON"),
        )
        .expect("source identity");
        fs::write(
            root.join("README.md"),
            b"tamper-evident requires external pin\n",
        )
        .expect("README");
    }

    fn expected(sealed: &SealedStagingV1) -> BTreeMap<&'static str, String> {
        [
            (SHA256SUMS, sealed.sha256sums.sha256.clone()),
            (MODES, sealed.modes.sha256.clone()),
            (METADATA, sealed.metadata.sha256.clone()),
            (ACL, sealed.acl.sha256.clone()),
            (XATTRS, sealed.xattrs.sha256.clone()),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn seal_and_publish_are_complete_nofollow_and_no_replace() {
        let parent = private_parent();
        fixture(parent.path(), ".incoming-one");
        let sealed = seal_staging(parent.path(), ".incoming-one").expect("seal staging");
        assert!(sealed.sha256sums.rows > 0);
        assert_eq!(sealed.modes.rows, sealed.metadata.rows);
        assert_eq!(sealed.acl.rows, sealed.metadata.rows);
        assert_eq!(sealed.xattrs.rows, sealed.metadata.rows);
        let published = publish_staging(
            parent.path(),
            ".incoming-one",
            "hepta-tool-one",
            &expected(&sealed),
        )
        .expect("publish staging");
        assert_eq!(published.publication_result.rename_rc, 0);
        assert!(published.publication_result.rename_no_replace);
        assert!(
            published
                .publication_result
                .tamper_evident_requires_external_pin
        );
        assert!(parent.path().join("hepta-tool-one").is_dir());
        assert!(
            parent
                .path()
                .join("hepta-tool-one.PUBLICATION-RESULT.json")
                .is_file()
        );

        fixture(parent.path(), ".incoming-two");
        let second = seal_staging(parent.path(), ".incoming-two").expect("seal second");
        assert!(
            publish_staging(
                parent.path(),
                ".incoming-two",
                "hepta-tool-one",
                &expected(&second),
            )
            .is_err()
        );
        assert!(parent.path().join(".incoming-two").is_dir());

        fixture(parent.path(), ".incoming-result-race");
        let result_race =
            seal_staging(parent.path(), ".incoming-result-race").expect("seal result race");
        fs::write(
            parent
                .path()
                .join("hepta-tool-result-race.PUBLICATION-RESULT.json"),
            b"preexisting\n",
        )
        .expect("precreate publication result");
        assert!(
            publish_staging(
                parent.path(),
                ".incoming-result-race",
                "hepta-tool-result-race",
                &expected(&result_race),
            )
            .is_err()
        );
        assert!(parent.path().join(".incoming-result-race").is_dir());
        assert!(!parent.path().join("hepta-tool-result-race").exists());
    }

    #[test]
    fn symlink_hardlink_xattr_and_external_pin_drift_fail_closed() {
        let parent = private_parent();
        fixture(parent.path(), ".incoming-symlink");
        std::os::unix::fs::symlink("README.md", parent.path().join(".incoming-symlink/link"))
            .expect("symlink fixture");
        assert!(seal_staging(parent.path(), ".incoming-symlink").is_err());

        fixture(parent.path(), ".incoming-hardlink");
        fs::hard_link(
            parent.path().join(".incoming-hardlink/README.md"),
            parent.path().join(".incoming-hardlink/README.alias"),
        )
        .expect("hardlink fixture");
        assert!(seal_staging(parent.path(), ".incoming-hardlink").is_err());

        #[cfg(target_os = "macos")]
        {
            fixture(parent.path(), ".incoming-xattr");
            let path = CString::new(
                parent
                    .path()
                    .join(".incoming-xattr/README.md")
                    .as_os_str()
                    .as_bytes(),
            )
            .expect("xattr path");
            let name = CString::new("com.hepta.test").expect("xattr name");
            // SAFETY: strings and one-byte value are live for the syscall.
            assert_eq!(
                unsafe {
                    libc::setxattr(path.as_ptr(), name.as_ptr(), b"x".as_ptr().cast(), 1, 0, 0)
                },
                0
            );
            assert!(seal_staging(parent.path(), ".incoming-xattr").is_err());
        }

        fixture(parent.path(), ".incoming-pin");
        let sealed = seal_staging(parent.path(), ".incoming-pin").expect("seal pin fixture");
        let mut wrong = expected(&sealed);
        wrong.insert(METADATA, "0".repeat(64));
        assert!(publish_staging(parent.path(), ".incoming-pin", "hepta-tool-pin", &wrong).is_err());
        assert!(parent.path().join(".incoming-pin").is_dir());
        assert!(!parent.path().join("hepta-tool-pin").exists());
    }
}

#[cfg(target_os = "linux")]
fn get_errno() -> libc::c_int {
    // SAFETY: __errno_location returns the calling thread's errno pointer.
    unsafe { *libc::__errno_location() }
}

#[cfg(target_os = "linux")]
fn set_errno(value: libc::c_int) {
    // SAFETY: __errno_location returns the calling thread's errno pointer.
    unsafe { *libc::__errno_location() = value };
}
