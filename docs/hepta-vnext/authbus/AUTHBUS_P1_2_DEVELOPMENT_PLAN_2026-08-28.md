# AuthBus P1.2 Development Plan — Durable Identity and Evidence Replay State

Date: 2026-08-28
Parent tranche: P1.1 signed identity and evidence verification
Exact parent candidate: `0be01b7b5063066794731e545cf304e4c07c1fc5`

## 1. Objective

P1.1 proves signatures, key purpose, epochs, audience, process identity, nonce,
fence and evidence monotonicity only inside one process. P1.2 closes the crash
and restart gap by placing every security-relevant replay decision in one
private SQLite WAL durability domain.

The tranche remains qualification-only. It creates no service listener, no
provider call, no secret-backend or OpenBao integration, no product caller and
no production writer.

## 2. Non-negotiable invariants

### 2.1 Verification-key ledger

The durable identity of a verification key is:

```text
issuer_id + purpose + key_id + key_epoch
```

Registration is exact-replay idempotent. Changed bytes at the same identity are
a conflict. Epochs are monotonic per issuer and purpose. Revocation is durable,
revisioned and survives reopen. A revoked or stale-purpose key cannot authorize
a new nonce, provider observation or manual observation.

### 2.2 Nonce replay ledger

A launch-nonce claim binds the P1.1 evidence digest, canonical identity binding,
issuer/key/epoch, subject, audience, process launch nonce, request nonce and
expiry. Acceptance may be returned only after the nonce transaction commits.
A committed claim is rejected after any reopen. Capacity exhaustion fails
closed; live claims are never silently evicted.

### 2.3 Operation and evidence ledgers

An operation binding fixes provider, profile, token family, status binding and
the complete authority/owner/generation/fencing tuple. Provider status and
manual evidence use separate revision heads and separate row-digest domains.
Exact evidence replays; same-revision changed evidence conflicts; time and
revision rollbacks fail closed.

Terminal provider outcomes atomically append the final evidence, advance the
head and create a terminal tombstone. Once terminal, changed evidence remains
immutable even after old detail rows are compacted.

### 2.4 Writer generation fencing

Every mutation rereads the durable writer identity. Same generation with a
different boot ID and any lower generation are stale. A higher generation may
rebind the store, after which every older store handle is fenced from key,
nonce, operation, evidence and GC writes.

### 2.5 Retention and garbage collection

Garbage collection is CAS-bound to a monotonically increasing cursor and a
bounded row limit. It may delete only:

- expired nonces;
- non-head evidence rows older than evidence retention;
- revoked, non-current keys older than key retention;
- old receipts;
- terminal operation state only after the independently longer tombstone
  retention deadline and after no live manual head remains.

A stale cursor produces no partial deletion.

### 2.6 Storage and corruption failure

The database runs with SQLite WAL, `synchronous=FULL`, foreign keys, private
filesystem permissions, `secure_delete=ON` and `trusted_schema=OFF`. Disk or
storage unavailability before commit, deterministic crash failpoints, row
digest drift, broken head references, tombstone drift and failed
`PRAGMA quick_check` all fail closed.

## 3. Durable schema

The first migration owns the following strict tables:

```text
authbus_p1_2_meta
p12_key_registrations
p12_key_heads
p12_nonce_claims
p12_operations
p12_status_evidence
p12_status_heads
p12_manual_evidence
p12_manual_heads
p12_terminal_tombstones
p12_durable_receipts
p12_gc_cursor
```

No column may store private key material, raw signatures, access/refresh tokens,
authorization headers, provider bodies or secret values.

## 4. Executable acceptance matrix

The exact-head candidate must pass with Rust 1.95 and a committed resolver-3
lockfile:

1. source, schema and negative-authority verifier;
2. package-scoped rustfmt;
3. default-feature-off unit test;
4. key replay, changed-registration conflict, rotation, revocation and reopen;
5. nonce replay after reopen, bounded capacity and expiry GC;
6. provider evidence exact replay, conflict, monotonicity, terminal tombstone
   and post-reopen terminal immutability;
7. independent manual evidence ledger and lookup-only resume;
8. writer-generation takeover and stale-handle fencing;
9. key/nonce/operation/status/manual/GC pre-commit rollback;
10. simulated storage-unavailable rollback;
11. CAS GC retaining current heads and live tombstones;
12. row corruption detection during integrity verification and reopen;
13. all-target `cargo check`;
14. strict all-target Clippy with `-D warnings`.

Queued jobs, `runner_id=0`, empty steps, source-only receipts and a generated but
uncommitted lockfile are not qualification evidence.

## 5. Promotion boundary

P1.2 can be called executable-qualified only when one exact final head has
non-empty successful hosted steps for every required gate. Even then all
production, effect, operator, promotion, G5 and execute authority remains false.

Product workspace membership, a real service adapter and any P1.3 registry work
must be separately reviewed and qualified.
