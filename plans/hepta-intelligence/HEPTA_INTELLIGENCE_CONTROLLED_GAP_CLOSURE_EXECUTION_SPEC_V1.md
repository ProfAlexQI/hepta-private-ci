# Hepta Intelligence Controlled Gap-Closure Execution Specification

> **SUBORDINATE_EXECUTION_SPEC / PLAN_ONLY / FAIL_CLOSED / NO_PROMOTION_AUTHORITY**
>
> Specification ID: `HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1`
>
> Version: `1.1.0`
>
> Canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `4.2.0`
>
> Repository: `ProfHepta/hepta-private-ci`
>
> Active phase: `A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY`

This specification is the package-level **Implementation Blueprint** for the
canonical master plan. It defines exact package ownership, field-level
contracts, command/receipt gates, failure campaigns, SLOs, external evidence
packages, handoffs and stop conditions. It grants no runtime, model, provider,
operator, production, promotion, release or `CALLERS` authority.

---

## 1. Exact baseline and A0 objective

```text
repository = ProfHepta/hepta-private-ci
Q0 branch = codex/hepta-intelligence-plan-v3-20260828
Q0 head = c768bcbeb4c1168088d2499828c24da521a2a73a
Q0 tree = ca455a9ef797cd95164c880c7b8faba80b305589
Q0 parent = aeb8ac0bfb30d570a16c4914b6e4b31ce035dd62
Q0 run = 33252922404
Q0 evidence = E1 x86_64 + E2 ARM64 + paired receipt
A0 branch = codex/hepta-intelligence-a0-authority-gap-closure-20260829
```

A0 must remain exactly one direct child of Q0. It is documentation/registry/
verifier/read-only-workflow only. Rust runtime, SQL migration, product caller,
tool registration, model/provider dispatch, H5/H6/H7 runtime, activation,
release and `CALLERS` are frozen.

Immediate sequence:

```text
exact-parent replacement → source gates → real-runner exact-head receipt
→ independent review → canonical selection → merge-candidate attribution
```

---

## 2. Replacement-commit and CI-trigger protocol

Repairs replace the A0 tree and create one commit whose sole parent is
`c768bcb...`; appending a second A0 commit is invalid.

A candidate workflow must never modify, commit or push candidate source. CI has
`contents: read`, may emit diagnostics only, and is separate from the source
publisher. Every replacement invalidates superseded-head evidence.

Checked-in state is `SOURCE_SNAPSHOT`, not live CI. `LIVE_EVIDENCE` is valid
only when a receipt binds repository/id, branch/head/tree/parent, workflow path
and SHA, run/attempt/event, job/runner/labels/non-empty successful steps,
artifact id/name/digest/expiry, source/current-truth digests and authority map.
Queued, `steps=[]`, `runner_id=0`, synthetic merge refs and PR prose are not
PASS.

---

## 3. Gap-loop state machine

```text
DISCOVER → CLASSIFY → BIND_EXACT_BASE → DEFINE_ACCEPTANCE
→ IMPLEMENT_SMALLEST_COHERENT_SLICE → SOURCE_GATES → EXECUTABLE_GATES
→ BIND_RECEIPT → REVALIDATE → CLOSE | BLOCK_EXTERNAL | BLOCK_UPSTREAM | STOP
```

Legal classes: `CLOSED_SOURCE_CONTROLLED`, `OPEN_SOURCE_CONTROLLED`,
`BLOCKED_EXTERNAL_EVIDENCE`, `BLOCKED_UPSTREAM`, `STOP_CONDITION`.
Fixtures may qualify mechanics but cannot close real corpus, reviewer,
hardware, soak, operator, promotion, release or `CALLERS` facts.

---

## 4. Global contract and error rules

Every durable/cross-package contract contains:

```text
schema/version/receipt_id
owner_agent_id/tenant_scope_sha256
created_at_monotonic/payload_sha256/code_identity
privacy_class/retention_class/training_eligibility/retrieval_eligibility
authority map/receipt_binding_sha256
```

Rules: `deny_unknown_fields`; canonical serialization; domain-separated
SHA-256; bounded strings/collections/encoded bytes; reject lengths before
allocation; explicit migration/downgrade; exact owner/tenant/run/episode,
source/model/tokenizer/policy/dataset/code binding; all authority false unless
a later independently governed schema explicitly permits it.

Error taxonomy:

```text
INVALID_INPUT / CONFLICT / ACCESS_DENIED / UNAVAILABLE / INDETERMINATE
CORRUPT / BLOCKED_UPSTREAM / BLOCKED_EXTERNAL / STOP_IDENTITY_DRIFT
```

An error that may follow commit carries idempotency identity and a store-derived
reconciliation cursor.

Initial hard bounds:

```text
contract/event <= 128 KiB; episode <= 1 MiB
candidate inventory <= 1,024; scored <= 128; returned <= 32
KG hops <= 2; nodes/edges <= 256/1,024
mutation transitions <= 64; receipt metadata entries <= 256
```

---

## 5. Package Implementation Blueprint

### B0 — Learning boundary extraction

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

Allowed DAG:

```text
contracts
├─ mutation-core
├─ grounding
├─ learning-ledger
├─ retrieval → grounding
├─ mutation-journal → mutation-core
├─ policy-runtime → retrieval
├─ mutation-coordinator → grounding + mutation-core + mutation-journal
└─ intelligence-eval → retrieval + policy-runtime + learning-ledger
```

Forbidden: cycles/reverse dependencies; contracts importing runtime; storage
importing Agentd/UI/provider; retrieval importing policy/eval; policy writing
learning ledger; eval importing effects; copied qualification algorithms;
Cargo/Bazel divergence; default-enabled effect features.

B0 slices and receipts:

```text
B0.1 shared IDs/digests/envelopes       → B0ContractsExtractionReceiptV1
B0.2 pure mutation state/property parity→ B0MutationCoreParityReceiptV1
B0.3 grounding validator parity         → B0GroundingParityReceiptV1
B0.4 journal adapter/schema/failpoints  → B0JournalParityReceiptV1
B0.5 retrieval planner/fusion contracts → B0RetrievalBoundaryReceiptV1
B0.6 learning contracts/lineage         → B0LearningBoundaryReceiptV1
B0.7 policy/eval no-effect interfaces   → B0PolicyEvalBoundaryReceiptV1
B0.8 remove duplicate implementations  → B0BoundaryClosureReceiptV1
```

Each slice is a separate Draft PR on the previous exact-qualified slice, with
changed-path allowlist, default-off feature, Cargo/Bazel parity, no-behavior-
change proof, rollback pointer and `PackageHandoffReceiptV1`.

---

## 6. Field-level causal contracts

### `RunStartSnapshotV2`

```text
run_id, agent_id, tenant_scope_sha256, process/spawn generation
code commit/tree, configuration_sha256, policy_artifact_sha256
model/tokenizer artifact SHA256?, retrieval index generation?
memory revision, projection generation, wall-clock observation
monotonic counter, privacy/retention/training eligibility
```

Producer: runtime host at admission. Immutable; absent optional artifacts are
`None`, never empty strings.

### `CandidateSetReceiptV1`

```text
episode_id/decision_slot, candidate_count, ordered candidate IDs
per-candidate provenance channel, grounding/truth/risk eligibility
score-feature digests, inclusion/exclusion reason, budget used
candidate_set_sha256
```

Must include every legally selectable action, including abstain/slow-path;
post-selection or truncated inventories are invalid for OPE.

### `LearningEpisodeV1` and `LearningEventV1`

```text
episode/run/context snapshot identity; episode kind/risk class
candidate-set and decision receipt digests; effect/postcondition receipt?
outcome/credit receipt?; policy/model/tokenizer/dataset identities
causal parent/sequence/terminal state; correction/forget/revocation state
```

Event kinds: opened, candidates captured, decision logged, effect observed,
postcondition observed, outcome attached, credit assigned, corrected,
forgotten, revoked, terminalized, quarantined. Producer is the component that
observes the fact. Events are append-only; corrections are new events.

### `PolicyDecisionReceiptV2` and `ExplorationPolicyReceiptV1`

Decision fields: complete candidate-set digest, selected action/abstain,
propensity numerator/denominator, support status, policy/model/adapter identity,
confidence/calibration bin, OOD score/bound, risk, veto/slow-path reason,
latency/resource receipt and decision digest.

Exploration fields: eligible action domain, risk/privacy exclusions,
randomization algorithm and committed seed, minimum propensity floor, clipping
policy, support target/window, kill-switch identity and approved scope. First
exploration domain is read-only `MemoryRetrievalRank`; auth/tool/provider/
credential/topology mutation is forbidden.

### `OutcomeReceiptV1` and `CreditLedgerV1`

Outcome fields: episode, type/source, observation timestamp/delay bucket,
fixed-point components, missing/censored reason, quality/safety/privacy labels,
review/effect provenance and digest. Revisions are new receipts.

Credit fields: episode/outcome identity, eligible decisions, method/version,
per-decision signed fixed-point credit, residual, discount/delay assumptions and
digest. Property: assigned credit plus declared residual equals frozen outcome
exactly.

### Evaluation/artifact/unlearning contracts

`DatasetSnapshotV1` binds included episode/event digests, filters, exclusions,
unlearning watermarks, split strategy and leakage checks.
`EvaluationReceiptV2` binds dataset, estimators, support, ESS, IPS, SNIPS, DR,
cluster/bootstrap CI, subgroup/safety/retention/resource results and frozen
thresholds. `PolicyArtifactManifestV2` binds parent artifact, training/eval
receipts, model/tokenizer/code, signatures, expiry/revocation, rollback and
`next-snapshot`; it never activates itself.

`UnlearningComplianceReceiptV1` binds correction/forget/revocation to affected
source/memory/episode IDs and every derived dataset/index/adapter/policy/eval
artifact, tombstone or crypto-shred evidence, rebuild/re-evaluation and a
content-free audit digest.

`NeuronSignalReceiptV2`, `PlasticityStateV1` and `TopologyProposalV1` remain
proposal-only until N1/S1. They bind sparse activation, inhibition,
homeostasis, eligibility trace, confidence/OOD, frozen backbone/adapter,
parent topology and bounded proposed changes; no in-place runtime mutation.

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
PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_2_SOURCE_ONLY
PASS_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRIES_AND_COMPATIBILITY
PASS_HEPTA_INTELLIGENCE_CURRENT_TRUTH_V1
PASS_HEPTA_INTELLIGENCE_A0_SOURCE_ONLY
PASS_HEPTA_INTELLIGENCE_A0_EXECUTABLE_QUALIFICATION
```

A0 exact-head receipt closes candidate qualification only. Independent review
uses `A0IndependentReviewReceiptV1`: reviewer identity/affiliation, exact
head/tree, artifact digests, findings/dispositions, conflict declaration,
APPROVE/REQUEST_CHANGES/ABSTAIN, signature and expiry. Implementer/publisher
cannot satisfy the reviewer role.

Merge attribution uses `RepositoryCheckAttributionReceiptV1` and classifies each
required check as PASS, FAIL_INTRODUCED_BY_CANDIDATE, FAIL_PRESENT_IN_BASE,
INFRASTRUCTURE_BLOCKED, CANCELLED_SUPERSEDED or NOT_REQUIRED_FOR_SELECTED_TARGET.
Only PASS or independently policy-approved target exclusion can yield merge
eligibility.

---

## 8. C0 — Durable learning episode ledger

Owned tables/records:

```text
learning migrations, run snapshots, episodes, events, candidate sets
outcomes, credit entries, lineage edges, unlearning watermarks, checkpoints
```

Tables are append-only; mutable pointers are separate CAS rows. Required gates:
fmt/check/test/Clippy, schema oracle, append/replay properties, duplicate/reorder/
foreign-owner rejection, outcome revision, credit conservation, unlearning
lineage and deterministic reopen. C0 remains default-off and grants no training
or product-write authority.

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

Only the observing store/adapter may produce MemoryWritten,
ProjectionPublished, OutboxDispatched or ReconcileApplied receipts. Caller-
asserted fact digests are rejected.

Failure matrix covers: before transaction; after binding; after memory before
journal (must be impossible in one transaction); after journal before commit;
after commit before response; dispatcher crash; ACK loss; stale lease/revision;
generation race; duplicate request; foreign owner. Exact retry adopts one
committed effect. E3 includes real process termination, not only injected
errors.

---

## 10. J0 — Journal lifecycle, recovery and capacity

Required design: active/history epochs; terminal checkpoint; chained hash/Merkle
root; incremental startup from latest verified checkpoint plus active tail;
create-only verified archive; background full scrub; bounded admission and
backpressure; corruption quarantine; backup/restore rehearsal; RTO/storage-
growth SLO.

Campaign: SIGKILL before/after commit, WAL/checkpoint interruption, disk full,
read-only/permission loss, contention/starvation, page/bit corruption,
backup/restore/archive replay, clock regression, duplicate delivery, owner/
tenant confusion and checkpoint substitution. Minimum evidence is x86_64 E3
plus independent ARM64/platform E3 with exact versions, non-empty steps,
artifacts, recovered roots and measured RTO/growth.

---

## 11. R1 — Grounded hybrid retrieval

```text
R1.0 side-stack selection receipt
R1.1 restack #40 deterministic planner
R1.2 restack #30 host evidence resolver
R1.3 split/rewrite #64 legacy inventory/quarantine
R1.4 rewrite #28 as product module
R1.5 split #34 provider/tokenizer/index contracts
R1.6 register real local semantic artifact
R1.7 reuse mechanics only from #45/#49/#54/#58
R1.8 shadow product integration and rollback
```

Every new head reruns source/Rust/product/security/repository gates. Old receipts
are provenance, not qualification. Hash-one-hot providers and synthetic seeds
qualify mechanics only.

---

## 12. N1, I1, L1 and C1 gates

### N1 — H5 adaptive signal unit

Sparse activation, lateral inhibition, homeostasis, bounded eligibility trace,
calibrated confidence, abstention/OOD, frozen backbone/adapter identity,
next-snapshot proposal, lesion/ablation and rollback. No biological replication
or neuromorphic claim.

### I1 — H6 calibrated fast policy

Every decision emits `PolicyDecisionReceiptV2`; high-risk actions abstain,
reject or use slow governed path. Held-out calibration and subgroup evidence are
mandatory.

### L1 — H7 causal evaluation and artifact pipeline

Required: support coverage, ESS, IPS, SNIPS, doubly robust, cluster/bootstrap
CI, candidate LCB, baseline UCB. Promotion requires:

```text
candidate LCB > baseline UCB
```

plus safety, subgroup, retention, privacy, resource, unlearning and rollback.

### C1 — Low-risk closed loop

First target is reversible read-only `MemoryRetrievalRank`. Require shadow,
canary bounds, kill switch, frozen baseline, delayed outcomes, credit
conservation, rollback rehearsal and independent operator approval. Tools,
providers, credentials, auth and topology mutation are excluded.

---

## 13. Evidence classes and external packages

```text
E0 source/static
E1 local executable
E2 independent runner/platform
E3 runtime/restart/failpoint
E4 efficacy/retention/resource/operator
```

`ReviewedCorpusEvidencePackageV1`: immutable corpus/digest, case/candidate/
locale manifest, two independent reviews per candidate, adjudication, reviewer
identity/affiliation/key/signature, license/provenance, privacy/PII/secret/
redaction approval, split/leakage proof, expiry/revocation.

`SemanticArtifactEvidencePackageV1`: model/tokenizer bytes/digests, license,
architecture/dimension/quantization, runtime compatibility, SBOM/vulnerability
scan, benchmark hardware, resource/energy receipt, signature/revocation/
rollback.

`OperatorAcceptancePackageV1`: exact integration candidate, runtime/efficacy/
rollback receipts, accepted scope/risk, canary/kill switch, observation window,
`CALLERS`, promotion/release decision and independent signature.

Missing real packages remain `BLOCKED_EXTERNAL_EVIDENCE`; tests may validate but
may not synthesize them.

---

## 14. Security, privacy, SLO and observability

Threats: prompt injection, memory poisoning, citation laundering, artifact
substitution, receipt replay, cross-agent confusion, embedding inversion,
membership inference, privilege escalation, malicious corpus and telemetry
leakage. Controls separate trust/grounding/truth; redact secrets/PII before
persistence; separate retrieval/training eligibility; sign/scope/expire/revoke
artifacts; enforce nonce domains, content-free telemetry, high-risk veto and
unlearning lineage.

Initial design budgets, not claims:

```text
local policy p95 <= 50 ms
retrieval p95 overhead <= 20% baseline or frozen absolute bound
journal rollover <= 8,000 active operations
incremental recovery <= 5 s / 10,000 active operations
critical safety regression = 0
```

Every package publishes p50/p95/p99, capacity, memory/disk, CPU/GPU/energy,
backpressure, quarantine/retry and recovery measurements before activation.

---

## 15. Promotion, handoff and closure order

Lifecycle:

```text
implemented → candidate_qualified → wired → runtime_qualified
→ efficacy_proven → operator_accepted → promoted
```

No author/CI/plan/model can self-grant a later state. Every promoted artifact
has previous-good pointer, migration proof, rollback command/rehearsal,
observation window and independent `CALLERS`/release decision.

Every work unit emits `PackageHandoffReceiptV1` with exact base/head/tree/parent,
changed paths/digests, commands/exits/toolchains, artifacts/receipts, closed/
open/external gaps, dependency revisions, authority map, rollback base and
resume predicate.

Repository-controlled order:

```text
A0 replacement/evidence/review/selection/merge attribution
→ B0 → C0 → M0 → J0 → R1 → N1 → I1 → L1 → C1
```

Stop on identity drift, >1 A0 commit above Q0, path escape, positive authority,
workflow source writeback, superseded/synthetic evidence, missing/expired/
digest-inconsistent artifact, unqualified dependency, ambiguous security/
privacy, or operator/production/promotion/release boundary.

Current claim boundary:

```text
system = L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS
H5 = N0_METAPHORICAL_TYPED_PROPOSAL
H6 = I0_DETERMINISTIC_SELECTIVE_POLICY
self_evolution = false
closed_loop_learning = false
structural_plasticity = false
neuromorphic_mechanism = false
local_small_model_used_by_h5 = false
local_small_model_used_by_h6 = false
runtime_wired = false
production_authority = false
promotion = false
release_authority = false
```
