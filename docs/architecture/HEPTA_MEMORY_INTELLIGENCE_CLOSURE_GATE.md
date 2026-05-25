# Hepta Memory / Intelligence Closure Gate

Date: 2026-05-25

This gate records the current boundary between the active Hepta runtime stack,
the core contract crate, and the memory/intelligence subsystems.

## Current Contract

The active service stack consumes memory and intelligence through the runtime
path:

```text
hepta-cli -> hepta-gateway -> hepta-runtime -> hepta-intelligence
                                      \-> hepta-kernel
                                      \-> hepta-memory
                                      \-> hepta-plugins
```

The `hepta-core` crate intentionally does not directly depend on
`hepta-memory`, `hepta-intelligence`, `hepta-runtime`, or `hepta-kernel`. It
remains the contract, report, and governance layer. This avoids core bloat and
dependency cycles while still allowing the active service stack to consume the
runtime subsystems.

## Live Memory Capability State

The live route `/api/hepta-memory-capability-absorption-inventory` is the source
of truth for the current memory/capability closure status.

Current required shape:

- `surface_count = 14`
- `absorbed_or_represented_count = 9`
- `gap_report_ready_count = 14`
- `live_mutation_enabled_count = 0`

The five remaining gap-only surfaces are:

- `memory-rem`
- `memory-tools`
- `native-residual-runtime`
- `plugin-migration`
- `skill-workshop`

## Safety Boundary

The closure gate is read-only. It must not:

- mutate the memory store
- mutate the capability registry
- mutate the plugin registry
- spawn a coding agent
- run a live search provider query
- write a skill workshop artifact
- invoke a provider or model
- perform channel delivery
- mutate the gateway

## Next Slices

The next work should close the remaining five gap-only surfaces one at a time.
Each slice should introduce a bounded, operator-approved status or plan contract
before enabling any live mutation.
