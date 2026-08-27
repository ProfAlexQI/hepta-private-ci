#![forbid(unsafe_code)]

mod browser_contracts;
mod browser_runtime;
#[cfg(test)]
mod browser_tests;
mod child;
#[cfg(test)]
mod child_tests;
mod closure;
mod digest;
mod driver;
#[cfg(test)]
mod driver_tests;
mod durable;
#[cfg(test)]
mod durable_tests;
mod importer;
#[cfg(test)]
mod importer_tests;
mod loopback;
#[cfg(test)]
mod loopback_tests;
mod observer;
#[cfg(test)]
mod observer_tests;
mod oracle;
#[cfg(test)]
mod oracle_tests;
mod product_database;
mod product_receipts;
#[cfg(test)]
mod product_receipts_tests;
mod report;
#[cfg(test)]
mod report_tests;
mod request;
#[cfg(test)]
mod request_tests;
mod runtime;
#[cfg(test)]
mod runtime_tests;
mod sealer;
#[cfg(test)]
mod sealer_tests;
mod semantic_verifier;
#[cfg(test)]
mod semantic_verifier_tests;
mod session;
#[cfg(test)]
mod session_tests;
#[cfg(test)]
mod test_support;
mod transport;
#[cfg(test)]
mod transport_tests;
mod trial;
#[cfg(test)]
mod trial_tests;
mod verification_primitives;

#[derive(Debug, thiserror::Error)]
pub enum QualificationError {
    #[error("invalid qualification input: {0}")]
    Invalid(String),
    #[error("failed to serialize qualification evidence: {0}")]
    Serialization(String),
    #[error("qualification evidence I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("qualification evidence database failed: {0}")]
    Database(#[from] sqlx::Error),
    #[error("invalid observer state: {0}")]
    State(String),
}

pub use browser_contracts::BROWSER_CONTROL_SCHEMA_VERSION;
pub use browser_contracts::BROWSER_EFFECT_AUTHORITY;
pub use browser_contracts::BROWSER_EXECUTE_ALLOWED;
pub use browser_contracts::BROWSER_EXTERNAL_EFFECT;
pub use browser_contracts::BROWSER_G5_ALLOWED;
pub use browser_contracts::BROWSER_OPERATOR_ACCEPTANCE;
pub use browser_contracts::BROWSER_PRODUCTION_CALLER;
pub use browser_contracts::BROWSER_PRODUCTION_WRITER;
pub use browser_contracts::BROWSER_PROMOTION;
pub use browser_contracts::BROWSER_QUALIFICATION_ONLY;
pub use browser_contracts::BROWSER_RECEIPT_SCHEMA_VERSION;
pub use browser_contracts::BrowserAction;
pub use browser_contracts::BrowserActivityReceipt;
pub use browser_contracts::BrowserActorKind;
pub use browser_contracts::BrowserAuthorityStatus;
pub use browser_contracts::BrowserChallengeCode;
pub use browser_contracts::BrowserCommand;
pub use browser_contracts::BrowserCommandKind;
pub use browser_contracts::BrowserControlMode;
pub use browser_contracts::BrowserDenialCode;
pub use browser_contracts::BrowserIndeterminateCode;
pub use browser_contracts::BrowserOutcome;
pub use browser_contracts::BrowserRequest;
pub use browser_contracts::BrowserResponse;
pub use browser_contracts::BrowserSessionId;
pub use browser_contracts::BrowserWaitCondition;
pub use browser_contracts::SemanticNode;
pub use browser_contracts::SemanticRef;
pub use browser_contracts::SemanticSnapshot;
pub use browser_contracts::WebEvidenceReceipt;
pub use browser_runtime::BrowserActor;
pub use browser_runtime::BrowserActorStatus;
pub use browser_runtime::BrowserEngine;
pub use browser_runtime::BrowserEngineError;
pub use browser_runtime::BrowserEngineExtract;
pub use browser_runtime::BrowserEngineNode;
pub use browser_runtime::BrowserEngineSnapshot;
pub use browser_runtime::FixtureBrowserEngine;
pub use child::ChildOutcome;
pub use child::ProductChild;
pub use closure::QualificationClosure;
pub use closure::QualificationClosureOutcome;
pub use driver::AppServerDriver;
pub use driver::McpDriver;
pub use driver::QualificationDriverRun;
pub use importer::ImportCheckpoint;
pub use importer::ImportFailure;
pub use loopback::HttpAuditRecord;
pub use loopback::LoopbackHandle;
pub use observer::CompletedPreSend;
pub use observer::DurablePreSendObserver;
pub use observer::DurablePreSendToken;
pub use oracle::FrozenOracle;
pub use product_receipts::ProductReceiptSet;
pub use report::QualificationManifest;
pub use report::QualificationReport;
pub use report::ReportFailure;
pub use report::SemanticSampleReport;
pub use request::Surface;
pub use runtime::FrozenProductBinary;
pub use runtime::QualificationRuntimeLayout;
pub use runtime::SurfaceRuntimeLayout;
pub use sealer::TerminalSeal;
pub use sealer::TerminalStatus;
pub use semantic_verifier::SemanticVerifier;
pub use semantic_verifier::VerifiedSemanticReceipt;
pub use trial::QualificationTrial;
pub use trial::QualificationTrialOutcome;
