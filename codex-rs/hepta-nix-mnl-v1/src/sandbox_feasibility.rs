use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;

use crate::ClosedAuthorityV1;
use crate::ClosedRunPlanDispositionV1;
use crate::InspectedNixClosedRunPlanV1;
use crate::NIX_CLOSED_RUN_PLAN_SCHEMA;
use crate::NIX_VERSION;
use crate::NixClosedRunPlanWireV1;
use crate::NixMnlError;
use crate::invalid;

pub const NIX_SANDBOX_REQUALIFICATION_SCHEMA: &str = "hepta_nix_mnl_sandbox_requalification_v1";
pub const NIX_SANDBOX_REQUALIFICATION_SCHEMA_VERSION: u32 = 1;
pub const MAX_NIX_SANDBOX_REQUALIFICATION_BYTES: usize = 16 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixSandboxFeasibilityDispositionV1 {
    CurrentV3ContractInvalidatedNoQualification,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixSandboxEvidenceDispositionV1 {
    NegativeDevelopmentProbeOnlyNoQualification,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixSandboxRequalificationAxisV1 {
    RunUniqueBoundedExecutableBuildScratchUnqualified,
    BuilderVerifierSecurityProfilesNotSeparated,
    CompleteNixSeedStoreDbProfileBootstrapUnqualified,
    ZeroAddedCapabilityUnprivilegedUserNamespacePrivateMountUnqualified,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixSandboxForbiddenFallbackV1 {
    InitialNamespaceCapSysAdmin,
    PrivilegedContainer,
    UnconfinedLsm,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixSandboxFailureDispositionV1 {
    RejectContainerUseDedicatedVmOrMicroVm,
}

/// Canonical negative requalification record for the current V3 sandbox plan.
///
/// This record freezes why the current plan is not qualified. It is neither a
/// positive probe result nor a request to mutate the executable closed plan.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixSandboxRequalificationEnvelopeV1 {
    pub authority: ClosedAuthorityV1,
    pub current_contract_disposition: NixSandboxFeasibilityDispositionV1,
    pub docker_image: String,
    pub docker_image_config_id_sha256: String,
    pub docker_image_manifest_sha256: String,
    pub evidence_disposition: NixSandboxEvidenceDispositionV1,
    pub failure_disposition: NixSandboxFailureDispositionV1,
    pub forbidden_fallbacks: Vec<NixSandboxForbiddenFallbackV1>,
    pub inspected_closed_plan_byte_count: u64,
    pub inspected_closed_plan_disposition: ClosedRunPlanDispositionV1,
    pub inspected_closed_plan_schema: String,
    pub inspected_closed_plan_schema_version: u32,
    pub inspected_closed_plan_sha256: String,
    pub launch_authorized: bool,
    pub nix_version: String,
    pub pass_authorized: bool,
    pub ready_to_plan: bool,
    pub receipt_acceptance_authorized: bool,
    pub replay_publication_authorized: bool,
    pub requalification_axes: Vec<NixSandboxRequalificationAxisV1>,
    pub sandbox_qualification_observed: bool,
    pub schema: String,
    pub schema_version: u32,
}

/// Opaque inspection of the exact negative requalification envelope.
///
/// It intentionally retains no closed-plan execution handle and cannot be
/// converted into launch, replay, receipt, qualification, or PASS authority.
#[derive(Debug)]
pub struct InvalidatedNixSandboxContractInspectionV1 {
    canonical_bytes: Vec<u8>,
    envelope: NixSandboxRequalificationEnvelopeV1,
    envelope_sha256: String,
}

impl InvalidatedNixSandboxContractInspectionV1 {
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }

    pub fn envelope(&self) -> &NixSandboxRequalificationEnvelopeV1 {
        &self.envelope
    }

    pub fn envelope_sha256(&self) -> &str {
        &self.envelope_sha256
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }

    pub const fn launch_grant_available(&self) -> bool {
        false
    }

    pub const fn launch_performed(&self) -> bool {
        false
    }

    pub const fn pass_authorized(&self) -> bool {
        false
    }

    pub const fn ready_to_plan(&self) -> bool {
        false
    }

    pub const fn receipt_acceptance_authorized(&self) -> bool {
        false
    }

    pub const fn receipt_accepted(&self) -> bool {
        false
    }

    pub const fn replay_publication_available(&self) -> bool {
        false
    }

    pub const fn replay_publication_authorized(&self) -> bool {
        false
    }

    pub const fn sandbox_qualified(&self) -> bool {
        false
    }

    pub const fn sandbox_qualification_observed(&self) -> bool {
        false
    }
}

pub fn derive_nix_sandbox_requalification_envelope(
    plan: &InspectedNixClosedRunPlanV1,
) -> Result<NixSandboxRequalificationEnvelopeV1, NixMnlError> {
    let wire = retained_wire_plan(plan)?;
    if wire.disposition
        != ClosedRunPlanDispositionV1::FreshSandboxBuildInspectionOnlyNoLaunchAuthority
    {
        return Err(invalid(
            "sandbox requalification accepts only the current fresh-sandbox plan",
        ));
    }
    let byte_count = u64::try_from(plan.canonical_bytes().len())
        .map_err(|_| invalid("closed Nix run plan byte count does not fit u64"))?;
    Ok(NixSandboxRequalificationEnvelopeV1 {
        authority: ClosedAuthorityV1::exact(),
        current_contract_disposition:
            NixSandboxFeasibilityDispositionV1::CurrentV3ContractInvalidatedNoQualification,
        docker_image: wire.host_preflight.image,
        docker_image_config_id_sha256: wire.host_preflight.image_config_id_sha256,
        docker_image_manifest_sha256: wire.host_preflight.image_manifest_sha256,
        evidence_disposition:
            NixSandboxEvidenceDispositionV1::NegativeDevelopmentProbeOnlyNoQualification,
        failure_disposition:
            NixSandboxFailureDispositionV1::RejectContainerUseDedicatedVmOrMicroVm,
        forbidden_fallbacks: vec![
            NixSandboxForbiddenFallbackV1::InitialNamespaceCapSysAdmin,
            NixSandboxForbiddenFallbackV1::PrivilegedContainer,
            NixSandboxForbiddenFallbackV1::UnconfinedLsm,
        ],
        inspected_closed_plan_byte_count: byte_count,
        inspected_closed_plan_disposition: wire.disposition,
        inspected_closed_plan_schema: wire.schema,
        inspected_closed_plan_schema_version: wire.schema_version,
        inspected_closed_plan_sha256: sha256_hex(plan.canonical_bytes()),
        launch_authorized: false,
        nix_version: wire.host_preflight.nix_version,
        pass_authorized: false,
        ready_to_plan: false,
        receipt_acceptance_authorized: false,
        replay_publication_authorized: false,
        requalification_axes: vec![
            NixSandboxRequalificationAxisV1::RunUniqueBoundedExecutableBuildScratchUnqualified,
            NixSandboxRequalificationAxisV1::BuilderVerifierSecurityProfilesNotSeparated,
            NixSandboxRequalificationAxisV1::CompleteNixSeedStoreDbProfileBootstrapUnqualified,
            NixSandboxRequalificationAxisV1::ZeroAddedCapabilityUnprivilegedUserNamespacePrivateMountUnqualified,
        ],
        sandbox_qualification_observed: false,
        schema: NIX_SANDBOX_REQUALIFICATION_SCHEMA.to_string(),
        schema_version: NIX_SANDBOX_REQUALIFICATION_SCHEMA_VERSION,
    })
}

pub fn inspect_canonical_nix_sandbox_requalification_envelope(
    plan: &InspectedNixClosedRunPlanV1,
    bytes: &[u8],
) -> Result<InvalidatedNixSandboxContractInspectionV1, NixMnlError> {
    if bytes.is_empty() || bytes.len() > MAX_NIX_SANDBOX_REQUALIFICATION_BYTES {
        return Err(invalid(
            "Nix sandbox requalification envelope byte length is outside its bound",
        ));
    }
    let envelope: NixSandboxRequalificationEnvelopeV1 =
        serde_json::from_slice(bytes).map_err(|error| {
            invalid(format!(
                "Nix sandbox requalification envelope is malformed: {error}"
            ))
        })?;
    let canonical = serde_json::to_vec(&envelope)?;
    if canonical != bytes {
        return Err(invalid(
            "Nix sandbox requalification envelope is not exact canonical JSON",
        ));
    }
    let expected = derive_nix_sandbox_requalification_envelope(plan)?;
    if envelope != expected {
        return Err(invalid(
            "Nix sandbox requalification envelope differs from the exact negative model",
        ));
    }
    Ok(InvalidatedNixSandboxContractInspectionV1 {
        canonical_bytes: canonical,
        envelope,
        envelope_sha256: sha256_hex(bytes),
    })
}

fn retained_wire_plan(
    plan: &InspectedNixClosedRunPlanV1,
) -> Result<NixClosedRunPlanWireV1, NixMnlError> {
    let wire: NixClosedRunPlanWireV1 =
        serde_json::from_slice(plan.canonical_bytes()).map_err(|error| {
            invalid(format!(
                "inspected closed Nix run plan cannot be retained: {error}"
            ))
        })?;
    if wire.schema != NIX_CLOSED_RUN_PLAN_SCHEMA
        || wire.schema_version != 3
        || wire.disposition != plan.disposition()
        || wire.host_preflight.image != wire.verifier_container.image
        || wire.host_preflight.image_config_id_sha256
            != wire.verifier_container.image_config_id_sha256
        || wire.host_preflight.image_manifest_sha256
            != wire.verifier_container.image_manifest_sha256
        || wire.host_preflight.nix_version != NIX_VERSION
    {
        return Err(invalid(
            "inspected closed Nix run plan lacks the exact retained V3 image/Nix binding",
        ));
    }
    if let Some(builder) = &wire.builder_container
        && (wire.host_preflight.image != builder.image
            || wire.host_preflight.image_config_id_sha256 != builder.image_config_id_sha256
            || wire.host_preflight.image_manifest_sha256 != builder.image_manifest_sha256)
    {
        return Err(invalid(
            "inspected closed Nix run plan builder image binding differs",
        ));
    }
    Ok(wire)
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}
