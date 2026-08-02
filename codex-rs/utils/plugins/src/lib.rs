//! Plugin path resolution, plaintext mention sigils, and MCP connector helpers shared across Hepta
//! crates.

use codex_utils_absolute_path::AbsolutePathBuf;

pub mod mcp_connector;
pub mod mention_syntax;
pub mod plugin_namespace;

pub use plugin_namespace::AGENT_PLUGIN_MANIFEST_RELATIVE_PATH;
pub use plugin_namespace::AGENT_PLUGIN_SCHEMA_PREFIX;
pub use plugin_namespace::AGENT_PLUGIN_SCHEMA_URI;
pub use plugin_namespace::AgentPluginSchemaStatus;
pub use plugin_namespace::PluginManifestPathResolution;
pub use plugin_namespace::SUPPORTED_AGENT_PLUGIN_SCHEMA_URIS;
pub use plugin_namespace::agent_plugin_schema_status;
pub use plugin_namespace::find_plugin_manifest_path;
pub use plugin_namespace::plugin_namespace_for_skill_path;
pub use plugin_namespace::resolve_plugin_manifest_path;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum SkillDiscoveryMode {
    #[default]
    Recursive,
    DirectChildren,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PluginSkillRoot {
    pub path: AbsolutePathBuf,
    pub plugin_id: String,
    /// Namespace captured from the manifest revision that produced this root.
    ///
    /// Consumers must use this value instead of reopening the manifest, so a plugin update cannot
    /// rename skills between catalog construction and skill loading.
    pub plugin_namespace: String,
    pub plugin_root: AbsolutePathBuf,
    pub discovery_mode: SkillDiscoveryMode,
}
