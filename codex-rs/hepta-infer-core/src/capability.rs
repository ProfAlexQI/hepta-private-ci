//! Secret-bearing capability primitives for the private inference control plane.
//!
//! The public UDS protocol never serializes these values. Only a daemon-owned
//! epoch key may mint worker authentication tags and one-time request grants.

use std::sync::atomic::Ordering;
use std::sync::atomic::compiler_fence;

use crate::Digest;
use crate::InferError;
use crate::InferenceRequest;
use crate::Result;
use crate::hashing::sha256;

pub const CAPABILITY_KEY_BYTES: usize = 32;
pub const PRIVATE_AUTH_TAG_BYTES: usize = 32;

const HMAC_BLOCK_BYTES: usize = 64;
const REQUEST_GRANT_DOMAIN: &[u8] = b"hepta.inference.request-grant.v1\0";
const REQUEST_GRANT_DIGEST_DOMAIN: &[u8] = b"hepta.inference.request-grant-digest.v1\0";
const WORKER_AUTH_DOMAIN: &[u8] = b"hepta.inference.worker-auth.v1\0";
const WORKER_SESSION_DOMAIN: &[u8] = b"hepta.inference.worker-session.v1\0";

/// Daemon epoch key. It intentionally implements neither `Clone` nor `Debug`.
pub struct CapabilityKey([u8; CAPABILITY_KEY_BYTES]);

/// Raw one-time request grant. It intentionally implements neither `Clone` nor `Debug`.
pub struct RequestGrant([u8; PRIVATE_AUTH_TAG_BYTES]);

/// Challenge response for a private worker handshake.
///
/// The value is short-lived and intentionally implements neither `Clone` nor `Debug`.
pub struct WorkerAuthenticationTag([u8; PRIVATE_AUTH_TAG_BYTES]);

#[derive(Clone, Copy)]
pub struct RequestGrantFence<'a> {
    pub request: &'a InferenceRequest,
    pub backend_generation: u64,
    pub worker_session_digest: &'a Digest,
}

#[derive(Clone, Copy)]
pub struct WorkerHandshakeFence<'a> {
    pub worker_pid: u32,
    pub backend_generation: u64,
    pub worker_nonce_digest: &'a Digest,
    pub daemon_challenge_digest: &'a Digest,
}

impl CapabilityKey {
    pub fn from_private_bytes(bytes: [u8; CAPABILITY_KEY_BYTES]) -> Result<Self> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(InferError::InvalidCapabilityKey);
        }
        Ok(Self(bytes))
    }

    pub fn derive_request_grant(&self, fence: RequestGrantFence<'_>) -> Result<RequestGrant> {
        validate_request_grant_fence(fence)?;
        let preimage = encode_request_grant_fence(fence)?;
        Ok(RequestGrant(hmac_sha256(
            &self.0,
            &[REQUEST_GRANT_DOMAIN, preimage.as_slice()],
        )?))
    }

    pub fn verify_request_grant(
        &self,
        fence: RequestGrantFence<'_>,
        presented: &RequestGrant,
    ) -> bool {
        let Ok(expected) = self.derive_request_grant(fence) else {
            return false;
        };
        constant_time_equal(&expected.0, &presented.0)
    }

    pub fn derive_worker_authentication(
        &self,
        fence: WorkerHandshakeFence<'_>,
    ) -> Result<WorkerAuthenticationTag> {
        validate_worker_handshake_fence(fence)?;
        let mut preimage = Vec::with_capacity(256);
        append_u64(&mut preimage, u64::from(fence.worker_pid));
        append_u64(&mut preimage, fence.backend_generation);
        append_text(&mut preimage, fence.worker_nonce_digest.as_str())?;
        append_text(&mut preimage, fence.daemon_challenge_digest.as_str())?;
        Ok(WorkerAuthenticationTag(hmac_sha256(
            &self.0,
            &[WORKER_AUTH_DOMAIN, preimage.as_slice()],
        )?))
    }

    pub fn verify_worker_authentication(
        &self,
        fence: WorkerHandshakeFence<'_>,
        presented: &WorkerAuthenticationTag,
    ) -> bool {
        let Ok(expected) = self.derive_worker_authentication(fence) else {
            return false;
        };
        constant_time_equal(&expected.0, &presented.0)
    }

    pub fn worker_session_digest(&self, fence: WorkerHandshakeFence<'_>) -> Result<Digest> {
        let authentication = self.derive_worker_authentication(fence)?;
        let mut preimage = Vec::with_capacity(320);
        append_u64(&mut preimage, u64::from(fence.worker_pid));
        append_u64(&mut preimage, fence.backend_generation);
        append_text(&mut preimage, fence.worker_nonce_digest.as_str())?;
        append_text(&mut preimage, fence.daemon_challenge_digest.as_str())?;
        preimage.extend_from_slice(authentication.private_bytes());
        digest_from_bytes(sha256(&[WORKER_SESSION_DOMAIN, preimage.as_slice()])?)
    }
}

impl Drop for CapabilityKey {
    fn drop(&mut self) {
        zeroize_private_bytes(&mut self.0);
    }
}

impl RequestGrant {
    pub fn from_private_bytes(bytes: [u8; PRIVATE_AUTH_TAG_BYTES]) -> Result<Self> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(InferError::InvalidCapability);
        }
        Ok(Self(bytes))
    }

    pub fn private_bytes(&self) -> &[u8; PRIVATE_AUTH_TAG_BYTES] {
        &self.0
    }

    pub fn digest(&self) -> Result<Digest> {
        digest_from_bytes(sha256(&[
            REQUEST_GRANT_DIGEST_DOMAIN,
            self.private_bytes(),
        ])?)
    }
}

impl Drop for RequestGrant {
    fn drop(&mut self) {
        zeroize_private_bytes(&mut self.0);
    }
}

impl WorkerAuthenticationTag {
    pub fn from_private_bytes(bytes: [u8; PRIVATE_AUTH_TAG_BYTES]) -> Result<Self> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(InferError::InvalidCapability);
        }
        Ok(Self(bytes))
    }

    pub fn private_bytes(&self) -> &[u8; PRIVATE_AUTH_TAG_BYTES] {
        &self.0
    }
}

impl Drop for WorkerAuthenticationTag {
    fn drop(&mut self) {
        zeroize_private_bytes(&mut self.0);
    }
}

fn validate_request_grant_fence(fence: RequestGrantFence<'_>) -> Result<()> {
    fence.request.validate_shape()?;
    if fence.backend_generation == 0 {
        return Err(InferError::InvalidGeneration);
    }
    Ok(())
}

fn validate_worker_handshake_fence(fence: WorkerHandshakeFence<'_>) -> Result<()> {
    if fence.worker_pid == 0 || fence.backend_generation == 0 {
        return Err(InferError::InvalidCapability);
    }
    Ok(())
}

fn encode_request_grant_fence(fence: RequestGrantFence<'_>) -> Result<Vec<u8>> {
    let request = fence.request;
    let mut preimage = Vec::with_capacity(1024);
    append_text(&mut preimage, request.identity.tenant_id.as_str())?;
    append_text(&mut preimage, request.identity.workspace_id.as_str())?;
    append_text(&mut preimage, request.identity.agent_id.as_str())?;
    append_text(&mut preimage, request.identity.task_id.as_str())?;
    append_text(&mut preimage, request.identity.request_id.as_str())?;
    append_u64(&mut preimage, request.agent_generation);
    append_u64(&mut preimage, request.request_generation);
    append_u64(&mut preimage, request.cancel_generation);
    append_u64(&mut preimage, fence.backend_generation);
    append_u64(&mut preimage, request.deadline_unix_ms);
    append_text(&mut preimage, request.model_tuple_digest.as_str())?;
    append_text(&mut preimage, request.policy_digest.as_str())?;
    append_text(&mut preimage, request.resource_budget_id.as_str())?;
    append_text(&mut preimage, request.prompt_digest.as_str())?;
    append_u64(&mut preimage, request.prompt_byte_length);
    append_u64(&mut preimage, u64::from(request.output_token_limit));
    append_text(&mut preimage, fence.worker_session_digest.as_str())?;
    Ok(preimage)
}

fn hmac_sha256(key: &[u8], parts: &[&[u8]]) -> Result<[u8; 32]> {
    let mut key_block = [0u8; HMAC_BLOCK_BYTES];
    if key.len() > HMAC_BLOCK_BYTES {
        key_block[..32].copy_from_slice(&sha256(&[key])?);
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }

    let mut inner_pad = [0x36u8; HMAC_BLOCK_BYTES];
    let mut outer_pad = [0x5cu8; HMAC_BLOCK_BYTES];
    for index in 0..HMAC_BLOCK_BYTES {
        inner_pad[index] ^= key_block[index];
        outer_pad[index] ^= key_block[index];
    }
    zeroize_private_bytes(&mut key_block);

    let payload_length = parts.iter().try_fold(HMAC_BLOCK_BYTES, |total, part| {
        total
            .checked_add(part.len())
            .ok_or(InferError::InvalidCapability)
    })?;
    let mut inner_input = Vec::with_capacity(payload_length);
    inner_input.extend_from_slice(&inner_pad);
    for part in parts {
        inner_input.extend_from_slice(part);
    }
    let inner = sha256(&[inner_input.as_slice()])?;
    zeroize_private_slice(inner_input.as_mut_slice());
    zeroize_private_bytes(&mut inner_pad);

    let output = sha256(&[&outer_pad, &inner])?;
    zeroize_private_bytes(&mut outer_pad);
    Ok(output)
}

fn constant_time_equal(left: &[u8; PRIVATE_AUTH_TAG_BYTES], right: &[u8; PRIVATE_AUTH_TAG_BYTES]) -> bool {
    let mut difference = 0u8;
    for index in 0..PRIVATE_AUTH_TAG_BYTES {
        difference |= left[index] ^ right[index];
    }
    difference == 0
}

fn append_text(buffer: &mut Vec<u8>, value: &str) -> Result<()> {
    let length = u64::try_from(value.len()).map_err(|_| InferError::InvalidCapability)?;
    append_u64(buffer, length);
    buffer.extend_from_slice(value.as_bytes());
    Ok(())
}

fn append_u64(buffer: &mut Vec<u8>, value: u64) {
    buffer.extend_from_slice(&value.to_be_bytes());
}

fn digest_from_bytes(bytes: [u8; 32]) -> Result<Digest> {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(71);
    encoded.push_str("sha256:");
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    Digest::parse(&encoded)
}

fn zeroize_private_bytes<const N: usize>(bytes: &mut [u8; N]) {
    zeroize_private_slice(bytes);
}

fn zeroize_private_slice(bytes: &mut [u8]) {
    for byte in bytes {
        // SAFETY: every pointer comes from a unique mutable slice element and is valid
        // for a single-byte volatile write. Volatile writes prevent dead-store removal.
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgentId;
    use crate::AuthoritySnapshot;
    use crate::RequestId;
    use crate::RequestIdentity;
    use crate::ResourceBudgetId;
    use crate::TaskId;
    use crate::TenantId;
    use crate::WorkspaceId;

    fn must<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("unexpected error: {error}"),
        }
    }

    fn digest(fill: char) -> Digest {
        must(Digest::parse(&format!(
            "sha256:{}",
            fill.to_string().repeat(64)
        )))
    }

    fn request() -> InferenceRequest {
        InferenceRequest {
            identity: RequestIdentity {
                tenant_id: must(TenantId::parse("tenant-a")),
                workspace_id: must(WorkspaceId::parse("workspace-a")),
                agent_id: must(AgentId::parse("agent-a")),
                task_id: must(TaskId::parse("task-a")),
                request_id: must(RequestId::parse("request-capability")),
            },
            agent_generation: 1,
            request_generation: 2,
            cancel_generation: 3,
            deadline_unix_ms: 9_999,
            model_tuple_digest: digest('a'),
            policy_digest: digest('b'),
            resource_budget_id: must(ResourceBudgetId::parse("budget-a")),
            prompt_digest: digest('c'),
            prompt_byte_length: 12,
            output_token_limit: 32,
            authority: AuthoritySnapshot::qualification_only_closed(),
        }
    }

    fn encode_hex(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        output
    }

    #[test]
    fn hmac_matches_rfc_4231_case_one() {
        let key = [0x0bu8; 20];
        let tag = must(hmac_sha256(&key, &[b"Hi There"]));
        assert_eq!(
            encode_hex(&tag),
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
    }

    #[test]
    fn request_grant_binds_session_and_every_generation() {
        let key = must(CapabilityKey::from_private_bytes([7u8; CAPABILITY_KEY_BYTES]));
        let request = request();
        let session_a = digest('d');
        let session_b = digest('e');
        let grant = must(key.derive_request_grant(RequestGrantFence {
            request: &request,
            backend_generation: 4,
            worker_session_digest: &session_a,
        }));
        assert!(key.verify_request_grant(
            RequestGrantFence {
                request: &request,
                backend_generation: 4,
                worker_session_digest: &session_a,
            },
            &grant,
        ));
        assert!(!key.verify_request_grant(
            RequestGrantFence {
                request: &request,
                backend_generation: 5,
                worker_session_digest: &session_a,
            },
            &grant,
        ));
        assert!(!key.verify_request_grant(
            RequestGrantFence {
                request: &request,
                backend_generation: 4,
                worker_session_digest: &session_b,
            },
            &grant,
        ));
        assert_ne!(must(grant.digest()), session_a);
    }

    #[test]
    fn worker_authentication_binds_pid_generation_nonce_and_challenge() {
        let key = must(CapabilityKey::from_private_bytes([9u8; CAPABILITY_KEY_BYTES]));
        let nonce = digest('a');
        let challenge = digest('b');
        let other_challenge = digest('c');
        let fence = WorkerHandshakeFence {
            worker_pid: 42,
            backend_generation: 7,
            worker_nonce_digest: &nonce,
            daemon_challenge_digest: &challenge,
        };
        let tag = must(key.derive_worker_authentication(fence));
        assert!(key.verify_worker_authentication(fence, &tag));
        assert!(!key.verify_worker_authentication(
            WorkerHandshakeFence {
                worker_pid: 42,
                backend_generation: 7,
                worker_nonce_digest: &nonce,
                daemon_challenge_digest: &other_challenge,
            },
            &tag,
        ));
        let session = must(key.worker_session_digest(fence));
        assert_ne!(session, nonce);
        assert_ne!(session, challenge);
    }

    #[test]
    fn all_zero_secret_material_fails_closed() {
        assert!(matches!(
            CapabilityKey::from_private_bytes([0u8; CAPABILITY_KEY_BYTES]),
            Err(InferError::InvalidCapabilityKey)
        ));
        assert!(matches!(
            RequestGrant::from_private_bytes([0u8; PRIVATE_AUTH_TAG_BYTES]),
            Err(InferError::InvalidCapability)
        ));
        assert!(matches!(
            WorkerAuthenticationTag::from_private_bytes([0u8; PRIVATE_AUTH_TAG_BYTES]),
            Err(InferError::InvalidCapability)
        ));
    }
}
