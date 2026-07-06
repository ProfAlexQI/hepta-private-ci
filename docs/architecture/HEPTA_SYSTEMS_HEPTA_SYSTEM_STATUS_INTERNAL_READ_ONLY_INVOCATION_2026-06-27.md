# Hepta Systems Hepta-System Status Internal Read-Only Invocation

Date: 2026-06-27

## Intent

Phase 8 opens the thinnest internal read-only `hepta-system status` invocation path without promoting the generic ToolRegistry dispatch switch or any live external tool execution.

The surface materializes an internal status payload from existing read-only evidence:

- Phase 4 `hepta-system` status read-only E2E chain.
- Phase 2 ToolRegistry read-only dispatch preflight.
- Phase 7 workflow durable-store test-only append fixture.

## Boundary

This is an internal read-model invocation only. It projects a local status payload and in-memory receipt shape for one selected candidate:

- Selected: `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`.
- Not selected: `preview:connector:hepta-system@hepta-local:hepta_system_local_app`, kept preflight-only.

The status payload route is `internal://hepta-system/status/read-only`.

There is no credential read, external network access, external tool invocation, ToolRegistry live switch, ledger write, approval request, approval acceptance, receipt persistence, workflow event-log write, SQLite write, Native POST mutation, channel send, or live execution.

## Gate

Local gate:

```bash
scripts/hepta-systems-hepta-system-status-internal-read-only-invocation-gate.sh
```

The gate verifies:

- Phase 4 E2E is ready while `ready_for_invocation=false`.
- ToolRegistry dispatch preflight still has two ready candidates with invocation, ledger, approval, and receipt writes disabled.
- Phase 7 test-only fixture covers all 9 workflow event contracts while runtime event-log and SQLite writes remain disabled.
- Exactly one internal status invocation entry is materialized.
- The app connector candidate remains preflight-only.
- All side-effect flags remain false.

## Next Step

Next migration step: `phase9_operator_approval_protocol_nonce_session_binding_without_auto_acceptance`.

Phase 9 should define the operator approval protocol and receipt shape with nonce/session binding, but still avoid auto-approval, evidence recording, credential reads, transport mutation, persistence, package/release writes, Public GA promotion, and live execution.
