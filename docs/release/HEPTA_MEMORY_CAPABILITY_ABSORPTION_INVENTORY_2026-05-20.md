# Hepta Memory/Capability Absorption Inventory

Date: 2026-05-20
Scope: old standalone Hepta memory/capability/absorption ops modules versus current `hepta-codex`
Status: read-only absorption gap inventory landed; registry/memory/plugin writes disabled

## Summary

The local tooling/content planning slice left memory, capability, plugin,
runtime absorption, coding-agent, search-provider, and skill-workshop surfaces
as the next safe migration target. This slice exposes those old modules only as
read-only absorption or gap entries:

- `/api/hepta-memory-capability-absorption-inventory`
- source-command equivalent: `/hepta-memory-capability-absorption-inventory --json`
- validation script: `scripts/hepta-codex-memory-capability-inventory.sh`

The route does not mutate memory stores, capability registries, plugin
registries, skill files, Gateway/native POST state, spawn coding agents, query
search providers, read credentials, invoke models/providers, read or send
channels, read or write files, or perform external sends.

## Inventory Counts

- old memory/capability/absorption ops files covered: `14`
- surfaces inventoried: `14`
- absorbed or represented surfaces: `9`
- gap-report-ready surfaces: `14`
- live mutation surfaces enabled: `0`
- current `hepta-codex` scripts: `11`
- current native gateway source commands: `58`
- Control UI route parity after this slice: `58/58`, missing `0`

## Files Covered

- `capability_surface_ops.rs`
- `hepta_p0_absorption_ops.rs`
- `hepta_p1_absorption_ops.rs`
- `hepta_runtime_absorption_ops.rs`
- `memory_rem_ops.rs`
- `memory_system_ops.rs`
- `memory_tools_ops.rs`
- `native_coding_agent_ops.rs`
- `native_plugin_metadata_ops.rs`
- `native_residual_runtime_ops.rs`
- `native_search_provider_ops.rs`
- `plugin_migration_ops.rs`
- `runtime_capability_matrix_ops.rs`
- `skill_workshop_ops.rs`

## Boundary

This is a read-only gap report. It intentionally keeps old CLI invocation
compatibility unclaimed and keeps memory writes, registry writes, plugin
migration writes, coding-agent spawn, search-provider live query, and
skill-workshop writes disabled.

## Remaining Blockers

- memory store mutation is not operator-approved
- capability registry mutation is not operator-approved
- plugin registry mutation is not operator-approved
- coding-agent spawn is not operator-approved
- search-provider live query is not operator-approved
- skill-workshop write is not operator-approved
- old memory/capability CLI invocation compatibility is not claimed

## Safe Next Slice

Port remaining external release and hardening scripts as local-only status gates
before any live handoff, external push, channel delivery, process execution,
filesystem mutation, credentialed provider call, or native POST real mutation.
