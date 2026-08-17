use ed25519_dalek::Signature;
use ed25519_dalek::Signer as _;
use ed25519_dalek::SigningKey;
use ed25519_dalek::VerifyingKey;
use serde::Serialize;
use sha2::Digest;
use static_assertions::assert_not_impl_any;
use static_assertions::const_assert;

use crate::DetachedSignatureManifestV1;
use crate::DetachedSignatureRoleV1;
use crate::RawDetachedEd25519SignatureV1;
use crate::VerifiedDetachedSignatureInspectionV1;
use crate::signature::SignaturePolicyEntryMaterialV1;
use crate::signature::SignaturePolicyMaterialV1;
use crate::signature::detached_signature_frame;
use crate::signature::inspect_canonical_detached_signature_with_policy;
use crate::signature::signature_policy_sha256;
use crate::*;

const TEST_TRUST_ROOT_ID: &str = "hepta-mnl-test-only-signature-root-v1";
const TEST_PROFILE_ID: &str = "hepta-mnl-test-only-profile-v1";
const TEST_POLICY_SHA256_GOLDEN: &str =
    "32a59f053f3c62b78220cb210ef73674041603d2f989bf3cd96a8fe19acaa911";
const TEST_MANIFEST_SHA256_GOLDEN: &str =
    "e31a90245904f77d8cc6b860378c69b0c0c73a0f55c445c51c14b7404c01c077";
const TEST_SIGNED_FRAME_SHA256_GOLDEN: &str =
    "ca6977edbe453fd8d4e39236eeb771de155ae9fd61f96e088ebc4a40f6231090";
const TEST_SIGNATURE_SHA256_GOLDEN: &str =
    "3c0e6ee3752773e94df4575a7294ed7fb1da79840fdda1bfbaec11905d7764cc";

assert_not_impl_any!(VerifiedDetachedSignatureInspectionV1: Clone, Serialize, serde::de::DeserializeOwned);
assert_not_impl_any!(RawDetachedEd25519SignatureV1<'static>: Clone, Serialize, serde::de::DeserializeOwned);
const_assert!(!PRODUCTION_SIGNATURE_POLICY_AVAILABLE);

#[test]
fn production_signature_policy_is_absent_before_material_is_read() {
    let raw = RawDetachedEd25519SignatureV1::new(b"not-a-signature");
    let error = inspect_pre_run_profile_signature(b"not-json", b"", &raw)
        .expect_err("production policy must remain absent");
    assert!(
        error
            .to_string()
            .contains("production detached-signature policy is absent")
    );
}

#[test]
fn rfc8032_ed25519_primitive_vector_is_strictly_verified() {
    let public_key =
        hex_array::<32>("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");
    let signature = hex_array::<64>(
        "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155\
         5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b",
    );
    let verifying_key = VerifyingKey::from_bytes(&public_key).expect("RFC8032 public key");
    assert!(!verifying_key.is_weak());
    let signature = Signature::from_slice(&signature).expect("RFC8032 signature");
    verifying_key
        .verify_strict(b"", &signature)
        .expect("RFC8032 vector must verify strictly");
}

#[test]
fn all_seven_roles_verify_exact_payloads_without_live_authority() {
    for role in ALL_DETACHED_SIGNATURE_ROLES {
        let fixture = signed_fixture(role);
        let inspected = inspect_fixture(&fixture, role).expect("role signature inspection");
        assert_eq!(inspected.role(), role);
        assert_eq!(inspected.payload_byte_count(), fixture.payload.len() as u64);
        assert_eq!(inspected.payload_schema(), role.payload_schema());
        assert_eq!(inspected.payload_sha256(), sha256(&fixture.payload));
        assert_eq!(inspected.profile_id(), TEST_PROFILE_ID);
        assert_eq!(inspected.signer_key_id(), fixture.manifest.signer_key_id);
        assert_eq!(inspected.trust_root_id(), TEST_TRUST_ROOT_ID);
        assert_eq!(inspected.trust_root_revision(), 1);
        assert!(!inspected.authorizes_live());
    }
}

#[test]
fn detached_signature_role_wire_domains_and_payload_schemas_are_exact() {
    let expected = [
        (
            DetachedSignatureRoleV1::FinalArtifactFreeze,
            "\"final_artifact_freeze\"",
            "hepta.mnl.role.final-artifact-freeze.v1",
            "hepta-mnl-v1/final-artifact-freeze",
        ),
        (
            DetachedSignatureRoleV1::PreRunProfile,
            "\"pre_run_profile\"",
            "hepta.mnl.role.pre-run-profile.v1",
            "hepta-mnl-v1/pre-run-profile",
        ),
        (
            DetachedSignatureRoleV1::FreezeManifest,
            "\"freeze_manifest\"",
            "hepta.mnl.role.freeze-manifest.v1",
            "hepta-mnl-v1/freeze-manifest",
        ),
        (
            DetachedSignatureRoleV1::SupervisorSeal,
            "\"supervisor_seal\"",
            "hepta.mnl.role.supervisor-seal.v1",
            "hepta-mnl-v1/supervisor-seal",
        ),
        (
            DetachedSignatureRoleV1::IndependentCopyAck,
            "\"independent_copy_ack\"",
            "hepta.mnl.role.independent-copy-ack.v1",
            "hepta-mnl-v1/independent-copy-ack",
        ),
        (
            DetachedSignatureRoleV1::TerminalManifest,
            "\"terminal_manifest\"",
            "hepta.mnl.role.terminal-manifest.v1",
            "hepta-mnl-v1/terminal-manifest",
        ),
        (
            DetachedSignatureRoleV1::PostRunResultEnvelope,
            "\"post_run_result_envelope\"",
            "hepta.mnl.role.post-run-result-envelope.v1",
            "hepta-mnl-v1/post-run-result-envelope",
        ),
    ];
    assert_eq!(ALL_DETACHED_SIGNATURE_ROLES.len(), expected.len());
    for (actual_role, (role, wire, domain, payload_schema)) in
        ALL_DETACHED_SIGNATURE_ROLES.into_iter().zip(expected)
    {
        assert_eq!(actual_role, role);
        assert_eq!(serde_json::to_string(&role).expect("role JSON"), wire);
        assert_eq!(role.role_domain(), domain);
        assert_eq!(role.payload_schema(), payload_schema);
    }
}

#[test]
fn fixed_signature_policy_manifest_frame_and_signature_goldens_are_stable() {
    let fixture = signed_fixture(DetachedSignatureRoleV1::FinalArtifactFreeze);
    let inspected = inspect_fixture(&fixture, fixture.manifest.role).expect("golden signature");
    assert_eq!(
        signature_policy_sha256(&fixture.policy).expect("policy digest"),
        TEST_POLICY_SHA256_GOLDEN
    );
    assert_eq!(inspected.manifest_sha256(), TEST_MANIFEST_SHA256_GOLDEN);
    assert_eq!(
        inspected.signed_frame_sha256(),
        TEST_SIGNED_FRAME_SHA256_GOLDEN
    );
    assert_eq!(inspected.signature_sha256(), TEST_SIGNATURE_SHA256_GOLDEN);
    assert_eq!(inspected.trust_policy_sha256(), TEST_POLICY_SHA256_GOLDEN);
}

#[test]
fn signature_rejects_payload_signature_and_cross_role_replay() {
    let fixture = signed_fixture(DetachedSignatureRoleV1::PreRunProfile);

    let mut payload_drift = fixture.payload.clone();
    payload_drift[0] ^= 1;
    assert!(inspect_fixture_with_payload(&fixture, fixture.manifest.role, &payload_drift).is_err());

    let mut raw_signature = fixture.signature.clone();
    raw_signature[0] ^= 1;
    let mut signature_drift = fixture.manifest.clone();
    signature_drift.signature_sha256 = sha256(&raw_signature);
    assert!(
        inspect_material(
            &fixture.policy,
            signature_drift.role,
            &signature_drift,
            &fixture.payload,
            &raw_signature,
        )
        .expect_err("self-consistent wrong signature")
        .to_string()
        .contains("strict verification failed")
    );

    for length in [63_usize, 65_usize] {
        let wrong_length = vec![0_u8; length];
        let error = inspect_material(
            &fixture.policy,
            fixture.manifest.role,
            &fixture.manifest,
            &fixture.payload,
            &wrong_length,
        )
        .expect_err("wrong signature length");
        assert!(error.to_string().contains("exactly 64 bytes"));
    }

    let target_role = DetachedSignatureRoleV1::FreezeManifest;
    let mut replay = fixture.manifest.clone();
    replay.role = target_role;
    replay.payload_schema = target_role.payload_schema().to_string();
    replay.signer_key_id = fixture.policy.entries[2].signer_key_id.clone();
    let error = inspect_material(
        &fixture.policy,
        target_role,
        &replay,
        &fixture.payload,
        &fixture.signature,
    )
    .expect_err("cross-role replay");
    assert!(error.to_string().contains("strict verification failed"));
}

#[test]
fn signature_frame_excludes_its_envelope_digest_but_manifest_binds_raw_signature() {
    let fixture = signed_fixture(DetachedSignatureRoleV1::FreezeManifest);
    let original_frame = detached_signature_frame(&fixture.manifest).expect("original frame");
    let mut different_envelope_digest = fixture.manifest.clone();
    different_envelope_digest.signature_sha256 = "2".repeat(64);
    let changed_frame =
        detached_signature_frame(&different_envelope_digest).expect("changed-envelope frame");
    assert_eq!(original_frame, changed_frame);

    let error = inspect_material(
        &fixture.policy,
        fixture.manifest.role,
        &different_envelope_digest,
        &fixture.payload,
        &fixture.signature,
    )
    .expect_err("manifest must still bind the exact detached signature bytes");
    assert!(
        error
            .to_string()
            .contains("digest differs from its exact bytes")
    );
}

#[test]
fn signature_rejects_every_signed_binding_transplant() {
    let fixture = signed_fixture(DetachedSignatureRoleV1::SupervisorSeal);

    let mut mutations = Vec::new();
    let mut value = fixture.manifest.clone();
    value.trust_root_id = "other-root-v1".to_string();
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.trust_root_revision += 1;
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.trust_policy_sha256 = "1".repeat(64);
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.signer_key_id = "other-key-v1".to_string();
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.profile_id = "other-profile-v1".to_string();
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.payload_schema = DetachedSignatureRoleV1::TerminalManifest
        .payload_schema()
        .to_string();
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.payload_byte_count += 1;
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.payload_sha256 = "2".repeat(64);
    mutations.push(value);

    for mutation in mutations {
        assert!(
            inspect_material(
                &fixture.policy,
                fixture.manifest.role,
                &mutation,
                &fixture.payload,
                &fixture.signature,
            )
            .is_err()
        );
    }
}

#[test]
fn signature_manifest_requires_exact_canonical_bounded_json() {
    let fixture = signed_fixture(DetachedSignatureRoleV1::TerminalManifest);
    let raw = RawDetachedEd25519SignatureV1::new(&fixture.signature);

    let mut padded = fixture.canonical_manifest.clone();
    padded.push(b'\n');
    assert!(
        inspect_canonical_detached_signature_with_policy(
            &fixture.policy,
            fixture.manifest.role,
            &padded,
            &fixture.payload,
            &raw,
        )
        .is_err()
    );

    let pretty = serde_json::to_vec_pretty(&fixture.manifest).expect("pretty manifest");
    assert!(
        inspect_canonical_detached_signature_with_policy(
            &fixture.policy,
            fixture.manifest.role,
            &pretty,
            &fixture.payload,
            &raw,
        )
        .is_err()
    );

    let mut value = serde_json::to_value(&fixture.manifest).expect("manifest value");
    value["authority"] = serde_json::json!(true);
    let unknown = serde_json::to_vec(&value).expect("unknown manifest");
    assert!(
        inspect_canonical_detached_signature_with_policy(
            &fixture.policy,
            fixture.manifest.role,
            &unknown,
            &fixture.payload,
            &raw,
        )
        .is_err()
    );

    let oversized = vec![b'x'; MAX_DETACHED_SIGNATURE_MANIFEST_BYTES + 1];
    assert!(
        inspect_canonical_detached_signature_with_policy(
            &fixture.policy,
            fixture.manifest.role,
            &oversized,
            &fixture.payload,
            &raw,
        )
        .is_err()
    );

    let mut missing_value = serde_json::to_value(&fixture.manifest).expect("manifest value");
    missing_value
        .as_object_mut()
        .expect("manifest object")
        .remove("profile_id");
    let missing = serde_json::to_vec(&missing_value).expect("missing-field manifest");
    assert!(
        inspect_canonical_detached_signature_with_policy(
            &fixture.policy,
            fixture.manifest.role,
            &missing,
            &fixture.payload,
            &raw,
        )
        .is_err()
    );

    let canonical = String::from_utf8(fixture.canonical_manifest.clone()).expect("manifest utf8");
    let schema_field = format!("\"schema\":\"{DETACHED_SIGNATURE_MANIFEST_SCHEMA}\"");
    let duplicate = canonical.replacen(&schema_field, &format!("{schema_field},{schema_field}"), 1);
    assert!(
        inspect_canonical_detached_signature_with_policy(
            &fixture.policy,
            fixture.manifest.role,
            duplicate.as_bytes(),
            &fixture.payload,
            &raw,
        )
        .is_err()
    );
}

#[test]
fn signature_rejects_noncanonical_fields_and_payload_bounds() {
    let fixture = signed_fixture(DetachedSignatureRoleV1::PostRunResultEnvelope);

    let mut mutations = Vec::new();
    let mut value = fixture.manifest.clone();
    value.schema = "other-schema".to_string();
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.algorithm = "other-algorithm".to_string();
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.profile_id = "Uppercase-is-not-canonical".to_string();
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.payload_sha256 = value.payload_sha256.to_ascii_uppercase();
    mutations.push(value);
    let mut value = fixture.manifest.clone();
    value.signature_sha256 = "0".repeat(64);
    mutations.push(value);

    for mutation in mutations {
        assert!(
            inspect_material(
                &fixture.policy,
                fixture.manifest.role,
                &mutation,
                &fixture.payload,
                &fixture.signature,
            )
            .is_err()
        );
    }

    assert!(
        inspect_fixture_with_payload(&fixture, fixture.manifest.role, b"").is_err(),
        "empty payloads must not acquire an inspection token"
    );
    let oversized_payload = vec![0_u8; MAX_DETACHED_SIGNATURE_PAYLOAD_BYTES + 1];
    assert!(
        inspect_fixture_with_payload(&fixture, fixture.manifest.role, &oversized_payload,).is_err(),
        "oversized payloads must fail before signature verification"
    );
}

#[test]
fn signature_policy_rejects_missing_reordered_duplicate_and_weak_keys() {
    let fixture = signed_fixture(DetachedSignatureRoleV1::IndependentCopyAck);

    let mut policies = Vec::new();
    let mut policy = fixture.policy.clone();
    policy.entries.pop();
    policies.push(policy);
    let mut policy = fixture.policy.clone();
    policy.entries.swap(0, 1);
    policies.push(policy);
    let mut policy = fixture.policy.clone();
    policy.entries[1].signer_key_id = policy.entries[0].signer_key_id.clone();
    policies.push(policy);
    let mut policy = fixture.policy.clone();
    policy.entries[1].verifying_key_bytes = policy.entries[0].verifying_key_bytes;
    policies.push(policy);
    let mut policy = fixture.policy.clone();
    policy.entries[0].verifying_key_bytes = [0_u8; 32];
    policies.push(policy);
    let mut policy = fixture.policy.clone();
    policy.trust_root_revision = 0;
    policies.push(policy);

    for policy in policies {
        assert!(
            inspect_material(
                &policy,
                fixture.manifest.role,
                &fixture.manifest,
                &fixture.payload,
                &fixture.signature,
            )
            .is_err()
        );
    }
}

struct SignatureFixture {
    canonical_manifest: Vec<u8>,
    manifest: DetachedSignatureManifestV1,
    payload: Vec<u8>,
    policy: SignaturePolicyMaterialV1,
    signature: Vec<u8>,
}

fn signed_fixture(role: DetachedSignatureRoleV1) -> SignatureFixture {
    let payload = format!("{{\"role\":\"{}\",\"value\":1}}", role.payload_schema()).into_bytes();
    signed_fixture_for_payload(role, TEST_PROFILE_ID, payload)
}

fn signed_fixture_for_payload(
    role: DetachedSignatureRoleV1,
    profile_id: &str,
    payload: Vec<u8>,
) -> SignatureFixture {
    let (policy, signing_keys) = test_policy();
    let role_index = ALL_DETACHED_SIGNATURE_ROLES
        .iter()
        .position(|candidate| *candidate == role)
        .expect("role is in exact roster");
    let mut manifest = DetachedSignatureManifestV1 {
        schema: DETACHED_SIGNATURE_MANIFEST_SCHEMA.to_string(),
        algorithm: DETACHED_SIGNATURE_ALGORITHM.to_string(),
        role,
        trust_root_id: policy.trust_root_id.clone(),
        trust_root_revision: policy.trust_root_revision,
        trust_policy_sha256: signature_policy_sha256(&policy).expect("test policy digest"),
        signer_key_id: policy.entries[role_index].signer_key_id.clone(),
        profile_id: profile_id.to_string(),
        payload_schema: role.payload_schema().to_string(),
        payload_byte_count: payload.len() as u64,
        payload_sha256: sha256(&payload),
        signature_sha256: "1".repeat(64),
    };
    let frame = detached_signature_frame(&manifest).expect("test signature frame");
    let signature = signing_keys[role_index].sign(&frame).to_bytes().to_vec();
    manifest.signature_sha256 = sha256(&signature);
    let canonical_manifest = serde_json::to_vec(&manifest).expect("canonical signature manifest");
    SignatureFixture {
        canonical_manifest,
        manifest,
        payload,
        policy,
        signature,
    }
}

pub(crate) fn inspect_test_signature_payload(
    role: DetachedSignatureRoleV1,
    profile_id: &str,
    payload: Vec<u8>,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    let fixture = signed_fixture_for_payload(role, profile_id, payload);
    inspect_fixture(&fixture, role)
}

fn test_policy() -> (SignaturePolicyMaterialV1, Vec<SigningKey>) {
    let mut entries = Vec::new();
    let mut signing_keys = Vec::new();
    for (index, role) in ALL_DETACHED_SIGNATURE_ROLES.iter().copied().enumerate() {
        let signing_key = SigningKey::from_bytes(&[(index + 1) as u8; 32]);
        entries.push(SignaturePolicyEntryMaterialV1 {
            role,
            signer_key_id: format!("hepta-mnl-test-role-{}-key-v1", index + 1),
            verifying_key_bytes: signing_key.verifying_key().to_bytes(),
        });
        signing_keys.push(signing_key);
    }
    (
        SignaturePolicyMaterialV1 {
            entries,
            trust_root_id: TEST_TRUST_ROOT_ID.to_string(),
            trust_root_revision: 1,
        },
        signing_keys,
    )
}

fn inspect_fixture(
    fixture: &SignatureFixture,
    expected_role: DetachedSignatureRoleV1,
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    let raw = RawDetachedEd25519SignatureV1::new(&fixture.signature);
    inspect_canonical_detached_signature_with_policy(
        &fixture.policy,
        expected_role,
        &fixture.canonical_manifest,
        &fixture.payload,
        &raw,
    )
}

fn inspect_fixture_with_payload(
    fixture: &SignatureFixture,
    expected_role: DetachedSignatureRoleV1,
    payload: &[u8],
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    let raw = RawDetachedEd25519SignatureV1::new(&fixture.signature);
    inspect_canonical_detached_signature_with_policy(
        &fixture.policy,
        expected_role,
        &fixture.canonical_manifest,
        payload,
        &raw,
    )
}

fn inspect_material(
    policy: &SignaturePolicyMaterialV1,
    expected_role: DetachedSignatureRoleV1,
    manifest: &DetachedSignatureManifestV1,
    payload: &[u8],
    signature: &[u8],
) -> Result<VerifiedDetachedSignatureInspectionV1, MnlTrustError> {
    let canonical_manifest = serde_json::to_vec(manifest).expect("canonical mutated manifest");
    let raw = RawDetachedEd25519SignatureV1::new(signature);
    inspect_canonical_detached_signature_with_policy(
        policy,
        expected_role,
        &canonical_manifest,
        payload,
        &raw,
    )
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn hex_array<const N: usize>(value: &str) -> [u8; N] {
    let compact = value
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    assert_eq!(compact.len(), N * 2);
    let mut output = [0_u8; N];
    for (index, pair) in compact.as_bytes().chunks_exact(2).enumerate() {
        output[index] = (test_hex_nibble(pair[0]) << 4) | test_hex_nibble(pair[1]);
    }
    output
}

fn test_hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("invalid test hex"),
    }
}
