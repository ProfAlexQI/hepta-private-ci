mod contracts;
mod coverage;
mod dispatch;
mod explanation;
mod integrity;
mod intent;
mod native_post;
mod report;
mod resolution;
mod runtime;
mod scaffolding;
mod telegram_config;
mod telegram_cursor;
mod telegram_delivery;
mod telegram_policy;
mod telegram_runtime;
mod telegram_status;
mod telegram_transport;
mod trace;

pub use contracts::{GatewayEnvelope, GatewayRoutePlan, GatewayTransport};
pub use coverage::GatewayPluginResolutionCoverageDigest;
pub use dispatch::{GatewayPluginHandoffDraft, plugin_handoff_draft};
pub use explanation::{GatewayResolutionCoverageGapNote, GatewayResolvedPluginDiagnosticNote};
pub use hepta_runtime::{
    CODEX_ENGINE_ID, DEFAULT_TELEGRAM_MLX_BASE_URL, DEFAULT_TELEGRAM_MLX_MAX_TOKENS,
    DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS, HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT,
    HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND, HEPTA_CORE_FUSION_READINESS_ENDPOINT,
    HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND, HEPTA_KERNEL_CONTRACT, HEPTA_KERNEL_OWNER,
    HEPTA_KERNEL_TELEGRAM_RUNNER_KIND, HEPTA_KERNEL_TELEGRAM_RUNNER_STRATEGY,
    HeptaCodexEngineAdapterBoundaryResponse, HeptaCodexEngineAdapterEnvelope,
    HeptaCodexEngineAdapterEnvelopeInput, HeptaCodexEngineAdapterParityEvidence,
    HeptaCodexEngineAdapterSurface, HeptaCodexEngineAdapterThreadingPlan,
    HeptaCoreFusionForbiddenSideEffects, HeptaCoreFusionReadinessResponse, HeptaKernelEngine,
    HeptaKernelTurnChannel, HeptaKernelTurnInput, HeptaKernelTurnPlan, HeptaKernelTurnStagePlan,
    HeptaProductRuntimeEntrypointInput, HeptaProductRuntimeEntrypointPlan,
    MAX_TELEGRAM_MLX_MAX_TOKENS, MAX_TELEGRAM_MODEL_TIMEOUT_MS, MIN_TELEGRAM_MODEL_TIMEOUT_MS,
    NativeTelegramModelRunnerInvocationOutcome, NativeTelegramModelRunnerPlan,
    classify_native_telegram_model_runner_error, extract_native_telegram_exec_child_final_message,
    extract_native_telegram_openai_chat_completion_text,
    hepta_codex_engine_adapter_boundary_report, hepta_codex_legacy_tui_cli_adapter_envelope,
    hepta_codex_legacy_tui_cli_adapter_threading_plan, hepta_codex_mcp_app_server_adapter_envelope,
    hepta_codex_mcp_app_server_adapter_threading_plan, hepta_codex_model_provider_adapter_envelope,
    hepta_codex_model_provider_adapter_threading_plan, hepta_codex_sandbox_exec_adapter_envelope,
    hepta_codex_sandbox_exec_adapter_threading_plan,
    hepta_codex_session_thread_store_adapter_envelope,
    hepta_codex_session_thread_store_adapter_threading_plan,
    hepta_codex_tool_invocation_adapter_envelope,
    hepta_codex_tool_invocation_adapter_threading_plan, hepta_core_fusion_readiness_report,
    hepta_kernel_telegram_prompt, hepta_product_runtime_entrypoint_plan,
    invoke_native_telegram_model_runner_with_plan, native_telegram_codex_core_prompt,
    native_telegram_exec_child_args, native_telegram_exec_child_status_error,
    native_telegram_hepta_kernel_prompt, native_telegram_mlx_chat_completion_body,
    native_telegram_model_timeout, parse_native_telegram_mlx_model_ref,
    plan_hepta_kernel_telegram_session_bridge, plan_hepta_kernel_turn,
    redact_native_telegram_model_runner_error, select_native_telegram_model_runner,
    wait_for_native_telegram_model_child,
};
pub use integrity::{
    GATEWAY_DISPATCH_READY, GATEWAY_RESOLUTION_SNAPSHOT_CONSISTENT, GATEWAY_TRANSPORT_SUPPORTED,
    GatewayDispatchReadinessReport, GatewayRouteIntegritySnapshot,
};
pub use intent::GatewayHandoffLookupIntentNote;
pub use native_post::{
    DEFAULT_NATIVE_POST_EXECUTION_STORE_DIR, DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS,
    DEFAULT_NATIVE_POST_STORE_MAX_BYTES, DEFAULT_NATIVE_POST_STORE_MAX_LINES,
    NATIVE_POST_ACTIVATION_PLAN_ENDPOINT, NATIVE_POST_EXECUTION_READINESS_ENDPOINT,
    NATIVE_POST_EXECUTION_STORE_DIR_ENV, NATIVE_POST_EXECUTION_STORES_ENDPOINT,
    NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT, NATIVE_POST_MAX_BODY_BYTES,
    NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV, NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
    NATIVE_POST_REAL_HANDLER_PLAN_KINDS, NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
    NATIVE_POST_REAL_HANDLERS_ENV, NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT,
    NATIVE_POST_STORE_MAX_BYTES_ENV, NATIVE_POST_STORE_MAX_LINES_ENV, NativePostActivationGate,
    NativePostActivationPlanResponse, NativePostAuditEventContract, NativePostBodyAdmission,
    NativePostBodySchema, NativePostConfirmationContract, NativePostExecutionAdmission,
    NativePostExecutionReadinessResponse, NativePostExecutionReadinessRoute,
    NativePostExecutionStoreFileStatus, NativePostExecutionStoreLimits,
    NativePostExecutionStoreRecord, NativePostExecutionStoreWriteReport,
    NativePostExecutionStoresResponse, NativePostGrayReleaseEvidenceResponse,
    NativePostIdempotencyEvidence, NativePostPlanResponse, NativePostPlanRouteSpec,
    NativePostRealHandlerHarness, NativePostRealHandlerObservation, NativePostRollbackContract,
    NativePostRolloutEvidencePlanKindCount, NativePostRolloutEvidenceRecordSummary,
    NativePostRolloutEvidenceResponse, NativePostSelectedHandlerRolloutEvidence,
    NativePostStoreEffectProjection, native_post_activation_plan_report,
    native_post_audit_event_contract, native_post_body_admission, native_post_body_schema,
    native_post_confirmation_contract, native_post_dispatch_plan_report,
    native_post_duplicate_check_required, native_post_execution_admission_with_scope,
    native_post_execution_readiness_report,
    native_post_execution_store_capacity_allows_append_with_limits,
    native_post_execution_store_contracts_ready, native_post_execution_store_record,
    native_post_execution_stores_report, native_post_gray_release_evidence_report,
    native_post_idempotency_evidence, native_post_plan_kind_has_real_handler,
    native_post_plan_parameter, native_post_plan_report, native_post_plan_route_specs,
    native_post_rate_limit_check_required, native_post_real_handler_harness,
    native_post_real_handler_harness_from_observation, native_post_real_handler_scope_matches,
    native_post_real_handler_scope_selected_kinds, native_post_redacted_fingerprint,
    native_post_rollback_contract, native_post_rollout_evidence_report,
    native_post_store_capacity_check_required, native_post_store_effect_projection,
    native_post_store_write_attempt_required, persist_native_post_execution_store_record,
};
pub use report::GatewayPluginResolutionContractReport;
pub use resolution::{
    GatewayPluginResolutionSnapshot, GatewayResolvedPluginCandidate, GatewayResolvedPluginTier,
};
pub use runtime::{
    GatewayAdapter, GatewayAdapterDescriptor, GatewayAdapterSendResult, GatewayDeliveryLedger,
    GatewayDeliveryRecord, GatewayDeliveryState, GatewayDeterministicAdapter,
    GatewayDeterministicDispatcher, GatewayDispatchOutcome, GatewayFrameBridgeReport,
    GatewayRetryPolicy, GatewayRuntime, GatewayRuntimeReadinessReport, GatewayRuntimeStatusReport,
    default_gateway_adapters, envelope_from_inbound_message, frame_bridge_report,
};
pub use scaffolding::{
    GatewayResolutionBindingScaffoldNote, GatewayResolutionPluginScaffoldStub,
    GatewayResolutionScaffoldPlan,
};
pub use telegram_config::{
    NativeTelegramConfigMetadata, NativeTelegramConfigStatus, NativeTelegramConfigStatusInput,
    NativeTelegramTokenObservation, NativeTelegramTokenObservationInput,
    build_native_telegram_config_status, extract_native_telegram_config_metadata,
    normalize_telegram_binding_id, parse_telegram_env_truthy_value, parse_telegram_env_u64_value,
    resolve_native_telegram_token_observation, resolve_telegram_secret_provider_path,
};
pub use telegram_cursor::{
    DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH, NativeTelegramCursorPlan, NativeTelegramCursorStatus,
    parse_telegram_cursor_next_update_offset, telegram_cursor_status,
    telegram_cursor_status_from_path, write_telegram_cursor_next_update_offset,
};
pub use telegram_delivery::{
    NativeTelegramDeliveryLedgerStatus, TELEGRAM_DELIVERY_MAX_RETRIES,
    TELEGRAM_DELIVERY_STORE_IDENTIFIER, append_telegram_delivery_lifecycle_record,
    telegram_delivery_backoff_ms, telegram_delivery_error_is_permanent,
    telegram_delivery_ledger_status, telegram_delivery_ledger_status_from_path,
    telegram_delivery_lifecycle_record,
};
pub use telegram_policy::{
    NativeTelegramCandidateMaterial, NativeTelegramDuplicateDecision, NativeTelegramExecutionPlan,
    NativeTelegramGatewayGateSummary, NativeTelegramGatewayGateSummaryInput,
    NativeTelegramIngressInspection, NativeTelegramModelExecutionReport,
    NativeTelegramModelInvocationRequestPlan, NativeTelegramModelTurnPlan,
    NativeTelegramReplyTargetMaterial, NativeTelegramSendExecutionReport,
    NativeTelegramSendRequestPlan, TELEGRAM_DRAIN_ONCE_STAGES, build_model_invocation_request_plan,
    build_telegram_gateway_gate_summary, extract_telegram_candidate_material,
    first_model_candidate_with_duplicate_decision, inspect_telegram_updates,
    plan_model_turn_for_updates, telegram_drain_execution_plan, telegram_drain_first_missing_gate,
    telegram_drain_status_probe_executes_pipeline, telegram_duplicate_decision,
    telegram_message_has_reply_target, telegram_message_is_reply_candidate,
    telegram_message_text_present, telegram_next_update_offset, telegram_update_already_drained,
};
pub use telegram_runtime::{
    NATIVE_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE, NativeTelegramDrainPipelineFinalStatus,
    NativeTelegramDrainPipelineInput, NativeTelegramDrainPipelineOutcome,
    NativeTelegramModelExecutionInput, NativeTelegramModelExecutionOutcome,
    NativeTelegramSessionBridgePlan, execute_telegram_drain_pipeline_for_updates,
    execute_telegram_model_turn_after_candidate, finalize_telegram_drain_pipeline_status,
    native_telegram_model_failure_fallback_message,
};
pub use telegram_status::{
    DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION, DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS,
    DEFAULT_TELEGRAM_SOAK_MIN_POLLS, MAX_TELEGRAM_POLL_LOOP_INTERVAL_MS,
    MAX_TELEGRAM_SOAK_MAX_ATTENTION, MAX_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS,
    MAX_TELEGRAM_SOAK_MIN_POLLS, MIN_TELEGRAM_POLL_LOOP_INTERVAL_MS,
    NativeTelegramDrainOnceApiResultInput, NativeTelegramDrainOnceApiResultPlan,
    NativeTelegramDrainOncePreflightInput, NativeTelegramDrainOncePreflightPlan,
    NativeTelegramDrainOnceShellReadinessInput, NativeTelegramDrainOnceShellReadinessPlan,
    NativeTelegramDrainOnceStatus, NativeTelegramDrainOnceStatusInput,
    NativeTelegramLiveSoakObservationReport, NativeTelegramLiveSoakObservationState,
    NativeTelegramLiveSoakStatus, NativeTelegramLiveSoakStatusInput,
    NativeTelegramModelBridgeStatus, NativeTelegramModelBridgeStatusInput,
    NativeTelegramModelTurnPlanStatus, NativeTelegramModelTurnPlanStatusInput,
    NativeTelegramPluginStatus, NativeTelegramPluginStatusInput, NativeTelegramPollLoopStatus,
    NativeTelegramPollLoopStatusInput, NativeTelegramProductionGuardPolicyInput,
    NativeTelegramProductionGuardStatus, NativeTelegramProductionGuardStatusInput,
    NativeTelegramProductionReadinessInput, NativeTelegramProductionReadinessStatus,
    NativeTelegramReceiveOnceApiResultInput, NativeTelegramReceiveOnceErrorInput,
    NativeTelegramReceiveOncePreflightInput, NativeTelegramReceiveOnceShellReadinessInput,
    NativeTelegramReceiveOnceShellReadinessPlan, NativeTelegramReceiveOnceStatus,
    NativeTelegramReceiveOnceStatusInput, NativeTelegramSendPlanStatus,
    NativeTelegramSendPlanStatusInput, build_telegram_drain_once_status,
    build_telegram_live_soak_status, build_telegram_model_bridge_status,
    build_telegram_model_turn_plan_status, build_telegram_plugin_status,
    build_telegram_poll_loop_status, build_telegram_production_guard_status,
    build_telegram_production_guard_status_from_policy, build_telegram_production_readiness_status,
    build_telegram_receive_once_error_status, build_telegram_receive_once_status,
    build_telegram_receive_once_status_from_api_result, build_telegram_send_plan_status,
    plan_telegram_drain_once_api_result, plan_telegram_drain_once_preflight,
    plan_telegram_drain_once_shell_readiness, plan_telegram_receive_once_preflight_status,
    plan_telegram_receive_once_shell_readiness, telegram_poll_loop_interval_ms_policy,
    telegram_poll_loop_should_spawn, telegram_receive_limit_policy,
    telegram_soak_max_attention_count_policy, telegram_soak_max_observed_age_ms_policy,
    telegram_soak_min_poll_iterations_policy, telegram_system_time_unix_ms,
};
pub use telegram_transport::{
    DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS, DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS,
    DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS, DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS,
    DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS, MAX_TELEGRAM_READ_MAX_ATTEMPTS,
    MAX_TELEGRAM_READ_RETRY_BACKOFF_MS, MAX_TELEGRAM_SEND_MAX_ATTEMPTS,
    MAX_TELEGRAM_SEND_MIN_INTERVAL_MS, MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS,
    MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS, NativeTelegramSendExecutionInput,
    NativeTelegramSendPlan, NativeTelegramTransportPlan, TELEGRAM_ALLOWED_UPDATES,
    TelegramTypingKeepalive, execute_telegram_send_after_model_output, telegram_bot_token_shape_ok,
    telegram_call_get_updates_once, telegram_call_send_chat_action, telegram_call_send_message,
    telegram_get_updates_error_is_conflict, telegram_get_updates_error_is_transient,
    telegram_get_updates_query, telegram_get_updates_should_retry, telegram_get_updates_with_retry,
    telegram_read_max_attempts_policy, telegram_read_retry_backoff_policy,
    telegram_redact_token_like_text, telegram_send_chat_action_request_body,
    telegram_send_error_is_transient, telegram_send_max_attempts_policy,
    telegram_send_message_request_body, telegram_send_min_interval_policy,
    telegram_send_rate_limit_sleep_for, telegram_send_retry_backoff_policy,
    telegram_send_should_retry, telegram_start_typing_keepalive,
    telegram_transport_plan_for_config_status, telegram_typing_keepalive_interval_policy,
    telegram_typing_keepalive_should_start, telegram_wait_for_send_rate_limit,
};
pub use trace::GatewayResolutionLookupTraceStep;

pub struct GatewaySurface;

impl GatewaySurface {
    pub fn id(&self) -> &'static str {
        "gateway"
    }

    pub fn supports_transport(&self, transport: GatewayTransport) -> bool {
        match transport {
            GatewayTransport::Cli | GatewayTransport::Webhook | GatewayTransport::Queue => true,
        }
    }

    pub fn route_plan(&self, envelope: &GatewayEnvelope) -> GatewayRoutePlan {
        GatewayRoutePlan::new(
            envelope.surface_id.trim(),
            self.session_key(envelope),
            envelope.transport,
            envelope.payload_text.trim(),
        )
    }

    pub fn plugin_handoff_draft(&self, envelope: &GatewayEnvelope) -> GatewayPluginHandoffDraft {
        GatewayPluginHandoffDraft::from_route(&self.route_plan(envelope))
    }

    fn session_key(&self, envelope: &GatewayEnvelope) -> String {
        if let Some(session_hint) = envelope
            .session_hint
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return session_hint.to_string();
        }

        let surface_id = envelope.surface_id.trim();
        let user_id = envelope.user_id.trim();
        if user_id.is_empty() {
            format!("{}:anonymous", surface_id)
        } else {
            format!("{}:user:{}", surface_id, user_id)
        }
    }
}

impl Default for GatewaySurface {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::{GatewayEnvelope, GatewaySurface, GatewayTransport};

    #[test]
    fn surface_id_is_stable() {
        assert_eq!(GatewaySurface.id(), "gateway");
    }

    #[test]
    fn route_plan_prefers_explicit_session_hint() {
        let surface = GatewaySurface;
        let envelope =
            GatewayEnvelope::new("hepta", "user-7", GatewayTransport::Webhook, "  /status  ")
                .with_session_hint("session-build-42");

        let plan = surface.route_plan(&envelope);

        assert_eq!(plan.surface_id, "hepta");
        assert_eq!(plan.session_key, "session-build-42");
        assert_eq!(plan.normalized_text, "/status");
        assert_eq!(plan.transport, GatewayTransport::Webhook);
    }

    #[test]
    fn route_plan_derives_session_key_from_user_when_needed() {
        let surface = GatewaySurface;
        let envelope = GatewayEnvelope::new("telegram", "user-9", GatewayTransport::Cli, "hello");

        let plan = surface.route_plan(&envelope);

        assert_eq!(plan.session_key, "telegram:user:user-9");
    }

    #[test]
    fn route_plan_falls_back_to_anonymous_when_user_missing() {
        let surface = GatewaySurface;
        let envelope = GatewayEnvelope::new("discord", "   ", GatewayTransport::Cli, "ping");

        let plan = surface.route_plan(&envelope);

        assert_eq!(plan.session_key, "discord:anonymous");
    }

    #[test]
    fn supported_transports_remain_explicit() {
        let surface = GatewaySurface;

        assert!(surface.supports_transport(GatewayTransport::Cli));
        assert!(surface.supports_transport(GatewayTransport::Webhook));
        assert!(surface.supports_transport(GatewayTransport::Queue));
    }

    #[test]
    fn plugin_handoff_draft_reuses_route_normalization() {
        let surface = GatewaySurface;
        let envelope = GatewayEnvelope::new(
            "hepta",
            "user-7",
            GatewayTransport::Webhook,
            "  /status   --json  ",
        );

        let draft = surface.plugin_handoff_draft(&envelope);

        assert_eq!(draft.surface_id, "hepta");
        assert_eq!(draft.session_key, "hepta:user:user-7");
        assert_eq!(draft.normalized_text, "/status   --json");
        assert_eq!(draft.command_selector.as_deref(), Some("/status"));
    }
}
