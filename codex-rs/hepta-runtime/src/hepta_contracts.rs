use hepta_core::AcpAgentBridgeMatrix;
use hepta_core::AcpCodexApprovalLifecyclePlane;
use hepta_core::ChannelDeliveryStreamingParityPlane;
use hepta_core::ChannelMessageContractMap;
use hepta_core::CliStatusAuthParityPlane;
use hepta_core::ConfigUpdateSecuritySecretsLifecycleDryRunMap;
use hepta_core::GatewayPluginStartupDiagnosticsPlane;
use hepta_core::GatewaySessionTaskLivenessPlane;
use hepta_core::Hepta2026_5_6HardeningRegressions;
use hepta_core::HeptaCliCompatibilityMap;
use hepta_core::HeptaContractPlaneSummary;
use hepta_core::NodeDeviceContractPlane;
use hepta_core::OperationalUtilityContractMap;
use hepta_core::PluginInstallSecretContractLifecyclePlane;
use hepta_core::QaLiveProofHarnessContractPlane;
use hepta_core::TalkSessionControllerContractPlane;
use hepta_core::VendoredHeptaSidecarRuntimeRpcContract;
use hepta_core::hepta_cli_compatibility_summary;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaRuntimeContractInventory {
    pub sample_run_executed: bool,
    pub cli_compatibility_map: HeptaCliCompatibilityMap,
    pub node_device_contract_plane: NodeDeviceContractPlane,
    pub config_update_security_secrets_lifecycle_dry_run_map:
        ConfigUpdateSecuritySecretsLifecycleDryRunMap,
    pub channel_message_contract_map: ChannelMessageContractMap,
    pub acp_agent_bridge_matrix: AcpAgentBridgeMatrix,
    pub operational_utility_contract_map: OperationalUtilityContractMap,
    pub vendored_sidecar_runtime_rpc_contract: VendoredHeptaSidecarRuntimeRpcContract,
    pub hepta_2026_5_6_hardening_regressions: Hepta2026_5_6HardeningRegressions,
    pub gateway_session_task_liveness_plane: GatewaySessionTaskLivenessPlane,
    pub channel_delivery_streaming_parity_plane: ChannelDeliveryStreamingParityPlane,
    pub plugin_install_secret_contract_lifecycle_plane: PluginInstallSecretContractLifecyclePlane,
    pub acp_codex_approval_lifecycle_plane: AcpCodexApprovalLifecyclePlane,
    pub cli_status_auth_parity_plane: CliStatusAuthParityPlane,
    pub gateway_plugin_startup_diagnostics_plane: GatewayPluginStartupDiagnosticsPlane,
    pub talk_session_controller_contract_plane: TalkSessionControllerContractPlane,
    pub qa_live_proof_harness_contract_plane: QaLiveProofHarnessContractPlane,
}

impl HeptaRuntimeContractInventory {
    pub fn new(sample_run: bool) -> Self {
        Self {
            sample_run_executed: sample_run,
            cli_compatibility_map: HeptaCliCompatibilityMap::current(sample_run),
            node_device_contract_plane: NodeDeviceContractPlane::new(sample_run),
            config_update_security_secrets_lifecycle_dry_run_map:
                ConfigUpdateSecuritySecretsLifecycleDryRunMap::new(sample_run),
            channel_message_contract_map: ChannelMessageContractMap::new(sample_run),
            acp_agent_bridge_matrix: AcpAgentBridgeMatrix::new(sample_run),
            operational_utility_contract_map: OperationalUtilityContractMap::new(sample_run),
            vendored_sidecar_runtime_rpc_contract: VendoredHeptaSidecarRuntimeRpcContract::new(
                sample_run,
            ),
            hepta_2026_5_6_hardening_regressions: Hepta2026_5_6HardeningRegressions::new(
                sample_run,
            ),
            gateway_session_task_liveness_plane: GatewaySessionTaskLivenessPlane::new(sample_run),
            channel_delivery_streaming_parity_plane: ChannelDeliveryStreamingParityPlane::new(
                sample_run,
            ),
            plugin_install_secret_contract_lifecycle_plane:
                PluginInstallSecretContractLifecyclePlane::new(sample_run),
            acp_codex_approval_lifecycle_plane: AcpCodexApprovalLifecyclePlane::new(sample_run),
            cli_status_auth_parity_plane: CliStatusAuthParityPlane::new(sample_run),
            gateway_plugin_startup_diagnostics_plane: GatewayPluginStartupDiagnosticsPlane::new(
                sample_run,
            ),
            talk_session_controller_contract_plane: TalkSessionControllerContractPlane::new(
                sample_run,
            ),
            qa_live_proof_harness_contract_plane: QaLiveProofHarnessContractPlane::new(sample_run),
        }
    }

    pub fn summaries(&self) -> Vec<HeptaContractPlaneSummary> {
        vec![
            hepta_cli_compatibility_summary(&self.cli_compatibility_map),
            self.node_device_contract_plane.summary(),
            self.config_update_security_secrets_lifecycle_dry_run_map
                .summary(),
            self.channel_message_contract_map.summary(),
            self.acp_agent_bridge_matrix.summary(),
            self.operational_utility_contract_map.summary(),
            self.vendored_sidecar_runtime_rpc_contract.summary(),
            self.hepta_2026_5_6_hardening_regressions.summary(),
            self.gateway_session_task_liveness_plane.summary(),
            self.channel_delivery_streaming_parity_plane.summary(),
            self.plugin_install_secret_contract_lifecycle_plane
                .summary(),
            self.acp_codex_approval_lifecycle_plane.summary(),
            self.cli_status_auth_parity_plane.summary(),
            self.gateway_plugin_startup_diagnostics_plane.summary(),
            self.talk_session_controller_contract_plane.summary(),
            self.qa_live_proof_harness_contract_plane.summary(),
        ]
    }

    pub fn all_ready_and_side_effect_free(&self) -> bool {
        self.summaries().iter().all(|summary| {
            summary.status == "ready"
                && !summary.side_effects_performed
                && !summary.credential_value_read
                && summary.deferred_count.unwrap_or(0) == 0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_inventory_wraps_typed_hepta_contract_planes() {
        let inventory = HeptaRuntimeContractInventory::new(true);
        let summaries = inventory.summaries();
        assert_eq!(summaries.len(), 16);
        assert!(inventory.all_ready_and_side_effect_free());
        assert_eq!(summaries[0].row_count, Some(56));
        assert_eq!(summaries[0].coverage_complete, Some(true));
        assert_eq!(
            summaries.last().map(|summary| summary.id.as_str()),
            Some("qa-live-proof-harness-contract-plane")
        );
    }
}
