# Hepta Core Activation Long-Soak Observation Non-Acceptance Gate

This gate binds the explicit release-long-soak observation path to the activation
receipt terminal-closure denial path.

It exists to prevent a successful 24-sample observation from being reused as
activation evidence by implication.

## Scope

The gate runs two read-only sources:

- `scripts/hepta-terminal-watchdog-soak-regression-gate.sh` with
  `HEPTA_TERMINAL_SOAK_SAMPLES >= HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES`
- `scripts/hepta-core-activation-evidence-receipt-terminal-closure-decision-gate.sh`

The first source proves that a release-long-soak-class observation can be
observed. The second source proves that terminal closure remains blocked until
explicit operator approval, fresh trusted evidence records, ledger records,
receipt persistence and acceptance, delivery, and completion acknowledgement
exist.

## Non-Acceptance Contract

A successful release-long-soak observation may set:

- `release_long_soak_observed=true`
- `release_long_soak_sample_count >= 24`
- `terminal_soak_regression_class=release_long_soak_observation`

When the observation path is blocked by known operator-security attention, the
same source may instead set `observation_soak_known_operator_security_attention=true`
and `release_long_soak_observed=false`. That is still a non-acceptance state,
not evidence satisfaction.

It must not set:

- `release_long_soak_evidence_recorded`
- `release_long_soak_evidence_persisted`
- `release_long_soak_evidence_accepted`
- `long_soak_evidence_fresh`
- `operator_approval_recorded`
- `receipt_persisted`
- `receipt_accepted`
- `ledger_recorded`
- `terminal_closure_recorded`
- `activation_allowed`

The gate emits `status=ready` only when those non-acceptance guarantees are
simultaneously true.

## Denied Surfaces

The report keeps all activation and publication surfaces false:

- active runtime mutation
- active binary mutation
- install or launchd restart
- release artifact write
- public release claim
- provider/model invocation
- channel delivery
- credential or secret read
- upstream fetch or merge

Only local observational reads are allowed.

## Preflight Position

`scripts/hepta-preflight.sh` runs this gate immediately after the terminal
watchdog/soak regression gate and before the core activation readiness summary.

That ordering keeps the release-long-soak observation boundary explicit before
any readiness aggregation consumes adjacent activation evidence gates.
