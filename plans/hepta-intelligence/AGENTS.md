# Hepta Intelligence Working Rules

This directory is governed fail-closed.

## Mandatory read order

```text
HEPTA_INTELLIGENCE_CURRENT_PLAN.json
→ HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json
→ HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json
→ HEPTA_INTELLIGENCE_MASTER_PLAN.md
```

`CURRENT_PLAN` is the sole aggregate machine authority. The master plan is the sole current human-readable plan. Registries are canonical registered inputs but have no production, operator, promotion, release, or CALLERS authority.

## Current phase

`Q0` has exact same-run E1/E2 paired executable evidence on candidate `c768bcbeb4c1168088d2499828c24da521a2a73a`. It is a qualified **source candidate**, not a wired or runtime-qualified product capability.

`A0` is the first active task. Until A0 is exact-head qualified, changes are limited to the master/current documents, registries, evidence summaries, verifiers and the A0 read-only workflow.

## Hard prohibitions

- no runtime source;
- no SQL migration;
- no product caller;
- no H5/H6/H7 runtime;
- no model/provider effect;
- no production authority;
- no CALLERS, operator, promotion or release change;
- no self-merge;
- no reuse of superseded-head checks;
- no interpretation of queued jobs, `steps=[]`, `runner_id=0`, source-only gates or PR prose as executable qualification.

## Compatibility

`HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json`,
`HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json` and registered tranche snapshots remain machine compatibility contracts until every registered consumer migrates through a receipt. They are not current truth and must not be replaced by redirects that break frozen consumers.

## Evidence discipline

- E0: source/static only.
- E1: local executable.
- E2: independent runner/platform.
- E3: restart/failpoint/runtime.
- E4: soak/retention/efficacy/energy/operator.

Each receipt binds exact repository, branch, head, tree, workflow, run, job, runner, steps, toolchain, gates, artifact and authority flags. Any mismatch must fail closed.

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
