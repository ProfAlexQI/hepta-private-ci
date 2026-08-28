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
  Service     Service       Service
     ^           |             |
     |       Memory Runtime    +---- typed queue ----+
     |                                           |
     +-------------------------------------------+
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

`codex-hepta-contracts` owns the only typed runtime authority kernel.

Local startup profiles remain closed-world:

- `AuthorityGrant::snapshot_read_only`;
- `AuthorityGrant::agent_local`;
- `AuthorityGrant::qualification_cognitive_write`.

No local profile grants model invocation, provider dispatch, external effect, fleet mutation,
operator acceptance, or release promotion. Runtime configuration and request overrides cannot widen
the selected local grant.

Architecture Convergence P0.2 adds the second, separately fenced path:

```text
externally supplied lease material
  -> AuthorityLeaseBinding shape/subject/generation/expiry checks
  -> CapabilityVerifier
  -> Authorized<C>
```

`Authorized<C>` cannot be deserialized or constructed directly. A local profile may mint only an
action it contains. An external lease may mint only the exact capability requested after an
independent verifier accepts the bound action, Agent, generation, grant digest, authority epoch,
owner epoch, fencing-token digest, and expiry.

The existing `ProductionAuthorityLease` protocol is retained for compatibility. Agentd now adapts it
to `Authorized<CognitiveWriteCapability>` **before** opening the Cognitive store, then the durable
writer repeats the legacy verifier check before lease mutation. This is intentionally a two-check
migration period. The adapter cannot mint `ExternalEffectCapability`, provider dispatch, model
invocation, fleet mutation, operator acceptance, or promotion.

A production outbox target is a separate authority boundary. `AgentdProductionWriterHost` accepts a
target only together with an externally verified `Authorized<ExternalEffectCapability>`. Attachment
and every dispatch require the effect witness to bind the same Agent and generation as the cognitive
writer, and every dispatch rechecks its expiry. The legacy production-lease adapter cannot create
this witness. Therefore cognitive-write authority alone can queue durable intent but cannot cross an
external effect boundary.

## Agentd service boundary

The task-supervision loop now owns process cancellation and task joining only. Product construction
and execution are delegated to explicit services:

- `AgentMemoryService` owns Memory authorization, Agent-private store open, federation discovery,
  attachment, and the optional qualification cognitive-write witness.
- `AgentAutomationService` owns Automation authorization, Agent-private store open, attachment, and
  scheduler execution.
- `AgentAppServerService` owns session-serving authorization and the embedded App Server launch.
- `AgentRuntimeComposition` validates the ProductGraph and constructs those services.

The supervision loop no longer opens `CognitiveStore`, opens `AutomationStore`, discovers federation,
or calls the Automation scheduler directly.

## Memory boundary

Architecture Convergence begins the Memory split at the runtime boundary without moving schema or
writer ownership prematurely:

- `CognitiveRuntime::open_agent_owned` owns the store-open facade.
- `CognitiveRuntime::with_discovered_federation` owns read-only federation discovery.
- `AgentMemoryService` owns lifecycle fencing and attachment around those operations.
- The existing Cognitive SQLite transaction domain remains unchanged until P0/P0.2 qualification is
  executable and green.

Physical crate or table extraction before that qualification remains forbidden. This prevents a
simultaneous schema, package, authority, and recovery migration.

## Canonical supporting documents

- Machine product graph: `docs/architecture/HEPTA_PRODUCT_ARCHITECTURE_V1.json`
- Data authority map: `docs/architecture/HEPTA_DATA_AUTHORITY_MAP_V1.md`
- Recovery order: `docs/architecture/HEPTA_RECOVERY_ORDER_V1.md`
- P0 implementation status: `docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2026-08-28.md`
- P0.2 implementation status: `docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_2026-08-28.md`

## Current claim boundary

Architecture Convergence P0.2 is a development/internal-test implementation slice. It does not
grant or activate:

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

The exact-head GitHub workflow must pass source verification, typed lease-adapter and external-effect
gate tests, real Agent-private SQLite service composition, default and qualification profiles,
all-target check, and strict Clippy before the slice can be called qualified.
