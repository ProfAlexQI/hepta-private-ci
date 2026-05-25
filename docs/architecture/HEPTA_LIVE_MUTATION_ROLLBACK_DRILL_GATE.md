# Hepta Live Mutation Rollback Drill Gate

Date: 2026-05-25

This gate proves that the post-absorption live mutation lane has a concrete
rollback plan before any mutation is allowed. It is deliberately a dry run: it
does not restore a binary, restart launchd, write release artifacts, or mutate
Gateway state.

## Contract

The gate requires:

- the release binary and installed binary both exist
- the release binary SHA matches the installed binary SHA
- at least one active-binary rollback anchor exists under
  `backups/hepta-active-binary-*/hepta.previous`
- the newest rollback anchor is executable, non-empty, and has a different SHA
  from the currently installed binary
- the installed binary directory exists and is writable by the operator account
- memory/capability absorption remains `14/14`
- `live_mutation_enabled_count = 0`
- release hardening still has `live_execution_enabled_count = 0`
- launchd and release-artifact mutation side effects remain false
- core fusion and engine dependency closure remain ready
- all live report side-effect maps remain false

## Dry-Run Output

The gate emits the exact restore sequence that would be reviewed under a
separate operator approval:

- copy the selected `hepta.previous` over the installed binary
- make the installed binary executable
- kickstart `ai.hepta.gateway`
- run `scripts/hepta-watchdog.sh`
- run a minimum 24-sample live soak

The emitted commands are evidence, not execution.

## Execution Boundary

The gate reports:

- `rollback_plan_ready = true`
- `rollback_execution_enabled = false`
- `operator_approval_required_before_execution = true`

Rollback execution still requires a separate approval id, a fresh backup of the
currently installed binary after approval, a single-surface activation scope,
post-restore watchdog evidence, post-restore 24-sample soak evidence, and a
side-effect receipt with no secret values.
