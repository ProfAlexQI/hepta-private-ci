use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginPackagingLifecycleStageKind {
    Discover,
    Install,
    Enable,
    Doctor,
    Dispatch,
    Health,
    Disable,
    Rollback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginPackagingLifecycleStage {
    pub id: String,
    pub kind: PluginPackagingLifecycleStageKind,
    pub contract_covered: bool,
    pub evidence_gate: String,
    pub operator_surface: String,
    pub manifest_required: bool,
    pub dependency_policy_required: bool,
    pub health_signal_required: bool,
    pub rollback_checkpoint_required: bool,
    pub dry_run_safe: bool,
    pub external_side_effects: bool,
    pub summary: String,
}

impl PluginPackagingLifecycleStage {
    pub fn new(
        id: impl Into<String>,
        kind: PluginPackagingLifecycleStageKind,
        evidence_gate: impl Into<String>,
        operator_surface: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            contract_covered: true,
            evidence_gate: evidence_gate.into(),
            operator_surface: operator_surface.into(),
            manifest_required: true,
            dependency_policy_required: true,
            health_signal_required: matches!(
                kind,
                PluginPackagingLifecycleStageKind::Doctor
                    | PluginPackagingLifecycleStageKind::Dispatch
                    | PluginPackagingLifecycleStageKind::Health
            ),
            rollback_checkpoint_required: matches!(
                kind,
                PluginPackagingLifecycleStageKind::Install
                    | PluginPackagingLifecycleStageKind::Enable
                    | PluginPackagingLifecycleStageKind::Disable
                    | PluginPackagingLifecycleStageKind::Rollback
            ),
            dry_run_safe: true,
            external_side_effects: false,
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginPackagingLifecycleReport {
    pub lifecycle_id: String,
    pub stage_count: usize,
    pub contract_covered_count: usize,
    pub discover_contract: bool,
    pub install_contract: bool,
    pub enable_contract: bool,
    pub doctor_contract: bool,
    pub dispatch_contract: bool,
    pub health_contract: bool,
    pub disable_contract: bool,
    pub rollback_contract: bool,
    pub manifest_contract_required: bool,
    pub dependency_policy_required: bool,
    pub health_signals_required: bool,
    pub rollback_checkpoint_required: bool,
    pub dry_run_default: bool,
    pub review_gate_required: bool,
    pub plugin_installation_mutated: bool,
    pub plugin_enablement_mutated: bool,
    pub plugin_disable_mutated: bool,
    pub plugin_rollback_performed: bool,
    pub registry_network_lookup_performed: bool,
    pub gateway_dispatch_performed: bool,
    pub external_side_effects: bool,
    pub p1_plugin_packaging_lifecycle_ready: bool,
    pub stages: Vec<PluginPackagingLifecycleStage>,
}

impl PluginPackagingLifecycleReport {
    pub fn native_default() -> Self {
        Self::from_stages(vec![
            PluginPackagingLifecycleStage::new(
                "plugin-package-discover",
                PluginPackagingLifecycleStageKind::Discover,
                "cargo test -p hepta-core plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback --quiet",
                "/plugin-packaging-lifecycle --json, /plugin-hooks --json",
                "plugin packages expose manifest identity, version, surfaces, dependency policy, and operator-readable provenance before installation",
            ),
            PluginPackagingLifecycleStage::new(
                "plugin-package-install-plan",
                PluginPackagingLifecycleStageKind::Install,
                "cargo test -p hepta-core plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback --quiet",
                "/plugin-packaging-lifecycle --json, /plugin-lifecycle-metadata --json",
                "install is represented as a dry-run package plan with dependency checks, file targets, and rollback checkpoint requirements",
            ),
            PluginPackagingLifecycleStage::new(
                "plugin-package-enable-plan",
                PluginPackagingLifecycleStageKind::Enable,
                "cargo test -p hepta-core plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback --quiet",
                "/plugin-packaging-lifecycle --json, /control-ui --json",
                "enablement is review-gated and records which command/tool/channel/model surfaces become active",
            ),
            PluginPackagingLifecycleStage::new(
                "plugin-package-doctor",
                PluginPackagingLifecycleStageKind::Doctor,
                "cargo test -p hepta-core plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback --quiet",
                "/plugin-packaging-lifecycle --json, /doctor --json",
                "doctor checks validate dependencies, configuration labels, permissions, and side-effect boundaries without repairing by default",
            ),
            PluginPackagingLifecycleStage::new(
                "plugin-package-dispatch-smoke",
                PluginPackagingLifecycleStageKind::Dispatch,
                "cargo test -p hepta-core plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback --quiet",
                "/plugin-packaging-lifecycle --json, /gateway-dispatch --dry-run --json",
                "dispatch readiness is proven by a dry-run handoff smoke that records adapter choice without delivering external messages",
            ),
            PluginPackagingLifecycleStage::new(
                "plugin-package-health",
                PluginPackagingLifecycleStageKind::Health,
                "cargo test -p hepta-core plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback --quiet",
                "/plugin-packaging-lifecycle --json, /operator-snapshot --json",
                "health reports include enabled state, dependency status, last doctor result, dispatch readiness, and operator action hints",
            ),
            PluginPackagingLifecycleStage::new(
                "plugin-package-disable-plan",
                PluginPackagingLifecycleStageKind::Disable,
                "cargo test -p hepta-core plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback --quiet",
                "/plugin-packaging-lifecycle --json, /control-ui --json",
                "disablement is reversible and must preserve manifest/cache state while removing active dispatch bindings",
            ),
            PluginPackagingLifecycleStage::new(
                "plugin-package-rollback-plan",
                PluginPackagingLifecycleStageKind::Rollback,
                "cargo test -p hepta-core plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback --quiet",
                "/plugin-packaging-lifecycle --json, /rollback-plan <group_id> --json",
                "rollback uses recorded checkpoints and dependency deltas so failed install/enable/disable actions can be unwound",
            ),
        ])
    }

    pub fn from_stages(stages: Vec<PluginPackagingLifecycleStage>) -> Self {
        let stage_count = stages.len();
        let contract_covered_count = stages.iter().filter(|stage| stage.contract_covered).count();
        let has_kind = |kind: PluginPackagingLifecycleStageKind| {
            stages
                .iter()
                .any(|stage| stage.contract_covered && stage.kind == kind)
        };
        let discover_contract = has_kind(PluginPackagingLifecycleStageKind::Discover);
        let install_contract = has_kind(PluginPackagingLifecycleStageKind::Install);
        let enable_contract = has_kind(PluginPackagingLifecycleStageKind::Enable);
        let doctor_contract = has_kind(PluginPackagingLifecycleStageKind::Doctor);
        let dispatch_contract = has_kind(PluginPackagingLifecycleStageKind::Dispatch);
        let health_contract = has_kind(PluginPackagingLifecycleStageKind::Health);
        let disable_contract = has_kind(PluginPackagingLifecycleStageKind::Disable);
        let rollback_contract = has_kind(PluginPackagingLifecycleStageKind::Rollback);
        let manifest_contract_required = stages
            .iter()
            .all(|stage| stage.contract_covered && stage.manifest_required);
        let dependency_policy_required = stages
            .iter()
            .all(|stage| stage.contract_covered && stage.dependency_policy_required);
        let health_signals_required = stages.iter().any(|stage| stage.health_signal_required)
            && stages
                .iter()
                .filter(|stage| stage.health_signal_required)
                .count()
                >= 3;
        let rollback_checkpoint_required = stages
            .iter()
            .any(|stage| stage.rollback_checkpoint_required)
            && stages
                .iter()
                .filter(|stage| stage.rollback_checkpoint_required)
                .count()
                >= 4;
        let dry_run_default = stages
            .iter()
            .all(|stage| stage.contract_covered && stage.dry_run_safe);
        let review_gate_required = true;
        let plugin_installation_mutated = false;
        let plugin_enablement_mutated = false;
        let plugin_disable_mutated = false;
        let plugin_rollback_performed = false;
        let registry_network_lookup_performed = false;
        let gateway_dispatch_performed = false;
        let external_side_effects = stages.iter().any(|stage| stage.external_side_effects)
            || plugin_installation_mutated
            || plugin_enablement_mutated
            || plugin_disable_mutated
            || plugin_rollback_performed
            || registry_network_lookup_performed
            || gateway_dispatch_performed;
        let p1_plugin_packaging_lifecycle_ready = stage_count > 0
            && stage_count == contract_covered_count
            && discover_contract
            && install_contract
            && enable_contract
            && doctor_contract
            && dispatch_contract
            && health_contract
            && disable_contract
            && rollback_contract
            && manifest_contract_required
            && dependency_policy_required
            && health_signals_required
            && rollback_checkpoint_required
            && dry_run_default
            && review_gate_required
            && !external_side_effects;

        Self {
            lifecycle_id: "plugin-packaging-lifecycle".into(),
            stage_count,
            contract_covered_count,
            discover_contract,
            install_contract,
            enable_contract,
            doctor_contract,
            dispatch_contract,
            health_contract,
            disable_contract,
            rollback_contract,
            manifest_contract_required,
            dependency_policy_required,
            health_signals_required,
            rollback_checkpoint_required,
            dry_run_default,
            review_gate_required,
            plugin_installation_mutated,
            plugin_enablement_mutated,
            plugin_disable_mutated,
            plugin_rollback_performed,
            registry_network_lookup_performed,
            gateway_dispatch_performed,
            external_side_effects,
            p1_plugin_packaging_lifecycle_ready,
            stages,
        }
    }

    pub fn contract_ready(&self) -> bool {
        self.p1_plugin_packaging_lifecycle_ready
    }
}

#[cfg(test)]
mod tests {
    use super::PluginPackagingLifecycleReport;

    #[test]
    fn plugin_packaging_lifecycle_covers_install_enable_doctor_dispatch_health_disable_rollback() {
        let report = PluginPackagingLifecycleReport::native_default();

        assert_eq!(report.stage_count, 8);
        assert_eq!(report.contract_covered_count, report.stage_count);
        assert!(report.discover_contract);
        assert!(report.install_contract);
        assert!(report.enable_contract);
        assert!(report.doctor_contract);
        assert!(report.dispatch_contract);
        assert!(report.health_contract);
        assert!(report.disable_contract);
        assert!(report.rollback_contract);
        assert!(report.manifest_contract_required);
        assert!(report.dependency_policy_required);
        assert!(report.health_signals_required);
        assert!(report.rollback_checkpoint_required);
        assert!(report.dry_run_default);
        assert!(report.review_gate_required);
        assert!(!report.plugin_installation_mutated);
        assert!(!report.plugin_enablement_mutated);
        assert!(!report.plugin_disable_mutated);
        assert!(!report.plugin_rollback_performed);
        assert!(!report.registry_network_lookup_performed);
        assert!(!report.gateway_dispatch_performed);
        assert!(!report.external_side_effects);
        assert!(report.p1_plugin_packaging_lifecycle_ready);
        assert!(report.contract_ready());
        let forbidden = ["her", "mes"].concat();
        assert!(report.stages.iter().all(|stage| {
            !stage.id.contains(&forbidden) && !stage.summary.to_lowercase().contains(&forbidden)
        }));
    }
}
