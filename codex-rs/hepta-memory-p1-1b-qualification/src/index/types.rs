#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AnnAlgorithm {
    DeterministicLsh64V1,
}

impl AnnAlgorithm {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeterministicLsh64V1 => "deterministic_lsh64_v1",
        }
    }

    const fn code(self) -> u32 {
        match self {
            Self::DeterministicLsh64V1 => 1,
        }
    }

    fn from_code(code: u32) -> Result<Self, ContractError> {
        match code {
            1 => Ok(Self::DeterministicLsh64V1),
            _ => Err(ContractError::Corrupt(
                "unknown local ANN algorithm code".to_string(),
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnIndexItemDraft {
    pub candidate_id: String,
    pub memory_revision: u64,
    pub content_sha256: Digest32,
    pub embedding: EmbeddedVector,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnIndexBuildDraft {
    pub index_id: String,
    pub generation: u64,
    pub seed_sha256: Digest32,
    pub provider: LocalEmbeddingDescriptor,
    pub items: Vec<AnnIndexItemDraft>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnIndexManifest {
    pub schema_version: u32,
    pub namespace: String,
    pub index_id: String,
    pub generation: u64,
    pub algorithm: AnnAlgorithm,
    pub provider_id: String,
    pub provider_descriptor_sha256: Digest32,
    pub model_sha256: Digest32,
    pub tokenizer_sha256: Digest32,
    pub dimensions: u32,
    pub metric: EmbeddingMetric,
    pub quantization: VectorQuantization,
    pub seed_sha256: Digest32,
    pub item_count: u32,
    pub bucket_count: u32,
    pub entries_sha256: Digest32,
    pub buckets_sha256: Digest32,
    pub immutable: bool,
    pub local_only: bool,
    pub remote_embedding: bool,
    pub model_download: bool,
    pub network_access: bool,
    pub runtime_wired: bool,
    pub default_recall_changed: bool,
    pub context_attachment: bool,
    pub physical_send: bool,
    pub production_authority: bool,
    pub manifest_sha256: Digest32,
}

impl AnnIndexManifest {
    pub fn validate(&self) -> Result<(), ContractError> {
        let item_count = usize::try_from(self.item_count).unwrap_or(usize::MAX);
        let bucket_count = usize::try_from(self.bucket_count).unwrap_or(usize::MAX);
        if self.schema_version != P1_1B_SCHEMA_VERSION
            || self.namespace != INDEX_NAMESPACE
            || self.generation == 0
            || self.item_count == 0
            || item_count > MAX_INDEX_ITEMS
            || self.bucket_count == 0
            || self.bucket_count > self.item_count
            || bucket_count > MAX_INDEX_ITEMS
            || !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&self.dimensions)
        {
            return Err(ContractError::Corrupt(
                "invalid local ANN manifest shape or bounded counts".to_string(),
            ));
        }
        validate_id(&self.index_id, "ANN index id")?;
        validate_id(&self.provider_id, "ANN provider id")?;
        if !self.immutable
            || !self.local_only
            || self.remote_embedding
            || self.model_download
            || self.network_access
            || self.runtime_wired
            || self.default_recall_changed
            || self.context_attachment
            || self.physical_send
            || self.production_authority
        {
            return Err(ContractError::Corrupt(
                "ANN manifest crosses the source-only authority boundary".to_string(),
            ));
        }
        if self.manifest_sha256 != manifest_digest(self) {
            return Err(ContractError::Corrupt(
                "ANN manifest digest does not match its contents".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedAnnIndexBinding {
    pub index_id: String,
    pub generation: u64,
    pub provider_descriptor_sha256: Digest32,
    pub model_sha256: Digest32,
    pub tokenizer_sha256: Digest32,
    pub dimensions: u32,
    pub manifest_sha256: Option<Digest32>,
}

impl ExpectedAnnIndexBinding {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.index_id, "expected ANN index id")?;
        if self.generation == 0
            || !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&self.dimensions)
        {
            return Err(ContractError::Invalid(
                "expected ANN generation or dimensions are outside the bounded contract"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexWriteReceipt {
    pub index_id: String,
    pub generation: u64,
    pub manifest_sha256: Digest32,
    pub file_sha256: Digest32,
    pub file_bytes: u64,
    pub create_only: bool,
    pub immutable: bool,
    pub local_only: bool,
    pub remote_embedding: bool,
    pub model_download: bool,
    pub network_access: bool,
    pub production_authority: bool,
    pub receipt_sha256: Digest32,
}

impl IndexWriteReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.index_id, "index write receipt id")?;
        if self.generation == 0
            || self.file_bytes == 0
            || self.file_bytes > MAX_INDEX_FILE_BYTES
            || !self.create_only
            || !self.immutable
            || !self.local_only
            || self.remote_embedding
            || self.model_download
            || self.network_access
            || self.production_authority
        {
            return Err(ContractError::Corrupt(
                "invalid index write receipt boundary".to_string(),
            ));
        }
        if self.receipt_sha256 != index_write_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "index write receipt digest mismatch".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnSearchResult {
    pub candidate_id: String,
    pub memory_revision: u64,
    pub content_sha256: Digest32,
    pub similarity_ppm: u32,
    pub vector_sha256: Digest32,
    pub result_sha256: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnSearchReceipt {
    pub index_id: String,
    pub generation: u64,
    pub manifest_sha256: Digest32,
    pub query_vector_sha256: Digest32,
    pub query_signature: u64,
    pub visited_bucket_count: u32,
    pub scanned_candidate_count: u32,
    pub result_count: u32,
    pub results: Vec<AnnSearchResult>,
    pub ann_search_executed: bool,
    pub lexical_fallback_required: bool,
    pub fallback_reason: Option<String>,
    pub runtime_wired: bool,
    pub default_recall_changed: bool,
    pub federation_recall_changed: bool,
    pub context_attachment: bool,
    pub physical_send: bool,
    pub remote_embedding: bool,
    pub model_download: bool,
    pub network_access: bool,
    pub external_effects: bool,
    pub production_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub callers_ratchet: bool,
    pub receipt_sha256: Digest32,
}

impl AnnSearchReceipt {
    pub fn validate(&self) -> Result<(), ContractError> {
        validate_id(&self.index_id, "ANN search index id")?;
        let result_count = usize::try_from(self.result_count).unwrap_or(usize::MAX);
        let scanned_candidate_count =
            usize::try_from(self.scanned_candidate_count).unwrap_or(usize::MAX);
        if self.generation == 0
            || result_count != self.results.len()
            || result_count > MAX_SEARCH_RESULTS
            || scanned_candidate_count > MAX_SEARCH_CANDIDATES
            || self.scanned_candidate_count < self.result_count
            || self.visited_bucket_count > 65
            || (self.scanned_candidate_count > 0 && self.visited_bucket_count == 0)
            || !self.ann_search_executed
            || self.runtime_wired
            || self.default_recall_changed
            || self.federation_recall_changed
            || self.context_attachment
            || self.physical_send
            || self.remote_embedding
            || self.model_download
            || self.network_access
            || self.external_effects
            || self.production_authority
            || self.operator_acceptance
            || self.promotion
            || self.callers_ratchet
        {
            return Err(ContractError::Corrupt(
                "ANN search receipt crosses the bounded source-only boundary".to_string(),
            ));
        }
        if self.lexical_fallback_required != self.results.is_empty()
            || self.lexical_fallback_required != (self.scanned_candidate_count == 0)
            || (self.lexical_fallback_required
                && self.fallback_reason.as_deref() != Some("ann_bucket_empty"))
            || (!self.lexical_fallback_required && self.fallback_reason.is_some())
        {
            return Err(ContractError::Corrupt(
                "ANN search fallback fields are inconsistent".to_string(),
            ));
        }

        let mut identities = BTreeSet::new();
        for result in &self.results {
            validate_id(&result.candidate_id, "ANN result candidate id")?;
            if result.memory_revision == 0 || result.similarity_ppm > 1_000_000 {
                return Err(ContractError::Corrupt(
                    "invalid ANN search result shape".to_string(),
                ));
            }
            if !identities.insert((result.candidate_id.clone(), result.memory_revision)) {
                return Err(ContractError::Corrupt(
                    "duplicate ANN search result identity".to_string(),
                ));
            }
            if result.result_sha256 != ann_result_digest(result) {
                return Err(ContractError::Corrupt(
                    "ANN result digest mismatch".to_string(),
                ));
            }
        }
        if self.results.windows(2).any(|pair| {
            let left = &pair[0];
            let right = &pair[1];
            left.similarity_ppm < right.similarity_ppm
                || (left.similarity_ppm == right.similarity_ppm
                    && left.candidate_id.as_str() > right.candidate_id.as_str())
                || (left.similarity_ppm == right.similarity_ppm
                    && left.candidate_id.as_str() == right.candidate_id.as_str()
                    && left.memory_revision < right.memory_revision)
        }) {
            return Err(ContractError::Corrupt(
                "ANN search results are not in deterministic order".to_string(),
            ));
        }
        if self.receipt_sha256 != ann_search_receipt_digest(self) {
            return Err(ContractError::Corrupt(
                "ANN search receipt digest mismatch".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StoredEntry {
    candidate_id: String,
    memory_revision: u64,
    content_sha256: Digest32,
    vector: Vec<i16>,
    vector_sha256: Digest32,
    signature: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalAnnIndex {
    manifest: AnnIndexManifest,
    entries: Vec<StoredEntry>,
    buckets: BTreeMap<u64, Vec<u32>>,
}
