//! One-process-per-workspace Hepta agent host.
//!
//! `agentd` binds exactly one fleet `AgentId`, embeds the existing Codex App
//! Server execution path, and exposes a small local lifecycle/control socket.
//! It does not implement a second runtime kernel or a fleet-wide message bus.

mod app_runtime;
mod client;
mod config;
mod control;
mod error;
mod event_buffer;
mod runtime;
mod state;

pub use client::AgentdClient;
pub use codex_hepta_agent_protocol::AGENTD_CONTROL_SCHEMA_VERSION;
pub use codex_hepta_agent_protocol::AgentdEvent;
pub use codex_hepta_agent_protocol::AgentdEventKind;
pub use codex_hepta_agent_protocol::AgentdMethod;
pub use codex_hepta_agent_protocol::AgentdPayload;
pub use codex_hepta_agent_protocol::AgentdRequest;
pub use codex_hepta_agent_protocol::AgentdResponse;
pub use codex_hepta_agent_protocol::EventBatch;
pub use codex_hepta_agent_protocol::HealthSnapshot;
pub use codex_hepta_agent_protocol::LifecycleSnapshot;
pub use codex_hepta_agent_protocol::MAX_CONTROL_FRAME_BYTES;
pub use codex_hepta_agent_protocol::MAX_EVENT_BATCH;
pub use codex_hepta_agent_protocol::SessionIngress;
pub use codex_hepta_agent_protocol::SessionTransport;
pub use config::AgentdConfig;
pub use config::AgentdIdentity;
pub use config::HEPTA_AGENT_GENERATION_ENV;
pub use config::HEPTA_AGENT_HOME_ENV;
pub use config::HEPTA_AGENT_ID_ENV;
pub use config::HEPTA_AGENT_RUN_ROOT_ENV;
pub use error::AgentdError;
pub use runtime::run;

use control::AgentdControlServer;
use event_buffer::EventBuffer;
use state::AgentdState;
