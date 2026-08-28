pub fn build_local_ann_index(
    mut draft: AnnIndexBuildDraft,
) -> Result<LocalAnnIndex, ContractError> {
    validate_id(&draft.index_id, "ANN index id")?;
    draft.provider.validate()?;
    if draft.generation == 0 {
        return Err(ContractError::Invalid(
            "ANN index generation must be positive".to_string(),
        ));
    }
    if draft.items.is_empty() || draft.items.len() > MAX_INDEX_ITEMS {
        return Err(ContractError::Limit {
            label: "ANN index items",
            max: MAX_INDEX_ITEMS,
        });
    }
    if draft.provider.metric != EmbeddingMetric::Cosine
        || draft.provider.quantization != VectorQuantization::I16Q15Unit
    {
        return Err(ContractError::Invalid(
            "P1.1b ANN index requires cosine Q15 vectors".to_string(),
        ));
    }

    draft.items.sort_by(|left, right| {
        left.candidate_id
            .cmp(&right.candidate_id)
            .then_with(|| left.memory_revision.cmp(&right.memory_revision))
    });

    let descriptor_sha256 = draft.provider.descriptor_sha256()?;
    let mut entries = Vec::with_capacity(draft.items.len());
    let mut previous_identity: Option<(String, u64)> = None;
    for item in draft.items {
        validate_id(&item.candidate_id, "ANN candidate id")?;
        item.embedding.validate()?;
        if item.memory_revision == 0 {
            return Err(ContractError::Invalid(
                "ANN memory revision must be positive".to_string(),
            ));
        }
        let identity = (item.candidate_id.clone(), item.memory_revision);
        if previous_identity.as_ref() == Some(&identity) {
            return Err(ContractError::Duplicate(format!(
                "ANN candidate {} revision {}",
                identity.0, identity.1
            )));
        }
        previous_identity = Some(identity);
        if item.content_sha256 != item.embedding.input_sha256
            || item.embedding.provider_descriptor_sha256 != descriptor_sha256
            || item.embedding.provider_id != draft.provider.provider_id
            || item.embedding.model_sha256 != draft.provider.model_sha256
            || item.embedding.tokenizer_sha256 != draft.provider.tokenizer_sha256
            || item.embedding.dimensions != draft.provider.dimensions
            || item.embedding.metric != draft.provider.metric
            || item.embedding.quantization != draft.provider.quantization
        {
            return Err(ContractError::Invalid(
                "ANN item embedding does not match index bindings".to_string(),
            ));
        }
        let signature = lsh_signature(&item.embedding.vector, &draft.seed_sha256)?;
        entries.push(StoredEntry {
            candidate_id: item.candidate_id,
            memory_revision: item.memory_revision,
            content_sha256: item.content_sha256,
            vector: item.embedding.vector,
            vector_sha256: item.embedding.vector_sha256,
            signature,
        });
    }

    let mut buckets = BTreeMap::<u64, Vec<u32>>::new();
    for (index, entry) in entries.iter().enumerate() {
        buckets
            .entry(entry.signature)
            .or_default()
            .push(usize_to_u32(index, "ANN entry index")?);
    }
    let entries_sha256 = entries_digest(&entries);
    let buckets_sha256 = buckets_digest(&buckets);
    let mut manifest = AnnIndexManifest {
        schema_version: P1_1B_SCHEMA_VERSION,
        namespace: INDEX_NAMESPACE.to_string(),
        index_id: draft.index_id,
        generation: draft.generation,
        algorithm: AnnAlgorithm::DeterministicLsh64V1,
        provider_id: draft.provider.provider_id,
        provider_descriptor_sha256: descriptor_sha256,
        model_sha256: draft.provider.model_sha256,
        tokenizer_sha256: draft.provider.tokenizer_sha256,
        dimensions: draft.provider.dimensions,
        metric: draft.provider.metric,
        quantization: draft.provider.quantization,
        seed_sha256: draft.seed_sha256,
        item_count: usize_to_u32(entries.len(), "ANN item count")?,
        bucket_count: usize_to_u32(buckets.len(), "ANN bucket count")?,
        entries_sha256,
        buckets_sha256,
        immutable: true,
        local_only: true,
        remote_embedding: P1_1B_REMOTE_EMBEDDING,
        model_download: P1_1B_MODEL_DOWNLOAD,
        network_access: P1_1B_NETWORK_ACCESS,
        runtime_wired: false,
        default_recall_changed: P1_1B_DEFAULT_RECALL_CHANGED,
        context_attachment: P1_1B_CONTEXT_ATTACHMENT,
        physical_send: P1_1B_PHYSICAL_SEND,
        production_authority: P1_1B_PRODUCTION_AUTHORITY,
        manifest_sha256: Digest32::for_bytes(b"uncomputed"),
    };
    manifest.manifest_sha256 = manifest_digest(&manifest);
    let index = LocalAnnIndex {
        manifest,
        entries,
        buckets,
    };
    index.verify()?;
    Ok(index)
}

pub fn reopen_local_ann_index(
    path: &Path,
    expected: &ExpectedAnnIndexBinding,
) -> Result<LocalAnnIndex, ContractError> {
    expected.validate()?;
    let mut file = File::open(path)?;
    let file_bytes = file.metadata()?.len();
    if file_bytes == 0 || file_bytes > MAX_INDEX_FILE_BYTES {
        return Err(ContractError::Corrupt(
            "ANN index file size is outside the bounded range".to_string(),
        ));
    }
    let mut bytes = Vec::with_capacity(
        usize::try_from(file_bytes).map_err(|_| ContractError::Overflow)?,
    );
    file.read_to_end(&mut bytes)?;
    let index = LocalAnnIndex::decode(&bytes)?;
    let manifest = index.manifest();
    if manifest.index_id != expected.index_id
        || manifest.generation != expected.generation
        || manifest.provider_descriptor_sha256 != expected.provider_descriptor_sha256
        || manifest.model_sha256 != expected.model_sha256
        || manifest.tokenizer_sha256 != expected.tokenizer_sha256
        || manifest.dimensions != expected.dimensions
        || expected
            .manifest_sha256
            .is_some_and(|digest| digest != manifest.manifest_sha256)
    {
        return Err(ContractError::Corrupt(
            "reopened ANN index does not match expected immutable binding".to_string(),
        ));
    }
    Ok(index)
}
