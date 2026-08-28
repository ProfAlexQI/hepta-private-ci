# Hepta data authority map

Every durable fact has one authoritative writer. A projection may be rebuilt
from its authority but must never overwrite or reinterpret that authority.

| Domain | Authoritative writer | Durable store | Other components |
|---|---|---|---|
| Fleet registry | Supervisor | control-root JSON registry | read snapshots only |
| Agent lifecycle | Supervisor | control-root lifecycle state | generation-fenced reads |
| Release and promotion | Supervisor with operator authority | signed release state | no inferred promotion |
| Thread/session state | Codex App Server | Agent-private SQLite | submit through App Server |
| Memory ledger | Memory Runtime | Agent-private cognitive SQLite | typed read/write ports |
| Knowledge facts/projection | Memory Runtime KG boundary | Agent-private cognitive SQLite | projection is derived |
| Automation schedules/occurrences | Automation Runtime | Agent-private automation SQLite | Agentd exposes control only |
| Matrix delivery projection | Matrix ingress/store | Agent-private Matrix SQLite | submits sessions through Agentd |
| Runtime health/readiness | Agentd | process memory/event buffer | descriptive, not durable authority |
| Governance/provider evidence | Evidence subsystem | evidence SQLite | append-only receipts |

## Cross-owner mutation rule

A source owner performs one local transaction:

```text
append operation intent
→ mutate source-owned state
→ append typed outbox envelope
→ commit
```

Delivery is at least once. Before committing, the destination verifies:

```text
operation_id
idempotency_key
binding_sha256
payload_sha256
source owner
destination owner
authority epoch
owner epoch
generation
fencing token
```

It then commits only destination-owned state and a digest-bound acknowledgement.
The source adopts that acknowledgement in a later local transaction.

## Prohibited patterns

- Agentd becoming the writer for Memory, KG, Automation, Matrix, or Evidence;
- two stores being described as one atomic commit;
- a projection updating its source ledger;
- blind retry after a delivery or provider boundary may have been crossed;
- qualification fixtures writing product state;
- booleans such as `production_writer=true` being treated as a capability;
- logs, model text, or transport disconnect being treated as proof of an
  external effect.
