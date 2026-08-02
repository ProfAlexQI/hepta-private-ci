use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::fs::Metadata;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use codex_core_plugins::contribution_point_abi::PluginContributionPointAbiMatrix;
use codex_core_plugins::contribution_point_abi::plan_hepta_system_plugin_contribution_point_abi;
use codex_core_plugins::contribution_point_loader_binding::PluginContributionPointLoaderBindingInputs;
use codex_core_plugins::contribution_point_loader_binding::PluginContributionPointLoaderBindingPlan;
use codex_core_plugins::contribution_point_loader_binding::declared_loader_manifest_fields_from_json;
use codex_core_plugins::contribution_point_loader_binding::plan_plugin_contribution_point_loader_binding;
use codex_core_plugins::lifecycle_state_machine::HeptaSystemPluginFixtureSummary;
use codex_core_plugins::lifecycle_state_machine::PluginLifecycleStateMachineInputs;
use codex_core_plugins::lifecycle_state_machine::PluginLifecycleStateMachinePlan;
use codex_core_plugins::lifecycle_state_machine::hepta_system_fixture_summary_from_json_at;
use codex_core_plugins::lifecycle_state_machine::plan_plugin_lifecycle_state_machine;
use codex_tools::PluginToolContributionInventoryPreviewPlan;
use codex_tools::PluginToolManifestPreflightInput;
use codex_tools::PluginToolManifestSchemaCutoverPreflightPlan;
use codex_tools::PluginToolRegistrySourceOfTruthDryRunPlan;
use codex_tools::plugin_tool_contribution_inventory_preview_plan;
use codex_tools::plugin_tool_manifest_preflight_input_from_manifest_json;
use codex_tools::plugin_tool_manifest_schema_cutover_preflight_plan;
use codex_tools::plugin_tool_registry_source_of_truth_dry_run_plan;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

pub(crate) const PLUGIN_COMPAT_REPORT_IDS: &[&str] = &[
    "hepta-systems-plugin-contribution-point-abi",
    "hepta-systems-plugin-contribution-point-loader-binding",
    "hepta-systems-plugin-lifecycle-state-machine",
    "hepta-systems-plugin-tool-contribution-inventory-preview",
    "hepta-systems-plugin-tool-manifest-schema-cutover-preflight",
];

const PLUGIN_ID: &str = "hepta-system@hepta-local";
const MANIFEST_RELATIVE_PATH: &str = "plugins/hepta-system/.codex-plugin/plugin.json";
const INTERNAL_REGISTRY_SOURCE_ID: &str = "hepta-internal-plugin-tool-registry-plan";
const MANIFEST_GENERATION: u64 = 410;
const ABI_GENERATION: u64 = 411;
const LOADER_GENERATION: u64 = 412;
const PREVIEW_GENERATION: u64 = 413;
const LIFECYCLE_GENERATION: u64 = 414;
const SCHEMA_GENERATION: u64 = 415;

/// The path boundary is intentionally explicit. The adapter rejects a symlink at the supplied
/// repository root, every manifest-relative component, every manifest-declared asset component,
/// and every entry below the declared skill root. On Windows it additionally rejects the reparse
/// attribute. Rust's portable path APIs do not provide handle-relative traversal, so a hostile
/// concurrent replacement between the pre/post metadata checks remains outside this boundary.
const PATH_INTEGRITY_BOUNDARY: &str = "component_walk_rejects_symlink_and_windows_reparse;pre_post_metadata_checked;concurrent_replacement_not_fd_bound";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpectedPathKind {
    Directory,
    RegularFile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PluginAssetKind {
    DirectoryTree,
    RegularFile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct PluginAssetObservation {
    pub manifest_field: String,
    pub relative_path: String,
    pub kind: PluginAssetKind,
    pub file_count: usize,
    pub byte_count: usize,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct PluginManifestObservation {
    pub plugin_id: String,
    pub manifest_relative_path: String,
    pub manifest_line_count: usize,
    pub manifest_byte_count: usize,
    pub manifest_sha256: String,
    pub declared_manifest_fields: Vec<String>,
    pub fixture: HeptaSystemPluginFixtureSummary,
    pub assets: Vec<PluginAssetObservation>,
    pub path_integrity_boundary: String,
}

impl PluginManifestObservation {
    fn has_integrity(&self) -> bool {
        self.plugin_id == PLUGIN_ID
            && self.manifest_relative_path == MANIFEST_RELATIVE_PATH
            && self.manifest_line_count > 0
            && self.manifest_byte_count > 0
            && digest_has_shape(&self.manifest_sha256)
            && self.declared_manifest_fields == ["skills", "mcpServers", "apps"]
            && self.fixture
                == HeptaSystemPluginFixtureSummary {
                    manifest_present: true,
                    skill_path_present: true,
                    mcp_servers_path_present: true,
                    apps_path_present: true,
                    skill_count: 1,
                    mcp_server_count: 1,
                    app_count: 1,
                    hook_count: 0,
                    tool_schema_count: 2,
                    permission_count: 2,
                    activation_event_count: 2,
                    tool_policy_count: 2,
                }
            && self.assets.len() == 3
            && self.assets[0].manifest_field == "skills"
            && self.assets[0].relative_path == "skills"
            && self.assets[0].kind == PluginAssetKind::DirectoryTree
            && self.assets[0].file_count == 1
            && self.assets[1].manifest_field == "mcpServers"
            && self.assets[1].relative_path == ".mcp.json"
            && self.assets[1].kind == PluginAssetKind::RegularFile
            && self.assets[1].file_count == 1
            && self.assets[2].manifest_field == "apps"
            && self.assets[2].relative_path == ".app.json"
            && self.assets[2].kind == PluginAssetKind::RegularFile
            && self.assets[2].file_count == 1
            && self.assets.iter().all(|asset| {
                asset.byte_count > 0
                    && asset.file_count > 0
                    && digest_has_shape(&asset.sha256)
                    && !Path::new(&asset.relative_path).is_absolute()
            })
            && self.path_integrity_boundary == PATH_INTEGRITY_BOUNDARY
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct PluginCompatSideEffects {
    pub app_connector_started: bool,
    pub approval_requested: bool,
    pub external_send_performed: bool,
    pub filesystem_written: bool,
    pub ledger_written: bool,
    pub local_storage_created: bool,
    pub mcp_server_started: bool,
    pub plugin_cache_mutated: bool,
    pub registry_mutated: bool,
    pub tool_invoked: bool,
}

impl PluginCompatSideEffects {
    const fn none() -> Self {
        Self {
            app_connector_started: false,
            approval_requested: false,
            external_send_performed: false,
            filesystem_written: false,
            ledger_written: false,
            local_storage_created: false,
            mcp_server_started: false,
            plugin_cache_mutated: false,
            registry_mutated: false,
            tool_invoked: false,
        }
    }

    fn is_closed(&self) -> bool {
        self == &Self::none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct PluginCompatSourceBinding {
    pub report_id: String,
    pub line_count: usize,
    pub sha256: String,
    pub generation: u64,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum PluginCompatPayload {
    ContributionPointAbi {
        plan: PluginContributionPointAbiMatrix,
    },
    ContributionPointLoaderBinding {
        plan: PluginContributionPointLoaderBindingPlan,
    },
    LifecycleStateMachine {
        plan: PluginLifecycleStateMachinePlan,
    },
    ToolContributionInventoryPreview {
        plan: PluginToolContributionInventoryPreviewPlan,
    },
    ToolManifestSchemaCutoverPreflight {
        internal_registry_plan: PluginToolRegistrySourceOfTruthDryRunPlan,
        manifest_input: PluginToolManifestPreflightInput,
        plan: PluginToolManifestSchemaCutoverPreflightPlan,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct PluginCompatReport {
    pub runtime: String,
    pub product: String,
    pub status: String,
    pub gate: String,
    pub schema_version: String,
    pub generation: u64,
    pub sequence: u64,
    pub sources: Vec<PluginCompatSourceBinding>,
    pub payload: PluginCompatPayload,
    pub legacy_business_fields: BTreeMap<String, Value>,
    pub production_authority_granted: bool,
    pub write_authority_granted: bool,
    pub ready_for_live_execution: bool,
    pub mutation_enabled: bool,
    pub side_effects: PluginCompatSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct PluginCompatReportSet {
    pub manifest: PluginManifestObservation,
    pub reports: Vec<PluginCompatReport>,
}

struct ValidatedPluginSource {
    manifest_json: String,
    manifest: PluginManifestObservation,
}

impl PluginCompatReport {
    fn new(
        gate: &str,
        generation: u64,
        sources: Vec<PluginCompatSourceBinding>,
        payload: PluginCompatPayload,
    ) -> Result<Self, String> {
        let mut report = Self {
            runtime: "hepta".to_string(),
            product: "Hepta".to_string(),
            status: "pass".to_string(),
            gate: gate.to_string(),
            schema_version: schema_for(gate)
                .ok_or_else(|| format!("unknown plugin compatibility report: {gate}"))?
                .to_string(),
            generation,
            sequence: generation,
            sources,
            payload,
            legacy_business_fields: BTreeMap::new(),
            production_authority_granted: false,
            write_authority_granted: false,
            ready_for_live_execution: false,
            mutation_enabled: false,
            side_effects: PluginCompatSideEffects::none(),
        };
        report.legacy_business_fields = expected_legacy_business_fields(&report)?;
        Ok(report)
    }

    fn render_legacy_line_protocol(&self) -> Result<String, String> {
        render_legacy_fields(&self.legacy_business_fields)
    }

    fn envelope_is_closed(&self) -> bool {
        self.runtime == "hepta"
            && self.product == "Hepta"
            && self.status == "pass"
            && schema_for(&self.gate) == Some(self.schema_version.as_str())
            && generation_for(&self.gate) == Some(self.generation)
            && self.sequence == self.generation
            && !self.sources.is_empty()
            && !self.production_authority_granted
            && !self.write_authority_granted
            && !self.ready_for_live_execution
            && !self.mutation_enabled
            && self.side_effects.is_closed()
            && self.sources.iter().all(source_binding_has_shape_integrity)
            && expected_legacy_business_fields(self)
                .is_ok_and(|expected| expected == self.legacy_business_fields)
            && serde_json::to_value(&self.payload)
                .is_ok_and(|payload| sensitive_fields_are_closed(&payload))
    }
}

impl PluginCompatReportSet {
    pub(crate) fn report(&self, id: &str) -> Option<&PluginCompatReport> {
        self.reports.iter().find(|report| report.gate == id)
    }

    pub(crate) fn has_integrity(&self) -> bool {
        macro_rules! require_ok {
            ($result:expr) => {
                match $result {
                    Ok(value) => value,
                    Err(_) => return false,
                }
            };
        }

        if !self.manifest.has_integrity()
            || self.reports.len() != PLUGIN_COMPAT_REPORT_IDS.len()
            || !self
                .reports
                .iter()
                .zip(PLUGIN_COMPAT_REPORT_IDS)
                .all(|(report, expected)| report.gate == *expected && report.envelope_is_closed())
        {
            return false;
        }

        let expected_abi = plan_hepta_system_plugin_contribution_point_abi(PLUGIN_ID);
        let Some(abi_report) = self.report(PLUGIN_COMPAT_REPORT_IDS[0]) else {
            return false;
        };
        let PluginCompatPayload::ContributionPointAbi { plan: abi } = &abi_report.payload else {
            return false;
        };
        if abi != &expected_abi || !abi_plan_is_ready(abi) {
            return false;
        }

        let declared_fields = match static_declared_manifest_fields(&self.manifest) {
            Some(fields) => fields,
            None => return false,
        };
        let expected_loader = plan_plugin_contribution_point_loader_binding(
            &expected_abi,
            &PluginContributionPointLoaderBindingInputs::synthetic_fixture(
                PLUGIN_ID,
                declared_fields,
            ),
        );
        let Some(loader_report) = self.report(PLUGIN_COMPAT_REPORT_IDS[1]) else {
            return false;
        };
        let PluginCompatPayload::ContributionPointLoaderBinding { plan: loader } =
            &loader_report.payload
        else {
            return false;
        };
        if loader != &expected_loader || !loader_plan_is_ready(loader) {
            return false;
        }

        let expected_preview = plugin_tool_contribution_inventory_preview_plan(PLUGIN_ID);
        let Some(preview_report) = self.report(PLUGIN_COMPAT_REPORT_IDS[3]) else {
            return false;
        };
        let PluginCompatPayload::ToolContributionInventoryPreview { plan: preview } =
            &preview_report.payload
        else {
            return false;
        };
        if preview != &expected_preview || !preview_plan_is_ready(preview) {
            return false;
        }

        let expected_lifecycle = plan_plugin_lifecycle_state_machine(
            &expected_abi,
            &expected_loader,
            &PluginLifecycleStateMachineInputs::synthetic_fixture(
                PLUGIN_ID,
                self.manifest.fixture.clone(),
                expected_preview.entries.len(),
            ),
        );
        let Some(lifecycle_report) = self.report(PLUGIN_COMPAT_REPORT_IDS[2]) else {
            return false;
        };
        let PluginCompatPayload::LifecycleStateMachine { plan: lifecycle } =
            &lifecycle_report.payload
        else {
            return false;
        };
        if lifecycle != &expected_lifecycle || !lifecycle_plan_is_ready(lifecycle) {
            return false;
        }

        let expected_registry =
            plugin_tool_registry_source_of_truth_dry_run_plan(&expected_preview);
        let expected_input = complete_manifest_input(&expected_preview);
        let expected_schema =
            plugin_tool_manifest_schema_cutover_preflight_plan(&expected_registry, &expected_input);
        let Some(schema_report) = self.report(PLUGIN_COMPAT_REPORT_IDS[4]) else {
            return false;
        };
        let PluginCompatPayload::ToolManifestSchemaCutoverPreflight {
            internal_registry_plan,
            manifest_input,
            plan: schema,
        } = &schema_report.payload
        else {
            return false;
        };
        if internal_registry_plan != &expected_registry
            || manifest_input != &expected_input
            || schema != &expected_schema
            || !registry_plan_is_ready(internal_registry_plan)
            || !schema_plan_is_ready(schema)
        {
            return false;
        }

        let manifest_source = require_ok!(source_binding_from_serializable(
            MANIFEST_RELATIVE_PATH,
            MANIFEST_GENERATION,
            &self.manifest,
        ));
        if abi_report.sources != [manifest_source.clone()]
            || loader_report.sources
                != [
                    require_ok!(source_binding_from_report(abi_report)),
                    manifest_source.clone(),
                ]
            || preview_report.sources
                != [
                    require_ok!(source_binding_from_report(loader_report)),
                    manifest_source.clone(),
                ]
            || lifecycle_report.sources
                != [
                    require_ok!(source_binding_from_report(abi_report)),
                    require_ok!(source_binding_from_report(loader_report)),
                    require_ok!(source_binding_from_report(preview_report)),
                    manifest_source.clone(),
                ]
            || schema_report.sources
                != [
                    require_ok!(source_binding_from_report(preview_report)),
                    manifest_source,
                    require_ok!(source_binding_from_serializable(
                        INTERNAL_REGISTRY_SOURCE_ID,
                        PREVIEW_GENERATION,
                        internal_registry_plan,
                    )),
                ]
        {
            return false;
        }

        true
    }
}

pub(crate) fn build_plugin_compat_reports(
    repo_root: &Path,
    manifest_bytes: &[u8],
) -> Result<PluginCompatReportSet, String> {
    let source = validate_plugin_source(repo_root, manifest_bytes)?;
    let abi = plan_hepta_system_plugin_contribution_point_abi(PLUGIN_ID);
    let declared_fields = declared_loader_manifest_fields_from_json(&source.manifest_json);
    let loader = plan_plugin_contribution_point_loader_binding(
        &abi,
        &PluginContributionPointLoaderBindingInputs::synthetic_fixture(PLUGIN_ID, declared_fields),
    );
    let preview = plugin_tool_contribution_inventory_preview_plan(PLUGIN_ID);
    let lifecycle = plan_plugin_lifecycle_state_machine(
        &abi,
        &loader,
        &PluginLifecycleStateMachineInputs::synthetic_fixture(
            PLUGIN_ID,
            source.manifest.fixture.clone(),
            preview.entries.len(),
        ),
    );
    let internal_registry_plan = plugin_tool_registry_source_of_truth_dry_run_plan(&preview);
    let manifest_input =
        plugin_tool_manifest_preflight_input_from_manifest_json(source.manifest_json.as_str());
    let schema = plugin_tool_manifest_schema_cutover_preflight_plan(
        &internal_registry_plan,
        &manifest_input,
    );

    if !abi_plan_is_ready(&abi)
        || !loader_plan_is_ready(&loader)
        || !preview_plan_is_ready(&preview)
        || !lifecycle_plan_is_ready(&lifecycle)
        || !registry_plan_is_ready(&internal_registry_plan)
        || !manifest_input_is_complete(&manifest_input, &preview)
        || !schema_plan_is_ready(&schema)
    {
        return Err(
            "plugin typed compatibility source failed closed planner integrity".to_string(),
        );
    }

    let manifest_source = source_binding_from_serializable(
        MANIFEST_RELATIVE_PATH,
        MANIFEST_GENERATION,
        &source.manifest,
    )?;
    let abi_report = PluginCompatReport::new(
        PLUGIN_COMPAT_REPORT_IDS[0],
        ABI_GENERATION,
        vec![manifest_source.clone()],
        PluginCompatPayload::ContributionPointAbi { plan: abi },
    )?;
    let loader_report = PluginCompatReport::new(
        PLUGIN_COMPAT_REPORT_IDS[1],
        LOADER_GENERATION,
        vec![
            source_binding_from_report(&abi_report)?,
            manifest_source.clone(),
        ],
        PluginCompatPayload::ContributionPointLoaderBinding { plan: loader },
    )?;
    let preview_report = PluginCompatReport::new(
        PLUGIN_COMPAT_REPORT_IDS[3],
        PREVIEW_GENERATION,
        vec![
            source_binding_from_report(&loader_report)?,
            manifest_source.clone(),
        ],
        PluginCompatPayload::ToolContributionInventoryPreview { plan: preview },
    )?;
    let lifecycle_report = PluginCompatReport::new(
        PLUGIN_COMPAT_REPORT_IDS[2],
        LIFECYCLE_GENERATION,
        vec![
            source_binding_from_report(&abi_report)?,
            source_binding_from_report(&loader_report)?,
            source_binding_from_report(&preview_report)?,
            manifest_source.clone(),
        ],
        PluginCompatPayload::LifecycleStateMachine { plan: lifecycle },
    )?;
    let schema_report = PluginCompatReport::new(
        PLUGIN_COMPAT_REPORT_IDS[4],
        SCHEMA_GENERATION,
        vec![
            source_binding_from_report(&preview_report)?,
            manifest_source,
            source_binding_from_serializable(
                INTERNAL_REGISTRY_SOURCE_ID,
                PREVIEW_GENERATION,
                &internal_registry_plan,
            )?,
        ],
        PluginCompatPayload::ToolManifestSchemaCutoverPreflight {
            internal_registry_plan,
            manifest_input,
            plan: schema,
        },
    )?;

    let reports = PluginCompatReportSet {
        manifest: source.manifest,
        reports: vec![
            abi_report,
            loader_report,
            lifecycle_report,
            preview_report,
            schema_report,
        ],
    };
    if !reports.has_integrity() {
        return Err("plugin typed compatibility report chain failed integrity".to_string());
    }
    Ok(reports)
}

fn validate_plugin_source(
    repo_root: &Path,
    manifest_bytes: &[u8],
) -> Result<ValidatedPluginSource, String> {
    validate_repo_root(repo_root)?;
    let manifest_path = validate_existing_relative_path(
        repo_root,
        Path::new(MANIFEST_RELATIVE_PATH),
        ExpectedPathKind::RegularFile,
    )?;
    let disk_manifest_bytes = read_regular_file_checked(&manifest_path)?;
    if disk_manifest_bytes != manifest_bytes {
        return Err("caller manifest bytes do not match the validated repository file".to_string());
    }
    let manifest_json = std::str::from_utf8(manifest_bytes)
        .map_err(|error| format!("plugin manifest is not UTF-8: {error}"))?
        .to_string();
    let Value::Object(manifest) = serde_json::from_str::<Value>(&manifest_json)
        .map_err(|error| format!("plugin manifest is not valid JSON: {error}"))?
    else {
        return Err("plugin manifest root must be a JSON object".to_string());
    };
    if manifest.get("name").and_then(Value::as_str) != Some("hepta-system") {
        return Err("plugin manifest name must be hepta-system".to_string());
    }

    let plugin_root = manifest_path
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "plugin manifest does not have a plugin root".to_string())?;
    let asset_specs = [
        ("skills", ExpectedPathKind::Directory),
        ("mcpServers", ExpectedPathKind::RegularFile),
        ("apps", ExpectedPathKind::RegularFile),
    ];
    let mut assets = Vec::new();
    for (manifest_field, expected_kind) in asset_specs {
        let declared_path = manifest
            .get(manifest_field)
            .and_then(Value::as_str)
            .ok_or_else(|| {
                format!("plugin manifest field {manifest_field} must be a relative path string")
            })?;
        assets.push(observe_plugin_asset(
            plugin_root,
            manifest_field,
            declared_path,
            expected_kind,
        )?);
    }

    let fixture = hepta_system_fixture_summary_from_json_at(&manifest_json, plugin_root);
    let declared_manifest_fields = declared_loader_manifest_fields_from_json(&manifest_json)
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let observation = PluginManifestObservation {
        plugin_id: PLUGIN_ID.to_string(),
        manifest_relative_path: MANIFEST_RELATIVE_PATH.to_string(),
        manifest_line_count: manifest_json.lines().count(),
        manifest_byte_count: manifest_bytes.len(),
        manifest_sha256: sha256_hex(manifest_bytes),
        declared_manifest_fields,
        fixture,
        assets,
        path_integrity_boundary: PATH_INTEGRITY_BOUNDARY.to_string(),
    };
    if !observation.has_integrity() {
        return Err("plugin manifest fixture failed typed source integrity".to_string());
    }
    Ok(ValidatedPluginSource {
        manifest_json,
        manifest: observation,
    })
}

fn validate_repo_root(repo_root: &Path) -> Result<(), String> {
    if !repo_root.is_absolute() {
        return Err("plugin repository root must be absolute".to_string());
    }
    let metadata = fs::symlink_metadata(repo_root)
        .map_err(|error| format!("cannot inspect plugin repository root: {error}"))?;
    if metadata_is_link_like(&metadata) || !metadata.file_type().is_dir() {
        return Err("plugin repository root must be a non-link directory".to_string());
    }
    Ok(())
}

fn observe_plugin_asset(
    plugin_root: &Path,
    manifest_field: &str,
    declared_path: &str,
    expected_kind: ExpectedPathKind,
) -> Result<PluginAssetObservation, String> {
    let (relative_path, portable_relative_path) = normalize_relative_path(declared_path)?;
    let asset_path = validate_existing_relative_path(plugin_root, &relative_path, expected_kind)?;
    match expected_kind {
        ExpectedPathKind::RegularFile => {
            let bytes = read_regular_file_checked(&asset_path)?;
            Ok(PluginAssetObservation {
                manifest_field: manifest_field.to_string(),
                relative_path: portable_relative_path,
                kind: PluginAssetKind::RegularFile,
                file_count: 1,
                byte_count: bytes.len(),
                sha256: sha256_hex(&bytes),
            })
        }
        ExpectedPathKind::Directory => {
            let mut files = Vec::new();
            collect_regular_tree(&asset_path, &asset_path, &mut files)?;
            if files.is_empty() {
                return Err(format!(
                    "plugin manifest field {manifest_field} resolves to an empty directory"
                ));
            }
            let mut hasher = Sha256::new();
            let mut byte_count = 0;
            for (relative_file, bytes) in &files {
                hasher.update((relative_file.len() as u64).to_le_bytes());
                hasher.update(relative_file.as_bytes());
                hasher.update((bytes.len() as u64).to_le_bytes());
                hasher.update(bytes);
                byte_count += bytes.len();
            }
            Ok(PluginAssetObservation {
                manifest_field: manifest_field.to_string(),
                relative_path: portable_relative_path,
                kind: PluginAssetKind::DirectoryTree,
                file_count: files.len(),
                byte_count,
                sha256: format!("{:x}", hasher.finalize()),
            })
        }
    }
}

fn collect_regular_tree(
    root: &Path,
    directory: &Path,
    output: &mut Vec<(String, Vec<u8>)>,
) -> Result<(), String> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot read plugin asset directory: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("cannot enumerate plugin asset directory: {error}"))?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("cannot inspect plugin asset entry: {error}"))?;
        if metadata_is_link_like(&metadata) {
            return Err(format!(
                "plugin asset entry is a symlink or reparse point: {}",
                path.display()
            ));
        }
        if metadata.file_type().is_dir() {
            collect_regular_tree(root, &path, output)?;
        } else if metadata.file_type().is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| "plugin asset escaped its declared root".to_string())?;
            output.push((portable_path(relative)?, read_regular_file_checked(&path)?));
        } else {
            return Err(format!(
                "plugin asset entry is not a regular file or directory: {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn validate_existing_relative_path(
    base: &Path,
    relative_path: &Path,
    expected_kind: ExpectedPathKind,
) -> Result<PathBuf, String> {
    if relative_path.is_absolute() {
        return Err("plugin path must be relative".to_string());
    }
    let components = relative_path
        .components()
        .filter_map(|component| match component {
            Component::CurDir => None,
            Component::Normal(component) => Some(Ok(component.to_owned())),
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                Some(Err("plugin path contains an escaping component".to_string()))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    if components.is_empty() {
        return Err("plugin path must contain a normal component".to_string());
    }

    let mut current = base.to_path_buf();
    for (index, component) in components.iter().enumerate() {
        current.push(component);
        let metadata = fs::symlink_metadata(&current).map_err(|error| {
            format!(
                "cannot inspect plugin path component {}: {error}",
                current.display()
            )
        })?;
        if metadata_is_link_like(&metadata) {
            return Err(format!(
                "plugin path component is a symlink or reparse point: {}",
                current.display()
            ));
        }
        let is_final = index + 1 == components.len();
        if !is_final && !metadata.file_type().is_dir() {
            return Err(format!(
                "plugin path intermediate component is not a directory: {}",
                current.display()
            ));
        }
        if is_final {
            let expected_shape = match expected_kind {
                ExpectedPathKind::Directory => metadata.file_type().is_dir(),
                ExpectedPathKind::RegularFile => metadata.file_type().is_file(),
            };
            if !expected_shape {
                return Err(format!(
                    "plugin path final component has the wrong file type: {}",
                    current.display()
                ));
            }
        }
    }

    let canonical_base = fs::canonicalize(base)
        .map_err(|error| format!("cannot canonicalize plugin path base: {error}"))?;
    let canonical_current = fs::canonicalize(&current)
        .map_err(|error| format!("cannot canonicalize plugin path: {error}"))?;
    if !canonical_current.starts_with(&canonical_base) {
        return Err("plugin path canonical target escaped its base".to_string());
    }
    Ok(current)
}

fn normalize_relative_path(raw_path: &str) -> Result<(PathBuf, String), String> {
    let path = Path::new(raw_path);
    if path.is_absolute() {
        return Err("plugin manifest declared an absolute asset path".to_string());
    }
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(component) => normalized.push(component),
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err("plugin manifest asset path escapes the plugin root".to_string());
            }
        }
    }
    if normalized.as_os_str().is_empty() {
        return Err("plugin manifest asset path is empty".to_string());
    }
    let portable = portable_path(&normalized)?;
    Ok((normalized, portable))
}

fn portable_path(path: &Path) -> Result<String, String> {
    path.components()
        .map(|component| match component {
            Component::Normal(component) => component
                .to_str()
                .map(str::to_string)
                .ok_or_else(|| "plugin path component is not UTF-8".to_string()),
            _ => Err("plugin portable path contains a non-normal component".to_string()),
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|components| components.join("/"))
}

fn read_regular_file_checked(path: &Path) -> Result<Vec<u8>, String> {
    let before = fs::symlink_metadata(path)
        .map_err(|error| format!("cannot inspect plugin file before read: {error}"))?;
    if metadata_is_link_like(&before) || !before.file_type().is_file() {
        return Err(format!(
            "plugin file is not a non-link regular file: {}",
            path.display()
        ));
    }
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "cannot read validated plugin file {}: {error}",
            path.display()
        )
    })?;
    let after = fs::symlink_metadata(path)
        .map_err(|error| format!("cannot inspect plugin file after read: {error}"))?;
    if metadata_is_link_like(&after)
        || !after.file_type().is_file()
        || before.len() != after.len()
        || after.len() != bytes.len() as u64
    {
        return Err(format!(
            "plugin file metadata changed across read: {}",
            path.display()
        ));
    }
    Ok(bytes)
}

fn metadata_is_link_like(metadata: &Metadata) -> bool {
    metadata.file_type().is_symlink() || metadata_is_windows_reparse(metadata)
}

#[cfg(windows)]
fn metadata_is_windows_reparse(metadata: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn metadata_is_windows_reparse(_metadata: &Metadata) -> bool {
    false
}

fn abi_plan_is_ready(plan: &PluginContributionPointAbiMatrix) -> bool {
    plan.plugin_id == PLUGIN_ID
        && plan.abi_ready
        && plan.all_runtime_execution_disabled
        && plan.all_live_paths_blocked
        && !plan.live_mutation_ready
        && serde_json::to_value(plan).is_ok_and(|value| sensitive_fields_are_closed(&value))
}

fn loader_plan_is_ready(plan: &PluginContributionPointLoaderBindingPlan) -> bool {
    plan.plugin_id == PLUGIN_ID
        && plan.loader_contract_ready
        && plan.current_fixture_binding_ready
        && plan.binding_ready
        && plan.all_declared_manifest_paths_bound
        && plan.all_live_paths_blocked
        && !plan.tool_registry_registration_enabled
        && !plan.runtime_execution_enabled
        && !plan.local_storage_created
        && !plan.live_mutation_ready
        && serde_json::to_value(plan).is_ok_and(|value| sensitive_fields_are_closed(&value))
}

fn preview_plan_is_ready(plan: &PluginToolContributionInventoryPreviewPlan) -> bool {
    plan.plugin_id == PLUGIN_ID
        && plan.preview_ready
        && plan.entries.len() == 2
        && plan.all_candidates_have_schema
        && plan.all_candidates_have_risk_metadata
        && plan.all_candidates_require_ledger
        && plan.mutating_candidates_require_approval
        && plan.all_candidates_have_guard_route
        && !plan.inventory_registration_enabled
        && !plan.tool_invocation_enabled
        && !plan.ledger_written
        && !plan.approval_requested
        && !plan.mcp_server_started
        && !plan.app_connector_started
        && !plan.live_mutation_ready
        && serde_json::to_value(plan).is_ok_and(|value| sensitive_fields_are_closed(&value))
}

fn lifecycle_plan_is_ready(plan: &PluginLifecycleStateMachinePlan) -> bool {
    plan.plugin_id == PLUGIN_ID
        && plan.lifecycle_state_machine_ready
        && plan.lifecycle_phase_summary_ready
        && plan.source_of_truth_ready
        && plan.fixture_shape_ready
        && plan.fixture_policy_metadata_ready
        && plan.all_live_paths_blocked
        && !plan.tool_registry_registration_enabled
        && !plan.tool_invocation_enabled
        && !plan.ledger_written
        && !plan.approval_requested
        && !plan.plugin_cache_mutated
        && !plan.local_storage_created
        && !plan.live_mutation_ready
        && serde_json::to_value(plan).is_ok_and(|value| sensitive_fields_are_closed(&value))
}

fn registry_plan_is_ready(plan: &PluginToolRegistrySourceOfTruthDryRunPlan) -> bool {
    plan.plugin_id == PLUGIN_ID
        && plan.registry_source_of_truth_dry_run_ready
        && plan.all_candidate_ids_unique
        && plan.all_preview_candidates_bound_to_registry
        && plan.all_candidates_have_schema
        && plan.all_candidates_have_risk_metadata
        && plan.all_candidates_require_ledger
        && plan.mutating_candidates_require_approval
        && plan.all_candidates_have_guard_route
        && !plan.registry_source_of_truth_enabled
        && !plan.tool_registry_registration_enabled
        && !plan.tool_invocation_enabled
        && !plan.ledger_written
        && !plan.approval_requested
        && !plan.mcp_server_started
        && !plan.app_connector_started
        && !plan.live_mutation_ready
        && plan.side_effect_free
        && serde_json::to_value(plan).is_ok_and(|value| sensitive_fields_are_closed(&value))
}

fn schema_plan_is_ready(plan: &PluginToolManifestSchemaCutoverPreflightPlan) -> bool {
    plan.plugin_id == PLUGIN_ID
        && plan.manifest_schema_cutover_preflight_ready
        && plan.registration_cutover_allowed
        && plan.all_manifest_declarations_bound_to_planned_candidates
        && plan.missing_manifest_precondition_count == 0
        && plan.registration_precondition_satisfied_count == plan.planned_candidate_count
        && !plan.registration_execution_enabled
        && !plan.tool_invocation_enabled
        && !plan.ledger_written
        && !plan.approval_requested
        && !plan.mcp_server_started
        && !plan.app_connector_started
        && !plan.live_mutation_ready
        && plan.side_effect_free
        && serde_json::to_value(plan).is_ok_and(|value| sensitive_fields_are_closed(&value))
}

fn complete_manifest_input(
    preview: &PluginToolContributionInventoryPreviewPlan,
) -> PluginToolManifestPreflightInput {
    let candidate_ids = preview
        .entries
        .iter()
        .map(|entry| entry.candidate_tool_id.to_string())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    PluginToolManifestPreflightInput {
        contribution_candidate_ids: candidate_ids.clone(),
        tool_schemas: candidate_ids.clone(),
        permissions: candidate_ids.clone(),
        activation_events: candidate_ids.clone(),
        tool_policies: candidate_ids.clone(),
        schema_complete_candidate_ids: candidate_ids.clone(),
        policy_complete_candidate_ids: candidate_ids,
    }
}

fn manifest_input_is_complete(
    input: &PluginToolManifestPreflightInput,
    preview: &PluginToolContributionInventoryPreviewPlan,
) -> bool {
    input == &complete_manifest_input(preview)
}

fn static_declared_manifest_fields(
    manifest: &PluginManifestObservation,
) -> Option<Vec<&'static str>> {
    manifest
        .declared_manifest_fields
        .iter()
        .map(|field| match field.as_str() {
            "skills" => Some("skills"),
            "mcpServers" => Some("mcpServers"),
            "apps" => Some("apps"),
            "hooks" => Some("hooks"),
            _ => None,
        })
        .collect()
}

fn schema_for(gate: &str) -> Option<&'static str> {
    match gate {
        "hepta-systems-plugin-contribution-point-abi" => {
            Some("plugin_contribution_point_abi_typed_v1")
        }
        "hepta-systems-plugin-contribution-point-loader-binding" => {
            Some("plugin_contribution_point_loader_binding_typed_v1")
        }
        "hepta-systems-plugin-lifecycle-state-machine" => {
            Some("plugin_lifecycle_state_machine_typed_v1")
        }
        "hepta-systems-plugin-tool-contribution-inventory-preview" => {
            Some("plugin_tool_contribution_inventory_preview_typed_v1")
        }
        "hepta-systems-plugin-tool-manifest-schema-cutover-preflight" => {
            Some("plugin_tool_manifest_schema_cutover_preflight_typed_v1")
        }
        _ => None,
    }
}

fn generation_for(gate: &str) -> Option<u64> {
    match gate {
        "hepta-systems-plugin-contribution-point-abi" => Some(ABI_GENERATION),
        "hepta-systems-plugin-contribution-point-loader-binding" => Some(LOADER_GENERATION),
        "hepta-systems-plugin-lifecycle-state-machine" => Some(LIFECYCLE_GENERATION),
        "hepta-systems-plugin-tool-contribution-inventory-preview" => Some(PREVIEW_GENERATION),
        "hepta-systems-plugin-tool-manifest-schema-cutover-preflight" => Some(SCHEMA_GENERATION),
        _ => None,
    }
}

fn source_binding_from_report(
    report: &PluginCompatReport,
) -> Result<PluginCompatSourceBinding, String> {
    let protocol = report.render_legacy_line_protocol()?;
    Ok(PluginCompatSourceBinding {
        report_id: report.gate.clone(),
        line_count: protocol.lines().count(),
        sha256: sha256_hex(protocol.as_bytes()),
        generation: report.generation,
        sequence: report.sequence,
    })
}

fn source_binding_from_serializable<T: Serialize>(
    report_id: &str,
    generation: u64,
    source: &T,
) -> Result<PluginCompatSourceBinding, String> {
    let fields = flattened_serializable_fields(source)?;
    let protocol = render_legacy_fields(&fields)?;
    Ok(PluginCompatSourceBinding {
        report_id: report_id.to_string(),
        line_count: protocol.lines().count(),
        sha256: sha256_hex(protocol.as_bytes()),
        generation,
        sequence: generation,
    })
}

fn source_binding_has_shape_integrity(source: &PluginCompatSourceBinding) -> bool {
    !source.report_id.is_empty()
        && source.line_count > 0
        && source.generation > 0
        && source.sequence > 0
        && digest_has_shape(&source.sha256)
}

fn expected_legacy_business_fields(
    report: &PluginCompatReport,
) -> Result<BTreeMap<String, Value>, String> {
    let mut fields = BTreeMap::from([
        ("result".to_string(), Value::String("pass".to_string())),
        (
            "generation".to_string(),
            Value::Number(report.generation.into()),
        ),
        (
            "sequence".to_string(),
            Value::Number(report.sequence.into()),
        ),
        ("production_authority".to_string(), Value::Bool(false)),
        ("write_authority".to_string(), Value::Bool(false)),
        ("live_execution".to_string(), Value::Bool(false)),
        ("mutation".to_string(), Value::Bool(false)),
    ]);
    append_source_fields(&mut fields, &report.sources);
    append_flattened(&mut fields, "typed", &report.payload)?;
    Ok(fields)
}

fn append_source_fields(
    fields: &mut BTreeMap<String, Value>,
    sources: &[PluginCompatSourceBinding],
) {
    fields.insert(
        "source_count".to_string(),
        Value::Number(sources.len().into()),
    );
    for (index, source) in sources.iter().enumerate() {
        let prefix = format!("source.{index}");
        fields.insert(
            format!("{prefix}.report_id"),
            Value::String(source.report_id.clone()),
        );
        fields.insert(
            format!("{prefix}.line_count"),
            Value::Number(source.line_count.into()),
        );
        fields.insert(
            format!("{prefix}.sha256"),
            Value::String(source.sha256.clone()),
        );
        fields.insert(
            format!("{prefix}.generation"),
            Value::Number(source.generation.into()),
        );
        fields.insert(
            format!("{prefix}.sequence"),
            Value::Number(source.sequence.into()),
        );
    }
}

fn append_flattened<T: Serialize>(
    target: &mut BTreeMap<String, Value>,
    prefix: &str,
    value: &T,
) -> Result<(), String> {
    let source = serde_json::to_value(value)
        .map_err(|error| format!("cannot serialize plugin typed source: {error}"))?;
    flatten_json(prefix, &source, target)
}

fn flattened_serializable_fields<T: Serialize>(
    source: &T,
) -> Result<BTreeMap<String, Value>, String> {
    let value = serde_json::to_value(source)
        .map_err(|error| format!("cannot serialize plugin source binding: {error}"))?;
    let mut fields = BTreeMap::new();
    flatten_json("source", &value, &mut fields)?;
    Ok(fields)
}

fn flatten_json(
    prefix: &str,
    value: &Value,
    output: &mut BTreeMap<String, Value>,
) -> Result<(), String> {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            if prefix.is_empty() {
                return Err("plugin legacy projection has an empty key".to_string());
            }
            output.insert(prefix.to_string(), value.clone());
        }
        Value::Array(values) => {
            output.insert(
                format!("{prefix}.count"),
                Value::Number(values.len().into()),
            );
            for (index, value) in values.iter().enumerate() {
                flatten_json(&format!("{prefix}.{index}"), value, output)?;
            }
        }
        Value::Object(values) => {
            if values.is_empty() {
                output.insert(format!("{prefix}.present"), Value::Bool(false));
            }
            for (key, value) in values {
                if !key
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
                {
                    return Err(format!(
                        "plugin typed source exposes unsafe legacy key segment: {key}"
                    ));
                }
                let child = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten_json(&child, value, output)?;
            }
        }
    }
    Ok(())
}

fn render_legacy_fields(fields: &BTreeMap<String, Value>) -> Result<String, String> {
    if fields.is_empty() {
        return Err("plugin legacy projection is empty".to_string());
    }
    let mut protocol = String::new();
    for (key, value) in fields {
        if key.is_empty()
            || !key.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'_' | b'.' | b'-')
            })
        {
            return Err(format!(
                "plugin legacy projection exposes unsafe key: {key}"
            ));
        }
        let rendered = match value {
            Value::Null => "null".to_string(),
            Value::Bool(value) => value.to_string(),
            Value::Number(value) => value.to_string(),
            Value::String(value) => {
                if value.contains(['\n', '\r', '=']) {
                    return Err(format!(
                        "plugin legacy projection exposes an injectable value for {key}"
                    ));
                }
                value.clone()
            }
            Value::Array(_) | Value::Object(_) => {
                return Err(format!(
                    "plugin legacy projection field {key} is not scalar"
                ));
            }
        };
        protocol.push_str(key);
        protocol.push('=');
        protocol.push_str(&rendered);
        protocol.push('\n');
    }
    Ok(protocol)
}

fn sensitive_fields_are_closed(value: &Value) -> bool {
    match value {
        Value::Array(values) => values.iter().all(sensitive_fields_are_closed),
        Value::Object(values) => values.iter().all(|(key, value)| {
            let sensitive_boolean = matches!(
                key.as_str(),
                "app_connector_started"
                    | "approval_request_enabled"
                    | "approval_requested"
                    | "credential_read_enabled"
                    | "external_mutation_enabled"
                    | "gateway_mutation_enabled"
                    | "inventory_registration_enabled"
                    | "ledger_write_enabled"
                    | "ledger_written"
                    | "live_mutation_ready"
                    | "local_storage_created"
                    | "mcp_server_started"
                    | "native_post_mutation_enabled"
                    | "plugin_cache_mutated"
                    | "provider_call_enabled"
                    | "registration_execution_enabled"
                    | "registry_source_of_truth_enabled"
                    | "runtime_execution_enabled"
                    | "source_of_truth_registration_enabled"
                    | "tool_invocation_enabled"
                    | "tool_registry_registration_enabled"
            );
            (!sensitive_boolean || value == &Value::Bool(false))
                && sensitive_fields_are_closed(value)
        }),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => true,
    }
}

fn digest_has_shape(digest: &str) -> bool {
    digest.len() == 64
        && digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Map;
    use serde_json::json;
    use tempfile::TempDir;

    fn candidate_ids() -> Vec<String> {
        plugin_tool_contribution_inventory_preview_plan(PLUGIN_ID)
            .entries
            .into_iter()
            .map(|entry| entry.candidate_tool_id.to_string())
            .collect()
    }

    fn complete_manifest() -> Value {
        let ids = candidate_ids();
        let tool_schemas = ids
            .iter()
            .map(|id| {
                (
                    id.clone(),
                    json!({
                        "inputSchema": {"type": "object"},
                        "outputSchema": {"type": "object"}
                    }),
                )
            })
            .collect::<Map<_, _>>();
        let permissions = ids
            .iter()
            .map(|id| (id.clone(), json!({"scope": "plugin_test"})))
            .collect::<Map<_, _>>();
        let activation_events = ids
            .iter()
            .map(|id| (id.clone(), json!({"event": "on_use"})))
            .collect::<Map<_, _>>();
        let tool_policies = ids
            .iter()
            .map(|id| {
                (
                    id.clone(),
                    json!({
                        "approval": "required",
                        "ledger": "required",
                        "timeoutMs": 30000
                    }),
                )
            })
            .collect::<Map<_, _>>();
        json!({
            "name": "hepta-system",
            "version": "0.0.0-fixture",
            "skills": "./skills",
            "mcpServers": "./.mcp.json",
            "apps": "./.app.json",
            "toolSchemas": tool_schemas,
            "permissions": permissions,
            "activationEvents": activation_events,
            "toolPolicies": tool_policies
        })
    }

    fn write_assets(repo_root: &Path) {
        let plugin_root = repo_root.join("plugins/hepta-system");
        fs::create_dir_all(plugin_root.join(".codex-plugin"))
            .expect("manifest directory must be created");
        fs::create_dir_all(plugin_root.join("skills/hepta-system-status"))
            .expect("skill directory must be created");
        fs::write(
            plugin_root.join("skills/hepta-system-status/SKILL.md"),
            "# Hepta system status\n\nRead-only fixture.\n",
        )
        .expect("skill fixture must be written");
        fs::write(
            plugin_root.join(".mcp.json"),
            serde_json::to_vec_pretty(&json!({
                "mcpServers": {"hepta_system_local_mcp": {"command": "disabled"}}
            }))
            .expect("MCP fixture must serialize"),
        )
        .expect("MCP fixture must be written");
        fs::write(
            plugin_root.join(".app.json"),
            serde_json::to_vec_pretty(&json!({
                "apps": {"hepta_system_local_app": {"enabled": false}}
            }))
            .expect("app fixture must serialize"),
        )
        .expect("app fixture must be written");
    }

    fn write_fixture(repo_root: &Path, manifest: &Value) -> Vec<u8> {
        write_assets(repo_root);
        let bytes = serde_json::to_vec_pretty(manifest).expect("manifest fixture must serialize");
        fs::write(repo_root.join(MANIFEST_RELATIVE_PATH), bytes.as_slice())
            .expect("manifest fixture must be written");
        bytes
    }

    fn report_index(id: &str) -> usize {
        PLUGIN_COMPAT_REPORT_IDS
            .iter()
            .position(|candidate| *candidate == id)
            .expect("plugin report id must exist")
    }

    fn flip_digest(digest: &mut String) {
        let replacement = if digest.starts_with('0') { '1' } else { '0' };
        digest.replace_range(..1, &replacement.to_string());
    }

    #[test]
    fn plugin_compat_report_valid_and_relocated_roots_are_exactly_equivalent() {
        let first_root = TempDir::new().expect("first repo root must be created");
        let second_root = TempDir::new().expect("second repo root must be created");
        let manifest = complete_manifest();
        let first_bytes = write_fixture(first_root.path(), &manifest);
        let second_bytes = write_fixture(second_root.path(), &manifest);

        let first = build_plugin_compat_reports(first_root.path(), &first_bytes)
            .expect("first typed report set must build");
        let second = build_plugin_compat_reports(second_root.path(), &second_bytes)
            .expect("relocated typed report set must build");

        assert_eq!(first, second);
        assert!(first.has_integrity());
        assert_eq!(first.reports.len(), 5);
        assert_eq!(
            first
                .reports
                .iter()
                .map(|report| report.gate.as_str())
                .collect::<Vec<_>>(),
            PLUGIN_COMPAT_REPORT_IDS
        );
        for report in &first.reports {
            assert!(!report.production_authority_granted);
            assert!(!report.write_authority_granted);
            assert!(!report.ready_for_live_execution);
            assert!(!report.mutation_enabled);
            assert!(report.side_effects.is_closed());
            assert!(report.render_legacy_line_protocol().is_ok());
        }
        let encoded = serde_json::to_string(&first).expect("typed reports must serialize");
        assert!(!encoded.contains(first_root.path().to_string_lossy().as_ref()));
        assert!(!encoded.contains(second_root.path().to_string_lossy().as_ref()));
        assert_eq!(
            first.manifest.path_integrity_boundary,
            PATH_INTEGRITY_BOUNDARY
        );
    }

    #[test]
    fn plugin_compat_report_missing_malformed_and_byte_mismatch_fail_closed() {
        let missing_root = TempDir::new().expect("missing repo root must be created");
        assert!(build_plugin_compat_reports(missing_root.path(), b"{}").is_err());

        let malformed_root = TempDir::new().expect("malformed repo root must be created");
        write_assets(malformed_root.path());
        let malformed = b"{not-json".to_vec();
        fs::write(
            malformed_root.path().join(MANIFEST_RELATIVE_PATH),
            &malformed,
        )
        .expect("malformed manifest must be written");
        assert!(build_plugin_compat_reports(malformed_root.path(), &malformed).is_err());

        let mismatch_root = TempDir::new().expect("mismatch repo root must be created");
        let bytes = write_fixture(mismatch_root.path(), &complete_manifest());
        let mut caller_bytes = bytes.clone();
        caller_bytes.push(b' ');
        assert!(build_plugin_compat_reports(mismatch_root.path(), &caller_bytes).is_err());
    }

    #[test]
    fn plugin_compat_report_unbound_incomplete_duplicate_and_blocked_inputs_fail_closed() {
        let incomplete_root = TempDir::new().expect("incomplete repo root must be created");
        let mut incomplete = complete_manifest();
        let first_id = candidate_ids().remove(0);
        incomplete
            .get_mut("toolSchemas")
            .and_then(Value::as_object_mut)
            .and_then(|schemas| schemas.get_mut(&first_id))
            .and_then(Value::as_object_mut)
            .expect("first tool schema must exist")
            .remove("outputSchema");
        let incomplete_bytes = write_fixture(incomplete_root.path(), &incomplete);
        assert!(build_plugin_compat_reports(incomplete_root.path(), &incomplete_bytes).is_err());

        let unbound_root = TempDir::new().expect("unbound repo root must be created");
        let mut unbound = complete_manifest();
        let ghost_id = "preview:mcp:unknown@local:ghost";
        let declarations = [
            (
                "toolSchemas",
                json!({
                    "inputSchema": {"type": "object"},
                    "outputSchema": {"type": "object"}
                }),
            ),
            ("permissions", json!({"scope": "ghost"})),
            ("activationEvents", json!({"event": "ghost"})),
            (
                "toolPolicies",
                json!({"approval": "required", "ledger": "required", "timeoutMs": 1}),
            ),
        ];
        for (field, declaration) in declarations {
            unbound
                .get_mut(field)
                .and_then(Value::as_object_mut)
                .expect("manifest declaration map must exist")
                .insert(ghost_id.to_string(), declaration);
        }
        let unbound_bytes = write_fixture(unbound_root.path(), &unbound);
        assert!(build_plugin_compat_reports(unbound_root.path(), &unbound_bytes).is_err());

        let valid_root = TempDir::new().expect("valid repo root must be created");
        let valid_bytes = write_fixture(valid_root.path(), &complete_manifest());
        let mut reports = build_plugin_compat_reports(valid_root.path(), &valid_bytes)
            .expect("valid typed reports must build");
        let preview_index = report_index(PLUGIN_COMPAT_REPORT_IDS[3]);
        let PluginCompatPayload::ToolContributionInventoryPreview { plan } =
            &mut reports.reports[preview_index].payload
        else {
            panic!("preview payload kind drifted")
        };
        plan.entries.push(plan.entries[0].clone());
        plan.candidate_inventory_entries
            .push(plan.candidate_inventory_entries[0].clone());
        let duplicate_registry = plugin_tool_registry_source_of_truth_dry_run_plan(plan);
        assert!(!duplicate_registry.registry_source_of_truth_dry_run_ready);
        assert!(!reports.has_integrity());

        let mut blocked = build_plugin_compat_reports(valid_root.path(), &valid_bytes)
            .expect("valid typed reports must rebuild");
        let schema_index = report_index(PLUGIN_COMPAT_REPORT_IDS[4]);
        let PluginCompatPayload::ToolManifestSchemaCutoverPreflight {
            internal_registry_plan,
            ..
        } = &mut blocked.reports[schema_index].payload
        else {
            panic!("schema payload kind drifted")
        };
        internal_registry_plan.registry_source_of_truth_dry_run_ready = false;
        assert!(!blocked.has_integrity());
    }

    #[test]
    fn plugin_compat_report_source_and_legacy_projection_tamper_fail_closed() {
        let repo_root = TempDir::new().expect("repo root must be created");
        let bytes = write_fixture(repo_root.path(), &complete_manifest());
        let reports = build_plugin_compat_reports(repo_root.path(), &bytes)
            .expect("typed reports must build");

        for index in 0..reports.reports.len() {
            let mut digest_tamper = reports.clone();
            flip_digest(&mut digest_tamper.reports[index].sources[0].sha256);
            assert!(!digest_tamper.has_integrity());
        }

        let mut manifest_tamper = reports.clone();
        flip_digest(&mut manifest_tamper.manifest.manifest_sha256);
        assert!(!manifest_tamper.has_integrity());

        let mut injection = reports.clone();
        injection.reports[0].legacy_business_fields.insert(
            "payload_injection".to_string(),
            Value::String("write_authority=true\n".to_string()),
        );
        assert!(!injection.has_integrity());
        assert!(injection.reports[0].render_legacy_line_protocol().is_err());
    }

    #[test]
    fn plugin_compat_report_authority_actions_and_side_effects_fail_closed() {
        let repo_root = TempDir::new().expect("repo root must be created");
        let bytes = write_fixture(repo_root.path(), &complete_manifest());
        let reports = build_plugin_compat_reports(repo_root.path(), &bytes)
            .expect("typed reports must build");

        let mut authority = reports.clone();
        authority.reports[0].production_authority_granted = true;
        assert!(!authority.has_integrity());

        let mut side_effect = reports.clone();
        side_effect.reports[0].side_effects.registry_mutated = true;
        assert!(!side_effect.has_integrity());

        let mut action = reports.clone();
        let PluginCompatPayload::ContributionPointAbi { plan } = &mut action.reports[0].payload
        else {
            panic!("ABI payload kind drifted")
        };
        plan.entries[0].runtime_execution_enabled = true;
        assert!(!action.has_integrity());
    }

    #[test]
    fn plugin_compat_report_relative_root_escape_and_nonregular_path_fail_closed() {
        assert!(build_plugin_compat_reports(Path::new("relative-root"), b"{}").is_err());

        let escape_root = TempDir::new().expect("escape repo root must be created");
        let mut escape = complete_manifest();
        escape["skills"] = Value::String("../outside".to_string());
        let escape_bytes = write_fixture(escape_root.path(), &escape);
        assert!(build_plugin_compat_reports(escape_root.path(), &escape_bytes).is_err());

        let nonregular_root = TempDir::new().expect("nonregular repo root must be created");
        let nonregular_bytes = write_fixture(nonregular_root.path(), &complete_manifest());
        let mcp_path = nonregular_root
            .path()
            .join("plugins/hepta-system/.mcp.json");
        fs::remove_file(&mcp_path).expect("temporary MCP file must be removed");
        fs::create_dir(&mcp_path).expect("temporary MCP directory must be created");
        assert!(build_plugin_compat_reports(nonregular_root.path(), &nonregular_bytes).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn plugin_compat_report_root_asset_and_nested_skill_symlinks_fail_closed() {
        use std::os::unix::fs::symlink;

        let real_root = TempDir::new().expect("real repo root must be created");
        let bytes = write_fixture(real_root.path(), &complete_manifest());
        let link_parent = TempDir::new().expect("link parent must be created");
        let root_link = link_parent.path().join("repo-link");
        symlink(real_root.path(), &root_link).expect("root symlink must be created");
        assert!(build_plugin_compat_reports(&root_link, &bytes).is_err());

        let asset_root = TempDir::new().expect("asset repo root must be created");
        let asset_bytes = write_fixture(asset_root.path(), &complete_manifest());
        let plugin_root = asset_root.path().join("plugins/hepta-system");
        let outside_mcp = asset_root.path().join("outside-mcp.json");
        fs::write(&outside_mcp, "{\"mcpServers\":{}}").expect("outside MCP file must be written");
        fs::remove_file(plugin_root.join(".mcp.json"))
            .expect("temporary MCP fixture must be removed");
        symlink(&outside_mcp, plugin_root.join(".mcp.json")).expect("MCP symlink must be created");
        assert!(build_plugin_compat_reports(asset_root.path(), &asset_bytes).is_err());

        let nested_root = TempDir::new().expect("nested repo root must be created");
        let nested_bytes = write_fixture(nested_root.path(), &complete_manifest());
        let nested_plugin_root = nested_root.path().join("plugins/hepta-system");
        let outside_skill = nested_root.path().join("outside-skill.md");
        fs::write(&outside_skill, "# outside\n").expect("outside skill must be written");
        symlink(
            &outside_skill,
            nested_plugin_root.join("skills/hepta-system-status/linked.md"),
        )
        .expect("nested skill symlink must be created");
        assert!(build_plugin_compat_reports(nested_root.path(), &nested_bytes).is_err());
    }
}
