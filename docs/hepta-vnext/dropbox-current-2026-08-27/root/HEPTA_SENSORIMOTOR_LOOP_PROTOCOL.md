# HEPTA Sensorimotor Loop Protocol v1.4

> **Current development pointer (E.45 document-sync successor / 2026-08-27)**
>
> ```yaml
> current_profile: DEVELOPMENT
> implementation_status: IMPLEMENTATION_BACKLOG_ONLY
> development_blockers: implementation_backlog_only
> plan_pointer: hepta-vnext-development-plan-final-2026-08-23.md#development-docs-sync-e45
> historical_e44_pointer: hepta-vnext-development-plan-final-2026-08-23.md#authbus11-artifact-closure-v13
> current_plan_pointer: hepta-vnext-development-plan-final-2026-08-23.md#development-docs-sync-e45
> current_binding_manifest: HEPTA_DEVELOPMENT_DOCS_CURRENT_BINDING_V1.json
> current_sync_receipt: HEPTA-DEVELOPMENT-DOCS-SYNC-RECEIPT-2026-08-27.json
> qualification_pointer: HEPTA_VNEXT_QUALIFICATION_INDEX.md#authbus11-execution-closure-v13
> authbus_registry: AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry
> authbus_stage_selector: AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map
> behavioral_implementation_evidence: NOT_RUN
> runtime_authority: false
> production_effect_authority: false
> ```
>
> This cross-domain pointer does not make this sensorimotor protocol an AuthBus
> canonical source or grant runtime, model/NPU, provider, effect, or promotion authority.
> The S3a sandbox boundary below remains in force; HNL federated work stays
> `NOT_READY_FAIL_CLOSED` until Gate-0.

**Contract state:** `S3a_SANDBOX_ONLY`  
**Claim level:** `L0_BASELINE_L1_SHADOW_ONLY`  
**Runtime authority:** `false`  
**Production execution:** `false`

## Loop

```text
TypedEvent/StateSnapshot
 -> NeuronSignal + ModelReceipt
 -> Intuition advisory DecisionReceipt
 -> authorized Activity/Effect intent
 -> Activity/EffectReceipt + Postcondition
 -> user/system feedback
 -> immutable causal trajectory
 -> offline NDU replay/OPE
 -> next approved snapshot
```

S3a stops at a read-only ActivityReceipt/observation trajectory. It cannot
call a model, network, tool, external effect, Memory/KG writer or production
TaskFlow. S3b is the later sandbox action seam and S4 is real Agent-local
execution; neither is implied by this document.

## Required causal fields

Every transition binds `run_id`, `episode_id`, `causal_parent`, sequence/hash
chain, snapshot/graph/model/policy digests, propensity/support (when a choice
is made), utility/postcondition status, latency/cost and fence. Missing or
stale evidence yields `abstain` or a blocked receipt.

`EffectReceipt` is non-terminal while `indeterminate`; reconciliation requires
external evidence and an explicit reconciled/rejected/revoked outcome. An
advisory DecisionReceipt can never set production `execute_allowed=true`.

## Learning interpretation

Only a trajectory containing action outcome, postcondition and feedback can
support a closed-loop policy-learning claim. S3a proves typed causal shape and
fault handling only; it is not a long-term efficacy or biomimetic result.
