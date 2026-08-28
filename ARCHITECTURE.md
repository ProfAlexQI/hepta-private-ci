# Hepta product architecture

This file is the single human-readable entry point for the current Hepta product architecture.
Historical development plans, qualification receipts, Dropbox snapshots, and Draft pull requests
remain useful provenance, but they do not override this file or the machine-readable product graph
at `docs/architecture/HEPTA_PRODUCT_ARCHITECTURE_V1.json`.

## Current architecture profile

Hepta is a **modular Agent runtime with a separate lifecycle control plane**. It is not a collection
of independent product kernels.

```text
Supervisor / Fleet Registry
          |
          | lifecycle generation + process fence
          v
       Agentd composition root
       /        |          \
 App Server  Memory      Automation
     ^        Runtime       Runtime
     |                         |
     +------ typed queue ------+
     ^
     |
 Matrix / UI / future ingress adapters
```

The invariant is one owner per state class:

- Supervisor owns fleet registration and Agent lifecycle generations.
- App Server owns threads, turns, sessions, model/tool admission, and the persistent thread queue.
- Memory Runtime owns the Agent-private memory ledger and deterministic knowledge projection.
- Automation Runtime owns Agent-private schedules, occurrences, and automation leases.
- Matrix ingress owns only its durable ingress projection and submits through Agentd/App Server.
- Agentd is the composition root and health/lifecycle boundary; it does not own a second session,
  workflow, model, tool, or fleet kernel.
- Qualification observes and validates the real product graph; it is never a product dependency or
  a source of runtime authority.

## Unified authority kernel

`codex-hepta-contracts::AuthorityGrant` is the only positive runtime authority input introduced by
Architecture Convergence P0. The current constructors are closed-world profiles:

- `snapshot_read_only`;
- `agent_local`;
- `qualification_cognitive_write`.

No current profile grants model invocation, provider dispatch, external effect, fleet mutation,
operator acceptance, or release promotion. Typed `Authorized<C>` values can only be minted after an
`AuthorityGrant` admits the corresponding action.

Compile-time feature selection may choose between the Agent-local and qualification cognitive-write
profiles, but runtime configuration and request overrides cannot widen the selected grant.

## Memory boundary

Architecture Convergence P0 begins the Memory split at the runtime boundary without moving schema or
writer ownership prematurely:

- Agentd no longer opens `CognitiveStore` or discovers federation directly.
- `CognitiveRuntime::open_agent_owned` owns the store-open boundary.
- `CognitiveRuntime::with_discovered_federation` owns read-only federation discovery.
- Agentd retains composition and generation fencing.
- The existing Cognitive SQLite transaction domain remains unchanged until this facade is qualified.

Physical crate extraction follows after the facade and product graph qualification are green; this
avoids a simultaneous schema, package, and authority migration.

## Canonical supporting documents

- Machine product graph: `docs/architecture/HEPTA_PRODUCT_ARCHITECTURE_V1.json`
- Data authority map: `docs/architecture/HEPTA_DATA_AUTHORITY_MAP_V1.md`
- Recovery order: `docs/architecture/HEPTA_RECOVERY_ORDER_V1.md`
- P0 implementation status: `docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2026-08-28.md`

## Current claim boundary

Architecture Convergence P0 is a development/internal-test implementation slice. It does not grant:

```text
production_caller
production_writer
model_invocation
provider_dispatch
external_effect
fleet_mutation
operator_acceptance
promotion
```

The exact-head GitHub workflow must pass source verification, real Agent-private SQLite composition,
package tests, check, and strict Clippy before the slice can be called qualified.
