# WEB-C1 artifact-to-browser startup bridge

Status: **implemented as a qualification-only integration candidate; compile and runtime evidence are still required**

## Purpose

The launch gate and the browser worker protocol were previously qualified as separate fixtures. This bridge exercises their required production ordering over one inherited private process channel:

```text
spawn exact child process
  -> artifact/build/source digest challenge
  -> spawned PID verification
  -> artifact gate HostAck / WorkerConfirm
  -> private browser capability + host nonce
  -> session / generation / owner-epoch / Servo source-pin handshake
  -> bounded Ping / Shutdown
  -> deadline / kill / reap
```

No command is admitted to the browser protocol before the executable, embedded build manifest, embedded source receipt, spawned PID, and one-process challenge match.

## Current implementation

`tools/hepta-browser-c1-startup-bridge` is a standalone qualification crate with two local path dependencies only:

- `hepta-browser-c1-artifact-gate-qualification`;
- `hepta-browser-worker-protocol-qualification`.

The process trial uses two anonymous inherited Unix socketpairs. It does not create a filesystem socket, TCP listener, UDP socket, HTTP surface, WebDriver endpoint, DevTools/CDP endpoint, public UDS, network client, profile, credential, or Servo WebView.

The normal trial proves:

1. parent and child independently hash the exact executable bytes;
2. both bind the same embedded build-manifest and source-receipt bytes;
3. the parent verifies the PID reported by the child is the PID it spawned;
4. the artifact challenge is accepted before browser bootstrap material is sent;
5. BrowserSessionId, generation, owner epoch, startup capability, host nonce, Servo commit and Servo tree all match;
6. only exact `Ping` and `Shutdown` are accepted;
7. the child shuts down and is reaped;
8. all runtime, product, effect, network, credential, operator, promotion and release authority stays false.

The forced-teardown trial proves that a child hanging after the browser handshake causes a private-channel read deadline, is killed, and is reaped within a bounded interval.

## Security boundary

This bridge closes the qualification gap between “artifact was checked” and “browser protocol was established.” It does **not** prove that the artifact is a real Servo worker. The embedded manifest and source receipt are explicit qualification fixtures, and `servo_linked=false` remains part of the result.

The bridge rejects or terminates on:

- executable/build/source digest mismatch;
- reported PID mismatch;
- artifact challenge mismatch;
- browser capability or nonce mismatch;
- stale session, generation or owner epoch;
- Servo commit/tree drift;
- unknown or positive authority posture;
- unexpected artifact or browser message;
- private-channel read/write deadline;
- unsuccessful or unreaped child process.

## Remaining gates

Before this can support a real WEB-C1 worker:

- the canonical two-fetch Servo source bundle must be sealed;
- a real build-input packet and worker build manifest must be generated;
- the real worker must be built twice and artifact receipts compared;
- the startup bridge must consume those external immutable receipts rather than embedded fixtures;
- executable open/verification/spawn TOCTOU must be hardened with a platform artifact handle or equivalent;
- Linux/macOS sandbox and descendant cleanup receipts must exist;
- the Windows SID-restricted named-pipe and Job Object equivalent must pass;
- one real local-fixture Servo WebView and semantic action must pass;
- exact-head GitHub Actions evidence must exist.

A passing fixture result is only `ARTIFACT_TO_BROWSER_HANDOFF_QUALIFICATION_PASS`. It is not Servo runtime qualification, operator acceptance, promotion, or release authority.
