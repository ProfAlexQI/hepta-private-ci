#!/usr/bin/env python3
"""Fail-closed verifier for real cross-owner operation wiring."""

from __future__ import annotations

import json
import pathlib
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
ARCHITECTURE = ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json"
LEDGER = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V1.json"


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_CROSS_OWNER_OPERATION_WIRING: {message}")


def read(relative: str) -> str:
    path = ROOT / relative
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {relative}: {error}")


def load_json(relative: str) -> dict[str, Any]:
    path = ROOT / relative

    def pairs_hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                fail(f"duplicate JSON key {key!r} in {relative}")
            result[key] = value
        return result

    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs_hook)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {relative}: {error}")
    if not isinstance(value, dict):
        fail(f"{relative} must contain one JSON object")
    return value


def require_markers(relative: str, markers: tuple[str, ...]) -> str:
    source = read(relative)
    for marker in markers:
        if marker not in source:
            fail(f"{relative} is missing {marker!r}")
    return source


def require_order(relative: str, markers: tuple[str, ...]) -> None:
    source = read(relative)
    positions = []
    for marker in markers:
        position = source.find(marker)
        if position < 0:
            fail(f"{relative} is missing ordered marker {marker!r}")
        positions.append(position)
    if positions != sorted(positions) or len(set(positions)) != len(positions):
        fail(f"{relative} operation order drifted: {markers!r}")


def verify_architecture() -> None:
    architecture = load_json(
        "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json"
    )
    if architecture.get("schema") != "hepta.current-architecture.v1":
        fail("wrong current architecture schema")
    if architecture.get("schemaVersion") != 2:
        fail("current architecture must include operation-wiring schema v2")

    components = architecture.get("components")
    if not isinstance(components, list):
        fail("components must be a list")
    by_id = {
        item.get("id"): item
        for item in components
        if isinstance(item, dict) and isinstance(item.get("id"), str)
    }
    provider = by_id.get("provider_effect_adapter")
    if not isinstance(provider, dict):
        fail("provider_effect_adapter is absent from the product graph")
    if provider.get("durableDomains") != []:
        fail("provider effect adapter must not become a second data owner")
    if provider.get("activeInLocalProfiles") is not False:
        fail("provider effect adapter must remain inert in local profiles")
    if provider.get("requiresExternalEffectCapability") is not True:
        fail("provider effect adapter must require external effect authority")

    protocol = architecture.get("crossOwnerProtocol")
    if not isinstance(protocol, dict):
        fail("crossOwnerProtocol must be an object")
    wired = set(protocol.get("wiredPaths", []))
    expected = {
        "automation_runtime_to_app_server",
        "matrix_ingress_to_app_server",
        "app_server_to_provider_effect_adapter",
    }
    if wired != expected:
        fail(f"cross-owner wired path set drifted: {wired!r}")
    if protocol.get("blindRetryAfterBoundary") is not False:
        fail("blind retry after a crossed boundary must remain false")

    provider_boundary = architecture.get("providerEffectBoundary")
    if not isinstance(provider_boundary, dict):
        fail("providerEffectBoundary is absent")
    if provider_boundary.get("dataOwner") is not False:
        fail("provider effect adapter cannot own product data")
    if provider_boundary.get("defaultAuthority") is not False:
        fail("provider effect adapter cannot gain default authority")
    if provider_boundary.get("dispatchRequires") != (
        "Authorized<ExternalEffectCapability>"
    ):
        fail("provider dispatch typed authority drifted")
    if provider_boundary.get("lookupAfterBoundaryOnly") is not True:
        fail("provider lookup must remain post-boundary only")
    if provider_boundary.get("blindRetry") is not False:
        fail("provider blind retry must remain false")


def verify_product_graph() -> None:
    source = require_markers(
        "codex-rs/hepta-contracts/src/product_graph.rs",
        (
            "PRODUCT_GRAPH_SCHEMA_VERSION: u32 = 2",
            "ProviderEffectAdapter",
            'Self::ProviderEffectAdapter => "provider_effect_adapter"',
            "from: ProductComponentId::AppServer",
            "to: ProductComponentId::ProviderEffectAdapter",
            "kind: ProductEdgeKind::SubmitsTo",
        ),
    )
    if "writer: ProductComponentId::ProviderEffectAdapter" in source:
        fail("provider effect adapter unexpectedly owns a durable domain")


def verify_matrix_operation() -> None:
    require_markers(
        "codex-rs/hepta-matrix-store/migrations/0005_matrix_operation_journal.sql",
        (
            "CREATE TABLE matrix_operations",
            "operation_id TEXT NOT NULL UNIQUE",
            "binding_sha256 TEXT NOT NULL",
            "CREATE TRIGGER matrix_operations_immutable_binding",
            "CREATE TRIGGER matrix_operations_no_delete",
            "Matrix operation binding is immutable",
        ),
    )
    require_markers(
        "codex-rs/hepta-matrix-store/src/operation.rs",
        (
            "pub struct MatrixOperationJournal",
            "ProductComponentId::MatrixIngress",
            "ProductComponentId::AppServer",
            "AuthorityAction::ServeSession",
            "pub async fn claim_delivery",
            "pub async fn mark_indeterminate",
            "pub async fn acknowledge",
            "pub async fn reconcile_applied",
            "RecoveryDecision::LookupOnly",
        ),
    )
    runtime = "codex-rs/hepta-matrixd/src/runtime.rs"
    require_markers(
        runtime,
        (
            "operations: MatrixOperationJournal",
            ".begin(inbox, &project_key, at_ms)",
            ".claim_delivery(&inbox.event_id",
            "MatrixAdmissionMode::ReconcileOnly",
            ".mark_indeterminate(&inbox.event_id",
            ".reconcile_applied(",
        ),
    )
    require_order(
        runtime,
        (
            ".begin(inbox, &project_key, at_ms)",
            ".begin_inbox_dispatch(&inbox.event_id, at_ms)",
            ".submit_matrix_event_on_binding(",
        ),
    )


def verify_provider_operation() -> None:
    require_markers(
        "codex-rs/hepta-contracts/src/provider_operation.rs",
        (
            "pub struct ProviderOperationRecord",
            "ProductComponentId::AppServer",
            "ProductComponentId::ProviderEffectAdapter",
            "AuthorityAction::ExternalEffect",
            "pub struct ProviderOperationCoordinator",
            "Authorized<ExternalEffectCapability>",
            "external_effect.is_external()",
            "binding.is_expired_at(observed_at_unix_seconds)",
            "OperationPhase::DeliveryClaimed",
            "OperationPhase::Indeterminate",
            "OperationPhase::ReconciledApplied",
            "OperationPhase::ReconciledNotApplied",
        ),
    )
    require_order(
        "codex-rs/hepta-contracts/src/provider_operation.rs",
        (
            "self.validate_authority(observed_at_unix_seconds)?;",
            "self.operation.claim_delivery()?;",
            "self.provider.dispatch_once(intent.clone()).await",
        ),
    )
    contracts = require_markers(
        "codex-rs/hepta-contracts/src/lib.rs",
        (
            "mod provider_operation;",
            "pub use provider_operation::ProviderOperationCoordinator;",
            "pub use provider_operation::ProviderOperationRecord;",
        ),
    )
    if "pub fn authorize_provider_operation_without_capability" in contracts:
        fail("provider operation exposes an authority bypass")


def verify_real_store_fault() -> None:
    require_markers(
        "codex-rs/hepta-matrix-store/tests/sqlite_full.rs",
        (
            "real_matrix_sqlite_full_rolls_back_failed_inbox_and_preserves_operation_reopen",
            "PRAGMA max_page_count",
            "real SQLite growth must reach SQLITE_FULL",
            "the failed product transaction must not leave a partial inbox row",
            "MatrixDurableStore::open(&layout",
            "reopened_operation",
        ),
    )


def verify_required_gate() -> None:
    workflow = read(".github/workflows/hepta-architecture-convergence-p0-2.yml")
    for marker in (
        "python3 scripts/verify-hepta-cross-owner-operation-wiring.py",
        "cargo test --locked -p codex-hepta-contracts provider_operation::tests",
        "cargo test --locked -p codex-hepta-matrix-store --test sqlite_full",
        "runs-on: ubuntu-24.04-arm",
        "Hepta architecture convergence required",
    ):
        if marker not in workflow:
            fail(f"canonical required workflow is missing {marker!r}")


def verify_ledger() -> None:
    ledger = load_json(
        "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V1.json"
    )
    work = ledger.get("postP0ProductWork")
    if not isinstance(work, dict):
        fail("postP0ProductWork must be an object")
    for gap in (
        "matrixOperationContractMigration",
        "providerEffectOperationContractMigration",
        "actualProductStoreFaultInjection",
    ):
        if work.get(gap) != "source_implemented":
            fail(f"ledger does not reflect implemented source gap: {gap}")


def main() -> int:
    verify_architecture()
    verify_product_graph()
    verify_matrix_operation()
    verify_provider_operation()
    verify_real_store_fault()
    verify_required_gate()
    verify_ledger()
    print(
        "PASS_HEPTA_CROSS_OWNER_OPERATION_WIRING_MATRIX_PROVIDER_SQLITE_FULL_SOURCE"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
