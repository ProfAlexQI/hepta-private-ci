//! Exact operator reconciliation for already-attempted local provider effects.
//!
//! This surface can retry only durable terminal-receipt persistence. It never
//! invokes a provider, reconstructs an effect, accepts a path, or mints an ACK.

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use hepta_runtime::ExecutionEffectInspectionState;
use hepta_runtime::PendingExecutionEffectInspection;
use hepta_runtime::RuntimeKernel;
use hmac::Hmac;
use hmac::Mac;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::secure_key_file::read_private_key;

pub(crate) const EFFECT_RECONCILIATION_INSPECT_ENDPOINT: &str =
    "/api/v2/runtime/effects/reconciliation/inspect";
pub(crate) const EFFECT_RECONCILIATION_RESOLVE_ENDPOINT: &str =
    "/api/v2/runtime/effects/reconciliation/resolve";
pub(crate) const EFFECT_RECONCILIATION_KEY_FILE_ENV: &str =
    "HEPTA_EFFECT_RECONCILIATION_AUTH_KEY_FILE";
const RECONCILIATION_SCHEMA: &str = "hepta.operator-effect-reconciliation.v1";
const RECONCILIATION_MAC_DOMAIN: &[u8] = b"hepta.operator-effect-reconciliation.hmac-sha256.v1";
const MAX_REQUEST_BYTES: usize = 4096;
type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub(crate) struct EffectReconciliationConfig {
    key_file: PathBuf,
}

impl EffectReconciliationConfig {
    pub(crate) fn from_env() -> Result<Option<Self>> {
        Self::from_lookup(|name| env::var_os(name))
    }

    fn from_lookup(mut lookup: impl FnMut(&str) -> Option<OsString>) -> Result<Option<Self>> {
        let Some(value) =
            lookup(EFFECT_RECONCILIATION_KEY_FILE_ENV).filter(|value| !value.is_empty())
        else {
            return Ok(None);
        };
        let key_file = PathBuf::from(value);
        if !key_file.is_absolute() {
            anyhow::bail!("{EFFECT_RECONCILIATION_KEY_FILE_ENV} must be an absolute path");
        }
        Ok(Some(Self { key_file }))
    }

    #[cfg(all(test, unix))]
    pub(crate) fn for_test(root: &std::path::Path) -> Result<Self> {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let key_file = root.join("effect-reconciliation.key");
        fs::write(
            &key_file,
            b"808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f",
        )?;
        fs::set_permissions(&key_file, fs::Permissions::from_mode(0o600))?;
        Ok(Self { key_file })
    }
}

pub(crate) struct EffectReconciliationAuthority {
    key: Zeroizing<[u8; 32]>,
}

impl std::fmt::Debug for EffectReconciliationAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EffectReconciliationAuthority")
            .field("key", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReconciliationRequest {
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
    RetryTerminalReceiptOnly,
}

#[derive(Debug, Serialize)]
struct ReconciliationResponse<'a> {
    schema: &'static str,
    authority: &'static str,
    operation: &'static str,
    request_binding_hash: &'a str,
    provider_replayed: bool,
    arbitrary_target_accepted: bool,
    terminal_retry_attempted: bool,
    terminal_retry_result: Option<&'static str>,
    inspection: PendingExecutionEffectInspection,
}

pub(crate) struct EffectReconciliationHttpResponse {
    pub(crate) status: &'static str,
    pub(crate) body: String,
    pub(crate) outcome_state_changed: bool,
}

struct PreparedReconciliation {
    request: ReconciliationRequest,
    decision: Option<ReconciliationDecision>,
    inspection: PendingExecutionEffectInspection,
}

impl EffectReconciliationAuthority {
    pub(crate) fn open(config: EffectReconciliationConfig) -> Result<Self> {
        let key = read_private_key(
            &config.key_file,
            EFFECT_RECONCILIATION_KEY_FILE_ENV,
            "effect reconciliation authentication",
        )?;
        Ok(Self { key })
    }

    pub(crate) fn prevalidate_resolve_http(
        &self,
        kernel: &RuntimeKernel,
        body: Option<&str>,
        request_binding_hash: &str,
        expected_session_binding_hash: &str,
    ) -> Option<EffectReconciliationHttpResponse> {
        self.prepare_request(
            kernel,
            "POST",
            EFFECT_RECONCILIATION_RESOLVE_ENDPOINT,
            body,
            request_binding_hash,
            expected_session_binding_hash,
        )
        .err()
    }

    pub(crate) fn route_http(
        &self,
        kernel: &RuntimeKernel,
        method: &str,
        path: &str,
        body: Option<&str>,
        request_binding_hash: &str,
        expected_session_binding_hash: &str,
    ) -> Option<EffectReconciliationHttpResponse> {
        if !matches!(
            path,
            EFFECT_RECONCILIATION_INSPECT_ENDPOINT | EFFECT_RECONCILIATION_RESOLVE_ENDPOINT
        ) {
            return None;
        }
        let PreparedReconciliation {
            request,
            decision,
            inspection,
        } = match self.prepare_request(
            kernel,
            method,
            path,
            body,
            request_binding_hash,
            expected_session_binding_hash,
        ) {
            Ok(prepared) => prepared,
            Err(response) => return Some(response),
        };
        if decision.is_none() {
            return Some(json(
                "200 OK",
                &ReconciliationResponse {
                    schema: RECONCILIATION_SCHEMA,
                    authority: "exact_attempt_effect_plan_hmac",
                    operation: "inspect_only",
                    request_binding_hash,
                    provider_replayed: false,
                    arbitrary_target_accepted: false,
                    terminal_retry_attempted: false,
                    terminal_retry_result: None,
                    inspection,
                },
                false,
            ));
        }
        if inspection.state != ExecutionEffectInspectionState::AppliedAcknowledged {
            return Some(error(
                "409 Conflict",
                "operator_effect_reconciliation.effect_not_exactly_acknowledged",
            ));
        }
        let (result, outcome_state_changed) = match kernel
            .reconcile_pending_outcome(&request.attempt_id)
        {
            Ok(hepta_memory::OutcomeRecordResult::Recorded) => ("recorded", true),
            Ok(hepta_memory::OutcomeRecordResult::AlreadyRecorded) => ("already_recorded", false),
            Err(_) => {
                return Some(error(
                    "409 Conflict",
                    "operator_effect_reconciliation.terminal_material_still_pending",
                ));
            }
        };
        Some(json(
            "200 OK",
            &ReconciliationResponse {
                schema: RECONCILIATION_SCHEMA,
                authority: "exact_attempt_effect_plan_hmac",
                operation: "retry_terminal_receipt_only",
                request_binding_hash,
                provider_replayed: false,
                arbitrary_target_accepted: false,
                terminal_retry_attempted: true,
                terminal_retry_result: Some(result),
                inspection,
            },
            outcome_state_changed,
        ))
    }

    fn prepare_request(
        &self,
        kernel: &RuntimeKernel,
        method: &str,
        path: &str,
        body: Option<&str>,
        request_binding_hash: &str,
        expected_session_binding_hash: &str,
    ) -> std::result::Result<PreparedReconciliation, EffectReconciliationHttpResponse> {
        if method != "POST" {
            return Err(error(
                "405 Method Not Allowed",
                "operator_effect_reconciliation.method_not_allowed",
            ));
        }
        let body = body.ok_or_else(|| {
            error(
                "400 Bad Request",
                "operator_effect_reconciliation.body_required",
            )
        })?;
        if body.len() > MAX_REQUEST_BYTES {
            return Err(error(
                "413 Payload Too Large",
                "operator_effect_reconciliation.body_too_large",
            ));
        }
        let request: ReconciliationRequest = serde_json::from_str(body).map_err(|_| {
            error(
                "400 Bad Request",
                "operator_effect_reconciliation.body_invalid",
            )
        })?;
        let decision = match path {
            EFFECT_RECONCILIATION_INSPECT_ENDPOINT if request.decision.is_none() => None,
            EFFECT_RECONCILIATION_RESOLVE_ENDPOINT
                if request.decision == Some(ReconciliationDecision::RetryTerminalReceiptOnly) =>
            {
                request.decision
            }
            _ => {
                return Err(error(
                    "422 Unprocessable Entity",
                    "operator_effect_reconciliation.decision_invalid",
                ));
            }
        };
        if !canonical_binding(&request.attempt_id)
            || !canonical_hash(&request.effect_plan_hash)
            || request.session_binding_hash != expected_session_binding_hash
            || !canonical_hash(request_binding_hash)
        {
            return Err(error(
                "403 Forbidden",
                "operator_effect_reconciliation.binding_invalid",
            ));
        }
        let expected_proof = self
            .proof(
                method,
                path,
                &request.session_binding_hash,
                &request.attempt_id,
                &request.effect_plan_hash,
                request_binding_hash,
                decision,
            )
            .map_err(|_| {
                error(
                    "503 Service Unavailable",
                    "operator_effect_reconciliation.authority_unavailable",
                )
            })?;
        if !constant_time_hex_equal(&expected_proof, &request.proof) {
            return Err(error(
                "403 Forbidden",
                "operator_effect_reconciliation.authentication_denied",
            ));
        }
        let inspection = exact_inspection(kernel, &request.attempt_id, &request.effect_plan_hash)
            .map_err(|error_kind| match error_kind {
            ExactInspectionError::NotFound => error(
                "404 Not Found",
                "operator_effect_reconciliation.pending_attempt_not_found",
            ),
            ExactInspectionError::BindingMismatch => error(
                "409 Conflict",
                "operator_effect_reconciliation.effect_plan_mismatch",
            ),
            ExactInspectionError::Unavailable => error(
                "503 Service Unavailable",
                "operator_effect_reconciliation.inspection_unavailable",
            ),
        })?;
        if decision.is_some()
            && inspection.state != ExecutionEffectInspectionState::AppliedAcknowledged
        {
            return Err(error(
                "409 Conflict",
                "operator_effect_reconciliation.effect_not_exactly_acknowledged",
            ));
        }
        Ok(PreparedReconciliation {
            request,
            decision,
            inspection,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn proof(
        &self,
        method: &str,
        path: &str,
        session_binding_hash: &str,
        attempt_id: &str,
        effect_plan_hash: &str,
        request_binding_hash: &str,
        decision: Option<ReconciliationDecision>,
    ) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(self.key.as_ref())
            .context("initialize effect reconciliation HMAC")?;
        update_frame(&mut mac, RECONCILIATION_MAC_DOMAIN);
        for value in [
            method,
            path,
            session_binding_hash,
            attempt_id,
            effect_plan_hash,
            request_binding_hash,
            match decision {
                Some(ReconciliationDecision::RetryTerminalReceiptOnly) => {
                    "retry_terminal_receipt_only"
                }
                None => "inspect_only",
            },
        ] {
            update_frame(&mut mac, value.as_bytes());
        }
        Ok(hex_encode(&mac.finalize().into_bytes()))
    }

    #[cfg(test)]
    pub(crate) fn inspect_proof_for_test(
        &self,
        session_binding_hash: &str,
        attempt_id: &str,
        effect_plan_hash: &str,
        request_binding_hash: &str,
    ) -> Result<String> {
        self.proof(
            "POST",
            EFFECT_RECONCILIATION_INSPECT_ENDPOINT,
            session_binding_hash,
            attempt_id,
            effect_plan_hash,
            request_binding_hash,
            None,
        )
    }
}

enum ExactInspectionError {
    NotFound,
    BindingMismatch,
    Unavailable,
}

fn exact_inspection(
    kernel: &RuntimeKernel,
    attempt_id: &str,
    effect_plan_hash: &str,
) -> std::result::Result<PendingExecutionEffectInspection, ExactInspectionError> {
    let inspections = kernel
        .pending_execution_effect_inspections()
        .map_err(|_| ExactInspectionError::Unavailable)?;
    let inspection = inspections
        .into_iter()
        .find(|inspection| inspection.attempt_id == attempt_id)
        .ok_or(ExactInspectionError::NotFound)?;
    if inspection.effect_plan_hash.as_deref() != Some(effect_plan_hash) {
        return Err(ExactInspectionError::BindingMismatch);
    }
    Ok(inspection)
}

fn canonical_binding(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':' | b'.'))
}

fn canonical_hash(value: &str) -> bool {
    let encoded = value.strip_prefix("sha256:").unwrap_or(value);
    encoded.len() == 64
        && encoded
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
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
    outcome_state_changed: bool,
) -> EffectReconciliationHttpResponse {
    match serde_json::to_string(value) {
        Ok(body) => EffectReconciliationHttpResponse {
            status,
            body,
            outcome_state_changed,
        },
        Err(_) => error(
            "503 Service Unavailable",
            "operator_effect_reconciliation.response_encoding_failed",
        ),
    }
}

fn error(status: &'static str, code: &'static str) -> EffectReconciliationHttpResponse {
    EffectReconciliationHttpResponse {
        status,
        body: serde_json::json!({"error": code}).to_string(),
        outcome_state_changed: false,
    }
}

#[cfg(all(test, unix))]
#[path = "../tests/unit/effect_reconciliation.rs"]
mod tests;
