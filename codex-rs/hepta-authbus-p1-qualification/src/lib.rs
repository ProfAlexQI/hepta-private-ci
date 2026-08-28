#![forbid(unsafe_code)]

//! AuthBus P1.1 signed identity and evidence qualification.
//!
//! The default build exposes only the negative-authority posture. Signature
//! verification and replay ledgers are compiled only with the explicit
//! `p1-qualification` feature. Nothing in this crate opens a listener, calls a
//! provider, loads a private key, or writes production state.

pub const AUTHBUS_P1_1_QUALIFICATION_ONLY: bool = true;
pub const AUTHBUS_P1_1_AUTHORITY: bool = false;
pub const AUTHBUS_P1_1_EFFECT_AUTHORITY: bool = false;
pub const AUTHBUS_P1_1_PRODUCTION_CALLER: bool = false;
pub const AUTHBUS_P1_1_PRODUCTION_WRITER: bool = false;
pub const AUTHBUS_P1_1_OPERATOR_ACCEPTANCE: bool = false;
pub const AUTHBUS_P1_1_PROMOTION: bool = false;
pub const AUTHBUS_P1_1_G5_ALLOWED: bool = false;
pub const AUTHBUS_P1_1_EXECUTE_ALLOWED: bool = false;
pub const AUTHBUS_P1_1_LISTENER_ENABLED: bool = false;
pub const AUTHBUS_P1_1_PROVIDER_CALL_ENABLED: bool = false;
pub const AUTHBUS_P1_1_OPENBAO_ENABLED: bool = false;
pub const AUTHBUS_P1_1_PRIVATE_KEY_STORAGE: bool = false;

const _: () = {
    assert!(AUTHBUS_P1_1_QUALIFICATION_ONLY);
    assert!(!AUTHBUS_P1_1_AUTHORITY);
    assert!(!AUTHBUS_P1_1_EFFECT_AUTHORITY);
    assert!(!AUTHBUS_P1_1_PRODUCTION_CALLER);
    assert!(!AUTHBUS_P1_1_PRODUCTION_WRITER);
    assert!(!AUTHBUS_P1_1_OPERATOR_ACCEPTANCE);
    assert!(!AUTHBUS_P1_1_PROMOTION);
    assert!(!AUTHBUS_P1_1_G5_ALLOWED);
    assert!(!AUTHBUS_P1_1_EXECUTE_ALLOWED);
    assert!(!AUTHBUS_P1_1_LISTENER_ENABLED);
    assert!(!AUTHBUS_P1_1_PROVIDER_CALL_ENABLED);
    assert!(!AUTHBUS_P1_1_OPENBAO_ENABLED);
    assert!(!AUTHBUS_P1_1_PRIVATE_KEY_STORAGE);
};

#[cfg(feature = "p1-qualification")]
mod model;
#[cfg(feature = "p1-qualification")]
mod verifier;

#[cfg(feature = "p1-qualification")]
pub use model::*;
#[cfg(feature = "p1-qualification")]
pub use verifier::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_1_default_build_has_no_authority() {
        const {
            assert!(AUTHBUS_P1_1_QUALIFICATION_ONLY);
            assert!(!AUTHBUS_P1_1_AUTHORITY);
            assert!(!AUTHBUS_P1_1_EFFECT_AUTHORITY);
            assert!(!AUTHBUS_P1_1_PRODUCTION_CALLER);
            assert!(!AUTHBUS_P1_1_PRODUCTION_WRITER);
            assert!(!AUTHBUS_P1_1_OPERATOR_ACCEPTANCE);
            assert!(!AUTHBUS_P1_1_PROMOTION);
            assert!(!AUTHBUS_P1_1_G5_ALLOWED);
            assert!(!AUTHBUS_P1_1_EXECUTE_ALLOWED);
            assert!(!AUTHBUS_P1_1_LISTENER_ENABLED);
            assert!(!AUTHBUS_P1_1_PROVIDER_CALL_ENABLED);
            assert!(!AUTHBUS_P1_1_OPENBAO_ENABLED);
            assert!(!AUTHBUS_P1_1_PRIVATE_KEY_STORAGE);
        }
    }
}
