use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;

use super::PAYLOAD_REGISTRY_SCHEMA;
use super::PayloadRegistry;
use super::sha256;

const SCHEMA_PATH: &str = "scripts/lib/hepta-gate-pair-compat-v2/source.schema.json";
const TEMPLATES_PATH: &str = "scripts/lib/hepta-gate-pair-compat-v2/templates.jsonl";
const PARAMETERS_PATH: &str = "scripts/lib/hepta-gate-pair-compat-v2/parameters.jsonl";
const SOURCE_SCHEMA: &str = "hepta_readable_normalized_token_source_v1";
const SOURCE_DOMAIN: &str = "hepta_gate_pair_compat_payloads";
const RECORD_BYTE_SEMANTICS: &str = "record_including_lf";
const MAX_RECORD_BYTE_LIMIT: usize = 355_619;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceSchema {
    schema: String,
    domain: String,
    artifact_schema: String,
    canonical_files: Vec<String>,
    generated_artifact: String,
    template_count: usize,
    parameter_row_count: usize,
    source_bytes: usize,
    source_lines: usize,
    aggregate_source_sha256: String,
    templates_sha256: String,
    parameters_sha256: String,
    jsonl_record_byte_semantics: String,
    templates_max_record_bytes: usize,
    parameters_max_record_bytes: usize,
    canonical_max_record_bytes: usize,
    normalized_effective_lines: usize,
    gzip_profile: GzipProfile,
    source_is_canonical: bool,
    artifact_is_generated: bool,
    exact_reassembly: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GzipProfile {
    compression: String,
    mtime: u32,
    os: u8,
}

pub(super) fn validate(repo_root: &Path, registry: &PayloadRegistry) -> Result<()> {
    if registry.canonical_max_record_bytes == 0
        || registry.canonical_max_record_bytes != registry.canonical_max_record_byte_limit
        || registry.canonical_max_record_byte_limit != MAX_RECORD_BYTE_LIMIT
        || registry.canonical_record_byte_semantics != RECORD_BYTE_SEMANTICS
    {
        anyhow::bail!("invalid Hepta gate-pair canonical registry contract");
    }

    let schema_bytes = read(repo_root, SCHEMA_PATH)?;
    if sha256(&schema_bytes) != registry.canonical_schema_sha256 {
        anyhow::bail!("Hepta gate-pair canonical source schema digest drifted");
    }
    let schema: SourceSchema = serde_json::from_slice(&schema_bytes)
        .context("failed to parse Hepta gate-pair canonical source schema")?;

    let templates = read(repo_root, TEMPLATES_PATH)?;
    if sha256(&templates) != registry.canonical_templates_sha256 {
        anyhow::bail!("Hepta gate-pair canonical templates digest drifted");
    }
    let parameters = read(repo_root, PARAMETERS_PATH)?;
    if sha256(&parameters) != registry.canonical_parameters_sha256 {
        anyhow::bail!("Hepta gate-pair canonical parameters digest drifted");
    }

    let (template_count, templates_max_record_bytes) =
        jsonl_record_stats(&templates, "gate-pair canonical templates")?;
    let (parameter_row_count, parameters_max_record_bytes) =
        jsonl_record_stats(&parameters, "gate-pair canonical parameters")?;
    let canonical_max_record_bytes = templates_max_record_bytes.max(parameters_max_record_bytes);

    if schema.schema != SOURCE_SCHEMA
        || schema.domain != SOURCE_DOMAIN
        || schema.artifact_schema != PAYLOAD_REGISTRY_SCHEMA
        || schema.canonical_files != ["templates.jsonl", "parameters.jsonl"]
        || schema.generated_artifact != "payloads.bundle.gz"
        || schema.template_count != template_count
        || schema.template_count != registry.normalized_payload_count
        || schema.parameter_row_count != parameter_row_count
        || schema.parameter_row_count != registry.normalized_parameter_row_count
        || schema.source_bytes != registry.source_bytes
        || schema.source_lines == 0
        || schema.aggregate_source_sha256 != registry.aggregate_source_sha256
        || schema.templates_sha256 != registry.canonical_templates_sha256
        || schema.parameters_sha256 != registry.canonical_parameters_sha256
        || schema.jsonl_record_byte_semantics != RECORD_BYTE_SEMANTICS
        || schema.jsonl_record_byte_semantics != registry.canonical_record_byte_semantics
        || schema.templates_max_record_bytes != templates_max_record_bytes
        || schema.parameters_max_record_bytes != parameters_max_record_bytes
        || schema.canonical_max_record_bytes != canonical_max_record_bytes
        || schema.canonical_max_record_bytes != registry.canonical_max_record_bytes
        || canonical_max_record_bytes > registry.canonical_max_record_byte_limit
        || schema.normalized_effective_lines != registry.normalized_effective_lines
        || schema.gzip_profile.compression != "zlib_best_compression"
        || schema.gzip_profile.mtime != 1
        || schema.gzip_profile.os != 255
        || !schema.source_is_canonical
        || !schema.artifact_is_generated
        || !schema.exact_reassembly
    {
        anyhow::bail!("invalid Hepta gate-pair canonical source contract");
    }
    Ok(())
}

fn read(repo_root: &Path, relative_path: &str) -> Result<Vec<u8>> {
    let path = repo_root.join(relative_path);
    fs::read(&path).with_context(|| format!("failed to read {}", path.display()))
}

fn jsonl_record_stats(bytes: &[u8], label: &str) -> Result<(usize, usize)> {
    if bytes.is_empty() || bytes.last() != Some(&b'\n') {
        anyhow::bail!("Hepta {label} must be non-empty and LF-terminated");
    }
    let mut record_count = 0usize;
    let mut max_record_bytes = 0usize;
    let mut record_start = 0usize;
    for (index, byte) in bytes.iter().enumerate() {
        if *byte != b'\n' {
            continue;
        }
        let record_bytes = index + 1 - record_start;
        if record_bytes == 1 {
            anyhow::bail!("Hepta {label} contains an empty JSONL record");
        }
        record_count += 1;
        max_record_bytes = max_record_bytes.max(record_bytes);
        record_start = index + 1;
    }
    if record_start != bytes.len() {
        anyhow::bail!("Hepta {label} record framing drifted");
    }
    Ok((record_count, max_record_bytes))
}
