#!/usr/bin/env python3
"""Fail-closed source gate for Hepta Intelligence P1.1b hardening."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CRATE = ROOT / "codex-rs/hepta-memory-p1-1b-qualification"
FILES = {
    "cargo": CRATE / "Cargo.toml",
    "lib": CRATE / "src/lib.rs",
    "tokenizer": CRATE / "src/tokenizer.rs",
    "embedding": CRATE / "src/embedding.rs",
    "index": CRATE / "src/index.rs",
    "index_types": CRATE / "src/index/types.rs",
    "index_impl": CRATE / "src/index/impl.rs",
    "index_math": CRATE / "src/index/search_math.rs",
    "index_tests": CRATE / "src/index/tests_module.rs",
    "route": CRATE / "src/route.rs",
    "plan": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_HARDENING_2026-08-28.md",
    "receipt": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_HARDENING_RECEIPT_2026-08-28.json",
    "status": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_HARDENING_STATUS_2026-08-28.json",
    "product_workspace": ROOT / "codex-rs/Cargo.toml",
    "product_retrieval": ROOT / "codex-rs/hepta-memory/src/cognitive_retrieval.rs",
}


def contains_all(text: str, markers: list[str]) -> bool:
    return all(marker in text for marker in markers)


def emit(checks: dict[str, bool]) -> int:
    failures = sorted(name for name, passed in checks.items() if not passed)
    output = {
        "schema": "hepta.intelligence.p1_1b.hardening_source_gate.v1",
        "status": (
            "PASS_P1_1B_HARDENING_SOURCE_ONLY"
            if not failures
            else "FAIL_P1_1B_HARDENING_SOURCE_CONTRACT"
        ),
        "implemented": not failures,
        "wired": False,
        "qualified": False,
        "default_recall_changed": False,
        "federation_recall_changed": False,
        "context_attachment": False,
        "physical_send": False,
        "external_effects": False,
        "production_authority": False,
        "checks": checks,
        "failures": failures,
    }
    print(json.dumps(output, indent=2, sort_keys=True))
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
        receipt = json.loads(texts["receipt"])
        status = json.loads(texts["status"])
    except (OSError, UnicodeError, json.JSONDecodeError):
        checks["machine_documents.valid_json"] = False
        return emit(checks)
    checks["machine_documents.valid_json"] = True

    checks["crate.remains_isolated_dependency_free"] = contains_all(
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

    checks["authority.frozen_false"] = contains_all(
        texts["lib"],
        [
            "P1_1B_WIRED: bool = false",
            "P1_1B_QUALIFIED: bool = false",
            "P1_1B_PRODUCT_WORKSPACE_MEMBER: bool = false",
            "P1_1B_PRODUCT_MODULE_REGISTERED: bool = false",
            "P1_1B_DEFAULT_RECALL_CHANGED: bool = false",
            "P1_1B_FEDERATION_RECALL_CHANGED: bool = false",
            "P1_1B_CONTEXT_ATTACHMENT: bool = false",
            "P1_1B_PHYSICAL_SEND: bool = false",
            "P1_1B_EXTERNAL_EFFECTS: bool = false",
            "P1_1B_PRODUCTION_AUTHORITY: bool = false",
        ],
    )

    checks["tokenizer.fallback_is_unbound"] = contains_all(
        texts["tokenizer"],
        [
            "fallback token receipt must be an unbound UTF-8 byte upper bound",
            "self.tokenizer_descriptor_sha256.is_some()",
            "self.tokenizer_artifact_sha256.is_some()",
            "self.tokenizer_vocabulary_sha256.is_some()",
            "self.model_compatibility_sha256.is_some()",
            "fallback_receipt_rejects_injected_tokenizer_bindings",
        ],
    )

    checks["embedding.norm_contract_hardened"] = contains_all(
        texts["embedding"],
        [
            "Q15_NORM_TOLERANCE_PPM: u64 = 10_000",
            "MAX_EMBEDDING_DIMENSIONS",
            "pub(crate) fn norm_is_q15_unit",
            "provider_rejects_non_unit_vector",
            "descriptor_rejects_remote_download_or_dimension_overflow",
        ],
    )

    index_source = "\n".join(
        texts[name]
        for name in ("index", "index_types", "index_impl", "index_math", "index_tests")
    )
    checks["index.decode_bounds_before_allocation"] = contains_all(
        index_source,
        [
            "ANN file dimensions exceed the pre-allocation bound",
            "bucket_count > item_count",
            "bucket_count_usize > MAX_INDEX_ITEMS",
            "decode_rejects_dimension_above_bound_before_payload",
            "decode_rejects_bucket_count_above_item_count",
        ],
    )
    checks["index.vector_and_cosine_hardening"] = contains_all(
        index_source,
        [
            "norm_is_q15_unit(entry_norm_squared)",
            "integer_sqrt_u128",
            "fixed_point_cosine_uses_actual_vector_norm",
            "verify_rejects_rehashed_non_unit_vector",
        ],
    )
    checks["index.exact_probe_priority"] = contains_all(
        index_source,
        [
            "ordered_probe_signatures",
            "signatures.push(query_signature)",
            "probe_signatures_keep_exact_bucket_first",
        ],
    )
    checks["search_receipt.bounded_ordered_unique"] = contains_all(
        texts["index_types"] + "\n" + texts["index_tests"],
        [
            "result_count > MAX_SEARCH_RESULTS",
            "scanned_candidate_count > MAX_SEARCH_CANDIDATES",
            "duplicate ANN search result identity",
            "ANN search results are not in deterministic order",
            "search_receipt_rejects_non_deterministic_order",
        ],
    )

    checks["route.receipt_recomputed"] = contains_all(
        texts["route"],
        [
            "expected_semantic_fallback",
            "semantic route receipt is inconsistent with its readiness fields",
            "route_receipt_rejects_inconsistent_readiness",
        ],
    )

    checks["plan.explicit_source_only_boundary"] = contains_all(
        texts["plan"],
        [
            "SOURCE_ONLY",
            "QUALIFICATION_PENDING",
            "ACTIVATION_BLOCKED",
            "exact-bucket-first",
            "10_000 ppm",
            "wired=false",
            "production_authority=false",
            "P1.1c remains blocked",
        ],
    )

    hardening = receipt.get("hardening", {})
    authority = receipt.get("authority", {})
    checks["receipt.hardening_claims"] = (
        receipt.get("implemented") is True
        and receipt.get("qualified") is False
        and hardening.get("decode_dimension_preallocation_bound") is True
        and hardening.get("bucket_count_bound_to_item_count") is True
        and hardening.get("exact_bucket_priority") is True
        and hardening.get("stored_vector_q15_reverification") is True
        and hardening.get("actual_norm_fixed_point_cosine") is True
        and hardening.get("q15_norm_tolerance_ppm") == 10000
        and hardening.get("fallback_binding_absence") is True
        and hardening.get("semantic_route_recomputation") is True
        and hardening.get("negative_regressions_added") is True
    )
    checks["receipt.authority_false"] = all(
        authority.get(key) is False
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

    current = status.get("stack", {}).get("current", {})
    next_tranche = status.get("stack", {}).get("next", {})
    checks["status.hardening_current_p1_1c_blocked"] = (
        current.get("id") == "P1.1b-hardening"
        and current.get("qualified") is False
        and next_tranche.get("id") == "P1.1c"
        and next_tranche.get("status") == "blocked"
    )

    checks["product.retrieval_not_wired"] = (
        "p1_1b" not in texts["product_retrieval"].lower()
        and "local_embedding_index" not in texts["product_retrieval"].lower()
    )

    forbidden = [
        "reqwest",
        "hyper::",
        "ureq",
        "TcpStream",
        "UdpSocket",
        "Command::new",
        "production_authority: true",
        "P1_1B_WIRED: bool = true",
        "P1_1B_DEFAULT_RECALL_CHANGED: bool = true",
    ]
    implementation_source = "\n".join(
        texts[name]
        for name in (
            "tokenizer",
            "embedding",
            "index",
            "index_types",
            "index_impl",
            "index_math",
            "index_tests",
            "route",
        )
    )
    checks["source.no_network_or_authority_path"] = not any(
        marker in implementation_source for marker in forbidden
    )

    return emit(checks)


if __name__ == "__main__":
    sys.exit(main())
