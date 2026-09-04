//! Immutable learning artifact and lineage registry.
//!
//! Registry eligibility is not selection, activation, promotion or release.
//! This crate deliberately exposes no API capable of granting those powers.

#![forbid(unsafe_code)]

mod error;
mod model;
mod registry;

pub use error::ArtifactRegistryError;
pub use model::ArtifactEvent;
pub use model::ArtifactKind;
pub use model::ArtifactManifest;
pub use model::ArtifactRecord;
pub use model::ArtifactRegistrySnapshot;
pub use model::ArtifactState;
pub use model::LineageDisposition;
pub use model::RegistryAppendDisposition;
pub use model::RegistryAppendReceipt;
pub use model::StateChange;
pub use registry::ArtifactRegistry;
