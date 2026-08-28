# Hepta Intelligence P0.4b — SQLite Mutation Journal

**Date:** 2026-08-28  
**Status:** `SOURCE + SQLITE CONTRACT / STACKED DRAFT / RUNTIME WIRING BLOCKED`  
**Base branch:** `codex/hepta-intelligence-mutation-state-machine-v4-20260828`  
**Base commit:** `39c30cc1f07c90c4d5176da05e9bd97e1792bd2a`  
**Development branch:** `codex/hepta-intelligence-mutation-journal-v4b-r2-20260828`

## 1. Scope

P0.4a defines the only legal typed mutation transition graph. P0.4b adds an opt-in Agent-local SQLite journal that persists operation bindings and immutable transition receipts, then reconstructs the P0.4a state machine from genesis on reopen.

```text
runtime_wired=false
default_open_wired=false
external_effects=false
production_authority=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

The default `CognitiveStore::open`, memory/KG writers, projection pointer, recall queries, physical-send path, TaskFlow and production durable writer remain unchanged.

## 2. Component migration 0012

The opt-in migration is stored at:

```text
codex-rs/hepta-memory/mutation-migrations/0012_intelligence_mutation_journal.sql
```

It creates an immutable component-migration ledger, immutable operation bindings and immutable transition receipts. It binds owner, lease/epoch, expected revision, starting projection generation and causal root, and rejects sequence gaps, causal-parent drift, non-monotonic counters, double writes, double publication and terminalization before durable-intent settlement.

It does not alter the canonical SQLx migration lineage.

## 3. Atomic append and replay

One append executes under `BEGIN IMMEDIATE`:

```text
verify component schema
→ load immutable operation binding
→ replay prior transitions from Planned
→ apply one typed P0.4a request
→ insert one immutable transition receipt
→ COMMIT
```

Exact duplicate retries return the existing receipt. Changed duplicates, binding drift, sequence drift and stale generation fail closed.

Reopen verification checks the migration checksum, exact 14-object schema inventory, scratch-database schema oracle, operation binding, action payload, request/parent/transition digests, phases, counters, generation and terminal invariants.

## 4. Crash boundaries

Qualification failpoints cover:

```text
BeforeTransitionInsert
AfterTransitionInsertBeforeCommit
AfterCommitBeforeReturn
```

Failures before commit leave no transition row. A failure after commit is reported as indeterminate; retrying the exact request adopts the committed receipt as Replay and does not create a second transition.

## 5. Qualification

```bash
python3 scripts/hepta-intelligence-mutation-journal-sqlite-selftest.py
python3 scripts/verify-hepta-intelligence-mutation-journal.py
cd codex-rs
cargo fmt --all -- --check
cargo test -p codex-hepta-memory intelligence_mutation_journal -- --nocapture
cargo test -p codex-hepta-memory intelligence_mutation -- --nocapture
cargo test -p codex-hepta-memory
cargo clippy -p codex-hepta-memory --all-targets -- -D warnings
```

The workflow must use the repository-pinned Rust `1.95.0` and verify `rustc --version` before running gates.

## 6. Exit gate

P0.4b remains `qualified=false` until one exact head has executable source/SQLite artifacts, Rust fmt/focused/full/clippy PASS, P0.1–P0.4a dependency qualification, crash-window review, candidate freeze and operator review.

Runtime wiring is a separate guarded decision. P1.1 must not be activated from source-only or SQLite-only evidence.
