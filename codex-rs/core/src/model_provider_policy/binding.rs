use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionRegistry;
use codex_extension_api::ModelProviderInvocationInput;
use codex_extension_api::ModelProviderPolicyError;
use codex_extension_api::ModelProviderRequestKind;
use codex_extension_api::ModelProviderSha256Digest;
use codex_extension_api::ModelProviderTransport;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest as _;
use sha2::Sha256;
use uuid::Uuid;

use crate::config::Config;

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
    // provider credentials or configuration derived from them.
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
/// `wire_semantic` must include every stable, behavior-affecting wire value
/// outside the encoded request body, including routing-hint header presence and
/// value. Authentication, attestation, trace, timing, and request-ID material
/// must remain excluded.
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
    // Do not hash provider configuration here. It may contain bearer/header/
    // query/retry configuration, so even its digest would make evidence
    // secret-derived and unstable across credential rotation.
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
        // Host identity is retry-stable across HTTP/WebSocket fallback and
        // incremental/full encodings. Hepta evidence additionally binds the
        // physical transport, wire semantics, endpoint and previous response.
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
    ModelProviderSha256Digest::parse(format!("{:x}", Sha256::digest(bytes)))
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
/// Query values, URL credentials, and fragments are deliberately excluded.
/// Query names are sorted so map iteration order cannot perturb evidence.
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

#[cfg(test)]
#[path = "binding_tests.rs"]
mod tests;
