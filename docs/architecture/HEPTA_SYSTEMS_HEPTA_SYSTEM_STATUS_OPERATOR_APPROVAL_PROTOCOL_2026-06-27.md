# Hepta Systems Hepta-System Status Operator Approval Protocol

Date: 2026-06-27

## Intent

Phase 9 defines the operator approval protocol for the Phase 8 internal
read-only `hepta-system status` invocation. It does not send an approval
request, accept an approval, record evidence, write the approval broker, persist
a receipt, or promote live execution.

The surface exists to make the next cutover protocol inspectable before any
mutable approval path is connected.

## Protocol

The protocol consumes:

- `scripts/hepta-systems-hepta-system-status-internal-read-only-invocation-report.sh`
- selected candidate
  `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- source payload fingerprint
  `hepta-system-status.internal-read-only.v1.e2e4.fixture9.live0`

Gate phrase: nonce/session binding.

It projects one approval packet:

- approval subject:
  `approval-subject:hepta-system/status/internal-read-only`
- approval packet:
  `approval-packet.hepta-system-status.internal-read-only.v1`
- nonce:
  `approval-nonce.hepta-system-status.internal-read-only.v1`
- session binding:
  `operator-session-binding.hepta-local.explicit-accept-required.v1`
- packet route:
  `approval://hepta-system/status/internal-read-only/v1`
- non-acceptance receipt projection:
  `receipt://hepta-system/status/internal-read-only/operator-approval/non-acceptance`

The three protocol steps are:

- `nonce_session_binding_preflight`
- `approval_packet_preview`
- `non_acceptance_receipt_projection`

All three steps require explicit operator acceptance, but acceptance is not
allowed or recorded by this phase.

## Boundary

This is a local read-model/protocol preview only. There is no approval request,
approval acceptance, approval recording, approval broker write, evidence
recording, credential read, external network access, ledger write, transport
mutation, receipt persistence, workflow event-log write, SQLite write, Native
POST mutation, channel send, or live execution.

Auto-approval is explicitly disabled. The approval broker file is not opened or
written.

Closed approval boundary: no approval request, approval acceptance, approval recording, approval broker write, evidence recording, credential read, external network access, ledger write, transport mutation, receipt persistence, workflow event-log write, SQLite write, Native POST mutation, channel send, or live execution.

## Gate

Local gate:

```bash
scripts/hepta-systems-hepta-system-status-operator-approval-protocol-gate.sh
```

The gate verifies:

- Phase 8 internal read-only invocation is ready.
- The packet binds the source payload, nonce, and operator session.
- The packet requires explicit operator acceptance.
- No approval request, acceptance, recording, broker write, evidence recording,
  persistence, credential read, transport mutation, or live execution is
  allowed.
- Targeted hepta-runtime Rust tests pass.

## Next Step

Next migration step:
`phase10_controlled_canary_readiness_plan_without_gateway_native_telegram_or_live_activation`.

Phase 10 should plan the controlled canary boundary while keeping Gateway/Auth,
Native POST, Telegram/channel transport, persistence, package/release writes,
Public GA, and live activation closed.
