# HEPTA Topology Proposal Contract v1.4

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
> This cross-domain pointer does not make this topology proposal an AuthBus canonical
> source or grant runtime, model/NPU, provider, effect, or promotion authority. The
> L3/S6 proposal-only boundary below remains in force; HNL federated work stays
> `NOT_READY_FAIL_CLOSED` until Gate-0.

**Contract state:** `PROPOSAL_ONLY_PLANNING`  
**Claim level:** `L0_BASELINE_L1_SHADOW_ONLY`  
**Runtime authority:** `false`  
**Effective interpretation:** L3/S6 only; no online graph mutation.

## Proposal shape

Each proposal MUST include `proposal_id`, operation (`add`, `split`, `merge`,
`retire` or `rewire`), `parent_graph_digest`, `graph_version`, complete
NodeSpec/EdgeSpec schemas, causal lineage, privacy/capability classes, resource
budget, expected utility, rollback graph and proposer receipt. Edges carry input
and output schema digests, fan-out/depth/cycle budgets and support metadata.

## Compiler and evaluation gates

The compiler rejects dangling edges, schema mismatch, unbounded cycles,
capability escalation, privacy downgrade, quota/churn overflow and missing
rollback. A proposal then passes replay/OPE, ablation/lesion, no-regression,
shadow and single-agent canary before independent operator acceptance and
signed registry promotion.

## Activation rule

An approved graph is activated atomically by registry CAS for a **future**
runtime snapshot. Existing runs retain their graph digest. If NDU, registry,
compiler or governance is unavailable, the previous deterministic graph remains
active.

## Prohibited changes

Online code rewriting, permission/capability changes, invariant removal, base
model replacement, goal changes and effect authority changes are prohibited.
This contract is not evidence of biological synaptic growth or neuromorphic
plasticity.
