//! Exact reconciliation for operator mutation journal finalization.
//!
//! This surface reads an already-recorded RuntimeKernel terminal receipt and
//! may finalize only the matching product journal entry. It never invokes a
//! provider, reconstructs an effect, accepts a path, or retries execution.

use anyhow::Context;
use anyhow::Result;
use hepta_runtime::RuntimeExecutionReceipt;
use hepta_runtime::RuntimeKernel;
use hmac::Hmac;
use hmac::Mac;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;

use crate::operator_mutation::load_authority;
use crate::operator_mutation_journal::OperatorMutationJournal;
use crate::operator_mutation_journal::OperatorMutationJournalInspection;

pub(crate) const OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT: &str =
    "/api/v2/operator-mutations/note/reconciliation/inspect";
pub(crate) const OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT: &str =
    "/api/v2/operator-mutations/note/reconciliation/resolve";
pub(crate) const OPERATOR_MUTATION_RECONCILIATION_PROOF_DOMAIN: &str =
    "hepta.native.operator-note.reconciliation.v1";

const MAX_REQUEST_BYTES: usize = 4096;
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReconciliationRequest {
    plan_hash: String,
    attempt_id: String,
    effect_plan_hash: String,
    session_binding_hash: String,
    #[serde(default)]
    decision: Option<ReconciliationDecision>,
    proof: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum ReconciliationDecision {
    FinalizeProductJournalOnly,
}

#[derive(Debug, Serialize)]
struct ReconciliationResponse<'a> {
    schema: &'static str,
    authority: &'static str,
    operation: &'static str,
    request_binding_hash: &'a str,
    provider_replayed: bool,
    terminal_receipt_retried: bool,
    product_journal_finalized: bool,
    inspection: OperatorMutationJournalInspection,
    runtime_receipt: RuntimeExecutionReceipt,
}

pub(crate) struct OperatorMutationReconciliationHttpResponse {
    pub(crate) status: &'static str,
    pub(crate) body: String,
    pub(crate) journal_state_changed: bool,
}

struct PreparedReconciliation {
    request: ReconciliationRequest,
    decision: Option<ReconciliationDecision>,
    inspection: OperatorMutationJournalInspection,
    runtime_receipt: RuntimeExecutionReceipt,
}

pub(crate) fn prevalidate_resolve_http(
    kernel: &RuntimeKernel,
    body: Option<&str>,
    request_binding_hash: &str,
    expected_session_binding_hash: &str,
) -> Option<OperatorMutationReconciliationHttpResponse> {
    let authority = match load_authority() {
        Ok(authority) => authority,
        Err(_) => {
            return Some(error(
                "503 Service Unavailable",
                "operator_mutation_reconciliation.authority_unavailable",
            ));
        }
    };
    prepare_request(
        kernel,
        "POST",
        OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT,
        body,
        request_binding_hash,
        expected_session_binding_hash,
        authority.key.as_ref(),
        &authority.journal,
    )
    .err()
}

pub(crate) fn route_http(
    kernel: &RuntimeKernel,
    method: &str,
    path: &str,
    body: Option<&str>,
    request_binding_hash: &str,
    expected_session_binding_hash: &str,
) -> Option<OperatorMutationReconciliationHttpResponse> {
    if !matches!(
        path,
        OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT
            | OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT
    ) {
        return None;
    }
    let authority = match load_authority() {
        Ok(authority) => authority,
        Err(_) => {
            return Some(error(
                "503 Service Unavailable",
                "operator_mutation_reconciliation.authority_unavailable",
            ));
        }
    };
    Some(route_http_with_authority(
        kernel,
        method,
        path,
        body,
        request_binding_hash,
        expected_session_binding_hash,
        authority.key.as_ref(),
        &authority.journal,
    ))
}

#[allow(clippy::too_many_arguments)]
fn route_http_with_authority(
    kernel: &RuntimeKernel,
    method: &str,
    path: &str,
    body: Option<&str>,
    request_binding_hash: &str,
    expected_session_binding_hash: &str,
    key: &[u8],
    journal: &OperatorMutationJournal,
) -> OperatorMutationReconciliationHttpResponse {
    let PreparedReconciliation {
        request,
        decision,
        inspection,
        runtime_receipt,
    } = match prepare_request(
        kernel,
        method,
        path,
        body,
        request_binding_hash,
        expected_session_binding_hash,
        key,
        journal,
    ) {
        Ok(prepared) => prepared,
        Err(response) => return response,
    };
    if decision.is_none() {
        return json(
            "200 OK",
            &ReconciliationResponse {
                schema: "hepta.native.operator-note-reconciliation.v1",
                authority: "exact_plan_attempt_effect_hmac_and_runtime_terminal_receipt",
                operation: "inspect_only",
                request_binding_hash,
                provider_replayed: false,
                terminal_receipt_retried: false,
                product_journal_finalized: inspection.phase == "succeeded",
                inspection,
                runtime_receipt,
            },
            false,
        );
    }
    if journal
        .finalize_linked_success(key, &request.plan_hash, &runtime_receipt)
        .is_err()
    {
        return error(
            "409 Conflict",
            "operator_mutation_reconciliation.journal_not_reconcilable",
        );
    }
    let inspection = match journal.inspect(key, &request.plan_hash) {
        Ok(inspection) => inspection,
        Err(_) => {
            return error(
                "503 Service Unavailable",
                "operator_mutation_reconciliation.journal_readback_failed",
            );
        }
    };
    json(
        "200 OK",
        &ReconciliationResponse {
            schema: "hepta.native.operator-note-reconciliation.v1",
            authority: "exact_plan_attempt_effect_hmac_and_runtime_terminal_receipt",
            operation: "finalize_product_journal_only",
            request_binding_hash,
            provider_replayed: false,
            terminal_receipt_retried: false,
            product_journal_finalized: true,
            inspection,
            runtime_receipt,
        },
        true,
    )
}

#[allow(clippy::too_many_arguments)]
fn prepare_request(
    kernel: &RuntimeKernel,
    method: &str,
    path: &str,
    body: Option<&str>,
    request_binding_hash: &str,
    expected_session_binding_hash: &str,
    key: &[u8],
    journal: &OperatorMutationJournal,
) -> std::result::Result<PreparedReconciliation, OperatorMutationReconciliationHttpResponse> {
    if method != "POST" {
        return Err(error(
            "405 Method Not Allowed",
            "operator_mutation_reconciliation.method_not_allowed",
        ));
    }
    let body = body.ok_or_else(|| {
        error(
            "400 Bad Request",
            "operator_mutation_reconciliation.body_required",
        )
    })?;
    if body.len() > MAX_REQUEST_BYTES {
        return Err(error(
            "413 Payload Too Large",
            "operator_mutation_reconciliation.body_too_large",
        ));
    }
    let request: ReconciliationRequest = serde_json::from_str(body).map_err(|_| {
        error(
            "400 Bad Request",
            "operator_mutation_reconciliation.body_invalid",
        )
    })?;
    let decision = match path {
        OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT if request.decision.is_none() => None,
        OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT
            if request.decision == Some(ReconciliationDecision::FinalizeProductJournalOnly) =>
        {
            request.decision
        }
        _ => {
            return Err(error(
                "422 Unprocessable Entity",
                "operator_mutation_reconciliation.decision_invalid",
            ));
        }
    };
    if !canonical_hex(&request.plan_hash)
        || !canonical_binding(&request.attempt_id)
        || !canonical_content_hash(&request.effect_plan_hash)
        || request.session_binding_hash != expected_session_binding_hash
        || !canonical_hex(request_binding_hash)
    {
        return Err(error(
            "403 Forbidden",
            "operator_mutation_reconciliation.binding_invalid",
        ));
    }
    let expected_proof = proof(
        key,
        method,
        path,
        &request.session_binding_hash,
        &request.plan_hash,
        &request.attempt_id,
        &request.effect_plan_hash,
        request_binding_hash,
        decision,
    )
    .map_err(|_| {
        error(
            "503 Service Unavailable",
            "operator_mutation_reconciliation.authority_unavailable",
        )
    })?;
    if !constant_time_hex_equal(&expected_proof, &request.proof) {
        return Err(error(
            "403 Forbidden",
            "operator_mutation_reconciliation.authentication_denied",
        ));
    }
    let inspection = journal.inspect(key, &request.plan_hash).map_err(|_| {
        error(
            "404 Not Found",
            "operator_mutation_reconciliation.plan_not_found",
        )
    })?;
    if inspection.session_binding_hash != request.session_binding_hash
        || inspection.attempt_id.as_deref() != Some(request.attempt_id.as_str())
        || inspection.effect_plan_hash.as_deref() != Some(request.effect_plan_hash.as_str())
    {
        return Err(error(
            "409 Conflict",
            "operator_mutation_reconciliation.runtime_linkage_mismatch",
        ));
    }
    let runtime_receipt = kernel
        .execution_receipt_by_attempt(&request.attempt_id)
        .map_err(|_| {
            error(
                "503 Service Unavailable",
                "operator_mutation_reconciliation.runtime_receipt_unavailable",
            )
        })?
        .ok_or_else(|| {
            error(
                "409 Conflict",
                "operator_mutation_reconciliation.runtime_receipt_missing",
            )
        })?;
    if !inspection_matches_runtime_receipt(&inspection, &runtime_receipt)
        || runtime_receipt.effect_plan_hash.as_deref() != Some(request.effect_plan_hash.as_str())
        || runtime_receipt.terminal_status != "succeeded"
    {
        return Err(error(
            "409 Conflict",
            "operator_mutation_reconciliation.runtime_receipt_mismatch",
        ));
    }
    if decision.is_some() && inspection.phase != "in_doubt" {
        return Err(error(
            "409 Conflict",
            "operator_mutation_reconciliation.journal_not_in_doubt",
        ));
    }
    Ok(PreparedReconciliation {
        request,
        decision,
        inspection,
        runtime_receipt,
    })
}

#[allow(clippy::too_many_arguments)]
fn proof(
    key: &[u8],
    method: &str,
    path: &str,
    session_binding_hash: &str,
    plan_hash: &str,
    attempt_id: &str,
    effect_plan_hash: &str,
    request_binding_hash: &str,
    decision: Option<ReconciliationDecision>,
) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(key)
        .context("initialize operator mutation reconciliation HMAC")?;
    update_frame(
        &mut mac,
        OPERATOR_MUTATION_RECONCILIATION_PROOF_DOMAIN.as_bytes(),
    );
    for value in [
        method,
        path,
        session_binding_hash,
        plan_hash,
        attempt_id,
        effect_plan_hash,
        request_binding_hash,
        match decision {
            Some(ReconciliationDecision::FinalizeProductJournalOnly) => {
                "finalize_product_journal_only"
            }
            None => "inspect_only",
        },
    ] {
        update_frame(&mut mac, value.as_bytes());
    }
    Ok(hex_encode(&mac.finalize().into_bytes()))
}

fn inspection_matches_runtime_receipt(
    inspection: &OperatorMutationJournalInspection,
    receipt: &RuntimeExecutionReceipt,
) -> bool {
    inspection.attempt_id.as_deref() == Some(receipt.attempt_id.as_str())
        && inspection.effect_plan_hash == receipt.effect_plan_hash
        && inspection.provider_effect_ack_hash == receipt.provider_effect_ack_hash
        && inspection.terminal_receipt_id.as_deref() == Some(receipt.terminal_receipt_id.as_str())
        && inspection.terminal_receipt_hash.as_deref()
            == Some(receipt.terminal_receipt_hash.as_str())
        && inspection.terminal_outcome_hash.as_deref()
            == Some(receipt.terminal_outcome_hash.as_str())
        && inspection.terminal_evidence_hash.as_deref()
            == Some(receipt.terminal_evidence_hash.as_str())
        && inspection.terminal_status.as_deref() == Some(receipt.terminal_status.as_str())
}

fn canonical_binding(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':' | b'.'))
}

fn canonical_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn canonical_content_hash(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(canonical_hex)
}

fn update_frame(mac: &mut HmacSha256, value: &[u8]) {
    mac.update(&(value.len() as u64).to_be_bytes());
    mac.update(value);
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

fn constant_time_hex_equal(left: &str, right: &str) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.bytes()
        .zip(right.bytes())
        .fold(0_u8, |difference, (left, right)| {
            difference | (left ^ right)
        })
        == 0
}

fn json<T: Serialize>(
    status: &'static str,
    value: &T,
    journal_state_changed: bool,
) -> OperatorMutationReconciliationHttpResponse {
    match serde_json::to_string(value) {
        Ok(body) => OperatorMutationReconciliationHttpResponse {
            status,
            body,
            journal_state_changed,
        },
        Err(_) => error(
            "503 Service Unavailable",
            "operator_mutation_reconciliation.response_encoding_failed",
        ),
    }
}

fn error(status: &'static str, code: &'static str) -> OperatorMutationReconciliationHttpResponse {
    OperatorMutationReconciliationHttpResponse {
        status,
        body: serde_json::json!({"error": code}).to_string(),
        journal_state_changed: false,
    }
}

#[cfg(all(test, unix))]
#[path = "../tests/unit/operator_mutation_reconciliation.rs"]
mod tests;
