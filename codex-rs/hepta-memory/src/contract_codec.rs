//! Canonical serde representation for dependency-free V2 contracts.
//!
//! These wire types live in the persistence owner so `hepta-contracts` can
//! remain dependency-free. Fallible contract constructors are used again on
//! decode; serde success alone never makes a recovered value authoritative.

use hepta_contracts::AuthorizationId;
use hepta_contracts::AuthorizationRef;
use hepta_contracts::CandidateId;
use hepta_contracts::CandidateRef;
use hepta_contracts::ContentHash;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::ObservationId;
use hepta_contracts::ObservationRef;
use hepta_contracts::OutcomeReceipt;
use hepta_contracts::OutcomeReceiptParts;
use hepta_contracts::OutcomeStatus;
use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceEvidenceRef;
use hepta_contracts::PreferenceEvidenceSignal;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransition;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptId;
use hepta_contracts::ReceiptRef;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use serde::Deserialize;
use serde::Serialize;

use crate::durable::DurableStorageError;
use crate::preference_cas::PreferenceStateDocument;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RevisionStampWire {
    revision: u64,
    content_hash: String,
}

impl From<&RevisionStamp> for RevisionStampWire {
    fn from(value: &RevisionStamp) -> Self {
        Self {
            revision: value.revision().get(),
            content_hash: value.content_hash().as_str().to_owned(),
        }
    }
}

impl RevisionStampWire {
    fn into_contract(self) -> RevisionStamp {
        RevisionStamp::new(
            Revision::new(self.revision),
            ContentHash::new(self.content_hash),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservationRefWire {
    id: String,
    revision: u64,
    content_hash: String,
}

impl From<&ObservationRef> for ObservationRefWire {
    fn from(value: &ObservationRef) -> Self {
        Self {
            id: value.id().as_str().to_owned(),
            revision: value.revision().get(),
            content_hash: value.content_hash().as_str().to_owned(),
        }
    }
}

impl ObservationRefWire {
    fn into_contract(self) -> ObservationRef {
        ObservationRef::new(
            ObservationId::new(self.id),
            Revision::new(self.revision),
            ContentHash::new(self.content_hash),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FrozenTurnContextWire {
    observation: ObservationRefWire,
    state: RevisionStampWire,
    policy: RevisionStampWire,
    capability_catalog: RevisionStampWire,
    preference: RevisionStampWire,
}

impl From<&FrozenTurnContext> for FrozenTurnContextWire {
    fn from(value: &FrozenTurnContext) -> Self {
        Self {
            observation: value.observation().into(),
            state: value.state().into(),
            policy: value.policy().into(),
            capability_catalog: value.capability_catalog().into(),
            preference: value.preference().into(),
        }
    }
}

impl FrozenTurnContextWire {
    fn into_contract(self) -> FrozenTurnContext {
        FrozenTurnContext::new(
            self.observation.into_contract(),
            self.state.into_contract(),
            self.policy.into_contract(),
            self.capability_catalog.into_contract(),
            self.preference.into_contract(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CandidateRefWire {
    id: String,
    revision: u64,
    content_hash: String,
    context: FrozenTurnContextWire,
    action_hash: String,
    metacontrol_hash: String,
    payload_set_hash: String,
}

impl From<&CandidateRef> for CandidateRefWire {
    fn from(value: &CandidateRef) -> Self {
        Self {
            id: value.id().as_str().to_owned(),
            revision: value.revision().get(),
            content_hash: value.content_hash().as_str().to_owned(),
            context: value.context().into(),
            action_hash: value.action_hash().as_str().to_owned(),
            metacontrol_hash: value.metacontrol_hash().as_str().to_owned(),
            payload_set_hash: value.payload_set_hash().as_str().to_owned(),
        }
    }
}

impl CandidateRefWire {
    fn into_contract(self) -> CandidateRef {
        CandidateRef::new(
            CandidateId::new(self.id),
            Revision::new(self.revision),
            ContentHash::new(self.content_hash),
            self.context.into_contract(),
            ContentHash::new(self.action_hash),
            ContentHash::new(self.metacontrol_hash),
            ContentHash::new(self.payload_set_hash),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorizationRefWire {
    id: String,
    revision: u64,
    content_hash: String,
}

impl From<&AuthorizationRef> for AuthorizationRefWire {
    fn from(value: &AuthorizationRef) -> Self {
        Self {
            id: value.id().as_str().to_owned(),
            revision: value.revision().get(),
            content_hash: value.content_hash().as_str().to_owned(),
        }
    }
}

impl AuthorizationRefWire {
    fn into_contract(self) -> AuthorizationRef {
        AuthorizationRef::new(
            AuthorizationId::new(self.id),
            Revision::new(self.revision),
            ContentHash::new(self.content_hash),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
enum OutcomeStatusWire {
    Succeeded,
    Failed { error_code: String },
    Cancelled { reason_code: String },
}

impl TryFrom<&OutcomeStatus> for OutcomeStatusWire {
    type Error = DurableStorageError;

    fn try_from(value: &OutcomeStatus) -> Result<Self, Self::Error> {
        match value {
            OutcomeStatus::Succeeded => Ok(Self::Succeeded),
            OutcomeStatus::Failed { error_code } => Ok(Self::Failed {
                error_code: error_code.clone(),
            }),
            OutcomeStatus::Cancelled { reason_code } => Ok(Self::Cancelled {
                reason_code: reason_code.clone(),
            }),
            _ => Err(DurableStorageError::persistence(
                "encode outcome status",
                "variant is not supported by durable schema v1",
            )),
        }
    }
}

impl OutcomeStatusWire {
    fn into_contract(self) -> OutcomeStatus {
        match self {
            Self::Succeeded => OutcomeStatus::Succeeded,
            Self::Failed { error_code } => OutcomeStatus::Failed { error_code },
            Self::Cancelled { reason_code } => OutcomeStatus::Cancelled { reason_code },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OutcomeReceiptWire {
    id: String,
    receipt_hash: String,
    candidate: CandidateRefWire,
    authorization: AuthorizationRefWire,
    payload_set_hash: String,
    executed_by: String,
    outcome_hash: String,
    status: OutcomeStatusWire,
}

impl OutcomeReceiptWire {
    pub(crate) fn from_contract(value: &OutcomeReceipt) -> Result<Self, DurableStorageError> {
        Ok(Self {
            id: value.id().as_str().to_owned(),
            receipt_hash: value.receipt_hash().as_str().to_owned(),
            candidate: value.candidate().into(),
            authorization: value.authorization().into(),
            payload_set_hash: value.payload_set_hash().as_str().to_owned(),
            executed_by: value.executed_by().as_str().to_owned(),
            outcome_hash: value.outcome_hash().as_str().to_owned(),
            status: value.status().try_into()?,
        })
    }

    pub(crate) fn into_contract(self) -> Result<OutcomeReceipt, DurableStorageError> {
        OutcomeReceipt::try_rehydrate(OutcomeReceiptParts::new(
            ReceiptId::new(self.id),
            ContentHash::new(self.receipt_hash),
            self.candidate.into_contract(),
            self.authorization.into_contract(),
            ContentHash::new(self.payload_set_hash),
            PrincipalId::new(self.executed_by),
            ContentHash::new(self.outcome_hash),
            self.status.into_contract(),
        ))
        .map_err(|error| {
            DurableStorageError::corrupt(format!(
                "persisted outcome receipt violates contract invariants: {error}"
            ))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PreferenceStateWire {
    revision: u64,
    content_hash: String,
}

impl From<&PreferenceState> for PreferenceStateWire {
    fn from(value: &PreferenceState) -> Self {
        Self {
            revision: value.revision().get(),
            content_hash: value.content_hash().as_str().to_owned(),
        }
    }
}

impl PreferenceStateWire {
    fn into_contract(self) -> PreferenceState {
        PreferenceState::new(
            Revision::new(self.revision),
            ContentHash::new(self.content_hash),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PreferenceStateDocumentWire {
    state: PreferenceStateWire,
    reducer_version: String,
    canonical_payload: String,
}

impl From<&PreferenceStateDocument> for PreferenceStateDocumentWire {
    fn from(value: &PreferenceStateDocument) -> Self {
        Self {
            state: value.state().into(),
            reducer_version: value.reducer_version().to_owned(),
            canonical_payload: value.canonical_payload().to_owned(),
        }
    }
}

impl PreferenceStateDocumentWire {
    pub(crate) fn into_contract(self) -> PreferenceStateDocument {
        PreferenceStateDocument::new(
            self.state.into_contract(),
            self.reducer_version,
            self.canonical_payload,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PreferenceEvidenceSignalWire {
    Accepted,
    Rejected,
}

impl From<PreferenceEvidenceSignal> for PreferenceEvidenceSignalWire {
    fn from(value: PreferenceEvidenceSignal) -> Self {
        match value {
            PreferenceEvidenceSignal::Accepted => Self::Accepted,
            PreferenceEvidenceSignal::Rejected => Self::Rejected,
        }
    }
}

impl PreferenceEvidenceSignalWire {
    fn into_contract(self) -> PreferenceEvidenceSignal {
        match self {
            Self::Accepted => PreferenceEvidenceSignal::Accepted,
            Self::Rejected => PreferenceEvidenceSignal::Rejected,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReceiptRefWire {
    id: String,
    receipt_hash: String,
}

impl From<&ReceiptRef> for ReceiptRefWire {
    fn from(value: &ReceiptRef) -> Self {
        Self {
            id: value.id().as_str().to_owned(),
            receipt_hash: value.receipt_hash().as_str().to_owned(),
        }
    }
}

impl ReceiptRefWire {
    fn into_contract(self) -> ReceiptRef {
        ReceiptRef::new(ReceiptId::new(self.id), ContentHash::new(self.receipt_hash))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreferenceEvidenceRefWire {
    id: String,
    evidence_hash: String,
    signal: PreferenceEvidenceSignalWire,
    receipt: ReceiptRefWire,
    session_binding_hash: String,
    subject: String,
    preference: String,
    target_binding_hash: String,
}

impl From<&PreferenceEvidenceRef> for PreferenceEvidenceRefWire {
    fn from(value: &PreferenceEvidenceRef) -> Self {
        Self {
            id: value.id().as_str().to_owned(),
            evidence_hash: value.evidence_hash().as_str().to_owned(),
            signal: value.signal().into(),
            receipt: value.receipt().into(),
            session_binding_hash: value.session_binding_hash().as_str().to_owned(),
            subject: value.subject().as_str().to_owned(),
            preference: value.preference().as_str().to_owned(),
            target_binding_hash: value.target_binding_hash().as_str().to_owned(),
        }
    }
}

impl PreferenceEvidenceRefWire {
    fn into_contract(self) -> PreferenceEvidenceRef {
        PreferenceEvidenceRef::new(
            PreferenceEvidenceId::new(self.id),
            ContentHash::new(self.evidence_hash),
            self.signal.into_contract(),
            self.receipt.into_contract(),
            ContentHash::new(self.session_binding_hash),
            PrincipalId::new(self.subject),
            PreferenceId::new(self.preference),
            ContentHash::new(self.target_binding_hash),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PreferenceTransitionWire {
    id: String,
    evidence: PreferenceEvidenceRefWire,
    cas_expected_previous: PreferenceStateWire,
    committed_next: PreferenceStateWire,
}

impl From<&PreferenceTransition> for PreferenceTransitionWire {
    fn from(value: &PreferenceTransition) -> Self {
        Self {
            id: value.id().as_str().to_owned(),
            evidence: value.evidence().into(),
            cas_expected_previous: value.cas_expected_previous().into(),
            committed_next: value.committed_next().into(),
        }
    }
}

impl PreferenceTransitionWire {
    pub(crate) fn into_contract(self) -> Result<PreferenceTransition, DurableStorageError> {
        let evidence = self.evidence.into_contract();
        PreferenceTransition::try_new(
            PreferenceTransitionId::new(self.id),
            &evidence,
            self.cas_expected_previous.into_contract(),
            self.committed_next.into_contract(),
        )
        .map_err(|error| {
            DurableStorageError::corrupt(format!(
                "persisted preference transition violates contract invariants: {error}"
            ))
        })
    }
}
