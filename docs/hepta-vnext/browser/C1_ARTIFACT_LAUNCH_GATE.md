# Hepta Browser C1 artifact-bound process launch gate

Status: **zero-dependency qualification scaffold implemented; no real Servo artifact or runtime is qualified**

## Purpose

`C1-004C-1` may not spawn a future Servo worker merely because a file exists. Before browser protocol traffic starts, the parent must bind the exact executable bytes to the exact build manifest and source receipt, authenticate the specific child process, enforce bounded I/O, and retain forced-kill/reap control.

The standalone crate is:

```text
tools/hepta-browser-c1-artifact-gate/
```

It intentionally has no third-party dependencies and does not depend on Servo or the product browser API.

## Qualification binding

For the current fixture trial both parent and child independently compute:

```text
executable_sha256
embedded_build_manifest_sha256
embedded_source_receipt_sha256
```

The parent also generates a 256-bit private challenge, creates two inherited Unix socketpairs, starts the exact current executable as a child, and requires the child-reported PID and all three digests to match before sending `HostAck`. The challenge is redacted from `Debug` and compared without early exit.

This fixture proves the launch-gate mechanics. A real C1 worker must replace the embedded fixture documents with the independently verified canonical build manifest and source receipt, and the executable digest must match `hepta.servo.worker_artifact_receipt.v1` before spawn.

## Protocol

The launch gate uses a separate, fixed, maximum-512-byte binary envelope with:

- magic and version;
- `WorkerHello`, `HostAck`, and `WorkerConfirm`;
- exact PID and artifact binding;
- qualification-only authority bits;
- `Ping/Pong` and `Shutdown/ShutdownAck` only.

Unknown kinds, unknown or positive authority bits, zero or oversized frames, truncation, trailing bytes, zero digests, zero challenge, PID mismatch, artifact mismatch, or challenge mismatch fail closed.

The launch gate is deliberately separate from the higher-level browser command protocol. Only after it succeeds may a future implementation hand the inherited channel to the session/generation/owner-epoch browser protocol.

## Deadlines and teardown

The Unix fixture configures bounded read/write deadlines. The normal trial performs authenticated ping and graceful shutdown, closes the channel, and reaps the child. A second trial intentionally hangs the worker after ping, requires the parent read deadline to fire, kills the exact child, and boundedly reaps it.

The guard kills and waits for an armed child on scope exit, including error paths.

## Non-authority boundary

The current crate:

```text
links Servo: false
uses a real Servo artifact: false
creates a WebView: false
uses TCP/UDP/HTTP/WebDriver/CDP: false
allows external network: false
exports credentials: false
runtime authority: false
production caller/writer: false
effect/operator/promotion/release authority: false
```

A passing job qualifies only the artifact-binding and process-control scaffold on the tested Unix runner. It does not prove executable provenance from a real Servo build, sandboxing, parent-death process groups, descendant cleanup, macOS equivalence, or Windows SID-restricted named pipes and Job Objects.
