use hepta_contracts::ContentHash;
use sha2::Digest;
use sha2::Sha256;

mod candidate_reference;
mod error;
pub use candidate_reference::candidate_reference_hash;
pub use error::ExecutionIntentError;

const RESOURCE_SUMMARY_DOMAIN: &str = "hepta.memory.execution-intent.resource-summary.v1";
const EFFECT_PLAN_DOMAIN: &str = "hepta.memory.execution-intent.effect-plan.v1";
const IDEMPOTENCY_KEY_DOMAIN: &str = "hepta.memory.execution-intent.idempotency-key.v3";

/// Exact pre-dispatch material from which one durable execution intent is built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionIntentParts {
    pub attempt_id: String,
    pub session_id: String,
    pub correlation_id: String,
    pub tool_name: String,
    pub payload_hash: ContentHash,
    pub candidate_hash: ContentHash,
    pub candidate_reference_hash: ContentHash,
    pub kernel_candidate_hash: ContentHash,
    pub payload_set_hash: ContentHash,
    pub capability_id: String,
    pub capability_revision: u64,
    pub capability_provider: String,
    pub capability_operation: String,
    pub capability_manifest_hash: ContentHash,
    pub executor_principal: String,
    pub authorization_digest: ContentHash,
    pub admission_id: String,
    pub admission_revision: u64,
    pub admission_digest: ContentHash,
    pub canonical_resource_summary: String,
    pub canonical_effect_plan: Option<String>,
}

/// Canonical, immutable plan persisted before a provider may be invoked.
///
/// The idempotency key and resource-summary digest are derived here rather
/// than accepted from a caller, so exact replay cannot silently substitute
/// either binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionIntent {
    attempt_id: String,
    session_id: String,
    correlation_id: String,
    tool_name: String,
    payload_hash: ContentHash,
    candidate_hash: ContentHash,
    candidate_reference_hash: ContentHash,
    kernel_candidate_hash: ContentHash,
    payload_set_hash: ContentHash,
    capability_id: String,
    capability_revision: u64,
    capability_provider: String,
    capability_operation: String,
    capability_manifest_hash: ContentHash,
    executor_principal: String,
    authorization_digest: ContentHash,
    admission_id: String,
    admission_revision: u64,
    admission_digest: ContentHash,
    canonical_resource_summary: String,
    resource_summary_hash: ContentHash,
    canonical_effect_plan: Option<String>,
    effect_plan_hash: Option<ContentHash>,
    idempotency_key: String,
}

impl ExecutionIntent {
    /// Validates exact plan material and derives its stable resource and replay bindings.
    pub fn try_new(parts: ExecutionIntentParts) -> Result<Self, ExecutionIntentError> {
        for (field, value) in [
            ("attempt_id", parts.attempt_id.as_str()),
            ("session_id", parts.session_id.as_str()),
            ("correlation_id", parts.correlation_id.as_str()),
            ("tool_name", parts.tool_name.as_str()),
            ("payload_hash", parts.payload_hash.as_str()),
            ("candidate_hash", parts.candidate_hash.as_str()),
            (
                "candidate_reference_hash",
                parts.candidate_reference_hash.as_str(),
            ),
            (
                "kernel_candidate_hash",
                parts.kernel_candidate_hash.as_str(),
            ),
            ("payload_set_hash", parts.payload_set_hash.as_str()),
            ("capability_id", parts.capability_id.as_str()),
            ("capability_provider", parts.capability_provider.as_str()),
            ("capability_operation", parts.capability_operation.as_str()),
            (
                "capability_manifest_hash",
                parts.capability_manifest_hash.as_str(),
            ),
            ("executor_principal", parts.executor_principal.as_str()),
            ("authorization_digest", parts.authorization_digest.as_str()),
            ("admission_id", parts.admission_id.as_str()),
            ("admission_digest", parts.admission_digest.as_str()),
            (
                "canonical_resource_summary",
                parts.canonical_resource_summary.as_str(),
            ),
        ] {
            if value.trim().is_empty() {
                return Err(ExecutionIntentError::EmptyField { field });
            }
        }
        if parts.payload_hash != parts.payload_set_hash {
            return Err(ExecutionIntentError::PayloadBindingMismatch);
        }
        if parts
            .canonical_effect_plan
            .as_deref()
            .is_some_and(|plan| plan.trim().is_empty())
        {
            return Err(ExecutionIntentError::EmptyEffectPlan);
        }

        let resource_summary_hash = framed_hash(
            RESOURCE_SUMMARY_DOMAIN,
            &[(
                "canonical_resource_summary",
                parts.canonical_resource_summary.as_str(),
            )],
        );
        let effect_plan_hash = parts
            .canonical_effect_plan
            .as_deref()
            .map(|plan| framed_hash(EFFECT_PLAN_DOMAIN, &[("canonical_effect_plan", plan)]));
        let idempotency_digest = framed_hash(
            IDEMPOTENCY_KEY_DOMAIN,
            &[
                ("attempt_id", parts.attempt_id.as_str()),
                ("session_id", parts.session_id.as_str()),
                ("correlation_id", parts.correlation_id.as_str()),
                ("tool_name", parts.tool_name.as_str()),
                ("payload_hash", parts.payload_hash.as_str()),
                ("candidate_hash", parts.candidate_hash.as_str()),
                (
                    "candidate_reference_hash",
                    parts.candidate_reference_hash.as_str(),
                ),
                (
                    "kernel_candidate_hash",
                    parts.kernel_candidate_hash.as_str(),
                ),
                ("payload_set_hash", parts.payload_set_hash.as_str()),
                ("capability_id", parts.capability_id.as_str()),
                (
                    "capability_revision",
                    &parts.capability_revision.to_string(),
                ),
                ("capability_provider", parts.capability_provider.as_str()),
                ("capability_operation", parts.capability_operation.as_str()),
                (
                    "capability_manifest_hash",
                    parts.capability_manifest_hash.as_str(),
                ),
                ("executor_principal", parts.executor_principal.as_str()),
                ("authorization_digest", parts.authorization_digest.as_str()),
                ("admission_id", parts.admission_id.as_str()),
                ("admission_revision", &parts.admission_revision.to_string()),
                ("admission_digest", parts.admission_digest.as_str()),
                ("resource_summary_hash", resource_summary_hash.as_str()),
                (
                    "effect_plan_hash",
                    effect_plan_hash
                        .as_ref()
                        .map(ContentHash::as_str)
                        .unwrap_or(""),
                ),
            ],
        );
        let idempotency_key = format!(
            "hepta-execution:{}:{}",
            parts.attempt_id,
            idempotency_digest.as_str()
        );

        Ok(Self {
            attempt_id: parts.attempt_id,
            session_id: parts.session_id,
            correlation_id: parts.correlation_id,
            tool_name: parts.tool_name,
            payload_hash: parts.payload_hash,
            candidate_hash: parts.candidate_hash,
            candidate_reference_hash: parts.candidate_reference_hash,
            kernel_candidate_hash: parts.kernel_candidate_hash,
            payload_set_hash: parts.payload_set_hash,
            capability_id: parts.capability_id,
            capability_revision: parts.capability_revision,
            capability_provider: parts.capability_provider,
            capability_operation: parts.capability_operation,
            capability_manifest_hash: parts.capability_manifest_hash,
            executor_principal: parts.executor_principal,
            authorization_digest: parts.authorization_digest,
            admission_id: parts.admission_id,
            admission_revision: parts.admission_revision,
            admission_digest: parts.admission_digest,
            canonical_resource_summary: parts.canonical_resource_summary,
            resource_summary_hash,
            canonical_effect_plan: parts.canonical_effect_plan,
            effect_plan_hash,
            idempotency_key,
        })
    }

    pub fn attempt_id(&self) -> &str {
        &self.attempt_id
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn correlation_id(&self) -> &str {
        &self.correlation_id
    }

    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    pub fn payload_hash(&self) -> &ContentHash {
        &self.payload_hash
    }

    pub fn candidate_hash(&self) -> &ContentHash {
        &self.candidate_hash
    }

    pub fn candidate_reference_hash(&self) -> &ContentHash {
        &self.candidate_reference_hash
    }

    pub fn kernel_candidate_hash(&self) -> &ContentHash {
        &self.kernel_candidate_hash
    }

    pub fn payload_set_hash(&self) -> &ContentHash {
        &self.payload_set_hash
    }

    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    pub const fn capability_revision(&self) -> u64 {
        self.capability_revision
    }

    pub fn capability_provider(&self) -> &str {
        &self.capability_provider
    }

    pub fn capability_operation(&self) -> &str {
        &self.capability_operation
    }

    pub fn capability_manifest_hash(&self) -> &ContentHash {
        &self.capability_manifest_hash
    }

    pub fn executor_principal(&self) -> &str {
        &self.executor_principal
    }

    pub fn authorization_digest(&self) -> &ContentHash {
        &self.authorization_digest
    }

    pub fn admission_id(&self) -> &str {
        &self.admission_id
    }

    pub const fn admission_revision(&self) -> u64 {
        self.admission_revision
    }

    pub fn admission_digest(&self) -> &ContentHash {
        &self.admission_digest
    }

    pub fn canonical_resource_summary(&self) -> &str {
        &self.canonical_resource_summary
    }

    pub fn resource_summary_hash(&self) -> &ContentHash {
        &self.resource_summary_hash
    }

    pub fn canonical_effect_plan(&self) -> Option<&str> {
        self.canonical_effect_plan.as_deref()
    }

    pub fn effect_plan_hash(&self) -> Option<&ContentHash> {
        self.effect_plan_hash.as_ref()
    }

    pub fn idempotency_key(&self) -> &str {
        &self.idempotency_key
    }
}

/// Result of durably staging one pre-dispatch intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionIntentStageResult {
    Staged,
    AlreadyStaged,
}

/// Result of resolving an intent after its exact terminal outcome is durable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionIntentResolveResult {
    Resolved,
    AlreadyResolved,
}

fn framed_hash(domain: &str, fields: &[(&str, &str)]) -> ContentHash {
    let mut hasher = Sha256::new();
    update_frame(&mut hasher, domain.as_bytes());
    for (name, value) in fields {
        update_frame(&mut hasher, name.as_bytes());
        update_frame(&mut hasher, value.as_bytes());
    }
    ContentHash::new(format!("sha256:{}", encode_hex(&hasher.finalize())))
}

fn update_frame(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}
