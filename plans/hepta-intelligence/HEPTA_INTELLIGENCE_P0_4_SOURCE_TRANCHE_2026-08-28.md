# Hepta Intelligence P0.4 Source Tranche

**Date:** 2026-08-28  
**Status:** `SOURCE_ONLY / STACKED_DRAFT / RUNTIME_WIRING_BLOCKED`  
**Base branch:** `codex/hepta-intelligence-grounding-gate-v3-20260828`  
**Base head:** `256a47d882413ac1f80892a80177419bb5d12c70`  
**Development branch:** `codex/hepta-intelligence-mutation-state-machine-v4-20260828`

## Scope

P0.4 introduces one explicit typed state machine for a logical Intelligence mutation:

```text
Planned
  -> SourceWitnessed
  -> GroundingValidated
  -> DurableIntentAppended
  -> MemoryFactsCommitted
  -> ProjectionPublished
  -> OutboxSettled
  -> Terminal
```

Exceptional resolution is explicit:

```text
DurableIntentAppended | MemoryFactsCommitted | ProjectionPublished
  -> Indeterminate
  -> ReconciledApplied | ReconciledNotApplied | Quarantined
```

## Binding and replay rules

Every request binds:

- operation ID;
- lease ID and positive lease epoch;
- expected memory revision;
- starting projection generation;
- causal root;
- exact transition sequence;
- previous transition digest;
- action-specific evidence or receipt digest.

An exact duplicate transition returns the original immutable receipt as `Replay`. A changed duplicate, reordered sequence, causal-parent drift, lease drift, double memory write, double projection publication, and stale generation all fail closed.

## Safety invariants

- terminalization is legal only after `OutboxSettled`;
- one logical operation writes memory facts at most once;
- one logical operation publishes at most one projection generation;
- projection generation advances exactly once;
- all normal and exceptional terminal outcomes settle the durable intent;
- `ReconciledNotApplied` is legal only when no write was observed;
- source-only receipts grant no runtime, SQLite, effect, production, operator, or promotion authority.

## Model and test boundary

The Rust module includes deterministic unit/property-style tests for:

- normal ordering;
- duplicate replay;
- changed replay and sequence reorder;
- causal parent and lease drift;
- stale generation;
- crash after intent;
- crash after memory commit;
- incompatible reconciliation;
- quarantine;
- deterministic transition digests.

`models/P0_4_INTELLIGENCE_MUTATION_STATE_MACHINE.tla` specifies the same transition graph and invariants: no double write, no double publication, bounded generation advance, terminal settlement, and resolved-indeterminate settlement.

## Explicitly incomplete

```text
runtime_wired=false
sqlite_persistence=false
sqlite_failpoint_execution=false
production_caller=false
production_authority=false
external_effects=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

Actual host orchestration, SQLite journal rows, failpoint execution at each persistence boundary, and integration with admission/grounding/projection/outbox/compact/rehydration remain blocked until P0.1-P0.3 receive executable qualification.

## Qualification-toolchain repair

The P0.2 and P0.3 dedicated workflows had drifted to Rust `1.88.0` while
`codex-rs/rust-toolchain.toml` pins `1.95.0`. This tranche updates both inherited
workflows to `1.95.0` and adds an explicit installed-version check against the
repository pin before formatting or tests run. This is qualification hardening;
it does not change runtime or production authority.

## Qualification

```bash
python3 scripts/verify-hepta-intelligence-mutation-state.py
cd codex-rs
cargo fmt --all -- --check
cargo test -p codex-hepta-memory intelligence_mutation_state -- --nocapture
cargo test -p codex-hepta-memory
cargo clippy -p codex-hepta-memory --all-targets -- -D warnings
```

The workflow must use the repository toolchain `1.95.0`, not the stale `1.88.0` previously copied into P0.2/P0.3 dedicated workflows.
