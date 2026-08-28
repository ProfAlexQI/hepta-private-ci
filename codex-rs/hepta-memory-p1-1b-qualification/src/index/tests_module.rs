#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedding::EmbeddingProviderKind;
    use crate::embedding::EmbeddingRegistry;
    use crate::embedding::QualificationHashOneHotProvider;

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

    fn build_single_item_index() -> LocalAnnIndex {
        let descriptor = descriptor();
        let mut registry = EmbeddingRegistry::new();
        registry
            .register(Box::new(
                QualificationHashOneHotProvider::new(descriptor.clone()).expect("provider"),
            ))
            .expect("register");
        let embedding = registry
            .embed_batch("qualification-hash-one-hot", &["alpha"])
            .expect("embed")
            .remove(0);
        build_local_ann_index(AnnIndexBuildDraft {
            index_id: "qualification-index".to_string(),
            generation: 1,
            seed_sha256: Digest32::for_bytes(b"seed"),
            provider: descriptor,
            items: vec![AnnIndexItemDraft {
                candidate_id: "memory-alpha".to_string(),
                memory_revision: 1,
                content_sha256: embedding.input_sha256,
                embedding,
            }],
        })
        .expect("index")
    }

    fn encoded_header_prefix(dimensions: u32, item_count: u32, bucket_count: u32) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend_from_slice(INDEX_MAGIC);
        write_u32(&mut output, P1_1B_SCHEMA_VERSION);
        write_string(&mut output, INDEX_NAMESPACE).expect("namespace");
        write_string(&mut output, "qualification-index").expect("index id");
        write_u64(&mut output, 1);
        write_u32(&mut output, AnnAlgorithm::DeterministicLsh64V1.code());
        write_string(&mut output, "qualification-provider").expect("provider id");
        write_digest(&mut output, Digest32::for_bytes(b"provider"));
        write_digest(&mut output, Digest32::for_bytes(b"model"));
        write_digest(&mut output, Digest32::for_bytes(b"tokenizer"));
        write_u32(&mut output, dimensions);
        if !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&dimensions) {
            return output;
        }
        write_u32(&mut output, metric_code(EmbeddingMetric::Cosine));
        write_u32(
            &mut output,
            quantization_code(VectorQuantization::I16Q15Unit),
        );
        write_digest(&mut output, Digest32::for_bytes(b"seed"));
        write_u32(&mut output, item_count);
        write_u32(&mut output, bucket_count);
        output
    }

    #[test]
    fn build_is_input_order_independent() {
        let descriptor = descriptor();
        let mut registry = EmbeddingRegistry::new();
        registry
            .register(Box::new(
                QualificationHashOneHotProvider::new(descriptor.clone()).expect("provider"),
            ))
            .expect("register");
        let embeddings = registry
            .embed_batch("qualification-hash-one-hot", &["alpha", "beta"])
            .expect("embed");
        let first = AnnIndexItemDraft {
            candidate_id: "memory-alpha".to_string(),
            memory_revision: 1,
            content_sha256: embeddings[0].input_sha256,
            embedding: embeddings[0].clone(),
        };
        let second = AnnIndexItemDraft {
            candidate_id: "memory-beta".to_string(),
            memory_revision: 1,
            content_sha256: embeddings[1].input_sha256,
            embedding: embeddings[1].clone(),
        };
        let build = |items| {
            build_local_ann_index(AnnIndexBuildDraft {
                index_id: "qualification-index".to_string(),
                generation: 1,
                seed_sha256: Digest32::for_bytes(b"seed"),
                provider: descriptor.clone(),
                items,
            })
            .expect("index")
        };
        let ordered = build(vec![first.clone(), second.clone()]);
        let reversed = build(vec![second, first]);
        assert_eq!(
            ordered.manifest().manifest_sha256,
            reversed.manifest().manifest_sha256
        );
    }

    #[test]
    fn decode_rejects_dimension_above_bound_before_payload() {
        let bytes = encoded_header_prefix(MAX_EMBEDDING_DIMENSIONS + 1, 1, 1);
        let error = LocalAnnIndex::decode(&bytes).expect_err("dimension bound");
        assert!(error.to_string().contains("pre-allocation bound"));
    }

    #[test]
    fn decode_rejects_bucket_count_above_item_count() {
        let bytes = encoded_header_prefix(64, 1, 2);
        let error = LocalAnnIndex::decode(&bytes).expect_err("bucket bound");
        assert!(error.to_string().contains("count limits"));
    }

    #[test]
    fn probe_signatures_keep_exact_bucket_first() {
        let exact = 0x6a5a_5aa5_1234_5678_u64;
        let probes = ordered_probe_signatures(exact);
        assert_eq!(probes.len(), 65);
        assert_eq!(probes[0], exact);
        assert_eq!(probes.iter().copied().collect::<BTreeSet<_>>().len(), 65);
        assert!(probes[1..].windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn fixed_point_cosine_uses_actual_vector_norm() {
        let mut vector = vec![0_i16; 8];
        vector[0] = 32_605;
        let vector_norm = norm_squared(&vector).expect("norm");
        assert!(norm_is_q15_unit(vector_norm));
        assert_eq!(
            cosine_similarity_ppm(&vector, &vector).expect("cosine"),
            1_000_000
        );
    }

    #[test]
    fn verify_rejects_rehashed_non_unit_vector() {
        let mut index = build_single_item_index();
        let signature = {
            let entry = &mut index.entries[0];
            entry.vector.fill(0);
            entry.vector_sha256 = vector_digest(&entry.vector);
            entry.signature = lsh_signature(&entry.vector, &index.manifest.seed_sha256)
                .expect("signature");
            entry.signature
        };
        index.buckets.clear();
        index.buckets.insert(signature, vec![0]);
        index.manifest.entries_sha256 = entries_digest(&index.entries);
        index.manifest.buckets_sha256 = buckets_digest(&index.buckets);
        index.manifest.bucket_count = 1;
        index.manifest.manifest_sha256 = manifest_digest(&index.manifest);
        assert!(index.verify().is_err());
    }

    #[test]
    fn search_receipt_rejects_non_deterministic_order() {
        let index = build_single_item_index();
        let mut registry = EmbeddingRegistry::new();
        registry
            .register(Box::new(
                QualificationHashOneHotProvider::new(descriptor()).expect("provider"),
            ))
            .expect("register");
        let query = registry
            .embed_batch("qualification-hash-one-hot", &["alpha"])
            .expect("query")
            .remove(0);
        let mut receipt = index.search(&query, 1).expect("search");
        let mut duplicate = receipt.results[0].clone();
        duplicate.memory_revision = 2;
        duplicate.result_sha256 = ann_result_digest(&duplicate);
        receipt.results.push(duplicate);
        receipt.result_count = 2;
        receipt.scanned_candidate_count = 2;
        receipt.receipt_sha256 = ann_search_receipt_digest(&receipt);
        assert!(receipt.validate().is_err());
    }
}
