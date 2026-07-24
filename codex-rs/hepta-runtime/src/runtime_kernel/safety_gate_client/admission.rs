use crate::HeptaError;
use crate::ModelRef;
use crate::PolicyDecision;
use crate::RuntimeKernel;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::ContentHash;
use hepta_contracts::JointCandidate;
use hepta_contracts::PrincipalId;
use hepta_kernel::HeptaKernelAdmissionEvidence;
use hepta_kernel::HeptaKernelPolicyEvidence;
use hepta_kernel::HeptaKernelSafetyAdmission;
use hepta_kernel::HeptaKernelSafetyGate;

#[allow(clippy::too_many_arguments)]
pub(super) fn admit_exact_tool_candidate(
    runtime: &RuntimeKernel,
    session_id: &str,
    active_model: &ModelRef,
    tool_name: &str,
    decision: &PolicyDecision,
    candidate: &JointCandidate,
    capability: CapabilityDescriptor,
    payload_hash: ContentHash,
) -> Result<HeptaKernelSafetyAdmission, HeptaError> {
    let risk_tier = runtime.tools.risk_tier(tool_name)?;
    let custom_rules = runtime
        .policy
        .custom_rules()
        .map_err(|error| HeptaError(error.0))?;
    let policy = HeptaKernelPolicyEvidence::new(
        candidate.context().policy().clone(),
        session_id,
        active_model.provider.clone(),
        tool_name,
        risk_tier,
        runtime.policy.default_rules(),
        custom_rules,
        decision.clone(),
    );
    let evidence = HeptaKernelAdmissionEvidence::from_tool_policy(policy, capability, payload_hash);
    let gate = HeptaKernelSafetyGate::new();
    let binding = gate.exact_candidate_binding(candidate);
    let admission = gate.admit_candidate(
        hepta_contracts::AdmissionId::new(format!(
            "admission:{}",
            binding.candidate_hash().as_str()
        )),
        candidate.revision(),
        candidate,
        PrincipalId::new("kernel:safety-gate"),
        &evidence,
    );
    if let Some(reason_code) = admission.rejection_reason_code() {
        return Err(HeptaError(format!(
            "kernel candidate admission rejected: {reason_code}"
        )));
    }
    Ok(admission)
}
