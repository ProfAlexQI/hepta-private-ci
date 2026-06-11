# Hepta Memory/Intelligence/KG Runtime Provider-Router Shadow Context Activation Execution Readiness Route

This route exposes the operator-approved shadow context activation execution surface as a native Hepta gateway readiness report.

It is intentionally report-only:

- It confirms the runtime-owned `execute_memory_context_activation_shadow` surface exists.
- It confirms the route is wired into the native gateway route/source-command inventory.
- It depends on the existing Memory/Intelligence/KG runtime readiness route.
- It does not invoke the shadow execution method.
- It does not install or restart the live 7373 service.
- It does not invoke providers or models, read credentials, call external networks, write KG, write Memory, send channels, publish releases, or mutate the active binary.

## Route

Endpoint:

`/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-readiness`

Source command:

`/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-readiness --json`

Compatibility mode:

`native_runtime_provider_router_shadow_context_activation_execution_readiness_route_source_only`

## Readiness Dependency

The route is ready only when the existing runtime readiness route remains ready:

- `full_enablement_activation_readiness_ready = true`
- `live_mutation_enabled_count = 0`
- `current_live_enabled_lane_count = 0`
- provider invocation is false
- model invocation is false
- credential read is false
- live KG write is false
- Memory store mutation is false

## Execution Boundary

The route is not an activation command. These fields must remain false:

- `execution_invoked_by_report_route`
- `live_route_exposes_activation_command`
- `provider_invocation_performed`
- `model_invocation_performed`
- `auth_secret_read_performed`
- `credential_read_performed`
- `external_network_call_performed`
- `live_kg_write_performed`

The side-effect payload also keeps live service and release surfaces false:

- `live_7373_router_mutated_by_report_route`
- `feature_flag_mutated_in_live_7373_by_report_route`
- `context_attached_to_live_7373_prompt_by_report_route`
- `service_restarted`
- `active_binary_mutated`
- `release_artifact_written`
- `public_release_claimed`

## Gate

The executable gate is:

`scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-readiness-route-gate.sh`

The gate:

- checks the current live runtime readiness route for ready/no-mutation state
- verifies native gateway source patterns for the new endpoint, source command, report function, and no-execution fields
- verifies the runtime source still contains `execute_memory_context_activation_shadow`
- runs a focused `codex-cli` native gateway test for the route
- emits JSON showing the route was source-tested while live route installation was not performed by the gate
