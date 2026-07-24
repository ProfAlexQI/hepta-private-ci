use super::admission_reason;
use super::test_support::*;
use super::*;
use hepta_contracts::CandidateId;
use hepta_contracts::ContentHash;
use hepta_contracts::JointCandidate;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;
use hepta_core::ApprovalRequirement;

#[test]
fn none_and_ask_policy_evidence_enter_the_feasible_set() {
    let gate = HeptaKernelSafetyGate::new();
    let none_candidate = candidate_for_requirement(
        context(1, 1, 1),
        "action",
        "payload",
        ApprovalRequirement::None,
    );
    let ask_candidate = candidate_for_requirement(
        context(1, 1, 1),
        "action",
        "payload",
        ApprovalRequirement::Ask,
    );

    let none = admission_for(&gate, &none_candidate, ApprovalRequirement::None);
    let ask = admission_for(&gate, &ask_candidate, ApprovalRequirement::Ask);

    assert!(none.is_admitted());
    assert!(ask.is_admitted());
    assert_ne!(none.evidence_hash(), ask.evidence_hash());
    assert_ne!(
        none.admission().content_hash(),
        ask.admission().content_hash()
    );
}

#[test]
fn deny_and_inconsistent_policy_evidence_fail_closed() {
    let gate = HeptaKernelSafetyGate::new();
    let denied_candidate = candidate_for_requirement(
        context(1, 1, 1),
        "action",
        "payload",
        ApprovalRequirement::Deny,
    );
    let denied = admission_for(&gate, &denied_candidate, ApprovalRequirement::Deny);
    assert_eq!(
        denied.rejection_reason_code(),
        Some(admission_reason::POLICY_DENIED)
    );

    let candidate = candidate(context(1, 1, 1), "action", "payload");
    let snapshot_drift = evidence_with_requirements(
        stamp("policy", 2),
        candidate.payload_set_hash(),
        descriptor(candidate.context()),
        ApprovalRequirement::None,
        ApprovalRequirement::None,
        "kernel test policy",
    );
    let decision_drift = evidence_with_requirements(
        candidate.context().policy().clone(),
        candidate.payload_set_hash(),
        descriptor(candidate.context()),
        ApprovalRequirement::None,
        ApprovalRequirement::Ask,
        "kernel test policy",
    );

    assert_eq!(
        admit_with_evidence(&gate, &candidate, &snapshot_drift).rejection_reason_code(),
        Some(admission_reason::POLICY_SNAPSHOT_MISMATCH)
    );
    assert_eq!(
        admit_with_evidence(&gate, &candidate, &decision_drift).rejection_reason_code(),
        Some(admission_reason::POLICY_DECISION_MISMATCH)
    );

    let fabricated_allow = candidate_with_requirements(
        context(1, 1, 1),
        "action",
        "payload",
        ApprovalRequirement::Deny,
        ApprovalRequirement::None,
    );
    let deny_rules_allow_decision = evidence_with_requirements(
        fabricated_allow.context().policy().clone(),
        fabricated_allow.payload_set_hash(),
        descriptor(fabricated_allow.context()),
        ApprovalRequirement::Deny,
        ApprovalRequirement::None,
        "kernel test policy",
    );
    assert_eq!(
        admit_with_evidence(&gate, &fabricated_allow, &deny_rules_allow_decision)
            .rejection_reason_code(),
        Some(admission_reason::POLICY_DECISION_MISMATCH)
    );
}

#[test]
fn malformed_capability_descriptor_and_request_count_fail_closed() {
    let gate = HeptaKernelSafetyGate::new();
    let candidate = candidate(context(1, 1, 1), "action", "payload");
    let malformed_descriptor = hepta_contracts::CapabilityDescriptor::new(
        descriptor(candidate.context()).id().clone(),
        Revision::new(1),
        ContentHash::new("manifest-1"),
        candidate.context().capability_catalog().clone(),
        PrincipalId::new(""),
        "",
    );
    let malformed_evidence = evidence(
        candidate.context(),
        candidate.payload_set_hash(),
        malformed_descriptor,
        ApprovalRequirement::None,
    );
    assert_eq!(
        admit_with_evidence(&gate, &candidate, &malformed_evidence).rejection_reason_code(),
        Some(admission_reason::CAPABILITY_DESCRIPTOR_MALFORMED)
    );

    let no_request = JointCandidate::try_new(
        CandidateId::new("candidate-no-request"),
        Revision::new(1),
        ContentHash::new("candidate-no-request"),
        candidate.context().clone(),
        candidate.action_hash().clone(),
        candidate.metacontrol_hash().clone(),
        candidate.payload_set_hash().clone(),
        Vec::new(),
        Vec::new(),
    )
    .expect("contracts permit a candidate without requests");
    let no_request_evidence = evidence(
        no_request.context(),
        no_request.payload_set_hash(),
        descriptor(no_request.context()),
        ApprovalRequirement::None,
    );
    assert_eq!(
        admit_with_evidence(&gate, &no_request, &no_request_evidence).rejection_reason_code(),
        Some(admission_reason::CAPABILITY_REQUEST_COUNT)
    );
}

#[test]
fn request_and_payload_set_substitution_have_stable_rejections() {
    let gate = HeptaKernelSafetyGate::new();
    let candidate = candidate(context(1, 1, 1), "action", "payload");
    let wrong_descriptor = hepta_contracts::CapabilityDescriptor::new(
        descriptor(candidate.context()).id().clone(),
        Revision::new(1),
        ContentHash::new("different-manifest"),
        candidate.context().capability_catalog().clone(),
        PrincipalId::new("provider"),
        "tool.invoke",
    );
    let descriptor_evidence = evidence(
        candidate.context(),
        candidate.payload_set_hash(),
        wrong_descriptor,
        ApprovalRequirement::None,
    );
    let payload_evidence = evidence(
        candidate.context(),
        &ContentHash::new("different-payload"),
        descriptor(candidate.context()),
        ApprovalRequirement::None,
    );
    assert_eq!(
        admit_with_evidence(&gate, &candidate, &descriptor_evidence).rejection_reason_code(),
        Some(admission_reason::CAPABILITY_REQUEST_DESCRIPTOR_MISMATCH)
    );
    assert_eq!(
        admit_with_evidence(&gate, &candidate, &payload_evidence).rejection_reason_code(),
        Some(admission_reason::CAPABILITY_REQUEST_PAYLOAD_MISMATCH)
    );

    let request = candidate.capability_requests()[0].clone();
    let mismatched_set = JointCandidate::try_new(
        CandidateId::new("candidate-payload-set-mismatch"),
        Revision::new(1),
        ContentHash::new("candidate-payload-set-mismatch"),
        candidate.context().clone(),
        candidate.action_hash().clone(),
        candidate.metacontrol_hash().clone(),
        ContentHash::new("different-payload-set"),
        candidate.contributors().to_vec(),
        vec![request],
    )
    .expect("contract keeps the producer-owned payload-set digest opaque");
    let exact_request_evidence = evidence(
        mismatched_set.context(),
        candidate.capability_requests()[0].payload_hash(),
        descriptor(mismatched_set.context()),
        ApprovalRequirement::None,
    );
    assert_eq!(
        admit_with_evidence(&gate, &mismatched_set, &exact_request_evidence)
            .rejection_reason_code(),
        Some(admission_reason::PAYLOAD_SET_MISMATCH)
    );
}
