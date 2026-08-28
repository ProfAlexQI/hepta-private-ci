use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use crate::ContractError;
use crate::Digest32;
use crate::MAX_INPUT_BYTES;
use crate::digest::framed_digest;
use crate::validate_id;

pub const MAX_EMBEDDING_REGISTRY_ENTRIES: usize = 16;
pub const MAX_EMBEDDING_DIMENSIONS: u32 = 4096;
pub const MAX_EMBEDDING_BATCH: u32 = 256;
pub const Q15_UNIT_NORM_SQUARED: u64 = 1_073_676_289;
const Q15_NORM_TOLERANCE_PPM: u64 = 100_000;
const SCORE_SCALE_PPM: u64 = 1_000_000;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EmbeddingProviderKind {
    QualificationReference,
    ModelAdapter,
}

impl EmbeddingProviderKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualificationReference => "qualification_reference",
            Self::ModelAdapter => "model_adapter",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EmbeddingMetric {
    Cosine,
}

impl EmbeddingMetric {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cosine => "cosine",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum VectorQuantization {
    I16Q15Unit,
}

impl VectorQuantization {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::I16Q15Unit => "i16_q15_unit",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalEmbeddingDescriptor {
    pub provider_id: String,
    pub model_id: String,
    pub model_sha256: Digest32,
    pub tokenizer_sha256: Digest32,
    pub dimensions: u32,
    pub max_batch: u32,
    pub max_input_bytes: u32,
    pub metric: EmbeddingMetric,
    pub quantization: VectorQuantization,
    pub provider_kind: EmbeddingProviderKind,
    pub local_execution_only: bool,
    pub remote_execution: bool,
    pub model_download: bool,
    pub network_access: bool,
    pub production_model: bool,
}

impl LocalEmbeddingDescriptor {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.provider_id, "embedding provider id")?;
        validate_id(&self.model_id, "embedding model id")?;
        if !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&self.dimensions) {
            return Err(ContractError::Invalid(format!(
                "embedding dimensions must contain 8..={MAX_EMBEDDING_DIMENSIONS}"
            )));
        }
        if !(1..=MAX_EMBEDDING_BATCH).contains(&self.max_batch) {
            return Err(ContractError::Invalid(format!(
                "embedding max batch must contain 1..={MAX_EMBEDDING_BATCH}"
            )));
        }
        if self.max_input_bytes == 0
            || usize::try_from(self.max_input_bytes).unwrap_or(usize::MAX) > MAX_INPUT_BYTES
        {
            return Err(ContractError::Invalid(format!(
                "embedding max input must contain 1..={MAX_INPUT_BYTES} bytes"
            )));
        }
        if !self.local_execution_only
            || self.remote_execution
            || self.model_download
            || self.network_access
        {
            return Err(ContractError::Invalid(
                "embedding descriptor crosses the local-only authority boundary".to_string(),
            ));
        }
        if self.provider_kind == EmbeddingProviderKind::QualificationReference
            && self.production_model
        {
            return Err(ContractError::Invalid(
                "qualification reference provider cannot claim a production model".to_string(),
            ));
        }
        Ok(())
    }

    pub fn descriptor_sha256(&self) -> Result<Digest32, ContractError> {
        self.validate()?;
        Ok(framed_digest(
            b"hepta:intelligence:p1.1b:embedding-descriptor:v1",
            &[
                self.provider_id.as_bytes(),
                self.model_id.as_bytes(),
                self.model_sha256.as_bytes(),
                self.tokenizer_sha256.as_bytes(),
                &self.dimensions.to_be_bytes(),
                &self.max_batch.to_be_bytes(),
                &self.max_input_bytes.to_be_bytes(),
                self.metric.as_str().as_bytes(),
                self.quantization.as_str().as_bytes(),
                self.provider_kind.as_str().as_bytes(),
                &[u8::from(self.local_execution_only)],
                &[u8::from(self.remote_execution)],
                &[u8::from(self.model_download)],
                &[u8::from(self.network_access)],
                &[u8::from(self.production_model)],
            ],
        ))
    }
}

pub trait LocalEmbeddingProvider: Send + Sync {
    fn descriptor(&self) -> &LocalEmbeddingDescriptor;

    fn embed_batch(&self, inputs: &[&str]) -> Result<Vec<Vec<i16>>, ContractError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddedVector {
    pub provider_descriptor_sha256: Digest32,
    pub provider_id: String,
    pub model_sha256: Digest32,
    pub tokenizer_sha256: Digest32,
    pub input_sha256: Digest32,
    pub dimensions: u32,
    pub metric: EmbeddingMetric,
    pub quantization: VectorQuantization,
    pub vector: Vec<i16>,
    pub vector_sha256: Digest32,
    pub norm_squared: u64,
    pub local_execution: bool,
    pub remote_execution: bool,
    pub model_download: bool,
    pub production_model_verified: bool,
    pub receipt_sha256: Digest32,
}

impl EmbeddedVector {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.provider_id, "embedded vector provider id")?;
        if self.dimensions == 0 || usize::try_from(self.dimensions).ok() != Some(self.vector.len())
        {
            return Err(ContractError::Corrupt(
                "embedded vector dimensions do not match vector length".to_string(),
            ));
        }
        if self.vector_sha256 != vector_digest(&self.vector) {
            return Err(ContractError::Corrupt(
                "embedded vector digest does not match vector bytes".to_string(),
            ));
        }
        let norm_squared = norm_squared(&self.vector)?;
        if norm_squared != self.norm_squared || !norm_is_q15_unit(norm_squared) {
            return Err(ContractError::Corrupt(
                "embedded vector is not a bounded Q15 unit vector".to_string(),
            ));
        }
        if !self.local_execution
            || self.remote_execution
            || self.model_download
            || self.production_model_verified
        {
            return Err(ContractError::Corrupt(
                "embedded vector receipt crosses the P1.1b authority boundary".to_string(),
            ));
        }
        if self.receipt_sha256 != embedded_vector_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "embedded vector receipt digest does not match its contents".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct EmbeddingRegistry {
    providers: BTreeMap<String, Box<dyn LocalEmbeddingProvider>>,
}

impl EmbeddingRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        provider: Box<dyn LocalEmbeddingProvider>,
    ) -> Result<Digest32, ContractError> {
        if self.providers.len() >= MAX_EMBEDDING_REGISTRY_ENTRIES {
            return Err(ContractError::Limit {
                label: "embedding registry entries",
                max: MAX_EMBEDDING_REGISTRY_ENTRIES,
            });
        }
        provider.descriptor().validate()?;
        let provider_id = provider.descriptor().provider_id.clone();
        let descriptor_sha256 = provider.descriptor().descriptor_sha256()?;
        match self.providers.entry(provider_id.clone()) {
            Entry::Occupied(_) => Err(ContractError::Duplicate(format!(
                "embedding provider id {provider_id}"
            ))),
            Entry::Vacant(entry) => {
                entry.insert(provider);
                Ok(descriptor_sha256)
            }
        }
    }

    pub fn descriptor(&self, provider_id: &str) -> Option<&LocalEmbeddingDescriptor> {
        self.providers
            .get(provider_id)
            .map(|provider| provider.descriptor())
    }

    pub fn embed_batch(
        &self,
        provider_id: &str,
        inputs: &[&str],
    ) -> Result<Vec<EmbeddedVector>, ContractError> {
        validate_id(provider_id, "requested embedding provider id")?;
        let provider = self
            .providers
            .get(provider_id)
            .ok_or(ContractError::Unavailable("embedding_provider"))?;
        let descriptor = provider.descriptor();
        descriptor.validate()?;

        if inputs.is_empty()
            || inputs.len() > usize::try_from(descriptor.max_batch).unwrap_or(usize::MAX)
        {
            return Err(ContractError::Limit {
                label: "embedding batch size",
                max: usize::try_from(descriptor.max_batch).unwrap_or(usize::MAX),
            });
        }
        for input in inputs {
            if input.len() > usize::try_from(descriptor.max_input_bytes).unwrap_or(usize::MAX) {
                return Err(ContractError::Limit {
                    label: "embedding input bytes",
                    max: usize::try_from(descriptor.max_input_bytes).unwrap_or(MAX_INPUT_BYTES),
                });
            }
        }

        let vectors = provider.embed_batch(inputs)?;
        if vectors.len() != inputs.len() {
            return Err(ContractError::Corrupt(
                "embedding provider returned the wrong batch cardinality".to_string(),
            ));
        }

        let descriptor_sha256 = descriptor.descriptor_sha256()?;
        inputs
            .iter()
            .zip(vectors)
            .map(|(input, vector)| {
                validate_provider_vector(descriptor, descriptor_sha256, input, vector)
            })
            .collect()
    }
}

pub struct QualificationHashOneHotProvider {
    descriptor: LocalEmbeddingDescriptor,
}

impl QualificationHashOneHotProvider {
    pub fn new(descriptor: LocalEmbeddingDescriptor) -> Result<Self, ContractError> {
        descriptor.validate()?;
        if descriptor.provider_kind != EmbeddingProviderKind::QualificationReference
            || descriptor.production_model
        {
            return Err(ContractError::Invalid(
                "hash one-hot provider is qualification-only".to_string(),
            ));
        }
        Ok(Self { descriptor })
    }
}

impl LocalEmbeddingProvider for QualificationHashOneHotProvider {
    fn descriptor(&self) -> &LocalEmbeddingDescriptor {
        &self.descriptor
    }

    fn embed_batch(&self, inputs: &[&str]) -> Result<Vec<Vec<i16>>, ContractError> {
        let dimensions =
            usize::try_from(self.descriptor.dimensions).map_err(|_| ContractError::Overflow)?;
        inputs
            .iter()
            .map(|input| {
                let digest = Digest32::for_bytes(input.as_bytes());
                let mut selector = [0_u8; 8];
                selector.copy_from_slice(&digest.as_bytes()[..8]);
                let index = usize::try_from(u64::from_be_bytes(selector)).unwrap_or(usize::MAX)
                    % dimensions;
                let mut vector = vec![0_i16; dimensions];
                vector[index] = if digest.as_bytes()[8] & 1 == 0 {
                    i16::MAX
                } else {
                    -i16::MAX
                };
                Ok(vector)
            })
            .collect()
    }
}

fn validate_provider_vector(
    descriptor: &LocalEmbeddingDescriptor,
    descriptor_sha256: Digest32,
    input: &str,
    vector: Vec<i16>,
) -> Result<EmbeddedVector, ContractError> {
    if usize::try_from(descriptor.dimensions).ok() != Some(vector.len()) {
        return Err(ContractError::Corrupt(
            "embedding provider output dimension mismatch".to_string(),
        ));
    }
    let norm_squared = norm_squared(&vector)?;
    if !norm_is_q15_unit(norm_squared) {
        return Err(ContractError::Corrupt(
            "embedding provider output is not Q15 unit-normalized".to_string(),
        ));
    }
    let mut receipt = EmbeddedVector {
        provider_descriptor_sha256: descriptor_sha256,
        provider_id: descriptor.provider_id.clone(),
        model_sha256: descriptor.model_sha256,
        tokenizer_sha256: descriptor.tokenizer_sha256,
        input_sha256: Digest32::for_bytes(input.as_bytes()),
        dimensions: descriptor.dimensions,
        metric: descriptor.metric,
        quantization: descriptor.quantization,
        vector_sha256: vector_digest(&vector),
        vector,
        norm_squared,
        local_execution: true,
        remote_execution: false,
        model_download: false,
        production_model_verified: false,
        receipt_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    receipt.receipt_sha256 = embedded_vector_receipt_digest(&receipt);
    receipt.validate()?;
    Ok(receipt)
}

pub(crate) fn vector_digest(vector: &[i16]) -> Digest32 {
    let mut bytes = Vec::with_capacity(vector.len().saturating_mul(2));
    for value in vector {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    Digest32::for_bytes(&bytes)
}

pub(crate) fn norm_squared(vector: &[i16]) -> Result<u64, ContractError> {
    vector.iter().try_fold(0_u64, |sum, value| {
        let signed = i64::from(*value);
        let square =
            u64::try_from(signed.saturating_mul(signed)).map_err(|_| ContractError::Overflow)?;
        sum.checked_add(square).ok_or(ContractError::Overflow)
    })
}

fn norm_is_q15_unit(norm_squared: u64) -> bool {
    let difference = norm_squared.abs_diff(Q15_UNIT_NORM_SQUARED);
    difference.saturating_mul(SCORE_SCALE_PPM)
        <= Q15_UNIT_NORM_SQUARED.saturating_mul(Q15_NORM_TOLERANCE_PPM)
}

fn embedded_vector_receipt_digest(receipt: &EmbeddedVector) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1b:embedded-vector-receipt:v1",
        &[
            receipt.provider_descriptor_sha256.as_bytes(),
            receipt.provider_id.as_bytes(),
            receipt.model_sha256.as_bytes(),
            receipt.tokenizer_sha256.as_bytes(),
            receipt.input_sha256.as_bytes(),
            &receipt.dimensions.to_be_bytes(),
            receipt.metric.as_str().as_bytes(),
            receipt.quantization.as_str().as_bytes(),
            receipt.vector_sha256.as_bytes(),
            &receipt.norm_squared.to_be_bytes(),
            &[u8::from(receipt.local_execution)],
            &[u8::from(receipt.remote_execution)],
            &[u8::from(receipt.model_download)],
            &[u8::from(receipt.production_model_verified)],
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor() -> LocalEmbeddingDescriptor {
        LocalEmbeddingDescriptor {
            provider_id: "qualification-hash-one-hot".to_string(),
            model_id: "qualification-model".to_string(),
            model_sha256: Digest32::for_bytes(b"model"),
            tokenizer_sha256: Digest32::for_bytes(b"tokenizer"),
            dimensions: 64,
            max_batch: 8,
            max_input_bytes: 4096,
            metric: EmbeddingMetric::Cosine,
            quantization: VectorQuantization::I16Q15Unit,
            provider_kind: EmbeddingProviderKind::QualificationReference,
            local_execution_only: true,
            remote_execution: false,
            model_download: false,
            network_access: false,
            production_model: false,
        }
    }

    #[test]
    fn reference_provider_is_local_deterministic_and_receipted() {
        let mut registry = EmbeddingRegistry::new();
        registry
            .register(Box::new(
                QualificationHashOneHotProvider::new(descriptor()).expect("provider"),
            ))
            .expect("register");
        let first = registry
            .embed_batch("qualification-hash-one-hot", &["alpha", "alpha"])
            .expect("embed");
        assert_eq!(first[0].vector, first[1].vector);
        assert_eq!(first[0].norm_squared, Q15_UNIT_NORM_SQUARED);
        first[0].validate().expect("receipt");
    }

    #[test]
    fn descriptor_rejects_remote_or_download_authority() {
        let mut invalid = descriptor();
        invalid.remote_execution = true;
        assert!(invalid.validate().is_err());
        let mut invalid = descriptor();
        invalid.model_download = true;
        assert!(invalid.validate().is_err());
    }
}
