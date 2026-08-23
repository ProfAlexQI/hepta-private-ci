//! Contract-only seam for a provider-backed exactly-once effect.
//!
//! These types deliberately do not make any claim about a provider currently
//! supporting idempotency.  A provider must offer both a stable key transport
//! and a durable status lookup before an adapter may report the supported
//! capability.  The existing Codex HTTP and WebSocket adapters do not
//! implement this seam.

use std::future::Future;
use std::pin::Pin;

use serde::Deserialize;
use serde::Serialize;

use crate::RequestBindingId;
use crate::Sha256Digest;
use crate::stable_id::parse_prefixed_sha256_id;

pub const PROVIDER_EFFECT_SCHEMA_VERSION: u32 = 1;

/// Provider capability required before a physical effect can be retried or
/// reconciled by key.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEffectIdempotencyCapability {
    /// The provider contract does not expose key-based dedupe and lookup.
    #[default]
    Unsupported,
    /// The provider contract exposes a stable key, same-key conflict rules,
    /// and durable status lookup.  No current provider is marked this way.
    KeyAndStatusLookup,
}

/// Stable logical identity for one occurrence across physical send attempts.
///
/// The key intentionally excludes the per-send nonce and payload digest.  A
/// provider can therefore detect a same-key/different-payload conflict rather
/// than silently treating a changed payload as a new effect.  The payload
/// digest is carried separately by [`ProviderEffectIntent`] and
/// [`ProviderEffectAck`].
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ProviderEffectKey(String);

impl ProviderEffectKey {
    /// Derives an occurrence-stable key from secret-free provider scope,
    /// caller-owned occurrence identity, and the logical request binding.
    ///
    /// `provider_scope` and `occurrence_id` must not contain credentials or
    /// request payload.  They are length-delimited before hashing.
    pub fn for_occurrence(
        provider_scope: &str,
        occurrence_id: &str,
        request_binding_id: &RequestBindingId,
    ) -> Result<Self, ProviderEffectBindingError> {
        validate_non_empty("provider scope", provider_scope)?;
        validate_non_empty("occurrence id", occurrence_id)?;
        Ok(Self(format!(
            "provider-effect:v1:{}",
            digest_parts([
                "provider-effect:v1",
                provider_scope,
                occurrence_id,
                request_binding_id.as_str(),
            ])
        )))
    }

    pub fn parse(value: impl Into<String>) -> Result<Self, ProviderEffectBindingError> {
        parse_prefixed_sha256_id(value, "provider-effect:v1:", "provider effect")
            .map(Self)
            .map_err(ProviderEffectBindingError::InvalidKey)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Durable local intent that must exist before a provider seam is crossed.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProviderEffectIntent {
    pub schema_version: u32,
    pub key: ProviderEffectKey,
    pub payload_sha256: Sha256Digest,
}

impl ProviderEffectIntent {
    pub fn new(key: ProviderEffectKey, payload_sha256: Sha256Digest) -> Self {
        Self {
            schema_version: PROVIDER_EFFECT_SCHEMA_VERSION,
            key,
            payload_sha256,
        }
    }

    pub fn validate(&self) -> Result<(), ProviderEffectBindingError> {
        if self.schema_version != PROVIDER_EFFECT_SCHEMA_VERSION {
            return Err(ProviderEffectBindingError::SchemaVersion);
        }
        ProviderEffectKey::parse(self.key.as_str().to_string())?;
        Sha256Digest::parse(self.payload_sha256.as_str().to_string())
            .map_err(ProviderEffectBindingError::InvalidDigest)?;
        Ok(())
    }
}

/// Provider-side terminal status carried by a key-bound acknowledgement.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderEffectAckStatus {
    /// The provider durably accepted the operation, but effect completion is
    /// not yet observed.
    Accepted,
    /// The provider contractually confirms completion for this key/payload.
    Completed,
    /// The provider rejected the operation and promises no effect occurred.
    Rejected,
}

/// Provider acknowledgement bound to one logical key and exact payload digest.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProviderEffectAck {
    pub schema_version: u32,
    pub key: ProviderEffectKey,
    pub payload_sha256: Sha256Digest,
    pub provider_operation_id_sha256: Sha256Digest,
    pub status: ProviderEffectAckStatus,
}

impl ProviderEffectAck {
    pub fn new(
        key: ProviderEffectKey,
        payload_sha256: Sha256Digest,
        provider_operation_id_sha256: Sha256Digest,
        status: ProviderEffectAckStatus,
    ) -> Self {
        Self {
            schema_version: PROVIDER_EFFECT_SCHEMA_VERSION,
            key,
            payload_sha256,
            provider_operation_id_sha256,
            status,
        }
    }

    /// Verifies that an externally returned acknowledgement can close the
    /// exact local intent.  A response or request ID alone is insufficient.
    pub fn validate_for(
        &self,
        intent: &ProviderEffectIntent,
    ) -> Result<(), ProviderEffectBindingError> {
        intent.validate()?;
        if self.schema_version != PROVIDER_EFFECT_SCHEMA_VERSION {
            return Err(ProviderEffectBindingError::SchemaVersion);
        }
        ProviderEffectKey::parse(self.key.as_str().to_string())?;
        Sha256Digest::parse(self.payload_sha256.as_str().to_string())
            .map_err(ProviderEffectBindingError::InvalidDigest)?;
        Sha256Digest::parse(self.provider_operation_id_sha256.as_str().to_string())
            .map_err(ProviderEffectBindingError::InvalidDigest)?;
        if self.key != intent.key {
            return Err(ProviderEffectBindingError::KeyMismatch);
        }
        if self.payload_sha256 != intent.payload_sha256 {
            return Err(ProviderEffectBindingError::PayloadMismatch);
        }
        Ok(())
    }

    /// Only a provider `Completed` acknowledgement can establish the
    /// provider-side effect terminal.  `Accepted` remains pending.
    pub const fn proves_effect_completion(&self) -> bool {
        matches!(self.status, ProviderEffectAckStatus::Completed)
    }
}

/// Result of asking a provider for the current state of a key.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum ProviderEffectLookup {
    Ack(ProviderEffectAck),
    NotFound,
    Conflict {
        observed_payload_sha256: Option<Sha256Digest>,
    },
    /// Network/process failure leaves the provider state unknown.  Callers
    /// must quarantine the intent and must not blind-retry.
    Unknown,
}

/// Result of one provider dispatch attempt.  This is intentionally separate
/// from [`ProviderEffectLookup`]: a dispatch response may be lost even when a
/// later status lookup can reconcile it.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum ProviderEffectDispatch {
    Ack(ProviderEffectAck),
    Rejected { reason_code: String },
    NotDispatched { reason_code: String },
    Unknown,
}

/// Errors returned while reconciling a provider lookup against local intent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderEffectBindingError {
    EmptyField(&'static str),
    InvalidKey(String),
    InvalidDigest(String),
    SchemaVersion,
    KeyMismatch,
    PayloadMismatch,
    UnsupportedCapability,
    NotFound,
    Unknown,
    Conflict,
}

/// Reconciles one provider lookup without ever retrying the physical send.
///
/// This helper is deliberately synchronous and network-agnostic so contract
/// tests can exercise the fail-closed state machine without a provider fixture.
pub fn reconcile_provider_lookup(
    capability: ProviderEffectIdempotencyCapability,
    intent: &ProviderEffectIntent,
    lookup: ProviderEffectLookup,
) -> Result<ProviderEffectAck, ProviderEffectBindingError> {
    intent.validate()?;
    if capability == ProviderEffectIdempotencyCapability::Unsupported {
        return Err(ProviderEffectBindingError::UnsupportedCapability);
    }
    match lookup {
        ProviderEffectLookup::Ack(ack) => {
            ack.validate_for(intent)?;
            Ok(ack)
        }
        ProviderEffectLookup::NotFound => Err(ProviderEffectBindingError::NotFound),
        ProviderEffectLookup::Conflict { .. } => Err(ProviderEffectBindingError::Conflict),
        ProviderEffectLookup::Unknown => Err(ProviderEffectBindingError::Unknown),
    }
}

/// Async adapter seam for a future provider implementation.
///
/// No current HTTP or WebSocket provider implements this trait. An adapter may
/// report `KeyAndStatusLookup` only after its provider contract proves stable
/// key transport, same-key conflict/dedupe, and durable lookup semantics.
pub type ProviderEffectFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait ProviderEffectAdapter: Send + Sync {
    fn capability(&self) -> ProviderEffectIdempotencyCapability {
        ProviderEffectIdempotencyCapability::Unsupported
    }

    fn dispatch<'a>(
        &'a self,
        intent: &'a ProviderEffectIntent,
    ) -> ProviderEffectFuture<'a, ProviderEffectDispatch>;

    fn lookup<'a>(
        &'a self,
        key: &'a ProviderEffectKey,
    ) -> ProviderEffectFuture<'a, ProviderEffectLookup>;
}

fn validate_non_empty(label: &'static str, value: &str) -> Result<(), ProviderEffectBindingError> {
    if value.trim().is_empty() {
        return Err(ProviderEffectBindingError::EmptyField(label));
    }
    Ok(())
}

fn digest_parts<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    use sha2::Digest;
    use sha2::Sha256;

    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PROVIDER_EVIDENCE_SCHEMA_VERSION;
    use crate::ProviderRequestBinding;
    use crate::ProviderRequestKind;
    use crate::ProviderTransport;

    fn request_binding_id() -> RequestBindingId {
        RequestBindingId::for_request(&ProviderRequestBinding {
            schema_version: PROVIDER_EVIDENCE_SCHEMA_VERSION,
            thread_id: "thread-1".to_string(),
            turn_id: "turn-1".to_string(),
            host_request_binding_id_sha256: Sha256Digest::for_bytes(b"host-request"),
            request_kind: ProviderRequestKind::Turn,
            provider_id: "provider-1".to_string(),
            provider_config_sha256: Sha256Digest::for_bytes(b"config"),
            model: "model-1".to_string(),
            transport: ProviderTransport::Http,
            endpoint_sha256: Sha256Digest::for_bytes(b"/responses"),
            logical_request_sha256: Sha256Digest::for_bytes(b"logical"),
            wire_semantic_sha256: Sha256Digest::for_bytes(b"wire"),
            ephemeral_input_sha256: None,
            ephemeral_input_witness_sha256: None,
            previous_response_id_sha256: None,
            generate: true,
        })
    }

    fn intent(payload: &[u8]) -> ProviderEffectIntent {
        let key = ProviderEffectKey::for_occurrence(
            "provider-1/config-v1",
            "hepta.automation.v1:agent-a:task-a:1",
            &request_binding_id(),
        )
        .expect("effect key");
        ProviderEffectIntent::new(key, Sha256Digest::for_bytes(payload))
    }

    fn ack(intent: &ProviderEffectIntent, payload: &[u8]) -> ProviderEffectAck {
        ProviderEffectAck::new(
            intent.key.clone(),
            Sha256Digest::for_bytes(payload),
            Sha256Digest::for_bytes(b"provider-operation-1"),
            ProviderEffectAckStatus::Completed,
        )
    }

    #[test]
    fn occurrence_key_is_stable_across_physical_retries_and_excludes_payload() {
        let binding = request_binding_id();
        let first = ProviderEffectKey::for_occurrence("provider-1/config-v1", "occ-1", &binding)
            .expect("first key");
        let retry = ProviderEffectKey::for_occurrence("provider-1/config-v1", "occ-1", &binding)
            .expect("retry key");
        let changed_payload =
            ProviderEffectKey::for_occurrence("provider-1/config-v1", "occ-1", &binding)
                .expect("changed payload key");
        let changed_occurrence =
            ProviderEffectKey::for_occurrence("provider-1/config-v1", "occ-2", &binding)
                .expect("changed occurrence key");

        assert_eq!(first, retry);
        assert_eq!(first, changed_payload);
        assert_ne!(first, changed_occurrence);
    }

    #[test]
    fn same_key_different_payload_is_rejected() {
        let intent = intent(b"payload-a");
        let mismatched = ack(&intent, b"payload-b");
        assert_eq!(
            mismatched.validate_for(&intent),
            Err(ProviderEffectBindingError::PayloadMismatch)
        );
        assert_eq!(
            reconcile_provider_lookup(
                ProviderEffectIdempotencyCapability::KeyAndStatusLookup,
                &intent,
                ProviderEffectLookup::Ack(mismatched),
            ),
            Err(ProviderEffectBindingError::PayloadMismatch)
        );
    }

    #[test]
    fn matching_ack_binds_key_payload_and_operation() {
        let intent = intent(b"payload-a");
        let matching = ack(&intent, b"payload-a");
        assert!(matching.validate_for(&intent).is_ok());
        assert!(matching.proves_effect_completion());
        let reconciled = reconcile_provider_lookup(
            ProviderEffectIdempotencyCapability::KeyAndStatusLookup,
            &intent,
            ProviderEffectLookup::Ack(matching.clone()),
        )
        .expect("matching lookup");
        assert_eq!(reconciled, matching);
    }

    #[test]
    fn unsupported_capability_fails_closed_even_with_matching_ack() {
        let intent = intent(b"payload-a");
        let matching = ack(&intent, b"payload-a");
        assert_eq!(
            reconcile_provider_lookup(
                ProviderEffectIdempotencyCapability::Unsupported,
                &intent,
                ProviderEffectLookup::Ack(matching),
            ),
            Err(ProviderEffectBindingError::UnsupportedCapability)
        );
    }

    #[test]
    fn unknown_and_not_found_lookup_remain_quarantined() {
        let intent = intent(b"payload-a");
        for lookup in [
            ProviderEffectLookup::Unknown,
            ProviderEffectLookup::NotFound,
        ] {
            let error = reconcile_provider_lookup(
                ProviderEffectIdempotencyCapability::KeyAndStatusLookup,
                &intent,
                lookup,
            )
            .expect_err("lookup must not close intent");
            assert!(matches!(
                error,
                ProviderEffectBindingError::Unknown | ProviderEffectBindingError::NotFound
            ));
        }
    }

    #[test]
    fn conflict_lookup_is_not_a_success() {
        let intent = intent(b"payload-a");
        assert_eq!(
            reconcile_provider_lookup(
                ProviderEffectIdempotencyCapability::KeyAndStatusLookup,
                &intent,
                ProviderEffectLookup::Conflict {
                    observed_payload_sha256: Some(Sha256Digest::for_bytes(b"payload-b")),
                },
            ),
            Err(ProviderEffectBindingError::Conflict)
        );
    }

    #[test]
    fn accepted_ack_does_not_prove_effect_completion() {
        let intent = intent(b"payload-a");
        let accepted = ProviderEffectAck::new(
            intent.key.clone(),
            intent.payload_sha256.clone(),
            Sha256Digest::for_bytes(b"provider-operation-1"),
            ProviderEffectAckStatus::Accepted,
        );
        assert!(!accepted.proves_effect_completion());
        assert!(accepted.validate_for(&intent).is_ok());
    }

    #[test]
    fn default_capability_is_unsupported() {
        assert_eq!(
            ProviderEffectIdempotencyCapability::default(),
            ProviderEffectIdempotencyCapability::Unsupported
        );
    }
}
