//! Fail-closed continual-learning primitives for the NDU shadow controller.
//!
//! The implementation deliberately owns no preference, approval, tool,
//! production-write, plugin, skill, adapter-publication, release, or base-model
//! authority. It produces replayable observations, bounded recommendations,
//! and proposal plans for authorities outside this module to evaluate.

use hepta_contracts::ContentHash;
use hepta_contracts::Revision;
use sha2::Digest;
use sha2::Sha256;

mod contracts;
mod h1_shadow;
mod h1_shadow_journal;

pub use contracts::BoundedUtilityScore;
pub use contracts::HardFeasibilityMask;
pub use contracts::HardFeasibilityVerdict;
pub use contracts::NduDatasetManifestRef;
pub use contracts::NduUtilityEventRef;
pub use contracts::NduUtilityTransitionRef;
pub use contracts::UtilityVector;
pub use h1_shadow::NduH1ShadowConfig;
pub use h1_shadow::NduH1ShadowController;
pub use h1_shadow::NduH1ShadowError;
pub use h1_shadow::NduH1ShadowReceipt;
pub use h1_shadow::NduH1ShadowRequest;
pub use h1_shadow_journal::NduH1ArmEvaluation;
pub use h1_shadow_journal::NduH1EvaluationSummary;
pub use h1_shadow_journal::NduH1Journal;
pub use h1_shadow_journal::NduH1JournalError;
pub use h1_shadow_journal::NduH1ShadowService;
pub use h1_shadow_journal::NduH1ShadowServiceError;
pub use h1_shadow_journal::NduH1ShadowServiceResult;

const SCORE_LIMIT: i32 = 10_000;

/// Baseline family evaluated alongside NDU in paired replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NduBaselineKind {
    /// Existing deterministic Hepta allocation heuristic.
    CurrentHeuristic,
    /// Simple contextual bandit baseline.
    ContextualBandit,
    /// Small frozen recurrent or feed-forward baseline.
    FrozenGruMlp,
    /// Candidate NDU shadow scorer.
    NduShadow,
}

/// Authority surface permanently excluded from NDU control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NduProhibitedAuthority {
    /// Authenticated explicit preference commits.
    PreferenceCommit,
    /// Approval and permission decisions.
    ApprovalOrPermission,
    /// Production writes and external tool effects.
    ProductionEffect,
    /// Verifier configuration and hidden tests.
    VerifierOrHiddenTest,
    /// Plugin installation and skill application.
    PluginOrSkillApplication,
    /// Adapter publication.
    AdapterPublication,
    /// Safety policy and release gates.
    SafetyOrReleaseGate,
    /// Base-model weight mutation.
    BaseModelWeights,
}

/// H0 threat-model contract for the NDU control boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduThreatModel {
    prohibited_authorities: [NduProhibitedAuthority; 8],
    tenant_isolation_required: bool,
    consent_and_revocation_required: bool,
    replay_protection_required: bool,
    feedback_poisoning_evaluation_required: bool,
    independent_verifier_required: bool,
}

impl NduThreatModel {
    /// Returns the canonical fail-closed threat model.
    pub const fn canonical() -> Self {
        Self {
            prohibited_authorities: [
                NduProhibitedAuthority::PreferenceCommit,
                NduProhibitedAuthority::ApprovalOrPermission,
                NduProhibitedAuthority::ProductionEffect,
                NduProhibitedAuthority::VerifierOrHiddenTest,
                NduProhibitedAuthority::PluginOrSkillApplication,
                NduProhibitedAuthority::AdapterPublication,
                NduProhibitedAuthority::SafetyOrReleaseGate,
                NduProhibitedAuthority::BaseModelWeights,
            ],
            tenant_isolation_required: true,
            consent_and_revocation_required: true,
            replay_protection_required: true,
            feedback_poisoning_evaluation_required: true,
            independent_verifier_required: true,
        }
    }

    /// Returns whether one authority is permanently prohibited.
    pub fn prohibits(&self, authority: NduProhibitedAuthority) -> bool {
        self.prohibited_authorities.contains(&authority)
    }

    /// Returns whether every mandatory threat control is enabled.
    pub const fn is_complete(&self) -> bool {
        self.tenant_isolation_required
            && self.consent_and_revocation_required
            && self.replay_protection_required
            && self.feedback_poisoning_evaluation_required
            && self.independent_verifier_required
    }
}

impl NduBaselineKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentHeuristic => "current_heuristic",
            Self::ContextualBandit => "contextual_bandit",
            Self::FrozenGruMlp => "frozen_gru_mlp",
            Self::NduShadow => "ndu_shadow",
        }
    }
}

/// Frozen execution boundary for H1 shadow observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduShadowBoundary {
    llm_frozen: bool,
    skills_frozen: bool,
    adapters_frozen: bool,
    production_writes_disabled: bool,
    tool_effects_disabled: bool,
    preference_commits_disabled: bool,
}

impl NduShadowBoundary {
    /// Returns the required H1 boundary with every mutation surface disabled.
    pub const fn required() -> Self {
        Self {
            llm_frozen: true,
            skills_frozen: true,
            adapters_frozen: true,
            production_writes_disabled: true,
            tool_effects_disabled: true,
            preference_commits_disabled: true,
        }
    }

    /// Returns whether the boundary is safe for shadow evaluation.
    pub const fn is_shadow_only(&self) -> bool {
        self.llm_frozen
            && self.skills_frozen
            && self.adapters_frozen
            && self.production_writes_disabled
            && self.tool_effects_disabled
            && self.preference_commits_disabled
    }
}

/// Normalized offline observation consumed by all paired replay arms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduShadowObservation {
    event: NduUtilityEventRef,
    task_signal_basis_points: i32,
    learning_signal_basis_points: i32,
    trust_signal_basis_points: i32,
    memory_pollution_risk_basis_points: i32,
    resource_cost_basis_points: i32,
    uncertainty_basis_points: i32,
    propensity_basis_points: u16,
    delayed_outcome_hash: Option<ContentHash>,
}

impl NduShadowObservation {
    /// Creates one normalized shadow observation.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event: NduUtilityEventRef,
        task_signal_basis_points: i32,
        learning_signal_basis_points: i32,
        trust_signal_basis_points: i32,
        memory_pollution_risk_basis_points: i32,
        resource_cost_basis_points: i32,
        uncertainty_basis_points: i32,
        propensity_basis_points: u16,
        delayed_outcome_hash: Option<ContentHash>,
    ) -> Self {
        Self {
            event,
            task_signal_basis_points,
            learning_signal_basis_points,
            trust_signal_basis_points,
            memory_pollution_risk_basis_points,
            resource_cost_basis_points,
            uncertainty_basis_points,
            propensity_basis_points,
            delayed_outcome_hash,
        }
    }

    /// Returns the immutable event reference.
    pub fn event(&self) -> &NduUtilityEventRef {
        &self.event
    }
}

/// Deterministic H1 paired-replay result for one baseline arm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduShadowArmResult {
    baseline: NduBaselineKind,
    transition: NduUtilityTransitionRef,
    replay_receipt_hash: ContentHash,
}

/// Returns whether an NDU result Pareto-dominates a simple baseline.
///
/// Safety is deliberately absent because both results must first pass their
/// independent hard feasibility masks. No utility gain can compensate for a
/// failed or unknown hard constraint.
pub fn ndu_pareto_dominates(
    ndu: &NduUtilityTransitionRef,
    baseline: &NduUtilityTransitionRef,
) -> bool {
    if !ndu.feasibility().permits_optimization() || !baseline.feasibility().permits_optimization() {
        return false;
    }
    let ndu = ndu.utility();
    let baseline = baseline.utility();
    let non_worse = ndu.task_value() >= baseline.task_value()
        && ndu.learning_value() >= baseline.learning_value()
        && ndu.trust() >= baseline.trust()
        && ndu.memory_pollution_risk() <= baseline.memory_pollution_risk()
        && ndu.resource_cost() <= baseline.resource_cost()
        && ndu.uncertainty() <= baseline.uncertainty();
    let strictly_better = ndu.task_value() > baseline.task_value()
        || ndu.learning_value() > baseline.learning_value()
        || ndu.trust() > baseline.trust()
        || ndu.memory_pollution_risk() < baseline.memory_pollution_risk()
        || ndu.resource_cost() < baseline.resource_cost()
        || ndu.uncertainty() < baseline.uncertainty();
    non_worse && strictly_better
}

impl NduShadowArmResult {
    /// Returns the evaluated arm.
    pub const fn baseline(&self) -> NduBaselineKind {
        self.baseline
    }

    /// Returns the replayable transition.
    pub fn transition(&self) -> &NduUtilityTransitionRef {
        &self.transition
    }

    /// Returns the deterministic replay receipt digest.
    pub fn replay_receipt_hash(&self) -> &ContentHash {
        &self.replay_receipt_hash
    }
}

/// Runs one deterministic read-only shadow observation.
#[allow(clippy::too_many_arguments)]
pub fn evaluate_ndu_shadow_arm(
    boundary: &NduShadowBoundary,
    baseline: NduBaselineKind,
    revision: Revision,
    previous_state_hash: ContentHash,
    observation: NduShadowObservation,
    model_hash: ContentHash,
    config_hash: ContentHash,
    feasibility: HardFeasibilityMask,
) -> Option<NduShadowArmResult> {
    if !boundary.is_shadow_only() {
        return None;
    }

    let utility = utility_for_baseline(baseline, &observation)?;
    let next_state_hash = stable_hash(&[
        "hepta_ndu_state_v1",
        previous_state_hash.as_str(),
        observation.event.event_hash().as_str(),
        model_hash.as_str(),
        config_hash.as_str(),
        baseline.as_str(),
        &revision.get().to_string(),
        &utility.task_value().basis_points().to_string(),
        &utility.learning_value().basis_points().to_string(),
        &utility.trust().basis_points().to_string(),
        &utility.memory_pollution_risk().basis_points().to_string(),
        &utility.resource_cost().basis_points().to_string(),
        &utility.uncertainty().basis_points().to_string(),
    ]);
    let transition = NduUtilityTransitionRef::new(
        revision,
        previous_state_hash,
        observation.event,
        model_hash,
        config_hash,
        next_state_hash.clone(),
        utility,
        feasibility,
    );
    let replay_receipt_hash = stable_hash(&[
        "hepta_ndu_paired_replay_receipt_v1",
        baseline.as_str(),
        next_state_hash.as_str(),
        &utility.task_value().basis_points().to_string(),
        &utility.learning_value().basis_points().to_string(),
        &utility.trust().basis_points().to_string(),
        &utility.memory_pollution_risk().basis_points().to_string(),
        &utility.resource_cost().basis_points().to_string(),
        &utility.uncertainty().basis_points().to_string(),
        feasibility_verdict(feasibility.safety()),
        feasibility_verdict(feasibility.permission()),
        feasibility_verdict(feasibility.budget()),
        feasibility_verdict(feasibility.correctability()),
        &observation.propensity_basis_points.to_string(),
        observation
            .delayed_outcome_hash
            .as_ref()
            .map_or("none", ContentHash::as_str),
    ]);

    Some(NduShadowArmResult {
        baseline,
        transition,
        replay_receipt_hash,
    })
}

pub(crate) const fn feasibility_verdict(verdict: HardFeasibilityVerdict) -> &'static str {
    match verdict {
        HardFeasibilityVerdict::Satisfied => "satisfied",
        HardFeasibilityVerdict::Violated => "violated",
        HardFeasibilityVerdict::Unknown => "unknown",
    }
}

/// Reason a bounded H2 canary abstained instead of recommending a budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NduCanaryAbstentionReason {
    /// A hard feasibility constraint was violated or unknown.
    HardFeasibilityBlocked,
    /// Model confidence was below the configured floor.
    LowConfidence,
    /// Observed drift exceeded the configured ceiling.
    DriftDetected,
    /// The requested budget delta exceeded its absolute bound.
    BudgetOutOfBounds,
}

/// H2 output limited to memory and context budget recommendations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NduBudgetCanaryDecision {
    /// Recommends bounded deltas without applying them.
    Recommend {
        /// Signed memory budget delta.
        memory_budget_delta: i32,
        /// Signed context budget delta.
        context_budget_delta: i32,
        /// Confidence in basis points.
        confidence_basis_points: u16,
        /// Receipt binding the recommendation.
        receipt_hash: ContentHash,
    },
    /// Fails closed and recommends no change.
    Abstain(NduCanaryAbstentionReason),
}

/// Evaluates one bounded H2 canary recommendation.
#[allow(clippy::too_many_arguments)]
pub fn evaluate_ndu_budget_canary(
    transition: &NduUtilityTransitionRef,
    memory_budget_delta: i32,
    context_budget_delta: i32,
    max_absolute_delta: u32,
    confidence_basis_points: u16,
    minimum_confidence_basis_points: u16,
    drift_basis_points: u16,
    maximum_drift_basis_points: u16,
) -> NduBudgetCanaryDecision {
    if !transition.feasibility().permits_optimization() {
        return NduBudgetCanaryDecision::Abstain(NduCanaryAbstentionReason::HardFeasibilityBlocked);
    }
    if confidence_basis_points < minimum_confidence_basis_points {
        return NduBudgetCanaryDecision::Abstain(NduCanaryAbstentionReason::LowConfidence);
    }
    if drift_basis_points > maximum_drift_basis_points {
        return NduBudgetCanaryDecision::Abstain(NduCanaryAbstentionReason::DriftDetected);
    }
    let max_absolute_delta = i64::from(max_absolute_delta);
    if i64::from(memory_budget_delta).abs() > max_absolute_delta
        || i64::from(context_budget_delta).abs() > max_absolute_delta
    {
        return NduBudgetCanaryDecision::Abstain(NduCanaryAbstentionReason::BudgetOutOfBounds);
    }

    let receipt_hash = stable_hash(&[
        "hepta_ndu_budget_canary_v1",
        transition.next_state_hash().as_str(),
        &memory_budget_delta.to_string(),
        &context_budget_delta.to_string(),
        &confidence_basis_points.to_string(),
        &drift_basis_points.to_string(),
    ]);
    NduBudgetCanaryDecision::Recommend {
        memory_budget_delta,
        context_budget_delta,
        confidence_basis_points,
        receipt_hash,
    }
}

/// Kind of H3 proposal NDU may draft but never apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NduProposalKind {
    /// A proposed governed skill change.
    Skill,
    /// A proposed governed workflow change.
    Workflow,
}

/// Fail-closed H3 proposal plan for external governance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduGovernedProposalPlan {
    kind: NduProposalKind,
    proposal_hash: ContentHash,
    journal_required: bool,
    sandbox_required: bool,
    hidden_tests_required: bool,
    human_approval_required: bool,
    apply_authority_granted: bool,
}

impl NduGovernedProposalPlan {
    /// Creates a proposal-only plan with no apply authority.
    pub const fn proposal_only(kind: NduProposalKind, proposal_hash: ContentHash) -> Self {
        Self {
            kind,
            proposal_hash,
            journal_required: true,
            sandbox_required: true,
            hidden_tests_required: true,
            human_approval_required: true,
            apply_authority_granted: false,
        }
    }

    /// Returns whether the plan remains non-applying and fully governed.
    pub const fn is_fail_closed(&self) -> bool {
        self.journal_required
            && self.sandbox_required
            && self.hidden_tests_required
            && self.human_approval_required
            && !self.apply_authority_granted
    }

    /// Returns the proposal kind.
    pub const fn kind(&self) -> NduProposalKind {
        self.kind
    }

    /// Returns the immutable proposal digest.
    pub fn proposal_hash(&self) -> &ContentHash {
        &self.proposal_hash
    }
}

/// H4 offline adapter-training plan with base weights frozen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduOfflineAdapterTrainingPlan {
    base_model_hash: ContentHash,
    dataset: NduDatasetManifestRef,
    trainer_config_hash: ContentHash,
    verifier_hash: ContentHash,
    base_weights_frozen: bool,
    shadow_required: bool,
    canary_required: bool,
    publication_authority_granted: bool,
}

impl NduOfflineAdapterTrainingPlan {
    /// Creates an offline plan that cannot publish an adapter.
    pub fn new(
        base_model_hash: ContentHash,
        dataset: NduDatasetManifestRef,
        trainer_config_hash: ContentHash,
        verifier_hash: ContentHash,
    ) -> Self {
        Self {
            base_model_hash,
            dataset,
            trainer_config_hash,
            verifier_hash,
            base_weights_frozen: true,
            shadow_required: true,
            canary_required: true,
            publication_authority_granted: false,
        }
    }

    /// Returns whether this plan is eligible for isolated offline execution.
    pub const fn is_offline_training_eligible(&self) -> bool {
        self.base_weights_frozen
            && self.shadow_required
            && self.canary_required
            && !self.publication_authority_granted
            && self.dataset.is_offline_evaluation_eligible()
    }

    /// Returns the base-model digest.
    pub fn base_model_hash(&self) -> &ContentHash {
        &self.base_model_hash
    }

    /// Returns the frozen dataset manifest.
    pub fn dataset(&self) -> &NduDatasetManifestRef {
        &self.dataset
    }

    /// Returns the trainer configuration digest.
    pub fn trainer_config_hash(&self) -> &ContentHash {
        &self.trainer_config_hash
    }

    /// Returns the independently versioned verifier digest.
    pub fn verifier_hash(&self) -> &ContentHash {
        &self.verifier_hash
    }
}

/// H5 research-only readiness for low-frequency consolidation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NduConsolidationReadiness {
    /// Long-horizon evidence is insufficient or unstable.
    Blocked,
    /// Research may begin, but no weight update authority is granted.
    ResearchOnly,
}

/// Evaluates the H5 long-horizon gate without changing any model weight.
pub const fn evaluate_ndu_consolidation_readiness(
    stable_windows: u32,
    required_stable_windows: u32,
    ndu_wins_over_all_simple_baselines: bool,
    statistical_significance_verified: bool,
    forgetting_regression_absent: bool,
    rollback_rehearsed: bool,
) -> NduConsolidationReadiness {
    if required_stable_windows > 0
        && stable_windows >= required_stable_windows
        && ndu_wins_over_all_simple_baselines
        && statistical_significance_verified
        && forgetting_regression_absent
        && rollback_rehearsed
    {
        NduConsolidationReadiness::ResearchOnly
    } else {
        NduConsolidationReadiness::Blocked
    }
}

fn utility_for_baseline(
    _baseline: NduBaselineKind,
    observation: &NduShadowObservation,
) -> Option<UtilityVector> {
    Some(UtilityVector::new(
        bounded(observation.task_signal_basis_points)?,
        bounded(observation.learning_signal_basis_points)?,
        bounded(observation.trust_signal_basis_points)?,
        bounded(observation.memory_pollution_risk_basis_points)?,
        bounded(observation.resource_cost_basis_points)?,
        bounded(observation.uncertainty_basis_points)?,
    ))
}

fn bounded(value: i32) -> Option<BoundedUtilityScore> {
    BoundedUtilityScore::try_new(value.clamp(-SCORE_LIMIT, SCORE_LIMIT))
}

fn stable_hash(parts: &[&str]) -> ContentHash {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part.as_bytes());
    }
    ContentHash::new(format!("sha256:{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn satisfied_mask() -> HardFeasibilityMask {
        HardFeasibilityMask::new(
            HardFeasibilityVerdict::Satisfied,
            HardFeasibilityVerdict::Satisfied,
            HardFeasibilityVerdict::Satisfied,
            HardFeasibilityVerdict::Satisfied,
        )
    }

    fn observation() -> NduShadowObservation {
        NduShadowObservation::new(
            NduUtilityEventRef::new(
                ContentHash::new("event"),
                ContentHash::new("receipt"),
                ContentHash::new("subject-pseudonym"),
                None,
            ),
            1_000,
            500,
            750,
            100,
            300,
            200,
            5_000,
            None,
        )
    }

    fn transition(mask: HardFeasibilityMask) -> NduUtilityTransitionRef {
        evaluate_ndu_shadow_arm(
            &NduShadowBoundary::required(),
            NduBaselineKind::NduShadow,
            Revision::new(1),
            ContentHash::new("previous"),
            observation(),
            ContentHash::new("model"),
            ContentHash::new("config"),
            mask,
        )
        .expect("required boundary should permit shadow evaluation")
        .transition
    }

    #[test]
    fn shadow_replay_is_deterministic_and_non_mutating() {
        let first = transition(satisfied_mask());
        let second = transition(satisfied_mask());

        assert_eq!(first, second);
        assert!(NduShadowBoundary::required().is_shadow_only());
    }

    #[test]
    fn canary_abstains_on_hard_constraint_failure() {
        let blocked = HardFeasibilityMask::new(
            HardFeasibilityVerdict::Unknown,
            HardFeasibilityVerdict::Satisfied,
            HardFeasibilityVerdict::Satisfied,
            HardFeasibilityVerdict::Satisfied,
        );
        let decision =
            evaluate_ndu_budget_canary(&transition(blocked), 10, 10, 100, 9_000, 8_000, 100, 500);

        assert_eq!(
            decision,
            NduBudgetCanaryDecision::Abstain(NduCanaryAbstentionReason::HardFeasibilityBlocked)
        );
    }

    #[test]
    fn proposal_adapter_and_consolidation_remain_fail_closed() {
        let proposal = NduGovernedProposalPlan::proposal_only(
            NduProposalKind::Skill,
            ContentHash::new("proposal"),
        );
        let adapter = NduOfflineAdapterTrainingPlan::new(
            ContentHash::new("base"),
            NduDatasetManifestRef::new(
                ContentHash::new("dataset"),
                ContentHash::new("schema"),
                10,
                true,
                ContentHash::new("consent"),
                ContentHash::new("revocation"),
            ),
            ContentHash::new("trainer"),
            ContentHash::new("verifier"),
        );

        assert!(proposal.is_fail_closed());
        assert!(adapter.is_offline_training_eligible());
        assert_eq!(
            evaluate_ndu_consolidation_readiness(11, 12, true, true, true, true),
            NduConsolidationReadiness::Blocked
        );
        assert_eq!(
            evaluate_ndu_consolidation_readiness(12, 12, true, true, true, true),
            NduConsolidationReadiness::ResearchOnly
        );
        let threat_model = NduThreatModel::canonical();
        assert!(threat_model.is_complete());
        assert!(threat_model.prohibits(NduProhibitedAuthority::BaseModelWeights));
    }
}
