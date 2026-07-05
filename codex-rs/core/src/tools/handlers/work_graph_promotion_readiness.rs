use crate::tools::handlers::work_graph_admission::WorkGraphAdmissionShadowCheck;
use crate::tools::handlers::work_graph_admission::WorkGraphRoleManifestShadowDecision;
use crate::tools::handlers::work_graph_admission::default_agent_card_manifest_registry;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

const PROMOTION_MATRIX_READY_SHADOW: &str = "promotion_matrix_ready_shadow_no_live_cutover";
const PROMOTION_MATRIX_NOT_READY_SHADOW: &str = "promotion_matrix_not_ready_shadow_no_live_cutover";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkGraphPromotionReadinessShadowMatrix {
    pub(crate) decision: &'static str,
    pub(crate) promotion_stage: &'static str,
    pub(crate) expected_source_surface_count: usize,
    pub(crate) observed_source_surface_count: usize,
    pub(crate) promotion_ready_count: usize,
    pub(crate) promotion_not_ready_count: usize,
    pub(crate) coverage_ready: bool,
    pub(crate) all_promotion_ready: bool,
    pub(crate) ready_source_surface_ids: Vec<String>,
    pub(crate) not_ready_source_surface_ids: Vec<String>,
    pub(crate) missing_source_surface_ids: Vec<String>,
    pub(crate) unexpected_source_surface_ids: Vec<String>,
    pub(crate) duplicate_source_surface_ids: Vec<String>,
    pub(crate) entries: Vec<WorkGraphPromotionReadinessShadowMatrixEntry>,
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
pub(crate) struct WorkGraphPromotionReadinessShadowMatrixEntry {
    pub(crate) source_surface_id: &'static str,
    pub(crate) manifest_id: String,
    pub(crate) role_name: Option<String>,
    pub(crate) lane: &'static str,
    pub(crate) role_manifest_decision: &'static str,
    pub(crate) configured_manifest_decision: &'static str,
    pub(crate) configured_overlay_decision: &'static str,
    pub(crate) promotion_readiness_decision: &'static str,
    pub(crate) promotion_ready: bool,
    pub(crate) configured_manifest_ready: bool,
    pub(crate) configured_overlay_ready: bool,
    pub(crate) role_contracts_ready: bool,
    pub(crate) budget_ready: bool,
    pub(crate) lane_ready: bool,
    pub(crate) attempted_tool_ready: bool,
    pub(crate) result_contract_ready: bool,
    pub(crate) verifier_reducer_ready: bool,
    pub(crate) promotion_denial_reasons: Vec<String>,
    pub(crate) role_denial_reasons: Vec<String>,
}

pub(crate) struct WorkGraphPromotionReadinessShadowMatrixInput<'a> {
    pub(crate) expected_source_surface_ids: &'a [&'static str],
    pub(crate) role_manifest_shadow_decisions: &'a [WorkGraphRoleManifestShadowDecision],
}

pub(crate) fn build_default_governed_promotion_readiness_shadow_matrix(
    role_manifest_shadow_decisions: &[WorkGraphRoleManifestShadowDecision],
) -> WorkGraphPromotionReadinessShadowMatrix {
    let expected_source_surface_ids = default_agent_card_manifest_registry()
        .entries()
        .iter()
        .map(|entry| entry.source_surface_id)
        .collect::<Vec<_>>();
    build_promotion_readiness_shadow_matrix(WorkGraphPromotionReadinessShadowMatrixInput {
        expected_source_surface_ids: expected_source_surface_ids.as_slice(),
        role_manifest_shadow_decisions,
    })
}

pub(crate) fn build_promotion_readiness_shadow_matrix(
    input: WorkGraphPromotionReadinessShadowMatrixInput<'_>,
) -> WorkGraphPromotionReadinessShadowMatrix {
    let expected_source_surface_ids = input
        .expected_source_surface_ids
        .iter()
        .map(|source_surface_id| (*source_surface_id).to_string())
        .collect::<BTreeSet<_>>();
    let observed_source_surface_counts = input.role_manifest_shadow_decisions.iter().fold(
        BTreeMap::<String, usize>::new(),
        |mut counts, decision| {
            *counts
                .entry(decision.source_surface_id.to_string())
                .or_default() += 1;
            counts
        },
    );
    let observed_source_surface_ids = observed_source_surface_counts
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();

    let missing_source_surface_ids = expected_source_surface_ids
        .difference(&observed_source_surface_ids)
        .cloned()
        .collect::<Vec<_>>();
    let unexpected_source_surface_ids = observed_source_surface_ids
        .difference(&expected_source_surface_ids)
        .cloned()
        .collect::<Vec<_>>();
    let duplicate_source_surface_ids = observed_source_surface_counts
        .iter()
        .filter(|(_source_surface_id, count)| **count > 1)
        .map(|(source_surface_id, _count)| source_surface_id.clone())
        .collect::<Vec<_>>();

    let entries = input
        .role_manifest_shadow_decisions
        .iter()
        .map(build_matrix_entry)
        .collect::<Vec<_>>();
    let ready_source_surface_ids = entries
        .iter()
        .filter(|entry| entry.promotion_ready)
        .map(|entry| entry.source_surface_id.to_string())
        .collect::<Vec<_>>();
    let not_ready_source_surface_ids = entries
        .iter()
        .filter(|entry| !entry.promotion_ready)
        .map(|entry| entry.source_surface_id.to_string())
        .collect::<Vec<_>>();
    let promotion_ready_count = ready_source_surface_ids.len();
    let promotion_not_ready_count = not_ready_source_surface_ids.len();
    let coverage_ready = missing_source_surface_ids.is_empty()
        && unexpected_source_surface_ids.is_empty()
        && duplicate_source_surface_ids.is_empty();
    let all_promotion_ready =
        !entries.is_empty() && entries.iter().all(|entry| entry.promotion_ready);
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "source_surface_coverage",
            passed: coverage_ready,
            detail: format!(
                "observed {} of {} expected governed source surfaces",
                entries.len(),
                expected_source_surface_ids.len()
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "promotion_readiness",
            passed: all_promotion_ready,
            detail: format!(
                "{promotion_ready_count} ready and {promotion_not_ready_count} not ready source surfaces"
            ),
        },
        WorkGraphAdmissionShadowCheck {
            name: "canary_disabled",
            passed: true,
            detail: "aggregate promotion readback keeps canary traffic disabled".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "live_cutover_disabled",
            passed: true,
            detail: "aggregate promotion readback does not enable live blocking or cutover"
                .to_string(),
        },
    ];
    let decision = if coverage_ready && all_promotion_ready {
        PROMOTION_MATRIX_READY_SHADOW
    } else {
        PROMOTION_MATRIX_NOT_READY_SHADOW
    };

    WorkGraphPromotionReadinessShadowMatrix {
        decision,
        promotion_stage: "shadow_only",
        expected_source_surface_count: expected_source_surface_ids.len(),
        observed_source_surface_count: entries.len(),
        promotion_ready_count,
        promotion_not_ready_count,
        coverage_ready,
        all_promotion_ready,
        ready_source_surface_ids,
        not_ready_source_surface_ids,
        missing_source_surface_ids,
        unexpected_source_surface_ids,
        duplicate_source_surface_ids,
        entries,
        checks,
        feature_flag_id: "work_graph_promotion_readiness_matrix_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn build_matrix_entry(
    decision: &WorkGraphRoleManifestShadowDecision,
) -> WorkGraphPromotionReadinessShadowMatrixEntry {
    let promotion_readiness = &decision.promotion_readiness_shadow_decision;
    let configured_manifest = &decision.configured_manifest_shadow_decision;
    let configured_overlay = &configured_manifest.configured_manifest_overlay_shadow_decision;
    let promotion_ready = promotion_readiness.denial_reasons.is_empty()
        && promotion_readiness.checks.iter().all(|check| check.passed);

    WorkGraphPromotionReadinessShadowMatrixEntry {
        source_surface_id: decision.source_surface_id,
        manifest_id: decision.manifest_id.clone(),
        role_name: decision.role_name.clone(),
        lane: decision.lane,
        role_manifest_decision: decision.decision,
        configured_manifest_decision: configured_manifest.decision,
        configured_overlay_decision: configured_overlay.decision,
        promotion_readiness_decision: promotion_readiness.decision,
        promotion_ready,
        configured_manifest_ready: promotion_readiness.configured_manifest_ready,
        configured_overlay_ready: promotion_readiness.configured_overlay_ready,
        role_contracts_ready: promotion_readiness.role_contracts_ready,
        budget_ready: promotion_readiness.budget_ready,
        lane_ready: promotion_readiness.lane_ready,
        attempted_tool_ready: promotion_readiness.attempted_tool_ready,
        result_contract_ready: promotion_readiness.result_contract_ready,
        verifier_reducer_ready: promotion_readiness.verifier_reducer_ready,
        promotion_denial_reasons: promotion_readiness.denial_reasons.clone(),
        role_denial_reasons: decision.denial_reasons.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AgentCardManifestConfig;
    use crate::tools::handlers::work_graph_admission::WorkGraphAgentCardManifest;
    use crate::tools::handlers::work_graph_admission::WorkGraphAgentCardManifestObservation;
    use crate::tools::handlers::work_graph_admission::build_agent_card_manifest_shadow_decision;
    use crate::tools::handlers::work_graph_admission::default_agent_card_manifest_registry;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    const AGENT_CARD_MANIFEST_VERSION: &str = "hepta.agent_card_manifest.v1";

    fn configured_overlay(manifest: WorkGraphAgentCardManifest) -> AgentCardManifestConfig {
        AgentCardManifestConfig {
            schema_version: Some(AGENT_CARD_MANIFEST_VERSION.to_string()),
            source_surface_id: Some(manifest.source_surface_id.to_string()),
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
            lane: Some(manifest.lane.to_string()),
            max_threads: Some(8),
            max_depth: None,
        }
    }

    fn configured_decision(
        manifest: WorkGraphAgentCardManifest,
    ) -> WorkGraphRoleManifestShadowDecision {
        build_agent_card_manifest_shadow_decision(
            manifest,
            WorkGraphAgentCardManifestObservation {
                role_name: manifest.role_name.map(str::to_string),
                role_declared: true,
                role_description_present: true,
                configured_manifest_source: Some(format!(
                    "agent-card://{}",
                    manifest.source_surface_id
                )),
                configured_manifest_version: Some(AGENT_CARD_MANIFEST_VERSION.to_string()),
                configured_manifest_overlay: Some(configured_overlay(manifest)),
                budget_present: true,
                output_contract_present: None,
                result_contract_present: None,
                verifier_present: None,
                reducer_present: None,
                attempted_tool: manifest.allowed_tools.first().copied(),
                observed_lane: Some(manifest.lane),
            },
        )
    }

    #[test]
    fn promotion_readiness_matrix_reports_all_governed_source_surfaces() {
        let registry = default_agent_card_manifest_registry();
        let expected_source_surface_ids = registry
            .entries()
            .iter()
            .map(|entry| entry.source_surface_id)
            .collect::<Vec<_>>();
        let decisions = registry
            .entries()
            .iter()
            .map(|entry| configured_decision(entry.manifest))
            .collect::<Vec<_>>();

        let matrix =
            build_promotion_readiness_shadow_matrix(WorkGraphPromotionReadinessShadowMatrixInput {
                expected_source_surface_ids: expected_source_surface_ids.as_slice(),
                role_manifest_shadow_decisions: decisions.as_slice(),
            });

        assert_eq!(
            matrix.decision,
            "promotion_matrix_not_ready_shadow_no_live_cutover"
        );
        assert_eq!(matrix.expected_source_surface_count, 8);
        assert_eq!(matrix.observed_source_surface_count, 8);
        assert_eq!(matrix.promotion_ready_count, 2);
        assert_eq!(matrix.promotion_not_ready_count, 6);
        assert!(matrix.coverage_ready);
        assert!(!matrix.all_promotion_ready);
        assert_eq!(
            matrix.ready_source_surface_ids,
            vec!["spawn_agents_on_csv", "report_agent_job_result"]
        );
        assert_eq!(
            matrix.not_ready_source_surface_ids,
            vec![
                "spawn_agent",
                "spawn_agent_v2",
                "send_message",
                "followup_task",
                "close_agent",
                "wait_agent"
            ]
        );
        assert!(matrix.missing_source_surface_ids.is_empty());
        assert!(matrix.unexpected_source_surface_ids.is_empty());
        assert!(matrix.duplicate_source_surface_ids.is_empty());
        assert!(!matrix.feature_flag_enabled);
        assert_eq!(matrix.canary_stage, "off");
        assert_eq!(matrix.canary_traffic_ppm, 0);
        assert!(!matrix.live_blocking_enabled);
        assert!(!matrix.live_cutover_enabled);

        let value = serde_json::to_value(&matrix).expect("matrix should serialize");
        assert_eq!(value["promotionStage"], json!("shadow_only"));
        assert_eq!(
            value["entries"][0]["configuredOverlayDecision"],
            json!("configured_manifest_overlay_compatible_shadow_no_live_blocking")
        );
        assert_eq!(value["liveCutoverEnabled"], json!(false));
    }

    #[test]
    fn promotion_readiness_matrix_reports_coverage_gaps() {
        let registry = default_agent_card_manifest_registry();
        let decision = configured_decision(
            registry
                .manifest_for_source("spawn_agents_on_csv")
                .expect("manifest should resolve"),
        );
        let decisions = vec![decision.clone(), decision];
        let expected_source_surface_ids = ["spawn_agents_on_csv", "wait_agent"];

        let matrix =
            build_promotion_readiness_shadow_matrix(WorkGraphPromotionReadinessShadowMatrixInput {
                expected_source_surface_ids: &expected_source_surface_ids,
                role_manifest_shadow_decisions: decisions.as_slice(),
            });

        assert_eq!(
            matrix.decision,
            "promotion_matrix_not_ready_shadow_no_live_cutover"
        );
        assert!(!matrix.coverage_ready);
        assert_eq!(matrix.missing_source_surface_ids, vec!["wait_agent"]);
        assert!(matrix.unexpected_source_surface_ids.is_empty());
        assert_eq!(
            matrix.duplicate_source_surface_ids,
            vec!["spawn_agents_on_csv"]
        );
        assert!(
            matrix
                .checks
                .iter()
                .any(|check| check.name == "source_surface_coverage" && !check.passed)
        );
        assert!(!matrix.live_cutover_enabled);
    }
}
