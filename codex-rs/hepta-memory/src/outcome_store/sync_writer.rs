//! Synchronous client for the asynchronous durable outcome store.
//!
//! A dedicated OS thread exclusively owns both a current-thread Tokio runtime
//! and [`DurableOutcomeStore`]. Callers never run or block a Tokio runtime:
//! they enqueue one bounded command and wait for the post-commit acknowledgement.

mod error;
mod intent;

use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use hepta_contracts::ContentHash;
use hepta_contracts::OutcomeReceipt;

use super::DurableOutcomeStore;
use super::ExecutionIntent;
use super::OutcomeIntentStageResult;
use super::OutcomeRecord;
use super::OutcomeRecordResult;
use super::OutcomeStoreError;
use super::durable::OUTCOME_COMMIT_OPERATION;
use crate::durable::DurableDatabaseIdentity;
use crate::durable::DurableIntegrityContext;
use crate::durable::DurableIntegrityKey;

pub use error::DurableOutcomeWriterError;

const DEFAULT_QUEUE_CAPACITY: usize = 64;
const DEFAULT_ACK_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_STARTUP_TIMEOUT: Duration = Duration::from_secs(30);
const WORKER_JOIN_GRACE: Duration = Duration::from_millis(25);

/// Cloneable synchronous writer backed by a dedicated durable worker.
///
/// Construction does not return until the worker has bootstrapped or recovered
/// the SQLite database. Clones share one bounded queue and worker. The last
/// owner closes the command channel, requests an orderly shutdown without
/// blocking on a full queue, and joins an already-responsive worker. A worker
/// still finishing ambiguous work after the bounded grace period is safely
/// detached; the closed channel guarantees it cannot accept further commands.
#[derive(Clone)]
pub struct SyncDurableOutcomeWriter {
    inner: Arc<WriterInner>,
}

impl fmt::Debug for SyncDurableOutcomeWriter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SyncDurableOutcomeWriter")
            .field("path", &self.inner.path)
            .field("queue_capacity", &self.inner.queue_capacity)
            .field(
                "acknowledgement_timeout",
                &self.inner.acknowledgement_timeout,
            )
            .finish_non_exhaustive()
    }
}

struct WriterInner {
    commands: Option<mpsc::SyncSender<WorkerCommand>>,
    worker: Option<thread::JoinHandle<()>>,
    path: PathBuf,
    identity: DurableDatabaseIdentity,
    integrity: DurableIntegrityContext,
    queue_capacity: usize,
    acknowledgement_timeout: Duration,
    startup_timeout: Duration,
}

#[derive(Debug, Clone)]
enum WriterOpenMode {
    BootstrapNew,
    OpenExisting,
    OpenExistingBound(DurableDatabaseIdentity),
}

enum WorkerCommand {
    Record {
        attempt_id: String,
        receipt: Box<OutcomeReceipt>,
        canonical_evidence: String,
        canonical_evidence_hash: ContentHash,
        execution_idempotency_key: Option<String>,
        acknowledgement: mpsc::SyncSender<Result<OutcomeRecordResult, OutcomeStoreError>>,
    },
    ReadByAttempt {
        attempt_id: String,
        acknowledgement: mpsc::SyncSender<Result<Option<OutcomeRecord>, OutcomeStoreError>>,
    },
    Shutdown,
}

#[derive(Debug, Clone, Copy, Default)]
struct WorkerHooks {
    startup_delay: Duration,
    command_delay: Duration,
    exit_before_commands: bool,
    exit_after_record_before_ack: bool,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SyncDurableOutcomeWriterTestHooks {
    pub(crate) startup_delay: Duration,
    pub(crate) command_delay: Duration,
    pub(crate) exit_before_commands: bool,
    pub(crate) exit_after_record_before_ack: bool,
}

#[cfg(test)]
impl From<SyncDurableOutcomeWriterTestHooks> for WorkerHooks {
    fn from(value: SyncDurableOutcomeWriterTestHooks) -> Self {
        Self {
            startup_delay: value.startup_delay,
            command_delay: value.command_delay,
            exit_before_commands: value.exit_before_commands,
            exit_after_record_before_ack: value.exit_after_record_before_ack,
        }
    }
}

impl SyncDurableOutcomeWriter {
    /// Exclusively reserves, then bootstraps, a new durable writer.
    ///
    /// Existing paths are refused. Initialization failure leaves the reserved
    /// artifact in place so later attempts remain fail-closed.
    pub fn bootstrap_new(path: impl AsRef<Path>) -> Result<Self, DurableOutcomeWriterError> {
        Self::open_inner(
            path.as_ref().to_path_buf(),
            WriterOpenMode::BootstrapNew,
            DurableIntegrityContext::unkeyed(),
            DEFAULT_QUEUE_CAPACITY,
            DEFAULT_ACK_TIMEOUT,
            DEFAULT_STARTUP_TIMEOUT,
            WorkerHooks::default(),
        )
    }

    /// Bootstraps a writer whose canonical rows require an external key.
    pub fn bootstrap_new_keyed(
        path: impl AsRef<Path>,
        key: DurableIntegrityKey,
    ) -> Result<Self, DurableOutcomeWriterError> {
        Self::open_inner(
            path.as_ref().to_path_buf(),
            WriterOpenMode::BootstrapNew,
            key.into_context(),
            DEFAULT_QUEUE_CAPACITY,
            DEFAULT_ACK_TIMEOUT,
            DEFAULT_STARTUP_TIMEOUT,
            WorkerHooks::default(),
        )
    }

    /// Opens and fully recovers an existing durable writer without creating or
    /// migrating its database.
    pub fn open_existing(path: impl AsRef<Path>) -> Result<Self, DurableOutcomeWriterError> {
        Self::open_inner(
            path.as_ref().to_path_buf(),
            WriterOpenMode::OpenExisting,
            DurableIntegrityContext::unkeyed(),
            DEFAULT_QUEUE_CAPACITY,
            DEFAULT_ACK_TIMEOUT,
            DEFAULT_STARTUP_TIMEOUT,
            WorkerHooks::default(),
        )
    }

    /// Opens a keyed writer and rejects a wrong or missing integrity key.
    pub fn open_existing_keyed(
        path: impl AsRef<Path>,
        key: DurableIntegrityKey,
    ) -> Result<Self, DurableOutcomeWriterError> {
        Self::open_inner(
            path.as_ref().to_path_buf(),
            WriterOpenMode::OpenExisting,
            key.into_context(),
            DEFAULT_QUEUE_CAPACITY,
            DEFAULT_ACK_TIMEOUT,
            DEFAULT_STARTUP_TIMEOUT,
            WorkerHooks::default(),
        )
    }

    /// Recovers this writer's original database identity after a worker loss.
    ///
    /// A valid database substituted at the same path is rejected.
    pub fn reopen_existing_bound(&self) -> Result<Self, DurableOutcomeWriterError> {
        Self::open_inner(
            self.inner.path.clone(),
            WriterOpenMode::OpenExistingBound(self.inner.identity.clone()),
            self.inner.integrity.clone(),
            self.inner.queue_capacity,
            self.inner.acknowledgement_timeout,
            self.inner.startup_timeout,
            WorkerHooks::default(),
        )
    }

    #[cfg(test)]
    pub(crate) fn bootstrap_new_for_test(
        path: impl AsRef<Path>,
        queue_capacity: usize,
        acknowledgement_timeout: Duration,
        startup_timeout: Duration,
        hooks: SyncDurableOutcomeWriterTestHooks,
    ) -> Result<Self, DurableOutcomeWriterError> {
        Self::open_inner(
            path.as_ref().to_path_buf(),
            WriterOpenMode::BootstrapNew,
            DurableIntegrityContext::unkeyed(),
            queue_capacity,
            acknowledgement_timeout,
            startup_timeout,
            hooks.into(),
        )
    }

    fn open_inner(
        path: PathBuf,
        open_mode: WriterOpenMode,
        integrity: DurableIntegrityContext,
        queue_capacity: usize,
        acknowledgement_timeout: Duration,
        startup_timeout: Duration,
        hooks: WorkerHooks,
    ) -> Result<Self, DurableOutcomeWriterError> {
        if queue_capacity == 0 {
            return Err(DurableOutcomeWriterError::InvalidConfiguration {
                detail: "queue capacity must be greater than zero".into(),
            });
        }
        if acknowledgement_timeout.is_zero() {
            return Err(DurableOutcomeWriterError::InvalidConfiguration {
                detail: "acknowledgement timeout must be greater than zero".into(),
            });
        }
        if startup_timeout.is_zero() {
            return Err(DurableOutcomeWriterError::InvalidConfiguration {
                detail: "startup timeout must be greater than zero".into(),
            });
        }

        let (command_sender, command_receiver) = mpsc::sync_channel(queue_capacity);
        let (startup_sender, startup_receiver) = mpsc::sync_channel(1);
        let worker_path = path.clone();
        let worker_integrity = integrity.clone();
        let worker = thread::Builder::new()
            .name("hepta-durable-outcome-writer".into())
            .spawn(move || {
                run_worker(
                    worker_path,
                    open_mode,
                    worker_integrity,
                    command_receiver,
                    startup_sender,
                    hooks,
                );
            })
            .map_err(|error| DurableOutcomeWriterError::WorkerStartup {
                detail: error.to_string(),
            })?;

        match startup_receiver.recv_timeout(startup_timeout) {
            Ok(Ok(identity)) => Ok(Self {
                inner: Arc::new(WriterInner {
                    commands: Some(command_sender),
                    worker: Some(worker),
                    path,
                    identity,
                    integrity,
                    queue_capacity,
                    acknowledgement_timeout,
                    startup_timeout,
                }),
            }),
            Ok(Err(error)) => {
                drop(command_sender);
                finish_worker_bounded(worker);
                Err(error)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                drop(command_sender);
                finish_worker_bounded(worker);
                Err(DurableOutcomeWriterError::StartupTimeout {
                    timeout_ms: duration_millis(startup_timeout),
                })
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                drop(command_sender);
                finish_worker_bounded(worker);
                Err(DurableOutcomeWriterError::WorkerStartup {
                    detail: "startup acknowledgement channel closed".into(),
                })
            }
        }
    }

    /// Returns the database path owned by the worker.
    pub fn path(&self) -> &Path {
        &self.inner.path
    }

    /// Persists one terminal record and waits for the post-commit ACK.
    pub fn record(
        &self,
        attempt_id: impl Into<String>,
        receipt: OutcomeReceipt,
        canonical_evidence: impl Into<String>,
        canonical_evidence_hash: ContentHash,
    ) -> Result<OutcomeRecordResult, DurableOutcomeWriterError> {
        let attempt_id = attempt_id.into();
        let canonical_evidence = canonical_evidence.into();
        if matches!(
            self.stage_intent(
                attempt_id.clone(),
                receipt.clone(),
                canonical_evidence.clone(),
                canonical_evidence_hash.clone(),
            )?,
            OutcomeIntentStageResult::AlreadyRecorded
        ) {
            return Ok(OutcomeRecordResult::AlreadyRecorded);
        }
        self.commit_staged_record(
            attempt_id,
            receipt,
            canonical_evidence,
            canonical_evidence_hash,
            None,
        )
    }

    fn commit_staged_record(
        &self,
        attempt_id: String,
        receipt: OutcomeReceipt,
        canonical_evidence: String,
        canonical_evidence_hash: ContentHash,
        execution_idempotency_key: Option<String>,
    ) -> Result<OutcomeRecordResult, DurableOutcomeWriterError> {
        let (acknowledgement, receiver) = mpsc::sync_channel(1);
        let command = WorkerCommand::Record {
            attempt_id: attempt_id.clone(),
            receipt: Box::new(receipt),
            canonical_evidence,
            canonical_evidence_hash,
            execution_idempotency_key,
            acknowledgement,
        };
        let Some(commands) = self.inner.commands.as_ref() else {
            return Err(DurableOutcomeWriterError::WorkerUnavailable);
        };
        match commands.try_send(command) {
            Ok(()) => {}
            Err(mpsc::TrySendError::Full(_)) => {
                return Err(DurableOutcomeWriterError::QueueFull {
                    capacity: self.inner.queue_capacity,
                });
            }
            Err(mpsc::TrySendError::Disconnected(_)) => {
                return Err(DurableOutcomeWriterError::WorkerUnavailable);
            }
        }

        match receiver.recv_timeout(self.inner.acknowledgement_timeout) {
            Ok(result) => {
                let result = map_record_result(&attempt_id, result)?;
                self.acknowledge_intent(&attempt_id)?;
                Ok(result)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                Err(DurableOutcomeWriterError::AcknowledgementTimeout {
                    attempt_id,
                    timeout_ms: duration_millis(self.inner.acknowledgement_timeout),
                })
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err(DurableOutcomeWriterError::CommitAmbiguous {
                    attempt_id,
                    detail: "worker exited before post-commit acknowledgement".into(),
                })
            }
        }
    }

    /// Atomically persists one exact terminal record and resolves its execution plan.
    pub fn record_and_resolve_execution(
        &self,
        attempt_id: impl Into<String>,
        receipt: OutcomeReceipt,
        canonical_evidence: impl Into<String>,
        canonical_evidence_hash: ContentHash,
        execution_intent: &ExecutionIntent,
    ) -> Result<OutcomeRecordResult, DurableOutcomeWriterError> {
        let attempt_id = attempt_id.into();
        if execution_intent.attempt_id() != attempt_id {
            return Err(DurableOutcomeWriterError::InvalidConfiguration {
                detail: format!(
                    "terminal attempt {attempt_id} differs from execution intent {}",
                    execution_intent.attempt_id()
                ),
            });
        }
        let canonical_evidence = canonical_evidence.into();
        if matches!(
            self.stage_intent(
                attempt_id.clone(),
                receipt.clone(),
                canonical_evidence.clone(),
                canonical_evidence_hash.clone(),
            )?,
            OutcomeIntentStageResult::AlreadyRecorded
        ) {
            return self
                .resolve_execution_intent(&attempt_id, execution_intent.idempotency_key())
                .map(|_| OutcomeRecordResult::AlreadyRecorded);
        }
        self.commit_staged_record(
            attempt_id,
            receipt,
            canonical_evidence,
            canonical_evidence_hash,
            Some(execution_intent.idempotency_key().to_owned()),
        )
    }

    /// Reads one recovered record by execution-attempt identity.
    ///
    /// The lookup uses the same bounded worker queue and acknowledgement
    /// deadline as writes, so synchronous runtime callers never enter or block
    /// an ambient Tokio runtime.
    pub fn read_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, DurableOutcomeWriterError> {
        let attempt_id = attempt_id.to_owned();
        let (acknowledgement, receiver) = mpsc::sync_channel(1);
        let command = WorkerCommand::ReadByAttempt {
            attempt_id: attempt_id.clone(),
            acknowledgement,
        };
        let Some(commands) = self.inner.commands.as_ref() else {
            return Err(DurableOutcomeWriterError::WorkerUnavailable);
        };
        match commands.try_send(command) {
            Ok(()) => {}
            Err(mpsc::TrySendError::Full(_)) => {
                return Err(DurableOutcomeWriterError::QueueFull {
                    capacity: self.inner.queue_capacity,
                });
            }
            Err(mpsc::TrySendError::Disconnected(_)) => {
                return Err(DurableOutcomeWriterError::WorkerUnavailable);
            }
        }

        match receiver.recv_timeout(self.inner.acknowledgement_timeout) {
            Ok(result) => result.map_err(|source| DurableOutcomeWriterError::Backend { source }),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                Err(DurableOutcomeWriterError::ReadAcknowledgementTimeout {
                    attempt_id,
                    timeout_ms: duration_millis(self.inner.acknowledgement_timeout),
                })
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err(DurableOutcomeWriterError::WorkerUnavailable)
            }
        }
    }
}

impl Drop for WriterInner {
    fn drop(&mut self) {
        // Closing the last sender guarantees eventual worker exit even when a
        // full queue cannot accept the advisory shutdown command.
        if let Some(commands) = self.commands.take() {
            let _ = commands.try_send(WorkerCommand::Shutdown);
            drop(commands);
        }
        if let Some(worker) = self.worker.take() {
            finish_worker_bounded(worker);
        }
    }
}

fn run_worker(
    path: PathBuf,
    open_mode: WriterOpenMode,
    integrity: DurableIntegrityContext,
    commands: mpsc::Receiver<WorkerCommand>,
    startup: mpsc::SyncSender<Result<DurableDatabaseIdentity, DurableOutcomeWriterError>>,
    hooks: WorkerHooks,
) {
    if !hooks.startup_delay.is_zero() {
        thread::sleep(hooks.startup_delay);
    }
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            let _ = startup.send(Err(DurableOutcomeWriterError::WorkerStartup {
                detail: error.to_string(),
            }));
            return;
        }
    };
    let store = match open_mode {
        WriterOpenMode::BootstrapNew => runtime.block_on(
            DurableOutcomeStore::bootstrap_new_with_integrity(&path, integrity),
        ),
        WriterOpenMode::OpenExisting => runtime.block_on(
            DurableOutcomeStore::open_existing_with_integrity(&path, integrity),
        ),
        WriterOpenMode::OpenExistingBound(identity) => runtime.block_on(
            DurableOutcomeStore::open_existing_bound_with_integrity(&path, identity, integrity),
        ),
    };
    let store = match store {
        Ok(store) => store,
        Err(source) => {
            let _ = startup.send(Err(DurableOutcomeWriterError::Backend { source }));
            return;
        }
    };
    let identity = store.database_identity();
    if hooks.exit_before_commands {
        drop(commands);
        let _ = startup.send(Ok(identity));
        return;
    }
    if startup.send(Ok(identity)).is_err() {
        return;
    }

    while let Ok(command) = commands.recv() {
        match command {
            WorkerCommand::Record {
                attempt_id,
                receipt,
                canonical_evidence,
                canonical_evidence_hash,
                execution_idempotency_key,
                acknowledgement,
            } => {
                if !hooks.command_delay.is_zero() {
                    thread::sleep(hooks.command_delay);
                }
                let result = match execution_idempotency_key {
                    Some(idempotency_key) => {
                        runtime.block_on(store.commit_staged_intent_and_resolve_execution(
                            attempt_id,
                            *receipt,
                            canonical_evidence,
                            canonical_evidence_hash,
                            idempotency_key,
                        ))
                    }
                    None => runtime.block_on(store.commit_staged_intent(
                        attempt_id,
                        *receipt,
                        canonical_evidence,
                        canonical_evidence_hash,
                    )),
                };
                if hooks.exit_after_record_before_ack {
                    return;
                }
                let _ = acknowledgement.send(result);
            }
            WorkerCommand::ReadByAttempt {
                attempt_id,
                acknowledgement,
            } => {
                if !hooks.command_delay.is_zero() {
                    thread::sleep(hooks.command_delay);
                }
                let result = runtime.block_on(store.read_by_attempt(&attempt_id));
                let _ = acknowledgement.send(result);
            }
            WorkerCommand::Shutdown => return,
        }
    }
}

fn map_record_result(
    attempt_id: &str,
    result: Result<OutcomeRecordResult, OutcomeStoreError>,
) -> Result<OutcomeRecordResult, DurableOutcomeWriterError> {
    match result {
        Ok(result) => Ok(result),
        Err(OutcomeStoreError::Persistence {
            operation: OUTCOME_COMMIT_OPERATION,
            detail,
        }) => Err(DurableOutcomeWriterError::CommitAmbiguous {
            attempt_id: attempt_id.to_owned(),
            detail,
        }),
        Err(source) => Err(DurableOutcomeWriterError::PendingIntent {
            attempt_id: attempt_id.to_owned(),
            source,
        }),
    }
}

fn duration_millis(duration: Duration) -> u64 {
    duration.as_millis().min(u128::from(u64::MAX)) as u64
}

fn finish_worker_bounded(worker: thread::JoinHandle<()>) {
    let deadline = Instant::now() + WORKER_JOIN_GRACE;
    while !worker.is_finished() && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    if worker.is_finished() {
        let _ = worker.join();
    }
}
