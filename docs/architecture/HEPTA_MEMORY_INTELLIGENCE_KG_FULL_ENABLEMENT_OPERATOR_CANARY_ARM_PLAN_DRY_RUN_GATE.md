# Hepta Memory/Intelligence/KG Operator Canary Arm Plan Dry-Run Gate

## Purpose

`scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-arm-plan-dry-run-gate.sh`
declares the report-only arm plan that would sit between an accepted operator
canary packet and any canary harness arm.

It does not arm the harness, execute a controlled request, attach prompt
context, invoke a provider/model, read credentials, write Memory, write KG
state, restart services, or mutate the active binary.

## Source

The gate captures:

- `hepta-memory-intelligence-kg-full-enablement-operator-canary-packet-value-fixture-scoreboard-gate.sh`

The source must already prove:

- 12/12 packet values are present only as synthetic fixtures.
- 0 packet values are trusted or accepted.
- 16/16 packet acceptance preconditions are shaped.
- 0 acceptance preconditions are satisfied.
- 5/5 canary stage bindings exist.
- 0 stages are armed, executable, executed, or live-enabled.

## Arm Plan Shape

The dry-run arm plan declares:

- one arm-plan id and schema id
- sixteen arm-plan guards derived from packet acceptance preconditions
- five stage transitions mapped to the A-E canary stages
- a 0/1 controlled request budget on each transition
- route, namespace, and rollback/kill-switch bindings inherited from the
  source canary scaffold

Every guard remains missing. Every stage transition remains
`blocked_plan_not_accepted`.

## Safety Invariants

The gate must keep these false or zero:

- `operator_canary_arm_plan_recorded`
- `operator_canary_arm_plan_persisted`
- `operator_canary_arm_plan_accepted`
- `operator_canary_packet_accepted`
- `operator_canary_packet_authorizes_canary_arm`
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

## Next Step

The next positive step is not more denial. It is to replace synthetic packet
fixtures with a trusted operator canary packet record and then allow this arm
plan to be recorded only after all guards are accepted.
