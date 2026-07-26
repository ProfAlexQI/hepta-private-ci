use std::env;

use anyhow::Context;
use anyhow::Result;
use hepta_core::ApprovalRequirement;
use hepta_runtime::RuntimeExecutionReceipt;
use hepta_runtime::RuntimeKernel;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

pub(crate) const RUNTIME_MUTATION_CANARY_ENDPOINT: &str =
    "/api/actions/runtime-kernel-mutation-canary";
pub(crate) const RUNTIME_MUTATION_CANARY_ENV: &str = "HEPTA_RUNTIME_MUTATION_CANARY";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RuntimeMutationCanaryRequest {
    confirm: bool,
    idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct RuntimeMutationCanaryReceipt {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    action: &'static str,
    request_binding_hash: String,
    idempotency_key_hash: String,
    target_class: &'static str,
    invoked_tool: String,
    execution_receipt: RuntimeExecutionReceipt,
    authority: &'static str,
    external_network_requested: bool,
    arbitrary_path_accepted: bool,
    arbitrary_content_accepted: bool,
}

pub(crate) fn body_admitted(body: Option<&str>) -> Option<String> {
    let request: RuntimeMutationCanaryRequest = serde_json::from_str(body?).ok()?;
    (request.confirm && canonical_hex(&request.idempotency_key)).then_some(request.idempotency_key)
}

pub(crate) fn enabled() -> bool {
    env::var(RUNTIME_MUTATION_CANARY_ENV)
        .ok()
        .is_some_and(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
}

pub(crate) fn prevalidate(request_binding_hash: &str, idempotency_key: &str) -> Result<()> {
    if !canonical_hex(request_binding_hash) || !canonical_hex(idempotency_key) {
        anyhow::bail!(
            "runtime mutation canary requires canonical request and idempotency bindings"
        );
    }
    Ok(())
}

pub(crate) fn execute(
    kernel: &RuntimeKernel,
    request_binding_hash: &str,
    idempotency_key: &str,
) -> Result<RuntimeMutationCanaryReceipt> {
    prevalidate(request_binding_hash, idempotency_key)?;
    let session_id = format!(
        "native-gateway:runtime-mutation-canary:{}",
        &request_binding_hash[..16]
    );
    let relative_target = format!("artifacts/.hepta-runtime-mutation-{idempotency_key}.json");
    kernel
        .switch_model_in_session(&session_id, "demo/demo-chat")
        .map_err(|error| anyhow::anyhow!("select isolated mutation model: {error}"))?;
    kernel
        .add_policy_rule(
            Some(&session_id),
            Some("demo"),
            Some("write_file"),
            None,
            ApprovalRequirement::None,
            Some("request-bound native mutation canary"),
        )
        .map_err(|error| anyhow::anyhow!("install scoped mutation authority: {error}"))?;
    let executor = tokio::runtime::Builder::new_current_thread()
        .build()
        .context("build isolated mutation canary executor")?;
    let result = executor
        .block_on(kernel.run_demo_turn_in_session(
            &session_id,
            &format!("overwrite:{relative_target} => {request_binding_hash}"),
        ))
        .map_err(|error| anyhow::anyhow!("execute RuntimeKernel mutation canary: {error}"))?;
    let receipt = result
        .execution_receipt
        .context("RuntimeKernel mutation canary completed without an execution receipt")?;
    if result.invoked_tool.as_deref() != Some("write_file")
        || !receipt.durable_intent_recorded
        || !receipt.effect_plan_recorded
        || receipt.provider_effect_ack_hash.is_none()
        || receipt.terminal_status != "succeeded"
    {
        anyhow::bail!("RuntimeKernel mutation canary lifecycle evidence failed closed");
    }
    Ok(RuntimeMutationCanaryReceipt {
        product: "Hepta",
        runtime: "hepta",
        status: "succeeded",
        action: "runtime-kernel-mutation-canary",
        request_binding_hash: request_binding_hash.to_string(),
        idempotency_key_hash: format!("sha256:{:x}", Sha256::digest(idempotency_key.as_bytes())),
        target_class: "fixed_local_artifact",
        invoked_tool: "write_file".to_string(),
        execution_receipt: receipt,
        authority: "runtime_kernel_exact_candidate",
        external_network_requested: false,
        arbitrary_path_accepted: false,
        arbitrary_content_accepted: false,
    })
}

fn canonical_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[cfg(test)]
#[path = "../tests/unit/runtime_mutation.rs"]
mod tests;
