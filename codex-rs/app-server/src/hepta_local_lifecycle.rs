//! Explicit app-server owner for the local-development witness seam.
//!
//! The owner is intentionally not an extension contributor.  It validates a
//! closed-world policy and exposes one host-invoked method; callers must hand
//! it the lease/checkpoint/executor handles they already own.  Constructing an
//! owner therefore does not register callbacks, create a scheduler, or add a
//! production caller.

use codex_hepta_memory::LocalDevelopmentLifecyclePolicy;
use codex_hepta_memory::LocalDevelopmentLifecyclePolicyError;
use codex_hepta_memory_extension::LocalRehydrationWitnessLifecycleError;
use codex_hepta_memory_extension::LocalRehydrationWitnessLifecycleInput;
use codex_hepta_memory_extension::LocalRehydrationWitnessLifecycleResult;

pub const HEPTA_LOCAL_LIFECYCLE_OWNER_RUNTIME_REGISTERED: bool = false;
pub const HEPTA_LOCAL_LIFECYCLE_OWNER_PRODUCTION_CALLER: bool = false;
pub const HEPTA_LOCAL_LIFECYCLE_OWNER_EXTERNAL_EFFECTS: bool = false;
pub const HEPTA_LOCAL_LIFECYCLE_OWNER_KG_WRITE_AUTHORITY: bool = false;

#[derive(Debug)]
pub enum HeptaLocalDevelopmentLifecycleOwnerError {
    Policy(LocalDevelopmentLifecyclePolicyError),
    Lifecycle(LocalRehydrationWitnessLifecycleError),
}

impl std::fmt::Display for HeptaLocalDevelopmentLifecycleOwnerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Policy(error) => write!(formatter, "local lifecycle policy rejected: {error}"),
            Self::Lifecycle(error) => write!(formatter, "local lifecycle write failed: {error}"),
        }
    }
}

impl std::error::Error for HeptaLocalDevelopmentLifecycleOwnerError {}

impl From<LocalDevelopmentLifecyclePolicyError> for HeptaLocalDevelopmentLifecycleOwnerError {
    fn from(error: LocalDevelopmentLifecyclePolicyError) -> Self {
        Self::Policy(error)
    }
}

impl From<LocalRehydrationWitnessLifecycleError> for HeptaLocalDevelopmentLifecycleOwnerError {
    fn from(error: LocalRehydrationWitnessLifecycleError) -> Self {
        Self::Lifecycle(error)
    }
}

/// Host-owned, qualification-only lifecycle owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaLocalDevelopmentLifecycleOwner {
    policy: LocalDevelopmentLifecyclePolicy,
}

impl HeptaLocalDevelopmentLifecycleOwner {
    pub fn new(
        policy: LocalDevelopmentLifecyclePolicy,
    ) -> Result<Self, HeptaLocalDevelopmentLifecycleOwnerError> {
        policy.validate()?;
        Ok(Self { policy })
    }

    pub fn qualification_only() -> Self {
        // The canonical constructor is statically known to satisfy the gate;
        // keep the checked constructor above for untrusted embedding input.
        Self::new(LocalDevelopmentLifecyclePolicy::qualification_only())
            .expect("canonical local-development policy must validate")
    }

    pub const fn policy(&self) -> LocalDevelopmentLifecyclePolicy {
        self.policy
    }

    pub const fn runtime_registered(&self) -> bool {
        HEPTA_LOCAL_LIFECYCLE_OWNER_RUNTIME_REGISTERED
    }

    pub const fn production_caller(&self) -> bool {
        HEPTA_LOCAL_LIFECYCLE_OWNER_PRODUCTION_CALLER
    }

    /// Invoke the extension seam exactly once for this host call.
    ///
    /// The input's policy is replaced with the owner's validated policy so a
    /// host cannot accidentally downgrade the gate between owner creation and
    /// the write.  No callback or background task is installed here.
    pub async fn write_local_rehydration_witness(
        &self,
        mut input: LocalRehydrationWitnessLifecycleInput<'_>,
    ) -> Result<LocalRehydrationWitnessLifecycleResult, HeptaLocalDevelopmentLifecycleOwnerError>
    {
        self.policy.validate()?;
        input.policy = self.policy;
        Ok(
            codex_hepta_memory_extension::write_local_rehydration_witness_at_lifecycle(input)
                .await?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_is_explicit_and_caller_zero() {
        let owner = HeptaLocalDevelopmentLifecycleOwner::qualification_only();
        assert!(owner.policy().permits_explicit_witness_write());
        assert!(!owner.runtime_registered());
        assert!(!owner.production_caller());
        assert!(!HEPTA_LOCAL_LIFECYCLE_OWNER_EXTERNAL_EFFECTS);
        assert!(!HEPTA_LOCAL_LIFECYCLE_OWNER_KG_WRITE_AUTHORITY);
    }

    #[test]
    fn owner_rejects_production_policy() {
        let mut policy = LocalDevelopmentLifecyclePolicy::qualification_only();
        policy.production_activation = true;
        assert!(matches!(
            HeptaLocalDevelopmentLifecycleOwner::new(policy),
            Err(HeptaLocalDevelopmentLifecycleOwnerError::Policy(_))
        ));
    }
}
