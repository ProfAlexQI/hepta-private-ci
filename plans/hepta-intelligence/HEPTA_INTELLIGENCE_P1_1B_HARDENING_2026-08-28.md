# Hepta Intelligence P1.1b Hardening — Bounded Decode, Receipt Coherence, and Deterministic ANN Probing

**Date:** 2026-08-28  
**Status:** `SOURCE_ONLY / STACKED_DRAFT / QUALIFICATION_PENDING / ACTIVATION_BLOCKED`  
**Parent PR:** `#34`  
**Parent branch:** `codex/hepta-intelligence-local-embedding-index-v1b-20260828`  
**Parent exact head:** `c804ce200e3eccec96e90b867de31c5cacfdf82f`  
**Hardening branch:** `codex/hepta-intelligence-p1-1b-hardening-v1-20260828`

## 1. Why this tranche exists

The P1.1b source review found several fail-closed gaps that must be fixed before the exact-head executable gate can be treated as sufficient evidence:

1. the ANN decoder accepted a file-provided dimension before enforcing the embedding dimension ceiling;
2. the decoder did not bind `bucket_count` to `item_count` before entering the bucket loop;
3. ANN probe signatures were globally sorted, so the exact bucket was not guaranteed to be scanned before Hamming-distance-1 buckets when the candidate cap was reached;
4. stored vectors were not independently rechecked for Q15 unit norm during reopen verification;
5. fallback token receipts could carry tokenizer binding fields if the receipt digest was recomputed;
6. semantic route receipts did not recompute route and reason from their readiness fields;
7. the Q15 norm tolerance was wider than required for a fixed-point local vector contract, and cosine scoring divided by the ideal norm rather than the actual accepted vector norms;
8. search receipt validation did not enforce bounded counts, unique identities, or deterministic ordering.

This tranche fixes those source-review findings only. It does not add a production model, product workspace membership, runtime registration, default recall, context attachment, physical-send, external effects, or production authority.

## 2. Hardening contract

### 2.1 Bounded decode before allocation

The decoder must reject dimensions outside:

```text
8..=MAX_EMBEDDING_DIMENSIONS
```

before it allocates an entry vector. It must reject:

```text
item_count == 0
item_count > MAX_INDEX_ITEMS
bucket_count == 0
bucket_count > item_count
bucket_count > MAX_INDEX_ITEMS
```

before decoding entries or buckets.

### 2.2 Stored-vector verification

Every built or reopened entry must satisfy all of:

```text
exact manifest dimensions
vector SHA-256
Q15 unit-norm bound
recomputed deterministic LSH signature
unique candidate ID + revision
exactly-one bucket membership
```

A file that recomputes its unkeyed structural digests around a non-unit vector must still fail verification.

### 2.3 Exact-bucket-first probing

Probe order is now:

```text
exact query signature
then sorted unique Hamming-distance-1 signatures
```

The exact bucket therefore cannot be starved by neighbor buckets when `MAX_SEARCH_CANDIDATES` is reached.

### 2.4 Actual fixed-point cosine

Cosine similarity uses:

```text
dot(left, right) / integer_sqrt(norm_squared(left) * norm_squared(right))
```

with integer arithmetic and a `1_000_000` score scale. The score no longer assumes the ideal Q15 norm for every accepted vector. The Q15 admission tolerance is tightened from `100_000 ppm` to `10_000 ppm`.

### 2.5 Receipt coherence

A UTF-8 byte fallback receipt must have no tokenizer descriptor, artifact, vocabulary, or model-compatibility bindings.

A semantic route receipt must exactly match the deterministic priority:

```text
dependency unqualified
→ tokenizer unavailable
→ embedding provider unavailable
→ index unavailable
→ binding mismatch
→ shadow semantic ready
```

ANN search receipts must enforce bounded counters, unique candidate identities, stable result ordering, and digest coherence.

## 3. Negative regression matrix

The hardening source adds executable regressions for:

- injected tokenizer bindings in fallback receipts;
- zero/non-unit provider vectors;
- dimensions above the pre-allocation ceiling;
- `bucket_count > item_count`;
- exact-bucket-first probe order;
- actual-norm fixed-point cosine;
- rehashed non-unit stored vectors;
- non-deterministically ordered ANN search receipts;
- semantic route receipts inconsistent with readiness fields.

## 4. Qualification

The hardening exact head must independently pass:

```bash
python3 scripts/verify-hepta-intelligence-local-embedding-index.py
cargo fmt --manifest-path codex-rs/hepta-memory-p1-1b-qualification/Cargo.toml --all -- --check
cargo test --manifest-path codex-rs/hepta-memory-p1-1b-qualification/Cargo.toml --all-targets -- --nocapture
cargo check --manifest-path codex-rs/hepta-memory-p1-1b-qualification/Cargo.toml --all-targets
cargo clippy --manifest-path codex-rs/hepta-memory-p1-1b-qualification/Cargo.toml --all-targets -- -D warnings
```

Qualification evidence must bind the exact hardening source commit and prove the qualification wrapper is the only additional path.

## 5. Authority boundary

The following remain false:

```text
wired=false
qualified=false
product_workspace_member=false
product_module_registered=false
default_recall_changed=false
federation_recall_changed=false
context_attachment=false
physical_send=false
remote_embedding=false
model_download=false
network_access=false
external_effects=false
production_authority=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

P1.1c remains blocked until this hardening source review and exact-head executable qualification are complete. Even then, P1.1c is a separate source-only stacked Draft and does not activate product recall or production authority.
