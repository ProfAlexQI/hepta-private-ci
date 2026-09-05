//! Bounded, deterministic Bellman-target candidate builder for qualification space.
//! It cannot mutate an online policy, activate an artifact, or write production state.
#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::Digest32;
use codex_hepta_types::FixedQ32;
use codex_hepta_types::Generation;
use codex_hepta_types::StableId;

const MAX_SAMPLES: usize = 16_384;
const SCALE: i128 = 1_i128 << 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transition {
    pub sample_id: StableId,
    pub state_id: StableId,
    pub action_id: StableId,
    pub reward: FixedQ32,
    pub next_value: FixedQ32,
    pub terminal: bool,
    pub support_digest: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatasetSnapshot {
    pub snapshot_id: StableId,
    pub objective_digest: Digest32,
    pub source_head_digest: Digest32,
    pub transitions: Vec<Transition>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingRequest {
    pub artifact_id: StableId,
    pub producer_id: StableId,
    pub generation: Generation,
    pub gamma: FixedQ32,
    pub dataset: DatasetSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BellmanTarget {
    pub sample_id: StableId,
    pub target: FixedQ32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegularityProfile {
    pub sample_count: u32,
    pub maximum_absolute_target: FixedQ32,
    pub terminal_fraction: FixedQ32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BellmanOperatorArtifact {
    pub artifact_id: StableId,
    pub producer_id: StableId,
    pub generation: Generation,
    pub objective_digest: Digest32,
    pub dataset_digest: Digest32,
    pub artifact_digest: Digest32,
    pub targets: Vec<BellmanTarget>,
    pub regularity: RegularityProfile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyDataset,
    SampleLimitExceeded,
    DuplicateSample(String),
    EmptyDigest(&'static str),
    InvalidGamma,
    Arithmetic,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl StdError for Error {}

pub fn train(mut request: TrainingRequest) -> Result<BellmanOperatorArtifact, Error> {
    if request.dataset.transitions.is_empty() {
        return Err(Error::EmptyDataset);
    }
    if request.dataset.transitions.len() > MAX_SAMPLES {
        return Err(Error::SampleLimitExceeded);
    }
    if request.gamma.raw() < 0 || request.gamma.raw() > FixedQ32::ONE.raw() {
        return Err(Error::InvalidGamma);
    }
    if request.dataset.objective_digest.is_zero() {
        return Err(Error::EmptyDigest("objective"));
    }
    if request.dataset.source_head_digest.is_zero() {
        return Err(Error::EmptyDigest("source head"));
    }
    request
        .dataset
        .transitions
        .sort_by(|left, right| left.sample_id.cmp(&right.sample_id));
    let mut seen = BTreeSet::new();
    for sample in &request.dataset.transitions {
        if !seen.insert(sample.sample_id.clone()) {
            return Err(Error::DuplicateSample(sample.sample_id.to_string()));
        }
        if sample.support_digest.is_zero() {
            return Err(Error::EmptyDigest("sample support"));
        }
    }

    let mut targets = Vec::with_capacity(request.dataset.transitions.len());
    let mut maximum = 0_i64;
    let mut terminal_count = 0_u64;
    for sample in &request.dataset.transitions {
        let continuation = if sample.terminal {
            terminal_count += 1;
            FixedQ32::ZERO
        } else {
            mul_q32(request.gamma, sample.next_value)?
        };
        let target = add_q32(sample.reward, continuation)?;
        maximum = maximum.max(target.raw().checked_abs().ok_or(Error::Arithmetic)?);
        targets.push(BellmanTarget {
            sample_id: sample.sample_id.clone(),
            target,
        });
    }

    let count = i128::try_from(targets.len()).map_err(|_| Error::Arithmetic)?;
    let terminal_raw = (i128::from(terminal_count) * SCALE) / count;
    let regularity = RegularityProfile {
        sample_count: u32::try_from(targets.len()).map_err(|_| Error::Arithmetic)?,
        maximum_absolute_target: FixedQ32::from_raw(maximum),
        terminal_fraction: FixedQ32::from_raw(
            i64::try_from(terminal_raw).map_err(|_| Error::Arithmetic)?,
        ),
    };
    let dataset_digest = digest_dataset(&request.dataset);
    let artifact_digest = digest_artifact(&request, dataset_digest, &targets, &regularity);
    Ok(BellmanOperatorArtifact {
        artifact_id: request.artifact_id,
        producer_id: request.producer_id,
        generation: request.generation,
        objective_digest: request.dataset.objective_digest,
        dataset_digest,
        artifact_digest,
        targets,
        regularity,
    })
}

fn mul_q32(left: FixedQ32, right: FixedQ32) -> Result<FixedQ32, Error> {
    let product = i128::from(left.raw()) * i128::from(right.raw());
    let adjusted = if product >= 0 {
        product + SCALE / 2
    } else {
        product - SCALE / 2
    };
    Ok(FixedQ32::from_raw(
        i64::try_from(adjusted / SCALE).map_err(|_| Error::Arithmetic)?,
    ))
}

fn add_q32(left: FixedQ32, right: FixedQ32) -> Result<FixedQ32, Error> {
    let raw = i128::from(left.raw()) + i128::from(right.raw());
    Ok(FixedQ32::from_raw(
        i64::try_from(raw).map_err(|_| Error::Arithmetic)?,
    ))
}

fn digest_dataset(dataset: &DatasetSnapshot) -> Digest32 {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.bellman.dataset.v1");
    push_id(&mut bytes, &dataset.snapshot_id);
    bytes.extend_from_slice(dataset.objective_digest.as_array());
    bytes.extend_from_slice(dataset.source_head_digest.as_array());
    for sample in &dataset.transitions {
        push_id(&mut bytes, &sample.sample_id);
        push_id(&mut bytes, &sample.state_id);
        push_id(&mut bytes, &sample.action_id);
        bytes.extend_from_slice(&sample.reward.raw().to_be_bytes());
        bytes.extend_from_slice(&sample.next_value.raw().to_be_bytes());
        bytes.push(u8::from(sample.terminal));
        bytes.extend_from_slice(sample.support_digest.as_array());
    }
    Digest32::of_bytes(&bytes)
}

fn digest_artifact(
    request: &TrainingRequest,
    dataset: Digest32,
    targets: &[BellmanTarget],
    regularity: &RegularityProfile,
) -> Digest32 {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.bellman-operator.artifact.v1");
    push_id(&mut bytes, &request.artifact_id);
    push_id(&mut bytes, &request.producer_id);
    bytes.extend_from_slice(&request.generation.get().to_be_bytes());
    bytes.extend_from_slice(dataset.as_array());
    bytes.extend_from_slice(&request.gamma.raw().to_be_bytes());
    for target in targets {
        push_id(&mut bytes, &target.sample_id);
        bytes.extend_from_slice(&target.target.raw().to_be_bytes());
    }
    bytes.extend_from_slice(&regularity.sample_count.to_be_bytes());
    bytes.extend_from_slice(&regularity.maximum_absolute_target.raw().to_be_bytes());
    bytes.extend_from_slice(&regularity.terminal_fraction.raw().to_be_bytes());
    Digest32::of_bytes(&bytes)
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    let length = u32::try_from(raw.len()).unwrap_or(u32::MAX);
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
