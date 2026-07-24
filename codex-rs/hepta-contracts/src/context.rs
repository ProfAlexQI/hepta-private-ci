use crate::ContentHash;
use crate::ObservationRef;
use crate::Revision;

/// Exact revision and canonical content digest for one frozen domain snapshot.
///
/// A revision alone is not sufficient at a safety boundary because a producer
/// could accidentally reuse or repair a revision in place. The content hash
/// makes that drift observable without requiring this crate to own hashing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevisionStamp {
    revision: Revision,
    content_hash: ContentHash,
}

impl RevisionStamp {
    /// Creates an exact stamp for one domain snapshot.
    pub fn new(revision: Revision, content_hash: ContentHash) -> Self {
        Self {
            revision,
            content_hash,
        }
    }

    /// Returns the domain-local revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the canonical snapshot at this revision.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }
}

/// Frozen inputs against which a turn is planned or safety-checked.
///
/// Every decision-stage record carries this complete value. This makes drift
/// across an approval wait visible and prevents a policy or catalog revision
/// from being inferred indirectly from an observation or candidate identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenTurnContext {
    observation: ObservationRef,
    state: RevisionStamp,
    policy: RevisionStamp,
    capability_catalog: RevisionStamp,
    preference: RevisionStamp,
}

impl FrozenTurnContext {
    /// Creates a complete frozen context for one turn decision.
    pub fn new(
        observation: ObservationRef,
        state: RevisionStamp,
        policy: RevisionStamp,
        capability_catalog: RevisionStamp,
        preference: RevisionStamp,
    ) -> Self {
        Self {
            observation,
            state,
            policy,
            capability_catalog,
            preference,
        }
    }

    /// Returns the exact observation snapshot.
    pub fn observation(&self) -> &ObservationRef {
        &self.observation
    }

    /// Returns the exact runtime or application-state snapshot.
    pub fn state(&self) -> &RevisionStamp {
        &self.state
    }

    /// Returns the exact safety-policy snapshot.
    pub fn policy(&self) -> &RevisionStamp {
        &self.policy
    }

    /// Returns the exact capability-catalog snapshot.
    pub fn capability_catalog(&self) -> &RevisionStamp {
        &self.capability_catalog
    }

    /// Returns the exact preference snapshot used by intelligence.
    pub fn preference(&self) -> &RevisionStamp {
        &self.preference
    }
}
