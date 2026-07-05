use crate::config::AgentCardManifestConfig;
use crate::tools::handlers::work_graph_agent_card_manifest::WorkGraphConfiguredManifestOverlayShadowDecision;
use crate::tools::handlers::work_graph_agent_card_manifest::WorkGraphConfiguredManifestOverlayShadowInput;
use crate::tools::handlers::work_graph_agent_card_manifest::build_configured_manifest_overlay_shadow_decision;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphAdmissionShadowDecision {
    pub(crate) source_surface_id: &'static str,
    pub(crate) decision: &'static str,
    pub(crate) task_id: Option<String>,
    pub(crate) job_id: Option<String>,
    pub(crate) role_manifest_shadow_decision: WorkGraphRoleManifestShadowDecision,
    pub(crate) requested_concurrency: usize,
    pub(crate) item_count: Option<usize>,
    pub(crate) child_depth: i32,
    pub(crate) max_depth: i32,
    pub(crate) max_threads: Option<usize>,
    pub(crate) side_effect_class: &'static str,
    pub(crate) output_contract_required: bool,
    pub(crate) output_contract_present: bool,
    pub(crate) result_contract_required: bool,
    pub(crate) result_contract_present: bool,
    pub(crate) reducer_required: bool,
    pub(crate) reducer_present: bool,
    pub(crate) denial_reasons: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_id: &'static str,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) blocking_guardrail_preview: bool,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphAdmissionShadowCheck {
    pub(crate) name: &'static str,
    pub(crate) passed: bool,
    pub(crate) detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphRoleManifestShadowDecision {
    pub(crate) source_surface_id: &'static str,
    pub(crate) manifest_id: String,
    pub(crate) definition_source: &'static str,
    pub(crate) manifest_version: &'static str,
    pub(crate) configured_manifest_shadow_decision: WorkGraphConfiguredManifestShadowDecision,
    pub(crate) promotion_readiness_shadow_decision: WorkGraphPromotionReadinessShadowDecision,
    pub(crate) task_result_contract_shadow_plan: WorkGraphTaskResultContractShadowPlan,
    pub(crate) decision: &'static str,
    pub(crate) role_name: Option<String>,
    pub(crate) description: &'static str,
    pub(crate) role_declared: bool,
    pub(crate) capabilities: Vec<String>,
    pub(crate) allowed_tools: Vec<String>,
    pub(crate) attempted_tool: Option<&'static str>,
    pub(crate) tool_allowed: Option<bool>,
    pub(crate) side_effect_class: &'static str,
    pub(crate) output_contract_required: bool,
    pub(crate) output_contract_present: bool,
    pub(crate) result_contract_required: bool,
    pub(crate) result_contract_present: bool,
    pub(crate) verifier_present: bool,
    pub(crate) reducer_present: bool,
    pub(crate) lane: &'static str,
    pub(crate) observed_lane: Option<&'static str>,
    pub(crate) lane_allowed: Option<bool>,
    pub(crate) denial_reasons: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_id: &'static str,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) blocking_guardrail_preview: bool,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphTaskResultContractShadowPlan {
    pub(crate) source_surface_id: &'static str,
    pub(crate) decision: &'static str,
    pub(crate) plan_stage: &'static str,
    pub(crate) result_contract_required: bool,
    pub(crate) result_contract_present: bool,
    pub(crate) verifier_present: bool,
    pub(crate) reducer_present: bool,
    pub(crate) result_contract_ready: bool,
    pub(crate) verifier_ready: bool,
    pub(crate) reducer_ready: bool,
    pub(crate) contract_plan_ready: bool,
    pub(crate) task_result_contract_id: &'static str,
    pub(crate) result_envelope_schema: &'static str,
    pub(crate) terminal_delivery_surface: &'static str,
    pub(crate) verifier_id: &'static str,
    pub(crate) reducer_id: &'static str,
    pub(crate) missing_contract_parts: Vec<String>,
    pub(crate) planned_shadow_events: Vec<String>,
    pub(crate) next_actions: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_id: &'static str,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) blocking_guardrail_preview: bool,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphPromotionReadinessShadowDecision {
    pub(crate) source_surface_id: &'static str,
    pub(crate) decision: &'static str,
    pub(crate) manifest_id: String,
    pub(crate) promotion_stage: &'static str,
    pub(crate) configured_manifest_ready: bool,
    pub(crate) configured_overlay_ready: bool,
    pub(crate) role_contracts_ready: bool,
    pub(crate) budget_ready: bool,
    pub(crate) lane_ready: bool,
    pub(crate) attempted_tool_ready: bool,
    pub(crate) result_contract_ready: bool,
    pub(crate) verifier_reducer_ready: bool,
    pub(crate) denial_reasons: Vec<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_id: &'static str,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) blocking_guardrail_preview: bool,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphConfiguredManifestShadowDecision {
    pub(crate) source_surface_id: &'static str,
    pub(crate) decision: &'static str,
    pub(crate) registry_source: &'static str,
    pub(crate) configured_manifest_present: bool,
    pub(crate) configured_manifest_source: Option<String>,
    pub(crate) expected_manifest_version: &'static str,
    pub(crate) configured_manifest_version: Option<String>,
    pub(crate) version_matches: bool,
    pub(crate) source_compatible: bool,
    pub(crate) configured_manifest_overlay_shadow_decision:
        WorkGraphConfiguredManifestOverlayShadowDecision,
    pub(crate) stale: bool,
    pub(crate) compatibility_reason: Option<String>,
    pub(crate) staleness_reason: Option<String>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_id: &'static str,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) blocking_guardrail_preview: bool,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WorkGraphAgentCardManifest {
    pub(crate) source_surface_id: &'static str,
    pub(crate) role_name: Option<&'static str>,
    pub(crate) description: &'static str,
    pub(crate) capabilities: &'static [&'static str],
    pub(crate) allowed_tools: &'static [&'static str],
    pub(crate) side_effect_class: &'static str,
    pub(crate) output_contract_required: bool,
    pub(crate) output_contract_present: bool,
    pub(crate) result_contract_required: bool,
    pub(crate) result_contract_present: bool,
    pub(crate) verifier_present: bool,
    pub(crate) reducer_present: bool,
    pub(crate) lane: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorkGraphAgentCardManifestProfile {
    AgentJobWorker,
    AgentJobResultReporter,
    SubagentSpawn,
    SubagentNamedSpawn,
    SubagentHandoff,
    SubagentLifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WorkGraphAgentCardManifestRegistryEntry {
    pub(crate) source_surface_id: &'static str,
    pub(crate) profile: WorkGraphAgentCardManifestProfile,
    pub(crate) manifest: WorkGraphAgentCardManifest,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WorkGraphAgentCardManifestRegistry {
    entries: &'static [WorkGraphAgentCardManifestRegistryEntry],
}

pub(crate) struct WorkGraphAgentCardManifestObservation {
    pub(crate) role_name: Option<String>,
    pub(crate) role_declared: bool,
    pub(crate) role_description_present: bool,
    pub(crate) configured_manifest_source: Option<String>,
    pub(crate) configured_manifest_version: Option<String>,
    pub(crate) configured_manifest_overlay: Option<AgentCardManifestConfig>,
    pub(crate) budget_present: bool,
    pub(crate) output_contract_present: Option<bool>,
    pub(crate) result_contract_present: Option<bool>,
    pub(crate) verifier_present: Option<bool>,
    pub(crate) reducer_present: Option<bool>,
    pub(crate) attempted_tool: Option<&'static str>,
    pub(crate) observed_lane: Option<&'static str>,
}

pub(crate) struct WorkGraphRoleManifestShadowInput {
    pub(crate) source_surface_id: &'static str,
    pub(crate) definition_source: &'static str,
    pub(crate) manifest_version: &'static str,
    pub(crate) role_name: Option<String>,
    pub(crate) description: &'static str,
    pub(crate) role_declared: bool,
    pub(crate) role_description_present: bool,
    pub(crate) configured_manifest_source: Option<String>,
    pub(crate) configured_manifest_version: Option<String>,
    pub(crate) configured_manifest_overlay: Option<AgentCardManifestConfig>,
    pub(crate) capabilities: Vec<String>,
    pub(crate) allowed_tools: Vec<String>,
    pub(crate) attempted_tool: Option<&'static str>,
    pub(crate) budget_present: bool,
    pub(crate) side_effect_class: &'static str,
    pub(crate) output_contract_required: bool,
    pub(crate) output_contract_present: bool,
    pub(crate) result_contract_required: bool,
    pub(crate) result_contract_present: bool,
    pub(crate) verifier_present: bool,
    pub(crate) reducer_present: bool,
    pub(crate) lane: &'static str,
    pub(crate) observed_lane: Option<&'static str>,
}

pub(crate) struct WorkGraphAdmissionShadowInput {
    pub(crate) source_surface_id: &'static str,
    pub(crate) task_id: Option<String>,
    pub(crate) job_id: Option<String>,
    pub(crate) role_manifest_shadow_decision: WorkGraphRoleManifestShadowDecision,
    pub(crate) requested_concurrency: usize,
    pub(crate) item_count: Option<usize>,
    pub(crate) child_depth: i32,
    pub(crate) max_depth: i32,
    pub(crate) max_threads: Option<usize>,
    pub(crate) enforce_depth_limit: bool,
    pub(crate) state_db_required: bool,
    pub(crate) state_db_available: bool,
    pub(crate) output_contract_required: bool,
    pub(crate) output_contract_present: bool,
    pub(crate) result_contract_required: bool,
    pub(crate) result_contract_present: bool,
    pub(crate) reducer_required: bool,
    pub(crate) reducer_present: bool,
    pub(crate) side_effect_class: &'static str,
}

const ADMISSION_ALLOW_SHADOW: &str = "allow_shadow_no_live_blocking";
const ADMISSION_DENY_SHADOW: &str = "deny_shadow_no_live_blocking";
const ROLE_MANIFEST_ALLOW_SHADOW: &str = "allow_shadow_manifest_no_live_blocking";
const ROLE_MANIFEST_DENY_SHADOW: &str = "deny_shadow_manifest_no_live_blocking";
const AGENT_CARD_DEFINITION_SOURCE: &str = "explicit_agent_card_manifest";
const AGENT_CARD_MANIFEST_VERSION: &str = "hepta.agent_card_manifest.v1";
const AGENT_CARD_REGISTRY_SOURCE: &str = "default_agent_card_manifest_registry";
const CONFIGURED_MANIFEST_PRESENT_SHADOW: &str =
    "configured_manifest_present_shadow_no_live_blocking";
const CONFIGURED_MANIFEST_MISSING_SHADOW: &str =
    "configured_manifest_missing_shadow_no_live_blocking";
const CONFIGURED_MANIFEST_VERSION_DRIFT_SHADOW: &str =
    "configured_manifest_version_drift_shadow_no_live_blocking";
const CONFIGURED_MANIFEST_INCOMPATIBLE_SHADOW: &str =
    "configured_manifest_incompatible_shadow_no_live_blocking";
const PROMOTION_READY_SHADOW: &str = "promotion_ready_shadow_no_live_cutover";
const PROMOTION_NOT_READY_SHADOW: &str = "promotion_not_ready_shadow_no_live_cutover";
const TASK_RESULT_CONTRACT_PLAN_READY_SHADOW: &str =
    "task_result_contract_plan_ready_shadow_no_live_cutover";
const TASK_RESULT_CONTRACT_PLAN_BLOCKED_SHADOW: &str =
    "task_result_contract_plan_blocked_shadow_no_live_cutover";

const AGENT_CARD_MANIFEST_REGISTRY_ENTRIES: &[WorkGraphAgentCardManifestRegistryEntry] = &[
    WorkGraphAgentCardManifestRegistryEntry {
        source_surface_id: "spawn_agents_on_csv",
        profile: WorkGraphAgentCardManifestProfile::AgentJobWorker,
        manifest: WorkGraphAgentCardManifest {
            source_surface_id: "spawn_agents_on_csv",
            role_name: Some("agent_job_worker"),
            description: "CSV row worker that must report exactly one structured TaskResult.",
            capabilities: &[
                "csv_row_processing",
                "task_result_reporting",
                "work_graph_shadow_event_emission",
            ],
            allowed_tools: &["report_agent_job_result"],
            side_effect_class: "local_agent_job_fanout",
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: true,
            result_contract_present: true,
            verifier_present: true,
            reducer_present: true,
            lane: "agent_jobs",
        },
    },
    WorkGraphAgentCardManifestRegistryEntry {
        source_surface_id: "report_agent_job_result",
        profile: WorkGraphAgentCardManifestProfile::AgentJobResultReporter,
        manifest: WorkGraphAgentCardManifest {
            source_surface_id: "report_agent_job_result",
            role_name: Some("agent_job_worker"),
            description: "Agent job worker reporting one terminal TaskResult.",
            capabilities: &["task_result_reporting", "work_graph_shadow_event_emission"],
            allowed_tools: &["report_agent_job_result"],
            side_effect_class: "local_agent_job_result_reporting",
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: true,
            result_contract_present: true,
            verifier_present: true,
            reducer_present: true,
            lane: "agent_jobs",
        },
    },
    WorkGraphAgentCardManifestRegistryEntry {
        source_surface_id: "spawn_agent",
        profile: WorkGraphAgentCardManifestProfile::SubagentSpawn,
        manifest: WorkGraphAgentCardManifest {
            source_surface_id: "spawn_agent",
            role_name: None,
            description: "Local subagent spawned from the current turn.",
            capabilities: &["local_subagent_spawn", "inter_agent_mailbox"],
            allowed_tools: &["send_message", "followup_task", "wait_agent", "close_agent"],
            side_effect_class: "local_subagent_spawn",
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: true,
            result_contract_present: false,
            verifier_present: false,
            reducer_present: false,
            lane: "subagent",
        },
    },
    WorkGraphAgentCardManifestRegistryEntry {
        source_surface_id: "spawn_agent_v2",
        profile: WorkGraphAgentCardManifestProfile::SubagentNamedSpawn,
        manifest: WorkGraphAgentCardManifest {
            source_surface_id: "spawn_agent_v2",
            role_name: None,
            description: "Local named subagent spawned from the current turn.",
            capabilities: &[
                "local_subagent_spawn",
                "inter_agent_mailbox",
                "named_task_path",
            ],
            allowed_tools: &["send_message", "followup_task", "wait_agent", "close_agent"],
            side_effect_class: "local_subagent_spawn",
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: true,
            result_contract_present: false,
            verifier_present: false,
            reducer_present: false,
            lane: "subagent",
        },
    },
    WorkGraphAgentCardManifestRegistryEntry {
        source_surface_id: "send_message",
        profile: WorkGraphAgentCardManifestProfile::SubagentHandoff,
        manifest: WorkGraphAgentCardManifest {
            source_surface_id: "send_message",
            role_name: None,
            description: "Local inter-agent handoff over the durable mailbox.",
            capabilities: &["inter_agent_mailbox", "named_task_handoff"],
            allowed_tools: &["send_message", "followup_task"],
            side_effect_class: "local_subagent_handoff",
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: false,
            result_contract_present: false,
            verifier_present: true,
            reducer_present: false,
            lane: "subagent_handoff",
        },
    },
    WorkGraphAgentCardManifestRegistryEntry {
        source_surface_id: "followup_task",
        profile: WorkGraphAgentCardManifestProfile::SubagentHandoff,
        manifest: WorkGraphAgentCardManifest {
            source_surface_id: "followup_task",
            role_name: None,
            description: "Local inter-agent follow-up task over the durable mailbox.",
            capabilities: &["inter_agent_mailbox", "named_task_handoff"],
            allowed_tools: &["send_message", "followup_task"],
            side_effect_class: "local_subagent_handoff",
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: false,
            result_contract_present: false,
            verifier_present: true,
            reducer_present: false,
            lane: "subagent_handoff",
        },
    },
    WorkGraphAgentCardManifestRegistryEntry {
        source_surface_id: "close_agent",
        profile: WorkGraphAgentCardManifestProfile::SubagentLifecycle,
        manifest: WorkGraphAgentCardManifest {
            source_surface_id: "close_agent",
            role_name: None,
            description: "Local subagent lifecycle close surface.",
            capabilities: &[
                "subagent_lifecycle_control",
                "work_graph_shadow_event_emission",
            ],
            allowed_tools: &["close_agent", "wait_agent"],
            side_effect_class: "local_subagent_lifecycle",
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: false,
            result_contract_present: false,
            verifier_present: true,
            reducer_present: false,
            lane: "subagent_lifecycle",
        },
    },
    WorkGraphAgentCardManifestRegistryEntry {
        source_surface_id: "wait_agent",
        profile: WorkGraphAgentCardManifestProfile::SubagentLifecycle,
        manifest: WorkGraphAgentCardManifest {
            source_surface_id: "wait_agent",
            role_name: None,
            description: "Local subagent lifecycle wait surface.",
            capabilities: &[
                "subagent_lifecycle_control",
                "work_graph_shadow_event_emission",
            ],
            allowed_tools: &["close_agent", "wait_agent"],
            side_effect_class: "local_subagent_lifecycle",
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: false,
            result_contract_present: false,
            verifier_present: true,
            reducer_present: false,
            lane: "subagent_lifecycle",
        },
    },
];

pub(crate) const fn default_agent_card_manifest_registry() -> WorkGraphAgentCardManifestRegistry {
    WorkGraphAgentCardManifestRegistry {
        entries: AGENT_CARD_MANIFEST_REGISTRY_ENTRIES,
    }
}

impl WorkGraphAgentCardManifestRegistry {
    pub(crate) fn entries(&self) -> &'static [WorkGraphAgentCardManifestRegistryEntry] {
        self.entries
    }

    pub(crate) fn manifest_for_source(
        &self,
        source_surface_id: &'static str,
    ) -> Option<WorkGraphAgentCardManifest> {
        self.entries
            .iter()
            .find(|entry| entry.source_surface_id == source_surface_id)
            .map(|entry| entry.manifest)
    }
}

fn manifest_for_source(source_surface_id: &'static str) -> WorkGraphAgentCardManifest {
    default_agent_card_manifest_registry()
        .manifest_for_source(source_surface_id)
        .unwrap_or_else(|| panic!("missing WorkGraph AgentCard manifest for {source_surface_id}"))
}

pub(crate) fn configured_agent_role_manifest_source(
    role_name: Option<&str>,
    role_config_present: bool,
    role_config_file_present: bool,
    explicit_manifest_source: Option<&str>,
) -> Option<String> {
    if !role_config_present {
        return None;
    }
    if let Some(source) = explicit_manifest_source
        .map(str::trim)
        .filter(|source| !source.is_empty())
    {
        return Some(source.to_string());
    }
    let source_kind = if role_config_file_present {
        "agent_role_config_file"
    } else {
        "agent_role_config_inline"
    };
    Some(match role_name {
        Some(role_name) if !role_name.trim().is_empty() => {
            format!("{source_kind}:{role_name}")
        }
        _ => source_kind.to_string(),
    })
}

pub(crate) fn agent_jobs_worker_agent_card_manifest() -> WorkGraphAgentCardManifest {
    manifest_for_source("spawn_agents_on_csv")
}

pub(crate) fn agent_job_result_agent_card_manifest() -> WorkGraphAgentCardManifest {
    manifest_for_source("report_agent_job_result")
}

pub(crate) fn subagent_spawn_agent_card_manifest(
    source_surface_id: &'static str,
) -> WorkGraphAgentCardManifest {
    manifest_for_source(source_surface_id)
}

pub(crate) fn subagent_lifecycle_agent_card_manifest(
    source_surface_id: &'static str,
) -> WorkGraphAgentCardManifest {
    manifest_for_source(source_surface_id)
}

pub(crate) fn subagent_handoff_agent_card_manifest(
    source_surface_id: &'static str,
) -> WorkGraphAgentCardManifest {
    manifest_for_source(source_surface_id)
}

pub(crate) fn build_agent_card_manifest_shadow_decision(
    manifest: WorkGraphAgentCardManifest,
    observation: WorkGraphAgentCardManifestObservation,
) -> WorkGraphRoleManifestShadowDecision {
    build_work_graph_role_manifest_shadow_decision(WorkGraphRoleManifestShadowInput {
        source_surface_id: manifest.source_surface_id,
        definition_source: AGENT_CARD_DEFINITION_SOURCE,
        manifest_version: AGENT_CARD_MANIFEST_VERSION,
        role_name: observation
            .role_name
            .or_else(|| manifest.role_name.map(str::to_string)),
        description: manifest.description,
        role_declared: observation.role_declared,
        role_description_present: observation.role_description_present,
        configured_manifest_source: observation.configured_manifest_source,
        configured_manifest_version: observation.configured_manifest_version,
        configured_manifest_overlay: observation.configured_manifest_overlay,
        capabilities: manifest
            .capabilities
            .iter()
            .map(|capability| (*capability).to_string())
            .collect(),
        allowed_tools: manifest
            .allowed_tools
            .iter()
            .map(|tool| (*tool).to_string())
            .collect(),
        attempted_tool: observation.attempted_tool,
        budget_present: observation.budget_present,
        side_effect_class: manifest.side_effect_class,
        output_contract_required: manifest.output_contract_required,
        output_contract_present: observation
            .output_contract_present
            .unwrap_or(manifest.output_contract_present),
        result_contract_required: manifest.result_contract_required,
        result_contract_present: observation
            .result_contract_present
            .unwrap_or(manifest.result_contract_present),
        verifier_present: observation
            .verifier_present
            .unwrap_or(manifest.verifier_present),
        reducer_present: observation
            .reducer_present
            .unwrap_or(manifest.reducer_present),
        lane: manifest.lane,
        observed_lane: observation.observed_lane,
    })
}

pub(crate) fn build_default_task_result_contract_shadow_plan(
    manifest: WorkGraphAgentCardManifest,
) -> WorkGraphTaskResultContractShadowPlan {
    build_task_result_contract_shadow_plan(&WorkGraphRoleManifestShadowInput {
        source_surface_id: manifest.source_surface_id,
        definition_source: AGENT_CARD_DEFINITION_SOURCE,
        manifest_version: AGENT_CARD_MANIFEST_VERSION,
        role_name: manifest.role_name.map(str::to_string),
        description: manifest.description,
        role_declared: true,
        role_description_present: true,
        configured_manifest_source: None,
        configured_manifest_version: None,
        configured_manifest_overlay: None,
        capabilities: manifest
            .capabilities
            .iter()
            .map(|capability| (*capability).to_string())
            .collect(),
        allowed_tools: manifest
            .allowed_tools
            .iter()
            .map(|tool| (*tool).to_string())
            .collect(),
        attempted_tool: None,
        budget_present: true,
        side_effect_class: manifest.side_effect_class,
        output_contract_required: manifest.output_contract_required,
        output_contract_present: manifest.output_contract_present,
        result_contract_required: manifest.result_contract_required,
        result_contract_present: manifest.result_contract_present,
        verifier_present: manifest.verifier_present,
        reducer_present: manifest.reducer_present,
        lane: manifest.lane,
        observed_lane: Some(manifest.lane),
    })
}

pub(crate) fn build_work_graph_role_manifest_shadow_decision(
    input: WorkGraphRoleManifestShadowInput,
) -> WorkGraphRoleManifestShadowDecision {
    let configured_manifest_shadow_decision = build_configured_manifest_shadow_decision(&input);
    let mut checks = Vec::new();
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "role_declared",
        passed: input.role_declared,
        detail: match input.role_name.as_deref() {
            Some(role_name) => format!("requested role `{role_name}` must resolve to a manifest"),
            None => "default generated role manifest is used for this entrypoint".to_string(),
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "role_description",
        passed: input.role_description_present,
        detail: "role manifest should include operator-facing description".to_string(),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "capabilities",
        passed: !input.capabilities.is_empty(),
        detail: format!("declared capabilities: {}", input.capabilities.join(", ")),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "allowed_tools",
        passed: !input.allowed_tools.is_empty(),
        detail: format!("declared allowed tools: {}", input.allowed_tools.join(", ")),
    });
    let tool_allowed = input.attempted_tool.map(|attempted_tool| {
        input
            .allowed_tools
            .iter()
            .any(|allowed_tool| allowed_tool == attempted_tool)
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "attempted_tool",
        passed: tool_allowed.unwrap_or(true),
        detail: match input.attempted_tool {
            Some(tool) => format!("attempted tool `{tool}` must be listed in allowed tools"),
            None => "no concrete tool invocation is being checked at this boundary".to_string(),
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "budget",
        passed: input.budget_present,
        detail: "role manifest has a shadow budget boundary".to_string(),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "side_effect_class",
        passed: matches!(
            input.side_effect_class,
            "local_agent_job_fanout"
                | "local_agent_job_result_reporting"
                | "local_subagent_spawn"
                | "local_subagent_handoff"
                | "local_subagent_lifecycle"
        ),
        detail: format!("side effect class: {}", input.side_effect_class),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "output_contract",
        passed: !input.output_contract_required || input.output_contract_present,
        detail: if input.output_contract_present {
            "role manifest output contract is present".to_string()
        } else {
            "role manifest output contract is absent".to_string()
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "result_contract",
        passed: !input.result_contract_required || input.result_contract_present,
        detail: if input.result_contract_present {
            "role manifest TaskResult contract is present".to_string()
        } else {
            "role manifest TaskResult contract is absent".to_string()
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "verifier",
        passed: input.verifier_present,
        detail: "role manifest declares a verifier".to_string(),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "reducer",
        passed: input.reducer_present,
        detail: "role manifest declares a reducer".to_string(),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "lane",
        passed: !input.lane.trim().is_empty()
            && input
                .observed_lane
                .is_none_or(|observed_lane| observed_lane == input.lane),
        detail: match input.observed_lane {
            Some(observed_lane) => format!(
                "observed lane `{observed_lane}` must match role manifest lane `{}`",
                input.lane
            ),
            None => format!("role manifest lane: {}", input.lane),
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "configured_manifest_current",
        passed: !configured_manifest_shadow_decision.configured_manifest_present
            || !configured_manifest_shadow_decision.stale,
        detail: configured_manifest_shadow_decision
            .staleness_reason
            .clone()
            .unwrap_or_else(|| {
                "configured AgentCard manifest is current with the runtime registry".to_string()
            }),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "configured_manifest_compatible",
        passed: !configured_manifest_shadow_decision.configured_manifest_present
            || configured_manifest_shadow_decision.source_compatible,
        detail: configured_manifest_shadow_decision
            .compatibility_reason
            .clone()
            .unwrap_or_else(|| {
                "configured AgentCard manifest source is compatible with registry merge".to_string()
            }),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "configured_manifest_overlay",
        passed: !configured_manifest_shadow_decision.configured_manifest_present
            || configured_manifest_shadow_decision
                .configured_manifest_overlay_shadow_decision
                .checks
                .iter()
                .all(|check| check.passed),
        detail: format!(
            "configured AgentCard manifest overlay returned {}",
            configured_manifest_shadow_decision
                .configured_manifest_overlay_shadow_decision
                .decision
        ),
    });
    let lane_allowed = input
        .observed_lane
        .map(|observed_lane| observed_lane == input.lane);

    let denial_reasons = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let decision = if denial_reasons.is_empty() {
        ROLE_MANIFEST_ALLOW_SHADOW
    } else {
        ROLE_MANIFEST_DENY_SHADOW
    };
    let role_key = input
        .role_name
        .as_deref()
        .filter(|role| !role.trim().is_empty())
        .unwrap_or("default");
    let manifest_id = format!("agent-card:{}:{role_key}", input.source_surface_id);
    let promotion_readiness_shadow_decision = build_promotion_readiness_shadow_decision(
        &input,
        manifest_id.clone(),
        &configured_manifest_shadow_decision,
        &checks,
    );
    let task_result_contract_shadow_plan = build_task_result_contract_shadow_plan(&input);

    WorkGraphRoleManifestShadowDecision {
        source_surface_id: input.source_surface_id,
        manifest_id,
        definition_source: input.definition_source,
        manifest_version: input.manifest_version,
        configured_manifest_shadow_decision,
        promotion_readiness_shadow_decision,
        task_result_contract_shadow_plan,
        decision,
        role_name: input.role_name,
        description: input.description,
        role_declared: input.role_declared,
        capabilities: input.capabilities,
        allowed_tools: input.allowed_tools,
        attempted_tool: input.attempted_tool,
        tool_allowed,
        side_effect_class: input.side_effect_class,
        output_contract_required: input.output_contract_required,
        output_contract_present: input.output_contract_present,
        result_contract_required: input.result_contract_required,
        result_contract_present: input.result_contract_present,
        verifier_present: input.verifier_present,
        reducer_present: input.reducer_present,
        lane: input.lane,
        observed_lane: input.observed_lane,
        lane_allowed,
        denial_reasons,
        checks,
        feature_flag_id: "work_graph_role_manifest_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn build_promotion_readiness_shadow_decision(
    input: &WorkGraphRoleManifestShadowInput,
    manifest_id: String,
    configured_manifest_shadow_decision: &WorkGraphConfiguredManifestShadowDecision,
    role_manifest_checks: &[WorkGraphAdmissionShadowCheck],
) -> WorkGraphPromotionReadinessShadowDecision {
    let configured_manifest_ready = configured_manifest_shadow_decision.configured_manifest_present
        && configured_manifest_shadow_decision.version_matches
        && configured_manifest_shadow_decision.source_compatible
        && !configured_manifest_shadow_decision.stale;
    let configured_overlay_ready = configured_manifest_shadow_decision
        .configured_manifest_overlay_shadow_decision
        .document_present
        && configured_manifest_shadow_decision
            .configured_manifest_overlay_shadow_decision
            .checks
            .iter()
            .all(|check| check.passed);
    let role_contracts_ready = input.role_declared
        && input.role_description_present
        && !input.capabilities.is_empty()
        && !input.allowed_tools.is_empty();
    let budget_ready = input.budget_present;
    let lane_ready = !input.lane.trim().is_empty()
        && input
            .observed_lane
            .is_none_or(|observed_lane| observed_lane == input.lane);
    let attempted_tool_ready = input.attempted_tool.is_none_or(|attempted_tool| {
        input
            .allowed_tools
            .iter()
            .any(|allowed_tool| allowed_tool == attempted_tool)
    });
    let result_contract_ready = !input.result_contract_required || input.result_contract_present;
    let verifier_reducer_ready = input.verifier_present && input.reducer_present;

    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "configured_manifest_ready",
            passed: configured_manifest_ready,
            detail:
                "configured manifest must be present, current, source-compatible, and non-stale"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "configured_overlay_ready",
            passed: configured_overlay_ready,
            detail:
                "configured manifest document overlay must match registry capabilities, tools, lane, and budget"
                    .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "role_contracts_ready",
            passed: role_contracts_ready,
            detail: "role declaration, description, capabilities, and allowed tools must be complete"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "budget_ready",
            passed: budget_ready,
            detail: "role manifest must expose a shadow budget boundary".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "lane_ready",
            passed: lane_ready,
            detail: "observed lane must match the role manifest lane".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "attempted_tool_ready",
            passed: attempted_tool_ready,
            detail: "attempted tool must be allowed by the role manifest".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "result_contract_ready",
            passed: result_contract_ready,
            detail: "required result contract must be present".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "verifier_reducer_ready",
            passed: verifier_reducer_ready,
            detail: "verifier and reducer must be present".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "role_manifest_checks_ready",
            passed: role_manifest_checks.iter().all(|check| check.passed),
            detail: "all role manifest shadow checks must pass before promotion".to_string(),
        },
    ];
    let denial_reasons = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let decision = if denial_reasons.is_empty() {
        PROMOTION_READY_SHADOW
    } else {
        PROMOTION_NOT_READY_SHADOW
    };

    WorkGraphPromotionReadinessShadowDecision {
        source_surface_id: input.source_surface_id,
        decision,
        manifest_id,
        promotion_stage: "shadow_only",
        configured_manifest_ready,
        configured_overlay_ready,
        role_contracts_ready,
        budget_ready,
        lane_ready,
        attempted_tool_ready,
        result_contract_ready,
        verifier_reducer_ready,
        denial_reasons,
        checks,
        feature_flag_id: "work_graph_role_manifest_promotion_readiness_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn build_task_result_contract_shadow_plan(
    input: &WorkGraphRoleManifestShadowInput,
) -> WorkGraphTaskResultContractShadowPlan {
    let result_contract_ready = !input.result_contract_required || input.result_contract_present;
    let verifier_ready = !input.result_contract_required || input.verifier_present;
    let reducer_ready = !input.result_contract_required || input.reducer_present;
    let contract_plan_ready = result_contract_ready && verifier_ready && reducer_ready;
    let mut missing_contract_parts = Vec::new();
    if input.result_contract_required && !input.result_contract_present {
        missing_contract_parts.push("task_result_contract".to_string());
    }
    if input.result_contract_required && !input.verifier_present {
        missing_contract_parts.push("verifier".to_string());
    }
    if input.result_contract_required && !input.reducer_present {
        missing_contract_parts.push("reducer".to_string());
    }
    let (task_result_contract_id, terminal_delivery_surface, verifier_id, reducer_id) =
        task_result_contract_plan_ids(input.source_surface_id);
    let planned_shadow_events = task_result_contract_planned_shadow_events(input.source_surface_id);
    let next_actions = task_result_contract_next_actions(&missing_contract_parts);
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "task_result_contract",
            passed: result_contract_ready,
            detail: if input.result_contract_required {
                "direct work surface must declare a terminal TaskResultEnvelope contract"
                    .to_string()
            } else {
                "this surface does not require a terminal TaskResultEnvelope contract".to_string()
            },
        },
        WorkGraphAdmissionShadowCheck {
            name: "task_result_verifier",
            passed: verifier_ready,
            detail: "required TaskResult contract must name a verifier before promotion"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "task_result_reducer",
            passed: reducer_ready,
            detail: "required TaskResult contract must name a reducer before promotion"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "terminal_delivery_surface",
            passed: !terminal_delivery_surface.is_empty(),
            detail: format!(
                "terminal TaskResult evidence is planned through `{terminal_delivery_surface}`"
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_contract_mutation",
            passed: true,
            detail:
                "TaskResult contract planning is shadow-only and does not mutate spawn, wait, or reducer behavior"
                    .to_string(),
        },
    ];
    let decision = if contract_plan_ready {
        TASK_RESULT_CONTRACT_PLAN_READY_SHADOW
    } else {
        TASK_RESULT_CONTRACT_PLAN_BLOCKED_SHADOW
    };

    WorkGraphTaskResultContractShadowPlan {
        source_surface_id: input.source_surface_id,
        decision,
        plan_stage: "task_result_contract_shadow_planning",
        result_contract_required: input.result_contract_required,
        result_contract_present: input.result_contract_present,
        verifier_present: input.verifier_present,
        reducer_present: input.reducer_present,
        result_contract_ready,
        verifier_ready,
        reducer_ready,
        contract_plan_ready,
        task_result_contract_id,
        result_envelope_schema: "hepta.task_result_envelope.v1",
        terminal_delivery_surface,
        verifier_id,
        reducer_id,
        missing_contract_parts,
        planned_shadow_events,
        next_actions,
        checks,
        feature_flag_id: "work_graph_task_result_contract_shadow_plan",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn task_result_contract_plan_ids(
    source_surface_id: &'static str,
) -> (&'static str, &'static str, &'static str, &'static str) {
    match source_surface_id {
        "spawn_agent" | "spawn_agent_v2" => (
            "subagent_task_result_contract_v1",
            "wait_agent(result_required=true)",
            "subagent_task_result_verifier_v1",
            "subagent_parent_reducer_v1",
        ),
        "spawn_agents_on_csv" | "report_agent_job_result" => (
            "agent_job_task_result_contract_v1",
            "report_agent_job_result",
            "agent_job_output_schema_verifier_v1",
            "agent_job_csv_result_reducer_v1",
        ),
        _ => (
            "task_result_contract_not_required",
            "not_required",
            "not_required",
            "not_required",
        ),
    }
}

fn task_result_contract_planned_shadow_events(source_surface_id: &'static str) -> Vec<String> {
    match source_surface_id {
        "spawn_agent" | "spawn_agent_v2" => vec![
            "subagent_task_result_contract_declared".to_string(),
            "wait_agent_result_required_terminal_evidence".to_string(),
            "subagent_parent_reducer_shadow_receipt".to_string(),
        ],
        "spawn_agents_on_csv" | "report_agent_job_result" => vec![
            "agent_job_task_result_reported".to_string(),
            "agent_job_result_verified".to_string(),
            "agent_job_result_reduced".to_string(),
        ],
        _ => vec!["task_result_contract_not_required".to_string()],
    }
}

fn task_result_contract_next_actions(missing_contract_parts: &[String]) -> Vec<String> {
    if missing_contract_parts.is_empty() {
        return vec![
            "keep TaskResult contract under shadow verification; do not enable reviewed flag, canary, blocking, or cutover".to_string(),
        ];
    }
    missing_contract_parts
        .iter()
        .map(|part| match part.as_str() {
            "task_result_contract" => {
                "declare the direct subagent TaskResultEnvelope result contract".to_string()
            }
            "verifier" => {
                "wire a shadow verifier for direct subagent TaskResultEnvelope evidence"
                    .to_string()
            }
            "reducer" => {
                "wire a shadow reducer from direct subagent TaskResultEnvelope evidence into the parent WorkGraph"
                    .to_string()
            }
            _ => format!("resolve missing TaskResult contract part `{part}`"),
        })
        .collect()
}

fn build_configured_manifest_shadow_decision(
    input: &WorkGraphRoleManifestShadowInput,
) -> WorkGraphConfiguredManifestShadowDecision {
    let configured_manifest_present = input.configured_manifest_source.is_some();
    let version_matches = input
        .configured_manifest_version
        .as_deref()
        .is_some_and(|version| version == input.manifest_version);
    let source_compatible = input
        .configured_manifest_source
        .as_deref()
        .is_some_and(configured_manifest_source_is_compatible);
    let configured_manifest_overlay_shadow_decision =
        build_configured_manifest_overlay_shadow_decision(
            WorkGraphConfiguredManifestOverlayShadowInput {
                source_surface_id: input.source_surface_id,
                configured_manifest_source: input.configured_manifest_source.clone(),
                overlay: input.configured_manifest_overlay.as_ref(),
                expected_capabilities: &input.capabilities,
                expected_allowed_tools: &input.allowed_tools,
                expected_lane: input.lane,
                budget_present: input.budget_present,
            },
        );
    let stale = configured_manifest_present && !version_matches;
    let compatibility_reason = if !configured_manifest_present {
        Some(
            "no configured AgentCard manifest overlay is present; default registry is active"
                .to_string(),
        )
    } else if source_compatible {
        None
    } else {
        Some(
            "configured AgentCard manifest source must use an agent-card:// URI before registry merge"
                .to_string(),
        )
    };
    let staleness_reason = if !configured_manifest_present {
        Some(
            "no configured AgentCard manifest overlay is present; default registry is active"
                .to_string(),
        )
    } else if version_matches {
        None
    } else if input.configured_manifest_version.is_some() {
        Some(format!(
            "configured AgentCard manifest version must match {}",
            input.manifest_version
        ))
    } else {
        Some(
            "configured role metadata has no AgentCard manifest version yet; default registry remains authoritative"
                .to_string(),
        )
    };
    let mut checks = Vec::new();
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "configured_manifest_presence",
        passed: configured_manifest_present,
        detail: if configured_manifest_present {
            "configured role/tool manifest overlay is visible to this boundary".to_string()
        } else {
            "no configured role/tool manifest overlay was supplied for this boundary".to_string()
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "configured_manifest_source",
        passed: !configured_manifest_present || source_compatible,
        detail: compatibility_reason.clone().unwrap_or_else(|| {
            "configured AgentCard manifest source is compatible with registry merge".to_string()
        }),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "configured_manifest_version",
        passed: !configured_manifest_present || version_matches,
        detail: match input.configured_manifest_version.as_deref() {
            Some(version) => format!(
                "configured version `{version}` should match expected `{}`",
                input.manifest_version
            ),
            None => "configured manifest version is absent".to_string(),
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "configured_manifest_staleness",
        passed: !stale,
        detail: staleness_reason.clone().unwrap_or_else(|| {
            "configured AgentCard manifest is current with the runtime registry".to_string()
        }),
    });
    let decision = if !configured_manifest_present {
        CONFIGURED_MANIFEST_MISSING_SHADOW
    } else if stale {
        CONFIGURED_MANIFEST_VERSION_DRIFT_SHADOW
    } else if !source_compatible {
        CONFIGURED_MANIFEST_INCOMPATIBLE_SHADOW
    } else {
        CONFIGURED_MANIFEST_PRESENT_SHADOW
    };

    WorkGraphConfiguredManifestShadowDecision {
        source_surface_id: input.source_surface_id,
        decision,
        registry_source: AGENT_CARD_REGISTRY_SOURCE,
        configured_manifest_present,
        configured_manifest_source: input.configured_manifest_source.clone(),
        expected_manifest_version: input.manifest_version,
        configured_manifest_version: input.configured_manifest_version.clone(),
        version_matches,
        source_compatible,
        configured_manifest_overlay_shadow_decision,
        stale,
        compatibility_reason,
        staleness_reason,
        checks,
        feature_flag_id: "work_graph_configured_agent_card_manifest_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn configured_manifest_source_is_compatible(source: &str) -> bool {
    source.trim().starts_with("agent-card://")
}

pub(crate) fn build_work_graph_admission_shadow_decision(
    input: WorkGraphAdmissionShadowInput,
) -> WorkGraphAdmissionShadowDecision {
    let mut checks = Vec::new();
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "dependencies_terminal",
        passed: true,
        detail: "no explicit upstream dependencies were supplied for this entrypoint".to_string(),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "lane_lease",
        passed: true,
        detail: "lane lease is observed as shadow metadata only; no live lock is mutated"
            .to_string(),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "depth_budget",
        passed: !input.enforce_depth_limit || input.child_depth <= input.max_depth,
        detail: if input.enforce_depth_limit {
            format!(
                "child depth {} must be <= max depth {}",
                input.child_depth, input.max_depth
            )
        } else {
            format!(
                "child depth {} observed; this entrypoint does not enforce max depth in shadow admission",
                input.child_depth
            )
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "thread_budget",
        passed: input.max_threads.is_none_or(|max_threads| {
            max_threads > 0 && input.requested_concurrency <= max_threads
        }),
        detail: match input.max_threads {
            Some(max_threads) => format!(
                "requested concurrency {} must be <= max threads {}",
                input.requested_concurrency, max_threads
            ),
            None => format!(
                "requested concurrency {} with no configured max thread cap",
                input.requested_concurrency
            ),
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "state_db",
        passed: !input.state_db_required || input.state_db_available,
        detail: if input.state_db_required {
            "state database is required for this WorkGraph shadow entrypoint".to_string()
        } else {
            "state database is optional for this WorkGraph shadow entrypoint".to_string()
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "side_effect_class",
        passed: matches!(
            input.side_effect_class,
            "local_agent_job_fanout"
                | "local_agent_job_result_reporting"
                | "local_subagent_spawn"
                | "local_subagent_handoff"
                | "local_subagent_lifecycle"
        ),
        detail: format!("side effect class: {}", input.side_effect_class),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "idempotency",
        passed: true,
        detail:
            "idempotency is recorded as a shadow readiness signal; no replay mutation is enabled"
                .to_string(),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "output_contract",
        passed: !input.output_contract_required || input.output_contract_present,
        detail: if input.output_contract_present {
            "output contract is present".to_string()
        } else {
            "output contract is absent".to_string()
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "result_contract",
        passed: !input.result_contract_required || input.result_contract_present,
        detail: if input.result_contract_present {
            "TaskResult/result reporting contract is present".to_string()
        } else {
            "TaskResult/result reporting contract is absent".to_string()
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "reducer",
        passed: !input.reducer_required || input.reducer_present,
        detail: if input.reducer_present {
            "result reducer is present".to_string()
        } else {
            "result reducer is absent".to_string()
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "role_manifest",
        passed: input
            .role_manifest_shadow_decision
            .denial_reasons
            .is_empty(),
        detail: format!(
            "role manifest {} returned {}",
            input.role_manifest_shadow_decision.manifest_id,
            input.role_manifest_shadow_decision.decision
        ),
    });

    let denial_reasons = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.detail))
        .collect::<Vec<_>>();
    let decision = if denial_reasons.is_empty() {
        ADMISSION_ALLOW_SHADOW
    } else {
        ADMISSION_DENY_SHADOW
    };

    WorkGraphAdmissionShadowDecision {
        source_surface_id: input.source_surface_id,
        decision,
        task_id: input.task_id,
        job_id: input.job_id,
        role_manifest_shadow_decision: input.role_manifest_shadow_decision,
        requested_concurrency: input.requested_concurrency,
        item_count: input.item_count,
        child_depth: input.child_depth,
        max_depth: input.max_depth,
        max_threads: input.max_threads,
        side_effect_class: input.side_effect_class,
        output_contract_required: input.output_contract_required,
        output_contract_present: input.output_contract_present,
        result_contract_required: input.result_contract_required,
        result_contract_present: input.result_contract_present,
        reducer_required: input.reducer_required,
        reducer_present: input.reducer_present,
        denial_reasons,
        checks,
        feature_flag_id: "work_graph_admission_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn role_manifest(
        source_surface_id: &'static str,
        result_contract_present: bool,
    ) -> WorkGraphRoleManifestShadowDecision {
        build_agent_card_manifest_shadow_decision(
            WorkGraphAgentCardManifest {
                source_surface_id,
                ..agent_jobs_worker_agent_card_manifest()
            },
            WorkGraphAgentCardManifestObservation {
                role_name: None,
                role_declared: true,
                role_description_present: true,
                configured_manifest_source: None,
                configured_manifest_version: None,
                configured_manifest_overlay: None,
                budget_present: true,
                output_contract_present: Some(false),
                result_contract_present: Some(result_contract_present),
                verifier_present: Some(result_contract_present),
                reducer_present: Some(result_contract_present),
                attempted_tool: Some("report_agent_job_result"),
                observed_lane: Some("agent_jobs"),
            },
        )
    }

    fn agent_job_worker_overlay() -> AgentCardManifestConfig {
        AgentCardManifestConfig {
            schema_version: Some(AGENT_CARD_MANIFEST_VERSION.to_string()),
            source_surface_id: Some("spawn_agents_on_csv".to_string()),
            capabilities: vec![
                "csv_row_processing".to_string(),
                "task_result_reporting".to_string(),
                "work_graph_shadow_event_emission".to_string(),
            ],
            allowed_tools: vec!["report_agent_job_result".to_string()],
            lane: Some("agent_jobs".to_string()),
            max_threads: Some(8),
            max_depth: None,
        }
    }

    #[test]
    fn agent_card_manifest_registry_resolves_current_governed_surfaces() {
        let registry = default_agent_card_manifest_registry();
        let expected = [
            (
                "spawn_agents_on_csv",
                WorkGraphAgentCardManifestProfile::AgentJobWorker,
                "agent_jobs",
            ),
            (
                "report_agent_job_result",
                WorkGraphAgentCardManifestProfile::AgentJobResultReporter,
                "agent_jobs",
            ),
            (
                "spawn_agent",
                WorkGraphAgentCardManifestProfile::SubagentSpawn,
                "subagent",
            ),
            (
                "spawn_agent_v2",
                WorkGraphAgentCardManifestProfile::SubagentNamedSpawn,
                "subagent",
            ),
            (
                "send_message",
                WorkGraphAgentCardManifestProfile::SubagentHandoff,
                "subagent_handoff",
            ),
            (
                "followup_task",
                WorkGraphAgentCardManifestProfile::SubagentHandoff,
                "subagent_handoff",
            ),
            (
                "close_agent",
                WorkGraphAgentCardManifestProfile::SubagentLifecycle,
                "subagent_lifecycle",
            ),
            (
                "wait_agent",
                WorkGraphAgentCardManifestProfile::SubagentLifecycle,
                "subagent_lifecycle",
            ),
        ];

        assert_eq!(registry.entries().len(), expected.len());
        for (source_surface_id, profile, lane) in expected {
            let entry = registry
                .entries()
                .iter()
                .find(|entry| entry.source_surface_id == source_surface_id)
                .expect("registry entry should exist");
            assert_eq!(entry.profile, profile);
            assert_eq!(entry.manifest.source_surface_id, source_surface_id);
            assert_eq!(entry.manifest.lane, lane);
            assert!(!entry.manifest.capabilities.is_empty());
            assert!(!entry.manifest.allowed_tools.is_empty());
            assert_eq!(
                registry
                    .manifest_for_source(source_surface_id)
                    .expect("manifest should resolve"),
                entry.manifest
            );
        }

        let v2_spawn = registry
            .manifest_for_source("spawn_agent_v2")
            .expect("v2 spawn manifest should resolve");
        assert!(v2_spawn.capabilities.contains(&"named_task_path"));
        assert!(v2_spawn.allowed_tools.contains(&"close_agent"));
    }

    #[test]
    fn agent_card_manifest_shadow_checks_attempted_tool_and_lane() {
        let decision = build_agent_card_manifest_shadow_decision(
            subagent_handoff_agent_card_manifest("send_message"),
            WorkGraphAgentCardManifestObservation {
                role_name: None,
                role_declared: true,
                role_description_present: true,
                configured_manifest_source: None,
                configured_manifest_version: None,
                configured_manifest_overlay: None,
                budget_present: true,
                output_contract_present: None,
                result_contract_present: None,
                verifier_present: None,
                reducer_present: None,
                attempted_tool: Some("report_agent_job_result"),
                observed_lane: Some("agent_jobs"),
            },
        );

        assert_eq!(decision.decision, ROLE_MANIFEST_DENY_SHADOW);
        assert_eq!(decision.definition_source, AGENT_CARD_DEFINITION_SOURCE);
        assert_eq!(decision.attempted_tool, Some("report_agent_job_result"));
        assert_eq!(decision.tool_allowed, Some(false));
        assert_eq!(decision.observed_lane, Some("agent_jobs"));
        assert_eq!(decision.lane_allowed, Some(false));
        assert!(
            decision
                .denial_reasons
                .iter()
                .any(|reason| reason.contains("attempted_tool"))
        );
        assert!(
            decision
                .denial_reasons
                .iter()
                .any(|reason| reason.contains("lane"))
        );
    }

    #[test]
    fn configured_manifest_shadow_tracks_presence_and_version_drift() {
        let default_only = build_agent_card_manifest_shadow_decision(
            agent_jobs_worker_agent_card_manifest(),
            WorkGraphAgentCardManifestObservation {
                role_name: None,
                role_declared: true,
                role_description_present: true,
                configured_manifest_source: None,
                configured_manifest_version: None,
                configured_manifest_overlay: None,
                budget_present: true,
                output_contract_present: Some(false),
                result_contract_present: None,
                verifier_present: None,
                reducer_present: None,
                attempted_tool: Some("report_agent_job_result"),
                observed_lane: Some("agent_jobs"),
            },
        );
        assert_eq!(
            default_only.configured_manifest_shadow_decision.decision,
            CONFIGURED_MANIFEST_MISSING_SHADOW
        );
        assert!(
            !default_only
                .configured_manifest_shadow_decision
                .configured_manifest_present
        );
        assert!(
            default_only
                .configured_manifest_shadow_decision
                .staleness_reason
                .as_deref()
                .is_some_and(|reason| reason.contains("default registry"))
        );
        assert_eq!(
            default_only.promotion_readiness_shadow_decision.decision,
            PROMOTION_NOT_READY_SHADOW
        );
        assert!(
            !default_only
                .promotion_readiness_shadow_decision
                .configured_manifest_ready
        );

        let configured_without_version = build_agent_card_manifest_shadow_decision(
            agent_jobs_worker_agent_card_manifest(),
            WorkGraphAgentCardManifestObservation {
                role_name: Some("agent_job_worker".to_string()),
                role_declared: true,
                role_description_present: true,
                configured_manifest_source: Some(
                    "agent_role_config_inline:agent_job_worker".to_string(),
                ),
                configured_manifest_version: None,
                configured_manifest_overlay: None,
                budget_present: true,
                output_contract_present: Some(false),
                result_contract_present: None,
                verifier_present: None,
                reducer_present: None,
                attempted_tool: Some("report_agent_job_result"),
                observed_lane: Some("agent_jobs"),
            },
        );
        assert_eq!(
            configured_without_version
                .configured_manifest_shadow_decision
                .decision,
            CONFIGURED_MANIFEST_VERSION_DRIFT_SHADOW
        );
        assert!(
            configured_without_version
                .configured_manifest_shadow_decision
                .configured_manifest_present
        );
        assert!(
            configured_without_version
                .configured_manifest_shadow_decision
                .stale
        );
        assert!(
            !configured_without_version
                .configured_manifest_shadow_decision
                .source_compatible
        );
        assert!(
            configured_without_version
                .configured_manifest_shadow_decision
                .staleness_reason
                .as_deref()
                .is_some_and(|reason| reason.contains("no AgentCard manifest version"))
        );
        assert_eq!(
            configured_without_version.decision,
            ROLE_MANIFEST_DENY_SHADOW
        );
        assert!(
            configured_without_version
                .denial_reasons
                .iter()
                .any(|reason| {
                    reason.contains("configured_manifest_current")
                        && reason.contains("no AgentCard manifest version")
                })
        );
        assert_eq!(
            configured_without_version
                .configured_manifest_shadow_decision
                .configured_manifest_overlay_shadow_decision
                .decision,
            "configured_manifest_overlay_missing_shadow_no_live_blocking"
        );

        let configured_incompatible = build_agent_card_manifest_shadow_decision(
            agent_jobs_worker_agent_card_manifest(),
            WorkGraphAgentCardManifestObservation {
                role_name: Some("agent_job_worker".to_string()),
                role_declared: true,
                role_description_present: true,
                configured_manifest_source: Some(
                    "agent_role_config_inline:agent_job_worker".to_string(),
                ),
                configured_manifest_version: Some(AGENT_CARD_MANIFEST_VERSION.to_string()),
                configured_manifest_overlay: None,
                budget_present: true,
                output_contract_present: Some(false),
                result_contract_present: None,
                verifier_present: None,
                reducer_present: None,
                attempted_tool: Some("report_agent_job_result"),
                observed_lane: Some("agent_jobs"),
            },
        );
        assert_eq!(
            configured_incompatible
                .configured_manifest_shadow_decision
                .decision,
            CONFIGURED_MANIFEST_INCOMPATIBLE_SHADOW
        );
        assert!(
            !configured_incompatible
                .configured_manifest_shadow_decision
                .source_compatible
        );
        assert_eq!(configured_incompatible.decision, ROLE_MANIFEST_DENY_SHADOW);
        assert!(
            configured_incompatible
                .denial_reasons
                .iter()
                .any(|reason| reason.contains("configured_manifest_compatible"))
        );

        let configured_overlay_diff = build_agent_card_manifest_shadow_decision(
            agent_jobs_worker_agent_card_manifest(),
            WorkGraphAgentCardManifestObservation {
                role_name: Some("agent_job_worker".to_string()),
                role_declared: true,
                role_description_present: true,
                configured_manifest_source: Some("agent-card://agent_job_worker".to_string()),
                configured_manifest_version: Some(AGENT_CARD_MANIFEST_VERSION.to_string()),
                configured_manifest_overlay: Some(AgentCardManifestConfig {
                    schema_version: Some(AGENT_CARD_MANIFEST_VERSION.to_string()),
                    source_surface_id: Some("spawn_agents_on_csv".to_string()),
                    capabilities: vec!["task_result_reporting".to_string()],
                    allowed_tools: vec!["send_message".to_string()],
                    lane: Some("subagent".to_string()),
                    max_threads: None,
                    max_depth: None,
                }),
                budget_present: true,
                output_contract_present: Some(false),
                result_contract_present: None,
                verifier_present: None,
                reducer_present: None,
                attempted_tool: Some("report_agent_job_result"),
                observed_lane: Some("agent_jobs"),
            },
        );
        assert_eq!(
            configured_overlay_diff
                .configured_manifest_shadow_decision
                .decision,
            CONFIGURED_MANIFEST_PRESENT_SHADOW
        );
        assert_eq!(
            configured_overlay_diff
                .configured_manifest_shadow_decision
                .configured_manifest_overlay_shadow_decision
                .decision,
            "configured_manifest_overlay_diff_shadow_no_live_blocking"
        );
        assert_eq!(configured_overlay_diff.decision, ROLE_MANIFEST_DENY_SHADOW);
        assert_eq!(
            configured_overlay_diff
                .promotion_readiness_shadow_decision
                .decision,
            PROMOTION_NOT_READY_SHADOW
        );
        assert!(
            !configured_overlay_diff
                .promotion_readiness_shadow_decision
                .configured_overlay_ready
        );
        assert!(
            configured_overlay_diff
                .denial_reasons
                .iter()
                .any(|reason| reason.contains("configured_manifest_overlay"))
        );

        let configured_current = build_agent_card_manifest_shadow_decision(
            agent_jobs_worker_agent_card_manifest(),
            WorkGraphAgentCardManifestObservation {
                role_name: Some("agent_job_worker".to_string()),
                role_declared: true,
                role_description_present: true,
                configured_manifest_source: Some("agent-card://agent_job_worker".to_string()),
                configured_manifest_version: Some(AGENT_CARD_MANIFEST_VERSION.to_string()),
                configured_manifest_overlay: Some(agent_job_worker_overlay()),
                budget_present: true,
                output_contract_present: Some(false),
                result_contract_present: None,
                verifier_present: None,
                reducer_present: None,
                attempted_tool: Some("report_agent_job_result"),
                observed_lane: Some("agent_jobs"),
            },
        );
        assert_eq!(
            configured_current
                .configured_manifest_shadow_decision
                .decision,
            CONFIGURED_MANIFEST_PRESENT_SHADOW
        );
        assert!(
            configured_current
                .configured_manifest_shadow_decision
                .version_matches
        );
        assert!(
            configured_current
                .configured_manifest_shadow_decision
                .source_compatible
        );
        assert_eq!(
            configured_current
                .configured_manifest_shadow_decision
                .configured_manifest_overlay_shadow_decision
                .decision,
            "configured_manifest_overlay_compatible_shadow_no_live_blocking"
        );
        assert!(!configured_current.configured_manifest_shadow_decision.stale);
        assert_eq!(configured_current.decision, ROLE_MANIFEST_ALLOW_SHADOW);
        assert_eq!(
            configured_current
                .promotion_readiness_shadow_decision
                .decision,
            PROMOTION_READY_SHADOW
        );
        assert!(
            configured_current
                .promotion_readiness_shadow_decision
                .configured_manifest_ready
        );
        assert!(
            configured_current
                .promotion_readiness_shadow_decision
                .configured_overlay_ready
        );
        assert!(
            !configured_current
                .promotion_readiness_shadow_decision
                .live_blocking_enabled
        );
        assert!(
            !configured_current
                .promotion_readiness_shadow_decision
                .live_cutover_enabled
        );
    }

    #[test]
    fn admission_shadow_allows_agent_job_fanout_with_contracts() {
        let decision = build_work_graph_admission_shadow_decision(WorkGraphAdmissionShadowInput {
            source_surface_id: "spawn_agents_on_csv",
            task_id: Some("agent-job:job-1".to_string()),
            job_id: Some("job-1".to_string()),
            role_manifest_shadow_decision: role_manifest("spawn_agents_on_csv", true),
            requested_concurrency: 4,
            item_count: Some(10),
            child_depth: 1,
            max_depth: 4,
            max_threads: Some(8),
            enforce_depth_limit: true,
            state_db_required: true,
            state_db_available: true,
            output_contract_required: false,
            output_contract_present: true,
            result_contract_required: true,
            result_contract_present: true,
            reducer_required: true,
            reducer_present: true,
            side_effect_class: "local_agent_job_fanout",
        });

        assert_eq!(decision.decision, ADMISSION_ALLOW_SHADOW);
        assert_eq!(
            decision.role_manifest_shadow_decision.decision,
            ROLE_MANIFEST_ALLOW_SHADOW
        );
        let plan = &decision
            .role_manifest_shadow_decision
            .task_result_contract_shadow_plan;
        assert_eq!(plan.decision, TASK_RESULT_CONTRACT_PLAN_READY_SHADOW);
        assert_eq!(
            plan.task_result_contract_id,
            "agent_job_task_result_contract_v1"
        );
        assert_eq!(plan.terminal_delivery_surface, "report_agent_job_result");
        assert!(plan.contract_plan_ready);
        assert!(decision.denial_reasons.is_empty());
        assert!(!decision.live_blocking_enabled);
        assert!(!decision.live_cutover_enabled);
    }

    #[test]
    fn admission_shadow_denies_spawn_without_task_result_contract() {
        let decision = build_work_graph_admission_shadow_decision(WorkGraphAdmissionShadowInput {
            source_surface_id: "spawn_agent",
            task_id: Some("spawn-agent:thread:call".to_string()),
            job_id: None,
            role_manifest_shadow_decision: build_agent_card_manifest_shadow_decision(
                subagent_spawn_agent_card_manifest("spawn_agent"),
                WorkGraphAgentCardManifestObservation {
                    role_name: None,
                    role_declared: true,
                    role_description_present: true,
                    configured_manifest_source: None,
                    configured_manifest_version: None,
                    configured_manifest_overlay: None,
                    budget_present: true,
                    output_contract_present: None,
                    result_contract_present: None,
                    verifier_present: None,
                    reducer_present: None,
                    attempted_tool: None,
                    observed_lane: Some("subagent"),
                },
            ),
            requested_concurrency: 1,
            item_count: Some(1),
            child_depth: 1,
            max_depth: 4,
            max_threads: Some(8),
            enforce_depth_limit: true,
            state_db_required: false,
            state_db_available: false,
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: true,
            result_contract_present: false,
            reducer_required: false,
            reducer_present: false,
            side_effect_class: "local_subagent_spawn",
        });

        assert_eq!(decision.decision, ADMISSION_DENY_SHADOW);
        assert_eq!(
            decision.role_manifest_shadow_decision.decision,
            ROLE_MANIFEST_DENY_SHADOW
        );
        assert!(
            decision
                .denial_reasons
                .iter()
                .any(|reason| reason.contains("result_contract"))
        );
        assert!(!decision.live_blocking_enabled);
        assert!(!decision.live_cutover_enabled);
    }

    #[test]
    fn task_result_contract_shadow_plan_blocks_direct_spawn_contract_gap() {
        let decision = build_agent_card_manifest_shadow_decision(
            subagent_spawn_agent_card_manifest("spawn_agent_v2"),
            WorkGraphAgentCardManifestObservation {
                role_name: Some("reviewer".to_string()),
                role_declared: true,
                role_description_present: true,
                configured_manifest_source: None,
                configured_manifest_version: None,
                configured_manifest_overlay: None,
                budget_present: true,
                output_contract_present: None,
                result_contract_present: None,
                verifier_present: None,
                reducer_present: None,
                attempted_tool: None,
                observed_lane: Some("subagent"),
            },
        );

        let plan = &decision.task_result_contract_shadow_plan;
        assert_eq!(plan.source_surface_id, "spawn_agent_v2");
        assert_eq!(plan.decision, TASK_RESULT_CONTRACT_PLAN_BLOCKED_SHADOW);
        assert_eq!(plan.plan_stage, "task_result_contract_shadow_planning");
        assert_eq!(
            plan.task_result_contract_id,
            "subagent_task_result_contract_v1"
        );
        assert_eq!(plan.result_envelope_schema, "hepta.task_result_envelope.v1");
        assert_eq!(
            plan.terminal_delivery_surface,
            "wait_agent(result_required=true)"
        );
        assert_eq!(plan.verifier_id, "subagent_task_result_verifier_v1");
        assert_eq!(plan.reducer_id, "subagent_parent_reducer_v1");
        assert_eq!(
            plan.missing_contract_parts,
            vec![
                "task_result_contract".to_string(),
                "verifier".to_string(),
                "reducer".to_string()
            ]
        );
        assert!(
            plan.planned_shadow_events
                .contains(&"wait_agent_result_required_terminal_evidence".to_string())
        );
        assert!(
            plan.next_actions
                .iter()
                .any(|action| action.contains("TaskResultEnvelope"))
        );
        assert!(!plan.contract_plan_ready);
        assert!(!plan.live_blocking_enabled);
        assert!(!plan.live_cutover_enabled);
    }

    #[test]
    fn admission_shadow_denies_over_thread_budget() {
        let decision = build_work_graph_admission_shadow_decision(WorkGraphAdmissionShadowInput {
            source_surface_id: "spawn_agents_on_csv",
            task_id: Some("agent-job:job-1".to_string()),
            job_id: Some("job-1".to_string()),
            role_manifest_shadow_decision: role_manifest("spawn_agents_on_csv", true),
            requested_concurrency: 9,
            item_count: Some(10),
            child_depth: 1,
            max_depth: 4,
            max_threads: Some(8),
            enforce_depth_limit: true,
            state_db_required: true,
            state_db_available: true,
            output_contract_required: false,
            output_contract_present: false,
            result_contract_required: true,
            result_contract_present: true,
            reducer_required: true,
            reducer_present: true,
            side_effect_class: "local_agent_job_fanout",
        });

        assert_eq!(decision.decision, ADMISSION_DENY_SHADOW);
        assert!(
            decision
                .denial_reasons
                .iter()
                .any(|reason| reason.contains("thread_budget"))
        );
    }

    #[test]
    fn role_manifest_shadow_denies_missing_declared_role_contracts() {
        let decision = build_agent_card_manifest_shadow_decision(
            subagent_spawn_agent_card_manifest("spawn_agent_v2"),
            WorkGraphAgentCardManifestObservation {
                role_name: Some("reviewer".to_string()),
                role_declared: false,
                role_description_present: false,
                configured_manifest_source: Some("agent_role_config_inline:reviewer".to_string()),
                configured_manifest_version: None,
                configured_manifest_overlay: None,
                budget_present: true,
                output_contract_present: None,
                result_contract_present: None,
                verifier_present: None,
                reducer_present: None,
                attempted_tool: None,
                observed_lane: Some("subagent"),
            },
        );

        assert_eq!(decision.decision, ROLE_MANIFEST_DENY_SHADOW);
        assert!(
            decision
                .denial_reasons
                .iter()
                .any(|reason| reason.contains("role_declared"))
        );
        assert!(
            decision
                .denial_reasons
                .iter()
                .any(|reason| reason.contains("result_contract"))
        );
        assert!(!decision.live_blocking_enabled);
        assert!(!decision.live_cutover_enabled);
    }
}
