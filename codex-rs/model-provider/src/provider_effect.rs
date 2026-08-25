//! Provider-effect transport seam.
//!
//! The normal Responses/Bedrock model transports are request/response APIs;
//! neither currently exposes a provider-visible occurrence key, durable
//! status lookup, and key+payload-bound effect acknowledgement.  This module
//! therefore provides only two safe pieces today: a canonical header builder
//! for a future qualified adapter, and a fail-closed adapter that refuses to
//! dispatch when the provider contract is not qualified.

use codex_hepta_contracts::ProviderEffectAdapter;
use codex_hepta_contracts::ProviderEffectBindingError;
use codex_hepta_contracts::ProviderEffectDispatch;
use codex_hepta_contracts::ProviderEffectFuture;
use codex_hepta_contracts::ProviderEffectIdempotencyCapability;
use codex_hepta_contracts::ProviderEffectIntent;
use codex_hepta_contracts::ProviderEffectKey;
use codex_hepta_contracts::ProviderEffectLookup;
use codex_hepta_contracts::Sha256Digest;
use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;

/// Header carrying the stable occurrence identity to a provider adapter.
pub const PROVIDER_EFFECT_IDEMPOTENCY_KEY_HEADER: &str = "idempotency-key";
/// Header carrying the exact payload binding alongside the occurrence key.
pub const PROVIDER_EFFECT_PAYLOAD_SHA256_HEADER: &str = "x-hepta-effect-payload-sha256";
/// Header identifying the version of the effect binding protocol.
pub const PROVIDER_EFFECT_SCHEMA_VERSION_HEADER: &str = "x-hepta-effect-schema-version";

/// The metadata a future HTTP/WS adapter must attach to every physical send.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderEffectDispatch {
    pub key: ProviderEffectKey,
    /// The exact bytes whose digest is carried by [`Self::headers`].
    ///
    /// Keeping the body and headers in one owned value prevents a future
    /// transport adapter from preparing one byte sequence and sending another
    /// after this qualification boundary returns.
    pub payload: Vec<u8>,
    pub payload_sha256: String,
    pub headers: HeaderMap,
}

/// Binds the exact wire payload to the durable effect intent and its headers.
///
/// This helper does not send a request and does not imply that the destination
/// honors the headers.  A provider may be marked `KeyAndStatusLookup` only
/// after an external contract and independent qualification prove that fact.
pub fn prepare_provider_effect_dispatch(
    intent: &ProviderEffectIntent,
    wire_payload: &[u8],
) -> Result<PreparedProviderEffectDispatch, ProviderEffectBindingError> {
    intent.validate()?;
    let payload_sha256 = Sha256Digest::for_bytes(wire_payload);
    if payload_sha256 != intent.payload_sha256 {
        return Err(ProviderEffectBindingError::PayloadMismatch);
    }
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static(PROVIDER_EFFECT_IDEMPOTENCY_KEY_HEADER),
        HeaderValue::from_str(intent.key.as_str())
            .map_err(|error| ProviderEffectBindingError::InvalidKey(error.to_string()))?,
    );
    headers.insert(
        HeaderName::from_static(PROVIDER_EFFECT_PAYLOAD_SHA256_HEADER),
        HeaderValue::from_str(payload_sha256.as_str())
            .map_err(|error| ProviderEffectBindingError::InvalidDigest(error.to_string()))?,
    );
    headers.insert(
        HeaderName::from_static(PROVIDER_EFFECT_SCHEMA_VERSION_HEADER),
        HeaderValue::from_static("1"),
    );
    Ok(PreparedProviderEffectDispatch {
        key: intent.key.clone(),
        payload: wire_payload.to_vec(),
        payload_sha256: payload_sha256.as_str().to_string(),
        headers,
    })
}

/// Adapter used by configured providers until an independently qualified
/// effect contract exists.  It never crosses a provider transport boundary.
#[derive(Clone, Copy, Debug, Default)]
pub struct FailClosedProviderEffectAdapter;

impl ProviderEffectAdapter for FailClosedProviderEffectAdapter {
    fn capability(&self) -> ProviderEffectIdempotencyCapability {
        ProviderEffectIdempotencyCapability::Unsupported
    }

    fn dispatch<'a>(
        &'a self,
        intent: &'a ProviderEffectIntent,
    ) -> ProviderEffectFuture<'a, ProviderEffectDispatch> {
        Box::pin(async move {
            if intent.validate().is_err() {
                return ProviderEffectDispatch::NotDispatched {
                    reason_code: "invalid_provider_effect_intent".to_string(),
                };
            }
            ProviderEffectDispatch::NotDispatched {
                reason_code: "provider_effect_capability_unsupported".to_string(),
            }
        })
    }

    fn lookup<'a>(
        &'a self,
        _key: &'a ProviderEffectKey,
    ) -> ProviderEffectFuture<'a, ProviderEffectLookup> {
        Box::pin(async { ProviderEffectLookup::Unknown })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_hepta_contracts::PROVIDER_EVIDENCE_SCHEMA_VERSION;
    use codex_hepta_contracts::ProviderRequestBinding;
    use codex_hepta_contracts::ProviderRequestKind;
    use codex_hepta_contracts::ProviderTransport;
    use codex_hepta_contracts::RequestBindingId;
    use codex_hepta_contracts::Sha256Digest;

    fn intent() -> ProviderEffectIntent {
        let binding = ProviderRequestBinding {
            schema_version: PROVIDER_EVIDENCE_SCHEMA_VERSION,
            thread_id: "thread-effect".to_string(),
            turn_id: "turn-effect".to_string(),
            host_request_binding_id_sha256: Sha256Digest::for_bytes(b"host"),
            request_kind: ProviderRequestKind::Turn,
            provider_id: "provider-effect".to_string(),
            provider_config_sha256: Sha256Digest::for_bytes(b"config"),
            model: "model".to_string(),
            transport: ProviderTransport::Http,
            endpoint_sha256: Sha256Digest::for_bytes(b"endpoint"),
            logical_request_sha256: Sha256Digest::for_bytes(b"logical"),
            wire_semantic_sha256: Sha256Digest::for_bytes(b"wire"),
            ephemeral_input_sha256: None,
            ephemeral_input_witness_sha256: None,
            previous_response_id_sha256: None,
            generate: true,
        };
        let request_binding_id = RequestBindingId::for_request(&binding);
        let key = ProviderEffectKey::for_occurrence(
            "provider-effect/config-v1",
            "occurrence-1",
            &request_binding_id,
        )
        .expect("key");
        ProviderEffectIntent::new(key, Sha256Digest::for_bytes(b"payload"))
    }

    #[test]
    fn headers_bind_stable_key_and_exact_payload_digest() {
        let wire_payload = b"payload";
        let prepared =
            prepare_provider_effect_dispatch(&intent(), wire_payload).expect("prepared dispatch");
        assert_eq!(prepared.payload, wire_payload);
        assert_eq!(
            prepared
                .headers
                .get(PROVIDER_EFFECT_IDEMPOTENCY_KEY_HEADER)
                .expect("idempotency key")
                .to_str()
                .expect("header value"),
            prepared.key.as_str()
        );
        assert_eq!(
            prepared
                .headers
                .get(PROVIDER_EFFECT_PAYLOAD_SHA256_HEADER)
                .expect("payload digest")
                .to_str()
                .expect("header value"),
            prepared.payload_sha256
        );
        assert_eq!(
            prepared
                .headers
                .get(PROVIDER_EFFECT_SCHEMA_VERSION_HEADER)
                .expect("schema version"),
            "1"
        );
    }

    #[test]
    fn mismatched_wire_payload_is_rejected_before_headers_are_prepared() {
        assert_eq!(
            prepare_provider_effect_dispatch(&intent(), b"different-payload"),
            Err(ProviderEffectBindingError::PayloadMismatch)
        );
    }

    #[tokio::test]
    async fn fail_closed_adapter_never_claims_or_fakes_provider_success() {
        let adapter = FailClosedProviderEffectAdapter;
        assert_eq!(
            adapter.capability(),
            ProviderEffectIdempotencyCapability::Unsupported
        );
        let dispatch = adapter.dispatch(&intent()).await;
        assert_eq!(
            dispatch,
            ProviderEffectDispatch::NotDispatched {
                reason_code: "provider_effect_capability_unsupported".to_string()
            }
        );
        assert_eq!(
            adapter.lookup(&intent().key).await,
            ProviderEffectLookup::Unknown
        );
    }
}
