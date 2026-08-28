use hepta_memory_p1_1c_qualification::{
    CalibrationContract, OfflineCorpus, evaluate_corpus,
};
use hepta_memory_p1_1c1_qualification::{
    AcceptancePolicy, DependencyState, ReviewBatch, evaluate_review_batch,
};
use hepta_memory_p1_1c2_qualification::{
    EfficacyPolicy, EvaluationRequest, P1_1C1_SOURCE_COMMIT, ReviewProjection,
    evaluate_reviewed_corpus,
};

const P1_1C_SEED: &str = include_str!(
    "../../../hepta-memory-p1-1c-qualification/fixtures/p1_1c_multilingual_seed.tsv"
);
const P1_1C1_REVIEW_SEED: &str = include_str!(
    "../../../hepta-memory-p1-1c1-qualification/fixtures/p1_1c1_review_seed.tsv"
);
const P1_1C1_ADJUDICATION_SEED: &str = include_str!(
    "../../../hepta-memory-p1-1c1-qualification/fixtures/p1_1c1_adjudication_seed.tsv"
);
const PROJECTION_SEED: &str = include_str!("../../fixtures/p1_1c2_projection_seed.tsv");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let corpus = OfflineCorpus::parse_tsv(P1_1C_SEED)?;
    let calibration = CalibrationContract::qualification_reference()?;
    let baseline = evaluate_corpus(&corpus, &calibration)?;
    let reviews = ReviewBatch::parse_tsv(P1_1C1_REVIEW_SEED, P1_1C1_ADJUDICATION_SEED)?;
    let dependency = DependencyState::blocked_seed(
        "fe33565ce74c013e574c307e4fab101820c0ea88",
    )?;
    let acceptance_policy = AcceptancePolicy::default();
    let acceptance = evaluate_review_batch(&reviews, &dependency, &acceptance_policy)?;
    let projection = ReviewProjection::parse_tsv(PROJECTION_SEED)?;
    let efficacy_policy = EfficacyPolicy::default();
    let request = EvaluationRequest {
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
    };
    let receipt = evaluate_reviewed_corpus(&request)?;
    print!("{}", receipt.to_json_pretty());
    Ok(())
}
