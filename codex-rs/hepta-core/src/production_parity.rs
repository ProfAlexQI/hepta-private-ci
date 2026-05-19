use serde::Serialize;

use crate::{
    ControlUiReport, ExternalProductionReadinessReport, ExternalProductionStatus,
    HeptaNativeAbsorptionReport, LocalConfigImportStatus,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProductionParityDimension {
    pub id: &'static str,
    pub title: &'static str,
    pub score_percent: u8,
    pub complete: bool,
    pub production_baseline: &'static str,
    pub hepta_position: &'static str,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProductionParityReport {
    pub product: &'static str,
    pub scope: &'static str,
    pub status: &'static str,
    pub overall_completion_percent: u8,
    pub dimension_count: usize,
    pub complete_dimension_count: usize,
    pub baseline_completion_percent: u8,
    pub baseline_surpass_count: usize,
    pub local_evidence_gated_ready: bool,
    pub public_ga_ready: bool,
    pub public_ga_boundary: &'static str,
    pub dimensions: Vec<ProductionParityDimension>,
    pub remaining_gaps: Vec<String>,
}

impl ProductionParityReport {
    pub fn complete(&self) -> bool {
        self.overall_completion_percent == 100
            && self.dimension_count > 0
            && self.dimension_count == self.complete_dimension_count
            && self.local_evidence_gated_ready
    }
}

pub fn production_parity_report(
    native: &HeptaNativeAbsorptionReport,
    local_import: &LocalConfigImportStatus,
    external: &ExternalProductionReadinessReport,
    control_ui: &ControlUiReport,
) -> ProductionParityReport {
    let dimensions = vec![
        native_runtime_dimension(native),
        config_catalog_dimension(local_import),
        adapter_matrix_dimension(local_import, external),
        worker_pressure_dimension(native, external),
        unattended_ops_dimension(external),
        security_ingress_dimension(external),
        control_ui_dimension(control_ui),
        release_benchmark_dimension(native, external, control_ui),
    ];
    let dimension_count = dimensions.len();
    let complete_dimension_count = dimensions.iter().filter(|item| item.complete).count();
    let overall_completion_percent = percent(
        dimensions
            .iter()
            .map(|dimension| dimension.score_percent as usize)
            .sum(),
        dimension_count * 100,
    );
    let baseline_surpass_count = dimensions
        .iter()
        .filter(|dimension| dimension.hepta_position == "ahead")
        .count();
    let local_evidence_gated_ready = overall_completion_percent == 100
        && native.native_absorption_complete()
        && local_import.local_import_complete
        && external.external_production_ready
        && control_ui.complete();
    let remaining_gaps = dimensions
        .iter()
        .filter(|dimension| !dimension.complete)
        .map(|dimension| format!("{}: {}%", dimension.id, dimension.score_percent))
        .collect::<Vec<_>>();

    ProductionParityReport {
        product: "Hepta",
        scope: "local evidence-gated production parity v1",
        status: if local_evidence_gated_ready {
            "complete"
        } else {
            "in_progress"
        },
        overall_completion_percent,
        dimension_count,
        complete_dimension_count,
        baseline_completion_percent: overall_completion_percent,
        baseline_surpass_count,
        local_evidence_gated_ready,
        public_ga_ready: false,
        public_ga_boundary: "Public GA still requires an explicit deployment decision, public ingress policy, external user acceptance, and release-channel ownership beyond this local parity gate.",
        dimensions,
        remaining_gaps,
    }
}

fn native_runtime_dimension(native: &HeptaNativeAbsorptionReport) -> ProductionParityDimension {
    let score = percent(
        native.native_absorption_coverage_percent as usize
            + native.local_executable_coverage_percent as usize,
        200,
    );
    dimension(
        "native-runtime-kernel",
        "Rust-native runtime, capability surface, and local executable harnesses",
        score,
        "mature agents expose broad runtime/tool/session surfaces",
        "ahead",
        vec![
            format!("capabilities={}", native.capability_count),
            format!("features={}", native.feature_count),
            format!(
                "native_absorption={}%, local_executable={}%",
                native.native_absorption_coverage_percent, native.local_executable_coverage_percent
            ),
        ],
    )
}

fn config_catalog_dimension(local_import: &LocalConfigImportStatus) -> ProductionParityDimension {
    let checks = [
        local_import.local_import_complete,
        local_import.config_surface_ready,
        local_import.external_interface_aligned,
        local_import.optional_config_catalog_ready,
        local_import.secret_material_local_only,
    ];
    dimension(
        "config-catalog-parity",
        "Redacted startup config, provider/model/media/channel/plugin/tool catalog parity",
        bool_score(&checks),
        "mature agents carry provider/channel/plugin registries and startup config surfaces",
        "equal",
        vec![
            format!(
                "local_import_complete={}",
                local_import.local_import_complete
            ),
            format!(
                "optional_config_catalog_ready={}",
                local_import.optional_config_catalog_ready
            ),
            local_import
                .manifest
                .as_ref()
                .and_then(|manifest| manifest.optional_config_catalog.as_ref())
                .map(|catalog| {
                    format!(
                        "catalogs={} model_providers={} models={} tools={} commands={}",
                        catalog.catalog_count,
                        catalog.model_provider_count,
                        catalog.model_catalog_model_count,
                        catalog.tool_count,
                        catalog.command_count
                    )
                })
                .unwrap_or_else(|| "catalogs=0".into()),
        ],
    )
}

fn adapter_matrix_dimension(
    local_import: &LocalConfigImportStatus,
    external: &ExternalProductionReadinessReport,
) -> ProductionParityDimension {
    let checks = [
        local_import.optional_config_catalog_ready,
        verified(external, "credentialed-provider-smoke"),
        verified(external, "channel-delivery-smoke"),
        verified(external, "device-node-smoke"),
        verified(external, "plugin-adapter-smoke"),
        verified(external, "remote-worker-harness"),
    ];
    dimension(
        "adapter-executable-matrix",
        "Provider/channel/device/plugin/worker adapter matrix backed by evidence refs",
        bool_score(&checks),
        "mature agents execute real provider, channel, plugin, and worker paths",
        "equal",
        vec![
            format!(
                "catalog_ready={}",
                local_import.optional_config_catalog_ready
            ),
            format!(
                "verified_external={}/{}",
                external.verified_count, external.requirement_count
            ),
        ],
    )
}

fn worker_pressure_dimension(
    native: &HeptaNativeAbsorptionReport,
    external: &ExternalProductionReadinessReport,
) -> ProductionParityDimension {
    let worker_native = native.capabilities.iter().any(|capability| {
        capability.id == "skills-automation-agents" && capability.local_executable
    });
    let checks = [
        worker_native,
        verified(external, "remote-worker-harness"),
        verified(external, "concurrency-resource-governance"),
        verified(external, "failure-recovery-drill"),
        verified(external, "multi-user-permission-smoke"),
    ];
    dimension(
        "worker-subagent-pressure",
        "Worker/subagent lifecycle, per-worker permission envelopes, hash-chained evidence ledgers, deterministic replay audit, merge confidence/risk scoring, explicit promotion/auto-merge gating, promotion approval trails, signed handoff bundle export, HTTP/API and Control UI handoff surface, dependency gating, batch scheduling, worker-pool pressure limits, timeout/retry budget policy, autonomous loop traces, isolation gates, structured patch review, conflict-driven revise/retry, patch-set batch apply, safe apply, rollback, and conflict handling",
        bool_score(&checks),
        "mature agents support subagents, queue supervision, and isolated execution",
        "equal",
        vec![
            format!("native_worker_lane={worker_native}"),
            "spawn/run/due/ready/cancel/supervisor/join surfaces present".into(),
            "worker-pool pressure reports, permission envelopes, hash-chained evidence ledgers, deterministic replay audit checks, merge confidence/risk scores, explicit promotion/auto-merge gates, promotion approval trails, signed handoff bundles, HTTP/API and Control UI handoff surfaces, timeout/retry budgets, autonomous loop traces, artifacts, diff summaries, structured patch proposals, conflict-driven revisions, patch-set batch apply, safe apply transactions, rollback, conflict status, and apply/reject review gates are produced for join review".into(),
        ],
    )
}

fn unattended_ops_dimension(
    external: &ExternalProductionReadinessReport,
) -> ProductionParityDimension {
    let checks = [
        verified(external, "unattended-scheduler-daemon"),
        verified(external, "long-production-soak"),
        verified(external, "incident-observability"),
        verified(external, "failure-recovery-drill"),
        verified(external, "backup-restore-drill"),
        verified(external, "production-docs-runbook"),
    ];
    dimension(
        "unattended-ops-soak",
        "Scheduler daemon, long soak, observability, recovery and runbook handoff",
        bool_score(&checks),
        "mature agents survive unattended operation with observable recovery",
        "equal",
        vec![format!(
            "external_verification={}%, blocked={}",
            external.external_verification_percent, external.blocked_count
        )],
    )
}

fn security_ingress_dimension(
    external: &ExternalProductionReadinessReport,
) -> ProductionParityDimension {
    let checks = [
        verified(external, "secret-rotation-drill"),
        verified(external, "external-policy-audit"),
        verified(external, "multi-user-permission-smoke"),
        verified(external, "public-network-ingress"),
    ];
    dimension(
        "security-rbac-ingress",
        "Secret rotation, policy audit, session/user isolation, and ingress posture evidence",
        bool_score(&checks),
        "mature agents must separate users, secrets, approvals, and exposed network surfaces",
        "ahead",
        vec![
            "redacted evidence ledger; raw secrets rejected by recorder".into(),
            "public ingress not opened by default".into(),
        ],
    )
}

fn control_ui_dimension(control_ui: &ControlUiReport) -> ProductionParityDimension {
    let checks = [
        control_ui.complete(),
        control_ui.screen_coverage_percent == 100,
        control_ui.asset_coverage_percent == 100,
        control_ui.command_binding_count >= 15,
    ];
    dimension(
        "operator-ui-api",
        "Local Control UI, JSON API surfaces, static fallback and command bindings",
        bool_score(&checks),
        "mature agents expose operator dashboards and stable machine-readable status",
        "ahead",
        vec![
            format!(
                "screens={}/{}",
                control_ui.implemented_screen_count, control_ui.screen_count
            ),
            format!("bindings={}", control_ui.command_binding_count),
        ],
    )
}

fn release_benchmark_dimension(
    native: &HeptaNativeAbsorptionReport,
    external: &ExternalProductionReadinessReport,
    control_ui: &ControlUiReport,
) -> ProductionParityDimension {
    let checks = [
        native.native_absorption_complete(),
        control_ui.complete(),
        verified(external, "release-channel-governance"),
        verified(external, "production-docs-runbook"),
        verified(external, "backup-restore-drill"),
    ];
    dimension(
        "release-benchmark-governance",
        "Deterministic benchmark/report surface, preflight package, rollback and release governance",
        bool_score(&checks),
        "mature agents need repeatable release gates and regression evidence",
        "equal",
        vec![
            "production parity report is deterministic JSON/text".into(),
            "preflight and backup/restore gates are evidence-backed".into(),
        ],
    )
}

fn dimension(
    id: &'static str,
    title: &'static str,
    score_percent: u8,
    production_baseline: &'static str,
    hepta_position: &'static str,
    evidence: Vec<String>,
) -> ProductionParityDimension {
    ProductionParityDimension {
        id,
        title,
        score_percent,
        complete: score_percent == 100,
        production_baseline,
        hepta_position,
        evidence,
    }
}

fn verified(external: &ExternalProductionReadinessReport, id: &str) -> bool {
    external
        .requirements
        .iter()
        .any(|item| item.id == id && item.status == ExternalProductionStatus::Verified)
}

fn bool_score(checks: &[bool]) -> u8 {
    percent(checks.iter().filter(|item| **item).count(), checks.len())
}

fn percent(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        return 0;
    }
    ((numerator * 100) / denominator) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ExternalProductionEvidence, ExternalProductionEvidenceManifest, control_ui_report,
        hepta_native_absorption_report,
    };
    use std::collections::BTreeMap;
    use std::fs;

    #[test]
    fn production_parity_reaches_complete_when_all_runtime_evidence_is_present() {
        let tmp = std::env::temp_dir().join(format!(
            "hepta-production-parity-{}.json",
            std::process::id()
        ));
        let evidence = crate::external_production_requirement_registry()
            .into_iter()
            .map(|requirement| {
                (
                    requirement.id.to_string(),
                    ExternalProductionEvidence {
                        verified: true,
                        observed_at_unix_ms: Some(1),
                        evidence_ref: Some(format!("test:{}", requirement.id)),
                        notes: Some("unit-test redacted evidence".into()),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        let manifest = ExternalProductionEvidenceManifest {
            schema_version: 1,
            evidence,
        };
        fs::write(&tmp, serde_json::to_string(&manifest).unwrap()).unwrap();

        let mut local = LocalConfigImportStatus::missing(
            ".hepta/local-import",
            ".hepta/local-import/manifest.json",
        );
        local.local_import_complete = true;
        local.config_surface_ready = true;
        local.external_interface_aligned = true;
        local.optional_config_catalog_ready = true;
        local.secret_material_local_only = true;

        let external = crate::external_production_readiness_report(&tmp);
        let report = production_parity_report(
            &hepta_native_absorption_report(),
            &local,
            &external,
            &control_ui_report(),
        );

        assert_eq!(report.overall_completion_percent, 100);
        assert_eq!(report.complete_dimension_count, report.dimension_count);
        assert!(report.local_evidence_gated_ready);
        assert!(report.complete());
        assert!(!report.public_ga_ready);

        let _ = fs::remove_file(tmp);
    }
}
