//! Explicit, feature-gated Agentd seam for the P0.4c shadow mutation host.
//!
//! The handle delegates only to the qualification journal adapter. Agentd
//! startup, App Server, tool registration, production writer, projection
//! pointer, recall path, physical-send path, and outbox dispatcher never attach
//! this handle automatically.

use std::fmt;
use std::sync::Arc;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_memory::CognitiveStore;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::AgentdConfig;
use crate::AgentdError;

pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION: u32 = 1;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_NAMESPACE: &str =
    "agentd_shadow_intelligence_mutation_host_v1";
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_RUNTIME_WIRED: bool = false;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_APP_RUNTIME_ATTACHED: bool = false;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_TOOL_REGISTERED: bool = false;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_MEMORY_WRITE_AUTHORITY: bool = false;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_PROJECTION_WRITE_AUTHORITY: bool = false;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_OUTBOX_DISPATCH_AUTHORITY: bool = false;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_EXTERNAL_EFFECTS: bool = false;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_PRODUCTION_AUTHORITY: bool = false;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_OPERATOR_ACCEPTANCE: bool = false;
pub const AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_PROMOTION: bool = false;

const MAX_ENVELOPE_BYTES: usize = 128 * 1024;
const MAX_ID_BYTES: usize = 256;

#[derive(Clone)]
pub struct AgentdShadowIntelligenceMutationHost {
    store: Arc<CognitiveStore>,
    agent_id: AgentId,
    spawn_generation: u64,
}

impl fmt::Debug for AgentdShadowIntelligenceMutationHost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentdShadowIntelligenceMutationHost")
            .field("agent_id", &self.agent_id)
            .field("spawn_generation", &self.spawn_generation)
            .field("store_path", &self.store.path())
            .field("runtime_wired", &false)
            .field("production_authority", &false)
            .finish()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct AgentdShadowBindingDraft {
    operation_id: String,
    lease_id: String,
    lease_epoch: u64,
    expected_revision: Option<u64>,
    starting_projection_generation: u64,
    causal_root_sha256: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AgentdShadowHostEnvelope {
    schema_version: u32,
    namespace: String,
    action: String,
    agent_id: String,
    spawn_generation: u64,
    payload_sha256: String,
    host_receipt_sha256: String,
    payload: Value,
    runtime_wired: bool,
    app_runtime_attached: bool,
    tool_registered: bool,
    memory_write_performed_by_agentd: bool,
    projection_write_performed_by_agentd: bool,
    outbox_dispatch_performed_by_agentd: bool,
    external_effects: bool,
    production_authority: bool,
    operator_acceptance: bool,
    promotion: bool,
}

impl AgentdShadowIntelligenceMutationHost {
    /// Opens only the opt-in P0.4c/P0.4b qualification store. Merely creating
    /// this handle does not mutate Agentd runtime configuration.
    pub async fn open(config: &AgentdConfig) -> Result<Self, AgentdError> {
        let identity = config.identity();
        let store = CognitiveStore::open_with_shadow_intelligence_mutation_host(&identity.layout)
            .await
            .map_err(|error| {
                AgentdError::Protocol(format!(
                    "open shadow intelligence mutation host store: {error}"
                ))
            })?;
        if store.owner_agent_id() != &identity.agent_id {
            return Err(AgentdError::GenerationFenced(
                "shadow intelligence mutation host owner does not match Agentd identity"
                    .to_string(),
            ));
        }
        if identity.spawn_generation == 0 {
            return Err(AgentdError::GenerationFenced(
                "shadow intelligence mutation host requires a positive spawn generation"
                    .to_string(),
            ));
        }
        Ok(Self {
            store: Arc::new(store),
            agent_id: identity.agent_id.clone(),
            spawn_generation: identity.spawn_generation,
        })
    }

    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    pub fn spawn_generation(&self) -> u64 {
        self.spawn_generation
    }

    /// Begins an Agentd/spawn-bound shadow operation. The caller-supplied
    /// operation ID and causal root are deterministically attenuated into the
    /// current Agentd identity and spawn generation before entering the core
    /// journal.
    pub async fn begin(&self, binding_json: &str) -> Result<String, AgentdError> {
        let draft: AgentdShadowBindingDraft = parse_bounded_json(binding_json, "binding")?;
        let core_binding = bind_for_host(&self.agent_id, self.spawn_generation, draft)?;
        let payload = self
            .store
            .begin_shadow_intelligence_mutation(&core_binding)
            .await
            .map_err(shadow_error)?;
        self.wrap("begin", payload)
    }

    /// Returns a spawn-bound Agentd envelope whose `payload` is the exact core
    /// prepared request. The whole envelope, not just the nested payload, must
    /// be supplied to `append`.
    pub async fn prepare(
        &self,
        operation_id: &str,
        observation_json: &str,
    ) -> Result<String, AgentdError> {
        let effective_operation_id = effective_operation_id(
            &self.agent_id,
            self.spawn_generation,
            operation_id,
        )?;
        let payload = self
            .store
            .prepare_shadow_intelligence_mutation_observation(
                &effective_operation_id,
                observation_json,
            )
            .await
            .map_err(shadow_error)?;
        self.wrap("prepare", payload)
    }

    /// Validates and unwraps the exact Agentd `prepare` envelope before
    /// appending the nested core request. Cross-agent, cross-spawn, changed
    /// payload, changed authority flag, and changed receipt digest all fail
    /// closed before the journal is touched.
    pub async fn append(&self, prepared_envelope_json: &str) -> Result<String, AgentdError> {
        let prepared_json = unwrap_prepared_payload(
            &self.agent_id,
            self.spawn_generation,
            prepared_envelope_json,
        )?;
        let payload = self
            .store
            .append_shadow_intelligence_mutation_observation(&prepared_json)
            .await
            .map_err(shadow_error)?;
        self.wrap("append", payload)
    }

    pub async fn observe(
        &self,
        operation_id: &str,
        observation_json: &str,
    ) -> Result<String, AgentdError> {
        let effective_operation_id = effective_operation_id(
            &self.agent_id,
            self.spawn_generation,
            operation_id,
        )?;
        let payload = self
            .store
            .observe_shadow_intelligence_mutation(
                &effective_operation_id,
                observation_json,
            )
            .await
            .map_err(shadow_error)?;
        self.wrap("observe", payload)
    }

    pub async fn inspect(&self, operation_id: &str) -> Result<String, AgentdError> {
        let effective_operation_id = effective_operation_id(
            &self.agent_id,
            self.spawn_generation,
            operation_id,
        )?;
        let payload = self
            .store
            .inspect_shadow_intelligence_mutation(&effective_operation_id)
            .await
            .map_err(shadow_error)?;
        self.wrap("inspect", payload)
    }

    fn wrap(&self, action: &str, payload_json: String) -> Result<String, AgentdError> {
        wrap_payload(
            &self.agent_id,
            self.spawn_generation,
            action,
            &payload_json,
        )
    }
}

fn bind_for_host(
    agent_id: &AgentId,
    spawn_generation: u64,
    draft: AgentdShadowBindingDraft,
) -> Result<String, AgentdError> {
    require_positive_spawn(spawn_generation)?;
    validate_id(&draft.operation_id, "operation id")?;
    validate_id(&draft.lease_id, "lease id")?;
    let caller_root = Sha256Digest::parse(draft.causal_root_sha256)
        .map_err(|error| AgentdError::Invalid(format!("invalid causal root digest: {error}")))?;
    let effective_operation_id = effective_operation_id(
        agent_id,
        spawn_generation,
        &draft.operation_id,
    )?;
    let effective_lease_id = effective_lease_id(
        agent_id,
        spawn_generation,
        &draft.lease_id,
    )?;
    let causal_root_sha256 = host_bound_causal_root(
        agent_id,
        spawn_generation,
        &caller_root,
    );
    Ok(serde_json::json!({
        "operation_id": effective_operation_id,
        "lease_id": effective_lease_id,
        "lease_epoch": draft.lease_epoch,
        "expected_revision": draft.expected_revision,
        "starting_projection_generation": draft.starting_projection_generation,
        "causal_root_sha256": causal_root_sha256.as_str()
    })
    .to_string())
}

fn effective_operation_id(
    agent_id: &AgentId,
    spawn_generation: u64,
    caller_operation_id: &str,
) -> Result<String, AgentdError> {
    validate_id(caller_operation_id, "operation id")?;
    let digest = host_identity_digest(
        b"hepta-agentd:shadow-intelligence-operation:v1",
        agent_id,
        spawn_generation,
        caller_operation_id.as_bytes(),
    )?;
    Ok(format!("agentd-shadow-operation:{}", digest.as_str()))
}

fn effective_lease_id(
    agent_id: &AgentId,
    spawn_generation: u64,
    caller_lease_id: &str,
) -> Result<String, AgentdError> {
    validate_id(caller_lease_id, "lease id")?;
    let digest = host_identity_digest(
        b"hepta-agentd:shadow-intelligence-lease:v1",
        agent_id,
        spawn_generation,
        caller_lease_id.as_bytes(),
    )?;
    Ok(format!("agentd-shadow-lease:{}", digest.as_str()))
}

fn host_bound_causal_root(
    agent_id: &AgentId,
    spawn_generation: u64,
    caller_root: &Sha256Digest,
) -> Sha256Digest {
    let mut framed = Vec::new();
    push_part(
        &mut framed,
        b"hepta-agentd:shadow-intelligence-causal-root:v1",
    );
    push_part(&mut framed, agent_id.as_str().as_bytes());
    push_part(&mut framed, &spawn_generation.to_be_bytes());
    push_part(&mut framed, caller_root.as_str().as_bytes());
    Sha256Digest::for_bytes(&framed)
}

fn host_identity_digest(
    domain: &[u8],
    agent_id: &AgentId,
    spawn_generation: u64,
    caller_value: &[u8],
) -> Result<Sha256Digest, AgentdError> {
    require_positive_spawn(spawn_generation)?;
    let mut framed = Vec::new();
    push_part(&mut framed, domain);
    push_part(&mut framed, agent_id.as_str().as_bytes());
    push_part(&mut framed, &spawn_generation.to_be_bytes());
    push_part(&mut framed, caller_value);
    Ok(Sha256Digest::for_bytes(&framed))
}

fn wrap_payload(
    agent_id: &AgentId,
    spawn_generation: u64,
    action: &str,
    payload_json: &str,
) -> Result<String, AgentdError> {
    require_positive_spawn(spawn_generation)?;
    validate_id(action, "host action")?;
    let payload: Value = serde_json::from_str(payload_json)?;
    let canonical_payload = serde_json::to_string(&payload)?;
    let payload_sha256 = Sha256Digest::for_bytes(canonical_payload.as_bytes());
    let host_receipt_sha256 = host_receipt_digest(
        agent_id,
        spawn_generation,
        action,
        &payload_sha256,
    );
    Ok(serde_json::to_string(&AgentdShadowHostEnvelope {
        schema_version: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION,
        namespace: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_NAMESPACE.to_string(),
        action: action.to_string(),
        agent_id: agent_id.as_str().to_string(),
        spawn_generation,
        payload_sha256: payload_sha256.as_str().to_string(),
        host_receipt_sha256: host_receipt_sha256.as_str().to_string(),
        payload,
        runtime_wired: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_RUNTIME_WIRED,
        app_runtime_attached: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_APP_RUNTIME_ATTACHED,
        tool_registered: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_TOOL_REGISTERED,
        memory_write_performed_by_agentd: false,
        projection_write_performed_by_agentd: false,
        outbox_dispatch_performed_by_agentd: false,
        external_effects: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_EXTERNAL_EFFECTS,
        production_authority: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_PRODUCTION_AUTHORITY,
        operator_acceptance: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_OPERATOR_ACCEPTANCE,
        promotion: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_PROMOTION,
    })?)
}

fn unwrap_prepared_payload(
    agent_id: &AgentId,
    spawn_generation: u64,
    envelope_json: &str,
) -> Result<String, AgentdError> {
    require_positive_spawn(spawn_generation)?;
    if envelope_json.is_empty()
        || envelope_json.len() > MAX_ENVELOPE_BYTES
        || envelope_json.as_bytes().contains(&0)
    {
        return Err(AgentdError::Invalid(format!(
            "prepared host envelope must contain 1..={MAX_ENVELOPE_BYTES} non-NUL bytes"
        )));
    }
    let envelope: AgentdShadowHostEnvelope = serde_json::from_str(envelope_json)?;
    if envelope.schema_version != AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION
        || envelope.namespace != AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_NAMESPACE
        || envelope.action != "prepare"
    {
        return Err(AgentdError::Invalid(
            "unsupported prepared host envelope contract".to_string(),
        ));
    }
    if envelope.agent_id != agent_id.as_str()
        || envelope.spawn_generation != spawn_generation
    {
        return Err(AgentdError::GenerationFenced(
            "prepared host envelope belongs to another Agentd identity or spawn"
                .to_string(),
        ));
    }
    if envelope.runtime_wired
        || envelope.app_runtime_attached
        || envelope.tool_registered
        || envelope.memory_write_performed_by_agentd
        || envelope.projection_write_performed_by_agentd
        || envelope.outbox_dispatch_performed_by_agentd
        || envelope.external_effects
        || envelope.production_authority
        || envelope.operator_acceptance
        || envelope.promotion
    {
        return Err(AgentdError::Invalid(
            "prepared host envelope crosses the shadow authority boundary"
                .to_string(),
        ));
    }
    let payload_json = serde_json::to_string(&envelope.payload)?;
    let payload_sha256 = Sha256Digest::for_bytes(payload_json.as_bytes());
    if payload_sha256.as_str() != envelope.payload_sha256 {
        return Err(AgentdError::Invalid(
            "prepared host envelope payload digest mismatch".to_string(),
        ));
    }
    let recorded_payload_digest = Sha256Digest::parse(envelope.payload_sha256)
        .map_err(|error| AgentdError::Invalid(format!("invalid payload digest: {error}")))?;
    let recorded_host_receipt = Sha256Digest::parse(envelope.host_receipt_sha256)
        .map_err(|error| AgentdError::Invalid(format!("invalid host receipt digest: {error}")))?;
    let expected_host_receipt = host_receipt_digest(
        agent_id,
        spawn_generation,
        "prepare",
        &recorded_payload_digest,
    );
    if recorded_host_receipt != expected_host_receipt {
        return Err(AgentdError::Invalid(
            "prepared host envelope receipt digest mismatch".to_string(),
        ));
    }
    Ok(payload_json)
}

fn host_receipt_digest(
    agent_id: &AgentId,
    spawn_generation: u64,
    action: &str,
    payload_sha256: &Sha256Digest,
) -> Sha256Digest {
    let mut framed = Vec::new();
    push_part(
        &mut framed,
        b"hepta-agentd:shadow-intelligence-mutation-host:v1",
    );
    push_part(&mut framed, agent_id.as_str().as_bytes());
    push_part(&mut framed, &spawn_generation.to_be_bytes());
    push_part(&mut framed, action.as_bytes());
    push_part(&mut framed, payload_sha256.as_str().as_bytes());
    Sha256Digest::for_bytes(&framed)
}

fn push_part(output: &mut Vec<u8>, part: &[u8]) {
    output.extend_from_slice(&(part.len() as u64).to_be_bytes());
    output.extend_from_slice(part);
}

fn parse_bounded_json<T>(value: &str, label: &str) -> Result<T, AgentdError>
where
    T: for<'de> Deserialize<'de>,
{
    if value.is_empty() || value.len() > MAX_ENVELOPE_BYTES || value.as_bytes().contains(&0) {
        return Err(AgentdError::Invalid(format!(
            "shadow {label} must contain 1..={MAX_ENVELOPE_BYTES} non-NUL bytes"
        )));
    }
    serde_json::from_str(value)
        .map_err(|error| AgentdError::Invalid(format!("invalid shadow {label} JSON: {error}")))
}

fn validate_id(value: &str, label: &str) -> Result<(), AgentdError> {
    if value.trim().is_empty() || value.len() > MAX_ID_BYTES || value.as_bytes().contains(&0) {
        return Err(AgentdError::Invalid(format!(
            "shadow {label} must contain 1..={MAX_ID_BYTES} non-NUL bytes"
        )));
    }
    Ok(())
}

fn require_positive_spawn(spawn_generation: u64) -> Result<(), AgentdError> {
    if spawn_generation == 0 {
        return Err(AgentdError::GenerationFenced(
            "shadow intelligence mutation host requires a positive spawn generation"
                .to_string(),
        ));
    }
    Ok(())
}

fn shadow_error(error: codex_hepta_memory::CognitiveStoreError) -> AgentdError {
    AgentdError::Protocol(format!(
        "shadow intelligence mutation host rejected observation: {error}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agent_id() -> AgentId {
        AgentId::parse("00000000-0000-4000-8000-000000000244").expect("agent id")
    }

    #[test]
    fn envelope_is_spawn_bound_and_authority_negative() {
        let agent_id = agent_id();
        let payload = serde_json::json!({
            "schema_version": 1,
            "operation_id": "shadow-operation",
            "production_authority": false
        })
        .to_string();
        let envelope = wrap_payload(&agent_id, 7, "inspect", &payload).expect("envelope");
        let envelope: Value = serde_json::from_str(&envelope).expect("json");
        assert_eq!(envelope["agent_id"], agent_id.as_str());
        assert_eq!(envelope["spawn_generation"], 7);
        assert!(envelope["host_receipt_sha256"].is_string());
        assert_eq!(envelope["runtime_wired"], false);
        assert_eq!(envelope["app_runtime_attached"], false);
        assert_eq!(envelope["tool_registered"], false);
        assert_eq!(envelope["memory_write_performed_by_agentd"], false);
        assert_eq!(envelope["projection_write_performed_by_agentd"], false);
        assert_eq!(envelope["outbox_dispatch_performed_by_agentd"], false);
        assert_eq!(envelope["production_authority"], false);
    }

    #[test]
    fn prepared_envelope_round_trips_and_rejects_cross_spawn() {
        let agent_id = agent_id();
        let prepared = serde_json::json!({
            "schema_version": 1,
            "namespace": "shadow_intelligence_mutation_prepared_observation_v1",
            "prepared_sha256": Sha256Digest::for_bytes(b"prepared").as_str()
        })
        .to_string();
        let envelope = wrap_payload(&agent_id, 7, "prepare", &prepared).expect("envelope");
        let unwrapped = unwrap_prepared_payload(&agent_id, 7, &envelope).expect("unwrap");
        let original: Value = serde_json::from_str(&prepared).expect("original");
        let round_trip: Value = serde_json::from_str(&unwrapped).expect("round trip");
        assert_eq!(round_trip, original);
        assert!(unwrap_prepared_payload(&agent_id, 8, &envelope).is_err());
    }

    #[test]
    fn prepared_envelope_rejects_payload_and_authority_tamper() {
        let agent_id = agent_id();
        let prepared = serde_json::json!({
            "schema_version": 1,
            "prepared_sha256": Sha256Digest::for_bytes(b"prepared").as_str()
        })
        .to_string();
        let envelope = wrap_payload(&agent_id, 7, "prepare", &prepared).expect("envelope");
        let mut tampered: Value = serde_json::from_str(&envelope).expect("json");
        tampered["payload"]["prepared_sha256"] =
            Value::String(Sha256Digest::for_bytes(b"changed").as_str().to_string());
        assert!(
            unwrap_prepared_payload(&agent_id, 7, &tampered.to_string()).is_err()
        );

        let mut escalated: Value = serde_json::from_str(&envelope).expect("json");
        escalated["production_authority"] = Value::Bool(true);
        assert!(
            unwrap_prepared_payload(&agent_id, 7, &escalated.to_string()).is_err()
        );
    }

    #[test]
    fn operation_and_causal_root_are_spawn_bound() {
        let agent_id = agent_id();
        let operation_7 =
            effective_operation_id(&agent_id, 7, "operation").expect("operation 7");
        let operation_8 =
            effective_operation_id(&agent_id, 8, "operation").expect("operation 8");
        assert_ne!(operation_7, operation_8);
        let root = Sha256Digest::for_bytes(b"root");
        assert_ne!(
            host_bound_causal_root(&agent_id, 7, &root),
            host_bound_causal_root(&agent_id, 8, &root)
        );
    }

    #[test]
    fn zero_spawn_generation_is_fenced() {
        let agent_id = agent_id();
        assert!(wrap_payload(&agent_id, 0, "inspect", "{}").is_err());
    }
}
