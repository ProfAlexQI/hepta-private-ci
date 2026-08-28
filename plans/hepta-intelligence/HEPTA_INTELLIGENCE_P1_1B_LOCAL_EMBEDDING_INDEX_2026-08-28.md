# Hepta Intelligence P1.1b — Local Embedding, Tokenizer Registry, and Immutable ANN Index

**Date:** 2026-08-28  
**Status:** `SOURCE_ONLY / STACKED_DRAFT / ACTIVATION_BLOCKED`  
**Base PR:** `#28`  
**Base branch:** `codex/hepta-intelligence-hybrid-retrieval-v2-p1a-20260828`  
**Base commit:** `906aad979ba9e606c2e41d804e7a0c226efae5cb`  
**Development branch:** `codex/hepta-intelligence-local-embedding-index-v1b-20260828`

## 1. Purpose

P1.1a defines a shadow query-plan, candidate, risk, grounding, fixed-point fusion, and context-budget contract. It deliberately leaves the vector backend and exact tokenizer absent.

P1.1b implements the next bounded source tranche:

```text
local tokenizer registry
+ local embedding provider contract
+ deterministic Q15 embedding receipts
+ immutable deterministic_lsh64_v1 ANN generation
+ create-only index persistence
+ exact reopen verification
+ local ANN search receipt
+ deterministic lexical-only fallback
```

It does not activate Hybrid Retrieval.

```text
product_workspace_member=false
product_module_registered=false
runtime_wired=false
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

P0.1–P1.1a remain `qualified=false`; source development proceeds only on a separate stacked Draft.

## 2. Compile and dependency isolation

The implementation is a nested dependency-free Rust workspace:

```text
codex-rs/hepta-memory-p1-1b-qualification/
```

It is not listed in `codex-rs/Cargo.toml`, is not imported by `codex-hepta-memory`, and does not modify the existing retrieval, federation, extension, Agentd, attachment, or physical-send paths.

The nested workspace provides executable source qualification without inheriting unrelated product-workspace format or module-path debt. It is a qualification contract, not a product runtime crate.

## 3. SHA-256 and receipt framing

The crate contains a dependency-free SHA-256 implementation with the standard `abc` known-answer test.

All receipts use length-prefixed framed hashing. Digests bind field boundaries, ordered vector bytes, artifact identities, generation, authority flags, and result ordering.

No receipt is a secret-key signature or independent authority capability.

## 4. Tokenizer registry

### 4.1 Descriptor

Each tokenizer binds:

```text
tokenizer ID
tokenizer artifact SHA-256
vocabulary SHA-256
model-compatibility SHA-256
tokenizer contract
implementation kind
maximum input bytes
local-only / remote / download flags
```

The registry is bounded and rejects duplicate IDs.

### 4.2 Exact and fallback modes

Token-count receipts distinguish:

```text
exact_local
utf8_byte_upper_bound
```

`exact_local` requires a registered local engine and binds the exact tokenizer descriptor.

When the requested tokenizer is absent, the registry deterministically falls back to:

```text
token_count = UTF-8 byte length
tokenizer_estimator = utf8_byte_upper_bound
exact = false
```

This is conservative for multibyte text and cannot be interpreted as the target model's exact token count.

The checked-in reference tokenizer is exact only for its disclosed `alphanumeric_punctuation_v1` qualification contract. It does not claim compatibility with a production model tokenizer.

## 5. Local embedding provider contract

Each provider binds:

```text
provider ID
model ID
model artifact SHA-256
tokenizer artifact SHA-256
dimensions
maximum batch
maximum input bytes
metric = cosine
quantization = i16_q15_unit
provider kind
local-only authority flags
```

Provider output is accepted only when:

- batch cardinality matches;
- every vector has the exact bound dimensions;
- vector bytes match the vector SHA-256;
- the vector is a bounded Q15 unit vector;
- model, tokenizer, metric, quantization, and provider descriptor agree;
- remote execution, model download, network access, and production-model verification remain false.

The `QualificationHashOneHotProvider` exists only as a deterministic local test implementation. It is not a semantic model and supplies no efficacy evidence.

## 6. Immutable local ANN index

### 6.1 Algorithm

The source implements:

```text
deterministic_lsh64_v1
```

For each vector, 64 deterministic signed projections are derived from the source-bound index seed. The 64 signs form an immutable bucket signature.

Search visits:

```text
exact signature
+ all Hamming-distance-1 signatures
```

Candidates inside visited buckets are scored with deterministic fixed-point cosine similarity. Tie-breaking is:

```text
similarity descending
candidate ID ascending
memory revision descending
```

The source is an approximate candidate index with exact fixed-point scoring inside the visited candidate set.

### 6.2 Immutable bindings

The manifest binds:

```text
index ID and generation
algorithm
provider descriptor
model artifact
tokenizer artifact
dimensions
metric and quantization
index seed
item and bucket counts
entries digest
buckets digest
all negative authority flags
manifest digest
```

Entries store:

```text
candidate ID
memory revision
content SHA-256
quantized vector
vector SHA-256
LSH signature
```

Raw memory and source content are not stored.

### 6.3 Create-only persistence

The writer uses:

```text
OpenOptions::create_new(true)
write_all
sync_all
```

An existing path cannot be replaced. A failed or partial file is never accepted merely because it exists; reopen must pass all structural and digest checks.

### 6.4 Reopen verification

Reopen verifies:

- bounded file size and exact magic;
- schema and namespace;
- no trailing bytes;
- item and bucket limits;
- candidate identity uniqueness;
- exact vector dimensions and SHA-256;
- deterministic LSH signature recomputation;
- ordered and complete bucket membership;
- entry and bucket digests;
- manifest digest;
- expected index, generation, provider, model, tokenizer, dimensions, and optional manifest digest.

Any drift fails closed.

## 7. Search and lexical fallback

A query vector must match the immutable provider/model/tokenizer/dimension/metric/quantization binding.

Search receipts bind:

```text
index and generation
manifest digest
query vector digest
query signature
visited bucket count
scanned candidate count
ordered results
fallback status
all negative authority fields
```

When no ANN candidate is available, the result explicitly requires lexical fallback. P1.1b does not itself call or replace the product lexical retrieval path.

A separate route receipt uses deterministic priority:

```text
dependency unqualified
→ tokenizer unavailable
→ embedding provider unavailable
→ index unavailable
→ binding mismatch
→ shadow semantic ready
```

Until every dependency is ready, route is `lexical_only`.

## 8. Tests

The isolated regression matrix covers:

- SHA-256 known-answer behavior;
- exact tokenizer count and UTF-8 fallback;
- deterministic local embedding and Q15 normalization;
- input-order-independent index generation;
- create-only persistence;
- close/reopen verification;
- exact query search;
- raw-content absence;
- file tamper rejection;
- model-binding drift rejection;
- deterministic dependency-first lexical fallback;
- all runtime, effect, and production flags remaining false.

## 9. Qualification

Source gate:

```bash
python3 scripts/verify-hepta-intelligence-local-embedding-index.py
```

Isolated Rust qualification:

```bash
cargo fmt \
  --manifest-path codex-rs/hepta-memory-p1-1b-qualification/Cargo.toml \
  --all -- --check

cargo test \
  --manifest-path codex-rs/hepta-memory-p1-1b-qualification/Cargo.toml \
  --all-targets -- --nocapture

cargo check \
  --manifest-path codex-rs/hepta-memory-p1-1b-qualification/Cargo.toml \
  --all-targets

cargo clippy \
  --manifest-path codex-rs/hepta-memory-p1-1b-qualification/Cargo.toml \
  --all-targets -- -D warnings
```

The dedicated workflow uses Rust `1.95.0`, has read-only repository permission, and contains no model download, external network call, deploy, publish, release, runtime activation, or product mutation job.

## 10. Exit gate

P1.1b may become `qualified=true` only when one frozen exact head has:

- source gate PASS;
- isolated formatting PASS;
- all unit/integration tests PASS;
- all-target check PASS;
- strict Clippy PASS;
- readable exact-head artifacts;
- P0.1–P1.1a executable qualification;
- review of index format, tamper behavior, token fallback, and no-product-wiring proof.

Even after qualification, product registration and CALLERS activation are separate decisions.

## 11. Next guarded tranche — P1.1c

P1.1c is:

```text
multilingual offline efficacy corpus
+ lexical/vector/KG ablations
+ grounding/truth/contradiction features
+ bounded KG two-hop evidence
+ calibrated reranker interface
+ Recall@4 / nDCG@4 / citation precision receipts
+ latency and token-cost receipts
```

P1.1c must keep runtime and production authority false until efficacy, operator, and CALLERS gates are independently satisfied.
