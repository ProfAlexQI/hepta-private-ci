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
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptId;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;

use crate::InMemoryOutcomeStore;
use crate::OutcomeRecordResult;
use crate::OutcomeStoreError;

type TestResult = Result<(), Box<dyn std::error::Error>>;

mod tempfile {
    pub(super) fn tempdir() -> std::io::Result<::tempfile::TempDir> {
        let directory = ::tempfile::tempdir()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700))?;
        }
        Ok(directory)
    }
}

mod durable;
#[path = "effect_ack.rs"]
mod effect_ack;
mod execution_intent;
#[path = "outcome_pending_intent.rs"]
mod pending_intent;
mod sync_writer;

#[test]
fn records_and_reads_a_complete_outcome_receipt() -> TestResult {
    let store = InMemoryOutcomeStore::default();
    let receipt = outcome_receipt("receipt-1", "sha256:receipt-1", "sha256:outcome-1")?;
    let evidence_envelope = r#"{"attempt":"attempt-1","terminal":"succeeded"}"#;
    let evidence = ContentHash::new("sha256:evidence-1");

    let result = store.record(
        "attempt-1",
        receipt.clone(),
        evidence_envelope,
        evidence.clone(),
    )?;
    assert_eq!(result, OutcomeRecordResult::Recorded);
    assert!(result.recorded_now());

    let by_receipt = store
        .read_by_receipt(receipt.id())?
        .expect("record should be indexed by receipt");
    let by_attempt = store
        .read_by_attempt("attempt-1")?
        .expect("record should be indexed by attempt");
    assert_eq!(by_receipt, by_attempt);
    assert_eq!(by_receipt.attempt_id(), "attempt-1");
    assert_eq!(by_receipt.receipt(), &receipt);
    assert_eq!(by_receipt.canonical_evidence(), evidence_envelope);
    assert_eq!(by_receipt.canonical_evidence_hash(), &evidence);
    Ok(())
}

#[test]
fn exact_replay_is_idempotent() -> TestResult {
    let store = InMemoryOutcomeStore::default();
    let receipt = outcome_receipt("receipt-replay", "sha256:receipt", "sha256:outcome")?;
    let evidence = ContentHash::new("sha256:evidence");
    let envelope = r#"{"terminal":"succeeded"}"#;

    assert_eq!(
        store.record(
            "attempt-replay",
            receipt.clone(),
            envelope,
            evidence.clone()
        )?,
        OutcomeRecordResult::Recorded
    );
    let replay = store.record("attempt-replay", receipt, envelope, evidence)?;
    assert_eq!(replay, OutcomeRecordResult::AlreadyRecorded);
    assert!(!replay.recorded_now());
    Ok(())
}

#[test]
fn evidence_hash_drift_is_not_exact_replay() -> TestResult {
    let store = InMemoryOutcomeStore::default();
    let receipt = outcome_receipt("receipt-evidence", "sha256:receipt", "sha256:outcome")?;
    store.record(
        "attempt-evidence",
        receipt.clone(),
        r#"{"terminal":"succeeded"}"#,
        ContentHash::new("sha256:evidence-a"),
    )?;

    let error = store
        .record(
            "attempt-evidence",
            receipt,
            r#"{"terminal":"succeeded"}"#,
            ContentHash::new("sha256:evidence-b"),
        )
        .expect_err("changed hash for exact evidence must not be idempotent");
    assert_eq!(
        error,
        OutcomeStoreError::EvidenceHashConflict {
            receipt: ReceiptId::new("receipt-evidence"),
            existing: ContentHash::new("sha256:evidence-a"),
            attempted: ContentHash::new("sha256:evidence-b"),
        }
    );
    Ok(())
}

#[test]
fn evidence_envelope_drift_is_not_exact_replay() -> TestResult {
    let store = InMemoryOutcomeStore::default();
    let receipt = outcome_receipt("receipt-envelope", "sha256:receipt", "sha256:outcome")?;
    let evidence_hash = ContentHash::new("sha256:evidence");
    store.record(
        "attempt-envelope",
        receipt.clone(),
        r#"{"terminal":"succeeded"}"#,
        evidence_hash.clone(),
    )?;

    let error = store
        .record(
            "attempt-envelope",
            receipt,
            r#"{"terminal":"failed"}"#,
            evidence_hash.clone(),
        )
        .expect_err("changed envelope under one hash must not be idempotent");
    assert_eq!(
        error,
        OutcomeStoreError::EvidenceEnvelopeConflict {
            receipt: ReceiptId::new("receipt-envelope"),
            evidence_hash,
        }
    );
    Ok(())
}

#[test]
fn receipt_identity_cannot_change_hash_or_envelope() -> TestResult {
    let store = InMemoryOutcomeStore::default();
    let first = outcome_receipt("receipt-fixed", "sha256:receipt-a", "sha256:outcome-a")?;
    store.record(
        "attempt-fixed",
        first,
        r#"{"terminal":"succeeded"}"#,
        ContentHash::new("sha256:evidence"),
    )?;

    let changed_hash = outcome_receipt("receipt-fixed", "sha256:receipt-b", "sha256:outcome-a")?;
    assert!(matches!(
        store.record(
            "attempt-fixed",
            changed_hash,
            r#"{"terminal":"succeeded"}"#,
            ContentHash::new("sha256:evidence")
        ),
        Err(OutcomeStoreError::ReceiptHashConflict { receipt, .. })
            if receipt == ReceiptId::new("receipt-fixed")
    ));

    let changed_envelope =
        outcome_receipt("receipt-fixed", "sha256:receipt-a", "sha256:outcome-b")?;
    assert_eq!(
        store
            .record(
                "attempt-fixed",
                changed_envelope,
                r#"{"terminal":"succeeded"}"#,
                ContentHash::new("sha256:evidence")
            )
            .expect_err("same identity and hash cannot hide a changed envelope"),
        OutcomeStoreError::ReceiptEnvelopeConflict {
            receipt: ReceiptId::new("receipt-fixed"),
        }
    );
    Ok(())
}

#[test]
fn one_attempt_can_have_only_one_terminal_receipt() -> TestResult {
    let store = InMemoryOutcomeStore::default();
    let first = outcome_receipt("receipt-first", "sha256:receipt-first", "sha256:outcome")?;
    let second = outcome_receipt("receipt-second", "sha256:receipt-second", "sha256:outcome")?;
    store.record(
        "attempt-single",
        first,
        r#"{"terminal":"succeeded"}"#,
        ContentHash::new("sha256:evidence"),
    )?;

    let error = store
        .record(
            "attempt-single",
            second,
            r#"{"terminal":"failed"}"#,
            ContentHash::new("sha256:evidence"),
        )
        .expect_err("an attempt cannot gain a second terminal receipt");
    assert_eq!(
        error,
        OutcomeStoreError::AttemptAlreadyFinalized {
            attempt_id: "attempt-single".into(),
            existing_receipt: ReceiptId::new("receipt-first"),
            attempted_receipt: ReceiptId::new("receipt-second"),
        }
    );
    Ok(())
}

#[test]
fn one_receipt_cannot_be_reused_across_attempts() -> TestResult {
    let store = InMemoryOutcomeStore::default();
    let receipt = outcome_receipt("receipt-bound", "sha256:receipt", "sha256:outcome")?;
    let evidence = ContentHash::new("sha256:evidence");
    let envelope = r#"{"terminal":"succeeded"}"#;
    store.record("attempt-a", receipt.clone(), envelope, evidence.clone())?;

    let error = store
        .record("attempt-b", receipt, envelope, evidence)
        .expect_err("a receipt cannot cross attempts");
    assert_eq!(
        error,
        OutcomeStoreError::ReceiptAttemptConflict {
            receipt: ReceiptId::new("receipt-bound"),
            existing_attempt: "attempt-a".into(),
            attempted_attempt: "attempt-b".into(),
        }
    );
    Ok(())
}

#[test]
fn competing_terminal_records_are_atomic() -> TestResult {
    let store = InMemoryOutcomeStore::default();
    let first = outcome_receipt("receipt-race-a", "sha256:receipt-a", "sha256:outcome-a")?;
    let second = outcome_receipt("receipt-race-b", "sha256:receipt-b", "sha256:outcome-b")?;
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));

    let first_handle = {
        let store = store.clone();
        let barrier = barrier.clone();
        std::thread::spawn(move || {
            barrier.wait();
            store.record(
                "attempt-race",
                first,
                r#"{"terminal":"succeeded","winner":"a"}"#,
                ContentHash::new("sha256:evidence-a"),
            )
        })
    };
    let second_handle = {
        let store = store.clone();
        let barrier = barrier.clone();
        std::thread::spawn(move || {
            barrier.wait();
            store.record(
                "attempt-race",
                second,
                r#"{"terminal":"succeeded","winner":"b"}"#,
                ContentHash::new("sha256:evidence-b"),
            )
        })
    };
    barrier.wait();

    let outcomes = [
        first_handle.join().expect("first thread should join"),
        second_handle.join().expect("second thread should join"),
    ];
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Ok(OutcomeRecordResult::Recorded)))
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome,
                    Err(OutcomeStoreError::AttemptAlreadyFinalized { .. })
                )
            })
            .count(),
        1
    );
    Ok(())
}

fn outcome_receipt(
    receipt_id: &str,
    receipt_hash: &str,
    outcome_hash: &str,
) -> Result<OutcomeReceipt, ContractError> {
    let observation = ObservationSnapshot::new(
        ObservationId::new("observation-outcome-store"),
        Revision::new(1),
        ContentHash::new("sha256:observation"),
        PrincipalId::new("observer"),
        Vec::new(),
    );
    let context = FrozenTurnContext::new(
        observation.reference(),
        revision_stamp("state"),
        revision_stamp("policy"),
        revision_stamp("catalog"),
        revision_stamp("preference"),
    );
    let capability = CapabilityDescriptor::new(
        CapabilityId::new("capability-outcome-store"),
        Revision::new(1),
        ContentHash::new("sha256:capability"),
        context.capability_catalog().clone(),
        PrincipalId::new("executor"),
        "test.outcome-store",
    );
    let request = CapabilityRequest::try_new(
        CapabilityRequestId::new("request-outcome-store"),
        ContentHash::new("sha256:request"),
        capability.reference(),
        PrincipalId::new("planner"),
        context.clone(),
        ContentHash::new("sha256:payload"),
    )?;
    let candidate = JointCandidate::try_new(
        CandidateId::new("candidate-outcome-store"),
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
        AdmissionId::new("admission-outcome-store"),
        Revision::new(1),
        ContentHash::new("sha256:admission"),
        &candidate,
        PrincipalId::new("safety-kernel"),
        AdmissionDecision::Admitted,
    );
    let authorization = Authorization::try_new_commit_time(
        AuthorizationId::new("authorization-outcome-store"),
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
        ReceiptId::new(receipt_id),
        ContentHash::new(receipt_hash),
        &authorization,
        PrincipalId::new("executor"),
        ContentHash::new(outcome_hash),
        OutcomeStatus::Succeeded,
    )
}

fn revision_stamp(domain: &str) -> RevisionStamp {
    RevisionStamp::new(
        Revision::new(1),
        ContentHash::new(format!("sha256:{domain}")),
    )
}
