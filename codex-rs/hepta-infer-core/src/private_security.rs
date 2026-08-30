//! Daemon-owned private-channel security state.
//!
//! Raw secrets deliberately implement neither `Clone`, `Copy`, `Debug`, nor
//! serialization. Public protocols only observe digests; request grants are
//! consumed exactly once by a bounded daemon-owned ledger.

use std::collections::BTreeMap;
#[cfg(unix)]
use std::fs::File;
#[cfg(unix)]
use std::io::Read;
use std::sync::atomic::Ordering;
use std::sync::atomic::compiler_fence;

use crate::CAPABILITY_KEY_BYTES;
use crate::CapabilityKey;
use crate::Digest;
use crate::InferError;
use crate::PRIVATE_AUTH_TAG_BYTES;
use crate::RequestGrant;
use crate::RequestGrantFence;
use crate::RequestId;
use crate::Result;
use crate::hashing::sha256;

const HMAC_BLOCK_BYTES: usize = 64;
const SESSION_NONCE_DOMAIN: &[u8] = b"hepta.inference.session-nonce.v1\0";
const OPERATOR_AUTH_DOMAIN: &[u8] = b"hepta.inference.operator-auth.v1\0";
const OPERATOR_SESSION_DOMAIN: &[u8] = b"hepta.inference.operator-session.v1\0";
const REQUEST_GRANT_FENCE_DOMAIN: &[u8] = b"hepta.inference.request-grant-fence.v1\0";

/// Generate a request-capability key from the operating system CSPRNG.
pub fn generate_request_capability_key_os() -> Result<CapabilityKey> {
    let mut bytes = [0u8; CAPABILITY_KEY_BYTES];
    fill_os_random(&mut bytes)?;
    CapabilityKey::from_private_bytes(bytes)
}

/// OS-CSPRNG session nonce. It intentionally exposes only a digest.
pub struct SessionNonce([u8; CAPABILITY_KEY_BYTES]);

impl SessionNonce {
    pub fn generate_os() -> Result<Self> {
        let mut bytes = [0u8; CAPABILITY_KEY_BYTES];
        fill_os_random(&mut bytes)?;
        if bytes.iter().all(|byte| *byte == 0) {
            zeroize_private_bytes(&mut bytes);
            return Err(InferError::EntropyUnavailable);
        }
        Ok(Self(bytes))
    }

    pub fn digest(&self) -> Result<Digest> {
        digest_from_bytes(sha256(&[SESSION_NONCE_DOMAIN, &self.0])?)
    }
}

impl Drop for SessionNonce {
    fn drop(&mut self) {
        zeroize_private_bytes(&mut self.0);
    }
}

/// Independent operator-channel epoch key.
pub struct OperatorCapabilityKey([u8; CAPABILITY_KEY_BYTES]);

/// Short-lived operator challenge response.
pub struct OperatorAuthenticationTag([u8; PRIVATE_AUTH_TAG_BYTES]);

#[derive(Clone, Copy)]
pub struct OperatorHandshakeFence<'a> {
    pub operator_pid: u32,
    pub backend_generation: u64,
    pub operator_nonce_digest: &'a Digest,
    pub daemon_challenge_digest: &'a Digest,
}

impl OperatorCapabilityKey {
    pub fn generate_os() -> Result<Self> {
        let mut bytes = [0u8; CAPABILITY_KEY_BYTES];
        fill_os_random(&mut bytes)?;
        Self::from_private_bytes(bytes)
    }

    pub fn from_private_bytes(bytes: [u8; CAPABILITY_KEY_BYTES]) -> Result<Self> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(InferError::InvalidCapabilityKey);
        }
        Ok(Self(bytes))
    }

    pub fn derive_authentication(
        &self,
        fence: OperatorHandshakeFence<'_>,
    ) -> Result<OperatorAuthenticationTag> {
        validate_operator_fence(fence)?;
        let mut preimage = encode_operator_fence(fence)?;
        let tag = hmac_sha256(&self.0, &[OPERATOR_AUTH_DOMAIN, preimage.as_slice()]);
        zeroize_private_slice(preimage.as_mut_slice());
        tag.map(OperatorAuthenticationTag)
    }

    pub fn verify_authentication(
        &self,
        fence: OperatorHandshakeFence<'_>,
        presented: &OperatorAuthenticationTag,
    ) -> bool {
        let Ok(expected) = self.derive_authentication(fence) else {
            return false;
        };
        constant_time_equal(&expected.0, &presented.0)
    }

    pub fn session_digest(&self, fence: OperatorHandshakeFence<'_>) -> Result<Digest> {
        let authentication = self.derive_authentication(fence)?;
        let mut preimage = encode_operator_fence(fence)?;
        preimage.extend_from_slice(authentication.private_bytes());
        let digest = digest_from_bytes(sha256(&[
            OPERATOR_SESSION_DOMAIN,
            preimage.as_slice(),
        ])?);
        zeroize_private_slice(preimage.as_mut_slice());
        digest
    }
}

impl Drop for OperatorCapabilityKey {
    fn drop(&mut self) {
        zeroize_private_bytes(&mut self.0);
    }
}

impl OperatorAuthenticationTag {
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

impl Drop for OperatorAuthenticationTag {
    fn drop(&mut self) {
        zeroize_private_bytes(&mut self.0);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrantDisposition {
    Issued,
    Consumed,
    Revoked,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrantConsumption {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub worker_session_digest: Digest,
    pub grant_digest: Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrantLedgerSnapshot {
    pub entries: usize,
    pub issued: usize,
    pub consumed: usize,
    pub revoked: usize,
    pub expired: usize,
    pub max_entries: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GrantRecord {
    request_id: RequestId,
    request_generation: u64,
    backend_generation: u64,
    worker_session_digest: Digest,
    fence_digest: Digest,
    expires_unix_ms: u64,
    disposition: GrantDisposition,
}

#[derive(Debug)]
pub struct RequestGrantLedger {
    max_entries: usize,
    entries: BTreeMap<Digest, GrantRecord>,
}

impl RequestGrantLedger {
    pub fn new(max_entries: usize) -> Result<Self> {
        if max_entries == 0 {
            return Err(InferError::GrantLedgerFull);
        }
        Ok(Self {
            max_entries,
            entries: BTreeMap::new(),
        })
    }

    pub fn issue(
        &mut self,
        key: &CapabilityKey,
        fence: RequestGrantFence<'_>,
        now_unix_ms: u64,
        expires_unix_ms: u64,
    ) -> Result<RequestGrant> {
        if expires_unix_ms <= now_unix_ms {
            return Err(InferError::GrantExpiryInvalid);
        }
        if self.entries.len() >= self.max_entries {
            return Err(InferError::GrantLedgerFull);
        }
        let grant = key.derive_request_grant(fence)?;
        let grant_digest = grant.digest()?;
        if self.entries.contains_key(&grant_digest) {
            return Err(InferError::DuplicateGrant);
        }
        self.entries.insert(
            grant_digest,
            GrantRecord {
                request_id: fence.request.identity.request_id.clone(),
                request_generation: fence.request.request_generation,
                backend_generation: fence.backend_generation,
                worker_session_digest: fence.worker_session_digest.clone(),
                fence_digest: request_grant_fence_digest(fence)?,
                expires_unix_ms,
                disposition: GrantDisposition::Issued,
            },
        );
        Ok(grant)
    }

    pub fn consume(
        &mut self,
        key: &CapabilityKey,
        fence: RequestGrantFence<'_>,
        presented: &RequestGrant,
        now_unix_ms: u64,
    ) -> Result<GrantConsumption> {
        let grant_digest = presented.digest()?;
        let fence_digest = request_grant_fence_digest(fence)?;
        let verified = key.verify_request_grant(fence, presented);
        let record = self
            .entries
            .get_mut(&grant_digest)
            .ok_or(InferError::UnknownGrant)?;
        match record.disposition {
            GrantDisposition::Consumed => return Err(InferError::GrantAlreadyConsumed),
            GrantDisposition::Revoked => return Err(InferError::GrantRevoked),
            GrantDisposition::Expired => return Err(InferError::GrantExpired),
            GrantDisposition::Issued => {}
        }
        if now_unix_ms >= record.expires_unix_ms {
            record.disposition = GrantDisposition::Expired;
            return Err(InferError::GrantExpired);
        }
        if !verified || record.fence_digest != fence_digest {
            return Err(InferError::GrantFenceMismatch);
        }
        record.disposition = GrantDisposition::Consumed;
        Ok(GrantConsumption {
            request_id: record.request_id.clone(),
            request_generation: record.request_generation,
            backend_generation: record.backend_generation,
            worker_session_digest: record.worker_session_digest.clone(),
            grant_digest,
        })
    }

    pub fn revoke(&mut self, grant_digest: &Digest) -> Result<()> {
        let record = self
            .entries
            .get_mut(grant_digest)
            .ok_or(InferError::UnknownGrant)?;
        match record.disposition {
            GrantDisposition::Issued => {
                record.disposition = GrantDisposition::Revoked;
                Ok(())
            }
            GrantDisposition::Consumed => Err(InferError::GrantAlreadyConsumed),
            GrantDisposition::Revoked => Err(InferError::GrantRevoked),
            GrantDisposition::Expired => Err(InferError::GrantExpired),
        }
    }

    pub fn expire_before(&mut self, now_unix_ms: u64) -> usize {
        let mut expired = 0usize;
        for record in self.entries.values_mut() {
            if record.disposition == GrantDisposition::Issued
                && now_unix_ms >= record.expires_unix_ms
            {
                record.disposition = GrantDisposition::Expired;
                expired += 1;
            }
        }
        expired
    }

    pub fn revoke_backend_generation(&mut self, backend_generation: u64) -> usize {
        let mut revoked = 0usize;
        for record in self.entries.values_mut() {
            if record.backend_generation == backend_generation
                && record.disposition == GrantDisposition::Issued
            {
                record.disposition = GrantDisposition::Revoked;
                revoked += 1;
            }
        }
        revoked
    }

    pub fn disposition(&self, grant_digest: &Digest) -> Result<GrantDisposition> {
        self.entries
            .get(grant_digest)
            .map(|record| record.disposition)
            .ok_or(InferError::UnknownGrant)
    }

    pub fn snapshot(&self) -> GrantLedgerSnapshot {
        let mut snapshot = GrantLedgerSnapshot {
            entries: self.entries.len(),
            issued: 0,
            consumed: 0,
            revoked: 0,
            expired: 0,
            max_entries: self.max_entries,
        };
        for record in self.entries.values() {
            match record.disposition {
                GrantDisposition::Issued => snapshot.issued += 1,
                GrantDisposition::Consumed => snapshot.consumed += 1,
                GrantDisposition::Revoked => snapshot.revoked += 1,
                GrantDisposition::Expired => snapshot.expired += 1,
            }
        }
        snapshot
    }
}

fn validate_operator_fence(fence: OperatorHandshakeFence<'_>) -> Result<()> {
    if fence.operator_pid == 0 || fence.backend_generation == 0 {
        return Err(InferError::InvalidCapability);
    }
    Ok(())
}

fn encode_operator_fence(fence: OperatorHandshakeFence<'_>) -> Result<Vec<u8>> {
    let mut preimage = Vec::with_capacity(256);
    append_u64(&mut preimage, u64::from(fence.operator_pid));
    append_u64(&mut preimage, fence.backend_generation);
    append_text(&mut preimage, fence.operator_nonce_digest.as_str())?;
    append_text(&mut preimage, fence.daemon_challenge_digest.as_str())?;
    Ok(preimage)
}

fn request_grant_fence_digest(fence: RequestGrantFence<'_>) -> Result<Digest> {
    let request = fence.request;
    request.validate_shape()?;
    if fence.backend_generation == 0 {
        return Err(InferError::InvalidGeneration);
    }
    let mut preimage = Vec::with_capacity(1024);
    preimage.extend_from_slice(REQUEST_GRANT_FENCE_DOMAIN);
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
    let digest = digest_from_bytes(sha256(&[preimage.as_slice()])?);
    zeroize_private_slice(preimage.as_mut_slice());
    digest
}

#[cfg(unix)]
fn fill_os_random(bytes: &mut [u8]) -> Result<()> {
    let mut source = File::open("/dev/urandom").map_err(|_| InferError::EntropyUnavailable)?;
    source
        .read_exact(bytes)
        .map_err(|_| InferError::EntropyUnavailable)
}

#[cfg(not(unix))]
fn fill_os_random(_bytes: &mut [u8]) -> Result<()> {
    Err(InferError::EntropyUnavailable)
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
    let mut inner = sha256(&[inner_input.as_slice()])?;
    zeroize_private_slice(inner_input.as_mut_slice());
    zeroize_private_bytes(&mut inner_pad);

    let output = sha256(&[&outer_pad, &inner])?;
    zeroize_private_bytes(&mut inner);
    zeroize_private_bytes(&mut outer_pad);
    Ok(output)
}

fn constant_time_equal(
    left: &[u8; PRIVATE_AUTH_TAG_BYTES],
    right: &[u8; PRIVATE_AUTH_TAG_BYTES],
) -> bool {
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
    use crate::InferenceRequest;
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
                request_id: must(RequestId::parse("request-private-security")),
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

    #[test]
    fn os_entropy_generates_nonzero_independent_material() {
        let _request_key = must(generate_request_capability_key_os());
        let first = must(SessionNonce::generate_os().and_then(|nonce| nonce.digest()));
        let second = must(SessionNonce::generate_os().and_then(|nonce| nonce.digest()));
        assert_ne!(first, second);
        let _operator_key = must(OperatorCapabilityKey::generate_os());
    }

    #[test]
    fn operator_authentication_is_pid_generation_nonce_and_challenge_bound() {
        let key = must(OperatorCapabilityKey::from_private_bytes([
            9u8;
            CAPABILITY_KEY_BYTES
        ]));
        let nonce = digest('a');
        let challenge = digest('b');
        let fence = OperatorHandshakeFence {
            operator_pid: 43,
            backend_generation: 7,
            operator_nonce_digest: &nonce,
            daemon_challenge_digest: &challenge,
        };
        let tag = must(key.derive_authentication(fence));
        assert!(key.verify_authentication(fence, &tag));
        let session = must(key.session_digest(fence));
        assert_ne!(session, nonce);
        assert!(!key.verify_authentication(
            OperatorHandshakeFence {
                operator_pid: 44,
                ..fence
            },
            &tag,
        ));
    }

    #[test]
    fn request_grant_is_consumed_once_and_expires_fail_closed() {
        let key = must(CapabilityKey::from_private_bytes([
            7u8;
            CAPABILITY_KEY_BYTES
        ]));
        let request = request();
        let session = digest('d');
        let fence = RequestGrantFence {
            request: &request,
            backend_generation: 4,
            worker_session_digest: &session,
        };
        let mut ledger = must(RequestGrantLedger::new(4));
        let grant = must(ledger.issue(&key, fence, 10, 20));
        let grant_digest = must(grant.digest());
        let consumed = must(ledger.consume(&key, fence, &grant, 11));
        assert_eq!(consumed.request_id, request.identity.request_id);
        assert_eq!(
            ledger.consume(&key, fence, &grant, 12),
            Err(InferError::GrantAlreadyConsumed)
        );
        assert_eq!(
            ledger.disposition(&grant_digest),
            Ok(GrantDisposition::Consumed)
        );

        let other_request = request();
        let other_session = digest('e');
        let other_fence = RequestGrantFence {
            request: &other_request,
            backend_generation: 5,
            worker_session_digest: &other_session,
        };
        let expiring = must(ledger.issue(&key, other_fence, 10, 20));
        assert_eq!(
            ledger.consume(&key, other_fence, &expiring, 20),
            Err(InferError::GrantExpired)
        );
    }
}
