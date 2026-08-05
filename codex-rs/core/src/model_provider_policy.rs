use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionRegistry;
use codex_extension_api::ModelProviderAttemptLease;
use codex_extension_api::ModelProviderInvocationInput;
use codex_extension_api::ModelProviderPolicyDecision;
use codex_extension_api::ModelProviderPolicyError;
use codex_extension_api::ModelProviderPolicyFuture;
use codex_extension_api::ModelProviderRequestKind;
use codex_extension_api::ModelProviderSha256Digest;
use codex_extension_api::ModelProviderTerminal;
use codex_extension_api::ModelProviderTransport;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest as _;
use sha2::Sha256;
use uuid::Uuid;

use crate::config::Config;

const BLOCKED_CLEANUP_REASON: &str = "model_provider_policy_blocked";
const ERROR_CLEANUP_REASON: &str = "model_provider_policy_begin_failed";

/// Result of aggregating all active model-provider policy contributors.
///
/// `NoPolicy` is deliberately distinct from `Allow`: callers can skip all
/// provider-policy request binding and terminal work when no contributor is
/// active, preserving the feature-off provider path.
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

/// Extension scopes and host identities required to bind one provider send.
///
/// The context owns only secret-free IDs. Extension stores remain borrowed
/// from Codex's single session/thread/turn lifecycle.
pub(crate) struct ModelProviderPolicyContext<'a> {
    pub(crate) registry: &'a ExtensionRegistry<Config>,
    pub(crate) session_store: &'a ExtensionData,
    pub(crate) thread_store: &'a ExtensionData,
    pub(crate) turn_store: &'a ExtensionData,
    pub(crate) thread_id: String,
    pub(crate) turn_id: String,
    pub(crate) request_kind: ModelProviderRequestKind,
}

/// Owned, digest-only material for one physical provider send.
///
/// Raw request/provider values are used only while constructing this value and
/// are never exposed across the Extension API seam.
pub(crate) struct PreparedModelProviderPolicy {
    attempt_id: String,
    request_binding_id: String,
    provider_id: String,
    // Compatibility-named Extension API field. This is deliberately the
    // digest of the stable, secret-free provider selector, never a digest of
    // `ModelProviderInfo` or any credentials/configuration derived from it.
    provider_config_sha256: ModelProviderSha256Digest,
    model: String,
    transport: ModelProviderTransport,
    endpoint_sha256: ModelProviderSha256Digest,
    logical_request_sha256: ModelProviderSha256Digest,
    wire_semantic_sha256: ModelProviderSha256Digest,
    previous_response_id_sha256: Option<ModelProviderSha256Digest>,
    generate: bool,
}

impl PreparedModelProviderPolicy {
    pub(crate) fn invocation_input<'a>(
        &'a self,
        context: &'a ModelProviderPolicyContext<'a>,
    ) -> ModelProviderInvocationInput<'a> {
        ModelProviderInvocationInput {
            schema_version: codex_extension_api::MODEL_PROVIDER_POLICY_INPUT_SCHEMA_VERSION,
            session_store: context.session_store,
            thread_store: context.thread_store,
            turn_store: context.turn_store,
            attempt_id: &self.attempt_id,
            request_binding_id: &self.request_binding_id,
            thread_id: &context.thread_id,
            turn_id: &context.turn_id,
            request_kind: context.request_kind,
            provider_id: &self.provider_id,
            provider_config_sha256: &self.provider_config_sha256,
            model: &self.model,
            transport: self.transport,
            endpoint_sha256: &self.endpoint_sha256,
            logical_request_sha256: &self.logical_request_sha256,
            wire_semantic_sha256: &self.wire_semantic_sha256,
            previous_response_id_sha256: self.previous_response_id_sha256.as_ref(),
            generate: self.generate,
        }
    }
}

/// Builds the secret-free binding immediately before a provider send.
///
/// Callers must first check [`has_active_model_provider_policy`]. This keeps
/// ordinary Codex requests byte-for-byte on their existing fast path and avoids
/// serializing provider/request material when no policy is active.
#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare_model_provider_policy<L: Serialize, W: Serialize>(
    context: &ModelProviderPolicyContext<'_>,
    provider_id: &str,
    model: &str,
    transport: ModelProviderTransport,
    endpoint: &str,
    logical_request: &L,
    wire_semantic: &W,
    previous_response_id: Option<&str>,
    generate: bool,
) -> Result<PreparedModelProviderPolicy, ModelProviderPolicyError> {
    // Do not hash the provider configuration here. `ModelProviderInfo` may
    // contain bearer/header/query/retry configuration, so even persisting its
    // digest would make authoritative evidence secret-derived and unstable
    // across credential rotation. The compatibility-named API field carries
    // only a versioned digest of the stable logical selector.
    let provider_config_sha256 =
        digest_parts_sha256(["model-provider-logical-selector:v1", provider_id])?;
    let endpoint_sha256 = canonical_endpoint_sha256(endpoint)?;
    let logical_request_sha256 = canonical_sha256(logical_request)?;
    let wire_semantic_sha256 = canonical_sha256(wire_semantic)?;
    let previous_response_id_sha256 = previous_response_id
        .map(|value| bytes_sha256(value.as_bytes()))
        .transpose()?;
    let request_binding_digest = digest_parts([
        "model-provider-request:v1",
        context.thread_id.as_str(),
        context.turn_id.as_str(),
        request_kind_name(context.request_kind),
        provider_id,
        model,
        logical_request_sha256.as_str(),
        // The host binding is retry-stable across HTTP/WebSocket fallback and
        // incremental/full wire encodings. The Hepta evidence binding still
        // binds transport, wire semantics, and previous-response identity.
        if generate { "generate" } else { "no-generate" },
    ]);

    Ok(PreparedModelProviderPolicy {
        attempt_id: format!("model-provider-attempt:v1:{}", Uuid::new_v4()),
        request_binding_id: format!("model-provider-request:v1:{request_binding_digest}"),
        provider_id: provider_id.to_string(),
        provider_config_sha256,
        model: model.to_string(),
        transport,
        endpoint_sha256,
        logical_request_sha256,
        wire_semantic_sha256,
        previous_response_id_sha256,
        generate,
    })
}

pub(crate) fn canonical_sha256<T: Serialize>(
    value: &T,
) -> Result<ModelProviderSha256Digest, ModelProviderPolicyError> {
    let value = serde_json::to_value(value).map_err(|error| {
        ModelProviderPolicyError::new(
            "model_provider_policy_serialization_failed",
            format!("failed to serialize provider semantic material: {error}"),
        )
    })?;
    let canonical = canonicalize_json(value);
    let bytes = serde_json::to_vec(&canonical).map_err(|error| {
        ModelProviderPolicyError::new(
            "model_provider_policy_serialization_failed",
            format!("failed to encode provider semantic material: {error}"),
        )
    })?;
    bytes_sha256(&bytes)
}

pub(crate) fn bytes_sha256(
    bytes: &[u8],
) -> Result<ModelProviderSha256Digest, ModelProviderPolicyError> {
    let digest = format!("{:x}", Sha256::digest(bytes));
    ModelProviderSha256Digest::parse(digest)
}

fn canonicalize_json(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonicalize_json).collect()),
        Value::Object(values) => {
            let mut entries = values.into_iter().collect::<Vec<_>>();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            Value::Object(
                entries
                    .into_iter()
                    .map(|(key, value)| (key, canonicalize_json(value)))
                    .collect(),
            )
        }
        scalar => scalar,
    }
}

fn digest_parts<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn digest_parts_sha256<'a>(
    parts: impl IntoIterator<Item = &'a str>,
) -> Result<ModelProviderSha256Digest, ModelProviderPolicyError> {
    ModelProviderSha256Digest::parse(digest_parts(parts))
}

/// Digests only the non-secret routing shape of a provider endpoint.
///
/// Query parameter values, URL credentials, and fragments are deliberately
/// excluded: they may contain credentials or volatile request configuration.
/// Query names are sorted so `HashMap` iteration order cannot perturb physical
/// attempt evidence. The logical request binding does not contain this digest.
fn canonical_endpoint_sha256(
    endpoint: &str,
) -> Result<ModelProviderSha256Digest, ModelProviderPolicyError> {
    #[derive(Serialize)]
    struct CanonicalEndpoint<'a> {
        schema: &'static str,
        scheme: &'a str,
        host: Option<&'a str>,
        port: Option<u16>,
        path: &'a str,
        query_names: Vec<String>,
    }

    let parsed = url::Url::parse(endpoint).map_err(|error| {
        ModelProviderPolicyError::new(
            "model_provider_policy_invalid_endpoint",
            format!("failed to parse provider endpoint URL: {error}"),
        )
    })?;
    let mut query_names = parsed
        .query_pairs()
        .map(|(name, _value)| name.into_owned())
        .collect::<Vec<_>>();
    query_names.sort();

    canonical_sha256(&CanonicalEndpoint {
        schema: "model-provider-endpoint:v1",
        scheme: parsed.scheme(),
        host: parsed.host_str(),
        port: parsed.port_or_known_default(),
        path: parsed.path(),
        query_names,
    })
}

const fn request_kind_name(kind: ModelProviderRequestKind) -> &'static str {
    match kind {
        ModelProviderRequestKind::Turn => "turn",
        ModelProviderRequestKind::Prewarm => "prewarm",
        ModelProviderRequestKind::Compaction => "compaction",
        ModelProviderRequestKind::Memory => "memory",
    }
}

/// Returns whether constructing a provider-policy binding is necessary.
///
/// Callers must use this before computing provider-policy-only digests. Merely
/// registering an inactive contributor must not add work to the provider path.
pub(crate) fn has_active_model_provider_policy<C: Sync>(
    registry: &ExtensionRegistry<C>,
    thread_store: &ExtensionData,
) -> bool {
    registry
        .model_provider_policy_contributors()
        .iter()
        .any(|contributor| contributor.is_active(thread_store))
}

/// Runs active provider policy contributors in registration order.
///
/// Every acquired lease is either returned as one opaque, single-use
/// composite lease or completed as `NotDispatched` before a later block/error
/// is returned. Cleanup and terminal failures are fail-closed and surfaced.
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
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex;

    use codex_extension_api::ExtensionRegistryBuilder;
    use codex_extension_api::MODEL_PROVIDER_POLICY_INPUT_SCHEMA_VERSION;
    use codex_extension_api::ModelProviderPolicyContributor;
    use codex_extension_api::ModelProviderRequestKind;
    use codex_extension_api::ModelProviderSha256Digest;
    use codex_extension_api::ModelProviderTransport;

    use super::*;

    #[derive(Clone)]
    enum Behavior {
        Allow { finish_error: bool },
        Block,
        Error,
    }

    struct RecordingContributor {
        name: &'static str,
        active: bool,
        behavior: Behavior,
        events: Arc<Mutex<Vec<String>>>,
    }

    impl ModelProviderPolicyContributor for RecordingContributor {
        fn is_active(&self, _thread_store: &ExtensionData) -> bool {
            self.active
        }

        fn begin<'a>(
            &'a self,
            _input: ModelProviderInvocationInput<'a>,
        ) -> ModelProviderPolicyFuture<'a, ModelProviderPolicyDecision> {
            self.events
                .lock()
                .expect("events lock should not be poisoned")
                .push(format!("begin:{}", self.name));
            let result = match self.behavior {
                Behavior::Allow { finish_error } => Ok(ModelProviderPolicyDecision::Allow {
                    lease: Box::new(RecordingLease {
                        name: self.name,
                        finish_error,
                        events: Arc::clone(&self.events),
                    }),
                }),
                Behavior::Block => Ok(ModelProviderPolicyDecision::Block {
                    reason_code: format!("{}_blocked", self.name),
                    message: format!("{} blocked", self.name),
                }),
                Behavior::Error => Err(ModelProviderPolicyError::new(
                    format!("{}_error", self.name),
                    format!("{} failed", self.name),
                )),
            };
            Box::pin(std::future::ready(result))
        }
    }

    struct RecordingLease {
        name: &'static str,
        finish_error: bool,
        events: Arc<Mutex<Vec<String>>>,
    }

    impl ModelProviderAttemptLease for RecordingLease {
        fn finish(
            self: Box<Self>,
            terminal: ModelProviderTerminal,
        ) -> ModelProviderPolicyFuture<'static, ()> {
            self.events
                .lock()
                .expect("events lock should not be poisoned")
                .push(format!("finish:{}:{terminal:?}", self.name));
            let result = if self.finish_error {
                Err(ModelProviderPolicyError::new(
                    format!("{}_finish_error", self.name),
                    format!("{} finish failed", self.name),
                ))
            } else {
                Ok(())
            };
            Box::pin(std::future::ready(result))
        }
    }

    fn contributor(
        name: &'static str,
        active: bool,
        behavior: Behavior,
        events: &Arc<Mutex<Vec<String>>>,
    ) -> Arc<dyn ModelProviderPolicyContributor> {
        Arc::new(RecordingContributor {
            name,
            active,
            behavior,
            events: Arc::clone(events),
        })
    }

    fn digest(byte: char) -> ModelProviderSha256Digest {
        ModelProviderSha256Digest::parse(byte.to_string().repeat(64))
            .expect("test digest should be valid")
    }

    fn input<'a>(
        session_store: &'a ExtensionData,
        thread_store: &'a ExtensionData,
        turn_store: &'a ExtensionData,
        digests: &'a [ModelProviderSha256Digest; 4],
    ) -> ModelProviderInvocationInput<'a> {
        ModelProviderInvocationInput {
            schema_version: MODEL_PROVIDER_POLICY_INPUT_SCHEMA_VERSION,
            session_store,
            thread_store,
            turn_store,
            attempt_id: "attempt-1",
            request_binding_id: "binding-1",
            thread_id: "thread-1",
            turn_id: "turn-1",
            request_kind: ModelProviderRequestKind::Turn,
            provider_id: "provider-1",
            provider_config_sha256: &digests[0],
            model: "model-1",
            transport: ModelProviderTransport::Http,
            endpoint_sha256: &digests[1],
            logical_request_sha256: &digests[2],
            wire_semantic_sha256: &digests[3],
            previous_response_id_sha256: None,
            generate: true,
        }
    }

    fn stores() -> (ExtensionData, ExtensionData, ExtensionData) {
        (
            ExtensionData::new("session"),
            ExtensionData::new("thread"),
            ExtensionData::new("turn"),
        )
    }

    #[tokio::test]
    async fn inactive_contributors_produce_no_policy_and_do_not_begin() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let mut builder = ExtensionRegistryBuilder::<crate::config::Config>::new();
        builder.model_provider_policy_contributor(contributor(
            "inactive",
            false,
            Behavior::Allow {
                finish_error: false,
            },
            &events,
        ));
        let registry = builder.build();
        let (session_store, thread_store, turn_store) = stores();
        let digests = [digest('a'), digest('b'), digest('c'), digest('d')];

        assert!(!has_active_model_provider_policy(&registry, &thread_store));
        assert!(matches!(
            begin_model_provider_policy(
                &registry,
                input(&session_store, &thread_store, &turn_store, &digests),
            )
            .await
            .expect("inactive contributors should not fail"),
            ModelProviderPolicyBegin::NoPolicy
        ));
        assert!(
            events
                .lock()
                .expect("events lock should not be poisoned")
                .is_empty()
        );
    }

    #[tokio::test]
    async fn active_contributors_begin_and_finish_in_registration_order() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let mut builder = ExtensionRegistryBuilder::<crate::config::Config>::new();
        for name in ["one", "two"] {
            builder.model_provider_policy_contributor(contributor(
                name,
                true,
                Behavior::Allow {
                    finish_error: false,
                },
                &events,
            ));
        }
        let registry = builder.build();
        let (session_store, thread_store, turn_store) = stores();
        let digests = [digest('a'), digest('b'), digest('c'), digest('d')];
        let result = begin_model_provider_policy(
            &registry,
            input(&session_store, &thread_store, &turn_store, &digests),
        )
        .await
        .expect("all contributors should allow");
        let ModelProviderPolicyBegin::Allow { lease } = result else {
            panic!("active allow contributors should produce a composite lease");
        };

        lease
            .finish(ModelProviderTerminal::NotDispatched {
                reason_code: "test_terminal".to_string(),
            })
            .await
            .expect("all child leases should finish");

        assert_eq!(
            events
                .lock()
                .expect("events lock should not be poisoned")
                .as_slice(),
            [
                "begin:one",
                "begin:two",
                "finish:one:NotDispatched { reason_code: \"test_terminal\" }",
                "finish:two:NotDispatched { reason_code: \"test_terminal\" }",
            ]
        );
    }

    #[tokio::test]
    async fn block_finishes_all_previously_acquired_leases() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let mut builder = ExtensionRegistryBuilder::<crate::config::Config>::new();
        builder.model_provider_policy_contributor(contributor(
            "allow",
            true,
            Behavior::Allow {
                finish_error: false,
            },
            &events,
        ));
        builder.model_provider_policy_contributor(contributor(
            "block",
            true,
            Behavior::Block,
            &events,
        ));
        let registry = builder.build();
        let (session_store, thread_store, turn_store) = stores();
        let digests = [digest('a'), digest('b'), digest('c'), digest('d')];

        let result = begin_model_provider_policy(
            &registry,
            input(&session_store, &thread_store, &turn_store, &digests),
        )
        .await
        .expect("cleanup should succeed");
        let ModelProviderPolicyBegin::Block {
            reason_code,
            message,
        } = result
        else {
            panic!("the block decision should be preserved");
        };
        assert_eq!(reason_code, "block_blocked");
        assert_eq!(message, "block blocked");
        assert_eq!(
            events
                .lock()
                .expect("events lock should not be poisoned")
                .as_slice(),
            [
                "begin:allow",
                "begin:block",
                "finish:allow:NotDispatched { reason_code: \"model_provider_policy_blocked\" }",
            ]
        );
    }

    #[tokio::test]
    async fn begin_error_finishes_previously_acquired_leases() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let mut builder = ExtensionRegistryBuilder::<crate::config::Config>::new();
        builder.model_provider_policy_contributor(contributor(
            "allow",
            true,
            Behavior::Allow {
                finish_error: false,
            },
            &events,
        ));
        builder.model_provider_policy_contributor(contributor(
            "error",
            true,
            Behavior::Error,
            &events,
        ));
        let registry = builder.build();
        let (session_store, thread_store, turn_store) = stores();
        let digests = [digest('a'), digest('b'), digest('c'), digest('d')];

        let result = begin_model_provider_policy(
            &registry,
            input(&session_store, &thread_store, &turn_store, &digests),
        )
        .await;
        let Err(error) = result else {
            panic!("begin error should fail closed");
        };
        assert_eq!(error.reason_code(), "error_error");
        assert_eq!(
            events
                .lock()
                .expect("events lock should not be poisoned")
                .as_slice(),
            [
                "begin:allow",
                "begin:error",
                "finish:allow:NotDispatched { reason_code: \"model_provider_policy_begin_failed\" }",
            ]
        );
    }

    #[tokio::test]
    async fn composite_finish_attempts_every_lease_and_surfaces_failures() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let mut builder = ExtensionRegistryBuilder::<crate::config::Config>::new();
        for (name, finish_error) in [("bad", true), ("good", false), ("also_bad", true)] {
            builder.model_provider_policy_contributor(contributor(
                name,
                true,
                Behavior::Allow { finish_error },
                &events,
            ));
        }
        let registry = builder.build();
        let (session_store, thread_store, turn_store) = stores();
        let digests = [digest('a'), digest('b'), digest('c'), digest('d')];
        let result = begin_model_provider_policy(
            &registry,
            input(&session_store, &thread_store, &turn_store, &digests),
        )
        .await
        .expect("all contributors should begin");
        let ModelProviderPolicyBegin::Allow { lease } = result else {
            panic!("active allow contributors should produce a composite lease");
        };

        let error = lease
            .finish(ModelProviderTerminal::NotDispatched {
                reason_code: "test_terminal".to_string(),
            })
            .await
            .expect_err("any child failure should fail the composite terminal");
        assert_eq!(error.reason_code(), "model_provider_policy_terminal_failed");
        assert!(error.detail().contains("bad_finish_error"));
        assert!(error.detail().contains("also_bad_finish_error"));
        assert!(
            events
                .lock()
                .expect("events lock should not be poisoned")
                .iter()
                .any(|event| event.starts_with("finish:good:"))
        );
    }

    #[test]
    fn canonical_digest_is_independent_of_object_key_order() {
        let left: Value =
            serde_json::from_str(r#"{"b":{"y":2,"x":1},"a":0}"#).expect("left JSON should parse");
        let right: Value =
            serde_json::from_str(r#"{"a":0,"b":{"x":1,"y":2}}"#).expect("right JSON should parse");

        assert_eq!(
            canonical_sha256(&left).expect("left digest should succeed"),
            canonical_sha256(&right).expect("right digest should succeed")
        );
    }

    #[test]
    fn retry_stable_request_binding_has_fresh_per_send_attempt_identity() {
        let registry = ExtensionRegistryBuilder::<crate::config::Config>::new().build();
        let (session_store, thread_store, turn_store) = stores();
        let context = ModelProviderPolicyContext {
            registry: &registry,
            session_store: &session_store,
            thread_store: &thread_store,
            turn_store: &turn_store,
            thread_id: "thread-1".to_string(),
            turn_id: "turn-1".to_string(),
            request_kind: ModelProviderRequestKind::Turn,
        };
        let request = serde_json::json!({"input": [1, 2, 3], "model": "model-1"});
        let incremental = serde_json::json!({"input": [3], "previous_response_id": "prior"});

        let first = prepare_model_provider_policy(
            &context,
            "provider-1",
            "model-1",
            ModelProviderTransport::WebSocket,
            "https://user:secret@example.test/responses?region=one&api_key=secret-one",
            &request,
            &incremental,
            Some("prior"),
            true,
        )
        .expect("first binding should succeed");
        let second = prepare_model_provider_policy(
            &context,
            "provider-1",
            "model-1",
            ModelProviderTransport::Http,
            "https://other:rotated@example.test/responses?api_key=rotated&region=two",
            &request,
            &request,
            None,
            true,
        )
        .expect("second binding should succeed");
        let different_endpoint = prepare_model_provider_policy(
            &context,
            "provider-1",
            "model-1",
            ModelProviderTransport::Http,
            "https://alternate.example/v2/responses?token=another-secret",
            &request,
            &request,
            None,
            true,
        )
        .expect("different physical endpoint should still bind");

        assert_eq!(first.request_binding_id, second.request_binding_id);
        assert_eq!(
            first.request_binding_id,
            different_endpoint.request_binding_id
        );
        assert_ne!(first.attempt_id, second.attempt_id);
        assert_eq!(first.provider_config_sha256, second.provider_config_sha256);
        assert_eq!(first.endpoint_sha256, second.endpoint_sha256);
        assert_ne!(first.endpoint_sha256, different_endpoint.endpoint_sha256);
        assert_eq!(first.logical_request_sha256, second.logical_request_sha256);
        assert_ne!(first.wire_semantic_sha256, second.wire_semantic_sha256);
    }

    #[test]
    fn endpoint_digest_sorts_query_names_and_excludes_values_and_credentials() {
        let left = canonical_endpoint_sha256(
            "https://alice:secret@example.test/v1/responses?z=secret-z&a=secret-a&z=again",
        )
        .expect("left endpoint should parse");
        let right = canonical_endpoint_sha256(
            "https://bob:rotated@example.test/v1/responses?z=changed&z=other&a=changed",
        )
        .expect("right endpoint should parse");
        let different_path = canonical_endpoint_sha256(
            "https://example.test/v2/responses?a=changed&z=changed&z=other",
        )
        .expect("different endpoint should parse");

        assert_eq!(left, right);
        assert_ne!(left, different_path);
    }

    #[test]
    fn logical_provider_selector_changes_binding_and_selector_digest() {
        let registry = ExtensionRegistryBuilder::<crate::config::Config>::new().build();
        let (session_store, thread_store, turn_store) = stores();
        let context = ModelProviderPolicyContext {
            registry: &registry,
            session_store: &session_store,
            thread_store: &thread_store,
            turn_store: &turn_store,
            thread_id: "thread-1".to_string(),
            turn_id: "turn-1".to_string(),
            request_kind: ModelProviderRequestKind::Turn,
        };
        let request = serde_json::json!({"input": [1], "model": "model-1"});
        let first = prepare_model_provider_policy(
            &context,
            "provider-1",
            "model-1",
            ModelProviderTransport::Http,
            "https://example.test/responses",
            &request,
            &request,
            None,
            true,
        )
        .expect("first binding should succeed");
        let second = prepare_model_provider_policy(
            &context,
            "provider-2",
            "model-1",
            ModelProviderTransport::Http,
            "https://example.test/responses",
            &request,
            &request,
            None,
            true,
        )
        .expect("second binding should succeed");

        assert_ne!(first.request_binding_id, second.request_binding_id);
        assert_ne!(first.provider_config_sha256, second.provider_config_sha256);
    }
}
