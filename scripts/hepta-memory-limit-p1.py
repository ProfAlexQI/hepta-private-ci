#!/usr/bin/env python3
"""Wire hard memory containment and resident-memory fencing into Agentd."""

from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKSPACE = ROOT / "codex-rs/Cargo.toml"
AGENTD_MANIFEST = ROOT / "codex-rs/hepta-agentd/Cargo.toml"
AGENTD_LIB = ROOT / "codex-rs/hepta-agentd/src/lib.rs"
AGENTD_ERROR = ROOT / "codex-rs/hepta-agentd/src/error.rs"
AGENTD_RUNTIME = ROOT / "codex-rs/hepta-agentd/src/runtime.rs"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_MEMORY_LIMIT_P1: {message}")


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        fail(f"{label}: expected one marker {old!r}, found {count}")
    return text.replace(old, new, 1)


def patch_workspace() -> None:
    source = WORKSPACE.read_text(encoding="utf-8")
    if 'windows-sys = { version = "0.61.2"' not in source:
        source = replace_once(
            source,
            'libc = "0.2.182"\n',
            'libc = "0.2.182"\n'
            'windows-sys = { version = "0.61.2", features = [\n'
            '    "Win32_Foundation",\n'
            '    "Win32_System_JobObjects",\n'
            '    "Win32_System_ProcessStatus",\n'
            '    "Win32_System_Threading",\n'
            '] }\n',
            "windows-sys workspace dependency",
        )
    WORKSPACE.write_text(source, encoding="utf-8")


def patch_agentd_manifest() -> None:
    source = AGENTD_MANIFEST.read_text(encoding="utf-8")
    if "[target.'cfg(unix)'.dependencies]" not in source:
        source = replace_once(
            source,
            "tokio-util = { workspace = true }\n\n[dev-dependencies]\n",
            "tokio-util = { workspace = true }\n\n"
            "[target.'cfg(unix)'.dependencies]\n"
            "libc = { workspace = true }\n\n"
            "[target.'cfg(windows)'.dependencies]\n"
            "windows-sys = { workspace = true }\n\n"
            "[dev-dependencies]\n",
            "platform memory dependencies",
        )
    AGENTD_MANIFEST.write_text(source, encoding="utf-8")


def patch_agentd_lib() -> None:
    source = AGENTD_LIB.read_text(encoding="utf-8")
    if "mod resource_budget;" not in source:
        source = replace_once(
            source,
            "mod qualification_writer;\nmod runtime;\n",
            "mod qualification_writer;\nmod resource_budget;\nmod runtime;\n",
            "resource budget module",
        )
    AGENTD_LIB.write_text(source, encoding="utf-8")


def patch_agentd_error() -> None:
    source = AGENTD_ERROR.read_text(encoding="utf-8")
    if "ResourceLimitExceeded" in source:
        return
    source = replace_once(
        source,
        "    #[error(\"agentd generation fenced: {0}\")]\n"
        "    GenerationFenced(String),\n",
        "    #[error(\"agentd generation fenced: {0}\")]\n"
        "    GenerationFenced(String),\n"
        "    #[error(\"cannot install agent resource limit: {0}\")]\n"
        "    ResourceLimitInstallation(String),\n"
        "    #[error(\"cannot observe agent resource use: {0}\")]\n"
        "    ResourceLimitObservation(String),\n"
        "    #[error(\"agent resource {resource} exceeded: observed {observed}, limit {limit}\")]\n"
        "    ResourceLimitExceeded {\n"
        "        resource: &'static str,\n"
        "        observed: u64,\n"
        "        limit: u64,\n"
        "    },\n",
        "resource limit errors",
    )
    AGENTD_ERROR.write_text(source, encoding="utf-8")


def patch_runtime() -> None:
    source = AGENTD_RUNTIME.read_text(encoding="utf-8")
    if "memory_budget.install_hard_limit" in source:
        return
    source = replace_once(
        source,
        "use crate::composition::AgentRuntimeComposition;\n"
        "use crate::composition::AgentRuntimeParts;\n",
        "use crate::composition::AgentRuntimeComposition;\n"
        "use crate::composition::AgentRuntimeParts;\n"
        "use crate::resource_budget::MemoryBudget;\n"
        "use crate::resource_budget::resident_memory_bytes;\n",
        "resource budget imports",
    )
    source = replace_once(
        source,
        "pub async fn run(config: AgentdConfig, arg0_paths: Arg0DispatchPaths) -> Result<(), AgentdError> {\n"
        "    let AgentRuntimeParts {\n",
        "pub async fn run(config: AgentdConfig, arg0_paths: Arg0DispatchPaths) -> Result<(), AgentdError> {\n"
        "    let memory_budget = MemoryBudget::from_mib(u64::from(\n"
        "        config.identity().resources.memory_limit_mib,\n"
        "    ))\n"
        "    .map_err(|error| AgentdError::Invalid(error.to_string()))?;\n"
        "    let _installed_memory_limit = memory_budget\n"
        "        .install_hard_limit()\n"
        "        .map_err(|error| AgentdError::ResourceLimitInstallation(error.to_string()))?;\n"
        "    let AgentRuntimeParts {\n",
        "memory limit installation",
    )
    source = replace_once(
        source,
        "    let mut monitor_task = tokio::spawn(monitor_runtime(Arc::clone(&state)));\n",
        "    let mut monitor_task = tokio::spawn(monitor_runtime(\n"
        "        Arc::clone(&state),\n"
        "        memory_budget,\n"
        "    ));\n",
        "memory monitor construction",
    )
    source = replace_once(
        source,
        "async fn monitor_runtime(state: Arc<AgentdState>) -> Result<(), AgentdError> {\n"
        "    let mut app_server_ready = false;\n"
        "    loop {\n",
        "async fn monitor_runtime(\n"
        "    state: Arc<AgentdState>,\n"
        "    memory_budget: MemoryBudget,\n"
        ") -> Result<(), AgentdError> {\n"
        "    let mut app_server_ready = false;\n"
        "    loop {\n"
        "        let observed_memory = resident_memory_bytes().map_err(|error| {\n"
        "            AgentdError::ResourceLimitObservation(error.to_string())\n"
        "        })?;\n"
        "        if let Err(exceeded) = memory_budget.check(observed_memory) {\n"
        "            state.mark_fenced();\n"
        "            return Err(AgentdError::ResourceLimitExceeded {\n"
        "                resource: \"memory_limit_mib\",\n"
        "                observed: exceeded.observed_bytes,\n"
        "                limit: exceeded.limit_bytes,\n"
        "            });\n"
        "        }\n",
        "memory monitor check",
    )
    AGENTD_RUNTIME.write_text(source, encoding="utf-8")


def main() -> int:
    for path in (WORKSPACE, AGENTD_MANIFEST, AGENTD_LIB, AGENTD_ERROR, AGENTD_RUNTIME):
        if not path.is_file():
            fail(f"required file is missing: {path.relative_to(ROOT)}")
    patch_workspace()
    patch_agentd_manifest()
    patch_agentd_lib()
    patch_agentd_error()
    patch_runtime()
    print("PASS_HEPTA_MEMORY_LIMIT_P1_SOURCE")
    return 0


if __name__ == "__main__":
    sys.exit(main())
