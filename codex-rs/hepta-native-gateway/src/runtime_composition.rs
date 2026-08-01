use std::env;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
#[cfg(test)]
use hepta_contracts::RevisionStamp;
use hepta_memory::DurableIntegrityKey;
use hepta_runtime::NduH1Runtime;
use hepta_runtime::NduH1RuntimeStatus;
use hepta_runtime::NduH1ShadowEvent;
use hepta_runtime::RuntimeExecutionReceipt;
use hepta_runtime::RuntimeKernel;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::durability_anchor::DurableAnchorStateSnapshot;
use crate::durability_anchor::ExternalMonotonicAnchor;
use crate::durability_anchor::ExternalMonotonicAnchorConfig;
use crate::durability_anchor::ExternalMonotonicAnchorEffectLease;
use crate::effect_reconciliation::EffectReconciliationAuthority;
use crate::effect_reconciliation::EffectReconciliationConfig;
use crate::effect_reconciliation::EffectReconciliationHttpResponse;
use crate::native_telegram::OperatorTelegramExecutionIdentity;
use crate::operator_mutation::OperatorMutationCommitReceipt;
use crate::operator_mutation::OperatorMutationPlanReceipt;
use crate::operator_mutation_reconciliation::OperatorMutationReconciliationHttpResponse;
use crate::preference_ingress::NativePreferenceIngress;
use crate::preference_ingress::NativePreferenceIngressConfig;
use crate::preference_ingress::PreferenceHttpResponse;
use crate::runtime_ingress::RuntimeIngressKind;
use crate::runtime_ingress::runtime_ingress_lifecycle;
use crate::runtime_mutation::RuntimeMutationCanaryReceipt;
#[cfg(all(test, unix))]
use crate::secure_key_file::PRIVATE_FILE_MODE;
use crate::secure_key_file::read_private_key;
use crate::telegram_authority::TelegramAuthority;
use crate::telegram_authority::TelegramAuthorityCommitReceipt;
use crate::telegram_authority::TelegramAuthorityConfig;
use crate::telegram_authority::TelegramAuthorityPlanReceipt;
use crate::telegram_authority::TelegramPipelineReceipt;
use crate::telegram_authority::TelegramReconciliationHttpResponse;

const OUTCOME_DATABASE_ENV: &str = "HEPTA_RUNTIME_OUTCOME_DATABASE";
const STATE_DATABASE_ENV: &str = "HEPTA_RUNTIME_STATE_DATABASE";
const INTEGRITY_KEY_FILE_ENV: &str = "HEPTA_RUNTIME_INTEGRITY_KEY_FILE";
const OUTCOME_MODE_ENV: &str = "HEPTA_RUNTIME_OUTCOME_MODE";
pub(crate) const NDU_H1_STATUS_ENDPOINT: &str = "/api/ndu/h1/status";
const NDU_H1_ENABLED_ENV: &str = "HEPTA_NDU_H1_SHADOW_ENABLED";
const NDU_H1_JOURNAL_ENV: &str = "HEPTA_NDU_H1_JOURNAL";
const NDU_H1_KILL_SWITCH_FILE_ENV: &str = "HEPTA_NDU_H1_KILL_SWITCH_FILE";
const NDU_H1_TENANT_SCOPE_HASH_ENV: &str = "HEPTA_NDU_H1_TENANT_SCOPE_HASH";
const NDU_H1_CONSENT_SCOPE_HASH_ENV: &str = "HEPTA_NDU_H1_CONSENT_SCOPE_HASH";
const NDU_H1_REVOCATION_SNAPSHOT_HASH_ENV: &str = "HEPTA_NDU_H1_REVOCATION_SNAPSHOT_HASH";
const NDU_H1_MODEL_HASH_ENV: &str = "HEPTA_NDU_H1_MODEL_HASH";
const NDU_H1_SCORER_CONFIG_HASH_ENV: &str = "HEPTA_NDU_H1_SCORER_CONFIG_HASH";
const NDU_H1_INITIAL_STATE_HASH_ENV: &str = "HEPTA_NDU_H1_INITIAL_STATE_HASH";
const NDU_H1_MAX_EVENTS_ENV: &str = "HEPTA_NDU_H1_MAX_EVENTS";
const OPEN_EXISTING_MODE: &str = "open-existing";
const BOOTSTRAP_NEW_MODE: &str = "bootstrap-new";
pub(crate) const RUNTIME_KERNEL_CANARY_ACTION_ENDPOINT: &str = "/api/actions/runtime-kernel-canary";
pub struct NativeGatewayRuntime {
    kernel: RuntimeKernel,
    preference_ingress: NativePreferenceIngress,
    effect_reconciliation: Option<EffectReconciliationAuthority>,
    monotonic_anchor: Option<Arc<ExternalMonotonicAnchor>>,
    telegram_authority: Option<TelegramAuthority>,
    ndu_h1_runtime: Option<NduH1Runtime>,
    outcome_mode: RuntimeOutcomeMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct NduH1GatewayStatus {
    schema: &'static str,
    enabled: bool,
    ready: bool,
    accepting_observations: bool,
    kill_switch_active: bool,
    shadow_only: bool,
    production_authority_granted: bool,
    observed_event_count: u64,
    recorded_count: u64,
    replay_count: u64,
    rejected_count: u64,
    journal_head: Option<String>,
    last_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct OperatorAuthorizedTelegramDrainReceipt {
    pub(crate) authorization: TelegramAuthorityCommitReceipt,
    pub(crate) pipeline: TelegramPipelineReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct RuntimeKernelCanaryReceipt {
    product: &'static str,
    runtime: &'static str,
    status: &'static str,
    action: &'static str,
    request_binding_hash: String,
    active_model_provider: String,
    active_model: String,
    invoked_tool: String,
    execution_receipt: RuntimeExecutionReceipt,
    provider_effect_ack_requirement: &'static str,
    external_network_requested: bool,
    external_side_effects: bool,
    live_surface_expanded: bool,
    raw_request_body_exposed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeRequestDisposition {
    ReadOnlyDispatch,
    PlanOnlyQuarantine,
    ExactAuthorityDispatch,
}

/// Request-bound readiness/quarantine evidence. This is neither exact tool
/// authority nor a durable execution outcome receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeRequestPreflightReceipt {
    pub(crate) request_binding_hash: String,
    pub(crate) disposition: RuntimeRequestDisposition,
    pub(crate) ingress_kind: RuntimeIngressKind,
    pub(crate) mutation_authorized: bool,
    pub(crate) durable_intent_recorded: bool,
    pub(crate) provider_effect_ack_recorded: bool,
    pub(crate) terminal_receipt_recorded: bool,
}

pub(crate) struct RuntimeTelegramReceiveAuthority {
    request_binding_hash: String,
}

impl RuntimeTelegramReceiveAuthority {
    pub(crate) fn request_binding_hash(&self) -> &str {
        &self.request_binding_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeTelegramDrainPreflightReceipt {
    pub(crate) request_binding_hash: String,
    pub(crate) live_read_authorized: bool,
    pub(crate) model_invocation_authorized: bool,
    pub(crate) send_authorized: bool,
    pub(crate) durable_intent_recorded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(crate) struct OperatorAuthorityChallengeReceipt {
    schema: &'static str,
    status: &'static str,
    session_binding_hash: String,
    preference_session_binding_hash: String,
    telegram_session_binding_hash: Option<String>,
    telegram_execution_identity_hash: Option<String>,
    telegram_cursor: Option<i64>,
    operator_mutation_enabled: bool,
    telegram_pipeline_enabled: bool,
    effect_reconciliation_enabled: bool,
    challenge_endpoint: &'static str,
    operator_plan_endpoint: &'static str,
    operator_commit_endpoint: &'static str,
    operator_reconciliation_inspect_endpoint: &'static str,
    operator_reconciliation_resolve_endpoint: &'static str,
    telegram_plan_endpoint: &'static str,
    telegram_commit_endpoint: &'static str,
    preference_challenge_endpoint: &'static str,
    preference_commit_endpoint: &'static str,
    effect_reconciliation_inspect_endpoint: &'static str,
    effect_reconciliation_resolve_endpoint: &'static str,
    authorization_request_binding_domain: &'static str,
    authorization_request_binding_framing: &'static str,
    authorization_request_body: &'static str,
    operator_plan_proof_domain: &'static str,
    operator_plan_proof_fields: [&'static str; 4],
    operator_commit_proof_domain: &'static str,
    operator_commit_proof_fields: [&'static str; 7],
    operator_reconciliation_proof_domain: &'static str,
    operator_reconciliation_proof_fields: [&'static str; 8],
    telegram_plan_proof_domain: &'static str,
    telegram_plan_proof_fields: [&'static str; 4],
    telegram_commit_proof_domain: &'static str,
    telegram_commit_proof_fields: [&'static str; 5],
    effect_reconciliation_proof_domain: &'static str,
    effect_reconciliation_proof_fields: [&'static str; 7],
    secret_material_returned: bool,
    external_effect_performed: bool,
}

#[derive(Debug)]
pub(crate) enum RuntimeTelegramDrainAdmissionError {
    ExactAuthorityUnavailable,
}

impl fmt::Display for RuntimeTelegramDrainAdmissionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("telegram_runtime_admission.exact_authority_unavailable")
    }
}

impl std::error::Error for RuntimeTelegramDrainAdmissionError {}

pub(crate) struct NativePostRuntimeGateInputs {
    pub(crate) real_handler_enabled: bool,
    pub(crate) operator_approval_enabled: bool,
}

impl RuntimeRequestPreflightReceipt {
    pub(crate) fn native_post_gate_inputs(
        &self,
        configured_enablement: bool,
        configured_approval: bool,
    ) -> NativePostRuntimeGateInputs {
        let exact_effect_authority = self.disposition
            == RuntimeRequestDisposition::PlanOnlyQuarantine
            && self.mutation_authorized
            && self.durable_intent_recorded
            && !self.provider_effect_ack_recorded
            && !self.terminal_receipt_recorded;
        NativePostRuntimeGateInputs {
            real_handler_enabled: configured_enablement && exact_effect_authority,
            operator_approval_enabled: configured_approval && exact_effect_authority,
        }
    }
}

impl RuntimeTelegramDrainPreflightReceipt {
    pub(crate) fn require_live_pipeline_authority(
        &self,
    ) -> std::result::Result<(), RuntimeTelegramDrainAdmissionError> {
        if self.live_read_authorized
            && self.model_invocation_authorized
            && self.send_authorized
            && self.durable_intent_recorded
        {
            Ok(())
        } else {
            Err(RuntimeTelegramDrainAdmissionError::ExactAuthorityUnavailable)
        }
    }
}

pub(crate) fn runtime_kernel_canary_body_admitted(body: Option<&str>) -> bool {
    let Some(body) = body else {
        return false;
    };
    let Ok(serde_json::Value::Object(object)) = serde_json::from_str(body) else {
        return false;
    };
    object.len() == 1 && object.get("dry_run") == Some(&serde_json::Value::Bool(true))
}

impl fmt::Debug for NativeGatewayRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeGatewayRuntime")
            .field("outcome_mode", &self.outcome_mode.as_str())
            .field("integrity_key", &"[REDACTED]")
            .field("preference_ingress", &self.preference_ingress)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuntimeOutcomeMode {
    OpenExisting,
    BootstrapNew,
}

impl RuntimeOutcomeMode {
    fn parse(value: &str) -> Result<Self> {
        match value {
            OPEN_EXISTING_MODE => Ok(Self::OpenExisting),
            BOOTSTRAP_NEW_MODE => Ok(Self::BootstrapNew),
            _ => anyhow::bail!(
                "{OUTCOME_MODE_ENV} must be {OPEN_EXISTING_MODE} or {BOOTSTRAP_NEW_MODE}"
            ),
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::OpenExisting => OPEN_EXISTING_MODE,
            Self::BootstrapNew => BOOTSTRAP_NEW_MODE,
        }
    }
}

struct RuntimeCompositionConfig {
    outcome_database: PathBuf,
    state_database: PathBuf,
    integrity_key_file: PathBuf,
    outcome_mode: RuntimeOutcomeMode,
}

struct NduH1RuntimeConfig {
    config: hepta_intelligence::NduH1ShadowConfig,
    journal_path: PathBuf,
    kill_switch_path: PathBuf,
}

impl NduH1RuntimeConfig {
    fn from_env() -> Result<Option<Self>> {
        let enabled = env::var(NDU_H1_ENABLED_ENV)
            .ok()
            .is_some_and(|value| value.trim() == "1");
        if !enabled {
            return Ok(None);
        }
        let journal_path = required_absolute_path(NDU_H1_JOURNAL_ENV)?;
        let kill_switch_path = required_absolute_path(NDU_H1_KILL_SWITCH_FILE_ENV)?;
        let max_events = required_env(NDU_H1_MAX_EVENTS_ENV)?
            .parse::<u64>()
            .with_context(|| format!("{NDU_H1_MAX_EVENTS_ENV} must be a positive integer"))?;
        if max_events == 0 {
            anyhow::bail!("{NDU_H1_MAX_EVENTS_ENV} must be greater than zero");
        }
        let hash = |name| -> Result<hepta_contracts::ContentHash> {
            Ok(hepta_contracts::ContentHash::new(required_env(name)?))
        };
        Ok(Some(Self {
            config: hepta_intelligence::NduH1ShadowConfig::new(
                hash(NDU_H1_TENANT_SCOPE_HASH_ENV)?,
                hash(NDU_H1_CONSENT_SCOPE_HASH_ENV)?,
                hash(NDU_H1_REVOCATION_SNAPSHOT_HASH_ENV)?,
                hash(NDU_H1_MODEL_HASH_ENV)?,
                hash(NDU_H1_SCORER_CONFIG_HASH_ENV)?,
                hash(NDU_H1_INITIAL_STATE_HASH_ENV)?,
                max_events,
                true,
            ),
            journal_path,
            kill_switch_path,
        }))
    }
}

impl RuntimeCompositionConfig {
    fn from_env() -> Result<Self> {
        let outcome_database = required_absolute_path(OUTCOME_DATABASE_ENV)?;
        let state_database = required_absolute_path(STATE_DATABASE_ENV)?;
        let integrity_key_file = required_absolute_path(INTEGRITY_KEY_FILE_ENV)?;
        let outcome_mode = env::var(OUTCOME_MODE_ENV)
            .ok()
            .map(|value| RuntimeOutcomeMode::parse(value.trim()))
            .transpose()?
            .unwrap_or(RuntimeOutcomeMode::OpenExisting);
        Ok(Self {
            outcome_database,
            state_database,
            integrity_key_file,
            outcome_mode,
        })
    }
}

impl NativeGatewayRuntime {
    /// Builds the fail-closed service composition from secure files.
    ///
    /// In addition to the keyed RuntimeKernel variables, `--serve-ui` requires
    /// `HEPTA_PREFERENCE_DATABASE`, `HEPTA_PREFERENCE_INTEGRITY_KEY_FILE`, and
    /// `HEPTA_PREFERENCE_INGRESS_AUTH_KEY_FILE`. Preference storage defaults to
    /// `open-existing`; explicit first boot uses
    /// `HEPTA_PREFERENCE_STORE_MODE=bootstrap-new`.
    ///
    /// Both preference endpoints require distinct domain-separated HMAC
    /// proofs plus strict loopback Host/Origin/CSRF admission. Clear HTTP makes
    /// no confidentiality claim beyond the local host boundary.
    pub fn from_env() -> Result<Self> {
        Self::open_with_operational_controls(
            RuntimeCompositionConfig::from_env()?,
            NativePreferenceIngressConfig::from_env()?,
            EffectReconciliationConfig::from_env()?,
            ExternalMonotonicAnchorConfig::from_env()?,
            TelegramAuthorityConfig::from_env()?,
            NduH1RuntimeConfig::from_env()?,
        )
    }

    #[cfg(all(test, unix))]
    fn open(
        config: RuntimeCompositionConfig,
        preference_config: NativePreferenceIngressConfig,
    ) -> Result<Self> {
        Self::open_with_operational_controls(config, preference_config, None, None, None, None)
    }

    fn open_with_operational_controls(
        config: RuntimeCompositionConfig,
        preference_config: NativePreferenceIngressConfig,
        reconciliation_config: Option<EffectReconciliationConfig>,
        anchor_config: Option<ExternalMonotonicAnchorConfig>,
        telegram_config: Option<TelegramAuthorityConfig>,
        ndu_h1_config: Option<NduH1RuntimeConfig>,
    ) -> Result<Self> {
        require_live_mutation_anchor(
            crate::operator_mutation::enabled(),
            telegram_config.is_some(),
            anchor_config.is_some(),
        )?;
        let outcome_integrity_key = read_integrity_key(&config.integrity_key_file)?;
        let state_integrity_key = read_integrity_key(&config.integrity_key_file)?;
        let prepared_preference = NativePreferenceIngress::prepare(preference_config)?;
        let kernel = match config.outcome_mode {
            RuntimeOutcomeMode::OpenExisting => {
                RuntimeKernel::open_with_durable_outcomes_and_state(
                    &config.outcome_database,
                    outcome_integrity_key,
                    &config.state_database,
                    state_integrity_key,
                )
            }
            RuntimeOutcomeMode::BootstrapNew => {
                RuntimeKernel::bootstrap_with_durable_outcomes_and_state(
                    &config.outcome_database,
                    outcome_integrity_key,
                    &config.state_database,
                    state_integrity_key,
                )
            }
        }
        .with_context(|| {
            format!(
                "initialize keyed RuntimeKernel with {} durable outcomes",
                config.outcome_mode.as_str()
            )
        })?;
        let preference_ingress = NativePreferenceIngress::open(prepared_preference)?;
        let effect_reconciliation = reconciliation_config
            .map(EffectReconciliationAuthority::open)
            .transpose()
            .context("initialize exact effect reconciliation authority")?;
        let monotonic_anchor = anchor_config
            .map(ExternalMonotonicAnchor::open)
            .transpose()
            .context("initialize external monotonic anchor")?
            .map(Arc::new);
        let telegram_authority = telegram_config
            .map(TelegramAuthority::open)
            .transpose()
            .context("initialize operator-bound Telegram pipeline authority")?;
        let ndu_h1_runtime = ndu_h1_config
            .map(|config| {
                NduH1Runtime::open_with_kill_switch(
                    config.config,
                    config.journal_path,
                    Some(config.kill_switch_path),
                )
            })
            .transpose()
            .map_err(|error| anyhow::anyhow!("initialize NDU H1 shadow runtime: {error:?}"))?;
        let runtime = Self {
            kernel,
            preference_ingress,
            effect_reconciliation,
            monotonic_anchor,
            telegram_authority,
            ndu_h1_runtime,
            outcome_mode: config.outcome_mode,
        };
        runtime
            .hydrate_authenticated_preference_context()
            .context("hydrate authenticated preference context at startup")?;
        runtime
            .synchronize_durable_anchor()
            .context("verify and advance external monotonic anchor at startup")?;
        Ok(runtime)
    }

    pub(crate) fn validate_readiness(&self) -> Result<()> {
        self.kernel
            .model_selection()
            .map_err(|error| anyhow::anyhow!("attached RuntimeKernel readiness failed: {error}"))?;
        self.preference_ingress
            .validate_readiness()
            .context("attached trusted preference ingress readiness failed")?;
        self.synchronize_durable_anchor()
            .context("external monotonic anchor readiness failed")
    }

    pub(crate) fn ndu_h1_status(&self) -> Result<NduH1GatewayStatus> {
        let Some(runtime) = &self.ndu_h1_runtime else {
            return Ok(NduH1GatewayStatus {
                schema: "hepta_ndu_h1_gateway_status_v1",
                enabled: false,
                ready: false,
                accepting_observations: false,
                kill_switch_active: false,
                shadow_only: true,
                production_authority_granted: false,
                observed_event_count: 0,
                recorded_count: 0,
                replay_count: 0,
                rejected_count: 0,
                journal_head: None,
                last_error: None,
            });
        };
        let NduH1RuntimeStatus {
            ready,
            accepting_observations,
            kill_switch_active,
            shadow_only,
            production_authority_granted,
            observed_event_count,
            recorded_count,
            replay_count,
            rejected_count,
            journal_head,
            last_error,
            ..
        } = runtime
            .status()
            .map_err(|error| anyhow::anyhow!("read NDU H1 shadow status: {error:?}"))?;
        Ok(NduH1GatewayStatus {
            schema: "hepta_ndu_h1_gateway_status_v1",
            enabled: true,
            ready,
            accepting_observations,
            kill_switch_active,
            shadow_only,
            production_authority_granted,
            observed_event_count,
            recorded_count,
            replay_count,
            rejected_count,
            journal_head: Some(journal_head),
            last_error,
        })
    }

    fn observe_ndu_h1_runtime_receipt(
        &self,
        request_binding_hash: &str,
        receipt: &RuntimeExecutionReceipt,
    ) {
        let Some(runtime) = &self.ndu_h1_runtime else {
            return;
        };
        let satisfied = hepta_intelligence::HardFeasibilityVerdict::Satisfied;
        let event = NduH1ShadowEvent {
            event_hash: hepta_contracts::ContentHash::new(receipt.terminal_outcome_hash.clone()),
            source_receipt_hash: hepta_contracts::ContentHash::new(
                receipt.terminal_receipt_hash.clone(),
            ),
            subject_pseudonym_hash: hepta_contracts::ContentHash::new(
                request_binding_hash.to_owned(),
            ),
            explicit_preference_evidence_hash: None,
            task_signal_basis_points: if receipt.terminal_status == "succeeded" {
                10_000
            } else {
                -10_000
            },
            learning_signal_basis_points: 0,
            trust_signal_basis_points: if receipt.durable_intent_recorded {
                10_000
            } else {
                -10_000
            },
            memory_pollution_risk_basis_points: 0,
            resource_cost_basis_points: if receipt.effect_plan_recorded {
                1_000
            } else {
                100
            },
            uncertainty_basis_points: 0,
            propensity_basis_points: 10_000,
            delayed_outcome_hash: Some(hepta_contracts::ContentHash::new(
                receipt.terminal_evidence_hash.clone(),
            )),
            feasibility: hepta_intelligence::HardFeasibilityMask::new(
                satisfied, satisfied, satisfied, satisfied,
            ),
        };
        if let Err(error) = runtime.observe_event(event) {
            eprintln!("NDU H1 shadow observation rejected: {error:?}");
        }
    }

    pub(crate) fn preflight_request(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<RuntimeRequestPreflightReceipt> {
        self.synchronize_durable_anchor()
            .context("durable rollback anchor denied request preflight")?;
        if !matches!(method, "GET" | "POST") {
            anyhow::bail!("attached RuntimeKernel denied unsupported HTTP method");
        }
        let lifecycle = runtime_ingress_lifecycle(method, path).with_context(|| {
            format!("attached RuntimeKernel denied unclassified native ingress: {method} {path}")
        })?;
        let disposition = lifecycle.disposition();
        if !path.starts_with('/') {
            anyhow::bail!("attached RuntimeKernel denied non-origin request target");
        }
        let session_id = self
            .kernel
            .active_session_id()
            .map_err(|error| anyhow::anyhow!("attached RuntimeKernel session failed: {error}"))?;
        let model = self
            .kernel
            .model_selection_for_session(&session_id)
            .map_err(|error| anyhow::anyhow!("attached RuntimeKernel model failed: {error}"))?
            .active;
        let mut hasher = Sha256::new();
        let proof_excluded_body = proof_excluded_authorization_body(path, body)?;
        let mut values = vec![
            if proof_excluded_body.is_some() {
                b"hepta-native-gateway-authorization-request-v2".as_slice()
            } else {
                b"hepta-native-gateway-runtime-request-v1".as_slice()
            },
            method.as_bytes(),
            path.as_bytes(),
            proof_excluded_body
                .as_deref()
                .map(str::as_bytes)
                .unwrap_or_else(|| body.unwrap_or_default().as_bytes()),
        ];
        if proof_excluded_body.is_none() {
            values.extend([
                session_id.as_bytes(),
                model.provider.as_bytes(),
                model.model.as_bytes(),
            ]);
        }
        for value in values {
            hasher.update((value.len() as u64).to_be_bytes());
            hasher.update(value);
        }
        Ok(RuntimeRequestPreflightReceipt {
            request_binding_hash: format!("{:x}", hasher.finalize()),
            disposition,
            ingress_kind: lifecycle.ingress_kind(),
            mutation_authorized: false,
            durable_intent_recorded: false,
            provider_effect_ack_recorded: false,
            terminal_receipt_recorded: false,
        })
    }

    pub(crate) fn authorize_telegram_receive(&self) -> Result<RuntimeTelegramReceiveAuthority> {
        let receipt = self.preflight_telegram_drain(/*next_update_offset*/ None)?;
        if receipt.live_read_authorized && receipt.durable_intent_recorded {
            return Ok(RuntimeTelegramReceiveAuthority {
                request_binding_hash: receipt.request_binding_hash,
            });
        }
        anyhow::bail!("telegram_runtime_admission.exact_read_authority_unavailable")
    }

    pub(crate) fn preflight_telegram_drain(
        &self,
        next_update_offset: Option<i64>,
    ) -> Result<RuntimeTelegramDrainPreflightReceipt> {
        let session_id = self
            .kernel
            .active_session_id()
            .map_err(|error| anyhow::anyhow!("attached RuntimeKernel session failed: {error}"))?;
        let model = self
            .kernel
            .model_selection_for_session(&session_id)
            .map_err(|error| anyhow::anyhow!("attached RuntimeKernel model failed: {error}"))?
            .active;
        let mut hasher = Sha256::new();
        for value in [
            b"hepta-native-gateway-telegram-drain-v1".as_slice(),
            next_update_offset
                .map(|offset| offset.to_string())
                .unwrap_or_default()
                .as_bytes(),
            session_id.as_bytes(),
            model.provider.as_bytes(),
            model.model.as_bytes(),
        ] {
            hasher.update((value.len() as u64).to_be_bytes());
            hasher.update(value);
        }
        Ok(RuntimeTelegramDrainPreflightReceipt {
            request_binding_hash: format!("{:x}", hasher.finalize()),
            live_read_authorized: false,
            model_invocation_authorized: false,
            send_authorized: false,
            durable_intent_recorded: false,
        })
    }

    pub(crate) const fn outcome_mode(&self) -> &'static str {
        self.outcome_mode.as_str()
    }

    pub(crate) const fn preference_mode(&self) -> &'static str {
        self.preference_ingress.mode()
    }

    pub(crate) const fn effect_reconciliation_enabled(&self) -> bool {
        self.effect_reconciliation.is_some()
    }

    pub(crate) const fn monotonic_anchor_enabled(&self) -> bool {
        self.monotonic_anchor.is_some()
    }

    pub(crate) const fn telegram_operator_pipeline_enabled(&self) -> bool {
        self.telegram_authority.is_some()
    }

    pub(crate) fn route_preference_ingress(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
        request_binding_hash: &str,
    ) -> Option<PreferenceHttpResponse> {
        let (session_id, session_binding_hash) = match self.preference_session_binding() {
            Ok(binding) => binding,
            Err(_) => {
                return Some(PreferenceHttpResponse {
                    status: "503 Service Unavailable",
                    body: r#"{"error":"trusted_preference_ingress.runtime_session_unavailable"}"#
                        .to_string(),
                    preference_context: None,
                });
            }
        };
        let anchor_lease = if method == "POST"
            && path == crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT
        {
            if self.monotonic_anchor.is_none() {
                return Some(PreferenceHttpResponse {
                    status: "503 Service Unavailable",
                    body: r#"{"error":"trusted_preference_ingress.monotonic_anchor_failed"}"#
                        .to_string(),
                    preference_context: None,
                });
            }
            if let Some(response) = self
                .preference_ingress
                .prevalidate_commit_http(body, &session_binding_hash)
            {
                return Some(response);
            }
            match self.begin_required_durable_effect_anchor_lease() {
                Ok(lease) => Some(lease),
                Err(_) => {
                    return Some(PreferenceHttpResponse {
                        status: "503 Service Unavailable",
                        body: r#"{"error":"trusted_preference_ingress.monotonic_anchor_failed"}"#
                            .to_string(),
                        preference_context: None,
                    });
                }
            }
        } else {
            None
        };
        let response = self.preference_ingress.route_http(
            method,
            path,
            body,
            request_binding_hash,
            &session_binding_hash,
        );
        let Some(mut response) = response else {
            if self
                .finalize_durable_effect_anchor_lease(anchor_lease)
                .is_err()
            {
                return Some(PreferenceHttpResponse {
                    status: "503 Service Unavailable",
                    body: r#"{"error":"trusted_preference_ingress.monotonic_anchor_failed"}"#
                        .to_string(),
                    preference_context: None,
                });
            }
            return None;
        };
        if let Some(stamp) = response.preference_context.take()
            && self
                .kernel
                .attach_authenticated_preference_context(&session_id, stamp)
                .is_err()
        {
            response = PreferenceHttpResponse {
                status: "503 Service Unavailable",
                body: r#"{"error":"trusted_preference_ingress.runtime_attachment_failed"}"#
                    .to_string(),
                preference_context: None,
            };
        }
        if self
            .finalize_durable_effect_anchor_lease(anchor_lease)
            .is_err()
        {
            return Some(PreferenceHttpResponse {
                status: "503 Service Unavailable",
                body: r#"{"error":"trusted_preference_ingress.monotonic_anchor_failed"}"#
                    .to_string(),
                preference_context: None,
            });
        }
        Some(response)
    }

    pub(crate) fn preference_session_binding(&self) -> Result<(String, String)> {
        let session_id = self
            .kernel
            .active_session_id()
            .map_err(|error| anyhow::anyhow!("active preference session unavailable: {error}"))?;
        let mut hasher = Sha256::new();
        for value in [
            b"hepta-native-preference-session-binding-v1".as_slice(),
            session_id.as_bytes(),
        ] {
            hasher.update((value.len() as u64).to_be_bytes());
            hasher.update(value);
        }
        Ok((session_id, format!("sha256:{:x}", hasher.finalize())))
    }

    fn operator_runtime_session_binding(&self) -> Result<String> {
        let session_id = self
            .kernel
            .active_session_id()
            .map_err(|error| anyhow::anyhow!("active operator session unavailable: {error}"))?;
        let model = self
            .kernel
            .model_selection_for_session(&session_id)
            .map_err(|error| anyhow::anyhow!("active operator model unavailable: {error}"))?
            .active;
        let mut hasher = Sha256::new();
        for value in [
            b"hepta-native-operator-session-binding-v1".as_slice(),
            session_id.as_bytes(),
            model.provider.as_bytes(),
            model.model.as_bytes(),
        ] {
            hasher.update((value.len() as u64).to_be_bytes());
            hasher.update(value);
        }
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn operator_telegram_runtime_session_binding(
        &self,
    ) -> Result<(String, String, OperatorTelegramExecutionIdentity)> {
        let session_id = self
            .kernel
            .active_session_id()
            .map_err(|error| anyhow::anyhow!("active Telegram session unavailable: {error}"))?;
        let model = self
            .kernel
            .model_selection_for_session(&session_id)
            .map_err(|error| anyhow::anyhow!("active Telegram model unavailable: {error}"))?
            .active;
        let execution_identity = crate::native_telegram::operator_telegram_execution_identity(
            &model.provider,
            &model.model,
        )?;
        let execution_identity_hash = execution_identity.binding_hash()?;
        let mut hasher = Sha256::new();
        for value in [
            b"hepta-native-operator-telegram-session-binding-v1".as_slice(),
            session_id.as_bytes(),
            model.provider.as_bytes(),
            model.model.as_bytes(),
            execution_identity_hash.as_bytes(),
        ] {
            hasher.update((value.len() as u64).to_be_bytes());
            hasher.update(value);
        }
        Ok((
            format!("{:x}", hasher.finalize()),
            execution_identity_hash,
            execution_identity,
        ))
    }

    pub(crate) fn operator_authority_challenge(&self) -> Result<OperatorAuthorityChallengeReceipt> {
        let telegram_pipeline_enabled = self.telegram_operator_pipeline_enabled();
        let (telegram_session_binding_hash, telegram_execution_identity_hash) =
            if telegram_pipeline_enabled {
                let (session_binding_hash, execution_identity_hash, _) =
                    self.operator_telegram_runtime_session_binding()?;
                (Some(session_binding_hash), Some(execution_identity_hash))
            } else {
                (None, None)
            };
        let (_, preference_session_binding_hash) = self.preference_session_binding()?;
        Ok(OperatorAuthorityChallengeReceipt {
            schema: "hepta.native.operator-authority-challenge.v1",
            status: "ready",
            session_binding_hash: self.operator_runtime_session_binding()?,
            preference_session_binding_hash,
            telegram_session_binding_hash,
            telegram_execution_identity_hash,
            telegram_cursor: if telegram_pipeline_enabled {
                self.current_telegram_cursor()?
            } else {
                None
            },
            operator_mutation_enabled: crate::operator_mutation::enabled(),
            telegram_pipeline_enabled,
            effect_reconciliation_enabled: self.effect_reconciliation_enabled(),
            challenge_endpoint: crate::runtime_ingress::OPERATOR_AUTHORITY_CHALLENGE_ENDPOINT,
            operator_plan_endpoint: crate::operator_mutation::OPERATOR_MUTATION_PLAN_ENDPOINT,
            operator_commit_endpoint: crate::operator_mutation::OPERATOR_MUTATION_COMMIT_ENDPOINT,
            operator_reconciliation_inspect_endpoint:
                crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT,
            operator_reconciliation_resolve_endpoint:
                crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT,
            telegram_plan_endpoint: crate::telegram_authority::TELEGRAM_AUTHORITY_PLAN_ENDPOINT,
            telegram_commit_endpoint: crate::telegram_authority::TELEGRAM_AUTHORITY_COMMIT_ENDPOINT,
            preference_challenge_endpoint: crate::preference_ingress::PREFERENCE_CHALLENGE_ENDPOINT,
            preference_commit_endpoint: crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT,
            effect_reconciliation_inspect_endpoint:
                crate::effect_reconciliation::EFFECT_RECONCILIATION_INSPECT_ENDPOINT,
            effect_reconciliation_resolve_endpoint:
                crate::effect_reconciliation::EFFECT_RECONCILIATION_RESOLVE_ENDPOINT,
            authorization_request_binding_domain: "hepta-native-gateway-authorization-request-v2",
            authorization_request_binding_framing: "u64be-length-prefixed-fields",
            authorization_request_body: "canonical-json-without-top-level-proof",
            operator_plan_proof_domain: "hepta.native.operator-note.plan.v1",
            operator_plan_proof_fields: [
                "mutation_id",
                "note",
                "plan_request_binding_hash",
                "session_binding_hash",
            ],
            operator_commit_proof_domain: "hepta.native.operator-note.commit.v1",
            operator_commit_proof_fields: [
                "mutation_id",
                "note",
                "plan_hash",
                "candidate_binding_hash",
                "plan_request_binding_hash",
                "session_binding_hash",
                "commit_request_binding_hash",
            ],
            operator_reconciliation_proof_domain:
                crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_PROOF_DOMAIN,
            operator_reconciliation_proof_fields: [
                "method",
                "path",
                "session_binding_hash",
                "plan_hash",
                "attempt_id",
                "effect_plan_hash",
                "request_binding_hash",
                "decision",
            ],
            telegram_plan_proof_domain: "hepta.telegram.operator-authority.plan.v1",
            telegram_plan_proof_fields: [
                "request_id",
                "cursor_binding",
                "plan_request_binding_hash",
                "session_binding_hash",
            ],
            telegram_commit_proof_domain: "hepta.telegram.operator-authority.commit.v1",
            telegram_commit_proof_fields: [
                "request_id",
                "plan_hash",
                "plan_request_binding_hash",
                "commit_request_binding_hash",
                "session_binding_hash",
            ],
            effect_reconciliation_proof_domain: "hepta.operator-effect-reconciliation.hmac-sha256.v1",
            effect_reconciliation_proof_fields: [
                "method",
                "path",
                "session_binding_hash",
                "attempt_id",
                "effect_plan_hash",
                "request_binding_hash",
                "decision",
            ],
            secret_material_returned: false,
            external_effect_performed: false,
        })
    }

    fn current_telegram_cursor(&self) -> Result<Option<i64>> {
        let status = crate::native_telegram::telegram_cursor_status(true);
        if !status.cursor_parse_ok {
            anyhow::bail!(
                "operator Telegram pipeline denied unreadable durable cursor: {}",
                status
                    .error
                    .as_deref()
                    .unwrap_or("unknown cursor validation error")
            );
        }
        Ok(status.next_update_offset)
    }

    pub(crate) fn plan_operator_telegram_drain(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
    ) -> Result<TelegramAuthorityPlanReceipt> {
        let authority = self
            .telegram_authority
            .as_ref()
            .context("operator Telegram pipeline is disabled")?;
        let (session_binding, _, _) = self.operator_telegram_runtime_session_binding()?;
        let cursor = self.current_telegram_cursor()?;
        self.require_durable_anchor_configured()?;
        authority
            .prevalidate_plan(body, request_binding_hash, &session_binding, cursor)
            .context("prevalidate Telegram authority plan before anchor reservation")?;
        let anchor_lease = self
            .begin_required_durable_effect_anchor_lease()
            .context("anchor Telegram authority plan before state mutation")?;
        let result = authority.plan(body, request_binding_hash, &session_binding, cursor);
        let anchor_result = self.finalize_durable_effect_anchor_lease(Some(anchor_lease));
        combine_effect_with_anchor(result, anchor_result, "anchor Telegram authority plan")
    }

    pub(crate) fn commit_operator_telegram_drain(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
    ) -> Result<OperatorAuthorizedTelegramDrainReceipt> {
        let authority = self
            .telegram_authority
            .as_ref()
            .context("operator Telegram pipeline is disabled")?;
        let (session_binding, _, execution_identity) =
            self.operator_telegram_runtime_session_binding()?;
        let cursor = self.current_telegram_cursor()?;
        self.require_durable_anchor_configured()?;
        authority
            .prevalidate_authorize(body, request_binding_hash, &session_binding, cursor)
            .context("prevalidate Telegram authorization before anchor reservation")?;
        let anchor_lease = self
            .begin_required_durable_effect_anchor_lease()
            .context("anchor Telegram pipeline before external effects")?;
        let result = (|| {
            let (authorization, permit) =
                authority.authorize(body, request_binding_hash, &session_binding, cursor)?;
            let pipeline = crate::native_telegram::execute_operator_authorized_telegram_drain(
                permit,
                execution_identity,
            )?;
            Ok(OperatorAuthorizedTelegramDrainReceipt {
                authorization,
                pipeline,
            })
        })();
        let anchor_result = self.finalize_durable_effect_anchor_lease(Some(anchor_lease));
        combine_effect_with_anchor(
            result,
            anchor_result,
            "anchor Telegram pipeline terminal state",
        )
    }

    #[cfg(test)]
    pub(crate) fn authenticated_preference_context_for_test(
        &self,
    ) -> Result<Option<RevisionStamp>> {
        let session_id = self.kernel.active_session_id()?;
        self.kernel
            .authenticated_preference_context(&session_id)
            .map_err(Into::into)
    }

    pub(crate) fn execute_runtime_kernel_canary(
        &self,
        request_binding_hash: &str,
    ) -> Result<RuntimeKernelCanaryReceipt> {
        if request_binding_hash.len() != 64
            || !request_binding_hash
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        {
            anyhow::bail!("runtime canary requires a canonical request binding");
        }
        self.require_durable_anchor_configured()?;
        let anchor_lease = self
            .begin_required_durable_effect_anchor_lease()
            .context("anchor RuntimeKernel canary before durable outcome")?;
        let result = (|| -> Result<RuntimeKernelCanaryReceipt> {
            let session_id = "native-gateway:runtime-kernel-canary";
            self.kernel
                .switch_model_in_session(session_id, "demo/demo-chat")
                .map_err(|error| anyhow::anyhow!("select isolated canary model: {error}"))?;
            let executor = tokio::runtime::Builder::new_current_thread()
                .build()
                .context("build isolated RuntimeKernel canary executor")?;
            let result = executor
                .block_on(self.kernel.run_demo_turn_in_session(
                    session_id,
                    &format!(
                        "Use the echo tool with arguments exactly {{\"text\":\"{request_binding_hash}\"}}. Do not answer directly."
                    ),
                ))
                .map_err(|error| anyhow::anyhow!("execute RuntimeKernel canary: {error}"))?;
            let receipt = result
                .execution_receipt
                .context("RuntimeKernel canary completed without an execution receipt")?;
            if result.active_model.provider != "demo"
                || result.active_model.model != "demo-chat"
                || result.invoked_tool.as_deref() != Some("echo")
                || !receipt.durable_intent_recorded
                || receipt.effect_plan_recorded
                || receipt.provider_effect_ack_hash.is_some()
                || receipt.terminal_status != "succeeded"
            {
                anyhow::bail!("RuntimeKernel canary lifecycle evidence failed closed");
            }
            self.observe_ndu_h1_runtime_receipt(request_binding_hash, &receipt);
            Ok(RuntimeKernelCanaryReceipt {
                product: "Hepta",
                runtime: "hepta",
                status: "succeeded",
                action: "runtime-kernel-canary",
                request_binding_hash: request_binding_hash.to_string(),
                active_model_provider: result.active_model.provider,
                active_model: result.active_model.model,
                invoked_tool: "echo".into(),
                execution_receipt: receipt,
                provider_effect_ack_requirement: "not_applicable_read_only_tool",
                external_network_requested: false,
                external_side_effects: false,
                live_surface_expanded: false,
                raw_request_body_exposed: false,
            })
        })();
        let anchor_result = self.finalize_durable_effect_anchor_lease(Some(anchor_lease));
        combine_effect_with_anchor(result, anchor_result, "anchor RuntimeKernel canary outcome")
    }

    pub(crate) fn execute_runtime_mutation_canary(
        &self,
        request_binding_hash: &str,
        idempotency_key: &str,
    ) -> Result<RuntimeMutationCanaryReceipt> {
        self.require_durable_anchor_configured()?;
        crate::runtime_mutation::prevalidate(request_binding_hash, idempotency_key)
            .context("prevalidate runtime mutation canary before anchor reservation")?;
        let anchor_lease = self
            .begin_required_durable_effect_anchor_lease()
            .context("anchor runtime mutation canary before local effect")?;
        let result =
            crate::runtime_mutation::execute(&self.kernel, request_binding_hash, idempotency_key);
        let anchor_result = self.finalize_durable_effect_anchor_lease(Some(anchor_lease));
        combine_effect_with_anchor(
            result,
            anchor_result,
            "anchor runtime mutation canary outcome",
        )
    }

    pub(crate) fn plan_operator_mutation(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
    ) -> Result<OperatorMutationPlanReceipt> {
        let session_binding_hash = self.operator_runtime_session_binding()?;
        self.require_durable_anchor_configured()?;
        crate::operator_mutation::prevalidate_plan(
            body,
            request_binding_hash,
            &session_binding_hash,
        )
        .context("prevalidate operator mutation plan before anchor reservation")?;
        let anchor_lease = self
            .begin_required_durable_effect_anchor_lease()
            .context("anchor operator mutation before plan state mutation")?;
        let result = crate::operator_mutation::plan(
            &self.kernel,
            body,
            request_binding_hash,
            &session_binding_hash,
        );
        let anchor_result = self.finalize_durable_effect_anchor_lease(Some(anchor_lease));
        combine_effect_with_anchor(result, anchor_result, "anchor operator mutation plan")
    }

    pub(crate) fn commit_operator_mutation(
        &self,
        body: Option<&str>,
        request_binding_hash: &str,
    ) -> Result<OperatorMutationCommitReceipt> {
        let session_binding_hash = self.operator_runtime_session_binding()?;
        self.require_durable_anchor_configured()?;
        crate::operator_mutation::prevalidate_commit(
            body,
            request_binding_hash,
            &session_binding_hash,
        )
        .context("prevalidate operator mutation commit before anchor reservation")?;
        let anchor_lease = self
            .begin_required_durable_effect_anchor_lease()
            .context("anchor operator mutation before local effect")?;
        let result = crate::operator_mutation::commit(
            &self.kernel,
            body,
            request_binding_hash,
            &session_binding_hash,
        );
        let anchor_result = self.finalize_durable_effect_anchor_lease(Some(anchor_lease));
        combine_effect_with_anchor(result, anchor_result, "anchor operator mutation outcome")
    }

    pub(crate) fn route_operator_mutation_reconciliation(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
        request_binding_hash: &str,
    ) -> Option<OperatorMutationReconciliationHttpResponse> {
        if !matches!(
            path,
            crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT
                | crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT
        ) {
            return None;
        }
        if !crate::operator_mutation::enabled() {
            return Some(OperatorMutationReconciliationHttpResponse {
                status: "403 Forbidden",
                body: r#"{"error":"operator_mutation_reconciliation.disabled"}"#.to_string(),
                journal_state_changed: false,
            });
        }
        let session_binding_hash = match self.operator_runtime_session_binding() {
            Ok(binding) => binding,
            Err(_) => {
                return Some(OperatorMutationReconciliationHttpResponse {
                    status: "503 Service Unavailable",
                    body:
                        r#"{"error":"operator_mutation_reconciliation.runtime_session_unavailable"}"#
                            .to_string(),
                    journal_state_changed: false,
                });
            }
        };
        let anchor_lease = if method == "POST"
            && path
                == crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT
        {
            if self.monotonic_anchor.is_none() {
                return Some(OperatorMutationReconciliationHttpResponse {
                    status: "503 Service Unavailable",
                    body:
                        r#"{"error":"operator_mutation_reconciliation.monotonic_anchor_failed"}"#
                            .to_string(),
                    journal_state_changed: false,
                });
            }
            if let Some(response) =
                crate::operator_mutation_reconciliation::prevalidate_resolve_http(
                    &self.kernel,
                    body,
                    request_binding_hash,
                    &session_binding_hash,
                )
            {
                return Some(response);
            }
            match self.begin_required_durable_effect_anchor_lease() {
                Ok(lease) => Some(lease),
                Err(_) => {
                    return Some(OperatorMutationReconciliationHttpResponse {
                        status: "503 Service Unavailable",
                        body:
                            r#"{"error":"operator_mutation_reconciliation.monotonic_anchor_failed"}"#
                                .to_string(),
                        journal_state_changed: false,
                    });
                }
            }
        } else {
            None
        };
        let response = crate::operator_mutation_reconciliation::route_http(
            &self.kernel,
            method,
            path,
            body,
            request_binding_hash,
            &session_binding_hash,
        );
        let anchor_result = self.finalize_durable_effect_anchor_lease(anchor_lease);
        let Some(response) = response else {
            if anchor_result.is_err() {
                return Some(OperatorMutationReconciliationHttpResponse {
                    status: "503 Service Unavailable",
                    body: r#"{"error":"operator_mutation_reconciliation.monotonic_anchor_failed"}"#
                        .to_string(),
                    journal_state_changed: false,
                });
            }
            return None;
        };
        debug_assert!(
            !response.journal_state_changed
                || path
                    == crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT
        );
        if anchor_result.is_err() {
            return Some(OperatorMutationReconciliationHttpResponse {
                status: "503 Service Unavailable",
                body: r#"{"error":"operator_mutation_reconciliation.monotonic_anchor_failed"}"#
                    .to_string(),
                journal_state_changed: false,
            });
        }
        Some(response)
    }

    pub(crate) fn route_effect_reconciliation(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
        request_binding_hash: &str,
    ) -> Option<EffectReconciliationHttpResponse> {
        if !matches!(
            path,
            crate::effect_reconciliation::EFFECT_RECONCILIATION_INSPECT_ENDPOINT
                | crate::effect_reconciliation::EFFECT_RECONCILIATION_RESOLVE_ENDPOINT
        ) {
            return None;
        }
        let Some(authority) = self.effect_reconciliation.as_ref() else {
            return Some(EffectReconciliationHttpResponse {
                status: "403 Forbidden",
                body: r#"{"error":"operator_effect_reconciliation.disabled"}"#.to_string(),
                outcome_state_changed: false,
            });
        };
        let session_binding_hash = match self.operator_runtime_session_binding() {
            Ok(binding) => binding,
            Err(_) => {
                return Some(EffectReconciliationHttpResponse {
                    status: "503 Service Unavailable",
                    body:
                        r#"{"error":"operator_effect_reconciliation.runtime_session_unavailable"}"#
                            .to_string(),
                    outcome_state_changed: false,
                });
            }
        };
        let anchor_lease = if method == "POST"
            && path == crate::effect_reconciliation::EFFECT_RECONCILIATION_RESOLVE_ENDPOINT
        {
            if self.monotonic_anchor.is_none() {
                return Some(EffectReconciliationHttpResponse {
                    status: "503 Service Unavailable",
                    body: r#"{"error":"operator_effect_reconciliation.monotonic_anchor_failed"}"#
                        .to_string(),
                    outcome_state_changed: false,
                });
            }
            if let Some(response) = authority.prevalidate_resolve_http(
                &self.kernel,
                body,
                request_binding_hash,
                &session_binding_hash,
            ) {
                return Some(response);
            }
            match self.begin_required_durable_effect_anchor_lease() {
                Ok(lease) => Some(lease),
                Err(_) => {
                    return Some(EffectReconciliationHttpResponse {
                        status: "503 Service Unavailable",
                        body:
                            r#"{"error":"operator_effect_reconciliation.monotonic_anchor_failed"}"#
                                .to_string(),
                        outcome_state_changed: false,
                    });
                }
            }
        } else {
            None
        };
        let response = authority.route_http(
            &self.kernel,
            method,
            path,
            body,
            request_binding_hash,
            &session_binding_hash,
        );
        let anchor_result = self.finalize_durable_effect_anchor_lease(anchor_lease);
        let Some(response) = response else {
            if anchor_result.is_err() {
                return Some(EffectReconciliationHttpResponse {
                    status: "503 Service Unavailable",
                    body: r#"{"error":"operator_effect_reconciliation.monotonic_anchor_failed"}"#
                        .to_string(),
                    outcome_state_changed: false,
                });
            }
            return None;
        };
        debug_assert!(
            !response.outcome_state_changed
                || path == crate::effect_reconciliation::EFFECT_RECONCILIATION_RESOLVE_ENDPOINT
        );
        if anchor_result.is_err() {
            return Some(EffectReconciliationHttpResponse {
                status: "503 Service Unavailable",
                body: r#"{"error":"operator_effect_reconciliation.monotonic_anchor_failed"}"#
                    .to_string(),
                outcome_state_changed: false,
            });
        }
        Some(response)
    }

    pub(crate) fn route_telegram_reconciliation(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
        request_binding_hash: &str,
    ) -> Option<TelegramReconciliationHttpResponse> {
        if !matches!(
            path,
            crate::telegram_authority::TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT
                | crate::telegram_authority::TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT
        ) {
            return None;
        }
        let Some(authority) = self.telegram_authority.as_ref() else {
            return Some(TelegramReconciliationHttpResponse {
                status: "403 Forbidden",
                body: r#"{"error":"telegram_terminal_reconciliation.disabled"}"#.to_string(),
                outcome_state_changed: false,
            });
        };
        let session_binding_hash = match self.operator_telegram_runtime_session_binding() {
            Ok((binding, _, _)) => binding,
            Err(_) => {
                return Some(TelegramReconciliationHttpResponse {
                    status: "503 Service Unavailable",
                    body:
                        r#"{"error":"telegram_terminal_reconciliation.runtime_session_unavailable"}"#
                            .to_string(),
                    outcome_state_changed: false,
                });
            }
        };
        let anchor_lease = if method == "POST"
            && path == crate::telegram_authority::TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT
        {
            if self.monotonic_anchor.is_none() {
                return Some(TelegramReconciliationHttpResponse {
                    status: "503 Service Unavailable",
                    body: r#"{"error":"telegram_terminal_reconciliation.monotonic_anchor_failed"}"#
                        .to_string(),
                    outcome_state_changed: false,
                });
            }
            if let Some(response) = authority.prevalidate_reconciliation_resolve_http(
                body,
                request_binding_hash,
                &session_binding_hash,
                Path::new(crate::native_telegram::TELEGRAM_DELIVERY_LEDGER_PATH),
                Path::new(crate::native_telegram::TELEGRAM_INGRESS_CURSOR_PATH),
            ) {
                return Some(response);
            }
            match self.begin_required_durable_effect_anchor_lease() {
                Ok(lease) => Some(lease),
                Err(_) => {
                    return Some(TelegramReconciliationHttpResponse {
                        status: "503 Service Unavailable",
                        body:
                            r#"{"error":"telegram_terminal_reconciliation.monotonic_anchor_failed"}"#
                                .to_string(),
                        outcome_state_changed: false,
                    });
                }
            }
        } else {
            None
        };
        let response = authority.route_reconciliation_http(
            method,
            path,
            body,
            request_binding_hash,
            &session_binding_hash,
            Path::new(crate::native_telegram::TELEGRAM_DELIVERY_LEDGER_PATH),
            Path::new(crate::native_telegram::TELEGRAM_INGRESS_CURSOR_PATH),
        );
        let anchor_result = self.finalize_durable_effect_anchor_lease(anchor_lease);
        let Some(response) = response else {
            if anchor_result.is_err() {
                return Some(TelegramReconciliationHttpResponse {
                    status: "503 Service Unavailable",
                    body: r#"{"error":"telegram_terminal_reconciliation.monotonic_anchor_failed"}"#
                        .to_string(),
                    outcome_state_changed: false,
                });
            }
            return None;
        };
        debug_assert!(
            !response.outcome_state_changed
                || path == crate::telegram_authority::TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT
        );
        if anchor_result.is_err() {
            return Some(TelegramReconciliationHttpResponse {
                status: "503 Service Unavailable",
                body: r#"{"error":"telegram_terminal_reconciliation.monotonic_anchor_failed"}"#
                    .to_string(),
                outcome_state_changed: false,
            });
        }
        Some(response)
    }

    fn hydrate_authenticated_preference_context(&self) -> Result<()> {
        let (session_id, session_binding_hash) = self.preference_session_binding()?;
        if let Some(stamp) = self
            .preference_ingress
            .hydrate_runtime_context(&session_binding_hash)?
        {
            self.kernel
                .attach_authenticated_preference_context(&session_id, stamp)
                .map_err(anyhow::Error::msg)?;
        }
        Ok(())
    }

    fn synchronize_durable_anchor(&self) -> Result<()> {
        let Some(anchor) = self.monotonic_anchor.as_ref() else {
            return Ok(());
        };
        anchor.verify_and_advance_with(|| self.durable_anchor_states())
    }

    fn begin_durable_effect_anchor_lease(
        &self,
    ) -> Result<Option<ExternalMonotonicAnchorEffectLease>> {
        let Some(anchor) = self.monotonic_anchor.as_ref() else {
            return Ok(None);
        };
        anchor
            .begin_effect_lease_with(|| self.durable_anchor_states())
            .map(Some)
    }

    fn begin_required_durable_effect_anchor_lease(
        &self,
    ) -> Result<ExternalMonotonicAnchorEffectLease> {
        self.begin_durable_effect_anchor_lease()?
            .context("external monotonic anchor is required for durable native mutation")
    }

    fn require_durable_anchor_configured(&self) -> Result<()> {
        self.monotonic_anchor
            .as_ref()
            .map(|_| ())
            .context("external monotonic anchor is required for durable native mutation")
    }

    fn finalize_durable_effect_anchor_lease(
        &self,
        lease: Option<ExternalMonotonicAnchorEffectLease>,
    ) -> Result<()> {
        let Some(lease) = lease else {
            return Ok(());
        };
        lease.finalize_with(|| self.durable_anchor_states())
    }

    fn durable_anchor_states(&self) -> Result<DurableAnchorStateSnapshot> {
        let telegram_state = self
            .telegram_authority
            .as_ref()
            .map(TelegramAuthority::monotonic_state)
            .transpose()
            .context("project Telegram authority monotonic state")?;
        let operator_state = crate::operator_mutation::monotonic_state()
            .context("project operator mutation monotonic state")?;
        Ok(DurableAnchorStateSnapshot {
            outcome: self
                .kernel
                .durable_outcome_monotonic_state()
                .map_err(anyhow::Error::msg)?,
            preference: self.preference_ingress.monotonic_state()?,
            telegram: telegram_state,
            operator: operator_state,
            runtime_state: self
                .kernel
                .durable_runtime_state_monotonic_state()
                .map_err(anyhow::Error::msg)?,
        })
    }

    #[cfg(all(test, unix))]
    pub(crate) fn terminal_receipt_recorded_for_test(&self, attempt_id: &str) -> Result<bool> {
        self.kernel
            .terminal_receipt_recorded(attempt_id)
            .map_err(anyhow::Error::msg)
    }

    #[cfg(all(test, unix))]
    pub(crate) fn bootstrap_for_test(root: &Path) -> Result<Self> {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(root, fs::Permissions::from_mode(0o700))?;
        let key = root.join("runtime.key");
        fs::write(
            &key,
            b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        )?;
        fs::set_permissions(&key, fs::Permissions::from_mode(PRIVATE_FILE_MODE))?;
        Self::open(
            RuntimeCompositionConfig {
                outcome_database: root.join("outcomes.sqlite3"),
                state_database: root.join("runtime-state.json"),
                integrity_key_file: key,
                outcome_mode: RuntimeOutcomeMode::BootstrapNew,
            },
            NativePreferenceIngressConfig::bootstrap_for_test(root)?,
        )
    }

    #[cfg(all(test, unix))]
    pub(crate) fn bootstrap_with_anchor_for_test(root: &Path) -> Result<Self> {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(root, fs::Permissions::from_mode(0o700))?;
        let key = root.join("runtime.key");
        fs::write(
            &key,
            b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        )?;
        fs::set_permissions(&key, fs::Permissions::from_mode(PRIVATE_FILE_MODE))?;
        Self::open_with_operational_controls(
            RuntimeCompositionConfig {
                outcome_database: root.join("outcomes.sqlite3"),
                state_database: root.join("runtime-state.json"),
                integrity_key_file: key,
                outcome_mode: RuntimeOutcomeMode::BootstrapNew,
            },
            NativePreferenceIngressConfig::bootstrap_for_test(root)?,
            None,
            Some(ExternalMonotonicAnchorConfig::for_runtime_test(root)?),
            None,
            None,
        )
    }

    #[cfg(all(test, unix))]
    pub(crate) fn bootstrap_with_ndu_for_test(root: &Path) -> Result<Self> {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(root, fs::Permissions::from_mode(0o700))?;
        let key = root.join("runtime.key");
        fs::write(
            &key,
            b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        )?;
        fs::set_permissions(&key, fs::Permissions::from_mode(PRIVATE_FILE_MODE))?;
        let hash = hepta_contracts::ContentHash::new;
        Self::open_with_operational_controls(
            RuntimeCompositionConfig {
                outcome_database: root.join("outcomes.sqlite3"),
                state_database: root.join("runtime-state.json"),
                integrity_key_file: key,
                outcome_mode: RuntimeOutcomeMode::BootstrapNew,
            },
            NativePreferenceIngressConfig::bootstrap_for_test(root)?,
            None,
            Some(ExternalMonotonicAnchorConfig::for_runtime_test(root)?),
            None,
            Some(NduH1RuntimeConfig {
                config: hepta_intelligence::NduH1ShadowConfig::new(
                    hash("tenant"),
                    hash("consent"),
                    hash("revocation"),
                    hash("model"),
                    hash("scorer"),
                    hash("initial"),
                    10,
                    true,
                ),
                journal_path: root.join("ndu-h1.jsonl"),
                kill_switch_path: root.join("ndu-h1.kill"),
            }),
        )
    }

    #[cfg(all(test, unix))]
    pub(crate) fn open_existing_with_anchor_for_test(root: &Path) -> Result<Self> {
        Self::open_with_operational_controls(
            RuntimeCompositionConfig {
                outcome_database: root.join("outcomes.sqlite3"),
                state_database: root.join("runtime-state.json"),
                integrity_key_file: root.join("runtime.key"),
                outcome_mode: RuntimeOutcomeMode::OpenExisting,
            },
            NativePreferenceIngressConfig {
                database: root.join("preferences.sqlite3"),
                integrity_key_file: root.join("preference-integrity.key"),
                authentication_key_file: root.join("preference-authentication.key"),
                mode: crate::preference_ingress::PreferenceStoreMode::OpenExisting,
            },
            None,
            Some(ExternalMonotonicAnchorConfig::for_runtime_test(root)?),
            None,
            None,
        )
    }

    #[cfg(all(test, unix))]
    pub(crate) fn open_existing_with_ndu_for_test(root: &Path) -> Result<Self> {
        let hash = hepta_contracts::ContentHash::new;
        Self::open_with_operational_controls(
            RuntimeCompositionConfig {
                outcome_database: root.join("outcomes.sqlite3"),
                state_database: root.join("runtime-state.json"),
                integrity_key_file: root.join("runtime.key"),
                outcome_mode: RuntimeOutcomeMode::OpenExisting,
            },
            NativePreferenceIngressConfig {
                database: root.join("preferences.sqlite3"),
                integrity_key_file: root.join("preference-integrity.key"),
                authentication_key_file: root.join("preference-authentication.key"),
                mode: crate::preference_ingress::PreferenceStoreMode::OpenExisting,
            },
            None,
            Some(ExternalMonotonicAnchorConfig::for_runtime_test(root)?),
            None,
            Some(NduH1RuntimeConfig {
                config: hepta_intelligence::NduH1ShadowConfig::new(
                    hash("tenant"),
                    hash("consent"),
                    hash("revocation"),
                    hash("model"),
                    hash("scorer"),
                    hash("initial"),
                    10,
                    true,
                ),
                journal_path: root.join("ndu-h1.jsonl"),
                kill_switch_path: root.join("ndu-h1.kill"),
            }),
        )
    }
}

fn required_absolute_path(env_name: &str) -> Result<PathBuf> {
    let path = env::var_os(env_name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .with_context(|| format!("{env_name} is required for --serve-ui"))?;
    if !path.is_absolute() {
        anyhow::bail!("{env_name} must be an absolute path");
    }
    Ok(path)
}

fn required_env(env_name: &str) -> Result<String> {
    env::var(env_name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .with_context(|| format!("{env_name} is required when NDU H1 shadow is enabled"))
}

fn proof_excluded_authorization_body(path: &str, body: Option<&str>) -> Result<Option<String>> {
    if !matches!(
        path,
        crate::operator_mutation::OPERATOR_MUTATION_PLAN_ENDPOINT
            | crate::operator_mutation::OPERATOR_MUTATION_COMMIT_ENDPOINT
            | crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT
            | crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT
            | crate::telegram_authority::TELEGRAM_AUTHORITY_PLAN_ENDPOINT
            | crate::telegram_authority::TELEGRAM_AUTHORITY_COMMIT_ENDPOINT
            | crate::telegram_authority::TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT
            | crate::telegram_authority::TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT
            | crate::effect_reconciliation::EFFECT_RECONCILIATION_INSPECT_ENDPOINT
            | crate::effect_reconciliation::EFFECT_RECONCILIATION_RESOLVE_ENDPOINT
    ) {
        return Ok(None);
    }
    let mut value: serde_json::Value = serde_json::from_str(
        body.context("authenticated request body is required before request binding")?,
    )
    .context("authenticated request body must be valid JSON before request binding")?;
    let object = value
        .as_object_mut()
        .context("authenticated request body must be a JSON object")?;
    let proof = object
        .remove("proof")
        .context("authenticated request body must include a proof")?;
    if !proof.is_string() {
        anyhow::bail!("authenticated request proof must be a JSON string");
    }
    let mut canonical = String::new();
    write_canonical_json(&value, &mut canonical)?;
    Ok(Some(canonical))
}

fn write_canonical_json(value: &serde_json::Value, output: &mut String) -> Result<()> {
    match value {
        serde_json::Value::Null => output.push_str("null"),
        serde_json::Value::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
        serde_json::Value::Number(value) => output.push_str(&value.to_string()),
        serde_json::Value::String(value) => {
            output.push_str(&serde_json::to_string(value).context("encode JSON string")?);
        }
        serde_json::Value::Array(values) => {
            output.push('[');
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                write_canonical_json(value, output)?;
            }
            output.push(']');
        }
        serde_json::Value::Object(values) => {
            output.push('{');
            let mut keys = values.keys().collect::<Vec<_>>();
            keys.sort_unstable();
            for (index, key) in keys.into_iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                output.push_str(&serde_json::to_string(key).context("encode JSON object key")?);
                output.push(':');
                write_canonical_json(
                    values
                        .get(key)
                        .context("canonical JSON object key disappeared")?,
                    output,
                )?;
            }
            output.push('}');
        }
    }
    Ok(())
}

fn require_live_mutation_anchor(
    operator_mutation_enabled: bool,
    telegram_pipeline_enabled: bool,
    anchor_configured: bool,
) -> Result<()> {
    if anchor_configured || (!operator_mutation_enabled && !telegram_pipeline_enabled) {
        return Ok(());
    }
    let required_by = if operator_mutation_enabled && telegram_pipeline_enabled {
        "operator mutation and Telegram pipeline"
    } else if operator_mutation_enabled {
        "operator mutation"
    } else {
        "Telegram pipeline"
    };
    anyhow::bail!("external monotonic anchor is required when {required_by} authority is enabled")
}

fn combine_effect_with_anchor<T>(
    effect: Result<T>,
    anchor: Result<()>,
    context: &str,
) -> Result<T> {
    match (effect, anchor) {
        (Ok(value), Ok(())) => Ok(value),
        (Ok(_), Err(error)) => Err(error.context(context.to_string())),
        (Err(error), Ok(())) => Err(error),
        (Err(error), Err(anchor_error)) => Err(anyhow::anyhow!(
            "{error:#}; external monotonic anchor also failed: {anchor_error:#}"
        )),
    }
}

fn read_integrity_key(path: &Path) -> Result<DurableIntegrityKey> {
    let bytes = read_private_key(path, INTEGRITY_KEY_FILE_ENV, "RuntimeKernel integrity")?;
    Ok(DurableIntegrityKey::from_bytes(*bytes))
}

#[cfg(all(test, unix))]
#[path = "../tests/unit/runtime_composition.rs"]
mod tests;
