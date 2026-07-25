use std::env;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
#[cfg(test)]
use hepta_contracts::RevisionStamp;
use hepta_memory::DurableIntegrityKey;
use hepta_runtime::RuntimeExecutionReceipt;
use hepta_runtime::RuntimeKernel;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::preference_ingress::NativePreferenceIngress;
use crate::preference_ingress::NativePreferenceIngressConfig;
use crate::preference_ingress::PreferenceHttpResponse;
use crate::runtime_ingress::RuntimeIngressKind;
use crate::runtime_ingress::runtime_ingress_kind;
use crate::runtime_mutation::RuntimeMutationCanaryReceipt;
#[cfg(all(test, unix))]
use crate::secure_key_file::PRIVATE_FILE_MODE;
use crate::secure_key_file::read_private_key;

const OUTCOME_DATABASE_ENV: &str = "HEPTA_RUNTIME_OUTCOME_DATABASE";
const INTEGRITY_KEY_FILE_ENV: &str = "HEPTA_RUNTIME_INTEGRITY_KEY_FILE";
const OUTCOME_MODE_ENV: &str = "HEPTA_RUNTIME_OUTCOME_MODE";
const OPEN_EXISTING_MODE: &str = "open-existing";
const BOOTSTRAP_NEW_MODE: &str = "bootstrap-new";
pub(crate) const RUNTIME_KERNEL_CANARY_ACTION_ENDPOINT: &str = "/api/actions/runtime-kernel-canary";
pub struct NativeGatewayRuntime {
    kernel: RuntimeKernel,
    preference_ingress: NativePreferenceIngress,
    outcome_mode: RuntimeOutcomeMode,
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
    integrity_key_file: PathBuf,
    outcome_mode: RuntimeOutcomeMode,
}

impl RuntimeCompositionConfig {
    fn from_env() -> Result<Self> {
        let outcome_database = required_absolute_path(OUTCOME_DATABASE_ENV)?;
        let integrity_key_file = required_absolute_path(INTEGRITY_KEY_FILE_ENV)?;
        let outcome_mode = env::var(OUTCOME_MODE_ENV)
            .ok()
            .map(|value| RuntimeOutcomeMode::parse(value.trim()))
            .transpose()?
            .unwrap_or(RuntimeOutcomeMode::OpenExisting);
        Ok(Self {
            outcome_database,
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
    /// proofs. The native gateway defaults to loopback, but its clear HTTP
    /// transport makes no confidentiality claim; the existing explicit
    /// non-loopback lab override is not a secure deployment mode.
    pub fn from_env() -> Result<Self> {
        Self::open(
            RuntimeCompositionConfig::from_env()?,
            NativePreferenceIngressConfig::from_env()?,
        )
    }

    fn open(
        config: RuntimeCompositionConfig,
        preference_config: NativePreferenceIngressConfig,
    ) -> Result<Self> {
        let integrity_key = read_integrity_key(&config.integrity_key_file)?;
        let prepared_preference = NativePreferenceIngress::prepare(preference_config)?;
        let kernel = match config.outcome_mode {
            RuntimeOutcomeMode::OpenExisting => {
                RuntimeKernel::open_with_durable_outcomes(&config.outcome_database, integrity_key)
            }
            RuntimeOutcomeMode::BootstrapNew => RuntimeKernel::bootstrap_with_durable_outcomes(
                &config.outcome_database,
                integrity_key,
            ),
        }
        .with_context(|| {
            format!(
                "initialize keyed RuntimeKernel with {} durable outcomes",
                config.outcome_mode.as_str()
            )
        })?;
        let preference_ingress = NativePreferenceIngress::open(prepared_preference)?;
        Ok(Self {
            kernel,
            preference_ingress,
            outcome_mode: config.outcome_mode,
        })
    }

    pub(crate) fn validate_readiness(&self) -> Result<()> {
        self.kernel
            .model_selection()
            .map_err(|error| anyhow::anyhow!("attached RuntimeKernel readiness failed: {error}"))?;
        self.preference_ingress
            .validate_readiness()
            .context("attached trusted preference ingress readiness failed")
    }

    pub(crate) fn preflight_request(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<RuntimeRequestPreflightReceipt> {
        let disposition = match method {
            "GET" => RuntimeRequestDisposition::ReadOnlyDispatch,
            "POST" => RuntimeRequestDisposition::PlanOnlyQuarantine,
            _ => anyhow::bail!("attached RuntimeKernel denied unsupported HTTP method"),
        };
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
        for value in [
            b"hepta-native-gateway-runtime-request-v1".as_slice(),
            method.as_bytes(),
            path.as_bytes(),
            body.unwrap_or_default().as_bytes(),
            session_id.as_bytes(),
            model.provider.as_bytes(),
            model.model.as_bytes(),
        ] {
            hasher.update((value.len() as u64).to_be_bytes());
            hasher.update(value);
        }
        Ok(RuntimeRequestPreflightReceipt {
            request_binding_hash: format!("{:x}", hasher.finalize()),
            disposition,
            ingress_kind: runtime_ingress_kind(method, path),
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
        let mut response = self.preference_ingress.route_http(
            method,
            path,
            body,
            request_binding_hash,
            &session_binding_hash,
        )?;
        if let Some(stamp) = response.preference_context.take()
            && self
                .kernel
                .attach_authenticated_preference_context(&session_id, stamp)
                .is_err()
        {
            return Some(PreferenceHttpResponse {
                status: "503 Service Unavailable",
                body: r#"{"error":"trusted_preference_ingress.runtime_attachment_failed"}"#
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
    }

    pub(crate) fn execute_runtime_mutation_canary(
        &self,
        request_binding_hash: &str,
        idempotency_key: &str,
    ) -> Result<RuntimeMutationCanaryReceipt> {
        crate::runtime_mutation::execute(&self.kernel, request_binding_hash, idempotency_key)
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
                integrity_key_file: key,
                outcome_mode: RuntimeOutcomeMode::BootstrapNew,
            },
            NativePreferenceIngressConfig::bootstrap_for_test(root)?,
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

fn read_integrity_key(path: &Path) -> Result<DurableIntegrityKey> {
    let bytes = read_private_key(path, INTEGRITY_KEY_FILE_ENV, "RuntimeKernel integrity")?;
    Ok(DurableIntegrityKey::from_bytes(*bytes))
}

#[cfg(all(test, unix))]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use tempfile::tempdir;

    use super::*;

    fn write_key(path: &Path, encoded: &[u8]) {
        fs::write(path, encoded).expect("write key");
        fs::set_permissions(path, fs::Permissions::from_mode(PRIVATE_FILE_MODE))
            .expect("set key permissions");
    }

    #[test]
    fn keyed_runtime_bootstraps_then_opens_existing_database() {
        let root = tempdir().expect("tempdir");
        fs::set_permissions(root.path(), fs::Permissions::from_mode(0o700))
            .expect("set root permissions");
        let key_path = root.path().join("runtime.key");
        let database_path = root.path().join("outcomes.sqlite3");
        write_key(
            &key_path,
            b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\n",
        );

        let preference_root = root.path().join("preference");
        fs::create_dir(&preference_root).expect("preference root");
        fs::set_permissions(&preference_root, fs::Permissions::from_mode(0o700))
            .expect("set preference root permissions");
        let bootstrap = NativeGatewayRuntime::open(
            RuntimeCompositionConfig {
                outcome_database: database_path.clone(),
                integrity_key_file: key_path.clone(),
                outcome_mode: RuntimeOutcomeMode::BootstrapNew,
            },
            NativePreferenceIngressConfig::bootstrap_for_test(&preference_root)
                .expect("preference config"),
        )
        .expect("bootstrap keyed runtime");
        bootstrap.validate_readiness().expect("bootstrap readiness");
        let read = bootstrap
            .preflight_request("GET", "/api/health", None)
            .expect("read-only preflight");
        assert_eq!(
            read.disposition,
            RuntimeRequestDisposition::ReadOnlyDispatch
        );
        assert!(!read.mutation_authorized);
        assert!(!read.durable_intent_recorded);
        assert!(!read.provider_effect_ack_recorded);
        assert!(!read.terminal_receipt_recorded);
        let plan = bootstrap
            .preflight_request("POST", "/api/tasks/publish", Some(r#"{"dry_run":true}"#))
            .expect("plan-only preflight");
        assert_eq!(
            plan.disposition,
            RuntimeRequestDisposition::PlanOnlyQuarantine
        );
        assert_ne!(read.request_binding_hash, plan.request_binding_hash);
        assert!(!plan.mutation_authorized);
        let configured = plan.native_post_gate_inputs(true, true);
        assert!(!configured.real_handler_enabled);
        assert!(!configured.operator_approval_enabled);
        assert!(
            bootstrap
                .preflight_request("DELETE", "/api/tasks/1", None)
                .expect_err("mutation method must fail closed")
                .to_string()
                .contains("unsupported HTTP method")
        );
        let telegram = bootstrap
            .preflight_telegram_drain(Some(42))
            .expect("telegram preflight");
        assert!(!telegram.request_binding_hash.is_empty());
        assert_eq!(
            telegram
                .require_live_pipeline_authority()
                .expect_err("telegram live pipeline must remain quarantined")
                .to_string(),
            "telegram_runtime_admission.exact_authority_unavailable"
        );
        drop(bootstrap);

        let opened = NativeGatewayRuntime::open(
            RuntimeCompositionConfig {
                outcome_database: database_path,
                integrity_key_file: key_path,
                outcome_mode: RuntimeOutcomeMode::OpenExisting,
            },
            NativePreferenceIngressConfig {
                database: preference_root.join("preferences.sqlite3"),
                integrity_key_file: preference_root.join("preference-integrity.key"),
                authentication_key_file: preference_root.join("preference-authentication.key"),
                mode: crate::preference_ingress::PreferenceStoreMode::OpenExisting,
            },
        )
        .expect("open keyed runtime");
        assert_eq!(opened.outcome_mode(), OPEN_EXISTING_MODE);
        opened.validate_readiness().expect("open readiness");
    }

    #[test]
    fn keyed_runtime_rejects_non_private_key_file() {
        let root = tempdir().expect("tempdir");
        let key_path = root.path().join("runtime.key");
        write_key(
            &key_path,
            b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        );
        fs::set_permissions(&key_path, fs::Permissions::from_mode(0o644))
            .expect("relax key permissions");

        let error = read_integrity_key(&key_path).expect_err("unsafe key must fail");
        assert!(
            error
                .to_string()
                .contains("integrity key must have mode 0o600")
        );
    }

    #[test]
    fn keyed_runtime_rejects_noncanonical_key_encoding() {
        let root = tempdir().expect("tempdir");
        let key_path = root.path().join("runtime.key");
        write_key(
            &key_path,
            b"000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F",
        );
        let error = read_integrity_key(&key_path).expect_err("uppercase key must fail");
        assert!(error.to_string().contains("canonical lowercase hex"));
    }
}
