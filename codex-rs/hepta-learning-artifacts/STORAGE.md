# Immutable artifact storage boundary

This ART-1/ART-2 storage sub-slice adds actual file I/O to the existing artifact
registry. It is not a second registry, runtime selector or full learning loop.
The caller supplies already-authorized regular files; this crate does not open
paths, obtain credentials, install models or select a production artifact.

## Snapshot and payload API

`write_registry_snapshot` writes a NEW empty file and syncs it before returning a
`RegistrySnapshotReceipt`. `read_registry_snapshot` requires that exact externally
retained receipt, checks bytes and history, then rebuilds the SAME `ArtifactRegistry`
through its existing validation. Register, quarantine and revoke retain canonical
lineage semantics. There is no repair of truncation and no fallback to old state.

The HEPTAR01 encoding is UTF-8/ASCII, newline terminated: magic, binding digest,
record count, then one pipe-delimited event per line. R records contain event ID,
artifact ID, kind tag, generation, optional predecessor, content/objective/support
digests, producer, compatibility digest and encoded size, in that order. Q and V
records contain event, artifact, evaluator and reason. IDs already prohibit pipes
and newlines. Re-encoding must match byte-for-byte, rejecting alternate integers,
line endings and other noncanonical input. Existing event and chain digest
algorithms are unchanged and checked after replay. The receipt binds scope,
record count, chain head, complete file digest and byte count.

Candidate payload functions verify current registry eligibility, byte length and
content digest. A revoked ancestor blocks loading descendants. Stored code or
model bytes are never executed. Snapshot limits are 4096 events and 8 MiB;
payloads are bounded by 64 MiB. Snapshot creation is O(history), bounded by the
pilot cap; this is not a high-frequency journal or hard-real-time controller.

## Host transaction and trust boundary

The host authenticates the scope, file ownership, identities and receipt. Digests
and differing evaluator strings alone do not authenticate people or services.
A current external revocation witness is mandatory before any runtime use: a valid
old snapshot plus its old receipt can still predate a deletion. This module cannot
infer the latest state from the suspect file. Never use an older snapshot to make
a revoked predecessor appear eligible for rollback.

Create payload -> sync -> create canonical registry snapshot -> sync -> durably
publish the receipt/witness -> independent evaluation/decision -> separately
owned next-run selection. Cross-store atomicity requires a host transaction or
outbox reconciliation; two synced files are not an atomic multi-store transaction.
A crash before witness publication may leave an orphan candidate, not a selected
artifact. An I/O failure is indeterminate and the caller must reconcile exact bytes
before retry; it must not overwrite or silently reuse a partial file.

The host owns containing-directory sync, trusted path traversal, encryption,
quota/retention, revocation freshness, physical erasure, backup deletion, independent
witness storage and selection/rollback. File locks fence cooperative independently
opened handles, not hostile writers or cloned/inherited handles. Platform-specific
file-lock/power-loss behavior needs separate qualification.

## Verification and non-claims

Regression sources cover real-file reopen, every snapshot truncation point,
independent witness mismatch, canonical form, no-overwrite, cooperative writer
fencing, payload integrity, revocation descendants and invalid binding. Exact
source and actual-base synthetic-merge compilation/tests/lint/format remain
mandatory. Local execution was unavailable while preparing this candidate;
test source is not execution evidence. No existing verifier or test is weakened.
No parent work package, production caller, efficacy or claim level is advanced.
