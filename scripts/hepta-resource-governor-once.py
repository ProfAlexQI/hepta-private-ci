#!/usr/bin/env python3
"""Materialize the Hepta App Server runtime resource governor exactly once."""

from __future__ import annotations

import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]


def path(relative: str) -> pathlib.Path:
    return ROOT / relative


def read(relative: str) -> str:
    return path(relative).read_text(encoding="utf-8")


def write(relative: str, content: str) -> None:
    target = path(relative)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(content, encoding="utf-8")


def replace(relative: str, old: str, new: str, count: int = 1) -> None:
    source = read(relative)
    actual = source.count(old)
    if actual != count:
        raise SystemExit(
            f"{relative}: expected {count} occurrence(s) of {old!r}, found {actual}"
        )
    write(relative, source.replace(old, new))


def add_governor_module() -> None:
    write(
        "codex-rs/app-server/src/runtime_resource_governor.rs",
        r'''use std::num::NonZeroUsize;
use std::sync::Arc;

use codex_app_server_protocol::JSONRPCErrorError;
use tokio::sync::OwnedSemaphorePermit;
use tokio::sync::Semaphore;
use tokio::sync::TryAcquireError;

use crate::error_code::OVERLOADED_ERROR_CODE;
use crate::error_code::internal_error;

pub(crate) type RuntimeResourcePermit = OwnedSemaphorePermit;

#[derive(Clone, Default)]
pub(crate) struct RuntimeResourceGovernor {
    turn_permits: Option<ResourcePermitPool>,
    tool_process_permits: Option<ResourcePermitPool>,
}

#[derive(Clone)]
struct ResourcePermitPool {
    resource: &'static str,
    limit: NonZeroUsize,
    semaphore: Arc<Semaphore>,
}

impl RuntimeResourceGovernor {
    pub(crate) fn new(
        max_concurrent_turns: Option<NonZeroUsize>,
        max_tool_processes: Option<NonZeroUsize>,
    ) -> Self {
        Self {
            turn_permits: max_concurrent_turns
                .map(|limit| ResourcePermitPool::new("max_concurrent_turns", limit)),
            tool_process_permits: max_tool_processes
                .map(|limit| ResourcePermitPool::new("max_tool_processes", limit)),
        }
    }

    pub(crate) fn try_acquire_turn(
        &self,
    ) -> Result<Option<RuntimeResourcePermit>, JSONRPCErrorError> {
        self.turn_permits
            .as_ref()
            .map(ResourcePermitPool::try_acquire)
            .transpose()
    }

    pub(crate) fn try_acquire_tool_process(
        &self,
    ) -> Result<Option<RuntimeResourcePermit>, JSONRPCErrorError> {
        self.tool_process_permits
            .as_ref()
            .map(ResourcePermitPool::try_acquire)
            .transpose()
    }
}

impl ResourcePermitPool {
    fn new(resource: &'static str, limit: NonZeroUsize) -> Self {
        Self {
            resource,
            limit,
            semaphore: Arc::new(Semaphore::new(limit.get())),
        }
    }

    fn try_acquire(&self) -> Result<RuntimeResourcePermit, JSONRPCErrorError> {
        match Arc::clone(&self.semaphore).try_acquire_owned() {
            Ok(permit) => Ok(permit),
            Err(TryAcquireError::NoPermits) => {
                let mut error = JSONRPCErrorError {
                    code: OVERLOADED_ERROR_CODE,
                    message: format!(
                        "{} resource budget exhausted (limit {})",
                        self.resource, self.limit
                    ),
                    data: None,
                };
                error.data = Some(serde_json::json!({
                    "error_code": "resource_budget_exhausted",
                    "resource": self.resource,
                    "limit": self.limit.get(),
                    "retryable": false,
                }));
                Err(error)
            }
            Err(TryAcquireError::Closed) => Err(internal_error(format!(
                "{} resource governor closed unexpectedly",
                self.resource
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_budget_rejects_n_plus_one_and_recovers_on_drop() {
        let governor = RuntimeResourceGovernor::new(NonZeroUsize::new(1), None);
        let permit = governor
            .try_acquire_turn()
            .expect("first acquisition")
            .expect("configured permit");
        let error = governor
            .try_acquire_turn()
            .expect_err("N+1 acquisition must fail closed");
        assert_eq!(error.code, OVERLOADED_ERROR_CODE);
        assert_eq!(
            error.data.as_ref().and_then(|data| data["resource"].as_str()),
            Some("max_concurrent_turns")
        );
        drop(permit);
        assert!(
            governor
                .try_acquire_turn()
                .expect("permit must recover")
                .is_some()
        );
    }

    #[test]
    fn tool_budget_is_shared_and_unconfigured_budget_is_unbounded() {
        let governor = RuntimeResourceGovernor::new(None, NonZeroUsize::new(1));
        assert!(governor.try_acquire_turn().expect("unbounded turn").is_none());
        let permit = governor
            .try_acquire_tool_process()
            .expect("first tool acquisition")
            .expect("configured tool permit");
        let error = governor
            .try_acquire_tool_process()
            .expect_err("second tool process must fail closed");
        assert_eq!(error.code, OVERLOADED_ERROR_CODE);
        drop(permit);
        assert!(
            governor
                .try_acquire_tool_process()
                .expect("tool permit must recover")
                .is_some()
        );
    }
}
''',
    )


def patch_lib() -> None:
    relative = "codex-rs/app-server/src/lib.rs"
    replace(relative, "mod request_serialization;\n", "mod request_serialization;\nmod runtime_resource_governor;\n")
    replace(
        relative,
        "    pub turn_queue_capacity: Option<NonZeroUsize>,\n",
        "    pub turn_queue_capacity: Option<NonZeroUsize>,\n"
        "    /// Optional process-wide limit for concurrently active assistant turns.\n"
        "    /// Admission is fail-closed and never waits in an implicit in-memory queue.\n"
        "    pub max_concurrent_turns: Option<NonZeroUsize>,\n"
        "    /// Optional process-wide limit shared by command/exec and process/spawn.\n"
        "    pub max_tool_processes: Option<NonZeroUsize>,\n",
    )
    replace(
        relative,
        '            .field("turn_queue_capacity", &self.turn_queue_capacity)\n',
        '            .field("turn_queue_capacity", &self.turn_queue_capacity)\n'
        '            .field("max_concurrent_turns", &self.max_concurrent_turns)\n'
        '            .field("max_tool_processes", &self.max_tool_processes)\n',
    )
    replace(
        relative,
        "            && self.turn_queue_capacity == other.turn_queue_capacity\n",
        "            && self.turn_queue_capacity == other.turn_queue_capacity\n"
        "            && self.max_concurrent_turns == other.max_concurrent_turns\n"
        "            && self.max_tool_processes == other.max_tool_processes\n",
    )
    replace(
        relative,
        "            turn_queue_capacity: None,\n",
        "            turn_queue_capacity: None,\n"
        "            max_concurrent_turns: None,\n"
        "            max_tool_processes: None,\n",
    )
    replace(
        relative,
        "            turn_queue_capacity: runtime_options.turn_queue_capacity,\n",
        "            turn_queue_capacity: runtime_options.turn_queue_capacity,\n"
        "            max_concurrent_turns: runtime_options.max_concurrent_turns,\n"
        "            max_tool_processes: runtime_options.max_tool_processes,\n",
    )


def patch_message_processor() -> None:
    relative = "codex-rs/app-server/src/message_processor.rs"
    replace(
        relative,
        "use crate::request_serialization::RequestSerializationQueues;\n",
        "use crate::request_serialization::RequestSerializationQueues;\n"
        "use crate::runtime_resource_governor::RuntimeResourceGovernor;\n",
    )
    replace(
        relative,
        "    pub(crate) turn_queue_capacity: Option<NonZeroUsize>,\n",
        "    pub(crate) turn_queue_capacity: Option<NonZeroUsize>,\n"
        "    pub(crate) max_concurrent_turns: Option<NonZeroUsize>,\n"
        "    pub(crate) max_tool_processes: Option<NonZeroUsize>,\n",
    )
    replace(
        relative,
        "            turn_queue_capacity,\n            hepta_cognitive_runtime,\n",
        "            turn_queue_capacity,\n"
        "            max_concurrent_turns,\n"
        "            max_tool_processes,\n"
        "            hepta_cognitive_runtime,\n",
    )
    replace(
        relative,
        "        let thread_state_manager = ThreadStateManager::new();\n",
        "        let thread_state_manager = ThreadStateManager::new();\n"
        "        let resource_governor =\n"
        "            RuntimeResourceGovernor::new(max_concurrent_turns, max_tool_processes);\n",
    )
    replace(
        relative,
        "            Arc::clone(&environment_manager_for_requests),\n        );\n        let process_exec_processor = ProcessExecRequestProcessor::new(\n            outgoing.clone(),\n            Arc::clone(&environment_manager_for_requests),\n        );\n",
        "            Arc::clone(&environment_manager_for_requests),\n"
        "            resource_governor.clone(),\n"
        "        );\n"
        "        let process_exec_processor = ProcessExecRequestProcessor::new(\n"
        "            outgoing.clone(),\n"
        "            Arc::clone(&environment_manager_for_requests),\n"
        "            resource_governor.clone(),\n"
        "        );\n",
    )
    replace(
        relative,
        "            turn_cost_worker.as_ref().map(TurnCostWorker::handle),\n        );\n        if matches!(plugin_startup_tasks",
        "            turn_cost_worker.as_ref().map(TurnCostWorker::handle),\n"
        "            resource_governor,\n"
        "        );\n"
        "        if matches!(plugin_startup_tasks",
    )


def patch_request_processors_imports() -> None:
    replace(
        "codex-rs/app-server/src/request_processors.rs",
        "use crate::request_serialization::RequestSerializationQueues;\n",
        "use crate::request_serialization::RequestSerializationQueues;\n"
        "use crate::runtime_resource_governor::RuntimeResourceGovernor;\n",
    )


def patch_turn_processor() -> None:
    relative = "codex-rs/app-server/src/request_processors/turn_processor.rs"
    replace(
        relative,
        "    turn_cost_worker: Option<crate::turn_cost_worker::TurnCostWorkerHandle>,\n",
        "    turn_cost_worker: Option<crate::turn_cost_worker::TurnCostWorkerHandle>,\n"
        "    resource_governor: RuntimeResourceGovernor,\n",
    )
    replace(
        relative,
        "        turn_cost_worker: Option<crate::turn_cost_worker::TurnCostWorkerHandle>,\n    ) -> Self {\n",
        "        turn_cost_worker: Option<crate::turn_cost_worker::TurnCostWorkerHandle>,\n"
        "        resource_governor: RuntimeResourceGovernor,\n"
        "    ) -> Self {\n",
    )
    replace(
        relative,
        "            turn_cost_worker,\n        }\n",
        "            turn_cost_worker,\n"
        "            resource_governor,\n"
        "        }\n",
        count=1,
    )
    old = '''        // Eligible memory startup uses Core's exact admission result. A steered,
        // queued, recovered, or rejected input must never start detached Memory.
        let turn_id = if should_capture_memory_policy {
            let submission = thread
                .start_or_steer_turn_and_capture_memory_policy(turn_input_request)
                .await
                .map_err(|err| {
                    let error = internal_error(format!("failed to submit turn input: {err}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    error
                })?;
            match submission {
                codex_core::MemoryTurnInputSubmission::Started {
                    turn_id,
                    provider_policy,
                } => {
                    let config = thread.config().await;
                    let config_snapshot = thread.config_snapshot().await;
                    let parent_permission_profile = parent_permission_profile_override
                        .unwrap_or(config_snapshot.permission_profile);
                    codex_memories_write::start_memories_startup_task(
                        Arc::clone(&self.thread_manager),
                        Arc::clone(&self.auth_manager),
                        thread_id,
                        Arc::clone(&thread),
                        provider_policy,
                        config,
                        parent_permission_profile,
                        &config_snapshot.session_source,
                    );
                    turn_id
                }
                codex_core::MemoryTurnInputSubmission::Steered { turn_id } => turn_id,
                codex_core::MemoryTurnInputSubmission::NotSubmitted { reason } => {
                    let error = internal_error(format!("failed to submit turn input: {reason:?}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    return Err(error);
                }
            }
        } else {
            let submission = thread
                .start_or_steer_turn(turn_input_request)
                .await
                .map_err(|err| {
                    let error = internal_error(format!("failed to submit turn input: {err}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    error
                })?;
            match submission {
                TurnInputSubmission::Started { turn_id }
                | TurnInputSubmission::Steered { turn_id } => turn_id,
                TurnInputSubmission::NotSubmitted { reason } => {
                    let error = internal_error(format!("failed to submit turn input: {reason:?}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    return Err(error);
                }
            }
        };
'''
    new = '''        // Eligible memory startup uses Core's exact admission result. A steered,
        // queued, recovered, or rejected input must never start detached Memory.
        // Admission takes a non-blocking process-wide permit. A steering result
        // returns that provisional permit immediately; only a newly started turn
        // binds it to the listener lifecycle.
        let mut turn_resource_permit = self.resource_governor.try_acquire_turn()?;
        let (turn_id, started_new_turn) = if should_capture_memory_policy {
            let submission = thread
                .start_or_steer_turn_and_capture_memory_policy(turn_input_request)
                .await
                .map_err(|err| {
                    let error = internal_error(format!("failed to submit turn input: {err}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    error
                })?;
            match submission {
                codex_core::MemoryTurnInputSubmission::Started {
                    turn_id,
                    provider_policy,
                } => {
                    let config = thread.config().await;
                    let config_snapshot = thread.config_snapshot().await;
                    let parent_permission_profile = parent_permission_profile_override
                        .unwrap_or(config_snapshot.permission_profile);
                    codex_memories_write::start_memories_startup_task(
                        Arc::clone(&self.thread_manager),
                        Arc::clone(&self.auth_manager),
                        thread_id,
                        Arc::clone(&thread),
                        provider_policy,
                        config,
                        parent_permission_profile,
                        &config_snapshot.session_source,
                    );
                    (turn_id, true)
                }
                codex_core::MemoryTurnInputSubmission::Steered { turn_id } => (turn_id, false),
                codex_core::MemoryTurnInputSubmission::NotSubmitted { reason } => {
                    let error = internal_error(format!("failed to submit turn input: {reason:?}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    return Err(error);
                }
            }
        } else {
            let submission = thread
                .start_or_steer_turn(turn_input_request)
                .await
                .map_err(|err| {
                    let error = internal_error(format!("failed to submit turn input: {err}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    error
                })?;
            match submission {
                TurnInputSubmission::Started { turn_id } => (turn_id, true),
                TurnInputSubmission::Steered { turn_id } => (turn_id, false),
                TurnInputSubmission::NotSubmitted { reason } => {
                    let error = internal_error(format!("failed to submit turn input: {reason:?}"));
                    self.track_error_response(&request_id, &error, /*error_type*/ None);
                    return Err(error);
                }
            }
        };
        if started_new_turn
            && let Some(permit) = turn_resource_permit.take()
        {
            self.thread_state_manager
                .thread_state(thread_id)
                .await
                .lock()
                .await
                .bind_turn_resource_permit(&turn_id, permit);
        }
'''
    replace(relative, old, new)


def patch_thread_state() -> None:
    relative = "codex-rs/app-server/src/thread_state.rs"
    replace(
        relative,
        "use tokio::sync::Mutex;\n",
        "use tokio::sync::Mutex;\nuse tokio::sync::OwnedSemaphorePermit;\n",
    )
    replace(
        relative,
        "    pub(crate) last_terminal_turn_id: Option<String>,\n",
        "    pub(crate) last_terminal_turn_id: Option<String>,\n"
        "    turn_resource_permits: HashMap<String, OwnedSemaphorePermit>,\n",
    )
    replace(
        relative,
        "        self.current_turn_history.reset();\n        self.listener_thread = None;\n",
        "        self.current_turn_history.reset();\n"
        "        self.turn_resource_permits.clear();\n"
        "        self.listener_thread = None;\n",
    )
    replace(
        relative,
        "    pub(crate) fn track_current_turn_event(&mut self, event_turn_id: &str, event: &EventMsg) {\n",
        "    pub(crate) fn bind_turn_resource_permit(\n"
        "        &mut self,\n"
        "        turn_id: &str,\n"
        "        permit: OwnedSemaphorePermit,\n"
        "    ) {\n"
        "        if self.last_terminal_turn_id.as_deref() == Some(turn_id) {\n"
        "            return;\n"
        "        }\n"
        "        self.turn_resource_permits\n"
        "            .insert(turn_id.to_string(), permit);\n"
        "    }\n\n"
        "    pub(crate) fn track_current_turn_event(&mut self, event_turn_id: &str, event: &EventMsg) {\n",
    )
    replace(
        relative,
        "            self.last_terminal_turn_id = Some(event_turn_id.to_string());\n",
        "            self.last_terminal_turn_id = Some(event_turn_id.to_string());\n"
        "            self.turn_resource_permits.remove(event_turn_id);\n",
    )
    replace(
        relative,
        "    #[test]\n    fn note_thread_settings_reports_only_effective_changes() {\n",
        "    #[test]\n"
        "    fn terminal_before_bind_drops_turn_permit_without_leak() {\n"
        "        let governor = crate::runtime_resource_governor::RuntimeResourceGovernor::new(\n"
        "            std::num::NonZeroUsize::new(1),\n"
        "            None,\n"
        "        );\n"
        "        let permit = governor\n"
        "            .try_acquire_turn()\n"
        "            .expect(\"acquire\")\n"
        "            .expect(\"configured\");\n"
        "        let mut state = ThreadState {\n"
        "            last_terminal_turn_id: Some(\"turn-1\".to_string()),\n"
        "            ..Default::default()\n"
        "        };\n"
        "        state.bind_turn_resource_permit(\"turn-1\", permit);\n"
        "        assert!(state.turn_resource_permits.is_empty());\n"
        "        assert!(governor.try_acquire_turn().expect(\"released\").is_some());\n"
        "    }\n\n"
        "    #[test]\n"
        "    fn listener_clear_releases_bound_turn_permit() {\n"
        "        let governor = crate::runtime_resource_governor::RuntimeResourceGovernor::new(\n"
        "            std::num::NonZeroUsize::new(1),\n"
        "            None,\n"
        "        );\n"
        "        let permit = governor\n"
        "            .try_acquire_turn()\n"
        "            .expect(\"acquire\")\n"
        "            .expect(\"configured\");\n"
        "        let mut state = ThreadState::default();\n"
        "        state.bind_turn_resource_permit(\"turn-1\", permit);\n"
        "        assert!(governor.try_acquire_turn().is_err());\n"
        "        state.clear_listener();\n"
        "        assert!(governor.try_acquire_turn().expect(\"released\").is_some());\n"
        "    }\n\n"
        "    #[test]\n"
        "    fn note_thread_settings_reports_only_effective_changes() {\n",
    )


def patch_command_exec() -> None:
    relative = "codex-rs/app-server/src/command_exec.rs"
    replace(
        relative,
        "use crate::outgoing_message::OutgoingMessageSender;\n",
        "use crate::outgoing_message::OutgoingMessageSender;\n"
        "use crate::runtime_resource_governor::RuntimeResourceGovernor;\n"
        "use crate::runtime_resource_governor::RuntimeResourcePermit;\n",
    )
    replace(
        relative,
        "    next_generated_process_id: Arc<AtomicI64>,\n",
        "    next_generated_process_id: Arc<AtomicI64>,\n"
        "    resource_governor: RuntimeResourceGovernor,\n",
    )
    replace(
        relative,
        "impl Default for CommandExecManager {\n    fn default() -> Self {\n        Self {\n            sessions: Arc::new(Mutex::new(HashMap::new())),\n            next_generated_process_id: Arc::new(AtomicI64::new(1)),\n        }\n    }\n}\n",
        "impl Default for CommandExecManager {\n"
        "    fn default() -> Self {\n"
        "        Self::new(RuntimeResourceGovernor::default())\n"
        "    }\n"
        "}\n",
    )
    replace(
        relative,
        "impl CommandExecManager {\n    pub(crate) async fn start(\n",
        "impl CommandExecManager {\n"
        "    pub(crate) fn new(resource_governor: RuntimeResourceGovernor) -> Self {\n"
        "        Self {\n"
        "            sessions: Arc::new(Mutex::new(HashMap::new())),\n"
        "            next_generated_process_id: Arc::new(AtomicI64::new(1)),\n"
        "            resource_governor,\n"
        "        }\n"
        "    }\n\n"
        "    pub(crate) async fn start(\n",
    )
    replace(
        relative,
        "        if process_id.is_none() && (tty || stream_stdin || stream_stdout_stderr) {\n",
        "        let tool_process_permit = self.resource_governor.try_acquire_tool_process()?;\n"
        "        if process_id.is_none() && (tty || stream_stdin || stream_stdout_stderr) {\n",
    )
    replace(
        relative,
        "            tokio::spawn(async move {\n                let _started_network_proxy = started_network_proxy;\n",
        "            tokio::spawn(async move {\n"
        "                let _tool_process_permit = tool_process_permit;\n"
        "                let _started_network_proxy = started_network_proxy;\n",
    )
    replace(
        relative,
        "    output_bytes_cap: Option<usize>,\n}\n\nstruct SpawnProcessOutputParams",
        "    output_bytes_cap: Option<usize>,\n"
        "    tool_process_permit: Option<RuntimeResourcePermit>,\n"
        "}\n\nstruct SpawnProcessOutputParams",
    )
    replace(
        relative,
        "                output_bytes_cap,\n            })\n",
        "                output_bytes_cap,\n"
        "                tool_process_permit,\n"
        "            })\n",
        count=1,
    )
    replace(
        relative,
        "        output_bytes_cap,\n    } = params;\n",
        "        output_bytes_cap,\n"
        "        tool_process_permit: _tool_process_permit,\n"
        "    } = params;\n",
        count=1,
    )


def patch_command_exec_processor() -> None:
    relative = "codex-rs/app-server/src/request_processors/command_exec_processor.rs"
    replace(
        relative,
        "        environment_manager: Arc<EnvironmentManager>,\n    ) -> Self {\n",
        "        environment_manager: Arc<EnvironmentManager>,\n"
        "        resource_governor: RuntimeResourceGovernor,\n"
        "    ) -> Self {\n",
    )
    replace(
        relative,
        "            command_exec_manager: CommandExecManager::default(),\n",
        "            command_exec_manager: CommandExecManager::new(resource_governor),\n",
    )


def patch_process_exec() -> None:
    relative = "codex-rs/app-server/src/request_processors/process_exec_processor.rs"
    replace(
        relative,
        "use crate::outgoing_message::OutgoingMessageSender;\n",
        "use crate::outgoing_message::OutgoingMessageSender;\n"
        "use crate::runtime_resource_governor::RuntimeResourceGovernor;\n"
        "use crate::runtime_resource_governor::RuntimeResourcePermit;\n",
    )
    replace(
        relative,
        "        environment_manager: Arc<EnvironmentManager>,\n    ) -> Self {\n",
        "        environment_manager: Arc<EnvironmentManager>,\n"
        "        resource_governor: RuntimeResourceGovernor,\n"
        "    ) -> Self {\n",
    )
    replace(
        relative,
        "            process_exec_manager: ProcessExecManager::default(),\n",
        "            process_exec_manager: ProcessExecManager::new(resource_governor),\n",
    )
    replace(
        relative,
        "#[derive(Clone, Default)]\nstruct ProcessExecManager {\n    sessions: Arc<Mutex<HashMap<ConnectionProcessHandle, ProcessSession>>>,\n}\n",
        "#[derive(Clone)]\n"
        "struct ProcessExecManager {\n"
        "    sessions: Arc<Mutex<HashMap<ConnectionProcessHandle, ProcessSession>>>,\n"
        "    resource_governor: RuntimeResourceGovernor,\n"
        "}\n\n"
        "impl Default for ProcessExecManager {\n"
        "    fn default() -> Self {\n"
        "        Self::new(RuntimeResourceGovernor::default())\n"
        "    }\n"
        "}\n",
    )
    replace(
        relative,
        "impl ProcessExecManager {\n    async fn start",
        "impl ProcessExecManager {\n"
        "    fn new(resource_governor: RuntimeResourceGovernor) -> Self {\n"
        "        Self {\n"
        "            sessions: Arc::new(Mutex::new(HashMap::new())),\n"
        "            resource_governor,\n"
        "        }\n"
        "    }\n\n"
        "    async fn start",
    )
    replace(
        relative,
        "        let (program, args) = command\n",
        "        let tool_process_permit = self.resource_governor.try_acquire_tool_process()?;\n\n"
        "        let (program, args) = command\n",
    )
    replace(
        relative,
        "    output_bytes_cap: Option<usize>,\n}\n\nstruct SpawnProcessOutputParams",
        "    output_bytes_cap: Option<usize>,\n"
        "    tool_process_permit: Option<RuntimeResourcePermit>,\n"
        "}\n\nstruct SpawnProcessOutputParams",
    )
    replace(
        relative,
        "                output_bytes_cap,\n            })\n",
        "                output_bytes_cap,\n"
        "                tool_process_permit,\n"
        "            })\n",
        count=1,
    )
    replace(
        relative,
        "        output_bytes_cap,\n    } = params;\n",
        "        output_bytes_cap,\n"
        "        tool_process_permit: _tool_process_permit,\n"
        "    } = params;\n",
        count=1,
    )


def patch_agentd() -> None:
    relative = "codex-rs/hepta-agentd/src/app_runtime.rs"
    replace(
        relative,
        "    let turn_queue_capacity = NonZeroUsize::new(turn_queue_capacity).ok_or_else(|| {\n        std::io::Error::other(\"agent manifest contains a zero turn queue capacity\")\n    })?;\n",
        "    let turn_queue_capacity = NonZeroUsize::new(turn_queue_capacity).ok_or_else(|| {\n"
        "        std::io::Error::other(\"agent manifest contains a zero turn queue capacity\")\n"
        "    })?;\n"
        "    let max_concurrent_turns =\n"
        "        NonZeroUsize::new(usize::from(identity.resources.max_concurrent_turns)).ok_or_else(\n"
        "            || std::io::Error::other(\"agent manifest contains zero concurrent turns\"),\n"
        "        )?;\n"
        "    let max_tool_processes =\n"
        "        NonZeroUsize::new(usize::from(identity.resources.max_tool_processes)).ok_or_else(\n"
        "            || std::io::Error::other(\"agent manifest contains zero tool processes\"),\n"
        "        )?;\n",
    )
    replace(
        relative,
        "        turn_queue_capacity: Some(turn_queue_capacity),\n",
        "        turn_queue_capacity: Some(turn_queue_capacity),\n"
        "        max_concurrent_turns: Some(max_concurrent_turns),\n"
        "        max_tool_processes: Some(max_tool_processes),\n",
    )
    replace(
        relative,
        "        assert_eq!(\n            Some(37),\n            options.turn_queue_capacity.map(std::num::NonZeroUsize::get)\n        );\n",
        "        assert_eq!(\n"
        "            Some(37),\n"
        "            options.turn_queue_capacity.map(std::num::NonZeroUsize::get)\n"
        "        );\n"
        "        assert_eq!(\n"
        "            Some(usize::from(identity.resources.max_concurrent_turns)),\n"
        "            options.max_concurrent_turns.map(std::num::NonZeroUsize::get)\n"
        "        );\n"
        "        assert_eq!(\n"
        "            Some(usize::from(identity.resources.max_tool_processes)),\n"
        "            options.max_tool_processes.map(std::num::NonZeroUsize::get)\n"
        "        );\n",
    )


def main() -> int:
    add_governor_module()
    patch_lib()
    patch_message_processor()
    patch_request_processors_imports()
    patch_turn_processor()
    patch_thread_state()
    patch_command_exec()
    patch_command_exec_processor()
    patch_process_exec()
    patch_agentd()
    print("PASS_HEPTA_RESOURCE_GOVERNOR_SOURCE")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
