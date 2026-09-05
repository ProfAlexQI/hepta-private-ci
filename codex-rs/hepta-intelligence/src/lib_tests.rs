use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn candidate(name: &str, score: i64) -> PlanCandidate {
    PlanCandidate {
        candidate_id: id(name),
        legal: true,
        hard_veto: false,
        score: FixedQ32::from_raw(score),
        support_digest: Digest32::of_bytes(name.as_bytes()),
    }
}

fn request(candidates: Vec<PlanCandidate>) -> PlanningRequest {
    PlanningRequest {
        plan_id: id("plan:1"),
        objective_digest: Digest32::of_bytes(b"objective"),
        context_digest: Digest32::of_bytes(b"context"),
        snapshot_digest: Digest32::of_bytes(b"snapshot"),
        candidates,
    }
}

#[test]
fn highest_eligible_candidate_is_selected_without_effect_authority() {
    let Ok(receipt) = compose(request(vec![
        candidate("candidate:a", 10),
        candidate("candidate:b", 20),
    ])) else {
        panic!("planning must succeed");
    };
    assert_eq!(receipt.decision, PlanDecision::Selected(id("candidate:b")));
    assert!(!receipt.effect_authority);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn hard_veto_is_never_overridden() {
    let allowed = candidate("candidate:allowed", 10);
    let mut vetoed = candidate("candidate:vetoed", 100);
    vetoed.hard_veto = true;
    let Ok(receipt) = compose(request(vec![vetoed, allowed])) else {
        panic!("planning must succeed");
    };
    assert_eq!(
        receipt.decision,
        PlanDecision::Selected(id("candidate:allowed"))
    );
}

#[test]
fn no_eligible_candidate_abstains() {
    let mut value = candidate("candidate:a", 10);
    value.legal = false;
    let Ok(receipt) = compose(request(vec![value])) else {
        panic!("planning must succeed");
    };
    assert_eq!(
        receipt.decision,
        PlanDecision::Abstained(AbstentionReason::NoEligibleCandidate)
    );
}

#[test]
fn duplicate_candidate_is_rejected() {
    let value = candidate("candidate:a", 10);
    assert_eq!(
        compose(request(vec![value.clone(), value])),
        Err(Error::DuplicateCandidate("candidate:a".to_string()))
    );
}
