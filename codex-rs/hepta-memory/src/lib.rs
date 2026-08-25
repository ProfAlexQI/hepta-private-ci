#![forbid(unsafe_code)]

mod cognitive_compact;
mod cognitive_federation;
mod cognitive_intelligence_writer;
mod cognitive_kg_store;
mod cognitive_memory_store;
mod cognitive_model;
mod cognitive_retrieval;
mod cognitive_runtime;
mod cognitive_store;
mod compact_persistence;
mod framing;
mod h7_runtime;
mod h7_trajectory_store;
mod intuition_shadow;
mod local_atomic_witness;
mod local_compact_executor;
mod local_lease_outbox;
mod local_policy;
mod local_turn_binding;
mod memory_admission;
mod neuron_proposal;
mod recall;
mod shadow_advisory;

pub use cognitive_compact::COGNITIVE_COMPACT_HOOK_NAMESPACE;
pub use cognitive_compact::COGNITIVE_COMPACT_HOOK_SCHEMA_VERSION;
pub use cognitive_compact::CognitiveCompactError;
pub use cognitive_compact::CompactCheckpoint;
pub use cognitive_compact::CompactCommitDecision;
pub use cognitive_compact::CompactConflictReason;
pub use cognitive_compact::CompactFence;
pub use cognitive_compact::CompactLease;
pub use cognitive_compact::CompactLossReport;
pub use cognitive_compact::CompactParentSnapshot;
pub use cognitive_compact::CompactProtectedRef;
pub use cognitive_compact::CompactRejectReason;
pub use cognitive_compact::CompactSummaryReceipt;
pub use cognitive_compact::RehydrationPlan;
pub use cognitive_compact::RehydrationStatus;
pub use cognitive_federation::FederatedMemoryExplanation;
pub use cognitive_federation::FederatedMemoryReader;
pub use cognitive_federation::FederatedMemoryRevalidationBinding;
pub use cognitive_federation::FederatedRecallSet;
pub use cognitive_federation::FederatedRetrievalBatch;
pub use cognitive_federation::FederatedRetrievalCandidate;
pub use cognitive_federation::FederatedRevalidationStatus;
pub use cognitive_federation::FederationCapability;
pub use cognitive_federation::FederationCapabilityId;
pub use cognitive_federation::FederationCapabilityState;
pub use cognitive_federation::FederationCapabilityStatus;
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
pub use cognitive_model::CognitiveProjectionReceipt;
pub use cognitive_model::CognitiveScope;
pub use cognitive_model::CognitiveWriteReceipt;
pub use cognitive_model::KgEdge;
pub use cognitive_model::KgEntityFactDraft;
pub use cognitive_model::KgFactSetDraft;
pub use cognitive_model::KgNode;
pub use cognitive_model::KgRelationFactDraft;
pub use cognitive_model::LedgerSourceKind;
pub use cognitive_model::MemoryDraft;
pub use cognitive_model::MemoryLifecycleState;
pub use cognitive_model::MemoryRevisionDraft;
pub use cognitive_model::MemoryRevisionId;
pub use cognitive_model::MemoryRevisionRecord;
pub use cognitive_model::MemoryVerification;
pub use cognitive_model::ProjectionGeneration;
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
pub use compact_persistence::COMPACT_PERSISTENCE_EXTERNAL_EFFECTS;
pub use compact_persistence::COMPACT_PERSISTENCE_KG_WRITE_AUTHORITY;
pub use compact_persistence::COMPACT_PERSISTENCE_NAMESPACE;
pub use compact_persistence::COMPACT_PERSISTENCE_SCHEMA_VERSION;
pub use compact_persistence::CompactPersistenceAppend;
pub use compact_persistence::CompactPersistenceError;
pub use compact_persistence::CompactPersistenceEvent;
pub use compact_persistence::CompactPersistenceEventKind;
pub use compact_persistence::CompactPersistenceJournal;
pub use compact_persistence::CompactPersistenceSnapshot;
pub use compact_persistence::CompactPersistenceState;
pub use compact_persistence::CompactReconcileOutcome;
pub use compact_persistence::CompactRehydrationRecord;
pub use compact_persistence::checkpoint_digest;
pub use framing::workspace_binding_digest;
pub use h7_runtime::H7_QUALIFICATION_RUNTIME_EXTERNAL_EFFECTS;
pub use h7_runtime::H7_QUALIFICATION_RUNTIME_NAMESPACE;
pub use h7_runtime::H7_QUALIFICATION_RUNTIME_PRODUCTION_AUTHORITY;
pub use h7_runtime::H7_QUALIFICATION_RUNTIME_SCHEMA_VERSION;
pub use h7_runtime::H7Approval;
pub use h7_runtime::H7Artifact;
pub use h7_runtime::H7Evaluation;
pub use h7_runtime::H7QualificationRuntime;
pub use h7_runtime::H7RuntimeError;
pub use h7_runtime::H7RuntimeState;
pub use h7_runtime::H7Trajectory;
pub use h7_runtime::H7TrajectoryEvent;
pub use h7_runtime::H7Transition;
pub use h7_trajectory_store::H7_TRAJECTORY_EXTERNAL_EFFECTS;
pub use h7_trajectory_store::H7_TRAJECTORY_KG_WRITE_AUTHORITY;
pub use h7_trajectory_store::H7_TRAJECTORY_NAMESPACE;
pub use h7_trajectory_store::H7_TRAJECTORY_PRODUCTION_CALLER;
pub use h7_trajectory_store::H7_TRAJECTORY_SCHEMA_VERSION;
pub use h7_trajectory_store::H7TrajectoryAppend;
pub use h7_trajectory_store::H7TrajectoryEventKind;
pub use h7_trajectory_store::H7TrajectoryRead;
pub use h7_trajectory_store::H7TrajectoryRecord;
pub use h7_trajectory_store::H7TrajectoryStoreError;
pub use h7_trajectory_store::append_h7_trajectory_event_bound;
pub use h7_trajectory_store::h7_trajectory_local_receipt_digest;
pub use h7_trajectory_store::read_h7_trajectory_bound;
pub use intuition_shadow::H6_INTUITION_SHADOW_NAMESPACE;
pub use intuition_shadow::H6_INTUITION_SHADOW_SCHEMA_VERSION;
pub use intuition_shadow::IntuitionAbstainReason;
pub use intuition_shadow::IntuitionCandidate;
pub use intuition_shadow::IntuitionDecision;
pub use intuition_shadow::IntuitionMode;
pub use intuition_shadow::IntuitionShadowAuthority;
pub use intuition_shadow::IntuitionShadowError;
pub use intuition_shadow::IntuitionShadowInput;
pub use intuition_shadow::IntuitionShadowPhase;
pub use intuition_shadow::IntuitionShadowReceipt;
pub use intuition_shadow::MAX_INTUITION_CANDIDATE_ID_BYTES;
pub use intuition_shadow::MAX_INTUITION_CANDIDATES;
pub use intuition_shadow::intuition_schema_digest;
pub use intuition_shadow::shadow_intuition_decide;
pub use local_atomic_witness::LOCAL_ATOMIC_WITNESS_EXTERNAL_EFFECTS;
pub use local_atomic_witness::LOCAL_ATOMIC_WITNESS_KG_WRITE_AUTHORITY;
pub use local_atomic_witness::LOCAL_ATOMIC_WITNESS_LEASE_EPOCH_BOUND;
pub use local_atomic_witness::LOCAL_ATOMIC_WITNESS_LEASE_EXPIRY_BOUND;
pub use local_atomic_witness::LOCAL_ATOMIC_WITNESS_LIFECYCLE_REGISTERED;
pub use local_atomic_witness::LOCAL_ATOMIC_WITNESS_NAMESPACE;
pub use local_atomic_witness::LOCAL_ATOMIC_WITNESS_SCHEMA_VERSION;
pub use local_atomic_witness::LocalAtomicWitnessError;
pub use local_atomic_witness::LocalAtomicWitnessFault;
pub use local_atomic_witness::LocalRehydrationWitnessReceipt;
pub use local_atomic_witness::LocalRehydrationWitnessWrite;
pub use local_atomic_witness::write_local_rehydration_witness;
pub use local_atomic_witness::write_local_rehydration_witness_with_fault;
pub use local_compact_executor::LOCAL_COMPACT_EXECUTOR_EXTERNAL_EFFECTS;
pub use local_compact_executor::LOCAL_COMPACT_EXECUTOR_KG_WRITE_AUTHORITY;
pub use local_compact_executor::LOCAL_COMPACT_EXECUTOR_NAMESPACE;
pub use local_compact_executor::LOCAL_COMPACT_EXECUTOR_SCHEMA_VERSION;
pub use local_compact_executor::LocalCompactExecutor;
pub use local_compact_executor::LocalCompactExecutorError;
pub use local_compact_executor::LocalCompactLeaseBinding;
pub use local_compact_executor::LocalRehydrationRead;
pub use local_lease_outbox::LOCAL_LEASE_OUTBOX_EXTERNAL_EFFECTS;
pub use local_lease_outbox::LOCAL_LEASE_OUTBOX_KG_WRITE_AUTHORITY;
pub use local_lease_outbox::LOCAL_LEASE_OUTBOX_NAMESPACE;
pub use local_lease_outbox::LOCAL_LEASE_OUTBOX_PRODUCTION_CALLER;
pub use local_lease_outbox::LOCAL_LEASE_OUTBOX_SCHEMA_VERSION;
pub use local_lease_outbox::LocalAdmission;
pub use local_lease_outbox::LocalAdmissionFault;
pub use local_lease_outbox::LocalLease;
pub use local_lease_outbox::LocalLeaseAcquire;
pub use local_lease_outbox::LocalLeaseBinding;
pub use local_lease_outbox::LocalLeaseOutbox;
pub use local_lease_outbox::LocalLeaseOutboxCounts;
pub use local_lease_outbox::LocalLeaseOutboxError;
pub use local_lease_outbox::LocalLeaseState;
pub use local_lease_outbox::LocalOutcomeReceipt;
pub use local_lease_outbox::LocalOutcomeState;
pub use local_lease_outbox::LocalReconcileOutcome;
pub use local_lease_outbox::LocalReplayFinalization;
pub use local_lease_outbox::QueuedReceipt;
pub use local_policy::LOCAL_DEVELOPMENT_LIFECYCLE_POLICY_NAMESPACE;
pub use local_policy::LOCAL_DEVELOPMENT_LIFECYCLE_POLICY_SCHEMA_VERSION;
pub use local_policy::LocalDevelopmentLifecyclePolicy;
pub use local_policy::LocalDevelopmentLifecyclePolicyError;
pub use local_turn_binding::LOCAL_TURN_LIFECYCLE_BINDING_EXTERNAL_EFFECTS;
pub use local_turn_binding::LOCAL_TURN_LIFECYCLE_BINDING_KG_WRITE_AUTHORITY;
pub use local_turn_binding::LOCAL_TURN_LIFECYCLE_BINDING_LIFECYCLE_REGISTERED;
pub use local_turn_binding::LOCAL_TURN_LIFECYCLE_BINDING_NAMESPACE;
pub use local_turn_binding::LOCAL_TURN_LIFECYCLE_BINDING_PRODUCTION_CALLER;
pub use local_turn_binding::LOCAL_TURN_LIFECYCLE_BINDING_SCHEMA_VERSION;
pub use local_turn_binding::LocalTurnLifecycleBinding;
pub use local_turn_binding::LocalTurnLifecycleBindingError;
pub use memory_admission::MemoryAdmissionEvidence;
pub use memory_admission::MemoryAdmissionReceipt;
pub use memory_admission::MemoryCandidateDraft;
pub use memory_admission::MemoryCandidateOrigin;
pub use memory_admission::MemoryCandidateState;
pub use neuron_proposal::H5_NEURON_PROPOSAL_SCHEMA_VERSION;
pub use neuron_proposal::MAX_NEURON_FEATURE_KEY_BYTES;
pub use neuron_proposal::MAX_NEURON_FEATURES;
pub use neuron_proposal::MAX_NEURON_UPDATE_BPS;
pub use neuron_proposal::NeuronAbstainReason;
pub use neuron_proposal::NeuronFeature;
pub use neuron_proposal::NeuronParameter;
pub use neuron_proposal::NeuronPosition;
pub use neuron_proposal::NeuronProposal;
pub use neuron_proposal::NeuronProposalAuthority;
pub use neuron_proposal::NeuronProposalDecision;
pub use neuron_proposal::NeuronProposalError;
pub use neuron_proposal::NeuronProposalInput;
pub use neuron_proposal::NeuronProposalPhase;
pub use neuron_proposal::shadow_neuron_propose;
pub use recall::RECALL_OBSERVATION_SCHEMA_VERSION;
pub use recall::RecallCandidate;
pub use recall::RecallCounts;
pub use recall::RecallObservation;
pub use recall::RecallObservationId;
pub use recall::RecallObservationReason;
pub use recall::shadow_recall;
pub use shadow_advisory::SHADOW_ADVISORY_NAMESPACE;
pub use shadow_advisory::SHADOW_ADVISORY_SCHEMA_VERSION;
pub use shadow_advisory::ShadowAdvisoryError;
pub use shadow_advisory::ShadowAdvisoryInput;
pub use shadow_advisory::ShadowAdvisoryReceipt;
pub use shadow_advisory::shadow_advisory_evaluate;

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
#[path = "cognitive_intelligence_writer_tests.rs"]
mod cognitive_intelligence_writer_tests;

#[cfg(test)]
#[path = "cognitive_retrieval_tests.rs"]
mod cognitive_retrieval_tests;

#[cfg(test)]
#[path = "cognitive_runtime_tests.rs"]
mod cognitive_runtime_tests;

#[cfg(test)]
#[path = "cognitive_compact_tests.rs"]
mod cognitive_compact_tests;

#[cfg(test)]
#[path = "local_atomic_witness_tests.rs"]
mod local_atomic_witness_tests;

#[cfg(test)]
#[path = "compact_persistence_tests.rs"]
mod compact_persistence_tests;

#[cfg(test)]
#[path = "local_compact_executor_tests.rs"]
mod local_compact_executor_tests;

#[cfg(test)]
#[path = "local_lease_outbox_tests.rs"]
mod local_lease_outbox_tests;

#[cfg(test)]
#[path = "h7_artifact_qualification_tests.rs"]
mod h7_artifact_qualification_tests;

#[cfg(test)]
#[path = "h7_trajectory_store_tests.rs"]
mod h7_trajectory_store_tests;

#[cfg(test)]
mod cognitive_test_support;
