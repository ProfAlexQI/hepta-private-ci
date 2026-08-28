# Hepta recovery order

Recovery is ordered by authority. A later layer must not start merely because
its process or database can be opened.

## 1. Release identity

Verify the executable, release manifest, profile, schema compatibility, and
signed release identity. Failure leaves the runtime stopped.

## 2. Supervisor registry and lifecycle

Verify Fleet roots, workspace bindings, resource budgets, Agent identity,
spawn generation, and the single Agentd writer lock. A stale or mismatched
identity is fenced before any Agent-private service opens.

## 3. Agent-private stores

Open each owner store independently and run its integrity posture:

- canonical path and owner binding;
- SQLite mode and durability posture;
- `quick_check` or stronger domain check;
- migration ledger and checksum verification;
- required schema/index/trigger oracle;
- immutable receipt and row-digest verification where applicable.

A missing optional store may produce a typed degraded service. A store required
by the selected authority profile fails startup closed.

## 4. Operation and outbox recovery

Replay each local operation journal. For every non-terminal operation:

- retry only when evidence proves the destination boundary was not crossed;
- use lookup-only reconciliation after a claimed delivery or uncertain result;
- adopt an exact destination acknowledgement without repeating the command;
- quarantine changed identity, digest, sequence, epoch, generation, or fence;
- never release post-dispatch quota/claims solely because a TTL elapsed.

## 5. Product graph composition

Agentd constructs the canonical acyclic graph using the exact typed authority
grant bound to the Agent identity and spawn generation. The graph must contain
one writer per durable domain and no qualification-plane dependency.

## 6. Execution spine and domain attachment

Start the Codex App Server, then attach Memory and Automation through their
bounded product ports. The App Server home, thread-store mode, queue budget,
and feature profile must match the Agent manifest.

## 7. Ingress adapters

Matrix, UI, CLI, and future ingress adapters start only after the owning Agent
execution spine is ready. They submit through typed ingress seams and do not
call models, tools, stores, or another Agent directly.

## 8. Readiness

Readiness requires the current lifecycle generation, App Server identity, and
all profile-required services. Qualification receipts and historical health
snapshots cannot make a runtime ready.

## Required fault matrix

The executable qualification lane must cover at least:

- process death before local commit;
- process death after local commit but before return;
- process death after delivery claim;
- exact acknowledgement loss and adoption;
- changed replay;
- stale generation and stale fencing token;
- unavailable/corrupt optional Automation store;
- unavailable/corrupt profile-required Cognitive store;
- destination indeterminate outcome and lookup-only recovery;
- disk-full or write failure before commit;
- restart with pending outbox rows;
- terminal operation reopen attempt.
