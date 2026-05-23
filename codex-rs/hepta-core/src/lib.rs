#![allow(async_fn_in_trait)]

pub mod agent_competition;
pub mod channels;
pub mod config;
pub mod control_ui;
pub mod doctor;
pub mod errors;
pub mod execution_safety_regressions;
pub mod external_agent_benchmark;
pub mod external_production;
pub mod hepta_contracts;
pub mod hepta_latest_absorption;
pub mod hepta_live_sequence;
pub mod hepta_runtime_surfaces;
pub mod intelligence;
pub mod local_config_import;
pub mod media_delivery;
pub mod memory;
pub mod memory_kernel;
pub mod model;
pub mod model_catalog_manifest;
pub mod native_capabilities;
pub mod operator_dashboard_polish;
pub mod operator_security;
pub mod plugin;
pub mod plugin_packaging;
pub mod plugin_provider_abi;
pub mod policy;
pub mod production_parity;
pub mod production_surface;
pub mod provider_breadth;
pub mod routines;
pub mod runtime_types;
pub mod scheduler;
pub mod self_improvement_review;
pub mod skill_curator;
pub mod skill_lifecycle;
pub mod skill_workshop;
pub mod tool_generation;
pub mod tools;
pub mod transport;
pub mod upstream_codex_sync;

pub use agent_competition::*;
pub use channels::*;
pub use config::*;
pub use control_ui::*;
pub use doctor::*;
pub use errors::*;
pub use execution_safety_regressions::*;
pub use external_agent_benchmark::*;
pub use external_production::*;
pub use hepta_contracts::*;
pub use hepta_latest_absorption::*;
pub use hepta_live_sequence::*;
pub use hepta_runtime_surfaces::*;
pub use intelligence::*;
pub use local_config_import::*;
pub use media_delivery::*;
pub use memory::*;
pub use memory_kernel::*;
pub use model::*;
pub use model_catalog_manifest::*;
pub use native_capabilities::*;
pub use operator_dashboard_polish::*;
pub use operator_security::*;
pub use plugin::*;
pub use plugin_packaging::*;
pub use plugin_provider_abi::*;
pub use policy::*;
pub use production_parity::*;
pub use production_surface::*;
pub use provider_breadth::*;
pub use routines::*;
pub use runtime_types::*;
pub use scheduler::*;
pub use self_improvement_review::*;
pub use skill_curator::*;
pub use skill_lifecycle::*;
pub use skill_workshop::*;
pub use tool_generation::*;
pub use tools::*;
pub use transport::*;
pub use upstream_codex_sync::*;

#[cfg(test)]
mod contract_tests {
    use serde_json::json;

    use crate::{
        AgentId, ApprovalRequirement, ChannelError, CorrelationId, Event, EventKind, MemoryError,
        ModelError, ModelRef, PathCapabilityGate, PolicyRule, ProviderTransportKind, RiskTier,
        SessionId, ToolError, ToolExecutionMetadata, Usage, WritePathScope,
    };

    #[test]
    fn runtime_event_contract_uses_snake_case_and_optional_payload() {
        let event = Event {
            kind: EventKind::WriteGroupRolledBack,
            session_id: Some(SessionId("session-1".into())),
            agent_id: Some(AgentId("builder".into())),
            correlation_id: Some(CorrelationId("corr-1".into())),
            summary: "rolled back overlapping writes".into(),
            payload: None,
        };

        let json = serde_json::to_value(&event).expect("event should serialize");

        assert_eq!(json["kind"], json!("write_group_rolled_back"));
        assert!(json.get("payload").is_none());

        let parsed: Event = serde_json::from_value(json!({
            "kind": "task_spawned",
            "session_id": "session-2",
            "agent_id": "reviewer",
            "correlation_id": "corr-2",
            "summary": "spawned follow-up task"
        }))
        .expect("event should deserialize without payload");

        assert_eq!(parsed.kind, EventKind::TaskSpawned);
        assert_eq!(parsed.payload, None);
    }

    #[test]
    fn provider_transport_contract_preserves_openai_compatible_http_name() {
        let json = serde_json::to_string(&ProviderTransportKind::OpenAiCompatibleHttp)
            .expect("transport should serialize");
        let parsed: ProviderTransportKind =
            serde_json::from_str(&json).expect("transport should deserialize");

        assert_eq!(json, "\"openai_compatible_http\"");
        assert_eq!(parsed, ProviderTransportKind::OpenAiCompatibleHttp);
    }

    #[test]
    fn policy_rule_roundtrips_risk_tier_and_optional_filters() {
        let rule = PolicyRule {
            id: "require-approval".into(),
            session_id: Some("session-1".into()),
            provider_name: Some("openai".into()),
            tool_name: Some("write".into()),
            risk_tier: Some(RiskTier::High),
            requirement: ApprovalRequirement::Ask,
            reason: "high-risk writes require review".into(),
        };

        let json = serde_json::to_value(&rule).expect("policy rule should serialize");
        let parsed: PolicyRule =
            serde_json::from_value(json.clone()).expect("policy rule should deserialize");

        assert_eq!(json["risk_tier"], json!("high"));
        assert_eq!(json["requirement"], json!("ask"));
        assert_eq!(parsed, rule);
    }

    #[test]
    fn metadata_and_error_contracts_remain_payload_light() {
        let metadata = ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        };
        let gate = PathCapabilityGate {
            id: "workspace-writes".into(),
            tool_name: "write".into(),
            argument_name: "path".into(),
            scope: crate::FilesystemScope::WorkspaceOnly,
        };

        let metadata_json = serde_json::to_string(&metadata).expect("metadata should serialize");
        let gate_json = serde_json::to_value(&gate).expect("gate should serialize");

        assert_eq!(
            serde_json::to_value(Usage::default()).expect("usage should serialize"),
            json!({
                "input_tokens": 0,
                "output_tokens": 0
            })
        );
        assert_eq!(
            metadata_json,
            "{\"read_only\":true,\"destructive\":false,\"idempotent\":true,\"produces_structured_output\":true}"
        );
        assert_eq!(gate_json["scope"], json!("workspace_only"));
        assert_eq!(
            serde_json::to_string(&WritePathScope::ArtifactsOnly).expect("scope should serialize"),
            "\"artifacts_only\""
        );

        for message in [
            HeptaLikeError::Model(ModelError("model failure".into())).to_string(),
            HeptaLikeError::Tool(ToolError("tool failure".into())).to_string(),
            HeptaLikeError::Memory(MemoryError("memory failure".into())).to_string(),
            HeptaLikeError::Channel(ChannelError("channel failure".into())).to_string(),
        ] {
            assert!(message.ends_with("failure"));
        }
    }

    #[derive(Debug)]
    enum HeptaLikeError {
        Model(ModelError),
        Tool(ToolError),
        Memory(MemoryError),
        Channel(ChannelError),
    }

    impl std::fmt::Display for HeptaLikeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Model(error) => write!(f, "{}", error),
                Self::Tool(error) => write!(f, "{}", error),
                Self::Memory(error) => write!(f, "{}", error),
                Self::Channel(error) => write!(f, "{}", error),
            }
        }
    }

    #[test]
    fn model_ref_roundtrips_stable_provider_and_model_ids() {
        let model = ModelRef {
            provider: "openai".into(),
            model: "gpt-5.4".into(),
        };

        let json = serde_json::to_string(&model).expect("model ref should serialize");
        let parsed: ModelRef = serde_json::from_str(&json).expect("model ref should deserialize");

        assert_eq!(parsed, model);
    }
}
