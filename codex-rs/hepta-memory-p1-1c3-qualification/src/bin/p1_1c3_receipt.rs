use ed25519_dalek::SigningKey;
use hepta_memory_p1_1c3_qualification::{
    CorpusIntakeRequest, Digest32, IntakePolicy, TrustDomain, TrustRole, TrustStore,
    TrustedKey, evaluate_corpus_intake, intake_policy_digest, trust_store_digest,
    trusted_key_digest,
};

fn fixture_trust_store() -> TrustStore {
    let signing = SigningKey::from_bytes(&[7_u8; 32]);
    let mut key = TrustedKey {
        key_id: "qualification-ci-key".to_string(),
        public_key: signing.verifying_key().to_bytes(),
        role: TrustRole::CiQualification,
        affiliation_id: "qualification-fixture".to_string(),
        allowed_locales: Vec::new(),
        valid_from_unix_seconds: 1,
        valid_to_unix_seconds: u64::MAX,
        domain: TrustDomain::QualificationFixture,
        revoked: false,
        key_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    key.key_sha256 = trusted_key_digest(&key);
    let mut store = TrustStore {
        store_id: "p1c3-fixture-store".to_string(),
        version: 1,
        keys: vec![key],
        store_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    store.store_sha256 = trust_store_digest(&store);
    store
}

fn main() {
    let trust_store = fixture_trust_store();
    let mut policy = IntakePolicy {
        policy_id: "p1c3-fixture-intake-policy".to_string(),
        expected_p1c_commit: "fe33565ce74c013e574c307e4fab101820c0ea88".to_string(),
        expected_p1c_tree: "832755e51af35d6cb6c8dee1dd13241c79e509e1".to_string(),
        expected_p1c1_commit: "f961a056ac0a35c1967a934de7cf5bf7ffb92a05".to_string(),
        expected_p1c1_tree: "e0e42f9c0e1af161058a57c13b6cf77710fd200c".to_string(),
        expected_trust_store_sha256: trust_store.store_sha256,
        allowed_spdx_license_ids: vec!["Apache-2.0".to_string()],
        minimum_items: 8,
        minimum_locales: 8,
        require_external_signers: true,
        require_non_fixture: true,
        require_derivative_rights: true,
        require_operator_approval: true,
        policy_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    policy.policy_sha256 = intake_policy_digest(&policy);
    let request = CorpusIntakeRequest {
        expected_dataset_sha256: Digest32::for_bytes(b"fixture-dataset"),
        item_count: 8,
        locale_count: 8,
        p1c_qualification: None,
        p1c1_qualification: None,
        acceptance: None,
        review_trust: None,
        license: None,
        provenance: None,
        privacy: None,
        operator_approval: None,
    };
    let receipt = evaluate_corpus_intake(&request, &policy, &trust_store, 100)
        .expect("blocked fixture intake receipt");
    assert!(!receipt.mechanically_accepted);
    assert!(!receipt.external_evidence_complete);
    assert!(!receipt.production_authority);
    print!("{}", receipt.to_json_pretty());
}
