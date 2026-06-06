# Hepta Memory / Intelligence / KG Trusted Operator Acceptance Record Negative Fixture Matrix Gate

This gate is the report-only negative fixture matrix after the trusted operator
acceptance record intake validator. It does not supply a real operator approval
and does not arm the canary. It proves that pseudo-complete or malformed trusted
operator records remain blocked.

## Purpose

The intake validator declares the trusted operator record fields needed before
any canary dispatch or live execution can be considered. This gate adds invalid
record fixtures around that intake boundary so the next positive path cannot
accept a record just because it has a plausible shape.

Covered fixture families:

- missing operator identity;
- signature hash mismatch;
- stale signed timestamp;
- route or namespace scope mismatch;
- value scoreboard, readback, or audit receipt hash mismatch;
- idempotency nonce replay;
- missing rollback plan or kill switch;
- dispatch budget above the one-request canary limit;
- secret or credential injection attempt.

## Output Contract

The report must show:

- negative fixture count: 12;
- blocked negative fixture count: 12;
- accepted/recorded/persisted/delivered negative fixture counts: 0;
- dispatch/context/provider/model/memory/KG/live authorization counts: 0;
- canary harness armed/executable/live/executed: false.

The fixtures are intentionally shaped as operator records, but each one violates
at least one authority-family rule. A future canary arm path must reject every
fixture before it accepts any trusted operator record.

## Non-Effects

This gate is report-only:

- no trusted operator record is recorded or persisted;
- no controlled request is dispatched or executed;
- no Memory write;
- no external KG read or live KG write;
- no prompt/context injection;
- no provider/model invocation;
- no secret or credential read;
- no channel send;
- no install, restart, or active binary mutation.
