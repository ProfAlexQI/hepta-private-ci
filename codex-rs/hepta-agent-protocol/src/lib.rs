//! Bounded local control protocol shared by one Hepta agent daemon and its supervisor.

#![forbid(unsafe_code)]

use std::path::PathBuf;

use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use serde::Deserialize;
use serde::Serialize;

pub const AGENTD_CONTROL_SCHEMA_VERSION: u32 = 1;
pub const MAX_CONTROL_FRAME_BYTES: u64 = 65_536;
pub const MAX_EVENT_BATCH: u16 = 256;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentdRequest {
    pub schema_version: u32,
    pub request_id: u64,
    pub spawn_generation: u64,
    pub method: AgentdMethod,
}

impl AgentdRequest {
    pub fn health(request_id: u64, spawn_generation: u64) -> Self {
        Self {
            schema_version: AGENTD_CONTROL_SCHEMA_VERSION,
            request_id,
            spawn_generation,
            method: AgentdMethod::Health,
        }
    }

    pub fn lifecycle(request_id: u64, spawn_generation: u64) -> Self {
        Self {
            schema_version: AGENTD_CONTROL_SCHEMA_VERSION,
            request_id,
            spawn_generation,
            method: AgentdMethod::Lifecycle,
        }
    }

    pub fn session_ingress(request_id: u64, spawn_generation: u64) -> Self {
        Self {
            schema_version: AGENTD_CONTROL_SCHEMA_VERSION,
            request_id,
            spawn_generation,
            method: AgentdMethod::SessionIngress,
        }
    }

    pub fn events(request_id: u64, spawn_generation: u64, after_cursor: u64, limit: u16) -> Self {
        Self {
            schema_version: AGENTD_CONTROL_SCHEMA_VERSION,
            request_id,
            spawn_generation,
            method: AgentdMethod::Events {
                after_cursor,
                limit,
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgentdMethod {
    Health,
    Lifecycle,
    SessionIngress,
    Events { after_cursor: u64, limit: u16 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentdResponse {
    pub schema_version: u32,
    pub request_id: u64,
    pub agent_id: AgentId,
    pub spawn_generation: u64,
    pub current_generation: u64,
    pub payload: AgentdPayload,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgentdPayload {
    Health(HealthSnapshot),
    Lifecycle(LifecycleSnapshot),
    SessionIngress(SessionIngress),
    Events(EventBatch),
    Error { code: String, message: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HealthSnapshot {
    /// True once the App Server initialized while this exact spawn still owns Starting.
    pub promotion_ready: bool,
    /// True only after the supervisor promoted this spawn to Running.
    pub ready: bool,
    pub fenced: bool,
    pub process_id: u32,
    pub workspace: PathBuf,
    pub home_root: PathBuf,
    pub run_root: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleSnapshot {
    pub lifecycle: AgentLifecycle,
    pub app_server_ready: bool,
    pub fenced: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SessionIngress {
    pub socket_path: PathBuf,
    pub transport: SessionTransport,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionTransport {
    CodexAppServerWebsocketOverUds,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentdEvent {
    pub cursor: u64,
    pub kind: AgentdEventKind,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum AgentdEventKind {
    Bootstrapped,
    Lifecycle {
        lifecycle: AgentLifecycle,
        generation: u64,
    },
    AppServerReady,
    Draining,
    GenerationFenced,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EventBatch {
    pub events: Vec<AgentdEvent>,
    pub gap: bool,
    pub next_cursor: u64,
    pub latest_cursor: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_wire_round_trip_is_strict_and_bounded() {
        let request = AgentdRequest::health(7, 11);
        let request_bytes = serde_json::to_vec(&request).expect("serialize request");
        assert!(request_bytes.len() as u64 <= MAX_CONTROL_FRAME_BYTES);
        assert_eq!(
            serde_json::from_slice::<AgentdRequest>(&request_bytes).expect("parse request"),
            request
        );

        let response = AgentdResponse {
            schema_version: AGENTD_CONTROL_SCHEMA_VERSION,
            request_id: 7,
            agent_id: AgentId::parse("018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12").expect("agent id"),
            spawn_generation: 11,
            current_generation: 11,
            payload: AgentdPayload::Health(HealthSnapshot {
                promotion_ready: true,
                ready: true,
                fenced: false,
                process_id: 17,
                workspace: PathBuf::from("/tmp/workspace"),
                home_root: PathBuf::from("/tmp/home"),
                run_root: PathBuf::from("/tmp/run"),
            }),
        };
        let response_bytes = serde_json::to_vec(&response).expect("serialize response");
        assert!(response_bytes.len() as u64 <= MAX_CONTROL_FRAME_BYTES);
        assert_eq!(
            serde_json::from_slice::<AgentdResponse>(&response_bytes).expect("parse response"),
            response
        );
    }
}
