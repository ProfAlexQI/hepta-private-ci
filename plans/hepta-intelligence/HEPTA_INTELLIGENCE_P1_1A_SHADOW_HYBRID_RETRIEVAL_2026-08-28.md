# Hepta Intelligence P1.1a — Shadow Hybrid Retrieval Contract

**Date:** 2026-08-28  
**Status:** `SOURCE_ONLY / STACKED_DRAFT / ACTIVATION_BLOCKED`  
**Base branch:** `codex/hepta-intelligence-shadow-host-adapter-v4c-20260828`  
**Base commit:** `7691978b786dd00c69477d1a3355be13db2c4d67`  
**Development branch:** `codex/hepta-intelligence-hybrid-retrieval-v2-p1a-20260828`

## 1. Purpose

The current product recall path remains lexical/entity/KG/recency retrieval with its existing deterministic fusion and physical-send revalidation. P1.1a does not replace that path.

This tranche establishes the first typed Hybrid Retrieval v2 contract:

```text
bounded query draft
→ deterministic query plan
→ typed multi-channel candidate evidence
→ lifecycle/truth/grounding/secret eligibility
→ fixed-point deterministic fusion
→ stable token-budget selection
→ digest-bound shadow receipt
```

P0.1–P0.4c remain `qualified=false`, so P1.1 is not activated. Source development is isolated in a stacked Draft and all runtime and authority flags remain negative.

```text
runtime_wired=false
default_recall_changed=false
vector_backend_registered=false
reranker_registered=false
context_attachment=false
physical_send=false
external_effects=false
production_authority=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

## 2. Scope

P1.1a implements:

- deterministic multilingual query normalization;
- typed intent and risk classes;
- bounded lexical terms and entity hints;
- semantic-query digest binding without a registered embedding backend;
- exact lexical, FTS, entity alias, semantic vector, KG and recency channel contracts;
- per-channel rank and PPM score validation;
- lifecycle, truth, grounding and secret eligibility;
- high-risk rejection of `legacy_unreviewed`;
- mandatory grounded evidence for KG channel candidates;
- UTF-8 byte upper-bound token estimation;
- deterministic fixed-point fusion;
- stable tie breaking;
- bounded context-token selection;
- candidate-set, selected-result, plan and receipt digests;
- source-only negative authority receipts.

P1.1a does not implement:

- SQLite vector tables or ANN;
- an embedding model or tokenizer;
- a model reranker;
- query execution against the cognitive store;
- a KG traversal;
- modification of `cognitive_retrieval.rs`;
- modification of federation recall;
- modification of the extension recall contributor;
- context attachment;
- physical-send integration;
- runtime or production activation.

## 3. Query plan

The input contract contains:

```text
query
scope key
intent
risk class
caller lexical terms
entity hints
optional semantic query
optional required time range
max result count
max context-token budget
```

The plan stores:

```text
query SHA-256
scope SHA-256
normalized unique lexical terms
normalized unique entities
optional semantic-query SHA-256
enabled channel set
time range
result and token budgets
token estimator type
negative runtime/authority flags
plan SHA-256
```

Raw semantic query content is not included in the receipt.

### Intent

```text
recall_fact
recall_preference
resolve_entity
resolve_temporal_state
retrieve_procedure
general_context
```

### Risk class

```text
low
standard
high
critical
```

`high` and `critical` requests reject `legacy_unreviewed` candidate revisions.

## 4. Candidate channels

```text
exact_lexical
lexical_fts
entity_alias
semantic_vector
knowledge_graph
recency
```

Every channel carries:

```text
rank >= 1
channel_score_ppm in 0..=1_000_000
```

Duplicate channel evidence on one candidate is rejected.

`semantic_vector` is a contract only. This tranche fixes:

```text
vector_backend_registered=false
```

A candidate may carry synthetic vector evidence in focused contract tests, but no embedding inference is invoked.

## 5. Eligibility

Candidates are rejected before scoring when:

- lifecycle is not `active`;
- truth status is `disputed`, `contradicted` or `expired`;
- the candidate is secret-like;
- high/critical risk encounters `legacy_unreviewed`;
- a KG channel lacks `grounded_v1` or `backfilled_grounded_v1`;
- a zero-fact candidate claims KG evidence;
- bounded IDs, revisions, PPM scores or token estimates are invalid.

The grounding states are:

```text
grounded_v1
backfilled_grounded_v1
legacy_unreviewed
zero_fact
```

The truth states are:

```text
candidate
grounded
confirmed
disputed
contradicted
expired
```

This source contract does not claim to create or independently verify those states. It consumes their typed classification.

## 6. Fusion

Fusion uses integer fixed-point arithmetic only.

Each channel contribution is based on:

```text
channel default weight
× channel score PPM
÷ (rank + reciprocal-rank K)
```

The combined channel score is calibrated with:

```text
source reliability
freshness
truth multiplier
grounding multiplier
```

The implementation uses `u128` checked arithmetic and returns an explicit overflow error rather than wrapping.

Results are sorted by:

```text
fused score descending
estimated tokens ascending
candidate ID ascending
memory revision descending
```

Selection stops at the plan result count and skips candidates that would exceed the context-token budget.

No model reranker runs in this tranche:

```text
reranker_registered=false
```

## 7. Token budget

The only estimator in P1.1a is:

```text
utf8_byte_upper_bound
```

For one string:

```text
estimated_tokens = UTF-8 byte length
```

This is deliberately conservative and deterministic. It is not described as the actual tokenizer count.

P1.1b must add a registry that binds:

```text
model ID digest
tokenizer digest
estimator kind
exact/estimated status
fallback margin
```

before an exact tokenizer can be claimed.

## 8. Digest contract

The implementation binds:

- query plan;
- candidate-set order-independent digest;
- each scored candidate;
- selected result order;
- rejection counts;
- selected token budget;
- all negative authority flags.

Tampering with result contents, result count or authority flags invalidates receipt validation.

## 9. Compile isolation

The source contract lives at:

```text
codex-rs/hepta-memory/src/shadow_hybrid_retrieval_v2.rs
```

It is not registered in the product library module graph.

A dedicated integration test compiles the exact source file:

```text
codex-rs/hepta-memory/tests/shadow_hybrid_retrieval_v2_contract.rs
```

This provides executable Rust qualification while proving that no product runtime caller was added.

## 10. Focused tests

The source includes tests for:

- deterministic multilingual planning;
- lexical-term normalization and deduplication;
- semantic/KG channel planning without backend registration;
- high-risk legacy rejection;
- KG grounding requirement;
- contradiction rejection;
- secret rejection;
- tombstone rejection;
- context-budget rejection;
- stable fusion under input reordering;
- no reranker/context/send authority;
- receipt tamper rejection;
- conservative multibyte token estimation;
- duplicate channel rejection.

## 11. Qualification

Source gate:

```bash
python3 scripts/verify-hepta-intelligence-hybrid-retrieval.py
```

Required Rust qualification:

```bash
cd codex-rs
cargo fmt --all -- --check
cargo test \
  -p codex-hepta-memory \
  --test shadow_hybrid_retrieval_v2_contract \
  -- --nocapture
cargo test -p codex-hepta-memory
cargo clippy \
  -p codex-hepta-memory \
  --test shadow_hybrid_retrieval_v2_contract \
  -- -D warnings
cargo clippy \
  -p codex-hepta-memory \
  --all-targets \
  -- -D warnings
```

The dedicated workflow must use repository Rust `1.95.0` and upload the exact source receipt.

## 12. Exit gate

P1.1a can become `qualified=true` only after one exact head has:

- source gate PASS;
- formatting PASS;
- focused integration test PASS;
- full memory crate tests PASS;
- focused and full strict clippy PASS;
- P0.1–P0.4c exact-head executable qualification;
- readable P0 crash-window artifacts;
- candidate freeze and operator review.

Even then, the source contract does not activate retrieval.

## 13. Next guarded tranches

### P1.1b — Local embedding and tokenizer adapter

Required work:

- explicit local embedding backend trait;
- model/tokenizer digest binding;
- bounded batch execution;
- deterministic no-backend fallback;
- vector dimension and normalization contract;
- SQLite or sidecar ANN schema;
- corpus-free adapter qualification;
- no default recall wiring.

### P1.1c — Offline efficacy harness

Required work:

- versioned multilingual de-identified corpus;
- lexical-only baseline;
- hybrid candidate;
- Recall@4 and nDCG@4;
- citation precision;
- false/stale/contradicted attachment rates;
- task-success delta;
- latency and token cost;
- subgroup and no-regression reports.

### P1.1d — Shadow runtime compare

Only after dependencies qualify:

```text
existing product retrieval
        │
        ├─ remains authoritative
        │
        └─ same bounded request
             → hybrid shadow result
             → comparison receipt
             → no context attachment
```

A separate CALLERS decision is required before any default recall, context or physical-send change.
