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

fn generation(value: u64) -> Generation {
    let Ok(value) = Generation::new(value) else {
        panic!("test generation must be non-zero");
    };
    value
}

fn request() -> ProposalRequest {
    ProposalRequest {
        proposal_id: id("proposal:1"),
        proposer_id: id("proposer:1"),
        evaluator_id: id("evaluator:1"),
        baseline_generation: generation(1),
        candidate_generation: generation(2),
        evaluation_digest: digest(b"evaluation"),
        evaluation_eligible: true,
        maximum_absolute_delta: FixedQ32::from_raw(10),
        parameter_deltas: vec![ParameterDelta {
            parameter_id: id("parameter:1"),
            delta: FixedQ32::from_raw(5),
            lower_bound: FixedQ32::from_raw(-10),
            upper_bound: FixedQ32::from_raw(10),
            evidence_digest: digest(b"parameter-evidence"),
        }],
        topology_deltas: vec![TopologyDelta {
            module_id: id("module:1"),
            operation: TopologyOperation::Replace,
            predecessor_digest: digest(b"old"),
            candidate_digest: digest(b"new"),
            evidence_digest: digest(b"topology-evidence"),
        }],
    }
}

#[test]
fn proposal_requires_independent_acceptance_and_grants_no_authority() {
    let Ok(proposal) = propose(request()) else {
        panic!("proposal must be valid");
    };
    assert_eq!(
        proposal.status,
        ProposalStatus::RequiresIndependentAcceptance
    );
    assert!(!proposal.authority.grants_any());
}

#[test]
fn self_evaluation_is_rejected() {
    let mut value = request();
    value.evaluator_id = value.proposer_id.clone();
    assert_eq!(propose(value), Err(Error::SelfEvaluation));
}

#[test]
fn parameter_delta_is_bounded() {
    let mut value = request();
    value.parameter_deltas[0].delta = FixedQ32::from_raw(11);
    assert_eq!(
        propose(value),
        Err(Error::DeltaOutsideBounds("parameter:1".to_string()))
    );
}

#[test]
fn generation_must_advance() {
    let mut value = request();
    value.candidate_generation = generation(1);
    assert_eq!(propose(value), Err(Error::GenerationNotAdvanced));
}

#[test]
fn registry_is_idempotent_and_conflict_detecting() {
    let Ok(proposal) = propose(request()) else {
        panic!("proposal must be valid");
    };
    let mut registry = ProposalRegistry::new(4);
    assert_eq!(
        registry.append(proposal.clone()),
        Ok(AppendDisposition::Inserted)
    );
    assert_eq!(
        registry.append(proposal.clone()),
        Ok(AppendDisposition::Unchanged)
    );
    let mut drifted = proposal;
    drifted.proposal_digest = digest(b"drift");
    assert_eq!(
        registry.append(drifted),
        Err(Error::ProposalConflict("proposal:1".to_string()))
    );
}
