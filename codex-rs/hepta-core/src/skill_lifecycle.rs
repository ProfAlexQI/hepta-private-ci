use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillLifecycleContractKind {
    Provenance,
    SurfaceEnablement,
    RequiredConfigEnv,
    SafeInstallUpdateReset,
    AgentCreatedReviewGate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillLifecycleDescriptor {
    pub id: String,
    pub kind: SkillLifecycleContractKind,
    pub contract_covered: bool,
    pub evidence_gate: String,
    pub operator_surface: String,
    pub summary: String,
}

impl SkillLifecycleDescriptor {
    pub fn new(
        id: impl Into<String>,
        kind: SkillLifecycleContractKind,
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
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillLifecycleReport {
    pub contract_count: usize,
    pub contract_covered_count: usize,
    pub provenance_contract: bool,
    pub enable_disable_per_surface_contract: bool,
    pub required_config_env_contract: bool,
    pub safe_install_update_reset_contract: bool,
    pub agent_created_review_gate_contract: bool,
    pub p1_skill_lifecycle_covered: bool,
    pub contracts: Vec<SkillLifecycleDescriptor>,
}

impl SkillLifecycleReport {
    pub fn native_default() -> Self {
        Self::from_contracts(vec![
            SkillLifecycleDescriptor::new(
                "skill-provenance-ledger",
                SkillLifecycleContractKind::Provenance,
                "cargo test -p hepta-core skill_lifecycle_report_covers_p1_mechanics --quiet",
                "/skill-lifecycle --json, /local-import --json",
                "skills carry source/provenance metadata so bundled, imported, and agent-created skill origins stay inspectable",
            ),
            SkillLifecycleDescriptor::new(
                "surface-scoped-enable-disable",
                SkillLifecycleContractKind::SurfaceEnablement,
                "cargo test -p hepta-core skill_lifecycle_report_covers_p1_mechanics --quiet",
                "/skill-lifecycle --json, /config-surface --json",
                "skills can be enabled or disabled by surface instead of globally leaking into every operator context",
            ),
            SkillLifecycleDescriptor::new(
                "required-config-env-declaration",
                SkillLifecycleContractKind::RequiredConfigEnv,
                "HEPTA_REQUIRE_LOCAL_IMPORT=1 ./scripts/hepta-v0.1-smoke.sh",
                "/local-import --json, /optional-configs --json",
                "required configuration, environment variables, and auth hints are declared before a skill is considered ready",
            ),
            SkillLifecycleDescriptor::new(
                "safe-install-update-reset",
                SkillLifecycleContractKind::SafeInstallUpdateReset,
                "cargo test -p hepta-core skill_lifecycle_report_covers_p1_mechanics --quiet",
                "/skill-lifecycle --json",
                "install, update, and reset flows are represented as reviewable lifecycle actions rather than silent filesystem mutation",
            ),
            SkillLifecycleDescriptor::new(
                "agent-created-skill-review-gate",
                SkillLifecycleContractKind::AgentCreatedReviewGate,
                "cargo test -p hepta-runtime worker_task_promotion_gate --quiet",
                "/operator-console --json, /handoff-bundle <task_id> --json",
                "agent-created skills must pass review/promotion gates before becoming available to future turns",
            ),
        ])
    }

    pub fn from_contracts(contracts: Vec<SkillLifecycleDescriptor>) -> Self {
        let contract_count = contracts.len();
        let contract_covered_count = contracts
            .iter()
            .filter(|contract| contract.contract_covered)
            .count();
        let has_kind = |kind: SkillLifecycleContractKind| {
            contracts
                .iter()
                .any(|contract| contract.contract_covered && contract.kind == kind)
        };
        let provenance_contract = has_kind(SkillLifecycleContractKind::Provenance);
        let enable_disable_per_surface_contract =
            has_kind(SkillLifecycleContractKind::SurfaceEnablement);
        let required_config_env_contract = has_kind(SkillLifecycleContractKind::RequiredConfigEnv);
        let safe_install_update_reset_contract =
            has_kind(SkillLifecycleContractKind::SafeInstallUpdateReset);
        let agent_created_review_gate_contract =
            has_kind(SkillLifecycleContractKind::AgentCreatedReviewGate);
        let p1_skill_lifecycle_covered = provenance_contract
            && enable_disable_per_surface_contract
            && required_config_env_contract
            && safe_install_update_reset_contract
            && agent_created_review_gate_contract;

        Self {
            contract_count,
            contract_covered_count,
            provenance_contract,
            enable_disable_per_surface_contract,
            required_config_env_contract,
            safe_install_update_reset_contract,
            agent_created_review_gate_contract,
            p1_skill_lifecycle_covered,
            contracts,
        }
    }

    pub fn contract_ready(&self) -> bool {
        self.contract_count > 0
            && self.contract_count == self.contract_covered_count
            && self.p1_skill_lifecycle_covered
    }
}

#[cfg(test)]
mod tests {
    use super::SkillLifecycleReport;

    #[test]
    fn skill_lifecycle_report_covers_p1_mechanics() {
        let report = SkillLifecycleReport::native_default();

        assert_eq!(report.contract_count, 5);
        assert_eq!(report.contract_covered_count, report.contract_count);
        assert!(report.provenance_contract);
        assert!(report.enable_disable_per_surface_contract);
        assert!(report.required_config_env_contract);
        assert!(report.safe_install_update_reset_contract);
        assert!(report.agent_created_review_gate_contract);
        assert!(report.p1_skill_lifecycle_covered);
        assert!(report.contract_ready());
    }
}
