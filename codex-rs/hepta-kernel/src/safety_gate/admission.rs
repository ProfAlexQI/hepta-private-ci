use super::FramedHash;
use super::HeptaKernelExactCandidateBinding;
use super::HeptaKernelPolicyEvidence;
use super::HeptaKernelSafetyAdmission;
use super::HeptaKernelSafetyGate;
use super::policy_evidence::PolicyEvidenceFailure;
use hepta_contracts::Admission;
use hepta_contracts::AdmissionDecision;
use hepta_contracts::AdmissionId;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::ContentHash;
use hepta_contracts::JointCandidate;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;
use hepta_core::ApprovalRequirement;

const ADMISSION_EVIDENCE_DOMAIN: &str = "hepta.kernel.safety-gate.admission-evidence.v1";
const ADMISSION_RECORD_DOMAIN: &str = "hepta.kernel.safety-gate.admission-record.v2";

/// Stable fail-closed reason codes produced by kernel admission.
pub mod reason {
    pub const POLICY_SNAPSHOT_MISMATCH: &str = "admission.policy_snapshot_mismatch";
    pub const POLICY_EVIDENCE_MALFORMED: &str = "admission.policy_evidence_malformed";
    pub const POLICY_EVALUATION_CONTEXT_MISMATCH: &str =
        "admission.policy_evaluation_context_mismatch";
    pub const POLICY_DECISION_MISMATCH: &str = "admission.policy_decision_mismatch";
    pub const POLICY_DENIED: &str = "admission.policy_denied";
    pub const CAPABILITY_DESCRIPTOR_MALFORMED: &str = "admission.capability_descriptor_malformed";
    pub const CAPABILITY_CATALOG_MISMATCH: &str = "admission.capability_catalog_mismatch";
    pub const CAPABILITY_REQUEST_COUNT: &str = "admission.capability_request_count";
    pub const CAPABILITY_REQUEST_MALFORMED: &str = "admission.capability_request_malformed";
    pub const CAPABILITY_REQUEST_CONTEXT_MISMATCH: &str =
        "admission.capability_request_context_mismatch";
    pub const CAPABILITY_REQUEST_DESCRIPTOR_MISMATCH: &str =
        "admission.capability_request_descriptor_mismatch";
    pub const CAPABILITY_REQUEST_PAYLOAD_MISMATCH: &str =
        "admission.capability_request_payload_mismatch";
    pub const PAYLOAD_SET_MISMATCH: &str = "admission.payload_set_mismatch";
}

/// Complete typed evidence consumed by the kernel for one exact tool candidate.
///
/// The policy decision is bound to its frozen policy snapshot, while the
/// canonical capability descriptor and actual payload allow the kernel to
/// validate the candidate request rather than trusting a caller-owned allow
/// boolean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelAdmissionEvidence {
    policy: HeptaKernelPolicyEvidence,
    capability: CapabilityDescriptor,
    payload_hash: ContentHash,
}

impl HeptaKernelAdmissionEvidence {
    /// Freezes the complete policy result and exact capability/payload evidence.
    pub fn from_tool_policy(
        policy: HeptaKernelPolicyEvidence,
        capability: CapabilityDescriptor,
        payload_hash: ContentHash,
    ) -> Self {
        Self {
            policy,
            capability,
            payload_hash,
        }
    }

    /// Returns the complete policy snapshot and evaluation evidence.
    pub fn policy(&self) -> &HeptaKernelPolicyEvidence {
        &self.policy
    }

    /// Returns the canonical policy-decision digest, if serialization succeeded.
    pub fn policy_decision_hash(&self) -> Option<&ContentHash> {
        self.policy.decision_hash()
    }

    /// Returns the exact capability descriptor evaluated for feasibility.
    pub fn capability(&self) -> &CapabilityDescriptor {
        &self.capability
    }

    /// Returns the actual payload digest evaluated for feasibility.
    pub fn payload_hash(&self) -> &ContentHash {
        &self.payload_hash
    }
}

impl HeptaKernelSafetyGate {
    /// Independently derives admission from complete typed candidate evidence.
    pub fn admit_candidate(
        &self,
        id: AdmissionId,
        revision: Revision,
        candidate: &JointCandidate,
        decided_by: PrincipalId,
        evidence: &HeptaKernelAdmissionEvidence,
    ) -> HeptaKernelSafetyAdmission {
        let binding = self.exact_candidate_binding(candidate);
        let decision = assess_candidate(candidate, evidence).map_or(
            AdmissionDecision::Admitted,
            |reason_code| AdmissionDecision::Rejected {
                reason_code: reason_code.to_string(),
            },
        );
        let evidence_hash = hash_admission_evidence(evidence);
        let content_hash = hash_admission_record(
            &id,
            revision,
            &binding,
            &evidence_hash,
            &decided_by,
            &decision,
        );
        let admission = Admission::new(id, revision, content_hash, candidate, decided_by, decision);
        HeptaKernelSafetyAdmission {
            admission,
            binding,
            evidence_hash,
        }
    }
}

fn assess_candidate(
    candidate: &JointCandidate,
    evidence: &HeptaKernelAdmissionEvidence,
) -> Option<&'static str> {
    let requirement = match evidence.policy.assess(candidate, &evidence.capability) {
        Ok(requirement) => requirement,
        Err(PolicyEvidenceFailure::SnapshotMismatch) => {
            return Some(reason::POLICY_SNAPSHOT_MISMATCH);
        }
        Err(PolicyEvidenceFailure::Malformed) => {
            return Some(reason::POLICY_EVIDENCE_MALFORMED);
        }
        Err(PolicyEvidenceFailure::EvaluationContextMismatch) => {
            return Some(reason::POLICY_EVALUATION_CONTEXT_MISMATCH);
        }
        Err(PolicyEvidenceFailure::DecisionMismatch) => {
            return Some(reason::POLICY_DECISION_MISMATCH);
        }
    };
    if requirement == ApprovalRequirement::Deny {
        return Some(reason::POLICY_DENIED);
    }
    if descriptor_malformed(evidence.capability()) {
        return Some(reason::CAPABILITY_DESCRIPTOR_MALFORMED);
    }
    if evidence.capability().catalog() != candidate.context().capability_catalog() {
        return Some(reason::CAPABILITY_CATALOG_MISMATCH);
    }
    let [request] = candidate.capability_requests() else {
        return Some(reason::CAPABILITY_REQUEST_COUNT);
    };
    if request.id().as_str().trim().is_empty()
        || request.request_hash().as_str().trim().is_empty()
        || request.requester().as_str().trim().is_empty()
        || request.payload_hash().as_str().trim().is_empty()
        || !candidate
            .contributors()
            .iter()
            .any(|contributor| contributor == request.requester())
    {
        return Some(reason::CAPABILITY_REQUEST_MALFORMED);
    }
    if request.context() != candidate.context() {
        return Some(reason::CAPABILITY_REQUEST_CONTEXT_MISMATCH);
    }
    if request.capability() != &evidence.capability().reference() {
        return Some(reason::CAPABILITY_REQUEST_DESCRIPTOR_MISMATCH);
    }
    if request.payload_hash() != evidence.payload_hash() {
        return Some(reason::CAPABILITY_REQUEST_PAYLOAD_MISMATCH);
    }
    if candidate.payload_set_hash() != evidence.payload_hash() {
        return Some(reason::PAYLOAD_SET_MISMATCH);
    }
    None
}

fn descriptor_malformed(descriptor: &CapabilityDescriptor) -> bool {
    descriptor.id().as_str().trim().is_empty()
        || descriptor.manifest_hash().as_str().trim().is_empty()
        || descriptor.provider().as_str().trim().is_empty()
        || descriptor.operation().trim().is_empty()
}

fn hash_admission_evidence(evidence: &HeptaKernelAdmissionEvidence) -> ContentHash {
    let mut hash = FramedHash::new(ADMISSION_EVIDENCE_DOMAIN);
    hash.text(
        "policy.evidence_hash",
        evidence.policy.evidence_hash().as_str(),
    );
    let capability = &evidence.capability;
    hash.text("capability.id", capability.id().as_str());
    hash.number("capability.revision", capability.revision().get());
    hash.text(
        "capability.manifest_hash",
        capability.manifest_hash().as_str(),
    );
    hash.number(
        "capability.catalog.revision",
        capability.catalog().revision().get(),
    );
    hash.text(
        "capability.catalog.content_hash",
        capability.catalog().content_hash().as_str(),
    );
    hash.text("capability.provider", capability.provider().as_str());
    hash.text("capability.operation", capability.operation());
    hash.text("payload_hash", evidence.payload_hash.as_str());
    hash.finish()
}

fn hash_admission_record(
    id: &AdmissionId,
    revision: Revision,
    binding: &HeptaKernelExactCandidateBinding,
    evidence_hash: &ContentHash,
    decided_by: &PrincipalId,
    decision: &AdmissionDecision,
) -> ContentHash {
    let mut hash = FramedHash::new(ADMISSION_RECORD_DOMAIN);
    hash.text("admission.id", id.as_str());
    hash.number("admission.revision", revision.get());
    hash.text(
        "admission.candidate_binding",
        binding.candidate_hash().as_str(),
    );
    hash.text(
        "admission.payload_set_hash",
        binding.payload_set_hash().as_str(),
    );
    hash.text("admission.evidence_hash", evidence_hash.as_str());
    hash.text("admission.decided_by", decided_by.as_str());
    match decision {
        AdmissionDecision::Admitted => hash.text("admission.decision", "admitted"),
        AdmissionDecision::Rejected { reason_code } => {
            hash.text("admission.decision", "rejected");
            hash.text("admission.reason_code", reason_code);
        }
        _ => hash.text("admission.decision", "unknown"),
    }
    hash.finish()
}
