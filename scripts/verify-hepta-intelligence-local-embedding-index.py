#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P1.1b."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CRATE = ROOT / "codex-rs/hepta-memory-p1-1b-qualification"
FILES = {
    "cargo": CRATE / "Cargo.toml",
    "lib": CRATE / "src/lib.rs",
    "digest": CRATE / "src/digest.rs",
    "tokenizer": CRATE / "src/tokenizer.rs",
    "embedding": CRATE / "src/embedding.rs",
    "index": CRATE / "src/index.rs",
    "index_types": CRATE / "src/index/types.rs",
    "index_impl": CRATE / "src/index/impl.rs",
    "index_build": CRATE / "src/index/build.rs",
    "index_math": CRATE / "src/index/search_math.rs",
    "index_digests": CRATE / "src/index/digests.rs",
    "index_codec": CRATE / "src/index/codec.rs",
    "index_tests": CRATE / "src/index/tests_module.rs",
    "route": CRATE / "src/route.rs",
    "tests": CRATE / "tests/p1_1b.rs",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_LOCAL_EMBEDDING_INDEX_2026-08-28.md",
    "receipt": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_IMPLEMENTATION_RECEIPT_2026-08-28.json",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json",
    "workflow": ROOT
    / ".github/workflows/hepta-intelligence-local-embedding-index.yml",
    "product_workspace": ROOT / "codex-rs/Cargo.toml",
    "product_retrieval": ROOT / "codex-rs/hepta-memory/src/cognitive_retrieval.rs",
    "p1_1a": ROOT / "codex-rs/hepta-memory/src/shadow_hybrid_retrieval_v2.rs",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    receipt = {
        "schema": "hepta.intelligence.p1.1b.local-embedding-index-source-gate.v1",
        "status": (
            "PASS_P1_1B_LOCAL_EMBEDDING_INDEX_SOURCE_ONLY"
            if not failures
            else "FAIL_P1_1B_LOCAL_EMBEDDING_INDEX_SOURCE_CONTRACT"
        ),
        "scope": "P1_1B_LOCAL_PROVIDER_TOKENIZER_IMMUTABLE_ANN_SOURCE_ONLY",
        "implemented": not failures,
        "wired": False,
        "qualified": False,
        "product_workspace_member": False,
        "product_module_registered": False,
        "default_recall_changed": False,
        "federation_recall_changed": False,
        "context_attachment": False,
        "physical_send": False,
        "remote_embedding": False,
        "model_download": False,
        "network_access": False,
        "external_effects": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "callers_ratchet": False,
        "efficacy_validation": False,
        "checks": checks,
        "failures": failures,
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0 if not failures else 1


def main() -> int:
    checks: dict[str, bool] = {
        f"file.{name}": path.is_file() and path.stat().st_size > 0
        for name, path in FILES.items()
    }
    if not all(checks.values()):
        return emit(checks)

    texts = {name: path.read_text(encoding="utf-8") for name, path in FILES.items()}
    try:
        status = json.loads(texts["status"])
        implementation = json.loads(texts["receipt"])
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["machine_documents.valid_json"] = False
        return emit(checks)
    checks["machine_documents.valid_json"] = True

    checks["crate.isolated_dependency_free_workspace"] = contains_all(
        texts["cargo"],
        [
            'name = "hepta-intelligence-p1-1b-qualification"',
            "[workspace]",
            'unsafe_code = "forbid"',
        ],
    ) and "[dependencies]" not in texts["cargo"]

    checks["crate.not_product_workspace_member"] = (
        "hepta-memory-p1-1b-qualification" not in texts["product_workspace"]
    )

    checks["lib.authority_boundary"] = contains_all(
        texts["lib"],
        [
            "P1_1B_IMPLEMENTED: bool = true",
            "P1_1B_WIRED: bool = false",
            "P1_1B_QUALIFIED: bool = false",
            "P1_1B_PRODUCT_WORKSPACE_MEMBER: bool = false",
            "P1_1B_PRODUCT_MODULE_REGISTERED: bool = false",
            "P1_1B_DEFAULT_RECALL_CHANGED: bool = false",
            "P1_1B_FEDERATION_RECALL_CHANGED: bool = false",
            "P1_1B_CONTEXT_ATTACHMENT: bool = false",
            "P1_1B_PHYSICAL_SEND: bool = false",
            "P1_1B_REMOTE_EMBEDDING: bool = false",
            "P1_1B_MODEL_DOWNLOAD: bool = false",
            "P1_1B_NETWORK_ACCESS: bool = false",
            "P1_1B_EXTERNAL_EFFECTS: bool = false",
            "P1_1B_PRODUCTION_AUTHORITY: bool = false",
            "P1_1B_OPERATOR_ACCEPTANCE: bool = false",
            "P1_1B_PROMOTION: bool = false",
            "P1_1B_CALLERS_RATCHET: bool = false",
        ],
    )

    checks["digest.real_sha256"] = contains_all(
        texts["digest"],
        [
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            "const K: [u32; 64]",
            "framed_digest",
        ],
    )

    checks["tokenizer.registry_and_fallback"] = contains_all(
        texts["tokenizer"],
        [
            "LocalTokenizerRegistry",
            "LocalTokenizerEngine",
            "TokenCountMode::ExactLocal",
            "TokenCountMode::Utf8ByteUpperBound",
            "production_model_compatibility_verified: false",
            "remote_execution: false",
            "model_download: false",
            "tokenizer_unavailable",
        ],
    )

    checks["embedding.local_provider_contract"] = contains_all(
        texts["embedding"],
        [
            "LocalEmbeddingProvider",
            "EmbeddingRegistry",
            "LocalEmbeddingDescriptor",
            "I16Q15Unit",
            "QualificationHashOneHotProvider",
            "remote_execution: false",
            "model_download: false",
            "network_access: false",
            "production_model_verified: false",
        ],
    )

    index_source = "\n".join(
        texts[name]
        for name in (
            "index",
            "index_types",
            "index_impl",
            "index_build",
            "index_math",
            "index_digests",
            "index_codec",
            "index_tests",
        )
    )
    checks["index.immutable_ann_and_reopen"] = contains_all(
        index_source,
        [
            "DeterministicLsh64V1",
            "build_local_ann_index",
            "write_create_only",
            "create_new(true)",
            "reopen_local_ann_index",
            "lsh_signature",
            "cosine_similarity_ppm",
            "manifest_sha256",
            "entries_sha256",
            "buckets_sha256",
            "lexical_fallback_required",
        ],
    )

    checks["route.deterministic_lexical_fallback"] = contains_all(
        texts["route"],
        [
            "decide_local_semantic_route",
            "LocalSemanticRoute::LexicalOnly",
            "SemanticFallbackReason::DependencyUnqualified",
            "SemanticFallbackReason::TokenizerUnavailable",
            "SemanticFallbackReason::EmbeddingProviderUnavailable",
            "SemanticFallbackReason::IndexUnavailable",
            "SemanticFallbackReason::BindingMismatch",
        ],
    )

    checks["tests.coverage"] = contains_all(
        texts["tests"],
        [
            "tokenizer_registry_produces_exact_and_deterministic_fallback_receipts",
            "local_embedding_index_is_create_only_reopen_verified_and_searchable",
            "reopen_rejects_tamper_and_binding_drift",
            "semantic_route_fails_to_lexical_until_every_dependency_is_qualified",
            "source_only_authority_boundary_is_frozen_false",
            "alpha project",
            "assert!(index.write_create_only(&path).is_err())",
        ],
    )

    forbidden = [
        "reqwest",
        "hyper::",
        "ureq",
        "TcpStream",
        "UdpSocket",
        "Command::new",
        "curl ",
        "wget ",
        "production_authority: true",
        "P1_1B_WIRED: bool = true",
        "P1_1B_DEFAULT_RECALL_CHANGED: bool = true",
    ]
    implementation_source = "\n".join(
        texts[name]
        for name in (
            "lib",
            "digest",
            "tokenizer",
            "embedding",
            "index",
            "index_types",
            "index_impl",
            "index_build",
            "index_math",
            "index_digests",
            "index_codec",
            "index_tests",
            "route",
            "tests",
        )
    )
    checks["source.no_network_or_authority_path"] = not any(
        marker in implementation_source for marker in forbidden
    )

    checks["product.retrieval_unchanged"] = (
        "p1_1b" not in texts["product_retrieval"].lower()
        and "local_embedding_index" not in texts["product_retrieval"].lower()
    )
    checks["p1_1a.contract_unchanged"] = "p1_1b" not in texts["p1_1a"].lower()

    current = status.get("current_tranche", {})
    claims = current.get("claims", {})
    dependency = status.get("dependency", {})
    checks["status.p1_1b_source_only"] = (
        current.get("id") == "P1.1b"
        and current.get("qualified") is False
        and claims.get("local_tokenizer_registry") is True
        and claims.get("local_embedding_provider_contract") is True
        and claims.get("immutable_ann_index") is True
        and claims.get("runtime_wired") is False
        and claims.get("default_recall_changed") is False
        and claims.get("production_authority") is False
    )
    checks["status.p1_1a_dependency_unqualified"] = (
        dependency.get("id") == "P1.1a"
        and dependency.get("pull_request") == 28
        and dependency.get("qualified") is False
    )

    checks["receipt.authority_false"] = all(
        implementation.get("authority", {}).get(key) is False
        for key in (
            "wired",
            "product_workspace_member",
            "product_module_registered",
            "default_recall_changed",
            "federation_recall_changed",
            "context_attachment",
            "physical_send",
            "remote_embedding",
            "model_download",
            "network_access",
            "external_effects",
            "production_authority",
            "operator_acceptance",
            "promotion",
            "callers_ratchet",
        )
    )

    checks["plan.explicit_boundary"] = contains_all(
        texts["plan"],
        [
            "SOURCE_ONLY",
            "ACTIVATION_BLOCKED",
            "deterministic_lsh64_v1",
            "utf8_byte_upper_bound",
            "product_workspace_member=false",
            "default_recall_changed=false",
            "remote_embedding=false",
            "model_download=false",
            "production_authority=false",
            "P1.1c",
        ],
    )

    checks["workflow.exact_isolated_commands"] = contains_all(
        texts["workflow"],
        [
            'toolchain: "1.95.0"',
            "verify-hepta-intelligence-local-embedding-index.py",
            "hepta-memory-p1-1b-qualification/Cargo.toml",
            "cargo fmt --manifest-path",
            "cargo test --manifest-path",
            "cargo check --manifest-path",
            "cargo clippy --manifest-path",
            "--all-targets -- -D warnings",
        ],
    )

    return emit(checks)


if __name__ == "__main__":
    sys.exit(main())
