use std::collections::BTreeMap;

use crate::AgentId;
use crate::Digest;
use crate::InferError;
use crate::InferenceRequest;
use crate::RequestId;
use crate::RequestIdentity;
use crate::Result;
use crate::TenantId;
use crate::WorkspaceId;
use crate::hashing::sha256;

const PRINCIPAL_DOMAIN: &[u8] = b"hepta.inference.public-principal.v1\0";
const OPERATOR_PRINCIPAL_DOMAIN: &[u8] = b"hepta.inference.operator-principal.v1\0";

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PeerProcessIdentity {
    uid: u32,
    pid: u32,
    process_start_time: u64,
    executable_digest: Digest,
}

impl PeerProcessIdentity {
    pub fn new(
        uid: u32,
        pid: u32,
        process_start_time: u64,
        executable_digest: Digest,
    ) -> Result<Self> {
        if pid == 0 || process_start_time == 0 {
            return Err(InferError::InvalidPrincipal);
        }
        Ok(Self {
            uid,
            pid,
            process_start_time,
            executable_digest,
        })
    }

    pub const fn uid(&self) -> u32 {
        self.uid
    }

    pub const fn pid(&self) -> u32 {
        self.pid
    }

    pub const fn process_start_time(&self) -> u64 {
        self.process_start_time
    }

    pub const fn executable_digest(&self) -> &Digest {
        &self.executable_digest
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PublicPrincipal {
    process: PeerProcessIdentity,
    tenant_id: TenantId,
    workspace_id: WorkspaceId,
    agent_id: AgentId,
    session_digest: Digest,
}

impl PublicPrincipal {
    pub fn new(
        process: PeerProcessIdentity,
        tenant_id: TenantId,
        workspace_id: WorkspaceId,
        agent_id: AgentId,
        session_digest: Digest,
    ) -> Self {
        Self {
            process,
            tenant_id,
            workspace_id,
            agent_id,
            session_digest,
        }
    }

    pub const fn process(&self) -> &PeerProcessIdentity {
        &self.process
    }

    pub const fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    pub const fn workspace_id(&self) -> &WorkspaceId {
        &self.workspace_id
    }

    pub const fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    pub const fn session_digest(&self) -> &Digest {
        &self.session_digest
    }

    pub fn authorize_admission(&self, request: &InferenceRequest) -> Result<()> {
        self.authorize_identity(&request.identity)
    }

    pub fn authorize_identity(&self, identity: &RequestIdentity) -> Result<()> {
        if identity.tenant_id == self.tenant_id
            && identity.workspace_id == self.workspace_id
            && identity.agent_id == self.agent_id
        {
            Ok(())
        } else {
            Err(InferError::PrincipalBindingMismatch)
        }
    }

    pub fn digest(&self) -> Result<Digest> {
        let mut preimage = Vec::with_capacity(512);
        preimage.extend_from_slice(PRINCIPAL_DOMAIN);
        append_u64(&mut preimage, u64::from(self.process.uid));
        append_u64(&mut preimage, u64::from(self.process.pid));
        append_u64(&mut preimage, self.process.process_start_time);
        append_text(&mut preimage, self.process.executable_digest.as_str())?;
        append_text(&mut preimage, self.tenant_id.as_str())?;
        append_text(&mut preimage, self.workspace_id.as_str())?;
        append_text(&mut preimage, self.agent_id.as_str())?;
        append_text(&mut preimage, self.session_digest.as_str())?;
        digest_from_bytes(sha256(&[preimage.as_slice()])?)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OperatorPrincipal {
    process: PeerProcessIdentity,
    session_digest: Digest,
}

impl OperatorPrincipal {
    pub fn new(process: PeerProcessIdentity, session_digest: Digest) -> Self {
        Self {
            process,
            session_digest,
        }
    }

    pub const fn process(&self) -> &PeerProcessIdentity {
        &self.process
    }

    pub const fn session_digest(&self) -> &Digest {
        &self.session_digest
    }

    pub fn digest(&self) -> Result<Digest> {
        let mut preimage = Vec::with_capacity(320);
        preimage.extend_from_slice(OPERATOR_PRINCIPAL_DOMAIN);
        append_u64(&mut preimage, u64::from(self.process.uid));
        append_u64(&mut preimage, u64::from(self.process.pid));
        append_u64(&mut preimage, self.process.process_start_time);
        append_text(&mut preimage, self.process.executable_digest.as_str())?;
        append_text(&mut preimage, self.session_digest.as_str())?;
        digest_from_bytes(sha256(&[preimage.as_slice()])?)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestOwnership {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub principal_digest: Digest,
    pub tenant_id: TenantId,
    pub workspace_id: WorkspaceId,
    pub agent_id: AgentId,
}

#[derive(Debug)]
pub struct RequestOwnershipLedger {
    max_entries: usize,
    entries: BTreeMap<RequestId, RequestOwnership>,
}

impl RequestOwnershipLedger {
    pub fn new(max_entries: usize) -> Result<Self> {
        if max_entries == 0 {
            return Err(InferError::OwnershipLedgerFull);
        }
        Ok(Self {
            max_entries,
            entries: BTreeMap::new(),
        })
    }

    pub fn register(
        &mut self,
        request: &InferenceRequest,
        principal: &PublicPrincipal,
    ) -> Result<RequestOwnership> {
        principal.authorize_admission(request)?;
        if self.entries.contains_key(&request.identity.request_id) {
            return Err(InferError::DuplicateOwnership);
        }
        if self.entries.len() >= self.max_entries {
            return Err(InferError::OwnershipLedgerFull);
        }
        let ownership = RequestOwnership {
            request_id: request.identity.request_id.clone(),
            request_generation: request.request_generation,
            principal_digest: principal.digest()?,
            tenant_id: request.identity.tenant_id.clone(),
            workspace_id: request.identity.workspace_id.clone(),
            agent_id: request.identity.agent_id.clone(),
        };
        self.entries
            .insert(ownership.request_id.clone(), ownership.clone());
        Ok(ownership)
    }

    pub fn authorize(
        &self,
        request_id: &RequestId,
        request_generation: u64,
        principal: &PublicPrincipal,
    ) -> Result<&RequestOwnership> {
        let ownership = self
            .entries
            .get(request_id)
            .ok_or(InferError::UnknownOwnership)?;
        if ownership.request_generation != request_generation
            || ownership.principal_digest != principal.digest()?
            || &ownership.tenant_id != principal.tenant_id()
            || &ownership.workspace_id != principal.workspace_id()
            || &ownership.agent_id != principal.agent_id()
        {
            return Err(InferError::RequestOwnershipMismatch);
        }
        Ok(ownership)
    }

    pub fn forget(&mut self, request_id: &RequestId) -> Result<RequestOwnership> {
        self.entries
            .remove(request_id)
            .ok_or(InferError::UnknownOwnership)
    }

    pub fn contains(&self, request_id: &RequestId) -> bool {
        self.entries.contains_key(request_id)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

fn append_text(buffer: &mut Vec<u8>, value: &str) -> Result<()> {
    let length = u64::try_from(value.len()).map_err(|_| InferError::InvalidPrincipal)?;
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
    use crate::AuthoritySnapshot;
    use crate::RequestIdentity;
    use crate::ResourceBudgetId;
    use crate::TaskId;

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

    fn principal(agent: &str, pid: u32, session_fill: char) -> PublicPrincipal {
        PublicPrincipal::new(
            must(PeerProcessIdentity::new(1000, pid, 1234, digest('e'))),
            must(TenantId::parse("tenant-a")),
            must(WorkspaceId::parse("workspace-a")),
            must(AgentId::parse(agent)),
            digest(session_fill),
        )
    }

    fn request(agent: &str) -> InferenceRequest {
        InferenceRequest {
            identity: RequestIdentity {
                tenant_id: must(TenantId::parse("tenant-a")),
                workspace_id: must(WorkspaceId::parse("workspace-a")),
                agent_id: must(AgentId::parse(agent)),
                task_id: must(TaskId::parse("task-a")),
                request_id: must(RequestId::parse("request-owner")),
            },
            agent_generation: 1,
            request_generation: 1,
            cancel_generation: 0,
            deadline_unix_ms: 10_000,
            model_tuple_digest: digest('a'),
            policy_digest: digest('b'),
            resource_budget_id: must(ResourceBudgetId::parse("budget-a")),
            prompt_digest: digest('c'),
            prompt_byte_length: 12,
            output_token_limit: 16,
            authority: AuthoritySnapshot::qualification_only_closed(),
        }
    }

    #[test]
    fn same_uid_different_principal_cannot_reuse_ownership() {
        let owner = principal("agent-a", 41, 'f');
        let other = principal("agent-a", 42, 'f');
        let request = request("agent-a");
        let request_id = request.identity.request_id.clone();
        let mut ledger = must(RequestOwnershipLedger::new(4));
        must(ledger.register(&request, &owner));
        must(ledger.authorize(&request_id, 1, &owner));
        assert_eq!(
            ledger.authorize(&request_id, 1, &other),
            Err(InferError::RequestOwnershipMismatch)
        );
    }

    #[test]
    fn admission_identity_must_match_principal_scope() {
        let owner = principal("agent-a", 41, 'f');
        assert_eq!(
            owner.authorize_admission(&request("agent-b")),
            Err(InferError::PrincipalBindingMismatch)
        );
    }

    #[test]
    fn ownership_is_bounded_and_exactly_forgotten() {
        let owner = principal("agent-a", 41, 'f');
        let request = request("agent-a");
        let request_id = request.identity.request_id.clone();
        let mut ledger = must(RequestOwnershipLedger::new(1));
        must(ledger.register(&request, &owner));
        assert_eq!(ledger.len(), 1);
        let forgotten = must(ledger.forget(&request_id));
        assert_eq!(forgotten.request_id, request_id);
        assert!(ledger.is_empty());
        assert_eq!(
            ledger.forget(&forgotten.request_id),
            Err(InferError::UnknownOwnership)
        );
    }
}
