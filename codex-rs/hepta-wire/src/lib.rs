//! Deterministic local wire envelopes for Hepta module ports.
//!
//! The format is deliberately small, length-delimited and strict. It does not
//! transport authority: consumers must validate an independently issued grant
//! at the effect boundary.

#![forbid(unsafe_code)]

mod envelope;

pub use envelope::MAX_WIRE_PAYLOAD_BYTES;
pub use envelope::WireEnvelope;
pub use envelope::WireError;
