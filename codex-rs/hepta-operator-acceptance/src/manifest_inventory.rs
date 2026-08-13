use std::collections::BTreeMap;
use std::collections::BTreeSet;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::Read;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::os::unix::ffi::OsStrExt;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::AcceptanceError;
use crate::durable::MAX_SMALL_FILE_BYTES;
use crate::durable::canonical_json;
use crate::durable::secure_hash;
use crate::durable::secure_read;
use crate::durable::secure_root;
use crate::durable::sha256;
use crate::durable::verify_secure_directory;

type LegacyPathInventories = (BTreeSet<String>, Vec<u8>, Vec<u8>);
#[cfg(target_os = "macos")]
type MacExtendedMetadata = (BTreeMap<String, Vec<u8>>, Option<String>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ManifestEntry {
    pub sha256: String,
    pub size_bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LegacyManifestSnapshot {
    pub extended_metadata_inventory: Vec<u8>,
    pub entries: BTreeMap<String, ManifestEntry>,
    pub hardlink_topology: Vec<u8>,
    pub inventory: Vec<u8>,
    pub metadata_inventory: Vec<u8>,
    pub manifest_relative_path: String,
    pub manifest_sha256: String,
    pub root: PathBuf,
    extended_metadata_policy: LegacyExtendedMetadataPolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LegacyExtendedMetadataPolicy {
    None,
    MacAttempt2,
    PortableInputs,
}

impl LegacyManifestSnapshot {
    pub(crate) fn reverify(&self) -> Result<Self, AcceptanceError> {
        let fresh = load_legacy_manifest_with_policy(
            &self.root,
            &self.manifest_relative_path,
            &self.manifest_sha256,
            self.entries.len().saturating_sub(1),
            self.extended_metadata_policy,
        )?;
        if fresh.entries != self.entries
            || fresh.extended_metadata_inventory != self.extended_metadata_inventory
            || fresh.hardlink_topology != self.hardlink_topology
            || fresh.inventory != self.inventory
            || fresh.metadata_inventory != self.metadata_inventory
        {
            return Err(invalid(
                "original receipt changed during wrapper provenance verification",
            ));
        }
        Ok(fresh)
    }
}

pub(crate) fn load_legacy_manifest(
    root: &Path,
    manifest_relative_path: &str,
    expected_sha256: &str,
    expected_entries: usize,
) -> Result<LegacyManifestSnapshot, AcceptanceError> {
    load_legacy_manifest_with_policy(
        root,
        manifest_relative_path,
        expected_sha256,
        expected_entries,
        LegacyExtendedMetadataPolicy::None,
    )
}

pub(crate) fn load_legacy_manifest_with_policy(
    root: &Path,
    manifest_relative_path: &str,
    expected_sha256: &str,
    expected_entries: usize,
    extended_metadata_policy: LegacyExtendedMetadataPolicy,
) -> Result<LegacyManifestSnapshot, AcceptanceError> {
    validate_relative_path(manifest_relative_path)?;
    if !digest_shape(expected_sha256) {
        return Err(invalid("original manifest digest is malformed"));
    }
    let root = secure_root(root, "original receipt root")?;
    let manifest_path = root.join(manifest_relative_path);
    let manifest_bytes = read_legacy_regular(
        &manifest_path,
        MAX_SMALL_FILE_BYTES as u64,
        extended_metadata_policy,
    )?
    .0;
    if sha256(&manifest_bytes) != expected_sha256 {
        return Err(invalid("original manifest differs from its frozen digest"));
    }
    let parsed = parse_manifest(&manifest_bytes)?;
    if parsed.len() != expected_entries {
        return Err(invalid(
            "original manifest entry count differs from its pin",
        ));
    }

    let (paths, metadata_inventory, extended_metadata_inventory) =
        legacy_paths(&root, extended_metadata_policy)?;
    let mut expected = parsed.keys().cloned().collect::<BTreeSet<_>>();
    expected.insert(manifest_relative_path.to_string());
    if paths != expected {
        return Err(invalid(
            "original receipt file set differs from its frozen hash manifest",
        ));
    }

    let mut entries = BTreeMap::new();
    let mut link_groups: BTreeMap<(u64, u64, u64), Vec<String>> = BTreeMap::new();
    for relative in &paths {
        let (digest, metadata) =
            hash_legacy_regular(&root.join(relative), extended_metadata_policy)?;
        let expected_digest = if relative == manifest_relative_path {
            expected_sha256
        } else {
            parsed
                .get(relative)
                .ok_or_else(|| invalid("original manifest path is absent"))?
        };
        if digest != expected_digest {
            return Err(invalid(format!(
                "original artifact hash differs for {relative}"
            )));
        }
        entries.insert(
            relative.clone(),
            ManifestEntry {
                sha256: digest,
                size_bytes: metadata.len(),
            },
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if metadata.nlink() > 1 {
                link_groups
                    .entry((metadata.dev(), metadata.ino(), metadata.nlink()))
                    .or_default()
                    .push(relative.clone());
            }
        }
    }
    let hardlink_topology = canonical_hardlink_topology(&mut link_groups)?;
    let inventory = entries
        .iter()
        .map(|(path, entry)| format!("{}\t{}\t./{path}\n", entry.sha256, entry.size_bytes))
        .collect::<String>()
        .into_bytes();
    Ok(LegacyManifestSnapshot {
        entries,
        extended_metadata_inventory,
        hardlink_topology,
        inventory,
        metadata_inventory,
        manifest_relative_path: manifest_relative_path.to_string(),
        manifest_sha256: expected_sha256.to_string(),
        root,
        extended_metadata_policy,
    })
}

fn legacy_paths(
    root: &Path,
    extended_metadata_policy: LegacyExtendedMetadataPolicy,
) -> Result<LegacyPathInventories, AcceptanceError> {
    let mut files = BTreeSet::new();
    let mut metadata_rows = BTreeMap::new();
    let mut extended_metadata_rows = BTreeMap::new();
    let mut pending = vec![(root.to_path_buf(), 0_u8)];
    while let Some((directory, depth)) = pending.pop() {
        if depth > 32 {
            return Err(invalid("original receipt directory depth exceeds 32"));
        }
        verify_legacy_directory(&directory, extended_metadata_policy)?;
        capture_legacy_extended_metadata(
            root,
            &directory,
            extended_metadata_policy,
            &mut extended_metadata_rows,
        )?;
        let directory_metadata = std::fs::symlink_metadata(&directory)?;
        let directory_relative = directory
            .strip_prefix(root)
            .map_err(|_| invalid("original directory escaped its root"))?
            .to_str()
            .ok_or_else(|| invalid("original directory path is not UTF-8"))?;
        let inventory_path = if directory_relative.is_empty() {
            ".".to_string()
        } else {
            format!("./{directory_relative}")
        };
        metadata_rows.insert(
            directory_relative.to_string(),
            format!(
                "Directory\t{:o}\t-\t{inventory_path}\n",
                posix_mode(&directory_metadata)?
            ),
        );
        let mut child_count = 0_usize;
        for entry in std::fs::read_dir(&directory)? {
            child_count += 1;
            let path = entry?.path();
            let metadata = std::fs::symlink_metadata(&path)?;
            if metadata.file_type().is_symlink() {
                return Err(invalid("original receipt contains a symlink"));
            }
            if metadata.is_dir() {
                pending.push((path, depth.saturating_add(1)));
            } else if metadata.is_file() {
                capture_legacy_extended_metadata(
                    root,
                    &path,
                    extended_metadata_policy,
                    &mut extended_metadata_rows,
                )?;
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_| invalid("original artifact escaped its root"))?
                    .to_str()
                    .ok_or_else(|| invalid("original artifact path is not UTF-8"))?
                    .to_string();
                validate_relative_path(&relative)?;
                metadata_rows.insert(
                    relative.clone(),
                    format!(
                        "Regular File\t{:o}\t{}\t./{relative}\n",
                        posix_mode(&metadata)?,
                        metadata.len()
                    ),
                );
                files.insert(relative);
                if files.len() > 4_097 {
                    return Err(invalid("original receipt exceeds 4097 files"));
                }
            } else {
                return Err(invalid("original receipt contains a special file"));
            }
        }
        if directory != root && child_count == 0 {
            return Err(invalid("original receipt contains an empty directory"));
        }
    }
    Ok((
        files,
        metadata_rows.into_values().collect::<String>().into_bytes(),
        extended_metadata_rows
            .into_values()
            .collect::<Vec<_>>()
            .concat(),
    ))
}

fn verify_legacy_directory(
    path: &Path,
    policy: LegacyExtendedMetadataPolicy,
) -> Result<(), AcceptanceError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(invalid(
            "original receipt directory is not a real directory",
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let mode = metadata.mode() & 0o7777;
        let allowed = match policy {
            LegacyExtendedMetadataPolicy::None => mode & 0o077 == 0,
            LegacyExtendedMetadataPolicy::MacAttempt2 => matches!(mode, 0o555 | 0o700),
            LegacyExtendedMetadataPolicy::PortableInputs => mode == 0o700,
        };
        if mode & 0o7000 != 0 || !allowed {
            return Err(invalid(
                "original receipt directory mode differs from its profile allowlist",
            ));
        }
        // SAFETY: geteuid takes no arguments and has no memory preconditions.
        if metadata.uid() != unsafe { libc::geteuid() } {
            return Err(invalid("original receipt directory owner differs"));
        }
    }
    Ok(())
}

fn posix_mode(metadata: &std::fs::Metadata) -> Result<u32, AcceptanceError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let mode = metadata.mode() & 0o7777;
        if mode & 0o7000 != 0 {
            return Err(invalid("original receipt contains special mode bits"));
        }
        Ok(mode)
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        Err(invalid(
            "original POSIX metadata inventory is unsupported on this platform",
        ))
    }
}

fn capture_legacy_extended_metadata(
    root: &Path,
    path: &Path,
    policy: LegacyExtendedMetadataPolicy,
    rows: &mut BTreeMap<String, Vec<u8>>,
) -> Result<(), AcceptanceError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| invalid("original extended metadata escaped its root"))?
        .to_str()
        .ok_or_else(|| invalid("original extended metadata path is not UTF-8"))?;
    #[cfg(target_os = "macos")]
    {
        let expected = policy == LegacyExtendedMetadataPolicy::MacAttempt2
            && matches!(
                relative,
                "full-state-fixture-source/archive/record.json"
                    | "full-state-snapshot/archive/record.json"
            );
        let (xattrs, acl) = capture_macos_extended_metadata(path)?;
        if expected {
            let expected_xattrs = [(
                "com.hepta.snapshot.canary".to_string(),
                b"full-root-v2-hardlinks".to_vec(),
            )]
            .into_iter()
            .collect::<BTreeMap<_, _>>();
            if xattrs != expected_xattrs
                || acl.as_deref()
                    != Some("group:ABCDEFAB-CDEF-ABCD-EFAB-CDEF0000000C:everyone:12:deny:delete")
            {
                return Err(invalid(
                    "Mac original intended extended metadata differs from its exact profile",
                ));
            }
            let encoded = format!(
                "ACL\t./{relative}\t{}\nXATTR\t./{relative}\t{}\t{}\n",
                hex_bytes(acl.as_deref().unwrap_or_default().as_bytes()),
                hex_bytes(b"com.hepta.snapshot.canary"),
                hex_bytes(b"full-root-v2-hardlinks"),
            )
            .into_bytes();
            rows.insert(relative.to_string(), encoded);
        } else if !xattrs.is_empty() || acl.is_some() {
            return Err(invalid(
                "Mac original contains unexpected extended metadata",
            ));
        }
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (root, path, rows, relative, policy);
        verify_no_extended_metadata(path)
    }
}

fn hex_bytes(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

#[cfg(target_os = "macos")]
fn capture_macos_extended_metadata(path: &Path) -> Result<MacExtendedMetadata, AcceptanceError> {
    const ACL_TYPE_EXTENDED: libc::c_int = 0x0000_0100;
    const XATTR_NOFOLLOW: libc::c_int = 0x0001;
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| invalid("original extended metadata path contains NUL"))?;
    // SAFETY: `path` is live and buffers are sized from the preceding calls.
    let name_bytes =
        unsafe { libc::listxattr(path.as_ptr(), std::ptr::null_mut(), 0, XATTR_NOFOLLOW) };
    if name_bytes < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let mut names = vec![0_u8; name_bytes as usize];
    if name_bytes > 0 {
        let read = unsafe {
            libc::listxattr(
                path.as_ptr(),
                names.as_mut_ptr().cast(),
                names.len(),
                XATTR_NOFOLLOW,
            )
        };
        if read != name_bytes {
            return Err(invalid("original xattr names changed during capture"));
        }
    }
    let mut xattrs = BTreeMap::new();
    for raw in names
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
    {
        let name =
            std::str::from_utf8(raw).map_err(|_| invalid("original xattr name is not UTF-8"))?;
        let name_c = CString::new(name).map_err(|_| invalid("original xattr name contains NUL"))?;
        let size = unsafe {
            libc::getxattr(
                path.as_ptr(),
                name_c.as_ptr(),
                std::ptr::null_mut(),
                0,
                0,
                XATTR_NOFOLLOW,
            )
        };
        if !(0..=1024 * 1024).contains(&size) {
            return Err(invalid("original xattr value is unreadable or too large"));
        }
        let mut value = vec![0_u8; size as usize];
        let read = unsafe {
            libc::getxattr(
                path.as_ptr(),
                name_c.as_ptr(),
                value.as_mut_ptr().cast(),
                value.len(),
                0,
                XATTR_NOFOLLOW,
            )
        };
        if read != size || xattrs.insert(name.to_string(), value).is_some() {
            return Err(invalid("original xattr changed during capture"));
        }
    }

    let acl = unsafe { acl_get_file(path.as_ptr(), ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ENOENT) {
            return Ok((xattrs, None));
        }
        return Err(error.into());
    }
    let mut length = 0_isize;
    let text = unsafe { acl_to_text(acl, &mut length) };
    let text_error = std::io::Error::last_os_error();
    let acl_free_result = unsafe { acl_free(acl) };
    if text.is_null() {
        return Err(text_error.into());
    }
    if acl_free_result != 0 || !(0..=1024 * 1024).contains(&length) {
        // SAFETY: `text` is the live allocation returned by acl_to_text.
        let _ = unsafe { acl_free(text.cast()) };
        return Err(if acl_free_result != 0 {
            text_error.into()
        } else {
            invalid("original ACL text length is invalid or too large")
        });
    }
    // Copy before freeing so every later parse error still follows the single
    // unconditional release below.
    let raw = unsafe { std::slice::from_raw_parts(text.cast::<u8>(), length as usize) }.to_vec();
    let free_text_result = unsafe { acl_free(text.cast()) };
    if free_text_result != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let normalized = std::str::from_utf8(&raw)
        .map_err(|_| invalid("original ACL text is not UTF-8"))?
        .trim()
        .strip_prefix("!#acl 1\n")
        .unwrap_or_else(|| std::str::from_utf8(&raw).unwrap_or_default().trim())
        .trim_start_matches("0: ")
        .to_string();
    Ok((xattrs, (!normalized.is_empty()).then_some(normalized)))
}

fn read_legacy_regular(
    path: &Path,
    max_bytes: u64,
    policy: LegacyExtendedMetadataPolicy,
) -> Result<(Vec<u8>, std::fs::Metadata), AcceptanceError> {
    let (mut file, before) = open_legacy_regular(path, policy)?;
    if before.len() > max_bytes {
        return Err(invalid("original artifact exceeds its read bound"));
    }
    let mut bytes = Vec::new();
    (&mut file)
        .take(max_bytes.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > max_bytes {
        return Err(invalid("original artifact exceeds its read bound"));
    }
    verify_legacy_file_unchanged(path, &file, &before)?;
    Ok((bytes, before))
}

fn hash_legacy_regular(
    path: &Path,
    policy: LegacyExtendedMetadataPolicy,
) -> Result<(String, std::fs::Metadata), AcceptanceError> {
    let (mut file, before) = open_legacy_regular(path, policy)?;
    if before.len() > 2 * 1024 * 1024 * 1024 {
        return Err(invalid("original artifact exceeds the 2 GiB bound"));
    }
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 128 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    verify_legacy_file_unchanged(path, &file, &before)?;
    Ok((format!("{:x}", hasher.finalize()), before))
}

fn open_legacy_regular(
    path: &Path,
    policy: LegacyExtendedMetadataPolicy,
) -> Result<(std::fs::File, std::fs::Metadata), AcceptanceError> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW);
    }
    let file = options.open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(invalid("original artifact is not a regular file"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let mode = metadata.mode() & 0o7777;
        let allowed = match policy {
            LegacyExtendedMetadataPolicy::None => mode & 0o077 == 0,
            LegacyExtendedMetadataPolicy::MacAttempt2 => {
                matches!(mode, 0o400 | 0o444 | 0o500 | 0o555 | 0o600)
            }
            LegacyExtendedMetadataPolicy::PortableInputs => matches!(mode, 0o400 | 0o500),
        };
        if mode & 0o7000 != 0 || !allowed {
            return Err(invalid(
                "original artifact mode differs from its profile allowlist",
            ));
        }
        // SAFETY: geteuid takes no arguments and has no memory preconditions.
        if metadata.uid() != unsafe { libc::geteuid() } {
            return Err(invalid("original artifact has an unexpected owner"));
        }
    }
    Ok((file, metadata))
}

fn verify_legacy_file_unchanged(
    path: &Path,
    file: &std::fs::File,
    before: &std::fs::Metadata,
) -> Result<(), AcceptanceError> {
    let fd_after = file.metadata()?;
    let path_after = std::fs::metadata(path)?;
    let link_after = std::fs::symlink_metadata(path)?;
    if link_after.file_type().is_symlink()
        || legacy_snapshot(before) != legacy_snapshot(&fd_after)
        || legacy_snapshot(before) != legacy_snapshot(&path_after)
    {
        return Err(invalid("original artifact changed while it was verified"));
    }
    Ok(())
}

fn legacy_snapshot(metadata: &std::fs::Metadata) -> String {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}",
            metadata.dev(),
            metadata.ino(),
            metadata.len(),
            metadata.mtime(),
            metadata.mtime_nsec(),
            metadata.ctime(),
            metadata.ctime_nsec(),
            metadata.mode(),
            metadata.nlink(),
        )
    }
    #[cfg(not(unix))]
    {
        format!("{}:{:?}", metadata.len(), metadata.modified().ok())
    }
}

fn canonical_hardlink_topology(
    groups: &mut BTreeMap<(u64, u64, u64), Vec<String>>,
) -> Result<Vec<u8>, AcceptanceError> {
    let mut rows = Vec::new();
    for ((_, _, expected_links), paths) in groups {
        paths.sort();
        if paths.len() as u64 != *expected_links || paths.len() < 2 {
            return Err(invalid(
                "original hardlink group escapes the receipt or is incomplete",
            ));
        }
        rows.push(format!(
            "{}\t{}\n",
            expected_links,
            paths
                .iter()
                .map(|path| format!("./{path}"))
                .collect::<Vec<_>>()
                .join("\t")
        ));
    }
    rows.sort();
    Ok(rows.concat().into_bytes())
}

pub(crate) struct VerifiedManifest {
    directories: BTreeSet<String>,
    entries: BTreeMap<String, ManifestEntry>,
    inventory: Inventory,
    manifest_relative_path: String,
    manifest_sha256: String,
    pub root: PathBuf,
}

impl VerifiedManifest {
    pub(crate) fn load(
        root: &Path,
        expected_sha256: &str,
        expected_entries: usize,
    ) -> Result<Self, AcceptanceError> {
        Self::load_named(root, "SHA256SUMS", expected_sha256, expected_entries)
    }

    pub(crate) fn load_named(
        root: &Path,
        manifest_relative_path: &str,
        expected_sha256: &str,
        expected_entries: usize,
    ) -> Result<Self, AcceptanceError> {
        let verified =
            Self::load_named_digest_pinned(root, manifest_relative_path, expected_sha256)?;
        if verified.entries.len() != expected_entries {
            return Err(invalid("hash manifest entry count differs from its pin"));
        }
        Ok(verified)
    }

    pub(crate) fn load_digest_pinned(
        root: &Path,
        expected_sha256: &str,
    ) -> Result<Self, AcceptanceError> {
        Self::load_named_digest_pinned(root, "SHA256SUMS", expected_sha256)
    }

    pub(crate) fn load_named_digest_pinned(
        root: &Path,
        manifest_relative_path: &str,
        expected_sha256: &str,
    ) -> Result<Self, AcceptanceError> {
        validate_relative_path(manifest_relative_path)?;
        let root = secure_root(root, "evidence root")?;
        let sums_path = root.join(manifest_relative_path);
        let sums = secure_read(&sums_path, MAX_SMALL_FILE_BYTES)?;
        if sha256(&sums) != expected_sha256 {
            return Err(invalid("hash manifest differs from its frozen digest"));
        }
        let parsed = parse_manifest(&sums)?;
        let actual = inventory(&root)?;
        let expected = parsed.keys().cloned().collect::<BTreeSet<_>>();
        let mut actual_paths = actual.files.keys().cloned().collect::<BTreeSet<_>>();
        actual_paths.remove(manifest_relative_path);
        if actual_paths != expected {
            return Err(invalid(
                "evidence inventory differs from the exact hash-manifest paths",
            ));
        }
        let mut entries = BTreeMap::new();
        for (relative, expected_sha256) in parsed {
            let (actual_sha256, size_bytes) = secure_hash(&root.join(&relative))?;
            if actual_sha256 != expected_sha256 {
                return Err(invalid(format!(
                    "evidence artifact hash differs for {relative}"
                )));
            }
            entries.insert(
                relative,
                ManifestEntry {
                    sha256: actual_sha256,
                    size_bytes,
                },
            );
        }
        let actual_after = inventory(&root)?;
        if actual_after != actual {
            return Err(invalid(
                "evidence paths or metadata changed during full artifact verification",
            ));
        }
        if sha256(&secure_read(&sums_path, MAX_SMALL_FILE_BYTES)?) != expected_sha256 {
            return Err(invalid(
                "hash manifest changed during evidence verification",
            ));
        }
        Ok(Self {
            directories: actual.directories.keys().cloned().collect(),
            entries,
            inventory: actual,
            manifest_relative_path: manifest_relative_path.to_string(),
            manifest_sha256: expected_sha256.to_string(),
            root,
        })
    }

    pub(crate) fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn bytes(&self, relative: &str) -> Result<Vec<u8>, AcceptanceError> {
        self.bytes_bounded(relative, MAX_SMALL_FILE_BYTES)
    }

    pub(crate) fn bytes_bounded(
        &self,
        relative: &str,
        max_bytes: usize,
    ) -> Result<Vec<u8>, AcceptanceError> {
        let entry = self
            .entries
            .get(relative)
            .ok_or_else(|| invalid(format!("required manifest entry is absent: {relative}")))?;
        if max_bytes < MAX_SMALL_FILE_BYTES || entry.size_bytes > max_bytes as u64 {
            return Err(invalid(
                "required artifact exceeds its explicitly compiled read bound",
            ));
        }
        let bytes = secure_read(&self.root.join(relative), max_bytes)?;
        if sha256(&bytes) != entry.sha256 {
            return Err(invalid(format!(
                "required artifact changed after manifest verification: {relative}"
            )));
        }
        Ok(bytes)
    }

    pub(crate) fn json_pinned<T: for<'de> Deserialize<'de>>(
        &self,
        relative: &str,
    ) -> Result<T, AcceptanceError> {
        let bytes = self.bytes(relative)?;
        serde_json::from_slice(&bytes)
            .map_err(|error| invalid(format!("invalid {relative}: {error}")))
    }

    pub(crate) fn json_canonical<T: for<'de> Deserialize<'de> + Serialize>(
        &self,
        relative: &str,
    ) -> Result<T, AcceptanceError> {
        let bytes = self.bytes(relative)?;
        let value = self.json_pinned(relative)?;
        if canonical_json(&value)? != bytes {
            return Err(invalid(format!("{relative} is not canonical JSON")));
        }
        Ok(value)
    }

    pub(crate) fn entry(&self, relative: &str) -> Option<&ManifestEntry> {
        self.entries.get(relative)
    }

    pub(crate) fn entry_paths(&self) -> impl Iterator<Item = &str> {
        self.entries.keys().map(String::as_str)
    }

    pub(crate) fn directory_paths(&self) -> impl Iterator<Item = &str> {
        self.directories.iter().map(String::as_str)
    }

    pub(crate) fn manifest_relative_path(&self) -> &str {
        &self.manifest_relative_path
    }

    pub(crate) fn reverify(&self) -> Result<(), AcceptanceError> {
        let fresh = Self::load_named(
            &self.root,
            &self.manifest_relative_path,
            &self.manifest_sha256,
            self.entries.len(),
        )?;
        if fresh.entries != self.entries
            || fresh.directories != self.directories
            || fresh.inventory != self.inventory
        {
            return Err(invalid(
                "evidence inventory differs during terminal re-verification",
            ));
        }
        Ok(())
    }

    pub(crate) fn require_hash(
        &self,
        relative: &str,
        expected: &str,
    ) -> Result<(), AcceptanceError> {
        if self.entry(relative).map(|entry| entry.sha256.as_str()) != Some(expected) {
            return Err(invalid(format!(
                "manifest binding differs for required artifact: {relative}"
            )));
        }
        Ok(())
    }
}

pub(crate) fn parse_manifest(bytes: &[u8]) -> Result<BTreeMap<String, String>, AcceptanceError> {
    if bytes.is_empty() || !bytes.ends_with(b"\n") {
        return Err(invalid(
            "SHA256SUMS must be nonempty and newline terminated",
        ));
    }
    let mut entries = BTreeMap::new();
    let mut previous: Option<String> = None;
    for line in bytes[..bytes.len() - 1].split(|byte| *byte == b'\n') {
        if line.len() < 67 || line.get(64..66) != Some(b"  ") {
            return Err(invalid("SHA256SUMS contains a malformed line"));
        }
        let digest_bytes = &line[..64];
        if !digest_bytes
            .iter()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(invalid("SHA256SUMS contains an invalid digest"));
        }
        let digest = std::str::from_utf8(digest_bytes)
            .map_err(|_| invalid("SHA256SUMS digest is not ASCII"))?;
        let suffix = std::str::from_utf8(&line[66..])
            .map_err(|_| invalid("SHA256SUMS path is not UTF-8"))?;
        let raw = suffix.strip_prefix("./").unwrap_or(suffix);
        validate_relative_path(raw)?;
        if previous.as_deref().is_some_and(|value| value >= raw) {
            return Err(invalid(
                "SHA256SUMS paths must be unique and strictly sorted",
            ));
        }
        previous = Some(raw.to_string());
        if entries
            .insert(raw.to_string(), digest.to_string())
            .is_some()
        {
            return Err(invalid("SHA256SUMS contains a duplicate path"));
        }
    }
    if entries.is_empty() {
        return Err(invalid("SHA256SUMS must contain at least one entry"));
    }
    Ok(entries)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Inventory {
    directories: BTreeMap<String, String>,
    files: BTreeMap<String, String>,
}

fn inventory(root: &Path) -> Result<Inventory, AcceptanceError> {
    let mut directories = BTreeMap::new();
    let mut files = BTreeMap::new();
    let mut pending = vec![(root.to_path_buf(), 0_u8)];
    while let Some((directory, depth)) = pending.pop() {
        if depth > 32 {
            return Err(invalid("evidence directory depth exceeds 32"));
        }
        verify_secure_directory(&directory, "evidence directory")?;
        verify_no_extended_metadata(&directory)?;
        let directory_relative = directory
            .strip_prefix(root)
            .map_err(|_| invalid("evidence directory escaped its root"))?
            .to_str()
            .ok_or_else(|| invalid("evidence directory path is not UTF-8"))?;
        directories.insert(
            directory_relative.to_string(),
            metadata_snapshot(&std::fs::symlink_metadata(&directory)?),
        );
        if directories.len() > 4_097 {
            return Err(invalid("evidence inventory exceeds 4097 directories"));
        }
        for entry in std::fs::read_dir(&directory)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = std::fs::symlink_metadata(&path)?;
            if metadata.file_type().is_symlink() {
                return Err(invalid("evidence tree contains a symlink"));
            }
            if metadata.is_dir() {
                verify_no_extended_metadata(&path)?;
                pending.push((path, depth.saturating_add(1)));
            } else if metadata.is_file() {
                verify_no_extended_metadata(&path)?;
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_| invalid("evidence path escaped its root"))?
                    .to_str()
                    .ok_or_else(|| invalid("evidence path is not UTF-8"))?
                    .to_string();
                validate_relative_path(&relative)?;
                files.insert(relative, metadata_snapshot(&metadata));
                if files.len() > 4_097 {
                    return Err(invalid("evidence inventory exceeds 4097 files"));
                }
            } else {
                return Err(invalid("evidence tree contains a special file"));
            }
        }
    }
    Ok(Inventory { directories, files })
}

#[cfg(target_os = "macos")]
fn verify_no_extended_metadata(path: &Path) -> Result<(), AcceptanceError> {
    const ACL_FIRST_ENTRY: libc::c_int = 0;
    const ACL_TYPE_EXTENDED: libc::c_int = 0x0000_0100;
    const XATTR_NOFOLLOW: libc::c_int = 0x0001;

    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| invalid("evidence path contains NUL"))?;
    // SAFETY: `path` is a live NUL-terminated string. A null buffer with zero
    // length requests only the extended-attribute byte count.
    let xattr_bytes =
        unsafe { libc::listxattr(path.as_ptr(), std::ptr::null_mut(), 0, XATTR_NOFOLLOW) };
    if xattr_bytes < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if xattr_bytes != 0 {
        return Err(invalid(
            "evidence path has extended attributes; V3 policy requires none",
        ));
    }

    // SAFETY: these are stable macOS ACL interfaces. The returned ACL object,
    // when non-null, is released exactly once with `acl_free`.
    let acl = unsafe { acl_get_file(path.as_ptr(), ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ENOENT) {
            return Ok(());
        }
        return Err(error.into());
    }
    let mut entry = std::ptr::null_mut();
    // SAFETY: `acl` is a valid object returned above and `entry` is writable.
    let entry_result = unsafe { acl_get_entry(acl, ACL_FIRST_ENTRY, &mut entry) };
    let entry_error = std::io::Error::last_os_error();
    // SAFETY: `acl` is owned by this function and has not been freed.
    let free_result = unsafe { acl_free(acl) };
    if free_result != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    match entry_result {
        0 => Err(invalid(
            "evidence path has an extended ACL; V3 policy requires none",
        )),
        -1 if entry_error.raw_os_error() == Some(libc::EINVAL) => Ok(()),
        _ => Err(entry_error.into()),
    }
}

#[cfg(target_os = "linux")]
fn verify_no_extended_metadata(path: &Path) -> Result<(), AcceptanceError> {
    let path_c = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| invalid("evidence path contains NUL"))?;
    // SAFETY: `path_c` is a live NUL-terminated string. A null buffer with
    // zero length requests only the extended-attribute byte count.
    let xattr_bytes = unsafe { libc::llistxattr(path_c.as_ptr(), std::ptr::null_mut(), 0) };
    if xattr_bytes < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if xattr_bytes != 0 {
        return Err(invalid(
            "evidence path has extended attributes, including a possible POSIX ACL",
        ));
    }

    let getfacl = Path::new("/usr/bin/getfacl");
    if !getfacl.is_file() {
        return Err(invalid(
            "Linux ACL policy cannot be verified because /usr/bin/getfacl is absent",
        ));
    }
    let output = std::process::Command::new(getfacl)
        .args(["--absolute-names", "--numeric", "--omit-header", "--"])
        .arg(path)
        .output()?;
    if !output.status.success() {
        return Err(invalid("Linux getfacl verification failed"));
    }
    let text = std::str::from_utf8(&output.stdout)
        .map_err(|_| invalid("Linux getfacl output is not UTF-8"))?;
    let mut base_entries = BTreeSet::new();
    for line in text.lines().filter(|line| !line.is_empty()) {
        let key = line
            .split_once(':')
            .map(|(key, _)| key)
            .ok_or_else(|| invalid("Linux getfacl output is malformed"))?;
        match line {
            value if value.starts_with("user::") => {
                base_entries.insert("user");
            }
            value if value.starts_with("group::") => {
                base_entries.insert("group");
            }
            value if value.starts_with("other::") => {
                base_entries.insert("other");
            }
            _ => {
                return Err(invalid(format!(
                    "evidence path has a non-base ACL entry: {key}"
                )));
            }
        }
    }
    if base_entries != ["group", "other", "user"].into_iter().collect() {
        return Err(invalid("Linux getfacl output omits a base ACL entry"));
    }
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn verify_no_extended_metadata(_path: &Path) -> Result<(), AcceptanceError> {
    Err(invalid(
        "V3 ACL and extended-attribute policy is unsupported on this platform",
    ))
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn acl_get_file(path_p: *const libc::c_char, acl_type: libc::c_int) -> *mut libc::c_void;
    fn acl_get_entry(
        acl: *mut libc::c_void,
        entry_id: libc::c_int,
        entry_p: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn acl_to_text(acl: *mut libc::c_void, len_p: *mut isize) -> *mut libc::c_char;
    fn acl_free(object: *mut libc::c_void) -> libc::c_int;
}

fn metadata_snapshot(metadata: &std::fs::Metadata) -> String {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            metadata.dev(),
            metadata.ino(),
            metadata.len(),
            metadata.mtime(),
            metadata.mtime_nsec(),
            metadata.ctime(),
            metadata.ctime_nsec(),
            metadata.mode(),
            metadata.uid(),
            metadata.gid(),
            metadata.nlink(),
        )
    }
    #[cfg(not(unix))]
    {
        format!(
            "{}:{:?}:{}",
            metadata.len(),
            metadata.modified().ok(),
            metadata.permissions().readonly(),
        )
    }
}

pub(crate) fn validate_relative_path(value: &str) -> Result<(), AcceptanceError> {
    if value.is_empty() || value.contains('\\') || value.chars().any(char::is_control) {
        return Err(invalid(
            "manifest path is empty or uses a forbidden control character",
        ));
    }
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(invalid("manifest path is not a safe relative path"));
    }
    Ok(())
}

pub(crate) fn digest_shape(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

#[cfg(test)]
#[path = "manifest_inventory_tests.rs"]
mod tests;
