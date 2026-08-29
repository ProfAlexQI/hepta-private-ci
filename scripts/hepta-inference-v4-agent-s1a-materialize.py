#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

CAPABILITY = r'''//! Secret-bearing capability primitives for the private inference control plane.
//!
//! The public UDS protocol never serializes these values. The daemon owns an
//! epoch key, a bounded one-time grant ledger, and a distinct worker bootstrap
//! key. Raw secret bytes are visible only to the private protocol codec.

use std::collections::BTreeMap;

use constant_time_eq::constant_time_eq;
use hmac::Hmac;
use hmac::Mac;
use sha2::Sha256;
use zeroize::Zeroize;

use crate::Digest;
use crate::InferError;
use crate::InferenceRequest;
use crate::RequestId;
use crate::Result;
use crate::hashing::sha256;

pub const CAPABILITY_KEY_BYTES: usize = 32;
pub const GRANT_NONCE_BYTES: usize = 16;
pub const PRIVATE_AUTH_TAG_BYTES: usize = 32;
pub const MAX_GRANT_TTL_MS: u64 = 5 * 60 * 1_000;

const REQUEST_GRANT_DOMAIN: &[u8] = b"hepta.inference.request-grant.v2\0";
const REQUEST_GRANT_DIGEST_DOMAIN: &[u8] = b"hepta.inference.request-grant-digest.v2\0";
const WORKER_AUTH_DOMAIN: &[u8] = b"hepta.inference.worker-auth.v2\0";
const WORKER_SESSION_DOMAIN: &[u8] = b"hepta.inference.worker-session.v2\0";
const WORKER_NONCE_DOMAIN: &[u8] = b"hepta.inference.worker-nonce.v1\0";
const DAEMON_CHALLENGE_DOMAIN: &[u8] = b"hepta.inference.daemon-challenge.v1\0";

type HmacSha256 = Hmac<Sha256>;

/// Secret key. It intentionally implements neither `Clone` nor `Debug`.
pub struct CapabilityKey([u8; CAPABILITY_KEY_BYTES]);

/// Out-of-band bootstrap material for one inherited private worker channel.
/// It intentionally implements neither `Clone` nor `Debug`.
pub struct WorkerBootstrapToken([u8; CAPABILITY_KEY_BYTES]);

/// Raw one-time request grant. It intentionally implements neither `Clone` nor `Debug`.
pub struct RequestGrant {
    nonce: [u8; GRANT_NONCE_BYTES],
    tag: [u8; PRIVATE_AUTH_TAG_BYTES],
    issued_at_unix_ms: u64,
    expires_at_unix_ms: u64,
    purpose: GrantPurpose,
}

/// Challenge response for a private worker handshake.
/// It intentionally implements neither `Clone` nor `Debug`.
pub struct WorkerAuthenticationTag([u8; PRIVATE_AUTH_TAG_BYTES]);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum GrantPurpose {
    Execute = 1,
    Cancel = 2,
}

impl GrantPurpose {
    pub const fn code(self) -> u8 {
        self as u8
    }

    pub(crate) fn from_code(value: u8) -> Result<Self> {
        match value {
            value if value == Self::Execute.code() => Ok(Self::Execute),
            value if value == Self::Cancel.code() => Ok(Self::Cancel),
            _ => Err(InferError::CapabilityPurposeMismatch),
        }
    }
}

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrantClaim {
    pub grant_digest: Digest,
    pub request_id: RequestId,
    pub request_generation: u64,
    pub cancel_generation: u64,
    pub backend_generation: u64,
    pub worker_session_digest: Digest,
    pub purpose: GrantPurpose,
    pub expires_at_unix_ms: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GrantState {
    Pending,
    Claimed,
}

#[derive(Clone, Debug)]
struct GrantRecord {
    request_id: RequestId,
    request_generation: u64,
    cancel_generation: u64,
    backend_generation: u64,
    worker_session_digest: Digest,
    purpose: GrantPurpose,
    expires_at_unix_ms: u64,
    state: GrantState,
}

#[derive(Debug)]
pub struct RequestGrantLedger {
    max_entries: usize,
    records: BTreeMap<Digest, GrantRecord>,
}

impl CapabilityKey {
    pub fn generate() -> Result<Self> {
        Self::from_private_bytes(random_nonzero_array()?)
    }

    pub fn generate_worker_bootstrap() -> Result<(Self, WorkerBootstrapToken)> {
        let bytes = random_nonzero_array()?;
        Ok((Self(bytes), WorkerBootstrapToken(bytes)))
    }

    pub(crate) fn from_private_bytes(bytes: [u8; CAPABILITY_KEY_BYTES]) -> Result<Self> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(InferError::InvalidCapabilityKey);
        }
        Ok(Self(bytes))
    }

    pub fn mint_request_grant(
        &self,
        fence: RequestGrantFence<'_>,
        purpose: GrantPurpose,
        issued_at_unix_ms: u64,
        ttl_ms: u64,
    ) -> Result<RequestGrant> {
        validate_request_grant_fence(fence)?;
        if ttl_ms == 0 || ttl_ms > MAX_GRANT_TTL_MS {
            return Err(InferError::InvalidCapability);
        }
        let expires_at_unix_ms = issued_at_unix_ms
            .checked_add(ttl_ms)
            .ok_or(InferError::InvalidCapability)?;
        let nonce = random_nonzero_array()?;
        let preimage = encode_request_grant_fence(
            fence,
            purpose,
            issued_at_unix_ms,
            expires_at_unix_ms,
            &nonce,
        )?;
        let tag = hmac_sha256(&self.0, &[REQUEST_GRANT_DOMAIN, preimage.as_slice()])?;
        Ok(RequestGrant {
            nonce,
            tag,
            issued_at_unix_ms,
            expires_at_unix_ms,
            purpose,
        })
    }

    pub fn verify_request_grant(
        &self,
        fence: RequestGrantFence<'_>,
        purpose: GrantPurpose,
        now_unix_ms: u64,
        presented: &RequestGrant,
    ) -> Result<()> {
        validate_request_grant_fence(fence)?;
        if presented.purpose != purpose {
            return Err(InferError::CapabilityPurposeMismatch);
        }
        if now_unix_ms < presented.issued_at_unix_ms {
            return Err(InferError::InvalidCapability);
        }
        if now_unix_ms >= presented.expires_at_unix_ms {
            return Err(InferError::CapabilityExpired);
        }
        let preimage = encode_request_grant_fence(
            fence,
            purpose,
            presented.issued_at_unix_ms,
            presented.expires_at_unix_ms,
            &presented.nonce,
        )?;
        let expected = hmac_sha256(&self.0, &[REQUEST_GRANT_DOMAIN, preimage.as_slice()])?;
        if constant_time_equal(&expected, &presented.tag) {
            Ok(())
        } else {
            Err(InferError::InvalidCapability)
        }
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
        preimage.extend_from_slice(&authentication.0);
        digest_from_bytes(sha256(&[WORKER_SESSION_DOMAIN, preimage.as_slice()])?)
    }
}

impl Drop for CapabilityKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl WorkerBootstrapToken {
    pub fn into_capability_key(mut self) -> Result<CapabilityKey> {
        let mut bytes = [0u8; CAPABILITY_KEY_BYTES];
        bytes.copy_from_slice(&self.0);
        self.0.zeroize();
        CapabilityKey::from_private_bytes(bytes)
    }

    pub(crate) fn from_wire(bytes: [u8; CAPABILITY_KEY_BYTES]) -> Result<Self> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(InferError::InvalidCapabilityKey);
        }
        Ok(Self(bytes))
    }

    pub(crate) fn wire_bytes(&self) -> &[u8; CAPABILITY_KEY_BYTES] {
        &self.0
    }
}

impl Drop for WorkerBootstrapToken {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl RequestGrant {
    pub fn digest(&self) -> Result<Digest> {
        let mut metadata = Vec::with_capacity(64);
        metadata.push(self.purpose.code());
        append_u64(&mut metadata, self.issued_at_unix_ms);
        append_u64(&mut metadata, self.expires_at_unix_ms);
        digest_from_bytes(sha256(&[
            REQUEST_GRANT_DIGEST_DOMAIN,
            metadata.as_slice(),
            &self.nonce,
            &self.tag,
        ])?)
    }

    pub const fn issued_at_unix_ms(&self) -> u64 {
        self.issued_at_unix_ms
    }

    pub const fn expires_at_unix_ms(&self) -> u64 {
        self.expires_at_unix_ms
    }

    pub const fn purpose(&self) -> GrantPurpose {
        self.purpose
    }

    pub(crate) fn from_wire(
        nonce: [u8; GRANT_NONCE_BYTES],
        tag: [u8; PRIVATE_AUTH_TAG_BYTES],
        issued_at_unix_ms: u64,
        expires_at_unix_ms: u64,
        purpose: GrantPurpose,
    ) -> Result<Self> {
        if nonce.iter().all(|byte| *byte == 0)
            || tag.iter().all(|byte| *byte == 0)
            || issued_at_unix_ms >= expires_at_unix_ms
            || expires_at_unix_ms - issued_at_unix_ms > MAX_GRANT_TTL_MS
        {
            return Err(InferError::InvalidCapability);
        }
        Ok(Self {
            nonce,
            tag,
            issued_at_unix_ms,
            expires_at_unix_ms,
            purpose,
        })
    }

    pub(crate) fn nonce_bytes(&self) -> &[u8; GRANT_NONCE_BYTES] {
        &self.nonce
    }

    pub(crate) fn tag_bytes(&self) -> &[u8; PRIVATE_AUTH_TAG_BYTES] {
        &self.tag
    }
}

impl Drop for RequestGrant {
    fn drop(&mut self) {
        self.nonce.zeroize();
        self.tag.zeroize();
    }
}

impl WorkerAuthenticationTag {
    pub(crate) fn from_wire(bytes: [u8; PRIVATE_AUTH_TAG_BYTES]) -> Result<Self> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(InferError::InvalidCapability);
        }
        Ok(Self(bytes))
    }

    pub(crate) fn wire_bytes(&self) -> &[u8; PRIVATE_AUTH_TAG_BYTES] {
        &self.0
    }
}

impl Drop for WorkerAuthenticationTag {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl RequestGrantLedger {
    pub fn new(max_entries: usize) -> Result<Self> {
        if max_entries == 0 {
            return Err(InferError::InvalidCapability);
        }
        Ok(Self {
            max_entries,
            records: BTreeMap::new(),
        })
    }

    pub fn issue(
        &mut self,
        key: &CapabilityKey,
        fence: RequestGrantFence<'_>,
        purpose: GrantPurpose,
        now_unix_ms: u64,
        ttl_ms: u64,
    ) -> Result<RequestGrant> {
        self.prune_expired(now_unix_ms);
        if self.records.len() >= self.max_entries {
            return Err(InferError::CapabilityLedgerFull);
        }
        let grant = key.mint_request_grant(fence, purpose, now_unix_ms, ttl_ms)?;
        let digest = grant.digest()?;
        if self.records.contains_key(&digest) {
            return Err(InferError::InvalidCapability);
        }
        self.records.insert(
            digest,
            GrantRecord {
                request_id: fence.request.identity.request_id.clone(),
                request_generation: fence.request.request_generation,
                cancel_generation: fence.request.cancel_generation,
                backend_generation: fence.backend_generation,
                worker_session_digest: fence.worker_session_digest.clone(),
                purpose,
                expires_at_unix_ms: grant.expires_at_unix_ms,
                state: GrantState::Pending,
            },
        );
        Ok(grant)
    }

    pub fn claim(
        &mut self,
        key: &CapabilityKey,
        fence: RequestGrantFence<'_>,
        purpose: GrantPurpose,
        now_unix_ms: u64,
        presented: &RequestGrant,
    ) -> Result<GrantClaim> {
        key.verify_request_grant(fence, purpose, now_unix_ms, presented)?;
        let digest = presented.digest()?;
        let record = self
            .records
            .get_mut(&digest)
            .ok_or(InferError::CapabilityUnknown)?;
        if record.request_id != fence.request.identity.request_id
            || record.request_generation != fence.request.request_generation
            || record.cancel_generation != fence.request.cancel_generation
            || record.backend_generation != fence.backend_generation
            || record.worker_session_digest != *fence.worker_session_digest
            || record.purpose != purpose
            || record.expires_at_unix_ms != presented.expires_at_unix_ms
        {
            return Err(InferError::InvalidCapability);
        }
        if record.state == GrantState::Claimed {
            return Err(InferError::CapabilityReplay);
        }
        record.state = GrantState::Claimed;
        Ok(GrantClaim {
            grant_digest: digest,
            request_id: record.request_id.clone(),
            request_generation: record.request_generation,
            cancel_generation: record.cancel_generation,
            backend_generation: record.backend_generation,
            worker_session_digest: record.worker_session_digest.clone(),
            purpose: record.purpose,
            expires_at_unix_ms: record.expires_at_unix_ms,
        })
    }

    pub fn prune_expired(&mut self, now_unix_ms: u64) -> usize {
        let before = self.records.len();
        self.records
            .retain(|_, record| record.expires_at_unix_ms > now_unix_ms);
        before - self.records.len()
    }

    pub fn invalidate_session(&mut self, worker_session_digest: &Digest) -> usize {
        let before = self.records.len();
        self.records
            .retain(|_, record| record.worker_session_digest != *worker_session_digest);
        before - self.records.len()
    }

    pub fn invalidate_backend_generation(&mut self, backend_generation: u64) -> usize {
        let before = self.records.len();
        self.records
            .retain(|_, record| record.backend_generation != backend_generation);
        before - self.records.len()
    }

    pub fn pending_entries(&self) -> usize {
        self.records
            .values()
            .filter(|record| record.state == GrantState::Pending)
            .count()
    }

    pub fn claimed_entries(&self) -> usize {
        self.records
            .values()
            .filter(|record| record.state == GrantState::Claimed)
            .count()
    }

    pub fn total_entries(&self) -> usize {
        self.records.len()
    }
}

pub fn generate_worker_nonce_digest() -> Result<Digest> {
    random_digest(WORKER_NONCE_DOMAIN)
}

pub fn generate_daemon_challenge_digest() -> Result<Digest> {
    random_digest(DAEMON_CHALLENGE_DOMAIN)
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

fn encode_request_grant_fence(
    fence: RequestGrantFence<'_>,
    purpose: GrantPurpose,
    issued_at_unix_ms: u64,
    expires_at_unix_ms: u64,
    nonce: &[u8; GRANT_NONCE_BYTES],
) -> Result<Vec<u8>> {
    let request = fence.request;
    let mut preimage = Vec::with_capacity(1_024);
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
    preimage.push(purpose.code());
    append_u64(&mut preimage, issued_at_unix_ms);
    append_u64(&mut preimage, expires_at_unix_ms);
    preimage.extend_from_slice(nonce);
    Ok(preimage)
}

fn hmac_sha256(key: &[u8], parts: &[&[u8]]) -> Result<[u8; PRIVATE_AUTH_TAG_BYTES]> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|_| InferError::InvalidCapabilityKey)?;
    for part in parts {
        mac.update(part);
    }
    let code = mac.finalize().into_bytes();
    let mut output = [0u8; PRIVATE_AUTH_TAG_BYTES];
    output.copy_from_slice(&code);
    Ok(output)
}

fn constant_time_equal(
    left: &[u8; PRIVATE_AUTH_TAG_BYTES],
    right: &[u8; PRIVATE_AUTH_TAG_BYTES],
) -> bool {
    constant_time_eq(left, right)
}

fn random_nonzero_array<const N: usize>() -> Result<[u8; N]> {
    let mut bytes = [0u8; N];
    if getrandom::fill(&mut bytes).is_err() {
        bytes.zeroize();
        return Err(InferError::EntropyUnavailable);
    }
    if bytes.iter().all(|byte| *byte == 0) {
        bytes.zeroize();
        return Err(InferError::EntropyUnavailable);
    }
    Ok(bytes)
}

fn random_digest(domain: &[u8]) -> Result<Digest> {
    let mut entropy = random_nonzero_array::<CAPABILITY_KEY_BYTES>()?;
    let result = digest_from_bytes(sha256(&[domain, &entropy])?);
    entropy.zeroize();
    result
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgentId;
    use crate::AuthoritySnapshot;
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

    fn test_key(fill: u8) -> CapabilityKey {
        must(CapabilityKey::from_private_bytes([fill; CAPABILITY_KEY_BYTES]))
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
    fn bootstrap_pair_authenticates_without_exposing_raw_bytes() {
        let (daemon_key, token) = must(CapabilityKey::generate_worker_bootstrap());
        let worker_key = must(token.into_capability_key());
        let nonce = must(generate_worker_nonce_digest());
        let challenge = must(generate_daemon_challenge_digest());
        let fence = WorkerHandshakeFence {
            worker_pid: 42,
            backend_generation: 7,
            worker_nonce_digest: &nonce,
            daemon_challenge_digest: &challenge,
        };
        let tag = must(worker_key.derive_worker_authentication(fence));
        assert!(daemon_key.verify_worker_authentication(fence, &tag));
        assert_eq!(
            must(daemon_key.worker_session_digest(fence)),
            must(worker_key.worker_session_digest(fence))
        );
    }

    #[test]
    fn one_fence_mints_unique_grants_and_claim_is_atomic_once() {
        let key = test_key(7);
        let request = request();
        let session = digest('d');
        let fence = RequestGrantFence {
            request: &request,
            backend_generation: 4,
            worker_session_digest: &session,
        };
        let mut ledger = must(RequestGrantLedger::new(4));
        let first = must(ledger.issue(&key, fence, GrantPurpose::Execute, 100, 50));
        let second = must(ledger.issue(&key, fence, GrantPurpose::Execute, 100, 50));
        assert_ne!(must(first.digest()), must(second.digest()));
        let claim = must(ledger.claim(
            &key,
            fence,
            GrantPurpose::Execute,
            101,
            &first,
        ));
        assert_eq!(claim.request_id, request.identity.request_id);
        assert_eq!(ledger.pending_entries(), 1);
        assert_eq!(ledger.claimed_entries(), 1);
        assert_eq!(
            ledger.claim(&key, fence, GrantPurpose::Execute, 102, &first),
            Err(InferError::CapabilityReplay)
        );
    }

    #[test]
    fn expiry_purpose_session_and_generation_fail_closed() {
        let key = test_key(9);
        let request = request();
        let session = digest('d');
        let other_session = digest('e');
        let fence = RequestGrantFence {
            request: &request,
            backend_generation: 4,
            worker_session_digest: &session,
        };
        let mut ledger = must(RequestGrantLedger::new(2));
        let grant = must(ledger.issue(&key, fence, GrantPurpose::Execute, 100, 10));
        assert_eq!(
            ledger.claim(&key, fence, GrantPurpose::Cancel, 101, &grant),
            Err(InferError::CapabilityPurposeMismatch)
        );
        assert_eq!(
            ledger.claim(
                &key,
                RequestGrantFence {
                    request: &request,
                    backend_generation: 4,
                    worker_session_digest: &other_session,
                },
                GrantPurpose::Execute,
                101,
                &grant,
            ),
            Err(InferError::InvalidCapability)
        );
        assert_eq!(
            ledger.claim(
                &key,
                RequestGrantFence {
                    request: &request,
                    backend_generation: 5,
                    worker_session_digest: &session,
                },
                GrantPurpose::Execute,
                101,
                &grant,
            ),
            Err(InferError::InvalidCapability)
        );
        assert_eq!(
            ledger.claim(&key, fence, GrantPurpose::Execute, 110, &grant),
            Err(InferError::CapabilityExpired)
        );
        assert_eq!(ledger.prune_expired(110), 1);
        assert_eq!(ledger.total_entries(), 0);
    }

    #[test]
    fn ledger_is_bounded_and_generation_invalidation_is_deterministic() {
        let key = test_key(5);
        let request = request();
        let session = digest('d');
        let fence = RequestGrantFence {
            request: &request,
            backend_generation: 4,
            worker_session_digest: &session,
        };
        let mut ledger = must(RequestGrantLedger::new(1));
        let _grant = must(ledger.issue(&key, fence, GrantPurpose::Execute, 100, 50));
        assert!(matches!(
            ledger.issue(&key, fence, GrantPurpose::Execute, 101, 50),
            Err(InferError::CapabilityLedgerFull)
        ));
        assert_eq!(ledger.invalidate_backend_generation(4), 1);
        assert_eq!(ledger.total_entries(), 0);
    }
}
'''

PRIVATE_PROTOCOL = r'''//! Canonical typed protocol for an inherited private worker channel.
//!
//! Secret-bearing values implement redacted `Debug`; raw bytes are only read or
//! written inside this module. Public client messages cannot be decoded as this
//! protocol because each direction has a distinct magic and tag space.

use std::fmt;

use crate::AcceptedEvent;
use crate::ClientMessage;
use crate::Digest;
use crate::GRANT_NONCE_BYTES;
use crate::GrantPurpose;
use crate::InferError;
use crate::InferenceRequest;
use crate::PRIVATE_AUTH_TAG_BYTES;
use crate::RequestGrant;
use crate::RequestId;
use crate::Result;
use crate::ServerMessage;
use crate::StateEvent;
use crate::TerminalReceipt;
use crate::WorkerAuthenticationTag;
use crate::WorkerBootstrapToken;
use crate::CAPABILITY_KEY_BYTES;

pub const PRIVATE_PROTOCOL_VERSION: u8 = 1;
pub const MAX_PRIVATE_FRAME_BYTES: usize = 64 * 1024;

const WORKER_MAGIC: &[u8; 4] = b"HPW1";
const DAEMON_MAGIC: &[u8; 4] = b"HPD1";
const BOOTSTRAP_MAGIC: &[u8; 4] = b"HPB1";

const WORKER_HELLO: u8 = 1;
const WORKER_AUTHENTICATE: u8 = 2;
const WORKER_READY: u8 = 3;
const WORKER_START_ACK: u8 = 4;
const WORKER_TOKEN: u8 = 5;
const WORKER_COMPLETE: u8 = 6;

const DAEMON_CHALLENGE: u8 = 101;
const DAEMON_AUTHENTICATED: u8 = 102;
const DAEMON_DISPATCH: u8 = 103;
const DAEMON_STATE: u8 = 104;
const DAEMON_RECEIPT: u8 = 105;
const DAEMON_IDLE: u8 = 106;
const DAEMON_ERROR: u8 = 199;

pub struct WorkerBootstrapEnvelope {
    token: WorkerBootstrapToken,
}

pub enum WorkerToDaemon {
    Hello {
        worker_pid: u32,
        backend_generation: u64,
        worker_nonce_digest: Digest,
    },
    Authenticate {
        worker_pid: u32,
        backend_generation: u64,
        worker_nonce_digest: Digest,
        daemon_challenge_digest: Digest,
        authentication: WorkerAuthenticationTag,
    },
    Ready {
        worker_session_digest: Digest,
    },
    StartAck {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        worker_session_digest: Digest,
        grant: RequestGrant,
    },
    Token {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        worker_session_digest: Digest,
        token_digest: Digest,
        token_byte_length: u64,
    },
    Complete {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        worker_session_digest: Digest,
        result_digest: Digest,
        output_tokens: u32,
    },
}

pub enum DaemonToWorker {
    Challenge {
        backend_generation: u64,
        daemon_challenge_digest: Digest,
        expires_at_unix_ms: u64,
    },
    Authenticated {
        worker_session_digest: Digest,
    },
    Dispatch {
        request: InferenceRequest,
        backend_generation: u64,
        worker_session_digest: Digest,
        grant: RequestGrant,
    },
    State(StateEvent),
    Receipt(TerminalReceipt),
    Idle,
    Error {
        code: String,
    },
}

impl WorkerBootstrapEnvelope {
    pub fn new(token: WorkerBootstrapToken) -> Self {
        Self { token }
    }

    pub fn encode(self) -> Result<Vec<u8>> {
        let mut writer = Writer::new(BOOTSTRAP_MAGIC, PRIVATE_PROTOCOL_VERSION);
        writer.fixed(self.token.wire_bytes());
        writer.finish()
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes, BOOTSTRAP_MAGIC)?;
        if reader.kind != PRIVATE_PROTOCOL_VERSION {
            return Err(InferError::ProtocolVersion);
        }
        let token = WorkerBootstrapToken::from_wire(reader.fixed()?)?;
        reader.finish()?;
        Ok(Self { token })
    }

    pub fn into_token(self) -> WorkerBootstrapToken {
        self.token
    }
}

impl WorkerToDaemon {
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        let kind = match self {
            Self::Hello { .. } => WORKER_HELLO,
            Self::Authenticate { .. } => WORKER_AUTHENTICATE,
            Self::Ready { .. } => WORKER_READY,
            Self::StartAck { .. } => WORKER_START_ACK,
            Self::Token { .. } => WORKER_TOKEN,
            Self::Complete { .. } => WORKER_COMPLETE,
        };
        let mut writer = Writer::new(WORKER_MAGIC, kind);
        match self {
            Self::Hello {
                worker_pid,
                backend_generation,
                worker_nonce_digest,
            } => {
                writer.u32(*worker_pid);
                writer.u64(*backend_generation);
                writer.text(worker_nonce_digest.as_str())?;
            }
            Self::Authenticate {
                worker_pid,
                backend_generation,
                worker_nonce_digest,
                daemon_challenge_digest,
                authentication,
            } => {
                writer.u32(*worker_pid);
                writer.u64(*backend_generation);
                writer.text(worker_nonce_digest.as_str())?;
                writer.text(daemon_challenge_digest.as_str())?;
                writer.fixed(authentication.wire_bytes());
            }
            Self::Ready {
                worker_session_digest,
            } => writer.text(worker_session_digest.as_str())?,
            Self::StartAck {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                worker_session_digest,
                grant,
            } => {
                writer.text(request_id.as_str())?;
                writer.u64(*request_generation);
                writer.u64(*backend_generation);
                writer.u64(*sequence);
                writer.text(worker_session_digest.as_str())?;
                encode_grant(&mut writer, grant);
            }
            Self::Token {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                worker_session_digest,
                token_digest,
                token_byte_length,
            } => {
                writer.text(request_id.as_str())?;
                writer.u64(*request_generation);
                writer.u64(*backend_generation);
                writer.u64(*sequence);
                writer.text(worker_session_digest.as_str())?;
                writer.text(token_digest.as_str())?;
                writer.u64(*token_byte_length);
            }
            Self::Complete {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                worker_session_digest,
                result_digest,
                output_tokens,
            } => {
                writer.text(request_id.as_str())?;
                writer.u64(*request_generation);
                writer.u64(*backend_generation);
                writer.u64(*sequence);
                writer.text(worker_session_digest.as_str())?;
                writer.text(result_digest.as_str())?;
                writer.u32(*output_tokens);
            }
        }
        writer.finish()
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes, WORKER_MAGIC)?;
        let message = match reader.kind {
            WORKER_HELLO => Self::Hello {
                worker_pid: reader.u32()?,
                backend_generation: reader.u64()?,
                worker_nonce_digest: read_digest(&mut reader)?,
            },
            WORKER_AUTHENTICATE => Self::Authenticate {
                worker_pid: reader.u32()?,
                backend_generation: reader.u64()?,
                worker_nonce_digest: read_digest(&mut reader)?,
                daemon_challenge_digest: read_digest(&mut reader)?,
                authentication: WorkerAuthenticationTag::from_wire(reader.fixed()?)?,
            },
            WORKER_READY => Self::Ready {
                worker_session_digest: read_digest(&mut reader)?,
            },
            WORKER_START_ACK => Self::StartAck {
                request_id: read_request_id(&mut reader)?,
                request_generation: reader.u64()?,
                backend_generation: reader.u64()?,
                sequence: reader.u64()?,
                worker_session_digest: read_digest(&mut reader)?,
                grant: decode_grant(&mut reader)?,
            },
            WORKER_TOKEN => Self::Token {
                request_id: read_request_id(&mut reader)?,
                request_generation: reader.u64()?,
                backend_generation: reader.u64()?,
                sequence: reader.u64()?,
                worker_session_digest: read_digest(&mut reader)?,
                token_digest: read_digest(&mut reader)?,
                token_byte_length: reader.u64()?,
            },
            WORKER_COMPLETE => Self::Complete {
                request_id: read_request_id(&mut reader)?,
                request_generation: reader.u64()?,
                backend_generation: reader.u64()?,
                sequence: reader.u64()?,
                worker_session_digest: read_digest(&mut reader)?,
                result_digest: read_digest(&mut reader)?,
                output_tokens: reader.u32()?,
            },
            _ => return Err(InferError::ProtocolShape),
        };
        reader.finish()?;
        Ok(message)
    }
}

impl DaemonToWorker {
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        let kind = match self {
            Self::Challenge { .. } => DAEMON_CHALLENGE,
            Self::Authenticated { .. } => DAEMON_AUTHENTICATED,
            Self::Dispatch { .. } => DAEMON_DISPATCH,
            Self::State(_) => DAEMON_STATE,
            Self::Receipt(_) => DAEMON_RECEIPT,
            Self::Idle => DAEMON_IDLE,
            Self::Error { .. } => DAEMON_ERROR,
        };
        let mut writer = Writer::new(DAEMON_MAGIC, kind);
        match self {
            Self::Challenge {
                backend_generation,
                daemon_challenge_digest,
                expires_at_unix_ms,
            } => {
                writer.u64(*backend_generation);
                writer.text(daemon_challenge_digest.as_str())?;
                writer.u64(*expires_at_unix_ms);
            }
            Self::Authenticated {
                worker_session_digest,
            } => writer.text(worker_session_digest.as_str())?,
            Self::Dispatch {
                request,
                backend_generation,
                worker_session_digest,
                grant,
            } => {
                let nested = ClientMessage::Admit(request.clone()).encode_canonical()?;
                writer.bytes(&nested)?;
                writer.u64(*backend_generation);
                writer.text(worker_session_digest.as_str())?;
                encode_grant(&mut writer, grant);
            }
            Self::State(event) => {
                let nested = ServerMessage::State(event.clone()).encode_canonical()?;
                writer.bytes(&nested)?;
            }
            Self::Receipt(receipt) => {
                let nested = ServerMessage::Receipt(receipt.clone()).encode_canonical()?;
                writer.bytes(&nested)?;
            }
            Self::Idle => {}
            Self::Error { code } => writer.text(code)?,
        }
        writer.finish()
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes, DAEMON_MAGIC)?;
        let message = match reader.kind {
            DAEMON_CHALLENGE => Self::Challenge {
                backend_generation: reader.u64()?,
                daemon_challenge_digest: read_digest(&mut reader)?,
                expires_at_unix_ms: reader.u64()?,
            },
            DAEMON_AUTHENTICATED => Self::Authenticated {
                worker_session_digest: read_digest(&mut reader)?,
            },
            DAEMON_DISPATCH => {
                let request = match ClientMessage::decode_canonical(reader.bytes()?)? {
                    ClientMessage::Admit(request) => request,
                    _ => return Err(InferError::ProtocolShape),
                };
                Self::Dispatch {
                    request,
                    backend_generation: reader.u64()?,
                    worker_session_digest: read_digest(&mut reader)?,
                    grant: decode_grant(&mut reader)?,
                }
            }
            DAEMON_STATE => match ServerMessage::decode_canonical(reader.bytes()?)? {
                ServerMessage::State(event) => Self::State(event),
                _ => return Err(InferError::ProtocolShape),
            },
            DAEMON_RECEIPT => match ServerMessage::decode_canonical(reader.bytes()?)? {
                ServerMessage::Receipt(receipt) => Self::Receipt(receipt),
                _ => return Err(InferError::ProtocolShape),
            },
            DAEMON_IDLE => Self::Idle,
            DAEMON_ERROR => Self::Error {
                code: reader.text()?,
            },
            _ => return Err(InferError::ProtocolShape),
        };
        reader.finish()?;
        Ok(message)
    }
}

impl fmt::Debug for WorkerBootstrapEnvelope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WorkerBootstrapEnvelope(<redacted>)")
    }
}

impl fmt::Debug for WorkerToDaemon {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hello {
                worker_pid,
                backend_generation,
                worker_nonce_digest,
            } => formatter
                .debug_struct("Hello")
                .field("worker_pid", worker_pid)
                .field("backend_generation", backend_generation)
                .field("worker_nonce_digest", worker_nonce_digest)
                .finish(),
            Self::Authenticate {
                worker_pid,
                backend_generation,
                worker_nonce_digest,
                daemon_challenge_digest,
                ..
            } => formatter
                .debug_struct("Authenticate")
                .field("worker_pid", worker_pid)
                .field("backend_generation", backend_generation)
                .field("worker_nonce_digest", worker_nonce_digest)
                .field("daemon_challenge_digest", daemon_challenge_digest)
                .field("authentication", &"<redacted>")
                .finish(),
            Self::Ready {
                worker_session_digest,
            } => formatter
                .debug_struct("Ready")
                .field("worker_session_digest", worker_session_digest)
                .finish(),
            Self::StartAck {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                worker_session_digest,
                ..
            } => formatter
                .debug_struct("StartAck")
                .field("request_id", request_id)
                .field("request_generation", request_generation)
                .field("backend_generation", backend_generation)
                .field("sequence", sequence)
                .field("worker_session_digest", worker_session_digest)
                .field("grant", &"<redacted>")
                .finish(),
            Self::Token {
                request_id,
                sequence,
                worker_session_digest,
                token_digest,
                token_byte_length,
                ..
            } => formatter
                .debug_struct("Token")
                .field("request_id", request_id)
                .field("sequence", sequence)
                .field("worker_session_digest", worker_session_digest)
                .field("token_digest", token_digest)
                .field("token_byte_length", token_byte_length)
                .finish(),
            Self::Complete {
                request_id,
                sequence,
                worker_session_digest,
                result_digest,
                output_tokens,
                ..
            } => formatter
                .debug_struct("Complete")
                .field("request_id", request_id)
                .field("sequence", sequence)
                .field("worker_session_digest", worker_session_digest)
                .field("result_digest", result_digest)
                .field("output_tokens", output_tokens)
                .finish(),
        }
    }
}

impl fmt::Debug for DaemonToWorker {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Challenge {
                backend_generation,
                daemon_challenge_digest,
                expires_at_unix_ms,
            } => formatter
                .debug_struct("Challenge")
                .field("backend_generation", backend_generation)
                .field("daemon_challenge_digest", daemon_challenge_digest)
                .field("expires_at_unix_ms", expires_at_unix_ms)
                .finish(),
            Self::Authenticated {
                worker_session_digest,
            } => formatter
                .debug_struct("Authenticated")
                .field("worker_session_digest", worker_session_digest)
                .finish(),
            Self::Dispatch {
                request,
                backend_generation,
                worker_session_digest,
                ..
            } => formatter
                .debug_struct("Dispatch")
                .field("request_id", &request.identity.request_id)
                .field("backend_generation", backend_generation)
                .field("worker_session_digest", worker_session_digest)
                .field("grant", &"<redacted>")
                .finish(),
            Self::State(event) => formatter.debug_tuple("State").field(event).finish(),
            Self::Receipt(receipt) => formatter.debug_tuple("Receipt").field(receipt).finish(),
            Self::Idle => formatter.write_str("Idle"),
            Self::Error { code } => formatter.debug_struct("Error").field("code", code).finish(),
        }
    }
}

fn encode_grant(writer: &mut Writer, grant: &RequestGrant) {
    writer.u8(grant.purpose().code());
    writer.u64(grant.issued_at_unix_ms());
    writer.u64(grant.expires_at_unix_ms());
    writer.fixed(grant.nonce_bytes());
    writer.fixed(grant.tag_bytes());
}

fn decode_grant(reader: &mut Reader<'_>) -> Result<RequestGrant> {
    let purpose = GrantPurpose::from_code(reader.u8()?)?;
    let issued_at_unix_ms = reader.u64()?;
    let expires_at_unix_ms = reader.u64()?;
    let nonce = reader.fixed()?;
    let tag = reader.fixed()?;
    RequestGrant::from_wire(
        nonce,
        tag,
        issued_at_unix_ms,
        expires_at_unix_ms,
        purpose,
    )
}

fn read_digest(reader: &mut Reader<'_>) -> Result<Digest> {
    Digest::parse(&reader.text()?)
}

fn read_request_id(reader: &mut Reader<'_>) -> Result<RequestId> {
    RequestId::parse(&reader.text()?)
}

struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn new(magic: &[u8; 4], kind: u8) -> Self {
        let mut bytes = Vec::with_capacity(256);
        bytes.extend_from_slice(magic);
        bytes.push(kind);
        Self { bytes }
    }

    fn finish(self) -> Result<Vec<u8>> {
        if self.bytes.len() > MAX_PRIVATE_FRAME_BYTES {
            Err(InferError::ProtocolBound)
        } else {
            Ok(self.bytes)
        }
    }

    fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn fixed<const N: usize>(&mut self, value: &[u8; N]) {
        self.bytes.extend_from_slice(value);
    }

    fn bytes(&mut self, value: &[u8]) -> Result<()> {
        if value.is_empty() || value.len() > MAX_PRIVATE_FRAME_BYTES {
            return Err(InferError::ProtocolBound);
        }
        let length = u32::try_from(value.len()).map_err(|_| InferError::ProtocolBound)?;
        self.u32(length);
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    fn text(&mut self, value: &str) -> Result<()> {
        self.bytes(value.as_bytes())
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    kind: u8,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8], magic: &[u8; 4]) -> Result<Self> {
        if bytes.len() < 5 || bytes.len() > MAX_PRIVATE_FRAME_BYTES || &bytes[..4] != magic {
            return Err(InferError::ProtocolShape);
        }
        Ok(Self {
            bytes,
            cursor: 5,
            kind: bytes[4],
        })
    }

    fn finish(self) -> Result<()> {
        if self.cursor == self.bytes.len() {
            Ok(())
        } else {
            Err(InferError::ProtocolTrailingData)
        }
    }

    fn u8(&mut self) -> Result<u8> {
        let byte = *self
            .bytes
            .get(self.cursor)
            .ok_or(InferError::ProtocolTruncated)?;
        self.cursor += 1;
        Ok(byte)
    }

    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.fixed()?))
    }

    fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_be_bytes(self.fixed()?))
    }

    fn fixed<const N: usize>(&mut self) -> Result<[u8; N]> {
        let end = self
            .cursor
            .checked_add(N)
            .ok_or(InferError::ProtocolBound)?;
        let slice = self
            .bytes
            .get(self.cursor..end)
            .ok_or(InferError::ProtocolTruncated)?;
        let mut output = [0u8; N];
        output.copy_from_slice(slice);
        self.cursor = end;
        Ok(output)
    }

    fn bytes(&mut self) -> Result<&'a [u8]> {
        let length = usize::try_from(self.u32()?).map_err(|_| InferError::ProtocolBound)?;
        if length == 0 || length > MAX_PRIVATE_FRAME_BYTES {
            return Err(InferError::ProtocolBound);
        }
        let end = self
            .cursor
            .checked_add(length)
            .ok_or(InferError::ProtocolBound)?;
        let value = self
            .bytes
            .get(self.cursor..end)
            .ok_or(InferError::ProtocolTruncated)?;
        self.cursor = end;
        Ok(value)
    }

    fn text(&mut self) -> Result<String> {
        std::str::from_utf8(self.bytes()?)
            .map(str::to_owned)
            .map_err(|_| InferError::ProtocolUtf8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgentId;
    use crate::AuthoritySnapshot;
    use crate::CapabilityKey;
    use crate::GrantPurpose;
    use crate::RequestGrantFence;
    use crate::RequestGrantLedger;
    use crate::RequestIdentity;
    use crate::ResourceBudgetId;
    use crate::TaskId;
    use crate::TenantId;
    use crate::WorkerHandshakeFence;
    use crate::WorkspaceId;
    use crate::generate_daemon_challenge_digest;
    use crate::generate_worker_nonce_digest;

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
                request_id: must(RequestId::parse("request-private-protocol")),
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
    fn bootstrap_envelope_round_trip_authenticates_and_debug_is_redacted() {
        let (daemon_key, token) = must(CapabilityKey::generate_worker_bootstrap());
        let encoded = must(WorkerBootstrapEnvelope::new(token).encode());
        let envelope = must(WorkerBootstrapEnvelope::decode(&encoded));
        assert_eq!(format!("{envelope:?}"), "WorkerBootstrapEnvelope(<redacted>)");
        let worker_key = must(envelope.into_token().into_capability_key());
        let nonce = must(generate_worker_nonce_digest());
        let challenge = must(generate_daemon_challenge_digest());
        let fence = WorkerHandshakeFence {
            worker_pid: 42,
            backend_generation: 7,
            worker_nonce_digest: &nonce,
            daemon_challenge_digest: &challenge,
        };
        let tag = must(worker_key.derive_worker_authentication(fence));
        let message = WorkerToDaemon::Authenticate {
            worker_pid: 42,
            backend_generation: 7,
            worker_nonce_digest: nonce,
            daemon_challenge_digest: challenge,
            authentication: tag,
        };
        assert!(format!("{message:?}").contains("<redacted>"));
        let decoded = must(WorkerToDaemon::decode_canonical(&must(
            message.encode_canonical(),
        )));
        match decoded {
            WorkerToDaemon::Authenticate {
                worker_pid,
                backend_generation,
                worker_nonce_digest,
                daemon_challenge_digest,
                authentication,
            } => {
                let fence = WorkerHandshakeFence {
                    worker_pid,
                    backend_generation,
                    worker_nonce_digest: &worker_nonce_digest,
                    daemon_challenge_digest: &daemon_challenge_digest,
                };
                assert!(daemon_key.verify_worker_authentication(fence, &authentication));
            }
            other => panic!("unexpected private message: {other:?}"),
        }
    }

    #[test]
    fn dispatch_and_start_ack_preserve_one_secret_grant_without_public_exposure() {
        let key = must(CapabilityKey::generate());
        let request = request();
        let session = digest('d');
        let fence = RequestGrantFence {
            request: &request,
            backend_generation: 4,
            worker_session_digest: &session,
        };
        let mut ledger = must(RequestGrantLedger::new(2));
        let grant = must(ledger.issue(&key, fence, GrantPurpose::Execute, 100, 50));
        let dispatch = DaemonToWorker::Dispatch {
            request: request.clone(),
            backend_generation: 4,
            worker_session_digest: session.clone(),
            grant,
        };
        assert!(format!("{dispatch:?}").contains("<redacted>"));
        let decoded = must(DaemonToWorker::decode_canonical(&must(
            dispatch.encode_canonical(),
        )));
        let grant = match decoded {
            DaemonToWorker::Dispatch {
                request: decoded_request,
                backend_generation,
                worker_session_digest,
                grant,
            } => {
                assert_eq!(decoded_request, request);
                assert_eq!(backend_generation, 4);
                assert_eq!(worker_session_digest, session);
                grant
            }
            other => panic!("unexpected daemon message: {other:?}"),
        };
        let start = WorkerToDaemon::StartAck {
            request_id: request.identity.request_id.clone(),
            request_generation: request.request_generation,
            backend_generation: 4,
            sequence: 2,
            worker_session_digest: session.clone(),
            grant,
        };
        let encoded = must(start.encode_canonical());
        for replay in 0..2 {
            let decoded = must(WorkerToDaemon::decode_canonical(&encoded));
            let presented = match decoded {
                WorkerToDaemon::StartAck { grant, .. } => grant,
                other => panic!("unexpected worker message: {other:?}"),
            };
            let result = ledger.claim(
                &key,
                fence,
                GrantPurpose::Execute,
                101,
                &presented,
            );
            if replay == 0 {
                let _claim = must(result);
            } else {
                assert_eq!(result, Err(InferError::CapabilityReplay));
            }
        }
    }

    #[test]
    fn direction_magic_trailing_data_and_truncation_fail_closed() {
        let hello = WorkerToDaemon::Hello {
            worker_pid: 42,
            backend_generation: 7,
            worker_nonce_digest: digest('a'),
        };
        let mut encoded = must(hello.encode_canonical());
        assert_eq!(
            DaemonToWorker::decode_canonical(&encoded),
            Err(InferError::ProtocolShape)
        );
        encoded.push(0);
        assert_eq!(
            WorkerToDaemon::decode_canonical(&encoded),
            Err(InferError::ProtocolTrailingData)
        );
        encoded.truncate(6);
        assert!(matches!(
            WorkerToDaemon::decode_canonical(&encoded),
            Err(InferError::ProtocolTruncated)
        ));
    }

    #[test]
    fn state_and_receipt_use_existing_canonical_public_value_codec() {
        let state = StateEvent {
            request_id: must(RequestId::parse("request-state")),
            request_generation: 1,
            backend_generation: 2,
            sequence: 3,
            state: crate::LifecycleState::Running,
        };
        let decoded = must(DaemonToWorker::decode_canonical(&must(
            DaemonToWorker::State(state.clone()).encode_canonical(),
        )));
        match decoded {
            DaemonToWorker::State(value) => assert_eq!(value, state),
            other => panic!("unexpected daemon message: {other:?}"),
        }

        let accepted = AcceptedEvent {
            request_id: must(RequestId::parse("request-unused")),
            request_generation: 1,
            backend_generation: 2,
            sequence: 1,
        };
        assert_eq!(accepted.sequence, 1);
    }
}
'''


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one occurrence, found {count}: {old!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def main() -> None:
    (ROOT / "codex-rs/hepta-infer-core/src/capability.rs").write_text(
        CAPABILITY, encoding="utf-8"
    )
    (ROOT / "codex-rs/hepta-infer-core/src/private_protocol.rs").write_text(
        PRIVATE_PROTOCOL, encoding="utf-8"
    )

    workspace = ROOT / "codex-rs/Cargo.toml"
    replace_once(workspace, 'futures = { version = "0.3", default-features = false }\n', 'futures = { version = "0.3", default-features = false }\ngetrandom = "0.3.4"\n')

    manifest = ROOT / "codex-rs/hepta-infer-core/Cargo.toml"
    replace_once(
        manifest,
        '[lints]\nworkspace = true\n',
        '[dependencies]\nconstant_time_eq = { workspace = true }\ngetrandom = { workspace = true }\nhmac = { workspace = true }\nsha2 = { workspace = true }\nzeroize = { workspace = true }\n\n[lints]\nworkspace = true\n',
    )

    crate_root = ROOT / "codex-rs/hepta-infer-core/src/lib.rs"
    replace_once(crate_root, 'mod protocol;\n', 'mod private_protocol;\nmod protocol;\n')
    replace_once(
        crate_root,
        'pub use capability::CAPABILITY_KEY_BYTES;\n',
        'pub use capability::CAPABILITY_KEY_BYTES;\npub use capability::GRANT_NONCE_BYTES;\npub use capability::GrantClaim;\npub use capability::GrantPurpose;\npub use capability::MAX_GRANT_TTL_MS;\n',
    )
    replace_once(
        crate_root,
        'pub use capability::RequestGrantFence;\n',
        'pub use capability::RequestGrantFence;\npub use capability::RequestGrantLedger;\n',
    )
    replace_once(
        crate_root,
        'pub use capability::WorkerAuthenticationTag;\n',
        'pub use capability::WorkerAuthenticationTag;\npub use capability::WorkerBootstrapToken;\npub use capability::generate_daemon_challenge_digest;\npub use capability::generate_worker_nonce_digest;\n',
    )
    replace_once(
        crate_root,
        'pub use protocol::ClientMessage;\n',
        'pub use private_protocol::DaemonToWorker;\npub use private_protocol::MAX_PRIVATE_FRAME_BYTES;\npub use private_protocol::PRIVATE_PROTOCOL_VERSION;\npub use private_protocol::WorkerBootstrapEnvelope;\npub use private_protocol::WorkerToDaemon;\npub use protocol::ClientMessage;\n',
    )
    replace_once(
        crate_root,
        '    AuthorityEscalation,\n',
        '    AuthorityEscalation,\n    CapabilityExpired,\n    CapabilityLedgerFull,\n    CapabilityPurposeMismatch,\n    CapabilityReplay,\n    CapabilityUnknown,\n',
    )
    replace_once(
        crate_root,
        '    EmptyToken,\n',
        '    EmptyToken,\n    EntropyUnavailable,\n',
    )
    replace_once(
        crate_root,
        '            Self::AuthorityEscalation => "INF_AUTHORITY_ESCALATION",\n',
        '            Self::AuthorityEscalation => "INF_AUTHORITY_ESCALATION",\n            Self::CapabilityExpired => "INF_CAPABILITY_EXPIRED",\n            Self::CapabilityLedgerFull => "INF_CAPABILITY_LEDGER_FULL",\n            Self::CapabilityPurposeMismatch => "INF_CAPABILITY_PURPOSE_MISMATCH",\n            Self::CapabilityReplay => "INF_CAPABILITY_REPLAY",\n            Self::CapabilityUnknown => "INF_CAPABILITY_UNKNOWN",\n',
    )
    replace_once(
        crate_root,
        '            Self::EmptyToken => "INF_EMPTY_TOKEN",\n',
        '            Self::EmptyToken => "INF_EMPTY_TOKEN",\n            Self::EntropyUnavailable => "INF_ENTROPY_UNAVAILABLE",\n',
    )


if __name__ == "__main__":
    main()
