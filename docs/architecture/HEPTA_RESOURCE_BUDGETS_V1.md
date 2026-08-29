# Hepta runtime resource budgets V1

**Status:** normative P0.8c contract. Values are development defaults until an
exact candidate qualifies platform-specific limits.

| Resource | Default bound | Admission owner | Terminal release owner | Over-limit behavior |
|---|---:|---|---|---|
| concurrent turns per Agent | 2 | Agentd/App Server composition | turn terminalizer | reject or durable queue |
| queued turns per Agent | 64 | App Server queue | queue reconciler | bounded rejection |
| concurrent tool processes | 4 | tool executor | process reaper | deny spawn |
| concurrent model requests | 2 | model adapter | terminal/indeterminate recorder | backpressure |
| concurrent provider effects | 1 per operation key | provider adapter | reconcile terminalizer | conflict or wait |
| Automation runnable leases | 8 | Automation scheduler | lease terminalizer | defer wakeup |
| TaskFlow active steps | 32 | TaskFlow owner | step terminalizer | pause run |
| Matrix in-flight deliveries | 16 | Matrix store/runtime | ACK/reconcile | durable backlog |
| Memory retrieval candidates | 256 | retrieval planner | request completion | deterministic truncate |
| Memory returned items | 32 | retrieval ranker | request completion | deterministic truncate |
| context fragment bytes | 1 MiB per turn | context composer | turn completion | omit lowest-priority refs |
| Agent cognitive DB | 4 GiB soft / 5 GiB hard | CognitiveStore | compactor/operator | compact then reject writes |
| Automation DB | 1 GiB hard | AutomationStore | operator | reject new durable work |
| Matrix DB | 2 GiB hard | MatrixStore | operator | stop ingress, retain reconcile |
| evidence DB | 4 GiB hard | EvidenceStore | retention operator | stop new effects before losing evidence |
| total outbox pending rows | 100,000 per Agent | each owner | dispatcher/reconciler | stop new effectful admission |
| one protocol frame | 64 KiB unless stricter | receiving service | receiver | close/reject frame |

## Accounting rules

1. Admission and reservation are one local transaction where possible.
2. A reservation has an operation ID, owner epoch, generation and fence.
3. Success, failure, cancellation and reconciled terminal state release exactly
   once. `Indeterminate` does not release effect capacity unless the provider
   contract proves no active effect.
4. Crash recovery reconstructs reservations from durable state; memory counters
   are never authoritative.
5. N+1 tests exist for every bound.
6. Bounds are included in the runtime fence and instance snapshot so a changed
   budget requires a new runtime generation.
7. Soft disk limits may trigger compaction; hard limits fail closed before an
   accepted operation loses its durable intent or evidence.

## Required telemetry

Expose bounded, non-secret metrics for current reservations, queue depth,
outbox age, WAL size, reconciliation age, rejection count and recovery count.
Metrics are observations only and cannot mutate budgets or authority.
