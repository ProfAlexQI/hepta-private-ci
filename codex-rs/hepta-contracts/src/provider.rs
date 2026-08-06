use serde::Deserialize;
use serde::Serialize;

use crate::stable_id::parse_prefixed_sha256_id;
use sha2::Digest;
use sha2::Sha256;

use crate::Sha256Digest;

pub const PROVIDER_EVIDENCE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct RequestBindingId(String);

impl RequestBindingId {
    pub fn for_request(binding: &ProviderRequestBinding) -> Self {
        let schema_version = binding.schema_version.to_string();
        let previous_response_id = binding
            .previous_response_id_sha256
            .as_ref()
            .map_or("", Sha256Digest::as_str);
        let previous_response_present = if binding.previous_response_id_sha256.is_some() {
            "present"
        } else {
            "absent"
        };
        let generate = if binding.generate {
            "generate"
        } else {
            "no_generate"
        };
        let mut parts = vec![
            schema_version.as_str(),
            binding.thread_id.as_str(),
            binding.turn_id.as_str(),
            binding.host_request_binding_id_sha256.as_str(),
            binding.request_kind.as_str(),
            binding.provider_id.as_str(),
            binding.provider_config_sha256.as_str(),
            binding.model.as_str(),
            binding.transport.as_str(),
            binding.endpoint_sha256.as_str(),
            binding.logical_request_sha256.as_str(),
            binding.wire_semantic_sha256.as_str(),
            previous_response_present,
            previous_response_id,
            generate,
        ];
        if binding.ephemeral_input_sha256.is_some()
            || binding.ephemeral_input_witness_sha256.is_some()
        {
            parts.extend([
                "ephemeral_input:v1",
                binding
                    .ephemeral_input_sha256
                    .as_ref()
                    .map_or("absent", Sha256Digest::as_str),
                binding
                    .ephemeral_input_witness_sha256
                    .as_ref()
                    .map_or("absent", Sha256Digest::as_str),
            ]);
        }
        Self(format!("provider-request:v1:{}", digest_parts(parts)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ProviderAttemptId(String);

impl ProviderAttemptId {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        parse_prefixed_sha256_id(value, "provider-attempt:v1:", "provider attempt").map(Self)
    }

    pub fn for_send(
        request_binding_id: &RequestBindingId,
        attempt_nonce_sha256: &Sha256Digest,
    ) -> Self {
        Self(format!(
            "provider-attempt:v1:{}",
            digest_parts([request_binding_id.as_str(), attempt_nonce_sha256.as_str()])
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ProviderReceiptId(String);

impl ProviderReceiptId {
    pub fn for_attempt(attempt_id: &ProviderAttemptId) -> Self {
        Self(format!(
            "provider-receipt:v1:{}",
            digest_parts([attempt_id.as_str()])
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderRequestKind {
    Turn,
    Prewarm,
    Compaction,
    Memory,
}

impl ProviderRequestKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Turn => "turn",
            Self::Prewarm => "prewarm",
            Self::Compaction => "compaction",
            Self::Memory => "memory",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderTransport {
    Http,
    WebSocket,
}

impl ProviderTransport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::WebSocket => "web_socket",
        }
    }
}

/// Stable, secret-free semantic material that identifies one logical provider request.
///
/// Request bodies, prompts, authentication headers, provider tokens, and response text do not
/// belong in this type. Their only permitted representation is a SHA-256 digest.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProviderRequestBinding {
    pub schema_version: u32,
    pub thread_id: String,
    pub turn_id: String,
    /// Digest of the host-owned retry-stable request binding identity.
    ///
    /// The opaque host identity itself must never cross the Hepta evidence boundary.
    pub host_request_binding_id_sha256: Sha256Digest,
    pub request_kind: ProviderRequestKind,
    pub provider_id: String,
    /// Compatibility-named digest of the versioned, secret-free provider
    /// selector. It must never be derived from credentials, headers, raw
    /// endpoint configuration, query values, or retry policy.
    pub provider_config_sha256: Sha256Digest,
    pub model: String,
    pub transport: ProviderTransport,
    pub endpoint_sha256: Sha256Digest,
    pub logical_request_sha256: Sha256Digest,
    /// Digest of bounded prompt-only input absent from conversation history.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_input_sha256: Option<Sha256Digest>,
    /// Host-minted single-use witness bound to the exact logical request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_input_witness_sha256: Option<Sha256Digest>,
    pub wire_semantic_sha256: Sha256Digest,
    pub previous_response_id_sha256: Option<Sha256Digest>,
    pub generate: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProviderInvocationIntent {
    pub schema_version: u32,
    pub attempt_id: ProviderAttemptId,
    pub request_binding_id: RequestBindingId,
    pub attempt_nonce_sha256: Sha256Digest,
    pub binding: ProviderRequestBinding,
}

impl ProviderInvocationIntent {
    /// Construct an intent from a host-generated 128-bit per-send nonce.
    ///
    /// The nonce itself is discarded immediately; only its SHA-256 digest can cross the
    /// governance boundary or be persisted.
    pub fn new(attempt_nonce: [u8; 16], binding: ProviderRequestBinding) -> Self {
        let attempt_nonce_sha256 = Sha256Digest::for_bytes(&attempt_nonce);
        Self::from_attempt_id_digest(attempt_nonce_sha256, binding)
    }

    /// Construct an intent from an opaque host-owned per-send attempt identity.
    ///
    /// Only the SHA-256 digest crosses the governance boundary; the host identity is discarded.
    pub fn for_host_attempt_id(host_attempt_id: &str, binding: ProviderRequestBinding) -> Self {
        let attempt_nonce_sha256 = Sha256Digest::for_bytes(host_attempt_id.as_bytes());
        Self::from_attempt_id_digest(attempt_nonce_sha256, binding)
    }

    fn from_attempt_id_digest(
        attempt_nonce_sha256: Sha256Digest,
        binding: ProviderRequestBinding,
    ) -> Self {
        let request_binding_id = RequestBindingId::for_request(&binding);
        let attempt_id = ProviderAttemptId::for_send(&request_binding_id, &attempt_nonce_sha256);
        Self {
            schema_version: PROVIDER_EVIDENCE_SCHEMA_VERSION,
            attempt_id,
            request_binding_id,
            attempt_nonce_sha256,
            binding,
        }
    }
}

/// Terminal observation for one provider send attempt.
///
/// `Completed` proves only that the provider emitted its response-completed signal. It is not an
/// effect acknowledgement and does not establish exactly-once execution.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "terminal", rename_all = "snake_case")]
pub enum ProviderTerminal {
    Completed {
        response_id_sha256: Sha256Digest,
        response_items_sha256: Sha256Digest,
        token_usage_sha256: Sha256Digest,
        /// Exact provider observation. `None` means the provider omitted the field.
        end_turn: Option<bool>,
    },
    Rejected {
        reason_code: String,
    },
    NotDispatched {
        reason_code: String,
    },
    Indeterminate {
        reason_code: String,
        partial_response_sha256: Option<Sha256Digest>,
    },
}

impl ProviderTerminal {
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Completed { .. } => "completed",
            Self::Rejected { .. } => "rejected",
            Self::NotDispatched { .. } => "not_dispatched",
            Self::Indeterminate { .. } => "indeterminate",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProviderInvocationReceipt {
    pub schema_version: u32,
    pub receipt_id: ProviderReceiptId,
    pub attempt_id: ProviderAttemptId,
    pub request_binding_id: RequestBindingId,
    pub intent: ProviderInvocationIntent,
    pub terminal: ProviderTerminal,
}

impl ProviderInvocationReceipt {
    pub fn new(intent: ProviderInvocationIntent, terminal: ProviderTerminal) -> Self {
        let attempt_id = intent.attempt_id.clone();
        let request_binding_id = intent.request_binding_id.clone();
        Self {
            schema_version: PROVIDER_EVIDENCE_SCHEMA_VERSION,
            receipt_id: ProviderReceiptId::for_attempt(&attempt_id),
            attempt_id,
            request_binding_id,
            intent,
            terminal,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn binding() -> ProviderRequestBinding {
        ProviderRequestBinding {
            schema_version: PROVIDER_EVIDENCE_SCHEMA_VERSION,
            thread_id: "thread-1".to_string(),
            turn_id: "turn-1".to_string(),
            host_request_binding_id_sha256: Sha256Digest::for_bytes(b"host-request-1"),
            request_kind: ProviderRequestKind::Turn,
            provider_id: "provider-1".to_string(),
            provider_config_sha256: Sha256Digest::for_bytes(b"config"),
            model: "model-1".to_string(),
            transport: ProviderTransport::Http,
            endpoint_sha256: Sha256Digest::for_bytes(b"/responses"),
            logical_request_sha256: Sha256Digest::for_bytes(b"logical"),
            ephemeral_input_sha256: None,
            ephemeral_input_witness_sha256: None,
            wire_semantic_sha256: Sha256Digest::for_bytes(b"wire"),
            previous_response_id_sha256: None,
            generate: true,
        }
    }

    #[test]
    fn provider_ids_are_versioned_stable_and_length_delimited() {
        let binding = binding();
        let request_id = RequestBindingId::for_request(&binding);
        let repeated = RequestBindingId::for_request(&binding);
        let left_nonce = Sha256Digest::for_bytes(b"ab:c");
        let right_nonce = Sha256Digest::for_bytes(b"a:bc");
        let left = ProviderAttemptId::for_send(&request_id, &left_nonce);
        let right = ProviderAttemptId::for_send(&request_id, &right_nonce);

        assert!(request_id.as_str().starts_with("provider-request:v1:"));
        assert_eq!(request_id, repeated);
        assert!(left.as_str().starts_with("provider-attempt:v1:"));
        assert_ne!(left, right);
        assert!(
            ProviderReceiptId::for_attempt(&left)
                .as_str()
                .starts_with("provider-receipt:v1:")
        );
    }

    #[test]
    fn absent_prompt_only_authority_preserves_legacy_identity_and_wire_shape() {
        let binding = binding();

        assert_eq!(
            RequestBindingId::for_request(&binding).as_str(),
            "provider-request:v1:af856c7c8c26482cec5a16aeb1c302f126e7eb1091cd4ad1a58b594fc0d40809"
        );
        let json = serde_json::to_string(&binding).expect("serialize provider binding");
        assert!(!json.contains("ephemeral_input_sha256"));
        assert!(!json.contains("ephemeral_input_witness_sha256"));
    }

    #[test]
    fn request_binding_changes_for_websocket_incremental_semantics() {
        let http = binding();
        let mut websocket = http.clone();
        websocket.transport = ProviderTransport::WebSocket;
        websocket.wire_semantic_sha256 = Sha256Digest::for_bytes(b"incremental-wire");
        websocket.previous_response_id_sha256 = Some(Sha256Digest::for_bytes(b"previous"));

        assert_ne!(
            RequestBindingId::for_request(&http),
            RequestBindingId::for_request(&websocket)
        );
    }

    #[test]
    fn request_binding_changes_for_prompt_only_authority() {
        let plain = binding();
        let mut attached = plain.clone();
        attached.ephemeral_input_sha256 = Some(Sha256Digest::for_bytes(b"ephemeral"));
        attached.ephemeral_input_witness_sha256 = Some(Sha256Digest::for_bytes(b"witness"));

        assert_ne!(
            RequestBindingId::for_request(&plain),
            RequestBindingId::for_request(&attached)
        );
    }
}
