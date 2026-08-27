# WEB-C1.3 — private Servo worker protocol and inherited-channel qualification

Status: `IMPLEMENTED_QUALIFICATION_ONLY / SERVO_NOT_LINKED / NOT_C1_COMPLETE`  
Plan relationship: normative implementation append to `WEB-PLAN-2026-08-27E`; it narrows the
`C1-003` worker-protocol slice and does not supersede C0–C7 ordering.  
Date: 2026-08-27  
Authority: none; all production/effect/operator/promotion flags remain false.

## 1. Decision

The first real C1 boundary is a Hepta-owned private worker protocol, not an exposed Servo
WebDriver server. One browser session receives one child worker and one inherited private
channel. The worker proves an exact source pin and exact lifecycle identity before it may accept
any command.

The qualification implementation lives at:

```text
tools/hepta-browser-c1-protocol/
```

It is intentionally a standalone zero-dependency crate. It does not join the production Cargo
workspace, import Servo, or create a product caller. The next Servo adapter must implement this
contract or a reviewed versioned successor; it must not bypass it with raw WebDriver/CDP.

## 2. Source and artifact boundary

The current research pin is:

```text
repository: servo/servo
commit: 0a48e298482659817eb50097df23841f2b8e3044
tree: b04d2f75b3217374d079d579c270177b57fa1389
license: MPL-2.0
status: SOURCE_PIN_ONLY
```

The pin is not an imported source tree or executable artifact. `WEB-C1` remains incomplete until
all of the following are separately bound:

1. source archive digest and tree inventory;
2. upstream signature/provenance observation;
3. Hepta patch inventory and deletion conditions;
4. Cargo/rust-toolchain/native dependency lock;
5. license notices and source-distribution obligations;
6. reproducible or independently repeated build receipt;
7. worker executable digest, symbols policy, SBOM, and platform identity;
8. sandbox/listener/egress negative evidence;
9. real one-WebView local-fixture smoke.

A worker may report the expected commit/tree only as an input to verification. Self-reporting is
not sufficient; the parent must also bind the launched executable digest to a source/build
receipt.

## 3. Process topology

```text
agentd / fleet supervisor
        |
        | lifecycle generation + private launch authority
        v
hepta-browser host / BrowserActor
        |
        | pre-created inherited channel
        | startup capability + host nonce
        v
pinned Servo worker process
        |
        `-- one WebView / one event loop / one private profile root
```

The worker is not discoverable. C1 must not bind TCP, wildcard addresses, HTTP, WebSocket,
WebDriver, CDP, or a public filesystem socket. Unix qualification uses `socketpair`; Windows must
use a pre-created named pipe with an explicit SID ACL and equivalent one-client semantics.

The parent creates the channel before process launch and passes exactly one endpoint. Child
process arguments and environment may contain only nonsecret descriptors and bounded identity
metadata. Startup capability and host nonce must be passed through inherited private material,
not command-line arguments, logs, or a world-readable file.

## 4. Identity and mutual startup handshake

Every worker identity is:

```text
BrowserSessionId
+ generation
+ owner_epoch
+ Servo source commit
+ Servo source tree
+ qualification-only authority posture
```

The current handshake is:

```text
WorkerHello(
  protocol_version,
  session_id,
  generation,
  owner_epoch,
  servo_commit,
  servo_tree,
  authority_bits,
  startup_capability
)

HostAck(
  same identity,
  accepted=true,
  host_nonce
)

WorkerConfirm(
  same identity,
  host_nonce
)
```

Rules:

- session, generation, and owner epoch are nonzero and exact;
- commit and tree are exactly 40 lowercase hexadecimal bytes;
- startup capability and host nonce are 256-bit nonzero opaque values;
- capability and nonce comparisons are constant-time with respect to equal-length values;
- any unknown or positive authority bit fails closed;
- no negative acknowledgement is sent after failed authentication; the host closes the channel;
- any message before or after the expected handshake step fails closed;
- reconnecting an old worker to a new generation or epoch is prohibited;
- a successful handshake creates only a local process binding, not execution/effect authority.

The current standalone implementation does not claim protection against a hostile process that
already controls the parent process memory. It establishes the correct child-process boundary for
the later OS sandbox and executable-digest checks.

## 5. Frame and codec contract

Wire framing:

```text
u32_be payload_length
payload bytes
```

Payload prefix:

```text
magic[8] = HEPTABR1
protocol_version: u16_be = 1
message_kind: u8
```

Hard limits:

```text
max frame                         65,536 bytes
max typed text                     4,096 bytes
max semantic reference               256 bytes
max local fixture id                  128 bytes
max denial/outcome code               128 bytes
max observe nodes                     512
max human-control lease            300,000 ms
```

Decoder requirements:

- read the length before allocating the payload;
- reject zero length and any length above the maximum;
- reject truncation, trailing bytes, unknown message kinds, unknown command kinds, unknown status,
  unsupported version, invalid UTF-8, noncanonical source pins, and invalid field bounds;
- consume exactly one frame; there is no implicit stream resynchronization after corruption;
- integer encoding is fixed-width big-endian;
- strings are `u32_be length + UTF-8 bytes` and may not be empty where the schema requires a value;
- no map, extension bag, arbitrary JSON, arbitrary JavaScript, raw header, cookie, or profile field
  exists in v1.

## 6. C1 command vocabulary

Allowed v1 commands are deliberately narrower than the final browser contract:

```text
Ping
NavigateLocal { fixture_id }
Observe { limit }
Click { semantic_ref }
TypeText { semantic_ref, text }
HumanTakeover { lease_ms }
HumanRelease
Shutdown
```

`NavigateLocal` is a lookup key into a parent-approved bundled fixture manifest. It is not a URL
parser. Values containing `://`, absolute paths, empty values, oversized values, or noncanonical
identifier bytes are rejected.

The v1 protocol contains no command for:

- external HTTP(S), DNS, redirect, WebSocket, WebRTC, or service worker access;
- raw WebDriver/CDP or unrestricted JavaScript;
- cookie/localStorage/profile/credential export;
- file URL, arbitrary path, upload, download, clipboard, or host process execution;
- preference mutation, proxy mutation, extension loading, or certificate override;
- production effect dispatch.

Adding any such command requires a schema version change plus its own policy, threat, receipt, and
qualification gate.

## 7. Post-handshake fencing

Every command and outcome binds:

```text
request_id
session_id
generation
owner_epoch
page_revision
```

A `FramedChannel` rejects a message whose process identity differs from the established binding
before writing it to the channel. The Servo adapter must additionally validate page revision and
semantic-ref revision inside the single BrowserActor queue.

The worker cannot increase generation, epoch, authority, or source pin. A lifecycle change causes
the parent to close the old channel and create a new process binding. There is no in-place worker
adoption across generations in C1.

## 8. Outcome semantics

The bounded outcome classes are:

```text
Completed
Denied
Stale
Invalid
Indeterminate
```

An outcome carries a stable bounded code, not raw page text, stack traces, headers, cookies,
credentials, or exception bodies. `Indeterminate` is nonterminal with respect to any future
external effect. It must not trigger blind replay or be translated to success.

C1 local fixtures have no external effect, so an indeterminate worker result causes session
fencing/teardown and evidence capture rather than retrying the command against an unknown page.

## 9. Security properties and nonclaims

Implemented and tested in this slice:

- bounded allocation before frame decode;
- exact source-pin syntax;
- exact lifecycle identity;
- mutual startup secret challenge;
- negative authority bitset;
- no listener API in the qualification crate;
- no dependency graph that could create network behavior;
- local-fixture-only navigation vocabulary;
- stale identity rejection before write;
- secret value redaction for `StartupCapability` debug output.

Not yet claimed:

- real Servo executable identity or source/build correspondence;
- OS peer credential or sandbox proof;
- Windows named-pipe implementation;
- macOS sandbox profile, Linux namespace/seccomp/landlock, or Windows AppContainer/job proof;
- crash-safe durable journal;
- real semantic extraction from Servo;
- external egress enforcement;
- product integration or readiness;
- operator acceptance or release qualification.

## 10. Qualification matrix

Required portable tests:

1. exact WorkerHello encode/decode;
2. command and outcome encode/decode;
3. length-prefixed I/O round trip;
4. oversize rejection before payload allocation;
5. truncation rejection;
6. trailing-byte rejection;
7. unknown message/command/status rejection;
8. unknown or positive authority-bit rejection;
9. uppercase, short, or invalid source-pin rejection;
10. zero session/capability/nonce rejection;
11. external URL and absolute-path fixture rejection;
12. zero/out-of-range observe and human lease rejection;
13. inherited Unix socketpair mutual handshake;
14. wrong startup capability fail-closed;
15. stale process identity rejected before write.

Static verifier requirements:

- no `TcpListener`, `TcpStream`, wildcard address, HTTP client, WebDriver server, or WebSocket
  implementation in the crate source;
- no third-party dependencies;
- all authority constants remain false except `QUALIFICATION_ONLY=true`;
- source pin, plan status, receipt schema, workflow commands, and file set agree.

## 11. Definition of done for C1.3

This narrow slice is complete only when:

- `cargo fmt --check` passes;
- locked `cargo test` passes on Linux and macOS, with Unix socketpair tests enabled;
- locked strict Clippy passes for all targets;
- the dependency-free plan/protocol verifier passes;
- the workflow uploads a source/tree-bound qualification-only posture artifact;
- no positive authority flag or external network capability is present.

Even after C1.3 passes, `WEB-C1` remains incomplete. C1 completion additionally requires the real
pinned Servo worker, exact artifact provenance, one-WebView fixture smoke, listener/egress negative
tests, and the supported platform matrix.

## 12. Immediate next implementation queue

```text
C1-004A  generate and verify a Servo source archive/tree/license receipt
C1-004B  define worker executable manifest and artifact-digest binding
C1-004C  implement a minimal Servo worker that consumes the inherited endpoint
C1-004D  map NavigateLocal/Observe/Click/TypeText to one real WebView
C1-004E  prove no listener and no external egress at OS level
C1-004F  bind the real worker to BrowserActor generation/epoch/revision fences
C1-004G  repeat build and local-fixture qualification on Linux/macOS/Windows
C1-004H  seal a nonpromotion WEB-C1 qualification receipt
```

No item in this queue may silently enable external navigation, credentials, production caller,
effect authority, operator acceptance, G5, promotion, or release.
