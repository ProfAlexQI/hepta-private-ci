# Hepta runtime-bootstrap transport and recovery matrix V1

**Date:** 2026-08-30  
**Status:** normative P0.7a component contract; source-only and authority-negative.  
**Namespace:** `hepta.runtime-authority-bootstrap.v1`

## 1. Scope

This contract closes the durable fallback handoff between the Supervisor-owned
runtime-bootstrap issuer and the Agentd consumer. It defines physical-file
identity, publication order, one-use claim semantics, crash classification and
generation recovery. It does not grant model, provider, tool, network,
filesystem, Matrix, fleet, operator, promotion or release authority.

The three generation-qualified objects are:

```text
R(g) = runtime bootstrap reservation
D(g) = signed bootstrap document
C(g) = durable one-use claim
```

All names are derived from the exact Agent generation. A file from generation
`g` is never reinterpreted for `g+1`.

## 2. Physical transport invariant

Every `R(g)`, `D(g)` and `C(g)` accepted by Agentd must satisfy all of the
following before any authority value is constructed:

1. the Agent run root is a physical, non-symlink, owner-only directory;
2. the object is opened with no-follow semantics on Unix;
3. `symlink_metadata` and opened-file metadata identify the same inode;
4. the object is a physical regular file;
5. its Unix UID equals the Agent run-root UID;
6. its mode is exactly `0400` on Unix, or read-only on other platforms;
7. before claim, its link count is exactly one;
8. during the claim CAS, `R(g)` and `C(g)` are the same inode with link count two;
9. after cleanup, retained `C(g)` has link count one;
10. size is non-zero and within the protocol byte bound;
11. device, inode, ctime and length remain stable during read;
12. decoded bytes bind the exact Agent, generation, envelope and nonce.

The immutable trust-root and release-provenance registry uses the same
no-follow, owner-bound, link-exact and metadata-stable rules. Registry objects
are published as exact `0400` files inside physical owner-only directories.
Release bytes are accepted only as physical single-link, non-writable files
whose UID matches their containing immutable release directory.

A symlink, dangling symlink, additional hardlink, wrong mode, owner mismatch,
path swap, byte drift or over-bound object fails closed. The implementation does
not delete the evidence that caused the failure.

## 3. Publication protocol

Supervisor publishes in this order:

```text
validate exact fleet/release/trust facts
  -> create private temporary reservation inode
  -> write + file fsync
  -> chmod via held file descriptor
  -> second file fsync
  -> no-replace hardlink to R(g)
  -> verify final path binds the held inode
  -> unlink temporary name
  -> parent-directory fsync
  -> repeat for D(g)
```

`R(g)` is published before `D(g)`. Failure after `R(g)` publication leaves the
reservation in place. The issuer must not retry the same generation over that
partial state.

The publisher checks all three final names with physical metadata. A dangling
symlink therefore counts as occupied state and cannot be bypassed by
`Path::exists()` behavior.

## 4. Claim protocol

Agentd performs:

```text
physical validation of D(g) and R(g)
  -> bounded no-follow reads
  -> signature, trust, profile, ProductGraph and release verification
  -> verify R(g) binds D(g) and its nonce
  -> no-replace hardlink R(g) to C(g)
  -> parent-directory fsync
  -> verify R(g) == C(g) inode and link count two
  -> no-follow read C(g) and compare exact verified reservation bytes
  -> re-read fleet and release provenance
  -> remove D(g) and R(g)
  -> parent-directory fsync
  -> retain C(g) as immutable consumed evidence
```

`C(g)` is the one-use compare-and-claim result. It is not an authority token.
It records that the start identity was consumed. Once `C(g)` exists, the same
generation can never replay the bootstrap, even if `D(g)` or `R(g)` remains.

## 5. Durable state matrix

Legend:

- `0`: path physically absent;
- `1`: path physically present;
- `PASS_START_IDENTITY`: Agentd may complete bootstrap verification;
- `RECOVERY_REQUIRED`: fail closed and preserve evidence;
- `CONSUMED`: same generation is permanently fenced.

| R | D | C | Classification | Required behavior |
|---:|---:|---:|---|---|
| 0 | 0 | 0 | absent | legacy local-closed-world only when no provenance-bound release requires bootstrap; otherwise fail |
| 1 | 0 | 0 | partial reservation | `RECOVERY_REQUIRED`; do not publish or consume |
| 0 | 1 | 0 | orphan document | `RECOVERY_REQUIRED`; do not infer a reservation |
| 1 | 1 | 0 | published and unclaimed | validate all physical and signed facts, then attempt claim |
| 1 | 1 | 1 | crash after claim | `CONSUMED`; preserve all three objects |
| 1 | 0 | 1 | claimed with document cleaned | `CONSUMED`; preserve remaining objects |
| 0 | 1 | 1 | claimed with reservation cleaned | `CONSUMED`; preserve remaining objects |
| 0 | 0 | 1 | clean consumed state | `CONSUMED`; same generation never replays |

Any path present with the wrong type, owner, mode, link count, inode relation or
content is `RECOVERY_REQUIRED`, regardless of the table row.

## 6. Crash-window requirements

### W1 — after reservation fsync, before `R(g)` publication

No final path exists. A later issuance may start only after proving the
temporary file is not a published object. Orphan temporary files remain
forensic data and are never promoted in place.

### W2 — after `R(g)` publication, before `D(g)` publication

`R=1,D=0,C=0`. The generation is blocked as partial. Source code does not delete
or overwrite the reservation.

### W3 — after `D(g)` publication, before process spawn

`R=1,D=1,C=0`. The exact generation may consume once if all bindings remain
valid. A different generation ignores these names and uses fresh names and a
fresh nonce.

### W4 — after verification, before claim

No authority object has escaped. Retry may repeat verification while `C(g)` is
absent, but it must revalidate physical metadata, signature, fleet facts and
expiry.

### W5 — after claim link, before claim directory fsync

The outcome is uncertain. Presence of `C(g)` on reopen fences the generation.
Absence permits a fresh claim attempt only after complete revalidation.

### W6 — after claim fsync, before fleet revalidation

`C(g)` is durable. Any later error retains consumed/recovery-required state and
must not unlink the claim.

### W7 — after claim, during document/reservation cleanup

All rows with `C=1` are consumed. Cleanup completion is not required to prove
one-use semantics.

### W8 — after successful admission, before App Server readiness

The generation remains consumed. Supervisor recovery may classify the process
as failed and start a later generation, but it may not reissue or replay `g`.

## 7. Fresh-generation recovery

A later generation is a new operation:

```text
g fails or is fenced
  -> fleet lifecycle CAS advances generation
  -> Supervisor resolves current release and trust facts again
  -> new signed document with new nonce
  -> publish R(g+1), D(g+1)
  -> Agentd claims C(g+1)
```

Historical `C(g)` is preserved and has no effect on the separate filenames for
`g+1`. The old claim proves only that generation `g` consumed a start identity;
it authorizes nothing in the new generation.

No cleanup, repair tool or operator action may rewrite old files into a current
generation. A future cleanup command, if implemented, must be a separately
versioned forensic operation with its own capability, receipt and rollback
contract.

## 8. Required tests

P0.7a source qualification must include:

- valid single use and same-generation replay rejection;
- provenance-bound release without handoff;
- expired and tampered document rejection before claim;
- wrong-mode rejection;
- symlink and dangling-symlink rejection;
- unsafe hardlink rejection;
- partial reservation and orphan document classification;
- simulated durable claim before cleanup;
- old retained claim plus successful fresh-generation issuance;
- trust-root/provenance wrong-mode, hardlink and symlink rejection;
- exact UID, mode, link count and inode publication assertions;
- candidate-clean, locked metadata, format, tests, check and strict Clippy.

Tests are source evidence until an assigned runner executes non-empty steps for
the exact candidate and merge candidate.

## 9. Authority boundary

This matrix keeps all of the following false:

```text
runtime authority elevation
production caller
production writer
model invocation
provider dispatch
tool execution
network connect
external filesystem mutation
secret operation
Matrix send
external effect
fleet mutation
operator acceptance
promotion
release
```
