//! Strict verification of the runtime-owned terminal evidence envelope.
//!
//! Execution-intent resolution is stronger than ordinary outcome storage:
//! the complete canonical producer envelope must be self-consistent with the
//! durable pre-dispatch plan and receipt before the outstanding plan is
//! removed. The deterministic hashes below authenticate no principal; they
//! prove canonical envelope integrity inside the runtime/storage trust domain.

use std::collections::BTreeMap;

use hepta_contracts::CandidateRef;
use hepta_contracts::ContentHash;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::OutcomeReceipt;
use hepta_contracts::OutcomeStatus;
use hepta_contracts::RevisionStamp;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

use crate::outcome_store::ExecutionEffectAck;
use crate::outcome_store::ExecutionIntent;
use crate::outcome_store::OutcomeRecord;
use crate::outcome_store::OutcomeStoreError;
use crate::outcome_store::candidate_reference_hash;

const OUTCOME_DOMAIN: &str = "hepta.runtime.tool-outcome.v1";
const EXECUTOR_BINDING_DOMAIN: &str = "hepta.runtime.tool-executor-binding.v1";
const EXECUTOR_PRINCIPAL_DOMAIN: &str = "hepta.runtime.executor-principal.v1";
const RECEIPT_ID_DOMAIN: &str = "hepta.runtime.outcome-receipt-id.v1";
const RECEIPT_DOMAIN: &str = "hepta.runtime.outcome-receipt.v1";

#[derive(Clone, Copy)]
enum FieldKind {
    Text,
    Number,
}

const TERMINAL_FIELDS: &[(&str, FieldKind)] = &[
    ("attempt.id", FieldKind::Text),
    ("session.id", FieldKind::Text),
    ("correlation.id", FieldKind::Text),
    ("tool.name", FieldKind::Text),
    ("tool.operation", FieldKind::Text),
    ("capability.id", FieldKind::Text),
    ("capability.revision", FieldKind::Number),
    ("capability.manifest_hash", FieldKind::Text),
    ("capability.catalog.revision", FieldKind::Number),
    ("capability.catalog.content_hash", FieldKind::Text),
    ("executor", FieldKind::Text),
    ("executor.provider", FieldKind::Text),
    ("executor.manifest_hash", FieldKind::Text),
    ("executor.binding_hash", FieldKind::Text),
    ("payload.hash", FieldKind::Text),
    ("execution.idempotency_key", FieldKind::Text),
    ("execution.resource_summary_hash", FieldKind::Text),
    ("execution.effect_plan_hash", FieldKind::Text),
    ("execution.effect_ack_hash", FieldKind::Text),
    ("time.started_unix_ms", FieldKind::Number),
    ("time.finished_unix_ms", FieldKind::Number),
    ("authorization.id", FieldKind::Text),
    ("authorization.revision", FieldKind::Number),
    ("authorization.content_hash", FieldKind::Text),
    ("admission.id", FieldKind::Text),
    ("admission.revision", FieldKind::Number),
    ("admission.content_hash", FieldKind::Text),
    ("candidate.id", FieldKind::Text),
    ("candidate.revision", FieldKind::Number),
    ("candidate.content_hash", FieldKind::Text),
    ("candidate.context.observation.id", FieldKind::Text),
    ("candidate.context.observation.revision", FieldKind::Number),
    (
        "candidate.context.observation.content_hash",
        FieldKind::Text,
    ),
    ("candidate.context.state.revision", FieldKind::Number),
    ("candidate.context.state.content_hash", FieldKind::Text),
    ("candidate.context.policy.revision", FieldKind::Number),
    ("candidate.context.policy.content_hash", FieldKind::Text),
    (
        "candidate.context.capability_catalog.revision",
        FieldKind::Number,
    ),
    (
        "candidate.context.capability_catalog.content_hash",
        FieldKind::Text,
    ),
    ("candidate.context.preference.revision", FieldKind::Number),
    ("candidate.context.preference.content_hash", FieldKind::Text),
    ("candidate.action_hash", FieldKind::Text),
    ("candidate.metacontrol_hash", FieldKind::Text),
    ("candidate.payload_set_hash", FieldKind::Text),
    ("authorization.context.observation.id", FieldKind::Text),
    (
        "authorization.context.observation.revision",
        FieldKind::Number,
    ),
    (
        "authorization.context.observation.content_hash",
        FieldKind::Text,
    ),
    ("authorization.context.state.revision", FieldKind::Number),
    ("authorization.context.state.content_hash", FieldKind::Text),
    ("authorization.context.policy.revision", FieldKind::Number),
    ("authorization.context.policy.content_hash", FieldKind::Text),
    (
        "authorization.context.capability_catalog.revision",
        FieldKind::Number,
    ),
    (
        "authorization.context.capability_catalog.content_hash",
        FieldKind::Text,
    ),
    (
        "authorization.context.preference.revision",
        FieldKind::Number,
    ),
    (
        "authorization.context.preference.content_hash",
        FieldKind::Text,
    ),
    ("authorization.policy.revision", FieldKind::Number),
    ("authorization.policy.content_hash", FieldKind::Text),
    ("authorization.decided_by", FieldKind::Text),
    ("authorization.scope_hash", FieldKind::Text),
    ("authorization.decision", FieldKind::Text),
    ("kernel.candidate_binding", FieldKind::Text),
    ("kernel.payload_set_hash", FieldKind::Text),
    ("terminal.status", FieldKind::Text),
    ("terminal.code", FieldKind::Text),
    ("terminal.timeout_ms", FieldKind::Number),
    ("terminal.error_hash", FieldKind::Text),
    ("effect.disposition", FieldKind::Text),
    ("validation.status", FieldKind::Text),
    ("validation.error_hash", FieldKind::Text),
    ("content.presence", FieldKind::Text),
    ("content.hash", FieldKind::Text),
    ("provider_output.presence", FieldKind::Text),
    ("provider_output.hash", FieldKind::Text),
    ("final_output.presence", FieldKind::Text),
    ("final_output.hash", FieldKind::Text),
    ("transaction.status", FieldKind::Text),
    ("transaction.id", FieldKind::Text),
    ("transaction.group_id", FieldKind::Text),
    ("transaction.entry_hash", FieldKind::Text),
    ("transaction.error_hash", FieldKind::Text),
];

#[derive(Clone)]
enum EvidenceValue {
    Text(String),
    Number(u64),
}

struct TerminalEvidence {
    fields: BTreeMap<String, EvidenceValue>,
    evidence_hash: ContentHash,
}

#[derive(Serialize)]
struct CanonicalEnvelope<'a> {
    domain: &'a str,
    fields: &'a [(String, Value)],
}

pub(super) fn validate(
    intent: &ExecutionIntent,
    outcome: &OutcomeRecord,
    effect_ack: Option<&ExecutionEffectAck>,
) -> Result<(), OutcomeStoreError> {
    let evidence = parse_strict(intent, outcome.canonical_evidence())?;
    if &evidence.evidence_hash != outcome.canonical_evidence_hash()
        || &evidence.evidence_hash != outcome.receipt().outcome_hash()
    {
        return mismatch(intent, "canonical evidence hash");
    }
    validate_intent(intent, effect_ack, &evidence)?;
    validate_receipt(intent, outcome.receipt(), &evidence)?;
    validate_terminal_shape(intent, outcome.receipt(), &evidence)
}

fn parse_strict(
    intent: &ExecutionIntent,
    canonical_evidence: &str,
) -> Result<TerminalEvidence, OutcomeStoreError> {
    let envelope: Value =
        serde_json::from_str(canonical_evidence).map_err(|error| OutcomeStoreError::Corrupt {
            detail: format!(
                "terminal evidence for execution intent {} is invalid JSON: {error}",
                intent.attempt_id()
            ),
        })?;
    let object = envelope
        .as_object()
        .ok_or_else(|| corrupt(intent, "is not an object"))?;
    if object.len() != 2
        || object.get("domain").and_then(Value::as_str) != Some(OUTCOME_DOMAIN)
        || !object.contains_key("fields")
    {
        return mismatch(intent, "canonical envelope");
    }
    let values = object
        .get("fields")
        .and_then(Value::as_array)
        .ok_or_else(|| corrupt(intent, "lacks canonical fields"))?;
    if values.len() != TERMINAL_FIELDS.len() {
        return mismatch(intent, "canonical field count");
    }

    let mut hasher = FrameHasher::new(OUTCOME_DOMAIN);
    let mut fields = BTreeMap::new();
    let mut canonical_pairs = Vec::with_capacity(values.len());
    for (position, (raw, (expected_name, kind))) in values.iter().zip(TERMINAL_FIELDS).enumerate() {
        let pair = raw
            .as_array()
            .ok_or_else(|| corrupt(intent, "contains a non-pair field"))?;
        let [name, value] = pair.as_slice() else {
            return Err(corrupt(intent, "contains a malformed field pair"));
        };
        if name.as_str() != Some(expected_name) {
            return Err(OutcomeStoreError::Corrupt {
                detail: format!(
                    "terminal evidence field {} must be {} for execution intent {}",
                    position + 1,
                    expected_name,
                    intent.attempt_id()
                ),
            });
        }
        let parsed = match kind {
            FieldKind::Text => {
                let value = value
                    .as_str()
                    .ok_or_else(|| corrupt(intent, "contains a non-text field value"))?;
                hasher.text(expected_name, value);
                EvidenceValue::Text(value.to_owned())
            }
            FieldKind::Number => {
                let value = value
                    .as_u64()
                    .ok_or_else(|| corrupt(intent, "contains a non-u64 field value"))?;
                hasher.number(expected_name, value);
                EvidenceValue::Number(value)
            }
        };
        fields.insert((*expected_name).to_owned(), parsed);
        canonical_pairs.push(((*expected_name).to_owned(), value.clone()));
    }
    let canonical = serde_json::to_string(&CanonicalEnvelope {
        domain: OUTCOME_DOMAIN,
        fields: &canonical_pairs,
    })
    .map_err(|error| OutcomeStoreError::Corrupt {
        detail: format!(
            "terminal evidence for execution intent {} cannot be canonicalized: {error}",
            intent.attempt_id()
        ),
    })?;
    if canonical != canonical_evidence {
        return mismatch(intent, "canonical byte representation");
    }
    Ok(TerminalEvidence {
        fields,
        evidence_hash: hasher.finish(),
    })
}

fn validate_intent(
    intent: &ExecutionIntent,
    effect_ack: Option<&ExecutionEffectAck>,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    for (name, expected) in [
        ("attempt.id", intent.attempt_id()),
        ("session.id", intent.session_id()),
        ("correlation.id", intent.correlation_id()),
        ("tool.name", intent.tool_name()),
        ("tool.operation", intent.capability_operation()),
        ("capability.id", intent.capability_id()),
        (
            "capability.manifest_hash",
            intent.capability_manifest_hash().as_str(),
        ),
        ("executor", intent.executor_principal()),
        ("executor.provider", intent.capability_provider()),
        (
            "executor.manifest_hash",
            intent.capability_manifest_hash().as_str(),
        ),
        ("payload.hash", intent.payload_hash().as_str()),
        (
            "authorization.content_hash",
            intent.authorization_digest().as_str(),
        ),
        ("admission.content_hash", intent.admission_digest().as_str()),
        ("candidate.content_hash", intent.candidate_hash().as_str()),
        (
            "kernel.candidate_binding",
            intent.kernel_candidate_hash().as_str(),
        ),
        (
            "kernel.payload_set_hash",
            intent.payload_set_hash().as_str(),
        ),
        ("execution.idempotency_key", intent.idempotency_key()),
        (
            "execution.resource_summary_hash",
            intent.resource_summary_hash().as_str(),
        ),
        (
            "execution.effect_plan_hash",
            intent
                .effect_plan_hash()
                .map(ContentHash::as_str)
                .unwrap_or(""),
        ),
        (
            "execution.effect_ack_hash",
            effect_ack
                .map(ExecutionEffectAck::ack_hash)
                .map(ContentHash::as_str)
                .unwrap_or(""),
        ),
    ] {
        if evidence.text(intent, name)? != expected {
            return mismatch(intent, name);
        }
    }
    if intent.effect_plan_hash().is_some() != effect_ack.is_some() {
        return mismatch(intent, "execution effect acknowledgement presence");
    }
    if evidence.number(intent, "capability.revision")? != intent.capability_revision() {
        return mismatch(intent, "capability.revision");
    }
    if evidence.text(intent, "admission.id")? != intent.admission_id()
        || evidence.number(intent, "admission.revision")? != intent.admission_revision()
    {
        return mismatch(intent, "admission exact reference");
    }
    if evidence.text(intent, "candidate.payload_set_hash")? != intent.payload_set_hash().as_str()
        || evidence.text(intent, "authorization.decision")? != "authorized"
    {
        return mismatch(intent, "authorization payload binding");
    }
    let started = evidence.number(intent, "time.started_unix_ms")?;
    let finished = evidence.number(intent, "time.finished_unix_ms")?;
    if finished < started {
        return mismatch(intent, "terminal time range");
    }
    Ok(())
}

fn validate_receipt(
    intent: &ExecutionIntent,
    receipt: &OutcomeReceipt,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    validate_candidate(intent, receipt.candidate(), evidence)?;
    let authorization = receipt.authorization();
    for (name, expected) in [
        ("authorization.id", authorization.id().as_str()),
        (
            "authorization.content_hash",
            authorization.content_hash().as_str(),
        ),
        ("executor", receipt.executed_by().as_str()),
        (
            "kernel.payload_set_hash",
            receipt.payload_set_hash().as_str(),
        ),
    ] {
        if evidence.text(intent, name)? != expected {
            return mismatch(intent, name);
        }
    }
    if evidence.number(intent, "authorization.revision")? != authorization.revision().get() {
        return mismatch(intent, "authorization.revision");
    }
    validate_capability_catalog(intent, receipt.candidate().context(), evidence)?;
    validate_context(
        intent,
        "authorization.context",
        receipt.candidate().context(),
        evidence,
    )?;
    validate_stamp(
        intent,
        "authorization.policy",
        receipt.candidate().context().policy(),
        evidence,
    )?;

    let computed_binding = executor_binding_hash(intent, evidence)?;
    if evidence.text(intent, "executor.binding_hash")? != computed_binding.as_str() {
        return mismatch(intent, "executor.binding_hash");
    }
    let computed_principal = executor_principal(intent, evidence)?;
    if receipt.executed_by().as_str() != computed_principal
        || intent.executor_principal() != computed_principal
    {
        return mismatch(intent, "executor principal");
    }
    let computed_scope = authorization_scope_hash(intent, evidence)?;
    if evidence.text(intent, "authorization.scope_hash")? != computed_scope.as_str() {
        return mismatch(intent, "authorization.scope_hash");
    }
    let computed_authorization = authorization_record_hash(intent, evidence, &computed_scope)?;
    if authorization.content_hash() != &computed_authorization
        || intent.authorization_digest() != &computed_authorization
    {
        return mismatch(intent, "authorization.content_hash");
    }
    let computed_id = receipt_id(intent, evidence, receipt.outcome_hash())?;
    if receipt.id().as_str() != computed_id {
        return mismatch(intent, "receipt.id");
    }
    let computed_hash = receipt_hash(intent, receipt, evidence)?;
    if receipt.receipt_hash().as_str() != computed_hash.as_str() {
        return mismatch(intent, "receipt.hash");
    }
    Ok(())
}

fn validate_candidate(
    intent: &ExecutionIntent,
    candidate: &CandidateRef,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    if candidate_reference_hash(candidate) != *intent.candidate_reference_hash() {
        return mismatch(intent, "candidate.reference_hash");
    }
    for (name, expected) in [
        ("candidate.id", candidate.id().as_str()),
        ("candidate.content_hash", candidate.content_hash().as_str()),
        ("candidate.action_hash", candidate.action_hash().as_str()),
        (
            "candidate.metacontrol_hash",
            candidate.metacontrol_hash().as_str(),
        ),
        (
            "candidate.payload_set_hash",
            candidate.payload_set_hash().as_str(),
        ),
    ] {
        if evidence.text(intent, name)? != expected {
            return mismatch(intent, name);
        }
    }
    if evidence.number(intent, "candidate.revision")? != candidate.revision().get() {
        return mismatch(intent, "candidate.revision");
    }
    validate_context(intent, "candidate.context", candidate.context(), evidence)
}

fn validate_context(
    intent: &ExecutionIntent,
    prefix: &str,
    context: &FrozenTurnContext,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    let observation = context.observation();
    if evidence.text(intent, &format!("{prefix}.observation.id"))? != observation.id().as_str()
        || evidence.text(intent, &format!("{prefix}.observation.content_hash"))?
            != observation.content_hash().as_str()
        || evidence.number(intent, &format!("{prefix}.observation.revision"))?
            != observation.revision().get()
    {
        return mismatch(intent, prefix);
    }
    for (name, stamp) in [
        ("state", context.state()),
        ("policy", context.policy()),
        ("capability_catalog", context.capability_catalog()),
        ("preference", context.preference()),
    ] {
        validate_stamp(intent, &format!("{prefix}.{name}"), stamp, evidence)?;
    }
    Ok(())
}

fn validate_stamp(
    intent: &ExecutionIntent,
    prefix: &str,
    stamp: &RevisionStamp,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    if evidence.number(intent, &format!("{prefix}.revision"))? != stamp.revision().get()
        || evidence.text(intent, &format!("{prefix}.content_hash"))?
            != stamp.content_hash().as_str()
    {
        return mismatch(intent, prefix);
    }
    Ok(())
}

fn validate_capability_catalog(
    intent: &ExecutionIntent,
    context: &FrozenTurnContext,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    let catalog = context.capability_catalog();
    if evidence.number(intent, "capability.catalog.revision")? != catalog.revision().get()
        || evidence.text(intent, "capability.catalog.content_hash")?
            != catalog.content_hash().as_str()
    {
        return mismatch(intent, "capability.catalog");
    }
    Ok(())
}

fn validate_terminal_shape(
    intent: &ExecutionIntent,
    receipt: &OutcomeReceipt,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    let terminal_status = evidence.text(intent, "terminal.status")?;
    let terminal_code = evidence.text(intent, "terminal.code")?;
    let timeout_ms = evidence.number(intent, "terminal.timeout_ms")?;
    let allowed = matches!(
        (terminal_status, terminal_code),
        ("succeeded", "ok")
            | ("failed", "runtime.outcome_receipt_breaker_open")
            | ("failed", "tool.invoke_error")
            | ("failed", "tool.output_missing")
            | ("failed", "tool.output_validation_failed")
            | ("failed", "tool.reported_failure")
            | ("failed", "mutation_durability_ambiguous")
            | ("cancelled", "tool.native_timeout")
            | ("failed", "runtime.tool_invoked_event_failed")
            | ("failed", "runtime.write_transaction_record_failed")
            | ("cancelled", "tool.dispatch_future_dropped")
    );
    if !allowed {
        return mismatch(intent, "terminal status/code");
    }
    let status_matches = match receipt.status() {
        OutcomeStatus::Succeeded => terminal_status == "succeeded" && terminal_code == "ok",
        OutcomeStatus::Failed { error_code } => {
            terminal_status == "failed" && terminal_code == error_code
        }
        OutcomeStatus::Cancelled { reason_code } => {
            terminal_status == "cancelled" && terminal_code == reason_code
        }
        _ => false,
    };
    if !status_matches
        || (terminal_code == "tool.native_timeout" && timeout_ms == 0)
        || (terminal_code != "tool.native_timeout" && timeout_ms != 0)
    {
        return mismatch(intent, "terminal receipt status");
    }
    let terminal_error = evidence.text(intent, "terminal.error_hash")?;
    let terminal_error_expected = !matches!(
        terminal_code,
        "ok" | "tool.output_missing" | "tool.dispatch_future_dropped"
    );
    if terminal_error_expected == terminal_error.is_empty()
        || (!terminal_error.is_empty() && !is_canonical_sha256(terminal_error))
    {
        return mismatch(intent, "terminal.error_hash");
    }

    let validation = evidence.text(intent, "validation.status")?;
    let validation_error = evidence.text(intent, "validation.error_hash")?;
    if !matches!(validation, "not_required" | "missing" | "valid" | "invalid")
        || (validation == "invalid") == validation_error.is_empty()
        || (!validation_error.is_empty() && !is_canonical_sha256(validation_error))
        || (terminal_code == "tool.output_missing" && validation != "missing")
        || (terminal_code == "tool.output_validation_failed" && validation != "invalid")
        || (terminal_status == "succeeded" && matches!(validation, "missing" | "invalid"))
    {
        return mismatch(intent, "validation shape");
    }
    for prefix in ["content", "provider_output", "final_output"] {
        validate_material(intent, prefix, evidence)?;
    }
    validate_transaction(intent, terminal_code, evidence)
}

fn validate_material(
    intent: &ExecutionIntent,
    prefix: &str,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    let presence = evidence.text(intent, &format!("{prefix}.presence"))?;
    let hash = evidence.text(intent, &format!("{prefix}.hash"))?;
    if !matches!(presence, "present" | "absent")
        || (presence == "present") == hash.is_empty()
        || (!hash.is_empty() && !is_canonical_sha256(hash))
    {
        return mismatch(intent, prefix);
    }
    Ok(())
}

fn validate_transaction(
    intent: &ExecutionIntent,
    terminal_code: &str,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    let effect = evidence.text(intent, "effect.disposition")?;
    let status = evidence.text(intent, "transaction.status")?;
    let id = evidence.text(intent, "transaction.id")?;
    let group = evidence.text(intent, "transaction.group_id")?;
    let entry = evidence.text(intent, "transaction.entry_hash")?;
    let error = evidence.text(intent, "transaction.error_hash")?;
    if !matches!(effect, "none" | "recorded" | "unknown")
        || !matches!(status, "not_applicable" | "preview" | "recorded" | "failed")
    {
        return mismatch(intent, "transaction enum");
    }
    let shape_matches = match status {
        "not_applicable" | "preview" => {
            id.is_empty() && group.is_empty() && entry.is_empty() && error.is_empty()
        }
        "recorded" => !id.is_empty() && !entry.is_empty() && error.is_empty(),
        "failed" => !error.is_empty(),
        _ => false,
    };
    if !shape_matches
        || (!entry.is_empty() && !is_canonical_sha256(entry))
        || (!error.is_empty() && !is_canonical_sha256(error))
        || (effect == "recorded" && status != "recorded")
        || (status == "recorded" && effect != "recorded")
        || (terminal_code == "runtime.write_transaction_record_failed" && status != "failed")
        || (status == "failed" && effect == "recorded")
    {
        return mismatch(intent, "transaction shape");
    }
    Ok(())
}

fn receipt_id(
    intent: &ExecutionIntent,
    evidence: &TerminalEvidence,
    outcome_hash: &ContentHash,
) -> Result<String, OutcomeStoreError> {
    let mut hash = FrameHasher::new(RECEIPT_ID_DOMAIN);
    hash.text("attempt.id", evidence.text(intent, "attempt.id")?);
    hash.text(
        "authorization.id",
        evidence.text(intent, "authorization.id")?,
    );
    hash.number(
        "authorization.revision",
        evidence.number(intent, "authorization.revision")?,
    );
    hash.text(
        "authorization.content_hash",
        evidence.text(intent, "authorization.content_hash")?,
    );
    hash.text("outcome.hash", outcome_hash.as_str());
    Ok(format!("receipt:{}", hash.finish().as_str()))
}

fn authorization_scope_hash(
    intent: &ExecutionIntent,
    evidence: &TerminalEvidence,
) -> Result<ContentHash, OutcomeStoreError> {
    let mut hash = FrameHasher::new("hepta.runtime.authorization-scope.v2");
    hash.text("attempt_id", evidence.text(intent, "attempt.id")?);
    hash.text("tool_name", evidence.text(intent, "tool.name")?);
    hash.text("payload_hash", evidence.text(intent, "payload.hash")?);
    hash.text("capability_id", evidence.text(intent, "capability.id")?);
    hash.text(
        "capability_revision",
        &evidence.number(intent, "capability.revision")?.to_string(),
    );
    hash.text(
        "capability_manifest_hash",
        evidence.text(intent, "capability.manifest_hash")?,
    );
    hash.text(
        "catalog_revision",
        &evidence
            .number(intent, "capability.catalog.revision")?
            .to_string(),
    );
    hash.text("executor_principal", evidence.text(intent, "executor")?);
    hash.text(
        "executor_provider",
        evidence.text(intent, "executor.provider")?,
    );
    hash.text(
        "executor_operation",
        evidence.text(intent, "tool.operation")?,
    );
    hash.text(
        "executor_manifest_hash",
        evidence.text(intent, "executor.manifest_hash")?,
    );
    hash.text(
        "state",
        evidence.text(intent, "authorization.context.state.content_hash")?,
    );
    hash.text(
        "policy",
        evidence.text(intent, "authorization.context.policy.content_hash")?,
    );
    hash.text(
        "catalog",
        evidence.text(
            intent,
            "authorization.context.capability_catalog.content_hash",
        )?,
    );
    Ok(hash.finish())
}

fn authorization_record_hash(
    intent: &ExecutionIntent,
    evidence: &TerminalEvidence,
    scope_hash: &ContentHash,
) -> Result<ContentHash, OutcomeStoreError> {
    let mut hash = FrameHasher::new("hepta.kernel.safety-gate.authorization-record.v1");
    hash.text(
        "authorization.id",
        evidence.text(intent, "authorization.id")?,
    );
    hash.number(
        "authorization.revision",
        evidence.number(intent, "authorization.revision")?,
    );
    hash.text(
        "authorization.admission.content_hash",
        evidence.text(intent, "admission.content_hash")?,
    );
    hash.text(
        "authorization.candidate_binding",
        evidence.text(intent, "kernel.candidate_binding")?,
    );
    hash.text(
        "authorization.payload_set_hash",
        evidence.text(intent, "kernel.payload_set_hash")?,
    );
    append_evidence_context(
        intent,
        &mut hash,
        "authorization.current_context",
        "authorization.context",
        evidence,
    )?;
    hash.text(
        "authorization.decided_by",
        evidence.text(intent, "authorization.decided_by")?,
    );
    hash.text("authorization.scope_hash", scope_hash.as_str());
    Ok(hash.finish())
}

fn receipt_hash(
    intent: &ExecutionIntent,
    receipt: &OutcomeReceipt,
    evidence: &TerminalEvidence,
) -> Result<ContentHash, OutcomeStoreError> {
    let mut hash = FrameHasher::new(RECEIPT_DOMAIN);
    hash.text("receipt.id", receipt.id().as_str());
    append_candidate(&mut hash, receipt.candidate());
    hash.text("authorization.id", receipt.authorization().id().as_str());
    hash.number(
        "authorization.revision",
        receipt.authorization().revision().get(),
    );
    hash.text(
        "authorization.content_hash",
        receipt.authorization().content_hash().as_str(),
    );
    append_capability(intent, &mut hash, evidence)?;
    hash.text("payload_set_hash", evidence.text(intent, "payload.hash")?);
    hash.text("executor", evidence.text(intent, "executor")?);
    hash.text(
        "executor.provider",
        evidence.text(intent, "executor.provider")?,
    );
    hash.text(
        "executor.manifest_hash",
        evidence.text(intent, "executor.manifest_hash")?,
    );
    hash.text(
        "executor.binding_hash",
        evidence.text(intent, "executor.binding_hash")?,
    );
    let (tag, code) = receipt_status(receipt.status());
    hash.text("status.tag", tag);
    hash.text("status.code", code);
    hash.text("outcome.hash", receipt.outcome_hash().as_str());
    Ok(hash.finish())
}

fn executor_binding_hash(
    intent: &ExecutionIntent,
    evidence: &TerminalEvidence,
) -> Result<ContentHash, OutcomeStoreError> {
    let mut hash = FrameHasher::new(EXECUTOR_BINDING_DOMAIN);
    append_capability(intent, &mut hash, evidence)?;
    hash.text("executor", evidence.text(intent, "executor")?);
    hash.text(
        "executor.provider",
        evidence.text(intent, "executor.provider")?,
    );
    hash.text(
        "executor.manifest_hash",
        evidence.text(intent, "executor.manifest_hash")?,
    );
    Ok(hash.finish())
}

fn executor_principal(
    intent: &ExecutionIntent,
    evidence: &TerminalEvidence,
) -> Result<String, OutcomeStoreError> {
    let mut hash = FrameHasher::new(EXECUTOR_PRINCIPAL_DOMAIN);
    hash.text("provider", evidence.text(intent, "executor.provider")?);
    hash.text("operation", evidence.text(intent, "tool.operation")?);
    hash.text(
        "manifest_hash",
        evidence.text(intent, "executor.manifest_hash")?,
    );
    Ok(format!("executor:{}", hash.finish().as_str()))
}

fn append_candidate(hash: &mut FrameHasher, candidate: &CandidateRef) {
    hash.text("candidate.id", candidate.id().as_str());
    hash.number("candidate.revision", candidate.revision().get());
    hash.text("candidate.content_hash", candidate.content_hash().as_str());
    append_context(hash, "candidate.context", candidate.context());
    hash.text("candidate.action_hash", candidate.action_hash().as_str());
    hash.text(
        "candidate.metacontrol_hash",
        candidate.metacontrol_hash().as_str(),
    );
    hash.text(
        "candidate.payload_set_hash",
        candidate.payload_set_hash().as_str(),
    );
}

fn append_capability(
    intent: &ExecutionIntent,
    hash: &mut FrameHasher,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    hash.text("capability.id", evidence.text(intent, "capability.id")?);
    hash.number(
        "capability.revision",
        evidence.number(intent, "capability.revision")?,
    );
    hash.text(
        "capability.manifest_hash",
        evidence.text(intent, "capability.manifest_hash")?,
    );
    hash.number(
        "capability.catalog.revision",
        evidence.number(intent, "capability.catalog.revision")?,
    );
    hash.text(
        "capability.catalog.content_hash",
        evidence.text(intent, "capability.catalog.content_hash")?,
    );
    Ok(())
}

fn append_context(hash: &mut FrameHasher, prefix: &str, context: &FrozenTurnContext) {
    hash.text(
        &format!("{prefix}.observation.id"),
        context.observation().id().as_str(),
    );
    hash.number(
        &format!("{prefix}.observation.revision"),
        context.observation().revision().get(),
    );
    hash.text(
        &format!("{prefix}.observation.content_hash"),
        context.observation().content_hash().as_str(),
    );
    append_stamp(hash, &format!("{prefix}.state"), context.state());
    append_stamp(hash, &format!("{prefix}.policy"), context.policy());
    append_stamp(
        hash,
        &format!("{prefix}.capability_catalog"),
        context.capability_catalog(),
    );
    append_stamp(hash, &format!("{prefix}.preference"), context.preference());
}

fn append_evidence_context(
    intent: &ExecutionIntent,
    hash: &mut FrameHasher,
    hash_prefix: &str,
    evidence_prefix: &str,
    evidence: &TerminalEvidence,
) -> Result<(), OutcomeStoreError> {
    hash.text(
        &format!("{hash_prefix}.observation.id"),
        evidence.text(intent, &format!("{evidence_prefix}.observation.id"))?,
    );
    hash.number(
        &format!("{hash_prefix}.observation.revision"),
        evidence.number(intent, &format!("{evidence_prefix}.observation.revision"))?,
    );
    hash.text(
        &format!("{hash_prefix}.observation.content_hash"),
        evidence.text(
            intent,
            &format!("{evidence_prefix}.observation.content_hash"),
        )?,
    );
    for field in ["state", "policy", "capability_catalog", "preference"] {
        hash.number(
            &format!("{hash_prefix}.{field}.revision"),
            evidence.number(intent, &format!("{evidence_prefix}.{field}.revision"))?,
        );
        hash.text(
            &format!("{hash_prefix}.{field}.content_hash"),
            evidence.text(intent, &format!("{evidence_prefix}.{field}.content_hash"))?,
        );
    }
    Ok(())
}

fn append_stamp(hash: &mut FrameHasher, prefix: &str, stamp: &RevisionStamp) {
    hash.number(&format!("{prefix}.revision"), stamp.revision().get());
    hash.text(
        &format!("{prefix}.content_hash"),
        stamp.content_hash().as_str(),
    );
}

fn receipt_status(status: &OutcomeStatus) -> (&'static str, &str) {
    match status {
        OutcomeStatus::Succeeded => ("succeeded", ""),
        OutcomeStatus::Failed { error_code } => ("failed", error_code),
        OutcomeStatus::Cancelled { reason_code } => ("cancelled", reason_code),
        _ => ("unknown", "unknown"),
    }
}

fn is_canonical_sha256(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

impl TerminalEvidence {
    fn text<'a>(
        &'a self,
        intent: &ExecutionIntent,
        name: &str,
    ) -> Result<&'a str, OutcomeStoreError> {
        match self.fields.get(name) {
            Some(EvidenceValue::Text(value)) => Ok(value),
            _ => mismatch(intent, name),
        }
    }

    fn number(&self, intent: &ExecutionIntent, name: &str) -> Result<u64, OutcomeStoreError> {
        match self.fields.get(name) {
            Some(EvidenceValue::Number(value)) => Ok(*value),
            _ => mismatch(intent, name),
        }
    }
}

struct FrameHasher(Sha256);

impl FrameHasher {
    fn new(domain: &str) -> Self {
        let mut value = Self(Sha256::new());
        value.bytes("domain", domain.as_bytes());
        value
    }

    fn bytes(&mut self, name: &str, value: &[u8]) {
        self.0.update((name.len() as u64).to_be_bytes());
        self.0.update(name.as_bytes());
        self.0.update((value.len() as u64).to_be_bytes());
        self.0.update(value);
    }

    fn text(&mut self, name: &str, value: &str) {
        self.bytes(name, value.as_bytes());
    }

    fn number(&mut self, name: &str, value: u64) {
        self.bytes(name, &value.to_be_bytes());
    }

    fn finish(self) -> ContentHash {
        ContentHash::new(format!("sha256:{:x}", self.0.finalize()))
    }
}

fn mismatch<T>(intent: &ExecutionIntent, field: &str) -> Result<T, OutcomeStoreError> {
    Err(corrupt(
        intent,
        &format!("field {field} does not match its exact binding"),
    ))
}

fn corrupt(intent: &ExecutionIntent, detail: &str) -> OutcomeStoreError {
    OutcomeStoreError::Corrupt {
        detail: format!(
            "terminal evidence for execution intent {} {detail}",
            intent.attempt_id()
        ),
    }
}
