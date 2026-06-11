# Hepta Memory/Intelligence/KG Runtime Provider-Router Operator-Approved Shadow Context Activation Execution Gate

This gate introduces the first runtime-owned execution surface for Memory/Intelligence/KG provider-router activation after the report-only staging chain.

The execution is deliberately scoped to `operator_approved_shadow_context_attachment_execution`:

- It consumes a previously recorded memory context activation handoff.
- It requires the release gate, explicit operator release approval, 0ppm canary traffic, canary telemetry readiness, an armed rollback kill switch, and a post-activation watchdog/soak plan.
- It records a local runtime router execution and marks the handoff as shadow context attached.
- It does not invoke a provider or model, read credentials, call external networks, write KG, send channels, publish releases, restart services, or mutate the active binary.

## Guarded Runtime Surface

The Rust runtime surface lives in `codex-rs/hepta-runtime/src/model_provider_router.rs`:

- `ModelProviderMemoryContextActivationExecutionRecord`
- `ModelProviderMemoryContextActivationExecutionInput`
- `ModelProviderMemoryContextActivationExecutionReport`
- `ModelProviderRouter::execute_memory_context_activation_shadow`

The method refuses execution unless all of these are true:

- `operator_confirmed`
- `policy_decision` contains an allow/approved decision
- `release_gate_ready`
- `operator_release_approved`
- `kill_switch_active == false`
- `canary_telemetry_ready`
- `rollback_kill_switch_armed`
- `post_activation_watchdog_soak_plan_ready`
- source handoff exists, is approved, and has `traffic_percent_ppm == 0`

## Allowed Effect

The only allowed effect is a local runtime-router shadow activation record:

- `feature_flag_mutated_by_adapter = true`
- `context_attached_to_live_prompt = true`
- readback evidence is appended

This is not provider execution. The following stay false:

- `provider_invoked_by_adapter`
- `auth_secret_read_by_adapter`
- `external_network_call_performed`
- `live_kg_write_performed`

## Gate Script

The executable gate is:

`scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-gate.sh`

It verifies the live readiness route is still ready and non-mutating, checks the source patterns, and runs focused Rust tests for:

- successful operator-approved shadow context activation execution
- denial when release gate, telemetry, rollback kill switch, or watchdog/soak plan are missing
- gated adapter E2E consumption without external provider/auth/KG effects
