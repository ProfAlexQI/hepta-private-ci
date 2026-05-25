# Hepta Live Mutation Governance Gate

Date: 2026-05-25

This gate records the boundary after memory/intelligence absorption reached
`14/14`. It does not enable live writes. Its job is to make the next decision
auditable before any memory, capability, plugin, search, skill, provider,
channel, runtime-store, or gateway mutation can be considered.

## Contract

The gate requires the current live reports to agree on these facts:

- memory/capability surfaces are `14/14` absorbed or represented
- `live_mutation_enabled_count = 0`
- memory store, capability registry, plugin registry, coding-agent spawn,
  search-provider live query, and skill-workshop write are all disabled
- provider/model invocation, channel delivery, runtime store mutation, and
  Gateway event enqueue remain disabled
- the public GA operator packet exists and its safe default mode is
  `plan_only_no_live_mutation`
- irreversible actions are blocked by default
- the active service is fully fused and the active binary package is
  `hepta-cli`
- the release binary SHA matches the installed binary SHA
- at least one current `hepta.previous` active-binary rollback anchor exists

## Mutation Policy

The gate deliberately reports `live_mutation_execution_ready = false`.

Any future live mutation must have a separate, scoped approval plan with:

- a concrete operator approval id
- a single-surface activation scope
- a current installed-binary backup
- a rollback or revert command
- a minimum 24-sample live soak before activation
- post-activation watchdog and soak evidence
- a side-effect receipt that records no secret values

## Safety Boundary

The gate must not:

- write memory
- write skills
- mutate plugin or capability registries
- spawn a coding agent
- perform provider/model invocation
- send through any channel
- mutate runtime stores or Gateway queues
- write release artifacts
- mutate launchd state

It may read live status reports and local binary/backup metadata only to prove
the approval, rollback, and soak prerequisites are enforceable.

The next gate is
`scripts/hepta-live-mutation-approval-evidence-receipt-gate.sh`, which binds
this governance report to the rollback drill report as a no-write candidate
receipt. That receipt still records `activation_allowed_by_receipt = false`.
