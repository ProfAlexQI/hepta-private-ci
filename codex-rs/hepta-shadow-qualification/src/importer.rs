use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::CompletedPreSend;
use crate::QualificationError;
use crate::Surface;
use crate::digest::framed_digest;
use crate::digest::sha256;
use crate::durable::read_private_bounded;
use crate::durable::sync_directory;
use crate::durable::verify_private_directory;
use crate::durable::write_private_new;
use crate::request::canonical_json;
use crate::request::parse_request;

const MAX_RECEIPT_BYTES: usize = 16 * 1024;
const MAX_REQUEST_BYTES: usize = 64 * 1024;
const EVIDENCE_SET_DOMAIN: &[u8] = b"hepta-live-product-shadow-evidence-set:v2";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ImportFailure {
    pub artifact: String,
    pub reason: String,
}

#[derive(Debug)]
pub struct ImportCheckpoint {
    checkpoint_sha256: String,
    evidence_set_sha256: String,
    failures: Vec<ImportFailure>,
    run_id: String,
    run_root: PathBuf,
    verified_count: usize,
}

impl ImportCheckpoint {
    pub fn create(completed: &CompletedPreSend) -> Result<Self, QualificationError> {
        verify_private_directory(completed.run_root())?;
        let mut failures = inventory_failures(completed.run_root())?;
        let mut evidence = Vec::with_capacity(4);
        for surface in [Surface::AppServer, Surface::Mcp] {
            for ordinal in 1..=2 {
                match verify_one(completed, surface, ordinal) {
                    Ok(digest) => evidence.push(digest),
                    Err(reason) => failures.push(ImportFailure {
                        artifact: format!("{}-{ordinal:02}", surface.as_str()),
                        reason,
                    }),
                }
            }
        }
        failures.sort_by(|left, right| {
            left.artifact
                .cmp(&right.artifact)
                .then(left.reason.cmp(&right.reason))
        });
        let verified_count = evidence.len();
        let failure_bytes = canonical_json(&failures)?;
        let evidence_set_sha256 = framed_digest(
            EVIDENCE_SET_DOMAIN,
            evidence
                .iter()
                .map(String::as_bytes)
                .chain(std::iter::once(failure_bytes.as_slice())),
        );
        let document = CheckpointDocument {
            authority: false,
            enforce: false,
            evidence_set_sha256: &evidence_set_sha256,
            failures: &failures,
            observed_artifact_count: 4,
            outbound: false,
            promotion: false,
            run_id: completed.run_id(),
            schema: "hepta_shadow_qualification_import_checkpoint_v2",
            schema_version: 2,
            status: if failures.is_empty() {
                "complete"
            } else {
                "failed"
            },
            verified_artifact_count: verified_count,
        };
        let checkpoint_bytes = canonical_json(&document)?;
        let checkpoint_sha256 = sha256(&checkpoint_bytes);
        write_private_new(
            &completed.run_root().join("import-checkpoint.json"),
            &checkpoint_bytes,
        )?;
        sync_directory(completed.run_root())?;
        Ok(Self {
            checkpoint_sha256,
            evidence_set_sha256,
            failures,
            run_id: completed.run_id().to_string(),
            run_root: completed.run_root().to_path_buf(),
            verified_count,
        })
    }

    pub fn checkpoint_sha256(&self) -> &str {
        &self.checkpoint_sha256
    }

    pub fn evidence_set_sha256(&self) -> &str {
        &self.evidence_set_sha256
    }

    pub fn failures(&self) -> &[ImportFailure] {
        &self.failures
    }

    pub fn is_complete(&self) -> bool {
        self.failures.is_empty() && self.verified_count == 4
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn run_root(&self) -> &Path {
        &self.run_root
    }

    pub fn verified_count(&self) -> usize {
        self.verified_count
    }
}

#[derive(Serialize)]
struct CheckpointDocument<'a> {
    authority: bool,
    enforce: bool,
    evidence_set_sha256: &'a str,
    failures: &'a [ImportFailure],
    observed_artifact_count: usize,
    outbound: bool,
    promotion: bool,
    run_id: &'a str,
    schema: &'static str,
    schema_version: u32,
    status: &'static str,
    verified_artifact_count: usize,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredReceipt {
    allow_send_token_sha256: String,
    artifact_binding_sha256: String,
    authority: bool,
    durability_scope: String,
    enforce: bool,
    intent_chain_sha256: String,
    intent_id: String,
    ordinal: u8,
    outbound: bool,
    product_http_pre_send_claimed: bool,
    promotion: bool,
    provider_semantic_sha256: String,
    raw_path_sha256: String,
    raw_request_body_sha256: String,
    raw_request_size_bytes: usize,
    raw_request_wire_sha256: String,
    run_id: String,
    sample_token_sha256: String,
    schema: String,
    schema_version: u32,
    surface: Surface,
}

fn verify_one(
    completed: &CompletedPreSend,
    surface: Surface,
    ordinal: u8,
) -> Result<String, String> {
    let stem = format!("{}-{ordinal:02}", surface.as_str());
    let raw_path = completed.run_root().join(format!("{stem}.raw.json"));
    let receipt_path = completed.run_root().join(format!("{stem}.pre-send.json"));
    let raw = read_private_bounded(&raw_path, MAX_REQUEST_BYTES).map_err(error_string)?;
    let receipt_bytes =
        read_private_bounded(&receipt_path, MAX_RECEIPT_BYTES).map_err(error_string)?;
    let receipt: StoredReceipt = serde_json::from_slice(&receipt_bytes)
        .map_err(|error| format!("invalid receipt JSON: {error}"))?;
    if canonical_json(&receipt).map_err(error_string)? != receipt_bytes {
        return Err("receipt is not canonical JSON".to_string());
    }
    let parsed = parse_request(surface, ordinal, &raw, completed.expected_work_directory())
        .map_err(error_string)?;
    let token = completed
        .token(surface, ordinal)
        .ok_or_else(|| "opaque durable token is missing".to_string())?;
    let raw_path_sha256 = sha256(raw_path.to_string_lossy().as_bytes());
    let valid = receipt.schema == "hepta_shadow_qualification_driver_pre_send_v2"
        && receipt.schema_version == 2
        && receipt.run_id == completed.run_id()
        && receipt.surface == surface
        && receipt.ordinal == ordinal
        && !receipt.authority
        && !receipt.enforce
        && !receipt.promotion
        && !receipt.outbound
        && !receipt.product_http_pre_send_claimed
        && receipt.durability_scope == "create_new_file_fsync_then_parent_fsync_before_token"
        && receipt.raw_path_sha256 == raw_path_sha256
        && receipt.raw_request_wire_sha256 == sha256(&raw)
        && receipt.raw_request_body_sha256 == parsed.body_sha256
        && receipt.raw_request_size_bytes == raw.len()
        && receipt.provider_semantic_sha256 == parsed.provider_semantic_sha256
        && receipt.sample_token_sha256 == parsed.sample_token_sha256
        && receipt.allow_send_token_sha256 == token.token_sha256()
        && digest_shape(&receipt.intent_id)
        && digest_shape(&receipt.intent_chain_sha256)
        && digest_shape(&receipt.artifact_binding_sha256);
    if !valid {
        return Err("receipt binding differs from its raw request or opaque token".to_string());
    }
    let raw_sha256 = sha256(&raw);
    let receipt_sha256 = sha256(&receipt_bytes);
    Ok(framed_digest(
        b"hepta-shadow-imported-artifact:v2",
        [raw_sha256.as_bytes(), receipt_sha256.as_bytes()],
    ))
}

fn inventory_failures(run_root: &Path) -> Result<Vec<ImportFailure>, QualificationError> {
    let mut allowed = BTreeSet::from([
        "http".to_string(),
        "qualification-manifest.json".to_string(),
        "run.json".to_string(),
    ]);
    for surface in [Surface::AppServer, Surface::Mcp] {
        for ordinal in 1..=2 {
            let stem = format!("{}-{ordinal:02}", surface.as_str());
            allowed.insert(format!("{stem}.raw.json"));
            allowed.insert(format!("{stem}.pre-send.json"));
        }
    }
    let mut failures = Vec::new();
    for entry in std::fs::read_dir(run_root)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if !allowed.contains(&name) {
            failures.push(ImportFailure {
                artifact: name,
                reason: "unexpected run-root entry".to_string(),
            });
        }
    }
    Ok(failures)
}

fn digest_shape(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn error_string(error: impl std::fmt::Display) -> String {
    error.to_string()
}
