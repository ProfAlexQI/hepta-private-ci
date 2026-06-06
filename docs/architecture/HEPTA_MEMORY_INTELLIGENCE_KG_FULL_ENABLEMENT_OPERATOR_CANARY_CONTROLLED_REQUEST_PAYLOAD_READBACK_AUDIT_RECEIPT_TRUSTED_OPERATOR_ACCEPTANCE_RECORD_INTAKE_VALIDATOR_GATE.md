# Hepta Memory / Intelligence / KG Trusted Operator Acceptance Record Intake Validator Gate

This gate is the report-only intake validator after the trusted operator
acceptance record scaffold. It does not supply or infer a real operator
approval. It declares what a real trusted operator acceptance record must carry
before any canary dispatch or live execution can be armed.

## Purpose

The scaffold gate produces five acceptance record candidates, all hash-bound to
the canary readback/audit/receipt value scoreboard. Those candidates are only
shapes. This gate turns them into an explicit intake contract:

- one intake validator per canary stage;
- sixteen required operator-supplied fields per stage;
- eighty required fields across all five stages;
- no trusted, present, accepted, recorded, persisted, or delivered values until
  a real operator record is supplied.

The gate is a contract for future authority, not authority itself.

## Required Operator Fields

Each canary stage requires:

- operator identity;
- operator signature hash;
- signed timestamp;
- approval scope;
- route and namespace;
- value-scoreboard and scaffold hashes;
- redaction policy hash;
- readback and audit receipt hashes;
- idempotency nonce;
- rollback plan and kill switch;
- dispatch budget;
- live execution bounds.

## Output Contract

The report must show:

- intake record count: 5;
- required field count: 80;
- present field count: 0;
- trusted field count: 0;
- accepted field count: 0;
- record accepted/recorded/persisted/delivered counts: 0;
- dispatch/context/provider/model/memory/KG/live authorization counts: 0.

If a future gate supplies real values, this validator must be extended with
source identity, signature, timestamp, scope, and kill-switch checks before any
canary dispatch path can proceed.

## Non-Effects

This gate is report-only:

- no operator record is recorded or persisted;
- no controlled request is dispatched or executed;
- no Memory write;
- no external KG read or live KG write;
- no prompt/context injection;
- no provider/model invocation;
- no secret or credential read;
- no channel send;
- no install, restart, or active binary mutation.
