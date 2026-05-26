# Hepta Upstream Codex Latest Release-Governance Non-Activation Gate

`scripts/hepta-upstream-codex-latest-release-governance-non-activation-gate.sh`
binds the latest observed Codex intake to Hepta's terminal release-governance
audit. It is intentionally a report-only gate.

The gate consumes two source reports:

- `scripts/hepta-upstream-codex-latest-active-safety-regression.sh`
- `scripts/hepta-terminal-release-governance-final-audit-index-gate.sh`

The current latest Codex delta remains:

- baseline: `9f42c89c0112771dc29100a6f3fc904049b2655f`
- target: `8a94430bb273623be42b68f144f1ab1df343bb53`
- scope: `12` commits and `57` changed Codex files

## Contract

The gate is ready only when all of these stay true:

- latest Codex intake remains oracle-only and activation-blocking;
- active `hepta-cli --bin hepta` dependency isolation remains clean with zero
  tracked Codex engine crates;
- watchdog and short soak remain green and observational only;
- terminal release-governance final audit remains ready;
- public release claims, public GA claims, public distribution, release artifact
  writes, and public artifact writes remain denied;
- evidence, publication summaries, receipts, ledgers, and this gate's own index
  are not persisted or materialized;
- upstream fetch, upstream merge, active dependency mutation, install, launchd
  restart, provider/model invocation, channel delivery, memory mutation, skill
  writes, plugin registry mutation, and Gateway/runtime mutation remain false.

## Denied Actions

This gate does not fetch upstream, merge, rebase, checkout, install, restart,
write release artifacts, persist evidence, record operator approval, invoke
providers, send through channels, or mutate Gateway/runtime state. It makes the
boundary explicit: seeing and classifying the latest upstream Codex state is not
the same as activating it or making a public release claim.

## Preflight Position

`scripts/hepta-preflight.sh` runs this gate after the latest active-safety
regression gate and before the gateway test block. This keeps upstream Codex
freshness, active-runtime non-mutation, and terminal release governance in one
auditable chain.
