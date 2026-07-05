use crate::config::AgentCardManifestConfig;
use crate::tools::handlers::work_graph_admission::WorkGraphAdmissionShadowCheck;
use serde::Serialize;
use std::collections::BTreeSet;

const OVERLAY_COMPATIBLE_SHADOW: &str =
    "configured_manifest_overlay_compatible_shadow_no_live_blocking";
const OVERLAY_MISSING_SHADOW: &str = "configured_manifest_overlay_missing_shadow_no_live_blocking";
const OVERLAY_DIFF_SHADOW: &str = "configured_manifest_overlay_diff_shadow_no_live_blocking";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphConfiguredManifestOverlayShadowDecision {
    pub(crate) source_surface_id: &'static str,
    pub(crate) decision: &'static str,
    pub(crate) configured_manifest_source: Option<String>,
    pub(crate) document_present: bool,
    pub(crate) document_schema_version: Option<String>,
    pub(crate) source_surface_matches: Option<bool>,
    pub(crate) configured_source_surface_id: Option<String>,
    pub(crate) capabilities_compatible: Option<bool>,
    pub(crate) missing_capabilities: Vec<String>,
    pub(crate) unexpected_capabilities: Vec<String>,
    pub(crate) allowed_tools_compatible: Option<bool>,
    pub(crate) missing_allowed_tools: Vec<String>,
    pub(crate) unexpected_allowed_tools: Vec<String>,
    pub(crate) lane_matches: Option<bool>,
    pub(crate) expected_lane: &'static str,
    pub(crate) configured_lane: Option<String>,
    pub(crate) budget_compatible: Option<bool>,
    pub(crate) configured_max_threads: Option<usize>,
    pub(crate) configured_max_depth: Option<i32>,
    pub(crate) checks: Vec<WorkGraphAdmissionShadowCheck>,
    pub(crate) feature_flag_id: &'static str,
    pub(crate) feature_flag_enabled: bool,
    pub(crate) canary_stage: &'static str,
    pub(crate) canary_traffic_ppm: u32,
    pub(crate) blocking_guardrail_preview: bool,
    pub(crate) live_blocking_enabled: bool,
    pub(crate) live_cutover_enabled: bool,
}

pub(crate) struct WorkGraphConfiguredManifestOverlayShadowInput<'a> {
    pub(crate) source_surface_id: &'static str,
    pub(crate) configured_manifest_source: Option<String>,
    pub(crate) overlay: Option<&'a AgentCardManifestConfig>,
    pub(crate) expected_capabilities: &'a [String],
    pub(crate) expected_allowed_tools: &'a [String],
    pub(crate) expected_lane: &'static str,
    pub(crate) budget_present: bool,
}

pub(crate) fn build_configured_manifest_overlay_shadow_decision(
    input: WorkGraphConfiguredManifestOverlayShadowInput<'_>,
) -> WorkGraphConfiguredManifestOverlayShadowDecision {
    let document_present = input.overlay.is_some();
    let document_schema_version = input
        .overlay
        .and_then(|overlay| overlay.schema_version.clone());
    let configured_source_surface_id = input
        .overlay
        .and_then(|overlay| overlay.source_surface_id.clone());
    let source_surface_matches = input
        .overlay
        .map(|overlay| overlay.source_surface_id.as_deref() == Some(input.source_surface_id));
    let (missing_capabilities, unexpected_capabilities) = overlay_delta(
        input.expected_capabilities,
        input.overlay.map(|overlay| overlay.capabilities.as_slice()),
    );
    let capabilities_compatible = input
        .overlay
        .map(|_| missing_capabilities.is_empty() && unexpected_capabilities.is_empty());
    let (missing_allowed_tools, unexpected_allowed_tools) = overlay_delta(
        input.expected_allowed_tools,
        input
            .overlay
            .map(|overlay| overlay.allowed_tools.as_slice()),
    );
    let allowed_tools_compatible = input
        .overlay
        .map(|_| missing_allowed_tools.is_empty() && unexpected_allowed_tools.is_empty());
    let configured_lane = input.overlay.and_then(|overlay| overlay.lane.clone());
    let lane_matches = input
        .overlay
        .map(|_| configured_lane.as_deref() == Some(input.expected_lane));
    let configured_max_threads = input.overlay.and_then(|overlay| overlay.max_threads);
    let configured_max_depth = input.overlay.and_then(|overlay| overlay.max_depth);
    let budget_compatible = input.overlay.map(|_| {
        input.budget_present && (configured_max_threads.is_some() || configured_max_depth.is_some())
    });

    let mut checks = Vec::new();
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "configured_manifest_document",
        passed: document_present,
        detail: if document_present {
            "configured AgentCard manifest document is parsed into an overlay".to_string()
        } else {
            "no structured configured AgentCard manifest document is available".to_string()
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "overlay_source_surface",
        passed: source_surface_matches.unwrap_or(false),
        detail: match configured_source_surface_id.as_deref() {
            Some(source_surface_id) => format!(
                "configured source surface `{source_surface_id}` must match `{}`",
                input.source_surface_id
            ),
            None => "configured AgentCard manifest document has no source surface".to_string(),
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "overlay_capabilities",
        passed: capabilities_compatible.unwrap_or(false),
        detail: overlay_delta_detail(
            "capabilities",
            &missing_capabilities,
            &unexpected_capabilities,
        ),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "overlay_allowed_tools",
        passed: allowed_tools_compatible.unwrap_or(false),
        detail: overlay_delta_detail(
            "allowed tools",
            &missing_allowed_tools,
            &unexpected_allowed_tools,
        ),
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "overlay_lane",
        passed: lane_matches.unwrap_or(false),
        detail: match configured_lane.as_deref() {
            Some(lane) => format!(
                "configured lane `{lane}` must match expected `{}`",
                input.expected_lane
            ),
            None => "configured AgentCard manifest document has no lane".to_string(),
        },
    });
    checks.push(WorkGraphAdmissionShadowCheck {
        name: "overlay_budget",
        passed: budget_compatible.unwrap_or(false),
        detail: if configured_max_threads.is_some() || configured_max_depth.is_some() {
            "configured AgentCard manifest has a shadow budget boundary".to_string()
        } else {
            "configured AgentCard manifest has no shadow budget boundary".to_string()
        },
    });

    let decision = if !document_present {
        OVERLAY_MISSING_SHADOW
    } else if checks.iter().any(|check| !check.passed) {
        OVERLAY_DIFF_SHADOW
    } else {
        OVERLAY_COMPATIBLE_SHADOW
    };

    WorkGraphConfiguredManifestOverlayShadowDecision {
        source_surface_id: input.source_surface_id,
        decision,
        configured_manifest_source: input.configured_manifest_source,
        document_present,
        document_schema_version,
        source_surface_matches,
        configured_source_surface_id,
        capabilities_compatible,
        missing_capabilities,
        unexpected_capabilities,
        allowed_tools_compatible,
        missing_allowed_tools,
        unexpected_allowed_tools,
        lane_matches,
        expected_lane: input.expected_lane,
        configured_lane,
        budget_compatible,
        configured_max_threads,
        configured_max_depth,
        checks,
        feature_flag_id: "work_graph_configured_agent_card_manifest_overlay_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn overlay_delta(expected: &[String], configured: Option<&[String]>) -> (Vec<String>, Vec<String>) {
    let expected = expected.iter().cloned().collect::<BTreeSet<_>>();
    let configured = configured
        .unwrap_or_default()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    (
        expected.difference(&configured).cloned().collect(),
        configured.difference(&expected).cloned().collect(),
    )
}

fn overlay_delta_detail(kind: &str, missing: &[String], unexpected: &[String]) -> String {
    if missing.is_empty() && unexpected.is_empty() {
        return format!("configured {kind} match the default registry");
    }

    let mut details = Vec::new();
    if !missing.is_empty() {
        details.push(format!("missing {kind}: {}", missing.join(", ")));
    }
    if !unexpected.is_empty() {
        details.push(format!("unexpected {kind}: {}", unexpected.join(", ")));
    }
    details.join("; ")
}
