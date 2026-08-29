# Hepta Intelligence Controlled Gap-Closure Execution Specification

> **SUBORDINATE_EXECUTION_SPEC / PLAN_ONLY / FAIL_CLOSED / NO_PROMOTION_AUTHORITY**
>
> Specification ID: `HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1`
>
> Version: `1.2.0`
>
> Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.4.0`
>
> Repository: `ProfHepta/hepta-private-ci`
>
> Active phase: `A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY`

This specification is the package-level implementation blueprint for the canonical master plan. It grants no current-plan, runtime, model, provider, operator, production, promotion, release or `CALLERS` authority.

---

## 1. Exact baseline and A0 objective

```text
repository = ProfHepta/hepta-private-ci
Q0 branch = codex/hepta-intelligence-plan-v3-20260828
Q0 head = c768bcbeb4c1168088d2499828c24da521a2a73a
Q0 tree = ca455a9ef797cd95164c880c7b8faba80b305589
Q0 parent = aeb8ac0bfb30d570a16c4914b6e4b31ce035dd62
Q0 run = 33252922404
A0 branch = codex/hepta-intelligence-a0-authority-gap-closure-20260829
```

A0 remains exactly one direct child of Q0 and may change documentation, registry, verifier and read-only workflow only. Runtime, SQL migration, product caller, model/provider dispatch, multimodal intake/index/retrieval, activation, release and `CALLERS` are frozen.

Immediate sequence:

```text
exact-parent replacement → source gates → real-runner exact-head receipt
→ independent review → canonical selection → merge-candidate attribution
```

---

## 2. Replacement-commit and CI-trigger protocol

Repairs replace the A0 tree and create one commit whose sole parent is `c768bcb...`; appending a second A0 commit is invalid.

**A candidate workflow must never modify**, commit, push or update candidate source. CI permissions remain read-only and evidence workflow identity remains distinct from source publisher identity. Every replacement invalidates all superseded-head artifacts and reviews.

`SOURCE_SNAPSHOT` contains deterministic contracts, not guessed live CI. `LIVE_EVIDENCE` is valid only through an exact receipt binding repository/id, branch/head/tree/parent, workflow path/SHA, run/attempt/event, job/runner/labels/non-empty successful steps, artifact id/name/digest/expiry, source/current-truth digests and all-negative authority. Queued, pending, `steps=[]`, `runner_id=0`, PR prose and superseded artifacts are not PASS.

---

## 3. Gap-loop state machine

```text
DISCOVER → CLASSIFY → BIND_EXACT_BASE → DEFINE_ACCEPTANCE
→ IMPLEMENT_SMALLEST_COHERENT_SLICE → SOURCE_GATES → EXECUTABLE_GATES
→ BIND_RECEIPT → REVALIDATE → CLOSE | BLOCK_EXTERNAL | BLOCK_UPSTREAM | STOP
```

Legal repository classes: `CLOSED_SOURCE_CONTROLLED`, `OPEN_SOURCE_CONTROLLED`, `BLOCKED_EXTERNAL_EVIDENCE`, `BLOCKED_UPSTREAM`, `STOP_CONDITION`.

Fixtures may qualify mechanics but cannot close real corpus, reviewer, selector, model-license approval, hardware, soak, operator, canary, promotion, release or `CALLERS` facts.

---

## 4. Global contract, error and attribution rules

Every durable/cross-package contract contains schema/version/receipt ID, owner/tenant scope, monotonic creation identity, payload/code digests, privacy/retention/training/retrieval eligibility and an explicit all-negative authority map.

Rules: `deny_unknown_fields`; canonical serialization; domain-separated SHA-256; bounded strings/collections/encoded bytes; reject lengths before allocation; exact owner/tenant/run/episode/source/model/preprocessor/policy/dataset/code binding; explicit migration/downgrade; unknown fails closed.

Initial hard bounds:

```text
contract/event <= 128 KiB; episode <= 1 MiB
candidate inventory <= 1,024; scored <= 128; returned <= 32
KG hops <= 2; nodes/edges <= 256/1,024
mutation transitions <= 64; receipt metadata entries <= 256
```

`RepositoryCheckAttributionReceiptV1.classification` is exactly one of:

```text
PASS
INTRODUCED_BY_CANDIDATE
PRE_EXISTING_ON_BASE
MERGE_INTERACTION
RUNNER_OR_PLATFORM_INFRA
CANCELLED_OR_SUPERSEDED
NOT_REQUIRED_BY_SELECTED_POLICY
UNKNOWN_FAIL_CLOSED
```

Any other value or `UNKNOWN_FAIL_CLOSED` blocks merge.

---

## 5. B0 — Learning boundary extraction

Target packages:

```text
codex-rs/hepta-intelligence-contracts / codex-hepta-intelligence-contracts
codex-rs/hepta-mutation-core          / codex-hepta-mutation-core
codex-rs/hepta-grounding              / codex-hepta-grounding
codex-rs/hepta-mutation-journal       / codex-hepta-mutation-journal
codex-rs/hepta-mutation-coordinator   / codex-hepta-mutation-coordinator
codex-rs/hepta-retrieval              / codex-hepta-retrieval
codex-rs/hepta-policy-runtime         / codex-hepta-policy-runtime
codex-rs/hepta-learning-ledger        / codex-hepta-learning-ledger
codex-rs/hepta-intelligence-eval      / codex-hepta-intelligence-eval
```

Allowed DAG is contracts-first; cycles/reverse dependencies, contracts importing runtime, retrieval importing policy/eval, copied qualification algorithms, Cargo/Bazel divergence and default-enabled effect features are forbidden.

B0 slices:

```text
B0.1 shared IDs/digests/envelopes        → B0ContractsExtractionReceiptV1
B0.2 pure mutation state/property parity → B0MutationCoreParityReceiptV1
B0.3 grounding validator parity          → B0GroundingParityReceiptV1
B0.4 journal adapter/schema/failpoints    → B0JournalParityReceiptV1
B0.5 retrieval planner/fusion contracts  → B0RetrievalBoundaryReceiptV1
B0.6 learning contracts/lineage          → B0LearningBoundaryReceiptV1
B0.7 policy/eval no-effect interfaces    → B0PolicyEvalBoundaryReceiptV1
B0.8 duplicate removal/parity closure    → B0BoundaryClosureReceiptV1
```

Every slice is an isolated Draft PR on the previous exact-qualified slice with rollback and `PackageHandoffReceiptV1`.

---

## 6. Field-level causal contracts

Required contracts include `RunStartSnapshotV2`, `CandidateSetReceiptV1`, `LearningEpisodeV1`, `LearningEventV1`, `PolicyDecisionReceiptV2`, `ExplorationPolicyReceiptV1`, `OutcomeReceiptV1`, `CreditLedgerV1`, `DatasetSnapshotV1`, `EvaluationReceiptV2`, `PolicyArtifactManifestV2` and `UnlearningComplianceReceiptV1`.

The complete candidate set, propensity support, causal parent, correction/forget/revocation lineage, dataset eligibility and next-snapshot rollback predecessor are mandatory. Current runs are never modified in place.

---

## 7. A0 commands and evidence receipts

```bash
python3 -m py_compile scripts/hepta-intelligence-current-truth.py \
  scripts/verify-hepta-intelligence-a0-authority.py \
  scripts/verify-hepta-intelligence-document-authority.py \
  scripts/verify-hepta-intelligence-master-plan.py
python3 scripts/verify-hepta-intelligence-master-plan.py
python3 scripts/verify-hepta-intelligence-document-authority.py
python3 scripts/hepta-intelligence-current-truth.py --verify
GITHUB_SHA="$EXACT_HEAD" python3 scripts/verify-hepta-intelligence-a0-authority.py
python3 scripts/hepta-intelligence-current-truth.py --compact > /tmp/a
python3 scripts/hepta-intelligence-current-truth.py --compact > /tmp/b
cmp /tmp/a /tmp/b
git diff --check HEAD^ HEAD
test -z "$(git status --porcelain --untracked-files=no)"
```

Expected markers:

```text
PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_4_SOURCE_ONLY
PASS_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_V4_4
PASS_HEPTA_INTELLIGENCE_CURRENT_TRUTH_V1
PASS_HEPTA_INTELLIGENCE_A0_SOURCE_ONLY
PASS_HEPTA_INTELLIGENCE_A0_EXECUTABLE_QUALIFICATION
```

A0 exact-head receipt closes candidate qualification only. `A0IndependentReviewReceiptV1` requires a distinct reviewer identity/affiliation, exact head/tree/artifact digests, findings/dispositions, conflict declaration, decision, signature and expiry.

---

## 8. C0 — Durable learning episode ledger

Owned tables/records: migrations, run snapshots, episodes, events, candidate sets, outcomes, credit entries, lineage edges, unlearning watermarks and checkpoints. Tables are append-only; mutable pointers are separate CAS rows. Required gates include schema oracle, append/replay, duplicate/reorder/foreign-owner rejection, outcome revision, credit conservation, unlearning lineage and deterministic reopen. C0 remains default-off.

---

## 9. M0 — Transactional mutation coordinator

Host-owned sequence:

```text
prepare immutable request → persist binding → begin transaction
→ verify lease/revision/generation → write source+memory+facts
→ append mutation transition → append transactional outbox → COMMIT
→ asynchronous dispatch → store/provider acknowledgement
→ append reconciliation/terminal transition
```

Only the observing store/adapter produces MemoryWritten, ProjectionPublished, OutboxDispatched or ReconcileApplied. E3 covers before/after commit, ACK loss, stale lease, duplicate request, process kill, disk full, permission loss and restart.

---

## 10. J0 — Journal lifecycle, recovery and capacity

Required design: active/history epochs; terminal checkpoint; chained root; incremental startup; create-only verified archive; background scrub; bounded admission/backpressure; corruption quarantine; backup/restore rehearsal; measured recovery/storage-growth SLO. Minimum evidence is x86_64 E3 plus an independent platform E3 receipt.

---

## 11. R1 — Grounded hybrid retrieval

```text
R1.0 side-stack selection receipt
R1.1 deterministic planner
R1.2 host evidence resolver
R1.3 legacy inventory/quarantine
R1.4 product module
R1.5 provider/tokenizer/index contracts
R1.6 real local semantic artifact
R1.7 mechanics reuse only from unselected side stacks
R1.8 shadow product integration and rollback
```

Hash-one-hot providers and synthetic seeds qualify mechanics only. A real `SemanticArtifactEvidencePackageV1` and a `ReviewedCorpusEvidencePackageV1` are required before efficacy.

---

## 12. N1 — H5 adaptive signal unit

Sparse activation, lateral inhibition, homeostasis, bounded eligibility trace, calibrated confidence, abstention/OOD, a shared frozen backbone plus adapter identity, next-snapshot proposal, lesion/ablation and rollback. No biological replication or neuromorphic claim.

---

## 13. I1 — H6 calibrated fast policy

Every decision emits `PolicyDecisionReceiptV2`; high-risk actions abstain, reject or use the slow governed path. Full candidate set, propensity support, held-out calibration, OOD, subgroup and rollback evidence are mandatory.

---

## 14. L1 — H7 causal evaluation and artifact pipeline

Required: support coverage, ESS, IPS, SNIPS, doubly robust, cluster/bootstrap CI, candidate LCB, baseline UCB, safety, subgroup, retention, privacy and resource evidence. Promotion requires:

```text
candidate LCB > baseline UCB
```

plus a signed rollback-capable next-snapshot artifact.

---

## 15. C1 — Low-risk closed loop

The first action domain is read-only `MemoryRetrievalRank`; auth/tool/provider/credential/topology mutation remains forbidden. Shadow, bounded canary, delayed outcomes, causal evaluation, operator acceptance, separate CALLERS receipt, kill switch and rollback are mandatory.

---

## 16. MM0 — Multimodal contract and authority foundation

### MM0-A scope, asset and segment

Outputs:

```text
MemoryScopeV2
MemoryAssetManifestV1
MediaSegmentV1
AssetLifecycleV1
AssetIntakeReceiptV1
```

Contracts bind canonical scope, immutable asset revision/content digest, MIME/modality/codec, object/key identity, privacy/trust/retention, byte/pixel/page/frame/sample/duration limits and precise byte/page/rectangle/time/frame/text locators. Unknown fields and pre-allocation overflow are rejected.

### MM0-B derivative, model, embedding, query and deletion

Outputs:

```text
DerivedArtifactV1
EvidenceSetV1
ModelArtifactManifestV1
EmbeddingSpaceManifestV1
EmbeddingManifestV1
CrossModalQueryV1
MultimodalCandidateReceiptV1
MultimodalRerankReceiptV1
MultimodalRetrievalReceiptV1
RevalidationBindingV1
ContextCompilationReceiptV1
DeletionPropagationReceiptV1
```

Generated artifacts always keep `source_truth=false`; model name alone is invalid; vector spaces/generations cannot be mixed without an adapter receipt; all authority remains false.

---

## 17. MM1 — Encrypted asset, segment and derivation ledger

Object-store boundary: encrypted immutable original/derived blobs keyed by asset/content generation. Database boundary: manifests, lifecycle heads/revisions, segments, key references, retention/legal hold, derivation edges and transactional outbox.

Write protocol: stream to quarantine while hashing/bounding → encrypt temporary object under lease → validate metadata → coordinator commits manifest/journal/outbox → finalize idempotently → reconcile orphan/missing objects → emit store-derived receipt.

Failure matrix includes crash before/after object write and database commit, duplicate key, stale CAS, disk full, partial/fsync failure, missing/corrupt object, key unavailable, restart, concurrent forget/intake and backup object/key mismatch.

---

## 18. MM2 — Sandboxed decode, extraction, redaction and quarantine

Decoder worker has no network, credentials, repository write or production token. It enforces compressed/decompressed bytes, pixels/edges/channels, pages/nested objects, frames/duration/rate/resolution, audio sample limits, CPU/wall/RSS/VRAM/temp-disk and derivative-count caps.

Security corpus covers malformed/truncated/polyglot media, bombs, external references, codec crash/hang, QR/barcode secrets, OCR confusables, prompt injection and adversarial audio/images/documents.

OCR/ASR/caption/scene/entity/redaction/thumbnail providers are registered immutable artifacts; failure, low confidence or unsupported codecs produce explicit abstain/quarantine, not empty success.

---

## 19. MM3 — Model registry, embedding spaces and immutable indexes

Artifact import is offline and digest-bound: weights, size, license, provenance/SBOM, quantization/runtime, tokenizer/preprocessor, modalities, limits, hardware compatibility, resource envelope and security review. Runtime download and network access are forbidden.

The typed provider accepts asset, segment, derived artifact, semantic memory revision or query media/text—not a bare `&str`. Output binds subject digest, model manifest, space/generation, dimensions/metric/encoding/normalization, vector digest and resource receipt.

Indexes are create-only generations with stable ordering, exact reopen, bounded candidates and scope partitions. Rebuild uses dual-read shadow; cutover and old-generation GC require receipts.

---

## 20. MM4 — Cross-modal retrieval and exact revalidation

Channels: memory FTS, OCR/transcript FTS, entity/KG/temporal, recency, text/image/document/audio/video embeddings and perceptual duplicate index. Every channel has quota/deadline; fusion is deterministic and bounded; reranker cannot create candidates.

Before return, bind scope/delegation, asset lifecycle/revision/digest, segment locator/digest, derivative lineage, model/preprocessor, memory head/citations, KG generation, embedding space/vector/index generation, policy/redaction/deletion state. Stale/revoked/deleted candidates are dropped with reason code.

---

## 21. MM5 — Semantic consolidation and bounded context

Source, derivative, proposed assertion, verified assertion, contradicted and revoked states are distinct. High-risk identity/biometric/credential/financial/medical/legal assertions require review or abstention. Corrections append revisions.

Context outputs are bounded text/OCR/transcript spans, approved crop/thumbnail, timestamped frames, short approved audio excerpt or content-free reference. Each fragment has exact citation, risk class, deterministic order and token/pixel/frame/byte caps.

---

## 22. MM6 — Deletion, security, efficacy, hardware, soak and canary

Deletion traverses source asset/key/object, segment, derivative, embedding/vector, index, cache/context, dataset, adapter/checkpoint/policy/evaluation and backup-restore non-resurrection. A content-free audit digest may remain.

Security campaign covers cross-scope ANN, embedding inversion/membership, multimodal injection, adversarial media, decoder crash/timeout, path/object-key traversal, forged model/preprocessor/receipt digest, stale generation/replay, PII/secret/biometric leakage and quota DoS.

External efficacy requires reviewed corpus provenance/consent/privacy, immutable digest, independent labels/adjudication, modality/query/locale/risk/task balance, exact grounding, OOD/correction/forget cases, target hardware/resource/energy, soak, shadow/canary and rollback.

---

## 23. Package dependency graph and PR staging

```text
A0 selected
  └─ B0 boundary/scope extraction
       ├─ C0 learning ledger
       ├─ M0 coordinator ─ J0 lifecycle
       ├─ R1 grounded text retrieval
       └─ MM0 contracts
            └─ MM1 asset/derivation ledger
                 └─ MM2 decode/extraction/redaction
                      └─ MM3 embedding/index generations
                           └─ MM4 retrieval/revalidation
                                └─ MM5 consolidation/context
                                     └─ MM6 deletion/security/efficacy/canary
```

Package PRs: `MM0-A`, `MM0-B`, `MM1-A`, `MM1-B`, `MM2-A`, `MM2-B`, `MM3-A`, `MM3-B`, `MM4-A`, `MM4-B`, `MM5`, `MM6-A`, `MM6-B`. Each uses one isolated branch, Draft PR, exact base, explicit paths, rollback, source/E1/E2 receipts and no self-merge.

---

## 24. Gap loop algorithm and exit states

For each package:

```text
1. Revalidate repository/ref/head/tree/parent and plan authority.
2. Read global and multimodal ledgers plus registered consumers.
3. Select the highest-severity unblocked source-controlled gap.
4. Implement the smallest coherent slice with negative authority.
5. Run E0/E1 and create exact receipts.
6. Publish without CI source writeback; obtain E2.
7. Run E3/E4 where required.
8. Independently review source, evidence, security, rollback and claims.
9. Update ledger truth; never infer closure from intent.
10. Continue until MODULE_CLOSED_CANDIDATE or valid BASE_DRIFT,
    BLOCKED_UPSTREAM, BLOCKED_EXTERNAL_EVIDENCE, STOP_CONDITION or RESUME_REQUIRED.
```

`MODULE_CLOSED_CANDIDATE` requires all package source gaps closed, exact-head executable evidence, independent review, rollback and honest authority. It does not imply merge, promotion, release or production.

---

## 25. Current claim boundary

```text
system = L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS
H5 = N0_METAPHORICAL_TYPED_PROPOSAL
H6 = I0_DETERMINISTIC_SELECTIVE_POLICY
multimodal_memory = MM0_SPECIFIED_ONLY
native_media_memory_wired = false
cross_modal_retrieval_qualified = false
multimodal_efficacy_proven = false
runtime_wired = false
production_authority = false
```
