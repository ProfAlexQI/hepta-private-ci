use ed25519_dalek::{Signer, SigningKey};
use hepta_memory_p1_1c1_qualification::{
    AcceptancePolicy, AcceptanceReceipt, DependencyState, ReviewBatch, evaluate_review_batch,
};
use hepta_memory_p1_1c3_qualification::*;
use std::collections::BTreeMap;

const P1C_COMMIT: &str = "fe33565ce74c013e574c307e4fab101820c0ea88";
const P1C_TREE: &str = "832755e51af35d6cb6c8dee1dd13241c79e509e1";
const P1C1_COMMIT: &str = "f961a056ac0a35c1967a934de7cf5bf7ffb92a05";
const P1C1_TREE: &str = "e0e42f9c0e1af161058a57c13b6cf77710fd200c";
const NOW: u64 = 2_000;
const LOCALES: [&str; 8] = [
    "ar-SA", "en-US", "es-ES", "fr-FR", "hi-IN", "ja-JP", "pt-BR", "zh-CN",
];

fn sign_digest(key_id: &str, payload: Digest32, signing: &SigningKey) -> SignedDigest {
    let signature = signing.sign(payload.as_bytes()).to_bytes();
    let mut signed = SignedDigest {
        key_id: key_id.to_string(),
        algorithm: ED25519_ALGORITHM.to_string(),
        payload_sha256: payload,
        signature,
        envelope_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    signed.envelope_sha256 = signed_digest_envelope_digest(&signed);
    signed
}

fn trusted_key(
    id: &str,
    seed: u8,
    role: TrustRole,
    affiliation: &str,
    locales: &[&str],
    domain: TrustDomain,
) -> (TrustedKey, SigningKey) {
    let signing = SigningKey::from_bytes(&[seed; 32]);
    let mut key = TrustedKey {
        key_id: id.to_string(),
        public_key: signing.verifying_key().to_bytes(),
        role,
        affiliation_id: affiliation.to_string(),
        allowed_locales: locales.iter().map(|locale| (*locale).to_string()).collect(),
        valid_from_unix_seconds: 1,
        valid_to_unix_seconds: 10_000,
        domain,
        revoked: false,
        key_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    key.key_sha256 = trusted_key_digest(&key);
    (key, signing)
}

fn trust_fixture(domain: TrustDomain) -> (TrustStore, BTreeMap<String, SigningKey>) {
    let definitions = [
        ("ci", 1, TrustRole::CiQualification, "ci-org", Vec::<&str>::new()),
        ("license", 2, TrustRole::LicenseApprover, "legal-org", Vec::new()),
        ("operator", 3, TrustRole::Operator, "operator-org", Vec::new()),
        ("privacy", 4, TrustRole::PrivacyApprover, "privacy-org", Vec::new()),
        ("provenance", 5, TrustRole::ProvenanceApprover, "data-org", Vec::new()),
        ("reviewer-a", 6, TrustRole::Reviewer, "review-org-a", LOCALES.to_vec()),
        ("reviewer-b", 7, TrustRole::Reviewer, "review-org-b", LOCALES.to_vec()),
    ];
    let mut keys = Vec::new();
    let mut signers = BTreeMap::new();
    for (id, seed, role, affiliation, locales) in definitions {
        let (key, signer) = trusted_key(id, seed, role, affiliation, &locales, domain);
        keys.push(key);
        signers.insert(id.to_string(), signer);
    }
    keys.sort_by(|left, right| left.key_id.cmp(&right.key_id));
    let mut store = TrustStore {
        store_id: "p1c3-test-store".to_string(),
        version: 1,
        keys,
        store_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    store.store_sha256 = trust_store_digest(&store);
    (store, signers)
}

fn review_batch(store: &TrustStore) -> ReviewBatch {
    let reviewer_a = store.find_key("reviewer-a").expect("reviewer a");
    let reviewer_b = store.find_key("reviewer-b").expect("reviewer b");
    let commitments = [
        reviewer_a.commitment_sha256().to_string(),
        reviewer_b.commitment_sha256().to_string(),
    ];
    let mut review_tsv = format!(
        concat!(
            "# schema=hepta.intelligence.p1_1c1.review_batch.v1\n",
            "# provenance=human_reviewed_v1\n",
            "# reviewed=true\n",
            "# human_review_attested=true\n",
            "# source_p1_1c_commit={}\n",
            "# locales={}\n",
            "item_id\tlocale\tquery_sha256\tcandidate_sha256\treviewer_commitment\t",
            "relevance\tcitation\tcontradiction\tprivacy\trationale_sha256\n"
        ),
        P1C_COMMIT,
        LOCALES.join(",")
    );
    for (index, locale) in LOCALES.iter().enumerate() {
        let item_id = format!("case-{index:02}:candidate-{index:02}");
        let query = Digest32::for_bytes(format!("query-{index}").as_bytes());
        let candidate = Digest32::for_bytes(format!("candidate-{index}").as_bytes());
        let relevance = u8::try_from(index % 4).expect("relevance");
        for (reviewer_index, commitment) in commitments.iter().enumerate() {
            let rationale = Digest32::for_bytes(
                format!("rationale-{index}-{reviewer_index}").as_bytes(),
            );
            review_tsv.push_str(&format!(
                "{item_id}\t{locale}\t{query}\t{candidate}\t{commitment}\t{relevance}\tsupported\tnone\tallow\t{rationale}\n"
            ));
        }
    }
    let adjudication_tsv = concat!(
        "# schema=hepta.intelligence.p1_1c1.adjudication_batch.v1\n",
        "item_id\tadjudicator_commitment\trelevance\tcitation\tcontradiction\tprivacy\t",
        "redaction_receipt_sha256\trationale_sha256\n"
    );
    ReviewBatch::parse_tsv(&review_tsv, adjudication_tsv).expect("human review batch")
}

fn acceptance(batch: &ReviewBatch) -> AcceptanceReceipt {
    evaluate_review_batch(
        batch,
        &DependencyState::qualified(P1C_COMMIT).expect("qualified dependency"),
        &AcceptancePolicy::default(),
    )
    .expect("accepted review batch")
}

fn review_trust(
    batch: &ReviewBatch,
    store: &TrustStore,
    signers: &BTreeMap<String, SigningKey>,
    require_external: bool,
) -> (ReviewTrustBundle, ReviewTrustPolicy, ReviewTrustReceipt) {
    let mut review_attestations = Vec::new();
    for review in &batch.reviews {
        let key = store
            .keys
            .iter()
            .find(|key| key.commitment_sha256().to_string() == review.reviewer_commitment.to_string())
            .expect("review trust key");
        let payload = review_record_digest(review).expect("review digest");
        review_attestations.push(ReviewAttestation {
            item_id: review.item_id.clone(),
            reviewer_key_id: key.key_id.clone(),
            review_record_sha256: payload,
            signed: sign_digest(&key.key_id, payload, &signers[&key.key_id]),
        });
    }
    let bundle = ReviewTrustBundle {
        review_attestations,
        adjudication_attestations: Vec::new(),
    };
    let mut policy = ReviewTrustPolicy {
        policy_id: "p1c3-review-trust".to_string(),
        expected_trust_store_sha256: store.store_sha256,
        require_external_signers: require_external,
        require_distinct_reviewer_affiliations: true,
        require_independent_adjudicator_affiliation: true,
        policy_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    policy.policy_sha256 = review_trust_policy_digest(&policy);
    let receipt = verify_review_trust(batch, &bundle, &policy, store, NOW)
        .expect("verified review trust");
    (bundle, policy, receipt)
}

fn qualification_evidence(
    source_commit: &str,
    source_tree: &str,
    workflow: &str,
    store: &TrustStore,
    signers: &BTreeMap<String, SigningKey>,
) -> (QualificationEvidence, QualificationPolicy, VerifiedQualificationReceipt) {
    let mut evidence = QualificationEvidence {
        schema: "hepta.intelligence.p1_1c3.executable_qualification.v1".to_string(),
        repository: "ProfAlexQI/hepta-private-ci".to_string(),
        source_commit: source_commit.to_string(),
        source_tree: source_tree.to_string(),
        workflow_name: workflow.to_string(),
        workflow_run_id: 101,
        workflow_run_attempt: 1,
        job_id: 202,
        runner_id: 303,
        runner_name: "github-actions-303".to_string(),
        runner_os: "Linux".to_string(),
        runner_arch: "ARM64".to_string(),
        step_count: 20,
        commands_executed: true,
        conclusion: "success".to_string(),
        toolchain: "1.95.0".to_string(),
        tests_passed: 25,
        artifact_id: 404,
        artifact_sha256: Digest32::for_bytes(workflow.as_bytes()),
        gates: QualificationGateSet {
            source_binding: true,
            source_gate: true,
            rustfmt: true,
            tests: true,
            check: true,
            clippy: true,
            receipt_reproducibility: true,
            receipt_redaction: true,
            clean_tree: true,
        },
        signed: sign_digest("ci", Digest32::for_bytes(b"placeholder"), &signers["ci"]),
    };
    evidence.signed = sign_digest("ci", evidence.payload_sha256(), &signers["ci"]);
    let mut policy = QualificationPolicy {
        policy_id: format!("qualification-{workflow}"),
        expected_repository: evidence.repository.clone(),
        expected_source_commit: source_commit.to_string(),
        expected_source_tree: source_tree.to_string(),
        expected_workflow_name: workflow.to_string(),
        expected_toolchain: "1.95.0".to_string(),
        minimum_tests_passed: 10,
        minimum_step_count: 10,
        expected_trust_store_sha256: store.store_sha256,
        require_external_signer: true,
        policy_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    policy.policy_sha256 = qualification_policy_digest(&policy);
    let receipt = verify_qualification(&evidence, &policy, store, NOW)
        .expect("verified qualification");
    (evidence, policy, receipt)
}

fn signed_license(
    dataset: Digest32,
    signers: &BTreeMap<String, SigningKey>,
) -> LicenseEvidence {
    let mut evidence = LicenseEvidence {
        dataset_sha256: dataset,
        spdx_license_id: "Apache-2.0".to_string(),
        license_text_sha256: Digest32::for_bytes(b"license-text"),
        permits_offline_evaluation: true,
        permits_storage: true,
        permits_derivatives: true,
        fixture_only: false,
        signed: sign_digest("license", Digest32::for_bytes(b"placeholder"), &signers["license"]),
    };
    evidence.signed = sign_digest("license", evidence.payload_sha256(), &signers["license"]);
    evidence
}

fn signed_provenance(
    dataset: Digest32,
    signers: &BTreeMap<String, SigningKey>,
) -> ProvenanceEvidence {
    let mut evidence = ProvenanceEvidence {
        dataset_sha256: dataset,
        source_manifest_sha256: Digest32::for_bytes(b"source-manifest"),
        acquisition_method: "consented_review_collection".to_string(),
        legal_basis: "documented_consent".to_string(),
        collected_from_unix_seconds: 100,
        collected_to_unix_seconds: 200,
        human_review_source: true,
        fixture_only: false,
        signed: sign_digest("provenance", Digest32::for_bytes(b"placeholder"), &signers["provenance"]),
    };
    evidence.signed = sign_digest("provenance", evidence.payload_sha256(), &signers["provenance"]);
    evidence
}

fn signed_privacy(
    dataset: Digest32,
    signers: &BTreeMap<String, SigningKey>,
) -> PrivacyEvidence {
    let mut evidence = PrivacyEvidence {
        dataset_sha256: dataset,
        scanner_id: "hepta-dlp-v1".to_string(),
        secret_scan_sha256: Digest32::for_bytes(b"secret-scan"),
        secret_scan_passed: true,
        pii_assessment_sha256: Digest32::for_bytes(b"pii-assessment"),
        pii_assessment_passed: true,
        redaction_manifest_sha256: Some(Digest32::for_bytes(b"redaction-manifest")),
        redaction_complete: true,
        residual_risk_accepted: true,
        fixture_only: false,
        signed: sign_digest("privacy", Digest32::for_bytes(b"placeholder"), &signers["privacy"]),
    };
    evidence.signed = sign_digest("privacy", evidence.payload_sha256(), &signers["privacy"]);
    evidence
}

struct FullFixture {
    store: TrustStore,
    signers: BTreeMap<String, SigningKey>,
    batch: ReviewBatch,
    acceptance: AcceptanceReceipt,
    review_bundle: ReviewTrustBundle,
    review_policy: ReviewTrustPolicy,
    review_trust: ReviewTrustReceipt,
    p1c_evidence: QualificationEvidence,
    p1c_policy: QualificationPolicy,
    p1c_qualification: VerifiedQualificationReceipt,
    p1c1_evidence: QualificationEvidence,
    p1c1_policy: QualificationPolicy,
    p1c1_qualification: VerifiedQualificationReceipt,
    license: LicenseEvidence,
    provenance: ProvenanceEvidence,
    privacy: PrivacyEvidence,
    operator: OperatorApprovalEvidence,
    intake_policy: IntakePolicy,
    dataset: Digest32,
}

impl FullFixture {
    fn build() -> Self {
        let (store, signers) = trust_fixture(TrustDomain::ExternalAttested);
        let batch = review_batch(&store);
        let acceptance = acceptance(&batch);
        let (review_bundle, review_policy, review_trust) =
            review_trust(&batch, &store, &signers, true);
        let (p1c_evidence, p1c_policy, p1c_qualification) = qualification_evidence(
            P1C_COMMIT,
            P1C_TREE,
            "hepta-intelligence-p1-1c-offline-efficacy",
            &store,
            &signers,
        );
        let (p1c1_evidence, p1c1_policy, p1c1_qualification) = qualification_evidence(
            P1C1_COMMIT,
            P1C1_TREE,
            "hepta-intelligence-p1-1c1-reviewed-corpus",
            &store,
            &signers,
        );
        let dataset = Digest32::from_hex(&acceptance.reviewed_corpus_sha256.to_string())
            .expect("dataset digest");
        let license = signed_license(dataset, &signers);
        let provenance = signed_provenance(dataset, &signers);
        let privacy = signed_privacy(dataset, &signers);
        let mut intake_policy = IntakePolicy {
            policy_id: "p1c3-intake-policy".to_string(),
            expected_p1c_commit: P1C_COMMIT.to_string(),
            expected_p1c_tree: P1C_TREE.to_string(),
            expected_p1c1_commit: P1C1_COMMIT.to_string(),
            expected_p1c1_tree: P1C1_TREE.to_string(),
            expected_trust_store_sha256: store.store_sha256,
            allowed_spdx_license_ids: vec!["Apache-2.0".to_string()],
            minimum_items: 8,
            minimum_locales: 8,
            require_external_signers: true,
            require_non_fixture: true,
            require_derivative_rights: true,
            require_operator_approval: true,
            policy_sha256: Digest32::for_bytes(b"uncomputed"),
        };
        intake_policy.policy_sha256 = intake_policy_digest(&intake_policy);
        let subject = intake_subject_digest(
            dataset,
            intake_policy.policy_sha256,
            Some(p1c_qualification.receipt_sha256),
            Some(p1c1_qualification.receipt_sha256),
            Some(Digest32::from_hex(&acceptance.receipt_sha256.to_string()).expect("acceptance digest")),
            Some(review_trust.receipt_sha256),
            Some(license.payload_sha256()),
            Some(provenance.payload_sha256()),
            Some(privacy.payload_sha256()),
        );
        let mut operator = OperatorApprovalEvidence {
            subject_sha256: subject,
            scope: "offline_corpus_intake".to_string(),
            approved: true,
            approved_at_unix_seconds: 1_000,
            expires_at_unix_seconds: 3_000,
            fixture_only: false,
            signed: sign_digest("operator", Digest32::for_bytes(b"placeholder"), &signers["operator"]),
        };
        operator.signed = sign_digest("operator", operator.payload_sha256(), &signers["operator"]);
        Self {
            store,
            signers,
            batch,
            acceptance,
            review_bundle,
            review_policy,
            review_trust,
            p1c_evidence,
            p1c_policy,
            p1c_qualification,
            p1c1_evidence,
            p1c1_policy,
            p1c1_qualification,
            license,
            provenance,
            privacy,
            operator,
            intake_policy,
            dataset,
        }
    }

    fn request(&self) -> CorpusIntakeRequest<'_> {
        CorpusIntakeRequest {
            expected_dataset_sha256: self.dataset,
            item_count: self.acceptance.item_count,
            locale_count: self.acceptance.locale_count,
            p1c_qualification: Some(&self.p1c_qualification),
            p1c1_qualification: Some(&self.p1c1_qualification),
            acceptance: Some(&self.acceptance),
            review_trust: Some(&self.review_trust),
            license: Some(&self.license),
            provenance: Some(&self.provenance),
            privacy: Some(&self.privacy),
            operator_approval: Some(&self.operator),
        }
    }
}

#[test]
fn fully_signed_external_mechanics_can_pass_without_product_authority() {
    let fixture = FullFixture::build();
    let receipt = evaluate_corpus_intake(
        &fixture.request(),
        &fixture.intake_policy,
        &fixture.store,
        NOW,
    )
    .expect("trusted mechanical intake");
    assert!(receipt.mechanically_accepted);
    assert!(receipt.external_evidence_complete);
    assert!(receipt.blocked_reasons.is_empty());
    assert!(!receipt.runtime_wired);
    assert!(!receipt.production_authority);
    assert!(!receipt.efficacy_claim);
    assert!(!receipt.promotion);
}

#[test]
fn missing_external_evidence_remains_deterministically_blocked() {
    let fixture = FullFixture::build();
    let request = CorpusIntakeRequest {
        expected_dataset_sha256: fixture.dataset,
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
    let first = evaluate_corpus_intake(&request, &fixture.intake_policy, &fixture.store, NOW)
        .expect("blocked intake");
    let second = evaluate_corpus_intake(&request, &fixture.intake_policy, &fixture.store, NOW)
        .expect("repeat blocked intake");
    assert_eq!(first, second);
    assert!(!first.mechanically_accepted);
    assert!(first.blocked_reasons.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(!first.to_json_pretty().contains("reviewer-a"));
}

#[test]
fn qualification_rejects_zero_runner_or_steps() {
    let fixture = FullFixture::build();
    let mut evidence = fixture.p1c_evidence.clone();
    evidence.runner_id = 0;
    evidence.step_count = 0;
    evidence.signed = sign_digest("ci", evidence.payload_sha256(), &fixture.signers["ci"]);
    assert!(verify_qualification(&evidence, &fixture.p1c_policy, &fixture.store, NOW).is_err());
}

#[test]
fn qualification_rejects_exact_source_drift() {
    let fixture = FullFixture::build();
    let mut evidence = fixture.p1c_evidence.clone();
    evidence.source_commit = "1111111111111111111111111111111111111111".to_string();
    evidence.signed = sign_digest("ci", evidence.payload_sha256(), &fixture.signers["ci"]);
    assert!(verify_qualification(&evidence, &fixture.p1c_policy, &fixture.store, NOW).is_err());
}

#[test]
fn tampered_ed25519_signature_is_rejected() {
    let fixture = FullFixture::build();
    let mut evidence = fixture.p1c_evidence.clone();
    evidence.signed.signature[0] ^= 0x80;
    evidence.signed.envelope_sha256 = signed_digest_envelope_digest(&evidence.signed);
    assert!(verify_qualification(&evidence, &fixture.p1c_policy, &fixture.store, NOW).is_err());
}

#[test]
fn external_policy_rejects_qualification_fixture_key() {
    let (store, signers) = trust_fixture(TrustDomain::QualificationFixture);
    let (evidence, mut policy, _) = qualification_evidence(
        P1C_COMMIT,
        P1C_TREE,
        "fixture-workflow",
        &store,
        &signers,
    );
    policy.require_external_signer = true;
    policy.policy_sha256 = qualification_policy_digest(&policy);
    assert!(verify_qualification(&evidence, &policy, &store, NOW).is_err());
}

#[test]
fn same_affiliation_reviewers_are_rejected() {
    let fixture = FullFixture::build();
    let mut store = fixture.store.clone();
    let reviewer_b = store
        .keys
        .iter_mut()
        .find(|key| key.key_id == "reviewer-b")
        .expect("reviewer b");
    reviewer_b.affiliation_id = "review-org-a".to_string();
    reviewer_b.key_sha256 = trusted_key_digest(reviewer_b);
    store.store_sha256 = trust_store_digest(&store);
    let mut policy = fixture.review_policy.clone();
    policy.expected_trust_store_sha256 = store.store_sha256;
    policy.policy_sha256 = review_trust_policy_digest(&policy);
    assert!(verify_review_trust(
        &fixture.batch,
        &fixture.review_bundle,
        &policy,
        &store,
        NOW,
    )
    .is_err());
}

#[test]
fn missing_review_signature_is_rejected() {
    let fixture = FullFixture::build();
    let mut bundle = fixture.review_bundle.clone();
    bundle.review_attestations.pop();
    assert!(verify_review_trust(
        &fixture.batch,
        &bundle,
        &fixture.review_policy,
        &fixture.store,
        NOW,
    )
    .is_err());
}

#[test]
fn locale_unauthorized_reviewer_is_rejected() {
    let fixture = FullFixture::build();
    let mut store = fixture.store.clone();
    let reviewer_a = store
        .keys
        .iter_mut()
        .find(|key| key.key_id == "reviewer-a")
        .expect("reviewer a");
    reviewer_a.allowed_locales = vec!["en-US".to_string()];
    reviewer_a.key_sha256 = trusted_key_digest(reviewer_a);
    store.store_sha256 = trust_store_digest(&store);
    let mut policy = fixture.review_policy.clone();
    policy.expected_trust_store_sha256 = store.store_sha256;
    policy.policy_sha256 = review_trust_policy_digest(&policy);
    assert!(verify_review_trust(
        &fixture.batch,
        &fixture.review_bundle,
        &policy,
        &store,
        NOW,
    )
    .is_err());
}

#[test]
fn disallowed_license_blocks_intake() {
    let fixture = FullFixture::build();
    let mut policy = fixture.intake_policy.clone();
    policy.allowed_spdx_license_ids = vec!["CC-BY-4.0".to_string()];
    policy.policy_sha256 = intake_policy_digest(&policy);
    let receipt = evaluate_corpus_intake(&fixture.request(), &policy, &fixture.store, NOW)
        .expect("license-blocked intake");
    assert!(receipt.blocked_reasons.contains(&"license.not_allowed".to_string()));
}

#[test]
fn failed_privacy_assessment_blocks_intake() {
    let mut fixture = FullFixture::build();
    fixture.privacy.pii_assessment_passed = false;
    fixture.privacy.signed = sign_digest(
        "privacy",
        fixture.privacy.payload_sha256(),
        &fixture.signers["privacy"],
    );
    let receipt = evaluate_corpus_intake(
        &fixture.request(),
        &fixture.intake_policy,
        &fixture.store,
        NOW,
    )
    .expect("privacy-blocked intake");
    assert!(receipt
        .blocked_reasons
        .contains(&"privacy.scan_or_assessment_failed".to_string()));
}

#[test]
fn dataset_digest_drift_blocks_intake() {
    let mut fixture = FullFixture::build();
    fixture.license.dataset_sha256 = Digest32::for_bytes(b"other-dataset");
    fixture.license.signed = sign_digest(
        "license",
        fixture.license.payload_sha256(),
        &fixture.signers["license"],
    );
    let receipt = evaluate_corpus_intake(
        &fixture.request(),
        &fixture.intake_policy,
        &fixture.store,
        NOW,
    )
    .expect("dataset-drift intake");
    assert!(receipt
        .blocked_reasons
        .contains(&"license.dataset_digest_mismatch".to_string()));
}

#[test]
fn operator_subject_drift_blocks_intake() {
    let mut fixture = FullFixture::build();
    fixture.operator.subject_sha256 = Digest32::for_bytes(b"different-subject");
    fixture.operator.signed = sign_digest(
        "operator",
        fixture.operator.payload_sha256(),
        &fixture.signers["operator"],
    );
    let receipt = evaluate_corpus_intake(
        &fixture.request(),
        &fixture.intake_policy,
        &fixture.store,
        NOW,
    )
    .expect("operator-drift intake");
    assert!(receipt
        .blocked_reasons
        .contains(&"operator.approval_binding_invalid".to_string()));
}

#[test]
fn receipt_rejects_duplicate_or_noncanonical_blockers() {
    let fixture = FullFixture::build();
    let request = CorpusIntakeRequest {
        expected_dataset_sha256: fixture.dataset,
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
    let mut receipt = evaluate_corpus_intake(&request, &fixture.intake_policy, &fixture.store, NOW)
        .expect("blocked receipt");
    receipt.blocked_reasons.push(receipt.blocked_reasons[0].clone());
    receipt.blocked_reasons.sort();
    assert!(receipt.validate().is_err());
}

#[test]
fn verified_receipts_remain_bound_to_expected_trust_store() {
    let fixture = FullFixture::build();
    assert!(fixture.p1c_qualification.is_verified());
    assert!(fixture.p1c1_qualification.is_verified());
    assert!(fixture.review_trust.is_verified());
    assert_eq!(fixture.p1c_qualification.trust_store_sha256, fixture.store.store_sha256);
    assert_eq!(fixture.p1c1_qualification.trust_store_sha256, fixture.store.store_sha256);
    assert_eq!(fixture.review_trust.trust_store_sha256, fixture.store.store_sha256);
    assert!(fixture.p1c1_evidence.gates.all_pass());
    assert_eq!(fixture.p1c1_policy.expected_source_commit, P1C1_COMMIT);
}
