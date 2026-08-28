# Hepta Architecture Convergence P0.2

## Exact stack

- Parent Draft PR: `#47` — Architecture Convergence P0.1.
- Parent branch: `codex/hepta-architecture-convergence-p0-20260828`.
- Parent exact head: `03160d3e20a4972085e4ae31186a6c7e59e7a50c`.
- P0.2 branch: `codex/hepta-authority-adapter-services-p0-2-20260828`.
- Frozen source candidate: `673b13be84220bab37f87f1048e305974e54a75c`.
- Frozen source tree: `fd7839147162fc15b62da51fab21321b4ff2aa07`.
- Relationship to parent: strictly ahead, zero commits behind.

## Implemented authority migration

The unified authority kernel is now schema v2 and supports an externally verified capability path:

```text
AuthorityLeaseBinding
  -> subject / generation / epoch / fence / expiry validation
  -> CapabilityVerifier(action, exact binding)
  -> Authorized<C>
```

`Authorized<C>` remains non-deserializable and cannot be directly constructed by product callers.
Local `AuthorityGrant` profiles remain closed-world and grant no model, provider, external-effect,
fleet, operator, or promotion action.

The existing `ProductionAuthorityLease` is retained as a compatibility input. Agentd converts it to
`Authorized<CognitiveWriteCapability>` only after the mandatory legacy verifier accepts the exact
Agent-bound lease. This typed verification occurs before Agentd opens the Cognitive store. The
existing durable writer repeats its verifier check before any lease/event/outbox mutation.

The legacy adapter cannot mint external-effect authority. A production target can be attached only
with a separately verified `Authorized<ExternalEffectCapability>`. Target attachment and every
subsequent dispatch require:

- an external lease-backed capability;
- the same Agent as the cognitive writer;
- the same generation as the cognitive writer;
- an unexpired effect lease.

Thus cognitive-write authority can append a durable intent but cannot cross an external-effect
boundary by itself.

## Agentd service extraction

The product supervision loop now supervises service objects rather than constructing raw stores:

- `AgentMemoryService` owns Memory capability binding, generation-fenced store open, federation
  discovery, store attachment, and optional qualification cognitive-write capability retention.
- `AgentAutomationService` owns Automation mutation capability binding, generation-fenced store
  open, attachment, degradation, and scheduler execution.
- `AgentAppServerService` owns typed session-serving capability and embedded App Server launch.
- `AgentRuntimeComposition` validates the canonical ProductGraph and constructs all three services.

The normal Agentd supervision loop no longer calls:

```text
CognitiveStore::open
AutomationStore::open
FederatedRecallSet::discover
run_automation_scheduler
```

directly.

## Memory split boundary

No Cognitive SQLite schema, migration lineage, table ownership, or transaction boundary was moved in
P0.2. P0.1 exact-head hosted qualification still has no executable runner evidence, so physical
Memory crate/table extraction remains fail-closed and explicitly forbidden by the machine
architecture manifest.

## Required executable qualification

The exact final PR head must execute with non-empty runner steps:

```text
python source/architecture verifier
package-scoped rustfmt
codex-hepta-contracts authority tests
legacy production-authority adapter tests
typed external-effect dispatch-gate tests
full contracts/memory/agentd library tests
qualification-cognitive-write profile tests
all-target check
strict Clippy -D warnings
```

A queued workflow, `runner_id=0`, `steps=[]`, source presence, or this document is not a qualification
receipt.

## Current claim

```text
SOURCE_PRESENT
EXECUTABLE_QUALIFICATION_PENDING
NO_RUNTIME_ACTIVATION
NO_PRODUCTION_AUTHORITY
NO_EXTERNAL_EFFECT
NO_PHYSICAL_MEMORY_EXTRACTION
```

Keep the PR Draft. Do not merge independently of P0.1, remove the legacy verifier before a paired
migration receipt exists, attach a production target without a distinct typed effect capability,
move Cognitive schema ownership, or interpret source-only evidence as runtime qualification.
