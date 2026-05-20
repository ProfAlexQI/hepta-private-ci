# Hepta CLI Command Breadth Inventory

Date: 2026-05-20
Scope: old standalone Hepta `hepta-cli` ops modules versus current `hepta-codex`
Status: read-only inventory landed; full old CLI command compatibility not claimed

## Summary

The old standalone Hepta repo still contains `65`
`crates/hepta-cli/src/*_ops.rs` files and roughly `574` slash/command
references. Those commands are not safe to bulk-enable in `hepta-codex` because
many are credentialed, channel-facing, provider-facing, or tied to old workspace
runtime assumptions.

This slice adds a side-effect-free native route:

- `/api/hepta-cli-command-inventory`
- source-command equivalent: `/hepta-cli-command-inventory --json`
- validation script: `scripts/hepta-codex-cli-command-inventory.sh`

The route does not invoke providers, read credentials, read Telegram, send
messages, perform native POST real mutations, write files, or call external
networks. It is an inventory and sequencing surface only.

## Inventory Counts

- old standalone `*_ops.rs`: `65`
- old rough command references: `574`
- old standalone scripts: `20`
- current `hepta-codex` scripts: `11` after memory/capability absorption inventory continuation
- current native gateway source commands: `58` after memory/capability absorption inventory continuation
- Control UI route parity after memory/capability absorption inventory continuation: `58/58`, missing `0`

## Ops Families

| Family | Count | Safe next mode |
| --- | ---: | --- |
| provider metadata bridges | 15 | read-only metadata report |
| channel/runtime adapters | 13 | disabled status inventory landed; explicit approval before live smoke |
| runtime ops/admin | 12 | dry-run inventory landed; expand planner contracts |
| local tooling/content | 11 | local planning inventory landed; explicit approval before temp-workspace smoke |
| memory/capability/absorption | 14 | read-only gap report |

The family counts sum to all `65` old ops files. That makes the remaining CLI
gap explicit without claiming command compatibility.

## Remaining Blockers

- old Hepta CLI command breadth is not fully migrated
- credentialed provider surfaces have not been live-smoked
- channel adapters have not been approved for owner handoff
- old CLI invocation compatibility is not claimed
- old external/release scripts are not fully ported

## Safe Sequence

1. Port remaining external release and hardening scripts as local-only status gates.
2. Only run temp-workspace local tooling smokes after explicit operator approval.
3. Defer credentialed/live smokes until explicit operator approval.

This keeps the migration moving while preserving the standing safety boundary:
no Telegram takeover, no live send, no native POST real handler, and no external
provider invocation without an explicit operator request.
