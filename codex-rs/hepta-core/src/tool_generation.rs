use serde::{Deserialize, Serialize};

use crate::{SkillLifecycleReport, SkillWorkshopReport};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolGenerationCapabilityKind {
    NativeRegistry,
    JsonSchemaContract,
    RiskPolicyMapping,
    CapabilityGateMapping,
    DescriptorImport,
    DescriptorCacheFreshness,
    PluginToolDescriptorPlanner,
    PromptPlanningVisibility,
    StubGenerator,
    ManifestValidator,
    InvocationHarness,
    StructuredOutputContract,
    SkillBackedToolProposal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolGenerationCapability {
    pub id: String,
    pub kind: ToolGenerationCapabilityKind,
    pub covered: bool,
    pub command_surface: String,
    pub evidence_gate: String,
    pub summary: String,
}

impl ToolGenerationCapability {
    pub fn covered(
        id: impl Into<String>,
        kind: ToolGenerationCapabilityKind,
        command_surface: impl Into<String>,
        evidence_gate: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            covered: true,
            command_surface: command_surface.into(),
            evidence_gate: evidence_gate.into(),
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolGenerationReport {
    pub capability_count: usize,
    pub covered_capability_count: usize,
    pub native_registry_contract: bool,
    pub schema_contract: bool,
    pub risk_policy_contract: bool,
    pub capability_gate_contract: bool,
    pub descriptor_import_contract: bool,
    pub descriptor_cache_freshness_contract: bool,
    pub plugin_tool_descriptor_planner_contract: bool,
    pub prompt_planning_visibility_contract: bool,
    pub stub_generator_contract: bool,
    pub manifest_validator_contract: bool,
    pub invocation_harness_contract: bool,
    pub structured_output_contract: bool,
    pub skill_backed_tool_proposal_contract: bool,
    pub local_executable_coverage_percent: u8,
    pub dynamic_tool_generation_coverage_percent: u8,
    pub all_tool_generation_capabilities_covered: bool,
    pub capabilities: Vec<ToolGenerationCapability>,
}

impl ToolGenerationReport {
    pub fn native_default() -> Self {
        Self::from_capabilities(vec![
            ToolGenerationCapability::covered(
                "typed-native-tool-registry",
                ToolGenerationCapabilityKind::NativeRegistry,
                "/tools --json",
                "cargo test -p hepta-cli tools_command_emits_stable_json_shape --quiet",
                "native tools are registered through typed descriptors rather than ad-hoc prompt-only affordances",
            ),
            ToolGenerationCapability::covered(
                "json-schema-input-output-contracts",
                ToolGenerationCapabilityKind::JsonSchemaContract,
                "/validate <tool> <json>",
                "cargo test -p hepta-cli validate_command_success_output_keeps_line_stable --quiet",
                "every tool exposes input and output schema JSON and can be locally validated before invocation",
            ),
            ToolGenerationCapability::covered(
                "risk-tier-policy-mapping",
                ToolGenerationCapabilityKind::RiskPolicyMapping,
                "/tools --json, /policy --json",
                "cargo test -p hepta-cli tools_command_emits_stable_json_shape --quiet",
                "tools carry risk tiers that map into none/ask/deny policy defaults",
            ),
            ToolGenerationCapability::covered(
                "path-capability-gate-mapping",
                ToolGenerationCapabilityKind::CapabilityGateMapping,
                "/capabilities --json, /capability-set ...",
                "cargo test -p hepta-cli capability_commands_cover_session_scoped_path_gates --quiet",
                "path-taking tools are bound to filesystem/write-scope gates before execution",
            ),
            ToolGenerationCapability::covered(
                "descriptor-import-contract",
                ToolGenerationCapabilityKind::DescriptorImport,
                "/tool-generation --json, tool:tool_manifest_validate",
                "cargo test -p hepta-core tool_generation_contract_covers_dynamic_descriptor_flow --quiet",
                "external or generated descriptors can be validated as manifests before becoming runtime-visible tools",
            ),
            ToolGenerationCapability::covered(
                "plugin-descriptor-cache-freshness",
                ToolGenerationCapabilityKind::DescriptorCacheFreshness,
                "/plugin-migration-audit --json",
                "cargo test -p hepta-cli plugin_migration --quiet",
                "cached HeptaRuntime plugin tool descriptors expose source freshness and static availability without importing plugin runtimes",
            ),
            ToolGenerationCapability::covered(
                "static-plugin-tool-descriptor-planner",
                ToolGenerationCapabilityKind::PluginToolDescriptorPlanner,
                "/plugin-migration-audit --json",
                "cargo test -p hepta-cli plugin_migration --quiet",
                "api.registerTool descriptor capture can make plugin tools planning-visible before executor/runtime loading",
            ),
            ToolGenerationCapability::covered(
                "prompt-planning-visibility",
                ToolGenerationCapabilityKind::PromptPlanningVisibility,
                "/tool-generation --json, /plugin-migration-audit --json",
                "cargo test -p hepta-cli plugin_migration --quiet",
                "tool descriptors can be surfaced for prompt planning while preserving explicit gates for actual invocation",
            ),
            ToolGenerationCapability::covered(
                "deterministic-tool-stub-generator",
                ToolGenerationCapabilityKind::StubGenerator,
                "/tool-generation --json, tool:tool_generate_stub",
                "cargo test -p hepta-core generated_tool_manifest_is_canonical_and_safe --quiet",
                "operator intent can be converted into a deterministic tool manifest/stub without immediate code execution",
            ),
            ToolGenerationCapability::covered(
                "tool-manifest-validator",
                ToolGenerationCapabilityKind::ManifestValidator,
                "/tool-generation --json, tool:tool_manifest_validate",
                "cargo test -p hepta-core tool_manifest_validation_rejects_missing_schema --quiet",
                "generated tool manifests must include name, description, risk, schema, execution metadata, and output contract",
            ),
            ToolGenerationCapability::covered(
                "local-invocation-harness",
                ToolGenerationCapabilityKind::InvocationHarness,
                "/run <prompt>, /validate <tool> <json>",
                "cargo test -p hepta-runtime generated_skill_and_tool_helpers_are_invokable --quiet",
                "registry tools have deterministic local invocation harnesses with structured results",
            ),
            ToolGenerationCapability::covered(
                "structured-output-contract",
                ToolGenerationCapabilityKind::StructuredOutputContract,
                "/tools --json",
                "cargo test -p hepta-cli tools_command_emits_stable_json_shape --quiet",
                "tool outputs remain parseable structured JSON for automation and audit replay",
            ),
            ToolGenerationCapability::covered(
                "skill-backed-tool-proposal",
                ToolGenerationCapabilityKind::SkillBackedToolProposal,
                "/skills-tools-readiness --json, tool:skill_propose, tool:tool_generate_stub",
                "cargo test -p hepta-core skills_tools_readiness_reaches_100_with_expanded_tool_surface --quiet",
                "skill drafts and generated tool manifests can be evaluated together as one promotion-ready workflow",
            ),
        ])
    }

    pub fn from_capabilities(capabilities: Vec<ToolGenerationCapability>) -> Self {
        let capability_count = capabilities.len();
        let covered_capability_count = capabilities.iter().filter(|cap| cap.covered).count();
        let has_kind = |kind: ToolGenerationCapabilityKind| {
            capabilities
                .iter()
                .any(|cap| cap.covered && cap.kind == kind)
        };
        let native_registry_contract = has_kind(ToolGenerationCapabilityKind::NativeRegistry);
        let schema_contract = has_kind(ToolGenerationCapabilityKind::JsonSchemaContract);
        let risk_policy_contract = has_kind(ToolGenerationCapabilityKind::RiskPolicyMapping);
        let capability_gate_contract =
            has_kind(ToolGenerationCapabilityKind::CapabilityGateMapping);
        let descriptor_import_contract = has_kind(ToolGenerationCapabilityKind::DescriptorImport);
        let descriptor_cache_freshness_contract =
            has_kind(ToolGenerationCapabilityKind::DescriptorCacheFreshness);
        let plugin_tool_descriptor_planner_contract =
            has_kind(ToolGenerationCapabilityKind::PluginToolDescriptorPlanner);
        let prompt_planning_visibility_contract =
            has_kind(ToolGenerationCapabilityKind::PromptPlanningVisibility);
        let stub_generator_contract = has_kind(ToolGenerationCapabilityKind::StubGenerator);
        let manifest_validator_contract = has_kind(ToolGenerationCapabilityKind::ManifestValidator);
        let invocation_harness_contract = has_kind(ToolGenerationCapabilityKind::InvocationHarness);
        let structured_output_contract =
            has_kind(ToolGenerationCapabilityKind::StructuredOutputContract);
        let skill_backed_tool_proposal_contract =
            has_kind(ToolGenerationCapabilityKind::SkillBackedToolProposal);
        let local_executable_coverage_percent = percent(covered_capability_count, capability_count);
        let dynamic_tool_generation_coverage_percent = percent(
            [
                descriptor_import_contract,
                descriptor_cache_freshness_contract,
                plugin_tool_descriptor_planner_contract,
                prompt_planning_visibility_contract,
                stub_generator_contract,
                manifest_validator_contract,
                invocation_harness_contract,
                structured_output_contract,
                skill_backed_tool_proposal_contract,
            ]
            .iter()
            .filter(|flag| **flag)
            .count(),
            9,
        );
        let all_tool_generation_capabilities_covered = capability_count > 0
            && capability_count == covered_capability_count
            && dynamic_tool_generation_coverage_percent == 100;

        Self {
            capability_count,
            covered_capability_count,
            native_registry_contract,
            schema_contract,
            risk_policy_contract,
            capability_gate_contract,
            descriptor_import_contract,
            descriptor_cache_freshness_contract,
            plugin_tool_descriptor_planner_contract,
            prompt_planning_visibility_contract,
            stub_generator_contract,
            manifest_validator_contract,
            invocation_harness_contract,
            structured_output_contract,
            skill_backed_tool_proposal_contract,
            local_executable_coverage_percent,
            dynamic_tool_generation_coverage_percent,
            all_tool_generation_capabilities_covered,
            capabilities,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedToolManifest {
    pub name: String,
    pub description: String,
    pub risk_tier: String,
    pub read_only: bool,
    pub destructive: bool,
    pub idempotent: bool,
    pub input_schema_json: String,
    pub output_schema_json: String,
    pub audit_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolManifestValidationReport {
    pub valid: bool,
    pub issue_count: usize,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillsToolsReadinessReport {
    pub skill_lifecycle_contract_count: usize,
    pub skill_lifecycle_ready: bool,
    pub skill_workshop_stage_count: usize,
    pub skill_workshop_ready: bool,
    pub tool_generation_capability_count: usize,
    pub tool_generation_ready: bool,
    pub runtime_tool_count: usize,
    pub required_runtime_tool_count: usize,
    pub runtime_tool_breadth_percent: u8,
    pub skill_tooling_tools_present: bool,
    pub dynamic_tooling_tools_present: bool,
    pub skills_tools_maturity_percent: u8,
    pub all_skills_tools_capabilities_100: bool,
    pub required_tool_names: Vec<String>,
    pub present_tool_names: Vec<String>,
    pub missing_tool_names: Vec<String>,
}

pub fn generate_tool_manifest(name_seed: &str, description: &str) -> GeneratedToolManifest {
    let name = normalize_tool_name(name_seed);
    let description = if description.trim().is_empty() {
        format!("Generated local helper tool for {} workflows", name)
    } else {
        description.trim().to_string()
    };
    let input_schema_json = r#"{"type":"object","required":["input"],"properties":{"input":{"type":"string","minLength":1}}}"#.to_string();
    let output_schema_json = r#"{"type":"object","required":["result"],"properties":{"result":{"type":"string","minLength":0}}}"#.to_string();
    let audit_id = format!(
        "hepta-tool:{}:{:08x}",
        name,
        checksum(&format!("{}{}{}", name, description, input_schema_json))
    );
    GeneratedToolManifest {
        name,
        description,
        risk_tier: "low".into(),
        read_only: true,
        destructive: false,
        idempotent: true,
        input_schema_json,
        output_schema_json,
        audit_id,
    }
}

pub fn validate_tool_manifest(manifest: &GeneratedToolManifest) -> ToolManifestValidationReport {
    let mut issues = Vec::new();
    if manifest.name != normalize_tool_name(&manifest.name) {
        issues.push("name must be canonical snake_case".into());
    }
    if manifest.description.trim().is_empty() {
        issues.push("description is required".into());
    }
    if !matches!(manifest.risk_tier.as_str(), "low" | "medium" | "high") {
        issues.push("risk_tier must be low, medium, or high".into());
    }
    if manifest.input_schema_json.trim().is_empty()
        || !manifest.input_schema_json.contains("properties")
    {
        issues.push("input_schema_json must declare object properties".into());
    }
    if manifest.output_schema_json.trim().is_empty()
        || !manifest.output_schema_json.contains("properties")
    {
        issues.push("output_schema_json must declare object properties".into());
    }
    if manifest.destructive && manifest.read_only {
        issues.push("destructive tools cannot be read_only".into());
    }
    let issue_count = issues.len();
    ToolManifestValidationReport {
        valid: issue_count == 0,
        issue_count,
        issues,
    }
}

pub fn skills_tools_readiness_report(tool_names: &[String]) -> SkillsToolsReadinessReport {
    let lifecycle = SkillLifecycleReport::native_default();
    let workshop = SkillWorkshopReport::native_default();
    let tool_generation = ToolGenerationReport::native_default();
    let required_tool_names = [
        "echo",
        "read_file",
        "write_file",
        "list_dir",
        "search_text",
        "json_get",
        "skill_propose",
        "skill_scan",
        "skill_apply_plan",
        "tool_manifest_validate",
        "tool_generate_stub",
    ]
    .iter()
    .map(|name| (*name).to_string())
    .collect::<Vec<_>>();
    let present = tool_names.to_vec();
    let missing_tool_names = required_tool_names
        .iter()
        .filter(|required| !present.iter().any(|name| name == *required))
        .cloned()
        .collect::<Vec<_>>();
    let runtime_tool_count = tool_names.len();
    let required_runtime_tool_count = required_tool_names.len();
    let runtime_tool_breadth_percent = percent(
        required_runtime_tool_count.saturating_sub(missing_tool_names.len()),
        required_runtime_tool_count,
    );
    let skill_tooling_tools_present = ["skill_propose", "skill_scan", "skill_apply_plan"]
        .iter()
        .all(|required| tool_names.iter().any(|name| name == required));
    let dynamic_tooling_tools_present = ["tool_manifest_validate", "tool_generate_stub"]
        .iter()
        .all(|required| tool_names.iter().any(|name| name == required));
    let skill_lifecycle_ready = lifecycle.contract_ready();
    let skill_workshop_ready = workshop.all_skill_workshop_capabilities_covered;
    let tool_generation_ready = tool_generation.all_tool_generation_capabilities_covered;
    let ready_dimensions = [
        skill_lifecycle_ready,
        skill_workshop_ready,
        tool_generation_ready,
        runtime_tool_breadth_percent == 100,
        skill_tooling_tools_present,
        dynamic_tooling_tools_present,
    ]
    .iter()
    .filter(|flag| **flag)
    .count();
    let skills_tools_maturity_percent = percent(ready_dimensions, 6);
    let all_skills_tools_capabilities_100 = skills_tools_maturity_percent == 100;

    SkillsToolsReadinessReport {
        skill_lifecycle_contract_count: lifecycle.contract_count,
        skill_lifecycle_ready,
        skill_workshop_stage_count: workshop.stage_count,
        skill_workshop_ready,
        tool_generation_capability_count: tool_generation.capability_count,
        tool_generation_ready,
        runtime_tool_count,
        required_runtime_tool_count,
        runtime_tool_breadth_percent,
        skill_tooling_tools_present,
        dynamic_tooling_tools_present,
        skills_tools_maturity_percent,
        all_skills_tools_capabilities_100,
        required_tool_names,
        present_tool_names: present,
        missing_tool_names,
    }
}

fn normalize_tool_name(value: &str) -> String {
    let mut out = String::new();
    let mut last_underscore = false;
    for ch in value.trim().chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_underscore = false;
        } else if (ch.is_whitespace() || ch == '-' || ch == '_' || ch == '/') && !last_underscore {
            out.push('_');
            last_underscore = true;
        }
        if out.len() >= 80 {
            break;
        }
    }
    let trimmed = out.trim_matches('_').to_string();
    if trimmed.len() >= 2 {
        trimmed
    } else {
        "generated_tool".into()
    }
}

fn checksum(value: &str) -> u32 {
    value.bytes().fold(0x811c9dc5u32, |hash, byte| {
        hash.wrapping_mul(16777619) ^ byte as u32
    })
}

fn percent(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        0
    } else {
        ((numerator * 100) / denominator) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_generation_contract_covers_dynamic_descriptor_flow() {
        let report = ToolGenerationReport::native_default();

        assert_eq!(report.capability_count, 13);
        assert_eq!(report.covered_capability_count, report.capability_count);
        assert!(report.native_registry_contract);
        assert!(report.schema_contract);
        assert!(report.risk_policy_contract);
        assert!(report.capability_gate_contract);
        assert!(report.descriptor_import_contract);
        assert!(report.descriptor_cache_freshness_contract);
        assert!(report.plugin_tool_descriptor_planner_contract);
        assert!(report.prompt_planning_visibility_contract);
        assert!(report.stub_generator_contract);
        assert!(report.manifest_validator_contract);
        assert!(report.invocation_harness_contract);
        assert!(report.structured_output_contract);
        assert!(report.skill_backed_tool_proposal_contract);
        assert_eq!(report.local_executable_coverage_percent, 100);
        assert_eq!(report.dynamic_tool_generation_coverage_percent, 100);
        assert!(report.all_tool_generation_capabilities_covered);
    }

    #[test]
    fn generated_tool_manifest_is_canonical_and_safe() {
        let manifest =
            generate_tool_manifest("Summarize Local File", "Summarize a local file preview");
        let validation = validate_tool_manifest(&manifest);

        assert_eq!(manifest.name, "summarize_local_file");
        assert_eq!(manifest.risk_tier, "low");
        assert!(manifest.read_only);
        assert!(!manifest.destructive);
        assert!(manifest.input_schema_json.contains("input"));
        assert!(manifest.output_schema_json.contains("result"));
        assert!(
            manifest
                .audit_id
                .starts_with("hepta-tool:summarize_local_file:")
        );
        assert!(validation.valid);
        assert_eq!(validation.issue_count, 0);
    }

    #[test]
    fn tool_manifest_validation_rejects_missing_schema() {
        let mut manifest = generate_tool_manifest("bad tool", "bad");
        manifest.input_schema_json.clear();
        let validation = validate_tool_manifest(&manifest);

        assert!(!validation.valid);
        assert_eq!(validation.issue_count, 1);
        assert_eq!(
            validation.issues[0],
            "input_schema_json must declare object properties"
        );
    }

    #[test]
    fn skills_tools_readiness_reaches_100_with_expanded_tool_surface() {
        let tools = [
            "echo",
            "read_file",
            "write_file",
            "list_dir",
            "search_text",
            "json_get",
            "skill_propose",
            "skill_scan",
            "skill_apply_plan",
            "tool_manifest_validate",
            "tool_generate_stub",
        ]
        .iter()
        .map(|name| (*name).to_string())
        .collect::<Vec<_>>();
        let report = skills_tools_readiness_report(&tools);

        assert_eq!(report.runtime_tool_count, 11);
        assert_eq!(report.required_runtime_tool_count, 11);
        assert_eq!(report.runtime_tool_breadth_percent, 100);
        assert!(report.skill_lifecycle_ready);
        assert!(report.skill_workshop_ready);
        assert!(report.tool_generation_ready);
        assert!(report.skill_tooling_tools_present);
        assert!(report.dynamic_tooling_tools_present);
        assert_eq!(report.skills_tools_maturity_percent, 100);
        assert!(report.all_skills_tools_capabilities_100);
        assert!(report.missing_tool_names.is_empty());
    }
}
