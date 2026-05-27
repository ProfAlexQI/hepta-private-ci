# Hepta Core Activation Readiness Summary Gate

## Purpose

This gate is the top-level read-only summary for the current Hepta Core activation boundary. It does not approve activation. It proves that the major prerequisite reports are present and coherent while the live mutation, public release, and release artifact paths remain blocked.

The summary verdict is:

- `blocked_until_explicit_operator_approval_and_fresh_live_evidence`

## Source Gates

The gate consumes five source reports:

1. `scripts/hepta-terminal-release-governance-final-audit-index-gate.sh`
2. `scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-gate.sh`
3. `scripts/hepta-active-service-dependency-isolation.sh`
4. `scripts/hepta-terminal-watchdog-soak-regression-gate.sh`
5. `scripts/hepta-memory-intelligence-closure.sh`

Each source report is captured through `scripts/lib/hepta-json-report-capture.sh`. The summary stores only in-process hashes in its JSON output and does not persist a ledger, receipt, release artifact, or approval record.

## Required Evidence

The summary requires:

- terminal release-governance final audit ready
- memory/intelligence surfaces absorbed or represented
- active `hepta-cli` dependency isolation from tracked Codex engine crates
- release artifact and public publication denial matrix ready
- watchdog route coverage ready with full fusion complete
- short soak healthy

The short soak is regression evidence only. It is not release-long-soak evidence and cannot authorize activation.

## Denied Outcomes

The gate keeps these outcomes false:

- operator approval recording
- activation
- live memory mutation
- memory store, capability registry, plugin registry, and skill workshop mutation
- provider or model invocation
- Telegram/channel/external send
- release artifact write
- public artifact write
- public distribution publication
- public release claim
- public GA claim
- install, restart, launchd mutation, and active binary mutation
- upstream fetch or merge
- credential or secret file read
- evidence, summary, watchdog, soak, final audit, or publication receipt persistence

## Readiness Families

The report emits six blocked readiness families:

1. memory/intelligence
2. release publication
3. final release-governance
4. active dependency isolation
5. watchdog and short soak
6. operator approval boundary

Each family must be `ready=true` and `blocked=true`.

## Verification

Expected verification:

- `bash -n` on the new gate and `scripts/hepta-preflight.sh`
- ASCII scan of the new gate, this doc, and the preflight edit
- focused gate execution
- `git diff --check`
- full light preflight:
  - `HEPTA_PREFLIGHT_NATIVE=0 HEPTA_PREFLIGHT_RELEASE=0 scripts/hepta-preflight.sh`
- live read-only sanity:
  - `scripts/hepta-watchdog.sh`
  - `HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=1 scripts/hepta-active-service-dependency-isolation.sh`
  - `HEPTA_SOAK_SAMPLES=3 HEPTA_SOAK_INTERVAL_SECONDS=1 scripts/hepta-live-soak.sh`

## Explicit Non-Actions

This gate must not:

- install or restart Hepta
- mutate launchd or the active binary
- write memory stores or registries
- write release artifacts or public artifacts
- publish, distribute, or claim public release status
- persist approval, final audit, watchdog, soak, or publication evidence
- invoke providers or models
- send Telegram, channel, or external messages
- read credentials or secret files
- fetch or merge upstream code
