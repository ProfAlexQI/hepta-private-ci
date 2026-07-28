#![deny(missing_docs)]
#![forbid(unsafe_code)]
//! Stable, dependency-free contracts shared by Hepta architecture layers.
//!
//! This crate owns data shapes, not behavior. Hashing, validation, persistence,
//! serialization, policy evaluation, and side effects belong to the layers
//! that consume these contracts. Keeping those concerns out makes this crate a
//! small dependency boundary for observation, capability, decision, execution,
//! and preference-learning code.

mod capability;
mod context;
mod decision;
mod identity;
mod observation;
mod outcome;
mod preference;
mod product_boundary;
mod tool;
mod validation;

pub use capability::CandidateRef;
pub use capability::CapabilityDescriptor;
pub use capability::CapabilityManifestRef;
pub use capability::CapabilityRequest;
pub use capability::CapabilityRequestRef;
pub use capability::JointCandidate;
pub use context::FrozenTurnContext;
pub use context::RevisionStamp;
pub use decision::Admission;
pub use decision::AdmissionDecision;
pub use decision::AdmissionRef;
pub use decision::Authorization;
pub use decision::AuthorizationDecision;
pub use decision::AuthorizationRef;
pub use identity::AdmissionId;
pub use identity::AuthorizationId;
pub use identity::CandidateId;
pub use identity::CapabilityId;
pub use identity::CapabilityRequestId;
pub use identity::ContentHash;
pub use identity::ObservationId;
pub use identity::PreferenceEvidenceId;
pub use identity::PreferenceId;
pub use identity::PreferenceTransitionId;
pub use identity::PrincipalId;
pub use identity::ReceiptId;
pub use identity::Revision;
pub use observation::ObservationFact;
pub use observation::ObservationRef;
pub use observation::ObservationSnapshot;
pub use outcome::OutcomeReceipt;
pub use outcome::OutcomeReceiptParts;
pub use outcome::OutcomeStatus;
pub use outcome::ReceiptRef;
pub use preference::PreferenceEvidenceRef;
pub use preference::PreferenceEvidenceSignal;
pub use preference::PreferenceState;
pub use preference::PreferenceTransition;
pub use product_boundary::LEGACY_CONTROL_UI_MUTATION_DISPOSITION;
pub use product_boundary::LEGACY_CONTROL_UI_MUTATION_NEXT_ACTION;
pub use product_boundary::LEGACY_TELEGRAM_REPLACEMENT_DISPOSITION;
pub use product_boundary::LEGACY_TELEGRAM_REPLACEMENT_NEXT_ACTION;
pub use product_boundary::OPENCLAW_GOVERNED_BACKEND_ROLE;
pub use product_boundary::PRODUCT_BOUNDARY;
pub use product_boundary::ProductBoundarySpec;
pub use tool::ToolSchema;
pub use validation::ContractError;
