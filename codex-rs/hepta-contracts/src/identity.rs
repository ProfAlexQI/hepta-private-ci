use std::fmt;

/// A monotonically increasing revision in an owning domain.
///
/// The contract does not require revisions from unrelated domains to be
/// comparable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Revision(u64);

impl Revision {
    /// Creates a revision from its domain-local sequence value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the domain-local sequence value.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for Revision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// An opaque digest of canonical content.
///
/// The producing layer owns the digest algorithm and canonicalization rules.
/// Consumers compare the complete string and must not infer an algorithm from
/// its shape.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentHash(String);

impl ContentHash {
    /// Creates a hash wrapper without interpreting or recalculating it.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the opaque digest.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns the opaque digest.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

macro_rules! string_identity {
    ($(#[$metadata:meta])* $name:ident) => {
        $(#[$metadata])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Creates an identity without applying domain-specific validation.
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Returns the opaque identity value.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the wrapper and returns the opaque identity value.
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

string_identity!(
    /// Identity of a human, agent, service, or other accountable principal.
    PrincipalId
);
string_identity!(
    /// Identity of an immutable observation snapshot.
    ObservationId
);
string_identity!(
    /// Identity of a capability definition.
    CapabilityId
);
string_identity!(
    /// Identity of one request to exercise a capability.
    CapabilityRequestId
);
string_identity!(
    /// Identity of a jointly assembled candidate.
    CandidateId
);
string_identity!(
    /// Identity of an admission record.
    AdmissionId
);
string_identity!(
    /// Identity of an authorization record.
    AuthorizationId
);
string_identity!(
    /// Identity of an execution outcome receipt.
    ReceiptId
);
string_identity!(
    /// Identity of a preference whose state can evolve.
    PreferenceId
);
string_identity!(
    /// Identity of immutable evidence supporting a preference transition.
    PreferenceEvidenceId
);
string_identity!(
    /// Identity of an auditable preference transition.
    PreferenceTransitionId
);
