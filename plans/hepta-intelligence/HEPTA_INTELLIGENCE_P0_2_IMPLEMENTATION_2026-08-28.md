# Hepta Intelligence P0.2 Implementation — Durable Fact-Grounding Ledger

**Date:** 2026-08-28 (Asia/Singapore)  
**Branch:** `codex/hepta-intelligence-grounding-ledger-v2-20260828`  
**Stack base:** `codex/hepta-intelligence-grounding-v2-20260828` at `10f89cd913b12dcb347a3bae5f290a455f985eb4`  
**State:** SOURCE GATE PASS / RUST AND HOSTED QUALIFICATION PENDING / `qualified=false`

## Scope

P0.2 implements an Agent-local append-only source-span evidence ledger without
activating the P0.3 production projection gate.

The tranche includes:

- component migration 0011;
- immutable grounding receipt and evidence-span tables;
- exact source/memory/fact-set foreign-key and trigger bindings;
- a dedicated migration ledger and schema oracle;
- atomic remember and correction paths;
- opt-in verified reopen;
- source-byte, UTF-8 boundary, evidence digest, fact identity, and receipt digest recomputation;
- explicit `grounded_v1`, `legacy_unreviewed`, and `zero_fact` inventory states;
- corruption, restart, correction, rollback, and legacy-status tests.

## Integration strategy

P0.1 is still blocked on executable hosted qualification. To avoid mutating the
canonical SQLx migration lineage before its dependency is qualified, P0.2 uses
a fail-closed **component migration 0011**:

```text
CognitiveStore::open
  → ensure_durable_fact_grounding_schema
  → dedicated component migration ledger
  → dedicated schema oracle
  → verify_durable_fact_grounding_ledger
```

The default `CognitiveStore::open` remains unchanged. The P0.2 path is exposed
through `CognitiveStore::open_with_durable_fact_grounding`, so this capability
is implemented but deliberately not production-wired.

Promotion into the main SQLx `0001..0011` lineage is a later integration
decision after P0.1 and P0.2 exact-head qualification. This prevents an
unqualified stacked branch from making every default database open depend on
new schema.

## Atomic write order

```text
BEGIN IMMEDIATE
  → append exact source
  → create/revise memory using CAS
  → canonicalize and insert immutable KG facts
  → insert durable grounding receipt
  → insert byte-exact evidence spans
  → verify declared/stored span count
  → publish the next projection generation
COMMIT
```

Any failure before `COMMIT` rolls back source, memory, KG facts, grounding
evidence, and projection generation together.

## Legacy policy

P0.2 never backfills historical non-empty fact sets as grounded.

```text
grounded_v1       = complete durable receipt and spans
legacy_unreviewed = non-empty fact set without durable grounding receipt
zero_fact         = explicit empty fact set
```

`legacy_unreviewed` is not an error while the P0.3 gate is inactive, but it may
not be interpreted as grounded or promoted automatically.

## Authority boundary

The following remain fixed, including `production_projection_gate=false`:

```text
production_projection_gate=false
production_authority=false
external_effects=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

Textual source grounding is evidence that a structure is supported by the
bound source text. It is not proof of external-world truth.

## Qualification

Source gate (locally reproduced on the staged candidate):

```bash
python3 scripts/verify-hepta-intelligence-grounding-ledger.py
# PASS_P0_2_DURABLE_GROUNDING_SOURCE_ONLY
```

Required executable qualification:

```bash
cargo fmt --all -- --check
cargo test -p codex-hepta-memory durable_grounding -- --nocapture
cargo test -p codex-hepta-memory fact_grounding -- --nocapture
cargo test -p codex-hepta-memory
cargo clippy -p codex-hepta-memory --all-targets -- -D warnings
```

P0.2 remains `qualified=false` until one exact head has readable executable
evidence. P0.3 remains blocked until P0.1 and P0.2 both satisfy their exit gates.
