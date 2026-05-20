# Hepta Provider Channel Dry Run Plan

Date: 2026-05-20
Scope: provider, search, channel, and runtime/session dry-run plan contracts
Status: local-only route landed; no live invocation, delivery, credential read, or store mutation

## Summary

This slice adds a deterministic `hepta-codex` route for the remaining high-risk
old OpenClaw/Hepta families that cannot be activated automatically:

- endpoint: `/api/hepta-provider-channel-dry-run-plan`
- source command: `/hepta-provider-channel-dry-run-plan --json`
- script gate: `scripts/hepta-codex-provider-channel-dry-run-plan.sh`

The route is a planner contract. It does not execute prompts, call providers,
query search/network surfaces, read credentials, deliver channel messages,
spawn processes, read or write files, enqueue gateway events, mutate task or
session stores, touch Telegram ownership, or activate native POST handlers.

## Covered Families

The route covers 43 unique old ops files as dry-run plan scope:

- provider ops: `15`
- adjacent search/readability ops: `3`
- channel adapter ops: `13`
- runtime/session/admin ops: `12`

The plan is grouped into five deterministic families:

- `provider-prompt-plan`
- `local-provider-registry-plan`
- `search-readability-plan`
- `channel-delivery-plan`
- `runtime-session-plan`

All five families expose isolated fixture contracts, and live execution remains
`0`.

## Safety Boundary

The endpoint reports all of the following as disabled:

- provider/model invocation
- credential read
- external network read or search query
- channel read/send
- Telegram owner handoff/read/send
- process spawn
- filesystem read/write
- task registry mutation
- session store mutation
- gateway event enqueue
- native POST mutation
- gateway mutation
- external send

## Script Contract

`scripts/hepta-codex-provider-channel-dry-run-plan.sh` validates:

- route-ready planner status
- 43 unique covered old ops files
- `5/5` dry-run families ready
- `5/5` isolated fixture contracts present
- live invocation, credential-read-required, provider prompt execution, search
  network query, channel delivery, runtime store mutation, and fixture
  materialization all disabled
- all side-effect flags remain false
- planner, release/hardening, memory/capability, local tooling/content, channel,
  runtime/session, provider, CLI, and merge-completion reports agree on route,
  script, and source-command counts.

## Current Counts

- current `hepta-codex` script total: `13`
- native gateway source-command count: `60`
- expected route parity after install: `60/60`

## Remaining Boundary

This closes the next safe dry-run planning slice, but it does not claim old CLI
invocation compatibility or production replacement. Actual provider prompts,
credentialed smokes, network search, channel delivery, runtime store mutation,
fixture materialization with file/process side effects, Telegram owner handoff,
or native POST real mutation still require an explicit operator request for the
specific action.
