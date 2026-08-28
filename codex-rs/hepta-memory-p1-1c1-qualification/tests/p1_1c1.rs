use hepta_memory_p1_1c1_qualification::{
    AcceptancePolicy, CorpusProvenance, DependencyState, PrivacyDecision,
    ReviewBatch, evaluate_review_batch,
};

const REVIEW_SEED: &str =
    include_str!("../fixtures/p1_1c1_review_seed.tsv");
const ADJUDICATION_SEED: &str =
    include_str!("../fixtures/p1_1c1_adjudication_seed.tsv");
const SOURCE_COMMIT: &str = "fe33565ce74c013e574c307e4fab101820c0ea88";
const REVIEWER_A: &str =
    "224dcda222ac22c529179c68139991aefb466584a5d1104362f7df1f0c4ee7b4";
const REVIEWER_B: &str =
    "ae6eca04872013b2a472d63bff68ad000948fdc3d24b42c18f616339077be61b";
const FIRST_QUERY: &str =
    "3dfa9f37d63ca73e7d22c0cd30290b5cb8117fa87a9fa3c4e5d988b0c030f9b0";

fn seed_batch() -> ReviewBatch {
    ReviewBatch::parse_tsv(REVIEW_SEED, ADJUDICATION_SEED)
        .expect("review seed")
}

fn seed_receipt(
) -> hepta_memory_p1_1c1_qualification::AcceptanceReceipt {
    let dependency =
        DependencyState::blocked_seed(SOURCE_COMMIT).expect("dependency");
    evaluate_review_batch(
        &seed_batch(),
        &dependency,
        &AcceptancePolicy::default(),
    )
    .expect("seed review receipt")
}

fn remove_adjudication(item_id: &str) -> String {
    ADJUDICATION_SEED
        .lines()
        .filter(|line| !line.starts_with(item_id))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn replace_review_privacy(
    item_id: &str,
    reviewer: &str,
    privacy: &str,
) -> String {
    let mut output = Vec::new();
    for line in REVIEW_SEED.lines() {
        if line.starts_with(item_id) && line.contains(reviewer) {
            let mut columns = line.split('\t').collect::<Vec<_>>();
            columns[8] = privacy;
            output.push(columns.join("\t"));
        } else {
            output.push(line.to_string());
        }
    }
    output.join("\n") + "\n"
}

#[test]
fn seed_contract_has_two_independent_reviews_for_eight_locales() {
    let batch = seed_batch();
    assert_eq!(batch.item_count(), 8);
    assert_eq!(batch.reviews.len(), 16);
    assert_eq!(batch.reviewer_count(), 2);
    assert_eq!(batch.adjudications.len(), 2);
    assert_eq!(batch.header.locales.len(), 8);
    assert_eq!(
        batch.header.provenance,
        CorpusProvenance::SyntheticReviewSeed
    );
    assert!(!batch.header.reviewed);
    assert!(!batch.header.human_review_attested);
}

#[test]
fn seed_agreement_metrics_are_fixed_point_and_deterministic() {
    let receipt = seed_receipt();
    assert_eq!(receipt.agreement.exact_tuple_agreement_ppm, 750_000);
    assert_eq!(
        receipt.agreement.weighted_relevance_kappa_ppm,
        666_666
    );
    assert_eq!(receipt.privacy_redact_count, 1);
    assert_eq!(receipt.unresolved_count, 0);
    assert_eq!(receipt.accepted_item_count, 8);
}

#[test]
fn synthetic_seed_validates_pipeline_but_never_claims_acceptance() {
    let receipt = seed_receipt();
    assert_eq!(
        receipt.status,
        "PASS_P1_1C1_REVIEW_PIPELINE_SEED_ONLY"
    );
    assert!(receipt.review_pipeline_validated);
    assert!(!receipt.reviewed_corpus_accepted);
    assert!(!receipt.corpus_reviewed);
    assert!(receipt
        .blocked_reasons
        .contains(&"P1_1C_SOURCE_QUALIFICATION_MISSING".to_string()));
    assert!(receipt
        .blocked_reasons
        .contains(&"CORPUS_PROVENANCE_NOT_HUMAN_REVIEWED".to_string()));
    assert!(receipt
        .blocked_reasons
        .contains(&"HUMAN_REVIEW_ATTESTATION_MISSING".to_string()));
}

#[test]
fn hypothetical_human_batch_requires_qualified_bound_dependency() {
    let mut batch = seed_batch();
    batch.header.provenance = CorpusProvenance::HumanReviewedV1;
    batch.header.reviewed = true;
    batch.header.human_review_attested = true;

    let blocked = evaluate_review_batch(
        &batch,
        &DependencyState::blocked_seed(SOURCE_COMMIT).expect("blocked"),
        &AcceptancePolicy::default(),
    )
    .expect("blocked receipt");
    assert!(!blocked.reviewed_corpus_accepted);

    let accepted = evaluate_review_batch(
        &batch,
        &DependencyState::qualified(SOURCE_COMMIT).expect("qualified"),
        &AcceptancePolicy::default(),
    )
    .expect("accepted receipt");
    assert_eq!(
        accepted.status,
        "PASS_P1_1C1_REVIEWED_CORPUS_ACCEPTED"
    );
    assert!(accepted.reviewed_corpus_accepted);
    assert!(accepted.corpus_reviewed);
    assert!(accepted.blocked_reasons.is_empty());
    assert!(!accepted.efficacy_validation);
    assert!(!accepted.efficacy_claim);
}

#[test]
fn missing_adjudication_remains_fail_closed_and_unresolved() {
    let adjudications = remove_adjudication("ar-case-001");
    let batch =
        ReviewBatch::parse_tsv(REVIEW_SEED, &adjudications)
            .expect("batch without one adjudication");
    let receipt = evaluate_review_batch(
        &batch,
        &DependencyState::blocked_seed(SOURCE_COMMIT).expect("dependency"),
        &AcceptancePolicy::default(),
    )
    .expect("receipt");
    assert_eq!(receipt.unresolved_count, 1);
    assert!(receipt
        .blocked_reasons
        .contains(&"UNRESOLVED_REVIEW_DISAGREEMENTS".to_string()));
    assert!(receipt.accepted_item_count < receipt.item_count);
}

#[test]
fn privacy_block_from_one_reviewer_cannot_be_overridden() {
    let reviews = replace_review_privacy(
        "en-case-001",
        REVIEWER_B,
        "block",
    );
    let batch =
        ReviewBatch::parse_tsv(&reviews, ADJUDICATION_SEED)
            .expect("privacy-blocked batch");
    let receipt = evaluate_review_batch(
        &batch,
        &DependencyState::blocked_seed(SOURCE_COMMIT).expect("dependency"),
        &AcceptancePolicy::default(),
    )
    .expect("privacy-blocked receipt");
    assert_eq!(receipt.privacy_block_count, 1);
    let blocked = receipt
        .items
        .iter()
        .find(|item| item.final_labels.privacy == PrivacyDecision::Block)
        .expect("blocked item");
    assert!(!blocked.accepted);
    assert!(!blocked.resolved);
}

#[test]
fn adjudication_cannot_downgrade_fail_closed_privacy() {
    let invalid = ADJUDICATION_SEED.replace(
        "\t3\tsupported\tnone\tredact\t1c8f53cd8fa5abc73e480aa332ee8d4c3c0ce7db164ff83cccfdd7bac799720a\t",
        "\t3\tsupported\tnone\tallow\t-\t",
    );
    let batch =
        ReviewBatch::parse_tsv(REVIEW_SEED, &invalid)
            .expect("syntactically valid batch");
    let error = evaluate_review_batch(
        &batch,
        &DependencyState::blocked_seed(SOURCE_COMMIT).expect("dependency"),
        &AcceptancePolicy::default(),
    )
    .expect_err("privacy downgrade");
    assert!(error.to_string().contains("downgrade"));
}

#[test]
fn redaction_adjudication_requires_a_redaction_receipt() {
    let invalid = ADJUDICATION_SEED.replace(
        "1c8f53cd8fa5abc73e480aa332ee8d4c3c0ce7db164ff83cccfdd7bac799720a",
        "-",
    );
    let error = ReviewBatch::parse_tsv(REVIEW_SEED, &invalid)
        .expect_err("missing redaction receipt");
    assert!(error.to_string().contains("redaction receipt"));
}

#[test]
fn duplicate_reviewer_commitment_is_rejected() {
    let invalid = REVIEW_SEED.replace(REVIEWER_B, REVIEWER_A);
    let error = ReviewBatch::parse_tsv(&invalid, ADJUDICATION_SEED)
        .expect_err("duplicate reviewer");
    assert!(error.to_string().contains("independent"));
}

#[test]
fn synthetic_header_cannot_claim_reviewed_status() {
    let invalid = REVIEW_SEED.replace(
        "# reviewed=false",
        "# reviewed=true",
    );
    let error = ReviewBatch::parse_tsv(&invalid, ADJUDICATION_SEED)
        .expect_err("synthetic reviewed claim");
    assert!(error.to_string().contains("synthetic review seed"));
}

#[test]
fn receipt_is_byte_deterministic_and_redacts_item_and_reviewer_evidence() {
    let first = seed_receipt();
    let second = seed_receipt();
    assert_eq!(first, second);
    let first_json = first.to_json_pretty();
    assert_eq!(first_json, second.to_json_pretty());
    assert!(!first_json.contains("en-case-001"));
    assert!(!first_json.contains(REVIEWER_A));
    assert!(!first_json.contains(REVIEWER_B));
    assert!(!first_json.contains(FIRST_QUERY));
    assert!(!first_json.contains("rationale"));
}

#[test]
fn receipt_keeps_every_runtime_and_authority_flag_false() {
    let receipt = seed_receipt();
    assert!(!receipt.source_contract_qualified);
    assert!(!receipt.efficacy_validation);
    assert!(!receipt.efficacy_claim);
    assert!(!receipt.product_workspace_member);
    assert!(!receipt.product_module_registered);
    assert!(!receipt.runtime_wired);
    assert!(!receipt.default_recall_changed);
    assert!(!receipt.federation_recall_changed);
    assert!(!receipt.context_attachment);
    assert!(!receipt.physical_send);
    assert!(!receipt.network_access);
    assert!(!receipt.model_download);
    assert!(!receipt.external_effects);
    assert!(!receipt.production_authority);
    assert!(!receipt.operator_acceptance);
    assert!(!receipt.promotion);
    assert!(!receipt.callers_ratchet);
}

#[test]
fn uppercase_digest_is_rejected() {
    let invalid = REVIEW_SEED.replacen(
        FIRST_QUERY,
        &FIRST_QUERY.to_uppercase(),
        1,
    );
    let error = ReviewBatch::parse_tsv(&invalid, ADJUDICATION_SEED)
        .expect_err("uppercase digest");
    assert!(error.to_string().contains("lowercase"));
}
