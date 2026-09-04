use std::fmt::Debug;

use codex_hepta_types::Digest32;
use codex_hepta_types::FixedQ32;
use codex_hepta_types::Generation;
use codex_hepta_types::StableId;
use pretty_assertions::assert_eq;

use super::canonical_scalarization_digest;
use super::evaluate_candidates;
use crate::AxisDirection;
use crate::AxisLimit;
use crate::AxisValue;
use crate::ContributionSet;
use crate::EvaluationDisposition;
use crate::FeasibilityPosture;
use crate::RequiredOrganSet;
use crate::ScalarizationProfile;
use crate::UtilityContribution;
use crate::UtilityProfile;

fn must<T, E: Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("unexpected error: {error:?}"),
    }
}

fn must_err<T: Debug, E>(result: Result<T, E>) -> E {
    match result {
        Err(error) => error,
        Ok(value) => panic!("expected error, received value: {value:?}"),
    }
}

fn id(value: &str) -> StableId {
    must(StableId::new(value))
}

fn q32(value: i64) -> FixedQ32 {
    FixedQ32::from_raw(value << 32)
}

fn contribution(candidate: &str, success: i64, latency: i64) -> UtilityContribution {
    UtilityContribution {
        candidate_id: id(candidate),
        organ_id: id("planner"),
        objective_digest: Digest32::of_bytes(b"objective"),
        generation: must(Generation::new(1)),
        feasibility: FeasibilityPosture::Feasible,
        utility: vec![
            AxisValue {
                axis: id("success"),
                value: q32(success),
            },
            AxisValue {
                axis: id("latency"),
                value: q32(latency),
            },
        ],
        risk: vec![AxisValue {
            axis: id("privacy-risk"),
            value: FixedQ32::ZERO,
        }],
        resource: vec![AxisValue {
            axis: id("compute"),
            value: q32(1),
        }],
        uncertainty: vec![AxisValue {
            axis: id("success"),
            value: FixedQ32::ZERO,
        }],
        support_digest: Digest32::of_bytes(candidate.as_bytes()),
    }
}

fn profile() -> UtilityProfile {
    UtilityProfile {
        profile_id: id("utility-v1"),
        dimensions: vec![
            (id("success"), AxisDirection::Maximize),
            (id("latency"), AxisDirection::Minimize),
        ],
        risk_ceilings: vec![AxisLimit {
            axis: id("privacy-risk"),
            maximum: FixedQ32::ZERO,
        }],
        resource_ceilings: vec![AxisLimit {
            axis: id("compute"),
            maximum: q32(10),
        }],
        required_organs: RequiredOrganSet {
            organ_ids: vec![id("planner")],
        },
    }
}

fn set(contributions: Vec<UtilityContribution>) -> ContributionSet {
    ContributionSet {
        objective_digest: Digest32::of_bytes(b"objective"),
        generation: must(Generation::new(1)),
        contributions,
    }
}

#[test]
fn hard_violation_is_filtered_before_utility() {
    let abstain = contribution("abstain", 0, 0);
    let mut unsafe_candidate = contribution("unsafe-high-score", 100, 0);
    unsafe_candidate.feasibility = FeasibilityPosture::HardConstraintViolation;

    let receipt = must(evaluate_candidates(
        set(vec![abstain, unsafe_candidate]),
        profile(),
        None,
    ));

    assert_eq!(
        receipt.disposition,
        EvaluationDisposition::InfeasibleExplicitAbstain
    );
    assert_eq!(receipt.advisory_recommendation, Some(id("abstain")));
    assert_eq!(receipt.rejected_candidates.len(), 1);
    assert_eq!(
        receipt.rejected_candidates[0].candidate_id,
        id("unsafe-high-score")
    );
}

#[test]
fn non_dominated_candidates_without_profile_require_slow_path() {
    let receipt = must(evaluate_candidates(
        set(vec![
            contribution("abstain", 0, 0),
            contribution("fast", 1, 1),
            contribution("accurate", 2, 2),
        ]),
        profile(),
        None,
    ));

    assert_eq!(
        receipt.disposition,
        EvaluationDisposition::ParetoSetRequiresSlowPath
    );
    assert_eq!(receipt.advisory_recommendation, None);
    assert_eq!(receipt.pareto_frontier.len(), 3);
}

#[test]
fn registered_scalarization_produces_advisory_recommendation() {
    let scalarization = ScalarizationProfile {
        profile_id: id("weights-v1"),
        weights: vec![
            AxisValue {
                axis: id("success"),
                value: FixedQ32::ONE,
            },
            AxisValue {
                axis: id("latency"),
                value: FixedQ32::ZERO,
            },
        ],
    };
    let receipt = must(evaluate_candidates(
        set(vec![
            contribution("abstain", 0, 0),
            contribution("fast", 1, 1),
            contribution("accurate", 2, 2),
        ]),
        profile(),
        Some(scalarization),
    ));

    assert_eq!(
        receipt.disposition,
        EvaluationDisposition::ScalarizedRecommendation
    );
    assert_eq!(receipt.advisory_recommendation, Some(id("accurate")));
    assert!(receipt.scalarization_profile_digest.is_some());
}

#[test]
fn missing_required_contribution_is_not_treated_as_zero() {
    let mut required = profile();
    required.required_organs.organ_ids.push(id("risk-observer"));

    let error = must_err(evaluate_candidates(
        set(vec![contribution("abstain", 0, 0)]),
        required,
        None,
    ));

    assert_eq!(error.code(), "NDU-E003");
}

#[test]
fn infeasible_abstain_is_rejected_before_any_recommendation() {
    let mut abstain = contribution("abstain", 0, 0);
    abstain.feasibility = FeasibilityPosture::HardConstraintViolation;

    let error = must_err(evaluate_candidates(
        set(vec![abstain, contribution("safe", 1, 1)]),
        profile(),
        None,
    ));

    assert_eq!(error.code(), "NDU-E005");
}

#[test]
fn scalarization_digest_is_computed_from_canonical_inputs() {
    let first = ScalarizationProfile {
        profile_id: id("weights-v1"),
        weights: vec![
            AxisValue {
                axis: id("success"),
                value: FixedQ32::ONE,
            },
            AxisValue {
                axis: id("latency"),
                value: FixedQ32::ZERO,
            },
        ],
    };
    let mut reordered = first.clone();
    reordered.weights.reverse();

    assert_eq!(
        must(canonical_scalarization_digest(&first)),
        must(canonical_scalarization_digest(&reordered))
    );
}
