#!/usr/bin/env python3
"""Apply the bounded P0.7a source-closure repair to an exact candidate tree."""

from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected exactly one {label}, found {count}")
    return text.replace(old, new, 1)


def repair_fleet_publication() -> None:
    path = Path("codex-rs/hepta-fleet/src/runtime_bootstrap_registry.rs")
    text = path.read_text(encoding="utf-8")
    replacements = (
        (
            "    let linked = std::fs::symlink_metadata(path)?;\n",
            "    let held_linked = file.metadata()?;\n"
            "    validate_registry_metadata(&temp, &held_linked, &parent_metadata, 2)?;\n"
            "    let linked = std::fs::symlink_metadata(path)?;\n",
            "post-link handle refresh",
        ),
        (
            "    if metadata_identity(&held) != metadata_identity(&linked) {\n",
            "    if metadata_identity(&held_linked) != metadata_identity(&linked) {\n",
            "post-link identity comparison",
        ),
        (
            "    let published = std::fs::symlink_metadata(path)?;\n",
            "    let held_published = file.metadata()?;\n"
            "    validate_registry_metadata(path, &held_published, &parent_metadata, 1)?;\n"
            "    let published = std::fs::symlink_metadata(path)?;\n",
            "post-unlink handle refresh",
        ),
        (
            "    if metadata_identity(&held) != metadata_identity(&published) {\n",
            "    if metadata_identity(&held_published) != metadata_identity(&published) {\n",
            "post-unlink identity comparison",
        ),
    )
    for old, new, label in replacements:
        text = replace_once(text, old, new, label)
    path.write_text(text, encoding="utf-8")


def restore_agentd_module_root() -> None:
    path = Path("codex-rs/hepta-agentd/src/lib.rs")
    path.write_text(
        '''#![forbid(unsafe_code)]

//! One-process-per-workspace Hepta agent host.
//!
//! `agentd` binds exactly one fleet `AgentId`, embeds the existing Codex App
//! Server execution path, and exposes a small local lifecycle/control socket.
//! It does not implement a second runtime kernel or a fleet-wide message bus.

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
mod runtime_bootstrap;
mod runtime_profile;
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
pub use codex_hepta_agent_protocol::MAX_FEDERATION_CONTROL_LIST;
pub use codex_hepta_agent_protocol::MemoryFederationCapabilityId;
pub use codex_hepta_agent_protocol::MemoryFederationCapabilitySnapshot;
pub use codex_hepta_agent_protocol::MemoryFederationCapabilityState;
pub use codex_hepta_agent_protocol::MemoryFederationScopeKind;
pub use codex_hepta_agent_protocol::SessionIngress;
pub use codex_hepta_agent_protocol::SessionTransport;
pub use codex_hepta_automation::AutomationSchedule;
pub use codex_hepta_automation::AutomationTask;
pub use codex_hepta_automation::AutomationTaskDraft;
pub use codex_hepta_automation::AutomationTaskId;
pub use config::AgentdConfig;
pub use config::AgentdIdentity;
pub use config::HEPTA_AGENT_GENERATION_ENV;
pub use config::HEPTA_AGENT_HOME_ENV;
pub use config::HEPTA_AGENT_ID_ENV;
pub use config::HEPTA_AGENT_RUN_ROOT_ENV;
pub use error::AgentdError;
pub use production_writer_host::AgentdProductionWriterHost;
pub use runtime::run;
pub use runtime_bootstrap::RuntimeBootstrapAdmission;

use control::AgentdControlServer;
use event_buffer::EventBuffer;
use state::AgentdState;
''',
        encoding="utf-8",
    )


if __name__ == "__main__":
    repair_fleet_publication()
    restore_agentd_module_root()
