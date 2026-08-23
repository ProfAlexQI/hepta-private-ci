#![forbid(unsafe_code)]

mod advisory_observation;
mod cognitive;
mod extension;
mod framing;
mod local_lifecycle;
mod observation;

pub use advisory_observation::HEPTA_MEMORY_SHADOW_ADVISORY_OBSERVATION_NAMESPACE;
pub use advisory_observation::HEPTA_MEMORY_SHADOW_ADVISORY_OBSERVATION_SCHEMA_VERSION;
pub use advisory_observation::SHADOW_ADVISORY_OBSERVATION_EXTERNAL_EFFECTS;
pub use advisory_observation::SHADOW_ADVISORY_OBSERVATION_KG_WRITE_AUTHORITY;
pub use advisory_observation::SHADOW_ADVISORY_OBSERVATION_RUNTIME_CONSUMER;
pub use advisory_observation::ShadowAdvisoryHostInput;
pub use advisory_observation::ShadowAdvisoryObservationError;
pub use advisory_observation::ShadowAdvisoryObservationInput;
pub use advisory_observation::ShadowAdvisoryObservationReason;
pub use advisory_observation::ShadowAdvisoryTurnObservation;
pub use advisory_observation::observe_shadow_advisory;
pub use advisory_observation::observe_shadow_advisory_input;
pub use advisory_observation::require_shadow_advisory_observation;
pub use advisory_observation::shadow_advisory_turn_observation;
pub use extension::HeptaMemoryExtension;
pub use extension::HeptaMemoryFeatureFlags;
pub use extension::HeptaMemoryThreadConfig;
pub use extension::install;
pub use observation::HEPTA_MEMORY_SHADOW_OBSERVATION_SCHEMA_VERSION;
pub use observation::ShadowRecallTurnObservation;
pub use observation::ShadowRecallTurnObservationId;
pub use observation::ShadowRecallTurnReason;
pub use observation::shadow_recall_turn_observation;
