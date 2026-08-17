use serde::Serialize;
use static_assertions::assert_not_impl_any;

use crate::*;

const TEST_TRUST_ROOT_ID: &str = "hepta-mnl-test-only-signature-root-v1";
const TEST_PRE_RUN_PROFILE_ID: &str = "hepta-mnl-test-only-pre-run-v1";
// Must equal the sandbox closed-plan golden in hepta-nix-mnl-v1/run_plan_tests.rs.
const TEST_NIX_SANDBOX_CLOSED_PLAN_SHA256: &str =
    "a16f0ee59b131432c6f699b66ab5458eb61508e106b75104018898e7f8fe86a3";
const TEST_PRE_RUN_SLOT_GOLDEN: &str =
    "74de5bc5c1c44cfeedf74b5d9bc234937bb9dfb2d2d3cf27930bf46154a2a357";
const TEST_PRE_RUN_FULL_BINDING_GOLDEN: &str =
    "2012586cffc7f8dd3c98a7a06f22a072efdfe304e5dbef20fc74d5d4ec76a2fa";
const TEST_COPY_SLOT_GOLDEN: &str =
    "24c78cddc11ec907367b2d999195193469801fd506b7c9226ce524282448e510";
const TEST_COPY_FULL_BINDING_GOLDEN: &str =
    "48cf431ff2a0e6849f75666e563abe37d12565e99390a459f00589ac1b787d3c";

assert_not_impl_any!(PreparedPreRunReplayClaimV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);
assert_not_impl_any!(PreparedCopyAckReplayClaimV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);
assert_not_impl_any!(MatchedPreparedPreRunReplayClaimInspectionV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);

#[test]
fn replay_namespace_and_platform_wire_ids_are_exact() {
    let namespaces = [
        (ReplayClaimNamespaceV1::PreRunLaunch, "\"pre_run_launch\""),
        (
            ReplayClaimNamespaceV1::IndependentCopyAck,
            "\"independent_copy_ack\"",
        ),
    ];
    for (value, wire) in namespaces {
        assert_eq!(serde_json::to_string(&value).expect("namespace JSON"), wire);
    }
    let platforms = [
        (ReplayPlatformScopeV1::MacOs, "\"macos\""),
        (ReplayPlatformScopeV1::LinuxPhase1, "\"linux_phase1\""),
        (ReplayPlatformScopeV1::Nix, "\"nix\""),
    ];
    for (value, wire) in platforms {
        assert_eq!(serde_json::to_string(&value).expect("platform JSON"), wire);
    }
}

#[test]
fn pre_run_and_copy_claims_are_structural_and_non_authorizing() {
    let fixture = fixture();
    let prepared_pre_run = inspect_pre_run(&fixture).expect("pre-run replay shape");
    assert_eq!(
        prepared_pre_run.namespace(),
        ReplayClaimNamespaceV1::PreRunLaunch
    );
    assert_eq!(
        prepared_pre_run.platform_scope(),
        ReplayPlatformScopeV1::Nix
    );
    assert_eq!(prepared_pre_run.profile_id(), TEST_PRE_RUN_PROFILE_ID);
    assert_eq!(
        prepared_pre_run.generation_epoch_id(),
        "test-generation-epoch-v1"
    );
    assert_eq!(prepared_pre_run.not_before_unix_seconds(), 1_700_000_000);
    assert_eq!(prepared_pre_run.expires_at_unix_seconds(), 1_700_000_120);
    assert_eq!(prepared_pre_run.maximum_lifetime_seconds(), 300);
    assert_eq!(
        prepared_pre_run.platform_closed_run_plan_sha256(),
        TEST_NIX_SANDBOX_CLOSED_PLAN_SHA256
    );
    assert_eq!(prepared_pre_run.session_nonce_sha256(), digest('d'));
    assert_eq!(
        prepared_pre_run.final_leaf_name(),
        format!("{}.claim-v1", prepared_pre_run.replay_slot_sha256())
    );
    assert!(!prepared_pre_run.authorizes_live());
    assert!(!prepared_pre_run.durable_commit_observed());
    assert!(!prepared_pre_run.wall_clock_verified());

    let copy_wire = copy_wire(&fixture, &prepared_pre_run);
    let canonical_copy = serde_json::to_vec(&copy_wire).expect("copy claim JSON");
    let prepared_copy = inspect_canonical_copy_ack_replay_claim(&prepared_pre_run, &canonical_copy)
        .expect("copy replay shape");
    assert_eq!(
        prepared_copy.namespace(),
        ReplayClaimNamespaceV1::IndependentCopyAck
    );
    assert_eq!(prepared_copy.platform_scope(), ReplayPlatformScopeV1::Nix);
    assert_ne!(
        prepared_copy.copy_replay_store_identity_sha256(),
        prepared_pre_run.pre_run_replay_store_identity_sha256()
    );
    assert_eq!(
        prepared_copy.pre_run_replay_slot_sha256(),
        prepared_pre_run.replay_slot_sha256()
    );
    assert_eq!(
        prepared_copy.pre_run_full_binding_sha256(),
        prepared_pre_run.full_binding_sha256()
    );
    assert!(!prepared_copy.authorizes_live());
    assert!(!prepared_copy.durable_commit_observed());
    assert!(!prepared_copy.wall_clock_verified());
}

#[test]
fn replay_slot_and_full_binding_goldens_are_stable() {
    let fixture = fixture();
    let prepared_pre_run = inspect_pre_run(&fixture).expect("pre-run replay shape");
    assert_eq!(
        prepared_pre_run.replay_slot_sha256(),
        TEST_PRE_RUN_SLOT_GOLDEN
    );
    assert_eq!(
        prepared_pre_run.full_binding_sha256(),
        TEST_PRE_RUN_FULL_BINDING_GOLDEN
    );
    let copy_wire = copy_wire(&fixture, &prepared_pre_run);
    let prepared_copy = inspect_canonical_copy_ack_replay_claim(
        &prepared_pre_run,
        &serde_json::to_vec(&copy_wire).expect("copy claim JSON"),
    )
    .expect("copy replay shape");
    assert_eq!(prepared_copy.replay_slot_sha256(), TEST_COPY_SLOT_GOLDEN);
    assert_eq!(
        prepared_copy.full_binding_sha256(),
        TEST_COPY_FULL_BINDING_GOLDEN
    );
    assert!(
        !prepared_pre_run
            .record_bytes()
            .windows(prepared_pre_run.full_binding_sha256().len())
            .any(|window| window == prepared_pre_run.full_binding_sha256().as_bytes())
    );
}

#[test]
fn closed_run_plan_digest_changes_binding_without_changing_the_nonce_slot() {
    let original = fixture();
    let changed = fixture_with_signed_profile_mutation(|profile| {
        profile.platform_closed_run_plan_sha256 = digest('8');
    });
    let original = inspect_pre_run(&original).expect("original pre-run replay shape");
    let changed = inspect_pre_run(&changed).expect("changed-plan pre-run replay shape");

    assert_eq!(original.replay_slot_sha256(), changed.replay_slot_sha256());
    assert_ne!(
        original.platform_closed_run_plan_sha256(),
        changed.platform_closed_run_plan_sha256()
    );
    assert_ne!(
        original.full_binding_sha256(),
        changed.full_binding_sha256()
    );

    let original_fixture = fixture();
    let changed_fixture = fixture_with_signed_profile_mutation(|profile| {
        profile.platform_closed_run_plan_sha256 = digest('8');
    });
    let original_pre_run = inspect_pre_run(&original_fixture).expect("original prepared claim");
    let changed_pre_run = inspect_pre_run(&changed_fixture).expect("changed prepared claim");
    let original_copy_wire = copy_wire(&original_fixture, &original_pre_run);
    let changed_copy_wire = copy_wire(&changed_fixture, &changed_pre_run);
    let original_copy = inspect_canonical_copy_ack_replay_claim(
        &original_pre_run,
        &serde_json::to_vec(&original_copy_wire).expect("original copy claim"),
    )
    .expect("original copy binding");
    let changed_copy = inspect_canonical_copy_ack_replay_claim(
        &changed_pre_run,
        &serde_json::to_vec(&changed_copy_wire).expect("changed copy claim"),
    )
    .expect("changed copy binding");
    assert_eq!(
        original_copy.replay_slot_sha256(),
        changed_copy.replay_slot_sha256()
    );
    assert_ne!(
        original_copy.pre_run_full_binding_sha256(),
        changed_copy.pre_run_full_binding_sha256()
    );
    assert_ne!(
        original_copy.full_binding_sha256(),
        changed_copy.full_binding_sha256()
    );
}

#[test]
fn prepared_claim_lineage_join_is_exact_and_non_authorizing() {
    let fixture = fixture();
    let prepared = inspect_pre_run(&fixture).expect("prepared claim");
    let expected = expected_lineage(&prepared);
    let matched = inspect_prepared_pre_run_replay_claim_lineage(prepared, &expected)
        .expect("exact prepared-claim lineage");

    assert_eq!(
        matched.prepared_claim().platform_closed_run_plan_sha256(),
        expected.platform_closed_run_plan_sha256
    );
    assert_eq!(
        matched.prepared_claim().run_nonce_sha256(),
        expected.run_nonce_sha256
    );
    assert!(!matched.authorizes_live());
    assert!(!matched.durable_commit_observed());
    assert!(!matched.launch_grant_available());
}

#[test]
fn prepared_claim_lineage_join_rejects_every_transplant_axis() {
    macro_rules! reject {
        ($field:ident, $value:expr) => {{
            let fixture = fixture();
            let prepared = inspect_pre_run(&fixture).expect("prepared claim");
            let mut expected = expected_lineage(&prepared);
            expected.$field = $value;
            let error = inspect_prepared_pre_run_replay_claim_lineage(prepared, &expected)
                .expect_err(concat!("must reject changed ", stringify!($field)));
            assert!(error.to_string().contains("exact platform lineage"));
        }};
    }

    reject!(platform_scope, ReplayPlatformScopeV1::MacOs);
    reject!(platform_closed_run_plan_sha256, digest('8'));
    reject!(profile_id, "other-profile-v1".to_string());
    reject!(run_identity_sha256, digest('7'));
    reject!(run_nonce_sha256, digest('6'));
    reject!(boot_id_sha256, digest('5'));
    reject!(host_identity_sha256, digest('4'));
    reject!(challenge_nonce_sha256, digest('3'));
    reject!(final_artifact_freeze_payload_sha256, digest('2'));
    reject!(
        final_artifact_freeze_profile_id,
        "other-final-freeze-v1".to_string()
    );
}

#[test]
fn signed_pre_run_semantics_cannot_be_rewritten_after_inspection() {
    let fixture = fixture();
    let first = inspect_pre_run(&fixture).expect("first pre-run replay shape");
    let mut changed_wire = fixture.pre_run_wire.clone();
    changed_wire.boot_id_sha256 = digest('2');
    changed_wire.host_identity_sha256 = digest('3');
    changed_wire.session_nonce_sha256 = digest('4');
    changed_wire.run_nonce_sha256 = changed_wire.session_nonce_sha256.clone();
    changed_wire.run_identity_sha256 =
        derive_run_identity_sha256(&changed_wire.run_nonce_sha256, &changed_wire.boot_id_sha256)
            .expect("changed run identity");
    changed_wire.generation_epoch_id = "changed-epoch-v2".to_string();
    let error = inspect_canonical_pre_run_replay_claim(
        &fixture.final_freeze,
        &fixture.pre_run,
        &serde_json::to_vec(&changed_wire).expect("changed pre-run JSON"),
    )
    .expect_err("caller-authored semantics must not escape the signed payload");
    assert!(error.to_string().contains("exact signed profile semantics"));

    let first_copy_wire = copy_wire(&fixture, &first);
    let first_copy = inspect_canonical_copy_ack_replay_claim(
        &first,
        &serde_json::to_vec(&first_copy_wire).expect("first copy JSON"),
    )
    .expect("first copy binding");
    let mut changed_copy_wire = first_copy_wire;
    changed_copy_wire.sealed_bundle_sha256 = digest('4');
    changed_copy_wire.sealed_bundle_byte_count += 1;
    changed_copy_wire.destination_identity_sha256 = digest('5');
    let changed_copy = inspect_canonical_copy_ack_replay_claim(
        &first,
        &serde_json::to_vec(&changed_copy_wire).expect("changed copy JSON"),
    )
    .expect("changed copy binding");
    assert_eq!(
        first_copy.replay_slot_sha256(),
        changed_copy.replay_slot_sha256()
    );
    assert_ne!(
        first_copy.full_binding_sha256(),
        changed_copy.full_binding_sha256()
    );
}

#[test]
fn slot_domain_root_and_primary_nonce_are_separated() {
    let challenge = digest('c');
    let copy_nonce = digest('e');
    let pre =
        derive_pre_run_replay_slot_sha256(TEST_TRUST_ROOT_ID, &challenge).expect("pre-run slot");
    let copy =
        derive_copy_ack_replay_slot_sha256(TEST_TRUST_ROOT_ID, &challenge).expect("copy slot");
    let other_root =
        derive_pre_run_replay_slot_sha256("other-root-v1", &challenge).expect("other-root slot");
    let other_nonce = derive_pre_run_replay_slot_sha256(TEST_TRUST_ROOT_ID, &copy_nonce)
        .expect("other-nonce slot");
    assert_ne!(pre, copy);
    assert_ne!(pre, other_root);
    assert_ne!(pre, other_nonce);
}

#[test]
fn pre_run_claim_rejects_role_policy_binding_nonce_and_time_transplants() {
    let fixture = fixture();
    let wrong_role = inspection(
        DetachedSignatureRoleV1::SupervisorSeal,
        TEST_PRE_RUN_PROFILE_ID,
        fixture.pre_run.exact_payload_bytes().to_vec(),
    );
    assert!(
        inspect_canonical_pre_run_replay_claim(
            &fixture.final_freeze,
            &wrong_role,
            &fixture.canonical_pre_run,
        )
        .is_err()
    );

    let mut mutations = Vec::new();
    let mut value = fixture.pre_run_wire.clone();
    value.pre_run_profile_payload_sha256 = digest('1');
    mutations.push(value);
    let mut value = fixture.pre_run_wire.clone();
    value.trust_root_revision += 1;
    mutations.push(value);
    let mut value = fixture.pre_run_wire.clone();
    value.namespace = ReplayClaimNamespaceV1::IndependentCopyAck;
    mutations.push(value);
    let mut value = fixture.pre_run_wire.clone();
    value.challenge_nonce_sha256 = value.session_nonce_sha256.clone();
    value.replay_slot_sha256 =
        derive_pre_run_replay_slot_sha256(&value.trust_root_id, &value.challenge_nonce_sha256)
            .expect("mutated slot");
    mutations.push(value);
    let mut value = fixture.pre_run_wire.clone();
    value.run_nonce_sha256 = digest('1');
    mutations.push(value);
    let mut value = fixture.pre_run_wire.clone();
    value.expires_at_unix_seconds = value.not_before_unix_seconds;
    mutations.push(value);
    let mut value = fixture.pre_run_wire.clone();
    value.maximum_lifetime_seconds = MAX_SIGNED_FRESHNESS_LIFETIME_SECONDS + 1;
    mutations.push(value);
    let mut value = fixture.pre_run_wire.clone();
    value.replay_slot_sha256 = digest('6');
    mutations.push(value);

    for mutation in mutations {
        assert!(
            inspect_canonical_pre_run_replay_claim(
                &fixture.final_freeze,
                &fixture.pre_run,
                &serde_json::to_vec(&mutation).expect("mutated pre-run JSON"),
            )
            .is_err()
        );
    }
}

#[test]
fn signed_profile_rejects_unrepresentable_time_and_store_alias_directly() {
    let time = fixture_with_signed_profile_mutation(|profile| {
        profile.not_before_unix_seconds = i64::MAX as u64 + 1;
        profile.expires_at_unix_seconds = i64::MAX as u64 + 2;
    });
    let error = inspect_pre_run(&time).expect_err("signed time_t overflow must fail");
    assert!(error.to_string().contains("64-bit system clock"));

    let alias = fixture_with_signed_profile_mutation(|profile| {
        profile.copy_replay_store_identity_sha256 =
            profile.pre_run_replay_store_identity_sha256.clone();
    });
    let error = inspect_pre_run(&alias).expect_err("signed replay-store alias must fail");
    assert!(error.to_string().contains("stores are not independent"));

    for invalid_digest in ["0".repeat(64), "A".repeat(64), "a".repeat(63)] {
        let invalid = fixture_with_signed_profile_mutation(|profile| {
            profile.platform_closed_run_plan_sha256 = invalid_digest;
        });
        let error = inspect_pre_run(&invalid).expect_err("invalid signed plan digest must fail");
        assert!(error.to_string().contains("platform closed run plan"));
    }
}

#[test]
fn every_pre_run_wire_binding_is_independently_rejected_when_transplanted() {
    let fixture = fixture();
    macro_rules! reject {
        ($field:ident, $value:expr) => {{
            let mut claim = fixture.pre_run_wire.clone();
            claim.$field = $value;
            assert!(
                inspect_canonical_pre_run_replay_claim(
                    &fixture.final_freeze,
                    &fixture.pre_run,
                    &serde_json::to_vec(&claim).expect("canonical transplanted pre-run claim"),
                )
                .is_err(),
                "transplanted {} must fail",
                stringify!($field),
            );
        }};
    }

    reject!(
        authorized_copy_ack_signer_key_id,
        "other-copy-key-v1".to_string()
    );
    reject!(boot_id_sha256, digest('2'));
    reject!(challenge_nonce_sha256, digest('2'));
    reject!(copy_replay_store_identity_sha256, digest('2'));
    reject!(copy_session_nonce_sha256, digest('2'));
    reject!(expires_at_unix_seconds, 1_700_000_121);
    reject!(final_artifact_freeze_manifest_sha256, digest('2'));
    reject!(final_artifact_freeze_payload_sha256, digest('2'));
    reject!(
        final_artifact_freeze_profile_id,
        "other-final-profile-v1".to_string()
    );
    reject!(final_artifact_freeze_signature_sha256, digest('2'));
    reject!(final_artifact_freeze_signed_frame_sha256, digest('2'));
    reject!(
        final_artifact_freeze_signer_key_id,
        "other-final-key-v1".to_string()
    );
    reject!(generation_epoch_id, "other-generation-v1".to_string());
    reject!(host_identity_sha256, digest('2'));
    reject!(maximum_lifetime_seconds, 301);
    reject!(namespace, ReplayClaimNamespaceV1::IndependentCopyAck);
    reject!(not_before_unix_seconds, 1_700_000_001);
    reject!(platform_closed_run_plan_sha256, digest('2'));
    reject!(platform_scope, ReplayPlatformScopeV1::MacOs);
    reject!(pre_run_profile_manifest_sha256, digest('2'));
    reject!(pre_run_profile_payload_sha256, digest('2'));
    reject!(pre_run_profile_signature_sha256, digest('2'));
    reject!(pre_run_profile_signed_frame_sha256, digest('2'));
    reject!(
        pre_run_profile_signer_key_id,
        "other-pre-run-key-v1".to_string()
    );
    reject!(profile_id, "other-pre-run-profile-v1".to_string());
    reject!(replay_slot_sha256, digest('2'));
    reject!(pre_run_replay_store_identity_sha256, digest('2'));
    reject!(run_identity_sha256, digest('2'));
    reject!(run_nonce_sha256, digest('2'));
    reject!(schema, "other_pre_run_schema_v1".to_string());
    reject!(session_nonce_sha256, digest('2'));
    reject!(trust_policy_sha256, digest('2'));
    reject!(trust_root_id, "other-root-v1".to_string());
    reject!(trust_root_revision, 2);
}

#[test]
fn copy_claim_rejects_pre_run_bundle_destination_and_slot_transplants() {
    let fixture = fixture();
    let prepared_pre_run = inspect_pre_run(&fixture).expect("pre-run replay shape");
    let copy = copy_wire(&fixture, &prepared_pre_run);
    let mut mutations = Vec::new();
    let mut value = copy.clone();
    value.pre_run_full_binding_sha256 = digest('1');
    mutations.push(value);
    let mut value = copy.clone();
    value.final_artifact_freeze_manifest_sha256 = digest('2');
    mutations.push(value);
    let mut value = copy.clone();
    value.pre_run_profile_signature_sha256 = digest('3');
    mutations.push(value);
    let mut value = copy.clone();
    value.authorized_copy_ack_signer_key_id = "other-copy-ack-key-v1".to_string();
    mutations.push(value);
    let mut value = copy.clone();
    value.copy_replay_store_identity_sha256 = digest('4');
    mutations.push(value);
    let mut value = copy.clone();
    value.copy_replay_store_identity_sha256 = fixture
        .pre_run_wire
        .pre_run_replay_store_identity_sha256
        .clone();
    mutations.push(value);
    let mut value = copy.clone();
    value.destination_identity_sha256 = fixture.pre_run_wire.host_identity_sha256;
    mutations.push(value);
    let mut value = copy.clone();
    value.copy_session_nonce_sha256 = digest('2');
    value.replay_slot_sha256 =
        derive_copy_ack_replay_slot_sha256(&value.trust_root_id, &value.copy_session_nonce_sha256)
            .expect("mutated copy slot");
    mutations.push(value);
    let mut value = copy.clone();
    value.sealed_bundle_byte_count = 0;
    mutations.push(value);
    let mut value = copy.clone();
    value.destination_failure_domain_id = "Bad Domain".to_string();
    mutations.push(value);
    let mut value = copy;
    value.replay_slot_sha256 = digest('3');
    mutations.push(value);

    for mutation in mutations {
        assert!(
            inspect_canonical_copy_ack_replay_claim(
                &prepared_pre_run,
                &serde_json::to_vec(&mutation).expect("mutated copy JSON"),
            )
            .is_err()
        );
    }
}

#[test]
fn every_copy_preauthorization_binding_is_independently_rejected_when_transplanted() {
    let fixture = fixture();
    let prepared = inspect_pre_run(&fixture).expect("prepared pre-run claim");
    let original = copy_wire(&fixture, &prepared);
    macro_rules! reject {
        ($field:ident, $value:expr) => {{
            let mut claim = original.clone();
            claim.$field = $value;
            assert!(
                inspect_canonical_copy_ack_replay_claim(
                    &prepared,
                    &serde_json::to_vec(&claim).expect("canonical transplanted copy claim"),
                )
                .is_err(),
                "transplanted {} must fail",
                stringify!($field),
            );
        }};
    }

    reject!(
        authorized_copy_ack_signer_key_id,
        "other-copy-key-v1".to_string()
    );
    reject!(boot_id_sha256, digest('2'));
    reject!(copy_session_nonce_sha256, digest('2'));
    reject!(copy_replay_store_identity_sha256, digest('2'));
    reject!(final_artifact_freeze_manifest_sha256, digest('2'));
    reject!(final_artifact_freeze_payload_sha256, digest('2'));
    reject!(
        final_artifact_freeze_profile_id,
        "other-final-profile-v1".to_string()
    );
    reject!(final_artifact_freeze_signature_sha256, digest('2'));
    reject!(final_artifact_freeze_signed_frame_sha256, digest('2'));
    reject!(
        final_artifact_freeze_signer_key_id,
        "other-final-key-v1".to_string()
    );
    reject!(host_identity_sha256, digest('2'));
    reject!(namespace, ReplayClaimNamespaceV1::PreRunLaunch);
    reject!(platform_scope, ReplayPlatformScopeV1::MacOs);
    reject!(pre_run_full_binding_sha256, digest('2'));
    reject!(pre_run_profile_manifest_sha256, digest('2'));
    reject!(pre_run_profile_payload_sha256, digest('2'));
    reject!(pre_run_profile_signature_sha256, digest('2'));
    reject!(pre_run_profile_signed_frame_sha256, digest('2'));
    reject!(
        pre_run_profile_signer_key_id,
        "other-pre-run-key-v1".to_string()
    );
    reject!(pre_run_replay_slot_sha256, digest('2'));
    reject!(profile_id, "other-pre-run-profile-v1".to_string());
    reject!(replay_slot_sha256, digest('2'));
    reject!(run_identity_sha256, digest('2'));
    reject!(run_nonce_sha256, digest('2'));
    reject!(schema, "other_copy_schema_v1".to_string());
    reject!(trust_policy_sha256, digest('2'));
    reject!(trust_root_id, "other-root-v1".to_string());
    reject!(trust_root_revision, 2);
}

#[test]
fn replay_claims_require_exact_canonical_bounded_json() {
    let fixture = fixture();
    let mut trailing = fixture.canonical_pre_run.clone();
    trailing.push(b'\n');
    assert!(
        inspect_canonical_pre_run_replay_claim(&fixture.final_freeze, &fixture.pre_run, &trailing,)
            .is_err()
    );
    let pretty = serde_json::to_vec_pretty(&fixture.pre_run_wire).expect("pretty pre-run JSON");
    assert!(
        inspect_canonical_pre_run_replay_claim(&fixture.final_freeze, &fixture.pre_run, &pretty,)
            .is_err()
    );
    let mut unknown = serde_json::to_value(&fixture.pre_run_wire).expect("pre-run value");
    unknown["authority"] = serde_json::json!(true);
    assert!(
        inspect_canonical_pre_run_replay_claim(
            &fixture.final_freeze,
            &fixture.pre_run,
            &serde_json::to_vec(&unknown).expect("unknown-field JSON"),
        )
        .is_err()
    );
    let oversized = vec![b'x'; MAX_REPLAY_CLAIM_BYTES + 1];
    assert!(
        inspect_canonical_pre_run_replay_claim(
            &fixture.final_freeze,
            &fixture.pre_run,
            &oversized,
        )
        .is_err()
    );
}

pub(crate) struct ReplayFixture {
    pub(crate) canonical_pre_run: Vec<u8>,
    pub(crate) final_freeze: InspectedFinalArtifactFreezeV1,
    pub(crate) pre_run: VerifiedDetachedSignatureInspectionV1,
    pub(crate) pre_run_wire: PreRunReplayClaimWireV1,
}

pub(crate) fn fixture() -> ReplayFixture {
    fixture_with_signed_profile_mutation(|_| {})
}

fn fixture_with_signed_profile_mutation(
    mutate: impl FnOnce(&mut SignedPreRunReplayProfileV1),
) -> ReplayFixture {
    let final_freeze = crate::final_freeze_tests::inspected_test_final_freeze();
    let final_freeze_signature = final_freeze.signature_inspection();
    let challenge_nonce_sha256 = digest('c');
    let session_nonce_sha256 = digest('d');
    let copy_session_nonce_sha256 = digest('e');
    let boot_id_sha256 = digest('b');
    let replay_slot_sha256 =
        derive_pre_run_replay_slot_sha256(TEST_TRUST_ROOT_ID, &challenge_nonce_sha256)
            .expect("pre-run replay slot");
    let run_identity_sha256 =
        derive_run_identity_sha256(&session_nonce_sha256, &boot_id_sha256).expect("run identity");
    let mut signed_profile = SignedPreRunReplayProfileV1 {
        authorized_copy_ack_signer_key_id: "hepta-mnl-test-copy-ack-key-v1".to_string(),
        boot_id_sha256,
        challenge_nonce_sha256,
        copy_replay_store_identity_sha256: digest('6'),
        copy_session_nonce_sha256,
        expires_at_unix_seconds: 1_700_000_120,
        final_artifact_freeze_manifest_sha256: final_freeze_signature.manifest_sha256().to_string(),
        final_artifact_freeze_payload_sha256: final_freeze_signature.payload_sha256().to_string(),
        final_artifact_freeze_profile_id: final_freeze_signature.profile_id().to_string(),
        final_artifact_freeze_signature_sha256: final_freeze_signature
            .signature_sha256()
            .to_string(),
        final_artifact_freeze_signed_frame_sha256: final_freeze_signature
            .signed_frame_sha256()
            .to_string(),
        final_artifact_freeze_signer_key_id: final_freeze_signature.signer_key_id().to_string(),
        generation_epoch_id: "test-generation-epoch-v1".to_string(),
        host_identity_sha256: digest('f'),
        maximum_lifetime_seconds: 300,
        not_before_unix_seconds: 1_700_000_000,
        platform_closed_run_plan_sha256: TEST_NIX_SANDBOX_CLOSED_PLAN_SHA256.to_string(),
        platform_scope: ReplayPlatformScopeV1::Nix,
        pre_run_replay_store_identity_sha256: digest('1'),
        profile_id: TEST_PRE_RUN_PROFILE_ID.to_string(),
        run_identity_sha256,
        run_nonce_sha256: session_nonce_sha256.clone(),
        schema: SIGNED_PRE_RUN_REPLAY_PROFILE_SCHEMA.to_string(),
        session_nonce_sha256,
    };
    mutate(&mut signed_profile);
    let pre_run = inspection(
        DetachedSignatureRoleV1::PreRunProfile,
        TEST_PRE_RUN_PROFILE_ID,
        serde_json::to_vec(&signed_profile).expect("canonical signed pre-run profile"),
    );
    let pre_run_wire = PreRunReplayClaimWireV1 {
        authorized_copy_ack_signer_key_id: signed_profile.authorized_copy_ack_signer_key_id.clone(),
        boot_id_sha256: signed_profile.boot_id_sha256.clone(),
        challenge_nonce_sha256: signed_profile.challenge_nonce_sha256.clone(),
        copy_replay_store_identity_sha256: signed_profile.copy_replay_store_identity_sha256.clone(),
        copy_session_nonce_sha256: signed_profile.copy_session_nonce_sha256.clone(),
        expires_at_unix_seconds: signed_profile.expires_at_unix_seconds,
        final_artifact_freeze_manifest_sha256: signed_profile
            .final_artifact_freeze_manifest_sha256
            .clone(),
        final_artifact_freeze_payload_sha256: signed_profile
            .final_artifact_freeze_payload_sha256
            .clone(),
        final_artifact_freeze_profile_id: signed_profile.final_artifact_freeze_profile_id.clone(),
        final_artifact_freeze_signature_sha256: signed_profile
            .final_artifact_freeze_signature_sha256
            .clone(),
        final_artifact_freeze_signed_frame_sha256: signed_profile
            .final_artifact_freeze_signed_frame_sha256
            .clone(),
        final_artifact_freeze_signer_key_id: signed_profile
            .final_artifact_freeze_signer_key_id
            .clone(),
        generation_epoch_id: signed_profile.generation_epoch_id.clone(),
        host_identity_sha256: signed_profile.host_identity_sha256.clone(),
        maximum_lifetime_seconds: signed_profile.maximum_lifetime_seconds,
        namespace: ReplayClaimNamespaceV1::PreRunLaunch,
        not_before_unix_seconds: signed_profile.not_before_unix_seconds,
        platform_closed_run_plan_sha256: signed_profile.platform_closed_run_plan_sha256.clone(),
        platform_scope: signed_profile.platform_scope,
        pre_run_profile_manifest_sha256: pre_run.manifest_sha256().to_string(),
        pre_run_profile_payload_sha256: pre_run.payload_sha256().to_string(),
        pre_run_profile_signature_sha256: pre_run.signature_sha256().to_string(),
        pre_run_profile_signed_frame_sha256: pre_run.signed_frame_sha256().to_string(),
        pre_run_profile_signer_key_id: pre_run.signer_key_id().to_string(),
        profile_id: pre_run.profile_id().to_string(),
        replay_slot_sha256,
        pre_run_replay_store_identity_sha256: signed_profile
            .pre_run_replay_store_identity_sha256
            .clone(),
        run_identity_sha256: signed_profile.run_identity_sha256.clone(),
        run_nonce_sha256: signed_profile.run_nonce_sha256.clone(),
        schema: PRE_RUN_REPLAY_CLAIM_SCHEMA.to_string(),
        session_nonce_sha256: signed_profile.session_nonce_sha256,
        trust_policy_sha256: pre_run.trust_policy_sha256().to_string(),
        trust_root_id: pre_run.trust_root_id().to_string(),
        trust_root_revision: pre_run.trust_root_revision(),
    };
    let canonical_pre_run = serde_json::to_vec(&pre_run_wire).expect("canonical pre-run JSON");
    ReplayFixture {
        canonical_pre_run,
        final_freeze,
        pre_run,
        pre_run_wire,
    }
}

pub(crate) fn inspect_pre_run(
    fixture: &ReplayFixture,
) -> Result<PreparedPreRunReplayClaimV1, MnlTrustError> {
    inspect_canonical_pre_run_replay_claim(
        &fixture.final_freeze,
        &fixture.pre_run,
        &fixture.canonical_pre_run,
    )
}

pub(crate) fn expected_lineage(
    prepared: &PreparedPreRunReplayClaimV1,
) -> ExpectedPreparedPreRunReplayClaimLineageV1 {
    ExpectedPreparedPreRunReplayClaimLineageV1 {
        boot_id_sha256: prepared.boot_id_sha256().to_string(),
        challenge_nonce_sha256: prepared.challenge_nonce_sha256().to_string(),
        final_artifact_freeze_payload_sha256: prepared
            .final_artifact_freeze_payload_sha256()
            .to_string(),
        final_artifact_freeze_profile_id: prepared.final_artifact_freeze_profile_id().to_string(),
        host_identity_sha256: prepared.host_identity_sha256().to_string(),
        platform_closed_run_plan_sha256: prepared.platform_closed_run_plan_sha256().to_string(),
        platform_scope: prepared.platform_scope(),
        profile_id: prepared.profile_id().to_string(),
        run_identity_sha256: prepared.run_identity_sha256().to_string(),
        run_nonce_sha256: prepared.run_nonce_sha256().to_string(),
    }
}

fn copy_wire(
    fixture: &ReplayFixture,
    prepared_pre_run: &PreparedPreRunReplayClaimV1,
) -> CopyAckReplayClaimWireV1 {
    let wire = &fixture.pre_run_wire;
    CopyAckReplayClaimWireV1 {
        authorized_copy_ack_signer_key_id: wire.authorized_copy_ack_signer_key_id.clone(),
        boot_id_sha256: wire.boot_id_sha256.clone(),
        copy_session_nonce_sha256: wire.copy_session_nonce_sha256.clone(),
        copy_replay_store_identity_sha256: wire.copy_replay_store_identity_sha256.clone(),
        destination_failure_domain_id: "test-independent-copy-domain-v1".to_string(),
        destination_identity_sha256: digest('7'),
        final_artifact_freeze_manifest_sha256: wire.final_artifact_freeze_manifest_sha256.clone(),
        final_artifact_freeze_payload_sha256: wire.final_artifact_freeze_payload_sha256.clone(),
        final_artifact_freeze_profile_id: wire.final_artifact_freeze_profile_id.clone(),
        final_artifact_freeze_signature_sha256: wire.final_artifact_freeze_signature_sha256.clone(),
        final_artifact_freeze_signed_frame_sha256: wire
            .final_artifact_freeze_signed_frame_sha256
            .clone(),
        final_artifact_freeze_signer_key_id: wire.final_artifact_freeze_signer_key_id.clone(),
        host_identity_sha256: wire.host_identity_sha256.clone(),
        namespace: ReplayClaimNamespaceV1::IndependentCopyAck,
        platform_scope: wire.platform_scope,
        pre_run_full_binding_sha256: prepared_pre_run.full_binding_sha256().to_string(),
        pre_run_profile_manifest_sha256: wire.pre_run_profile_manifest_sha256.clone(),
        pre_run_profile_payload_sha256: wire.pre_run_profile_payload_sha256.clone(),
        pre_run_profile_signature_sha256: wire.pre_run_profile_signature_sha256.clone(),
        pre_run_profile_signed_frame_sha256: wire.pre_run_profile_signed_frame_sha256.clone(),
        pre_run_profile_signer_key_id: wire.pre_run_profile_signer_key_id.clone(),
        pre_run_replay_slot_sha256: prepared_pre_run.replay_slot_sha256().to_string(),
        profile_id: wire.profile_id.clone(),
        replay_slot_sha256: derive_copy_ack_replay_slot_sha256(
            &wire.trust_root_id,
            &wire.copy_session_nonce_sha256,
        )
        .expect("copy replay slot"),
        run_identity_sha256: wire.run_identity_sha256.clone(),
        run_nonce_sha256: wire.run_nonce_sha256.clone(),
        schema: COPY_ACK_REPLAY_CLAIM_SCHEMA.to_string(),
        sealed_bundle_byte_count: 4096,
        sealed_bundle_sha256: digest('8'),
        trust_policy_sha256: wire.trust_policy_sha256.clone(),
        trust_root_id: wire.trust_root_id.clone(),
        trust_root_revision: wire.trust_root_revision,
    }
}

fn inspection(
    role: DetachedSignatureRoleV1,
    profile_id: &str,
    payload_bytes: Vec<u8>,
) -> VerifiedDetachedSignatureInspectionV1 {
    crate::signature_tests::inspect_test_signature_payload(role, profile_id, payload_bytes)
        .expect("real test-policy detached signature inspection")
}

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}
