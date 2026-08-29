# Hepta data authority map

> GENERATED FILE — do not hand edit. The normative source is
> `docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json`.

Every authoritative fact belongs to exactly one scope and has one writer. A
projection may be rebuilt from its authority but must never overwrite or reinterpret
that authority.

| Scope | Domain | Authoritative writer | Durable store | Reader / projection rule |
|---|---|---|---|---|
| `runtime_product_graph` | Fleet registry | `supervisor` | `json_registry` | `read_snapshots_only` |
| `runtime_product_graph` | Agent lifecycle | `supervisor` | `json_registry` | `generation_fenced_reads` |
| `runtime_product_graph` | Thread/session state | `app_server` | `agent_private_sqlite` | `submit_through_app_server` |
| `runtime_product_graph` | Memory ledger | `memory_runtime` | `agent_private_sqlite` | `typed_memory_ports` |
| `runtime_product_graph` | Knowledge facts/projection | `memory_runtime` | `agent_private_sqlite` | `projection_is_derived` |
| `runtime_product_graph` | Automation schedules/occurrences | `automation_runtime` | `agent_private_sqlite` | `agentd_exposes_control_only` |
| `runtime_product_graph` | Matrix delivery projection | `matrix_ingress` | `agent_private_sqlite` | `submits_sessions_through_agentd` |
| `runtime_product_graph` | Runtime health/readiness | `agentd` | `process_memory` | `descriptive_ephemeral_observation` |
| `external_control` | Release and promotion | `supervisor` | `signed_release_state` | `no_inferred_promotion` |
| `qualification_evidence` | Governance/provider evidence | `evidence_subsystem` | `evidence_sqlite` | `append_only_receipts` |

## Cross-owner mutation rule

A source owner performs one local transaction:

```text
append operation intent
→ mutate source-owned state
→ append typed outbox envelope
→ commit
```

Delivery is at least once. Before committing, the destination verifies the exact
operation/idempotency identity, binding and payload digests, source/destination
owners, authority epoch, owner epoch, generation, fencing token, and sequence. It
commits only destination-owned state plus a digest-bound acknowledgement; the
source adopts that acknowledgement in a later local transaction.

## Prohibited patterns

- Agentd becoming the writer for Memory, KG, Automation, Matrix, or Evidence;
- two stores being described as one atomic commit;
- a projection updating its source ledger;
- blind retry after a delivery or provider boundary may have been crossed;
- qualification fixtures writing product state;
- booleans or receipts being converted into capabilities;
- logs, model text, or disconnects being treated as proof of an external effect.
