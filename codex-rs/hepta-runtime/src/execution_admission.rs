use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use thiserror::Error;

const ADMISSION_DOMAIN: &[u8] = b"hepta.execution-admission.v1";
const EFFECT_PLAN_DOMAIN: &[u8] = b"hepta.effect-plan.v1";
const PROVIDER_ACK_DOMAIN: &[u8] = b"hepta.provider-effect-ack.v1";
const TERMINAL_RECEIPT_DOMAIN: &[u8] = b"hepta.terminal-effect-receipt.v1";

#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[error("{0}")]
pub struct ExecutionAdmissionError(String);

impl ExecutionAdmissionError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionIngress {
    AppServer,
    McpServer,
    Cli,
    NativeGateway,
    Telegram,
    ModelProvider,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactExecutionAuthority {
    caller_binding: String,
    workspace_binding: String,
    session_binding: String,
}

impl ExactExecutionAuthority {
    pub fn new(
        caller_binding: impl Into<String>,
        workspace_binding: impl Into<String>,
        session_binding: impl Into<String>,
    ) -> Result<Self, ExecutionAdmissionError> {
        let authority = Self {
            caller_binding: caller_binding.into(),
            workspace_binding: workspace_binding.into(),
            session_binding: session_binding.into(),
        };
        require_sha256_hex(&authority.caller_binding, "caller binding")?;
        require_sha256_hex(&authority.workspace_binding, "workspace binding")?;
        require_sha256_hex(&authority.session_binding, "session binding")?;
        Ok(authority)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionAdmission {
    ingress: ExecutionIngress,
    operation: String,
    authority: ExactExecutionAuthority,
    intent_binding: String,
    candidate_binding: String,
    admission_hash: String,
}

impl ExecutionAdmission {
    pub fn new(
        ingress: ExecutionIngress,
        operation: impl Into<String>,
        authority: ExactExecutionAuthority,
        intent_binding: impl Into<String>,
        candidate_binding: impl Into<String>,
    ) -> Result<Self, ExecutionAdmissionError> {
        let operation = operation.into();
        require_label(&operation, "operation")?;
        let intent_binding = intent_binding.into();
        let candidate_binding = candidate_binding.into();
        require_sha256_hex(&intent_binding, "intent binding")?;
        require_content_hash(&candidate_binding, "candidate binding")?;
        let mut admission = Self {
            ingress,
            operation,
            authority,
            intent_binding,
            candidate_binding,
            admission_hash: String::new(),
        };
        admission.admission_hash = canonical_hash(ADMISSION_DOMAIN, &admission)?;
        Ok(admission)
    }

    pub fn admission_hash(&self) -> &str {
        &self.admission_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectPlan {
    admission_hash: String,
    effect_kind: String,
    target_binding: String,
    payload_digest: String,
    idempotency_binding: String,
    effect_plan_hash: String,
}

impl EffectPlan {
    pub fn new(
        admission_hash: impl Into<String>,
        effect_kind: impl Into<String>,
        target_binding: impl Into<String>,
        payload_digest: impl Into<String>,
        idempotency_binding: impl Into<String>,
    ) -> Result<Self, ExecutionAdmissionError> {
        let admission_hash = admission_hash.into();
        let effect_kind = effect_kind.into();
        let target_binding = target_binding.into();
        let payload_digest = payload_digest.into();
        let idempotency_binding = idempotency_binding.into();
        require_content_hash(&admission_hash, "admission hash")?;
        require_label(&effect_kind, "effect kind")?;
        require_label(&target_binding, "target binding")?;
        require_content_hash(&payload_digest, "payload digest")?;
        require_sha256_hex(&idempotency_binding, "idempotency binding")?;
        let mut plan = Self {
            admission_hash,
            effect_kind,
            target_binding,
            payload_digest,
            idempotency_binding,
            effect_plan_hash: String::new(),
        };
        plan.effect_plan_hash = canonical_hash(EFFECT_PLAN_DOMAIN, &plan)?;
        Ok(plan)
    }

    pub fn effect_plan_hash(&self) -> &str {
        &self.effect_plan_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderEffectAck {
    effect_plan_hash: String,
    provider_binding: String,
    provider_receipt_hash: String,
    ack_hash: String,
}

impl ProviderEffectAck {
    pub fn new(
        effect_plan_hash: impl Into<String>,
        provider_binding: impl Into<String>,
        provider_receipt_hash: impl Into<String>,
    ) -> Result<Self, ExecutionAdmissionError> {
        let effect_plan_hash = effect_plan_hash.into();
        let provider_binding = provider_binding.into();
        let provider_receipt_hash = provider_receipt_hash.into();
        require_content_hash(&effect_plan_hash, "effect plan hash")?;
        require_label(&provider_binding, "provider binding")?;
        require_content_hash(&provider_receipt_hash, "provider receipt hash")?;
        let mut ack = Self {
            effect_plan_hash,
            provider_binding,
            provider_receipt_hash,
            ack_hash: String::new(),
        };
        ack.ack_hash = canonical_hash(PROVIDER_ACK_DOMAIN, &ack)?;
        Ok(ack)
    }

    pub fn ack_hash(&self) -> &str {
        &self.ack_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalEffectReceipt {
    ack_hash: String,
    terminal_status: String,
    runtime_receipt_hash: String,
    terminal_evidence_hash: String,
    receipt_hash: String,
}

impl TerminalEffectReceipt {
    pub fn terminal(
        ack_hash: impl Into<String>,
        terminal_status: impl Into<String>,
        runtime_receipt_hash: impl Into<String>,
        terminal_evidence_hash: impl Into<String>,
    ) -> Result<Self, ExecutionAdmissionError> {
        let ack_hash = ack_hash.into();
        let terminal_status = terminal_status.into();
        let runtime_receipt_hash = runtime_receipt_hash.into();
        let terminal_evidence_hash = terminal_evidence_hash.into();
        require_content_hash(&ack_hash, "provider ACK hash")?;
        require_label(&terminal_status, "terminal status")?;
        require_content_hash(&runtime_receipt_hash, "runtime receipt hash")?;
        require_content_hash(&terminal_evidence_hash, "terminal evidence hash")?;
        let mut receipt = Self {
            ack_hash,
            terminal_status,
            runtime_receipt_hash,
            terminal_evidence_hash,
            receipt_hash: String::new(),
        };
        receipt.receipt_hash = canonical_hash(TERMINAL_RECEIPT_DOMAIN, &receipt)?;
        Ok(receipt)
    }

    pub fn receipt_hash(&self) -> &str {
        &self.receipt_hash
    }
}

#[derive(Debug)]
pub struct EffectBroker {
    admission: ExecutionAdmission,
    effect_plan: Option<EffectPlan>,
    provider_ack: Option<ProviderEffectAck>,
    terminal_receipt: Option<TerminalEffectReceipt>,
}

impl EffectBroker {
    pub fn admit(admission: ExecutionAdmission) -> Self {
        Self {
            admission,
            effect_plan: None,
            provider_ack: None,
            terminal_receipt: None,
        }
    }

    pub fn record_effect_plan(
        &mut self,
        effect_plan: EffectPlan,
    ) -> Result<(), ExecutionAdmissionError> {
        if self.effect_plan.is_some() {
            return Err(ExecutionAdmissionError::new("effect plan already recorded"));
        }
        if effect_plan.admission_hash != self.admission.admission_hash {
            return Err(ExecutionAdmissionError::new(
                "effect plan does not bind the admitted execution",
            ));
        }
        self.effect_plan = Some(effect_plan);
        Ok(())
    }

    pub fn record_provider_ack(
        &mut self,
        provider_ack: ProviderEffectAck,
    ) -> Result<(), ExecutionAdmissionError> {
        if self.provider_ack.is_some() {
            return Err(ExecutionAdmissionError::new(
                "provider ACK already recorded",
            ));
        }
        let effect_plan = self
            .effect_plan
            .as_ref()
            .ok_or_else(|| ExecutionAdmissionError::new("effect plan must precede provider ACK"))?;
        if provider_ack.effect_plan_hash != effect_plan.effect_plan_hash {
            return Err(ExecutionAdmissionError::new(
                "provider ACK does not bind the recorded effect plan",
            ));
        }
        self.provider_ack = Some(provider_ack);
        Ok(())
    }

    pub fn record_terminal_receipt(
        &mut self,
        terminal_receipt: TerminalEffectReceipt,
    ) -> Result<(), ExecutionAdmissionError> {
        if self.terminal_receipt.is_some() {
            return Err(ExecutionAdmissionError::new(
                "terminal receipt already recorded",
            ));
        }
        let provider_ack = self.provider_ack.as_ref().ok_or_else(|| {
            ExecutionAdmissionError::new("provider ACK must precede terminal receipt")
        })?;
        if terminal_receipt.ack_hash != provider_ack.ack_hash {
            return Err(ExecutionAdmissionError::new(
                "terminal receipt does not bind the recorded provider ACK",
            ));
        }
        self.terminal_receipt = Some(terminal_receipt);
        Ok(())
    }

    pub fn completed_receipt_hash(&self) -> Result<&str, ExecutionAdmissionError> {
        self.terminal_receipt
            .as_ref()
            .map(TerminalEffectReceipt::receipt_hash)
            .ok_or_else(|| ExecutionAdmissionError::new("execution lifecycle is not terminal"))
    }

    pub fn completed_provider_ack_hash(&self) -> Result<&str, ExecutionAdmissionError> {
        self.provider_ack
            .as_ref()
            .map(ProviderEffectAck::ack_hash)
            .ok_or_else(|| ExecutionAdmissionError::new("provider ACK is not recorded"))
    }
}

fn canonical_hash<T: Serialize>(
    domain: &[u8],
    value: &T,
) -> Result<String, ExecutionAdmissionError> {
    let encoded = serde_json::to_vec(value).map_err(|error| {
        ExecutionAdmissionError::new(format!("encode lifecycle value: {error}"))
    })?;
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update([0]);
    hasher.update((encoded.len() as u64).to_be_bytes());
    hasher.update(encoded);
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

fn require_label(value: &str, name: &str) -> Result<(), ExecutionAdmissionError> {
    if value.is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        return Err(ExecutionAdmissionError::new(format!(
            "{name} must be a bounded printable value"
        )));
    }
    Ok(())
}

fn require_sha256_hex(value: &str, name: &str) -> Result<(), ExecutionAdmissionError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(ExecutionAdmissionError::new(format!(
            "{name} must be lowercase SHA-256 hex"
        )));
    }
    Ok(())
}

fn require_content_hash(value: &str, name: &str) -> Result<(), ExecutionAdmissionError> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(ExecutionAdmissionError::new(format!(
            "{name} must use the sha256 content-hash domain"
        )));
    };
    require_sha256_hex(hex, name)
}
