//! Qualification-only host-bound turn writer.
//!
//! This is the narrow bridge between the host's explicit turn binding and the
//! Agent-local SQLite lease/event/outbox journal.  It is intentionally a
//! lifecycle contributor, but it never invents a binding: the host must attach
//! a [`QualificationTurnWriterInput`] to the turn store before the callback is
//! invoked.  Missing, malformed, stale, or cross-store inputs are ignored by
//! the callback (the extension API has no error channel) and are exposed as
//! errors by the input constructor/host helpers.
//!
//! The contributor is not installed by `install`.  A qualification embedding
//! may register it explicitly after it has supplied the host-bound input.  The
//! default and production extension profiles therefore cannot acquire a lease
//! or append an outbox row accidentally.

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;
use std::time::Duration;

use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionDataInit;
use codex_extension_api::ExtensionFuture;
use codex_extension_api::ExtensionRegistryBuilder;
use codex_extension_api::ThreadLifecycleContributor;
use codex_extension_api::ThreadStartInput;
use codex_extension_api::TurnAbortInput;
use codex_extension_api::TurnErrorInput;
use codex_extension_api::TurnLifecycleContributor;
use codex_extension_api::TurnStartInput;
use codex_extension_api::TurnStopInput;
use codex_hepta_memory::LocalAdmission;
use codex_hepta_memory::LocalCompactExecutor;
use codex_hepta_memory::LocalCompactExecutorError;
use codex_hepta_memory::LocalLeaseOutbox;
use codex_hepta_memory::LocalLeaseOutboxError;
use codex_hepta_memory::LocalReplayFinalization;
use codex_hepta_memory::LocalTurnLifecycleBinding;
use codex_hepta_memory::LocalTurnLifecycleBindingError;

/// Schema version for the qualification-only turn writer payload.
pub const QUALIFICATION_TURN_WRITER_SCHEMA_VERSION: u32 = 1;
/// This writer never dispatches its local outbox.
pub const QUALIFICATION_TURN_WRITER_EXTERNAL_EFFECTS: bool = false;
/// The writer records lifecycle metadata only; it does not mutate the KG.
pub const QUALIFICATION_TURN_WRITER_KG_WRITE_AUTHORITY: bool = false;
/// The contributor is opt-in and is not automatically registered.
pub const QUALIFICATION_TURN_WRITER_LIFECYCLE_REGISTERED: bool = false;
/// The writer is never a production caller.
pub const QUALIFICATION_TURN_WRITER_PRODUCTION_CALLER: bool = false;

const TURN_START_TOPIC: &str = "codex.turn.qualification.start.v1";
const IO_TIMEOUT: Duration = Duration::from_millis(750);

/// Errors found while constructing the immutable host attachment.
#[derive(Debug)]
pub enum QualificationTurnWriterInputError {
    Binding(LocalTurnLifecycleBindingError),
    Lease(LocalLeaseOutboxError),
    Executor(LocalCompactExecutorError),
    Invalid(String),
}

impl fmt::Display for QualificationTurnWriterInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binding(error) => write!(formatter, "turn binding rejected: {error}"),
            Self::Lease(error) => write!(formatter, "turn lease rejected: {error}"),
            Self::Executor(error) => write!(formatter, "turn executor rejected: {error}"),
            Self::Invalid(error) => write!(formatter, "invalid turn writer input: {error}"),
        }
    }
}

impl std::error::Error for QualificationTurnWriterInputError {}

impl From<LocalTurnLifecycleBindingError> for QualificationTurnWriterInputError {
    fn from(error: LocalTurnLifecycleBindingError) -> Self {
        Self::Binding(error)
    }
}

impl From<LocalLeaseOutboxError> for QualificationTurnWriterInputError {
    fn from(error: LocalLeaseOutboxError) -> Self {
        Self::Lease(error)
    }
}

impl From<LocalCompactExecutorError> for QualificationTurnWriterInputError {
    fn from(error: LocalCompactExecutorError) -> Self {
        Self::Executor(error)
    }
}

/// Future returned by an embedding-owned qualification writer factory.
pub type QualificationTurnWriterPrepareFuture = Pin<
    Box<
        dyn Future<Output = Result<QualificationTurnWriterInput, QualificationTurnWriterInputError>>
            + Send
            + 'static,
    >,
>;

type QualificationTurnWriterPrepareFn =
    dyn Fn(String) -> QualificationTurnWriterPrepareFuture + Send + Sync + 'static;

/// Explicit host capability for preparing one fully bound local turn input.
///
/// The host callback owns the authority contract.  It must return an input
/// containing a validated [`LocalTurnLifecycleBinding`] and exact lease and
/// compact-executor handles; this type never invents epochs, generations,
/// fencing tokens, leases, or expiry values.  A missing capability means the
/// qualification writer remains inert.
#[derive(Clone)]
pub struct QualificationTurnWriterHost {
    capability_id: Arc<str>,
    prepare: Arc<QualificationTurnWriterPrepareFn>,
}

impl fmt::Debug for QualificationTurnWriterHost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("QualificationTurnWriterHost")
            .field("capability_id", &self.capability_id)
            .finish_non_exhaustive()
    }
}

impl PartialEq for QualificationTurnWriterHost {
    fn eq(&self, other: &Self) -> bool {
        self.capability_id == other.capability_id && Arc::ptr_eq(&self.prepare, &other.prepare)
    }
}

impl Eq for QualificationTurnWriterHost {}

impl QualificationTurnWriterHost {
    /// Build a host capability from an embedding-owned asynchronous factory.
    ///
    /// `capability_id` is diagnostic provenance only; it does not grant
    /// authority and is intentionally not used to derive any fence value.
    pub fn from_fn<F, Fut>(capability_id: impl Into<String>, prepare: F) -> Self
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<QualificationTurnWriterInput, QualificationTurnWriterInputError>>
            + Send
            + 'static,
    {
        Self {
            capability_id: Arc::from(capability_id.into()),
            prepare: Arc::new(move |turn_id| Box::pin(prepare(turn_id))),
        }
    }

    pub fn capability_id(&self) -> &str {
        &self.capability_id
    }

    async fn prepare(
        &self,
        turn_id: &str,
    ) -> Result<QualificationTurnWriterInput, QualificationTurnWriterInputError> {
        let input = (self.prepare)(turn_id.to_owned()).await?;
        input.validate_for_turn(turn_id)?;
        Ok(input)
    }
}

/// Seed a host capability into a thread's extension initializer.
///
/// The operation is append-only for this capability type: an embedding cannot
/// replace a capability that another owner already supplied.
pub fn insert_qualification_turn_writer_host(
    init: &mut ExtensionDataInit,
    host: QualificationTurnWriterHost,
) -> bool {
    init.insert(host).is_none()
}

/// Immutable host-supplied input for one turn writer invocation.
///
/// The lease and compact executor are cloned handles to the same Agent-local
/// store.  [`LocalTurnLifecycleBinding`] is derived from those exact handles,
/// so an input cannot be constructed from guessed epochs or a legacy lease.
#[derive(Clone)]
pub struct QualificationTurnWriterInput {
    pub schema_version: u32,
    pub turn_id: String,
    pub binding: LocalTurnLifecycleBinding,
    pub occurrence_key: String,
    pub payload_json: String,
    pub lease: LocalLeaseOutbox,
    pub executor: LocalCompactExecutor,
}

impl QualificationTurnWriterInput {
    /// Build an input from exact host-owned handles and a binding made from
    /// those handles.  No database mutation occurs here.
    pub fn new(
        turn_id: impl Into<String>,
        binding: LocalTurnLifecycleBinding,
        lease: LocalLeaseOutbox,
        executor: LocalCompactExecutor,
        occurrence_key: impl Into<String>,
        payload_json: impl Into<String>,
    ) -> Result<Self, QualificationTurnWriterInputError> {
        let turn_id = turn_id.into();
        let occurrence_key = occurrence_key.into();
        let payload_json = payload_json.into();
        binding.validate()?;
        if binding.turn_id != turn_id {
            return Err(QualificationTurnWriterInputError::Invalid(
                "binding turn id does not match input turn id".to_string(),
            ));
        }
        if occurrence_key.trim().is_empty()
            || occurrence_key.len() > 512
            || occurrence_key.as_bytes().contains(&0)
        {
            return Err(QualificationTurnWriterInputError::Invalid(
                "occurrence key must contain 1..=512 non-NUL bytes".to_string(),
            ));
        }
        if payload_json.trim().is_empty()
            || payload_json.len() > 65_536
            || payload_json.as_bytes().contains(&0)
        {
            return Err(QualificationTurnWriterInputError::Invalid(
                "payload must contain 1..=65536 non-NUL bytes".to_string(),
            ));
        }
        Ok(Self {
            schema_version: QUALIFICATION_TURN_WRITER_SCHEMA_VERSION,
            turn_id,
            binding,
            occurrence_key,
            payload_json,
            lease,
            executor,
        })
    }

    fn validate_for_turn(&self, turn_id: &str) -> Result<(), QualificationTurnWriterInputError> {
        if self.schema_version != QUALIFICATION_TURN_WRITER_SCHEMA_VERSION {
            return Err(QualificationTurnWriterInputError::Invalid(
                "unsupported writer input schema".to_string(),
            ));
        }
        if self.turn_id != turn_id || self.binding.turn_id != turn_id {
            return Err(QualificationTurnWriterInputError::Invalid(
                "writer input is bound to a different turn".to_string(),
            ));
        }
        self.binding.validate()?;
        Ok(())
    }
}

/// Atomically attach an input to a turn store.
///
/// A second attachment is rejected rather than replaced.  This prevents a
/// late or untrusted host callback from swapping the lease/fence underneath a
/// running turn.
pub fn attach_qualification_turn_writer(
    turn_store: &ExtensionData,
    input: QualificationTurnWriterInput,
) -> bool {
    turn_store.insert_if(input, |current| current.is_none())
}

/// Register the qualification contributor on an embedding-owned registry.
///
/// This function is intentionally separate from [`super::install`].  An
/// embedding must choose the qualification profile explicitly and must still
/// attach a host-bound input for each turn; registration alone cannot create a
/// lease or grant authority.
pub fn install_qualification_turn_writer<C: Sync>(builder: &mut ExtensionRegistryBuilder<C>) {
    builder.turn_lifecycle_contributor(Arc::new(QualificationTurnLifecycleContributor::new()));
}

/// Register the qualification contributor with an explicit host capability.
///
/// Registration is still opt-in; the host is copied into each thread's
/// extension scope and is consulted only for turns whose embedding supplied
/// the capability.  No host means no automatic lease or outbox activity.
pub fn install_qualification_turn_writer_with_host<C: Sync>(
    builder: &mut ExtensionRegistryBuilder<C>,
    host: QualificationTurnWriterHost,
) {
    builder.turn_lifecycle_contributor(Arc::new(QualificationTurnLifecycleContributor::with_host(
        host,
    )));
}

#[derive(Default)]
struct TurnWriterState {
    attempted: bool,
    starting: bool,
    terminal_requested: Option<TerminalAction>,
    terminal_started: bool,
    active: Option<QualificationTurnWriterInput>,
}

#[derive(Clone, Debug)]
enum TerminalAction {
    Stop,
    Indeterminate(String),
}

/// Explicit qualification-only lifecycle contributor.
///
/// Hosts may either attach a [`QualificationTurnWriterInput`] before
/// `on_turn_start` or supply a [`QualificationTurnWriterHost`] capability.
/// The regular extension installer deliberately does not register it.
pub struct QualificationTurnLifecycleContributor {
    host: Option<QualificationTurnWriterHost>,
}

impl Default for QualificationTurnLifecycleContributor {
    fn default() -> Self {
        Self { host: None }
    }
}

impl QualificationTurnLifecycleContributor {
    pub const fn new() -> Self {
        Self { host: None }
    }

    pub fn with_host(host: QualificationTurnWriterHost) -> Self {
        Self { host: Some(host) }
    }

    fn state<'a>(&self, turn_store: &'a ExtensionData) -> std::sync::Arc<Mutex<TurnWriterState>> {
        turn_store.get_or_init(Mutex::default)
    }

    async fn admit(
        input: &QualificationTurnWriterInput,
    ) -> Result<bool, QualificationTurnWriterInputError> {
        input
            .binding
            .verify_current(&input.lease, &input.executor)
            .await?;
        let replay = input
            .lease
            .finalize_replayed_occurrence(input.occurrence_key.clone())
            .await?;
        match replay {
            LocalReplayFinalization::Released { .. } => Ok(false),
            LocalReplayFinalization::Queued(_) => Ok(true),
            LocalReplayFinalization::NotAdmitted => {
                match input
                    .lease
                    .admit(
                        input.occurrence_key.clone(),
                        TURN_START_TOPIC,
                        input.payload_json.clone(),
                    )
                    .await?
                {
                    LocalAdmission::Queued(_) | LocalAdmission::Replay(_) => Ok(true),
                }
            }
        }
    }

    async fn complete(
        input: &QualificationTurnWriterInput,
        action: TerminalAction,
    ) -> Result<(), QualificationTurnWriterInputError> {
        // Re-check the host-owned binding before any terminal write.  A stale
        // callback must not mark an occurrence indeterminate (or release a
        // lease) after ownership has moved to a newer epoch.
        input
            .binding
            .verify_current(&input.lease, &input.executor)
            .await?;
        if let TerminalAction::Indeterminate(reason) = action {
            input
                .lease
                .mark_indeterminate(input.occurrence_key.clone(), reason)
                .await?;
        }
        input.lease.release().await?;
        Ok(())
    }

    async fn start_one(&self, input: QualificationTurnWriterInput, turn_store: &ExtensionData) {
        let result = tokio::time::timeout(IO_TIMEOUT, Self::admit(&input)).await;
        let active = matches!(result, Ok(Ok(true)));
        let terminal = {
            let state = turn_store.get::<Mutex<TurnWriterState>>();
            let Some(state) = state else { return };
            let mut guard = state.lock().unwrap_or_else(PoisonError::into_inner);
            guard.starting = false;
            if active {
                guard.active = Some(input.clone());
            }
            guard
                .terminal_requested
                .take()
                .map(|action| (input, action))
        };
        if let Some((input, action)) = terminal {
            let completed = tokio::time::timeout(IO_TIMEOUT, Self::complete(&input, action))
                .await
                .is_ok_and(|result| result.is_ok());
            if let Some(state) = turn_store.get::<Mutex<TurnWriterState>>() {
                let mut guard = state.lock().unwrap_or_else(PoisonError::into_inner);
                guard.terminal_started = completed;
                if completed {
                    guard.active = None;
                } else if active {
                    guard.active = Some(input);
                }
            }
        }
    }

    async fn finish(&self, turn_store: &ExtensionData, action: TerminalAction) {
        let state = turn_store.get::<Mutex<TurnWriterState>>();
        let Some(state) = state else { return };
        let active = {
            let mut guard = state.lock().unwrap_or_else(PoisonError::into_inner);
            if guard.terminal_started {
                return;
            }
            if guard.starting {
                guard.terminal_requested.get_or_insert(action);
                return;
            }
            let Some(active) = guard.active.clone() else {
                return;
            };
            guard.terminal_started = true;
            active
        };
        let completed = tokio::time::timeout(IO_TIMEOUT, Self::complete(&active, action))
            .await
            .is_ok_and(|result| result.is_ok());
        let mut guard = state.lock().unwrap_or_else(PoisonError::into_inner);
        if completed {
            guard.active = None;
        } else {
            guard.terminal_started = false;
            guard.active = Some(active);
        }
    }
}

impl<C: Sync> ThreadLifecycleContributor<C> for QualificationTurnLifecycleContributor {
    fn on_thread_start<'a>(&'a self, input: ThreadStartInput<'a, C>) -> ExtensionFuture<'a, ()> {
        Box::pin(async move {
            let Some(host) = self.host.as_ref() else {
                return;
            };
            // A host may already have seeded this exact capability through
            // `StartThreadOptions.thread_extension_init`; never replace it.
            input
                .thread_store
                .insert_if(host.clone(), |current| current.is_none());
        })
    }
}

impl TurnLifecycleContributor for QualificationTurnLifecycleContributor {
    fn on_turn_start<'a>(&'a self, input: TurnStartInput<'a>) -> ExtensionFuture<'a, ()> {
        Box::pin(async move {
            let host_input = if let Some(host_input) =
                input.turn_store.get::<QualificationTurnWriterInput>()
            {
                host_input
            } else {
                // Prefer the host capability seeded into the exact thread
                // scope.  The contributor's captured host is a fallback for
                // resumed/forked threads whose initializer was reconstructed
                // by Core without the embedding's optional seed.
                let host = input
                    .thread_store
                    .get::<QualificationTurnWriterHost>()
                    .map(|host| (*host).clone())
                    .or_else(|| self.host.clone());
                let Some(host) = host else {
                    return;
                };
                let prepared = tokio::time::timeout(IO_TIMEOUT, host.prepare(input.turn_id)).await;
                let Ok(Ok(prepared)) = prepared else {
                    return;
                };
                if !attach_qualification_turn_writer(input.turn_store, prepared) {
                    return;
                }
                let Some(host_input) = input.turn_store.get::<QualificationTurnWriterInput>()
                else {
                    return;
                };
                host_input
            };
            if host_input.validate_for_turn(input.turn_id).is_err()
                || input.turn_store.level_id() != input.turn_id
            {
                return;
            }
            let state = self.state(input.turn_store);
            {
                let mut guard = state.lock().unwrap_or_else(PoisonError::into_inner);
                if guard.attempted {
                    return;
                }
                guard.attempted = true;
                guard.starting = true;
            }
            self.start_one((*host_input).clone(), input.turn_store)
                .await;
        })
    }

    fn on_turn_stop<'a>(&'a self, input: TurnStopInput<'a>) -> ExtensionFuture<'a, ()> {
        Box::pin(async move { self.finish(input.turn_store, TerminalAction::Stop).await })
    }

    fn on_turn_abort<'a>(&'a self, input: TurnAbortInput<'a>) -> ExtensionFuture<'a, ()> {
        Box::pin(async move {
            self.finish(
                input.turn_store,
                TerminalAction::Indeterminate(format!("turn_aborted:{:?}", input.reason)),
            )
            .await;
        })
    }

    fn on_turn_error<'a>(&'a self, input: TurnErrorInput<'a>) -> ExtensionFuture<'a, ()> {
        Box::pin(async move {
            if input.turn_id != input.turn_store.level_id() {
                return;
            }
            self.finish(
                input.turn_store,
                TerminalAction::Indeterminate(format!("turn_error:{:?}", input.error)),
            )
            .await;
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use codex_extension_api::ExtensionData;
    use codex_extension_api::ThreadStartInput;
    use codex_extension_api::TurnStartInput;
    use codex_extension_api::TurnStopInput;
    use codex_hepta_contracts::AgentId;
    use codex_hepta_memory::CognitiveAccess;
    use codex_hepta_memory::CognitiveScope;
    use codex_hepta_memory::CognitiveStore;
    use codex_hepta_memory::CompactFence;
    use codex_hepta_memory::KgFactSetDraft;
    use codex_hepta_memory::LedgerSourceKind;
    use codex_hepta_memory::LocalLeaseAcquire;
    use codex_hepta_memory::LocalLeaseState;
    use codex_hepta_memory::MemoryAdmissionEvidence;
    use codex_hepta_memory::MemoryCandidateDraft;
    use codex_hepta_memory::MemoryCandidateOrigin;
    use codex_hepta_memory::MemoryCandidateState;
    use codex_hepta_memory::MemoryLifecycleState;
    use codex_hepta_memory::RetrievalRequest;
    use codex_hepta_memory::SourceDraft;
    use codex_hepta_paths::HeptaFleetRoot;
    use codex_protocol::config_types::{CollaborationMode, ModeKind, Settings};
    use codex_protocol::protocol::SessionSource;
    use codex_protocol::protocol::TokenUsage;
    use tempfile::TempDir;

    use super::*;

    const TURN_ID: &str = "turn:writer-e26";
    const LEASE_ID: &str = "lease:writer-e26";
    const OCCURRENCE: &str = "occurrence:writer-e26";

    fn mode() -> CollaborationMode {
        CollaborationMode {
            mode: ModeKind::Default,
            settings: Settings {
                model: "test-model".to_string(),
                reasoning_effort: None,
                developer_instructions: None,
            },
        }
    }

    async fn prepared() -> (TempDir, CognitiveStore, QualificationTurnWriterInput) {
        let temp = TempDir::new().expect("temp");
        let fleet_root = temp.path().join("fleet");
        fs::create_dir_all(&fleet_root).expect("fleet root");
        let fleet = HeptaFleetRoot::parse(fleet_root).expect("fleet");
        let owner = AgentId::parse("00000000-0000-4000-8000-000000000981").expect("owner");
        let store = CognitiveStore::open(&fleet.layout().agent(&owner))
            .await
            .expect("store");
        let fence = CompactFence::new(17, 19, 1, "writer-e26-fence").expect("fence");
        let expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_secs()
            + 3_600;
        let lease = match store
            .acquire_local_lease_bound(
                LEASE_ID,
                fence.authority_epoch,
                fence.owner_epoch,
                fence.generation,
                fence.fencing_token.clone(),
                expires,
            )
            .await
            .expect("bound lease")
        {
            LocalLeaseAcquire::Acquired(lease) | LocalLeaseAcquire::Replay(lease) => lease,
        };
        let executor = store
            .open_local_compact_executor_bound("journal:writer-e26", fence, &lease)
            .await
            .expect("bound executor");
        let binding =
            LocalTurnLifecycleBinding::from_handles(TURN_ID, &lease, &executor).expect("binding");
        let input = QualificationTurnWriterInput::new(
            TURN_ID,
            binding,
            lease,
            executor,
            OCCURRENCE,
            r#"{"schema_version":1,"external_effect":false,"kg_write_authority":false}"#,
        )
        .expect("writer input");
        (temp, store, input)
    }

    fn start_input<'a>(
        turn_store: &'a ExtensionData,
        thread_store: &'a ExtensionData,
        session_store: &'a ExtensionData,
        mode: &'a CollaborationMode,
        usage: &'a TokenUsage,
    ) -> TurnStartInput<'a> {
        TurnStartInput {
            turn_id: TURN_ID,
            collaboration_mode: mode,
            token_usage_at_turn_start: usage,
            session_store,
            thread_store,
            turn_store,
        }
    }

    #[tokio::test]
    async fn explicit_bound_writer_admits_once_and_releases_on_stop() {
        let (_temp, store, input) = prepared().await;
        let turn_store = ExtensionData::new(TURN_ID);
        let thread_store = ExtensionData::new("thread:writer-e26");
        let session_store = ExtensionData::new("session:writer-e26");
        assert!(attach_qualification_turn_writer(&turn_store, input.clone()));
        assert!(!attach_qualification_turn_writer(
            &turn_store,
            input.clone()
        ));
        let contributor = QualificationTurnLifecycleContributor::new();
        let mode = mode();
        let usage = TokenUsage::default();
        contributor
            .on_turn_start(start_input(
                &turn_store,
                &thread_store,
                &session_store,
                &mode,
                &usage,
            ))
            .await;
        contributor
            .on_turn_start(start_input(
                &turn_store,
                &thread_store,
                &session_store,
                &mode,
                &usage,
            ))
            .await;
        let counts = input.lease.snapshot_counts().await.expect("counts");
        assert_eq!(counts.event_rows, 1);
        assert_eq!(counts.outbox_rows, 1);
        contributor
            .on_turn_stop(TurnStopInput {
                session_store: &session_store,
                thread_store: &thread_store,
                turn_store: &turn_store,
            })
            .await;
        contributor
            .on_turn_stop(TurnStopInput {
                session_store: &session_store,
                thread_store: &thread_store,
                turn_store: &turn_store,
            })
            .await;
        assert!(
            store
                .reopen_local_lease(LEASE_ID, 1, input.lease.fencing_token().to_string())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn host_capability_seeds_thread_and_prepares_exact_turn_input() {
        let (_temp, _store, template) = prepared().await;
        let callback_template = template.clone();
        let host =
            QualificationTurnWriterHost::from_fn("qualification-test-host", move |turn_id| {
                let callback_template = callback_template.clone();
                async move {
                    QualificationTurnWriterInput::new(
                        turn_id,
                        callback_template.binding.clone(),
                        callback_template.lease.clone(),
                        callback_template.executor.clone(),
                        callback_template.occurrence_key.clone(),
                        callback_template.payload_json.clone(),
                    )
                }
            });
        let contributor = QualificationTurnLifecycleContributor::with_host(host.clone());
        let turn_store = ExtensionData::new(TURN_ID);
        let thread_store = ExtensionData::new("thread:writer-e26");
        let session_store = ExtensionData::new("session:writer-e26");
        let config = ();
        contributor
            .on_thread_start(ThreadStartInput {
                config: &config,
                session_source: &SessionSource::Cli,
                installation_id: "qualification-installation",
                persistent_thread_state_available: true,
                environments: &[],
                mcp_resource_client: None,
                extension_metrics: None,
                session_store: &session_store,
                thread_store: &thread_store,
            })
            .await;
        assert_eq!(
            thread_store
                .get::<QualificationTurnWriterHost>()
                .expect("thread host capability")
                .capability_id(),
            "qualification-test-host"
        );

        let mode = mode();
        let usage = TokenUsage::default();
        contributor
            .on_turn_start(start_input(
                &turn_store,
                &thread_store,
                &session_store,
                &mode,
                &usage,
            ))
            .await;
        assert!(
            turn_store.get::<QualificationTurnWriterInput>().is_some(),
            "host callback must attach the exact prepared input"
        );
        let counts = template.lease.snapshot_counts().await.expect("counts");
        assert_eq!(counts.event_rows, 1);
        assert_eq!(counts.outbox_rows, 1);
        contributor
            .on_turn_stop(TurnStopInput {
                session_store: &session_store,
                thread_store: &thread_store,
                turn_store: &turn_store,
            })
            .await;
    }

    #[tokio::test]
    async fn reopened_host_replays_same_occurrence_without_second_outbox_row() {
        let (temp, store, input) = prepared().await;
        let first_turn = ExtensionData::new(TURN_ID);
        let thread_store = ExtensionData::new("thread:writer-e26");
        let session_store = ExtensionData::new("session:writer-e26");
        assert!(attach_qualification_turn_writer(&first_turn, input.clone()));
        let contributor = QualificationTurnLifecycleContributor::new();
        let mode = mode();
        let usage = TokenUsage::default();
        contributor
            .on_turn_start(start_input(
                &first_turn,
                &thread_store,
                &session_store,
                &mode,
                &usage,
            ))
            .await;
        let counts_before = input.lease.snapshot_counts().await.expect("counts");
        drop(contributor);
        drop(first_turn);
        drop(input);
        drop(store);

        let fleet_root = temp.path().join("fleet");
        let fleet = HeptaFleetRoot::parse(fleet_root).expect("fleet reopen");
        let owner = AgentId::parse("00000000-0000-4000-8000-000000000981").expect("owner");
        let reopened_store = CognitiveStore::open(&fleet.layout().agent(&owner))
            .await
            .expect("reopen store");
        let lease = reopened_store
            .reopen_local_lease(LEASE_ID, 1, "writer-e26-fence")
            .await
            .expect("reopen lease");
        let fence = CompactFence::new(17, 19, 1, "writer-e26-fence").expect("fence");
        let executor = reopened_store
            .open_local_compact_executor_bound("journal:writer-e26", fence, &lease)
            .await
            .expect("reopen executor");
        let binding = LocalTurnLifecycleBinding::from_handles(TURN_ID, &lease, &executor)
            .expect("reopen binding");
        let input = QualificationTurnWriterInput::new(
            TURN_ID,
            binding,
            lease,
            executor,
            OCCURRENCE,
            r#"{"schema_version":1,"external_effect":false,"kg_write_authority":false}"#,
        )
        .expect("reopen input");
        let second_turn = ExtensionData::new(TURN_ID);
        assert!(attach_qualification_turn_writer(
            &second_turn,
            input.clone()
        ));
        let contributor = QualificationTurnLifecycleContributor::new();
        contributor
            .on_turn_start(start_input(
                &second_turn,
                &thread_store,
                &session_store,
                &mode,
                &usage,
            ))
            .await;
        assert_eq!(
            input.lease.snapshot_counts().await.expect("counts"),
            counts_before
        );
        contributor
            .on_turn_stop(TurnStopInput {
                session_store: &session_store,
                thread_store: &thread_store,
                turn_store: &second_turn,
            })
            .await;
    }

    /// The qualification-only admission/Saga/forget path must keep all state
    /// in one real Agent-local SQLite store: a candidate is admitted,
    /// explicitly verified without facts, bound to one lifecycle outbox
    /// occurrence, host-tombstoned, and then observed again after reopen.
    ///
    /// This intentionally exercises no production caller, shared KG, or
    /// external effect.  The queued outbox row is only a local intent, and a
    /// replay after the simulated crash must return that same row rather than
    /// append a second admission.
    #[tokio::test]
    async fn qualification_admission_saga_forget_tombstone_survives_reopen() {
        let (temp, store, prepared_input) = prepared().await;
        let owner = prepared_input.lease.owner_agent_id().clone();
        let access = CognitiveAccess::agent_private(owner.clone());
        let content = "qualification candidate is withdrawn by the host";
        let candidate = store
            .admit_memory_candidate(
                &access,
                &MemoryCandidateDraft {
                    stable_key: "writer-e26-forget-candidate".to_string(),
                    scope: CognitiveScope::AgentPrivate,
                    content: content.to_string(),
                    source_event_key: "qualification:writer-e26:candidate".to_string(),
                    observed_at_unix_seconds: 100,
                    origin: MemoryCandidateOrigin::CompactionSummary,
                },
            )
            .await
            .expect("candidate admission");
        assert_eq!(candidate.state, MemoryCandidateState::Provisional);
        assert!(!candidate.fact_admitted);
        assert_eq!(candidate.write.projection.entity_count, 0);
        assert_eq!(candidate.write.projection.relation_count, 0);

        let evidence = MemoryAdmissionEvidence::from_bytes(content.as_bytes(), "qualification")
            .expect("content-bound evidence");
        let verify_source = SourceDraft {
            scope: CognitiveScope::AgentPrivate,
            kind: LedgerSourceKind::ExplicitMemoryDirective,
            event_key: "qualification:writer-e26:verify".to_string(),
            content: content.as_bytes().to_vec(),
            observed_at_unix_seconds: 101,
        };
        let verified = store
            .verify_memory_candidate(
                &access,
                &candidate.candidate_id,
                1,
                candidate.origin,
                &verify_source,
                content.to_string(),
                101,
                &evidence,
                &KgFactSetDraft::default(),
            )
            .await
            .expect("candidate verification");
        assert_eq!(verified.state, MemoryCandidateState::Verified);
        assert!(!verified.fact_admitted);
        assert_eq!(verified.revision, 2);
        assert_eq!(verified.write.projection.entity_count, 0);
        assert_eq!(verified.write.projection.relation_count, 0);

        let before_forget = store
            .retrieve_memory_candidates(&access, &RetrievalRequest::new("withdrawn host", 200))
            .await
            .expect("pre-forget retrieval");
        assert_eq!(before_forget.candidates.len(), 1);
        assert_eq!(
            before_forget.candidates[0].memory.id.memory_id,
            candidate.candidate_id
        );

        // Bind the Saga payload to the exact local candidate identity.  This
        // remains a local queue intent; it is never sent to a provider.
        let writer_input = QualificationTurnWriterInput::new(
            TURN_ID,
            prepared_input.binding.clone(),
            prepared_input.lease.clone(),
            prepared_input.executor.clone(),
            OCCURRENCE,
            format!(
                r#"{{"candidate_id":"{}","external_effect":false,"kg_write_authority":false}}"#,
                candidate.candidate_id.as_str()
            ),
        )
        .expect("candidate-bound writer input");
        let turn_store = ExtensionData::new(TURN_ID);
        let thread_store = ExtensionData::new("thread:writer-e26");
        let session_store = ExtensionData::new("session:writer-e26");
        assert!(attach_qualification_turn_writer(
            &turn_store,
            writer_input.clone()
        ));
        let contributor = QualificationTurnLifecycleContributor::new();
        let mode = mode();
        let usage = TokenUsage::default();
        contributor
            .on_turn_start(start_input(
                &turn_store,
                &thread_store,
                &session_store,
                &mode,
                &usage,
            ))
            .await;
        let counts_before_forget = writer_input.lease.snapshot_counts().await.expect("counts");
        assert_eq!(counts_before_forget.event_rows, 1);
        assert_eq!(counts_before_forget.outbox_rows, 1);
        assert!(
            !writer_input
                .lease
                .admit(
                    OCCURRENCE,
                    TURN_START_TOPIC,
                    r#"{"candidate_id":"different"}"#,
                )
                .await
                .is_ok(),
            "replaying with a different payload must fail closed"
        );

        let reason = "qualification host withdrawal".to_string();
        let tombstone_source = SourceDraft {
            scope: CognitiveScope::AgentPrivate,
            kind: LedgerSourceKind::ExplicitMemoryDirective,
            event_key: "qualification:writer-e26:forget".to_string(),
            content: reason.as_bytes().to_vec(),
            observed_at_unix_seconds: 102,
        };
        let tombstone = store
            .tombstone_memory_candidate(
                &access,
                &candidate.candidate_id,
                2,
                candidate.origin,
                &tombstone_source,
                reason,
                102,
            )
            .await
            .expect("host tombstone");
        assert_eq!(tombstone.state, MemoryCandidateState::Tombstoned);
        assert_eq!(tombstone.revision, 3);
        assert!(!tombstone.fact_admitted);
        assert_eq!(tombstone.write.projection.entity_count, 0);
        assert_eq!(tombstone.write.projection.relation_count, 0);
        assert!(matches!(
            tombstone.write.memory.lifecycle,
            MemoryLifecycleState::Tombstoned { .. }
        ));
        assert!(
            store
                .retrieve_memory_candidates(&access, &RetrievalRequest::new("withdrawn host", 200))
                .await
                .expect("post-forget retrieval")
                .candidates
                .is_empty()
        );

        // Simulate a host crash after durable admission and forget but before
        // lifecycle terminalization.  Reopen must replay the same local
        // occurrence and keep the immutable tombstone visible.
        drop(contributor);
        drop(turn_store);
        drop(writer_input);
        drop(prepared_input);
        drop(store);

        let fleet = HeptaFleetRoot::parse(temp.path().join("fleet")).expect("fleet reopen");
        let reopened_store = CognitiveStore::open(&fleet.layout().agent(&owner))
            .await
            .expect("reopen store");
        let reopened_access = CognitiveAccess::agent_private(owner.clone());
        let reopened_latest = reopened_store
            .latest_memory(&reopened_access, &candidate.candidate_id)
            .await
            .expect("reopened tombstone");
        assert_eq!(reopened_latest.id.revision, 3);
        assert!(matches!(
            reopened_latest.lifecycle,
            MemoryLifecycleState::Tombstoned { .. }
        ));
        assert!(
            reopened_store
                .retrieve_memory_candidates(
                    &reopened_access,
                    &RetrievalRequest::new("withdrawn host", 200)
                )
                .await
                .expect("reopened post-forget retrieval")
                .candidates
                .is_empty()
        );

        let reopened_lease = reopened_store
            .reopen_local_lease(LEASE_ID, 1, "writer-e26-fence")
            .await
            .expect("reopen active Saga lease");
        let replay = reopened_lease
            .finalize_replayed_occurrence(OCCURRENCE)
            .await
            .expect("replay local occurrence");
        match replay {
            LocalReplayFinalization::Queued(receipt) => {
                assert!(!receipt.external_effect);
            }
            other => panic!("queued occurrence changed state on reopen: {other:?}"),
        }
        assert_eq!(
            reopened_lease
                .snapshot_counts()
                .await
                .expect("reopened counts"),
            counts_before_forget
        );
        let released = reopened_lease
            .release()
            .await
            .expect("terminalize Saga lease");
        assert_eq!(released.state, LocalLeaseState::Released);
        assert!(!QUALIFICATION_TURN_WRITER_EXTERNAL_EFFECTS);
        assert!(!QUALIFICATION_TURN_WRITER_KG_WRITE_AUTHORITY);
        assert!(!QUALIFICATION_TURN_WRITER_PRODUCTION_CALLER);
    }

    #[tokio::test]
    async fn forged_binding_is_rejected_before_any_admission() {
        let (_temp, _store, mut input) = prepared().await;
        input.binding.fence.owner_epoch += 1;
        let turn_store = ExtensionData::new(TURN_ID);
        assert!(attach_qualification_turn_writer(&turn_store, input));
        let contributor = QualificationTurnLifecycleContributor::new();
        let thread_store = ExtensionData::new("thread:writer-e26");
        let session_store = ExtensionData::new("session:writer-e26");
        let mode = mode();
        let usage = TokenUsage::default();
        contributor
            .on_turn_start(start_input(
                &turn_store,
                &thread_store,
                &session_store,
                &mode,
                &usage,
            ))
            .await;
        let state = turn_store.get::<Mutex<TurnWriterState>>();
        assert!(
            state.is_none(),
            "invalid input must not create writer state"
        );
    }

    #[tokio::test]
    async fn stale_terminal_callback_verifies_before_indeterminate_write() {
        let (_temp, _store, input) = prepared().await;
        input
            .lease
            .admit(OCCURRENCE, TURN_START_TOPIC, input.payload_json.clone())
            .await
            .expect("admit occurrence");

        // Keep the lease active but make only the callback's copied binding
        // stale.  A terminal callback must reject this before appending an
        // indeterminate outcome; otherwise a late stale owner can mutate the
        // current occurrence even though its fence no longer matches.
        let mut stale = input.clone();
        stale.binding.fence.owner_epoch += 1;
        assert!(
            QualificationTurnLifecycleContributor::complete(
                &stale,
                TerminalAction::Indeterminate("stale callback".to_string()),
            )
            .await
            .is_err()
        );

        let replay = input
            .lease
            .finalize_replayed_occurrence(OCCURRENCE)
            .await
            .expect("replay after stale callback");
        assert!(matches!(replay, LocalReplayFinalization::Queued(_)));
    }
}
