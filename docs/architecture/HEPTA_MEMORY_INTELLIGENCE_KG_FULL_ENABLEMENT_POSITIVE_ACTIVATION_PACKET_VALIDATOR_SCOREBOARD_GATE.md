# Hepta Memory/Intelligence/KG Positive Activation Packet Validator Scoreboard Gate

## Purpose

`scripts/hepta-memory-intelligence-kg-full-enablement-positive-activation-packet-validator-scoreboard-gate.sh`
turns the positive activation packet dry-run scaffold into a compact authority
scoreboard for the future canary live harness.

The gate is intentionally report-only. It validates that the Phase A-E packet
shapes exist, then records the positive authority material still missing before
any live activation can be attempted.

## Source

The gate captures:

- `hepta-memory-intelligence-kg-full-enablement-positive-activation-packet-dry-run-scaffold-gate.sh`

The captured scaffold must already prove:

- 5 source reports are represented.
- 14/14 memory surfaces are absorbed or represented.
- 5 activation phases are declared and blocked.
- 0 phases are accepted.
- 0 phases are live-enabled.
- No context injection, provider/model invocation, memory write, KG read/write,
  credential read, install, restart, or active binary mutation occurred.

## Scoreboard

For each of the A-E phases, the validator emits 8 required authority families:

- Operator approval record
- Single route and namespace scope
- Rollback and kill switch
- Redaction and context bounds
- Audit trail
- Readback receipt
- Idempotency nonce
- Retention, export, and observability

That yields 40 scoreboard items. In this stage every item is declared as a
shape but remains `missing`; therefore the scoreboard is ready but blocked.

## Canary Harness Bridge

The gate also declares 12 canary harness preconditions:

- Explicit operator approval
- Accepted activation packet digest
- Single route binding
- Single namespace binding
- 0/1 controlled request budget
- Rollback kill switch acceptance
- Redaction policy acceptance
- Readback receipt acceptance
- Idempotency nonce acceptance
- Audit and retention acceptance
- Provider/model secret-use policy acceptance
- Phase-specific memory/KG write policy acceptance

The next slice may build a canary live harness scaffold from these
preconditions. This gate does not perform that canary and does not grant live
authority.

## Safety Invariants

This gate must always keep the following false:

- `operator_approval_accepted`
- `activation_packet_accepted`
- `context_injection_performed`
- `provider_invoked`
- `model_invoked`
- `memory_store_mutated`
- `external_kg_adapter_read_performed`
- `live_kg_write_performed`
- `credential_read`
- `secret_file_read`
- `install_performed`
- `service_restarted`
- `active_binary_mutated`

The gate is a positive readiness validator, not a live enablement gate.
