# G5 local-development profile v1

This profile removes the external ceremony from ordinary Hepta development.
It is the default profile for local implementation, qualification, shadow, and
sandbox work.  It does not change the production release contract.

## What is removed from the local path

- No external provider-owner handoff is required before local work starts.
- No 900-second challenge window, detached SSHSIG, or independent trust owner
  is required for a local development acknowledgement.
- No CALLERS production ratchet, deployment, or promotion is implied by a
  local receipt.

## Effect semantics

The local path uses `at_least_once_indeterminate_reconcile`:

`EffectIntent → DispatchAccepted → EffectReceipt` or
`Indeterminate → reconcile`.

The adapter must quarantine an unknown result and must not blind-retry it.  A
local receipt never claims provider physical exactly-once and external effects
are disabled by default; sandbox-only effects need an explicit test harness.

## Authority fields

The generated profile may set `g5_local_complete`,
`local_operator_acceptance`, and `local_fleet_shadow_allowed`.  It must keep
`production_activation`, `promotion`, `g5_allowed`,
`fleet_and_automation_unfrozen`, and `provider_physical_exactly_once` false.

The local acknowledgement records the interactive user's scope and is not a
cryptographic signature or an independent trust decision.  The production
profile, if ever requested, is a separate opt-in profile and may reintroduce
provider and trust requirements at that time; those requirements do not block
the local development plan.

The profile also records hard negative controls: `planning_only=true`,
`production_caller=false`, `production_writer=false`, `provider_effects=false`,
`kg_write_authority=false`, `governance_bypass=false`, and
`required_governance_mode=Shadow`.  A local profile cannot disable governance
or turn a local/sandbox receipt into production authority.

Generate a profile with:

```sh
python3 scripts/hepta-g5-local-profile.py \
  --candidate /Volumes/T5/hepta-vnext/worktrees/r2-g5-local-dev-profile-20260824 \
  --output /Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-profile-20260824 \
  --operator local-development-user --ack
```
