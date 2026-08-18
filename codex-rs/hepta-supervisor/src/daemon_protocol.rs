use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::ReleaseId;
use serde::Deserialize;
use serde::Serialize;

pub const SUPERVISORD_CONTROL_SCHEMA_VERSION: u32 = 1;
pub const MAX_SUPERVISORD_CONTROL_FRAME_BYTES: u64 = 65_536;
pub const MAX_SUPERVISORD_ROSTER: u16 = 256;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupervisordRequest {
    pub schema_version: u32,
    pub request_id: u64,
    pub method: SupervisordMethod,
}

impl SupervisordRequest {
    pub fn new(request_id: u64, method: SupervisordMethod) -> Self {
        Self {
            schema_version: SUPERVISORD_CONTROL_SCHEMA_VERSION,
            request_id,
            method,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum SupervisordMethod {
    Health,
    Roster {
        limit: u16,
    },
    Snapshot {
        agent_id: AgentId,
    },
    Start {
        agent_id: AgentId,
        release_id: ReleaseId,
    },
    Drain {
        agent_id: AgentId,
    },
    Stop {
        agent_id: AgentId,
    },
    Kill {
        agent_id: AgentId,
    },
    Restart {
        agent_id: AgentId,
    },
    Upgrade {
        agent_id: AgentId,
        release_id: ReleaseId,
    },
    Rollback {
        agent_id: AgentId,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupervisordResponse {
    pub schema_version: u32,
    pub request_id: u64,
    pub payload: SupervisordPayload,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum SupervisordPayload {
    Health(SupervisordHealth),
    Roster { agents: Vec<SupervisordAgentStatus> },
    Agent(SupervisordAgentStatus),
    Error { code: String, message: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupervisordHealth {
    pub ready: bool,
    pub process_id: u32,
    pub registered_agents: u16,
    pub observed_faults: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupervisordAgentStatus {
    pub agent_id: AgentId,
    pub lifecycle: AgentLifecycle,
    pub lifecycle_generation: u64,
    pub active: bool,
    pub healthy: bool,
    pub process_id: Option<u64>,
    pub spawn_generation: Option<u64>,
    pub runtime_generation: Option<u64>,
    pub current_release: Option<ReleaseId>,
    pub previous_release: Option<ReleaseId>,
    pub release_change_pending: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_wire_does_not_accept_commands_or_paths() {
        let request = SupervisordRequest::new(
            7,
            SupervisordMethod::Upgrade {
                agent_id: AgentId::parse("018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12")
                    .expect("fixed agent id"),
                release_id: ReleaseId::parse("agentd-v2").expect("fixed release id"),
            },
        );
        let bytes = serde_json::to_vec(&request).expect("serialize request");
        assert!(bytes.len() as u64 <= MAX_SUPERVISORD_CONTROL_FRAME_BYTES);
        assert_eq!(
            serde_json::from_slice::<SupervisordRequest>(&bytes).expect("parse request"),
            request
        );
        assert!(!String::from_utf8_lossy(&bytes).contains("program"));
        assert!(!String::from_utf8_lossy(&bytes).contains("args"));
    }
}
