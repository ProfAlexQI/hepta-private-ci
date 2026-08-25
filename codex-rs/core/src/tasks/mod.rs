mod compact;
mod lifecycle;
mod regular;
mod review;
mod user_shell;

use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;

use codex_diagnostics::Gauge;
use codex_extension_api::QualificationTurnAdmissionIdentity;
use codex_extension_api::ThreadIdleCause;
use futures::FutureExt;
use futures::future::BoxFuture;
use tokio::select;
use tokio::sync::Mutex;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;
use tokio_util::task::AbortOnDropHandle;
use tracing::Instrument;
use tracing::Span;
use tracing::field;
use tracing::info_span;
use tracing::trace;
use tracing::trace_span;
use tracing::warn;

use crate::codex_thread::BackgroundTerminalInfo;
use crate::config::Config;
use crate::context::ContextualUserFragment;
use crate::context_manager::ContextManager;
use crate::session::TurnInput;
use crate::session::session::RecoveryCandidate;
use crate::session::session::Session;
use crate::session::turn::TurnRunOrigin;
use crate::session::turn::run_hooks_and_record_inputs;
use crate::session::turn_context::TurnContext;
use crate::state::ActiveTurn;
use crate::state::DurableRecoveryState;
use crate::state::RunningTask;
use crate::state::StartReservationHandle;
use crate::state::StartTransition;
use crate::state::StartTransitionCompletion;
use crate::state::TaskKind;
use crate::state::TaskTerminalization;
use crate::state::TaskTerminalizationKind;
use crate::state::TaskTerminalizationPhase;
use crate::state::TurnRecoveryAuthority;
use crate::state::TurnState;
use codex_analytics::TurnProfileFact;
use codex_analytics::TurnTokenUsageFact;
use codex_context_fragments::RenderedFragment;
use codex_otel::SessionTelemetry;
use codex_otel::TURN_E2E_DURATION_METRIC;
use codex_otel::TURN_MEMORY_METRIC;
use codex_otel::TURN_NETWORK_PROXY_METRIC;
use codex_otel::TURN_TOKEN_USAGE_METRIC;
use codex_otel::TURN_TOOL_CALL_METRIC;
use codex_otel::TURN_UNIFIED_EXEC_RUNNING_PROCESSES_METRIC;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::MultiAgentVersion;
use codex_protocol::protocol::TokenUsage;
use codex_protocol::protocol::TurnAbortReason;
use codex_protocol::protocol::TurnAbortedEvent;
use codex_protocol::protocol::TurnCompleteEvent;
use codex_protocol::protocol::WarningEvent;
use codex_protocol::user_input::user_input_payload_sha256;
use codex_thread_store::PersistContext;

use codex_features::Feature;
use codex_protocol::error::CodexErr;
use codex_protocol::error::CodexErrorDetails;
use codex_protocol::error::Result as CodexResult;
pub(crate) use compact::CompactTask;
pub(crate) use regular::RegularTask;
pub(crate) use review::ReviewTask;
pub(crate) use user_shell::UserShellCommandMode;
pub(crate) use user_shell::UserShellCommandTask;
pub(crate) use user_shell::execute_user_shell_command;

pub(crate) const GRACEFULL_INTERRUPTION_TIMEOUT_MS: u64 = 100;
const TASK_COMPACT_METRIC: &str = "codex.task.compact";
static ACTIVE_TURNS: Gauge = Gauge::new("core.turns.active");

pub(crate) type SessionTaskResult = CodexResult<Option<String>>;

pub(crate) enum MailboxParentProvenance {
    Ignore,
    Attribute,
}

/// The two history views needed for a recovery start.  The rewound view is
/// installed only after the host-owned start marker exists; the original view
/// is restored if the transition is aborted before `RunningTask` attach.
pub(crate) struct RecoveryHistoryTransition {
    pub(crate) install: ContextManager,
    pub(crate) restore: ContextManager,
}

/// Snapshot of one exact task's recovery authority before it is detached.
///
/// The authority itself remains attached so the terminal publication path can
/// prove that no Ready -> Unready/Ready transition raced with cancellation.
struct RecoverySeed {
    turn_id: String,
    authority: Arc<TurnRecoveryAuthority>,
    generation: u64,
    request_fingerprint_sha256: String,
    replay: codex_history::TurnRecoveryReplayV1,
    persistence_failure_generation: u64,
    attach_epoch: u64,
    confirmation_generation: Option<u64>,
}

#[derive(Default)]
struct TaskAbortOutcome {
    task_quiesced: bool,
    terminal_persistence_generation: Option<u64>,
    recovery_seed: Option<RecoverySeed>,
}

struct DetachedTaskForAbort {
    task: RunningTask,
    turn_state: Arc<Mutex<TurnState>>,
    terminalization_identity: Arc<()>,
    recovery_seed: Option<RecoverySeed>,
    recovery_authority: Option<Arc<TurnRecoveryAuthority>>,
}

/// Exact task witness owned by root-turn suspension.  Suspension is not a
/// terminal protocol outcome, but it still has to keep the active slot fenced
/// while cancellation and persistence drain; the typed marker prevents a
/// replacement admission from observing a false idle turn.
pub(crate) struct DetachedTaskForSuspension {
    pub(crate) task: Option<RunningTask>,
    pub(crate) turn_state: Arc<Mutex<TurnState>>,
    pub(crate) terminalization_identity: Arc<()>,
    pub(crate) task_identity: Arc<dyn AnySessionTask>,
    pub(crate) turn_context: Arc<TurnContext>,
    pub(crate) attach_epoch: u64,
}

enum ActiveTurnAbortTarget {
    Running(DetachedTaskForAbort),
    Starting { deferred_idle: bool },
    Terminalizing,
}

enum StartTransitionClearOutcome {
    Stale,
    Cleared {
        deferred_idle_cause: Option<ThreadIdleCause>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AbortTurnOutcome {
    NotActive,
    Running,
    Starting,
    DeferredStart,
    Terminalizing,
}

#[derive(Debug)]
pub(crate) enum StartReservationRelease {
    Released,
    AbortRequested(TurnAbortReason),
    Stale,
}

/// Owns a caller-owned start reservation across the asynchronous preparation
/// preamble.  The reservation must not outlive the future that installed it:
/// cancellation can happen at any of the awaits before `start_task_owned`
/// promotes it to a host-owned transition.  Normal callers disarm this guard
/// after the owned start outcome is known; `Drop` supplies the cancellation
/// path and uses the same identity CAS as explicit cleanup.
pub(crate) struct StartReservationOwner {
    session: Arc<Session>,
    handle: Option<StartReservationHandle>,
}

impl StartReservationOwner {
    pub(crate) fn new(session: &Arc<Session>, handle: StartReservationHandle) -> Self {
        Self {
            session: Arc::clone(session),
            handle: Some(handle),
        }
    }

    pub(crate) fn handle(&self) -> &StartReservationHandle {
        self.handle
            .as_ref()
            .expect("start reservation owner remains armed until completion")
    }

    pub(crate) fn disarm(&mut self) {
        self.handle = None;
    }
}

impl Drop for StartReservationOwner {
    fn drop(&mut self) {
        let Some(handle) = self.handle.take() else {
            return;
        };
        let session = Arc::clone(&self.session);
        // A construction-time runtime handle is not a liveness witness:
        // Tokio silently shuts down a future submitted through a closed
        // `OwnedTasks` list.  Only use the thread-local current runtime for
        // the asynchronous path.  When Drop runs off-runtime (including
        // after the construction runtime has shut down), perform the exact
        // identity CAS synchronously; this reservation release has no async
        // persistence or lifecycle ordering to preserve.
        let Some(runtime_handle) = tokio::runtime::Handle::try_current().ok() else {
            let mut active = session.active_turn.blocking_lock();
            let release =
                Session::release_start_reservation_if_current_locked(&mut active, &handle);
            if !matches!(release, StartReservationRelease::Stale) {
                session.settle_consumed_recovery_status();
            }
            return;
        };
        runtime_handle.spawn(async move {
            let release = session.release_start_reservation_if_current(&handle).await;
            if !matches!(release, StartReservationRelease::Stale) {
                session.settle_consumed_recovery_status();
            }
        });
    }
}

/// Payload retained by the detached cleanup continuation for one exact
/// host-owned start transition.  The owner future is allowed to disappear at
/// any cancellation point after promotion; this witness gives the cleanup
/// continuation enough identity to terminalize only that transition.
struct StartTransitionCleanup {
    session: Arc<Session>,
    task: Arc<dyn AnySessionTask>,
    turn_context: Arc<TurnContext>,
    turn_state: Arc<Mutex<TurnState>>,
    transition_identity: Arc<()>,
    completion: Arc<StartTransitionCompletion>,
    recovery_history_restore: Option<ContextManager>,
}

/// Keeps a host-owned start transition live independently of its caller while
/// the Tokio runtime is still able to schedule a cleanup continuation.
///
/// A caller-owned reservation is released by [`StartReservationOwner`], but
/// promotion changes the state machine: the transition now owns lifecycle and
/// terminal ordering.  If the owner future is cancelled after that point, a
/// detached placeholder terminalizer finishes the same abort path instead of
/// leaving `active_turn.start_transition` installed forever.  Panic/hang
/// handling in the terminalizer itself remains a separate watchdog slice.  If
/// the runtime has already shut down, this guard deliberately preserves the
/// marker and fails closed; a future session-shutdown drain must own that
/// terminalization rather than silently dropping durable/lifecycle state.
struct StartTransitionOwner {
    cleanup: Option<StartTransitionCleanup>,
    runtime_handle: Option<tokio::runtime::Handle>,
    cleanup_spawned: bool,
}

impl StartTransitionOwner {
    fn new(
        session: &Arc<Session>,
        task: Arc<dyn AnySessionTask>,
        turn_context: Arc<TurnContext>,
        turn_state: Arc<Mutex<TurnState>>,
        transition_identity: Arc<()>,
        completion: Arc<StartTransitionCompletion>,
        recovery_history_restore: Option<ContextManager>,
    ) -> Self {
        Self {
            cleanup: Some(StartTransitionCleanup {
                session: Arc::clone(session),
                task,
                turn_context,
                turn_state,
                transition_identity,
                completion,
                recovery_history_restore,
            }),
            runtime_handle: tokio::runtime::Handle::try_current().ok(),
            cleanup_spawned: false,
        }
    }

    /// Spawn exactly one detached terminalizer.  The returned handle is only
    /// for callers that want to await normal terminal ordering; dropping the
    /// owner after this method has been called never aborts the cleanup task.
    fn spawn_cleanup(
        &mut self,
        fallback_reason: TurnAbortReason,
    ) -> Option<tokio::task::JoinHandle<()>> {
        if self.cleanup_spawned {
            return None;
        }
        let cleanup = self.cleanup.take()?;
        let Some(runtime_handle) = self.runtime_handle.take() else {
            warn!(
                turn_id = %cleanup.turn_context.sub_id,
                "start transition cleanup cannot be scheduled because no Tokio runtime is available; preserving its fence"
            );
            // No runtime exists on which a durable terminalizer could run.
            // Keep the cleanup payload attached to the owner and leave the
            // completion unresolved so teardown fails closed instead of
            // silently losing the transition's terminal work.
            self.cleanup = Some(cleanup);
            self.cleanup_spawned = true;
            return None;
        };
        self.cleanup_spawned = true;
        let session = Arc::clone(&cleanup.session);
        let turn_id = cleanup.turn_context.sub_id.clone();
        Some(runtime_handle.spawn(async move {
            let result =
                AssertUnwindSafe(session.abort_dropped_start_transition(cleanup, fallback_reason))
                    .catch_unwind()
                    .await;
            if result.is_err() {
                warn!(
                    %turn_id,
                    "start-transition terminalizer panicked; retaining its completion fence"
                );
            }
        }))
    }

    /// Mark a successfully attached or already-stale transition as complete.
    fn disarm(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup
                .session
                .finish_start_transition(&cleanup.transition_identity, &cleanup.completion);
        }
        self.cleanup_spawned = true;
    }
}

impl Drop for StartTransitionOwner {
    fn drop(&mut self) {
        // A dropped owner is an implicit interruption.  The detached task is
        // deliberately not stored in this guard, so dropping the guard cannot
        // abort the terminalizer it just spawned while the runtime is live.
        let _ = self.spawn_cleanup(TurnAbortReason::Interrupted);
    }
}

impl Session {
    /// Registers the completion fence before releasing the active-turn lock.
    /// The registry intentionally outlives `ActiveTurn::start_transition` so
    /// shutdown can wait through post-clear idle/recovery/pending-work side
    /// effects.
    fn register_start_transition(
        &self,
        transition_identity: Arc<()>,
        completion: Arc<StartTransitionCompletion>,
    ) {
        self.pending_start_transition_completions
            .lock()
            .expect("start transition completion registry mutex poisoned")
            .push((transition_identity, completion));
    }

    /// Removes one exact fence before signalling it.  Identity and completion
    /// are both checked so a stale continuation cannot retire a later turn's
    /// registry entry.
    fn finish_start_transition(
        &self,
        transition_identity: &Arc<()>,
        completion: &Arc<StartTransitionCompletion>,
    ) {
        let mut pending = self
            .pending_start_transition_completions
            .lock()
            .expect("start transition completion registry mutex poisoned");
        pending.retain(|(identity, current)| {
            !(Arc::ptr_eq(identity, transition_identity) && Arc::ptr_eq(current, completion))
        });
        drop(pending);
        completion.complete();
    }

    fn pending_start_transition_completions(&self) -> Vec<Arc<StartTransitionCompletion>> {
        self.pending_start_transition_completions
            .lock()
            .expect("start transition completion registry mutex poisoned")
            .iter()
            .map(|(_, completion)| Arc::clone(completion))
            .collect()
    }

    /// Registers a task terminalizer before its owner releases the
    /// active-turn lock.  The registry is independent of `ActiveTurn` because
    /// a replacement may be admitted as soon as the old slot CAS succeeds,
    /// while shutdown still has to wait for the old owner's post-CAS recovery
    /// and lifecycle side effects.
    fn register_task_terminalization(
        &self,
        identity: Arc<()>,
        completion: Arc<StartTransitionCompletion>,
        kind: TaskTerminalizationKind,
    ) {
        self.pending_task_terminalization_completions
            .lock()
            .expect("task terminalization completion registry mutex poisoned")
            .push((identity, completion, kind));
    }

    /// Retires and signals one exact task terminalizer after all of its
    /// post-terminal side effects have completed.  A stale owner cannot
    /// retire a later entry because both identity pointers are checked.
    fn finish_task_terminalization(
        &self,
        identity: &Arc<()>,
        completion: &Arc<StartTransitionCompletion>,
    ) {
        let mut pending = self
            .pending_task_terminalization_completions
            .lock()
            .expect("task terminalization completion registry mutex poisoned");
        pending.retain(|(current_identity, current_completion, _kind)| {
            !(Arc::ptr_eq(current_identity, identity)
                && Arc::ptr_eq(current_completion, completion))
        });
        drop(pending);
        completion.complete();
    }

    fn pending_task_terminalization_completions_except(
        &self,
        excluded_identity: Option<&Arc<()>>,
    ) -> Vec<Arc<StartTransitionCompletion>> {
        self.pending_task_terminalization_completions
            .lock()
            .expect("task terminalization completion registry mutex poisoned")
            .iter()
            .filter(|(identity, _, _)| {
                excluded_identity
                    .map_or(true, |excluded| !Arc::ptr_eq(identity, excluded))
            })
            .map(|(_, completion, _)| Arc::clone(completion))
            .collect()
    }

    /// Shutdown waits for every materialized task terminalizer.  A panic or
    /// hang intentionally leaves its fence unresolved, so teardown remains
    /// fail-closed instead of publishing thread-stop or closing persistence
    /// behind an unfinished terminal path.
    pub(crate) async fn drain_task_terminalizations_for_shutdown_except(
        &self,
        excluded_identity: Option<&Arc<()>>,
    ) {
        loop {
            let completions = self.pending_task_terminalization_completions_except(excluded_identity);
            if completions.is_empty() {
                return;
            }
            for completion in completions {
                completion.wait().await;
            }
        }
    }

    pub(crate) fn has_pending_start_transition(&self) -> bool {
        self.has_pending_start_transition_except(None)
    }

    pub(crate) fn has_pending_start_transition_except(
        &self,
        ignored_identity: Option<&Arc<()>>,
    ) -> bool {
        self.pending_start_transition_completions
            .lock()
            .expect("start transition completion registry mutex poisoned")
            .iter()
            .any(|(identity, _)| {
                ignored_identity.map_or(true, |ignored| !Arc::ptr_eq(identity, ignored))
            })
    }

    /// Returns true while any task finish/abort/suspend owner still has
    /// post-terminal work outstanding.  The marker is intentionally kept in
    /// an independent registry after the active slot CAS so idle observers
    /// cannot admit history mutation or a replacement in the clear→publish
    /// window.
    pub(crate) fn has_pending_task_terminalization(&self) -> bool {
        self.has_pending_task_terminalization_except(None)
    }

    pub(crate) fn has_pending_task_terminalization_except(
        &self,
        ignored_identity: Option<&Arc<()>>,
    ) -> bool {
        self.pending_task_terminalization_completions
            .lock()
            .expect("task terminalization completion registry mutex poisoned")
            .iter()
            .any(|(identity, _, _)| {
                ignored_identity.map_or(true, |ignored| !Arc::ptr_eq(identity, ignored))
            })
    }

    /// Admission-facing fence for host starts.  Keep the legacy
    /// `has_pending_start_transition` semantics for shutdown's dedicated
    /// transition drain, while all normal starts also respect task
    /// terminalizers that have already cleared the active slot.
    pub(crate) fn has_pending_admission_fence(&self) -> bool {
        self.has_pending_admission_fence_except(None)
    }

    pub(crate) fn has_pending_admission_fence_except(
        &self,
        ignored_terminalization: Option<&Arc<()>>,
    ) -> bool {
        self.has_pending_start_transition()
            || self.has_pending_task_terminalization_except(ignored_terminalization)
    }

    pub(crate) fn begin_shutdown(&self) {
        self.shutdown_started.store(true, Ordering::Release);
    }

    pub(crate) fn shutdown_started(&self) -> bool {
        self.shutdown_started.load(Ordering::Acquire)
    }
}

/// Result of handing a materialized turn context to the host start state
/// machine.  Owned callers use this to avoid reporting `Started` after a
/// caller reservation was fenced by an abort or replacement; legacy direct
/// callers may ignore the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StartTaskOutcome {
    Attached,
    Aborted,
    Stale,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RecoveryReadyForSampling {
    Ready,
    PendingInput,
    Detached,
}

/// One outer sampling generation may authorize the same pre-output physical
/// request across auth retry or HTTP/WS fallback. Once any provider output or
/// terminal error closes the arm, an internal retry can never mint Ready
/// again from the same logical sampling generation.
pub(crate) struct RecoveryDispatchArm {
    closed: AtomicBool,
    recovery_disabled: AtomicBool,
    expected_fingerprint_sha256: Option<String>,
}

impl RecoveryDispatchArm {
    pub(crate) fn new(expected_fingerprint_sha256: Option<String>) -> Self {
        Self {
            closed: AtomicBool::new(false),
            recovery_disabled: AtomicBool::new(false),
            expected_fingerprint_sha256,
        }
    }

    pub(crate) fn close(&self) {
        self.closed.store(true, Ordering::Release);
    }

    pub(crate) fn is_open(&self) -> bool {
        !self.closed.load(Ordering::Acquire)
    }

    pub(crate) fn disable_recovery(&self) {
        self.recovery_disabled.store(true, Ordering::Release);
    }

    pub(crate) fn recovery_disabled(&self) -> bool {
        self.recovery_disabled.load(Ordering::Acquire)
    }

    pub(crate) fn expected_fingerprint_sha256(&self) -> Option<&str> {
        self.expected_fingerprint_sha256.as_deref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RecoveryProviderOutputGate {
    Attached,
    Detached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InterruptedTurnHistoryMarker {
    Disabled,
    ContextualUser,
    Developer,
}

impl InterruptedTurnHistoryMarker {
    pub(crate) fn from_config_and_version(
        config: &Config,
        multi_agent_version: MultiAgentVersion,
    ) -> Self {
        if !config.agent_interrupt_message_enabled {
            return Self::Disabled;
        }
        if multi_agent_version == MultiAgentVersion::V2 {
            Self::Developer
        } else {
            Self::ContextualUser
        }
    }
}

/// Shared model-visible marker used by both the real interrupt path and
/// interrupted fork snapshots.
pub(crate) fn interrupted_turn_history_marker(
    marker: InterruptedTurnHistoryMarker,
) -> Option<ResponseItem> {
    match marker {
        InterruptedTurnHistoryMarker::Disabled => None,
        InterruptedTurnHistoryMarker::ContextualUser => Some(ContextualUserFragment::into(
            crate::context::TurnAborted::new(crate::context::TurnAborted::INTERRUPTED_GUIDANCE),
        )),
        InterruptedTurnHistoryMarker::Developer => {
            let marker = crate::context::TurnAborted::new(
                crate::context::TurnAborted::INTERRUPTED_DEVELOPER_GUIDANCE,
            );
            let (_, content) = marker.render_fragment().into_parts();
            Some(RenderedFragment::new("developer", content).into())
        }
    }
}

fn emit_turn_network_proxy_metric(
    session_telemetry: &SessionTelemetry,
    network_proxy_active: bool,
    tmp_mem: (&str, &str),
) {
    let active = if network_proxy_active {
        "true"
    } else {
        "false"
    };
    session_telemetry.counter(
        TURN_NETWORK_PROXY_METRIC,
        /*inc*/ 1,
        &[("active", active), tmp_mem],
    );
}

fn emit_turn_memory_metric(
    session_telemetry: &SessionTelemetry,
    feature_enabled: bool,
    config_enabled: bool,
    has_citations: bool,
) {
    let read_allowed = feature_enabled && config_enabled;
    session_telemetry.counter(
        TURN_MEMORY_METRIC,
        /*inc*/ 1,
        &[
            ("read_allowed", bool_tag(read_allowed)),
            ("feature_enabled", bool_tag(feature_enabled)),
            ("config_use_memories", bool_tag(config_enabled)),
            ("has_citations", bool_tag(has_citations)),
        ],
    );
}

pub(crate) fn emit_compact_metric(
    session_telemetry: &SessionTelemetry,
    compact_type: &'static str,
    manual: bool,
) {
    session_telemetry.counter(
        TASK_COMPACT_METRIC,
        /*inc*/ 1,
        &[("type", compact_type), ("manual", bool_tag(manual))],
    );
}

fn bool_tag(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

/// Async task that drives a [`Session`] turn.
///
/// Implementations encapsulate a specific Codex workflow (regular chat,
/// reviews, ghost snapshots, etc.). Each task instance is owned by a
/// [`Session`] and executed on a background Tokio task. The trait is
/// intentionally small: implementers identify themselves via
/// [`SessionTask::kind`], perform their work in [`SessionTask::run`], and may
/// release resources in [`SessionTask::abort`].
pub(crate) trait SessionTask: Send + Sync + 'static {
    /// Describes the type of work the task performs so the session can
    /// surface it in telemetry and UI.
    fn kind(&self) -> TaskKind;

    /// Whether an interrupted instance can resume as the same model turn.
    /// Auxiliary tasks must leave this false even when their UI kind is
    /// `Regular`.
    fn recovery_eligible_model_turn(&self) -> bool {
        false
    }

    /// Shared task-owned authority that becomes Ready only at a strict
    /// provider-dispatch checkpoint.
    fn recovery_authority(&self) -> Option<Arc<TurnRecoveryAuthority>> {
        None
    }

    /// Returns the tracing name for a spawned task span.
    fn span_name(&self) -> &'static str;

    /// Lifecycle origin exposed to extension contributors before the task
    /// reaches the provider boundary.
    fn turn_start_origin(&self) -> codex_extension_api::TurnStartOrigin {
        codex_extension_api::TurnStartOrigin::NewTurn
    }

    /// Executes the task until completion or cancellation.
    ///
    /// Implementations typically stream protocol events using `session` and
    /// `ctx`, returning an optional final agent message when finished. The
    /// provided `cancellation_token` is cancelled when the session requests an
    /// abort; implementers should watch for it and terminate quickly once it
    /// fires. Returning [`Some`] yields a final message that
    /// [`Session::on_task_finished`] will emit to the client. Returning
    /// [`CodexErr::TurnAborted`] completes the task through the aborted-turn
    /// lifecycle instead.
    fn run(
        self: Arc<Self>,
        session: Arc<Session>,
        ctx: Arc<TurnContext>,
        input: Vec<TurnInput>,
        cancellation_token: CancellationToken,
    ) -> impl std::future::Future<Output = SessionTaskResult> + Send;

    /// Gives the task a chance to perform cleanup after an abort.
    ///
    /// The default implementation is a no-op; override this if additional
    /// teardown or notifications are required once
    /// [`Session::abort_all_tasks`] cancels the task.
    /// An accepted host-owned start transition may invoke this hook before
    /// [`SessionTask::run`] has begun, so implementations must tolerate
    /// pre-run cleanup as well as cancellation of a running task.
    fn abort(
        &self,
        session: Arc<Session>,
        ctx: Arc<TurnContext>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let _ = (session, ctx);
        }
    }
}

pub(crate) trait AnySessionTask: Send + Sync + 'static {
    fn kind(&self) -> TaskKind;

    fn recovery_eligible_model_turn(&self) -> bool;

    fn recovery_authority(&self) -> Option<Arc<TurnRecoveryAuthority>>;

    fn span_name(&self) -> &'static str;

    fn turn_start_origin(&self) -> codex_extension_api::TurnStartOrigin;

    fn run(
        self: Arc<Self>,
        session: Arc<Session>,
        ctx: Arc<TurnContext>,
        input: Vec<TurnInput>,
        cancellation_token: CancellationToken,
    ) -> BoxFuture<'static, SessionTaskResult>;

    fn abort<'a>(&'a self, session: Arc<Session>, ctx: Arc<TurnContext>) -> BoxFuture<'a, ()>;
}

impl<T> AnySessionTask for T
where
    T: SessionTask,
{
    fn kind(&self) -> TaskKind {
        SessionTask::kind(self)
    }

    fn recovery_eligible_model_turn(&self) -> bool {
        SessionTask::recovery_eligible_model_turn(self)
    }

    fn recovery_authority(&self) -> Option<Arc<TurnRecoveryAuthority>> {
        SessionTask::recovery_authority(self)
    }

    fn span_name(&self) -> &'static str {
        SessionTask::span_name(self)
    }

    fn turn_start_origin(&self) -> codex_extension_api::TurnStartOrigin {
        SessionTask::turn_start_origin(self)
    }

    fn run(
        self: Arc<Self>,
        session: Arc<Session>,
        ctx: Arc<TurnContext>,
        input: Vec<TurnInput>,
        cancellation_token: CancellationToken,
    ) -> BoxFuture<'static, SessionTaskResult> {
        Box::pin(SessionTask::run(
            self,
            session,
            ctx,
            input,
            cancellation_token,
        ))
    }

    fn abort<'a>(&'a self, session: Arc<Session>, ctx: Arc<TurnContext>) -> BoxFuture<'a, ()> {
        Box::pin(SessionTask::abort(self, session, ctx))
    }
}

impl Session {
    /// Installs one exact terminalization owner while the active-turn lock is
    /// held. The running task stays attached until the owner has revoked its
    /// recovery authority, so every admission path continues to see a busy
    /// slot during the lock-free await phase.
    fn claim_task_terminalization_locked(
        &self,
        active_turn: &mut ActiveTurn,
        task_identity: &Arc<dyn AnySessionTask>,
        turn_context: &Arc<TurnContext>,
        attach_epoch: u64,
        kind: TaskTerminalizationKind,
    ) -> Option<(Arc<()>, Arc<StartTransitionCompletion>)> {
        if active_turn.task_terminalization.is_some() {
            return None;
        }
        let identity = Arc::new(());
        let completion = StartTransitionCompletion::new();
        active_turn.task_terminalization = Some(TaskTerminalization {
            identity: Arc::clone(&identity),
            task_identity: Arc::clone(task_identity),
            turn_context: Arc::clone(turn_context),
            turn_state: Arc::clone(&active_turn.turn_state),
            attach_epoch,
            kind,
            phase: TaskTerminalizationPhase::Claimed,
            completion: Arc::clone(&completion),
        });
        // Register while the active-turn lock is still held.  A shutdown
        // drain must never observe the marker before its independent fence is
        // visible, otherwise abort_all_tasks can return for Terminalizing and
        // shutdown can race past this owner's terminal event/flush.
        self.register_task_terminalization(
            Arc::clone(&identity),
            Arc::clone(&completion),
            kind,
        );
        Some((identity, completion))
    }

    fn task_terminalization_matches_locked(
        active_turn: &ActiveTurn,
        identity: &Arc<()>,
        task: &RunningTask,
        kind: TaskTerminalizationKind,
    ) -> bool {
        active_turn.task_terminalization.as_ref().is_some_and(|marker| {
            marker.kind == kind
                && Arc::ptr_eq(&marker.identity, identity)
                && Arc::ptr_eq(&marker.task_identity, &task.task)
                && Arc::ptr_eq(&marker.turn_context, &task.turn_context)
                && Arc::ptr_eq(&marker.turn_state, &active_turn.turn_state)
                && marker.attach_epoch == task.attach_epoch
        })
    }

    /// Moves the exact task into its terminal owner while retaining the marker
    /// in the active slot. A stale owner can never consume a replacement task.
    fn take_task_for_terminalization_locked(
        active_turn: &mut ActiveTurn,
        identity: &Arc<()>,
        kind: TaskTerminalizationKind,
    ) -> Option<RunningTask> {
        let task = active_turn.task.as_ref()?;
        if !Self::task_terminalization_matches_locked(active_turn, identity, task, kind) {
            return None;
        }
        active_turn
            .task_terminalization
            .as_mut()
            .expect("terminalization marker matched")
            .phase = TaskTerminalizationPhase::Terminalizing;
        active_turn.task.take()
    }

    /// Clears only a terminalization marker that still owns the exact turn
    /// state and task identity. The completion fence is signalled after the
    /// slot CAS, never before it.
    fn clear_task_terminalization_if_current_locked(
        active: &mut Option<ActiveTurn>,
        identity: &Arc<()>,
        kind: TaskTerminalizationKind,
        turn_state: &Arc<Mutex<TurnState>>,
        task_identity: &Arc<dyn AnySessionTask>,
        turn_context: &Arc<TurnContext>,
        attach_epoch: u64,
    ) -> Option<Arc<StartTransitionCompletion>> {
        let Some(active_turn) = active.as_mut() else {
            return None;
        };
        let Some(marker) = active_turn.task_terminalization.as_ref() else {
            return None;
        };
        if active_turn.task.is_some()
            || active_turn.start_reservation.is_some()
            || active_turn.start_transition.is_some()
            || marker.kind != kind
            || marker.phase != TaskTerminalizationPhase::Terminalizing
            || !Arc::ptr_eq(&marker.identity, identity)
            || !Arc::ptr_eq(&marker.task_identity, task_identity)
            || !Arc::ptr_eq(&marker.turn_context, turn_context)
            || !Arc::ptr_eq(&marker.turn_state, turn_state)
            || marker.attach_epoch != attach_epoch
        {
            return None;
        }
        let completion = Arc::clone(&marker.completion);
        active_turn.task_terminalization = None;
        *active = None;
        Some(completion)
    }

    /// Claims and detaches the exact regular task for root-turn suspension.
    /// The marker remains in `active_turn` while the caller performs bounded
    /// cancellation and closes persistence, so no start/input path can treat
    /// the session as idle in that interval.
    pub(crate) async fn take_task_for_suspension(
        &self,
    ) -> Option<DetachedTaskForSuspension> {
        if self.shutdown_started() {
            return None;
        }
        let _claim_lock = self.terminalization_claim_lock.lock().await;
        let mut active = self.active_turn.lock().await;
        if self.shutdown_started() {
            return None;
        }
        let active_turn = active.as_mut()?;
        if active_turn.task_terminalization.is_some()
            || active_turn.start_reservation.is_some()
            || active_turn.start_transition.is_some()
        {
            return None;
        }
        let task_ref = active_turn.task.as_ref()?;
        if task_ref.kind != TaskKind::Regular {
            return None;
        }
        let task_identity = Arc::clone(&task_ref.task);
        let task_context = Arc::clone(&task_ref.turn_context);
        let attach_epoch = task_ref.attach_epoch;
        let turn_state = Arc::clone(&active_turn.turn_state);
        let (terminalization_identity, _terminalization_completion) =
            self.claim_task_terminalization_locked(
                active_turn,
                &task_identity,
                &task_context,
                attach_epoch,
                TaskTerminalizationKind::Suspend,
            )?;
        let task = Self::take_task_for_terminalization_locked(
            active_turn,
            &terminalization_identity,
            TaskTerminalizationKind::Suspend,
        )?;
        // Suspension owns the shutdown handoff from this point.  Seal it
        // before releasing the active-turn lock so a concurrent shutdown or
        // idle mutation cannot observe the detached slot as admissible.
        self.begin_shutdown();
        drop(active);
        Some(DetachedTaskForSuspension {
            task: Some(task),
            turn_state,
            terminalization_identity,
            task_identity,
            turn_context: task_context,
            attach_epoch,
        })
    }

    /// Releases the suspension marker only after the caller has closed the
    /// writer and completed every shutdown-side persistence step.
    pub(crate) async fn finish_task_suspension(
        &self,
        suspended: &DetachedTaskForSuspension,
    ) -> bool {
        let completion = {
            let mut active = self.active_turn.lock().await;
            Self::clear_task_terminalization_if_current_locked(
                &mut active,
                &suspended.terminalization_identity,
                TaskTerminalizationKind::Suspend,
                &suspended.turn_state,
                &suspended.task_identity,
                &suspended.turn_context,
                suspended.attach_epoch,
            )
        };
        if let Some(completion) = completion.as_ref() {
            self.finish_task_terminalization(&suspended.terminalization_identity, completion);
            true
        } else {
            false
        }
    }

    fn recovery_seed_for_task(
        task: &RunningTask,
        abort_reason: Option<&TurnAbortReason>,
    ) -> Option<RecoverySeed> {
        if !task.recovery_eligible_model_turn
            || !matches!(
                abort_reason,
                Some(TurnAbortReason::Interrupted | TurnAbortReason::BudgetLimited)
            )
        {
            return None;
        }
        let authority = task.recovery_authority.as_ref()?.clone();
        // A transition in flight is intentionally not recoverable. Waiting on
        // this mutex could delay cancellation behind a blocked rollout flush;
        // fail closed instead and let the abort path quiesce the task.
        let state = authority.state.try_lock().ok()?;
        if state.poisoned
            || state.durable_state != DurableRecoveryState::Ready
            || !authority.ready.load(Ordering::Acquire)
        {
            return None;
        }
        let generation = state.generation;
        let request_fingerprint_sha256 = state.request_fingerprint_sha256.clone()?;
        let replay = state.replay.clone()?;
        let persistence_failure_generation = state.ready_persistence_failure_generation?;
        drop(state);
        Some(RecoverySeed {
            turn_id: task.turn_context.sub_id.clone(),
            authority,
            generation,
            request_fingerprint_sha256,
            replay,
            persistence_failure_generation,
            attach_epoch: task.attach_epoch,
            confirmation_generation: None,
        })
    }

    /// Emits one terminal event exactly once and proves that both its append
    /// and the following durability barrier completed without any swallowed
    /// persistence failure. The returned generation is later matched against
    /// the exact generation bound into the task's durable Ready authority.
    async fn send_terminal_event_and_flush(
        &self,
        turn_context: &TurnContext,
        event: EventMsg,
    ) -> Option<u64> {
        let before = self.rollout_persistence_failure_generation();
        self.send_event(turn_context, event).await;
        let after_append = self.rollout_persistence_failure_generation();
        let flush_succeeded = match self.flush_rollout().await {
            Ok(()) => true,
            Err(err) => {
                warn!("failed to flush rollout after emitting terminal turn event: {err}");
                false
            }
        };
        let after_flush = self.rollout_persistence_failure_generation();
        (flush_succeeded && before == after_append && after_append == after_flush)
            .then_some(after_flush)
    }

    /// Revokes any task-owned Ready marker before controlled detach. Only an
    /// exact, quiescent, failure-free seed survives to the post-terminal
    /// InterruptedConfirmed phase; every other path remains durably Unready.
    async fn prepare_recovery_seed_for_controlled_detach(
        &self,
        turn_id: &str,
        authority: Option<&Arc<TurnRecoveryAuthority>>,
        mut recovery_seed: Option<RecoverySeed>,
    ) -> Option<RecoverySeed> {
        let Some(authority) = authority else {
            return None;
        };
        let seed_interval_is_valid = recovery_seed.as_ref().is_some_and(|seed| {
            seed.persistence_failure_generation == self.rollout_persistence_failure_generation()
        });
        let confirmation_generation = match self
            .prepare_turn_recovery_for_controlled_detach(turn_id, authority.as_ref())
            .await
        {
            Ok(generation) => generation,
            Err(err) => {
                warn!("failed to revoke turn recovery before controlled detach: {err}");
                if !self
                    .persist_turn_recovery_failure_tombstone(turn_id, authority.as_ref())
                    .await
                {
                    warn!(
                        "failed to persist the recovery tombstone after a revoke failure; \
                         recovery provenance is fail-stop/unknown"
                    );
                }
                return None;
            }
        };
        let seed = recovery_seed.as_mut()?;
        if !seed_interval_is_valid
            || confirmation_generation != seed.generation.saturating_add(1)
            || self.rollout_persistence_failure_generation() != seed.persistence_failure_generation
        {
            let mut state = authority.state.lock().await;
            authority.ready.store(false, Ordering::Release);
            state.ready_persistence_failure_generation = None;
            state.poisoned = true;
            return None;
        }
        seed.confirmation_generation = Some(confirmation_generation);
        recovery_seed
    }

    /// Publishes one live recovery candidate only after the task is quiescent
    /// and its terminal event is durable. All detach paths converge here so a
    /// stale atomic Ready bit can never outlive a generation/state transition.
    async fn publish_recovery_seed_after_terminal(
        &self,
        recovery_seed: Option<RecoverySeed>,
        task_quiesced: bool,
        terminal_persistence_generation: Option<u64>,
    ) -> bool {
        let Some(seed) = recovery_seed else {
            return false;
        };
        let Some(confirmation_generation) = seed.confirmation_generation else {
            return false;
        };
        let persistence_proven = terminal_persistence_generation.is_some_and(|generation| {
            generation == seed.persistence_failure_generation
                && self.rollout_persistence_failure_generation()
                    == seed.persistence_failure_generation
        });
        if !task_quiesced || !persistence_proven {
            self.persist_turn_recovery_failure_tombstone(&seed.turn_id, seed.authority.as_ref())
                .await;
            return false;
        }

        // Keep the same lock order as provider-boundary publication: active
        // turn first, then recovery authority state.
        let active_turn = self.active_turn.lock().await;
        if active_turn.is_some() || self.turn_epoch.load(Ordering::Acquire) != seed.attach_epoch {
            drop(active_turn);
            self.persist_turn_recovery_failure_tombstone(&seed.turn_id, seed.authority.as_ref())
                .await;
            return false;
        }
        if self.shutdown_started() {
            drop(active_turn);
            self.persist_turn_recovery_failure_tombstone(&seed.turn_id, seed.authority.as_ref())
                .await;
            return false;
        }
        if let Err(err) = self
            .confirm_interrupted_turn_recovery(
                &seed.turn_id,
                seed.authority.as_ref(),
                confirmation_generation,
                seed.persistence_failure_generation,
                &seed.request_fingerprint_sha256,
                &seed.replay,
            )
            .await
        {
            warn!("failed to confirm interrupted turn recovery: {err}");
            drop(active_turn);
            self.persist_turn_recovery_failure_tombstone(&seed.turn_id, seed.authority.as_ref())
                .await;
            return false;
        }
        if self.shutdown_started() {
            drop(active_turn);
            self.persist_turn_recovery_failure_tombstone(&seed.turn_id, seed.authority.as_ref())
                .await;
            return false;
        }
        let state = seed.authority.state.lock().await;
        let authority_invalid = state.poisoned
            || state.durable_state != DurableRecoveryState::InterruptedConfirmed
            || state.generation != confirmation_generation
            || state.ready_persistence_failure_generation.is_some()
            || state.request_fingerprint_sha256.as_deref()
                != Some(seed.request_fingerprint_sha256.as_str())
            || state.replay.as_ref() != Some(&seed.replay)
            || seed.authority.ready.load(Ordering::Acquire)
            || self.rollout_persistence_failure_generation() != seed.persistence_failure_generation;
        if authority_invalid {
            drop(state);
            drop(active_turn);
            self.persist_turn_recovery_failure_tombstone(&seed.turn_id, seed.authority.as_ref())
                .await;
            return false;
        }
        if self.shutdown_started() {
            drop(state);
            drop(active_turn);
            self.persist_turn_recovery_failure_tombstone(&seed.turn_id, seed.authority.as_ref())
                .await;
            return false;
        }
        *self
            .recovery_candidate
            .lock()
            .expect("recovery candidate mutex poisoned") = Some(RecoveryCandidate {
            turn_id: seed.turn_id,
            marker_generation: confirmation_generation,
            request_fingerprint_sha256: seed.request_fingerprint_sha256,
            replay: seed.replay,
            epoch: seed.attach_epoch,
            persistence_failure_generation: seed.persistence_failure_generation,
        });
        drop(state);
        drop(active_turn);
        true
    }

    /// Serializes the provider Ready boundary with active-turn steer
    /// acceptance. If a steer is already pending, sampling may continue but
    /// recovery remains fail-closed until that input is consumed and durable.
    #[cfg(test)]
    pub(crate) async fn mark_recovery_ready_for_sampling(
        &self,
        turn_id: &str,
        authority: &Arc<TurnRecoveryAuthority>,
        persistence_failure_baseline: u64,
        request_fingerprint_sha256: &str,
    ) -> CodexResult<RecoveryReadyForSampling> {
        let replay = codex_history::TurnRecoveryReplayV1 {
            history_boundary: self.current_recovery_history_boundary().await?,
            turn_context_sha256: "test-turn-context".to_string(),
            start: codex_history::TurnRecoveryStartState {
                final_output_json_schema: None,
                parent_turn_id: None,
                root_turn_id: Some(turn_id.to_string()),
                responses_metadata_extra: Default::default(),
            },
            environments: Vec::new(),
        };
        self.mark_recovery_ready_for_sampling_with_replay(
            turn_id,
            authority,
            persistence_failure_baseline,
            request_fingerprint_sha256,
            &replay,
        )
        .await
    }

    pub(crate) async fn mark_recovery_ready_for_sampling_with_replay(
        &self,
        turn_id: &str,
        authority: &Arc<TurnRecoveryAuthority>,
        persistence_failure_baseline: u64,
        request_fingerprint_sha256: &str,
        replay: &codex_history::TurnRecoveryReplayV1,
    ) -> CodexResult<RecoveryReadyForSampling> {
        if !self.enabled(Feature::HeptaTurnRecovery) {
            return Ok(RecoveryReadyForSampling::Ready);
        }
        let active = self.active_turn.lock().await;
        let Some(active_turn) = active.as_ref() else {
            return Ok(RecoveryReadyForSampling::Detached);
        };
        if active_turn.task_terminalization.is_some() {
            return Ok(RecoveryReadyForSampling::Detached);
        }
        let Some(task) = active_turn.task.as_ref() else {
            return Ok(RecoveryReadyForSampling::Detached);
        };
        if task.turn_context.sub_id != turn_id
            || !task
                .recovery_authority
                .as_ref()
                .is_some_and(|current| Arc::ptr_eq(current, authority))
        {
            return Ok(RecoveryReadyForSampling::Detached);
        }
        let has_pending_steer = {
            let turn_state = active_turn.turn_state.lock().await;
            !turn_state.pending_input.is_empty()
        };
        // Mailbox input is session-scoped rather than turn-state scoped. It is
        // still accepted model-visible input, so it must suppress Ready when
        // it arrives after the run loop's last drain but before dispatch.
        let has_pending_mailbox = self.input_queue.has_pending_mailbox_items().await;
        if has_pending_steer || has_pending_mailbox {
            return Ok(RecoveryReadyForSampling::PendingInput);
        }
        self.mark_turn_recovery_ready(
            turn_id,
            authority.as_ref(),
            persistence_failure_baseline,
            request_fingerprint_sha256,
            replay,
        )
        .await?;
        Ok(RecoveryReadyForSampling::Ready)
    }

    /// Revalidates attachment for a sampling generation whose provider setup
    /// is intentionally not recoverable. This path must not mint Ready, but it
    /// still must not authorize provider retry/fallback after the owning task
    /// has detached.
    pub(crate) async fn gate_unrecoverable_provider_dispatch(
        &self,
        turn_id: &str,
        authority: &Arc<TurnRecoveryAuthority>,
    ) -> RecoveryProviderOutputGate {
        let active = self.active_turn.lock().await;
        if self.shutdown_started() {
            return RecoveryProviderOutputGate::Detached;
        }
        let recovery_enabled = self.enabled(Feature::HeptaTurnRecovery);
        if active
            .as_ref()
            .is_some_and(|active_turn| active_turn.task_terminalization.is_some())
        {
            return RecoveryProviderOutputGate::Detached;
        }
        let Some(task) = active
            .as_ref()
            .and_then(|active_turn| active_turn.task.as_ref())
        else {
            return RecoveryProviderOutputGate::Detached;
        };
        if task.turn_context.sub_id != turn_id
            || (recovery_enabled
                && !task
                    .recovery_authority
                    .as_ref()
                    .is_some_and(|current| Arc::ptr_eq(current, authority)))
        {
            return RecoveryProviderOutputGate::Detached;
        }
        if !recovery_enabled {
            return RecoveryProviderOutputGate::Attached;
        }
        RecoveryProviderOutputGate::Attached
    }

    /// Serializes the first provider event consumed by the outer turn executor
    /// with controlled detach. The event may close the recovery window only
    /// while the exact task and authority are still attached; an event that
    /// lost the race to abort must not reach model-visible output persistence,
    /// tool dispatch, hooks, commands, or other product effects. Provider-policy
    /// evidence, tracing, and diagnostic telemetry belong to the transport
    /// mapper and are explicitly outside this bounded gate.
    pub(crate) async fn gate_first_provider_output(
        &self,
        turn_id: &str,
        authority: &Arc<TurnRecoveryAuthority>,
    ) -> CodexResult<RecoveryProviderOutputGate> {
        let active = self.active_turn.lock().await;
        if self.shutdown_started() {
            return Ok(RecoveryProviderOutputGate::Detached);
        }
        if active
            .as_ref()
            .is_some_and(|active_turn| active_turn.task_terminalization.is_some())
        {
            return Ok(RecoveryProviderOutputGate::Detached);
        }
        let recovery_enabled = self.enabled(Feature::HeptaTurnRecovery);
        let Some(task) = active
            .as_ref()
            .and_then(|active_turn| active_turn.task.as_ref())
        else {
            return Ok(RecoveryProviderOutputGate::Detached);
        };
        if task.turn_context.sub_id != turn_id
            || (recovery_enabled
                && !task
                    .recovery_authority
                    .as_ref()
                    .is_some_and(|current| Arc::ptr_eq(current, authority)))
        {
            return Ok(RecoveryProviderOutputGate::Detached);
        }
        if recovery_enabled {
            self.ensure_turn_recovery_unready(turn_id, authority.as_ref())
                .await?;
        }
        Ok(RecoveryProviderOutputGate::Attached)
    }

    pub async fn spawn_task<T: SessionTask>(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        input: Vec<TurnInput>,
        task: T,
    ) -> CodexResult<()> {
        if self.shutdown_started() {
            return Err(CodexErr::InvalidRequest(
                "cannot start a task while the session is shutting down".to_string(),
            ));
        }
        if self.has_pending_admission_fence() {
            return Err(CodexErr::InvalidRequest(
                "cannot start a task while the previous turn is terminalizing".to_string(),
            ));
        }
        self.abort_all_tasks(TurnAbortReason::Replaced).await;
        let start_reservation = {
            let mut active_turn = self.active_turn.lock().await;
            if self.shutdown_started()
                || self.has_pending_admission_fence()
                || active_turn.is_some()
            {
                return Err(CodexErr::InvalidRequest(
                    "cannot start replacement task while the previous turn is transitioning"
                        .to_string(),
                ));
            }
            self.consume_recovery_candidate_for_mutation().await?;
            let active = active_turn.get_or_insert_with(ActiveTurn::default);
            active
                .reserve_start(turn_context.sub_id.clone())
                .expect("idle slot should accept one start reservation")
        };
        let mut start_reservation_owner = StartReservationOwner::new(self, start_reservation);
        self.clear_connector_selection().await;
        let start_outcome = self
            .start_task_owned(
                turn_context,
                input,
                task,
                MailboxParentProvenance::Ignore,
                start_reservation_owner.handle().clone(),
            )
            .await;
        match start_outcome {
            StartTaskOutcome::Attached | StartTaskOutcome::Aborted => {
                start_reservation_owner.disarm();
                Ok(())
            }
            StartTaskOutcome::Stale => Err(CodexErr::InvalidRequest(
                "start reservation became stale before task attachment".to_string(),
            )),
        }
    }

    pub(crate) async fn start_task<T: SessionTask>(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        input: Vec<TurnInput>,
        task: T,
        mailbox_parent_provenance: MailboxParentProvenance,
    ) {
        self.start_task_with_options(
            turn_context,
            input,
            task,
            mailbox_parent_provenance,
            None,
            None,
            None,
        )
        .await;
    }

    pub(crate) async fn start_task_with_recovery<T: SessionTask>(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        input: Vec<TurnInput>,
        task: T,
        mailbox_parent_provenance: MailboxParentProvenance,
        recovery_history: Option<RecoveryHistoryTransition>,
    ) {
        self.start_task_with_options(
            turn_context,
            input,
            task,
            mailbox_parent_provenance,
            recovery_history,
            None,
            None,
        )
        .await;
    }

    pub(crate) async fn start_task_owned<T: SessionTask>(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        input: Vec<TurnInput>,
        task: T,
        mailbox_parent_provenance: MailboxParentProvenance,
        start_reservation: StartReservationHandle,
    ) -> StartTaskOutcome {
        self.start_task_with_options(
            turn_context,
            input,
            task,
            mailbox_parent_provenance,
            None,
            Some(start_reservation),
            None,
        )
        .await
    }

    pub(crate) async fn start_task_with_recovery_owned<T: SessionTask>(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        input: Vec<TurnInput>,
        task: T,
        mailbox_parent_provenance: MailboxParentProvenance,
        recovery_history: Option<RecoveryHistoryTransition>,
        start_reservation: StartReservationHandle,
    ) -> StartTaskOutcome {
        self.start_task_with_options(
            turn_context,
            input,
            task,
            mailbox_parent_provenance,
            recovery_history,
            Some(start_reservation),
            None,
        )
        .await
    }

    async fn start_task_with_options<T: SessionTask>(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        input: Vec<TurnInput>,
        task: T,
        mailbox_parent_provenance: MailboxParentProvenance,
        recovery_history: Option<RecoveryHistoryTransition>,
        start_reservation: Option<StartReservationHandle>,
        terminalization_owner: Option<Arc<()>>,
    ) -> StartTaskOutcome {
        let task: Arc<dyn AnySessionTask> = Arc::new(task);
        let task_kind = task.kind();
        // Mark the host-owned async start transition before any subsequent
        // await owned by this function.  While lifecycle contributors, input
        // preparation, or prewarm work are running there is intentionally no
        // `RunningTask` yet, so abort must target this exact continuation
        // instead of silently returning as if the session were idle.  The
        // caller-owned reservation preamble before entering `start_task`
        // remains a separate, inert reservation by design.
        let start_transition_identity;
        let transition_turn_state;
        let transition_completion;
        {
            let mut active = self.active_turn.lock().await;
            if self.shutdown_started()
                || self
                    .has_pending_admission_fence_except(terminalization_owner.as_ref())
            {
                return StartTaskOutcome::Stale;
            }
            if let Some(start_reservation) = start_reservation.as_ref() {
                let Some(turn) = active.as_mut() else {
                    warn!(turn_id = %turn_context.sub_id, "start reservation lost before promotion");
                    return StartTaskOutcome::Stale;
                };
                if start_reservation.turn_id != turn_context.sub_id
                    || !turn.promote_start(start_reservation)
                {
                    warn!(turn_id = %turn_context.sub_id, "start reservation failed identity promotion");
                    return StartTaskOutcome::Stale;
                }
                start_transition_identity = Arc::clone(
                    &turn
                        .start_transition
                        .as_ref()
                        .expect("promotion installs start transition")
                        .identity,
                );
                transition_turn_state = Arc::clone(&turn.turn_state);
                transition_completion = Arc::clone(
                    &turn
                        .start_transition
                        .as_ref()
                        .expect("promotion installs start transition")
                        .completion,
                );
            } else {
                if active.is_some() {
                    warn!(
                        turn_id = %turn_context.sub_id,
                        "refusing to steal an existing idle or caller reservation"
                    );
                    return StartTaskOutcome::Stale;
                }
                let turn = active.get_or_insert_with(ActiveTurn::default);
                start_transition_identity = Arc::new(());
                turn.start_transition = Some(StartTransition::new(
                    turn_context.sub_id.clone(),
                    Arc::clone(&start_transition_identity),
                ));
                transition_turn_state = Arc::clone(&turn.turn_state);
                transition_completion = Arc::clone(
                    &turn
                        .start_transition
                        .as_ref()
                        .expect("direct start installs start transition")
                        .completion,
                );
            }
            // Register while the active-turn lock is still held.  Shutdown
            // can begin concurrently, so publishing the marker and its
            // completion fence must be one serialization unit.
            self.register_start_transition(
                Arc::clone(&start_transition_identity),
                Arc::clone(&transition_completion),
            );
        }
        let recovery_history_restore_witness = recovery_history
            .as_ref()
            .map(|history| history.restore.clone());
        let mut start_transition_owner = StartTransitionOwner::new(
            self,
            Arc::clone(&task),
            Arc::clone(&turn_context),
            Arc::clone(&transition_turn_state),
            Arc::clone(&start_transition_identity),
            transition_completion,
            recovery_history_restore_witness,
        );
        let mut recovery_history_restore = if let Some(recovery_history) = recovery_history {
            self.install_recovery_history_snapshot(recovery_history.install)
                .await;
            Some(recovery_history.restore)
        } else {
            None
        };
        let hepta_turn_recovery_enabled = turn_context
            .config
            .features
            .enabled(Feature::HeptaTurnRecovery);
        let recovery_eligible_model_turn =
            hepta_turn_recovery_enabled && task.recovery_eligible_model_turn();
        let recovery_authority = hepta_turn_recovery_enabled
            .then(|| task.recovery_authority())
            .flatten();
        let span_name = task.span_name();
        let started_at = Instant::now();
        let turn_started_at_unix_ms = turn_context
            .turn_timing_state
            .mark_turn_started(started_at)
            .await;
        turn_context
            .turn_metadata_state
            .set_turn_started_at_unix_ms(turn_started_at_unix_ms);
        let token_usage_at_turn_start = self.total_token_usage().await.unwrap_or_default();

        let cancellation_token = CancellationToken::new();
        let done = Arc::new(Notify::new());

        self.services
            .guardian_rejection_circuit_breaker
            .lock()
            .await
            .clear_turn(&turn_context.sub_id);

        let (pending_items, parent_turn_id, root_turn_id) =
            self.input_queue.get_pending_input(&self.active_turn).await;
        // Preserve the durable client/input identity in the turn scope before
        // any qualification contributor runs.  The extension API remains
        // inert for mailbox/automatic turns and for direct inputs that lack a
        // client message id; those paths cannot honestly claim cross-spawn
        // replay.
        if let Some(identity) =
            qualification_admission_identity(self.thread_id.to_string().as_str(), &input)
        {
            // A host may have supplied an identity through the turn scope.
            // Preserve that authority instead of replacing it with a
            // newly-derived value after the scope was initialized.
            let attached = turn_context
                .extension_data
                .insert_if(identity.clone(), |existing| existing.is_none());
            if !attached
                && turn_context
                    .extension_data
                    .get::<QualificationTurnAdmissionIdentity>()
                    .is_some_and(|existing| *existing != identity)
            {
                // A mismatched pre-seeded value is not allowed to become a
                // qualification authority for this accepted input.  Remove
                // it so the extension remains inert rather than binding the
                // wrong logical turn.
                turn_context
                    .extension_data
                    .remove::<QualificationTurnAdmissionIdentity>();
            }
        } else {
            // Do not let a reused turn context carry a prior user's durable
            // identity into an automatic/mailbox/direct turn.
            turn_context
                .extension_data
                .remove::<QualificationTurnAdmissionIdentity>();
        }
        if let MailboxParentProvenance::Attribute = mailbox_parent_provenance {
            if let Some(id) = parent_turn_id {
                if let Some(initiating_agent_path) = pending_items.iter().find_map(|item| {
                    let TurnInput::InterAgentCommunication(communication) = item else {
                        return None;
                    };
                    communication
                        .trigger_turn
                        .then(|| communication.author.clone())
                }) {
                    turn_context
                        .turn_metadata_state
                        .set_initiating_agent_path(initiating_agent_path);
                }
                turn_context.turn_metadata_state.set_parent_turn_id(id);
            }
            if let Some(id) = root_turn_id {
                turn_context.turn_metadata_state.set_root_turn_id(id);
            }
        } else if pending_items.iter().any(|item| {
            matches!(
                item,
                TurnInput::InterAgentCommunication(communication) if communication.trigger_turn
            )
        }) && turn_context.turn_metadata_state.root_turn_id() != root_turn_id
        {
            turn_context.turn_metadata_state.mark_root_turn_ambiguous();
        }
        let turn_state = {
            let active = self.active_turn.lock().await;
            let Some(turn) = active.as_ref() else {
                drop(active);
                self.restore_recovery_history_if_current(
                    None,
                    &start_transition_identity,
                    &mut recovery_history_restore,
                )
                .await;
                start_transition_owner.disarm();
                return StartTaskOutcome::Stale;
            };
            if turn.task.is_some() || turn.task_terminalization.is_some() {
                drop(active);
                self.restore_recovery_history_if_current(
                    None,
                    &start_transition_identity,
                    &mut recovery_history_restore,
                )
                .await;
                start_transition_owner.disarm();
                return StartTaskOutcome::Stale;
            }
            let Some(transition) = turn.start_transition.as_ref() else {
                drop(active);
                self.restore_recovery_history_if_current(
                    None,
                    &start_transition_identity,
                    &mut recovery_history_restore,
                )
                .await;
                start_transition_owner.disarm();
                return StartTaskOutcome::Stale;
            };
            if !Arc::ptr_eq(&transition.identity, &start_transition_identity) {
                drop(active);
                self.restore_recovery_history_if_current(
                    None,
                    &start_transition_identity,
                    &mut recovery_history_restore,
                )
                .await;
                start_transition_owner.disarm();
                return StartTaskOutcome::Stale;
            }
            Arc::clone(&turn.turn_state)
        };
        turn_state.lock().await.token_usage_at_turn_start = token_usage_at_turn_start.clone();
        self.input_queue
            .extend_pending_input_for_turn_state(turn_state.as_ref(), pending_items)
            .await;
        self.emit_turn_start_lifecycle(
            turn_context.as_ref(),
            &token_usage_at_turn_start,
            task.turn_start_origin(),
        )
        .await;

        let mut active = self.active_turn.lock().await;
        let Some(turn) = active.as_mut() else {
            drop(active);
            self.restore_recovery_history_if_current(
                None,
                &start_transition_identity,
                &mut recovery_history_restore,
            )
            .await;
            start_transition_owner.disarm();
            return StartTaskOutcome::Stale;
        };
        if turn.task.is_some() || turn.task_terminalization.is_some() {
            drop(active);
            self.restore_recovery_history_if_current(
                None,
                &start_transition_identity,
                &mut recovery_history_restore,
            )
            .await;
            start_transition_owner.disarm();
            return StartTaskOutcome::Stale;
        }
        let Some(transition) = turn.start_transition.as_ref() else {
            drop(active);
            self.restore_recovery_history_if_current(
                None,
                &start_transition_identity,
                &mut recovery_history_restore,
            )
            .await;
            start_transition_owner.disarm();
            return StartTaskOutcome::Stale;
        };
        if !Arc::ptr_eq(&transition.identity, &start_transition_identity) {
            drop(active);
            self.restore_recovery_history_if_current(
                None,
                &start_transition_identity,
                &mut recovery_history_restore,
            )
            .await;
            start_transition_owner.disarm();
            return StartTaskOutcome::Stale;
        }
        if let Some(reason) = transition.abort_reason.clone() {
            // Keep the marker installed while terminalization awaits.  The
            // abort side records the reason but never emits lifecycle or
            // terminal events concurrently with an in-flight on_turn_start;
            // retaining the marker also prevents a concurrent reservation
            // clearer or replacement start from stealing this turn state.
            drop(active);
            if let Some(cleanup) = start_transition_owner.spawn_cleanup(reason) {
                if let Err(error) = cleanup.await {
                    // A panic or runtime teardown in the detached path must
                    // remain observable.  The cleanup CAS is intentionally
                    // fail-closed, so an errored join cannot make this turn
                    // look idle or permit a replacement start.
                    warn!(
                        turn_id = %turn_context.sub_id,
                        ?error,
                        "start transition terminalizer did not complete"
                    );
                }
            }
            return StartTaskOutcome::Aborted;
        }
        let agent_execution_guard = self.services.agent_control.execution_guard(
            turn_context.multi_agent_version,
            &turn_context.session_source,
        );
        let done_clone = Arc::clone(&done);
        let session = Arc::clone(self);
        let ctx = Arc::clone(&turn_context);
        let task_for_run = Arc::clone(&task);
        let task_input = input;
        let task_cancellation_token = cancellation_token.child_token();
        // Task-owned turn spans keep a core-owned span open for the
        // full task lifecycle after the submission dispatch span ends.
        let reasoning_effort = turn_context.effective_reasoning_effort_for_tracing();
        let task_span = info_span!(
            "turn",
            otel.name = span_name,
            thread.id = %self.thread_id,
            turn.id = %turn_context.sub_id,
            model = %turn_context.model_info.slug,
            codex.turn.reasoning_effort = %reasoning_effort,
            codex.turn.token_usage.input_tokens = field::Empty,
            codex.turn.token_usage.cached_input_tokens = field::Empty,
            codex.turn.token_usage.cache_write_input_tokens = field::Empty,
            codex.turn.token_usage.non_cached_input_tokens = field::Empty,
            codex.turn.token_usage.output_tokens = field::Empty,
            codex.turn.token_usage.reasoning_output_tokens = field::Empty,
            codex.turn.token_usage.total_tokens = field::Empty,
        );
        let handle = tokio::spawn(
            async move {
                let ctx_for_finish = Arc::clone(&ctx);
                // Enforce the host-owned gate for every task kind at the
                // common spawn boundary.  RegularTask repeats the check
                // before TurnStarted; this common check also protects review,
                // compact, and shell tasks if a gate is ever attached there.
                let task_result = AssertUnwindSafe(
                    async {
                        if let Some(gate) = ctx
                            .extension_data
                            .get::<codex_extension_api::TurnStartGate>()
                            && !gate.is_allowed()
                        {
                            return Err(CodexErr::TurnAborted);
                        }
                        task_for_run
                            .run(
                                Arc::clone(&session),
                                ctx,
                                task_input,
                                task_cancellation_token.child_token(),
                            )
                            .await
                    }
                    .instrument(trace_span!("session_task.run")),
                )
                .catch_unwind()
                .await;
                let task_result = match task_result {
                    Ok(result) => result,
                    Err(_) => {
                        // A panic in a SessionTask must still pass through the
                        // ordinary terminalization path. The RunningTask is
                        // already attached to `active_turn`; letting the
                        // JoinHandle unwind would otherwise strand that task,
                        // its recovery authority, and every later admission.
                        warn!(
                            turn_id = %ctx_for_finish.sub_id,
                            "session task panicked; converting panic to a terminal error"
                        );
                        Err(CodexErr::Fatal("session task panicked".to_string()))
                    }
                };
                let sess = Arc::clone(&session);
                if !task_cancellation_token.is_cancelled() {
                    // Finish uniformly from the spawn site so all tasks share the same lifecycle.
                    sess.on_task_finished(Arc::clone(&ctx_for_finish), task_result)
                        .await;
                }
                done_clone.notify_waiters();
            }
            .instrument(task_span),
        );
        let timer = turn_context
            .session_telemetry
            .start_timer(TURN_E2E_DURATION_METRIC, &[])
            .ok();
        // Attaching any task invalidates an older recovery token and mints the
        // epoch while the active-turn critical section is still held.
        *self
            .recovery_candidate
            .lock()
            .expect("recovery candidate mutex poisoned") = None;
        let attach_epoch = self.turn_epoch.fetch_add(1, Ordering::AcqRel) + 1;
        let running_task = RunningTask {
            done,
            handle: AbortOnDropHandle::new(handle),
            kind: task_kind,
            recovery_eligible_model_turn,
            recovery_authority,
            attach_epoch,
            task,
            cancellation_token,
            turn_context: Arc::clone(&turn_context),
            _agent_execution_guard: agent_execution_guard,
            _diagnostics_guard: ACTIVE_TURNS.track(),
            _timer: timer,
        };
        turn.task = Some(running_task);
        turn.start_transition = None;
        start_transition_owner.disarm();
        StartTaskOutcome::Attached
    }

    /// Returns whether an extension has marked this thread as durably asleep.
    pub(crate) fn has_outstanding_durable_sleep(&self) -> bool {
        self.services
            .thread_extension_data
            .get::<codex_extension_items::sleep::SleepItem>()
            .is_some()
    }

    /// Starts a regular turn when the session is idle and pending work is waiting.
    ///
    /// Pending work includes mailbox mail marked with `trigger_turn`, or any mailbox mail while
    /// an outstanding durable sleep is attached to the thread.
    ///
    /// This helper generates a fresh sub-id for the synthetic turn before delegating to the
    /// explicit-sub-id variant.
    pub(crate) fn maybe_start_turn_for_pending_work(self: &Arc<Self>) -> BoxFuture<'static, ()> {
        let session = Arc::clone(self);
        Box::pin(async move {
            session
                .maybe_start_turn_for_pending_work_with_sub_id_and_owner(
                    uuid::Uuid::new_v4().to_string(),
                    None,
                )
                .await;
        })
    }

    pub(crate) fn maybe_start_turn_for_pending_work_after_terminalization(
        self: &Arc<Self>,
        terminalization_owner: Arc<()>,
    ) -> BoxFuture<'static, ()> {
        let session = Arc::clone(self);
        Box::pin(async move {
            session
                .maybe_start_turn_for_pending_work_with_sub_id_and_owner(
                    uuid::Uuid::new_v4().to_string(),
                    Some(terminalization_owner),
                )
                .await;
        })
    }

    /// Starts a regular turn with the provided sub-id when pending work should wake an idle
    /// session.
    ///
    /// The turn is created only when the session is idle and mailbox mail either requests a turn
    /// or can wake an outstanding durable sleep.
    pub(crate) async fn maybe_start_turn_for_pending_work_with_sub_id(
        self: &Arc<Self>,
        sub_id: String,
    ) {
        self.maybe_start_turn_for_pending_work_with_sub_id_and_owner(sub_id, None)
            .await;
    }

    async fn maybe_start_turn_for_pending_work_with_sub_id_and_owner(
        self: &Arc<Self>,
        sub_id: String,
        terminalization_owner: Option<Arc<()>>,
    ) {
        if self.shutdown_started()
            || self
                .has_pending_admission_fence_except(terminalization_owner.as_ref())
        {
            return;
        }
        if !self.input_queue.has_pending_mailbox_items().await
            || (!self.input_queue.has_trigger_turn_mailbox_items().await
                && !self.has_outstanding_durable_sleep())
        {
            return;
        }

        let start_reservation = {
            let mut active_turn = self.active_turn.lock().await;
            if self.shutdown_started()
                || self
                    .has_pending_admission_fence_except(terminalization_owner.as_ref())
                || active_turn.is_some()
            {
                return;
            }
            if self.enabled(Feature::HeptaTurnRecovery)
                && self
                    .recovery_candidate
                    .lock()
                    .expect("recovery candidate mutex poisoned")
                    .is_some()
            {
                // Durable queued work must wait for explicit recovery or an
                // explicit user action that consumes the interrupted tail.
                return;
            }
            let active = active_turn.get_or_insert_with(ActiveTurn::default);
            active
                .reserve_start(sub_id.clone())
                .expect("idle slot should accept one start reservation")
        };

        let mut start_reservation_owner = StartReservationOwner::new(self, start_reservation);
        let turn_context = self.new_default_turn_with_sub_id(sub_id).await;
        self.maybe_emit_model_warnings_for_turn(turn_context.as_ref())
            .await;
        let start_outcome = self
            .start_task_with_options(
                turn_context,
                Vec::new(),
                RegularTask::new(TurnRunOrigin::NewTurn),
                MailboxParentProvenance::Attribute,
                None,
                Some(start_reservation_owner.handle().clone()),
                terminalization_owner,
            )
            .await;
        if start_outcome != StartTaskOutcome::Stale {
            start_reservation_owner.disarm();
        }
    }

    /// Runs abort terminalization in a detached job. Dropping the caller's
    /// future after the task-terminalization claim must not drop the owner
    /// body; Tokio JoinHandle drop detaches the job and the registry fence
    /// remains the teardown backstop if the job itself fails.
    pub async fn abort_all_tasks(self: &Arc<Self>, reason: TurnAbortReason) {
        let session = Arc::clone(self);
        let join = self.services.runtime_handle.spawn(
            async move {
                let result = AssertUnwindSafe(session.abort_all_tasks_inner(reason))
                    .catch_unwind()
                    .await;
                if result.is_err() {
                    warn!("abort-all terminalizer panicked; retaining its completion fence");
                }
            }
            .in_current_span(),
        );
        if let Err(error) = join.await {
            warn!(?error, "abort-all terminalizer did not complete");
        }
    }

    async fn abort_all_tasks_inner(self: &Arc<Self>, reason: TurnAbortReason) {
        let mut aborted_turn = false;
        let mut reserved_turn_state = None;
        let mut turn_context = None;
        let mut terminalization_identity = None;
        let mut task_identity = None;
        let mut task_context = None;
        let mut task_epoch = None;
        let mut cleared_completion = None;
        let mut abort_outcome = TaskAbortOutcome::default();
        if let Some(target) = self
            .detach_active_task_for_abort(
                &reason, /*expected_turn_id*/ None, /*expected_turn_state*/ None,
                /*deferred_idle_cause*/ None,
            )
            .await
        {
            let detached = match target {
                ActiveTurnAbortTarget::Running(detached) => detached,
                ActiveTurnAbortTarget::Terminalizing => {
                    // A finish/abort owner already has the exact task. The
                    // ordinary replacement path must remain non-blocking: a
                    // second abort cannot drive the owner's cleanup and
                    // waiting here would deadlock callers that are expected
                    // to release the owner's test/abort latch.
                    return;
                }
                ActiveTurnAbortTarget::Starting { .. } => {
                    // The start owner performs terminalization after its
                    // in-flight lifecycle callback returns. Finalizing here
                    // would race on_turn_start/on_turn_abort ordering. The
                    // shutdown handler performs the separate completion drain.
                    return;
                }
            };
            aborted_turn = true;
            turn_context = Some(Arc::clone(&detached.task.turn_context));
            reserved_turn_state = Some(Arc::clone(&detached.turn_state));
            terminalization_identity = Some(Arc::clone(&detached.terminalization_identity));
            task_identity = Some(Arc::clone(&detached.task.task));
            task_context = Some(Arc::clone(&detached.task.turn_context));
            task_epoch = Some(detached.task.attach_epoch);
            abort_outcome = self
                .handle_task_abort(
                    detached.task,
                    reason.clone(),
                    detached.recovery_seed,
                    detached.recovery_authority,
                )
                .await;
        }

        if let Some(turn_context) = turn_context.as_deref() {
            self.emit_turn_abort_lifecycle(reason.clone(), turn_context.extension_data.as_ref())
                .await;
        }
        if let (
            Some(turn_state),
            Some(terminalization_identity),
            Some(task_identity),
            Some(task_context),
            Some(task_epoch),
        ) = (
            reserved_turn_state.as_ref(),
            terminalization_identity.as_ref(),
            task_identity.as_ref(),
            task_context.as_ref(),
            task_epoch,
        ) {
            // Let interrupted tasks observe cancellation before dropping pending approvals, or an
            // in-flight approval wait can surface as a model-visible rejection before TurnAborted.
            self.input_queue
                .clear_pending_for_turn_state(turn_state.as_ref())
                .await;
            let mut active = self.active_turn.lock().await;
            cleared_completion = Self::clear_task_terminalization_if_current_locked(
                &mut active,
                terminalization_identity,
                TaskTerminalizationKind::Abort,
                turn_state,
                task_identity,
                task_context,
                task_epoch,
            );
        }
        self.publish_recovery_seed_after_terminal(
            abort_outcome.recovery_seed,
            abort_outcome.task_quiesced,
            abort_outcome.terminal_persistence_generation,
        )
        .await;
        if let (Some(identity), Some(completion)) =
            (terminalization_identity.as_ref(), cleared_completion.as_ref())
        {
            if reason == TurnAbortReason::Interrupted && aborted_turn {
                self.maybe_start_turn_for_pending_work_after_terminalization(Arc::clone(identity))
                    .await;
            }
            self.finish_task_terminalization(identity, completion);
        }
    }

    /// Drain a materialized start transition during session teardown.
    ///
    /// Ordinary `abort_all_tasks` callers intentionally return immediately for
    /// `Starting`: a lifecycle contributor may be waiting on the caller that
    /// requested replacement/steer.  Teardown has a stronger ordering
    /// obligation, so its handler calls this dedicated drain before emitting
    /// thread-stop lifecycle.  A terminalizer that panics, hangs, or cannot be
    /// scheduled leaves the completion fence unresolved and therefore keeps
    /// teardown fail-closed instead of publishing thread-stop out of order.
    pub(crate) async fn drain_start_transition_for_shutdown(self: &Arc<Self>) {
        loop {
            let completions = self.pending_start_transition_completions();
            if completions.is_empty() {
                return;
            }
            for completion in completions {
                completion.wait().await;
            }
        }
    }

    pub(crate) async fn abort_turn_if_active(
        self: &Arc<Self>,
        turn_id: &str,
        reason: TurnAbortReason,
    ) -> bool {
        !matches!(
            self.abort_turn_if_active_impl(
                turn_id, /*expected_turn_state*/ None, reason,
                /*deferred_idle_cause*/ None,
            )
            .await,
            AbortTurnOutcome::NotActive
        )
    }

    /// Guardian-specific abort binding. The state identity is captured with
    /// the review request, so a delayed callback cannot target a later attempt
    /// that happens to reuse the same protocol turn id. A start transition
    /// owns the deferred idle callback; the guardian caller must not emit one
    /// for `DeferredStart` or it could race and double-publish.
    pub(crate) async fn abort_turn_if_active_for_guardian(
        self: &Arc<Self>,
        turn_id: &str,
        expected_turn_state: &Arc<Mutex<TurnState>>,
        reason: TurnAbortReason,
    ) -> AbortTurnOutcome {
        self.abort_turn_if_active_impl(
            turn_id,
            Some(expected_turn_state),
            reason,
            Some(ThreadIdleCause::Interrupted),
        )
        .await
    }

    async fn abort_turn_if_active_impl(
        self: &Arc<Self>,
        turn_id: &str,
        expected_turn_state: Option<&Arc<Mutex<TurnState>>>,
        reason: TurnAbortReason,
        deferred_idle_cause: Option<ThreadIdleCause>,
    ) -> AbortTurnOutcome {
        let session = Arc::clone(self);
        let turn_id = turn_id.to_string();
        let expected_turn_state = expected_turn_state.cloned();
        let join = self.services.runtime_handle.spawn(
            async move {
                let result = AssertUnwindSafe(async {
                    let outcome = session
                        .abort_turn_if_active_inner(
                            &turn_id,
                            expected_turn_state.as_ref(),
                            reason,
                            deferred_idle_cause,
                        )
                        .await;
                    if matches!(outcome, AbortTurnOutcome::Running)
                        && let Some(cause) = deferred_idle_cause
                    {
                        // Guardian's running-task abort bypasses the ordinary finish
                        // lifecycle. Keep this callback inside the detached owner so
                        // cancellation of the guardian caller cannot lose it.
                        session.emit_thread_idle_lifecycle_if_idle(cause).await;
                    }
                    outcome
                })
                .catch_unwind()
                .await;
                match result {
                    Ok(outcome) => outcome,
                    Err(_) => {
                        warn!(
                            %turn_id,
                            "abort terminalizer panicked; retaining its completion fence"
                        );
                        AbortTurnOutcome::Terminalizing
                    }
                }
            }
            .in_current_span(),
        );
        match join.await {
            Ok(outcome) => outcome,
            Err(error) => {
                warn!(?error, "abort terminalizer did not complete");
                // The detached job may have claimed the marker before
                // panicking. Keep callers fail-closed rather than reporting a
                // false idle/not-active result.
                AbortTurnOutcome::Terminalizing
            }
        }
    }

    async fn abort_turn_if_active_inner(
        self: &Arc<Self>,
        turn_id: &str,
        expected_turn_state: Option<&Arc<Mutex<TurnState>>>,
        reason: TurnAbortReason,
        deferred_idle_cause: Option<ThreadIdleCause>,
    ) -> AbortTurnOutcome {
        let Some(target) = self
            .detach_active_task_for_abort(
                &reason,
                Some(turn_id),
                expected_turn_state,
                deferred_idle_cause,
            )
            .await
        else {
            return AbortTurnOutcome::NotActive;
        };
        let detached = match target {
            ActiveTurnAbortTarget::Running(detached) => detached,
            ActiveTurnAbortTarget::Terminalizing => {
                return AbortTurnOutcome::Terminalizing;
            }
            ActiveTurnAbortTarget::Starting { deferred_idle, .. } => {
                return if deferred_idle {
                    AbortTurnOutcome::DeferredStart
                } else {
                    AbortTurnOutcome::Starting
                };
            }
        };

        let turn_context = Arc::clone(&detached.task.turn_context);
        let reserved_turn_state = Arc::clone(&detached.turn_state);
        let terminalization_identity = Arc::clone(&detached.terminalization_identity);
        let task_identity = Arc::clone(&detached.task.task);
        let task_epoch = detached.task.attach_epoch;
        let abort_outcome = self
            .handle_task_abort(
                detached.task,
                reason.clone(),
                detached.recovery_seed,
                detached.recovery_authority,
            )
            .await;
        self.emit_turn_abort_lifecycle(reason.clone(), turn_context.extension_data.as_ref())
            .await;
        // Let interrupted tasks observe cancellation before dropping pending approvals, or an
        // in-flight approval wait can surface as a model-visible rejection before TurnAborted.
        self.input_queue
            .clear_pending_for_turn_state(reserved_turn_state.as_ref())
            .await;
        let mut active = self.active_turn.lock().await;
        let cleared_completion = Self::clear_task_terminalization_if_current_locked(
            &mut active,
            &terminalization_identity,
            TaskTerminalizationKind::Abort,
            &reserved_turn_state,
            &task_identity,
            &turn_context,
            task_epoch,
        );
        drop(active);

        self.publish_recovery_seed_after_terminal(
            abort_outcome.recovery_seed,
            abort_outcome.task_quiesced,
            abort_outcome.terminal_persistence_generation,
        )
        .await;

        if let Some(completion) = cleared_completion.as_ref() {
            if reason == TurnAbortReason::Interrupted {
                self.maybe_start_turn_for_pending_work_after_terminalization(
                    Arc::clone(&terminalization_identity),
                )
                .await;
            }
            self.finish_task_terminalization(&terminalization_identity, completion);
        }

        AbortTurnOutcome::Running
    }

    /// Keeps the task's terminal owner alive if the task runner/caller is
    /// cancelled while recovery, persistence, or lifecycle callbacks await.
    /// The inner job owns the exact task context and marker CAS; dropping this
    /// JoinHandle detaches it rather than aborting the terminalizer.
    pub async fn on_task_finished(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        task_result: SessionTaskResult,
    ) {
        let session = Arc::clone(self);
        let join = self.services.runtime_handle.spawn(
            async move {
                let result =
                    AssertUnwindSafe(session.on_task_finished_inner(turn_context, task_result))
                        .catch_unwind()
                        .await;
                if result.is_err() {
                    warn!("finish terminalizer panicked; retaining its completion fence");
                }
            }
            .in_current_span(),
        );
        if let Err(error) = join.await {
            warn!(?error, "task-finish terminalizer did not complete");
        }
    }

    async fn on_task_finished_inner(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        task_result: SessionTaskResult,
    ) {
        let abort_reason_hint = match &task_result {
            Err(err) if matches!(err.details(), CodexErrorDetails::TurnAborted) => {
                Some(TurnAbortReason::Interrupted)
            }
            Ok(_) | Err(_) => None,
        };
        // Claim the exact task before the first recovery await. The active
        // slot remains occupied by the typed marker, while the lock itself is
        // released so unrelated state readers and shutdown can make progress.
        let Some((
            terminalization_identity,
            turn_state,
            recovery_seed,
            recovery_authority,
        )) = ({
            let _claim_lock = self.terminalization_claim_lock.lock().await;
            let mut active = self.active_turn.lock().await;
            let Some(active_turn) = active.as_mut() else {
                return;
            };
            let Some(task) = active_turn.task.as_ref() else {
                return;
            };
            if !Arc::ptr_eq(&task.turn_context, &turn_context)
                || active_turn.task_terminalization.is_some()
            {
                return;
            }
            let recovery_seed = Self::recovery_seed_for_task(task, abort_reason_hint.as_ref());
            let recovery_authority = task.recovery_authority.clone();
            let task_identity = Arc::clone(&task.task);
            let task_context = Arc::clone(&task.turn_context);
            let task_epoch = task.attach_epoch;
            let turn_state = Arc::clone(&active_turn.turn_state);
            let (terminalization_identity, _terminalization_completion) =
                self.claim_task_terminalization_locked(
                    active_turn,
                    &task_identity,
                    &task_context,
                    task_epoch,
                    TaskTerminalizationKind::Finish,
                )
                .expect("task terminalization claim should be unique");
            Some((
                terminalization_identity,
                turn_state,
                recovery_seed,
                recovery_authority,
            ))
        }) else {
            return;
        };
        let recovery_seed = self
            .prepare_recovery_seed_for_controlled_detach(
                &turn_context.sub_id,
                recovery_authority.as_ref(),
                recovery_seed,
            )
            .await;
        let task = {
            let mut active = self.active_turn.lock().await;
            let Some(active_turn) = active.as_mut() else {
                warn!(
                    turn_id = %turn_context.sub_id,
                    "finish terminalizer lost active slot after recovery preparation; retaining fence"
                );
                return;
            };
            let Some(task_ref) = active_turn.task.as_ref() else {
                warn!(
                    turn_id = %turn_context.sub_id,
                    "finish terminalizer lost task after recovery preparation; retaining fence"
                );
                return;
            };
            if !Arc::ptr_eq(&task_ref.turn_context, &turn_context)
                || !Self::task_terminalization_matches_locked(
                    active_turn,
                    &terminalization_identity,
                    task_ref,
                    TaskTerminalizationKind::Finish,
                )
            {
                warn!(
                    turn_id = %turn_context.sub_id,
                    "finish terminalizer identity changed after recovery preparation; retaining fence"
                );
                return;
            }
            Self::take_task_for_terminalization_locked(
                active_turn,
                &terminalization_identity,
                TaskTerminalizationKind::Finish,
            )
            .expect("claimed finish task should remain attached")
        };
        turn_context
            .turn_metadata_state
            .cancel_git_enrichment_task();
        task.handle.detach();
        if let Err(err) = self.flush_rollout().await {
            warn!("failed to flush rollout before completing turn: {err}");
            self.send_event(
                turn_context.as_ref(),
                EventMsg::Warning(WarningEvent {
                    message: format!(
                        "Failed to save the conversation transcript; Codex will continue retrying. Error: {err}"
                    ),
                }),
            )
            .await;
        }
        let (last_agent_message, abort_reason) = match task_result {
            Ok(last_agent_message) => (last_agent_message, None),
            Err(err) if matches!(err.details(), CodexErrorDetails::TurnAborted) => {
                (None, Some(TurnAbortReason::Interrupted))
            }
            Err(err) => {
                warn!(%err, "session task returned an unexpected error");
                self.emit_turn_error_lifecycle(
                    turn_context.as_ref(),
                    err.to_codex_protocol_error(),
                )
                .await;
                self.track_turn_codex_error(turn_context.as_ref(), &err);
                self.send_event(
                    turn_context.as_ref(),
                    EventMsg::Error(err.to_error_event(/*message_prefix*/ None)),
                )
                .await;
                (None, None)
            }
        };
        let pending_input = self
            .input_queue
            .take_pending_input_for_turn_state(turn_state.as_ref())
            .await;
        let (turn_had_memory_citation, turn_tool_calls, token_usage_at_turn_start) = {
            let ts = turn_state.lock().await;
            (
                ts.has_memory_citation,
                ts.tool_calls,
                ts.token_usage_at_turn_start.clone(),
            )
        };
        run_hooks_and_record_inputs(
            self,
            &turn_context,
            &pending_input,
            PersistContext::Standard,
        )
        .await;
        let task_ended_before_persistence = self
            .pending_user_message_admissions
            .complete_task_end(&turn_context.sub_id);
        // Emit token usage metrics.
        {
            // TODO(jif): drop this
            let tmp_mem = (
                "tmp_mem_enabled",
                if self.enabled(Feature::MemoryTool) {
                    "true"
                } else {
                    "false"
                },
            );
            let network_proxy = self.services.network_proxy.load_full();
            let network_proxy_active = match network_proxy.as_ref() {
                Some(started_network_proxy) => {
                    match started_network_proxy.proxy().current_cfg().await {
                        Ok(config) => config.enabled,
                        Err(err) => {
                            warn!(
                                "failed to read managed network proxy state for turn metrics: {err:#}"
                            );
                            false
                        }
                    }
                }
                None => false,
            };
            emit_turn_network_proxy_metric(
                &self.services.session_telemetry,
                network_proxy_active,
                tmp_mem,
            );
            self.services.session_telemetry.histogram(
                TURN_TOOL_CALL_METRIC,
                i64::try_from(turn_tool_calls).unwrap_or(i64::MAX),
                &[tmp_mem],
            );
            let total_token_usage = self.total_token_usage().await.unwrap_or_default();
            let turn_token_usage = TokenUsage {
                input_tokens: (total_token_usage.input_tokens
                    - token_usage_at_turn_start.input_tokens)
                    .max(0),
                cached_input_tokens: (total_token_usage.cached_input_tokens
                    - token_usage_at_turn_start.cached_input_tokens)
                    .max(0),
                cache_write_input_tokens: (total_token_usage.cache_write_input_tokens
                    - token_usage_at_turn_start.cache_write_input_tokens)
                    .max(0),
                output_tokens: (total_token_usage.output_tokens
                    - token_usage_at_turn_start.output_tokens)
                    .max(0),
                reasoning_output_tokens: (total_token_usage.reasoning_output_tokens
                    - token_usage_at_turn_start.reasoning_output_tokens)
                    .max(0),
                total_tokens: (total_token_usage.total_tokens
                    - token_usage_at_turn_start.total_tokens)
                    .max(0),
                codex_rollout_budget_units: None,
            };
            let current_span = Span::current();
            current_span.record(
                "codex.turn.token_usage.input_tokens",
                turn_token_usage.input_tokens,
            );
            current_span.record(
                "codex.turn.token_usage.cached_input_tokens",
                turn_token_usage.cached_input(),
            );
            current_span.record(
                "codex.turn.token_usage.cache_write_input_tokens",
                turn_token_usage.cache_write_input_tokens,
            );
            current_span.record(
                "codex.turn.token_usage.non_cached_input_tokens",
                turn_token_usage.non_cached_input(),
            );
            current_span.record(
                "codex.turn.token_usage.output_tokens",
                turn_token_usage.output_tokens,
            );
            current_span.record(
                "codex.turn.token_usage.reasoning_output_tokens",
                turn_token_usage.reasoning_output_tokens,
            );
            current_span.record(
                "codex.turn.token_usage.total_tokens",
                turn_token_usage.total_tokens,
            );
            self.services
                .analytics_events_client
                .track_turn_token_usage(TurnTokenUsageFact {
                    turn_id: turn_context.sub_id.clone(),
                    thread_id: self.thread_id.to_string(),
                    token_usage: turn_token_usage.clone(),
                });
            self.services.session_telemetry.histogram(
                TURN_TOKEN_USAGE_METRIC,
                turn_token_usage.total_tokens,
                &[("token_type", "total"), tmp_mem],
            );
            self.services.session_telemetry.histogram(
                TURN_TOKEN_USAGE_METRIC,
                turn_token_usage.input_tokens,
                &[("token_type", "input"), tmp_mem],
            );
            self.services.session_telemetry.histogram(
                TURN_TOKEN_USAGE_METRIC,
                turn_token_usage.cached_input(),
                &[("token_type", "cached_input"), tmp_mem],
            );
            self.services.session_telemetry.histogram(
                TURN_TOKEN_USAGE_METRIC,
                turn_token_usage.cache_write_input_tokens,
                &[("token_type", "cache_write_input"), tmp_mem],
            );
            self.services.session_telemetry.histogram(
                TURN_TOKEN_USAGE_METRIC,
                turn_token_usage.output_tokens,
                &[("token_type", "output"), tmp_mem],
            );
            self.services.session_telemetry.histogram(
                TURN_TOKEN_USAGE_METRIC,
                turn_token_usage.reasoning_output_tokens,
                &[("token_type", "reasoning_output"), tmp_mem],
            );
        }
        emit_turn_memory_metric(
            &self.services.session_telemetry,
            turn_context.config.features.enabled(Feature::MemoryTool),
            turn_context.config.memories.use_memories,
            turn_had_memory_citation,
        );
        self.services.session_telemetry.counter(
            TURN_UNIFIED_EXEC_RUNNING_PROCESSES_METRIC,
            i64::try_from(self.list_background_terminals().await.len()).unwrap_or(i64::MAX),
            &[],
        );
        let started_at = turn_context.turn_timing_state.started_at_unix_secs().await;
        let (completed_at, duration_ms, profile) = turn_context
            .turn_timing_state
            .complete_profile_and_duration_ms()
            .await;
        self.services
            .analytics_events_client
            .track_turn_profile(TurnProfileFact {
                turn_id: turn_context.sub_id.clone(),
                profile,
            });
        let idle_cause = if matches!(
            abort_reason.as_ref(),
            Some(TurnAbortReason::Interrupted | TurnAbortReason::BudgetLimited)
        ) {
            ThreadIdleCause::Interrupted
        } else if task_ended_before_persistence
            || (abort_reason.is_none() && turn_context.terminal_error.lock().await.is_some())
        {
            ThreadIdleCause::Failed
        } else {
            ThreadIdleCause::Completed
        };
        let event = if let Some(reason) = abort_reason {
            self.emit_turn_abort_lifecycle(reason.clone(), turn_context.extension_data.as_ref())
                .await;
            EventMsg::TurnAborted(TurnAbortedEvent {
                turn_id: Some(turn_context.sub_id.clone()),
                reason,
                started_at,
                completed_at,
                duration_ms,
            })
        } else {
            let time_to_first_token_ms = turn_context
                .turn_timing_state
                .time_to_first_token_ms()
                .await;
            let error = turn_context.terminal_error.lock().await.clone();
            self.emit_turn_stop_lifecycle(turn_context.extension_data.as_ref())
                .await;
            EventMsg::TurnComplete(TurnCompleteEvent {
                turn_id: turn_context.sub_id.clone(),
                last_agent_message,
                error,
                started_at,
                completed_at,
                duration_ms,
                time_to_first_token_ms,
            })
        };
        let terminal_persistence_generation = self
            .send_terminal_event_and_flush(turn_context.as_ref(), event)
            .await;
        self.services
            .guardian_rejection_circuit_breaker
            .lock()
            .await
            .clear_turn(&turn_context.sub_id);

        let cleared_completion = {
            let mut active = self.active_turn.lock().await;
            Self::clear_task_terminalization_if_current_locked(
                &mut active,
                &terminalization_identity,
                TaskTerminalizationKind::Finish,
                &turn_state,
                &task.task,
                &turn_context,
                task.attach_epoch,
            )
        };
        self.publish_recovery_seed_after_terminal(
            recovery_seed,
            /*task_quiesced*/ true,
            terminal_persistence_generation,
        )
        .await;
        if cleared_completion.is_some() {
            if let Some(completion) = cleared_completion.as_ref() {
                self.emit_thread_idle_lifecycle_if_idle_for_terminalization(
                    idle_cause,
                    Some(&terminalization_identity),
                )
                .await;
                self.maybe_start_turn_for_pending_work_after_terminalization(
                    Arc::clone(&terminalization_identity),
                )
                .await;
                self.finish_task_terminalization(&terminalization_identity, completion);
            }
        }
    }

    /// Revokes task-owned recovery authority while the active slot is locked,
    /// then either detaches a running task or fences a caller reservation / the
    /// host-owned start transition. Starts and injections therefore cannot
    /// observe an idle session until the old task is quiescent and its terminal
    /// is durable (or the start owner has completed its deferred handoff).
    async fn detach_active_task_for_abort(
        &self,
        reason: &TurnAbortReason,
        expected_turn_id: Option<&str>,
        expected_turn_state: Option<&Arc<Mutex<TurnState>>>,
        deferred_idle_cause: Option<ThreadIdleCause>,
    ) -> Option<ActiveTurnAbortTarget> {
        let (
            terminalization_identity,
            turn_state,
            task_context,
            recovery_seed,
            recovery_authority,
            attach_epoch,
        ) = {
            let _claim_lock = self.terminalization_claim_lock.lock().await;
            let mut active = self.active_turn.lock().await;
            let active_turn = active.as_mut()?;
            if expected_turn_state
                .is_some_and(|expected| !Arc::ptr_eq(&active_turn.turn_state, expected))
            {
                return None;
            }
            if let Some(marker) = active_turn.task_terminalization.as_ref() {
                if expected_turn_id.is_some_and(|expected| marker.turn_context.sub_id != expected)
                {
                    return None;
                }
                return Some(ActiveTurnAbortTarget::Terminalizing);
            }
            let Some(task) = active_turn.task.as_ref() else {
                if let Some(transition) = active_turn.start_transition.as_mut() {
                    if expected_turn_id.is_some_and(|expected| transition.turn_id != expected) {
                        return None;
                    }
                    let accepted = transition.request_abort(reason.clone());
                    if accepted {
                        if matches!(
                            reason,
                            TurnAbortReason::Interrupted | TurnAbortReason::BudgetLimited
                        ) {
                            self.mark_interrupted();
                        }
                        if let Some(cause) = deferred_idle_cause {
                            transition.request_deferred_idle(cause);
                        }
                    }
                    return Some(ActiveTurnAbortTarget::Starting {
                        deferred_idle: accepted && deferred_idle_cause.is_some(),
                    });
                }
                if let Some(reservation) = active_turn.start_reservation.as_mut() {
                    if expected_turn_id.is_some_and(|expected| reservation.turn_id != expected) {
                        return None;
                    }
                    if reservation.request_abort(reason.clone())
                        && matches!(
                            reason,
                            TurnAbortReason::Interrupted | TurnAbortReason::BudgetLimited
                        )
                    {
                        self.mark_interrupted();
                    }
                    return Some(ActiveTurnAbortTarget::Starting {
                        deferred_idle: false,
                    });
                }
                return None;
            };
            if expected_turn_id.is_some_and(|expected| task.turn_context.sub_id != expected) {
                return None;
            }
            if matches!(
                reason,
                TurnAbortReason::Interrupted | TurnAbortReason::BudgetLimited
            ) {
                self.mark_interrupted();
            }
            let recovery_seed = Self::recovery_seed_for_task(task, Some(reason));
            let recovery_authority = task.recovery_authority.clone();
            let task_context = Arc::clone(&task.turn_context);
            let task_identity = Arc::clone(&task.task);
            let turn_state = Arc::clone(&active_turn.turn_state);
            let attach_epoch = task.attach_epoch;
            let (terminalization_identity, _terminalization_completion) =
                self.claim_task_terminalization_locked(
                active_turn,
                &task_identity,
                &task_context,
                attach_epoch,
                TaskTerminalizationKind::Abort,
            )
            .expect("task terminalization claim should be unique");
            (
                terminalization_identity,
                turn_state,
                task_context,
                recovery_seed,
                recovery_authority,
                attach_epoch,
            )
        };
        let recovery_seed = self
            .prepare_recovery_seed_for_controlled_detach(
                &task_context.sub_id,
                recovery_authority.as_ref(),
                recovery_seed,
            )
            .await;
        let task = {
            let mut active = self.active_turn.lock().await;
            let Some(active_turn) = active.as_mut() else {
                warn!("abort terminalizer lost active slot after recovery preparation; retaining fence");
                return None;
            };
            let Some(task_ref) = active_turn.task.as_ref() else {
                warn!("abort terminalizer lost task after recovery preparation; retaining fence");
                return None;
            };
            if !Arc::ptr_eq(&task_ref.turn_context, &task_context)
                || task_ref.attach_epoch != attach_epoch
                || !Self::task_terminalization_matches_locked(
                    active_turn,
                    &terminalization_identity,
                    task_ref,
                    TaskTerminalizationKind::Abort,
                )
            {
                warn!(
                    turn_id = %task_context.sub_id,
                    "abort terminalizer identity changed after recovery preparation; retaining fence"
                );
                return None;
            }
            Self::take_task_for_terminalization_locked(
                active_turn,
                &terminalization_identity,
                TaskTerminalizationKind::Abort,
            )
            .expect("claimed abort task should remain attached")
        };
        Some(ActiveTurnAbortTarget::Running(DetachedTaskForAbort {
            task,
            turn_state,
            terminalization_identity,
            recovery_seed,
            recovery_authority,
        }))
    }

    pub(crate) async fn close_unified_exec_processes(&self) {
        self.services
            .unified_exec_manager
            .terminate_all_processes()
            .await;
    }

    pub(crate) async fn list_background_terminals(&self) -> Vec<BackgroundTerminalInfo> {
        self.services.unified_exec_manager.list_processes().await
    }

    pub(crate) async fn terminate_background_terminal(&self, process_id: i32) -> bool {
        self.services
            .unified_exec_manager
            .terminate_process(process_id)
            .await
    }

    /// Completes an abort accepted during the host-owned start transition.
    ///
    /// There is no physical `RunningTask` to detach yet, but the normal abort
    /// path still owns important terminal semantics (admission completion,
    /// interrupted history marker, TurnAborted rollout/event, profile and
    /// guardian cleanup).  A pre-signalled placeholder lets that path run
    /// without ever invoking the real task's provider-facing `run` method.
    async fn abort_unstarted_turn(
        self: &Arc<Self>,
        task: Arc<dyn AnySessionTask>,
        turn_context: Arc<TurnContext>,
        turn_state: Arc<Mutex<TurnState>>,
        transition_identity: Arc<()>,
        mut recovery_history_restore: Option<ContextManager>,
        reason: TurnAbortReason,
    ) -> bool {
        if !self
            .restore_recovery_history_if_current(
                Some(&turn_state),
                &transition_identity,
                &mut recovery_history_restore,
            )
            .await
        {
            return false;
        }
        let done = Arc::new(Notify::new());
        // `handle_task_abort` waits for the task's completion notification;
        // pre-signal it because this placeholder never entered `run`.
        done.notify_one();
        let placeholder = RunningTask {
            done,
            kind: task.kind(),
            recovery_eligible_model_turn: false,
            recovery_authority: None,
            attach_epoch: self.turn_epoch.load(Ordering::Acquire),
            task,
            cancellation_token: CancellationToken::new(),
            handle: AbortOnDropHandle::new(tokio::spawn(std::future::pending::<()>())),
            turn_context: Arc::clone(&turn_context),
            _agent_execution_guard: None,
            _diagnostics_guard: ACTIVE_TURNS.track(),
            _timer: None,
        };
        let abort_outcome = self
            .handle_task_abort(placeholder, reason.clone(), None, None)
            .await;
        self.emit_turn_abort_lifecycle(reason.clone(), turn_context.extension_data.as_ref())
            .await;
        // Let the same terminal ordering as a running task clear queued input
        // only after the durable TurnAborted event has been emitted.
        self.input_queue
            .clear_pending_for_turn_state(turn_state.as_ref())
            .await;
        let clear_outcome = self
            .clear_start_transition_after_abort(&turn_state, &transition_identity)
            .await;
        let StartTransitionClearOutcome::Cleared {
            deferred_idle_cause,
        } = clear_outcome
        else {
            // A stale start future must not publish recovery or wake another
            // turn after its reservation has been replaced or fenced.
            return false;
        };
        if let Some(cause) = deferred_idle_cause {
            self.emit_thread_idle_lifecycle_if_idle_after_start_transition(
                cause,
                &transition_identity,
            )
            .await;
        }
        self.publish_recovery_seed_after_terminal(
            abort_outcome.recovery_seed,
            abort_outcome.task_quiesced,
            abort_outcome.terminal_persistence_generation,
        )
        .await;
        true
    }

    /// Claims an exact host-owned transition for detached cleanup after its
    /// owner future disappeared.  An external abort may already have stored a
    /// stronger reason; otherwise cancellation is represented as Interrupted.
    async fn abort_dropped_start_transition(
        self: &Arc<Self>,
        cleanup: StartTransitionCleanup,
        fallback_reason: TurnAbortReason,
    ) {
        let completion = Arc::clone(&cleanup.completion);
        let turn_state = Arc::clone(&cleanup.turn_state);
        let transition_identity = Arc::clone(&cleanup.transition_identity);
        let reason = {
            let mut active = self.active_turn.lock().await;
            let Some(active_turn) = active.as_mut() else {
                self.finish_start_transition(&transition_identity, &completion);
                return;
            };
            if active_turn.task.is_some()
                || active_turn.start_reservation.is_some()
                || active_turn.task_terminalization.is_some()
                || !Arc::ptr_eq(&active_turn.turn_state, &cleanup.turn_state)
                || !active_turn
                    .start_transition
                    .as_ref()
                    .is_some_and(|transition| {
                        Arc::ptr_eq(&transition.identity, &cleanup.transition_identity)
                    })
            {
                self.finish_start_transition(&transition_identity, &completion);
                return;
            }
            let transition = active_turn
                .start_transition
                .as_mut()
                .expect("transition identity was checked above");
            if transition.abort_reason.is_none() {
                if transition.request_abort(fallback_reason.clone())
                    && matches!(
                        fallback_reason,
                        TurnAbortReason::Interrupted | TurnAbortReason::BudgetLimited
                    )
                {
                    self.mark_interrupted();
                }
            }
            transition
                .abort_reason
                .clone()
                .expect("dropped transition cleanup stores an abort reason")
        };

        let terminalized = self
            .abort_unstarted_turn(
                cleanup.task,
                cleanup.turn_context,
                cleanup.turn_state,
                cleanup.transition_identity,
                cleanup.recovery_history_restore,
                reason.clone(),
            )
            .await;
        if terminalized {
            // Release the exact phase fence before allowing a pending mailbox
            // wakeup to reserve a replacement turn.  Shutdown admission is
            // already closed, so teardown will not self-wake here.
            self.finish_start_transition(&transition_identity, &completion);
            if reason == TurnAbortReason::Interrupted && !self.shutdown_started() {
                self.maybe_start_turn_for_pending_work().await;
            }
        } else {
            self.complete_start_transition_if_not_current(
                &completion,
                &turn_state,
                &transition_identity,
            )
            .await;
        }
    }

    async fn complete_start_transition_if_not_current(
        &self,
        completion: &Arc<StartTransitionCompletion>,
        turn_state: &Arc<Mutex<TurnState>>,
        transition_identity: &Arc<()>,
    ) {
        let active = self.active_turn.lock().await;
        let still_current = active.as_ref().is_some_and(|active_turn| {
            active_turn.task.is_none()
                && active_turn.start_reservation.is_none()
                && active_turn.task_terminalization.is_none()
                && Arc::ptr_eq(&active_turn.turn_state, turn_state)
                && active_turn
                    .start_transition
                    .as_ref()
                    .is_some_and(|transition| {
                        Arc::ptr_eq(&transition.identity, transition_identity)
                    })
        });
        drop(active);
        if !still_current {
            self.finish_start_transition(transition_identity, completion);
        }
    }

    /// Restores a recovery caller's pre-rewind history only while the exact
    /// host-owned start transition still owns the active slot.  The marker is
    /// retained across the history lock await, so a replacement cannot race
    /// the restore; a stale continuation simply abandons its witness.
    async fn restore_recovery_history_if_current(
        &self,
        turn_state: Option<&Arc<Mutex<TurnState>>>,
        transition_identity: &Arc<()>,
        recovery_history_restore: &mut Option<ContextManager>,
    ) -> bool {
        if recovery_history_restore.is_none() {
            return true;
        }
        let active = self.active_turn.lock().await;
        let Some(active_turn) = active.as_ref() else {
            return false;
        };
        let current = active_turn.task.is_none()
            && active_turn.start_reservation.is_none()
            && active_turn.task_terminalization.is_none()
            && turn_state.is_none_or(|expected| Arc::ptr_eq(&active_turn.turn_state, expected))
            && active_turn
                .start_transition
                .as_ref()
                .is_some_and(|transition| Arc::ptr_eq(&transition.identity, transition_identity));
        if !current {
            return false;
        }
        drop(active);
        let history = recovery_history_restore
            .take()
            .expect("recovery history witness remained present");
        // The witness is intentionally consumed only after the identity check;
        // the actual install is performed below without the active lock.
        self.install_recovery_history_snapshot(history).await;
        true
    }

    /// Clears the host-owned start-transition reservation after its deferred
    /// terminalization has completed.  This is an identity-fenced CAS: a
    /// stale start future must never clear a later reservation, even if the
    /// logical turn id or turn state happens to be reused.
    async fn clear_start_transition_after_abort(
        &self,
        turn_state: &Arc<Mutex<TurnState>>,
        transition_identity: &Arc<()>,
    ) -> StartTransitionClearOutcome {
        let mut active = self.active_turn.lock().await;
        let Some(active_turn) = active.as_mut() else {
            return StartTransitionClearOutcome::Stale;
        };
        if active_turn.task.is_some()
            || active_turn.start_reservation.is_some()
            || active_turn.task_terminalization.is_some()
            || !Arc::ptr_eq(&active_turn.turn_state, turn_state)
            || !active_turn
                .start_transition
                .as_ref()
                .is_some_and(|transition| Arc::ptr_eq(&transition.identity, transition_identity))
        {
            return StartTransitionClearOutcome::Stale;
        }
        let deferred_idle_cause = active_turn
            .start_transition
            .as_mut()
            .and_then(|transition| transition.take_deferred_idle());
        // Keep the marker installed until all terminal side effects above
        // have completed; only this final identity check may release it.
        *active = None;
        StartTransitionClearOutcome::Cleared {
            deferred_idle_cause,
        }
    }

    /// Releases a caller-owned reservation only through its exact handle. An
    /// accepted abort is returned to the owner instead of being silently
    /// dropped; there is no valid turn context yet, so the owner must take the
    /// explicit cancelled-before-context branch.
    pub(crate) async fn release_start_reservation_if_current(
        &self,
        handle: &StartReservationHandle,
    ) -> StartReservationRelease {
        let mut active = self.active_turn.lock().await;
        Self::release_start_reservation_if_current_locked(&mut active, handle)
    }

    fn release_start_reservation_if_current_locked(
        active: &mut Option<ActiveTurn>,
        handle: &StartReservationHandle,
    ) -> StartReservationRelease {
        let Some(active_turn) = active.as_mut() else {
            return StartReservationRelease::Stale;
        };
        if active_turn.task.is_some()
            || active_turn.start_transition.is_some()
            || active_turn.task_terminalization.is_some()
        {
            return StartReservationRelease::Stale;
        }
        let Some(reservation) = active_turn.start_reservation.as_ref() else {
            return StartReservationRelease::Stale;
        };
        if reservation.turn_id != handle.turn_id
            || !Arc::ptr_eq(&reservation.identity, &handle.identity)
            || !Arc::ptr_eq(&active_turn.turn_state, &handle.turn_state)
        {
            return StartReservationRelease::Stale;
        }
        let reservation = active_turn
            .start_reservation
            .take()
            .expect("reservation remained present after identity check");
        *active = None;
        match reservation.abort_reason {
            Some(reason) => StartReservationRelease::AbortRequested(reason),
            None => StartReservationRelease::Released,
        }
    }

    async fn handle_task_abort(
        self: &Arc<Self>,
        task: RunningTask,
        reason: TurnAbortReason,
        recovery_seed: Option<RecoverySeed>,
        recovery_authority: Option<Arc<TurnRecoveryAuthority>>,
    ) -> TaskAbortOutcome {
        let sub_id = task.turn_context.sub_id.clone();
        // The caller already persisted Unready while holding the active-turn
        // lock and left a reservation installed. Every controlled side effect
        // below therefore happens after durable revocation and before the
        // session can be observed as idle.
        if task.cancellation_token.is_cancelled() {
            if let Some(authority) = recovery_authority.as_ref() {
                let mut state = authority.state.lock().await;
                authority.ready.store(false, Ordering::Release);
                state.ready_persistence_failure_generation = None;
                state.poisoned = true;
            }
            return TaskAbortOutcome::default();
        }

        self.pending_user_message_admissions
            .complete_task_end(&sub_id);
        trace!(task_kind = ?task.kind, sub_id, "aborting running task");
        task.cancellation_token.cancel();
        if reason == TurnAbortReason::Interrupted
            && task
                .turn_context
                .config
                .features
                .enabled(Feature::CodeModeInterrupt)
        {
            self.services
                .code_mode_service
                .interrupt_active_cells()
                .await;
        }
        task.turn_context
            .turn_metadata_state
            .cancel_git_enrichment_task();
        let session_task = task.task;

        let mut task_quiesced = select! {
            _ = task.done.notified() => {
                true
            },
            _ = tokio::time::sleep(Duration::from_millis(GRACEFULL_INTERRUPTION_TIMEOUT_MS)) => {
                warn!("task {sub_id} didn't complete gracefully after {}ms", GRACEFULL_INTERRUPTION_TIMEOUT_MS);
                false
            }
        };

        task.handle.abort();
        if !task_quiesced {
            task_quiesced = tokio::time::timeout(
                Duration::from_millis(GRACEFULL_INTERRUPTION_TIMEOUT_MS),
                async {
                    while !task.handle.is_finished() {
                        tokio::task::yield_now().await;
                    }
                },
            )
            .await
            .is_ok();
        }

        session_task
            .abort(Arc::clone(self), Arc::clone(&task.turn_context))
            .await;

        // The task runner deliberately skips `on_task_finished` after the
        // cancellation token is set.  That is the right ownership boundary
        // for an abort, but it also means the runner's ordinary pre-terminal
        // rollout barrier is no longer available to persist output produced
        // immediately before cancellation.  Recovery authority has already
        // been revoked by the caller before entering this method, so this
        // barrier is now safe and preserves the normal abort durability
        // ordering: task output, interrupted marker, then TurnAborted.
        if let Err(err) = self.flush_rollout().await {
            warn!("failed to flush rollout before terminalizing aborted turn: {err}");
        }

        if reason == TurnAbortReason::Interrupted
            && let Some(marker) = interrupted_turn_history_marker(
                InterruptedTurnHistoryMarker::from_config_and_version(
                    task.turn_context.config.as_ref(),
                    task.turn_context.multi_agent_version,
                ),
            )
        {
            self.record_conversation_items(
                task.turn_context.as_ref(),
                std::slice::from_ref(&marker),
            )
            .await;
            // Ensure the marker is durably visible before emitting TurnAborted: some clients
            // synchronously re-read the rollout on receipt of the abort event.
            if let Err(err) = self.flush_rollout().await {
                warn!("failed to flush interrupted-turn marker before emitting TurnAborted: {err}");
            }
        }

        let started_at = task
            .turn_context
            .turn_timing_state
            .started_at_unix_secs()
            .await;
        let (completed_at, duration_ms, profile) = task
            .turn_context
            .turn_timing_state
            .complete_profile_and_duration_ms()
            .await;
        self.services
            .analytics_events_client
            .track_turn_profile(TurnProfileFact {
                turn_id: task.turn_context.sub_id.clone(),
                profile,
            });
        let event = EventMsg::TurnAborted(TurnAbortedEvent {
            turn_id: Some(task.turn_context.sub_id.clone()),
            reason,
            started_at,
            completed_at,
            duration_ms,
        });
        let terminal_persistence_generation = self
            .send_terminal_event_and_flush(task.turn_context.as_ref(), event)
            .await;
        self.services
            .guardian_rejection_circuit_breaker
            .lock()
            .await
            .clear_turn(&task.turn_context.sub_id);
        TaskAbortOutcome {
            task_quiesced,
            terminal_persistence_generation,
            recovery_seed,
        }
    }
}

fn qualification_admission_identity(
    thread_scope_key: &str,
    input: &[TurnInput],
) -> Option<QualificationTurnAdmissionIdentity> {
    let mut user_input = None;
    for item in input {
        if matches!(item, TurnInput::InterAgentCommunication(_)) {
            // A user message mixed with mailbox/inter-agent input is not a
            // single client admission.  Keep qualification identity inert
            // rather than binding the wrong source to the provider turn.
            return None;
        }
        let TurnInput::UserInput { content, client_id } = item else {
            continue;
        };
        if content.is_empty() || client_id.is_none() || user_input.is_some() {
            return None;
        }
        user_input = Some((content.as_slice(), client_id.as_deref()?));
    }
    let Some((content, client_id)) = user_input else {
        return None;
    };
    let payload_sha256 = user_input_payload_sha256(content).ok()?;
    QualificationTurnAdmissionIdentity::new(
        thread_scope_key.to_string(),
        client_id.to_string(),
        payload_sha256,
    )
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
