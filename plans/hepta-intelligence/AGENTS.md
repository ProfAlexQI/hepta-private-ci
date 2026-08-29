# Hepta Intelligence Working Rules

This directory is governed **fail closed**.

## Mandatory read order

```text
HEPTA_INTELLIGENCE_CURRENT_PLAN.json
→ HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json
→ HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json
→ HEPTA_INTELLIGENCE_MASTER_PLAN.md
→ HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md
```

`CURRENT_PLAN` is the sole aggregate machine authority. The master plan is the sole current human-readable plan. The controlled execution specification is subordinate and has no current-plan, runtime, operator, production, promotion, release or `CALLERS` authority.

## V4.4 current phase

```text
phase = A0
work_unit = A0.3_REPLACE_BOT_HEAD_AND_OBTAIN_EXACT_HEAD_EXECUTABLE_EVIDENCE
allowed = documentation / registry / verifier / read-only workflow
runtime_wired = false
native_media_memory_wired = false
production_authority = false
```

Q0 exact base:

```text
head = c768bcbeb4c1168088d2499828c24da521a2a73a
tree = ca455a9ef797cd95164c880c7b8faba80b305589
```

Every A0 repair is a replacement, not an appended commit. Build the complete tree, create one commit whose sole parent is Q0, and atomically move the A0 branch. Every prior A0 head/run/artifact/review becomes stale.

## Source snapshot and live evidence

`SOURCE_SNAPSHOT` is deterministic checked-in contract truth. Its time is bound by the commit/executable receipt; source must not predeclare a future live observation. `LIVE_EVIDENCE` requires exact repository, branch, head, tree, parent, workflow, run, job, runner, non-empty completed steps, artifact identity/digest/expiry and all-negative authority.

Never treat queued, pending, cancelled, `steps=[]`, `runner_id=0`, source-only output, merge refs, stale artifacts, PR prose or superseded heads as qualification.

## Separation of duty

Implementer, source publisher, CI evidence workflow, independent reviewer, canonical selector, operator and CALLERS authority are distinct roles. Repository permission does not synthesize an independent review, selector decision, corpus, hardware, soak or operator fact. No self-review, self-approval or self-merge.

## Gap-loop discipline

Read both:

```text
HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json#/gap_closure_ledger
HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json#/multimodal_memory_gap_ledger
```

Allowed repository classifications:

```text
CLOSED_SOURCE_CONTROLLED
OPEN_SOURCE_CONTROLLED
BLOCKED_EXTERNAL_EVIDENCE
BLOCKED_UPSTREAM
STOP_CONDITION
```

Fixture success cannot close reviewer, selector, reviewed corpus, model/license approval, target hardware, soak, operator, CALLERS, promotion, release or production facts.

## Repository check attribution

`RepositoryCheckAttributionReceiptV1` classifications are exactly PASS, INTRODUCED_BY_CANDIDATE, PRE_EXISTING_ON_BASE, MERGE_INTERACTION, RUNNER_OR_PLATFORM_INFRA, CANCELLED_OR_SUPERSEDED, NOT_REQUIRED_BY_SELECTED_POLICY and UNKNOWN_FAIL_CLOSED. Unknown blocks merge.

## Multimodal package guardrails

MM0–MM6 remain plan-only during A0. Original media is immutable evidence in encrypted object storage, not a string/BLOB stuffed into semantic memory. OCR/ASR/caption are `DerivedArtifactV1` with `source_truth=false`. Every vector binds model/preprocessor/space/index generation. Every returned candidate is exactly revalidated. Deletion traverses raw asset, key/object, derivative, vector, index, cache, dataset, adapter, policy, evaluation and backup resurrection paths.

No runtime source, SQL migration, product caller, decoder, model import, embedding index, cross-modal retrieval, CALLERS, promotion or release change is allowed until A0 review/selection/merge admission.

## Package discipline after A0

Each B0/MM0+ package uses an exact qualified parent, isolated Draft PR, changed-path allowlist, smallest coherent slice, Cargo/Bazel parity, production implementation reused by qualification wrappers, commands/exits, artifact digests, `PackageHandoffReceiptV1`, rollback pointer and no self-merge.

## Compatibility

Legacy execution status V2/V3 and registered snapshots remain compatibility contracts until all frozen consumers migrate with paired evidence. They are not current truth.

## Claim discipline

```text
system=L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS
H5=N0_METAPHORICAL_TYPED_PROPOSAL
H6=I0_DETERMINISTIC_SELECTIVE_POLICY
multimodal_memory=MM0_SPECIFIED_ONLY
self_evolution=false
closed_loop_learning=false
structural_plasticity=false
neuromorphic_mechanism=false
native_media_memory_wired=false
cross_modal_retrieval_qualified=false
multimodal_efficacy_proven=false
```
