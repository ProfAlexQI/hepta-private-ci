# Hepta Systems Gate Recursion Lean Contract Readback

This note records the local readback step that turns the observed gate-cost
reduction into an explicit source-report smoke plus targeted Rust test contract.

## Scope

- Surface: `hepta_systems_gate_recursion_lean_contract_readback`.
- Source boundary: `hepta_systems_gate_recursion_cost_boundary_readback`.
- Projection: five local contract entries are exposed:
  - recovery-receipt local persistence gate already uses source-report
    invariants and one targeted Rust test;
  - recovery-window feature-gated source gate still represents an older
    recursive pattern;
  - a WorkGraph closeout receipt rerun gate remains visible as a legacy
    recursion inventory target;
  - matrix single-render and controlled-live dashboard paths consume one
    matrix render instead of recursively rendering downstream.

## Boundary

This is a readback-only contract. It does not rewrite source report semantics or
execute any upstream gate chain. New gates should validate source-report
invariants and run only their local targeted Rust test unless a later, explicit
lane-owned gate broadens the scope.

The closed boundary is explicit: no matrix cache write, compact cache persistence, source report semantic change, recursive source-gate invocation for new gates, workflow execution, replay execution, event-log write, SQLite write, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

## Current Next Gate

The next local gate is
`hepta_systems_workgraph_legacy_gate_recursion_inventory_readback`.

That gate should inventory the remaining WorkGraph/report routes that still
carry legacy recursion risk before more suffix or closeout paths are added. It
should remain readback-only and must not mutate git, cache state, source report
semantics, runtime persistence, transports, release state, or live execution.
