# WEB-C1.3 worker timeout, teardown, and recovery contract

Status: `NORMATIVE_PLAN / IMPLEMENTATION_BACKLOG`

## Deadlines

The real host owns independent monotonic deadlines for:

- process spawn and endpoint inheritance;
- WorkerHello;
- HostAck/WorkerConfirm completion;
- each frame header and body read;
- each frame write/flush;
- command completion;
- human takeover transition;
- drain and graceful shutdown;
- forced termination and process reap.

Wall-clock time is evidence metadata only. It must not be the sole expiry/fencing source inside a
running process.

## Failure handling

Before handshake completion, any timeout, EOF, protocol error, identity mismatch, source mismatch,
authority mismatch, capability mismatch, or nonce mismatch closes the endpoint and terminates the
child. No negative frame containing diagnostic details is sent to an unauthenticated peer.

After establishment, malformed or stale frames fence the session. The parent stops admitting new
commands, records a bounded failure code and process identity, closes the channel, terminates/reaps
the worker, and advances generation/owner epoch before any replacement worker is launched.

## Unknown outcomes

C1 local-fixture commands have no external effect. A worker hang or channel loss is nevertheless
reported as `Indeterminate` when the parent cannot prove whether the in-process page mutation
completed. The current page generation is discarded; the command is not replayed into the same or
a new worker by default.

Later external-effect stages require TaskFlow-owned idempotency, durable intent, provider/browser
status evidence, and reconcile decisions. This protocol does not provide those semantics.

## Recovery

Recovery creates a new worker process and a new generation or owner epoch. It never adopts a stale
worker, reuses a startup capability/host nonce, or accepts a prior established channel. A new
process may reconstruct only from a separately authenticated, receipt-bound checkpoint explicitly
allowed by the current stage. C1 has no such live-page recovery and starts from a clean local
fixture.

## Resource cleanup

The parent must prove bounded cleanup of:

- child process and descendants;
- inherited descriptors/handles;
- profile and temporary roots;
- renderer/cache/shared-memory artifacts;
- downloads and pending file descriptors;
- OS sandbox/job/cgroup membership;
- queued commands and human leases;
- response cache and request-ID tombstones;
- sensitive startup material.

Cleanup failure is a fenced degraded state and blocks replacement until an explicit supervisor
policy resolves it.
