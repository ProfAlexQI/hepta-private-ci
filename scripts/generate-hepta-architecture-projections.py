#!/usr/bin/env python3
"""Generate deterministic Hepta architecture projections from the V2 model."""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
MODEL_PATH = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json"
OUTPUTS = {
    ROOT / "ARCHITECTURE.md": "architecture_markdown",
    ROOT / "docs/architecture/DATA_AUTHORITY_MAP.md": "data_authority_markdown",
    ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json": "legacy_json",
}


def load_model() -> dict[str, Any]:
    value = json.loads(MODEL_PATH.read_text(encoding="utf-8"))
    if not isinstance(value, dict) or value.get("schema") != "hepta.architecture-model.v2":
        raise SystemExit("invalid Hepta architecture model")
    return value


def component_by_id(model: dict[str, Any]) -> dict[str, dict[str, Any]]:
    components = model.get("components")
    if not isinstance(components, list):
        raise SystemExit("architecture components must be a list")
    result: dict[str, dict[str, Any]] = {}
    for component in components:
        if not isinstance(component, dict) or not isinstance(component.get("id"), str):
            raise SystemExit("invalid architecture component")
        identifier = component["id"]
        if identifier in result:
            raise SystemExit(f"duplicate architecture component: {identifier}")
        result[identifier] = component
    return result


def data_domains(model: dict[str, Any]) -> list[dict[str, Any]]:
    domains = model.get("dataDomains")
    if not isinstance(domains, list):
        raise SystemExit("architecture dataDomains must be a list")
    result: list[dict[str, Any]] = []
    seen: set[str] = set()
    for domain in domains:
        if not isinstance(domain, dict) or not isinstance(domain.get("id"), str):
            raise SystemExit("invalid architecture data domain")
        identifier = domain["id"]
        if identifier in seen:
            raise SystemExit(f"duplicate architecture data domain: {identifier}")
        seen.add(identifier)
        result.append(domain)
    return result


def runtime_profiles(model: dict[str, Any]) -> list[dict[str, Any]]:
    profiles = model.get("runtimeProfiles")
    if not isinstance(profiles, list):
        raise SystemExit("architecture runtimeProfiles must be a list")
    return [profile for profile in profiles if isinstance(profile, dict)]


def architecture_markdown(model: dict[str, Any]) -> str:
    components = component_by_id(model)
    profiles = runtime_profiles(model)
    lines = [
        "# Hepta current product architecture",
        "",
        "> GENERATED FILE — do not hand edit. The normative source is",
        "> `docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json`; regenerate with",
        "> `python3 scripts/generate-hepta-architecture-projections.py`.",
        "",
        "## Authority and scope",
        "",
        "The V2 model is the only editable product-architecture source. Rust contracts",
        "and generated human/machine projections are checked against it in CI. Historical",
        "plans, qualification receipts, Draft pull requests, and captured external documents",
        "are evidence or implementation inputs; they cannot grant runtime or release authority.",
        "",
        "Architecture data is deliberately split into three scopes:",
        "",
        "- `runtime_product_graph`: domains represented by Rust `ProductGraph`;",
        "- `external_control`: Supervisor-owned release and promotion authority;",
        "- `qualification_evidence`: append-only evidence outside the product dependency graph.",
        "",
        "## Runtime topology",
        "",
        "```text",
        "Supervisor control plane",
        "  │ lifecycle, release identity, signed grants",
        "  ▼",
        "Agentd composition root",
        "  ├─ Codex App Server       — thread/session owner",
        "  ├─ Hepta Memory Runtime   — memory/KG owner",
        "  ├─ Hepta Automation       — schedule/occurrence owner",
        "  └─ local ingress adapters — submit through typed product seams",
        "",
        "Qualification plane / Evidence subsystem",
        "  └─ read product artifacts and runtime evidence only",
        "     never become product dependencies or runtime authority sources",
        "```",
        "",
        "Agentd owns no durable product domain. It validates one closed authority profile,",
        "constructs one acyclic product graph, binds the runtime-profile contract, retains",
        "the process writer lock, starts owned services, monitors generation fencing, and",
        "shuts the composition down.",
        "",
        "## Runtime profiles",
        "",
        "| Profile | Allowed actions | Required services | Optional services | Disabled services |",
        "|---|---|---|---|---|",
    ]
    for profile in profiles:
        services = profile.get("services", [])
        required = [item["id"] for item in services if item.get("requirement") == "required"]
        optional = [item["id"] for item in services if item.get("requirement") == "optional"]
        disabled = [item["id"] for item in services if item.get("requirement") == "disabled"]
        lines.append(
            "| `{}` | {} | {} | {} | {} |".format(
                profile["id"],
                ", ".join(f"`{action}`" for action in profile.get("actions", [])) or "none",
                ", ".join(f"`{service}`" for service in required) or "none",
                ", ".join(f"`{service}`" for service in optional) or "none",
                ", ".join(f"`{service}`" for service in disabled) or "none",
            )
        )
    lines.extend(
        [
            "",
            "Required services fail startup closed and gate readiness. Optional services may",
            "enter a typed degraded state and never gate readiness. Disabled services must not",
            "start. The dormant provider-effect boundary cannot be activated by any local profile.",
            "",
            "## Product components",
            "",
            "| Component | Plane | Role | Product graph | Durable domains |",
            "|---|---|---|---:|---|",
        ]
    )
    for component in components.values():
        lines.append(
            "| `{}` | `{}` | `{}` | {} | {} |".format(
                component["id"],
                component["plane"],
                component["role"],
                "yes" if component.get("productGraphMember") else "no",
                ", ".join(f"`{item}`" for item in component.get("durableDomains", [])) or "none",
            )
        )
    lines.extend(
        [
            "",
            "## Data ownership",
            "",
            "Every authoritative domain has exactly one writer. Cross-owner changes use the",
            "digest-bound transactional outbox and acknowledgement protocol; no component",
            "dual-writes another owner's store. See `docs/architecture/DATA_AUTHORITY_MAP.md`.",
            "",
            "## Recovery",
            "",
            "Recovery establishes release and lifecycle identity before opening Agent-private",
            "stores, reconciles operations and outboxes before composition, and starts ingress",
            "only after the execution spine is ready. See `docs/architecture/RECOVERY_ORDER.md`.",
            "",
            "## Qualification identities",
            "",
            "Source head and merge candidate are separate evidence identities. A source-only",
            "verifier, queued job, zero runner, empty steps, generated artifact, or fixture is",
            "not executable qualification. Operator acceptance, promotion, and release remain",
            "independently issued external gates.",
            "",
        ]
    )
    return "\n".join(lines)


def data_authority_markdown(model: dict[str, Any]) -> str:
    components = component_by_id(model)
    domains = data_domains(model)
    lines = [
        "# Hepta data authority map",
        "",
        "> GENERATED FILE — do not hand edit. The normative source is",
        "> `docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json`.",
        "",
        "Every authoritative fact belongs to exactly one scope and has one writer. A",
        "projection may be rebuilt from its authority but must never overwrite or reinterpret",
        "that authority.",
        "",
        "| Scope | Domain | Authoritative writer | Durable store | Reader / projection rule |",
        "|---|---|---|---|---|",
    ]
    for domain in domains:
        writer = components.get(domain["writer"])
        if writer is None:
            raise SystemExit(f"unknown writer component: {domain['writer']}")
        lines.append(
            "| `{}` | {} | `{}` | `{}` | `{}` |".format(
                domain["scope"],
                domain["displayName"],
                writer["id"],
                domain["store"],
                domain["readerRule"],
            )
        )
    lines.extend(
        [
            "",
            "## Cross-owner mutation rule",
            "",
            "A source owner performs one local transaction:",
            "",
            "```text",
            "append operation intent",
            "→ mutate source-owned state",
            "→ append typed outbox envelope",
            "→ commit",
            "```",
            "",
            "Delivery is at least once. Before committing, the destination verifies the exact",
            "operation/idempotency identity, binding and payload digests, source/destination",
            "owners, authority epoch, owner epoch, generation, fencing token, and sequence. It",
            "commits only destination-owned state plus a digest-bound acknowledgement; the",
            "source adopts that acknowledgement in a later local transaction.",
            "",
            "## Prohibited patterns",
            "",
            "- Agentd becoming the writer for Memory, KG, Automation, Matrix, or Evidence;",
            "- two stores being described as one atomic commit;",
            "- a projection updating its source ledger;",
            "- blind retry after a delivery or provider boundary may have been crossed;",
            "- qualification fixtures writing product state;",
            "- booleans or receipts being converted into capabilities;",
            "- logs, model text, or disconnects being treated as proof of an external effect.",
            "",
        ]
    )
    return "\n".join(lines)


def legacy_json(model: dict[str, Any]) -> str:
    components = component_by_id(model)
    profiles = runtime_profiles(model)
    domains = data_domains(model)
    legacy_components = []
    for component in components.values():
        if component["id"] == "evidence_subsystem":
            continue
        entry: dict[str, Any] = {
            "id": component["id"],
            "plane": component["plane"],
            "role": component["role"],
            "durableDomains": component.get("durableDomains", []),
        }
        for key in ("activeInLocalProfiles", "requiresExternalEffectCapability", "productDependencyAllowed"):
            if key in component:
                entry[key] = component[key]
        legacy_components.append(entry)

    legacy_profiles = []
    for profile in profiles:
        entry: dict[str, Any] = {
            "id": profile["id"],
            "actions": profile.get("actions", []),
            "externalEffects": False,
            "modelInvocation": False,
            "promotion": False,
        }
        if profile["id"] == "agent_local":
            entry["cognitiveWrite"] = False
        if profile.get("qualificationOnly") is True:
            entry["qualificationOnly"] = True
        legacy_profiles.append(entry)

    legacy_domains = [
        {"domain": domain["id"], "writer": domain["writer"], "store": domain["store"]}
        for domain in domains
        if domain["scope"] in {"runtime_product_graph", "external_control"}
    ]
    value = {
        "schema": "hepta.current-architecture.v1",
        "schemaVersion": 2,
        "projection": {
            "status": "generated_compatibility_projection",
            "normativeModel": "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json",
            "generator": "scripts/generate-hepta-architecture-projections.py",
            "qualificationAuthority": False,
            "runtimeAuthority": False,
        },
        "authority": {
            "status": "canonical_product_architecture_compatibility_projection",
            "humanEntryPoint": "ARCHITECTURE.md",
            "machineEntryPoint": "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json",
            "rustAuthority": [
                "codex-rs/hepta-contracts/src/authority.rs",
                "codex-rs/hepta-contracts/src/product_graph.rs",
                "codex-rs/hepta-contracts/src/operation.rs",
                "codex-rs/hepta-contracts/src/provider_operation.rs",
                "codex-rs/hepta-agentd/src/runtime_profile.rs",
            ],
            "externalPlanSnapshotsAreAuthority": False,
            "qualificationReceiptsAreProductAuthority": False,
            "draftPullRequestsAreProductAuthority": False,
        },
        "runtimeProfiles": legacy_profiles,
        "components": legacy_components,
        "dataAuthorities": legacy_domains,
        "crossOwnerProtocol": model["crossOwnerProtocol"],
        "providerEffectBoundary": {
            **model["providerEffectBoundary"],
            "operationContract": "ProviderOperationRecord",
            "coordinator": "ProviderOperationCoordinator",
        },
        "recoveryOrder": model["recoveryOrder"],
        "authorityBoundary": model["authorityBoundary"],
    }
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def render(model: dict[str, Any], kind: str) -> str:
    if kind == "architecture_markdown":
        return architecture_markdown(model)
    if kind == "data_authority_markdown":
        return data_authority_markdown(model)
    if kind == "legacy_json":
        return legacy_json(model)
    raise AssertionError(kind)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    model = load_model()
    mismatches: list[str] = []
    for path, kind in OUTPUTS.items():
        expected = render(model, kind)
        if args.check:
            try:
                actual = path.read_text(encoding="utf-8")
            except FileNotFoundError:
                mismatches.append(str(path.relative_to(ROOT)))
                continue
            if actual != expected:
                mismatches.append(str(path.relative_to(ROOT)))
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(expected, encoding="utf-8")
    if mismatches:
        print("FAIL_HEPTA_ARCHITECTURE_PROJECTION_DRIFT: " + ", ".join(mismatches), file=sys.stderr)
        return 1
    print("PASS_HEPTA_ARCHITECTURE_PROJECTIONS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
