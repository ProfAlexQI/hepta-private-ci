# HEPTA Neuron Runtime Contract v1.4

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
> This cross-domain pointer does not make this Neuron contract an AuthBus canonical
> source or grant runtime, model/NPU, provider, effect, or promotion authority. The
> E.21/v1.4 interpretation below remains scoped to neuron planning; HNL federated work
> stays `NOT_READY_FAIL_CLOSED` until Gate-0.

**Contract state:** `PLANNING_ONLY_SHADOW`  
**Claim level:** `L0_BASELINE_L1_SHADOW_CONTRACT_ONLY`  
**Runtime authority:** `false`  
**Effective interpretation:** E.21 / v1.4; v1.3 text is historical background.

## Purpose

This contract defines the smallest independently testable unit of the Hepta
Neuron runtime. A neuron is a typed position contract plus a backend artifact,
feature builder, calibration, gate and receipt. It is not an execution actor,
memory authority, workflow owner or self-modifying process.

## Required NeuronSpec

Every registered position MUST declare:

- stable `neuron_id` and `position_id`;
- `trigger` event predicates and a bounded tick/depth budget;
- typed `input_ports` and `output_ports`, with schema digests;
- `state_policy` (`stateless`, `run_snapshot_only`, or bounded recurrent state);
- privacy class, capability class, resource budget and fallback profile;
- approved `graph_digest`, `model_digest`, `head_digest`, `calibration_digest`
  and compatibility version;
- threshold, hysteresis, cooldown and abstention policy;
- receipt schema and causal-parent requirements.

The four artifact digests are loaded from one immutable `RunStartSnapshot`.
The runtime MUST reject a mixed snapshot, stale fence, unknown schema or
missing causal parent before producing a usable signal.

## Runtime sequence

```text
TypedEvent + RunStartSnapshot
  -> FeatureBuilder
  -> backend selector (local artifact / deterministic guard / allowed fallback)
  -> calibration
  -> schema + evidence + privacy + budget + CAS/fence validator
  -> provisional NeuronSignal + ModelReceipt
```

`NeuronSignal` is advisory and provisional. It MUST carry input/output/model,
policy, graph, calibration, evidence, snapshot, causal-parent and fence
digests. It MUST NOT write Memory/CognitiveStore/KG, invoke a tool, dispatch an
effect or grant capability.

## Failure and fallback

Fallback order is position-specific and explicit. A failure, timeout, privacy
veto, unsupported operator or budget exhaustion yields a deterministic
fallback or `abstain`; it never silently upgrades authority. Remote execution
requires an allowlisted endpoint, redaction receipt and an explicit
`proposal_local` policy.

## Learning boundary

NDU may change only approved head/adapter, calibration, threshold or routing
artifacts offline. A new artifact is accepted only after replay/OPE,
no-regression, shadow, canary, signature and rollback checks; a running run
never changes its snapshot. Topology, permissions, invariants, base weights and
goals remain frozen under this contract.

## Verification status

The E.21 canonical contract lane verifies strict nested schemas, authority
scope, digest compatibility, receipt terminality, hash-chain reconstruction
and negative mutations. Passing that lane proves contract semantics only; it
does not prove a real model, Core ML/NPU execution, long-term efficacy or
production promotion.
