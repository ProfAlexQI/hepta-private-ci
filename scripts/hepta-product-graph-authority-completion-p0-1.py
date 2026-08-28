#!/usr/bin/env python3
"""Complete release and governance-evidence ownership in the product graph."""

from __future__ import annotations

import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
PRODUCT_GRAPH = ROOT / "codex-rs/hepta-contracts/src/product_graph.rs"
ARCHITECTURE = ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json"
VERIFIER = ROOT / "scripts/verify-hepta-architecture-convergence-p0-2.py"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"


def replace_once(source: str, old: str, new: str, label: str) -> str:
    if new in source:
        return source
    if source.count(old) != 1:
        raise SystemExit(f"{label} source anchor drifted")
    return source.replace(old, new, 1)


def migrate_rust_graph() -> None:
    source = PRODUCT_GRAPH.read_text(encoding="utf-8")
    source = replace_once(
        source,
        "    AutomationRuntime,\n    MatrixIngress,",
        "    AutomationRuntime,\n    EvidenceRuntime,\n    MatrixIngress,",
        "component enum",
    )
    source = replace_once(
        source,
        '            Self::AutomationRuntime => "automation_runtime",\n            Self::MatrixIngress',
        '            Self::AutomationRuntime => "automation_runtime",\n'
        '            Self::EvidenceRuntime => "evidence_runtime",\n'
        '            Self::MatrixIngress',
        "component name",
    )
    source = replace_once(
        source,
        "    AgentLifecycle,\n    ThreadSession,",
        "    AgentLifecycle,\n    ReleasePromotion,\n    ThreadSession,",
        "release domain enum",
    )
    source = replace_once(
        source,
        "    AutomationSchedule,\n    IngressProjection,",
        "    AutomationSchedule,\n    GovernanceEvidence,\n    IngressProjection,",
        "evidence domain enum",
    )
    source = replace_once(
        source,
        '            Self::AgentLifecycle => "agent_lifecycle",\n            Self::ThreadSession',
        '            Self::AgentLifecycle => "agent_lifecycle",\n'
        '            Self::ReleasePromotion => "release_promotion",\n'
        '            Self::ThreadSession',
        "release domain name",
    )
    source = replace_once(
        source,
        '            Self::AutomationSchedule => "automation_schedule",\n            Self::IngressProjection',
        '            Self::AutomationSchedule => "automation_schedule",\n'
        '            Self::GovernanceEvidence => "governance_evidence",\n'
        '            Self::IngressProjection',
        "evidence domain name",
    )
    source = replace_once(
        source,
        "    JsonRegistry,\n    AgentPrivateSqlite,",
        "    JsonRegistry,\n    SignedReleaseState,\n    AgentPrivateSqlite,",
        "release store enum",
    )
    source = replace_once(
        source,
        '            Self::JsonRegistry => "json_registry",\n            Self::AgentPrivateSqlite',
        '            Self::JsonRegistry => "json_registry",\n'
        '            Self::SignedReleaseState => "signed_release_state",\n'
        '            Self::AgentPrivateSqlite',
        "release store name",
    )
    source = replace_once(
        source,
        "                ProductComponentId::AutomationRuntime,\n                ProductComponentId::MatrixIngress,",
        "                ProductComponentId::AutomationRuntime,\n"
        "                ProductComponentId::EvidenceRuntime,\n"
        "                ProductComponentId::MatrixIngress,",
        "component graph entry",
    )
    source = replace_once(
        source,
        "                ProductEdge { from: ProductComponentId::AutomationRuntime, to: ProductComponentId::AppServer, kind: ProductEdgeKind::SubmitsTo },\n                ProductEdge { from: ProductComponentId::MatrixIngress,",
        "                ProductEdge { from: ProductComponentId::AutomationRuntime, to: ProductComponentId::AppServer, kind: ProductEdgeKind::SubmitsTo },\n"
        "                ProductEdge { from: ProductComponentId::AppServer, to: ProductComponentId::EvidenceRuntime, kind: ProductEdgeKind::Composes },\n"
        "                ProductEdge { from: ProductComponentId::MatrixIngress,",
        "evidence graph edge",
    )
    source = replace_once(
        source,
        "                DataAuthority { domain: DataDomain::AgentLifecycle, writer: ProductComponentId::Supervisor, store: DataStoreKind::JsonRegistry },\n                DataAuthority { domain: DataDomain::ThreadSession,",
        "                DataAuthority { domain: DataDomain::AgentLifecycle, writer: ProductComponentId::Supervisor, store: DataStoreKind::JsonRegistry },\n"
        "                DataAuthority { domain: DataDomain::ReleasePromotion, writer: ProductComponentId::Supervisor, store: DataStoreKind::SignedReleaseState },\n"
        "                DataAuthority { domain: DataDomain::ThreadSession,",
        "release data authority",
    )
    source = replace_once(
        source,
        "                DataAuthority { domain: DataDomain::AutomationSchedule, writer: ProductComponentId::AutomationRuntime, store: DataStoreKind::AgentPrivateSqlite },\n                DataAuthority { domain: DataDomain::IngressProjection,",
        "                DataAuthority { domain: DataDomain::AutomationSchedule, writer: ProductComponentId::AutomationRuntime, store: DataStoreKind::AgentPrivateSqlite },\n"
        "                DataAuthority { domain: DataDomain::GovernanceEvidence, writer: ProductComponentId::EvidenceRuntime, store: DataStoreKind::AgentPrivateSqlite },\n"
        "                DataAuthority { domain: DataDomain::IngressProjection,",
        "evidence data authority",
    )
    source = replace_once(
        source,
        "for domain in [DataDomain::FleetRegistry, DataDomain::AgentLifecycle, DataDomain::ThreadSession, DataDomain::MemoryLedger, DataDomain::KnowledgeProjection, DataDomain::AutomationSchedule, DataDomain::IngressProjection, DataDomain::RuntimeHealth]",
        "for domain in [DataDomain::FleetRegistry, DataDomain::AgentLifecycle, DataDomain::ReleasePromotion, DataDomain::ThreadSession, DataDomain::MemoryLedger, DataDomain::KnowledgeProjection, DataDomain::AutomationSchedule, DataDomain::GovernanceEvidence, DataDomain::IngressProjection, DataDomain::RuntimeHealth]",
        "required data authorities",
    )
    PRODUCT_GRAPH.write_text(source, encoding="utf-8")


def migrate_architecture_json() -> None:
    value = json.loads(ARCHITECTURE.read_text(encoding="utf-8"))
    components = value.get("components")
    authorities = value.get("dataAuthorities")
    if not isinstance(components, list) or not isinstance(authorities, list):
        raise SystemExit("canonical architecture lists are missing")
    if not any(component.get("id") == "evidence_runtime" for component in components):
        insert_at = next(
            (index + 1 for index, component in enumerate(components) if component.get("id") == "automation_runtime"),
            len(components),
        )
        components.insert(
            insert_at,
            {
                "id": "evidence_runtime",
                "plane": "domain",
                "role": "governance_and_provider_evidence_owner",
                "durableDomains": ["governance_evidence"],
            },
        )
    if not any(entry.get("domain") == "governance_evidence" for entry in authorities):
        authorities.append(
            {
                "domain": "governance_evidence",
                "writer": "evidence_runtime",
                "store": "agent_private_sqlite",
            }
        )
    ARCHITECTURE.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def migrate_verifier() -> None:
    source = VERIFIER.read_text(encoding="utf-8")
    source = replace_once(
        source,
        '    "automation_schedule": "automation_runtime",\n    "ingress_projection":',
        '    "automation_schedule": "automation_runtime",\n'
        '    "governance_evidence": "evidence_runtime",\n'
        '    "ingress_projection":',
        "source verifier evidence owner",
    )
    VERIFIER.write_text(source, encoding="utf-8")


def migrate_status() -> None:
    value = json.loads(STATUS.read_text(encoding="utf-8"))
    implemented = value.get("implemented")
    if not isinstance(implemented, dict):
        raise SystemExit("status implemented map is missing")
    implemented["releasePromotionDataOwner"] = True
    implemented["governanceEvidenceDataOwner"] = True
    STATUS.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    migrate_rust_graph()
    migrate_architecture_json()
    migrate_verifier()
    migrate_status()
    print("PRODUCT_GRAPH_AUTHORITY_COMPLETION_P0_1_APPLIED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
