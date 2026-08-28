# Hepta Intelligence P1.1c — Multilingual Offline Efficacy and Ablation Plan

Status: implementation complete, executable qualification pending  
Authority: source-only qualification; no product activation  
Stack base: canonical P1.1b local embedding and immutable ANN index branch

## 1. Objective

P1.1c establishes a deterministic, dependency-free evaluation surface for the retrieval stack introduced by P1.1a and P1.1b. It measures lexical, vector, and bounded knowledge-graph signals under a common fixed-point contract and emits auditable machine receipts for:

- Recall@4;
- nDCG@4;
- citation precision;
- P50 and P95 synthetic latency;
- top-four token cost;
- lexical/vector/KG ablations;
- a deterministic full reranker using grounding, truth, contradiction, and citation features.

This tranche validates the evaluation machinery and its authority boundaries. It does **not** claim real-world efficacy, production calibration, model quality, or activation readiness.

## 2. Frozen trust boundary

The P1.1c crate is nested at:

```text
codex-rs/hepta-memory-p1-1c-qualification/
```

It must remain outside the product `codex-rs` workspace and outside the `codex-hepta-memory` module graph. The source verifier fails closed unless the canonical P1.1b base contains all bounded-decoder hardening markers:

- `MAX_EMBEDDING_DIMENSIONS` is enforced before vector allocation;
- `bucket_count > item_count` is rejected;
- `read_bounded_index_bytes` caps reads at `MAX_INDEX_FILE_BYTES + 1`;
- metadata/read-length drift is rejected;
- the two hardening regression tests are present.

P1.1c must not alter runtime recall, federation recall, context attachment, physical-send, model loading, network access, or production authority.

## 3. Implemented contracts

### 3.1 Governed multilingual corpus

The seed corpus uses a strict 17-column TSV schema with bounded identifiers, queries, locales, cases, and candidates. It contains eight locales and 48 candidates. The checked-in corpus is explicitly marked:

```text
provenance=synthetic_seed
reviewed=false
```

Any mismatch between provenance and review status fails closed. The evaluation receipt contains only corpus/query/candidate/path digests and aggregate metrics; it does not contain query text or KG node text.

### 3.2 Bounded two-hop KG evidence

The KG evaluator permits one or two directed hops only. Node and edge counts are bounded, traversal order is deterministic, cycles do not expand beyond the two-hop surface, and path selection uses stable fixed-point tie breaking. A path receipt includes only a SHA-256 digest and bounded counters.

### 3.3 Seven ablation lanes

The evaluator emits exactly one receipt for each lane:

1. lexical;
2. vector;
3. KG;
4. lexical + vector;
5. lexical + KG;
6. vector + KG;
7. lexical + vector + KG + fixed-point reranker.

All scores use integer parts per million. The reference reranker weights are checked in, digest-bound, deterministic, unlearned, unreviewed, and not production calibrated. Contradiction is an explicit subtractive feature.

### 3.4 Deterministic metrics

Recall@4, nDCG@4, citation precision, latency percentiles, and token cost are computed without floating point. Ranking is stable by descending score and ascending candidate ID. Every lane and the overall evaluation have SHA-256 receipts.

## 4. Executable qualification matrix

The dedicated ARM64 lane must execute on a frozen branch head with Rust 1.95.0 and produce readable artifacts for:

- exact stack-base ancestry;
- P1.1b bounded-decoder prerequisite markers;
- changed-path allowlist;
- Python source verifier;
- `cargo fmt --check`;
- all unit and integration tests;
- `cargo check --all-targets`;
- strict Clippy with `-D warnings`;
- deterministic receipt generation twice with byte equality;
- JSON parse and negative-authority assertions;
- clean tracked source and no committed `Cargo.lock` or `target/` output.

## 5. Exit semantics

Passing the executable matrix means only:

```text
implemented=true
source_qualified=true
seed_pipeline_reproducible=true
```

The following remain false:

```text
efficacy_validation=false
efficacy_claim=false
corpus_reviewed=false
production_calibrated=false
runtime_wired=false
product_workspace_member=false
product_module_registered=false
default_recall_changed=false
federation_recall_changed=false
context_attachment=false
physical_send=false
network_access=false
model_download=false
external_effects=false
production_authority=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

## 6. Follow-on tranche

P1.1c.1 must replace or supplement the synthetic seed corpus with a separately reviewed multilingual corpus that has documented provenance, adjudication, contradiction labels, citation support labels, privacy review, and immutable corpus digests. Only that reviewed-corpus tranche may set `efficacy_validation=true`, and it still cannot activate product recall without independent operator acceptance and promotion gates.
