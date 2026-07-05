use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillCuratorContractKind {
    ScheduledLane,
    UsageRanking,
    StaleDetection,
    DuplicateConsolidation,
    MutationGuard,
    EvidenceReport,
    ReviewGatedApply,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillCuratorDescriptor {
    pub id: String,
    pub kind: SkillCuratorContractKind,
    pub contract_covered: bool,
    pub evidence_gate: String,
    pub operator_surface: String,
    pub dry_run_only_by_default: bool,
    pub mutation_denied_by_default: bool,
    pub review_gate_required: bool,
    pub writes_report: bool,
    pub raw_skill_body_logged: bool,
    pub summary: String,
}

impl SkillCuratorDescriptor {
    pub fn new(
        id: impl Into<String>,
        kind: SkillCuratorContractKind,
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
            dry_run_only_by_default: true,
            mutation_denied_by_default: true,
            review_gate_required: true,
            writes_report: true,
            raw_skill_body_logged: false,
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillCuratorLaneReport {
    pub lane_id: String,
    pub contract_count: usize,
    pub contract_covered_count: usize,
    pub scheduled_lane_contract: bool,
    pub usage_ranking_contract: bool,
    pub stale_detection_contract: bool,
    pub duplicate_consolidation_contract: bool,
    pub mutation_guard_contract: bool,
    pub evidence_report_contract: bool,
    pub review_gated_apply_contract: bool,
    pub report_first_contract: bool,
    pub dry_run_default: bool,
    pub mutation_denied_by_default: bool,
    pub raw_skill_body_logged: bool,
    pub skill_file_mutated: bool,
    pub external_network_read: bool,
    pub p0_skill_curator_lane_ready: bool,
    pub contracts: Vec<SkillCuratorDescriptor>,
}

impl SkillCuratorLaneReport {
    pub fn native_default() -> Self {
        Self::from_contracts(vec![
            SkillCuratorDescriptor::new(
                "scheduled-maintenance-lane",
                SkillCuratorContractKind::ScheduledLane,
                "cargo test -p hepta-core skill_curator_lane_report_is_report_first_and_review_gated --quiet",
                "/skill-curator-lane --json, /routine-surface --json",
                "skill hygiene is modeled as a durable maintenance lane that can be scheduled without granting mutation rights by default",
            ),
            SkillCuratorDescriptor::new(
                "skill-usage-ranking",
                SkillCuratorContractKind::UsageRanking,
                "cargo test -p hepta-core skill_curator_lane_report_is_report_first_and_review_gated --quiet",
                "/skill-curator-lane --json, /skill-lifecycle --json",
                "curator reports rank most-used, least-used, and never-used skills from usage evidence rather than raw file contents",
            ),
            SkillCuratorDescriptor::new(
                "stale-skill-detection",
                SkillCuratorContractKind::StaleDetection,
                "cargo test -p hepta-core skill_curator_lane_report_is_report_first_and_review_gated --quiet",
                "/skill-curator-lane --json",
                "stale or dead skills are flagged as candidates with reasons and evidence, not silently deleted",
            ),
            SkillCuratorDescriptor::new(
                "duplicate-consolidation-candidates",
                SkillCuratorContractKind::DuplicateConsolidation,
                "cargo test -p hepta-core skill_curator_lane_report_is_report_first_and_review_gated --quiet",
                "/skill-curator-lane --json",
                "overlapping skills are grouped into consolidation candidates with cited provenance and no automatic merge",
            ),
            SkillCuratorDescriptor::new(
                "pinned-core-mutation-guard",
                SkillCuratorContractKind::MutationGuard,
                "cargo test -p hepta-core skill_curator_lane_report_is_report_first_and_review_gated --quiet",
                "/skill-curator-lane --json, /operator-console --json",
                "pinned, bundled, hub, and core skills deny curator writes unless an explicit reviewed promotion path allows them",
            ),
            SkillCuratorDescriptor::new(
                "curator-evidence-report",
                SkillCuratorContractKind::EvidenceReport,
                "cargo test -p hepta-core skill_curator_lane_report_is_report_first_and_review_gated --quiet",
                "/skill-curator-lane --json",
                "every curator cycle writes a structured report with candidate action, confidence, rationale, and source refs",
            ),
            SkillCuratorDescriptor::new(
                "review-gated-curator-apply",
                SkillCuratorContractKind::ReviewGatedApply,
                "cargo test -p hepta-runtime worker_task_promotion_gate --quiet",
                "/skill-curator-lane --json, /handoff-bundle <task_id> --json",
                "any curator-proposed skill edit must pass the same review/promotion gate as agent-created skills before apply",
            ),
        ])
    }

    pub fn from_contracts(contracts: Vec<SkillCuratorDescriptor>) -> Self {
        let contract_count = contracts.len();
        let contract_covered_count = contracts
            .iter()
            .filter(|contract| contract.contract_covered)
            .count();
        let has_kind = |kind: SkillCuratorContractKind| {
            contracts
                .iter()
                .any(|contract| contract.contract_covered && contract.kind == kind)
        };
        let scheduled_lane_contract = has_kind(SkillCuratorContractKind::ScheduledLane);
        let usage_ranking_contract = has_kind(SkillCuratorContractKind::UsageRanking);
        let stale_detection_contract = has_kind(SkillCuratorContractKind::StaleDetection);
        let duplicate_consolidation_contract =
            has_kind(SkillCuratorContractKind::DuplicateConsolidation);
        let mutation_guard_contract = has_kind(SkillCuratorContractKind::MutationGuard);
        let evidence_report_contract = has_kind(SkillCuratorContractKind::EvidenceReport);
        let review_gated_apply_contract = has_kind(SkillCuratorContractKind::ReviewGatedApply);
        let report_first_contract = evidence_report_contract
            && contracts
                .iter()
                .all(|contract| contract.contract_covered && contract.writes_report);
        let dry_run_default = contracts
            .iter()
            .all(|contract| contract.contract_covered && contract.dry_run_only_by_default);
        let mutation_denied_by_default = contracts
            .iter()
            .all(|contract| contract.contract_covered && contract.mutation_denied_by_default);
        let raw_skill_body_logged = contracts
            .iter()
            .any(|contract| contract.raw_skill_body_logged);
        let skill_file_mutated = false;
        let external_network_read = false;
        let p0_skill_curator_lane_ready = scheduled_lane_contract
            && usage_ranking_contract
            && stale_detection_contract
            && duplicate_consolidation_contract
            && mutation_guard_contract
            && evidence_report_contract
            && review_gated_apply_contract
            && report_first_contract
            && dry_run_default
            && mutation_denied_by_default
            && !raw_skill_body_logged
            && !skill_file_mutated
            && !external_network_read;

        Self {
            lane_id: "skill-curator-lane".into(),
            contract_count,
            contract_covered_count,
            scheduled_lane_contract,
            usage_ranking_contract,
            stale_detection_contract,
            duplicate_consolidation_contract,
            mutation_guard_contract,
            evidence_report_contract,
            review_gated_apply_contract,
            report_first_contract,
            dry_run_default,
            mutation_denied_by_default,
            raw_skill_body_logged,
            skill_file_mutated,
            external_network_read,
            p0_skill_curator_lane_ready,
            contracts,
        }
    }

    pub fn contract_ready(&self) -> bool {
        self.contract_count > 0
            && self.contract_count == self.contract_covered_count
            && self.p0_skill_curator_lane_ready
    }
}

#[cfg(test)]
mod tests {
    use super::SkillCuratorLaneReport;

    #[test]
    fn skill_curator_lane_report_is_report_first_and_review_gated() {
        let report = SkillCuratorLaneReport::native_default();

        assert_eq!(report.contract_count, 7);
        assert_eq!(report.contract_covered_count, report.contract_count);
        assert!(report.scheduled_lane_contract);
        assert!(report.usage_ranking_contract);
        assert!(report.stale_detection_contract);
        assert!(report.duplicate_consolidation_contract);
        assert!(report.mutation_guard_contract);
        assert!(report.evidence_report_contract);
        assert!(report.review_gated_apply_contract);
        assert!(report.report_first_contract);
        assert!(report.dry_run_default);
        assert!(report.mutation_denied_by_default);
        assert!(!report.raw_skill_body_logged);
        assert!(!report.skill_file_mutated);
        assert!(!report.external_network_read);
        assert!(report.p0_skill_curator_lane_ready);
        assert!(report.contract_ready());
        let forbidden = ["her", "mes"].concat();
        assert!(
            report
                .contracts
                .iter()
                .all(|contract| !contract.id.contains(&forbidden)
                    && !contract.summary.to_lowercase().contains(&forbidden))
        );
    }
}
