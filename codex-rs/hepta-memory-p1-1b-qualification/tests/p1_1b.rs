use std::fs;
use std::path::PathBuf;

use hepta_intelligence_p1_1b_qualification::AlphanumericPunctuationTokenizer;
use hepta_intelligence_p1_1b_qualification::AnnIndexBuildDraft;
use hepta_intelligence_p1_1b_qualification::AnnIndexItemDraft;
use hepta_intelligence_p1_1b_qualification::Digest32;
use hepta_intelligence_p1_1b_qualification::EmbeddingMetric;
use hepta_intelligence_p1_1b_qualification::EmbeddingProviderKind;
use hepta_intelligence_p1_1b_qualification::EmbeddingRegistry;
use hepta_intelligence_p1_1b_qualification::ExpectedAnnIndexBinding;
use hepta_intelligence_p1_1b_qualification::LocalEmbeddingDescriptor;
use hepta_intelligence_p1_1b_qualification::LocalSemanticReadiness;
use hepta_intelligence_p1_1b_qualification::LocalSemanticRoute;
use hepta_intelligence_p1_1b_qualification::LocalTokenizerDescriptor;
use hepta_intelligence_p1_1b_qualification::LocalTokenizerRegistry;
use hepta_intelligence_p1_1b_qualification::P1_1B_CALLERS_RATCHET;
use hepta_intelligence_p1_1b_qualification::P1_1B_CONTEXT_ATTACHMENT;
use hepta_intelligence_p1_1b_qualification::P1_1B_DEFAULT_RECALL_CHANGED;
use hepta_intelligence_p1_1b_qualification::P1_1B_EXTERNAL_EFFECTS;
use hepta_intelligence_p1_1b_qualification::P1_1B_FEDERATION_RECALL_CHANGED;
use hepta_intelligence_p1_1b_qualification::P1_1B_MODEL_DOWNLOAD;
use hepta_intelligence_p1_1b_qualification::P1_1B_NETWORK_ACCESS;
use hepta_intelligence_p1_1b_qualification::P1_1B_OPERATOR_ACCEPTANCE;
use hepta_intelligence_p1_1b_qualification::P1_1B_PHYSICAL_SEND;
use hepta_intelligence_p1_1b_qualification::P1_1B_PRODUCT_MODULE_REGISTERED;
use hepta_intelligence_p1_1b_qualification::P1_1B_PRODUCT_WORKSPACE_MEMBER;
use hepta_intelligence_p1_1b_qualification::P1_1B_PRODUCTION_AUTHORITY;
use hepta_intelligence_p1_1b_qualification::P1_1B_PROMOTION;
use hepta_intelligence_p1_1b_qualification::P1_1B_REMOTE_EMBEDDING;
use hepta_intelligence_p1_1b_qualification::P1_1B_WIRED;
use hepta_intelligence_p1_1b_qualification::QualificationHashOneHotProvider;
use hepta_intelligence_p1_1b_qualification::TokenCountMode;
use hepta_intelligence_p1_1b_qualification::TokenizerContract;
use hepta_intelligence_p1_1b_qualification::TokenizerImplementationKind;
use hepta_intelligence_p1_1b_qualification::VectorQuantization;
use hepta_intelligence_p1_1b_qualification::build_local_ann_index;
use hepta_intelligence_p1_1b_qualification::decide_local_semantic_route;
use hepta_intelligence_p1_1b_qualification::reopen_local_ann_index;

fn tokenizer_descriptor() -> LocalTokenizerDescriptor {
    LocalTokenizerDescriptor {
        tokenizer_id: "qualification-tokenizer-v1".to_string(),
        artifact_sha256: Digest32::for_bytes(b"tokenizer-artifact"),
        vocabulary_sha256: Digest32::for_bytes(b"tokenizer-vocabulary"),
        model_compatibility_sha256: Digest32::for_bytes(b"qualification-model"),
        contract: TokenizerContract::AlphanumericPunctuationV1,
        implementation_kind: TokenizerImplementationKind::QualificationReference,
        max_input_bytes: 4096,
        local_execution_only: true,
        remote_execution: false,
        model_download: false,
    }
}

fn embedding_descriptor() -> LocalEmbeddingDescriptor {
    LocalEmbeddingDescriptor {
        provider_id: "qualification-hash-one-hot".to_string(),
        model_id: "qualification-model".to_string(),
        model_sha256: Digest32::for_bytes(b"qualification-model"),
        tokenizer_sha256: tokenizer_descriptor().artifact_sha256,
        dimensions: 64,
        max_batch: 16,
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

fn temp_path(label: &str) -> PathBuf {
    let suffix = Digest32::for_bytes(
        format!(
            "{label}:{}:{:?}",
            std::process::id(),
            std::thread::current().id()
        )
        .as_bytes(),
    )
    .to_hex();
    std::env::temp_dir().join(format!("hepta-p1-1b-{label}-{suffix}.ann"))
}

fn build_index() -> (
    LocalEmbeddingDescriptor,
    EmbeddingRegistry,
    hepta_intelligence_p1_1b_qualification::LocalAnnIndex,
) {
    let descriptor = embedding_descriptor();
    let mut registry = EmbeddingRegistry::new();
    registry
        .register(Box::new(
            QualificationHashOneHotProvider::new(descriptor.clone()).expect("provider"),
        ))
        .expect("register provider");

    let content = ["alpha project", "beta project", "gamma procedure"];
    let embeddings = registry
        .embed_batch("qualification-hash-one-hot", &content)
        .expect("embed documents");
    let items = content
        .iter()
        .zip(embeddings)
        .enumerate()
        .map(|(index, (text, embedding))| AnnIndexItemDraft {
            candidate_id: format!("memory-{}", index + 1),
            memory_revision: 1,
            content_sha256: Digest32::for_bytes(text.as_bytes()),
            embedding,
        })
        .collect();

    let index = build_local_ann_index(AnnIndexBuildDraft {
        index_id: "qualification-local-ann-index".to_string(),
        generation: 1,
        seed_sha256: Digest32::for_bytes(b"deterministic-lsh-seed"),
        provider: descriptor.clone(),
        items,
    })
    .expect("build index");
    (descriptor, registry, index)
}

#[test]
fn tokenizer_registry_produces_exact_and_deterministic_fallback_receipts() {
    let mut registry = LocalTokenizerRegistry::new();
    registry
        .register(Box::new(
            AlphanumericPunctuationTokenizer::new(tokenizer_descriptor()).expect("tokenizer"),
        ))
        .expect("register tokenizer");

    let exact = registry
        .count_or_fallback(Some("qualification-tokenizer-v1"), "hello, 世界!")
        .expect("exact token count");
    assert_eq!(exact.mode, TokenCountMode::ExactLocal);
    assert_eq!(exact.token_count, 4);
    assert!(exact.exact);
    exact.validate().expect("exact receipt");

    let fallback = registry
        .count_or_fallback(Some("missing-tokenizer"), "世界")
        .expect("fallback count");
    assert_eq!(fallback.mode, TokenCountMode::Utf8ByteUpperBound);
    assert_eq!(fallback.token_count, 6);
    assert!(!fallback.exact);
    fallback.validate().expect("fallback receipt");
}

#[test]
fn local_embedding_index_is_create_only_reopen_verified_and_searchable() {
    let (descriptor, registry, index) = build_index();
    let path = temp_path("round-trip");
    let _ = fs::remove_file(&path);

    let write = index.write_create_only(&path).expect("write index");
    write.validate().expect("write receipt");
    assert!(index.write_create_only(&path).is_err());

    let expected = ExpectedAnnIndexBinding {
        index_id: index.manifest().index_id.clone(),
        generation: index.manifest().generation,
        provider_descriptor_sha256: descriptor.descriptor_sha256().expect("descriptor"),
        model_sha256: descriptor.model_sha256,
        tokenizer_sha256: descriptor.tokenizer_sha256,
        dimensions: descriptor.dimensions,
        manifest_sha256: Some(index.manifest().manifest_sha256),
    };
    let reopened = reopen_local_ann_index(&path, &expected).expect("reopen");
    assert_eq!(reopened.item_count(), 3);

    let query = registry
        .embed_batch("qualification-hash-one-hot", &["alpha project"])
        .expect("query embedding")
        .remove(0);
    let receipt = reopened.search(&query, 3).expect("search");
    receipt.validate().expect("search receipt");
    assert!(!receipt.lexical_fallback_required);
    assert_eq!(receipt.results[0].candidate_id, "memory-1");
    assert_eq!(receipt.results[0].similarity_ppm, 1_000_000);

    let bytes = fs::read(&path).expect("read index");
    assert!(
        !bytes
            .windows("alpha project".len())
            .any(|window| window == b"alpha project"),
        "immutable index must not store raw memory content"
    );
    fs::remove_file(path).expect("remove index");
}

#[test]
fn reopen_rejects_tamper_and_binding_drift() {
    let (descriptor, _registry, index) = build_index();
    let path = temp_path("tamper-source");
    let tampered_path = temp_path("tamper-copy");
    let _ = fs::remove_file(&path);
    let _ = fs::remove_file(&tampered_path);
    index.write_create_only(&path).expect("write");

    let expected = ExpectedAnnIndexBinding {
        index_id: index.manifest().index_id.clone(),
        generation: index.manifest().generation,
        provider_descriptor_sha256: descriptor.descriptor_sha256().expect("descriptor"),
        model_sha256: descriptor.model_sha256,
        tokenizer_sha256: descriptor.tokenizer_sha256,
        dimensions: descriptor.dimensions,
        manifest_sha256: Some(index.manifest().manifest_sha256),
    };

    let mut bytes = fs::read(&path).expect("read");
    let last = bytes.len() - 1;
    bytes[last] ^= 0x01;
    fs::write(&tampered_path, bytes).expect("write tamper");
    assert!(reopen_local_ann_index(&tampered_path, &expected).is_err());

    let mut wrong = expected;
    wrong.model_sha256 = Digest32::for_bytes(b"other-model");
    assert!(reopen_local_ann_index(&path, &wrong).is_err());

    fs::remove_file(path).expect("remove");
    fs::remove_file(tampered_path).expect("remove tamper");
}

#[test]
fn semantic_route_fails_to_lexical_until_every_dependency_is_qualified() {
    let blocked = decide_local_semantic_route(&LocalSemanticReadiness {
        tokenizer_registered: true,
        embedding_provider_registered: true,
        index_opened_and_verified: true,
        model_tokenizer_index_bindings_match: true,
        dependencies_executable_qualified: false,
    })
    .expect("route");
    assert_eq!(blocked.route, LocalSemanticRoute::LexicalOnly);
    blocked.validate().expect("blocked receipt");

    let ready = decide_local_semantic_route(&LocalSemanticReadiness {
        tokenizer_registered: true,
        embedding_provider_registered: true,
        index_opened_and_verified: true,
        model_tokenizer_index_bindings_match: true,
        dependencies_executable_qualified: true,
    })
    .expect("route");
    assert_eq!(ready.route, LocalSemanticRoute::ShadowSemanticReady);
    ready.validate().expect("ready receipt");
}

fn assert_false(name: &str, value: bool) {
    assert!(!value, "{name} unexpectedly became true");
}

#[test]
fn source_only_authority_boundary_is_frozen_false() {
    for (name, value) in [
        ("wired", P1_1B_WIRED),
        ("product_workspace_member", P1_1B_PRODUCT_WORKSPACE_MEMBER),
        ("product_module_registered", P1_1B_PRODUCT_MODULE_REGISTERED),
        ("default_recall_changed", P1_1B_DEFAULT_RECALL_CHANGED),
        ("federation_recall_changed", P1_1B_FEDERATION_RECALL_CHANGED),
        ("context_attachment", P1_1B_CONTEXT_ATTACHMENT),
        ("physical_send", P1_1B_PHYSICAL_SEND),
        ("remote_embedding", P1_1B_REMOTE_EMBEDDING),
        ("model_download", P1_1B_MODEL_DOWNLOAD),
        ("network_access", P1_1B_NETWORK_ACCESS),
        ("external_effects", P1_1B_EXTERNAL_EFFECTS),
        ("production_authority", P1_1B_PRODUCTION_AUTHORITY),
        ("operator_acceptance", P1_1B_OPERATOR_ACCEPTANCE),
        ("promotion", P1_1B_PROMOTION),
        ("callers_ratchet", P1_1B_CALLERS_RATCHET),
    ] {
        assert_false(name, value);
    }
}
