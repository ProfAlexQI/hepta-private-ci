use crate::ContentHash;
use crate::ObservationId;
use crate::PrincipalId;
use crate::Revision;

/// One ordered fact in an observation snapshot.
///
/// Keys and values are deliberately opaque. The observing layer owns their
/// vocabulary and canonical representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationFact {
    key: String,
    value: String,
}

impl ObservationFact {
    /// Creates an opaque observation fact.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Returns the fact key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the canonical fact value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Stable reference to the exact observation revision consumed by a layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationRef {
    id: ObservationId,
    revision: Revision,
    content_hash: ContentHash,
}

impl ObservationRef {
    /// Creates a reference to an exact observation revision.
    pub fn new(id: ObservationId, revision: Revision, content_hash: ContentHash) -> Self {
        Self {
            id,
            revision,
            content_hash,
        }
    }

    /// Returns the observation identity.
    pub fn id(&self) -> &ObservationId {
        &self.id
    }

    /// Returns the referenced observation revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest that freezes the referenced content.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }
}

/// Frozen input observed before candidates are proposed or evaluated.
///
/// Fact order is part of the content contract. The producer computes
/// `content_hash` from its canonical representation before publishing the
/// snapshot; this crate intentionally performs no hashing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationSnapshot {
    id: ObservationId,
    revision: Revision,
    content_hash: ContentHash,
    observed_by: PrincipalId,
    facts: Vec<ObservationFact>,
}

impl ObservationSnapshot {
    /// Creates a frozen observation snapshot.
    pub fn new(
        id: ObservationId,
        revision: Revision,
        content_hash: ContentHash,
        observed_by: PrincipalId,
        facts: Vec<ObservationFact>,
    ) -> Self {
        Self {
            id,
            revision,
            content_hash,
            observed_by,
            facts,
        }
    }

    /// Returns the observation identity.
    pub fn id(&self) -> &ObservationId {
        &self.id
    }

    /// Returns the snapshot revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the digest of the canonical snapshot.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }

    /// Returns the principal that produced the snapshot.
    pub fn observed_by(&self) -> &PrincipalId {
        &self.observed_by
    }

    /// Returns the ordered frozen facts.
    pub fn facts(&self) -> &[ObservationFact] {
        &self.facts
    }

    /// Returns a stable reference to this exact snapshot revision.
    pub fn reference(&self) -> ObservationRef {
        ObservationRef::new(self.id.clone(), self.revision, self.content_hash.clone())
    }
}
