use std::collections::BTreeSet;

use codex_hepta_contracts::GovernanceMode;
use codex_hepta_contracts::Sha256Digest;
use futures::TryStreamExt;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use sqlx::Row;
use sqlx::SqliteConnection;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteRow;

use crate::AppendDisposition;
use crate::EvidenceError;
use crate::HeptaEvidenceStore;
use crate::canonical::canonical_json;
use crate::schema_validation::classify_sqlx_error;
use crate::store::now_millis;

pub const FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

const BINDING_DOMAIN: &[u8] = b"hepta-frozen-oracle-qualification-binding:v1";
const OBSERVATION_DOMAIN: &[u8] = b"hepta-frozen-oracle-conformance-observation:v1";
const TERMINAL_DOMAIN: &[u8] = b"hepta-frozen-oracle-qualification-terminal:v1";
const QUALIFICATION_RUN_ID_PREFIX: &str = "frozen-oracle-qualification:v1:";
const TERMINAL_ID_PREFIX: &str = "frozen-oracle-qualification-terminal:v1:";
const PINNED_FROZEN_ORACLE_CORPUS_BYTES: &[u8] =
    include_bytes!("../fixtures/frozen_oracle_conformance_2f704_v2.json");
const PINNED_FROZEN_ORACLE_CORPUS_SHA256: &str =
    "6fbf5ef6eba851554f1c342fc6e262ff17c51dc58033ed2ca5d86dcafb7d804f";
const PINNED_FROZEN_ORACLE_COMMIT: &str = "2f704dc7c1172cefca908852456beccf4d02a5d1";
const PINNED_FROZEN_ORACLE_TREE: &str = "7be9a382b2610790838eef874cb4d381b5025490";
const PINNED_FROZEN_ORACLE_MANIFEST_SHA256: &str =
    "2c82d45303e912b92a7b9ac31da4661197e59a5ca415d3c70375b49169691377";
const PINNED_FROZEN_ORACLE_GENERATOR_SHA256: &str =
    "42e15311f4cfaf82254e0feaf6a3628ad74de4708c96d187c0dfce06cde11242";
const PINNED_FROZEN_ORACLE_SAMPLE_COUNT: usize = 252;
const MAX_FROZEN_ORACLE_CORPUS_BYTES: usize = 1_048_576;
const MAX_EXTENSION_CALLBACK_PROJECTION_BYTES: usize = 65_536;
const MAX_BINDING_JSON_BYTES: usize = 16_384;
const MAX_OBSERVATION_JSON_BYTES: usize = 16_384;
const MAX_TERMINAL_JSON_BYTES: usize = 16_384;
const BINDING_VERIFY_PAGE_SIZE: i64 = 128;

pub fn pinned_frozen_oracle_corpus_bytes() -> &'static [u8] {
    PINNED_FROZEN_ORACLE_CORPUS_BYTES
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrozenOracleCorpusDocument {
    canonical_encoding: String,
    canonical_object: String,
    cases: Vec<FrozenOracleCorpusCaseDocument>,
    formula: String,
    generator: FrozenOracleCorpusGeneratorDocument,
    identity_pattern: FrozenOracleCorpusIdentityPattern,
    live_product_shadow_collector_completed: bool,
    oracle_commit: String,
    oracle_manifest_sha256: String,
    oracle_tree: String,
    schema: String,
    scope: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrozenOracleCorpusCaseDocument {
    dimensions: FrozenOracleCorpusDimensions,
    expected_lifecycle_projection: serde_json::Value,
    expected_output_sha256: String,
    ordinal: u64,
    sample_id_sha256: String,
}

#[derive(Clone, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(deny_unknown_fields)]
struct FrozenOracleCorpusDimensions {
    host_accepted: bool,
    outcome_kind: String,
    payload_kind: String,
    receipt_phase: String,
    source_kind: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrozenOracleCorpusGeneratorDocument {
    entrypoint: String,
    lifecycle: String,
    schema: String,
    source_digest_computed_from_executing_bytes: bool,
    source_sha256: String,
    version: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrozenOracleCorpusIdentityPattern {
    call_id: String,
    thread_id: String,
    tool_name: String,
    turn_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct VerifiedFrozenOracleCase {
    ordinal: u64,
    sample_id_sha256: Sha256Digest,
    canonical_oracle_output_sha256: Sha256Digest,
}

/// Opaque proof that bytes matched the one library-pinned v2 frozen corpus.
///
/// This capability cannot be deserialized or constructed from caller-provided
/// digests. Cloning preserves the same verified, immutable corpus identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedFrozenOracleCorpus {
    corpus_sha256: Sha256Digest,
    cases: Vec<VerifiedFrozenOracleCase>,
}

impl VerifiedFrozenOracleCorpus {
    /// Loads only the library-pinned v2 corpus from bounded JSON bytes.
    pub fn load(bytes: &[u8]) -> Result<Self, EvidenceError> {
        if bytes.len() > MAX_FROZEN_ORACLE_CORPUS_BYTES {
            return invalid_value(format!(
                "frozen-oracle corpus exceeds the {MAX_FROZEN_ORACLE_CORPUS_BYTES}-byte cap"
            ));
        }
        let corpus_sha256 = Sha256Digest::for_bytes(bytes);
        if corpus_sha256.as_str() != PINNED_FROZEN_ORACLE_CORPUS_SHA256 {
            return invalid_value("frozen-oracle corpus bytes do not match the library pin");
        }
        let document: FrozenOracleCorpusDocument =
            serde_json::from_slice(bytes).map_err(|error| {
                EvidenceError::InvalidRecord(format!("invalid frozen-oracle corpus JSON: {error}"))
            })?;
        validate_pinned_corpus_document(&document)?;

        let mut cases = Vec::with_capacity(PINNED_FROZEN_ORACLE_SAMPLE_COUNT);
        let mut sample_ids = BTreeSet::new();
        let mut output_ids = BTreeSet::new();
        let mut dimension_keys = BTreeSet::new();
        for (index, case) in document.cases.into_iter().enumerate() {
            let expected_ordinal = index as u64 + 1;
            if case.ordinal != expected_ordinal {
                return invalid_value(format!(
                    "frozen-oracle corpus ordinal {} is not contiguous at position {expected_ordinal}",
                    case.ordinal
                ));
            }
            validate_corpus_dimensions(&case.dimensions)?;
            if !dimension_keys.insert(case.dimensions.clone()) {
                return invalid_value("frozen-oracle corpus repeats a dimension tuple");
            }
            let sample_id_sha256 = Sha256Digest::parse(case.sample_id_sha256).map_err(|error| {
                EvidenceError::InvalidRecord(format!("invalid frozen-oracle sample id: {error}"))
            })?;
            let sample_label = format!(
                "hepta-frozen-oracle-conformance:v2:{}:{}:{}:{}:{}",
                case.dimensions.payload_kind,
                case.dimensions.receipt_phase,
                case.dimensions.source_kind,
                case.dimensions.outcome_kind,
                case.dimensions.host_accepted
            );
            if sample_id_sha256 != Sha256Digest::for_bytes(sample_label.as_bytes())
                || !sample_ids.insert(sample_id_sha256.as_str().to_string())
            {
                return invalid_value("frozen-oracle corpus has an invalid or duplicate sample id");
            }
            let canonical_projection = canonical_json(&case.expected_lifecycle_projection)?;
            if canonical_projection.len() > MAX_EXTENSION_CALLBACK_PROJECTION_BYTES {
                return invalid_value(
                    "frozen-oracle extension callback projection exceeds its byte cap",
                );
            }
            let canonical_oracle_output_sha256 = Sha256Digest::for_bytes(&canonical_projection);
            let declared_output_sha256 =
                Sha256Digest::parse(case.expected_output_sha256).map_err(|error| {
                    EvidenceError::InvalidRecord(format!(
                        "invalid frozen-oracle output digest: {error}"
                    ))
                })?;
            if declared_output_sha256 != canonical_oracle_output_sha256
                || !output_ids.insert(declared_output_sha256.as_str().to_string())
            {
                return invalid_value(
                    "frozen-oracle declared output does not match its canonical projection or is duplicated",
                );
            }
            cases.push(VerifiedFrozenOracleCase {
                ordinal: case.ordinal,
                sample_id_sha256,
                canonical_oracle_output_sha256,
            });
        }
        if cases.len() != PINNED_FROZEN_ORACLE_SAMPLE_COUNT
            || dimension_keys.len() != PINNED_FROZEN_ORACLE_SAMPLE_COUNT
        {
            return invalid_value(
                "frozen-oracle corpus does not contain the complete 252-case Cartesian product",
            );
        }
        Ok(Self {
            corpus_sha256,
            cases,
        })
    }

    pub fn corpus_sha256(&self) -> &Sha256Digest {
        &self.corpus_sha256
    }

    pub fn sample_count(&self) -> u64 {
        self.cases.len() as u64
    }

    pub fn frozen_oracle_commit(&self) -> &'static str {
        PINNED_FROZEN_ORACLE_COMMIT
    }

    pub fn frozen_oracle_tree(&self) -> &'static str {
        PINNED_FROZEN_ORACLE_TREE
    }

    pub fn frozen_oracle_manifest_sha256(&self) -> Sha256Digest {
        pinned_digest(PINNED_FROZEN_ORACLE_MANIFEST_SHA256)
    }

    pub fn qualification_plan(
        &self,
        candidate_commit: impl Into<String>,
        candidate_tree: impl Into<String>,
        qualification_nonce_sha256: Sha256Digest,
    ) -> Result<FrozenOracleQualificationPlan, EvidenceError> {
        Ok(FrozenOracleQualificationPlan {
            binding: FrozenOracleQualificationBinding::new(
                self,
                candidate_commit,
                candidate_tree,
                qualification_nonce_sha256,
            )?,
        })
    }

    fn case(&self, ordinal: u64) -> Result<&VerifiedFrozenOracleCase, EvidenceError> {
        let index = usize::try_from(ordinal)
            .ok()
            .and_then(|ordinal| ordinal.checked_sub(1))
            .ok_or_else(|| {
                EvidenceError::InvalidRecord("frozen-oracle ordinal must be positive".to_string())
            })?;
        self.cases.get(index).ok_or_else(|| {
            EvidenceError::InvalidRecord(format!(
                "frozen-oracle ordinal {ordinal} exceeds the pinned {}-case corpus",
                self.cases.len()
            ))
        })
    }

    fn validate_binding(
        &self,
        binding: &FrozenOracleQualificationBinding,
    ) -> Result<(), EvidenceError> {
        validate_binding(binding)?;
        if binding.frozen_oracle_commit != PINNED_FROZEN_ORACLE_COMMIT
            || binding.frozen_oracle_tree != PINNED_FROZEN_ORACLE_TREE
            || binding.frozen_oracle_manifest_sha256.as_str()
                != PINNED_FROZEN_ORACLE_MANIFEST_SHA256
            || binding.canonical_oracle_corpus_sha256 != self.corpus_sha256
            || binding.required_sample_count != self.sample_count()
        {
            return invalid(
                "frozen-oracle qualification binding does not match the verified corpus capability",
            );
        }
        Ok(())
    }
}

fn validate_pinned_corpus_document(
    document: &FrozenOracleCorpusDocument,
) -> Result<(), EvidenceError> {
    if document.schema != "hepta_frozen_oracle_conformance_corpus_v2"
        || document.scope != "bounded_offline_frozen_oracle_qualification_conformance"
        || document.canonical_encoding != "compact_utf8_json_recursive_lexicographic_object_keys"
        || document.canonical_object != "hepta_governance_durable_lifecycle_projection_v1"
        || document.formula
            != "3_payloads_x_2_receipt_phases_x_3_sources_x_7_outcomes_x_2_host_accepted"
        || document.live_product_shadow_collector_completed
        || document.oracle_commit != PINNED_FROZEN_ORACLE_COMMIT
        || document.oracle_tree != PINNED_FROZEN_ORACLE_TREE
        || document.oracle_manifest_sha256 != PINNED_FROZEN_ORACLE_MANIFEST_SHA256
        || document.cases.len() != PINNED_FROZEN_ORACLE_SAMPLE_COUNT
        || document.generator.schema != "hepta_frozen_oracle_conformance_generator_v2"
        || document.generator.version != 2
        || document.generator.source_sha256 != PINNED_FROZEN_ORACLE_GENERATOR_SHA256
        || !document
            .generator
            .source_digest_computed_from_executing_bytes
        || document.generator.entrypoint != "tests::emit_frozen_oracle_conformance_corpus_v2"
        || document.generator.lifecycle
            != "HeptaGovernanceExtension.ToolPolicyContributor.admit_authorize_on_terminal_then_durable_read"
        || document.identity_pattern.thread_id != "thread-oracle"
        || document.identity_pattern.turn_id != "turn-oracle"
        || document.identity_pattern.call_id != "frozen-oracle-case-{ordinal:03}"
        || document.identity_pattern.tool_name != "exec_command"
    {
        return invalid(
            "frozen-oracle corpus metadata does not match the library-pinned v2 identity",
        );
    }
    Ok(())
}

fn validate_corpus_dimensions(
    dimensions: &FrozenOracleCorpusDimensions,
) -> Result<(), EvidenceError> {
    if !matches!(
        dimensions.payload_kind.as_str(),
        "function" | "tool_search" | "custom"
    ) || !matches!(
        dimensions.receipt_phase.as_str(),
        "admission_only" | "admission_and_authorization"
    ) || !matches!(
        dimensions.source_kind.as_str(),
        "direct" | "direct_plaintext_message" | "code_mode"
    ) || !matches!(
        dimensions.outcome_kind.as_str(),
        "completed_success"
            | "completed_reported_failure"
            | "blocked"
            | "failed_before_handler"
            | "failed_after_handler"
            | "aborted"
            | "indeterminate"
    ) {
        return invalid("frozen-oracle corpus contains an unknown dimension value");
    }
    Ok(())
}

#[cfg(test)]
#[derive(Clone)]
pub(crate) struct FrozenOracleQualificationLoadSnapshotHook {
    database_path: std::path::PathBuf,
    qualification_run_id: FrozenOracleQualificationRunId,
    pub(crate) binding_loaded: std::sync::Arc<tokio::sync::Notify>,
    pub(crate) resume: std::sync::Arc<tokio::sync::Notify>,
}

#[cfg(test)]
static FROZEN_ORACLE_QUALIFICATION_LOAD_SNAPSHOT_HOOK: std::sync::Mutex<
    Option<FrozenOracleQualificationLoadSnapshotHook>,
> = std::sync::Mutex::new(None);

#[cfg(test)]
pub(crate) fn install_frozen_oracle_qualification_load_snapshot_hook(
    store: &HeptaEvidenceStore,
    qualification_run_id: &FrozenOracleQualificationRunId,
) -> FrozenOracleQualificationLoadSnapshotHook {
    let hook = FrozenOracleQualificationLoadSnapshotHook {
        database_path: store.path().to_path_buf(),
        qualification_run_id: qualification_run_id.clone(),
        binding_loaded: std::sync::Arc::new(tokio::sync::Notify::new()),
        resume: std::sync::Arc::new(tokio::sync::Notify::new()),
    };
    let mut installed = FROZEN_ORACLE_QUALIFICATION_LOAD_SNAPSHOT_HOOK
        .lock()
        .expect("frozen-oracle qualification snapshot hook lock");
    assert!(
        installed.is_none(),
        "frozen-oracle qualification snapshot hook already installed"
    );
    *installed = Some(hook.clone());
    hook
}

#[cfg(test)]
async fn pause_after_frozen_oracle_qualification_binding_select(
    database_path: &std::path::Path,
    qualification_run_id: &FrozenOracleQualificationRunId,
) {
    let hook = {
        let mut installed = FROZEN_ORACLE_QUALIFICATION_LOAD_SNAPSHOT_HOOK
            .lock()
            .expect("frozen-oracle qualification snapshot hook lock");
        if installed.as_ref().is_some_and(|hook| {
            hook.database_path == database_path
                && hook.qualification_run_id == *qualification_run_id
        }) {
            installed.take()
        } else {
            None
        }
    };
    if let Some(hook) = hook {
        hook.binding_loaded.notify_one();
        hook.resume.notified().await;
    }
}

const BINDING_SELECT_BY_RUN: &str = "SELECT
    seq, qualification_run_id, schema_version, candidate_commit, candidate_tree,
    frozen_oracle_commit, frozen_oracle_tree, frozen_oracle_manifest_sha256,
    canonical_oracle_corpus_sha256, qualification_nonce_sha256, required_sample_count,
    qualification_run_started_at_ms, governance_mode, enforce_enabled, qualification_only,
    promotion_authority_granted, outbound_enabled, memory_mutation_enabled,
    proof_authority_enabled, retirement_authority_enabled, binding_json,
    payload_sha256, binding_sha256
 FROM frozen_oracle_qualification_bindings
 WHERE qualification_run_id = ?";

const OBSERVATION_SELECT_BY_RUN: &str = "SELECT
    seq, qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256, ordinal,
    sample_id_sha256, candidate_output_sha256,
    canonical_oracle_output_sha256, canonical_oracle_matched,
    previous_observation_sha256, observation_sha256, schema_version,
    qualification_only, promotion_authority_granted, payload_json, payload_sha256
 FROM frozen_oracle_qualification_observations
 WHERE qualification_run_id = ?
 ORDER BY ordinal ASC";

const OBSERVATION_SELECT_BY_HASH: &str = "SELECT
    seq, qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256, ordinal,
    sample_id_sha256, candidate_output_sha256,
    canonical_oracle_output_sha256, canonical_oracle_matched,
    previous_observation_sha256, observation_sha256, schema_version,
    qualification_only, promotion_authority_granted, payload_json, payload_sha256
 FROM frozen_oracle_qualification_observations
 WHERE observation_sha256 = ?";

const OBSERVATION_SELECT_BY_ORDINAL: &str = "SELECT
    seq, qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256, ordinal,
    sample_id_sha256, candidate_output_sha256,
    canonical_oracle_output_sha256, canonical_oracle_matched,
    previous_observation_sha256, observation_sha256, schema_version,
    qualification_only, promotion_authority_granted, payload_json, payload_sha256
 FROM frozen_oracle_qualification_observations
 WHERE qualification_run_id = ? AND ordinal = ?";

const HEAD_SELECT_BY_RUN: &str = "SELECT
    qualification_run_id, binding_sha256, observation_count,
    canonical_oracle_match_count, canonical_oracle_divergence_count,
    head_observation_sha256
 FROM frozen_oracle_qualification_heads
 WHERE qualification_run_id = ?";

const TERMINAL_SELECT_BY_RUN: &str = "SELECT
    seq, terminal_id, qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256,
    conformance_status, observation_count, canonical_oracle_match_count, canonical_oracle_divergence_count,
    head_observation_sha256, qualification_run_started_at_ms, qualification_run_finished_at_ms, schema_version,
    governance_mode, enforce_enabled, qualification_only,
    promotion_authority_granted, terminal_sha256, payload_json, payload_sha256
 FROM frozen_oracle_qualification_terminals
 WHERE qualification_run_id = ?";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct FrozenOracleQualificationRunId(String);

impl FrozenOracleQualificationRunId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EvidenceError> {
        let value = value.into();
        validate_prefixed_digest(
            "frozen-oracle qualification run id",
            &value,
            QUALIFICATION_RUN_ID_PREFIX,
        )?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct FrozenOracleQualificationTerminalId(String);

impl FrozenOracleQualificationTerminalId {
    fn for_digest(digest: &Sha256Digest) -> Self {
        Self(format!("{TERMINAL_ID_PREFIX}{}", digest.as_str()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenOracleQualificationBinding {
    schema_version: u32,
    candidate_commit: String,
    candidate_tree: String,
    frozen_oracle_commit: String,
    frozen_oracle_tree: String,
    frozen_oracle_manifest_sha256: Sha256Digest,
    canonical_oracle_corpus_sha256: Sha256Digest,
    qualification_nonce_sha256: Sha256Digest,
    required_sample_count: u64,
    qualification_run_started_at_ms: i64,
    governance_mode: GovernanceMode,
    enforce_enabled: bool,
    qualification_only: bool,
    promotion_authority_granted: bool,
    outbound_enabled: bool,
    memory_mutation_enabled: bool,
    proof_authority_enabled: bool,
    retirement_authority_enabled: bool,
}

impl FrozenOracleQualificationBinding {
    fn new(
        corpus: &VerifiedFrozenOracleCorpus,
        candidate_commit: impl Into<String>,
        candidate_tree: impl Into<String>,
        qualification_nonce_sha256: Sha256Digest,
    ) -> Result<Self, EvidenceError> {
        Self::new_at(
            corpus,
            candidate_commit,
            candidate_tree,
            qualification_nonce_sha256,
            now_millis()?,
        )
    }

    fn new_at(
        corpus: &VerifiedFrozenOracleCorpus,
        candidate_commit: impl Into<String>,
        candidate_tree: impl Into<String>,
        qualification_nonce_sha256: Sha256Digest,
        qualification_run_started_at_ms: i64,
    ) -> Result<Self, EvidenceError> {
        let binding = Self {
            schema_version: FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION,
            candidate_commit: candidate_commit.into(),
            candidate_tree: candidate_tree.into(),
            frozen_oracle_commit: corpus.frozen_oracle_commit().to_string(),
            frozen_oracle_tree: corpus.frozen_oracle_tree().to_string(),
            frozen_oracle_manifest_sha256: corpus.frozen_oracle_manifest_sha256(),
            canonical_oracle_corpus_sha256: corpus.corpus_sha256().clone(),
            qualification_nonce_sha256,
            required_sample_count: corpus.sample_count(),
            qualification_run_started_at_ms,
            governance_mode: GovernanceMode::Shadow,
            enforce_enabled: false,
            qualification_only: true,
            promotion_authority_granted: false,
            outbound_enabled: false,
            memory_mutation_enabled: false,
            proof_authority_enabled: false,
            retirement_authority_enabled: false,
        };
        corpus.validate_binding(&binding)?;
        Ok(binding)
    }

    pub fn candidate_commit(&self) -> &str {
        &self.candidate_commit
    }

    pub fn candidate_tree(&self) -> &str {
        &self.candidate_tree
    }

    pub fn frozen_oracle_commit(&self) -> &str {
        &self.frozen_oracle_commit
    }

    pub fn frozen_oracle_tree(&self) -> &str {
        &self.frozen_oracle_tree
    }

    pub fn frozen_oracle_manifest_sha256(&self) -> &Sha256Digest {
        &self.frozen_oracle_manifest_sha256
    }

    pub fn canonical_oracle_corpus_sha256(&self) -> &Sha256Digest {
        &self.canonical_oracle_corpus_sha256
    }

    pub fn required_sample_count(&self) -> u64 {
        self.required_sample_count
    }

    pub fn qualification_run_started_at_ms(&self) -> i64 {
        self.qualification_run_started_at_ms
    }

    pub fn governance_mode(&self) -> GovernanceMode {
        self.governance_mode
    }

    pub fn enforce_enabled(&self) -> bool {
        self.enforce_enabled
    }

    pub fn qualification_only(&self) -> bool {
        self.qualification_only
    }

    pub fn promotion_authority_granted(&self) -> bool {
        self.promotion_authority_granted
    }
}

/// Opaque, replayable plan derived from one verified corpus capability.
///
/// The embedded timestamp names this bounded qualification run; it is not soak
/// duration evidence. The caller cannot supply corpus identity or sample count.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenOracleQualificationPlan {
    pub(crate) binding: FrozenOracleQualificationBinding,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenOracleConformanceRecord {
    pub(crate) schema_version: u32,
    pub(crate) qualification_run_id: FrozenOracleQualificationRunId,
    pub(crate) binding_sha256: Sha256Digest,
    pub(crate) canonical_oracle_corpus_sha256: Sha256Digest,
    pub(crate) ordinal: u64,
    pub(crate) sample_id_sha256: Sha256Digest,
    pub(crate) candidate_output_sha256: Sha256Digest,
    pub(crate) canonical_oracle_output_sha256: Sha256Digest,
    pub(crate) canonical_oracle_matched: bool,
    pub(crate) previous_observation_sha256: Sha256Digest,
    pub(crate) qualification_only: bool,
    pub(crate) promotion_authority_granted: bool,
}

impl FrozenOracleConformanceRecord {
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn qualification_run_id(&self) -> &FrozenOracleQualificationRunId {
        &self.qualification_run_id
    }

    pub fn binding_sha256(&self) -> &Sha256Digest {
        &self.binding_sha256
    }

    pub fn canonical_oracle_corpus_sha256(&self) -> &Sha256Digest {
        &self.canonical_oracle_corpus_sha256
    }

    pub fn ordinal(&self) -> u64 {
        self.ordinal
    }

    pub fn sample_id_sha256(&self) -> &Sha256Digest {
        &self.sample_id_sha256
    }

    pub fn candidate_output_sha256(&self) -> &Sha256Digest {
        &self.candidate_output_sha256
    }

    pub fn canonical_oracle_output_sha256(&self) -> &Sha256Digest {
        &self.canonical_oracle_output_sha256
    }

    pub fn canonical_oracle_matched(&self) -> bool {
        self.canonical_oracle_matched
    }

    pub fn previous_observation_sha256(&self) -> &Sha256Digest {
        &self.previous_observation_sha256
    }

    pub fn qualification_only(&self) -> bool {
        self.qualification_only
    }

    pub fn promotion_authority_granted(&self) -> bool {
        self.promotion_authority_granted
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StoredFrozenOracleConformanceObservation {
    pub(crate) seq: i64,
    pub(crate) observation_sha256: Sha256Digest,
    pub(crate) observation: FrozenOracleConformanceRecord,
}

impl StoredFrozenOracleConformanceObservation {
    /// Non-authoritative database-local ordering metadata.
    pub fn operational_seq(&self) -> i64 {
        self.seq
    }

    pub fn observation_sha256(&self) -> &Sha256Digest {
        &self.observation_sha256
    }

    pub fn observation(&self) -> &FrozenOracleConformanceRecord {
        &self.observation
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum FrozenOracleConformanceStatus {
    Conformant,
    Diverged,
}

impl FrozenOracleConformanceStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conformant => "conformant",
            Self::Diverged => "diverged",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenOracleQualificationTerminalRecord {
    pub(crate) schema_version: u32,
    pub(crate) qualification_run_id: FrozenOracleQualificationRunId,
    pub(crate) binding_sha256: Sha256Digest,
    pub(crate) canonical_oracle_corpus_sha256: Sha256Digest,
    pub(crate) conformance_status: FrozenOracleConformanceStatus,
    pub(crate) observation_count: u64,
    pub(crate) canonical_oracle_match_count: u64,
    pub(crate) canonical_oracle_divergence_count: u64,
    pub(crate) head_observation_sha256: Sha256Digest,
    pub(crate) qualification_run_started_at_ms: i64,
    pub(crate) qualification_run_finished_at_ms: i64,
    pub(crate) governance_mode: GovernanceMode,
    pub(crate) enforce_enabled: bool,
    pub(crate) qualification_only: bool,
    pub(crate) promotion_authority_granted: bool,
}

impl FrozenOracleQualificationTerminalRecord {
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn qualification_run_id(&self) -> &FrozenOracleQualificationRunId {
        &self.qualification_run_id
    }

    pub fn binding_sha256(&self) -> &Sha256Digest {
        &self.binding_sha256
    }

    pub fn canonical_oracle_corpus_sha256(&self) -> &Sha256Digest {
        &self.canonical_oracle_corpus_sha256
    }

    pub fn conformance_status(&self) -> FrozenOracleConformanceStatus {
        self.conformance_status
    }

    pub fn observation_count(&self) -> u64 {
        self.observation_count
    }

    pub fn canonical_oracle_match_count(&self) -> u64 {
        self.canonical_oracle_match_count
    }

    pub fn canonical_oracle_divergence_count(&self) -> u64 {
        self.canonical_oracle_divergence_count
    }

    pub fn head_observation_sha256(&self) -> &Sha256Digest {
        &self.head_observation_sha256
    }

    pub fn qualification_run_started_at_ms(&self) -> i64 {
        self.qualification_run_started_at_ms
    }

    pub fn qualification_run_finished_at_ms(&self) -> i64 {
        self.qualification_run_finished_at_ms
    }

    pub fn governance_mode(&self) -> GovernanceMode {
        self.governance_mode
    }

    pub fn enforce_enabled(&self) -> bool {
        self.enforce_enabled
    }

    pub fn qualification_only(&self) -> bool {
        self.qualification_only
    }

    pub fn promotion_authority_granted(&self) -> bool {
        self.promotion_authority_granted
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StoredFrozenOracleQualificationTerminal {
    pub(crate) seq: i64,
    pub(crate) terminal_id: FrozenOracleQualificationTerminalId,
    pub(crate) terminal_sha256: Sha256Digest,
    pub(crate) terminal: FrozenOracleQualificationTerminalRecord,
}

impl StoredFrozenOracleQualificationTerminal {
    /// Non-authoritative database-local ordering metadata.
    pub fn operational_seq(&self) -> i64 {
        self.seq
    }

    pub fn terminal_id(&self) -> &FrozenOracleQualificationTerminalId {
        &self.terminal_id
    }

    pub fn terminal_sha256(&self) -> &Sha256Digest {
        &self.terminal_sha256
    }

    pub fn terminal(&self) -> &FrozenOracleQualificationTerminalRecord {
        &self.terminal
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenOracleQualificationRegistration {
    pub(crate) qualification_run_id: FrozenOracleQualificationRunId,
    pub(crate) binding_sha256: Sha256Digest,
    pub(crate) disposition: AppendDisposition,
}

impl FrozenOracleQualificationRegistration {
    pub fn qualification_run_id(&self) -> &FrozenOracleQualificationRunId {
        &self.qualification_run_id
    }

    pub fn binding_sha256(&self) -> &Sha256Digest {
        &self.binding_sha256
    }

    pub fn disposition(&self) -> AppendDisposition {
        self.disposition
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenOracleConformanceAppend {
    pub(crate) disposition: AppendDisposition,
    pub(crate) stored: StoredFrozenOracleConformanceObservation,
}

impl FrozenOracleConformanceAppend {
    pub fn disposition(&self) -> AppendDisposition {
        self.disposition
    }

    pub fn stored(&self) -> &StoredFrozenOracleConformanceObservation {
        &self.stored
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FrozenOracleQualificationSummary {
    pub(crate) qualification_run_id: FrozenOracleQualificationRunId,
    pub(crate) binding_sha256: Sha256Digest,
    pub(crate) binding: FrozenOracleQualificationBinding,
    pub(crate) observation_count: u64,
    pub(crate) canonical_oracle_match_count: u64,
    pub(crate) canonical_oracle_divergence_count: u64,
    pub(crate) head_observation_sha256: Sha256Digest,
    pub(crate) terminal: Option<StoredFrozenOracleQualificationTerminal>,
}

impl FrozenOracleQualificationSummary {
    pub fn qualification_run_id(&self) -> &FrozenOracleQualificationRunId {
        &self.qualification_run_id
    }

    pub fn binding_sha256(&self) -> &Sha256Digest {
        &self.binding_sha256
    }

    pub fn binding(&self) -> &FrozenOracleQualificationBinding {
        &self.binding
    }

    pub fn observation_count(&self) -> u64 {
        self.observation_count
    }

    pub fn canonical_oracle_match_count(&self) -> u64 {
        self.canonical_oracle_match_count
    }

    pub fn canonical_oracle_divergence_count(&self) -> u64 {
        self.canonical_oracle_divergence_count
    }

    pub fn head_observation_sha256(&self) -> &Sha256Digest {
        &self.head_observation_sha256
    }

    pub fn terminal(&self) -> Option<&StoredFrozenOracleQualificationTerminal> {
        self.terminal.as_ref()
    }
}

struct BindingMaterial {
    qualification_run_id: FrozenOracleQualificationRunId,
    payload_json: String,
    payload_sha256: Sha256Digest,
    binding_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct QualificationHead {
    qualification_run_id: FrozenOracleQualificationRunId,
    binding_sha256: Sha256Digest,
    observation_count: u64,
    canonical_oracle_match_count: u64,
    canonical_oracle_divergence_count: u64,
    head_observation_sha256: Sha256Digest,
}

struct VerifiedFrozenOracleQualification {
    binding: FrozenOracleQualificationBinding,
    binding_sha256: Sha256Digest,
    observation_count: u64,
    canonical_oracle_match_count: u64,
    canonical_oracle_divergence_count: u64,
    head_observation_sha256: Sha256Digest,
    terminal: Option<StoredFrozenOracleQualificationTerminal>,
}

impl VerifiedFrozenOracleQualification {
    fn summary(
        &self,
        qualification_run_id: FrozenOracleQualificationRunId,
    ) -> FrozenOracleQualificationSummary {
        FrozenOracleQualificationSummary {
            qualification_run_id,
            binding_sha256: self.binding_sha256.clone(),
            binding: self.binding.clone(),
            observation_count: self.observation_count,
            canonical_oracle_match_count: self.canonical_oracle_match_count,
            canonical_oracle_divergence_count: self.canonical_oracle_divergence_count,
            head_observation_sha256: self.head_observation_sha256.clone(),
            terminal: self.terminal.clone(),
        }
    }
}

impl HeptaEvidenceStore {
    /// Registers or exactly replays one finite frozen-oracle qualification run.
    ///
    /// This is bounded conformance evidence, not a live product Shadow soak and
    /// not promotion authority. Idempotent recovery must reuse the same binding.
    pub async fn begin_frozen_oracle_qualification_run(
        &self,
        corpus: &VerifiedFrozenOracleCorpus,
        plan: &FrozenOracleQualificationPlan,
    ) -> Result<FrozenOracleQualificationRegistration, EvidenceError> {
        let binding = &plan.binding;
        corpus.validate_binding(binding)?;
        let material = binding_material(binding)?;
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let insert = sqlx::query(
            "INSERT INTO frozen_oracle_qualification_bindings (
                qualification_run_id, schema_version, candidate_commit, candidate_tree,
                frozen_oracle_commit, frozen_oracle_tree,
                frozen_oracle_manifest_sha256, canonical_oracle_corpus_sha256,
                qualification_nonce_sha256, required_sample_count, qualification_run_started_at_ms,
                governance_mode, enforce_enabled, qualification_only,
                promotion_authority_granted, outbound_enabled,
                memory_mutation_enabled, proof_authority_enabled,
                retirement_authority_enabled, binding_json, payload_sha256,
                binding_sha256, recorded_at_ms
             ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'shadow', 0, 1, 0, 0, 0, 0,
                0, ?, ?, ?, ?
             )
             ON CONFLICT DO NOTHING",
        )
        .bind(material.qualification_run_id.as_str())
        .bind(i64::from(FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION))
        .bind(binding.candidate_commit())
        .bind(binding.candidate_tree())
        .bind(binding.frozen_oracle_commit())
        .bind(binding.frozen_oracle_tree())
        .bind(binding.frozen_oracle_manifest_sha256().as_str())
        .bind(binding.canonical_oracle_corpus_sha256().as_str())
        .bind(binding.qualification_nonce_sha256.as_str())
        .bind(
            i64::try_from(binding.required_sample_count)
                .map_err(|error| EvidenceError::InvalidRecord(error.to_string()))?,
        )
        .bind(binding.qualification_run_started_at_ms)
        .bind(&material.payload_json)
        .bind(material.payload_sha256.as_str())
        .bind(material.binding_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let row = sqlx::query(BINDING_SELECT_BY_RUN)
            .bind(material.qualification_run_id.as_str())
            .fetch_optional(&mut *transaction)
            .await
            .map_err(classify_sqlx_error)?
            .ok_or_else(|| {
                EvidenceError::Corrupt(format!(
                    "frozen-oracle qualification binding {} disappeared after insert",
                    material.qualification_run_id.as_str()
                ))
            })?;
        let (stored_binding, stored_material) = decode_binding_row(&row)?;
        if stored_binding != *binding
            || stored_material.qualification_run_id != material.qualification_run_id
            || stored_material.payload_json != material.payload_json
            || stored_material.payload_sha256 != material.payload_sha256
            || stored_material.binding_sha256 != material.binding_sha256
        {
            return Err(EvidenceError::IdempotencyConflict {
                record_id: material.qualification_run_id.as_str().to_string(),
            });
        }
        if insert.rows_affected() == 1 {
            sqlx::query(
                "INSERT INTO frozen_oracle_qualification_heads (
                    qualification_run_id, binding_sha256, observation_count,
                    canonical_oracle_match_count,
                    canonical_oracle_divergence_count, head_observation_sha256
                 ) VALUES (?, ?, 0, 0, 0, ?)",
            )
            .bind(material.qualification_run_id.as_str())
            .bind(material.binding_sha256.as_str())
            .bind(material.binding_sha256.as_str())
            .execute(&mut *transaction)
            .await
            .map_err(classify_sqlx_error)?;
        }
        let head = load_qualification_head(&mut transaction, &material.qualification_run_id)
            .await?
            .ok_or_else(|| {
                EvidenceError::Corrupt(format!(
                    "frozen-oracle qualification run {} has no append head",
                    material.qualification_run_id.as_str()
                ))
            })?;
        if head.binding_sha256 != material.binding_sha256 {
            return Err(EvidenceError::Corrupt(
                "frozen-oracle qualification append head has the wrong binding".to_string(),
            ));
        }
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(FrozenOracleQualificationRegistration {
            qualification_run_id: material.qualification_run_id,
            binding_sha256: material.binding_sha256,
            disposition: if insert.rows_affected() == 1 {
                AppendDisposition::Inserted
            } else {
                AppendDisposition::AlreadyPresent
            },
        })
    }

    pub async fn append_frozen_oracle_conformance_observation(
        &self,
        qualification_run_id: &FrozenOracleQualificationRunId,
        corpus: &VerifiedFrozenOracleCorpus,
        ordinal: u64,
        candidate_projection: &serde_json::Value,
    ) -> Result<FrozenOracleConformanceAppend, EvidenceError> {
        FrozenOracleQualificationRunId::parse(qualification_run_id.as_str())?;
        let corpus_case = corpus.case(ordinal)?;
        let candidate_projection = canonical_json(candidate_projection)?;
        if candidate_projection.len() > MAX_EXTENSION_CALLBACK_PROJECTION_BYTES {
            return invalid_value("candidate extension callback projection exceeds its byte cap");
        }
        let candidate_output_sha256 = Sha256Digest::for_bytes(&candidate_projection);
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let binding_row = sqlx::query(BINDING_SELECT_BY_RUN)
            .bind(qualification_run_id.as_str())
            .fetch_optional(&mut *transaction)
            .await
            .map_err(classify_sqlx_error)?;
        let Some(binding_row) = binding_row else {
            return Err(EvidenceError::InvalidRecord(format!(
                "frozen-oracle qualification run {} is not registered",
                qualification_run_id.as_str()
            )));
        };
        let (binding, material) = decode_binding_row(&binding_row)?;
        if material.qualification_run_id != *qualification_run_id {
            return Err(EvidenceError::Corrupt(
                "frozen-oracle qualification binding identity drifted".to_string(),
            ));
        }
        corpus.validate_binding(&binding)?;
        let existing = sqlx::query(OBSERVATION_SELECT_BY_ORDINAL)
            .bind(qualification_run_id.as_str())
            .bind(i64::try_from(ordinal).map_err(|error| {
                EvidenceError::InvalidRecord(format!("invalid frozen-oracle ordinal: {error}"))
            })?)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(classify_sqlx_error)?
            .map(|row| decode_observation_row(&row))
            .transpose()?;
        if let Some(existing) = existing {
            if existing.observation.ordinal != ordinal
                || existing.observation.sample_id_sha256 != corpus_case.sample_id_sha256
                || existing.observation.candidate_output_sha256 != candidate_output_sha256
                || existing.observation.canonical_oracle_output_sha256
                    != corpus_case.canonical_oracle_output_sha256
            {
                return Err(EvidenceError::IdempotencyConflict {
                    record_id: format!("{}:{ordinal}", qualification_run_id.as_str()),
                });
            }
            transaction.commit().await.map_err(classify_sqlx_error)?;
            return Ok(FrozenOracleConformanceAppend {
                disposition: AppendDisposition::AlreadyPresent,
                stored: existing,
            });
        }
        if sqlx::query(TERMINAL_SELECT_BY_RUN)
            .bind(qualification_run_id.as_str())
            .fetch_optional(&mut *transaction)
            .await
            .map_err(classify_sqlx_error)?
            .is_some()
        {
            return Err(EvidenceError::InvalidRecord(
                "frozen-oracle qualification run is already terminal".to_string(),
            ));
        }
        let head = load_qualification_head(&mut transaction, qualification_run_id)
            .await?
            .ok_or_else(|| {
                EvidenceError::Corrupt(
                    "frozen-oracle qualification append head is missing".to_string(),
                )
            })?;
        if head.binding_sha256 != material.binding_sha256 {
            return Err(EvidenceError::Corrupt(
                "frozen-oracle qualification append head binding drifted".to_string(),
            ));
        }
        let expected_ordinal = head.observation_count.checked_add(1).ok_or_else(|| {
            EvidenceError::InvalidRecord(
                "frozen-oracle conformance observation ordinal overflow".to_string(),
            )
        })?;
        if ordinal != expected_ordinal {
            return Err(EvidenceError::InvalidRecord(format!(
                "frozen-oracle qualification observations must be appended in exact order; expected {expected_ordinal}, received {ordinal}"
            )));
        }
        let canonical_oracle_matched =
            candidate_output_sha256 == corpus_case.canonical_oracle_output_sha256;
        let observation = FrozenOracleConformanceRecord {
            schema_version: FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION,
            qualification_run_id: qualification_run_id.clone(),
            binding_sha256: material.binding_sha256,
            canonical_oracle_corpus_sha256: binding.canonical_oracle_corpus_sha256.clone(),
            ordinal,
            sample_id_sha256: corpus_case.sample_id_sha256.clone(),
            candidate_output_sha256,
            canonical_oracle_output_sha256: corpus_case.canonical_oracle_output_sha256.clone(),
            canonical_oracle_matched,
            previous_observation_sha256: head.head_observation_sha256,
            qualification_only: true,
            promotion_authority_granted: false,
        };
        validate_observation(&observation)?;
        let payload = canonical_json(&observation)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let payload_sha256 = Sha256Digest::for_bytes(&payload);
        let observation_sha256 = domain_digest(OBSERVATION_DOMAIN, &payload);
        let insert = sqlx::query(
            "INSERT INTO frozen_oracle_qualification_observations (
                qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256,
                ordinal, sample_id_sha256, candidate_output_sha256,
                canonical_oracle_output_sha256, canonical_oracle_matched,
                previous_observation_sha256, observation_sha256, schema_version,
                qualification_only, promotion_authority_granted, payload_json,
                payload_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 0, ?, ?, ?)",
        )
        .bind(qualification_run_id.as_str())
        .bind(observation.binding_sha256.as_str())
        .bind(observation.canonical_oracle_corpus_sha256.as_str())
        .bind(
            i64::try_from(observation.ordinal)
                .map_err(|error| EvidenceError::InvalidRecord(error.to_string()))?,
        )
        .bind(observation.sample_id_sha256.as_str())
        .bind(observation.candidate_output_sha256.as_str())
        .bind(observation.canonical_oracle_output_sha256.as_str())
        .bind(observation.canonical_oracle_matched)
        .bind(observation.previous_observation_sha256.as_str())
        .bind(observation_sha256.as_str())
        .bind(i64::from(FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION))
        .bind(&payload_json)
        .bind(payload_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        if insert.rows_affected() != 1 {
            return Err(EvidenceError::Corrupt(
                "frozen-oracle conformance observation insert affected no row".to_string(),
            ));
        }
        let row = sqlx::query(OBSERVATION_SELECT_BY_HASH)
            .bind(observation_sha256.as_str())
            .fetch_one(&mut *transaction)
            .await
            .map_err(classify_sqlx_error)?;
        let stored = decode_observation_row(&row)?;
        if stored.observation != observation
            || stored.observation_sha256 != observation_sha256
            || row.get::<String, _>("payload_json") != payload_json
            || row.get::<String, _>("payload_sha256") != payload_sha256.as_str()
        {
            return Err(EvidenceError::Corrupt(
                "inserted frozen-oracle conformance observation differs from canonical input"
                    .to_string(),
            ));
        }
        let advanced = load_qualification_head(&mut transaction, qualification_run_id)
            .await?
            .ok_or_else(|| {
                EvidenceError::Corrupt(
                    "frozen-oracle qualification append head disappeared".to_string(),
                )
            })?;
        if advanced.observation_count != ordinal
            || advanced.head_observation_sha256 != observation_sha256
            || advanced.canonical_oracle_match_count
                != head.canonical_oracle_match_count + u64::from(canonical_oracle_matched)
            || advanced.canonical_oracle_divergence_count
                != head.canonical_oracle_divergence_count + u64::from(!canonical_oracle_matched)
        {
            return Err(EvidenceError::Corrupt(
                "frozen-oracle qualification append head did not advance canonically".to_string(),
            ));
        }
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(FrozenOracleConformanceAppend {
            disposition: AppendDisposition::Inserted,
            stored,
        })
    }

    pub async fn finish_frozen_oracle_qualification_run(
        &self,
        qualification_run_id: &FrozenOracleQualificationRunId,
        corpus: &VerifiedFrozenOracleCorpus,
    ) -> Result<StoredFrozenOracleQualificationTerminal, EvidenceError> {
        FrozenOracleQualificationRunId::parse(qualification_run_id.as_str())?;
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let Some(verified) = load_verified_frozen_oracle_qualification(
            &mut transaction,
            qualification_run_id,
            None,
            corpus,
        )
        .await?
        else {
            return Err(EvidenceError::InvalidRecord(format!(
                "frozen-oracle qualification run {} is not registered",
                qualification_run_id.as_str()
            )));
        };
        corpus.validate_binding(&verified.binding)?;
        if let Some(terminal) = verified.terminal {
            transaction.commit().await.map_err(classify_sqlx_error)?;
            return Ok(terminal);
        }
        let observation_count = verified.observation_count;
        if observation_count != verified.binding.required_sample_count {
            return Err(EvidenceError::InvalidRecord(format!(
                "frozen-oracle qualification run requires exactly {} samples before terminal; found {}",
                verified.binding.required_sample_count, observation_count
            )));
        }
        let qualification_run_finished_at_ms = now_millis()?;
        if qualification_run_finished_at_ms < verified.binding.qualification_run_started_at_ms {
            return Err(EvidenceError::Unavailable(
                "system clock moved before frozen-oracle qualification run start time".to_string(),
            ));
        }
        let terminal = FrozenOracleQualificationTerminalRecord {
            schema_version: FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION,
            qualification_run_id: qualification_run_id.clone(),
            binding_sha256: verified.binding_sha256.clone(),
            canonical_oracle_corpus_sha256: verified.binding.canonical_oracle_corpus_sha256.clone(),
            conformance_status: if verified.canonical_oracle_divergence_count == 0 {
                FrozenOracleConformanceStatus::Conformant
            } else {
                FrozenOracleConformanceStatus::Diverged
            },
            observation_count,
            canonical_oracle_match_count: verified.canonical_oracle_match_count,
            canonical_oracle_divergence_count: verified.canonical_oracle_divergence_count,
            head_observation_sha256: verified.head_observation_sha256,
            qualification_run_started_at_ms: verified.binding.qualification_run_started_at_ms,
            qualification_run_finished_at_ms,
            governance_mode: GovernanceMode::Shadow,
            enforce_enabled: false,
            qualification_only: true,
            promotion_authority_granted: false,
        };
        validate_terminal(&terminal)?;
        let payload = canonical_json(&terminal)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let payload_sha256 = Sha256Digest::for_bytes(&payload);
        let terminal_sha256 = domain_digest(TERMINAL_DOMAIN, &payload);
        let terminal_id = FrozenOracleQualificationTerminalId::for_digest(&terminal_sha256);
        let insert = sqlx::query(
            "INSERT INTO frozen_oracle_qualification_terminals (
                terminal_id, qualification_run_id, binding_sha256,
                canonical_oracle_corpus_sha256, conformance_status,
                observation_count, canonical_oracle_match_count, canonical_oracle_divergence_count,
                head_observation_sha256, qualification_run_started_at_ms, qualification_run_finished_at_ms,
                schema_version, governance_mode, enforce_enabled,
                qualification_only, promotion_authority_granted, terminal_sha256,
                payload_json, payload_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'shadow', 0, 1, 0, ?, ?, ?, ?)",
        )
        .bind(terminal_id.as_str())
        .bind(qualification_run_id.as_str())
        .bind(terminal.binding_sha256.as_str())
        .bind(terminal.canonical_oracle_corpus_sha256.as_str())
        .bind(terminal.conformance_status.as_str())
        .bind(
            i64::try_from(terminal.observation_count)
                .map_err(|error| EvidenceError::InvalidRecord(error.to_string()))?,
        )
        .bind(
            i64::try_from(terminal.canonical_oracle_match_count)
                .map_err(|error| EvidenceError::InvalidRecord(error.to_string()))?,
        )
        .bind(
            i64::try_from(terminal.canonical_oracle_divergence_count)
                .map_err(|error| EvidenceError::InvalidRecord(error.to_string()))?,
        )
        .bind(terminal.head_observation_sha256.as_str())
        .bind(terminal.qualification_run_started_at_ms)
        .bind(terminal.qualification_run_finished_at_ms)
        .bind(i64::from(FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION))
        .bind(terminal_sha256.as_str())
        .bind(&payload_json)
        .bind(payload_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        if insert.rows_affected() != 1 {
            return Err(EvidenceError::Corrupt(
                "frozen-oracle qualification terminal insert affected no row".to_string(),
            ));
        }
        let row = sqlx::query(TERMINAL_SELECT_BY_RUN)
            .bind(qualification_run_id.as_str())
            .fetch_one(&mut *transaction)
            .await
            .map_err(classify_sqlx_error)?;
        let stored = decode_terminal_row(&row)?;
        if stored.terminal != terminal
            || stored.terminal_id != terminal_id
            || stored.terminal_sha256 != terminal_sha256
            || row.get::<String, _>("payload_json") != payload_json
            || row.get::<String, _>("payload_sha256") != payload_sha256.as_str()
        {
            return Err(EvidenceError::Corrupt(
                "inserted frozen-oracle qualification terminal differs from canonical input"
                    .to_string(),
            ));
        }
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(stored)
    }

    pub async fn get_frozen_oracle_qualification_run_summary(
        &self,
        qualification_run_id: &FrozenOracleQualificationRunId,
        corpus: &VerifiedFrozenOracleCorpus,
    ) -> Result<Option<FrozenOracleQualificationSummary>, EvidenceError> {
        FrozenOracleQualificationRunId::parse(qualification_run_id.as_str())?;
        let mut transaction = self.pool.begin().await.map_err(classify_sqlx_error)?;
        let summary = load_verified_frozen_oracle_qualification(
            &mut transaction,
            qualification_run_id,
            Some(self.path()),
            corpus,
        )
        .await?
        .map(|verified| verified.summary(qualification_run_id.clone()));
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(summary)
    }
}

fn binding_material(
    binding: &FrozenOracleQualificationBinding,
) -> Result<BindingMaterial, EvidenceError> {
    validate_binding(binding)?;
    let payload = canonical_json(binding)?;
    let payload_json = String::from_utf8(payload.clone())
        .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
    let payload_sha256 = Sha256Digest::for_bytes(&payload);
    let binding_sha256 = domain_digest(BINDING_DOMAIN, &payload);
    let qualification_run_id = FrozenOracleQualificationRunId(format!(
        "{QUALIFICATION_RUN_ID_PREFIX}{}",
        binding_sha256.as_str()
    ));
    Ok(BindingMaterial {
        qualification_run_id,
        payload_json,
        payload_sha256,
        binding_sha256,
    })
}

fn domain_digest(domain: &[u8], payload: &[u8]) -> Sha256Digest {
    let mut hasher = Sha256::new();
    for part in [domain, payload] {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part);
    }
    match Sha256Digest::parse(format!("{:x}", hasher.finalize())) {
        Ok(digest) => digest,
        Err(error) => panic!("SHA-256 formatting violated its lowercase digest invariant: {error}"),
    }
}

fn validate_binding(binding: &FrozenOracleQualificationBinding) -> Result<(), EvidenceError> {
    if binding.schema_version != FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION {
        return invalid("unsupported frozen-oracle qualification run schema version");
    }
    for (label, value) in [
        ("candidate commit", binding.candidate_commit.as_str()),
        ("candidate tree", binding.candidate_tree.as_str()),
        (
            "frozen oracle commit",
            binding.frozen_oracle_commit.as_str(),
        ),
        ("frozen oracle tree", binding.frozen_oracle_tree.as_str()),
    ] {
        validate_git_object_id(label, value)?;
    }
    for (label, digest) in [
        (
            "frozen oracle manifest",
            &binding.frozen_oracle_manifest_sha256,
        ),
        (
            "canonical oracle corpus",
            &binding.canonical_oracle_corpus_sha256,
        ),
        ("qualification nonce", &binding.qualification_nonce_sha256),
    ] {
        validate_digest(label, digest)?;
    }
    if binding.frozen_oracle_commit != PINNED_FROZEN_ORACLE_COMMIT
        || binding.frozen_oracle_tree != PINNED_FROZEN_ORACLE_TREE
        || binding.frozen_oracle_manifest_sha256.as_str() != PINNED_FROZEN_ORACLE_MANIFEST_SHA256
        || binding.canonical_oracle_corpus_sha256.as_str() != PINNED_FROZEN_ORACLE_CORPUS_SHA256
        || binding.required_sample_count != PINNED_FROZEN_ORACLE_SAMPLE_COUNT as u64
    {
        return invalid(
            "frozen-oracle qualification run must bind the library-pinned v2 corpus and all 252 samples",
        );
    }
    if binding.qualification_run_started_at_ms < 0 {
        return invalid("frozen-oracle qualification run start time must be non-negative");
    }
    if binding.governance_mode != GovernanceMode::Shadow
        || binding.enforce_enabled
        || !binding.qualification_only
        || binding.promotion_authority_granted
        || binding.outbound_enabled
        || binding.memory_mutation_enabled
        || binding.proof_authority_enabled
        || binding.retirement_authority_enabled
    {
        return invalid(
            "frozen-oracle qualification run must remain qualification-only Shadow with every authority disabled",
        );
    }
    Ok(())
}

fn validate_observation(record: &FrozenOracleConformanceRecord) -> Result<(), EvidenceError> {
    if record.schema_version != FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION {
        return invalid("unsupported frozen-oracle conformance observation schema version");
    }
    FrozenOracleQualificationRunId::parse(record.qualification_run_id.as_str())?;
    for (label, digest) in [
        ("binding", &record.binding_sha256),
        (
            "canonical oracle corpus",
            &record.canonical_oracle_corpus_sha256,
        ),
        ("sample id", &record.sample_id_sha256),
        ("candidate output", &record.candidate_output_sha256),
        (
            "canonical oracle output",
            &record.canonical_oracle_output_sha256,
        ),
        (
            "previous frozen-oracle conformance observation",
            &record.previous_observation_sha256,
        ),
    ] {
        validate_digest(label, digest)?;
    }
    if record.ordinal == 0 {
        return invalid("frozen-oracle conformance observation ordinal must be positive");
    }
    let expected_match = record.candidate_output_sha256 == record.canonical_oracle_output_sha256;
    if record.canonical_oracle_matched != expected_match
        || !record.qualification_only
        || record.promotion_authority_granted
    {
        return invalid(
            "frozen-oracle conformance observation canonical_oracle_matched or qualification authority is invalid",
        );
    }
    Ok(())
}

fn validate_terminal(
    record: &FrozenOracleQualificationTerminalRecord,
) -> Result<(), EvidenceError> {
    if record.schema_version != FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION {
        return invalid("unsupported frozen-oracle qualification terminal schema version");
    }
    FrozenOracleQualificationRunId::parse(record.qualification_run_id.as_str())?;
    for (label, digest) in [
        ("binding", &record.binding_sha256),
        (
            "canonical oracle corpus",
            &record.canonical_oracle_corpus_sha256,
        ),
        (
            "head frozen-oracle conformance observation",
            &record.head_observation_sha256,
        ),
    ] {
        validate_digest(label, digest)?;
    }
    if record.observation_count == 0
        || record
            .canonical_oracle_match_count
            .checked_add(record.canonical_oracle_divergence_count)
            != Some(record.observation_count)
    {
        return invalid("frozen-oracle qualification terminal counts are invalid");
    }
    let expected_kind = if record.canonical_oracle_divergence_count == 0 {
        FrozenOracleConformanceStatus::Conformant
    } else {
        FrozenOracleConformanceStatus::Diverged
    };
    if record.conformance_status != expected_kind
        || record.qualification_run_finished_at_ms < record.qualification_run_started_at_ms
        || record.governance_mode != GovernanceMode::Shadow
        || record.enforce_enabled
        || !record.qualification_only
        || record.promotion_authority_granted
    {
        return invalid(
            "frozen-oracle qualification terminal status, timing, or qualification authority is invalid",
        );
    }
    Ok(())
}

fn validate_git_object_id(label: &str, value: &str) -> Result<(), EvidenceError> {
    if !matches!(value.len(), 40 | 64)
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return invalid(format!(
            "{label} must be a 40- or 64-character lowercase hexadecimal Git object id"
        ));
    }
    Ok(())
}

fn validate_prefixed_digest(label: &str, value: &str, prefix: &str) -> Result<(), EvidenceError> {
    let Some(digest) = value.strip_prefix(prefix) else {
        return invalid(format!("{label} has an invalid domain prefix"));
    };
    Sha256Digest::parse(digest.to_string())
        .map(|_| ())
        .map_err(|error| EvidenceError::InvalidRecord(format!("{label}: {error}")))
}

fn validate_digest(label: &str, digest: &Sha256Digest) -> Result<(), EvidenceError> {
    Sha256Digest::parse(digest.as_str().to_string())
        .map(|_| ())
        .map_err(|error| EvidenceError::InvalidRecord(format!("{label}: {error}")))
}

fn invalid(detail: impl Into<String>) -> Result<(), EvidenceError> {
    Err(EvidenceError::InvalidRecord(detail.into()))
}

fn invalid_value<T>(detail: impl Into<String>) -> Result<T, EvidenceError> {
    Err(EvidenceError::InvalidRecord(detail.into()))
}

fn pinned_digest(value: &str) -> Sha256Digest {
    Sha256Digest::parse(value.to_string())
        .unwrap_or_else(|error| panic!("invalid compile-time frozen-oracle digest pin: {error}"))
}

pub(crate) async fn verify_frozen_oracle_qualification_integrity(
    pool: &SqlitePool,
) -> Result<(), EvidenceError> {
    let corpus = VerifiedFrozenOracleCorpus::load(PINNED_FROZEN_ORACLE_CORPUS_BYTES)
        .map_err(invalid_as_corrupt)?;
    let mut transaction = pool.begin().await.map_err(classify_sqlx_error)?;
    // `seq` is deliberately not used as identity or as the paging cursor. The
    // validated, unique run id gives bounded deterministic pages while every
    // authoritative binding and chain digest is recomputed.
    let mut last_run_id = String::new();
    loop {
        let rows = sqlx::query(
            "SELECT qualification_run_id
             FROM frozen_oracle_qualification_bindings
             WHERE qualification_run_id > ?
             ORDER BY qualification_run_id ASC LIMIT ?",
        )
        .bind(&last_run_id)
        .bind(BINDING_VERIFY_PAGE_SIZE)
        .fetch_all(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        if rows.is_empty() {
            break;
        }
        for row in rows {
            let value: String = row.get("qualification_run_id");
            last_run_id.clone_from(&value);
            let qualification_run_id =
                FrozenOracleQualificationRunId::parse(value).map_err(invalid_as_corrupt)?;
            load_verified_frozen_oracle_qualification(
                &mut transaction,
                &qualification_run_id,
                None,
                &corpus,
            )
            .await?
            .ok_or_else(|| {
                EvidenceError::Corrupt(format!(
                    "frozen-oracle qualification run {} disappeared during integrity verification",
                    qualification_run_id.as_str()
                ))
            })?;
        }
    }
    transaction.commit().await.map_err(classify_sqlx_error)?;
    Ok(())
}

async fn load_verified_frozen_oracle_qualification(
    connection: &mut SqliteConnection,
    qualification_run_id: &FrozenOracleQualificationRunId,
    snapshot_hook_database_path: Option<&std::path::Path>,
    corpus: &VerifiedFrozenOracleCorpus,
) -> Result<Option<VerifiedFrozenOracleQualification>, EvidenceError> {
    let binding_row = sqlx::query(BINDING_SELECT_BY_RUN)
        .bind(qualification_run_id.as_str())
        .fetch_optional(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
    let Some(binding_row) = binding_row else {
        return Ok(None);
    };
    let (binding, material) = decode_binding_row(&binding_row)?;
    corpus
        .validate_binding(&binding)
        .map_err(invalid_as_corrupt)?;
    if material.qualification_run_id != *qualification_run_id {
        return Err(EvidenceError::Corrupt(
            "frozen-oracle qualification binding identity differs from requested run".to_string(),
        ));
    }
    #[cfg(test)]
    if let Some(database_path) = snapshot_hook_database_path {
        pause_after_frozen_oracle_qualification_binding_select(database_path, qualification_run_id)
            .await;
    }
    #[cfg(not(test))]
    let _ = snapshot_hook_database_path;
    let mut rows = sqlx::query(OBSERVATION_SELECT_BY_RUN)
        .bind(qualification_run_id.as_str())
        .fetch(&mut *connection);
    let mut observation_count = 0_u64;
    let mut previous = material.binding_sha256.clone();
    let mut canonical_oracle_match_count = 0_u64;
    let mut canonical_oracle_divergence_count = 0_u64;
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        let stored = decode_observation_row(&row)?;
        let expected_ordinal = observation_count.checked_add(1).ok_or_else(|| {
            EvidenceError::Corrupt(
                "frozen-oracle conformance observation ordinal overflow".to_string(),
            )
        })?;
        let corpus_case = corpus.case(expected_ordinal).map_err(invalid_as_corrupt)?;
        if stored.observation.qualification_run_id != *qualification_run_id
            || stored.observation.binding_sha256 != material.binding_sha256
            || stored.observation.canonical_oracle_corpus_sha256
                != binding.canonical_oracle_corpus_sha256
            || stored.observation.ordinal != expected_ordinal
            || stored.observation.sample_id_sha256 != corpus_case.sample_id_sha256
            || stored.observation.canonical_oracle_output_sha256
                != corpus_case.canonical_oracle_output_sha256
            || stored.observation.previous_observation_sha256 != previous
        {
            return Err(EvidenceError::Corrupt(format!(
                "frozen-oracle qualification run {} has a broken sequence or hash-chain binding at ordinal {}",
                qualification_run_id.as_str(),
                expected_ordinal
            )));
        }
        if stored.observation.canonical_oracle_matched {
            canonical_oracle_match_count =
                canonical_oracle_match_count.checked_add(1).ok_or_else(|| {
                    EvidenceError::Corrupt(
                        "frozen-oracle qualification match count overflow".to_string(),
                    )
                })?;
        } else {
            canonical_oracle_divergence_count = canonical_oracle_divergence_count
                .checked_add(1)
                .ok_or_else(|| {
                    EvidenceError::Corrupt(
                        "frozen-oracle qualification divergence count overflow".to_string(),
                    )
                })?;
        }
        previous = stored.observation_sha256.clone();
        observation_count = expected_ordinal;
    }
    drop(rows);
    if observation_count > binding.required_sample_count {
        return Err(EvidenceError::Corrupt(format!(
            "frozen-oracle qualification run {} has {} observations but is bound to at most {}",
            qualification_run_id.as_str(),
            observation_count,
            binding.required_sample_count
        )));
    }
    let head = load_qualification_head(connection, qualification_run_id)
        .await?
        .ok_or_else(|| {
            EvidenceError::Corrupt(format!(
                "frozen-oracle qualification run {} has no append head",
                qualification_run_id.as_str()
            ))
        })?;
    if head.qualification_run_id != *qualification_run_id
        || head.binding_sha256 != material.binding_sha256
        || head.observation_count != observation_count
        || head.canonical_oracle_match_count != canonical_oracle_match_count
        || head.canonical_oracle_divergence_count != canonical_oracle_divergence_count
        || head.head_observation_sha256 != previous
    {
        return Err(EvidenceError::Corrupt(format!(
            "frozen-oracle qualification append head for {} does not match the immutable chain",
            qualification_run_id.as_str()
        )));
    }
    let terminal_row = sqlx::query(TERMINAL_SELECT_BY_RUN)
        .bind(qualification_run_id.as_str())
        .fetch_optional(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
    let terminal = terminal_row
        .map(|row| decode_terminal_row(&row))
        .transpose()?;
    if let Some(stored) = terminal.as_ref()
        && (stored.terminal.qualification_run_id != *qualification_run_id
            || stored.terminal.binding_sha256 != material.binding_sha256
            || stored.terminal.canonical_oracle_corpus_sha256
                != binding.canonical_oracle_corpus_sha256
            || stored.terminal.observation_count != observation_count
            || stored.terminal.observation_count != binding.required_sample_count
            || stored.terminal.canonical_oracle_match_count != canonical_oracle_match_count
            || stored.terminal.canonical_oracle_divergence_count
                != canonical_oracle_divergence_count
            || stored.terminal.head_observation_sha256 != previous
            || stored.terminal.qualification_run_started_at_ms
                != binding.qualification_run_started_at_ms)
    {
        return Err(EvidenceError::Corrupt(format!(
            "frozen-oracle qualification terminal for {} does not anchor the authoritative chain",
            qualification_run_id.as_str()
        )));
    }
    Ok(Some(VerifiedFrozenOracleQualification {
        binding,
        binding_sha256: material.binding_sha256,
        observation_count,
        canonical_oracle_match_count,
        canonical_oracle_divergence_count,
        head_observation_sha256: previous,
        terminal,
    }))
}

async fn load_qualification_head(
    connection: &mut SqliteConnection,
    qualification_run_id: &FrozenOracleQualificationRunId,
) -> Result<Option<QualificationHead>, EvidenceError> {
    sqlx::query(HEAD_SELECT_BY_RUN)
        .bind(qualification_run_id.as_str())
        .fetch_optional(connection)
        .await
        .map_err(classify_sqlx_error)?
        .map(|row| decode_qualification_head_row(&row))
        .transpose()
}

fn decode_qualification_head_row(row: &SqliteRow) -> Result<QualificationHead, EvidenceError> {
    let qualification_run_id =
        FrozenOracleQualificationRunId::parse(row.get::<String, _>("qualification_run_id"))
            .map_err(invalid_as_corrupt)?;
    let binding_sha256 = Sha256Digest::parse(row.get::<String, _>("binding_sha256"))
        .map_err(EvidenceError::Corrupt)?;
    let head_observation_sha256 =
        Sha256Digest::parse(row.get::<String, _>("head_observation_sha256"))
            .map_err(EvidenceError::Corrupt)?;
    let observation_count = u64::try_from(row.get::<i64, _>("observation_count"))
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    let canonical_oracle_match_count =
        u64::try_from(row.get::<i64, _>("canonical_oracle_match_count"))
            .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    let canonical_oracle_divergence_count =
        u64::try_from(row.get::<i64, _>("canonical_oracle_divergence_count"))
            .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    if canonical_oracle_match_count.checked_add(canonical_oracle_divergence_count)
        != Some(observation_count)
    {
        return Err(EvidenceError::Corrupt(
            "frozen-oracle qualification append-head counts are inconsistent".to_string(),
        ));
    }
    Ok(QualificationHead {
        qualification_run_id,
        binding_sha256,
        observation_count,
        canonical_oracle_match_count,
        canonical_oracle_divergence_count,
        head_observation_sha256,
    })
}

fn decode_binding_row(
    row: &SqliteRow,
) -> Result<(FrozenOracleQualificationBinding, BindingMaterial), EvidenceError> {
    let payload_json: String = row.get("binding_json");
    verify_stored_payload(
        &payload_json,
        row.get("payload_sha256"),
        "frozen-oracle qualification binding",
        MAX_BINDING_JSON_BYTES,
    )?;
    let binding: FrozenOracleQualificationBinding = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    validate_binding(&binding).map_err(invalid_as_corrupt)?;
    let material = binding_material(&binding).map_err(invalid_as_corrupt)?;
    if material.payload_json != payload_json
        || row.get::<String, _>("qualification_run_id") != material.qualification_run_id.as_str()
        || row.get::<i64, _>("schema_version") != i64::from(binding.schema_version)
        || row.get::<String, _>("candidate_commit") != binding.candidate_commit
        || row.get::<String, _>("candidate_tree") != binding.candidate_tree
        || row.get::<String, _>("frozen_oracle_commit") != binding.frozen_oracle_commit
        || row.get::<String, _>("frozen_oracle_tree") != binding.frozen_oracle_tree
        || row.get::<String, _>("frozen_oracle_manifest_sha256")
            != binding.frozen_oracle_manifest_sha256.as_str()
        || row.get::<String, _>("canonical_oracle_corpus_sha256")
            != binding.canonical_oracle_corpus_sha256.as_str()
        || row.get::<String, _>("qualification_nonce_sha256")
            != binding.qualification_nonce_sha256.as_str()
        || row.get::<i64, _>("required_sample_count")
            != i64::try_from(binding.required_sample_count)
                .map_err(|error| EvidenceError::Corrupt(error.to_string()))?
        || row.get::<i64, _>("qualification_run_started_at_ms")
            != binding.qualification_run_started_at_ms
        || row.get::<String, _>("governance_mode") != "shadow"
        || row.get::<bool, _>("enforce_enabled") != binding.enforce_enabled
        || row.get::<bool, _>("qualification_only") != binding.qualification_only
        || row.get::<bool, _>("promotion_authority_granted") != binding.promotion_authority_granted
        || row.get::<bool, _>("outbound_enabled") != binding.outbound_enabled
        || row.get::<bool, _>("memory_mutation_enabled") != binding.memory_mutation_enabled
        || row.get::<bool, _>("proof_authority_enabled") != binding.proof_authority_enabled
        || row.get::<bool, _>("retirement_authority_enabled")
            != binding.retirement_authority_enabled
        || row.get::<String, _>("payload_sha256") != material.payload_sha256.as_str()
        || row.get::<String, _>("binding_sha256") != material.binding_sha256.as_str()
    {
        return Err(EvidenceError::Corrupt(
            "frozen-oracle qualification binding columns do not match canonical payload"
                .to_string(),
        ));
    }
    Ok((binding, material))
}

fn decode_observation_row(
    row: &SqliteRow,
) -> Result<StoredFrozenOracleConformanceObservation, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    verify_stored_payload(
        &payload_json,
        row.get("payload_sha256"),
        "frozen-oracle conformance observation",
        MAX_OBSERVATION_JSON_BYTES,
    )?;
    let observation: FrozenOracleConformanceRecord = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    validate_observation(&observation).map_err(invalid_as_corrupt)?;
    let canonical = canonical_json(&observation).map_err(invalid_as_corrupt)?;
    let canonical_json = String::from_utf8(canonical.clone())
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    let payload_sha256 = Sha256Digest::for_bytes(&canonical);
    let observation_sha256 = domain_digest(OBSERVATION_DOMAIN, &canonical);
    if canonical_json != payload_json
        || row.get::<String, _>("qualification_run_id") != observation.qualification_run_id.as_str()
        || row.get::<String, _>("binding_sha256") != observation.binding_sha256.as_str()
        || row.get::<String, _>("canonical_oracle_corpus_sha256")
            != observation.canonical_oracle_corpus_sha256.as_str()
        || row.get::<i64, _>("ordinal")
            != i64::try_from(observation.ordinal)
                .map_err(|error| EvidenceError::Corrupt(error.to_string()))?
        || row.get::<String, _>("sample_id_sha256") != observation.sample_id_sha256.as_str()
        || row.get::<String, _>("candidate_output_sha256")
            != observation.candidate_output_sha256.as_str()
        || row.get::<String, _>("canonical_oracle_output_sha256")
            != observation.canonical_oracle_output_sha256.as_str()
        || row.get::<bool, _>("canonical_oracle_matched") != observation.canonical_oracle_matched
        || row.get::<String, _>("previous_observation_sha256")
            != observation.previous_observation_sha256.as_str()
        || row.get::<String, _>("observation_sha256") != observation_sha256.as_str()
        || row.get::<i64, _>("schema_version") != i64::from(observation.schema_version)
        || row.get::<bool, _>("qualification_only") != observation.qualification_only
        || row.get::<bool, _>("promotion_authority_granted")
            != observation.promotion_authority_granted
        || row.get::<String, _>("payload_sha256") != payload_sha256.as_str()
    {
        return Err(EvidenceError::Corrupt(
            "frozen-oracle conformance observation columns do not match canonical payload"
                .to_string(),
        ));
    }
    Ok(StoredFrozenOracleConformanceObservation {
        seq: row.get("seq"),
        observation_sha256,
        observation,
    })
}

fn decode_terminal_row(
    row: &SqliteRow,
) -> Result<StoredFrozenOracleQualificationTerminal, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    verify_stored_payload(
        &payload_json,
        row.get("payload_sha256"),
        "frozen-oracle qualification terminal",
        MAX_TERMINAL_JSON_BYTES,
    )?;
    let terminal: FrozenOracleQualificationTerminalRecord = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    validate_terminal(&terminal).map_err(invalid_as_corrupt)?;
    let canonical = canonical_json(&terminal).map_err(invalid_as_corrupt)?;
    let canonical_json = String::from_utf8(canonical.clone())
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    let payload_sha256 = Sha256Digest::for_bytes(&canonical);
    let terminal_sha256 = domain_digest(TERMINAL_DOMAIN, &canonical);
    let terminal_id = FrozenOracleQualificationTerminalId::for_digest(&terminal_sha256);
    if canonical_json != payload_json
        || row.get::<String, _>("terminal_id") != terminal_id.as_str()
        || row.get::<String, _>("qualification_run_id") != terminal.qualification_run_id.as_str()
        || row.get::<String, _>("binding_sha256") != terminal.binding_sha256.as_str()
        || row.get::<String, _>("canonical_oracle_corpus_sha256")
            != terminal.canonical_oracle_corpus_sha256.as_str()
        || row.get::<String, _>("conformance_status") != terminal.conformance_status.as_str()
        || row.get::<i64, _>("observation_count")
            != i64::try_from(terminal.observation_count)
                .map_err(|error| EvidenceError::Corrupt(error.to_string()))?
        || row.get::<i64, _>("canonical_oracle_match_count")
            != i64::try_from(terminal.canonical_oracle_match_count)
                .map_err(|error| EvidenceError::Corrupt(error.to_string()))?
        || row.get::<i64, _>("canonical_oracle_divergence_count")
            != i64::try_from(terminal.canonical_oracle_divergence_count)
                .map_err(|error| EvidenceError::Corrupt(error.to_string()))?
        || row.get::<String, _>("head_observation_sha256")
            != terminal.head_observation_sha256.as_str()
        || row.get::<i64, _>("qualification_run_started_at_ms")
            != terminal.qualification_run_started_at_ms
        || row.get::<i64, _>("qualification_run_finished_at_ms")
            != terminal.qualification_run_finished_at_ms
        || row.get::<i64, _>("schema_version") != i64::from(terminal.schema_version)
        || row.get::<String, _>("governance_mode") != "shadow"
        || row.get::<bool, _>("enforce_enabled") != terminal.enforce_enabled
        || row.get::<bool, _>("qualification_only") != terminal.qualification_only
        || row.get::<bool, _>("promotion_authority_granted") != terminal.promotion_authority_granted
        || row.get::<String, _>("terminal_sha256") != terminal_sha256.as_str()
        || row.get::<String, _>("payload_sha256") != payload_sha256.as_str()
    {
        return Err(EvidenceError::Corrupt(
            "frozen-oracle qualification terminal columns do not match canonical payload"
                .to_string(),
        ));
    }
    Ok(StoredFrozenOracleQualificationTerminal {
        seq: row.get("seq"),
        terminal_id,
        terminal_sha256,
        terminal,
    })
}

fn verify_stored_payload(
    payload_json: &str,
    stored_digest: String,
    label: &str,
    max_bytes: usize,
) -> Result<(), EvidenceError> {
    if !(2..=max_bytes).contains(&payload_json.len()) {
        return Err(EvidenceError::Corrupt(format!(
            "{label} JSON is outside its 2..={max_bytes} byte bound"
        )));
    }
    let expected = Sha256Digest::for_bytes(payload_json.as_bytes());
    let stored = Sha256Digest::parse(stored_digest)
        .map_err(|error| EvidenceError::Corrupt(format!("{label}: {error}")))?;
    if stored == expected {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "{label} payload digest does not match stored JSON"
        )))
    }
}

fn invalid_as_corrupt(error: EvidenceError) -> EvidenceError {
    match error {
        EvidenceError::InvalidRecord(detail) => EvidenceError::Corrupt(detail),
        other => other,
    }
}
