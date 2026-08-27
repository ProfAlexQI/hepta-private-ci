# ADR-0002: inherited private channel for the Servo worker

- Status: Accepted for WEB-C1 qualification
- Date: 2026-08-27
- Scope: browser host ↔ one Servo worker process
- Authority impact: none

## Context

Upstream Servo currently exposes automation through a WebDriver HTTP server. A general listener is
not an acceptable Hepta process boundary: it creates discovery, authentication, routing,
downgrade, stale-process, and same-user attack surfaces before the browser policy and lifecycle
fences are established.

Hepta requires one worker per browser session, exact generation/epoch fencing, a single page
owner, and no network control listener. The parent already owns process lifecycle and can create a
channel before spawning the child.

## Decision

The C1 worker uses one inherited private bidirectional byte stream:

- Unix: `socketpair` or an equivalent inherited connected Unix stream;
- Windows: a pre-created named pipe with explicit allowed SID and one-client semantics;
- no TCP/UDP/HTTP/WebSocket listener;
- no path-discoverable UDS in C1;
- no raw WebDriver/CDP surface;
- one startup capability and one host nonce per process generation;
- exact session, generation, owner epoch, source commit, and source tree binding;
- close-on-authentication-failure and close-on-fence-change.

The transport carries a Hepta-owned bounded protocol. Servo automation handlers may be reused
inside the child implementation, but their network server and unrestricted command vocabulary are
not exposed.

## Consequences

Positive:

- no port allocation, wildcard bind, discovery, or ambient network authentication;
- the child cannot attach to another session without the inherited endpoint and startup material;
- lifecycle fencing is established before browser commands;
- protocol limits and negative authority are controlled by Hepta;
- the same parent-side BrowserActor remains the mutation owner.

Costs:

- Hepta must maintain a small protocol and adapter;
- Windows requires a separate named-pipe implementation and ACL qualification;
- debugging cannot rely on an unrestricted WebDriver client;
- protocol additions require explicit versioning and qualification.

## Rejected alternatives

### Upstream WebDriver bound to `0.0.0.0`

Rejected because it exposes a general network listener before Hepta authentication and violates
the no-listener C1 boundary.

### Loopback TCP plus bearer token

Rejected for C1. Loopback reduces reachability but still introduces port discovery, proxy and
firewall interactions, stale listener ownership, browser-origin confusion, and a second network
protocol. It may only be reconsidered through a new ADR with a demonstrated platform requirement.

### Public filesystem UDS

Rejected for C1 because path ownership, stale socket cleanup, same-UID callers, and ACL semantics
would become part of the initial trust boundary. An inherited connected endpoint is narrower.

### In-process Servo inside agentd

Rejected because a browser crash, renderer fault, native dependency issue, or memory exhaustion
would share the agent runtime failure domain and weaken one-session isolation.

## Follow-up gates

1. implement and qualify the Windows named-pipe equivalent;
2. bind the launched executable digest to the Servo source/build receipt;
3. add OS sandbox and resource-limit evidence;
4. prove no listener and no external egress at runtime;
5. bind the real WebView adapter to BrowserActor revision fencing;
6. retain all production/effect/operator/promotion flags as false until independent later gates.
