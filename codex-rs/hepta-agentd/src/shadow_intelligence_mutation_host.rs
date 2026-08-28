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

#[derive(Debug, Serialize)]
struct AgentdShadowHostEnvelope {
    schema_version: u32,
    namespace: &'static str,
    action: &'static str,
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

    pub async fn begin(&self, binding_json: &str) -> Result<String, AgentdError> {
        let payload = self
            .store
            .begin_shadow_intelligence_mutation(binding_json)
            .await
            .map_err(shadow_error)?;
        self.wrap("begin", payload)
    }

    pub async fn prepare(
        &self,
        operation_id: &str,
        observation_json: &str,
    ) -> Result<String, AgentdError> {
        let payload = self
            .store
            .prepare_shadow_intelligence_mutation_observation(operation_id, observation_json)
            .await
            .map_err(shadow_error)?;
        self.wrap("prepare", payload)
    }

    pub async fn append(&self, prepared_json: &str) -> Result<String, AgentdError> {
        let payload = self
            .store
            .append_shadow_intelligence_mutation_observation(prepared_json)
            .await
            .map_err(shadow_error)?;
        self.wrap("append", payload)
    }

    pub async fn observe(
        &self,
        operation_id: &str,
        observation_json: &str,
    ) -> Result<String, AgentdError> {
        let payload = self
            .store
            .observe_shadow_intelligence_mutation(operation_id, observation_json)
            .await
            .map_err(shadow_error)?;
        self.wrap("observe", payload)
    }

    pub async fn inspect(&self, operation_id: &str) -> Result<String, AgentdError> {
        let payload = self
            .store
            .inspect_shadow_intelligence_mutation(operation_id)
            .await
            .map_err(shadow_error)?;
        self.wrap("inspect", payload)
    }

    fn wrap(&self, action: &'static str, payload_json: String) -> Result<String, AgentdError> {
        wrap_payload(
            &self.agent_id,
            self.spawn_generation,
            action,
            &payload_json,
        )
    }
}

fn wrap_payload(
    agent_id: &AgentId,
    spawn_generation: u64,
    action: &'static str,
    payload_json: &str,
) -> Result<String, AgentdError> {
    if spawn_generation == 0 {
        return Err(AgentdError::GenerationFenced(
            "shadow intelligence mutation host requires a positive spawn generation"
                .to_string(),
        ));
    }
    let payload: Value = serde_json::from_str(payload_json)?;
    let payload_sha256 = Sha256Digest::for_bytes(payload_json.as_bytes());
    let mut framed = Vec::new();
    let mut push = |part: &[u8]| {
        framed.extend_from_slice(&(part.len() as u64).to_be_bytes());
        framed.extend_from_slice(part);
    };
    push(b"hepta-agentd:shadow-intelligence-mutation-host:v1");
    push(agent_id.as_str().as_bytes());
    push(&spawn_generation.to_be_bytes());
    push(action.as_bytes());
    push(payload_sha256.as_str().as_bytes());
    let host_receipt_sha256 = Sha256Digest::for_bytes(&framed);
    Ok(serde_json::to_string(&AgentdShadowHostEnvelope {
        schema_version: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION,
        namespace: AGENTD_SHADOW_INTELLIGENCE_MUTATION_HOST_NAMESPACE,
        action,
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

fn shadow_error(error: codex_hepta_memory::CognitiveStoreError) -> AgentdError {
    AgentdError::Protocol(format!(
        "shadow intelligence mutation host rejected observation: {error}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_is_spawn_bound_and_authority_negative() {
        let agent_id = AgentId::parse("00000000-0000-4000-8000-000000000244")
            .expect("agent id");
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
    fn zero_spawn_generation_is_fenced() {
        let agent_id = AgentId::parse("00000000-0000-4000-8000-000000000245")
            .expect("agent id");
        assert!(wrap_payload(&agent_id, 0, "inspect", "{}").is_err());
    }
}
