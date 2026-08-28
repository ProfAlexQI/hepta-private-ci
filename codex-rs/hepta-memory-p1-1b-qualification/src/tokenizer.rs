use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use crate::ContractError;
use crate::Digest32;
use crate::MAX_INPUT_BYTES;
use crate::digest::framed_digest;
use crate::usize_to_u32;
use crate::validate_id;

pub const MAX_TOKENIZER_REGISTRY_ENTRIES: usize = 32;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TokenizerContract {
    AlphanumericPunctuationV1,
    ModelAdapterV1,
}

impl TokenizerContract {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlphanumericPunctuationV1 => "alphanumeric_punctuation_v1",
            Self::ModelAdapterV1 => "model_adapter_v1",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TokenizerImplementationKind {
    QualificationReference,
    ModelAdapter,
}

impl TokenizerImplementationKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualificationReference => "qualification_reference",
            Self::ModelAdapter => "model_adapter",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalTokenizerDescriptor {
    pub tokenizer_id: String,
    pub artifact_sha256: Digest32,
    pub vocabulary_sha256: Digest32,
    pub model_compatibility_sha256: Digest32,
    pub contract: TokenizerContract,
    pub implementation_kind: TokenizerImplementationKind,
    pub max_input_bytes: u32,
    pub local_execution_only: bool,
    pub remote_execution: bool,
    pub model_download: bool,
}

impl LocalTokenizerDescriptor {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.tokenizer_id, "tokenizer id")?;
        if self.max_input_bytes == 0
            || usize::try_from(self.max_input_bytes).unwrap_or(usize::MAX) > MAX_INPUT_BYTES
        {
            return Err(ContractError::Invalid(format!(
                "tokenizer max input must contain 1..={MAX_INPUT_BYTES} bytes"
            )));
        }
        if !self.local_execution_only || self.remote_execution || self.model_download {
            return Err(ContractError::Invalid(
                "P1.1b tokenizer descriptor crosses the local-only authority boundary".to_string(),
            ));
        }
        Ok(())
    }

    pub fn descriptor_sha256(&self) -> Result<Digest32, ContractError> {
        self.validate()?;
        Ok(framed_digest(
            b"hepta:intelligence:p1.1b:tokenizer-descriptor:v1",
            &[
                self.tokenizer_id.as_bytes(),
                self.artifact_sha256.as_bytes(),
                self.vocabulary_sha256.as_bytes(),
                self.model_compatibility_sha256.as_bytes(),
                self.contract.as_str().as_bytes(),
                self.implementation_kind.as_str().as_bytes(),
                &self.max_input_bytes.to_be_bytes(),
                &[u8::from(self.local_execution_only)],
                &[u8::from(self.remote_execution)],
                &[u8::from(self.model_download)],
            ],
        ))
    }
}

pub trait LocalTokenizerEngine: Send + Sync {
    fn descriptor(&self) -> &LocalTokenizerDescriptor;

    fn count_tokens(&self, input: &str) -> Result<u32, ContractError>;
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TokenCountMode {
    ExactLocal,
    Utf8ByteUpperBound,
}

impl TokenCountMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLocal => "exact_local",
            Self::Utf8ByteUpperBound => "utf8_byte_upper_bound",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenCountReceipt {
    pub requested_tokenizer_id: Option<String>,
    pub tokenizer_descriptor_sha256: Option<Digest32>,
    pub tokenizer_artifact_sha256: Option<Digest32>,
    pub tokenizer_vocabulary_sha256: Option<Digest32>,
    pub model_compatibility_sha256: Option<Digest32>,
    pub input_sha256: Digest32,
    pub input_bytes: u32,
    pub token_count: u32,
    pub mode: TokenCountMode,
    pub exact: bool,
    pub local_execution: bool,
    pub production_model_compatibility_verified: bool,
    pub remote_execution: bool,
    pub model_download: bool,
    pub fallback_reason: Option<String>,
    pub receipt_sha256: Digest32,
}

impl TokenCountReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.exact != (self.mode == TokenCountMode::ExactLocal) {
            return Err(ContractError::Corrupt(
                "token count exact flag does not match its mode".to_string(),
            ));
        }
        if !self.local_execution
            || self.production_model_compatibility_verified
            || self.remote_execution
            || self.model_download
        {
            return Err(ContractError::Corrupt(
                "token count receipt crosses the P1.1b authority boundary".to_string(),
            ));
        }
        match self.mode {
            TokenCountMode::ExactLocal => {
                if self.requested_tokenizer_id.is_none()
                    || self.tokenizer_descriptor_sha256.is_none()
                    || self.tokenizer_artifact_sha256.is_none()
                    || self.tokenizer_vocabulary_sha256.is_none()
                    || self.model_compatibility_sha256.is_none()
                    || self.fallback_reason.is_some()
                {
                    return Err(ContractError::Corrupt(
                        "exact token receipt is missing tokenizer bindings".to_string(),
                    ));
                }
            }
            TokenCountMode::Utf8ByteUpperBound => {
                if self.token_count != self.input_bytes || self.fallback_reason.is_none() {
                    return Err(ContractError::Corrupt(
                        "fallback token receipt is not a UTF-8 byte upper bound".to_string(),
                    ));
                }
            }
        }
        if self.receipt_sha256 != token_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "token count receipt digest does not match its contents".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct LocalTokenizerRegistry {
    engines: BTreeMap<String, Box<dyn LocalTokenizerEngine>>,
}

impl LocalTokenizerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        engine: Box<dyn LocalTokenizerEngine>,
    ) -> Result<Digest32, ContractError> {
        if self.engines.len() >= MAX_TOKENIZER_REGISTRY_ENTRIES {
            return Err(ContractError::Limit {
                label: "tokenizer registry entries",
                max: MAX_TOKENIZER_REGISTRY_ENTRIES,
            });
        }
        let descriptor = engine.descriptor();
        descriptor.validate()?;
        let id = descriptor.tokenizer_id.clone();
        let digest = descriptor.descriptor_sha256()?;
        match self.engines.entry(id.clone()) {
            Entry::Occupied(_) => Err(ContractError::Duplicate(format!(
                "tokenizer id {id}"
            ))),
            Entry::Vacant(entry) => {
                entry.insert(engine);
                Ok(digest)
            }
        }
    }

    pub fn descriptor(&self, tokenizer_id: &str) -> Option<&LocalTokenizerDescriptor> {
        self.engines.get(tokenizer_id).map(|engine| engine.descriptor())
    }

    pub fn count_or_fallback(
        &self,
        tokenizer_id: Option<&str>,
        input: &str,
    ) -> Result<TokenCountReceipt, ContractError> {
        if input.len() > MAX_INPUT_BYTES {
            return Err(ContractError::Limit {
                label: "tokenizer input bytes",
                max: MAX_INPUT_BYTES,
            });
        }
        if let Some(tokenizer_id) = tokenizer_id {
            validate_id(tokenizer_id, "requested tokenizer id")?;
            if let Some(engine) = self.engines.get(tokenizer_id) {
                return exact_receipt(engine.as_ref(), input);
            }
            return fallback_receipt(
                Some(tokenizer_id.to_string()),
                input,
                "tokenizer_unavailable",
            );
        }
        fallback_receipt(None, input, "tokenizer_not_requested")
    }
}

pub struct AlphanumericPunctuationTokenizer {
    descriptor: LocalTokenizerDescriptor,
}

impl AlphanumericPunctuationTokenizer {
    pub fn new(descriptor: LocalTokenizerDescriptor) -> Result<Self, ContractError> {
        descriptor.validate()?;
        if descriptor.contract != TokenizerContract::AlphanumericPunctuationV1
            || descriptor.implementation_kind
                != TokenizerImplementationKind::QualificationReference
        {
            return Err(ContractError::Invalid(
                "reference tokenizer requires the qualification reference contract".to_string(),
            ));
        }
        Ok(Self { descriptor })
    }
}

impl LocalTokenizerEngine for AlphanumericPunctuationTokenizer {
    fn descriptor(&self) -> &LocalTokenizerDescriptor {
        &self.descriptor
    }

    fn count_tokens(&self, input: &str) -> Result<u32, ContractError> {
        if input.len()
            > usize::try_from(self.descriptor.max_input_bytes).unwrap_or(usize::MAX)
        {
            return Err(ContractError::Limit {
                label: "exact tokenizer input bytes",
                max: usize::try_from(self.descriptor.max_input_bytes)
                    .unwrap_or(MAX_INPUT_BYTES),
            });
        }

        let mut count = 0_u32;
        let mut in_alphanumeric = false;
        for character in input.chars() {
            if character.is_alphanumeric() {
                if !in_alphanumeric {
                    count = count.checked_add(1).ok_or(ContractError::Overflow)?;
                    in_alphanumeric = true;
                }
            } else {
                in_alphanumeric = false;
                if !character.is_whitespace() {
                    count = count.checked_add(1).ok_or(ContractError::Overflow)?;
                }
            }
        }
        Ok(count)
    }
}

fn exact_receipt(
    engine: &dyn LocalTokenizerEngine,
    input: &str,
) -> Result<TokenCountReceipt, ContractError> {
    let descriptor = engine.descriptor();
    descriptor.validate()?;
    let token_count = engine.count_tokens(input)?;
    let mut receipt = TokenCountReceipt {
        requested_tokenizer_id: Some(descriptor.tokenizer_id.clone()),
        tokenizer_descriptor_sha256: Some(descriptor.descriptor_sha256()?),
        tokenizer_artifact_sha256: Some(descriptor.artifact_sha256),
        tokenizer_vocabulary_sha256: Some(descriptor.vocabulary_sha256),
        model_compatibility_sha256: Some(descriptor.model_compatibility_sha256),
        input_sha256: Digest32::for_bytes(input.as_bytes()),
        input_bytes: usize_to_u32(input.len(), "tokenizer input bytes")?,
        token_count,
        mode: TokenCountMode::ExactLocal,
        exact: true,
        local_execution: true,
        production_model_compatibility_verified: false,
        remote_execution: false,
        model_download: false,
        fallback_reason: None,
        receipt_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    receipt.receipt_sha256 = token_receipt_digest(&receipt);
    receipt.validate()?;
    Ok(receipt)
}

fn fallback_receipt(
    requested_tokenizer_id: Option<String>,
    input: &str,
    reason: &str,
) -> Result<TokenCountReceipt, ContractError> {
    let bytes = usize_to_u32(input.len(), "fallback tokenizer input bytes")?;
    let mut receipt = TokenCountReceipt {
        requested_tokenizer_id,
        tokenizer_descriptor_sha256: None,
        tokenizer_artifact_sha256: None,
        tokenizer_vocabulary_sha256: None,
        model_compatibility_sha256: None,
        input_sha256: Digest32::for_bytes(input.as_bytes()),
        input_bytes: bytes,
        token_count: bytes,
        mode: TokenCountMode::Utf8ByteUpperBound,
        exact: false,
        local_execution: true,
        production_model_compatibility_verified: false,
        remote_execution: false,
        model_download: false,
        fallback_reason: Some(reason.to_string()),
        receipt_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    receipt.receipt_sha256 = token_receipt_digest(&receipt);
    receipt.validate()?;
    Ok(receipt)
}

fn token_receipt_digest(receipt: &TokenCountReceipt) -> Digest32 {
    let requested_id = receipt.requested_tokenizer_id.as_deref().unwrap_or("");
    let descriptor = receipt
        .tokenizer_descriptor_sha256
        .map_or([0_u8; 32], |digest| *digest.as_bytes());
    let artifact = receipt
        .tokenizer_artifact_sha256
        .map_or([0_u8; 32], |digest| *digest.as_bytes());
    let vocabulary = receipt
        .tokenizer_vocabulary_sha256
        .map_or([0_u8; 32], |digest| *digest.as_bytes());
    let compatibility = receipt
        .model_compatibility_sha256
        .map_or([0_u8; 32], |digest| *digest.as_bytes());
    let fallback_reason = receipt.fallback_reason.as_deref().unwrap_or("");

    framed_digest(
        b"hepta:intelligence:p1.1b:token-count-receipt:v1",
        &[
            requested_id.as_bytes(),
            &descriptor,
            &artifact,
            &vocabulary,
            &compatibility,
            receipt.input_sha256.as_bytes(),
            &receipt.input_bytes.to_be_bytes(),
            &receipt.token_count.to_be_bytes(),
            receipt.mode.as_str().as_bytes(),
            &[u8::from(receipt.exact)],
            &[u8::from(receipt.local_execution)],
            &[u8::from(receipt.production_model_compatibility_verified)],
            &[u8::from(receipt.remote_execution)],
            &[u8::from(receipt.model_download)],
            fallback_reason.as_bytes(),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor() -> LocalTokenizerDescriptor {
        LocalTokenizerDescriptor {
            tokenizer_id: "qualification-tokenizer-v1".to_string(),
            artifact_sha256: Digest32::for_bytes(b"tokenizer"),
            vocabulary_sha256: Digest32::for_bytes(b"vocabulary"),
            model_compatibility_sha256: Digest32::for_bytes(b"model"),
            contract: TokenizerContract::AlphanumericPunctuationV1,
            implementation_kind: TokenizerImplementationKind::QualificationReference,
            max_input_bytes: 4096,
            local_execution_only: true,
            remote_execution: false,
            model_download: false,
        }
    }

    #[test]
    fn exact_and_fallback_receipts_are_distinct() {
        let mut registry = LocalTokenizerRegistry::new();
        registry
            .register(Box::new(
                AlphanumericPunctuationTokenizer::new(descriptor()).expect("tokenizer"),
            ))
            .expect("register");

        let exact = registry
            .count_or_fallback(Some("qualification-tokenizer-v1"), "hello, 世界!")
            .expect("exact");
        assert_eq!(exact.token_count, 4);
        assert!(exact.exact);
        exact.validate().expect("exact receipt");

        let fallback = registry
            .count_or_fallback(Some("missing"), "世界")
            .expect("fallback");
        assert_eq!(fallback.token_count, 6);
        assert!(!fallback.exact);
        fallback.validate().expect("fallback receipt");
    }
}
