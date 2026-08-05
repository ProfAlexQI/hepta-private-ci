#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;

use codex_extension_api::ExtensionFuture;
use codex_extension_api::ExtensionRegistryBuilder;
use codex_extension_api::ThreadLifecycleContributor;
use codex_extension_api::ThreadStartInput;
use codex_extension_api::ToolCallOutcome;
use codex_extension_api::ToolCallSource;
use codex_extension_api::ToolPayload;
use codex_extension_api::ToolPolicyContributor;
use codex_extension_api::ToolPolicyDecision;
use codex_extension_api::ToolPolicyError;
use codex_extension_api::ToolPolicyFuture;
use codex_extension_api::ToolPolicyInput;
use codex_extension_api::ToolPolicyTerminalInput;
use codex_hepta_contracts::ActionId;
use codex_hepta_contracts::GOVERNANCE_SCHEMA_VERSION;
use codex_hepta_contracts::GovernanceDecision;
use codex_hepta_contracts::GovernanceDecisionRecord;
use codex_hepta_contracts::GovernanceMode;
use codex_hepta_contracts::GovernanceReceipt;
use codex_hepta_contracts::HandlerOutcome;
use codex_hepta_contracts::PolicyPhase;
use codex_hepta_contracts::PolicyStamp;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::ToolAction;
use codex_hepta_contracts::ToolActionSource;
use codex_hepta_evidence::AppendDisposition;
use codex_hepta_evidence::HeptaEvidenceStore;
use codex_state::StateRuntime;

mod provider_policy;

const BOOTSTRAP_POLICY_ID: &str = "hepta.bootstrap_integrity.v1";
const BOOTSTRAP_POLICY_REVISION: u64 = 1;
const BOOTSTRAP_POLICY_CONTENT: &[u8] =
    br#"{"decision":"not_evaluated","scope":"payload_digest_and_evidence_integrity"}"#;

#[derive(Default)]
struct InProcessClaims {
    /// Actions whose first durable admission insert was won by this process.
    ///
    /// This is deliberately only an execution witness. The SQLite evidence is
    /// authoritative for the decision and receipt material.
    owned: BTreeMap<ActionId, String>,
    /// Policy blocks caused by a replay must not finalize the original action.
    blocked_replays: BTreeSet<(ActionId, String)>,
}

pub struct GovernanceState {
    enabled: bool,
    mode: GovernanceMode,
    evidence: Result<Arc<HeptaEvidenceStore>, Arc<str>>,
    claims: Mutex<InProcessClaims>,
}

impl GovernanceState {
    fn disabled() -> Self {
        Self {
            enabled: false,
            mode: GovernanceMode::Shadow,
            evidence: Err(Arc::from("governance disabled")),
            claims: Mutex::new(InProcessClaims::default()),
        }
    }

    fn enabled(mode: GovernanceMode, evidence: Result<Arc<HeptaEvidenceStore>, Arc<str>>) -> Self {
        Self {
            enabled: true,
            mode,
            evidence,
            claims: Mutex::new(InProcessClaims::default()),
        }
    }

    async fn evaluate(
        &self,
        input: ToolPolicyInput<'_>,
        phase: PolicyPhase,
    ) -> Result<ToolPolicyDecision, ToolPolicyError> {
        if !self.enabled {
            return Ok(ToolPolicyDecision::Allow);
        }
        let action = tool_action(&input)?;
        let record = GovernanceDecisionRecord::new(
            action,
            phase,
            self.mode,
            bootstrap_policy_stamp(),
            GovernanceDecision::NotEvaluated,
        );
        let evidence = match self.evidence.as_ref() {
            Ok(evidence) => evidence,
            Err(detail) => return self.unavailable_or_shadow(detail),
        };
        let override_decision = match phase {
            PolicyPhase::Admission => self.admit(evidence, &record, input.attempt_id).await?,
            PolicyPhase::Authorization => {
                self.authorize(evidence, &record, input.attempt_id).await?
            }
        };
        if let Some(decision) = override_decision {
            return Ok(decision);
        }
        core_decision(self.mode, &record.decision)
    }

    async fn admit(
        &self,
        evidence: &HeptaEvidenceStore,
        record: &GovernanceDecisionRecord,
        attempt_id: &str,
    ) -> Result<Option<ToolPolicyDecision>, ToolPolicyError> {
        let disposition = match evidence.append_decision(record).await {
            Ok(disposition) => disposition,
            Err(error) => {
                return self.storage_failure_or_shadow(error.to_string()).map(Some);
            }
        };
        match disposition {
            AppendDisposition::Inserted => {
                let mut claims = match self.claims.lock() {
                    Ok(claims) => claims,
                    Err(_) => {
                        return self
                            .integrity_failure_or_shadow(
                                "hepta_governance_state_poisoned",
                                "in-process governance claim lock is poisoned",
                            )
                            .map(Some);
                    }
                };
                if claims
                    .owned
                    .insert(record.action.action_id.clone(), attempt_id.to_string())
                    .is_some()
                {
                    return self
                        .integrity_failure_or_shadow(
                            "hepta_admission_claim_conflict",
                            "one process claimed the same durable action more than once",
                        )
                        .map(Some);
                }
                Ok(None)
            }
            AppendDisposition::AlreadyPresent => self
                .replay_or_shadow(&record.action.action_id, attempt_id, PolicyPhase::Admission)
                .map(Some),
        }
    }

    async fn authorize(
        &self,
        evidence: &HeptaEvidenceStore,
        record: &GovernanceDecisionRecord,
        attempt_id: &str,
    ) -> Result<Option<ToolPolicyDecision>, ToolPolicyError> {
        let owns_action = match self.owns_action(&record.action.action_id, attempt_id) {
            Ok(owns_action) => owns_action,
            Err(error) => {
                return match self.mode {
                    GovernanceMode::Enforce => Err(error),
                    GovernanceMode::Shadow => {
                        tracing::warn!(
                            reason_code = error.reason_code(),
                            detail = error.detail(),
                            "shadow governance claim check failed"
                        );
                        Ok(Some(ToolPolicyDecision::Allow))
                    }
                };
            }
        };
        if !owns_action {
            return self
                .integrity_failure_or_shadow(
                    "hepta_authorization_without_claim",
                    "authorization has no in-process durable admission claim",
                )
                .map(Some);
        }
        let stored = match evidence.get_action_evidence(&record.action.action_id).await {
            Ok(stored) => stored,
            Err(error) => {
                return self.storage_failure_or_shadow(error.to_string()).map(Some);
            }
        };
        let Some(admission) = stored.admission.as_ref() else {
            return self
                .integrity_failure_or_shadow(
                    "hepta_authorization_without_admission",
                    "authorization has no durable admission decision",
                )
                .map(Some);
        };
        if stored.receipt.is_some() {
            return self
                .replay_or_shadow(
                    &record.action.action_id,
                    attempt_id,
                    PolicyPhase::Authorization,
                )
                .map(Some);
        }
        if stored.authorization.is_some() {
            return self
                .replay_or_shadow(
                    &record.action.action_id,
                    attempt_id,
                    PolicyPhase::Authorization,
                )
                .map(Some);
        }
        if !same_action_identity(&admission.action, &record.action)
            || admission.phase != PolicyPhase::Admission
            || admission.mode != self.mode
            || admission.policy != record.policy
            || admission.decision != GovernanceDecision::NotEvaluated
        {
            return self
                .integrity_failure_or_shadow(
                    "hepta_authorization_binding_drift",
                    "authorization identity or policy drifted from durable admission",
                )
                .map(Some);
        }
        match evidence.append_decision(record).await {
            Ok(AppendDisposition::Inserted) => Ok(None),
            Ok(AppendDisposition::AlreadyPresent) => self
                .replay_or_shadow(
                    &record.action.action_id,
                    attempt_id,
                    PolicyPhase::Authorization,
                )
                .map(Some),
            Err(error) => self.storage_failure_or_shadow(error.to_string()).map(Some),
        }
    }

    async fn terminal(&self, input: ToolPolicyTerminalInput<'_>) -> Result<(), ToolPolicyError> {
        if !self.enabled {
            return Ok(());
        }
        let action_id =
            ActionId::for_tool_call(input.thread_store.level_id(), input.turn_id, input.call_id);
        if matches!(input.outcome, ToolCallOutcome::Blocked)
            && self.consume_blocked_replay(&action_id, input.attempt_id)?
        {
            return Ok(());
        }
        let owns_action = match self.owns_action(&action_id, input.attempt_id) {
            Ok(owns_action) => owns_action,
            Err(error) => {
                return match self.mode {
                    GovernanceMode::Enforce => Err(error),
                    GovernanceMode::Shadow => {
                        tracing::warn!(
                            reason_code = error.reason_code(),
                            detail = error.detail(),
                            "shadow governance terminal claim check failed"
                        );
                        Ok(())
                    }
                };
            }
        };
        if !owns_action {
            // A replay or a storage failure never owns the original action's
            // terminal material. Leaving a durable pending action untouched is
            // safer than minting a false receipt.
            return Ok(());
        }
        let evidence = match self.evidence.as_ref() {
            Ok(evidence) => evidence,
            Err(detail) => {
                return self.terminal_unavailable_or_shadow(detail, &action_id, input.attempt_id);
            }
        };
        let stored = match evidence.get_action_evidence(&action_id).await {
            Ok(stored) => stored,
            Err(error) => {
                return self.terminal_storage_failure_or_shadow_with_action(
                    error.to_string(),
                    &action_id,
                    input.attempt_id,
                );
            }
        };
        if stored.receipt.is_some() {
            self.release_action_for_mode(&action_id, input.attempt_id)?;
            return Ok(());
        }
        let Some(admission) = stored.admission else {
            return self.terminal_integrity_failure_or_shadow(
                "hepta_terminal_without_admission",
                "terminal callback has no durable admission decision",
                &action_id,
                input.attempt_id,
            );
        };
        if !terminal_matches_action(&input, &admission.action) {
            return self.terminal_integrity_failure_or_shadow(
                "hepta_terminal_binding_drift",
                "terminal callback does not bind the admitted tool identity",
                &action_id,
                input.attempt_id,
            );
        }
        let outcome = handler_outcome(input.outcome, stored.authorization.is_some());
        let receipt = GovernanceReceipt::new(
            admission,
            stored.authorization,
            input.host_accepted,
            outcome,
        );
        match evidence.append_receipt(&receipt).await {
            Ok(AppendDisposition::Inserted | AppendDisposition::AlreadyPresent) => {
                self.release_action_for_mode(&action_id, input.attempt_id)?;
                Ok(())
            }
            Err(error) => self.terminal_storage_failure_or_shadow_with_action(
                error.to_string(),
                &action_id,
                input.attempt_id,
            ),
        }
    }

    fn owns_action(&self, action_id: &ActionId, attempt_id: &str) -> Result<bool, ToolPolicyError> {
        self.claims
            .lock()
            .map(|claims| {
                claims
                    .owned
                    .get(action_id)
                    .is_some_and(|owned_attempt| owned_attempt == attempt_id)
            })
            .map_err(|_| {
                ToolPolicyError::new(
                    "hepta_governance_state_poisoned",
                    "in-process governance claim lock is poisoned",
                )
            })
    }

    fn release_action(
        &self,
        action_id: &ActionId,
        attempt_id: &str,
    ) -> Result<(), ToolPolicyError> {
        self.claims
            .lock()
            .map(|mut claims| {
                if claims
                    .owned
                    .get(action_id)
                    .is_some_and(|owned_attempt| owned_attempt == attempt_id)
                {
                    claims.owned.remove(action_id);
                }
            })
            .map_err(|_| {
                ToolPolicyError::new(
                    "hepta_governance_state_poisoned",
                    "in-process governance claim lock is poisoned",
                )
            })
    }

    fn release_action_for_mode(
        &self,
        action_id: &ActionId,
        attempt_id: &str,
    ) -> Result<(), ToolPolicyError> {
        match self.release_action(action_id, attempt_id) {
            Ok(()) => Ok(()),
            Err(error) if self.mode == GovernanceMode::Shadow => {
                tracing::warn!(
                    reason_code = error.reason_code(),
                    detail = error.detail(),
                    "shadow governance could not release an in-process claim"
                );
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn replay_or_shadow(
        &self,
        action_id: &ActionId,
        attempt_id: &str,
        phase: PolicyPhase,
    ) -> Result<ToolPolicyDecision, ToolPolicyError> {
        let reason_code = match phase {
            PolicyPhase::Admission => "hepta_action_replay",
            PolicyPhase::Authorization => "hepta_authorization_replay",
        };
        match self.mode {
            GovernanceMode::Shadow => {
                tracing::warn!(
                    action_id = action_id.as_str(),
                    phase = phase.as_str(),
                    "shadow governance observed a durable action replay"
                );
                Ok(ToolPolicyDecision::Allow)
            }
            GovernanceMode::Enforce => {
                let mut claims = self.claims.lock().map_err(|_| {
                    ToolPolicyError::new(
                        "hepta_governance_state_poisoned",
                        "in-process governance claim lock is poisoned",
                    )
                })?;
                if !claims
                    .blocked_replays
                    .insert((action_id.clone(), attempt_id.to_string()))
                {
                    return Err(ToolPolicyError::new(
                        "hepta_replay_attempt_conflict",
                        "one policy attempt tried to claim the same replay twice",
                    ));
                }
                Ok(ToolPolicyDecision::Block {
                    reason_code: reason_code.to_string(),
                    message: "Hepta blocked a replay of an existing durable tool action"
                        .to_string(),
                })
            }
        }
    }

    fn consume_blocked_replay(
        &self,
        action_id: &ActionId,
        attempt_id: &str,
    ) -> Result<bool, ToolPolicyError> {
        let mut claims = self.claims.lock().map_err(|_| {
            ToolPolicyError::new(
                "hepta_governance_state_poisoned",
                "in-process governance claim lock is poisoned",
            )
        })?;
        Ok(claims
            .blocked_replays
            .remove(&(action_id.clone(), attempt_id.to_string())))
    }

    fn unavailable_or_shadow(
        &self,
        detail: &Arc<str>,
    ) -> Result<ToolPolicyDecision, ToolPolicyError> {
        match self.mode {
            GovernanceMode::Enforce => Err(ToolPolicyError::new(
                "hepta_evidence_unavailable",
                detail.to_string(),
            )),
            GovernanceMode::Shadow => {
                tracing::warn!(%detail, "shadow governance evidence backend is unavailable");
                Ok(ToolPolicyDecision::Allow)
            }
        }
    }

    fn storage_failure_or_shadow(
        &self,
        detail: String,
    ) -> Result<ToolPolicyDecision, ToolPolicyError> {
        match self.mode {
            GovernanceMode::Enforce => {
                Err(ToolPolicyError::new("hepta_evidence_write_failed", detail))
            }
            GovernanceMode::Shadow => {
                tracing::warn!(%detail, "shadow governance evidence write failed");
                Ok(ToolPolicyDecision::Allow)
            }
        }
    }

    fn integrity_failure_or_shadow(
        &self,
        reason_code: &'static str,
        detail: &'static str,
    ) -> Result<ToolPolicyDecision, ToolPolicyError> {
        match self.mode {
            GovernanceMode::Enforce => Err(ToolPolicyError::new(reason_code, detail)),
            GovernanceMode::Shadow => {
                tracing::warn!(
                    reason_code,
                    detail,
                    "shadow governance integrity check failed"
                );
                Ok(ToolPolicyDecision::Allow)
            }
        }
    }

    fn terminal_unavailable_or_shadow(
        &self,
        detail: &Arc<str>,
        action_id: &ActionId,
        attempt_id: &str,
    ) -> Result<(), ToolPolicyError> {
        match self.mode {
            GovernanceMode::Enforce => Err(ToolPolicyError::new(
                "hepta_evidence_unavailable",
                detail.to_string(),
            )),
            GovernanceMode::Shadow => {
                tracing::warn!(%detail, "shadow governance terminal evidence is unavailable");
                self.release_action_for_mode(action_id, attempt_id)?;
                Ok(())
            }
        }
    }

    fn terminal_storage_failure_or_shadow_with_action(
        &self,
        detail: String,
        action_id: &ActionId,
        attempt_id: &str,
    ) -> Result<(), ToolPolicyError> {
        match self.mode {
            GovernanceMode::Enforce => {
                // Retain the in-process claim. The durable authorized decision
                // remains pending and any replay is blocked on the next admit.
                Err(ToolPolicyError::new("hepta_evidence_write_failed", detail))
            }
            GovernanceMode::Shadow => {
                tracing::warn!(%detail, "shadow governance terminal evidence write failed");
                self.release_action_for_mode(action_id, attempt_id)?;
                Ok(())
            }
        }
    }

    fn terminal_integrity_failure_or_shadow(
        &self,
        reason_code: &'static str,
        detail: &'static str,
        action_id: &ActionId,
        attempt_id: &str,
    ) -> Result<(), ToolPolicyError> {
        match self.mode {
            GovernanceMode::Enforce => Err(ToolPolicyError::new(reason_code, detail)),
            GovernanceMode::Shadow => {
                tracing::warn!(
                    reason_code,
                    detail,
                    "shadow terminal integrity check failed"
                );
                self.release_action_for_mode(action_id, attempt_id)?;
                Ok(())
            }
        }
    }
}

struct HeptaGovernanceExtension<F> {
    enabled: F,
    mode: GovernanceMode,
    state_db: Option<Arc<StateRuntime>>,
    evidence: tokio::sync::OnceCell<Arc<HeptaEvidenceStore>>,
}

impl<F> HeptaGovernanceExtension<F> {
    async fn initialize_thread<C>(
        &self,
        config: &C,
        thread_store: &codex_extension_api::ExtensionData,
    ) where
        F: Fn(&C) -> bool,
    {
        if !(self.enabled)(config) {
            thread_store.insert(GovernanceState::disabled());
            return;
        }
        let evidence = match self.state_db.as_ref() {
            Some(state_db) => self
                .evidence
                .get_or_try_init(|| async {
                    HeptaEvidenceStore::open(state_db.sqlite())
                        .await
                        .map(Arc::new)
                        .map_err(|error| Arc::<str>::from(error.to_string()))
                })
                .await
                .cloned(),
            None => Err(Arc::from("Codex state runtime is unavailable")),
        };
        thread_store.insert(GovernanceState::enabled(self.mode, evidence));
    }
}

impl<C, F> ThreadLifecycleContributor<C> for HeptaGovernanceExtension<F>
where
    C: Sync,
    F: Fn(&C) -> bool + Send + Sync,
{
    fn on_thread_start<'a>(&'a self, input: ThreadStartInput<'a, C>) -> ExtensionFuture<'a, ()> {
        Box::pin(async move {
            self.initialize_thread(input.config, input.thread_store)
                .await;
        })
    }
}

impl<F> ToolPolicyContributor for HeptaGovernanceExtension<F>
where
    F: Send + Sync,
{
    fn is_active(&self, thread_store: &codex_extension_api::ExtensionData) -> bool {
        // A missing state is kept active so enforce mode can fail closed and
        // shadow mode can emit its explicit observation. A feature-disabled
        // thread always contains a disabled state and is a true no-op.
        thread_store
            .get::<GovernanceState>()
            .is_none_or(|state| state.enabled)
    }

    fn admit<'a>(&'a self, input: ToolPolicyInput<'a>) -> ToolPolicyFuture<'a, ToolPolicyDecision> {
        Box::pin(async move {
            let Some(state) = governance_state(input.thread_store, self.mode)? else {
                return Ok(ToolPolicyDecision::Allow);
            };
            state.evaluate(input, PolicyPhase::Admission).await
        })
    }

    fn authorize<'a>(
        &'a self,
        input: ToolPolicyInput<'a>,
    ) -> ToolPolicyFuture<'a, ToolPolicyDecision> {
        Box::pin(async move {
            let Some(state) = governance_state(input.thread_store, self.mode)? else {
                return Ok(ToolPolicyDecision::Allow);
            };
            state.evaluate(input, PolicyPhase::Authorization).await
        })
    }

    fn on_terminal<'a>(&'a self, input: ToolPolicyTerminalInput<'a>) -> ToolPolicyFuture<'a, ()> {
        Box::pin(async move {
            let Some(state) = governance_state(input.thread_store, self.mode)? else {
                return Ok(());
            };
            state.terminal(input).await
        })
    }
}

pub fn install<C, F>(
    registry: &mut ExtensionRegistryBuilder<C>,
    state_db: Option<Arc<StateRuntime>>,
    enabled: F,
) where
    C: Sync + 'static,
    F: Fn(&C) -> bool + Send + Sync + 'static,
{
    install_with_mode(registry, state_db, GovernanceMode::Shadow, enabled);
}

/// Install the governance extension with an explicit rollout mode.
///
/// Product surfaces use shadow mode until the durable oracle soak is accepted;
/// focused tests exercise enforce-mode fail-closed behavior through this API.
pub fn install_with_mode<C, F>(
    registry: &mut ExtensionRegistryBuilder<C>,
    state_db: Option<Arc<StateRuntime>>,
    mode: GovernanceMode,
    enabled: F,
) where
    C: Sync + 'static,
    F: Fn(&C) -> bool + Send + Sync + 'static,
{
    let extension = Arc::new(HeptaGovernanceExtension {
        enabled,
        mode,
        state_db,
        evidence: tokio::sync::OnceCell::new(),
    });
    registry.thread_lifecycle_contributor(extension.clone());
    registry.model_provider_policy_contributor(extension.clone());
    registry.tool_policy_contributor(extension);
}

fn governance_state(
    thread_store: &codex_extension_api::ExtensionData,
    mode: GovernanceMode,
) -> Result<Option<Arc<GovernanceState>>, ToolPolicyError> {
    if let Some(state) = thread_store.get::<GovernanceState>() {
        return Ok(Some(state));
    }
    match mode {
        GovernanceMode::Enforce => Err(ToolPolicyError::new(
            "hepta_governance_state_missing",
            "thread governance state was not initialized",
        )),
        GovernanceMode::Shadow => {
            tracing::warn!("shadow governance thread state was not initialized");
            Ok(None)
        }
    }
}

fn bootstrap_policy_stamp() -> PolicyStamp {
    PolicyStamp::new(
        BOOTSTRAP_POLICY_ID,
        BOOTSTRAP_POLICY_REVISION,
        BOOTSTRAP_POLICY_CONTENT,
    )
}

fn tool_action(input: &ToolPolicyInput<'_>) -> Result<ToolAction, ToolPolicyError> {
    Ok(ToolAction {
        schema_version: GOVERNANCE_SCHEMA_VERSION,
        action_id: ActionId::for_tool_call(
            input.thread_store.level_id(),
            input.turn_id,
            input.call_id,
        ),
        thread_id: input.thread_store.level_id().to_string(),
        turn_id: input.turn_id.to_string(),
        call_id: input.call_id.to_string(),
        tool_name: input.tool_name.to_string(),
        source: tool_action_source(&input.source),
        payload_sha256: payload_digest(input.payload)?,
    })
}

fn tool_action_source(source: &ToolCallSource) -> ToolActionSource {
    match source {
        ToolCallSource::Direct => ToolActionSource::Direct,
        ToolCallSource::DirectPlaintextMessage => ToolActionSource::DirectPlaintextMessage,
        ToolCallSource::CodeMode {
            cell_id,
            runtime_tool_call_id,
        } => ToolActionSource::CodeMode {
            cell_id: cell_id.clone(),
            runtime_tool_call_id: runtime_tool_call_id.clone(),
        },
    }
}

fn payload_digest(payload: &ToolPayload) -> Result<Sha256Digest, ToolPolicyError> {
    let (kind, body) = match payload {
        ToolPayload::Function { arguments } => ("function", arguments.as_bytes().to_vec()),
        ToolPayload::ToolSearch { arguments } => (
            "tool_search",
            serde_json::to_vec(arguments).map_err(|error| {
                ToolPolicyError::new("hepta_payload_serialization_failed", error.to_string())
            })?,
        ),
        ToolPayload::Custom { input } => ("custom", input.as_bytes().to_vec()),
    };
    let mut canonical = Vec::with_capacity(kind.len() + body.len() + 8);
    canonical.extend_from_slice((kind.len() as u64).to_be_bytes().as_ref());
    canonical.extend_from_slice(kind.as_bytes());
    canonical.extend_from_slice(&body);
    Ok(Sha256Digest::for_bytes(&canonical))
}

fn same_action_identity(left: &ToolAction, right: &ToolAction) -> bool {
    left.action_id == right.action_id
        && left.thread_id == right.thread_id
        && left.turn_id == right.turn_id
        && left.call_id == right.call_id
        && left.tool_name == right.tool_name
        && left.source == right.source
}

fn terminal_matches_action(input: &ToolPolicyTerminalInput<'_>, action: &ToolAction) -> bool {
    action.schema_version == GOVERNANCE_SCHEMA_VERSION
        && action.action_id
            == ActionId::for_tool_call(input.thread_store.level_id(), input.turn_id, input.call_id)
        && action.thread_id == input.thread_store.level_id()
        && action.turn_id == input.turn_id
        && action.call_id == input.call_id
        && action.tool_name == input.tool_name.to_string()
        && action.source == tool_action_source(&input.source)
}

fn core_decision(
    mode: GovernanceMode,
    decision: &GovernanceDecision,
) -> Result<ToolPolicyDecision, ToolPolicyError> {
    match (mode, decision) {
        (_, GovernanceDecision::NotEvaluated | GovernanceDecision::Allow)
        | (GovernanceMode::Shadow, GovernanceDecision::Block { .. }) => {
            Ok(ToolPolicyDecision::Allow)
        }
        (GovernanceMode::Enforce, GovernanceDecision::Block { reason_code }) => {
            Ok(ToolPolicyDecision::Block {
                reason_code: reason_code.clone(),
                message: format!("Hepta governance blocked this tool call ({reason_code})"),
            })
        }
    }
}

fn handler_outcome(outcome: ToolCallOutcome, authorization_exists: bool) -> HandlerOutcome {
    match outcome {
        ToolCallOutcome::Completed { success } => HandlerOutcome::HandlerCompleted {
            reported_success: success,
        },
        ToolCallOutcome::Blocked => HandlerOutcome::Blocked,
        ToolCallOutcome::Failed { handler_executed } => {
            HandlerOutcome::HandlerFailed { handler_executed }
        }
        ToolCallOutcome::Aborted if authorization_exists => HandlerOutcome::Indeterminate {
            reason_code: "cancelled_after_authorization".to_string(),
        },
        ToolCallOutcome::Aborted => HandlerOutcome::Aborted,
        ToolCallOutcome::Indeterminate { reason_code } => HandlerOutcome::Indeterminate {
            reason_code: reason_code.to_string(),
        },
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
