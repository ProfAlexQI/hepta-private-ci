use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use flate2::read::GzDecoder;
use serde::Deserialize;
use sha2::Digest;
use sha2::Sha256;

const ENTRYPOINT_REGISTRY_PATH: &str = "scripts/hepta-gate-pair-entrypoints-v2.json";
const PAYLOAD_REGISTRY_PATH: &str = "scripts/lib/hepta-gate-pair-compat-v2/registry.json";
const PAYLOAD_BUNDLE_PATH: &str = "scripts/lib/hepta-gate-pair-compat-v2/payloads.bundle.gz";
const LONG_PATH_REGISTRY_PATH: &str = "scripts/lib/hepta-long-path-v1/registry.json";
const ENTRYPOINT_REGISTRY_SCHEMA: &str = "hepta_gate_pair_entrypoints_v2";
const PAYLOAD_REGISTRY_SCHEMA: &str = "hepta_gate_pair_payload_bundle_v2";
const PAYLOAD_BUNDLE_MAGIC: &[u8] = b"hepta_gate_pair_payload_bundle_v2\0";
const LONG_PATH_REGISTRY_SCHEMA: &str = "hepta_long_path_relocation_v1";
const MAX_PAYLOAD_BUNDLE_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug)]
pub(crate) struct GatePairArchive {
    retained_paths: BTreeSet<String>,
    payloads: BTreeMap<String, Vec<u8>>,
    long_paths: BTreeMap<String, LongPathEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EntrypointRegistry {
    schema: String,
    status: String,
    source_registry_sha256: String,
    pair_count: usize,
    original_entrypoint_count: usize,
    retained_entrypoint_count: usize,
    virtual_entrypoint_count: usize,
    virtual_long_path_count: usize,
    max_retained_entrypoints: usize,
    max_tracked_path_bytes: usize,
    launcher: String,
    retained_paths: Vec<String>,
    virtual_paths_sha256: String,
    reversible_unpack: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PayloadRegistry {
    schema: String,
    status: String,
    payload_count: usize,
    registered_payload_count: usize,
    source_bytes: usize,
    aggregate_source_sha256: String,
    bundle_bytes: usize,
    bundle_sha256: String,
    source_registry_sha256: String,
    legacy_archive_bytes: usize,
    legacy_archive_sha256: String,
    legacy_archive_payload_count: usize,
    source_files_materialized: bool,
    runtime_overlay_generation: bool,
    reversible_unpack: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LongPathRegistry {
    schema: String,
    status: String,
    audit_baseline_long_path_count: usize,
    prepack_materialized_long_path_count: usize,
    postpack_materialized_long_path_count: usize,
    max_tracked_path_bytes: usize,
    relocation_count: usize,
    rewritten_reference_file_count: usize,
    rewritten_reference_paths: Vec<String>,
    entries: Vec<LongPathEntry>,
    reversible_unpack: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LongPathEntry {
    original_path: String,
    relocated_path: String,
    original_path_sha256: String,
    original_content_sha256: String,
    mode: u32,
    relocated_content_sha256: String,
}

impl GatePairArchive {
    pub(crate) fn load(repo_root: &Path, source_registry: &[u8]) -> Result<Self> {
        let source_registry_sha256 = sha256(source_registry);
        let entrypoint_registry: EntrypointRegistry = read_json(
            &repo_root.join(ENTRYPOINT_REGISTRY_PATH),
            "gate-pair entrypoint registry",
        )?;
        validate_entrypoint_registry(&entrypoint_registry, &source_registry_sha256)?;

        let payload_registry: PayloadRegistry = read_json(
            &repo_root.join(PAYLOAD_REGISTRY_PATH),
            "gate-pair payload registry",
        )?;
        validate_payload_registry(&payload_registry, &source_registry_sha256)?;

        let bundle_path = repo_root.join(PAYLOAD_BUNDLE_PATH);
        let compressed = fs::read(&bundle_path)
            .with_context(|| format!("failed to read {}", bundle_path.display()))?;
        if compressed.len() != payload_registry.bundle_bytes
            || sha256(&compressed) != payload_registry.bundle_sha256
        {
            anyhow::bail!("Hepta gate-pair payload bundle digest drifted");
        }
        let payloads = decode_payload_bundle(&compressed)?;
        if payloads.len() != payload_registry.payload_count {
            anyhow::bail!("Hepta gate-pair payload bundle count drifted");
        }
        let aggregate_source_sha256 = aggregate_payload_sha256(&payloads);
        if aggregate_source_sha256 != payload_registry.aggregate_source_sha256 {
            anyhow::bail!("Hepta gate-pair payload aggregate digest drifted");
        }
        let source_bytes = payloads.values().map(Vec::len).sum::<usize>();
        if source_bytes != payload_registry.source_bytes {
            anyhow::bail!("Hepta gate-pair payload source byte count drifted");
        }
        let long_path_registry: LongPathRegistry = read_json(
            &repo_root.join(LONG_PATH_REGISTRY_PATH),
            "long-path relocation registry",
        )?;
        let long_paths = validate_long_path_registry(repo_root, long_path_registry)?;

        Ok(Self {
            retained_paths: entrypoint_registry.retained_paths.into_iter().collect(),
            payloads,
            long_paths,
        })
    }

    pub(crate) fn is_retained(&self, relative_path: &str) -> bool {
        self.retained_paths.contains(relative_path)
    }

    pub(crate) fn retained_entrypoint_count(&self) -> usize {
        self.retained_paths.len()
    }

    pub(crate) fn payload(&self, relative_path: &str) -> Option<&[u8]> {
        self.payloads.get(relative_path).map(Vec::as_slice)
    }

    pub(crate) fn long_path_entry(&self, relative_path: &str) -> Option<&LongPathEntry> {
        self.long_paths.get(relative_path)
    }

    pub(crate) fn long_script_entries(&self) -> impl Iterator<Item = &LongPathEntry> {
        self.long_paths
            .values()
            .filter(|entry| entry.original_path.starts_with("scripts/"))
    }
}

impl LongPathEntry {
    pub(crate) fn original_path(&self) -> &str {
        &self.original_path
    }

    pub(crate) fn relocated_path(&self) -> &str {
        &self.relocated_path
    }

    pub(crate) fn original_content_sha256(&self) -> &str {
        &self.original_content_sha256
    }
}

fn read_json<T>(path: &Path, label: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("failed to parse Hepta {label}"))
}

fn validate_entrypoint_registry(
    registry: &EntrypointRegistry,
    source_registry_sha256: &str,
) -> Result<()> {
    if registry.schema != ENTRYPOINT_REGISTRY_SCHEMA
        || registry.status != "pruned"
        || registry.source_registry_sha256 != source_registry_sha256
        || registry.pair_count.saturating_mul(2) != registry.original_entrypoint_count
        || registry.retained_entrypoint_count != registry.retained_paths.len()
        || registry.retained_entrypoint_count > registry.max_retained_entrypoints
        || registry.retained_entrypoint_count + registry.virtual_entrypoint_count
            != registry.original_entrypoint_count
        || registry
            .retained_paths
            .iter()
            .any(|path| path.len() > registry.max_tracked_path_bytes)
        || registry.launcher != "scripts/hepta-gate-pair-launch"
        || registry.virtual_paths_sha256.len() != 64
        || registry.virtual_long_path_count > registry.virtual_entrypoint_count
        || !registry.reversible_unpack
    {
        anyhow::bail!("invalid Hepta gate-pair entrypoint registry");
    }
    Ok(())
}

fn validate_payload_registry(
    registry: &PayloadRegistry,
    source_registry_sha256: &str,
) -> Result<()> {
    if registry.schema != PAYLOAD_REGISTRY_SCHEMA
        || registry.status != "packed"
        || registry.source_registry_sha256 != source_registry_sha256
        || registry.registered_payload_count > registry.payload_count
        || registry.bundle_sha256.len() != 64
        || registry.aggregate_source_sha256.len() != 64
        || registry.legacy_archive_bytes == 0
        || registry.legacy_archive_sha256.len() != 64
        || registry.legacy_archive_payload_count != registry.payload_count
        || registry.source_files_materialized
        || !registry.runtime_overlay_generation
        || !registry.reversible_unpack
    {
        anyhow::bail!("invalid Hepta gate-pair payload registry");
    }
    Ok(())
}

fn validate_long_path_registry(
    repo_root: &Path,
    registry: LongPathRegistry,
) -> Result<BTreeMap<String, LongPathEntry>> {
    if registry.schema != LONG_PATH_REGISTRY_SCHEMA
        || registry.status != "relocated"
        || registry.audit_baseline_long_path_count < registry.relocation_count
        || registry.prepack_materialized_long_path_count != registry.relocation_count
        || registry.postpack_materialized_long_path_count != 0
        || registry.max_tracked_path_bytes != 240
        || registry.relocation_count != registry.entries.len()
        || registry.rewritten_reference_file_count != registry.rewritten_reference_paths.len()
        || !registry.reversible_unpack
    {
        anyhow::bail!("invalid Hepta long-path relocation registry");
    }
    let mut entries = BTreeMap::new();
    for entry in registry.entries {
        if entry.original_path.len() <= registry.max_tracked_path_bytes
            || entry.relocated_path.len() > registry.max_tracked_path_bytes
            || entry.original_path_sha256 != sha256(entry.original_path.as_bytes())
            || entry.original_content_sha256.len() != 64
            || entry.relocated_content_sha256.len() != 64
            || entry.mode & !0o777 != 0
        {
            anyhow::bail!(
                "invalid Hepta long-path relocation entry: {}",
                entry.original_path
            );
        }
        let relocated = repo_root.join(&entry.relocated_path);
        let relocated_bytes = fs::read(&relocated)
            .with_context(|| format!("failed to read {}", relocated.display()))?;
        if sha256(&relocated_bytes) != entry.relocated_content_sha256
            || entries.insert(entry.original_path.clone(), entry).is_some()
        {
            anyhow::bail!("Hepta long-path relocation entry drifted");
        }
    }
    Ok(entries)
}

fn decode_payload_bundle(compressed: &[u8]) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut decoder = GzDecoder::new(compressed).take(MAX_PAYLOAD_BUNDLE_BYTES + 1);
    let mut bundle = Vec::new();
    decoder
        .read_to_end(&mut bundle)
        .context("failed to decode Hepta gate-pair payload bundle")?;
    if bundle.len() as u64 > MAX_PAYLOAD_BUNDLE_BYTES {
        anyhow::bail!("Hepta gate-pair payload bundle exceeds byte limit");
    }

    let mut cursor = 0;
    consume_exact(&bundle, &mut cursor, PAYLOAD_BUNDLE_MAGIC)?;
    let payload_count = read_u32(&bundle, &mut cursor)? as usize;
    let mut payloads = BTreeMap::new();
    for _ in 0..payload_count {
        let path_length = read_u32(&bundle, &mut cursor)? as usize;
        let source_length = read_u32(&bundle, &mut cursor)? as usize;
        let path_bytes = read_bytes(&bundle, &mut cursor, path_length)?;
        let expected_sha256 = read_bytes(&bundle, &mut cursor, 32)?;
        let source = read_bytes(&bundle, &mut cursor, source_length)?.to_vec();
        if Sha256::digest(&source).as_slice() != expected_sha256 {
            anyhow::bail!("Hepta gate-pair payload digest mismatch");
        }
        let path = std::str::from_utf8(path_bytes)
            .context("Hepta gate-pair payload path is not UTF-8")?
            .to_string();
        if !is_valid_payload_path(&path) || payloads.insert(path.clone(), source).is_some() {
            anyhow::bail!("invalid or duplicate Hepta gate-pair payload path: {path}");
        }
    }
    if cursor != bundle.len() {
        anyhow::bail!("Hepta gate-pair payload bundle has trailing bytes");
    }
    Ok(payloads)
}

fn consume_exact(bundle: &[u8], cursor: &mut usize, expected: &[u8]) -> Result<()> {
    if read_bytes(bundle, cursor, expected.len())? != expected {
        anyhow::bail!("invalid Hepta gate-pair payload bundle magic");
    }
    Ok(())
}

fn read_u32(bundle: &[u8], cursor: &mut usize) -> Result<u32> {
    let bytes: [u8; 4] = read_bytes(bundle, cursor, 4)?
        .try_into()
        .expect("four-byte slice");
    Ok(u32::from_be_bytes(bytes))
}

fn read_bytes<'a>(bundle: &'a [u8], cursor: &mut usize, length: usize) -> Result<&'a [u8]> {
    let end = cursor
        .checked_add(length)
        .context("Hepta gate-pair payload bundle offset overflow")?;
    let bytes = bundle
        .get(*cursor..end)
        .context("truncated Hepta gate-pair payload bundle")?;
    *cursor = end;
    Ok(bytes)
}

fn is_valid_payload_path(path: &str) -> bool {
    let Some(name) = path.strip_prefix("scripts/lib/hepta-gate-pair-compat-v1/") else {
        return false;
    };
    let Some(stem) = name
        .strip_suffix(".gate")
        .or_else(|| name.strip_suffix(".report"))
    else {
        return false;
    };
    !stem.is_empty()
        && stem
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn aggregate_payload_sha256(payloads: &BTreeMap<String, Vec<u8>>) -> String {
    let mut hasher = Sha256::new();
    for (path, source) in payloads {
        hasher.update(path.as_bytes());
        hasher.update(b"\0");
        hasher.update(sha256(source).as_bytes());
        hasher.update(b"\n");
    }
    hex_digest(hasher.finalize())
}

fn sha256(bytes: &[u8]) -> String {
    hex_digest(Sha256::digest(bytes))
}

fn hex_digest(bytes: impl AsRef<[u8]>) -> String {
    let bytes = bytes.as_ref();
    let mut digest = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        let _ = write!(&mut digest, "{byte:02x}");
    }
    digest
}
