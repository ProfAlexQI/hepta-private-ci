# Hepta Intelligence P1.1c.2 — Reviewed Corpus Offline Efficacy Re-run

Status: `IMPLEMENTED_SOURCE_ONLY` / `REAL_CORPUS_DEPENDENCY_BLOCKED`  
Stack parent: P1.1c.1 Draft PR #49  
Exact parent head: `f961a056ac0a35c1967a934de7cf5bf7ffb92a05`

## 1. Objective

P1.1c.2 is the fail-closed bridge from an accepted P1.1c.1 immutable reviewed-corpus receipt to a new deterministic seven-lane offline efficacy evaluation.

It does not accept a corpus merely because a file says `reviewed=true`. It recomputes the P1.1c.1 acceptance receipt, binds every reviewed item to one exact evaluation candidate, proves complete candidate coverage, then reruns the P1.1c lexical/vector/KG ablation matrix and emits digest-bound deltas against the original synthetic-seed baseline.

The checked-in inputs remain qualification fixtures. No real human-reviewed corpus is added by this tranche.

## 2. Exact dependency chain

A reviewed evaluation can run only when one request proves all of the following:

```text
P1.1c exact source commit matches
P1.1c source_qualified=true
P1.1c seed_pipeline_reproducible=true
P1.1c.1 exact source commit matches
P1.1c.1 acceptance receipt validates
P1.1c.1 acceptance receipt recomputes byte-for-byte from the review batch
reviewed_corpus_accepted=true
corpus_reviewed=true
human_review_attested=true
review projection is not fixture-only
all review items map to exact evaluation candidates
all evaluation candidates have exactly two bound reviews
query digests match
candidate projection digests match
locale bindings match
```

Any unmet condition produces a blocker-code-only receipt with zero lane evidence.

## 3. Isolated implementation

The implementation lives in:

```text
codex-rs/hepta-memory-p1-1c2-qualification
```

It is a nested, publish-disabled workspace. It uses only local path dependencies on the isolated P1.1c and P1.1c.1 qualification crates. It is not a member of the product `codex-rs` workspace and is not imported by Memory, federation, Agentd, context attachment, physical send, model inference or a production caller.

## 4. Review-to-evaluation projection

P1.1c.1 accepts review items; P1.1c evaluates cases containing multiple candidates. P1.1c.2 therefore adds an explicit projection contract:

```text
item_id
case_id
candidate_id
query_sha256
candidate_projection_sha256
```

The candidate projection digest covers the exact evaluation-relevant candidate representation:

```text
case and locale binding
candidate identity
relevance grade
lexical and vector scores
citation support
latency and token cost
bounded KG nodes and edge truth/contradiction values
```

The projection fails closed unless:

```text
review item count == evaluation candidate count
projection row count == evaluation candidate count
every review item is projected exactly once
every case/candidate pair is projected exactly once
every query digest matches the evaluation case
every candidate digest matches the evaluation candidate representation
every review locale matches the evaluation case locale
fixture_only=false
```

This closes the current seed mismatch: P1.1c.1 contains 8 reviewed items while the P1.1c evaluation corpus contains 48 candidates. The checked-in seed is therefore intentionally ineligible for reviewed-corpus efficacy evaluation.

## 5. Seven-lane rerun

After the complete acceptance and projection chain passes, the engine reruns exactly:

1. lexical;
2. vector;
3. KG;
4. lexical + vector;
5. lexical + KG;
6. vector + KG;
7. lexical + vector + KG + fixed-point reranker.

For every lane the receipt includes only aggregate reviewed-corpus metrics and signed deltas against the exact P1.1c seed baseline:

```text
Recall@4
nDCG@4
citation precision
P50 latency
P95 latency
mean top-four token cost
```

No raw query, candidate, reviewer, adjudicator, rationale or review-item identifier enters the machine receipt.

## 6. Source efficacy policy

The source-only default policy is explicit, deterministic and uncalibrated:

```text
minimum cases = 8
minimum locales = 8
minimum full-lane Recall@4 = 750000 ppm
minimum full-lane nDCG@4 = 700000 ppm
minimum full-lane citation precision = 400000 ppm
maximum full-lane P95 synthetic latency = 1000 microseconds
maximum full-lane mean top-four token cost = 512
production_calibrated=false
```

A passing policy may set `efficacy_validation=true` only for the exact reviewed offline corpus and exact source evidence. It cannot set `efficacy_claim=true`, calibrate production ranking, alter default recall or grant production authority.

## 7. Checked-in fixture boundary

The checked-in projection declares:

```text
fixture_only=true
projection rows=8
review items=8
evaluation candidates=48
```

It must emit:

```text
status=BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY
reviewed_corpus_present=false
projection_complete=false
reviewed_corpus_evaluated=false
efficacy_thresholds_passed=false
efficacy_validation=false
lanes=[]
```

Positive-path Rust tests construct qualification-only in-memory evidence covering all candidates. Those tests prove the engine path, not the existence of an external human-reviewed corpus.

## 8. Qualification matrix

```text
python3 scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py
cargo fmt --manifest-path codex-rs/hepta-memory-p1-1c2-qualification/Cargo.toml --all -- --check
cargo test --manifest-path codex-rs/hepta-memory-p1-1c2-qualification/Cargo.toml --all-targets -- --nocapture
cargo check --manifest-path codex-rs/hepta-memory-p1-1c2-qualification/Cargo.toml --all-targets
cargo clippy --manifest-path codex-rs/hepta-memory-p1-1c2-qualification/Cargo.toml --all-targets -- -D warnings
cargo run --quiet --manifest-path codex-rs/hepta-memory-p1-1c2-qualification/Cargo.toml --bin p1_1c2_receipt
```

The receipt binary is executed twice and must be byte-identical. CI also validates JSON semantics, blocker codes, receipt redaction, exact changed paths, local-only dependencies and a clean source tree.

## 9. Authority boundary

```text
implemented=true
source_qualified=false
reviewed_corpus_present=false
reviewed_corpus_evaluated=false
efficacy_validation=false
efficacy_claim=false
product_workspace_member=false
product_module_registered=false
runtime_wired=false
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

Keep Draft. Do not merge independently of P1.1c.1, treat qualification-only positive tests as external human review, substitute an eight-item review seed for a 48-candidate evaluation corpus, expose raw review content, wire product recall, attach context, enable physical send, claim production efficacy or grant production authority.

## 10. Exit and follow-on

P1.1c.2 may leave dependency-blocked status only after an external P1.1c.1 corpus is accepted with complete candidate coverage and exact immutable digests. A later operator-calibration and promotion tranche must independently review the corpus license, privacy posture, reviewer trust, metric thresholds and production calibration before any product caller can change.
