use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginProviderDescriptor {
    pub provider_id: String,
    pub display_name: String,
    pub abi_version: String,
    pub supports_ctx_llm: bool,
    pub supports_transform_llm_output: bool,
    pub credential_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLocalLlmContext {
    pub provider_id: String,
    pub model: String,
    pub session_id: String,
    pub request_id: String,
    pub prompt_cache_key: String,
    pub local_metadata: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLlmMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLlmRequestEnvelope {
    pub ctx_llm: ProviderLocalLlmContext,
    pub messages: Vec<ProviderLlmMessage>,
    pub max_output_tokens: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLlmOutputEnvelope {
    pub provider_id: String,
    pub model: String,
    pub text: String,
    pub transformed_by: Vec<String>,
}

pub trait PluginModelProviderAbi {
    fn descriptor(&self) -> PluginProviderDescriptor;
    fn build_ctx_llm(
        &self,
        model: &str,
        session_id: &str,
        request_id: &str,
    ) -> ProviderLocalLlmContext;
    fn transform_llm_output(
        &self,
        ctx: &ProviderLocalLlmContext,
        output: ProviderLlmOutputEnvelope,
    ) -> ProviderLlmOutputEnvelope;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginProviderAbiReport {
    pub status: String,
    pub abi_version: String,
    pub provider_id: String,
    pub descriptor_registered: bool,
    pub ctx_llm_schema_ready: bool,
    pub request_envelope_ready: bool,
    pub transform_hook_ready: bool,
    pub credential_values_read: bool,
    pub external_provider_invoked: bool,
    pub sample_run: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<PluginProviderAbiRuntimeSample>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginProviderAbiRuntimeSample {
    pub descriptor: PluginProviderDescriptor,
    pub request: ProviderLlmRequestEnvelope,
    pub transformed_output: ProviderLlmOutputEnvelope,
}

pub fn plugin_provider_abi_report(sample_run: bool) -> PluginProviderAbiReport {
    let provider = LocalEchoProviderAbi::new("local-echo-provider");
    let sample = sample_run.then(|| {
        let ctx = provider.build_ctx_llm("hepta-local-test", "session:operator", "req-ctx-llm-1");
        let request = ProviderLlmRequestEnvelope {
            ctx_llm: ctx.clone(),
            messages: vec![ProviderLlmMessage {
                role: "user".into(),
                content: "hello provider".into(),
            }],
            max_output_tokens: Some(128),
        };
        let output = ProviderLlmOutputEnvelope {
            provider_id: ctx.provider_id.clone(),
            model: ctx.model.clone(),
            text: "raw provider output".into(),
            transformed_by: Vec::new(),
        };
        let transformed_output = provider.transform_llm_output(&ctx, output);
        PluginProviderAbiRuntimeSample {
            descriptor: provider.descriptor(),
            request,
            transformed_output,
        }
    });

    PluginProviderAbiReport {
        status: "ready".into(),
        abi_version: "hepta-plugin-provider-abi-v0".into(),
        provider_id: provider.provider_id.clone(),
        descriptor_registered: true,
        ctx_llm_schema_ready: true,
        request_envelope_ready: true,
        transform_hook_ready: true,
        credential_values_read: false,
        external_provider_invoked: false,
        sample_run,
        sample,
    }
}

#[derive(Debug, Clone)]
struct LocalEchoProviderAbi {
    provider_id: String,
}

impl LocalEchoProviderAbi {
    fn new(provider_id: &str) -> Self {
        Self {
            provider_id: provider_id.into(),
        }
    }
}

impl PluginModelProviderAbi for LocalEchoProviderAbi {
    fn descriptor(&self) -> PluginProviderDescriptor {
        PluginProviderDescriptor {
            provider_id: self.provider_id.clone(),
            display_name: "Local Echo Provider ABI Sample".into(),
            abi_version: "hepta-plugin-provider-abi-v0".into(),
            supports_ctx_llm: true,
            supports_transform_llm_output: true,
            credential_policy: "no credential values in ctx.llm; readiness probes are redacted"
                .into(),
        }
    }

    fn build_ctx_llm(
        &self,
        model: &str,
        session_id: &str,
        request_id: &str,
    ) -> ProviderLocalLlmContext {
        ProviderLocalLlmContext {
            provider_id: self.provider_id.clone(),
            model: model.into(),
            session_id: session_id.into(),
            request_id: request_id.into(),
            prompt_cache_key: format!("{}:{}:{}", self.provider_id, model, session_id),
            local_metadata: json!({
                "transport": "local_deterministic_sample",
                "ctx_llm": true,
                "external_call": false
            }),
        }
    }

    fn transform_llm_output(
        &self,
        ctx: &ProviderLocalLlmContext,
        mut output: ProviderLlmOutputEnvelope,
    ) -> ProviderLlmOutputEnvelope {
        output.provider_id = ctx.provider_id.clone();
        output.model = ctx.model.clone();
        output.text = format!("[{}] {}", ctx.request_id, output.text.trim());
        output.transformed_by.push("transform_llm_output".into());
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_abi_builds_ctx_llm_and_transform_hook_without_external_call() {
        let report = plugin_provider_abi_report(true);
        assert_eq!(report.status, "ready");
        assert!(report.descriptor_registered);
        assert!(report.ctx_llm_schema_ready);
        assert!(report.request_envelope_ready);
        assert!(report.transform_hook_ready);
        assert!(!report.credential_values_read);
        assert!(!report.external_provider_invoked);
        let sample = report.sample.expect("sample run should include ABI sample");
        assert!(sample.descriptor.supports_ctx_llm);
        assert_eq!(sample.request.ctx_llm.provider_id, "local-echo-provider");
        assert!(
            sample
                .transformed_output
                .transformed_by
                .contains(&"transform_llm_output".to_string())
        );
        assert!(sample.transformed_output.text.contains("req-ctx-llm-1"));
    }
}
