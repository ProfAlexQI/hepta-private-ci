# Hepta Terminal Watchdog/Soak Regression Gate

`scripts/hepta-terminal-watchdog-soak-regression-gate.sh` is a report-only release governance guard for the active Hepta service.

The gate runs two existing observational checks:

- `scripts/hepta-watchdog.sh`
- `scripts/hepta-live-soak.sh`

The default terminal soak is intentionally short: 3 samples at 1 second intervals. It is a regression check for preflight speed, not release-long-soak evidence and not an activation approval. Long-soak evidence remains a separate requirement before any live mutation path can be considered.

The same gate can also be run in explicit release-long-soak observation mode:

```sh
HEPTA_TERMINAL_SOAK_SAMPLES=24 HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS=1 \
  scripts/hepta-terminal-watchdog-soak-regression-gate.sh
```

In that mode the gate reports `terminal_soak_regression_class=release_long_soak_observation` and `release_long_soak_observed=true` when all samples pass. This is still report-only: it does not record, persist, accept, or materialize activation evidence, and it cannot authorize live mutation or public release claims.

## Contract

The gate requires:

- watchdog status `ok`
- health status `ready`
- route count `69`
- missing route count `0`
- release and installed binary SHA match
- full fusion complete
- phase 4 name/repository closure remaining surface count `0`
- phase 5 engine dependency closure remaining dependency count `0`
- short soak status `ready`
- soak failures `0`
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
- `regression_index_hash_sha256`
- `terminal_soak_samples`
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

`side_effects` must remain all false. The gate performs local observational reads only and does not persist the generated evidence.

## Preflight Position

`scripts/hepta-preflight.sh` runs this gate after the terminal release-governance final audit index gate and before the `hepta-gateway` tests. This keeps the terminal governance layer connected to active service health without turning every preflight into a long soak.
