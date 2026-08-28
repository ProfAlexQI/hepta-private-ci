#![forbid(unsafe_code)]

//! AuthBus P1.2 durable identity/evidence replay-ledger qualification.
//!
//! The default build exposes only immutable negative-authority constants.
//! SQLite WAL persistence is compiled only with `p1-2-qualification`. The
//! feature stores already-verified public key metadata, canonical bindings and
//! SHA-256 evidence digests; raw signatures, private keys, credentials and
//! secret values are intentionally not representable in the durable schema.

pub const AUTHBUS_P1_2_QUALIFICATION_ONLY: bool = true;
pub const AUTHBUS_P1_2_AUTHORITY: bool = false;
pub const AUTHBUS_P1_2_EFFECT_AUTHORITY: bool = false;
pub const AUTHBUS_P1_2_PRODUCTION_CALLER: bool = false;
pub const AUTHBUS_P1_2_PRODUCTION_WRITER: bool = false;
pub const AUTHBUS_P1_2_OPERATOR_ACCEPTANCE: bool = false;
pub const AUTHBUS_P1_2_PROMOTION: bool = false;
pub const AUTHBUS_P1_2_G5_ALLOWED: bool = false;
pub const AUTHBUS_P1_2_EXECUTE_ALLOWED: bool = false;
pub const AUTHBUS_P1_2_LISTENER_ENABLED: bool = false;
pub const AUTHBUS_P1_2_PROVIDER_CALL_ENABLED: bool = false;
pub const AUTHBUS_P1_2_OPENBAO_ENABLED: bool = false;
pub const AUTHBUS_P1_2_PRIVATE_KEY_STORAGE: bool = false;
pub const AUTHBUS_P1_2_RAW_SIGNATURE_STORAGE: bool = false;
pub const AUTHBUS_P1_2_SECRET_STORAGE: bool = false;
pub const AUTHBUS_P1_2_PARENT_WORKSPACE_WIRED: bool = false;

const _: () = {
    assert!(AUTHBUS_P1_2_QUALIFICATION_ONLY);
    assert!(!AUTHBUS_P1_2_AUTHORITY);
    assert!(!AUTHBUS_P1_2_EFFECT_AUTHORITY);
    assert!(!AUTHBUS_P1_2_PRODUCTION_CALLER);
    assert!(!AUTHBUS_P1_2_PRODUCTION_WRITER);
    assert!(!AUTHBUS_P1_2_OPERATOR_ACCEPTANCE);
    assert!(!AUTHBUS_P1_2_PROMOTION);
    assert!(!AUTHBUS_P1_2_G5_ALLOWED);
    assert!(!AUTHBUS_P1_2_EXECUTE_ALLOWED);
    assert!(!AUTHBUS_P1_2_LISTENER_ENABLED);
    assert!(!AUTHBUS_P1_2_PROVIDER_CALL_ENABLED);
    assert!(!AUTHBUS_P1_2_OPENBAO_ENABLED);
    assert!(!AUTHBUS_P1_2_PRIVATE_KEY_STORAGE);
    assert!(!AUTHBUS_P1_2_RAW_SIGNATURE_STORAGE);
    assert!(!AUTHBUS_P1_2_SECRET_STORAGE);
    assert!(!AUTHBUS_P1_2_PARENT_WORKSPACE_WIRED);
};

#[cfg(feature = "p1-2-qualification")]
mod model;
#[cfg(feature = "p1-2-qualification")]
mod store;

#[cfg(feature = "p1-2-qualification")]
pub use model::*;
#[cfg(feature = "p1-2-qualification")]
pub use store::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_2_default_build_has_no_authority() {
        const {
            assert!(AUTHBUS_P1_2_QUALIFICATION_ONLY);
            assert!(!AUTHBUS_P1_2_AUTHORITY);
            assert!(!AUTHBUS_P1_2_EFFECT_AUTHORITY);
            assert!(!AUTHBUS_P1_2_PRODUCTION_CALLER);
            assert!(!AUTHBUS_P1_2_PRODUCTION_WRITER);
            assert!(!AUTHBUS_P1_2_OPERATOR_ACCEPTANCE);
            assert!(!AUTHBUS_P1_2_PROMOTION);
            assert!(!AUTHBUS_P1_2_G5_ALLOWED);
            assert!(!AUTHBUS_P1_2_EXECUTE_ALLOWED);
            assert!(!AUTHBUS_P1_2_LISTENER_ENABLED);
            assert!(!AUTHBUS_P1_2_PROVIDER_CALL_ENABLED);
            assert!(!AUTHBUS_P1_2_OPENBAO_ENABLED);
            assert!(!AUTHBUS_P1_2_PRIVATE_KEY_STORAGE);
            assert!(!AUTHBUS_P1_2_RAW_SIGNATURE_STORAGE);
            assert!(!AUTHBUS_P1_2_SECRET_STORAGE);
            assert!(!AUTHBUS_P1_2_PARENT_WORKSPACE_WIRED);
        }
    }
}
