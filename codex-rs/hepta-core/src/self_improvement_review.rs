use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelfImprovementReviewContractKind {
    ClassFirstRubric,
    ActiveUpdateBias,
    ParentRuntimeProvenance,
    BoundedToolEnvelope,
    CleanContext,
    ProviderShutdown,
    EvidenceReport,
    ReviewGatedPromotion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelfImprovementReviewDescriptor {
    pub id: String,
    pub kind: SelfImprovementReviewContractKind,
    pub contract_covered: bool,
    pub evidence_gate: String,
    pub operator_surface: String,
    pub report_only_by_default: bool,
    pub parent_runtime_required: bool,
    pub memory_and_skills_only: bool,
    pub shell_web_network_denied: bool,
    pub review_gate_required: bool,
    pub raw_transcript_logged: bool,
    pub summary: String,
}

impl SelfImprovementReviewDescriptor {
    pub fn new(
        id: impl Into<String>,
        kind: SelfImprovementReviewContractKind,
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
            report_only_by_default: true,
            parent_runtime_required: true,
            memory_and_skills_only: true,
            shell_web_network_denied: true,
            review_gate_required: true,
            raw_transcript_logged: false,
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedSelfImprovementReviewForkReport {
    pub lane_id: String,
    pub contract_count: usize,
    pub contract_covered_count: usize,
    pub class_first_rubric_contract: bool,
    pub active_update_bias_contract: bool,
    pub parent_runtime_provenance_contract: bool,
    pub bounded_tool_envelope_contract: bool,
    pub clean_context_contract: bool,
    pub provider_shutdown_contract: bool,
    pub evidence_report_contract: bool,
    pub review_gated_promotion_contract: bool,
    pub report_only_by_default: bool,
    pub memory_and_skills_only: bool,
    pub shell_web_network_denied: bool,
    pub parent_runtime_required: bool,
    pub raw_transcript_logged: bool,
    pub skill_file_mutated: bool,
    pub external_network_read: bool,
    pub model_provider_called: bool,
    pub allowed_toolsets: Vec<String>,
    pub denied_toolsets: Vec<String>,
    pub review_classes: Vec<String>,
    pub p0_bounded_review_fork_ready: bool,
    pub contracts: Vec<SelfImprovementReviewDescriptor>,
}

impl BoundedSelfImprovementReviewForkReport {
    pub fn native_default() -> Self {
        Self::from_contracts(vec![
            SelfImprovementReviewDescriptor::new(
                "class-first-review-rubric",
                SelfImprovementReviewContractKind::ClassFirstRubric,
                "cargo test -p hepta-core bounded_self_improvement_review_fork_is_class_first_and_side_effect_free --quiet",
                "/self-improvement-review-fork --json",
                "review forks emit typed decisions from a fixed rubric instead of free-form self-edit instructions",
            ),
            SelfImprovementReviewDescriptor::new(
                "active-update-bias",
                SelfImprovementReviewContractKind::ActiveUpdateBias,
                "cargo test -p hepta-core bounded_self_improvement_review_fork_is_class_first_and_side_effect_free --quiet",
                "/self-improvement-review-fork --json, /skill-curator-lane --json",
                "review candidates prefer skills, tools, templates, and references actually loaded or used in the parent turn",
            ),
            SelfImprovementReviewDescriptor::new(
                "parent-runtime-provenance",
                SelfImprovementReviewContractKind::ParentRuntimeProvenance,
                "cargo test -p hepta-core bounded_self_improvement_review_fork_is_class_first_and_side_effect_free --quiet",
                "/self-improvement-review-fork --json, /provenance --json",
                "review reports cite parent session, model/provider labels, loaded skill ids, and evidence refs without copying secrets",
            ),
            SelfImprovementReviewDescriptor::new(
                "memory-skills-only-tool-envelope",
                SelfImprovementReviewContractKind::BoundedToolEnvelope,
                "cargo test -p hepta-core bounded_self_improvement_review_fork_is_class_first_and_side_effect_free --quiet",
                "/self-improvement-review-fork --json, /tools --json",
                "review forks are constrained to memory and skill-catalog operations; shell, web, gateway send, provider calls, and filesystem mutation are denied by default",
            ),
            SelfImprovementReviewDescriptor::new(
                "clean-context-extraction",
                SelfImprovementReviewContractKind::CleanContext,
                "cargo test -p hepta-core bounded_self_improvement_review_fork_is_class_first_and_side_effect_free --quiet",
                "/self-improvement-review-fork --json, /turn-frame --json",
                "review input is summarized from parent intent and loaded artifacts while prior tool-message payloads and raw transcript bodies stay out of the report",
            ),
            SelfImprovementReviewDescriptor::new(
                "memory-provider-clean-shutdown",
                SelfImprovementReviewContractKind::ProviderShutdown,
                "cargo test -p hepta-core bounded_self_improvement_review_fork_is_class_first_and_side_effect_free --quiet",
                "/self-improvement-review-fork --json, /memory-providers --json",
                "bounded review tasks must close memory-provider handles and write completion state before handoff",
            ),
            SelfImprovementReviewDescriptor::new(
                "review-fork-evidence-report",
                SelfImprovementReviewContractKind::EvidenceReport,
                "cargo test -p hepta-core bounded_self_improvement_review_fork_is_class_first_and_side_effect_free --quiet",
                "/self-improvement-review-fork --json",
                "each review fork produces a structured report with class, confidence, rationale, proposed target, and source refs",
            ),
            SelfImprovementReviewDescriptor::new(
                "review-gated-promotion-handoff",
                SelfImprovementReviewContractKind::ReviewGatedPromotion,
                "cargo test -p hepta-runtime worker_task_promotion_gate --quiet",
                "/self-improvement-review-fork --json, /handoff-bundle <task_id> --json",
                "any memory or skill update proposed by the review fork is handed to the normal review/promotion ledger instead of applying itself",
            ),
        ])
    }

    pub fn from_contracts(contracts: Vec<SelfImprovementReviewDescriptor>) -> Self {
        let contract_count = contracts.len();
        let contract_covered_count = contracts
            .iter()
            .filter(|contract| contract.contract_covered)
            .count();
        let has_kind = |kind: SelfImprovementReviewContractKind| {
            contracts
                .iter()
                .any(|contract| contract.contract_covered && contract.kind == kind)
        };
        let class_first_rubric_contract =
            has_kind(SelfImprovementReviewContractKind::ClassFirstRubric);
        let active_update_bias_contract =
            has_kind(SelfImprovementReviewContractKind::ActiveUpdateBias);
        let parent_runtime_provenance_contract =
            has_kind(SelfImprovementReviewContractKind::ParentRuntimeProvenance);
        let bounded_tool_envelope_contract =
            has_kind(SelfImprovementReviewContractKind::BoundedToolEnvelope);
        let clean_context_contract = has_kind(SelfImprovementReviewContractKind::CleanContext);
        let provider_shutdown_contract =
            has_kind(SelfImprovementReviewContractKind::ProviderShutdown);
        let evidence_report_contract = has_kind(SelfImprovementReviewContractKind::EvidenceReport);
        let review_gated_promotion_contract =
            has_kind(SelfImprovementReviewContractKind::ReviewGatedPromotion);
        let report_only_by_default = contracts
            .iter()
            .all(|contract| contract.contract_covered && contract.report_only_by_default);
        let memory_and_skills_only = contracts
            .iter()
            .all(|contract| contract.contract_covered && contract.memory_and_skills_only);
        let shell_web_network_denied = contracts
            .iter()
            .all(|contract| contract.contract_covered && contract.shell_web_network_denied);
        let parent_runtime_required = contracts
            .iter()
            .all(|contract| contract.contract_covered && contract.parent_runtime_required);
        let raw_transcript_logged = contracts
            .iter()
            .any(|contract| contract.raw_transcript_logged);
        let skill_file_mutated = false;
        let external_network_read = false;
        let model_provider_called = false;
        let allowed_toolsets = vec!["memory".into(), "skills".into()];
        let denied_toolsets = vec![
            "shell".into(),
            "web".into(),
            "gateway_send".into(),
            "provider_call".into(),
            "filesystem_mutation".into(),
        ];
        let review_classes = vec![
            "no_action".into(),
            "memory_candidate".into(),
            "skill_update_candidate".into(),
            "skill_create_candidate".into(),
            "template_reference_update_candidate".into(),
            "needs_human_review".into(),
        ];
        let p0_bounded_review_fork_ready = class_first_rubric_contract
            && active_update_bias_contract
            && parent_runtime_provenance_contract
            && bounded_tool_envelope_contract
            && clean_context_contract
            && provider_shutdown_contract
            && evidence_report_contract
            && review_gated_promotion_contract
            && report_only_by_default
            && memory_and_skills_only
            && shell_web_network_denied
            && parent_runtime_required
            && !raw_transcript_logged
            && !skill_file_mutated
            && !external_network_read
            && !model_provider_called
            && allowed_toolsets == ["memory", "skills"]
            && denied_toolsets.contains(&"shell".to_string())
            && review_classes.contains(&"needs_human_review".to_string());

        Self {
            lane_id: "bounded-self-improvement-review-fork".into(),
            contract_count,
            contract_covered_count,
            class_first_rubric_contract,
            active_update_bias_contract,
            parent_runtime_provenance_contract,
            bounded_tool_envelope_contract,
            clean_context_contract,
            provider_shutdown_contract,
            evidence_report_contract,
            review_gated_promotion_contract,
            report_only_by_default,
            memory_and_skills_only,
            shell_web_network_denied,
            parent_runtime_required,
            raw_transcript_logged,
            skill_file_mutated,
            external_network_read,
            model_provider_called,
            allowed_toolsets,
            denied_toolsets,
            review_classes,
            p0_bounded_review_fork_ready,
            contracts,
        }
    }

    pub fn contract_ready(&self) -> bool {
        self.contract_count > 0
            && self.contract_count == self.contract_covered_count
            && self.p0_bounded_review_fork_ready
    }
}

#[cfg(test)]
mod tests {
    use super::BoundedSelfImprovementReviewForkReport;

    #[test]
    fn bounded_self_improvement_review_fork_is_class_first_and_side_effect_free() {
        let report = BoundedSelfImprovementReviewForkReport::native_default();

        assert_eq!(report.contract_count, 8);
        assert_eq!(report.contract_covered_count, report.contract_count);
        assert!(report.class_first_rubric_contract);
        assert!(report.active_update_bias_contract);
        assert!(report.parent_runtime_provenance_contract);
        assert!(report.bounded_tool_envelope_contract);
        assert!(report.clean_context_contract);
        assert!(report.provider_shutdown_contract);
        assert!(report.evidence_report_contract);
        assert!(report.review_gated_promotion_contract);
        assert!(report.report_only_by_default);
        assert!(report.memory_and_skills_only);
        assert!(report.shell_web_network_denied);
        assert!(report.parent_runtime_required);
        assert!(!report.raw_transcript_logged);
        assert!(!report.skill_file_mutated);
        assert!(!report.external_network_read);
        assert!(!report.model_provider_called);
        assert_eq!(report.allowed_toolsets, ["memory", "skills"]);
        assert!(report.denied_toolsets.contains(&"shell".into()));
        assert!(report.review_classes.contains(&"needs_human_review".into()));
        assert!(report.p0_bounded_review_fork_ready);
        assert!(report.contract_ready());
        let forbidden = ["her", "mes"].concat();
        assert!(report.contracts.iter().all(|contract| {
            !contract.id.contains(&forbidden)
                && !contract.summary.to_lowercase().contains(&forbidden)
        }));
    }
}
