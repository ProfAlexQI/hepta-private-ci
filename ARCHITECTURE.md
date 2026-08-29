# Hepta current product architecture

> Generated from `docs/architecture/HEPTA_ARCHITECTURE_CATALOG_V1.json`.
> Change the catalog and run `python3 scripts/generate-hepta-architecture-views.py --write`;
> CI accepts only byte-identical `--check` output.

The catalog is the sole editable architecture fact source. Rust remains the
runtime enforcement for typed authority, the Agent product graph and cross-owner
operations. Historical plans, qualification receipts and Draft pull requests are
evidence or development inputs; none can mint runtime authority.

## Runtime topology

```text
Supervisor control plane
  │ lifecycle, release identity, signed grants
  ▼
Agentd composition root
  ├─ Codex App Server       — thread/session owner
  ├─ Hepta Memory Runtime   — memory/KG owner
  ├─ Hepta Automation       — schedule/occurrence owner
  └─ local ingress adapters — typed submissions only

Evidence store / qualification plane
  └─ outside the Agent product graph; observes candidate-bound evidence only
```

Agentd owns no durable product domain. It validates one closed authority
profile, constructs the acyclic product graph, holds the process writer lock,
binds services, monitors lifecycle generation and shuts down the composition.

## Runtime profile availability

| Profile | Required services | Optional services | Disabled services | Qualification prerequisites | Qualification only |
|---|---|---|---|---|---|
| `snapshot_read_only` | `supervisor`, `memory_runtime` | none | `agentd`, `app_server`, `automation_runtime`, `matrix_ingress`, `provider_effect_adapter`, `evidence_store`, `qualification_plane` | none | no |
| `agent_local` | `supervisor`, `agentd`, `app_server` | `memory_runtime`, `automation_runtime` | `matrix_ingress`, `provider_effect_adapter`, `evidence_store`, `qualification_plane` | none | no |
| `qualification_cognitive_write` | `supervisor`, `agentd`, `app_server`, `memory_runtime` | `automation_runtime` | `matrix_ingress`, `provider_effect_adapter`, `evidence_store`, `qualification_plane` | `qualification_plane` | yes |

A missing required service fails startup closed. A missing optional service
produces an explicit degraded state. A disabled service appearing in the runtime
is a configuration error; source presence never activates it.

## Authority

Runtime calls consume typed `Authorized<C>` witnesses. Serialized claims,
qualification booleans and source receipts are descriptive only. Current local
profiles cannot grant model invocation, provider dispatch, external effects,
fleet mutation, operator acceptance, promotion or release.

## Data ownership

The generated `docs/architecture/DATA_AUTHORITY_MAP.md` distinguishes the
Agent product graph from external control/evidence planes. Every domain has one
writer and one migration owner. Cross-owner changes use durable
command/outbox/acknowledgement semantics and never dual-write another owner.

## Recovery

Recovery establishes release and lifecycle identity before opening Agent-private
stores, then validates migrations, integrity, operation journals and outboxes
before composition. See `docs/architecture/RECOVERY_ORDER.md`.

## Qualification identities

Source head and merge candidate are separate evidence identities. A queued job,
`runner_id=0`, `steps=[]`, generated artifact or source-only verifier is not
executable qualification. Operator acceptance and promotion are later,
independently issued decisions.
