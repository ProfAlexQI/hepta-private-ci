# Hepta data authority map v1

This document defines the current single-writer boundary. A cache, projection, receipt, or
qualification artifact cannot write back into its source owner.

| Data domain | Single durable writer | Durable store | Other components may |
|---|---|---|---|
| Fleet registry | Supervisor/Fleet Registry | Canonical JSON registry | Read exact manifests and roots |
| Agent lifecycle/generation | Supervisor/Fleet Registry | Canonical lifecycle JSON | Observe and fail closed on drift |
| Thread/turn/session/queue | Codex App Server | Agent-private thread SQLite | Submit typed queue requests |
| Memory source/revision ledger | Memory Runtime | Agent-private Cognitive SQLite | Propose, recall, cite, revalidate |
| Knowledge graph projection | Memory Runtime deterministic projector | Same Cognitive SQLite transaction domain | Read typed nodes/edges; never infer writes from prose |
| Automation schedule/occurrence/lease | Automation Runtime | Agent-private Automation SQLite | Submit due occurrence to App Server |
| Matrix ingress/outbox projection | Matrix ingress | Agent-private Matrix SQLite | Submit canonical message through Agentd/App Server |
| Agent runtime health/events | Agentd | Bounded process memory | Read through local control protocol |
| Governance/provider evidence | Evidence owner | Evidence SQLite | Reference immutable receipt digests |
| Qualification evidence | Qualification workflow | GitHub artifact/receipt | Observe only; never grant runtime authority |

## Cross-domain mutation rule

Cross-domain work uses `local transaction + typed outbox + idempotent delivery + acknowledgement`.
There is no shared universal database and no direct dual write. Unknown external outcomes remain
`Indeterminate` until the owning effect ledger reconciles them.

## Current Cognitive SQLite boundary

Memory, KG projection, compact journal, local lease/outbox, logical-turn registry, and H7 trajectory
currently share one Agent-private Cognitive SQLite transaction domain. Architecture Convergence P0
does not split that database. It first isolates the public runtime facade and proves the real Agentd
composition. Physical package/table extraction requires a later migration plan, backup/restore proof,
and exact-head qualification.
