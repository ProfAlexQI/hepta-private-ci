//! Durable, supervisor-owned control state for independent Hepta agents.
//!
//! This crate does not execute turns, forward messages, or own a model queue.

#![forbid(unsafe_code)]

mod error;
mod model;
mod registry;
mod release;
mod runtime_bootstrap_registry;
mod runtime_launch;

pub use error::FleetRegistryError;
pub use model::AGENT_MANIFEST_SCHEMA_VERSION;
pub use model::AGENT_STATE_SCHEMA_VERSION;
pub use model::AgentLifecycle;
pub use model::AgentLifecycleState;
pub use model::AgentManifest;
pub use model::ResourceBudget;
pub use model::WorkspaceBinding;
pub use registry::AgentRecord;
pub use registry::FleetRegistry;
pub use registry::FleetSnapshot;
pub use release::AGENT_RELEASE_STATE_SCHEMA_VERSION;
pub use release::AgentReleaseState;
pub use release::RELEASE_METADATA_SCHEMA_VERSION;
pub use release::RegisteredProgram;
pub use release::RegisteredRelease;
pub use release::ReleaseId;
pub use release::ReleaseMetadata;
pub use release::ReleaseProgramMetadata;
pub use runtime_bootstrap_registry::RUNTIME_BOOTSTRAP_REGISTRY_MAX_BYTES;
pub use runtime_bootstrap_registry::RUNTIME_RELEASE_PROVENANCE_SCHEMA_VERSION;
pub use runtime_bootstrap_registry::ResolvedRuntimeRelease;
pub use runtime_bootstrap_registry::RuntimeReleaseProvenance;
pub use runtime_launch::RUNTIME_LAUNCH_BINDING_SCHEMA_VERSION;
pub use runtime_launch::RuntimeLaunchBinding;
pub use runtime_launch::RuntimeLaunchBindingError;
