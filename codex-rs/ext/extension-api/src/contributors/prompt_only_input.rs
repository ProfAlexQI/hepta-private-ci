use std::path::PathBuf;

use crate::ExtensionData;

use super::ExtensionFuture;
use super::ModelProviderPolicyError;
use super::ModelProviderSha256Digest;

/// Schema version for [`PromptOnlyInputProposal`].
pub const PROMPT_ONLY_INPUT_PROPOSAL_SCHEMA_VERSION: u32 = 1;

/// Stable, non-secret source identity for prompt-only input.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PromptOnlyInputSource(String);

impl PromptOnlyInputSource {
    pub fn parse(value: impl Into<String>) -> Result<Self, ModelProviderPolicyError> {
        let value = value.into();
        let mut bytes = value.bytes();
        let Some(first) = bytes.next() else {
            return Err(invalid_source());
        };
        if value.len() > 64
            || !first.is_ascii_lowercase()
            || !bytes.all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'_' | b'-' | b'.')
            })
        {
            return Err(invalid_source());
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Host facts rechecked immediately before one prompt is assembled.
#[derive(Clone, Debug)]
pub struct PromptOnlyInputContext {
    pub thread_id: String,
    pub turn_id: String,
    pub cwd: PathBuf,
    pub model_context_window: Option<i64>,
    pub host_authority_enabled: bool,
}

/// Bounded extension proposal that is never conversation history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromptOnlyInputProposal {
    pub schema_version: u32,
    pub source: PromptOnlyInputSource,
    pub thread_id: String,
    pub turn_id: String,
    pub source_binding_sha256: ModelProviderSha256Digest,
    pub content_sha256: ModelProviderSha256Digest,
    pub content: String,
    pub claimed_token_count: u32,
}

/// Contributor for bounded, host-authorized, prompt-only model input.
///
/// Implementations must revalidate their source on every call and return raw
/// domain content rather than a pre-rendered model message. The host owns final
/// rendering, witness minting, provider binding, and persistence exclusion.
pub trait PromptOnlyInputContributor: Send + Sync {
    /// Returns whether this contributor has turn-local proposal state.
    fn is_active(&self, _thread_store: &ExtensionData, _turn_store: &ExtensionData) -> bool {
        true
    }

    /// Revalidates and returns at most one proposal for the exact host turn.
    fn contribute<'a>(
        &'a self,
        input: PromptOnlyInputContext,
        session_store: &'a ExtensionData,
        thread_store: &'a ExtensionData,
        turn_store: &'a ExtensionData,
    ) -> ExtensionFuture<'a, Result<Option<PromptOnlyInputProposal>, ModelProviderPolicyError>>;
}

fn invalid_source() -> ModelProviderPolicyError {
    ModelProviderPolicyError::new(
        "prompt_only_input_source_invalid",
        "prompt-only input source must match [a-z][a-z0-9_.-]{0,63}",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_identity_is_bounded_and_canonical() {
        assert_eq!(
            PromptOnlyInputSource::parse("hepta_memory_same_thread_v1")
                .expect("source")
                .as_str(),
            "hepta_memory_same_thread_v1"
        );
        for invalid in ["", "Hepta", "hepta/memory", &"a".repeat(65)] {
            assert!(PromptOnlyInputSource::parse(invalid).is_err());
        }
    }
}
