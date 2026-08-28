use hepta_memory_p1_1c_qualification::{
    CalibrationContract, OfflineCorpus, evaluate_corpus,
};

const SEED_CORPUS: &str = include_str!("../../fixtures/p1_1c_multilingual_seed.tsv");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let corpus = OfflineCorpus::parse_tsv(SEED_CORPUS)?;
    let calibration = CalibrationContract::qualification_reference()?;
    let receipt = evaluate_corpus(&corpus, &calibration)?;
    print!("{}", receipt.to_json_pretty());
    Ok(())
}
