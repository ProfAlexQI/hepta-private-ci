#!/usr/bin/env python3
"""Wire the Agent tool-process budget through both App Server exec APIs."""

from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
APP_LIB = ROOT / "codex-rs/app-server/src/lib.rs"
MESSAGE = ROOT / "codex-rs/app-server/src/message_processor.rs"
COMMAND_PROCESSOR = ROOT / "codex-rs/app-server/src/request_processors/command_exec_processor.rs"
COMMAND_MANAGER = ROOT / "codex-rs/app-server/src/command_exec.rs"
PROCESS_PROCESSOR = ROOT / "codex-rs/app-server/src/request_processors/process_exec_processor.rs"
AGENTD_APP = ROOT / "codex-rs/hepta-agentd/src/app_runtime.rs"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_MAX_TOOL_PROCESSES_P1: {message}")


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        fail(f"{label}: expected one marker {old!r}, found {count}")
    return text.replace(old, new, 1)


def patch_app_lib() -> None:
    source = APP_LIB.read_text(encoding="utf-8")
    if "pub max_tool_processes: Option<NonZeroUsize>" in source:
        return
    source = replace_once(
        source,
        "mod thread_status;\nmod transport;\n",
        "mod thread_status;\nmod tool_process_budget;\nmod transport;\n",
        "tool process budget module",
    )
    source = replace_once(
        source,
        "    pub max_concurrent_turns: Option<NonZeroUsize>,\n",
        "    pub max_concurrent_turns: Option<NonZeroUsize>,\n"
        "    /// Optional process-wide cap shared by command/exec and process/spawn.\n"
        "    pub max_tool_processes: Option<NonZeroUsize>,\n",
        "AppServerRuntimeOptions field",
    )
    source = replace_once(
        source,
        '            .field("max_concurrent_turns", &self.max_concurrent_turns)\n',
        '            .field("max_concurrent_turns", &self.max_concurrent_turns)\n'
        '            .field("max_tool_processes", &self.max_tool_processes)\n',
        "AppServerRuntimeOptions Debug",
    )
    source = replace_once(
        source,
        "            && self.max_concurrent_turns == other.max_concurrent_turns\n",
        "            && self.max_concurrent_turns == other.max_concurrent_turns\n"
        "            && self.max_tool_processes == other.max_tool_processes\n",
        "AppServerRuntimeOptions PartialEq",
    )
    source = replace_once(
        source,
        "            max_concurrent_turns: None,\n",
        "            max_concurrent_turns: None,\n"
        "            max_tool_processes: None,\n",
        "AppServerRuntimeOptions Default",
    )
    source = replace_once(
        source,
        "            max_concurrent_turns: runtime_options.max_concurrent_turns,\n",
        "            max_concurrent_turns: runtime_options.max_concurrent_turns,\n"
        "            max_tool_processes: runtime_options.max_tool_processes,\n",
        "MessageProcessorArgs construction",
    )
    APP_LIB.write_text(source, encoding="utf-8")


def patch_message_processor() -> None:
    source = MESSAGE.read_text(encoding="utf-8")
    if "pub(crate) max_tool_processes: Option<NonZeroUsize>" in source:
        return
    source = replace_once(
        source,
        "use crate::thread_state::ConnectionCapabilities;\n",
        "use crate::thread_state::ConnectionCapabilities;\n"
        "use crate::tool_process_budget::ToolProcessBudget;\n",
        "MessageProcessor budget import",
    )
    source = replace_once(
        source,
        "    pub(crate) max_concurrent_turns: Option<NonZeroUsize>,\n",
        "    pub(crate) max_concurrent_turns: Option<NonZeroUsize>,\n"
        "    pub(crate) max_tool_processes: Option<NonZeroUsize>,\n",
        "MessageProcessorArgs field",
    )
    source = replace_once(
        source,
        "            max_concurrent_turns,\n            hepta_cognitive_runtime,\n",
        "            max_concurrent_turns,\n"
        "            max_tool_processes,\n"
        "            hepta_cognitive_runtime,\n",
        "MessageProcessorArgs destructure",
    )
    source = replace_once(
        source,
        "        let pending_thread_unloads = Arc::new(Mutex::new(HashSet::new()));\n",
        "        let pending_thread_unloads = Arc::new(Mutex::new(HashSet::new()));\n"
        "        let tool_process_budget = ToolProcessBudget::new(max_tool_processes);\n",
        "shared process budget construction",
    )
    source = replace_once(
        source,
        "            config_manager.clone(),\n"
        "            Arc::clone(&environment_manager_for_requests),\n"
        "        );\n"
        "        let process_exec_processor = ProcessExecRequestProcessor::new(\n"
        "            outgoing.clone(),\n"
        "            Arc::clone(&environment_manager_for_requests),\n"
        "        );\n",
        "            config_manager.clone(),\n"
        "            Arc::clone(&environment_manager_for_requests),\n"
        "            tool_process_budget.clone(),\n"
        "        );\n"
        "        let process_exec_processor = ProcessExecRequestProcessor::new(\n"
        "            outgoing.clone(),\n"
        "            Arc::clone(&environment_manager_for_requests),\n"
        "            tool_process_budget,\n"
        "        );\n",
        "exec processor budget injection",
    )
    MESSAGE.write_text(source, encoding="utf-8")


def patch_command_processor() -> None:
    source = COMMAND_PROCESSOR.read_text(encoding="utf-8")
    if "tool_process_budget: ToolProcessBudget" in source:
        return
    source = replace_once(
        source,
        "use codex_protocol::shell_environment::is_non_inheritable_env_var;\n",
        "use codex_protocol::shell_environment::is_non_inheritable_env_var;\n\n"
        "use crate::tool_process_budget::ToolProcessBudget;\n",
        "command processor budget import",
    )
    source = replace_once(
        source,
        "    command_exec_manager: CommandExecManager,\n",
        "    command_exec_manager: CommandExecManager,\n"
        "    tool_process_budget: ToolProcessBudget,\n",
        "command processor field",
    )
    source = replace_once(
        source,
        "        environment_manager: Arc<EnvironmentManager>,\n"
        "    ) -> Self {\n",
        "        environment_manager: Arc<EnvironmentManager>,\n"
        "        tool_process_budget: ToolProcessBudget,\n"
        "    ) -> Self {\n",
        "command processor constructor parameter",
    )
    source = replace_once(
        source,
        "            environment_manager,\n"
        "            command_exec_manager: CommandExecManager::default(),\n",
        "            environment_manager,\n"
        "            command_exec_manager: CommandExecManager::default(),\n"
        "            tool_process_budget,\n",
        "command processor constructor field",
    )
    source = replace_once(
        source,
        "        let started_network_proxy = match network_proxy_spec.as_ref() {\n",
        "        let process_permit = self\n"
        "            .tool_process_budget\n"
        "            .try_reserve()\n"
        "            .map_err(tool_process_limit_error)?;\n"
        "        let started_network_proxy = match network_proxy_spec.as_ref() {\n",
        "command process reservation",
    )
    source = replace_once(
        source,
        "                size,\n"
        "            })\n",
        "                size,\n"
        "                process_permit,\n"
        "            })\n",
        "command start permit",
    )
    source += """

fn tool_process_limit_error(limit: usize) -> JSONRPCErrorError {
    let mut error = invalid_request(format!(
        "Agent tool process limit of {limit} has been reached"
    ));
    error.data = Some(serde_json::json!({
        "resource": "max_tool_processes",
        "limit": limit,
    }));
    error
}
"""
    COMMAND_PROCESSOR.write_text(source, encoding="utf-8")


def patch_command_manager() -> None:
    source = COMMAND_MANAGER.read_text(encoding="utf-8")
    if "process_permit: Option<OwnedSemaphorePermit>" in source:
        return
    source = replace_once(
        source,
        "use tokio::sync::Mutex;\n",
        "use tokio::sync::Mutex;\n"
        "use tokio::sync::OwnedSemaphorePermit;\n",
        "command manager permit import",
    )
    source = replace_once(
        source,
        "    pub(crate) size: Option<TerminalSize>,\n"
        "}\n\n"
        "struct RunCommandParams {\n",
        "    pub(crate) size: Option<TerminalSize>,\n"
        "    pub(crate) process_permit: Option<OwnedSemaphorePermit>,\n"
        "}\n\n"
        "struct RunCommandParams {\n",
        "StartCommandExecParams permit",
    )
    source = replace_once(
        source,
        "    output_bytes_cap: Option<usize>,\n"
        "}\n\n"
        "struct SpawnProcessOutputParams {\n",
        "    output_bytes_cap: Option<usize>,\n"
        "    process_permit: Option<OwnedSemaphorePermit>,\n"
        "}\n\n"
        "struct SpawnProcessOutputParams {\n",
        "RunCommandParams permit",
    )
    source = replace_once(
        source,
        "            output_bytes_cap,\n"
        "            size,\n"
        "        } = params;\n",
        "            output_bytes_cap,\n"
        "            size,\n"
        "            process_permit,\n"
        "        } = params;\n",
        "command start destructure",
    )
    source = replace_once(
        source,
        "            tokio::spawn(async move {\n"
        "                let _started_network_proxy = started_network_proxy;\n",
        "            tokio::spawn(async move {\n"
        "                let _process_permit = process_permit;\n"
        "                let _started_network_proxy = started_network_proxy;\n",
        "Windows command permit lifetime",
    )
    source = replace_once(
        source,
        "                output_bytes_cap,\n"
        "            })\n",
        "                output_bytes_cap,\n"
        "                process_permit,\n"
        "            })\n",
        "regular command permit move",
    )
    source = replace_once(
        source,
        "        output_bytes_cap,\n"
        "    } = params;\n",
        "        output_bytes_cap,\n"
        "        process_permit: _process_permit,\n"
        "    } = params;\n",
        "run command permit lifetime",
    )
    COMMAND_MANAGER.write_text(source, encoding="utf-8")


def patch_process_processor() -> None:
    source = PROCESS_PROCESSOR.read_text(encoding="utf-8")
    if "tool_process_budget: ToolProcessBudget" in source:
        return
    source = replace_once(
        source,
        "use tokio::sync::Mutex;\n",
        "use tokio::sync::Mutex;\n"
        "use tokio::sync::OwnedSemaphorePermit;\n",
        "process permit import",
    )
    source = replace_once(
        source,
        "use crate::outgoing_message::OutgoingMessageSender;\n",
        "use crate::outgoing_message::OutgoingMessageSender;\n"
        "use crate::tool_process_budget::ToolProcessBudget;\n",
        "process budget import",
    )
    source = replace_once(
        source,
        "    process_exec_manager: ProcessExecManager,\n",
        "    process_exec_manager: ProcessExecManager,\n"
        "    tool_process_budget: ToolProcessBudget,\n",
        "process processor field",
    )
    source = replace_once(
        source,
        "        environment_manager: Arc<EnvironmentManager>,\n"
        "    ) -> Self {\n",
        "        environment_manager: Arc<EnvironmentManager>,\n"
        "        tool_process_budget: ToolProcessBudget,\n"
        "    ) -> Self {\n",
        "process processor constructor parameter",
    )
    source = replace_once(
        source,
        "            environment_manager,\n"
        "            process_exec_manager: ProcessExecManager::default(),\n",
        "            environment_manager,\n"
        "            process_exec_manager: ProcessExecManager::default(),\n"
        "            tool_process_budget,\n",
        "process processor constructor field",
    )
    source = replace_once(
        source,
        "        self.process_exec_manager\n"
        "            .start(StartProcessParams {\n",
        "        let process_permit = self\n"
        "            .tool_process_budget\n"
        "            .try_reserve()\n"
        "            .map_err(tool_process_limit_error)?;\n\n"
        "        self.process_exec_manager\n"
        "            .start(StartProcessParams {\n",
        "process reservation",
    )
    source = replace_once(
        source,
        "                output_bytes_cap,\n"
        "                size,\n"
        "            })\n",
        "                output_bytes_cap,\n"
        "                size,\n"
        "                process_permit,\n"
        "            })\n",
        "process start permit",
    )
    source = replace_once(
        source,
        "    size: Option<TerminalSize>,\n"
        "}\n\n"
        "struct RunProcessParams {\n",
        "    size: Option<TerminalSize>,\n"
        "    process_permit: Option<OwnedSemaphorePermit>,\n"
        "}\n\n"
        "struct RunProcessParams {\n",
        "StartProcessParams permit",
    )
    source = replace_once(
        source,
        "    output_bytes_cap: Option<usize>,\n"
        "}\n\n"
        "struct SpawnProcessOutputParams {\n",
        "    output_bytes_cap: Option<usize>,\n"
        "    process_permit: Option<OwnedSemaphorePermit>,\n"
        "}\n\n"
        "struct SpawnProcessOutputParams {\n",
        "RunProcessParams permit",
    )
    source = replace_once(
        source,
        "            output_bytes_cap,\n"
        "            size,\n"
        "        } = params;\n",
        "            output_bytes_cap,\n"
        "            size,\n"
        "            process_permit,\n"
        "        } = params;\n",
        "process start destructure",
    )
    source = replace_once(
        source,
        "                output_bytes_cap,\n"
        "            })\n",
        "                output_bytes_cap,\n"
        "                process_permit,\n"
        "            })\n",
        "process permit move",
    )
    source = replace_once(
        source,
        "        output_bytes_cap,\n"
        "    } = params;\n",
        "        output_bytes_cap,\n"
        "        process_permit: _process_permit,\n"
        "    } = params;\n",
        "run process permit lifetime",
    )
    source += """

fn tool_process_limit_error(limit: usize) -> JSONRPCErrorError {
    let mut error = invalid_request(format!(
        "Agent tool process limit of {limit} has been reached"
    ));
    error.data = Some(serde_json::json!({
        "resource": "max_tool_processes",
        "limit": limit,
    }));
    error
}
"""
    PROCESS_PROCESSOR.write_text(source, encoding="utf-8")


def patch_agentd() -> None:
    source = AGENTD_APP.read_text(encoding="utf-8")
    if "max_tool_processes: Some(max_tool_processes)" in source:
        return
    source = replace_once(
        source,
        "    let max_concurrent_turns = NonZeroUsize::new(max_concurrent_turns).ok_or_else(|| {\n"
        "        std::io::Error::other(\"agent manifest contains zero max concurrent turns\")\n"
        "    })?;\n",
        "    let max_concurrent_turns = NonZeroUsize::new(max_concurrent_turns).ok_or_else(|| {\n"
        "        std::io::Error::other(\"agent manifest contains zero max concurrent turns\")\n"
        "    })?;\n"
        "    let max_tool_processes = usize::try_from(identity.resources.max_tool_processes)\n"
        "        .map_err(|_| {\n"
        "            std::io::Error::other(\"max tool processes does not fit this platform\")\n"
        "        })?;\n"
        "    let max_tool_processes = NonZeroUsize::new(max_tool_processes).ok_or_else(|| {\n"
        "        std::io::Error::other(\"agent manifest contains zero max tool processes\")\n"
        "    })?;\n",
        "Agentd tool process conversion",
    )
    source = replace_once(
        source,
        "        max_concurrent_turns: Some(max_concurrent_turns),\n",
        "        max_concurrent_turns: Some(max_concurrent_turns),\n"
        "        max_tool_processes: Some(max_tool_processes),\n",
        "Agentd tool process runtime option",
    )
    source = replace_once(
        source,
        "        assert_eq!(\n"
        "            Some(identity.resources.max_concurrent_turns as usize),\n"
        "            options\n"
        "                .max_concurrent_turns\n"
        "                .map(std::num::NonZeroUsize::get)\n"
        "        );\n",
        "        assert_eq!(\n"
        "            Some(identity.resources.max_concurrent_turns as usize),\n"
        "            options\n"
        "                .max_concurrent_turns\n"
        "                .map(std::num::NonZeroUsize::get)\n"
        "        );\n"
        "        assert_eq!(\n"
        "            Some(identity.resources.max_tool_processes as usize),\n"
        "            options\n"
        "                .max_tool_processes\n"
        "                .map(std::num::NonZeroUsize::get)\n"
        "        );\n",
        "Agentd tool process runtime test",
    )
    AGENTD_APP.write_text(source, encoding="utf-8")


def main() -> int:
    for path in (
        APP_LIB,
        MESSAGE,
        COMMAND_PROCESSOR,
        COMMAND_MANAGER,
        PROCESS_PROCESSOR,
        AGENTD_APP,
    ):
        if not path.is_file():
            fail(f"required file is missing: {path.relative_to(ROOT)}")
    patch_app_lib()
    patch_message_processor()
    patch_command_processor()
    patch_command_manager()
    patch_process_processor()
    patch_agentd()
    print("PASS_HEPTA_MAX_TOOL_PROCESSES_P1_SOURCE")
    return 0


if __name__ == "__main__":
    sys.exit(main())
