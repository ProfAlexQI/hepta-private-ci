use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use sha2::Digest;
use sha2::Sha256;

mod canonical_source;
mod normalized_payload_bundle;

const ENTRYPOINT_REGISTRY_PATH: &str = "scripts/hepta-gate-pair-entrypoints-v2.json";
const PAYLOAD_REGISTRY_PATH: &str = "scripts/lib/hepta-gate-pair-compat-v2/registry.json";
const PAYLOAD_BUNDLE_PATH: &str = "scripts/lib/hepta-gate-pair-compat-v2/payloads.bundle.gz";
const LONG_PATH_REGISTRY_PATH: &str = "scripts/lib/hepta-long-path-v1/registry.json";
const ENTRYPOINT_REGISTRY_SCHEMA: &str = "hepta_gate_pair_entrypoints_v2";
const PAYLOAD_REGISTRY_SCHEMA: &str = "hepta_gate_pair_normalized_payload_bundle_v3";
const LONG_PATH_REGISTRY_SCHEMA: &str = "hepta_long_path_relocation_v1";

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
    normalized_virtual_entrypoint_count: usize,
    normalized_virtual_entrypoint_limit: usize,
    virtual_entrypoint_template_kinds: Vec<String>,
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
    normalized_payload_count: usize,
    normalized_payload_limit: usize,
    normalized_parameter_row_count: usize,
    normalized_effective_lines: usize,
    normalized_effective_line_limit: usize,
    registered_payload_count: usize,
    source_bytes: usize,
    aggregate_source_sha256: String,
    bundle_bytes: usize,
    bundle_sha256: String,
    source_registry_sha256: String,
    canonical_schema_sha256: String,
    canonical_templates_sha256: String,
    canonical_parameters_sha256: String,
    canonical_max_record_bytes: usize,
    canonical_max_record_byte_limit: usize,
    canonical_record_byte_semantics: String,
    legacy_blob_generation: String,
    legacy_blob_count: usize,
    legacy_blob_aggregate_sha256: String,
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
        canonical_source::validate(repo_root, &payload_registry)?;

        let bundle_path = repo_root.join(PAYLOAD_BUNDLE_PATH);
        let compressed = fs::read(&bundle_path)
            .with_context(|| format!("failed to read {}", bundle_path.display()))?;
        if compressed.len() != payload_registry.bundle_bytes
            || sha256(&compressed) != payload_registry.bundle_sha256
        {
            anyhow::bail!("Hepta gate-pair payload bundle digest drifted");
        }
        let (payloads, normalized_payload_count) = normalized_payload_bundle::decode(&compressed)?;
        if payloads.len() != payload_registry.payload_count {
            anyhow::bail!("Hepta gate-pair payload bundle count drifted");
        }
        if normalized_payload_count != payload_registry.normalized_payload_count {
            anyhow::bail!("Hepta normalized gate-pair payload count drifted");
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
        || registry.normalized_virtual_entrypoint_count == 0
        || registry.normalized_virtual_entrypoint_count
            > registry.normalized_virtual_entrypoint_limit
        || registry.virtual_entrypoint_template_kinds != ["gate", "report"]
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
        || registry.status != "normalized"
        || registry.source_registry_sha256 != source_registry_sha256
        || registry.registered_payload_count > registry.payload_count
        || registry.normalized_payload_count == 0
        || registry.normalized_payload_count > registry.normalized_payload_limit
        || registry.normalized_parameter_row_count != registry.payload_count
        || registry.normalized_effective_lines == 0
        || registry.normalized_effective_lines > registry.normalized_effective_line_limit
        || registry.bundle_sha256.len() != 64
        || registry.aggregate_source_sha256.len() != 64
        || registry.legacy_blob_generation != "deterministic_raw_or_gzip_default_mtime_zero_v1"
        || registry.legacy_blob_count != registry.payload_count
        || registry.legacy_blob_aggregate_sha256.len() != 64
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
