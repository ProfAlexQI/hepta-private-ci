# Hepta Memory / Intelligence / KG Operator Canary Harness Redacted Payload Preview No-Materialization Gate

This gate is the report-only layer after the canary harness no-dispatch
scoreboard. It does not arm the harness, materialize a request payload,
dispatch a controlled request, persist readback/audit receipts, or execute live
work.

## Purpose

The no-dispatch scoreboard fixes a single route, single namespace, dispatch
budget one, readback hash, audit hash, idempotency nonce, rollback/kill switch,
and absent-secret boundary. This gate binds that shape to one synthetic
redacted payload preview hash.

The payload preview is not a runtime payload. It is a deterministic stdout JSON
shape used to check that the future canary request can be reviewed without
creating a file, reading a raw payload, injecting context, or invoking a
provider.

## Output Contract

The report must show:

- source harness scoreboard entries: 9;
- accepted scoreboard entries: 0;
- payload preview count: 1;
- payload preview hash shape declared: 1;
- payload preview accepted/recorded/persisted/delivered: 0;
- request payload materialization allowed/materialized/persisted/file-written:
  0;
- raw payload inspected: 0;
- readback and audit receipt previews declared, but persisted: 0;
- controlled request dispatch budget declared as 1 but not accepted or
  consumed;
- controlled request dispatch/execution: 0;
- context injection, provider/model invocation, Memory writes, external KG
  reads, live KG writes, channel sends, credential reads, and restarts: 0;
- seven negative fixtures remain blocked.

## Non-Effects

This gate is stdout-only and report-only:

- no payload preview is accepted as authority;
- no raw payload is inspected;
- no request payload file is written;
- no controlled request is dispatched or executed;
- no readback or audit receipt is persisted;
- no Memory or KG state is read or written;
- no prompt/context injection happens;
- no provider/model is invoked;
- no secret or credential is read;
- no install, restart, or active binary mutation occurs.
