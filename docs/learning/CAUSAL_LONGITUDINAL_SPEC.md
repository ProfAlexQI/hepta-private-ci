# Causal and longitudinal learning implementation specification

**Plan:** `HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN` v8.0.0  
**Specification:** `ALG-CAUSAL-LONGITUDINAL`  
**Bound modules:** `learning.ledger`, `learning.eval`, `learning.artifacts`, `prompt.registry`, `prompt.optimizer`, `context.compiler`  
**Documentation state:** `closed`  
**Implementation state:** not implied

## 1. Scope, ownership and non-claims

This specification defines the evidence path by which an immutable candidate may be judged better than a baseline across future time. `learning.ledger` owns causal episode, decision, outcome, credit and unlearning facts. `learning.eval` computes independent estimates. `learning.artifacts` stores immutable candidates and lineage. Prompt and context modules provide intervention identity and delivery observations but cannot label success.

Memory persistence, an offline-loss decrease, replay accuracy, prompt assignment, compilation, delivery or correlation with success is not long-term learning. A policy cannot observe and certify its own effect. Documentation closure does not advance `systemLearning`, prompt, intuition, operator or artifact claim levels.

## 2. Symbols, dimensions, units and normalization

| Symbol | Meaning | Constraint |
|---|---|---|
| `i` | decision unit | stable episode/boundary identity |
| `S_i` | pre-decision state snapshot | immutable digest-bound features |
| `C_i` | generator-relative complete candidate set | nonempty bounded list |
| `A_i` | chosen candidate | exactly one member or explicit abstain |
| `pi_b(A_i|S_i,C_i)` | logged behavior propensity | Q32 probability, `>0` for chosen action |
| `pi_e(a|S_i,C_i)` | evaluation policy probability | same candidate grammar |
| `Y_i` | independently observed outcome | typed vector with watermark |
| `W_i` | importance weight | bounded by profile |
| `m(S_i,a)` | outcome model | artifact-bound estimate |
| `T_i` | event time | UTC plus logical sequence |
| `G_i` | principal/subgroup key | privacy-approved bounded category |

Propensities use canonical fixed-point encoding and must sum to one within one quantization unit. Candidate order is canonical. Outcome units, direction, valid range, censoring semantics and terminality are registered per objective class. Missing outcome is not zero reward.

## 3. Formal model and invariants

For every adaptive decision, the ledger records the state snapshot, exact candidate generator and version, generator-relative complete candidate set, assignment distribution, chosen item, chosen propensity, randomization seed digest, compiled intervention/context, delivery observation, authorized action witness and independent outcome.

Completeness is relative to an explicit legal generator, not the universe of conceivable actions. A `CandidateSetCompletenessReceiptV1` binds generator code, grammar, hard filters, input state, enumeration/truncation policy, omitted-count bound and candidate digest. Truncation is permitted only before random assignment and must be deterministic for the snapshot.

The independent outcome observer is the owner of the effect or its trusted adapter. The evaluated policy cannot write `OutcomeReceiptV1`. Delayed outcomes use a watermark containing latest observable time, expected delay distribution, censoring reason and finalization state. Corrections append superseding receipts.

Off-policy estimators are

\[
W_i=\frac{\pi_e(A_i|S_i,C_i)}{\pi_b(A_i|S_i,C_i)},
\]

\[
\widehat V_{IPS}=\frac1n\sum_i W_iY_i,
\qquad
\widehat V_{SNIPS}=\frac{\sum_i W_iY_i}{\sum_i W_i},
\]

\[
\widehat V_{DR}=\frac1n\sum_i\left[
\sum_{a\in C_i}\pi_e(a|S_i,C_i)m(S_i,a)
+W_i(Y_i-m(S_i,A_i))\right].
\]

Effective sample size is `(sum W)^2/sum(W^2)`. Estimates are invalid when positivity/support gates fail. Weight clipping is declared before analysis and reported with unclipped diagnostics; it cannot be chosen after seeing the result.

Credit assignment conserves the bounded terminal outcome across decisions, prompt factors, models and tools. Every `CreditAssignmentReceiptV1` records allocations and residual. A policy that generated an action may propose explanatory features but cannot write conserved credit.

## 4. Deterministic reference algorithm

The reference evaluator operates on a canonical fixture without machine learning:

```text
freeze preregistered policy, outcome definition and analysis plan
validate candidate-set completeness and propensity normalization
exclude only rows named by preregistered integrity rules
finalize delayed-outcome watermark or mark row censored
compute support intersection and per-row importance weights
compute IPS, SNIPS and tabular direct-model DR
compute ESS, maximum weight and subgroup coverage
compute cluster/bootstrap confidence intervals with fixed seed digest
compare candidate lower confidence bound with baseline upper bound
apply every safety, privacy, resource and retention floor
emit immutable EvaluationReceiptV1 and never select the artifact
```

Golden vector `OPE-GV-001` contains four rows with behavior propensities `[0.5,0.25,0.5,0.25]`, evaluation propensities `[0.25,0.5,0.25,0.5]`, outcomes `[1,0,1,1]` and an all-zero tabular outcome model. The exact weights are `[0.5,2,0.5,2]`, `IPS=DR=0.75`, `SNIPS=0.6`, and `ESS=50/17`. Signed Q32 round-to-nearest/ties-to-even outputs are weights `[2147483648,8589934592,2147483648,8589934592]`, `IPS=DR=3221225472`, `SNIPS=2576980378`, and `ESS=12632256753`. Permuting row order must not change any result.

## 5. Trainable or estimated algorithm

Outcome models, credit models and policy candidates are trained only from immutable dataset snapshots. Cross-fitting is mandatory for DR: the model predicting row `i` is trained without the episode, principal and future window containing `i`. Hyperparameter search is nested inside training windows and cannot inspect the final future holdout.

Sequential monitoring uses preregistered boundaries and alpha spending or confidence-sequence rules. Multiple objective dimensions, factor combinations, timing arms and subgroups require declared multiplicity correction. Model version, tokenizer, prompt realization, tool schema and provider/runtime tuple are isolated or explicitly modeled.

Distribution shift is measured on state, candidate support, propensity, outcome delay, subgroup and resource features. Change points split evaluation windows rather than being averaged away. The simplest valid estimator is preferred; learned outcome models are rejected when calibration, support or cross-fit diagnostics are worse than the tabular/linear baseline.

## 6. Data, protocol and lineage schema

The durable episode chain is:

```text
RunStartSnapshotV1
CandidateSetCompletenessReceiptV1
LearningDecisionV1
PromptCandidateSetReceiptV1
PromptPricingReceiptV1
PromptPortfolioReceiptV1
PromptExerciseDecisionV1
ContextCompilationReceiptV1
PromptDeliveryObservationV1
VerifiedUseTokenWitnessV1
OutcomeReceiptV1
CreditAssignmentReceiptV1
DatasetSnapshotV1
EvaluationReceiptV1
LongitudinalEvaluationReceiptV1
LearningArtifactManifestV1
UnlearningComplianceReceiptV1
```

The following additions are canonical cross-module protocols registered in `docs/contracts/CONTRACTS.json` and `docs/contracts/PROTOCOL_SCHEMAS.json`:

```text
CandidateSetCompletenessReceiptV1 {
  set_id, state_digest, generator_id, generator_code_digest,
  grammar_digest, hard_filter_digest, truncation_digest,
  candidates_digest, candidate_count, omitted_count_bound,
  canonical_order_digest, decision
}

OutcomeWatermarkV1 {
  episode_id, observer_id, latest_observable_time,
  expected_delay_profile, terminality, censoring_reason,
  correction_predecessor, finalized_at
}

SupportAuditReceiptV1 {
  evaluation_policy_digest, behavior_policy_digests,
  support_intersection_digest, ESS_q32, max_weight_q32,
  clipped_and_unclipped_diagnostics, subgroup_coverage,
  decision
}
```

Ledger tables are append-only, keyed by stable IDs and semantic digests. Projection indexes are rebuildable. A deletion request marks source rows ineligible, traverses derived dataset/artifact lineage and requires a rebuilt successor or revocation. Backups are tested to ensure deleted rows and derived artifacts do not reappear.

## 7. Numerical stability, complexity and resource bounds

Probability arithmetic and published estimates use fixed-point or reproducible decimal accumulation with deterministic summation order. Denominators below the registered floor fail rather than overflow. Weight caps are objective-class configuration; pilot cap is `20`, while any unclipped maximum above `50` blocks promotion.

Evaluation is streaming `O(n*k)` where `k` is the bounded candidate count; no unbounded episode materialization is required. Pilot limits are candidate count `<=128`, episode events `<=4096`, evaluation batch `<=1,000,000` rows, encoded row `<=256 KiB`, and one confidence computation wall-clock budget declared in the package. Resource use and incomplete/censored counts accompany every estimate.

Confidence intervals use episode/principal clustering when repeated decisions are correlated. Pilot bootstrap uses at least `2,000` counter-based replicates; small-sample exact or conservative intervals replace asymptotic intervals when assumptions fail.

## 8. Failure detection, fallback and rollback

Evaluation is invalid for missing candidate set, chosen action outside the set, non-normalized or zero propensity, assignment-policy drift, observer conflict, unresolved correction, insufficient support, ESS breach, undeclared censoring, future leakage, dataset/artifact digest mismatch or evaluator/writer identity collision.

Fallback is deterministic baseline comparison or `insufficient_evidence`; it is never “assume no harm.” A candidate that cannot be evaluated remains proposed or shadow-only. Rollback selects the predecessor artifact, invalidates caches and confirms runtime reload. Delayed evidence that later reverses a decision triggers revocation and a new independent review.

## 9. Security, authority, privacy and unlearning

Evaluation modules have no production-write, selection, merge, promotion or release authority. The production writer may provide observations but cannot issue the independent decision. Raw secrets, credentials, unrestricted prompts and private payloads are excluded from general learning rows; approved features retain purpose and principal scope.

Subgroups are evaluated only when privacy and minimum-count rules permit. Small groups are aggregated or suppressed, never silently omitted from safety analysis. Unlearning covers raw ledger rows, projections, candidate caches, replay, datasets, checkpoints, prompt graphs, Bellman sensors when derived, artifacts, indexes and backup/restore. Completion requires `UnlearningComplianceReceiptV1` and non-resurrection tests.

## 10. Verification, golden vectors and property tests

Required tests cover exact OPE golden vectors, candidate-order permutation, probability quantization, zero-support rejection, extreme weights, ESS, clipped/unclipped reporting, cross-fitting leakage, delayed watermark, censoring, outcome correction, policy self-label rejection, credit conservation, subgroup suppression, future-time splits, change points, retention and deletion restore.

Property tests assert chosen membership, probability sum, deterministic estimates, DR equality to IPS under zero outcome-model contribution, SNIPS invariance to uniform weight scale, immutable preregistration and no evaluator authority. Fault tests kill the process between every append/index update and confirm idempotent recovery.

## 11. Quantitative acceptance gates

| Gate | Required threshold |
|---|---|
| Complete candidate receipt | `100%` evaluated decisions |
| Chosen propensity | `>0` and exactly logged |
| Propensity sum error | `<=1` Q32 unit |
| ESS | `>=400` and `>=10%` of eligible rows |
| Unclipped max weight | `<=50` |
| Missing finalized outcome | within preregistered censoring bound |
| Credit residual | `<=1` Q32 unit |
| Future-time windows | at least `2` |
| Independent snapshots | at least `3` |
| Candidate efficacy | candidate LCB `>` baseline UCB |
| Safety/subgroup floors | no registered breach |
| Old-task degradation | no worse than `2%` per protected slice |
| Rollback reload | `100%` exact predecessor |
| Deletion non-resurrection | `0` restored deleted/derived records |
| Self-issued evaluation/selection | `0` |

No average metric can compensate for a safety, privacy, support, retention or deletion failure.

## 12. Paper traceability and Hepta extensions

`PAPER-HOLDER-Q-2026` informs only the bounded operator candidate evaluated by this pipeline; it does not provide causal identification or longitudinal efficacy. Candidate completeness, logged propensity, independent outcomes, OPE, future-time validation, retention, rollback and unlearning are Hepta engineering requirements, not claims of that paper.

The NDU papers motivate recursive utility but do not prove that Hepta telemetry causally identifies preference change. This specification therefore keeps utility definition, outcome observation, policy assignment and evaluation in separate ownership lanes.

## 13. Implementation sequence and completion rule

Implementation order is protocol schemas → append-only episode/outcome store → deterministic completeness/propensity checks → golden OPE evaluator → delayed watermark and corrections → immutable datasets → cross-fit DR → subgroup/shift/future windows → artifact reload/rollback → retention and unlearning → independent longitudinal decision.

Documentation closure means this file and its registries pass exact source and synthetic merge gates. Source completion, causal closed-loop learning and longitudinal efficacy remain separate claims. This specification does not by itself advance `L0_STATIC`.
