use serde::Serialize;
use static_assertions::assert_not_impl_any;

use crate::*;

const TEST_FINAL_FREEZE_PROFILE_ID: &str = "hepta-mnl-test-only-final-freeze-v1";

assert_not_impl_any!(InspectedFinalArtifactFreezeV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);
assert_not_impl_any!(MatchedFinalFreezePlanClaimInspectionV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);

#[test]
fn typed_final_freeze_consumes_n1_n2_and_remains_non_authorizing() {
    let inspected = inspected_test_final_freeze();
    assert_eq!(inspected.profile_id(), TEST_FINAL_FREEZE_PROFILE_ID);
    assert_eq!(
        inspected.final_tooling(),
        inspected.ancestry().final_tooling()
    );
    assert_eq!(inspected.platform_artifacts().len(), 6);
    assert_eq!(inspected.named_materials().len(), 3);
    assert!(!inspected.authorizes_live());
    assert!(!inspected.actual_artifact_bytes_observed());
    assert!(!inspected.source_provenance_observed());
    assert!(!inspected.toolchain_provenance_observed());

    let payload: FinalArtifactFreezePayloadV1 =
        serde_json::from_slice(inspected.signature_inspection().exact_payload_bytes())
            .expect("typed final-freeze payload");
    assert_eq!(payload.schema, FINAL_ARTIFACT_FREEZE_PAYLOAD_SCHEMA);
    assert_eq!(payload.phase_a_anchor, exact_phase_a_anchor());
    assert_eq!(payload.ancestry_commit_count, 2);
    assert_eq!(
        payload
            .platform_artifacts
            .iter()
            .map(|record| record.role_id.as_str())
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
        payload
            .named_materials
            .iter()
            .map(|record| record.name.as_str())
            .collect::<Vec<_>>(),
        [
            "docker_image_config_id",
            "docker_image_manifest",
            "nix_store_seed_inventory",
        ]
    );
}

#[test]
fn final_freeze_rejects_role_profile_anchor_and_every_ancestry_transplant() {
    let ancestry = crate::tests::inspect_test_structural_ancestry();
    let payload = test_payload(&ancestry);
    let signature = test_signature(
        DetachedSignatureRoleV1::PreRunProfile,
        TEST_FINAL_FREEZE_PROFILE_ID,
        serde_json::to_vec(&payload).expect("wrong-role payload"),
    );
    assert!(inspect_final_artifact_freeze_semantics(ancestry, signature).is_err());

    reject_payload_mutation(|payload| payload.profile_id = "other-final-freeze-v1".to_string());
    reject_payload_mutation(|payload| payload.phase_a_anchor.head = "1".repeat(40));
    reject_payload_mutation(|payload| payload.phase_a_anchor.tree = "2".repeat(40));
    reject_payload_mutation(|payload| payload.final_tooling.head = "3".repeat(40));
    reject_payload_mutation(|payload| payload.final_tooling.tree = "4".repeat(40));
    reject_payload_mutation(|payload| payload.ancestry_manifest_sha256 = digest('5'));
    reject_payload_mutation(|payload| payload.ancestry_raw_objects_sha256 = digest('6'));
    reject_payload_mutation(|payload| payload.ancestry_commit_count += 1);
}

#[test]
fn final_freeze_payload_is_strict_canonical_and_rejects_self_or_downstream_fields() {
    let ancestry = crate::tests::inspect_test_structural_ancestry();
    let payload = test_payload(&ancestry);
    let mut trailing = serde_json::to_vec(&payload).expect("canonical payload");
    trailing.push(b'\n');
    assert!(inspect_exact_payload(ancestry, trailing).is_err());

    let ancestry = crate::tests::inspect_test_structural_ancestry();
    let pretty = serde_json::to_vec_pretty(&test_payload(&ancestry)).expect("pretty payload");
    assert!(inspect_exact_payload(ancestry, pretty).is_err());

    for forbidden in [
        "payload_sha256",
        "signature_sha256",
        "manifest_sha256",
        "platform_closed_run_plan_sha256",
        "receipt_sha256",
        "challenge_nonce_sha256",
        "host_identity_sha256",
    ] {
        let ancestry = crate::tests::inspect_test_structural_ancestry();
        let mut value = serde_json::to_value(test_payload(&ancestry)).expect("payload value");
        value[forbidden] = serde_json::json!(digest('9'));
        let exact = serde_json::to_vec(&value).expect("unknown-field payload");
        assert!(
            inspect_exact_payload(ancestry, exact).is_err(),
            "unknown field {forbidden} must fail"
        );
    }
}

#[test]
fn final_freeze_rejects_malformed_source_artifact_and_material_records() {
    reject_payload_mutation(|payload| payload.schema = "other_schema_v1".to_string());
    reject_payload_mutation(|payload| payload.canonical_source.archive.byte_count = 0);
    reject_payload_mutation(|payload| payload.canonical_source.archive.mode = "444".to_string());
    reject_payload_mutation(|payload| payload.canonical_source.archive.sha256 = digest('0'));
    reject_payload_mutation(|payload| {
        payload.canonical_source.archive_recipe_sha256 = "A".repeat(64)
    });
    reject_payload_mutation(|payload| {
        payload.canonical_source.source_tree_manifest_sha256 = "1".repeat(63)
    });
    reject_payload_mutation(|payload| {
        payload.canonical_source.toolchain_manifest_sha256 = digest('0')
    });
    reject_payload_mutation(|payload| payload.platform_artifacts.clear());
    reject_payload_mutation(|payload| payload.platform_artifacts[0].artifact.byte_count = 0);
    reject_payload_mutation(|payload| {
        payload.platform_artifacts[0].role_id = "Bad Role".to_string()
    });
    reject_payload_mutation(|payload| {
        payload.platform_artifacts[0].build_recipe_sha256 = digest('0')
    });
    reject_payload_mutation(|payload| {
        payload.platform_artifacts[0].role_source_manifest_sha256 = "F".repeat(64)
    });
    reject_payload_mutation(|payload| {
        payload.platform_artifacts[0].toolchain_manifest_sha256 = "f".repeat(63)
    });
    reject_payload_mutation(|payload| payload.platform_artifacts.swap(0, 1));
    reject_payload_mutation(|payload| {
        payload.platform_artifacts[1].role_id = payload.platform_artifacts[0].role_id.clone()
    });
    reject_payload_mutation(|payload| {
        payload.platform_artifacts[1].artifact.sha256 =
            payload.platform_artifacts[0].artifact.sha256.clone()
    });
    reject_payload_mutation(|payload| payload.named_materials.swap(0, 1));
    reject_payload_mutation(|payload| payload.named_materials.clear());
    reject_payload_mutation(|payload| {
        payload.named_materials[1].name = payload.named_materials[0].name.clone()
    });
    reject_payload_mutation(|payload| {
        payload.named_materials[1].sha256 = payload.named_materials[0].sha256.clone()
    });
    reject_payload_mutation(|payload| payload.named_materials[0].sha256 = digest('0'));
}

#[test]
fn typed_freeze_matcher_joins_exact_claim_and_plan_projection_without_authority() {
    let fixture = crate::replay_tests::fixture();
    let prepared = crate::replay_tests::inspect_pre_run(&fixture).expect("prepared claim");
    let lineage = crate::replay_tests::expected_lineage(&prepared);
    let matched_claim =
        inspect_prepared_pre_run_replay_claim_lineage(prepared, &lineage).expect("matched claim");
    let expected = expected_platform_freeze(&fixture.final_freeze);
    let matched =
        match_final_freeze_to_prepared_claim(fixture.final_freeze, matched_claim, &expected)
            .expect("matched final freeze, plan, and claim");

    assert_eq!(matched.platform_scope(), ReplayPlatformScopeV1::Nix);
    assert!(matched.typed_final_freeze_bound());
    assert_eq!(
        matched
            .final_freeze()
            .canonical_source()
            .archive_recipe_sha256,
        digest('2')
    );
    assert_eq!(
        matched.final_freeze().platform_artifacts()[0].build_recipe_sha256,
        digest('a')
    );
    assert!(!matched.authorizes_live());
    assert!(!matched.actual_artifact_bytes_observed());
    assert!(!matched.source_provenance_observed());
    assert!(!matched.toolchain_provenance_observed());
    assert!(!matched.durable_claim_observed());
    assert!(!matched.wall_clock_verified());
    assert!(!matched.launch_grant_available());
    assert!(!matched.launch_performed());
    assert!(!matched.receipt_emitted());
}

#[test]
fn matcher_rejects_every_complete_signature_lineage_transplant() {
    type Mutation = fn(&mut VerifiedDetachedSignatureInspectionV1);
    let mutations: [(&str, Mutation); 8] = [
        ("root", |signature| {
            signature.trust_root_id = "other-test-root-v1".to_string()
        }),
        ("revision", |signature| signature.trust_root_revision += 1),
        ("policy", |signature| {
            signature.trust_policy_sha256 = digest('1')
        }),
        ("key", |signature| {
            signature.signer_key_id = "other-final-freeze-key-v1".to_string()
        }),
        ("manifest", |signature| {
            signature.manifest_sha256 = digest('2')
        }),
        ("payload", |signature| {
            signature.payload_sha256 = digest('3')
        }),
        ("frame", |signature| {
            signature.signed_frame_sha256 = digest('4')
        }),
        ("signature", |signature| {
            signature.signature_sha256 = digest('5')
        }),
    ];
    for (label, mutate) in mutations {
        let claim_fixture = crate::replay_tests::fixture();
        let prepared =
            crate::replay_tests::inspect_pre_run(&claim_fixture).expect("prepared original claim");
        let lineage = crate::replay_tests::expected_lineage(&prepared);
        let matched_claim = inspect_prepared_pre_run_replay_claim_lineage(prepared, &lineage)
            .expect("matched original claim");
        let final_freeze = inspected_test_final_freeze_with_signature_mutation(mutate);
        let expected = expected_platform_freeze(&final_freeze);
        assert!(
            match_final_freeze_to_prepared_claim(final_freeze, matched_claim, &expected).is_err(),
            "transplanted {label} must fail"
        );
    }

    let claim_fixture = crate::replay_tests::fixture();
    let prepared = crate::replay_tests::inspect_pre_run(&claim_fixture).expect("prepared claim");
    let lineage = crate::replay_tests::expected_lineage(&prepared);
    let matched_claim =
        inspect_prepared_pre_run_replay_claim_lineage(prepared, &lineage).expect("matched claim");
    let final_freeze = inspected_test_final_freeze_for_profile("other-final-freeze-v1");
    let expected = expected_platform_freeze(&final_freeze);
    assert!(
        match_final_freeze_to_prepared_claim(final_freeze, matched_claim, &expected).is_err(),
        "transplanted profile must fail"
    );
}

#[test]
fn matcher_rejects_common_source_artifact_and_material_projection_drift() {
    reject_expected_mutation(|expected| expected.platform_scope = ReplayPlatformScopeV1::MacOs);
    reject_expected_mutation(|expected| expected.final_tooling.head = "1".repeat(40));
    reject_expected_mutation(|expected| expected.final_tooling.tree = "2".repeat(40));
    reject_expected_mutation(|expected| {
        expected.final_artifact_freeze_payload_sha256 = digest('3')
    });
    reject_expected_mutation(|expected| {
        expected.final_artifact_freeze_profile_id = "other-final-freeze-v1".to_string()
    });
    reject_expected_mutation(|expected| expected.canonical_source.archive.sha256 = digest('4'));
    reject_expected_mutation(|expected| expected.canonical_source.archive.byte_count += 1);
    reject_expected_mutation(|expected| {
        expected.canonical_source.archive.mode = "0400".to_string()
    });
    reject_expected_mutation(|expected| {
        expected.canonical_source.source_tree_manifest_sha256 = digest('5')
    });
    reject_expected_mutation(|expected| expected.platform_artifacts.swap(0, 1));
    reject_expected_mutation(|expected| {
        expected.platform_artifacts[0].role_id = "other_collector".to_string()
    });
    reject_expected_mutation(|expected| expected.platform_artifacts.pop().map(drop).unwrap());
    reject_expected_mutation(|expected| expected.named_materials.swap(0, 1));
    reject_expected_mutation(|expected| expected.named_materials.pop().map(drop).unwrap());

    for index in 0..6 {
        reject_expected_mutation(|expected| {
            expected.platform_artifacts[index].artifact.sha256 = digest('f')
        });
        reject_expected_mutation(|expected| {
            expected.platform_artifacts[index].artifact.byte_count += 1
        });
        reject_expected_mutation(|expected| {
            expected.platform_artifacts[index].artifact.mode = "0400".to_string()
        });
    }
    for index in 0..3 {
        reject_expected_mutation(|expected| expected.named_materials[index].sha256 = digest('9'));
    }
}

pub(crate) fn inspected_test_final_freeze() -> InspectedFinalArtifactFreezeV1 {
    inspected_test_final_freeze_for_profile(TEST_FINAL_FREEZE_PROFILE_ID)
}

fn inspected_test_final_freeze_for_profile(profile_id: &str) -> InspectedFinalArtifactFreezeV1 {
    let ancestry = crate::tests::inspect_test_structural_ancestry();
    let mut payload = test_payload(&ancestry);
    payload.profile_id = profile_id.to_string();
    let signature = test_signature(
        DetachedSignatureRoleV1::FinalArtifactFreeze,
        profile_id,
        serde_json::to_vec(&payload).expect("test final-freeze payload"),
    );
    inspect_final_artifact_freeze_semantics(ancestry, signature).expect("typed test final freeze")
}

fn inspected_test_final_freeze_with_signature_mutation(
    mutate: impl FnOnce(&mut VerifiedDetachedSignatureInspectionV1),
) -> InspectedFinalArtifactFreezeV1 {
    let ancestry = crate::tests::inspect_test_structural_ancestry();
    let payload = test_payload(&ancestry);
    let mut signature = test_signature(
        DetachedSignatureRoleV1::FinalArtifactFreeze,
        TEST_FINAL_FREEZE_PROFILE_ID,
        serde_json::to_vec(&payload).expect("test final-freeze payload"),
    );
    mutate(&mut signature);
    inspect_final_artifact_freeze_semantics(ancestry, signature)
        .expect("typed test final freeze with signature mutation")
}

fn expected_platform_freeze(
    freeze: &InspectedFinalArtifactFreezeV1,
) -> ExpectedPlatformArtifactFreezeV1 {
    ExpectedPlatformArtifactFreezeV1 {
        canonical_source: ExpectedCanonicalSourceFreezeV1 {
            archive: freeze.canonical_source().archive.clone(),
            source_tree_manifest_sha256: freeze
                .canonical_source()
                .source_tree_manifest_sha256
                .clone(),
        },
        final_artifact_freeze_payload_sha256: freeze.payload_sha256().to_string(),
        final_artifact_freeze_profile_id: freeze.profile_id().to_string(),
        final_tooling: freeze.final_tooling().clone(),
        named_materials: freeze
            .named_materials()
            .iter()
            .filter(|material| material.platform_scope == ReplayPlatformScopeV1::Nix)
            .cloned()
            .collect(),
        platform_artifacts: freeze
            .platform_artifacts()
            .iter()
            .filter(|artifact| artifact.platform_scope == ReplayPlatformScopeV1::Nix)
            .map(|artifact| ExpectedPlatformArtifactBytesV1 {
                artifact: artifact.artifact.clone(),
                platform_scope: artifact.platform_scope,
                role_id: artifact.role_id.clone(),
            })
            .collect(),
        platform_scope: ReplayPlatformScopeV1::Nix,
    }
}

fn reject_payload_mutation(mutate: impl FnOnce(&mut FinalArtifactFreezePayloadV1)) {
    let ancestry = crate::tests::inspect_test_structural_ancestry();
    let mut payload = test_payload(&ancestry);
    mutate(&mut payload);
    let signature = test_signature(
        DetachedSignatureRoleV1::FinalArtifactFreeze,
        TEST_FINAL_FREEZE_PROFILE_ID,
        serde_json::to_vec(&payload).expect("mutated final-freeze payload"),
    );
    assert!(inspect_final_artifact_freeze_semantics(ancestry, signature).is_err());
}

fn reject_expected_mutation(mutate: impl FnOnce(&mut ExpectedPlatformArtifactFreezeV1)) {
    let fixture = crate::replay_tests::fixture();
    let prepared = crate::replay_tests::inspect_pre_run(&fixture).expect("prepared claim");
    let lineage = crate::replay_tests::expected_lineage(&prepared);
    let matched_claim =
        inspect_prepared_pre_run_replay_claim_lineage(prepared, &lineage).expect("matched claim");
    let mut expected = expected_platform_freeze(&fixture.final_freeze);
    mutate(&mut expected);
    assert!(
        match_final_freeze_to_prepared_claim(fixture.final_freeze, matched_claim, &expected,)
            .is_err()
    );
}

fn inspect_exact_payload(
    ancestry: StructuralAncestryInspectionV1,
    exact_payload: Vec<u8>,
) -> Result<InspectedFinalArtifactFreezeV1, MnlTrustError> {
    let signature = test_signature(
        DetachedSignatureRoleV1::FinalArtifactFreeze,
        TEST_FINAL_FREEZE_PROFILE_ID,
        exact_payload,
    );
    inspect_final_artifact_freeze_semantics(ancestry, signature)
}

fn test_payload(ancestry: &StructuralAncestryInspectionV1) -> FinalArtifactFreezePayloadV1 {
    let artifact = |role_id: &str, character: char, mode: &str| PlatformArtifactFreezeV1 {
        artifact: frozen_bytes(character, mode),
        build_recipe_sha256: digest('a'),
        platform_scope: ReplayPlatformScopeV1::Nix,
        role_id: role_id.to_string(),
        role_source_manifest_sha256: digest('b'),
        toolchain_manifest_sha256: digest('c'),
    };
    FinalArtifactFreezePayloadV1 {
        ancestry_commit_count: ancestry.commit_count() as u64,
        ancestry_manifest_sha256: ancestry.manifest_sha256().to_string(),
        ancestry_raw_objects_sha256: ancestry.raw_objects_sha256().to_string(),
        canonical_source: CanonicalSourceFreezeV1 {
            archive: frozen_bytes('1', "0444"),
            archive_recipe_sha256: digest('2'),
            source_tree_manifest_sha256: digest('3'),
            toolchain_manifest_sha256: digest('4'),
        },
        final_tooling: ancestry.final_tooling().clone(),
        named_materials: vec![
            named_material("docker_image_config_id", 'd'),
            named_material("docker_image_manifest", 'e'),
            named_material("nix_store_seed_inventory", 'f'),
        ],
        phase_a_anchor: exact_phase_a_anchor(),
        platform_artifacts: vec![
            artifact("collector", '5', "0555"),
            artifact("driver", '6', "0555"),
            artifact("nix_store_seed_bundle", '7', "0444"),
            artifact("runner", '8', "0555"),
            artifact("seccomp_profile", '9', "0444"),
            artifact("verifier", 'a', "0555"),
        ],
        profile_id: TEST_FINAL_FREEZE_PROFILE_ID.to_string(),
        schema: FINAL_ARTIFACT_FREEZE_PAYLOAD_SCHEMA.to_string(),
    }
}

fn named_material(name: &str, character: char) -> NamedMaterialFreezeV1 {
    NamedMaterialFreezeV1 {
        name: name.to_string(),
        platform_scope: ReplayPlatformScopeV1::Nix,
        sha256: digest(character),
    }
}

fn frozen_bytes(character: char, mode: &str) -> FrozenArtifactBytesV1 {
    FrozenArtifactBytesV1 {
        byte_count: 4096,
        mode: mode.to_string(),
        sha256: digest(character),
    }
}

fn test_signature(
    role: DetachedSignatureRoleV1,
    profile_id: &str,
    payload: Vec<u8>,
) -> VerifiedDetachedSignatureInspectionV1 {
    crate::signature_tests::inspect_test_signature_payload(role, profile_id, payload)
        .expect("real test-policy detached signature inspection")
}

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}
