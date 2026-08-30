#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CONTRACTS = ROOT / "codex-rs/hepta-contracts"
P02 = ROOT / "codex-rs/hepta-authbus-qualification"
P03 = ROOT / "codex-rs/hepta-authbus-p0-3-qualification"
P13 = ROOT / "codex-rs/hepta-authbus-p1-3-qualification"
DOCS = ROOT / "docs/hepta-vnext/authbus"

registry = (CONTRACTS / "src/quota_registry.rs").read_text(encoding="utf-8")
contracts_lib = (CONTRACTS / "src/lib.rs").read_text(encoding="utf-8")
b4 = (CONTRACTS / "src/authbus_b4.rs").read_text(encoding="utf-8")
p02_model = (P02 / "src/model.rs").read_text(encoding="utf-8")
p03_scheduler = (P03 / "src/scheduler.rs").read_text(encoding="utf-8")
p03_tests = (P03 / "tests/p0_3.rs").read_text(encoding="utf-8")
p13_manifest = (P13 / "Cargo.toml").read_text(encoding="utf-8")
p13_tests = (P13 / "tests/p1_3.rs").read_text(encoding="utf-8")
status = json.loads(
    (DOCS / "AUTHBUS_P1_3_IMPLEMENTATION_STATUS_2026-08-29.json").read_text(
        encoding="utf-8"
    )
)

dimensions = (
    "request_count",
    "rpm",
    "tpm",
    "concurrency",
    "day_budget",
    "context",
)
for index, dimension in enumerate(dimensions):
    assert f'canonical_key: "{dimension}"' in registry, dimension
    assert f"ordinal: {index}" in registry, dimension

for token in (
    "wire_key:",
    "sqlite_limit_column:",
    "sqlite_reserved_column:",
    "sqlite_used_column:",
    "receipt_key:",
    "metric_key:",
    "AUTHBUS_QUOTA_REGISTRY_SHA256",
    "LegacyRequestCountPolicy::RejectMissing",
    "LegacyRequestCountPolicy::AssumeOnePerPermit",
    "QuotaProjectionError::LossyLegacyDowngrade",
):
    assert token in registry, token

assert (
    "dfcab028e1a135a0895b3f9eddec9f5f99cf5f392701b98ad14180058a284bf1"
    in registry
)
assert "mod quota_registry;" in contracts_lib
assert "pub use quota_registry::CanonicalQuotaVector;" in contracts_lib
assert "pub use quota_registry::CanonicalQuotaLimits;" in contracts_lib
assert "pub use quota_registry::AUTHBUS_QUOTA_DIMENSIONS;" in contracts_lib

assert "pub struct CanonicalQuotaVector" not in p03_scheduler
assert "pub struct CanonicalQuotaLimits" not in p03_scheduler
assert (
    "pub use codex_hepta_contracts::CanonicalQuotaLimits;" in p03_scheduler
)
assert (
    "pub use codex_hepta_contracts::CanonicalQuotaVector;" in p03_scheduler
)
assert "LegacyRequestCountPolicy::RejectMissing" in p03_tests

for source in (b4, p02_model):
    assert "try_into_canonical" in source
    assert "try_from_canonical" in source
    assert "LegacyRequestCountPolicy" in source
    assert "QuotaProjectionError" in source

assert 'name = "codex-hepta-authbus-p1-3-qualification"' in p13_manifest
assert "default = []" in p13_manifest
assert "p1-3-qualification" in p13_manifest
assert "descriptor_registry_owns_every_projection_surface" in p13_tests
assert "p0_3_scheduler_reexports_the_contract_owned_type" in p13_tests
assert "p0_2_storage_projection_requires_the_same_explicit_policy" in p13_tests

assert status["schema"] == "hepta.authbus.p1.3.implementation-status.v1"
assert status["implemented"] is True
assert status["qualified"] is False
assert status["qualification_only"] is True
assert status["wired"] is False
assert status["dimensions"] == 6
assert status["projection_surfaces"] == 6
assert status["p0_3_duplicate_removed"] is True
assert status["legacy_adapter_count"] == 2
assert status["missing_request_count_policy_explicit"] is True
assert status["lossy_downgrade_rejected"] is True

for field in (
    "authority",
    "effect_authority",
    "production_caller",
    "production_writer",
    "operator_acceptance",
    "promotion",
    "g5_allowed",
    "execute_allowed",
    "listener_enabled",
    "provider_call_enabled",
    "openbao_enabled",
    "parent_workspace_wired",
    "private_key_storage",
    "raw_signature_storage",
    "secret_storage",
):
    assert status[field] is False, field

print(
    json.dumps(
        {
            "claim": "PASS_AUTHBUS_P1_3_SOURCE_ONLY",
            "implemented": True,
            "qualified": False,
            "qualification_only": True,
            "wired": False,
            "dimensions": 6,
            "projection_surfaces": 6,
            "p0_3_duplicate_removed": True,
            "legacy_adapter_count": 2,
            "authority": False,
            "effect_authority": False,
            "production_caller": False,
            "production_writer": False,
            "listener_enabled": False,
            "provider_call_enabled": False,
            "openbao_enabled": False,
        },
        sort_keys=True,
        separators=(",", ":"),
    )
)


# Semantic B2 closure: source registry binding, explicit unknown/absence,
# deterministic rounding, lifecycle and canonical six-dimensional wire shape.
_semantic_root = Path(__file__).resolve().parents[1]
_semantic_registry = (_semantic_root / "codex-rs/hepta-contracts/src/quota_registry.rs").read_text(encoding="utf-8")
_semantic_b2 = (_semantic_root / "codex-rs/hepta-contracts/src/authbus_b2.rs").read_text(encoding="utf-8")
_semantic_contract = (_semantic_root / "docs/hepta-vnext/dropbox-current-2026-08-27/root/AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml").read_text(encoding="utf-8")
_semantic_canonical = (_semantic_root / "docs/hepta-vnext/dropbox-current-2026-08-27/root/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml").read_text(encoding="utf-8")
assert 'pub struct UsageVector' in _semantic_registry
assert 'pub enum QuotaQuantity' in _semantic_registry
assert 'ExplicitUnknown' in _semantic_registry
assert 'NotDeclared' in _semantic_registry
assert 'pub enum QuotaDimensionLifecycle' in _semantic_registry
assert 'pub enum QuotaRounding' in _semantic_registry
assert 'integer_round_up_before_hold' in _semantic_registry
assert 'integer_exact_on_finalize' in _semantic_registry
assert 'OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry' in _semantic_registry
assert 'cda83c4776d4c2b3c2851474e476e775d6ca26fa815373083aac47fdfd0c89f5' in _semantic_registry
assert 'OpenClaw/AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml#/execution_closure_v1_3/usage_vector' in _semantic_registry
assert 'generated_domain_projection:AUTHBUS.11-v1.3:usage_vector' in _semantic_registry
assert 'AUTHBUS.11-v1.3' in _semantic_registry
assert 'pub struct QuotaReservationV1_3' in _semantic_b2
assert 'Decode-only four-dimensional quota hold retained for compatibility.' in _semantic_b2
assert 'absent_dimension: not_declared' in _semantic_contract
assert 'unknown_representation: explicit_unknown' in _semantic_contract
assert 'rounding: integer_round_up_before_hold_and_integer_exact_on_finalize' in _semantic_contract
assert 'mixed_vector_scalar_encoding: forbidden' in _semantic_contract
assert 'status: REQUIRED_AT_B2' in _semantic_canonical
assert 'projection_must_record: [source_registry_ref, source_registry_digest, projection_transform]' in _semantic_canonical
