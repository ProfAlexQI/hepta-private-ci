#!/usr/bin/env python3
"""Whitespace-tolerant release/evidence ownership migration for ProductGraph."""

from __future__ import annotations

import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
PRODUCT_GRAPH = ROOT / "codex-rs/hepta-contracts/src/product_graph.rs"
ARCHITECTURE = ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json"
VERIFIER = ROOT / "scripts/verify-hepta-architecture-convergence-p0-2.py"
STATUS = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json"


def insert_after_line(source: str, anchor: str, insertion: str, present: str) -> str:
    if present in source:
        return source
    lines = source.splitlines(keepends=True)
    matches = [index for index, line in enumerate(lines) if anchor in line]
    if len(matches) != 1:
        raise SystemExit(f"expected one anchor {anchor!r}, found {len(matches)}")
    index = matches[0]
    indent = lines[index][: len(lines[index]) - len(lines[index].lstrip())]
    lines.insert(index + 1, indent + insertion + "\n")
    return "".join(lines)


def insert_before_regex(source: str, pattern: str, block: str, present: str) -> str:
    if present in source:
        return source
    match = re.search(pattern, source, flags=re.MULTILINE | re.DOTALL)
    if match is None:
        raise SystemExit(f"source anchor pattern not found: {pattern}")
    return source[: match.start()] + block + source[match.start() :]


def migrate_rust_graph() -> None:
    source = PRODUCT_GRAPH.read_text(encoding="utf-8")

    # Enum variants and their stable string forms.
    source = insert_after_line(
        source,
        "    AutomationRuntime,",
        "EvidenceRuntime,",
        "    EvidenceRuntime,",
    )
    source = insert_after_line(
        source,
        'Self::AutomationRuntime => "automation_runtime",',
        'Self::EvidenceRuntime => "evidence_runtime",',
        'Self::EvidenceRuntime => "evidence_runtime",',
    )
    source = insert_after_line(
        source,
        "    AgentLifecycle,",
        "ReleasePromotion,",
        "    ReleasePromotion,",
    )
    source = insert_after_line(
        source,
        "    AutomationSchedule,",
        "GovernanceEvidence,",
        "    GovernanceEvidence,",
    )
    source = insert_after_line(
        source,
        'Self::AgentLifecycle => "agent_lifecycle",',
        'Self::ReleasePromotion => "release_promotion",',
        'Self::ReleasePromotion => "release_promotion",',
    )
    source = insert_after_line(
        source,
        'Self::AutomationSchedule => "automation_schedule",',
        'Self::GovernanceEvidence => "governance_evidence",',
        'Self::GovernanceEvidence => "governance_evidence",',
    )
    source = insert_after_line(
        source,
        "    JsonRegistry,",
        "SignedReleaseState,",
        "    SignedReleaseState,",
    )
    source = insert_after_line(
        source,
        'Self::JsonRegistry => "json_registry",',
        'Self::SignedReleaseState => "signed_release_state",',
        'Self::SignedReleaseState => "signed_release_state",',
    )

    # Product component list.
    if "ProductComponentId::EvidenceRuntime," not in source:
        component_list = source.find("components: vec![")
        if component_list < 0:
            raise SystemExit("product component list is missing")
        anchor = source.find("ProductComponentId::AutomationRuntime,", component_list)
        if anchor < 0:
            raise SystemExit("automation component entry is missing")
        line_end = source.find("\n", anchor)
        line_start = source.rfind("\n", component_list, anchor) + 1
        indent = source[line_start:anchor]
        source = (
            source[: line_end + 1]
            + indent
            + "ProductComponentId::EvidenceRuntime,\n"
            + source[line_end + 1 :]
        )

    # Evidence runtime edge, inserted before Matrix ingress submission.
    evidence_edge_marker = "to: ProductComponentId::EvidenceRuntime"
    source = insert_before_regex(
        source,
        r"(?P<indent>\s*)ProductEdge\s*\{\s*from:\s*ProductComponentId::MatrixIngress,",
        "                ProductEdge {\n"
        "                    from: ProductComponentId::AppServer,\n"
        "                    to: ProductComponentId::EvidenceRuntime,\n"
        "                    kind: ProductEdgeKind::Composes,\n"
        "                },\n",
        evidence_edge_marker,
    )

    # Release and governance data authorities.
    source = insert_before_regex(
        source,
        r"(?P<indent>\s*)DataAuthority\s*\{\s*domain:\s*DataDomain::ThreadSession,",
        "                DataAuthority {\n"
        "                    domain: DataDomain::ReleasePromotion,\n"
        "                    writer: ProductComponentId::Supervisor,\n"
        "                    store: DataStoreKind::SignedReleaseState,\n"
        "                },\n",
        "domain: DataDomain::ReleasePromotion",
    )
    source = insert_before_regex(
        source,
        r"(?P<indent>\s*)DataAuthority\s*\{\s*domain:\s*DataDomain::IngressProjection,",
        "                DataAuthority {\n"
        "                    domain: DataDomain::GovernanceEvidence,\n"
        "                    writer: ProductComponentId::EvidenceRuntime,\n"
        "                    store: DataStoreKind::AgentPrivateSqlite,\n"
        "                },\n",
        "domain: DataDomain::GovernanceEvidence",
    )

    # Required domain list. Restrict the insertion to the validation loop.
    loop_start = source.find("for domain in [")
    if loop_start < 0:
        raise SystemExit("required-domain validation loop is missing")
    loop_end = source.find("]", loop_start)
    if loop_end < 0:
        raise SystemExit("required-domain validation loop is unterminated")
    domain_list = source[loop_start:loop_end]
    if "DataDomain::ReleasePromotion" not in domain_list:
        domain_list = domain_list.replace(
            "DataDomain::AgentLifecycle,",
            "DataDomain::AgentLifecycle, DataDomain::ReleasePromotion,",
            1,
        )
    if "DataDomain::GovernanceEvidence" not in domain_list:
        domain_list = domain_list.replace(
            "DataDomain::AutomationSchedule,",
            "DataDomain::AutomationSchedule, DataDomain::GovernanceEvidence,",
            1,
        )
    source = source[:loop_start] + domain_list + source[loop_end:]

    PRODUCT_GRAPH.write_text(source, encoding="utf-8")


def migrate_architecture_json() -> None:
    value = json.loads(ARCHITECTURE.read_text(encoding="utf-8"))
    components = value.get("components")
    authorities = value.get("dataAuthorities")
    if not isinstance(components, list) or not isinstance(authorities, list):
        raise SystemExit("canonical architecture lists are missing")
    if not any(component.get("id") == "evidence_runtime" for component in components):
        insert_at = next(
            (
                index + 1
                for index, component in enumerate(components)
                if component.get("id") == "automation_runtime"
            ),
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
    line = '    "governance_evidence": "evidence_runtime",\n'
    if line not in source:
        anchor = '    "automation_schedule": "automation_runtime",\n'
        if source.count(anchor) != 1:
            raise SystemExit("source verifier writer-map anchor drifted")
        source = source.replace(anchor, anchor + line, 1)
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
    print("PRODUCT_GRAPH_AUTHORITY_COMPLETION_P0_1_V2_APPLIED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
