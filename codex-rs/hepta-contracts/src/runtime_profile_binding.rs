use std::fmt;

use crate::AuthorityAction;
use crate::AuthorityError;
use crate::AuthorityGrant;
use crate::RuntimeAuthorityProfile;
use crate::Sha256Digest;

pub const RUNTIME_PROFILE_BINDING_SCHEMA_VERSION: u32 = 1;

/// Canonical, product-independent identity of one closed runtime profile.
///
/// The service rows are owned here rather than by Agentd so the Supervisor and
/// Agentd can independently reconstruct the same digest without introducing a
/// dependency from the control plane back into the product host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeProfileBinding {
    schema_version: u32,
    profile: RuntimeAuthorityProfile,
    authority_grant_sha256: Sha256Digest,
    profile_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeProfileBindingError {
    Authority(AuthorityError),
    EscapedAuthority(Vec<AuthorityAction>),
}

impl fmt::Display for RuntimeProfileBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authority(error) => write!(formatter, "runtime profile authority failed: {error}"),
            Self::EscapedAuthority(actions) => {
                write!(formatter, "runtime profile contains escaped authority: {actions:?}")
            }
        }
    }
}

impl std::error::Error for RuntimeProfileBindingError {}

impl From<AuthorityError> for RuntimeProfileBindingError {
    fn from(error: AuthorityError) -> Self {
        Self::Authority(error)
    }
}

impl RuntimeProfileBinding {
    pub fn for_authority(
        authority: &AuthorityGrant,
    ) -> Result<Self, RuntimeProfileBindingError> {
        authority.validate_binding(authority.subject_agent_id(), authority.generation())?;
        let escaped = authority.dangerous_actions();
        if !escaped.is_empty() {
            return Err(RuntimeProfileBindingError::EscapedAuthority(escaped));
        }

        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:runtime-profile-binding:v1");
        frame(
            &mut bytes,
            &RUNTIME_PROFILE_BINDING_SCHEMA_VERSION.to_be_bytes(),
        );
        frame(&mut bytes, authority.profile().as_str().as_bytes());
        frame(&mut bytes, authority.digest().as_str().as_bytes());
        for row in service_rows(authority.profile()) {
            frame(&mut bytes, row.as_bytes());
        }

        Ok(Self {
            schema_version: RUNTIME_PROFILE_BINDING_SCHEMA_VERSION,
            profile: authority.profile(),
            authority_grant_sha256: authority.digest(),
            profile_sha256: Sha256Digest::for_bytes(&bytes),
        })
    }

    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub const fn profile(&self) -> RuntimeAuthorityProfile {
        self.profile
    }

    pub const fn profile_name(&self) -> &'static str {
        self.profile.as_str()
    }

    pub fn authority_grant_sha256(&self) -> &Sha256Digest {
        &self.authority_grant_sha256
    }

    pub fn profile_sha256(&self) -> &Sha256Digest {
        &self.profile_sha256
    }

    pub fn service_rows(&self) -> [&'static str; 7] {
        service_rows(self.profile)
    }
}

/// Stable rows included in the profile digest. Each row is
/// `service|placement|requirement|failure_mode|readiness_required`.
pub const fn service_rows(profile: RuntimeAuthorityProfile) -> [&'static str; 7] {
    match profile {
        RuntimeAuthorityProfile::SnapshotReadOnly => [
            "supervisor|control_plane|required|fail_closed|true",
            "agentd|in_process|disabled|not_started|false",
            "app_server|in_process|disabled|not_started|false",
            "memory_runtime|in_process|required|fail_closed|true",
            "automation_runtime|in_process|disabled|not_started|false",
            "matrix_ingress|adapter_process|disabled|not_started|false",
            "provider_effect_adapter|dormant_boundary|disabled|not_started|false",
        ],
        RuntimeAuthorityProfile::AgentLocal => [
            "supervisor|control_plane|required|fail_closed|true",
            "agentd|in_process|required|fail_closed|true",
            "app_server|in_process|required|fail_closed|true",
            "memory_runtime|in_process|optional|degraded|false",
            "automation_runtime|in_process|optional|degraded|false",
            "matrix_ingress|adapter_process|optional|degraded|false",
            "provider_effect_adapter|dormant_boundary|disabled|not_started|false",
        ],
        RuntimeAuthorityProfile::QualificationCognitiveWrite => [
            "supervisor|control_plane|required|fail_closed|true",
            "agentd|in_process|required|fail_closed|true",
            "app_server|in_process|required|fail_closed|true",
            "memory_runtime|in_process|required|fail_closed|true",
            "automation_runtime|in_process|optional|degraded|false",
            "matrix_ingress|adapter_process|disabled|not_started|false",
            "provider_effect_adapter|dormant_boundary|disabled|not_started|false",
        ],
    }
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
#[path = "runtime_profile_binding_tests.rs"]
mod tests;
