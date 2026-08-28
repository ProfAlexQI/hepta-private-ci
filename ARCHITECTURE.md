# Hepta current product architecture

This file is the human entry point for the current Hepta product architecture.
The machine-readable authority is
[`docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json`](docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json).
Rust enforces the authority, product graph, and cross-owner operation contracts in
`codex-rs/hepta-contracts`.

External plan snapshots, qualification receipts, historical status documents,
and Draft pull requests are evidence or development inputs. They are not the
current product architecture and cannot grant runtime authority.

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

Qualification plane
  └─ observes product artifacts and runtime evidence only
     never becomes a dependency or authority source of the product graph
```

Agentd owns no durable product domain. It validates one closed authority profile,
constructs one acyclic product graph, holds the process writer lock, starts the
owned services, monitors generation fencing, and shuts the composition down.

## Authority model

Runtime calls use typed capabilities from `codex-hepta-contracts`.
`AuthorityStatus`, qualification booleans, source receipts, or a serialized
claim are descriptive only and cannot become `Authorized<C>`.

The currently constructible profiles are closed:

- `snapshot_read_only`: memory reads only;
- `agent_local`: session serving, memory reads, memory-federation control, and
  automation control;
- `qualification_cognitive_write`: the Agent-local profile plus a typed,
  build-time-only cognitive writer capability.

None of these profiles grants model invocation, provider dispatch, external
effects, fleet mutation, operator acceptance, promotion, or release.

## Data ownership

Each durable domain has exactly one writer. Cross-owner changes use a durable
command/outbox/acknowledgement protocol and never dual-write another owner's
store. The canonical map is documented in
[`docs/architecture/DATA_AUTHORITY_MAP.md`](docs/architecture/DATA_AUTHORITY_MAP.md).

## Recovery

Recovery establishes release and lifecycle identity before opening Agent-private
stores. Product composition occurs only after migration, integrity, operation,
and outbox checks. The complete ordering and fail-closed states are documented
in [`docs/architecture/RECOVERY_ORDER.md`](docs/architecture/RECOVERY_ORDER.md).

## Qualification identities

Two evidence identities are intentionally separate:

1. **source head** — proves the exact proposed commit and tree;
2. **merge candidate** — proves the candidate combined with its target branch.

A green source-only verifier, queued job, `runner_id=0`, `steps=[]`, generated
artifact, or qualification fixture is not product activation evidence.
