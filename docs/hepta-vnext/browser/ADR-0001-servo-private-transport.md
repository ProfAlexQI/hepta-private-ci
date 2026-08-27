# ADR-0001: Private Servo worker transport

Status: **Accepted for WEB-C1 bootstrap**  
Scope: development and qualification only  
Authority: none

## Context

Servo's current WebDriver server starts an HTTP listener on `0.0.0.0`. Exposing that listener—even on an ephemeral port—would create an unnecessary unauthenticated control surface and would make origin, caller identity and lifecycle fencing depend on a generic WebDriver boundary.

Hepta also needs stronger semantics than raw WebDriver: session generation, owner epoch, page revision, human takeover, canonical request idempotency, policy digest, redacted evidence and fail-closed recovery.

## Decision

WEB-C1 will run Servo as a **supervisor-owned worker process with no TCP control listener**.

### Process model

- Initial qualification uses one Servo worker per browser session.
- The supervisor creates the control channel before spawning the worker and passes the connected endpoint directly.
- Unix uses an inherited private Unix stream/socketpair file descriptor. Windows uses a supervisor-created named pipe restricted to the current user SID and expected child process.
- The worker receives a random, one-use startup capability through the inherited channel, not through command-line arguments or environment variables visible to unrelated processes.
- The worker cannot outlive its session generation. Parent death, generation change, protocol violation or heartbeat expiry terminates the worker and invalidates its profile.

### Protocol

- Hepta-owned request/response/event types only; no public WebDriver passthrough.
- Four-byte big-endian frame length followed by compact canonical JSON for C1 bootstrap, maximum 1 MiB per frame. A later binary encoding requires a versioned migration rather than silent replacement.
- Every frame binds protocol version, session ID, generation, owner epoch, page revision, monotonically increasing channel sequence and request/event digest.
- Unknown fields, unknown mandatory message types, duplicate sequence numbers, oversized frames and non-canonical bytes close the channel.
- Requests remain idempotent under the Browser Actor contract; transport reconnect does not authorize replay of an indeterminate action.

### Servo integration surface

The worker embeds or links the pinned Servo/servoshell library through a Hepta adapter. It exposes only:

- create/close session;
- navigate to an already admitted URL;
- bounded semantic observation;
- bounded click/type/scroll/focus/wait actions;
- viewport screenshot under evidence policy;
- status, cancellation and teardown.

Arbitrary JavaScript execution, Servo preference mutation, raw cookies/storage, profile paths, extension routes and unrestricted WebDriver commands are excluded from C1.

### Network posture

- C1 qualification renders only local fixtures and starts with OS-level external egress denied.
- C4 introduces an explicit egress policy; enabling external network before C4 is a qualification failure.
- Servo's HTTP/WebDriver listener is disabled or not invoked. CI scans the worker process for wildcard/listening sockets during smoke tests.

### Storage posture

- Each worker receives a fresh private profile directory under a supervisor-owned session root.
- Profile, cache, downloads and temporary files are size-bounded, non-symlink roots and never returned to the Agent.
- Normal close deletes ephemeral state after evidence sealing. Crash cleanup is idempotent; retained forensic state requires an explicit private retention policy.

## Consequences

Positive:

- no browser control TCP attack surface;
- lifecycle and caller identity derive from the supervisor-created channel;
- Hepta contracts remain stable if Servo internals change;
- per-session process isolation simplifies crash and secret containment.

Costs:

- per-session process startup and memory overhead;
- a maintained adapter and protocol corpus;
- Windows and Unix transport implementations;
- Servo upgrades require adapter compatibility qualification.

## Rejected alternatives

1. **Expose Servo WebDriver on 0.0.0.0** — rejected: unauthenticated wildcard listener.
2. **Loopback WebDriver as the primary public Hepta API** — rejected: overly broad semantic and security surface. It may be used only inside a tightly bounded test harness.
3. **In-process Servo inside agentd for C1** — deferred: a browser crash or native dependency failure would expand the Agent daemon failure domain.
4. **Shared long-lived Servo worker pool** — deferred until isolation, profile reset and cross-tenant cache behavior are independently qualified.

## Qualification requirements

- no TCP listener exists in the worker smoke test;
- wrong startup capability, session, generation, epoch, revision or sequence closes/fences the channel;
- worker crash and parent death clean up the session without replay;
- raw cookie/profile/credential scan is zero;
- external network is unavailable during C1 tests;
- Linux, macOS and Windows transport fixtures pass before C1 is marked complete.
