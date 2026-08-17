use codex_hepta_mnl_trust_v1::ExpectedCanonicalSourceFreezeV1;
use codex_hepta_mnl_trust_v1::ExpectedPlatformArtifactBytesV1;
use codex_hepta_mnl_trust_v1::ExpectedPlatformArtifactFreezeV1;
use codex_hepta_mnl_trust_v1::FrozenArtifactBytesV1;
use codex_hepta_mnl_trust_v1::InspectedFinalArtifactFreezeV1;
use codex_hepta_mnl_trust_v1::MatchedFinalFreezePlanClaimInspectionV1;
use codex_hepta_mnl_trust_v1::NamedMaterialFreezeV1;
use codex_hepta_mnl_trust_v1::ReplayPlatformScopeV1;
use codex_hepta_mnl_trust_v1::RepositoryIdentityV1 as TrustRepositoryIdentityV1;
use codex_hepta_mnl_trust_v1::match_final_freeze_to_prepared_claim;

use crate::ClosedArtifactPinV1;
use crate::InspectedNixClosedRunPlanV1;
use crate::JoinedNixClosedRunPlanPreparedClaimInspectionV1;
use crate::NIX_WORKSPACE_CHECK_CONTRACT_NAMED_MATERIAL;
use crate::NixClosedRunPlanBindingV1;
use crate::NixMnlError;
use crate::PINNED_IMAGE_SHA256;
use crate::invalid;

pub const NIX_FINAL_FREEZE_PLATFORM_SCOPE: ReplayPlatformScopeV1 = ReplayPlatformScopeV1::Nix;

const COLLECTOR_ROLE_ID: &str = "collector";
const DRIVER_ROLE_ID: &str = "driver";
const NIX_STORE_SEED_BUNDLE_ROLE_ID: &str = "nix_store_seed_bundle";
const RUNNER_ROLE_ID: &str = "runner";
const SECCOMP_PROFILE_ROLE_ID: &str = "seccomp_profile";
const VERIFIER_ROLE_ID: &str = "verifier";

const DOCKER_IMAGE_CONFIG_ID_MATERIAL: &str = "docker_image_config_id";
const DOCKER_IMAGE_MANIFEST_MATERIAL: &str = "docker_image_manifest";
const NIX_STORE_SEED_INVENTORY_MATERIAL: &str = "nix_store_seed_inventory";

/// Opaque structural join of typed final-freeze semantics, a closed Nix plan,
/// and the prepared signed claim already matched to that plan.
///
/// This token records exact equality only. It observes no artifact bytes,
/// source/build provenance, durable publication, wall clock, launch, or
/// receipt, and it cannot authorize a live action.
#[derive(Debug)]
pub struct JoinedNixFinalFreezePlanClaimInspectionV1 {
    matched_freeze_plan_claim: MatchedFinalFreezePlanClaimInspectionV1,
    plan: InspectedNixClosedRunPlanV1,
}

impl JoinedNixFinalFreezePlanClaimInspectionV1 {
    pub fn plan(&self) -> &InspectedNixClosedRunPlanV1 {
        &self.plan
    }

    pub fn matched_freeze_plan_claim(&self) -> &MatchedFinalFreezePlanClaimInspectionV1 {
        &self.matched_freeze_plan_claim
    }

    pub const fn typed_final_freeze_bound(&self) -> bool {
        true
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }

    pub const fn actual_artifact_bytes_observed(&self) -> bool {
        false
    }

    pub const fn source_provenance_observed(&self) -> bool {
        false
    }

    pub const fn toolchain_provenance_observed(&self) -> bool {
        false
    }

    pub const fn durable_claim_observed(&self) -> bool {
        false
    }

    pub const fn wall_clock_verified(&self) -> bool {
        false
    }

    pub const fn launch_grant_available(&self) -> bool {
        false
    }

    pub const fn launch_performed(&self) -> bool {
        false
    }

    pub const fn receipt_emitted(&self) -> bool {
        false
    }
}

pub fn join_nix_final_freeze_plan_claim(
    final_freeze: InspectedFinalArtifactFreezeV1,
    joined_plan_claim: JoinedNixClosedRunPlanPreparedClaimInspectionV1,
) -> Result<JoinedNixFinalFreezePlanClaimInspectionV1, NixMnlError> {
    let (plan, matched_claim) = joined_plan_claim.into_parts();
    let expected = expected_nix_final_freeze_projection(plan.binding());
    let matched_freeze_plan_claim = match_final_freeze_to_prepared_claim(
        final_freeze,
        matched_claim,
        &expected,
    )
    .map_err(|error| {
        invalid(format!(
            "typed final artifact freeze differs from its exact Nix plan/claim projection: {error}"
        ))
    })?;
    Ok(JoinedNixFinalFreezePlanClaimInspectionV1 {
        matched_freeze_plan_claim,
        plan,
    })
}

pub(crate) fn expected_nix_final_freeze_projection(
    binding: &NixClosedRunPlanBindingV1,
) -> ExpectedPlatformArtifactFreezeV1 {
    ExpectedPlatformArtifactFreezeV1 {
        canonical_source: ExpectedCanonicalSourceFreezeV1 {
            archive: frozen_bytes(&binding.source_archive),
            source_tree_manifest_sha256: binding.source_tree_manifest_sha256.clone(),
        },
        final_artifact_freeze_payload_sha256: binding.final_artifact_freeze_payload_sha256.clone(),
        final_artifact_freeze_profile_id: binding.final_artifact_freeze_profile_id.clone(),
        final_tooling: TrustRepositoryIdentityV1 {
            head: binding.final_tooling.head.clone(),
            tree: binding.final_tooling.tree.clone(),
        },
        named_materials: vec![
            named_material(
                DOCKER_IMAGE_CONFIG_ID_MATERIAL,
                &binding.docker_platform_config_image_id_sha256,
            ),
            named_material(DOCKER_IMAGE_MANIFEST_MATERIAL, PINNED_IMAGE_SHA256),
            named_material(
                NIX_STORE_SEED_INVENTORY_MATERIAL,
                &binding.nix_store_seed_inventory_sha256,
            ),
            named_material(
                NIX_WORKSPACE_CHECK_CONTRACT_NAMED_MATERIAL,
                &binding.workspace_check_contract_sha256,
            ),
        ],
        platform_artifacts: vec![
            platform_artifact(COLLECTOR_ROLE_ID, &binding.collector_binary),
            platform_artifact(DRIVER_ROLE_ID, &binding.driver_binary),
            platform_artifact(
                NIX_STORE_SEED_BUNDLE_ROLE_ID,
                &binding.nix_store_seed_bundle,
            ),
            platform_artifact(RUNNER_ROLE_ID, &binding.runner_binary),
            platform_artifact(SECCOMP_PROFILE_ROLE_ID, &binding.seccomp_profile),
            platform_artifact(VERIFIER_ROLE_ID, &binding.verifier_binary),
        ],
        platform_scope: NIX_FINAL_FREEZE_PLATFORM_SCOPE,
    }
}

fn frozen_bytes(pin: &ClosedArtifactPinV1) -> FrozenArtifactBytesV1 {
    FrozenArtifactBytesV1 {
        byte_count: pin.byte_count,
        mode: pin.mode.clone(),
        sha256: pin.sha256.clone(),
    }
}

fn platform_artifact(role_id: &str, pin: &ClosedArtifactPinV1) -> ExpectedPlatformArtifactBytesV1 {
    ExpectedPlatformArtifactBytesV1 {
        artifact: frozen_bytes(pin),
        platform_scope: NIX_FINAL_FREEZE_PLATFORM_SCOPE,
        role_id: role_id.to_string(),
    }
}

fn named_material(name: &str, sha256: &str) -> NamedMaterialFreezeV1 {
    NamedMaterialFreezeV1 {
        name: name.to_string(),
        platform_scope: NIX_FINAL_FREEZE_PLATFORM_SCOPE,
        sha256: sha256.to_string(),
    }
}
