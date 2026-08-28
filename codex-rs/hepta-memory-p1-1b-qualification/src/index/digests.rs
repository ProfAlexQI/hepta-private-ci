fn entries_digest(entries: &[StoredEntry]) -> Digest32 {
    let mut bytes = Vec::new();
    for entry in entries {
        append_length_prefixed(&mut bytes, entry.candidate_id.as_bytes());
        bytes.extend_from_slice(&entry.memory_revision.to_be_bytes());
        bytes.extend_from_slice(entry.content_sha256.as_bytes());
        bytes.extend_from_slice(entry.vector_sha256.as_bytes());
        bytes.extend_from_slice(&entry.signature.to_be_bytes());
        for value in &entry.vector {
            bytes.extend_from_slice(&value.to_be_bytes());
        }
    }
    Digest32::for_bytes(&bytes)
}

fn buckets_digest(buckets: &BTreeMap<u64, Vec<u32>>) -> Digest32 {
    let mut bytes = Vec::new();
    for (signature, indices) in buckets {
        bytes.extend_from_slice(&signature.to_be_bytes());
        bytes.extend_from_slice(
            &u32::try_from(indices.len())
                .unwrap_or(u32::MAX)
                .to_be_bytes(),
        );
        for index in indices {
            bytes.extend_from_slice(&index.to_be_bytes());
        }
    }
    Digest32::for_bytes(&bytes)
}

fn manifest_digest(manifest: &AnnIndexManifest) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1b:ann-manifest:v1",
        &[
            &manifest.schema_version.to_be_bytes(),
            manifest.namespace.as_bytes(),
            manifest.index_id.as_bytes(),
            &manifest.generation.to_be_bytes(),
            manifest.algorithm.as_str().as_bytes(),
            manifest.provider_id.as_bytes(),
            manifest.provider_descriptor_sha256.as_bytes(),
            manifest.model_sha256.as_bytes(),
            manifest.tokenizer_sha256.as_bytes(),
            &manifest.dimensions.to_be_bytes(),
            manifest.metric.as_str().as_bytes(),
            manifest.quantization.as_str().as_bytes(),
            manifest.seed_sha256.as_bytes(),
            &manifest.item_count.to_be_bytes(),
            &manifest.bucket_count.to_be_bytes(),
            manifest.entries_sha256.as_bytes(),
            manifest.buckets_sha256.as_bytes(),
            &[u8::from(manifest.immutable)],
            &[u8::from(manifest.local_only)],
            &[u8::from(manifest.remote_embedding)],
            &[u8::from(manifest.model_download)],
            &[u8::from(manifest.network_access)],
            &[u8::from(manifest.runtime_wired)],
            &[u8::from(manifest.default_recall_changed)],
            &[u8::from(manifest.context_attachment)],
            &[u8::from(manifest.physical_send)],
            &[u8::from(manifest.production_authority)],
        ],
    )
}

fn index_write_receipt_digest(receipt: &IndexWriteReceipt) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1b:index-write-receipt:v1",
        &[
            receipt.index_id.as_bytes(),
            &receipt.generation.to_be_bytes(),
            receipt.manifest_sha256.as_bytes(),
            receipt.file_sha256.as_bytes(),
            &receipt.file_bytes.to_be_bytes(),
            &[u8::from(receipt.create_only)],
            &[u8::from(receipt.immutable)],
            &[u8::from(receipt.local_only)],
            &[u8::from(receipt.remote_embedding)],
            &[u8::from(receipt.model_download)],
            &[u8::from(receipt.network_access)],
            &[u8::from(receipt.production_authority)],
        ],
    )
}

fn ann_result_digest(result: &AnnSearchResult) -> Digest32 {
    framed_digest(
        b"hepta:intelligence:p1.1b:ann-search-result:v1",
        &[
            result.candidate_id.as_bytes(),
            &result.memory_revision.to_be_bytes(),
            result.content_sha256.as_bytes(),
            &result.similarity_ppm.to_be_bytes(),
            result.vector_sha256.as_bytes(),
        ],
    )
}

fn ann_search_receipt_digest(receipt: &AnnSearchReceipt) -> Digest32 {
    let mut result_bytes = Vec::new();
    for result in &receipt.results {
        result_bytes.extend_from_slice(result.result_sha256.as_bytes());
    }
    let fallback = receipt.fallback_reason.as_deref().unwrap_or("");
    framed_digest(
        b"hepta:intelligence:p1.1b:ann-search-receipt:v1",
        &[
            receipt.index_id.as_bytes(),
            &receipt.generation.to_be_bytes(),
            receipt.manifest_sha256.as_bytes(),
            receipt.query_vector_sha256.as_bytes(),
            &receipt.query_signature.to_be_bytes(),
            &receipt.visited_bucket_count.to_be_bytes(),
            &receipt.scanned_candidate_count.to_be_bytes(),
            &receipt.result_count.to_be_bytes(),
            &result_bytes,
            &[u8::from(receipt.ann_search_executed)],
            &[u8::from(receipt.lexical_fallback_required)],
            fallback.as_bytes(),
            &[u8::from(receipt.runtime_wired)],
            &[u8::from(receipt.default_recall_changed)],
            &[u8::from(receipt.federation_recall_changed)],
            &[u8::from(receipt.context_attachment)],
            &[u8::from(receipt.physical_send)],
            &[u8::from(receipt.remote_embedding)],
            &[u8::from(receipt.model_download)],
            &[u8::from(receipt.network_access)],
            &[u8::from(receipt.external_effects)],
            &[u8::from(receipt.production_authority)],
            &[u8::from(receipt.operator_acceptance)],
            &[u8::from(receipt.promotion)],
            &[u8::from(receipt.callers_ratchet)],
        ],
    )
}

fn metric_code(metric: EmbeddingMetric) -> u32 {
    match metric {
        EmbeddingMetric::Cosine => 1,
    }
}

fn metric_from_code(code: u32) -> Result<EmbeddingMetric, ContractError> {
    match code {
        1 => Ok(EmbeddingMetric::Cosine),
        _ => Err(ContractError::Corrupt(
            "unknown embedding metric code".to_string(),
        )),
    }
}

fn quantization_code(quantization: VectorQuantization) -> u32 {
    match quantization {
        VectorQuantization::I16Q15Unit => 1,
    }
}

fn quantization_from_code(code: u32) -> Result<VectorQuantization, ContractError> {
    match code {
        1 => Ok(VectorQuantization::I16Q15Unit),
        _ => Err(ContractError::Corrupt(
            "unknown vector quantization code".to_string(),
        )),
    }
}
