# Hepta Intelligence P1.1c.1 — Reviewed Multilingual Corpus Acceptance

Status: `SOURCE_ONLY` / `ACTIVATION_BLOCKED`  
Stack parent: P1.1c Draft PR #45  
Parent head at tranche creation: `fe33565ce74c013e574c307e4fab101820c0ea88`

## 1. Objective

P1.1c.1 introduces the fail-closed acceptance boundary between the deterministic multilingual seed-evaluation pipeline and any future claim that a corpus has been independently human reviewed.

The tranche must prove that review provenance, reviewer independence, disagreement adjudication, citation/contradiction labels, privacy treatment, agreement metrics and immutable corpus digests can be validated without exposing raw queries, raw candidate text, reviewer identities or rationale text in machine receipts.

It does **not** provide a reviewed corpus. The checked-in data is a deliberately synthetic review seed used only to qualify the acceptance mechanism.

## 2. Dependency gate

P1.1c.1 is stacked on P1.1c and is acceptance-blocked until all of the following are true for one exact P1.1c commit:

```text
P1.1c source_qualified=true
P1.1c seed_pipeline_reproducible=true
P1.1c receipt redaction proof=true
P1.1c exact source commit matches review-batch metadata
```

At tranche creation, the P1.1c dependency remains unqualified. The seed receipt must therefore contain:

```text
dependency_source_qualified=false
dependency_seed_pipeline_reproducible=false
reviewed_corpus_accepted=false
corpus_reviewed=false
```

## 3. Isolated implementation

The implementation lives in a dependency-free nested workspace:

```text
codex-rs/hepta-memory-p1-1c1-qualification
```

It is not a member of the product workspace and is not imported by memory retrieval, federation, Agentd, context attachment, physical send, local model inference or any production caller.

## 4. Review evidence model

Every review item binds only digest-level evidence:

```text
item_id
locale
query_sha256
candidate_sha256
reviewer_commitment
relevance grade 0..3
citation label
contradiction label
privacy decision
rationale_sha256
```

Exactly two reviews are required per item. Their reviewer commitments must differ. Reviewer commitments are pseudonymous SHA-256 values; no reviewer name, email or account identifier is emitted in aggregate receipts.

## 5. Adjudication and privacy

A disagreement in any review field requires an independent adjudicator commitment. The adjudicator must differ from both reviewers.

Adjudication may select only relevance, citation and contradiction values that appeared in one of the two independent reviews. Privacy is stricter:

```text
allow < redact < block
```

The final privacy decision must equal the maximum severity selected by either reviewer. It can never be downgraded by adjudication.

`redact` requires a redaction-receipt SHA-256. `block` can never produce an accepted item. Missing adjudication is represented as unresolved and is conservatively resolved for computation only; it cannot pass corpus acceptance.

## 6. Agreement metrics

All metrics use checked integer PPM arithmetic:

```text
exact tuple agreement
relevance agreement
citation agreement
contradiction agreement
privacy agreement
quadratic-weighted relevance Cohen kappa
```

Default source policy:

```text
minimum items = 8
minimum locales = 8
minimum exact tuple agreement = 700000 ppm
minimum weighted relevance kappa = 600000 ppm
zero unresolved disagreements required
zero privacy-blocked items required
all reviewed items accepted
```

No floating point or model-generated adjudication is used.

## 7. Immutable receipts

The implementation generates deterministic digests for:

```text
reviewer set
review batch
adjudication batch
review pair
resolved item
reviewed corpus
aggregate acceptance receipt
```

Item identifiers are hashed before they enter item receipts. The JSON aggregate receipt does not emit item IDs, query digests, candidate digests, reviewer commitments, adjudicator commitments or rationale digests.

## 8. Synthetic seed boundary

Checked-in fixtures declare:

```text
provenance=synthetic_review_seed
reviewed=false
human_review_attested=false
```

They cover eight locales, sixteen review rows, two independent reviewer commitments and two adjudications. They test the pipeline only.

A synthetic seed must emit:

```text
status=PASS_P1_1C1_REVIEW_PIPELINE_SEED_ONLY
review_pipeline_validated=true
reviewed_corpus_accepted=false
corpus_reviewed=false
efficacy_validation=false
efficacy_claim=false
```

## 9. Real corpus acceptance requirements

A real reviewed-corpus receipt can be accepted only when all of these conditions hold simultaneously:

```text
provenance=human_reviewed_v1
reviewed=true
human_review_attested=true
P1.1c dependency commit matches and is source-qualified
P1.1c seed evidence is reproducible
two independent reviews exist for every item
all disagreements are independently adjudicated
all required redaction receipts are present
no privacy-blocked item remains
agreement thresholds pass
all item and aggregate digests validate
```

Even successful corpus acceptance does not establish retrieval efficacy. A later P1.1c.2 tranche must rerun the seven-lane offline evaluation against the accepted immutable corpus digest.

## 10. Qualification matrix

```text
python3 scripts/verify-hepta-intelligence-p1-1c1-reviewed-corpus.py
cargo fmt --manifest-path codex-rs/hepta-memory-p1-1c1-qualification/Cargo.toml --all -- --check
cargo test --manifest-path codex-rs/hepta-memory-p1-1c1-qualification/Cargo.toml --all-targets -- --nocapture
cargo check --manifest-path codex-rs/hepta-memory-p1-1c1-qualification/Cargo.toml --all-targets
cargo clippy --manifest-path codex-rs/hepta-memory-p1-1c1-qualification/Cargo.toml --all-targets -- -D warnings
cargo run --quiet --manifest-path codex-rs/hepta-memory-p1-1c1-qualification/Cargo.toml --bin p1_1c1_receipt
```

The receipt binary is executed twice and must be byte-identical. JSON semantics and redaction are checked separately.

## 11. Authority boundary

```text
implemented=true
source_qualified=false
review_pipeline_validated=true
reviewed_corpus_accepted=false
corpus_reviewed=false
human_review_attested=false
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

Keep Draft. Do not merge independently of P1.1c, replace the synthetic seed with unreviewed data, reveal reviewer identities, treat pipeline validation as human review, rerun product recall, attach context, enable physical send, claim efficacy or grant production authority.
