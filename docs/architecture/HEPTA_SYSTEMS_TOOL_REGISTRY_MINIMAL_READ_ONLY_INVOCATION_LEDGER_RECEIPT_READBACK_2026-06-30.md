# Hepta Systems ToolRegistry Minimal Read-Only Invocation Ledger Receipt Readback

Date: 2026-06-30

## Scope

This readback closes the first narrow tools-system runtime contract after the
gate-recursion inventory. It binds the selected hepta-system status read-only tool path across:

- ToolRegistry lookup preview
- internal status payload projection
- ledger and approval preflight
- operator approval packet preview
- in-memory result receipt projection

The non-selected app connector candidate remains preflight-only. This preserves
the existing dogfood plugin fixture shape while avoiding a broad tool runtime
cutover.

## Source Reports

The local report consumes the following existing sources:

- `scripts/hepta-systems-workgraph-legacy-gate-recursion-inventory-readback-report.sh`
- `scripts/hepta-systems-hepta-system-status-internal-read-only-invocation-report.sh`
- `scripts/hepta-systems-hepta-system-status-operator-approval-protocol-report.sh`
- `scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh`
- `scripts/hepta-systems-tool-invocation-ledger-approval-preflight-report.sh`
- `scripts/hepta-systems-tool-invocation-receipt-projection-report.sh`

The Rust read model lives in
`codex-rs/hepta-runtime/src/hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback.rs`.

## Readback Contract

The report is ready-blocked when all of these hold:

- 2 candidates are visible from the hepta-system status tool fixture.
- 1 selected minimal path is bound to the MCP status candidate.
- 1 non-selected app connector candidate is kept preflight-only.
- The selected path has 4 stages: lookup preview, internal status payload,
  ledger/approval preflight, and result receipt projection.
- Registry lookup preview, ledger preview, approval preflight, approval packet
  preview, and receipt projection are each required exactly once for the
  selected path.
- Operator protocol keeps 3 explicit-accept steps and only projects
  non-acceptance/readback receipts in memory.

## Closed Boundary

Closed boundary: no tool write, ToolRegistry registration, registry mutation, ledger write, approval request, approval acceptance, receipt persistence, workflow event-log write, SQLite write, external network, credential read, provider/model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Next Step

The next migration step is
`hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback`.
That should lift the hepta-system plugin fixture into a canonical manifest,
permission, activation, tool-policy, version, signature, and trust readback
contract without installing the plugin, mutating cache, or enabling live
activation.
