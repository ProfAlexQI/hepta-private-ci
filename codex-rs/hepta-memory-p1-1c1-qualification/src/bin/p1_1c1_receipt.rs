use hepta_memory_p1_1c1_qualification::{
    AcceptancePolicy, DependencyState, ReviewBatch, evaluate_review_batch,
};

const REVIEW_SEED: &str =
    include_str!("../../fixtures/p1_1c1_review_seed.tsv");
const ADJUDICATION_SEED: &str =
    include_str!("../../fixtures/p1_1c1_adjudication_seed.tsv");

fn main() {
    let batch =
        ReviewBatch::parse_tsv(REVIEW_SEED, ADJUDICATION_SEED)
            .expect("P1.1c.1 review seed must parse");
    let dependency = DependencyState::blocked_seed(
        "fe33565ce74c013e574c307e4fab101820c0ea88",
    )
    .expect("blocked dependency state");
    let receipt = evaluate_review_batch(
        &batch,
        &dependency,
        &AcceptancePolicy::default(),
    )
    .expect("P1.1c.1 review pipeline evaluation");
    print!("{}", receipt.to_json_pretty());
}
