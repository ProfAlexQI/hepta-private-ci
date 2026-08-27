# WEB-C1.3 implementation security review

Status: `OPEN / MUST_CLOSE_BEFORE_QUALIFICATION_ONLY_PASS`

This review is intentionally separate from the implementation plan. A green compiler or unit test
does not close these findings.

## SR-1 — Debug redaction

The startup capability and host challenge/nonce are authentication material. Every type that owns
one of those values must implement an explicit redacted `Debug`; deriving `Debug` is forbidden.
Tests must assert that formatted values do not contain the underlying byte pattern.

## SR-2 — Fixture path traversal

`NavigateLocal.fixture_id` is a manifest key, not a path. The v1 validator must reject `/`, `\\`,
empty path segments, `.` and `..`, URL schemes, percent-decoded separators, and absolute paths.
The future worker must resolve the key through a preloaded exact manifest rather than joining it to
a caller-controlled filesystem path.

## SR-3 — Established-channel message classes

After mutual startup handshake, the channel accepts only `Command` and `Outcome`. Handshake
message kinds are invalid after establishment; receiving one must fence and close the channel.

## SR-4 — Capability lifetime and storage

Startup capability and host nonce are one-process, one-generation values. They must not appear in
argv, environment dumps, logs, receipts, panic output, core dumps, or reusable files. The real
launcher must supply them through inherited private memory/channel material and zeroize temporary
copies where the platform/runtime permits.

## SR-5 — Source self-report is not provenance

The source commit/tree inside `WorkerHello` is an expected-identity field only. It is not trusted
as proof. The host must independently bind the executable digest to a source/build/SBOM receipt.
Mismatch fails before any browser command.

## SR-6 — Request replay durability

C1 protocol request IDs are transport identities, not a durable effect ledger. The real BrowserActor
must retain used-ID tombstones or a monotonic watermark across bounded response-cache eviction and,
for any later effect-capable stage, across restart. An evicted response must not make a request ID
unused again.

## SR-7 — Timeout, hang, and half-open channel

The standalone codec is synchronous and does not set deadlines. The real host must own bounded
handshake, read, write, command, shutdown, and drain deadlines. Timeout causes process fencing and
teardown; it does not authorize replay against unknown external state.

## SR-8 — Platform equivalence

Unix `socketpair` evidence does not qualify Windows. The named-pipe implementation requires exact
SID ACL, one-client semantics, stale endpoint prevention, inherited process binding, and equivalent
negative tests. macOS and Linux additionally require sandbox and descriptor inheritance review.

## SR-9 — Parser and allocation fuzzing

Before a real Servo worker consumes the protocol, add deterministic corpus and fuzz/property tests
for all message kinds, arbitrary truncation, length boundaries, UTF-8 errors, duplicate frames,
unknown versions, and state-machine order. Any parser panic, unbounded allocation, or silent trailing
input is release-blocking.

## SR-10 — No authority inheritance

A successful process handshake proves only the local worker binding. It never grants runtime,
production, effect, network, credential, operator, G5, promotion, or release authority. This must
remain encoded in protocol validation and receipt schemas rather than being a documentation-only
statement.
