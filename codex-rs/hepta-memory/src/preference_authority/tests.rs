use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use hepta_contracts::ContentHash;
use hepta_contracts::ContractError;
use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceEvidenceSignal;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptId;
use hepta_contracts::ReceiptRef;
use hepta_contracts::Revision;

use super::*;
use crate::PreferenceGenesisOutcome;

const REDUCER_ID: &str = "test.preference.reducer";
const REDUCER_VERSION: &str = "test.preference.reducer.v1";

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn private_tempdir() -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700))?;
    }
    Ok(directory)
}

#[test]
fn authenticated_request_advances_once_and_replay_fails_before_reauthentication() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let genesis = genesis_document();
    let request = request("once", genesis.state().clone());
    store.get_or_init_genesis(
        request.preference().clone(),
        request.subject().clone(),
        genesis.clone(),
    )?;
    let authenticator = AcceptingAuthenticator::new(source("primary"));
    let reducer = TestReducer::new(reducer_ref(REDUCER_VERSION), 1);

    let outcome =
        store.advance_preference_with_authority(request.clone(), &authenticator, &reducer)?;
    assert!(outcome.committed_now());
    assert_eq!(outcome.transition_id(), request.transition_id());
    assert_eq!(outcome.source(), &source("primary"));
    assert_eq!(outcome.reducer(), &reducer_ref(REDUCER_VERSION));
    assert_eq!(outcome.expected_previous(), genesis.state());
    assert_eq!(outcome.evidence().subject(), request.subject());
    assert_eq!(
        store
            .read_document(request.preference(), request.subject())?
            .expect("advanced document must exist")
            .state()
            .revision(),
        Revision::new(1)
    );

    assert!(matches!(
        store
            .advance_preference_with_authority(request, &authenticator, &reducer)
            .expect_err("an exact prior-state request must not advance twice"),
        PreferenceAuthorityError::Cas(PreferenceCasError::StateConflict { .. })
    ));
    assert_eq!(authenticator.calls.load(Ordering::SeqCst), 1);
    assert_eq!(reducer.calls.load(Ordering::SeqCst), 1);
    Ok(())
}

#[test]
fn authentication_denial_and_source_or_reducer_drift_do_not_mutate() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let genesis = genesis_document();
    let base = request("denied", genesis.state().clone());
    store.get_or_init_genesis(
        base.preference().clone(),
        base.subject().clone(),
        genesis.clone(),
    )?;
    let reducer = TestReducer::new(reducer_ref(REDUCER_VERSION), 1);
    let denying = DenyingAuthenticator {
        source: source("denied"),
    };
    assert_eq!(
        store
            .advance_preference_with_authority(base.clone(), &denying, &reducer)
            .expect_err("source denial must fail closed"),
        PreferenceAuthorityError::Authentication(PreferenceFeedbackAuthenticationError::new(
            "test.denied"
        ))
    );
    assert_eq!(reducer.calls.load(Ordering::SeqCst), 0);

    let drifting_source = DriftingAuthenticator {
        first: source("first"),
        second: source("second"),
        reads: AtomicUsize::new(0),
    };
    assert!(matches!(
        store
            .advance_preference_with_authority(base.clone(), &drifting_source, &reducer)
            .expect_err("source drift must fail closed"),
        PreferenceAuthorityError::SourceBindingChanged { .. }
    ));
    assert_eq!(reducer.calls.load(Ordering::SeqCst), 0);

    let accepting = AcceptingAuthenticator::new(source("stable"));
    let drifting_reducer = DriftingReducer {
        first: reducer_ref(REDUCER_VERSION),
        second: reducer_ref("test.preference.reducer.v2"),
        reads: AtomicUsize::new(0),
    };
    assert!(matches!(
        store
            .advance_preference_with_authority(base.clone(), &accepting, &drifting_reducer)
            .expect_err("reducer drift must fail closed"),
        PreferenceAuthorityError::ReducerBindingChanged { .. }
    ));
    assert_eq!(
        store.read_document(base.preference(), base.subject())?,
        Some(genesis)
    );
    Ok(())
}

#[test]
fn reducer_cannot_skip_a_revision() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let genesis = genesis_document();
    let request = request("skip", genesis.state().clone());
    store.get_or_init_genesis(
        request.preference().clone(),
        request.subject().clone(),
        genesis.clone(),
    )?;
    let authenticator = AcceptingAuthenticator::new(source("skip"));
    let reducer = TestReducer::new(reducer_ref(REDUCER_VERSION), 2);

    assert_eq!(
        store
            .advance_preference_with_authority(request.clone(), &authenticator, &reducer)
            .expect_err("a reducer must advance exactly one revision"),
        PreferenceAuthorityError::Contract(ContractError::PreferenceRevisionNotAdvanced {
            expected: Revision::new(0),
            committed: Revision::new(2),
        })
    );
    assert_eq!(
        store.read_document(request.preference(), request.subject())?,
        Some(genesis)
    );
    Ok(())
}

#[test]
fn evidence_digest_changes_for_every_authority_binding() -> TestResult {
    let base_parts = request_parts("binding", state(0, "sha256:previous"));
    let base_request = PreferenceFeedbackRequest::try_new(base_parts.clone())?;
    let base_source = source("binding");
    let base_reducer = reducer_ref(REDUCER_VERSION);
    let base =
        PreferenceFeedbackChallenge::new(base_request, base_source.clone(), base_reducer.clone());

    let mut changed = base_parts.clone();
    changed.subject = PrincipalId::new("subject:other");
    assert_hash_differs(&base, changed, base_source.clone(), base_reducer.clone())?;

    let mut changed = base_parts.clone();
    changed.target_binding_hash = ContentHash::new("sha256:target-other");
    assert_hash_differs(&base, changed, base_source.clone(), base_reducer.clone())?;

    let mut changed = base_parts.clone();
    changed.expected_previous = state(1, "sha256:previous");
    assert_hash_differs(&base, changed, base_source.clone(), base_reducer.clone())?;

    let mut changed = base_parts.clone();
    changed.signal = PreferenceEvidenceSignal::Rejected;
    assert_hash_differs(&base, changed, base_source.clone(), base_reducer.clone())?;

    let mut changed = base_parts.clone();
    changed.receipt = ReceiptRef::new(
        ReceiptId::new("receipt:other"),
        ContentHash::new("sha256:receipt-binding"),
    );
    assert_hash_differs(&base, changed, base_source.clone(), base_reducer.clone())?;

    let mut changed = base_parts;
    changed.session_binding_hash = ContentHash::new("sha256:session-other");
    assert_hash_differs(&base, changed, base_source.clone(), base_reducer.clone())?;

    let source_changed = PreferenceFeedbackChallenge::new(
        request("binding", state(0, "sha256:previous")),
        source("other"),
        base_reducer.clone(),
    );
    assert_ne!(base.evidence_hash(), source_changed.evidence_hash());
    let reducer_changed = PreferenceFeedbackChallenge::new(
        request("binding", state(0, "sha256:previous")),
        base_source,
        reducer_ref("test.preference.reducer.v2"),
    );
    assert_ne!(base.evidence_hash(), reducer_changed.evidence_hash());
    Ok(())
}

#[test]
fn competing_authority_requests_have_one_cas_winner() -> TestResult {
    let store = Arc::new(InMemoryPreferenceStore::default());
    let genesis = genesis_document();
    let first = request("race-a", genesis.state().clone());
    let second = request("race-b", genesis.state().clone());
    store.get_or_init_genesis(first.preference().clone(), first.subject().clone(), genesis)?;
    let authenticator = Arc::new(AcceptingAuthenticator::new(source("race")));
    let reducer = Arc::new(TestReducer::new(reducer_ref(REDUCER_VERSION), 1));
    let barrier = Arc::new(std::sync::Barrier::new(3));
    let handles = [first, second].map(|request| {
        let store = Arc::clone(&store);
        let authenticator = Arc::clone(&authenticator);
        let reducer = Arc::clone(&reducer);
        let barrier = Arc::clone(&barrier);
        std::thread::spawn(move || {
            barrier.wait();
            store.advance_preference_with_authority(request, &*authenticator, &*reducer)
        })
    });
    barrier.wait();
    let outcomes = handles.map(|handle| handle.join().expect("authority thread should join"));

    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome
                .as_ref()
                .is_ok_and(|outcome| outcome.committed_now()))
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(
                outcome,
                Err(PreferenceAuthorityError::Cas(
                    PreferenceCasError::StateConflict { .. }
                ))
            ))
            .count(),
        1
    );
    Ok(())
}

#[tokio::test]
async fn durable_authority_persists_the_single_advance() -> TestResult {
    let temporary = private_tempdir()?;
    let database_path = temporary.path().join("preference-authority.sqlite");
    let store = DurablePreferenceStore::bootstrap_new(&database_path).await?;
    let genesis = genesis_document();
    let request = request("durable", genesis.state().clone());
    assert_eq!(
        store
            .get_or_init_genesis(
                request.preference().clone(),
                request.subject().clone(),
                genesis,
            )
            .await?,
        PreferenceGenesisOutcome::Initialized
    );
    let outcome = store
        .advance_preference_with_authority(
            request.clone(),
            &AcceptingAuthenticator::new(source("durable")),
            &TestReducer::new(reducer_ref(REDUCER_VERSION), 1),
        )
        .await?;
    assert!(outcome.committed_now());
    drop(store);

    let reopened = DurablePreferenceStore::open_existing(&database_path).await?;
    assert_eq!(
        reopened
            .read_document(request.preference(), request.subject())
            .await?
            .expect("durable head must exist")
            .state()
            .revision(),
        Revision::new(1)
    );
    Ok(())
}

fn assert_hash_differs(
    base: &PreferenceFeedbackChallenge,
    changed: PreferenceFeedbackRequestParts,
    source: PreferenceFeedbackSourceRef,
    reducer: PreferenceReducerRef,
) -> TestResult {
    let changed = PreferenceFeedbackChallenge::new(
        PreferenceFeedbackRequest::try_new(changed)?,
        source,
        reducer,
    );
    assert_ne!(base.evidence_hash(), changed.evidence_hash());
    Ok(())
}

fn genesis_document() -> PreferenceStateDocument {
    PreferenceStateDocument::new(state(0, "sha256:genesis"), REDUCER_VERSION, "revision=0")
}

fn request(label: &str, expected_previous: PreferenceState) -> PreferenceFeedbackRequest {
    PreferenceFeedbackRequest::try_new(request_parts(label, expected_previous))
        .expect("test preference request must be valid")
}

fn request_parts(
    label: &str,
    expected_previous: PreferenceState,
) -> PreferenceFeedbackRequestParts {
    PreferenceFeedbackRequestParts {
        transition_id: PreferenceTransitionId::new(format!("transition:{label}")),
        evidence_id: PreferenceEvidenceId::new(format!("evidence:{label}")),
        signal: PreferenceEvidenceSignal::Accepted,
        receipt: ReceiptRef::new(
            ReceiptId::new(format!("receipt:{label}")),
            ContentHash::new(format!("sha256:receipt-{label}")),
        ),
        session_binding_hash: ContentHash::new(format!("sha256:session-{label}")),
        subject: PrincipalId::new("subject:alice"),
        preference: PreferenceId::new("preference:capability"),
        target_binding_hash: ContentHash::new("sha256:target"),
        expected_previous,
    }
}

fn source(label: &str) -> PreferenceFeedbackSourceRef {
    PreferenceFeedbackSourceRef::try_new(
        PrincipalId::new(format!("feedback-source:{label}")),
        Revision::new(7),
        ContentHash::new(format!("sha256:feedback-source-{label}")),
    )
    .expect("test feedback source must be valid")
}

fn reducer_ref(version: &str) -> PreferenceReducerRef {
    PreferenceReducerRef::try_new(REDUCER_ID, version).expect("test reducer binding must be valid")
}

fn state(revision: u64, hash: &str) -> PreferenceState {
    PreferenceState::new(Revision::new(revision), ContentHash::new(hash))
}

struct AcceptingAuthenticator {
    source: PreferenceFeedbackSourceRef,
    calls: AtomicUsize,
}

impl AcceptingAuthenticator {
    fn new(source: PreferenceFeedbackSourceRef) -> Self {
        Self {
            source,
            calls: AtomicUsize::new(0),
        }
    }
}

impl PreferenceFeedbackAuthenticator for AcceptingAuthenticator {
    fn source(&self) -> PreferenceFeedbackSourceRef {
        self.source.clone()
    }

    fn authenticate(
        &self,
        challenge: &PreferenceFeedbackChallenge,
    ) -> Result<(), PreferenceFeedbackAuthenticationError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if challenge.source() == &self.source {
            Ok(())
        } else {
            Err(PreferenceFeedbackAuthenticationError::new(
                "test.source_mismatch",
            ))
        }
    }
}

struct DenyingAuthenticator {
    source: PreferenceFeedbackSourceRef,
}

impl PreferenceFeedbackAuthenticator for DenyingAuthenticator {
    fn source(&self) -> PreferenceFeedbackSourceRef {
        self.source.clone()
    }

    fn authenticate(
        &self,
        _challenge: &PreferenceFeedbackChallenge,
    ) -> Result<(), PreferenceFeedbackAuthenticationError> {
        Err(PreferenceFeedbackAuthenticationError::new("test.denied"))
    }
}

struct DriftingAuthenticator {
    first: PreferenceFeedbackSourceRef,
    second: PreferenceFeedbackSourceRef,
    reads: AtomicUsize,
}

impl PreferenceFeedbackAuthenticator for DriftingAuthenticator {
    fn source(&self) -> PreferenceFeedbackSourceRef {
        if self.reads.fetch_add(1, Ordering::SeqCst) == 0 {
            self.first.clone()
        } else {
            self.second.clone()
        }
    }

    fn authenticate(
        &self,
        _challenge: &PreferenceFeedbackChallenge,
    ) -> Result<(), PreferenceFeedbackAuthenticationError> {
        Ok(())
    }
}

struct TestReducer {
    binding: PreferenceReducerRef,
    revision_step: u64,
    calls: AtomicUsize,
}

impl TestReducer {
    fn new(binding: PreferenceReducerRef, revision_step: u64) -> Self {
        Self {
            binding,
            revision_step,
            calls: AtomicUsize::new(0),
        }
    }
}

impl PreferenceDomainReducer for TestReducer {
    fn reducer(&self) -> PreferenceReducerRef {
        self.binding.clone()
    }

    fn reduce(
        &self,
        current: &PreferenceStateDocument,
        feedback: &AuthenticatedPreferenceFeedback,
    ) -> Result<PreferenceReductionDraft, PreferenceDomainReducerError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if current.state() != feedback.expected_previous() {
            return Err(PreferenceDomainReducerError::new("test.previous_mismatch"));
        }
        let next_revision = current.state().revision().get() + self.revision_step;
        Ok(PreferenceReductionDraft::new(
            state(next_revision, &format!("sha256:state-{next_revision}")),
            format!("revision={next_revision}"),
        ))
    }
}

struct DriftingReducer {
    first: PreferenceReducerRef,
    second: PreferenceReducerRef,
    reads: AtomicUsize,
}

impl PreferenceDomainReducer for DriftingReducer {
    fn reducer(&self) -> PreferenceReducerRef {
        if self.reads.fetch_add(1, Ordering::SeqCst) == 0 {
            self.first.clone()
        } else {
            self.second.clone()
        }
    }

    fn reduce(
        &self,
        current: &PreferenceStateDocument,
        _feedback: &AuthenticatedPreferenceFeedback,
    ) -> Result<PreferenceReductionDraft, PreferenceDomainReducerError> {
        Ok(PreferenceReductionDraft::new(
            state(1, "sha256:state-1"),
            format!("revision={}", current.state().revision().get() + 1),
        ))
    }
}
