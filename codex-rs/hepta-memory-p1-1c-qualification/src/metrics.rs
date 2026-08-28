use crate::{ContractError, Digest32, PPM_DENOMINATOR, RankedCandidate};

const TOP_K: usize = 4;
const NDCG_DISCOUNTS_PPM: [u32; TOP_K] = [1_000_000, 630_930, 500_000, 430_677];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseMetrics {
    pub recall_at_4_ppm: u32,
    pub ndcg_at_4_ppm: u32,
    pub citation_precision_ppm: u32,
    pub latency_micros: u64,
    pub token_cost: u64,
    pub top4_sha256: Digest32,
}

impl CaseMetrics {
    pub fn from_ranking(
        ranking: &[RankedCandidate],
        all_relevance_grades: &[u8],
    ) -> Result<Self, ContractError> {
        if ranking.is_empty() || all_relevance_grades.is_empty() {
            return Err(ContractError::Invalid(
                "case metrics require non-empty ranking and gold labels".to_string(),
            ));
        }
        for candidate in ranking {
            candidate.validate()?;
        }
        if all_relevance_grades.iter().any(|grade| *grade > 3) {
            return Err(ContractError::Invalid(
                "gold relevance grades must be in 0..=3".to_string(),
            ));
        }
        let relevant_total = all_relevance_grades
            .iter()
            .filter(|grade| **grade > 0)
            .count();
        if relevant_total == 0 {
            return Err(ContractError::Invalid(
                "case metrics require at least one relevant candidate".to_string(),
            ));
        }

        let top = &ranking[..ranking.len().min(TOP_K)];
        let retrieved_relevant = top
            .iter()
            .filter(|candidate| candidate.relevance_grade > 0)
            .count();
        let recall_at_4_ppm = ratio_ppm(retrieved_relevant as u128, relevant_total as u128)?;

        let dcg = discounted_gain(top.iter().map(|candidate| candidate.relevance_grade));
        let mut ideal_grades = all_relevance_grades.to_vec();
        ideal_grades.sort_by(|left, right| right.cmp(left));
        let ideal_dcg = discounted_gain(ideal_grades.into_iter());
        if ideal_dcg == 0 {
            return Err(ContractError::Invalid(
                "ideal DCG must be non-zero".to_string(),
            ));
        }
        let ndcg_at_4_ppm = ratio_ppm(dcg, ideal_dcg)?;

        let supported = top
            .iter()
            .filter(|candidate| candidate.citation_supported)
            .count();
        let citation_precision_ppm = ratio_ppm(supported as u128, top.len() as u128)?;
        let latency_micros = top.iter().try_fold(0_u64, |accumulator, candidate| {
            accumulator
                .checked_add(u64::from(candidate.latency_micros))
                .ok_or(ContractError::Overflow)
        })?;
        let token_cost = top.iter().try_fold(0_u64, |accumulator, candidate| {
            accumulator
                .checked_add(u64::from(candidate.token_cost))
                .ok_or(ContractError::Overflow)
        })?;
        let mut canonical = String::new();
        for candidate in top {
            use std::fmt::Write as _;
            write!(
                &mut canonical,
                "{}:{}:{}:{};",
                candidate.candidate_id,
                candidate.score_ppm,
                candidate.relevance_grade,
                candidate.citation_supported
            )
            .expect("writing to String cannot fail");
        }

        Ok(Self {
            recall_at_4_ppm,
            ndcg_at_4_ppm,
            citation_precision_ppm,
            latency_micros,
            token_cost,
            top4_sha256: Digest32::for_bytes(canonical.as_bytes()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneMetrics {
    pub case_count: u32,
    pub mean_recall_at_4_ppm: u32,
    pub mean_ndcg_at_4_ppm: u32,
    pub mean_citation_precision_ppm: u32,
    pub p50_latency_micros: u64,
    pub p95_latency_micros: u64,
    pub mean_token_cost: u64,
    pub cases_sha256: Digest32,
}

impl LaneMetrics {
    pub fn aggregate(cases: &[CaseMetrics]) -> Result<Self, ContractError> {
        if cases.is_empty() {
            return Err(ContractError::Invalid(
                "lane metrics require at least one case".to_string(),
            ));
        }
        let case_count = u32::try_from(cases.len()).map_err(|_| ContractError::Overflow)?;
        let denominator = u128::from(case_count);
        let recall_sum = cases.iter().try_fold(0_u128, |accumulator, value| {
            accumulator
                .checked_add(u128::from(value.recall_at_4_ppm))
                .ok_or(ContractError::Overflow)
        })?;
        let ndcg_sum = cases.iter().try_fold(0_u128, |accumulator, value| {
            accumulator
                .checked_add(u128::from(value.ndcg_at_4_ppm))
                .ok_or(ContractError::Overflow)
        })?;
        let citation_sum = cases.iter().try_fold(0_u128, |accumulator, value| {
            accumulator
                .checked_add(u128::from(value.citation_precision_ppm))
                .ok_or(ContractError::Overflow)
        })?;
        let token_sum = cases.iter().try_fold(0_u128, |accumulator, value| {
            accumulator
                .checked_add(u128::from(value.token_cost))
                .ok_or(ContractError::Overflow)
        })?;
        let mut latencies: Vec<u64> = cases.iter().map(|value| value.latency_micros).collect();
        latencies.sort_unstable();
        let p50_latency_micros = percentile(&latencies, 50)?;
        let p95_latency_micros = percentile(&latencies, 95)?;

        let mut canonical = String::new();
        for case in cases {
            use std::fmt::Write as _;
            write!(
                &mut canonical,
                "{}:{}:{}:{}:{}:{};",
                case.recall_at_4_ppm,
                case.ndcg_at_4_ppm,
                case.citation_precision_ppm,
                case.latency_micros,
                case.token_cost,
                case.top4_sha256
            )
            .expect("writing to String cannot fail");
        }

        Ok(Self {
            case_count,
            mean_recall_at_4_ppm: u32::try_from(recall_sum / denominator)
                .map_err(|_| ContractError::Overflow)?,
            mean_ndcg_at_4_ppm: u32::try_from(ndcg_sum / denominator)
                .map_err(|_| ContractError::Overflow)?,
            mean_citation_precision_ppm: u32::try_from(citation_sum / denominator)
                .map_err(|_| ContractError::Overflow)?,
            p50_latency_micros,
            p95_latency_micros,
            mean_token_cost: u64::try_from(token_sum / denominator)
                .map_err(|_| ContractError::Overflow)?,
            cases_sha256: Digest32::for_bytes(canonical.as_bytes()),
        })
    }
}

fn ratio_ppm(numerator: u128, denominator: u128) -> Result<u32, ContractError> {
    if denominator == 0 {
        return Err(ContractError::Invalid(
            "metric denominator must be non-zero".to_string(),
        ));
    }
    let scaled = numerator
        .checked_mul(u128::from(PPM_DENOMINATOR))
        .ok_or(ContractError::Overflow)?
        / denominator;
    u32::try_from(scaled.min(u128::from(PPM_DENOMINATOR)))
        .map_err(|_| ContractError::Overflow)
}

fn discounted_gain<I>(grades: I) -> u128
where
    I: IntoIterator<Item = u8>,
{
    grades
        .into_iter()
        .take(TOP_K)
        .enumerate()
        .map(|(index, grade)| {
            let gain = match grade {
                0 => 0_u128,
                1 => 1,
                2 => 3,
                _ => 7,
            };
            gain * u128::from(NDCG_DISCOUNTS_PPM[index])
        })
        .sum()
}

fn percentile(sorted: &[u64], percentile: usize) -> Result<u64, ContractError> {
    if sorted.is_empty() || !(1..=100).contains(&percentile) {
        return Err(ContractError::Invalid(
            "percentile requires non-empty data and a percentile in 1..=100".to_string(),
        ));
    }
    let rank = sorted
        .len()
        .checked_mul(percentile)
        .and_then(|value| value.checked_add(99))
        .ok_or(ContractError::Overflow)?
        / 100;
    Ok(sorted[rank.saturating_sub(1)])
}

#[cfg(test)]
mod tests {
    use super::CaseMetrics;
    use crate::{Digest32, RankedCandidate};

    fn candidate(id: &str, grade: u8, citation: bool, score: u32) -> RankedCandidate {
        RankedCandidate {
            candidate_id: id.to_string(),
            relevance_grade: grade,
            citation_supported: citation,
            score_ppm: score,
            latency_micros: 10,
            token_cost: 2,
            feature_sha256: Digest32::for_bytes(id.as_bytes()),
        }
    }

    #[test]
    fn perfect_top_four_metrics_are_one_million_ppm() {
        let ranking = vec![
            candidate("a", 3, true, 900_000),
            candidate("b", 2, true, 800_000),
            candidate("c", 0, true, 700_000),
            candidate("d", 0, true, 600_000),
        ];
        let metrics = CaseMetrics::from_ranking(&ranking, &[3, 2, 0, 0]).expect("metrics");
        assert_eq!(metrics.recall_at_4_ppm, 1_000_000);
        assert_eq!(metrics.ndcg_at_4_ppm, 1_000_000);
        assert_eq!(metrics.citation_precision_ppm, 1_000_000);
    }
}
