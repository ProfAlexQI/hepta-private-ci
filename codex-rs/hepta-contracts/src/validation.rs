use crate::Revision;
use std::error::Error;
use std::fmt;

/// Construction error for a Hepta cross-layer contract.
///
/// These errors only cover invariants that can be checked from the supplied
/// contract values. Domain validation, canonical hashing, policy evaluation,
/// and persistence remain responsibilities of their owning layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContractError {
    /// A requested capability manifest does not belong to the frozen catalog.
    CapabilityCatalogMismatch,
    /// A candidate request was produced from a different frozen turn context.
    CandidateRequestContextMismatch {
        /// Zero-based position of the mismatched request in candidate order.
        request_index: usize,
    },
    /// A candidate request refers to a manifest outside the frozen catalog.
    CandidateRequestCatalogMismatch {
        /// Zero-based position of the mismatched request in candidate order.
        request_index: usize,
    },
    /// A candidate omitted the accountable requester from its contributors.
    CandidateRequestRequesterMissing {
        /// Zero-based position of the mismatched request in candidate order.
        request_index: usize,
    },
    /// Commit-time authorization was attempted for a rejected admission.
    AdmissionNotAdmitted,
    /// An outcome receipt was attempted for a denied authorization.
    AuthorizationNotAuthorized,
    /// Rehydrated receipt parts disagree about the executed payload set.
    OutcomePayloadSetMismatch,
    /// A failed terminal outcome omitted its stable machine-readable code.
    OutcomeFailureCodeEmpty,
    /// A cancelled terminal outcome omitted its stable machine-readable code.
    OutcomeCancellationCodeEmpty,
    /// A committed preference state was not the exact CAS successor.
    PreferenceRevisionNotAdvanced {
        /// Revision supplied as the compare-and-swap expectation.
        expected: Revision,
        /// Revision claimed as the committed next state.
        committed: Revision,
    },
    /// The CAS expectation has no representable successor revision.
    PreferenceRevisionOverflow {
        /// Maximum revision supplied as the compare-and-swap expectation.
        expected: Revision,
    },
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapabilityCatalogMismatch => {
                formatter.write_str("capability manifest is outside the frozen catalog")
            }
            Self::CandidateRequestContextMismatch { request_index } => {
                write!(
                    formatter,
                    "candidate request at index {request_index} uses a different frozen context"
                )
            }
            Self::CandidateRequestCatalogMismatch { request_index } => {
                write!(
                    formatter,
                    "candidate request at index {request_index} is outside the frozen catalog"
                )
            }
            Self::CandidateRequestRequesterMissing { request_index } => {
                write!(
                    formatter,
                    "candidate request at index {request_index} omits its requester from contributors"
                )
            }
            Self::AdmissionNotAdmitted => {
                formatter.write_str("commit-time authorization requires an admitted candidate")
            }
            Self::AuthorizationNotAuthorized => {
                formatter.write_str("outcome receipt requires an authorized execution")
            }
            Self::OutcomePayloadSetMismatch => {
                formatter.write_str("outcome receipt payload set does not match its candidate")
            }
            Self::OutcomeFailureCodeEmpty => {
                formatter.write_str("failed outcome receipt requires a non-empty error code")
            }
            Self::OutcomeCancellationCodeEmpty => {
                formatter.write_str("cancelled outcome receipt requires a non-empty reason code")
            }
            Self::PreferenceRevisionNotAdvanced {
                expected,
                committed,
            } => write!(
                formatter,
                "committed preference revision {committed} must be exactly one greater than {expected}"
            ),
            Self::PreferenceRevisionOverflow { expected } => write!(
                formatter,
                "preference revision {expected} has no representable successor"
            ),
        }
    }
}

impl Error for ContractError {}
