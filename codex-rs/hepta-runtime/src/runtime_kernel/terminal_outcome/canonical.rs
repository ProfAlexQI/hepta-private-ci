//! Canonical outcome envelope construction.

use hepta_contracts::AuthorizationDecision;
use hepta_contracts::ContentHash;
use serde_json::Value;
use serde_json::json;

use super::OutcomeMaterial;
use super::ToolDispatchTerminal;
use super::ToolEffectDisposition;
use super::ToolOutcomeFinalizationError;
use super::ToolOutcomeFinalizationInput;
use super::ToolOutputValidationStatus;
use super::ToolTransactionEvidence;
use super::hashing::FrameSink;
use super::hashing::HashFrames;
use super::hashing::append_candidate;
use super::hashing::append_capability;
use super::hashing::append_context;
use super::hashing::append_stamp;
use super::hashing::sensitive_hash;

const OUTCOME_DOMAIN: &str = "hepta.runtime.tool-outcome.v1";

pub(super) fn build_canonical_evidence(
    input: &ToolOutcomeFinalizationInput<'_>,
    scope_hash: &ContentHash,
) -> Result<(String, ContentHash), ToolOutcomeFinalizationError> {
    let mut out = EvidenceFrames::new();
    out.text("attempt.id", input.attempt_id);
    out.text("session.id", input.session_id);
    out.text("correlation.id", input.correlation_id);
    out.text("tool.name", input.tool_name);
    out.text("tool.operation", input.operation);
    append_capability(&mut out, "capability", &input.executor.capability);
    out.text("executor", input.executor.executor.as_str());
    out.text("executor.provider", &input.executor.provider);
    out.text(
        "executor.manifest_hash",
        input.executor.executor_manifest_hash.as_str(),
    );
    out.text(
        "executor.binding_hash",
        input.executor.binding_hash.as_str(),
    );
    out.text("payload.hash", input.payload_hash.as_str());
    out.text(
        "execution.idempotency_key",
        input.execution_idempotency_key.unwrap_or(""),
    );
    out.text(
        "execution.resource_summary_hash",
        input
            .execution_resource_summary_hash
            .map(ContentHash::as_str)
            .unwrap_or(""),
    );
    out.text(
        "execution.effect_plan_hash",
        input
            .execution_effect_plan_hash
            .map(ContentHash::as_str)
            .unwrap_or(""),
    );
    out.text(
        "execution.effect_ack_hash",
        input
            .execution_effect_ack_hash
            .map(ContentHash::as_str)
            .unwrap_or(""),
    );
    out.number("time.started_unix_ms", input.started_at_unix_ms);
    out.number("time.finished_unix_ms", input.finished_at_unix_ms);
    append_authorization(&mut out, input, scope_hash);
    append_terminal(&mut out, input.terminal);
    out.text("effect.disposition", effect_tag(input.effect));
    append_validation(&mut out, input.validation);
    append_material(&mut out, "content", input.content);
    append_material(&mut out, "provider_output", input.provider_output);
    append_material(&mut out, "final_output", input.final_output);
    append_transaction(&mut out, input.transaction);
    out.finish()
}

fn append_authorization(
    out: &mut EvidenceFrames,
    input: &ToolOutcomeFinalizationInput<'_>,
    scope_hash: &ContentHash,
) {
    let witness = input.authorization;
    let authorization = witness.authorization();
    out.text("authorization.id", authorization.id().as_str());
    out.number("authorization.revision", authorization.revision().get());
    out.text(
        "authorization.content_hash",
        authorization.content_hash().as_str(),
    );
    out.text("admission.id", authorization.admission().id().as_str());
    out.number(
        "admission.revision",
        authorization.admission().revision().get(),
    );
    out.text(
        "admission.content_hash",
        authorization.admission().content_hash().as_str(),
    );
    append_candidate(out, "candidate", authorization.candidate());
    append_context(
        out,
        "authorization.context",
        authorization.current_context(),
    );
    append_stamp(out, "authorization.policy", authorization.policy());
    out.text(
        "authorization.decided_by",
        authorization.decided_by().as_str(),
    );
    out.text("authorization.scope_hash", scope_hash.as_str());
    out.text(
        "authorization.decision",
        match authorization.decision() {
            AuthorizationDecision::Authorized { .. } => "authorized",
            AuthorizationDecision::Denied { .. } => "denied",
            _ => "unknown",
        },
    );
    out.text(
        "kernel.candidate_binding",
        witness.binding().candidate_hash().as_str(),
    );
    out.text(
        "kernel.payload_set_hash",
        witness.binding().payload_set_hash().as_str(),
    );
}

fn append_terminal(out: &mut EvidenceFrames, terminal: ToolDispatchTerminal<'_>) {
    let (tag, code, error, timeout_ms) = match terminal {
        ToolDispatchTerminal::Succeeded => ("succeeded", "ok", None, 0),
        ToolDispatchTerminal::DispatchBlocked { reason } => (
            "failed",
            "runtime.outcome_receipt_breaker_open",
            Some(reason),
            0,
        ),
        ToolDispatchTerminal::ToolError { error } => {
            ("failed", "tool.invoke_error", Some(error), 0)
        }
        ToolDispatchTerminal::StructuredOutputMissing => ("failed", "tool.output_missing", None, 0),
        ToolDispatchTerminal::OutputValidationFailed { error } => {
            ("failed", "tool.output_validation_failed", Some(error), 0)
        }
        ToolDispatchTerminal::ToolReportedFailure { error } => {
            ("failed", "tool.reported_failure", Some(error), 0)
        }
        ToolDispatchTerminal::TimedOut { timeout_ms, error } => {
            ("cancelled", "tool.native_timeout", Some(error), timeout_ms)
        }
        ToolDispatchTerminal::EventRecordingFailed { error } => (
            "failed",
            "runtime.tool_invoked_event_failed",
            Some(error),
            0,
        ),
        ToolDispatchTerminal::TransactionRecordingFailed { error } => (
            "failed",
            "runtime.write_transaction_record_failed",
            Some(error),
            0,
        ),
        ToolDispatchTerminal::DispatchFutureDropped => {
            ("cancelled", "tool.dispatch_future_dropped", None, 0)
        }
    };
    out.text("terminal.status", tag);
    out.text("terminal.code", code);
    out.number("terminal.timeout_ms", timeout_ms);
    out.sensitive("terminal.error_hash", error);
}

fn append_validation(out: &mut EvidenceFrames, status: ToolOutputValidationStatus<'_>) {
    let (tag, error) = match status {
        ToolOutputValidationStatus::NotRequired => ("not_required", None),
        ToolOutputValidationStatus::Missing => ("missing", None),
        ToolOutputValidationStatus::Valid => ("valid", None),
        ToolOutputValidationStatus::Invalid { error } => ("invalid", Some(error)),
    };
    out.text("validation.status", tag);
    out.sensitive("validation.error_hash", error);
}

fn append_transaction(out: &mut EvidenceFrames, evidence: ToolTransactionEvidence<'_>) {
    let (tag, id, group, entry_hash, error) = match evidence {
        ToolTransactionEvidence::NotApplicable => ("not_applicable", "", "", "", None),
        ToolTransactionEvidence::Preview => ("preview", "", "", "", None),
        ToolTransactionEvidence::Recorded {
            transaction_id,
            group_id,
            entry_hash,
        } => (
            "recorded",
            transaction_id,
            group_id.unwrap_or(""),
            entry_hash.as_str(),
            None,
        ),
        ToolTransactionEvidence::Failed {
            error,
            transaction_id,
            group_id,
            entry_hash,
        } => (
            "failed",
            transaction_id.unwrap_or(""),
            group_id.unwrap_or(""),
            entry_hash.map(ContentHash::as_str).unwrap_or(""),
            Some(error),
        ),
    };
    out.text("transaction.status", tag);
    out.text("transaction.id", id);
    out.text("transaction.group_id", group);
    out.text("transaction.entry_hash", entry_hash);
    out.sensitive("transaction.error_hash", error);
}

fn append_material(out: &mut EvidenceFrames, name: &str, material: OutcomeMaterial<'_>) {
    let hash = match material {
        OutcomeMaterial::Absent => None,
        #[cfg(test)]
        OutcomeMaterial::Hashed(hash) => Some(hash.clone()),
        OutcomeMaterial::Raw(raw) => Some(sensitive_hash(name, raw)),
    };
    out.text(
        &format!("{name}.presence"),
        if hash.is_some() { "present" } else { "absent" },
    );
    out.text(
        &format!("{name}.hash"),
        hash.as_ref().map(ContentHash::as_str).unwrap_or(""),
    );
}

fn effect_tag(effect: ToolEffectDisposition) -> &'static str {
    match effect {
        ToolEffectDisposition::None => "none",
        ToolEffectDisposition::Recorded => "recorded",
        ToolEffectDisposition::Unknown => "unknown",
    }
}

struct EvidenceFrames {
    hash: HashFrames,
    fields: Vec<(String, Value)>,
}

impl EvidenceFrames {
    fn new() -> Self {
        Self {
            hash: HashFrames::new(OUTCOME_DOMAIN),
            fields: Vec::new(),
        }
    }

    fn sensitive(&mut self, name: &str, raw: Option<&str>) {
        self.text(
            name,
            raw.map(|value| sensitive_hash(name, value))
                .as_ref()
                .map(ContentHash::as_str)
                .unwrap_or(""),
        );
    }

    fn finish(self) -> Result<(String, ContentHash), ToolOutcomeFinalizationError> {
        let evidence = serde_json::to_string(&json!({
            "domain": OUTCOME_DOMAIN,
            "fields": self.fields,
        }))
        .map_err(|_| ToolOutcomeFinalizationError::EvidenceSerialization)?;
        Ok((evidence, self.hash.finish()))
    }
}

impl FrameSink for EvidenceFrames {
    fn text(&mut self, name: &str, value: &str) {
        self.hash.text(name, value);
        self.fields
            .push((name.to_string(), Value::String(value.to_string())));
    }

    fn number(&mut self, name: &str, value: u64) {
        self.hash.number(name, value);
        self.fields.push((name.to_string(), Value::from(value)));
    }
}
