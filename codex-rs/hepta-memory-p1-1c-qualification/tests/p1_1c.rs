use hepta_memory_p1_1c_qualification::{
    AblationLane, CalibrationContract, CorpusProvenance, OfflineCorpus, evaluate_corpus,
};

const SEED_CORPUS: &str = include_str!("../fixtures/p1_1c_multilingual_seed.tsv");

fn evaluated() -> hepta_memory_p1_1c_qualification::EvaluationReceipt {
    let corpus = OfflineCorpus::parse_tsv(SEED_CORPUS).expect("seed corpus");
    let calibration = CalibrationContract::qualification_reference().expect("calibration");
    evaluate_corpus(&corpus, &calibration).expect("evaluation")
}

#[test]
fn seed_corpus_has_eight_locales_and_forty_eight_candidates() {
    let corpus = OfflineCorpus::parse_tsv(SEED_CORPUS).expect("seed corpus");
    assert_eq!(corpus.cases.len(), 8);
    assert_eq!(corpus.header.locales.len(), 8);
    assert_eq!(
        corpus
            .cases
            .iter()
            .map(|case| case.candidates.len())
            .sum::<usize>(),
        48
    );
    assert_eq!(corpus.header.provenance, CorpusProvenance::SyntheticSeed);
    assert!(!corpus.header.reviewed);
}

#[test]
fn provenance_and_review_flag_must_agree() {
    let invalid = SEED_CORPUS.replace("# reviewed=false", "# reviewed=true");
    let error = OfflineCorpus::parse_tsv(&invalid).expect_err("review mismatch");
    assert!(error.to_string().contains("provenance"));
}

#[test]
fn evaluation_is_byte_deterministic() {
    let first = evaluated();
    let second = evaluated();
    assert_eq!(first, second);
    assert_eq!(first.to_json_pretty(), second.to_json_pretty());
}

#[test]
fn all_seven_ablation_lanes_are_emitted_once() {
    let receipt = evaluated();
    assert_eq!(receipt.lanes.len(), AblationLane::ALL.len());
    for lane in AblationLane::ALL {
        assert_eq!(
            receipt
                .lanes
                .iter()
                .filter(|candidate| candidate.lane == lane)
                .count(),
            1
        );
    }
}

#[test]
fn full_lane_outperforms_lexical_and_vector_seed_baselines() {
    let receipt = evaluated();
    let lane = |target| {
        receipt
            .lanes
            .iter()
            .find(|candidate| candidate.lane == target)
            .expect("lane")
    };
    let full = lane(AblationLane::Full);
    let lexical = lane(AblationLane::Lexical);
    let vector = lane(AblationLane::Vector);
    let kg = lane(AblationLane::Kg);

    assert!(full.metrics.mean_recall_at_4_ppm > lexical.metrics.mean_recall_at_4_ppm);
    assert!(full.metrics.mean_recall_at_4_ppm > vector.metrics.mean_recall_at_4_ppm);
    assert!(full.metrics.mean_ndcg_at_4_ppm > lexical.metrics.mean_ndcg_at_4_ppm);
    assert!(full.metrics.mean_ndcg_at_4_ppm > vector.metrics.mean_ndcg_at_4_ppm);
    assert!(
        full.metrics.mean_citation_precision_ppm
            > lexical.metrics.mean_citation_precision_ppm
    );
    assert!(full.metrics.mean_ndcg_at_4_ppm >= kg.metrics.mean_ndcg_at_4_ppm);
}

#[test]
fn seed_receipt_keeps_all_runtime_and_authority_flags_false() {
    let receipt = evaluated();
    assert!(receipt.deterministic);
    assert!(receipt.offline);
    assert!(!receipt.network_access);
    assert!(!receipt.model_download);
    assert!(!receipt.product_workspace_member);
    assert!(!receipt.product_module_registered);
    assert!(!receipt.runtime_wired);
    assert!(!receipt.default_recall_changed);
    assert!(!receipt.federation_recall_changed);
    assert!(!receipt.context_attachment);
    assert!(!receipt.physical_send);
    assert!(!receipt.external_effects);
    assert!(!receipt.production_authority);
    assert!(!receipt.efficacy_validation);
    assert!(!receipt.efficacy_claim);
    assert!(!receipt.operator_acceptance);
    assert!(!receipt.promotion);
    assert!(!receipt.callers_ratchet);
}

#[test]
fn receipt_redacts_query_and_kg_node_text() {
    let json = evaluated().to_json_pretty();
    assert!(!json.contains("Which evidence binds"));
    assert!(!json.contains("哪些证据"));
    assert!(!json.contains("en-ann-gold-s"));
    assert!(!json.contains("zh-ann-gold-m"));
}

#[test]
fn corpus_rejects_scores_above_one_million_ppm() {
    let invalid = SEED_CORPUS.replacen("\t990000\t", "\t1000001\t", 1);
    let error = OfflineCorpus::parse_tsv(&invalid).expect_err("oversized score");
    assert!(error.to_string().contains("exceeds"));
}

#[test]
fn calibration_never_claims_learned_or_production_weights() {
    let calibration = CalibrationContract::qualification_reference().expect("calibration");
    assert!(!calibration.learned_weights);
    assert!(!calibration.source_reviewed);
    assert!(!calibration.production_calibrated);
    calibration.validate().expect("valid calibration");
}
