#![forbid(unsafe_code)]

//! AuthBus B4 P0.3 qualification seam.
//!
//! The default build exposes only the immutable negative-authority posture.
//! The scheduler model is available solely through the explicit
//! `p0-3-qualification` feature and is not a daemon, listener, provider
//! adapter, product scheduler, or production writer.

pub const AUTHBUS_B4_P0_3_QUALIFICATION_ONLY: bool = true;
pub const AUTHBUS_B4_P0_3_AUTHORITY: bool = false;
pub const AUTHBUS_B4_P0_3_EFFECT_AUTHORITY: bool = false;
pub const AUTHBUS_B4_P0_3_PRODUCTION_CALLER: bool = false;
pub const AUTHBUS_B4_P0_3_PRODUCTION_WRITER: bool = false;
pub const AUTHBUS_B4_P0_3_OPERATOR_ACCEPTANCE: bool = false;
pub const AUTHBUS_B4_P0_3_PROMOTION: bool = false;
pub const AUTHBUS_B4_P0_3_G5_ALLOWED: bool = false;
pub const AUTHBUS_B4_P0_3_EXECUTE_ALLOWED: bool = false;

const _: () = {
    assert!(AUTHBUS_B4_P0_3_QUALIFICATION_ONLY);
    assert!(!AUTHBUS_B4_P0_3_AUTHORITY);
    assert!(!AUTHBUS_B4_P0_3_EFFECT_AUTHORITY);
    assert!(!AUTHBUS_B4_P0_3_PRODUCTION_CALLER);
    assert!(!AUTHBUS_B4_P0_3_PRODUCTION_WRITER);
    assert!(!AUTHBUS_B4_P0_3_OPERATOR_ACCEPTANCE);
    assert!(!AUTHBUS_B4_P0_3_PROMOTION);
    assert!(!AUTHBUS_B4_P0_3_G5_ALLOWED);
    assert!(!AUTHBUS_B4_P0_3_EXECUTE_ALLOWED);
};

#[cfg(feature = "p0-3-qualification")]
mod scheduler;

#[cfg(feature = "p0-3-qualification")]
impl Copy for scheduler::P03ReconcileOutcome {}

#[cfg(feature = "p0-3-qualification")]
pub use scheduler::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_posture_never_grants_authority() {
        const {
            assert!(AUTHBUS_B4_P0_3_QUALIFICATION_ONLY);
            assert!(!AUTHBUS_B4_P0_3_AUTHORITY);
            assert!(!AUTHBUS_B4_P0_3_EFFECT_AUTHORITY);
            assert!(!AUTHBUS_B4_P0_3_PRODUCTION_CALLER);
            assert!(!AUTHBUS_B4_P0_3_PRODUCTION_WRITER);
            assert!(!AUTHBUS_B4_P0_3_OPERATOR_ACCEPTANCE);
            assert!(!AUTHBUS_B4_P0_3_PROMOTION);
            assert!(!AUTHBUS_B4_P0_3_G5_ALLOWED);
            assert!(!AUTHBUS_B4_P0_3_EXECUTE_ALLOWED);
        }
    }
}
