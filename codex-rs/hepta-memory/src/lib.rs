#![forbid(unsafe_code)]

mod cognitive_kg_store;
mod cognitive_memory_store;
mod cognitive_model;
mod cognitive_store;
mod framing;
mod recall;

pub use cognitive_model::CognitiveAccess;
pub use cognitive_model::CognitiveScope;
pub use cognitive_model::KgEdge;
pub use cognitive_model::KgNode;
pub use cognitive_model::LedgerSourceKind;
pub use cognitive_model::MemoryDraft;
pub use cognitive_model::MemoryLifecycleState;
pub use cognitive_model::MemoryRevisionDraft;
pub use cognitive_model::MemoryRevisionId;
pub use cognitive_model::MemoryRevisionRecord;
pub use cognitive_model::MemoryVerification;
pub use cognitive_model::SourceDraft;
pub use cognitive_model::SourceEventId;
pub use cognitive_model::SourceRevisionId;
pub use cognitive_model::StableMemoryId;
pub use cognitive_store::CognitiveStore;
pub use cognitive_store::CognitiveStoreError;
pub use cognitive_store::ProjectionGeneration;
pub use recall::RECALL_OBSERVATION_SCHEMA_VERSION;
pub use recall::RecallCandidate;
pub use recall::RecallCounts;
pub use recall::RecallObservation;
pub use recall::RecallObservationId;
pub use recall::RecallObservationReason;
pub use recall::shadow_recall;

#[cfg(test)]
#[path = "cognitive_store_tests.rs"]
mod cognitive_store_tests;

#[cfg(test)]
#[path = "cognitive_memory_store_tests.rs"]
mod cognitive_memory_store_tests;

#[cfg(test)]
#[path = "cognitive_kg_store_tests.rs"]
mod cognitive_kg_store_tests;

#[cfg(test)]
mod cognitive_test_support;
