# G5 local-development profile v1

This profile removes the external ceremony from ordinary Hepta development.
It is the documented declaration profile for local implementation,
qualification, shadow, and sandbox work.  It does not change the production
release contract or configure a runtime by itself.

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
`required_governance_mode=shadow`.  The declaration does not authorize
disabling governance or turn a local/sandbox receipt into production authority;
the actual entrypoint/configuration must still enforce that boundary.

This is a profile-specific, declaration-only manifest; it is not a production
runtime switch.  The generator accepts only the fixed local worktree,
the exact candidate head/tree/parent and the two profile files as the delta.
Evidence is read only from the T5 artifact root selected by the SSD wrapper and
is rejected if it is a symlink or changes while being hashed.  The manifest records
`authority_status=not_granted`, `promotion_status=not_eligible`, and
`production_operator_acceptance=false` so a local acknowledgement cannot be
mistaken for a production operator decision.

Generate a profile with:

```sh
HEAD=$(git -C /Volumes/T5/hepta-vnext/worktrees/r2-g5-local-dev-profile-20260824 rev-parse HEAD)
TREE=$(git -C /Volumes/T5/hepta-vnext/worktrees/r2-g5-local-dev-profile-20260824 rev-parse 'HEAD^{tree}')
PARENT=$(git -C /Volumes/T5/hepta-vnext/worktrees/r2-g5-local-dev-profile-20260824 rev-parse HEAD^)
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run r2-g5-local-dev-profile-20260824 -- \
python3 scripts/hepta-g5-local-profile.py \
  --candidate /Volumes/T5/hepta-vnext/worktrees/r2-g5-local-dev-profile-20260824 \
  --output /Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-profile-v5-20260824 \
  --operator local-development-user --ack \
  --expected-head "$HEAD" --expected-tree "$TREE" --expected-parent "$PARENT"
```
