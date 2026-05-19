use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::model::{ModelRef, ProviderDescriptor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalConfigImportSourceSummary {
    pub kind: String,
    pub label: String,
    pub source: String,
    pub target: String,
    pub secret_material: bool,
    pub file_count: usize,
    pub byte_count: u64,
    pub skill_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupConfigSectionSummary {
    pub name: String,
    pub value_kind: String,
    pub item_count: usize,
    pub enabled_count: usize,
    pub secret_field_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigOptionSurface {
    pub path: String,
    pub value_kind: String,
    pub item_count: usize,
    pub secret_material: bool,
    pub redacted: bool,
    pub external_interface_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigSectionSurface {
    pub name: String,
    pub value_kind: String,
    pub direct_keys: Vec<String>,
    pub item_count: usize,
    pub enabled_count: usize,
    pub option_count: usize,
    pub secret_option_count: usize,
    pub redacted_option_count: usize,
    pub exposed_option_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalConfigSurfaceSummary {
    pub source_config_file: String,
    pub expected_top_level_keys: Vec<String>,
    pub top_level_keys: Vec<String>,
    pub missing_top_level_keys: Vec<String>,
    pub unexpected_top_level_keys: Vec<String>,
    pub top_level_alignment_complete: bool,
    pub section_count: usize,
    pub direct_key_count: usize,
    pub option_count: usize,
    pub secret_option_count: usize,
    pub redacted_option_count: usize,
    pub exposed_option_count: usize,
    pub sections: Vec<ConfigSectionSurface>,
    pub options: Vec<ConfigOptionSurface>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageGenerationModelDescriptor {
    pub provider: String,
    pub model: String,
    pub configured: bool,
    pub default_for_provider: bool,
    pub active_default: bool,
    pub requires_auth: bool,
    pub auth_hint: String,
    pub supports_editing: bool,
    pub max_reference_images: usize,
    pub sizes: Vec<String>,
    pub aspect_ratios: Vec<String>,
    pub resolutions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigCatalogItem {
    pub id: String,
    pub configured: bool,
    pub enabled: bool,
    pub active_default: bool,
    pub requires_auth: bool,
    pub auth_hint: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaGenerationModelDescriptor {
    pub media_kind: String,
    pub provider: String,
    pub model: String,
    pub configured: bool,
    pub default_for_provider: bool,
    pub active_default: bool,
    pub requires_auth: bool,
    pub auth_hint: String,
    pub modes: Vec<String>,
    pub max_outputs: usize,
    pub max_input_images: usize,
    pub max_input_videos: usize,
    pub max_duration_seconds: usize,
    pub supported_duration_seconds: Vec<usize>,
    pub sizes: Vec<String>,
    pub aspect_ratios: Vec<String>,
    pub resolutions: Vec<String>,
    pub formats: Vec<String>,
    pub supports_audio: bool,
    pub supports_lyrics: bool,
    pub supports_instrumental: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalConfigCatalogSummary {
    #[serde(default)]
    pub source: String,
    #[serde(default, alias = "hepta_runtime_version")]
    pub hepta_runtime_version: String,
    #[serde(default)]
    pub source_ok: bool,
    #[serde(default)]
    pub source_error: String,
    pub catalog_count: usize,
    #[serde(default)]
    pub config_schema_option_count: usize,
    #[serde(default)]
    pub config_schema_choice_path_count: usize,
    #[serde(default)]
    pub config_schema_top_level_key_count: usize,
    #[serde(default)]
    pub config_schema_ui_hint_count: usize,
    #[serde(default)]
    pub bundled_plugin_count: usize,
    #[serde(default)]
    pub channel_catalog_entry_count: usize,
    #[serde(default)]
    pub chat_command_count: usize,
    #[serde(default)]
    pub subcli_command_count: usize,
    #[serde(default)]
    pub thinking_level_count: usize,
    #[serde(default)]
    pub skill_catalog_count: usize,
    #[serde(default)]
    pub effective_tool_count: usize,
    #[serde(default)]
    pub tool_schema_count: usize,
    #[serde(default)]
    pub tool_parameter_option_count: usize,
    #[serde(default)]
    pub provider_auth_choice_count: usize,
    #[serde(default)]
    pub provider_auth_flag_count: usize,
    #[serde(default)]
    pub provider_auth_alias_count: usize,
    #[serde(default)]
    pub secret_target_count: usize,
    #[serde(default)]
    pub browser_profile_count: usize,
    #[serde(default)]
    pub browser_config_option_count: usize,
    #[serde(default)]
    pub tool_policy_group_count: usize,
    #[serde(default)]
    pub tool_policy_profile_count: usize,
    #[serde(default)]
    pub node_command_count: usize,
    #[serde(default)]
    pub debug_proxy_coverage_count: usize,
    #[serde(default)]
    pub setup_surface_count: usize,
    #[serde(default)]
    pub channel_message_schema_count: usize,
    #[serde(default)]
    pub model_provider_count: usize,
    #[serde(default)]
    pub model_catalog_model_count: usize,
    pub search_engine_count: usize,
    #[serde(default)]
    pub web_search_provider_count: usize,
    #[serde(default)]
    pub web_fetch_provider_count: usize,
    #[serde(default)]
    pub image_generation_provider_count: usize,
    #[serde(default)]
    pub image_generation_model_count: usize,
    pub video_generation_provider_count: usize,
    pub video_generation_model_count: usize,
    pub music_generation_provider_count: usize,
    pub music_generation_model_count: usize,
    #[serde(default)]
    pub media_understanding_provider_count: usize,
    #[serde(default)]
    pub speech_provider_count: usize,
    #[serde(default)]
    pub embedding_provider_count: usize,
    pub channel_count: usize,
    pub acp_agent_count: usize,
    pub plugin_count: usize,
    #[serde(default)]
    pub provider_extension_count: usize,
    pub tool_count: usize,
    pub command_count: usize,
    #[serde(default)]
    pub capability_count: usize,
    #[serde(default)]
    pub source_file_count: usize,
    #[serde(default)]
    pub config_schema_options: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub bundled_plugins: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub channel_catalog_entries: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub chat_commands: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub subcli_commands: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub thinking_levels: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub skills: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub effective_tools: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub tool_schemas: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub tool_parameter_options: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub provider_auth_choices: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub provider_auth_flags: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub provider_auth_aliases: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub secret_targets: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub browser_profiles: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub browser_config_options: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub tool_policy_groups: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub tool_policy_profiles: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub node_commands: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub debug_proxy_coverage: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub setup_surfaces: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub channel_message_schemas: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub model_providers: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub model_catalog_models: Vec<ConfigCatalogItem>,
    pub search_engines: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub web_search_providers: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub web_fetch_providers: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub image_generation_models: Vec<ImageGenerationModelDescriptor>,
    pub video_generation_models: Vec<MediaGenerationModelDescriptor>,
    pub music_generation_models: Vec<MediaGenerationModelDescriptor>,
    #[serde(default)]
    pub media_understanding_providers: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub speech_providers: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub embedding_providers: Vec<ConfigCatalogItem>,
    pub channels: Vec<ConfigCatalogItem>,
    pub acp_agents: Vec<ConfigCatalogItem>,
    pub plugins: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub provider_extensions: Vec<ConfigCatalogItem>,
    pub tools: Vec<ConfigCatalogItem>,
    pub commands: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub capabilities: Vec<ConfigCatalogItem>,
    #[serde(default)]
    pub source_files: Vec<ConfigCatalogItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupConfigImportSummary {
    pub source_config_file: String,
    pub top_level_keys: Vec<String>,
    pub section_count: usize,
    pub model_provider_count: usize,
    pub model_count: usize,
    pub model_providers: Vec<ProviderDescriptor>,
    pub agent_count: usize,
    pub agent_model_ref_count: usize,
    pub search_engine_count: usize,
    pub search_engines: Vec<String>,
    pub image_generation_engine_count: usize,
    pub image_generation_engines: Vec<ModelRef>,
    pub image_generation_provider_count: usize,
    pub available_image_generation_model_count: usize,
    pub available_image_generation_models: Vec<ImageGenerationModelDescriptor>,
    pub external_channel_count: usize,
    pub enabled_external_channel_count: usize,
    pub external_channels: Vec<String>,
    pub plugin_count: usize,
    pub enabled_plugin_count: usize,
    pub skill_entry_count: usize,
    pub acp_agent_count: usize,
    pub sections: Vec<StartupConfigSectionSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalConfigImportManifest {
    pub schema_version: u32,
    pub imported_at_unix_ms: u64,
    pub source_root: String,
    pub install_root: String,
    pub import_root: String,
    pub secret_material_policy: String,
    pub config_file_count: usize,
    pub auth_file_count: usize,
    pub credential_file_count: usize,
    pub skill_source_count: usize,
    pub skill_count: usize,
    #[serde(default)]
    pub visible_skill_count: usize,
    #[serde(default)]
    pub visible_skill_filter_applied: bool,
    #[serde(default)]
    pub visible_skill_ids: Vec<String>,
    pub copied_file_count: usize,
    pub copied_bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_config: Option<StartupConfigImportSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_config_surface: Option<ExternalConfigSurfaceSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_config_catalog: Option<OptionalConfigCatalogSummary>,
    #[serde(default)]
    pub sources: Vec<LocalConfigImportSourceSummary>,
}

impl LocalConfigImportManifest {
    pub fn has_config(&self) -> bool {
        self.config_file_count > 0
    }

    pub fn has_auth(&self) -> bool {
        self.auth_file_count > 0 || self.credential_file_count > 0
    }

    pub fn has_skills(&self) -> bool {
        self.skill_source_count > 0 && self.skill_count > 0
    }

    pub fn has_startup_config(&self) -> bool {
        self.startup_config
            .as_ref()
            .map(|startup| {
                startup.section_count > 0
                    && startup.model_provider_count > 0
                    && startup.search_engine_count > 0
                    && startup.image_generation_engine_count > 0
                    && startup.external_channel_count > 0
            })
            .unwrap_or(false)
    }

    pub fn has_external_config_surface(&self) -> bool {
        self.external_config_surface
            .as_ref()
            .map(|surface| {
                surface.top_level_alignment_complete
                    && surface.section_count > 0
                    && surface.option_count > 0
                    && surface.exposed_option_count > 0
            })
            .unwrap_or(false)
    }

    pub fn has_optional_config_catalog(&self) -> bool {
        self.optional_config_catalog
            .as_ref()
            .map(|catalog| {
                catalog.source_ok
                    && catalog.catalog_count >= 40
                    && catalog.config_schema_option_count > 0
                    && catalog.config_schema_choice_path_count > 0
                    && catalog.config_schema_top_level_key_count > 0
                    // HeptaRuntime 2026.5.2 can expose plugin parity through the
                    // runtime provider-extension/plugin inventory without a
                    // separate bundled-plugin metadata feed.
                    && (catalog.bundled_plugin_count > 0
                        || catalog.plugin_count > 0
                        || catalog.provider_extension_count > 0)
                    && catalog.channel_catalog_entry_count > 0
                    && catalog.channel_message_schema_count > 0
                    && catalog.chat_command_count > 0
                    && catalog.subcli_command_count > 0
                    && catalog.thinking_level_count > 0
                    && catalog.skill_catalog_count > 0
                    && catalog.effective_tool_count > 0
                    && catalog.tool_schema_count > 0
                    && catalog.tool_parameter_option_count > 0
                    && catalog.provider_auth_choice_count > 0
                    && catalog.provider_auth_flag_count > 0
                    && catalog.secret_target_count > 0
                    && catalog.browser_config_option_count > 0
                    && catalog.tool_policy_group_count > 0
                    && catalog.tool_policy_profile_count > 0
                    && catalog.node_command_count > 0
                    && catalog.debug_proxy_coverage_count > 0
                    // Setup surfaces are optional in current HeptaRuntime builds;
                    // provider auth choices/flags/secret targets carry the
                    // local onboarding contract when no dedicated setup feed is
                    // exported.
                    && (catalog.setup_surface_count > 0
                        || (catalog.provider_auth_choice_count > 0
                            && catalog.provider_auth_flag_count > 0
                            && catalog.secret_target_count > 0))
                    && catalog.model_provider_count > 0
                    && catalog.model_catalog_model_count > 0
                    && catalog.search_engine_count > 0
                    && catalog.image_generation_model_count > 0
                    && catalog.video_generation_model_count > 0
                    && catalog.music_generation_model_count > 0
                    && catalog.media_understanding_provider_count > 0
                    && catalog.speech_provider_count > 0
                    && catalog.embedding_provider_count > 0
                    && catalog.channel_count > 0
                    && catalog.acp_agent_count > 0
                    && catalog.plugin_count > 0
                    && catalog.tool_count > 0
                    && catalog.command_count > 0
                    && catalog.source_file_count > 0
            })
            .unwrap_or(false)
    }

    pub fn local_import_complete(&self) -> bool {
        self.has_config()
            && self.has_auth()
            && self.has_skills()
            && self.has_startup_config()
            && self.has_external_config_surface()
            && self.has_optional_config_catalog()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalConfigSurfaceStatus {
    pub import_root: String,
    pub manifest_path: String,
    pub manifest_present: bool,
    pub config_surface_ready: bool,
    pub external_interface_aligned: bool,
    pub section_count: usize,
    pub option_count: usize,
    pub secret_option_count: usize,
    pub redacted_option_count: usize,
    pub exposed_option_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<ExternalConfigSurfaceSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageGenerationCatalogStatus {
    pub import_root: String,
    pub manifest_path: String,
    pub manifest_present: bool,
    pub image_generation_ready: bool,
    pub provider_count: usize,
    pub model_count: usize,
    pub configured_model_count: usize,
    pub active_default_count: usize,
    pub models: Vec<ImageGenerationModelDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalConfigCatalogStatus {
    pub import_root: String,
    pub manifest_path: String,
    pub manifest_present: bool,
    pub optional_config_catalog_ready: bool,
    #[serde(default)]
    pub catalog_hepta_runtime_version: String,
    #[serde(default)]
    pub installed_hepta_runtime_version: String,
    #[serde(default)]
    pub catalog_version_current: bool,
    #[serde(default)]
    pub catalog_refresh_recommended: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_refresh_plan: Option<LocalImportCatalogRefreshPlan>,
    pub catalog_count: usize,
    pub config_schema_option_count: usize,
    pub config_schema_choice_path_count: usize,
    pub config_schema_top_level_key_count: usize,
    pub config_schema_ui_hint_count: usize,
    pub bundled_plugin_count: usize,
    pub channel_catalog_entry_count: usize,
    pub chat_command_count: usize,
    pub subcli_command_count: usize,
    pub thinking_level_count: usize,
    pub skill_catalog_count: usize,
    pub effective_tool_count: usize,
    pub available_tool_count: usize,
    pub tool_schema_count: usize,
    pub tool_parameter_option_count: usize,
    pub provider_auth_choice_count: usize,
    pub provider_auth_flag_count: usize,
    pub provider_auth_alias_count: usize,
    pub secret_target_count: usize,
    pub browser_profile_count: usize,
    pub browser_config_option_count: usize,
    pub tool_policy_group_count: usize,
    pub tool_policy_profile_count: usize,
    pub node_command_count: usize,
    pub debug_proxy_coverage_count: usize,
    pub setup_surface_count: usize,
    pub channel_message_schema_count: usize,
    pub source_file_count: usize,
    pub model_provider_count: usize,
    pub model_catalog_model_count: usize,
    pub search_engine_count: usize,
    pub web_search_provider_count: usize,
    pub web_fetch_provider_count: usize,
    pub image_generation_model_count: usize,
    pub video_generation_model_count: usize,
    pub music_generation_model_count: usize,
    pub media_understanding_provider_count: usize,
    pub speech_provider_count: usize,
    pub embedding_provider_count: usize,
    pub channel_count: usize,
    pub acp_agent_count: usize,
    pub plugin_count: usize,
    pub provider_extension_count: usize,
    pub tool_count: usize,
    pub command_count: usize,
    pub capability_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog: Option<OptionalConfigCatalogSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

fn available_tool_count_from_catalog(catalog: Option<&OptionalConfigCatalogSummary>) -> usize {
    let Some(catalog) = catalog else {
        return 0;
    };
    let mut ids = BTreeSet::new();
    for tool in catalog.tools.iter().chain(catalog.effective_tools.iter()) {
        if !tool.id.trim().is_empty() {
            ids.insert(tool.id.as_str());
        }
    }
    ids.len()
        .max(catalog.tool_count)
        .max(catalog.effective_tool_count)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalConfigImportStatus {
    pub import_root: String,
    pub manifest_path: String,
    pub manifest_present: bool,
    pub local_import_complete: bool,
    #[serde(default)]
    pub catalog_hepta_runtime_version: String,
    #[serde(default)]
    pub installed_hepta_runtime_version: String,
    #[serde(default)]
    pub catalog_version_current: bool,
    #[serde(default)]
    pub catalog_refresh_recommended: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_refresh_plan: Option<LocalImportCatalogRefreshPlan>,
    pub config_ready: bool,
    pub auth_ready: bool,
    pub skills_ready: bool,
    pub startup_config_ready: bool,
    pub model_providers_ready: bool,
    pub search_engines_ready: bool,
    pub image_engines_ready: bool,
    pub external_channels_ready: bool,
    pub config_surface_ready: bool,
    pub external_interface_aligned: bool,
    pub optional_config_catalog_ready: bool,
    pub video_generation_ready: bool,
    pub music_generation_ready: bool,
    pub secret_material_local_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<LocalConfigImportManifest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalImportCatalogRefreshPlan {
    pub refresh_needed: bool,
    pub catalog_hepta_runtime_version: String,
    pub installed_hepta_runtime_version: String,
    pub local_import_script: String,
    pub boundary: String,
    pub safe_default: String,
    pub requires_explicit_confirmation: bool,
    pub import_script_executed: bool,
    pub private_config_copy_possible: bool,
    pub private_config_copied: bool,
    pub credential_value_read: bool,
    pub network_call_attempted: bool,
    pub recommended_operator_action: String,
}

impl LocalImportCatalogRefreshPlan {
    fn new(
        refresh_needed: bool,
        catalog_hepta_runtime_version: impl Into<String>,
        installed_hepta_runtime_version: impl Into<String>,
    ) -> Self {
        let catalog_hepta_runtime_version = catalog_hepta_runtime_version.into();
        let installed_hepta_runtime_version = installed_hepta_runtime_version.into();
        Self {
            refresh_needed,
            catalog_hepta_runtime_version: catalog_hepta_runtime_version.clone(),
            installed_hepta_runtime_version: installed_hepta_runtime_version.clone(),
            local_import_script: "scripts/hepta-local-import.sh".into(),
            boundary: "local-only refresh planning; import script may copy private config material"
                .into(),
            safe_default:
                "report-only; do not run import script without explicit operator confirmation"
                    .into(),
            requires_explicit_confirmation: refresh_needed,
            import_script_executed: false,
            private_config_copy_possible: true,
            private_config_copied: false,
            credential_value_read: false,
            network_call_attempted: false,
            recommended_operator_action: if refresh_needed {
                format!(
                    "Confirm before refreshing local-import catalog from {catalog_hepta_runtime_version} to {installed_hepta_runtime_version}; the import script can copy private config under .hepta/local-import/private."
                )
            } else {
                "No catalog refresh needed; local-import catalog matches installed HeptaRuntime version."
                    .into()
            },
        }
    }
}

impl LocalConfigImportStatus {
    pub fn missing(import_root: impl Into<String>, manifest_path: impl Into<String>) -> Self {
        Self {
            import_root: import_root.into(),
            manifest_path: manifest_path.into(),
            manifest_present: false,
            local_import_complete: false,
            catalog_hepta_runtime_version: String::new(),
            installed_hepta_runtime_version: String::new(),
            catalog_version_current: false,
            catalog_refresh_recommended: false,
            catalog_refresh_plan: None,
            config_ready: false,
            auth_ready: false,
            skills_ready: false,
            startup_config_ready: false,
            model_providers_ready: false,
            search_engines_ready: false,
            image_engines_ready: false,
            external_channels_ready: false,
            config_surface_ready: false,
            external_interface_aligned: false,
            optional_config_catalog_ready: false,
            video_generation_ready: false,
            music_generation_ready: false,
            secret_material_local_only: true,
            manifest: None,
            error: Some(
                "local config import manifest not found; run scripts/hepta-local-import.sh".into(),
            ),
        }
    }

    pub fn from_manifest_path(path: impl AsRef<Path>) -> Self {
        let manifest_path = path.as_ref().to_path_buf();
        let import_root = manifest_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from(".hepta/local-import"));
        if !manifest_path.exists() {
            return Self::missing(
                import_root.to_string_lossy().to_string(),
                manifest_path.to_string_lossy().to_string(),
            );
        }

        match fs::read_to_string(&manifest_path)
            .map_err(|err| err.to_string())
            .and_then(|text| {
                serde_json::from_str::<LocalConfigImportManifest>(&text)
                    .map_err(|err| err.to_string())
            }) {
            Ok(manifest) => {
                let config_ready = manifest.has_config();
                let auth_ready = manifest.has_auth();
                let skills_ready = manifest.has_skills();
                let startup_config_ready = manifest.has_startup_config();
                let model_providers_ready = manifest
                    .startup_config
                    .as_ref()
                    .map(|startup| startup.model_provider_count > 0 && startup.model_count > 0)
                    .unwrap_or(false);
                let search_engines_ready = manifest
                    .startup_config
                    .as_ref()
                    .map(|startup| startup.search_engine_count > 0)
                    .unwrap_or(false);
                let image_engines_ready = manifest
                    .startup_config
                    .as_ref()
                    .map(|startup| startup.image_generation_engine_count > 0)
                    .unwrap_or(false);
                let external_channels_ready = manifest
                    .startup_config
                    .as_ref()
                    .map(|startup| startup.external_channel_count > 0)
                    .unwrap_or(false);
                let config_surface_ready = manifest.has_external_config_surface();
                let external_interface_aligned = manifest
                    .external_config_surface
                    .as_ref()
                    .map(|surface| {
                        surface.top_level_alignment_complete
                            && surface.option_count
                                == surface.exposed_option_count + surface.redacted_option_count
                    })
                    .unwrap_or(false);
                let optional_config_catalog_ready = manifest.has_optional_config_catalog();
                let catalog_hepta_runtime_version = manifest
                    .optional_config_catalog
                    .as_ref()
                    .map(|catalog| catalog.hepta_runtime_version.clone())
                    .unwrap_or_default();
                let installed_hepta_runtime_version =
                    read_hepta_runtime_package_version(&manifest.install_root);
                let catalog_version_current = !catalog_hepta_runtime_version.is_empty()
                    && !installed_hepta_runtime_version.is_empty()
                    && catalog_hepta_runtime_version == installed_hepta_runtime_version;
                let catalog_refresh_recommended = optional_config_catalog_ready
                    && !installed_hepta_runtime_version.is_empty()
                    && !catalog_version_current;
                let catalog_refresh_plan = catalog_refresh_recommended.then(|| {
                    LocalImportCatalogRefreshPlan::new(
                        true,
                        catalog_hepta_runtime_version.clone(),
                        installed_hepta_runtime_version.clone(),
                    )
                });
                let video_generation_ready = manifest
                    .optional_config_catalog
                    .as_ref()
                    .map(|catalog| catalog.video_generation_model_count > 0)
                    .unwrap_or(false);
                let music_generation_ready = manifest
                    .optional_config_catalog
                    .as_ref()
                    .map(|catalog| catalog.music_generation_model_count > 0)
                    .unwrap_or(false);
                let mut manifest = manifest;
                if let Some(catalog) = &mut manifest.optional_config_catalog {
                    let available_tool_count = available_tool_count_from_catalog(Some(catalog));
                    catalog.tool_count = available_tool_count;
                    catalog.effective_tool_count = available_tool_count;
                }
                let local_import_complete = manifest.local_import_complete();
                let secret_material_local_only = manifest
                    .secret_material_policy
                    .contains(".hepta/local-import/private")
                    && manifest.secret_material_policy.contains("never commit");
                Self {
                    import_root: import_root.to_string_lossy().to_string(),
                    manifest_path: manifest_path.to_string_lossy().to_string(),
                    manifest_present: true,
                    local_import_complete,
                    catalog_hepta_runtime_version,
                    installed_hepta_runtime_version,
                    catalog_version_current,
                    catalog_refresh_recommended,
                    catalog_refresh_plan,
                    config_ready,
                    auth_ready,
                    skills_ready,
                    startup_config_ready,
                    model_providers_ready,
                    search_engines_ready,
                    image_engines_ready,
                    external_channels_ready,
                    config_surface_ready,
                    external_interface_aligned,
                    optional_config_catalog_ready,
                    video_generation_ready,
                    music_generation_ready,
                    secret_material_local_only,
                    manifest: Some(manifest),
                    error: None,
                }
            }
            Err(error) => Self {
                import_root: import_root.to_string_lossy().to_string(),
                manifest_path: manifest_path.to_string_lossy().to_string(),
                manifest_present: true,
                local_import_complete: false,
                catalog_hepta_runtime_version: String::new(),
                installed_hepta_runtime_version: String::new(),
                catalog_version_current: false,
                catalog_refresh_recommended: false,
                catalog_refresh_plan: None,
                config_ready: false,
                auth_ready: false,
                skills_ready: false,
                startup_config_ready: false,
                model_providers_ready: false,
                search_engines_ready: false,
                image_engines_ready: false,
                external_channels_ready: false,
                config_surface_ready: false,
                external_interface_aligned: false,
                optional_config_catalog_ready: false,
                video_generation_ready: false,
                music_generation_ready: false,
                secret_material_local_only: false,
                manifest: None,
                error: Some(error),
            },
        }
    }

    pub fn redacted_for_product_surface(mut self) -> Self {
        if let Some(manifest) = &mut self.manifest {
            manifest.source_root = "local-config-home".into();
            manifest.install_root = "local-install-root".into();
            manifest.import_root = self.import_root.clone();
            if let Some(startup) = &mut manifest.startup_config {
                startup.source_config_file = "local-startup-config.json".into();
            }
            if let Some(surface) = &mut manifest.external_config_surface {
                surface.source_config_file = "local-startup-config.json".into();
            }
            manifest.sources.clear();
        }
        self
    }

    pub fn config_surface_status(&self) -> LocalConfigSurfaceStatus {
        let surface = self
            .manifest
            .as_ref()
            .and_then(|manifest| manifest.external_config_surface.clone());
        LocalConfigSurfaceStatus {
            import_root: self.import_root.clone(),
            manifest_path: self.manifest_path.clone(),
            manifest_present: self.manifest_present,
            config_surface_ready: self.config_surface_ready,
            external_interface_aligned: self.external_interface_aligned,
            section_count: surface
                .as_ref()
                .map(|item| item.section_count)
                .unwrap_or_default(),
            option_count: surface
                .as_ref()
                .map(|item| item.option_count)
                .unwrap_or_default(),
            secret_option_count: surface
                .as_ref()
                .map(|item| item.secret_option_count)
                .unwrap_or_default(),
            redacted_option_count: surface
                .as_ref()
                .map(|item| item.redacted_option_count)
                .unwrap_or_default(),
            exposed_option_count: surface
                .as_ref()
                .map(|item| item.exposed_option_count)
                .unwrap_or_default(),
            surface,
            error: self.error.clone(),
        }
    }

    pub fn image_generation_catalog_status(&self) -> ImageGenerationCatalogStatus {
        let models = self
            .manifest
            .as_ref()
            .and_then(|manifest| manifest.startup_config.as_ref())
            .map(|startup| startup.available_image_generation_models.clone())
            .unwrap_or_default();
        let provider_count = models
            .iter()
            .map(|model| model.provider.as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        ImageGenerationCatalogStatus {
            import_root: self.import_root.clone(),
            manifest_path: self.manifest_path.clone(),
            manifest_present: self.manifest_present,
            image_generation_ready: !models.is_empty(),
            provider_count,
            model_count: models.len(),
            configured_model_count: models.iter().filter(|model| model.configured).count(),
            active_default_count: models.iter().filter(|model| model.active_default).count(),
            models,
            error: self.error.clone(),
        }
    }

    pub fn optional_config_catalog_status(&self) -> OptionalConfigCatalogStatus {
        let catalog = self
            .manifest
            .as_ref()
            .and_then(|manifest| manifest.optional_config_catalog.clone());
        let catalog_hepta_runtime_version = if self.catalog_hepta_runtime_version.is_empty() {
            catalog
                .as_ref()
                .map(|item| item.hepta_runtime_version.clone())
                .unwrap_or_default()
        } else {
            self.catalog_hepta_runtime_version.clone()
        };
        let installed_hepta_runtime_version = if self.installed_hepta_runtime_version.is_empty() {
            self.manifest
                .as_ref()
                .map(|manifest| read_hepta_runtime_package_version(&manifest.install_root))
                .unwrap_or_default()
        } else {
            self.installed_hepta_runtime_version.clone()
        };
        let catalog_version_current = !catalog_hepta_runtime_version.is_empty()
            && !installed_hepta_runtime_version.is_empty()
            && catalog_hepta_runtime_version == installed_hepta_runtime_version;
        let catalog_refresh_recommended = self.catalog_refresh_recommended
            || (self.manifest_present
                && self.optional_config_catalog_ready
                && !installed_hepta_runtime_version.is_empty()
                && !catalog_version_current);
        let catalog_refresh_plan = catalog_refresh_recommended.then(|| {
            LocalImportCatalogRefreshPlan::new(
                true,
                catalog_hepta_runtime_version.clone(),
                installed_hepta_runtime_version.clone(),
            )
        });
        let available_tool_count = available_tool_count_from_catalog(catalog.as_ref());
        OptionalConfigCatalogStatus {
            import_root: self.import_root.clone(),
            manifest_path: self.manifest_path.clone(),
            manifest_present: self.manifest_present,
            optional_config_catalog_ready: self.optional_config_catalog_ready,
            catalog_hepta_runtime_version,
            installed_hepta_runtime_version,
            catalog_version_current,
            catalog_refresh_recommended,
            catalog_refresh_plan,
            catalog_count: catalog
                .as_ref()
                .map(|item| item.catalog_count)
                .unwrap_or_default(),
            config_schema_option_count: catalog
                .as_ref()
                .map(|item| item.config_schema_option_count)
                .unwrap_or_default(),
            config_schema_choice_path_count: catalog
                .as_ref()
                .map(|item| item.config_schema_choice_path_count)
                .unwrap_or_default(),
            config_schema_top_level_key_count: catalog
                .as_ref()
                .map(|item| item.config_schema_top_level_key_count)
                .unwrap_or_default(),
            config_schema_ui_hint_count: catalog
                .as_ref()
                .map(|item| item.config_schema_ui_hint_count)
                .unwrap_or_default(),
            bundled_plugin_count: catalog
                .as_ref()
                .map(|item| item.bundled_plugin_count)
                .unwrap_or_default(),
            channel_catalog_entry_count: catalog
                .as_ref()
                .map(|item| item.channel_catalog_entry_count)
                .unwrap_or_default(),
            chat_command_count: catalog
                .as_ref()
                .map(|item| item.chat_command_count)
                .unwrap_or_default(),
            subcli_command_count: catalog
                .as_ref()
                .map(|item| item.subcli_command_count)
                .unwrap_or_default(),
            thinking_level_count: catalog
                .as_ref()
                .map(|item| item.thinking_level_count)
                .unwrap_or_default(),
            skill_catalog_count: catalog
                .as_ref()
                .map(|item| item.skill_catalog_count)
                .unwrap_or_default(),
            effective_tool_count: available_tool_count,
            available_tool_count,
            tool_schema_count: catalog
                .as_ref()
                .map(|item| item.tool_schema_count)
                .unwrap_or_default(),
            tool_parameter_option_count: catalog
                .as_ref()
                .map(|item| item.tool_parameter_option_count)
                .unwrap_or_default(),
            provider_auth_choice_count: catalog
                .as_ref()
                .map(|item| item.provider_auth_choice_count)
                .unwrap_or_default(),
            provider_auth_flag_count: catalog
                .as_ref()
                .map(|item| item.provider_auth_flag_count)
                .unwrap_or_default(),
            provider_auth_alias_count: catalog
                .as_ref()
                .map(|item| item.provider_auth_alias_count)
                .unwrap_or_default(),
            secret_target_count: catalog
                .as_ref()
                .map(|item| item.secret_target_count)
                .unwrap_or_default(),
            browser_profile_count: catalog
                .as_ref()
                .map(|item| item.browser_profile_count)
                .unwrap_or_default(),
            browser_config_option_count: catalog
                .as_ref()
                .map(|item| item.browser_config_option_count)
                .unwrap_or_default(),
            tool_policy_group_count: catalog
                .as_ref()
                .map(|item| item.tool_policy_group_count)
                .unwrap_or_default(),
            tool_policy_profile_count: catalog
                .as_ref()
                .map(|item| item.tool_policy_profile_count)
                .unwrap_or_default(),
            node_command_count: catalog
                .as_ref()
                .map(|item| item.node_command_count)
                .unwrap_or_default(),
            debug_proxy_coverage_count: catalog
                .as_ref()
                .map(|item| item.debug_proxy_coverage_count)
                .unwrap_or_default(),
            setup_surface_count: catalog
                .as_ref()
                .map(|item| item.setup_surface_count)
                .unwrap_or_default(),
            channel_message_schema_count: catalog
                .as_ref()
                .map(|item| item.channel_message_schema_count)
                .unwrap_or_default(),
            source_file_count: catalog
                .as_ref()
                .map(|item| item.source_file_count)
                .unwrap_or_default(),
            model_provider_count: catalog
                .as_ref()
                .map(|item| item.model_provider_count)
                .unwrap_or_default(),
            model_catalog_model_count: catalog
                .as_ref()
                .map(|item| item.model_catalog_model_count)
                .unwrap_or_default(),
            search_engine_count: catalog
                .as_ref()
                .map(|item| item.search_engine_count)
                .unwrap_or_default(),
            web_search_provider_count: catalog
                .as_ref()
                .map(|item| item.web_search_provider_count)
                .unwrap_or_default(),
            web_fetch_provider_count: catalog
                .as_ref()
                .map(|item| item.web_fetch_provider_count)
                .unwrap_or_default(),
            image_generation_model_count: catalog
                .as_ref()
                .map(|item| item.image_generation_model_count)
                .unwrap_or_default(),
            video_generation_model_count: catalog
                .as_ref()
                .map(|item| item.video_generation_model_count)
                .unwrap_or_default(),
            music_generation_model_count: catalog
                .as_ref()
                .map(|item| item.music_generation_model_count)
                .unwrap_or_default(),
            media_understanding_provider_count: catalog
                .as_ref()
                .map(|item| item.media_understanding_provider_count)
                .unwrap_or_default(),
            speech_provider_count: catalog
                .as_ref()
                .map(|item| item.speech_provider_count)
                .unwrap_or_default(),
            embedding_provider_count: catalog
                .as_ref()
                .map(|item| item.embedding_provider_count)
                .unwrap_or_default(),
            channel_count: catalog
                .as_ref()
                .map(|item| item.channel_count)
                .unwrap_or_default(),
            acp_agent_count: catalog
                .as_ref()
                .map(|item| item.acp_agent_count)
                .unwrap_or_default(),
            plugin_count: catalog
                .as_ref()
                .map(|item| item.plugin_count)
                .unwrap_or_default(),
            provider_extension_count: catalog
                .as_ref()
                .map(|item| item.provider_extension_count)
                .unwrap_or_default(),
            tool_count: available_tool_count,
            command_count: catalog
                .as_ref()
                .map(|item| item.command_count)
                .unwrap_or_default(),
            capability_count: catalog
                .as_ref()
                .map(|item| item.capability_count)
                .unwrap_or_default(),
            catalog,
            error: self.error.clone(),
        }
    }
}

fn read_hepta_runtime_package_version(install_root: &str) -> String {
    if install_root.trim().is_empty() {
        return String::new();
    }
    let package_path = Path::new(install_root).join("package.json");
    fs::read_to_string(package_path)
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|value| {
            value
                .get("version")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string)
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_completion_requires_config_auth_and_skills() {
        let manifest = LocalConfigImportManifest {
            schema_version: 1,
            imported_at_unix_ms: 1,
            source_root: "/tmp/local-config".into(),
            install_root: "/tmp/install".into(),
            import_root: "/tmp/hepta/.hepta/local-import".into(),
            secret_material_policy:
                "copied locally under .hepta/local-import/private; never commit".into(),
            config_file_count: 2,
            auth_file_count: 1,
            credential_file_count: 0,
            skill_source_count: 1,
            skill_count: 3,
            visible_skill_count: 3,
            visible_skill_filter_applied: false,
            visible_skill_ids: vec!["one".into(), "two".into(), "three".into()],
            copied_file_count: 6,
            copied_bytes: 42,
            startup_config: Some(StartupConfigImportSummary {
                source_config_file: "/tmp/local-config/startup.json".into(),
                top_level_keys: vec!["models".into(), "tools".into(), "channels".into()],
                section_count: 3,
                model_provider_count: 1,
                model_count: 1,
                model_providers: vec![ProviderDescriptor {
                    id: "local-provider".into(),
                    display_name: "Local Provider".into(),
                    transport_kind: crate::ProviderTransportKind::OpenAiCompatibleHttp,
                    default_model: ModelRef {
                        provider: "local-provider".into(),
                        model: "local-chat".into(),
                    },
                    available_models: vec![ModelRef {
                        provider: "local-provider".into(),
                        model: "local-chat".into(),
                    }],
                    requires_auth: true,
                    supports_tool_calls: true,
                }],
                agent_count: 1,
                agent_model_ref_count: 1,
                search_engine_count: 1,
                search_engines: vec!["local-search".into()],
                image_generation_engine_count: 1,
                image_generation_engines: vec![ModelRef {
                    provider: "local-image".into(),
                    model: "image-model".into(),
                }],
                image_generation_provider_count: 1,
                available_image_generation_model_count: 1,
                available_image_generation_models: vec![ImageGenerationModelDescriptor {
                    provider: "local-image".into(),
                    model: "image-model".into(),
                    configured: true,
                    default_for_provider: true,
                    active_default: true,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    supports_editing: true,
                    max_reference_images: 1,
                    sizes: vec!["1024x1024".into()],
                    aspect_ratios: vec!["1:1".into()],
                    resolutions: vec!["1K".into()],
                }],
                external_channel_count: 1,
                enabled_external_channel_count: 1,
                external_channels: vec!["telegram".into()],
                plugin_count: 1,
                enabled_plugin_count: 1,
                skill_entry_count: 1,
                acp_agent_count: 1,
                sections: vec![],
            }),
            external_config_surface: Some(ExternalConfigSurfaceSummary {
                source_config_file: "/tmp/local-config/startup.json".into(),
                expected_top_level_keys: vec!["channels".into(), "models".into(), "tools".into()],
                top_level_keys: vec!["channels".into(), "models".into(), "tools".into()],
                missing_top_level_keys: vec![],
                unexpected_top_level_keys: vec![],
                top_level_alignment_complete: true,
                section_count: 3,
                direct_key_count: 3,
                option_count: 4,
                secret_option_count: 1,
                redacted_option_count: 1,
                exposed_option_count: 3,
                sections: vec![ConfigSectionSurface {
                    name: "models".into(),
                    value_kind: "object".into(),
                    direct_keys: vec!["providers".into()],
                    item_count: 1,
                    enabled_count: 1,
                    option_count: 2,
                    secret_option_count: 0,
                    redacted_option_count: 0,
                    exposed_option_count: 2,
                }],
                options: vec![
                    ConfigOptionSurface {
                        path: "models".into(),
                        value_kind: "object".into(),
                        item_count: 1,
                        secret_material: false,
                        redacted: false,
                        external_interface_available: true,
                    },
                    ConfigOptionSurface {
                        path: "auth.token".into(),
                        value_kind: "string".into(),
                        item_count: 1,
                        secret_material: true,
                        redacted: true,
                        external_interface_available: true,
                    },
                    ConfigOptionSurface {
                        path: "channels".into(),
                        value_kind: "object".into(),
                        item_count: 1,
                        secret_material: false,
                        redacted: false,
                        external_interface_available: true,
                    },
                    ConfigOptionSurface {
                        path: "tools".into(),
                        value_kind: "object".into(),
                        item_count: 1,
                        secret_material: false,
                        redacted: false,
                        external_interface_available: true,
                    },
                ],
            }),
            optional_config_catalog: Some(OptionalConfigCatalogSummary {
                source: "hepta_runtime-dist-source-registry".into(),
                hepta_runtime_version: "test".into(),
                source_ok: true,
                source_error: String::new(),
                catalog_count: 41,
                config_schema_option_count: 1,
                config_schema_choice_path_count: 1,
                config_schema_top_level_key_count: 1,
                config_schema_ui_hint_count: 1,
                bundled_plugin_count: 1,
                channel_catalog_entry_count: 1,
                chat_command_count: 1,
                subcli_command_count: 1,
                thinking_level_count: 1,
                skill_catalog_count: 1,
                effective_tool_count: 1,
                tool_schema_count: 1,
                tool_parameter_option_count: 1,
                provider_auth_choice_count: 1,
                provider_auth_flag_count: 1,
                provider_auth_alias_count: 1,
                secret_target_count: 1,
                browser_profile_count: 1,
                browser_config_option_count: 1,
                tool_policy_group_count: 1,
                tool_policy_profile_count: 1,
                node_command_count: 1,
                debug_proxy_coverage_count: 1,
                setup_surface_count: 1,
                channel_message_schema_count: 1,
                model_provider_count: 1,
                model_catalog_model_count: 1,
                search_engine_count: 1,
                web_search_provider_count: 1,
                web_fetch_provider_count: 1,
                image_generation_provider_count: 1,
                image_generation_model_count: 1,
                video_generation_provider_count: 1,
                video_generation_model_count: 1,
                music_generation_provider_count: 1,
                music_generation_model_count: 1,
                media_understanding_provider_count: 1,
                speech_provider_count: 1,
                embedding_provider_count: 1,
                channel_count: 1,
                acp_agent_count: 1,
                plugin_count: 1,
                provider_extension_count: 1,
                tool_count: 1,
                command_count: 1,
                capability_count: 1,
                source_file_count: 1,
                config_schema_options: vec![ConfigCatalogItem {
                    id: "models.providers.*.api".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime runtime JSON schema".into(),
                    capabilities: vec!["type:string".into(), "choices:2".into()],
                }],
                bundled_plugins: vec![ConfigCatalogItem {
                    id: "local-plugin".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "bundled HeptaRuntime plugin manifest".into(),
                    capabilities: vec!["bundled-plugin".into()],
                }],
                channel_catalog_entries: vec![ConfigCatalogItem {
                    id: "telegram".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: true,
                    auth_hint: "channel setup".into(),
                    capabilities: vec!["channel-catalog".into()],
                }],
                chat_commands: vec![ConfigCatalogItem {
                    id: "chat-command:status".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime command registry metadata".into(),
                    capabilities: vec!["chat-command".into()],
                }],
                subcli_commands: vec![ConfigCatalogItem {
                    id: "subcli:status".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime command registry metadata".into(),
                    capabilities: vec!["subcli".into()],
                }],
                thinking_levels: vec![ConfigCatalogItem {
                    id: "low".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime thinking level registry".into(),
                    capabilities: vec!["thinking-level".into()],
                }],
                skills: vec![ConfigCatalogItem {
                    id: "example".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "local/bundled AgentSkill metadata".into(),
                    capabilities: vec!["skill".into()],
                }],
                effective_tools: vec![ConfigCatalogItem {
                    id: "exec".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime effective tool inventory".into(),
                    capabilities: vec!["effective-tool".into()],
                }],
                tool_schemas: vec![ConfigCatalogItem {
                    id: "message".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime gateway tool JSON schema".into(),
                    capabilities: vec!["gateway-tool-schema".into()],
                }],
                tool_parameter_options: vec![ConfigCatalogItem {
                    id: "message.action".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime tool parameter schema".into(),
                    capabilities: vec!["tool-parameter".into()],
                }],
                provider_auth_choices: vec![ConfigCatalogItem {
                    id: "openai-api-key".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: true,
                    auth_hint: "provider auth method metadata; secret value not imported".into(),
                    capabilities: vec!["provider-auth-choice".into()],
                }],
                provider_auth_flags: vec![ConfigCatalogItem {
                    id: "openai-api-key".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: true,
                    auth_hint: "setup/onboard secret flag name only; value redacted".into(),
                    capabilities: vec!["provider-auth-flag".into()],
                }],
                provider_auth_aliases: vec![ConfigCatalogItem {
                    id: "openai".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime provider auth alias map".into(),
                    capabilities: vec!["provider-auth-alias".into()],
                }],
                secret_targets: vec![ConfigCatalogItem {
                    id: "models.providers.*.apiKey".into(),
                    configured: false,
                    enabled: true,
                    active_default: false,
                    requires_auth: true,
                    auth_hint: "HeptaRuntime secret target registry path only; value redacted"
                        .into(),
                    capabilities: vec!["secret-target".into()],
                }],
                browser_profiles: vec![ConfigCatalogItem {
                    id: "default".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime browser profile runtime config".into(),
                    capabilities: vec!["browser-profile".into()],
                }],
                browser_config_options: vec![ConfigCatalogItem {
                    id: "driver".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint:
                        "HeptaRuntime browser runtime config default/resolved value metadata".into(),
                    capabilities: vec!["browser-config".into()],
                }],
                tool_policy_groups: vec![ConfigCatalogItem {
                    id: "core".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime tool policy group registry".into(),
                    capabilities: vec!["tool-policy-group".into()],
                }],
                tool_policy_profiles: vec![ConfigCatalogItem {
                    id: "coding".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime tool policy profile registry".into(),
                    capabilities: vec!["tool-policy-profile".into()],
                }],
                node_commands: vec![ConfigCatalogItem {
                    id: "system.run".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime paired-node invoke command registry".into(),
                    capabilities: vec!["node-command".into()],
                }],
                debug_proxy_coverage: vec![ConfigCatalogItem {
                    id: "playwright".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime debug proxy coverage registry".into(),
                    capabilities: vec!["debug-proxy-coverage".into()],
                }],
                setup_surfaces: vec![ConfigCatalogItem {
                    id: "openai".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime setup/provider onboarding surface metadata".into(),
                    capabilities: vec!["setup-surface".into()],
                }],
                channel_message_schemas: vec![ConfigCatalogItem {
                    id: "telegram.message".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "HeptaRuntime channel-specific message tool schema".into(),
                    capabilities: vec!["channel-message-tool-schema".into()],
                }],
                model_providers: vec![ConfigCatalogItem {
                    id: "local-provider".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["model-provider".into()],
                }],
                model_catalog_models: vec![ConfigCatalogItem {
                    id: "local-provider/local-chat".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["input:text".into()],
                }],
                search_engines: vec![ConfigCatalogItem {
                    id: "local-search".into(),
                    configured: true,
                    enabled: true,
                    active_default: true,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["web-search".into()],
                }],
                web_search_providers: vec![ConfigCatalogItem {
                    id: "local-search".into(),
                    configured: true,
                    enabled: true,
                    active_default: true,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["web-search".into()],
                }],
                web_fetch_providers: vec![ConfigCatalogItem {
                    id: "local-fetch".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["web-fetch".into()],
                }],
                image_generation_models: vec![ImageGenerationModelDescriptor {
                    provider: "local-image".into(),
                    model: "image-model".into(),
                    configured: true,
                    default_for_provider: true,
                    active_default: true,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    supports_editing: true,
                    max_reference_images: 1,
                    sizes: vec!["1024x1024".into()],
                    aspect_ratios: vec!["1:1".into()],
                    resolutions: vec!["1K".into()],
                }],
                video_generation_models: vec![MediaGenerationModelDescriptor {
                    media_kind: "video".into(),
                    provider: "local-video".into(),
                    model: "video-model".into(),
                    configured: true,
                    default_for_provider: true,
                    active_default: true,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    modes: vec!["generate".into()],
                    max_outputs: 1,
                    max_input_images: 1,
                    max_input_videos: 0,
                    max_duration_seconds: 8,
                    supported_duration_seconds: vec![4, 8],
                    sizes: vec!["1280x720".into()],
                    aspect_ratios: vec!["16:9".into()],
                    resolutions: vec!["720P".into()],
                    formats: vec!["mp4".into()],
                    supports_audio: true,
                    supports_lyrics: false,
                    supports_instrumental: false,
                }],
                music_generation_models: vec![MediaGenerationModelDescriptor {
                    media_kind: "music".into(),
                    provider: "local-music".into(),
                    model: "music-model".into(),
                    configured: true,
                    default_for_provider: true,
                    active_default: true,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    modes: vec!["generate".into()],
                    max_outputs: 1,
                    max_input_images: 0,
                    max_input_videos: 0,
                    max_duration_seconds: 0,
                    supported_duration_seconds: vec![],
                    sizes: vec![],
                    aspect_ratios: vec![],
                    resolutions: vec![],
                    formats: vec!["mp3".into()],
                    supports_audio: true,
                    supports_lyrics: true,
                    supports_instrumental: true,
                }],
                media_understanding_providers: vec![ConfigCatalogItem {
                    id: "local-vision".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["image".into(), "audio".into(), "video".into()],
                }],
                speech_providers: vec![ConfigCatalogItem {
                    id: "local-tts".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["speech".into()],
                }],
                embedding_providers: vec![ConfigCatalogItem {
                    id: "local".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["embedding".into()],
                }],
                channels: vec![ConfigCatalogItem {
                    id: "telegram".into(),
                    configured: true,
                    enabled: true,
                    active_default: true,
                    requires_auth: true,
                    auth_hint: "local channel pairing".into(),
                    capabilities: vec!["send".into()],
                }],
                acp_agents: vec![ConfigCatalogItem {
                    id: "codex".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "acpx".into(),
                    capabilities: vec!["session".into()],
                }],
                plugins: vec![ConfigCatalogItem {
                    id: "local-plugin".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["extension".into()],
                }],
                provider_extensions: vec![ConfigCatalogItem {
                    id: "local-plugin".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "local".into(),
                    capabilities: vec!["extension".into()],
                }],
                tools: vec![ConfigCatalogItem {
                    id: "exec".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "policy".into(),
                    capabilities: vec!["shell".into()],
                }],
                commands: vec![ConfigCatalogItem {
                    id: "native".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "auto".into(),
                    capabilities: vec!["slash".into()],
                }],
                capabilities: vec![ConfigCatalogItem {
                    id: "capability:model.run".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "metadata".into(),
                    capabilities: vec!["capability-command".into()],
                }],
                source_files: vec![ConfigCatalogItem {
                    id: "dist/test.js".into(),
                    configured: true,
                    enabled: true,
                    active_default: false,
                    requires_auth: false,
                    auth_hint: "source".into(),
                    capabilities: vec!["source-evidence".into()],
                }],
            }),
            sources: vec![],
        };

        assert!(manifest.has_config());
        assert!(manifest.has_auth());
        assert!(manifest.has_skills());
        assert!(manifest.has_startup_config());
        assert!(manifest.has_external_config_surface());
        assert!(manifest.has_optional_config_catalog());
        assert!(manifest.local_import_complete());
    }

    #[test]
    fn optional_catalog_accepts_legacy_hepta_runtime_version_label() {
        let catalog: OptionalConfigCatalogSummary = serde_json::from_value(serde_json::json!({
            "source": "hepta_runtime-dist-source-registry",
            "hepta_runtime_version": "2026.5.2",
            "source_ok": true,
            "catalog_count": 1,
            "search_engine_count": 1,
            "video_generation_provider_count": 1,
            "video_generation_model_count": 1,
            "music_generation_provider_count": 1,
            "music_generation_model_count": 1,
            "channel_count": 1,
            "acp_agent_count": 1,
            "plugin_count": 1,
            "tool_count": 1,
            "command_count": 1,
            "search_engines": [],
            "video_generation_models": [],
            "music_generation_models": [],
            "channels": [],
            "acp_agents": [],
            "plugins": [],
            "tools": [],
            "commands": []
        }))
        .expect("legacy catalog summary should deserialize");

        assert_eq!(catalog.hepta_runtime_version, "2026.5.2");
    }

    #[test]
    fn catalog_refresh_plan_is_report_only_until_confirmed() {
        let plan = LocalImportCatalogRefreshPlan::new(true, "2026.5.2", "2026.5.4");

        assert!(plan.refresh_needed);
        assert!(plan.requires_explicit_confirmation);
        assert_eq!(plan.local_import_script, "scripts/hepta-local-import.sh");
        assert!(plan.private_config_copy_possible);
        assert!(!plan.import_script_executed);
        assert!(!plan.private_config_copied);
        assert!(!plan.credential_value_read);
        assert!(!plan.network_call_attempted);
    }

    #[test]
    fn missing_status_is_not_complete_and_suggests_import_script() {
        let status = LocalConfigImportStatus::missing(
            ".hepta/local-import",
            ".hepta/local-import/manifest.json",
        );

        assert!(!status.manifest_present);
        assert!(!status.local_import_complete);
        assert!(
            status
                .error
                .as_deref()
                .expect("missing import should explain remediation")
                .contains("hepta-local-import.sh")
        );
    }
}
