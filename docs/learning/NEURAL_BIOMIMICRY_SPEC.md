# Functional neural biomimicry implementation specification

**Plan:** `HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN` v8.0.0  
**Specification:** `ALG-NEURAL-BIOMIMICRY`  
**Bound modules:** `neuron.runtime`, `intuition.policy`, `learning.operator`, `learning.plasticity`  
**Documentation state:** `closed`  
**Implementation state:** not implied

## 1. Scope, ownership and non-claims

This specification defines a testable functional-biomimicry level for Hepta. `neuron.runtime` produces bounded temporal signals and checkpoints. `intuition.policy` consumes those signals for calibrated fast decisions. `learning.operator` supports replay and prediction-error candidates. `learning.plasticity` proposes next-snapshot parameter or topology changes.

The term “neuron” does not claim biological equivalence. NDU supplies preference and utility semantics, not cellular plasticity. A local language model, recurrent state or sparse activation alone is insufficient. Neuromorphic hardware, spiking dynamics, timing-dependent plasticity and energy claims remain a separate research level.

## 2. Symbols, dimensions, units and normalization

| Symbol | Meaning | Pilot bound |
|---|---|---:|
| `x_t` | approved encoded input | `d_x <=256`, normalized |
| `h_t` | bounded temporal state | `d_h <=256`, Q24 in `[-8,8]` |
| `z_t` | pre-competition activation | `d_z <=512` |
| `a_t` | sparse post-competition activation | top-k, `k/d_z in [1%,20%]` |
| `theta_t` | adaptive threshold | one per unit or group |
| `e_t` | eligibility trace | same shape as trainable local weights |
| `m_t` | low-dimensional neuromodulator | `d_m <=8`, each in `[-1,1]` |
| `delta_t` | prediction error | bounded Q24 |
| `W` | selected local adapter/head weights | immutable current-snapshot artifact |
| `Delta W` | proposed update | trust-region bounded, next snapshot only |
| `tau_h,tau_e,tau_theta` | decay constants | positive versioned scalars |

All dimensions, normalization, top-k policy, inhibitory graph, thresholds, decay constants and fixed-point scales are artifact fields. Runtime state is generation-bound. A process may not combine a checkpoint from one encoder or tokenizer with another generation.

## 3. Formal model and invariants

A pilot temporal cell is

\[
\tilde h_{t+1}=\rho_h h_t+F_W(x_t,h_t),\qquad
z_{t+1}=G_W(\tilde h_{t+1})-\lambda_I L a_t-\theta_t,
\]

\[
a_{t+1}=\operatorname{TopKPositive}(z_{t+1},k),
\qquad
h_{t+1}=\operatorname{clip}(\tilde h_{t+1},-H_{max},H_{max}).
\]

`L` is a registered nonnegative lateral-inhibition matrix with zero diagonal. Competition is deterministic for equal activations through canonical unit ordering. Threshold homeostasis is

\[
\theta_{t+1}=\operatorname{clip}
(\theta_t+\eta_\theta(\bar a_t-a_{target}),\theta_{min},\theta_{max}).
\]

The bounded eligibility trace and three-factor candidate update are

\[
e_{t+1}=\lambda_e e_t+\psi(pre_t,post_t),
\]

\[
q_t=B_m m_t,\qquad
\Delta W_t^{(g)}=\Pi_{\mathcal T_g}[\eta_w\,q_{t,g}\,e_t^{(g)}],
\qquad
W_{candidate}=W_{selected}+\sum_t\Delta W_t.

`B_m` is a manifest-bound map from the `d_m<=8` modulator vector to registered parameter groups `g`; every row has `L1` norm at most one. This removes ambiguous broadcasting between a low-dimensional modulator and a weight-shaped eligibility trace.
\]

`m_t` is derived from independently observed prediction error, utility residual and safety/resource modulators. It cannot contain credentials or authority. `Pi_T` is a trust region: per-layer relative norm, global norm, sign/monotonicity constraints and quantization limits are all enforced.

Runtime never mutates selected `W`. It accumulates an immutable proposal or sufficient statistics for a future artifact. Homeostatic state may evolve inside the current temporal checkpoint only within declared bounds; it cannot change model topology or hard constraints.

Replay consolidation uses immutable episodes sampled by a preregistered mixture of recency, surprise, underrepresented objective class and old-task retention. Replay cannot turn deleted or revoked rows back into eligible data. Prediction error is measured against a frozen world-model revision and an independent outcome.

## 4. Deterministic reference algorithm

```text
validate exact encoder, head, threshold, inhibition and checkpoint generation
encode approved input with frozen local encoder
update bounded temporal state in fixed-point arithmetic
subtract lateral inhibition and adaptive threshold
select deterministic top-k positive activations
update activation-rate moving average and bounded threshold
update eligibility trace from registered local pre/post rule
compute OOD, confidence and abstention signals
append checkpoint and signal receipts atomically
if a low-dimensional independent modulator exists:
  accumulate trust-region-bounded next-snapshot update proposal
never replace current model or topology
```

Golden vector `BIO-GV-001` uses two units, `z=[0.8,0.6]`, `k=1`, inhibition `L=[[0,1],[1,0]]`, prior activation `[0,1]`, `lambda_I=0.2`, thresholds `[0.1,0.1]`. The adjusted values are `[0.5,0.5]`, or `[8388608,8388608]` in signed Q24; canonical ordering activates unit `0`. With `a_target=0.5`, `eta_theta=0.1`, and per-unit activity `[1,0]`, the next thresholds are `[0.15,0.05]`, or `[2516582,838861]` in Q24. With `lambda_e=0.5`, prior eligibility `[1,-1]`, and zero local increment, the next eligibility is `[0.5,-0.5]`. The selected weight digest must remain bit-identical.

## 5. Trainable or estimated algorithm

The encoder is frozen during a runtime generation. Pilot trainable candidates are small adapters or heads, not unrestricted full-model retraining. Training separates:

- representation reconstruction or contrastive objective for approved local features;
- prediction-error head for expected next observation/outcome;
- calibration/OOD head;
- sparse competition and activation-load regularizers;
- temporal stability and state-recovery loss;
- eligibility-alignment loss comparing local proposals with a bounded offline gradient oracle;
- old-task retention and subgroup safety losses.

The loss is

\[
L=L_{task}+\lambda_pL_{prediction}+\lambda_cL_{calibration}
+\lambda_sL_{sparsity}+\lambda_hL_{homeostasis}
+\lambda_eL_{eligibility}+\lambda_rL_{retention}.
\]

A local three-factor rule is not assumed equivalent to backpropagation. Its cosine agreement, utility effect, stability and retention are measured. Candidates with high agreement but poor causal utility fail. Hyperparameters and replay mixture are immutable manifests and evaluated on future windows.

## 6. Data, protocol and lineage schema

The following records are canonical cross-module protocols registered in `docs/contracts/CONTRACTS.json` and `docs/contracts/PROTOCOL_SCHEMAS.json`:

```text
NeuronCheckpointV1 {
  checkpoint_id, predecessor, generation, encoder_digest,
  head_digest, temporal_state_digest, threshold_digest,
  activation_summary, eligibility_digest, logical_sequence,
  normalization_digest, expiry
}

NeuronSignalReceiptV1 {
  signal_set_id, checkpoint_before, checkpoint_after,
  model_runtime_digest, input_feature_digest,
  signals, activation_sparsity_ppm, inhibition_residual,
  prediction_error, confidence_ppm, ood_ppm, abstain
}

PlasticityProposalV1 {
  proposal_id, selected_artifact, dataset_digest,
  update_rule_digest, modulator_digest, eligibility_digest,
  parameter_delta_digest, trust_region_metrics,
  retention_metrics, evaluator_receipts, rollback_predecessor
}

TopologyProposalV1 {
  proposal_id, predecessor_topology, operation,
  typed_nodes_edges, compatibility_plan, resource_delta,
  security_review, lesion_plan, rollback_plan
}
```

Checkpoints are append-only and generation-specific. A compact checkpoint may summarize eligibility but must preserve replay-equivalent recovery within tolerance. Artifact lineage includes encoder, tokenizer, preprocessor, quantization, license/SBOM, device/runtime, dataset, training code and real consumer evidence.

## 7. Numerical stability, complexity and resource bounds

Pilot runtime cost is bounded by `O(d_h*k_f + |E_I|)` for sparse fan-in `k_f` and inhibitory edges `E_I`. No dense `d_h^2` path is allowed above `d_h=256` without qualification. p95 signal latency is `<=3 ms`, p99 `<=8 ms`, transient allocation `<=512 KiB`, active checkpoint `<=1 MiB`, and checkpoint write amplification `<=4x` logical bytes.

State norm, activation rate, threshold, eligibility norm, modulator norm and update norm are hard bounded. Pilot values are `H_max=8`, eligibility norm `<=4`, modulator absolute value `<=1`, per-layer relative parameter delta `<=0.5%`, global relative delta `<=0.25%` and at most one proposal per artifact/window.

Replay batches are bounded by rows, bytes, objective classes and age. Consolidation has a declared resource budget and cannot starve foreground operation. A missed replay window is observable degradation, not permission for an unbounded catch-up queue.

## 8. Failure detection, fallback and rollback

Failures include model/tokenizer/preprocessor mismatch, invalid checkpoint generation, state explosion/collapse, activation collapse, all-unit activation, threshold saturation, eligibility overflow, modulator provenance failure, replay lineage violation, OOD false acceptance, trust-region breach and attempted current-run weight/topology mutation.

Fallback order is valid temporal candidate → stateless selected head → deterministic calibrated rule → slow-path request. A corrupt or incompatible checkpoint is quarantined; the process may reconstruct from the last valid checkpoint and ordered events. Rollback selects the exact predecessor artifact and topology snapshot and verifies checkpoint compatibility. No partial layer mix is allowed.

## 9. Security, authority, privacy and unlearning

Neural state carries no authority and never stores raw credentials, secret values, unrestricted prompt text or external instructions. Local-model output is advisory. `intuition.policy` cannot bypass a hard veto, and `learning.plasticity` cannot select or install its proposal.

Feature admission is purpose- and principal-scoped. Checkpoint inspection exposes bounded summaries, not raw private content. Unlearning traverses inputs, feature caches, checkpoints, replay eligibility, world-model rows, adapters, proposals, artifacts and backups. A checkpoint depending on deleted content is revoked or rebuilt before reuse.

## 10. Verification, golden vectors and property tests

Required tests cover deterministic top-k ties, lateral inhibition, target activation rate, threshold recovery, eligibility decay, zero modulator, positive/negative modulator, trust-region clipping, state checkpoint/reopen, encoder mismatch, OOD/abstention, replay scheduling, deleted-row exclusion and predecessor rollback.

Ablation families include full mechanism, no inhibition, no homeostasis, no eligibility, no replay, shuffled modulator and frozen temporal state. Lesion tests remove registered units or edges and measure utility, stability, calibration, forgetting and resource change. Property tests enforce bounded state, bounded sparsity, deterministic tie-breaking, zero current-artifact mutation and replay lineage closure.

## 11. Quantitative acceptance gates

| Gate | Required threshold |
|---|---|
| Temporal recovery parity | max component error `<=2` Q24 units |
| Activation sparsity | within registered target ±`2 percentage points` |
| Dead-unit fraction | `<5%` after warm-up |
| Always-active fraction | `<1%` |
| Threshold saturation | `<0.1%` updates |
| State/eligibility overflow | `0` |
| OOD false acceptance | `<0.5%` |
| Calibration ECE | `<=0.03` |
| Three-factor/oracle cosine | lower 95% bound `>0` on supported slice |
| Parameter proposal | within every trust-region bound |
| Current-run parameter/topology mutation | `0` |
| Full mechanism versus each ablation | preregistered utility/stability benefit or claim withheld |
| Old-task degradation | no worse than `2%` per slice |
| Rollback/reopen | `100%` mandatory fault suite |
| Deleted-row replay | `0` |

Functional biomimicry requires the full ablation and longitudinal evidence set; passing runtime unit tests is insufficient.

## 12. Paper traceability and Hepta extensions

`PAPER-NDU-EU-2025` informs bounded temporal preference/utility semantics, not the local neural update. `PAPER-HOLDER-Q-2026` informs the slow value/operator candidate, not biological plasticity. Sparse competition, lateral inhibition, eligibility, homeostasis, neuromodulation, replay consolidation, lesion/ablation and trust-region next-snapshot updates are Hepta hypotheses requiring direct evidence.

No cited paper is used to claim spiking neurons, synaptic biological identity, consciousness, neuromorphic energy efficiency or safe autonomous evolution.

## 13. Implementation sequence and completion rule

Implementation order is exact local-model manifest → deterministic temporal cell and checkpoint → sparse competition/inhibition → calibration and OOD → eligibility/homeostasis → independent modulator → bounded proposal artifact → replay consolidation → prediction-error world model → ablation/lesion qualification → future-time retention → optional topology proposal and canary.

Documentation closure means the mechanism, state, tests and thresholds are specified and machine-gated. Source implementation, `N1` real consumer evidence, `N2` temporal recovery and `N3` functional-biomimicry evidence remain separate. This file does not by itself advance `N0_METAPHORICAL`.
