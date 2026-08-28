use hepta_memory_p1_1c_qualification::{
    CalibrationContract, EvaluationReceipt, OfflineCorpus, evaluate_corpus,
};
use hepta_memory_p1_1c1_qualification::{
    AcceptancePolicy, AcceptanceReceipt, DependencyState, ReviewBatch, evaluate_review_batch,
};
use hepta_memory_p1_1c2_qualification::{
    Digest32, EfficacyPolicy, EvaluationRequest, P1_1C1_SOURCE_COMMIT,
    P1_1C_SOURCE_COMMIT, ReviewProjection, candidate_projection_digest,
    evaluate_reviewed_corpus,
};

const P1_1C_SEED: &str = include_str!(
    "../../hepta-memory-p1-1c-qualification/fixtures/p1_1c_multilingual_seed.tsv"
);
const P1_1C1_REVIEW_SEED: &str = include_str!(
    "../../hepta-memory-p1-1c1-qualification/fixtures/p1_1c1_review_seed.tsv"
);
const P1_1C1_ADJUDICATION_SEED: &str = include_str!(
    "../../hepta-memory-p1-1c1-qualification/fixtures/p1_1c1_adjudication_seed.tsv"
);
const PROJECTION_SEED: &str = include_str!("../fixtures/p1_1c2_projection_seed.tsv");
const EMPTY_ADJUDICATIONS: &str = concat!(
    "# schema=hepta.intelligence.p1_1c1.adjudication_batch.v1\n",
    "item_id\tadjudicator_commitment\trelevance\tcitation\tcontradiction\tprivacy\t",
    "redaction_receipt_sha256\trationale_sha256\n"
);

struct PositiveStack {
    reviews: ReviewBatch,
    dependency: DependencyState,
    acceptance_policy: AcceptancePolicy,
    acceptance: AcceptanceReceipt,
    projection: ReviewProjection,
    reviewed_corpus: OfflineCorpus,
    baseline: EvaluationReceipt,
    calibration: CalibrationContract,
    efficacy_policy: EfficacyPolicy,
}

impl PositiveStack {
    fn request(&self) -> EvaluationRequest<'_> {
        EvaluationRequest {
            review_batch: &self.reviews,
            dependency: &self.dependency,
            acceptance_policy: &self.acceptance_policy,
            acceptance_receipt: &self.acceptance,
            projection: &self.projection,
            reviewed_corpus: &self.reviewed_corpus,
            baseline_receipt: &self.baseline,
            calibration: &self.calibration,
            efficacy_policy: &self.efficacy_policy,
            p1_1c1_source_commit: P1_1C1_SOURCE_COMMIT,
        }
    }
}

fn reviewed_corpus_text() -> String {
    P1_1C_SEED
        .replace("# provenance=synthetic_seed", "# provenance=reviewed_human")
        .replace("# reviewed=false", "# reviewed=true")
}

fn review_tsv_for_corpus(corpus: &OfflineCorpus) -> String {
    let reviewer_a = Digest32::for_bytes(b"qualification-reviewer-a");
    let reviewer_b = Digest32::for_bytes(b"qualification-reviewer-b");
    let mut output = format!(
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
        P1_1C_SOURCE_COMMIT,
        corpus.header.locales.join(",")
    );
    for case in &corpus.cases {
        for candidate in &case.candidates {
            let item_id = format!("{}:{}", case.case_id, candidate.candidate_id);
            let candidate_sha256 = candidate_projection_digest(case, candidate);
            let citation = if candidate.citation_supported {
                "supported"
            } else {
                "unsupported"
            };
            let truth = u64::from(candidate.edge1_truth_ppm)
                + u64::from(candidate.edge2_truth_ppm);
            let contradiction = u64::from(candidate.edge1_contradiction_ppm)
                + u64::from(candidate.edge2_contradiction_ppm);
            let contradiction_label = if contradiction > truth {
                "confirmed"
            } else {
                "none"
            };
            for reviewer in [reviewer_a, reviewer_b] {
                let rationale = Digest32::for_bytes(
                    format!("{item_id}:{reviewer}:qualification-rationale").as_bytes(),
                );
                output.push_str(&format!(
                    "{item_id}\t{}\t{}\t{}\t{}\t{}\t{citation}\t{contradiction_label}\tallow\t{}\n",
                    case.locale,
                    case.query_sha256,
                    candidate_sha256,
                    reviewer,
                    candidate.relevance_grade,
                    rationale
                ));
            }
        }
    }
    output
}

fn projection_tsv_for_corpus(
    corpus: &OfflineCorpus,
    fixture_only: bool,
    omit_last: bool,
    drift_query: bool,
    drift_candidate: bool,
    source_commit: &str,
) -> String {
    let mut rows = Vec::new();
    for case in &corpus.cases {
        for candidate in &case.candidates {
            rows.push((case, candidate));
        }
    }
    if omit_last {
        rows.pop();
    }
    let mut output = format!(
        concat!(
            "# schema=hepta.intelligence.p1_1c2.review_projection.v1\n",
            "# fixture_only={}\n",
            "# p1_1c1_source_commit={}\n",
            "item_id\tcase_id\tcandidate_id\tquery_sha256\tcandidate_sha256\n"
        ),
        fixture_only, source_commit
    );
    for (index, (case, candidate)) in rows.into_iter().enumerate() {
        let query = if drift_query && index == 0 {
            "0000000000000000000000000000000000000000000000000000000000000000"
                .to_string()
        } else {
            case.query_sha256.to_string()
        };
        let candidate_digest = if drift_candidate && index == 0 {
            "1111111111111111111111111111111111111111111111111111111111111111"
                .to_string()
        } else {
            candidate_projection_digest(case, candidate).to_string()
        };
        output.push_str(&format!(
            "{}:{}\t{}\t{}\t{}\t{}\n",
            case.case_id,
            candidate.candidate_id,
            case.case_id,
            candidate.candidate_id,
            query,
            candidate_digest
        ));
    }
    output
}

fn positive_stack() -> PositiveStack {
    let reviewed_corpus =
        OfflineCorpus::parse_tsv(&reviewed_corpus_text()).expect("reviewed corpus");
    let reviews = ReviewBatch::parse_tsv(
        &review_tsv_for_corpus(&reviewed_corpus),
        EMPTY_ADJUDICATIONS,
    )
    .expect("review batch");
    let dependency = DependencyState::qualified(P1_1C_SOURCE_COMMIT).expect("dependency");
    let acceptance_policy = AcceptancePolicy::default();
    let acceptance = evaluate_review_batch(&reviews, &dependency, &acceptance_policy)
        .expect("accepted corpus receipt");
    assert!(acceptance.reviewed_corpus_accepted);
    let projection = ReviewProjection::for_corpus(
        &reviewed_corpus,
        P1_1C1_SOURCE_COMMIT,
        false,
    )
    .expect("complete projection");
    let seed_corpus = OfflineCorpus::parse_tsv(P1_1C_SEED).expect("seed corpus");
    let calibration = CalibrationContract::qualification_reference().expect("calibration");
    let baseline = evaluate_corpus(&seed_corpus, &calibration).expect("baseline");
    PositiveStack {
        reviews,
        dependency,
        acceptance_policy,
        acceptance,
        projection,
        reviewed_corpus,
        baseline,
        calibration,
        efficacy_policy: EfficacyPolicy::default(),
    }
}

fn blocked_seed_receipt() -> hepta_memory_p1_1c2_qualification::ReviewedCorpusEvaluationReceipt {
    let corpus = OfflineCorpus::parse_tsv(P1_1C_SEED).expect("seed corpus");
    let calibration = CalibrationContract::qualification_reference().expect("calibration");
    let baseline = evaluate_corpus(&corpus, &calibration).expect("baseline");
    let reviews = ReviewBatch::parse_tsv(P1_1C1_REVIEW_SEED, P1_1C1_ADJUDICATION_SEED)
        .expect("review seed");
    let dependency = DependencyState::blocked_seed(P1_1C_SOURCE_COMMIT).expect("dependency");
    let acceptance_policy = AcceptancePolicy::default();
    let acceptance = evaluate_review_batch(&reviews, &dependency, &acceptance_policy)
        .expect("review seed receipt");
    let projection = ReviewProjection::parse_tsv(PROJECTION_SEED).expect("projection seed");
    let efficacy_policy = EfficacyPolicy::default();
    evaluate_reviewed_corpus(&EvaluationRequest {
        review_batch: &reviews,
        dependency: &dependency,
        acceptance_policy: &acceptance_policy,
        acceptance_receipt: &acceptance,
        projection: &projection,
        reviewed_corpus: &corpus,
        baseline_receipt: &baseline,
        calibration: &calibration,
        efficacy_policy: &efficacy_policy,
        p1_1c1_source_commit: P1_1C1_SOURCE_COMMIT,
    })
    .expect("blocked receipt")
}

#[test]
fn reviewed_corpus_with_complete_projection_runs_all_seven_lanes() {
    let stack = positive_stack();
    let audit = stack
        .projection
        .audit(&stack.reviews, &stack.reviewed_corpus)
        .expect("projection audit");
    assert!(audit.eligible_for_reviewed_evaluation);
    assert_eq!(audit.evaluation_candidate_count, 48);
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("evaluation");
    assert!(receipt.reviewed_corpus_evaluated);
    assert!(receipt.efficacy_thresholds_passed);
    assert!(receipt.efficacy_validation);
    assert_eq!(receipt.lanes.len(), 7);
    assert_eq!(receipt.status, "PASS_P1_1C2_REVIEWED_CORPUS_EFFICACY_VALIDATION");
}

#[test]
fn checked_in_seed_is_blocked_without_lane_evidence() {
    let receipt = blocked_seed_receipt();
    assert_eq!(
        receipt.status,
        "BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY"
    );
    assert!(!receipt.reviewed_corpus_present);
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(!receipt.efficacy_validation);
    assert!(receipt.lanes.is_empty());
    assert!(!receipt.blocked_reasons.is_empty());
}

#[test]
fn fixture_only_projection_cannot_activate_reviewed_evaluation() {
    let mut stack = positive_stack();
    stack.projection = ReviewProjection::for_corpus(
        &stack.reviewed_corpus,
        P1_1C1_SOURCE_COMMIT,
        true,
    )
    .expect("fixture projection");
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked fixture");
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(receipt
        .blocked_reasons
        .contains(&"projection.fixture_only".to_string()));
}

#[test]
fn incomplete_candidate_coverage_is_blocked() {
    let mut stack = positive_stack();
    stack.projection = ReviewProjection::parse_tsv(&projection_tsv_for_corpus(
        &stack.reviewed_corpus,
        false,
        true,
        false,
        false,
        P1_1C1_SOURCE_COMMIT,
    ))
    .expect("incomplete projection");
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked coverage");
    assert!(!receipt.projection_complete);
    assert!(receipt
        .blocked_reasons
        .contains(&"projection.candidate_coverage_incomplete".to_string()));
}

#[test]
fn query_digest_drift_is_blocked() {
    let mut stack = positive_stack();
    stack.projection = ReviewProjection::parse_tsv(&projection_tsv_for_corpus(
        &stack.reviewed_corpus,
        false,
        false,
        true,
        false,
        P1_1C1_SOURCE_COMMIT,
    ))
    .expect("query-drift projection");
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked query drift");
    assert!(receipt
        .blocked_reasons
        .contains(&"projection.query_digest_mismatch".to_string()));
}

#[test]
fn candidate_digest_drift_is_blocked() {
    let mut stack = positive_stack();
    stack.projection = ReviewProjection::parse_tsv(&projection_tsv_for_corpus(
        &stack.reviewed_corpus,
        false,
        false,
        false,
        true,
        P1_1C1_SOURCE_COMMIT,
    ))
    .expect("candidate-drift projection");
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked candidate drift");
    assert!(receipt
        .blocked_reasons
        .contains(&"projection.candidate_digest_mismatch".to_string()));
}

#[test]
fn projection_bound_to_another_p1c1_head_is_blocked() {
    let mut stack = positive_stack();
    stack.projection = ReviewProjection::parse_tsv(&projection_tsv_for_corpus(
        &stack.reviewed_corpus,
        false,
        false,
        false,
        false,
        "0000000000000000000000000000000000000000",
    ))
    .expect("wrong-head projection");
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked head drift");
    assert!(receipt
        .blocked_reasons
        .contains(&"projection.p1c1_source_commit_mismatch".to_string()));
}

#[test]
fn blocked_acceptance_receipt_cannot_be_reused_with_qualified_dependency() {
    let stack = positive_stack();
    let blocked_dependency =
        DependencyState::blocked_seed(P1_1C_SOURCE_COMMIT).expect("blocked dependency");
    let blocked_acceptance = evaluate_review_batch(
        &stack.reviews,
        &blocked_dependency,
        &stack.acceptance_policy,
    )
    .expect("blocked acceptance");
    let receipt = evaluate_reviewed_corpus(&EvaluationRequest {
        review_batch: &stack.reviews,
        dependency: &stack.dependency,
        acceptance_policy: &stack.acceptance_policy,
        acceptance_receipt: &blocked_acceptance,
        projection: &stack.projection,
        reviewed_corpus: &stack.reviewed_corpus,
        baseline_receipt: &stack.baseline,
        calibration: &stack.calibration,
        efficacy_policy: &stack.efficacy_policy,
        p1_1c1_source_commit: P1_1C1_SOURCE_COMMIT,
    })
    .expect("mismatched acceptance receipt");
    assert!(!receipt.acceptance_receipt_matches);
    assert!(receipt
        .blocked_reasons
        .contains(&"acceptance.receipt_recomputation_mismatch".to_string()));
}

#[test]
fn reviewed_evaluation_receipt_is_deterministic() {
    let stack = positive_stack();
    let first = evaluate_reviewed_corpus(&stack.request()).expect("first receipt");
    let second = evaluate_reviewed_corpus(&stack.request()).expect("second receipt");
    assert_eq!(first, second);
    assert_eq!(first.to_json_pretty(), second.to_json_pretty());
}

#[test]
fn machine_receipt_redacts_queries_candidates_and_reviewers() {
    let stack = positive_stack();
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("receipt");
    let json = receipt.to_json_pretty();
    for forbidden in [
        "Which evidence binds",
        "哪些证据",
        "en-ann-gold",
        "qualification-reviewer-a",
        "qualification-reviewer-b",
        "qualification-rationale",
    ] {
        assert!(!json.contains(forbidden), "leaked {forbidden}");
    }
}

#[test]
fn authority_boundary_remains_frozen_false() {
    let receipt = blocked_seed_receipt();
    assert!(!receipt.source_qualified);
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
fn tampered_machine_receipt_is_rejected() {
    let stack = positive_stack();
    let mut receipt = evaluate_reviewed_corpus(&stack.request()).expect("receipt");
    receipt.efficacy_claim = true;
    assert!(receipt.validate().is_err());
}

#[test]
fn efficacy_policy_digest_is_fail_closed() {
    let mut policy = EfficacyPolicy::default();
    policy.maximum_full_p95_latency_micros = 1;
    assert!(policy.validate().is_err());
}
