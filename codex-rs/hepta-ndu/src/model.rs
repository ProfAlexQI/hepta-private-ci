use codex_hepta_types::Digest32;
use codex_hepta_types::FixedQ32;
use codex_hepta_types::Generation;
use codex_hepta_types::StableId;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AxisDirection {
    Maximize,
    Minimize,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AxisValue {
    pub axis: StableId,
    pub value: FixedQ32,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AxisLimit {
    pub axis: StableId,
    pub maximum: FixedQ32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeasibilityPosture {
    Feasible,
    HardConstraintViolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UtilityContribution {
    pub candidate_id: StableId,
    pub organ_id: StableId,
    pub objective_digest: Digest32,
    pub generation: Generation,
    pub feasibility: FeasibilityPosture,
    pub utility: Vec<AxisValue>,
    pub risk: Vec<AxisValue>,
    pub resource: Vec<AxisValue>,
    pub uncertainty: Vec<AxisValue>,
    pub support_digest: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContributionSet {
    pub objective_digest: Digest32,
    pub generation: Generation,
    pub contributions: Vec<UtilityContribution>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequiredOrganSet {
    pub organ_ids: Vec<StableId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UtilityProfile {
    pub profile_id: StableId,
    pub dimensions: Vec<(StableId, AxisDirection)>,
    pub risk_ceilings: Vec<AxisLimit>,
    pub resource_ceilings: Vec<AxisLimit>,
    pub required_organs: RequiredOrganSet,
}

/// Registered scalarization inputs. The evaluator computes and returns the
/// canonical profile digest; callers cannot inject a self-asserted digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScalarizationProfile {
    pub profile_id: StableId,
    pub weights: Vec<AxisValue>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateUtility {
    pub candidate_id: StableId,
    pub utility: Vec<AxisValue>,
    pub risk: Vec<AxisValue>,
    pub resource: Vec<AxisValue>,
    pub uncertainty: Vec<AxisValue>,
    pub support_digest: Digest32,
    pub scalar_score: Option<FixedQ32>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CandidateRejectionReason {
    HardConstraintViolation,
    RiskCeilingExceeded,
    ResourceCeilingExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedCandidate {
    pub candidate_id: StableId,
    pub reasons: Vec<CandidateRejectionReason>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvaluationDisposition {
    InfeasibleExplicitAbstain,
    UniqueParetoRecommendation,
    ParetoSetRequiresSlowPath,
    ScalarizedRecommendation,
    ScalarizationTieRequiresSlowPath,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NduEvaluationReceipt {
    pub objective_digest: Digest32,
    pub generation: Generation,
    pub disposition: EvaluationDisposition,
    pub utility_profile_digest: Digest32,
    pub scalarization_profile_digest: Option<Digest32>,
    pub evaluated_candidates: Vec<CandidateUtility>,
    pub rejected_candidates: Vec<RejectedCandidate>,
    pub pareto_frontier: Vec<CandidateUtility>,
    pub advisory_recommendation: Option<StableId>,
    pub evaluation_digest: Digest32,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SubjectClass {
    System,
    Domain,
    Agent,
    Episode,
}

impl SubjectClass {
    pub(crate) const fn tag(self) -> u8 {
        match self {
            Self::System => 0,
            Self::Domain => 1,
            Self::Agent => 2,
            Self::Episode => 3,
        }
    }
}
