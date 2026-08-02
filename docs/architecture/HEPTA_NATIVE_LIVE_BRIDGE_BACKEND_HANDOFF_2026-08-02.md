# Hepta Native Live Bridge Backend Handoff

Date: 2026-08-02
Owner: hepta-ui
Status: blocked on authoritative backend envelope and authenticated session binding

Machine markers:

- `HEPTA_NATIVE_LIVE_BRIDGE_BACKEND_HANDOFF_VERSION:1`
- `HEPTA_NATIVE_LIVE_BRIDGE_CANONICAL_ENDPOINT:GET /api/hepta-native-bridge/v1/snapshot`
- `HEPTA_NATIVE_LIVE_BRIDGE_LOOPBACK_ONLY:true`
- `HEPTA_NATIVE_LIVE_BRIDGE_EXPLICIT_OPT_IN_REQUIRED:true`
- `HEPTA_NATIVE_LIVE_BRIDGE_MATRIX_LOGIN_REQUIRED:true`
- `HEPTA_NATIVE_LIVE_BRIDGE_MUTATIONS_ENABLED:false`
- `HEPTA_NATIVE_LIVE_BRIDGE_CURRENT_READY:false`
- `HEPTA_NATIVE_LIVE_BRIDGE_BOUNDARY:no-runtime-gateway-auth-mutation-implemented-by-ui-lane`

## Decision

The current Hepta GET endpoints cannot be mapped honestly into
`BridgeSnapshot`. The Native production adapter therefore remains disabled.
No catalog, fixture, aggregate readiness report, Matrix event, or transcript
search result may be promoted as task/tool/approval truth.

The UI lane has landed the strict preflight policy, a machine-readable backend
contract, an envelope validator, and a fail-closed blocker receipt. The backend
lane must provide the canonical envelope and authentication binding before the
UI lane can construct a read-only transport.

## Existing Endpoint Audit

| Endpoint | Actual source shape | Why it cannot become `BridgeSnapshot` |
| --- | --- | --- |
| `GET /api/operator-snapshot` | Aggregate health, route parity, gateway replacement and Telegram readiness | No stable record ids, revisions, cursor, Hepta session/correlation binding, or per-record provenance |
| `GET /api/session-activity` | Redacted filesystem session inventory (`NativeSessionsResponse`) | Session filenames are inventory, not authoritative task/tool/approval records; no outer correlation/revision/provenance envelope |
| `GET /api/task/<task_id>` | Redacted transcript search (`NativeTaskArtifactResponse`) | The route deliberately hides the task id and reports matching lines; it is not a structured task registry |
| `GET /api/approvals` | Guarded POST route catalog with `pending_approval_count = 0` | It does not read an authoritative pending-approval store or return approval identities/state |
| `GET /api/activity` | Redacted transcript event-type previews (`NativeEventsResponse`) | Preview rows lack stable id, revision, correlation, cursor, and canonical provenance |
| `GET /api/gateway-runtime` | Gateway readiness, route status and migration inventory | It contains no task/tool/approval/activity collections and no request-bound session/correlation envelope |

The source audit is encoded in
`apps/hepta-native/hepta-live-bridge-backend-contract-v1.json`. The gate checks
that these classifications remain explicit instead of inferring truth from
similar field names.

## Minimum Backend Contract

The backend-owned route is:

```text
GET /api/hepta-native-bridge/v1/snapshot
```

Requirements:

1. Bind only to loopback. The Native client rejects non-loopback URLs,
   credential-bearing URLs, query strings, fragments, and every non-canonical
   path.
2. Authenticate the caller and bind the request to the already-authenticated
   in-process Native/Matrix session. An environment variable is not login
   proof.
3. Accept caller-issued session and correlation identifiers through a backend
   contract that does not leak credentials into URLs or logs, and echo both in
   the authoritative `BridgeUpdate` metadata.
4. Return exactly one schema-v1 `BridgeUpdate` whose update type is `snapshot`.
   Every record must have a stable id, revision, timestamp, authoritative
   origin, presenter-safe redaction status, non-empty provenance, matching
   session, and matching correlation.
5. Set `Cache-Control: no-store`; never return raw source payloads or secrets.
6. The route is GET/read-only. It must not invoke a provider, send a channel
   message, write a cursor, approve/reject/cancel work, or mutate gateway state.
7. V1 capabilities are snapshot-only. Subscribe, prepare, confirm, reject and
   cancel stay false even after login and opt-in.

The exact machine-readable fields and promotion checklist live in
`apps/hepta-native/hepta-live-bridge-backend-contract-v1.json`.

## Native Post-login Lifecycle

`LiveBridgePreflight` is evaluated from in-process state:

1. Before `LoginAction::LoginSuccess`, the bridge is ineligible.
2. After login, the bridge is still ineligible until the user explicitly opts
   in, the endpoint is canonical loopback, the host authenticates and binds the
   Hepta session, and the authoritative snapshot contract is negotiated.
3. Passing the policy only permits construction of a snapshot-only adapter. It
   does not perform a request and does not grant write authority.
4. `LoginFailure` and logout must drop the transport and session binding before
   returning to the login screen.

The production `HeptaBridge` remains backed by `DisabledBridgeAdapter` until the
backend contract exists and an actual live integration test can exercise this
lifecycle.

## Live Receipt Contract

A future live receipt must be generated by the Native process from an actual
request and response. It must include:

- receipt schema/version and source commit;
- exact canonical endpoint and method `GET`;
- HTTP status and content type;
- a credential-free canonical request-descriptor SHA-256;
- raw response byte length and SHA-256;
- expected and observed session/correlation match booleans;
- authoritative origin, redaction and provenance validation results;
- `matrix_session_authenticated=true` and `explicit_user_opt_in=true`, derived
  from in-process state;
- all mutation capabilities false;
- provider invocation, channel delivery, cursor write, gateway mutation, and
  external mutation false.

No source-only gate or synthetic response is a live receipt. Until an actual
request passes, `hepta_live_bridge_ready` stays false.

## Verification

```bash
scripts/hepta-native-live-bridge-contract-gate.sh --output /tmp/hepta-native-live-bridge-blocker.json
scripts/hepta-native-live-bridge-contract-gate-self-test.sh
cargo test --manifest-path apps/hepta-native/Cargo.toml --lib hepta_bridge -- --nocapture
```

The contract gate succeeds only when the blocked state is represented
truthfully. Its output is a structured blocker receipt, not a readiness receipt.
