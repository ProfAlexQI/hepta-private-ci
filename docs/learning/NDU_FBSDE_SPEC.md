# NDU forward-backward implementation specification

**Plan:** `HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN` v8.0.0  
**Specification:** `ALG-NDU-FBSDE`  
**Bound modules:** `objective.compiler`, `utility.ndu`, `intelligence.control`  
**Documentation state:** `closed`  
**Implementation state:** not implied

## 1. Scope, ownership and non-claims

This specification closes the implementation-design gap for Hepta Neural Differential Utility. It defines the formal state, deterministic reference path, stochastic candidate path, persistence model, numerical bounds, failure policy and qualification thresholds. `utility.ndu` owns preference and recursive-utility projections; `objective.compiler` owns immutable goals, legal action classes and hard constraints; `intelligence.control` may compose snapshots but owns neither state nor artifacts.

NDU is not a generic forward/backward software label, an individual Hepta neuron, a reward table, a database state machine or an authority source. A completed document does not establish dynamic-preference efficacy, longitudinal improvement, biological fidelity or production activation. Those claims remain at the levels recorded in `docs/evidence/CLAIMS.json` until exact evidence exists.

## 2. Symbols, dimensions, units and normalization

| Symbol | Meaning | Pilot bound | Durable representation |
|---|---|---:|---|
| `s` | subject identity | system/domain/agent/episode only | scoped `id128` |
| `k` | event boundary | monotone `u64` | integer |
| `dt_k` | logical step duration | `[1 ms, 3600 s]` | microseconds |
| `P_k` | latent preference state | `d_p <= 64` | Q32 vector in `[-1,1]` |
| `O_k` | registered observations | `d_o <= 256` | normalized Q24 vector |
| `C_k` | resource consumption | `d_c <= 32` | nonnegative Q32 vector |
| `R_k` | bounded risk state | `d_r <= 32` | Q32 vector in `[0,1]` |
| `A_k` | legal action descriptor | `d_a <= 128` | bounded typed encoding |
| `M^P,M^U` | martingale drivers | each dimension `<=32` | sampled increment receipt |
| `U_k` | recursive utility | scalar or `d_u <= 8` | Q32 |
| `Z_k` | utility noise sensitivity | `d_u x d_m` | bounded Q24 matrix |
| `B_k` | hierarchy boundary condition | one parent/child edge | typed receipt |

Normalization statistics, scale exponents, dimensions, units, clipping policy and feature order are immutable artifact fields bound into `RunStartSnapshotV1`. Missing statistics, dimension drift, NaN, infinity or unit mismatch is a hard validation failure. Soft clipping is permitted only for fields explicitly marked `soft_clip_allowed`, and the pre-clip violation must be recorded.

## 3. Formal model and invariants

For each resource-constrained subject, the analysis model is the forward-backward stochastic system

\[
dP_t=b_\theta(t,P_t,O_t,C_t,R_t,A_t)dt+\sigma_\theta(t,P_t,O_t,C_t,R_t,A_t)dM_t^P,
\]

\[
dU_t=-f_\phi(t,P_t,O_t,C_t,R_t,A_t,U_t,Z_t)dt+Z_t dM_t^U,
\qquad U_T=G(P_T,Y_T,J).
\]

`J` is the immutable objective snapshot and `Y_T` is an independently observed terminal outcome. The two martingale components are logically distinct even when sampled from one registered generator. Their covariance declaration is explicit and digest-bound.

Every adaptive coefficient artifact requires `NduWellPosednessCertificateV1`. On its declared compact operating domain it must record boundedness and Lipschitz constants for `b`, `sigma`, `f` and `G`; square-integrability and conditional-zero-mean diagnostics for the martingale increments; monotonicity or dissipativity of `f` in utility; linear-growth bounds in `Z`; solver range; and an independent decision. Failure to establish well-posedness selects the deterministic baseline. It never relaxes authority, privacy, truth, deletion or writer ownership.

The engineering discretization is

\[
P_{k+1}=\Pi_P[P_k+b_\theta(k,\cdot)dt_k+\sigma_\theta(k,\cdot)\Delta M_{k+1}^P],
\]

\[
Z_k=\operatorname{Regress}_{\mathcal F_k}
\left[U_{k+1}\Delta M_{k+1}^{U\top}/\max(dt_k,dt_{min})\right],
\]

\[
U_k=\Pi_U\,E_k[U_{k+1}+f_\phi(k,P_k,\ldots,U_{k+1},Z_k)dt_k].
\]

Hard objective fields are inputs, never learned coordinates in `P`. Legal actions come only from `LegalActionCandidateSetV1`. Preference adaptation may change bounded resource allocation, evidence effort, exploration, abstention or scheduling weights; it may not substitute a different goal.

Hierarchy is a Hepta extension. Parent and child exchange only `NduBoundaryConditionV1`: budget, shadow price, continuation utility, uncertainty, residual and expiry. The child is evaluated against a frozen parent revision and accepted with damped update

\[
P_{k+1}^{accepted}=(1-\eta)P_k+\eta\widehat P_{k+1},\qquad \eta\in[1/16,1/4].
\]

Parent and child parameter artifacts may not be selected in the same generation. Resource and risk conservation residuals are bounded, and the upper 95% confidence bound of the measured coupling spectral radius must be below `0.95` before activation.

## 4. Deterministic reference algorithm

The mandatory deterministic reference sets `sigma=0`, both martingale increments to zero and uses canonical fixed-point arithmetic.

```text
validate objective, subject, feature and legal-candidate revisions
P[0] = predecessor preference state
for k in ordered events:
    drift = bounded_preference_drift(P[k], event[k], objective)
    raw = P[k] + dt[k] * drift
    P[k+1] = registered_projection(raw)
U[n] = bounded_terminal_utility(P[n], independent_outcome, objective)
for k from n-1 down to 0:
    instant = bounded_instant_utility(P[k], event[k], objective)
    U[k] = registered_projection(instant + discount[k] * U[k+1])
emit immutable update receipts, projection counters and residuals
```

The idempotency key is `(subject_id, predecessor_revision, event_digest, coefficient_digest)`. Reuse with different semantics is a conflict. A crash before commit leaves the predecessor active. A crash after commit but before acknowledgement is reconciled by exact digest. The deterministic artifact remains the production fallback until an adaptive artifact independently passes all later gates.

Golden vector `NDU-GV-001` uses `P0=0`, two unit steps, drifts `0.25` then `-0.5`, instant utilities `0.5` and `0.25`, discount `0.8`, and terminal utility `1.0`. Expected values are `P=[0,0.25,-0.25]` and `U=[1.34,1.05,1.0]`. With signed Q32 round-to-nearest/ties-to-even, the canonical integers are `P=[0,1073741824,-1073741824]` and `U=[5755256177,4509715661,4294967296]`; every conforming reference must reproduce them exactly.

## 5. Trainable or estimated algorithm

The first adaptive candidate separates estimation from decision:

- `PreferenceFilter` estimates latent `P_k` and calibrated uncertainty from the prior state and approved observations.
- `PreferenceDriftNet` produces bounded drift through a final `tanh` and norm-capped residual blocks.
- `PreferenceDiffusionNet` produces a lower-triangular factor with positive diagonal and a registered spectral cap.
- `UtilityGeneratorNet` produces a bounded generator and uses a monotone parameterization when the certificate requires it.
- `ConditionalZHead` estimates the backward sensitivity.

Training uses immutable `DatasetSnapshotV1` rows grouped by episode, objective class and principal scope. Splits are future-time splits; rows from one episode never cross train and validation. The loss is

\[
L=w_P L_{filter}+w_U L_{BSDE}+w_Z L_Z+w_C L_{conservation}
+w_S L_{stability}+w_{cal}L_{calibration}.
\]

Pilot defaults are AdamW, learning rate `3e-4`, weight decay `1e-4`, gradient-norm cap `1.0`, at most `200` epochs and early stop after `12` validation evaluations. Every manifest records exact optimizer, precision, device, software tuple, batch construction and counter-based seed digest. A candidate is rejected when claimed subject classes lack support, identifiability probes fail, uncertainty falls below empirical residuals, or in-sample gain is accompanied by future-time or retention loss.

## 6. Data, protocol and lineage schema

The following bounded records are canonical cross-module protocols registered in `docs/contracts/CONTRACTS.json` and `docs/contracts/PROTOCOL_SCHEMAS.json`; this Markdown file cannot mint or redefine them:

```text
NduCoefficientManifestV1 {
  artifact_id, subject_class, objective_class_digest,
  dimensions, fixed_point_scales, coefficient_bounds,
  lipschitz_bounds, monotonicity_bounds, normalization_digest,
  runtime_tuple_digest, predecessor, expiry, rollback_digest
}

NduWellPosednessCertificateV1 {
  manifest_digest, operating_domain_digest,
  square_integrability, conditional_mean,
  coefficient_bounds, lipschitz, generator_monotonicity,
  terminal_lipschitz, continuity_scope, solver_stability,
  evaluator_identity, decision
}

NduUpdateReceiptV1 {
  subject_id, predecessor_revision, next_revision,
  event_digest, objective_digest, coefficient_digest,
  dt_micros, before_digest, after_digest, utility_digest,
  uncertainty_ppm, projection_count, boundary_residual_ppm,
  conservation_residual_q32, disposition
}
```

Preference and utility rows are append-only by revision. A transactional pointer identifies the selected projection after the immutable row exists. Correction or deletion appends revocation edges and rebuilt successors. Lineage closes source event → feature generation → dataset → training code → coefficient artifact → evaluation → selected snapshot → runtime receipt. Raw prompts, credentials, secret values, unrestricted payloads and authority tokens never enter this chain.

## 7. Numerical stability, complexity and resource bounds

Dense pilot runtime is bounded by `O(d_p^2+d_p*d_o)` per update; sparse heads are bounded by registered fan-in. The hot path may not allocate proportional to episode history. Pilot envelopes on the qualified reference host are p95 `<=2 ms`, p99 `<=5 ms`, transient allocation `<=256 KiB`, persistent state `<=256 KiB` per active subject, hierarchy depth `<=4`, zero central synchronous RPC and no more than one artifact lookup per process generation.

Training truncates episodes at `512` boundaries and uses gradient checkpointing above `128`. Batches are bounded by row count and encoded bytes. The numerical report contains maximum state and generator norms, empirical Lipschitz estimates, martingale conditional means, tail integrability, projection rate, conservation residual, coupling spectral radius, calibration and zero-noise parity.

## 8. Failure detection, fallback and rollback

Hard failures include unknown subject class, stale objective, missing candidate-set digest, invalid dimensions, unit drift, non-finite values, expired or revoked artifact, certificate mismatch, conservation violation, stale parent boundary or learned hard-axis mutation. Optional observations may degrade only through an explicit input mask and increased uncertainty; degradation may not widen authority or risk.

Fallback order is adaptive selected artifact → selected deterministic artifact → last valid deterministic snapshot for the objective class → immutable objective baseline → abstain and request slow path. Mixed generations are forbidden. Rollback selects the declared predecessor and rebuilds projections from the append-only ledger. State explosion, state collapse, uncertainty collapse, repeated projection, monotonicity failure or coupling instability quarantines the candidate without widening bounds.

## 9. Security, authority, privacy and unlearning

`utility.ndu` has no effect, model-dispatch, tool, network, external filesystem, secret, Matrix, fleet, selection, merge, promotion or release authority. The artifact reader cannot select the artifact it consumes. Subject identifiers are scope-pseudonymous; cross-domain aggregation uses bounded summaries. NDU state must not be presented as a psychological diagnosis or an immutable fact about a person.

Negative tests inject credentials, instructions, authority material and out-of-scope memory into preference features; all must be rejected before persistence. Unlearning traverses feature caches, projections, datasets, optimizer checkpoints, coefficient artifacts, evaluations, manifests and backup/restore paths. Only a non-reversible tombstone digest may remain to prevent resurrection.

## 10. Verification, golden vectors and property tests

Required tests cover the analytic zero-noise golden vector, boundary projection, independent and correlated martingale fixtures, generator monotonicity counterexamples, terminal-revision mismatch, parent-child conservation, frozen-parent versus simultaneous-update oscillation, crash/reopen, idempotency, correction/unlearning rebuild, zero-noise adaptive/reference parity and fixed-point limits.

Property tests enforce boundedness, deterministic replay, revision monotonicity, conservation, projection non-expansion, no hard-axis mutation and rollback equivalence. Fault tests cover storage-full, corrupted manifest, expired boundary condition, revoked artifact, acknowledgement loss and process kill at each transaction boundary.

## 11. Quantitative acceptance gates

| Gate | Required threshold |
|---|---|
| Zero-noise parity | every component within `2` Q32 units |
| Undeclared bound violation | `0` |
| Projection rate after warm-up | `<0.4%` |
| Resource residual | `<=1` Q32 unit |
| Risk residual | `<=10 ppm` |
| Boundary residual p99 | `<=10,000 ppm` |
| Coupling spectral radius upper 95% bound | `<0.95` |
| Standardized martingale conditional mean | absolute value `<0.02` |
| 90% interval coverage | `[0.87,0.93]` |
| Future utility | candidate LCB `>` baseline UCB |
| Old-task subgroup degradation | no worse than `2%` |
| Deterministic fallback coverage | `100%` supported objective classes |
| Current-run replacement | `0` |
| Learned hard-axis mutation | `0` |

A utility average cannot waive another failed gate. Threshold changes require a versioned qualification profile and independent review.

## 12. Paper traceability and Hepta extensions

`PAPER-NDU-FOUNDATIONS-2024` supplies the resource-constrained endogenous-preference motivation. `PAPER-NDU-UPA-2025` supplies the continuous-time FBSDE and residual-network approximation framing. `PAPER-NDU-EU-2025` supplies the multidimensional square-integrable martingale, forward preference state, backward utility aggregator and explicit well-posedness/control-condition boundary.

The four-level hierarchy, event sourcing, fixed-point durability, deterministic authority axes, causal evaluation, privacy/unlearning lineage and next-snapshot governance are Hepta extensions. The papers do not prove identifiability from Hepta telemetry, hierarchy stability, policy efficacy, biological fidelity or safe software self-iteration.

## 13. Implementation sequence and completion rule

Implementation order is protocol and fixed-point types → deterministic reference and golden vectors → append-only projections → latent filter → stochastic shadow path and certificate → backward regression and parity → frozen-parent hierarchy → immutable training → causal future-time and retention evaluation → signed next-snapshot artifact → rollback rehearsal → independent activation decision.

Documentation completion means this file, `ALGORITHM_SPECS.json`, `PAPER_TRACEABILITY.json` and exact CI agree. Source completion additionally requires code and tests. Dynamic NDU efficacy additionally requires future-time causal evidence. This specification does not by itself advance `D0_SPECIFIED_ONLY`.
