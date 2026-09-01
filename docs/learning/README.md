# Hepta adaptive learning specifications

This directory contains the canonical machine registries and implementation-level specifications for Hepta adaptive and longitudinal intelligence. The global development authority remains [`../DEVELOPMENT.md`](../DEVELOPMENT.md). Documentation closure does not imply source implementation, activation, efficacy, acceptance, promotion or release.

## Read order

1. [`ALGORITHM_SPECS.json`](ALGORITHM_SPECS.json) — closed-world coverage, exact Git blob identities and mandatory closure gates.
2. [`PAPER_TRACEABILITY.json`](PAPER_TRACEABILITY.json) — exact paper identities, claims used, non-claims and Hepta-specific extensions.
3. [`ALGORITHM_STATUS.md`](ALGORITHM_STATUS.md) — generated coverage and truthful capability posture.
4. [`NDU_FBSDE_SPEC.md`](NDU_FBSDE_SPEC.md) — forward-backward preference/recursive-utility model, deterministic baseline, well-posedness, hierarchy and thresholds.
5. [`HOLDER_BELLMAN_SPEC.md`](HOLDER_BELLMAN_SPEC.md) — Hölder applicability certificate, smooth/jump/hard partition, sensor geometry, anisotropic operator and error budget.
6. [`CAUSAL_LONGITUDINAL_SPEC.md`](CAUSAL_LONGITUDINAL_SPEC.md) — complete candidate sets, propensity, independent outcomes, OPE, future-time retention and unlearning.
7. [`NEURAL_BIOMIMICRY_SPEC.md`](NEURAL_BIOMIMICRY_SPEC.md) — temporal state, sparse competition, inhibition, homeostasis, eligibility, replay, lesion and ablation.
8. [`SELF_ITERATION_SPEC.md`](SELF_ITERATION_SPEC.md) — bounded mutation grammar, sandbox, no-change baseline, independent decisions, pull-request proposal and rollback.
9. [`REFERENCE_CONFORMANCE_SPEC.md`](REFERENCE_CONFORMANCE_SPEC.md) — fixed-point, randomness, golden vectors, fault, performance, exact-source and synthetic-merge conformance.
10. Existing machine sources: [`LEARNING_SYSTEM.json`](LEARNING_SYSTEM.json), [`EXPERIMENTS.json`](EXPERIMENTS.json) and [`ARTIFACTS.json`](ARTIFACTS.json).

## Validation

```bash
python3 scripts/hepta-algorithm-docs.py self-test
python3 scripts/hepta-algorithm-docs.py generate-status
git diff --exit-code -- docs/learning/ALGORITHM_STATUS.md
python3 scripts/hepta-algorithm-docs.py verify
```

The dedicated workflow verifies both the exact source head and a deterministically constructed synthetic merge candidate, retains receipts, and also reruns the global and forty-module documentation gates.
