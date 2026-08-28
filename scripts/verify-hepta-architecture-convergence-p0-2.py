#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta architecture convergence P0.2."""

from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
ARCHITECTURE = ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"

REQUIRED_FILES = (
    ROOT / "ARCHITECTURE.md",
    ROOT / "docs/architecture/DATA_AUTHORITY_MAP.md",
    ROOT / "docs/architecture/RECOVERY_ORDER.md",
    ROOT / "codex-rs/hepta-contracts/src/authority.rs",
    ROOT / "codex-rs/hepta-contracts/src/product_graph.rs",
    ROOT / "codex-rs/hepta-contracts/src/operation.rs",
    ROOT / "codex-rs/hepta-memory/src/cognitive_runtime.rs",
    ROOT / "codex-rs/hepta-agentd/src/composition.rs",
    ROOT / "codex-rs/hepta-agentd/src/runtime.rs",
    ROOT / "codex-rs/hepta-agentd/src/app_runtime.rs",
)

CLOSED_AUTHORITY_FIELDS = (
    "productionCaller",
    "productionWriter",
    "effectAuthority",
    "externalEffect",
    "modelInvocationAuthority",
    "providerDispatchAuthority",
    "fleetMutationAuthority",
    "operatorAcceptance",
    "promotion",
    "release",
)

REQUIRED_DATA_WRITERS = {
    "fleet_registry": "supervisor",
    "agent_lifecycle": "supervisor",
    "release_promotion": "supervisor",
    "thread_session": "app_server",
    "memory_ledger": "memory_runtime",
    "knowledge_projection": "memory_runtime",
    "automation_schedule": "automation_runtime",
    "ingress_projection": "matrix_ingress",
    "runtime_health": "agentd",
}


def fail(message: str) -> "NoReturn":
    raise SystemExit(f"FAIL_ARCHITECTURE_CONVERGENCE_P0_2: {message}")


def load_json(path: pathlib.Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot load {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one JSON object")
    return value


def require_source(path: pathlib.Path, needles: tuple[str, ...]) -> None:
    try:
        source = path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")
    for needle in needles:
        if needle not in source:
            fail(f"{path.relative_to(ROOT)} is missing {needle!r}")


def main() -> int:
    missing = [str(path.relative_to(ROOT)) for path in REQUIRED_FILES if not path.is_file()]
    if missing:
        fail(f"required files are absent: {missing}")

    architecture = load_json(ARCHITECTURE)
    status = load_json(STATUS)

    if architecture.get("schema") != "hepta.current-architecture.v1":
        fail("unexpected architecture schema")
    authority = architecture.get("authority")
    if not isinstance(authority, dict):
        fail("architecture authority must be an object")
    for field in (
        "externalPlanSnapshotsAreAuthority",
        "qualificationReceiptsAreProductAuthority",
        "draftPullRequestsAreProductAuthority",
    ):
        if authority.get(field) is not False:
            fail(f"{field} must remain false")

    components = architecture.get("components")
    if not isinstance(components, list) or not components:
        fail("components must be a non-empty list")
    component_by_id: dict[str, dict[str, Any]] = {}
    for component in components:
        if not isinstance(component, dict) or not isinstance(component.get("id"), str):
            fail("every component must have a string id")
        component_id = component["id"]
        if component_id in component_by_id:
            fail(f"duplicate component {component_id}")
        component_by_id[component_id] = component
    if component_by_id.get("agentd", {}).get("durableDomains") != []:
        fail("agentd must own no durable domain")
    if component_by_id.get("qualification_plane", {}).get("productDependencyAllowed") is not False:
        fail("qualification plane must not be a product dependency")

    data_authorities = architecture.get("dataAuthorities")
    if not isinstance(data_authorities, list):
        fail("dataAuthorities must be a list")
    writers: dict[str, str] = {}
    for entry in data_authorities:
        if not isinstance(entry, dict):
            fail("data authority entry must be an object")
        domain = entry.get("domain")
        writer = entry.get("writer")
        if not isinstance(domain, str) or not isinstance(writer, str):
            fail("data authority domain and writer must be strings")
        if domain in writers:
            fail(f"durable domain {domain} has more than one writer")
        writers[domain] = writer
    if writers != REQUIRED_DATA_WRITERS:
        fail(f"data writer map drifted: {writers!r}")

    protocol = architecture.get("crossOwnerProtocol")
    if not isinstance(protocol, dict):
        fail("crossOwnerProtocol must be an object")
    if protocol.get("transport") != "transactional_outbox":
        fail("cross-owner transport must remain transactional_outbox")
    if protocol.get("blindRetryAfterBoundary") is not False:
        fail("blind retry after a crossed boundary must remain false")

    status_authority = status.get("authority")
    if not isinstance(status_authority, dict):
        fail("status authority must be an object")
    for field in CLOSED_AUTHORITY_FIELDS:
        if status_authority.get(field) is not False:
            fail(f"status authority field {field} must remain false")
    qualification = status.get("qualification")
    if not isinstance(qualification, dict) or qualification.get("qualified") is not False:
        fail("source status cannot claim executable qualification")

    require_source(
        ROOT / "codex-rs/hepta-contracts/src/authority.rs",
        (
            "pub struct AuthorityGrant",
            "pub struct Authorized<C>",
            "AuthorityAction::ExternalEffect",
            "AuthorityAction::PromoteRelease",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-contracts/src/product_graph.rs",
        (
            "pub struct ProductGraph",
            "QualificationComponentInProductGraph",
            "DuplicateDataWriter",
            "DependencyCycle",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-contracts/src/operation.rs",
        (
            "pub struct OperationBinding",
            "pub struct OutboxEnvelope",
            "RecoveryDecision::LookupOnly",
            "blindly_retries_after_delivery_boundary",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-agentd/src/composition.rs",
        (
            "pub(crate) struct AgentRuntimeComposition",
            "ProductGraph::agent_local",
            "AuthorityGrant::agent_local",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-agentd/src/app_runtime.rs",
        (
            "authorize::<CognitiveWriteCapability>()",
            "unexpected cognitive write authority",
            "authority.is_product_closed()",
        ),
    )
    require_source(
        ROOT / "codex-rs/hepta-agentd/src/runtime.rs",
        (
            "AgentRuntimeComposition::open(config)",
            "let _product_graph = product_graph",
        ),
    )

    print("PASS_ARCHITECTURE_CONVERGENCE_P0_2_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
