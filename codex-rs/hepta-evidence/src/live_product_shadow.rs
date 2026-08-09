use codex_hepta_contracts::ActionId;
use codex_hepta_contracts::GovernanceDecision;
use codex_hepta_contracts::GovernanceDecisionRecord;
use codex_hepta_contracts::GovernanceMode;
use codex_hepta_contracts::GovernanceReceipt;
use codex_hepta_contracts::HandlerOutcome;
use codex_hepta_contracts::PolicyPhase;
use codex_hepta_contracts::ReceiptId;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::ToolAction;
use codex_hepta_contracts::ToolActionSource;
use futures::TryStreamExt;
use serde::Deserialize;
use sqlx::Row;
use sqlx::SqliteConnection;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteRow;

use crate::EvidenceError;
use crate::canonical::canonical_json;
use crate::schema_validation::classify_sqlx_error;

pub(crate) const LIVE_PRODUCT_SHADOW_V2_SCHEMA_VERSION: u32 = 2;

const ORACLE_COMMIT: &str = "2f704dc7c1172cefca908852456beccf4d02a5d1";
const ORACLE_TREE: &str = "7be9a382b2610790838eef874cb4d381b5025490";
const ORACLE_MANIFEST_SHA256: &str =
    "2c82d45303e912b92a7b9ac31da4661197e59a5ca415d3c70375b49169691377";
const ORACLE_GENERATOR_SHA256: &str =
    "0778717e2ef2a9adfc7eb3c6980a8c2e7433e4ffbbbc6f124fb9e4098b4d1ab9";
const ORACLE_CORPUS_SHA256: &str =
    "dfe4f04d26895a6fabfb8435b77d7e807f57379fbb8d2a96c85af747e996cda7";
const TRACKED_ORACLE_SHA256: &str =
    "faa924acbdca3df64ffacf272d3367a8be26e6d5ed9ec384b71a2d744ccce0e9";
const ORACLE_CORPUS_BYTE_LEN: usize = 5_194;
const TRACKED_ORACLE_BYTE_LEN: usize = ORACLE_CORPUS_BYTE_LEN + 1;
const MAX_ORACLE_CORPUS_BYTES: usize = ORACLE_CORPUS_BYTE_LEN;
const MAX_NORMALIZED_RECEIPT_BYTES: usize = 4_096;
const ORACLE_PROFILE: &str = "live_product_parser_reachable_semantics_v2";
const ORACLE_SAMPLE_ID_SHA256: &str =
    "426468e3c420e5557f2edbbb0adfc845b611c00416112c1ed95d99219fa9c5ef";
const RAW_FUNCTION_ARGUMENTS: &str =
    r#"{"command":"/usr/bin/printf hepta-shadow-probe","login":false,"timeout_ms":5000}"#;
const RAW_FUNCTION_ARGUMENTS_SHA256: &str =
    "28543d724c56a81d59ccb9c183300ff568b158cb33bc8330a581a3aa32ab239d";
const PAYLOAD_SHA256: &str = "0918708543060974ab1e37c2b08d0ea688838f4ec54477eb9945d62478e07cbf";
const NORMALIZED_RECEIPT_SHA256: &str =
    "8904f0cc74e8a1b465eb75c7cd0c3f6ebef916c414dc9f5b6610d5822e9f68c0";
const POLICY_ID: &str = "hepta.bootstrap_integrity.v1";
const POLICY_CONTENT_SHA256: &str =
    "7d08d602c3a825f3e4c981296b9928e4e205f7cfc2984eb60a1ba82d80a907e0";
const FIXED_THREAD_ID: &str = "thread-oracle-v2";
const FIXED_TURN_ID: &str = "turn-oracle-v2";
const FIXED_CALL_ID: &str = "call-oracle-v2";
const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

const RUN_ID_DOMAIN: &[u8] = b"hepta-live-product-shadow-run-id:v2";
const RUN_BINDING_DOMAIN: &[u8] = b"hepta-live-product-shadow-run-binding:v2";
const SEGMENT_ID_DOMAIN: &[u8] = b"hepta-live-product-shadow-segment-id:v2";
const SEGMENT_BINDING_DOMAIN: &[u8] = b"hepta-live-product-shadow-segment-binding:v2";
const INTENT_ID_DOMAIN: &[u8] = b"hepta-live-product-shadow-intent-id:v2";
const INTENT_CHAIN_DOMAIN: &[u8] = b"hepta-live-product-shadow-intent-chain:v2";
const IMPORT_DOMAIN: &[u8] = b"hepta-live-product-shadow-artifact-import:v2";
const EVIDENCE_SET_DOMAIN: &[u8] = b"hepta-live-product-shadow-evidence-set:v2";
const TERMINAL_DOMAIN: &[u8] = b"hepta-live-product-shadow-terminal:v2";

const TRACKED_ORACLE_BYTES: &[u8] = include_bytes!("../fixtures/live_product_oracle_v2_2f704.json");

fn invalid(message: impl Into<String>) -> EvidenceError {
    EvidenceError::InvalidRecord(message.into())
}

fn corrupt(message: impl Into<String>) -> EvidenceError {
    EvidenceError::Corrupt(message.into())
}

pub(crate) fn pinned_live_product_oracle_v2_bytes() -> Result<&'static [u8], EvidenceError> {
    if TRACKED_ORACLE_BYTES.len() != TRACKED_ORACLE_BYTE_LEN
        || TRACKED_ORACLE_BYTES.last() != Some(&b'\n')
        || Sha256Digest::for_bytes(TRACKED_ORACLE_BYTES).as_str() != TRACKED_ORACLE_SHA256
    {
        return Err(corrupt(
            "tracked live product oracle v2 representation does not match its library pin",
        ));
    }
    let official = &TRACKED_ORACLE_BYTES[..ORACLE_CORPUS_BYTE_LEN];
    if official.last() != Some(&b'}')
        || Sha256Digest::for_bytes(official).as_str() != ORACLE_CORPUS_SHA256
    {
        return Err(corrupt(
            "derived official live product oracle v2 bytes do not match their library pin",
        ));
    }
    Ok(official)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleDocument {
    authority: OracleAuthority,
    canonical_encoding: String,
    canonical_object: String,
    case_count: u64,
    cases: Vec<OracleCase>,
    generator: OracleGenerator,
    normalization: OracleNormalization,
    oracle_commit: String,
    oracle_manifest_sha256: String,
    oracle_profile: String,
    oracle_tree: String,
    qualification_only: bool,
    reachability: OracleReachability,
    schema: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleAuthority {
    enforce: bool,
    operator_acceptance: bool,
    outbound: bool,
    promotion: bool,
    retirement: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleCase {
    expected_normalized_receipt_canonical_json: String,
    expected_normalized_receipt_sha256: String,
    expected_output_sha256: String,
    expected_projection: serde_json::Value,
    function_arguments_raw: String,
    function_arguments_raw_sha256: String,
    ordinal: u64,
    payload_kind: String,
    payload_sha256: String,
    receipt_phase: String,
    sample_id_sha256: String,
    source_kind: String,
    terminal_kind: String,
    tool_name: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleGenerator {
    entrypoint: String,
    schema: String,
    source_sha256: String,
    version: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleNormalization {
    fixed_identity: OracleFixedIdentity,
    formula: String,
    output: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleFixedIdentity {
    call_id: String,
    thread_id: String,
    turn_id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OracleReachability {
    actual_live_product_reachability: String,
    actual_live_product_trial_closure_required: bool,
    parser_reachable_semantics: bool,
    parser_type: String,
    statement: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ShellCommandArgsPin {
    command: String,
    login: bool,
    timeout_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VerifiedLiveProductOracleV2 {
    corpus_sha256: Sha256Digest,
    raw_function_arguments_sha256: Sha256Digest,
    payload_sha256: Sha256Digest,
    normalized_receipt_sha256: Sha256Digest,
    sample_id_sha256: Sha256Digest,
    normalized_receipt: GovernanceReceipt,
}

impl VerifiedLiveProductOracleV2 {
    pub(crate) fn load_pinned() -> Result<Self, EvidenceError> {
        Self::load(pinned_live_product_oracle_v2_bytes()?)
    }

    pub(crate) fn load(bytes: &[u8]) -> Result<Self, EvidenceError> {
        if bytes.len() > MAX_ORACLE_CORPUS_BYTES {
            return Err(invalid(format!(
                "live product oracle v2 exceeds the {MAX_ORACLE_CORPUS_BYTES}-byte cap"
            )));
        }
        if bytes.len() != ORACLE_CORPUS_BYTE_LEN
            || Sha256Digest::for_bytes(bytes).as_str() != ORACLE_CORPUS_SHA256
        {
            return Err(invalid(
                "live product oracle v2 bytes do not match the exact library pin",
            ));
        }
        let value: serde_json::Value = serde_json::from_slice(bytes)
            .map_err(|error| invalid(format!("invalid live product oracle v2 JSON: {error}")))?;
        if canonical_json(&value)? != bytes {
            return Err(invalid(
                "live product oracle v2 is not compact canonical JSON",
            ));
        }
        let document: OracleDocument = serde_json::from_value(value).map_err(|error| {
            invalid(format!(
                "live product oracle v2 has an unknown, missing, or invalid field: {error}"
            ))
        })?;
        validate_header(&document)?;
        if document.case_count != 1 || document.cases.len() != 1 {
            return Err(invalid(
                "live product oracle v2 must contain exactly one pinned case",
            ));
        }
        let case = document
            .cases
            .into_iter()
            .next()
            .ok_or_else(|| invalid("live product oracle v2 case is missing"))?;
        validate_case_header(&case)?;

        let raw_value: serde_json::Value = serde_json::from_str(&case.function_arguments_raw)
            .map_err(|error| invalid(format!("invalid pinned Function arguments JSON: {error}")))?;
        if canonical_json(&raw_value)? != case.function_arguments_raw.as_bytes() {
            return Err(invalid(
                "pinned Function arguments are not compact canonical JSON",
            ));
        }
        let raw_args: ShellCommandArgsPin = serde_json::from_value(raw_value).map_err(|error| {
            invalid(format!(
                "pinned Function arguments do not match the strict shell-command shape: {error}"
            ))
        })?;
        if raw_args.command != "/usr/bin/printf hepta-shadow-probe"
            || raw_args.login
            || raw_args.timeout_ms != 5_000
        {
            return Err(invalid(
                "pinned Function arguments do not match the exact shell-command probe",
            ));
        }

        if case.expected_normalized_receipt_canonical_json.len() > MAX_NORMALIZED_RECEIPT_BYTES {
            return Err(invalid(
                "normalized live product receipt exceeds its byte cap",
            ));
        }
        let receipt_value: serde_json::Value = serde_json::from_str(
            &case.expected_normalized_receipt_canonical_json,
        )
        .map_err(|error| {
            invalid(format!(
                "invalid normalized live product receipt JSON: {error}"
            ))
        })?;
        if canonical_json(&receipt_value)?
            != case.expected_normalized_receipt_canonical_json.as_bytes()
        {
            return Err(invalid(
                "normalized live product receipt is not compact canonical JSON",
            ));
        }
        let receipt: GovernanceReceipt =
            serde_json::from_value(receipt_value).map_err(|error| {
                invalid(format!("invalid normalized live product receipt: {error}"))
            })?;
        if canonical_json(&receipt)? != case.expected_normalized_receipt_canonical_json.as_bytes() {
            return Err(invalid(
                "normalized live product receipt contains unknown or lossy fields",
            ));
        }
        validate_receipt_identity(&receipt)?;
        validate_reachable_receipt_semantics(&receipt)?;
        let normalized = normalize_live_product_receipt_v2(&receipt)?;
        let normalized_bytes = canonical_json(&normalized)?;
        if normalized_bytes != case.expected_normalized_receipt_canonical_json.as_bytes()
            || Sha256Digest::for_bytes(&normalized_bytes).as_str() != NORMALIZED_RECEIPT_SHA256
        {
            return Err(invalid(
                "normalized live product receipt does not reproduce its pinned vector",
            ));
        }
        if canonical_json(&receipt_projection(&receipt))?
            != canonical_json(&case.expected_projection)?
        {
            return Err(invalid(
                "live product receipt projection does not match its pinned vector",
            ));
        }

        Ok(Self {
            corpus_sha256: parse_digest(ORACLE_CORPUS_SHA256, "oracle corpus")?,
            raw_function_arguments_sha256: parse_digest(
                RAW_FUNCTION_ARGUMENTS_SHA256,
                "raw Function arguments",
            )?,
            payload_sha256: parse_digest(PAYLOAD_SHA256, "private payload")?,
            normalized_receipt_sha256: parse_digest(
                NORMALIZED_RECEIPT_SHA256,
                "normalized receipt",
            )?,
            sample_id_sha256: parse_digest(ORACLE_SAMPLE_ID_SHA256, "oracle sample id")?,
            normalized_receipt: normalized,
        })
    }

    fn verify_internal_pins(&self) -> Result<(), EvidenceError> {
        let normalized_bytes = canonical_json(&self.normalized_receipt)?;
        if self.corpus_sha256.as_str() != ORACLE_CORPUS_SHA256
            || self.raw_function_arguments_sha256.as_str() != RAW_FUNCTION_ARGUMENTS_SHA256
            || self.payload_sha256.as_str() != PAYLOAD_SHA256
            || self.normalized_receipt_sha256.as_str() != NORMALIZED_RECEIPT_SHA256
            || self.sample_id_sha256.as_str() != ORACLE_SAMPLE_ID_SHA256
            || Sha256Digest::for_bytes(&normalized_bytes) != self.normalized_receipt_sha256
        {
            return Err(corrupt(
                "loaded live product oracle v2 capability has inconsistent internal pins",
            ));
        }
        Ok(())
    }
}

fn validate_header(document: &OracleDocument) -> Result<(), EvidenceError> {
    let authority_absent = !document.authority.enforce
        && !document.authority.operator_acceptance
        && !document.authority.outbound
        && !document.authority.promotion
        && !document.authority.retirement;
    if document.schema != "hepta_live_product_shadow_oracle_corpus_v2"
        || document.canonical_encoding != "compact_utf8_json_recursive_lexicographic_object_keys"
        || document.canonical_object != "codex_hepta_contracts::GovernanceReceipt"
        || document.oracle_commit != ORACLE_COMMIT
        || document.oracle_tree != ORACLE_TREE
        || document.oracle_manifest_sha256 != ORACLE_MANIFEST_SHA256
        || document.oracle_profile != ORACLE_PROFILE
        || !document.qualification_only
        || !authority_absent
        || document.generator.schema != "hepta_live_product_oracle_generator_v2"
        || document.generator.version != 2
        || document.generator.source_sha256 != ORACLE_GENERATOR_SHA256
        || document.generator.entrypoint
            != "live_product_oracle_v2_generator::emit_frozen_live_product_oracle_v2_one_reachable_case"
        || document.normalization.fixed_identity.thread_id != FIXED_THREAD_ID
        || document.normalization.fixed_identity.turn_id != FIXED_TURN_ID
        || document.normalization.fixed_identity.call_id != FIXED_CALL_ID
        || document.normalization.formula
            != "replace dynamic thread/turn/call identity with fixed v2 identity; recompute action, decision, and receipt ids; preserve tool, source, payload, policy, decisions, host acceptance, and outcome"
        || document.normalization.output
            != "compact canonical JSON of the normalized GovernanceReceipt"
        || document.reachability.actual_live_product_reachability != "not_proven"
        || !document
            .reachability
            .actual_live_product_trial_closure_required
        || !document.reachability.parser_reachable_semantics
        || document.reachability.parser_type != "codex_protocol::models::ShellCommandToolCallParams"
        || document.reachability.statement
            != "This corpus proves only that the exact Function arguments deserialize through the frozen public shell-command parser and that frozen governance maps the semantic record. Actual live reachability requires a later exact frozen-product trial closure."
    {
        return Err(invalid(
            "live product oracle v2 header does not match its exact library pin",
        ));
    }
    Ok(())
}

fn validate_case_header(case: &OracleCase) -> Result<(), EvidenceError> {
    let expected_sample = Sha256Digest::for_bytes(
        b"hepta-live-product-shadow-oracle:v2:shell_command:function:direct:admission_and_authorization:handler_completed",
    );
    if case.ordinal != 1
        || case.payload_kind != "function"
        || case.receipt_phase != "admission_and_authorization"
        || case.source_kind != "direct"
        || case.terminal_kind != "handler_completed"
        || case.tool_name != "shell_command"
        || case.function_arguments_raw != RAW_FUNCTION_ARGUMENTS
        || case.function_arguments_raw_sha256 != RAW_FUNCTION_ARGUMENTS_SHA256
        || Sha256Digest::for_bytes(case.function_arguments_raw.as_bytes()).as_str()
            != RAW_FUNCTION_ARGUMENTS_SHA256
        || case.payload_sha256 != PAYLOAD_SHA256
        || case.sample_id_sha256 != ORACLE_SAMPLE_ID_SHA256
        || expected_sample.as_str() != ORACLE_SAMPLE_ID_SHA256
        || case.expected_normalized_receipt_sha256 != NORMALIZED_RECEIPT_SHA256
        || case.expected_output_sha256 != NORMALIZED_RECEIPT_SHA256
        || Sha256Digest::for_bytes(case.expected_normalized_receipt_canonical_json.as_bytes())
            .as_str()
            != NORMALIZED_RECEIPT_SHA256
    {
        return Err(invalid(
            "live product oracle v2 case does not match all exact sample pins",
        ));
    }
    Ok(())
}

fn validate_receipt_identity(receipt: &GovernanceReceipt) -> Result<(), EvidenceError> {
    let action = &receipt.admission.action;
    let expected_action_id =
        ActionId::for_tool_call(&action.thread_id, &action.turn_id, &action.call_id);
    if action.schema_version != codex_hepta_contracts::GOVERNANCE_SCHEMA_VERSION
        || action.action_id != expected_action_id
        || receipt.action_id != expected_action_id
        || receipt.receipt_id != ReceiptId::for_action(&expected_action_id)
        || receipt.admission.phase != PolicyPhase::Admission
        || receipt.admission.decision_id
            != codex_hepta_contracts::DecisionId::for_action(
                &expected_action_id,
                PolicyPhase::Admission.as_str(),
            )
    {
        return Err(invalid("live product receipt identity is inconsistent"));
    }
    let authorization = receipt
        .authorization
        .as_ref()
        .ok_or_else(|| invalid("live product receipt is missing authorization evidence"))?;
    if authorization.action != *action
        || authorization.phase != PolicyPhase::Authorization
        || authorization.decision_id
            != codex_hepta_contracts::DecisionId::for_action(
                &expected_action_id,
                PolicyPhase::Authorization.as_str(),
            )
    {
        return Err(invalid(
            "live product receipt authorization identity is inconsistent",
        ));
    }
    Ok(())
}

fn validate_reachable_receipt_semantics(receipt: &GovernanceReceipt) -> Result<(), EvidenceError> {
    let action = &receipt.admission.action;
    let authorization = receipt
        .authorization
        .as_ref()
        .ok_or_else(|| invalid("live product receipt is missing authorization evidence"))?;
    let policy_matches = |record: &GovernanceDecisionRecord| {
        record.policy.policy_id == POLICY_ID
            && record.policy.revision == 1
            && record.policy.content_sha256.as_str() == POLICY_CONTENT_SHA256
    };
    if action.tool_name != "shell_command"
        || action.source != ToolActionSource::Direct
        || action.payload_sha256.as_str() != PAYLOAD_SHA256
        || receipt.admission.mode != GovernanceMode::Shadow
        || authorization.mode != GovernanceMode::Shadow
        || receipt.admission.decision != GovernanceDecision::NotEvaluated
        || authorization.decision != GovernanceDecision::NotEvaluated
        || !policy_matches(&receipt.admission)
        || !policy_matches(authorization)
        || !receipt.host_accepted
        || receipt.outcome
            != (HandlerOutcome::HandlerCompleted {
                reported_success: true,
            })
    {
        return Err(invalid(
            "live product receipt is outside the single pinned reachable semantic case",
        ));
    }
    Ok(())
}

pub(crate) fn normalize_live_product_receipt_v2(
    receipt: &GovernanceReceipt,
) -> Result<GovernanceReceipt, EvidenceError> {
    validate_receipt_identity(receipt)?;
    validate_reachable_receipt_semantics(receipt)?;
    let normalized_action_id =
        ActionId::for_tool_call(FIXED_THREAD_ID, FIXED_TURN_ID, FIXED_CALL_ID);
    let normalized_action = ToolAction {
        schema_version: codex_hepta_contracts::GOVERNANCE_SCHEMA_VERSION,
        action_id: normalized_action_id,
        thread_id: FIXED_THREAD_ID.to_string(),
        turn_id: FIXED_TURN_ID.to_string(),
        call_id: FIXED_CALL_ID.to_string(),
        tool_name: receipt.admission.action.tool_name.clone(),
        source: receipt.admission.action.source.clone(),
        payload_sha256: receipt.admission.action.payload_sha256.clone(),
    };
    let admission = GovernanceDecisionRecord::new(
        normalized_action.clone(),
        PolicyPhase::Admission,
        receipt.admission.mode,
        receipt.admission.policy.clone(),
        receipt.admission.decision.clone(),
    );
    let source_authorization = receipt
        .authorization
        .as_ref()
        .ok_or_else(|| invalid("live product receipt is missing authorization evidence"))?;
    let authorization = GovernanceDecisionRecord::new(
        normalized_action,
        PolicyPhase::Authorization,
        source_authorization.mode,
        source_authorization.policy.clone(),
        source_authorization.decision.clone(),
    );
    Ok(GovernanceReceipt::new(
        admission,
        Some(authorization),
        receipt.host_accepted,
        receipt.outcome.clone(),
    ))
}

fn receipt_projection(receipt: &GovernanceReceipt) -> serde_json::Value {
    let authorization = receipt.authorization.as_ref().map(|record| {
        serde_json::json!({
            "decision": record.decision,
            "decision_id": record.decision_id.as_str(),
            "mode": record.mode,
            "phase": record.phase.as_str(),
        })
    });
    serde_json::json!({
        "action_id": receipt.action_id.as_str(),
        "admission": {
            "decision": receipt.admission.decision,
            "decision_id": receipt.admission.decision_id.as_str(),
            "mode": receipt.admission.mode,
            "phase": receipt.admission.phase.as_str(),
        },
        "authorization": authorization,
        "call_id": receipt.admission.action.call_id,
        "host_accepted": receipt.host_accepted,
        "outcome": receipt.outcome,
        "payload_sha256": receipt.admission.action.payload_sha256.as_str(),
        "receipt_id": receipt.receipt_id.as_str(),
        "source": receipt.admission.action.source,
        "thread_id": receipt.admission.action.thread_id,
        "tool_name": receipt.admission.action.tool_name,
        "turn_id": receipt.admission.action.turn_id,
    })
}

fn parse_digest(value: &str, field: &str) -> Result<Sha256Digest, EvidenceError> {
    Sha256Digest::parse(value.to_string())
        .map_err(|error| invalid(format!("invalid {field} SHA-256: {error}")))
}

fn framed_digest<'a>(domain: &[u8], fields: impl IntoIterator<Item = &'a [u8]>) -> Sha256Digest {
    let mut preimage = Vec::new();
    preimage.extend_from_slice(&(domain.len() as u64).to_be_bytes());
    preimage.extend_from_slice(domain);
    for field in fields {
        preimage.extend_from_slice(&(field.len() as u64).to_be_bytes());
        preimage.extend_from_slice(field);
    }
    Sha256Digest::for_bytes(&preimage)
}

fn run_id(run_nonce_sha256: &str, started_at_ms: i64) -> Sha256Digest {
    let started = started_at_ms.to_string();
    framed_digest(
        RUN_ID_DOMAIN,
        [run_nonce_sha256.as_bytes(), started.as_bytes()],
    )
}

fn run_binding(run_id: &str) -> Sha256Digest {
    framed_digest(
        RUN_BINDING_DOMAIN,
        [
            run_id.as_bytes(),
            ORACLE_COMMIT.as_bytes(),
            ORACLE_TREE.as_bytes(),
            ORACLE_MANIFEST_SHA256.as_bytes(),
            ORACLE_CORPUS_SHA256.as_bytes(),
            ORACLE_GENERATOR_SHA256.as_bytes(),
            ORACLE_PROFILE.as_bytes(),
        ],
    )
}

fn segment_id(
    run_id: &str,
    segment_ordinal: i64,
    surface: &str,
    source_database_nonce_sha256: &str,
) -> Sha256Digest {
    let ordinal = segment_ordinal.to_string();
    framed_digest(
        SEGMENT_ID_DOMAIN,
        [
            run_id.as_bytes(),
            ordinal.as_bytes(),
            surface.as_bytes(),
            source_database_nonce_sha256.as_bytes(),
        ],
    )
}

fn segment_binding(segment_id: &str, run_binding: &str) -> Sha256Digest {
    framed_digest(
        SEGMENT_BINDING_DOMAIN,
        [segment_id.as_bytes(), run_binding.as_bytes()],
    )
}

fn intent_id(segment_id: &str, intent_ordinal: i64, sample_token_sha256: &str) -> Sha256Digest {
    let ordinal = intent_ordinal.to_string();
    framed_digest(
        INTENT_ID_DOMAIN,
        [
            segment_id.as_bytes(),
            ordinal.as_bytes(),
            sample_token_sha256.as_bytes(),
        ],
    )
}

fn intent_chain_digest(
    previous_intent_sha256: &str,
    intent_id: &str,
    provider_request_semantic_sha256: &str,
) -> Sha256Digest {
    framed_digest(
        INTENT_CHAIN_DOMAIN,
        [
            previous_intent_sha256.as_bytes(),
            intent_id.as_bytes(),
            provider_request_semantic_sha256.as_bytes(),
        ],
    )
}

struct ImportDigestFields<'a> {
    intent_id: &'a str,
    import_status: &'a str,
    artifact_path_sha256: &'a str,
    stable_bundle_manifest_sha256: &'a str,
    verification_snapshot_sha256: &'a str,
    transcript_sha256: &'a str,
    normalized_receipt_sha256: Option<&'a str>,
    oracle_sample_id_sha256: Option<&'a str>,
    imported_at_ms: i64,
}

fn import_digest(fields: &ImportDigestFields<'_>) -> Sha256Digest {
    let imported_at = fields.imported_at_ms.to_string();
    framed_digest(
        IMPORT_DOMAIN,
        [
            fields.intent_id.as_bytes(),
            fields.import_status.as_bytes(),
            fields.artifact_path_sha256.as_bytes(),
            fields.stable_bundle_manifest_sha256.as_bytes(),
            fields.verification_snapshot_sha256.as_bytes(),
            fields.transcript_sha256.as_bytes(),
            fields.normalized_receipt_sha256.unwrap_or("").as_bytes(),
            fields.oracle_sample_id_sha256.unwrap_or("").as_bytes(),
            imported_at.as_bytes(),
        ],
    )
}

fn terminal_digest(
    run_id: &str,
    terminal_status: &str,
    observed_intent_count: i64,
    observed_import_count: i64,
    evidence_set_sha256: &str,
    recorded_at_ms: i64,
) -> Sha256Digest {
    let intent_count = observed_intent_count.to_string();
    let import_count = observed_import_count.to_string();
    let recorded_at = recorded_at_ms.to_string();
    framed_digest(
        TERMINAL_DOMAIN,
        [
            run_id.as_bytes(),
            terminal_status.as_bytes(),
            intent_count.as_bytes(),
            import_count.as_bytes(),
            evidence_set_sha256.as_bytes(),
            recorded_at.as_bytes(),
        ],
    )
}

fn row_text(row: &SqliteRow, name: &str) -> Result<String, EvidenceError> {
    row.try_get(name).map_err(|error| {
        corrupt(format!(
            "invalid live product-Shadow v2 column {name}: {error}"
        ))
    })
}

fn row_optional_text(row: &SqliteRow, name: &str) -> Result<Option<String>, EvidenceError> {
    row.try_get(name).map_err(|error| {
        corrupt(format!(
            "invalid live product-Shadow v2 column {name}: {error}"
        ))
    })
}

fn row_i64(row: &SqliteRow, name: &str) -> Result<i64, EvidenceError> {
    row.try_get(name).map_err(|error| {
        corrupt(format!(
            "invalid live product-Shadow v2 column {name}: {error}"
        ))
    })
}

pub(crate) async fn verify_live_product_shadow_v2_integrity(
    pool: &SqlitePool,
) -> Result<(), EvidenceError> {
    let oracle = VerifiedLiveProductOracleV2::load_pinned()?;
    oracle.verify_internal_pins()?;
    // Keep every table scan on one SQLite snapshot. The staged foundation has
    // no public writer; any future concurrent importer must additionally own
    // the store-wide write lock and validate each row before committing it.
    let mut transaction = pool.begin().await.map_err(classify_sqlx_error)?;
    verify_run_rows(&mut transaction, &oracle).await?;
    verify_segment_rows(&mut transaction).await?;
    verify_intent_rows(&mut transaction).await?;
    verify_import_rows(&mut transaction, &oracle).await?;
    verify_terminal_rows(&mut transaction).await?;
    transaction.commit().await.map_err(classify_sqlx_error)?;
    Ok(())
}

async fn verify_run_rows(
    connection: &mut SqliteConnection,
    oracle: &VerifiedLiveProductOracleV2,
) -> Result<(), EvidenceError> {
    let mut rows = sqlx::query(
        "SELECT run_id, schema_version, run_binding_sha256, run_nonce_sha256, oracle_commit,
                oracle_tree, oracle_manifest_sha256, oracle_corpus_sha256,
                oracle_generator_sha256, oracle_profile, source_identity_status,
                exact_verified, oracle_live_reachable,
                actual_live_trial_closure_required, strict_artifact_import_required,
                qualification_status, governance_mode, enforce_enabled,
                promotion_authority_granted, outbound_enabled,
                retirement_authority_granted, operator_acceptance_recorded,
                started_at_ms, recorded_at_ms
         FROM live_product_shadow_v2_runs ORDER BY run_id",
    )
    .fetch(&mut *connection);
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        let run_id_value = row_text(&row, "run_id")?;
        let binding = row_text(&row, "run_binding_sha256")?;
        let nonce = row_text(&row, "run_nonce_sha256")?;
        let started_at = row_i64(&row, "started_at_ms")?;
        let recorded_at = row_i64(&row, "recorded_at_ms")?;
        if parse_digest(&run_id_value, "run id").is_err()
            || row_i64(&row, "schema_version")? != i64::from(LIVE_PRODUCT_SHADOW_V2_SCHEMA_VERSION)
            || parse_digest(&binding, "run binding").is_err()
            || parse_digest(&nonce, "run nonce").is_err()
            || run_id(&nonce, started_at).as_str() != run_id_value
            || run_binding(&run_id_value).as_str() != binding
            || row_text(&row, "oracle_commit")? != ORACLE_COMMIT
            || row_text(&row, "oracle_tree")? != ORACLE_TREE
            || row_text(&row, "oracle_manifest_sha256")? != ORACLE_MANIFEST_SHA256
            || row_text(&row, "oracle_corpus_sha256")? != oracle.corpus_sha256.as_str()
            || row_text(&row, "oracle_generator_sha256")? != ORACLE_GENERATOR_SHA256
            || row_text(&row, "oracle_profile")? != ORACLE_PROFILE
            || row_text(&row, "source_identity_status")? != "identity_claim"
            || row_i64(&row, "exact_verified")? != 0
            || row_i64(&row, "oracle_live_reachable")? != 0
            || row_i64(&row, "actual_live_trial_closure_required")? != 1
            || row_i64(&row, "strict_artifact_import_required")? != 1
            || row_text(&row, "qualification_status")? != "pending_strict_artifact_import"
            || row_text(&row, "governance_mode")? != "shadow"
            || row_i64(&row, "enforce_enabled")? != 0
            || row_i64(&row, "promotion_authority_granted")? != 0
            || row_i64(&row, "outbound_enabled")? != 0
            || row_i64(&row, "retirement_authority_granted")? != 0
            || row_i64(&row, "operator_acceptance_recorded")? != 0
            || started_at <= 0
            || recorded_at < started_at
        {
            return Err(corrupt(format!(
                "live product-Shadow v2 run {run_id_value} has an invalid binding or authority state"
            )));
        }
    }
    Ok(())
}

async fn verify_segment_rows(connection: &mut SqliteConnection) -> Result<(), EvidenceError> {
    let mut rows = sqlx::query(
        "SELECT segment.segment_id, segment.run_id, segment.schema_version,
                segment.segment_ordinal,
                segment.surface, segment.source_database_nonce_sha256,
                segment.source_database_fresh, segment.segment_binding_sha256,
                segment.opened_at_ms, segment.recorded_at_ms,
                run.run_binding_sha256,
                run.recorded_at_ms AS run_recorded_at_ms
         FROM live_product_shadow_v2_segments AS segment
         JOIN live_product_shadow_v2_runs AS run ON run.run_id = segment.run_id
         ORDER BY segment.run_id, segment.segment_ordinal",
    )
    .fetch(&mut *connection);
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        let segment = row_text(&row, "segment_id")?;
        let run = row_text(&row, "run_id")?;
        let ordinal = row_i64(&row, "segment_ordinal")?;
        let surface = row_text(&row, "surface")?;
        let nonce = row_text(&row, "source_database_nonce_sha256")?;
        let binding = row_text(&row, "segment_binding_sha256")?;
        let run_binding = row_text(&row, "run_binding_sha256")?;
        let opened_at = row_i64(&row, "opened_at_ms")?;
        let recorded_at = row_i64(&row, "recorded_at_ms")?;
        let run_recorded_at = row_i64(&row, "run_recorded_at_ms")?;
        let expected_surface = match ordinal {
            1 => "app_server",
            2 => "mcp",
            _ => "",
        };
        if surface != expected_surface
            || row_i64(&row, "schema_version")? != i64::from(LIVE_PRODUCT_SHADOW_V2_SCHEMA_VERSION)
            || parse_digest(&nonce, "source database nonce").is_err()
            || segment_id(&run, ordinal, &surface, &nonce).as_str() != segment
            || segment_binding(&segment, &run_binding).as_str() != binding
            || row_i64(&row, "source_database_fresh")? != 1
            || opened_at < run_recorded_at
            || recorded_at < opened_at
        {
            return Err(corrupt(format!(
                "live product-Shadow v2 segment {segment} has an invalid binding"
            )));
        }
    }
    Ok(())
}

async fn verify_intent_rows(connection: &mut SqliteConnection) -> Result<(), EvidenceError> {
    let mut rows = sqlx::query(
        "SELECT intent.intent_id, intent.run_id, intent.segment_id,
                intent.schema_version,
                intent.intent_ordinal, intent.previous_intent_sha256,
                intent.sample_token_sha256,
                intent.provider_request_semantic_sha256, intent.intent_sha256,
                intent.recorded_at_ms,
                segment.recorded_at_ms AS segment_recorded_at_ms
         FROM live_product_shadow_v2_pre_send_intents AS intent
         JOIN live_product_shadow_v2_segments AS segment
           ON segment.run_id = intent.run_id AND segment.segment_id = intent.segment_id
         ORDER BY intent.run_id, intent.segment_id, intent.intent_ordinal",
    )
    .fetch(&mut *connection);
    let mut current_segment = None::<String>;
    let mut last_ordinal = 0_i64;
    let mut head = ZERO_SHA256.to_string();
    let mut last_recorded_at = 0_i64;
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        let intent = row_text(&row, "intent_id")?;
        let segment = row_text(&row, "segment_id")?;
        let ordinal = row_i64(&row, "intent_ordinal")?;
        let previous = row_text(&row, "previous_intent_sha256")?;
        let sample = row_text(&row, "sample_token_sha256")?;
        let provider_request = row_text(&row, "provider_request_semantic_sha256")?;
        let chain = row_text(&row, "intent_sha256")?;
        let recorded_at = row_i64(&row, "recorded_at_ms")?;
        let segment_recorded_at = row_i64(&row, "segment_recorded_at_ms")?;
        if current_segment.as_deref() != Some(segment.as_str()) {
            current_segment = Some(segment.clone());
            last_ordinal = 0;
            head = ZERO_SHA256.to_string();
            last_recorded_at = segment_recorded_at;
        }
        if ordinal != last_ordinal + 1
            || row_i64(&row, "schema_version")? != i64::from(LIVE_PRODUCT_SHADOW_V2_SCHEMA_VERSION)
            || previous != head
            || intent_id(&segment, ordinal, &sample).as_str() != intent
            || intent_chain_digest(&previous, &intent, &provider_request).as_str() != chain
            || parse_digest(&sample, "sample token").is_err()
            || parse_digest(&provider_request, "provider request semantic binding").is_err()
            || recorded_at < segment_recorded_at
            || recorded_at < last_recorded_at
        {
            return Err(corrupt(format!(
                "live product-Shadow v2 intent {intent} has an invalid chain binding"
            )));
        }
        last_ordinal = ordinal;
        head = chain;
        last_recorded_at = recorded_at;
    }
    Ok(())
}

async fn verify_import_rows(
    connection: &mut SqliteConnection,
    oracle: &VerifiedLiveProductOracleV2,
) -> Result<(), EvidenceError> {
    let mut rows = sqlx::query(
        "SELECT imported.import_id, imported.intent_id, imported.schema_version,
                imported.importer_schema, imported.import_status,
                imported.artifact_path_sha256,
                imported.stable_bundle_manifest_sha256,
                imported.verification_snapshot_sha256,
                imported.transcript_sha256,
                imported.normalized_receipt_sha256,
                imported.oracle_sample_id_sha256,
                imported.strict_artifact_validated,
                imported.canonical_oracle_matched,
                imported.qualification_authority_granted,
                imported.import_sha256, imported.imported_at_ms,
                intent.recorded_at_ms AS intent_recorded_at_ms
         FROM live_product_shadow_v2_artifact_imports AS imported
         JOIN live_product_shadow_v2_pre_send_intents AS intent
           ON intent.run_id = imported.run_id
          AND intent.segment_id = imported.segment_id
          AND intent.intent_id = imported.intent_id
         ORDER BY imported.run_id, imported.segment_id, imported.intent_id",
    )
    .fetch(&mut *connection);
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        let import_id_value = row_text(&row, "import_id")?;
        let intent = row_text(&row, "intent_id")?;
        let status = row_text(&row, "import_status")?;
        let artifact_path = row_text(&row, "artifact_path_sha256")?;
        let stable_bundle = row_text(&row, "stable_bundle_manifest_sha256")?;
        let snapshot = row_text(&row, "verification_snapshot_sha256")?;
        let transcript = row_text(&row, "transcript_sha256")?;
        let normalized = row_optional_text(&row, "normalized_receipt_sha256")?;
        let sample = row_optional_text(&row, "oracle_sample_id_sha256")?;
        let imported_at = row_i64(&row, "imported_at_ms")?;
        let fields = ImportDigestFields {
            intent_id: &intent,
            import_status: &status,
            artifact_path_sha256: &artifact_path,
            stable_bundle_manifest_sha256: &stable_bundle,
            verification_snapshot_sha256: &snapshot,
            transcript_sha256: &transcript,
            normalized_receipt_sha256: normalized.as_deref(),
            oracle_sample_id_sha256: sample.as_deref(),
            imported_at_ms: imported_at,
        };
        let expected = import_digest(&fields);
        let strict_matches = status == "strict_verified"
            && row_i64(&row, "strict_artifact_validated")? == 1
            && row_i64(&row, "canonical_oracle_matched")? == 1
            && normalized.as_deref() == Some(oracle.normalized_receipt_sha256.as_str())
            && sample.as_deref() == Some(oracle.sample_id_sha256.as_str());
        let rejected_matches = status == "rejected"
            && row_i64(&row, "strict_artifact_validated")? == 0
            && row_i64(&row, "canonical_oracle_matched")? == 0
            && normalized.is_none()
            && sample.is_none();
        if expected.as_str() != import_id_value
            || row_i64(&row, "schema_version")? != i64::from(LIVE_PRODUCT_SHADOW_V2_SCHEMA_VERSION)
            || row_text(&row, "importer_schema")?
                != "hepta_live_product_shadow_strict_artifact_import_v2"
            || row_text(&row, "import_sha256")? != import_id_value
            || (!strict_matches && !rejected_matches)
            || row_i64(&row, "qualification_authority_granted")? != 0
            || imported_at < row_i64(&row, "intent_recorded_at_ms")?
            || [
                artifact_path.as_str(),
                stable_bundle.as_str(),
                snapshot.as_str(),
                transcript.as_str(),
            ]
            .iter()
            .any(|value| parse_digest(value, "artifact import binding").is_err())
        {
            return Err(corrupt(format!(
                "live product-Shadow v2 artifact import {import_id_value} has an invalid strict-import binding"
            )));
        }
    }
    Ok(())
}

async fn evidence_set_digest(
    connection: &mut SqliteConnection,
    run_id: &str,
) -> Result<Sha256Digest, EvidenceError> {
    let rows = sqlx::query(
        "SELECT segment.segment_id, intent.intent_id, imported.import_id
         FROM live_product_shadow_v2_segments AS segment
         LEFT JOIN live_product_shadow_v2_pre_send_intents AS intent
           ON intent.segment_id = segment.segment_id
         LEFT JOIN live_product_shadow_v2_artifact_imports AS imported
           ON imported.intent_id = intent.intent_id
         WHERE segment.run_id = ?
         ORDER BY segment.segment_ordinal, intent.intent_ordinal",
    )
    .bind(run_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(classify_sqlx_error)?;
    let mut fields = Vec::<String>::with_capacity(rows.len() * 2 + 1);
    fields.push(run_id.to_string());
    let mut current_segment = None::<String>;
    for row in rows {
        let segment = row_text(&row, "segment_id")?;
        if current_segment.as_deref() != Some(segment.as_str()) {
            fields.push(segment.clone());
            current_segment = Some(segment);
        }
        if let Some(intent) = row_optional_text(&row, "intent_id")? {
            fields.push(intent);
            fields.push(
                row_optional_text(&row, "import_id")?.unwrap_or_else(|| ZERO_SHA256.to_string()),
            );
        }
    }
    Ok(framed_digest(
        EVIDENCE_SET_DOMAIN,
        fields.iter().map(std::string::String::as_bytes),
    ))
}

async fn verify_terminal_rows(connection: &mut SqliteConnection) -> Result<(), EvidenceError> {
    let mut after_run_id = String::new();
    loop {
        // Keyset pagination releases the connection before the aggregate
        // queries below, while preserving the transaction's single snapshot.
        let row = sqlx::query(
            "SELECT terminal.terminal_id, terminal.run_id, terminal.schema_version,
                terminal.terminal_status, terminal.observed_intent_count,
                observed_import_count, evidence_set_sha256,
                strict_imports_complete, canonical_oracle_all_matched,
                clean_qualified, duration_claimed, terminal.exact_verified,
                terminal.promotion_authority_granted,
                terminal.operator_acceptance_recorded,
                terminal.enforce_enabled, terminal.outbound_enabled,
                terminal.retirement_authority_granted,
                terminal_sha256, terminal.recorded_at_ms,
                run.recorded_at_ms AS run_recorded_at_ms
         FROM live_product_shadow_v2_terminals AS terminal
         JOIN live_product_shadow_v2_runs AS run ON run.run_id = terminal.run_id
         WHERE terminal.run_id > ?
         ORDER BY terminal.run_id
         LIMIT 1",
        )
        .bind(&after_run_id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
        let Some(row) = row else {
            break;
        };
        let terminal = row_text(&row, "terminal_id")?;
        let run = row_text(&row, "run_id")?;
        let status = row_text(&row, "terminal_status")?;
        let intent_count = row_i64(&row, "observed_intent_count")?;
        let import_count = row_i64(&row, "observed_import_count")?;
        let evidence_set = row_text(&row, "evidence_set_sha256")?;
        let recorded_at = row_i64(&row, "recorded_at_ms")?;
        let actual_intents: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM live_product_shadow_v2_pre_send_intents WHERE run_id = ?",
        )
        .bind(&run)
        .fetch_one(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
        let actual_imports: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM live_product_shadow_v2_artifact_imports WHERE run_id = ?",
        )
        .bind(&run)
        .fetch_one(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
        let strict_imports: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM live_product_shadow_v2_artifact_imports
             WHERE run_id = ? AND import_status = 'strict_verified'
               AND strict_artifact_validated = 1 AND canonical_oracle_matched = 1",
        )
        .bind(&run)
        .fetch_one(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
        let segment_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM live_product_shadow_v2_segments WHERE run_id = ?",
        )
        .bind(&run)
        .fetch_one(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
        let app_server_intents: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM live_product_shadow_v2_pre_send_intents AS intent
             JOIN live_product_shadow_v2_segments AS segment
               ON segment.segment_id = intent.segment_id
             WHERE intent.run_id = ? AND segment.surface = 'app_server'",
        )
        .bind(&run)
        .fetch_one(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
        let mcp_intents: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM live_product_shadow_v2_pre_send_intents AS intent
             JOIN live_product_shadow_v2_segments AS segment
               ON segment.segment_id = intent.segment_id
             WHERE intent.run_id = ? AND segment.surface = 'mcp'",
        )
        .bind(&run)
        .fetch_one(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
        let latest_evidence_ms: i64 = sqlx::query_scalar(
            "SELECT MAX(evidence_time_ms)
             FROM (
                 SELECT recorded_at_ms AS evidence_time_ms
                 FROM live_product_shadow_v2_runs WHERE run_id = ?
                 UNION ALL
                 SELECT recorded_at_ms
                 FROM live_product_shadow_v2_segments WHERE run_id = ?
                 UNION ALL
                 SELECT recorded_at_ms
                 FROM live_product_shadow_v2_pre_send_intents WHERE run_id = ?
                 UNION ALL
                 SELECT imported_at_ms
                 FROM live_product_shadow_v2_artifact_imports WHERE run_id = ?
             )",
        )
        .bind(&run)
        .bind(&run)
        .bind(&run)
        .bind(&run)
        .fetch_one(&mut *connection)
        .await
        .map_err(classify_sqlx_error)?;
        let expected_set = evidence_set_digest(connection, &run).await?;
        let expected_terminal = terminal_digest(
            &run,
            &status,
            intent_count,
            import_count,
            &evidence_set,
            recorded_at,
        );
        let strict_state = status == "strict_artifact_import_complete"
            && row_i64(&row, "strict_imports_complete")? == 1
            && row_i64(&row, "canonical_oracle_all_matched")? == 1
            && intent_count == 4
            && import_count == 4
            && strict_imports == 4
            && segment_count == 2
            && app_server_intents == 2
            && mcp_intents == 2;
        let negative_state = matches!(status.as_str(), "failed" | "incomplete")
            && row_i64(&row, "strict_imports_complete")? == 0
            && row_i64(&row, "canonical_oracle_all_matched")? == 0;
        for name in [
            "clean_qualified",
            "duration_claimed",
            "exact_verified",
            "promotion_authority_granted",
            "operator_acceptance_recorded",
            "enforce_enabled",
            "outbound_enabled",
            "retirement_authority_granted",
        ] {
            if row_i64(&row, name)? != 0 {
                return Err(corrupt(format!(
                    "live product-Shadow v2 terminal {terminal} has authority-bearing state"
                )));
            }
        }
        if intent_count != actual_intents
            || row_i64(&row, "schema_version")? != i64::from(LIVE_PRODUCT_SHADOW_V2_SCHEMA_VERSION)
            || import_count != actual_imports
            || expected_set.as_str() != evidence_set
            || expected_terminal.as_str() != terminal
            || row_text(&row, "terminal_sha256")? != terminal
            || recorded_at < row_i64(&row, "run_recorded_at_ms")?
            || recorded_at < latest_evidence_ms
            || (!strict_state && !negative_state)
        {
            return Err(corrupt(format!(
                "live product-Shadow v2 terminal {terminal} has an invalid non-authoritative state"
            )));
        }
        after_run_id = run;
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    #[derive(Clone)]
    pub(crate) struct SeededFoundation {
        pub(crate) run_id: String,
        pub(crate) app_segment_id: String,
        pub(crate) intent_id: Option<String>,
        pub(crate) intent_sha256: Option<String>,
    }

    #[derive(Clone)]
    pub(crate) struct SeededRun {
        pub(crate) run_id: String,
        pub(crate) run_binding_sha256: String,
        pub(crate) started_at_ms: i64,
    }

    #[derive(Clone)]
    pub(crate) struct SeededStrictComplete {
        pub(crate) run_id: String,
        pub(crate) segment_ids: Vec<String>,
        pub(crate) intent_ids: Vec<String>,
        pub(crate) import_ids: Vec<String>,
        pub(crate) terminal_id: String,
    }

    pub(crate) fn digest(label: &str) -> String {
        Sha256Digest::for_bytes(label.as_bytes())
            .as_str()
            .to_string()
    }

    pub(crate) async fn seed_run_only(
        pool: &SqlitePool,
        nonce_label: &str,
    ) -> Result<SeededRun, EvidenceError> {
        let started_at = 1_000_i64;
        let run_nonce = digest(nonce_label);
        let run = run_id(&run_nonce, started_at).as_str().to_string();
        let binding = run_binding(&run).as_str().to_string();
        sqlx::query(
            "INSERT INTO live_product_shadow_v2_runs (
                run_id, schema_version, run_binding_sha256, run_nonce_sha256,
                oracle_commit, oracle_tree, oracle_manifest_sha256,
                oracle_corpus_sha256, oracle_generator_sha256, oracle_profile,
                source_identity_status, exact_verified, oracle_live_reachable,
                actual_live_trial_closure_required, strict_artifact_import_required,
                qualification_status, governance_mode, enforce_enabled,
                promotion_authority_granted, outbound_enabled,
                retirement_authority_granted, operator_acceptance_recorded,
                started_at_ms, recorded_at_ms
             ) VALUES (?, 2, ?, ?, ?, ?, ?, ?, ?, ?, 'identity_claim', 0, 0,
                       1, 1, 'pending_strict_artifact_import', 'shadow', 0, 0,
                       0, 0, 0, ?, ?)",
        )
        .bind(&run)
        .bind(&binding)
        .bind(&run_nonce)
        .bind(ORACLE_COMMIT)
        .bind(ORACLE_TREE)
        .bind(ORACLE_MANIFEST_SHA256)
        .bind(ORACLE_CORPUS_SHA256)
        .bind(ORACLE_GENERATOR_SHA256)
        .bind(ORACLE_PROFILE)
        .bind(started_at)
        .bind(started_at + 1)
        .execute(pool)
        .await
        .map_err(classify_sqlx_error)?;
        Ok(SeededRun {
            run_id: run,
            run_binding_sha256: binding,
            started_at_ms: started_at,
        })
    }

    pub(crate) async fn insert_app_segment(
        pool: &SqlitePool,
        run: &SeededRun,
        database_nonce_label: &str,
        recorded_at_ms: i64,
    ) -> Result<String, EvidenceError> {
        let database_nonce = digest(database_nonce_label);
        let segment = segment_id(&run.run_id, 1, "app_server", &database_nonce)
            .as_str()
            .to_string();
        let binding = segment_binding(&segment, &run.run_binding_sha256)
            .as_str()
            .to_string();
        sqlx::query(
            "INSERT INTO live_product_shadow_v2_segments (
                segment_id, run_id, schema_version, segment_ordinal, surface,
                source_database_nonce_sha256, source_database_fresh,
                segment_binding_sha256, opened_at_ms, recorded_at_ms
             ) VALUES (?, ?, 2, 1, 'app_server', ?, 1, ?, ?, ?)",
        )
        .bind(&segment)
        .bind(&run.run_id)
        .bind(&database_nonce)
        .bind(&binding)
        .bind(recorded_at_ms)
        .bind(recorded_at_ms)
        .execute(pool)
        .await
        .map_err(classify_sqlx_error)?;
        Ok(segment)
    }

    pub(crate) async fn seed_pending_foundation(
        pool: &SqlitePool,
        with_intent: bool,
    ) -> Result<SeededFoundation, EvidenceError> {
        let seeded_run = seed_run_only(pool, "v2 test run nonce").await?;
        let started_at = seeded_run.started_at_ms;
        let run = seeded_run.run_id;
        let binding = seeded_run.run_binding_sha256;

        let mut app_segment_id = String::new();
        for (ordinal, surface) in [(1_i64, "app_server"), (2_i64, "mcp")] {
            let database_nonce = digest(&format!("{surface} database nonce"));
            let segment = segment_id(&run, ordinal, surface, &database_nonce)
                .as_str()
                .to_string();
            let segment_binding = segment_binding(&segment, &binding).as_str().to_string();
            sqlx::query(
                "INSERT INTO live_product_shadow_v2_segments (
                    segment_id, run_id, schema_version, segment_ordinal, surface,
                    source_database_nonce_sha256, source_database_fresh,
                    segment_binding_sha256, opened_at_ms, recorded_at_ms
                 ) VALUES (?, ?, 2, ?, ?, ?, 1, ?, ?, ?)",
            )
            .bind(&segment)
            .bind(&run)
            .bind(ordinal)
            .bind(surface)
            .bind(&database_nonce)
            .bind(&segment_binding)
            .bind(started_at + ordinal)
            .bind(started_at + ordinal)
            .execute(pool)
            .await
            .map_err(classify_sqlx_error)?;
            if ordinal == 1 {
                app_segment_id = segment;
            }
        }

        let (seeded_intent_id, seeded_intent_sha256) = if with_intent {
            let sample = digest("v2 test sample token");
            let provider = digest("v2 test provider request");
            let intent = intent_id(&app_segment_id, 1, &sample).as_str().to_string();
            let chain = intent_chain_digest(ZERO_SHA256, &intent, &provider)
                .as_str()
                .to_string();
            sqlx::query(
                "INSERT INTO live_product_shadow_v2_pre_send_intents (
                    intent_id, run_id, segment_id, schema_version, intent_ordinal,
                    previous_intent_sha256, sample_token_sha256,
                    provider_request_semantic_sha256, intent_sha256, recorded_at_ms
                 ) VALUES (?, ?, ?, 2, 1, ?, ?, ?, ?, ?)",
            )
            .bind(&intent)
            .bind(&run)
            .bind(&app_segment_id)
            .bind(ZERO_SHA256)
            .bind(&sample)
            .bind(&provider)
            .bind(&chain)
            .bind(started_at + 10)
            .execute(pool)
            .await
            .map_err(classify_sqlx_error)?;
            (Some(intent), Some(chain))
        } else {
            (None, None)
        };
        Ok(SeededFoundation {
            run_id: run,
            app_segment_id,
            intent_id: seeded_intent_id,
            intent_sha256: seeded_intent_sha256,
        })
    }

    pub(crate) async fn seed_incomplete_terminal(
        pool: &SqlitePool,
        run_id: &str,
        recorded_at_ms: i64,
    ) -> Result<String, EvidenceError> {
        let intent_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM live_product_shadow_v2_pre_send_intents WHERE run_id = ?",
        )
        .bind(run_id)
        .fetch_one(pool)
        .await
        .map_err(classify_sqlx_error)?;
        let import_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM live_product_shadow_v2_artifact_imports WHERE run_id = ?",
        )
        .bind(run_id)
        .fetch_one(pool)
        .await
        .map_err(classify_sqlx_error)?;
        let mut connection = pool.acquire().await.map_err(classify_sqlx_error)?;
        let evidence_set = evidence_set_digest(&mut connection, run_id).await?;
        drop(connection);
        let terminal = terminal_digest(
            run_id,
            "incomplete",
            intent_count,
            import_count,
            evidence_set.as_str(),
            recorded_at_ms,
        )
        .as_str()
        .to_string();
        sqlx::query(
            "INSERT INTO live_product_shadow_v2_terminals (
                terminal_id, run_id, schema_version, terminal_status,
                observed_intent_count, observed_import_count,
                evidence_set_sha256, strict_imports_complete,
                canonical_oracle_all_matched, clean_qualified,
                duration_claimed, exact_verified, promotion_authority_granted,
                operator_acceptance_recorded, enforce_enabled, outbound_enabled,
                retirement_authority_granted, terminal_sha256, recorded_at_ms
             ) VALUES (?, ?, 2, 'incomplete', ?, ?, ?, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ?, ?)",
        )
        .bind(&terminal)
        .bind(run_id)
        .bind(intent_count)
        .bind(import_count)
        .bind(evidence_set.as_str())
        .bind(&terminal)
        .bind(recorded_at_ms)
        .execute(pool)
        .await
        .map_err(classify_sqlx_error)?;
        Ok(terminal)
    }

    pub(crate) async fn seed_strict_complete_foundation(
        pool: &SqlitePool,
    ) -> Result<SeededStrictComplete, EvidenceError> {
        let run = seed_run_only(pool, "strict-complete-run").await?;
        let mut segment_ids = Vec::with_capacity(2);
        let mut intent_rows = Vec::<(String, String, i64)>::with_capacity(4);
        for (segment_ordinal, surface) in [(1_i64, "app_server"), (2_i64, "mcp")] {
            let database_nonce = digest(&format!("strict {surface} database nonce"));
            let segment = segment_id(&run.run_id, segment_ordinal, surface, &database_nonce)
                .as_str()
                .to_string();
            let binding = segment_binding(&segment, &run.run_binding_sha256)
                .as_str()
                .to_string();
            let segment_time = 1_001 + segment_ordinal;
            sqlx::query(
                "INSERT INTO live_product_shadow_v2_segments (
                    segment_id, run_id, schema_version, segment_ordinal, surface,
                    source_database_nonce_sha256, source_database_fresh,
                    segment_binding_sha256, opened_at_ms, recorded_at_ms
                 ) VALUES (?, ?, 2, ?, ?, ?, 1, ?, ?, ?)",
            )
            .bind(&segment)
            .bind(&run.run_id)
            .bind(segment_ordinal)
            .bind(surface)
            .bind(&database_nonce)
            .bind(&binding)
            .bind(segment_time)
            .bind(segment_time)
            .execute(pool)
            .await
            .map_err(classify_sqlx_error)?;

            let mut previous = ZERO_SHA256.to_string();
            for intent_ordinal in 1_i64..=2_i64 {
                let sample = digest(&format!("strict {surface} sample token {intent_ordinal}"));
                let provider = digest(&format!(
                    "strict {surface} provider request {intent_ordinal}"
                ));
                let intent = intent_id(&segment, intent_ordinal, &sample)
                    .as_str()
                    .to_string();
                let chain = intent_chain_digest(&previous, &intent, &provider)
                    .as_str()
                    .to_string();
                let intent_time = 1_010 + (segment_ordinal - 1) * 10 + intent_ordinal;
                sqlx::query(
                    "INSERT INTO live_product_shadow_v2_pre_send_intents (
                        intent_id, run_id, segment_id, schema_version,
                        intent_ordinal, previous_intent_sha256,
                        sample_token_sha256, provider_request_semantic_sha256,
                        intent_sha256, recorded_at_ms
                     ) VALUES (?, ?, ?, 2, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&intent)
                .bind(&run.run_id)
                .bind(&segment)
                .bind(intent_ordinal)
                .bind(&previous)
                .bind(&sample)
                .bind(&provider)
                .bind(&chain)
                .bind(intent_time)
                .execute(pool)
                .await
                .map_err(classify_sqlx_error)?;
                intent_rows.push((segment.clone(), intent, intent_time));
                previous = chain;
            }
            segment_ids.push(segment);
        }

        let mut intent_ids = Vec::with_capacity(4);
        let mut import_ids = Vec::with_capacity(4);
        for (index, (segment, intent, intent_time)) in intent_rows.into_iter().enumerate() {
            let artifact_path = digest(&format!("strict artifact path {index}"));
            let stable_bundle = digest(&format!("strict stable bundle {index}"));
            let snapshot = digest(&format!("strict verification snapshot {index}"));
            let transcript = digest(&format!("strict transcript {index}"));
            let imported_at = 1_030
                + i64::try_from(index).map_err(|error| {
                    EvidenceError::InvalidRecord(format!("test import index overflow: {error}"))
                })?;
            if imported_at < intent_time {
                return Err(EvidenceError::InvalidRecord(
                    "test strict import predates its intent".to_string(),
                ));
            }
            let fields = ImportDigestFields {
                intent_id: &intent,
                import_status: "strict_verified",
                artifact_path_sha256: &artifact_path,
                stable_bundle_manifest_sha256: &stable_bundle,
                verification_snapshot_sha256: &snapshot,
                transcript_sha256: &transcript,
                normalized_receipt_sha256: Some(NORMALIZED_RECEIPT_SHA256),
                oracle_sample_id_sha256: Some(ORACLE_SAMPLE_ID_SHA256),
                imported_at_ms: imported_at,
            };
            let imported = import_digest(&fields).as_str().to_string();
            sqlx::query(
                "INSERT INTO live_product_shadow_v2_artifact_imports (
                    import_id, run_id, segment_id, intent_id, schema_version,
                    importer_schema, import_status, artifact_path_sha256,
                    stable_bundle_manifest_sha256,
                    verification_snapshot_sha256, transcript_sha256,
                    normalized_receipt_sha256, oracle_sample_id_sha256,
                    strict_artifact_validated, canonical_oracle_matched,
                    qualification_authority_granted, import_sha256,
                    imported_at_ms
                 ) VALUES (?, ?, ?, ?, 2,
                           'hepta_live_product_shadow_strict_artifact_import_v2',
                           'strict_verified', ?, ?, ?, ?, ?, ?, 1, 1, 0, ?, ?)",
            )
            .bind(&imported)
            .bind(&run.run_id)
            .bind(&segment)
            .bind(&intent)
            .bind(&artifact_path)
            .bind(&stable_bundle)
            .bind(&snapshot)
            .bind(&transcript)
            .bind(NORMALIZED_RECEIPT_SHA256)
            .bind(ORACLE_SAMPLE_ID_SHA256)
            .bind(&imported)
            .bind(imported_at)
            .execute(pool)
            .await
            .map_err(classify_sqlx_error)?;
            intent_ids.push(intent);
            import_ids.push(imported);
        }

        let mut connection = pool.acquire().await.map_err(classify_sqlx_error)?;
        let evidence_set = evidence_set_digest(&mut connection, &run.run_id).await?;
        drop(connection);
        let terminal_time = 1_040_i64;
        let terminal = terminal_digest(
            &run.run_id,
            "strict_artifact_import_complete",
            4,
            4,
            evidence_set.as_str(),
            terminal_time,
        )
        .as_str()
        .to_string();
        sqlx::query(
            "INSERT INTO live_product_shadow_v2_terminals (
                terminal_id, run_id, schema_version, terminal_status,
                observed_intent_count, observed_import_count,
                evidence_set_sha256, strict_imports_complete,
                canonical_oracle_all_matched, clean_qualified,
                duration_claimed, exact_verified, promotion_authority_granted,
                operator_acceptance_recorded, enforce_enabled, outbound_enabled,
                retirement_authority_granted, terminal_sha256, recorded_at_ms
             ) VALUES (?, ?, 2, 'strict_artifact_import_complete', 4, 4, ?,
                       1, 1, 0, 0, 0, 0, 0, 0, 0, 0, ?, ?)",
        )
        .bind(&terminal)
        .bind(&run.run_id)
        .bind(evidence_set.as_str())
        .bind(&terminal)
        .bind(terminal_time)
        .execute(pool)
        .await
        .map_err(classify_sqlx_error)?;
        Ok(SeededStrictComplete {
            run_id: run.run_id,
            segment_ids,
            intent_ids,
            import_ids,
            terminal_id: terminal,
        })
    }

    pub(crate) fn pinned_receipt() -> GovernanceReceipt {
        VerifiedLiveProductOracleV2::load_pinned()
            .expect("pinned v2 corpus")
            .normalized_receipt
    }

    pub(crate) fn normalization_digest(receipt: &GovernanceReceipt) -> String {
        let normalized = normalize_live_product_receipt_v2(receipt).expect("normalize receipt");
        Sha256Digest::for_bytes(&canonical_json(&normalized).expect("canonical receipt"))
            .as_str()
            .to_string()
    }

    pub(crate) fn expected_normalized_receipt_sha256() -> &'static str {
        NORMALIZED_RECEIPT_SHA256
    }

    pub(crate) fn expected_sample_id_sha256() -> &'static str {
        ORACLE_SAMPLE_ID_SHA256
    }

    pub(crate) fn expected_corpus_sha256() -> &'static str {
        ORACLE_CORPUS_SHA256
    }

    pub(crate) fn expected_tracked_corpus_sha256() -> &'static str {
        TRACKED_ORACLE_SHA256
    }
}
