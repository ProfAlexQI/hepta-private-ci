#![forbid(unsafe_code)]

mod cognitive_federation;
mod cognitive_kg_store;
mod cognitive_memory_store;
mod cognitive_model;
mod cognitive_retrieval;
mod cognitive_runtime;
mod cognitive_store;
mod framing;
mod recall;

pub use cognitive_federation::FederatedMemoryExplanation;
pub use cognitive_federation::FederatedMemoryReader;
pub use cognitive_federation::FederatedMemoryRevalidationBinding;
pub use cognitive_federation::FederatedRecallSet;
pub use cognitive_federation::FederatedRetrievalBatch;
pub use cognitive_federation::FederatedRetrievalCandidate;
pub use cognitive_federation::FederatedRevalidationStatus;
pub use cognitive_federation::FederationCapability;
pub use cognitive_federation::FederationCapabilityId;
pub use cognitive_federation::FederationConsumerAccess;
pub use cognitive_federation::FederationGrantRequest;
pub use cognitive_federation::FederationGrantScope;
pub use cognitive_federation::FederationRevalidationDrift;
pub use cognitive_federation::FederationRevocation;
pub use cognitive_federation::MAX_FEDERATION_CAPABILITIES_PER_STORE;
pub use cognitive_federation::MAX_FEDERATION_CAPABILITY_REVISIONS;
pub use cognitive_federation::MAX_FEDERATION_GRANT_LIFETIME_SECONDS;
pub use cognitive_federation::MAX_FEDERATION_SOURCES_PER_AGENT;
pub use cognitive_memory_store::ForgetMemoryDraft;
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
pub use cognitive_retrieval::MAX_RETRIEVAL_CHANNEL_CANDIDATES;
pub use cognitive_retrieval::MAX_RETRIEVAL_QUERY_BYTES;
pub use cognitive_retrieval::MAX_RETRIEVAL_RESULTS;
pub use cognitive_retrieval::MemoryExplanation;
pub use cognitive_retrieval::MemoryRevalidationBinding;
pub use cognitive_retrieval::RetrievalBatch;
pub use cognitive_retrieval::RetrievalCandidate;
pub use cognitive_retrieval::RetrievalChannel;
pub use cognitive_retrieval::RetrievalRequest;
pub use cognitive_retrieval::RevalidationDrift;
pub use cognitive_retrieval::RevalidationStatus;
pub use cognitive_retrieval::SourceCitationRecord;
pub use cognitive_retrieval::SourceRevalidationBinding;
pub use cognitive_runtime::CognitiveRuntime;
pub use cognitive_runtime::CognitiveUnavailableReason;
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
#[path = "cognitive_federation_tests.rs"]
mod cognitive_federation_tests;

#[cfg(test)]
#[path = "cognitive_memory_store_tests.rs"]
mod cognitive_memory_store_tests;

#[cfg(test)]
#[path = "cognitive_kg_store_tests.rs"]
mod cognitive_kg_store_tests;

#[cfg(test)]
#[path = "cognitive_retrieval_tests.rs"]
mod cognitive_retrieval_tests;

#[cfg(test)]
#[path = "cognitive_runtime_tests.rs"]
mod cognitive_runtime_tests;

#[cfg(test)]
mod cognitive_test_support;
