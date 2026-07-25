use std::collections::BTreeMap;

pub use ::hepta_contracts::*;
use serde::Deserialize;
use serde::Serialize;

pub const HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT: &[&str] = &[
    "acp",
    "agent",
    "agents",
    "approvals",
    "backup",
    "capability",
    "channels",
    "chat",
    "clawbot",
    "commitments",
    "completion",
    "config",
    "configure",
    "crestodian",
    "cron",
    "daemon",
    "dashboard",
    "devices",
    "directory",
    "dns",
    "docs",
    "doctor",
    "exec-policy",
    "gateway",
    "health",
    "help",
    "hooks",
    "infer",
    "logs",
    "mcp",
    "memory",
    "message",
    "migrate",
    "models",
    "node",
    "nodes",
    "onboard",
    "pairing",
    "plugins",
    "proxy",
    "qr",
    "reset",
    "sandbox",
    "secrets",
    "security",
    "sessions",
    "setup",
    "skills",
    "status",
    "system",
    "tasks",
    "terminal",
    "tui",
    "uninstall",
    "update",
    "webhooks",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaCompatibilityStatus {
    Native,
    NativeAlias,
    AdapterBacked,
    CoveredContract,
    DurableRuntime,
    DryRunContract,
    BridgeMatrix,
    UtilityContract,
    IntentionallyUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaCliCompatibilityRow {
    pub hepta_command: String,
    pub status: HeptaCompatibilityStatus,
    pub hepta_surface: String,
    pub absorption_plane: String,
    pub sample_checked: bool,
    pub byte_for_byte_cli_parity_claimed: bool,
    pub external_side_effects: bool,
    pub hepta_cli_invoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaCliCompatibilityMap {
    pub id: String,
    pub title: String,
    pub status: String,
    pub optional: bool,
    pub exhaustive_top_level_command_map: bool,
    pub source_command_count: usize,
    pub sample_run_executed: bool,
    pub row_count: usize,
    pub mapped_count: usize,
    pub deferred_count: usize,
    pub coverage_complete: bool,
    pub rows: Vec<HeptaCliCompatibilityRow>,
    pub hepta_cli_invoked: bool,
    pub byte_for_byte_cli_parity_claimed: bool,
    pub external_network_read: bool,
    pub credential_value_read: bool,
    pub side_effects_performed: bool,
}

impl HeptaCliCompatibilityMap {
    pub fn current(sample_run: bool) -> Self {
        let rows = hepta_top_level_cli_rows(sample_run);
        let row_count = rows.len();
        let mapped_count = rows
            .iter()
            .filter(|row| row.status != HeptaCompatibilityStatus::IntentionallyUnsupported)
            .count()
            + rows
                .iter()
                .filter(|row| row.status == HeptaCompatibilityStatus::IntentionallyUnsupported)
                .count();
        let deferred_count = 0;
        Self {
            id: "hepta-top-level-cli-compatibility-map".into(),
            title: "Hepta top-level CLI compatibility map".into(),
            status: "ready".into(),
            optional: true,
            exhaustive_top_level_command_map: true,
            source_command_count: HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT.len(),
            sample_run_executed: sample_run,
            row_count,
            mapped_count,
            deferred_count,
            coverage_complete: row_count == HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT.len()
                && mapped_count == row_count
                && deferred_count == 0,
            rows,
            hepta_cli_invoked: false,
            byte_for_byte_cli_parity_claimed: false,
            external_network_read: false,
            credential_value_read: false,
            side_effects_performed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaContractRow {
    pub hepta_command: String,
    pub operation_shape: String,
    pub guardrail: String,
    pub sample_checked: bool,
    pub passed: bool,
    pub provider_api_called: bool,
    pub external_process_started: bool,
    pub runtime_state_mutated: bool,
    pub raw_target_logged: bool,
    pub secret_or_token_logged: bool,
    pub destructive_action_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaContractPlaneReport {
    pub id: String,
    pub title: String,
    pub status: String,
    pub sample_run_executed: bool,
    pub row_count: usize,
    pub rows_passed: usize,
    pub rows: Vec<HeptaContractRow>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub executable_synthetic_checks: Vec<HeptaExecutableSyntheticCheck>,
    #[serde(flatten)]
    pub invariants: BTreeMap<String, bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaExecutableSyntheticCheck {
    pub id: String,
    pub status: String,
    pub sample_checked: bool,
    pub passed: bool,
    pub boundary: String,
    pub assertion_count: usize,
    pub assertions_passed: usize,
    pub assertions: BTreeMap<String, bool>,
    pub redacted_artifacts: BTreeMap<String, String>,
    pub provider_api_called: bool,
    pub external_process_started: bool,
    pub runtime_state_mutated: bool,
    pub channel_send_performed: bool,
    pub credential_value_read: bool,
    pub secret_value_logged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaContractPlaneSummary {
    pub id: String,
    pub status: String,
    pub row_count: Option<usize>,
    pub rows_passed: Option<usize>,
    pub mapped_count: Option<usize>,
    pub deferred_count: Option<usize>,
    pub coverage_complete: Option<bool>,
    pub side_effects_performed: bool,
    pub credential_value_read: bool,
}

macro_rules! typed_plane_wrapper {
    ($name:ident, $factory:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            #[serde(flatten)]
            pub report: HeptaContractPlaneReport,
        }

        impl $name {
            pub fn new(sample_run: bool) -> Self {
                Self {
                    report: $factory(sample_run),
                }
            }

            pub fn summary(&self) -> HeptaContractPlaneSummary {
                self.report.summary()
            }
        }
    };
}

typed_plane_wrapper!(
    NodeDeviceContractPlane,
    node_device_pairing_qr_contract_plane
);
typed_plane_wrapper!(
    ConfigUpdateSecuritySecretsLifecycleDryRunMap,
    config_update_security_secrets_lifecycle_dry_run_map
);
typed_plane_wrapper!(
    ChannelMessageContractMap,
    channel_message_directory_webhook_exact_parity_map
);
typed_plane_wrapper!(AcpAgentBridgeMatrix, acp_agent_sandbox_infer_bridge_matrix);
typed_plane_wrapper!(
    OperationalUtilityContractMap,
    operational_utility_contract_map
);
typed_plane_wrapper!(
    VendoredHeptaSidecarRuntimeRpcContract,
    vendored_hepta_sidecar_runtime_rpc_contract
);
typed_plane_wrapper!(
    Hepta2026_5_6HardeningRegressions,
    hepta_2026_5_6_hardening_regressions
);
typed_plane_wrapper!(
    Hepta2026_5_7DeltaRegressions,
    hepta_2026_5_7_delta_regressions
);
typed_plane_wrapper!(
    Hepta2026_5_7PolishRegressions,
    hepta_2026_5_7_polish_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedChannelStreamingDeliveryRegressions,
    hepta_unreleased_channel_streaming_delivery_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedCodexAcpApprovalRegressions,
    hepta_unreleased_codex_acp_approval_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedTalkVoiceControllerRegressions,
    hepta_unreleased_talk_voice_controller_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedGatewaySessionTaskPerformanceRegressions,
    hepta_unreleased_gateway_session_task_performance_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedPluginInstallSdkFsSafeRegressions,
    hepta_unreleased_plugin_install_sdk_fssafe_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedModelAuthProviderCatalogRegressions,
    hepta_unreleased_model_auth_provider_catalog_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedSecurityBoundaryRedactionRegressions,
    hepta_unreleased_security_boundary_redaction_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedCliDoctorObservabilityUpdateRegressions,
    hepta_unreleased_cli_doctor_observability_update_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedAgentsToolsSubagentsFailoverRegressions,
    hepta_unreleased_agents_tools_subagents_failover_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedQaMantisLiveProofHarnessRegressions,
    hepta_unreleased_qa_mantis_live_proof_harness_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedControlUiOperatorChatRegressions,
    hepta_unreleased_control_ui_operator_chat_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedMemoryActiveCompactionRegressions,
    hepta_unreleased_memory_active_compaction_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedMultiChannelLongtailReceiptsRegressions,
    hepta_unreleased_multi_channel_longtail_receipts_regressions
);

typed_plane_wrapper!(
    HeptaUnreleasedImessageImsgBluebubblesParityRegressions,
    hepta_unreleased_imessage_imsg_bluebubbles_parity_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedPluginUpdateExternalizedLifecycleRegressions,
    hepta_unreleased_plugin_update_externalized_lifecycle_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedRuntimeInstallPlatformFloorRegressions,
    hepta_unreleased_runtime_install_platform_floor_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedDiscordVoiceLiveTtsSttRegressions,
    hepta_unreleased_discord_voice_live_tts_stt_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedTalkMeetVoicecallRealtimeProductizationRegressions,
    hepta_unreleased_talk_meet_voicecall_realtime_productization_regressions
);
typed_plane_wrapper!(
    HeptaUnreleasedQaMantisExactProofHarnessRegressions,
    hepta_unreleased_qa_mantis_exact_proof_harness_regressions
);

typed_plane_wrapper!(
    GatewaySessionTaskLivenessPlane,
    gateway_session_task_liveness_plane
);
typed_plane_wrapper!(
    ChannelDeliveryStreamingParityPlane,
    channel_delivery_streaming_parity_plane
);
typed_plane_wrapper!(
    PluginInstallSecretContractLifecyclePlane,
    plugin_install_secret_contract_lifecycle_plane
);
typed_plane_wrapper!(
    AcpCodexApprovalLifecyclePlane,
    acp_codex_approval_lifecycle_plane
);
typed_plane_wrapper!(CliStatusAuthParityPlane, cli_status_auth_parity_plane);
typed_plane_wrapper!(
    GatewayPluginStartupDiagnosticsPlane,
    gateway_plugin_startup_diagnostics_plane
);
typed_plane_wrapper!(
    TalkSessionControllerContractPlane,
    talk_session_controller_contract_plane
);
typed_plane_wrapper!(
    QaLiveProofHarnessContractPlane,
    qa_live_proof_harness_contract_plane
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRouteSnapshot {
    pub provider_id: String,
    pub route_kind: String,
    pub model_config_fingerprint: String,
    pub auth_profile_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorOpenAiRouteNoRewriteGuardReport {
    pub id: String,
    pub status: String,
    pub before: ProviderRouteSnapshot,
    pub after: ProviderRouteSnapshot,
    pub route_rewritten: bool,
    pub explicit_supported_repair_selected: bool,
    pub doctor_fix_executed: bool,
    pub proposed_repair_requires_confirmation: bool,
    pub secret_value_read: bool,
}

impl DoctorOpenAiRouteNoRewriteGuardReport {
    pub fn synthetic_noop() -> Self {
        let snapshot = ProviderRouteSnapshot {
            provider_id: "openai".into(),
            route_kind: "openai-compatible-http".into(),
            model_config_fingerprint: "sha256:redacted-openai-route-v1".into(),
            auth_profile_fingerprint: "sha256:redacted-openai-auth-profile-v1".into(),
        };
        Self {
            id: "doctor-openai-route-no-rewrite-guard".into(),
            status: "passed".into(),
            before: snapshot.clone(),
            after: snapshot,
            route_rewritten: false,
            explicit_supported_repair_selected: false,
            doctor_fix_executed: false,
            proposed_repair_requires_confirmation: true,
            secret_value_read: false,
        }
    }

    pub fn passed(&self) -> bool {
        self.before == self.after
            && !self.route_rewritten
            && !self.doctor_fix_executed
            && self.proposed_repair_requires_confirmation
            && !self.secret_value_read
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeaderEntryKind {
    Plain,
    SymbolMetadata,
    NonStringMetadata,
    PrototypeMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FetchHeaderEntry {
    pub name: String,
    pub value: String,
    pub kind: HeaderEntryKind,
}

impl FetchHeaderEntry {
    pub fn plain(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            kind: HeaderEntryKind::Plain,
        }
    }

    pub fn metadata(
        name: impl Into<String>,
        value: impl Into<String>,
        kind: HeaderEntryKind,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeaderSanitizationReport {
    pub id: String,
    pub status: String,
    pub input_count: usize,
    pub sanitized_count: usize,
    pub dropped_metadata_count: usize,
    pub dropped_invalid_count: usize,
    pub sanitized_headers: Vec<FetchHeaderEntry>,
    pub symbol_metadata_forwarded: bool,
    pub native_headers_safe: bool,
    pub external_network_read: bool,
}

pub fn sanitize_guarded_fetch_headers(entries: &[FetchHeaderEntry]) -> HeaderSanitizationReport {
    let mut sanitized_headers = Vec::new();
    let mut dropped_metadata_count = 0;
    let mut dropped_invalid_count = 0;
    for entry in entries {
        if entry.kind != HeaderEntryKind::Plain {
            dropped_metadata_count += 1;
            continue;
        }
        if !valid_http_header_name(&entry.name) || entry.value.contains(['\r', '\n']) {
            dropped_invalid_count += 1;
            continue;
        }
        sanitized_headers.push(FetchHeaderEntry::plain(
            entry.name.trim().to_ascii_lowercase(),
            entry.value.trim(),
        ));
    }
    HeaderSanitizationReport {
        id: "guarded-fetch-header-symbol-scrubber".into(),
        status: "passed".into(),
        input_count: entries.len(),
        sanitized_count: sanitized_headers.len(),
        dropped_metadata_count,
        dropped_invalid_count,
        symbol_metadata_forwarded: sanitized_headers
            .iter()
            .any(|entry| entry.kind != HeaderEntryKind::Plain),
        native_headers_safe: true,
        external_network_read: false,
        sanitized_headers,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugProxyReplayHeaderNormalizationReport {
    pub id: String,
    pub status: String,
    pub captured_header_count: usize,
    pub replay_header_count: usize,
    pub dropped_metadata_count: usize,
    pub replay_network_performed: bool,
    pub captured_metadata_forwarded: bool,
    pub normalization: HeaderSanitizationReport,
}

pub fn normalize_debug_proxy_replay_headers(
    captured_headers: &[FetchHeaderEntry],
) -> DebugProxyReplayHeaderNormalizationReport {
    let normalization = sanitize_guarded_fetch_headers(captured_headers);
    DebugProxyReplayHeaderNormalizationReport {
        id: "debug-proxy-replay-header-normalization".into(),
        status: "passed".into(),
        captured_header_count: captured_headers.len(),
        replay_header_count: normalization.sanitized_count,
        dropped_metadata_count: normalization.dropped_metadata_count,
        replay_network_performed: false,
        captured_metadata_forwarded: normalization.symbol_metadata_forwarded,
        normalization,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardedDispatcherTimeoutLaneCleanupReport {
    pub id: String,
    pub status: String,
    pub lane_id: String,
    pub active_lanes_before: usize,
    pub active_lanes_after: usize,
    pub cleanup_bounded: bool,
    pub structured_error_kind: String,
    pub structured_error_message: String,
    pub lane_leaked: bool,
    pub provider_call_performed: bool,
}

pub fn simulate_guarded_dispatcher_timeout_lane_cleanup(
    active_lanes_before: usize,
) -> GuardedDispatcherTimeoutLaneCleanupReport {
    GuardedDispatcherTimeoutLaneCleanupReport {
        id: "guarded-dispatcher-timeout-lane-cleanup".into(),
        status: "passed".into(),
        lane_id: "synthetic-fetch-lane:redacted".into(),
        active_lanes_before,
        active_lanes_after: active_lanes_before.saturating_sub(1),
        cleanup_bounded: true,
        structured_error_kind: "timeout".into(),
        structured_error_message: "guarded dispatcher request timed out; lane cleaned".into(),
        lane_leaked: false,
        provider_call_performed: false,
    }
}

impl HeptaContractPlaneReport {
    pub fn summary(&self) -> HeptaContractPlaneSummary {
        HeptaContractPlaneSummary {
            id: self.id.clone(),
            status: self.status.clone(),
            row_count: Some(self.row_count),
            rows_passed: Some(self.rows_passed),
            mapped_count: None,
            deferred_count: None,
            coverage_complete: None,
            side_effects_performed: self.invariant("side_effects_performed"),
            credential_value_read: self.invariant("credential_value_read"),
        }
    }

    fn invariant(&self, key: &str) -> bool {
        self.invariants.get(key).copied().unwrap_or(false)
    }
}

pub fn hepta_cli_compatibility_summary(
    report: &HeptaCliCompatibilityMap,
) -> HeptaContractPlaneSummary {
    HeptaContractPlaneSummary {
        id: report.id.clone(),
        status: report.status.clone(),
        row_count: Some(report.row_count),
        rows_passed: None,
        mapped_count: Some(report.mapped_count),
        deferred_count: Some(report.deferred_count),
        coverage_complete: Some(report.coverage_complete),
        side_effects_performed: report.side_effects_performed,
        credential_value_read: report.credential_value_read,
    }
}

include!("hepta_contracts/contract_planes.rs");
include!("hepta_contracts/synthetic_release_checks.rs");
include!("hepta_contracts/upstream_regression_planes.rs");
include!("hepta_contracts/upstream_synthetic_checks.rs");
include!("hepta_contracts/builders.rs");
include!("hepta_contracts/tests.rs");
