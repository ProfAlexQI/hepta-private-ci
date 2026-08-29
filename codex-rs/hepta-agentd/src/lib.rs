#![forbid(unsafe_code)]

#[cfg(not(unix))]
compile_error!("codex-hepta-agentd requires Unix domain sockets");

mod app_runtime;
mod automation;
mod automation_service;
mod client;
mod composition;
mod config;
mod control;
mod error;
mod event_buffer;
mod memory_service;
mod production_authority_adapter;
mod production_writer_host;
mod qualification_writer;
mod runtime;
mod runtime_profile;
mod state;

use std::collections::BTreeMap;
use std::process::ExitStatus;
use std::sync::Arc;

use codex_app_server_protocol::ConversationId;
use codex_app_server_protocol::ThreadId;
use codex_hepta_automation::AutomationStore;
use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::FleetRegistry;
use codex_hepta_memory::CognitiveStore;
use codex_hepta_paths::HeptaAgentLayout;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::Mutex;

pub use client::AgentdClient;
pub use config::AgentdConfig;
pub use config::AgentdIdentity;
pub use config::HEPTA_AGENT_GENERATION_ENV;
pub use config::HEPTA_AGENT_HOME_ENV;
pub use config::HEPTA_AGENT_ID_ENV;
pub use config::HEPTA_AGENT_RUN_ROOT_ENV;
pub use error::AgentdError;
pub use production_writer_host::AgentdProductionWriterHost;
pub use runtime::run;

use control::AgentdControlServer;
use event_buffer::EventBuffer;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentdMethod {
    Status,
    Kill,
    CreateAutomation,
    UpdateAutomation,
    CancelAutomation,
    AutomationStatus,
    AutomationList,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentdRequest {
    pub method: AgentdMethod,
    #[serde(default)]
    pub payload: AgentdPayload,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentdPayload {
    pub project_id: Option<String>,
    pub thread_id: Option<ConversationId>,
    pub server_thread_id: Option<ThreadId>,
    pub turn_id: Option<String>,
    pub automation_task_id: Option<String>,
    pub automation_thread_id: Option<String>,
    pub automation_prompt: Option<String>,
    pub automation_schedule: Option<codex_hepta_automation::AutomationSchedule>,
    pub automation_first_run_at_ms: Option<u64>,
    pub automation_enabled: Option<bool>,
    pub automation_expected_updated_at_ms: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentdResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub result: Option<AgentdResult>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentdResult {
    pub agent_id: AgentId,
    pub lifecycle: codex_hepta_fleet::AgentLifecycle,
    pub generation: u64,
    pub ready: bool,
    pub ready_since_unix_ms: Option<u64>,
    pub app_server_socket: String,
    pub cognitive_available: bool,
    pub automation_available: bool,
    pub recorded_threads: u64,
    pub event_sequence: u64,
    pub stop_requested: bool,
    pub last_runtime_error: Option<String>,
    pub automation_task: Option<codex_hepta_automation::AutomationTask>,
    pub automation_tasks: Option<Vec<codex_hepta_automation::AutomationTask>>,
}

pub(crate) struct AgentdState {
    identity: AgentdIdentity,
    registry: FleetRegistry,
    cognitive_store: Mutex<Option<Arc<CognitiveStore>>>,
    automation_store: Mutex<Option<AutomationStore>>,
    threads: Mutex<BTreeMap<String, ThreadRuntimeState>>,
    events: EventBuffer,
    runtime: Mutex<RuntimeState>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ThreadRuntimeState {
    thread_id: ConversationId,
    server_thread_id: Option<ThreadId>,
    latest_turn_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct RuntimeState {
    ready: bool,
    ready_since_unix_ms: Option<u64>,
    stop_requested: bool,
    last_runtime_error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PersistedThreadState {
    version: u32,
    agent_id: AgentId,
    thread_id: ConversationId,
    server_thread_id: Option<ThreadId>,
    latest_turn_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentEvent {
    pub sequence: u64,
    pub occurred_at_unix_ms: u64,
    pub kind: String,
    pub project_id: Option<String>,
    pub thread_id: Option<ConversationId>,
    pub server_thread_id: Option<ThreadId>,
    pub turn_id: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppServerRunPrerequisites {
    pub environment_overrides_applied_before_core: bool,
    pub account_manager_secondary: bool,
    pub apps_manager_secondary: bool,
    pub no_auth_error: bool,
    pub model_list_ready: bool,
    pub app_server_initialized: bool,
}

impl AppServerRunPrerequisites {
    pub fn allows_running(self) -> bool {
        self.environment_overrides_applied_before_core
            && self.account_manager_secondary
            && self.apps_manager_secondary
            && self.no_auth_error
            && self.model_list_ready
            && self.app_server_initialized
    }

    pub fn blockers(self) -> Vec<&'static str> {
        let mut blockers = Vec::new();
        if !self.environment_overrides_applied_before_core {
            blockers.push("environment_overrides_not_applied");
        }
        if !self.account_manager_secondary {
            blockers.push("account_manager_primary");
        }
        if !self.apps_manager_secondary {
            blockers.push("apps_manager_primary");
        }
        if !self.no_auth_error {
            blockers.push("auth_error");
        }
        if !self.model_list_ready {
            blockers.push("model_list_unavailable");
        }
        if !self.app_server_initialized {
            blockers.push("app_server_uninitialized");
        }
        blockers
    }
}

#[derive(Debug)]
pub enum AgentProcessExit {
    Exited(ExitStatus),
    Signaled,
}

impl AgentProcessExit {
    pub fn clean(&self) -> bool {
        match self {
            Self::Exited(status) => status.success(),
            Self::Signaled => false,
        }
    }
}

pub struct AgentHandle {
    pub agent_id: AgentId,
    pub layout: HeptaAgentLayout,
    pub pid: u32,
    pub process: tokio::process::Child,
}
