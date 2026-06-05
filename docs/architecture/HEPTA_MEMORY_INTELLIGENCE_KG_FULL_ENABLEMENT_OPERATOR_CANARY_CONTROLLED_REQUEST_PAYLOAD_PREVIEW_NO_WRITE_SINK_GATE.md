# Hepta Memory / Intelligence / KG Controlled Request Payload Preview No-Write Sink Gate

This gate is the report-only bridge between dispatch envelope preview and any
future controlled canary request payload.

It does not accept a dispatch envelope, write a payload file, write a sink,
dispatch a request, execute a request, attach context, invoke a provider/model,
read secrets, write Memory/KG state, or mutate/restart the active service.

## Source

The gate captures and validates:

- `hepta-memory-intelligence-kg-full-enablement-operator-canary-dispatch-envelope-preview-gate.sh`

The source dispatch envelope preview must remain blocked:

- 5 dispatch envelopes are shaped and preview-ready.
- 0 dispatch envelopes are accepted.
- 0 dispatch preconditions are satisfied.
- controlled request dispatch-ready, dispatch-allowed, dispatched, and executed
  counts remain zero.
- request payload materialized and persisted counts remain zero.

## Payload Preview Shape

The gate emits one controlled request payload preview shape per A-E stage. Each
preview declares the fields a future controlled request payload will need:

- route and namespace binding
- request method and budget
- redacted payload preview shape
- payload hash shape
- no-write sink contract
- redaction proof shape
- audit entry shape
- readback receipt shape
- idempotency nonce shape
- rollback kill-switch shape

The preview is materialized only as part of the stdout JSON report. It is not a
runtime payload, filesystem artifact, workspace artifact, external sink write,
dispatch request, or execution input.

## No-Write Sink Contract

Every payload preview is bound to a no-write sink contract:

- filesystem writes are not allowed
- workspace writes are not allowed
- external writes are not allowed
- dispatch is not allowed
- execution is not allowed

This separates "we can inspect the shape of the request body" from "we have
created a payload that can be dispatched."

## Safety Invariants

The gate asserts that all live side effects remain false:

- no payload preview record or persistence
- no request payload materialization or persistence
- no no-write sink write
- no controlled request dispatch or execution
- no context attachment or injection
- no provider/model invocation
- no Memory write
- no external KG read/write
- no credential or secret read
- no channel/external send
- no install/restart/active binary mutation

## Next Required Step

A future canary still requires accepted operator authority, accepted dispatch
envelope, accepted payload hash/readback/audit hooks, and explicit dispatch
authorization. This gate only makes the bounded request-payload shape inspectable
without creating anything executable.
