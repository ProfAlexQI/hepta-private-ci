# Hepta architecture convergence plan v1

This is the current execution plan for converging the Hepta product
architecture. It is subordinate to the canonical architecture authority in
`HEPTA_CURRENT_ARCHITECTURE_V1.json`; it cannot grant runtime or promotion
authority.

## State vocabulary

Every tranche uses the same states:

```text
planned
→ source_implemented
→ product_graph_bound
→ executable_qualified
→ operator_accepted
→ promoted
```

`source_implemented` and `product_graph_bound` are not executable
qualification. A queued job, `runner_id=0`, `steps=[]`, source receipt, local
fixture, or generated artifact cannot advance a tranche to
`executable_qualified`.

## P0.1 — Canonical architecture and closed authority kernel

Current state: `source_implemented`.

Delivered source:

- canonical human and machine architecture entry points;
- closed runtime profiles;
- `AuthorityGrant` and typed `Authorized<C>`;
- acyclic product graph;
- one writer per durable data domain;
- Memory runtime authority facade;
- exact source-head and merge-candidate evidence identities.

Remaining acceptance:

- exact-head hosted fmt/test/check/Clippy;
- merge-candidate product integration;
- caller-ratchet review.

## P0.2 — Thin Agentd and physical Memory runtime boundary

Current state: `source_implemented`, pending finalizer materialization.

Delivered source:

- `AgentRuntimeComposition` owns construction and process-bound resources;
- `runtime.rs` consumes composition parts rather than opening domain stores;
- Agentd owns no durable product domain;
- physical `codex-hepta-memory-runtime` facade crate;
- deterministic workspace/lock/Agentd migration;
- default profile cannot acquire cognitive write;
- qualification writer requires typed cognitive-write authority;
- authority grant digest remains bound through the Memory facade.

Exit criteria:

- facade is one product-workspace member and one Cargo.lock package;
- Agentd lock entry contains the facade dependency;
- default and qualification Agentd tests pass;
- Supervisor/Matrixd/Native Gateway compile on the same lock graph;
- no external/model/promotion authority is introduced.

## P0.3 — Legacy production lease attenuation

Current state: `source_implemented`, caller migration incomplete.

Delivered source:

- digest-bound `LegacyProductionLeaseEvidence`;
- external verifier trait;
- owner/generation/expiry validation;
- exact lease-head and verifier-receipt binding;
- attenuation to only `Authorized<CognitiveWriteCapability>`;
- negative tests for stale generation, expiry, and verifier rejection.

Still required:

- adapt the existing `AgentdProductionWriterHost` verifier result into the new
  evidence type;
- prove existing writer lock, lease CAS, epoch, and expiry semantics are
  unchanged;
- remove direct legacy authorization branching from the caller;
- keep model, provider, external effect, operator, and release capabilities
  unavailable.

## P0.4 — Physical Memory bounded-context extraction

Current state: `runtime_facade_source_implemented`.

Extraction order:

1. `hepta-memory-runtime` — Agent composition boundary;
2. `hepta-memory-model` — stable IDs, scopes, revisions, receipts;
3. `hepta-memory-store` — source/memory ledger and migration ownership;
4. `hepta-kg` — immutable facts and projection planning;
5. `hepta-retrieval` — lexical/vector/KG query and fusion ports;
6. `hepta-compact` — compact lease/checkpoint/rehydration;
7. `hepta-learning` — trajectory, intuition, neuron proposals;
8. reduce legacy `hepta-memory` to compatibility reexports, then delete it.

Rules:

- one Agent-private database owner during each migration;
- no duplicate SQLx migration lineage;
- no cross-crate transaction described as atomic;
- callers migrate through stable facades before source moves;
- every extraction preserves schema and receipt digests unless a separately
  reviewed migration changes them.

## P0.5 — Cross-owner Unit of Work migration

Current state: `contract_source_implemented`.

Delivered source:

- `OperationBinding`;
- idempotency and owner identity;
- authority/owner epoch, generation, fence, payload, and size binding;
- transactional outbox envelope;
- exact destination acknowledgement;
- fixed operation transition graph;
- lookup-only recovery after a crossed or uncertain boundary.

Caller migration order:

1. Automation dispatch to App Server;
2. Matrix ingress to Agentd/App Server;
3. provider effect coordinator to evidence store;
4. Memory/KG projection publication where ownership becomes separate;
5. release/promotion commands from operator control plane.

Each migrated caller must prove no blind retry, changed replay rejection,
terminal immutability, and exact acknowledgement adoption.

## P0.6 — Product fault qualification

Current state: `portable_file_fault_source_implemented`.

The current durable fault matrix covers:

- death before write;
- death after sync before publish;
- acknowledgement loss after publish;
- simulated disk full;
- truncation and corruption;
- stale generation;
- changed payload;
- terminal reopen.

Still required on actual product stores:

- SQLite transaction failpoints for Memory, Automation, Evidence, and Matrix;
- process kill/reopen around WAL commits;
- filesystem full and permission loss;
- pending outbox restart;
- stale Supervisor generation during store open and delivery;
- backup/restore consistency drill;
- Windows and Linux exact-head receipts.

## P0.7 — Delivery closure

Before merge:

- all contents-write bootstrap workflows are retired to read-only provenance
  stubs;
- exact source-head and merge-candidate gates are independently green;
- the branch contains no queued-job or source-only PASS overclaim;
- the machine status records all hosted run/job/artifact identities;
- Draft remains Draft until executable evidence review completes;
- operator acceptance and promotion remain separate later decisions.

## Global authority boundary

Throughout P0:

```text
production_caller=false
production_writer=false
effect_authority=false
external_effect=false
model_invocation_authority=false
provider_dispatch_authority=false
fleet_mutation_authority=false
operator_acceptance=false
promotion=false
release=false
```
