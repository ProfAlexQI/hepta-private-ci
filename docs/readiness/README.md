# Hepta implementation-readiness closure

This directory closes the remaining pre-coding documentation blockers for the Hepta V8 architecture. It is a canonical subordinate specification layer under `docs/DEVELOPMENT.md`; it does not claim source implementation, runtime activation, longitudinal efficacy, biological equivalence, physical safety, autonomous propagation, acceptance, selection, merge, promotion or release.

## Read order

1. [`READINESS.json`](READINESS.json) — closed-world document, module, lane, integration and assimilation bindings.
2. [`PROTOCOLS.json`](PROTOCOLS.json) — 31 implementation-level typed protocols.
3. [`GAPS.json`](GAPS.json) — 54 documentation gaps closed at specification level and separately named external gates.
4. [`SOURCE_BASELINE_AND_BRANCH_POLICY.md`](SOURCE_BASELINE_AND_BRANCH_POLICY.md) — exact source, branch purpose, merge identity and base-drift rules.
5. [`OBJECTIVE_COMPILER_EXECUTION.md`](OBJECTIVE_COMPILER_EXECUTION.md) — bounded objective grammar, precedence, canonicalization and fixtures.
6. [`NDU_SYSTEM_EXECUTION.md`](NDU_SYSTEM_EXECUTION.md) — cross-organ utility, Pareto policy, deterministic hierarchy and convergence.
7. [`NEURON_RUNTIME_EXECUTION.md`](NEURON_RUNTIME_EXECUTION.md) — state layout, tick ordering, checkpointing, performance and ablations.
8. [`LEARNING_EVALUATION_EXECUTION.md`](LEARNING_EVALUATION_EXECUTION.md) — preregistration, support, outcomes, OPE, future windows, retention and unlearning.
9. [`SELF_ITERATION_EXECUTION.md`](SELF_ITERATION_EXECUTION.md) — mutation grammar, protected surfaces, sandbox, lineage and rollback.
10. [`EMBODIED_RUNTIME_EXECUTION.md`](EMBODIED_RUNTIME_EXECUTION.md) — timing, calibration, body generation, reflex, actuation and HIL semantics.
11. [`EXTERNAL_SYSTEM_ASSIMILATION.md`](EXTERNAL_SYSTEM_ASSIMILATION.md) — explicitly authorized Debian/POSIX discovery, wrapping, migration, qualification and federation.
12. [`PARALLEL_DEVELOPMENT.md`](PARALLEL_DEVELOPMENT.md) — all-40-module lane plan and integration checkpoints.
13. [`STATUS.md`](STATUS.md) — deterministic generated closure status.

## Validation

```bash
python3 scripts/hepta-readiness.py self-test
python3 scripts/hepta-readiness.py generate-status --check
python3 scripts/hepta-readiness.py verify
python3 scripts/hepta-docs.py verify
```

The first implementation slice remains deterministic and read-only. Adaptive, structural, physical and external-system changes are candidates until separate evidence and independent decisions exist.
