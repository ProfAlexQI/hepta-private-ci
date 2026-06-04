# Hepta Memory/Intelligence/KG Canary Live Harness Scaffold Gate

## Purpose

`scripts/hepta-memory-intelligence-kg-full-enablement-canary-live-harness-scaffold-gate.sh`
declares the future canary harness shape for Memory, Hepta Intelligence, and
KG activation.

This is still a report-only gate. It does not arm or execute a canary, bind a
live route, attach prompt context, invoke a provider/model, read credentials,
write memory, write KG state, restart services, or mutate the active binary.

## Source

The gate captures:

- `hepta-memory-intelligence-kg-full-enablement-positive-activation-packet-validator-scoreboard-gate.sh`

The captured validator must already prove:

- 5 A-E activation phases are shaped and blocked.
- 8 authority families produce 40 missing scoreboard items.
- 12 canary preconditions are shaped and missing.
- `canary_harness_shape_ready=true`.
- `canary_harness_activation_ready=false`.
- `canary_harness_next_slice_performs_live_activation=false`.

## Harness Shape

The scaffold defines:

- one report-only route id
- one report-only namespace id
- one report-only rollback kill-switch id
- a 0/1 controlled request budget shape
- five canary stages mapped to Phase A-E
- twelve guard records inherited from the validator preconditions

Every stage is `blocked_not_armed`. The route and namespace are shaped but not
accepted. The request budget is shaped but not accepted. No request is
dispatched or executed.

## Guard Families

The guard records cover:

- explicit operator approval
- accepted activation packet digest
- single route binding
- single namespace binding
- 0/1 controlled request budget
- rollback kill-switch acceptance
- redaction policy acceptance
- readback receipt acceptance
- idempotency nonce acceptance
- audit and retention acceptance
- provider/model secret-use policy acceptance
- phase-specific memory/KG write policy acceptance

All guards remain missing in this scaffold.

## Safety Invariants

This gate must keep the following false:

- `canary_harness_armed`
- `canary_harness_executable`
- `canary_live_enabled`
- `canary_execution_performed`
- `context_injection_performed`
- `provider_invoked`
- `model_invoked`
- `memory_store_mutated`
- `external_kg_adapter_read_performed`
- `live_kg_write_performed`
- `credential_read`
- `secret_file_read`
- `service_restarted`
- `active_binary_mutated`

The next real activation step would require an explicit operator-approved
canary packet. This scaffold is not that packet.
