use super::FullMatrixVerdictV1;
use super::GateContractV1;
use super::GateEvidenceV1;
use super::RequiredGateObservationV1;
use super::ScopeVerdictV1;
use super::ScopedQualificationAssessmentV1;
use super::ScopedQualificationInputV1;
use super::profiles;
use crate::AcceptanceError;
use crate::manifest_inventory::digest_shape;

pub fn assess(
    input: &ScopedQualificationInputV1,
) -> Result<ScopedQualificationAssessmentV1, AcceptanceError> {
    profiles::validate_contract(&input.contract)?;
    if input.evidence.len() != profiles::LISTED_GATE_COUNT {
        return Err(invalid(
            "scope evidence must list all five gates in canonical order",
        ));
    }

    let mut blockers = Vec::new();
    let mut required_pass_count = 0;
    for (evidence, contract) in input.evidence.iter().zip(&input.contract.gates) {
        match (evidence, contract) {
            (
                GateEvidenceV1::Required {
                    gate,
                    observation,
                    profile,
                },
                GateContractV1::Required {
                    gate: expected_gate,
                    profile: expected_profile,
                },
            ) if gate == expected_gate && profile == expected_profile => match observation {
                RequiredGateObservationV1::Pass { receipt } => {
                    validate_receipt_pin(&receipt.sha256)?;
                    required_pass_count += 1;
                }
                RequiredGateObservationV1::Fail { receipt } => {
                    validate_receipt_pin(&receipt.sha256)?;
                    blockers.push(format!("gate:{}:FAIL", gate.as_str()));
                }
                RequiredGateObservationV1::Missing => {
                    blockers.push(format!("gate:{}:MISSING", gate.as_str()));
                }
            },
            (
                GateEvidenceV1::Deferred { gate, profile },
                GateContractV1::Deferred {
                    gate: expected_gate,
                    profile: expected_profile,
                    ..
                },
            ) if gate == expected_gate && profile == expected_profile => {}
            _ => {
                return Err(invalid(
                    "scope evidence differs from the compiled disposition, gate order, or profile",
                ));
            }
        }
    }

    let scope_verdict = if blockers.is_empty() {
        ScopeVerdictV1::Pass
    } else {
        ScopeVerdictV1::Blocked
    };
    Ok(ScopedQualificationAssessmentV1 {
        blockers,
        candidate: profiles::exact_candidate(),
        deferred_gate_count: profiles::DEFERRED_GATE_COUNT,
        full_matrix_verdict: FullMatrixVerdictV1::NotClaimed,
        listed_gate_count: profiles::LISTED_GATE_COUNT,
        ready_for_scoped_challenge: scope_verdict == ScopeVerdictV1::Pass,
        required_gate_count: profiles::REQUIRED_GATE_COUNT,
        required_pass_count,
        schema: profiles::ASSESSMENT_SCHEMA.to_string(),
        scope_verdict,
    })
}

fn validate_receipt_pin(digest: &str) -> Result<(), AcceptanceError> {
    if !digest_shape(digest) {
        return Err(invalid("required receipt manifest pin is malformed"));
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}
