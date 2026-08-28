# Hepta Intelligence P0.4c — Shadow Host Orchestration Adapter

**Date:** 2026-08-28  
**Status:** `SOURCE_ONLY / STACKED_DRAFT / ACTIVATION_BLOCKED`  
**Base branch:** `codex/hepta-intelligence-mutation-journal-v4b-r2-20260828`  
**Base commit:** `a5de899e9f6a97fa0803cc1e73d664be8b7d68bc`  
**Development branch:** `codex/hepta-intelligence-shadow-host-adapter-v4c-20260828`

## 1. Purpose

P0.4a defines the only legal typed transition graph. P0.4b persists and replays that graph in an opt-in SQLite journal. P0.4c adds an explicit host-facing adapter that can record receipts already observed by a qualification host.

It does **not** execute the represented operation. The adapter never appends product source data, writes memory/KG facts, refreshes the active projection, settles a real outbox, invokes a tool, sends model input, dispatches a provider effect, or changes Agentd startup.

```text
runtime_wired=false
default_open_wired=false
app_runtime_attached=false
tool_registered=false
memory_write_authority=false
projection_write_authority=false
outbox_dispatch_authority=false
external_effects=false
production_authority=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

P0.1–P0.4b remain `qualified=false` because their exact hosted jobs have not received runners. Source work is allowed on a separate stacked Draft, but activation is blocked.

## 2. Core shadow adapter

`codex-rs/hepta-memory/src/intelligence_mutation_shadow_host.rs` adds public inherent methods on `CognitiveStore` while keeping the private P0.4a/P0.4b implementation types hidden:

```text
open_with_shadow_intelligence_mutation_host
begin_shadow_intelligence_mutation
prepare_shadow_intelligence_mutation_observation
append_shadow_intelligence_mutation_observation
observe_shadow_intelligence_mutation
inspect_shadow_intelligence_mutation
```

The default `CognitiveStore::open` path is unchanged.

### Binding contract

Each operation binds:

```json
{
  "operation_id": "...",
  "lease_id": "...",
  "lease_epoch": 1,
  "expected_revision": null,
  "starting_projection_generation": 4,
  "causal_root_sha256": "<64 lowercase hex>"
}
```

The P0.4b immutable operation ledger rejects changed binding replay.

### Observation contract

The host may record only typed observations:

```text
source_witnessed
grounding_validated
durable_intent_appended
memory_facts_committed
projection_published
outbox_settled
terminalized
indeterminate
reconciled_applied
reconciled_not_applied
quarantined
```

Every evidence-bearing observation carries a lowercase SHA-256 digest. Projection observations also carry the expected previous generation and the next generation.

These names describe facts the host claims it already observed. They do not grant the adapter permission to perform the corresponding product mutation.

## 3. Prepare/append retry protocol

`prepare_shadow_intelligence_mutation_observation` freezes:

```text
operation binding
next sequence
causal parent transition digest
observation payload
prepared request digest
negative authority flags
```

The caller should persist the exact prepared JSON before append. Reusing it after acknowledgement loss produces P0.4b `Replay` with the original transition digest. Changing sequence, evidence, binding, parent, observation, or authority flags invalidates the prepared digest or is rejected by typed replay.

The convenience `observe` method returns both the exact prepared request and the append receipt. It is intended for bounded qualification tests, not as a production retry store.

## 4. Product-effect isolation

The core source contains no calls to:

```text
append_source
remember_with_*
correct_with_*
refresh_scope_projection
ProductionDurableWriter
ProductionOutboxDispatcher
ToolContributor
physical-send
```

Focused tests assert that after a complete shadow path:

```text
source_ledger rows = 0
memory_revisions rows = 0
kg_projection rows = 0
```

Only the opt-in operation and transition journal receives rows.

Receipts distinguish observed counters from adapter effects:

```text
observed_memory_write_count
observed_projection_publish_count

memory_write_performed_by_adapter=false
projection_write_performed_by_adapter=false
outbox_dispatch_performed_by_adapter=false
```

## 5. Agentd seam

`codex-rs/hepta-agentd/src/shadow_intelligence_mutation_host.rs` provides `AgentdShadowIntelligenceMutationHost`.

The seam is available only when built with:

```text
--features qualification-intelligence-mutation-shadow
```

`default = []` remains unchanged. Agentd `runtime.rs` and `app_runtime.rs` do not construct, attach, register, or retain this handle.

`open` binds the store owner to the exact `AgentdIdentity.agent_id`. Every returned envelope also binds:

```text
agent_id
spawn_generation
action
payload_sha256
host_receipt_sha256
```

The Agentd envelope repeats all negative authority flags and never exposes a dispatcher target.

## 6. Tests

Core focused tests cover:

- full normal observation path;
- exact prepared-request replay;
- prepared-request tamper rejection;
- post-commit acknowledgement loss and exact adoption;
- zero product source/memory/projection rows;
- authority-negative receipts.

Agentd focused tests cover:

- deterministic spawn-bound envelope digest;
- zero spawn-generation fencing;
- runtime/app/tool/effect authority flags remaining false.

## 7. Qualification

Source gate:

```bash
python3 scripts/verify-hepta-intelligence-shadow-host.py
```

Required executable Rust qualification:

```bash
cd codex-rs
cargo fmt --all -- --check
cargo test -p codex-hepta-memory shadow_intelligence_mutation_host -- --nocapture
cargo test -p codex-hepta-agentd \
  --features qualification-intelligence-mutation-shadow \
  shadow_intelligence_mutation_host -- --nocapture
cargo test -p codex-hepta-memory
cargo test -p codex-hepta-agentd \
  --features qualification-intelligence-mutation-shadow
cargo clippy -p codex-hepta-memory --all-targets -- -D warnings
cargo clippy -p codex-hepta-agentd \
  --features qualification-intelligence-mutation-shadow \
  --all-targets -- -D warnings
```

The dedicated workflow must use repository Rust `1.95.0` and upload the exact source receipt.

## 8. Exit gate

P0.4c may become `qualified=true` only after one exact head has:

- source gate PASS;
- memory and Agentd formatting PASS;
- focused and full tests PASS;
- strict clippy PASS;
- P0.1–P0.4b executable exact-head qualification;
- readable P0.4b crash-window artifacts;
- review proving zero default runtime/App Server/tool registration changes;
- candidate freeze and operator review.

Even after qualification, activation is a separate CALLERS decision. P1.1 Hybrid Retrieval v2 remains blocked until P0 authority, rollback, and runtime-boundary gates are satisfied.
