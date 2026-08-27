# HEPTA Longitudinal Evaluation Protocol v1.4

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
> This cross-domain pointer does not make this evaluation protocol an AuthBus canonical
> source or grant runtime, model/NPU, provider, effect, or promotion authority. The
> contract state and claim boundary below remain in force; HNL federated work remains
> `NOT_READY_FAIL_CLOSED` until Gate-0.

**Contract state:** `PLANNING_ONLY_SHADOW`  
**Claim level:** `L0_BASELINE_L1_SHADOW_CONTRACT_ONLY`  
**Runtime authority:** `false`

## Dataset and split

Use a redacted, versioned corpus spanning weeks to months. Split by episode,
user/time boundary and task family; freeze a held-out no-regression set. Record
redaction, consent, deletion/forget-revocation and poisoning-audit receipts.

## Required measures

Report action utility and practical gain (not only non-inferiority),
support/propensity coverage, OPE confidence intervals, delayed-feedback
missingness, retention/forgetting, memory pollution, calibration (Brier/ECE),
drift/change points, latency/RSS/energy and rollback recovery. Pre-register
power analysis, cluster bootstrap, multiple-comparison correction and an
explicit minimum practical effect.

## Promotion gates

An artifact must beat or be practically non-inferior to the deterministic
baseline on the locked set, stay within safety and resource budgets, and show
no material retention/forgetting regression. Evaluation proceeds replay/OPE →
shadow → single-agent canary → independent operator acceptance → signed
promotion → rollback rehearsal. Missing support or feedback is a blocked
claim, not a zero reward.

## Claim boundary

This is a protocol and analysis contract. Current Hepta evidence remains
`L0_BASELINE_L1_SHADOW`; no model efficacy, long-term learning, self-evolution
or production authority is claimed until the gates above have real data.
