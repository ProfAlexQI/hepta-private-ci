//! Provider-effect transport seam.
//!
//! The normal Responses/Bedrock model transports are request/response APIs;
//! neither currently exposes a provider-visible occurrence key, durable
//! status lookup, and key+payload-bound effect acknowledgement.  This module
//! therefore provides only two safe pieces today: a canonical header builder
//! for a future qualified adapter, and a fail-closed adapter that refuses to
//! dispatch when the provider contract is not qualified.

use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

use codex_hepta_contracts::ProviderEffectAdapter;
use codex_hepta_contracts::ProviderEffectAck;
use codex_hepta_contracts::ProviderEffectAckStatus;
use codex_hepta_contracts::ProviderEffectBindingError;
use codex_hepta_contracts::ProviderEffectDispatch;
use codex_hepta_contracts::ProviderEffectFuture;
use codex_hepta_contracts::ProviderEffectIdempotencyCapability;
use codex_hepta_contracts::ProviderEffectIntent;
use codex_hepta_contracts::ProviderEffectKey;
use codex_hepta_contracts::ProviderEffectLookup;
use codex_hepta_contracts::Sha256Digest;
use codex_http_client::ClientRouteClass;
use codex_http_client::HttpClient;
use codex_http_client::HttpClientFactory;
use codex_http_client::OutboundProxyPolicy;
use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;
use serde::Deserialize;
use url::Url;

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

/// Explicit provider-owned HTTP effect contract configuration.
#[derive(Clone)]
pub struct HttpProviderEffectConfig {
    pub dispatch_url: String,
    /// Exactly one `{key}` placeholder is required.
    pub lookup_url_template: String,
    /// Authentication/provider headers supplied by the external authority.
    /// Binding headers generated from the intent override duplicates.
    pub headers: HeaderMap,
    pub timeout: Duration,
    pub contract_id: String,
    /// Pinned external contract evidence. The adapter refuses to construct
    /// without this value; the normal provider factory never creates it.
    pub attestation: Option<HttpProviderEffectContractAttestation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpProviderEffectContractAttestation {
    pub contract_id: String,
    pub contract_sha256: Sha256Digest,
    pub authority_epoch: u64,
}

impl fmt::Debug for HttpProviderEffectConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpProviderEffectConfig")
            .field("dispatch_url", &self.dispatch_url)
            .field("lookup_url_template", &self.lookup_url_template)
            .field("headers", &"<redacted>")
            .field("timeout", &self.timeout)
            .field("contract_id", &self.contract_id)
            .field("attestation", &self.attestation)
            .finish()
    }
}

#[derive(Clone)]
pub struct HttpProviderEffectAdapter {
    client: HttpClient,
    config: HttpProviderEffectConfig,
}

impl fmt::Debug for HttpProviderEffectAdapter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpProviderEffectAdapter")
            .field("config", &self.config)
            .finish()
    }
}

impl HttpProviderEffectAdapter {
    /// Builds an adapter only when an external authority has pinned the
    /// endpoint contract. HTTP is accepted solely for loopback test fixtures;
    /// non-loopback production endpoints must use HTTPS.
    pub fn new(config: HttpProviderEffectConfig) -> Result<Self, String> {
        if config.dispatch_url.trim().is_empty()
            || config.dispatch_url.contains("{key}")
            || config.lookup_url_template.matches("{key}").count() != 1
        {
            return Err("invalid dispatch/lookup URL template".to_string());
        }
        if config.contract_id.trim().is_empty() || config.contract_id.len() > 128 {
            return Err("contract_id must be 1..=128 bytes".to_string());
        }
        let attestation = config
            .attestation
            .as_ref()
            .ok_or_else(|| "external contract attestation is required".to_string())?;
        if attestation.contract_id != config.contract_id || attestation.authority_epoch == 0 {
            return Err("contract attestation binding is invalid".to_string());
        }
        Sha256Digest::parse(attestation.contract_sha256.as_str().to_string())
            .map_err(|error| format!("invalid contract digest: {error}"))?;
        if config.timeout.is_zero() || config.timeout > Duration::from_secs(300) {
            return Err("timeout must be between 1ms and 300s".to_string());
        }
        let dispatch_endpoint = validate_effect_endpoint(&config.dispatch_url)?;
        let lookup_endpoint = validate_effect_endpoint(
            &config.lookup_url_template.replace("{key}", "hepta-key"),
        )?;
        if dispatch_endpoint.scheme() != lookup_endpoint.scheme()
            || dispatch_endpoint.host_str() != lookup_endpoint.host_str()
            || dispatch_endpoint.port_or_known_default() != lookup_endpoint.port_or_known_default()
        {
            return Err("dispatch and lookup endpoints must share one origin".to_string());
        }
        let factory = HttpClientFactory::new(OutboundProxyPolicy::ReqwestDefault);
        let client = factory
            .build_client_without_request_logging(&config.dispatch_url, ClientRouteClass::Api)
            .map_err(|error| format!("invalid dispatch endpoint: {error}"))?;
        Ok(Self { client, config })
    }

    pub fn config(&self) -> &HttpProviderEffectConfig {
        &self.config
    }

    async fn body(response: codex_http_client::HttpResponse) -> Option<Vec<u8>> {
        let bytes = response.bytes().await.ok()?;
        (bytes.len() <= 65_536).then(|| bytes.to_vec())
    }

    async fn dispatch_http(
        &self,
        intent: &ProviderEffectIntent,
        wire_payload: &[u8],
    ) -> ProviderEffectDispatch {
        let prepared = match prepare_provider_effect_dispatch(intent, wire_payload) {
            Ok(value) => value,
            Err(_) => {
                return ProviderEffectDispatch::NotDispatched {
                    reason_code: "provider_effect_payload_binding_invalid".to_string(),
                }
            }
        };
        let mut headers = self.config.headers.clone();
        for (name, value) in &prepared.headers {
            headers.insert(name.clone(), value.clone());
        }
        let response = match self
            .client
            .post(&self.config.dispatch_url)
            .headers(headers)
            .timeout(self.config.timeout)
            .body(prepared.payload)
            .send()
            .await
        {
            Ok(value) => value,
            Err(_) => return ProviderEffectDispatch::Unknown,
        };
        if !response.status().is_success() {
            return ProviderEffectDispatch::Unknown;
        }
        let body = match Self::body(response).await {
            Some(value) => value,
            None => return ProviderEffectDispatch::Unknown,
        };
        match parse_wire_ack(&body).and_then(|ack| {
            ack.validate_for(intent).map(|_| ack).map_err(|error| format!("{error:?}"))
        }) {
            Ok(ack) => ProviderEffectDispatch::Ack(ack),
            Err(_) => ProviderEffectDispatch::Unknown,
        }
    }

    async fn lookup_http(&self, key: &ProviderEffectKey) -> ProviderEffectLookup {
        let url = self
            .config
            .lookup_url_template
            .replace("{key}", &encode_path_segment(key.as_str()));
        let mut headers = self.config.headers.clone();
        if let Ok(value) = HeaderValue::from_str(key.as_str()) {
            headers.insert(HeaderName::from_static(PROVIDER_EFFECT_IDEMPOTENCY_KEY_HEADER), value);
        }
        headers.insert(
            HeaderName::from_static(PROVIDER_EFFECT_SCHEMA_VERSION_HEADER),
            HeaderValue::from_static("1"),
        );
        let response = match self
            .client
            .get(url)
            .headers(headers)
            .timeout(self.config.timeout)
            .send()
            .await
        {
            Ok(value) => value,
            Err(_) => return ProviderEffectLookup::Unknown,
        };
        let status = response.status().as_u16();
        let body = Self::body(response).await;
        if status == 404 {
            return ProviderEffectLookup::NotFound;
        }
        if status == 409 {
            return ProviderEffectLookup::Conflict {
                observed_payload_sha256: body.as_deref().and_then(parse_wire_conflict_digest),
            };
        }
        if !(200..300).contains(&status) {
            return ProviderEffectLookup::Unknown;
        }
        body.as_deref()
            .and_then(|bytes| parse_wire_ack(bytes).ok())
            .map(ProviderEffectLookup::Ack)
            .unwrap_or(ProviderEffectLookup::Unknown)
    }
}

impl ProviderEffectAdapter for HttpProviderEffectAdapter {
    fn capability(&self) -> ProviderEffectIdempotencyCapability {
        if self.config.attestation.is_some() {
            ProviderEffectIdempotencyCapability::KeyAndStatusLookup
        } else {
            ProviderEffectIdempotencyCapability::Unsupported
        }
    }

    fn dispatch<'a>(
        &'a self,
        _intent: &'a ProviderEffectIntent,
    ) -> ProviderEffectFuture<'a, ProviderEffectDispatch> {
        Box::pin(async {
            ProviderEffectDispatch::NotDispatched {
                reason_code: "provider_effect_wire_payload_required".to_string(),
            }
        })
    }

    fn dispatch_with_payload<'a>(
        &'a self,
        intent: &'a ProviderEffectIntent,
        wire_payload: &'a [u8],
    ) -> ProviderEffectFuture<'a, ProviderEffectDispatch> {
        Box::pin(async move { self.dispatch_http(intent, wire_payload).await })
    }

    fn lookup<'a>(
        &'a self,
        key: &'a ProviderEffectKey,
    ) -> ProviderEffectFuture<'a, ProviderEffectLookup> {
        Box::pin(async move { self.lookup_http(key).await })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireAck {
    effect_key: String,
    payload_sha256: String,
    provider_operation_id_sha256: String,
    status: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireConflict {
    observed_payload_sha256: Option<String>,
}

fn parse_wire_ack(body: &[u8]) -> Result<ProviderEffectAck, String> {
    let wire: WireAck = serde_json::from_slice(body).map_err(|error| error.to_string())?;
    let key = ProviderEffectKey::parse(wire.effect_key).map_err(|error| format!("{error:?}"))?;
    let payload_sha256 = Sha256Digest::parse(wire.payload_sha256)?;
    let operation = Sha256Digest::parse(wire.provider_operation_id_sha256)?;
    let status = match wire.status.as_str() {
        "accepted" => ProviderEffectAckStatus::Accepted,
        "completed" => ProviderEffectAckStatus::Completed,
        "rejected" => ProviderEffectAckStatus::Rejected,
        _ => return Err("unknown provider status".to_string()),
    };
    Ok(ProviderEffectAck::new(key, payload_sha256, operation, status))
}

fn parse_wire_conflict_digest(body: &[u8]) -> Option<Sha256Digest> {
    let wire: WireConflict = serde_json::from_slice(body).ok()?;
    Sha256Digest::parse(wire.observed_payload_sha256?).ok()
}

fn encode_path_segment(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
                vec![byte as char]
            } else {
                format!("%{byte:02X}").chars().collect()
            }
        })
        .collect()
}

fn validate_effect_endpoint(raw: &str) -> Result<Url, String> {
    let url = Url::parse(raw).map_err(|error| format!("invalid effect endpoint URL: {error}"))?;
    if !url.username().is_empty() || url.password().is_some() {
        return Err("effect endpoint URL must not contain credentials".to_string());
    }
    let host = url
        .host_str()
        .ok_or_else(|| "effect endpoint URL must contain a host".to_string())?;
    let loopback = host.eq_ignore_ascii_case("localhost")
        || host
            .parse::<IpAddr>()
            .map(|address| address.is_loopback())
            .unwrap_or(false);
    if url.scheme() != "https" && !(url.scheme() == "http" && loopback) {
        return Err("effect endpoints must use HTTPS except loopback fixtures".to_string());
    }
    if url.fragment().is_some() {
        return Err("effect endpoint URL must not contain a fragment".to_string());
    }
    Ok(url)
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
