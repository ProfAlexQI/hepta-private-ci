#!/usr/bin/env python3
"""Verify one shared tool-process budget covers both App Server exec APIs."""

from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
APP_LIB = ROOT / "codex-rs/app-server/src/lib.rs"
MESSAGE = ROOT / "codex-rs/app-server/src/message_processor.rs"
BUDGET = ROOT / "codex-rs/app-server/src/tool_process_budget.rs"
COMMAND_PROCESSOR = ROOT / "codex-rs/app-server/src/request_processors/command_exec_processor.rs"
COMMAND_MANAGER = ROOT / "codex-rs/app-server/src/command_exec.rs"
PROCESS_PROCESSOR = ROOT / "codex-rs/app-server/src/request_processors/process_exec_processor.rs"
AGENTD_APP = ROOT / "codex-rs/hepta-agentd/src/app_runtime.rs"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_MAX_TOOL_PROCESSES_P1: {message}")


def require(path: pathlib.Path, markers: tuple[str, ...]) -> str:
    try:
        source = path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")
    for marker in markers:
        if marker not in source:
            fail(f"{path.relative_to(ROOT)} is missing {marker!r}")
    return source


def main() -> int:
    require(
        APP_LIB,
        (
            "mod tool_process_budget;",
            "pub max_tool_processes: Option<NonZeroUsize>",
            'field("max_tool_processes", &self.max_tool_processes)',
            "max_tool_processes: None",
            "max_tool_processes: runtime_options.max_tool_processes",
        ),
    )
    message = require(
        MESSAGE,
        (
            "pub(crate) max_tool_processes: Option<NonZeroUsize>",
            "let tool_process_budget = ToolProcessBudget::new(max_tool_processes);",
            "tool_process_budget.clone(),",
            "tool_process_budget,",
        ),
    )
    if message.count("ToolProcessBudget::new(max_tool_processes)") != 1:
        fail("MessageProcessor must construct exactly one shared tool-process budget")

    budget = require(
        BUDGET,
        (
            "pub(crate) struct ToolProcessBudget",
            "capacity: Option<Arc<Semaphore>>",
            "pub(crate) fn try_reserve(",
            ".try_acquire_owned()",
            "cloned_budgets_share_one_process_capacity",
        ),
    )
    if "acquire_owned().await" in budget:
        fail("tool-process admission must fail fast rather than wait indefinitely")

    command_processor = require(
        COMMAND_PROCESSOR,
        (
            "tool_process_budget: ToolProcessBudget",
            ".try_reserve()",
            ".map_err(tool_process_limit_error)?",
            '"resource": "max_tool_processes"',
            "process_permit,",
        ),
    )
    if command_processor.find(".try_reserve()") > command_processor.find(".start(StartCommandExecParams"):
        fail("command/exec reserves capacity after crossing the spawn boundary")

    command_manager = require(
        COMMAND_MANAGER,
        (
            "pub(crate) process_permit: Option<OwnedSemaphorePermit>",
            "process_permit: Option<OwnedSemaphorePermit>",
            "let _process_permit = process_permit;",
            "process_permit: _process_permit,",
        ),
    )
    if command_manager.count("process_permit: Option<OwnedSemaphorePermit>") != 2:
        fail("command/exec permit must be present in start and run envelopes")

    process_processor = require(
        PROCESS_PROCESSOR,
        (
            "tool_process_budget: ToolProcessBudget",
            ".try_reserve()",
            ".map_err(tool_process_limit_error)?",
            '"resource": "max_tool_processes"',
            "process_permit: Option<OwnedSemaphorePermit>",
            "process_permit: _process_permit,",
        ),
    )
    if process_processor.find(".try_reserve()") > process_processor.find(".start(StartProcessParams"):
        fail("process/spawn reserves capacity after crossing the spawn boundary")
    if process_processor.count("process_permit: Option<OwnedSemaphorePermit>") != 2:
        fail("process/spawn permit must be present in start and run envelopes")

    require(
        AGENTD_APP,
        (
            "identity.resources.max_tool_processes",
            "max_tool_processes: Some(max_tool_processes)",
            "options.max_tool_processes",
        ),
    )

    print("PASS_HEPTA_MAX_TOOL_PROCESSES_P1")
    return 0


if __name__ == "__main__":
    sys.exit(main())
