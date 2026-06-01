# Hepta Memory, Intelligence, and KG Full Enablement Activation Readiness Gate

This gate is the first total-ordering checkpoint for moving the memory, Hepta Intelligence, and KG stack from connected/read-only/report-only coverage toward operator-approved live enablement.

It does not activate live writes, prompt preview, context injection, external KG adapters, provider calls, credential reads, service restarts, source command migration, or public release claims.

## Contract

- Script: `scripts/hepta-memory-intelligence-kg-full-enablement-activation-readiness-gate.sh`
- Gate: `hepta_memory_intelligence_kg_full_enablement_activation_readiness_gate`
- Schema: `memory_intelligence_kg_full_enablement_activation_readiness_v1`
- Mode: `operator_requested_full_enablement_readiness_no_live_side_effects`
- Status: `ready`

## Source Evidence

The gate composes the existing proof chain instead of replacing it:

- `hepta-memory-intelligence-closure.sh` proves the active Hepta service stack consumes memory/intelligence crates while `hepta-core` keeps its dependency boundary clean.
- `hepta-kg-prompt-preview-terminal-next-action-activation-denial-summary-gate.sh` is the required preceding preflight marker that proves the KG prompt-preview chain is linked, blocked, report-only, and safe by default. This gate references that marker instead of recursively rerunning the KG chain.
- `/api/hepta-memory-capability-absorption-inventory` proves 14/14 memory capability surfaces are absorbed or represented while live mutation remains disabled.
- `/api/hepta-core-fusion-readiness` proves core fusion is complete for the active `hepta-cli` binary and has no remaining direct Codex dependency blockers.
- Rust contract references cover memory cutover, provider-router activation, live-turn dispatch, live-turn preflight, KG context-injection readiness, KG prompt-preview preflight, and runtime phase-2 memory/intelligence readiness. The main preflight cargo check keeps these contracts compiling before this shell gate runs.

## Readiness Lanes

The gate reports six lanes as ready for operator-approved activation slicing:

- `memory_store_live_mutation`
- `hepta_intelligence_live_context`
- `kg_context_handoff_prompt_preview`
- `kg_external_adapter_staging`
- `runtime_provider_router_context_attachment`
- `rollback_observability_receipts`

Each lane remains non-live in this gate. The readiness result means the lane has enough source evidence to receive a bounded activation packet later.

## Non-Activation Guarantees

The report keeps the following actions blocked and records every side effect as false:

- memory store mutation
- context attachment
- prompt preview or prompt payload materialization
- context injection
- model/provider invocation
- external KG adapter read
- network/database write
- live KG write
- credential read
- channel delivery
- gateway route/source command migration
- active runtime wiring or service restart
- release/public GA claim

## Next Slices

The allowed next work is deliberately narrow:

- maintain the route-count-aware runtime readiness endpoint for this gate
- bind memory live mutation to an operator-approval-bound staging fixture shape without recording, persisting, materializing, or executing it
- stage KG external adapter credentials and rollback receipts without live writes
- only then accept a bounded prompt-preview/context-handoff activation packet
