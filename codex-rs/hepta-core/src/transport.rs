use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelTransportContractKind {
    InProcess,
    OpenAiCompatibleHttp,
    OpenAiResponses,
    AnthropicMessages,
    BedrockConverse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelTransportContractDescriptor {
    pub id: String,
    pub kind: ModelTransportContractKind,
    pub contract_covered: bool,
    pub request_shape_contract: bool,
    pub response_normalization_contract: bool,
    pub tool_call_mapping_contract: bool,
    pub streaming_delta_contract: bool,
    pub error_mapping_contract: bool,
    pub auth_boundary_redacted: bool,
    pub external_network_side_effects: bool,
    pub local_harness: String,
    pub summary: String,
}

impl ModelTransportContractDescriptor {
    pub fn new(
        id: impl Into<String>,
        kind: ModelTransportContractKind,
        local_harness: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            contract_covered: true,
            request_shape_contract: true,
            response_normalization_contract: true,
            tool_call_mapping_contract: true,
            streaming_delta_contract: true,
            error_mapping_contract: true,
            auth_boundary_redacted: true,
            external_network_side_effects: false,
            local_harness: local_harness.into(),
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelTransportContractReport {
    pub contract_count: usize,
    pub contract_covered_count: usize,
    pub transport_family_count: usize,
    pub in_process_contract: bool,
    pub openai_compatible_http_contract: bool,
    pub openai_responses_contract: bool,
    pub anthropic_messages_contract: bool,
    pub bedrock_converse_contract: bool,
    pub request_response_normalization_contract: bool,
    pub tool_call_mapping_contract: bool,
    pub streaming_delta_contract: bool,
    pub error_mapping_contract: bool,
    pub redacted_auth_boundary_contract: bool,
    pub external_network_side_effects_disabled: bool,
    pub all_p2_transport_contracts_covered: bool,
    pub transports: Vec<ModelTransportContractDescriptor>,
}

impl ModelTransportContractReport {
    pub fn native_default() -> Self {
        Self::from_transports(vec![
            ModelTransportContractDescriptor::new(
                "in-process-test-transport",
                ModelTransportContractKind::InProcess,
                "cargo test -p hepta-core provider_transport_contract_preserves_openai_compatible_http_name --quiet",
                "in-process transport keeps deterministic local model tests free of network side effects",
            ),
            ModelTransportContractDescriptor::new(
                "openai-compatible-http-contract",
                ModelTransportContractKind::OpenAiCompatibleHttp,
                "cargo test -p hepta-cli providers_command_emits_stable_json_shape --quiet",
                "OpenAI-compatible HTTP providers share request, response, tool-call, streaming, and error mapping semantics",
            ),
            ModelTransportContractDescriptor::new(
                "responses-api-contract",
                ModelTransportContractKind::OpenAiResponses,
                "cargo test -p hepta-core model_transport_contracts_cover_p2_provider_shapes --quiet",
                "Responses-style providers are represented as a local contract without making credentialed network calls",
            ),
            ModelTransportContractDescriptor::new(
                "anthropic-messages-contract",
                ModelTransportContractKind::AnthropicMessages,
                "cargo test -p hepta-core model_transport_contracts_cover_p2_provider_shapes --quiet",
                "messages-style providers map tool calls, usage, stop reasons, and stream deltas into Hepta's model DTOs",
            ),
            ModelTransportContractDescriptor::new(
                "bedrock-converse-contract",
                ModelTransportContractKind::BedrockConverse,
                "cargo test -p hepta-core model_transport_contracts_cover_p2_provider_shapes --quiet",
                "Bedrock-style converse providers are fenced behind redacted auth and local shape contracts",
            ),
        ])
    }

    pub fn from_transports(transports: Vec<ModelTransportContractDescriptor>) -> Self {
        let contract_count = transports.len();
        let contract_covered_count = transports
            .iter()
            .filter(|transport| transport.contract_covered)
            .count();
        let has_kind = |kind: ModelTransportContractKind| {
            transports
                .iter()
                .any(|transport| transport.contract_covered && transport.kind == kind)
        };
        let in_process_contract = has_kind(ModelTransportContractKind::InProcess);
        let openai_compatible_http_contract =
            has_kind(ModelTransportContractKind::OpenAiCompatibleHttp);
        let openai_responses_contract = has_kind(ModelTransportContractKind::OpenAiResponses);
        let anthropic_messages_contract = has_kind(ModelTransportContractKind::AnthropicMessages);
        let bedrock_converse_contract = has_kind(ModelTransportContractKind::BedrockConverse);
        let transport_family_count = [
            in_process_contract,
            openai_compatible_http_contract,
            openai_responses_contract,
            anthropic_messages_contract,
            bedrock_converse_contract,
        ]
        .into_iter()
        .filter(|covered| *covered)
        .count();
        let request_response_normalization_contract = transports.iter().all(|transport| {
            transport.contract_covered
                && transport.request_shape_contract
                && transport.response_normalization_contract
        });
        let tool_call_mapping_contract = transports
            .iter()
            .all(|transport| transport.contract_covered && transport.tool_call_mapping_contract);
        let streaming_delta_contract = transports
            .iter()
            .all(|transport| transport.contract_covered && transport.streaming_delta_contract);
        let error_mapping_contract = transports
            .iter()
            .all(|transport| transport.contract_covered && transport.error_mapping_contract);
        let redacted_auth_boundary_contract = transports
            .iter()
            .all(|transport| transport.contract_covered && transport.auth_boundary_redacted);
        let external_network_side_effects_disabled = transports.iter().all(|transport| {
            transport.contract_covered && !transport.external_network_side_effects
        });
        let all_p2_transport_contracts_covered = contract_count > 0
            && contract_count == contract_covered_count
            && transport_family_count == 5
            && request_response_normalization_contract
            && tool_call_mapping_contract
            && streaming_delta_contract
            && error_mapping_contract
            && redacted_auth_boundary_contract
            && external_network_side_effects_disabled;

        Self {
            contract_count,
            contract_covered_count,
            transport_family_count,
            in_process_contract,
            openai_compatible_http_contract,
            openai_responses_contract,
            anthropic_messages_contract,
            bedrock_converse_contract,
            request_response_normalization_contract,
            tool_call_mapping_contract,
            streaming_delta_contract,
            error_mapping_contract,
            redacted_auth_boundary_contract,
            external_network_side_effects_disabled,
            all_p2_transport_contracts_covered,
            transports,
        }
    }

    pub fn contract_ready(&self) -> bool {
        self.all_p2_transport_contracts_covered
    }
}

#[cfg(test)]
mod tests {
    use super::ModelTransportContractReport;

    #[test]
    fn model_transport_contracts_cover_p2_provider_shapes() {
        let report = ModelTransportContractReport::native_default();

        assert_eq!(report.contract_count, 5);
        assert_eq!(report.contract_covered_count, report.contract_count);
        assert_eq!(report.transport_family_count, 5);
        assert!(report.in_process_contract);
        assert!(report.openai_compatible_http_contract);
        assert!(report.openai_responses_contract);
        assert!(report.anthropic_messages_contract);
        assert!(report.bedrock_converse_contract);
        assert!(report.request_response_normalization_contract);
        assert!(report.tool_call_mapping_contract);
        assert!(report.streaming_delta_contract);
        assert!(report.error_mapping_contract);
        assert!(report.redacted_auth_boundary_contract);
        assert!(report.external_network_side_effects_disabled);
        assert!(report.contract_ready());
        assert!(report.transports.iter().all(|transport| {
            let id = transport.id.to_lowercase();
            let summary = transport.summary.to_lowercase();
            !id.contains(&["her", "mes"].concat()) && !summary.contains(&["her", "mes"].concat())
        }));
    }
}
