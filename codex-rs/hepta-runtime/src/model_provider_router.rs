use std::{fs, path::PathBuf};

use hepta_core::HeptaError;
use serde::{Deserialize, Serialize};

use crate::{current_unix_ms, delivery_queue::ReadbackEvidenceLedger};

pub const DEFAULT_MODEL_PROVIDER_ROUTER_PATH: &str = ".hepta/model-provider-router-v0.json";
pub const DEFAULT_MODEL_PROVIDER_ROUTER_ID: &str = "hepta-native-model-provider-router";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelProviderStatus {
    Available,
    Degraded,
    Disabled,
}

impl ModelProviderStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Degraded => "degraded",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProviderRouterFile {
    pub version: u32,
    pub router_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub providers: Vec<ModelProviderRouteRecord>,
    #[serde(default)]
    pub invocation_handoffs: Vec<ModelProviderInvocationHandoffRecord>,
    #[serde(default)]
    pub memory_context_activation_handoffs: Vec<ModelProviderMemoryContextActivationRecord>,
    #[serde(default)]
    pub memory_context_activation_executions:
        Vec<ModelProviderMemoryContextActivationExecutionRecord>,
    #[serde(default)]
    pub local_invocations: Vec<ModelProviderLocalInvocationRecord>,
    #[serde(default)]
    pub plugin_contracts: Vec<ModelProviderPluginContractRecord>,
    #[serde(default)]
    pub route_events: Vec<ModelProviderRouteEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProviderRouteRecord {
    pub provider_id: String,
    pub model_id: String,
    pub capability: String,
    pub status: ModelProviderStatus,
    pub priority: u32,
    pub policy_label: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProviderInvocationHandoffRecord {
    pub handoff_id: String,
    pub provider_id: String,
    pub model_id: String,
    pub capability: String,
    pub fallback_provider_ids: Vec<String>,
    pub request_preview: String,
    pub auth_readiness: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub provider_invoked_by_gate: bool,
    pub auth_secret_read_by_gate: bool,
    pub usage_recorded_by_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProviderMemoryContextActivationRecord {
    pub handoff_id: String,
    pub provider_router_id: String,
    pub feature_flag_id: String,
    pub activation_contract: String,
    pub selected_canary_stage_id: String,
    pub traffic_percent_ppm: u32,
    pub max_context_node_count: usize,
    pub fallback_no_memory_provider_turn_hash: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub router_handoff_recorded: bool,
    pub feature_flag_mutated_by_adapter: bool,
    pub context_attached_to_live_prompt: bool,
    pub provider_invoked_by_adapter: bool,
    pub auth_secret_read_by_adapter: bool,
    pub usage_recorded_by_adapter: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProviderLocalInvocationRecord {
    pub invocation_id: String,
    pub handoff_id: String,
    pub provider_id: String,
    pub model_id: String,
    pub capability: String,
    pub request_preview: String,
    pub response_preview: String,
    pub prompt_char_count: usize,
    pub response_char_count: usize,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub provider_invoked_by_adapter: bool,
    pub auth_secret_read_by_adapter: bool,
    pub usage_recorded_by_adapter: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProviderPluginContractRecord {
    pub contract_id: String,
    pub provider_id: String,
    pub plugin_id: String,
    pub active_model_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_var_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_service_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_parameters: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    #[serde(default)]
    pub structured_media_extraction_declared: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub normalization_rules: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fallback_provider_ids: Vec<String>,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub dependency_cone_isolated: bool,
    pub env_discovery_recorded: bool,
    pub local_service_startup_gated: bool,
    pub token_caps_forwarded: bool,
    pub active_model_metadata_exposed: bool,
    pub provider_normalization_owned: bool,
    pub provider_invoked_by_contract: bool,
    pub local_service_started_by_contract: bool,
    pub auth_secret_read_by_contract: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProviderRouteEvent {
    pub event_id: String,
    pub event_type: String,
    pub provider_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderRouterReport {
    pub router_path: String,
    pub router: ModelProviderRouterFile,
    pub provider_count: usize,
    pub available_count: usize,
    pub degraded_count: usize,
    pub disabled_count: usize,
    pub memory_context_activation_handoff_count: usize,
    pub memory_context_activation_execution_count: usize,
    pub local_invocation_count: usize,
    pub plugin_contract_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderRegisterReport {
    pub router_path: String,
    pub provider: ModelProviderRouteRecord,
    pub replaced_existing: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderSelectionReport {
    pub router_path: String,
    pub requested_capability: String,
    pub selected_provider_id: String,
    pub selected_model_id: String,
    pub fallback_provider_ids: Vec<String>,
    pub provider_invoked: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderInvocationHandoffInput {
    pub capability: String,
    pub request_preview: String,
    pub auth_readiness: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderInvocationHandoffReport {
    pub router_path: String,
    pub evidence_ledger_path: String,
    pub handoff: ModelProviderInvocationHandoffRecord,
    pub duplicate_idempotency_key: bool,
    pub router_mutated_by_gate: bool,
    pub provider_invoked_by_gate: bool,
    pub auth_secret_read_by_gate: bool,
    pub usage_recorded_by_gate: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderMemoryContextActivationInput {
    pub feature_flag_id: String,
    pub activation_contract: String,
    pub provider_router_id: String,
    pub selected_canary_stage_id: String,
    pub traffic_percent_ppm: u32,
    pub max_context_node_count: usize,
    pub cutover_gate_ready: bool,
    pub operator_release_approved: bool,
    pub kill_switch_active: bool,
    pub fallback_no_memory_provider_turn_hash: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderMemoryContextActivationReport {
    pub router_path: String,
    pub evidence_ledger_path: String,
    pub handoff: ModelProviderMemoryContextActivationRecord,
    pub duplicate_idempotency_key: bool,
    pub router_mutated_by_adapter: bool,
    pub cutover_gate_ready: bool,
    pub operator_release_approved: bool,
    pub kill_switch_active: bool,
    pub router_handoff_allowed: bool,
    pub feature_flag_mutated_by_adapter: bool,
    pub context_attached_to_live_prompt: bool,
    pub provider_invoked_by_adapter: bool,
    pub auth_secret_read_by_adapter: bool,
    pub usage_recorded_by_adapter: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProviderMemoryContextActivationExecutionRecord {
    pub execution_id: String,
    pub handoff_id: String,
    pub provider_router_id: String,
    pub feature_flag_id: String,
    pub activation_contract: String,
    pub selected_canary_stage_id: String,
    pub traffic_percent_ppm: u32,
    pub max_context_node_count: usize,
    pub activation_mode: String,
    pub fallback_no_memory_provider_turn_hash: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub release_gate_ready: bool,
    pub operator_release_approved: bool,
    pub canary_telemetry_ready: bool,
    pub rollback_kill_switch_armed: bool,
    pub post_activation_watchdog_soak_plan_ready: bool,
    pub feature_flag_mutated_by_adapter: bool,
    pub context_attached_to_live_prompt: bool,
    pub provider_invoked_by_adapter: bool,
    pub auth_secret_read_by_adapter: bool,
    pub usage_recorded_by_adapter: bool,
    pub external_network_call_performed: bool,
    pub live_kg_write_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderMemoryContextActivationExecutionInput {
    pub handoff_id: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub release_gate_ready: bool,
    pub operator_release_approved: bool,
    pub kill_switch_active: bool,
    pub canary_telemetry_ready: bool,
    pub rollback_kill_switch_armed: bool,
    pub post_activation_watchdog_soak_plan_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderMemoryContextActivationExecutionReport {
    pub router_path: String,
    pub evidence_ledger_path: String,
    pub execution: ModelProviderMemoryContextActivationExecutionRecord,
    pub duplicate_idempotency_key: bool,
    pub router_mutated_by_adapter: bool,
    pub release_gate_ready: bool,
    pub operator_release_approved: bool,
    pub kill_switch_active: bool,
    pub canary_telemetry_ready: bool,
    pub rollback_kill_switch_armed: bool,
    pub post_activation_watchdog_soak_plan_ready: bool,
    pub shadow_context_attachment_allowed: bool,
    pub feature_flag_mutated_by_adapter: bool,
    pub context_attached_to_live_prompt: bool,
    pub provider_invoked_by_adapter: bool,
    pub auth_secret_read_by_adapter: bool,
    pub usage_recorded_by_adapter: bool,
    pub external_network_call_performed: bool,
    pub live_kg_write_performed: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderLocalInvocationInput {
    pub handoff_id: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderLocalInvocationReport {
    pub router_path: String,
    pub evidence_ledger_path: String,
    pub invocation: ModelProviderLocalInvocationRecord,
    pub duplicate_idempotency_key: bool,
    pub router_mutated_by_adapter: bool,
    pub provider_invoked_by_adapter: bool,
    pub auth_secret_read_by_adapter: bool,
    pub usage_recorded_by_adapter: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderPluginContractInput {
    pub provider_id: String,
    pub plugin_id: String,
    pub active_model_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_var_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_service_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_parameters: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    pub structured_media_extraction_declared: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub normalization_rules: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fallback_provider_ids: Vec<String>,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelProviderPluginContractReport {
    pub router_path: String,
    pub evidence_ledger_path: String,
    pub contract: ModelProviderPluginContractRecord,
    pub duplicate_idempotency_key: bool,
    pub router_mutated_by_contract: bool,
    pub dependency_cone_isolated: bool,
    pub env_discovery_recorded: bool,
    pub local_service_startup_gated: bool,
    pub token_caps_forwarded: bool,
    pub active_model_metadata_exposed: bool,
    pub provider_normalization_owned: bool,
    pub provider_invoked_by_contract: bool,
    pub local_service_started_by_contract: bool,
    pub auth_secret_read_by_contract: bool,
    pub persisted: bool,
}

pub struct ModelProviderRouter {
    path: PathBuf,
}

impl ModelProviderRouter {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        let cwd = std::env::current_dir().map_err(|err| {
            HeptaError(format!(
                "failed to resolve cwd for model-provider-router: {err}"
            ))
        })?;
        Ok(Self::new(cwd.join(DEFAULT_MODEL_PROVIDER_ROUTER_PATH)))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(
        &self,
        now_unix_ms: Option<u64>,
    ) -> Result<ModelProviderRouterReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let router = self.load_or_default(now)?;
        Ok(ModelProviderRouterReport {
            router_path: self.path_display(),
            provider_count: router.providers.len(),
            available_count: count_status(&router, ModelProviderStatus::Available),
            degraded_count: count_status(&router, ModelProviderStatus::Degraded),
            disabled_count: count_status(&router, ModelProviderStatus::Disabled),
            memory_context_activation_handoff_count: router
                .memory_context_activation_handoffs
                .len(),
            memory_context_activation_execution_count: router
                .memory_context_activation_executions
                .len(),
            local_invocation_count: router.local_invocations.len(),
            plugin_contract_count: router.plugin_contracts.len(),
            persisted: self.path.exists(),
            router,
        })
    }

    pub fn register_provider(
        &self,
        provider_id: &str,
        model_id: &str,
        capability: &str,
        status: ModelProviderStatus,
        priority: u32,
        policy_label: &str,
    ) -> Result<ModelProviderRegisterReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let provider_id = normalize_scoped_id(provider_id, "provider id")?;
        let model_id = normalize_scoped_id(model_id, "model id")?;
        let capability = normalize_scoped_id(capability, "capability")?;
        let policy_label = normalize_non_empty(policy_label, "policy label")?;
        let mut replaced_existing = false;
        if let Some(existing) = router.providers.iter_mut().find(|provider| {
            provider.provider_id == provider_id && provider.capability == capability
        }) {
            existing.model_id = model_id;
            existing.status = status;
            existing.priority = priority;
            existing.policy_label = policy_label;
            existing.updated_at_unix_ms = now;
            replaced_existing = true;
        } else {
            router.providers.push(ModelProviderRouteRecord {
                provider_id: provider_id.clone(),
                model_id,
                capability: capability.clone(),
                status,
                priority,
                policy_label,
                created_at_unix_ms: now,
                updated_at_unix_ms: now,
            });
        }
        let provider = router
            .providers
            .iter()
            .find(|provider| {
                provider.provider_id == provider_id && provider.capability == capability
            })
            .cloned()
            .expect("registered provider should exist");
        push_event(
            &mut router,
            "provider_registered",
            &provider_id,
            now,
            "model provider route registered locally; provider not invoked",
        );
        self.save(&mut router, now)?;
        Ok(ModelProviderRegisterReport {
            router_path: self.path_display(),
            provider,
            replaced_existing,
            persisted: true,
        })
    }

    pub fn select_provider(
        &self,
        capability: &str,
    ) -> Result<ModelProviderSelectionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let capability = normalize_scoped_id(capability, "capability")?;
        let mut candidates: Vec<_> = router
            .providers
            .iter()
            .filter(|provider| {
                provider.capability == capability
                    && provider.status != ModelProviderStatus::Disabled
            })
            .cloned()
            .collect();
        candidates.sort_by_key(|provider| (status_rank(provider.status), provider.priority));
        let selected = candidates.first().cloned().ok_or_else(|| {
            HeptaError(format!(
                "no enabled model provider route for capability {capability}"
            ))
        })?;
        let fallback_provider_ids = candidates
            .iter()
            .skip(1)
            .map(|provider| provider.provider_id.clone())
            .collect::<Vec<_>>();
        push_event(
            &mut router,
            "provider_selected",
            &selected.provider_id,
            now,
            "model provider selected locally; provider invocation remains false",
        );
        self.save(&mut router, now)?;
        Ok(ModelProviderSelectionReport {
            router_path: self.path_display(),
            requested_capability: capability,
            selected_provider_id: selected.provider_id,
            selected_model_id: selected.model_id,
            fallback_provider_ids,
            provider_invoked: false,
            persisted: true,
        })
    }

    pub fn gated_invocation_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: ModelProviderInvocationHandoffInput,
    ) -> Result<ModelProviderInvocationHandoffReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let capability = normalize_scoped_id(&input.capability, "capability")?;
        let request_preview = normalize_non_empty(&input.request_preview, "request preview")?;
        let auth_readiness = normalize_auth_readiness(&input.auth_readiness)?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "model provider handoff for {capability} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "model provider handoff for {capability} requires allow/approved policy decision"
            )));
        }
        if let Some(existing) = router
            .invocation_handoffs
            .iter()
            .find(|handoff| handoff.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ModelProviderInvocationHandoffReport {
                router_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                handoff: existing,
                duplicate_idempotency_key: true,
                router_mutated_by_gate: false,
                provider_invoked_by_gate: false,
                auth_secret_read_by_gate: false,
                usage_recorded_by_gate: false,
                persisted: self.path.exists(),
            });
        }
        let mut candidates: Vec<_> = router
            .providers
            .iter()
            .filter(|provider| {
                provider.capability == capability
                    && provider.status != ModelProviderStatus::Disabled
            })
            .cloned()
            .collect();
        candidates.sort_by_key(|provider| (status_rank(provider.status), provider.priority));
        let selected = candidates.first().cloned().ok_or_else(|| {
            HeptaError(format!(
                "no enabled model provider route for capability {capability}"
            ))
        })?;
        let fallback_provider_ids = candidates
            .iter()
            .skip(1)
            .map(|provider| provider.provider_id.clone())
            .collect::<Vec<_>>();
        let handoff_id = format!(
            "modelhandoff-{}-{}",
            now,
            router.invocation_handoffs.len() + 1
        );
        let evidence = evidence_ledger.append(
            "model_provider_invocation_handoff",
            &handoff_id,
            "handoff_recorded",
            &format!(
                "model provider invocation handoff recorded for capability {capability}; provider={}; model={}; provider invocation/auth secret read/usage recording not performed by this gate",
                selected.provider_id, selected.model_id
            ),
        )?;
        let handoff = ModelProviderInvocationHandoffRecord {
            handoff_id: handoff_id.clone(),
            provider_id: selected.provider_id.clone(),
            model_id: selected.model_id,
            capability: capability.clone(),
            fallback_provider_ids,
            request_preview,
            auth_readiness,
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            provider_invoked_by_gate: false,
            auth_secret_read_by_gate: false,
            usage_recorded_by_gate: false,
        };
        router.invocation_handoffs.push(handoff.clone());
        router.invocation_handoffs.truncate(1024);
        push_event(
            &mut router,
            "provider_invocation_handoff_recorded",
            &selected.provider_id,
            now,
            "model provider invocation handoff recorded with readback evidence; provider not invoked",
        );
        self.save(&mut router, now)?;
        Ok(ModelProviderInvocationHandoffReport {
            router_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            handoff,
            duplicate_idempotency_key: false,
            router_mutated_by_gate: true,
            provider_invoked_by_gate: false,
            auth_secret_read_by_gate: false,
            usage_recorded_by_gate: false,
            persisted: evidence.persisted,
        })
    }

    pub fn invoke_local_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: ModelProviderLocalInvocationInput,
    ) -> Result<ModelProviderLocalInvocationReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let handoff_id = normalize_non_empty(&input.handoff_id, "handoff id")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "local model provider invocation for {handoff_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "local model provider invocation for {handoff_id} requires allow/approved policy decision"
            )));
        }
        if let Some(existing) = router
            .local_invocations
            .iter()
            .find(|invocation| invocation.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ModelProviderLocalInvocationReport {
                router_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                invocation: existing,
                duplicate_idempotency_key: true,
                router_mutated_by_adapter: false,
                provider_invoked_by_adapter: false,
                auth_secret_read_by_adapter: false,
                usage_recorded_by_adapter: false,
                persisted: self.path.exists(),
            });
        }
        let handoff_index = router
            .invocation_handoffs
            .iter()
            .position(|handoff| handoff.handoff_id == handoff_id)
            .ok_or_else(|| HeptaError(format!("model provider handoff not found: {handoff_id}")))?;
        let handoff = router.invocation_handoffs[handoff_index].clone();
        if !handoff.operator_confirmed || !policy_allows_handoff(&handoff.policy_decision) {
            return Err(HeptaError(format!(
                "model provider handoff {handoff_id} is not approved for local invocation"
            )));
        }
        if handoff.provider_invoked_by_gate {
            return Err(HeptaError(format!(
                "model provider handoff {handoff_id} has already been invoked"
            )));
        }
        if !is_hepta_local_provider(&handoff.provider_id) {
            return Err(HeptaError(format!(
                "model provider handoff {handoff_id} targets non-local provider {}; exact external provider adapters remain gated",
                handoff.provider_id
            )));
        }
        let response_preview = local_fixture_response_preview(&handoff);
        let invocation_id = format!("modelinvoke-{}-{}", now, router.local_invocations.len() + 1);
        let evidence = evidence_ledger.append(
            "model_provider_local_invocation",
            &invocation_id,
            "invoked",
            &format!(
                "local model provider invoked for capability {}; provider={}; model={}; auth secret not read",
                handoff.capability, handoff.provider_id, handoff.model_id
            ),
        )?;
        let invocation = ModelProviderLocalInvocationRecord {
            invocation_id: invocation_id.clone(),
            handoff_id: handoff.handoff_id.clone(),
            provider_id: handoff.provider_id.clone(),
            model_id: handoff.model_id.clone(),
            capability: handoff.capability.clone(),
            request_preview: handoff.request_preview.clone(),
            prompt_char_count: handoff.request_preview.chars().count(),
            response_char_count: response_preview.chars().count(),
            response_preview,
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            provider_invoked_by_adapter: true,
            auth_secret_read_by_adapter: false,
            usage_recorded_by_adapter: true,
        };
        router.invocation_handoffs[handoff_index].provider_invoked_by_gate = true;
        router.invocation_handoffs[handoff_index].usage_recorded_by_gate = true;
        router.local_invocations.push(invocation.clone());
        router.local_invocations.truncate(1024);
        push_event(
            &mut router,
            "provider_local_invoked",
            &handoff.provider_id,
            now,
            "local model provider invocation completed with usage/readback evidence",
        );
        self.save(&mut router, now)?;
        Ok(ModelProviderLocalInvocationReport {
            router_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            invocation,
            duplicate_idempotency_key: false,
            router_mutated_by_adapter: true,
            provider_invoked_by_adapter: true,
            auth_secret_read_by_adapter: false,
            usage_recorded_by_adapter: true,
            persisted: evidence.persisted,
        })
    }

    pub fn record_memory_context_activation_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: ModelProviderMemoryContextActivationInput,
    ) -> Result<ModelProviderMemoryContextActivationReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let feature_flag_id = normalize_scoped_id(&input.feature_flag_id, "feature flag id")?;
        let activation_contract =
            normalize_scoped_id(&input.activation_contract, "activation contract")?;
        let provider_router_id =
            normalize_scoped_id(&input.provider_router_id, "provider router id")?;
        let selected_canary_stage_id =
            normalize_scoped_id(&input.selected_canary_stage_id, "selected canary stage id")?;
        let fallback_no_memory_provider_turn_hash = normalize_non_empty(
            &input.fallback_no_memory_provider_turn_hash,
            "fallback no-memory provider turn hash",
        )?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "memory context activation handoff for {feature_flag_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "memory context activation handoff for {feature_flag_id} requires allow/approved policy decision"
            )));
        }
        if !input.cutover_gate_ready {
            return Err(HeptaError(format!(
                "memory context activation handoff for {feature_flag_id} requires ready cutover gate"
            )));
        }
        if !input.operator_release_approved {
            return Err(HeptaError(format!(
                "memory context activation handoff for {feature_flag_id} requires operator release approval"
            )));
        }
        if input.kill_switch_active {
            return Err(HeptaError(format!(
                "memory context activation handoff for {feature_flag_id} is blocked by active kill switch"
            )));
        }
        if input.traffic_percent_ppm != 0 {
            return Err(HeptaError(format!(
                "memory context activation handoff for {feature_flag_id} must remain shadow-only with 0 traffic ppm"
            )));
        }
        if input.max_context_node_count == 0 || input.max_context_node_count > 128 {
            return Err(HeptaError(format!(
                "memory context activation handoff for {feature_flag_id} requires max_context_node_count between 1 and 128"
            )));
        }
        if let Some(existing) = router
            .memory_context_activation_handoffs
            .iter()
            .find(|handoff| handoff.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ModelProviderMemoryContextActivationReport {
                router_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                duplicate_idempotency_key: true,
                router_mutated_by_adapter: false,
                cutover_gate_ready: input.cutover_gate_ready,
                operator_release_approved: input.operator_release_approved,
                kill_switch_active: input.kill_switch_active,
                router_handoff_allowed: true,
                feature_flag_mutated_by_adapter: existing.feature_flag_mutated_by_adapter,
                context_attached_to_live_prompt: existing.context_attached_to_live_prompt,
                provider_invoked_by_adapter: existing.provider_invoked_by_adapter,
                auth_secret_read_by_adapter: existing.auth_secret_read_by_adapter,
                usage_recorded_by_adapter: existing.usage_recorded_by_adapter,
                persisted: self.path.exists(),
                handoff: existing,
            });
        }
        let handoff_id = format!(
            "memoryctxhandoff-{}-{}",
            now,
            router.memory_context_activation_handoffs.len() + 1
        );
        let evidence = evidence_ledger.append(
            "model_provider_memory_context_activation_handoff",
            &handoff_id,
            "handoff_recorded",
            &format!(
                "memory context activation handoff recorded for feature_flag={feature_flag_id}; router={provider_router_id}; stage={selected_canary_stage_id}; feature flag mutation/live prompt attachment/provider invocation/auth secret read/usage recording not performed by this adapter"
            ),
        )?;
        let handoff = ModelProviderMemoryContextActivationRecord {
            handoff_id: handoff_id.clone(),
            provider_router_id: provider_router_id.clone(),
            feature_flag_id: feature_flag_id.clone(),
            activation_contract,
            selected_canary_stage_id,
            traffic_percent_ppm: input.traffic_percent_ppm,
            max_context_node_count: input.max_context_node_count,
            fallback_no_memory_provider_turn_hash,
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            router_handoff_recorded: true,
            feature_flag_mutated_by_adapter: false,
            context_attached_to_live_prompt: false,
            provider_invoked_by_adapter: false,
            auth_secret_read_by_adapter: false,
            usage_recorded_by_adapter: false,
        };
        router
            .memory_context_activation_handoffs
            .push(handoff.clone());
        router.memory_context_activation_handoffs.truncate(1024);
        push_event(
            &mut router,
            "memory_context_activation_handoff_recorded",
            &provider_router_id,
            now,
            "memory context activation handoff recorded with readback evidence; live prompt attachment and provider invocation remain disabled",
        );
        self.save(&mut router, now)?;
        Ok(ModelProviderMemoryContextActivationReport {
            router_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            handoff,
            duplicate_idempotency_key: false,
            router_mutated_by_adapter: true,
            cutover_gate_ready: input.cutover_gate_ready,
            operator_release_approved: input.operator_release_approved,
            kill_switch_active: input.kill_switch_active,
            router_handoff_allowed: true,
            feature_flag_mutated_by_adapter: false,
            context_attached_to_live_prompt: false,
            provider_invoked_by_adapter: false,
            auth_secret_read_by_adapter: false,
            usage_recorded_by_adapter: false,
            persisted: evidence.persisted,
        })
    }

    pub fn execute_memory_context_activation_shadow(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: ModelProviderMemoryContextActivationExecutionInput,
    ) -> Result<ModelProviderMemoryContextActivationExecutionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let handoff_id = normalize_non_empty(&input.handoff_id, "handoff id")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "memory context activation execution for {handoff_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "memory context activation execution for {handoff_id} requires allow/approved policy decision"
            )));
        }
        if !input.release_gate_ready {
            return Err(HeptaError(format!(
                "memory context activation execution for {handoff_id} requires a ready release gate"
            )));
        }
        if !input.operator_release_approved {
            return Err(HeptaError(format!(
                "memory context activation execution for {handoff_id} requires operator release approval"
            )));
        }
        if input.kill_switch_active {
            return Err(HeptaError(format!(
                "memory context activation execution for {handoff_id} is blocked by active kill switch"
            )));
        }
        if !input.canary_telemetry_ready {
            return Err(HeptaError(format!(
                "memory context activation execution for {handoff_id} requires canary telemetry readiness"
            )));
        }
        if !input.rollback_kill_switch_armed {
            return Err(HeptaError(format!(
                "memory context activation execution for {handoff_id} requires an armed rollback kill switch"
            )));
        }
        if !input.post_activation_watchdog_soak_plan_ready {
            return Err(HeptaError(format!(
                "memory context activation execution for {handoff_id} requires post-activation watchdog/soak plan"
            )));
        }
        if let Some(existing) = router
            .memory_context_activation_executions
            .iter()
            .find(|execution| execution.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ModelProviderMemoryContextActivationExecutionReport {
                router_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                duplicate_idempotency_key: true,
                router_mutated_by_adapter: false,
                release_gate_ready: input.release_gate_ready,
                operator_release_approved: input.operator_release_approved,
                kill_switch_active: input.kill_switch_active,
                canary_telemetry_ready: input.canary_telemetry_ready,
                rollback_kill_switch_armed: input.rollback_kill_switch_armed,
                post_activation_watchdog_soak_plan_ready: input
                    .post_activation_watchdog_soak_plan_ready,
                shadow_context_attachment_allowed: true,
                feature_flag_mutated_by_adapter: existing.feature_flag_mutated_by_adapter,
                context_attached_to_live_prompt: existing.context_attached_to_live_prompt,
                provider_invoked_by_adapter: existing.provider_invoked_by_adapter,
                auth_secret_read_by_adapter: existing.auth_secret_read_by_adapter,
                usage_recorded_by_adapter: existing.usage_recorded_by_adapter,
                external_network_call_performed: existing.external_network_call_performed,
                live_kg_write_performed: existing.live_kg_write_performed,
                persisted: self.path.exists(),
                execution: existing,
            });
        }
        let handoff_index = router
            .memory_context_activation_handoffs
            .iter()
            .position(|handoff| handoff.handoff_id == handoff_id)
            .ok_or_else(|| {
                HeptaError(format!(
                    "memory context activation handoff not found: {handoff_id}"
                ))
            })?;
        if let Some(existing) = router
            .memory_context_activation_executions
            .iter()
            .find(|execution| execution.handoff_id == handoff_id)
        {
            return Err(HeptaError(format!(
                "memory context activation handoff {handoff_id} already executed by {}",
                existing.execution_id
            )));
        }
        let handoff = router.memory_context_activation_handoffs[handoff_index].clone();
        if !handoff.operator_confirmed || !policy_allows_handoff(&handoff.policy_decision) {
            return Err(HeptaError(format!(
                "memory context activation handoff {handoff_id} is not approved for execution"
            )));
        }
        if handoff.traffic_percent_ppm != 0 {
            return Err(HeptaError(format!(
                "memory context activation handoff {handoff_id} must remain shadow-only with 0 traffic ppm"
            )));
        }
        if handoff.max_context_node_count == 0 || handoff.max_context_node_count > 128 {
            return Err(HeptaError(format!(
                "memory context activation handoff {handoff_id} has invalid context node budget"
            )));
        }
        if handoff.provider_invoked_by_adapter || handoff.auth_secret_read_by_adapter {
            return Err(HeptaError(format!(
                "memory context activation handoff {handoff_id} has forbidden provider/auth side effects"
            )));
        }
        let execution_id = format!(
            "memoryctxactivate-{}-{}",
            now,
            router.memory_context_activation_executions.len() + 1
        );
        let evidence = evidence_ledger.append(
            "model_provider_memory_context_activation_shadow_execution",
            &execution_id,
            "shadow_context_attached",
            &format!(
                "operator-approved memory context activation shadow execution recorded for feature_flag={}; router={}; stage={}; traffic_ppm=0; provider invocation/auth secret read/external network/KG write not performed",
                handoff.feature_flag_id, handoff.provider_router_id, handoff.selected_canary_stage_id
            ),
        )?;
        let execution = ModelProviderMemoryContextActivationExecutionRecord {
            execution_id: execution_id.clone(),
            handoff_id: handoff.handoff_id.clone(),
            provider_router_id: handoff.provider_router_id.clone(),
            feature_flag_id: handoff.feature_flag_id.clone(),
            activation_contract: handoff.activation_contract.clone(),
            selected_canary_stage_id: handoff.selected_canary_stage_id.clone(),
            traffic_percent_ppm: handoff.traffic_percent_ppm,
            max_context_node_count: handoff.max_context_node_count,
            activation_mode: "operator_approved_shadow_context_attachment".into(),
            fallback_no_memory_provider_turn_hash: handoff
                .fallback_no_memory_provider_turn_hash
                .clone(),
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            release_gate_ready: input.release_gate_ready,
            operator_release_approved: input.operator_release_approved,
            canary_telemetry_ready: input.canary_telemetry_ready,
            rollback_kill_switch_armed: input.rollback_kill_switch_armed,
            post_activation_watchdog_soak_plan_ready: input
                .post_activation_watchdog_soak_plan_ready,
            feature_flag_mutated_by_adapter: true,
            context_attached_to_live_prompt: true,
            provider_invoked_by_adapter: false,
            auth_secret_read_by_adapter: false,
            usage_recorded_by_adapter: false,
            external_network_call_performed: false,
            live_kg_write_performed: false,
        };
        router.memory_context_activation_handoffs[handoff_index].feature_flag_mutated_by_adapter =
            true;
        router.memory_context_activation_handoffs[handoff_index].context_attached_to_live_prompt =
            true;
        router
            .memory_context_activation_executions
            .push(execution.clone());
        router.memory_context_activation_executions.truncate(1024);
        push_event(
            &mut router,
            "memory_context_activation_shadow_executed",
            &handoff.provider_router_id,
            now,
            "operator-approved shadow memory context attachment executed with readback evidence; provider/auth/KG/external effects remain disabled",
        );
        self.save(&mut router, now)?;
        Ok(ModelProviderMemoryContextActivationExecutionReport {
            router_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            execution,
            duplicate_idempotency_key: false,
            router_mutated_by_adapter: true,
            release_gate_ready: input.release_gate_ready,
            operator_release_approved: input.operator_release_approved,
            kill_switch_active: input.kill_switch_active,
            canary_telemetry_ready: input.canary_telemetry_ready,
            rollback_kill_switch_armed: input.rollback_kill_switch_armed,
            post_activation_watchdog_soak_plan_ready: input
                .post_activation_watchdog_soak_plan_ready,
            shadow_context_attachment_allowed: true,
            feature_flag_mutated_by_adapter: true,
            context_attached_to_live_prompt: true,
            provider_invoked_by_adapter: false,
            auth_secret_read_by_adapter: false,
            usage_recorded_by_adapter: false,
            external_network_call_performed: false,
            live_kg_write_performed: false,
            persisted: evidence.persisted,
        })
    }

    pub fn record_plugin_contract(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: ModelProviderPluginContractInput,
    ) -> Result<ModelProviderPluginContractReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let provider_id = normalize_scoped_id(&input.provider_id, "provider id")?;
        let plugin_id = normalize_scoped_id(&input.plugin_id, "plugin id")?;
        let active_model_id = normalize_scoped_id(&input.active_model_id, "active model id")?;
        let env_var_names = normalize_env_var_names(&input.env_var_names)?;
        let local_service_ref = input
            .local_service_ref
            .as_deref()
            .map(|value| normalize_scoped_id(value, "local service ref"))
            .transpose()?;
        let supported_parameters =
            normalize_unique_scoped_ids(&input.supported_parameters, "supported parameter")?;
        let normalization_rules =
            normalize_unique_scoped_ids(&input.normalization_rules, "normalization rule")?;
        let fallback_provider_ids =
            normalize_unique_scoped_ids(&input.fallback_provider_ids, "fallback provider id")?;
        let max_tokens = normalize_token_cap(input.max_tokens, "max_tokens")?;
        let max_completion_tokens =
            normalize_token_cap(input.max_completion_tokens, "max_completion_tokens")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "model provider plugin contract for {provider_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "model provider plugin contract for {provider_id} requires allow/approved policy decision"
            )));
        }
        if let Some(existing) = router
            .plugin_contracts
            .iter()
            .find(|contract| contract.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ModelProviderPluginContractReport {
                router_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                dependency_cone_isolated: existing.dependency_cone_isolated,
                env_discovery_recorded: existing.env_discovery_recorded,
                local_service_startup_gated: existing.local_service_startup_gated,
                token_caps_forwarded: existing.token_caps_forwarded,
                active_model_metadata_exposed: existing.active_model_metadata_exposed,
                provider_normalization_owned: existing.provider_normalization_owned,
                provider_invoked_by_contract: existing.provider_invoked_by_contract,
                local_service_started_by_contract: existing.local_service_started_by_contract,
                auth_secret_read_by_contract: existing.auth_secret_read_by_contract,
                contract: existing,
                duplicate_idempotency_key: true,
                router_mutated_by_contract: false,
                persisted: self.path.exists(),
            });
        }
        let contract_id = format!(
            "modelplugincontract-{}-{}",
            now,
            router.plugin_contracts.len() + 1
        );
        let token_caps_forwarded = max_tokens.is_some() || max_completion_tokens.is_some();
        let provider_normalization_owned =
            !normalization_rules.is_empty() || !fallback_provider_ids.is_empty();
        let evidence = evidence_ledger.append(
            "model_provider_plugin_contract",
            &contract_id,
            "recorded",
            &format!(
                "model provider plugin contract recorded for provider={provider_id}; plugin={plugin_id}; active_model={active_model_id}; provider invocation/local service start/auth secret read not performed"
            ),
        )?;
        let contract = ModelProviderPluginContractRecord {
            contract_id: contract_id.clone(),
            provider_id: provider_id.clone(),
            plugin_id,
            active_model_id,
            env_var_names: env_var_names.clone(),
            local_service_ref,
            supported_parameters,
            max_tokens,
            max_completion_tokens,
            structured_media_extraction_declared: input.structured_media_extraction_declared,
            normalization_rules,
            fallback_provider_ids,
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            dependency_cone_isolated: true,
            env_discovery_recorded: !env_var_names.is_empty(),
            local_service_startup_gated: true,
            token_caps_forwarded,
            active_model_metadata_exposed: true,
            provider_normalization_owned,
            provider_invoked_by_contract: false,
            local_service_started_by_contract: false,
            auth_secret_read_by_contract: false,
        };
        router.plugin_contracts.push(contract.clone());
        router.plugin_contracts.truncate(1024);
        push_event(
            &mut router,
            "provider_plugin_contract_recorded",
            &provider_id,
            now,
            "model provider plugin contract recorded with env discovery, localService gate, active-model metadata, token caps, normalization/fallback rules, and readback evidence",
        );
        self.save(&mut router, now)?;
        Ok(ModelProviderPluginContractReport {
            router_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            dependency_cone_isolated: contract.dependency_cone_isolated,
            env_discovery_recorded: contract.env_discovery_recorded,
            local_service_startup_gated: contract.local_service_startup_gated,
            token_caps_forwarded: contract.token_caps_forwarded,
            active_model_metadata_exposed: contract.active_model_metadata_exposed,
            provider_normalization_owned: contract.provider_normalization_owned,
            provider_invoked_by_contract: contract.provider_invoked_by_contract,
            local_service_started_by_contract: contract.local_service_started_by_contract,
            auth_secret_read_by_contract: contract.auth_secret_read_by_contract,
            contract,
            duplicate_idempotency_key: false,
            router_mutated_by_contract: true,
            persisted: evidence.persisted,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<ModelProviderRouterFile, HeptaError> {
        if !self.path.exists() {
            return Ok(ModelProviderRouterFile {
                version: 1,
                router_id: DEFAULT_MODEL_PROVIDER_ROUTER_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                providers: Vec::new(),
                invocation_handoffs: Vec::new(),
                memory_context_activation_handoffs: Vec::new(),
                memory_context_activation_executions: Vec::new(),
                local_invocations: Vec::new(),
                plugin_contracts: Vec::new(),
                route_events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read model-provider router {}: {err}",
                self.path.display()
            ))
        })?;
        let mut router: ModelProviderRouterFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse model-provider router {}: {err}",
                self.path.display()
            ))
        })?;
        if router.version != 1 {
            return Err(HeptaError(format!(
                "unsupported model-provider router version {} in {}",
                router.version,
                self.path.display()
            )));
        }
        router.route_events.truncate(1024);
        router.invocation_handoffs.truncate(1024);
        router.memory_context_activation_handoffs.truncate(1024);
        router.memory_context_activation_executions.truncate(1024);
        router.local_invocations.truncate(1024);
        router.plugin_contracts.truncate(1024);
        Ok(router)
    }

    fn save(
        &self,
        router: &mut ModelProviderRouterFile,
        now_unix_ms: u64,
    ) -> Result<(), HeptaError> {
        router.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create model-provider router directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(router).map_err(|err| {
            HeptaError(format!("failed to serialize model-provider router: {err}"))
        })?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write model-provider router {}: {err}",
                self.path.display()
            ))
        })
    }
}

fn count_status(router: &ModelProviderRouterFile, status: ModelProviderStatus) -> usize {
    router
        .providers
        .iter()
        .filter(|provider| provider.status == status)
        .count()
}

fn status_rank(status: ModelProviderStatus) -> u8 {
    match status {
        ModelProviderStatus::Available => 0,
        ModelProviderStatus::Degraded => 1,
        ModelProviderStatus::Disabled => 2,
    }
}

fn normalize_scoped_id(value: &str, label: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, label)?;
    if value.contains('\n') || value.contains('\r') || value.contains("..") {
        return Err(HeptaError(format!(
            "model provider {label} must be single-line and scoped"
        )));
    }
    Ok(value)
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "model provider {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn normalize_unique_scoped_ids(values: &[String], label: &str) -> Result<Vec<String>, HeptaError> {
    let mut out = Vec::new();
    for value in values {
        let value = normalize_scoped_id(value, label)?;
        if !out.iter().any(|existing| existing == &value) {
            out.push(value);
        }
    }
    Ok(out)
}

fn normalize_env_var_names(values: &[String]) -> Result<Vec<String>, HeptaError> {
    let mut out = Vec::new();
    for value in values {
        let value = normalize_non_empty(value, "env var name")?;
        if value.contains('=') || value.contains(char::is_whitespace) {
            return Err(HeptaError(format!(
                "model provider env var name {value} must be a name, not an assignment"
            )));
        }
        let mut chars = value.chars();
        let Some(first) = chars.next() else {
            return Err(HeptaError(
                "model provider env var name must not be empty".into(),
            ));
        };
        if !(first == '_' || first.is_ascii_alphabetic())
            || !chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
        {
            return Err(HeptaError(format!(
                "model provider env var name {value} must be shell-safe"
            )));
        }
        if !out.iter().any(|existing| existing == &value) {
            out.push(value);
        }
    }
    Ok(out)
}

fn normalize_token_cap(value: Option<u32>, label: &str) -> Result<Option<u32>, HeptaError> {
    if let Some(value) = value {
        if value == 0 || value > 1_000_000 {
            return Err(HeptaError(format!(
                "model provider {label} must be between 1 and 1000000"
            )));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

fn normalize_auth_readiness(value: &str) -> Result<String, HeptaError> {
    let readiness = normalize_non_empty(value, "auth readiness")?;
    let lower = readiness.to_ascii_lowercase();
    if lower == "ready"
        || lower.starts_with("auth-ready")
        || lower == "not_required"
        || lower == "not-required"
        || lower.starts_with("not-required:")
    {
        return Ok(readiness);
    }
    Err(HeptaError(format!(
        "model provider auth readiness {readiness} must indicate ready or not_required"
    )))
}

fn policy_allows_handoff(policy_decision: &str) -> bool {
    let policy = policy_decision.to_ascii_lowercase();
    policy.contains("allow") || policy.contains("approved")
}

fn is_hepta_local_provider(provider_id: &str) -> bool {
    provider_id == "hepta-local"
        || provider_id.starts_with("hepta-local:")
        || provider_id.starts_with("hepta-fixture")
}

fn local_fixture_response_preview(handoff: &ModelProviderInvocationHandoffRecord) -> String {
    let mut preview = format!(
        "hepta-local-response capability={} model={} request_chars={}",
        handoff.capability,
        handoff.model_id,
        handoff.request_preview.chars().count()
    );
    if preview.chars().count() > 240 {
        preview = preview.chars().take(240).collect::<String>();
        preview.push_str("...");
    }
    preview
}

fn push_event(
    router: &mut ModelProviderRouterFile,
    event_type: &str,
    provider_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    router.route_events.push(ModelProviderRouteEvent {
        event_id: format!(
            "modelrouteevt-{}-{}",
            now_unix_ms,
            router.route_events.len() + 1
        ),
        event_type: event_type.into(),
        provider_id: provider_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    router.route_events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReadbackEvidenceLedger;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-model-router-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn model_provider_router_selects_available_route_without_invoking_provider() {
        let path = temp_file("select");
        let router = ModelProviderRouter::new(&path);
        router
            .register_provider(
                "openai-codex",
                "gpt-5.5",
                "agent_text",
                ModelProviderStatus::Degraded,
                2,
                "fallback-only",
            )
            .unwrap();
        router
            .register_provider(
                "hepta-local",
                "hepta-native-small",
                "agent_text",
                ModelProviderStatus::Available,
                1,
                "local-first",
            )
            .unwrap();
        router
            .register_provider(
                "disabled-cloud",
                "remote-model",
                "agent_text",
                ModelProviderStatus::Disabled,
                0,
                "disabled",
            )
            .unwrap();
        let selected = router.select_provider("agent_text").unwrap();
        assert_eq!(selected.selected_provider_id, "hepta-local");
        assert_eq!(selected.fallback_provider_ids, vec!["openai-codex"]);
        assert!(!selected.provider_invoked);
        let report = router.report(None).unwrap();
        assert_eq!(report.provider_count, 3);
        assert_eq!(report.available_count, 1);
        assert_eq!(report.disabled_count, 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn model_provider_router_replaces_and_rejects_bad_routes() {
        let path = temp_file("replace");
        let router = ModelProviderRouter::new(&path);
        assert!(
            router
                .register_provider(
                    "../bad",
                    "model",
                    "agent_text",
                    ModelProviderStatus::Available,
                    1,
                    "bad",
                )
                .is_err()
        );
        router
            .register_provider(
                "hepta-local",
                "model-a",
                "vision",
                ModelProviderStatus::Available,
                3,
                "local",
            )
            .unwrap();
        let replaced = router
            .register_provider(
                "hepta-local",
                "model-b",
                "vision",
                ModelProviderStatus::Available,
                1,
                "local",
            )
            .unwrap();
        assert!(replaced.replaced_existing);
        assert_eq!(replaced.provider.model_id, "model-b");
        assert!(router.select_provider("missing").is_err());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn model_provider_router_gated_invocation_handoff_records_readback_without_invoking() {
        let path = temp_file("handoff");
        let ledger_path = temp_file("handoff-ledger");
        let router = ModelProviderRouter::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        router
            .register_provider(
                "openai-codex",
                "gpt-5.5",
                "agent_text",
                ModelProviderStatus::Degraded,
                2,
                "fallback",
            )
            .unwrap();
        router
            .register_provider(
                "hepta-local",
                "hepta-native-small",
                "agent_text",
                ModelProviderStatus::Available,
                1,
                "local-first",
            )
            .unwrap();
        let unconfirmed = ModelProviderInvocationHandoffInput {
            capability: "agent_text".into(),
            request_preview: "summarize task state".into(),
            auth_readiness: "auth-ready".into(),
            policy_decision: "approved-provider".into(),
            operator_confirmed: false,
            idempotency_key: "model-handoff-idem".into(),
        };
        assert!(
            router
                .gated_invocation_handoff(&ledger, unconfirmed)
                .is_err()
        );
        let missing_auth = ModelProviderInvocationHandoffInput {
            capability: "agent_text".into(),
            request_preview: "summarize task state".into(),
            auth_readiness: "missing".into(),
            policy_decision: "approved-provider".into(),
            operator_confirmed: true,
            idempotency_key: "model-handoff-idem".into(),
        };
        assert!(
            router
                .gated_invocation_handoff(&ledger, missing_auth)
                .is_err()
        );
        let confirmed = ModelProviderInvocationHandoffInput {
            capability: "agent_text".into(),
            request_preview: "summarize task state".into(),
            auth_readiness: "auth-ready-redacted".into(),
            policy_decision: "allow-provider-invocation".into(),
            operator_confirmed: true,
            idempotency_key: "model-handoff-idem".into(),
        };
        let report = router
            .gated_invocation_handoff(&ledger, confirmed.clone())
            .expect("confirmed model handoff should record");
        assert!(report.router_mutated_by_gate);
        assert_eq!(report.handoff.provider_id, "hepta-local");
        assert_eq!(report.handoff.fallback_provider_ids, vec!["openai-codex"]);
        assert!(!report.provider_invoked_by_gate);
        assert!(!report.auth_secret_read_by_gate);
        assert!(!report.usage_recorded_by_gate);
        assert!(report.handoff.readback_evidence_id.starts_with("rb-"));
        let duplicate = router
            .gated_invocation_handoff(&ledger, confirmed)
            .expect("duplicate model handoff should be idempotent");
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.router_mutated_by_gate);
        let router_report = router.report(None).unwrap();
        assert_eq!(router_report.router.invocation_handoffs.len(), 1);
        assert!(router_report.router.route_events.iter().any(|event| {
            event.event_type == "provider_invocation_handoff_recorded"
                && event.summary.contains("provider not invoked")
        }));
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 1);
        assert_eq!(
            ledger_report.ledger.entries[0].subject_kind,
            "model_provider_invocation_handoff"
        );
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn model_provider_router_records_memory_context_activation_without_live_effects() {
        let path = temp_file("memory-context-activation");
        let ledger_path = temp_file("memory-context-activation-ledger");
        let router = ModelProviderRouter::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let unconfirmed = ModelProviderMemoryContextActivationInput {
            feature_flag_id: "memory-context-provider-turn-v1".into(),
            activation_contract: "hepta-intelligence-memory-provider-router-activation-gate-v1"
                .into(),
            provider_router_id: "hepta-native-model-provider-router".into(),
            selected_canary_stage_id: "shadow-canary-0ppm".into(),
            traffic_percent_ppm: 0,
            max_context_node_count: 5,
            cutover_gate_ready: true,
            operator_release_approved: true,
            kill_switch_active: false,
            fallback_no_memory_provider_turn_hash: "fallback-hash".into(),
            policy_decision: "approved-memory-router-handoff".into(),
            operator_confirmed: false,
            idempotency_key: "memory-router-runtime-idem".into(),
        };
        assert!(
            router
                .record_memory_context_activation_handoff(&ledger, unconfirmed)
                .is_err()
        );
        let input = ModelProviderMemoryContextActivationInput {
            feature_flag_id: "memory-context-provider-turn-v1".into(),
            activation_contract: "hepta-intelligence-memory-provider-router-activation-gate-v1"
                .into(),
            provider_router_id: "hepta-native-model-provider-router".into(),
            selected_canary_stage_id: "shadow-canary-0ppm".into(),
            traffic_percent_ppm: 0,
            max_context_node_count: 5,
            cutover_gate_ready: true,
            operator_release_approved: true,
            kill_switch_active: false,
            fallback_no_memory_provider_turn_hash: "fallback-hash".into(),
            policy_decision: "approved-memory-router-handoff".into(),
            operator_confirmed: true,
            idempotency_key: "memory-router-runtime-idem".into(),
        };
        let report = router
            .record_memory_context_activation_handoff(&ledger, input.clone())
            .expect("approved memory context activation handoff should record");
        assert!(report.router_mutated_by_adapter);
        assert!(report.router_handoff_allowed);
        assert!(report.handoff.router_handoff_recorded);
        assert_eq!(
            report.handoff.provider_router_id,
            "hepta-native-model-provider-router"
        );
        assert_eq!(report.handoff.traffic_percent_ppm, 0);
        assert!(!report.feature_flag_mutated_by_adapter);
        assert!(!report.context_attached_to_live_prompt);
        assert!(!report.provider_invoked_by_adapter);
        assert!(!report.auth_secret_read_by_adapter);
        assert!(!report.usage_recorded_by_adapter);
        assert!(report.handoff.readback_evidence_id.starts_with("rb-"));
        let duplicate = router
            .record_memory_context_activation_handoff(&ledger, input)
            .expect("duplicate memory activation handoff should be idempotent");
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.router_mutated_by_adapter);
        let router_report = router.report(None).unwrap();
        assert_eq!(router_report.memory_context_activation_handoff_count, 1);
        assert_eq!(
            router_report
                .router
                .memory_context_activation_handoffs
                .len(),
            1
        );
        assert!(router_report.router.route_events.iter().any(|event| {
            event.event_type == "memory_context_activation_handoff_recorded"
                && event
                    .summary
                    .contains("provider invocation remain disabled")
        }));
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 1);
        assert_eq!(
            ledger_report.ledger.entries[0].subject_kind,
            "model_provider_memory_context_activation_handoff"
        );
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn model_provider_router_blocks_memory_context_activation_without_cutover_or_with_kill_switch()
    {
        let path = temp_file("memory-context-activation-blocked");
        let ledger_path = temp_file("memory-context-activation-blocked-ledger");
        let router = ModelProviderRouter::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let base = ModelProviderMemoryContextActivationInput {
            feature_flag_id: "memory-context-provider-turn-v1".into(),
            activation_contract: "hepta-intelligence-memory-provider-router-activation-gate-v1"
                .into(),
            provider_router_id: "hepta-native-model-provider-router".into(),
            selected_canary_stage_id: "shadow-canary-0ppm".into(),
            traffic_percent_ppm: 0,
            max_context_node_count: 5,
            cutover_gate_ready: true,
            operator_release_approved: true,
            kill_switch_active: false,
            fallback_no_memory_provider_turn_hash: "fallback-hash".into(),
            policy_decision: "approved-memory-router-handoff".into(),
            operator_confirmed: true,
            idempotency_key: "memory-router-runtime-idem-blocked".into(),
        };
        let mut missing_cutover = base.clone();
        missing_cutover.cutover_gate_ready = false;
        assert!(
            router
                .record_memory_context_activation_handoff(&ledger, missing_cutover)
                .is_err()
        );
        let mut kill_switch = base.clone();
        kill_switch.kill_switch_active = true;
        assert!(
            router
                .record_memory_context_activation_handoff(&ledger, kill_switch)
                .is_err()
        );
        let mut non_shadow = base;
        non_shadow.traffic_percent_ppm = 1000;
        assert!(
            router
                .record_memory_context_activation_handoff(&ledger, non_shadow)
                .is_err()
        );
        assert!(!path.exists());
        assert!(!ledger_path.exists());
    }

    #[test]
    fn model_provider_router_executes_operator_approved_memory_context_shadow_activation() {
        let path = temp_file("memory-context-shadow-execution");
        let ledger_path = temp_file("memory-context-shadow-execution-ledger");
        let router = ModelProviderRouter::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let handoff = router
            .record_memory_context_activation_handoff(
                &ledger,
                ModelProviderMemoryContextActivationInput {
                    feature_flag_id: "memory-context-provider-turn-v1".into(),
                    activation_contract:
                        "hepta-intelligence-memory-provider-router-activation-gate-v1".into(),
                    provider_router_id: "hepta-native-model-provider-router".into(),
                    selected_canary_stage_id: "shadow-canary-0ppm".into(),
                    traffic_percent_ppm: 0,
                    max_context_node_count: 5,
                    cutover_gate_ready: true,
                    operator_release_approved: true,
                    kill_switch_active: false,
                    fallback_no_memory_provider_turn_hash: "fallback-hash".into(),
                    policy_decision: "approved-memory-router-handoff".into(),
                    operator_confirmed: true,
                    idempotency_key: "memory-router-shadow-handoff".into(),
                },
            )
            .expect("approved memory context activation handoff should record");
        let execution_input = ModelProviderMemoryContextActivationExecutionInput {
            handoff_id: handoff.handoff.handoff_id.clone(),
            policy_decision: "approved-shadow-context-activation".into(),
            operator_confirmed: true,
            idempotency_key: "memory-router-shadow-execution".into(),
            release_gate_ready: true,
            operator_release_approved: true,
            kill_switch_active: false,
            canary_telemetry_ready: true,
            rollback_kill_switch_armed: true,
            post_activation_watchdog_soak_plan_ready: true,
        };
        let execution = router
            .execute_memory_context_activation_shadow(&ledger, execution_input.clone())
            .expect("operator-approved shadow activation should execute");
        assert!(execution.router_mutated_by_adapter);
        assert!(execution.shadow_context_attachment_allowed);
        assert!(execution.feature_flag_mutated_by_adapter);
        assert!(execution.context_attached_to_live_prompt);
        assert!(!execution.provider_invoked_by_adapter);
        assert!(!execution.auth_secret_read_by_adapter);
        assert!(!execution.usage_recorded_by_adapter);
        assert!(!execution.external_network_call_performed);
        assert!(!execution.live_kg_write_performed);
        assert!(execution.execution.readback_evidence_id.starts_with("rb-"));
        assert_eq!(execution.execution.traffic_percent_ppm, 0);
        assert_eq!(
            execution.execution.activation_mode,
            "operator_approved_shadow_context_attachment"
        );

        let duplicate = router
            .execute_memory_context_activation_shadow(&ledger, execution_input)
            .expect("duplicate shadow activation execution should be idempotent");
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.router_mutated_by_adapter);
        assert!(duplicate.context_attached_to_live_prompt);

        let router_report = router.report(None).unwrap();
        assert_eq!(router_report.memory_context_activation_handoff_count, 1);
        assert_eq!(router_report.memory_context_activation_execution_count, 1);
        let stored_handoff = router_report
            .router
            .memory_context_activation_handoffs
            .first()
            .expect("stored handoff should exist");
        assert!(stored_handoff.feature_flag_mutated_by_adapter);
        assert!(stored_handoff.context_attached_to_live_prompt);
        assert!(!stored_handoff.provider_invoked_by_adapter);
        assert!(!stored_handoff.auth_secret_read_by_adapter);
        assert!(router_report.router.route_events.iter().any(|event| {
            event.event_type == "memory_context_activation_shadow_executed"
                && event
                    .summary
                    .contains("provider/auth/KG/external effects remain disabled")
        }));
        let ledger_report = ledger.report(None).unwrap();
        let subject_kinds = ledger_report
            .ledger
            .entries
            .iter()
            .map(|entry| entry.subject_kind.as_str())
            .collect::<Vec<_>>();
        assert!(subject_kinds.contains(&"model_provider_memory_context_activation_handoff"));
        assert!(
            subject_kinds.contains(&"model_provider_memory_context_activation_shadow_execution")
        );
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn model_provider_router_blocks_shadow_activation_without_release_gate_or_kill_switch() {
        let path = temp_file("memory-context-shadow-execution-blocked");
        let ledger_path = temp_file("memory-context-shadow-execution-blocked-ledger");
        let router = ModelProviderRouter::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let handoff = router
            .record_memory_context_activation_handoff(
                &ledger,
                ModelProviderMemoryContextActivationInput {
                    feature_flag_id: "memory-context-provider-turn-v1".into(),
                    activation_contract:
                        "hepta-intelligence-memory-provider-router-activation-gate-v1".into(),
                    provider_router_id: "hepta-native-model-provider-router".into(),
                    selected_canary_stage_id: "shadow-canary-0ppm".into(),
                    traffic_percent_ppm: 0,
                    max_context_node_count: 5,
                    cutover_gate_ready: true,
                    operator_release_approved: true,
                    kill_switch_active: false,
                    fallback_no_memory_provider_turn_hash: "fallback-hash".into(),
                    policy_decision: "approved-memory-router-handoff".into(),
                    operator_confirmed: true,
                    idempotency_key: "memory-router-shadow-handoff-blocked".into(),
                },
            )
            .expect("approved memory context activation handoff should record");
        let base = ModelProviderMemoryContextActivationExecutionInput {
            handoff_id: handoff.handoff.handoff_id.clone(),
            policy_decision: "approved-shadow-context-activation".into(),
            operator_confirmed: true,
            idempotency_key: "memory-router-shadow-execution-blocked".into(),
            release_gate_ready: true,
            operator_release_approved: true,
            kill_switch_active: false,
            canary_telemetry_ready: true,
            rollback_kill_switch_armed: true,
            post_activation_watchdog_soak_plan_ready: true,
        };
        let mut missing_release_gate = base.clone();
        missing_release_gate.release_gate_ready = false;
        assert!(
            router
                .execute_memory_context_activation_shadow(&ledger, missing_release_gate)
                .is_err()
        );
        let mut kill_switch = base.clone();
        kill_switch.kill_switch_active = true;
        assert!(
            router
                .execute_memory_context_activation_shadow(&ledger, kill_switch)
                .is_err()
        );
        let mut no_telemetry = base.clone();
        no_telemetry.canary_telemetry_ready = false;
        assert!(
            router
                .execute_memory_context_activation_shadow(&ledger, no_telemetry)
                .is_err()
        );
        let mut no_rollback = base.clone();
        no_rollback.rollback_kill_switch_armed = false;
        assert!(
            router
                .execute_memory_context_activation_shadow(&ledger, no_rollback)
                .is_err()
        );
        let mut no_soak = base;
        no_soak.post_activation_watchdog_soak_plan_ready = false;
        assert!(
            router
                .execute_memory_context_activation_shadow(&ledger, no_soak)
                .is_err()
        );
        let router_report = router.report(None).unwrap();
        assert_eq!(router_report.memory_context_activation_execution_count, 0);
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn model_provider_router_invokes_local_fixture_with_usage_readback() {
        let path = temp_file("local-invocation");
        let ledger_path = temp_file("local-invocation-ledger");
        let router = ModelProviderRouter::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        router
            .register_provider(
                "openai-codex",
                "gpt-5.5",
                "agent_text",
                ModelProviderStatus::Degraded,
                2,
                "fallback",
            )
            .unwrap();
        router
            .register_provider(
                "hepta-local",
                "hepta-fixture-small",
                "agent_text",
                ModelProviderStatus::Available,
                1,
                "local-fixture",
            )
            .unwrap();
        let handoff = router
            .gated_invocation_handoff(
                &ledger,
                ModelProviderInvocationHandoffInput {
                    capability: "agent_text".into(),
                    request_preview: "summarize task state".into(),
                    auth_readiness: "not_required".into(),
                    policy_decision: "allow-provider-invocation".into(),
                    operator_confirmed: true,
                    idempotency_key: "model-local-handoff-idem".into(),
                },
            )
            .unwrap();
        let unconfirmed = ModelProviderLocalInvocationInput {
            handoff_id: handoff.handoff.handoff_id.clone(),
            policy_decision: "approved-local-provider".into(),
            operator_confirmed: false,
            idempotency_key: "model-local-invoke-idem".into(),
        };
        assert!(router.invoke_local_handoff(&ledger, unconfirmed).is_err());
        let input = ModelProviderLocalInvocationInput {
            handoff_id: handoff.handoff.handoff_id.clone(),
            policy_decision: "approved-local-provider".into(),
            operator_confirmed: true,
            idempotency_key: "model-local-invoke-idem".into(),
        };
        let invocation = router
            .invoke_local_handoff(&ledger, input.clone())
            .expect("approved hepta-local handoff should invoke fixture provider");
        assert!(invocation.router_mutated_by_adapter);
        assert!(invocation.provider_invoked_by_adapter);
        assert!(!invocation.auth_secret_read_by_adapter);
        assert!(invocation.usage_recorded_by_adapter);
        assert_eq!(invocation.invocation.provider_id, "hepta-local");
        assert_eq!(invocation.invocation.model_id, "hepta-fixture-small");
        assert!(invocation.invocation.prompt_char_count > 0);
        assert!(invocation.invocation.response_char_count > 0);
        assert!(
            invocation
                .invocation
                .response_preview
                .contains("hepta-local-response")
        );
        let duplicate = router
            .invoke_local_handoff(&ledger, input)
            .expect("duplicate local provider invocation should be idempotent");
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.provider_invoked_by_adapter);
        let router_report = router.report(None).unwrap();
        assert_eq!(router_report.local_invocation_count, 1);
        assert!(router_report.router.invocation_handoffs[0].provider_invoked_by_gate);
        assert!(router_report.router.invocation_handoffs[0].usage_recorded_by_gate);
        assert_eq!(router_report.router.local_invocations.len(), 1);
        assert!(router_report.router.route_events.iter().any(|event| {
            event.event_type == "provider_local_invoked"
                && event.summary.contains("usage/readback evidence")
        }));
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 2);
        assert_eq!(
            ledger_report.ledger.entries[1].subject_kind,
            "model_provider_local_invocation"
        );
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn model_provider_router_records_plugin_contract_without_invoking_provider() {
        let path = temp_file("plugin-contract");
        let ledger_path = temp_file("plugin-contract-ledger");
        let router = ModelProviderRouter::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        assert!(
            router
                .record_plugin_contract(
                    &ledger,
                    ModelProviderPluginContractInput {
                        provider_id: "ollama-openai-compatible".into(),
                        plugin_id: "ollama-provider".into(),
                        active_model_id: "qwen3:8b".into(),
                        env_var_names: vec!["OLLAMA_HOST=http://bad".into()],
                        local_service_ref: Some("local-service:ollama".into()),
                        supported_parameters: vec!["max_tokens".into()],
                        max_tokens: Some(8192),
                        max_completion_tokens: Some(2048),
                        structured_media_extraction_declared: true,
                        normalization_rules: vec!["ollama-normalize-tags".into()],
                        fallback_provider_ids: vec!["hepta-local".into()],
                        policy_decision: "allow-provider-contract".into(),
                        operator_confirmed: true,
                        idempotency_key: "bad-env-contract".into(),
                    },
                )
                .is_err()
        );
        assert!(
            router
                .record_plugin_contract(
                    &ledger,
                    ModelProviderPluginContractInput {
                        provider_id: "ollama-openai-compatible".into(),
                        plugin_id: "ollama-provider".into(),
                        active_model_id: "qwen3:8b".into(),
                        env_var_names: vec!["OLLAMA_HOST".into()],
                        local_service_ref: Some("local-service:ollama".into()),
                        supported_parameters: vec!["max_tokens".into()],
                        max_tokens: Some(0),
                        max_completion_tokens: Some(2048),
                        structured_media_extraction_declared: true,
                        normalization_rules: vec!["ollama-normalize-tags".into()],
                        fallback_provider_ids: vec!["hepta-local".into()],
                        policy_decision: "allow-provider-contract".into(),
                        operator_confirmed: true,
                        idempotency_key: "bad-token-cap-contract".into(),
                    },
                )
                .is_err()
        );
        let input = ModelProviderPluginContractInput {
            provider_id: "ollama-openai-compatible".into(),
            plugin_id: "ollama-provider".into(),
            active_model_id: "qwen3:8b".into(),
            env_var_names: vec!["OLLAMA_HOST".into(), "OLLAMA_HOST".into()],
            local_service_ref: Some("local-service:ollama".into()),
            supported_parameters: vec![
                "max_tokens".into(),
                "max_completion_tokens".into(),
                "temperature".into(),
            ],
            max_tokens: Some(8192),
            max_completion_tokens: Some(2048),
            structured_media_extraction_declared: true,
            normalization_rules: vec![
                "ollama-normalize-tags".into(),
                "openai-compatible-token-caps".into(),
            ],
            fallback_provider_ids: vec!["hepta-local".into(), "hepta-local".into()],
            policy_decision: "approved-provider-contract".into(),
            operator_confirmed: true,
            idempotency_key: "plugin-contract-idem".into(),
        };
        let report = router
            .record_plugin_contract(&ledger, input.clone())
            .expect("provider plugin contract should be recorded");
        assert!(report.router_mutated_by_contract);
        assert!(report.dependency_cone_isolated);
        assert!(report.env_discovery_recorded);
        assert!(report.local_service_startup_gated);
        assert!(report.token_caps_forwarded);
        assert!(report.active_model_metadata_exposed);
        assert!(report.provider_normalization_owned);
        assert!(!report.provider_invoked_by_contract);
        assert!(!report.local_service_started_by_contract);
        assert!(!report.auth_secret_read_by_contract);
        assert_eq!(report.contract.env_var_names, vec!["OLLAMA_HOST"]);
        assert_eq!(report.contract.fallback_provider_ids, vec!["hepta-local"]);
        assert_eq!(report.contract.max_tokens, Some(8192));
        assert_eq!(report.contract.max_completion_tokens, Some(2048));
        assert!(report.contract.structured_media_extraction_declared);
        let duplicate = router
            .record_plugin_contract(&ledger, input)
            .expect("duplicate provider plugin contract should be idempotent");
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.router_mutated_by_contract);
        let router_report = router.report(None).unwrap();
        assert_eq!(router_report.plugin_contract_count, 1);
        assert_eq!(router_report.router.plugin_contracts.len(), 1);
        assert!(router_report.router.route_events.iter().any(|event| {
            event.event_type == "provider_plugin_contract_recorded"
                && event.summary.contains("localService gate")
        }));
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 1);
        assert_eq!(
            ledger_report.ledger.entries[0].subject_kind,
            "model_provider_plugin_contract"
        );
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }
}
