use hepta_contracts::Admission;
use hepta_contracts::AdmissionDecision;
use hepta_contracts::Authorization;
use hepta_contracts::AuthorizationDecision;
use hepta_contracts::AuthorizationId;
use hepta_contracts::ContentHash;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::JointCandidate;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use sha2::Digest;
use sha2::Sha256;
use std::error::Error;
use std::fmt;

const CANDIDATE_BINDING_DOMAIN: &str = "hepta.kernel.safety-gate.candidate-binding.v1";
const AUTHORIZATION_RECORD_DOMAIN: &str = "hepta.kernel.safety-gate.authorization-record.v1";

mod admission;
pub use admission::HeptaKernelAdmissionEvidence;
pub use admission::reason as admission_reason;
mod policy_evidence;
pub use policy_evidence::HeptaKernelPolicyEvidence;

/// Kernel-computed binding for one complete candidate and its payload set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelExactCandidateBinding {
    candidate_hash: ContentHash,
    payload_set_hash: ContentHash,
}

impl HeptaKernelExactCandidateBinding {
    /// Returns the kernel-computed digest of the complete candidate envelope.
    pub fn candidate_hash(&self) -> &ContentHash {
        &self.candidate_hash
    }

    /// Returns the exact ordered payload-set digest bound to the candidate.
    pub fn payload_set_hash(&self) -> &ContentHash {
        &self.payload_set_hash
    }
}

/// Admission contract paired with the stronger kernel-owned candidate binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelSafetyAdmission {
    admission: Admission,
    binding: HeptaKernelExactCandidateBinding,
    evidence_hash: ContentHash,
}

impl HeptaKernelSafetyAdmission {
    /// Returns the cross-layer admission contract.
    pub fn admission(&self) -> &Admission {
        &self.admission
    }

    /// Returns the exact binding that an approval must preserve.
    pub fn binding(&self) -> &HeptaKernelExactCandidateBinding {
        &self.binding
    }

    /// Returns the kernel digest of the complete admission evidence.
    pub fn evidence_hash(&self) -> &ContentHash {
        &self.evidence_hash
    }

    /// Returns whether the kernel independently admitted this candidate.
    pub fn is_admitted(&self) -> bool {
        matches!(self.admission.decision(), AdmissionDecision::Admitted)
    }

    /// Returns the stable rejection reason, if admission failed closed.
    pub fn rejection_reason_code(&self) -> Option<&str> {
        match self.admission.decision() {
            AdmissionDecision::Rejected { reason_code } => Some(reason_code),
            _ => None,
        }
    }

    /// Splits the wrapper into the contract and its exact kernel binding.
    pub fn into_parts(self) -> (Admission, HeptaKernelExactCandidateBinding) {
        (self.admission, self.binding)
    }
}

/// Kernel-minted witness for one exact commit-time authorization.
///
/// The contained contract is an audit DTO, while this wrapper proves that the
/// kernel safety gate checked the exact admitted binding and current context.
/// Its fields are private and it has no public constructor, so downstream
/// execution boundaries can require this witness instead of trusting a
/// caller-constructed [`Authorization`].
#[derive(Debug, PartialEq, Eq)]
pub struct HeptaKernelSafetyAuthorization {
    authorization: Authorization,
    binding: HeptaKernelExactCandidateBinding,
}

impl HeptaKernelSafetyAuthorization {
    /// Returns the commit-time authorization contract minted by the gate.
    pub fn authorization(&self) -> &Authorization {
        &self.authorization
    }

    /// Returns the exact candidate binding authorized at commit time.
    pub fn binding(&self) -> &HeptaKernelExactCandidateBinding {
        &self.binding
    }
}

/// Stable safety-gate failures that prevent commit authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HeptaKernelSafetyGateError {
    /// Commit was requested for an admission that was not admitted.
    AdmissionNotAdmitted,
    /// Approval evidence names a different exact candidate binding.
    ApprovedCandidateBindingMismatch,
    /// The candidate presented for execution differs from the admitted one.
    CandidateSubstitution,
    /// The payload presented for execution differs from the approved payload.
    PayloadSubstitution,
    /// The exact observation changed after admission.
    ObservationDrift,
    /// Runtime or application state changed after admission.
    StateDrift,
    /// Safety policy changed after admission.
    PolicyDrift,
    /// The capability catalog changed after admission.
    CapabilityCatalogDrift,
    /// Preference state changed after admission.
    PreferenceDrift,
}

impl HeptaKernelSafetyGateError {
    /// Returns a stable machine-readable failure code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::AdmissionNotAdmitted => "safety_gate.admission_not_admitted",
            Self::ApprovedCandidateBindingMismatch => {
                "safety_gate.approved_candidate_binding_mismatch"
            }
            Self::CandidateSubstitution => "safety_gate.candidate_substitution",
            Self::PayloadSubstitution => "safety_gate.payload_substitution",
            Self::ObservationDrift => "safety_gate.observation_drift",
            Self::StateDrift => "safety_gate.state_drift",
            Self::PolicyDrift => "safety_gate.policy_drift",
            Self::CapabilityCatalogDrift => "safety_gate.capability_catalog_drift",
            Self::PreferenceDrift => "safety_gate.preference_drift",
        }
    }
}

impl fmt::Display for HeptaKernelSafetyGateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl Error for HeptaKernelSafetyGateError {}

/// Pure kernel policy boundary for exact admission and commit-time reauthorization.
#[derive(Debug, Default, Clone, Copy)]
pub struct HeptaKernelSafetyGate;

impl HeptaKernelSafetyGate {
    /// Creates a stateless safety gate.
    pub const fn new() -> Self {
        Self
    }

    /// Computes a domain-separated digest over the complete candidate envelope.
    pub fn exact_candidate_binding(
        &self,
        candidate: &JointCandidate,
    ) -> HeptaKernelExactCandidateBinding {
        let mut hash = FramedHash::new(CANDIDATE_BINDING_DOMAIN);
        hash.text("candidate.id", candidate.id().as_str());
        hash.number("candidate.revision", candidate.revision().get());
        hash.text(
            "candidate.declared_content_hash",
            candidate.content_hash().as_str(),
        );
        hash_context(&mut hash, "candidate.context", candidate.context());
        hash.text("candidate.action_hash", candidate.action_hash().as_str());
        hash.text(
            "candidate.metacontrol_hash",
            candidate.metacontrol_hash().as_str(),
        );
        hash.text(
            "candidate.payload_set_hash",
            candidate.payload_set_hash().as_str(),
        );
        hash.number(
            "candidate.contributors.count",
            candidate.contributors().len() as u64,
        );
        for (index, contributor) in candidate.contributors().iter().enumerate() {
            hash.number("candidate.contributor.index", index as u64);
            hash.text("candidate.contributor.principal", contributor.as_str());
        }
        hash.number(
            "candidate.requests.count",
            candidate.capability_requests().len() as u64,
        );
        for (index, request) in candidate.capability_requests().iter().enumerate() {
            hash.number("candidate.request.index", index as u64);
            hash.text("candidate.request.id", request.id().as_str());
            hash.text(
                "candidate.request.request_hash",
                request.request_hash().as_str(),
            );
            hash.text("candidate.request.requester", request.requester().as_str());
            let capability = request.capability();
            hash.text("candidate.request.capability.id", capability.id().as_str());
            hash.number(
                "candidate.request.capability.revision",
                capability.revision().get(),
            );
            hash.text(
                "candidate.request.capability.manifest_hash",
                capability.manifest_hash().as_str(),
            );
            hash_stamp(
                &mut hash,
                "candidate.request.capability.catalog",
                capability.catalog(),
            );
            hash_context(&mut hash, "candidate.request.context", request.context());
            hash.text(
                "candidate.request.payload_hash",
                request.payload_hash().as_str(),
            );
        }
        HeptaKernelExactCandidateBinding {
            candidate_hash: hash.finish(),
            payload_set_hash: candidate.payload_set_hash().clone(),
        }
    }

    /// Reauthorizes an exact admitted candidate immediately before execution.
    #[allow(clippy::too_many_arguments)]
    pub fn authorize_at_commit(
        &self,
        id: AuthorizationId,
        revision: Revision,
        admitted: &HeptaKernelSafetyAdmission,
        approved_binding: &HeptaKernelExactCandidateBinding,
        candidate: &JointCandidate,
        presented_payload_set_hash: &ContentHash,
        current_context: &FrozenTurnContext,
        decided_by: PrincipalId,
        scope_hash: ContentHash,
    ) -> Result<HeptaKernelSafetyAuthorization, HeptaKernelSafetyGateError> {
        if !matches!(admitted.admission().decision(), AdmissionDecision::Admitted) {
            return Err(HeptaKernelSafetyGateError::AdmissionNotAdmitted);
        }
        if approved_binding != admitted.binding() {
            return Err(HeptaKernelSafetyGateError::ApprovedCandidateBindingMismatch);
        }
        if presented_payload_set_hash != approved_binding.payload_set_hash()
            || candidate.payload_set_hash() != approved_binding.payload_set_hash()
        {
            return Err(HeptaKernelSafetyGateError::PayloadSubstitution);
        }
        if self.exact_candidate_binding(candidate) != *approved_binding {
            return Err(HeptaKernelSafetyGateError::CandidateSubstitution);
        }
        ensure_context_unchanged(candidate.context(), current_context)?;

        let content_hash = hash_authorization_record(
            &id,
            revision,
            admitted,
            current_context,
            &decided_by,
            &scope_hash,
        );
        let authorization = Authorization::try_new_commit_time(
            id,
            revision,
            content_hash,
            admitted.admission(),
            current_context.clone(),
            decided_by,
            AuthorizationDecision::Authorized { scope_hash },
        )
        .map_err(|_| HeptaKernelSafetyGateError::AdmissionNotAdmitted)?;
        Ok(HeptaKernelSafetyAuthorization {
            authorization,
            binding: approved_binding.clone(),
        })
    }
}

fn ensure_context_unchanged(
    candidate: &FrozenTurnContext,
    current: &FrozenTurnContext,
) -> Result<(), HeptaKernelSafetyGateError> {
    if candidate.observation() != current.observation() {
        return Err(HeptaKernelSafetyGateError::ObservationDrift);
    }
    if candidate.state() != current.state() {
        return Err(HeptaKernelSafetyGateError::StateDrift);
    }
    if candidate.policy() != current.policy() {
        return Err(HeptaKernelSafetyGateError::PolicyDrift);
    }
    if candidate.capability_catalog() != current.capability_catalog() {
        return Err(HeptaKernelSafetyGateError::CapabilityCatalogDrift);
    }
    if candidate.preference() != current.preference() {
        return Err(HeptaKernelSafetyGateError::PreferenceDrift);
    }
    Ok(())
}

fn hash_authorization_record(
    id: &AuthorizationId,
    revision: Revision,
    admitted: &HeptaKernelSafetyAdmission,
    current_context: &FrozenTurnContext,
    decided_by: &PrincipalId,
    scope_hash: &ContentHash,
) -> ContentHash {
    let mut hash = FramedHash::new(AUTHORIZATION_RECORD_DOMAIN);
    hash.text("authorization.id", id.as_str());
    hash.number("authorization.revision", revision.get());
    hash.text(
        "authorization.admission.content_hash",
        admitted.admission().content_hash().as_str(),
    );
    hash.text(
        "authorization.candidate_binding",
        admitted.binding().candidate_hash().as_str(),
    );
    hash.text(
        "authorization.payload_set_hash",
        admitted.binding().payload_set_hash().as_str(),
    );
    hash_context(&mut hash, "authorization.current_context", current_context);
    hash.text("authorization.decided_by", decided_by.as_str());
    hash.text("authorization.scope_hash", scope_hash.as_str());
    hash.finish()
}

fn hash_context(hash: &mut FramedHash, prefix: &str, context: &FrozenTurnContext) {
    hash.text(
        &format!("{prefix}.observation.id"),
        context.observation().id().as_str(),
    );
    hash.number(
        &format!("{prefix}.observation.revision"),
        context.observation().revision().get(),
    );
    hash.text(
        &format!("{prefix}.observation.content_hash"),
        context.observation().content_hash().as_str(),
    );
    hash_stamp(hash, &format!("{prefix}.state"), context.state());
    hash_stamp(hash, &format!("{prefix}.policy"), context.policy());
    hash_stamp(
        hash,
        &format!("{prefix}.capability_catalog"),
        context.capability_catalog(),
    );
    hash_stamp(hash, &format!("{prefix}.preference"), context.preference());
}

fn hash_stamp(hash: &mut FramedHash, prefix: &str, stamp: &RevisionStamp) {
    hash.number(&format!("{prefix}.revision"), stamp.revision().get());
    hash.text(
        &format!("{prefix}.content_hash"),
        stamp.content_hash().as_str(),
    );
}

struct FramedHash(Sha256);

impl FramedHash {
    fn new(domain: &str) -> Self {
        let mut value = Self(Sha256::new());
        value.bytes("domain", domain.as_bytes());
        value
    }

    fn text(&mut self, field: &str, value: &str) {
        self.bytes(field, value.as_bytes());
    }

    fn number(&mut self, field: &str, value: u64) {
        self.bytes(field, &value.to_be_bytes());
    }

    fn bytes(&mut self, field: &str, value: &[u8]) {
        self.0.update((field.len() as u64).to_be_bytes());
        self.0.update(field.as_bytes());
        self.0.update((value.len() as u64).to_be_bytes());
        self.0.update(value);
    }

    fn finish(self) -> ContentHash {
        ContentHash::new(format!("sha256:{:x}", self.0.finalize()))
    }
}

#[cfg(test)]
#[path = "safety_gate/test_support.rs"]
mod test_support;

#[cfg(test)]
#[path = "safety_gate/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "safety_gate/admission_tests.rs"]
mod admission_tests;
