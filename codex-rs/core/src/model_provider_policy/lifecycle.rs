use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionRegistry;
use codex_extension_api::ModelProviderAttemptLease;
use codex_extension_api::ModelProviderInvocationInput;
use codex_extension_api::ModelProviderPolicyDecision;
use codex_extension_api::ModelProviderPolicyError;
use codex_extension_api::ModelProviderPolicyFuture;
use codex_extension_api::ModelProviderTerminal;

const BLOCKED_CLEANUP_REASON: &str = "model_provider_policy_blocked";
const ERROR_CLEANUP_REASON: &str = "model_provider_policy_begin_failed";

/// Result of aggregating all active model-provider policy contributors.
///
/// `NoPolicy` remains distinct from `Allow`: callers can preserve the ordinary
/// provider fast path when no contributor is active.
pub(crate) enum ModelProviderPolicyBegin {
    NoPolicy,
    Allow {
        lease: Box<dyn ModelProviderAttemptLease>,
    },
    Block {
        reason_code: String,
        message: String,
    },
}

/// Returns whether constructing a provider-policy binding is necessary.
pub(crate) fn has_active_model_provider_policy<C: Sync>(
    registry: &ExtensionRegistry<C>,
    thread_store: &ExtensionData,
) -> bool {
    registry
        .model_provider_policy_contributors()
        .iter()
        .any(|contributor| contributor.is_active(thread_store))
}

/// Runs active provider-policy contributors in registration order.
///
/// Every acquired lease is either returned as one opaque, single-use
/// composite lease or completed as `NotDispatched` before a later block/error
/// is returned. Cleanup and terminal failures fail closed and are surfaced.
pub(crate) async fn begin_model_provider_policy<C: Sync>(
    registry: &ExtensionRegistry<C>,
    input: ModelProviderInvocationInput<'_>,
) -> Result<ModelProviderPolicyBegin, ModelProviderPolicyError> {
    let active = registry
        .model_provider_policy_contributors()
        .iter()
        .filter(|contributor| contributor.is_active(input.thread_store));
    let mut leases = Vec::new();

    for contributor in active {
        match contributor.begin(copy_input(&input)).await {
            Ok(ModelProviderPolicyDecision::Allow { lease }) => leases.push(lease),
            Ok(ModelProviderPolicyDecision::Block {
                reason_code,
                message,
            }) => {
                let block = format!("{reason_code}: {message}");
                finish_leases(
                    leases,
                    ModelProviderTerminal::NotDispatched {
                        reason_code: BLOCKED_CLEANUP_REASON.to_string(),
                    },
                    "model_provider_policy_block_cleanup_failed",
                )
                .await
                .map_err(|cleanup| {
                    ModelProviderPolicyError::new(
                        "model_provider_policy_block_and_cleanup_failed",
                        format!(
                            "policy blocked ({block}); cleanup failed ({})",
                            cleanup.detail()
                        ),
                    )
                })?;
                return Ok(ModelProviderPolicyBegin::Block {
                    reason_code,
                    message,
                });
            }
            Err(error) => {
                if leases.is_empty() {
                    return Err(error);
                }
                let origin = format!("{}: {}", error.reason_code(), error.detail());
                finish_leases(
                    leases,
                    ModelProviderTerminal::NotDispatched {
                        reason_code: ERROR_CLEANUP_REASON.to_string(),
                    },
                    "model_provider_policy_error_cleanup_failed",
                )
                .await
                .map_err(|cleanup| {
                    ModelProviderPolicyError::new(
                        "model_provider_policy_begin_and_cleanup_failed",
                        format!(
                            "begin failed ({origin}); cleanup failed ({})",
                            cleanup.detail()
                        ),
                    )
                })?;
                return Err(error);
            }
        }
    }

    if leases.is_empty() {
        Ok(ModelProviderPolicyBegin::NoPolicy)
    } else {
        Ok(ModelProviderPolicyBegin::Allow {
            lease: Box::new(CompositeModelProviderAttemptLease { leases }),
        })
    }
}

struct CompositeModelProviderAttemptLease {
    leases: Vec<Box<dyn ModelProviderAttemptLease>>,
}

impl ModelProviderAttemptLease for CompositeModelProviderAttemptLease {
    fn finish(
        self: Box<Self>,
        terminal: ModelProviderTerminal,
    ) -> ModelProviderPolicyFuture<'static, ()> {
        Box::pin(finish_leases(
            self.leases,
            terminal,
            "model_provider_policy_terminal_failed",
        ))
    }
}

async fn finish_leases(
    leases: Vec<Box<dyn ModelProviderAttemptLease>>,
    terminal: ModelProviderTerminal,
    aggregate_reason_code: &'static str,
) -> Result<(), ModelProviderPolicyError> {
    let mut failures = Vec::new();
    for lease in leases {
        if let Err(error) = lease.finish(terminal.clone()).await {
            failures.push(format!("{}: {}", error.reason_code(), error.detail()));
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(ModelProviderPolicyError::new(
            aggregate_reason_code,
            failures.join("; "),
        ))
    }
}

fn copy_input<'a>(input: &ModelProviderInvocationInput<'a>) -> ModelProviderInvocationInput<'a> {
    ModelProviderInvocationInput {
        schema_version: input.schema_version,
        session_store: input.session_store,
        thread_store: input.thread_store,
        turn_store: input.turn_store,
        attempt_id: input.attempt_id,
        request_binding_id: input.request_binding_id,
        thread_id: input.thread_id,
        turn_id: input.turn_id,
        request_kind: input.request_kind,
        provider_id: input.provider_id,
        provider_config_sha256: input.provider_config_sha256,
        model: input.model,
        transport: input.transport,
        endpoint_sha256: input.endpoint_sha256,
        logical_request_sha256: input.logical_request_sha256,
        wire_semantic_sha256: input.wire_semantic_sha256,
        previous_response_id_sha256: input.previous_response_id_sha256,
        generate: input.generate,
    }
}

#[cfg(test)]
#[path = "lifecycle_tests.rs"]
mod tests;
