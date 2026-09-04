# NDU system integration and solver specification

**Overlay:** `HEPTA-V8-PRECODING-READINESS` v8.2.0-readiness
**Bound modules:** `utility.ndu`, `objective.compiler`, `control.runtime`, `intuition.policy`, `learning.eval`, `learning.ledger`
**Source target:** `codex-rs/hepta-ndu`

## 1. Scope and authority boundary

This specification turns NDU from an isolated mathematical document into a system-wide, typed value contract. NDU compares only feasible candidates supplied by an immutable objective and legal-action set. Authority, truth, privacy, deletion, single-writer ownership and emergency-stop state never become utility dimensions and cannot be traded for higher reward.

The first implementation is deterministic and fixed-point. Stochastic preference dynamics and learned FBSDE coefficients remain later shadow candidates. `utility.ndu` may publish summaries and next-revision candidates; it cannot execute effects, select artifacts, diagnose a person or mutate the current objective.

## 2. Cross-organ utility contract

Every organ or module that contributes to planning emits `UtilityContributionV1` with:

```text
objective and subject identity
organ identity and support digest
bounded utility vector
bounded risk vector
resource-cost vector
uncertainty vector
time horizon
hard-constraint violation flag
```

The utility vector uses registered dimensions such as task success, evidence quality, latency, resource cost, reversibility and user burden. Dimensions have explicit units and monotonic direction. A contribution without support, units, objective digest or current generation is unavailable. An organ cannot publish a utility contribution for another owner’s state.

`control.runtime` aggregates current contributions into a snapshot; `utility.ndu` computes preference and continuation utility; `learning.ledger` records the complete candidate/contribution set. Missing contribution is represented as uncertainty or unavailability, never zero benefit.

## 3. Multi-objective feasibility and Pareto policy

Selection is two-stage:

```text
hard feasibility filter -> Pareto frontier -> optional registered scalarization
```

Candidates violating any hard constraint are removed before utility arithmetic. Among feasible candidates, dominated points are removed using dimension direction and tolerance. Scalarization is permitted only when the objective names a versioned weight profile whose weights and units are complete. Without a valid profile, the output is the Pareto set plus an abstain or slow-path request, not an invented total order.

Risk ceilings and essential resource floors are constraints, not negative utility. Lexicographic profiles are allowed for safety-critical tasks and take precedence over weighted sums. The no-op/abstain candidate remains in every legal set.

## 4. Deterministic hierarchical solver

The baseline has four subject classes: system, domain, agent and episode. Updates are staged rather than simultaneous:

```text
1. freeze system and domain revisions for a run generation;
2. update episode state at registered decision boundaries;
3. consolidate episode evidence into an agent candidate after terminal outcome;
4. evaluate and accept agent candidates against frozen domain boundaries;
5. update domain candidates in a later generation;
6. update the system candidate only after domain snapshots and independent evaluation.
```

For subject `s` and step `k`, the deterministic baseline is:

```text
P_candidate = project(P_k + dt * bounded_drift(observation, cost, risk, action))
P_next = (1 - eta) * P_k + eta * P_candidate
U_k = project(instant_utility + discount * continuation_utility)
```

All arithmetic is signed Q32 with round-to-nearest, ties-to-even. `eta` is in `[1/16, 1/4]`. Parent and child artifacts cannot be selected in the same generation. Each step emits `NduIterationReceiptV1` with residuals, projection counts and exact predecessor.

## 5. Convergence, infeasibility and multiple solutions

A deterministic solve has explicit maximum iterations, residual tolerance and wall-clock budget. Pilot values are `64` iterations, maximum absolute normalized residual `<=2^-20`, resource conservation `<=1` Q32 unit and risk conservation `<=10 ppm`. Hitting the iteration or time bound returns `unavailable` with the last bounded interval; it does not assert convergence.

Infeasible hard constraints produce an empty feasible set and an explicit abstain/clarification result. For multiple fixed points, candidates are partitioned by admissible basin; the solver selects the predecessor-nearest solution only when it is stable under registered perturbations and independently certified. Otherwise it emits `multiple_solution_unresolved`.

`NduConvergenceCertificateV1` binds solver, initialization, residuals, spectral-radius upper confidence bound, conservation and evaluator identity. A certificate with spectral-radius upper 95% bound `>=0.95`, unsupported dimensions or stale objective fails activation.

## 6. State, persistence and scheduling

Preference and utility projections are append-only revisions owned by `utility.ndu`. A transactional selected pointer is updated only after the immutable row and certificate exist. Idempotency key is `(subject_id, predecessor_revision, objective_digest, event_digest, coefficient_digest)`.

The local hot path consumes cached system/domain summaries and current episode/agent state. It never performs synchronous fleet-wide optimization. Corrections and deletion append revocation edges; rebuild excludes revoked source events, datasets and artifacts. Mixed objective, preference or body generations are rejected.

## 7. Goodhart and wireheading controls

Outcome definitions and observers are owned outside the policy being evaluated. NDU cannot write terminal success, change its own evidence requirements, alter evaluation slices or treat internal activation as user utility. Proxy metrics are registered with known failure modes and at least one non-proxy holdout.

Controls include complete candidate logging, independent outcomes, future-window evaluation, adversarial proxy tests, no self-issued selection, reward-channel integrity checks, causal ablations, subgroup floors and explicit resource accounting. A gain in one utility dimension cannot waive safety, privacy, deletion, support or retention failure.

## 8. Numerical and resource envelope

Pilot bounds are preference dimension `<=64`, utility dimension `<=8`, risk/resource dimensions `<=32`, hierarchy depth `4`, candidate count `<=128` and one selected coefficient artifact per process generation. Update p95 is `<=2 ms`, p99 `<=5 ms`, persistent state `<=256 KiB` per active subject and transient allocation `<=256 KiB`.

All normalization, units, clipping and fixed-point scales are manifest-bound. NaN, infinity, dimension drift, unknown unit, excessive projection rate or conservation failure quarantines the candidate and selects the deterministic predecessor.

## 9. Golden fixtures and tests

- `NDU-SYS-GV-001`: the existing two-step zero-noise preference/utility vector reproduces exact Q32 values.
- `NDU-SYS-GV-002`: a candidate with higher utility but a hard privacy violation is filtered before Pareto analysis.
- `NDU-SYS-GV-003`: two non-dominated candidates without a scalarization profile return a Pareto set and slow-path disposition.
- `NDU-SYS-GV-004`: simultaneous parent/child update oscillates in the fixture and is rejected; staged damped update converges.
- `NDU-SYS-GV-005`: resource contributions sum above endowment and produce infeasible, not negative utility.
- `NDU-SYS-GV-006`: tampering with an outcome observer identity invalidates the evaluation chain.

Property tests cover permutation invariance, projection boundedness, monotonic revision, conservation, parent freezing, no hard-axis mutation, crash/reopen, rollback and deletion rebuild.

## 10. Implementation sequence

Implement `UtilityContributionV1`, deterministic vector feasibility/Pareto logic, fixed-point subject state, episode update, backward utility recursion, append-only persistence, staged agent/domain hierarchy, convergence certificates and deterministic fallback. Only then add stochastic coefficients, learned approximators and Bellman integration in shadow mode.

## 11. Coding-entry checklist

Coding may start when the three readiness protocols and canonical NDU protocols compile, the objective and legal-set digests are frozen, utility dimensions and units are registered, deterministic golden fixtures are immutable, persistence ownership is confirmed, and `NDU-0`, `NDU-1` and `NDU-2` envelopes preserve no current-run mutation and no effect authority.

## Appendix A. Closed gap and protocol mapping

This appendix is a closed-world traceability projection. Each identifier is normative in `READINESS.json`, `PROTOCOLS.json` or `GAPS.json`; this Markdown file does not redefine the registry record.

Protocols:

- `UtilityContributionV1`
- `NduIterationReceiptV1`
- `NduConvergenceCertificateV1`

Closed documentation gaps:

- `RDY-GAP-NDU-001`
- `RDY-GAP-NDU-002`
- `RDY-GAP-NDU-003`
- `RDY-GAP-NDU-004`
- `RDY-GAP-NDU-005`
- `RDY-GAP-NDU-006`

Bound work packages:

- `BIO-0-NEURON-INTUITION-CONTRACTS`
- `DOC-3E-PRECODING-READINESS-CLOSED-WORLD`
- `INT-1-CALIBRATED-INTUITION-POLICY`
- `LONG-1-TEMPORAL-HOLDOUT`
- `LONG-2-RETENTION-FORGETTING`
- `LONG-3-UNLEARNING-NON-RESURRECTION`
- `LRN-0-CAUSAL-LEARNING-CONTRACTS`
- `LRN-1-DURABLE-EPISODE-LEDGER`
- `LRN-2-CAUSAL-EVALUATION`
- `NDU-0-PREFERENCE-UTILITY-CONTRACTS`
- `NDU-1-DETERMINISTIC-UTILITY-BASELINE`
- `NDU-2-AGENT-DOMAIN-HIERARCHY`
- `OBJ-0-OBJECTIVE-CONTRACTS`
- `OBJ-1-OBJECTIVE-COMPILER`
- `RCP-1-RUNTIME-CONTROL-PLANE`
