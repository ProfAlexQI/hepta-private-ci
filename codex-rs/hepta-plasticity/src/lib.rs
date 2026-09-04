//! Governed parameter and topology proposal generation.
//!
//! Proposals are next-generation, qualification-only artifacts. This crate has
//! no API for runtime mutation, authority mutation, self-promotion or release.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error as StdError;
use std::fmt;

use codex_hepta_types::{AuthorityPosture, Digest32, FixedQ32, Generation, StableId};

const MAX_PARAMETER_DELTAS: usize = 4_096;
const MAX_TOPOLOGY_DELTAS: usize = 256;
const MAX_PROPOSALS: usize = 4_096;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ParameterDelta {
    pub parameter_id: StableId,
    pub delta: FixedQ32,
    pub lower_bound: FixedQ32,
    pub upper_bound: FixedQ32,
    pub evidence_digest: Digest32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TopologyOperation {
    Add,
    Remove,
    Replace,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TopologyDelta {
    pub module_id: StableId,
    pub operation: TopologyOperation,
    pub predecessor_digest: Digest32,
    pub candidate_digest: Digest32,
    pub evidence_digest: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalRequest {
    pub proposal_id: StableId,
    pub proposer_id: StableId,
    pub evaluator_id: StableId,
    pub baseline_generation: Generation,
    pub candidate_generation: Generation,
    pub evaluation_digest: Digest32,
    pub evaluation_eligible: bool,
    pub maximum_absolute_delta: FixedQ32,
    pub parameter_deltas: Vec<ParameterDelta>,
    pub topology_deltas: Vec<TopologyDelta>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    RequiresIndependentAcceptance,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlasticityProposal {
    pub proposal_id: StableId,
    pub proposer_id: StableId,
    pub evaluator_id: StableId,
    pub baseline_generation: Generation,
    pub candidate_generation: Generation,
    pub evaluation_digest: Digest32,
    pub parameter_deltas: Vec<ParameterDelta>,
    pub topology_deltas: Vec<TopologyDelta>,
    pub proposal_digest: Digest32,
    pub status: ProposalStatus,
    pub authority: AuthorityPosture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppendDisposition {
    Inserted,
    Unchanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    SelfEvaluation,
    EvaluationIneligible,
    GenerationNotAdvanced,
    EmptyDigest(&'static str),
    InvalidMaximumDelta,
    ParameterLimitExceeded,
    TopologyLimitExceeded,
    DuplicateParameter(String),
    DuplicateTopology(String),
    InvertedBounds(String),
    DeltaOutsideBounds(String),
    DeltaLimitExceeded(String),
    TopologyDigestUnchanged(String),
    RegistryCapacityExceeded,
    ProposalConflict(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl StdError for Error {}

pub fn propose(mut request: ProposalRequest) -> Result<PlasticityProposal, Error> {
    validate_header(&request)?;
    request
        .parameter_deltas
        .sort_by(|left, right| left.parameter_id.cmp(&right.parameter_id));
    request.topology_deltas.sort_by(|left, right| {
        left.module_id
            .cmp(&right.module_id)
            .then_with(|| left.operation.cmp(&right.operation))
    });
    validate_parameters(&request)?;
    validate_topology(&request)?;

    let proposal_digest = digest_proposal(&request);
    Ok(PlasticityProposal {
        proposal_id: request.proposal_id,
        proposer_id: request.proposer_id,
        evaluator_id: request.evaluator_id,
        baseline_generation: request.baseline_generation,
        candidate_generation: request.candidate_generation,
        evaluation_digest: request.evaluation_digest,
        parameter_deltas: request.parameter_deltas,
        topology_deltas: request.topology_deltas,
        proposal_digest,
        status: ProposalStatus::RequiresIndependentAcceptance,
        authority: AuthorityPosture::DENY_ALL,
    })
}

fn validate_header(request: &ProposalRequest) -> Result<(), Error> {
    if request.proposer_id == request.evaluator_id {
        return Err(Error::SelfEvaluation);
    }
    if !request.evaluation_eligible {
        return Err(Error::EvaluationIneligible);
    }
    if request.candidate_generation <= request.baseline_generation {
        return Err(Error::GenerationNotAdvanced);
    }
    if request.evaluation_digest.is_zero() {
        return Err(Error::EmptyDigest("evaluation"));
    }
    if request.maximum_absolute_delta <= FixedQ32::ZERO {
        return Err(Error::InvalidMaximumDelta);
    }
    if request.parameter_deltas.len() > MAX_PARAMETER_DELTAS {
        return Err(Error::ParameterLimitExceeded);
    }
    if request.topology_deltas.len() > MAX_TOPOLOGY_DELTAS {
        return Err(Error::TopologyLimitExceeded);
    }
    Ok(())
}

fn validate_parameters(request: &ProposalRequest) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for delta in &request.parameter_deltas {
        if !seen.insert(delta.parameter_id.clone()) {
            return Err(Error::DuplicateParameter(delta.parameter_id.to_string()));
        }
        if delta.evidence_digest.is_zero() {
            return Err(Error::EmptyDigest("parameter evidence"));
        }
        if delta.lower_bound > delta.upper_bound {
            return Err(Error::InvertedBounds(delta.parameter_id.to_string()));
        }
        if delta.delta < delta.lower_bound || delta.delta > delta.upper_bound {
            return Err(Error::DeltaOutsideBounds(delta.parameter_id.to_string()));
        }
        let Some(absolute) = delta.delta.raw().checked_abs() else {
            return Err(Error::DeltaLimitExceeded(delta.parameter_id.to_string()));
        };
        if absolute > request.maximum_absolute_delta.raw() {
            return Err(Error::DeltaLimitExceeded(delta.parameter_id.to_string()));
        }
    }
    Ok(())
}

fn validate_topology(request: &ProposalRequest) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for delta in &request.topology_deltas {
        let identity = (delta.module_id.clone(), delta.operation);
        if !seen.insert(identity) {
            return Err(Error::DuplicateTopology(delta.module_id.to_string()));
        }
        if delta.predecessor_digest.is_zero()
            || delta.candidate_digest.is_zero()
            || delta.evidence_digest.is_zero()
        {
            return Err(Error::EmptyDigest("topology lineage"));
        }
        if delta.predecessor_digest == delta.candidate_digest {
            return Err(Error::TopologyDigestUnchanged(delta.module_id.to_string()));
        }
    }
    Ok(())
}

fn digest_proposal(request: &ProposalRequest) -> Digest32 {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"hepta.plasticity.proposal.v1");
    push_id(&mut bytes, &request.proposal_id);
    push_id(&mut bytes, &request.proposer_id);
    push_id(&mut bytes, &request.evaluator_id);
    bytes.extend_from_slice(&request.baseline_generation.get().to_be_bytes());
    bytes.extend_from_slice(&request.candidate_generation.get().to_be_bytes());
    bytes.extend_from_slice(request.evaluation_digest.as_array());
    bytes.extend_from_slice(&request.maximum_absolute_delta.raw().to_be_bytes());
    for delta in &request.parameter_deltas {
        push_id(&mut bytes, &delta.parameter_id);
        bytes.extend_from_slice(&delta.delta.raw().to_be_bytes());
        bytes.extend_from_slice(&delta.lower_bound.raw().to_be_bytes());
        bytes.extend_from_slice(&delta.upper_bound.raw().to_be_bytes());
        bytes.extend_from_slice(delta.evidence_digest.as_array());
    }
    for delta in &request.topology_deltas {
        push_id(&mut bytes, &delta.module_id);
        bytes.push(topology_code(delta.operation));
        bytes.extend_from_slice(delta.predecessor_digest.as_array());
        bytes.extend_from_slice(delta.candidate_digest.as_array());
        bytes.extend_from_slice(delta.evidence_digest.as_array());
    }
    Digest32::of_bytes(&bytes)
}

fn topology_code(value: TopologyOperation) -> u8 {
    match value {
        TopologyOperation::Add => 0,
        TopologyOperation::Remove => 1,
        TopologyOperation::Replace => 2,
    }
}

fn push_id(bytes: &mut Vec<u8>, value: &StableId) {
    let raw = value.as_str().as_bytes();
    bytes.extend_from_slice(&u32::try_from(raw.len()).unwrap_or(u32::MAX).to_be_bytes());
    bytes.extend_from_slice(raw);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalRegistry {
    proposals: BTreeMap<StableId, PlasticityProposal>,
    maximum_records: usize,
}

impl ProposalRegistry {
    pub fn new(maximum_records: usize) -> Self {
        Self {
            proposals: BTreeMap::new(),
            maximum_records: maximum_records.min(MAX_PROPOSALS),
        }
    }

    pub fn append(
        &mut self,
        proposal: PlasticityProposal,
    ) -> Result<AppendDisposition, Error> {
        if let Some(existing) = self.proposals.get(&proposal.proposal_id) {
            if existing == &proposal {
                return Ok(AppendDisposition::Unchanged);
            }
            return Err(Error::ProposalConflict(proposal.proposal_id.to_string()));
        }
        if self.proposals.len() >= self.maximum_records {
            return Err(Error::RegistryCapacityExceeded);
        }
        self.proposals.insert(proposal.proposal_id.clone(), proposal);
        Ok(AppendDisposition::Inserted)
    }

    pub fn get(&self, proposal_id: &StableId) -> Option<&PlasticityProposal> {
        self.proposals.get(proposal_id)
    }
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
