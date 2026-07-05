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

pub use contracts::GatewayEnvelope;
pub use contracts::GatewayRoutePlan;
pub use contracts::GatewayTransport;
pub use coverage::GatewayPluginResolutionCoverageDigest;
pub use dispatch::GatewayPluginHandoffDraft;
pub use dispatch::plugin_handoff_draft;
pub use explanation::GatewayResolutionCoverageGapNote;
pub use explanation::GatewayResolvedPluginDiagnosticNote;
pub use hepta_runtime::CODEX_ENGINE_ID;
pub use hepta_runtime::DEFAULT_TELEGRAM_MLX_BASE_URL;
pub use hepta_runtime::DEFAULT_TELEGRAM_MLX_MAX_TOKENS;
pub use hepta_runtime::DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS;
pub use hepta_runtime::HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT;
pub use hepta_runtime::HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND;
pub use hepta_runtime::HEPTA_CORE_FUSION_READINESS_ENDPOINT;
pub use hepta_runtime::HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND;
pub use hepta_runtime::HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT;
pub use hepta_runtime::HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND;
pub use hepta_runtime::HEPTA_ENGINE_DEPENDENCY_CLOSURE_ENDPOINT;
pub use hepta_runtime::HEPTA_ENGINE_DEPENDENCY_CLOSURE_SOURCE_COMMAND;
pub use hepta_runtime::HEPTA_KERNEL_CONTRACT;
pub use hepta_runtime::HEPTA_KERNEL_OWNER;
pub use hepta_runtime::HEPTA_KERNEL_TELEGRAM_RUNNER_KIND;
pub use hepta_runtime::HEPTA_KERNEL_TELEGRAM_RUNNER_STRATEGY;
pub use hepta_runtime::HEPTA_NAME_REPOSITORY_CLOSURE_ENDPOINT;
pub use hepta_runtime::HEPTA_NAME_REPOSITORY_CLOSURE_SOURCE_COMMAND;
pub use hepta_runtime::HeptaCodexEngineAdapterBoundaryResponse;
pub use hepta_runtime::HeptaCodexEngineAdapterEnvelope;
pub use hepta_runtime::HeptaCodexEngineAdapterEnvelopeInput;
pub use hepta_runtime::HeptaCodexEngineAdapterParityEvidence;
pub use hepta_runtime::HeptaCodexEngineAdapterShadowReplayResult;
pub use hepta_runtime::HeptaCodexEngineAdapterSurface;
pub use hepta_runtime::HeptaCodexEngineAdapterThreadingPlan;
pub use hepta_runtime::HeptaCoreFusionForbiddenSideEffects;
pub use hepta_runtime::HeptaCoreFusionReadinessResponse;
pub use hepta_runtime::HeptaEngineDependencyClosureResponse;
pub use hepta_runtime::HeptaEngineDependencyClosureSurface;
pub use hepta_runtime::HeptaKernelEngine;
pub use hepta_runtime::HeptaKernelTurnChannel;
pub use hepta_runtime::HeptaKernelTurnInput;
pub use hepta_runtime::HeptaKernelTurnPlan;
pub use hepta_runtime::HeptaKernelTurnStagePlan;
pub use hepta_runtime::HeptaNameRepositoryClosureResponse;
pub use hepta_runtime::HeptaNameRepositoryClosureSurface;
pub use hepta_runtime::HeptaProductRuntimeEntrypointInput;
pub use hepta_runtime::HeptaProductRuntimeEntrypointPlan;
pub use hepta_runtime::MAX_TELEGRAM_MLX_MAX_TOKENS;
pub use hepta_runtime::MAX_TELEGRAM_MODEL_TIMEOUT_MS;
pub use hepta_runtime::MIN_TELEGRAM_MODEL_TIMEOUT_MS;
pub use hepta_runtime::NativeTelegramModelRunnerInvocationOutcome;
pub use hepta_runtime::NativeTelegramModelRunnerPlan;
pub use hepta_runtime::classify_native_telegram_model_runner_error;
pub use hepta_runtime::extract_native_telegram_exec_child_final_message;
pub use hepta_runtime::extract_native_telegram_openai_chat_completion_text;
pub use hepta_runtime::hepta_codex_engine_adapter_boundary_report;
pub use hepta_runtime::hepta_codex_legacy_tui_cli_adapter_envelope;
pub use hepta_runtime::hepta_codex_legacy_tui_cli_adapter_shadow_replay;
pub use hepta_runtime::hepta_codex_legacy_tui_cli_adapter_threading_plan;
pub use hepta_runtime::hepta_codex_mcp_app_server_adapter_envelope;
pub use hepta_runtime::hepta_codex_mcp_app_server_adapter_shadow_replay;
pub use hepta_runtime::hepta_codex_mcp_app_server_adapter_threading_plan;
pub use hepta_runtime::hepta_codex_model_provider_adapter_envelope;
pub use hepta_runtime::hepta_codex_model_provider_adapter_shadow_replay;
pub use hepta_runtime::hepta_codex_model_provider_adapter_threading_plan;
pub use hepta_runtime::hepta_codex_sandbox_exec_adapter_envelope;
pub use hepta_runtime::hepta_codex_sandbox_exec_adapter_shadow_replay;
pub use hepta_runtime::hepta_codex_sandbox_exec_adapter_threading_plan;
pub use hepta_runtime::hepta_codex_session_thread_store_adapter_envelope;
pub use hepta_runtime::hepta_codex_session_thread_store_adapter_shadow_replay;
pub use hepta_runtime::hepta_codex_session_thread_store_adapter_threading_plan;
pub use hepta_runtime::hepta_codex_tool_invocation_adapter_envelope;
pub use hepta_runtime::hepta_codex_tool_invocation_adapter_shadow_replay;
pub use hepta_runtime::hepta_codex_tool_invocation_adapter_threading_plan;
pub use hepta_runtime::hepta_core_fusion_readiness_report;
pub use hepta_runtime::hepta_engine_dependency_closure_report;
pub use hepta_runtime::hepta_kernel_telegram_prompt;
pub use hepta_runtime::hepta_name_repository_closure_report;
pub use hepta_runtime::hepta_product_runtime_entrypoint_plan;
pub use hepta_runtime::invoke_native_telegram_model_runner_with_plan;
pub use hepta_runtime::native_telegram_codex_core_prompt;
pub use hepta_runtime::native_telegram_exec_child_args;
pub use hepta_runtime::native_telegram_exec_child_status_error;
pub use hepta_runtime::native_telegram_hepta_kernel_prompt;
pub use hepta_runtime::native_telegram_mlx_chat_completion_body;
pub use hepta_runtime::native_telegram_model_timeout;
pub use hepta_runtime::parse_native_telegram_mlx_model_ref;
pub use hepta_runtime::plan_hepta_kernel_telegram_session_bridge;
pub use hepta_runtime::plan_hepta_kernel_turn;
pub use hepta_runtime::redact_native_telegram_model_runner_error;
pub use hepta_runtime::select_native_telegram_model_runner;
pub use hepta_runtime::wait_for_native_telegram_model_child;
pub use integrity::GATEWAY_DISPATCH_READY;
pub use integrity::GATEWAY_RESOLUTION_SNAPSHOT_CONSISTENT;
pub use integrity::GATEWAY_TRANSPORT_SUPPORTED;
pub use integrity::GatewayDispatchReadinessReport;
pub use integrity::GatewayRouteIntegritySnapshot;
pub use intent::GatewayHandoffLookupIntentNote;
pub use native_post::DEFAULT_NATIVE_POST_EXECUTION_STORE_DIR;
pub use native_post::DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS;
pub use native_post::DEFAULT_NATIVE_POST_STORE_MAX_BYTES;
pub use native_post::DEFAULT_NATIVE_POST_STORE_MAX_LINES;
pub use native_post::NATIVE_POST_ACTIVATION_PLAN_ENDPOINT;
pub use native_post::NATIVE_POST_EXECUTION_READINESS_ENDPOINT;
pub use native_post::NATIVE_POST_EXECUTION_STORE_DIR_ENV;
pub use native_post::NATIVE_POST_EXECUTION_STORES_ENDPOINT;
pub use native_post::NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT;
pub use native_post::NATIVE_POST_MAX_BODY_BYTES;
pub use native_post::NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV;
pub use native_post::NATIVE_POST_REAL_HANDLER_APPROVAL_ENV;
pub use native_post::NATIVE_POST_REAL_HANDLER_PLAN_KINDS;
pub use native_post::NATIVE_POST_REAL_HANDLER_SCOPE_ENV;
pub use native_post::NATIVE_POST_REAL_HANDLERS_ENV;
pub use native_post::NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT;
pub use native_post::NATIVE_POST_STORE_MAX_BYTES_ENV;
pub use native_post::NATIVE_POST_STORE_MAX_LINES_ENV;
pub use native_post::NativePostActivationGate;
pub use native_post::NativePostActivationPlanResponse;
pub use native_post::NativePostAuditEventContract;
pub use native_post::NativePostBodyAdmission;
pub use native_post::NativePostBodySchema;
pub use native_post::NativePostConfirmationContract;
pub use native_post::NativePostExecutionAdmission;
pub use native_post::NativePostExecutionReadinessResponse;
pub use native_post::NativePostExecutionReadinessRoute;
pub use native_post::NativePostExecutionStoreFileStatus;
pub use native_post::NativePostExecutionStoreLimits;
pub use native_post::NativePostExecutionStoreRecord;
pub use native_post::NativePostExecutionStoreWriteReport;
pub use native_post::NativePostExecutionStoresResponse;
pub use native_post::NativePostGrayReleaseEvidenceResponse;
pub use native_post::NativePostIdempotencyEvidence;
pub use native_post::NativePostPlanResponse;
pub use native_post::NativePostPlanRouteSpec;
pub use native_post::NativePostRealHandlerHarness;
pub use native_post::NativePostRealHandlerObservation;
pub use native_post::NativePostRollbackContract;
pub use native_post::NativePostRolloutEvidencePlanKindCount;
pub use native_post::NativePostRolloutEvidenceRecordSummary;
pub use native_post::NativePostRolloutEvidenceResponse;
pub use native_post::NativePostSelectedHandlerRolloutEvidence;
pub use native_post::NativePostStoreEffectProjection;
pub use native_post::native_post_activation_plan_report;
pub use native_post::native_post_audit_event_contract;
pub use native_post::native_post_body_admission;
pub use native_post::native_post_body_schema;
pub use native_post::native_post_confirmation_contract;
pub use native_post::native_post_dispatch_plan_report;
pub use native_post::native_post_duplicate_check_required;
pub use native_post::native_post_execution_admission_with_scope;
pub use native_post::native_post_execution_readiness_report;
pub use native_post::native_post_execution_store_capacity_allows_append_with_limits;
pub use native_post::native_post_execution_store_contracts_ready;
pub use native_post::native_post_execution_store_record;
pub use native_post::native_post_execution_stores_report;
pub use native_post::native_post_gray_release_evidence_report;
pub use native_post::native_post_idempotency_evidence;
pub use native_post::native_post_plan_kind_has_real_handler;
pub use native_post::native_post_plan_parameter;
pub use native_post::native_post_plan_report;
pub use native_post::native_post_plan_route_specs;
pub use native_post::native_post_rate_limit_check_required;
pub use native_post::native_post_real_handler_harness;
pub use native_post::native_post_real_handler_harness_from_observation;
pub use native_post::native_post_real_handler_scope_matches;
pub use native_post::native_post_real_handler_scope_selected_kinds;
pub use native_post::native_post_redacted_fingerprint;
pub use native_post::native_post_rollback_contract;
pub use native_post::native_post_rollout_evidence_report;
pub use native_post::native_post_store_capacity_check_required;
pub use native_post::native_post_store_effect_projection;
pub use native_post::native_post_store_write_attempt_required;
pub use native_post::persist_native_post_execution_store_record;
pub use report::GatewayPluginResolutionContractReport;
pub use resolution::GatewayPluginResolutionSnapshot;
pub use resolution::GatewayResolvedPluginCandidate;
pub use resolution::GatewayResolvedPluginTier;
pub use runtime::GatewayAdapter;
pub use runtime::GatewayAdapterDescriptor;
pub use runtime::GatewayAdapterSendResult;
pub use runtime::GatewayDeliveryLedger;
pub use runtime::GatewayDeliveryRecord;
pub use runtime::GatewayDeliveryState;
pub use runtime::GatewayDeterministicAdapter;
pub use runtime::GatewayDeterministicDispatcher;
pub use runtime::GatewayDispatchOutcome;
pub use runtime::GatewayFrameBridgeReport;
pub use runtime::GatewayRetryPolicy;
pub use runtime::GatewayRuntime;
pub use runtime::GatewayRuntimeReadinessReport;
pub use runtime::GatewayRuntimeStatusReport;
pub use runtime::default_gateway_adapters;
pub use runtime::envelope_from_inbound_message;
pub use runtime::frame_bridge_report;
pub use scaffolding::GatewayResolutionBindingScaffoldNote;
pub use scaffolding::GatewayResolutionPluginScaffoldStub;
pub use scaffolding::GatewayResolutionScaffoldPlan;
pub use telegram_config::NativeTelegramConfigMetadata;
pub use telegram_config::NativeTelegramConfigStatus;
pub use telegram_config::NativeTelegramConfigStatusInput;
pub use telegram_config::NativeTelegramTokenObservation;
pub use telegram_config::NativeTelegramTokenObservationInput;
pub use telegram_config::build_native_telegram_config_status;
pub use telegram_config::extract_native_telegram_config_metadata;
pub use telegram_config::normalize_telegram_binding_id;
pub use telegram_config::parse_telegram_env_truthy_value;
pub use telegram_config::parse_telegram_env_u64_value;
pub use telegram_config::resolve_native_telegram_token_observation;
pub use telegram_config::resolve_telegram_secret_provider_path;
pub use telegram_cursor::DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH;
pub use telegram_cursor::NativeTelegramCursorPlan;
pub use telegram_cursor::NativeTelegramCursorStatus;
pub use telegram_cursor::parse_telegram_cursor_next_update_offset;
pub use telegram_cursor::telegram_cursor_status;
pub use telegram_cursor::telegram_cursor_status_from_path;
pub use telegram_cursor::write_telegram_cursor_next_update_offset;
pub use telegram_delivery::NativeTelegramDeliveryLedgerStatus;
pub use telegram_delivery::TELEGRAM_DELIVERY_MAX_RETRIES;
pub use telegram_delivery::TELEGRAM_DELIVERY_STORE_IDENTIFIER;
pub use telegram_delivery::append_telegram_delivery_lifecycle_record;
pub use telegram_delivery::telegram_delivery_backoff_ms;
pub use telegram_delivery::telegram_delivery_error_is_permanent;
pub use telegram_delivery::telegram_delivery_ledger_status;
pub use telegram_delivery::telegram_delivery_ledger_status_from_path;
pub use telegram_delivery::telegram_delivery_lifecycle_record;
pub use telegram_policy::NativeTelegramCandidateMaterial;
pub use telegram_policy::NativeTelegramDuplicateDecision;
pub use telegram_policy::NativeTelegramExecutionPlan;
pub use telegram_policy::NativeTelegramGatewayGateSummary;
pub use telegram_policy::NativeTelegramGatewayGateSummaryInput;
pub use telegram_policy::NativeTelegramIngressInspection;
pub use telegram_policy::NativeTelegramModelExecutionReport;
pub use telegram_policy::NativeTelegramModelInvocationRequestPlan;
pub use telegram_policy::NativeTelegramModelTurnPlan;
pub use telegram_policy::NativeTelegramReplyTargetMaterial;
pub use telegram_policy::NativeTelegramSendExecutionReport;
pub use telegram_policy::NativeTelegramSendRequestPlan;
pub use telegram_policy::TELEGRAM_DRAIN_ONCE_STAGES;
pub use telegram_policy::build_model_invocation_request_plan;
pub use telegram_policy::build_telegram_gateway_gate_summary;
pub use telegram_policy::extract_telegram_candidate_material;
pub use telegram_policy::first_model_candidate_with_duplicate_decision;
pub use telegram_policy::inspect_telegram_updates;
pub use telegram_policy::plan_model_turn_for_updates;
pub use telegram_policy::telegram_drain_execution_plan;
pub use telegram_policy::telegram_drain_first_missing_gate;
pub use telegram_policy::telegram_drain_status_probe_executes_pipeline;
pub use telegram_policy::telegram_duplicate_decision;
pub use telegram_policy::telegram_message_has_reply_target;
pub use telegram_policy::telegram_message_is_reply_candidate;
pub use telegram_policy::telegram_message_text_present;
pub use telegram_policy::telegram_next_update_offset;
pub use telegram_policy::telegram_update_already_drained;
pub use telegram_runtime::NATIVE_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE;
pub use telegram_runtime::NativeTelegramDrainPipelineFinalStatus;
pub use telegram_runtime::NativeTelegramDrainPipelineInput;
pub use telegram_runtime::NativeTelegramDrainPipelineOutcome;
pub use telegram_runtime::NativeTelegramModelExecutionInput;
pub use telegram_runtime::NativeTelegramModelExecutionOutcome;
pub use telegram_runtime::NativeTelegramSessionBridgePlan;
pub use telegram_runtime::execute_telegram_drain_pipeline_for_updates;
pub use telegram_runtime::execute_telegram_model_turn_after_candidate;
pub use telegram_runtime::finalize_telegram_drain_pipeline_status;
pub use telegram_runtime::native_telegram_model_failure_fallback_message;
pub use telegram_status::DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION;
pub use telegram_status::DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS;
pub use telegram_status::DEFAULT_TELEGRAM_SOAK_MIN_POLLS;
pub use telegram_status::MAX_TELEGRAM_POLL_LOOP_INTERVAL_MS;
pub use telegram_status::MAX_TELEGRAM_SOAK_MAX_ATTENTION;
pub use telegram_status::MAX_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS;
pub use telegram_status::MAX_TELEGRAM_SOAK_MIN_POLLS;
pub use telegram_status::MIN_TELEGRAM_POLL_LOOP_INTERVAL_MS;
pub use telegram_status::NativeTelegramDrainOnceApiResultInput;
pub use telegram_status::NativeTelegramDrainOnceApiResultPlan;
pub use telegram_status::NativeTelegramDrainOncePreflightInput;
pub use telegram_status::NativeTelegramDrainOncePreflightPlan;
pub use telegram_status::NativeTelegramDrainOnceShellReadinessInput;
pub use telegram_status::NativeTelegramDrainOnceShellReadinessPlan;
pub use telegram_status::NativeTelegramDrainOnceStatus;
pub use telegram_status::NativeTelegramDrainOnceStatusInput;
pub use telegram_status::NativeTelegramLiveSoakObservationReport;
pub use telegram_status::NativeTelegramLiveSoakObservationState;
pub use telegram_status::NativeTelegramLiveSoakStatus;
pub use telegram_status::NativeTelegramLiveSoakStatusInput;
pub use telegram_status::NativeTelegramModelBridgeStatus;
pub use telegram_status::NativeTelegramModelBridgeStatusInput;
pub use telegram_status::NativeTelegramModelTurnPlanStatus;
pub use telegram_status::NativeTelegramModelTurnPlanStatusInput;
pub use telegram_status::NativeTelegramPluginStatus;
pub use telegram_status::NativeTelegramPluginStatusInput;
pub use telegram_status::NativeTelegramPollLoopStatus;
pub use telegram_status::NativeTelegramPollLoopStatusInput;
pub use telegram_status::NativeTelegramProductionGuardPolicyInput;
pub use telegram_status::NativeTelegramProductionGuardStatus;
pub use telegram_status::NativeTelegramProductionGuardStatusInput;
pub use telegram_status::NativeTelegramProductionReadinessInput;
pub use telegram_status::NativeTelegramProductionReadinessStatus;
pub use telegram_status::NativeTelegramReceiveOnceApiResultInput;
pub use telegram_status::NativeTelegramReceiveOnceErrorInput;
pub use telegram_status::NativeTelegramReceiveOncePreflightInput;
pub use telegram_status::NativeTelegramReceiveOnceShellReadinessInput;
pub use telegram_status::NativeTelegramReceiveOnceShellReadinessPlan;
pub use telegram_status::NativeTelegramReceiveOnceStatus;
pub use telegram_status::NativeTelegramReceiveOnceStatusInput;
pub use telegram_status::NativeTelegramSendPlanStatus;
pub use telegram_status::NativeTelegramSendPlanStatusInput;
pub use telegram_status::build_telegram_drain_once_status;
pub use telegram_status::build_telegram_live_soak_status;
pub use telegram_status::build_telegram_model_bridge_status;
pub use telegram_status::build_telegram_model_turn_plan_status;
pub use telegram_status::build_telegram_plugin_status;
pub use telegram_status::build_telegram_poll_loop_status;
pub use telegram_status::build_telegram_production_guard_status;
pub use telegram_status::build_telegram_production_guard_status_from_policy;
pub use telegram_status::build_telegram_production_readiness_status;
pub use telegram_status::build_telegram_receive_once_error_status;
pub use telegram_status::build_telegram_receive_once_status;
pub use telegram_status::build_telegram_receive_once_status_from_api_result;
pub use telegram_status::build_telegram_send_plan_status;
pub use telegram_status::plan_telegram_drain_once_api_result;
pub use telegram_status::plan_telegram_drain_once_preflight;
pub use telegram_status::plan_telegram_drain_once_shell_readiness;
pub use telegram_status::plan_telegram_receive_once_preflight_status;
pub use telegram_status::plan_telegram_receive_once_shell_readiness;
pub use telegram_status::telegram_poll_loop_interval_ms_policy;
pub use telegram_status::telegram_poll_loop_should_spawn;
pub use telegram_status::telegram_receive_limit_policy;
pub use telegram_status::telegram_soak_max_attention_count_policy;
pub use telegram_status::telegram_soak_max_observed_age_ms_policy;
pub use telegram_status::telegram_soak_min_poll_iterations_policy;
pub use telegram_status::telegram_system_time_unix_ms;
pub use telegram_transport::DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS;
pub use telegram_transport::DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS;
pub use telegram_transport::DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS;
pub use telegram_transport::DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS;
pub use telegram_transport::DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS;
pub use telegram_transport::MAX_TELEGRAM_READ_MAX_ATTEMPTS;
pub use telegram_transport::MAX_TELEGRAM_READ_RETRY_BACKOFF_MS;
pub use telegram_transport::MAX_TELEGRAM_SEND_MAX_ATTEMPTS;
pub use telegram_transport::MAX_TELEGRAM_SEND_MIN_INTERVAL_MS;
pub use telegram_transport::MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS;
pub use telegram_transport::MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS;
pub use telegram_transport::NativeTelegramSendExecutionInput;
pub use telegram_transport::NativeTelegramSendPlan;
pub use telegram_transport::NativeTelegramTransportPlan;
pub use telegram_transport::TELEGRAM_ALLOWED_UPDATES;
pub use telegram_transport::TelegramTypingKeepalive;
pub use telegram_transport::execute_telegram_send_after_model_output;
pub use telegram_transport::telegram_bot_token_shape_ok;
pub use telegram_transport::telegram_call_get_updates_once;
pub use telegram_transport::telegram_call_send_chat_action;
pub use telegram_transport::telegram_call_send_message;
pub use telegram_transport::telegram_get_updates_error_is_conflict;
pub use telegram_transport::telegram_get_updates_error_is_transient;
pub use telegram_transport::telegram_get_updates_query;
pub use telegram_transport::telegram_get_updates_should_retry;
pub use telegram_transport::telegram_get_updates_with_retry;
pub use telegram_transport::telegram_read_max_attempts_policy;
pub use telegram_transport::telegram_read_retry_backoff_policy;
pub use telegram_transport::telegram_redact_token_like_text;
pub use telegram_transport::telegram_send_chat_action_request_body;
pub use telegram_transport::telegram_send_error_is_transient;
pub use telegram_transport::telegram_send_max_attempts_policy;
pub use telegram_transport::telegram_send_message_request_body;
pub use telegram_transport::telegram_send_min_interval_policy;
pub use telegram_transport::telegram_send_rate_limit_sleep_for;
pub use telegram_transport::telegram_send_retry_backoff_policy;
pub use telegram_transport::telegram_send_should_retry;
pub use telegram_transport::telegram_start_typing_keepalive;
pub use telegram_transport::telegram_transport_plan_for_config_status;
pub use telegram_transport::telegram_typing_keepalive_interval_policy;
pub use telegram_transport::telegram_typing_keepalive_should_start;
pub use telegram_transport::telegram_wait_for_send_rate_limit;
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
    use super::GatewayEnvelope;
    use super::GatewaySurface;
    use super::GatewayTransport;

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
