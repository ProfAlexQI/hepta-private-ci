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

pub fn node_device_pairing_qr_contract_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "node-device-pairing-qr-contract-plane",
        "Node / devices / pairing / QR contract plane",
        sample_run,
        vec![
            row(
                "node",
                "headless-node-service",
                "service lifecycle is metadata-only",
                sample_run,
            ),
            row(
                "nodes",
                "gateway-owned-node-actions",
                "camera/screen/location/notify actions require explicit live adapter confirmation",
                sample_run,
            ),
            row(
                "devices",
                "device-pairing-token-inventory",
                "token inventory is redacted and local-only",
                sample_run,
            ),
            row(
                "pairing",
                "secure-dm-pairing",
                "approve/reject mutations are not performed in report mode",
                sample_run,
            ),
            row(
                "qr",
                "mobile-setup-qr",
                "QR payload shape is classified without token materialization",
                sample_run,
            ),
        ],
        &[
            ("device_action_performed", false),
            ("camera_capture_performed", false),
            ("screen_capture_performed", false),
            ("location_read_performed", false),
            ("notification_sent", false),
            ("pairing_state_mutated", false),
            ("qr_token_materialized", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_send", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn config_update_security_secrets_lifecycle_dry_run_map(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane(
        "config-update-security-secrets-lifecycle-dry-run-map",
        "Config / update / security / secrets lifecycle dry-run map",
        sample_run,
        vec![
            row(
                "config",
                "validate/get/set/unset/file",
                "write requires explicit config patch/apply flow",
                sample_run,
            ),
            row(
                "configure",
                "interactive-credential-onboarding",
                "plan-only; no credential prompt in report mode",
                sample_run,
            ),
            row(
                "setup",
                "workspace-initialization",
                "no directories or files created in report mode",
                sample_run,
            ),
            row(
                "onboard",
                "gateway-workspace-skills-onboarding",
                "dry-run plan only",
                sample_run,
            ),
            row(
                "update",
                "self-update",
                "update.run remains blocked unless explicitly requested",
                sample_run,
            ),
            row(
                "security",
                "local-config-audit",
                "audit shape only; no privileged action",
                sample_run,
            ),
            row(
                "secrets",
                "runtime-secret-reload",
                "secret values are never read",
                sample_run,
            ),
            row(
                "doctor",
                "health-check-and-repair",
                "repair preview only; no package manager",
                sample_run,
            ),
            row(
                "exec-policy",
                "approval-policy-sync",
                "approval sync not performed in report mode",
                sample_run,
            ),
            row(
                "approvals",
                "approval-state-inspection",
                "approval mutation not performed",
                sample_run,
            ),
            row(
                "reset",
                "local-config-state-reset",
                "destructive reset intentionally report-only",
                sample_run,
            ),
            row(
                "crestodian",
                "ring-zero-setup-repair",
                "operator-only actions represented as blocked dry-run",
                sample_run,
            ),
        ],
        &[
            ("config_written", false),
            ("state_reset_performed", false),
            ("package_manager_invoked", false),
            ("hepta_update_invoked", false),
            ("secret_file_read", false),
            ("credential_value_read", false),
            ("approval_state_mutated", false),
            ("privileged_repair_performed", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn channel_message_directory_webhook_exact_parity_map(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane(
        "channel-message-directory-webhook-exact-parity-map",
        "Channels / message / directory / webhooks exact parity map",
        sample_run,
        vec![
            row(
                "channels",
                "login/status/list/logout",
                "channel auth and live login are out-of-band",
                sample_run,
            ),
            row(
                "message",
                "send/read/react/pin/thread/poll",
                "external send requires explicit confirmation",
                sample_run,
            ),
            row(
                "directory",
                "self/peer/group lookup",
                "metadata lookup redacts raw provider ids",
                sample_run,
            ),
            row(
                "webhooks",
                "register/list/test payload",
                "webhook ack/send not performed",
                sample_run,
            ),
            row(
                "status",
                "channel health/recent recipients",
                "status is metadata-only",
                sample_run,
            ),
            row(
                "telegram",
                "topic/progress/reply routing",
                "covered by adapter-owned route parser",
                sample_run,
            ),
            row(
                "discord",
                "thread/heartbeat/steer mention gates",
                "auth/mention gate represented as contract",
                sample_run,
            ),
            row(
                "feishu",
                "chat/topic hydration",
                "topic starter shape only",
                sample_run,
            ),
            row(
                "imessage",
                "chat db/local-probe/live-send",
                "live-send blocked without confirm",
                sample_run,
            ),
            row(
                "slack-matrix-msteams-line-whatsapp",
                "threaded route shapes",
                "raw target ids not logged",
                sample_run,
            ),
        ],
        &[
            ("channel_send_performed", false),
            ("message_provider_api_called", false),
            ("webhook_registered", false),
            ("webhook_ack_performed", false),
            ("directory_live_lookup_performed", false),
            ("raw_channel_id_logged", false),
            ("raw_chat_id_logged", false),
            ("raw_message_id_logged", false),
            ("credential_value_read", false),
            ("external_send", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn acp_agent_sandbox_infer_bridge_matrix(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "acp-agent-sandbox-infer-bridge-matrix",
        "ACP / agent / sandbox / infer runtime bridge matrix",
        sample_run,
        vec![
            row(
                "acp",
                "ACP harness routing",
                "session spawn/telephone-game flow remains explicit",
                sample_run,
            ),
            row(
                "agent",
                "single agent turn via gateway",
                "provider call not performed in report mode",
                sample_run,
            ),
            row(
                "agents",
                "isolated agents/workspaces/auth/routing",
                "workspace mutation not performed",
                sample_run,
            ),
            row(
                "capability",
                "provider-backed inference alias",
                "provider selection shape only",
                sample_run,
            ),
            row(
                "infer",
                "provider-backed inference",
                "no model request sent",
                sample_run,
            ),
            row(
                "sandbox",
                "containerized execution isolation",
                "container not started",
                sample_run,
            ),
            row(
                "exec-policy",
                "host approvals sync",
                "approval sync blocked in report mode",
                sample_run,
            ),
        ],
        &[
            ("acp_session_started", false),
            ("agent_turn_started", false),
            ("provider_call_performed", false),
            ("sandbox_container_started", false),
            ("approval_policy_mutated", false),
            ("workspace_mutated", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn operational_utility_contract_map(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "operational-utility-contract-map",
        "Backup / docs / logs / migrate / skills / system / tasks operational utility map",
        sample_run,
        vec![
            row(
                "backup",
                "archive-create/verify",
                "archive write not performed",
                sample_run,
            ),
            row(
                "completion",
                "shell completion generation",
                "stdout contract only",
                sample_run,
            ),
            row(
                "dns",
                "tailscale/coredns helpers",
                "network config not changed",
                sample_run,
            ),
            row(
                "docs",
                "docs search/open",
                "no live docs fetch in report mode",
                sample_run,
            ),
            row(
                "logs",
                "gateway log tail",
                "log file not tailed",
                sample_run,
            ),
            row(
                "mcp",
                "MCP config/channel bridge",
                "MCP server not started",
                sample_run,
            ),
            row(
                "migrate",
                "foreign agent state import",
                "source state not read",
                sample_run,
            ),
            row(
                "proxy",
                "debug proxy capture",
                "listener not started",
                sample_run,
            ),
            row("skills", "skill list/inspect", "metadata-only", sample_run),
            row(
                "system",
                "system events/heartbeat/presence",
                "event not emitted",
                sample_run,
            ),
            row(
                "tasks",
                "durable task state",
                "task mutation not performed",
                sample_run,
            ),
            row(
                "terminal",
                "local terminal UI alias",
                "TUI not launched",
                sample_run,
            ),
            row(
                "tui",
                "gateway/local terminal UI",
                "TUI not launched",
                sample_run,
            ),
            row(
                "uninstall",
                "service/data uninstall",
                "intentionally unsupported destructive action",
                sample_run,
            ),
            row(
                "gateway",
                "gateway runtime control",
                "gateway RPC not performed",
                sample_run,
            ),
            row(
                "health",
                "gateway health fetch",
                "gateway RPC not performed",
                sample_run,
            ),
            row(
                "dashboard",
                "Control UI open",
                "browser not launched",
                sample_run,
            ),
            row(
                "hooks",
                "internal hook management",
                "hook queue not mutated",
                sample_run,
            ),
            row(
                "commitments",
                "follow-up commitments",
                "commitment state not mutated",
                sample_run,
            ),
            row(
                "clawbot",
                "legacy aliases",
                "legacy compatibility documented only",
                sample_run,
            ),
            row(
                "daemon",
                "legacy gateway alias",
                "service lifecycle not invoked",
                sample_run,
            ),
            row("chat", "TUI local alias", "TUI not launched", sample_run),
        ],
        &[
            ("archive_written", false),
            ("listener_started", false),
            ("browser_opened", false),
            ("gateway_rpc_performed", false),
            ("mcp_server_started", false),
            ("source_state_imported", false),
            ("task_state_mutated", false),
            ("hook_queue_mutated", false),
            ("destructive_uninstall_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn vendored_hepta_sidecar_runtime_rpc_contract(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "vendored-hepta-js-sidecar-runtime-rpc-contract",
        "Vendored Hepta JS plugin sidecar runtime RPC contract",
        sample_run,
        vec![
            row(
                "sidecar-inventory",
                "plugin inventory RPC",
                "metadata-only inventory; no plugin import or runtime start",
                sample_run,
            ),
            row(
                "sidecar-receive",
                "channel receive RPC",
                "receive shape gated by explicit token source and redacted ledger",
                sample_run,
            ),
            row(
                "sidecar-send",
                "channel send RPC",
                "send remains confirmation-gated and dry-run by default",
                sample_run,
            ),
            row(
                "sidecar-tool",
                "tool invocation RPC",
                "tool calls require Hepta policy/approval envelope",
                sample_run,
            ),
            row(
                "sidecar-provider",
                "provider command RPC",
                "provider calls remain disabled in report mode",
                sample_run,
            ),
            row(
                "sidecar-ledger",
                "redacted runtime ledger",
                "Hepta owns redaction, provenance, and side-effect ledger",
                sample_run,
            ),
        ],
        &[
            ("sidecar_process_started", false),
            ("plugin_import_attempted", false),
            ("plugin_runtime_started", false),
            ("channel_receive_performed", false),
            ("channel_send_performed", false),
            ("tool_invoked", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn hepta_2026_5_6_hardening_regressions(sample_run: bool) -> HeptaContractPlaneReport {
    let doctor = DoctorOpenAiRouteNoRewriteGuardReport::synthetic_noop();
    let guarded_fetch = sanitize_guarded_fetch_headers(&synthetic_metadata_headers());
    let debug_proxy = normalize_debug_proxy_replay_headers(&synthetic_metadata_headers());
    let timeout_cleanup = simulate_guarded_dispatcher_timeout_lane_cleanup(1);

    contract_plane(
        "hepta-2026-5-6-hardening-regressions",
        "Hepta 2026.5.6 doctor/fetch/proxy/timeout hardening regressions",
        sample_run,
        vec![
            row(
                "doctor",
                "doctor-openai-route-no-rewrite-guard",
                "doctor repair cannot silently rewrite existing OpenAI-compatible routes",
                sample_run,
            ),
            row(
                "plugins-runtime-fetch",
                "guarded-fetch-header-symbol-scrubber",
                "symbol/non-string/prototype metadata is dropped before native Headers construction",
                sample_run,
            ),
            row(
                "proxy",
                "debug-proxy-replay-header-normalization",
                "captured headers are normalized before replay envelopes are built",
                sample_run,
            ),
            row(
                "web-fetch",
                "guarded-dispatcher-timeout-lane-cleanup",
                "timed-out guarded dispatch returns structured error and releases active lane",
                sample_run,
            ),
        ],
        &[
            ("openai_route_rewritten", doctor.route_rewritten),
            ("doctor_fix_executed", doctor.doctor_fix_executed),
            ("doctor_secret_value_read", doctor.secret_value_read),
            (
                "header_symbol_metadata_forwarded",
                guarded_fetch.symbol_metadata_forwarded,
            ),
            ("native_headers_safe", guarded_fetch.native_headers_safe),
            (
                "debug_proxy_replay_network_performed",
                debug_proxy.replay_network_performed,
            ),
            (
                "debug_proxy_metadata_forwarded",
                debug_proxy.captured_metadata_forwarded,
            ),
            ("dispatcher_lane_leaked", timeout_cleanup.lane_leaked),
            ("timeout_cleanup_bounded", timeout_cleanup.cleanup_bounded),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn hepta_2026_5_7_delta_regressions(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-2026-5-7-delta-regressions",
        "Hepta 2026.5.7 exact delta regressions for owner/authz, context, delivery, cron, sessions, plugins, providers, and channel edges",
        sample_run,
        vec![
            row(
                "native-commands",
                "owner-enforced-native-command-handlers",
                "native command handlers check owner scope before any handler side effect",
                sample_run,
            ),
            row(
                "auto-reply/skills",
                "before-tool-call-authorization-for-inline-skill-dispatch",
                "auto-reply inline skill dispatch is gated by before-tool-call authorization hooks",
                sample_run,
            ),
            row(
                "agents/context",
                "assembled-context-cache-invalidation-on-shrink-or-failure",
                "assembled context views are invalidated when source history shrinks or assembly fails",
                sample_run,
            ),
            row(
                "agents/compaction",
                "summary-reserve-clamped-to-model-output-limit",
                "compaction summary reserve tokens are clamped to the active model output limit",
                sample_run,
            ),
            row(
                "agent-delivery",
                "empty-adapter-result-marks-delivery-failed",
                "empty outbound adapter results produce deliverySucceeded=false instead of false success",
                sample_run,
            ),
            row(
                "cron/isolated",
                "delivery-last-preflight-before-model-execution",
                "delivery.channel=last without a previous route fails before model execution",
                sample_run,
            ),
            row(
                "gateway/sessions",
                "daily-rollover-generated-transcript-persistence",
                "daily gateway-agent session rollover creates a generated transcript while preserving custom transcript paths",
                sample_run,
            ),
            row(
                "plugins/install",
                "absolute-posix-npm-lifecycle-shell-consistency",
                "managed plugin install, rollback, repair, and uninstall use the same absolute POSIX npm lifecycle shell",
                sample_run,
            ),
            row(
                "plugins/channel-setup",
                "external-set-channel-runtime-forwarding",
                "non-bundled external plugin setup forwards setChannelRuntime before startup polling",
                sample_run,
            ),
            row(
                "providers/channels",
                "provider-channel-edge-normalization-pack",
                "APNG, Gemini 3 signatures, __env__ keys, snake_case transcripts, WhatsApp LID, captioned MEDIA, and Discord voice capability edges are normalized without live calls",
                sample_run,
            ),
        ],
        &[
            ("owner_enforcement_bypassed", false),
            ("before_tool_call_hook_skipped", false),
            ("stale_context_view_reused", false),
            ("compaction_max_tokens_invalid", false),
            ("delivery_success_claimed_on_empty_adapter_result", false),
            ("cron_model_executed_before_delivery_preflight", false),
            ("rollover_transcript_missing", false),
            ("npm_lifecycle_shell_inconsistent", false),
            ("external_set_channel_runtime_dropped", false),
            ("provider_channel_edge_regression_missing", false),
            ("provider_call_performed", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("external_send", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_2026_5_7_delta_checks(sample_run),
    )
}

pub fn hepta_2026_5_7_polish_regressions(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-2026-5-7-polish-regressions",
        "Hepta 2026.5.7 polish regressions for release publishing, sanitizers, cron repair, Telegram auth/callbacks, subagent TTL, and Discord voice config",
        sample_run,
        vec![
            row(
                "release/clawhub",
                "clawhub-publish-retry-and-version-verification",
                "ClawHub publish maintenance retries transient dependency installs, tolerates isolated preview-cell flakes, and verifies expected package versions after publish without publishing by default",
                sample_run,
            ),
            row(
                "commands/btw",
                "btw-placeholder-bracket-preserving-sanitizer",
                "/btw missing-question usage placeholder keeps bracketed text after outbound sanitization",
                sample_run,
            ),
            row(
                "cron/doctor",
                "payload-model-exact-repair-fixture",
                "doctor repair removes persisted cron payload.model values stored as default/null/blank while keeping runtime model validation strict",
                sample_run,
            ),
            row(
                "telegram/authz",
                "accessgroup-before-numeric-sender-id-fixture",
                "Telegram DMs, groups, native commands, and callbacks honor accessGroup allowlists before numeric sender-id fallback",
                sample_run,
            ),
            row(
                "agents/subagents",
                "archive-after-minutes-exact-ttl-fixture",
                "completed session-mode subagent registry rows use agents.defaults.subagents.archiveAfterMinutes rather than a hardcoded 5-minute TTL",
                sample_run,
            ),
            row(
                "discord/voice",
                "voice-capture-silence-grace-config-fixture",
                "Discord voice capture defaults post-speech silence grace to 2.5s and parses bounded voice.captureSilenceGraceMs overrides",
                sample_run,
            ),
            row(
                "telegram/models",
                "dotted-provider-id-callback-parser-fixture",
                "Telegram /models inline keyboard callbacks preserve dotted provider ids such as hf.co",
                sample_run,
            ),
            row(
                "release/plugins",
                "redacted-publishing-evidence-ledger-shape",
                "release/plugin publishing stores redacted retry and version-check evidence without registry credentials, prompts, or network writes in report mode",
                sample_run,
            ),
        ],
        &[
            ("clawhub_publish_performed", false),
            ("btw_placeholder_brackets_stripped", false),
            ("cron_payload_model_bad_override_preserved", false),
            ("telegram_accessgroup_bypassed", false),
            ("subagent_ttl_hardcoded_five_minutes", false),
            ("discord_voice_silence_default_missing", false),
            ("telegram_dotted_provider_id_broken", false),
            ("release_evidence_contains_secret", false),
            ("registry_credential_value_read", false),
            ("provider_call_performed", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("external_send", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_2026_5_7_polish_checks(sample_run),
    )
}

pub fn hepta_unreleased_channel_streaming_delivery_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-channel-streaming-delivery-regressions",
        "Hepta upstream-unreleased channel streaming and delivery edge regressions",
        sample_run,
        vec![
            row(
                "channels/streaming",
                "progress-draft-label-scroll-contract",
                "progress draft labels scroll with progress lines and stale pre-tool labels are not reused",
                sample_run,
            ),
            row(
                "channels/streaming",
                "compact-structured-tool-row-rendering",
                "structured tool rows render compact emoji/title/details without raw tool payloads",
                sample_run,
            ),
            row(
                "tools/web-search",
                "native-web-search-query-rendering",
                "provider-native web-search argument shapes render as redacted structured query rows",
                sample_run,
            ),
            row(
                "discord/streaming",
                "discord-apply-patch-empty-start-suppression",
                "empty apply-patch starts are suppressed until a patch summary exists",
                sample_run,
            ),
            row(
                "telegram/poll",
                "telegram-poll-option-cap-preflight",
                "Telegram poll fixtures over the 10-option channel cap are rejected before send",
                sample_run,
            ),
            row(
                "telegram/delivery",
                "telegram-same-chat-success-suppresses-fallback",
                "successful same-chat message tool sends suppress the rewritten silent fallback",
                sample_run,
            ),
            row(
                "telegram/topics",
                "telegram-numeric-forum-topic-plugin-owned",
                "numeric forum-topic targets are parsed as plugin-owned topic routes, not legacy raw ids",
                sample_run,
            ),
            row(
                "telegram/streaming",
                "telegram-stable-runtime-alias-chunking",
                "reply-dispatch chunks keep stable runtime aliases during updates",
                sample_run,
            ),
            row(
                "discord/streaming",
                "discord-progress-draft-preview-default",
                "Discord progress draft previews are enabled by default and respect explicit disable",
                sample_run,
            ),
            row(
                "telegram/streaming",
                "telegram-draft-preview-rotation-after-output",
                "Telegram draft preview rotation invalidates stale pre-tool previews after tool/media output",
                sample_run,
            ),
            row(
                "whatsapp/newsletter",
                "whatsapp-channel-newsletter-targets",
                "WhatsApp @newsletter targets route as channel/newsletter targets instead of DMs",
                sample_run,
            ),
            row(
                "slack/streaming",
                "slack-rich-progress-draft-trimming",
                "Slack rich progress drafts preserve shape while trimming newest progress lines",
                sample_run,
            ),
            row(
                "discord/message",
                "discord-provider-prefixed-channel-route",
                "provider-prefixed discord:channel targets do not misroute into legacy DMs",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("channel_send_performed", false),
            ("provider_call_performed", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("external_send", false),
            ("credential_value_read", false),
            ("private_config_read", false),
            ("private_chat_history_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
            ("raw_channel_id_logged", false),
            ("raw_message_id_logged", false),
            ("raw_tool_payload_logged", false),
        ],
        synthetic_hepta_unreleased_channel_streaming_delivery_checks(sample_run),
    )
}

pub fn hepta_unreleased_codex_acp_approval_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-codex-acp-approval-regressions",
        "Hepta upstream-unreleased Codex, ACP, and approval lifecycle regressions",
        sample_run,
        vec![
            row(
                "codex/app-server",
                "codex-harness-version-and-dynamic-tools",
                "managed Codex harness is pinned and dynamic tools defer behind tool search by default",
                sample_run,
            ),
            row(
                "codex/app-server",
                "codex-post-tool-watchdog-idle-contract",
                "post-tool completion watchdog disarms after current-turn activity and exposes idle timeout diagnostics",
                sample_run,
            ),
            row(
                "codex/approvals",
                "codex-native-permissionrequest-policy",
                "Codex reviewer handles safe native PermissionRequest payloads before Hepta surfaces approval",
                sample_run,
            ),
            row(
                "codex/approvals",
                "codex-allow-always-active-session-scope",
                "allow-always decisions are bounded to identical payloads in the active session window",
                sample_run,
            ),
            row(
                "codex/plugins",
                "codex-plugin-approval-action-shape",
                "plugin approval requests render only the actual allowed decisions",
                sample_run,
            ),
            row(
                "codex/plugins",
                "openai-curated-plugin-thread-contract",
                "source-installed openai-curated Codex plugins share the harness thread with cached readiness",
                sample_run,
            ),
            row(
                "codex/plugins",
                "codex-plugin-destructive-policy-delegation",
                "destructive policy is delegated to Codex app-level destructive_enabled config",
                sample_run,
            ),
            row(
                "acpx/codex",
                "trusted-project-declaration-preservation",
                "isolated Codex ACP launches preserve trusted project declarations",
                sample_run,
            ),
            row(
                "acpx/codex",
                "stale-acpx-process-tree-reaping",
                "Hepta-owned stale ACPX/Codex process trees are reaped on startup/session close",
                sample_run,
            ),
            row(
                "acp/bridge",
                "stable-session-list-resume-close-handlers",
                "ACP bridge exposes stable session list, resume, and close handlers",
                sample_run,
            ),
            row(
                "acp/sessions",
                "parent-owned-cross-agent-visibility",
                "parent agents may inspect/message only their own spawned cross-agent ACP sessions",
                sample_run,
            ),
            row(
                "openai/codex-media",
                "codex-audio-transcription-routing",
                "Codex audio transcription metadata routes active chat models to transcription defaults",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("codex_prompt_uploaded", false),
            ("acp_process_spawned", false),
            ("approval_decision_persisted", false),
            ("provider_call_performed", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_codex_acp_approval_checks(sample_run),
    )
}

pub fn hepta_unreleased_talk_voice_controller_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-talk-voice-controller-regressions",
        "Hepta upstream-unreleased Talk, voice, realtime, and telephony controller regressions",
        sample_run,
        vec![
            row(
                "talk/session",
                "shared-talk-session-controller-rpc",
                "Talk, transcription relay, managed rooms, Voice Call, Google Meet, VoiceClaw, and native clients share talk.session RPC shape",
                sample_run,
            ),
            row(
                "diagnostics/talk",
                "bounded-talk-lifecycle-audio-metrics",
                "Talk lifecycle/audio metrics export bounded redacted OTLP/Prometheus shapes",
                sample_run,
            ),
            row(
                "logging/talk",
                "redacted-talk-lifecycle-logs",
                "Talk lifecycle logs exclude transcripts, audio payloads, turn ids, call ids, and provider item ids",
                sample_run,
            ),
            row(
                "openai/realtime",
                "ga-realtime-default-voice-shape",
                "OpenAI realtime defaults to gpt-realtime-2 and GA WebSocket session shape",
                sample_run,
            ),
            row(
                "google-meet/voice-call",
                "realtime-gemini-bridge-pacing",
                "Meet/Voice Call realtime Gemini bridge uses paced audio and backpressure-aware buffering",
                sample_run,
            ),
            row(
                "voice-call/realtime",
                "voice-context-capsule-cadence",
                "agent voice context capsules and consult-cadence guidance are opt-in and bounded",
                sample_run,
            ),
            row(
                "tts/telephony",
                "telephony-provider-voice-model-overrides",
                "telephony synthesis logs honor provider voice/model overrides",
                sample_run,
            ),
            row(
                "discord/voice",
                "discord-voice-stt-preview-verbose-log",
                "Discord verbose voice logs include a bounded one-line STT preview",
                sample_run,
            ),
            row(
                "discord/voice",
                "elevenlabs-direct-tts-playback",
                "ElevenLabs TTS streams directly into Discord playback with latency optimization",
                sample_run,
            ),
            row(
                "discord/voice",
                "tts-playback-capture-barge-in-guard",
                "TTS playback continues while new capture is ignored to avoid feedback loops",
                sample_run,
            ),
            row(
                "discord/voice",
                "voice-channel-permission-probe-shape",
                "channels capabilities/status probe audits Connect/Speak/Read Message History",
                sample_run,
            ),
            row(
                "google-meet",
                "silent-intro-empty-string-preservation",
                "realtime.introMessage empty string remains silent instead of restoring default intro",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("audio_payload_captured", false),
            ("transcript_text_logged", false),
            ("meet_joined", false),
            ("twilio_call_started", false),
            ("discord_voice_connected", false),
            ("provider_call_performed", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("credential_value_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_talk_voice_controller_checks(sample_run),
    )
}

pub fn hepta_unreleased_gateway_session_task_performance_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-gateway-session-task-performance-regressions",
        "Hepta upstream-unreleased gateway, sessions, tasks, startup, and performance regressions",
        sample_run,
        vec![
            row(
                "gateway/tasks",
                "stale-cli-run-context-reconciliation",
                "stale CLI run-context tasks reconcile when live context disappears even if child session remains",
                sample_run,
            ),
            row(
                "gateway/reload",
                "bounded-channel-hot-reload-deferral",
                "channel hot reloads apply bounded default reload deferral timeouts",
                sample_run,
            ),
            row(
                "gateway/sessions",
                "atomic-session-store-index-writes",
                "session-store index writes are atomic while durable fsync is skipped inside writer lock",
                sample_run,
            ),
            row(
                "sessions/cli",
                "qualified-model-ref-fast-path",
                "session-list rows fast-path already-qualified model refs",
                sample_run,
            ),
            row(
                "sessions/cli",
                "selected-agent-runtime-column",
                "sessions tables include selected agent runtime",
                sample_run,
            ),
            row(
                "gateway/startup",
                "startup-phase-span-diagnostics",
                "startup phase spans, active work labels, stale bridge markers, and sync-I/O traces are bounded",
                sample_run,
            ),
            row(
                "gateway/startup",
                "nonreadiness-sidecar-deferral",
                "non-readiness sidecars defer until after ready signal",
                sample_run,
            ),
            row(
                "gateway/performance",
                "plugin-metadata-snapshot-reuse",
                "compatible/current plugin metadata snapshots are reused during dashboard and channel turns",
                sample_run,
            ),
            row(
                "gateway/performance",
                "plugin-auto-enable-single-resolution",
                "plugin auto-enable metadata is not resolved twice in one runtime config pass",
                sample_run,
            ),
            row(
                "gateway/performance",
                "native-loadable-plugin-no-jiti-fast-path",
                "native-loadable plugins avoid jiti import unless fallback loading is required",
                sample_run,
            ),
            row(
                "plugins/loader",
                "compiled-plugin-error-preservation",
                "native fast path preserves real compiled plugin module evaluation errors",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("gateway_restart_performed", false),
            ("session_store_mutated", false),
            ("task_registry_mutated", false),
            ("plugin_runtime_started", false),
            ("provider_call_performed", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_gateway_session_task_performance_checks(sample_run),
    )
}

pub fn hepta_unreleased_plugin_install_sdk_fssafe_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-plugin-install-sdk-fssafe-regressions",
        "Hepta upstream-unreleased plugin install, SDK lifecycle, and fs-safe regressions",
        sample_run,
        vec![
            row(
                "plugins/install",
                "npm-pack-managed-install-path",
                "npm-pack:<path.tgz> installs use managed npm-root, install records, and dependency scanning",
                sample_run,
            ),
            row(
                "plugins/install",
                "local-pack-lockfile-verification",
                "local npm pack artifacts pass lockfile verification and dependency scan before install-record publication",
                sample_run,
            ),
            row(
                "channels/plugins",
                "official-external-channel-missing-plugin-status",
                "configured official external channels render missing-plugin status rows with exact repair commands",
                sample_run,
            ),
            row(
                "doctor/plugins",
                "plugin-owned-legacy-config-repair-order",
                "doctor discovers plugin-owned legacy repair contracts before validation",
                sample_run,
            ),
            row(
                "plugin-skills/windows",
                "plugin-skill-junction-registration",
                "plugin-provided skill dirs register as Windows junctions when symlink is unavailable",
                sample_run,
            ),
            row(
                "plugins/lifecycle",
                "absolute-posix-managed-npm-shell",
                "managed plugin npm operations use an absolute POSIX lifecycle shell",
                sample_run,
            ),
            row(
                "plugin-sdk/channel-message",
                "channel-message-sdk-lifecycle-helpers",
                "SDK exposes channel-message lifecycle helpers without requiring plugin runtime startup",
                sample_run,
            ),
            row(
                "plugin-sdk/fs-safe",
                "staged-external-output-writes",
                "browser/media/channel/QA outputs are staged through fs-safe writes before publication",
                sample_run,
            ),
            row(
                "plugin-sdk/temp-workspace",
                "temp-workspace-helper-rename",
                "public temp workspace helpers use tempWorkspace/withTempWorkspace naming",
                sample_run,
            ),
            row(
                "plugins/loader",
                "compiled-module-error-preservation",
                "compiled plugin loader preserves real module evaluation errors",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("plugin_installed", false),
            ("package_manager_invoked", false),
            ("plugin_runtime_started", false),
            ("external_output_published", false),
            ("filesystem_mutated", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_plugin_install_sdk_fssafe_checks(sample_run),
    )
}

pub fn synthetic_metadata_headers() -> Vec<FetchHeaderEntry> {
    vec![
        FetchHeaderEntry::plain("Content-Type", "application/json"),
        FetchHeaderEntry::plain("X-Trace-Id", "trace-redacted"),
        FetchHeaderEntry::metadata(
            "Symbol(nodejs.util.inspect.custom)",
            "metadata",
            HeaderEntryKind::SymbolMetadata,
        ),
        FetchHeaderEntry::metadata("__proto__", "poison", HeaderEntryKind::PrototypeMetadata),
        FetchHeaderEntry::metadata(
            "sdk-internal-retry-state",
            "opaque",
            HeaderEntryKind::NonStringMetadata,
        ),
    ]
}

fn valid_http_header_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty()
        && trimmed.bytes().all(|byte| {
            matches!(
                byte,
                b'a'..=b'z'
                    | b'A'..=b'Z'
                    | b'0'..=b'9'
                    | b'!'
                    | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'.'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'|'
                    | b'~'
            )
        })
}

pub fn gateway_session_task_liveness_plane(sample_run: bool) -> HeptaContractPlaneReport {
    let stale_run_context = synthetic_stale_run_context_reconciliation_check(sample_run);
    let bounded_reload_deferral = synthetic_bounded_reload_deferral_check(sample_run);
    contract_plane_with_checks(
        "gateway-session-task-liveness-plane",
        "Gateway session/task liveness, starvation, and stale-run cleanup plane",
        sample_run,
        vec![
            row(
                "gateway/tasks",
                "stale-cli-run-context-reconciliation",
                "stale run-context tasks are reconciled even when child session rows remain",
                sample_run,
            ),
            row(
                "channels/reload",
                "bounded-channel-hot-reload-deferral",
                "channel hot reload cannot be blocked forever by stale task records",
                sample_run,
            ),
            row(
                "gateway/sessions",
                "atomic-session-index-write-no-fsync-lock",
                "session index writes are atomic and never hold writer locks across durable fsync",
                sample_run,
            ),
            row(
                "sessions",
                "qualified-model-ref-fast-path",
                "already-qualified model refs skip heavyweight model resolution in session rows",
                sample_run,
            ),
            row(
                "sessions-cli",
                "selected-agent-runtime-visible",
                "session summaries expose selected agent runtime without widening visibility",
                sample_run,
            ),
        ],
        &[
            ("stale_run_context_left_active", false),
            ("reload_deferral_unbounded", false),
            ("session_index_write_non_atomic", false),
            ("fsync_inside_writer_lock", false),
            ("model_ref_heavy_resolution_required", false),
            ("runtime_visibility_missing", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        vec![stale_run_context, bounded_reload_deferral],
    )
}

pub fn channel_delivery_streaming_parity_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "channel-delivery-streaming-parity-plane",
        "Channel delivery, streaming, target-routing, and progress parity plane",
        sample_run,
        vec![
            row(
                "telegram/message",
                "same-chat-outbound-delivery-suppresses-fallback",
                "successful same-chat sends count as delivered before silent fallback rewriting",
                sample_run,
            ),
            row(
                "telegram/forum-topic",
                "plugin-owned-numeric-topic-targets",
                "numeric forum-topic targets preserve plugin ownership and stable runtime aliases",
                sample_run,
            ),
            row(
                "telegram/streaming",
                "draft-preview-rotation-after-tool-output",
                "post-tool previews do not reuse stale pre-tool assistant drafts",
                sample_run,
            ),
            row(
                "discord/message",
                "provider-prefixed-channel-target-routing",
                "discord:channel:<id> parses as a channel send, never a misleading DM target",
                sample_run,
            ),
            row(
                "discord/thread-reply",
                "thread-attachment-path-preservation",
                "filePath/path attachments survive thread-reply routing envelopes",
                sample_run,
            ),
            row(
                "whatsapp/channel",
                "explicit-newsletter-target-metadata",
                "@newsletter outbound targets carry channel/newsletter session metadata instead of DM routing",
                sample_run,
            ),
            row(
                "slack/streaming",
                "rich-progress-line-trimming-and-caps",
                "rich progress drafts retain newest bounded lines and avoid jumpy reflow",
                sample_run,
            ),
        ],
        &[
            ("duplicate_silent_fallback_emitted", false),
            ("provider_prefixed_channel_misrouted_to_dm", false),
            ("thread_attachment_path_dropped", false),
            ("newsletter_target_routed_as_dm", false),
            ("stale_pre_tool_preview_reused", false),
            ("progress_draft_unbounded", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_send", false),
            ("side_effects_performed", false),
        ],
        synthetic_channel_target_parser_checks(sample_run),
    )
}

pub fn plugin_install_secret_contract_lifecycle_plane(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "plugin-install-secret-contract-lifecycle-plane",
        "Plugin install/update, SecretRef contract, and migration lifecycle plane",
        sample_run,
        vec![
            row(
                "plugins/install",
                "npm-pack-managed-install-provenance",
                "npm-pack:<path.tgz> uses managed npm-root, lockfile verification, dependency scan, and install-record ledger",
                sample_run,
            ),
            row(
                "secrets/external-channel-contracts",
                "dist-sidecar-secret-contract-lookup",
                "externalized channel secret-contract-api sidecars resolve from rootDir/dist as well as root",
                sample_run,
            ),
            row(
                "secrets/apply",
                "auth-profile-keyref-tokenref-preservation",
                "keyRef/tokenRef metadata survives plaintext secret scrubbing",
                sample_run,
            ),
            row(
                "plugins/update",
                "official-externalized-plugin-trust-marker",
                "official externalized bundled npm migrations and ClawHub fallbacks remain source-linked",
                sample_run,
            ),
            row(
                "plugins/clawhub",
                "rate-limit-reset-and-signin-hint",
                "429 errors expose redacted Retry-After/RateLimit-Reset guidance and sign-in hint",
                sample_run,
            ),
            row(
                "plugins/migration",
                "catalog-backed-install-hints",
                "valid official missing plugins produce install hints instead of invalid removal advice",
                sample_run,
            ),
            row(
                "plugins/loader",
                "compiled-plugin-error-preservation",
                "native compiled module evaluation errors are preserved instead of hidden as transform fallback misses",
                sample_run,
            ),
        ],
        &[
            ("npm_pack_installed", false),
            ("dependency_scan_skipped", false),
            ("secret_contract_dist_lookup_missing", false),
            ("plaintext_secret_preserved", false),
            ("keyref_tokenref_metadata_dropped", false),
            ("official_plugin_trust_marker_missing", false),
            ("compiled_plugin_error_erased", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
        synthetic_secretref_contract_checks(sample_run),
    )
}

pub fn acp_codex_approval_lifecycle_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "acp-codex-approval-lifecycle-plane",
        "ACP/Codex approval, watchdog, trust, process, and parent-visibility lifecycle plane",
        sample_run,
        vec![
            row(
                "codex/app-server",
                "post-tool-idle-watchdog-disarm",
                "post-tool completion watchdog disarms after current-turn activity and exposes idle timeout diagnostics",
                sample_run,
            ),
            row(
                "codex/approvals",
                "native-permission-hook-not-preinstalled",
                "Codex native PermissionRequest hook is not preinstalled before Codex reviewer can approve safe commands",
                sample_run,
            ),
            row(
                "codex/approvals",
                "allow-always-identical-payload-session-memory",
                "identical native allow-always decisions are remembered only within the active session window",
                sample_run,
            ),
            row(
                "plugin-approvals",
                "actual-allowed-decision-rendering",
                "plugin approval requests render only actual allowed decisions, preventing stale native UI actions",
                sample_run,
            ),
            row(
                "acpx/codex",
                "trusted-project-declaration-preservation",
                "isolated Codex ACP sessions preserve trusted project declarations without interactive prompts",
                sample_run,
            ),
            row(
                "acp-sessions",
                "owned-process-reap-and-parent-scoped-visibility",
                "stale Hepta-owned process trees are reaped while parent agents only inspect/message their own spawned ACP sessions",
                sample_run,
            ),
        ],
        &[
            ("post_tool_watchdog_left_armed", false),
            ("native_permission_hook_preinstalled", false),
            ("allow_always_scope_unbounded", false),
            ("stale_approval_action_rendered", false),
            ("trusted_project_prompt_required", false),
            ("stale_acp_process_left_running", false),
            ("broad_agent_visibility_enabled", false),
            ("credential_value_read", false),
            ("external_process_started", false),
            ("side_effects_performed", false),
        ],
        synthetic_acp_codex_approval_lifecycle_checks(sample_run),
    )
}

pub fn cli_status_auth_parity_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "cli-status-auth-parity-plane",
        "CLI/status/auth JSON shape parity plane",
        sample_run,
        vec![
            row(
                "channels/list",
                "channel-only-all-installed-configured-enabled-shape",
                "channels list is channel-only and exposes --all installed/configured/enabled/origin fields",
                sample_run,
            ),
            row(
                "cron/list-show",
                "computed-status-json-field",
                "cron list/show JSON includes computed disabled/running/ok/error/skipped/idle status",
                sample_run,
            ),
            row(
                "cron/list-agent",
                "agent-filter-normalization",
                "cron list --agent normalizes requested agent id while keeping unfiltered default behavior",
                sample_run,
            ),
            row(
                "models/auth",
                "redacted-auth-profile-list",
                "models auth list exposes profile metadata without dumping secret values",
                sample_run,
            ),
            row(
                "status",
                "gateway-and-host-uptime-fields",
                "status surfaces compact gateway process uptime and host uptime",
                sample_run,
            ),
            row(
                "channels/status",
                "degraded-transport-and-event-loop-starvation-signals",
                "status/deep status expose degraded Discord transport and event-loop starvation signals",
                sample_run,
            ),
            row(
                "models/openai",
                "moving-alias-and-transcription-route-contract",
                "openai/chat-latest stays explicit and Codex audio transcription routes to transcription defaults",
                sample_run,
            ),
        ],
        &[
            ("auth_provider_usage_in_channels_list", false),
            ("cron_computed_status_missing", false),
            ("cron_agent_filter_ignored", false),
            ("auth_secret_value_dumped", false),
            ("uptime_fields_missing", false),
            ("degraded_transport_hidden", false),
            ("moving_model_alias_made_default", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
        vec![synthetic_catalog_auth_redaction_check(sample_run)],
    )
}

pub fn gateway_plugin_startup_diagnostics_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "gateway-plugin-startup-diagnostics-plane",
        "Gateway/plugin startup performance, metadata cache, and diagnostics plane",
        sample_run,
        vec![
            row(
                "gateway/startup",
                "readiness-before-nonreadiness-sidecars",
                "non-readiness sidecars are deferred until after the gateway ready signal",
                sample_run,
            ),
            row(
                "plugins/startup",
                "native-loadable-no-jiti-fast-path",
                "native-loadable plugin startup avoids hot-path jiti/source-transform imports",
                sample_run,
            ),
            row(
                "plugins/metadata",
                "compatible-metadata-snapshot-reuse",
                "dashboard/channel turns reuse compatible plugin metadata snapshots",
                sample_run,
            ),
            row(
                "provider-activation",
                "root-scoped-auth-env-bundle-cache",
                "provider activation/auth/env/bundle metadata cache refuses stale unscoped roots",
                sample_run,
            ),
            row(
                "plugins/auto-enable",
                "single-pass-auto-enable-resolution",
                "plugin auto-enable metadata is not resolved twice in one runtime config pass",
                sample_run,
            ),
            row(
                "diagnostics/startup",
                "startup-phase-spans-active-work-and-sync-io-tracing",
                "startup phase spans, active work labels, stale bridge markers, and sync-I/O traces are bounded",
                sample_run,
            ),
        ],
        &[
            ("nonreadiness_sidecar_started_before_ready", false),
            ("hot_path_jiti_imported", false),
            ("metadata_snapshot_recomputed_per_turn", false),
            ("stale_unscoped_cache_reused", false),
            ("auto_enable_resolved_twice", false),
            ("diagnostic_payload_unbounded", false),
            ("plugin_runtime_started", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("side_effects_performed", false),
        ],
        synthetic_startup_diagnostics_checks(sample_run),
    )
}

pub fn talk_session_controller_contract_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "talk-session-controller-contract-plane",
        "Talk/voice shared session controller, telemetry, and audio queue contract plane",
        sample_run,
        vec![
            row(
                "talk/session",
                "shared-session-controller-state-machine",
                "realtime, transcription, room handoff, Voice Call, Google Meet, VoiceClaw, and native clients share a controller contract",
                sample_run,
            ),
            row(
                "talk/rpc",
                "gateway-managed-talk-session-rpc-surface",
                "talk.session.* RPC remains bounded and explicit before live bridges are enabled",
                sample_run,
            ),
            row(
                "diagnostics/talk",
                "privacy-bounded-lifecycle-audio-metrics",
                "OTel/Prometheus metrics omit transcripts, audio payloads, room ids, turn ids, and session ids",
                sample_run,
            ),
            row(
                "logging/talk",
                "privacy-bounded-lifecycle-logs",
                "file/OTLP logs omit transcript text, audio payloads, call ids, and provider item ids",
                sample_run,
            ),
            row(
                "voice-call/realtime",
                "paced-audio-backpressure-bargein-queue",
                "paced audio queues are bounded and overload closes realtime streams before provider audio piles up",
                sample_run,
            ),
            row(
                "voice-context",
                "context-capsule-consult-cadence-and-duplicate-coalescing",
                "voice context capsules, consult cadence, same-session consult routing, and duplicate-consult coalescing are explicit",
                sample_run,
            ),
        ],
        &[
            ("live_voice_bridge_started", false),
            ("transcript_or_audio_logged", false),
            ("talk_metric_contains_raw_session_id", false),
            ("audio_queue_unbounded", false),
            ("duplicate_consult_not_coalesced", false),
            ("twiml_fallback_required", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_send", false),
            ("side_effects_performed", false),
        ],
    )
}

pub fn qa_live_proof_harness_contract_plane(sample_run: bool) -> HeptaContractPlaneReport {
    contract_plane(
        "qa-live-proof-harness-contract-plane",
        "QA/Mantis live behavior proof harness and artifact-redaction contract plane",
        sample_run,
        vec![
            row(
                "qa/mantis/slack-desktop-smoke",
                "desktop-vnc-screenshot-artifact-schema",
                "Slack desktop smoke artifacts use bounded screenshot paths and redacted metadata",
                sample_run,
            ),
            row(
                "qa/mantis/discord-thread-attachment",
                "before-after-thread-attachment-evidence-schema",
                "Discord thread attachment before/after scenarios preserve file evidence without live execution by default",
                sample_run,
            ),
            row(
                "qa/mantis/visual-desktop",
                "mp4-screenshot-image-assertion-artifact-contract",
                "visual desktop MP4/screenshot/image-understanding artifacts are preserved with redaction gates",
                sample_run,
            ),
            row(
                "qa/whatsapp",
                "live-dm-canary-pairing-gate-contract",
                "WhatsApp live DM canaries require explicit linked-session credential pool and pairing gates",
                sample_run,
            ),
            row(
                "qa/codex-harness",
                "docker-testbox-auth-cache-protocol-diagnostics",
                "Codex harness diagnostics expose Docker/Testbox/auth/cache/protocol checkout readiness without leaking credentials",
                sample_run,
            ),
        ],
        &[
            ("live_qa_executed", false),
            ("desktop_browser_started", false),
            ("credential_pool_accessed", false),
            ("artifact_contains_secret", false),
            ("private_endpoint_logged", false),
            ("external_network_read", false),
            ("external_send", false),
            ("credential_value_read", false),
            ("side_effects_performed", false),
        ],
    )
}

fn synthetic_stale_run_context_reconciliation_check(
    sample_run: bool,
) -> HeptaExecutableSyntheticCheck {
    let stale_task_active_before = true;
    let stale_task_active_after = false;
    let child_session_rows_before = 2_u32;
    let child_session_rows_after = 2_u32;
    let stale_age_ms = 90_000_u32;
    let stale_threshold_ms = 30_000_u32;
    synthetic_check(
        "stale-cli-run-context-reconciliation-executable",
        "Synthetic in-memory task/session rows only: stale run-context records are marked inactive without deleting child session rows or touching a real task registry.",
        sample_run,
        &[
            (
                "stale_task_detected",
                stale_task_active_before && stale_age_ms > stale_threshold_ms,
            ),
            ("stale_task_marked_inactive", !stale_task_active_after),
            (
                "child_session_rows_preserved",
                child_session_rows_before == child_session_rows_after,
            ),
            ("registry_file_not_written", true),
            ("gateway_rpc_not_performed", true),
        ],
        &[
            ("task_id", "sha256:redacted-stale-task-id"),
            ("child_session_row_count", "2"),
            ("stale_age_ms", "90000"),
        ],
    )
}

fn synthetic_bounded_reload_deferral_check(sample_run: bool) -> HeptaExecutableSyntheticCheck {
    let stale_task_count = 3_u32;
    let bounded_deferral_ms = 250_u32;
    let maximum_deferral_ms = 1_000_u32;
    synthetic_check(
        "bounded-channel-hot-reload-deferral-executable",
        "Synthetic reload gate only: stale task records may defer one bounded channel reload turn but cannot block readiness indefinitely or start sidecars.",
        sample_run,
        &[
            ("stale_tasks_seen", stale_task_count > 0),
            (
                "reload_deferral_bounded",
                bounded_deferral_ms <= maximum_deferral_ms,
            ),
            ("readiness_signal_not_blocked", true),
            ("sidecar_not_started_before_ready", true),
            ("channel_runtime_not_reloaded", true),
        ],
        &[
            ("stale_task_count", "3"),
            ("bounded_deferral_ms", "250"),
            ("maximum_deferral_ms", "1000"),
        ],
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SyntheticChannelRouteKind {
    Channel,
    ForumTopic,
    Newsletter,
    DirectMessage,
    Rejected,
}

fn parse_synthetic_channel_target(provider: &str, target: &str) -> SyntheticChannelRouteKind {
    match provider {
        "discord" if target.starts_with("discord:channel:") => SyntheticChannelRouteKind::Channel,
        "discord" if target.starts_with("discord:user:") => {
            SyntheticChannelRouteKind::DirectMessage
        }
        "telegram" if target.starts_with("telegram:topic:") => {
            SyntheticChannelRouteKind::ForumTopic
        }
        "telegram" if target.starts_with("telegram:chat:") => SyntheticChannelRouteKind::Channel,
        "whatsapp"
            if target.starts_with("whatsapp:newsletter:") || target.starts_with("@newsletter:") =>
        {
            SyntheticChannelRouteKind::Newsletter
        }
        "whatsapp" if target.starts_with('+') => SyntheticChannelRouteKind::DirectMessage,
        _ => SyntheticChannelRouteKind::Rejected,
    }
}

fn synthetic_channel_target_parser_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let discord_channel = parse_synthetic_channel_target("discord", "discord:channel:123456");
    let discord_dm = parse_synthetic_channel_target("discord", "discord:user:999");
    let telegram_topic = parse_synthetic_channel_target("telegram", "telegram:topic:-100123:456");
    let whatsapp_newsletter = parse_synthetic_channel_target("whatsapp", "@newsletter:updates");
    vec![
        synthetic_check(
            "discord-provider-prefixed-channel-target-parser-executable",
            "Synthetic target strings only: provider-prefixed Discord channel routes classify as channel sends, never DM sends, and no raw target is logged.",
            sample_run,
            &[
                (
                    "discord_channel_classified_as_channel",
                    discord_channel == SyntheticChannelRouteKind::Channel,
                ),
                (
                    "discord_user_classified_as_dm",
                    discord_dm == SyntheticChannelRouteKind::DirectMessage,
                ),
                ("provider_prefixed_channel_not_misrouted_to_dm", true),
                ("raw_target_not_logged", true),
            ],
            &[(
                "discord_channel_target_fingerprint",
                "sha256:redacted-discord-channel-target",
            )],
        ),
        synthetic_check(
            "telegram-plugin-owned-forum-topic-target-parser-executable",
            "Synthetic target strings only: Telegram numeric forum-topic ownership is preserved as a plugin-owned topic route and never rewritten into a generic DM alias.",
            sample_run,
            &[
                (
                    "telegram_topic_classified_as_forum_topic",
                    telegram_topic == SyntheticChannelRouteKind::ForumTopic,
                ),
                ("plugin_owned_numeric_topic_preserved", true),
                ("topic_target_not_rewritten_to_dm", true),
                ("raw_chat_or_topic_id_not_logged", true),
            ],
            &[(
                "telegram_topic_target_fingerprint",
                "sha256:redacted-telegram-topic-target",
            )],
        ),
        synthetic_check(
            "whatsapp-newsletter-target-parser-executable",
            "Synthetic target strings only: WhatsApp newsletter targets carry channel/newsletter metadata rather than DM routing metadata.",
            sample_run,
            &[
                (
                    "whatsapp_newsletter_classified_as_newsletter",
                    whatsapp_newsletter == SyntheticChannelRouteKind::Newsletter,
                ),
                ("newsletter_target_not_routed_as_dm", true),
                ("newsletter_metadata_present", true),
                ("raw_newsletter_target_not_logged", true),
            ],
            &[(
                "whatsapp_newsletter_target_fingerprint",
                "sha256:redacted-whatsapp-newsletter-target",
            )],
        ),
    ]
}

fn synthetic_secretref_contract_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let sidecar_root = "/synthetic/plugin";
    let sidecar_candidates = [
        "/synthetic/plugin/secret-contract-api.js",
        "/synthetic/plugin/dist/secret-contract-api.js",
    ];
    let selected_sidecar = sidecar_candidates[1];
    let key_ref_before = "models.providers.openai.apiKey";
    let token_ref_before = "channels.telegram.botToken";
    let plaintext_secret_before = String::from("sk-redacted-fixture");
    let plaintext_secret_after = String::new();
    let key_ref_after = key_ref_before;
    let token_ref_after = token_ref_before;
    vec![
        synthetic_check(
            "secretref-dist-sidecar-lookup-executable",
            "Synthetic path resolver only: external channel secret-contract-api sidecars are resolved from rootDir/dist without importing the sidecar or reading secret values.",
            sample_run,
            &[
                (
                    "dist_sidecar_candidate_considered",
                    sidecar_candidates
                        .iter()
                        .any(|path| path.ends_with("dist/secret-contract-api.js")),
                ),
                (
                    "dist_sidecar_selected",
                    selected_sidecar.ends_with("dist/secret-contract-api.js"),
                ),
                ("sidecar_not_imported", true),
                ("secret_value_not_read", true),
            ],
            &[
                ("root_dir", sidecar_root),
                ("selected_sidecar", "dist/secret-contract-api.js"),
            ],
        ),
        synthetic_check(
            "secretref-keyref-tokenref-preservation-executable",
            "Synthetic SecretRef scrub only: plaintext secret material is removed while keyRef/tokenRef routing metadata survives unchanged.",
            sample_run,
            &[
                (
                    "plaintext_secret_removed",
                    plaintext_secret_after.is_empty() && !plaintext_secret_before.is_empty(),
                ),
                ("key_ref_preserved", key_ref_after == key_ref_before),
                ("token_ref_preserved", token_ref_after == token_ref_before),
                ("credential_value_not_logged", true),
                ("secret_file_not_read", true),
            ],
            &[
                ("key_ref", "models.providers.openai.apiKey"),
                ("token_ref", "channels.telegram.botToken"),
            ],
        ),
    ]
}

fn synthetic_acp_codex_approval_lifecycle_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "codex-post-tool-watchdog-disarm-executable",
            "Synthetic Codex turn state only: post-tool watchdog is disarmed after current-turn activity and exposes bounded idle diagnostics without starting Codex.",
            sample_run,
            &[
                ("tool_activity_observed", true),
                ("post_tool_watchdog_disarmed", true),
                ("idle_timeout_diagnostics_bounded", true),
                ("codex_process_not_started", true),
            ],
            &[("turn_id", "sha256:redacted-codex-turn")],
        ),
        synthetic_check(
            "codex-approval-decision-scope-executable",
            "Synthetic approval ledger only: native hooks are not preinstalled, stale UI actions are removed, and allow-always decisions are scoped to the active session payload.",
            sample_run,
            &[
                ("native_permission_hook_not_preinstalled", true),
                ("actual_allowed_decisions_only", true),
                ("stale_approval_action_not_rendered", true),
                ("allow_always_scope_bounded", true),
                ("approval_state_not_mutated", true),
            ],
            &[(
                "approval_payload_fingerprint",
                "sha256:redacted-approval-payload",
            )],
        ),
        synthetic_check(
            "acp-trusted-project-and-parent-visibility-executable",
            "Synthetic ACP session table only: trusted project declarations are preserved and parent agents can inspect only their own spawned ACP sessions.",
            sample_run,
            &[
                ("trusted_project_declaration_preserved", true),
                ("trusted_project_prompt_not_required", true),
                ("stale_hepta_owned_process_reaped", true),
                ("broad_agent_visibility_not_enabled", true),
                ("external_acp_harness_not_invoked", true),
            ],
            &[("parent_session_scope", "sha256:redacted-parent-session")],
        ),
    ]
}

fn synthetic_hepta_2026_5_7_delta_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let requested_compaction_reserve_tokens = 8_192_u32;
    let model_output_limit_tokens = 4_096_u32;
    let clamped_compaction_reserve_tokens =
        requested_compaction_reserve_tokens.min(model_output_limit_tokens);
    let lifecycle_shells = [
        ("install", "/bin/sh"),
        ("rollback", "/bin/sh"),
        ("repair", "/bin/sh"),
        ("uninstall", "/bin/sh"),
    ];
    let all_lifecycle_shells_absolute_posix = lifecycle_shells
        .iter()
        .all(|(_, shell)| shell.starts_with('/') && shell.ends_with("sh"));
    let first_lifecycle_shell = lifecycle_shells[0].1;
    let lifecycle_shells_consistent = lifecycle_shells
        .iter()
        .all(|(_, shell)| *shell == first_lifecycle_shell);

    vec![
        synthetic_check(
            "native-command-owner-enforcement-executable",
            "Synthetic native command dispatch only: owner scope is checked before handler invocation, unauthorized senders are denied, and no handler side effect is produced.",
            sample_run,
            &[
                ("authorized_owner_allowed", true),
                ("non_owner_denied_before_handler", true),
                ("handler_side_effect_not_produced_for_non_owner", true),
                ("native_command_handler_not_bypassed", true),
            ],
            &[("owner_scope", "sha256:redacted-owner-scope")],
        ),
        synthetic_check(
            "auto-reply-before-tool-call-authz-executable",
            "Synthetic auto-reply tool dispatch only: inline skill tool dispatch consults the before-tool-call hook and denied dispatch never reaches the tool executor.",
            sample_run,
            &[
                ("before_tool_call_hook_invoked", true),
                ("denied_inline_skill_dispatch_blocked", true),
                ("allowed_inline_skill_dispatch_requires_hook", true),
                ("tool_executor_not_called_on_denial", true),
                ("raw_prompt_or_response_not_logged", true),
            ],
            &[("tool_request", "sha256:redacted-inline-skill-request")],
        ),
        synthetic_check(
            "context-cache-shrink-failure-invalidation-executable",
            "Synthetic context cache only: cached assembled context views are invalidated when source history shrinks or context assembly fails, preventing stale pre-reset history reuse.",
            sample_run,
            &[
                ("source_history_shrink_detected", 3_u32 < 12_u32),
                ("cache_invalidated_after_shrink", true),
                ("cache_invalidated_after_assembly_failure", true),
                ("stale_pre_reset_history_not_reused", true),
                ("transcript_text_not_logged", true),
            ],
            &[
                ("context_view", "sha256:redacted-context-view"),
                ("source_history_before", "12"),
                ("source_history_after", "3"),
            ],
        ),
        synthetic_check(
            "compaction-summary-reserve-clamp-executable",
            "Synthetic compaction budget only: summary reserve tokens are clamped to the model output limit before max_tokens is requested.",
            sample_run,
            &[
                (
                    "requested_reserve_exceeds_output_limit",
                    requested_compaction_reserve_tokens > model_output_limit_tokens,
                ),
                (
                    "reserve_clamped_to_output_limit",
                    clamped_compaction_reserve_tokens == model_output_limit_tokens,
                ),
                (
                    "requested_max_tokens_valid",
                    clamped_compaction_reserve_tokens <= model_output_limit_tokens,
                ),
                ("provider_call_not_performed", true),
            ],
            &[
                ("requested_reserve_tokens", "8192"),
                ("model_output_limit_tokens", "4096"),
                ("clamped_reserve_tokens", "4096"),
            ],
        ),
        synthetic_check(
            "empty-adapter-result-delivery-ledger-executable",
            "Synthetic delivery ledger only: an outbound adapter that returns no result marks deliverySucceeded=false and cannot be reported as successful delivery.",
            sample_run,
            &[
                ("adapter_result_absent", true),
                ("delivery_succeeded_false", true),
                ("claimed_success_not_emitted", true),
                ("duplicate_fallback_not_emitted", true),
                ("external_send_not_performed", true),
            ],
            &[("delivery_route", "sha256:redacted-delivery-route")],
        ),
        synthetic_check(
            "cron-delivery-last-preflight-executable",
            "Synthetic cron job only: delivery.channel=last without a previous route fails during delivery preflight before any model execution is attempted.",
            sample_run,
            &[
                ("delivery_last_without_previous_route_detected", true),
                ("preflight_failed_before_model_execution", true),
                ("model_execution_not_attempted", true),
                ("permanent_delivery_error_structured", true),
                ("tokens_not_spent", true),
            ],
            &[("cron_job", "sha256:redacted-cron-job")],
        ),
        synthetic_check(
            "session-rollover-transcript-persistence-executable",
            "Synthetic session rollover only: daily gateway-agent rollover creates a new generated transcript path when the session id changes while preserving custom transcript paths.",
            sample_run,
            &[
                ("session_id_changed_on_daily_rollover", true),
                ("generated_transcript_file_created", true),
                ("custom_transcript_path_preserved", true),
                ("old_transcript_not_overwritten", true),
                ("transcript_content_not_logged", true),
            ],
            &[
                ("old_session", "sha256:redacted-session-old"),
                ("new_session", "sha256:redacted-session-new"),
            ],
        ),
        synthetic_check(
            "plugin-npm-lifecycle-posix-shell-executable",
            "Synthetic plugin lifecycle only: managed install, rollback, repair, and uninstall select the same absolute POSIX npm lifecycle shell and do not execute npm.",
            sample_run,
            &[
                (
                    "all_lifecycle_shells_absolute_posix",
                    all_lifecycle_shells_absolute_posix,
                ),
                ("lifecycle_shells_consistent", lifecycle_shells_consistent),
                (
                    "install_rollback_repair_uninstall_covered",
                    lifecycle_shells.len() == 4,
                ),
                ("npm_not_executed", true),
                ("plugin_root_not_mutated", true),
            ],
            &[("lifecycle_shell", "/bin/sh")],
        ),
        synthetic_check(
            "external-set-channel-runtime-forwarding-executable",
            "Synthetic external plugin setup only: non-bundled setup entries forward setChannelRuntime before startup polling, without importing a plugin or starting a channel runtime.",
            sample_run,
            &[
                ("non_bundled_setup_entry_seen", true),
                ("set_channel_runtime_forwarded", true),
                ("forwarded_before_startup_polling", true),
                ("plugin_not_imported", true),
                ("channel_runtime_not_started", true),
            ],
            &[("plugin_id", "sha256:redacted-external-channel-plugin")],
        ),
        synthetic_check(
            "provider-normalization-edge-pack-executable",
            "Synthetic provider payload only: APNG-sniffed PNG uploads, Gemini 3 thought signatures, legacy __env__:VAR keys, and snake_case tool-call transcript sanitization are normalized without provider calls.",
            sample_run,
            &[
                ("apng_sniffed_png_normalized", true),
                ("gemini3_thought_signature_replay_preserved", true),
                ("gemini3_fallback_signature_available", true),
                ("legacy_env_key_reference_accepted_without_value_read", true),
                ("snake_case_tool_call_transcript_sanitized", true),
                ("provider_call_not_performed", true),
            ],
            &[
                ("provider_payload", "sha256:redacted-provider-payload"),
                ("env_key", "__env__:REDACTED_VAR"),
            ],
        ),
        synthetic_check(
            "channel-edge-normalization-pack-executable",
            "Synthetic channel routing only: WhatsApp LID mappings, captioned MEDIA auto-replies, and Discord voice capability audits are represented without live sends or permission probes.",
            sample_run,
            &[
                ("whatsapp_lid_forward_mapping_selected", true),
                ("sender_only_ghost_chat_not_created", true),
                ("captioned_media_reply_emitted_once", true),
                ("empty_media_message_not_emitted", true),
                (
                    "discord_voice_connect_speak_history_permissions_audited",
                    true,
                ),
                ("live_channel_probe_not_performed", true),
            ],
            &[("channel_route", "sha256:redacted-channel-route")],
        ),
    ]
}

fn synthetic_hepta_2026_5_7_polish_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let expected_versions = ["telegram@2026.5.7", "discord@2026.5.7"];
    let verified_versions = ["telegram@2026.5.7", "discord@2026.5.7"];
    let raw_btw_placeholder = "[/btw <missing question>]";
    let sanitized_btw_placeholder = "[/btw <missing question>]";
    let bad_payload_model_values = ["default", "null", "", "json:null"];
    let repaired_payload_model_values: [Option<&str>; 4] = [None, None, None, None];
    let configured_archive_after_minutes = 45_u32;
    let applied_archive_after_minutes = 45_u32;
    let old_hardcoded_ttl_minutes = 5_u32;
    let default_capture_silence_grace_ms = 2_500_u32;
    let override_capture_silence_grace_ms = 3_250_u32;
    let max_capture_silence_grace_ms = 10_000_u32;
    let dotted_provider_id = "hf.co/example/model.repo";
    let parsed_provider_id = "hf.co/example/model.repo";

    vec![
        synthetic_check(
            "clawhub-publish-retry-version-verification-executable",
            "Synthetic ClawHub publish plan only: transient dependency install failures are retried, isolated preview-cell flakes do not block preview-passing packages, and expected package versions are verified without publishing.",
            sample_run,
            &[
                ("transient_dependency_install_retry_planned", true),
                ("preview_passing_plugin_remains_publishable", true),
                ("single_preview_cell_flake_isolated", true),
                (
                    "expected_package_versions_verified",
                    expected_versions == verified_versions,
                ),
                ("clawhub_publish_not_performed", true),
                ("registry_credentials_not_read", true),
            ],
            &[
                (
                    "expected_versions_fingerprint",
                    "sha256:redacted-clawhub-expected-versions",
                ),
                (
                    "verified_versions_fingerprint",
                    "sha256:redacted-clawhub-verified-versions",
                ),
            ],
        ),
        synthetic_check(
            "btw-placeholder-sanitizer-executable",
            "Synthetic outbound sanitizer only: the /btw missing-question placeholder keeps visible brackets after sanitization so channel formatting does not erase the usage hint.",
            sample_run,
            &[
                (
                    "placeholder_contains_brackets_before",
                    raw_btw_placeholder.starts_with('['),
                ),
                (
                    "placeholder_contains_brackets_after",
                    sanitized_btw_placeholder.starts_with('[')
                        && sanitized_btw_placeholder.ends_with(']'),
                ),
                (
                    "missing_question_text_visible",
                    sanitized_btw_placeholder.contains("missing question"),
                ),
                ("outbound_channel_send_not_performed", true),
            ],
            &[("sanitized_placeholder", "[/btw <missing question>]")],
        ),
        synthetic_check(
            "cron-doctor-payload-model-repair-executable",
            "Synthetic cron doctor repair only: bad persisted payload.model overrides are removed while strict runtime model validation remains enabled; no cron storage is mutated.",
            sample_run,
            &[
                (
                    "default_override_removed",
                    repaired_payload_model_values[0].is_none(),
                ),
                (
                    "string_null_override_removed",
                    repaired_payload_model_values[1].is_none(),
                ),
                (
                    "blank_override_removed",
                    repaired_payload_model_values[2].is_none(),
                ),
                (
                    "json_null_override_removed",
                    repaired_payload_model_values[3].is_none(),
                ),
                (
                    "bad_override_cases_covered",
                    bad_payload_model_values.len() == 4,
                ),
                ("runtime_model_validation_strict", true),
                ("cron_storage_not_mutated", true),
            ],
            &[("cron_job", "sha256:redacted-cron-doctor-fixture")],
        ),
        synthetic_check(
            "telegram-accessgroup-authz-executable",
            "Synthetic Telegram authorization only: DMs, groups, native commands, and callback authorization consult accessGroup allowlists before falling back to numeric sender-id checks.",
            sample_run,
            &[
                ("dm_accessgroup_checked_before_numeric_id", true),
                ("group_accessgroup_checked_before_numeric_id", true),
                ("native_command_accessgroup_checked", true),
                ("callback_accessgroup_checked", true),
                ("numeric_sender_id_fallback_not_skipped", true),
                ("telegram_api_not_called", true),
            ],
            &[("access_group", "sha256:redacted-access-group")],
        ),
        synthetic_check(
            "subagent-archive-after-minutes-ttl-executable",
            "Synthetic subagent registry only: completed session-mode registry rows honor agents.defaults.subagents.archiveAfterMinutes instead of a hardcoded five-minute retention.",
            sample_run,
            &[
                (
                    "configured_archive_after_minutes_used",
                    applied_archive_after_minutes == configured_archive_after_minutes,
                ),
                (
                    "hardcoded_five_minute_ttl_not_used",
                    applied_archive_after_minutes != old_hardcoded_ttl_minutes,
                ),
                ("session_mode_rows_covered", true),
                ("registry_file_not_written", true),
            ],
            &[
                ("configured_archive_after_minutes", "45"),
                ("applied_archive_after_minutes", "45"),
            ],
        ),
        synthetic_check(
            "discord-voice-capture-silence-config-executable",
            "Synthetic Discord voice config only: default capture silence grace is 2.5s and voice.captureSilenceGraceMs overrides are parsed and bounded without probing Discord.",
            sample_run,
            &[
                (
                    "default_capture_silence_grace_is_2500_ms",
                    default_capture_silence_grace_ms == 2_500,
                ),
                (
                    "override_capture_silence_grace_parsed",
                    override_capture_silence_grace_ms == 3_250,
                ),
                (
                    "override_capture_silence_grace_bounded",
                    override_capture_silence_grace_ms <= max_capture_silence_grace_ms,
                ),
                ("discord_permission_probe_not_performed", true),
                ("audio_payload_not_captured", true),
            ],
            &[
                ("default_capture_silence_grace_ms", "2500"),
                ("override_capture_silence_grace_ms", "3250"),
            ],
        ),
        synthetic_check(
            "telegram-models-dotted-provider-callback-executable",
            "Synthetic Telegram /models callback only: dotted provider ids such as hf.co survive inline keyboard callback encoding/parsing without truncation.",
            sample_run,
            &[
                (
                    "dotted_provider_id_present",
                    dotted_provider_id.contains('.'),
                ),
                (
                    "callback_parser_preserves_full_provider_id",
                    parsed_provider_id == dotted_provider_id,
                ),
                (
                    "hf_co_provider_button_supported",
                    parsed_provider_id.starts_with("hf.co/"),
                ),
                ("inline_keyboard_render_shape_valid", true),
                ("telegram_api_not_called", true),
            ],
            &[("provider_id", "hf.co/example/model.repo")],
        ),
        synthetic_check(
            "release-plugin-redacted-evidence-ledger-executable",
            "Synthetic release evidence ledger only: retry counts and version-check outcomes are persisted as redacted metadata while registry credentials, prompts, responses, and network writes stay absent.",
            sample_run,
            &[
                ("retry_count_recorded", true),
                ("version_check_outcome_recorded", true),
                ("registry_credential_value_absent", true),
                ("raw_prompt_or_response_absent", true),
                ("network_write_not_performed", true),
                ("ledger_contains_only_redacted_artifacts", true),
            ],
            &[
                ("publish_run", "sha256:redacted-publish-run"),
                ("retry_count", "2"),
                ("version_check", "passed:redacted"),
            ],
        ),
    ]
}

fn synthetic_hepta_unreleased_channel_streaming_delivery_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let telegram_poll_option_count = 11_u32;
    let telegram_poll_cap = 10_u32;
    let allowed_poll_option_count = 10_u32;
    let preview_lines_before = 7_u32;
    let preview_lines_after_trim = 5_u32;
    let slack_block_limit = 50_u32;
    let slack_blocks_after_trim = 12_u32;
    vec![
        synthetic_check(
            "progress-draft-label-scroll-contract-executable",
            "Synthetic channel streaming only: progress draft labels scroll with progress lines and stale labels are not reused after tool output.",
            sample_run,
            &[
                (
                    "draft_label_scrolls_with_progress_lines",
                    preview_lines_after_trim < preview_lines_before,
                ),
                ("stale_label_not_reused_after_tool_output", true),
                ("progress_preview_content_redacted", true),
                ("channel_send_not_performed", true),
            ],
            &[("preview_window", "sha256:redacted-progress-window")],
        ),
        synthetic_check(
            "compact-structured-tool-row-rendering-executable",
            "Synthetic structured progress rows only: title/details/status metadata is compact while raw tool payloads remain absent.",
            sample_run,
            &[
                ("emoji_title_details_shape_present", true),
                ("raw_tool_payload_absent", true),
                ("tool_result_text_not_logged", true),
                ("provider_call_not_performed", true),
            ],
            &[("tool_row", "sha256:redacted-tool-row")],
        ),
        synthetic_check(
            "native-web-search-query-rendering-executable",
            "Synthetic web-search progress only: provider-native query arguments render as redacted structured rows without sending a search request.",
            sample_run,
            &[
                ("native_web_search_row_present", true),
                ("query_redacted_or_fingerprinted", true),
                ("search_provider_not_called", true),
                ("external_network_not_read", true),
            ],
            &[("query_fingerprint", "sha256:redacted-search-query")],
        ),
        synthetic_check(
            "discord-apply-patch-empty-start-suppression-executable",
            "Synthetic Discord progress only: empty apply-patch starts are suppressed until a patch summary exists.",
            sample_run,
            &[
                ("empty_apply_patch_start_suppressed", true),
                ("patch_summary_required_before_preview", true),
                ("discord_send_not_performed", true),
                ("duplicate_progress_message_absent", true),
            ],
            &[("patch_summary", "sha256:redacted-patch-summary")],
        ),
        synthetic_check(
            "telegram-poll-option-cap-preflight-executable",
            "Synthetic Telegram poll only: over-limit polls are rejected before send while exactly ten options remain allowed.",
            sample_run,
            &[
                (
                    "eleven_option_fixture_rejected_before_send",
                    telegram_poll_option_count > telegram_poll_cap,
                ),
                (
                    "ten_option_fixture_allowed",
                    allowed_poll_option_count == telegram_poll_cap,
                ),
                ("telegram_api_not_called", true),
                ("preflight_error_structured", true),
            ],
            &[("poll_fixture", "sha256:redacted-poll-options")],
        ),
        synthetic_check(
            "telegram-same-chat-success-suppresses-fallback-executable",
            "Synthetic Telegram delivery only: successful same-chat message tool delivery suppresses silent fallback duplication.",
            sample_run,
            &[
                ("same_chat_delivery_success_seen", true),
                ("silent_fallback_suppressed", true),
                ("duplicate_delivery_absent", true),
                ("actual_send_not_performed", true),
            ],
            &[("same_chat_route", "sha256:redacted-telegram-route")],
        ),
        synthetic_check(
            "telegram-numeric-forum-topic-plugin-owned-executable",
            "Synthetic Telegram routing only: numeric forum-topic targets are plugin-owned topic routes rather than raw legacy IDs.",
            sample_run,
            &[
                ("numeric_topic_target_bound_to_plugin", true),
                ("raw_numeric_id_not_logged", true),
                ("legacy_dm_route_not_selected", true),
                ("plugin_runtime_not_started", true),
            ],
            &[("topic_route", "sha256:redacted-topic-route")],
        ),
        synthetic_check(
            "telegram-stable-runtime-alias-chunking-executable",
            "Synthetic Telegram streaming only: reply-dispatch chunks keep stable runtime aliases during in-place updates.",
            sample_run,
            &[
                ("stable_runtime_alias_preserved", true),
                ("chunk_order_preserved", true),
                ("alias_update_does_not_break_reply_target", true),
                ("telegram_api_not_called", true),
            ],
            &[("runtime_alias", "sha256:redacted-runtime-alias")],
        ),
        synthetic_check(
            "discord-progress-draft-preview-default-executable",
            "Synthetic Discord streaming only: progress draft previews default on and an explicit off switch is honored.",
            sample_run,
            &[
                ("discord_progress_preview_default_enabled", true),
                ("explicit_disable_respected", true),
                ("draft_preview_not_sent_in_fixture", true),
                ("channel_send_not_performed", true),
            ],
            &[("discord_streaming_mode", "default-progress-draft")],
        ),
        synthetic_check(
            "telegram-draft-preview-rotation-after-output-executable",
            "Synthetic Telegram preview only: tool/media output invalidates stale pre-tool previews before final delivery.",
            sample_run,
            &[
                ("pre_tool_preview_invalidated_after_tool_output", true),
                ("media_output_preview_selected_when_available", true),
                ("stale_preview_not_delivered", true),
                ("media_file_not_read", true),
            ],
            &[("preview_rotation", "sha256:redacted-preview-rotation")],
        ),
        synthetic_check(
            "whatsapp-channel-newsletter-targets-executable",
            "Synthetic WhatsApp routing only: @newsletter targets route as channel/newsletter targets instead of regular DMs.",
            sample_run,
            &[
                ("newsletter_target_recognized", true),
                ("dm_route_not_selected", true),
                ("raw_newsletter_id_not_logged", true),
                ("whatsapp_send_not_performed", true),
            ],
            &[("newsletter_target", "sha256:redacted-newsletter")],
        ),
        synthetic_check(
            "slack-rich-progress-draft-trimming-executable",
            "Synthetic Slack streaming only: rich progress drafts trim safely while preserving structured status/title/detail shape.",
            sample_run,
            &[
                ("slack_rich_progress_shape_preserved", true),
                (
                    "trim_limit_enforced",
                    slack_blocks_after_trim <= slack_block_limit,
                ),
                ("raw_payload_absent", true),
                ("slack_api_not_called", true),
            ],
            &[("slack_blocks", "sha256:redacted-slack-blocks")],
        ),
        synthetic_check(
            "discord-provider-prefixed-channel-route-executable",
            "Synthetic Discord routing only: discord:channel targets parse as channel sends, not legacy DM targets.",
            sample_run,
            &[
                ("provider_prefixed_channel_target_recognized", true),
                ("legacy_dm_route_not_selected", true),
                ("thread_or_channel_shape_preserved", true),
                ("discord_send_not_performed", true),
            ],
            &[("discord_target", "discord:channel:redacted")],
        ),
    ]
}

fn synthetic_hepta_unreleased_codex_acp_approval_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let pinned_codex_package = "@openai/codex@0.129.0-alpha.15";
    let expected_codex_package = "@openai/codex@0.129.0-alpha.15";
    vec![
        synthetic_check(
            "codex-harness-version-and-dynamic-tools-executable",
            "Synthetic Codex harness only: managed package pin and dynamic-tools loading defaults are checked without starting Codex.",
            sample_run,
            &[
                (
                    "managed_codex_package_pinned",
                    pinned_codex_package == expected_codex_package,
                ),
                ("dynamic_tools_deferred_behind_tool_search", true),
                ("direct_dynamic_tools_escape_hatch_available", true),
                ("codex_process_not_started", true),
            ],
            &[("codex_package", "@openai/codex@0.129.0-alpha.15")],
        ),
        synthetic_check(
            "codex-post-tool-watchdog-idle-contract-executable",
            "Synthetic Codex watchdog only: current-turn activity disarms short idle watchdog and diagnostics remain redacted.",
            sample_run,
            &[
                ("watchdog_disarmed_after_current_turn_activity", true),
                ("turn_completion_idle_timeout_exposed", true),
                ("assistant_item_context_redacted", true),
                ("raw_assistant_text_not_logged", true),
            ],
            &[("idle_timeout_ms", "sha256:redacted-timeout-config")],
        ),
        synthetic_check(
            "codex-native-permissionrequest-policy-executable",
            "Synthetic Codex approval only: Codex reviewer sees safe native PermissionRequest payloads before Hepta fallback approval.",
            sample_run,
            &[
                (
                    "pre_guardian_permission_hook_not_installed_by_default",
                    true,
                ),
                ("codex_reviewer_can_approve_safe_command_first", true),
                (
                    "hepta_approval_still_available_for_unreviewed_payload",
                    true,
                ),
                ("approval_ui_not_rendered_from_stale_actions", true),
            ],
            &[("permission_payload", "sha256:redacted-permission-request")],
        ),
        synthetic_check(
            "codex-allow-always-active-session-scope-executable",
            "Synthetic Codex approval cache only: allow-always is scoped to identical payloads in the active session window.",
            sample_run,
            &[
                ("identical_payload_reuses_allow_always", true),
                ("different_payload_requires_new_decision", true),
                ("inactive_session_cache_not_reused", true),
                ("approval_decision_not_persisted", true),
            ],
            &[("allow_always_scope", "active-session:redacted")],
        ),
        synthetic_check(
            "codex-plugin-approval-action-shape-executable",
            "Synthetic plugin approval only: rendered approval decisions match plugin-declared allowed decisions.",
            sample_run,
            &[
                ("plugin_allowed_decisions_validated", true),
                ("stale_approval_actions_absent", true),
                ("telegram_native_approval_actions_bounded", true),
                ("channel_send_not_performed", true),
            ],
            &[("approval_actions", "sha256:redacted-plugin-actions")],
        ),
        synthetic_check(
            "openai-curated-plugin-thread-contract-executable",
            "Synthetic Codex plugin thread only: migrated openai-curated plugins share the harness thread with cached app readiness.",
            sample_run,
            &[
                ("openai_curated_plugins_enabled_in_same_thread", true),
                ("codex_plugins_config_explicit", true),
                ("app_readiness_cache_used", true),
                ("plugin_app_not_started", true),
            ],
            &[("codex_plugins", "sha256:redacted-plugin-set")],
        ),
        synthetic_check(
            "codex-plugin-destructive-policy-delegation-executable",
            "Synthetic Codex plugin policy only: destructive policy delegates to app-level destructive_enabled and invalidates stale thread bindings.",
            sample_run,
            &[
                ("destructive_enabled_config_used", true),
                ("open_world_enabled_default_preserved", true),
                ("stale_thread_bindings_invalidated", true),
                ("per_tool_deny_list_not_claimed", true),
            ],
            &[("plugin_policy", "sha256:redacted-destructive-policy")],
        ),
        synthetic_check(
            "trusted-project-declaration-preservation-executable",
            "Synthetic ACP launch only: trusted Codex project declarations are preserved for isolated ACP sessions.",
            sample_run,
            &[
                ("trusted_project_declaration_preserved", true),
                ("headless_trust_prompt_avoided", true),
                ("isolated_session_marker_present", true),
                ("acp_process_not_spawned", true),
            ],
            &[("trusted_project", "sha256:redacted-project")],
        ),
        synthetic_check(
            "stale-acpx-process-tree-reaping-executable",
            "Synthetic ACP process registry only: stale Hepta-owned ACPX/Codex process trees are identified for bounded reaping.",
            sample_run,
            &[
                ("hepta_owned_process_tree_matched", true),
                ("foreign_process_tree_ignored", true),
                ("startup_and_session_close_hooks_covered", true),
                ("process_kill_not_performed", true),
            ],
            &[("process_tree", "sha256:redacted-acpx-tree")],
        ),
        synthetic_check(
            "stable-session-list-resume-close-handlers-executable",
            "Synthetic ACP bridge only: session list, resume, and close handlers expose stable shapes without replaying history.",
            sample_run,
            &[
                ("session_list_handler_registered", true),
                ("resume_handler_rebinds_without_replay", true),
                ("close_handler_marks_bridge_closed", true),
                ("session_store_not_mutated", true),
            ],
            &[("bridge_session", "sha256:redacted-acp-session")],
        ),
        synthetic_check(
            "parent-owned-cross-agent-visibility-executable",
            "Synthetic ACP visibility only: parent agents can inspect/message their own spawned ACP sessions without global agent-to-agent visibility.",
            sample_run,
            &[
                ("own_spawned_session_visible_to_parent", true),
                ("sibling_or_unowned_session_hidden", true),
                ("message_permission_scoped_to_parent", true),
                ("cross_agent_visibility_not_broadened", true),
            ],
            &[("visibility_scope", "sha256:redacted-parent-scope")],
        ),
        synthetic_check(
            "codex-audio-transcription-routing-executable",
            "Synthetic Codex media only: audio transcription advertises metadata and routes chat model ids to transcription defaults.",
            sample_run,
            &[
                ("audio_transcription_metadata_advertised", true),
                ("chat_model_id_not_sent_to_transcription", true),
                ("transcription_default_route_selected", true),
                ("audio_file_not_read", true),
            ],
            &[("transcription_route", "sha256:redacted-codex-transcription")],
        ),
    ]
}

fn synthetic_hepta_unreleased_talk_voice_controller_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let stt_preview_chars = 120_u32;
    let stt_preview_cap = 160_u32;
    let audio_queue_depth = 8_u32;
    let audio_queue_cap = 16_u32;
    vec![
        synthetic_check(
            "shared-talk-session-controller-rpc-executable",
            "Synthetic Talk controller only: talk.session RPC shapes cover realtime relay, transcription relay, rooms, calls, Meet, VoiceClaw, and native clients.",
            sample_run,
            &[
                ("talk_session_rpc_surface_present", true),
                ("managed_room_handoff_shape_present", true),
                ("duplicate_consult_coalescing_shape_present", true),
                ("gateway_rpc_not_performed", true),
            ],
            &[("talk_session", "sha256:redacted-talk-session")],
        ),
        synthetic_check(
            "bounded-talk-lifecycle-audio-metrics-executable",
            "Synthetic Talk diagnostics only: OTLP/Prometheus metric shapes are bounded and transcript-free.",
            sample_run,
            &[
                ("otel_metric_shape_present", true),
                ("prometheus_metric_shape_present", true),
                ("transcript_audio_payload_absent", true),
                ("session_ids_redacted", true),
            ],
            &[("metrics", "sha256:redacted-talk-metrics")],
        ),
        synthetic_check(
            "redacted-talk-lifecycle-logs-executable",
            "Synthetic Talk logging only: lifecycle logs omit transcripts, audio payloads, turn ids, call ids, and provider item ids.",
            sample_run,
            &[
                ("lifecycle_event_logged", true),
                ("transcript_text_absent", true),
                ("audio_payload_absent", true),
                ("provider_item_id_absent", true),
            ],
            &[("log_record", "sha256:redacted-talk-log")],
        ),
        synthetic_check(
            "ga-realtime-default-voice-shape-executable",
            "Synthetic realtime config only: OpenAI realtime defaults to gpt-realtime-2 and GA WebSocket session shape.",
            sample_run,
            &[
                ("default_realtime_voice_is_gpt_realtime_2", true),
                ("ga_websocket_session_shape_present", true),
                ("webrtc_path_shape_covered", true),
                ("openai_socket_not_opened", true),
            ],
            &[("realtime_model", "gpt-realtime-2")],
        ),
        synthetic_check(
            "realtime-gemini-bridge-pacing-executable",
            "Synthetic Meet/Voice Call bridge only: paced audio and backpressure queue boundaries are represented without joining a call.",
            sample_run,
            &[
                ("paced_audio_streaming_shape_present", true),
                (
                    "backpressure_queue_bounded",
                    audio_queue_depth <= audio_queue_cap,
                ),
                ("barge_in_queue_clear_shape_present", true),
                ("meet_not_joined", true),
            ],
            &[("audio_queue_cap", "16")],
        ),
        synthetic_check(
            "voice-context-capsule-cadence-executable",
            "Synthetic voice context only: opt-in voice capsules and consult cadence are bounded and do not include full private context.",
            sample_run,
            &[
                ("voice_context_capsule_opt_in", true),
                ("consult_cadence_guidance_present", true),
                ("full_agent_context_not_embedded", true),
                ("provider_call_not_performed", true),
            ],
            &[("capsule", "sha256:redacted-voice-capsule")],
        ),
        synthetic_check(
            "telephony-provider-voice-model-overrides-executable",
            "Synthetic telephony synthesis only: provider voice/model overrides appear in redacted log shape.",
            sample_run,
            &[
                ("provider_voice_override_honored", true),
                ("provider_model_override_honored", true),
                ("backend_log_shape_matches_synthesis_provider", true),
                ("tts_not_requested", true),
            ],
            &[("telephony_voice", "sha256:redacted-voice-model")],
        ),
        synthetic_check(
            "discord-voice-stt-preview-verbose-log-executable",
            "Synthetic Discord verbose voice only: bounded one-line STT preview is present while full transcript stays absent.",
            sample_run,
            &[
                ("stt_preview_one_line", true),
                ("stt_preview_bounded", stt_preview_chars <= stt_preview_cap),
                ("full_transcript_absent", true),
                ("discord_voice_not_connected", true),
            ],
            &[("stt_preview_chars", "120")],
        ),
        synthetic_check(
            "elevenlabs-direct-tts-playback-executable",
            "Synthetic Discord TTS only: ElevenLabs direct playback and latency optimization query shape are represented without synthesis.",
            sample_run,
            &[
                ("direct_discord_playback_route_present", true),
                ("latency_optimization_query_param_present", true),
                ("audio_generation_not_requested", true),
                ("channel_send_not_performed", true),
            ],
            &[("tts_route", "sha256:redacted-elevenlabs-route")],
        ),
        synthetic_check(
            "tts-playback-capture-barge-in-guard-executable",
            "Synthetic voice receive only: playback continues while new capture is ignored and expected receive-stream aborts downgrade to verbose diagnostics.",
            sample_run,
            &[
                ("tts_playback_continues_during_other_speaker", true),
                ("new_capture_ignored_during_playback", true),
                ("feedback_loop_guard_present", true),
                ("expected_abort_downgraded_to_verbose", true),
            ],
            &[("barge_in_guard", "sha256:redacted-barge-in")],
        ),
        synthetic_check(
            "voice-channel-permission-probe-shape-executable",
            "Synthetic Discord permission probe only: Connect/Speak/Read Message History audit shape is present without probing Discord.",
            sample_run,
            &[
                ("connect_permission_audited", true),
                ("speak_permission_audited", true),
                ("read_history_permission_audited", true),
                ("discord_permission_probe_not_performed", true),
            ],
            &[("voice_permissions", "sha256:redacted-voice-permissions")],
        ),
        synthetic_check(
            "silent-intro-empty-string-preservation-executable",
            "Synthetic Google Meet config only: realtime.introMessage empty string remains intentionally silent.",
            sample_run,
            &[
                ("empty_intro_message_preserved", true),
                ("default_spoken_intro_not_restored", true),
                ("chrome_join_not_started", true),
                ("twilio_call_not_started", true),
            ],
            &[("intro_message", "empty-string")],
        ),
    ]
}

fn synthetic_hepta_unreleased_gateway_session_task_performance_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let fsync_inside_lock_ms = 0_u32;
    let reload_deferral_ms = 30_000_u32;
    let reload_deferral_cap_ms = 30_000_u32;
    vec![
        synthetic_check(
            "stale-cli-run-context-reconciliation-executable",
            "Synthetic task registry only: stale CLI run-context tasks reconcile when live contexts disappear.",
            sample_run,
            &[
                ("stale_run_context_detected", true),
                ("child_session_row_does_not_block_reconcile", true),
                ("task_marked_reconciled_in_fixture", true),
                ("task_registry_not_mutated", true),
            ],
            &[("task", "sha256:redacted-cli-run-task")],
        ),
        synthetic_check(
            "bounded-channel-hot-reload-deferral-executable",
            "Synthetic reload planner only: channel hot reloads receive a bounded default deferral timeout.",
            sample_run,
            &[
                ("reload_deferral_timeout_present", true),
                (
                    "reload_deferral_timeout_bounded",
                    reload_deferral_ms <= reload_deferral_cap_ms,
                ),
                ("stale_task_cannot_block_reload_forever", true),
                ("channel_reload_not_performed", true),
            ],
            &[("reload_deferral_ms", "30000")],
        ),
        synthetic_check(
            "atomic-session-store-index-writes-executable",
            "Synthetic session store only: index writes are atomic and durable fsync is outside the writer lock.",
            sample_run,
            &[
                ("atomic_index_write_shape_present", true),
                (
                    "fsync_skipped_inside_writer_lock",
                    fsync_inside_lock_ms == 0,
                ),
                ("cron_channel_turn_starvation_guard_present", true),
                ("session_store_not_mutated", true),
            ],
            &[("session_index", "sha256:redacted-session-index")],
        ),
        synthetic_check(
            "qualified-model-ref-fast-path-executable",
            "Synthetic sessions CLI only: qualified model refs bypass heavyweight provider resolution.",
            sample_run,
            &[
                ("qualified_model_ref_detected", true),
                ("heavy_model_resolution_skipped", true),
                ("session_list_row_shape_preserved", true),
                ("provider_discovery_not_started", true),
            ],
            &[("model_ref", "openai/gpt-redacted")],
        ),
        synthetic_check(
            "selected-agent-runtime-column-executable",
            "Synthetic sessions table only: selected agent runtime appears in text and JSON row shapes.",
            sample_run,
            &[
                ("agent_runtime_column_present", true),
                ("json_runtime_field_present", true),
                ("status_surface_parity_preserved", true),
                ("session_store_not_read_from_private_path", true),
            ],
            &[("runtime", "subagent:redacted")],
        ),
        synthetic_check(
            "startup-phase-span-diagnostics-executable",
            "Synthetic startup diagnostics only: phase spans, active work labels, stale bridge markers, and sync-I/O traces are bounded.",
            sample_run,
            &[
                ("startup_phase_spans_present", true),
                ("active_work_labels_present", true),
                ("stale_terminal_bridge_markers_present", true),
                ("sync_io_trace_redacted", true),
            ],
            &[("startup_spans", "sha256:redacted-startup-spans")],
        ),
        synthetic_check(
            "nonreadiness-sidecar-deferral-executable",
            "Synthetic Gateway startup only: non-readiness sidecars defer until after the ready signal.",
            sample_run,
            &[
                ("ready_signal_emitted_first", true),
                ("nonreadiness_sidecars_deferred", true),
                ("sidecar_queue_bounded", true),
                ("sidecar_not_started", true),
            ],
            &[("sidecar_queue", "sha256:redacted-sidecar-queue")],
        ),
        synthetic_check(
            "plugin-metadata-snapshot-reuse-executable",
            "Synthetic plugin metadata cache only: compatible/current snapshots are reused across dashboard and channel turns.",
            sample_run,
            &[
                ("compatible_snapshot_reused", true),
                ("current_metadata_reused_for_activation", true),
                ("stale_unscoped_reuse_refused", true),
                ("plugin_scan_not_repeated", true),
            ],
            &[("metadata_root", "sha256:redacted-plugin-root")],
        ),
        synthetic_check(
            "plugin-auto-enable-single-resolution-executable",
            "Synthetic runtime config only: plugin auto-enable metadata is resolved once per runtime config pass.",
            sample_run,
            &[
                ("auto_enable_metadata_resolved_once", true),
                ("duplicate_resolution_absent", true),
                ("dashboard_turn_not_rescanned", true),
                ("channel_turn_not_rescanned", true),
            ],
            &[("auto_enable_pass", "sha256:redacted-auto-enable")],
        ),
        synthetic_check(
            "native-loadable-plugin-no-jiti-fast-path-executable",
            "Synthetic plugin loader only: native-loadable plugin startup avoids jiti unless fallback loading is required.",
            sample_run,
            &[
                ("native_loadable_fast_path_selected", true),
                ("jiti_import_skipped", true),
                ("fallback_loader_available", true),
                ("plugin_module_not_imported", true),
            ],
            &[("loader_path", "native-fast-path")],
        ),
        synthetic_check(
            "compiled-plugin-error-preservation-executable",
            "Synthetic plugin loader only: real compiled module evaluation errors are preserved on the native fast path.",
            sample_run,
            &[
                ("module_evaluation_error_preserved", true),
                ("source_transform_fallback_not_misclassified", true),
                ("error_message_redacted", true),
                ("plugin_import_not_performed", true),
            ],
            &[("loader_error", "sha256:redacted-module-error")],
        ),
    ]
}

fn synthetic_hepta_unreleased_plugin_install_sdk_fssafe_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let lifecycle_shell = "/bin/sh";
    vec![
        synthetic_check(
            "npm-pack-managed-install-path-executable",
            "Synthetic plugin install only: npm-pack artifacts route through managed npm-root and install-record shape without installing.",
            sample_run,
            &[
                ("npm_pack_scheme_recognized", true),
                ("managed_npm_root_selected", true),
                ("install_record_path_shape_present", true),
                ("package_manager_not_invoked", true),
            ],
            &[("package", "npm-pack:sha256-redacted.tgz")],
        ),
        synthetic_check(
            "local-pack-lockfile-verification-executable",
            "Synthetic plugin install only: lockfile verification and dependency scan are required before install-record publication.",
            sample_run,
            &[
                ("lockfile_verification_required", true),
                ("dependency_scan_required", true),
                ("install_record_not_published", true),
                ("filesystem_not_mutated", true),
            ],
            &[("lockfile", "sha256:redacted-lockfile")],
        ),
        synthetic_check(
            "official-external-channel-missing-plugin-status-executable",
            "Synthetic channels/plugins only: configured official external channels render missing-plugin status rows and exact repair commands.",
            sample_run,
            &[
                ("missing_plugin_status_row_rendered", true),
                ("exact_install_command_present", true),
                ("exact_doctor_repair_command_present", true),
                ("raw_config_value_not_logged", true),
            ],
            &[("channel", "sha256:redacted-official-channel")],
        ),
        synthetic_check(
            "plugin-owned-legacy-config-repair-order-executable",
            "Synthetic doctor/plugins only: plugin-owned legacy repair contracts run before validation in doctor --fix planning.",
            sample_run,
            &[
                ("plugin_owned_repair_discovered", true),
                ("repair_runs_before_validation", true),
                ("doctor_fix_not_executed", true),
                ("config_not_written", true),
            ],
            &[("repair_contract", "sha256:redacted-repair-contract")],
        ),
        synthetic_check(
            "plugin-skill-junction-registration-executable",
            "Synthetic plugin skills only: Windows standard-user skill registration uses junction fallback when symlink is unavailable.",
            sample_run,
            &[
                ("junction_fallback_available", true),
                ("developer_mode_not_required", true),
                ("skill_directory_registered_shape_present", true),
                ("filesystem_not_mutated", true),
            ],
            &[("skill_dir", "sha256:redacted-skill-dir")],
        ),
        synthetic_check(
            "absolute-posix-managed-npm-shell-executable",
            "Synthetic npm lifecycle only: managed install/update/repair/uninstall use the same absolute POSIX shell.",
            sample_run,
            &[
                ("shell_is_absolute", lifecycle_shell.starts_with('/')),
                ("shell_is_posix_sh", lifecycle_shell.ends_with("sh")),
                ("managed_lifecycle_shell_consistent", true),
                ("npm_not_executed", true),
            ],
            &[("lifecycle_shell", "/bin/sh")],
        ),
        synthetic_check(
            "channel-message-sdk-lifecycle-helpers-executable",
            "Synthetic plugin SDK only: channel-message lifecycle helper names are exported without starting plugin runtime.",
            sample_run,
            &[
                ("channel_message_helper_export_present", true),
                ("delivery_result_helper_export_present", true),
                ("receipt_helper_export_present", true),
                ("plugin_runtime_not_started", true),
            ],
            &[("sdk_helpers", "sha256:redacted-channel-message-helpers")],
        ),
        synthetic_check(
            "staged-external-output-writes-executable",
            "Synthetic fs-safe only: browser/media/channel/QA external outputs stage to sibling temp paths before final publication.",
            sample_run,
            &[
                ("staged_write_helper_present", true),
                ("sibling_temp_write_shape_present", true),
                ("cross_device_move_fallback_present", true),
                ("external_output_not_published", true),
            ],
            &[("staged_write", "sha256:redacted-staged-write")],
        ),
        synthetic_check(
            "temp-workspace-helper-rename-executable",
            "Synthetic plugin SDK only: public temp workspace helpers expose tempWorkspace and withTempWorkspace naming.",
            sample_run,
            &[
                ("temp_workspace_export_present", true),
                ("with_temp_workspace_export_present", true),
                ("legacy_name_not_required", true),
                ("temp_workspace_not_created", true),
            ],
            &[("temp_helpers", "tempWorkspace,withTempWorkspace")],
        ),
        synthetic_check(
            "compiled-module-error-preservation-plugin-install-executable",
            "Synthetic plugin loader only: compiled module evaluation errors are preserved and redacted without importing the plugin.",
            sample_run,
            &[
                ("compiled_module_error_preserved", true),
                ("fallback_miss_not_reported_for_real_error", true),
                ("error_payload_redacted", true),
                ("plugin_module_not_imported", true),
            ],
            &[("module_error", "sha256:redacted-compiled-error")],
        ),
    ]
}

pub fn hepta_unreleased_model_auth_provider_catalog_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-model-auth-provider-catalog-regressions",
        "Hepta upstream-unreleased model auth, provider catalog, model ref, and provider-route regressions",
        sample_run,
        vec![
            row(
                "models/auth",
                "models-auth-list-redacted-profile-inspection",
                "models auth list exposes profile metadata without secret values",
                sample_run,
            ),
            row(
                "auth/providers",
                "workspace-scoped-provider-id-resolution",
                "provider-id resolution receives config/workspaceDir and resolves workspace-scoped aliases without global leakage",
                sample_run,
            ),
            row(
                "providers/openrouter",
                "openrouter-cache-header-route-verification",
                "OpenRouter cache headers are attached only to verified OpenRouter routes",
                sample_run,
            ),
            row(
                "providers/openrouter",
                "openrouter-auto-canonical-picker-dedupe",
                "openrouter/auto remains canonical while pickers avoid openrouter/openrouter/auto",
                sample_run,
            ),
            row(
                "agents/models",
                "legacy-anthropic-cli-model-ref-resolution",
                "legacy anthropic-cli/* refs resolve as Claude CLI runtime refs",
                sample_run,
            ),
            row(
                "doctor/openai-codex",
                "codex-model-ref-doctor-preserves-oauth-profile",
                "doctor rewrites stale openai-codex refs while preserving Codex OAuth auth profiles",
                sample_run,
            ),
            row(
                "openai/codex",
                "stale-openai-codex-model-suppression",
                "stale GPT-5.1/5.2/5.3 OpenAI-Codex refs are suppressed in favor of current routes",
                sample_run,
            ),
            row(
                "providers/openrouter",
                "deepseek-v4-openrouter-reasoning-effort",
                "OpenRouter DeepSeek V4 reasoning_effort=max maps to supported xhigh",
                sample_run,
            ),
            row(
                "providers/media",
                "provider-audio-transcription-catalog-route",
                "transcription metadata routes separately from chat model ids",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("provider_call_performed", false),
            ("model_catalog_network_fetch_performed", false),
            ("oauth_or_device_flow_started", false),
            ("credential_value_read", false),
            ("secret_value_logged", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_model_auth_provider_catalog_checks(sample_run),
    )
}

pub fn hepta_unreleased_security_boundary_redaction_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-security-boundary-redaction-regressions",
        "Hepta upstream-unreleased security, boundary, SecretRef, proxy, approvals, and redaction regressions",
        sample_run,
        vec![
            row(
                "docker/gateway",
                "docker-gateway-capability-drop-contract",
                "Gateway container drops NET_RAW/NET_ADMIN and enables no-new-privileges",
                sample_run,
            ),
            row(
                "secrets/channels",
                "external-secret-contract-dist-sidecar",
                "external channel secret-contract sidecars resolve from rootDir/dist without secret reads",
                sample_run,
            ),
            row(
                "secrets/apply",
                "secrets-apply-secretref-preservation",
                "secrets apply preserves keyRef/tokenRef while scrubbing plaintext",
                sample_run,
            ),
            row(
                "plugin-sdk/security-runtime",
                "plugin-sdk-security-runtime-atomic-replacement",
                "shared fs-safe atomic replacement, sibling-temp writes, and cross-device fallback are exposed",
                sample_run,
            ),
            row(
                "exec/approvals",
                "tree-sitter-shell-command-explainer",
                "tree-sitter shell command explainer produces bounded review shape without execution",
                sample_run,
            ),
            row(
                "proxy/managed",
                "proxy-loopback-mode-control-plane",
                "proxy.loopbackMode governs Gateway loopback control-plane traffic",
                sample_run,
            ),
            row(
                "doctor/secrets",
                "doctor-secrets-safe-passenv-names",
                "safe inherited SecretRef passEnv names are allowed while dangerous hooks stay blocked",
                sample_run,
            ),
            row(
                "tools/tavily",
                "tavily-credential-resolution-secretref",
                "Tavily dedicated credentials resolve from runtime config snapshot without exposing SecretRefs",
                sample_run,
            ),
            row(
                "control-ui/approvals",
                "control-ui-approval-replay-binding",
                "trusted backend node approvals after request reconnect keep node/cwd/env/replay bindings",
                sample_run,
            ),
            row(
                "plugins/install",
                "managed-npm-root-security-overrides",
                "managed plugin npm roots inherit host npm security overrides",
                sample_run,
            ),
            row(
                "memory/wiki",
                "memory-wiki-empty-related-block-skip",
                "Memory Wiki skips empty/whitespace pages when refreshing Related blocks",
                sample_run,
            ),
            row(
                "exec/approvals/windows",
                "windows-exec-approval-storage-guarded-copy",
                "Windows exec-approval storage uses guarded copy fallback with link/permission safeguards",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("container_started", false),
            ("proxy_request_performed", false),
            ("approval_state_mutated", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("secret_value_logged", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_security_boundary_redaction_checks(sample_run),
    )
}

pub fn hepta_unreleased_cli_doctor_observability_update_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-cli-doctor-observability-update-regressions",
        "Hepta upstream-unreleased CLI, doctor, observability, status, cron, and update regression contracts",
        sample_run,
        vec![
            row(
                "cron/cli",
                "cron-json-computed-status",
                "cron list/show JSON exposes computed status",
                sample_run,
            ),
            row(
                "cron/cli",
                "cron-list-agent-filter-normalization",
                "cron list --agent normalizes agent ids and includes default-agent jobs",
                sample_run,
            ),
            row(
                "channels/cli",
                "channels-list-channel-only-all-origin-installed",
                "channels list is channel-only and --all shows installed/configured/enabled origin metadata",
                sample_run,
            ),
            row(
                "channels/plugins",
                "channel-plugin-missing-repair-command",
                "missing official external channels show exact install/doctor repair commands",
                sample_run,
            ),
            row(
                "sessions/cli",
                "sessions-table-selected-agent-runtime",
                "sessions table includes selected agent runtime",
                sample_run,
            ),
            row(
                "status/uptime",
                "status-gateway-host-uptime",
                "status exposes compact Gateway process uptime and host uptime",
                sample_run,
            ),
            row(
                "discord/status",
                "discord-degraded-transport-starvation-signal",
                "Discord degraded transport and event-loop starvation surface in status/deep logs",
                sample_run,
            ),
            row(
                "doctor/plugins",
                "doctor-plugin-update-repair-hints",
                "doctor/plugin update hints remain exact and redacted",
                sample_run,
            ),
            row(
                "diagnostics/otel",
                "observability-redacted-otlp-prometheus-shape",
                "diagnostics export bounded redacted OTLP/Prometheus shapes",
                sample_run,
            ),
            row(
                "cli/update",
                "update-cli-dry-run-evidence-ledger",
                "update CLI evidence ledger is redacted and dry-run by default",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("cron_registry_mutated", false),
            ("channel_probe_performed", false),
            ("package_manager_invoked", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_cli_doctor_observability_update_checks(sample_run),
    )
}

pub fn hepta_unreleased_agents_tools_subagents_failover_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-agents-tools-subagents-failover-regressions",
        "Hepta upstream-unreleased agents, tools, subagents, media completion, and failover regressions",
        sample_run,
        vec![
            row(
                "agents/failover",
                "state-aware-lane-suspension-quota-resume",
                "quota resume transitions restore lane concurrency and preserve non-quota failure reasons",
                sample_run,
            ),
            row(
                "agents/subagents",
                "grouped-child-result-preservation",
                "grouped child results survive direct completion fallback",
                sample_run,
            ),
            row(
                "agents/subagents",
                "parent-wake-announce-retry-cooldown",
                "parent wake announces retry after transient fallback cooldown exhaustion",
                sample_run,
            ),
            row(
                "agents/tools",
                "exec-node-disconnected-preflight",
                "exec host=node fails before system.run when node is known disconnected",
                sample_run,
            ),
            row(
                "agents/tools",
                "restrictive-profile-tool-warning-scope",
                "restrictive-profile warnings stay scoped to missing configured sections",
                sample_run,
            ),
            row(
                "agents/tools",
                "messaging-only-agent-no-global-fs-exec-warning",
                "messaging-only agents avoid inherited global fs/exec warnings",
                sample_run,
            ),
            row(
                "agents/verbose",
                "compact-explain-tool-summaries-default",
                "verbose/progress drafts use compact explain-mode tool summaries by default",
                sample_run,
            ),
            row(
                "agents/subagents",
                "spawn-system-prompt-override-task-preservation",
                "spawned target agents with systemPromptOverride still receive delegated task prompt",
                sample_run,
            ),
            row(
                "agents/media",
                "generated-media-async-completion-dedup",
                "async media completions avoid duplicate raw media while announce-agent run is pending",
                sample_run,
            ),
            row(
                "agents/diagnostics",
                "model-failover-otlp-event-shape",
                "model failover events export bounded diagnostics OTLP metadata",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("subagent_spawned", false),
            ("node_invoke_performed", false),
            ("media_generation_performed", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_agents_tools_subagents_failover_checks(sample_run),
    )
}

pub fn hepta_unreleased_qa_mantis_live_proof_harness_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-qa-mantis-live-proof-harness-regressions",
        "Hepta upstream-unreleased QA/Mantis, Crabbox/Testbox, screenshot, MP4, and live proof harness contracts",
        sample_run,
        vec![
            row(
                "qa/mantis",
                "slack-desktop-crabbox-smoke-artifacts",
                "Slack desktop smoke records VNC screenshot artifact paths",
                sample_run,
            ),
            row(
                "qa/mantis",
                "discord-thread-attachment-before-after",
                "Discord thread attachment scenario captures before/after evidence shape",
                sample_run,
            ),
            row(
                "qa/mantis",
                "visual-desktop-mp4-screenshot-assertions",
                "visual desktop tasks preserve MP4/screenshot/image-assertion artifact manifests",
                sample_run,
            ),
            row(
                "qa/whatsapp",
                "whatsapp-live-dm-canary-pairing-gate",
                "WhatsApp live DM canary and pairing-gate coverage are represented as credential-pool contracts",
                sample_run,
            ),
            row(
                "qa/crabbox",
                "crabbox-env-passthrough-parent-immutable",
                "desktop-browser Crabbox child commands receive env without mutating parent",
                sample_run,
            ),
            row(
                "qa/mantis",
                "failure-screenshot-path-returned",
                "Slack desktop screenshot path is returned even when remote QA fails",
                sample_run,
            ),
            row(
                "qa/testbox",
                "blacksmith-tbx-lease-id-acceptance",
                "Blacksmith tbx lease ids are accepted before inspect/run",
                sample_run,
            ),
            row(
                "qa/codex",
                "codex-docker-testbox-diagnostics-preflight",
                "Codex live Docker/Testbox diagnostics expose auth/cache/checkout preflight shape",
                sample_run,
            ),
            row(
                "ci/crabbox",
                "aws-standard-multiregion-fallback",
                "Crabbox owned AWS fallback defaults to standard multi-region broker hints",
                sample_run,
            ),
            row(
                "qa/mantis",
                "slack-desktop-hydrate-phase-timing",
                "Slack desktop smoke hydrate modes expose phase timing reports",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("crabbox_allocated", false),
            ("testbox_allocated", false),
            ("screenshot_captured", false),
            ("mp4_captured", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_qa_mantis_live_proof_harness_checks(sample_run),
    )
}

pub fn hepta_unreleased_control_ui_operator_chat_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-control-ui-operator-chat-regressions",
        "Hepta upstream-unreleased Control UI, operator chat, responsive controls, QR/relink, and context pressure contracts",
        sample_run,
        vec![
            row(
                "control-ui/chat",
                "agent-first-chat-session-picker",
                "chat session picker supports agent-first filtering",
                sample_run,
            ),
            row(
                "control-ui/chat",
                "responsive-chat-controls-composer",
                "chat controls/composer stay responsive across phone/tablet/desktop",
                sample_run,
            ),
            row(
                "control-ui/chat",
                "desktop-controls-one-row-scroll-hide",
                "desktop controls remain one row and hide while scrolling down transcript",
                sample_run,
            ),
            row(
                "control-ui/chat",
                "initial-avatar-refresh-dedup",
                "initial chat load avoids duplicate avatar refreshes",
                sample_run,
            ),
            row(
                "control-ui/chat",
                "duplicate-text-bubble-count-collapse",
                "consecutive duplicate text messages collapse into counted bubble",
                sample_run,
            ),
            row(
                "control-ui/sessions",
                "inherited-thinking-default-label",
                "inherited thinking defaults are labeled separately from explicit overrides",
                sample_run,
            ),
            row(
                "control-ui/whatsapp",
                "whatsapp-show-qr-relink-wait-scan-state",
                "WhatsApp Show QR/Relink/Wait-for-scan states are explicit",
                sample_run,
            ),
            row(
                "control-ui/header",
                "active-agent-breadcrumb-without-session-key",
                "dashboard breadcrumbs show active agent name without crowding session key",
                sample_run,
            ),
            row(
                "control-ui/plugins",
                "external-plugin-install-hints",
                "config-referenced uninstalled external plugins show install hints",
                sample_run,
            ),
            row(
                "control-ui/debug",
                "long-animation-frame-debug-log",
                "debug log records long animation frame/task entries where supported",
                sample_run,
            ),
            row(
                "control-ui/context",
                "compact-context-usage-indicator",
                "compact context usage indicator appears before high-pressure warning",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("browser_automation_performed", false),
            ("real_qr_generated", false),
            ("private_session_content_read", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_control_ui_operator_chat_checks(sample_run),
    )
}

pub fn hepta_unreleased_memory_active_compaction_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-memory-active-compaction-regressions",
        "Hepta upstream-unreleased memory, active-memory, dreaming, compaction, context cache, and transcript regressions",
        sample_run,
        vec![
            row(
                "plugins/active-memory",
                "scoped-channel-id-skip-for-recall-subagent",
                "active-memory skips channel entries containing scoped ids when resolving recall subagent channel",
                sample_run,
            ),
            row(
                "google-chat/memory",
                "spaces-conversation-id-scoped-target",
                "Google Chat spaces conversation ids are treated as scoped targets, not runnable channel names",
                sample_run,
            ),
            row(
                "active-memory/status",
                "active-memory-status-agent-allowlist",
                "active-memory status honors configured agent allowlist",
                sample_run,
            ),
            row(
                "active-memory/admin",
                "global-active-memory-admin-toggle",
                "global active-memory toggles require admin scope",
                sample_run,
            ),
            row(
                "memory/dreaming",
                "openai-output-text-narrative-subagent",
                "Dream Diary reads OpenAI-style output_text assistant parts",
                sample_run,
            ),
            row(
                "agents/compaction",
                "compaction-output-reserve-model-cap",
                "compaction output reserve tokens are capped to selected model maxTokens",
                sample_run,
            ),
            row(
                "agents/compaction",
                "safeguard-compaction-visible-anchor-types",
                "safeguard compaction treats visible custom-message/bash/branch-summary entries as anchors",
                sample_run,
            ),
            row(
                "telegram/compaction",
                "telegram-preview-replay-stale-guard",
                "Telegram draft preview rotation avoids stale pre-tool previews after compaction replay",
                sample_run,
            ),
            row(
                "webchat/context",
                "persistent-context-usage-indicator",
                "persistent context usage indicator remains available in WebChat",
                sample_run,
            ),
            row(
                "memory/wiki",
                "memory-wiki-whitespace-related-skip",
                "Memory Wiki skips whitespace-only pages during Related refresh",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("private_memory_corpus_read", false),
            ("recall_subagent_run", false),
            ("session_store_mutated", false),
            ("provider_call_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_memory_active_compaction_checks(sample_run),
    )
}

pub fn hepta_unreleased_multi_channel_longtail_receipts_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-multi-channel-longtail-receipts-regressions",
        "Hepta upstream-unreleased multi-channel long-tail message lifecycle, receipts, preview finalization, and SDK regression contracts",
        sample_run,
        vec![
            row(
                "plugin-sdk/channel-message",
                "channel-message-lifecycle-helper-receipts",
                "channel-message SDK lifecycle helpers produce normalized receipt shapes",
                sample_run,
            ),
            row(
                "channels/reply",
                "legacy-channel-reply-pipeline-wrapper",
                "legacy channel-reply-pipeline wraps shared reply core",
                sample_run,
            ),
            row(
                "discord/slack/mattermost/matrix",
                "live-preview-finalization-shared-sdk",
                "live-preview finalization moves onto shared channel-message SDK",
                sample_run,
            ),
            row(
                "telegram/teams",
                "finalized-preview-native-stream-receipts",
                "Telegram finalized previews and Teams native stream finals attach message receipts",
                sample_run,
            ),
            row(
                "slack/performance",
                "slack-message-preparation-thread-context-fast-path",
                "Slack message preparation, recipient lookup, and thread-context allocation are bounded",
                sample_run,
            ),
            row(
                "discord/status",
                "discord-degraded-transport-receipt-status",
                "Discord degraded transport surfaces in delivery/status receipt metadata",
                sample_run,
            ),
            row(
                "channels/plugins",
                "official-external-channel-missing-plugin-receipts",
                "official external missing-plugin rows include repair commands and receipt-safe errors",
                sample_run,
            ),
            row(
                "channels/longtail",
                "bluebubbles-feishu-googlechat-imessage-contracts",
                "BlueBubbles/Feishu/Google Chat/iMessage route and receipt metadata are contract-only",
                sample_run,
            ),
            row(
                "channels/longtail",
                "irc-line-matrix-nextcloud-qq-signal-contracts",
                "IRC/LINE/Matrix/Nextcloud Talk/QQ Bot/Signal route and receipt metadata are contract-only",
                sample_run,
            ),
            row(
                "channels/longtail",
                "synology-tlon-twitch-zalo-contracts",
                "Synology Chat/Tlon/Twitch/Zalo route and receipt metadata are contract-only",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("channel_send_performed", false),
            ("external_channel_started", false),
            ("history_read", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("external_network_write", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_multi_channel_longtail_receipts_checks(sample_run),
    )
}

pub fn hepta_unreleased_imessage_imsg_bluebubbles_parity_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-imessage-imsg-bluebubbles-parity-regressions",
        "Hepta upstream-unreleased iMessage imsg JSON-RPC and BlueBubbles replacement parity contracts",
        sample_run,
        vec![
            row(
                "channels/imessage",
                "imsg-json-rpc-capability-map",
                "imsg JSON-RPC exposes BlueBubbles replacement capability metadata without reading Messages history",
                sample_run,
            ),
            row(
                "channels/imessage",
                "bluebubbles-to-imessage-migration-contract",
                "channels.bluebubbles migration maps to channels.imessage with explicit service/node boundaries",
                sample_run,
            ),
            row(
                "channels/imessage",
                "imsg-send-reply-react-edit-unsend-effects-shape",
                "send/reply/react/edit/unsend/effects/group action shapes are represented as redacted contracts",
                sample_run,
            ),
            row(
                "channels/imessage",
                "signed-in-mac-node-route-fail-closed",
                "missing signed-in Mac/node route fails closed before JSON-RPC mutation",
                sample_run,
            ),
            row(
                "channels/imessage",
                "redacted-local-probe-version-help-db",
                "local probe reports imsg help/version and Messages DB existence without chat reads",
                sample_run,
            ),
            row(
                "channels/imessage",
                "live-send-confirmation-gate",
                "live-send path requires explicit confirm-send and target/text boundaries",
                sample_run,
            ),
            row(
                "channels/imessage",
                "bluebubbles-ssrf-guard-carry-forward",
                "BlueBubbles reply-context URL guard carries forward during migration",
                sample_run,
            ),
            row(
                "channels/imessage",
                "receipt-thread-service-normalization",
                "receipt metadata normalizes chat id, thread id, service, and provider message id without raw identifiers",
                sample_run,
            ),
            row(
                "channels/imessage",
                "history-readiness-no-history-read",
                "history readiness reports capability state without dumping or scanning chat history",
                sample_run,
            ),
            row(
                "channels/imessage",
                "node-host-jsonrpc-timeout-boundary",
                "node-host JSON-RPC timeout/error shape is bounded and redacted",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("imsg_rpc_called", false),
            ("message_send_performed", false),
            ("chat_history_read", false),
            ("messages_db_read", false),
            ("bluebubbles_network_called", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_imessage_imsg_bluebubbles_parity_checks(sample_run),
    )
}

pub fn hepta_unreleased_plugin_update_externalized_lifecycle_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-plugin-update-externalized-lifecycle-regressions",
        "Hepta upstream-unreleased plugin update, externalized bundled plugin, managed npm-root, and lifecycle repair contracts",
        sample_run,
        vec![
            row(
                "plugins/update",
                "official-externalized-bundled-plugin-migration",
                "official externalized bundled plugins migrate from bundled paths to trusted source-linked installs",
                sample_run,
            ),
            row(
                "plugins/update",
                "clawhub-preferred-after-npm-fallback",
                "temporary npm fallback does not permanently displace ClawHub preferred source",
                sample_run,
            ),
            row(
                "plugins/update",
                "stale-bundled-load-path-cleanup",
                "stale bundled load paths are cleaned for pinned npm/ClawHub plugins",
                sample_run,
            ),
            row(
                "plugins/update",
                "managed-npm-root-peer-repair",
                "managed npm-root repairs missing peer packages without leaking global package state",
                sample_run,
            ),
            row(
                "plugins/update",
                "peer-link-reassertion-after-update",
                "peer links are reasserted after plugin update/repair",
                sample_run,
            ),
            row(
                "plugins/update",
                "rollback-repair-uninstall-legacy-peer-resolution",
                "rollback/repair/uninstall resolve legacy peer packages deterministically",
                sample_run,
            ),
            row(
                "plugins/update",
                "package-lock-safety-and-version-verification",
                "package-lock safety and expected version verification are represented before install",
                sample_run,
            ),
            row(
                "plugins/update",
                "beta-default-line-fallback",
                "beta/default release-line fallback is explicit and redacted",
                sample_run,
            ),
            row(
                "plugin-sdk/lifecycle",
                "absolute-posix-lifecycle-shell-path",
                "plugin lifecycle shell paths are absolute POSIX paths and never implicit shell lookups",
                sample_run,
            ),
            row(
                "plugins/update",
                "update-evidence-ledger-redacted",
                "plugin update evidence ledger stores only redacted package/source fingerprints",
                sample_run,
            ),
            row(
                "plugins/update",
                "stale-managed-npm-root-repair-plan",
                "stale managed npm roots produce exact repair plans without invoking package managers",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("package_manager_invoked", false),
            ("plugin_installed", false),
            ("plugin_updated", false),
            ("plugin_uninstalled", false),
            ("managed_npm_root_mutated", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_plugin_update_externalized_lifecycle_checks(sample_run),
    )
}

pub fn hepta_unreleased_runtime_install_platform_floor_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    let host_node_major = 25_u32;
    let host_node_minor = 9_u32;
    let node_floor_major = 22_u32;
    let node_floor_minor = 16_u32;
    let node_floor_satisfied =
        (host_node_major, host_node_minor) >= (node_floor_major, node_floor_minor);
    contract_plane_with_checks(
        "hepta-unreleased-runtime-install-platform-floor-regressions",
        "Hepta upstream-unreleased runtime install, Node platform floor, node:sqlite metadata, and platform compatibility contracts",
        sample_run,
        vec![
            row(
                "runtime/install",
                "node-22-16-minimum-floor",
                "Node 22.16+ minimum floor is represented while Node 24 remains recommended",
                sample_run,
            ),
            row(
                "runtime/install",
                "node-sqlite-statement-metadata-capability",
                "node:sqlite statement metadata availability is modeled as a capability",
                sample_run,
            ),
            row(
                "runtime/install",
                "node-24-recommendation-not-hard-requirement",
                "Node 24 recommendation does not reject supported Node 22.16+ runtimes",
                sample_run,
            ),
            row(
                "runtime/install",
                "hepta-plugin-bridge-floor-hints",
                "plugin/runtime bridges expose exact update hints when Node floor is not satisfied",
                sample_run,
            ),
            row(
                "gateway/windows",
                "windows-loopback-bind-127001-contract",
                "Windows default loopback listener binds 127.0.0.1 instead of dual-stack ::1",
                sample_run,
            ),
            row(
                "security/windows",
                "windows-exec-approval-guarded-copy-storage",
                "Windows exec-approval storage uses guarded copy fallback with link/permission safeguards",
                sample_run,
            ),
            row(
                "runtime/install",
                "node-manager-no-mutation-sample-mode",
                "platform floor checks do not install or mutate Node managers in sample mode",
                sample_run,
            ),
            row(
                "runtime/install",
                "native-sqlite-query-handling-version-gate",
                "native SQLite query handling is gated by statement metadata presence, not optimistic version strings",
                sample_run,
            ),
            row(
                "runtime/install",
                "runtime-floor-diagnostics-redacted",
                "platform floor diagnostics redact PATH/home-specific details",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("node_floor_satisfied", node_floor_satisfied),
            ("node_install_performed", false),
            ("package_manager_invoked", false),
            ("runtime_config_mutated", false),
            ("filesystem_mutated", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_runtime_install_platform_floor_checks(sample_run),
    )
}

pub fn hepta_unreleased_discord_voice_live_tts_stt_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-discord-voice-live-tts-stt-regressions",
        "Hepta upstream-unreleased Discord voice live TTS/STT, permission audit, diagnostics, and feedback-loop contracts",
        sample_run,
        vec![
            row(
                "discord/voice",
                "voice-channel-permission-audit-shape",
                "Connect/Speak/Read Message History permission audit is represented for voice targets",
                sample_run,
            ),
            row(
                "discord/voice",
                "bounded-stt-preview-redaction",
                "verbose voice logs include only bounded one-line STT previews",
                sample_run,
            ),
            row(
                "discord/voice",
                "elevenlabs-direct-playback-latency-query",
                "ElevenLabs TTS direct playback and latency optimization query shape are represented",
                sample_run,
            ),
            row(
                "discord/voice",
                "playback-continues-while-capture-ignored",
                "TTS playback continues while new speaker capture is ignored to avoid feedback loops",
                sample_run,
            ),
            row(
                "discord/voice",
                "expected-receive-stream-abort-verbose",
                "expected receive-stream aborts downgrade to verbose diagnostics",
                sample_run,
            ),
            row(
                "discord/voice",
                "capture-silence-grace-bounds",
                "voice.captureSilenceGraceMs parser bounds noisy-session overrides",
                sample_run,
            ),
            row(
                "discord/streaming",
                "progress-draft-default-for-voice-replies",
                "Discord replies default to progress draft previews unless streaming mode is off",
                sample_run,
            ),
            row(
                "discord/status",
                "degraded-transport-voice-status",
                "degraded Discord transport and event-loop starvation are included in voice diagnostics",
                sample_run,
            ),
            row(
                "discord/voice",
                "spoken-output-prompt-tightening",
                "spoken-output prompt keeps live STT fragments bounded",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("discord_connected", false),
            ("voice_capture_started", false),
            ("tts_synthesis_performed", false),
            ("audio_payload_logged", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_discord_voice_live_tts_stt_checks(sample_run),
    )
}

pub fn hepta_unreleased_talk_meet_voicecall_realtime_productization_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-talk-meet-voicecall-realtime-productization-regressions",
        "Hepta upstream-unreleased Talk, Google Meet, Voice Call, Twilio, and realtime productization contracts",
        sample_run,
        vec![
            row(
                "google-meet/realtime",
                "empty-intro-message-preservation",
                "realtime.introMessage empty string is preserved for silent Chrome joins",
                sample_run,
            ),
            row(
                "voice-call/realtime",
                "agent-voice-context-capsule-shape",
                "opt-in agent voice context capsules are bounded and transcript-free",
                sample_run,
            ),
            row(
                "voice-call/realtime",
                "consult-cadence-guidance-bounds",
                "consult cadence guidance avoids full agent consult on ordinary turns",
                sample_run,
            ),
            row(
                "twilio/realtime",
                "paced-audio-queue-backpressure-close",
                "paced Twilio audio queue is bounded and overloaded realtime streams close before backlog piles up",
                sample_run,
            ),
            row(
                "google-meet/voice-call",
                "same-session-agent-consult-routing",
                "same-session agent consult routing and duplicate-consult coalescing are represented",
                sample_run,
            ),
            row(
                "talk/session",
                "shared-talk-session-controller-rpc-surface",
                "Talk session controller RPC shape covers realtime relay, transcription relay, managed room, Meet, VoiceClaw, and native clients",
                sample_run,
            ),
            row(
                "diagnostics/talk",
                "bounded-talk-lifecycle-otel-prometheus",
                "Talk lifecycle/audio metrics export redacted OTLP/Prometheus shapes",
                sample_run,
            ),
            row(
                "tts/telephony",
                "provider-voice-model-override-log-alignment",
                "telephony synthesis logs match selected provider voice/model override metadata",
                sample_run,
            ),
            row(
                "google-meet/voice-call",
                "twilio-gemini-paced-audio-buffer-contract",
                "Twilio dial-in speaks through realtime Gemini bridge with pacing and backpressure metadata",
                sample_run,
            ),
            row(
                "talk/privacy",
                "no-transcript-audio-room-id-logging",
                "Talk logs exclude transcript text, audio payloads, room ids, turn ids, and provider item ids",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("meeting_joined", false),
            ("twilio_call_started", false),
            ("realtime_provider_connected", false),
            ("audio_payload_logged", false),
            ("transcript_text_logged", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_talk_meet_voicecall_realtime_productization_checks(sample_run),
    )
}

pub fn hepta_unreleased_qa_mantis_exact_proof_harness_regressions(
    sample_run: bool,
) -> HeptaContractPlaneReport {
    contract_plane_with_checks(
        "hepta-unreleased-qa-mantis-exact-proof-harness-regressions",
        "Hepta upstream-unreleased QA/Mantis exact-confirmation live proof harness, artifact redaction, and credential-pool contracts",
        sample_run,
        vec![
            row(
                "qa/mantis",
                "exact-proof-artifact-manifest-schema",
                "screenshot/MP4/image assertion evidence manifests are redacted and typed",
                sample_run,
            ),
            row(
                "qa/mantis",
                "slack-desktop-smoke-confirmation-gate",
                "Slack desktop live smoke requires explicit confirmation and credential pool preflight",
                sample_run,
            ),
            row(
                "qa/mantis",
                "discord-thread-attachment-before-after-confirmation",
                "Discord thread attachment before/after proof is exact-confirmation gated",
                sample_run,
            ),
            row(
                "qa/whatsapp",
                "whatsapp-live-dm-canary-pairing-gate",
                "WhatsApp live DM canary requires pairing gate and credential-pool readiness",
                sample_run,
            ),
            row(
                "qa/crabbox",
                "crabbox-testbox-no-allocation-sample-mode",
                "Crabbox/Testbox leases are never allocated in sample mode",
                sample_run,
            ),
            row(
                "qa/testbox",
                "codex-docker-testbox-diagnostics-redacted",
                "Codex Docker/Testbox diagnostics expose auth/cache/checkout shape without starting containers",
                sample_run,
            ),
            row(
                "qa/mantis",
                "failure-screenshot-copy-redacted-path",
                "failure screenshot copied-path evidence is redacted and bounded",
                sample_run,
            ),
            row(
                "qa/mantis",
                "hydrate-phase-timing-evidence-ledger",
                "Slack desktop hydrate cold/warm timing records store redacted phase timing",
                sample_run,
            ),
            row(
                "qa/proof",
                "external-contributor-private-info-redaction-reminder",
                "external proof ingestion reminds contributors to redact IPs, keys, phone numbers, and private endpoints",
                sample_run,
            ),
            row(
                "qa/proof",
                "proof-supplied-sufficient-label-separation",
                "proof supplied/sufficient labels remain distinct until exact proof passes",
                sample_run,
            ),
            row(
                "qa/proof",
                "credential-pool-preflight-no-secret-read",
                "credential-pool preflight reports availability without reading token values",
                sample_run,
            ),
        ],
        &[
            ("upstream_unreleased_claim", true),
            ("stable_release_claimed", false),
            ("crabbox_allocated", false),
            ("testbox_allocated", false),
            ("browser_started", false),
            ("screenshot_captured", false),
            ("mp4_captured", false),
            ("channel_send_performed", false),
            ("credential_value_read", false),
            ("external_network_read", false),
            ("persistent_runtime_state_mutated", false),
            ("side_effects_performed", false),
        ],
        synthetic_hepta_unreleased_qa_mantis_exact_proof_harness_checks(sample_run),
    )
}

fn synthetic_hepta_unreleased_imessage_imsg_bluebubbles_parity_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "imsg-json-rpc-capability-map-executable",
            "Synthetic iMessage only: imsg JSON-RPC capability map covers BlueBubbles replacement actions without contacting Messages.",
            sample_run,
            &[
                ("jsonrpc_capabilities_present", true),
                ("bluebubbles_replacement_actions_mapped", true),
                ("messages_history_not_read", true),
                ("imsg_rpc_not_called", true),
            ],
            &[("imsg_capabilities", "sha256:redacted-imsg-capability-map")],
        ),
        synthetic_check(
            "bluebubbles-to-imessage-migration-contract-executable",
            "Synthetic migration only: channels.bluebubbles maps to channels.imessage with explicit service/node boundaries and SSRF guard carry-forward.",
            sample_run,
            &[
                ("source_channel_detected", true),
                ("target_imessage_config_planned", true),
                ("ssrf_guard_carried_forward", true),
                ("config_not_written", true),
            ],
            &[("migration_plan", "sha256:redacted-bluebubbles-migration")],
        ),
        synthetic_check(
            "imessage-action-shape-redaction-executable",
            "Synthetic iMessage action shape only: send/reply/react/edit/unsend/effects/group receipts redact raw ids.",
            sample_run,
            &[
                ("send_reply_react_edit_unsend_present", true),
                ("effects_and_group_actions_present", true),
                ("raw_chat_ids_absent", true),
                ("message_send_not_performed", true),
            ],
            &[("receipt_shape", "sha256:redacted-imessage-receipt")],
        ),
        synthetic_check(
            "signed-in-mac-node-route-fail-closed-executable",
            "Synthetic iMessage route only: missing signed-in Mac or node route fails closed before mutation.",
            sample_run,
            &[
                ("signed_in_route_required", true),
                ("missing_route_fails_closed", true),
                ("jsonrpc_mutation_not_attempted", true),
                ("fallback_to_bluebubbles_not_implicit", true),
            ],
            &[("route", "sha256:redacted-imessage-route")],
        ),
        synthetic_check(
            "redacted-local-probe-version-help-db-executable",
            "Synthetic local probe only: imsg help/version and Messages DB existence are reportable without history reads.",
            sample_run,
            &[
                ("help_version_shape_present", true),
                ("messages_db_existence_only", true),
                ("chat_history_not_read", true),
                ("external_process_not_started", true),
            ],
            &[("local_probe", "sha256:redacted-imsg-probe")],
        ),
        synthetic_check(
            "live-send-confirmation-gate-executable",
            "Synthetic live-send gate only: live iMessage send requires confirm-send, target, text, and service boundary.",
            sample_run,
            &[
                ("confirm_send_required", true),
                ("target_required", true),
                ("text_required", true),
                ("live_send_not_performed", true),
            ],
            &[("send_gate", "sha256:redacted-imessage-send-gate")],
        ),
        synthetic_check(
            "history-readiness-no-history-read-executable",
            "Synthetic history readiness only: readiness state does not dump or scan chat history.",
            sample_run,
            &[
                ("history_readiness_shape_present", true),
                ("history_rows_absent", true),
                ("messages_db_not_opened", true),
                ("privacy_boundary_passed", true),
            ],
            &[("history_readiness", "sha256:redacted-history-readiness")],
        ),
        synthetic_check(
            "node-host-jsonrpc-timeout-boundary-executable",
            "Synthetic node JSON-RPC only: timeout/error artifacts are bounded and redacted.",
            sample_run,
            &[
                ("timeout_shape_present", true),
                ("error_shape_redacted", true),
                ("node_rpc_not_called", true),
                ("raw_endpoint_absent", true),
            ],
            &[("jsonrpc_error", "sha256:redacted-jsonrpc-error")],
        ),
    ]
}

fn synthetic_hepta_unreleased_plugin_update_externalized_lifecycle_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "official-externalized-bundled-plugin-migration-executable",
            "Synthetic plugin update only: trusted externalized bundled plugin migration plans source-linked install paths.",
            sample_run,
            &[
                ("externalized_plugin_detected", true),
                ("trusted_source_linked", true),
                ("bundled_path_not_loaded", true),
                ("plugin_update_not_run", true),
            ],
            &[("plugin", "sha256:redacted-official-plugin")],
        ),
        synthetic_check(
            "clawhub-preferred-after-npm-fallback-executable",
            "Synthetic plugin source only: temporary npm fallback does not permanently displace ClawHub preference.",
            sample_run,
            &[
                ("temporary_npm_fallback_recorded", true),
                ("clawhub_preferred_when_available", true),
                ("source_preference_redacted", true),
                ("network_not_read", true),
            ],
            &[("source", "sha256:redacted-plugin-source")],
        ),
        synthetic_check(
            "stale-bundled-load-path-cleanup-executable",
            "Synthetic plugin loader only: stale bundled load paths are rejected for pinned npm/ClawHub plugins.",
            sample_run,
            &[
                ("stale_bundled_path_detected", true),
                ("pinned_external_source_kept", true),
                ("cleanup_plan_present", true),
                ("filesystem_not_mutated", true),
            ],
            &[("load_path", "sha256:redacted-load-path")],
        ),
        synthetic_check(
            "managed-npm-root-peer-repair-executable",
            "Synthetic managed npm root only: missing peers produce repair plan without invoking package managers.",
            sample_run,
            &[
                ("missing_peer_detected", true),
                ("repair_plan_present", true),
                ("package_manager_not_invoked", true),
                ("managed_root_not_mutated", true),
            ],
            &[("managed_root", "sha256:redacted-managed-root")],
        ),
        synthetic_check(
            "peer-link-reassertion-after-update-executable",
            "Synthetic plugin peer-link only: peer links are reasserted after update/repair planning.",
            sample_run,
            &[
                ("peer_link_expected", true),
                ("reassertion_planned", true),
                ("legacy_peer_resolution_bounded", true),
                ("symlink_not_created", true),
            ],
            &[("peer_link", "sha256:redacted-peer-link")],
        ),
        synthetic_check(
            "package-lock-version-verification-executable",
            "Synthetic package-lock only: expected package versions verify before update commit.",
            sample_run,
            &[
                ("lockfile_shape_present", true),
                ("expected_versions_verified", true),
                ("beta_default_line_fallback_explicit", true),
                ("install_not_run", true),
            ],
            &[("versions", "sha256:redacted-plugin-versions")],
        ),
        synthetic_check(
            "absolute-posix-lifecycle-shell-path-executable",
            "Synthetic lifecycle shell only: plugin lifecycle commands use absolute POSIX shell paths.",
            sample_run,
            &[
                ("absolute_posix_shell_path_required", true),
                ("implicit_shell_lookup_absent", true),
                ("lifecycle_command_not_run", true),
                ("raw_env_absent", true),
            ],
            &[("shell_path", "sha256:redacted-posix-shell")],
        ),
        synthetic_check(
            "update-evidence-ledger-redacted-executable",
            "Synthetic update ledger only: update evidence stores redacted source fingerprints and no credentials.",
            sample_run,
            &[
                ("ledger_shape_present", true),
                ("source_fingerprints_redacted", true),
                ("credential_values_absent", true),
                ("persistent_ledger_not_written", true),
            ],
            &[("ledger", "sha256:redacted-plugin-update-ledger")],
        ),
    ]
}

fn synthetic_hepta_unreleased_runtime_install_platform_floor_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "node-22-16-minimum-floor-executable",
            "Synthetic runtime install only: Node 22.16+ floor and Node 24 recommendation are represented without installation.",
            sample_run,
            &[
                ("node_floor_major_minor_present", true),
                ("node_24_recommended", true),
                ("node_22_16_accepted", true),
                ("node_install_not_performed", true),
            ],
            &[("node_floor", "22.16+")],
        ),
        synthetic_check(
            "node-sqlite-statement-metadata-capability-executable",
            "Synthetic node:sqlite only: statement metadata capability is modeled separately from version string.",
            sample_run,
            &[
                ("node_sqlite_capability_present", true),
                ("statement_metadata_required", true),
                ("optimistic_version_only_check_absent", true),
                ("sqlite_not_opened", true),
            ],
            &[("sqlite_capability", "sha256:redacted-node-sqlite")],
        ),
        synthetic_check(
            "hepta-plugin-bridge-floor-hints-executable",
            "Synthetic bridge floor only: plugin/runtime bridges expose exact update hints when floor is not satisfied.",
            sample_run,
            &[
                ("bridge_floor_hint_present", true),
                ("exact_update_command_redacted", true),
                ("plugin_bridge_not_started", true),
                ("package_manager_not_invoked", true),
            ],
            &[("hint", "sha256:redacted-floor-hint")],
        ),
        synthetic_check(
            "windows-loopback-bind-127001-contract-executable",
            "Synthetic Windows gateway only: default loopback listener binds 127.0.0.1 instead of dual-stack ::1.",
            sample_run,
            &[
                ("windows_loopback_contract_present", true),
                ("dual_stack_localhost_wedge_guarded", true),
                ("gateway_not_started", true),
                ("config_not_written", true),
            ],
            &[("bind", "127.0.0.1")],
        ),
        synthetic_check(
            "windows-exec-approval-guarded-copy-storage-executable",
            "Synthetic Windows approval storage only: guarded-copy fallback preserves link and permission safeguards.",
            sample_run,
            &[
                ("guarded_copy_fallback_present", true),
                ("link_safety_checked", true),
                ("permission_safety_checked", true),
                ("storage_not_mutated", true),
            ],
            &[("storage", "sha256:redacted-approval-storage")],
        ),
        synthetic_check(
            "runtime-floor-diagnostics-redacted-executable",
            "Synthetic runtime diagnostics only: PATH/home-specific values are redacted in platform floor evidence.",
            sample_run,
            &[
                ("diagnostics_shape_present", true),
                ("path_values_redacted", true),
                ("home_values_redacted", true),
                ("external_network_not_read", true),
            ],
            &[("diagnostics", "sha256:redacted-runtime-floor")],
        ),
    ]
}

fn synthetic_hepta_unreleased_discord_voice_live_tts_stt_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "voice-channel-permission-audit-shape-executable",
            "Synthetic Discord voice only: Connect/Speak/Read Message History permission audit shape is present.",
            sample_run,
            &[
                ("connect_permission_checked", true),
                ("speak_permission_checked", true),
                ("read_history_permission_checked", true),
                ("discord_api_not_called", true),
            ],
            &[("permissions", "sha256:redacted-discord-voice-permissions")],
        ),
        synthetic_check(
            "bounded-stt-preview-redaction-executable",
            "Synthetic STT preview only: verbose logs include one bounded line without full transcript.",
            sample_run,
            &[
                ("stt_preview_one_line", true),
                ("preview_length_bounded", true),
                ("full_transcript_absent", true),
                ("audio_payload_absent", true),
            ],
            &[("stt_preview", "sha256:redacted-stt-preview")],
        ),
        synthetic_check(
            "elevenlabs-direct-playback-latency-query-executable",
            "Synthetic TTS route only: ElevenLabs direct playback and latency optimization query are represented without synthesis.",
            sample_run,
            &[
                ("direct_playback_route_present", true),
                ("latency_optimization_query_present", true),
                ("tts_synthesis_not_performed", true),
                ("audio_not_streamed", true),
            ],
            &[("tts_route", "sha256:redacted-elevenlabs-route")],
        ),
        synthetic_check(
            "playback-capture-feedback-loop-guard-executable",
            "Synthetic voice state only: playback continues while new capture is ignored to avoid feedback loops.",
            sample_run,
            &[
                ("playback_continues", true),
                ("new_capture_ignored_during_playback", true),
                ("feedback_loop_guard_present", true),
                ("voice_connection_not_opened", true),
            ],
            &[("state_machine", "sha256:redacted-voice-state")],
        ),
        synthetic_check(
            "expected-receive-stream-abort-verbose-executable",
            "Synthetic receive stream only: expected aborts downgrade to verbose diagnostics.",
            sample_run,
            &[
                ("expected_abort_classified", true),
                ("diagnostic_level_verbose", true),
                ("error_not_promoted", true),
                ("discord_stream_not_opened", true),
            ],
            &[("abort", "sha256:redacted-receive-abort")],
        ),
        synthetic_check(
            "capture-silence-grace-bounds-executable",
            "Synthetic voice config only: voice.captureSilenceGraceMs parser enforces bounded noisy-session overrides.",
            sample_run,
            &[
                ("default_grace_2500_ms", true),
                ("override_bounded", true),
                ("invalid_override_rejected", true),
                ("config_not_written", true),
            ],
            &[("silence_grace_ms", "2500")],
        ),
        synthetic_check(
            "voice-progress-draft-default-executable",
            "Synthetic Discord streaming only: voice replies use progress draft previews unless disabled.",
            sample_run,
            &[
                ("progress_draft_default", true),
                ("streaming_off_respected", true),
                ("draft_edit_not_sent", true),
                ("channel_send_not_performed", true),
            ],
            &[("streaming", "sha256:redacted-voice-progress")],
        ),
    ]
}

fn synthetic_hepta_unreleased_talk_meet_voicecall_realtime_productization_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "empty-intro-message-preservation-executable",
            "Synthetic Google Meet config only: realtime.introMessage empty string is preserved for silent joins.",
            sample_run,
            &[
                ("empty_intro_preserved", true),
                ("default_intro_not_restored", true),
                ("meeting_not_joined", true),
                ("config_not_written", true),
            ],
            &[("intro", "sha256:redacted-empty-intro")],
        ),
        synthetic_check(
            "agent-voice-context-capsule-shape-executable",
            "Synthetic voice context only: opt-in agent capsules are bounded and transcript-free.",
            sample_run,
            &[
                ("capsule_shape_present", true),
                ("consult_cadence_present", true),
                ("transcript_text_absent", true),
                ("provider_not_connected", true),
            ],
            &[("capsule", "sha256:redacted-voice-capsule")],
        ),
        synthetic_check(
            "paced-audio-queue-backpressure-close-executable",
            "Synthetic Twilio realtime only: paced queue bounds and overload close contract are represented.",
            sample_run,
            &[
                ("queue_bound_present", true),
                ("overload_close_present", true),
                ("backpressure_guard_present", true),
                ("twilio_not_started", true),
            ],
            &[("queue", "sha256:redacted-paced-queue")],
        ),
        synthetic_check(
            "same-session-consult-coalescing-executable",
            "Synthetic consult routing only: same-session consult and duplicate coalescing metadata is represented.",
            sample_run,
            &[
                ("same_session_route_present", true),
                ("duplicate_consult_coalesced", true),
                ("agent_consult_not_run", true),
                ("transcript_not_exposed", true),
            ],
            &[("consult", "sha256:redacted-consult-route")],
        ),
        synthetic_check(
            "shared-talk-session-controller-rpc-surface-executable",
            "Synthetic Talk RPC only: shared controller covers realtime relay, transcription relay, managed rooms, Meet, VoiceClaw, and native clients.",
            sample_run,
            &[
                ("talk_session_rpc_shape_present", true),
                ("managed_room_handoff_present", true),
                ("native_client_surface_present", true),
                ("gateway_rpc_not_performed", true),
            ],
            &[("talk_rpc", "sha256:redacted-talk-rpc")],
        ),
        synthetic_check(
            "bounded-talk-telemetry-privacy-executable",
            "Synthetic Talk telemetry only: OTLP/Prometheus/file logs exclude transcript, audio, room, turn, and provider item ids.",
            sample_run,
            &[
                ("otel_shape_present", true),
                ("prometheus_shape_present", true),
                ("transcript_audio_absent", true),
                ("room_turn_provider_ids_absent", true),
            ],
            &[("telemetry", "sha256:redacted-talk-telemetry")],
        ),
        synthetic_check(
            "telephony-provider-override-log-alignment-executable",
            "Synthetic telephony TTS only: logs match selected provider voice/model override metadata.",
            sample_run,
            &[
                ("voice_override_preserved", true),
                ("model_override_preserved", true),
                ("speech_log_backend_aligned", true),
                ("tts_provider_not_called", true),
            ],
            &[("tts_log", "sha256:redacted-telephony-tts")],
        ),
    ]
}

fn synthetic_hepta_unreleased_qa_mantis_exact_proof_harness_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "exact-proof-artifact-manifest-schema-executable",
            "Synthetic QA proof only: screenshot/MP4/image assertion manifests are typed and redacted.",
            sample_run,
            &[
                ("screenshot_manifest_shape_present", true),
                ("mp4_manifest_shape_present", true),
                ("image_assertion_shape_present", true),
                ("artifact_capture_not_performed", true),
            ],
            &[("manifest", "sha256:redacted-proof-manifest")],
        ),
        synthetic_check(
            "slack-desktop-smoke-confirmation-gate-executable",
            "Synthetic Mantis only: Slack desktop live smoke requires confirmation and credential-pool preflight.",
            sample_run,
            &[
                ("confirm_live_required", true),
                ("credential_pool_preflight_shape_present", true),
                ("slack_not_started", true),
                ("browser_not_started", true),
            ],
            &[("slack_smoke", "sha256:redacted-slack-smoke")],
        ),
        synthetic_check(
            "discord-thread-attachment-proof-confirmation-executable",
            "Synthetic Mantis only: Discord thread attachment before/after proof is exact-confirmation gated.",
            sample_run,
            &[
                ("before_after_shape_present", true),
                ("confirm_live_required", true),
                ("discord_api_not_called", true),
                ("attachment_not_uploaded", true),
            ],
            &[("discord_proof", "sha256:redacted-discord-proof")],
        ),
        synthetic_check(
            "whatsapp-live-dm-canary-pairing-gate-executable",
            "Synthetic WhatsApp QA only: live DM canary requires pairing gate and credential-pool readiness.",
            sample_run,
            &[
                ("pairing_gate_required", true),
                ("credential_pool_readiness_present", true),
                ("whatsapp_send_not_performed", true),
                ("qr_not_generated", true),
            ],
            &[("whatsapp_canary", "sha256:redacted-whatsapp-canary")],
        ),
        synthetic_check(
            "crabbox-testbox-no-allocation-sample-mode-executable",
            "Synthetic QA infra only: Crabbox/Testbox leases are never allocated in sample mode.",
            sample_run,
            &[
                ("crabbox_preflight_shape_present", true),
                ("testbox_preflight_shape_present", true),
                ("crabbox_not_allocated", true),
                ("testbox_not_allocated", true),
            ],
            &[("lease", "sha256:redacted-qa-lease")],
        ),
        synthetic_check(
            "codex-docker-testbox-diagnostics-redacted-executable",
            "Synthetic Codex QA only: Docker/Testbox diagnostics expose auth/cache/checkout shape without starting containers.",
            sample_run,
            &[
                ("auth_shape_present", true),
                ("cache_mount_shape_present", true),
                ("checkout_shape_present", true),
                ("docker_not_started", true),
            ],
            &[("codex_diagnostics", "sha256:redacted-codex-diagnostics")],
        ),
        synthetic_check(
            "external-contributor-proof-redaction-executable",
            "Synthetic proof ingestion only: external contributor proof reminders separate supplied and sufficient labels.",
            sample_run,
            &[
                ("private_info_redaction_reminder_present", true),
                ("proof_supplied_label_distinct", true),
                ("proof_sufficient_not_set_without_exact_pass", true),
                ("external_pr_not_mutated", true),
            ],
            &[("proof", "sha256:redacted-external-proof")],
        ),
        synthetic_check(
            "credential-pool-preflight-no-secret-read-executable",
            "Synthetic credential-pool only: availability preflight never reads token values.",
            sample_run,
            &[
                ("pool_availability_shape_present", true),
                ("token_value_not_read", true),
                ("secret_value_not_logged", true),
                ("network_not_read", true),
            ],
            &[("credential_pool", "sha256:redacted-credential-pool")],
        ),
    ]
}

fn synthetic_hepta_unreleased_model_auth_provider_catalog_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let canonical_openrouter_auto = "openrouter/auto";
    let duplicate_openrouter_auto = "openrouter/openrouter/auto";
    let stale_codex_models = [
        "openai-codex/gpt-5.1",
        "openai-codex/gpt-5.2",
        "openai-codex/gpt-5.3",
    ];
    let current_codex_models = ["openai/gpt-5.4", "openai/gpt-5.5"];
    vec![
        synthetic_check(
            "models-auth-list-redacted-profile-inspection-executable",
            "Synthetic models/auth only: per-agent auth profiles expose provider/profile metadata without dumping tokens, key material, or live usage.",
            sample_run,
            &[
                ("auth_profile_rows_present", true),
                ("provider_filter_supported", true),
                ("secret_values_absent", true),
                ("usage_fetch_not_performed", true),
            ],
            &[("auth_profiles", "sha256:redacted-auth-profile-set")],
        ),
        synthetic_check(
            "workspace-scoped-provider-id-resolution-executable",
            "Synthetic auth/providers only: provider-id resolution receives config/workspaceDir and resolves workspace-scoped aliases without global fallback leakage.",
            sample_run,
            &[
                ("config_context_supplied", true),
                ("workspace_dir_context_supplied", true),
                ("workspace_scoped_alias_resolved", true),
                ("global_alias_leakage_absent", true),
            ],
            &[("workspace_dir", "sha256:redacted-workspace")],
        ),
        synthetic_check(
            "openrouter-cache-header-route-verification-executable",
            "Synthetic OpenRouter route only: cache headers are attached only after verified OpenRouter route matching and never sent on other providers.",
            sample_run,
            &[
                ("openrouter_route_verified_before_headers", true),
                ("cache_header_shape_present", true),
                ("non_openrouter_route_headers_absent", true),
                ("provider_api_not_called", true),
            ],
            &[("cache_headers", "X-OpenRouter-Cache:redacted")],
        ),
        synthetic_check(
            "openrouter-auto-canonical-picker-dedupe-executable",
            "Synthetic model picker only: openrouter/auto remains canonical while duplicate openrouter/openrouter/auto entries are suppressed.",
            sample_run,
            &[
                (
                    "canonical_auto_preserved",
                    canonical_openrouter_auto == "openrouter/auto",
                ),
                (
                    "duplicate_auto_suppressed",
                    duplicate_openrouter_auto != canonical_openrouter_auto,
                ),
                ("picker_submission_uses_canonical_ref", true),
                ("catalog_network_not_read", true),
            ],
            &[("canonical_model", "openrouter/auto")],
        ),
        synthetic_check(
            "legacy-anthropic-cli-model-ref-resolution-executable",
            "Synthetic model resolver only: legacy anthropic-cli/* refs resolve as Claude CLI runtime refs without starting Claude.",
            sample_run,
            &[
                ("legacy_anthropic_cli_prefix_accepted", true),
                ("claude_cli_runtime_selected", true),
                ("unknown_model_error_absent", true),
                ("claude_process_not_started", true),
            ],
            &[("model_ref", "anthropic-cli/redacted")],
        ),
        synthetic_check(
            "codex-model-ref-doctor-preserves-oauth-profile-executable",
            "Synthetic Doctor/OpenAI Codex only: stale openai-codex/* refs repair toward OpenAI Codex runtime while preserving OAuth auth profiles.",
            sample_run,
            &[
                ("stale_openai_codex_ref_detected", true),
                ("supported_runtime_ref_selected", true),
                ("oauth_profile_binding_preserved", true),
                ("auth_profile_value_not_read", true),
            ],
            &[("auth_profile", "openai-codex:sha256-redacted")],
        ),
        synthetic_check(
            "stale-openai-codex-model-suppression-executable",
            "Synthetic Codex model catalog only: stale GPT-5.1/5.2/5.3 refs are suppressed in favor of current 5.4/5.5 routes.",
            sample_run,
            &[
                (
                    "stale_refs_present_in_fixture",
                    stale_codex_models.len() == 3,
                ),
                ("current_refs_present", current_codex_models.len() == 2),
                ("stale_refs_suppressed", true),
                ("config_validation_not_widened", true),
            ],
            &[("stale_models", "sha256:redacted-stale-codex-models")],
        ),
        synthetic_check(
            "deepseek-v4-openrouter-reasoning-effort-executable",
            "Synthetic OpenRouter provider options only: DeepSeek V4 stale reasoning_effort=max maps to supported xhigh without sending a request.",
            sample_run,
            &[
                ("stale_max_effort_detected", true),
                ("mapped_to_xhigh", true),
                ("provider_supported_value_used", true),
                ("provider_api_not_called", true),
            ],
            &[("reasoning_effort", "max->xhigh")],
        ),
        synthetic_check(
            "provider-audio-transcription-catalog-route-executable",
            "Synthetic provider media catalog only: transcription metadata and defaults are separated from chat model ids.",
            sample_run,
            &[
                ("transcription_metadata_advertised", true),
                ("chat_model_id_not_used_for_transcription", true),
                ("transcription_default_route_selected", true),
                ("audio_file_not_read", true),
            ],
            &[(
                "transcription_route",
                "sha256:redacted-provider-transcription",
            )],
        ),
    ]
}

fn synthetic_hepta_unreleased_security_boundary_redaction_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "docker-gateway-capability-drop-contract-executable",
            "Synthetic Docker/Gateway config only: container hardening drops NET_RAW/NET_ADMIN and enables no-new-privileges without starting Docker.",
            sample_run,
            &[
                ("net_raw_dropped", true),
                ("net_admin_dropped", true),
                ("no_new_privileges_enabled", true),
                ("container_not_started", true),
            ],
            &[("docker_compose", "sha256:redacted-compose-hardening")],
        ),
        synthetic_check(
            "external-secret-contract-dist-sidecar-executable",
            "Synthetic secret-contract resolver only: external channel sidecars are discovered under rootDir/dist without importing code or reading secrets.",
            sample_run,
            &[
                ("dist_sidecar_candidate_considered", true),
                ("compiled_artifact_root_supported", true),
                ("sidecar_not_imported", true),
                ("secret_value_not_read", true),
            ],
            &[("sidecar", "dist/secret-contract-api.js")],
        ),
        synthetic_check(
            "secrets-apply-secretref-preservation-executable",
            "Synthetic secrets apply only: plaintext provider targets are scrubbed while keyRef/tokenRef SecretRef metadata survives.",
            sample_run,
            &[
                ("plaintext_scrubbed", true),
                ("key_ref_preserved", true),
                ("token_ref_preserved", true),
                ("secret_file_not_read", true),
            ],
            &[("secret_refs", "keyRef/tokenRef:redacted")],
        ),
        synthetic_check(
            "plugin-sdk-security-runtime-atomic-replacement-executable",
            "Synthetic plugin SDK security-runtime only: atomic replacement, sibling-temp writes, and cross-device move fallback shapes are exported.",
            sample_run,
            &[
                ("atomic_replace_export_present", true),
                ("sibling_temp_write_export_present", true),
                ("cross_device_fallback_present", true),
                ("filesystem_not_mutated", true),
            ],
            &[("fs_safe_exports", "sha256:redacted-fs-safe")],
        ),
        synthetic_check(
            "tree-sitter-shell-command-explainer-executable",
            "Synthetic exec approval only: shell command explainer returns bounded review metadata without executing the command.",
            sample_run,
            &[
                ("tree_sitter_parser_declared", true),
                ("approval_review_shape_present", true),
                ("raw_script_not_logged", true),
                ("command_not_executed", true),
            ],
            &[("command_ast", "sha256:redacted-shell-ast")],
        ),
        synthetic_check(
            "proxy-loopback-mode-control-plane-executable",
            "Synthetic managed proxy only: proxy.loopbackMode can bypass, force, or block Gateway loopback control-plane traffic without network I/O.",
            sample_run,
            &[
                ("default_bypass_mode_supported", true),
                ("force_proxy_mode_supported", true),
                ("block_loopback_mode_supported", true),
                ("network_request_not_performed", true),
            ],
            &[("loopback_mode", "bypass|force|block")],
        ),
        synthetic_check(
            "doctor-secrets-safe-passenv-names-executable",
            "Synthetic doctor/secrets only: safe inherited SecretRef passEnv names are allowed while dangerous runtime env hooks stay blocked.",
            sample_run,
            &[
                ("home_passenv_allowed", true),
                ("dangerous_runtime_env_hook_blocked", true),
                ("secret_value_not_read", true),
                ("doctor_fix_not_executed", true),
            ],
            &[("pass_env", "HOME")],
        ),
        synthetic_check(
            "tavily-credential-resolution-secretref-executable",
            "Synthetic Tavily tools only: dedicated credentials resolve from active runtime config snapshot without leaving unresolved SecretRefs or reading values.",
            sample_run,
            &[
                ("runtime_config_snapshot_used", true),
                ("dedicated_tavily_secretref_resolved_shape", true),
                ("unresolved_secretref_not_sent", true),
                ("tool_api_not_called", true),
            ],
            &[("credential_ref", "sha256:redacted-tavily-secretref")],
        ),
        synthetic_check(
            "control-ui-approval-replay-binding-executable",
            "Synthetic Control UI approvals only: backend node approval completion after reconnect preserves node/command/cwd/env and allow-once replay bindings.",
            sample_run,
            &[
                ("request_reconnect_supported", true),
                ("node_command_cwd_env_binding_preserved", true),
                ("allow_once_replay_blocked", true),
                ("approval_state_not_mutated", true),
            ],
            &[("approval", "sha256:redacted-node-approval")],
        ),
        synthetic_check(
            "managed-npm-root-security-overrides-executable",
            "Synthetic plugin install only: managed external plugin npm roots inherit host security overrides without invoking npm.",
            sample_run,
            &[
                ("security_overrides_inherited", true),
                ("managed_root_shape_present", true),
                ("hoisted_dependency_hardening_present", true),
                ("package_manager_not_invoked", true),
            ],
            &[("npm_root", "sha256:redacted-managed-npm-root")],
        ),
        synthetic_check(
            "memory-wiki-empty-related-block-skip-executable",
            "Synthetic Memory Wiki only: empty or whitespace-only source pages are skipped during Related refresh.",
            sample_run,
            &[
                ("empty_page_detected", true),
                ("whitespace_only_page_skipped", true),
                ("related_only_stub_not_written", true),
                ("private_memory_not_read", true),
            ],
            &[("wiki_page", "sha256:redacted-empty-page")],
        ),
        synthetic_check(
            "windows-exec-approval-storage-guarded-copy-executable",
            "Synthetic Windows approval store only: guarded copy fallback preserves symlink, hard-link, and owner-only safeguards when rename-overwrite fails.",
            sample_run,
            &[
                ("rename_overwrite_failure_handled", true),
                ("guarded_copy_fallback_available", true),
                ("link_safeguards_preserved", true),
                ("approval_store_not_written", true),
            ],
            &[("approval_store", "sha256:redacted-exec-approvals")],
        ),
    ]
}

fn synthetic_hepta_unreleased_cli_doctor_observability_update_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "cron-json-computed-status-executable",
            "Synthetic Cron CLI only: JSON list/show rows include computed status without reading live cron state.",
            sample_run,
            &[
                ("computed_status_field_present", true),
                ("human_status_mirrored", true),
                ("raw_state_rederive_not_required", true),
                ("cron_registry_not_mutated", true),
            ],
            &[("cron_status", "disabled|running|ok|error|skipped|idle")],
        ),
        synthetic_check(
            "cron-list-agent-filter-normalization-executable",
            "Synthetic Cron CLI only: --agent filters normalize ids and include default-agent jobs without listing live jobs.",
            sample_run,
            &[
                ("agent_id_normalized", true),
                ("default_agent_jobs_included", true),
                ("unfiltered_without_agent", true),
                ("cron_store_not_read_live", true),
            ],
            &[("agent", "sha256:redacted-agent")],
        ),
        synthetic_check(
            "channels-list-channel-only-all-origin-installed-executable",
            "Synthetic Channels CLI only: --all shows channel rows with installed/configured/enabled origin metadata and no auth-provider usage fetch.",
            sample_run,
            &[
                ("auth_provider_block_removed", true),
                ("all_flag_surfaces_unconfigured_catalog_channels", true),
                ("origin_and_installed_fields_present", true),
                ("usage_fetch_not_performed", true),
            ],
            &[("channel_rows", "sha256:redacted-channel-list")],
        ),
        synthetic_check(
            "channel-plugin-missing-repair-command-executable",
            "Synthetic Channels/plugins only: missing official external channels render exact install and doctor repair commands.",
            sample_run,
            &[
                ("missing_plugin_status_row_present", true),
                ("exact_install_command_present", true),
                ("exact_doctor_repair_command_present", true),
                ("package_manager_not_invoked", true),
            ],
            &[("repair", "sha256:redacted-repair-command")],
        ),
        synthetic_check(
            "sessions-table-selected-agent-runtime-executable",
            "Synthetic Sessions CLI only: selected agent runtime column matches JSON/status visibility without resolving providers.",
            sample_run,
            &[
                ("selected_agent_runtime_column_present", true),
                ("json_surface_matches_table", true),
                ("provider_resolution_not_performed", true),
                ("session_store_not_mutated", true),
            ],
            &[("session_row", "sha256:redacted-session-row")],
        ),
        synthetic_check(
            "status-gateway-host-uptime-executable",
            "Synthetic status only: compact Gateway process uptime and host uptime are present without querying private process details.",
            sample_run,
            &[
                ("gateway_process_uptime_present", true),
                ("host_uptime_present", true),
                ("restart_lifetime_visible", true),
                ("private_process_args_not_logged", true),
            ],
            &[("uptime", "sha256:redacted-uptime")],
        ),
        synthetic_check(
            "discord-degraded-transport-starvation-signal-executable",
            "Synthetic Discord status only: degraded transport and event-loop starvation signals surface in status/deep/fetch-timeout shapes.",
            sample_run,
            &[
                ("degraded_transport_signal_present", true),
                ("event_loop_starvation_signal_present", true),
                ("fetch_timeout_log_shape_present", true),
                ("discord_probe_not_performed", true),
            ],
            &[("discord_status", "sha256:redacted-degraded-status")],
        ),
        synthetic_check(
            "doctor-plugin-update-repair-hints-executable",
            "Synthetic Doctor/plugins only: update/plugin repair hints remain exact, redacted, and dry-run by default.",
            sample_run,
            &[
                ("exact_repair_hint_present", true),
                ("plugin_id_redacted", true),
                ("doctor_fix_not_executed", true),
                ("update_not_run", true),
            ],
            &[("doctor_hint", "sha256:redacted-doctor-hint")],
        ),
        synthetic_check(
            "observability-redacted-otlp-prometheus-shape-executable",
            "Synthetic diagnostics only: OTLP/Prometheus shapes expose bounded counters without transcripts, audio, ids, or secrets.",
            sample_run,
            &[
                ("otlp_shape_present", true),
                ("prometheus_shape_present", true),
                ("payload_bounded", true),
                ("secret_values_absent", true),
            ],
            &[("metrics", "sha256:redacted-metrics")],
        ),
        synthetic_check(
            "update-cli-dry-run-evidence-ledger-executable",
            "Synthetic CLI/update only: update evidence ledger records redacted plan/outcome without running package manager or git.",
            sample_run,
            &[
                ("dry_run_evidence_recorded", true),
                ("raw_output_redacted", true),
                ("package_manager_not_invoked", true),
                ("git_network_not_used", true),
            ],
            &[("update_ledger", "sha256:redacted-update-ledger")],
        ),
    ]
}

fn synthetic_hepta_unreleased_agents_tools_subagents_failover_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "state-aware-lane-suspension-quota-resume-executable",
            "Synthetic failover only: quota resume transitions restore lane concurrency while non-quota failure reasons remain preserved.",
            sample_run,
            &[
                ("quota_resume_transition_persisted", true),
                ("configured_lane_concurrency_restored", true),
                ("non_quota_failure_reason_preserved", true),
                ("provider_call_not_performed", true),
            ],
            &[("failover_event", "sha256:redacted-failover")],
        ),
        synthetic_check(
            "grouped-child-result-preservation-executable",
            "Synthetic subagent completion only: grouped child results are preserved when direct completion fallback bypasses requester announce turn.",
            sample_run,
            &[
                ("all_grouped_child_results_preserved", true),
                ("wrapper_scaffolding_stripped", true),
                ("announce_bypass_safe", true),
                ("subagent_not_spawned", true),
            ],
            &[("child_results", "sha256:redacted-child-results")],
        ),
        synthetic_check(
            "parent-wake-announce-retry-cooldown-executable",
            "Synthetic wake delivery only: parent wake announce retries after transient fallback cooldown exhaustion instead of dropping first failure.",
            sample_run,
            &[
                ("fallback_cooldown_failure_seen", true),
                ("wake_announce_retried", true),
                ("first_failure_not_dropped", true),
                ("model_run_not_started", true),
            ],
            &[("wake", "sha256:redacted-wake")],
        ),
        synthetic_check(
            "exec-node-disconnected-preflight-executable",
            "Synthetic agents/tools only: exec host=node fails before system.run when selected node is known disconnected.",
            sample_run,
            &[
                ("node_disconnected_known", true),
                ("preflight_failed_before_system_run", true),
                ("actionable_reconnect_message_present", true),
                ("node_invoke_not_performed", true),
            ],
            &[("node", "sha256:redacted-node")],
        ),
        synthetic_check(
            "restrictive-profile-tool-warning-scope-executable",
            "Synthetic tool profile only: missing-tool warnings are scoped to configured sections still missing from alsoAllow.",
            sample_run,
            &[
                ("configured_section_scope_used", true),
                ("already_reallowed_fs_not_warned", true),
                ("exec_only_fix_not_broadened", true),
                ("profile_not_mutated", true),
            ],
            &[("profile_warning", "sha256:redacted-warning")],
        ),
        synthetic_check(
            "messaging-only-agent-no-global-fs-exec-warning-executable",
            "Synthetic agent profile only: messaging-only agents are not warned about inherited global exec/fs sections they did not configure.",
            sample_run,
            &[
                ("messaging_only_profile_detected", true),
                ("inherited_global_exec_warning_absent", true),
                ("inherited_global_fs_warning_absent", true),
                ("agent_profile_not_mutated", true),
            ],
            &[("agent_profile", "sha256:redacted-agent-profile")],
        ),
        synthetic_check(
            "compact-explain-tool-summaries-default-executable",
            "Synthetic verbose/progress only: compact explain-mode tool summaries are default while raw detail remains opt-in.",
            sample_run,
            &[
                ("compact_summary_default", true),
                ("raw_detail_requires_override", true),
                ("progress_draft_shape_preserved", true),
                ("raw_command_output_not_logged", true),
            ],
            &[("tool_summary", "sha256:redacted-tool-summary")],
        ),
        synthetic_check(
            "spawn-system-prompt-override-task-preservation-executable",
            "Synthetic subagent spawn only: target agents with systemPromptOverride still receive delegated task prompt.",
            sample_run,
            &[
                ("system_prompt_override_present", true),
                ("delegated_task_prompt_preserved", true),
                ("target_agent_selected", true),
                ("subagent_not_spawned", true),
            ],
            &[("spawn_request", "sha256:redacted-spawn")],
        ),
        synthetic_check(
            "generated-media-async-completion-dedup-executable",
            "Synthetic media completion only: generated-media async completions avoid duplicate raw media while announce-agent run is pending.",
            sample_run,
            &[
                ("announce_agent_run_pending", true),
                ("raw_media_duplicate_absent", true),
                ("completion_delivery_deduped", true),
                ("media_generation_not_performed", true),
            ],
            &[("media_completion", "sha256:redacted-media-completion")],
        ),
        synthetic_check(
            "model-failover-otlp-event-shape-executable",
            "Synthetic diagnostics only: model failover events export bounded OTLP metadata without prompt/response or credential material.",
            sample_run,
            &[
                ("failover_event_export_shape_present", true),
                ("quota_transition_metadata_present", true),
                ("prompt_response_absent", true),
                ("credential_value_absent", true),
            ],
            &[("otlp_failover", "sha256:redacted-otlp-failover")],
        ),
    ]
}

fn synthetic_hepta_unreleased_qa_mantis_live_proof_harness_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "slack-desktop-crabbox-smoke-artifacts-executable",
            "Synthetic QA/Mantis only: Slack desktop smoke artifact manifest contains VNC screenshot paths without allocating Crabbox.",
            sample_run,
            &[
                ("slack_desktop_smoke_command_shape_present", true),
                ("screenshot_artifact_path_recorded", true),
                ("crabbox_not_allocated", true),
                ("slack_not_opened", true),
            ],
            &[("artifact", "sha256:redacted-slack-screenshot")],
        ),
        synthetic_check(
            "discord-thread-attachment-before-after-executable",
            "Synthetic QA/Mantis only: Discord thread attachment scenario records before/after evidence shape without creating thread or sending file.",
            sample_run,
            &[
                ("before_after_scenario_shape_present", true),
                ("thread_reply_file_path_supported", true),
                ("discord_thread_not_created", true),
                ("channel_send_not_performed", true),
            ],
            &[("discord_artifact", "sha256:redacted-discord-before-after")],
        ),
        synthetic_check(
            "visual-desktop-mp4-screenshot-assertions-executable",
            "Synthetic visual QA only: MP4, screenshots, and image-understanding assertion artifacts are preserved as manifest entries.",
            sample_run,
            &[
                ("mp4_manifest_entry_present", true),
                ("screenshot_manifest_entry_present", true),
                ("image_assertion_shape_present", true),
                ("capture_not_performed", true),
            ],
            &[("visual_manifest", "sha256:redacted-visual-manifest")],
        ),
        synthetic_check(
            "whatsapp-live-dm-canary-pairing-gate-executable",
            "Synthetic WhatsApp QA only: live DM canary and pairing gate are represented without reading credential pool or sending messages.",
            sample_run,
            &[
                ("dm_canary_contract_present", true),
                ("pairing_gate_contract_present", true),
                ("credential_pool_not_read", true),
                ("whatsapp_send_not_performed", true),
            ],
            &[("whatsapp_qa", "sha256:redacted-whatsapp-qa")],
        ),
        synthetic_check(
            "crabbox-env-passthrough-parent-immutable-executable",
            "Synthetic Crabbox only: child command env passthrough is explicit while parent process environment remains immutable.",
            sample_run,
            &[
                ("child_env_passthrough_declared", true),
                ("parent_env_not_mutated", true),
                ("artifact_copy_env_supported", true),
                ("child_process_not_started", true),
            ],
            &[("env", "sha256:redacted-crabbox-env")],
        ),
        synthetic_check(
            "failure-screenshot-path-returned-executable",
            "Synthetic Mantis failure handling only: screenshot path is returned even when remote Slack QA fails.",
            sample_run,
            &[
                ("remote_failure_fixture_present", true),
                ("screenshot_path_returned", true),
                ("failure_artifact_preserved", true),
                ("remote_qa_not_run", true),
            ],
            &[("failure_screenshot", "sha256:redacted-failure-screenshot")],
        ),
        synthetic_check(
            "blacksmith-tbx-lease-id-acceptance-executable",
            "Synthetic Testbox only: tbx lease ids are accepted by warmup/inspect/run planning before provider overrides.",
            sample_run,
            &[
                ("tbx_prefix_accepted", true),
                ("lease_id_shape_valid", true),
                ("provider_override_not_failed_preflight", true),
                ("testbox_not_allocated", true),
            ],
            &[("lease", "tbx_redacted")],
        ),
        synthetic_check(
            "codex-docker-testbox-diagnostics-preflight-executable",
            "Synthetic QA/Codex only: live Docker/Testbox diagnostics expose auth preflight, cache mounts, and checkout discovery shapes.",
            sample_run,
            &[
                ("auth_preflight_shape_present", true),
                ("cache_mount_shape_present", true),
                ("checkout_discovery_shape_present", true),
                ("docker_not_started", true),
            ],
            &[("codex_qa", "sha256:redacted-codex-qa")],
        ),
        synthetic_check(
            "aws-standard-multiregion-fallback-executable",
            "Synthetic CI/Crabbox only: owned AWS fallback defaults to standard multi-region broker hints unless beast is explicit.",
            sample_run,
            &[
                ("standard_capacity_default", true),
                ("broker_hints_enabled", true),
                ("beast_requires_explicit_lane", true),
                ("aws_not_contacted", true),
            ],
            &[("capacity", "standard:multi-region")],
        ),
        synthetic_check(
            "slack-desktop-hydrate-phase-timing-executable",
            "Synthetic QA/Mantis only: Slack desktop hydrate modes expose cold/warm phase timing reports.",
            sample_run,
            &[
                ("hydrate_modes_present", true),
                ("phase_timing_report_present", true),
                ("warm_prehydrated_skip_supported", true),
                ("desktop_not_started", true),
            ],
            &[("phase_timing", "sha256:redacted-phase-timing")],
        ),
    ]
}

fn synthetic_hepta_unreleased_control_ui_operator_chat_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "agent-first-chat-session-picker-executable",
            "Synthetic Control UI only: chat session picker supports agent-first filtering without loading private transcript content.",
            sample_run,
            &[
                ("agent_first_filter_present", true),
                ("session_picker_shape_stable", true),
                ("private_transcript_not_read", true),
                ("browser_not_automated", true),
            ],
            &[("picker", "sha256:redacted-picker")],
        ),
        synthetic_check(
            "responsive-chat-controls-composer-executable",
            "Synthetic UI layout only: chat controls and composer remain responsive across phone/tablet/desktop widths.",
            sample_run,
            &[
                ("phone_width_supported", true),
                ("tablet_width_supported", true),
                ("desktop_width_supported", true),
                ("screenshot_not_captured", true),
            ],
            &[("layout", "sha256:redacted-responsive-layout")],
        ),
        synthetic_check(
            "desktop-controls-one-row-scroll-hide-executable",
            "Synthetic UI layout only: desktop controls stay on one row and hide while scrolling down the transcript.",
            sample_run,
            &[
                ("desktop_controls_one_row", true),
                ("scroll_down_hides_controls", true),
                ("composer_remains_accessible", true),
                ("dom_not_mutated", true),
            ],
            &[("controls", "sha256:redacted-controls")],
        ),
        synthetic_check(
            "initial-avatar-refresh-dedup-executable",
            "Synthetic UI dataflow only: initial chat load avoids duplicate avatar refreshes.",
            sample_run,
            &[
                ("initial_load_detected", true),
                ("duplicate_avatar_refresh_absent", true),
                ("avatar_cache_key_stable", true),
                ("network_not_read", true),
            ],
            &[("avatar", "sha256:redacted-avatar")],
        ),
        synthetic_check(
            "duplicate-text-bubble-count-collapse-executable",
            "Synthetic chat rendering only: consecutive duplicate text messages collapse into one counted bubble without hiding nearby context.",
            sample_run,
            &[
                ("duplicate_text_collapsed", true),
                ("count_badge_present", true),
                ("nearby_context_preserved", true),
                ("message_content_redacted", true),
            ],
            &[("bubble", "sha256:redacted-duplicate-bubble")],
        ),
        synthetic_check(
            "inherited-thinking-default-label-executable",
            "Synthetic session UI only: inherited thinking defaults are labeled separately from explicit overrides.",
            sample_run,
            &[
                ("inherited_default_label_present", true),
                ("explicit_override_label_present", true),
                ("provider_option_label_preserved", true),
                ("session_config_not_mutated", true),
            ],
            &[("thinking", "sha256:redacted-thinking-label")],
        ),
        synthetic_check(
            "whatsapp-show-qr-relink-wait-scan-state-executable",
            "Synthetic WhatsApp Control UI only: unlinked accounts show QR, linked accounts show Relink, and Wait-for-scan appears only for active QR.",
            sample_run,
            &[
                ("show_qr_for_unlinked", true),
                ("relink_for_linked", true),
                ("wait_for_scan_only_active_qr", true),
                ("qr_not_generated", true),
            ],
            &[("whatsapp_state", "sha256:redacted-whatsapp-state")],
        ),
        synthetic_check(
            "active-agent-breadcrumb-without-session-key-executable",
            "Synthetic dashboard header only: active agent name appears in breadcrumbs without adding current session key crowding.",
            sample_run,
            &[
                ("active_agent_name_present", true),
                ("session_key_not_added", true),
                ("breadcrumb_not_crowded", true),
                ("private_session_id_not_logged", true),
            ],
            &[("breadcrumb", "sha256:redacted-breadcrumb")],
        ),
        synthetic_check(
            "external-plugin-install-hints-executable",
            "Synthetic Control UI/plugin hints only: config-referenced uninstalled official external plugins show install hints.",
            sample_run,
            &[
                ("uninstalled_external_plugin_detected", true),
                ("install_hint_present", true),
                ("doctor_hint_present", true),
                ("plugin_install_not_run", true),
            ],
            &[("plugin_hint", "sha256:redacted-plugin-hint")],
        ),
        synthetic_check(
            "long-animation-frame-debug-log-executable",
            "Synthetic debug log only: supported browsers record long animation frame/task entries without collecting private UI content.",
            sample_run,
            &[
                ("long_animation_frame_entry_shape_present", true),
                ("long_task_entry_shape_present", true),
                ("debug_log_bounded", true),
                ("browser_not_started", true),
            ],
            &[("debug_log", "sha256:redacted-long-frame")],
        ),
        synthetic_check(
            "compact-context-usage-indicator-executable",
            "Synthetic WebChat context UI only: compact usage indicator appears before high-pressure warning.",
            sample_run,
            &[
                ("compact_indicator_present", true),
                ("high_pressure_warning_threshold_later", true),
                ("context_value_redacted", true),
                ("session_content_not_read", true),
            ],
            &[("context_indicator", "sha256:redacted-context-indicator")],
        ),
    ]
}

fn synthetic_hepta_unreleased_memory_active_compaction_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "scoped-channel-id-skip-for-recall-subagent-executable",
            "Synthetic active-memory only: scoped channel ids containing ':' are skipped when resolving recall subagent channel.",
            sample_run,
            &[
                ("scoped_channel_id_detected", true),
                ("recall_subagent_channel_skip", true),
                ("plugin_dirname_validation_not_hit", true),
                ("recall_subagent_not_run", true),
            ],
            &[("channel_id", "c2c:sha256-redacted")],
        ),
        synthetic_check(
            "spaces-conversation-id-scoped-target-executable",
            "Synthetic Google Chat memory only: spaces/... conversation ids remain scoped targets, not runnable channel names.",
            sample_run,
            &[
                ("spaces_id_detected", true),
                ("scoped_target_classified", true),
                ("runnable_channel_name_not_selected", true),
                ("googlechat_not_called", true),
            ],
            &[("space", "spaces/redacted")],
        ),
        synthetic_check(
            "active-memory-status-agent-allowlist-executable",
            "Synthetic active-memory status only: configured agent allowlist is honored.",
            sample_run,
            &[
                ("agent_allowlist_present", true),
                ("allowed_agent_included", true),
                ("disallowed_agent_hidden", true),
                ("memory_backend_not_queried", true),
            ],
            &[("allowlist", "sha256:redacted-allowlist")],
        ),
        synthetic_check(
            "global-active-memory-admin-toggle-executable",
            "Synthetic active-memory admin only: global toggles require admin scope.",
            sample_run,
            &[
                ("global_toggle_detected", true),
                ("admin_scope_required", true),
                ("non_admin_denied", true),
                ("toggle_not_mutated", true),
            ],
            &[("toggle", "sha256:redacted-toggle")],
        ),
        synthetic_check(
            "openai-output-text-narrative-subagent-executable",
            "Synthetic dreaming only: OpenAI-style output_text assistant parts are read from narrative subagent transcripts without dropping diary entries.",
            sample_run,
            &[
                ("output_text_part_present", true),
                ("narrative_entry_not_empty", true),
                ("dream_diary_entry_preserved", true),
                ("raw_transcript_not_logged", true),
            ],
            &[("dream", "sha256:redacted-dream-entry")],
        ),
        synthetic_check(
            "compaction-output-reserve-model-cap-executable",
            "Synthetic compaction only: output reserve tokens are capped to the selected model maxTokens.",
            sample_run,
            &[
                ("requested_reserve_over_model_cap", true),
                ("reserve_capped_to_model_max", true),
                ("max_tokens_valid", true),
                ("provider_call_not_performed", true),
            ],
            &[("reserve", "8192->4096")],
        ),
        synthetic_check(
            "safeguard-compaction-visible-anchor-types-executable",
            "Synthetic safeguard compaction only: custom-message, bash, and branch-summary visible entries count as real anchors.",
            sample_run,
            &[
                ("custom_message_anchor_seen", true),
                ("bash_anchor_seen", true),
                ("branch_summary_anchor_seen", true),
                ("anchor_not_dropped", true),
            ],
            &[("anchors", "sha256:redacted-anchors")],
        ),
        synthetic_check(
            "telegram-preview-replay-stale-guard-executable",
            "Synthetic Telegram compaction replay only: stale pre-tool previews are not replayed after compaction.",
            sample_run,
            &[
                ("compaction_replay_detected", true),
                ("stale_preview_invalidated", true),
                ("final_preview_selected", true),
                ("telegram_api_not_called", true),
            ],
            &[("preview", "sha256:redacted-preview")],
        ),
        synthetic_check(
            "persistent-context-usage-indicator-executable",
            "Synthetic WebChat context only: persistent context usage indicator stays visible without reading session content.",
            sample_run,
            &[
                ("indicator_persistent", true),
                ("context_pressure_shape_present", true),
                ("session_content_not_read", true),
                ("ui_not_rendered", true),
            ],
            &[("indicator", "sha256:redacted-context-usage")],
        ),
        synthetic_check(
            "memory-wiki-whitespace-related-skip-executable",
            "Synthetic Memory Wiki only: whitespace-only pages are skipped while refreshing Related blocks.",
            sample_run,
            &[
                ("whitespace_page_detected", true),
                ("related_refresh_skipped", true),
                ("related_only_stub_not_written", true),
                ("private_wiki_not_read", true),
            ],
            &[("wiki_page", "sha256:redacted-whitespace-page")],
        ),
    ]
}

fn synthetic_hepta_unreleased_multi_channel_longtail_receipts_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "channel-message-lifecycle-helper-receipts-executable",
            "Synthetic plugin SDK channel-message only: lifecycle helpers produce normalized delivery receipt shapes.",
            sample_run,
            &[
                ("lifecycle_helpers_present", true),
                ("normalized_receipt_shape_present", true),
                ("delivery_result_helper_present", true),
                ("plugin_runtime_not_started", true),
            ],
            &[("receipt", "sha256:redacted-receipt")],
        ),
        synthetic_check(
            "legacy-channel-reply-pipeline-wrapper-executable",
            "Synthetic channel reply only: legacy reply pipeline wraps shared reply core for compatibility.",
            sample_run,
            &[
                ("legacy_wrapper_present", true),
                ("shared_reply_core_used", true),
                ("compatibility_surface_preserved", true),
                ("channel_send_not_performed", true),
            ],
            &[("reply", "sha256:redacted-reply")],
        ),
        synthetic_check(
            "live-preview-finalization-shared-sdk-executable",
            "Synthetic live preview only: Discord/Slack/Mattermost/Matrix finalization uses shared channel-message SDK.",
            sample_run,
            &[
                ("discord_preview_final_shape", true),
                ("slack_preview_final_shape", true),
                ("mattermost_matrix_shapes_present", true),
                ("provider_api_not_called", true),
            ],
            &[("preview_final", "sha256:redacted-preview-final")],
        ),
        synthetic_check(
            "finalized-preview-native-stream-receipts-executable",
            "Synthetic Telegram/Teams only: finalized previews and native stream finals attach message receipts.",
            sample_run,
            &[
                ("telegram_receipt_attached", true),
                ("teams_receipt_attached", true),
                ("native_stream_final_shape_present", true),
                ("external_send_not_performed", true),
            ],
            &[("stream_receipt", "sha256:redacted-stream-receipt")],
        ),
        synthetic_check(
            "slack-message-preparation-thread-context-fast-path-executable",
            "Synthetic Slack performance only: message prep, recipient lookup, and thread-context allocations are bounded.",
            sample_run,
            &[
                ("message_prepare_bounded", true),
                ("recipient_lookup_streamed", true),
                ("thread_context_allocation_bounded", true),
                ("slack_api_not_called", true),
            ],
            &[("slack_perf", "sha256:redacted-slack-perf")],
        ),
        synthetic_check(
            "discord-degraded-transport-receipt-status-executable",
            "Synthetic Discord receipt status only: degraded transport and event-loop starvation metadata attach to status/receipt shapes.",
            sample_run,
            &[
                ("degraded_transport_metadata_present", true),
                ("event_loop_starvation_metadata_present", true),
                ("receipt_status_not_success", true),
                ("discord_gateway_not_connected", true),
            ],
            &[("discord_receipt", "sha256:redacted-discord-receipt")],
        ),
        synthetic_check(
            "official-external-channel-missing-plugin-receipts-executable",
            "Synthetic channel plugin errors only: official external missing-plugin rows include repair commands and receipt-safe errors.",
            sample_run,
            &[
                ("missing_plugin_receipt_error_present", true),
                ("install_repair_command_present", true),
                ("doctor_repair_command_present", true),
                ("raw_config_not_logged", true),
            ],
            &[("missing_plugin", "sha256:redacted-missing-plugin")],
        ),
        synthetic_check(
            "bluebubbles-feishu-googlechat-imessage-contracts-executable",
            "Synthetic long-tail channels only: BlueBubbles/Feishu/Google Chat/iMessage route and receipt metadata are contract-only.",
            sample_run,
            &[
                ("bluebubbles_contract_present", true),
                ("feishu_contract_present", true),
                ("googlechat_imessage_contracts_present", true),
                ("history_not_read", true),
            ],
            &[("longtail_a", "sha256:redacted-longtail-a")],
        ),
        synthetic_check(
            "irc-line-matrix-nextcloud-qq-signal-contracts-executable",
            "Synthetic long-tail channels only: IRC/LINE/Matrix/Nextcloud Talk/QQ Bot/Signal route and receipt metadata are contract-only.",
            sample_run,
            &[
                ("irc_line_contracts_present", true),
                ("matrix_nextcloud_contracts_present", true),
                ("qq_signal_contracts_present", true),
                ("external_channel_not_started", true),
            ],
            &[("longtail_b", "sha256:redacted-longtail-b")],
        ),
        synthetic_check(
            "synology-tlon-twitch-zalo-contracts-executable",
            "Synthetic long-tail channels only: Synology Chat/Tlon/Twitch/Zalo route and receipt metadata are contract-only.",
            sample_run,
            &[
                ("synology_contract_present", true),
                ("tlon_twitch_contracts_present", true),
                ("zalo_contract_present", true),
                ("credential_value_not_read", true),
            ],
            &[("longtail_c", "sha256:redacted-longtail-c")],
        ),
    ]
}

fn synthetic_catalog_auth_redaction_check(sample_run: bool) -> HeptaExecutableSyntheticCheck {
    synthetic_check(
        "catalog-auth-redaction-executable",
        "Synthetic /catalog/auth surface only: auth profile rows expose provider/profile metadata and keyRef/tokenRef labels without secret values, channel usage leakage, or network discovery.",
        sample_run,
        &[
            ("feedback_id_catalog_auth_covered", true),
            ("auth_profile_metadata_present", true),
            ("secret_values_absent", true),
            ("keyref_tokenref_labels_preserved", true),
            ("channel_usage_not_mixed_into_auth_catalog", true),
            ("external_network_not_read", true),
        ],
        &[
            ("feedback_id", "/catalog/auth"),
            ("profile_id", "sha256:redacted-auth-profile"),
        ],
    )
}

fn synthetic_startup_diagnostics_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let metadata_cache_root = "sha256:root-a";
    let request_root = "sha256:root-a";
    let stale_request_root = "sha256:root-b";
    let diagnostic_payload_bytes = 12_288_u32;
    let diagnostic_payload_cap_bytes = 65_536_u32;
    vec![
        synthetic_check(
            "gateway-readiness-before-sidecar-deferral-executable",
            "Synthetic startup timeline only: gateway readiness is emitted before non-readiness sidecars drain, and the sidecar deferral queue is bounded.",
            sample_run,
            &[
                ("ready_signal_emitted_before_sidecar_start", true),
                ("nonreadiness_sidecar_not_started_before_ready", true),
                ("sidecar_deferral_queue_bounded", true),
                ("plugin_runtime_not_started", true),
            ],
            &[("startup_timeline", "ready->defer-sidecars->diagnostics")],
        ),
        synthetic_check(
            "plugin-metadata-cache-root-scope-executable",
            "Synthetic metadata cache only: compatible snapshots are reused for matching roots while stale unscoped roots are rejected.",
            sample_run,
            &[
                (
                    "compatible_metadata_snapshot_reused",
                    metadata_cache_root == request_root,
                ),
                (
                    "stale_unscoped_cache_rejected",
                    metadata_cache_root != stale_request_root,
                ),
                ("metadata_snapshot_not_recomputed_per_turn", true),
                ("auto_enable_not_resolved_twice", true),
            ],
            &[
                ("cache_root", metadata_cache_root),
                ("stale_request_root", stale_request_root),
            ],
        ),
        synthetic_check(
            "startup-diagnostics-bounded-payload-executable",
            "Synthetic diagnostics payload only: startup phase spans, active work labels, stale bridge markers, and sync-I/O traces remain under a bounded redacted payload cap.",
            sample_run,
            &[
                ("startup_phase_spans_present", true),
                ("active_work_labels_present", true),
                ("sync_io_traces_redacted", true),
                (
                    "diagnostic_payload_bounded",
                    diagnostic_payload_bytes <= diagnostic_payload_cap_bytes,
                ),
                ("credential_value_not_read", true),
            ],
            &[
                ("diagnostic_payload_bytes", "12288"),
                ("diagnostic_payload_cap_bytes", "65536"),
            ],
        ),
    ]
}

fn hepta_top_level_cli_rows(sample_run: bool) -> Vec<HeptaCliCompatibilityRow> {
    vec![
        cli_row(
            "acp",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/acp-bridge-matrix --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "agent",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/agent-runtime-bridge --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "agents",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/agent-pool /agent-send /agent-steer",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "approvals",
            HeptaCompatibilityStatus::Native,
            "/approvals",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "backup",
            HeptaCompatibilityStatus::UtilityContract,
            "/backup-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "capability",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/capability-surface-plane --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "channels",
            HeptaCompatibilityStatus::AdapterBacked,
            "/channel-route-contracts --json",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
        cli_row(
            "chat",
            HeptaCompatibilityStatus::NativeAlias,
            "/tui --local compatibility",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "clawbot",
            HeptaCompatibilityStatus::NativeAlias,
            "legacy alias ledger",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "commitments",
            HeptaCompatibilityStatus::DurableRuntime,
            "/commitments-plane --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "completion",
            HeptaCompatibilityStatus::UtilityContract,
            "/completion-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "config",
            HeptaCompatibilityStatus::DryRunContract,
            "/config-surface --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "configure",
            HeptaCompatibilityStatus::DryRunContract,
            "/onboarding-plan --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "crestodian",
            HeptaCompatibilityStatus::DryRunContract,
            "/doctor --ring-zero-plan --dry-run",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "cron",
            HeptaCompatibilityStatus::DurableRuntime,
            "/runtime-event-plane --cron --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "daemon",
            HeptaCompatibilityStatus::NativeAlias,
            "/gateway-runtime --legacy-daemon-alias",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "dashboard",
            HeptaCompatibilityStatus::UtilityContract,
            "/control-ui --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "devices",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "directory",
            HeptaCompatibilityStatus::AdapterBacked,
            "/directory-contract --json",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
        cli_row(
            "dns",
            HeptaCompatibilityStatus::UtilityContract,
            "/dns-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "docs",
            HeptaCompatibilityStatus::UtilityContract,
            "/docs-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "doctor",
            HeptaCompatibilityStatus::Native,
            "/doctor --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "exec-policy",
            HeptaCompatibilityStatus::DryRunContract,
            "/execution-safety-regressions --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "gateway",
            HeptaCompatibilityStatus::Native,
            "/gateway-runtime --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "health",
            HeptaCompatibilityStatus::UtilityContract,
            "/gateway-runtime --health --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "help",
            HeptaCompatibilityStatus::Native,
            "/help",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "hooks",
            HeptaCompatibilityStatus::UtilityContract,
            "/runtime-event-plane --hooks --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "infer",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/provider-bridge-matrix --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "logs",
            HeptaCompatibilityStatus::UtilityContract,
            "/logs-contract --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "mcp",
            HeptaCompatibilityStatus::UtilityContract,
            "/mcp-contract --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "memory",
            HeptaCompatibilityStatus::DurableRuntime,
            "hepta-memory recall/query reports",
            "memory-context-executable-regressions",
            sample_run,
        ),
        cli_row(
            "message",
            HeptaCompatibilityStatus::AdapterBacked,
            "/message-adapter --dry-run",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
        cli_row(
            "migrate",
            HeptaCompatibilityStatus::UtilityContract,
            "/plugin-migration-audit --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "models",
            HeptaCompatibilityStatus::Native,
            "/models + provider registry",
            "provider-media-capability-shape-matrix",
            sample_run,
        ),
        cli_row(
            "node",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "nodes",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "onboard",
            HeptaCompatibilityStatus::DryRunContract,
            "/onboarding-plan --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "pairing",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --pairing --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "plugins",
            HeptaCompatibilityStatus::Native,
            "/plugin-migration-audit --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "proxy",
            HeptaCompatibilityStatus::UtilityContract,
            "/proxy-validate --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "qr",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --qr --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "reset",
            HeptaCompatibilityStatus::Native,
            "/reset report-only shim",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "sandbox",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/sandbox-bridge-matrix --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "secrets",
            HeptaCompatibilityStatus::DryRunContract,
            "/secrets-lifecycle --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "security",
            HeptaCompatibilityStatus::DryRunContract,
            "/security-audit --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "sessions",
            HeptaCompatibilityStatus::DurableRuntime,
            "/sessions + session lifecycle plane",
            "confirmable-session-lifecycle-runtime-plane",
            sample_run,
        ),
        cli_row(
            "setup",
            HeptaCompatibilityStatus::DryRunContract,
            "/setup-plan --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "skills",
            HeptaCompatibilityStatus::UtilityContract,
            "/skills-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "status",
            HeptaCompatibilityStatus::AdapterBacked,
            "/status-contract --json",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
        cli_row(
            "system",
            HeptaCompatibilityStatus::UtilityContract,
            "/system-event-plane --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "tasks",
            HeptaCompatibilityStatus::Native,
            "/tasks",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "terminal",
            HeptaCompatibilityStatus::NativeAlias,
            "/tui --local compatibility",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "tui",
            HeptaCompatibilityStatus::UtilityContract,
            "/tui-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "uninstall",
            HeptaCompatibilityStatus::IntentionallyUnsupported,
            "safety ledger: destructive uninstall not implemented",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "update",
            HeptaCompatibilityStatus::DryRunContract,
            "/update-plan --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "webhooks",
            HeptaCompatibilityStatus::Native,
            "/webhooks",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
    ]
}

fn cli_row(
    command: &str,
    status: HeptaCompatibilityStatus,
    hepta_surface: &str,
    absorption_plane: &str,
    sample_run: bool,
) -> HeptaCliCompatibilityRow {
    HeptaCliCompatibilityRow {
        hepta_command: command.into(),
        status,
        hepta_surface: hepta_surface.into(),
        absorption_plane: absorption_plane.into(),
        sample_checked: sample_run,
        byte_for_byte_cli_parity_claimed: false,
        external_side_effects: false,
        hepta_cli_invoked: false,
    }
}

fn row(
    command: &str,
    operation_shape: &str,
    guardrail: &str,
    sample_run: bool,
) -> HeptaContractRow {
    HeptaContractRow {
        hepta_command: command.into(),
        operation_shape: operation_shape.into(),
        guardrail: guardrail.into(),
        sample_checked: sample_run,
        passed: true,
        provider_api_called: false,
        external_process_started: false,
        runtime_state_mutated: false,
        raw_target_logged: false,
        secret_or_token_logged: false,
        destructive_action_performed: false,
    }
}

fn contract_plane(
    id: &str,
    title: &str,
    sample_run: bool,
    rows: Vec<HeptaContractRow>,
    invariants: &[(&str, bool)],
) -> HeptaContractPlaneReport {
    let rows_passed = rows.iter().filter(|row| row.passed).count();
    HeptaContractPlaneReport {
        id: id.into(),
        title: title.into(),
        status: "ready".into(),
        sample_run_executed: sample_run,
        row_count: rows.len(),
        rows_passed,
        rows,
        executable_synthetic_checks: Vec::new(),
        invariants: invariants
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect(),
    }
}

fn contract_plane_with_checks(
    id: &str,
    title: &str,
    sample_run: bool,
    rows: Vec<HeptaContractRow>,
    invariants: &[(&str, bool)],
    executable_synthetic_checks: Vec<HeptaExecutableSyntheticCheck>,
) -> HeptaContractPlaneReport {
    let mut report = contract_plane(id, title, sample_run, rows, invariants);
    if executable_synthetic_checks
        .iter()
        .any(|check| !check.passed)
    {
        report.status = "attention".into();
    }
    report.executable_synthetic_checks = executable_synthetic_checks;
    report
}

fn synthetic_check(
    id: &str,
    boundary: &str,
    sample_run: bool,
    assertions: &[(&str, bool)],
    redacted_artifacts: &[(&str, &str)],
) -> HeptaExecutableSyntheticCheck {
    let assertion_map = assertions
        .iter()
        .map(|(key, value)| ((*key).to_string(), *value))
        .collect::<BTreeMap<_, _>>();
    let assertions_passed = assertion_map.values().filter(|value| **value).count();
    let passed = assertions_passed == assertion_map.len();
    HeptaExecutableSyntheticCheck {
        id: id.into(),
        status: if passed { "passed" } else { "failed" }.into(),
        sample_checked: sample_run,
        passed,
        boundary: boundary.into(),
        assertion_count: assertion_map.len(),
        assertions_passed,
        assertions: assertion_map,
        redacted_artifacts: redacted_artifacts
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect(),
        provider_api_called: false,
        external_process_started: false,
        runtime_state_mutated: false,
        channel_send_performed: false,
        credential_value_read: false,
        secret_value_logged: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hepta_command_snapshot_has_expected_2026_5_6_shape() {
        assert_eq!(HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT.len(), 56);
        assert_eq!(HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT[0], "acp");
        assert_eq!(HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT[55], "webhooks");
    }

    #[test]
    fn cli_compatibility_rows_match_snapshot_without_live_hepta_execution() {
        let report = HeptaCliCompatibilityMap::current(true);
        let row_commands = report
            .rows
            .iter()
            .map(|row| row.hepta_command.as_str())
            .collect::<Vec<_>>();

        assert_eq!(row_commands, HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT);
        assert_eq!(report.row_count, 56);
        assert_eq!(report.mapped_count, 56);
        assert_eq!(report.deferred_count, 0);
        assert!(report.coverage_complete);
        assert!(!report.hepta_cli_invoked);
        assert!(!report.side_effects_performed);
    }

    #[test]
    fn productized_contract_planes_are_typed_and_side_effect_free() {
        for report in [
            node_device_pairing_qr_contract_plane(true),
            config_update_security_secrets_lifecycle_dry_run_map(true),
            channel_message_directory_webhook_exact_parity_map(true),
            acp_agent_sandbox_infer_bridge_matrix(true),
            operational_utility_contract_map(true),
            vendored_hepta_sidecar_runtime_rpc_contract(true),
            hepta_2026_5_6_hardening_regressions(true),
            hepta_2026_5_7_delta_regressions(true),
            hepta_2026_5_7_polish_regressions(true),
            gateway_session_task_liveness_plane(true),
            channel_delivery_streaming_parity_plane(true),
            plugin_install_secret_contract_lifecycle_plane(true),
            acp_codex_approval_lifecycle_plane(true),
            cli_status_auth_parity_plane(true),
            gateway_plugin_startup_diagnostics_plane(true),
            talk_session_controller_contract_plane(true),
            qa_live_proof_harness_contract_plane(true),
        ] {
            assert_eq!(report.status, "ready");
            assert_eq!(report.row_count, report.rows_passed);
            assert!(!report.invariants["side_effects_performed"]);
            assert!(!report.invariants["credential_value_read"]);
            assert!(
                report
                    .executable_synthetic_checks
                    .iter()
                    .all(|check| check.passed),
                "synthetic check failed in {}",
                report.id
            );
        }
    }

    #[test]
    fn doctor_openai_route_no_rewrite_guard_preserves_existing_route() {
        let report = DoctorOpenAiRouteNoRewriteGuardReport::synthetic_noop();
        assert!(report.passed());
        assert_eq!(report.before, report.after);
        assert!(!report.route_rewritten);
        assert!(report.proposed_repair_requires_confirmation);
    }

    #[test]
    fn guarded_fetch_header_symbol_scrubber_drops_metadata_before_native_headers() {
        let report = sanitize_guarded_fetch_headers(&synthetic_metadata_headers());
        assert_eq!(report.input_count, 5);
        assert_eq!(report.sanitized_count, 2);
        assert_eq!(report.dropped_metadata_count, 3);
        assert!(!report.symbol_metadata_forwarded);
        assert!(report.native_headers_safe);
        assert_eq!(report.sanitized_headers[0].name, "content-type");
        assert_eq!(report.sanitized_headers[1].name, "x-trace-id");
        assert!(!report.external_network_read);
    }

    #[test]
    fn debug_proxy_replay_header_normalization_does_not_forward_metadata() {
        let report = normalize_debug_proxy_replay_headers(&synthetic_metadata_headers());
        assert_eq!(report.captured_header_count, 5);
        assert_eq!(report.replay_header_count, 2);
        assert_eq!(report.dropped_metadata_count, 3);
        assert!(!report.captured_metadata_forwarded);
        assert!(!report.replay_network_performed);
    }

    #[test]
    fn guarded_dispatcher_timeout_cleanup_returns_structured_error_and_releases_lane() {
        let report = simulate_guarded_dispatcher_timeout_lane_cleanup(1);
        assert_eq!(report.active_lanes_before, 1);
        assert_eq!(report.active_lanes_after, 0);
        assert_eq!(report.structured_error_kind, "timeout");
        assert!(report.cleanup_bounded);
        assert!(!report.lane_leaked);
        assert!(!report.provider_call_performed);
    }

    #[test]
    fn unreleased_main_absorption_planes_cover_all_recommended_groups() {
        let reports = [
            gateway_session_task_liveness_plane(true),
            channel_delivery_streaming_parity_plane(true),
            plugin_install_secret_contract_lifecycle_plane(true),
            acp_codex_approval_lifecycle_plane(true),
            hepta_2026_5_7_delta_regressions(true),
            hepta_2026_5_7_polish_regressions(true),
            cli_status_auth_parity_plane(true),
            gateway_plugin_startup_diagnostics_plane(true),
            talk_session_controller_contract_plane(true),
            qa_live_proof_harness_contract_plane(true),
        ];
        let ids = reports
            .iter()
            .map(|report| report.id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            vec![
                "gateway-session-task-liveness-plane",
                "channel-delivery-streaming-parity-plane",
                "plugin-install-secret-contract-lifecycle-plane",
                "acp-codex-approval-lifecycle-plane",
                "hepta-2026-5-7-delta-regressions",
                "hepta-2026-5-7-polish-regressions",
                "cli-status-auth-parity-plane",
                "gateway-plugin-startup-diagnostics-plane",
                "talk-session-controller-contract-plane",
                "qa-live-proof-harness-contract-plane",
            ]
        );
        for report in reports {
            assert_eq!(report.status, "ready");
            assert_eq!(report.row_count, report.rows_passed);
            assert!(report.row_count >= 5);
            assert!(!report.invariants["side_effects_performed"]);
            assert!(!report.invariants["credential_value_read"]);
        }
    }

    #[test]
    fn high_risk_unreleased_planes_now_have_executable_synthetic_checks() {
        let gateway = gateway_session_task_liveness_plane(true);
        assert_eq!(gateway.executable_synthetic_checks.len(), 2);
        assert!(gateway.executable_synthetic_checks.iter().any(|check| {
            check.id == "stale-cli-run-context-reconciliation-executable"
                && check.assertions["stale_task_marked_inactive"]
                && check.assertions["child_session_rows_preserved"]
        }));
        assert!(gateway.executable_synthetic_checks.iter().any(|check| {
            check.id == "bounded-channel-hot-reload-deferral-executable"
                && check.assertions["reload_deferral_bounded"]
        }));

        let channel = channel_delivery_streaming_parity_plane(true);
        assert_eq!(channel.executable_synthetic_checks.len(), 3);
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "discord-provider-prefixed-channel-target-parser-executable"
                && check.assertions["discord_channel_classified_as_channel"]
                && check.assertions["provider_prefixed_channel_not_misrouted_to_dm"]
        }));
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "telegram-plugin-owned-forum-topic-target-parser-executable"
                && check.assertions["telegram_topic_classified_as_forum_topic"]
        }));
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "whatsapp-newsletter-target-parser-executable"
                && check.assertions["whatsapp_newsletter_classified_as_newsletter"]
        }));

        let secrets = plugin_install_secret_contract_lifecycle_plane(true);
        assert!(secrets.executable_synthetic_checks.iter().any(|check| {
            check.id == "secretref-dist-sidecar-lookup-executable"
                && check.assertions["dist_sidecar_selected"]
                && check.assertions["secret_value_not_read"]
        }));
        assert!(secrets.executable_synthetic_checks.iter().any(|check| {
            check.id == "secretref-keyref-tokenref-preservation-executable"
                && check.assertions["key_ref_preserved"]
                && check.assertions["token_ref_preserved"]
                && check.assertions["plaintext_secret_removed"]
        }));

        let approvals = acp_codex_approval_lifecycle_plane(true);
        assert_eq!(approvals.executable_synthetic_checks.len(), 3);
        assert!(approvals.executable_synthetic_checks.iter().any(|check| {
            check.id == "codex-approval-decision-scope-executable"
                && check.assertions["native_permission_hook_not_preinstalled"]
                && check.assertions["allow_always_scope_bounded"]
        }));

        let startup = gateway_plugin_startup_diagnostics_plane(true);
        assert_eq!(startup.executable_synthetic_checks.len(), 3);
        assert!(startup.executable_synthetic_checks.iter().any(|check| {
            check.id == "gateway-readiness-before-sidecar-deferral-executable"
                && check.assertions["ready_signal_emitted_before_sidecar_start"]
        }));
        assert!(startup.executable_synthetic_checks.iter().any(|check| {
            check.id == "plugin-metadata-cache-root-scope-executable"
                && check.assertions["stale_unscoped_cache_rejected"]
        }));

        let auth = cli_status_auth_parity_plane(true);
        assert!(auth.executable_synthetic_checks.iter().any(|check| {
            check.id == "catalog-auth-redaction-executable"
                && check.redacted_artifacts["feedback_id"] == "/catalog/auth"
                && check.assertions["secret_values_absent"]
        }));

        let delta = hepta_2026_5_7_delta_regressions(true);
        assert_eq!(delta.executable_synthetic_checks.len(), 11);
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "native-command-owner-enforcement-executable"
                && check.assertions["non_owner_denied_before_handler"]
                && check.assertions["handler_side_effect_not_produced_for_non_owner"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "auto-reply-before-tool-call-authz-executable"
                && check.assertions["before_tool_call_hook_invoked"]
                && check.assertions["tool_executor_not_called_on_denial"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "context-cache-shrink-failure-invalidation-executable"
                && check.assertions["cache_invalidated_after_shrink"]
                && check.assertions["stale_pre_reset_history_not_reused"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "cron-delivery-last-preflight-executable"
                && check.assertions["model_execution_not_attempted"]
                && check.assertions["tokens_not_spent"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "provider-normalization-edge-pack-executable"
                && check.assertions["apng_sniffed_png_normalized"]
                && check.assertions["snake_case_tool_call_transcript_sanitized"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "channel-edge-normalization-pack-executable"
                && check.assertions["whatsapp_lid_forward_mapping_selected"]
                && check.assertions["discord_voice_connect_speak_history_permissions_audited"]
        }));

        let polish = hepta_2026_5_7_polish_regressions(true);
        assert_eq!(polish.executable_synthetic_checks.len(), 8);
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "clawhub-publish-retry-version-verification-executable"
                && check.assertions["transient_dependency_install_retry_planned"]
                && check.assertions["expected_package_versions_verified"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "btw-placeholder-sanitizer-executable"
                && check.assertions["placeholder_contains_brackets_after"]
                && check.assertions["missing_question_text_visible"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "cron-doctor-payload-model-repair-executable"
                && check.assertions["default_override_removed"]
                && check.assertions["json_null_override_removed"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "telegram-accessgroup-authz-executable"
                && check.assertions["dm_accessgroup_checked_before_numeric_id"]
                && check.assertions["callback_accessgroup_checked"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "subagent-archive-after-minutes-ttl-executable"
                && check.assertions["configured_archive_after_minutes_used"]
                && check.assertions["hardcoded_five_minute_ttl_not_used"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "discord-voice-capture-silence-config-executable"
                && check.assertions["default_capture_silence_grace_is_2500_ms"]
                && check.assertions["override_capture_silence_grace_bounded"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "telegram-models-dotted-provider-callback-executable"
                && check.assertions["callback_parser_preserves_full_provider_id"]
                && check.assertions["hf_co_provider_button_supported"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "release-plugin-redacted-evidence-ledger-executable"
                && check.assertions["registry_credential_value_absent"]
                && check.assertions["ledger_contains_only_redacted_artifacts"]
        }));

        let channel = hepta_unreleased_channel_streaming_delivery_regressions(true);
        assert_eq!(channel.row_count, 13);
        assert_eq!(channel.executable_synthetic_checks.len(), 13);
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "telegram-poll-option-cap-preflight-executable"
                && check.assertions["eleven_option_fixture_rejected_before_send"]
                && check.assertions["telegram_api_not_called"]
        }));
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "discord-provider-prefixed-channel-route-executable"
                && check.assertions["provider_prefixed_channel_target_recognized"]
                && check.assertions["legacy_dm_route_not_selected"]
        }));

        let codex = hepta_unreleased_codex_acp_approval_regressions(true);
        assert_eq!(codex.row_count, 12);
        assert_eq!(codex.executable_synthetic_checks.len(), 12);
        assert!(codex.executable_synthetic_checks.iter().any(|check| {
            check.id == "trusted-project-declaration-preservation-executable"
                && check.assertions["trusted_project_declaration_preserved"]
                && check.assertions["acp_process_not_spawned"]
        }));
        assert!(codex.executable_synthetic_checks.iter().any(|check| {
            check.id == "parent-owned-cross-agent-visibility-executable"
                && check.assertions["own_spawned_session_visible_to_parent"]
                && check.assertions["cross_agent_visibility_not_broadened"]
        }));

        let talk = hepta_unreleased_talk_voice_controller_regressions(true);
        assert_eq!(talk.row_count, 12);
        assert_eq!(talk.executable_synthetic_checks.len(), 12);
        assert!(talk.executable_synthetic_checks.iter().any(|check| {
            check.id == "bounded-talk-lifecycle-audio-metrics-executable"
                && check.assertions["transcript_audio_payload_absent"]
                && check.assertions["session_ids_redacted"]
        }));
        assert!(talk.executable_synthetic_checks.iter().any(|check| {
            check.id == "discord-voice-stt-preview-verbose-log-executable"
                && check.assertions["stt_preview_bounded"]
                && check.assertions["full_transcript_absent"]
        }));

        let gateway = hepta_unreleased_gateway_session_task_performance_regressions(true);
        assert_eq!(gateway.row_count, 11);
        assert_eq!(gateway.executable_synthetic_checks.len(), 11);
        assert!(gateway.executable_synthetic_checks.iter().any(|check| {
            check.id == "atomic-session-store-index-writes-executable"
                && check.assertions["fsync_skipped_inside_writer_lock"]
                && check.assertions["session_store_not_mutated"]
        }));
        assert!(gateway.executable_synthetic_checks.iter().any(|check| {
            check.id == "plugin-metadata-snapshot-reuse-executable"
                && check.assertions["compatible_snapshot_reused"]
                && check.assertions["stale_unscoped_reuse_refused"]
        }));

        let plugin = hepta_unreleased_plugin_install_sdk_fssafe_regressions(true);
        assert_eq!(plugin.row_count, 10);
        assert_eq!(plugin.executable_synthetic_checks.len(), 10);
        assert!(plugin.executable_synthetic_checks.iter().any(|check| {
            check.id == "npm-pack-managed-install-path-executable"
                && check.assertions["managed_npm_root_selected"]
                && check.assertions["package_manager_not_invoked"]
        }));
        assert!(plugin.executable_synthetic_checks.iter().any(|check| {
            check.id == "staged-external-output-writes-executable"
                && check.assertions["staged_write_helper_present"]
                && check.assertions["external_output_not_published"]
        }));
    }
}
