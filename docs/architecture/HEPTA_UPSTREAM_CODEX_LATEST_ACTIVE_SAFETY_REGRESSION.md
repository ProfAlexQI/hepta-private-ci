# Hepta Upstream Codex Latest Active-Safety Regression Gate

`scripts/hepta-upstream-codex-latest-active-safety-regression.sh` binds the
latest observed Codex intake to active Hepta runtime safety evidence.

It consumes three existing source gates:

- `scripts/hepta-upstream-codex-latest-multisurface-absorption.sh`
- `HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0 scripts/hepta-active-service-dependency-isolation.sh`
- `scripts/hepta-terminal-watchdog-soak-regression-gate.sh`

The current latest Codex delta remains:

- baseline: `9f42c89c0112771dc29100a6f3fc904049b2655f`
- target: `8a94430bb273623be42b68f144f1ab1df343bb53`
- scope: `12` commits and `57` changed Codex files

## Contract

The gate is ready only when all of these are true:

- the latest upstream delta is classified as oracle-only intake;
- all five latest-delta families are ready and activation-blocking;
- the active `hepta-cli --bin hepta` cargo tree contains zero tracked Codex
  engine crates;
- watchdog evidence remains ready with route count `>=69`, no missing routes,
  binary SHA match, `full_fusion_complete=true`, and either watchdog `ok` or
  known operator-security attention;
- short soak passes as observational regression evidence only, or is classified
  as known operator-security attention with all samples failed and no live
  mutation authority;
- the short soak does not authorize live mutation, public release claims,
  release artifact writes, public distribution, or evidence persistence.

## Denied Actions

This gate does not fetch upstream, merge, rebase, checkout, install, restart,
write release artifacts, persist evidence, invoke providers, send through
channels, or mutate Gateway/runtime state. It is a regression binding between
latest upstream observation and active-runtime non-mutation.

## Preflight Position

`scripts/hepta-preflight.sh` runs this gate after the terminal watchdog/soak
regression gate and before the gateway test block. This keeps the newest
upstream Codex observation tied to the same active safety envelope used for
terminal release-governance checks.
