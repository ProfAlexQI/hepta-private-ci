# HEPTA Neuron Model Evaluation Matrix v1.4

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
> This cross-domain pointer does not make this evaluation matrix an AuthBus canonical
> source or grant runtime, model/NPU, provider, effect, or promotion authority. The
> matrix remains an evaluation contract only; HNL federated work stays
> `NOT_READY_FAIL_CLOSED` until Gate-0.

**Contract state:** `PLANNING_ONLY_SHADOW`  
**Claim level:** `L0_BASELINE_L1_SHADOW_CONTRACT_ONLY`  
**Runtime authority:** `false`

## Candidate backends

| Function | Primary candidate | Fallback | Authority |
|---|---|---|---|
| Chinese salience/entity/novelty | BGE-small-zh shared encoder + task head | rules + TF-IDF/linear | advisory |
| English short-text intent | all-MiniLM-L6-v2 + task head | rules + deterministic classifier | advisory |
| Mixed-language retrieval | multilingual-e5-small (benchmark first) | lexical scorer | advisory |
| Pair reranking | language-validated cross-encoder | lexical scorer | advisory |
| Hard safety/precondition | deterministic rules | abstain/escalate | veto-capable |
| Explanation/proposal | small generative teacher | no generation | proposal-only |

Model size alone does not define a neuron. Each candidate is registered with
weight, tokenizer, compiled-operator and SBOM digests, license, privacy class,
device target and a task-specific head/calibrator.

## Required bake-off measurements

The same redacted, versioned, split-safe corpus MUST be used for every
candidate and deterministic baseline. Record task utility, coverage and
abstention, Brier/ECE calibration, false-veto/false-allow rates, p50/p95
latency, RSS, energy, cold-start, operator coverage and reproducibility.

Synthetic fixtures are labelled `NEGATIVE_MATH_FIXTURE_NOT_CALIBRATION_PASS` if
they violate a threshold (for example ECE 0.075 against a 0.05 limit). They
cannot support a model-efficacy claim.

## Hard gates

No model is eligible for a runtime snapshot without nested schema validation,
locked calibration, privacy/redaction checks, deterministic fallback,
no-regression against the baseline and a signed artifact lineage. Real claims
also require a device benchmark and a longitudinal corpus; Core ML conversion
does not imply Neural Engine execution.

## Claim boundary

This matrix is an evaluation contract, not a benchmark result. It does not
install a model, connect NPU, grant effect authority or establish long-term
learning.
