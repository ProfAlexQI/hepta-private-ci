#![forbid(unsafe_code)]

mod digest;
mod request;
#[cfg(test)]
mod request_tests;

#[derive(Debug, thiserror::Error)]
pub enum QualificationError {
    #[error("invalid qualification input: {0}")]
    Invalid(String),
    #[error("failed to serialize qualification evidence: {0}")]
    Serialization(String),
}
