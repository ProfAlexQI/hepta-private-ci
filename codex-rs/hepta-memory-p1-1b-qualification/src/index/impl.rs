impl LocalAnnIndex {
    pub fn manifest(&self) -> &AnnIndexManifest {
        &self.manifest
    }

    pub fn item_count(&self) -> usize {
        self.entries.len()
    }

    pub fn verify(&self) -> Result<(), ContractError> {
        self.manifest.validate()?;
        if usize::try_from(self.manifest.item_count).ok() != Some(self.entries.len())
            || usize::try_from(self.manifest.bucket_count).ok() != Some(self.buckets.len())
        {
            return Err(ContractError::Corrupt(
                "ANN manifest counts do not match index data".to_string(),
            ));
        }

        let mut identities = BTreeSet::new();
        for entry in &self.entries {
            validate_id(&entry.candidate_id, "stored ANN candidate id")?;
            let entry_norm_squared = norm_squared(&entry.vector)?;
            if entry.memory_revision == 0
                || usize::try_from(self.manifest.dimensions).ok() != Some(entry.vector.len())
                || entry.vector_sha256 != vector_digest(&entry.vector)
                || !norm_is_q15_unit(entry_norm_squared)
                || entry.signature
                    != lsh_signature(&entry.vector, &self.manifest.seed_sha256)?
            {
                return Err(ContractError::Corrupt(
                    "stored ANN entry failed vector, norm, or signature verification".to_string(),
                ));
            }
            if !identities.insert((entry.candidate_id.clone(), entry.memory_revision)) {
                return Err(ContractError::Corrupt(
                    "duplicate ANN candidate identity".to_string(),
                ));
            }
        }

        let mut seen_indices = BTreeSet::new();
        for (signature, indices) in &self.buckets {
            if indices.is_empty() {
                return Err(ContractError::Corrupt(
                    "ANN bucket must not be empty".to_string(),
                ));
            }
            let mut previous = None;
            for index in indices {
                let index_usize = usize::try_from(*index)
                    .map_err(|_| ContractError::Corrupt("ANN entry index overflow".to_string()))?;
                let entry = self.entries.get(index_usize).ok_or_else(|| {
                    ContractError::Corrupt("ANN bucket references a missing entry".to_string())
                })?;
                if entry.signature != *signature
                    || previous.is_some_and(|old| old >= *index)
                    || !seen_indices.insert(*index)
                {
                    return Err(ContractError::Corrupt(
                        "ANN bucket membership or ordering is invalid".to_string(),
                    ));
                }
                previous = Some(*index);
            }
        }
        if seen_indices.len() != self.entries.len() {
            return Err(ContractError::Corrupt(
                "ANN buckets do not cover every entry exactly once".to_string(),
            ));
        }
        if self.manifest.entries_sha256 != entries_digest(&self.entries)
            || self.manifest.buckets_sha256 != buckets_digest(&self.buckets)
        {
            return Err(ContractError::Corrupt(
                "ANN entries or buckets digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    pub fn write_create_only(&self, path: &Path) -> Result<IndexWriteReceipt, ContractError> {
        self.verify()?;
        let bytes = self.encode()?;
        let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
        file.write_all(&bytes)?;
        file.sync_all()?;

        let mut receipt = IndexWriteReceipt {
            index_id: self.manifest.index_id.clone(),
            generation: self.manifest.generation,
            manifest_sha256: self.manifest.manifest_sha256,
            file_sha256: Digest32::for_bytes(&bytes),
            file_bytes: usize_to_u64(bytes.len(), "ANN index file bytes")?,
            create_only: true,
            immutable: true,
            local_only: true,
            remote_embedding: P1_1B_REMOTE_EMBEDDING,
            model_download: P1_1B_MODEL_DOWNLOAD,
            network_access: P1_1B_NETWORK_ACCESS,
            production_authority: P1_1B_PRODUCTION_AUTHORITY,
            receipt_sha256: Digest32::for_bytes(b"uncomputed"),
        };
        receipt.receipt_sha256 = index_write_receipt_digest(&receipt);
        receipt.validate()?;
        Ok(receipt)
    }

    pub fn search(
        &self,
        query: &EmbeddedVector,
        max_results: usize,
    ) -> Result<AnnSearchReceipt, ContractError> {
        self.verify()?;
        query.validate()?;
        if max_results == 0 || max_results > MAX_SEARCH_RESULTS {
            return Err(ContractError::Limit {
                label: "ANN search results",
                max: MAX_SEARCH_RESULTS,
            });
        }
        if query.provider_descriptor_sha256 != self.manifest.provider_descriptor_sha256
            || query.model_sha256 != self.manifest.model_sha256
            || query.tokenizer_sha256 != self.manifest.tokenizer_sha256
            || query.dimensions != self.manifest.dimensions
            || query.metric != self.manifest.metric
            || query.quantization != self.manifest.quantization
        {
            return Err(ContractError::Invalid(
                "query vector does not match immutable ANN bindings".to_string(),
            ));
        }

        let query_signature = lsh_signature(&query.vector, &self.manifest.seed_sha256)?;
        let visited_signatures = ordered_probe_signatures(query_signature);
        let mut visited_bucket_count = 0_usize;
        let mut candidate_indices = BTreeSet::new();
        for signature in &visited_signatures {
            if let Some(indices) = self.buckets.get(signature) {
                visited_bucket_count = visited_bucket_count
                    .checked_add(1)
                    .ok_or(ContractError::Overflow)?;
                for index in indices {
                    candidate_indices.insert(*index);
                    if candidate_indices.len() >= MAX_SEARCH_CANDIDATES {
                        break;
                    }
                }
            }
            if candidate_indices.len() >= MAX_SEARCH_CANDIDATES {
                break;
            }
        }

        let scanned_candidate_count = candidate_indices.len();
        let mut results = Vec::with_capacity(scanned_candidate_count);
        for index in candidate_indices {
            let entry = self
                .entries
                .get(usize::try_from(index).map_err(|_| ContractError::Overflow)?)
                .ok_or_else(|| {
                    ContractError::Corrupt("ANN search candidate index is missing".to_string())
                })?;
            let similarity_ppm = cosine_similarity_ppm(&query.vector, &entry.vector)?;
            let mut result = AnnSearchResult {
                candidate_id: entry.candidate_id.clone(),
                memory_revision: entry.memory_revision,
                content_sha256: entry.content_sha256,
                similarity_ppm,
                vector_sha256: entry.vector_sha256,
                result_sha256: Digest32::for_bytes(b"uncomputed"),
            };
            result.result_sha256 = ann_result_digest(&result);
            results.push(result);
        }

        results.sort_by(|left, right| {
            right
                .similarity_ppm
                .cmp(&left.similarity_ppm)
                .then_with(|| left.candidate_id.cmp(&right.candidate_id))
                .then_with(|| right.memory_revision.cmp(&left.memory_revision))
        });
        results.truncate(max_results);

        let lexical_fallback_required = results.is_empty();
        let mut receipt = AnnSearchReceipt {
            index_id: self.manifest.index_id.clone(),
            generation: self.manifest.generation,
            manifest_sha256: self.manifest.manifest_sha256,
            query_vector_sha256: query.vector_sha256,
            query_signature,
            visited_bucket_count: usize_to_u32(
                visited_bucket_count,
                "visited ANN bucket count",
            )?,
            scanned_candidate_count: usize_to_u32(
                scanned_candidate_count,
                "scanned ANN candidate count",
            )?,
            result_count: usize_to_u32(results.len(), "ANN result count")?,
            results,
            ann_search_executed: true,
            lexical_fallback_required,
            fallback_reason: lexical_fallback_required.then_some("ann_bucket_empty".to_string()),
            runtime_wired: false,
            default_recall_changed: P1_1B_DEFAULT_RECALL_CHANGED,
            federation_recall_changed: P1_1B_FEDERATION_RECALL_CHANGED,
            context_attachment: P1_1B_CONTEXT_ATTACHMENT,
            physical_send: P1_1B_PHYSICAL_SEND,
            remote_embedding: P1_1B_REMOTE_EMBEDDING,
            model_download: P1_1B_MODEL_DOWNLOAD,
            network_access: P1_1B_NETWORK_ACCESS,
            external_effects: P1_1B_EXTERNAL_EFFECTS,
            production_authority: P1_1B_PRODUCTION_AUTHORITY,
            operator_acceptance: P1_1B_OPERATOR_ACCEPTANCE,
            promotion: P1_1B_PROMOTION,
            callers_ratchet: P1_1B_CALLERS_RATCHET,
            receipt_sha256: Digest32::for_bytes(b"uncomputed"),
        };
        receipt.receipt_sha256 = ann_search_receipt_digest(&receipt);
        receipt.validate()?;
        Ok(receipt)
    }

    fn encode(&self) -> Result<Vec<u8>, ContractError> {
        self.verify()?;
        let mut output = Vec::new();
        output.extend_from_slice(INDEX_MAGIC);
        write_u32(&mut output, self.manifest.schema_version);
        write_string(&mut output, &self.manifest.namespace)?;
        write_string(&mut output, &self.manifest.index_id)?;
        write_u64(&mut output, self.manifest.generation);
        write_u32(&mut output, self.manifest.algorithm.code());
        write_string(&mut output, &self.manifest.provider_id)?;
        write_digest(&mut output, self.manifest.provider_descriptor_sha256);
        write_digest(&mut output, self.manifest.model_sha256);
        write_digest(&mut output, self.manifest.tokenizer_sha256);
        write_u32(&mut output, self.manifest.dimensions);
        write_u32(&mut output, metric_code(self.manifest.metric));
        write_u32(&mut output, quantization_code(self.manifest.quantization));
        write_digest(&mut output, self.manifest.seed_sha256);
        write_u32(&mut output, self.manifest.item_count);
        write_u32(&mut output, self.manifest.bucket_count);
        write_digest(&mut output, self.manifest.entries_sha256);
        write_digest(&mut output, self.manifest.buckets_sha256);
        write_digest(&mut output, self.manifest.manifest_sha256);

        for entry in &self.entries {
            write_string(&mut output, &entry.candidate_id)?;
            write_u64(&mut output, entry.memory_revision);
            write_digest(&mut output, entry.content_sha256);
            write_digest(&mut output, entry.vector_sha256);
            write_u64(&mut output, entry.signature);
            write_u32(
                &mut output,
                usize_to_u32(entry.vector.len(), "ANN vector dimensions")?,
            );
            for value in &entry.vector {
                output.extend_from_slice(&value.to_be_bytes());
            }
        }
        for (signature, indices) in &self.buckets {
            write_u64(&mut output, *signature);
            write_u32(
                &mut output,
                usize_to_u32(indices.len(), "ANN bucket entry count")?,
            );
            for index in indices {
                write_u32(&mut output, *index);
            }
        }
        Ok(output)
    }

    fn decode(bytes: &[u8]) -> Result<Self, ContractError> {
        let mut cursor = ByteCursor::new(bytes);
        if cursor.take(INDEX_MAGIC.len())? != INDEX_MAGIC {
            return Err(ContractError::Corrupt(
                "local ANN file magic mismatch".to_string(),
            ));
        }
        let schema_version = cursor.read_u32()?;
        let namespace = cursor.read_string()?;
        let index_id = cursor.read_string()?;
        let generation = cursor.read_u64()?;
        let algorithm = AnnAlgorithm::from_code(cursor.read_u32()?)?;
        let provider_id = cursor.read_string()?;
        let provider_descriptor_sha256 = cursor.read_digest()?;
        let model_sha256 = cursor.read_digest()?;
        let tokenizer_sha256 = cursor.read_digest()?;
        let dimensions = cursor.read_u32()?;
        if !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&dimensions) {
            return Err(ContractError::Corrupt(
                "ANN file dimensions exceed the pre-allocation bound".to_string(),
            ));
        }
        let metric = metric_from_code(cursor.read_u32()?)?;
        let quantization = quantization_from_code(cursor.read_u32()?)?;
        let seed_sha256 = cursor.read_digest()?;
        let item_count = cursor.read_u32()?;
        let bucket_count = cursor.read_u32()?;
        let item_count_usize =
            usize::try_from(item_count).map_err(|_| ContractError::Overflow)?;
        let bucket_count_usize =
            usize::try_from(bucket_count).map_err(|_| ContractError::Overflow)?;
        if item_count == 0
            || item_count_usize > MAX_INDEX_ITEMS
            || bucket_count == 0
            || bucket_count > item_count
            || bucket_count_usize > MAX_INDEX_ITEMS
        {
            return Err(ContractError::Corrupt(
                "local ANN file count limits are invalid".to_string(),
            ));
        }
        let entries_sha256 = cursor.read_digest()?;
        let buckets_sha256 = cursor.read_digest()?;
        let manifest_sha256 = cursor.read_digest()?;

        let mut entries = Vec::with_capacity(item_count_usize);
        for _ in 0..item_count {
            let candidate_id = cursor.read_string()?;
            let memory_revision = cursor.read_u64()?;
            let content_sha256 = cursor.read_digest()?;
            let vector_sha256 = cursor.read_digest()?;
            let signature = cursor.read_u64()?;
            let vector_len = cursor.read_u32()?;
            if vector_len != dimensions {
                return Err(ContractError::Corrupt(
                    "ANN file vector dimension mismatch".to_string(),
                ));
            }
            let vector_len_usize =
                usize::try_from(vector_len).map_err(|_| ContractError::Overflow)?;
            let mut vector = Vec::with_capacity(vector_len_usize);
            for _ in 0..vector_len {
                vector.push(cursor.read_i16()?);
            }
            entries.push(StoredEntry {
                candidate_id,
                memory_revision,
                content_sha256,
                vector,
                vector_sha256,
                signature,
            });
        }

        let mut buckets = BTreeMap::new();
        for _ in 0..bucket_count {
            let signature = cursor.read_u64()?;
            let count = cursor.read_u32()?;
            if count == 0 || count > item_count {
                return Err(ContractError::Corrupt(
                    "ANN file bucket count is invalid".to_string(),
                ));
            }
            let mut indices = Vec::with_capacity(
                usize::try_from(count).map_err(|_| ContractError::Overflow)?,
            );
            for _ in 0..count {
                indices.push(cursor.read_u32()?);
            }
            if buckets.insert(signature, indices).is_some() {
                return Err(ContractError::Corrupt(
                    "ANN file contains duplicate bucket signatures".to_string(),
                ));
            }
        }
        if !cursor.is_finished() {
            return Err(ContractError::Corrupt(
                "ANN file contains trailing bytes".to_string(),
            ));
        }

        let index = Self {
            manifest: AnnIndexManifest {
                schema_version,
                namespace,
                index_id,
                generation,
                algorithm,
                provider_id,
                provider_descriptor_sha256,
                model_sha256,
                tokenizer_sha256,
                dimensions,
                metric,
                quantization,
                seed_sha256,
                item_count,
                bucket_count,
                entries_sha256,
                buckets_sha256,
                immutable: true,
                local_only: true,
                remote_embedding: false,
                model_download: false,
                network_access: false,
                runtime_wired: false,
                default_recall_changed: false,
                context_attachment: false,
                physical_send: false,
                production_authority: false,
                manifest_sha256,
            },
            entries,
            buckets,
        };
        index.verify()?;
        Ok(index)
    }
}
