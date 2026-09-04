use std::collections::BTreeMap;
use std::collections::BTreeSet;

use codex_hepta_types::Digest32;
use codex_hepta_types::FixedQ32;
use codex_hepta_types::StableId;

use crate::AxisDirection;
use crate::AxisLimit;
use crate::AxisValue;
use crate::CandidateRejectionReason;
use crate::CandidateUtility;
use crate::ContributionSet;
use crate::EvaluationDisposition;
use crate::FeasibilityPosture;
use crate::NduError;
use crate::NduEvaluationReceipt;
use crate::RejectedCandidate;
use crate::ScalarizationProfile;
use crate::UtilityContribution;
use crate::UtilityProfile;
use crate::evaluation_digest::EvaluationDigestInput;
use crate::evaluation_digest::digest_evaluation;
use crate::evaluation_digest::digest_profile;
use crate::evaluation_digest::push_axis_values;
use crate::scoring::pareto_frontier;
use crate::scoring::score_frontier;

const MAX_CONTRIBUTIONS: usize = 4096;
const MAX_CANDIDATES: usize = 128;
const MAX_UTILITY_DIMENSIONS: usize = 8;
const MAX_RISK_RESOURCE_DIMENSIONS: usize = 32;
const MAX_REQUIRED_ORGANS: usize = 32;
const SCALARIZATION_DIGEST_DOMAIN: &[u8] = b"hepta.ndu.scalarization-profile.v1";

#[derive(Default)]
struct CandidateAccumulator {
    organs: BTreeSet<StableId>,
    utility: BTreeMap<StableId, FixedQ32>,
    risk: BTreeMap<StableId, FixedQ32>,
    resource: BTreeMap<StableId, FixedQ32>,
    uncertainty: BTreeMap<StableId, FixedQ32>,
    support_digests: Vec<Digest32>,
    hard_violation: bool,
}

/// Applies hard feasibility, contribution completeness, Pareto filtering and
/// optional registered scalarization. Any recommendation is advisory only.
pub fn evaluate_candidates(
    set: ContributionSet,
    mut profile: UtilityProfile,
    scalarization: Option<ScalarizationProfile>,
) -> Result<NduEvaluationReceipt, NduError> {
    validate_profile(&mut profile)?;
    validate_contribution_envelope(&set)?;
    let utility_profile_digest = digest_profile(&profile);

    let mut grouped: BTreeMap<StableId, CandidateAccumulator> = BTreeMap::new();
    for contribution in set.contributions {
        accumulate(&mut grouped, contribution, &profile)?;
    }
    if grouped.len() > MAX_CANDIDATES {
        return Err(NduError::CandidateLimitExceeded);
    }
    let abstain = stable_id("abstain")?;
    if !grouped.contains_key(&abstain) {
        return Err(NduError::MissingAbstainCandidate);
    }

    let mut evaluated_candidates = Vec::new();
    let mut rejected_candidates = Vec::new();
    for (candidate_id, accumulator) in grouped {
        validate_required_organs(&candidate_id, &accumulator, &profile)?;
        let mut reasons = Vec::new();
        if accumulator.hard_violation {
            reasons.push(CandidateRejectionReason::HardConstraintViolation);
        }
        if exceeds_any(&candidate_id, &accumulator.risk, &profile.risk_ceilings)? {
            reasons.push(CandidateRejectionReason::RiskCeilingExceeded);
        }
        if exceeds_any(
            &candidate_id,
            &accumulator.resource,
            &profile.resource_ceilings,
        )? {
            reasons.push(CandidateRejectionReason::ResourceCeilingExceeded);
        }
        if reasons.is_empty() {
            evaluated_candidates.push(finalize_candidate(candidate_id, accumulator, &profile)?);
        } else {
            rejected_candidates.push(RejectedCandidate {
                candidate_id,
                reasons,
            });
        }
    }

    evaluated_candidates.sort_by(candidate_order);
    rejected_candidates.sort_by(rejected_order);
    if rejected_candidates
        .iter()
        .any(|candidate| candidate.candidate_id == abstain)
    {
        return Err(NduError::AbstainInfeasible);
    }

    let mut frontier = pareto_frontier(&evaluated_candidates, &profile.dimensions);
    frontier.sort_by(candidate_order);
    let scalarization_profile_digest = scalarization
        .as_ref()
        .map(canonical_scalarization_digest)
        .transpose()?;
    let (disposition, advisory_recommendation) =
        if evaluated_candidates.len() == 1 && evaluated_candidates[0].candidate_id == abstain {
            (
                EvaluationDisposition::InfeasibleExplicitAbstain,
                Some(abstain),
            )
        } else if frontier.len() == 1 {
            (
                EvaluationDisposition::UniqueParetoRecommendation,
                Some(frontier[0].candidate_id.clone()),
            )
        } else if let Some(scalarization) = scalarization {
            score_frontier(&mut frontier, &profile, scalarization)?
        } else {
            (EvaluationDisposition::ParetoSetRequiresSlowPath, None)
        };

    let evaluation_digest = digest_evaluation(EvaluationDigestInput {
        objective_digest: set.objective_digest,
        generation: set.generation.get(),
        disposition,
        evaluated: &evaluated_candidates,
        rejected: &rejected_candidates,
        frontier: &frontier,
        advisory: advisory_recommendation.as_ref(),
        utility_profile_digest,
        scalarization_digest: scalarization_profile_digest,
    });
    Ok(NduEvaluationReceipt {
        objective_digest: set.objective_digest,
        generation: set.generation,
        disposition,
        utility_profile_digest,
        scalarization_profile_digest,
        evaluated_candidates,
        rejected_candidates,
        pareto_frontier: frontier,
        advisory_recommendation,
        evaluation_digest,
    })
}

/// Returns the canonical digest of a scalarization profile after validating and
/// normalizing it. This does not register or authorize the profile.
pub fn canonical_scalarization_digest(
    profile: &ScalarizationProfile,
) -> Result<Digest32, NduError> {
    let mut normalized = profile.clone();
    normalize_axis_values(&mut normalized.weights)?;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(SCALARIZATION_DIGEST_DOMAIN);
    push_id(&mut bytes, &normalized.profile_id);
    push_axis_values(&mut bytes, &normalized.weights);
    Ok(Digest32::of_bytes(&bytes))
}

fn validate_contribution_envelope(set: &ContributionSet) -> Result<(), NduError> {
    if set.objective_digest.is_zero() {
        return Err(NduError::EmptyObjectiveDigest);
    }
    if set.contributions.is_empty() {
        return Err(NduError::EmptyContributions);
    }
    if set.contributions.len() > MAX_CONTRIBUTIONS {
        return Err(NduError::ContributionLimitExceeded);
    }
    if set
        .contributions
        .iter()
        .any(|value| value.objective_digest != set.objective_digest)
    {
        return Err(NduError::MixedObjective);
    }
    if set
        .contributions
        .iter()
        .any(|value| value.generation != set.generation)
    {
        return Err(NduError::MixedGeneration);
    }
    Ok(())
}

fn validate_profile(profile: &mut UtilityProfile) -> Result<(), NduError> {
    if profile.dimensions.is_empty()
        || profile.dimensions.len() > MAX_UTILITY_DIMENSIONS
        || profile.risk_ceilings.len() > MAX_RISK_RESOURCE_DIMENSIONS
        || profile.resource_ceilings.len() > MAX_RISK_RESOURCE_DIMENSIONS
    {
        return Err(NduError::DimensionLimitExceeded);
    }
    if profile.required_organs.organ_ids.len() > MAX_REQUIRED_ORGANS {
        return Err(NduError::RequiredOrganLimitExceeded);
    }
    profile.dimensions.sort_by(dimension_order);
    profile.risk_ceilings.sort();
    profile.resource_ceilings.sort();
    profile.required_organs.organ_ids.sort();
    reject_duplicate_ids(profile.dimensions.iter().map(|value| &value.0))?;
    reject_duplicate_ids(profile.risk_ceilings.iter().map(|value| &value.axis))?;
    reject_duplicate_ids(profile.resource_ceilings.iter().map(|value| &value.axis))?;
    reject_duplicate_ids(profile.required_organs.organ_ids.iter())?;
    for limit in profile
        .risk_ceilings
        .iter()
        .chain(profile.resource_ceilings.iter())
    {
        if limit.maximum < FixedQ32::ZERO {
            return Err(NduError::NegativeCeiling(limit.axis.to_string()));
        }
    }
    Ok(())
}

fn reject_duplicate_ids<'a>(values: impl Iterator<Item = &'a StableId>) -> Result<(), NduError> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value.clone()) {
            return Err(NduError::DuplicateAxis(value.to_string()));
        }
    }
    Ok(())
}

fn accumulate(
    grouped: &mut BTreeMap<StableId, CandidateAccumulator>,
    mut contribution: UtilityContribution,
    profile: &UtilityProfile,
) -> Result<(), NduError> {
    if contribution.support_digest.is_zero() {
        return Err(NduError::EmptySupportDigest {
            candidate: contribution.candidate_id.to_string(),
            organ: contribution.organ_id.to_string(),
        });
    }
    validate_contribution_dimensions(&contribution)?;
    normalize_axis_values(&mut contribution.utility)?;
    normalize_axis_values(&mut contribution.risk)?;
    normalize_axis_values(&mut contribution.resource)?;
    normalize_axis_values(&mut contribution.uncertainty)?;
    validate_known_axes(&contribution, profile)?;
    let candidate_id = contribution.candidate_id.clone();
    let accumulator = grouped.entry(candidate_id.clone()).or_default();
    if !accumulator.organs.insert(contribution.organ_id.clone()) {
        return Err(NduError::DuplicateOrganContribution {
            candidate: candidate_id.to_string(),
            organ: contribution.organ_id.to_string(),
        });
    }
    accumulator.hard_violation |=
        contribution.feasibility == FeasibilityPosture::HardConstraintViolation;
    sum_values(&mut accumulator.utility, contribution.utility)?;
    sum_values(&mut accumulator.risk, contribution.risk)?;
    sum_values(&mut accumulator.resource, contribution.resource)?;
    max_values(&mut accumulator.uncertainty, contribution.uncertainty);
    accumulator
        .support_digests
        .push(contribution.support_digest);
    Ok(())
}

fn validate_contribution_dimensions(contribution: &UtilityContribution) -> Result<(), NduError> {
    if contribution.utility.len() > MAX_UTILITY_DIMENSIONS
        || contribution.risk.len() > MAX_RISK_RESOURCE_DIMENSIONS
        || contribution.resource.len() > MAX_RISK_RESOURCE_DIMENSIONS
        || contribution.uncertainty.len() > MAX_UTILITY_DIMENSIONS
    {
        return Err(NduError::DimensionLimitExceeded);
    }
    Ok(())
}

pub(crate) fn normalize_axis_values(values: &mut [AxisValue]) -> Result<(), NduError> {
    values.sort();
    for window in values.windows(2) {
        if window[0].axis == window[1].axis {
            return Err(NduError::DuplicateAxis(window[0].axis.to_string()));
        }
    }
    Ok(())
}

fn validate_known_axes(
    contribution: &UtilityContribution,
    profile: &UtilityProfile,
) -> Result<(), NduError> {
    let utility: BTreeSet<_> = profile.dimensions.iter().map(|value| &value.0).collect();
    let risk: BTreeSet<_> = profile
        .risk_ceilings
        .iter()
        .map(|value| &value.axis)
        .collect();
    let resource: BTreeSet<_> = profile
        .resource_ceilings
        .iter()
        .map(|value| &value.axis)
        .collect();
    for value in contribution
        .utility
        .iter()
        .chain(contribution.uncertainty.iter())
    {
        if !utility.contains(&value.axis) {
            return Err(NduError::UnknownAxis(value.axis.to_string()));
        }
    }
    for value in &contribution.risk {
        if !risk.contains(&value.axis) {
            return Err(NduError::UnknownAxis(value.axis.to_string()));
        }
    }
    for value in &contribution.resource {
        if !resource.contains(&value.axis) {
            return Err(NduError::UnknownAxis(value.axis.to_string()));
        }
    }
    Ok(())
}

fn sum_values(
    target: &mut BTreeMap<StableId, FixedQ32>,
    values: Vec<AxisValue>,
) -> Result<(), NduError> {
    for value in values {
        let current = target.entry(value.axis).or_insert(FixedQ32::ZERO);
        *current = current
            .checked_add(value.value)
            .map_err(|_| NduError::Arithmetic)?;
    }
    Ok(())
}

fn max_values(target: &mut BTreeMap<StableId, FixedQ32>, values: Vec<AxisValue>) {
    for value in values {
        let current = target.entry(value.axis).or_insert(value.value);
        *current = (*current).max(value.value);
    }
}

fn validate_required_organs(
    candidate_id: &StableId,
    accumulator: &CandidateAccumulator,
    profile: &UtilityProfile,
) -> Result<(), NduError> {
    for organ_id in &profile.required_organs.organ_ids {
        if !accumulator.organs.contains(organ_id) {
            return Err(NduError::MissingRequiredOrgan {
                candidate: candidate_id.to_string(),
                organ: organ_id.to_string(),
            });
        }
    }
    Ok(())
}

fn exceeds_any(
    candidate_id: &StableId,
    values: &BTreeMap<StableId, FixedQ32>,
    ceilings: &[AxisLimit],
) -> Result<bool, NduError> {
    for ceiling in ceilings {
        let value = values
            .get(&ceiling.axis)
            .ok_or_else(|| NduError::MissingAxis {
                candidate: candidate_id.to_string(),
                axis: ceiling.axis.to_string(),
            })?;
        if *value > ceiling.maximum {
            return Ok(true);
        }
    }
    Ok(false)
}

fn finalize_candidate(
    candidate_id: StableId,
    mut accumulator: CandidateAccumulator,
    profile: &UtilityProfile,
) -> Result<CandidateUtility, NduError> {
    for (axis, _) in &profile.dimensions {
        if !accumulator.utility.contains_key(axis) {
            return Err(NduError::MissingAxis {
                candidate: candidate_id.to_string(),
                axis: axis.to_string(),
            });
        }
    }
    accumulator.support_digests.sort();
    let mut support = Vec::with_capacity(accumulator.support_digests.len() * 32);
    for digest in accumulator.support_digests {
        support.extend_from_slice(digest.as_array());
    }
    Ok(CandidateUtility {
        candidate_id,
        utility: into_axis_values(accumulator.utility),
        risk: into_axis_values(accumulator.risk),
        resource: into_axis_values(accumulator.resource),
        uncertainty: into_axis_values(accumulator.uncertainty),
        support_digest: Digest32::of_bytes(&support),
        scalar_score: None,
    })
}

fn into_axis_values(values: BTreeMap<StableId, FixedQ32>) -> Vec<AxisValue> {
    values
        .into_iter()
        .map(|(axis, value)| AxisValue { axis, value })
        .collect()
}

fn stable_id(value: &str) -> Result<StableId, NduError> {
    StableId::new(value).map_err(|_| NduError::Arithmetic)
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    push_len(bytes, raw.len());
    bytes.extend_from_slice(raw);
}

fn push_len(bytes: &mut Vec<u8>, value: usize) {
    let converted = u32::try_from(value).unwrap_or(u32::MAX);
    bytes.extend_from_slice(&converted.to_be_bytes());
}

fn dimension_order(
    left: &(StableId, AxisDirection),
    right: &(StableId, AxisDirection),
) -> std::cmp::Ordering {
    (&left.0, left.1).cmp(&(&right.0, right.1))
}

fn candidate_order(left: &CandidateUtility, right: &CandidateUtility) -> std::cmp::Ordering {
    left.candidate_id.cmp(&right.candidate_id)
}

fn rejected_order(left: &RejectedCandidate, right: &RejectedCandidate) -> std::cmp::Ordering {
    left.candidate_id.cmp(&right.candidate_id)
}

#[cfg(test)]
#[path = "evaluator_tests.rs"]
mod tests;
