# Hepta Systems WorkGraph Legacy Gate Recursion Inventory Readback

Date: 2026-06-30

## Scope

This note records the `hepta_systems_workgraph_legacy_gate_recursion_inventory_readback`
surface. It consumes the gate-recursion lean contract and inventories the remaining
legacy WorkGraph rerun-preview gate chains that still carry large
`required_prior_gate_count` ladders.

The readback does not execute those legacy gates. It records their route, family,
and prior-gate burden so the next migrations can replace recursive source-gate
validation with source-report smoke plus targeted Rust test contracts.

## Inventory

- Surface: `hepta_systems_workgraph_legacy_gate_recursion_inventory_readback`.
- Gate: `hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_gate`.
- Schema: `hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_v1`.
- Source: `hepta_systems_gate_recursion_lean_contract_readback`.
- Entries: 8 legacy WorkGraph rerun-preview gates.
- Total `required_prior_gate_count`: 663.
- Maximum single route burden: 116.

The inventory groups the legacy routes into:

- terminal no-cutover receipt acknowledgement chain
- replay-idempotency chain
- closeout receipt chain
- runtime write boundary chain
- operator review packet chain

## Convergence Update

On 2026-07-24, all 8 inventoried gate bodies moved to the shared
`legacy_workgraph_projection_v1` runner contract. The legacy report payloads
remain the compatibility projection source, but recursive dependencies now use
an invocation-scoped bounded JSON DAG cache. Each consolidated gate validates
the report identity, unique prior-gate set, source probes, closed live-execution
boundary, and zero side effects before running the route's targeted Rust tests.

The 8 retired gate payload hashes remain in
`scripts/hepta-gate-pair-specs-v1.json` as parity evidence. Before deletion, all
8 replacement gates passed the complete legacy assertions, and the terminal
receipt acknowledgement report matched its pre-migration JSON byte for byte.
No report semantics or live surfaces were added.

## Contract

Every inventoried legacy route must be treated as a migration target. New or
touched gates in these families should use a source-report smoke plus targeted
Rust test contract instead of invoking a full upstream gate ladder. The inventory
keeps the routes queryable and diffable while preserving the existing read-only
boundary.

There is no runtime writes, event-log writes, SQLite writes, workflow execution,
replay execution, source report semantic changes, matrix cache writes, compact
cache persistence, provider/model invocation, Gateway/Auth mutation, Native POST
mutation, Telegram transport mutation, channel send, package/release, canary
activation, Public GA promotion, or live execution.

Closed boundary: no runtime writes, event-log writes, SQLite writes, workflow execution, replay execution, source report semantic changes, matrix cache writes, compact cache persistence, provider/model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Next Step

After the recursion inventory, the systems lane should move to
`hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback`.
That next step should pick the narrowest read-only status tool path and project
the minimal invocation, ledger, approval, and receipt contract without opening
tool writes, approval requests, receipt persistence, external network access,
credentials, or live execution.
