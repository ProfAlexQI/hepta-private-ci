# Hepta Terminal Operator-Readiness Non-Approval Index Gate

This gate is a schema-only, side-effect-free operator-readiness denial index. It
combines the terminal release-claim denial index with the public-GA operator
approval packet and rollback drill evidence, then proves that readiness evidence
does not equal operator approval, rollback authorization, activation, public
release, or artifact write permission.

## Purpose

The previous terminal release-claim index establishes that watchdog and public
GA readiness are evidence only. This gate adds the operator boundary: a ready
operator packet and a ready rollback plan remain non-authorizing until a future
explicit operator approval is recorded, scoped, bound to evidence, and reviewed.

## Source Reports

The gate consumes exactly three existing reports:

- `scripts/hepta-terminal-non-activation-release-claim-index-gate.sh`
- `scripts/hepta-public-ga-operator-approval-packet.sh`
- `scripts/hepta-live-mutation-rollback-drill-gate.sh`

Each report is hashed into the final index. The gate does not fetch upstream
code, merge branches, mutate the active runtime, write files, replace binaries,
restart launchd, execute rollback commands, invoke providers, read credentials,
send messages, publish releases, or persist operator evidence.

## Ready Criteria

The operator-readiness non-approval index is ready only when all of these hold:

- release-claim denial index is ready, activation-blocking, and has 47 denial
  reasons;
- operator packet is ready, synchronized, route-complete, and in
  `plan_only_no_live_mutation` mode with eight required operator approvals;
- rollback drill is ready but `dry_run_no_restore_no_restart`;
- release and installed SHA match, rollback backup exists and would change the
  installed binary, but rollback execution stays disabled;
- operator approval, operator identity acceptance, rollback restore, launchd
  restart, post-restore soak, activation, live mutation, public release claim,
  public GA claim, release artifact write, and public artifact write are all
  denied;
- the operator-readiness index itself is not recorded, persisted, materialized,
  or written to the filesystem.

## Side-Effect Boundary

All side effects stay false:

- memory, capability, plugin, runtime, gateway, launchd, and service mutation;
- upstream fetch, merge, checkout, auto-rebase, or dependency mutation;
- command, materialization, receipt, ledger, rollback, or filesystem execution;
- release artifact, public artifact, public release, or public GA claim;
- provider/model invocation, external send, credential read, or secret read.

## Relationship To Approval

This gate intentionally does not approve anything. It is a local proof that
Hepta can describe operator readiness and rollback readiness while keeping every
irreversible action locked behind a future explicit approval, scoped request,
fresh evidence bundle, and reviewed rollback path.
