# Hepta P0.7b/B4 Global No-Bypass Contract V1

Status: source candidate; no runtime, production-caller, operator, promotion, or release authority.

## Purpose

B4 turns the B0–B3 checked-boundary stack into a repository-wide fail-closed ratchet. It does not infer safety from a text search. The verifier binds three facts:

1. the Cargo workspace and target graph accepted by `cargo metadata --locked`;
2. a Rust-aware lexical pass that removes nested comments and all string/character forms before identifying sensitive boundary symbols;
3. an exact closed-set inventory that classifies every occurrence as boundary implementation, qualification-only, fixture-only, dead code, or production via verified use.

Any unclassified production occurrence is a failure. New files, generated/include-based sensitive source, symlinked Rust source, stale inventory entries, unknown Cargo ownership, or changed physical-kind bindings fail closed.

## Production entry requirements

A production entry is valid only when the inventory binds all of:

- file and symbol;
- physical capability kind;
- exact capability and issuer;
- consumer;
- final-payload builder;
- durable claim-store owner;
- durable witness-store owner;
- reconciliation owner.

A production binary may not gain authority merely because source compiles. B4 only proves source topology and classification. Exact-head qualification, merge-candidate qualification, independent review, runtime registration, operator acceptance, promotion, and release remain separate facts.

## Current candidate posture

The current inventory contains no production callsite. B0–B3 remain checked source boundaries and qualification fixtures only. Therefore every runtime/effect authority flag remains false.

## Qualification

The required workflow must retain an exact-SHA report containing every sensitive occurrence, its line, Cargo package, target kind, classification, and physical kind. It also runs the B0–B3 source verifiers, locked metadata, contracts tests, all-target check, strict Clippy, Bazel lock verification, and clean-worktree verification.

A queued, skipped, cancelled, empty-step, stale-head, or self-issued result is not PASS.
