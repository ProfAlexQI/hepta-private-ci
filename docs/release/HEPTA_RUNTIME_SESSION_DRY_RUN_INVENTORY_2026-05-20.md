# Hepta Runtime/Session Dry-Run Inventory

Date: 2026-05-20
Scope: old standalone Hepta runtime, task, session, gateway, diagnostics, and admin ops modules versus current `hepta-codex`
Status: read-only dry-run inventory landed; no live runtime mutation enabled

## Summary

The CLI breadth and provider/search inventories left runtime/task/session ops as
the next safe migration slice. This slice exposes those old modules as local
dry-run migration plans:

- `/api/hepta-runtime-session-dry-run-inventory`
- source-command equivalent: `/hepta-runtime-session-dry-run-inventory --json`
- validation script: `scripts/hepta-codex-runtime-session-dry-run-inventory.sh`

The route is intentionally side-effect-free. It does not mutate the task
registry, write session state, enqueue gateway/runtime events, enqueue hooks,
spawn processes, invoke providers/models, read credentials, read Telegram, send
messages, activate native POST real handlers, push telemetry, or write files.

## Inventory Counts

- old runtime/admin ops files covered: `12`
- dry-run surfaces exposed: `12`
- planner-ready surfaces: `12`
- live mutation surfaces enabled: `0`
- current `hepta-codex` scripts: `8`
- current native gateway source commands: `55`
- Control UI route parity after this slice: `55/55`, missing `0`

## Files Covered

- `commitment_ops.rs`
- `diagnostics_otel_ops.rs`
- `diagnostics_prometheus_ops.rs`
- `gateway_ops.rs`
- `heartbeat_ops.rs`
- `operator_admin_ops.rs`
- `runtime_control_ops.rs`
- `runtime_event_ops.rs`
- `session_orchestration_ops.rs`
- `task_provenance_ops.rs`
- `thread_binding_ops.rs`
- `update_plan_ops.rs`

## Boundary

The inventory provides a safe next-mode for each old module. It does not claim
old CLI invocation compatibility and does not execute old commands. Live task,
session, gateway, hook, process, telemetry, channel, provider, or native POST
effects remain blocked until an explicit operator request exists.

## Remaining Blockers

- old runtime CLI invocation compatibility is not claimed
- task registry live mutation is not operator-approved
- session store live mutation is not operator-approved
- gateway event enqueue is not operator-approved
- external telemetry push is not operator-approved

## Safe Next Slice

Continue with channel adapters as disabled live-gated status reports. After
that, inventory local tooling/content surfaces before any process execution,
filesystem mutation, network read, or channel delivery smoke.
