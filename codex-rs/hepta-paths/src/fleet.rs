//! Typed, side-effect-free path geometry for a Hepta agent fleet.

use std::env;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use codex_hepta_contracts::AgentId;

use crate::validate_absolute_non_root;

pub const HEPTA_FLEET_ROOT_ENV: &str = "HEPTA_FLEET_ROOT";
const FLEET_DIRECTORY_NAME: &str = "fleet-v1";

/// A normalized, absolute, non-root path containing fleet-owned metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeptaFleetRoot(PathBuf);

impl HeptaFleetRoot {
    pub fn parse(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        validate_absolute_non_root(&path)
            .with_context(|| format!("validate {HEPTA_FLEET_ROOT_ENV}"))?;
        Ok(Self(path))
    }

    pub fn from_env() -> Result<Self> {
        if let Some(path) = env::var_os(HEPTA_FLEET_ROOT_ENV) {
            return Self::parse(path);
        }
        let home =
            env::var_os("HOME").context("HOME is required when HEPTA_FLEET_ROOT is unset")?;
        Self::production_default(Path::new(&home))
    }

    pub fn production_default(home: &Path) -> Result<Self> {
        validate_absolute_non_root(home).context("validate home directory")?;
        Self::parse(
            home.join(".local/share/hepta-vnext")
                .join(FLEET_DIRECTORY_NAME),
        )
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn layout(&self) -> HeptaFleetLayout {
        HeptaFleetLayout::new(self.clone())
    }
}

impl AsRef<Path> for HeptaFleetRoot {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl fmt::Display for HeptaFleetRoot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display().fmt(formatter)
    }
}

/// Fleet-wide paths. Only the future supervisor may mutate the shared state
/// and run roots; independently supervised agents own their per-agent roots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaFleetLayout {
    fleet_root: HeptaFleetRoot,
    fleet_config: PathBuf,
    state_root: PathBuf,
    supervisor_database: PathBuf,
    run_root: PathBuf,
    supervisor_socket: PathBuf,
    supervisor_lock: PathBuf,
    agents_root: PathBuf,
}

impl HeptaFleetLayout {
    pub fn new(fleet_root: HeptaFleetRoot) -> Self {
        let state_root = fleet_root.as_path().join("state");
        let run_root = fleet_root.as_path().join("run");
        Self {
            fleet_config: fleet_root.as_path().join("fleet.toml"),
            supervisor_database: state_root.join("supervisor.sqlite3"),
            supervisor_socket: run_root.join("supervisor.sock"),
            supervisor_lock: run_root.join("supervisor.lock"),
            agents_root: fleet_root.as_path().join("agents"),
            fleet_root,
            state_root,
            run_root,
        }
    }

    pub fn fleet_root(&self) -> &HeptaFleetRoot {
        &self.fleet_root
    }

    pub fn fleet_config(&self) -> &Path {
        &self.fleet_config
    }

    pub fn state_root(&self) -> &Path {
        &self.state_root
    }

    pub fn supervisor_database(&self) -> &Path {
        &self.supervisor_database
    }

    pub fn run_root(&self) -> &Path {
        &self.run_root
    }

    pub fn supervisor_socket(&self) -> &Path {
        &self.supervisor_socket
    }

    pub fn supervisor_lock(&self) -> &Path {
        &self.supervisor_lock
    }

    pub fn agents_root(&self) -> &Path {
        &self.agents_root
    }

    pub fn agent(&self, agent_id: &AgentId) -> HeptaAgentLayout {
        HeptaAgentLayout::new(self.agents_root.join(agent_id.as_str()), agent_id.clone())
    }
}

/// Paths exclusively owned by one independently supervised workspace agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaAgentLayout {
    agent_id: AgentId,
    agent_root: PathBuf,
    agent_config: PathBuf,
    home_root: PathBuf,
    run_root: PathBuf,
    app_server_socket: PathBuf,
    writer_lock: PathBuf,
    generation_cursor: PathBuf,
    logs_root: PathBuf,
    releases_root: PathBuf,
    active_release: PathBuf,
    cognitive_root: PathBuf,
}

impl HeptaAgentLayout {
    fn new(agent_root: PathBuf, agent_id: AgentId) -> Self {
        let run_root = agent_root.join("run");
        let releases_root = agent_root.join("releases");
        Self {
            agent_config: agent_root.join("agent.toml"),
            home_root: agent_root.join("home"),
            app_server_socket: run_root.join("app-server.sock"),
            writer_lock: run_root.join("writer.lock"),
            generation_cursor: run_root.join("generation.json"),
            logs_root: agent_root.join("logs"),
            active_release: releases_root.join("active"),
            cognitive_root: agent_root.join("cognitive"),
            agent_id,
            agent_root,
            run_root,
            releases_root,
        }
    }

    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    pub fn agent_root(&self) -> &Path {
        &self.agent_root
    }

    pub fn agent_config(&self) -> &Path {
        &self.agent_config
    }

    pub fn home_root(&self) -> &Path {
        &self.home_root
    }

    pub fn run_root(&self) -> &Path {
        &self.run_root
    }

    pub fn app_server_socket(&self) -> &Path {
        &self.app_server_socket
    }

    pub fn writer_lock(&self) -> &Path {
        &self.writer_lock
    }

    pub fn generation_cursor(&self) -> &Path {
        &self.generation_cursor
    }

    pub fn logs_root(&self) -> &Path {
        &self.logs_root
    }

    pub fn releases_root(&self) -> &Path {
        &self.releases_root
    }

    pub fn active_release(&self) -> &Path {
        &self.active_release
    }

    pub fn cognitive_root(&self) -> &Path {
        &self.cognitive_root
    }
}

#[cfg(test)]
#[path = "fleet_tests.rs"]
mod tests;
