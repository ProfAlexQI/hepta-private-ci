use crate::{
    ContractError, ReviewBatch, ReviewTuple, SCORE_SCALE_PPM, usize_to_u32,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgreementMetrics {
    pub item_count: u32,
    pub exact_tuple_agreement_ppm: u32,
    pub relevance_agreement_ppm: u32,
    pub citation_agreement_ppm: u32,
    pub contradiction_agreement_ppm: u32,
    pub privacy_agreement_ppm: u32,
    pub weighted_relevance_kappa_ppm: i32,
}

impl AgreementMetrics {
    pub fn calculate(batch: &ReviewBatch) -> Result<Self, ContractError> {
        let pairs = batch.grouped_reviews()?;
        if pairs.is_empty() {
            return Err(ContractError::Invalid(
                "agreement metrics require at least one reviewed item".to_string(),
            ));
        }

        let mut exact = 0_u64;
        let mut relevance = 0_u64;
        let mut citation = 0_u64;
        let mut contradiction = 0_u64;
        let mut privacy = 0_u64;
        let mut relevance_pairs = Vec::with_capacity(pairs.len());

        for pair in pairs.values() {
            let left = pair[0].labels;
            let right = pair[1].labels;
            exact += u64::from(left == right);
            relevance += u64::from(left.relevance == right.relevance);
            citation += u64::from(left.citation == right.citation);
            contradiction +=
                u64::from(left.contradiction == right.contradiction);
            privacy += u64::from(left.privacy == right.privacy);
            relevance_pairs.push((left.relevance, right.relevance));
        }

        let count = u64::try_from(pairs.len()).map_err(|_| ContractError::Overflow)?;
        Ok(Self {
            item_count: usize_to_u32(pairs.len())?,
            exact_tuple_agreement_ppm: ratio_ppm(exact, count)?,
            relevance_agreement_ppm: ratio_ppm(relevance, count)?,
            citation_agreement_ppm: ratio_ppm(citation, count)?,
            contradiction_agreement_ppm: ratio_ppm(contradiction, count)?,
            privacy_agreement_ppm: ratio_ppm(privacy, count)?,
            weighted_relevance_kappa_ppm:
                weighted_relevance_kappa_ppm(&relevance_pairs)?,
        })
    }

    pub(crate) fn digest_parts(&self) -> [[u8; 4]; 7] {
        [
            self.item_count.to_be_bytes(),
            self.exact_tuple_agreement_ppm.to_be_bytes(),
            self.relevance_agreement_ppm.to_be_bytes(),
            self.citation_agreement_ppm.to_be_bytes(),
            self.contradiction_agreement_ppm.to_be_bytes(),
            self.privacy_agreement_ppm.to_be_bytes(),
            self.weighted_relevance_kappa_ppm.to_be_bytes(),
        ]
    }
}

fn ratio_ppm(numerator: u64, denominator: u64) -> Result<u32, ContractError> {
    if denominator == 0 {
        return Err(ContractError::Invalid(
            "agreement denominator must be non-zero".to_string(),
        ));
    }
    let scaled = u128::from(numerator)
        .checked_mul(u128::from(SCORE_SCALE_PPM))
        .ok_or(ContractError::Overflow)?
        / u128::from(denominator);
    u32::try_from(scaled).map_err(|_| ContractError::Overflow)
}

fn weighted_relevance_kappa_ppm(
    pairs: &[(u8, u8)],
) -> Result<i32, ContractError> {
    if pairs.is_empty() {
        return Err(ContractError::Invalid(
            "weighted kappa requires at least one pair".to_string(),
        ));
    }
    let mut left_counts = [0_u64; 4];
    let mut right_counts = [0_u64; 4];
    let mut observed_weight_sum = 0_u128;

    for (left, right) in pairs {
        if *left > 3 || *right > 3 {
            return Err(ContractError::Corrupt(
                "weighted kappa received an out-of-range relevance grade"
                    .to_string(),
            ));
        }
        left_counts[usize::from(*left)] += 1;
        right_counts[usize::from(*right)] += 1;
        observed_weight_sum = observed_weight_sum
            .checked_add(u128::from(quadratic_weight_ppm(*left, *right)))
            .ok_or(ContractError::Overflow)?;
    }

    let count = u128::try_from(pairs.len()).map_err(|_| ContractError::Overflow)?;
    let observed_ppm = observed_weight_sum / count;

    let mut expected_weight_sum = 0_u128;
    for (left, left_count) in left_counts.iter().copied().enumerate() {
        for (right, right_count) in right_counts.iter().copied().enumerate() {
            let pair_count = u128::from(left_count)
                .checked_mul(u128::from(right_count))
                .ok_or(ContractError::Overflow)?;
            let weight = quadratic_weight_ppm(
                u8::try_from(left).map_err(|_| ContractError::Overflow)?,
                u8::try_from(right).map_err(|_| ContractError::Overflow)?,
            );
            expected_weight_sum = expected_weight_sum
                .checked_add(
                    pair_count
                        .checked_mul(u128::from(weight))
                        .ok_or(ContractError::Overflow)?,
                )
                .ok_or(ContractError::Overflow)?;
        }
    }
    let expected_ppm = expected_weight_sum
        / count
            .checked_mul(count)
            .ok_or(ContractError::Overflow)?;

    let scale = i128::from(SCORE_SCALE_PPM);
    let observed = i128::try_from(observed_ppm).map_err(|_| ContractError::Overflow)?;
    let expected = i128::try_from(expected_ppm).map_err(|_| ContractError::Overflow)?;

    let kappa = if expected == scale {
        if observed == scale {
            scale
        } else {
            -scale
        }
    } else {
        (observed - expected)
            .checked_mul(scale)
            .ok_or(ContractError::Overflow)?
            / (scale - expected)
    };
    let clamped = kappa.clamp(-scale, scale);
    i32::try_from(clamped).map_err(|_| ContractError::Overflow)
}

fn quadratic_weight_ppm(left: u8, right: u8) -> u32 {
    let difference = u32::from(left.abs_diff(right));
    SCORE_SCALE_PPM - difference * difference * SCORE_SCALE_PPM / 9
}

pub(crate) fn labels_match_any(
    final_labels: ReviewTuple,
    left: ReviewTuple,
    right: ReviewTuple,
) -> bool {
    (final_labels.relevance == left.relevance
        || final_labels.relevance == right.relevance)
        && (final_labels.citation == left.citation
            || final_labels.citation == right.citation)
        && (final_labels.contradiction == left.contradiction
            || final_labels.contradiction == right.contradiction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_relevance_agreement_has_full_kappa() {
        let pairs = [(3, 3), (2, 2), (1, 1), (0, 0)];
        assert_eq!(
            weighted_relevance_kappa_ppm(&pairs).expect("kappa"),
            1_000_000
        );
    }

    #[test]
    fn deterministic_seed_shape_has_positive_kappa() {
        let pairs = [
            (3, 3),
            (3, 2),
            (2, 2),
            (3, 3),
            (2, 2),
            (2, 1),
            (3, 3),
            (2, 2),
        ];
        assert_eq!(
            weighted_relevance_kappa_ppm(&pairs).expect("kappa"),
            666_666
        );
    }
}
