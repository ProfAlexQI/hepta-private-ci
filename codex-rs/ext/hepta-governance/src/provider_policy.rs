use std::sync::Arc;

use codex_extension_api::MODEL_PROVIDER_POLICY_INPUT_SCHEMA_VERSION;
use codex_extension_api::ModelProviderAttemptLease;
use codex_extension_api::ModelProviderInvocationInput;
use codex_extension_api::ModelProviderPolicyContributor;
use codex_extension_api::ModelProviderPolicyDecision;
use codex_extension_api::ModelProviderPolicyError;
use codex_extension_api::ModelProviderPolicyFuture;
use codex_extension_api::ModelProviderRequestKind as ApiRequestKind;
use codex_extension_api::ModelProviderTerminal as ApiTerminal;
use codex_extension_api::ModelProviderTransport as ApiTransport;
use codex_hepta_contracts::PROVIDER_EVIDENCE_SCHEMA_VERSION;
use codex_hepta_contracts::ProviderInvocationIntent;
use codex_hepta_contracts::ProviderInvocationReceipt;
use codex_hepta_contracts::ProviderRequestBinding;
use codex_hepta_contracts::ProviderRequestKind;
use codex_hepta_contracts::ProviderTerminal;
use codex_hepta_contracts::ProviderTransport;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_evidence::AppendDisposition;
use codex_hepta_evidence::EvidenceError;
use codex_hepta_evidence::HeptaEvidenceStore;
use codex_hepta_evidence::ProviderBindingState;
use codex_hepta_evidence::ProviderIntentClaimDisposition;

use super::GovernanceState;
use super::HeptaGovernanceExtension;
use codex_hepta_contracts::GovernanceMode;

struct DurableProviderAttemptLease {
    evidence: Arc<HeptaEvidenceStore>,
    intent: ProviderInvocationIntent,
    mode: GovernanceMode,
}

impl ModelProviderAttemptLease for DurableProviderAttemptLease {
    fn finish(self: Box<Self>, terminal: ApiTerminal) -> ModelProviderPolicyFuture<'static, ()> {
        Box::pin(async move {
            let DurableProviderAttemptLease {
                evidence,
                intent,
                mode,
            } = *self;
            let result = async {
                let terminal = provider_terminal(terminal)?;
                let receipt = ProviderInvocationReceipt::new(intent, terminal);
                match evidence.append_provider_receipt(&receipt).await {
                    Ok(AppendDisposition::Inserted | AppendDisposition::AlreadyPresent) => Ok(()),
                    Err(error) => Err(provider_evidence_error(
                        "hepta_provider_terminal_write_failed",
                        error,
                    )),
                }
            }
            .await;
            match (mode, result) {
                (_, Ok(())) => Ok(()),
                (GovernanceMode::Enforce, Err(error)) => Err(error),
                (GovernanceMode::Shadow, Err(error)) => {
                    tracing::warn!(
                        reason_code = error.reason_code(),
                        detail = error.detail(),
                        "shadow provider terminal observation was not persisted"
                    );
                    Ok(())
                }
            }
        })
    }
}

/// Shadow-only lease for an invocation that did not win an authoritative claim.
///
/// It deliberately owns no intent and therefore cannot finalize an older pending attempt.
struct DetachedShadowProviderLease;

impl ModelProviderAttemptLease for DetachedShadowProviderLease {
    fn finish(self: Box<Self>, _terminal: ApiTerminal) -> ModelProviderPolicyFuture<'static, ()> {
        Box::pin(std::future::ready(Ok(())))
    }
}

impl GovernanceState {
    async fn begin_provider(
        &self,
        input: ModelProviderInvocationInput<'_>,
    ) -> Result<ModelProviderPolicyDecision, ModelProviderPolicyError> {
        if !self.enabled {
            return Ok(detached_shadow_allow());
        }
        let intent = match provider_intent(&input) {
            Ok(intent) => intent,
            Err(error) => return Ok(self.provider_failure_or_shadow(error)),
        };
        let evidence = match self.evidence.as_ref() {
            Ok(evidence) => Arc::clone(evidence),
            Err(detail) => {
                return Ok(
                    self.provider_failure_or_shadow(ModelProviderPolicyError::new(
                        "hepta_provider_evidence_unavailable",
                        detail.to_string(),
                    )),
                );
            }
        };
        match self.mode {
            GovernanceMode::Shadow => match evidence.append_provider_intent(&intent).await {
                Ok(AppendDisposition::Inserted) => Ok(durable_allow(evidence, intent, self.mode)),
                Ok(AppendDisposition::AlreadyPresent) => {
                    tracing::warn!(
                        attempt_id = intent.attempt_id.as_str(),
                        request_binding_id = intent.request_binding_id.as_str(),
                        "shadow governance observed an exact provider attempt replay"
                    );
                    Ok(detached_shadow_allow())
                }
                Err(error) => Ok(self.provider_failure_or_shadow(provider_evidence_error(
                    "hepta_provider_intent_write_failed",
                    error,
                ))),
            },
            GovernanceMode::Enforce => match evidence.claim_provider_intent(&intent).await {
                Ok(ProviderIntentClaimDisposition::Inserted) => {
                    Ok(durable_allow(evidence, intent, self.mode))
                }
                Ok(ProviderIntentClaimDisposition::ExactReplay) => {
                    let reason_code = match evidence.get_provider_attempt(&intent.attempt_id).await
                    {
                        Ok(Some(stored)) if stored.receipt.is_some() => {
                            "hepta_provider_attempt_replay"
                        }
                        Ok(Some(_)) => "hepta_provider_attempt_pending",
                        Ok(None) => "hepta_provider_evidence_corrupt",
                        Err(_) => "hepta_provider_evidence_read_failed",
                    };
                    Ok(provider_block(
                        reason_code,
                        "Hepta blocked replay of an existing durable provider attempt",
                    ))
                }
                Ok(ProviderIntentClaimDisposition::BlockedByBinding(state)) => {
                    let (reason_code, message) = match state {
                        ProviderBindingState::Pending => (
                            "hepta_provider_request_pending",
                            "Hepta blocked retry of a provider request with a pending attempt",
                        ),
                        ProviderBindingState::Completed => (
                            "hepta_provider_request_completed",
                            "Hepta blocked retry of an already completed provider request",
                        ),
                        ProviderBindingState::Indeterminate => (
                            "hepta_provider_request_indeterminate",
                            "Hepta blocked automatic retry of an indeterminate provider request",
                        ),
                    };
                    Ok(provider_block(reason_code, message))
                }
                Err(error) => Ok(provider_block_for_error(error)),
            },
        }
    }

    fn provider_failure_or_shadow(
        &self,
        error: ModelProviderPolicyError,
    ) -> ModelProviderPolicyDecision {
        match self.mode {
            GovernanceMode::Enforce => provider_block(
                error.reason_code(),
                "Hepta could not establish durable provider intent evidence",
            ),
            GovernanceMode::Shadow => {
                tracing::warn!(
                    reason_code = error.reason_code(),
                    detail = error.detail(),
                    "shadow provider governance observation failed"
                );
                detached_shadow_allow()
            }
        }
    }
}

impl<F> ModelProviderPolicyContributor for HeptaGovernanceExtension<F>
where
    F: Send + Sync,
{
    fn is_active(&self, thread_store: &codex_extension_api::ExtensionData) -> bool {
        thread_store
            .get::<GovernanceState>()
            .is_none_or(|state| state.enabled)
    }

    fn begin<'a>(
        &'a self,
        input: ModelProviderInvocationInput<'a>,
    ) -> ModelProviderPolicyFuture<'a, ModelProviderPolicyDecision> {
        Box::pin(async move {
            let Some(state) = input.thread_store.get::<GovernanceState>() else {
                return Ok(match self.mode {
                    GovernanceMode::Enforce => provider_block(
                        "hepta_governance_state_missing",
                        "Hepta provider governance state was not initialized",
                    ),
                    GovernanceMode::Shadow => {
                        tracing::warn!(
                            "shadow provider governance thread state was not initialized"
                        );
                        detached_shadow_allow()
                    }
                });
            };
            state.begin_provider(input).await
        })
    }
}

fn provider_intent(
    input: &ModelProviderInvocationInput<'_>,
) -> Result<ProviderInvocationIntent, ModelProviderPolicyError> {
    if input.schema_version != MODEL_PROVIDER_POLICY_INPUT_SCHEMA_VERSION {
        return Err(ModelProviderPolicyError::new(
            "hepta_provider_schema_unsupported",
            "unsupported model-provider policy input schema version",
        ));
    }
    for (label, value) in [
        ("attempt id", input.attempt_id),
        ("request binding id", input.request_binding_id),
        ("thread id", input.thread_id),
        ("turn id", input.turn_id),
        ("provider id", input.provider_id),
        ("model", input.model),
    ] {
        if value.trim().is_empty() {
            return Err(ModelProviderPolicyError::new(
                "hepta_provider_identity_invalid",
                format!("provider invocation requires a non-empty {label}"),
            ));
        }
    }
    if input.thread_id != input.thread_store.level_id()
        || input.turn_id != input.turn_store.level_id()
    {
        return Err(ModelProviderPolicyError::new(
            "hepta_provider_scope_mismatch",
            "provider invocation identity does not match extension store scope",
        ));
    }
    let binding = ProviderRequestBinding {
        schema_version: PROVIDER_EVIDENCE_SCHEMA_VERSION,
        thread_id: input.thread_id.to_string(),
        turn_id: input.turn_id.to_string(),
        host_request_binding_id_sha256: Sha256Digest::for_bytes(
            input.request_binding_id.as_bytes(),
        ),
        request_kind: match input.request_kind {
            ApiRequestKind::Turn => ProviderRequestKind::Turn,
            ApiRequestKind::Prewarm => ProviderRequestKind::Prewarm,
            ApiRequestKind::Compaction => ProviderRequestKind::Compaction,
            ApiRequestKind::Memory => ProviderRequestKind::Memory,
        },
        provider_id: input.provider_id.to_string(),
        provider_config_sha256: contract_digest(input.provider_config_sha256.as_str())?,
        model: input.model.to_string(),
        transport: match input.transport {
            ApiTransport::Http => ProviderTransport::Http,
            ApiTransport::WebSocket => ProviderTransport::WebSocket,
        },
        endpoint_sha256: contract_digest(input.endpoint_sha256.as_str())?,
        logical_request_sha256: contract_digest(input.logical_request_sha256.as_str())?,
        wire_semantic_sha256: contract_digest(input.wire_semantic_sha256.as_str())?,
        previous_response_id_sha256: input
            .previous_response_id_sha256
            .map(|digest| contract_digest(digest.as_str()))
            .transpose()?,
        generate: input.generate,
    };
    Ok(ProviderInvocationIntent::for_host_attempt_id(
        input.attempt_id,
        binding,
    ))
}

fn provider_terminal(terminal: ApiTerminal) -> Result<ProviderTerminal, ModelProviderPolicyError> {
    Ok(match terminal {
        ApiTerminal::Completed {
            response_id_sha256,
            response_items_sha256,
            token_usage_sha256,
            end_turn,
        } => ProviderTerminal::Completed {
            response_id_sha256: contract_digest(response_id_sha256.as_str())?,
            response_items_sha256: contract_digest(response_items_sha256.as_str())?,
            token_usage_sha256: contract_digest(token_usage_sha256.as_str())?,
            end_turn,
        },
        ApiTerminal::Rejected { reason_code } => ProviderTerminal::Rejected {
            reason_code: stable_reason_code(reason_code)?,
        },
        ApiTerminal::NotDispatched { reason_code } => ProviderTerminal::NotDispatched {
            reason_code: stable_reason_code(reason_code)?,
        },
        ApiTerminal::Indeterminate {
            reason_code,
            partial_response_sha256,
        } => ProviderTerminal::Indeterminate {
            reason_code: stable_reason_code(reason_code)?,
            partial_response_sha256: partial_response_sha256
                .map(|digest| contract_digest(digest.as_str()))
                .transpose()?,
        },
    })
}

fn stable_reason_code(reason_code: String) -> Result<String, ModelProviderPolicyError> {
    if (1..=128).contains(&reason_code.len())
        && reason_code.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-' | b'.')
        })
    {
        Ok(reason_code)
    } else {
        Err(ModelProviderPolicyError::new(
            "hepta_provider_reason_code_invalid",
            "provider terminal reason code is not a stable secret-free identifier",
        ))
    }
}

fn contract_digest(value: &str) -> Result<Sha256Digest, ModelProviderPolicyError> {
    Sha256Digest::parse(value)
        .map_err(|detail| ModelProviderPolicyError::new("hepta_provider_digest_invalid", detail))
}

fn durable_allow(
    evidence: Arc<HeptaEvidenceStore>,
    intent: ProviderInvocationIntent,
    mode: GovernanceMode,
) -> ModelProviderPolicyDecision {
    ModelProviderPolicyDecision::Allow {
        lease: Box::new(DurableProviderAttemptLease {
            evidence,
            intent,
            mode,
        }),
    }
}

fn detached_shadow_allow() -> ModelProviderPolicyDecision {
    ModelProviderPolicyDecision::Allow {
        lease: Box::new(DetachedShadowProviderLease),
    }
}

fn provider_block(
    reason_code: impl Into<String>,
    message: impl Into<String>,
) -> ModelProviderPolicyDecision {
    ModelProviderPolicyDecision::Block {
        reason_code: reason_code.into(),
        message: message.into(),
    }
}

fn provider_block_for_error(error: EvidenceError) -> ModelProviderPolicyDecision {
    let error = provider_evidence_error("hepta_provider_intent_write_failed", error);
    tracing::warn!(
        reason_code = error.reason_code(),
        detail = error.detail(),
        "enforced provider governance intent claim failed"
    );
    provider_block(
        error.reason_code(),
        "Hepta could not claim authoritative provider intent evidence",
    )
}

fn provider_evidence_error(
    fallback_reason_code: &'static str,
    error: EvidenceError,
) -> ModelProviderPolicyError {
    let reason_code = match error {
        EvidenceError::IdempotencyConflict { .. } => "hepta_provider_evidence_conflict",
        EvidenceError::Corrupt(_) => "hepta_provider_evidence_corrupt",
        EvidenceError::Unavailable(_) => "hepta_provider_evidence_unavailable",
        EvidenceError::InvalidRecord(_) => "hepta_provider_evidence_invalid",
        EvidenceError::Serialization(_) => fallback_reason_code,
    };
    ModelProviderPolicyError::new(reason_code, error.to_string())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use codex_extension_api::ExtensionData;
    use codex_extension_api::ExtensionRegistryBuilder;
    use codex_extension_api::MODEL_PROVIDER_POLICY_INPUT_SCHEMA_VERSION;
    use codex_extension_api::ModelProviderAttemptLease;
    use codex_extension_api::ModelProviderInvocationInput;
    use codex_extension_api::ModelProviderPolicyDecision;
    use codex_extension_api::ModelProviderRequestKind;
    use codex_extension_api::ModelProviderSha256Digest;
    use codex_extension_api::ModelProviderTerminal;
    use codex_extension_api::ModelProviderTransport;
    use codex_hepta_contracts::GovernanceMode;
    use codex_hepta_contracts::ProviderTerminal as StoredTerminal;
    use codex_hepta_contracts::Sha256Digest;
    use codex_hepta_evidence::AppendDisposition;
    use codex_hepta_evidence::HeptaEvidenceStore;
    use codex_state::SqliteConfig;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use tempfile::TempDir;

    use super::DurableProviderAttemptLease;
    use super::GovernanceState;
    use super::provider_intent;
    use crate::install_with_mode;

    struct Digests {
        provider_config: ModelProviderSha256Digest,
        endpoint: ModelProviderSha256Digest,
        logical: ModelProviderSha256Digest,
        wire: ModelProviderSha256Digest,
    }

    fn sqlite_config(temp: &TempDir) -> SqliteConfig {
        SqliteConfig::new_for_testing(
            AbsolutePathBuf::try_from(temp.path().to_path_buf()).expect("absolute temp path"),
        )
    }

    async fn evidence(temp: &TempDir) -> Arc<HeptaEvidenceStore> {
        Arc::new(
            HeptaEvidenceStore::open(&sqlite_config(temp))
                .await
                .expect("open evidence"),
        )
    }

    fn stores() -> (ExtensionData, ExtensionData, ExtensionData) {
        (
            ExtensionData::new("session-1"),
            ExtensionData::new("thread-1"),
            ExtensionData::new("turn-1"),
        )
    }

    fn api_digest(bytes: &[u8]) -> ModelProviderSha256Digest {
        ModelProviderSha256Digest::parse(Sha256Digest::for_bytes(bytes).as_str())
            .expect("fixture digest")
    }

    fn digests(endpoint: &[u8]) -> Digests {
        Digests {
            provider_config: api_digest(b"provider-config"),
            endpoint: api_digest(endpoint),
            logical: api_digest(b"logical-request"),
            wire: api_digest(b"wire-request"),
        }
    }

    fn input<'a>(
        session: &'a ExtensionData,
        thread: &'a ExtensionData,
        turn: &'a ExtensionData,
        attempt_id: &'a str,
        request_binding_id: &'a str,
        digests: &'a Digests,
    ) -> ModelProviderInvocationInput<'a> {
        ModelProviderInvocationInput {
            schema_version: MODEL_PROVIDER_POLICY_INPUT_SCHEMA_VERSION,
            session_store: session,
            thread_store: thread,
            turn_store: turn,
            attempt_id,
            request_binding_id,
            thread_id: "thread-1",
            turn_id: "turn-1",
            request_kind: ModelProviderRequestKind::Turn,
            provider_id: "provider-fixture",
            provider_config_sha256: &digests.provider_config,
            model: "model-fixture",
            transport: ModelProviderTransport::Http,
            endpoint_sha256: &digests.endpoint,
            logical_request_sha256: &digests.logical,
            wire_semantic_sha256: &digests.wire,
            previous_response_id_sha256: None,
            generate: true,
        }
    }

    fn completed() -> ModelProviderTerminal {
        ModelProviderTerminal::Completed {
            response_id_sha256: api_digest(b"response-id"),
            response_items_sha256: api_digest(b"response-items"),
            token_usage_sha256: api_digest(b"token-usage"),
            end_turn: Some(true),
        }
    }

    #[tokio::test]
    async fn inserted_intent_returns_owned_lease_and_finishes_exact_terminal() {
        const HOST_ATTEMPT: &str = "host-attempt-secret-fixture-701";
        const HOST_BINDING: &str = "host-binding-secret-fixture-702";
        const ENDPOINT: &[u8] = b"https://provider.invalid/responses?secret=703";
        const OUTPUT: &[u8] = b"provider-output-secret-fixture-704";

        let temp = TempDir::new().expect("temp dir");
        let evidence = evidence(&temp).await;
        let state = GovernanceState::enabled(GovernanceMode::Enforce, Ok(evidence.clone()));
        let (session, thread, turn) = stores();
        let digests = digests(ENDPOINT);
        let invocation = input(
            &session,
            &thread,
            &turn,
            HOST_ATTEMPT,
            HOST_BINDING,
            &digests,
        );
        let expected_intent = provider_intent(&invocation).expect("convert intent");
        let decision = state
            .begin_provider(invocation)
            .await
            .expect("provider begin");
        let ModelProviderPolicyDecision::Allow { lease } = decision else {
            panic!("first durable insert must own an allow lease");
        };
        assert_eq!(
            evidence
                .pending_provider_attempt_count()
                .await
                .expect("pending count"),
            1
        );

        lease.finish(completed()).await.expect("persist terminal");

        let stored = evidence
            .get_provider_attempt(&expected_intent.attempt_id)
            .await
            .expect("read attempt")
            .expect("attempt exists");
        assert_eq!(stored.intent.intent, expected_intent);
        let receipt = stored.receipt.expect("terminal exists").receipt;
        assert!(matches!(receipt.terminal, StoredTerminal::Completed { .. }));
        let json = serde_json::to_string(&receipt).expect("serialize receipt");
        for forbidden in [
            HOST_ATTEMPT.as_bytes(),
            HOST_BINDING.as_bytes(),
            ENDPOINT,
            OUTPUT,
        ] {
            assert!(
                !json
                    .as_bytes()
                    .windows(forbidden.len())
                    .any(|window| window == forbidden),
                "provider evidence leaked forbidden plaintext"
            );
        }
        assert_eq!(
            receipt.intent.attempt_nonce_sha256,
            Sha256Digest::for_bytes(HOST_ATTEMPT.as_bytes())
        );
        assert_eq!(
            receipt.intent.binding.host_request_binding_id_sha256,
            Sha256Digest::for_bytes(HOST_BINDING.as_bytes())
        );
    }

    #[tokio::test]
    async fn concurrent_exact_attempt_has_one_owner_and_one_typed_block() {
        let temp = TempDir::new().expect("temp dir");
        let evidence = evidence(&temp).await;
        let state = GovernanceState::enabled(GovernanceMode::Enforce, Ok(evidence.clone()));
        let (session, thread, turn) = stores();
        let digests = digests(b"endpoint");

        let (left, right) = tokio::join!(
            state.begin_provider(input(
                &session,
                &thread,
                &turn,
                "attempt-concurrent",
                "binding-concurrent",
                &digests,
            )),
            state.begin_provider(input(
                &session,
                &thread,
                &turn,
                "attempt-concurrent",
                "binding-concurrent",
                &digests,
            ))
        );
        let mut owner = None;
        let mut blocked = 0;
        for decision in [left.expect("left"), right.expect("right")] {
            match decision {
                ModelProviderPolicyDecision::Allow { lease } => owner = Some(lease),
                ModelProviderPolicyDecision::Block { reason_code, .. } => {
                    assert_eq!(reason_code, "hepta_provider_attempt_pending");
                    blocked += 1;
                }
            }
        }
        assert_eq!(blocked, 1);
        owner
            .expect("one exact owner")
            .finish(ModelProviderTerminal::NotDispatched {
                reason_code: "cancelled_before_send".to_string(),
            })
            .await
            .expect("finish owner");
        assert_eq!(
            evidence
                .pending_provider_attempt_count()
                .await
                .expect("pending count"),
            0
        );
    }

    #[tokio::test]
    async fn restart_pending_and_new_attempt_same_binding_are_blocked_in_enforce() {
        let temp = TempDir::new().expect("temp dir");
        let evidence = evidence(&temp).await;
        let (session, thread, turn) = stores();
        let digests = digests(b"endpoint");
        let first = GovernanceState::enabled(GovernanceMode::Enforce, Ok(evidence.clone()));
        let first_decision = first
            .begin_provider(input(
                &session,
                &thread,
                &turn,
                "attempt-pending",
                "binding-pending",
                &digests,
            ))
            .await
            .expect("first begin");
        assert!(matches!(
            first_decision,
            ModelProviderPolicyDecision::Allow { .. }
        ));
        drop(first_decision);
        drop(first);

        let restarted = GovernanceState::enabled(GovernanceMode::Enforce, Ok(evidence));
        for (attempt_id, expected_reason) in [
            ("attempt-pending", "hepta_provider_attempt_pending"),
            ("attempt-retry", "hepta_provider_request_pending"),
        ] {
            let decision = restarted
                .begin_provider(input(
                    &session,
                    &thread,
                    &turn,
                    attempt_id,
                    "binding-pending",
                    &digests,
                ))
                .await
                .expect("typed block");
            assert!(matches!(
                decision,
                ModelProviderPolicyDecision::Block { reason_code, .. }
                    if reason_code == expected_reason
            ));
        }
    }

    #[tokio::test]
    async fn rejected_or_not_dispatched_binding_can_be_claimed_again() {
        let temp = TempDir::new().expect("temp dir");
        let evidence = evidence(&temp).await;
        let state = GovernanceState::enabled(GovernanceMode::Enforce, Ok(evidence));
        let (session, thread, turn) = stores();
        let digests = digests(b"endpoint");

        for (attempt_id, terminal) in [
            (
                "attempt-rejected",
                ModelProviderTerminal::Rejected {
                    reason_code: "provider_unauthorized".to_string(),
                },
            ),
            (
                "attempt-not-dispatched",
                ModelProviderTerminal::NotDispatched {
                    reason_code: "transport_not_entered".to_string(),
                },
            ),
        ] {
            let decision = state
                .begin_provider(input(
                    &session,
                    &thread,
                    &turn,
                    attempt_id,
                    "binding-retry-safe",
                    &digests,
                ))
                .await
                .expect("safe retry begin");
            let ModelProviderPolicyDecision::Allow { lease } = decision else {
                panic!("rejected/not-dispatched predecessor must permit a fresh attempt");
            };
            lease.finish(terminal).await.expect("finish safe attempt");
        }
    }

    #[tokio::test]
    async fn completed_and_indeterminate_bindings_block_new_attempts() {
        for (terminal, expected_reason) in [
            (completed(), "hepta_provider_request_completed"),
            (
                ModelProviderTerminal::Indeterminate {
                    reason_code: "stream_eof_before_completed".to_string(),
                    partial_response_sha256: None,
                },
                "hepta_provider_request_indeterminate",
            ),
        ] {
            let temp = TempDir::new().expect("temp dir");
            let evidence = evidence(&temp).await;
            let state = GovernanceState::enabled(GovernanceMode::Enforce, Ok(evidence));
            let (session, thread, turn) = stores();
            let digests = digests(b"endpoint");
            let first = state
                .begin_provider(input(
                    &session,
                    &thread,
                    &turn,
                    "attempt-first",
                    "binding-terminal",
                    &digests,
                ))
                .await
                .expect("first begin");
            let ModelProviderPolicyDecision::Allow { lease } = first else {
                panic!("first attempt owns lease");
            };
            lease.finish(terminal).await.expect("finish first");
            let retry = state
                .begin_provider(input(
                    &session,
                    &thread,
                    &turn,
                    "attempt-retry",
                    "binding-terminal",
                    &digests,
                ))
                .await
                .expect("typed retry block");
            assert!(matches!(
                retry,
                ModelProviderPolicyDecision::Block { reason_code, .. }
                    if reason_code == expected_reason
            ));
        }
    }

    #[tokio::test]
    async fn shadow_exact_replay_lease_cannot_finish_stale_pending_intent() {
        let temp = TempDir::new().expect("temp dir");
        let evidence = evidence(&temp).await;
        let state = GovernanceState::enabled(GovernanceMode::Shadow, Ok(evidence.clone()));
        let (session, thread, turn) = stores();
        let digests = digests(b"endpoint");
        let first = state
            .begin_provider(input(
                &session,
                &thread,
                &turn,
                "attempt-shadow",
                "binding-shadow",
                &digests,
            ))
            .await
            .expect("first begin");
        assert!(matches!(first, ModelProviderPolicyDecision::Allow { .. }));
        drop(first);

        let replay = state
            .begin_provider(input(
                &session,
                &thread,
                &turn,
                "attempt-shadow",
                "binding-shadow",
                &digests,
            ))
            .await
            .expect("shadow replay");
        let ModelProviderPolicyDecision::Allow { lease } = replay else {
            panic!("shadow replay remains observational");
        };
        lease.finish(completed()).await.expect("detached finish");
        assert_eq!(
            evidence
                .pending_provider_attempt_count()
                .await
                .expect("pending count"),
            1,
            "detached shadow replay must not mint the old terminal"
        );
    }

    #[tokio::test]
    async fn terminal_conflict_is_an_error_and_preserves_original_receipt() {
        let temp = TempDir::new().expect("temp dir");
        let evidence = evidence(&temp).await;
        let (session, thread, turn) = stores();
        let digests = digests(b"endpoint");
        let invocation = input(
            &session,
            &thread,
            &turn,
            "attempt-terminal-conflict",
            "binding-terminal-conflict",
            &digests,
        );
        let intent = provider_intent(&invocation).expect("intent");
        assert_eq!(
            evidence
                .append_provider_intent(&intent)
                .await
                .expect("insert intent"),
            AppendDisposition::Inserted
        );
        Box::new(DurableProviderAttemptLease {
            evidence: evidence.clone(),
            intent: intent.clone(),
            mode: GovernanceMode::Enforce,
        })
        .finish(completed())
        .await
        .expect("first terminal");
        let error = Box::new(DurableProviderAttemptLease {
            evidence: evidence.clone(),
            intent: intent.clone(),
            mode: GovernanceMode::Enforce,
        })
        .finish(ModelProviderTerminal::Rejected {
            reason_code: "provider_unauthorized".to_string(),
        })
        .await
        .expect_err("different terminal must conflict");
        assert_eq!(error.reason_code(), "hepta_provider_evidence_conflict");
        let stored = evidence
            .get_provider_attempt(&intent.attempt_id)
            .await
            .expect("read attempt")
            .expect("attempt");
        assert!(matches!(
            stored.receipt.expect("receipt").receipt.terminal,
            StoredTerminal::Completed { .. }
        ));
    }

    #[tokio::test]
    async fn shadow_terminal_conflict_is_observational_and_preserves_original_receipt() {
        let temp = TempDir::new().expect("temp dir");
        let evidence = evidence(&temp).await;
        let (session, thread, turn) = stores();
        let digests = digests(b"endpoint");
        let intent = provider_intent(&input(
            &session,
            &thread,
            &turn,
            "attempt-shadow-terminal-conflict",
            "binding-shadow-terminal-conflict",
            &digests,
        ))
        .expect("intent");
        evidence
            .append_provider_intent(&intent)
            .await
            .expect("insert intent");
        Box::new(DurableProviderAttemptLease {
            evidence: evidence.clone(),
            intent: intent.clone(),
            mode: GovernanceMode::Enforce,
        })
        .finish(completed())
        .await
        .expect("first terminal");

        Box::new(DurableProviderAttemptLease {
            evidence: evidence.clone(),
            intent: intent.clone(),
            mode: GovernanceMode::Shadow,
        })
        .finish(ModelProviderTerminal::Rejected {
            reason_code: "provider_unauthorized".to_string(),
        })
        .await
        .expect("shadow terminal conflict should remain observational");

        let stored = evidence
            .get_provider_attempt(&intent.attempt_id)
            .await
            .expect("read attempt")
            .expect("attempt");
        assert!(matches!(
            stored.receipt.expect("receipt").receipt.terminal,
            StoredTerminal::Completed { .. }
        ));
    }

    #[tokio::test]
    async fn backend_unavailable_blocks_enforce_and_detaches_shadow() {
        let (session, thread, turn) = stores();
        let digests = digests(b"endpoint");
        let unavailable = Err(Arc::<str>::from("offline"));
        let enforce = GovernanceState::enabled(GovernanceMode::Enforce, unavailable.clone());
        let decision = enforce
            .begin_provider(input(
                &session,
                &thread,
                &turn,
                "attempt-unavailable",
                "binding-unavailable",
                &digests,
            ))
            .await
            .expect("typed block");
        assert!(matches!(
            decision,
            ModelProviderPolicyDecision::Block { reason_code, .. }
                if reason_code == "hepta_provider_evidence_unavailable"
        ));

        let shadow = GovernanceState::enabled(GovernanceMode::Shadow, unavailable);
        let decision = shadow
            .begin_provider(input(
                &session,
                &thread,
                &turn,
                "attempt-unavailable",
                "binding-unavailable",
                &digests,
            ))
            .await
            .expect("shadow allow");
        let ModelProviderPolicyDecision::Allow { lease } = decision else {
            panic!("shadow unavailable is observational");
        };
        lease.finish(completed()).await.expect("detached finish");
    }

    #[test]
    fn installing_governance_registers_the_same_provider_policy_extension() {
        let mut builder = ExtensionRegistryBuilder::<()>::new();
        install_with_mode(&mut builder, None, GovernanceMode::Shadow, |_: &()| true);
        let registry = builder.build();
        assert_eq!(registry.thread_lifecycle_contributors().len(), 1);
        assert_eq!(registry.tool_policy_contributors().len(), 1);
        assert_eq!(registry.model_provider_policy_contributors().len(), 1);
    }

    #[test]
    fn raw_host_ids_change_durable_digests_without_crossing_evidence_boundary() {
        let (session, thread, turn) = stores();
        let digests = digests(b"endpoint");
        let left = provider_intent(&input(
            &session,
            &thread,
            &turn,
            "host-attempt-a",
            "host-binding-a",
            &digests,
        ))
        .expect("left intent");
        let right_attempt = provider_intent(&input(
            &session,
            &thread,
            &turn,
            "host-attempt-b",
            "host-binding-a",
            &digests,
        ))
        .expect("right attempt intent");
        let right_binding = provider_intent(&input(
            &session,
            &thread,
            &turn,
            "host-attempt-a",
            "host-binding-b",
            &digests,
        ))
        .expect("right binding intent");

        assert_ne!(left.attempt_id, right_attempt.attempt_id);
        assert_ne!(left.request_binding_id, right_binding.request_binding_id);
        let json = serde_json::to_string(&[left, right_attempt, right_binding])
            .expect("serialize intents");
        for forbidden in [
            "host-attempt-a",
            "host-attempt-b",
            "host-binding-a",
            "host-binding-b",
        ] {
            assert!(!json.contains(forbidden));
        }
    }
}
