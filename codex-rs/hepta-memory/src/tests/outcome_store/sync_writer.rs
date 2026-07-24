use std::time::Duration;

use super::*;

use crate::DurableIntegrityKey;
use crate::DurableOutcomeWriterError;
use crate::OutcomeIntentState;
use crate::SyncDurableOutcomeWriter;
use crate::outcome_store::SyncDurableOutcomeWriterTestHooks;

fn durable_integrity_key(byte: u8) -> DurableIntegrityKey {
    DurableIntegrityKey::from_bytes([byte; 32])
}

#[test]
fn synchronous_writer_recovers_and_preserves_exact_replay() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let receipt = outcome_receipt(
        "receipt-sync-replay",
        "sha256:receipt-sync-replay",
        "sha256:outcome-sync-replay",
    )?;
    let evidence = ContentHash::new("sha256:evidence-sync-replay");
    let envelope = r#"{"terminal":"succeeded","sync":true}"#;
    {
        let writer = SyncDurableOutcomeWriter::bootstrap_new(&database_path)?;
        assert_eq!(
            writer.record(
                "attempt-sync-replay",
                receipt.clone(),
                envelope,
                evidence.clone(),
            )?,
            OutcomeRecordResult::Recorded
        );
        let clone = writer.clone();
        drop(writer);
        assert_eq!(clone.path(), database_path);
    }

    let reopened = SyncDurableOutcomeWriter::open_existing(&database_path)?;
    assert_eq!(
        reopened.record("attempt-sync-replay", receipt, envelope, evidence)?,
        OutcomeRecordResult::AlreadyRecorded
    );
    Ok(())
}

#[test]
fn keyed_synchronous_writer_preserves_integrity_across_control_threads() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-keyed-writer.sqlite3");
    let receipt = outcome_receipt(
        "receipt-keyed-writer",
        "sha256:receipt-keyed-writer",
        "sha256:outcome-keyed-writer",
    )?;
    let evidence = ContentHash::new("sha256:evidence-keyed-writer");
    let envelope = r#"{"terminal":"succeeded","writer":"keyed"}"#;
    {
        let writer = SyncDurableOutcomeWriter::bootstrap_new_keyed(
            &database_path,
            durable_integrity_key(7),
        )?;
        assert_eq!(
            writer.record(
                "attempt-keyed-writer",
                receipt.clone(),
                envelope,
                evidence.clone(),
            )?,
            OutcomeRecordResult::Recorded
        );
    }

    let reopened =
        SyncDurableOutcomeWriter::open_existing_keyed(&database_path, durable_integrity_key(7))?;
    assert_eq!(
        reopened.record("attempt-keyed-writer", receipt, envelope, evidence)?,
        OutcomeRecordResult::AlreadyRecorded
    );
    drop(reopened);

    assert!(matches!(
        SyncDurableOutcomeWriter::open_existing_keyed(&database_path, durable_integrity_key(8)),
        Err(DurableOutcomeWriterError::Backend {
            source: OutcomeStoreError::Corrupt { detail },
        }) if detail.contains("integrity key or algorithm")
    ));
    Ok(())
}

#[test]
fn synchronous_writer_reads_recovered_record_by_attempt() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let receipt = outcome_receipt(
        "receipt-sync-read",
        "sha256:receipt-sync-read",
        "sha256:outcome-sync-read",
    )?;
    let evidence = ContentHash::new("sha256:evidence-sync-read");
    let envelope = r#"{"terminal":"succeeded","sync_read":true}"#;
    {
        let writer = SyncDurableOutcomeWriter::bootstrap_new(&database_path)?;
        assert_eq!(
            writer.record(
                "attempt-sync-read",
                receipt.clone(),
                envelope,
                evidence.clone(),
            )?,
            OutcomeRecordResult::Recorded
        );
    }

    let reopened = SyncDurableOutcomeWriter::open_existing(&database_path)?;
    let recovered = reopened
        .read_by_attempt("attempt-sync-read")?
        .expect("record must survive worker restart");
    assert_eq!(recovered.attempt_id(), "attempt-sync-read");
    assert_eq!(recovered.receipt(), &receipt);
    assert_eq!(recovered.canonical_evidence(), envelope);
    assert_eq!(recovered.canonical_evidence_hash(), &evidence);
    assert!(
        reopened
            .read_by_attempt("attempt-sync-read-missing")?
            .is_none()
    );
    Ok(())
}

#[test]
fn synchronous_writer_attempt_read_timeout_is_bounded_and_typed() -> TestResult {
    let directory = tempfile::tempdir()?;
    let writer = SyncDurableOutcomeWriter::bootstrap_new_for_test(
        directory.path().join("v2-memory.sqlite3"),
        1,
        Duration::from_millis(5),
        Duration::from_secs(1),
        SyncDurableOutcomeWriterTestHooks {
            command_delay: Duration::from_millis(100),
            ..Default::default()
        },
    )?;
    let started = std::time::Instant::now();
    assert_eq!(
        writer
            .read_by_attempt("attempt-sync-read-timeout")
            .expect_err("delayed attempt lookup must time out"),
        DurableOutcomeWriterError::ReadAcknowledgementTimeout {
            attempt_id: "attempt-sync-read-timeout".into(),
            timeout_ms: 5,
        }
    );
    assert!(started.elapsed() < Duration::from_millis(75));
    Ok(())
}

#[test]
fn synchronous_writer_reports_worker_unavailable_before_acceptance() -> TestResult {
    let directory = tempfile::tempdir()?;
    let writer = SyncDurableOutcomeWriter::bootstrap_new_for_test(
        directory.path().join("v2-memory.sqlite3"),
        1,
        Duration::from_secs(1),
        Duration::from_secs(1),
        SyncDurableOutcomeWriterTestHooks {
            exit_before_commands: true,
            ..Default::default()
        },
    )?;
    let receipt = outcome_receipt(
        "receipt-sync-worker-exit",
        "sha256:receipt-sync-worker-exit",
        "sha256:outcome-sync-worker-exit",
    )?;
    assert_eq!(
        writer
            .record(
                "attempt-sync-worker-exit",
                receipt.clone(),
                "{}",
                ContentHash::new("sha256:evidence-sync-worker-exit"),
            )
            .expect_err("exited worker must fail closed"),
        DurableOutcomeWriterError::WorkerUnavailable
    );
    let pending = writer
        .pending_intent("attempt-sync-worker-exit")?
        .expect("worker loss must retain exact intent");
    assert_eq!(pending.state(), OutcomeIntentState::Pending);
    assert_eq!(pending.record().receipt(), &receipt);
    Ok(())
}

#[test]
fn synchronous_writer_timeout_is_ambiguous_and_exactly_reconcilable() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let receipt = outcome_receipt(
        "receipt-sync-timeout",
        "sha256:receipt-sync-timeout",
        "sha256:outcome-sync-timeout",
    )?;
    let evidence = ContentHash::new("sha256:evidence-sync-timeout");
    let writer = SyncDurableOutcomeWriter::bootstrap_new_for_test(
        &database_path,
        1,
        Duration::from_millis(10),
        Duration::from_secs(1),
        SyncDurableOutcomeWriterTestHooks {
            command_delay: Duration::from_millis(75),
            ..Default::default()
        },
    )?;
    assert!(matches!(
        writer.record(
            "attempt-sync-timeout",
            receipt.clone(),
            "{}",
            evidence.clone(),
        ),
        Err(DurableOutcomeWriterError::AcknowledgementTimeout {
            attempt_id,
            timeout_ms: 10,
        }) if attempt_id == "attempt-sync-timeout"
    ));
    drop(writer);

    let reopened = SyncDurableOutcomeWriter::open_existing(&database_path)?;
    assert!(reopened.pending_intent("attempt-sync-timeout")?.is_some());
    assert!(matches!(
        reopened.record("attempt-sync-timeout", receipt, "{}", evidence)?,
        OutcomeRecordResult::Recorded | OutcomeRecordResult::AlreadyRecorded
    ));
    Ok(())
}

#[test]
fn synchronous_writer_exit_after_commit_is_typed_ambiguous() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let receipt = outcome_receipt(
        "receipt-sync-ambiguous",
        "sha256:receipt-sync-ambiguous",
        "sha256:outcome-sync-ambiguous",
    )?;
    let evidence = ContentHash::new("sha256:evidence-sync-ambiguous");
    let writer = SyncDurableOutcomeWriter::bootstrap_new_for_test(
        &database_path,
        1,
        Duration::from_secs(1),
        Duration::from_secs(1),
        SyncDurableOutcomeWriterTestHooks {
            exit_after_record_before_ack: true,
            ..Default::default()
        },
    )?;
    assert!(matches!(
        writer.record(
            "attempt-sync-ambiguous",
            receipt.clone(),
            "{}",
            evidence.clone(),
        ),
        Err(DurableOutcomeWriterError::CommitAmbiguous { attempt_id, .. })
            if attempt_id == "attempt-sync-ambiguous"
    ));
    drop(writer);

    let reopened = SyncDurableOutcomeWriter::open_existing(&database_path)?;
    assert_eq!(
        reopened
            .pending_intent("attempt-sync-ambiguous")?
            .expect("lost ACK must retain committed intent")
            .state(),
        OutcomeIntentState::Committed
    );
    assert_eq!(
        reopened.record("attempt-sync-ambiguous", receipt, "{}", evidence)?,
        OutcomeRecordResult::AlreadyRecorded
    );
    Ok(())
}

#[test]
fn synchronous_writer_bounded_queue_rejects_excess_work() -> TestResult {
    let directory = tempfile::tempdir()?;
    let writer = SyncDurableOutcomeWriter::bootstrap_new_for_test(
        directory.path().join("v2-memory.sqlite3"),
        1,
        Duration::from_secs(2),
        Duration::from_secs(1),
        SyncDurableOutcomeWriterTestHooks {
            command_delay: Duration::from_millis(100),
            ..Default::default()
        },
    )?;
    let first_writer = writer.clone();
    let first = std::thread::spawn(move || {
        first_writer.record(
            "attempt-sync-queue-a",
            outcome_receipt(
                "receipt-sync-queue-a",
                "sha256:receipt-sync-queue-a",
                "sha256:outcome-sync-queue-a",
            )
            .expect("fixture receipt must construct"),
            "{}",
            ContentHash::new("sha256:evidence-sync-queue-a"),
        )
    });
    std::thread::sleep(Duration::from_millis(20));
    let second_writer = writer.clone();
    let second = std::thread::spawn(move || {
        second_writer.record(
            "attempt-sync-queue-b",
            outcome_receipt(
                "receipt-sync-queue-b",
                "sha256:receipt-sync-queue-b",
                "sha256:outcome-sync-queue-b",
            )
            .expect("fixture receipt must construct"),
            "{}",
            ContentHash::new("sha256:evidence-sync-queue-b"),
        )
    });
    std::thread::sleep(Duration::from_millis(20));
    let third = outcome_receipt(
        "receipt-sync-queue-c",
        "sha256:receipt-sync-queue-c",
        "sha256:outcome-sync-queue-c",
    )?;
    assert_eq!(
        writer
            .record(
                "attempt-sync-queue-c",
                third,
                "{}",
                ContentHash::new("sha256:evidence-sync-queue-c"),
            )
            .expect_err("third command must not exceed bounded capacity"),
        DurableOutcomeWriterError::QueueFull { capacity: 1 }
    );
    let pending = writer
        .pending_intent("attempt-sync-queue-c")?
        .expect("queue rejection must retain exact intent");
    assert_eq!(pending.state(), OutcomeIntentState::Pending);
    assert!(first.join().expect("first writer thread must join").is_ok());
    assert!(
        second
            .join()
            .expect("second writer thread must join")
            .is_ok()
    );
    Ok(())
}

#[test]
fn synchronous_writer_startup_timeout_returns_without_waiting_for_worker() -> TestResult {
    let directory = tempfile::tempdir()?;
    let started = std::time::Instant::now();
    let error = SyncDurableOutcomeWriter::bootstrap_new_for_test(
        directory.path().join("v2-memory.sqlite3"),
        1,
        Duration::from_secs(1),
        Duration::from_millis(5),
        SyncDurableOutcomeWriterTestHooks {
            startup_delay: Duration::from_millis(150),
            ..Default::default()
        },
    )
    .expect_err("startup delay must exceed the bounded recovery deadline");
    assert_eq!(
        error,
        DurableOutcomeWriterError::StartupTimeout { timeout_ms: 5 }
    );
    assert!(started.elapsed() < Duration::from_millis(100));
    Ok(())
}

#[test]
fn only_last_clone_shutdown_is_bounded_when_worker_is_delayed() -> TestResult {
    let directory = tempfile::tempdir()?;
    let writer = SyncDurableOutcomeWriter::bootstrap_new_for_test(
        directory.path().join("v2-memory.sqlite3"),
        1,
        Duration::from_millis(5),
        Duration::from_secs(1),
        SyncDurableOutcomeWriterTestHooks {
            command_delay: Duration::from_millis(150),
            ..Default::default()
        },
    )?;
    let last_owner = writer.clone();
    drop(writer);
    let receipt = outcome_receipt(
        "receipt-sync-drop",
        "sha256:receipt-sync-drop",
        "sha256:outcome-sync-drop",
    )?;
    assert!(matches!(
        last_owner.record(
            "attempt-sync-drop",
            receipt,
            "{}",
            ContentHash::new("sha256:evidence-sync-drop"),
        ),
        Err(DurableOutcomeWriterError::AcknowledgementTimeout { .. })
    ));
    let started = std::time::Instant::now();
    drop(last_owner);
    assert!(started.elapsed() < Duration::from_millis(100));
    Ok(())
}

#[test]
fn synchronous_writer_open_modes_never_create_or_overwrite_implicitly() -> TestResult {
    let directory = tempfile::tempdir()?;
    let missing_parent = directory.path().join("missing-parent");
    let missing_path = missing_parent.join("v2-memory.sqlite3");
    assert!(matches!(
        SyncDurableOutcomeWriter::open_existing(&missing_path),
        Err(DurableOutcomeWriterError::Backend {
            source: OutcomeStoreError::Persistence {
                operation: "inspect durable database parent",
                ..
            }
        })
    ));
    assert!(!missing_parent.exists());

    let occupied_path = directory.path().join("occupied.sqlite3");
    std::fs::write(&occupied_path, b"sentinel")?;
    assert!(matches!(
        SyncDurableOutcomeWriter::bootstrap_new(&occupied_path),
        Err(DurableOutcomeWriterError::Backend {
            source: OutcomeStoreError::Persistence {
                operation: "reserve new database file",
                ..
            }
        })
    ));
    assert_eq!(std::fs::read(&occupied_path)?, b"sentinel");
    Ok(())
}

#[test]
fn synchronous_writer_fails_closed_after_database_replacement() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let displaced_path = directory.path().join("displaced.sqlite3");
    let writer = SyncDurableOutcomeWriter::bootstrap_new(&database_path)?;
    std::fs::rename(&database_path, &displaced_path)?;
    std::fs::write(&database_path, b"replacement")?;

    assert!(matches!(
        writer.read_by_attempt("attempt-after-replace"),
        Err(DurableOutcomeWriterError::Backend {
            source: OutcomeStoreError::Corrupt { detail }
        }) if detail.contains("deleted or replaced")
    ));
    Ok(())
}

#[test]
fn synchronous_writer_bound_reopen_rejects_valid_database_replacement() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let displaced_path = directory.path().join("displaced.sqlite3");
    let replacement_path = directory.path().join("replacement.sqlite3");
    let writer = SyncDurableOutcomeWriter::bootstrap_new(&database_path)?;
    drop(SyncDurableOutcomeWriter::bootstrap_new(&replacement_path)?);
    std::fs::rename(&database_path, &displaced_path)?;
    std::fs::rename(&replacement_path, &database_path)?;

    assert!(matches!(
        writer.reopen_existing_bound(),
        Err(DurableOutcomeWriterError::Backend {
            source: OutcomeStoreError::Corrupt { detail }
        }) if detail.contains("deleted or replaced")
    ));
    Ok(())
}
