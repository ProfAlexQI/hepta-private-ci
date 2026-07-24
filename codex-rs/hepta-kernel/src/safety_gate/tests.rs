use super::test_support::*;
use super::*;
use hepta_contracts::ObservationId;
use hepta_contracts::ObservationRef;
use hepta_core::ApprovalRequirement;

#[test]
fn exact_admitted_candidate_is_authorized() {
    let gate = HeptaKernelSafetyGate::new();
    let candidate = candidate(context(1, 1, 1), "action-1", "payload-1");
    let admission = admission(&gate, &candidate);

    let authorized = authorize(
        &gate,
        &admission,
        &candidate,
        candidate.payload_set_hash(),
        candidate.context(),
    )
    .expect("unchanged admitted candidate must authorize");
    let authorization = authorized.authorization();

    assert!(matches!(
        authorization.decision(),
        AuthorizationDecision::Authorized { .. }
    ));
    assert_eq!(
        authorization.payload_set_hash(),
        candidate.payload_set_hash()
    );
    assert_eq!(authorization.current_context(), candidate.context());
    assert_eq!(authorized.binding(), admission.binding());
}

#[test]
fn payload_substitution_is_rejected() {
    let gate = HeptaKernelSafetyGate::new();
    let candidate = candidate(context(1, 1, 1), "action-1", "payload-1");
    let admission = admission(&gate, &candidate);

    let error = authorize(
        &gate,
        &admission,
        &candidate,
        &ContentHash::new("payload-2"),
        candidate.context(),
    )
    .expect_err("substituted payload must fail");

    assert_eq!(error, HeptaKernelSafetyGateError::PayloadSubstitution);
    assert_eq!(error.code(), "safety_gate.payload_substitution");
}

#[test]
fn declared_content_hash_cannot_hide_candidate_substitution() {
    let gate = HeptaKernelSafetyGate::new();
    let admitted_candidate = candidate(context(1, 1, 1), "action-1", "payload-1");
    let substituted_candidate = candidate(context(1, 1, 1), "action-2", "payload-1");
    let admission = admission(&gate, &admitted_candidate);

    let error = authorize(
        &gate,
        &admission,
        &substituted_candidate,
        substituted_candidate.payload_set_hash(),
        substituted_candidate.context(),
    )
    .expect_err("same declared hash must not mask changed action");

    assert_eq!(error, HeptaKernelSafetyGateError::CandidateSubstitution);
}

#[test]
fn approval_must_name_the_admitted_exact_binding() {
    let gate = HeptaKernelSafetyGate::new();
    let admitted_candidate = candidate(context(1, 1, 1), "action-1", "payload-1");
    let other = candidate(context(1, 1, 1), "action-2", "payload-1");
    let admission = admission(&gate, &admitted_candidate);
    let other_binding = gate.exact_candidate_binding(&other);

    let error = gate
        .authorize_at_commit(
            AuthorizationId::new("authorization"),
            Revision::new(1),
            &admission,
            &other_binding,
            &admitted_candidate,
            admitted_candidate.payload_set_hash(),
            admitted_candidate.context(),
            PrincipalId::new("safety-gate"),
            ContentHash::new("scope-1"),
        )
        .expect_err("approval for another exact candidate must fail");

    assert_eq!(
        error,
        HeptaKernelSafetyGateError::ApprovedCandidateBindingMismatch
    );
}

#[test]
fn policy_catalog_and_state_drift_have_stable_errors() {
    let gate = HeptaKernelSafetyGate::new();
    let candidate = candidate(context(1, 1, 1), "action-1", "payload-1");
    let admission = admission(&gate, &candidate);

    for (current, expected) in [
        (context(1, 2, 1), HeptaKernelSafetyGateError::PolicyDrift),
        (
            context(1, 1, 2),
            HeptaKernelSafetyGateError::CapabilityCatalogDrift,
        ),
        (context(2, 1, 1), HeptaKernelSafetyGateError::StateDrift),
    ] {
        let error = authorize(
            &gate,
            &admission,
            &candidate,
            candidate.payload_set_hash(),
            &current,
        )
        .expect_err("context drift must fail");
        assert_eq!(error, expected);
    }
}

#[test]
fn observation_and_preference_drift_have_stable_errors() {
    let gate = HeptaKernelSafetyGate::new();
    let candidate = candidate(context(1, 1, 1), "action-1", "payload-1");
    let admission = admission(&gate, &candidate);
    let original = candidate.context();
    let observation_drift = FrozenTurnContext::new(
        ObservationRef::new(
            ObservationId::new("observation-2"),
            Revision::new(2),
            ContentHash::new("observation-2"),
        ),
        original.state().clone(),
        original.policy().clone(),
        original.capability_catalog().clone(),
        original.preference().clone(),
    );
    let preference_drift = FrozenTurnContext::new(
        original.observation().clone(),
        original.state().clone(),
        original.policy().clone(),
        original.capability_catalog().clone(),
        stamp("preference", 2),
    );

    for (current, expected) in [
        (
            observation_drift,
            HeptaKernelSafetyGateError::ObservationDrift,
        ),
        (
            preference_drift,
            HeptaKernelSafetyGateError::PreferenceDrift,
        ),
    ] {
        let error = authorize(
            &gate,
            &admission,
            &candidate,
            candidate.payload_set_hash(),
            &current,
        )
        .expect_err("observation or preference drift must fail");
        assert_eq!(error, expected);
    }
}

#[test]
fn rejected_admission_cannot_be_authorized() {
    let gate = HeptaKernelSafetyGate::new();
    let candidate = candidate_for_requirement(
        context(1, 1, 1),
        "action-1",
        "payload-1",
        ApprovalRequirement::Deny,
    );
    let admission = admission_for(&gate, &candidate, ApprovalRequirement::Deny);

    let error = authorize(
        &gate,
        &admission,
        &candidate,
        candidate.payload_set_hash(),
        candidate.context(),
    )
    .expect_err("rejected admission must fail");

    assert_eq!(error, HeptaKernelSafetyGateError::AdmissionNotAdmitted);
}

#[test]
fn framed_hash_protocol_has_a_stable_golden_digest() {
    let mut hash = FramedHash::new("hepta.test.kernel-framing.v1");
    hash.text("text", "value");
    hash.number("number", 7);

    assert_eq!(
        hash.finish().as_str(),
        "sha256:c726d655e3b6cb01f098ce94b4d0082331d5b483a82649c0a22e77d53b7b36d3"
    );
}
