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

`CURRENT_PLAN` is the sole aggregate machine authority. `HEPTA_INTELLIGENCE_MASTER_PLAN.md` is the sole current human-readable plan. The controlled execution specification is subordinate and has no current-plan, operator, promotion, production, release, or `CALLERS` authority.

## V4.3 current phase

```text
phase = A0
work_unit = A0.3_REPLACE_BOT_HEAD_AND_OBTAIN_EXACT_HEAD_EXECUTABLE_EVIDENCE
allowed = documentation / registry / verifier / read-only workflow
runtime_wired = false
production_authority = false
```

Q0 exact base:

```text
head = c768bcbeb4c1168088d2499828c24da521a2a73a
tree = ca455a9ef797cd95164c880c7b8faba80b305589
```

Every A0 repair is a **replacement**, not an appended second A0 commit. Build the desired tree, create one commit whose sole parent is the Q0 head, and move the A0 branch. A replacement invalidates every prior A0 head/run/artifact/review receipt.

## Source snapshot and live evidence

`SOURCE_SNAPSHOT` is checked-in deterministic truth. `LIVE_EVIDENCE` is valid only through a receipt binding repository, branch, head, tree, parent, workflow, run, job, runner, non-empty completed steps, artifact identity/digest/expiry and all-negative authority.

Never treat queued, pending, cancelled, `steps=[]`, `runner_id=0`, source-only output, synthetic merge refs, stale artifacts, PR prose, or a superseded head as executable qualification.

## Separation of duty

- implementer may author allowed source;
- source publisher may publish the exact replacement;
- CI may only read source and emit evidence;
- independent reviewer must be distinct from implementer/publisher;
- canonical selector chooses the one integration candidate;
- operator accepts runtime/rollback risk;
- a separate `CALLERS` decision controls activation.

One actor must not impersonate the next role. User-granted repository permission does not synthesize reviewer, selector, operator or external evidence.

## Gap-loop discipline

Read `HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json#/gap_closure_ledger`.

Every gap requires:

```text
gap_id
classification
owner_class
status
dependencies
closure_evidence
next_action
authority_effect
resume_predicate
```

Allowed terminal classifications:

```text
CLOSED_SOURCE_CONTROLLED
OPEN_SOURCE_CONTROLLED
BLOCKED_EXTERNAL_EVIDENCE
BLOCKED_UPSTREAM
STOP_CONDITION
```

A fixture may qualify mechanics. It may not close a real corpus, reviewer, hardware, soak, operator, `CALLERS`, promotion, release or production fact.

## Repository check attribution

A selected synthetic merge candidate must produce `RepositoryCheckAttributionReceiptV1`. Every required check is classified as PASS, candidate-introduced, base-pre-existing, merge-interaction, runner/platform infrastructure, cancelled/superseded, policy-excluded, or unknown fail-closed. `UNKNOWN_FAIL_CLOSED` blocks merge.

## Package discipline after A0

B0 and later packages require:

- exact qualified parent;
- isolated branch and Draft PR;
- changed-path allowlist;
- smallest coherent slice;
- Cargo/Bazel parity;
- production implementation reused by qualification wrappers;
- commands/exits and artifact digests;
- `PackageHandoffReceiptV1`;
- rollback pointer;
- no self-merge.

Target boundaries:

```text
hepta-intelligence-contracts
hepta-grounding
hepta-mutation-core
hepta-mutation-journal
hepta-mutation-coordinator
hepta-retrieval
hepta-policy-runtime
hepta-learning-ledger
hepta-intelligence-eval
```

## Hard prohibitions during A0

- no runtime source;
- no SQL migration;
- no product caller;
- no H5/H6/H7 runtime;
- no model/provider effect;
- no production authority;
- no `CALLERS`, operator, promotion or release change;
- no self-review, self-approval or self-merge;
- no candidate workflow source writeback;
- no stale evidence reuse;
- no side-stack implicit selection.

## Compatibility

`HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json`, `HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json` and registered status snapshots remain compatibility contracts until every frozen consumer migrates through a paired receipt. They are not current truth and must not be replaced by schema-breaking redirects.

## Evidence classes

```text
E0 source/static
E1 local executable
E2 independent runner/platform
E3 runtime/restart/failpoint
E4 soak/retention/efficacy/energy/operator
```

## Claim discipline

```text
system=L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS
H5=N0_METAPHORICAL_TYPED_PROPOSAL
H6=I0_DETERMINISTIC_SELECTIVE_POLICY
self_evolution=false
closed_loop_learning=false
structural_plasticity=false
neuromorphic_mechanism=false
local_small_model_used_by_h5=false
local_small_model_used_by_h6=false
```
