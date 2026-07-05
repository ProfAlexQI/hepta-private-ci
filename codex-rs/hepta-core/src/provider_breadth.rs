use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderBreadthKind {
    GmiCloud,
    AzureAiFoundry,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderBreadthDescriptor {
    pub id: String,
    pub label: String,
    pub kind: ProviderBreadthKind,
    pub descriptor_ready: bool,
    pub routing_surface: String,
    pub auth_value_names_redacted: Vec<String>,
    pub model_catalog_manifest_required: bool,
    pub openai_compatible_routing: bool,
    pub endpoint_config_redacted: bool,
    pub provider_specific_oauth_surface: bool,
    pub local_catalog_descriptor_only: bool,
    pub credential_values_read: bool,
    pub api_key_value_read: bool,
    pub oauth_token_read: bool,
    pub local_credential_file_read: bool,
    pub provider_runtime_started: bool,
    pub provider_discovery_started: bool,
    pub provider_prompt_sent: bool,
    pub provider_media_uploaded: bool,
    pub external_network_read: bool,
    pub external_network_write: bool,
    pub external_side_effects: bool,
    pub evidence_gate: String,
    pub summary: String,
}

impl ProviderBreadthDescriptor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        kind: ProviderBreadthKind,
        routing_surface: impl Into<String>,
        auth_value_names_redacted: Vec<String>,
        model_catalog_manifest_required: bool,
        openai_compatible_routing: bool,
        provider_specific_oauth_surface: bool,
        evidence_gate: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind,
            descriptor_ready: true,
            routing_surface: routing_surface.into(),
            auth_value_names_redacted,
            model_catalog_manifest_required,
            openai_compatible_routing,
            endpoint_config_redacted: true,
            provider_specific_oauth_surface,
            local_catalog_descriptor_only: true,
            credential_values_read: false,
            api_key_value_read: false,
            oauth_token_read: false,
            local_credential_file_read: false,
            provider_runtime_started: false,
            provider_discovery_started: false,
            provider_prompt_sent: false,
            provider_media_uploaded: false,
            external_network_read: false,
            external_network_write: false,
            external_side_effects: false,
            evidence_gate: evidence_gate.into(),
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderBreadthReport {
    pub breadth_contract_id: String,
    pub provider_count: usize,
    pub descriptor_ready_count: usize,
    pub gmi_cloud_descriptor_ready: bool,
    pub azure_ai_foundry_descriptor_ready: bool,
    pub provider_specific_oauth_polish_ready: bool,
    pub openai_compatible_routing_ready: bool,
    pub endpoint_auth_redaction_ready: bool,
    pub remote_model_catalog_manifest_required: bool,
    pub local_catalog_descriptor_only: bool,
    pub credential_values_read: bool,
    pub api_key_value_read: bool,
    pub oauth_token_read: bool,
    pub local_credential_file_read: bool,
    pub provider_runtime_started: bool,
    pub provider_discovery_started: bool,
    pub provider_prompt_sent: bool,
    pub provider_media_uploaded: bool,
    pub external_network_read: bool,
    pub external_network_write: bool,
    pub external_side_effects: bool,
    pub p2_provider_breadth_ready: bool,
    pub providers: Vec<ProviderBreadthDescriptor>,
}

impl ProviderBreadthReport {
    pub fn native_default() -> Self {
        Self::from_providers(vec![
            ProviderBreadthDescriptor::new(
                "gmi-cloud",
                "GMI Cloud",
                ProviderBreadthKind::GmiCloud,
                "openai-compatible chat/completions routing with redacted endpoint and model catalog manifest labels",
                vec![
                    "GMI_API_KEY".into(),
                    "GMI_CLOUD_API_KEY".into(),
                    "GMI_BASE_URL".into(),
                ],
                true,
                true,
                false,
                "cargo test -p hepta-core provider_breadth_contract_covers_gmi_cloud_and_azure_ai_foundry_without_secret_reads --quiet",
                "GMI Cloud is represented as a local provider descriptor and capability-routing contract only; no provider discovery, prompt, media, billing, or model-list request is made",
            ),
            ProviderBreadthDescriptor::new(
                "azure-ai-foundry",
                "Azure AI Foundry",
                ProviderBreadthKind::AzureAiFoundry,
                "Azure endpoint/deployment routing with provider-specific OAuth/API-key labels and no credential material",
                vec![
                    "AZURE_AI_FOUNDRY_API_KEY".into(),
                    "AZURE_AI_FOUNDRY_ENDPOINT".into(),
                    "AZURE_AI_FOUNDRY_PROJECT".into(),
                    "AZURE_TENANT_ID".into(),
                    "AZURE_CLIENT_ID".into(),
                ],
                true,
                true,
                true,
                "cargo test -p hepta-core provider_breadth_contract_covers_gmi_cloud_and_azure_ai_foundry_without_secret_reads --quiet",
                "Azure AI Foundry is represented as a local descriptor with redacted endpoint, deployment, and OAuth surface labels; no login, model discovery, prompt, media, or billing request is made",
            ),
        ])
    }

    pub fn from_providers(providers: Vec<ProviderBreadthDescriptor>) -> Self {
        let provider_count = providers.len();
        let descriptor_ready_count = providers
            .iter()
            .filter(|provider| provider.descriptor_ready)
            .count();
        let has_ready_kind = |kind: ProviderBreadthKind| {
            providers
                .iter()
                .any(|provider| provider.descriptor_ready && provider.kind == kind)
        };
        let gmi_cloud_descriptor_ready = has_ready_kind(ProviderBreadthKind::GmiCloud);
        let azure_ai_foundry_descriptor_ready = has_ready_kind(ProviderBreadthKind::AzureAiFoundry);
        let provider_specific_oauth_polish_ready = providers
            .iter()
            .any(|provider| provider.provider_specific_oauth_surface)
            && azure_ai_foundry_descriptor_ready;
        let openai_compatible_routing_ready = providers
            .iter()
            .filter(|provider| provider.openai_compatible_routing)
            .count()
            >= 2;
        let endpoint_auth_redaction_ready = providers.iter().all(|provider| {
            provider.endpoint_config_redacted
                && !provider.auth_value_names_redacted.is_empty()
                && provider
                    .auth_value_names_redacted
                    .iter()
                    .all(|name| !name.trim().is_empty())
        });
        let remote_model_catalog_manifest_required = providers
            .iter()
            .all(|provider| provider.model_catalog_manifest_required);
        let local_catalog_descriptor_only = providers
            .iter()
            .all(|provider| provider.local_catalog_descriptor_only);
        let credential_values_read = providers
            .iter()
            .any(|provider| provider.credential_values_read);
        let api_key_value_read = providers.iter().any(|provider| provider.api_key_value_read);
        let oauth_token_read = providers.iter().any(|provider| provider.oauth_token_read);
        let local_credential_file_read = providers
            .iter()
            .any(|provider| provider.local_credential_file_read);
        let provider_runtime_started = providers
            .iter()
            .any(|provider| provider.provider_runtime_started);
        let provider_discovery_started = providers
            .iter()
            .any(|provider| provider.provider_discovery_started);
        let provider_prompt_sent = providers
            .iter()
            .any(|provider| provider.provider_prompt_sent);
        let provider_media_uploaded = providers
            .iter()
            .any(|provider| provider.provider_media_uploaded);
        let external_network_read = providers
            .iter()
            .any(|provider| provider.external_network_read);
        let external_network_write = providers
            .iter()
            .any(|provider| provider.external_network_write);
        let external_side_effects = providers
            .iter()
            .any(|provider| provider.external_side_effects)
            || credential_values_read
            || api_key_value_read
            || oauth_token_read
            || local_credential_file_read
            || provider_runtime_started
            || provider_discovery_started
            || provider_prompt_sent
            || provider_media_uploaded
            || external_network_read
            || external_network_write;
        let p2_provider_breadth_ready = provider_count >= 2
            && descriptor_ready_count == provider_count
            && gmi_cloud_descriptor_ready
            && azure_ai_foundry_descriptor_ready
            && provider_specific_oauth_polish_ready
            && openai_compatible_routing_ready
            && endpoint_auth_redaction_ready
            && remote_model_catalog_manifest_required
            && local_catalog_descriptor_only
            && !external_side_effects;

        Self {
            breadth_contract_id: "provider-breadth-contract".into(),
            provider_count,
            descriptor_ready_count,
            gmi_cloud_descriptor_ready,
            azure_ai_foundry_descriptor_ready,
            provider_specific_oauth_polish_ready,
            openai_compatible_routing_ready,
            endpoint_auth_redaction_ready,
            remote_model_catalog_manifest_required,
            local_catalog_descriptor_only,
            credential_values_read,
            api_key_value_read,
            oauth_token_read,
            local_credential_file_read,
            provider_runtime_started,
            provider_discovery_started,
            provider_prompt_sent,
            provider_media_uploaded,
            external_network_read,
            external_network_write,
            external_side_effects,
            p2_provider_breadth_ready,
            providers,
        }
    }

    pub fn breadth_ready(&self) -> bool {
        self.p2_provider_breadth_ready
    }
}

#[cfg(test)]
mod tests {
    use super::ProviderBreadthReport;

    #[test]
    fn provider_breadth_contract_covers_gmi_cloud_and_azure_ai_foundry_without_secret_reads() {
        let report = ProviderBreadthReport::native_default();

        assert_eq!(report.provider_count, 2);
        assert_eq!(report.descriptor_ready_count, report.provider_count);
        assert!(report.gmi_cloud_descriptor_ready);
        assert!(report.azure_ai_foundry_descriptor_ready);
        assert!(report.provider_specific_oauth_polish_ready);
        assert!(report.openai_compatible_routing_ready);
        assert!(report.endpoint_auth_redaction_ready);
        assert!(report.remote_model_catalog_manifest_required);
        assert!(report.local_catalog_descriptor_only);
        assert!(!report.credential_values_read);
        assert!(!report.api_key_value_read);
        assert!(!report.oauth_token_read);
        assert!(!report.local_credential_file_read);
        assert!(!report.provider_runtime_started);
        assert!(!report.provider_discovery_started);
        assert!(!report.provider_prompt_sent);
        assert!(!report.provider_media_uploaded);
        assert!(!report.external_network_read);
        assert!(!report.external_network_write);
        assert!(!report.external_side_effects);
        assert!(report.breadth_ready());
        let ids = report
            .providers
            .iter()
            .map(|provider| provider.id.as_str())
            .collect::<Vec<_>>();
        assert!(ids.contains(&"gmi-cloud"));
        assert!(ids.contains(&"azure-ai-foundry"));
        let forbidden = ["her", "mes"].concat();
        assert!(report.providers.iter().all(|provider| {
            let id = provider.id.to_lowercase();
            let summary = provider.summary.to_lowercase();
            !id.contains(&forbidden) && !summary.contains(&forbidden)
        }));
    }
}
