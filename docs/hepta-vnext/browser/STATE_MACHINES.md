# Hepta Browser State Machines V1

Status: normative development contract; qualification-only.

## 1. Session lifecycle

```text
Allocated
  -> Starting
  -> AgentControlled
  -> HumanControlled
  -> AgentControlled
  -> Draining
  -> Closed

Any non-terminal state -> Fenced -> Draining/Closed
Starting -> FailedClosed
```

### Allocated

A session ID, generation, policy digest and private runtime root are reserved. No Servo process or mutation authority exists.

### Starting

The supervisor creates the private control channel and launches the pinned worker. Startup capability, source/artifact identity, profile root and process identity are verified before the Actor reports ready.

### AgentControlled

One Browser Actor accepts Agent mutations. Every request must match the current session ID, generation, owner epoch and page revision.

### HumanControlled

Entering this state increments `owner_epoch` before acknowledging takeover. Queued Agent mutations that have not crossed the engine boundary are denied. In-flight mutations are allowed to finish only if their dispatch was already durably recorded; otherwise the session becomes `Fenced/Indeterminate` and requires reconciliation.

The human lease is evaluated with a supervisor-provided monotonic clock. Wall-clock timestamps are evidence only. Clock reset, overflow or unavailable monotonic time fails closed. Expiry changes the control mode but does not revive old requests; callers must read the current epoch and revision.

### Draining

No new mutation is admitted. Read-only status/evidence may continue. Pending durable outcomes are sealed or marked indeterminate before teardown.

### Fenced

A generation, owner, integrity, protocol, policy, worker or supervisor violation occurred. New commands and stale reconnects are rejected. Fencing is monotonic for the current generation.

### Closed

The worker/control channel is gone, ephemeral profile cleanup is complete or explicitly retained, and the terminal receipt is sealed. Closed sessions cannot reopen; recovery creates a new generation.

## 2. Page revision

`page_revision` is a strictly increasing `u64` within a session generation.

It increments for any event that can invalidate semantic identity, including:

- committed navigation or document replacement;
- frame context replacement;
- semantic tree rebuild after relevant DOM/accessibility changes;
- history traversal that changes the active document;
- engine recovery from a snapshot;
- explicit full refresh requested by policy.

Overflow fences the session. Semantic references bind the exact revision and cannot be re-resolved heuristically after a mismatch.

## 3. Owner epoch

`owner_epoch` is a strictly increasing `u64` and changes before:

- Agent -> human takeover;
- human -> Agent release/expiry;
- supervisor adoption or worker replacement;
- recovery after a fenced/indeterminate state;
- policy decision that changes mutation ownership.

Overflow fences the session. Epoch changes are recorded before new-owner commands are admitted.

## 4. Request idempotency

```text
Unseen(request_id)
  -> Prepared(request_digest)
  -> Dispatched
  -> Completed(response_digest)
       | Failed(response_digest)
       | Indeterminate(reconcile_ref)
```

Rules:

1. Reusing a request ID with another canonical digest is always denied.
2. Reusing a request ID with the same digest returns the sealed outcome when available.
3. A bounded in-memory response cache may evict response bodies, but it MUST NOT forget that a request ID was consumed. The durable journal or a monotonic request high-watermark/tombstone set preserves conflict detection.
4. When the bounded consumed-ID structure cannot safely represent another request, admission fails with `capacity_exhausted`; it never evicts an authority-relevant tombstone silently.
5. Request sequence wrap, rollback or reuse across generation boundaries is invalid.

The current fixture Actor is qualification-only; C5 must add durable tombstones and crash recovery before any external effect.

## 5. Command crossing boundary

A command is not considered dispatched merely because it was queued or written partially. The transport adapter records a durable dispatch fact only after the complete canonical frame is accepted by the worker channel under the current fence.

- Failure before dispatch proof: safe to report `NotDispatched`.
- Failure after dispatch proof but before terminal outcome: `Indeterminate`.
- `Indeterminate` cannot be converted to success or blindly retried without reconciliation evidence.

## 6. Human takeover race matrix

| Agent command state | takeover result |
|---|---|
| not admitted | reject under new epoch |
| admitted, not prepared | reject |
| prepared, not dispatched | cancel and seal `NotDispatched` |
| dispatched, read-only | finish or cancel under bounded rule |
| dispatched, mutating | await terminal proof or mark `Indeterminate`; do not replay |
| terminal | retain terminal outcome |

## 7. Worker crash and reconnect

A crashed worker cannot reconnect to an existing channel or generation. The supervisor closes the inherited endpoint and fences the session. Recovery verifies the durable journal and creates a new worker under a new owner epoch; if the active document cannot be proven equivalent, page revision increments and all old semantic refs are invalid.

## 8. Cancellation

Cancellation binds request ID, digest, session generation and cancel generation. It is advisory until the worker returns a typed cancellation proof. A cancellation race with completion retains the first durable terminal outcome. A late result from an old cancel/session generation is quarantined.

## 9. Required property tests

- generations, epochs and revisions never decrease or wrap;
- stale values never mutate state;
- takeover linearizes before human acknowledgement;
- request conflict survives response-cache eviction and restart;
- no terminal outcome changes after sealing;
- indeterminate actions are never blindly replayed;
- worker reconnect cannot reuse a closed generation;
- clock rollback cannot extend or revive a human lease.
