//! Checked physical-boundary contracts built on the common verified-use kernel.
//!
//! Submodules define boundary-specific final-payload identities and adapters.
//! The module root intentionally exports no authority issuer and no production
//! caller. Every boundary remains dormant until a separately governed product
//! composition supplies externally verified capabilities and durable stores.

pub mod external;
pub mod governed;

pub const PHYSICAL_BOUNDARIES_RUNTIME_REGISTERED: bool = false;
pub const PHYSICAL_BOUNDARIES_PRODUCTION_CALLER: bool = false;
pub const PHYSICAL_BOUNDARIES_PRODUCTION_WRITER: bool = false;
pub const PHYSICAL_BOUNDARIES_EXTERNAL_EFFECT: bool = false;
pub const PHYSICAL_BOUNDARIES_OPERATOR_ACCEPTANCE: bool = false;
pub const PHYSICAL_BOUNDARIES_PROMOTION: bool = false;
pub const PHYSICAL_BOUNDARIES_RELEASE: bool = false;
