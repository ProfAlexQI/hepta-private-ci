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
}
