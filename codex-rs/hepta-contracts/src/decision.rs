use crate::AdmissionId;
use crate::AuthorizationId;
use crate::CandidateRef;
use crate::ContentHash;
use crate::ContractError;
use crate::FrozenTurnContext;
use crate::JointCandidate;
use crate::PrincipalId;
use crate::Revision;
use crate::RevisionStamp;

/// Result of evaluating whether a candidate may enter the decision pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AdmissionDecision {
    /// The exact candidate and payload set may proceed.
    Admitted,
    /// The candidate was rejected under the named reason code.
    Rejected {
        /// Stable, machine-readable reason owned by the admitting layer.
        reason_code: String,
    },
}

/// Exact reference to one immutable admission record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionRef {
    id: AdmissionId,
    revision: Revision,
    content_hash: ContentHash,
}

impl AdmissionRef {
    /// Creates an exact admission reference.
    pub fn new(id: AdmissionId, revision: Revision, content_hash: ContentHash) -> Self {
        Self {
            id,
            revision,
            content_hash,
        }
    }

    /// Returns the admission identity.
    pub fn id(&self) -> &AdmissionId {
        &self.id
    }

    /// Returns the admission record revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the complete canonical admission record.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }
}

/// Auditable kernel admission for an exact candidate under a frozen context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admission {
    id: AdmissionId,
    revision: Revision,
    content_hash: ContentHash,
    candidate: CandidateRef,
    payload_set_hash: ContentHash,
    context: FrozenTurnContext,
    policy: RevisionStamp,
    decided_by: PrincipalId,
    decision: AdmissionDecision,
}

impl Admission {
    /// Creates an admission record.
    ///
    /// Candidate, frozen context, payload-set hash, and policy stamp are copied
    /// from the joint candidate so these direct bindings cannot disagree inside
    /// a constructed record.
    pub fn new(
        id: AdmissionId,
        revision: Revision,
        content_hash: ContentHash,
        candidate: &JointCandidate,
        decided_by: PrincipalId,
        decision: AdmissionDecision,
    ) -> Self {
        let candidate_ref = candidate.reference();
        let context = candidate.context().clone();
        let payload_set_hash = candidate.payload_set_hash().clone();
        let policy = context.policy().clone();
        Self {
            id,
            revision,
            content_hash,
            candidate: candidate_ref,
            payload_set_hash,
            context,
            policy,
            decided_by,
            decision,
        }
    }

    /// Returns the admission identity.
    pub fn id(&self) -> &AdmissionId {
        &self.id
    }

    /// Returns the admission record revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the complete canonical admission record.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }

    /// Returns the exact evaluated candidate.
    pub fn candidate(&self) -> &CandidateRef {
        &self.candidate
    }

    /// Returns the exact ordered execution-payload-set digest.
    pub fn payload_set_hash(&self) -> &ContentHash {
        &self.payload_set_hash
    }

    /// Returns the complete context frozen for admission.
    pub fn context(&self) -> &FrozenTurnContext {
        &self.context
    }

    /// Returns the exact policy snapshot evaluated for admission.
    pub fn policy(&self) -> &RevisionStamp {
        &self.policy
    }

    /// Returns the admitting principal.
    pub fn decided_by(&self) -> &PrincipalId {
        &self.decided_by
    }

    /// Returns the admission decision.
    pub fn decision(&self) -> &AdmissionDecision {
        &self.decision
    }

    /// Returns an exact reference to this admission record.
    pub fn reference(&self) -> AdmissionRef {
        AdmissionRef::new(self.id.clone(), self.revision, self.content_hash.clone())
    }
}

/// Result of commit-time reauthorization for an exact candidate and payload.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthorizationDecision {
    /// Execution is authorized under the referenced scope.
    Authorized {
        /// Digest of the canonical execution scope and constraints.
        scope_hash: ContentHash,
    },
    /// Execution is denied under the named reason code.
    Denied {
        /// Stable, machine-readable reason owned by the authorizing layer.
        reason_code: String,
    },
}

/// Exact reference to one immutable commit-time authorization record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationRef {
    id: AuthorizationId,
    revision: Revision,
    content_hash: ContentHash,
}

impl AuthorizationRef {
    /// Creates an exact authorization reference.
    pub fn new(id: AuthorizationId, revision: Revision, content_hash: ContentHash) -> Self {
        Self {
            id,
            revision,
            content_hash,
        }
    }

    /// Returns the authorization identity.
    pub fn id(&self) -> &AuthorizationId {
        &self.id
    }

    /// Returns the authorization record revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the complete canonical authorization record.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }
}

/// Final authorization evaluated immediately before execution commit.
///
/// Candidate and payload bindings are copied directly from the admission, while
/// `current_context` is freshly supplied after any approval wait. A runtime can
/// therefore observe revision drift without relying on an admission identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authorization {
    id: AuthorizationId,
    revision: Revision,
    content_hash: ContentHash,
    admission: AdmissionRef,
    candidate: CandidateRef,
    payload_set_hash: ContentHash,
    current_context: FrozenTurnContext,
    policy: RevisionStamp,
    decided_by: PrincipalId,
    decision: AuthorizationDecision,
}

impl Authorization {
    /// Creates the final commit-time authorization record.
    ///
    /// Commit-time authorization is only valid for an admitted candidate.
    #[allow(clippy::too_many_arguments)]
    pub fn try_new_commit_time(
        id: AuthorizationId,
        revision: Revision,
        content_hash: ContentHash,
        admission: &Admission,
        current_context: FrozenTurnContext,
        decided_by: PrincipalId,
        decision: AuthorizationDecision,
    ) -> Result<Self, ContractError> {
        if !matches!(admission.decision(), AdmissionDecision::Admitted) {
            return Err(ContractError::AdmissionNotAdmitted);
        }

        let policy = current_context.policy().clone();
        Ok(Self {
            id,
            revision,
            content_hash,
            admission: admission.reference(),
            candidate: admission.candidate().clone(),
            payload_set_hash: admission.payload_set_hash().clone(),
            current_context,
            policy,
            decided_by,
            decision,
        })
    }

    /// Returns the authorization identity.
    pub fn id(&self) -> &AuthorizationId {
        &self.id
    }

    /// Returns the authorization record revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the complete canonical authorization record.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }

    /// Returns the exact admission audit reference.
    pub fn admission(&self) -> &AdmissionRef {
        &self.admission
    }

    /// Returns the exact candidate reauthorized at commit time.
    pub fn candidate(&self) -> &CandidateRef {
        &self.candidate
    }

    /// Returns the exact ordered payload set reauthorized at commit time.
    pub fn payload_set_hash(&self) -> &ContentHash {
        &self.payload_set_hash
    }

    /// Returns the freshly frozen context evaluated immediately before commit.
    pub fn current_context(&self) -> &FrozenTurnContext {
        &self.current_context
    }

    /// Returns the exact current policy evaluated at commit time.
    pub fn policy(&self) -> &RevisionStamp {
        &self.policy
    }

    /// Returns the authorizing principal.
    pub fn decided_by(&self) -> &PrincipalId {
        &self.decided_by
    }

    /// Returns the commit-time authorization decision.
    pub fn decision(&self) -> &AuthorizationDecision {
        &self.decision
    }

    /// Returns an exact reference to this authorization record.
    pub fn reference(&self) -> AuthorizationRef {
        AuthorizationRef::new(self.id.clone(), self.revision, self.content_hash.clone())
    }
}
