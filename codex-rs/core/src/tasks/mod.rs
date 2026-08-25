mod compact;
mod lifecycle;
mod regular;
mod review;
mod user_shell;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;

use codex_diagnostics::Gauge;
use codex_extension_api::QualificationTurnAdmissionIdentity;
use codex_extension_api::ThreadIdleCause;
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
use crate::state::TaskKind;
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
    recovery_seed: Option<RecoverySeed>,
    recovery_authority: Option<Arc<TurnRecoveryAuthority>>,
}

enum ActiveTurnAbortTarget {
    Running(DetachedTaskForAbort),
    Starting { deferred_idle: bool },
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
}

#[derive(Debug)]
pub(crate) enum StartReservationRelease {
    Released,
    AbortRequested(TurnAbortReason),
    Stale,
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
        if !self.enabled(Feature::HeptaTurnRecovery) {
            return RecoveryProviderOutputGate::Attached;
        }
        let active = self.active_turn.lock().await;
        let Some(task) = active
            .as_ref()
            .and_then(|active_turn| active_turn.task.as_ref())
        else {
            return RecoveryProviderOutputGate::Detached;
        };
        if task.turn_context.sub_id != turn_id
            || !task
                .recovery_authority
                .as_ref()
                .is_some_and(|current| Arc::ptr_eq(current, authority))
        {
            return RecoveryProviderOutputGate::Detached;
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
        if !self.enabled(Feature::HeptaTurnRecovery) {
            return Ok(RecoveryProviderOutputGate::Attached);
        }
        let active = self.active_turn.lock().await;
        let Some(task) = active
            .as_ref()
            .and_then(|active_turn| active_turn.task.as_ref())
        else {
            return Ok(RecoveryProviderOutputGate::Detached);
        };
        if task.turn_context.sub_id != turn_id
            || !task
                .recovery_authority
                .as_ref()
                .is_some_and(|current| Arc::ptr_eq(current, authority))
        {
            return Ok(RecoveryProviderOutputGate::Detached);
        }
        self.ensure_turn_recovery_unready(turn_id, authority.as_ref())
            .await?;
        Ok(RecoveryProviderOutputGate::Attached)
    }

    pub async fn spawn_task<T: SessionTask>(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        input: Vec<TurnInput>,
        task: T,
    ) -> CodexResult<()> {
        self.abort_all_tasks(TurnAbortReason::Replaced).await;
        let start_reservation = {
            let mut active_turn = self.active_turn.lock().await;
            if active_turn.is_some() {
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
        self.clear_connector_selection().await;
        let start_outcome = self
            .start_task_owned(
                turn_context,
                input,
                task,
                MailboxParentProvenance::Ignore,
                start_reservation,
            )
            .await;
        match start_outcome {
            StartTaskOutcome::Attached | StartTaskOutcome::Aborted => Ok(()),
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
        {
            let mut active = self.active_turn.lock().await;
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
            }
        }
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
                return StartTaskOutcome::Stale;
            };
            debug_assert!(turn.task.is_none());
            let Some(transition) = turn.start_transition.as_ref() else {
                drop(active);
                self.restore_recovery_history_if_current(
                    None,
                    &start_transition_identity,
                    &mut recovery_history_restore,
                )
                .await;
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
            return StartTaskOutcome::Stale;
        };
        debug_assert!(turn.task.is_none());
        let Some(transition) = turn.start_transition.as_ref() else {
            drop(active);
            self.restore_recovery_history_if_current(
                None,
                &start_transition_identity,
                &mut recovery_history_restore,
            )
            .await;
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
            return StartTaskOutcome::Stale;
        }
        if let Some(reason) = transition.abort_reason.clone() {
            // Keep the marker installed while terminalization awaits.  The
            // abort side records the reason but never emits lifecycle or
            // terminal events concurrently with an in-flight on_turn_start;
            // retaining the marker also prevents a concurrent reservation
            // clearer or replacement start from stealing this turn state.
            drop(active);
            self.abort_unstarted_turn(
                task,
                turn_context,
                turn_state,
                start_transition_identity,
                recovery_history_restore,
                reason,
            )
            .await;
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
                let task_result = async {
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
                .instrument(trace_span!("session_task.run"))
                .await;
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
                .maybe_start_turn_for_pending_work_with_sub_id(uuid::Uuid::new_v4().to_string())
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
        if !self.input_queue.has_pending_mailbox_items().await
            || (!self.input_queue.has_trigger_turn_mailbox_items().await
                && !self.has_outstanding_durable_sleep())
        {
            return;
        }

        let start_reservation = {
            let mut active_turn = self.active_turn.lock().await;
            if active_turn.is_some() {
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

        let turn_context = self.new_default_turn_with_sub_id(sub_id).await;
        self.maybe_emit_model_warnings_for_turn(turn_context.as_ref())
            .await;
        self.start_task_owned(
            turn_context,
            Vec::new(),
            RegularTask::new(TurnRunOrigin::NewTurn),
            MailboxParentProvenance::Attribute,
            start_reservation,
        )
        .await;
    }

    pub async fn abort_all_tasks(self: &Arc<Self>, reason: TurnAbortReason) {
        let mut aborted_turn = false;
        let mut reserved_turn_state = None;
        let mut turn_context = None;
        let mut abort_outcome = TaskAbortOutcome::default();
        if let Some(target) = self
            .detach_active_task_for_abort(
                &reason, /*expected_turn_id*/ None, /*expected_turn_state*/ None,
                /*deferred_idle_cause*/ None,
            )
            .await
        {
            let ActiveTurnAbortTarget::Running(detached) = target else {
                // The start owner will perform terminalization after its
                // in-flight lifecycle callback returns.  Finalizing here
                // would race on_turn_start/on_turn_abort ordering.
                return;
            };
            aborted_turn = true;
            turn_context = Some(Arc::clone(&detached.task.turn_context));
            reserved_turn_state = Some(Arc::clone(&detached.turn_state));
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
        if let Some(turn_state) = reserved_turn_state.as_ref() {
            // Let interrupted tasks observe cancellation before dropping pending approvals, or an
            // in-flight approval wait can surface as a model-visible rejection before TurnAborted.
            self.input_queue
                .clear_pending_for_turn_state(turn_state.as_ref())
                .await;
            self.clear_reserved_idle_turn(turn_state).await;
        }
        self.publish_recovery_seed_after_terminal(
            abort_outcome.recovery_seed,
            abort_outcome.task_quiesced,
            abort_outcome.terminal_persistence_generation,
        )
        .await;
        if reason == TurnAbortReason::Interrupted && aborted_turn {
            self.maybe_start_turn_for_pending_work().await;
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
        let ActiveTurnAbortTarget::Running(detached) = target else {
            let ActiveTurnAbortTarget::Starting { deferred_idle } = target else {
                unreachable!("non-running abort target must be starting")
            };
            return if deferred_idle {
                AbortTurnOutcome::DeferredStart
            } else {
                AbortTurnOutcome::Starting
            };
        };

        let turn_context = Arc::clone(&detached.task.turn_context);
        let reserved_turn_state = Arc::clone(&detached.turn_state);
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
        self.clear_reserved_idle_turn(&reserved_turn_state).await;

        self.publish_recovery_seed_after_terminal(
            abort_outcome.recovery_seed,
            abort_outcome.task_quiesced,
            abort_outcome.terminal_persistence_generation,
        )
        .await;

        if reason == TurnAbortReason::Interrupted {
            self.maybe_start_turn_for_pending_work().await;
        }

        AbortTurnOutcome::Running
    }

    pub async fn on_task_finished(
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
        let task_state = {
            let mut active = self.active_turn.lock().await;
            let Some(active_turn) = active.as_mut() else {
                return;
            };
            let Some(task) = active_turn.task.as_ref() else {
                return;
            };
            if !Arc::ptr_eq(&task.turn_context, &turn_context) {
                return;
            }
            // Keep active -> authority/rollout ordering while revoking Ready,
            // then remove the task. No flush, warning, error lifecycle, or
            // protocol event may become observable while a first-event Ready
            // marker still authorizes cold recovery.
            let recovery_seed = Self::recovery_seed_for_task(task, abort_reason_hint.as_ref());
            let recovery_authority = task.recovery_authority.clone();
            let recovery_seed = self
                .prepare_recovery_seed_for_controlled_detach(
                    &turn_context.sub_id,
                    recovery_authority.as_ref(),
                    recovery_seed,
                )
                .await;
            let task = active_turn
                .task
                .take()
                .expect("task remained attached while recovery authority was revoked");
            Some((Arc::clone(&active_turn.turn_state), task, recovery_seed))
        };
        let Some((turn_state, task, recovery_seed)) = task_state else {
            return;
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

        let cleared_active_turn = {
            let mut active = self.active_turn.lock().await;
            if let Some(active_turn) = active.as_ref()
                && active_turn.task.is_none()
                && active_turn.start_reservation.is_none()
                && active_turn.start_transition.is_none()
                && Arc::ptr_eq(&active_turn.turn_state, &turn_state)
            {
                *active = None;
                true
            } else {
                false
            }
        };
        if cleared_active_turn {
            self.emit_thread_idle_lifecycle_if_idle(idle_cause).await;
        }
        self.publish_recovery_seed_after_terminal(
            recovery_seed,
            /*task_quiesced*/ true,
            terminal_persistence_generation,
        )
        .await;
        if cleared_active_turn {
            self.maybe_start_turn_for_pending_work().await;
        }
    }

    /// Revokes task-owned recovery authority while the active slot is locked,
    /// then either detaches a running task or fences a caller reservation / the
    /// host-owned start transition. Starts and injections therefore cannot
    /// observe an idle session until the old task is quiescent and its terminal
    /// is durable (or the start owner has completed its deferred handoff).
    #[expect(
        clippy::await_holding_invalid_type,
        reason = "active -> recovery authority/rollout ordering is the serialization boundary"
    )]
    async fn detach_active_task_for_abort(
        &self,
        reason: &TurnAbortReason,
        expected_turn_id: Option<&str>,
        expected_turn_state: Option<&Arc<Mutex<TurnState>>>,
        deferred_idle_cause: Option<ThreadIdleCause>,
    ) -> Option<ActiveTurnAbortTarget> {
        let mut active = self.active_turn.lock().await;
        let active_turn = active.as_mut()?;
        if expected_turn_state
            .is_some_and(|expected| !Arc::ptr_eq(&active_turn.turn_state, expected))
        {
            return None;
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
        let turn_id = task.turn_context.sub_id.clone();
        let recovery_seed = self
            .prepare_recovery_seed_for_controlled_detach(
                &turn_id,
                recovery_authority.as_ref(),
                recovery_seed,
            )
            .await;
        let task = active_turn
            .task
            .take()
            .expect("task remained attached while recovery authority was revoked");
        Some(ActiveTurnAbortTarget::Running(DetachedTaskForAbort {
            task,
            turn_state: Arc::clone(&active_turn.turn_state),
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
    ) {
        if !self
            .restore_recovery_history_if_current(
                Some(&turn_state),
                &transition_identity,
                &mut recovery_history_restore,
            )
            .await
        {
            return;
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
            return;
        };
        if let Some(cause) = deferred_idle_cause {
            self.emit_thread_idle_lifecycle_if_idle(cause).await;
        }
        self.publish_recovery_seed_after_terminal(
            abort_outcome.recovery_seed,
            abort_outcome.task_quiesced,
            abort_outcome.terminal_persistence_generation,
        )
        .await;
        if reason == TurnAbortReason::Interrupted {
            self.maybe_start_turn_for_pending_work().await;
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
        let Some(active_turn) = active.as_mut() else {
            return StartReservationRelease::Stale;
        };
        if active_turn.task.is_some() || active_turn.start_transition.is_some() {
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
