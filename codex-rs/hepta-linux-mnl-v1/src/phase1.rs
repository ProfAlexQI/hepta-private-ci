use serde::Serialize;

use crate::CompositeIdentityV1;
use crate::LinuxMnlError;
use crate::RepositoryIdentityV1;
use crate::blocked;
use crate::canonical::sha256;
use crate::canonical::validate_digest;
use crate::canonical_json;
use crate::canonical_sha256;
use crate::exact_composite_identity;
use crate::identity::VerifiedSuccessorToolingIdentityV1;
use crate::identity::missing_successor_tooling_pins;
use crate::identity::required_compiled_successor_tooling_identity;
use crate::identity::validate_successor_tooling_identity;
use crate::invalid;
use crate::profiles::TargetRoleV1;
use crate::profiles::VerifiedPublishedProfilesV1;
use crate::profiles::compiled_profile_status;
use crate::profiles::required_compiled_published_profiles;
use crate::validate_composite_identity;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityLayerV1 {
    ObservationOnly,
    TargetQualification,
    LinuxScopedGate,
    MnlScopedAggregate,
    OperatorCeremony,
    RefOrStateTransition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptKindV1 {
    Phase1Observation,
    TargetQualification,
    LinuxScopedGate,
    MnlScopedAggregate,
    OperatorCeremony,
    RefOrStateTransition,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceiptTopologyNodeV1 {
    authority_layer: AuthorityLayerV1,
    kind: ReceiptKindV1,
    predecessor: Option<ReceiptKindV1>,
    phase1_may_mint: bool,
}

impl ReceiptTopologyNodeV1 {
    pub fn phase1_may_mint(&self) -> bool {
        self.phase1_may_mint
    }
}

pub fn exact_receipt_topology() -> Vec<ReceiptTopologyNodeV1> {
    use AuthorityLayerV1 as Layer;
    use ReceiptKindV1 as Kind;

    vec![
        ReceiptTopologyNodeV1 {
            authority_layer: Layer::ObservationOnly,
            kind: Kind::Phase1Observation,
            predecessor: None,
            phase1_may_mint: true,
        },
        ReceiptTopologyNodeV1 {
            authority_layer: Layer::TargetQualification,
            kind: Kind::TargetQualification,
            predecessor: Some(Kind::Phase1Observation),
            phase1_may_mint: false,
        },
        ReceiptTopologyNodeV1 {
            authority_layer: Layer::LinuxScopedGate,
            kind: Kind::LinuxScopedGate,
            predecessor: Some(Kind::TargetQualification),
            phase1_may_mint: false,
        },
        ReceiptTopologyNodeV1 {
            authority_layer: Layer::MnlScopedAggregate,
            kind: Kind::MnlScopedAggregate,
            predecessor: Some(Kind::LinuxScopedGate),
            phase1_may_mint: false,
        },
        ReceiptTopologyNodeV1 {
            authority_layer: Layer::OperatorCeremony,
            kind: Kind::OperatorCeremony,
            predecessor: Some(Kind::MnlScopedAggregate),
            phase1_may_mint: false,
        },
        ReceiptTopologyNodeV1 {
            authority_layer: Layer::RefOrStateTransition,
            kind: Kind::RefOrStateTransition,
            predecessor: Some(Kind::OperatorCeremony),
            phase1_may_mint: false,
        },
    ]
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityBoundaryV1 {
    pub automatic_transition: bool,
    pub canary: bool,
    pub credential_use: bool,
    pub cutover: bool,
    pub default_ref_change: bool,
    pub deletion: bool,
    pub enforce: bool,
    pub filesystem_mutation: bool,
    pub full_matrix_claim: bool,
    pub ga_claim: bool,
    pub global_authority: bool,
    pub install_activation: bool,
    pub linux_gate_pass: bool,
    pub local_ref_change: bool,
    pub operator_acceptance: bool,
    pub outbound: bool,
    pub process_execution: bool,
    pub production: bool,
    pub promotion: bool,
    pub qualification_authority: bool,
    pub receipt_signing: bool,
    pub recutover: bool,
    pub remote_ref_change: bool,
    pub retirement: bool,
    pub rollback: bool,
    pub service_control: bool,
    pub snapshot: bool,
    pub state_root_mutation: bool,
    pub target_qualification_pass: bool,
    pub watermark_state_mutation: bool,
    pub writer_control: bool,
}

impl AuthorityBoundaryV1 {
    pub const fn closed() -> Self {
        Self {
            automatic_transition: false,
            canary: false,
            credential_use: false,
            cutover: false,
            default_ref_change: false,
            deletion: false,
            enforce: false,
            filesystem_mutation: false,
            full_matrix_claim: false,
            ga_claim: false,
            global_authority: false,
            install_activation: false,
            linux_gate_pass: false,
            local_ref_change: false,
            operator_acceptance: false,
            outbound: false,
            process_execution: false,
            production: false,
            promotion: false,
            qualification_authority: false,
            receipt_signing: false,
            recutover: false,
            remote_ref_change: false,
            retirement: false,
            rollback: false,
            service_control: false,
            snapshot: false,
            state_root_mutation: false,
            target_qualification_pass: false,
            watermark_state_mutation: false,
            writer_control: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectorEnvironmentV1 {
    ProductionLinuxMnlTarget,
    X230QualificationFixture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationKindV1 {
    CompositeIdentity,
    SuccessorFinalToolingIdentity,
    CollectorSourceIdentity,
    CollectorBinaryIdentity,
    PublishedTrustProfileSet,
    InstallEpochCompletion,
    StateRootProfile,
    ExternalWatermarkProviderProfile,
    ExternalWatermarkCurrentTip,
    TargetProfile,
    StateRootMetadataReadOnly,
    MachineIdentityReadOnly,
    WorkloadIdentityReadOnly,
}

impl ObservationKindV1 {
    const fn all() -> [Self; 13] {
        [
            Self::CompositeIdentity,
            Self::SuccessorFinalToolingIdentity,
            Self::CollectorSourceIdentity,
            Self::CollectorBinaryIdentity,
            Self::PublishedTrustProfileSet,
            Self::InstallEpochCompletion,
            Self::StateRootProfile,
            Self::ExternalWatermarkProviderProfile,
            Self::ExternalWatermarkCurrentTip,
            Self::TargetProfile,
            Self::StateRootMetadataReadOnly,
            Self::MachineIdentityReadOnly,
            Self::WorkloadIdentityReadOnly,
        ]
    }
}

/// Opaque verified plan. It deliberately has no `Deserialize` implementation
/// and all fields are private, so only the production verifier in this crate
/// can construct one for downstream use.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phase1CollectorPlanV1 {
    authority: AuthorityBoundaryV1,
    challenge_nonce_sha256: String,
    collector_binary_sha256: String,
    collector_environment: CollectorEnvironmentV1,
    collector_source_sha256: String,
    composite_identity: CompositeIdentityV1,
    observation_order: Vec<ObservationKindV1>,
    profile_documents_sha256: String,
    receipt_topology: Vec<ReceiptTopologyNodeV1>,
    schema: String,
    schema_version: u32,
    successor_final_tooling: RepositoryIdentityV1,
    target_host_alias: String,
    target_profile_sha256: String,
    target_role: TargetRoleV1,
}

impl Phase1CollectorPlanV1 {
    pub fn authority(&self) -> &AuthorityBoundaryV1 {
        &self.authority
    }

    pub fn challenge_nonce_sha256(&self) -> &str {
        &self.challenge_nonce_sha256
    }

    pub fn collector_environment(&self) -> CollectorEnvironmentV1 {
        self.collector_environment
    }

    pub fn composite_identity(&self) -> &CompositeIdentityV1 {
        &self.composite_identity
    }

    pub fn observation_order(&self) -> &[ObservationKindV1] {
        &self.observation_order
    }

    pub fn receipt_topology(&self) -> &[ReceiptTopologyNodeV1] {
        &self.receipt_topology
    }

    pub fn successor_final_tooling(&self) -> &RepositoryIdentityV1 {
        &self.successor_final_tooling
    }

    pub fn target_host_alias(&self) -> &str {
        &self.target_host_alias
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompiledPhase1StatusV1 {
    pub blocked: bool,
    pub missing_requirements: Vec<String>,
    pub production_plan_available: bool,
    pub schema: String,
}

pub fn compiled_phase1_status() -> CompiledPhase1StatusV1 {
    let profile_status = compiled_profile_status();
    let mut missing_requirements = profile_status
        .missing_pins
        .into_iter()
        .map(|pin| format!("profile:{pin}"))
        .collect::<Vec<_>>();
    missing_requirements.extend(
        missing_successor_tooling_pins()
            .into_iter()
            .map(|pin| format!("tooling:{pin}")),
    );
    // This first slice intentionally contains no entropy source. A later,
    // independently reviewed collector must generate a fresh challenge in
    // process; a reusable compiled nonce is explicitly forbidden.
    missing_requirements.push("collector:internal_fresh_challenge_nonce_generator".to_string());
    CompiledPhase1StatusV1 {
        blocked: !missing_requirements.is_empty(),
        production_plan_available: missing_requirements.is_empty(),
        missing_requirements,
        schema: "hepta_linux_mnl_compiled_phase1_status_v1".to_string(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InternalChallengeNonceV1 {
    sha256: String,
}

/// The production entrypoint accepts no caller-selected roots, paths, pins,
/// targets, profile documents, tooling identity, collector identity, or
/// challenge. It remains hard blocked until all source pins are reviewed and
/// a fresh internal challenge generator is implemented.
pub fn production_phase1_plan() -> Result<Phase1CollectorPlanV1, LinuxMnlError> {
    let status = compiled_phase1_status();
    if status.blocked {
        return Err(blocked(format!(
            "production Phase 1 requirements are absent: {}",
            status.missing_requirements.join(",")
        )));
    }
    let verified_profiles = required_compiled_published_profiles()?;
    let verified_tooling = required_compiled_successor_tooling_identity()?;
    let challenge = production_internal_challenge_nonce()?;
    build_phase1_plan(verified_profiles, verified_tooling, challenge)
}

fn production_internal_challenge_nonce() -> Result<InternalChallengeNonceV1, LinuxMnlError> {
    Err(blocked(
        "fresh internal challenge nonce generator is not implemented",
    ))
}

fn build_phase1_plan(
    verified: VerifiedPublishedProfilesV1,
    tooling: VerifiedSuccessorToolingIdentityV1,
    challenge: InternalChallengeNonceV1,
) -> Result<Phase1CollectorPlanV1, LinuxMnlError> {
    let plan = Phase1CollectorPlanV1 {
        authority: AuthorityBoundaryV1::closed(),
        challenge_nonce_sha256: challenge.sha256,
        collector_binary_sha256: tooling.collector_binary_sha256,
        collector_environment: CollectorEnvironmentV1::ProductionLinuxMnlTarget,
        collector_source_sha256: tooling.collector_source_sha256,
        composite_identity: exact_composite_identity(),
        observation_order: ObservationKindV1::all().to_vec(),
        profile_documents_sha256: verified.documents_sha256().to_string(),
        receipt_topology: exact_receipt_topology(),
        schema: "hepta_linux_mnl_phase1_collector_plan_v1".to_string(),
        schema_version: 1,
        successor_final_tooling: tooling.successor_final_tooling,
        target_host_alias: verified.target_host_alias().to_string(),
        target_profile_sha256: verified.target_profile_sha256().to_string(),
        target_role: TargetRoleV1::ProductionLinuxMnlTarget,
    };
    validate_plan(&plan)?;
    Ok(plan)
}

fn validate_plan(plan: &Phase1CollectorPlanV1) -> Result<(), LinuxMnlError> {
    if plan.schema != "hepta_linux_mnl_phase1_collector_plan_v1"
        || plan.schema_version != 1
        || plan.authority != AuthorityBoundaryV1::closed()
        || plan.collector_environment != CollectorEnvironmentV1::ProductionLinuxMnlTarget
        || plan.observation_order != ObservationKindV1::all()
        || plan.receipt_topology != exact_receipt_topology()
        || plan.target_role != TargetRoleV1::ProductionLinuxMnlTarget
        || plan.target_host_alias != crate::profiles::PRODUCTION_TARGET_ALIAS_V1
    {
        return Err(invalid(
            "phase1 plan escapes the exact opaque read-only production-target boundary",
        ));
    }
    validate_composite_identity(&plan.composite_identity)?;
    validate_successor_tooling_identity(
        &plan.successor_final_tooling,
        &plan.collector_source_sha256,
        &plan.collector_binary_sha256,
    )?;
    validate_digest("internal challenge", &plan.challenge_nonce_sha256)?;
    validate_digest("profile document set", &plan.profile_documents_sha256)?;
    validate_digest("target profile", &plan.target_profile_sha256)?;
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)] // Constructed by the next collector slice; exercised here by unit tests.
enum ObservationStatusV1 {
    ExactMatch,
    Missing,
    Mismatch,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)] // Constructed by the next collector slice; exercised here by unit tests.
struct ObservationResultV1 {
    evidence_sha256: Option<String>,
    kind: ObservationKindV1,
    observed_bytes: Option<u64>,
    status: ObservationStatusV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)] // Constructed by the next collector slice; exercised here by unit tests.
struct ObservationReceiptV1 {
    canonical_payload_bytes: u64,
    canonical_payload_sha256: String,
    receipt_manifest_sha256: String,
    schema: String,
    signature_verified: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)] // Constructed by the next collector slice; exercised here by unit tests.
enum Phase1VerdictV1 {
    BlockedNoAuthority,
    StructurallyCompleteNoAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)] // Constructed by the next collector slice; exercised here by unit tests.
pub(crate) struct Phase1CollectorResultV1 {
    authority: AuthorityBoundaryV1,
    challenge_nonce_sha256: String,
    claimed_target_qualification_pass: bool,
    collector_binary_sha256: String,
    collector_environment: CollectorEnvironmentV1,
    collector_source_sha256: String,
    composite_identity: CompositeIdentityV1,
    linux_gate_receipt_sha256: Option<String>,
    observations: Vec<ObservationResultV1>,
    operator_ceremony_receipt_sha256: Option<String>,
    phase1_observation_receipt: ObservationReceiptV1,
    plan_sha256: String,
    receipt_topology: Vec<ReceiptTopologyNodeV1>,
    schema: String,
    schema_version: u32,
    scoped_aggregate_receipt_sha256: Option<String>,
    successor_final_tooling: RepositoryIdentityV1,
    target_host_alias: String,
    target_profile_sha256: String,
    target_qualification_receipt_sha256: Option<String>,
    verdict: Phase1VerdictV1,
}

/// Internal validation only. Raw result validation is intentionally not a
/// public API; a future collector must construct results inside this crate
/// from an opaque verified plan.
#[allow(dead_code)] // Called by the next collector slice; exercised here by unit tests.
pub(crate) fn validate_phase1_result(
    plan: &Phase1CollectorPlanV1,
    result: &Phase1CollectorResultV1,
) -> Result<(), LinuxMnlError> {
    validate_plan(plan)?;
    if result.schema != "hepta_linux_mnl_phase1_collector_result_v1"
        || result.schema_version != 1
        || result.authority != AuthorityBoundaryV1::closed()
        || result.claimed_target_qualification_pass
        || result.collector_environment != CollectorEnvironmentV1::ProductionLinuxMnlTarget
        || result.collector_environment != plan.collector_environment
        || result.receipt_topology != exact_receipt_topology()
        || result.receipt_topology != plan.receipt_topology
        || result.target_host_alias != plan.target_host_alias
        || result.target_profile_sha256 != plan.target_profile_sha256
        || result.challenge_nonce_sha256 != plan.challenge_nonce_sha256
        || result.collector_source_sha256 != plan.collector_source_sha256
        || result.collector_binary_sha256 != plan.collector_binary_sha256
        || result.successor_final_tooling != plan.successor_final_tooling
        || result.target_qualification_receipt_sha256.is_some()
        || result.linux_gate_receipt_sha256.is_some()
        || result.scoped_aggregate_receipt_sha256.is_some()
        || result.operator_ceremony_receipt_sha256.is_some()
    {
        return Err(invalid(
            "phase1 result leaks target, gate, aggregate, ceremony, or global authority",
        ));
    }
    validate_composite_identity(&result.composite_identity)?;
    if result.composite_identity != plan.composite_identity {
        return Err(invalid("phase1 result changed the composite identity"));
    }
    if result.plan_sha256 != canonical_sha256(plan)? {
        return Err(invalid(
            "phase1 result is not bound to the exact opaque plan bytes",
        ));
    }
    validate_observations(&result.observations)?;
    let expected_receipt = expected_observation_receipt(plan, &result.observations)?;
    if result.phase1_observation_receipt != expected_receipt {
        return Err(invalid(
            "phase1 observation receipt is not a deterministic binding of plan, identity, target, challenge, and ordered observations",
        ));
    }

    let all_exact = result
        .observations
        .iter()
        .all(|observation| observation.status == ObservationStatusV1::ExactMatch);
    let expected_verdict = if all_exact {
        Phase1VerdictV1::StructurallyCompleteNoAuthority
    } else {
        Phase1VerdictV1::BlockedNoAuthority
    };
    if result.verdict != expected_verdict {
        return Err(invalid(
            "phase1 structural verdict does not match observations or attempts to encode PASS",
        ));
    }
    Ok(())
}

#[derive(Serialize)]
#[allow(dead_code)] // Constructed by the next collector slice; exercised here by unit tests.
struct ObservationPayloadV1<'a> {
    challenge_nonce_sha256: &'a str,
    collector_binary_sha256: &'a str,
    collector_source_sha256: &'a str,
    composite_identity: &'a CompositeIdentityV1,
    observations: &'a [ObservationResultV1],
    plan_sha256: &'a str,
    schema: &'static str,
    schema_version: u32,
    successor_final_tooling: &'a RepositoryIdentityV1,
    target_host_alias: &'a str,
    target_profile_sha256: &'a str,
}

#[derive(Serialize)]
#[allow(dead_code)] // Constructed by the next collector slice; exercised here by unit tests.
struct ObservationManifestBindingV1<'a> {
    canonical_payload_bytes: u64,
    canonical_payload_sha256: &'a str,
    challenge_nonce_sha256: &'a str,
    composite_identity_sha256: String,
    observations_sha256: String,
    plan_sha256: &'a str,
    schema: &'static str,
    schema_version: u32,
    successor_final_tooling_sha256: String,
    target_host_alias: &'a str,
    target_profile_sha256: &'a str,
}

#[allow(dead_code)] // Called by the next collector slice; exercised here by unit tests.
fn expected_observation_receipt(
    plan: &Phase1CollectorPlanV1,
    observations: &[ObservationResultV1],
) -> Result<ObservationReceiptV1, LinuxMnlError> {
    let plan_sha256 = canonical_sha256(plan)?;
    let payload = ObservationPayloadV1 {
        challenge_nonce_sha256: &plan.challenge_nonce_sha256,
        collector_binary_sha256: &plan.collector_binary_sha256,
        collector_source_sha256: &plan.collector_source_sha256,
        composite_identity: &plan.composite_identity,
        observations,
        plan_sha256: &plan_sha256,
        schema: "hepta_linux_mnl_phase1_observation_payload_v1",
        schema_version: 1,
        successor_final_tooling: &plan.successor_final_tooling,
        target_host_alias: &plan.target_host_alias,
        target_profile_sha256: &plan.target_profile_sha256,
    };
    let payload_bytes = canonical_json(&payload)?;
    let canonical_payload_bytes = u64::try_from(payload_bytes.len())
        .map_err(|_| invalid("phase1 observation payload length overflow"))?;
    let canonical_payload_sha256 = sha256(&payload_bytes);
    let manifest = ObservationManifestBindingV1 {
        canonical_payload_bytes,
        canonical_payload_sha256: &canonical_payload_sha256,
        challenge_nonce_sha256: &plan.challenge_nonce_sha256,
        composite_identity_sha256: canonical_sha256(&plan.composite_identity)?,
        observations_sha256: canonical_sha256(&observations)?,
        plan_sha256: &plan_sha256,
        schema: "hepta_linux_mnl_phase1_observation_manifest_binding_v1",
        schema_version: 1,
        successor_final_tooling_sha256: canonical_sha256(&plan.successor_final_tooling)?,
        target_host_alias: &plan.target_host_alias,
        target_profile_sha256: &plan.target_profile_sha256,
    };
    let receipt_manifest_sha256 = canonical_sha256(&manifest)?;
    Ok(ObservationReceiptV1 {
        canonical_payload_bytes,
        canonical_payload_sha256,
        receipt_manifest_sha256,
        schema: "hepta_linux_mnl_phase1_observation_receipt_v1".to_string(),
        signature_verified: false,
    })
}

#[allow(dead_code)] // Called by the next collector slice; exercised here by unit tests.
fn validate_observations(observations: &[ObservationResultV1]) -> Result<(), LinuxMnlError> {
    if observations.len() != ObservationKindV1::all().len() {
        return Err(invalid("phase1 observation set is incomplete"));
    }
    for (observation, expected_kind) in observations.iter().zip(ObservationKindV1::all()) {
        if observation.kind != expected_kind {
            return Err(invalid(
                "phase1 observations are not in exact canonical order",
            ));
        }
        match observation.status {
            ObservationStatusV1::ExactMatch | ObservationStatusV1::Mismatch => {
                let digest = observation
                    .evidence_sha256
                    .as_deref()
                    .ok_or_else(|| invalid("observed evidence digest is missing"))?;
                validate_digest("phase1 observation evidence", digest)?;
                if observation.observed_bytes == Some(0) || observation.observed_bytes.is_none() {
                    return Err(invalid("observed evidence byte count is missing"));
                }
            }
            ObservationStatusV1::Missing => {
                if observation.evidence_sha256.is_some() || observation.observed_bytes.is_some() {
                    return Err(invalid("missing observation unexpectedly carries evidence"));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use crate::identity::test_support as identity;
    use crate::profiles::test_support as profiles;

    pub(crate) fn plan() -> Phase1CollectorPlanV1 {
        plan_with_challenge("private-test-challenge-nonce")
    }

    pub(crate) fn plan_with_challenge(label: &str) -> Phase1CollectorPlanV1 {
        let documents = profiles::documents();
        let verified = profiles::verify(&documents, profiles::pins(&documents)).expect("profiles");
        let tooling = identity::verified_successor_tooling();
        let challenge = InternalChallengeNonceV1 {
            sha256: profiles::nonempty_sha256(label),
        };
        build_phase1_plan(verified, tooling, challenge).expect("plan")
    }

    pub(crate) fn result(plan: &Phase1CollectorPlanV1) -> Phase1CollectorResultV1 {
        let observations = ObservationKindV1::all()
            .into_iter()
            .map(|kind| ObservationResultV1 {
                evidence_sha256: Some(profiles::nonempty_sha256(&format!("{kind:?}"))),
                kind,
                observed_bytes: Some(1),
                status: ObservationStatusV1::ExactMatch,
            })
            .collect::<Vec<_>>();
        Phase1CollectorResultV1 {
            authority: AuthorityBoundaryV1::closed(),
            challenge_nonce_sha256: plan.challenge_nonce_sha256.clone(),
            claimed_target_qualification_pass: false,
            collector_binary_sha256: plan.collector_binary_sha256.clone(),
            collector_environment: CollectorEnvironmentV1::ProductionLinuxMnlTarget,
            collector_source_sha256: plan.collector_source_sha256.clone(),
            composite_identity: exact_composite_identity(),
            linux_gate_receipt_sha256: None,
            operator_ceremony_receipt_sha256: None,
            phase1_observation_receipt: expected_observation_receipt(plan, &observations)
                .expect("observation receipt"),
            observations,
            plan_sha256: canonical_sha256(plan).expect("plan hash"),
            receipt_topology: exact_receipt_topology(),
            schema: "hepta_linux_mnl_phase1_collector_result_v1".to_string(),
            schema_version: 1,
            scoped_aggregate_receipt_sha256: None,
            successor_final_tooling: plan.successor_final_tooling.clone(),
            target_host_alias: plan.target_host_alias.clone(),
            target_profile_sha256: plan.target_profile_sha256.clone(),
            target_qualification_receipt_sha256: None,
            verdict: Phase1VerdictV1::StructurallyCompleteNoAuthority,
        }
    }

    pub(crate) fn authority_mut(result: &mut Phase1CollectorResultV1) -> &mut AuthorityBoundaryV1 {
        &mut result.authority
    }

    pub(crate) fn claims_target_pass(result: &mut Phase1CollectorResultV1) {
        result.claimed_target_qualification_pass = true;
        result.target_qualification_receipt_sha256 = Some(profiles::digest('a'));
    }

    pub(crate) fn set_fixture_environment(result: &mut Phase1CollectorResultV1) {
        result.collector_environment = CollectorEnvironmentV1::X230QualificationFixture;
    }

    pub(crate) fn set_first_observation_missing(result: &mut Phase1CollectorResultV1) {
        result.observations[0].status = ObservationStatusV1::Missing;
        result.observations[0].evidence_sha256 = None;
        result.observations[0].observed_bytes = None;
    }

    pub(crate) fn set_first_observation_mismatch(result: &mut Phase1CollectorResultV1) {
        result.observations[0].status = ObservationStatusV1::Mismatch;
    }

    pub(crate) fn set_blocked_verdict_and_rebind(
        plan: &Phase1CollectorPlanV1,
        result: &mut Phase1CollectorResultV1,
    ) {
        result.verdict = Phase1VerdictV1::BlockedNoAuthority;
        result.phase1_observation_receipt =
            expected_observation_receipt(plan, &result.observations).expect("rebind receipt");
    }

    pub(crate) fn tamper_receipt_manifest(result: &mut Phase1CollectorResultV1) {
        result.phase1_observation_receipt.receipt_manifest_sha256 = profiles::digest('f');
    }

    pub(crate) fn is_structurally_complete(result: &Phase1CollectorResultV1) -> bool {
        result.verdict == Phase1VerdictV1::StructurallyCompleteNoAuthority
    }

    pub(crate) fn has_target_receipt(result: &Phase1CollectorResultV1) -> bool {
        result.target_qualification_receipt_sha256.is_some()
    }

    pub(crate) fn authority(result: &Phase1CollectorResultV1) -> &AuthorityBoundaryV1 {
        &result.authority
    }
}
