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

pub(crate) fn execute(
    kernel: &RuntimeKernel,
    request_binding_hash: &str,
    idempotency_key: &str,
) -> Result<RuntimeMutationCanaryReceipt> {
    if !canonical_hex(request_binding_hash) || !canonical_hex(idempotency_key) {
        anyhow::bail!(
            "runtime mutation canary requires canonical request and idempotency bindings"
        );
    }
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
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use hepta_memory::DurableIntegrityKey;

    use super::*;

    struct TestArtifact(PathBuf);

    impl Drop for TestArtifact {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.0);
        }
    }

    #[test]
    fn mutation_body_requires_exact_confirmation_and_idempotency() {
        let key = "a".repeat(64);
        assert_eq!(
            body_admitted(Some(&format!(
                r#"{{"confirm":true,"idempotency_key":"{key}"}}"#
            ))),
            Some(key)
        );
        assert!(body_admitted(Some(r#"{"confirm":false,"idempotency_key":"bad"}"#)).is_none());
        assert!(body_admitted(Some(r#"{"confirm":true,"idempotency_key":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","extra":1}"#)).is_none());
    }

    #[test]
    fn mutation_canary_records_intent_effect_ack_and_terminal_receipt() {
        let outcome_root = tempfile::tempdir().expect("outcome root");
        let kernel = RuntimeKernel::bootstrap_with_durable_outcomes(
            outcome_root.path().join("outcomes.sqlite3"),
            DurableIntegrityKey::from_bytes([7; 32]),
        )
        .expect("durable runtime");
        let nonce = format!("{}:{:?}", std::process::id(), std::time::SystemTime::now());
        let key = format!("{:x}", Sha256::digest(nonce.as_bytes()));
        let request = format!("{:x}", Sha256::digest(format!("request:{key}").as_bytes()));
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let artifact =
            TestArtifact(workspace.join(format!("artifacts/.hepta-runtime-mutation-{key}.json")));
        let receipt = execute(&kernel, &request, &key).expect("mutation receipt");
        assert!(receipt.execution_receipt.durable_intent_recorded);
        assert!(receipt.execution_receipt.effect_plan_recorded);
        assert!(receipt.execution_receipt.provider_effect_ack_hash.is_some());
        assert_eq!(receipt.execution_receipt.terminal_status, "succeeded");
        assert_eq!(fs::read_to_string(&artifact.0).expect("artifact"), request);
    }
}
