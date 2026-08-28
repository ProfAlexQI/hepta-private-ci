# Hepta Intelligence P0.4b — SQLite Mutation Journal

**Date:** 2026-08-28  
**Status:** `SOURCE + SQLITE CONTRACT / STACKED DRAFT / RUNTIME WIRING BLOCKED`  
**Base branch:** `codex/hepta-intelligence-mutation-state-machine-v4-20260828`  
**Base commit:** `39c30cc1f07c90c4d5176da05e9bd97e1792bd2a`  
**Development branch:** `codex/hepta-intelligence-mutation-journal-v4b-r1-20260828`

## 1. Scope

P0.4a defines the only legal typed mutation transition graph. P0.4b adds an opt-in Agent-local SQLite journal that persists operation bindings and immutable transition receipts, then reconstructs the P0.4a state machine from genesis on reopen.

This tranche does **not** wire the journal into a host or production caller.

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

It creates:

- one immutable component-migration ledger;
- one immutable operation-binding table;
- one immutable transition table;
- exact owner, lease/epoch, expected revision, starting generation and causal-root binding;
- sequence and causal-parent guards;
- monotonic intent/write/publication counters;
- exact one-generation publication guard;
- terminal-settlement guard;
- immutable lookup indexes.

It does not alter the canonical SQLx migration lineage.

## 3. Atomic append semantics

One append executes under `BEGIN IMMEDIATE`:

```text
verify component schema
→ load immutable operation binding
→ replay every prior transition from genesis
→ apply one typed P0.4a request
→ insert one immutable transition receipt
→ COMMIT
```

Exact duplicate retries return the existing receipt. Changed duplicates, sequence gaps, causal-parent drift, binding drift, double writes, double publication and stale generation fail closed.

## 4. Crash boundaries

Qualification failpoints cover:

```text
BeforeTransitionInsert
AfterTransitionInsertBeforeCommit
AfterCommitBeforeReturn
```

Expected behavior:

- failures before commit leave no transition row;
- a failure after commit is reported as indeterminate;
- retrying the exact request adopts the committed receipt as Replay;
- no second transition, write or publication is generated.

## 5. Reopen verification

`verify_intelligence_mutation_journal` performs:

1. migration checksum verification;
2. exact 14-object schema inventory verification;
3. scratch-database schema oracle comparison;
4. owner and operation-binding recomputation;
5. ordered transition replay from `Planned`;
6. action payload reconstruction;
7. exact request, causal-parent and transition digest comparison;
8. exact phase/counter/generation receipt comparison;
9. terminal and no-double-write/no-double-publication invariant validation.

Any divergence rejects reopen of the opt-in journal path.

## 6. Qualification

Executable SQLite contract:

```bash
python3 scripts/hepta-intelligence-mutation-journal-sqlite-selftest.py
```

Fail-closed source gate:

```bash
python3 scripts/verify-hepta-intelligence-mutation-journal.py
```

Required Rust qualification:

```bash
cd codex-rs
cargo fmt --all -- --check
cargo test -p codex-hepta-memory intelligence_mutation_journal -- --nocapture
cargo test -p codex-hepta-memory intelligence_mutation -- --nocapture
cargo test -p codex-hepta-memory
cargo clippy -p codex-hepta-memory --all-targets -- -D warnings
```

The workflow must use the repository-pinned Rust `1.95.0` and verify `rustc --version` before running gates.

## 7. Exit gate

P0.4b remains `qualified=false` until one exact head has:

- executable source-gate PASS artifact;
- executable SQLite self-test PASS artifact;
- Rust fmt, focused tests, full crate tests and strict clippy PASS;
- P0.1–P0.4a dependency qualification;
- crash-window receipt review;
- candidate freeze and operator review.

Even after qualification, runtime wiring is a separate guarded decision. P1.1 must not be activated from source-only or SQLite-only evidence.
