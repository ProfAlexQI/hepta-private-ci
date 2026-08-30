#![forbid(unsafe_code)]

//! AuthBus P1.3 canonical quota-registry qualification seam.
//!
//! The default build exposes only immutable negative-authority constants.
//! Registry and adapter qualification is compiled only through the explicit
//! `p1-3-qualification` feature.

/// This tranche is qualification evidence only.
pub const AUTHBUS_P1_3_QUALIFICATION_ONLY: bool = true;
/// This tranche grants no AuthBus authority.
pub const AUTHBUS_P1_3_AUTHORITY: bool = false;
/// This tranche grants no external-effect authority.
pub const AUTHBUS_P1_3_EFFECT_AUTHORITY: bool = false;
/// This tranche is not a production caller.
pub const AUTHBUS_P1_3_PRODUCTION_CALLER: bool = false;
/// This tranche is not a production writer.
pub const AUTHBUS_P1_3_PRODUCTION_WRITER: bool = false;
/// This tranche records no operator acceptance.
pub const AUTHBUS_P1_3_OPERATOR_ACCEPTANCE: bool = false;
/// This tranche performs no promotion.
pub const AUTHBUS_P1_3_PROMOTION: bool = false;
/// This tranche cannot satisfy G5.
pub const AUTHBUS_P1_3_G5_ALLOWED: bool = false;
/// This tranche cannot execute an external effect.
pub const AUTHBUS_P1_3_EXECUTE_ALLOWED: bool = false;

const _: () = {
    assert!(AUTHBUS_P1_3_QUALIFICATION_ONLY);
    assert!(!AUTHBUS_P1_3_AUTHORITY);
    assert!(!AUTHBUS_P1_3_EFFECT_AUTHORITY);
    assert!(!AUTHBUS_P1_3_PRODUCTION_CALLER);
    assert!(!AUTHBUS_P1_3_PRODUCTION_WRITER);
    assert!(!AUTHBUS_P1_3_OPERATOR_ACCEPTANCE);
    assert!(!AUTHBUS_P1_3_PROMOTION);
    assert!(!AUTHBUS_P1_3_G5_ALLOWED);
    assert!(!AUTHBUS_P1_3_EXECUTE_ALLOWED);
};

/// Return the complete negative-authority posture through a runtime API.
///
/// The compile-time block above prevents any source change from silently
/// opening authority. This accessor lets executable qualification verify the
/// same posture without using assertions on compile-time constants, which are
/// rejected by the strict Rust 1.95 Clippy gate.
#[must_use]
pub fn authority_posture_is_closed() -> bool {
    AUTHBUS_P1_3_QUALIFICATION_ONLY
        && !AUTHBUS_P1_3_AUTHORITY
        && !AUTHBUS_P1_3_EFFECT_AUTHORITY
        && !AUTHBUS_P1_3_PRODUCTION_CALLER
        && !AUTHBUS_P1_3_PRODUCTION_WRITER
        && !AUTHBUS_P1_3_OPERATOR_ACCEPTANCE
        && !AUTHBUS_P1_3_PROMOTION
        && !AUTHBUS_P1_3_G5_ALLOWED
        && !AUTHBUS_P1_3_EXECUTE_ALLOWED
}

#[cfg(test)]
mod tests {
    use super::authority_posture_is_closed;

    #[test]
    fn default_build_has_no_authority() {
        assert!(authority_posture_is_closed());
    }
}
