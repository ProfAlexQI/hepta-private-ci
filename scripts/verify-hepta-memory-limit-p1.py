#!/usr/bin/env python3
"""Verify hard memory containment and runtime RSS fencing are product-bound."""

from __future__ import annotations

import pathlib
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKSPACE = ROOT / "codex-rs/Cargo.toml"
AGENTD_MANIFEST = ROOT / "codex-rs/hepta-agentd/Cargo.toml"
AGENTD_LIB = ROOT / "codex-rs/hepta-agentd/src/lib.rs"
AGENTD_ERROR = ROOT / "codex-rs/hepta-agentd/src/error.rs"
AGENTD_RUNTIME = ROOT / "codex-rs/hepta-agentd/src/runtime.rs"
RESOURCE = ROOT / "codex-rs/hepta-agentd/src/resource_budget.rs"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_MEMORY_LIMIT_P1: {message}")


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")


def require(path: pathlib.Path, markers: tuple[str, ...]) -> str:
    source = read(path)
    for marker in markers:
        if marker not in source:
            fail(f"{path.relative_to(ROOT)} is missing {marker!r}")
    return source


def main() -> int:
    workspace = tomllib.loads(read(WORKSPACE))
    windows = workspace.get("workspace", {}).get("dependencies", {}).get("windows-sys")
    if not isinstance(windows, dict) or windows.get("version") != "0.61.2":
        fail("windows-sys workspace binding is missing or drifted")
    expected_features = {
        "Win32_Foundation",
        "Win32_System_JobObjects",
        "Win32_System_ProcessStatus",
        "Win32_System_Threading",
    }
    if set(windows.get("features", [])) != expected_features:
        fail("windows-sys feature surface drifted")

    manifest = tomllib.loads(read(AGENTD_MANIFEST))
    targets = manifest.get("target", {})
    if targets.get("cfg(unix)", {}).get("dependencies", {}).get("libc") != {"workspace": True}:
        fail("Agentd Unix libc dependency is missing")
    if targets.get("cfg(windows)", {}).get("dependencies", {}).get("windows-sys") != {
        "workspace": True
    }:
        fail("Agentd Windows API dependency is missing")

    require(AGENTD_LIB, ("mod resource_budget;",))
    require(
        AGENTD_ERROR,
        (
            "ResourceLimitInstallation(String)",
            "ResourceLimitObservation(String)",
            "ResourceLimitExceeded {",
            "resource: &'static str",
        ),
    )
    resource = require(
        RESOURCE,
        (
            "pub(crate) struct MemoryBudget",
            "pub(crate) fn install_hard_limit",
            "libc::RLIMIT_AS",
            "libc::setrlimit",
            "JOB_OBJECT_LIMIT_PROCESS_MEMORY",
            "AssignProcessToJobObject",
            "K32GetProcessMemoryInfo",
            'starts_with("VmRSS:")',
            "resident_memory_check_fails_only_above_limit",
        ),
    )
    if "JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE" in resource:
        fail("normal Agentd shutdown must not kill itself through job-handle drop")
    if "memory_limit_mib must be non-zero" not in resource:
        fail("zero memory budget is not rejected")

    runtime = require(
        AGENTD_RUNTIME,
        (
            "config.identity().resources.memory_limit_mib",
            "memory_budget.install_hard_limit()",
            "monitor_runtime(\n        Arc::clone(&state),\n        memory_budget,",
            "resident_memory_bytes()",
            "state.mark_fenced();",
            'resource: "memory_limit_mib"',
        ),
    )
    install_index = runtime.find("memory_budget.install_hard_limit()")
    composition_index = runtime.find("AgentRuntimeComposition::open(config)")
    if install_index < 0 or composition_index < 0 or install_index > composition_index:
        fail("hard memory limit is not installed before product composition")

    print("PASS_HEPTA_MEMORY_LIMIT_P1")
    return 0


if __name__ == "__main__":
    sys.exit(main())
