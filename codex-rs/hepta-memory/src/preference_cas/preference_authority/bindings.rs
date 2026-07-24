use hepta_contracts::ContentHash;
use hepta_contracts::PrincipalId;
use hepta_contracts::Revision;

use super::error::PreferenceAuthorityError;
use super::error::require_nonempty;

/// Exact identity of the authority that authenticates preference feedback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceFeedbackSourceRef {
    identity: PrincipalId,
    revision: Revision,
    content_hash: ContentHash,
}

impl PreferenceFeedbackSourceRef {
    /// Creates a non-empty, versioned feedback-source reference.
    pub fn try_new(
        identity: PrincipalId,
        revision: Revision,
        content_hash: ContentHash,
    ) -> Result<Self, PreferenceAuthorityError> {
        let source = Self {
            identity,
            revision,
            content_hash,
        };
        source.validate()?;
        Ok(source)
    }

    /// Returns the accountable source identity.
    pub fn identity(&self) -> &PrincipalId {
        &self.identity
    }

    /// Returns the exact source revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the exact source configuration or implementation digest.
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }

    pub(in crate::preference_authority) fn validate(&self) -> Result<(), PreferenceAuthorityError> {
        require_nonempty("feedback_source.identity", self.identity.as_str())?;
        require_nonempty("feedback_source.content_hash", self.content_hash.as_str())
    }
}

/// Exact identity and semantic version of a preference reducer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceReducerRef {
    identity: String,
    version: String,
}

impl PreferenceReducerRef {
    /// Creates a non-empty reducer reference.
    pub fn try_new(
        identity: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, PreferenceAuthorityError> {
        let reducer = Self {
            identity: identity.into(),
            version: version.into(),
        };
        reducer.validate()?;
        Ok(reducer)
    }

    /// Returns the stable reducer identity.
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Returns the exact reducer semantic version.
    pub fn version(&self) -> &str {
        &self.version
    }

    pub(in crate::preference_authority) fn validate(&self) -> Result<(), PreferenceAuthorityError> {
        require_nonempty("reducer.identity", &self.identity)?;
        require_nonempty("reducer.version", &self.version)
    }
}
