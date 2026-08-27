# ADR-0003: Hepta-owned Servo embedder instead of direct servoshell linkage

Status: **accepted for WEB-C1 implementation; qualification-only; no build or runtime authority**

Date: 2026-08-28

## Context

The pinned Servo source exposes a public embedding API from `components/servo`.
Its documentation defines the required integration sequence around
`EventLoopWaker`, `ServoBuilder`, a `RenderingContext`, `WebViewBuilder`, a
`WebViewDelegate`, `Servo::spin_event_loop`, `WebView::paint`, and
`RenderingContext::present`.

The existing `ports/servoshell` binary is useful implementation reference, but
it is not a safe minimal build root for Hepta:

- its `servo` dependency unconditionally enables `background_hang_monitor`,
  `bluetooth`, and `testbinding`, even when Cargo default features are disabled;
- it unconditionally depends on `webdriver_server`;
- its runtime state imports WebDriver controls and can call
  `webdriver_server::start_server`;
- the upstream server binds an HTTP listener to `0.0.0.0`;
- its normal application model owns a map of windows and a collection of
  WebViews rather than the Hepta one-session/one-process/one-WebView contract.

Consequently, `--no-default-features` on `servoshell` is insufficient to prove
the negative feature and listener posture already required by WEB-C1.

## Decision

WEB-C1 will use an **out-of-tree Hepta-owned worker crate** that depends directly
on the exact pinned `components/servo` crate. The worker is a separately built
artifact outside the Codex Cargo workspace and outside the Servo source tree.

The initial dependency contract is:

```text
package: servo
path: components/servo
default-features: false
required features:
  - background_hang_monitor
  - bundled
conditionally permitted after platform evidence:
  - js_jit
```

The following Servo features are forbidden in the initial worker:

```text
bluetooth
clipboard
default
default_web_features
default_without_allocator
gamepad
media-gstreamer
native-bluetooth
testbinding
webgl
webgpu
webxr
```

The worker will use only the public embedding surface frozen by
`SERVO_WORKER_SOURCE_TOPOLOGY_V1.json`. `ports/servoshell` and
`components/webdriver_server` are reference-only and must not appear in the
worker dependency graph.

The worker state will contain exactly:

```text
one Servo
one RenderingContext
Option<WebView>
one private inherited-channel actor
```

It must not contain a collection of WebViews or windows.

## Consequences

### Positive

- no WebDriver HTTP server dependency;
- no wildcard or loopback automation listener;
- Bluetooth and testbinding remain absent without maintaining a broad
  servoshell fork;
- Hepta controls the private process protocol, lifecycle fences, bounded command
  vocabulary, one-WebView invariant, and evidence format;
- the initial patch inventory can remain empty unless the public Servo embedding
  API proves insufficient.

### Costs

- Hepta must implement and qualify its own minimal platform event-loop and
  rendering-context integration;
- selected public Servo APIs and exact upstream blobs become release-blocking;
- any upstream API drift requires a reviewed successor topology and receipt;
- Linux, macOS, and Windows platform adapters require independent evidence.

## Rejected alternatives

### Build unmodified servoshell with `--no-default-features`

Rejected because the pinned manifest still unconditionally enables Bluetooth,
testbinding, and `webdriver_server`.

### Patch servoshell into the Hepta worker immediately

Rejected as the initial strategy because it creates a larger MPL patch surface,
retains multi-window/WebDriver coupling, and increases upstream-rebase cost.

A narrowly governed Servo patch remains possible only if the public embedding
API cannot satisfy a specific frozen requirement. Such a patch must identify
the exact missing API, affected files, security boundary, upstream issue or PR,
compatibility tests, and deletion condition.

### Reuse the upstream WebDriver server on loopback

Rejected. Loopback still creates a second authority surface and the pinned
implementation binds `0.0.0.0`. Raw WebDriver also exposes operations outside
the Hepta typed command and evidence contracts.

## Authority

This ADR grants no source acceptance, build permission, process-launch
permission, runtime authority, external network, production caller, effect
authority, operator acceptance, promotion, or release qualification.
