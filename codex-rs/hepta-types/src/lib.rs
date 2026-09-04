//! Stable, authority-free primitives shared by Hepta modules.
//!
//! These values carry identity, bounds, generations and deterministic numeric
//! representations. They deliberately contain no runtime handle, credential,
//! ambient authority or product writer.

#![forbid(unsafe_code)]

mod bounded;
mod digest;
mod fixed;
mod identity;

pub use bounded::BoundedBytes;
pub use bounded::BoundedText;
pub use bounded::BoundedValueError;
pub use digest::Digest32;
pub use digest::DigestParseError;
pub use fixed::FixedQ32;
pub use fixed::FixedQ32Error;
pub use fixed::ProbabilityQ32;
pub use identity::AuthorityPosture;
pub use identity::Generation;
pub use identity::LogicalSequence;
pub use identity::Revision;
pub use identity::StableId;
