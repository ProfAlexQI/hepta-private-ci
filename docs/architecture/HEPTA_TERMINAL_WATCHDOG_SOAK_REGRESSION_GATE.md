# Hepta Terminal Watchdog/Soak Regression Gate

`scripts/hepta-terminal-watchdog-soak-regression-gate.sh` is a report-only release governance guard for the active Hepta service.

The gate runs two existing observational checks:

- `scripts/hepta-watchdog.sh`
- `scripts/hepta-live-soak.sh`

The watchdog source may be `ok` or a parseable operator-security attention
report, provided binary SHA parity, health, route coverage, full-fusion closure,
and side-effect boundaries remain intact. The companion attention-budget
diagnostic classifies the concrete owner/poll-loop shape, including legacy-owner
disabled-poll-loop observations. Both states are observational inputs only.

The soak source may also emit a parseable failed report under a known
operator-security attention boundary: either bounded production attention, or a
legacy-owner observation where Hepta Telegram ownership was not requested and
the poll loop is disabled. In that case the gate reports
`soak_known_operator_security_attention=true`; it does not treat the failed soak
as a passed soak or accepted long-soak evidence.

The default terminal soak is intentionally short: 3 samples at 1 second intervals. It is a regression check for preflight speed, not release-long-soak evidence and not an activation approval. Long-soak evidence remains a separate requirement before any live mutation path can be considered.

The same gate can also be run in explicit release-long-soak observation mode:

```sh
HEPTA_TERMINAL_SOAK_SAMPLES=24 HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS=1 \
  scripts/hepta-terminal-watchdog-soak-regression-gate.sh
```

In that mode the gate reports `terminal_soak_regression_class=release_long_soak_observation` and `release_long_soak_observed=true` when all samples pass. This is still report-only: it does not record, persist, accept, or materialize activation evidence, and it cannot authorize live mutation or public release claims.

If the same long-sample path is blocked by known operator-security attention, the
gate reports the attempted long-soak class with
`soak_known_operator_security_attention=true` and
`release_long_soak_observed=false`.

## Contract

The gate requires:

- watchdog status `ok`, or `watchdog_known_operator_security_attention=true`
- health status `ready`
- route count `>=69` (`70` after the memory/Intelligence/KG full-enablement runtime-readiness source route)
- missing route count `0`
- release and installed binary SHA match
- full fusion complete
- phase 4 name/repository closure remaining surface count `0`
- phase 5 engine dependency closure remaining dependency count `0`
- short soak status `ready` with failures `0`, or
  `soak_known_operator_security_attention=true` for a bounded attention or
  legacy-owner disabled-poll-loop observation
- at least 3 terminal soak samples
- minimum long-soak policy of at least 24 samples

The gate denies:

- install execution
- release build execution
- active service restart
- launchd mutation
- live mutation authorization
- public release or GA claims
- public distribution publication
- release/public artifact writes
- memory store mutation
- capability or plugin registry mutation
- provider/model invocation
- channel delivery
- upstream fetch or merge
- persistence of the terminal regression index
- persistence of watchdog or soak evidence

## Output

The script emits one JSON report followed by a pass line.

Important fields:

- `watchdog_soak_regression_ready`
- `watchdog_soak_regression_mode`
- `watchdog_soak_regression_decision`
- `source_watchdog_report_sha256`
- `source_soak_report_sha256`
- `watchdog_status_known`
- `watchdog_known_operator_security_attention`
- `regression_index_hash_sha256`
- `terminal_soak_samples`
- `soak_status_known`
- `soak_known_operator_security_attention`
- `soak_passed`
- `minimum_long_soak_required_samples`
- `terminal_soak_is_release_long_soak`
- `terminal_soak_regression_class`
- `release_long_soak_observed`
- `release_long_soak_sample_count`
- `release_long_soak_evidence_recorded`
- `release_long_soak_evidence_persisted`
- `release_long_soak_evidence_accepted`
- `release_long_soak_authorizes_activation`
- `terminal_soak_authorizes_live_mutation`
- `watchdog_soak_denied_by_count`
- `regression_families`
- `side_effects`

`side_effects` must remain all false. The gate performs local observational reads only and does not persist the generated evidence. A known watchdog attention report or known attention soak failure does not authorize live mutation, owner handoff, evidence persistence, release-long-soak acceptance, public release claims, artifact writes, install, restart, provider/model invocation, or channel delivery.

## Preflight Position

`scripts/hepta-preflight.sh` runs this gate after the terminal release-governance final audit index gate and before the `hepta-gateway` tests. This keeps the terminal governance layer connected to active service health without turning every preflight into a long soak.
