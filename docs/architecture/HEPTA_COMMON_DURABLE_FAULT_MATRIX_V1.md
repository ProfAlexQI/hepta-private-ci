# Hepta common durable-store fault matrix V1

**Status:** normative P0.7d contract.  
**Stores:** Memory, Automation, Matrix, Evidence, Fleet/Supervisor and TaskFlow.

Every durable owner must execute the same semantic matrix against its real
SQLite/file implementation. A source-only model or in-memory mock cannot close a
row marked `physical`.

| ID | Fault | Injection point | Required result |
|---|---|---|---|
| F01 | process kill before intent commit | before local transaction commit | no intent, projection or dispatch exists after reopen |
| F02 | process kill after intent commit | after fsync, before dispatch | one recoverable intent/outbox row; no false terminal |
| F03 | process kill after boundary crossing | after dispatch, before acknowledgement | `Indeterminate` or delivery-pending; lookup-only reconcile |
| F04 | lost acknowledgement | receiver applied, sender did not record ACK | duplicate delivery is deduped; one mutation |
| F05 | duplicate command/event | same key and same digest | `AlreadyApplied`; no second event/effect |
| F06 | key collision with different digest | same key, changed payload | conflict, quarantine or manual resolution; never overwrite |
| F07 | stale owner/generation/fence | before any mutation | fail closed with no durable change |
| F08 | lease expiry | immediately before commit and boundary | commit/dispatch denied; current owner may reconcile |
| F09 | SQLite full | inside the real writer transaction | full rollback, no partial projection/outbox, clean reopen |
| F10 | permission loss/read-only media | before journal or WAL write | explicit unavailable state, no memory-only success |
| F11 | WAL/SHM reopen after crash | dirty but valid local state | deterministic recovery to one legal state |
| F12 | corruption | page, journal or manifest digest corruption | fail closed; no automatic destructive repair |
| F13 | backup during pending operation | intent/outbox not terminal | backup records pending state and restores it recoverably |
| F14 | restore to new destination | after verified backup | owner/path binding rederived; replay against old binding rejected |
| F15 | clock regression/advance | lease and retry scheduling | monotonic fence prevents stale-owner success or retry storm |
| F16 | cancellation race | cancel concurrent with dispatch/ACK | sticky cancel; terminal transition follows canonical precedence |
| F17 | bounded queue exhaustion | N+1 admission | bounded rejection/backpressure; no dropped durable accepted work |
| F18 | schema migration interruption | each migration publication phase | old or new schema is identifiable; no mixed authority |

## Required evidence per store

Each store publishes an exact candidate-bound result with:

- store owner and schema version;
- source commit/tree and test binary digest;
- physical or model injection class;
- before/after canonical state digest;
- intent, event, outbox, receipt and terminal counts;
- lease/epoch/generation/fence identity;
- reopen, backup and restore result;
- proof that no duplicate external effect occurred;
- raw test log digest and runner identity.

A skipped row, `steps=[]`, runner ID zero or synthetic-only substitute is
`not_run`, not pass.
