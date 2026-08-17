use serde::Serialize;
use static_assertions::assert_not_impl_any;

use codex_hepta_mnl_trust_v1::FrozenArtifactBytesV1;
use codex_hepta_mnl_trust_v1::ReplayPlatformScopeV1;

use crate::*;

assert_not_impl_any!(JoinedNixFinalFreezePlanClaimInspectionV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);

#[test]
fn exact_nix_final_freeze_projection_covers_source_roles_and_named_materials() {
    let binding = crate::run_plan_tests::binding();
    let expected = expected_nix_final_freeze_projection(&binding);

    assert_eq!(expected.platform_scope, ReplayPlatformScopeV1::Nix);
    assert_eq!(
        expected.final_artifact_freeze_payload_sha256,
        binding.final_artifact_freeze_payload_sha256
    );
    assert_eq!(
        expected.final_artifact_freeze_profile_id,
        binding.final_artifact_freeze_profile_id
    );
    assert_eq!(expected.final_tooling.head, binding.final_tooling.head);
    assert_eq!(expected.final_tooling.tree, binding.final_tooling.tree);
    assert_eq!(
        expected.canonical_source.archive,
        frozen(&binding.source_archive)
    );
    assert_eq!(
        expected.canonical_source.source_tree_manifest_sha256,
        binding.source_tree_manifest_sha256
    );

    assert_eq!(
        expected
            .platform_artifacts
            .iter()
            .map(|artifact| artifact.role_id.as_str())
            .collect::<Vec<_>>(),
        [
            "collector",
            "driver",
            "nix_store_seed_bundle",
            "runner",
            "seccomp_profile",
            "verifier",
        ]
    );
    assert_eq!(
        expected
            .platform_artifacts
            .iter()
            .map(|artifact| artifact.artifact.clone())
            .collect::<Vec<_>>(),
        vec![
            frozen(&binding.collector_binary),
            frozen(&binding.driver_binary),
            frozen(&binding.nix_store_seed_bundle),
            frozen(&binding.runner_binary),
            frozen(&binding.seccomp_profile),
            frozen(&binding.verifier_binary),
        ]
    );
    assert!(
        expected
            .platform_artifacts
            .iter()
            .all(|artifact| artifact.platform_scope == ReplayPlatformScopeV1::Nix)
    );

    assert_eq!(
        expected
            .named_materials
            .iter()
            .map(|material| material.name.as_str())
            .collect::<Vec<_>>(),
        [
            "docker_image_config_id",
            "docker_image_manifest",
            "nix_store_seed_inventory",
        ]
    );
    assert_eq!(
        expected
            .named_materials
            .iter()
            .map(|material| material.sha256.as_str())
            .collect::<Vec<_>>(),
        [
            binding.docker_platform_config_image_id_sha256.as_str(),
            PINNED_IMAGE_SHA256,
            binding.nix_store_seed_inventory_sha256.as_str(),
        ]
    );
    assert!(
        expected
            .named_materials
            .iter()
            .all(|material| material.platform_scope == ReplayPlatformScopeV1::Nix)
    );
}

#[test]
fn every_plan_pin_axis_changes_the_non_authorizing_freeze_projection() {
    let original = crate::run_plan_tests::binding();
    let expected = expected_nix_final_freeze_projection(&original);
    let mut mutations = Vec::new();

    macro_rules! mutate_pin {
        ($field:ident, $character:literal) => {{
            let mut changed = original.clone();
            changed.$field.sha256 = digest($character);
            mutations.push(changed);
            let mut changed = original.clone();
            changed.$field.byte_count += 1;
            mutations.push(changed);
            let mut changed = original.clone();
            changed.$field.mode = if changed.$field.mode == "0444" {
                "0400".to_string()
            } else {
                "0500".to_string()
            };
            mutations.push(changed);
        }};
    }

    mutate_pin!(source_archive, 'a');
    mutate_pin!(collector_binary, 'b');
    mutate_pin!(driver_binary, 'c');
    mutate_pin!(nix_store_seed_bundle, 'd');
    mutate_pin!(runner_binary, 'e');
    mutate_pin!(seccomp_profile, 'f');
    mutate_pin!(verifier_binary, 'a');

    for (field, value) in [
        ("source tree", digest('a')),
        ("Docker config image", digest('b')),
        ("seed inventory", digest('c')),
        ("final-freeze payload", digest('d')),
    ] {
        let mut changed = original.clone();
        match field {
            "source tree" => changed.source_tree_manifest_sha256 = value,
            "Docker config image" => changed.docker_platform_config_image_id_sha256 = value,
            "seed inventory" => changed.nix_store_seed_inventory_sha256 = value,
            "final-freeze payload" => changed.final_artifact_freeze_payload_sha256 = value,
            _ => unreachable!(),
        }
        mutations.push(changed);
    }
    let mut changed = original.clone();
    changed.final_artifact_freeze_profile_id = "different-final-freeze-profile".to_string();
    mutations.push(changed);
    let mut changed = original.clone();
    changed.final_tooling.head = "c".repeat(40);
    mutations.push(changed);
    let mut changed = original;
    changed.final_tooling.tree = "d".repeat(40);
    mutations.push(changed);

    for changed in mutations {
        assert_ne!(expected_nix_final_freeze_projection(&changed), expected);
    }
}

#[test]
fn dynamic_plan_state_is_bound_by_the_signed_plan_not_static_final_freeze() {
    let binding = crate::run_plan_tests::binding();
    let expected = expected_nix_final_freeze_projection(&binding);
    let mut changed_docker_config = binding.clone();
    changed_docker_config.docker_config_sha256 = digest('b');
    assert_eq!(
        expected_nix_final_freeze_projection(&changed_docker_config),
        expected
    );

    let mut presealed = binding;
    presealed.isolation_mode = NixIsolationModeV1::PresealedOfflineClosure;
    presealed.presealed_offline_closure_sha256 = Some(digest('a'));
    presealed.presealed_output_store_path =
        Some("/nix/store/abcdfghijklmnpqrsvwxyz0123456789-hepta-product-v1".to_string());
    presealed.presealed_check_output_store_path =
        Some("/nix/store/bbcdfghijklmnpqrsvwxyz0123456789-hepta-check-v1".to_string());

    assert_eq!(expected_nix_final_freeze_projection(&presealed), expected);
}

fn frozen(pin: &ClosedArtifactPinV1) -> FrozenArtifactBytesV1 {
    FrozenArtifactBytesV1 {
        byte_count: pin.byte_count,
        mode: pin.mode.clone(),
        sha256: pin.sha256.clone(),
    }
}

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}
