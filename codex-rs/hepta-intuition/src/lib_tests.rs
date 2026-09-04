use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn probability(raw: u64) -> ProbabilityQ32 {
    let Ok(value) = ProbabilityQ32::from_raw(raw) else {
        panic!("test probability must be in range");
    };
    value
}

fn candidate(name: &str, utility: i64, confidence: u64) -> ActionCandidate {
    ActionCandidate {
        candidate_id: id(name),
        legal: true,
        hard_veto: false,
        utility: FixedQ32::from_raw(utility),
        confidence: probability(confidence),
        support_digest: digest(name.as_bytes()),
    }
}

fn request(candidates: Vec<ActionCandidate>) -> DecisionRequest {
    DecisionRequest {
        decision_id: id("decision:1"),
        objective_digest: digest(b"objective"),
        candidate_set_digest: digest(b"candidate-set"),
        minimum_confidence: probability(1),
        candidates,
    }
}

#[test]
fn hard_veto_cannot_be_overridden() {
    let mut vetoed = candidate("action:vetoed", 100, ProbabilityQ32::ONE.raw());
    vetoed.hard_veto = true;
    let allowed = candidate("action:allowed", 10, ProbabilityQ32::ONE.raw());
    let Ok(receipt) = decide(request(vec![vetoed, allowed])) else {
        panic!("decision must succeed");
    };
    assert_eq!(receipt.decision, Decision::Selected(id("action:allowed")));
    assert!(!receipt.authority.grants_any());
}

#[test]
fn low_confidence_abstains_and_records_complete_propensities() {
    let mut value = request(vec![candidate("action:a", 10, 1)]);
    value.minimum_confidence = ProbabilityQ32::ONE;
    let Ok(receipt) = decide(value) else {
        panic!("decision must succeed");
    };
    assert_eq!(
        receipt.decision,
        Decision::Abstained(AbstentionReason::LowConfidence)
    );
    assert_eq!(receipt.propensities.len(), 1);
    assert_eq!(receipt.propensities[0].probability, ProbabilityQ32::ZERO);
    assert_eq!(receipt.abstain_probability, ProbabilityQ32::ONE);
}

#[test]
fn tie_breaking_is_canonical() {
    let left = candidate("action:b", 10, ProbabilityQ32::ONE.raw());
    let right = candidate("action:a", 10, ProbabilityQ32::ONE.raw());
    let Ok(receipt) = decide(request(vec![left, right])) else {
        panic!("decision must succeed");
    };
    assert_eq!(receipt.decision, Decision::Selected(id("action:a")));
}

#[test]
fn duplicate_candidates_are_rejected() {
    let value = candidate("action:a", 10, ProbabilityQ32::ONE.raw());
    assert_eq!(
        decide(request(vec![value.clone(), value])),
        Err(Error::DuplicateCandidate("action:a".to_string()))
    );
}
