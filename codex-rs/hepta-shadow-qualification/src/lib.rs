#![forbid(unsafe_code)]

mod digest;
mod durable;
mod importer;
#[cfg(test)]
mod importer_tests;
mod observer;
#[cfg(test)]
mod observer_tests;
mod oracle;
#[cfg(test)]
mod oracle_tests;
mod report;
#[cfg(test)]
mod report_tests;
mod request;
#[cfg(test)]
mod request_tests;
mod sealer;
#[cfg(test)]
mod sealer_tests;
mod semantic_verifier;
#[cfg(test)]
mod semantic_verifier_tests;
#[cfg(test)]
mod test_support;
mod verification_primitives;

#[derive(Debug, thiserror::Error)]
pub enum QualificationError {
    #[error("invalid qualification input: {0}")]
    Invalid(String),
    #[error("failed to serialize qualification evidence: {0}")]
    Serialization(String),
    #[error("qualification evidence I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid observer state: {0}")]
    State(String),
}

pub use importer::ImportCheckpoint;
pub use importer::ImportFailure;
pub use observer::CompletedPreSend;
pub use observer::DurablePreSendObserver;
pub use observer::DurablePreSendToken;
pub use oracle::FrozenOracle;
pub use report::QualificationManifest;
pub use report::QualificationReport;
pub use report::ReportFailure;
pub use report::SemanticSampleReport;
pub use request::Surface;
pub use sealer::TerminalSeal;
pub use sealer::TerminalStatus;
pub use semantic_verifier::SemanticVerifier;
pub use semantic_verifier::VerifiedSemanticReceipt;
