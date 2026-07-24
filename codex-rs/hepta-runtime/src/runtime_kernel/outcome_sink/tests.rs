use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use hepta_contracts::OutcomeStatus;
use hepta_core::CorrelationId;
use hepta_core::PolicyEvaluationContext;
use hepta_core::SessionId;
use hepta_memory::DurableIntegrityKey;
use hepta_memory::DurableOutcomeStore;
use hepta_memory::DurableOutcomeWriterError;
use hepta_memory::ExecutionIntent;
use hepta_memory::ExecutionIntentStageResult;
use hepta_memory::InMemoryOutcomeStore;
use hepta_memory::OutcomeRecord;
use hepta_memory::OutcomeRecordResult;
use hepta_memory::OutcomeStoreError;
use hepta_memory::SyncDurableOutcomeWriter;

use super::ExactOutcomeRecord;
use super::OutcomeReceiptSink;
use super::OutcomeReceiptSinkError;
use crate::ExecutionBus;
use crate::OutcomeRecorder;
use crate::RuntimeKernel;
use crate::SafetyGateClient;
use crate::runtime_kernel::execution_attempt::AuthorizedToolExecution;

fn durable_integrity_key() -> DurableIntegrityKey {
    DurableIntegrityKey::from_bytes([0x77; 32])
}

fn wrong_durable_integrity_key() -> DurableIntegrityKey {
    DurableIntegrityKey::from_bytes([0x78; 32])
}

#[derive(Debug, Clone, Copy)]
enum FirstRecordBehavior {
    StageReadOnly,
    StageAmbiguousBeforeCommit,
    StageAmbiguousAfterCommit,
    CommitThenTimeout,
    AmbiguousBeforeCommit,
    AlreadyRecorded,
    Unavailable,
    QueueFullThenRecord,
    UnavailableThenRecord,
    QueueFullThenAlready,
    AmbiguousThenConflict,
}

struct ScriptedOutcomeSink {
    store: InMemoryOutcomeStore,
    behavior: FirstRecordBehavior,
    calls: AtomicUsize,
    staged_execution_intent: Mutex<Option<ExecutionIntent>>,
}

struct JournalingQueueFullSink {
    writer: SyncDurableOutcomeWriter,
}

#[derive(Default)]
struct CapturedExecutionMaterialSink {
    intent: Mutex<Option<ExecutionIntent>>,
    exact: Mutex<Option<ExactOutcomeRecord>>,
}

impl CapturedExecutionMaterialSink {
    fn take(&self) -> (ExecutionIntent, ExactOutcomeRecord) {
        let intent = self
            .intent
            .lock()
            .expect("captured intent mutex")
            .take()
            .expect("captured execution intent");
        let exact = self
            .exact
            .lock()
            .expect("captured exact mutex")
            .take()
            .expect("captured exact terminal material");
        (intent, exact)
    }
}

impl OutcomeReceiptSink for CapturedExecutionMaterialSink {
    fn record(
        &self,
        exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        *self.exact.lock().expect("captured exact mutex") = Some(exact.clone());
        Err(captured_material_error())
    }

    fn read_by_attempt(
        &self,
        _attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, OutcomeReceiptSinkError> {
        Ok(None)
    }

    fn stage_execution_intent(
        &self,
        intent: &ExecutionIntent,
    ) -> Result<ExecutionIntentStageResult, OutcomeReceiptSinkError> {
        *self.intent.lock().expect("captured intent mutex") = Some(intent.clone());
        Ok(ExecutionIntentStageResult::Staged)
    }

    fn record_and_resolve_execution(
        &self,
        exact: &ExactOutcomeRecord,
        _intent: &ExecutionIntent,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        self.record(exact)
    }

    fn pending_execution_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionIntent>, OutcomeReceiptSinkError> {
        Ok(self
            .intent
            .lock()
            .expect("captured intent mutex")
            .as_ref()
            .filter(|intent| intent.attempt_id() == attempt_id)
            .cloned())
    }

    fn pending_execution_intents(&self) -> Result<Vec<ExecutionIntent>, OutcomeReceiptSinkError> {
        Ok(self
            .intent
            .lock()
            .expect("captured intent mutex")
            .clone()
            .into_iter()
            .collect())
    }
}

fn captured_material_error() -> OutcomeReceiptSinkError {
    OutcomeReceiptSinkError::Durable(DurableOutcomeWriterError::Backend {
        source: OutcomeStoreError::Persistence {
            operation: "capture exact terminal material",
            detail: "injected capture boundary".into(),
        },
    })
}

impl OutcomeReceiptSink for JournalingQueueFullSink {
    fn record(
        &self,
        exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        self.writer
            .stage_intent(
                exact.attempt_id.clone(),
                exact.receipt.clone(),
                exact.canonical_evidence.clone(),
                exact.canonical_evidence_hash.clone(),
            )
            .map_err(OutcomeReceiptSinkError::Durable)?;
        Err(OutcomeReceiptSinkError::Durable(
            DurableOutcomeWriterError::QueueFull { capacity: 1 },
        ))
    }

    fn read_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, OutcomeReceiptSinkError> {
        self.writer
            .read_by_attempt(attempt_id)
            .map_err(OutcomeReceiptSinkError::Durable)
    }

    fn stage_execution_intent(
        &self,
        intent: &ExecutionIntent,
    ) -> Result<ExecutionIntentStageResult, OutcomeReceiptSinkError> {
        self.writer
            .stage_execution_intent(intent.clone())
            .map_err(OutcomeReceiptSinkError::Durable)
    }

    fn record_and_resolve_execution(
        &self,
        exact: &ExactOutcomeRecord,
        _intent: &ExecutionIntent,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        self.record(exact)
    }

    fn pending_execution_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionIntent>, OutcomeReceiptSinkError> {
        self.writer
            .pending_execution_intent(attempt_id)
            .map_err(OutcomeReceiptSinkError::Durable)
    }

    fn pending_execution_intents(&self) -> Result<Vec<ExecutionIntent>, OutcomeReceiptSinkError> {
        self.writer
            .pending_execution_intents()
            .map_err(OutcomeReceiptSinkError::Durable)
    }
}

impl ScriptedOutcomeSink {
    fn new(behavior: FirstRecordBehavior) -> Self {
        Self {
            store: InMemoryOutcomeStore::default(),
            behavior,
            calls: AtomicUsize::new(0),
            staged_execution_intent: Mutex::new(None),
        }
    }

    fn store_record(
        &self,
        exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        self.store
            .record(
                exact.attempt_id.clone(),
                exact.receipt.clone(),
                exact.canonical_evidence.clone(),
                exact.canonical_evidence_hash.clone(),
            )
            .map_err(OutcomeReceiptSinkError::InMemory)
    }
}

impl OutcomeReceiptSink for ScriptedOutcomeSink {
    fn record(
        &self,
        exact: &ExactOutcomeRecord,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        match (self.behavior, call) {
            (FirstRecordBehavior::CommitThenTimeout, 0) => {
                self.store_record(exact)?;
                Err(OutcomeReceiptSinkError::Durable(
                    DurableOutcomeWriterError::AcknowledgementTimeout {
                        attempt_id: exact.attempt_id.clone(),
                        timeout_ms: 1,
                    },
                ))
            }
            (FirstRecordBehavior::AmbiguousBeforeCommit, 0)
            | (FirstRecordBehavior::AmbiguousThenConflict, 0) => Err(
                OutcomeReceiptSinkError::Durable(DurableOutcomeWriterError::CommitAmbiguous {
                    attempt_id: exact.attempt_id.clone(),
                    detail: "injected lost post-commit acknowledgement".into(),
                }),
            ),
            (FirstRecordBehavior::AlreadyRecorded, 0) => Ok(OutcomeRecordResult::AlreadyRecorded),
            (
                FirstRecordBehavior::QueueFullThenRecord
                | FirstRecordBehavior::QueueFullThenAlready,
                0,
            ) => Err(OutcomeReceiptSinkError::Durable(
                DurableOutcomeWriterError::QueueFull { capacity: 1 },
            )),
            (FirstRecordBehavior::UnavailableThenRecord, 0) => Err(
                OutcomeReceiptSinkError::Durable(DurableOutcomeWriterError::WorkerUnavailable),
            ),
            (FirstRecordBehavior::QueueFullThenAlready, _) => {
                Ok(OutcomeRecordResult::AlreadyRecorded)
            }
            (FirstRecordBehavior::Unavailable, _) => Err(OutcomeReceiptSinkError::Durable(
                DurableOutcomeWriterError::WorkerUnavailable,
            )),
            (FirstRecordBehavior::AmbiguousThenConflict, _) => Err(
                OutcomeReceiptSinkError::Durable(DurableOutcomeWriterError::Backend {
                    source: OutcomeStoreError::Corrupt {
                        detail: "injected reconciliation conflict".into(),
                    },
                }),
            ),
            _ => self.store_record(exact),
        }
    }

    fn read_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, OutcomeReceiptSinkError> {
        self.store
            .read_by_attempt(attempt_id)
            .map_err(OutcomeReceiptSinkError::InMemory)
    }

    fn stage_execution_intent(
        &self,
        intent: &ExecutionIntent,
    ) -> Result<ExecutionIntentStageResult, OutcomeReceiptSinkError> {
        if matches!(self.behavior, FirstRecordBehavior::StageReadOnly) {
            return Err(OutcomeReceiptSinkError::Durable(
                DurableOutcomeWriterError::Backend {
                    source: OutcomeStoreError::Persistence {
                        operation: "commit pre-dispatch execution intent",
                        detail: "injected read-only database".into(),
                    },
                },
            ));
        }
        if matches!(
            self.behavior,
            FirstRecordBehavior::StageAmbiguousAfterCommit
        ) {
            *self
                .staged_execution_intent
                .lock()
                .expect("staged execution intent mutex") = Some(intent.clone());
        }
        if matches!(
            self.behavior,
            FirstRecordBehavior::StageAmbiguousBeforeCommit
                | FirstRecordBehavior::StageAmbiguousAfterCommit
        ) {
            return Err(OutcomeReceiptSinkError::Durable(
                DurableOutcomeWriterError::CommitAmbiguous {
                    attempt_id: intent.attempt_id().to_owned(),
                    detail: "injected execution-intent stage ambiguity".into(),
                },
            ));
        }
        Ok(ExecutionIntentStageResult::Staged)
    }

    fn record_and_resolve_execution(
        &self,
        exact: &ExactOutcomeRecord,
        _intent: &ExecutionIntent,
    ) -> Result<OutcomeRecordResult, OutcomeReceiptSinkError> {
        let result = self.record(exact);
        if result.is_ok() {
            *self
                .staged_execution_intent
                .lock()
                .expect("staged execution intent mutex") = None;
        }
        result
    }

    fn pending_execution_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionIntent>, OutcomeReceiptSinkError> {
        Ok(self
            .staged_execution_intent
            .lock()
            .expect("staged execution intent mutex")
            .clone()
            .filter(|intent| intent.attempt_id() == attempt_id))
    }

    fn pending_execution_intents(&self) -> Result<Vec<ExecutionIntent>, OutcomeReceiptSinkError> {
        Ok(self
            .staged_execution_intent
            .lock()
            .expect("staged execution intent mutex")
            .clone()
            .into_iter()
            .collect())
    }
}

fn runtime_with_scripted_sink(behavior: FirstRecordBehavior) -> RuntimeKernel {
    RuntimeKernel::new_with_outcome_sink(Arc::new(ScriptedOutcomeSink::new(behavior)))
}

fn canonical_text_field(record: &OutcomeRecord, field_name: &str) -> Option<String> {
    let envelope: serde_json::Value = serde_json::from_str(record.canonical_evidence()).ok()?;
    envelope
        .get("fields")?
        .as_array()?
        .iter()
        .find_map(|field| {
            let [name, value] = field.as_array()?.as_slice() else {
                return None;
            };
            (name.as_str()? == field_name)
                .then(|| value.as_str().map(str::to_owned))
                .flatten()
        })
}

fn authorize_echo(runtime: &RuntimeKernel, correlation: &str) -> (AuthorizedToolExecution, String) {
    let session_id = SessionId("session-main".into());
    let correlation_id = CorrelationId(correlation.into());
    let model = runtime.model_selection().expect("model").active;
    let decision = runtime
        .policy
        .evaluate_with_match(PolicyEvaluationContext {
            session_id: Some(session_id.clone()),
            model: Some(model.clone()),
            tool_name: "echo".into(),
            risk_tier: runtime.tools.risk_tier("echo").expect("risk tier"),
        })
        .expect("exact policy");
    let candidate = SafetyGateClient::prepare_candidate(
        runtime,
        &session_id.0,
        &model,
        "echo",
        r#"{"text":"durable outcome"}"#,
        &decision,
    )
    .expect("candidate");
    let epoch = runtime
        .capture_execution_epoch(&session_id.0)
        .expect("execution epoch");
    let lease = runtime
        .begin_execution_lease(epoch)
        .expect("execution lease")
        .bind_tool_resources(
            runtime,
            &session_id.0,
            "echo",
            &candidate.canonical_arguments,
        )
        .expect("resource-bound lease");
    let execution = SafetyGateClient::authorize_execution_without_grant(
        runtime,
        &session_id,
        &correlation_id,
        &candidate,
        &candidate,
        lease,
    )
    .expect("authorization");
    let attempt_id = execution.attempt_id().to_string();
    (execution, attempt_id)
}

async fn finalize_echo(runtime: &RuntimeKernel, correlation: &str) -> (String, bool) {
    let (execution, attempt_id) = authorize_echo(runtime, correlation);
    let mut captured = ExecutionBus::new(runtime).dispatch(execution).await;
    captured.capture_write_transaction();
    let finalized = OutcomeRecorder::new(runtime)
        .finalize_tool_dispatch(&mut captured)
        .is_ok();
    (attempt_id, finalized)
}

fn ambiguity_count(runtime: &RuntimeKernel) -> usize {
    runtime
        .execution_outcome_state
        .lock()
        .expect("outcome state")
        .breaker
        .ambiguity_count()
}

#[tokio::test]
async fn durable_runtime_normal_outcome_survives_reopen() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory.path().join("runtime-outcomes.sqlite3");
    let attempt_id = {
        let runtime =
            RuntimeKernel::bootstrap_with_durable_outcomes(&database_path, durable_integrity_key())
                .expect("durable runtime");
        let (attempt_id, finalized) = finalize_echo(&runtime, "durable-normal").await;
        assert!(finalized);
        attempt_id
    };

    let reopened =
        RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("recovered runtime");
    let record = reopened
        .outcome_record_by_attempt(&attempt_id)
        .expect("durable read")
        .expect("record");
    assert!(matches!(
        record.receipt().status(),
        OutcomeStatus::Succeeded
    ));
}

#[test]
fn durable_runtime_rejects_wrong_integrity_key_without_fallback() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory.path().join("runtime-wrong-key.sqlite3");
    drop(
        RuntimeKernel::bootstrap_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("durable runtime"),
    );

    let error = match RuntimeKernel::open_with_durable_outcomes(
        &database_path,
        wrong_durable_integrity_key(),
    ) {
        Ok(_) => panic!("wrong durable integrity key must not construct a runtime"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        DurableOutcomeWriterError::Backend {
            source: OutcomeStoreError::Corrupt { detail }
        } if detail.contains("integrity key or algorithm")
    ));
}

#[tokio::test]
async fn failed_pre_dispatch_stage_never_invokes_provider() {
    let runtime = runtime_with_scripted_sink(FirstRecordBehavior::StageReadOnly);
    let (execution, attempt_id) = authorize_echo(&runtime, "stage-read-only");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;

    assert_eq!(runtime.tools.provider_invocation_count("echo"), 0);
    assert!(captured.tool_result().is_none());
    assert!(
        captured
            .outward_error()
            .expect("stage failure")
            .0
            .contains("not durably staged")
    );
    captured.capture_write_transaction();
    OutcomeRecorder::new(&runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("blocked execution still has a terminal receipt");
    let record = runtime
        .outcome_record_by_attempt(&attempt_id)
        .expect("record-only outcome read")
        .expect("definite stage failure still records a terminal outcome");
    assert_eq!(
        canonical_text_field(&record, "execution.idempotency_key").as_deref(),
        Some("")
    );
}

#[tokio::test]
async fn ambiguous_stage_before_commit_records_without_resolving_and_holds_breaker() {
    let runtime = runtime_with_scripted_sink(FirstRecordBehavior::StageAmbiguousBeforeCommit);
    let (execution, attempt_id) = authorize_echo(&runtime, "stage-ambiguous-before");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;

    assert_eq!(runtime.tools.provider_invocation_count("echo"), 0);
    assert!(
        captured
            .outward_error()
            .expect("unconfirmed stage ambiguity")
            .0
            .contains("read-back did not confirm")
    );
    captured.capture_write_transaction();
    OutcomeRecorder::new(&runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("unconfirmed intent uses record-only terminal flow");
    let record = runtime
        .outcome_record_by_attempt(&attempt_id)
        .expect("record-only outcome read")
        .expect("blocked attempt terminal outcome");
    assert_eq!(
        canonical_text_field(&record, "execution.idempotency_key").as_deref(),
        Some("")
    );
    assert!(runtime.ensure_outcome_dispatch_open().is_err());
}

#[tokio::test]
async fn ambiguous_stage_after_commit_requires_exact_readback_before_provider() {
    let runtime = runtime_with_scripted_sink(FirstRecordBehavior::StageAmbiguousAfterCommit);
    let (execution, attempt_id) = authorize_echo(&runtime, "stage-ambiguous-after");
    let mut captured = ExecutionBus::new(&runtime).dispatch(execution).await;

    assert_eq!(runtime.tools.provider_invocation_count("echo"), 1);
    assert!(captured.outward_error().is_none());
    captured.capture_write_transaction();
    OutcomeRecorder::new(&runtime)
        .finalize_tool_dispatch(&mut captured)
        .expect("exact read-back permits atomic terminal resolution");
    let record = runtime
        .outcome_record_by_attempt(&attempt_id)
        .expect("resolved outcome read")
        .expect("provider terminal outcome");
    let idempotency_key = canonical_text_field(&record, "execution.idempotency_key")
        .expect("execution idempotency evidence");
    assert!(idempotency_key.starts_with(&format!("hepta-execution:{attempt_id}:sha256:")));
    assert!(runtime.ensure_outcome_dispatch_open().is_ok());
}

#[test]
fn unpolled_durable_dispatch_drop_never_stages_or_invokes() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory.path().join("runtime-unpolled-drop.sqlite3");
    let runtime =
        RuntimeKernel::bootstrap_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("durable runtime");
    let (execution, attempt_id) = authorize_echo(&runtime, "unpolled-drop");
    let bus = ExecutionBus::new(&runtime);
    drop(bus.dispatch(execution));

    assert_eq!(runtime.tools.provider_invocation_count("echo"), 0);
    assert!(
        runtime
            .pending_execution_intents()
            .expect("intent enumeration")
            .is_empty()
    );
    assert!(
        runtime
            .outcome_record_by_attempt(&attempt_id)
            .expect("drop outcome read")
            .is_some()
    );
}

#[tokio::test]
async fn post_provider_crash_leaves_restart_visible_in_doubt_intent() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory.path().join("runtime-post-provider-crash.sqlite3");
    let attempt_id = {
        let runtime =
            RuntimeKernel::bootstrap_with_durable_outcomes(&database_path, durable_integrity_key())
                .expect("durable runtime");
        let (execution, attempt_id) = authorize_echo(&runtime, "post-provider-crash");
        let captured = ExecutionBus::new(&runtime).dispatch(execution).await;
        assert_eq!(runtime.tools.provider_invocation_count("echo"), 1);
        assert_eq!(
            runtime
                .pending_execution_intents()
                .expect("staged intent")
                .len(),
            1
        );
        std::mem::forget(captured);
        attempt_id
    };

    let reopened =
        RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("restart recovery");
    let intents = reopened
        .pending_execution_intents()
        .expect("recovered execution intents");
    let [intent] = intents.as_slice() else {
        panic!("one post-provider execution intent must survive restart");
    };
    assert_eq!(intent.attempt_id(), attempt_id);
    assert!(intent.idempotency_key().starts_with("hepta-execution:"));
    assert!(reopened.ensure_outcome_dispatch_open().is_err());
    assert!(
        reopened
            .outcome_record_by_attempt(&attempt_id)
            .expect("outcome lookup")
            .is_none()
    );
}

#[tokio::test]
async fn restart_exactly_resolves_committed_outcome_without_outcome_intent() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory
        .path()
        .join("runtime-committed-before-execution-resolve.sqlite3");
    let capture = Arc::new(CapturedExecutionMaterialSink::default());
    let runtime = RuntimeKernel::new_with_outcome_sink(capture.clone());
    let (attempt_id, finalized) = finalize_echo(&runtime, "capture-terminal-material").await;
    assert!(!finalized);
    let (intent, exact) = capture.take();
    assert_eq!(intent.attempt_id(), attempt_id);

    {
        let store =
            DurableOutcomeStore::bootstrap_new_keyed(&database_path, durable_integrity_key())
                .await
                .expect("durable store");
        store
            .stage_execution_intent(intent)
            .await
            .expect("stage execution intent");
        store
            .record(
                exact.attempt_id.clone(),
                exact.receipt.clone(),
                exact.canonical_evidence.clone(),
                exact.canonical_evidence_hash.clone(),
            )
            .await
            .expect("commit terminal outcome without resolving execution intent");
        assert!(
            store
                .pending_intents()
                .await
                .expect("outcome intents")
                .is_empty()
        );
        assert_eq!(
            store
                .pending_execution_intents()
                .await
                .expect("execution intents")
                .len(),
            1
        );
    }

    let reopened =
        RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("restart recovery");
    assert!(reopened.ensure_outcome_dispatch_open().is_err());
    assert_eq!(
        reopened
            .reconcile_pending_outcome(&attempt_id)
            .expect("exact durable outcome must resolve execution intent"),
        OutcomeRecordResult::AlreadyRecorded
    );
    assert!(reopened.ensure_outcome_dispatch_open().is_ok());
    assert!(
        reopened
            .pending_execution_intents()
            .expect("resolved execution intents")
            .is_empty()
    );
}

#[test]
fn durable_runtime_authorized_drop_survives_reopen() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory.path().join("runtime-drop-outcomes.sqlite3");
    let attempt_id = {
        let runtime =
            RuntimeKernel::bootstrap_with_durable_outcomes(&database_path, durable_integrity_key())
                .expect("durable runtime");
        let (execution, attempt_id) = authorize_echo(&runtime, "durable-drop");
        drop(execution);
        attempt_id
    };

    let reopened =
        RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("recovered runtime");
    let record = reopened
        .outcome_record_by_attempt(&attempt_id)
        .expect("durable read")
        .expect("drop record");
    assert!(matches!(
        record.receipt().status(),
        OutcomeStatus::Cancelled { reason_code }
            if reason_code == "tool.dispatch_future_dropped"
    ));
}

#[tokio::test]
async fn durable_runtime_constructor_rejects_corrupt_recovery_without_fallback() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory.path().join("runtime-corrupt-outcomes.sqlite3");
    let attempt_id = {
        let runtime =
            RuntimeKernel::bootstrap_with_durable_outcomes(&database_path, durable_integrity_key())
                .expect("durable runtime");
        finalize_echo(&runtime, "durable-corrupt").await.0
    };
    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&database_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .expect("tamper connection");
    sqlx::query(
        "UPDATE hepta_v2_outcome_records
         SET storage_hash = 'sha256:tampered'
         WHERE attempt_id = ?",
    )
    .bind(attempt_id)
    .execute(&pool)
    .await
    .expect("tamper row");
    pool.close().await;

    let error =
        match RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key()) {
            Ok(_) => panic!("corrupt durable outcomes must not construct a runtime"),
            Err(error) => error,
        };
    assert!(matches!(
        error,
        DurableOutcomeWriterError::Backend {
            source: OutcomeStoreError::Corrupt { detail }
        } if detail.contains("keyed integrity tag")
    ));
}

#[tokio::test]
async fn durable_pending_intent_blocks_other_kernel_and_survives_restart() {
    let directory = tempfile::tempdir().expect("tempdir");
    let database_path = directory.path().join("runtime-pending-intent.sqlite3");
    let first = RuntimeKernel::new_with_outcome_sink(Arc::new(JournalingQueueFullSink {
        writer: SyncDurableOutcomeWriter::bootstrap_new_keyed(
            &database_path,
            durable_integrity_key(),
        )
        .expect("durable intent writer"),
    }));
    let (attempt_id, finalized) = finalize_echo(&first, "durable-pending-intent").await;
    assert!(!finalized);
    assert!(first.ensure_outcome_dispatch_open().is_err());

    let second = RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
        .expect("second durable kernel");
    let blocked = second
        .ensure_outcome_dispatch_open()
        .expect_err("durable pending intent must open the cross-kernel breaker");
    assert!(blocked.0.contains("execution attempt"));
    assert_eq!(
        second
            .reconcile_pending_outcome(&attempt_id)
            .expect("exact recovered intent must reconcile"),
        OutcomeRecordResult::Recorded
    );
    assert!(second.ensure_outcome_dispatch_open().is_ok());
    drop(first);
    drop(second);

    let reopened =
        RuntimeKernel::open_with_durable_outcomes(&database_path, durable_integrity_key())
            .expect("restart recovery");
    assert!(reopened.ensure_outcome_dispatch_open().is_ok());
    assert!(
        reopened
            .outcome_record_by_attempt(&attempt_id)
            .expect("durable record read")
            .is_some()
    );
}

#[tokio::test]
async fn exact_ambiguity_reconciliation_accepts_recorded_or_exact_replay_only() {
    for (behavior, expected) in [
        (
            FirstRecordBehavior::CommitThenTimeout,
            OutcomeRecordResult::AlreadyRecorded,
        ),
        (
            FirstRecordBehavior::AmbiguousBeforeCommit,
            OutcomeRecordResult::Recorded,
        ),
    ] {
        let runtime = runtime_with_scripted_sink(behavior);
        let (attempt_id, finalized) = finalize_echo(&runtime, "ambiguous-exact").await;
        assert!(!finalized);
        assert_eq!(ambiguity_count(&runtime), 1);
        assert!(runtime.ensure_outcome_dispatch_open().is_err());

        assert_eq!(
            runtime
                .reconcile_ambiguous_outcome(&attempt_id)
                .expect("exact replay must reconcile"),
            expected
        );
        assert_eq!(ambiguity_count(&runtime), 0);
        assert!(runtime.ensure_outcome_dispatch_open().is_ok());
        assert_eq!(runtime.outcome_record_count().expect("count"), 1);
        assert!(
            runtime
                .outcome_record_by_attempt(&attempt_id)
                .expect("read")
                .is_some()
        );
    }
}

#[tokio::test]
async fn exact_planned_already_recorded_is_terminal_success() {
    let runtime = runtime_with_scripted_sink(FirstRecordBehavior::AlreadyRecorded);
    let (_, finalized) = finalize_echo(&runtime, "exact-planned-duplicate").await;
    assert!(finalized);
    assert_eq!(ambiguity_count(&runtime), 0);
    assert!(runtime.ensure_outcome_dispatch_open().is_ok());
}

#[tokio::test]
async fn safe_preacceptance_failures_retain_exact_and_reconcile_to_recorded() {
    for behavior in [
        FirstRecordBehavior::QueueFullThenRecord,
        FirstRecordBehavior::UnavailableThenRecord,
    ] {
        let runtime = runtime_with_scripted_sink(behavior);
        let (attempt_id, finalized) = finalize_echo(&runtime, "safe-retry").await;
        assert!(!finalized);
        assert_eq!(ambiguity_count(&runtime), 1);
        assert!(runtime.ensure_outcome_dispatch_open().is_err());
        assert_eq!(
            runtime
                .reconcile_pending_outcome(&attempt_id)
                .expect("safe exact retry"),
            OutcomeRecordResult::Recorded
        );
        assert_eq!(ambiguity_count(&runtime), 0);
        assert!(runtime.ensure_outcome_dispatch_open().is_ok());
    }
}

#[tokio::test]
async fn safe_retry_exact_replay_accepts_another_kernel_commit() {
    let runtime = runtime_with_scripted_sink(FirstRecordBehavior::QueueFullThenAlready);
    let (attempt_id, finalized) = finalize_echo(&runtime, "safe-retry-duplicate").await;
    assert!(!finalized);
    assert_eq!(
        runtime
            .reconcile_pending_outcome(&attempt_id)
            .expect("exact replay may observe another kernel's commit"),
        OutcomeRecordResult::AlreadyRecorded
    );
    assert_eq!(ambiguity_count(&runtime), 0);
    assert!(runtime.ensure_outcome_dispatch_open().is_ok());
}

#[tokio::test]
async fn reconciliation_conflict_keeps_structured_breaker_open() {
    let runtime = runtime_with_scripted_sink(FirstRecordBehavior::AmbiguousThenConflict);
    let (attempt_id, finalized) = finalize_echo(&runtime, "ambiguous-conflict").await;
    assert!(!finalized);
    assert_eq!(ambiguity_count(&runtime), 1);
    assert!(
        runtime
            .reconcile_ambiguous_outcome(&attempt_id)
            .expect_err("conflict must fail closed")
            .0
            .contains("remains pending")
    );
    assert_eq!(ambiguity_count(&runtime), 1);
    assert!(runtime.ensure_outcome_dispatch_open().is_err());
}

#[tokio::test]
async fn durable_sink_unavailable_never_falls_back_to_memory() {
    let runtime = runtime_with_scripted_sink(FirstRecordBehavior::Unavailable);
    let (attempt_id, finalized) = finalize_echo(&runtime, "unavailable-no-fallback").await;
    assert!(!finalized);
    assert_eq!(ambiguity_count(&runtime), 1);
    assert!(
        runtime
            .outcome_record_by_attempt(&attempt_id)
            .expect("scripted store read")
            .is_none()
    );
    assert!(runtime.ensure_outcome_dispatch_open().is_err());
}

#[test]
fn durable_outcome_sink_adapters_require_explicit_open_mode() {
    let directory = tempfile::tempdir().expect("tempdir");
    let missing_parent = directory.path().join("missing-parent");
    let missing_path = missing_parent.join("runtime-outcomes.sqlite3");
    assert!(matches!(
        super::open_existing_durable_outcome_sink(&missing_path, durable_integrity_key()),
        Err(DurableOutcomeWriterError::Backend {
            source: OutcomeStoreError::Persistence {
                operation: "inspect durable database parent",
                ..
            }
        })
    ));
    assert!(!missing_parent.exists());

    let database_path = directory.path().join("runtime-outcomes.sqlite3");
    drop(
        super::bootstrap_new_durable_outcome_sink(&database_path, durable_integrity_key())
            .expect("new durable sink must bootstrap"),
    );
    assert!(matches!(
        super::bootstrap_new_durable_outcome_sink(&database_path, durable_integrity_key()),
        Err(DurableOutcomeWriterError::Backend {
            source: OutcomeStoreError::Persistence {
                operation: "reserve new database file",
                ..
            }
        })
    ));
    drop(
        super::open_existing_durable_outcome_sink(&database_path, durable_integrity_key())
            .expect("existing durable sink must recover"),
    );
}

#[test]
fn provider_seam_rejects_missing_or_attempt_drifted_idempotency_identity() {
    let attempt_id = uuid::Uuid::new_v4().to_string();
    let valid = format!("hepta-execution:{attempt_id}:sha256:{}", "a".repeat(64));
    assert!(
        crate::validate_provider_execution_identity(&attempt_id, &valid, &attempt_id, &valid)
            .is_ok()
    );

    let missing = crate::validate_provider_execution_identity(&attempt_id, &valid, &attempt_id, "")
        .expect_err("missing provider idempotency key must fail closed");
    assert!(missing.0.contains("differs"));

    let other_attempt = uuid::Uuid::new_v4().to_string();
    let drifted =
        crate::validate_provider_execution_identity(&attempt_id, &valid, &other_attempt, &valid)
            .expect_err("key from another attempt must fail closed");
    assert!(drifted.0.contains("differs"));

    let substituted = format!("hepta-execution:{attempt_id}:sha256:{}", "b".repeat(64));
    let substitution =
        crate::validate_provider_execution_identity(&attempt_id, &valid, &attempt_id, &substituted)
            .expect_err("same-attempt arbitrary digest must not satisfy exact staged identity");
    assert!(substitution.0.contains("differs"));
}
