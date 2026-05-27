# Hepta Core Activation Long-Soak Observation Freshness Denial Gate

This gate separates a successful release-long-soak observation from a fresh
trusted long-soak evidence record.

It exists to prevent a 24/24 observation from being treated as fresh evidence by
timestamp inference, report reuse, stale record replay, or activation-scope
confusion.

## Scope

The gate runs two read-only sources:

- `scripts/hepta-core-activation-long-soak-observation-non-acceptance-gate.sh`
- `scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh`

The first source proves that an explicit 24-sample-class observation exists but
is not accepted as evidence. The second source proves that all activation
evidence slots remain missing and that fresh evidence count is still zero.

## Freshness Denial Contract

A release-long-soak observation may report:

- `release_long_soak_observed=true`
- `release_long_soak_sample_count >= 24`
- `release_long_soak_ok_count == release_long_soak_sample_count`

It still must not report:

- `long_soak_observation_recorded_as_evidence`
- `long_soak_observation_persisted_as_evidence`
- `long_soak_observation_accepted_as_fresh_evidence`
- `long_soak_evidence_fresh`
- `fresh_trusted_record_count > 0`
- `operator_approval_recorded`
- `ledger_recorded`
- `receipt_accepted`
- `terminal_closure_accepted`
- `activation_allowed`

The gate emits `status=ready` only when those denials remain true.

## Rejected Fixtures

The fixture matrix rejects:

- observation without a trusted evidence id
- stale long-soak evidence
- insufficient sample count
- source report hash mismatch
- activation scope mismatch
- fresh soak without operator approval
- fresh soak without ledger or receipt acceptance
- freshness claims paired with public release or artifact-write attempts

All fixtures remain blocked and side-effect free.

## Denied Surfaces

The report keeps these surfaces false:

- active runtime mutation
- active binary mutation
- install or launchd restart
- release artifact write
- public release claim
- provider/model invocation
- channel delivery
- credential or secret read
- upstream fetch or merge

Only local observational source reads are allowed.

## Preflight Position

`scripts/hepta-preflight.sh` runs this gate immediately after the long-soak
observation non-acceptance gate and before the core activation readiness
summary.

That ordering makes freshness denial explicit before readiness aggregation and
before later operator-approval, ledger, receipt, acceptance, and terminal
closure gates consume adjacent activation evidence fields.
