use codex_hepta_contracts::Sha256Digest;

use crate::ProofError;
use crate::command::PROOF_SCHEMA_VERSION;
use crate::command::ProofIntent;
use crate::command::ProofInvocationId;
use crate::command::ProofReceipt;
use crate::command::ProofReceiptId;
use crate::command::ProofSubject;
use crate::command::ProofTerminal;
use crate::command::expected_receipt_sha256;

pub(crate) fn validate_intent(intent: &ProofIntent) -> Result<(), ProofError> {
    if intent.schema_version != PROOF_SCHEMA_VERSION {
        return Err(ProofError::Corrupt(
            "proof intent schema version is unsupported".to_string(),
        ));
    }
    validate_subject(&intent.subject)?;
    validate_digest(
        &intent.command_binding_sha256,
        "proof command binding digest",
    )?;
    validate_digest(&intent.nonce_sha256, "proof nonce digest")?;
    let expected = ProofInvocationId::for_intent(
        &intent.subject,
        &intent.command_binding_sha256,
        &intent.nonce_sha256,
    );
    if intent.invocation_id != expected {
        return Err(ProofError::Corrupt(
            "proof invocation ID does not match its bindings".to_string(),
        ));
    }
    Ok(())
}

pub(crate) fn validate_receipt(receipt: &ProofReceipt) -> Result<(), ProofError> {
    if receipt.schema_version() != PROOF_SCHEMA_VERSION {
        return Err(ProofError::Corrupt(
            "proof receipt schema version is unsupported".to_string(),
        ));
    }
    validate_subject(receipt.subject())?;
    validate_digest(
        receipt.command_binding_sha256(),
        "proof command binding digest",
    )?;
    validate_digest(receipt.receipt_sha256(), "proof receipt digest")?;
    if receipt.receipt_id() != &ProofReceiptId::for_invocation(receipt.invocation_id()) {
        return Err(ProofError::Corrupt(
            "proof receipt ID does not match its invocation".to_string(),
        ));
    }
    if receipt.finished_at_unix_ms() < receipt.started_at_unix_ms() {
        return Err(ProofError::Corrupt(
            "proof receipt time range is invalid".to_string(),
        ));
    }
    receipt
        .stdout()
        .validate_shape()
        .map_err(ProofError::Corrupt)?;
    receipt
        .stderr()
        .validate_shape()
        .map_err(ProofError::Corrupt)?;
    receipt
        .terminal()
        .validate_shape()
        .map_err(ProofError::Corrupt)?;
    match receipt.terminal() {
        ProofTerminal::Completed { success, exit_code } => {
            if !receipt.stdout().is_complete() || !receipt.stderr().is_complete() {
                return Err(ProofError::Corrupt(
                    "completed proof receipt has incomplete stream evidence".to_string(),
                ));
            }
            if *success != (*exit_code == Some(0)) {
                return Err(ProofError::Corrupt(
                    "completed proof success and exit code disagree".to_string(),
                ));
            }
        }
        ProofTerminal::TimedOut | ProofTerminal::OutputLimitExceeded { .. } => {
            if receipt.stdout().is_complete() || receipt.stderr().is_complete() {
                return Err(ProofError::Corrupt(
                    "interrupted proof receipt has complete stream evidence".to_string(),
                ));
            }
        }
        ProofTerminal::NotStarted { .. } => {
            if receipt.stdout().is_complete() || receipt.stderr().is_complete() {
                return Err(ProofError::Corrupt(
                    "unavailable proof receipt has complete stream evidence".to_string(),
                ));
            }
        }
        ProofTerminal::Indeterminate { .. } => {
            if receipt.stdout().is_complete() != receipt.stderr().is_complete() {
                return Err(ProofError::Corrupt(
                    "indeterminate proof streams disagree on completeness".to_string(),
                ));
            }
        }
    }
    if receipt.receipt_sha256() != &expected_receipt_sha256(receipt)? {
        return Err(ProofError::Corrupt(
            "proof receipt digest does not match its bindings".to_string(),
        ));
    }
    Ok(())
}

fn validate_subject(subject: &ProofSubject) -> Result<(), ProofError> {
    validate_digest(subject.candidate_sha256(), "proof candidate digest")?;
    validate_digest(subject.context_sha256(), "proof context digest")
}

fn validate_digest(digest: &Sha256Digest, label: &str) -> Result<(), ProofError> {
    Sha256Digest::parse(digest.as_str())
        .map(|_| ())
        .map_err(|_| ProofError::Corrupt(format!("{label} is not a canonical SHA-256 digest")))
}
