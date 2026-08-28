use crate::{ContractError, Digest32, checked_ppm, validate_id};

pub const PPM_DENOMINATOR: u32 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AblationLane {
    Lexical,
    Vector,
    Kg,
    LexicalVector,
    LexicalKg,
    VectorKg,
    Full,
}

impl AblationLane {
    pub const ALL: [Self; 7] = [
        Self::Lexical,
        Self::Vector,
        Self::Kg,
        Self::LexicalVector,
        Self::LexicalKg,
        Self::VectorKg,
        Self::Full,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lexical => "lexical",
            Self::Vector => "vector",
            Self::Kg => "kg",
            Self::LexicalVector => "lexical_vector",
            Self::LexicalKg => "lexical_kg",
            Self::VectorKg => "vector_kg",
            Self::Full => "lexical_vector_kg_reranked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalibrationContract {
    pub contract_id: String,
    pub lexical_weight_ppm: u32,
    pub vector_weight_ppm: u32,
    pub kg_weight_ppm: u32,
    pub grounding_weight_ppm: u32,
    pub truth_weight_ppm: u32,
    pub citation_weight_ppm: u32,
    pub contradiction_penalty_ppm: u32,
    pub contract_sha256: Digest32,
    pub learned_weights: bool,
    pub source_reviewed: bool,
    pub production_calibrated: bool,
}

impl CalibrationContract {
    pub fn qualification_reference() -> Result<Self, ContractError> {
        let mut contract = Self {
            contract_id: "p1-1c-fixed-point-reference-v1".to_string(),
            lexical_weight_ppm: 150_000,
            vector_weight_ppm: 250_000,
            kg_weight_ppm: 200_000,
            grounding_weight_ppm: 100_000,
            truth_weight_ppm: 200_000,
            citation_weight_ppm: 100_000,
            contradiction_penalty_ppm: 300_000,
            contract_sha256: Digest32::for_bytes(b"pending"),
            learned_weights: false,
            source_reviewed: false,
            production_calibrated: false,
        };
        contract.contract_sha256 = Digest32::for_bytes(contract.canonical_payload().as_bytes());
        contract.validate()?;
        Ok(contract)
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.contract_id, "calibration contract ID")?;
        let positive_weights = [
            self.lexical_weight_ppm,
            self.vector_weight_ppm,
            self.kg_weight_ppm,
            self.grounding_weight_ppm,
            self.truth_weight_ppm,
            self.citation_weight_ppm,
        ];
        let sum = positive_weights.iter().try_fold(0_u32, |accumulator, value| {
            checked_ppm(*value, "calibration weight")?;
            accumulator.checked_add(*value).ok_or(ContractError::Overflow)
        })?;
        if sum != PPM_DENOMINATOR {
            return Err(ContractError::Invalid(format!(
                "positive calibration weights must sum to {PPM_DENOMINATOR} ppm"
            )));
        }
        checked_ppm(
            self.contradiction_penalty_ppm,
            "contradiction penalty",
        )?;
        if self.contract_sha256 != Digest32::for_bytes(self.canonical_payload().as_bytes()) {
            return Err(ContractError::Corrupt(
                "calibration contract digest does not match fixed-point weights".to_string(),
            ));
        }
        if self.learned_weights || self.source_reviewed || self.production_calibrated {
            return Err(ContractError::Invalid(
                "qualification reference weights must not claim learning, review, or production calibration"
                    .to_string(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn score(&self, lane: AblationLane, features: &CandidateFeatures) -> u32 {
        match lane {
            AblationLane::Lexical => features.lexical_ppm,
            AblationLane::Vector => features.vector_ppm,
            AblationLane::Kg => features.kg_net_support_ppm,
            AblationLane::LexicalVector => {
                weighted_pair(features.lexical_ppm, 450_000, features.vector_ppm, 550_000)
            }
            AblationLane::LexicalKg => weighted_pair(
                features.lexical_ppm,
                450_000,
                features.kg_net_support_ppm,
                550_000,
            ),
            AblationLane::VectorKg => weighted_pair(
                features.vector_ppm,
                550_000,
                features.kg_net_support_ppm,
                450_000,
            ),
            AblationLane::Full => self.full_score(features),
        }
    }

    fn full_score(&self, features: &CandidateFeatures) -> u32 {
        let positive = [
            (features.lexical_ppm, self.lexical_weight_ppm),
            (features.vector_ppm, self.vector_weight_ppm),
            (features.kg_net_support_ppm, self.kg_weight_ppm),
            (features.grounding_ppm, self.grounding_weight_ppm),
            (features.truth_ppm, self.truth_weight_ppm),
            (features.citation_ppm, self.citation_weight_ppm),
        ]
        .into_iter()
        .fold(0_u128, |accumulator, (value, weight)| {
            accumulator + u128::from(value) * u128::from(weight)
        })
            / u128::from(PPM_DENOMINATOR);
        let penalty = u128::from(features.contradiction_ppm)
            * u128::from(self.contradiction_penalty_ppm)
            / u128::from(PPM_DENOMINATOR);
        let score = positive.saturating_sub(penalty);
        u32::try_from(score.min(u128::from(PPM_DENOMINATOR))).unwrap_or(PPM_DENOMINATOR)
    }

    fn canonical_payload(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.contract_id,
            self.lexical_weight_ppm,
            self.vector_weight_ppm,
            self.kg_weight_ppm,
            self.grounding_weight_ppm,
            self.truth_weight_ppm,
            self.citation_weight_ppm,
            self.contradiction_penalty_ppm,
            self.learned_weights,
            self.source_reviewed,
            self.production_calibrated
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateFeatures {
    pub lexical_ppm: u32,
    pub vector_ppm: u32,
    pub kg_net_support_ppm: u32,
    pub grounding_ppm: u32,
    pub truth_ppm: u32,
    pub contradiction_ppm: u32,
    pub citation_ppm: u32,
    pub feature_sha256: Digest32,
}

impl CandidateFeatures {
    pub fn new(
        lexical_ppm: u32,
        vector_ppm: u32,
        kg_net_support_ppm: u32,
        grounding_ppm: u32,
        truth_ppm: u32,
        contradiction_ppm: u32,
        citation_ppm: u32,
    ) -> Result<Self, ContractError> {
        for (value, field) in [
            (lexical_ppm, "lexical feature"),
            (vector_ppm, "vector feature"),
            (kg_net_support_ppm, "KG net support feature"),
            (grounding_ppm, "grounding feature"),
            (truth_ppm, "truth feature"),
            (contradiction_ppm, "contradiction feature"),
            (citation_ppm, "citation feature"),
        ] {
            checked_ppm(value, field)?;
        }
        let payload = format!(
            "{lexical_ppm}|{vector_ppm}|{kg_net_support_ppm}|{grounding_ppm}|{truth_ppm}|{contradiction_ppm}|{citation_ppm}"
        );
        Ok(Self {
            lexical_ppm,
            vector_ppm,
            kg_net_support_ppm,
            grounding_ppm,
            truth_ppm,
            contradiction_ppm,
            citation_ppm,
            feature_sha256: Digest32::for_bytes(payload.as_bytes()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedCandidate {
    pub candidate_id: String,
    pub relevance_grade: u8,
    pub citation_supported: bool,
    pub score_ppm: u32,
    pub latency_micros: u32,
    pub token_cost: u32,
    pub feature_sha256: Digest32,
}

impl RankedCandidate {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.candidate_id, "ranked candidate ID")?;
        if self.relevance_grade > 3 {
            return Err(ContractError::Invalid(
                "ranked relevance grade must be in 0..=3".to_string(),
            ));
        }
        checked_ppm(self.score_ppm, "ranked score")?;
        if self.latency_micros == 0 {
            return Err(ContractError::Invalid(
                "ranked latency must be non-zero".to_string(),
            ));
        }
        Ok(())
    }
}

fn weighted_pair(first: u32, first_weight: u32, second: u32, second_weight: u32) -> u32 {
    let weighted = u128::from(first) * u128::from(first_weight)
        + u128::from(second) * u128::from(second_weight);
    u32::try_from(weighted / u128::from(PPM_DENOMINATOR)).unwrap_or(PPM_DENOMINATOR)
}

#[cfg(test)]
mod tests {
    use super::{AblationLane, CalibrationContract, CandidateFeatures};

    #[test]
    fn contradiction_penalty_changes_full_ranking() {
        let calibration = CalibrationContract::qualification_reference().expect("calibration");
        let clean = CandidateFeatures::new(
            600_000, 800_000, 800_000, 1_000_000, 850_000, 10_000, 1_000_000,
        )
        .expect("features");
        let contradicted = CandidateFeatures::new(
            900_000, 900_000, 900_000, 1_000_000, 900_000, 900_000, 0,
        )
        .expect("features");
        assert!(
            calibration.score(AblationLane::Full, &clean)
                > calibration.score(AblationLane::Full, &contradicted)
        );
    }
}
