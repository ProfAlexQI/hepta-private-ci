# Hepta Intelligence Working Rules

This directory is governed fail closed.

## Mandatory canonical read order

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

`CURRENT_PLAN` is the sole aggregate machine authority.
`HEPTA_INTELLIGENCE_MASTER_PLAN.md` is the sole current human-readable plan.
The execution specification is the package-level Implementation Blueprint.
Registries and subordinate documents have no production, operator, promotion,
release, or `CALLERS` authority.

## Source snapshot versus live evidence

Checked-in status is a deterministic `SOURCE_SNAPSHOT`. It must never represent
a queued or later GitHub observation as PASS. `LIVE_EVIDENCE` is valid only
after a receipt binds exact repository, branch, head, tree, parent, workflow,
run, job, real runner, non-empty completed steps, artifact identity/digest/
expiry, and all-negative authority.

Live API observations do not directly mutate canonical source. A separately
authenticated publisher applies reviewed source; every new head requires fresh
exact-head evidence. Evidence from a superseded head is invalid.

## Current phase

`Q0` has same-run E1/E2 paired executable evidence on
`c768bcbeb4c1168088d2499828c24da521a2a73a`. It is a qualified source
candidate, not a wired or runtime-qualified product capability.

`A0` is active. The current work unit is:

```text
A0.3_REPLACE_BOT_HEAD_AND_OBTAIN_EXACT_HEAD_EXECUTABLE_EVIDENCE
```

Until A0 is exact-head qualified, independently reviewed, and canonically
selected, changes are limited to master/current documents, registries,
subordinate execution specification, verifiers, and read-only workflows.

## Exact-parent replacement protocol

The A0 candidate must remain exactly one direct child of the Q0 candidate. A
repair must not append a second A0 commit. Build a replacement tree, create a
new commit whose sole parent is `c768bcb...`, and atomically move the A0 branch.

Before every write, revalidate:

```text
repository
branch
current head and tree
Q0 parent and tree
changed-path allowlist
source snapshot
authority flags
```

A base/head/tree mismatch is a stop condition.

## Gap-loop discipline

Read:

```text
HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json#/gap_closure_ledger
```

Every gap requires a unique ID, classification, owner class, status,
dependencies, closure evidence, next action, authority effect, and resume
predicate.

Allowed terminal classifications:

- `CLOSED_SOURCE_CONTROLLED`
- `OPEN_SOURCE_CONTROLLED`
- `BLOCKED_EXTERNAL_EVIDENCE`
- `BLOCKED_UPSTREAM`
- `STOP_CONDITION`

A fixture can qualify mechanics. It cannot close a real corpus, reviewer,
hardware, soak, operator, `CALLERS`, promotion, or release fact.

## Package handoff

Every work unit emits a package handoff binding exact base/head/tree/parent,
changed paths, commands/exits, artifacts, receipts, gaps, authority, rollback,
and resume predicate. Resume only when all exact dependencies still match.

## Hard prohibitions during A0

- no runtime source;
- no SQL migration;
- no product caller;
- no H5/H6/H7 runtime;
- no model/provider effect;
- no production authority;
- no `CALLERS`, operator, promotion, or release change;
- no self-merge or self-approval;
- no candidate workflow source writeback;
- no reuse of superseded-head checks;
- no interpretation of queued jobs, `steps=[]`, `runner_id=0`, source-only gates,
  synthetic merge refs, or PR prose as executable qualification.

## Compatibility

`HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json`,
`HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json`, and registered tranche snapshots
remain machine compatibility contracts until every registered consumer
migrates through a receipt. They are not current truth and must not be replaced
by redirects that break frozen consumers.

## Evidence discipline

- E0: source/static.
- E1: local executable.
- E2: independent runner/platform.
- E3: runtime/restart/failpoint.
- E4: soak/retention/efficacy/energy/operator.

Each receipt binds exact identities and all authority flags. Any mismatch fails
closed.

## Claim discipline

Current claims remain:

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
