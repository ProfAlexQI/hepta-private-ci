use hepta_contracts::Admission;
use hepta_contracts::AdmissionDecision;
use hepta_contracts::AdmissionId;
use hepta_contracts::Authorization;
use hepta_contracts::AuthorizationDecision;
use hepta_contracts::AuthorizationId;
use hepta_contracts::CandidateId;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::CapabilityId;
use hepta_contracts::CapabilityRequest;
use hepta_contracts::CapabilityRequestId;
use hepta_contracts::ContentHash;
use hepta_contracts::ContractError;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::JointCandidate;
use hepta_contracts::ObservationId;
use hepta_contracts::ObservationSnapshot;
use hepta_contracts::OutcomeReceipt;
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
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use crate::InMemoryPreferenceStore;
use crate::PreferenceCasError;
use crate::PreferenceCommitOutcome;
use crate::PreferenceDocumentCommitOutcome;
use crate::PreferenceGenesisOutcome;
use crate::PreferenceSeedOutcome;
use crate::PreferenceStateDocument;

type TestResult = Result<(), Box<dyn std::error::Error>>;
fn preference_state(revision: u64, hash: &str) -> PreferenceState {
    PreferenceState::new(Revision::new(revision), ContentHash::new(hash))
}

fn preference_document(
    revision: u64,
    hash: &str,
    reducer_version: &str,
    canonical_payload: &str,
) -> PreferenceStateDocument {
    PreferenceStateDocument::new(
        preference_state(revision, hash),
        reducer_version,
        canonical_payload,
    )
}

fn transition(
    id: &str,
    preference: PreferenceId,
    subject: PrincipalId,
    previous: PreferenceState,
    next: PreferenceState,
    receipt: &OutcomeReceipt,
) -> Result<PreferenceTransition, ContractError> {
    transition_with_evidence(
        id,
        &format!("evidence-{id}"),
        &format!("sha256:evidence-{id}"),
        preference,
        subject,
        previous,
        next,
        receipt,
    )
}

#[allow(clippy::too_many_arguments)]
fn transition_with_evidence(
    id: &str,
    evidence_id: &str,
    evidence_hash: &str,
    preference: PreferenceId,
    subject: PrincipalId,
    previous: PreferenceState,
    next: PreferenceState,
    receipt: &OutcomeReceipt,
) -> Result<PreferenceTransition, ContractError> {
    let evidence = preference_evidence(evidence_id, evidence_hash, preference, subject, receipt);
    PreferenceTransition::try_new(PreferenceTransitionId::new(id), &evidence, previous, next)
}

fn preference_evidence(
    id: &str,
    evidence_hash: &str,
    preference: PreferenceId,
    subject: PrincipalId,
    receipt: &OutcomeReceipt,
) -> PreferenceEvidenceRef {
    PreferenceEvidenceRef::new(
        PreferenceEvidenceId::new(id),
        ContentHash::new(evidence_hash),
        PreferenceEvidenceSignal::Accepted,
        receipt.reference(),
        ContentHash::new("sha256:session-binding"),
        subject,
        preference,
        ContentHash::new("sha256:target-binding"),
    )
}

fn outcome_receipt(id: &str, receipt_hash: &str) -> Result<OutcomeReceipt, ContractError> {
    let observation = ObservationSnapshot::new(
        ObservationId::new("observation-preference-cas"),
        Revision::new(1),
        ContentHash::new("sha256:observation"),
        PrincipalId::new("observer"),
        Vec::new(),
    );
    let context = FrozenTurnContext::new(
        observation.reference(),
        revision_stamp(1, "sha256:state"),
        revision_stamp(1, "sha256:policy"),
        revision_stamp(1, "sha256:catalog"),
        revision_stamp(1, "sha256:preference"),
    );
    let capability = CapabilityDescriptor::new(
        CapabilityId::new("capability-preference-cas"),
        Revision::new(1),
        ContentHash::new("sha256:capability"),
        context.capability_catalog().clone(),
        PrincipalId::new("executor"),
        "test.preference-cas",
    );
    let request = CapabilityRequest::try_new(
        CapabilityRequestId::new("request-preference-cas"),
        ContentHash::new("sha256:request"),
        capability.reference(),
        PrincipalId::new("planner"),
        context.clone(),
        ContentHash::new("sha256:payload"),
    )?;
    let candidate = JointCandidate::try_new(
        CandidateId::new("candidate-preference-cas"),
        Revision::new(1),
        ContentHash::new("sha256:candidate"),
        context.clone(),
        ContentHash::new("sha256:action"),
        ContentHash::new("sha256:metacontrol"),
        ContentHash::new("sha256:payload-set"),
        vec![PrincipalId::new("planner")],
        vec![request.reference()],
    )?;
    let admission = Admission::new(
        AdmissionId::new("admission-preference-cas"),
        Revision::new(1),
        ContentHash::new("sha256:admission"),
        &candidate,
        PrincipalId::new("safety-kernel"),
        AdmissionDecision::Admitted,
    );
    let authorization = Authorization::try_new_commit_time(
        AuthorizationId::new("authorization-preference-cas"),
        Revision::new(1),
        ContentHash::new("sha256:authorization"),
        &admission,
        context,
        PrincipalId::new("safety-kernel"),
        AuthorizationDecision::Authorized {
            scope_hash: ContentHash::new("sha256:scope"),
        },
    )?;

    OutcomeReceipt::try_new(
        ReceiptId::new(id),
        ContentHash::new(receipt_hash),
        &authorization,
        PrincipalId::new("executor"),
        ContentHash::new("sha256:outcome"),
        OutcomeStatus::Succeeded,
    )
}

fn revision_stamp(revision: u64, hash: &str) -> RevisionStamp {
    RevisionStamp::new(Revision::new(revision), ContentHash::new(hash))
}
