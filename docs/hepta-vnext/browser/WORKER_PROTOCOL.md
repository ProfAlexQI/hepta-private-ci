# Hepta Browser private worker protocol v1

Status: **implemented as a qualification scaffold; production transport and
Servo runtime are not qualified**

## 1. Purpose

The protocol separates the Hepta Browser Actor from a crashable browser worker
without exposing Servo or WebDriver types. It carries one typed browser session
and nothing else. The worker reports facts and outcomes; it never grants
policy, effect, caller, operator or promotion authority.

The current executable harness uses inherited child stdin/stdout pipes solely
to qualify framing and lifecycle behavior. Production C1 requires an inherited
Unix socketpair and a Windows named pipe whose ACL is bound to the expected
SID. Neither production transport exists yet.

## 2. Envelope

Every frame has exactly:

```text
schema_version
session_id
generation
sequence
payload
```

The encoding is compact recursively sorted canonical JSON preceded by a
four-byte unsigned big-endian length. The payload length is `1..=65536` bytes.
Unknown fields, duplicate fields rejected by the strict decoder, malformed JSON,
noncanonical encoding, zero length and larger lengths fail closed.

A protocol error closes the channel generation. There is no resynchronization,
frame skipping, partial recovery or fallback to a network listener.

## 3. Startup capability

The parent generates 32 unpredictable bytes for each process launch. The raw
capability is sent exactly once in the initial private-channel `ParentHello`.
Only its domain-separated SHA-256 digest is passed through the process launch
environment and retained by the parent/server state machines.

The capability:

- is bound to the exact `session_id` and nonzero `generation` envelope;
- is compared without early exit;
- is redacted from `Debug` output;
- is zero-filled when its Rust value is dropped;
- is not a credential for another session or process generation;
- does not authorize browser commands, network access or external effects.

The current qualification harness does not prove same-UID peer identity. The
production Unix and Windows transports must add OS process identity and artifact
digest binding before C1 can pass.

## 4. Sequence rules

Handshake frames use sequence zero in each direction. After readiness, each
direction starts at one and increments by exactly one. Duplicate, stale,
skipped or overflowing sequence values close the session.

There is no wraparound. A future reconnect creates a new worker process and
browser generation; it cannot resume the old sequence space.

## 5. Payloads

### Parent to worker

- `parent_hello`
- `request`
- `shutdown`

### Worker to parent

- `worker_ready`
- `response`
- `protocol_error`
- `shutdown_ack`

`request` contains the Hepta `BrowserRequest`. `response` contains the Hepta
`BrowserResponse`. The parent and worker independently re-check session and
generation. Ready and response frames must carry a closed
`BrowserAuthorityStatus`; any enabled authority bit closes the channel.

Raw WebDriver commands, Servo preferences, JavaScript objects, cookie jars,
profiles, files, network handles and native pointers have no protocol variant.
Adding one requires a successor schema and threat-model review.

## 6. Channel state machines

Parent:

```text
AwaitingReady -> Ready -> Closing -> Closed
       |          |          |
       +----------+----------+--> Closed on any violation
```

Worker:

```text
AwaitingHello -> Ready -> Closing -> Closed
       |          |          |
       +----------+----------+--> Closed on any violation
```

The parent cannot send a request before a valid ready frame. The worker cannot
send a response before a valid hello. Shutdown is explicit and acknowledged.
EOF, timeout, process exit, capability failure, sequence failure, authority
widening or unexpected payload terminates the qualification channel.

## 7. Timeouts and process ownership

The qualification launcher applies bounded startup and I/O timeouts, pipes only
stdin/stdout, inherits stderr for diagnostics, and sets kill-on-drop. The child
exits on parent pipe closure or explicit shutdown.

Production C1 must additionally provide:

- Unix parent-death/process-group handling;
- Windows Job Object termination;
- executable and artifact digest verification;
- private runtime/profile roots;
- resource limits;
- sandbox and external-egress denial;
- crash and hung-process receipts.

## 8. Privacy

Serialized frame buffers are overwritten before release where the current
implementation owns them. The startup capability is never printed. Ordinary
receipts contain digests and typed outcomes, not raw profile or secret bytes.

This is not yet a complete secret-memory proof: serializer internals, allocator
copies, core-dump policy and OS process inspection remain C1/C4 qualification
items.

## 9. Qualification requirements

The current tests are required to prove:

- canonical bounded frame round trip;
- noncanonical and oversized rejection;
- wrong and replayed capability rejection;
- skipped sequence closes the channel;
- typed Actor request/response keeps authority closed;
- a separate child process can navigate and observe the local fixture over
  private pipes and shut down cleanly.

Passing those tests qualifies only the protocol scaffold. WEB-C1 remains open
until the same protocol is bound to a real pinned Servo worker and the required
platform-private transports, provenance, sandbox, listener and egress receipts.
