use crate::native_telegram;
use crate::preference_ingress::PREFERENCE_CHALLENGE_ENDPOINT;
use crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT;
use crate::runtime_composition::NativeGatewayRuntime;
use crate::runtime_composition::RUNTIME_KERNEL_CANARY_ACTION_ENDPOINT;
use crate::runtime_composition::RuntimeRequestDisposition;
use crate::runtime_composition::RuntimeRequestPreflightReceipt;
use crate::runtime_mutation::RUNTIME_MUTATION_CANARY_ENDPOINT;

pub(crate) const TELEGRAM_RECEIVE_ONCE_ENDPOINT: &str = "/api/telegram-receive-once";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeIngressKind {
    MetadataRead,
    CredentialedNetworkRead,
    AuthenticatedPreferencePlan,
    AuthenticatedPreferenceCommit,
    RuntimeKernelCanary,
    MutationPlan,
}

pub(crate) struct RuntimeIngressResponse {
    pub(crate) status: &'static str,
    pub(crate) body: String,
}

pub(crate) fn telegram_receive_once_response(
    runtime: Option<&NativeGatewayRuntime>,
    requested: bool,
    limit: usize,
) -> RuntimeIngressResponse {
    let Some(runtime) = runtime else {
        return RuntimeIngressResponse {
            status: "503 Service Unavailable",
            body: r#"{"error":"telegram_runtime_admission.runtime_unavailable"}"#.to_string(),
        };
    };
    let authority = match runtime.authorize_telegram_receive() {
        Ok(authority) => authority,
        Err(error) => {
            return RuntimeIngressResponse {
                status: "503 Service Unavailable",
                body: serde_json::json!({
                    "error": error.to_string(),
                    "ingress": "credentialed_network_read",
                    "config_observed": false,
                    "token_observed": false,
                    "cursor_observed": false,
                    "external_network_read": false,
                })
                .to_string(),
            };
        }
    };
    let status = native_telegram::telegram_receive_once_status(requested, limit, &authority);
    RuntimeIngressResponse {
        status: "200 OK",
        body: serde_json::to_string(&status).unwrap_or_else(|error| {
            serde_json::json!({"error": format!("serialization failed: {error}")}).to_string()
        }),
    }
}

pub(crate) fn runtime_ingress_kind(method: &str, path: &str) -> RuntimeIngressKind {
    match (method, path) {
        ("GET", TELEGRAM_RECEIVE_ONCE_ENDPOINT) => RuntimeIngressKind::CredentialedNetworkRead,
        ("POST", PREFERENCE_CHALLENGE_ENDPOINT) => RuntimeIngressKind::AuthenticatedPreferencePlan,
        ("POST", PREFERENCE_COMMIT_ENDPOINT) => RuntimeIngressKind::AuthenticatedPreferenceCommit,
        ("POST", RUNTIME_KERNEL_CANARY_ACTION_ENDPOINT) => RuntimeIngressKind::RuntimeKernelCanary,
        ("POST", RUNTIME_MUTATION_CANARY_ENDPOINT) => RuntimeIngressKind::MutationPlan,
        ("POST", _) => RuntimeIngressKind::MutationPlan,
        _ => RuntimeIngressKind::MetadataRead,
    }
}

pub(crate) fn runtime_preflight_matches(
    method: &str,
    path: &str,
    preflight: &RuntimeRequestPreflightReceipt,
) -> bool {
    let disposition = if method == "GET" {
        RuntimeRequestDisposition::ReadOnlyDispatch
    } else {
        RuntimeRequestDisposition::PlanOnlyQuarantine
    };
    !preflight.request_binding_hash.is_empty()
        && preflight.disposition == disposition
        && preflight.ingress_kind == runtime_ingress_kind(method, path)
}
