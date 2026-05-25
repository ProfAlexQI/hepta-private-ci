# Hepta Terminal Non-Activation Release-Claim Index Gate

This gate is a schema-only, side-effect-free release-claim denial index. It
combines the terminal denial index with live watchdog and public-GA readiness
evidence, then proves that operational readiness does not imply public release,
public GA, release artifact, public artifact, activation, or live mutation
permission.

## Purpose

Hepta can be operationally healthy while still refusing public release claims.
This gate makes that distinction explicit. A green watchdog, synchronized local
reports, and complete fusion readiness are evidence inputs only; they are not
authorization to publish, write artifacts, activate upstream Codex runtime
wiring, or persist release evidence.

## Source Reports

The gate consumes exactly three existing reports:

- `scripts/hepta-terminal-denial-index-gate.sh`
- `scripts/hepta-watchdog.sh`
- `scripts/hepta-public-ga-readiness.sh`

Each report is hashed into the final index. The gate does not fetch upstream
code, merge branches, mutate the active runtime, write files, restart launchd,
invoke providers, read credentials, send messages, publish releases, or persist
release evidence.

## Ready Criteria

The terminal non-activation release-claim index is ready only when all of these
conditions hold:

- terminal denial index is ready, activation-blocking, and has 39 terminal
  denial reasons;
- watchdog is `ok`, health is `ready`, route count is 69, binary SHA matches,
  full fusion is complete, and Phase 4/Phase 5 remaining counts are zero;
- public-GA readiness reports are synchronized, route gaps are zero, and no
  public GA claim has been made;
- public release claim, public GA claim, release artifact write, public artifact
  write, activation, active wiring, and live mutation remain denied;
- the release-claim index itself is not recorded, persisted, materialized, or
  written to the filesystem.

## Side-Effect Boundary

All side effects must stay false:

- memory, capability, plugin, runtime, gateway, launchd, and service mutation;
- upstream fetch, merge, checkout, auto-rebase, or dependency mutation;
- command, materialization, receipt, ledger, or filesystem persistence;
- release artifact, public artifact, public release, or public GA claim;
- provider/model invocation, external send, credential read, or secret read.

## Relationship To Public GA

This gate is intentionally not a public GA claim. It is a local proof that the
system can report operational health while the public release and artifact
surface remains locked behind explicit operator approval, fresh evidence, and
future release governance.
