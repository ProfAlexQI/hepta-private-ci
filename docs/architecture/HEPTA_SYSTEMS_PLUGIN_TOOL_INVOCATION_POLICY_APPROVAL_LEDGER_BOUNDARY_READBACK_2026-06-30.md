# Hepta Systems Plugin Tool Invocation Policy Approval Ledger Boundary Readback

## Purpose

This readback turns the plugin tool invocation noop denial receipt into a
policy, approval, ledger, receipt, and idempotency boundary. It keeps
`hepta-system@hepta-local` ready-blocked while proving that the operator-visible
decision points before a plugin tool invocation are deterministic and queryable.

The readback is query-only. It does not persist policy decisions, execute
approval preflights, attempt ledger writes, persist receipt anchors, register
tools, execute ToolRegistry lookup, invoke tools, persist noop results, start
dynamic activation, grant permissions, start MCP servers, start app connectors,
install plugins, materialize cache, write runtime event logs, or open live
execution.

## Source

- Source report:
  `scripts/hepta-systems-plugin-tool-invocation-noop-denial-receipt-readback-report.sh`
- Readback gate:
  `scripts/hepta-systems-plugin-tool-invocation-policy-approval-ledger-boundary-readback-gate.sh`
- Source plugin:
  `plugins/hepta-system/.codex-plugin/plugin.json`

## Contract

For each hepta-system candidate contribution, this readback projects:

- a policy decision id
- a policy decision digest
- an approval preflight denial id
- an approval denial receipt id
- a ledger write denial id
- a ledger denial receipt id
- a receipt anchor id
- stable first and second policy boundary receipt ids
- stable first and second policy idempotency keys

Together these make policy decision ids, policy decision digests, approval preflight denials, approval denial receipts, ledger write denials, ledger denial receipts, receipt anchors, stable policy boundary receipts, and idempotency keys queryable before any plugin tool path can invoke or persist.

The source plugin tool invocation noop denial receipt readback must already have
projected invocation denial ids, noop result projections, noop result digests,
stable invocation denial receipts, ledger denial anchors, approval denial
anchors, receipt denial anchors, idempotency keys, and zero registration,
lookup, invocation, ledger, receipt, runtime, or live side effects.

## Closed Boundary

Closed boundary: no policy decision persistence, approval preflight execution, ledger write attempt, receipt anchor persistence, ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, noop result persistence, ledger write, approval request, receipt persistence, dynamic activation, permission grant, MCP server start, app connector start, plugin install, cache mutation, install-cache materialization, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Expected Readback

- 2 candidate contributions remain in scope.
- 2/2 policy decision ids are projected.
- 2/2 policy decision digests are projected.
- 2/2 approval preflight denial ids are projected.
- 2/2 approval denial receipts are projected.
- 2/2 ledger write denial ids are projected.
- 2/2 ledger denial receipts are projected.
- 2/2 receipt anchors are projected.
- 2/2 policy boundary receipts are projected.
- 2/2 policy boundary receipts are stable.
- 2/2 policy boundary receipts are unique.
- 2/2 policy idempotency keys are projected.
- 2/2 policy idempotency keys are stable.
- 2/2 policy idempotency keys are unique.
- 0 policy decision persistence writes, approval preflight executions, ledger
  write attempts, receipt anchor persistence writes, ToolRegistry registrations,
  ToolRegistry mutations, registry lookups, tool invocations, noop result
  persistence writes, ledger writes, approval requests, receipt persistence
  writes, dynamic activations, permission grants, connector starts, cache
  mutations, plugin installs, runtime writes, SQLite writes, or live executions
  occur.

## Next Step

The recommended next step is
`hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback`:
bind the selected read-only hepta-system status tool to a feature-gated dry-run
execution contract while still keeping registration, lookup, invocation,
ledger, receipt, runtime, and live mutation closed.
