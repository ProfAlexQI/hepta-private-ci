use super::context_freezer::add_ttl;
use super::context_freezer::now_ms;
use crate::ApprovalSnapshot;
use crate::HeptaError;
use crate::PendingApproval;
use crate::SessionApprovalState;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::ContentHash;
use hepta_contracts::JointCandidate;
use hepta_kernel::HeptaKernelSafetyAdmission;
use std::time::Duration;

pub(crate) const EXACT_PENDING_TTL: Duration = Duration::from_secs(10 * 60);
const EXACT_GRANT_TTL: Duration = Duration::from_secs(5 * 60);
const PROACTIVE_TOKEN_TTL: Duration = Duration::from_secs(2 * 60);

#[derive(Debug, Clone)]
pub(crate) struct ExactApprovalMaterial {
    pub(crate) tool_name: String,
    pub(crate) reason: String,
    pub(crate) canonical_arguments: String,
    pub(crate) payload_hash: ContentHash,
    pub(crate) capability_descriptor: CapabilityDescriptor,
    pub(crate) candidate: JointCandidate,
    pub(crate) admission: HeptaKernelSafetyAdmission,
    pub(crate) expires_at_unix_ms: u64,
}

impl ExactApprovalMaterial {
    pub(crate) fn binding_hash(&self) -> &ContentHash {
        self.admission.binding().candidate_hash()
    }

    fn same_binding(&self, other: &Self) -> bool {
        self.tool_name == other.tool_name
            && self.payload_hash == other.payload_hash
            && self.capability_descriptor == other.capability_descriptor
            && self.binding_hash() == other.binding_hash()
    }

    fn is_live(&self, now_ms: u64) -> bool {
        self.expires_at_unix_ms > now_ms
    }
}

#[derive(Debug, Clone)]
struct ProactiveApprovalToken {
    tool_name: String,
    expires_at_unix_ms: u64,
    remaining_uses: u8,
}

#[derive(Debug, Clone, Default)]
struct ExactSessionApprovalState {
    session_id: String,
    pending: Vec<ExactApprovalMaterial>,
    grants: Vec<ExactApprovalMaterial>,
    proactive: Vec<ProactiveApprovalToken>,
}

#[derive(Debug, Clone)]
pub(crate) enum CandidateApproval {
    Exact(Box<ExactApprovalMaterial>),
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ApprovalGrant {
    Exact { binding_hash: String },
    Proactive,
}

#[derive(Debug, Default)]
pub(crate) struct ApprovalState {
    legacy_sessions: Vec<SessionApprovalState>,
    exact_sessions: Vec<ExactSessionApprovalState>,
}

impl ApprovalState {
    fn legacy_session(&self, session_id: &str) -> Option<&SessionApprovalState> {
        self.legacy_sessions
            .iter()
            .find(|session| session.session_id == session_id)
    }

    fn legacy_session_mut(&mut self, session_id: &str) -> &mut SessionApprovalState {
        if let Some(index) = self
            .legacy_sessions
            .iter()
            .position(|session| session.session_id == session_id)
        {
            return &mut self.legacy_sessions[index];
        }
        let index = self.legacy_sessions.len();
        self.legacy_sessions.push(SessionApprovalState {
            session_id: session_id.to_string(),
            granted_tools: Vec::new(),
            pending: Vec::new(),
        });
        &mut self.legacy_sessions[index]
    }

    fn exact_session_mut(&mut self, session_id: &str) -> &mut ExactSessionApprovalState {
        if let Some(index) = self
            .exact_sessions
            .iter()
            .position(|session| session.session_id == session_id)
        {
            return &mut self.exact_sessions[index];
        }
        let index = self.exact_sessions.len();
        self.exact_sessions.push(ExactSessionApprovalState {
            session_id: session_id.to_string(),
            ..ExactSessionApprovalState::default()
        });
        &mut self.exact_sessions[index]
    }

    pub(crate) fn purge_expired(&mut self, now_ms: u64) {
        for session in &mut self.exact_sessions {
            session.pending.retain(|item| item.is_live(now_ms));
            session.grants.retain(|item| item.is_live(now_ms));
            session
                .proactive
                .retain(|item| item.expires_at_unix_ms > now_ms && item.remaining_uses > 0);
        }
        self.exact_sessions.retain(|session| {
            !session.pending.is_empty()
                || !session.grants.is_empty()
                || !session.proactive.is_empty()
        });
    }

    pub(crate) fn snapshot_for(&self, session_id: &str) -> ApprovalSnapshot {
        self.legacy_session(session_id)
            .map(|session| ApprovalSnapshot {
                granted_tools: session.granted_tools.clone(),
                pending: session.pending.clone(),
            })
            .unwrap_or_default()
    }

    pub(crate) fn remember_pending_exact(
        &mut self,
        session_id: &str,
        material: ExactApprovalMaterial,
    ) {
        self.purge_expired(now_ms());
        let tool_name = material.tool_name.clone();
        let reason = material.reason.clone();
        let candidate_binding_hash = material.binding_hash().as_str().to_string();
        let payload_hash = material.payload_hash.as_str().to_string();
        let exact = self.exact_session_mut(session_id);
        if !exact
            .pending
            .iter()
            .any(|pending| pending.same_binding(&material))
        {
            exact.pending.push(material);
        }
        let legacy = self.legacy_session_mut(session_id);
        legacy.granted_tools.retain(|tool| tool != &tool_name);
        if let Some(pending) = legacy.pending.iter_mut().find(|pending| {
            pending.candidate_binding_hash.as_deref() == Some(&candidate_binding_hash)
        }) {
            pending.reason = reason;
        } else {
            legacy.pending.push(PendingApproval {
                tool_name,
                reason,
                candidate_binding_hash: Some(candidate_binding_hash),
                payload_hash: Some(payload_hash),
            });
        }
    }

    pub(crate) fn approve_tool(
        &mut self,
        session_id: &str,
        tool_name: &str,
        proactive_allowed: bool,
    ) -> Result<ApprovalGrant, HeptaError> {
        let now = now_ms();
        self.purge_expired(now);
        let pending_indices = self
            .exact_sessions
            .iter()
            .find(|session| session.session_id == session_id)
            .map(|session| {
                session
                    .pending
                    .iter()
                    .enumerate()
                    .filter_map(|(index, item)| (item.tool_name == tool_name).then_some(index))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if pending_indices.len() > 1 {
            return Err(HeptaError(format!(
                "multiple exact pending candidates for {tool_name}; approval is ambiguous"
            )));
        }
        let expired_exact_display = pending_indices.is_empty()
            && self
                .legacy_session(session_id)
                .map(|session| {
                    session.pending.iter().any(|pending| {
                        pending.tool_name == tool_name && pending.candidate_binding_hash.is_some()
                    })
                })
                .unwrap_or(false);
        if expired_exact_display {
            self.legacy_session_mut(session_id)
                .pending
                .retain(|pending| {
                    pending.tool_name != tool_name || pending.candidate_binding_hash.is_none()
                });
            return Err(HeptaError(format!(
                "exact pending candidate for {tool_name} expired; rerun the tool request before approving"
            )));
        }
        let grant = if let Some(index) = pending_indices.first().copied() {
            let exact = self.exact_session_mut(session_id);
            let mut material = exact.pending.remove(index);
            material.expires_at_unix_ms = add_ttl(now, EXACT_GRANT_TTL);
            exact.grants.retain(|grant| !grant.same_binding(&material));
            let binding_hash = material.binding_hash().as_str().to_string();
            exact.grants.push(material);
            ApprovalGrant::Exact { binding_hash }
        } else {
            if !proactive_allowed {
                return Err(HeptaError(format!(
                    "proactive approval is not allowed for effectful tool {tool_name}; run it once to create an exact pending candidate"
                )));
            }
            let exact = self.exact_session_mut(session_id);
            exact.proactive.retain(|token| token.tool_name != tool_name);
            exact.proactive.push(ProactiveApprovalToken {
                tool_name: tool_name.to_string(),
                expires_at_unix_ms: add_ttl(now, PROACTIVE_TOKEN_TTL),
                remaining_uses: 1,
            });
            ApprovalGrant::Proactive
        };
        let legacy = self.legacy_session_mut(session_id);
        legacy.pending.retain(|item| item.tool_name != tool_name);
        if !legacy.granted_tools.iter().any(|tool| tool == tool_name) {
            legacy.granted_tools.push(tool_name.to_string());
        }
        Ok(grant)
    }

    pub(crate) fn approve_candidate(
        &mut self,
        session_id: &str,
        binding_hash: &str,
    ) -> Result<(String, String), HeptaError> {
        let now = now_ms();
        self.purge_expired(now);
        let matches = self
            .exact_sessions
            .iter()
            .find(|session| session.session_id == session_id)
            .map(|session| {
                session
                    .pending
                    .iter()
                    .enumerate()
                    .filter_map(|(index, pending)| {
                        (pending.binding_hash().as_str() == binding_hash).then_some(index)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if matches.len() != 1 {
            self.legacy_session_mut(session_id)
                .pending
                .retain(|pending| pending.candidate_binding_hash.as_deref() != Some(binding_hash));
            return Err(HeptaError(format!(
                "exact pending candidate binding not found or ambiguous: {}",
                short_hash(binding_hash)
            )));
        }
        let exact = self.exact_session_mut(session_id);
        let mut material = exact.pending.remove(matches[0]);
        material.expires_at_unix_ms = add_ttl(now, EXACT_GRANT_TTL);
        exact.grants.retain(|grant| !grant.same_binding(&material));
        let tool_name = material.tool_name.clone();
        let approved_hash = material.binding_hash().as_str().to_string();
        exact.grants.push(material);

        let legacy = self.legacy_session_mut(session_id);
        legacy
            .pending
            .retain(|item| item.candidate_binding_hash.as_deref() != Some(approved_hash.as_str()));
        if !legacy.granted_tools.iter().any(|tool| tool == &tool_name) {
            legacy.granted_tools.push(tool_name.clone());
        }
        Ok((tool_name, approved_hash))
    }

    pub(crate) fn candidate_approval(
        &mut self,
        session_id: &str,
        material: &ExactApprovalMaterial,
    ) -> CandidateApproval {
        let now = now_ms();
        self.purge_expired(now);
        let exact = self.exact_session_mut(session_id);
        if let Some(grant) = exact
            .grants
            .iter()
            .find(|grant| grant.same_binding(material))
            .cloned()
        {
            return CandidateApproval::Exact(Box::new(grant));
        }
        if let Some(index) = exact.proactive.iter().position(|token| {
            token.tool_name == material.tool_name
                && token.remaining_uses > 0
                && token.expires_at_unix_ms > now
        }) {
            exact.proactive.remove(index);
            let mut bound = material.clone();
            bound.expires_at_unix_ms = add_ttl(now, EXACT_GRANT_TTL);
            exact.grants.push(bound.clone());
            return CandidateApproval::Exact(Box::new(bound));
        }
        CandidateApproval::Missing
    }

    pub(crate) fn grant_index(
        &self,
        session_id: &str,
        expected: &ExactApprovalMaterial,
        now_ms: u64,
    ) -> Option<(usize, usize)> {
        self.exact_sessions
            .iter()
            .enumerate()
            .find(|(_, session)| session.session_id == session_id)
            .and_then(|(session_index, session)| {
                session
                    .grants
                    .iter()
                    .position(|grant| grant.same_binding(expected) && grant.is_live(now_ms))
                    .map(|grant_index| (session_index, grant_index))
            })
    }

    pub(crate) fn remove_grant(&mut self, session_index: usize, grant_index: usize) {
        self.exact_sessions[session_index]
            .grants
            .remove(grant_index);
    }

    pub(crate) fn all_sessions(&self) -> Vec<SessionApprovalState> {
        self.legacy_sessions.clone()
    }

    pub(crate) fn replace_legacy_sessions(&mut self, sessions: Vec<SessionApprovalState>) {
        self.legacy_sessions = sessions;
        self.exact_sessions.clear();
    }

    pub(crate) fn set_legacy_snapshot(&mut self, session_id: &str, snapshot: ApprovalSnapshot) {
        self.remove_session(session_id);
        if !snapshot.granted_tools.is_empty() || !snapshot.pending.is_empty() {
            self.legacy_sessions.push(SessionApprovalState {
                session_id: session_id.to_string(),
                granted_tools: snapshot.granted_tools,
                pending: snapshot.pending,
            });
        }
    }

    pub(crate) fn remove_session(&mut self, session_id: &str) {
        self.legacy_sessions
            .retain(|session| session.session_id != session_id);
        self.exact_sessions
            .retain(|session| session.session_id != session_id);
    }
}

pub(crate) fn short_hash(value: &str) -> &str {
    value
        .strip_prefix("sha256:")
        .unwrap_or(value)
        .get(..12)
        .unwrap_or(value)
}
