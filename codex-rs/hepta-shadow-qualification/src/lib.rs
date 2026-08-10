#![forbid(unsafe_code)]

mod digest;
mod durable;
mod observer;
#[cfg(test)]
mod observer_tests;
mod request;
#[cfg(test)]
mod request_tests;

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

pub use observer::CompletedPreSend;
pub use observer::DurablePreSendObserver;
pub use observer::DurablePreSendToken;
pub use request::Surface;
