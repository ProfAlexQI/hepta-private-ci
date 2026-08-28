#![forbid(unsafe_code)]

//! AuthBus P0.2 SQLite WAL qualification crate.
//!
//! The crate is intentionally a nested, opt-in workspace. Its default build
//! exposes only the negative-authority posture below. The SQLite coordinator
//! is compiled only with the explicit `sqlite-qualification` feature and is
//! never connected to a listener, product caller, OpenBao client, or runtime
//! authority path.

pub const AUTHBUS_P0_2_QUALIFICATION_ONLY: bool = true;
pub const AUTHBUS_P0_2_AUTHORITY: bool = false;
pub const AUTHBUS_P0_2_EFFECT_AUTHORITY: bool = false;
pub const AUTHBUS_P0_2_PRODUCTION_CALLER: bool = false;
pub const AUTHBUS_P0_2_PRODUCTION_WRITER: bool = false;
pub const AUTHBUS_P0_2_OPERATOR_ACCEPTANCE: bool = false;
pub const AUTHBUS_P0_2_PROMOTION: bool = false;
pub const AUTHBUS_P0_2_G5_ALLOWED: bool = false;
pub const AUTHBUS_P0_2_EXECUTE_ALLOWED: bool = false;

const _: () = {
    assert!(AUTHBUS_P0_2_QUALIFICATION_ONLY);
    assert!(!AUTHBUS_P0_2_AUTHORITY);
    assert!(!AUTHBUS_P0_2_EFFECT_AUTHORITY);
    assert!(!AUTHBUS_P0_2_PRODUCTION_CALLER);
    assert!(!AUTHBUS_P0_2_PRODUCTION_WRITER);
    assert!(!AUTHBUS_P0_2_OPERATOR_ACCEPTANCE);
    assert!(!AUTHBUS_P0_2_PROMOTION);
    assert!(!AUTHBUS_P0_2_G5_ALLOWED);
    assert!(!AUTHBUS_P0_2_EXECUTE_ALLOWED);
};

#[cfg(feature = "sqlite-qualification")]
mod model;
#[cfg(feature = "sqlite-qualification")]
mod store;

#[cfg(feature = "sqlite-qualification")]
pub use model::*;
#[cfg(feature = "sqlite-qualification")]
pub use store::QualificationStore;
