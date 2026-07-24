use super::HeptaKernelAdmissionEvidence;
use super::HeptaKernelPolicyEvidence;
use super::HeptaKernelSafetyAdmission;
use super::HeptaKernelSafetyAuthorization;
use super::HeptaKernelSafetyGate;
use super::HeptaKernelSafetyGateError;
use hepta_contracts::AdmissionId;
use hepta_contracts::AuthorizationId;
use hepta_contracts::CandidateId;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::CapabilityId;
use hepta_contracts::CapabilityRequestId;
use hepta_contracts::CapabilityRequestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::JointCandidate;
use hepta_contracts::ObservationId;
use hepta_contracts::ObservationRef;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use hepta_core::ApprovalRequirement;
use hepta_core::PolicyDecision;
use hepta_core::PolicyRule;
use hepta_core::RiskTier;

pub(super) fn stamp(name: &str, revision: u64) -> RevisionStamp {
    RevisionStamp::new(
        Revision::new(revision),
        ContentHash::new(format!("{name}-{revision}")),
    )
}

pub(super) fn context(state: u64, policy: u64, catalog: u64) -> FrozenTurnContext {
    let default_rules = rules(ApprovalRequirement::None);
    let policy_stamp =
        HeptaKernelPolicyEvidence::snapshot_for_rules(Revision::new(policy), &default_rules, &[])
            .expect("test policy snapshot");
    FrozenTurnContext::new(
        ObservationRef::new(
            ObservationId::new("tool-observation:session:tool"),
            Revision::new(1),
            ContentHash::new("observation-1"),
        ),
        stamp("state", state),
        policy_stamp,
        stamp("catalog", catalog),
        stamp("preference", 1),
    )
}

pub(super) fn candidate(context: FrozenTurnContext, action: &str, payload: &str) -> JointCandidate {
    candidate_for_requirement(context, action, payload, ApprovalRequirement::None)
}

pub(super) fn candidate_for_requirement(
    context: FrozenTurnContext,
    action: &str,
    payload: &str,
    requirement: ApprovalRequirement,
) -> JointCandidate {
    candidate_with_requirements(context, action, payload, requirement, requirement)
}

pub(super) fn candidate_with_requirements(
    context: FrozenTurnContext,
    action: &str,
    payload: &str,
    rule_requirement: ApprovalRequirement,
    presented_requirement: ApprovalRequirement,
) -> JointCandidate {
    let default_rules = rules(rule_requirement);
    let policy = HeptaKernelPolicyEvidence::snapshot_for_rules(
        context.policy().revision(),
        &default_rules,
        &[],
    )
    .expect("test policy snapshot");
    let context = FrozenTurnContext::new(
        context.observation().clone(),
        context.state().clone(),
        policy,
        context.capability_catalog().clone(),
        context.preference().clone(),
    );
    let descriptor = descriptor(&context);
    let payload_hash = ContentHash::new(payload);
    let evidence = evidence_with_requirements(
        context.policy().clone(),
        &payload_hash,
        descriptor.clone(),
        rule_requirement,
        presented_requirement,
        "kernel test policy",
    );
    let request = CapabilityRequestRef::new(
        CapabilityRequestId::new("request"),
        ContentHash::new("request-1"),
        descriptor.reference(),
        PrincipalId::new("model:test-provider/demo"),
        context.clone(),
        payload_hash.clone(),
    );
    JointCandidate::try_new(
        CandidateId::new("candidate"),
        Revision::new(1),
        ContentHash::new("same-declared-candidate-hash"),
        context,
        ContentHash::new(action),
        evidence
            .policy_decision_hash()
            .expect("policy decision is serializable")
            .clone(),
        payload_hash,
        vec![PrincipalId::new("model:test-provider/demo")],
        vec![request],
    )
    .expect("valid candidate")
}

pub(super) fn descriptor(context: &FrozenTurnContext) -> CapabilityDescriptor {
    CapabilityDescriptor::new(
        CapabilityId::new("tool:tool"),
        Revision::new(1),
        ContentHash::new("manifest-1"),
        context.capability_catalog().clone(),
        PrincipalId::new("provider"),
        "tool.invoke",
    )
}

pub(super) fn evidence(
    context: &FrozenTurnContext,
    payload: &ContentHash,
    descriptor: CapabilityDescriptor,
    requirement: ApprovalRequirement,
) -> HeptaKernelAdmissionEvidence {
    evidence_with_requirements(
        context.policy().clone(),
        payload,
        descriptor,
        requirement,
        requirement,
        "kernel test policy",
    )
}

pub(super) fn evidence_with_requirements(
    policy: RevisionStamp,
    payload: &ContentHash,
    descriptor: CapabilityDescriptor,
    rule_requirement: ApprovalRequirement,
    presented_requirement: ApprovalRequirement,
    reason: &str,
) -> HeptaKernelAdmissionEvidence {
    let default_rules = rules(rule_requirement);
    let policy = HeptaKernelPolicyEvidence::new(
        policy,
        "session",
        "test-provider",
        "tool",
        RiskTier::Low,
        default_rules,
        Vec::new(),
        PolicyDecision {
            requirement: presented_requirement,
            reason: reason.into(),
            matched_rule_id: Some("kernel-test".into()),
        },
    );
    HeptaKernelAdmissionEvidence::from_tool_policy(policy, descriptor, payload.clone())
}

pub(super) fn admission(
    gate: &HeptaKernelSafetyGate,
    candidate: &JointCandidate,
) -> HeptaKernelSafetyAdmission {
    admission_for(gate, candidate, ApprovalRequirement::None)
}

fn rules(requirement: ApprovalRequirement) -> Vec<PolicyRule> {
    vec![PolicyRule {
        id: "kernel-test".into(),
        session_id: None,
        provider_name: None,
        tool_name: None,
        risk_tier: None,
        requirement,
        reason: "kernel test policy".into(),
    }]
}

pub(super) fn admission_for(
    gate: &HeptaKernelSafetyGate,
    candidate: &JointCandidate,
    requirement: ApprovalRequirement,
) -> HeptaKernelSafetyAdmission {
    let evidence = evidence(
        candidate.context(),
        candidate.payload_set_hash(),
        descriptor(candidate.context()),
        requirement,
    );
    gate.admit_candidate(
        AdmissionId::new("admission"),
        Revision::new(1),
        candidate,
        PrincipalId::new("safety-gate"),
        &evidence,
    )
}

pub(super) fn admit_with_evidence(
    gate: &HeptaKernelSafetyGate,
    candidate: &JointCandidate,
    evidence: &HeptaKernelAdmissionEvidence,
) -> HeptaKernelSafetyAdmission {
    gate.admit_candidate(
        AdmissionId::new("admission"),
        Revision::new(1),
        candidate,
        PrincipalId::new("safety-gate"),
        evidence,
    )
}

pub(super) fn authorize(
    gate: &HeptaKernelSafetyGate,
    admission: &HeptaKernelSafetyAdmission,
    candidate: &JointCandidate,
    payload: &ContentHash,
    current_context: &FrozenTurnContext,
) -> Result<HeptaKernelSafetyAuthorization, HeptaKernelSafetyGateError> {
    gate.authorize_at_commit(
        AuthorizationId::new("authorization"),
        Revision::new(1),
        admission,
        admission.binding(),
        candidate,
        payload,
        current_context,
        PrincipalId::new("safety-gate"),
        ContentHash::new("scope-1"),
    )
}
