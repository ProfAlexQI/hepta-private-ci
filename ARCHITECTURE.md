# Hepta current product architecture

> GENERATED FILE — do not hand edit. The normative source is
> `docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json`; regenerate with
> `python3 scripts/generate-hepta-architecture-projections.py`.

## Authority and scope

The V2 model is the only editable product-architecture source. Rust contracts
and generated human/machine projections are checked against it in CI. Historical
plans, qualification receipts, Draft pull requests, and captured external documents
are evidence or implementation inputs; they cannot grant runtime or release authority.

Architecture data is deliberately split into three scopes:

- `runtime_product_graph`: domains represented by Rust `ProductGraph`;
- `external_control`: Supervisor-owned release and promotion authority;
- `qualification_evidence`: append-only evidence outside the product dependency graph.

## Runtime topology

```text
Supervisor control plane
  │ lifecycle, release identity, signed grants
  ▼
Agentd composition root
  ├─ Codex App Server       — thread/session owner
  ├─ Hepta Memory Runtime   — memory/KG owner
  ├─ Hepta Automation       — schedule/occurrence owner
  └─ local ingress adapters — submit through typed product seams

Qualification plane / Evidence subsystem
  └─ read product artifacts and runtime evidence only
     never become product dependencies or runtime authority sources
```

Agentd owns no durable product domain. It validates one closed authority profile,
constructs one acyclic product graph, binds the runtime-profile contract, retains
the process writer lock, starts owned services, monitors generation fencing, and
shuts the composition down.

## Runtime profiles

| Profile | Allowed actions | Required services | Optional services | Disabled services |
|---|---|---|---|---|
| `snapshot_read_only` | `read_memory` | `supervisor`, `memory_runtime` | none | `agentd`, `app_server`, `automation_runtime`, `matrix_ingress`, `provider_effect_adapter` |
| `agent_local` | `serve_session`, `read_memory`, `mutate_memory_federation`, `mutate_automation` | `supervisor`, `agentd`, `app_server` | `memory_runtime`, `automation_runtime`, `matrix_ingress` | `provider_effect_adapter` |
| `qualification_cognitive_write` | `serve_session`, `read_memory`, `mutate_memory_federation`, `mutate_automation`, `write_cognitive_state` | `supervisor`, `agentd`, `app_server`, `memory_runtime` | `automation_runtime` | `matrix_ingress`, `provider_effect_adapter` |

Required services fail startup closed and gate readiness. Optional services may
enter a typed degraded state and never gate readiness. Disabled services must not
start. The dormant provider-effect boundary cannot be activated by any local profile.

## Product components

| Component | Plane | Role | Product graph | Durable domains |
|---|---|---|---:|---|
| `supervisor` | `control` | `lifecycle_release_and_grant_owner` | yes | `fleet_registry`, `agent_lifecycle`, `release_promotion` |
| `agentd` | `composition` | `thin_composition_root` | yes | none |
| `app_server` | `execution` | `session_execution_spine` | yes | `thread_session` |
| `memory_runtime` | `domain` | `memory_and_knowledge_owner` | yes | `memory_ledger`, `knowledge_projection` |
| `automation_runtime` | `domain` | `automation_owner` | yes | `automation_schedule` |
| `matrix_ingress` | `adapter` | `matrix_delivery_owner` | yes | `ingress_projection` |
| `provider_effect_adapter` | `adapter` | `dormant_external_effect_boundary` | yes | none |
| `qualification_plane` | `qualification` | `read_only_evidence_consumer` | no | none |
| `evidence_subsystem` | `qualification` | `append_only_governance_and_provider_evidence_owner` | no | `governance_provider_evidence` |

## Data ownership

Every authoritative domain has exactly one writer. Cross-owner changes use the
digest-bound transactional outbox and acknowledgement protocol; no component
dual-writes another owner's store. See `docs/architecture/DATA_AUTHORITY_MAP.md`.

## Recovery

Recovery establishes release and lifecycle identity before opening Agent-private
stores, reconciles operations and outboxes before composition, and starts ingress
only after the execution spine is ready. See `docs/architecture/RECOVERY_ORDER.md`.

## Qualification identities

Source head and merge candidate are separate evidence identities. A source-only
verifier, queued job, zero runner, empty steps, generated artifact, or fixture is
not executable qualification. Operator acceptance, promotion, and release remain
independently issued external gates.
