use std::fmt::Debug;

use codex_hepta_types::Digest32;
use codex_hepta_types::FixedQ32;
use codex_hepta_types::StableId;

use super::*;

fn must<T, E: Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("unexpected error: {error:?}"),
    }
}

fn id(value: &str) -> StableId {
    must(StableId::new(value))
}

fn request() -> EvaluationRequest {
    EvaluationRequest {
        evaluation_id: id("eval-1"),
        evaluator_id: id("independent-eval"),
        candidate_id: id("candidate"),
        candidate_producer_id: id("producer"),
        baseline_id: id("baseline"),
        objective_digest: Digest32::of_bytes(b"objective"),
        comparisons: vec![MetricComparison {
            metric_id: id("safety"),
            direction: Direction::Minimize,
            candidate: FixedQ32::ZERO,
            baseline: FixedQ32::ONE,
            minimum_delta: FixedQ32::ZERO,
            hard: true,
            support_digest: Digest32::of_bytes(b"evidence"),
        }],
    }
}

#[test]
fn eligible_is_not_promotion() {
    assert_eq!(
        must(evaluate(request())).disposition,
        Disposition::EligibleForFurtherReview
    );
}

#[test]
fn self_evaluation_is_rejected() {
    let mut value = request();
    value.evaluator_id = value.candidate_producer_id.clone();
    assert_eq!(evaluate(value), Err(Error::SelfEvaluation));
}

#[test]
fn hard_regression_is_rejected() {
    let mut value = request();
    value.comparisons[0].candidate = FixedQ32::ONE;
    value.comparisons[0].baseline = FixedQ32::ZERO;
    assert_eq!(must(evaluate(value)).disposition, Disposition::Ineligible);
}

#[test]
fn missing_support_is_insufficient() {
    let mut value = request();
    value.comparisons[0].support_digest = Digest32::ZERO;
    assert_eq!(
        must(evaluate(value)).disposition,
        Disposition::InsufficientEvidence
    );
}
