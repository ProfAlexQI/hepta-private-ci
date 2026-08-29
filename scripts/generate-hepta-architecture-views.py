#!/usr/bin/env python3
"""Generate all current Hepta architecture views from the sole catalog."""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
CATALOG_PATH = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_CATALOG_V1.json"
OUTPUTS = (
    ROOT / "ARCHITECTURE.md",
    ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json",
    ROOT / "docs/architecture/DATA_AUTHORITY_MAP.md",
    ROOT / "docs/architecture/HEPTA_RUNTIME_PROFILE_MATRIX_V1.json",
)


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_ARCHITECTURE_GENERATION: {message}")


def load_json_no_duplicates(path: pathlib.Path) -> dict[str, Any]:
    def pairs_hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                fail(f"duplicate JSON key {key!r} in {path.relative_to(ROOT)}")
            result[key] = value
        return result

    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs_hook)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot read {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail("catalog must contain one JSON object")
    return value


def json_bytes(value: object) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()


def render_current(catalog: dict[str, Any]) -> bytes:
    authority = catalog["authority"]
    components = []
    for component in catalog["components"]:
        components.append(
            {
                "id": component["id"],
                "plane": component["plane"],
                "role": component["role"],
                "durableDomains": component["durableDomains"],
                "runtimeGraphMember": component["runtimeGraphMember"],
            }
        )

    profiles = []
    for profile in catalog["runtimeProfiles"]:
        services = profile["services"]
        profiles.append(
            {
                "id": profile["id"],
                "actions": profile["actions"],
                "hostsAgentProductGraph": profile["hostsAgentProductGraph"],
                "qualificationOnly": profile["qualificationOnly"],
                "requiredServices": sorted(
                    service for service, state in services.items() if state == "required"
                ),
                "optionalServices": sorted(
                    service for service, state in services.items() if state == "optional"
                ),
                "disabledServices": sorted(
                    service for service, state in services.items() if state == "disabled"
                ),
                "qualificationPrerequisites": profile["qualificationPrerequisites"],
                "externalEffects": False,
                "modelInvocation": False,
                "promotion": False,
            }
        )

    data_authorities = []
    for domain in catalog["dataDomains"]:
        data_authorities.append(
            {
                "domain": domain["id"],
                "plane": domain["plane"],
                "durability": domain["durability"],
                "writer": domain["writer"],
                "store": domain["store"],
                "migrationOwner": domain["migrationOwner"],
                "rustProductGraph": domain["rustProductGraph"],
            }
        )

    value = {
        "schema": "hepta.current-architecture.v1",
        "schemaVersion": 2,
        "catalogVersion": catalog["schemaVersion"],
        "authority": {
            "status": "generated_current_architecture_view",
            "humanEntryPoint": authority["humanEntryPoint"],
            "catalog": authority["catalog"],
            "machineEntryPoint": "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json",
            "generator": authority["generator"],
            "rustAuthority": [
                "codex-rs/hepta-contracts/src/authority.rs",
                "codex-rs/hepta-contracts/src/product_graph.rs",
                "codex-rs/hepta-contracts/src/operation.rs",
                "codex-rs/hepta-contracts/src/provider_operation.rs",
            ],
            "externalPlanSnapshotsAreAuthority": False,
            "qualificationReceiptsAreProductAuthority": False,
            "draftPullRequestsAreProductAuthority": False,
        },
        "runtimeProfiles": profiles,
        "components": components,
        "productEdges": catalog["productEdges"],
        "dataAuthorities": data_authorities,
        "productGraphScope": {
            "rustEnforcedDomains": [
                domain["id"] for domain in catalog["dataDomains"] if domain["rustProductGraph"]
            ],
            "externalControlDomains": [
                domain["id"]
                for domain in catalog["dataDomains"]
                if domain["plane"] == "control" and not domain["rustProductGraph"]
            ],
            "externalEvidenceDomains": [
                domain["id"]
                for domain in catalog["dataDomains"]
                if domain["plane"] == "evidence" and not domain["rustProductGraph"]
            ],
        },
        "crossOwnerProtocol": catalog["crossOwnerProtocol"],
        "providerEffectBoundary": catalog["providerEffectBoundary"],
        "recoveryOrder": catalog["recoveryOrder"],
        "authorityBoundary": catalog["authorityBoundary"],
    }
    return json_bytes(value)


def render_profile_matrix(catalog: dict[str, Any]) -> bytes:
    value = {
        "schema": "hepta.runtime-profile-matrix.v1",
        "schemaVersion": 1,
        "catalog": catalog["authority"]["catalog"],
        "states": ["required", "optional", "disabled"],
        "profiles": [
            {
                "id": profile["id"],
                "hostsAgentProductGraph": profile["hostsAgentProductGraph"],
                "qualificationOnly": profile["qualificationOnly"],
                "actions": profile["actions"],
                "services": profile["services"],
                "qualificationPrerequisites": profile["qualificationPrerequisites"],
                "failurePolicy": {
                    "requiredUnavailable": "startup_fail_closed",
                    "optionalUnavailable": "typed_degraded",
                    "disabledPresent": "configuration_error",
                },
            }
            for profile in catalog["runtimeProfiles"]
        ],
        "authorityBoundary": catalog["authorityBoundary"],
    }
    return json_bytes(value)


def render_data_map(catalog: dict[str, Any]) -> bytes:
    lines = [
        "# Hepta data authority map",
        "",
        "> Generated by `scripts/generate-hepta-architecture-views.py` from",
        "> `HEPTA_ARCHITECTURE_CATALOG_V1.json`. Do not edit this view directly.",
        "",
        "Every fact has one authoritative writer. Durable product facts, control-plane",
        "facts, evidence-plane facts, and ephemeral observations are deliberately",
        "separated; only rows marked `yes` are enforced by the Agent product `ProductGraph`.",
        "",
        "| Domain | Plane | Durability | Authoritative writer | Store | Migration owner | Rust ProductGraph |",
        "|---|---|---|---|---|---|---|",
    ]
    for domain in catalog["dataDomains"]:
        migration = domain["migrationOwner"] or "n/a"
        rust = "yes" if domain["rustProductGraph"] else "no"
        lines.append(
            f"| `{domain['id']}` | {domain['plane']} | {domain['durability']} | "
            f"`{domain['writer']}` | `{domain['store']}` | `{migration}` | {rust} |"
        )
    lines.extend(
        [
            "",
            "## Cross-owner mutation rule",
            "",
            "A source owner commits its own state and an immutable outbox envelope in one",
            "local transaction. The destination validates the exact operation identity,",
            "idempotency key, owners, payload digest, authority epoch, owner epoch,",
            "generation and fencing-token digest before committing only destination-owned",
            "state. The source adopts the digest-bound acknowledgement later.",
            "",
            "Delivery is at least once, but a crossed or uncertain boundary is always",
            "lookup-only. Changed replay, stale generation, stale epoch, stale fence,",
            "terminal reopen and implicit dual-write fail closed.",
            "",
            "## Prohibited patterns",
            "",
            "- Agentd becoming a durable writer for Memory, KG, Automation, Matrix or Evidence;",
            "- treating control/evidence domains as members of the Agent-private product graph;",
            "- describing two local databases as one atomic transaction;",
            "- a projection overwriting its source ledger;",
            "- blind retry after a delivery/provider boundary may have been crossed;",
            "- qualification fixtures writing product state;",
            "- a status boolean, log line or model output being treated as a capability;",
            "- a transport disconnect being treated as proof of an external effect.",
            "",
        ]
    )
    return "\n".join(lines).encode()


def render_architecture(catalog: dict[str, Any]) -> bytes:
    profile_rows = []
    for profile in catalog["runtimeProfiles"]:
        services = profile["services"]
        required = ", ".join(
            f"`{service}`" for service, state in services.items() if state == "required"
        ) or "none"
        optional = ", ".join(
            f"`{service}`" for service, state in services.items() if state == "optional"
        ) or "none"
        disabled = ", ".join(
            f"`{service}`" for service, state in services.items() if state == "disabled"
        ) or "none"
        prerequisites = ", ".join(
            f"`{service}`" for service in profile["qualificationPrerequisites"]
        ) or "none"
        profile_rows.append(
            f"| `{profile['id']}` | {required} | {optional} | {disabled} | "
            f"{prerequisites} | {'yes' if profile['qualificationOnly'] else 'no'} |"
        )

    lines = [
        "# Hepta current product architecture",
        "",
        "> Generated from `docs/architecture/HEPTA_ARCHITECTURE_CATALOG_V1.json`.",
        "> Change the catalog and run `python3 scripts/generate-hepta-architecture-views.py --write`;",
        "> CI accepts only byte-identical `--check` output.",
        "",
        "The catalog is the sole editable architecture fact source. Rust remains the",
        "runtime enforcement for typed authority, the Agent product graph and cross-owner",
        "operations. Historical plans, qualification receipts and Draft pull requests are",
        "evidence or development inputs; none can mint runtime authority.",
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
        "  └─ local ingress adapters — typed submissions only",
        "",
        "Evidence store / qualification plane",
        "  └─ outside the Agent product graph; observes candidate-bound evidence only",
        "```",
        "",
        "Agentd owns no durable product domain. It validates one closed authority",
        "profile, constructs the acyclic product graph, holds the process writer lock,",
        "binds services, monitors lifecycle generation and shuts down the composition.",
        "",
        "## Runtime profile availability",
        "",
        "| Profile | Required services | Optional services | Disabled services | Qualification prerequisites | Qualification only |",
        "|---|---|---|---|---|---|",
        *profile_rows,
        "",
        "A missing required service fails startup closed. A missing optional service",
        "produces an explicit degraded state. A disabled service appearing in the runtime",
        "is a configuration error; source presence never activates it.",
        "",
        "## Authority",
        "",
        "Runtime calls consume typed `Authorized<C>` witnesses. Serialized claims,",
        "qualification booleans and source receipts are descriptive only. Current local",
        "profiles cannot grant model invocation, provider dispatch, external effects,",
        "fleet mutation, operator acceptance, promotion or release.",
        "",
        "## Data ownership",
        "",
        "The generated `docs/architecture/DATA_AUTHORITY_MAP.md` distinguishes the",
        "Agent product graph from external control/evidence planes. Every domain has one",
        "writer and one migration owner. Cross-owner changes use durable",
        "command/outbox/acknowledgement semantics and never dual-write another owner.",
        "",
        "## Recovery",
        "",
        "Recovery establishes release and lifecycle identity before opening Agent-private",
        "stores, then validates migrations, integrity, operation journals and outboxes",
        "before composition. See `docs/architecture/RECOVERY_ORDER.md`.",
        "",
        "## Qualification identities",
        "",
        "Source head and merge candidate are separate evidence identities. A queued job,",
        "`runner_id=0`, `steps=[]`, generated artifact or source-only verifier is not",
        "executable qualification. Operator acceptance and promotion are later,",
        "independently issued decisions.",
        "",
    ]
    return "\n".join(lines).encode()


def render_all(catalog: dict[str, Any]) -> dict[pathlib.Path, bytes]:
    return {
        ROOT / "ARCHITECTURE.md": render_architecture(catalog),
        ROOT / "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json": render_current(catalog),
        ROOT / "docs/architecture/DATA_AUTHORITY_MAP.md": render_data_map(catalog),
        ROOT / "docs/architecture/HEPTA_RUNTIME_PROFILE_MATRIX_V1.json": render_profile_matrix(catalog),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--write", action="store_true")
    args = parser.parse_args()

    catalog = load_json_no_duplicates(CATALOG_PATH)
    rendered = render_all(catalog)
    for path, expected in rendered.items():
        if args.check:
            try:
                actual = path.read_bytes()
            except OSError as error:
                fail(f"cannot read generated view {path.relative_to(ROOT)}: {error}")
            if actual != expected:
                fail(f"generated view drifted: {path.relative_to(ROOT)}")
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(expected)

    print(
        "PASS_HEPTA_ARCHITECTURE_VIEWS_CHECK"
        if args.check
        else "PASS_HEPTA_ARCHITECTURE_VIEWS_WRITE"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
