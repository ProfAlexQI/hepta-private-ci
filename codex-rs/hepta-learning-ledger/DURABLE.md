# Durable causal episode ledger

This is the file-backed persistence sub-slice of `LRN-1-DURABLE-EPISODE-LEDGER`.
It uses the existing `LearningLedger` semantics and event/chain digests, not a
second causal ledger or a canonical platform wire protocol. It does not complete
all of LRN-1 or grant production activation.

## API and ownership

`DurableLedger::create(file, binding, max_records)` initializes an empty file;
`recover(file, binding, max_records, recovery)` never initializes missing history.
The host must pass a separately opened, already authorized read/write `File` and
an authenticated store/scope/purpose/epoch binding. No path, credential, socket,
provider, model handle or external fact writer is accepted by the adapter.
Cooperating independent writers are fenced using the standard exclusive file
lock. Locks are not a defense against hostile writers, aliases or privileged
filesystem modification. The host owns new-file directory synchronization,
revocation, isolation, encryption and filesystem qualification.

## Commit and recovery algorithm

The pure core now separates validation/preparation from publication. The public
in-memory `append` retains its behavior. The durable adapter prepares without
mutating the core, checks the exact predecessor, encodes one bounded event frame,
appends it, calls `sync_all`, and only then publishes the event and receipt in
memory. Failed semantic validation cannot write the file. I/O uncertainty poisons
the handle and blocks further reads/writes until reopen and reconciliation.
An equal canonical retry returns the original event/chain identity with the
existing `IdempotentReplay` disposition and performs no new disk write, including
a retry after later events. Candidate permutation remains canonical; changed
content under the same identity or a wrong predecessor is not last-write-wins.

A 72-byte HEPTLR01 header binds format version, host binding and checksum. Frames
use a checked length/complement pair, sequence, predecessor digest, the existing
canonical event encoding, chain digest and checksum. Numbers are big-endian.
Recovery bounds lengths before allocation, decodes typed events, runs the SAME
causal validation and reconstructs each exact canonical frame. Duplicate frames,
unknown types, noncanonical order, bad checksums and rehashed invalid lineage
reject. Only an incomplete final frame may be repaired.

`LedgerRecovery::Acknowledged(LedgerAnchor)` requires an externally retained
sequence and chain digest. Its prefix must exist and match before any repair.
Later valid complete frames are preserved for lost-acknowledgement reconciliation;
corruption after the anchor still rejects. Recovered bytes are synced before
exposing committed results. The host must authenticate and bind the witness,
retain it independently and acknowledge externally only after retaining it.
It must not retry a failed anchored recovery as `Unacknowledged`. This patch does
not supply the independent witness store; an unanchored recovery cannot detect
loss of a whole valid suffix.

## Causal and resource boundaries

The existing explicit-abstain, complete-candidate, nonzero-propensity,
independent-observer-ID, terminal-outcome credit and no-double-credit rules are
preserved. Host authentication is still necessary: differing supplied identity
strings alone do not prove independent observation. Decisions, outcomes, credits
and logical revocations share one ordered durable chain. Replay applies all
revocations before exposing active records, including causal descendant exclusion.
Revoked bytes remain in the audit journal: this is NOT physical erasure, backup
deletion, machine unlearning or evidence of future learning improvement.

Pilot caps are 1..8192 records, 8 MiB per segment, 32 KiB per encoded event,
128 candidates and 128 bytes per stable identity. Quota exhaustion stops rather
than dropping history. Replay and indexes are bounded by these caps; equal retry
lookup is linear in the bounded record count. The synced path has no hard
real-time or target-host latency claim. Segment rollover, compaction, migration,
canonical learning protocols, authenticated observer/witness services, and
production/evaluation/artifact consumers remain separate integration work.

## Verification and rollback

The existing eight core tests remain unchanged. Sixteen new tests cover actual
file recovery, all four event types, revocation descendants, acknowledged-history
loss at each final-frame cut, canonical retry after later events, stale CAS,
corruption, malformed lengths and variants, quota, writer fencing, failed writes,
unacknowledged complete frames, and an independent byte-level golden vector.
The one-event golden file is 362 bytes with SHA256
`eba8162e7d3f4e8eb26babe2552731774ee9c6cd04facf81a3bb2a004eefbfcf`.
Native filesystem and physical power-loss qualification are not implied by a
Linux test result. The dedicated read-only CI checks exact source and actual-base
synthetic merge independently. No parent work-package or capability status is
advanced by creating this code. Rollback leaves the new file inert; never read
an older snapshot as if it included later revocations or confirmed results.
