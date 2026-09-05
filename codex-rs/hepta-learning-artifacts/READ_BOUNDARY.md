# Artifact file reader and writer boundary

This ART-1/ART-2 hardening retains the existing immutable snapshot and payload
formats, manifests, lineage validation, content digests, quotas and no-overwrite
semantics. It adds no selection or installation operation.

`read_registry_snapshot` and `read_candidate_payload` now acquire shared locks
and accept independently opened OS read-only files. A consumer does not need
write access to load a witnessed registry or eligible artifact. `write_new`
continues to require an exclusive lock and synchronizes before returning.

A private owned-file guard is created only after lock acquisition succeeds.
It explicitly unlocks on normal success or rejection before closing the file.
This prevents a transient inherited open description from retaining a completed
operation's lock. Failure to acquire a lock never unlocks another owner. The
bounded reader borrows the file for `Read::take` so ownership cannot escape the
guard. Caller-supplied already locked or concurrently used aliases remain
unsupported; duplicate handles appear only in regression fixtures.

Four new tests cover read-only snapshot/payload loading, shared readers versus
exclusive writers, writer completion with a transient duplicate retained, and
rejected reads with that duplicate retained. They preserve the original storage
suite. Linux duplicate-description tests do not claim Windows or power-loss
qualification. Formatting of the touched storage implementation is normalized
without changing the serializer's record order or fields.

The host still authenticates and refreshes witnesses and registry revocations,
owns parent-directory synchronization and cross-store reconciliation, and
separately authorizes use. File locking is not authentication, hostile-writer
isolation or continuous revocation freshness. Logical revocation is not physical
erasure. Test source and a CI submission are not passed execution evidence;
source-head, actual-base merge, product-matrix and independent review gates
remain mandatory. No capability or completion state is advanced here.
